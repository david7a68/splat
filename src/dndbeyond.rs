#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use crate::state_context::StateContext;
use crate::{auth::Session, campaigns::Campaigns};
use reqwest::{blocking::Client, redirect::Policy};
use serde::Deserialize;
use std::{
    collections::HashSet,
    io::Read,
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

const URL: &str = "https://www.dndbeyond.com/api/campaign/stt/user-campaigns";
const MAX_RESPONSE: u64 = 4 * 1024 * 1024;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemoteCampaign {
    #[serde(deserialize_with = "campaign_id")]
    id: String,
    name: String,
    dm_username: Option<String>,
    player_count: Option<u64>,
}

pub fn campaign_id<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    let value = serde_json::Value::deserialize(deserializer)?;
    let id = match value {
        serde_json::Value::String(value) => value.parse::<u64>().ok(),
        serde_json::Value::Number(value) => value.as_u64(),
        _ => None,
    }
    .filter(|id| *id > 0)
    .ok_or_else(|| serde::de::Error::custom("invalid campaign ID"))?;
    Ok(id.to_string())
}

fn parse(bytes: &[u8]) -> Result<Vec<RemoteCampaign>, String> {
    #[derive(Deserialize)]
    struct Envelope {
        status: String,
        data: Option<Vec<RemoteCampaign>>,
    }
    let response: Envelope = serde_json::from_slice(bytes)
        .map_err(|_| "D&D Beyond returned an unexpected campaign response. The site may be blocking the request.".to_owned())?;
    if response.status != "success" {
        return Err("D&D Beyond could not list campaigns. Try again later.".into());
    }
    let campaigns = response
        .data
        .ok_or("D&D Beyond omitted the campaign list.")?;
    let mut ids = HashSet::new();
    if campaigns
        .iter()
        .any(|c| c.name.trim().is_empty() || !ids.insert(&c.id))
    {
        return Err("D&D Beyond returned an invalid campaign list.".into());
    }
    Ok(campaigns)
}

fn fetch(session: Session) -> Result<Vec<RemoteCampaign>, String> {
    // Test builds have no route to D&D Beyond. A signed-in test sees this list.
    if cfg!(any(test, feature = "test-support")) {
        return Ok(vec![
            RemoteCampaign {
                id: "7700001".into(),
                name: "The Emberfall Marches".into(),
                dm_username: None,
                player_count: Some(5),
            },
            RemoteCampaign {
                id: "7700002".into(),
                name: "Quiet Harbor".into(),
                dm_username: None,
                player_count: Some(4),
            },
            RemoteCampaign {
                id: "9002".into(),
                name: "Salt Road".into(),
                dm_username: Some("A DM".into()),
                player_count: Some(4),
            },
        ]);
    }
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .redirect(Policy::none())
        .user_agent("Splat/0.1")
        .build()
        .map_err(|_| "Could not start the campaign request.")?;
    let response = crate::auth::website_request(
        &session,
        client.get(URL).header("Accept", "application/json"),
    )?
    .send()
    .map_err(|_| "Could not reach D&D Beyond. Check your connection.")?;
    match response.status().as_u16() {
        200..=299 => {},
        401 | 403 => return Err("D&D Beyond rejected the request. Reconnect in Settings, or try again later if the site is blocking access.".into()),
        429 => return Err("D&D Beyond is limiting requests. Wait before refreshing.".into()),
        _ => return Err("D&D Beyond could not list campaigns. Try again later.".into()),
    }
    let mut bytes = Vec::new();
    response
        .take(MAX_RESPONSE + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Could not read the campaign response.")?;
    if bytes.len() as u64 > MAX_RESPONSE {
        return Err("The campaign response is too large.".into());
    }
    parse(&bytes)
}

/// The link's one kept result, `Campaign link saved.`, of the local campaign whose link was
/// saved. It ends with the next refresh or link, a failed link attempt, an account change, and a
/// change of the open campaign (`poll`, every frame, on any page).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkSaved(uuid::Uuid);

impl crate::design::results::Owner for LinkSaved {
    type Scope = ();
    fn scope(&self) -> &() {
        &()
    }
}

impl crate::design::results::Kept for LinkSaved {
    type Owner = LinkSaved;
    const ENDS: crate::design::results::Ends = crate::design::results::Ends {
        // A refresh ends it (`owner_changed`); a new link is a new result.
        owner_changed: crate::design::results::OwnerChanged::Ends,
        disk_changed: crate::design::results::DiskChanged::Stays,
        press: crate::design::results::Press::Nothing,
    };
    fn owner(&self) -> LinkSaved {
        *self
    }
}

#[derive(Default)]
pub struct DndBeyond {
    account_epoch: u64,
    campaigns: Vec<RemoteCampaign>,
    pending: Option<Receiver<Result<Vec<RemoteCampaign>, String>>>,
    loaded: bool,
    error: Option<String>,
    /// The link's result, with the order the link took for the footer (the store's).
    message: crate::design::Results<LinkSaved, LinkSaved>,
    /// The open local campaign as the last `poll` saw it: a change ends the link's result.
    open: Option<uuid::Uuid>,
    visible_last_frame: bool,
    requested_visible_link: Option<(uuid::Uuid, String)>,
    pending_link: Option<(uuid::Uuid, String)>,
}

impl DndBeyond {
    /// The list's or the link's error, as the footer says it (tests).
    #[cfg(test)]
    pub(crate) fn error_for_test(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// A list as a refresh would bring it: `(id, name)` per remote campaign (tests).
    #[cfg(test)]
    pub(crate) fn listed_for_test(list: &[(&str, &str)]) -> Self {
        Self {
            campaigns: list
                .iter()
                .map(|(id, name)| RemoteCampaign {
                    id: (*id).into(),
                    name: (*name).into(),
                    dm_username: None,
                    player_count: None,
                })
                .collect(),
            loaded: true,
            ..Default::default()
        }
    }

    /// A list is on its way, so what is on screen is due to change.
    #[cfg(any(test, feature = "test-support"))]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    pub fn begin_frame(&mut self) {
        if !self.visible_last_frame {
            self.requested_visible_link = None;
        }
        self.visible_last_frame = false;
    }

    /// The link's result ends, whichever campaign's (see `LinkSaved`).
    fn end_link_result(&mut self) {
        self.message.end_unreachable(|_| false);
    }

    pub fn poll(&mut self, ctx: &StateContext, epoch: u64, connected: bool, local: &mut Campaigns) {
        let open = local.active_campaign().map(|campaign| campaign.id);
        if open != self.open {
            self.open = open;
            self.end_link_result();
        }
        if epoch != self.account_epoch || !connected {
            // Dropping the receiver discards a previous account's late response.
            self.pending = None;
            self.pending_link = None;
            self.requested_visible_link = None;
            self.campaigns.clear();
            local.cancel_import();
            self.loaded = false;
            self.account_epoch = epoch;
            self.error = None;
            self.end_link_result();
            return;
        }
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    match result {
                        Ok(campaigns) => {
                            self.campaigns = campaigns;
                            self.loaded = true;
                            self.error = None;
                            if let Some((id, remote_id)) = self.pending_link.take() {
                                if let Some(remote) =
                                    self.campaigns.iter().find(|c| c.id == remote_id)
                                {
                                    if let Err(error) =
                                        local.cache_remote_name(id, &remote_id, &remote.name)
                                    {
                                        self.error = Some(error);
                                    }
                                } else {
                                    self.error = Some("The linked campaign was not returned. Keeping its cached name.".into());
                                }
                            }
                        }
                        Err(error) => {
                            self.error = Some(error);
                        }
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {
                    ctx.request_repaint_after(Duration::from_millis(100));
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.error = Some("The campaign request stopped. Try refreshing.".into());
                }
            }
        }
    }

    fn refresh(&mut self, session: Session, ctx: &StateContext) {
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        self.error = None;
        self.end_link_result();
        let ctx = ctx.waker();
        thread::spawn(move || {
            let _ = tx.send(fetch(session));
            ctx.request_repaint();
        });
    }

    /// Round 26: the D&D Beyond list's error and, on Link and folder (`link`), the link's result,
    /// in the footer of the pane that shows them. Both live until the next refresh or link.
    /// The messages are typed data, so the pane's body can change this list while the footer
    /// waits for it (`Messages::pane`). `frame` is the pane's. `link` is the open campaign on
    /// the Link and folder pane, read when the footer is built, after the campaign switcher
    /// drew this frame (review of PR 665): only its own result shows, also in the frame of a
    /// switch, before `poll` ends the other's. `None` on Settings, which shows no link result.
    pub fn footer_messages<'f, A: 'static>(
        &self,
        frame: &'f crate::design::outcome::PaneFrame,
        messages: &mut crate::design::outcome::Messages<'f, A>,
        link: Option<uuid::Uuid>,
    ) {
        if let Some(error) = &self.error {
            messages.outcome(crate::design::outcome::FaultMessage::new(
                &crate::campaigns::NotDone::words(error),
            ));
        }
        if let Some(open) = link
            && let Some(shown) = self.message.get(&LinkSaved(open))
        {
            messages.kept(shown.outcome(frame, |_| {
                crate::design::outcome::Said::new(crate::sentence!("Campaign link saved.")).into()
            }));
        }
    }
}

