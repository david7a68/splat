use crate::{
    content_workspace::CampaignDocument,
    persistence::{Campaign, Store, UserData},
};
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
    remote: Option<(String, String)>,
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
            remote: None,
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
    pub fn start_import(&mut self, id: &str, name: &str) {
        self.clear_form();
        self.name = name.into();
        self.remote = Some((id.into(), name.into()));
    }

    pub fn cancel_import(&mut self) {
        if self.remote.is_some() {
            self.clear_form();
        }
    }

    fn save_form(&mut self) -> Result<(), String> {
        let path = PathBuf::from(self.path.trim())
            .canonicalize()
            .map_err(|_| "Choose an existing campaign folder.")?;
        let found = CampaignDocument::discover(&path)?;
        if self.editing.is_some() && found.is_none() {
            return Err("Choose the folder containing this campaign’s manifest.".into());
        }
        let mut campaign = found.unwrap_or_else(|| Campaign {
            id: self.editing.unwrap_or_else(Uuid::new_v4),
            name: self.name.trim().into(),
            path,
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        });
        if let Some(id) = self.editing {
            if campaign.id != id {
                return Err("This folder belongs to another campaign.".into());
            }
            campaign.name = self.name.trim().into();
        }
        if let Some((id, name)) = &self.remote {
            if campaign
                .dndbeyond_campaign_id
                .as_ref()
                .is_some_and(|current| current != id)
            {
                return Err("This folder is linked to another D&D Beyond campaign.".into());
            }
            campaign.dndbeyond_campaign_id = Some(id.clone());
            campaign.dndbeyond_campaign_name = Some(name.clone());
        }
        let mut data = self.data.clone();
        if let Some(old) = data.campaigns.iter_mut().find(|c| c.id == campaign.id) {
            if old
                .path
                .canonicalize()
                .ok()
                .is_some_and(|path| path != campaign.path)
            {
                return Err("This campaign is registered at another folder. Move that folder or remove its registration first.".into());
            }
            *old = campaign.clone();
        } else {
            data.campaigns.push(campaign.clone());
        }
        data.active_campaign_id = Some(campaign.id);
        self.persist(data)?;
        self.clear_form();
        Ok(())
    }

    pub fn all(&self) -> &[Campaign] {
        &self.data.campaigns
    }

    pub fn modules(&self) -> &[crate::persistence::Module] {
        &self.data.modules
    }

    pub fn register_module(&mut self, module: crate::persistence::Module) -> Result<(), String> {
        if self
            .data
            .modules
            .iter()
            .any(|m| m.id == module.id || m.path == module.path)
        {
            return Err("This module is already registered.".into());
        }
        if self
            .data
            .modules
            .iter()
            .any(|m| module.path.starts_with(&m.path) || m.path.starts_with(&module.path))
            || self.data.campaigns.iter().any(|c| {
                let path = c.path.canonicalize().unwrap_or_else(|_| c.path.clone());
                module.path.starts_with(&path) || path.starts_with(&module.path)
            })
        {
            return Err(
                "Keep module folders separate from campaign folders and other registered modules."
                    .into(),
            );
        }
        let mut data = self.data.clone();
        data.modules.push(module);
        self.persist(data)
    }

    pub fn rename_module(&mut self, id: Uuid, name: String) -> Result<(), String> {
        let mut data = self.data.clone();
        let module = data
            .modules
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or("Module is no longer registered.")?;
        module.name = name;
        self.persist(data)
    }

    #[cfg(test)]
    pub fn at(path: PathBuf) -> Self {
        let mut store = Store::at(path);
        let data = store.load().unwrap();
        Self {
            store: Some(store),
            data,
            error: None,
            writable: true,
            editing: None,
            name: String::new(),
            path: String::new(),
            remote: None,
        }
    }

    pub fn unregister_module(&mut self, id: Uuid) -> Result<(), String> {
        let mut data = self.data.clone();
        data.modules.retain(|m| m.id != id);
        self.persist(data)
    }

    fn reload(&mut self) {
        if let Some(store) = &mut self.store {
            match store.load() {
                Ok(mut data) => {
                    if let Err(error) = data
                        .campaigns
                        .iter_mut()
                        .try_for_each(CampaignDocument::reload_metadata)
                        .and_then(|()| data.validate())
                    {
                        self.error = Some(error);
                        self.writable = false;
                        return;
                    }
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
        if !self.writable {
            return Err("Reload valid campaign settings before saving.".into());
        }
        let store = self.store.as_mut().ok_or("Settings are unavailable.")?;
        store.check_save(&data)?;
        for campaign in &data.campaigns {
            let previous = self.data.campaigns.iter().find(|c| c.id == campaign.id);
            if previous != Some(campaign) {
                CampaignDocument::save_metadata(campaign, previous)?;
            }
        }
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
        self.remote = None;
    }

    pub fn active_campaign(&self) -> Option<Campaign> {
        if !self.writable {
            return None;
        }
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
            dndbeyond.show_import(ui, session.clone(), self);
            ui.add_space(8.0);
            if let Some(campaign) = self.active_campaign() {
                egui::Frame::group(ui.style()).inner_margin(12.0).show(ui, |ui| {
                    ui.set_min_width((ui.available_width() - 2.0).max(0.0));
                    ui.horizontal(|ui| {
                        ui.strong(&campaign.name);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("Edit details").clicked() {
                                self.clear_form();
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
                        ui.weak("Settings are stored in the campaign manifest.");
                        if ui.small_button("Remove from Splat").on_hover_text("Removes this registration. The campaign folder and link are kept.").clicked() {
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
                .open(self.remote.as_ref().map(|_| true))
                .default_open(editing || self.data.campaigns.is_empty())
                .show(ui, |ui| {
            ui.add_space(8.0);
            if let Some((id, name)) = &self.remote { ui.label(format!("{name} · D&D Beyond ID {id}")); }
            egui::Grid::new("campaign_fields").num_columns(2).spacing([16.0, 8.0]).show(ui, |ui| {
                ui.label("Name");
                ui.add(egui::TextEdit::singleline(&mut self.name).hint_text("Campaign name").desired_width(ui.available_width()));
                ui.end_row();
                ui.label("Folder");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.path);
                    if ui.button("Browse…").clicked() && let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.path = path.to_string_lossy().into_owned();
                    }
                });
                ui.end_row();
            });
            ui.horizontal(|ui| {
                if ui.button("Open campaign").clicked() && let Err(error) = self.save_form() {
                    self.error = Some(error);
                }
                if ui.button("Cancel").clicked() {
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
    fn import_and_reopen_preserve_campaign_identity_and_custom_settings() {
        let temp = tempfile::tempdir().unwrap();
        let folder = temp.path().join("campaign");
        std::fs::create_dir(&folder).unwrap();
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        campaigns.start_import("12", "Remote name");
        campaigns.path = folder.to_string_lossy().into_owned();
        campaigns.save_form().unwrap();
        let campaign = campaigns.active_campaign().unwrap();
        let file = folder.join("Campaign.md");
        let text = std::fs::read_to_string(&file).unwrap().replace(
            "splat_schema: 1",
            "splat_schema: 1\nsplat_run_storage: {directory: Sessions/Combat}",
        ) + "\nMy notes.\n";
        std::fs::write(&file, &text).unwrap();
        let mut reopened = Campaigns::at(temp.path().join("other-settings.json"));
        reopened.path = folder.to_string_lossy().into_owned();
        reopened.save_form().unwrap();
        assert_eq!(reopened.active_campaign(), Some(campaign));
        assert_eq!(std::fs::read_to_string(&file).unwrap(), text);
        reopened.start_import("34", "Other campaign");
        reopened.path = folder.to_string_lossy().into_owned();
        assert!(reopened.save_form().is_err());
        assert_eq!(std::fs::read_to_string(&file).unwrap(), text);
    }

    #[test]
    fn one_to_one_links_replace_and_unlink_without_changing_other_campaigns() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let mut store = Store::at(path.clone());
        let mut data = store.load().unwrap();
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();
        for id in [first, second] {
            let folder = dir.path().join(id.to_string());
            std::fs::create_dir(&folder).unwrap();
            data.campaigns.push(Campaign {
                id,
                name: "Campaign".into(),
                path: folder,
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
            remote: None,
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
