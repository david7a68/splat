use crate::{
    content_workspace::CampaignDocument,
    design::{self, memo::Revised},
    persistence::{Campaign, Store, UserData},
};
use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

/// What a press in the campaign messages of a footer asks for (`Campaigns::footer_messages`,
/// done by `Campaigns::footer_act`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CampaignAct {
    UndoDiscard,
    UndoRegister,
    BuildEncounter,
    RegisterAsNew,
}

/// The kept results of the campaign form and list (migration-32 C0): the store issues each
/// result's order when its act is done, so the footer puts them newest first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FormResult {
    /// The details draft Discard dropped, of this campaign: its Undo puts it back only into that
    /// campaign's form. It ends with an edit, Save, Cancel, an Edit details press (any
    /// campaign's form, from either page), a change of the open campaign, the next Discard (a
    /// new result), and its Undo.
    Discarded(Uuid),
    /// The campaign just added or the copy just registered. It ends with the next add, the
    /// registered campaign's removal, and the copy's Undo.
    Added,
}

impl crate::design::results::Owner for FormResult {
    type Scope = ();
    fn scope(&self) -> &() {
        &()
    }
}

/// What a kept result's act left: the dropped draft, or the add's words.
enum FormKept {
    Discarded {
        campaign: Uuid,
        name: String,
        path: String,
    },
    Added(String),
}

impl crate::design::results::Kept for FormKept {
    type Owner = FormResult;
    const ENDS: crate::design::results::Ends = crate::design::results::Ends {
        // The form ends its own result at each end event above (`end_result`).
        owner_changed: crate::design::results::OwnerChanged::Ends,
        disk_changed: crate::design::results::DiskChanged::Stays,
        // Undo discard and Undo register campaign leave the store only when they work.
        press: crate::design::results::Press::Undo,
    };
    fn owner(&self) -> FormResult {
        match self {
            Self::Discarded { campaign, .. } => FormResult::Discarded(*campaign),
            Self::Added(_) => FormResult::Added,
        }
    }
}

/// A failed act of the Settings and Link and folder footers (a campaign's, the account's, D&D
/// Beyond's), in its own words: the check's or the system's.
pub struct NotDone(
    crate::design::outcome::Sentence,
    crate::design::outcome::Severity,
);

impl NotDone {
    /// A failed act: the error tone.
    pub fn words(words: &str) -> Self {
        Self(
            footer_words(words),
            crate::design::outcome::Severity::NotSaved,
        )
    }

    /// An act that waits for the DM to fix something first (a copy that needs an entry of
    /// this campaign only): the warning tone.
    pub fn needs_fixing(words: &str) -> Self {
        Self(
            footer_words(words),
            crate::design::outcome::Severity::NeedsFixing,
        )
    }
}

impl crate::design::outcome::Fault for NotDone {
    fn severity(&self) -> crate::design::outcome::Severity {
        self.1
    }
    fn cause(&self) -> crate::design::outcome::Sentence {
        self.0.clone()
    }
}