#[path = "dndbeyond/view.rs"]
pub mod view;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visibility_keeps_request_marker_until_setting_leaves_view() {
        let key = (uuid::Uuid::new_v4(), "12".to_owned());
        let mut remote = DndBeyond {
            visible_last_frame: true,
            requested_visible_link: Some(key.clone()),
            ..Default::default()
        };
        remote.begin_frame();
        assert_eq!(remote.requested_visible_link, Some(key));
        // No show call in this frame: the settings page is hidden.
        remote.begin_frame();
        assert!(remote.requested_visible_link.is_none());
    }

    #[test]
    fn reads_numeric_and_string_ids_and_optional_details() {
        let campaigns = parse(br#"{"status":"success","data":[{"id":12,"name":"One","dmUsername":"DM","playerCount":3},{"id":"34","name":"Two"}]}"#).unwrap();
        assert_eq!(campaigns[0].id, "12");
        assert_eq!(campaigns[1].id, "34");
        assert_eq!(campaigns[0].player_count, Some(3));
    }

    #[test]
    fn rejects_unexpected_and_duplicate_data() {
        for bytes in [
            b"<html>Login</html>".as_slice(),
            br#"{"status":"failure","data":null}"#,
            br#"{"status":"success","data":[{"id":"../bad","name":"Bad"}]}"#,
            br#"{"status":"success","data":[{"id":1,"name":"One"},{"id":1,"name":"Two"}]}"#,
        ] {
            assert!(parse(bytes).is_err());
        }
        assert!(
            parse(br#"{"status":"success","data":[]}"#)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn disconnect_discards_pending_campaign_response() {
        let (tx, rx) = mpsc::channel();
        let mut campaigns = DndBeyond {
            pending: Some(rx),
            ..Default::default()
        };
        campaigns.poll(&StateContext::new(), 1, false, &mut Campaigns::default());
        assert!(tx.send(Ok(Vec::new())).is_err());
        assert!(campaigns.campaigns.is_empty());
    }
}
