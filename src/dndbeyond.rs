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

pub(crate) fn campaign_id<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
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
    if cfg!(test) {
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
        ui.add_space(crate::design::XS);
        for word in "to update this link.".split_inclusive(' ') {
            ui.label(
                crate::design::Type::Small
                    .rich(ui, word)
                    .color(crate::design::colors().muted),
            );
        }
    });
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
    /// The link's result, with the order the link took for the footer.
    message: Option<(String, crate::design::ResultOrder)>,
    visible_last_frame: bool,
    requested_visible_link: Option<(uuid::Uuid, String)>,
    pending_link: Option<(uuid::Uuid, String)>,
}

impl DndBeyond {
    /// Give one remote campaign another name, as a new list from D&D Beyond would. Tests only:
    /// the test list is fixed (`fetch`).
    #[cfg(test)]
    pub fn rename_remote(&mut self, id: &str, name: &str) {
        for remote in self.campaigns.iter_mut().filter(|remote| remote.id == id) {
            remote.name = name.to_owned();
        }
        self.name_column = None;
    }

    /// A list is on its way, so what is on screen is due to change.
    #[cfg(test)]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    pub fn begin_frame(&mut self) {
        if !self.visible_last_frame {
            self.requested_visible_link = None;
        }
        self.visible_last_frame = false;
    }

    pub fn poll(
        &mut self,
        ctx: &egui::Context,
        epoch: u64,
        connected: bool,
        local: &mut Campaigns,
    ) {
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
        self.message = None;
        let ctx = ctx.clone();
        thread::spawn(move || {
            let _ = tx.send(fetch(session));
            ctx.request_repaint();
        });
    }

    pub fn show_import(
        &mut self,
        ui: &mut egui::Ui,
        session: Option<Session>,
        local: &mut Campaigns,
    ) {
        if session.is_none() {
            let heading_visible = ui
                .ctx()
                .data(|data| data.get_temp::<f32>(egui::Id::new("account_heading_content_y")))
                .is_some_and(|y| {
                    ui.clip_rect()
                        .contains(egui::pos2(ui.clip_rect().center().x, y))
                });
            if ui.ctx().content_rect().width() >= 1000.0 || heading_visible {
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
            return;
        }
        let header = egui::CollapsingHeader::new("Add from D&D Beyond")
            .icon(crate::design::section_icon)
            .icon(crate::design::section_icon)
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
                    ui.horizontal(|ui| {
                        use crate::design::{self, Action, L};
                        let left = ui.cursor().left();
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
                        let name = ui.add(egui::Label::new(words));
                        if cut {
                            name.on_hover_text(&remote.name);
                        }
                        // One action column, `L` after the widest name that fits.
                        let at = left + column.min(name_room) + L;
                        // The cursor is already one item gap past the name.
                        ui.add_space((at - ui.cursor().left()).max(0.0));
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            if added {
                                crate::design::status(
                                    ui,
                                    crate::design::Status {
                                        icon: crate::design::icons::OK,
                                        word: "Added",
                                        color: crate::design::colors().muted,
                                    },
                                );
                            } else if !local.importing(&remote.id)
                                && crate::design::Action::secondary("Add…")
                                    .name(&format!("Add {} from D&D Beyond", remote.name))
                                    .show(ui)
                                    .clicked()
                            {
                                local.start_import(&remote.id, &remote.name);
                            }
                        });
                    });
                    local.show_import_form(ui, &remote.id);
                }
            });
        crate::design::locked_control(ui, &header.header_response);
    }

    /// Round 26: the D&D Beyond list's error and, on Link and folder (`link`), the link's result,
    /// in the footer of the pane that shows them. Both live until the next refresh or link.
    pub fn footer_messages<'a>(&'a self, footer: &mut crate::design::Footer<'a>, link: bool) {
        if let Some(error) = &self.error {
            footer.reason(error.as_str(), move |ui| {
                crate::design::message(ui, crate::design::Tone::Error, error);
            });
        }
        if link && let Some((message, order)) = &self.message {
            footer.result(*order, message.as_str(), false, move |ui| {
                crate::design::message(ui, crate::design::Tone::Success, message);
            });
        }
    }

    #[allow(clippy::disallowed_methods)]
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        session: Option<Session>,
        local: &mut Campaigns,
        blocked: Option<&str>,
    ) {
        ui.horizontal(|ui| {
            crate::design::group_heading(ui, "D&D Beyond", 0);
            if self.pending.is_some() {
                crate::design::spinner(ui);
                ui.weak("Refreshing…");
            } else if session.is_none() {
                ui.weak("Not connected · showing the name saved on this computer");
            } else {
                crate::design::status(
                    ui,
                    crate::design::Status {
                        icon: crate::design::icons::ONLINE,
                        word: "Connected",
                        color: crate::design::colors().connection.connected,
                    },
                );
            }
        });
        let Some(campaign) = local.active_campaign() else {
            ui.label("Select a local campaign to configure its D&D Beyond link.");
            return;
        };
        if session.is_none() {
            connection_help(ui);
        }
        if crate::design::Action::secondary("Refresh list")
            .density(crate::design::Density::Read)
            .blocked(if session.is_none() {
                Some("Connect to D&D Beyond in Settings first.")
            } else if self.pending.is_some() {
                Some("Already refreshing.")
            } else {
                None
            })
            .show(ui)
            .clicked()
            && let Some(session) = session.clone()
        {
            self.pending_link = campaign
                .dndbeyond_campaign_id
                .clone()
                .map(|id| (campaign.id, id));
            self.requested_visible_link = self.pending_link.clone();
            self.refresh(session, ui.ctx());
        }
        // Round 26: the link's error and result are in the pane's footer (`footer_messages`).
        let current = campaign.dndbeyond_campaign_id;
        let mut selected = current.clone();
        let label = current
            .as_ref()
            .map(|id| {
                self.campaigns
                    .iter()
                    .find(|c| &c.id == id)
                    .map(|c| c.name.clone())
                    .or_else(|| campaign.dndbeyond_campaign_name.clone())
                    .unwrap_or_else(|| format!("Campaign {id}"))
            })
            .unwrap_or_else(|| "Not linked".into());
        ui.horizontal(|ui| {
            let dropdown = crate::design::locked(ui, blocked, |ui| {
                egui::ComboBox::from_id_salt(("ddb_link", campaign.id))
                    .icon(crate::design::combo_icon)
                    // Round 26 FN6: one width in every state. A lock keeps the form's width,
                    // so the picker ends with the Session folder field; its reason is above.
                    .width(ui.available_width().min(crate::design::FORM_WIDTH))
                    .selected_text(crate::design::Type::Body.rich(ui, label).color(
                        if ui.is_enabled() {
                            crate::design::colors().text
                        } else {
                            crate::design::colors().disabled
                        },
                    ))
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
                    })
            });
            crate::design::named(ui, &dropdown.response, "D&D Beyond campaign link");
            if ui.is_rect_visible(dropdown.response.rect) {
                self.visible_last_frame = true;
                let key = current.clone().map(|id| (campaign.id, id));
                if key.is_some()
                    && key != self.requested_visible_link
                    && self.pending.is_none()
                    && let Some(session) = session
                {
                    self.requested_visible_link = key.clone();
                    self.pending_link = key;
                    self.refresh(session, ui.ctx());
                }
            }
        });
        if selected != current {
            match local.link(selected.clone()) {
                Ok(()) => {
                    self.requested_visible_link = None;
                    self.message = Some((
                        "Campaign link saved.".into(),
                        crate::design::ResultOrder::issue(),
                    ));
                    self.error = None;
                    if let Some(remote) = self
                        .campaigns
                        .iter()
                        .find(|c| Some(&c.id) == selected.as_ref())
                        && let Err(error) =
                            local.cache_remote_name(campaign.id, &remote.id, &remote.name)
                    {
                        self.error = Some(error);
                    }
                }
                Err(error) => self.error = Some(error),
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter1_c_offline_link_opens_account_settings() {
        use egui_kittest::kittest::Queryable;
        let mut harness = egui_kittest::Harness::builder().build(|ctx| {
            egui::CentralPanel::default().show(ctx, connection_help);
        });
        crate::design::theme::install(&harness.ctx);
        harness.run_steps(2);
        harness.get_by_label("Settings").click();
        harness.run_steps(2);
        assert!(
            harness
                .ctx
                .data(|data| data.get_temp::<bool>(egui::Id::new("open_account_settings")))
                .unwrap_or(false)
        );
    }

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