/// Words of the Settings and Link and folder footers made from data (a result that names a
/// campaign, a check's or the system's error) as a sentence: the one escape of these footers.
#[allow(
    clippy::disallowed_methods,
    reason = "the footer’s results and errors are made from data, or come from the system"
)]
pub fn footer_words(words: &str) -> crate::design::outcome::Sentence {
    crate::design::outcome::Sentence::outside(words).unwrap_or(crate::sentence!("It did not work."))
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    pub static MODULE_READS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub struct Campaigns {
    store: Option<Store>,
    data: UserData,
    /// Positions in registration order. A new catalogue gets a new revision, including on rename.
    module_positions: Revised<HashMap<Uuid, usize>>,
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
    /// The discarded details draft and the add's result, each with the order its act took.
    results: crate::design::Results<FormResult, FormKept>,
    /// The open import form: the remote campaign it adds.
    remote: Option<ImportForm>,
    confirm_remove: Option<Uuid>,
    /// The DM asked to go on to Encounters, from that message.
    pub wants_encounters: bool,
}

fn index_modules(modules: &[crate::persistence::Module]) -> Revised<HashMap<Uuid, usize>> {
    Revised::new(
        modules
            .iter()
            .enumerate()
            .map(|(at, module)| (module.id, at))
            .collect(),
    )
}

/// A missing Campaign.md without a copy. The write is a choice the DM makes with the button.
pub(crate) const NO_CAMPAIGN_COPY: &str = "Campaign.md is missing, and Splat has no copy. You can write a new one with the default packages.";

impl Default for Campaigns {
    fn default() -> Self {
        let mut result = Self {
            store: None,
            data: UserData::default(),
            module_positions: Revised::new(HashMap::new()),
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
            results: Default::default(),
            remote: None,
            confirm_remove: None,
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
    /// `Add…` on a remote campaign: the import form opens for it, with a folder Splat proposes
    /// and the name field taking the keyboard.
    pub fn begin_import(&mut self, id: &str, name: &str) {
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
        self.remote = Some(ImportForm {
            id: id.into(),
            name: name.into(),
        });
        self.focus_name = true;
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
        if let Some(ImportForm { id, name, .. }) = &self.remote {
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
        if adding {
            self.results
                .record(FormKept::Added(format!("Added {}.", campaign.name)));
        } else {
            self.results.owner_changed(&FormResult::Added);
        }
        Ok(())
    }

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
        self.results.record(FormKept::Added(copy_registered(&name)));
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
        #[cfg(any(test, feature = "test-support"))]
        MODULE_READS.with(|reads| reads.set(reads.get() + 1));
        &self.data.modules
    }

    pub fn module_positions(&self) -> &Revised<HashMap<Uuid, usize>> {
        &self.module_positions
    }

    fn replace_data(&mut self, data: UserData) {
        if self.data.modules != data.modules {
            self.module_positions = index_modules(&data.modules);
        }
        self.data = data;
    }

    /// Validated settings always contain the active campaign, when one is selected.
    pub fn active_id(&self) -> Option<Uuid> {
        self.data.active_campaign_id.filter(|_| self.writable)
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

    #[cfg(any(test, feature = "test-support"))]
    pub fn at(path: PathBuf) -> Self {
        let mut store = Store::at(path);
        let data = store.load().unwrap();
        Self {
            store: Some(store),
            module_positions: index_modules(&data.modules),
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
            results: Default::default(),
            remote: None,
            confirm_remove: None,
            wants_encounters: false,
        }
    }

    /// Add `campaign` if it is new, and make it the active campaign, as the switcher does.
    #[cfg(any(test, feature = "test-support"))]
    pub fn open_for_test(&mut self, campaign: &Campaign) {
        let mut data = self.data.clone();
        if !data.campaigns.iter().any(|known| known.id == campaign.id) {
            data.campaigns.push(campaign.clone());
        }
        data.active_campaign_id = Some(campaign.id);
        self.persist(data).unwrap();
    }

    /// The same startup load as `default`, from a temporary settings file.
    #[cfg(any(test, feature = "test-support"))]
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
                    self.replace_data(data);
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
        self.replace_data(data);
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

    /// The discarded details draft ends (see `FormResult::Discarded`), whichever campaign's.
    fn end_discard(&mut self) {
        self.results
            .end_unreachable(|kept| !matches!(kept, FormKept::Discarded { .. }));
    }

    fn clear_form(&mut self) {
        self.end_discard();
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
        let id = self.active_id()?;
        self.data.campaigns.iter().find(|c| c.id == id).cloned()
    }

    /// The campaign switcher's words for a UI that draws it itself (the GPUI rail): the active
    /// campaign's name, else `No campaign` or `Select campaign`.
    pub fn switcher_label(&self) -> &str {
        let current = self.data.active_campaign_id;
        self.data
            .campaigns
            .iter()
            .find(|p| Some(p.id) == current)
            .map(|p| p.name.as_str())
            .unwrap_or(if self.data.campaigns.is_empty() {
                "No campaign"
            } else {
                "Select campaign"
            })
    }

    /// Why the switcher cannot change now: `blocked` (an open run, a draft), or settings that
    /// could not be read.
    pub fn switcher_lock<'a>(&self, blocked: Option<&'a str>) -> Option<&'a str> {
        if self.writable {
            blocked
        } else {
            Some("Reload valid campaign settings in Settings first.")
        }
    }

    /// Make `id` the active campaign (`None`: no campaign), as the switcher does.
    pub fn choose(&mut self, id: Option<Uuid>) {
        self.select(id);
    }

    fn select(&mut self, id: Option<Uuid>) {
        // Another campaign: a discarded details draft was the other campaign's.
        if id != self.data.active_campaign_id {
            self.end_discard();
        }
        let mut data = self.data.clone();
        data.active_campaign_id = id;
        if let Err(error) = self.persist(data) {
            self.error = Some(error);
        }
    }

    /// The Link and folder page's lock, in its own words: a run's lock names what it keeps.
    pub fn link_lock(blocked: Option<&str>) -> Option<&str> {
        blocked.map(|reason| {
            if reason == "Close the run to change campaigns." {
                "Close the run to change the folder, the session folder or the link."
            } else {
                reason
            }
        })
    }

    /// `Build an encounter` after a result: the result stays, with its Undo, for the DM's return.
    fn build_encounter(&mut self) {
        self.wants_encounters = true;
        if self.campaign_copy.is_none() {
            self.results.owner_changed(&FormResult::Added);
        }
    }

    /// The add's or the copy's result words, while the result stands.
    fn added(&self) -> Option<&str> {
        match self.results.get(&FormResult::Added)?.value() {
            FormKept::Added(words) => Some(words),
            FormKept::Discarded { .. } => None,
        }
    }

    /// `Undo register campaign`: the copy's own files go back and its registration goes. Other
    /// settings, including changes made since, stay. The active campaign goes back only while
    /// it is still the copy. The result that the Undo followed ends; a failed Undo keeps it.
    fn undo_campaign_copy(&mut self, ticket: design::results::Ticket) -> Result<(), String> {
        let mut results = std::mem::take(&mut self.results);
        let undone = results.undo_by(ticket, |_| self.undo_copy_files());
        self.results = results;
        match undone {
            Ok(_) => Ok(()),
            Err(design::results::ByTicket::Failed(error)) => Err(error),
            Err(design::results::ByTicket::Gone) => Err("Nothing to undo.".into()),
        }
    }

    /// The ticket of the add's or the copy's result line, for tests of its Undo.
    #[cfg(test)]
    fn added_ticket(&self) -> design::results::Ticket {
        self.results
            .get(&FormResult::Added)
            .expect("an add or copy result")
            .ticket_for_caller_test()
    }

    fn undo_copy_files(&mut self) -> Result<(), String> {
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
        Ok(())
    }

    /// Round 26: the campaign form's and the campaign list's messages, in the footer of the pane
    /// that shows them (Settings, or Campaign → Link and folder), so none of them moves the form
    /// or the rows. A discarded details draft with its live Undo first (while its form is open:
    /// it ends with an edit, Save, Cancel or the next Discard), then a registered copy with
    /// `Undo register campaign`; then the error and its own recovery; then the plain add result.
    /// `list` is true in Settings, where the add results belong.
    /// `blocked` is the page's lock (a run, a recording, an unsaved draft): the footer is outside
    /// the list's lock, so an action here that changes the campaigns takes it too.
    /// The lines are typed data (layout PR 2b); a press comes back from the pane, and the host
    /// gives it to `footer_act`. The results are the store's (`results`), each with the order
    /// its act took; `frame` is the pane's.
    #[allow(
        clippy::disallowed_methods,
        reason = "the page’s lock words are app-owned `&str` from `blocked_switch`, made into a sentence here"
    )]
    pub fn footer_messages<'f, A: From<CampaignAct> + 'static>(
        &self,
        frame: &'f design::outcome::PaneFrame,
        messages: &mut design::outcome::Messages<'f, A>,
        list: bool,
        blocked: Option<&str>,
    ) {
        use design::outcome::{
            Choice, FaultMessage, KeptOutcome, MessageChoice, Notice, Said, Sentence, UndoStep,
            Verb,
        };
        let locked =
            blocked.or((!self.writable).then_some("Reload valid campaign settings first."));
        if let Some(editing) = self.editing
            && let Some(shown) = self.results.get(&FormResult::Discarded(editing))
        {
            messages.kept(shown.outcome(frame, |_| {
                KeptOutcome::Notice(
                    Notice::new(crate::sentence!("Campaign details draft discarded.")),
                    UndoStep::of(Verb::Discard, "Campaign details")
                        .pressed_as(A::from(CampaignAct::UndoDiscard)),
                )
            }));
        }
        let mut added = self
            .results
            .get(&FormResult::Added)
            .filter(|_| list && self.added().is_some());
        let copy = self.campaign_copy.is_some();
        // Build an encounter is another choice after the result's words; only Undo is quiet.
        let build = || {
            Choice::new(
                MessageChoice::Choice("Build an encounter"),
                A::from(CampaignAct::BuildEncounter),
            )
        };
        if let Some(shown) = added.take_if(|_| copy) {
            // Review round 26: Undo changes the registration and can change the active campaign,
            // so it waits for what blocks a campaign change, with the reason. The page's lock
            // comes and goes (a run, a recording, a draft): `locked_when`, so its line keeps its
            // room while it is open and nothing below moves. Its words come from `blocked_switch`
            // in `src/lib.rs` as app-owned `&str`; while open, the room is the draft's.
            let reason = locked
                .and_then(Sentence::outside)
                .unwrap_or(splat::sentence!(
                    "Save or discard your changes to change campaigns."
                ));
            let undo = UndoStep::of(Verb::Register, "campaign")
                .pressed_as(A::from(CampaignAct::UndoRegister))
                .locked_when(locked.is_some(), reason);
            let words = footer_words(self.added().unwrap_or_default());
            messages.kept(shown.outcome(frame, |_| {
                Said::new(words).undo(undo).choice(build()).into()
            }));
        }
        if let Some(error) = &self.error {
            let mut fault = FaultMessage::new(&NotDone::words(error));
            if error.contains("registered at another folder") {
                let reason = locked
                    .or(self.name.trim().is_empty().then_some("Type a name first."))
                    .and_then(Sentence::outside);
                let choice = Choice::new(
                    MessageChoice::Choice("Register as a new campaign"),
                    A::from(CampaignAct::RegisterAsNew),
                );
                fault = fault.choice(match reason {
                    Some(reason) => choice.locked_when(true, reason),
                    None => choice,
                });
            }
            messages.outcome(fault);
        }
        if let Some(shown) = added {
            let words = footer_words(self.added().unwrap_or_default());
            messages.kept(shown.outcome(frame, |_| Said::new(words).choice(build()).into()));
        }
    }

    /// What a press in the campaign messages of a footer asks for (`footer_messages`), done
    /// after the pane is drawn.
    /// An Undo names its result by the ticket of the line that was pressed (`Results::undo_by`),
    /// so it never undoes a newer result of the same kind.
    pub fn footer_act(&mut self, press: design::outcome::Press<CampaignAct>) {
        match (press.id, press.ticket) {
            (CampaignAct::UndoDiscard, Some(ticket)) => {
                // Only into its own campaign's open form.
                let editing = self.editing;
                let undone = self.results.undo_by(ticket, |kept| match kept {
                    FormKept::Discarded {
                        campaign,
                        name,
                        path,
                    } if Some(*campaign) == editing => Ok((name.clone(), path.clone())),
                    FormKept::Discarded { .. } | FormKept::Added(_) => Err(()),
                });
                if let Ok((_, (name, path))) = undone {
                    self.name = name;
                    self.path = path;
                }
            }
            (CampaignAct::UndoRegister, Some(ticket)) => {
                if let Err(error) = self.undo_campaign_copy(ticket) {
                    self.error = Some(error);
                }
            }
            (CampaignAct::UndoDiscard | CampaignAct::UndoRegister, None) => {}
            (CampaignAct::BuildEncounter, _) => self.build_encounter(),
            (CampaignAct::RegisterAsNew, _) => {
                if let Err(error) = self.register_campaign_copy() {
                    self.error = Some(error);
                }
            }
        }
    }

    pub fn importing(&self, remote: &str) -> bool {
        self.remote.as_ref().is_some_and(|form| form.id == remote)
    }

    pub fn focus_add(&mut self) {
        self.focus_name = true;
    }

    pub fn form_open(&self) -> bool {
        self.editing.is_some() || self.form_open
    }

    /// Whether the details form holds the saved name and folder of the campaign it edits.
    fn details_unchanged(&self) -> bool {
        self.editing
            .and_then(|id| {
                self.data
                    .campaigns
                    .iter()
                    .find(|campaign| campaign.id == id)
            })
            .is_some_and(|campaign| {
                campaign.name == self.name.trim()
                    && campaign.path == std::path::Path::new(self.path.trim())
            })
    }

    /// What the form's Save waits for, if anything.
    fn form_reason(&self, unchanged: bool) -> Option<FormReason> {
        match (self.name.trim().is_empty(), self.path.trim().is_empty()) {
            (true, true) => Some(FormReason::NameAndFolder),
            (true, false) => Some(FormReason::Name),
            (false, true) => Some(FormReason::Folder),
            (false, false) if unchanged => Some(FormReason::Unchanged),
            (false, false) => None,
        }
    }
}

