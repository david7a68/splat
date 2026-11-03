//! Campaign encounter templates. Runtime state is deliberately separate.
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue, EntryRef, patch_fields},
    design::{self, Action, Density, Status, Tone, Type, colors, icons},
    library::{note_path, write_note},
    party::Party,
    persistence::{Campaign, Module},
};
use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_yaml_ng::{Mapping, Value};
use splat::content::{
    Entry,
    overrides::{Overrides, Resolved},
};
use std::{collections::BTreeSet, fs, sync::Arc, sync::mpsc};
use uuid::Uuid;

mod groups;
mod history;
use groups::GroupDraft;
use history::Template;

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
    /// Changes this slot builds in to its creature. Kept as written until a check reads them,
    /// so an error can name the slot.
    #[serde(default, skip_serializing_if = "Mapping::is_empty")]
    overrides: Mapping,
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
    /// The party's sheets decide its starting values, and the run stays matched with them. An
    /// encounter says so only when it turns this off, so older notes do not change.
    #[serde(default = "sync_by_default", skip_serializing_if = "is_true")]
    combat_sync: bool,
    /// It stays in Ready to run after a run ends, for a fight that can happen again.
    #[serde(default, skip_serializing_if = "is_false")]
    repeatable: bool,
    #[serde(flatten)]
    extra: Mapping,
}
fn sync_by_default() -> bool {
    true
}
fn is_true(value: &bool) -> bool {
    *value
}
fn is_false(value: &bool) -> bool {
    !*value
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
    id: Uuid,
    path: String,
    baseline: Option<String>,
    source: String,
    name: String,
    setup: Setup,
    /// What the DM reads to the table, and their notes: the note's own text.
    body: String,
    saved_name: String,
    saved_setup: Setup,
    saved_body: String,
    /// The groups that belong to this encounter. They are built here and saved with it.
    groups: Vec<GroupDraft>,
    deleted_groups: Vec<GroupDraft>,
    /// Whether those groups have been read. A new encounter has none to read.
    groups_read: bool,
    /// The group that takes what the search adds, by its path. With none, the roster does.
    add_to: Option<String>,
    /// A group just made, whose name takes the keyboard.
    focus_group: Option<String>,
    /// A creature just went to the group the DM chose, so that group comes into view.
    reveal_group: bool,
    /// The last save wrote a group, even if it then stopped, so the campaign needs a new read.
    groups_written: bool,
    /// One group detached from this draft, until the next save.
    removed_group: Option<(usize, GroupDraft, bool)>,
}
impl Draft {
    fn new() -> Self {
        let id = Uuid::new_v4();
        let name = "New encounter".to_owned();
        let source = format!(
            "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: encounter\nsplat_name: New encounter\nsplat_encounter: {{}}\n---\n\n"
        );
        Self {
            id,
            path: format!("Encounters/{id}.md"),
            baseline: None,
            source,
            name: name.clone(),
            saved_name: name,
            setup: Setup::default(),
            body: String::new(),
            saved_setup: Setup::default(),
            saved_body: String::new(),
            groups: Vec::new(),
            deleted_groups: Vec::new(),
            groups_read: true,
            add_to: None,
            focus_group: None,
            reveal_group: false,
            groups_written: false,
            removed_group: None,
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
            id: entry.id,
            path: path.into(),
            baseline: Some(text.clone()),
            source: text,
            saved_name: entry.name.clone(),
            name: entry.name,
            saved_setup: setup.clone(),
            setup,
            body: entry.body.trim().to_owned(),
            saved_body: entry.body.trim().to_owned(),
            groups: Vec::new(),
            deleted_groups: Vec::new(),
            groups_read: false,
            add_to: None,
            focus_group: None,
            reveal_group: false,
            groups_written: false,
            removed_group: None,
        })
    }
    fn dirty(&self) -> bool {
        self.baseline.is_none()
            || self.name != self.saved_name
            || self.setup != self.saved_setup
            || self.body != self.saved_body
            || self.groups.iter().any(GroupDraft::dirty)
            || !self.deleted_groups.is_empty()
    }
    /// Read this encounter's own groups, once the campaign has been read.
    fn read_groups(&mut self, root: &std::path::Path, snapshot: &Snapshot) -> Result<(), String> {
        self.groups_read = true;
        for group in snapshot
            .groups
            .iter()
            .filter(|group| group.encounter == Some(self.id))
        {
            self.groups.push(GroupDraft::open(root, &group.path)?);
        }
        Ok(())
    }
    /// The group that takes what the search adds, if the DM chose one.
    fn target_group(&mut self) -> Option<&mut GroupDraft> {
        let path = self.add_to.as_deref()?;
        self.groups.iter_mut().find(|group| group.path() == path)
    }
    /// Add what the search found, to the group the DM chose or to the roster.
    fn add_chosen(&mut self, id: Uuid, party: bool) {
        match self.target_group() {
            Some(group) => {
                group.add(id);
                self.reveal_group = true;
            }
            None => self.add(id, party),
        }
    }
    fn add_selected(&mut self, id: Uuid, party: bool, snapshot: &Snapshot) {
        let npc = snapshot.npc_stat_block(id);
        match npc {
            Some((stat, name)) => {
                if let Some(group) = self.target_group() {
                    group.add_named(stat, name);
                    self.reveal_group = true;
                } else {
                    self.add(stat, false);
                    self.setup
                        .roster
                        .last_mut()
                        .expect("added slot")
                        .overrides
                        .insert("name".into(), name.into());
                }
            }
            None => self.add_chosen(id, party),
        }
    }
    fn text(&self) -> Result<String, String> {
        let text = patch_fields(
            &self.source,
            &[
                ("splat_name", Some(self.name.clone().into())),
                (
                    "splat_encounter",
                    Some(serde_yaml_ng::to_value(&self.setup).map_err(|e| e.to_string())?),
                ),
            ],
        )?;
        // Words the DM did not touch keep their exact form.
        Ok(if self.body == self.saved_body {
            text
        } else {
            with_body(&text, &self.body)
        })
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
            overrides: Mapping::new(),
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
        // Nothing is written unless the encounter and every changed group can be.
        let mut group_texts = Vec::new();
        for (index, group) in self.groups.iter().enumerate() {
            if group.dirty() {
                let text = group
                    .checked(&snapshot, campaign, Some(self.id))
                    .map_err(|e| match group.name().trim() {
                        "" => e,
                        name => format!("Group {name}: {e}"),
                    })?;
                group_texts.push((index, text));
            }
        }
        for group in &self.deleted_groups {
            group.check_delete(campaign)?;
        }
        // Each was checked against what is saved, and these are saved together.
        let mut names = BTreeSet::from([self.name.trim().to_ascii_lowercase()]);
        for name in self.groups.iter().filter_map(GroupDraft::linkable_name) {
            if !names.insert(name.to_ascii_lowercase()) {
                return Err(format!(
                    "Group {name}: a group for any encounter needs a name of its own. Choose another name."
                ));
            }
        }
        // Create the manifest first so a new campaign keeps its module selections.
        let mut document = CampaignDocument::load(campaign)?;
        if document.baseline.is_none() {
            document.save()?;
        }
        let path = note_path(&campaign.path, &self.path)?;
        fs::create_dir_all(path.parent().ok_or("Missing encounter folder")?)
            .map_err(|e| e.to_string())?;
        // A change to a group alone leaves the encounter's note as it is.
        if self.baseline.as_deref() != Some(text.as_str()) {
            write_note(&campaign.path, &self.path, self.baseline.as_deref(), &text)?;
        }
        self.source = text.clone();
        self.baseline = Some(text);
        self.saved_name = self.name.clone();
        self.saved_setup = self.setup.clone();
        self.saved_body = self.body.clone();
        for (index, text) in group_texts {
            let group = &mut self.groups[index];
            group.write(campaign, &text).map_err(|e| {
                format!(
                    "The encounter saved, but group {} did not: {e}",
                    group.name()
                )
            })?;
            self.groups_written = true;
        }
        for group in &self.deleted_groups {
            group.delete_saved(campaign)?;
            self.groups_written = true;
        }
        self.deleted_groups.clear();
        // A group the DM gave to another encounter is no longer this one's.
        let id = self.id;
        self.groups.retain(|group| group.belongs_to(id));
        self.removed_group = None;
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
                    self.validate_overrides(slot, &snapshot.catalogue.entries[&id].entry)?;
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
    /// A change to a stat must leave it valid for this slot's creature and starting state.
    fn validate_overrides(&self, slot: &Slot, entry: &Entry) -> Result<(), String> {
        let named = |error: String| format!("{} ({}): {error}", slot.key, entry.name);
        let overrides = Overrides::parse(&slot.overrides).map_err(&named)?;
        let resolved = overrides.resolve_for(entry, &slot.side).map_err(&named)?;
        if let Some(side) = &overrides.side
            && !self.setup.sides.contains_key(side.trim())
        {
            return Err(named(format!(
                "side {side} is not a side of this encounter."
            )));
        }
        if overrides.max_hp.is_some()
            && let (Some(hp), Some(max)) = (
                slot.initial.get("hp").and_then(Value::as_i64),
                resolved.max_hp,
            )
            && hp > max
        {
            return Err(named(format!(
                "starting HP {hp} is above the changed maximum of {max}."
            )));
        }
        Ok(())
    }
}
/// A note's text with another body after its frontmatter.
fn with_body(text: &str, body: &str) -> String {
    let front = text.strip_prefix('\u{feff}').unwrap_or(text);
    let skipped = text.len() - front.len();
    let end = front.strip_prefix("---").and_then(|rest| {
        rest.match_indices("\n---")
            .find(|(at, _)| {
                rest[at + 4..]
                    .chars()
                    .next()
                    .is_none_or(|c| c == '\n' || c == '\r')
            })
            .map(|(at, _)| skipped + 3 + at + 4)
    });
    let Some(end) = end else {
        return text.to_owned();
    };
    format!("{}\n\n{}\n", &text[..end], body.trim_end())
}

/// A row of a mapped table: `` | ` key ` | ` value ` | ``. Returns false if it has no such row.
fn set_table_value(text: &mut String, key: &str, value: &str) -> bool {
    let start = format!("| ` {key} ` | ` ");
    let Some(at) = text.find(&start) else {
        return false;
    };
    let from = at + start.len();
    let Some(length) = text[from..].find(" `") else {
        return false;
    };
    text.replace_range(from..from + length, value);
    true
}

/// What the DM asked of the creature open beside the roster.
enum CreatureStep {
    Save,
    Close,
}

/// What the DM can change on a creature, as they typed it.
#[derive(Clone, PartialEq)]
struct CreatureFields {
    name: String,
    hp: String,
    armor_class: String,
    body: String,
}

/// A creature the DM made their own, open beside the roster.
struct CreatureEditor {
    path: String,
    baseline: String,
    fields: CreatureFields,
    saved: CreatureFields,
    error: Option<String>,
    focus: bool,
}
impl CreatureEditor {
    fn open(root: &std::path::Path, path: &str) -> Result<Self, String> {
        let text = fs::read_to_string(note_path(root, path)?).map_err(|e| e.to_string())?;
        let entry = Entry::parse(path, &text)?;
        let stat = |name: &str| {
            entry
                .fields
                .get("splat_stats")
                .and_then(|stats| stats.get(name))
                .and_then(|stat| stat.get("value"))
                .and_then(Value::as_i64)
                .map_or_else(String::new, |n| n.to_string())
        };
        let fields = CreatureFields {
            hp: stat("max_hp"),
            armor_class: stat("armor_class"),
            body: entry.body.trim().to_owned(),
            name: entry.name,
        };
        Ok(Self {
            path: path.to_owned(),
            baseline: text,
            saved: fields.clone(),
            fields,
            error: None,
            focus: true,
        })
    }
    fn dirty(&self) -> bool {
        self.saved != self.fields
    }
    fn text(&self) -> Result<String, String> {
        let CreatureFields {
            name,
            hp,
            armor_class,
            body,
        } = &self.fields;
        if name.trim().is_empty() {
            return Err("Enter a name.".into());
        }
        let number = |text: &str, word: &str| {
            text.trim()
                .parse::<u32>()
                .map_err(|_| format!("{word} must be a whole number."))
        };
        let new_hp = number(hp, "Hit points")?;
        let new_armor_class = number(armor_class, "Armor class")?;
        let mut body = body.clone();
        // A number the DM left alone stays as it is, and so does any edit to the text. The stats
        // of an imported creature are rows in its text. Other notes hold them above.
        let mut above = Vec::new();
        if *hp != self.saved.hp {
            if !set_table_value(&mut body, "max_hp", &new_hp.to_string()) {
                above.push(("max_hp", new_hp));
            }
            // Its dice no longer give this number, so they go with the old one.
            set_table_value(&mut body, "hit_points_text", &new_hp.to_string());
        }
        if *armor_class != self.saved.armor_class
            && !set_table_value(&mut body, "armor_class", &new_armor_class.to_string())
        {
            above.push(("armor_class", new_armor_class));
        }
        let mut changes = vec![("splat_name", Some(name.trim().into()))];
        if !above.is_empty() {
            let entry = Entry::parse(&self.path, &self.baseline)?;
            let mut stats = entry
                .fields
                .get("splat_stats")
                .and_then(Value::as_mapping)
                .cloned()
                .unwrap_or_default();
            for (key, value) in above {
                let mut stat = stats
                    .get(key)
                    .and_then(Value::as_mapping)
                    .cloned()
                    .unwrap_or_default();
                stat.insert("value".into(), value.into());
                stats.insert(key.into(), Value::Mapping(stat));
            }
            changes.push(("splat_stats", Some(Value::Mapping(stats))));
        }
        let text = patch_fields(&self.baseline, &changes)?;
        Ok(with_body(&text, &body))
    }
    fn save(&mut self, root: &std::path::Path) -> Result<(), String> {
        let text = self.text()?;
        Entry::parse(&self.path, &text)?;
        write_note(root, &self.path, Some(&self.baseline), &text)?;
        self.baseline = text;
        self.saved = self.fields.clone();
        Ok(())
    }
}
struct Creature {
    id: Uuid,
    name: String,
    lower: String,
    /// CR, HP and AC, from fields the entry already has.
    detail: String,
    /// The package it came from. A party copy says so in `detail` instead.
    source: Option<String>,
    party: bool,
    /// A character whose sheet is on D&D Beyond. A sheet cannot be two participants.
    linked: bool,
    /// The D&D Beyond character a party copy is of.
    character: Option<String>,
}
const ROSTER_MAX_WIDTH: f32 = 860.0;
const ADD_PANE: &str = "encounter_add";
const ADD_PANE_MAX_WIDTH: f32 = 640.0;
const RESULT_ROW: f32 = 40.0;
/// A result's stats, each in its own place, so that they can be compared down the list. The
/// first place holds a party copy's word, which has no challenge rating. The package follows
/// if the row has room for it.
fn detail_columns(ui: &mut egui::Ui, detail: &str, source: Option<&str>) {
    if detail.contains("HP not set") {
        ui.horizontal_wrapped(|ui| {
            for part in detail.split(" · ") {
                if part.contains("HP not set") {
                    design::status(
                        ui,
                        design::Status {
                            icon: design::icons::WARN,
                            word: "HP not set",
                            color: colors().result.warning,
                        },
                    );
                } else {
                    ui.label(Type::Small.rich(ui, part).color(colors().muted));
                }
            }
        });
        return;
    }
    const COLUMNS: [(&str, f32); 3] = [("CR ", 66.0), ("HP ", 46.0), ("AC ", 38.0)];
    let parts: Vec<&str> = detail.split(" · ").collect();
    let other = parts
        .iter()
        .find(|part| !COLUMNS.iter().any(|(prefix, _)| part.starts_with(prefix)));
    // A row of plain words: it sits close under its name, not at a control's height.
    ui.spacing_mut().interact_size.y = 0.0;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for (prefix, width) in COLUMNS {
            let part = parts
                .iter()
                .find(|part| part.starts_with(prefix))
                .or(other.filter(|_| prefix == COLUMNS[0].0));
            ui.allocate_ui_with_layout(
                egui::vec2(width, Type::Small.line()),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.set_min_width(width);
                    if let Some(part) = part {
                        ui.label(Type::Small.rich(ui, *part).color(colors().muted));
                    }
                },
            );
        }
        if detail.starts_with("NPC · as ") {
            ui.add_space(design::S);
            ui.label(
                Type::Small
                    .rich(ui, parts[..2].join(" · "))
                    .color(colors().muted),
            );
        }
        // Only whole: a cut name says nothing, and Read always shows it.
        if let Some(source) = source
            && design::number_width(ui, source, &Type::Small.font(ui)) <= ui.available_width()
        {
            ui.label(Type::Small.rich(ui, source).color(colors().muted));
        }
    });
}

