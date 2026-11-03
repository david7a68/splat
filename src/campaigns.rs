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
    focus_name: bool,
    form_open: bool,
    add_name: String,
    add_path: String,
    name: String,
    path: String,
    remote: Option<(String, String)>,
    confirm_remove: Option<Uuid>,
    /// The campaign just added, and where to go next.
    added: Option<String>,
    /// The DM asked to go on to Encounters, from that message.
    pub wants_encounters: bool,
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
            focus_name: false,
            form_open: false,
            add_name: String::new(),
            add_path: String::new(),
            name: String::new(),
            path: String::new(),
            remote: None,
            confirm_remove: None,
            added: None,
            wants_encounters: false,
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
        let parent = self
            .data
            .campaigns
            .first()
            .and_then(|campaign| campaign.path.parent())
            .map(PathBuf::from)
            .or_else(|| {
                self.store
                    .as_ref()
                    .and_then(|store| store.path.parent().map(|path| path.join("campaigns")))
            });
        if let Some(parent) = parent {
            let safe: String = name
                .chars()
                .map(|ch| if matches!(ch, '/' | '\\') { '-' } else { ch })
                .collect();
            let mut candidate = parent.join(&safe);
            let mut suffix = 2;
            while candidate.exists() {
                candidate = parent.join(format!("{safe} {suffix}"));
                suffix += 1;
            }
            self.path = candidate.to_string_lossy().into_owned();
        }
        self.remote = Some((id.into(), name.into()));
    }

    pub fn cancel_import(&mut self) {
        if self.remote.is_some() {
            self.clear_form();
        }
    }

    fn save_form(&mut self) -> Result<(), String> {
        if self.remote.is_some() && self.editing.is_none() {
            std::fs::create_dir_all(self.path.trim())
                .map_err(|error| format!("Could not create the campaign folder: {error}"))?;
        }
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
        self.added = adding.then(|| format!("Added {}.", campaign.name));
        Ok(())
    }

    pub fn all(&self) -> &[Campaign] {
        &self.data.campaigns
    }

    pub fn package_folder(&self, name: &str) -> Result<PathBuf, String> {
        let store = self
            .store
            .as_ref()
            .ok_or("The settings folder is not available.")?;
        Ok(store
            .path
            .parent()
            .ok_or("The settings folder is not available.")?
            .join("packages")
            .join(name))
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

    /// Restore a missing package, including one whose registered folder moved.
    pub fn restore_module(&mut self, module: crate::persistence::Module) -> Result<(), String> {
        let mut data = self.data.clone();
        data.modules.retain(|known| known.id != module.id);
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
            focus_name: false,
            form_open: false,
            add_name: String::new(),
            add_path: String::new(),
            name: String::new(),
            path: String::new(),
            remote: None,
            confirm_remove: None,
            added: None,
            wants_encounters: false,
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
        if let Some(module) = self.data.modules.iter().find(|module| module.id == id) {
            // Keep the last registration name in older campaign manifests too.
            for campaign in &self.data.campaigns {
                let Ok(mut document) = CampaignDocument::load(campaign) else {
                    continue;
                };
                let mut changed = false;
                for selection in &mut document.selections {
                    if selection.package.id == Some(id)
                        && selection.name.as_ref() != Some(&module.name)
                    {
                        selection.name = Some(module.name.clone());
                        changed = true;
                    }
                }
                if changed {
                    document.save()?;
                }
            }
        }
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
        self.data.campaigns.iter().any(|c| links_to(c, remote_id))
    }

    pub fn linked_elsewhere(&self, remote_id: &str) -> bool {
        self.data
            .campaigns
            .iter()
            .any(|c| Some(c.id) != self.data.active_campaign_id && links_to(c, remote_id))
    }

    fn clear_form(&mut self) {
        let editing = self.editing.take().is_some();
        self.name = if editing {
            std::mem::take(&mut self.add_name)
        } else {
            String::new()
        };
        self.path = if editing {
            std::mem::take(&mut self.add_path)
        } else {
            String::new()
        };
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
    pub fn selector(&mut self, ui: &mut egui::Ui, blocked: Option<&str>) -> bool {
        let current = self.data.active_campaign_id;
        let mut add = false;
        let mut selected = current;
        let label = self
            .data
            .campaigns
            .iter()
            .find(|p| Some(p.id) == current)
            .map(|p| p.name.as_str())
            .unwrap_or(if self.data.campaigns.is_empty() {
                "No campaign"
            } else {
                "Select campaign"
            });
        let reason = if self.writable {
            blocked
        } else {
            Some("Reload valid campaign settings in Settings first.")
        };
        // The name, lock and caret share a fixed-height control.
        let top = ui.cursor().top();
        let locked = reason.is_some();
        let combo = egui::ComboBox::from_id_salt("active_campaign")
            .icon(move |ui, rect, visuals, open| {
                if locked {
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        icons::READ_ONLY,
                        Type::Body.font(ui),
                        colors().muted,
                    );
                } else {
                    design::combo_icon(ui, rect, visuals, open);
                }
            })
            .width(ui.available_width())
            .wrap()
            .selected_text(Type::Small.rich(ui, label).color(colors().text))
            .show_ui(ui, |ui| {
                ui.set_min_width(220.0);
                if let Some(reason) = reason {
                    design::blocked_reason(ui, Some(reason));
                    ui.add_space(design::L);
                }
                ui.add_enabled_ui(reason.is_none(), |ui| {
                    if self.data.campaigns.is_empty() {
                        if ui.selectable_label(false, "Add campaign…").clicked() {
                            add = true;
                            ui.close();
                        }
                    } else {
                        ui.selectable_value(&mut selected, None, "No campaign");
                    }
                    for campaign in &self.data.campaigns {
                        ui.selectable_value(&mut selected, Some(campaign.id), &campaign.name);
                    }
                });
            });
        design::named(ui, &combo.response, &format!("Active campaign: {label}"));
        if let Some(reason) = reason {
            ui.ctx()
                .accesskit_node_builder(combo.response.id, |node| node.set_description(reason));
        }
        combo.response.on_hover_text(reason.unwrap_or(label));
        let used = ui.cursor().top() - top;
        ui.add_space((SELECTOR_HEIGHT - used).max(0.0));
        if selected != current {
            self.select(selected);
        }
        add
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
        blocked: Option<&str>,
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
                    .blocked(blocked)
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    if self.editing.is_none() {
                        self.add_name = self.name.clone();
                        self.add_path = self.path.clone();
                    }
                    self.remote = None;
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
        let link_blocked = blocked.map(|reason| {
            if reason == "Close the run to change campaigns." {
                "Close the run to change the link."
            } else {
                reason
            }
        });
        dndbeyond.show(ui, session, self, link_blocked);
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
        egui::Modal::new(egui::Id::new("remove_campaign"))
            .frame(egui::Frame::new().fill(colors().raised)
                .stroke(egui::Stroke::new(design::HAIRLINE_WIDTH, colors().rule))
                .inner_margin(egui::Margin::same(design::L as i8)))
            .show(ctx, |ui| {
                ui.set_width(design::FORM_WIDTH.min(ctx.content_rect().width() - design::XXL * 2.0));
                ui.label(Type::Section.rich(ui, format!("Remove {}?", campaign.name)));
                ui.add_space(design::XS);
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
            });
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
        ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
        self.remove_dialog(ui.ctx());
        if let Some(added) = self.added.clone()
            && design::message_with_action(ui, Tone::Success, &added, "Build an encounter")
        {
            self.wants_encounters = true;
            self.added = None;
        }
        ui.horizontal(|ui| {
            design::group_heading(ui, "Campaigns", self.data.campaigns.len());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if !self.data.campaigns.is_empty()
                    && Action::quiet("Reload from disk")
                        .icon(icons::REFRESH)
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
            if self.data.campaigns.is_empty() {
                ui.add_space(design::XS);
                ui.label(
                    Type::Body
                        .rich(ui, "No campaign yet. Add one below.")
                        .color(c.muted),
                );
            }
            let mut open = None;
            for campaign in self.data.campaigns.clone() {
                let active = self.data.active_campaign_id == Some(campaign.id);
                ui.push_id(campaign.id, |ui| {
                    ui.horizontal(|ui| {
                        ui.set_min_height(Density::Glance.row() + design::XS);
                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            // The active row has one more action to keep room for.
                            let actions = if active { 150.0 } else { 220.0 };
                            ui.set_max_width((ui.available_width() - actions).max(120.0));
                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::Label::new(
                                        Type::Name.rich(ui, &campaign.name).color(c.text),
                                    )
                                    .wrap(),
                                );
                                if active {
                                    design::status(
                                        ui,
                                        Status {
                                            icon: icons::OK,
                                            word: "Active",
                                            color: c.text,
                                        },
                                    );
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
                                    .map(|name| format!("Linked to {name}"))
                                    .or(campaign
                                        .dndbeyond_campaign_id
                                        .as_deref()
                                        .map(|id| format!("Linked to campaign {id}")))
                                    .unwrap_or_else(|| "Not linked".to_owned());
                                design::status(
                                    ui,
                                    Status {
                                        icon: if campaign.dndbeyond_campaign_id.is_some() {
                                            icons::LINK
                                        } else {
                                            icons::SYNC_OFF
                                        },
                                        word: &link,
                                        color: c.muted,
                                    },
                                );
                            }
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Buttons keep every letter. The name and status give way first.
                            ui.spacing_mut().item_spacing.x = design::XS;
                            design::row_menu(ui, &campaign.name, |ui| {
                                if ui.button("Remove…").clicked() {
                                    self.confirm_remove = Some(campaign.id);
                                    ui.close();
                                }
                            });
                            if self.editing != Some(campaign.id)
                                && Action::quiet("Edit details")
                                    .density(Density::Read)
                                    .show(ui)
                                    .clicked()
                            {
                                if self.editing.is_none() {
                                    self.add_name = self.name.clone();
                                    self.add_path = self.path.clone();
                                }
                                self.remote = None;
                                self.editing = Some(campaign.id);
                                self.name = campaign.name.clone();
                                self.path = campaign.path.to_string_lossy().into_owned();
                            }
                            if !active
                                && Action::secondary("Open")
                                    .density(Density::Read)
                                    .name(&format!("Make {} the active campaign", campaign.name))
                                    .show(ui)
                                    .clicked()
                            {
                                open = Some(campaign.id);
                            }
                        });
                    });
                    if self.editing == Some(campaign.id) {
                        self.form(ui, false, true);
                    }
                });
                design::rule(ui);
            }
            if let Some(id) = open {
                self.select(Some(id));
            }
            ui.add_space(design::S);
            dndbeyond.show_import(ui, session.clone(), self);
            if self.editing.is_none() && self.remote.is_none() {
                self.form(ui, false, true);
            }
        });
    }

    pub fn importing(&self, remote: &str) -> bool {
        self.remote.as_ref().is_some_and(|(id, _)| id == remote)
    }

    pub fn show_import_form(&mut self, ui: &mut egui::Ui, remote_id: &str) {
        if self.remote.as_ref().is_some_and(|(id, _)| id == remote_id) {
            self.form(ui, false, true);
        }
    }

    pub fn focus_add(&mut self) {
        self.focus_name = true;
    }

    pub fn show_add_form(&mut self, ui: &mut egui::Ui) {
        self.form(ui, true, true);
    }

    /// Edit the active campaign's details from the page that links to them.
    pub fn show_form_if_editing(&mut self, ui: &mut egui::Ui) {
        if self.editing.is_some() {
            ui.add_space(design::S);
            self.form(ui, false, true);
        }
    }

    pub fn form_open(&self) -> bool {
        self.editing.is_some() || self.form_open
    }

    fn form(&mut self, ui: &mut egui::Ui, _inline: bool, primary: bool) {
        let editing = self.editing.is_some();
        // Keep the add form's open state when Settings changes between one and two columns.
        let form_scope = egui::Id::new(("campaign_form", self.editing));
        ui.scope_builder(egui::UiBuilder::new().id(form_scope), |ui| {
            let mut added = false;
            let direct = editing || self.remote.is_some() || self.data.campaigns.is_empty();
            let title = if editing {
                "Details".to_owned()
            } else if let Some((_, name)) = &self.remote {
                format!("Add {name}")
            } else {
                "Add campaign".to_owned()
            };
            let editing_id = self.editing;
            let focus = self.focus_name;
            let empty = self.data.campaigns.is_empty();
            let mut fields = |ui: &mut egui::Ui| {
                ui.add_space(design::XS);
                let name = design::text_field(ui, "Campaign name", &mut self.name, None, None);
                if std::mem::take(&mut self.focus_name) {
                    name.request_focus();
                }
                // The button sits on the field's line, and ends where the fields above end.
                let room = ui.available_width();
                let field = ui
                    .vertical(|ui| {
                        ui.set_max_width((room - BROWSE_WIDTH).max(120.0));
                        design::text_field(ui, "Folder", &mut self.path, None, None)
                    })
                    .inner;
                let browse = egui::Rect::from_min_size(
                    egui::pos2(field.rect.right() + design::S, field.rect.top()),
                    egui::vec2(BROWSE_WIDTH, field.rect.height()),
                );
                ui.scope_builder(egui::UiBuilder::new().max_rect(browse), |ui| {
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
                        "Add campaign"
                    };
                    let unchanged = self
                        .editing
                        .and_then(|id| {
                            self.data
                                .campaigns
                                .iter()
                                .find(|campaign| campaign.id == id)
                        })
                        .is_some_and(|campaign| {
                            campaign.name == self.name.trim()
                                && campaign.path == std::path::Path::new(self.path.trim())
                        });
                    let reason = match (self.name.trim().is_empty(), self.path.trim().is_empty()) {
                        (true, true) => Some("Enter a campaign name and a folder first."),
                        (true, false) => Some("Enter a campaign name first."),
                        (false, true) => Some("Choose a folder first."),
                        (false, false) if unchanged => Some("No changes to save."),
                        (false, false) => None,
                    };
                    let missing = reason.is_some();
                    let action = if primary {
                        Action::primary(submit)
                    } else {
                        Action::secondary(submit)
                    };
                    if unchanged {
                        design::save_row_state(
                            ui,
                            design::Status {
                                icon: design::icons::OK,
                                word: "No changes",
                                color: colors().muted,
                            },
                            false,
                            None,
                        );
                    } else {
                        design::blocked_reason(ui, reason);
                    }
                    if action.blocked(reason).show(ui).clicked() {
                        match self.save_form() {
                            Ok(()) => added = !editing,
                            Err(error) => self.error = Some(error),
                        }
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
            };
            let form = if direct {
                if editing {
                    design::group_heading(ui, &title, 0);
                }
                fields(ui);
                None
            } else {
                Some(
                    egui::CollapsingHeader::new("Add campaign")
                        .icon(design::section_icon)
                        .id_salt(("campaign_form", editing_id))
                        .open(focus.then_some(true))
                        .default_open(empty)
                        .show(ui, fields),
                )
            };
            self.form_open = !added
                && (direct
                    || form
                        .as_ref()
                        .is_some_and(|form| form.body_response.is_some()));
            if added && let Some(form) = form {
                let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
                    ui.ctx(),
                    form.header_response.id,
                    false,
                );
                state.set_open(false);
                state.store(ui.ctx());
            }
        });
    }
}

