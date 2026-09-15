use crate::{auth::Session, projects::Projects};
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
struct Campaign {
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

fn parse(bytes: &[u8]) -> Result<Vec<Campaign>, String> {
    #[derive(Deserialize)]
    struct Envelope {
        status: String,
        data: Option<Vec<Campaign>>,
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

fn fetch(session: Session) -> Result<Vec<Campaign>, String> {
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
pub struct Campaigns {
    account_epoch: u64,
    campaigns: Vec<Campaign>,
    selected: HashSet<String>,
    pending: Option<Receiver<Result<Vec<Campaign>, String>>>,
    loaded: bool,
    error: Option<String>,
    message: Option<String>,
}

impl Campaigns {
    pub fn poll(&mut self, ctx: &egui::Context, epoch: u64, connected: bool) {
        if epoch != self.account_epoch || !connected {
            // Dropping the receiver discards a previous account's late response.
            self.pending = None;
            self.campaigns.clear();
            self.selected.clear();
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
                            self.selected
                                .retain(|id| campaigns.iter().any(|c| &c.id == id));
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

    pub fn show(&mut self, ui: &mut egui::Ui, session: Option<Session>, projects: &mut Projects) {
        ui.heading("D&D Beyond campaigns");
        ui.label("Campaign details stay in memory. Only campaign IDs linked to local projects are saved.");
        let active = projects.active_projects();
        if active.is_empty() {
            ui.label("Select a local project from the dropdown before linking campaigns.");
        }
        if session.is_none() {
            ui.label("Connect to D&D Beyond in Settings to load campaigns.");
        }
        if ui
            .add_enabled(
                session.is_some() && self.pending.is_none(),
                egui::Button::new("Refresh campaigns"),
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
        egui::ScrollArea::vertical().show(ui, |ui| {
            if self.loaded && self.campaigns.is_empty() {
                ui.label("No campaigns were returned for this account.");
            }
            for campaign in &self.campaigns {
                ui.push_id(&campaign.id, |ui| {
                    let mut selected = self.selected.contains(&campaign.id);
                    if ui.checkbox(&mut selected, &campaign.name).changed() {
                        if selected {
                            self.selected.insert(campaign.id.clone());
                        } else {
                            self.selected.remove(&campaign.id);
                        }
                    }
                    ui.horizontal(|ui| {
                        if let Some(dm) = &campaign.dm_username {
                            ui.label(format!("DM: {dm}"));
                        }
                        if let Some(count) = campaign.player_count {
                            ui.label(format!("Players: {count}"));
                        }
                        ui.hyperlink_to(
                            "Open campaign",
                            format!("https://www.dndbeyond.com/campaigns/{}", campaign.id),
                        );
                    });
                });
            }
            if ui
                .add_enabled(
                    !active.is_empty() && !self.selected.is_empty(),
                    egui::Button::new("Link selected campaigns to this project"),
                )
                .clicked()
            {
                let mut ids: Vec<_> = self.selected.iter().cloned().collect();
                ids.sort();
                match projects.link_campaigns(&ids) {
                    Ok(()) => {
                        self.message = Some("Campaign links saved.".into());
                        self.error = None;
                    }
                    Err(error) => self.error = Some(error),
                }
            }
            ui.separator();
            ui.heading("Saved campaign links");
            for project in projects.active_projects() {
                ui.push_id(project.id, |ui| {
                    ui.label(&project.name);
                    if project.dndbeyond_campaign_ids.is_empty() {
                        ui.weak("No linked campaigns.");
                    }
                    for id in &project.dndbeyond_campaign_ids {
                        ui.push_id(id, |ui| {
                            ui.horizontal(|ui| {
                                let name = self
                                    .campaigns
                                    .iter()
                                    .find(|c| &c.id == id)
                                    .map(|c| c.name.as_str());
                                ui.label(
                                    name.map(|name| format!("{name} ({id})"))
                                        .unwrap_or_else(|| format!("Campaign {id}")),
                                );
                                if ui.button("Unlink").clicked() {
                                    match projects.unlink_campaign(project.id, id) {
                                        Ok(()) => {
                                            self.message = Some("Campaign link removed.".into());
                                            self.error = None;
                                        }
                                        Err(error) => self.error = Some(error),
                                    }
                                }
                            });
                        });
                    }
                });
            }
        });
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
        let mut campaigns = Campaigns {
            pending: Some(rx),
            ..Default::default()
        };
        campaigns.poll(&egui::Context::default(), 1, false);
        assert!(tx.send(Ok(Vec::new())).is_err());
        assert!(campaigns.campaigns.is_empty());
    }
}