fn grouped(value: u64) -> String {
    design::count(value as usize, "", "").trim().to_owned()
}

pub(crate) fn creature_name_link(ui: &mut egui::Ui, name: &str) -> egui::Response {
    let galley = design::one_line(
        ui,
        name,
        Type::Body.font(ui),
        colors().text,
        ui.available_width(),
    );
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(galley.size().x, Density::Read.control() - design::XS),
        egui::Sense::click(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Link, ui.is_enabled(), name));
    if ui.is_rect_visible(rect) {
        let y = rect.center().y - galley.size().y / 2.0;
        if response.hovered() {
            ui.painter().hline(
                rect.x_range(),
                y + galley.size().y - design::HAIRLINE_WIDTH,
                egui::Stroke::new(design::HAIRLINE_WIDTH, colors().text),
            );
        }
        ui.painter()
            .galley(egui::pos2(rect.left(), y), galley, colors().text);
        if response.has_focus() {
            design::focus_ring(ui, rect);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// `changed` holds this creature's values when a slot or member changes them.
fn creature_detail(entry: &Entry, party: bool, changed: Option<&Resolved>) -> String {
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
        .and_then(|text| text.split_whitespace().next())
        // A creature with no field has it on the line the stat block shows.
        .or_else(|| {
            entry
                .body
                .lines()
                .find_map(|line| line.trim().strip_prefix("CR "))
                .and_then(|text| text.split_whitespace().next())
        });
    let mut parts = Vec::new();
    if party {
        parts.push("Party copy".to_owned());
    }
    if let Some(cr) = cr {
        parts.push(format!("CR {cr}"));
    }
    if let Some(hp) = changed.map_or_else(|| stat("max_hp"), |changed| changed.max_hp) {
        parts.push(format!("HP {hp}"));
    } else {
        parts.push(format!("{} HP not set", design::icons::WARN));
    }
    if let Some(ac) = changed.map_or_else(|| stat("armor_class"), |changed| changed.armor_class) {
        parts.push(format!("AC {ac}"));
    }
    parts.join(" · ")
}
/// The sheet of the party member in this roster slot, if the slot holds one.
fn slot_sheet<'a>(
    slot: &Slot,
    snapshot: &Snapshot,
    party: &'a Party,
) -> Option<&'a splat::character::EntitySnapshot> {
    let id = snapshot
        .catalogue
        .resolve_ref(slot.creature.as_ref()?)
        .ok()?;
    let creature = snapshot.creatures.iter().find(|c| c.id == id)?;
    party.sheet(creature.character.as_deref()?)
}
fn roster_summary(setup: &Setup, catalogue: &Catalogue) -> String {
    let mut party = 0;
    let mut creatures = std::collections::BTreeMap::<String, usize>::new();
    for slot in &setup.roster {
        let side = slot
            .overrides
            .get("side")
            .and_then(Value::as_str)
            .unwrap_or(&slot.side);
        if side.trim() == "party" {
            party += 1;
        } else {
            let name = slot
                .creature
                .as_ref()
                .and_then(|reference| catalogue.resolve_ref(reference).ok())
                .and_then(|id| catalogue.entries.get(&id))
                .map_or_else(
                    || "Unknown creature".to_owned(),
                    |entry| entry.entry.name.clone(),
                );
            *creatures
                .entry(groups::shown_name(&slot.overrides, name))
                .or_default() += 1;
        }
    }
    let mut parts = Vec::new();
    if party > 0 {
        parts.push(design::count(party, "party", "party"));
    }
    parts.extend(creatures.into_iter().map(|(name, count)| {
        if count == 1 {
            name
        } else {
            format!("{} × {name}", design::count(count, "", "").trim())
        }
    }));
    if setup.roster.iter().any(|slot| !slot.overrides.is_empty()) {
        parts.push("Changed".to_owned());
    }
    parts.join(" · ")
}
/// Keep a stat label with its value. A second line can break between facts.
fn stat_line(text: &str) -> String {
    text.split(" · ")
        .map(|part| part.replace(' ', "\u{a0}"))
        .collect::<Vec<_>>()
        .join(" · ")
}

struct SearchRoster<'a> {
    already: &'a dyn Fn(&Creature) -> bool,
}

