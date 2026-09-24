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
    visible_last_frame: bool,
    requested_visible_link: Option<(uuid::Uuid, String)>,
    pending_link: Option<(uuid::Uuid, String)>,
    import_selected: Option<RemoteCampaign>,
    import_path: String,
    picker: crate::folder_picker::FolderPicker,
}

impl DndBeyond {
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
            self.import_selected = None;
            self.import_path.clear();
            self.picker = Default::default();
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
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.strong("Add from D&D Beyond");
            let Some(session) = session else {
                ui.label(
                    "Connect your account in Settings, or open an existing campaign folder below.",
                );
                return;
            };
            if !self.loaded && self.pending.is_none() && self.error.is_none() {
                self.refresh(session.clone(), ui.ctx());
            }
            if self.pending.is_some() {
                ui.spinner();
            }
            if ui
                .add_enabled(
                    self.pending.is_none(),
                    egui::Button::new("Refresh campaigns"),
                )
                .clicked()
            {
                self.refresh(session, ui.ctx());
            }
            if let Some(error) = &self.error {
                ui.colored_label(egui::Color32::LIGHT_RED, error);
            }
            if self.loaded && self.campaigns.is_empty() {
                ui.label("No campaigns returned for this account.");
            }
            egui::ComboBox::from_id_salt("import_campaign")
                .selected_text(
                    self.import_selected
                        .as_ref()
                        .map_or("Select campaign", |c| c.name.as_str()),
                )
                .show_ui(ui, |ui| {
                    for remote in &self.campaigns {
                        if ui
                            .selectable_label(
                                self.import_selected
                                    .as_ref()
                                    .is_some_and(|c| c.id == remote.id),
                                format!("{} · {}", remote.name, remote.id),
                            )
                            .clicked()
                        {
                            self.import_selected = Some(remote.clone());
                            self.import_path.clear();
                            self.picker = Default::default();
                        }
                    }
                });
            if let Some(remote) = self.import_selected.clone() {
                ui.label(format!("D&D Beyond campaign ID: {}", remote.id));
                ui.label("Choose the folder for this campaign's data.");
                ui.horizontal(|ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.import_path)
                            .hint_text("Absolute folder path"),
                    );
                    self.picker.show(ui, &mut self.import_path);
                });
                ui.horizontal(|ui| {
                    if ui
                        .add_enabled(
                            !self.import_path.trim().is_empty(),
                            egui::Button::new("Open campaign"),
                        )
                        .clicked()
                    {
                        match local.open_folder(
                            std::path::Path::new(self.import_path.trim()),
                            Some((&remote.id, &remote.name)),
                        ) {
                            Ok(()) => {
                                self.import_selected = None;
                                self.import_path.clear();
                                self.error = None;
                                self.message = Some(
                                    "Campaign opened. Settings are stored in its folder.".into(),
                                );
                            }
                            Err(error) => self.error = Some(error),
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.import_selected = None;
                        self.import_path.clear();
                        self.picker = Default::default();
                    }
                });
            }
        });
    }

    pub fn show(&mut self, ui: &mut egui::Ui, session: Option<Session>, local: &mut Campaigns) {
        ui.horizontal(|ui| {
            ui.strong("D&D Beyond");
            if self.pending.is_some() {
                ui.spinner();
                ui.weak("Refreshing…");
            } else if session.is_none() {
                ui.weak("Offline · cached name");
            } else {
                ui.weak("Connected");
            }
        });
        let Some(campaign) = local.active_campaign() else {
            ui.label("Select a local campaign to configure its D&D Beyond link.");
            return;
        };
        if session.is_none() {
            ui.weak("Connect your account above to update this link.");
        }
        if ui
            .add_enabled(
                session.is_some() && self.pending.is_none(),
                egui::Button::new("Refresh list"),
            )
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
                    .or_else(|| campaign.dndbeyond_campaign_name.clone())
                    .unwrap_or_else(|| format!("Campaign {id}"))
            })
            .unwrap_or_else(|| "Not linked".into());
        let dropdown = egui::ComboBox::from_id_salt(("ddb_link", campaign.id))
            .width(ui.available_width().min(520.0))
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
        if selected != current {
            match local.link(selected.clone()) {
                Ok(()) => {
                    self.requested_visible_link = None;
                    self.message = Some("Campaign link saved.".into());
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
                ui.hyperlink_to(
                    "Open D&D Beyond campaign",
                    format!("https://www.dndbeyond.com/campaigns/{id}"),
                );
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
    fn selected_remote_renders_before_any_local_campaign_and_cancel_writes_nothing() {
        let temp = tempfile::tempdir().unwrap();
        let mut local = Campaigns::at(temp.path().join("settings.json"));
        let remote = RemoteCampaign {
            id: "12".into(),
            name: "Example campaign".into(),
            dm_username: None,
            player_count: None,
        };
        let mut beyond = DndBeyond {
            campaigns: vec![remote.clone()],
            loaded: true,
            import_selected: Some(remote),
            import_path: temp.path().to_string_lossy().into_owned(),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(720.0, 600.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    beyond.show_import(ui, Some(Session::test_session()), &mut local)
                });
            },
        );
        assert!(!output.shapes.is_empty());
        assert!(local.all().is_empty());
        assert!(!temp.path().join("Campaign.md").exists());
        // Disconnect cancels the pending selection just like Cancel does.
        beyond.poll(&ctx, 1, false, &mut local);
        assert!(beyond.import_selected.is_none());
        assert!(beyond.import_path.is_empty());
        assert!(!temp.path().join("settings.json").exists());
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
