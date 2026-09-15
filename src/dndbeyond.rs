use crate::{auth::Session, campaigns::Campaigns};
use eframe::egui;
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

fn campaign_id<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
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
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .redirect(Policy::none())
        .user_agent("Splat/0.1")
        .build()
        .map_err(|_| "Could not start the campaign request.")?;
    let response = session
        .website_request(client.get(URL).header("Accept", "application/json"))?
        .send()
        .map_err(|_| "Could not reach D&D Beyond. Check your connection and try again.")?;
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

#[derive(Default)]
pub struct DndBeyond {
    account_epoch: u64,
    campaigns: Vec<RemoteCampaign>,
    pending: Option<Receiver<Result<Vec<RemoteCampaign>, String>>>,
    loaded: bool,
    error: Option<String>,
    message: Option<String>,
}

impl DndBeyond {
    pub fn poll(&mut self, ctx: &egui::Context, epoch: u64, connected: bool) {
        if epoch != self.account_epoch || !connected {
            // Dropping the receiver discards a previous account's late response.
            self.pending = None;
            self.campaigns.clear();
            self.loaded = false;
            self.account_epoch = epoch;
            self.error = None;
            self.message = None;
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

    fn refresh(&mut self, session: Session, ctx: &egui::Context) {
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        self.error = None;
        self.message = None;
        let ctx = ctx.clone();
        thread::spawn(move || {
            let _ = tx.send(fetch(session));
            ctx.request_repaint();
        });
    }

    pub fn show(&mut self, ui: &mut egui::Ui, session: Option<Session>, local: &mut Campaigns) {
        ui.strong("D&D Beyond link");
        let Some(campaign) = local.active_campaign() else {
            ui.label("Select a local campaign to configure its D&D Beyond link.");
            return;
        };
        ui.label("Each local campaign can link to one D&D Beyond campaign. Each D&D Beyond campaign can be used once.");
        if session.is_none() {
            ui.label("Connect above to load your D&D Beyond campaigns.");
        }
        if ui
            .add_enabled(
                session.is_some() && self.pending.is_none(),
                egui::Button::new("Refresh D&D Beyond campaigns"),
            )
            .clicked()
            && let Some(session) = session
        {
            self.refresh(session, ui.ctx());
        }
        if self.pending.is_some() {
            ui.spinner();
        }
        if let Some(error) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
        if let Some(message) = &self.message {
            ui.label(message);
        }
        let current = campaign.dndbeyond_campaign_id;
        let mut selected = current.clone();
        let label = current
            .as_ref()
            .map(|id| {
                self.campaigns
                    .iter()
                    .find(|c| &c.id == id)
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| format!("Campaign {id}"))
            })
            .unwrap_or_else(|| "Not linked".into());
        egui::ComboBox::from_id_salt(("ddb_link", campaign.id))
            .selected_text(label)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut selected, None, "Not linked");
                for remote in &self.campaigns {
                    let used = local.linked_elsewhere(&remote.id);
                    ui.add_enabled_ui(!used, |ui| {
                        let name = if used {
                            format!("{} (already linked)", remote.name)
                        } else {
                            remote.name.clone()
                        };
                        ui.selectable_value(&mut selected, Some(remote.id.clone()), name);
                    });
                }
            });
        if selected != current {
            match local.link(selected) {
                Ok(()) => {
                    self.message = Some("Campaign link saved.".into());
                    self.error = None;
                }
                Err(error) => self.error = Some(error),
            }
        }
        if let Some(id) = &current {
            if let Some(remote) = self.campaigns.iter().find(|c| &c.id == id) {
                if let Some(dm) = &remote.dm_username {
                    ui.label(format!("DM: {dm}"));
                }
                if let Some(count) = remote.player_count {
                    ui.label(format!("Players: {count}"));
                }
            }
            ui.hyperlink_to(
                "Open D&D Beyond campaign",
                format!("https://www.dndbeyond.com/campaigns/{id}"),
            );
        }
        if self.loaded && self.campaigns.is_empty() {
            ui.label("No campaigns returned for this account.");
        }
        ui.weak("Only the link ID is saved. D&D Beyond campaign details stay in memory.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        campaigns.poll(&egui::Context::default(), 1, false);
        assert!(tx.send(Ok(Vec::new())).is_err());
        assert!(campaigns.campaigns.is_empty());
    }
}