struct Snapshot {
    catalogue: Catalogue,
    creatures: Vec<Creature>,
    templates: Vec<Template>,
    issues: Vec<String>,
    options: crate::encounter_settings::Options,
    saved_options: crate::encounter_settings::Options,
    campaign_text: String,
    runs: Vec<crate::encounter_run::SavedRunInfo>,
    groups: Arc<Vec<Arc<crate::encounter_run::RunGroup>>>,
    run_creatures: Arc<Vec<Arc<crate::encounter_run::RunCreature>>>,
}
impl Snapshot {
    /// A refresh keeps settings the DM is editing. Otherwise the file decides, or the next
    /// save would write an old value over a change made outside Splat.
    fn keep_edited_settings(&mut self, old: &Self) {
        if old.options != old.saved_options {
            self.options = old.options.clone();
            self.saved_options = old.saved_options.clone();
        }
    }
    fn npc_stat_block(&self, id: Uuid) -> Option<(Uuid, String)> {
        let npc = &self.catalogue.entries.get(&id)?.entry;
        if npc.kind != "world"
            || npc
                .fields
                .get("splat_world")
                .and_then(|world| world.get("type"))
                .and_then(Value::as_str)
                != Some("npc")
        {
            return None;
        }
        let value = npc.fields.get("splat_world")?.get("creature")?;
        let reference: EntryRef = serde_yaml_ng::from_value(value.clone()).ok()?;
        let stat = self.catalogue.resolve_ref(&reference).ok()?;
        self.creatures
            .iter()
            .any(|c| c.id == stat)
            .then(|| (stat, npc.name.clone()))
    }
    fn npc_name_for(&self, entry: &Entry, overrides: &Mapping) -> Option<String> {
        let name = overrides.get("name")?.as_str()?;
        self.catalogue.entries.values().find_map(|candidate| {
            let (stat, npc_name) = self.npc_stat_block(candidate.entry.id)?;
            (stat == entry.id && name == npc_name).then_some(npc_name)
        })
    }
    fn load(campaign: &Campaign, modules: &[Module]) -> Result<Self, String> {
        let document = CampaignDocument::load(campaign)?;
        let catalogue = Catalogue::load(modules, Some(campaign))?;
        let closure = catalogue.closure(&document.selections, campaign.id);
        let main_source = crate::content_workspace::builtin_manifest().id;
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
                    detail: creature_detail(&e.entry, party, None),
                    source: match (party, e.private) {
                        (true, _) => None,
                        (false, true) => Some("Campaign".to_owned()),
                        (false, false) if e.owner == main_source => None,
                        (false, false) => Some(catalogue.package_name(e.owner)),
                    },
                    party,
                    linked: e.entry.fields.contains_key("splat_external"),
                    character: e
                        .entry
                        .fields
                        .get("splat_external")
                        .and_then(|external| external.get("character_id"))
                        .and_then(Value::as_str)
                        .filter(|_| party)
                        .map(str::to_owned),
                }
            })
            .collect();
        let npcs: Vec<_> = catalogue
            .entries
            .values()
            .filter_map(|entry| {
                let npc = &entry.entry;
                if (npc.kind != "world"
                    || npc
                        .fields
                        .get("splat_world")
                        .and_then(|world| world.get("type"))
                        .and_then(Value::as_str)
                        != Some("npc"))
                    || !(entry.private && entry.owner == campaign.id
                        || closure.direct.contains(&npc.id)
                        || closure.required.contains_key(&npc.id))
                {
                    return None;
                }
                let value = npc.fields.get("splat_world")?.get("creature")?;
                let reference: EntryRef = serde_yaml_ng::from_value(value.clone()).ok()?;
                let id = catalogue.resolve_ref(&reference).ok()?;
                let stat = creatures.iter().find(|c| c.id == id && !c.linked)?;
                Some(Creature {
                    id: npc.id,
                    name: npc.name.clone(),
                    lower: npc.name.to_lowercase(),
                    detail: format!("NPC · as {} · {}", stat.name, stat.detail),
                    source: None,
                    party: false,
                    linked: false,
                    character: None,
                })
            })
            .collect();
        creatures.extend(npcs);
        creatures.sort_by_key(|c| (!c.party, c.name.to_lowercase(), c.id));
        let mut templates: Vec<_> = catalogue
            .entries
            .values()
            .filter(|e| e.private && e.owner == campaign.id && e.entry.kind == "encounter")
            .map(|e| {
                let setup = e
                    .entry
                    .fields
                    .get("splat_encounter")
                    .and_then(|setup| serde_yaml_ng::from_value::<Setup>(setup.clone()).ok());
                Template {
                    path: e.entry.path.clone(),
                    id: e.entry.id,
                    name: e.entry.name.clone(),
                    summary: setup
                        .as_ref()
                        .map(|setup| roster_summary(setup, &catalogue)),
                    repeatable: setup.is_some_and(|setup| setup.repeatable),
                }
            })
            .collect();
        templates.sort_by_key(|template| template.name.to_lowercase());
        let issues = catalogue
            .errors
            .iter()
            .chain(&closure.errors)
            .cloned()
            .collect();
        let options = crate::encounter_settings::Options::load(campaign, &catalogue)?;
        let runs = crate::encounter_run::Run::list(&options.directory(campaign)?, campaign.id)?;
        let groups = Arc::new(groups::load(&catalogue, &creatures, campaign.id));
        let run_creatures = Arc::new(
            creatures
                .iter()
                .filter(|creature| !creature.linked)
                .filter_map(|creature| {
                    Some(Arc::new(crate::encounter_run::RunCreature {
                        entry: Arc::new(catalogue.entries.get(&creature.id)?.entry.clone()),
                        lower: creature.lower.clone(),
                        detail: crate::library::presentation::list_subtitle(
                            &creature.detail,
                            creature.source.as_deref(),
                        ),
                    }))
                })
                .collect(),
        );
        Ok(Self {
            groups,
            run_creatures,
            options: options.clone(),
            saved_options: options,
            campaign_text: document.text,
            runs,
            catalogue,
            creatures,
            templates,
            issues,
        })
    }
    /// The encounters a group can belong to, in the list's order.
    fn encounter_choices(&self) -> impl Iterator<Item = (Uuid, &str)> {
        self.templates
            .iter()
            .map(|template| (template.id, template.name.as_str()))
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
    /// The draft `Discard changes` last dropped, for one step of undo.
    discarded: Option<Draft>,
    discarded_options: Option<crate::encounter_settings::Options>,
    pub saved: bool,
    run: Option<crate::encounter_run::Run>,
    /// History is a read-only snapshot and never owns the open-run slot.
    history_read: Option<(std::path::PathBuf, crate::encounter_run::FightSummary)>,
    /// A run just opened, or the DM asked to go back to it from its row in the list, so the
    /// shell goes to the Run page.
    pub go_to_run: bool,
    /// A creature to read beside the draft, in Lookup.
    pub peek: Option<Uuid>,
    focus_name: bool,
    focus_search: bool,
    editor_pane: EditorPane,
    expanded: Option<String>,
    group_draft: Option<GroupDraft>,
    /// The group draft `Discard changes` last dropped, for one step of undo.
    discarded_group: Option<GroupDraft>,
    /// A creature made the campaign's own, with its text open beside the roster.
    creature: Option<CreatureEditor>,
    /// The open encounter came from the Session page, and goes back there.
    pub from_session: bool,
    /// A new encounter joins the session's plan when it is first saved.
    plan_on_save: bool,
    pub saved_for_plan: Option<String>,
    /// A group's old and new name, for the notes that link it.
    pub renamed_group: Option<(String, String)>,
    /// The names linked in tonight's plan, for an open run.
    pub planned_groups: Vec<String>,
    /// The open group draft goes back to the session's plan when it is saved.
    group_in_plan: bool,
    pub wants_session: bool,
    /// The run just opened began on the Session page, by way of this editor.
    pub run_from_session: bool,
    /// Where the party stands after the last fight of the session being played.
    pub carry: Option<crate::encounter_run::Carry>,
    /// The saved run `Delete` last removed, for one step of undo.
    deleted_run: Option<crate::encounter_run::DeletedRun>,
    removed_saved_group: Option<groups::RemovedGroup>,
}
/// One encounter, as a session's plan offers it.
pub struct PlanFight {
    pub saved: Option<crate::encounter_run::SavedRunInfo>,
    pub name: String,
    pub path: String,
    pub summary: String,
}
/// What the page needs from the story: where each run's event is, and which sessions exist.
pub struct Story<'a> {
    pub events: &'a [crate::timeline::Event],
    pub sessions: &'a [crate::timeline::page::SessionLink],
}
/// What the DM asked for from a row of the list. One press in a frame, so one of these.
enum ListAction {
    Resume(std::path::PathBuf),
    ReadHistory(std::path::PathBuf),
    BackToRun,
    Delete(std::path::PathBuf),
    Edit(String),
    Run(String),
    OpenGroup(String),
    RemoveGroup(String),
}
#[derive(Clone, Copy, Default, PartialEq)]
enum EditorPane {
    #[default]
    Roster,
    Add,
}
const START_FIGHT: &str = "Start fight";
const NAME_FIELD: &str = "Encounter name";
const SEARCH_FIELD: &str = "Find a creature or party member";
const SYNC_OPTION: &str = "Sync party with D&D Beyond";
const REPEATABLE_OPTION: &str = "Keep in Ready during a run";
const BLOCKED_BY_RUN: &str = "Close the open run first.";
const BLOCKED_BY_DRAFT: &str = "Save or discard the open encounter first.";
const BLOCKED_BY_CREATURE: &str = "Save or discard the changes to this creature first.";
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

    /// A participant under another name. No fixture has a name this long.
    #[cfg(test)]
    pub fn simulate_rename(&mut self, name: &str, to: &str) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_rename(name, to);
    }

    /// A read of this participant's sheet, holding its last matched damage plus `more_damage`.
    #[cfg(test)]
    pub fn simulate_read(&mut self, name: &str, more_damage: i64) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_read(name, more_damage);
    }

    /// Running syncs react to the session from now on, as real ones do.
    #[cfg(test)]
    pub fn simulate_session_end(&mut self) {
        self.run
            .as_mut()
            .expect("an open run")
            .stop_simulating_sync();
    }

    /// Show an error line. An empty message clears it.
    #[cfg(test)]
    pub fn simulate_run_error(&mut self, message: &str) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_error(message);
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
    /// Identify the open copy when several unfinished runs share an encounter.
    pub fn run_copy_label(&self) -> Option<String> {
        let run = self.run.as_ref()?.live_info()?;
        let snapshot = self.snapshot.as_ref()?;
        let others = snapshot.runs.iter().any(|other| {
            other.id != run.id
                && other.standing == crate::encounter_run::Standing::Unfinished
                && (if run.source.is_some() {
                    other.source == run.source
                } else {
                    other.encounter.split(" · saved ").next()
                        == run.encounter.split(" · saved ").next()
                })
        });
        others.then(|| {
            run.encounter
                .split_once(" · ")
                .map_or(run.encounter.as_str(), |(_, copy)| copy)
                .to_owned()
        })
    }
    pub fn settings_unsaved(&self) -> bool {
        self.snapshot
            .as_ref()
            .is_some_and(|s| s.options != s.saved_options)
    }
    /// The open run's name, if it has changes that are not on disk.
    pub fn run_not_saved(&self) -> Option<String> {
        self.run
            .as_ref()
            .filter(|run| run.needs_save())
            .map(|run| run.strip().name)
    }
    pub fn unsaved(&self) -> bool {
        self.run
            .as_ref()
            .is_some_and(crate::encounter_run::Run::needs_save)
            || self.draft_dirty()
            || self
                .snapshot
                .as_ref()
                .is_some_and(|s| s.options != s.saved_options)
    }
    pub fn dirty(&self) -> bool {
        self.has_run() || self.unsaved()
    }
    pub fn draft_name(&self) -> Option<String> {
        self.draft
            .as_ref()
            .map(|d| d.name.clone())
            .or_else(|| self.group_draft.as_ref().map(GroupDraft::name))
    }
    /// Every encounter the campaign has, for a session's plan.
    pub fn plan_fights(&self) -> Vec<PlanFight> {
        self.snapshot.as_ref().map_or_else(Vec::new, |snapshot| {
            snapshot
                .templates
                .iter()
                .map(|template| PlanFight {
                    saved: snapshot
                        .runs
                        .iter()
                        .find(|run| {
                            run.source == Some(template.id)
                                && run.standing == crate::encounter_run::Standing::Unfinished
                        })
                        .cloned(),
                    name: template.name.clone(),
                    path: template.path.clone(),
                    summary: template.summary.clone().unwrap_or_default(),
                })
                .collect()
        })
    }
    /// The folder that holds this campaign's saved runs.
    pub fn run_directory(&self, active: &Campaign) -> Option<std::path::PathBuf> {
        self.snapshot.as_ref()?.options.directory(active).ok()
    }
    /// Why no fight can start now, in words.
    pub fn plan_blocked(&self) -> Option<&'static str> {
        self.start_blocked().or(self
            .pending
            .is_some()
            .then_some("Still reading this campaign."))
    }
    /// Reopen a saved run, to read it or go on with it.
    pub fn open_saved_run(&mut self, path: &std::path::Path, active: &Campaign) {
        match crate::encounter_run::fight_summary(path, active.id) {
            Ok(summary) if summary.ended => {
                self.history_read = Some((path.to_path_buf(), summary));
                self.error = None;
            }
            Ok(_) if self.has_run() => self.error = Some(BLOCKED_BY_RUN.to_owned()),
            Ok(_) => match crate::encounter_run::Run::open(path, active.id) {
                Ok(run) => {
                    self.run = Some(run);
                    self.go_to_run = true;
                    self.error = None;
                }
                Err(error) => self.error = Some(error),
            },
            Err(error) => self.error = Some(error),
        }
    }
    /// The newest unfinished save is one click away from any page.
    pub fn paused_run(&self) -> Option<&crate::encounter_run::SavedRunInfo> {
        if self.has_run() {
            return None;
        }
        self.snapshot
            .as_ref()?
            .runs
            .iter()
            .find(|run| run.standing == crate::encounter_run::Standing::Unfinished)
    }
    pub fn paused_run_count(&self) -> usize {
        self.snapshot.as_ref().map_or(0, |snapshot| {
            snapshot
                .runs
                .iter()
                .filter(|run| run.standing == crate::encounter_run::Standing::Unfinished)
                .count()
        })
    }
    /// A new encounter from a session's plan. It joins the plan, and back leads there.
    /// Whether a fight is open that has not ended.
    pub fn run_takes_arrivals(&self) -> bool {
        self.run
            .as_ref()
            .is_some_and(crate::encounter_run::Run::takes_arrivals)
    }
    /// The open run's panel for what joins the fight.
    pub fn show_arrivals(&mut self) {
        if let Some(run) = &mut self.run {
            run.show_arrivals();
        }
    }
    /// Assemble a group where the plan is, with the session's note still beside it.
    pub fn new_group_in_plan(&mut self) {
        self.group_draft = Some(GroupDraft::new());
        self.discarded_group = None;
        self.group_in_plan = true;
        self.search.clear();
        self.focus_name = true;
        self.editor_pane = EditorPane::Add;
        self.expanded = None;
        self.error = None;
        self.message = None;
    }
    /// Returns false when the group cannot be read. Encounters says why.
    pub fn open_group_in_plan(&mut self, path: &str, active: &Campaign) -> bool {
        match GroupDraft::open(&active.path, path) {
            Ok(draft) => {
                self.group_draft = Some(draft);
                self.discarded_group = None;
                self.group_in_plan = true;
                self.editor_pane = EditorPane::Roster;
                self.expanded = None;
                self.error = None;
                self.message = None;
                true
            }
            Err(error) => {
                self.error = Some(error);
                false
            }
        }
    }
    pub fn group_open_in_plan(&self) -> bool {
        self.group_in_plan && self.group_draft.is_some()
    }
    /// A new group started in the plan was discarded there, and can come back.
    pub fn group_discarded_in_plan(&self) -> bool {
        // The plan shows it again, so an encounter draft left open cannot hide it there.
        self.group_in_plan && self.group_draft.is_none() && self.discarded_group.is_some()
    }
    pub fn undo_group_discard_in_plan(&mut self) {
        if self.group_discarded_in_plan() {
            self.group_draft = self.discarded_group.take();
            self.message = None;
            self.error = None;
        }
    }
    /// The group editor, as a part of the session's page.
    pub fn group_in_plan_view(
        &mut self,
        ui: &mut egui::Ui,
        active: &Campaign,
        campaigns: &Campaigns,
    ) {
        self.group_editor_view(ui, active, campaigns, true);
    }
    /// The groups a session's plan can offer and link: those for any encounter. A group that
    /// belongs to one encounter comes with that encounter.
    pub fn plan_groups(&self) -> Vec<Arc<crate::encounter_run::RunGroup>> {
        self.snapshot.as_ref().map_or_else(Vec::new, |snapshot| {
            snapshot
                .groups
                .iter()
                .filter(|group| group.encounter.is_none())
                .cloned()
                .collect()
        })
    }
    /// Why a group cannot be assembled from the plan now.
    pub fn plan_group_blocked(&self) -> Option<&'static str> {
        self.draft_dirty().then_some(BLOCKED_BY_DRAFT).or(self
            .snapshot
            .is_none()
            .then_some("Still reading this campaign."))
    }
    pub fn new_from_plan(&mut self) {
        self.start_new_draft();
        self.from_session = true;
        self.plan_on_save = true;
    }
    fn start_new_draft(&mut self) {
        let mut draft = Draft::new();
        // Nearly every fight has the party in it, so nearly every draft starts that way.
        if let Some(snapshot) = &self.snapshot {
            for creature in snapshot.creatures.iter().filter(|c| c.party) {
                draft.add(creature.id, true);
            }
        }
        self.draft = Some(draft);
        self.focus_name = true;
        self.error = None;
        self.message = None;
        self.from_session = false;
        self.plan_on_save = false;
        self.creature = None;
    }
    /// Start a fight from a saved encounter, with no editor left open.
    pub fn run_path(&mut self, path: &str, active: &Campaign, modules: &[Module]) {
        if let Some(reason) = self.start_blocked() {
            self.error = Some(reason.to_owned());
            return;
        }
        self.creature = None;
        match Draft::open(&active.path, path) {
            Ok(d) => {
                self.draft = Some(d);
                self.open_run(active, modules);
                self.draft = None;
            }
            Err(e) => self.error = Some(e),
        }
    }
    /// Add a freshly copied creature to the open draft.
    pub fn add_to_draft(&mut self, id: Uuid) {
        if let Some(draft) = &mut self.draft {
            draft.add_chosen(id, false);
        } else if let Some(draft) = &mut self.group_draft {
            draft.add(id);
        }
    }
    /// What the rail calls the open draft.
    pub fn draft_word(&self) -> &'static str {
        if self.group_draft.is_some() {
            "Group draft"
        } else {
            "Encounter draft"
        }
    }
    /// What a message calls the open draft.
    pub fn draft_kind(&self) -> &'static str {
        if self.group_draft.is_some() {
            "Group"
        } else {
            "Encounter"
        }
    }
    pub fn draft_dirty(&self) -> bool {
        self.encounter_dirty() || self.creature_dirty()
    }
    /// The encounter itself has unsaved changes, apart from any creature open beside it.
    pub fn encounter_dirty(&self) -> bool {
        self.draft.as_ref().is_some_and(Draft::dirty)
            || self.group_draft.as_ref().is_some_and(GroupDraft::dirty)
    }
    pub fn creature_dirty(&self) -> bool {
        self.creature.as_ref().is_some_and(CreatureEditor::dirty)
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
        self.creature = None;
        self.history_read = None;
        // A discarded draft belongs to its campaign. Undo must not save it into another.
        if self.active.as_ref().is_none_or(|old| old.id != campaign.id) {
            self.discarded = None;
            self.discarded_group = None;
            self.discarded_options = None;
        }
        // Its path can be out of date after this, so undo could put it where nothing lists it.
        self.deleted_run = None;
        self.removed_saved_group = None;
        self.active = Some(campaign.clone());
        self.draft = None;
        self.group_draft = None;
        self.group_in_plan = false;
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
        // A run restored at startup comes before the first load. With nothing loaded,
        // there is nothing a load could drop, so it does not wait for the run to close.
        if let Some(active) = campaigns.active_campaign()
            && match &self.active {
                None => true,
                Some(c) => (c.id != active.id || c.path != active.path) && !self.dirty(),
            }
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
                                s.keep_edited_settings(old);
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
        if self.creature.as_ref().is_some_and(CreatureEditor::dirty)
            && ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S))
            && let Some(active) = campaigns.active_campaign()
        {
            self.save_creature(&active, campaigns.modules());
            return;
        }
        if self.draft_dirty()
            && ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S))
            && let Some(active) = campaigns.active_campaign()
        {
            if self.group_draft.is_some() {
                self.save_group_draft(ctx, &active, campaigns.modules());
            } else {
                self.save_draft(ctx, &active, campaigns.modules());
            }
        }
    }
    fn save_draft(&mut self, ctx: &egui::Context, active: &Campaign, modules: &[Module]) {
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        let outcome = draft.save(active, modules);
        let groups_saved = std::mem::take(&mut draft.groups_written);
        // The group being built stays in view, because it still takes what the search adds.
        draft.reveal_group |= groups_saved;
        match outcome {
            Ok(()) => {
                self.saved = true;
                self.error = None;
                self.message = Some("Encounter saved.".into());
                if std::mem::take(&mut self.plan_on_save) {
                    self.saved_for_plan = Some(draft.name.clone());
                }
                if let (Some(s), Ok(entry)) =
                    (&mut self.snapshot, Entry::parse(&draft.path, &draft.source))
                {
                    s.templates.retain(|template| template.path != draft.path);
                    s.templates.push(Template {
                        path: draft.path.clone(),
                        id: entry.id,
                        name: draft.name.clone(),
                        summary: Some(roster_summary(&draft.setup, &s.catalogue)),
                        repeatable: draft.setup.repeatable,
                    });
                    s.templates
                        .sort_by_key(|template| template.name.to_lowercase());
                }
            }
            Err(e) => self.error = Some(e),
        }
        if groups_saved {
            self.refreshing = true;
            self.start_load(ctx, active.clone(), modules.to_vec());
        }
    }
    /// Make the creature on this row the campaign's own: a copy, swapped in, and open beside
    /// the roster. A creature that is already its own only opens.
    fn make_it_yours(&mut self, slot: usize, active: &Campaign, modules: &[Module]) {
        if self.creature_dirty() {
            self.error = Some(BLOCKED_BY_CREATURE.to_owned());
            return;
        }
        let result = (|| -> Result<CreatureEditor, String> {
            let snapshot = self
                .snapshot
                .as_ref()
                .ok_or("Still reading this campaign.")?;
            let draft = self.draft.as_ref().ok_or("No encounter is open.")?;
            let reference = draft.setup.roster[slot]
                .creature
                .as_ref()
                .ok_or("This row has no creature.")?;
            let id = snapshot.catalogue.resolve_ref(reference)?;
            let indexed = &snapshot.catalogue.entries[&id];
            if indexed.private && indexed.owner == active.id {
                return CreatureEditor::open(&active.path, &indexed.entry.path);
            }
            let text = snapshot
                .catalogue
                .text_of(id)
                .ok_or("This creature's text is not available.")?;
            let source = crate::content_workspace::CopySource {
                entry: indexed.entry.clone(),
                text,
                root: None,
                campaign: None,
            };
            let base = format!("{} copy", indexed.entry.name);
            let name = (1..1000)
                .map(|n| {
                    if n == 1 {
                        base.clone()
                    } else {
                        format!("{base} {n}")
                    }
                })
                .find(|name| {
                    note_path(&active.path, &format!("{name}.md")).is_ok_and(|p| !p.exists())
                })
                .ok_or("Too many copies of this creature.")?;
            let plan = crate::content_workspace::CopyPlan::prepare(
                &source,
                &active.path,
                Some(active.id),
                &name,
                "",
                &snapshot.catalogue,
            )?;
            let copy = plan.save()?;
            let path = copy.path.clone();
            let draft = self.draft.as_mut().ok_or("No encounter is open.")?;
            draft.setup.roster[slot].creature = Some(EntryRef::id(copy.id));
            CreatureEditor::open(&active.path, &path)
        })();
        match result {
            Ok(editor) => {
                self.creature = Some(editor);
                self.editor_pane = EditorPane::Add;
                self.reload_snapshot(active, modules);
                self.saved = true;
                self.error = None;
            }
            Err(e) => self.error = Some(e),
        }
    }
    /// Read the campaign again now: a row must not show a new creature as unavailable.
    fn reload_snapshot(&mut self, active: &Campaign, modules: &[Module]) {
        match Snapshot::load(active, modules) {
            // Run settings being edited are the DM's, and this read must not drop them.
            Ok(mut snapshot) => {
                if let Some(old) = self.snapshot.take() {
                    snapshot.options = old.options;
                    snapshot.saved_options = old.saved_options;
                    snapshot.campaign_text = old.campaign_text;
                }
                self.snapshot = Some(snapshot);
            }
            Err(e) => self.error = Some(e),
        }
    }
    /// The DM did what the message asked, so it goes.
    fn clear_creature_block(&mut self) {
        if self.error.as_deref() == Some(BLOCKED_BY_CREATURE) {
            self.error = None;
        }
    }
    fn save_creature(&mut self, active: &Campaign, modules: &[Module]) {
        let Some(editor) = &mut self.creature else {
            return;
        };
        match editor.save(&active.path) {
            Ok(()) => {
                self.creature = None;
                self.clear_creature_block();
                self.reload_snapshot(active, modules);
                self.saved = true;
                self.focus_search = true;
            }
            Err(e) => editor.error = Some(e),
        }
    }
    /// Beside the roster, in place of the search. It says what this creature is, and only the
    /// numbers a DM most often changes have fields of their own.
    fn creature_pane(ui: &mut egui::Ui, editor: &mut CreatureEditor) -> Option<CreatureStep> {
        let dirty = editor.dirty();
        let (mut save, mut close) = (false, false);
        ui.label(
            Type::Heading
                .rich(ui, "This campaign's creature")
                .color(colors().text),
        );
        ui.horizontal(|ui| {
            if dirty {
                close = Action::quiet("Discard")
                    .name("Discard creature changes")
                    .density(Density::Read)
                    .show(ui)
                    .clicked();
                design::status(
                    ui,
                    Status {
                        icon: icons::DRAFT,
                        word: "Unsaved",
                        color: colors().document.unsaved,
                    },
                );
            } else {
                ui.label(
                    Type::Small
                        .rich(ui, "No changes to save.")
                        .color(colors().muted),
                );
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                save = Action::secondary("Save creature")
                    .density(Density::Read)
                    .key("Ctrl+S")
                    .blocked_if(!dirty, "No changes to save.")
                    .show(ui)
                    .clicked();
                if !dirty {
                    close = Action::quiet("Done")
                        .name("Done with this creature")
                        .density(Density::Read)
                        .show(ui)
                        .clicked();
                }
            });
        });
        if let Some(error) = &editor.error {
            design::message(ui, Tone::Error, error);
        }
        // As wide as the pane: a form's fixed width would push the scroll bar past its edge.
        design::field(ui, "Name", None, None, |ui| {
            let width = ui.available_width() - 2.0 * design::S;
            design::text_input(ui, "Name", &mut editor.fields.name, "Name", width)
        });
        let focus_hp = std::mem::take(&mut editor.focus);
        ui.horizontal(|ui| {
            design::field(ui, "Hit points", None, None, |ui| {
                let response = design::number_field(
                    ui,
                    "Hit points",
                    &mut editor.fields.hp,
                    "HP",
                    56.0,
                    Density::Read.control(),
                );
                if focus_hp {
                    design::focus_and_select(ui, response.id, &editor.fields.hp);
                }
            });
            design::field(ui, "Armor class", None, None, |ui| {
                design::number_field(
                    ui,
                    "Armor class",
                    &mut editor.fields.armor_class,
                    "AC",
                    56.0,
                    Density::Read.control(),
                );
            });
        });
        // The file's source, closed until wanted: the common changes have fields above.
        design::quiet_section(ui, "Its text, as Markdown", |ui| {
            let room = ui.available_height();
            egui::ScrollArea::vertical()
                .id_salt("creature_text")
                .max_height(room)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    let response = ui.add(
                        egui::TextEdit::multiline(&mut editor.fields.body)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(ui.available_width() - 2.0 * design::S - design::M)
                            .margin(egui::vec2(design::S, design::S)),
                    );
                    design::named(ui, &response, "Creature text");
                });
        });
        if save {
            Some(CreatureStep::Save)
        } else {
            close.then_some(CreatureStep::Close)
        }
    }
    /// What the roster and the creature beside it asked for, once neither pane is borrowed.
    fn after_panes(
        &mut self,
        step: Option<CreatureStep>,
        make_yours: Option<usize>,
        active: &Campaign,
        modules: &[Module],
    ) {
        match step {
            Some(CreatureStep::Save) => self.save_creature(active, modules),
            Some(CreatureStep::Close) => {
                self.creature = None;
                self.clear_creature_block();
                self.focus_search = true;
            }
            None => {}
        }
        if let Some(slot) = make_yours {
            self.make_it_yours(slot, active, modules);
        }
    }
    fn start_run(
        &self,
        campaign: &Campaign,
        modules: &[Module],
    ) -> Result<crate::encounter_run::Run, String> {
        if self.has_run() {
            return Err(BLOCKED_BY_RUN.to_owned());
        }
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
        // What each carried value would have been from the sheet, by place in the roster.
        let mut sheets = Vec::new();
        for slot in &fresh.setup.roster {
            let reference = slot
                .creature
                .as_ref()
                .ok_or("Party-slot bindings are not supported by this runner yet.")?;
            let id = snapshot.catalogue.resolve_ref(reference)?;
            let mut entry = snapshot.catalogue.entries[&id].entry.clone();
            if let Some(name) = snapshot.npc_name_for(&entry, &slot.overrides) {
                entry
                    .fields
                    .insert("splat_run_npc_name".into(), name.into());
            }
            // A synced player's sheet decides these, so the run starts from its copy of the sheet.
            let synced =
                fresh.setup.combat_sync && snapshot.creatures.iter().any(|c| c.id == id && c.party);
            let starting = if synced {
                Mapping::new()
            } else {
                slot.initial.clone()
            };
            let present = match slot.extra.get("initial_presence").and_then(Value::as_str) {
                None | Some("active") => true,
                Some("absent" | "removed") => false,
                Some(_) => return Err(format!("{}: invalid starting presence.", slot.key)),
            };
            let overrides = Overrides::parse(&slot.overrides)
                .map_err(|error| format!("{}: {error}", slot.key))?;
            let mut participant = crate::encounter_run::Participant::with_overrides(
                slot.key.clone(),
                slot.side.clone(),
                entry,
                &starting,
                &overrides,
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
            // A starting HP written in the encounter is a decision, and it stays.
            if !slot.initial.contains_key("hp")
                && let Some(carry) = &self.carry
                && let Some(sheet) = participant.carry_over(carry)
            {
                sheets.push((participants.len(), sheet.0, sheet.1));
            }
            participants.push(participant);
        }
        let setup = serde_yaml_ng::to_string(&fresh.setup).map_err(|e| e.to_string())?;
        let entry = Entry::parse(&fresh.path, &fresh.source)?;
        let mut run = crate::encounter_run::Run::new(
            fresh.name,
            participants,
            format!(
                "{}\n\n{}\n\nPhases, assumptions, completion conditions, and limits are resolved manually.\n\n```yaml\n{setup}\n```",
                entry.body,
                crate::encounter_run::SETUP_HEADING,
            ),
        )?;
        run.came_from(entry.id);
        run.configure_storage(
            campaign.id,
            &snapshot.options.directory(campaign)?,
            snapshot.options.initiative,
        )?;
        if fresh.setup.combat_sync && campaign.dndbeyond_campaign_id.is_some() {
            run.keep_party_synced();
        }
        if let Some(carry) = self
            .carry
            .as_ref()
            .filter(|carry| !sheets.is_empty() || !carry.missing.is_empty())
        {
            run.note_carry(carry, sheets);
        }
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
        if self.run.is_some() {
            return;
        }
        match crate::encounter_run::Run::open(path, campaign) {
            Ok(mut run) => {
                run.restore_sync(synced);
                self.run = Some(run);
                self.go_to_run = true;
            }
            Err(error) => {
                self.error = Some(format!(
                    "The run open at last close did not reopen: {error}"
                ));
            }
        }
    }
    /// The Run page. Returns true when the run has just closed.
    pub fn show_run(&mut self, ui: &mut egui::Ui, active: &Campaign) -> bool {
        let Some(run) = &mut self.run else {
            return false;
        };
        if let Some(snapshot) = &self.snapshot {
            run.groups = Some(snapshot.groups.clone());
            run.creatures = Some(snapshot.run_creatures.clone());
        }
        run.planned.clone_from(&self.planned_groups);
        if !run.show(ui) {
            return false;
        }
        // A setup nobody touched holds nothing. It would only be a line to remove later.
        if run.untouched_setup()
            && let Some(path) = run.path()
        {
            let _ = fs::remove_file(path);
        }
        self.run = None;
        self.read_runs_again(active);
        true
    }
    pub fn read_runs_again(&mut self, active: &Campaign) {
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
    }
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        campaigns: &Campaigns,
        party: &Party,
        story: &Story<'_>,
    ) {
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
        if self.group_draft.is_some() {
            // Taken up on this page, it is this page's. Its save stays here, and links nothing.
            self.group_in_plan = false;
            self.group_editor_view(ui, &active, campaigns, false);
            if self.peek.is_some() {
                self.editor_pane = EditorPane::Add;
            }
        } else if self.draft.is_some() {
            self.editor_view(ui, &active, campaigns, party);
            // What was just read may be what the DM adds next, so the results stay in view.
            if self.peek.is_some() {
                self.editor_pane = EditorPane::Add;
            }
        } else {
            self.list_view(ui, &active, campaigns, story);
        }
    }

    fn start_blocked(&self) -> Option<&'static str> {
        if self.has_run() {
            Some(BLOCKED_BY_RUN)
        } else if self.creature_dirty() {
            Some(BLOCKED_BY_CREATURE)
        } else if self.draft_dirty() {
            Some(BLOCKED_BY_DRAFT)
        } else {
            None
        }
    }

    /// One step back from `Discard changes`. Undo is the guard, so no dialog is needed.
    fn discard_undo(&mut self, ui: &mut egui::Ui) {
        // A creature's changes and a group's are not this encounter's, and leave its undo alone.
        if self.draft.as_ref().is_some_and(Draft::dirty) {
            self.discarded = None;
        }
        // Each undo waits while the other kind of draft is open. Restored behind that editor,
        // a draft could be neither reached nor closed.
        if self.encounter_undo_ready() && design::draft_discarded(ui) {
            self.draft = self.discarded.take();
            self.message = None;
            self.error = None;
        }
        if self.group_draft.as_ref().is_some_and(GroupDraft::dirty) {
            self.discarded_group = None;
        }
        if self.group_undo_ready() && design::draft_discarded(ui) {
            self.group_draft = self.discarded_group.take();
            self.message = None;
            self.error = None;
        }
    }

    fn encounter_undo_ready(&self) -> bool {
        self.discarded.is_some() && self.group_draft.is_none()
    }

    fn group_undo_ready(&self) -> bool {
        self.discarded_group.is_some() && self.draft.is_none()
    }

    /// One step back from `Delete` on a saved run. Undo is the guard, so no dialog is needed.
    fn delete_undo(&mut self, ui: &mut egui::Ui, active: &Campaign) {
        let Some(deleted) = &self.deleted_run else {
            return;
        };
        let words = deleted.message();
        if design::message_with_action(
            ui,
            Tone::Info,
            &words,
            crate::encounter_run::DeletedRun::UNDO_LABEL,
        ) {
            match deleted.restore() {
                Ok(()) => {
                    self.deleted_run = None;
                    self.error = None;
                    self.read_runs_again(active);
                }
                Err(error) => self.error = Some(format!("The run did not come back: {error}")),
            }
        }
    }

    fn results(&mut self, ui: &mut egui::Ui) {
        self.discard_undo(ui);
        if let Some(e) = &self.error {
            design::message(ui, Tone::Error, e);
        }
        if let Some(m) = &self.message {
            design::message(ui, Tone::Success, m);
        }
    }

    fn list_view(
        &mut self,
        ui: &mut egui::Ui,
        active: &Campaign,
        campaigns: &Campaigns,
        story: &Story<'_>,
    ) {
        let size = egui::vec2(
            ui.available_width().min(LIST_MEASURE),
            ui.available_height(),
        );
        ui.allocate_ui_with_layout(size, egui::Layout::top_down(egui::Align::LEFT), |ui| {
            self.ledger_view(ui, active, campaigns, story);
        });
    }

    fn ledger_view(
        &mut self,
        ui: &mut egui::Ui,
        active: &Campaign,
        campaigns: &Campaigns,
        story: &Story<'_>,
    ) {
        ui.set_max_width(ui.available_width().min(LIST_MEASURE));
        let loading = self.pending.is_some();
        let blocked = self.draft_dirty().then_some(BLOCKED_BY_DRAFT);
        let settings_changed = self.settings_unsaved();
        let mut new = false;
        let mut new_group = false;
        let mut reload = false;
        // With nothing here yet, the page's one action is in its body.
        let empty = self
            .snapshot
            .as_ref()
            .is_some_and(|s| s.templates.is_empty() && s.runs.is_empty() && s.groups.is_empty());
        // A group can be prepared while a run is open. It only joins the run when it is added.
        let group_blocked = self.draft_dirty().then_some(BLOCKED_BY_DRAFT);
        let open_run = self
            .run
            .as_ref()
            .and_then(crate::encounter_run::Run::live_info);
        let context = self
            .snapshot
            .as_ref()
            .map(|snapshot| {
                let page = history::arrange(
                    &snapshot.templates,
                    &snapshot.runs,
                    open_run.as_ref(),
                    story.events,
                    story.sessions,
                );
                let mut counts = Vec::new();
                let ready = page.ready.iter().filter(|row| !row.unfinished_run).count();
                if ready > 0 {
                    counts.push(design::count(ready, "ready", "ready"));
                }
                if !page.in_progress.is_empty() {
                    counts.push(design::count(
                        page.in_progress.len(),
                        "in progress",
                        "in progress",
                    ));
                }
                counts.join(" · ")
            })
            .unwrap_or_else(|| "Reading encounters…".into());
        design::page_header(ui, "Encounters", Some(&context), |ui| {
            if !empty {
                new = Action::secondary("New encounter")
                    .icon(icons::ADD)
                    .blocked(blocked.or(loading.then_some("Still reading this campaign.")))
                    .show(ui)
                    .clicked();
                new_group = Action::quiet("New group")
                    .icon(icons::ADD)
                    .blocked(group_blocked.or(loading.then_some("Still reading this campaign.")))
                    .hint("A reusable set of creatures to add to a running encounter")
                    .show(ui)
                    .clicked();
            }
            // A run stays open through this, so a note fixed outside Splat reaches its Groups tab.
            reload = Action::quiet("Reload from disk")
                .icon(icons::REFRESH)
                .blocked(
                    group_blocked
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
            self.start_new_draft();
            return;
        }
        if new_group {
            self.group_in_plan = false;
            self.group_draft = Some(GroupDraft::new());
            self.expanded = None;
            self.focus_name = true;
            self.error = None;
            self.message = None;
            return;
        }
        if reload {
            if self.has_run() {
                self.error = None;
                self.refresh_content(ui.ctx(), campaigns, &active.path);
            } else {
                self.reload(ui.ctx(), active.clone(), campaigns.modules().to_vec());
                return;
            }
        }
        self.results(ui);
        self.delete_undo(ui, active);
        self.group_remove_undo(ui, active, campaigns.modules());
        if self.pending.is_some() && self.snapshot.is_none() {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
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
                Type::Small
                    .rich(
                        ui,
                        format!(
                            "{} {}",
                            icons::WARN,
                            design::count(snapshot.issues.len(), "content issue", "content issues")
                        ),
                    )
                    .color(colors().result.warning),
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
        if empty {
            let mut create = false;
            let mut create_group = false;
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
                    create_group = Action::quiet("New group")
                        .icon(icons::ADD)
                        .blocked(group_blocked)
                        .hint("A reusable set of creatures to add to a running encounter")
                        .show(ui)
                        .clicked();
                },
            );
            if create {
                self.start_new_draft();
            }
            if create_group {
                self.group_in_plan = false;
                self.group_draft = Some(GroupDraft::new());
                self.expanded = None;
                self.focus_name = true;
            }
            return;
        }
        let live = self
            .run
            .as_ref()
            .and_then(crate::encounter_run::Run::live_info);
        let page = history::arrange(
            &snapshot.templates,
            &snapshot.runs,
            live.as_ref(),
            story.events,
            story.sessions,
        );
        let start_blocked = self.start_blocked();
        let mut action = None;
        egui::ScrollArea::vertical()
            .id_salt("encounter_list")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                in_progress_list(
                    ui,
                    &page,
                    &snapshot.templates,
                    live.as_ref(),
                    (start_blocked, loading),
                    story,
                    &mut action,
                );
                ready_list(ui, &page, live.as_ref(), start_blocked, &mut action);
                ui.add_space(design::L);
                group_list(ui, snapshot, group_blocked, &mut action);
                history_list(
                    ui,
                    &page,
                    start_blocked,
                    live.as_ref(),
                    self.history_read.as_ref(),
                    &mut action,
                );
            });
        let Some(action) = action else {
            return;
        };
        match action {
            ListAction::BackToRun => self.go_to_run = true,
            ListAction::OpenGroup(path) => match GroupDraft::open(&active.path, &path) {
                Ok(draft) => {
                    self.group_in_plan = false;
                    self.group_draft = Some(draft);
                    self.expanded = None;
                    self.error = None;
                    self.message = None;
                }
                Err(e) => self.error = Some(e),
            },
            ListAction::RemoveGroup(path) => match groups::RemovedGroup::remove(active, &path) {
                Ok(removed) => {
                    self.removed_saved_group = Some(removed);
                    self.reload_snapshot(active, campaigns.modules());
                    self.saved = true;
                }
                Err(error) => self.error = Some(error),
            },
            ListAction::Resume(path) => self.open_saved_run(&path, active),
            ListAction::ReadHistory(path) => {
                if self
                    .history_read
                    .as_ref()
                    .is_some_and(|(open, _)| *open == path)
                {
                    self.history_read = None;
                } else {
                    self.open_saved_run(&path, active);
                }
            }
            ListAction::Delete(path) => {
                if self
                    .run
                    .as_ref()
                    .and_then(|run| run.path())
                    .is_some_and(|open| open == path)
                {
                    self.error = Some("Close this run before deleting its save.".to_owned());
                    return;
                }
                let deleted = self
                    .snapshot
                    .as_ref()
                    .and_then(|s| s.runs.iter().find(|run| run.path == path))
                    .map(crate::encounter_run::DeletedRun::delete);
                match deleted {
                    Some(Ok(deleted)) => {
                        self.deleted_run = Some(deleted);
                        self.error = None;
                        self.read_runs_again(active);
                    }
                    Some(Err(e)) => self.error = Some(e),
                    None => {}
                }
            }
            ListAction::Edit(path) => match Draft::open(&active.path, &path) {
                Ok(d) => {
                    self.draft = Some(d);
                    self.from_session = false;
                    self.plan_on_save = false;
                    self.error = None;
                    self.message = None;
                }
                Err(e) => self.error = Some(e),
            },
            ListAction::Run(path) => self.run_path(&path, active, campaigns.modules()),
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

    fn editor_view(
        &mut self,
        ui: &mut egui::Ui,
        active: &Campaign,
        campaigns: &Campaigns,
        party: &Party,
    ) {
        let dirty = self.encounter_dirty();
        let creature_dirty = self.creature_dirty();
        let mut back = false;
        let mut save = false;
        let mut discard = false;
        let mut run_it = false;
        // Beside Lookup at the minimum window, the roster and the search take turns.
        let compact = ui.available_width() < 760.0;
        // The title and its actions need more room than the panes do.
        let has_run = self.has_run();
        let loading = self.pending.is_some();
        let saved_note = self.message.clone();
        let unfinished = self
            .draft
            .as_ref()
            .and_then(|draft| {
                self.snapshot.as_ref()?.runs.iter().find(|run| {
                    run.standing == crate::encounter_run::Standing::Unfinished
                        && (run.source == Some(draft.id)
                            || (run.source.is_none() && run.encounter == draft.saved_name))
                })
            })
            .cloned();
        let mut start_another = false;
        let draft_ref = self.draft.as_mut().expect("the editor has a draft");
        let focus_name = &mut self.focus_name;
        let focus_search_after_name = &mut self.focus_search;
        let from_session = self.from_session;
        let stack_actions = ui.available_rect_before_wrap().width() < design::READER_MEASURE;
        ui.horizontal_wrapped(|ui| {
            let back_name = if from_session {
                "Back to session"
            } else {
                "Back to encounters"
            };
            back = Action::quiet(icons::BACK)
                .name(back_name)
                .hint(back_name)
                .blocked(
                    dirty
                        .then_some("Save or discard this encounter first.")
                        .or(creature_dirty.then_some(BLOCKED_BY_CREATURE)),
                )
                .show(ui)
                .clicked();
            let width = if stack_actions {
                (ui.available_rect_before_wrap().width() - 2.0 * design::S)
                    .max(Density::Scan.control())
            } else {
                (ui.available_rect_before_wrap().width()
                    - design::S * if unfinished.is_some() { 38.0 } else { 33.0 }
                    - if has_run {
                        design::number_width(ui, "Close the open run first", &Type::Small.font(ui))
                            + design::L
                    } else {
                        0.0
                    }
                    - if dirty {
                        0.0
                    } else {
                        design::number_width(ui, "No changes to save.", &Type::Small.font(ui))
                            + design::L
                    })
                .max(Density::Scan.control())
            };
            let response = ui.add(
                egui::TextEdit::singleline(&mut draft_ref.name)
                    .id(design::focus_id(NAME_FIELD))
                    .desired_width(width)
                    .min_size(egui::vec2(0.0, Density::Scan.control()))
                    .margin(egui::vec2(design::S, design::XS))
                    .font(Type::Heading.font(ui)),
            );
            design::named(ui, &response, &format!("{NAME_FIELD}: {}", draft_ref.name));
            if std::mem::take(focus_name) {
                design::focus_and_select(ui, response.id, &draft_ref.name);
            }
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                *focus_search_after_name = true;
            }
            let action_width = if stack_actions {
                ui.max_rect().width()
            } else {
                ui.available_rect_before_wrap().width()
            };
            ui.allocate_ui_with_layout(
                egui::vec2(action_width, Density::Scan.control()),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    let start_blocked = has_run
                        .then_some(BLOCKED_BY_RUN)
                        .or(creature_dirty.then_some(BLOCKED_BY_CREATURE))
                        .or(loading.then_some("Still reading this campaign."))
                        .or(draft_ref
                            .setup
                            .roster
                            .is_empty()
                            .then_some("Add participants first"));
                    if unfinished.is_some() {
                        row_menu(ui, &draft_ref.name, |ui| {
                            start_another = menu_action(ui, "Start another fight", start_blocked);
                        });
                    }
                    let word = if unfinished.is_some() {
                        "Resume"
                    } else {
                        START_FIGHT
                    };
                    run_it = if has_run || (dirty && unfinished.is_none()) {
                        Action::secondary(word)
                    } else {
                        Action::primary(word)
                    }
                    .blocked(if unfinished.is_some() {
                        has_run
                            .then_some(BLOCKED_BY_RUN)
                            .or(creature_dirty.then_some(BLOCKED_BY_CREATURE))
                            .or(loading.then_some("Still reading this campaign."))
                    } else {
                        start_blocked
                    })
                    .hint(if unfinished.is_some() {
                        "Save this encounter and resume its unfinished run."
                    } else {
                        "Save this encounter and start a run."
                    })
                    .show(ui)
                    .clicked();
                    if has_run {
                        blocked_reason(ui, Some("Close the open run first"));
                        ui.add_space(design::L);
                    }
                    save = if dirty && !has_run && unfinished.is_none() {
                        Action::primary("Save encounter")
                    } else {
                        Action::secondary("Save encounter")
                    }
                    .blocked_if(!dirty, "No changes to save.")
                    .show(ui)
                    .clicked();
                    blocked_reason(ui, (!dirty).then_some("No changes to save."));
                },
            );
        });
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Read.control());
            if dirty {
                discard = Action::quiet("Discard")
                    .blocked(creature_dirty.then_some(BLOCKED_BY_CREATURE))
                    .show(ui)
                    .clicked();
            }
            let (icon, word, color) = if dirty {
                (icons::DRAFT, "Unsaved", colors().document.unsaved)
            } else if saved_note.is_some() {
                (icons::OK, "Saved", colors().document.saved)
            } else {
                (icons::OK, "No changes", colors().muted)
            };
            if dirty || saved_note.is_some() {
                design::status(ui, Status { icon, word, color });
            }
            if dirty {
                blocked_reason(ui, Some(draft_switch_reason()));
            }
            if let Some(run) = &unfinished {
                design::status(
                    ui,
                    Status {
                        icon: icons::RUN,
                        word: &run_state_word(
                            run,
                            self.run
                                .as_ref()
                                .and_then(|open| open.path())
                                .is_none_or(|path| path != run.path),
                        ),
                        color: colors().muted,
                    },
                );
            }
            blocked_reason(
                ui,
                creature_dirty
                    .then_some(BLOCKED_BY_CREATURE)
                    .or(loading.then_some("Still reading this campaign."))
                    .or(
                        (!has_run && unfinished.is_none() && draft_ref.setup.roster.is_empty())
                            .then_some("Add participants first"),
                    ),
            );
        });
        if unfinished.is_some() {
            blocked_reason(
                ui,
                Some("Changes apply to the next fight. Add to this fight from Arrivals."),
            );
        }
        design::rule(ui);
        if back {
            self.draft = None;
            self.message = None;
            self.error = None;
            self.creature = None;
            self.wants_session = std::mem::take(&mut self.from_session);
            self.plan_on_save = false;
            return;
        }
        if discard {
            self.creature = None;
            self.plan_on_save = false;
            let draft = self.draft.take().expect("the editor has a draft");
            // A new draft has nothing to go back to, so it is no longer the session's.
            if draft.baseline.is_none() {
                self.from_session = false;
            }
            if draft.baseline.is_some() {
                match Draft::open(&active.path, &draft.path) {
                    Ok(d) => self.draft = Some(d),
                    Err(e) => self.error = Some(e),
                }
            }
            self.discarded = Some(draft);
            self.discarded_group = None;
            return;
        }
        if save {
            self.save_draft(ui.ctx(), active, campaigns.modules());
        }
        if run_it || start_another {
            if dirty {
                self.save_draft(ui.ctx(), active, campaigns.modules());
            }
            if !self.draft_dirty() {
                if !start_another && let Some(run) = &unfinished {
                    self.open_saved_run(&run.path, active);
                } else {
                    self.open_run(active, campaigns.modules());
                }
                // A fight started from the session goes back there, not to this editor.
                if self.from_session && self.run.is_some() {
                    self.draft = None;
                    self.from_session = false;
                    self.run_from_session = true;
                    return;
                }
            }
        }
        self.discard_undo(ui);
        // After a load in progress, so that a group just saved is among them.
        if self.pending.is_none()
            && let (Some(snapshot), Some(draft)) = (&self.snapshot, self.draft.as_mut())
            && !draft.groups_read
            && let Err(e) = draft.read_groups(&active.path, snapshot)
        {
            self.error = Some(e);
        }
        if let Some(e) = &self.error {
            design::message(ui, Tone::Error, e);
        }
        let Some(snapshot) = &self.snapshot else {
            crate::design::spinner(ui);
            return;
        };
        let mut make_yours = None;
        let mut creature_step = None;
        let open_creature = self.creature.is_some();
        let draft = self.draft.as_mut().expect("the editor has a draft");
        let mut pane = self.editor_pane;
        if compact {
            // The count stays on screen, so that Add shows a result on the other tab.
            let roster = if draft.setup.roster.is_empty() {
                "Roster".to_owned()
            } else {
                format!("Roster ({})", grouped(draft.setup.roster.len() as u64))
            };
            design::segmented(
                ui,
                &mut pane,
                &[
                    (EditorPane::Roster, roster.as_str()),
                    (
                        EditorPane::Add,
                        if open_creature {
                            "This creature"
                        } else {
                            "Add participants"
                        },
                    ),
                ],
            );
            self.editor_pane = pane;
        }
        let search = &mut self.search;
        let focus_search = &mut self.focus_search;
        let peek = &mut self.peek;
        let expanded = &mut self.expanded;
        let linked = active.dndbeyond_campaign_id.is_some();
        if compact {
            let go_to_search = match pane {
                EditorPane::Roster => Self::roster_pane(
                    ui,
                    draft,
                    snapshot,
                    focus_search,
                    expanded,
                    linked.then_some(party),
                    &mut make_yours,
                ),
                EditorPane::Add => {
                    match &mut self.creature {
                        Some(editor) => creature_step = Self::creature_pane(ui, editor),
                        None => {
                            Self::add_pane(ui, draft, snapshot, search, focus_search, peek, compact)
                        }
                    }
                    false
                }
            };
            if go_to_search {
                self.editor_pane = EditorPane::Add;
            }
            self.after_panes(creature_step, make_yours, active, campaigns.modules());
            return;
        }
        // In a wide window the roster keeps a name near its controls, and the two panes
        // stay together at the left.
        let width = design::side_panel_width(ui.ctx().content_rect().width())
            .max(ui.available_width() - ROSTER_MAX_WIDTH)
            .min(ADD_PANE_MAX_WIDTH);
        let creature = &mut self.creature;
        design::Column::right(
            ADD_PANE,
            width,
            design::PANEL_MIN_WIDTH..=ADD_PANE_MAX_WIDTH,
        )
        .page_keeps(design::RUN_LIST_KEEPS)
        .frame(egui::Frame::new().inner_margin(egui::Margin {
            left: design::S as i8,
            right: 0,
            top: design::S as i8,
            bottom: 0,
        }))
        .show_inside(ui, |ui| match creature {
            Some(editor) => creature_step = Self::creature_pane(ui, editor),
            None => Self::add_pane(ui, draft, snapshot, search, focus_search, peek, compact),
        });
        egui::CentralPanel::default()
            // The panel's drag handle lies on this edge, so the roster stops short of it.
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: 0,
                right: design::S as i8,
                top: design::S as i8,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                // A name stays near its controls in a wide window, until the DM moves the panel.
                if !design::column_dragged(ui.ctx(), ADD_PANE) {
                    ui.set_max_width(ui.available_width().min(ROSTER_MAX_WIDTH));
                }
                Self::roster_pane(
                    ui,
                    draft,
                    snapshot,
                    focus_search,
                    expanded,
                    linked.then_some(party),
                    &mut make_yours,
                );
            });
        self.after_panes(creature_step, make_yours, active, campaigns.modules());
    }

    fn add_pane(
        ui: &mut egui::Ui,
        draft: &mut Draft,
        snapshot: &Snapshot,
        search: &mut String,
        focus_search: &mut bool,
        peek: &mut Option<Uuid>,
        compact: bool,
    ) {
        if !compact {
            design::group_heading(ui, "Add participants", 0);
        }
        if draft.add_to.is_none() {
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(ui, "Party members use the saved local character copies.")
                        .color(colors().muted),
                )
                .wrap(),
            );
        }
        groups::add_target(ui, draft, snapshot);
        if draft.target_group().is_some() {
            let chosen = Self::creature_search(
                ui,
                snapshot.creatures.iter().filter(|c| !c.linked),
                search,
                focus_search,
                peek,
                SearchRoster {
                    already: &|_| false,
                },
            );
            if let Some((id, party)) = chosen {
                draft.add_selected(id, party, snapshot);
            }
            return;
        }
        // Who is still to come: a party member belongs on a roster once.
        let on_roster = |draft: &Draft, id: Uuid| {
            draft
                .setup
                .roster
                .iter()
                .any(|slot| slot.creature.as_ref().is_some_and(|c| c.id == Some(id)))
        };
        let any_party = snapshot.creatures.iter().any(|c| c.party);
        let missing: Vec<Uuid> = snapshot
            .creatures
            .iter()
            .filter(|c| c.party && !on_roster(draft, c.id))
            .map(|c| c.id)
            .collect();
        if !missing.is_empty() || !any_party {
            if Action::secondary("Add party")
                .icon(icons::PARTY)
                .blocked(if !any_party {
                    Some("No saved party copies yet. Connect to D&D Beyond in Settings.")
                } else if missing.is_empty() {
                    Some("The whole party is on the roster.")
                } else {
                    None
                })
                .show(ui)
                .clicked()
            {
                for id in missing {
                    draft.add(id, true);
                }
                *focus_search = true;
            }
        } else {
            design::status(
                ui,
                Status {
                    icon: icons::OK,
                    word: "Party in roster",
                    color: colors().muted,
                },
            );
        }
        if !any_party {
            blocked_reason(
                ui,
                Some("No saved party copies yet. Connect to D&D Beyond in Settings."),
            );
        }
        let chosen = Self::creature_search(
            ui,
            snapshot.creatures.iter(),
            search,
            focus_search,
            peek,
            SearchRoster {
                already: &|c: &Creature| c.party && on_roster(draft, c.id),
            },
        );
        if let Some((id, party)) = chosen {
            draft.add_selected(id, party, snapshot);
        }
    }

    /// The search and its results, for a roster and for a group. Returns what the DM added.
    fn creature_search<'a>(
        ui: &mut egui::Ui,
        creatures: impl Iterator<Item = &'a Creature>,
        search: &mut String,
        focus_search: &mut bool,
        peek: &mut Option<Uuid>,
        roster: SearchRoster<'_>,
    ) -> Option<(Uuid, bool)> {
        let mut chosen = None;
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
        let mut matches: Vec<&Creature> = creatures
            .filter(|c| {
                c.lower.contains(&query) && (!query.is_empty() || !c.party || !(roster.already)(c))
            })
            .collect();
        // The plain name first, so "wolf" finds Wolf before Dire Wolf. The sort keeps the
        // list's order inside each rank.
        if !query.is_empty() {
            matches.sort_by_key(|c| {
                let name = c.name.to_lowercase();
                u8::from(name != query) + u8::from(!name.starts_with(&query))
            });
        }
        // Enter adds the best match, with focus kept for the next name.
        if response.lost_focus()
            && ui.input(|i| i.key_pressed(egui::Key::Enter))
            && !query.is_empty()
            && let Some(first) = matches.first()
            && !(roster.already)(first)
        {
            chosen = Some((first.id, first.party));
            *focus_search = true;
        }
        if matches.is_empty() {
            let index = ui.ctx().data(|data| {
                data.get_temp::<Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                    "reader_link_index",
                ))
            });
            if let Some(index) = index {
                for (package, entry, name, owner) in index.unused_creatures(&query) {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            Type::Small
                                .rich(
                                    ui,
                                    format!(
                                        "{name} is in {owner}, which this campaign does not use."
                                    ),
                                )
                                .color(colors().muted),
                        );
                        if design::link(ui, "Use it").clicked() {
                            ui.ctx().data_mut(|data| {
                                data.insert_temp(
                                    egui::Id::new("use_package_entry"),
                                    (package, entry),
                                )
                            });
                        }
                    });
                }
            }
            design::empty_state(
                ui,
                icons::SEARCH,
                "No creatures match",
                "Check the spelling, or the packages this campaign uses.",
                |_| {},
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
                            // The name is what the eye looks for, so it comes first.
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                ui.set_max_width(
                                    (ui.available_width() - design::S * 12.0).max(design::S * 10.0),
                                );
                                if creature_name_link(ui, &c.name)
                                    .on_hover_text("Read this entry in Lookup")
                                    .clicked()
                                {
                                    *peek = Some(c.id);
                                }
                                detail_columns(ui, &c.detail, c.source.as_deref());
                            });
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if (roster.already)(c) {
                                        ui.label(
                                            Type::Small.rich(ui, "In roster").color(colors().muted),
                                        );
                                    } else if Action::secondary("Add")
                                        .density(Density::Read)
                                        .show(ui)
                                        .clicked()
                                    {
                                        chosen = Some((c.id, c.party));
                                        *focus_search = true;
                                    }
                                },
                            );
                        });
                    });
                }
            });
        chosen
    }

    fn roster_pane(
        ui: &mut egui::Ui,
        draft: &mut Draft,
        snapshot: &Snapshot,
        focus_search: &mut bool,
        expanded: &mut Option<String>,
        linked_party: Option<&Party>,
        make_yours: &mut Option<usize>,
    ) -> bool {
        // All of it scrolls, so that the settings give up their room to a group being built.
        egui::ScrollArea::vertical()
            .id_salt("encounter_editor")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                design::group_heading(ui, "Roster", draft.setup.roster.len());
                // Without a link there is no sheet to sync with, whatever the note says.
                let live = linked_party.filter(|_| draft.setup.combat_sync);
                if draft.setup.roster.is_empty() {
                    ui.add_space(design::S);
                    blocked_reason(ui, Some("No participants yet. Add the party and creatures from Add participants."));
                }
                Self::roster(ui, draft, snapshot, focus_search, expanded, live, make_yours);
                ui.add_space(design::M);
                if let Some(party) = linked_party {
                    Self::sync_option(ui, draft, snapshot, party);
                }
                ui.checkbox(&mut draft.setup.repeatable, REPEATABLE_OPTION);
                ui.add_space(design::M);
                design::group_heading(ui, "Read-aloud and notes", 0);
                let response = ui.add(
                    egui::TextEdit::multiline(&mut draft.body)
                        .hint_text("What you read to the table, then your tactics. The run shows this before the first turn.")
                        .desired_width(ui.available_width() - 2.0 * design::S - design::M)
                        .desired_rows(5)
                        .margin(egui::vec2(design::S, design::S)),
                );
                design::named(ui, &response, "Read-aloud and notes");
                groups::of_encounter(ui, draft, snapshot, focus_search, expanded)
            })
            .inner
    }

    /// Whether this encounter uses the party as D&D Beyond has it now. With it on, the DM sees
    /// what the party has left while they choose what to put against it.
    fn sync_option(ui: &mut egui::Ui, draft: &mut Draft, snapshot: &Snapshot, party: &Party) {
        ui.checkbox(&mut draft.setup.combat_sync, SYNC_OPTION);
        let words = if draft.setup.combat_sync {
            "The party starts with the HP and conditions on their sheets. When connected, the run and D&D Beyond stay matched."
        } else {
            "The party starts from their saved copies and the starting values here. Nothing in this encounter reaches D&D Beyond, unless you select Sync party in the run."
        };
        ui.add(egui::Label::new(Type::Small.rich(ui, words).color(colors().muted)).wrap());
        if !draft.setup.combat_sync {
            return;
        }
        let sheets: Vec<_> = draft
            .setup
            .roster
            .iter()
            .filter_map(|slot| slot_sheet(slot, snapshot, party))
            .collect();
        if let Some(line) = crate::party::party_line(&sheets, party.connected()) {
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Label::new(Type::Small.rich(ui, line).color(colors().text)).wrap());
                if !party.connected() && design::link(ui, "Connect to sync").clicked() {
                    ui.ctx().data_mut(|data| {
                        data.insert_temp(egui::Id::new("open_account_settings"), true)
                    });
                }
            });
        }
    }

    fn roster(
        ui: &mut egui::Ui,
        draft: &mut Draft,
        snapshot: &Snapshot,
        focus_search: &mut bool,
        expanded: &mut Option<String>,
        live: Option<&Party>,
        make_yours: &mut Option<usize>,
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
                let name = if resolved.is_some() {
                    groups::shown_name(&slot.overrides, name)
                } else {
                    name
                };
                let definition = resolved
                    .and_then(|id| snapshot.catalogue.entries.get(&id))
                    .map(|e| &e.entry);
                let creature =
                    resolved.and_then(|id| snapshot.creatures.iter().find(|c| c.id == id));
                let synced = live.is_some() && creature.is_some_and(|c| c.party);
                let detail = live
                    .zip(creature)
                    .and_then(|(party, creature)| party.sheet(creature.character.as_deref()?))
                    .map_or_else(
                        || {
                            creature.zip(definition).map_or_else(
                                String::new,
                                |(creature, entry)| {
                                    groups::shown_detail(
                                        &slot.overrides,
                                        entry,
                                        &slot.side,
                                        creature,
                                    )
                                },
                            )
                        },
                        |record| crate::party::SheetNow::of(record).line(),
                    );
                // Its side says it is the party's, and the name has the room.
                let detail = detail
                    .strip_prefix("Party copy · ")
                    .map_or_else(|| detail.clone(), str::to_owned);
                let detail = roster_detail(&detail);
                let detail = match creature.and_then(|creature| creature.source.as_deref()) {
                    Some(source) => format!("{detail} · {source}"),
                    None => detail,
                };
                let side_icon = if slot.side == "party" { icons::PARTY } else if slot.side == "enemies" { icons::ENCOUNTERS } else { icons::GROUP };
                let identity = definition.filter(|entry| snapshot.npc_name_for(entry, &slot.overrides).is_some()).map(|entry| format!("as {} · ", entry.name)).unwrap_or_default();
                let detail = format!("{identity}{side_icon} {} · {detail}", design::reader::human_label(&slot.side));
                let is_open = expanded.as_deref() == Some(slot.key.as_str());
                let member_row = ui.horizontal(|ui| {
                    ui.set_min_height(Density::Glance.row());
                    // Every slot is reserved, so columns line up between rows.
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        row_menu(ui, &name, |ui| {
                            if menu_action(ui, "Move up", (i == 0).then_some("Already first.")) { raise = Some(i); }
                            if menu_action(ui, "Move down", (i + 1 == count).then_some("Already last.")) { lower = Some(i); }
                            if menu_action(ui, "Duplicate", None) { duplicate = Some(i); }
                            if creature.is_some_and(|c| !c.party) {
                                if menu_action(ui, "Make it yours", None) { *make_yours = Some(i); }
                                ui.add(egui::Label::new(Type::Small.rich(ui, "Edit a copy in this campaign.").color(colors().muted)).wrap());
                            }
                            if menu_action(ui, "Remove", None) { remove = Some(i); }
                        });
                        if Action::quiet("Edit")
                            .density(Density::Read)
                            .name(groups::SLOT_OPTIONS)
                            .selected(is_open)
                            .show(ui)
                            .clicked()
                        {
                            *expanded = if is_open {
                                None
                            } else {
                                groups::show_form_when_it_opens(ui);
                                Some(slot.key.clone())
                            };
                        }
                        ui.add_space(design::XS);
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
                                // What a player has left is the reason to look, so all of it shows.
                                ui.spacing_mut().interact_size.y = 0.0;
                                ui.horizontal_wrapped(|ui| {
                                    ui.spacing_mut().item_spacing.x = 0.0;
                                    for (at, part) in detail.split(" · ").enumerate() {
                                        if at > 0 { ui.label(Type::Small.rich(ui, " · ").color(colors().muted)); }
                                        if part == "Bloodied" {
                                            design::status(ui, Status { icon: icons::BLOODIED, word: part, color: colors().hp.bloodied });
                                        } else {
                                            ui.label(Type::Small.rich(ui, stat_line(part)).color(colors().muted));
                                        }
                                    }
                                });
                            }
                        });
                    });
                });
                let member_row = member_row.response.rect;
                if let Some(entry) = definition
                    && !(slot.overrides.len() == 1 && snapshot.npc_name_for(entry, &slot.overrides).is_some())
                    && groups::changes_link(ui, &slot.overrides, entry, &slot.side, is_open)
                {
                    groups::show_form_when_it_opens(ui);
                    *expanded = Some(slot.key.clone());
                }
                if is_open {
                    ui.horizontal(|ui| {
                        ui.label(Type::Small.rich(ui, "Side").color(colors().muted));
                        side_control(ui, "Side", &mut slot.side, &sides);
                    });
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
                        // A synced player's sheet decides these. The values wait here in case
                        // the DM turns sync off.
                        let overrides: &[_] = if synced {
                            &[]
                        } else {
                            &[("hp", "Starting HP"), ("temporary_hp", "Temp HP")]
                        };
                        for &(field, label) in overrides {
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
                                if synced {
                                    "D&D Beyond decides this player's HP while this encounter syncs."
                                } else {
                                    "With a value off, the creature's own starting state applies."
                                },
                            )
                            .color(colors().muted),
                    );
                    if let Some(entry) = definition {
                        let done = groups::overrides_form(
                            ui,
                            &mut slot.overrides,
                            entry,
                            &slot.side,
                            Some(&sides),
                        );
                        if done { *expanded = None; ui.scroll_to_rect(member_row, Some(egui::Align::Min)); }
                    }
                }
                design::rule(ui);
            });
        }
        for side in draft.setup.roster.iter().flat_map(|slot| {
            [
                Some(slot.side.as_str()),
                slot.overrides.get("side").and_then(Value::as_str),
            ]
            .into_iter()
            .flatten()
        }) {
            if !side.trim().is_empty() && !draft.setup.sides.contains_key(side) {
                draft
                    .setup
                    .sides
                    .insert(side.into(), yaml("{hostile_to: []}"));
            }
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
        let run_open = self.has_run();
        let mut settings_saved = false;
        let mut error = None;
        let Some(snapshot) = &mut self.snapshot else {
            crate::design::spinner(ui);
            return;
        };
        ui.add_space(design::S);
        let closed_while = draft_open.then_some("Save or discard the open encounter first.");
        ui.scope_builder(egui::UiBuilder::new().max_rect(egui::Rect::from_min_size(
            ui.cursor().min, egui::vec2(ui.available_width().min(design::FORM_WIDTH), ui.available_height()))), |ui| {
        design::locked(ui, closed_while, |ui| {
            design::text_field(
                ui,
                "Run folder",
                &mut snapshot.options.folder,
                Some("Relative to the campaign folder, or an absolute path."),
                None,
            );
            design::field(ui, "Roll NPC initiative", None, None, |ui| {
                let combo = egui::ComboBox::from_id_salt("campaign_initiative")
                    .icon(design::combo_icon)
                    .width(ui.available_width().min(320.0))
                    .selected_text(snapshot.options.initiative.grouping.label())
                    .show_ui(ui, |ui| {
                        for mode in crate::encounter_settings::Grouping::ALL {
                            ui.selectable_value(
                                &mut snapshot.options.initiative.grouping,
                                mode,
                                mode.label(),
                            );
                        }
                    });
                design::named(ui, &combo.response, "Roll NPC initiative");
            });
            ui.checkbox(
                &mut snapshot.options.initiative.roster_ties,
                "Break initiative ties in roster order",
            );
            ui.label(
                Type::Small
                    .rich(ui, "This is the default for the roll in run setup, which can change it for one roll. Side and initiative groups use the first NPC's bonus and roll mode. Players keep their entered initiative. An encounter group follows this rule when it arrives, and the run's Arrivals tab says how.")
                    .color(colors().muted),
            );
            let changed = snapshot.options != snapshot.saved_options;
            let left = snapshot.options.runs_left_in(&snapshot.saved_options, &active);
            let run_files = left.as_ref().map_or(0, Vec::len);
            let folder_changed = snapshot.options.directory(&active).ok() != snapshot.saved_options.directory(&active).ok();
            let blocked = if run_open { Some("Close the open run to save run settings.") }
                else if folder_changed && left.is_err() { Some("Could not read saved runs. Check the old run folder before saving.") }
                else if !changed { Some("No changes to save.") } else { None };
            if run_files > 0 {
                design::message(ui, Tone::Warning, &format!("{} {} in {}. Splat will not show {} from the new folder.",
                    design::count(run_files, "saved run", "saved runs"), if run_files == 1 { "stays" } else { "stay" }, snapshot.saved_options.folder, if run_files == 1 { "it" } else { "them" }));
            }
            ui.horizontal(|ui| {
                let save_key = changed && blocked.is_none() && run_files == 0
                    && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S));
                if changed {
                    self.discarded_options = None;
                    design::status(
                        ui,
                        Status {
                            icon: icons::DRAFT,
                            word: "Unsaved",
                            color: colors().document.unsaved,
                        },
                    );
                    if Action::quiet("Discard").show(ui).clicked() {
                        self.discarded_options = Some(std::mem::replace(
                            &mut snapshot.options,
                            snapshot.saved_options.clone(),
                        ));
                    }
                }
                if !changed
                    && self.discarded_options.is_some()
                    && Action::secondary("Undo discard").show(ui).clicked()
                    && let Some(options) = self.discarded_options.take()
                {
                    snapshot.options = options;
                }
                let pressed = ui
                    .with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if run_files > 0 && !run_open {
                            let move_runs = Action::primary("Move runs and save").show(ui).clicked();
                            let leave_runs = Action::quiet("Save without moving").show(ui).clicked();
                            if move_runs {
                                match snapshot.options.move_runs_and_save(&snapshot.saved_options, &active, &snapshot.campaign_text) {
                                    Ok(warning) => { settings_saved = true; error = warning; },
                                    Err(e) => error = Some(e),
                                }
                            }
                            leave_runs
                        } else {
                            let pressed = Action::primary("Save run settings").key("Ctrl+S")
                                .blocked(blocked).show(ui).clicked();
                            design::blocked_reason(ui, blocked);
                            pressed
                        }
                    })
                    .inner;
                if (pressed || save_key) && !run_open {
                    if folder_changed && let Err(e) = &left { error = Some(e.clone()); return; }
                    match snapshot.options.save(&active, &snapshot.campaign_text) {
                        Ok(()) => settings_saved = true,
                        Err(e) => error = Some(e),
                    }
                }
            });
        });
        });
        if draft_open {
            design::message(
                ui,
                Tone::Info,
                "Save or discard the open encounter to change run settings.",
            );
        }
        if settings_saved {
            self.saved = true;
            self.reload(ui.ctx(), active, campaigns.modules().to_vec());
        }
        if let Some(e) = error {
            self.error = Some(e);
        }
        if let Some(e) = &self.error {
            design::message(ui, Tone::Error, e);
        }
    }
}