crate::create_reasons! {
    /// Why the campaign form cannot add or save yet: every reason it can show, so its kept line
    /// keeps room for the tallest.
    enum FormReason {
        NameAndFolder => "Enter a campaign name and a folder first.",
        Name => "Enter a campaign name first.",
        Folder => "Choose a folder first.",
        Unchanged => "No changes to save.",
    }
}
pub const FOLDER_HELP: &str = "Splat keeps this campaign’s notes, encounters and runs here. A folder in your Obsidian vault works.";

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

/// The folder `Add campaign` will add, by its last name (`…/Salt Road`), after the words
/// `Will create `: the end of the path is what a narrow field cuts. A folder name alone (a
/// relative path) is the name.
fn folder_preview(path: &str) -> Option<String> {
    let path = std::path::Path::new(path.trim());
    let name = path.file_name()?.to_string_lossy();
    Some(
        match path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            Some(_) => format!("…/{name}"),
            None => name.into_owned(),
        },
    )
}

/// The open import form (`Campaigns::remote`).
struct ImportForm {
    id: String,
    name: String,
}

/// The result of registering a copied campaign folder, named for the new campaign.
pub fn copy_registered(name: &str) -> String {
    format!("{name} is registered as a new campaign. Its D&D Beyond link is off.")
}

