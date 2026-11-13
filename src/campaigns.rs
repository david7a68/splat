#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
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
    manifest_errors: std::collections::BTreeMap<Uuid, String>,
    held_manifests: std::cell::RefCell<std::collections::BTreeMap<Uuid, (String, String)>>,
    /// The copy's registration that `Undo register campaign` takes back: the copy's own file
    /// changes, its campaign ID, and the active campaign before it.
    campaign_copy: Option<(crate::content_workspace::PackageCopy, Uuid, Option<Uuid>)>,
    writable: bool,
    editing: Option<Uuid>,
    focus_name: bool,
    form_open: bool,
    add_name: String,
    add_path: String,
    name: String,
    path: String,
    discarded_details: Option<(String, String)>,
    /// When the details draft was discarded: its result's place in the footer.
    discarded_order: crate::design::ResultOrder,
    remote: Option<(String, String)>,
    confirm_remove: Option<Uuid>,
    /// The campaign just added, and where to go next.
    added: Option<String>,
    /// When that campaign was added: its result's place in the footer.
    added_order: crate::design::ResultOrder,
    /// The DM asked to go on to Encounters, from that message.
    pub wants_encounters: bool,
    /// Round 26 AG1: the widest name and link block and the widest action group, as the last
    /// frame drew them. The list is as wide as these with an `L` gap, so the actions follow
    /// the block. Drawing the rows measures them; nothing else does.
    list_measure: (f32, f32),
}

/// The rail's campaign selector, with a name on two lines.
const SELECTOR_HEIGHT: f32 = 44.0;
/// A missing Campaign.md without a copy. The write is a choice the DM makes with the button.
pub(crate) const NO_CAMPAIGN_COPY: &str = "Campaign.md is missing, and Splat has no copy. You can write a new one with the default packages.";