/// The widest a list row grows, so that its actions stay near the name they belong to.
use crate::design::LIST_MEASURE;
mod list;
pub(crate) use list::run_state_word;
use list::{
    blocked_reason, group_list, history_list, in_progress_list, menu_action, ready_list, row_menu,
    side_control,
};

pub(crate) fn draft_switch_reason() -> &'static str {
    "Save or discard to switch campaign."
}

fn roster_detail(detail: &str) -> String {
    let parts: Vec<_> = detail.split(" · ").collect();
    let mut result = Vec::new();
    if let Some(hp) = parts.iter().find(|p| p.starts_with("HP ")) {
        let mut hp = (*hp).to_owned();
        if let Some(temp) = parts.iter().find_map(|p| p.strip_prefix("Temp ")) {
            hp.push_str(&format!(" {} +{temp}", icons::TEMP_HP));
        }
        result.push(hp);
    }
    if let Some(ac) = parts.iter().find(|p| p.starts_with("AC ")) {
        result.push((*ac).to_owned());
    }
    result.extend(
        parts
            .iter()
            .filter(|p| !p.starts_with("HP ") && !p.starts_with("AC ") && !p.starts_with("Temp "))
            .map(|p| (*p).to_owned()),
    );
    result.join(" · ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn undo_discard_stays_in_its_campaign() {
        let (a, b) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let (first, second) = (campaign(a.path()), campaign(b.path()));
        let ctx = egui::Context::default();
        let mut encounters = Encounters {
            discarded: Some(Draft::new()),
            active: Some(first.clone()),
            ..Default::default()
        };
        encounters.reload(&ctx, first, Vec::new());
        assert!(encounters.discarded.is_some(), "a refresh keeps the undo");
        encounters.reload(&ctx, second, Vec::new());
        assert!(encounters.discarded.is_none(), "another campaign drops it");
    }
    /// Older notes do not change, and a note says so only when the DM turned this on.
    #[test]
    fn repeatable_is_written_only_when_it_is_on() {
        let mut setup = Setup::default();
        let written = serde_yaml_ng::to_string(&setup).unwrap();
        assert!(!written.contains("repeatable"));
        setup.repeatable = true;
        let written = serde_yaml_ng::to_string(&setup).unwrap();
        let read: Setup = serde_yaml_ng::from_str(&written).unwrap();
        assert!(read.repeatable);
    }
    /// Typing in a new group must not cost the DM the undo of an encounter they discarded.
    #[test]
    fn a_dirty_draft_of_the_other_kind_does_not_end_an_undo() {
        let mut encounters = Encounters {
            discarded: Some(Draft::new()),
            group_draft: Some(GroupDraft::new()),
            ..Default::default()
        };
        assert!(encounters.draft_dirty());
        let _ = egui::Context::default().run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| encounters.discard_undo(ui));
        });
        assert!(encounters.discarded.is_some());
    }

    /// Reload is how the DM takes a change made outside Splat. Keeping the old value would
    /// write it back over that change at the next save.
    #[test]
    fn a_refresh_keeps_settings_only_while_they_are_being_edited() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let mut old = Snapshot::load(&active, &[]).unwrap();
        let mut changed = old.options.clone();
        changed.folder = "Elsewhere".into();

        let mut new = Snapshot::load(&active, &[]).unwrap();
        new.options = changed.clone();
        new.saved_options = changed.clone();
        new.keep_edited_settings(&old);
        assert_eq!(new.saved_options, changed, "the file decides");

        old.options.folder = "Being typed".into();
        new.keep_edited_settings(&old);
        assert_eq!(new.options.folder, "Being typed", "the edit stays");
    }

    /// Restored behind the group editor, the encounter draft could be neither reached nor closed.
    #[test]
    fn undo_discard_waits_while_the_other_kind_of_draft_is_open() {
        let mut encounters = Encounters {
            discarded: Some(Draft::new()),
            group_draft: Some(GroupDraft::new()),
            ..Default::default()
        };
        assert!(!encounters.encounter_undo_ready());
        encounters.group_draft = None;
        assert!(encounters.encounter_undo_ready(), "back at the list");

        let mut encounters = Encounters {
            discarded_group: Some(GroupDraft::new()),
            draft: Some(Draft::new()),
            ..Default::default()
        };
        assert!(!encounters.group_undo_ready());
        encounters.draft = None;
        assert!(encounters.group_undo_ready(), "back at the list");
    }
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
    fn a_named_npc_uses_its_stat_block_and_existing_name_override() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let stat = creature(dir.path(), "Guard", false);
        let text = format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: world\nsplat_name: Rook Ambry\nsplat_world:\n  type: npc\n  creature: {{id: {stat}}}\n---\n",
            Uuid::new_v4()
        );
        let npc = write_note(dir.path(), "Rook.md", None, &text).unwrap().id;
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        assert!(
            snapshot
                .creatures
                .iter()
                .any(|c| c.id == npc && c.detail.starts_with("NPC · as Guard · CR "))
        );
        let mut draft = Draft::new();
        draft.add_selected(npc, false, &snapshot);
        assert_eq!(
            draft.setup.roster[0].creature.as_ref().unwrap().id,
            Some(stat)
        );
        assert_eq!(
            draft.setup.roster[0].overrides["name"].as_str(),
            Some("Rook Ambry")
        );
        draft.save(&campaign, &[]).unwrap();
        let reopened = Draft::open(dir.path(), &draft.path).unwrap();
        assert_eq!(
            reopened.setup.roster[0].overrides,
            draft.setup.roster[0].overrides
        );
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
        assert!(snapshot.templates.iter().any(|t| t.path == d.path));
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
    fn a_body_is_replaced_and_the_frontmatter_stays() {
        let text = "---\nsplat_name: A\nsplat_note: \"a\\n---b\"\n---\n\nOld words.\n";
        assert_eq!(
            with_body(text, "New words."),
            "---\nsplat_name: A\nsplat_note: \"a\\n---b\"\n---\n\nNew words.\n"
        );
        assert_eq!(with_body("no frontmatter", "New"), "no frontmatter");
    }

    #[test]
    fn a_stat_row_changes_and_a_missing_one_is_said() {
        let mut text = "| ` armor_class ` | ` 11 ` |  |\n| ` max_hp ` | ` 68 ` |  |\n".to_owned();
        assert!(set_table_value(&mut text, "max_hp", "90"));
        assert_eq!(
            text,
            "| ` armor_class ` | ` 11 ` |  |\n| ` max_hp ` | ` 90 ` |  |\n"
        );
        assert!(!set_table_value(&mut text, "speed", "30"));
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
    fn creatures_show_campaign_ownership_and_omit_the_main_source() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let hero = creature(dir.path(), "Hero", true);
        let local = creature(dir.path(), "Local", false);
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let source = |id: Uuid| {
            snapshot
                .creatures
                .iter()
                .find(|c| c.id == id)
                .unwrap()
                .source
                .as_deref()
        };
        assert_eq!(source(hero), None);
        assert_eq!(source(local), Some("Campaign"));
        // A new campaign uses the built-in package, so those rows need no source suffix.
        let builtin = crate::content_workspace::builtin_manifest().id;
        let main: Vec<_> = snapshot
            .creatures
            .iter()
            .filter(|creature| snapshot.catalogue.entries[&creature.id].owner == builtin)
            .collect();
        assert!(!main.is_empty());
        assert!(main.iter().all(|creature| creature.source.is_none()));
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
                    egui::CentralPanel::default().show(ctx, |ui| {
                        editor.show(
                            ui,
                            &campaigns,
                            &Party::default(),
                            &Story {
                                events: &[],
                                sessions: &[],
                            },
                        );
                    });
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
    #[test]
    fn history_never_takes_the_live_slot_or_writes_the_saved_run() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let wolf = stat_creature(dir.path(), "Wolf", 20);
        let mut draft = Draft::new();
        draft.add(wolf, false);
        draft.save(&campaign, &[]).unwrap();
        let template = draft.path.clone();
        let mut editor = Encounters {
            active: Some(campaign.clone()),
            draft: Some(draft),
            ..Default::default()
        };
        let ended = editor.start_run(&campaign, &[]).unwrap();
        let path = ended.path().unwrap().to_path_buf();
        let mut saved: serde_json::Value =
            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        saved["state"]["phase"] = serde_json::json!({"Ended": {"outcome": "Won"}});
        saved["state"]["round"] = 3.into();
        let bytes = serde_json::to_vec_pretty(&saved).unwrap();
        fs::write(&path, &bytes).unwrap();
        editor.open_saved_run(&path, &campaign);
        assert!(!editor.has_run());
        assert!(!editor.go_to_run);
        assert!(editor.history_read.as_ref().unwrap().1.ended);
        assert_eq!(fs::read(&path).unwrap(), bytes);

        editor.run = Some(editor.start_run(&campaign, &[]).unwrap());
        let current = editor.run().unwrap().live_info().unwrap().id;
        editor.open_saved_run(&path, &campaign);
        assert_eq!(editor.run().unwrap().live_info().unwrap().id, current);
        assert_eq!(fs::read(&path).unwrap(), bytes);
        editor.run_path(&template, &campaign, &[]);
        assert_eq!(editor.error.as_deref(), Some(BLOCKED_BY_RUN));
        assert_eq!(editor.run().unwrap().live_info().unwrap().id, current);
        assert!(
            editor.draft.is_some(),
            "a refused start must keep the editor"
        );
    }

    #[test]
    fn the_rail_resumes_the_newest_unfinished_run_and_keeps_a_draft() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let wolf = stat_creature(dir.path(), "Wolf", 20);
        let mut draft = Draft::new();
        draft.add(wolf, false);
        draft.save(&campaign, &[]).unwrap();
        let mut editor = Encounters {
            active: Some(campaign.clone()),
            draft: Some(draft),
            ..Default::default()
        };
        let run = editor.start_run(&campaign, &[]).unwrap();
        let path = run.path().unwrap().to_path_buf();
        editor.snapshot = Some(Snapshot::load(&campaign, &[]).unwrap());
        assert_eq!(editor.paused_run().unwrap().path, path);
        editor.draft.as_mut().unwrap().body = "Keep this draft".to_owned();
        editor.open_saved_run(&path, &campaign);
        assert!(editor.has_run());
        assert!(editor.go_to_run);
        assert!(editor.paused_run().is_none());
        assert_eq!(editor.draft.as_ref().unwrap().body, "Keep this draft");
    }

    fn slot_override(draft: &mut Draft, index: usize, text: &str) {
        draft.setup.roster[index].overrides = serde_yaml_ng::from_str(text).unwrap();
    }
    fn stat_creature(root: &std::path::Path, name: &str, hp: i64) -> Uuid {
        let id = Uuid::new_v4();
        write_note(
            root,
            &format!("{name}.md"),
            None,
            &format!("---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: creature\nsplat_name: {name}\nsplat_creature: {{}}\nsplat_stats:\n  max_hp: {{value: {hp}}}\n  armor_class: {{value: 13}}\n  initiative: {{value: 1}}\n---\n"),
        )
        .unwrap();
        id
    }
    #[test]
    fn overrides_save_with_the_slot_and_reach_the_run() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let rival = stat_creature(dir.path(), "Rival", 20);
        let creature_note = fs::read_to_string(dir.path().join("Rival.md")).unwrap();
        let mut d = Draft::new();
        d.add(rival, false);
        d.add(rival, false);
        slot_override(
            &mut d,
            0,
            "{name: Captain, max_hp: {add: 10}, attack_bonus: {add: 1}, damage_bonus: {add: 1}}",
        );
        d.setup.roster[1]
            .extra
            .insert("initial_presence".into(), "absent".into());
        d.save(&campaign, &[]).unwrap();
        let reopened = Draft::open(dir.path(), &d.path).unwrap();
        assert_eq!(reopened.setup, d.setup, "the block survives its note");
        let editor = Encounters {
            active: Some(campaign.clone()),
            draft: Some(reopened),
            ..Default::default()
        };
        let mut run = editor.start_run(&campaign, &[]).unwrap();
        let path = dir.path().join(&d.path);
        let text = fs::read_to_string(&path).unwrap();
        // The run holds what it began with. A later edit of the file changes nothing in it.
        let edited = text.replace("add: 10", "add: 99");
        assert_ne!(text, edited);
        fs::write(&path, &edited).unwrap();
        run.autosave();
        let campaign_run =
            crate::encounter_run::Run::open(run.path().unwrap(), campaign.id).unwrap();
        let strip = campaign_run.strip();
        assert_eq!(strip.name, d.name);
        assert_eq!(
            run.participant_stats(0),
            ("Captain".to_owned(), Some(30), true)
        );
        assert_eq!(
            campaign_run.participant_stats(0),
            ("Captain".to_owned(), Some(30), true)
        );
        assert_eq!(
            campaign_run.participant_stats(1),
            ("Rival".to_owned(), Some(20), false),
            "an absent slot, with no change"
        );
        // Nothing is written back.
        assert_eq!(
            fs::read_to_string(dir.path().join("Rival.md")).unwrap(),
            creature_note
        );
    }
    #[test]
    fn a_bad_override_names_its_slot_and_saves_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = stat_creature(dir.path(), "Rival", 20);
        let mut d = Draft::new();
        d.add(id, false);
        d.save(&campaign, &[]).unwrap();
        let saved = d.baseline.clone().unwrap();
        let key = d.setup.roster[0].key.clone();
        for (block, words) in [
            ("{hit_points: 3}", "unknown override"),
            ("{max_hp: {set: 0}}", "from 1 to"),
            ("{max_hp: {add: -20}}", "would become 0"),
            ("{attack_bonus: {set: 2}}", "attack_bonus"),
            ("{side: rivals}", "not a side of this encounter"),
        ] {
            slot_override(&mut d, 0, block);
            let error = d.save(&campaign, &[]).unwrap_err();
            assert!(
                error.contains(&key) && error.contains("Rival") && error.contains(words),
                "{error}"
            );
        }
        slot_override(&mut d, 0, "{max_hp: {set: 5}}");
        d.setup.roster[0].initial.insert("hp".into(), 12.into());
        let error = d.save(&campaign, &[]).unwrap_err();
        assert!(error.contains("above the changed maximum"), "{error}");
        assert_eq!(fs::read_to_string(dir.path().join(&d.path)).unwrap(), saved);
    }
    #[test]
    fn a_file_with_a_bad_override_says_which_slot_when_opened() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = stat_creature(dir.path(), "Rival", 20);
        let mut d = Draft::new();
        d.add(id, false);
        d.setup.roster[0].key = "rival_one".into();
        slot_override(&mut d, 0, "{note: Sealed}");
        d.save(&campaign, &[]).unwrap();
        let path = dir.path().join(&d.path);
        let text = fs::read_to_string(&path)
            .unwrap()
            .replace("note: Sealed", "level: 3");
        fs::write(&path, text).unwrap();
        let error = Draft::open(dir.path(), &d.path).err().unwrap();
        assert!(
            error.contains("rival_one") && error.contains("unknown override"),
            "{error}"
        );
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        assert!(
            snapshot
                .issues
                .iter()
                .any(|issue| issue.contains("rival_one"))
        );
    }
    #[test]
    fn a_side_override_uses_the_encounters_sides_and_counts_in_its_summary() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = stat_creature(dir.path(), "Rival", 20);
        let mut d = Draft::new();
        d.add(id, false);
        slot_override(&mut d, 0, "{side: party}");
        d.save(&campaign, &[]).unwrap();
        assert_eq!(
            roster_summary(&d.setup, &Snapshot::load(&campaign, &[]).unwrap().catalogue),
            "1 party · Changed"
        );
        slot_override(&mut d, 0, "{side: ' party '}");
        assert_eq!(
            roster_summary(&d.setup, &Snapshot::load(&campaign, &[]).unwrap().catalogue),
            "1 party · Changed"
        );
        slot_override(&mut d, 0, "{side: party}");
        let editor = Encounters {
            active: Some(campaign.clone()),
            draft: Some(Draft::open(dir.path(), &d.path).unwrap()),
            ..Default::default()
        };
        assert!(editor.start_run(&campaign, &[]).is_ok());
    }
    #[test]
    fn a_linked_character_cannot_override_what_its_sheet_decides() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let hero = creature(dir.path(), "Hero", true);
        let mut d = Draft::new();
        d.add(hero, true);
        for block in ["{max_hp: {set: 9}}", "{conditions: [Prone]}"] {
            slot_override(&mut d, 0, block);
            let error = d.save(&campaign, &[]).unwrap_err();
            assert!(error.contains("D&D Beyond"), "{error}");
        }
        slot_override(&mut d, 0, "{note: Sealed}");
        d.save(&campaign, &[]).unwrap();
    }
    #[test]
    fn a_changed_override_changes_the_roster_fingerprint_and_none_leaves_it_alone() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = stat_creature(dir.path(), "Rival", 20);
        let mut d = Draft::new();
        d.add(id, false);
        d.save(&campaign, &[]).unwrap();
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let digest = |setup: &Setup| {
            let slots: Vec<_> = setup
                .roster
                .iter()
                .map(|slot| {
                    let resolved = snapshot
                        .catalogue
                        .resolve_ref(slot.creature.as_ref().unwrap())
                        .unwrap();
                    let fields = serde_yaml_ng::to_value(slot).unwrap();
                    (resolved, fields.as_mapping().unwrap().clone())
                })
                .collect();
            let placements: Vec<_> = slots.iter().map(|(id, slot)| (*id, slot)).collect();
            splat::content::overrides::roster_fingerprint(&placements).unwrap()
        };
        let plain = digest(&d.setup);
        let written = serde_yaml_ng::to_string(&d.setup).unwrap();
        assert!(
            !written.contains("overrides"),
            "a slot with none writes none: {written}"
        );
        slot_override(&mut d, 0, "{max_hp: {add: 10}}");
        let first = digest(&d.setup);
        assert_ne!(plain, first);
        slot_override(&mut d, 0, "{max_hp: {add: 11}}");
        assert_ne!(first, digest(&d.setup));
        slot_override(&mut d, 0, "{name: Captain}");
        assert_ne!(plain, digest(&d.setup));
        d.setup.roster[0].overrides.clear();
        assert_eq!(plain, digest(&d.setup));
    }
    #[test]
    fn combat_sync_is_on_unless_an_encounter_says_off_and_older_notes_do_not_change() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = creature(dir.path(), "Hero", true);
        let mut draft = Draft::new();
        draft.add(id, true);
        assert!(draft.setup.combat_sync);
        draft.save(&campaign, &[]).unwrap();
        let path = dir.path().join(&draft.path);
        assert!(!fs::read_to_string(&path).unwrap().contains("combat_sync"));
        assert!(
            Draft::open(dir.path(), &draft.path)
                .unwrap()
                .setup
                .combat_sync
        );

        draft.setup.combat_sync = false;
        assert!(draft.dirty());
        draft.save(&campaign, &[]).unwrap();
        assert!(
            fs::read_to_string(&path)
                .unwrap()
                .contains("combat_sync: false")
        );
        assert!(
            !Draft::open(dir.path(), &draft.path)
                .unwrap()
                .setup
                .combat_sync
        );
    }
    #[test]
    fn a_synced_player_starts_from_the_sheet_and_every_other_slot_from_the_encounter() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let hero = creature(dir.path(), "Hero", true);
        let monster = creature(dir.path(), "Monster", false);
        let mut draft = Draft::new();
        draft.add(hero, true);
        draft.add(monster, false);
        for slot in &mut draft.setup.roster {
            slot.initial.insert("hp".into(), 7.into());
        }
        draft.save(&campaign, &[]).unwrap();
        let mut editor = Encounters {
            active: Some(campaign.clone()),
            draft: Some(draft),
            ..Default::default()
        };
        let run = editor.start_run(&campaign, &[]).unwrap();
        assert!(run.keeps_party_synced());
        assert_eq!(
            run.hp_of(0),
            Some(1),
            "the template HP stays while the setup override waits for the sheet"
        );
        assert_eq!(run.hp_of(1), Some(7));

        let draft = editor.draft.as_mut().unwrap();
        draft.setup.combat_sync = false;
        draft.save(&campaign, &[]).unwrap();
        let run = editor.start_run(&campaign, &[]).unwrap();
        assert!(!run.keeps_party_synced());
        assert_eq!(run.hp_of(0), Some(7), "with sync off, the override applies");
        assert_eq!(run.hp_of(1), Some(7));
    }
}