/// The Browse button and the gap before it, beside the folder field.
const BROWSE_WIDTH: f32 = 92.0;

/// By number, because a link can be stored with other formatting. An ID that is not a number
/// links to nothing, and neither does a campaign with no link.
fn links_to(campaign: &Campaign, remote_id: &str) -> bool {
    let Ok(remote) = remote_id.parse::<u64>() else {
        return false;
    };
    campaign
        .dndbeyond_campaign_id
        .as_deref()
        .and_then(|id| id.parse::<u64>().ok())
        == Some(remote)
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
            focus_name: false,
            form_open: false,
            add_name: String::new(),
            add_path: String::new(),
            name: String::new(),
            path: String::new(),
            remote: None,
            confirm_remove: None,
            added: None,
            wants_encounters: false,
        };
        campaigns.select(Some(first));
        campaigns.link(Some("12".into())).unwrap();
        campaigns.select(Some(second));
        assert!(campaigns.linked_elsewhere("12"));
        // Not a number: it links to nothing, even with a campaign that has no link.
        assert!(!campaigns.linked_elsewhere("not-a-number"));
        assert!(!campaigns.is_added("not-a-number"));
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
    #[test]
    fn unregistering_remembers_legacy_names_and_finding_a_folder_restores_one_registration() {
        let temp = tempfile::tempdir().unwrap();
        let folder = temp.path().join("campaign");
        std::fs::create_dir(&folder).unwrap();
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        campaigns.start_import("12", "Campaign");
        campaigns.path = folder.to_string_lossy().into_owned();
        campaigns.save_form().unwrap();
        let campaign = campaigns.active_campaign().unwrap();
        let package = crate::persistence::Module {
            id: Uuid::new_v4(),
            name: "Ashen Marches".into(),
            path: temp.path().join("old-folder"),
        };
        campaigns.register_module(package.clone()).unwrap();
        let mut document = CampaignDocument::load(&campaign).unwrap();
        document
            .selections
            .push(crate::content_workspace::PackageSelection {
                name: None,
                package: crate::content_workspace::EntryRef::id(package.id),
                selection: crate::content_workspace::Selection::All,
            });
        document.save().unwrap();
        campaigns.unregister_module(package.id).unwrap();
        let document = CampaignDocument::load(&campaign).unwrap();
        assert_eq!(
            document
                .selections
                .iter()
                .find(|selection| selection.package.id == Some(package.id))
                .unwrap()
                .name
                .as_deref(),
            Some("Ashen Marches")
        );
        assert!(campaigns.modules().is_empty());
        campaigns.restore_module(package.clone()).unwrap();
        let mut moved = package.clone();
        moved.path = temp.path().join("new-folder");
        campaigns.restore_module(moved.clone()).unwrap();
        assert_eq!(campaigns.modules(), &[moved]);
        let mut overlapping = package;
        overlapping.path = folder;
        assert!(campaigns.restore_module(overlapping).is_err());
        assert_eq!(campaigns.modules()[0].path, temp.path().join("new-folder"));
    }
}
