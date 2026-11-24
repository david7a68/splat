#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
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

fn connection_help(ui: &mut egui::Ui) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for word in "Connect your account in ".split_inclusive(' ') {
            ui.label(
                crate::design::Type::Small
                    .rich(ui, word)
                    .color(crate::design::colors().muted),
            );
        }
        if crate::design::small_link(ui, "Settings").clicked() {
            ui.ctx()
                .data_mut(|data| data.insert_temp(egui::Id::new("open_account_settings"), true));
        }
        crate::design::gap(ui, crate::design::Gap::XS);
        for word in "to update this link.".split_inclusive(' ') {
            ui.label(
                crate::design::Type::Small
                    .rich(ui, word)
                    .color(crate::design::colors().muted),
            );
        }
    });
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
    /// The widest remote campaign name, measured once per list (round 27 AN2). It ends when
    /// `campaigns` is replaced or cleared.
    name_column: Option<f32>,
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

    /// Give one remote campaign another name, as a new list from D&D Beyond would. Tests only:
    /// the test list is fixed (`fetch`).
    #[cfg(any(test, feature = "test-support"))]
    pub fn rename_remote(&mut self, id: &str, name: &str) {
        for remote in self.campaigns.iter_mut().filter(|remote| remote.id == id) {
            remote.name = name.to_owned();
        }
        self.name_column = None;
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

    pub fn poll(
        &mut self,
        ctx: &egui::Context,
        epoch: u64,
        connected: bool,
        local: &mut Campaigns,
    ) {
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
            self.name_column = None;
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
                            self.name_column = None;
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

    fn refresh(&mut self, session: Session, ctx: &egui::Context) {
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        self.error = None;
        self.end_link_result();
        let ctx = ctx.clone();
        thread::spawn(move || {
            let _ = tx.send(fetch(session));
            ctx.request_repaint();
        });
    }

    /// How to import from D&D Beyond, under the open Add campaign form when no account is
    /// connected, in the narrow layout (the account section is then below the campaigns, often
    /// out of view). It follows the form, so opening the form never moves what is above it, and
    /// it depends on the form, the session and the layout, never on what is in view (review of
    /// PR 320: it was drawn above the form when the account heading left the view, and moved
    /// the pressed `Add campaign` by its height).
    pub fn import_hint(ui: &mut egui::Ui) {
        if ui.ctx().content_rect().width() >= 1000.0 {
            return;
        }
        ui.horizontal_wrapped(|ui| {
            ui.label("To import from D&D Beyond,");
            if crate::design::link(ui, "connect your account").clicked() {
                ui.ctx()
                    .data_mut(|data| data.insert_temp(egui::Id::new("focus_cookie"), true));
            }
            ui.label("first.");
        });
    }

    pub fn show_import(
        &mut self,
        ui: &mut egui::Ui,
        session: Option<Session>,
        local: &mut Campaigns,
    ) {
        // Without a session there is nothing to list: the open Add campaign form says how to
        // import (`import_hint`).
        if session.is_none() {
            return;
        }
        let header = crate::design::Section::new("Add from D&D Beyond")
            .default_open(local.all().is_empty())
            .show(ui, |ui| {
                let Some(session) = session else {
                    ui.label("Connect your account first, or open an existing folder below.");
                    return;
                };
                if (!self.loaded && self.error.is_none() && self.pending.is_none())
                    || crate::design::Action::secondary("Refresh from D&D Beyond")
                        .density(crate::design::Density::Read)
                        .blocked_if(self.pending.is_some(), "Already refreshing.")
                        .show(ui)
                        .clicked()
                {
                    self.refresh(session, ui.ctx());
                }
                if self.pending.is_some() {
                    ui.horizontal(|ui| {
                        crate::design::spinner(ui);
                        crate::design::blocked_reason(ui, Some("Already refreshing."));
                    });
                }
                // Round 26: the list's error is in the pane's footer (`footer_messages`).
                if self.loaded && self.campaigns.is_empty() {
                    ui.label("No campaigns returned for this account.");
                }
                // Round 27 AN2: one action column, `L` after the widest name, so each Add or
                // Added sits near its name (it was at the column's far edge).
                let font = crate::design::Type::Strong.font(ui);
                let column = *self.name_column.get_or_insert_with(|| {
                    self.campaigns
                        .iter()
                        .map(|remote| crate::design::number_width(ui, &remote.name, &font))
                        .fold(0.0, f32::max)
                });
                for remote in &self.campaigns {
                    let added = local.is_added(&remote.id);
                    let row = ui.horizontal(|ui| {
                        use crate::design::{self, Action, L};
                        let item_gap = ui.spacing().item_spacing.x;
                        // Round 27 review: the action's room comes first, so Add (or Added) is
                        // always whole inside the column; the name gets what is left, and a name
                        // too long for it ends in `…`, whole on hover.
                        let action =
                            Action::secondary("Add…")
                                .natural_width(ui)
                                .max(design::number_width(
                                    ui,
                                    &format!("{} Added", design::icons::OK),
                                    &design::Type::Body.font(ui),
                                ));
                        let name_room = (ui.available_width() - action - L).max(0.0);
                        let words = design::one_line(
                            ui,
                            &remote.name,
                            font.clone(),
                            design::colors().text,
                            name_room,
                        );
                        let cut = words.elided;
                        // One action column, `L` after the widest name that fits: the name
                        // takes a cell of that width.
                        ui.spacing_mut().item_spacing.x = L;
                        let cell = egui::vec2(column.min(name_room), ui.spacing().interact_size.y);
                        ui.allocate_ui_with_layout(
                            cell,
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.set_min_size(cell);
                                ui.set_max_width(cell.x);
                                let name = ui.add(egui::Label::new(words));
                                if cut {
                                    name.on_hover_text(&remote.name);
                                }
                            },
                        );
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.spacing_mut().item_spacing.x = item_gap;
                            if added {
                                crate::design::status(
                                    ui,
                                    crate::design::Status {
                                        icon: crate::design::icons::OK,
                                        word: "Added",
                                        color: crate::design::colors().muted,
                                    },
                                );
                            } else if !local.importing(&remote.id) {
                                let add = crate::design::Action::secondary("Add…")
                                    .name(&format!("Add {} from D&D Beyond", remote.name))
                                    .show(ui);
                                if let Some(act) = crate::design::reveal::Act::clicked(ui, &add) {
                                    local.start_import(act, &remote.id, &remote.name);
                                }
                            }
                        });
                    });
                    crate::design::register_row(ui, row.response.rect);
                    local.show_import_form(ui, &remote.id);
                }
            });
        crate::design::locked_control(ui, &header.header_response);
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

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        session: Option<Session>,
        local: &mut Campaigns,
        blocked: Option<&str>,
    ) {
        // Ruling R24, row 93: the block's heading row is the one form of the Link and folder
        // page's three values; its note is the link's state.
        use crate::design::HeadingNote;
        let note = if self.pending.is_some() {
            HeadingNote::Busy("Refreshing…")
        } else if session.is_none() {
            HeadingNote::Muted("Not connected · showing the name saved on this computer")
        } else {
            HeadingNote::Status(crate::design::Status {
                icon: crate::design::icons::ONLINE,
                word: "Connected",
                color: crate::design::colors().connection.connected,
            })
        };
        let (block, _) = crate::design::ValueBlock::new(
            ui,
            crate::design::HeadingRow::new("D&D Beyond").note(note),
        );
        let (block, ()) = block.field(|ui| self.show_link(ui, session, local, blocked));
        block.end();
    }

    /// The link's field: the campaign picker, its refresh and its help.
    fn show_link(
        &mut self,
        ui: &mut egui::Ui,
        session: Option<Session>,
        local: &mut Campaigns,
        blocked: Option<&str>,
    ) {
        let Some(campaign) = local.active_campaign() else {
            ui.label("Select a local campaign to configure its D&D Beyond link.");
            return;
        };
        if session.is_none() {
            connection_help(ui);
        }
        // Round 28 FO7: the pane's lock (a run) blocks Refresh list as it blocks the picker.
        if crate::design::Action::secondary("Refresh list")
            .density(crate::design::Density::Read)
            .blocked(self.link_refresh_lock(blocked, session.is_some()))
            .show(ui)
            .clicked()
            && let Some(session) = session.clone()
        {
            self.refresh_link(session, local, ui.ctx());
        }
        // Round 26: the link's error and result are in the pane's footer (`footer_messages`).
        let current = campaign.dndbeyond_campaign_id;
        let mut selected = current.clone();
        // A link the list does not hold yet (not refreshed, or gone from the account) keeps
        // its saved name as a choice, so the control still names it.
        let unlisted = current
            .as_ref()
            .filter(|id| !self.campaigns.iter().any(|c| &c.id == *id))
            .map(|id| {
                let name = campaign
                    .dndbeyond_campaign_name
                    .clone()
                    .unwrap_or_else(|| format!("Campaign {id}"));
                (Some(id.clone()), name)
            });
        ui.horizontal(|ui| {
            let mut choices = crate::design::Dropdown::new(
                crate::field_id!("ddb_link").of(campaign.id),
                "D&D Beyond campaign link",
            )
            .choice(None, "Not linked")
            .choices(unlisted);
            for remote in &self.campaigns {
                choices = if local.linked_elsewhere(&remote.id) {
                    choices.blocked_choice(
                        Some(remote.id.clone()),
                        remote.name.as_str(),
                        "Already linked to another campaign.",
                    )
                } else {
                    choices.choice(Some(remote.id.clone()), remote.name.as_str())
                };
            }
            // Round 26 FN6: one width in every state. A lock keeps the form's width, so the
            // picker ends with the Session folder field; its reason is above.
            let dropdown = choices
                .width(ui.available_width().min(crate::design::FORM_WIDTH))
                .blocked(blocked)
                .show(ui, &mut selected);
            if ui.is_rect_visible(dropdown.response.rect) {
                self.link_in_view(session, local, ui.ctx());
            }
        });
        if selected != current {
            self.choose_link(local, selected);
        }
        if let Some(id) = &current {
            ui.horizontal_wrapped(|ui| {
                if let Some(remote) = self.campaigns.iter().find(|c| &c.id == id) {
                    if let Some(dm) = &remote.dm_username {
                        ui.label(format!("DM: {dm}"));
                    }
                    if let Some(count) = remote.player_count {
                        ui.label(format!("Players: {count}"));
                    }
                }
                if crate::design::link(ui, "Open D&D Beyond campaign").clicked() {
                    ui.ctx().open_url(egui::OpenUrl::new_tab(format!(
                        "https://www.dndbeyond.com/campaigns/{id}"
                    )));
                }
            });
        }
        if self.loaded && self.campaigns.is_empty() {
            ui.label("No campaigns returned for this account.");
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
        campaigns.poll(
            &egui::Context::default(),
            1,
            false,
            &mut Campaigns::default(),
        );
        assert!(tx.send(Ok(Vec::new())).is_err());
        assert!(campaigns.campaigns.is_empty());
    }
}
