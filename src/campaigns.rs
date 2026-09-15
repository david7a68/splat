use crate::persistence::{Campaign, Store, UserData};
use eframe::egui;
use std::path::PathBuf;
use uuid::Uuid;

pub struct Campaigns {
    store: Option<Store>,
    data: UserData,
    error: Option<String>,
    writable: bool,
    editing: Option<Uuid>,
    name: String,
    path: String,
}

impl Default for Campaigns {
    fn default() -> Self {
        let mut result = Self {
            store: None,
            data: UserData::default(),
            error: None,
            writable: false,
            editing: None,
            name: String::new(),
            path: String::new(),
        };
        match Store::user() {
            Ok(store) => {
                result.store = Some(store);
                result.reload();
            }
            Err(error) => result.error = Some(error),
        }
        result
    }
}

impl Campaigns {
    fn reload(&mut self) {
        if let Some(store) = &mut self.store {
            match store.load() {
                Ok(data) => {
                    self.data = data;
                    self.error = None;
                    self.writable = true;
                    self.clear_form();
                }
                Err(error) => {
                    self.error = Some(error);
                    self.writable = false;
                }
            }
        }
    }

    fn persist(&mut self, data: UserData) -> Result<(), String> {
        let store = self.store.as_mut().ok_or("Settings are unavailable.")?;
        store.save(&data)?;
        self.data = data;
        self.error = None;
        Ok(())
    }

    fn save(&mut self, data: UserData) {
        match self.persist(data) {
            Ok(()) => self.clear_form(),
            Err(error) => self.error = Some(error),
        }
    }

    pub fn link(&mut self, remote_id: Option<String>) -> Result<(), String> {
        let id = self
            .data
            .active_campaign_id
            .ok_or("Select a local campaign.")?;
        let mut data = self.data.clone();
        let campaign = data
            .campaigns
            .iter_mut()
            .find(|c| c.id == id)
            .ok_or("Campaign not found.")?;
        if campaign.dndbeyond_campaign_id != remote_id {
            campaign.dndbeyond_campaign_name = None;
        }
        campaign.dndbeyond_campaign_id = remote_id;
        self.persist(data)
    }

    pub fn cache_remote_name(
        &mut self,
        id: Uuid,
        remote_id: &str,
        name: &str,
    ) -> Result<(), String> {
        let mut data = self.data.clone();
        let Some(campaign) = data
            .campaigns
            .iter_mut()
            .find(|c| c.id == id && c.dndbeyond_campaign_id.as_deref() == Some(remote_id))
        else {
            return Ok(());
        };
        if campaign.dndbeyond_campaign_name.as_deref() == Some(name) {
            return Ok(());
        }
        campaign.dndbeyond_campaign_name = Some(name.into());
        self.persist(data)
    }

    pub fn linked_elsewhere(&self, remote_id: &str) -> bool {
        self.data.campaigns.iter().any(|c| {
            Some(c.id) != self.data.active_campaign_id
                && c.dndbeyond_campaign_id
                    .as_deref()
                    .and_then(|id| id.parse::<u64>().ok())
                    == remote_id.parse::<u64>().ok()
        })
    }

    fn clear_form(&mut self) {
        self.editing = None;
        self.name.clear();
        self.path.clear();
    }

    pub fn active_campaign(&self) -> Option<Campaign> {
        self.data
            .campaigns
            .iter()
            .find(|c| Some(c.id) == self.data.active_campaign_id)
            .cloned()
    }

