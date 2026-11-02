use crate::{
    content_workspace::CampaignDocument,
    design::{self, Action, Density, Status, Tone, Type, colors, icons},
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
    confirm_remove: Option<Uuid>,
    /// The campaign just added, and where to go next.
    added: Option<String>,
}

/// The rail's campaign selector, with a name on two lines.
const SELECTOR_HEIGHT: f32 = 44.0;

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
            confirm_remove: None,
            added: None,
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
        let adding = self.editing.is_none();
        self.persist(data)?;
        self.clear_form();
        self.added = adding.then(|| {
            format!(
                "Added {}. Next: build an encounter in Encounters.",
                campaign.name
            )
        });
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
            return Err("This package is already registered.".into());
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
                "Keep package folders separate from campaign folders and other registered packages."
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
            .ok_or("Package is no longer registered.")?;
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
            confirm_remove: None,
            added: None,
        }
    }

    /// The same startup load as `default`, from a temporary settings file.
    #[cfg(test)]
    pub fn open_at(path: PathBuf) -> Self {
        let mut campaigns = Self::at(path);
        campaigns.reload();
        campaigns
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

    /// Any campaign in the list, active or not, already links to this one.
    pub fn is_added(&self, remote_id: &str) -> bool {
        let Ok(remote) = remote_id.parse::<u64>() else {
            return false;
        };
        self.data.campaigns.iter().any(|c| {
            c.dndbeyond_campaign_id
                .as_deref()
                .and_then(|id| id.parse().ok())
                == Some(remote)
        })
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

    /// The always-visible campaign switcher. `blocked` says why it cannot change now.
    pub fn selector(&mut self, ui: &mut egui::Ui, blocked: Option<&str>) {
        let current = self.data.active_campaign_id;
        let mut selected = current;
        let label = self
            .data
            .campaigns
            .iter()
            .find(|p| Some(p.id) == current)
            .map(|p| p.name.as_str())
            .unwrap_or("Select campaign");
        let reason = if self.writable {
            blocked
        } else {
            Some("Reload valid campaign settings in Settings first.")
        };
        ui.add_enabled_ui(self.writable && blocked.is_none(), |ui| {
            // A name wraps to two lines here, rather than ending in dots.
            // Small type, so a name fits in two lines. A fixed height, so the items below it stay
            // in one place, whichever campaign is open.
            let top = ui.cursor().top();
            let combo = egui::ComboBox::from_id_salt("active_campaign")
                .icon(design::combo_icon)
                .width(ui.available_width())
                .wrap()
                .selected_text(Type::Small.rich(ui, label).color(colors().text))
                .show_ui(ui, |ui| {
                    ui.set_min_width(220.0);
                    ui.selectable_value(&mut selected, None, "No campaign");
                    for campaign in &self.data.campaigns {
                        ui.selectable_value(&mut selected, Some(campaign.id), &campaign.name);
                    }
                });
            design::named(ui, &combo.response, &format!("Active campaign: {label}"));
            let response = combo.response;
            match reason {
                Some(reason) => response.on_disabled_hover_text(reason),
                None => response.on_hover_text(label),
            };
            let used = ui.cursor().top() - top;
            ui.add_space((SELECTOR_HEIGHT - used).max(0.0));
        });
        if selected != current {
            self.select(selected);
        }
    }

    fn select(&mut self, id: Option<Uuid>) {
        let mut data = self.data.clone();
        data.active_campaign_id = id;
        if let Err(error) = self.persist(data) {
            self.error = Some(error);
        }
    }

    /// The active campaign's folder and D&D Beyond link.
    pub fn show_active(
        &mut self,
        ui: &mut egui::Ui,
        session: Option<crate::auth::Session>,
        dndbeyond: &mut crate::dndbeyond::DndBeyond,
    ) {
        let c = colors();
        let Some(campaign) = self.active_campaign() else {
            ui.label(
                Type::Body
                    .rich(ui, "Select a campaign, or add one in Settings.")
                    .color(c.muted),
            );
            return;
        };
        ui.add_space(design::S);
        ui.set_max_width(ui.available_width().min(design::FORM_WIDTH + 120.0));
        ui.horizontal(|ui| {
            design::group_label(ui, "Folder");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if Action::quiet("Edit details")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.clear_form();
                    self.editing = Some(campaign.id);
                    self.name = campaign.name.clone();
                    self.path = campaign.path.to_string_lossy().into_owned();
                }
            });
        });
        ui.add(egui::Label::new(campaign.path.display().to_string()).wrap());
        Self::folder_status(ui, &campaign);
        ui.add_space(design::S);
        design::rule(ui);
        ui.add_space(design::S);
        dndbeyond.show(ui, session, self);
        ui.add_space(design::S);
        design::quiet_section(ui, "Technical details", |ui| {
            ui.label(format!("ID: {}", campaign.id));
            ui.label(
                Type::Small
                    .rich(ui, "Settings are stored in the campaign manifest.")
                    .color(c.muted),
            );
        });
    }

    fn folder_status(ui: &mut egui::Ui, campaign: &Campaign) -> egui::Response {
        let c = colors();
        if campaign.path.is_dir() {
            design::status(
                ui,
                Status {
                    icon: icons::OK,
                    word: "Folder available",
                    color: c.result.success,
                },
            )
        } else {
            design::status(
                ui,
                Status {
                    icon: icons::WARN,
                    word: "Folder unavailable",
                    color: c.result.warning,
                },
            )
        }
    }

    fn remove_dialog(&mut self, ctx: &egui::Context) {
        let Some(id) = self.confirm_remove else {
            return;
        };
        let Some(campaign) = self.data.campaigns.iter().find(|c| c.id == id).cloned() else {
            self.confirm_remove = None;
            return;
        };
        let mut keep = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let mut remove = false;
        design::dialog(
            ctx,
            "remove_campaign",
            &format!("Remove {}?", campaign.name),
            |ui| {
                ui.add(
                egui::Label::new(
                    "Splat forgets this campaign. Its encounters, runs and D&D Beyond link stay in its folder. To use it again, add the folder back.",
                )
                .wrap(),
            );
                ui.add(
                    egui::Label::new(
                        Type::Small
                            .rich(ui, campaign.path.display().to_string())
                            .color(colors().muted),
                    )
                    .wrap(),
                );
                ui.add_space(design::S);
                ui.horizontal(|ui| {
                    keep |= Action::secondary("Keep the campaign").show(ui).clicked();
                    ui.add_space(design::L);
                    remove = Action::destructive("Remove from Splat").show(ui).clicked();
                });
            },
        );
        if keep {
            self.confirm_remove = None;
        }
        if remove {
            self.confirm_remove = None;
            let mut data = self.data.clone();
            data.campaigns.retain(|c| c.id != id);
            if data.active_campaign_id == Some(id) {
                data.active_campaign_id = None;
            }
            self.save(data);
        }
    }

    /// The campaign list. Which exist, where, and how each is linked.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        session: Option<crate::auth::Session>,
        dndbeyond: &mut crate::dndbeyond::DndBeyond,
    ) {
        let c = colors();
        self.remove_dialog(ui.ctx());
        if let Some(added) = &self.added {
            design::message(ui, Tone::Success, added);
        }
        ui.horizontal(|ui| {
            ui.label(Type::Heading.rich(ui, "Campaigns").color(c.text));
            ui.label(
                Type::Small
                    .rich(ui, format!("{} configured", self.data.campaigns.len()))
                    .color(c.muted),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if Action::quiet("Reload")
                    .density(Density::Read)
                    .hint("Reload campaign settings from disk")
                    .show(ui)
                    .clicked()
                {
                    self.reload();
                }
            });
        });
        if let Some(error) = &self.error {
            design::message(ui, Tone::Error, error);
        }
        let unwritable = (!self.writable).then_some("Reload valid campaign settings first.");
        design::locked(ui, unwritable, |ui| {
            design::rule(ui);
            if self.data.campaigns.is_empty() {
                ui.add_space(design::XS);
                ui.label(
                    Type::Body
                        .rich(
                            ui,
                            "No campaigns yet. Add one below, from a folder or from D&D Beyond.",
                        )
                        .color(c.muted),
                );
            }
            let mut open = None;
            let mut edit = None;
            for campaign in self.data.campaigns.clone() {
                let active = self.data.active_campaign_id == Some(campaign.id);
                ui.push_id(campaign.id, |ui| {
                    ui.horizontal(|ui| {
                        ui.set_min_height(Density::Glance.row() + design::XS);
                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            // The active row has one more action to keep room for.
                            let actions = if active { 240.0 } else { 180.0 };
                            ui.set_max_width((ui.available_width() - actions).max(120.0));
                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::Label::new(
                                        Type::Name.rich(ui, &campaign.name).color(c.text),
                                    )
                                    .wrap(),
                                );
                                if active {
                                    ui.label(Type::Small.rich(ui, "Active").color(c.accent));
                                }
                            });
                            // The folder is worth a word only when it is missing. The link has its own line.
                            if !campaign.path.is_dir() {
                                Self::folder_status(ui, &campaign)
                                    .on_hover_text(campaign.path.display().to_string());
                            }
                            {
                                let link = campaign
                                    .dndbeyond_campaign_name
                                    .as_deref()
                                    .map(|name| format!("Linked: {name}"))
                                    .or(campaign
                                        .dndbeyond_campaign_id
                                        .as_deref()
                                        .map(|id| format!("Linked: campaign {id}")))
                                    .unwrap_or_else(|| "Not linked".to_owned());
                                ui.add(
                                    egui::Label::new(Type::Small.rich(ui, link).color(c.muted))
                                        .wrap(),
                                );
                            }
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Buttons keep every letter. The name and status give way first.
                            ui.spacing_mut().item_spacing.x = design::XS;
                            // Rarely wanted, and it asks first. It should not be the loudest
                            // thing on the row.
                            if Action::quiet("Remove…")
                                .density(Density::Read)
                                .name(&format!("Remove {} from Splat", campaign.name))
                                .show(ui)
                                .clicked()
                            {
                                self.confirm_remove = Some(campaign.id);
                            }
                            ui.add_space(design::M);
                            if active {
                                if Action::quiet("Edit details")
                                    .density(Density::Read)
                                    .show(ui)
                                    .clicked()
                                {
                                    edit = Some(campaign.clone());
                                }
                            } else if Action::secondary("Open")
                                .density(Density::Read)
                                .name(&format!("Make {} the active campaign", campaign.name))
                                .show(ui)
                                .clicked()
                            {
                                open = Some(campaign.id);
                            }
                        });
                    });
                });
                design::rule(ui);
            }
            if let Some(id) = open {
                self.select(Some(id));
            }
            if let Some(campaign) = edit {
                self.clear_form();
                self.editing = Some(campaign.id);
                self.name = campaign.name.clone();
                self.path = campaign.path.to_string_lossy().into_owned();
            }
            ui.add_space(design::S);
            dndbeyond.show_import(ui, session.clone(), self);
            self.form(ui);
        });
    }

    /// Edit the active campaign's details from the page that links to them.
    pub fn show_form_if_editing(&mut self, ui: &mut egui::Ui) {
        if self.editing.is_some() {
            ui.add_space(design::S);
            self.form(ui);
        }
    }

    fn form(&mut self, ui: &mut egui::Ui) {
        let editing = self.editing.is_some();
        egui::CollapsingHeader::new(if editing {
            "Edit campaign details"
        } else {
            "Add campaign"
        })
        .icon(design::section_icon)
        .icon(design::section_icon)
        .id_salt(("campaign_form", self.editing))
        .open(self.remote.as_ref().map(|_| true))
        .default_open(editing || self.data.campaigns.is_empty())
        .show(ui, |ui| {
            ui.add_space(design::XS);
            if let Some((id, name)) = &self.remote {
                ui.label(format!("{name} · D&D Beyond ID {id}"));
            }
            design::text_field(ui, "Campaign name", &mut self.name, None, None);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Max), |ui| {
                ui.vertical(|ui| {
                    ui.set_max_width((ui.available_width() - 112.0).max(120.0));
                    design::text_field(ui, "Folder", &mut self.path, None, None);
                });
                if Action::secondary("Browse…")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                    && let Some(path) = crate::hermetic::pick_folder()
                {
                    self.path = path.to_string_lossy().into_owned();
                }
            });
            ui.horizontal(|ui| {
                let submit = if editing {
                    "Save details"
                } else {
                    "Open campaign"
                };
                let missing = self.name.trim().is_empty() || self.path.trim().is_empty();
                if Action::primary(submit)
                    .blocked_if(missing, "Enter a campaign name and a folder first.")
                    .show(ui)
                    .clicked()
                    && let Err(error) = self.save_form()
                {
                    self.error = Some(error);
                }
                if (editing || !missing || self.remote.is_some())
                    && Action::quiet("Cancel").show(ui).clicked()
                {
                    self.clear_form();
                }
            });
            if let Some(error) = &self.error {
                design::message(ui, Tone::Error, error);
            }
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
            confirm_remove: None,
            added: None,
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
