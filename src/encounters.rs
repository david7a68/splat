//! Campaign encounter templates. Runtime state is deliberately separate.
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue, EntryRef, patch_fields},
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
            .map_err(|e| format!("This encounter needs repair in Campaign → Content: {e}"))?;
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
                (None, Some(_)) => return Err("This encounter uses party bindings. Edit those slots in Campaign → Content; this editor currently adds local character copies.".into()),
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
    party: bool,
}
struct Snapshot {
    catalogue: Catalogue,
    creatures: Vec<Creature>,
    encounters: Vec<(String, String)>,
    issues: Vec<String>,
    options: crate::encounter_settings::Options,
    saved_options: crate::encounter_settings::Options,
    campaign_text: String,
    runs: Vec<(std::path::PathBuf, String)>,
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
            .map(|(id, e)| Creature {
                id: *id,
                name: e.entry.name.clone(),
                party: e.private
                    && e.entry
                        .fields
                        .get("splat_external")
                        .is_some_and(|external| {
                            external.get("provider").and_then(Value::as_str) == Some("dndbeyond")
                                && external.get("campaign_id").and_then(Value::as_str)
                                    == campaign.dndbeyond_campaign_id.as_deref()
                        }),
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
            issues,
        })
    }
}
#[derive(Default)]
pub struct Encounters {
    active: Option<Campaign>,
    snapshot: Option<Snapshot>,
    pending: Option<mpsc::Receiver<Result<Snapshot, String>>>,
    draft: Option<Draft>,
    search: String,
    error: Option<String>,
    message: Option<String>,
    pub saved: bool,
    run: Option<crate::encounter_run::Run>,
}
impl Encounters {
    pub fn has_run(&self) -> bool {
        self.run.is_some()
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
    fn reload(&mut self, ctx: &egui::Context, campaign: Campaign, modules: Vec<Module>) {
        self.active = Some(campaign.clone());
        self.draft = None;
        self.snapshot = None;
        self.error = None;
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let _ = tx.send(Snapshot::load(&campaign, &modules));
            ctx.request_repaint();
        });
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
    pub fn show(&mut self, ui: &mut egui::Ui, campaigns: &Campaigns) {
        ui.visuals_mut().override_text_color = Some(egui::Color32::from_gray(215));
        ui.visuals_mut().weak_text_color = Some(egui::Color32::from_gray(165));
        let Some(active) = campaigns.active_campaign() else {
            return;
        };
        if self
            .active
            .as_ref()
            .is_none_or(|c| c.id != active.id || c.path != active.path)
        {
            if self.dirty() {
                ui.label("Save or discard the previous campaign's encounter first.");
                return;
            }
            self.reload(ui.ctx(), active.clone(), campaigns.modules().to_vec());
        }
        if let Some(run) = &mut self.run {
            if run.show(ui) {
                self.run = None;
                if let Some(s) = &mut self.snapshot {
                    match s
                        .options
                        .directory(&active)
                        .and_then(|dir| crate::encounter_run::Run::list(&dir, active.id))
                    {
                        Ok(runs) => s.runs = runs,
                        Err(e) => self.error = Some(e),
                    }
                }
            }
            return;
        }
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    match result {
                        Ok(s) => self.snapshot = Some(s),
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
        if self.dirty() {
            self.message = None;
        }
        ui.heading("Encounters");
        ui.label("Build a roster, then run the saved encounter with manual DM controls.");
        let mut settings_saved = false;
        let can_settings = !self.draft.as_ref().is_some_and(Draft::dirty);
        if let Some(snapshot) = &mut self.snapshot {
            ui.collapsing("Campaign run settings", |ui| {
                ui.add_enabled_ui(can_settings, |ui| {
                    ui.label("Run folder · relative to campaign, or an absolute path");
                    ui.text_edit_singleline(&mut snapshot.options.folder);
                    egui::ComboBox::from_id_salt("campaign_initiative").selected_text(snapshot.options.initiative.grouping.label()).show_ui(ui, |ui| {
                        use crate::encounter_settings::Grouping;
                        for mode in [Grouping::Identical, Grouping::Individual, Grouping::BySide, Grouping::Explicit] {
                            ui.selectable_value(&mut snapshot.options.initiative.grouping, mode, mode.label());
                        }
                    });
                    ui.checkbox(&mut snapshot.options.initiative.roster_ties, "Break initiative ties in roster order");
                    ui.weak("Side and explicit groups use the first NPC's bonus and roll mode. Players keep their entered initiative.");
                    ui.horizontal(|ui| {
                        if ui.button("Save run settings").clicked() {
                            match snapshot.options.save(&active, &snapshot.campaign_text) {
                                Ok(()) => settings_saved = true, Err(e) => self.error = Some(e),
                            }
                        }
                        if ui.button("Discard settings changes").clicked() { snapshot.options = snapshot.saved_options.clone(); }
                    });
                });
            });
        }
        if settings_saved {
            self.saved = true;
            self.reload(ui.ctx(), active.clone(), campaigns.modules().to_vec());
        }
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    !self.dirty() && self.snapshot.is_some(),
                    egui::Button::new("New encounter"),
                )
                .clicked()
            {
                self.draft = Some(Draft::new());
                self.error = None;
                self.message = None;
            }
            if ui
                .add_enabled(
                    !self.dirty() && self.pending.is_none(),
                    egui::Button::new("Reload"),
                )
                .clicked()
            {
                self.reload(ui.ctx(), active.clone(), campaigns.modules().to_vec());
            }
            if ui
                .add_enabled(self.dirty(), egui::Button::new("Discard changes"))
                .clicked()
                && let Some(d) = &self.draft
            {
                match &d.baseline {
                    Some(_) => match Draft::open(&active.path, &d.path) {
                        Ok(d) => self.draft = Some(d),
                        Err(e) => {
                            self.error = Some(e);
                            self.draft = None;
                        }
                    },
                    None => self.draft = None,
                }
            }
            if ui
                .add_enabled(self.dirty(), egui::Button::new("Save encounter"))
                .clicked()
            {
                match self
                    .draft
                    .as_mut()
                    .unwrap()
                    .save(&active, campaigns.modules())
                {
                    Ok(()) => {
                        self.saved = true;
                        self.error = None;
                        self.message = Some("Encounter saved.".into());
                        let d = self.draft.as_ref().unwrap();
                        if let Some(s) = &mut self.snapshot {
                            s.encounters.retain(|(p, _)| p != &d.path);
                            s.encounters.push((d.path.clone(), d.name.clone()));
                        }
                    }
                    Err(e) => self.error = Some(e),
                }
            }
        });
        let can_run = self.pending.is_none() && self.draft.as_ref().is_some_and(|d| !d.dirty());
        ui.horizontal(|ui| {
            if ui
                .add_enabled(can_run, egui::Button::new("Run encounter as DM"))
                .clicked()
            {
                match self.start_run(&active, campaigns.modules()) {
                    Ok(run) => {
                        self.run = Some(run);
                        self.error = None;
                    }
                    Err(error) => self.error = Some(error),
                }
            }
            if self.draft.as_ref().is_some_and(Draft::dirty) {
                ui.weak("Save the encounter before starting a run.");
            }
        });
        if self.run.is_some() {
            ui.ctx().request_repaint();
            return;
        }
        if let Some(e) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, e);
        }
        if let Some(m) = &self.message {
            ui.label(m);
        }
        if self.pending.is_some() {
            ui.spinner();
            return;
        }
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        if !snapshot.issues.is_empty() {
            ui.collapsing("Content issues", |ui| {
                for issue in &snapshot.issues {
                    ui.label(issue);
                }
            });
        }
        let dirty = self.dirty();
        ui.collapsing(format!("Saved runs ({})", snapshot.runs.len()), |ui| {
            for (path, name) in &snapshot.runs {
                ui.horizontal_wrapped(|ui| {
                    if ui
                        .add_enabled(!dirty, egui::Button::new("Resume"))
                        .clicked()
                    {
                        match crate::encounter_run::Run::open(path, active.id) {
                            Ok(run) => self.run = Some(run),
                            Err(e) => self.error = Some(e),
                        }
                    }
                    ui.label(name).on_hover_text(path.display().to_string());
                });
            }
            if snapshot.runs.is_empty() {
                ui.weak("No saved runs yet. Start an encounter to create one.");
            }
        });
        if self.run.is_some() {
            ui.ctx().request_repaint();
            return;
        }
        egui::ComboBox::from_id_salt("encounter_selection")
            .selected_text(
                self.draft
                    .as_ref()
                    .map_or("Select encounter", |d| d.name.as_str()),
            )
            .show_ui(ui, |ui| {
                for (path, name) in &snapshot.encounters {
                    if ui.add_enabled(!dirty, egui::Button::new(name)).clicked() {
                        match Draft::open(&active.path, path) {
                            Ok(d) => {
                                self.draft = Some(d);
                                self.error = None;
                                self.message = None;
                            }
                            Err(e) => self.error = Some(e),
                        }
                    }
                }
            });
        let Some(draft) = &mut self.draft else {
            ui.label("Select an encounter or create one.");
            return;
        };
        egui::ScrollArea::vertical().id_salt("encounter_editor").show(ui, |ui| {
            ui.horizontal(|ui| { ui.label("Name"); ui.text_edit_singleline(&mut draft.name); });
            ui.weak("Each copy has its own starting state. Leave HP overrides off to use the creature's starting state.");
            let sides: Vec<_> = draft.setup.sides.keys().filter_map(Value::as_str).map(str::to_owned).collect();
            let mut remove = None; let mut duplicate = None; let mut reorder = None;
            for (i, slot) in draft.setup.roster.iter_mut().enumerate() {
                ui.push_id(&slot.key, |ui| { ui.group(|ui| {
                    let name = slot.creature.as_ref().and_then(|r| snapshot.catalogue.resolve_ref(r).ok()).and_then(|id| snapshot.catalogue.entries.get(&id)).map(|e| e.entry.name.clone())
                        .unwrap_or_else(|| format!("Unavailable creature / party slot: {}", slot.party_slot.as_deref().unwrap_or(&slot.key)));
                    ui.horizontal_wrapped(|ui| {
                        ui.strong(format!("{}. {name}", i + 1));
                        egui::ComboBox::from_id_salt("side").selected_text(&slot.side).show_ui(ui, |ui| { for side in &sides { ui.selectable_value(&mut slot.side, side.clone(), side); } });
                        if ui.button("Duplicate").clicked() { duplicate = Some(i); }
                        if ui.button("Remove").clicked() { remove = Some(i); }
                        if ui.add_enabled(i > 0, egui::Button::new("Move up")).clicked() { reorder = Some(i); }
                    });
                    ui.horizontal(|ui| {
                        let mut group = slot.extra.get("initiative_group").and_then(Value::as_str).unwrap_or("").to_owned();
                        ui.label("Initiative group (optional)");
                        if ui.text_edit_singleline(&mut group).changed() {
                            if group.trim().is_empty() { slot.extra.remove("initiative_group"); } else { slot.extra.insert("initiative_group".into(), group.into()); }
                        }
                    });
                    ui.horizontal_wrapped(|ui| { for (field, label) in [("hp", "Starting HP"), ("temporary_hp", "Temporary HP")] {
                        let mut enabled = slot.initial.contains_key(field);
                        if ui.checkbox(&mut enabled, label).changed() { if enabled { slot.initial.insert(field.into(), 0.into()); } else { slot.initial.remove(field); } }
                        if enabled { let mut value = slot.initial.get(field).and_then(Value::as_u64).unwrap_or(0);
                            if ui.add(egui::DragValue::new(&mut value).range(0..=i64::MAX as u64)).changed() { slot.initial.insert(field.into(), value.into()); }
                        }
                    } });
                }); });
            }
            if let Some(i) = remove { draft.setup.roster.remove(i); }
            else if let Some(i) = duplicate { let mut slot = draft.setup.roster[i].clone(); slot.key = format!("actor_{}", Uuid::new_v4().simple()); draft.setup.roster.insert(i + 1, slot); }
            else if let Some(i) = reorder { draft.setup.roster.swap(i, i - 1); }
            ui.separator(); ui.heading("Add participants");
            ui.label("Party members use the saved local character copies. Refresh Party to update them.");
            if ui.add_enabled(snapshot.creatures.iter().any(|c| c.party), egui::Button::new("Add all cached party members")).clicked() { for c in &snapshot.creatures { if c.party { draft.add(c.id, true); } } }
            ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Find a creature or party member…"));
            let query = self.search.to_lowercase();
            for c in snapshot.creatures.iter().filter(|c| c.name.to_lowercase().contains(&query)) {
                ui.push_id(c.id, |ui| { ui.horizontal(|ui| {
                    if ui.button("Add").clicked() { draft.add(c.id, c.party); }
                    ui.label(&c.name).on_hover_text(c.id.to_string()); if c.party { ui.weak("Party copy"); }
                }); });
            }
        });
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