impl Default for Campaigns {
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn default() -> Self {
        let mut result = Self {
            store: None,
            data: UserData::default(),
            error: None,
            manifest_errors: Default::default(),
            held_manifests: Default::default(),
            campaign_copy: None,
            writable: false,
            editing: None,
            focus_name: false,
            form_open: false,
            add_name: String::new(),
            add_path: String::new(),
            name: String::new(),
            path: String::new(),
            discarded_details: None,
            discarded_order: crate::design::ResultOrder::issue(),
            remote: None,
            confirm_remove: None,
            added: None,
            added_order: crate::design::ResultOrder::issue(),
            wants_encounters: false,
            list_measure: (0.0, 0.0),
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

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
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
        // A new result replaces the copy's registration result, and its Undo with it.
        self.campaign_copy = None;
        self.added = adding.then(|| format!("Added {}.", campaign.name));
        self.added_order = crate::design::ResultOrder::issue();
        Ok(())
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn register_campaign_copy(&mut self) -> Result<(), String> {
        let path = PathBuf::from(self.path.trim());
        let found =
            CampaignDocument::discover(&path)?.ok_or("Choose a copied campaign folder first.")?;
        if !self
            .data
            .campaigns
            .iter()
            .any(|known| known.id == found.id && known.path != found.path)
        {
            return Err("This folder is not a copy of a registered campaign.".into());
        }
        let copy =
            crate::content_workspace::PackageCopy::write_owner(&path, &self.name, "campaign")?;
        let campaign = CampaignDocument::discover(&path)?.ok_or("Campaign.md is missing.")?;
        let name = campaign.name.clone();
        let id = campaign.id;
        let before = self.data.clone();
        let mut after = before.clone();
        after.active_campaign_id = Some(campaign.id);
        after.campaigns.push(campaign);
        if let Err(error) = self.persist(after) {
            copy.undo()?;
            return Err(error);
        }
        self.campaign_copy = Some((copy, id, before.active_campaign_id));
        self.clear_form();
        self.added = Some(copy_registered(&name));
        self.added_order = crate::design::ResultOrder::issue();
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

    pub fn unregistered_module(&self, id: Uuid) -> Option<&crate::persistence::Module> {
        self.data
            .unregistered_modules
            .iter()
            .find(|module| module.id == id)
    }

    pub fn register_module(&mut self, module: crate::persistence::Module) -> Result<(), String> {
        if let Some(registered) = self
            .data
            .modules
            .iter()
            .find(|m| m.id == module.id || m.path == module.path)
        {
            return Err(format!(
                "This folder has the same package identity as {} ({}). Choose a package with a different identity.",
                registered.name, registered.id
            ));
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
        let namespace_of = |module: &crate::persistence::Module| {
            splat::content::read_directory(&module.path)
                .ok()?
                .0
                .into_iter()
                .find(|entry| entry.kind == "package")?
                .fields
                .get("splat_namespace")?
                .as_str()
                .map(str::to_owned)
        };
        if let Some(namespace) = namespace_of(&module)
            && self
                .data
                .modules
                .iter()
                .any(|known| namespace == namespace_of(known).unwrap_or_default())
        {
            return Err(format!(
                "The namespace {namespace} is already registered. Use a different namespace."
            ));
        }
        let mut data = self.data.clone();
        data.unregistered_modules
            .retain(|known| known.id != module.id);
        data.modules.push(module);
        self.persist(data)
    }

    /// Restore a missing package, including one whose registered folder moved.
    pub fn restore_module(&mut self, module: crate::persistence::Module) -> Result<(), String> {
        let mut data = self.data.clone();
        data.unregistered_modules
            .retain(|known| known.id != module.id);
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
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    pub fn at(path: PathBuf) -> Self {
        let mut store = Store::at(path);
        let data = store.load().unwrap();
        Self {
            store: Some(store),
            data,
            error: None,
            manifest_errors: Default::default(),
            held_manifests: Default::default(),
            campaign_copy: None,
            writable: true,
            editing: None,
            focus_name: false,
            form_open: false,
            add_name: String::new(),
            add_path: String::new(),
            name: String::new(),
            path: String::new(),
            discarded_details: None,
            discarded_order: crate::design::ResultOrder::issue(),
            remote: None,
            confirm_remove: None,
            added: None,
            added_order: crate::design::ResultOrder::issue(),
            wants_encounters: false,
            list_measure: (0.0, 0.0),
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
        if let Some(module) = data.modules.iter().find(|module| module.id == id).cloned() {
            data.unregistered_modules.retain(|known| known.id != id);
            data.unregistered_modules.push(module);
        }
        data.modules.retain(|m| m.id != id);
        self.persist(data)
    }

    fn reload(&mut self) {
        if let Some(store) = &mut self.store {
            match store.load() {
                Ok(mut data) => {
                    self.manifest_errors.clear();
                    for campaign in &mut data.campaigns {
                        if let Err(error) = CampaignDocument::reload_metadata(campaign) {
                            self.manifest_errors.insert(
                                campaign.id,
                                format!("{}: {error}", campaign.path.display()),
                            );
                        } else if let Ok(document) = CampaignDocument::load(campaign) {
                            self.held_manifests
                                .get_mut()
                                .insert(campaign.id, (document.path, document.text));
                        }
                    }
                    if let Err(error) = data.validate() {
                        self.error = Some(error);
                        self.writable = false;
                        return;
                    }
                    self.data = data;
                    self.error = (!self.manifest_errors.is_empty()).then(|| {
                        self.manifest_errors
                            .values()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join("\n")
                    });
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
        let mut changes = Vec::new();
        for campaign in &data.campaigns {
            let previous = self.data.campaigns.iter().find(|c| c.id == campaign.id);
            if previous != Some(campaign) {
                changes.push(CampaignDocument::metadata_change(campaign, previous)?);
            }
        }
        store.save_with(&data, changes)?;
        self.data = data;
        for campaign in &self.data.campaigns {
            if let Ok(document) = CampaignDocument::load(campaign) {
                self.held_manifests
                    .get_mut()
                    .insert(campaign.id, (document.path, document.text));
            }
        }
        self.error = None;
        Ok(())
    }

    pub fn remember_campaign_document(&self, document: &CampaignDocument) {
        self.held_manifests.borrow_mut().insert(
            document.campaign.id,
            (document.path.clone(), document.text.clone()),
        );
    }

    /// The whole message for a missing Campaign.md, and the action that writes it.
    pub fn campaign_write_preview(&self, campaign: &Campaign) -> (String, &'static str) {
        if let Some((path, text)) = self.held_manifests.borrow().get(&campaign.id)
            && let Ok(entry) = splat::content::Entry::parse(path, text)
        {
            let packages = entry
                .fields
                .get("splat_packages")
                .and_then(serde_yaml_ng::Value::as_sequence)
                .map_or(0, Vec::len);
            let rules = entry
                .fields
                .get("splat_rule_choices")
                .and_then(serde_yaml_ng::Value::as_mapping)
                .map_or(0, serde_yaml_ng::Mapping::len);
            let counts = [
                design::count(packages, "package", "packages"),
                design::count(rules, "rule", "rules"),
            ]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(", ");
            (
                format!(
                    "Campaign.md is missing. Splat still has the copy it last read: {}.",
                    if counts.is_empty() {
                        "no package or rule selections"
                    } else {
                        &counts
                    }
                ),
                "Write Campaign.md again",
            )
        } else {
            (NO_CAMPAIGN_COPY.into(), "Write a new Campaign.md")
        }
    }

    pub fn write_campaign_again(
        &self,
        campaign: &Campaign,
    ) -> Result<crate::content_workspace::ManifestWrite, String> {
        if let Some((path, text)) = self.held_manifests.borrow().get(&campaign.id) {
            crate::content_workspace::ManifestWrite::restore_campaign(campaign, path, text)
        } else {
            let document = CampaignDocument::new(campaign)?;
            crate::content_workspace::ManifestWrite::restore_campaign(
                campaign,
                &document.path,
                &document.text,
            )
        }
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
        self.discarded_details = None;
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
        let column_right = ui.clip_rect().right();
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
                // The menu stays inside the navigation column, clear of its splitter. A
                // narrow column wraps the names.
                let room = column_right
                    - design::S
                    - ui.style().spacing.menu_margin.rightf()
                    - ui.max_rect().left();
                if room < 220.0 {
                    ui.set_max_width(room);
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                }
                ui.set_min_width(room.min(220.0));
                if let Some(reason) = reason {
                    design::blocked_reason(ui, Some(reason));
                    ui.add_space(design::L);
                }
                ui.add_enabled_ui(reason.is_none(), |ui| {
                    if self.data.campaigns.is_empty() {
                        // Round 25 FE4: the one command keeps its words on one line, with `S` on
                        // each side. In the menu's Strong type it needs 119 px and the column
                        // gives 103, so it uses `Small`, the type of the switcher's own words. A
                        // column the DM makes narrower still wraps it.
                        let clicked = ui
                            .scope(|ui| {
                                ui.spacing_mut().button_padding.x = design::S;
                                let words = Type::Small.rich(ui, "Add campaign");
                                ui.selectable_label(false, words).clicked()
                            })
                            .inner;
                        if clicked {
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
        ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
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
        let link_blocked = blocked.map(|reason| {
            if reason == "Close the run to change campaigns." {
                "Close the run to change the folder, the session folder or the link."
            } else {
                reason
            }
        });
        design::blocked_reason(ui, link_blocked);
        ui.add(egui::Label::new(campaign.path.display().to_string()).wrap());
        Self::folder_status(ui, &campaign);
        ui.add_space(design::S);
        design::rule(ui);
        ui.add_space(design::S);
        dndbeyond.show(ui, session, self, link_blocked);
        ui.add_space(design::S);
        design::quiet_section(ui, "Technical details", |ui| {
            ui.label(format!("ID: {}", campaign.id));
            design::help(ui, "Settings are stored in the campaign manifest.");
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
        // Round 24 FC5: the shared dialog, anchored at the top like every other dialog.
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
                design::dialog_actions(
                    ui,
                    |ui| {
                        remove = Action::destructive("Remove from Splat").show(ui).clicked();
                    },
                    |ui| {
                        keep |= Action::secondary("Keep the campaign").show(ui).clicked();
                    },
                );
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
            // The result of registering a removed campaign ends with it.
            if !self.data.campaigns.iter().any(|c| c.id == id) {
                self.campaign_copy = None;
                self.added = None;
            }
        }
    }

    /// `Build an encounter` after a result: the result stays, with its Undo, for the DM's return.
    fn build_encounter(&mut self) {
        self.wants_encounters = true;
        if self.campaign_copy.is_none() {
            self.added = None;
        }
    }

    /// `Undo register campaign`: the copy's own files go back and its registration goes. Other
    /// settings, including changes made since, stay. The active campaign goes back only while
    /// it is still the copy. The result that the Undo followed ends.
    fn undo_campaign_copy(&mut self) -> Result<(), String> {
        let (copy, id, active) = self.campaign_copy.as_ref().ok_or("Nothing to undo.")?;
        let mut data = self.data.clone();
        data.campaigns.retain(|campaign| campaign.id != *id);
        if data.active_campaign_id == Some(*id) {
            data.active_campaign_id =
                active.filter(|active| data.campaigns.iter().any(|c| c.id == *active));
        }
        self.store
            .as_ref()
            .ok_or("Settings are unavailable.")?
            .check_save(&data)?;
        copy.undo()?;
        self.persist(data)?;
        self.campaign_copy = None;
        self.added = None;
        Ok(())
    }

    /// The campaign list. Which exist, where, and how each is linked.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        session: Option<crate::auth::Session>,
        dndbeyond: &mut crate::dndbeyond::DndBeyond,
        blocked: Option<&str>,
    ) {
        let c = colors();
        ui.set_max_width(self.list_width(ui.available_width()));
        self.remove_dialog(ui.ctx());
        ui.horizontal(|ui| {
            design::group_heading(ui, "Campaigns", self.data.campaigns.len());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if (!self.data.campaigns.is_empty() || !self.writable)
                    && Action::quiet("Reload from disk")
                        .blocked(blocked)
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
        design::blocked_reason(ui, blocked);
        // Round 26: the list's errors and results sit in the Settings footer
        // (`footer_messages`).
        let unwritable =
            blocked.or((!self.writable).then_some("Reload valid campaign settings first."));
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
            // Round 26 AG1: one action column for every row, an `L` gap after the widest name
            // and link block, so the DM's eye does not cross an empty band to reach a row's
            // actions. Until the rows are measured, the actions keep the old room.
            let (measured_block, measured_actions) = self.list_measure;
            let actions_room = if measured_actions > 0.0 {
                measured_actions
            } else {
                220.0
            };
            let mut widest = (0.0_f32, 0.0_f32);
            for campaign in self.data.campaigns.clone() {
                let active = self.data.active_campaign_id == Some(campaign.id);
                ui.push_id(campaign.id, |ui| {
                    ui.horizontal(|ui| {
                        ui.set_min_height(Density::Glance.row() + design::XS);
                        let row_left = ui.cursor().left();
                        let row_right = ui.max_rect().right();
                        let block = ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            ui.set_max_width(
                                (row_right - row_left - actions_room - design::L).max(120.0),
                            );
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
                            if let Some(error) = self.manifest_errors.get(&campaign.id) {
                                ui.label(Type::Small.rich(ui, error).color(c.result.error));
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
                        widest.0 = widest.0.max(block.response.rect.width());
                        // The action column starts an `L` gap after the widest block, or ends at
                        // the list's edge when the block must give way.
                        let actions_left = (row_left + measured_block + design::L)
                            .min(row_right - actions_room)
                            .max(ui.cursor().left());
                        let line = ui.min_rect();
                        let actions = egui::Rect::from_min_max(
                            egui::pos2(actions_left, line.top()),
                            egui::pos2(actions_left + actions_room, line.bottom()),
                        );
                        ui.scope_builder(
                            egui::UiBuilder::new()
                                .max_rect(actions)
                                .layout(egui::Layout::right_to_left(egui::Align::Center)),
                            |ui| {
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
                                    self.discarded_details = None;
                                    self.name = campaign.name.clone();
                                    self.path = campaign.path.to_string_lossy().into_owned();
                                }
                                if !active
                                    && Action::secondary("Open")
                                        .density(Density::Read)
                                        .name(&format!(
                                            "Make {} the active campaign",
                                            campaign.name
                                        ))
                                        .show(ui)
                                        .clicked()
                                {
                                    open = Some(campaign.id);
                                }
                                widest.1 = widest.1.max(ui.min_rect().width());
                            },
                        );
                    });
                    if self.editing == Some(campaign.id) {
                        self.form(ui, false, true);
                    }
                });
                design::rule(ui);
            }
            if (widest.0 - measured_block).abs() > 0.5 || (widest.1 - measured_actions).abs() > 0.5
            {
                self.list_measure = widest;
                ui.ctx().request_discard("the campaign list was measured");
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

    /// The campaign list's width within `room`: its widest name and link block, an `L` gap and
    /// its action column, and never narrower than the campaign form below it (round 26 AG1).
    pub fn list_width(&self, room: f32) -> f32 {
        let (block, actions) = self.list_measure;
        let content = if block > 0.0 && actions > 0.0 {
            block + design::L + actions
        } else {
            design::LIST_MEASURE
        };
        room.min(design::LIST_MEASURE)
            .min(content.max(design::FORM_WIDTH))
    }

    /// Round 26: the campaign form's and the campaign list's messages, in the footer of the pane
    /// that shows them (Settings, or Campaign → Link and folder), so none of them moves the form
    /// or the rows. A discarded details draft with its live Undo first (while its form is open:
    /// it ends with an edit, Save, Cancel or the next Discard), then a registered copy with
    /// `Undo register campaign`; then the error and its own recovery; then the plain add result.
    /// `list` is true in Settings, where the add results belong.
    /// `blocked` is the page's lock (a run, a recording, an unsaved draft): the footer is outside
    /// the list's lock, so an action here that changes the campaigns takes it too.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    pub fn footer_messages<'a>(
        me: &'a std::cell::RefCell<&mut Campaigns>,
        footer: &mut design::Footer<'a>,
        list: bool,
        blocked: Option<&'a str>,
    ) {
        const DISCARDED: &str = "Campaign details draft discarded.";
        let (discarded, added, copy, error, locked, (discarded_order, added_order)) = {
            let this = me.borrow();
            let locked =
                blocked.or((!this.writable).then_some("Reload valid campaign settings first."));
            (
                this.editing.is_some() && this.discarded_details.is_some(),
                this.added.clone().filter(|_| list),
                this.campaign_copy.is_some(),
                this.error.clone(),
                locked,
                (this.discarded_order, this.added_order),
            )
        };
        if discarded {
            footer.result(discarded_order, DISCARDED, true, move |ui| {
                if design::draft_discarded(ui, "Campaign details") {
                    let mut this = me.borrow_mut();
                    if let Some((name, path)) = this.discarded_details.take() {
                        this.name = name;
                        this.path = path;
                    }
                }
            });
        }
        if let Some(added) = added.clone().filter(|_| copy) {
            footer.result(added_order, added.clone(), true, move |ui| {
                // The registration result and its Undo are one message. Build an encounter is
                // another choice: it starts below, at the message words. Only Undo is quiet.
                // Review round 26: Undo changes the registration and can change the active
                // campaign, so it waits for what blocks a campaign change, with the reason.
                if crate::library::kept_undo_message(
                    ui,
                    Tone::Success,
                    &added,
                    "Undo register campaign",
                    locked,
                    true,
                ) {
                    let mut this = me.borrow_mut();
                    if let Err(error) = this.undo_campaign_copy() {
                        this.error = Some(error);
                    }
                }
                let build = design::message_choice_row(ui, Tone::Success, |ui| {
                    Action::secondary("Build an encounter")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                });
                if build {
                    me.borrow_mut().build_encounter();
                }
            });
        }
        if let Some(error) = error {
            footer.reason(error.clone(), move |ui| {
                Self::draw_error(me, ui, &error, locked)
            });
        }
        if let Some(added) = added.filter(|_| !copy) {
            footer.result(added_order, added.clone(), false, move |ui| {
                if design::message_with_action(ui, Tone::Success, &added, "Build an encounter") {
                    me.borrow_mut().build_encounter();
                }
            });
        }
    }

    /// An error, and its own recovery after it.
    fn draw_error(
        me: &std::cell::RefCell<&mut Campaigns>,
        ui: &mut egui::Ui,
        error: &str,
        locked: Option<&str>,
    ) {
        design::message(ui, Tone::Error, error);
        if error.contains("registered at another folder") {
            let reason = locked.or(me
                .borrow()
                .name
                .trim()
                .is_empty()
                .then_some("Type a name first."));
            if Action::secondary("Register as a new campaign")
                .blocked(reason)
                .show(ui)
                .clicked()
            {
                let mut this = me.borrow_mut();
                if let Err(error) = this.register_campaign_copy() {
                    this.error = Some(error);
                }
            }
            design::blocked_reason(ui, reason);
        }
    }

    /// The form's error where its page has no footer: the first campaign's form on an empty
    /// page, which has no scroll area, so nothing below it can move.
    pub fn form_error_inline(&mut self, ui: &mut egui::Ui) {
        let error = self.error.clone();
        let cell = std::cell::RefCell::new(self);
        if let Some(error) = error {
            Self::draw_error(&cell, ui, &error, None);
        }
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
    /// Draws the details form while it is open, and says whether it drew it (with its primary save).
    pub fn show_form_if_editing(&mut self, ui: &mut egui::Ui) -> bool {
        let open = self.editing.is_some();
        if open {
            ui.add_space(design::S);
            self.form(ui, false, true);
        }
        open
    }

    pub fn form_open(&self) -> bool {
        self.editing.is_some() || self.form_open
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn form(&mut self, ui: &mut egui::Ui, _inline: bool, primary: bool) {
        let editing = self.editing.is_some();
        // Keep the add form's open state when Settings changes between one and two columns.
        let form_scope = egui::Id::new(("campaign_form", self.editing));
        ui.scope_builder(egui::UiBuilder::new().id(form_scope), |ui| {
            ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
            let mut added = false;
            let direct = editing || self.remote.is_some() || self.data.campaigns.is_empty();
            let editing_id = self.editing;
            let focus = self.focus_name;
            let empty = self.data.campaigns.is_empty();
            let mut fields = |ui: &mut egui::Ui| {
                ui.add_space(design::XS);
                let name = design::small_text_field(ui, "Campaign name", &mut self.name, None, None);
                if std::mem::take(&mut self.focus_name) {
                    name.request_focus();
                }
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
                let action = || {
                    if primary {
                        Action::primary(submit)
                    } else {
                        Action::secondary(submit)
                    }
                };
                // The buttons sit on the field's line, and end where the fields above end. Round 26
                // FB6: in the add form, Add campaign follows the last field on its line.
                let create_width = if editing {
                    0.0
                } else {
                    action().natural_width(ui) + design::S
                };
                let room = ui.available_width();
                let field = ui
                    .vertical(|ui| {
                        ui.set_max_width((room - BROWSE_WIDTH - design::S - create_width).max(120.0));
                        design::small_text_field(ui, "Folder", &mut self.path, None, None)
                    })
                    .inner;
                let browse = egui::Rect::from_min_size(
                    egui::pos2(field.rect.right() + design::S, field.rect.top()),
                    egui::vec2(BROWSE_WIDTH, field.rect.height()),
                );
                ui.scope_builder(egui::UiBuilder::new().max_rect(browse).layout(egui::Layout::right_to_left(egui::Align::Center)), |ui| {
                    if Action::secondary("Browse…")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                        && let Some(path) = crate::hermetic::pick_folder()
                    {
                        self.path = path.to_string_lossy().into_owned();
                    }
                });
                let mut commit = false;
                if !editing {
                    let create = egui::Rect::from_min_size(
                        egui::pos2(browse.right() + design::S, field.rect.top()),
                        egui::vec2(create_width - design::S, field.rect.height()),
                    );
                    ui.scope_builder(egui::UiBuilder::new().max_rect(create).layout(egui::Layout::left_to_right(egui::Align::Center)), |ui| {
                        commit = action().blocked(reason).density(Density::Read).show(ui).clicked();
                    });
                    // The reason sits on its own line below, which stays when empty, so typing
                    // moves nothing; quiet Cancel starts that line when there is something to
                    // cancel.
                    ui.horizontal(|ui| {
                        // As tall as Cancel, so Cancel coming moves nothing.
                        ui.set_min_height(Density::Read.control());
                        if !missing || self.remote.is_some() {
                            if Action::quiet("Cancel")
                                .density(Density::Read)
                                .line_start()
                                .show(ui)
                                .clicked()
                            {
                                self.clear_form();
                            }
                            ui.add_space(design::L - ui.spacing().item_spacing.x);
                        }
                        design::blocked_reason(ui, reason);
                    });
                }
                design::help(ui, "Splat keeps this campaign’s notes, encounters and runs here. A folder in your Obsidian vault works.");
                if editing {
                    // A save row: state and Discard at the left, Save details at the right edge.
                    ui.horizontal(|ui| {
                        design::save_row_state(
                            ui,
                            design::Status {
                                icon: if unchanged {
                                    design::icons::OK
                                } else {
                                    design::icons::DRAFT
                                },
                                word: if unchanged { "No changes" } else { "Unsaved" },
                                color: if unchanged {
                                    colors().muted
                                } else {
                                    colors().document.unsaved
                                },
                            },
                            false,
                            None,
                        );
                        if !unchanged {
                            self.discarded_details = None;
                            if Action::quiet("Discard").show(ui).clicked()
                                && let Some(saved) = self.data.campaigns.iter().find(|c| Some(c.id) == self.editing)
                            {
                                self.discarded_details = Some((self.name.clone(), self.path.clone()));
                                self.discarded_order = crate::design::ResultOrder::issue();
                                self.name = saved.name.clone();
                                self.path = saved.path.to_string_lossy().into_owned();
                            }
                        } else if Action::quiet("Cancel").line_start().show(ui).clicked() {
                            self.clear_form();
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            commit = action().blocked(reason).show(ui).clicked();
                            ui.spacing_mut().item_spacing.x = design::S;
                            design::blocked_reason(ui, reason.filter(|_| !unchanged));
                        });
                    });
                }
                if commit {
                    match self.save_form() {
                        Ok(()) => added = !editing,
                        Err(error) => self.error = Some(error),
                    }
                }
            };
            let form = if direct {
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
            if let Some(form) = &form {
                design::locked_control(ui, &form.header_response);
            }
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

/// An inline creation form's blocked reason, on its own `Small` line below the Create action.
/// The line stays when the reason is empty, so the rows below never move as the DM types
/// (round 26 FB6). Settings, Manage packages and New entry share it.
pub(crate) fn creation_reason(ui: &mut egui::Ui, reason: Option<&str>) {
    // Not a wrapping row: that pads its line to a control's height.
    ui.vertical(|ui| {
        ui.set_min_height(Type::Small.line());
        design::blocked_reason(ui, reason);
    });
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

/// The result of registering a copied campaign folder, named for the new campaign.
pub(crate) fn copy_registered(name: &str) -> String {
    format!("{name} is registered as a new campaign. Its D&D Beyond link is off.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copied_campaign_can_register_with_new_identities_and_kept_fields() {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("original");
        let copied = temp.path().join("copy");
        for folder in [&original, &copied] {
            std::fs::create_dir(folder).unwrap();
        }
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        campaigns.name = "Original".into();
        campaigns.path = original.to_string_lossy().into_owned();
        campaigns.save_form().unwrap();
        let campaign = campaigns.active_campaign().unwrap();
        let before = std::fs::read_to_string(original.join("Campaign.md"))
            .unwrap()
            .replace(
                "splat_schema:",
                "tags: [keep]\ncustom: {weather: rain}\nsplat_schema:",
            );
        std::fs::write(original.join("Campaign.md"), &before).unwrap();
        std::fs::write(copied.join("Campaign.md"), &before).unwrap();
        campaigns.name = "New campaign".into();
        campaigns.path = copied.to_string_lossy().into_owned();
        assert!(campaigns.save_form().is_err());
        campaigns.register_campaign_copy().unwrap();
        assert_ne!(campaigns.active_campaign().unwrap().id, campaign.id);
        assert_eq!(campaigns.active_campaign().unwrap().name, "New campaign");
        let document = CampaignDocument::load(&campaigns.active_campaign().unwrap()).unwrap();
        let entry = splat::content::Entry::parse(&document.path, &document.text).unwrap();
        assert_eq!(entry.fields["tags"][0].as_str(), Some("keep"));
        assert_eq!(entry.fields["custom"]["weather"].as_str(), Some("rain"));
        assert_eq!(
            std::fs::read_to_string(original.join("Campaign.md")).unwrap(),
            before
        );
        campaigns.campaign_copy.as_ref().unwrap().0.undo().unwrap();
        assert_eq!(
            std::fs::read_to_string(copied.join("Campaign.md")).unwrap(),
            before
        );
    }

    /// SD2: the copy's registration result is named, and its Undo follows it. Build an encounter
    /// leaves both; Undo ends both; a later result replaces both, so Undo never follows another
    /// result. A plain add's result still ends with Build an encounter.
    #[test]
    fn iter20_e_sd2_register_campaign_undo_follows_its_named_result() {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("original");
        let copied = temp.path().join("copy");
        let third = temp.path().join("third");
        for folder in [&original, &copied, &third] {
            std::fs::create_dir(folder).unwrap();
        }
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        campaigns.name = "Original".into();
        campaigns.path = original.to_string_lossy().into_owned();
        campaigns.save_form().unwrap();
        let first = campaigns.active_campaign().unwrap().id;
        let manifest = std::fs::read_to_string(original.join("Campaign.md")).unwrap();
        std::fs::write(copied.join("Campaign.md"), &manifest).unwrap();
        let register = |campaigns: &mut Campaigns| {
            campaigns.name = "Copy of Original".into();
            campaigns.path = copied.to_string_lossy().into_owned();
            assert!(campaigns.save_form().is_err());
            campaigns.register_campaign_copy().unwrap();
        };

        register(&mut campaigns);
        let result =
            "Copy of Original is registered as a new campaign. Its D&D Beyond link is off.";
        assert_eq!(campaigns.added.as_deref(), Some(result), "SD2-named");
        assert!(campaigns.campaign_copy.is_some());

        campaigns.build_encounter();
        assert!(campaigns.wants_encounters);
        assert_eq!(
            campaigns.added.as_deref(),
            Some(result),
            "SD2-build-keeps-result"
        );
        assert!(campaigns.campaign_copy.is_some(), "SD2-build-keeps-undo");

        // Another settings change while the result lives: Library registers a package.
        let package = crate::persistence::Module {
            id: uuid::Uuid::new_v4(),
            name: "Later package".into(),
            path: temp.path().join("later-package"),
        };
        std::fs::create_dir(&package.path).unwrap();
        campaigns.register_module(package.clone()).unwrap();

        campaigns.undo_campaign_copy().unwrap();
        assert!(
            campaigns.added.is_none() && campaigns.campaign_copy.is_none(),
            "SD2-undo-ends"
        );
        assert!(
            campaigns
                .modules()
                .iter()
                .any(|module| module.id == package.id),
            "SD2-undo-keeps-later-changes: {:?}",
            campaigns.modules()
        );
        let reloaded = Campaigns::at(temp.path().join("settings.json"));
        assert!(
            reloaded
                .modules()
                .iter()
                .any(|module| module.id == package.id),
            "SD2-undo-keeps-later-changes-on-disk"
        );
        assert_eq!(campaigns.all().len(), 1);
        assert_eq!(campaigns.active_campaign().unwrap().id, first);
        assert_eq!(
            std::fs::read_to_string(copied.join("Campaign.md")).unwrap(),
            manifest,
            "Undo puts the copy's own file back"
        );

        // Another result replaces the registration result and ends its Undo.
        register(&mut campaigns);
        campaigns.name = "Third".into();
        campaigns.path = third.to_string_lossy().into_owned();
        campaigns.save_form().unwrap();
        assert_eq!(campaigns.added.as_deref(), Some("Added Third."));
        assert!(
            campaigns.campaign_copy.is_none(),
            "SD2-new-result-ends-undo"
        );

        // A plain add has no Undo: Build an encounter ends its result, as before.
        campaigns.build_encounter();
        assert!(campaigns.added.is_none());
    }

    /// SD2: Undo takes back only the copy's registration. A campaign the DM opened since stays
    /// active; the one active before the copy comes back only while the copy is still active.
    #[test]
    fn iter20_e_sd2_undo_keeps_a_campaign_opened_since() {
        let temp = tempfile::tempdir().unwrap();
        let (original, third, copied) = (
            temp.path().join("original"),
            temp.path().join("third"),
            temp.path().join("copy"),
        );
        for folder in [&original, &third, &copied] {
            std::fs::create_dir(folder).unwrap();
        }
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        for (name, folder) in [("Original", &original), ("Third", &third)] {
            campaigns.name = name.into();
            campaigns.path = folder.to_string_lossy().into_owned();
            campaigns.save_form().unwrap();
        }
        let first = campaigns.all()[0].id;
        let before_copy = campaigns.active_campaign().unwrap().id;
        assert_ne!(first, before_copy, "Third is active before the copy");
        std::fs::copy(original.join("Campaign.md"), copied.join("Campaign.md")).unwrap();
        campaigns.name = "Copy".into();
        campaigns.path = copied.to_string_lossy().into_owned();
        assert!(campaigns.save_form().is_err());
        campaigns.register_campaign_copy().unwrap();
        campaigns.select(Some(first));
        campaigns.undo_campaign_copy().unwrap();
        assert_eq!(
            campaigns.active_campaign().map(|c| c.id),
            Some(first),
            "SD2-undo-keeps-opened-campaign"
        );
        assert_eq!(campaigns.all().len(), 2);
    }

    #[test]
    fn broken_campaign_manifest_does_not_hide_other_campaigns() {
        let temp = tempfile::tempdir().unwrap();
        let settings = temp.path().join("settings.json");
        let mut campaigns = Campaigns::at(settings.clone());
        for name in ["Broken", "Healthy"] {
            let folder = temp.path().join(name);
            std::fs::create_dir(&folder).unwrap();
            campaigns.name = name.into();
            campaigns.path = folder.to_string_lossy().into_owned();
            campaigns.save_form().unwrap();
        }
        let healthy = campaigns.active_campaign().unwrap();
        let path = temp.path().join("Broken/Campaign.md");
        let valid = std::fs::read_to_string(&path).unwrap();
        std::fs::write(&path, "---\nsplat_id: [\n---\n").unwrap();
        campaigns.reload();
        assert_eq!(campaigns.all().len(), 2);
        assert_eq!(campaigns.active_campaign(), Some(healthy.clone()));
        assert!(format!("{:?}", campaigns.error).contains("Campaign.md"));
        std::fs::write(&path, &valid).unwrap();
        std::fs::write(path.with_file_name("Campaign copy.md"), &valid).unwrap();
        let reopened = Campaigns::open_at(settings);
        assert_eq!(reopened.active_campaign(), Some(healthy));
        assert!(format!("{:?}", reopened.error).contains("Campaign copy.md"));
    }

    #[test]
    fn deleted_campaign_manifest_needs_explicit_repair_and_keeps_held_choices() {
        let temp = tempfile::tempdir().unwrap();
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        campaigns.name = "Home".into();
        campaigns.path = temp.path().to_string_lossy().into_owned();
        campaigns.save_form().unwrap();
        let campaign = campaigns.active_campaign().unwrap();
        let path = campaign.path.join("Campaign.md");
        let text = std::fs::read_to_string(&path).unwrap().replace(
            "splat_schema: 2",
            "splat_schema: 2\nsplat_session_folder: My sessions\ntags: [keep]",
        );
        std::fs::write(&path, &text).unwrap();
        campaigns.reload();
        let text = format!("{text}\nLatest words read by the Campaign page.\n");
        std::fs::write(&path, &text).unwrap();
        campaigns.remember_campaign_document(&CampaignDocument::load(&campaign).unwrap());
        std::fs::remove_file(&path).unwrap();
        campaigns.reload();
        assert!(!path.exists(), "Reload must never recreate the manifest");
        assert!(CampaignDocument::load(&campaign).is_err());
        assert!(format!("{:?}", campaigns.error).contains("Campaign.md is missing"));
        let (preview, action) = campaigns.campaign_write_preview(&campaign);
        assert!(preview.contains("Splat still has the copy it last read: 1 package."));
        assert_eq!(action, "Write Campaign.md again");
        let fresh = Campaigns::at(temp.path().join("new-settings.json"));
        assert_eq!(
            fresh.campaign_write_preview(&campaign),
            (NO_CAMPAIGN_COPY.into(), "Write a new Campaign.md")
        );
        let write = campaigns.write_campaign_again(&campaign).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), text);
        write.undo().unwrap();
        assert!(!path.exists());
        let write = campaigns.write_campaign_again(&campaign).unwrap();
        let outside = format!("{text}\nOutside notes.\n");
        std::fs::write(&path, &outside).unwrap();
        assert!(write.undo().is_err());
        assert_eq!(std::fs::read_to_string(path).unwrap(), outside);
    }

    #[test]
    fn copied_package_folder_names_the_duplicate_identity_without_registering_it() {
        let dir = tempfile::tempdir().unwrap();
        let mut campaigns = Campaigns::at(dir.path().join("settings.json"));
        let original = dir.path().join("original");
        let copied = dir.path().join("copy");
        std::fs::create_dir(&original).unwrap();
        std::fs::create_dir(&copied).unwrap();
        std::fs::write(
            original.join("Package.md"),
            crate::library::note_text("Ashen Marches", "package", Some("ashen")),
        )
        .unwrap();
        std::fs::copy(original.join("Package.md"), copied.join("Package.md")).unwrap();
        let module = crate::library::local_module(&original).unwrap();
        campaigns.register_module(module.clone()).unwrap();
        let before = std::fs::read(copied.join("Package.md")).unwrap();
        let error = campaigns
            .register_module(crate::library::local_module(&copied).unwrap())
            .unwrap_err();
        assert!(error.contains("same package identity as Ashen Marches"));
        assert!(error.contains(&module.id.to_string()));
        assert!(!error.contains("already registered"));
        assert_eq!(campaigns.modules().len(), 1);
        assert_eq!(std::fs::read(copied.join("Package.md")).unwrap(), before);
        let independent = crate::content_workspace::patch_fields(
            std::str::from_utf8(&before).unwrap(),
            &[("splat_id", Some(Uuid::new_v4().to_string().into()))],
        )
        .unwrap();
        std::fs::write(copied.join("Package.md"), &independent).unwrap();
        let error = campaigns
            .register_module(crate::library::local_module(&copied).unwrap())
            .unwrap_err();
        assert!(error.contains("namespace ashen is already registered"));
        assert_eq!(campaigns.modules().len(), 1);
        assert_eq!(
            std::fs::read_to_string(copied.join("Package.md")).unwrap(),
            independent
        );
    }

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
            "splat_schema: 2",
            "splat_schema: 2\nsplat_run_folder: Sessions/Combat",
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
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
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
        for campaign in &data.campaigns {
            CampaignDocument::new(campaign).unwrap().save().unwrap();
        }
        let mut campaigns = Campaigns {
            store: Some(store),
            data,
            error: None,
            manifest_errors: Default::default(),
            held_manifests: Default::default(),
            campaign_copy: None,
            writable: true,
            editing: None,
            focus_name: false,
            form_open: false,
            add_name: String::new(),
            add_path: String::new(),
            name: String::new(),
            path: String::new(),
            discarded_details: None,
            discarded_order: crate::design::ResultOrder::issue(),
            remote: None,
            confirm_remove: None,
            added: None,
            added_order: crate::design::ResultOrder::issue(),
            wants_encounters: false,
            list_measure: (0.0, 0.0),
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
        let reopened = Campaigns::open_at(temp.path().join("settings.json"));
        assert_eq!(reopened.unregistered_module(package.id), Some(&package));
        campaigns.restore_module(package.clone()).unwrap();
        assert!(campaigns.unregistered_module(package.id).is_none());
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