    pub fn selector(&mut self, ui: &mut egui::Ui) {
        let current = self.data.active_campaign_id;
        let mut selected = current;
        let label = self
            .data
            .campaigns
            .iter()
            .find(|p| Some(p.id) == current)
            .map(|p| p.name.as_str())
            .unwrap_or("Select campaign");
        ui.add_enabled_ui(self.writable, |ui| {
            egui::ComboBox::from_id_salt("active_campaign")
                .selected_text(label)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut selected, None, "No campaign");
                    for campaign in &self.data.campaigns {
                        ui.selectable_value(&mut selected, Some(campaign.id), &campaign.name);
                    }
                });
        });
        if selected != current {
            self.select(selected);
        }
        if let Some(error) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
    }

    fn select(&mut self, id: Option<Uuid>) {
        let mut data = self.data.clone();
        data.active_campaign_id = id;
        if let Err(error) = self.persist(data) {
            self.error = Some(error);
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        session: Option<crate::auth::Session>,
        dndbeyond: &mut crate::dndbeyond::DndBeyond,
    ) {
        ui.horizontal(|ui| {
            ui.heading("Campaigns");
            ui.weak(format!("{} configured", self.data.campaigns.len()));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .small_button("Reload")
                    .on_hover_text("Reload campaign settings from disk")
                    .clicked()
                {
                    self.reload();
                }
            });
        });
        if let Some(error) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
        ui.add_enabled_ui(self.writable, |ui| {
            ui.horizontal(|ui| {
                ui.label("Active campaign");
                ui.push_id("settings_campaign_selector", |ui| self.selector(ui));
            });
            ui.add_space(8.0);
            if let Some(campaign) = self.active_campaign() {
                egui::Frame::group(ui.style()).inner_margin(12.0).show(ui, |ui| {
                    ui.set_min_width((ui.available_width() - 2.0).max(0.0));
                    ui.horizontal(|ui| {
                        ui.strong(&campaign.name);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("Edit details").clicked() {
                                self.editing = Some(campaign.id);
                                self.name = campaign.name.clone();
                                self.path = campaign.path.to_string_lossy().into_owned();
                            }
                        });
                    });
                    ui.add_space(6.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.weak("Folder");
                        ui.label(campaign.path.display().to_string());
                        if campaign.path.is_dir() {
                            ui.weak("Available");
                        } else {
                            ui.colored_label(egui::Color32::YELLOW, "Folder unavailable");
                        }
                    });
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);
                    dndbeyond.show(ui, session.clone(), self);
                    ui.add_space(8.0);
                    ui.collapsing("Campaign details", |ui| {
                        ui.label(format!("ID: {}", campaign.id));
                        if let Some(store) = &self.store { ui.weak(format!("Settings: {}", store.path.display())); }
                        if ui.small_button("Remove from Splat").on_hover_text("Removes this entry and link. The campaign folder and its files are kept.").clicked() {
                            let mut data = self.data.clone();
                            data.campaigns.retain(|c| c.id != campaign.id);
                            data.active_campaign_id = None;
                            self.save(data);
                        }
                    });
                });
            } else {
                ui.weak("Select a campaign above, or add one below.");
            }
            let editing = self.editing.is_some();
            egui::CollapsingHeader::new(if editing { "Edit campaign details" } else { "Add campaign" })
                .id_salt(("campaign_form", self.editing))
                .default_open(editing || self.data.campaigns.is_empty())
                .show(ui, |ui| {
            ui.add_space(8.0);
            egui::Grid::new("campaign_fields").num_columns(2).spacing([16.0, 8.0]).show(ui, |ui| {
                ui.label("Name");
                ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Campaign name").desired_width(ui.available_width()));
                ui.end_row();
                ui.label("Folder");
                ui.add(egui::TextEdit::singleline(&mut self.path).hint_text("Absolute folder path").desired_width(ui.available_width()));
                ui.end_row();
            });
            ui.horizontal(|ui| {
                if ui.button("Save campaign").clicked() {
                    let path = PathBuf::from(self.path.trim());
                    if self.name.trim().is_empty() || !path.is_absolute() {
                        self.error =
                            Some("Enter a campaign name and an absolute folder path.".into());
                    } else {
                        let mut data = self.data.clone();
                        if let Some(campaign) = data
                            .campaigns
                            .iter_mut()
                            .find(|p| Some(p.id) == self.editing)
                        {
                            campaign.name = self.name.trim().into();
                            campaign.path = path;
                        } else {
                            let id = Uuid::new_v4();
                            data.campaigns.push(Campaign {
                                id,
                                name: self.name.trim().into(),
                                path,
                                dndbeyond_campaign_id: None,
                                dndbeyond_campaign_name: None,
                            });
                            data.active_campaign_id = Some(id);
                        }
                        self.save(data);
                    }
                }
                if self.editing.is_some() && ui.button("Cancel edit").clicked() {
                    self.clear_form();
                }
            });
            });
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_to_one_links_replace_and_unlink_without_changing_other_campaigns() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let mut store = Store::at(path.clone());
        let mut data = store.load().unwrap();
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();
        for id in [first, second] {
            data.campaigns.push(Campaign {
                id,
                name: "Campaign".into(),
                path: dir.path().into(),
                dndbeyond_campaign_id: None,
                dndbeyond_campaign_name: None,
            });
        }
        let mut campaigns = Campaigns {
            store: Some(store),
            data,
            error: None,
            writable: true,
            editing: None,
            name: String::new(),
            path: String::new(),
        };
        campaigns.select(Some(first));
        campaigns.link(Some("12".into())).unwrap();
        campaigns.select(Some(second));
        assert!(campaigns.linked_elsewhere("12"));
        assert!(campaigns.link(Some("12".into())).is_err());
        assert!(
            campaigns
                .active_campaign()
                .unwrap()
                .dndbeyond_campaign_id
                .is_none()
        );
        campaigns.link(Some("34".into())).unwrap();
        campaigns
            .cache_remote_name(second, "34", "Cached name")
            .unwrap();
        assert_eq!(
            Store::at(path.clone()).load().unwrap().campaigns[1]
                .dndbeyond_campaign_name
                .as_deref(),
            Some("Cached name")
        );
        campaigns.link(Some("56".into())).unwrap();
        // A response for the replaced link must not repopulate its old name.
        campaigns
            .cache_remote_name(second, "34", "Late name")
            .unwrap();
        assert!(
            campaigns
                .active_campaign()
                .unwrap()
                .dndbeyond_campaign_name
                .is_none()
        );
        campaigns
            .cache_remote_name(second, "56", "New name")
            .unwrap();
        let saved = Store::at(path.clone()).load().unwrap();
        assert_eq!(
            saved.campaigns[0].dndbeyond_campaign_id.as_deref(),
            Some("12")
        );
        assert_eq!(
            saved.campaigns[1].dndbeyond_campaign_id.as_deref(),
            Some("56")
        );
        campaigns.link(None).unwrap();
        assert!(
            Store::at(path.clone()).load().unwrap().campaigns[1]
                .dndbeyond_campaign_name
                .is_none()
        );
        campaigns.select(Some(first));
        campaigns.link(None).unwrap();
        campaigns.select(Some(second));
        campaigns.link(Some("12".into())).unwrap();
        assert_eq!(
            Store::at(path).load().unwrap().campaigns[1]
                .dndbeyond_campaign_id
                .as_deref(),
            Some("12")
        );
    }
}
