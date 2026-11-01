//! Campaign encounter templates. Runtime state is deliberately separate.
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue, EntryRef, patch_fields},
    design::{self, Action, Density, Status, Tone, Type, colors, icons},
    library::{note_path, write_note},
    persistence::{Campaign, Module},
};
use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_yaml_ng::{Mapping, Value};
use splat::content::Entry;
use std::{collections::BTreeSet, fs, sync::mpsc};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Slot {
    key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    creature: Option<EntryRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    party_slot: Option<String>,
    side: String,
    controller: Value,
    #[serde(default, skip_serializing_if = "Mapping::is_empty")]
    initial: Mapping,
    #[serde(flatten)]
    extra: Mapping,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Setup {
    roster: Vec<Slot>,
    sides: Mapping,
    assumptions: Value,
    completion: Value,
    limits: Value,
    #[serde(flatten)]
    extra: Mapping,
}
fn yaml(text: &str) -> Value {
    serde_yaml_ng::from_str(text).expect("static YAML")
}
impl Default for Setup {
    fn default() -> Self {
        serde_yaml_ng::from_value(yaml("roster: []\nsides:\n  party: {hostile_to: [enemies]}\n  enemies: {hostile_to: [party]}\nassumptions: []\ncompletion: {mode: manual}\nlimits: {rounds: 20, resolution_events: 10000}\n")).expect("default setup")
    }
}
struct Draft {
    path: String,
    baseline: Option<String>,
    source: String,
    name: String,
    setup: Setup,
    saved_name: String,
    saved_setup: Setup,
}
impl Draft {
    fn new() -> Self {
        let id = Uuid::new_v4();
        let name = "New encounter".to_owned();
        let source = format!(
            "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: encounter\nsplat_name: New encounter\nsplat_encounter: {{}}\n---\n\n"
        );
        Self {
            path: format!("Encounters/{id}.md"),
            baseline: None,
            source,
            name: name.clone(),
            saved_name: name,
            setup: Setup::default(),
            saved_setup: Setup::default(),
        }
    }
    fn open(root: &std::path::Path, path: &str) -> Result<Self, String> {
        let text = fs::read_to_string(note_path(root, path)?).map_err(|e| e.to_string())?;
        let entry = Entry::parse(path, &text)?;
        if entry.kind != "encounter" {
            return Err("Select an encounter note.".into());
        }
        let setup: Setup = serde_yaml_ng::from_value(entry.fields["splat_encounter"].clone())
            .map_err(|e| {
                format!("This encounter needs repair in Campaign → Campaign entries: {e}")
            })?;
        Ok(Self {
            path: path.into(),
            baseline: Some(text.clone()),
            source: text,
            saved_name: entry.name.clone(),
            name: entry.name,
            saved_setup: setup.clone(),
            setup,
        })
    }
    fn dirty(&self) -> bool {
        self.baseline.is_none() || self.name != self.saved_name || self.setup != self.saved_setup
    }
    fn text(&self) -> Result<String, String> {
        patch_fields(
            &self.source,
            &[
                ("splat_name", Some(self.name.clone().into())),
                (
                    "splat_encounter",
                    Some(serde_yaml_ng::to_value(&self.setup).map_err(|e| e.to_string())?),
                ),
            ],
        )
    }
    fn add(&mut self, id: Uuid, party: bool) {
        self.setup.roster.push(Slot {
            key: format!("actor_{}", Uuid::new_v4().simple()),
            creature: Some(EntryRef::id(id)),
            party_slot: None,
            side: if party { "party" } else { "enemies" }.into(),
            controller: yaml(if party {
                "{kind: manual, role: player}"
            } else {
                "{kind: manual, role: gm}"
            }),
            initial: Mapping::new(),
            extra: Mapping::new(),
        });
        for (key, value) in Setup::default().sides {
            self.setup.sides.entry(key).or_insert(value);
        }
    }
    fn save(&mut self, campaign: &Campaign, modules: &[Module]) -> Result<(), String> {
        let snapshot = Snapshot::load(campaign, modules)?;
        self.validate(&snapshot)?;
        let text = self.text()?;
        let entry = Entry::parse(&self.path, &text)?;
        if snapshot.catalogue.entries.values().any(|e| {
            e.entry.id == entry.id && (e.owner != campaign.id || e.entry.path != self.path)
        }) {
            return Err("Another note uses this encounter identity.".into());
        }
        // Create the manifest first so a new campaign keeps its module selections.
        let mut document = CampaignDocument::load(campaign)?;
        if document.baseline.is_none() {
            document.save()?;
        }
        let path = note_path(&campaign.path, &self.path)?;
        fs::create_dir_all(path.parent().ok_or("Missing encounter folder")?)
            .map_err(|e| e.to_string())?;
        write_note(&campaign.path, &self.path, self.baseline.as_deref(), &text)?;
        self.source = text.clone();
        self.baseline = Some(text);
        self.saved_name = self.name.clone();
        self.saved_setup = self.setup.clone();
        Ok(())
    }
    fn validate(&self, snapshot: &Snapshot) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Enter an encounter name.".into());
        }
        let mut keys = BTreeSet::new();
        for (side, data) in &self.setup.sides {
            if side.as_str().is_none_or(|s| s.trim().is_empty()) {
                return Err("Sides need names.".into());
            }
            let hostile = data
                .get("hostile_to")
                .and_then(Value::as_sequence)
                .ok_or("Each side needs a hostile_to list.")?;
            if hostile.iter().any(|s| !self.setup.sides.contains_key(s)) {
                return Err("A hostile side is missing.".into());
            }
        }
        for slot in &self.setup.roster {
            if slot.key.trim().is_empty() || !keys.insert(&slot.key) {
                return Err("Roster keys must be unique and non-empty.".into());
            }
            if !self
                .setup
                .sides
                .contains_key(Value::from(slot.side.clone()))
            {
                return Err(format!("{}: side is missing.", slot.key));
            }
            match (&slot.creature, &slot.party_slot) {
                (Some(reference), None) => {
                    let id = snapshot.catalogue.resolve_ref(reference)?;
                    if !snapshot.creatures.iter().any(|c| c.id == id) { return Err(format!("{}: creature is unavailable in campaign content.", slot.key)); }
                }
                (None, Some(_)) => return Err("This encounter uses party bindings. Edit those slots in Campaign → Campaign entries; this editor currently adds local character copies.".into()),
                _ => return Err("Each roster slot needs exactly one creature or party slot.".into()),
            }
            for field in ["hp", "temporary_hp"] {
                if slot
                    .initial
                    .get(field)
                    .is_some_and(|v| v.as_u64().is_none())
                {
                    return Err(format!(
                        "{}: {field} must be a non-negative whole number.",
                        slot.key
                    ));
                }
            }
        }
        Ok(())
    }
}
struct Creature {
    id: Uuid,
    name: String,
    lower: String,
    /// CR, HP and AC, from fields the entry already has.
    detail: String,
    party: bool,
}
const ROSTER_MAX_WIDTH: f32 = 760.0;
const RESULT_ROW: f32 = 40.0;
fn creature_detail(entry: &Entry, party: bool) -> String {
    let stat = |name: &str| {
        entry
            .fields
            .get("splat_stats")?
            .get(name)?
            .get("value")?
            .as_i64()
    };
    let cr = entry
        .fields
        .get("splat_creature")
        .and_then(|creature| creature.get("cr_text"))
        .and_then(Value::as_str)
        .and_then(|text| text.split_whitespace().next());
    let mut parts = Vec::new();
    if party {
        parts.push("Party copy".to_owned());
    }
    if let Some(cr) = cr {
        parts.push(format!("CR {cr}"));
    }
    if let Some(hp) = stat("max_hp") {
        parts.push(format!("HP {hp}"));
    }
    if let Some(ac) = stat("armor_class") {
        parts.push(format!("AC {ac}"));
    }
    parts.join(" · ")
}
fn roster_summary(setup: &Setup) -> String {
    let party = setup.roster.iter().filter(|s| s.side == "party").count();
    let others = setup.roster.len() - party;
    format!("{party} party · {others} others")
}
struct Snapshot {
    catalogue: Catalogue,
    creatures: Vec<Creature>,
    encounters: Vec<(String, String)>,
    encounter_summaries: std::collections::BTreeMap<String, String>,
    issues: Vec<String>,
    options: crate::encounter_settings::Options,
    saved_options: crate::encounter_settings::Options,
    campaign_text: String,
    runs: Vec<crate::encounter_run::SavedRunInfo>,
}
impl Snapshot {
    fn load(campaign: &Campaign, modules: &[Module]) -> Result<Self, String> {
        let document = CampaignDocument::load(campaign)?;
        let catalogue = Catalogue::load(modules, Some(campaign))?;
        let closure = catalogue.closure(&document.selections, campaign.id);
        let mut creatures: Vec<_> = catalogue
            .entries
            .iter()
            .filter(|(id, e)| {
                e.entry.kind == "creature"
                    && ((e.private && e.owner == campaign.id)
                        || closure.direct.contains(id)
                        || closure.required.contains_key(id))
            })
            .map(|(id, e)| {
                let party = e.private
                    && e.entry
                        .fields
                        .get("splat_external")
                        .is_some_and(|external| {
                            external.get("provider").and_then(Value::as_str) == Some("dndbeyond")
                                && external.get("campaign_id").and_then(Value::as_str)
                                    == campaign.dndbeyond_campaign_id.as_deref()
                        });
                Creature {
                    id: *id,
                    name: e.entry.name.clone(),
                    lower: e.entry.name.to_lowercase(),
                    detail: creature_detail(&e.entry, party),
                    party,
                }
            })
            .collect();
        creatures.sort_by_key(|c| (!c.party, c.name.to_lowercase(), c.id));
        let mut encounters: Vec<_> = catalogue
            .entries
            .values()
            .filter(|e| e.private && e.owner == campaign.id && e.entry.kind == "encounter")
            .map(|e| (e.entry.path.clone(), e.entry.name.clone()))
            .collect();
        encounters.sort_by_key(|e| e.1.to_lowercase());
        let encounter_summaries = catalogue
            .entries
            .values()
            .filter(|e| e.private && e.owner == campaign.id && e.entry.kind == "encounter")
            .filter_map(|e| {
                let setup: Setup =
                    serde_yaml_ng::from_value(e.entry.fields.get("splat_encounter")?.clone())
                        .ok()?;
                Some((e.entry.path.clone(), roster_summary(&setup)))
            })
            .collect();
        let issues = catalogue
            .errors
            .iter()
            .chain(&closure.errors)
            .cloned()
            .collect();
        let options = crate::encounter_settings::Options::load(campaign, &catalogue)?;
        let runs = crate::encounter_run::Run::list(&options.directory(campaign)?, campaign.id)?;
        Ok(Self {
            options: options.clone(),
            saved_options: options,
            campaign_text: document.text,
            runs,
            catalogue,
            creatures,
            encounters,
            encounter_summaries,
            issues,
        })
    }
}
#[derive(Default)]
pub struct Encounters {
    active: Option<Campaign>,
    snapshot: Option<Snapshot>,
    pending: Option<mpsc::Receiver<Result<Snapshot, String>>>,
    /// The load in progress keeps the open draft and any edited settings.
    refreshing: bool,
    draft: Option<Draft>,
    search: String,
    error: Option<String>,
    message: Option<String>,
    pub saved: bool,
    run: Option<crate::encounter_run::Run>,
    /// A run just opened, so the shell goes to the Run page.
    pub go_to_run: bool,
    /// A creature to read beside the draft, in Lookup.
    pub peek: Option<Uuid>,
    focus_name: bool,
    focus_search: bool,
    editor_pane: EditorPane,
    expanded: Option<String>,
}
#[derive(Clone, Copy, Default, PartialEq)]
enum EditorPane {
    #[default]
    Roster,
    Add,
}
const NAME_FIELD: &str = "Encounter name";
const SEARCH_FIELD: &str = "Find a creature or party member";
const BLOCKED_BY_RUN: &str = "Close the open run first.";
const BLOCKED_BY_DRAFT: &str = "Save or discard the open encounter first.";
impl Encounters {
    #[cfg(test)]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    #[cfg(test)]
    pub fn simulate_sync_conflict(&mut self, name: &str) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_sync_conflict(name);
    }

    #[cfg(test)]
    pub fn simulate_connection(&mut self) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_connection();
    }

    #[cfg(test)]
    pub fn simulate_sync_state(&mut self, name: &str, state: crate::combat_sync::SyncState) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_sync_state(name, state);
    }

    #[cfg(test)]
    pub fn simulate_sheet_update(&mut self, name: &str) {
        self.run
            .as_mut()
            .expect("an open run")
            .note_sheet_update(&[name.to_owned()]);
    }

    pub fn has_run(&self) -> bool {
        self.run.is_some()
    }
    pub fn run_mut(&mut self) -> Option<&mut crate::encounter_run::Run> {
        self.run.as_mut()
    }
    pub fn run(&self) -> Option<&crate::encounter_run::Run> {
        self.run.as_ref()
    }
    pub fn unsaved(&self) -> bool {
        self.run
            .as_ref()
            .is_some_and(crate::encounter_run::Run::needs_save)
            || self.draft.as_ref().is_some_and(Draft::dirty)
            || self
                .snapshot
                .as_ref()
                .is_some_and(|s| s.options != s.saved_options)
    }
    pub fn dirty(&self) -> bool {
        self.has_run() || self.unsaved()
    }
    pub fn draft_name(&self) -> Option<String> {
        self.draft.as_ref().map(|d| d.name.clone())
    }
    /// Add a freshly copied creature to the open draft.
    pub fn add_to_draft(&mut self, id: Uuid) {
        if let Some(draft) = &mut self.draft {
            draft.add(id, false);
        }
    }
    pub fn draft_dirty(&self) -> bool {
        self.draft.as_ref().is_some_and(Draft::dirty)
    }
    pub fn pending_sync_names(&self) -> Vec<String> {
        self.run
            .as_ref()
            .map_or_else(Vec::new, crate::encounter_run::Run::pending_sync_names)
    }
    pub fn pending_sync(&self) -> bool {
        self.run
            .as_ref()
            .is_some_and(crate::encounter_run::Run::pending_sync)
    }
    fn reload(&mut self, ctx: &egui::Context, campaign: Campaign, modules: Vec<Module>) {
        self.active = Some(campaign.clone());
        self.draft = None;
        self.snapshot = None;
        self.error = None;
        self.refreshing = false;
        self.start_load(ctx, campaign, modules);
    }
    fn start_load(&mut self, ctx: &egui::Context, campaign: Campaign, modules: Vec<Module>) {
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let _ = tx.send(Snapshot::load(&campaign, &modules));
            ctx.request_repaint();
        });
    }
    /// New content on disk. The open draft stays, however unsaved (T8.1).
    pub fn refresh_content(
        &mut self,
        ctx: &egui::Context,
        campaigns: &Campaigns,
        root: &std::path::Path,
    ) {
        if let Some(active) = campaigns.active_campaign()
            && self
                .active
                .as_ref()
                .is_some_and(|c| c.id == active.id && c.path == active.path)
        {
            let _ = root;
            self.refreshing = true;
            self.start_load(ctx, active, campaigns.modules().to_vec());
        }
    }
    /// Keep the snapshot current. The shell calls this every frame.
    pub fn poll(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        if let Some(active) = campaigns.active_campaign()
            && self
                .active
                .as_ref()
                .is_none_or(|c| c.id != active.id || c.path != active.path)
            && !self.dirty()
        {
            self.reload(ctx, active, campaigns.modules().to_vec());
        }
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    match result {
                        Ok(mut s) => {
                            if std::mem::take(&mut self.refreshing)
                                && let Some(old) = &self.snapshot
                            {
                                s.options = old.options.clone();
                                s.saved_options = old.saved_options.clone();
                            }
                            self.snapshot = Some(s);
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.error = Some("Loading stopped. Reload to retry.".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
    }
    /// Ctrl+S on this page saves the open encounter.
    pub fn save_shortcut(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        if self.draft_dirty()
            && ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S))
            && let Some(active) = campaigns.active_campaign()
        {
            self.save_draft(&active, campaigns.modules());
        }
    }
    fn save_draft(&mut self, active: &Campaign, modules: &[Module]) {
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        match draft.save(active, modules) {
            Ok(()) => {
                self.saved = true;
                self.error = None;
                self.message = Some("Encounter saved.".into());
                let (path, name) = (draft.path.clone(), draft.name.clone());
                let summary = roster_summary(&draft.setup);
                if let Some(s) = &mut self.snapshot {
                    s.encounters.retain(|(p, _)| p != &path);
                    s.encounter_summaries.insert(path.clone(), summary);
                    s.encounters.push((path, name));
                    s.encounters.sort_by_key(|e| e.1.to_lowercase());
                }
            }
            Err(e) => self.error = Some(e),
        }
    }
    fn start_run(
        &self,
        campaign: &Campaign,
        modules: &[Module],
    ) -> Result<crate::encounter_run::Run, String> {
        let draft = self
            .draft
            .as_ref()
            .ok_or("Select a saved encounter first.")?;
        if draft.dirty() {
            return Err("Save the encounter before running it.".into());
        }
        let fresh = Draft::open(&campaign.path, &draft.path)?;
        if fresh.baseline != draft.baseline {
            return Err("The encounter changed on disk. Reload it before running.".into());
        }
        let snapshot = Snapshot::load(campaign, modules)?;
        fresh.validate(&snapshot)?;
        let mut participants = Vec::new();
        for slot in &fresh.setup.roster {
            let reference = slot
                .creature
                .as_ref()
                .ok_or("Party-slot bindings are not supported by this runner yet.")?;
            let id = snapshot.catalogue.resolve_ref(reference)?;
            let entry = snapshot.catalogue.entries[&id].entry.clone();
            let present = match slot.extra.get("initial_presence").and_then(Value::as_str) {
                None | Some("active") => true,
                Some("absent" | "removed") => false,
                Some(_) => return Err(format!("{}: invalid starting presence.", slot.key)),
            };
            let mut participant = crate::encounter_run::Participant::new(
                slot.key.clone(),
                slot.side.clone(),
                entry,
                &slot.initial,
                present,
            )?;
            participant.initiative_group = slot
                .extra
                .get("initiative_group")
                .and_then(Value::as_str)
                .map(str::to_owned);
            if let Some(role) = slot.controller.get("role").and_then(Value::as_str) {
                participant.npc = role != "player";
            }
            participants.push(participant);
        }
        let setup = serde_yaml_ng::to_string(&fresh.setup).map_err(|e| e.to_string())?;
        let entry = Entry::parse(&fresh.path, &fresh.source)?;
        let mut run = crate::encounter_run::Run::new(
            fresh.name,
            participants,
            format!(
                "{}\n\n## Setup (DM reference)\n\nPhases, assumptions, completion conditions, and limits are resolved manually.\n\n```yaml\n{setup}\n```",
                entry.body
            ),
        )?;
        run.configure_storage(
            campaign.id,
            &snapshot.options.directory(campaign)?,
            snapshot.options.initiative,
        )?;
        Ok(run)
    }
    pub fn tick_sync(
        &mut self,
        ctx: &egui::Context,
        session: Option<crate::auth::Session>,
        epoch: u64,
        active: Option<&Campaign>,
    ) {
        if let Some(run) = &mut self.run {
            run.tick_sync(
                ctx,
                session,
                epoch,
                active.and_then(|c| c.dndbeyond_campaign_id.as_deref()),
            );
            run.autosave();
        }
    }
    /// Reopen a run after a restart, with its participants' sync turned back on.
    pub fn restore_run(&mut self, path: &std::path::Path, campaign: uuid::Uuid, synced: &[String]) {
        if self.run.is_none()
            && let Ok(mut run) = crate::encounter_run::Run::open(path, campaign)
        {
            run.restore_sync(synced);
            self.run = Some(run);
            self.go_to_run = true;
        }
    }
    /// The Run page. Returns true when the run has just closed.
    pub fn show_run(&mut self, ui: &mut egui::Ui, active: &Campaign) -> bool {
        let Some(run) = &mut self.run else {
            return false;
        };
        if !run.show(ui) {
            return false;
        }
        self.run = None;
        if let Some(s) = &mut self.snapshot {
            match s
                .options
                .directory(active)
                .and_then(|dir| crate::encounter_run::Run::list(&dir, active.id))
            {
                Ok(runs) => s.runs = runs,
                Err(e) => self.error = Some(e),
            }
        }
        true
    }
    pub fn show(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns) {
        self.poll(ui.ctx(), campaigns);
        let Some(active) = campaigns.active_campaign() else {
            return;
        };
        if self
            .active
            .as_ref()
            .is_none_or(|c| c.id != active.id || c.path != active.path)
        {
            design::message(
                ui,
                Tone::Warning,
                "Save or discard the previous campaign's encounter first.",
            );
            return;
        }
        if self.draft_dirty() {
            self.message = None;
        }
        if self.draft.is_some() {
            self.editor_view(ui, &active, campaigns);
        } else {
            self.list_view(ui, &active, campaigns);
        }
    }

    fn start_blocked(&self) -> Option<&'static str> {
        if self.has_run() {
            Some(BLOCKED_BY_RUN)
        } else if self.draft_dirty() {
            Some(BLOCKED_BY_DRAFT)
        } else {
            None
        }
    }

    fn results(&mut self, ui: &mut egui::Ui) {
        if let Some(e) = &self.error {
            design::message(ui, Tone::Error, e);
        }
        if let Some(m) = &self.message {
            design::message(ui, Tone::Success, m);
        }
    }

    fn list_view(&mut self, ui: &mut egui::Ui, active: &Campaign, campaigns: &Campaigns) {
        let loading = self.pending.is_some();
        let blocked = self.start_blocked();
        let settings_changed = self.unsaved();
        let mut new = false;
        let mut reload = false;
        design::page_header(ui, "Encounters", Some(&active.name), |ui| {
            new = Action::secondary("New encounter")
                .icon(icons::ADD)
                .blocked(blocked.or(loading.then_some("Still reading this campaign.")))
                .show(ui)
                .clicked();
            reload = Action::quiet("Reload")
                .icon(icons::REFRESH)
                .blocked(
                    blocked
                        .or(loading.then_some("Still reading this campaign."))
                        .or(settings_changed.then_some(
                            "Save or discard the changes in Campaign, Rules and runs first.",
                        )),
                )
                .hint("Read encounters and runs from disk again")
                .show(ui)
                .clicked();
        });
        if new {
            self.draft = Some(Draft::new());
            self.focus_name = true;
            self.error = None;
            self.message = None;
            return;
        }
        if reload {
            self.reload(ui.ctx(), active.clone(), campaigns.modules().to_vec());
            return;
        }
        self.results(ui);
        if self.pending.is_some() {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(
                    Type::Body
                        .rich(ui, "Reading this campaign…")
                        .color(colors().muted),
                );
            });
            return;
        }
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        if !snapshot.issues.is_empty() {
            design::section(
                ui,
                format!("Content issues ({})", snapshot.issues.len()),
                |ui| {
                    for issue in &snapshot.issues {
                        ui.add(
                            egui::Label::new(Type::Small.rich(ui, issue).color(colors().muted))
                                .wrap(),
                        );
                    }
                },
            );
        }
        let mut open_path = None;
        let mut run_path = None;
        let mut resume = None;
        let two_columns = ui.available_width() >= 640.0;
        if snapshot.encounters.is_empty() && snapshot.runs.is_empty() {
            let mut create = false;
            design::empty_state(
                ui,
                icons::ENCOUNTERS,
                "No encounters yet",
                "Build a roster from your party and the creatures this campaign can use. Then run it at the table.",
                |ui| {
                    create = Action::primary("New encounter")
                        .icon(icons::ADD)
                        .blocked(blocked)
                        .name("Create a first encounter")
                        .show(ui)
                        .clicked();
                },
            );
            if create {
                self.draft = Some(Draft::new());
                self.focus_name = true;
            }
            return;
        }
        let run_list = |ui: &mut egui::Ui, resume: &mut Option<std::path::PathBuf>| {
            let heading = format!("Saved runs ({})", snapshot.runs.len());
            ui.label(Type::Heading.rich(ui, heading).color(colors().text));
            design::rule(ui);
            if snapshot.runs.is_empty() {
                ui.label(
                    Type::Body
                        .rich(ui, "No saved runs yet. Start an encounter to create one.")
                        .color(colors().muted),
                );
            }
            // Group by encounter. Runs still in progress come before finished ones.
            let mut groups: Vec<&str> = Vec::new();
            for run in &snapshot.runs {
                if !groups.contains(&run.encounter.as_str()) {
                    groups.push(&run.encounter);
                }
            }
            for group in groups {
                if !group.is_empty() {
                    ui.add_space(design::XS);
                    design::group_label(ui, group);
                }
                let mut rows: Vec<_> = snapshot
                    .runs
                    .iter()
                    .filter(|run| run.encounter == group)
                    .collect();
                rows.sort_by_key(|run| run.finished);
                for info in rows {
                    ui.push_id(&info.path, |ui| {
                        ui.horizontal(|ui| {
                            ui.set_min_height(Density::Scan.row());
                            // At the table, picking up the interrupted run comes first.
                            let action = if info.finished {
                                Action::secondary("Resume").density(Density::Read)
                            } else {
                                Action::primary("Resume")
                            };
                            if info.readable
                                && action.blocked(self.start_blocked()).show(ui).clicked()
                            {
                                *resume = Some(info.path.clone());
                            }
                            let color = if info.finished {
                                colors().muted
                            } else {
                                colors().text
                            };
                            let response = ui.add(
                                egui::Label::new(Type::Body.rich(ui, &info.label).color(color))
                                    .truncate(),
                            );
                            // The group heading shows the encounter. The tree needs it too.
                            design::named(
                                ui,
                                &response,
                                &format!("{} · {}", info.encounter, info.label),
                            );
                            response.on_hover_text(info.path.display().to_string());
                        });
                    });
                }
            }
        };
        let template_list = |ui: &mut egui::Ui,
                             open_path: &mut Option<String>,
                             run_path: &mut Option<String>| {
            ui.label(Type::Heading.rich(ui, "Encounters").color(colors().text));
            design::rule(ui);
            if snapshot.encounters.is_empty() {
                ui.label(
                    Type::Body
                        .rich(ui, "No encounters yet. Create one with New encounter.")
                        .color(colors().muted),
                );
            }
            for (path, name) in &snapshot.encounters {
                ui.push_id(path, |ui| {
                    ui.horizontal(|ui| {
                        ui.set_min_height(Density::Scan.row() + design::S);
                        ui.vertical(|ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            ui.set_max_width((ui.available_width() - 180.0).max(100.0));
                            ui.add(
                                egui::Label::new(Type::Strong.rich(ui, name).color(colors().text))
                                    .truncate(),
                            );
                            if let Some(summary) = snapshot.encounter_summaries.get(path) {
                                ui.add(
                                    egui::Label::new(
                                        Type::Small.rich(ui, summary).color(colors().muted),
                                    )
                                    .truncate(),
                                );
                            }
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if Action::secondary("Run encounter")
                                .icon(icons::RUN)
                                .density(Density::Read)
                                .blocked(self.start_blocked())
                                .show(ui)
                                .clicked()
                            {
                                *run_path = Some(path.clone());
                            }
                            if Action::quiet("Edit")
                                .density(Density::Read)
                                .blocked(self.start_blocked())
                                .show(ui)
                                .clicked()
                            {
                                *open_path = Some(path.clone());
                            }
                        });
                    });
                });
            }
        };
        egui::ScrollArea::vertical()
            .id_salt("encounter_list")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if two_columns {
                    ui.columns(2, |columns| {
                        run_list(&mut columns[0], &mut resume);
                        template_list(&mut columns[1], &mut open_path, &mut run_path);
                    });
                } else {
                    run_list(ui, &mut resume);
                    ui.add_space(design::L);
                    template_list(ui, &mut open_path, &mut run_path);
                }
            });
        if let Some(path) = resume {
            match crate::encounter_run::Run::open(&path, active.id) {
                Ok(run) => {
                    self.run = Some(run);
                    self.go_to_run = true;
                    self.error = None;
                }
                Err(e) => self.error = Some(e),
            }
        }
        if let Some(path) = open_path {
            match Draft::open(&active.path, &path) {
                Ok(d) => {
                    self.draft = Some(d);
                    self.error = None;
                    self.message = None;
                }
                Err(e) => self.error = Some(e),
            }
        }
        if let Some(path) = run_path {
            match Draft::open(&active.path, &path) {
                Ok(d) => {
                    self.draft = Some(d);
                    self.open_run(active, campaigns.modules());
                    self.draft = None;
                }
                Err(e) => self.error = Some(e),
            }
        }
    }

    fn open_run(&mut self, active: &Campaign, modules: &[Module]) {
        match self.start_run(active, modules) {
            Ok(run) => {
                self.run = Some(run);
                self.go_to_run = true;
                self.error = None;
            }
            Err(error) => self.error = Some(error),
        }
    }

    fn editor_view(&mut self, ui: &mut egui::Ui, active: &Campaign, campaigns: &Campaigns) {
        let dirty = self.draft_dirty();
        let mut back = false;
        let mut save = false;
        let mut discard = false;
        let mut run_it = false;
        // Beside Lookup at the minimum window, the roster and the search take turns.
        let compact = ui.available_width() < 760.0;
        let has_run = self.has_run();
        let loading = self.pending.is_some();
        let draft_ref = self.draft.as_mut().expect("the editor has a draft");
        let focus_name = &mut self.focus_name;
        let mut title = |ui: &mut egui::Ui, width: f32| {
            let response = ui.add(
                egui::TextEdit::singleline(&mut draft_ref.name)
                    .id(design::focus_id(NAME_FIELD))
                    .desired_width(width)
                    .min_size(egui::vec2(0.0, Density::Scan.control()))
                    .margin(egui::vec2(design::S, 5.0))
                    .font(Type::Section.font(ui)),
            );
            design::named(ui, &response, &format!("{NAME_FIELD}: {}", draft_ref.name));
            if std::mem::take(focus_name) {
                design::focus_and_select(ui, response.id, &draft_ref.name);
            }
            if dirty {
                design::status(
                    ui,
                    Status {
                        icon: icons::DRAFT,
                        word: "Unsaved",
                        color: colors().document.unsaved,
                    },
                );
            }
        };
        let mut actions = |ui: &mut egui::Ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                run_it = Action::primary("Run encounter")
                    .icon(icons::RUN)
                    .blocked(
                        has_run
                            .then_some(BLOCKED_BY_RUN)
                            .or(dirty.then_some("Save the encounter before starting a run."))
                            .or(loading.then_some("Still reading this campaign.")),
                    )
                    .show(ui)
                    .clicked();
                save = Action::secondary("Save encounter")
                    .blocked_if(!dirty, "No changes to save.")
                    .key("Ctrl+S")
                    .show(ui)
                    .clicked();
                discard = Action::quiet("Discard changes")
                    .blocked_if(!dirty, "No changes to discard.")
                    .show(ui)
                    .clicked();
            });
        };
        let back_action = |ui: &mut egui::Ui| {
            Action::quiet("Encounters")
                .icon(icons::BACK)
                .name("Back to encounters")
                .blocked_if(dirty, "Save or discard this encounter first.")
                .show(ui)
                .clicked()
        };
        if compact {
            ui.horizontal(|ui| {
                back = back_action(ui);
                let width = (ui.available_width() - 110.0).max(120.0);
                title(ui, width);
            });
            ui.horizontal(|ui| actions(ui));
        } else {
            ui.horizontal(|ui| {
                back = back_action(ui);
                let width = (ui.available_width() - 610.0).clamp(120.0, 360.0);
                title(ui, width);
                actions(ui);
            });
        }
        design::rule(ui);
        if back {
            self.draft = None;
            self.message = None;
            self.error = None;
            return;
        }
        if discard {
            let draft = self.draft.as_ref().expect("the editor has a draft");
            match &draft.baseline {
                Some(_) => match Draft::open(&active.path, &draft.path) {
                    Ok(d) => self.draft = Some(d),
                    Err(e) => {
                        self.error = Some(e);
                        self.draft = None;
                    }
                },
                None => self.draft = None,
            }
            return;
        }
        if save {
            self.save_draft(active, campaigns.modules());
        }
        if run_it {
            self.open_run(active, campaigns.modules());
        }
        self.results(ui);
        let Some(snapshot) = &self.snapshot else {
            ui.spinner();
            return;
        };
        let draft = self.draft.as_mut().expect("the editor has a draft");
        let mut pane = self.editor_pane;
        if compact {
            design::segmented(
                ui,
                &mut pane,
                &[
                    (EditorPane::Roster, "Roster"),
                    (EditorPane::Add, "Add participants"),
                ],
            );
            self.editor_pane = pane;
        }
        let search = &mut self.search;
        let focus_search = &mut self.focus_search;
        let peek = &mut self.peek;
        let expanded = &mut self.expanded;
        if compact {
            match pane {
                EditorPane::Roster => {
                    Self::roster_pane(ui, draft, snapshot, focus_search, expanded);
                }
                EditorPane::Add => {
                    Self::add_pane(ui, draft, snapshot, search, focus_search, peek);
                }
            }
            return;
        }
        // In a wide window the roster keeps a name near its controls, and the two panes
        // stay together at the left.
        let width = design::side_panel_width(ui.ctx().content_rect().width())
            .max(ui.available_width() - ROSTER_MAX_WIDTH);
        egui::SidePanel::right("encounter_add")
            .resizable(false)
            .exact_width(width)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::M as i8,
                right: 0,
                top: design::S as i8,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                let room = egui::vec2(
                    ui.available_width().min(design::PANEL_MAX_WIDTH),
                    ui.available_height(),
                );
                ui.allocate_ui_with_layout(room, egui::Layout::top_down(egui::Align::Min), |ui| {
                    ui.set_max_width(room.x);
                    Self::add_pane(ui, draft, snapshot, search, focus_search, peek);
                });
            });
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: 0,
                right: 0,
                top: design::S as i8,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                Self::roster_pane(ui, draft, snapshot, focus_search, expanded);
            });
    }

    fn add_pane(
        ui: &mut egui::Ui,
        draft: &mut Draft,
        snapshot: &Snapshot,
        search: &mut String,
        focus_search: &mut bool,
        peek: &mut Option<Uuid>,
    ) {
        ui.label(
            Type::Heading
                .rich(ui, "Add participants")
                .color(colors().text),
        );
        ui.add(
            egui::Label::new(
                Type::Small
                    .rich(ui, "Party members use the saved local character copies.")
                    .color(colors().muted),
            )
            .wrap(),
        );
        if Action::secondary("Add party")
            .icon(icons::PARTY)
            .blocked_if(
                !snapshot.creatures.iter().any(|c| c.party),
                "No saved party copies. Refresh Party first.",
            )
            .show(ui)
            .clicked()
        {
            for c in &snapshot.creatures {
                if c.party {
                    draft.add(c.id, true);
                }
            }
            *focus_search = true;
        }
        let response = design::search_field(
            ui,
            SEARCH_FIELD,
            search,
            "Find a creature…",
            ui.available_width(),
        );
        if std::mem::take(focus_search) {
            design::focus_and_select(ui, response.id, search);
        }
        let query = search.to_lowercase();
        let matches: Vec<&Creature> = snapshot
            .creatures
            .iter()
            .filter(|c| c.lower.contains(&query))
            .collect();
        if matches.is_empty() {
            ui.add(
                egui::Label::new(
                    Type::Body
                        .rich(
                            ui,
                            "No creatures match. Check the spelling, or the packages this campaign uses.",
                        )
                        .color(colors().muted),
                )
                .wrap(),
            );
        }
        ui.spacing_mut().item_spacing.y = 0.0;
        egui::ScrollArea::vertical()
            .id_salt(("encounter_results", &query))
            .auto_shrink([false, false])
            .show_rows(ui, RESULT_ROW, matches.len(), |ui, range| {
                for c in &matches[range] {
                    ui.push_id(c.id, |ui| {
                        ui.horizontal(|ui| {
                            ui.set_min_height(RESULT_ROW);
                            if Action::secondary("Add")
                                .density(Density::Read)
                                .show(ui)
                                .clicked()
                            {
                                draft.add(c.id, c.party);
                                *focus_search = true;
                            }
                            if Action::quiet("Read")
                                .density(Density::Read)
                                .hint("Read this entry in Lookup. The draft stays open.")
                                .show(ui)
                                .clicked()
                            {
                                *peek = Some(c.id);
                            }
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                ui.add(
                                    egui::Label::new(
                                        Type::Body.rich(ui, &c.name).color(colors().text),
                                    )
                                    .truncate(),
                                );
                                ui.add(
                                    egui::Label::new(
                                        Type::Small.rich(ui, &c.detail).color(colors().muted),
                                    )
                                    .truncate(),
                                );
                            });
                        });
                    });
                }
            });
    }

    fn roster_pane(
        ui: &mut egui::Ui,
        draft: &mut Draft,
        snapshot: &Snapshot,
        focus_search: &mut bool,
        expanded: &mut Option<String>,
    ) {
        ui.label(
            Type::Heading
                .rich(ui, format!("Roster ({})", draft.setup.roster.len()))
                .color(colors().text),
        );
        if draft.setup.roster.is_empty() {
            ui.add_space(design::S);
            ui.label(
                Type::Body
                    .rich(ui, "Add the party and some creatures from the list.")
                    .color(colors().muted),
            );
        }
        egui::ScrollArea::vertical()
            .id_salt("encounter_editor")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                Self::roster(ui, draft, snapshot, focus_search, expanded);
            });
    }

    fn roster(
        ui: &mut egui::Ui,
        draft: &mut Draft,
        snapshot: &Snapshot,
        focus_search: &mut bool,
        expanded: &mut Option<String>,
    ) {
        let sides: Vec<_> = draft
            .setup
            .sides
            .keys()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect();
        let (mut remove, mut duplicate, mut raise, mut lower) = (None, None, None, None);
        let count = draft.setup.roster.len();
        for (i, slot) in draft.setup.roster.iter_mut().enumerate() {
            ui.push_id(&slot.key, |ui| {
                let resolved = slot
                    .creature
                    .as_ref()
                    .and_then(|r| snapshot.catalogue.resolve_ref(r).ok());
                let name = resolved
                    .and_then(|id| snapshot.catalogue.entries.get(&id))
                    .map(|e| e.entry.name.clone())
                    .unwrap_or_else(|| {
                        format!(
                            "Unavailable creature / party slot: {}",
                            slot.party_slot.as_deref().unwrap_or(&slot.key)
                        )
                    });
                let detail = resolved
                    .and_then(|id| snapshot.creatures.iter().find(|c| c.id == id))
                    .map_or("", |c| c.detail.as_str());
                let is_open = expanded.as_deref() == Some(slot.key.as_str());
                ui.horizontal(|ui| {
                    ui.set_min_height(Density::Glance.row());
                    // Every slot is reserved, so columns line up between rows.
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.spacing_mut().item_spacing.x = 2.0;
                        if design::icon_button(ui, icons::REMOVE, "Remove").clicked() {
                            remove = Some(i);
                        }
                        if design::icon_button(ui, icons::COPY, "Duplicate").clicked() {
                            duplicate = Some(i);
                        }
                        if i + 1 < count {
                            if design::icon_button(ui, icons::DOWN, "Move down").clicked() {
                                lower = Some(i);
                            }
                        } else {
                            ui.add_space(Density::Read.control());
                        }
                        if i > 0 {
                            if design::icon_button(ui, icons::UP, "Move up").clicked() {
                                raise = Some(i);
                            }
                        } else {
                            ui.add_space(Density::Read.control());
                        }
                        if design::icon_button(
                            ui,
                            icons::EXPAND,
                            "Starting HP, temporary HP and initiative group",
                        )
                        .clicked()
                        {
                            *expanded = if is_open {
                                None
                            } else {
                                Some(slot.key.clone())
                            };
                        }
                        ui.add_space(design::XS);
                        let combo = egui::ComboBox::from_id_salt("side")
                            .icon(design::combo_icon)
                            .width(104.0)
                            .selected_text(&slot.side)
                            .show_ui(ui, |ui| {
                                for side in &sides {
                                    ui.selectable_value(&mut slot.side, side.clone(), side);
                                }
                            });
                        design::named_combo(ui, &combo, "Side");
                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            ui.add(
                                egui::Label::new(
                                    Type::Strong
                                        .rich(ui, format!("{}. {name}", i + 1))
                                        .color(colors().text),
                                )
                                .truncate(),
                            );
                            if !detail.is_empty() {
                                ui.add(
                                    egui::Label::new(
                                        Type::Small.rich(ui, detail).color(colors().muted),
                                    )
                                    .truncate(),
                                );
                            }
                        });
                    });
                });
                if is_open {
                    ui.horizontal_wrapped(|ui| {
                        let mut group = slot
                            .extra
                            .get("initiative_group")
                            .and_then(Value::as_str)
                            .unwrap_or("")
                            .to_owned();
                        let response = design::text_input(
                            ui,
                            "Initiative group (optional)",
                            &mut group,
                            "Initiative group",
                            120.0,
                        );
                        if response.changed() {
                            if group.trim().is_empty() {
                                slot.extra.remove("initiative_group");
                            } else {
                                slot.extra.insert("initiative_group".into(), group.into());
                            }
                        }
                        for (field, label) in [("hp", "Starting HP"), ("temporary_hp", "Temp HP")] {
                            let mut enabled = slot.initial.contains_key(field);
                            if ui.checkbox(&mut enabled, label).changed() {
                                if enabled {
                                    slot.initial.insert(field.into(), 0.into());
                                } else {
                                    slot.initial.remove(field);
                                }
                            }
                            if enabled {
                                let mut value =
                                    slot.initial.get(field).and_then(Value::as_u64).unwrap_or(0);
                                let response = ui.add(
                                    egui::DragValue::new(&mut value).range(0..=i64::MAX as u64),
                                );
                                design::named(ui, &response, label);
                                if response.changed() {
                                    slot.initial.insert(field.into(), value.into());
                                }
                            }
                        }
                    });
                    ui.label(
                        Type::Small
                            .rich(
                                ui,
                                "With an override off, the creature's own starting state applies.",
                            )
                            .color(colors().muted),
                    );
                }
                design::rule(ui);
            });
        }
        if let Some(i) = remove {
            draft.setup.roster.remove(i);
        } else if let Some(i) = duplicate {
            let mut slot = draft.setup.roster[i].clone();
            slot.key = format!("actor_{}", Uuid::new_v4().simple());
            draft.setup.roster.insert(i + 1, slot);
            *focus_search = true;
        } else if let Some(i) = raise {
            draft.setup.roster.swap(i, i - 1);
        } else if let Some(i) = lower {
            draft.setup.roster.swap(i, i + 1);
        }
    }

    /// Run folder and initiative rule. These belong to the campaign, not to one encounter.
    pub fn run_settings(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns) {
        self.poll(ui.ctx(), campaigns);
        let Some(active) = campaigns.active_campaign() else {
            return;
        };
        let draft_open = self.draft_dirty();
        let mut settings_saved = false;
        let mut error = None;
        let Some(snapshot) = &mut self.snapshot else {
            ui.spinner();
            return;
        };
        ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
        ui.add_space(design::S);
        ui.add_enabled_ui(!draft_open, |ui| {
            design::text_field(
                ui,
                "Run folder",
                &mut snapshot.options.folder,
                Some("Relative to the campaign folder, or an absolute path."),
                None,
            );
            design::field(ui, "Initiative rule", None, None, |ui| {
                let combo = egui::ComboBox::from_id_salt("campaign_initiative")
                    .icon(design::combo_icon)
                    .width(ui.available_width().min(280.0))
                    .selected_text(snapshot.options.initiative.grouping.label())
                    .show_ui(ui, |ui| {
                        use crate::encounter_settings::Grouping;
                        for mode in [
                            Grouping::Identical,
                            Grouping::Individual,
                            Grouping::BySide,
                            Grouping::Explicit,
                        ] {
                            ui.selectable_value(
                                &mut snapshot.options.initiative.grouping,
                                mode,
                                mode.label(),
                            );
                        }
                    });
                design::named_combo(ui, &combo, "Initiative rule");
            });
            ui.checkbox(
                &mut snapshot.options.initiative.roster_ties,
                "Break initiative ties in roster order",
            );
            ui.label(
                Type::Small
                    .rich(ui, "Side and explicit groups use the first NPC's bonus and roll mode. Players keep their entered initiative.")
                    .color(colors().muted),
            );
            let changed = snapshot.options != snapshot.saved_options;
            ui.horizontal(|ui| {
                if Action::secondary("Save run settings")
                    .blocked_if(!changed, "No changes to save.")
                    .show(ui)
                    .clicked()
                {
                    match snapshot.options.save(&active, &snapshot.campaign_text) {
                        Ok(()) => settings_saved = true,
                        Err(e) => error = Some(e),
                    }
                }
                if Action::quiet("Discard settings changes")
                    .blocked_if(!changed, "No changes to discard.")
                    .show(ui)
                    .clicked()
                {
                    snapshot.options = snapshot.saved_options.clone();
                }
            });
        });
        if draft_open {
            design::message(
                ui,
                Tone::Info,
                "Save or discard the open encounter to change run settings.",
            );
        }
        if let Some(e) = error {
            self.error = Some(e);
        }
        if let Some(e) = &self.error {
            design::message(ui, Tone::Error, e);
        }
        if settings_saved {
            self.saved = true;
            self.reload(ui.ctx(), active, campaigns.modules().to_vec());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn campaign(root: &std::path::Path) -> Campaign {
        Campaign {
            id: Uuid::new_v4(),
            name: "Test campaign".into(),
            path: root.into(),
            dndbeyond_campaign_id: Some("123".into()),
            dndbeyond_campaign_name: None,
        }
    }
    fn creature(root: &std::path::Path, name: &str, party: bool) -> Uuid {
        let text = crate::library::note_text(name, "creature", None);
        let text = if party {
            patch_fields(
                &text,
                &[(
                    "splat_external",
                    Some(yaml("{provider: dndbeyond, campaign_id: '123'}")),
                )],
            )
            .unwrap()
        } else {
            text
        };
        write_note(root, &format!("{name}.md"), None, &text)
            .unwrap()
            .id
    }
    #[test]
    fn save_reopen_party_and_duplicate_monsters_preserves_order_identity_and_state() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let pc = creature(dir.path(), "Hero", true);
        let monster = creature(dir.path(), "Monster", false);
        let mut d = Draft::new();
        d.name = "Ambush".into();
        d.add(pc, true);
        d.add(monster, false);
        d.add(monster, false);
        d.setup.roster[1].initial.insert("hp".into(), 7.into());
        d.setup.roster[2].initial.insert("hp".into(), 12.into());
        d.setup.roster[2]
            .initial
            .insert("temporary_hp".into(), 3.into());
        d.save(&campaign, &[]).unwrap();
        assert!(!d.dirty());
        let reopened = Draft::open(dir.path(), &d.path).unwrap();
        assert_eq!(reopened.name, "Ambush");
        assert_eq!(reopened.setup, d.setup);
        assert_ne!(reopened.setup.roster[1].key, reopened.setup.roster[2].key);
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        assert!(snapshot.creatures.iter().any(|c| c.id == pc && c.party));
        assert!(snapshot.encounters.iter().any(|(p, _)| p == &d.path));
        assert!(dir.path().join("Campaign.md").exists());
    }
    #[test]
    fn editing_preserves_notes_unknown_fields_and_detects_external_changes() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let mut d = Draft::new();
        d.source.push_str("## DM notes\nKeep this text.\n");
        d.setup
            .extra
            .insert("phases".into(), yaml("[{key: arrival, custom: kept}]"));
        d.save(&campaign, &[]).unwrap();
        let id = Entry::parse(&d.path, &d.text().unwrap()).unwrap().id;
        d.name = "Renamed".into();
        d.save(&campaign, &[]).unwrap();
        let saved = fs::read_to_string(dir.path().join(&d.path)).unwrap();
        assert!(saved.contains("## DM notes\nKeep this text."));
        assert_eq!(Entry::parse(&d.path, &saved).unwrap().id, id);
        assert_eq!(
            Draft::open(dir.path(), &d.path).unwrap().setup.extra,
            d.setup.extra
        );
        let external = format!("{saved}\nExternal edit\n");
        fs::write(dir.path().join(&d.path), &external).unwrap();
        d.name = "Blocked".into();
        assert!(d.save(&campaign, &[]).is_err());
        assert_eq!(
            fs::read_to_string(dir.path().join(&d.path)).unwrap(),
            external
        );
        assert!(d.dirty());
    }
    #[test]
    fn invalid_roster_and_missing_creature_cannot_overwrite_saved_encounter() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = creature(dir.path(), "Monster", false);
        let mut d = Draft::new();
        d.add(id, false);
        d.save(&campaign, &[]).unwrap();
        let saved = d.baseline.clone().unwrap();
        d.setup.roster.push(d.setup.roster[0].clone());
        assert!(d.save(&campaign, &[]).is_err());
        d.setup.roster.pop();
        d.setup.roster[0].initial.insert("hp".into(), (-1).into());
        assert!(d.save(&campaign, &[]).is_err());
        d.setup.roster[0].initial.clear();
        d.setup.roster[0].side = "missing".into();
        assert!(d.save(&campaign, &[]).is_err());
        d.setup.roster[0].side = "enemies".into();
        fs::remove_file(dir.path().join("Monster.md")).unwrap();
        assert!(d.save(&campaign, &[]).is_err());
        assert_eq!(fs::read_to_string(dir.path().join(&d.path)).unwrap(), saved);
    }
    #[test]
    fn unselected_library_creatures_are_not_offered() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let mut document = CampaignDocument::load(&campaign).unwrap();
        document.selections.clear();
        document.save().unwrap();
        let local = creature(dir.path(), "Local", false);
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        assert_eq!(snapshot.creatures.len(), 1);
        assert_eq!(snapshot.creatures[0].id, local);
    }
    #[test]
    fn editor_renders_at_supported_sizes_without_changing_the_draft() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = creature(dir.path(), "Hero", true);
        let settings = dir.path().join("settings.json");
        let mut store = crate::persistence::Store::at(settings.clone());
        let mut data = store.load().unwrap();
        data.active_campaign_id = Some(campaign.id);
        data.campaigns.push(campaign.clone());
        store.save(&data).unwrap();
        let campaigns = Campaigns::at(settings);
        let mut draft = Draft::new();
        draft.add(id, true);
        let expected = draft.text().unwrap();
        let mut editor = Encounters {
            active: Some(campaign.clone()),
            snapshot: Some(Snapshot::load(&campaign, &[]).unwrap()),
            draft: Some(draft),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        for size in [egui::vec2(960.0, 640.0), egui::vec2(1200.0, 800.0)] {
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| editor.show(ui, &campaigns));
                },
            );
            assert!(!output.shapes.is_empty());
            assert!(editor.dirty());
            assert_eq!(editor.draft.as_ref().unwrap().text().unwrap(), expected);
        }
    }
    #[test]
    fn dm_run_requires_saved_current_template_and_protects_its_memory_state() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = creature(dir.path(), "Hero", true);
        let mut draft = Draft::new();
        draft.add(id, true);
        let mut editor = Encounters {
            active: Some(campaign.clone()),
            draft: Some(draft),
            ..Default::default()
        };
        assert!(editor.start_run(&campaign, &[]).is_err());
        editor.draft.as_mut().unwrap().save(&campaign, &[]).unwrap();
        let path = dir.path().join(&editor.draft.as_ref().unwrap().path);
        let original = fs::read_to_string(&path).unwrap();
        let character = fs::read_to_string(dir.path().join("Hero.md")).unwrap();
        let run = editor.start_run(&campaign, &[]).unwrap();
        assert!(!editor.dirty());
        editor.run = Some(run);
        assert!(editor.dirty());
        assert!(editor.has_run());
        assert_eq!(fs::read_to_string(&path).unwrap(), original);
        assert_eq!(
            fs::read_to_string(dir.path().join("Hero.md")).unwrap(),
            character
        );
        editor.run = None;
        assert!(!editor.dirty());
        fs::write(&path, format!("{original}External edit\n")).unwrap();
        assert!(editor.start_run(&campaign, &[]).is_err());
    }
}