#[path = "campaigns/view.rs"]
mod view;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_context::StateContext;

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
        assert_eq!(campaigns.added(), Some(result), "SD2-named");
        assert!(campaigns.campaign_copy.is_some());

        campaigns.build_encounter();
        assert!(campaigns.wants_encounters);
        assert_eq!(campaigns.added(), Some(result), "SD2-build-keeps-result");
        assert!(campaigns.campaign_copy.is_some(), "SD2-build-keeps-undo");

        // Another settings change while the result lives: Library registers a package.
        let package = crate::persistence::Module {
            id: uuid::Uuid::new_v4(),
            name: "Later package".into(),
            path: temp.path().join("later-package"),
        };
        std::fs::create_dir(&package.path).unwrap();
        campaigns.register_module(package.clone()).unwrap();

        campaigns
            .undo_campaign_copy(campaigns.added_ticket())
            .unwrap();
        assert!(
            campaigns.added().is_none() && campaigns.campaign_copy.is_none(),
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
        assert_eq!(campaigns.added(), Some("Added Third."));
        assert!(
            campaigns.campaign_copy.is_none(),
            "SD2-new-result-ends-undo"
        );

        // A plain add has no Undo: Build an encounter ends its result, as before.
        campaigns.build_encounter();
        assert!(campaigns.added().is_none());
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
        campaigns
            .undo_campaign_copy(campaigns.added_ticket())
            .unwrap();
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
        campaigns.begin_import("12", "Remote name");
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
        reopened.begin_import("34", "Other campaign");
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
        for campaign in &data.campaigns {
            CampaignDocument::new(campaign).unwrap().save().unwrap();
        }
        let mut campaigns = Campaigns {
            store: Some(store),
            module_positions: index_modules(&data.modules),
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
            results: Default::default(),
            remote: None,
            confirm_remove: None,
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
            Store::at(path.clone()).load().unwrap().campaigns[1]
                .dndbeyond_campaign_id
                .as_deref(),
            Some("12")
        );
        // Review of PR 642: each result belongs to its campaign. The Link and folder pane's
        // footer as the page builds it (the GPUI `campaign_lines`): the campaign's messages, then
        // D&D Beyond's for the campaign open now; a press is done after it (`footer_act`).
        let mut dndbeyond =
            crate::dndbeyond::DndBeyond::listed_for_test(&[("12", "Watchtower"), ("34", "Harbor")]);
        let ctx = StateContext::new();
        let lines = |campaigns: &Campaigns, dndbeyond: &crate::dndbeyond::DndBeyond| {
            let frame = design::outcome::PaneFrame::new();
            let mut messages = design::outcome::Messages::<CampaignAct>::new();
            campaigns.footer_messages(&frame, &mut messages, false, None);
            let open = campaigns.active_campaign().map(|campaign| campaign.id);
            dndbeyond.footer_messages(&frame, &mut messages, open);
            messages.into_plain()
        };
        const SAVED: &str = "Campaign link saved.";
        const UNDO: &str = "Undo discard of Campaign details";
        let shows =
            |campaigns: &Campaigns, dndbeyond: &crate::dndbeyond::DndBeyond, words: &str| {
                lines(campaigns, dndbeyond).iter().any(|line| {
                    line.words.contains(words)
                        || line.undo.as_ref().is_some_and(|undo| undo.label == words)
                })
            };
        dndbeyond.poll(&ctx, 0, true, &mut campaigns);
        // A (the second campaign) unlinks: its result. B (the first, unlinked) opens: A's
        // result ends with the change, and it does not come back with A.
        dndbeyond.choose_link(&mut campaigns, None);
        assert!(
            shows(&campaigns, &dndbeyond, SAVED),
            "unlinking A says its result"
        );
        // The switcher opens B after `poll` ran (review of PR 665): the footer built before the
        // next `poll` reads the campaign open now, so A's result is not in it.
        campaigns.select(Some(first));
        assert!(
            !shows(&campaigns, &dndbeyond, SAVED),
            "A's link result showed in the frame B opened"
        );
        dndbeyond.poll(&ctx, 0, true, &mut campaigns);
        assert!(
            !shows(&campaigns, &dndbeyond, SAVED),
            "A's link result followed into B"
        );
        campaigns.select(Some(second));
        dndbeyond.poll(&ctx, 0, true, &mut campaigns);
        assert!(
            !shows(&campaigns, &dndbeyond, SAVED),
            "A's ended link result came back"
        );
        // What Undo of A must not touch, and A's saved folder (the draft adds `-a` to it).
        let stored = |campaigns: &Campaigns, id: Uuid| {
            campaigns
                .all()
                .iter()
                .find(|campaign| campaign.id == id)
                .map(|campaign| (campaign.name.clone(), campaign.path.clone()))
                .unwrap()
        };
        let b_stored = stored(&campaigns, first);
        let a_folder = stored(&campaigns, second).1.to_string_lossy().into_owned();
        // A's details: an edit of the name and the folder (review of PR 665: A's folder comes
        // back too), then Discard, keep A's draft with its Undo in A's own form.
        let a = campaigns.active_campaign().unwrap();
        campaigns.start_edit(&a, true);
        campaigns.set_form_name("Campaign A");
        campaigns.details_edited();
        campaigns.set_form_path(&format!("{a_folder}-a"));
        campaigns.details_edited();
        campaigns.discard_details();
        // Unrelated B: A's link changes (a result of its own). Undo of A then puts A's draft
        // back into A's form, and B's change and its result stay.
        dndbeyond.choose_link(&mut campaigns, Some("34".into()));
        assert!(
            shows(&campaigns, &dndbeyond, SAVED),
            "B's link change says its result"
        );
        let undo = lines(&campaigns, &dndbeyond)
            .iter()
            .find(|line| line.undo.as_ref().is_some_and(|undo| undo.label == UNDO))
            .and_then(|line| line.press_undo())
            .expect("the discard's Undo");
        campaigns.footer_act(undo);
        // Review of PR 688 (P1) and PR 693 (P2): after Undo, the open form is A's, its fields
        // hold A's draft, and B's stored details did not change.
        let (owner, (name, folder)) = (campaigns.editing, campaigns.form_words());
        assert!(
            owner == Some(second)
                && name == "Campaign A"
                && folder == format!("{a_folder}-a")
                && stored(&campaigns, first) == b_stored,
            "Undo of A left the form or B wrong: form of {owner:?} (A is {second}), name {name:?}, \
             folder {folder:?}, B stored {:?}",
            stored(&campaigns, first)
        );
        assert!(
            shows(&campaigns, &dndbeyond, SAVED)
                && campaigns
                    .active_campaign()
                    .is_some_and(|campaign| campaign.dndbeyond_campaign_id.as_deref() == Some("34")),
            "Undo of A changed B's link or ended its result"
        );
        campaigns.discard_details();
        assert!(shows(&campaigns, &dndbeyond, UNDO));
        // A link attempt fails (the settings cannot be written): the failure is the newest
        // answer, and the older success ends.
        use std::os::unix::fs::PermissionsExt;
        let settings = path.parent().unwrap().to_path_buf();
        let mode = |path: &std::path::Path, mode| {
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).unwrap()
        };
        mode(&path, 0o444);
        mode(&settings, 0o555);
        dndbeyond.choose_link(&mut campaigns, Some("12".into()));
        mode(&settings, 0o755);
        mode(&path, 0o644);
        assert!(
            dndbeyond.error_for_test().is_some() && !shows(&campaigns, &dndbeyond, SAVED),
            "a failed link left its older success beside the error ({:?})",
            dndbeyond.error_for_test()
        );
        // B opens and its Edit details opens B's form. A's draft is not B's: no Undo of it
        // shows there, and B's form keeps B's details.
        campaigns.select(Some(first));
        dndbeyond.poll(&ctx, 0, true, &mut campaigns);
        campaigns.start_edit(&campaigns.active_campaign().unwrap(), true);
        assert!(
            !shows(&campaigns, &dndbeyond, UNDO) && campaigns.name == "Campaign",
            "A's discarded draft offers its Undo in B's form (name {:?})",
            campaigns.name
        );
        // Back in A: the draft ended with the change; A's form shows A's saved details.
        campaigns.select(Some(second));
        dndbeyond.poll(&ctx, 0, true, &mut campaigns);
        campaigns.start_edit(&campaigns.active_campaign().unwrap(), true);
        assert!(!shows(&campaigns, &dndbeyond, UNDO));
        assert_eq!(campaigns.name, "Campaign");
    }
    #[test]
    fn unregistering_remembers_legacy_names_and_finding_a_folder_restores_one_registration() {
        let temp = tempfile::tempdir().unwrap();
        let folder = temp.path().join("campaign");
        std::fs::create_dir(&folder).unwrap();
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        campaigns.begin_import("12", "Campaign");
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

    /// G137, G138: the Folder field cuts a long folder Splat proposes. The kept line names it
    /// whole by its last name, `Will create …/<name>` (`form_preview`; the GPUI form draws it).
    #[test]
    fn import_form_names_the_proposed_folder_whole() {
        let temp = tempfile::tempdir().unwrap();
        let settings = temp
            .path()
            .join("a-long-folder-of-settings/with/several/levels/settings");
        let mut campaigns = Campaigns::at(settings.join("settings.json"));
        campaigns.begin_import("7", "Salt Road");
        assert!(campaigns.path.ends_with("Salt Road"));
        assert_eq!(campaigns.form_preview().as_deref(), Some("…/Salt Road"));
        // The form opens for that campaign, with the name field taking the keyboard.
        assert!(campaigns.importing("7") && campaigns.take_focus_name());
    }
}
