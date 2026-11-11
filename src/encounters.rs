//! Campaign encounter templates. Runtime state is deliberately separate.
#[cfg(test)]
use crate::content_workspace::patch_fields;
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue, EntryRef},
    design::{self, Action, Density, Status, Tone, Type, colors, icons},
    library::{note_path, write_note},
    party::Party,
    persistence::{Campaign, Module},
};
use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_yaml_ng::{Mapping, Value};
use splat::content::{
    self, Entry,
    creature_filter::{Facets, Filter, Query},
    overrides::{Overrides, Resolved},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    sync::Arc,
    sync::mpsc,
};
use uuid::Uuid;

/// The last sentence of a deleted-entry roster fault.
const REPLACE_DELETED_ENTRY: &str = "Choose a replacement in the roster.";
/// Where the party members that Add participants offers come from.
const PARTY_SOURCE: &str = "Party members come from the sheets Splat saved last.";
/// Where an encounter that this editor cannot change is repaired.
const REPAIR_IN_ENTRIES: &str = "Repair this encounter in Campaign → Campaign entries.";
/// An encounter whose party rows the campaign fills, which this editor and the run cannot change.
const PARTY_FROM_CAMPAIGN: &str = "This encounter fills some rows from the campaign’s party. Edit those rows in Campaign → Campaign entries. This editor adds party members from the sheets Splat saved last.";

/// Why an encounter cannot be saved or run, with the tone that its kind of fault takes. The tone is
/// set where the fault is found, never read back from the words.
#[derive(Clone, PartialEq)]
struct Invalid {
    text: String,
    tone: Tone,
}

impl std::fmt::Debug for Invalid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

impl From<String> for Invalid {
    fn from(text: String) -> Self {
        Self {
            text,
            tone: Tone::Error,
        }
    }
}

impl From<&str> for Invalid {
    fn from(text: &str) -> Self {
        text.to_owned().into()
    }
}

impl From<Invalid> for String {
    fn from(invalid: Invalid) -> Self {
        invalid.text
    }
}

/// `Copy N` from a run's list label, or `None` for the original run.
fn run_copy_word(label: &str) -> Option<String> {
    label
        .split(" · ")
        .next()
        .filter(|word| word.starts_with("Copy "))
        .map(str::to_owned)
}

fn named_note_path(root: &std::path::Path, folder: &str, name: &str) -> Result<String, String> {
    let stem: String = name
        .trim()
        .chars()
        .map(|c| {
            if c.is_control() || "<>:\"/\\|?*#[]^".contains(c) {
                '-'
            } else {
                c
            }
        })
        .collect();
    let stem = stem.trim_matches([' ', '.']);
    if stem.is_empty() {
        return Err("Enter a file name.".into());
    }
    for number in 1..10000 {
        let suffix = if number == 1 {
            String::new()
        } else {
            format!(" {number}")
        };
        let path = format!("{folder}/{stem}{suffix}.md");
        if !root.join(&path).exists() {
            return Ok(path);
        }
    }
    Err("Choose another name for this file.".into())
}

mod disk;
mod groups;
mod history;
use groups::GroupDraft;
pub(crate) use history::{Template, template_of};

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
/// A table row that the parser excluded. Keep its identity beside the repair.
#[derive(Clone, Debug)]
pub(crate) struct RosterFault {
    pub name: String,
    pub line: usize,
    pub problem: String,
    /// How many read rows come before this one, so the roster shows it in its place (FC3).
    pub before: usize,
}

impl RosterFault {
    /// `<name> · line <n>: <problem>`: the one form that the encounter editor, the run setup and
    /// Save's reason give an unread roster row.
    pub(crate) fn words(&self) -> String {
        row_fault_words(&self.name, self.line, &self.problem)
    }
}

/// Where the editor leaves the roster's unread rows, for the roster to show in their places.
fn roster_fault_rows_id(draft: Uuid) -> egui::Id {
    egui::Id::new(("roster_fault_rows", draft))
}

/// A roster row's own mark for its unread row fault (FC3): short, as a row's other faults are.
fn roster_row_fault_words(line: usize) -> String {
    format!("Line {line} not read")
}

/// FC3: an unread roster row in its place, with no number (it is not in the roster Splat reads)
/// and its warning mark. The message above names its cause, and its file line the repair.
fn unread_roster_row(ui: &mut egui::Ui, fault: &RosterFault) {
    ui.push_id(("unread_roster_row", fault.line), |ui| {
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Glance.row());
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.add(
                    egui::Label::new(Type::Strong.rich(ui, &fault.name).color(colors().text))
                        .truncate(),
                );
                let mut job = egui::text::LayoutJob::default();
                for (text, color) in [
                    (format!("{} ", icons::WARN), colors().result.warning),
                    (roster_row_fault_words(fault.line), colors().muted),
                ] {
                    job.append(
                        &text,
                        0.0,
                        egui::TextFormat {
                            font_id: Type::Small.font(ui),
                            color,
                            ..Default::default()
                        },
                    );
                }
                ui.add(egui::Label::new(job).wrap());
            });
        });
        design::rule(ui);
    });
}

fn row_fault_words(name: &str, line: usize, problem: &str) -> String {
    format!("{name} · line {line}: {problem}")
}

/// Read rejected rows from a fresh parse, before catalogue reference resolution.
pub(crate) fn roster_faults(entry: &Entry, known: &BTreeMap<String, String>) -> Vec<RosterFault> {
    let Some(table) = splat::content::layout::Document::new(&entry.body)
        .table(splat::content::list_formats::ROSTER)
    else {
        return Vec::new();
    };
    let mut read = 0;
    let mut faults = Vec::new();
    for row in &table.rows {
        let Some(first) = row.first() else {
            continue;
        };
        let line = first.line + entry.body_line.saturating_sub(1);
        let Some(flag) = entry.flags.iter().find(|flag| flag.line == line) else {
            read += 1;
            continue;
        };
        // A row with missing cells is a fault too; never index past its end.
        let label = row.get(1).map_or("", |cell| cell.value.as_str());
        let target = label
            .split("%%")
            .next()
            .unwrap_or(label)
            .trim()
            .trim_start_matches("[[")
            .trim_end_matches("]]");
        let named = row.get(2).map_or("", |cell| cell.value.as_str());
        let name = if !named.is_empty() {
            named.to_owned()
        } else if target.is_empty() {
            format!("Row on line {line}")
        } else {
            known
                .get(target)
                .cloned()
                .unwrap_or_else(|| target.rsplit('|').next().unwrap_or(target).to_owned())
        };
        // A row fault names its line itself, so it needs the words without it. Rows are read
        // from the file, never from a saved copy; `unread_notes` shows any other flag.
        let Some(problem) = flag.words() else {
            continue;
        };
        faults.push(RosterFault {
            name,
            line,
            problem,
            before: read,
        });
    }
    faults
}

/// What the DM does about unread roster rows before a fight.
fn fix_roster_rows(count: usize) -> &'static str {
    if count == 1 {
        "Fix this roster row or remove it in Obsidian before starting the fight."
    } else {
        "Fix these roster rows or remove them in Obsidian before starting the fight."
    }
}

/// One warning for each unread line of an encounter file. When the file line follows (a roster
/// row is among them), the warning names the row and the line, and the file line holds the path.
/// Without a file line, the warning names the file.
fn unread_notes(
    path: &str,
    flags: &[splat::content::layout::Flag],
    faults: &[RosterFault],
) -> Vec<String> {
    flags
        .iter()
        .map(|flag| {
            let row = faults.iter().find(|fault| fault.line == flag.line);
            // A flag read back from a saved copy has only its sentence, which names its line:
            // it follows the file or the row, and no line is added.
            let Some(words) = flag.words() else {
                return match row {
                    _ if faults.is_empty() => format!("{path}: {}", flag.message),
                    Some(fault) => format!("{}: {}", fault.name, flag.message),
                    None => flag.message.clone(),
                };
            };
            match row {
                _ if faults.is_empty() => format!("{path}, line {}: {words}", flag.line),
                Some(fault) => row_fault_words(&fault.name, flag.line, &words),
                None => flag.message.clone(),
            }
        })
        .collect()
}

/// `N content issues to repair`: the words of a page's collapsing content issue count.
pub(crate) fn content_issue_count(count: usize) -> String {
    design::count(count, "content issue to repair", "content issues to repair")
}

/// A collapsing content issue count is a status, in one look on every page: the warning icon
/// and colour, then the count, at the size of a section title.
pub(crate) fn issue_count_title(ui: &egui::Ui, heading: &str) -> egui::text::LayoutJob {
    let format = egui::TextFormat {
        font_id: egui::TextStyle::Button.resolve(ui.style()),
        color: colors().result.warning,
        ..Default::default()
    };
    let mut job = egui::text::LayoutJob::default();
    job.append(icons::WARN, 0.0, format.clone());
    job.append(" ", 0.0, format.clone());
    job.append(heading, 0.0, format);
    job
}

/// A warning's file line: the path, whole, then `Edit in Obsidian` and `Show file`. A path too
/// long to share the line with its links wraps on lines of its own above them; it is never cut.
pub(crate) fn file_line(ui: &mut egui::Ui, root: &std::path::Path, relative: &str) {
    let links = crate::world::entry_file_links_width(ui, true);
    let budget = ui.available_width() - links - design::L;
    if design::number_width(ui, relative, &Type::Small.font(ui)) <= budget {
        crate::world::entry_file_actions(ui, root, relative, true);
        return;
    }
    let path = root.join(relative);
    let response =
        ui.add(egui::Label::new(Type::Small.rich(ui, relative).color(colors().muted)).wrap());
    design::named(ui, &response, &path.display().to_string());
    response.on_hover_text(path.display().to_string());
    ui.horizontal_wrapped(|ui| crate::world::entry_file_links(ui, &path, true));
}

/// The encounter file's unread rows, one warning each. The last warning says what to do, and the
/// file's path and links start at its words, as every message's actions do.
fn unread_roster_rows(
    ui: &mut egui::Ui,
    root: &std::path::Path,
    path: &str,
    notes: &[String],
    faults: usize,
) {
    let Some((last, before)) = notes.split_last() else {
        return;
    };
    for note in before {
        design::message(ui, Tone::Warning, note);
    }
    if faults == 0 {
        design::message(ui, Tone::Warning, last);
        return;
    }
    design::message_with_actions(
        ui,
        Tone::Warning,
        &format!("{last} {}", fix_roster_rows(faults)),
        |ui| file_line(ui, root, path),
    );
}

fn roster_fault_reason(path: &str, faults: &[RosterFault]) -> Option<String> {
    (!faults.is_empty()).then(|| {
        format!(
            "{}: {}. {}",
            path,
            faults
                .iter()
                .map(|fault| {
                    row_fault_words(&fault.name, fault.line, fault.problem.trim_end_matches('.'))
                })
                .collect::<Vec<_>>()
                .join("; "),
            fix_roster_rows(faults.len())
        )
    })
}

struct Draft {
    recovery: disk::Recovery,
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
    /// The note's `## Loot` list, one line each, kept apart from `body` while this owns it.
    loot: Vec<String>,
    saved_loot: Vec<String>,
    /// Whether the loot field owns the list. A section with other words in it stays in `body`,
    /// so that nothing in it is lost.
    loot_owned: bool,
    /// The heading after the note's Loot section, so a save puts the list back in its place.
    loot_before: Option<String>,
    flags: Vec<splat::content::layout::Flag>,
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
            "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: encounter\nsplat_name: New encounter\n---\n\n## Roster\n\n| Side | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n"
        );
        Self {
            recovery: Default::default(),
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
            loot: Vec::new(),
            saved_loot: Vec::new(),
            loot_owned: true,
            loot_before: None,
            flags: Vec::new(),
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
        let prose = splat::content::list_formats::prose(&entry);
        // A second Loot section, or one with more than a list, stays in the notes as written.
        let loot_owned = splat::content::loot::only_items(&prose)
            && !splat::content::loot::has_section(&splat::content::loot::without(&prose));
        let loot_before = splat::content::loot::heading_after(&prose);
        let (body, loot) = if loot_owned {
            (
                splat::content::loot::without(&prose),
                splat::content::loot::items(&prose),
            )
        } else {
            (prose, Vec::new())
        };
        Ok(Self {
            recovery: Default::default(),
            id: entry.id,
            path: path.into(),
            baseline: Some(text.clone()),
            source: text,
            saved_name: entry.name.clone(),
            name: entry.name.clone(),
            saved_setup: setup.clone(),
            setup,
            saved_body: body.clone(),
            body,
            saved_loot: loot.clone(),
            loot,
            loot_owned,
            loot_before,
            flags: entry.flags.clone(),
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
            || self.loot != self.saved_loot
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
        let disk_source = self
            .recovery
            .change
            .as_ref()
            .and_then(|change| change.text.as_deref())
            .unwrap_or(&self.source);
        let source = if self.body == self.saved_body && self.loot == self.saved_loot {
            disk_source.to_owned()
        } else {
            splat::content::list_formats::with_prose(disk_source, &self.prose())?
        };
        let text = splat::content::formats::patch_model(
            &source,
            &[
                ("splat_name", Some(self.name.clone().into())),
                (
                    "splat_encounter",
                    Some(serde_yaml_ng::to_value(&self.setup).map_err(|e| e.to_string())?),
                ),
            ],
        )?;
        Ok(text)
    }
    /// The note's own words, with the loot list at their end while the field owns it.
    fn prose(&self) -> String {
        if self.loot_owned {
            splat::content::loot::with(&self.body, &self.loot, self.loot_before.as_deref())
        } else {
            self.body.clone()
        }
    }
    fn comparison_text(&self, snapshot: &Snapshot) -> Result<String, String> {
        disk::comparison_text(&self.path, self.text()?, snapshot)
    }
    fn write_text(&self, snapshot: &Snapshot) -> Result<String, String> {
        splat::content::list_formats::write_references(
            &self.text()?,
            snapshot
                .catalogue
                .entries
                .values()
                .map(|entry| &entry.entry),
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
            overrides: Mapping::new(),
            extra: Mapping::new(),
        });
        for (key, value) in Setup::default().sides {
            self.setup.sides.entry(key).or_insert(value);
        }
    }
    fn save(&mut self, campaign: &Campaign, modules: &[Module]) -> Result<(), String> {
        if self.baseline.is_none() {
            self.path = named_note_path(&campaign.path, "Encounters", &self.name)?;
        }
        self.recovery.check(
            &campaign.path,
            &mut self.path,
            self.baseline.as_deref(),
            "encounter",
        )?;
        let snapshot = Snapshot::load(campaign, modules)?;
        self.validate(&snapshot)?;
        let text = self.write_text(&snapshot)?;
        let entry = Entry::parse(&self.path, &text)?;
        debug_assert_eq!(entry.id, self.id);
        // Nothing is written unless the encounter and every changed group can be.
        let mut group_texts = Vec::new();
        let mut reserved_paths = BTreeSet::from([self.path.clone()]);
        for (index, group) in self.groups.iter_mut().enumerate() {
            if group.dirty() {
                let old_path = group.path().to_owned();
                group.check_disk(campaign)?;
                if !group.has_file() {
                    let stem = group.path().trim_end_matches(".md").to_owned();
                    let mut number = 2;
                    while reserved_paths.contains(group.path()) {
                        group.set_new_path(format!("{stem} {number}.md"));
                        number += 1;
                    }
                }
                if self.add_to.as_deref() == Some(old_path.as_str()) {
                    self.add_to = Some(group.path().to_owned());
                }
                if self.focus_group.as_deref() == Some(old_path.as_str()) {
                    self.focus_group = Some(group.path().to_owned());
                }
                reserved_paths.insert(group.path().to_owned());
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
        let document = CampaignDocument::load(campaign)?;
        let mut changes = Vec::new();
        if document.baseline.is_none() {
            changes.push(crate::file_batch::Change::text(
                note_path(&campaign.path, &document.path)?,
                None,
                Some(&document.text),
            ));
        }
        let path = note_path(&campaign.path, &self.path)?;
        fs::create_dir_all(path.parent().ok_or("Missing encounter folder")?)
            .map_err(|e| e.to_string())?;
        // A change to a group alone leaves the encounter's note as it is.
        if self.baseline.as_deref() != Some(text.as_str()) {
            changes.push(crate::file_batch::Change::text(
                path,
                self.baseline.as_deref(),
                Some(&text),
            ));
        }
        for (index, text) in &group_texts {
            changes.push(self.groups[*index].change(campaign, Some(text))?);
        }
        for group in &self.deleted_groups {
            if group.has_file() {
                changes.push(group.change(campaign, None)?);
            }
        }
        crate::file_batch::apply(&changes)?;
        self.source = text.clone();
        self.baseline = Some(text);
        self.saved_name = self.name.clone();
        self.saved_setup = self.setup.clone();
        self.saved_body = self.body.clone();
        self.saved_loot = self.loot.clone();
        self.recovery = Default::default();
        for (index, text) in group_texts {
            let group = &mut self.groups[index];
            group.saved_text(&text);
            self.groups_written = true;
        }
        for _ in &self.deleted_groups {
            self.groups_written = true;
        }
        self.deleted_groups.clear();
        // A group the DM gave to another encounter is no longer this one's.
        let id = self.id;
        self.groups.retain(|group| group.belongs_to(id));
        self.removed_group = None;
        Ok(())
    }
    fn validate(&self, snapshot: &Snapshot) -> Result<(), Invalid> {
        #[cfg(test)]
        VALIDATIONS.with(|count| count.set(count.get() + 1));
        let entry = Entry::parse(&self.path, &self.source)?;
        if let Some(reason) = roster_fault_reason(
            &self.path,
            &roster_faults(&entry, &snapshot.known_creature_names),
        ) {
            return Err(reason.into());
        }
        if self.name.trim().is_empty() {
            return Err("Enter an encounter name.".into());
        }
        // The Loot list writes that section, so a second one typed in the notes would be lost.
        if self.loot_owned && splat::content::loot::has_section(&self.body) {
            return Err("Read-aloud and notes has a Loot heading. Put those lines in the Loot list, or rename the heading.".into());
        }
        let mut keys = BTreeSet::new();
        for (side, data) in &self.setup.sides {
            if side.as_str().is_none_or(|s| s.trim().is_empty()) {
                return Err("Sides need names.".into());
            }
            let hostile = data
                .get("hostile_to")
                .and_then(Value::as_sequence)
                .ok_or("Each side needs a list of the sides it fights.")?;
            if hostile.iter().any(|s| !self.setup.sides.contains_key(s)) {
                return Err("A hostile side is missing.".into());
            }
        }
        for (index, slot) in self.setup.roster.iter().enumerate() {
            let row = || roster_row(index, slot, snapshot);
            if slot.key.trim().is_empty() || !keys.insert(&slot.key) {
                return Err(format!(
                    "{} shares its identity with another row, or has none. {REPAIR_IN_ENTRIES}",
                    row()
                )
                .into());
            }
            if !self
                .setup
                .sides
                .contains_key(Value::from(slot.side.clone()))
            {
                return Err(format!("{}: its side is not in this encounter.", row()).into());
            }
            match (&slot.creature, &slot.party_slot) {
                (Some(reference), None) => {
                    let id = snapshot.catalogue.resolve_ref(reference).map_err(|_| {
                        let count = self.setup.roster.iter().filter(|other| other.creature.as_ref() == Some(reference)).count();
                        let mut named = slot.clone();
                        named.overrides.remove("name");
                        let name = roster_name(&named, None, &snapshot.known_creature_names);
                        // The kind of fault sets the tone, here where the message is made.
                        let unreadable = snapshot.catalogue.unreadable_ref(reference);
                        Invalid {
                            tone: missing_creature_tone(unreadable),
                            text: unreadable.map_or_else(|| if count > 1 {
                                format!("{count} {name} rows have no creature. Their package entry was deleted. {REPLACE_DELETED_ENTRY}")
                            } else {
                                format!("{name} has no creature. Its package entry was deleted. {REPLACE_DELETED_ENTRY}")
                            },
                                |note| format!("{}. Open in {} to fix it.", note.word, if crate::world::repair_in_world(&note.path) { "World" } else { "Library" })),
                        }
                    })?;
                    if !snapshot.creatures.iter().any(|c| c.id == id) {
                        return Err(format!(
                            "{}: this creature is unavailable in campaign content.",
                            row()
                        )
                        .into());
                    }
                    self.validate_overrides(&row(), slot, &snapshot.catalogue.entries[&id].entry)?;
                }
                (None, Some(_)) => return Err(PARTY_FROM_CAMPAIGN.into()),
                _ => {
                    return Err(format!("{} needs one creature or one party member.", row()).into());
                }
            }
            for (field, words) in [("hp", "HP"), ("temporary_hp", "Temporary HP")] {
                if slot
                    .initial
                    .get(field)
                    .is_some_and(|v| v.as_u64().is_none())
                {
                    return Err(format!(
                        "{}: its starting {words} must be a whole number, 0 or more.",
                        row()
                    )
                    .into());
                }
            }
        }
        Ok(())
    }
    /// A change to a stat must leave it valid for this slot's creature and starting state.
    fn validate_overrides(&self, row: &str, slot: &Slot, entry: &Entry) -> Result<(), String> {
        let named = |error: String| format!("{row}: {error}");
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
#[cfg(test)]
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
    recovery: disk::Recovery,
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
            recovery: Default::default(),
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
        Entry::parse(&self.path, &self.baseline)?;
        {
            let text = if *body == self.saved.body {
                self.baseline.clone()
            } else {
                with_body(&self.baseline, body)
            };
            let mut stats = Entry::parse(&self.path, &text)?.fields["splat_stats"].clone();
            for (key, changed, value) in [
                ("max_hp", *hp != self.saved.hp, new_hp),
                (
                    "armor_class",
                    *armor_class != self.saved.armor_class,
                    new_armor_class,
                ),
            ] {
                if changed {
                    stats
                        .as_mapping_mut()
                        .ok_or("Missing stats")?
                        .insert(key.into(), yaml(&format!("{{value: {value}}}")));
                }
            }
            let mut changes = vec![("splat_stats", Some(stats))];
            if *name != self.saved.name {
                changes.push(("splat_name", Some(name.trim().into())));
            }
            content::formats::patch_model(&text, &changes)
        }
    }
    fn save(&mut self, root: &std::path::Path) -> Result<(), String> {
        self.recovery
            .check(root, &mut self.path, Some(&self.baseline), "creature")?;
        let text = self.text()?;
        Entry::parse(&self.path, &text)?;
        write_note(root, &self.path, Some(&self.baseline), &text)?;
        self.fields.body = Entry::parse(&self.path, &text)?.body.trim().to_owned();
        self.baseline = text;
        self.saved = self.fields.clone();
        self.recovery = Default::default();
        Ok(())
    }
}
struct Creature {
    unavailable: Option<String>,
    id: Uuid,
    name: String,
    lower: String,
    /// CR, HP and AC, from fields the entry already has.
    detail: String,
    /// The stat block has no hit points, so `detail` shows the `HP not set` fault.
    hp_missing: bool,
    /// The package it came from. A party copy says so in `detail` instead.
    source: Option<String>,
    party: bool,
    origin: crate::lookup::Origin,
    /// A character whose sheet is on D&D Beyond. A sheet cannot be two participants.
    linked: bool,
    /// The D&D Beyond character a party copy is of.
    character: Option<String>,
    /// Habitat, type, size and the other facts that search and the filter use.
    facets: Option<Facets>,
}
const ROSTER_MAX_WIDTH: f32 = design::LIST_MEASURE;
const ADD_PANE: &str = "encounter_add";
const ADD_PANE_MAX_WIDTH: f32 = 640.0;
const RESULT_ROW: f32 = design::XL + design::L + design::S;
/// A result's stats, each in its own place, so that they can be compared down the list. The
/// first place holds a party copy's word, which has no challenge rating. The package follows
/// if the row has room for it.
/// `hp_missing` comes from the creature's stats: the HP fault is data, never found in the words.
pub(crate) fn detail_columns(
    ui: &mut egui::Ui,
    detail: &str,
    source: Option<&str>,
    hp_missing: bool,
) {
    if hp_missing {
        ui.horizontal_wrapped(|ui| {
            for part in detail_parts(detail, hp_missing) {
                if part.hp_fault {
                    design::status(
                        ui,
                        design::Status {
                            icon: design::icons::WARN,
                            word: HP_NOT_SET,
                            color: part.color(),
                        },
                    );
                } else {
                    ui.label(Type::Small.rich(ui, part.words).color(part.color()));
                }
            }
        });
        return;
    }
    const COLUMNS: [(&str, &str); 3] = [("CR ", "CR 1/8"), ("HP ", "HP 999"), ("AC ", "AC 25")];
    let parts: Vec<&str> = detail.split(" · ").collect();
    let other = parts
        .iter()
        .find(|part| !COLUMNS.iter().any(|(prefix, _)| part.starts_with(prefix)));
    // A row of plain words: it sits close under its name, not at a control's height.
    ui.spacing_mut().interact_size.y = 0.0;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for (prefix, widest) in COLUMNS {
            let gap = if prefix == "CR " {
                design::M
            } else {
                design::S
            };
            let width = design::number_width(ui, widest, &Type::Small.font(ui)) + gap;
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
            ui.add_space(design::M - design::S);
            ui.label(
                Type::Small
                    .rich(ui, parts[..2].join(" · "))
                    .color(colors().muted),
            );
        }
        // Only whole: a cut name says nothing, and Read always shows it.
        if let Some(source) = source
            && design::number_width(ui, source, &Type::Small.font(ui)) + design::M - design::S
                <= ui.available_width()
        {
            ui.add_space(design::M - design::S);
            ui.label(Type::Small.rich(ui, source).color(colors().muted));
        }
    });
}

fn editor_save_state(
    ui: &mut egui::Ui,
    state: Status<'_>,
    dirty: bool,
    blocked: Option<&str>,
) -> bool {
    ui.set_min_height(Density::Read.control());
    design::status(ui, state);
    dirty
        && Action::quiet("Discard")
            .density(Density::Read)
            .blocked(blocked)
            .show(ui)
            .clicked()
}

/// The width a vertical scroll area's content keeps whether or not its bar shows: the room
/// before the area, less the bar's lane. Content set to this width does not move when a list
/// grows long enough to scroll.
pub(super) fn scroll_lane(ui: &egui::Ui) -> f32 {
    (ui.available_width() - ui.spacing().scroll.allocated_width()).max(0.0)
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
/// The creature's hit points, after its changes. None is the `HP not set` fault.
pub(crate) fn max_hp(entry: &Entry, changed: Option<&Resolved>) -> Option<i64> {
    changed.map_or_else(
        || {
            entry
                .fields
                .get("splat_stats")?
                .get("max_hp")?
                .get("value")?
                .as_i64()
        },
        |changed| changed.max_hp,
    )
}

fn creature_detail(entry: &Entry, party: bool, changed: Option<&Resolved>) -> String {
    let stat = |name: &str| {
        entry
            .fields
            .get("splat_stats")?
            .get(name)?
            .get("value")?
            .as_i64()
    };
    let cr = crate::library::presentation::challenge_text(entry)
        .and_then(|text| text.split_whitespace().next());
    let mut parts = Vec::new();
    if party {
        parts.push("Party copy".to_owned());
    }
    if let Some(cr) = cr {
        parts.push(format!("CR {cr}"));
    }
    if let Some(hp) = max_hp(entry, changed) {
        parts.push(format!("HP {hp}"));
    } else {
        parts.push(format!("{} {HP_NOT_SET}", design::icons::WARN));
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
    // Loot alone does not change a creature.
    if setup.roster.iter().any(|slot| {
        Overrides::parse(&slot.overrides)
            .map_or(!slot.overrides.is_empty(), |o| o.changes_creature())
    }) {
        parts.push("Changed".to_owned());
    }
    parts.join(" · ")
}
/// Keep a stat label with its value. A second line can break between facts.
/// The detail that fits before the change link, and whether it still holds the HP fault. The fault
/// is shortened as late as the hit points it stands for.
fn changed_detail_parts(
    ui: &egui::Ui,
    detail: &str,
    word: &str,
    width: f32,
    hp_missing: bool,
) -> (String, bool) {
    let font = Type::Small.font(ui);
    let reserved = design::number_width(ui, word, &font) + design::number_width(ui, " · ", &font);
    let width = (width - reserved).max(0.0);
    let mut parts = detail_parts(detail, hp_missing);
    let kept = |parts: &[DetailPart<'_>]| parts.iter().any(|part| part.hp_fault);
    if design::number_width(ui, detail, &font) <= width {
        return (detail.into(), kept(&parts));
    }
    while !parts.is_empty() {
        let priority = |part: &DetailPart<'_>| {
            let words = part.words;
            if words.starts_with("as ") {
                6
            } else if part.hp_fault || words.starts_with("HP ") {
                4
            } else if !(["CR ", "AC "]
                .iter()
                .any(|prefix| words.starts_with(prefix)))
                && !["Enemies", "Party", "Rivals"].contains(&words)
            {
                1
            } else if words.starts_with("CR ") {
                2
            } else if words.starts_with("AC ") {
                3
            } else {
                5
            }
        };
        let remove = parts
            .iter()
            .enumerate()
            .min_by_key(|(at, part)| if *at == 0 { 5 } else { priority(part) })
            .map(|(at, _)| at)
            .unwrap();
        parts.remove(remove);
        let text = parts
            .iter()
            .map(|part| part.words)
            .collect::<Vec<_>>()
            .join(" · ");
        if design::number_width(ui, &text, &font) <= width {
            return (text, kept(&parts));
        }
    }
    (String::new(), false)
}

/// The detail part of a creature without hit points.
const HP_NOT_SET: &str = "HP not set";

/// One part of a creature's detail line, and whether it is the `HP not set` fault.
pub(crate) struct DetailPart<'a> {
    pub(crate) words: &'a str,
    pub(crate) hp_fault: bool,
}

impl DetailPart<'_> {
    /// The fault has the warning status colour, icon and words; every other part is muted.
    pub(crate) fn color(&self) -> egui::Color32 {
        if self.hp_fault {
            colors().result.warning
        } else {
            colors().muted
        }
    }
}

/// The parts of a detail line. The fault comes from the creature's data (`hp_missing`), not from
/// the words: only then is the HP slot marked. `creature_detail` writes that slot before any name
/// the DM chose (a package, an `as` name), so it is the first part with the fault's text.
pub(crate) fn detail_parts(detail: &str, hp_missing: bool) -> Vec<DetailPart<'_>> {
    let fault = format!("{} {HP_NOT_SET}", design::icons::WARN);
    let kept = stat_line(&fault);
    let mut marked = !hp_missing;
    detail
        .split(" · ")
        .map(|words| {
            let hp_fault = !marked && (words == fault || words == kept);
            marked |= hp_fault;
            DetailPart { words, hp_fault }
        })
        .collect()
}

/// A detail line as one label: muted, with the HP fault part, if any, in the warning colour.
/// Without the fault it is drawn exactly as before.
pub(crate) fn detail_text(ui: &egui::Ui, detail: &str, hp_missing: bool) -> egui::WidgetText {
    let parts = detail_parts(detail, hp_missing);
    if !parts.iter().any(|part| part.hp_fault) {
        return Type::Small.rich(ui, detail).color(colors().muted).into();
    }
    let mut job = egui::text::LayoutJob::default();
    for (at, part) in parts.iter().enumerate() {
        let format = |color| egui::TextFormat {
            font_id: Type::Small.font(ui),
            color,
            ..Default::default()
        };
        if at > 0 {
            job.append(" · ", 0.0, format(colors().muted));
        }
        job.append(part.words, 0.0, format(part.color()));
    }
    job.into()
}

fn stat_line(text: &str) -> String {
    text.split(" · ")
        .map(|part| part.replace(' ', "\u{a0}"))
        .collect::<Vec<_>>()
        .join(" · ")
}

const MISSING_CREATURE: &str = "Not found. Its package entry was deleted.";

/// The tone of a roster row whose creature is not there, and of the message that states the same
/// fault. The tone comes from the fault's kind: a file that cannot be read is an error; a deleted
/// package entry and two files that claim one identity are warnings, as Library shows them.
fn missing_creature_tone(unreadable: Option<&crate::content_workspace::UnreadableNote>) -> Tone {
    if unreadable.is_some_and(|note| !note.duplicate) {
        Tone::Error
    } else {
        Tone::Warning
    }
}

/// The icon and colour of that tone inside the row's muted words, as `design::message` draws it.
fn missing_creature_mark(
    unreadable: Option<&crate::content_workspace::UnreadableNote>,
) -> (&'static str, egui::Color32) {
    match missing_creature_tone(unreadable) {
        Tone::Error => (icons::ERROR, colors().result.error),
        _ => (icons::WARN, colors().result.warning),
    }
}

/// A DM's notes in an editor. Every notes field has the same inner margin, and its frame ends at
/// the right edge of the column it is in.
fn notes_field(text: &mut String, rows: usize) -> egui::TextEdit<'_> {
    egui::TextEdit::multiline(text)
        .desired_rows(rows)
        // The width holds the margins, so the frame ends at the column's right edge.
        .desired_width(f32::INFINITY)
        .margin(egui::vec2(design::S, design::S))
}

fn roster_name(
    slot: &Slot,
    definition: Option<&Entry>,
    known: &BTreeMap<String, String>,
) -> String {
    let name = definition
        .map(|entry| entry.name.clone())
        .or_else(|| {
            slot.creature
                .as_ref()
                .and_then(|reference| known.get(&reference.label()))
                .cloned()
        })
        .unwrap_or_else(|| "Missing creature".into());
    groups::shown_name(&slot.overrides, name)
}

/// A roster row as the DM sees it in a message: its place in the roster and its shown name, never
/// its stored key.
fn roster_row(index: usize, slot: &Slot, snapshot: &Snapshot) -> String {
    let definition = slot
        .creature
        .as_ref()
        .and_then(|reference| snapshot.catalogue.resolve_ref(reference).ok())
        .and_then(|id| snapshot.catalogue.entries.get(&id))
        .map(|indexed| &indexed.entry);
    format!(
        "Row {} ({})",
        index + 1,
        roster_name(slot, definition, &snapshot.known_creature_names)
    )
}

fn creature_result_row(
    ui: &mut egui::Ui,
    c: &Creature,
    action: &str,
    already: bool,
) -> (bool, bool) {
    let (mut chosen, mut read) = (false, false);
    ui.horizontal(|ui| {
        ui.set_min_height(RESULT_ROW);
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            let controls = if c.unavailable.is_some() {
                Action::secondary(action)
                    .density(Density::Read)
                    .natural_width(ui)
                    + Action::secondary("Open in World")
                        .density(Density::Read)
                        .natural_width(ui)
                    + design::M
                    // The gaps between the words and both buttons: the results keep the scroll
                    // bar's lane (FH3), so nothing may spill into it.
                    + 2.0 * ui.spacing().item_spacing.x
            } else {
                design::S * 12.0
            };
            ui.set_max_width((ui.available_width() - controls).max(design::S * 10.0));
            read = creature_name_link(ui, &c.name)
                .on_hover_text("Read this entry in Lookup")
                .clicked();
            if let Some(reason) = &c.unavailable {
                // A status is an icon, a word and a colour, like the other warnings here. It
                // wraps, because a warning must not be cut to an ellipsis at 960.
                design::wrapped_status(
                    ui,
                    design::Status {
                        icon: icons::WARN,
                        word: reason,
                        color: colors().result.warning,
                    },
                );
            } else {
                detail_columns(ui, &c.detail, c.source.as_deref(), c.hp_missing);
            }
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if already {
                ui.label(Type::Small.rich(ui, "In roster").color(colors().muted));
            } else {
                let name = if action == "Add" {
                    action.into()
                } else {
                    format!("{action} {}", c.name)
                };
                chosen = Action::secondary(action)
                    .name(&name)
                    .density(Density::Read)
                    .blocked(c.unavailable.as_deref())
                    .show(ui)
                    .clicked();
                if c.unavailable.is_some()
                    && Action::secondary("Open in World")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                {
                    ui.ctx().data_mut(|data| {
                        data.insert_temp(egui::Id::new("reveal_encounter_npc"), c.id)
                    });
                }
            }
        });
    });
    (chosen, read)
}

/// A copied note is still a search result, with the same repair used by Lookup.
fn duplicate_creature_results(ui: &mut egui::Ui, query: &str) -> bool {
    let Some(index) = ui.ctx().data(|data| {
        data.get_temp::<Arc<crate::lookup::ContentIndex>>(egui::Id::new("reader_link_index"))
    }) else {
        return false;
    };
    let Some(campaign) = &index.campaign else {
        return false;
    };
    let undo_key = egui::Id::new(("encounter_identity_undo", campaign.id));
    let undo = ui.data(|data| data.get_temp::<crate::world::Change>(undo_key));
    if let Some(undo) = undo
        && design::undo_message(
            ui,
            Tone::Success,
            undo.message(),
            undo.identity_undo_label(),
        )
    {
        match undo.undo(campaign) {
            Ok(()) => {
                ui.data_mut(|data| {
                    data.remove::<crate::world::Change>(undo_key);
                    data.insert_temp(egui::Id::new("encounter_identity_changed"), true);
                });
            }
            Err(error) => {
                design::message(ui, Tone::Warning, &error);
            }
        }
    }
    let mut seen = BTreeSet::new();
    for item in &index.items {
        if !item.entry.name.to_lowercase().contains(query)
            || !(item.entry.kind == "creature"
                || (item.entry.kind == "world" && crate::world::fights(&item.entry)))
        {
            continue;
        }
        let Some(paths) = index.duplicate_world.get(&item.entry.path) else {
            continue;
        };
        let mut identity = paths.clone();
        identity.sort();
        if !seen.insert(identity) {
            continue;
        }
        let users: Vec<_> = index
            .items
            .iter()
            .filter(|other| {
                other.entry.kind == "encounter"
                    && crate::library::encounter_uses_entry(&other.entry, &item.entry)
            })
            .map(|other| other.entry.name.clone())
            .collect();
        let noun = crate::world::IdentityNoun::of(&item.entry);
        design::message_with_actions(ui, Tone::Warning, noun.duplicate_warning(), |ui| {
            for (position, path) in paths.iter().enumerate() {
                let filename = path.rsplit('/').next().unwrap_or(path);
                let action = noun.action(filename);
                if Action::secondary(&action).hint(&action).show(ui).clicked() {
                    match crate::world::new_identity(campaign, path) {
                        Ok((undo, _)) => ui.data_mut(|data| {
                            data.insert_temp(undo_key, undo);
                            data.insert_temp(egui::Id::new("encounter_identity_changed"), true);
                        }),
                        Err(error) => {
                            design::message(ui, Tone::Warning, &error);
                        }
                    }
                }
                if let Some(remaining) = paths.iter().find(|other| *other != path) {
                    crate::library::duplicate_repair_consequence(
                        ui,
                        &users,
                        remaining.rsplit('/').next().unwrap_or(remaining),
                        position == 0,
                    );
                }
                ui.add_space(design::XS);
            }
        });
    }
    !seen.is_empty()
}

fn replace_missing_creature(
    ui: &mut egui::Ui,
    snapshot: &Snapshot,
    count: usize,
    name: &str,
) -> Option<(Uuid, bool)> {
    let key = ui.id().with("replacement_creature");
    let mut search = ui
        .ctx()
        .data_mut(|data| data.get_temp::<String>(key))
        .unwrap_or_default();
    design::search_field(
        ui,
        "Replacement creature",
        &mut search,
        "Choose another creature…",
        ui.available_width(),
    );
    let all_key = key.with("all");
    let mut all = ui
        .ctx()
        .data_mut(|data| data.get_temp::<bool>(all_key))
        .unwrap_or(true);
    if count > 1 {
        ui.checkbox(&mut all, format!("Replace all {count} {name} rows"));
    }
    let query = search.to_lowercase();
    let mut replaced = None;
    for creature in snapshot
        .creatures
        .iter()
        .filter(|c| !c.linked && c.lower.contains(&query))
        .take(5)
    {
        ui.push_id(creature.id, |ui| {
            let (chosen, read) = creature_result_row(ui, creature, "Use", false);
            if chosen {
                replaced = Some((creature.id, all));
            }
            if read {
                ui.ctx().data_mut(|data| {
                    data.insert_temp(
                        egui::Id::new("markdown_clicked_lookup"),
                        creature.name.clone(),
                    )
                });
            }
        });
    }
    ui.ctx().data_mut(|data| {
        data.insert_temp(key, search);
        data.insert_temp(all_key, all);
    });
    replaced
}

// Group members use the same picker without the roster's bulk action.
fn choose_replacement_creature(ui: &mut egui::Ui, snapshot: &Snapshot) -> Option<EntryRef> {
    replace_missing_creature(ui, snapshot, 1, "").map(|(id, _)| EntryRef::id(id))
}

#[derive(Clone)]
struct ReplacementUndo {
    before: Vec<Slot>,
    after: Vec<Slot>,
    /// The result in words, such as `2 Cinder Wolf rows replaced with Guard.`
    words: String,
}

/// The replacement's result for its Undo message.
fn replacement_words(old: &str, rows: usize, new: &str) -> String {
    if rows > 1 {
        format!("{rows} {old} rows replaced with {new}.")
    } else {
        format!("{old} replaced with {new}.")
    }
}

fn replace_roster(
    roster: &mut [Slot],
    selected: usize,
    creature: Uuid,
    all: bool,
    snapshot: &Snapshot,
) {
    let missing = roster[selected].creature.clone();
    for (i, slot) in roster.iter_mut().enumerate() {
        if i == selected || (all && slot.creature == missing) {
            let old_name = roster_name(slot, None, &snapshot.known_creature_names);
            slot.creature = Some(EntryRef::id(creature));
            slot.party_slot = None;
            slot.overrides.insert("name".into(), old_name.into());
        }
    }
}

struct SearchRoster<'a> {
    already: &'a dyn Fn(&Creature) -> bool,
}

struct Snapshot {
    revision: Uuid,
    campaign_root: std::path::PathBuf,
    catalogue: Catalogue,
    known_creature_names: BTreeMap<String, String>,
    creatures: Vec<Creature>,
    templates: Vec<Template>,
    issues: Vec<String>,
    options: crate::encounter_settings::Options,
    saved_options: crate::encounter_settings::Options,
    campaign_text: String,
    runs: Vec<crate::encounter_run::SavedRunInfo>,
    /// AB1: the run list could not be read, and an issue says why. In progress and History then
    /// say they cannot be read instead of their empty states. It ends when a read succeeds.
    runs_unread: bool,
    /// The run folder is not there and nothing names a run, so it may never have been made. The
    /// page turns this into `runs_unread` when it has seen this campaign's folder hold runs.
    runs_missing: Option<String>,
    groups: Arc<Vec<Arc<crate::encounter_run::RunGroup>>>,
    run_creatures: Arc<Vec<Arc<crate::encounter_run::RunCreature>>>,
}
/// Load what the Encounters page shows, for the load benchmark.
#[cfg(test)]
pub(crate) fn load_for_bench(campaign: &Campaign, modules: &[Module]) -> Result<(), String> {
    Snapshot::load(campaign, modules).map(drop)
}

impl Snapshot {
    /// A refresh keeps settings the DM is editing. Otherwise the file decides, or the next
    /// save would write an old value over a change made outside Splat.
    fn keep_edited_settings(&mut self, old: &Self) {
        for (reference, name) in &old.known_creature_names {
            self.known_creature_names
                .entry(reference.clone())
                .or_insert_with(|| name.clone());
        }
        if old.options != old.saved_options {
            self.options = old.options.clone();
            self.saved_options = old.saved_options.clone();
        }
    }
    fn npc_stat_block(&self, id: Uuid) -> Option<(Uuid, String)> {
        let npc = &self.catalogue.entries.get(&id)?.entry;
        if npc.kind != "world" || !crate::world::fights(npc) {
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
    fn already_on_roster(&self, draft: &Draft, creature: &Creature) -> bool {
        creature.party
            && draft.setup.roster.iter().any(|slot| {
                slot.creature.as_ref().and_then(|reference| reference.id) == Some(creature.id)
            })
            || self
                .npc_stat_block(creature.id)
                .is_some_and(|(stat, name)| {
                    draft.setup.roster.iter().any(|slot| {
                        slot.creature.as_ref().and_then(|reference| reference.id) == Some(stat)
                            && slot.overrides.get("name").and_then(Value::as_str)
                                == Some(name.as_str())
                    })
                })
    }
    fn npc_name_for(&self, entry: &Entry, overrides: &Mapping) -> Option<String> {
        let name = overrides.get("name")?.as_str()?;
        self.catalogue
            .entries
            .values()
            .chain(&self.catalogue.duplicate_world)
            .find_map(|candidate| {
                let npc = &candidate.entry;
                if npc.kind != "world" || npc.name != name {
                    return None;
                }
                let world = npc.fields.get("splat_world")?;
                if world.get("type")?.as_str()? != "npc" {
                    return None;
                }
                let reference: EntryRef =
                    serde_yaml_ng::from_value(world.get("creature")?.clone()).ok()?;
                (self.catalogue.resolve_ref(&reference).ok()? == entry.id).then(|| npc.name.clone())
            })
    }
    fn load(campaign: &Campaign, modules: &[Module]) -> Result<Self, String> {
        let document = CampaignDocument::load_with_modules(campaign, modules)?;
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
                    unavailable: None,
                    id: *id,
                    name: e.entry.name.clone(),
                    lower: e.entry.name.to_lowercase(),
                    facets: Facets::of(&e.entry),
                    detail: creature_detail(&e.entry, party, None),
                    hp_missing: max_hp(&e.entry, None).is_none(),
                    origin: crate::lookup::entry_origin(Some(&e.entry), e.private),
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
                if (npc.kind != "world" || !crate::world::fights(npc))
                    || !(entry.private && entry.owner == campaign.id
                        || closure.direct.contains(&npc.id)
                        || closure.required.contains_key(&npc.id))
                {
                    return None;
                }
                let value = npc.fields.get("splat_world")?.get("creature")?;
                let reference: EntryRef = serde_yaml_ng::from_value(value.clone()).ok()?;
                let stat = catalogue
                    .resolve_ref(&reference)
                    .ok()
                    .and_then(|id| creatures.iter().find(|c| c.id == id && !c.linked));
                let missing_name = reference
                    .link
                    .as_deref()
                    .map(crate::sessions::plain_links)
                    .filter(|name| !name.is_empty())
                    .unwrap_or_else(|| "This NPC’s".into());
                let unavailable = stat
                    .is_none()
                    .then(|| format!("{missing_name} stat block not found"));
                Some(Creature {
                    unavailable: unavailable.clone(),
                    id: npc.id,
                    name: npc.name.clone(),
                    lower: npc.name.to_lowercase(),
                    detail: stat.map_or_else(
                        || unavailable.clone().unwrap_or_default(),
                        |stat| format!("NPC · as {} · {}", stat.name, stat.detail),
                    ),
                    hp_missing: stat.is_some_and(|stat| stat.hp_missing),
                    facets: stat.and_then(|stat| stat.facets.clone()),
                    origin: crate::lookup::entry_origin(Some(npc), entry.private),
                    source: None,
                    party: false,
                    linked: false,
                    character: None,
                })
            })
            .collect();
        creatures.extend(npcs);
        creatures.sort_by_key(|c| (!c.party, c.name.to_lowercase(), c.origin, c.id));
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
        let mut issues: Vec<String> = catalogue
            .errors
            .iter()
            .chain(&closure.errors)
            .cloned()
            .collect();
        let options = crate::encounter_settings::Options::load(campaign, &catalogue)?;
        let directory = options.directory(campaign)?;
        let mut runs_missing = None;
        let (runs, runs_unread) = match crate::encounter_run::Run::list(&directory, campaign.id) {
            Ok(runs) => (runs, false),
            Err(error) => {
                // A folder that was never made holds no runs yet. One the timeline names, or
                // one that is there but cannot be read, is a fault the lists must not hide.
                let folder = crate::sessions::folder(campaign).ok();
                let timeline_names_runs = folder
                    .and_then(|folder| fs::read_to_string(folder.join("Timeline.md")).ok())
                    .is_some_and(|text| {
                        text.lines()
                            .any(|line| line.trim_start().starts_with("run:"))
                    });
                let unread = timeline_names_runs || directory.exists();
                if unread {
                    issues.push(error);
                } else {
                    runs_missing = Some(error);
                }
                (Vec::new(), unread)
            }
        };
        let groups = Arc::new(groups::load(&catalogue, &creatures, campaign.id));
        let run_creatures = Arc::new(
            creatures
                .iter()
                // An NPC whose stat block is gone stays out of the live run's Add creature.
                .filter(|creature| !creature.linked && creature.unavailable.is_none())
                .map(|creature| {
                    Arc::new(crate::encounter_run::RunCreature {
                        entry: Arc::new(catalogue.run_entry(creature.id)),
                        lower: creature.lower.clone(),
                        detail: crate::library::presentation::list_subtitle(
                            &creature.detail,
                            creature.source.as_deref(),
                        ),
                        hp_missing: creature.hp_missing,
                    })
                })
                .collect(),
        );
        let mut known_creature_names = BTreeMap::new();
        for creature in &creatures {
            known_creature_names.insert(creature.id.to_string(), creature.name.clone());
            if let Some(key) = catalogue
                .entries
                .get(&creature.id)
                .and_then(|e| e.entry.key.as_ref())
            {
                known_creature_names.insert(key.clone(), creature.name.clone());
            }
        }
        Ok(Self {
            revision: Uuid::new_v4(),
            campaign_root: campaign.path.clone(),
            groups,
            known_creature_names,
            run_creatures,
            options: options.clone(),
            saved_options: options,
            campaign_text: document.text,
            runs,
            runs_unread,
            runs_missing,
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
    /// The campaign whose run folder this page has seen hold runs, or fail to read (AB1).
    runs_known: Option<Uuid>,
    error: Option<String>,
    /// The last failed check of the open draft, with its tone. It sets the message's tone only
    /// while `error` shows its words.
    validation_error: Option<Invalid>,
    validation_path: Option<String>,
    /// A Save or a start was refused for the draft at `validation_path`, so its check runs again
    /// when that draft or the campaign changes. It ends with another draft.
    refusal_watch: bool,
    /// Bumped on each frame that can change a draft (an editing input) and the frame after it,
    /// because a click's change lands late in its frame.
    edit_epoch: u64,
    edited_last_frame: bool,
    /// The snapshot revision, edit epoch and draft kind of the last check. An unchanged frame
    /// with the same key does not check the draft again.
    validated: Option<(Uuid, u64, bool)>,
    /// How often frames reached that key's guard.
    #[cfg(test)]
    validation_reached: usize,
    message: Option<String>,
    /// The draft `Discard changes` last dropped, for one step of undo.
    discarded: Option<Draft>,
    discarded_options: Option<crate::encounter_settings::Options>,
    pub saved: bool,
    run: Option<crate::encounter_run::Run>,
    /// The encounter editor's last frame found that Save would refuse its changed draft, so
    /// Ctrl+S is blocked as Save is. The editor sets it each frame it draws.
    save_refused: bool,
    /// History is a read-only snapshot and never owns the open-run slot.
    history_read: Option<(std::path::PathBuf, crate::encounter_run::FightSummary)>,
    /// Why the fight read in History cannot go on (`continue_blocked`), when it cannot.
    history_blocked: Option<&'static str>,
    /// What the rule's answers were worked out from: the Timeline's count of changes, the
    /// open run if it is ended, and the ID of the fight read in History. Not its path: another
    /// run can take the file at that path.
    continue_key: Option<(u64, Option<uuid::Uuid>, Option<uuid::Uuid>)>,
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
    pub wants_run_folder: bool,
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
    ContinueFight(std::path::PathBuf),
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
const RELOAD: &str = "Reload from disk";
const RELOAD_HINT: &str = "Read encounters and runs from disk again";
const NAME_FIELD: &str = "Encounter name";
const SEARCH_FIELD: &str = "Find a creature or party member";
const SYNC_OPTION: &str = "Sync party with D&D Beyond";
const REPEATABLE_OPTION: &str = "Keep in Ready during a run";
const REPEATABLE_HELP: &str = "Lists this encounter under Ready to run while a fight from it is unfinished. Start fight waits until no fight is open.";
const LOOT_FIELD: &str = "Loot";
const ENCOUNTER_LOOT_HELP: &str =
    "What the party finds when this fight ends. A creature’s own loot is in its Changes.";
#[cfg(test)]
thread_local! {
    /// How often this thread ran an encounter draft's full check.
    static VALIDATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Whether this frame has an input that can change a draft: a key, text, a press, a wheel, a
/// held pointer (a drag), a dropped file or an accessibility action. Hover alone cannot.
fn edit_input(ctx: &egui::Context) -> bool {
    ctx.input(|i| {
        i.pointer.any_down()
            || !i.raw.dropped_files.is_empty()
            || i.events.iter().any(|event| {
                matches!(
                    event,
                    egui::Event::Key { .. }
                        | egui::Event::Text(_)
                        | egui::Event::Paste(_)
                        | egui::Event::Cut
                        | egui::Event::Ime(_)
                        | egui::Event::PointerButton { .. }
                        | egui::Event::MouseWheel { .. }
                        | egui::Event::AccessKitActionRequest(_)
                )
            })
    })
}

const BLOCKED_BY_RUN: &str = "Close the open run first.";
const BLOCKED_BY_DRAFT: &str = "Save or discard the open encounter first.";
const BLOCKED_BY_CREATURE: &str = "Save or discard the changes to this creature first.";
impl Encounters {
    pub fn refocus_draft(&mut self) {
        self.focus_name = true;
    }
    /// Check a refused draft again, only when it or the campaign may have changed. `edited` says
    /// whether this frame had an editing input. A check that passes clears the stored refusal;
    /// the watch stays, so an Undo that brings the fault back shows its reason again.
    fn refresh_validation_error(&mut self, group: bool, edited: bool) {
        let edited_before = std::mem::replace(&mut self.edited_last_frame, edited);
        if edited || edited_before {
            self.edit_epoch += 1;
        }
        let path = if group {
            self.group_draft.as_ref().map(GroupDraft::path)
        } else {
            self.draft.as_ref().map(|draft| draft.path.as_str())
        };
        if path != self.validation_path.as_deref() {
            self.validation_error = None;
            self.refusal_watch = false;
            self.validated = None;
            return;
        }
        if !self.refusal_watch || (self.error.is_some() && !self.shows_validation_error()) {
            return;
        }
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        #[cfg(test)]
        {
            self.validation_reached += 1;
        }
        let key = (snapshot.revision, self.edit_epoch, group);
        if self.validated == Some(key) {
            return;
        }
        self.validated = Some(key);
        let invalid = if group {
            self.group_draft
                .as_ref()
                .and_then(|draft| draft.validate(snapshot, None).err())
                .map(Invalid::from)
        } else {
            self.draft
                .as_ref()
                .and_then(|draft| draft.validate(snapshot).err())
        };
        self.error = invalid.as_ref().map(|invalid| invalid.text.clone());
        self.validation_error = invalid;
    }
    /// Save or a start checks this draft now. A refusal starts the watch (`refused`). It stays
    /// through a later Save, so an Undo that brings the fault back says so again; another draft
    /// ends it.
    fn watch_refusals(&mut self, path: String) {
        self.validation_path = Some(path);
        self.validated = None;
    }
    /// Record what Save's check refused, if anything. A refusal starts the watch.
    fn refused(&mut self, invalid: Option<Invalid>) {
        self.refusal_watch |= invalid.is_some();
        self.validation_error = invalid;
    }
    /// Whether the shown message is the last failed check of the draft.
    fn shows_validation_error(&self) -> bool {
        self.validation_error
            .as_ref()
            .is_some_and(|invalid| self.error.as_deref() == Some(invalid.text.as_str()))
    }
    /// The shown message's tone comes from the kind of fault recorded with it. A message from any
    /// other source is an error.
    fn error_tone(&self) -> Tone {
        if self.shows_validation_error() {
            self.validation_error
                .as_ref()
                .map_or(Tone::Error, |invalid| invalid.tone)
        } else {
            Tone::Error
        }
    }
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
        // A recovery copy's label starts with `Copy N`; the original has none, so the two differ.
        others.then(|| run_copy_word(&run.label)).flatten()
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
    /// The saved runs used by the Session plan and Timeline.
    pub fn saved_runs(&self) -> Vec<crate::encounter_run::SavedRunInfo> {
        let mut runs = self
            .snapshot
            .as_ref()
            .map_or_else(Vec::new, |snapshot| snapshot.runs.clone());
        if let Some(open) = self
            .run
            .as_ref()
            .and_then(crate::encounter_run::Run::live_info)
        {
            runs.retain(|run| run.id != open.id);
            runs.push(open);
        }
        runs
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
                Ok(mut run) => {
                    run.guard_campaign(&active.path);
                    if let Some(carry) = &self.carry {
                        run.rest_saved_setup(carry.rest_order);
                    }
                    self.run = Some(run);
                    self.go_to_run = true;
                    self.error = None;
                }
                Err(error) => self.error = Some(error),
            },
            Err(error) => self.error = Some(error),
        }
    }
    /// Go on with a fight that ended: open its run, and take the ending away as one step that
    /// Undo puts back.
    pub fn continue_saved_run(&mut self, path: &std::path::Path, active: &Campaign) {
        let open_here = self
            .run
            .as_ref()
            .is_some_and(|run| run.path() == Some(path));
        if !open_here {
            if self.has_run() {
                self.error = Some(BLOCKED_BY_RUN.to_owned());
                return;
            }
            match crate::encounter_run::Run::open(path, active.id) {
                Ok(mut run) => {
                    run.guard_campaign(&active.path);
                    self.run = Some(run);
                }
                Err(error) => {
                    self.error = Some(error);
                    return;
                }
            }
        }
        // The app has the Timeline, so it answers this by the rule before the next frame.
        if let Some(run) = &mut self.run
            && run.ended()
        {
            run.wants_unend = Some(crate::encounter_run::Unend::Continue);
        }
        self.history_read = None;
        self.go_to_run = true;
        self.error = None;
    }
    /// Work out again, by `continue_blocked`, whether the ended fights on screen can go on: the
    /// open run, and the fight read in History. The rule reads every event, so only when the
    /// Timeline or those fights changed.
    pub fn follow_continue_rule(&mut self, timeline: &crate::timeline::Book) {
        let ended = self
            .run
            .as_ref()
            .filter(|run| run.ended())
            .map(|run| run.id());
        let read = self.history_read.as_ref().and_then(|(path, _)| {
            let runs = &self.snapshot.as_ref()?.runs;
            runs.iter().find(|run| run.path == *path)?.id
        });
        if self.continue_key == Some((timeline.changes(), ended, read)) {
            return;
        }
        let rule = |id| {
            crate::encounter_run::continue_blocked(
                &timeline.events,
                timeline.unreadable.is_none(),
                id,
            )
        };
        if let Some(run) = self.run.as_mut().filter(|run| run.ended()) {
            run.continue_blocked = rule(run.id());
        }
        self.history_blocked = read.and_then(rule);
        self.continue_key = Some((timeline.changes(), ended, read));
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
        self.clear_notice();
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
        self.clear_notice();
        match GroupDraft::open(&active.path, path) {
            Ok(draft) => {
                if self
                    .group_draft
                    .as_ref()
                    .is_none_or(|open| open.path() != draft.path())
                {
                    self.search.clear();
                }
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
    pub fn discarded_group_name(&self) -> String {
        self.discarded_group
            .as_ref()
            .map_or_else(String::new, GroupDraft::name)
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
        self.clear_notice();
        self.search.clear();
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
    pub fn edit_path(&mut self, path: &str, active: &Campaign) {
        self.clear_notice();
        match Draft::open(&active.path, path) {
            Ok(draft) => {
                self.search.clear();
                self.draft = Some(draft);
                self.from_session = false;
                self.plan_on_save = false;
                self.error = None;
                self.message = None;
            }
            Err(error) => self.error = Some(error),
        }
    }

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
                            self.note_run_folder(&mut s);
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
            } else if !self.save_refused {
                // FC1: blocked as Save is, while the editor says Invalid.
                self.save_draft(ctx, &active, campaigns.modules());
            }
        }
    }
    fn save_draft(&mut self, ctx: &egui::Context, active: &Campaign, modules: &[Module]) {
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        let path = draft.path.clone();
        self.watch_refusals(path);
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        if let Some(error) = self
            .snapshot
            .as_ref()
            .and_then(|snapshot| draft.validate(snapshot).err())
        {
            self.error = Some(error.text.clone());
            self.refused(Some(error));
            return;
        }
        let outcome = draft.save(active, modules);
        let groups_saved = std::mem::take(&mut draft.groups_written);
        // The group being built stays in view, because it still takes what the search adds.
        draft.reveal_group |= groups_saved;
        match outcome {
            Ok(()) => {
                self.saved = true;
                self.error = None;
                self.message = Some("Encounter saved.".into());
                // AB1, FB1: the open fight from this encounter keeps its own copy of the Prep
                // text. The same Save gives it the new words; its combat state stays.
                if let (Some(run), Ok(entry)) =
                    (self.run.as_mut(), Entry::parse(&draft.path, &draft.source))
                    && run.follow_prep(entry.id, &splat::content::list_formats::prose(&entry))
                {
                    run.autosave();
                }

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
            Err(e) => {
                let invalid = self
                    .snapshot
                    .as_ref()
                    .and_then(|snapshot| draft.validate(snapshot).err())
                    .filter(|validation| validation.text == e);
                self.refused(invalid);
                self.error = Some(e);
            }
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
        self.clear_notice();
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
                .ok_or("This creature’s text is not available.")?;
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
        if editor.recovery.change.is_some() {
            if let Some(keep) =
                editor
                    .recovery
                    .show(ui, &editor.text().unwrap_or_default(), "creature")
            {
                if keep {
                    if let Some(change) = editor.recovery.accept() {
                        editor.path = change.path;
                        editor.baseline = change.text.unwrap_or_default();
                        editor.error = None;
                    }
                } else {
                    return Some(CreatureStep::Close);
                }
            }
            return None;
        }
        let dirty = editor.dirty();
        let (mut save, mut close) = (false, false);
        ui.label(
            Type::Heading
                .rich(ui, "This campaign’s creature")
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
                design::save_row_state(
                    ui,
                    Status {
                        icon: icons::OK,
                        word: "No changes",
                        color: colors().muted,
                    },
                    false,
                    None,
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
        // AK2, FK3: `text_input` takes the frame's width, so the field ends where Save does.
        design::field(ui, "Name", None, None, |ui| {
            let width = ui.available_width();
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
                    let output = egui::TextEdit::multiline(&mut editor.fields.body)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(ui.available_width() - 2.0 * design::S - design::M)
                        .margin(egui::vec2(design::S, design::S))
                        .show(ui);
                    if let Some(note) = crate::markdown_source::pressed_note_link(ui, &output, None)
                    {
                        crate::markdown_view::send_note_link(ui.ctx(), note, None);
                    }
                    design::named(ui, &output.response, "Creature text");
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
                self.clear_notice();
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
        for (index, slot) in fresh.setup.roster.iter().enumerate() {
            let row = || roster_row(index, slot, &snapshot);
            let reference = slot.creature.as_ref().ok_or(PARTY_FROM_CAMPAIGN)?;
            let id = snapshot.catalogue.resolve_ref(reference)?;
            let mut entry = snapshot.catalogue.run_entry(id);
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
                Some(_) => {
                    return Err(format!(
                        "{}: Splat cannot tell if it starts in the fight.",
                        row()
                    ));
                }
            };
            let overrides =
                Overrides::parse(&slot.overrides).map_err(|error| format!("{}: {error}", row()))?;
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
        let mut run_setup = fresh.setup.clone();
        for slot in &mut run_setup.roster {
            if let Some(reference) = &mut slot.creature
                && (reference.id.is_some() || reference.key.is_some())
            {
                reference.link = None;
            }
        }
        let setup = serde_yaml_ng::to_string(&run_setup).map_err(|e| e.to_string())?;
        let entry = Entry::parse(&fresh.path, &fresh.source)?;
        let mut run = crate::encounter_run::Run::new(
            fresh.name,
            participants,
            format!(
                "{}\n\n{}\n\nPhases, assumptions, completion conditions, and limits are resolved manually.\n\n```yaml\n{setup}\n```",
                splat::content::list_formats::prose(&entry),
                crate::encounter_run::SETUP_HEADING,
            ),
        )?;
        run.came_from(entry.id);
        run.set_inspiration(snapshot.options.inspiration());
        run.configure_storage(
            campaign.id,
            &snapshot.options.directory(campaign)?,
            snapshot.options.initiative,
        )?;
        run.guard_campaign(&campaign.path);
        if fresh.setup.combat_sync && campaign.dndbeyond_campaign_id.is_some() {
            run.keep_party_synced();
        }
        if let Some(carry) = self.carry.as_ref() {
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
    pub fn restore_run(&mut self, path: &std::path::Path, campaign: &Campaign, synced: &[String]) {
        if self.run.is_some() {
            return;
        }
        match crate::encounter_run::Run::open(path, campaign.id) {
            Ok(mut run) => {
                run.guard_campaign(&campaign.path);
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
            run.set_roster_names(snapshot.revision, &snapshot.known_creature_names);
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
    /// AB1: a folder that held this campaign's runs and is now gone is a read failure, not an
    /// empty list, also after a reload. The page remembers that the folder held runs until the
    /// campaign changes.
    fn note_run_folder(&mut self, snapshot: &mut Snapshot) {
        let campaign = self.active.as_ref().map(|campaign| campaign.id);
        if let Some(error) = snapshot.runs_missing.take()
            && campaign.is_some()
            && self.runs_known == campaign
        {
            snapshot.runs_unread = true;
            snapshot.issues.push(error);
        }
        if !snapshot.runs.is_empty() || snapshot.runs_unread {
            self.runs_known = campaign;
        }
    }
    pub fn read_runs_again(&mut self, active: &Campaign) {
        if let Some(s) = &mut self.snapshot {
            match s
                .saved_options
                .directory(active)
                .and_then(|dir| crate::encounter_run::Run::list(&dir, active.id))
            {
                Ok(runs) => {
                    s.runs = runs;
                    s.runs_unread = false;
                    s.issues
                        .retain(|issue| !issue.starts_with("Run folder not found"));
                }
                Err(error) => {
                    s.runs.clear();
                    s.runs_unread = true;
                    if !s.issues.contains(&error) {
                        s.issues.push(error);
                    }
                }
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
        // FA5: the title and the first column start at the shell's edge (x 148), as on World,
        // Library and Campaign. A quiet action at its text edge uses `.line_start()`.
        self.show_page(ui, campaigns, party, story);
    }

    fn show_page(
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
                "Save or discard the previous campaign’s encounter first.",
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

    /// `Discard changes` in the encounter editor: the saved encounter comes back, and the draft
    /// waits for one step of undo.
    fn discard_draft(&mut self, active: &Campaign) {
        self.clear_notice();
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
        // Its refusal was the discarded draft's: the words go with it. The watch stays for its
        // path, so the saved encounter (or the draft that Undo brings back) is checked again.
        if self.shows_validation_error() {
            self.error = None;
        }
        self.validation_error = None;
        self.validated = None;
        self.discarded = Some(draft);
        self.discarded_group = None;
    }
    /// The undo of `discard_draft`.
    fn undo_discard(&mut self) {
        self.draft = self.discarded.take();
        self.message = None;
        self.error = None;
    }
    /// One step back from `Discard changes`. Undo is the guard, so no dialog is needed.
    fn discard_undo(&mut self, ui: &mut egui::Ui) {
        // A creature's changes and a group's are not this encounter's, and leave its undo alone.
        if self.draft.as_ref().is_some_and(Draft::dirty) {
            self.discarded = None;
        }
        // Each undo waits while the other kind of draft is open. Restored behind that editor,
        // a draft could be neither reached nor closed.
        // FF7: a draft whose file was gone was removed with it; its Undo says so.
        if self.encounter_undo_ready() {
            let discarded = self.discarded.as_ref().unwrap();
            let undo = if discarded.recovery.file_gone() {
                design::undo_message(
                    ui,
                    Tone::Info,
                    &disk::removed_with_draft(&discarded.name),
                    "Undo remove",
                )
            } else {
                design::draft_discarded(ui, &discarded.name)
            };
            if undo {
                self.undo_discard();
            }
        }
        if self.group_draft.as_ref().is_some_and(GroupDraft::dirty) {
            self.discarded_group = None;
        }
        let group_undo = self.group_undo_ready() && {
            let discarded = self.discarded_group.as_ref().unwrap();
            if discarded.file_gone() {
                design::undo_message(
                    ui,
                    Tone::Info,
                    &disk::removed_with_draft(&discarded.name()),
                    "Undo remove",
                )
            } else {
                design::draft_discarded(ui, &discarded.name())
            }
        };
        if group_undo {
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
        if design::undo_message(
            ui,
            if deleted.unsent.is_empty() {
                Tone::Success
            } else {
                Tone::Warning
            },
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

    pub fn clear_notice(&mut self) {}

    fn results(&mut self, ui: &mut egui::Ui) {
        self.discard_undo(ui);
        if let Some(e) = &self.error {
            design::message(ui, Tone::Error, e);
            if e.starts_with("Run folder not found")
                && Action::secondary("Choose folder").show(ui).clicked()
            {
                self.wants_run_folder = true;
            }
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
                let ready = page.ready.len();
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
        // FE4, FA5: the title starts at the shell's edge (x 148), as Library's and World's do,
        // and the content below it starts there too. The header is a child that the page only
        // steps past.
        let room = ui.available_rect_before_wrap();
        let mut header = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(room)
                .layout(egui::Layout::top_down(egui::Align::LEFT)),
        );
        {
            let ui = &mut header;
            // Beside Lookup in a small window, the header cannot hold every word. The counts go
            // first. Then Reload keeps its icon, with its name and its reason on hover.
            let reload_blocked = group_blocked
                .or(loading.then_some("Still reading this campaign."))
                .or(settings_changed
                    .then_some("Save or discard the changes in Campaign, Rules and runs first."));
            let gap = ui.spacing().item_spacing.x;
            let new_button = || Action::secondary("New encounter").icon(icons::ADD);
            let group_button = || Action::quiet("New group").icon(icons::ADD);
            let reload_button = || Action::quiet(RELOAD).icon(icons::REFRESH);
            let mut words = reload_button().natural_width(ui) + gap;
            if !empty {
                words +=
                    new_button().natural_width(ui) + group_button().natural_width(ui) + 2.0 * gap;
            }
            let spare = ui.available_width()
                - design::number_width(ui, "Encounters", &Type::Title.font(ui))
                - words;
            let counts_fit = spare
                >= design::XS + gap + design::number_width(ui, &context, &Type::Small.font(ui));
            let context = counts_fit.then_some(context.as_str());
            design::page_header(ui, "Encounters", context, |ui| {
                if !empty {
                    new = new_button()
                        .blocked(blocked.or(loading.then_some("Still reading this campaign.")))
                        .show(ui)
                        .clicked();
                    new_group = group_button()
                        .blocked(
                            group_blocked.or(loading.then_some("Still reading this campaign.")),
                        )
                        .hint("A reusable set of creatures to add to a running encounter")
                        .show(ui)
                        .clicked();
                }
                // A run stays open through this, so a note fixed outside Splat reaches its Groups tab.
                reload = if spare >= 0.0 {
                    reload_button()
                        .blocked(reload_blocked)
                        .hint(RELOAD_HINT)
                        .show(ui)
                        .clicked()
                } else {
                    design::locked(ui, reload_blocked, |ui| {
                        design::icon_button(ui, icons::REFRESH, RELOAD)
                            .on_hover_text(RELOAD_HINT)
                            .clicked()
                    })
                };
            });
        }
        let used = header.min_rect();
        ui.advance_cursor_after_rect(used.with_min_x(room.min.x));
        if new {
            self.start_new_draft();
            return;
        }
        if new_group {
            self.search.clear();
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
        for issue in snapshot
            .issues
            .iter()
            .filter(|issue| issue.starts_with("Run folder not found"))
        {
            if design::message_with_action(ui, Tone::Warning, issue, "Choose folder") {
                self.wants_run_folder = true;
            }
        }
        if !snapshot.issues.is_empty() {
            design::section(
                ui,
                issue_count_title(ui, &content_issue_count(snapshot.issues.len())),
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
                    ui.horizontal(|ui| {
                        create = Action::primary("New encounter")
                            .icon(icons::ADD)
                            .blocked(blocked)
                            .name("Create a first encounter")
                            .show(ui)
                            .clicked();
                        ui.add_space(design::L);
                        create_group = Action::quiet("New group")
                            .icon(icons::ADD)
                            .blocked(group_blocked)
                            .hint("A reusable set of creatures to add to a running encounter")
                            .show(ui)
                            .clicked();
                    });
                },
            );
            if create {
                self.start_new_draft();
            }
            if create_group {
                self.search.clear();
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
        let mut page = history::arrange(
            &snapshot.templates,
            &snapshot.runs,
            live.as_ref(),
            story.events,
            story.sessions,
        );
        page.runs_unread = snapshot.runs_unread;
        let start_blocked = self.start_blocked();
        let mut action = None;
        // FB2: the list keeps the scroll bar's lane whether or not the bar shows, so opening
        // Groups moves no row action.
        let lane = scroll_lane(ui);
        egui::ScrollArea::vertical()
            .id_salt("encounter_list")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_max_width(lane);
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
                    self.history_blocked,
                    &mut action,
                );
            });
        let Some(action) = action else {
            return;
        };
        self.clear_notice();
        match action {
            ListAction::BackToRun => self.go_to_run = true,
            ListAction::OpenGroup(path) => match GroupDraft::open(&active.path, &path) {
                Ok(draft) => {
                    self.search.clear();
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
            ListAction::ContinueFight(path) => self.continue_saved_run(&path, active),
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
                    self.search.clear();
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
        if let Some(path) = self.draft.as_ref().map(|draft| draft.path.clone()) {
            self.watch_refusals(path);
        }
        if let Some(error) = self
            .snapshot
            .as_ref()
            .zip(self.draft.as_ref())
            .and_then(|(snapshot, draft)| draft.validate(snapshot).err())
        {
            self.error = Some(error.text.clone());
            self.refused(Some(error));
            return;
        }
        match self.start_run(active, modules) {
            Ok(run) => {
                self.run = Some(run);
                self.go_to_run = true;
                self.error = None;
            }
            Err(error) => {
                let invalid = self
                    .snapshot
                    .as_ref()
                    .zip(self.draft.as_ref())
                    .and_then(|(snapshot, draft)| draft.validate(snapshot).err())
                    .filter(|validation| validation.text == error);
                self.refused(invalid);
                self.error = Some(error);
            }
        }
    }

    fn editor_view(
        &mut self,
        ui: &mut egui::Ui,
        active: &Campaign,
        campaigns: &Campaigns,
        party: &Party,
    ) {
        if self.encounter_disk_view(ui, active, campaigns.modules()) {
            return;
        }
        let no_names = BTreeMap::new();
        let names = self
            .snapshot
            .as_ref()
            .map(|snapshot| &snapshot.known_creature_names)
            .unwrap_or(&no_names);
        // Drawn below the header, so a fault does not move the name field or its actions.
        let unread_rows = self.draft.as_ref().map(|draft| {
            let faults = Entry::parse(&draft.path, &draft.source)
                .map(|entry| roster_faults(&entry, names))
                .unwrap_or_default();
            let notes = unread_notes(&draft.path, &draft.flags, &faults);
            let count = faults.len();
            // FC3: the roster shows each row that blocks Start fight in its place. The faults
            // move there; nothing is copied.
            let rows: Arc<Vec<RosterFault>> = Arc::new(faults);
            ui.data_mut(|data| data.insert_temp(roster_fault_rows_id(draft.id), rows));
            (notes, count, draft.path.clone())
        });
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
        // FC1: a draft that Save refuses says Invalid with its cause, and Save and Ctrl+S are
        // blocked. The cause is the roster check Save runs first (this frame already makes it),
        // or the last refusal while it still holds (`refresh_validation_error` checks it again).
        let refusal = self
            .shows_validation_error()
            .then(|| self.error.clone())
            .flatten();
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
        let roster_reason = self.draft.as_ref().and_then(|draft| {
            Entry::parse(&draft.path, &draft.source)
                .ok()
                .and_then(|entry| roster_fault_reason(&draft.path, &roster_faults(&entry, names)))
        });
        let refusal = roster_reason.clone().or(refusal);
        self.save_refused = dirty && refusal.is_some();
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
                    let start_blocked = roster_reason.as_deref().or(has_run
                        .then_some(BLOCKED_BY_RUN)
                        .or(creature_dirty.then_some(BLOCKED_BY_CREATURE))
                        .or(loading.then_some("Still reading this campaign."))
                        .or(draft_ref
                            .setup
                            .roster
                            .is_empty()
                            .then_some("Add participants first")));
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
                    // Unsaved changes own the primary: Resume waits until they are saved or
                    // discarded.
                    run_it = if has_run || dirty {
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
                    save = if dirty && !has_run && refusal.is_none() {
                        Action::primary("Save encounter")
                    } else {
                        Action::secondary("Save encounter")
                    }
                    .blocked(if dirty {
                        refusal.as_deref()
                    } else {
                        Some("No changes to save.")
                    })
                    .show(ui)
                    .clicked();
                },
            );
        });
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Read.control());
            let (icon, word, color) = if dirty && refusal.is_some() {
                // The message above names the cause, as Library's and World's do.
                (icons::ERROR, "Invalid", colors().document.invalid)
            } else if dirty {
                (icons::DRAFT, "Unsaved", colors().document.unsaved)
            } else if saved_note.is_some() {
                (icons::OK, "Saved", colors().document.saved)
            } else {
                (icons::OK, "No changes", colors().muted)
            };
            {
                discard = editor_save_state(
                    ui,
                    Status { icon, word, color },
                    dirty,
                    creature_dirty.then_some(BLOCKED_BY_CREATURE),
                );
            }
            // FB6: while Save is blocked, Discard is the one way out.
            if dirty {
                blocked_reason(
                    ui,
                    Some(if refusal.is_some() {
                        blocked_draft_switch_reason()
                    } else {
                        draft_switch_reason()
                    }),
                );
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
            // FB1: the open fight from this encounter takes its read-aloud text and notes at
            // Save; anything else waits for the next fight.
            let follows = self
                .run
                .as_ref()
                .and_then(crate::encounter_run::Run::source)
                .is_some_and(|source| Some(source) == self.draft.as_ref().map(|draft| draft.id));
            blocked_reason(
                ui,
                Some(if follows {
                    "Save gives this fight’s Prep the new read-aloud text and notes. Other changes apply to the next fight. To add to this fight, use Add to fight in the run."
                } else {
                    "Changes apply to the next fight. To add to this fight, use Add to fight in the run."
                }),
            );
        }
        if let Some((notes, faults, path)) = &unread_rows {
            unread_roster_rows(ui, &active.path, path, notes, *faults);
        }
        design::rule(ui);
        if back {
            self.clear_notice();
            self.draft = None;
            self.message = None;
            self.error = None;
            self.creature = None;
            self.wants_session = std::mem::take(&mut self.from_session);
            self.plan_on_save = false;
            return;
        }
        if discard {
            self.discard_draft(active);
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
        self.refresh_validation_error(false, edit_input(ui.ctx()));
        if let Some(e) = &self.error {
            // A deleted entry and an unreadable file are stated in the rows' tone too.
            design::message(ui, self.error_tone(), e);
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
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                design::counted_tab(
                    ui,
                    &mut pane,
                    EditorPane::Roster,
                    "Roster",
                    draft.setup.roster.len(),
                    ("participant", "participants"),
                );
                design::segmented(
                    ui,
                    &mut pane,
                    &[(
                        EditorPane::Add,
                        if open_creature {
                            "This creature"
                        } else {
                            "Add participants"
                        },
                    )],
                );
            });
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
                    true,
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
        let add_width = if design::column_dragged(ui.ctx(), ADD_PANE) {
            design::column_widths(ui.ctx())
                .get(ADD_PANE)
                .copied()
                .unwrap_or(ADD_PANE_MAX_WIDTH)
        } else {
            ADD_PANE_MAX_WIDTH
        };
        let panes_width = ui
            .available_width()
            .min(ROSTER_MAX_WIDTH + add_width + design::S);
        ui.allocate_ui_with_layout(
            egui::vec2(panes_width, ui.available_height()),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.set_width(panes_width);
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
                    None => {
                        Self::add_pane(ui, draft, snapshot, search, focus_search, peek, compact)
                    }
                });
                // A panel clips to its own rect. It starts `S` before the roster's text edge, so
                // a line-start quiet action there keeps its whole press area.
                let rest = ui.available_rect_before_wrap();
                let mut roster_ui = ui.new_child(
                    egui::UiBuilder::new().max_rect(rest.with_min_x(rest.min.x - design::S)),
                );
                egui::CentralPanel::default()
                    // The panel's drag handle lies on this edge, so the roster stops short of it.
                    .frame(egui::Frame::new().inner_margin(egui::Margin {
                        left: design::S as i8,
                        right: design::S as i8,
                        top: design::S as i8,
                        bottom: 0,
                    }))
                    .show_inside(&mut roster_ui, |ui| {
                        // The content cap also holds after the DM moves the panel.
                        ui.set_width(ui.available_width().min(ROSTER_MAX_WIDTH));
                        Self::roster_pane(
                            ui,
                            draft,
                            snapshot,
                            focus_search,
                            expanded,
                            linked.then_some(party),
                            &mut make_yours,
                            false,
                        );
                    });
                ui.advance_cursor_after_rect(roster_ui.min_rect());
            },
        );

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
            design::help(ui, PARTY_SOURCE);
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
                already: &|c: &Creature| snapshot.already_on_roster(draft, c),
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
        Self::creature_search_with_summary(ui, creatures, search, focus_search, peek, roster, "")
    }

    fn creature_search_with_summary<'a>(
        ui: &mut egui::Ui,
        creatures: impl Iterator<Item = &'a Creature>,
        search: &mut String,
        focus_search: &mut bool,
        peek: &mut Option<Uuid>,
        roster: SearchRoster<'_>,
        summary: &str,
    ) -> Option<(Uuid, bool)> {
        let mut chosen = None;
        let response = design::search_field(
            ui,
            SEARCH_FIELD,
            search,
            "Find a creature by name or detail…",
            ui.available_width(),
        );
        if std::mem::take(focus_search) {
            design::focus_and_select(ui, response.id, search);
        }
        let creatures: Vec<&Creature> = creatures.collect();
        // The roster and a group share one filter, as the DM looks for the same creatures.
        let filter_id = egui::Id::new("creature_search_filter");
        let mut filter: Filter = ui.data(|data| data.get_temp(filter_id)).unwrap_or_default();
        if crate::creature_filter::filter_bar(ui, &mut filter, || {
            creatures.iter().filter_map(|c| c.facets.as_ref()).collect()
        }) {
            ui.data_mut(|data| data.insert_temp(filter_id, filter.clone()));
        }
        if !summary.is_empty() {
            let shown =
                design::words_that_fit(ui, summary, &Type::Small.font(ui), ui.available_width());
            ui.label(Type::Small.rich(ui, shown).color(colors().muted))
                .on_hover_text(summary);
        }
        let query = search.to_lowercase();
        let parsed = Query::parse(&query);
        let typed = crate::matching::Typed::new(&parsed.words);
        let mut ranked: Vec<(crate::matching::Rank, &Creature)> = creatures
            .into_iter()
            .filter(|c| {
                let facets = c.facets.as_ref();
                (!parsed.names_facts() || facets.is_some_and(|f| parsed.matches_facts(f)))
                    && (filter.is_empty() || facets.is_some_and(|f| filter.matches(f)))
                    && (!query.is_empty() || !c.party || !(roster.already)(c))
            })
            .filter_map(|c| {
                if parsed.words.is_empty() {
                    return Some((crate::matching::Rank::typed(0), c));
                }
                let facts = c
                    .facets
                    .as_ref()
                    .is_some_and(|f| parsed.words_in(&c.name, f));
                Some((crate::matching::with_other(typed.name(&c.lower), facts)?, c))
            })
            .collect();
        // The plain name first, so "wolf" finds Wolf before Dire Wolf. A fact match comes
        // after every name match as typed, and a typo in the name comes last. The sort is
        // stable, so the list's order stays inside each rank.
        if !parsed.words.is_empty() {
            ranked.sort_by_key(|&(rank, c)| (rank, c.origin));
        }
        let as_typed = ranked.iter().take_while(|(rank, _)| !rank.typo).count();
        let matches: Vec<&Creature> = ranked.into_iter().map(|(_, c)| c).collect();
        // Enter adds the best match, with focus kept for the next name. A name found only with
        // a typo is a guess, so Enter does not add it.
        if response.lost_focus()
            && ui.input(|i| i.key_pressed(egui::Key::Enter))
            && !query.is_empty()
            && as_typed > 0
            && let Some(first) = matches.first()
            && !(roster.already)(first)
            && first.unavailable.is_none()
        {
            chosen = Some((first.id, first.party));
            *focus_search = true;
        }
        let hidden_duplicates = duplicate_creature_results(ui, &query);
        if as_typed == 0 && !matches.is_empty() {
            ui.label(
                Type::Small
                    .rich(ui, crate::matching::CLOSE_NAMES)
                    .color(colors().muted),
            );
        }
        if as_typed == 0 && !hidden_duplicates {
            let index = ui.ctx().data(|data| {
                data.get_temp::<Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                    "reader_link_index",
                ))
            });
            if let Some(index) = index {
                for (package, entry, name, owner) in index.unused_creatures(&parsed.words) {
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
        }
        if matches.is_empty() && !hidden_duplicates {
            design::empty_state_with_body(
                ui,
                icons::SEARCH,
                "No creatures match",
                |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x =
                            design::number_width(ui, " ", &Type::Body.font(ui));
                        ui.label(
                            Type::Body
                                .rich(
                                    ui,
                                    if filter.is_empty() {
                                        "Check the spelling, or check"
                                    } else {
                                        "Remove a filter, or check"
                                    },
                                )
                                .color(colors().muted),
                        );
                        // FB4: one sentence in one size. The link is a Body link inside it, and
                        // the full stop follows its last word.
                        // The link and its full stop wrap together.
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            if design::link(ui, groups::PACKAGES_LINK).clicked() {
                                ui.data_mut(|data| {
                                    data.insert_temp(egui::Id::new("open_campaign_packages"), true)
                                });
                            }
                            ui.label(Type::Body.rich(ui, ".").color(colors().muted));
                        });
                    });
                },
                |_| {},
            );
        }
        ui.spacing_mut().item_spacing.y = 0.0;
        let row_height = RESULT_ROW
            + if matches.iter().any(|c| c.unavailable.is_some()) {
                Type::Small.line()
            } else {
                0.0
            };
        // FH3, SH2: the results keep the scroll bar's lane whether or not the bar shows, so each
        // `Add` keeps its x as the DM types and the list grows or shrinks.
        let lane = scroll_lane(ui);
        egui::ScrollArea::vertical()
            .id_salt(("encounter_results", &query))
            .auto_shrink([false, false])
            .show_rows(ui, row_height, matches.len(), |ui, range| {
                ui.set_max_width(lane);
                for c in &matches[range] {
                    ui.push_id(c.id, |ui| {
                        let (add, read) = creature_result_row(ui, c, "Add", (roster.already)(c));
                        if read {
                            *peek = Some(c.id);
                        }
                        if add {
                            chosen = Some((c.id, c.party));
                            *focus_search = true;
                        }
                    });
                }
            });
        chosen
    }

    #[allow(clippy::too_many_arguments)]
    fn roster_pane(
        ui: &mut egui::Ui,
        draft: &mut Draft,
        snapshot: &Snapshot,
        focus_search: &mut bool,
        expanded: &mut Option<String>,
        linked_party: Option<&Party>,
        make_yours: &mut Option<usize>,
        tabbed: bool,
    ) -> bool {
        // All of it scrolls, so that the settings give up their room to a group being built.
        // A line-start quiet action reaches `S` into the page's
        // gutter. The scroll area starts there too, so it shows the whole target, and its margin
        // keeps the text where it was.
        let area = ui.available_rect_before_wrap();
        ui.scope_builder(
            egui::UiBuilder::new()
                .id(("encounter_roster", draft.id))
                .max_rect(area.with_min_x(area.min.x - design::S)),
            |ui| {
        // The roster keeps the scroll bar's lane whether or not the bar shows (FB2's family).
        let lane = scroll_lane(ui) - design::S;
        egui::ScrollArea::vertical()
            .id_salt("encounter_editor")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui::Frame::new()
                    .inner_margin(egui::Margin { left: design::S as i8, ..egui::Margin::ZERO })
                    .show(ui, |ui| {
                ui.set_width(ui.available_width().min(design::LIST_MEASURE).min(lane));
                if !tabbed { design::group_heading(ui, "Roster", draft.setup.roster.len()); }
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
                // FI7: what the option changes, in the form the sync option's help uses.
                design::help(ui, REPEATABLE_HELP);
                ui.add_space(design::M);
                Self::loot_field(ui, draft);
                ui.add_space(design::M);
                design::field(ui, "Read-aloud and notes", None, None, |ui| {
                let output = notes_field(&mut draft.body, 5)
                    .id(egui::Id::new(("encounter_notes", draft.id)))
                    .hint_text(design::placeholder(ui, "What you read to the table, then your tactics. The run shows this before the first turn."))
                    .show(ui);
                if let Some(note) = crate::markdown_source::pressed_note_link(ui, &output, None) {
                    crate::markdown_view::send_note_link(ui.ctx(), note, None);
                }
                design::named(ui, &output.response, "Read-aloud and notes");
                });
                groups::of_encounter(ui, draft, snapshot, focus_search, expanded)
                    })
                    .inner
            })
            .inner
        }).inner
    }

    /// What the party finds when this fight is over, one line each. The note keeps it as its
    /// `## Loot` list, which Obsidian shows too.
    fn loot_field(ui: &mut egui::Ui, draft: &mut Draft) {
        design::group_heading(ui, LOOT_FIELD, draft.loot.len());
        if !draft.loot_owned {
            blocked_reason(
                ui,
                Some(
                    "The Loot section of this note has more than a list. Edit it in Read-aloud and notes.",
                ),
            );
            return;
        }
        design::help(ui, ENCOUNTER_LOOT_HELP);
        crate::loot_editor::show(ui, draft.id, "the encounter", &mut draft.loot, false);
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
        // AD1: help under an option keeps the reader measure.
        design::help(ui, words);
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
        let mut replacement = None;
        let missing_counts: Vec<_> = draft
            .setup
            .roster
            .iter()
            .map(|slot| {
                draft
                    .setup
                    .roster
                    .iter()
                    .filter(|other| other.creature == slot.creature)
                    .count()
            })
            .collect();
        let fault_rows = ui
            .data(|data| data.get_temp::<Arc<Vec<RosterFault>>>(roster_fault_rows_id(draft.id)))
            .unwrap_or_default();
        let undo_key = ui.id().with(("replacement_undo", draft.id));
        let undo = ui
            .ctx()
            .data_mut(|data| data.get_temp::<ReplacementUndo>(undo_key));
        if let Some(undo) = undo {
            if undo.after == draft.setup.roster {
                // The result is a message, so its Undo keeps the undo icon as every other
                // Undo does.
                if design::undo_message(ui, Tone::Success, &undo.words, "Undo replacement") {
                    draft.setup.roster = undo.before;
                }
            } else {
                ui.ctx()
                    .data_mut(|data| data.remove::<ReplacementUndo>(undo_key));
            }
        }
        for (i, slot) in draft.setup.roster.iter_mut().enumerate() {
            for fault in fault_rows.iter().filter(|fault| fault.before == i) {
                unread_roster_row(ui, fault);
            }
            ui.push_id(slot.key.clone(), |ui| {
                let resolved = slot
                    .creature
                    .as_ref()
                    .and_then(|r| snapshot.catalogue.resolve_ref(r).ok());
                let definition = resolved
                    .and_then(|id| snapshot.catalogue.entries.get(&id))
                    .map(|e| &e.entry);
                let name = roster_name(slot, definition, &snapshot.known_creature_names);
                let missing = definition.is_none();
                let unreadable = slot
                    .creature
                    .as_ref()
                    .and_then(|reference| snapshot.catalogue.unreadable_ref(reference));
                let creature =
                    resolved.and_then(|id| snapshot.creatures.iter().find(|c| c.id == id));
                let synced = live.is_some() && creature.is_some_and(|c| c.party);
                // A party sheet always has hit points; otherwise the stats and changes say.
                let hp_missing = live
                    .zip(creature)
                    .and_then(|(party, creature)| party.sheet(creature.character.as_deref()?))
                    .is_none()
                    && creature.zip(definition).is_some_and(|(creature, entry)| {
                        groups::shown_hp_missing(&slot.overrides, entry, &slot.side, creature)
                    });
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
                let side_icon = if slot.side == "party" {
                    icons::PARTY
                } else if slot.side == "enemies" {
                    icons::ENCOUNTERS
                } else {
                    icons::GROUP
                };
                let identity = definition
                    .filter(|_| slot.overrides.contains_key("name"))
                    .map(|entry| format!(" · as {}", entry.name))
                    .unwrap_or_default();
                let changed = definition.is_some_and(|entry| {
                    !(slot.overrides.is_empty()
                        || slot.overrides.len() == 1
                            && snapshot.npc_name_for(entry, &slot.overrides).is_some())
                });
                let side = design::reader::human_label(&slot.side);
                // The row states the fault in the tone of the message above the roster.
                let (fault_icon, fault_color) = missing_creature_mark(unreadable);
                // Two files of one identity: the row says the short cause, and the message above
                // the roster and the row's hover name the files once.
                let fault_words = unreadable.map_or(MISSING_CREATURE, |note| {
                    if !note.duplicate {
                        note.word.as_str()
                    } else if crate::world::repair_in_world(&note.path) {
                        // World's row words for the same fault.
                        "Same note in two files"
                    } else {
                        crate::library::SAME_ENTRY
                    }
                });
                let detail = if missing {
                    format!("{side_icon} {side} · {fault_icon} {fault_words}")
                } else if detail.is_empty() {
                    format!("{side_icon} {side}{identity}")
                } else {
                    format!("{side_icon} {side} · {detail}{identity}")
                };
                let is_open = expanded.as_deref() == Some(slot.key.as_str());
                let member_row = ui.horizontal(|ui| {
                    ui.set_min_height(Density::Glance.row());
                    // Every slot is reserved, so columns line up between rows.
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        row_menu(ui, &name, |ui| {
                            if menu_action(ui, "Edit", None) {
                                *expanded = Some(slot.key.clone());
                            }
                            if menu_action(ui, "Move up", (i == 0).then_some("Already first.")) {
                                raise = Some(i);
                            }
                            if menu_action(
                                ui,
                                "Move down",
                                (i + 1 == count).then_some("Already last."),
                            ) {
                                lower = Some(i);
                            }
                            if menu_action(ui, "Duplicate", None) {
                                duplicate = Some(i);
                            }
                            if creature.is_some_and(|c| !c.party) {
                                if menu_action(ui, "Make it yours", None) {
                                    *make_yours = Some(i);
                                }
                                choice_note(
                                    ui,
                                    "Make it yours",
                                    Some("Edit a copy in this campaign."),
                                );
                            }
                            if menu_action(ui, "Remove", None) {
                                remove = Some(i);
                            }
                        });
                        if let Some(note) = unreadable {
                            if Action::quiet(if crate::world::repair_in_world(&note.path) {
                                "Open in World"
                            } else {
                                "Open in Library"
                            })
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                            {
                                ui.ctx().data_mut(|data| {
                                    data.insert_temp(
                                        egui::Id::new("open_library_repair"),
                                        note.path.clone(),
                                    )
                                });
                            }
                        } else if Action::quiet(if creature.is_none() {
                            "Replace…"
                        } else {
                            "Edit"
                        })
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
                            if missing {
                                let split = detail.find(fault_icon).unwrap_or(0);
                                let mut job = egui::text::LayoutJob::default();
                                for (text, color) in [
                                    (&detail[..split], colors().muted),
                                    (&detail[split..split + fault_icon.len()], fault_color),
                                    (&detail[split + fault_icon.len()..], colors().muted),
                                ] {
                                    job.append(
                                        text,
                                        0.0,
                                        egui::TextFormat {
                                            font_id: Type::Small.font(ui),
                                            color,
                                            ..Default::default()
                                        },
                                    );
                                }
                                let shown = ui.add(egui::Label::new(job).wrap());
                                if let Some(note) = unreadable.filter(|note| note.duplicate) {
                                    shown.on_hover_text(&note.word);
                                }
                            } else if !detail.is_empty() {
                                if changed && !is_open {
                                    ui.spacing_mut().interact_size.y = 0.0;
                                    ui.horizontal(|ui| {
                                        // The separator's own blanks are the gap on both sides, as in the text before it.
                                        ui.spacing_mut().item_spacing.x = 0.0;
                                        let word = definition
                                            .and_then(|entry| {
                                                Overrides::parse(&slot.overrides).ok().map(
                                                    |overrides| {
                                                        groups::change_words(
                                                            &overrides,
                                                            entry,
                                                            &slot.side,
                                                            &crate::loot_editor::missing_in(
                                                                ui.ctx(),
                                                                overrides.loot.as_deref(),
                                                            ),
                                                        )
                                                    },
                                                )
                                            })
                                            .unwrap_or_else(|| "Changed".into());
                                        let change_width =
                                            design::number_width(ui, &word, &Type::Small.font(ui));
                                        let (fitted, fault_kept) = changed_detail_parts(
                                            ui,
                                            &detail,
                                            &word,
                                            ui.available_width(),
                                            hp_missing,
                                        );
                                        let detail_rect = ui
                                            .label(detail_text(ui, &stat_line(&fitted), fault_kept))
                                            .on_hover_text(&detail)
                                            .rect;
                                        let separator = ui
                                            .label(
                                                Type::Small.rich(ui, " · ").color(colors().muted),
                                            )
                                            .rect;
                                        // Keep the text baseline and row pitch. The 24 px target extends into the row's padding.
                                        let at = egui::pos2(
                                            separator.right(),
                                            detail_rect.center().y - design::XL / 2.0,
                                        );
                                        let mut inline = ui.new_child(
                                            egui::UiBuilder::new()
                                                .max_rect(egui::Rect::from_min_size(
                                                    at,
                                                    egui::vec2(change_width, design::XL),
                                                ))
                                                .layout(egui::Layout::top_down(egui::Align::Min)),
                                        );
                                        if design::small_link(&mut inline, &word).clicked() {
                                            groups::show_form_when_it_opens(ui);
                                            *expanded = Some(slot.key.clone());
                                        }
                                    });
                                    return;
                                }
                                // What a player has left is the reason to look, so all of it shows.
                                ui.spacing_mut().interact_size.y = 0.0;
                                ui.horizontal_wrapped(|ui| {
                                    ui.spacing_mut().item_spacing.x = 0.0;
                                    for (at, part) in
                                        detail_parts(&detail, hp_missing).iter().enumerate()
                                    {
                                        if at > 0 {
                                            ui.label(
                                                Type::Small.rich(ui, " · ").color(colors().muted),
                                            );
                                        }
                                        if part.words == "Bloodied" {
                                            design::status(
                                                ui,
                                                Status {
                                                    icon: icons::BLOODIED,
                                                    word: part.words,
                                                    color: colors().hp.bloodied,
                                                },
                                            );
                                        } else {
                                            ui.label(
                                                Type::Small
                                                    .rich(ui, stat_line(part.words))
                                                    .color(part.color()),
                                            );
                                        }
                                    }
                                });
                            }
                        });
                    });
                });
                let member_row = member_row.response.rect;
                if is_open && missing && unreadable.is_none() {
                    if let Some((creature, all)) =
                        replace_missing_creature(ui, snapshot, missing_counts[i], &name)
                    {
                        replacement = Some((i, creature, all));
                        *expanded = None;
                    }
                } else if is_open && !missing {
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
                    design::help(
                        ui,
                        if synced {
                            "D&D Beyond decides this player’s HP while this encounter syncs."
                        } else {
                            "With a value off, the creature’s own starting state applies."
                        },
                    );
                    if let Some(entry) = definition {
                        let done = groups::overrides_form(
                            ui,
                            &format!("{}:{}", draft.id, slot.key),
                            &mut slot.overrides,
                            entry,
                            &slot.side,
                            Some(&sides),
                        );
                        if done {
                            *expanded = None;
                            ui.scroll_to_rect(member_row, Some(egui::Align::Min));
                        }
                    }
                }
                design::rule(ui);
            });
        }
        for fault in fault_rows.iter().filter(|fault| fault.before >= count) {
            unread_roster_row(ui, fault);
        }
        if let Some((selected, creature, all)) = replacement {
            let before = draft.setup.roster.clone();
            replace_roster(&mut draft.setup.roster, selected, creature, all, snapshot);
            let old = roster_name(&before[selected], None, &snapshot.known_creature_names);
            let rows = before
                .iter()
                .zip(&draft.setup.roster)
                .filter(|(was, now)| was.creature != now.creature)
                .count();
            let new = snapshot
                .catalogue
                .entries
                .get(&creature)
                .map_or_else(|| "the new creature".to_owned(), |e| e.entry.name.clone());
            ui.ctx().data_mut(|data| {
                data.insert_temp(
                    undo_key,
                    ReplacementUndo {
                        before,
                        after: draft.setup.roster.clone(),
                        words: replacement_words(&old, rows, &new),
                    },
                )
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
        let rule_flags: Vec<_> = snapshot
            .catalogue
            .entries
            .get(&active.id)
            .into_iter()
            .flat_map(|item| &item.entry.flags)
            // Every flag Campaign sends here with its repair link, by its kind, never its words.
            .filter(|flag| flag.kind == splat::content::layout::FlagKind::RuleChoice)
            .collect();
        let closed_while = draft_open.then_some("Save or discard the open encounter first.");
        ui.scope_builder(egui::UiBuilder::new().max_rect(egui::Rect::from_min_size(
            ui.cursor().min, egui::vec2(ui.available_width().min(design::FORM_WIDTH), ui.available_height()))), |ui| {
        design::locked(ui, closed_while, |ui| {
            design::small_text_field(
                ui,
                "Run folder",
                &mut snapshot.options.folder,
                Some("Relative to the campaign folder, or an absolute path."),
                None,
            );
            // The warning names the line; the file's path and links start at the last
            // warning's words.
            for (at, flag) in rule_flags.iter().enumerate() {
                let text = flag.message.clone();
                if at + 1 == rule_flags.len() {
                    design::message_with_actions(ui, Tone::Warning, &text, |ui| {
                        file_line(ui, &active.path, "Campaign.md");
                    });
                } else {
                    design::message(ui, Tone::Warning, &text);
                }
            }
            design::small_field(ui, "Roll NPC initiative", None, None, |ui| {
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
            ui.label(Type::Small.rich(ui, "Run setup starts with this choice. You can change it there for one roll. Groups that arrive later use it too.").color(colors().muted));
            ui.checkbox(
                &mut snapshot.options.initiative.roster_ties,
                "Break initiative ties in roster order",
            );
            // The campaign's packages decide inspiration. Only several rules need a choice.
            if snapshot.options.inspiration_rules.len() > 1 {
                let rules = snapshot.options.inspiration_rules.clone();
                let line = |rule: &crate::encounter_settings::InspirationRule| {
                    format!("{}: {}", rule.name, rule.effect.effect())
                };
                let selected = rules
                    .iter()
                    .find(|rule| Some(rule.id) == snapshot.options.inspiration_choice)
                    .map_or_else(|| "Choose a rule".to_owned(), line);
                design::small_field(ui, "Inspiration rule", None, None, |ui| {
                    let combo = egui::ComboBox::from_id_salt("campaign_inspiration")
                        .icon(design::combo_icon)
                        .width(ui.available_width().min(320.0))
                        .selected_text(selected)
                        .show_ui(ui, |ui| {
                            for rule in &rules {
                                ui.selectable_value(
                                    &mut snapshot.options.inspiration_choice,
                                    Some(rule.id),
                                    line(rule),
                                );
                            }
                        });
                    design::named(ui, &combo.response, "Inspiration rule");
                });
                if snapshot.options.inspiration().is_none() {
                    design::help(ui, "This campaign’s packages have more than one inspiration rule. Runs offer inspiration once you choose one.");
                }
            }
            let changed = snapshot.options != snapshot.saved_options;
            let left = snapshot.options.runs_left_in(&snapshot.saved_options, &active);
            let run_files = left.as_ref().map_or(0, Vec::len);
            let folder_changed = snapshot.options.directory(&active).ok() != snapshot.saved_options.directory(&active).ok();
            let blocked = if run_open { Some("Close the open run to save run settings.") }
                else if folder_changed && left.is_err() { Some("Could not read saved runs. Check the old run folder before saving.") }
                else if !changed && rule_flags.is_empty() { Some("No changes to save.") } else { None };
            if run_files > 0 {
                let runs: Vec<_> = left.as_ref().ok().into_iter().flatten()
                    .filter_map(|path| crate::encounter_run::Run::open(path, active.id).ok())
                    .filter(|run| !run.state_words(true).starts_with("Ended"))
                    .collect();
                let words = if run_files == 1 && let Some(run) = runs.first() {
                    format!("{} ({}) stays in {}. From the new folder, Splat will not show it or resume it.", run.name, run.state_words(true), snapshot.saved_options.folder)
                } else {
                    format!("{} {} in {}. {} are unfinished. From the new folder, Splat will not show these runs or resume them.", design::count(run_files, "saved run", "saved runs"), if run_files == 1 { "stays" } else { "stay" }, snapshot.saved_options.folder, runs.len())
                };
                design::message(ui, Tone::Warning, &words);
            }
            ui.vertical(|ui| {
                // Ctrl+S saves exactly when the button it labels can: a rule repair with no
                // other change too.
                let save_key = blocked.is_none() && run_files == 0
                    && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S));
                // The save state, Discard and the reasons; then Save, or the two run-moving
                // choices.
                macro_rules! save_state {
                    ($ui:ident) => {{
                        let ui = &mut *$ui;
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
                if !changed && !rule_flags.is_empty() {
                    // Save is the repair here: it writes the shown settings over the unread choice.
                    let lines: Vec<_> = rule_flags.iter().map(|flag| flag.line.to_string()).collect();
                    let lines = match lines.split_last() {
                        Some((last, [])) => format!("line {last}"),
                        Some((last, before)) => format!("lines {} and {last}", before.join(", ")),
                        None => String::new(),
                    };
                    let word = format!("Save replaces the unread choice on {lines} with these settings");
                    design::save_row_state(ui, Status { icon: icons::WARN, word: &word, color: colors().result.warning }, false, None);
                } else if !changed {
                    design::save_row_state(ui, Status { icon: icons::OK, word: "No changes", color: colors().muted }, false, None);
                }
                if !changed
                    && self.discarded_options.is_some()
                    && design::draft_discarded(ui, "Rules and runs")
                    && let Some(options) = self.discarded_options.take()
                {
                    snapshot.options = options;
                }
                if snapshot.options != snapshot.saved_options {
                    // FB6: while Save is blocked, Discard is the one way out.
                    design::blocked_reason(
                        ui,
                        Some(if blocked.is_some() {
                            blocked_draft_switch_reason()
                        } else {
                            draft_switch_reason()
                        }),
                    );
                }
                    }};
                }
                macro_rules! save_actions {
                    ($ui:ident) => {{
                        let ui = &mut *$ui;
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
                            design::blocked_reason(ui, blocked.filter(|_| changed));
                            pressed
                        }
                    }};
                }
                // AF1: with one Save, the state starts the row and Save ends it, as on every save
                // row; without room, the state's words wrap before Save. The two run-moving
                // choices need the width, so they take the row below the state.
                let one_row = run_files == 0 || run_open;
                let pressed = if one_row {
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), Density::Scan.control()),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            let pressed = save_actions!(ui);
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center)
                                    .with_main_wrap(true),
                                |ui| save_state!(ui),
                            );
                            pressed
                        },
                    )
                    .inner
                } else {
                    ui.horizontal(|ui| save_state!(ui));
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), Density::Scan.control()),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| save_actions!(ui),
                    )
                    .inner
                };
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
#[cfg(test)]
pub(crate) use list::{ENDED_UNREAD, UNFINISHED_UNREAD};
use list::{
    blocked_reason, choice_note, group_list, history_list, in_progress_list, menu_action,
    ready_list, row_menu, side_control,
};

pub(crate) fn draft_switch_reason() -> &'static str {
    "Save or discard to switch campaign."
}

/// The campaign switch lock of a draft whose Save is blocked: Discard is the one way out, so
/// the reason does not name Save (round 25 FB6).
pub(crate) fn blocked_draft_switch_reason() -> &'static str {
    "Discard to switch campaign."
}

fn roster_detail(detail: &str) -> String {
    let parts: Vec<_> = detail.split(" · ").collect();
    let mut result = Vec::new();
    if let Some(cr) = parts.iter().find(|p| p.starts_with("CR ")) {
        result.push((*cr).to_owned());
    }
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
            .filter(|p| {
                !p.starts_with("CR ")
                    && !p.starts_with("HP ")
                    && !p.starts_with("AC ")
                    && !p.starts_with("Temp ")
            })
            .map(|p| (*p).to_owned()),
    );
    result.join(" · ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_copy_word_names_copies_and_not_the_original() {
        assert_eq!(
            super::run_copy_word("Copy 2 · Closed last · Round 2 · Paused").as_deref(),
            Some("Copy 2")
        );
        assert_eq!(super::run_copy_word("Round 2 · Paused"), None);
        assert_eq!(super::run_copy_word("Copying · Round 1"), None);
    }

    #[test]
    fn iter_2_b_npc_with_deleted_stat_block_stays_in_search_and_cannot_be_added() {
        use egui_kittest::kittest::{NodeT, Queryable};
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let text = splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: world\nsplat_name: Watchkeeper\nsplat_world:\n  type: npc\n  creature: {{id: {}, link: '[[Watchkeeper Guard]]'}}\n", Uuid::new_v4(), Uuid::new_v4()), "").unwrap();
        fs::create_dir_all(dir.path().join("NPCs")).unwrap();
        fs::write(dir.path().join("NPCs/Watchkeeper.md"), text).unwrap();
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        assert!(snapshot.creatures.iter().any(|c| c.name == "Watchkeeper"));
        assert!(
            !snapshot
                .run_creatures
                .iter()
                .any(|creature| creature.entry.name == "Watchkeeper"),
            "the live run cannot add an NPC whose stat block is missing"
        );
        let mut harness = egui_kittest::Harness::new(move |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let chosen = Encounters::creature_search(
                    ui,
                    snapshot.creatures.iter(),
                    &mut "Watchkeeper".to_owned(),
                    &mut false,
                    &mut None,
                    SearchRoster {
                        already: &|_| false,
                    },
                );
                assert!(chosen.is_none());
            });
        });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        assert!(harness.query_by_label("No creatures match").is_none());
        // FD3: the reason is a status, the warning icon and then the words.
        assert!(
            harness
                .query_by_label(&format!(
                    "{} Watchkeeper Guard stat block not found",
                    icons::WARN
                ))
                .is_some(),
            "iter18-B-FD3: the missing stat block reason has its warning icon"
        );
        assert!(harness.query_by_label("Open in World").is_some());
        assert!(harness.get_by_label("Add").accesskit_node().is_disabled());
    }
    #[test]
    fn editor_save_row_has_one_height_in_every_state() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut heights = Vec::new();
                for (dirty, word) in [(true, "Unsaved"), (false, "Saved"), (false, "No changes")] {
                    heights.push(
                        ui.horizontal(|ui| {
                            editor_save_state(
                                ui,
                                Status {
                                    icon: icons::OK,
                                    word,
                                    color: colors().muted,
                                },
                                dirty,
                                None,
                            );
                        })
                        .response
                        .rect
                        .height(),
                    );
                }
                assert_eq!(heights, vec![Density::Read.control(); 3]);
            });
        });
    }

    /// FD3 edge content: unread rows that are not roster rows get no file line; one roster row
    /// and two roster rows with a long path in a narrow column each end with the right sentence,
    /// and the file line starts at the last warning's words with its links whole. The path is
    /// painted whole inside the column: a long one wraps above its links, never cut.
    /// iter-22-B FD5: the warnings come from `unread_notes`. With a file line below them they name
    /// the row and the line, never the path; without one, the warning names the file.
    #[test]
    fn iter21_b_fd3_unread_roster_rows_end_with_one_file_line_at_the_last_words() {
        let long = "Encounters/The Emberfall Marches, Act Two/Shrine of Cinders beneath the Ashen Reaches.md";
        let flag = |line: usize, message: &str| {
            splat::content::layout::Flag::new(line, message.to_owned())
        };
        let row = |name: &str, line: usize| RosterFault {
            name: name.to_owned(),
            line,
            problem: String::new(),
            before: 0,
        };
        // A flag read back from a saved copy has only its sentence: it follows the file or the
        // row, and no line is added, so each names its line once.
        let saved = |line: usize, message: &str| -> splat::content::layout::Flag {
            serde_json::from_value(serde_json::json!({"line": line, "message": message})).unwrap()
        };
        assert_eq!(
            unread_notes(
                "Encounters/Shrine.md",
                &[saved(3, "Line 3: the date was not read.")],
                &[]
            ),
            ["Encounters/Shrine.md: Line 3: the date was not read."]
        );
        assert_eq!(
            unread_notes(
                "Encounters/Shrine.md",
                &[saved(20, "Line 20: the change was not read.")],
                &[row("Ember Cultist", 20)]
            ),
            ["Ember Cultist: Line 20: the change was not read."]
        );
        for (flags, rows, path, expected, sentence) in [
            (
                vec![flag(3, "the date was not read.")],
                vec![],
                "Encounters/Shrine.md",
                vec!["Encounters/Shrine.md, line 3: the date was not read."],
                None,
            ),
            (
                vec![flag(20, "the change was not read.")],
                vec![row("Ember Cultist", 20)],
                "Encounters/Shrine.md",
                vec!["Ember Cultist · line 20: the change was not read."],
                Some(fix_roster_rows(1)),
            ),
            (
                // A flag that is not a roster row, beside two that are: it names its line only.
                vec![
                    flag(20, "the change “HP lots” was not read."),
                    flag(21, "the creature cell is empty."),
                    flag(4, "the date was not read."),
                ],
                vec![row("Ember Cultist", 20), row("Row on line 21", 21)],
                long,
                vec![
                    "Ember Cultist · line 20: the change “HP lots” was not read.",
                    "Row on line 21 · line 21: the creature cell is empty.",
                    "Line 4: the date was not read.",
                ],
                Some(fix_roster_rows(2)),
            ),
        ] {
            let notes = unread_notes(path, &flags, &rows);
            assert_eq!(notes, expected);
            // The run setup and Save's reason give each row the editor's words (iter-23 FA4).
            for row in &rows {
                let flag = flags.iter().find(|flag| flag.line == row.line).unwrap();
                let fault = RosterFault {
                    problem: flag.words().unwrap(),
                    ..row.clone()
                };
                assert!(notes.contains(&fault.words()), "{}", fault.words());
                let reason = roster_fault_reason(path, std::slice::from_ref(&fault)).unwrap();
                assert!(
                    reason.contains(fault.words().trim_end_matches('.')),
                    "{reason}"
                );
            }
            let faults = rows.len();
            let ctx = egui::Context::default();
            design::theme::install(&ctx);
            let mut bounds = egui::Rect::NOTHING;
            let output = ctx.run(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    // The encounter editor's column at 960.
                    let column =
                        egui::Rect::from_min_size(ui.cursor().min, egui::vec2(440.0, 2000.0));
                    ui.scope_builder(egui::UiBuilder::new().max_rect(column), |ui| {
                        bounds = ui.max_rect();
                        unread_roster_rows(
                            ui,
                            std::path::Path::new("/campaign"),
                            path,
                            &notes,
                            faults,
                        );
                    });
                });
            });
            assert!(bounds.width() <= 440.0 + 0.5, "{bounds:?}");
            let painted: Vec<(String, egui::Rect)> = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some((
                        text.galley.job.text.clone(),
                        egui::Rect::from_min_size(text.pos, text.galley.size()),
                    )),
                    _ => None,
                })
                .collect();
            let find = |words: &str| -> Vec<&(String, egui::Rect)> {
                painted
                    .iter()
                    .filter(|(text, _)| text.starts_with(words))
                    .collect()
            };
            let links = find("Edit in Obsidian").len() + find("Show file").len();
            let Some(sentence) = sentence else {
                assert_eq!(
                    links, 0,
                    "a fault that is not a roster row has no file line"
                );
                assert_eq!(find(&notes[0]).len(), 1);
                continue;
            };
            assert_eq!(
                links, 2,
                "one file line for every unread roster row together"
            );
            let ended: Vec<_> = painted
                .iter()
                .filter(|(text, _)| text.ends_with(sentence))
                .collect();
            assert_eq!(ended.len(), 1, "only the last warning says what to do");
            let last = ended[0].1;
            assert!(ended[0].0.starts_with(notes.last().unwrap().as_str()));
            for note in &notes[..notes.len() - 1] {
                let found = find(note);
                assert_eq!(
                    found.len(),
                    1,
                    "each earlier warning is painted once: {note}"
                );
                let (text, rect) = found[0];
                assert!(
                    !text.contains(sentence) && rect.bottom() <= last.top(),
                    "an earlier warning comes first and has no instruction: {text}"
                );
            }
            let shown = find(path);
            assert_eq!(shown.len(), 1, "the path is painted once");
            let (text, place) = shown[0];
            assert_eq!(text, path, "the path is whole, with no ellipsis");
            assert!(
                (place.left() - last.left()).abs() <= 0.5
                    && place.top() >= last.bottom()
                    && place.right() <= bounds.right() + 0.5,
                "the path starts at the words and stays in the column: {place:?} in {bounds:?}"
            );
            for link in ["Edit in Obsidian", "Show file"] {
                let (text, rect) = find(link)[0];
                assert_eq!(text, link, "the link is whole");
                assert!(rect.left() >= last.left() - 0.5 && rect.top() >= last.bottom());
                assert!(
                    rect.right() <= bounds.right() + 0.5
                        && (rect.left() >= place.right() || rect.top() >= place.bottom()),
                    "{link} follows the path inside the column: {rect:?}, path {place:?}"
                );
            }
            let first_on_file_line = painted
                .iter()
                .filter(|(_, rect)| rect.top() >= last.bottom())
                .map(|(_, rect)| rect.left())
                .fold(f32::INFINITY, f32::min);
            assert!(
                (first_on_file_line - last.left()).abs() <= 0.5,
                "the file line starts at the words"
            );
        }
    }

    #[test]
    fn creature_detail_columns_reserve_room_for_their_widest_values() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let output = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                detail_columns(ui, "CR 1/8 · HP 999 · AC 25", Some("Ashen Marches"), false);
            });
        });
        let bounds = |word| {
            output
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.job.text == word => {
                        Some(egui::Rect::from_min_size(text.pos, text.galley.size()))
                    }
                    _ => None,
                })
                .expect("the stat and source text must be painted")
        };
        for (before, after, gap) in [
            ("CR 1/8", "HP 999", design::S),
            ("HP 999", "AC 25", design::S),
            ("AC 25", "Ashen Marches", design::M),
        ] {
            assert!(bounds(after).left() - bounds(before).right() >= gap - 0.5);
        }
    }

    #[test]
    fn i20_w1_search_finds_a_hidden_duplicate_and_repairs_it_with_undo() {
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        creature(dir.path(), "Rook", false);
        let original = fs::read_to_string(dir.path().join("Rook.md")).unwrap();
        fs::write(dir.path().join("Rook copy.md"), &original).unwrap();
        let index = Arc::new(crate::lookup::ContentIndex::load(&[], Some(&active)).unwrap());
        let mut harness = egui_kittest::Harness::new(move |ctx| {
            ctx.data_mut(|data| {
                data.insert_temp(egui::Id::new("reader_link_index"), index.clone())
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                Encounters::creature_search(
                    ui,
                    std::iter::empty(),
                    &mut "Rook".to_owned(),
                    &mut false,
                    &mut None,
                    SearchRoster {
                        already: &|_| false,
                    },
                );
            });
        });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        assert!(harness.query_by_label("No creatures match").is_none());
        harness.get_by_label("These two files claim to be the same entry.");
        harness
            .get_by_label("Make Rook copy.md a separate entry")
            .click();
        harness.run_steps(2);
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        assert_eq!(
            snapshot
                .creatures
                .iter()
                .filter(|c| c.name.starts_with("Rook"))
                .count(),
            2
        );
        harness.get_by_label("Rook copy.md is now a separate entry.");
        harness.get_by_label("Undo separate entry").click();
        harness.run_steps(2);
        assert_eq!(
            fs::read_to_string(dir.path().join("Rook copy.md")).unwrap(),
            original
        );
    }

    #[test]
    fn iter7_b_duplicate_npc_has_aligned_single_repair_choices() {
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let original = splat::content::formats::fixture(
            "Fixture.md",
            &format!(
                "splat_id: {}\nsplat_kind: world\nsplat_name: Rook\nsplat_world: {{type: npc}}\n",
                Uuid::new_v4()
            ),
            "",
        )
        .expect("schema-2 fixture");
        fs::write(dir.path().join("Rook.md"), &original).unwrap();
        fs::write(dir.path().join("Rook copy.md"), &original).unwrap();
        let index = Arc::new(crate::lookup::ContentIndex::load(&[], Some(&active)).unwrap());
        let mut harness = egui_kittest::Harness::new(move |ctx| {
            ctx.data_mut(|data| {
                data.insert_temp(egui::Id::new("reader_link_index"), index.clone())
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                Encounters::creature_search(
                    ui,
                    std::iter::empty(),
                    &mut "Rook".to_owned(),
                    &mut false,
                    &mut None,
                    SearchRoster {
                        already: &|_| false,
                    },
                );
            });
        });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        assert_eq!(
            harness
                .query_all_by_label("Make Rook copy.md a separate note")
                .count(),
            1
        );
        assert_eq!(
            harness
                .query_all_by_label("Make Rook.md a separate note")
                .count(),
            1
        );
        assert_eq!(
            harness
                .query_all_by_label("These two files claim to be the same note.")
                .count(),
            1
        );
        let warning = harness
            .get_by_label("These two files claim to be the same note.")
            .rect();
        for label in [
            "Make Rook copy.md a separate note",
            "Make Rook.md a separate note",
        ] {
            let action = harness.get_by_label(label).rect();
            assert!(
                (action.left() - warning.left()).abs() < 1.0,
                "iter7-B-FD3: repair {action:?} must align with warning text {warning:?}"
            );
        }
    }

    #[test]
    fn iter7_b_roster_keeps_unsent_notes_focus_and_scroll_across_parent_changes() {
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let draft = std::rc::Rc::new(std::cell::RefCell::new(Draft::new()));
        let shown = draft.clone();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(960.0, 640.0))
            .build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let parent = if ui.available_width() > 760.0 {
                        "wide"
                    } else {
                        "compact"
                    };
                    ui.push_id(parent, |ui| {
                        Encounters::roster_pane(
                            ui,
                            &mut shown.borrow_mut(),
                            &snapshot,
                            &mut false,
                            &mut None,
                            None,
                            &mut None,
                            parent == "compact",
                        );
                    });
                });
            });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        harness
            .query_all_by_label("Read-aloud and notes")
            .max_by(|a, b| a.rect().width().total_cmp(&b.rect().width()))
            .unwrap()
            .click();
        harness.event(egui::Event::Text("Unsent tactics.\n".repeat(60)));
        harness.run_steps(2);
        let focus = harness.ctx.memory(|memory| memory.focused()).unwrap();
        let text = draft.borrow().body.clone();
        assert!(text.contains("Unsent tactics."));
        // Outer scroll moves down to the caret; keep that same scroll state when the parent changes.
        let scroll_id = egui::Id::new(("encounter_roster", draft.borrow().id))
            .with(egui::Id::new("encounter_editor"));
        let offset = egui::scroll_area::State::load(&harness.ctx, scroll_id)
            .unwrap()
            .offset
            .y;
        assert!(offset > 0.0, "Exercise a scrolled editor");
        for width in [600.0, 960.0] {
            harness.set_size(egui::vec2(width, 640.0));
            harness.run_steps(2);
            assert_eq!(draft.borrow().body, text, "Unsent text survives");
            assert_eq!(
                harness.ctx.memory(|memory| memory.focused()),
                Some(focus),
                "The same notes control keeps focus"
            );
            let current = egui::scroll_area::State::load(&harness.ctx, scroll_id)
                .unwrap()
                .offset
                .y;
            assert!(
                (current - offset).abs() < 1.0,
                "Scroll survives parent changes: {offset} -> {current}"
            );
        }
    }

    #[test]
    fn iter7_b_roster_scroll_content_keeps_list_measure_and_warning_words_muted() {
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let id = creature(dir.path(), "Cinder Wolf", false);
        fs::copy(
            dir.path().join("Cinder Wolf.md"),
            dir.path().join("Cinder Wolf copy.md"),
        )
        .unwrap();
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let mut draft = Draft::new();
        draft.add(id, false);
        draft.add(id, false);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(1200.0, 1080.0))
            .build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    Encounters::roster_pane(
                        ui, &mut draft, &snapshot, &mut false, &mut None, None, &mut None, false,
                    );
                });
            });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        let notes = harness
            .query_all_by_label("Read-aloud and notes")
            .map(|node| node.rect())
            .max_by(|a, b| a.width().total_cmp(&b.width()))
            .unwrap();
        // Width is checked after the colour, so each fault has its own red evidence.
        let warnings: Vec<_> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                // Round 24 FA3: each row says the short cause; the file names are on its hover.
                egui::Shape::Text(text)
                    if text.galley.text().contains(crate::library::SAME_ENTRY) =>
                {
                    Some(text)
                }
                _ => None,
            })
            .collect();
        assert_eq!(
            warnings.len(),
            2,
            "Two repeated roster warnings must be checked"
        );
        for text in warnings {
            let mut marked = 0;
            for section in &text.galley.job.sections {
                let words = &text.galley.text()[section.byte_range.clone()];
                if section.format.color == colors().muted {
                    continue;
                }
                // Round 23 AD2: two files that claim one identity are a warning in the row, as in
                // Library and in the message above the roster. Only an unreadable file is an error.
                assert!(
                    words.trim() == icons::WARN && section.format.color == colors().result.warning,
                    "iter7-B-FD8: the fault colour is only for the icon: {words}"
                );
                marked += 1;
            }
            assert_eq!(marked, 1, "iter-19-B-FD5: each row marks its fault once");
        }
        assert!(
            notes.width() <= design::LIST_MEASURE,
            "iter7-B-AT1: {notes:?}"
        );
    }

    /// The replacement result names what changed, with a count when it replaced several rows.
    #[test]
    fn iter18_b_replacement_words_name_the_rows() {
        assert_eq!(
            replacement_words("Cinder Wolf", 2, "Guard"),
            "2 Cinder Wolf rows replaced with Guard."
        );
        assert_eq!(
            replacement_words("Cinder Wolf", 1, "Guard"),
            "Cinder Wolf replaced with Guard."
        );
    }

    /// FD4: the loot row and the notes field end at the roster's right edge, one column edge.
    #[test]
    fn iter18_b_loot_and_notes_end_at_the_roster_edge() {
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let id = creature(dir.path(), "Cinder Wolf", false);
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let mut draft = Draft::new();
        draft.add(id, false);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(1200.0, 1080.0))
            .build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    Encounters::roster_pane(
                        ui, &mut draft, &snapshot, &mut false, &mut None, None, &mut None, false,
                    );
                });
            });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        // The roster's row rule marks the column's right edge.
        let rule = harness
            .output()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::LineSegment { points, .. } if points[0].y == points[1].y => {
                    Some(points[1].x.max(points[0].x))
                }
                _ => None,
            })
            .reduce(f32::max)
            .expect("the roster draws a row rule");
        for name in [
            "More actions for Cinder Wolf",
            "Roll on a table for the encounter",
            "Read-aloud and notes",
        ] {
            // The field's caption carries the same name; the widest node is the control.
            let right = harness
                .query_all_by_label(name)
                .map(|node| node.rect().right())
                .reduce(f32::max)
                .unwrap();
            assert!(
                (right - rule).abs() <= 0.5,
                "iter18-B-FD4: {name} ends at {right}, the roster rule at {rule}"
            );
        }
        // Must stay true: the loot field keeps its gap before the menu.
        let field = harness.get_by_label("Add loot to the encounter").rect();
        let menu = harness
            .get_by_label("Roll on a table for the encounter")
            .rect();
        assert!(
            menu.left() - field.right() >= design::XS,
            "iter18-B-FD4: the loot field and its menu stay apart: {field:?} {menu:?}"
        );
    }

    #[test]
    fn iter8_b_removed_source_has_no_ellipsis() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let detail = "Enemies · CR 2 · HP 43 · AC 15 · A very long source name from the Ashen Marches · as Ember Cultist";
                let fitted = changed_detail_parts(ui, detail, "Changed · 1 to add by hand", 400.0, false).0;
                assert!(!fitted.contains("Ashen Marches"));
                assert!(!fitted.contains('…'), "iter8-B-FT8: {fitted}");
                assert!(fitted.contains("as Ember Cultist"));
                // iter21-B-FD1: the HP fault is data. Shortened, it outlasts a package that has
                // its words; with hit points, those words are only a package name.
                let fault = format!("{} {HP_NOT_SET}", design::icons::WARN);
                let missing = format!(
                    "Enemies · CR 2 · {fault} · AC 15 · {fault} · as Ember Cultist"
                );
                let (fitted, kept) =
                    changed_detail_parts(ui, &missing, "Changed · 1 to add by hand", 500.0, true);
                assert!(kept, "iter21-B-FD1: {fitted}");
                assert_eq!(fitted.matches(fault.as_str()).count(), 1, "iter21-B-FD1: {fitted}");
                let parts = detail_parts(&fitted, kept);
                assert_eq!(
                    parts.iter().position(|part| part.hp_fault),
                    Some(2),
                    "iter21-B-FD1: the HP slot is the fault: {fitted}"
                );
                let healthy = format!("Enemies · CR 2 · HP 43 · AC 15 · {fault}");
                let (fitted, kept) =
                    changed_detail_parts(ui, &healthy, "Changed", 400.0, false);
                assert!(!kept && fitted.contains(fault.as_str()), "iter21-B-FD1: {fitted}");
                assert!(detail_parts(&healthy, false).iter().all(|part| !part.hp_fault));
            });
        });
    }

    #[test]
    fn iter8_b_clean_creature_uses_shared_status() {
        use egui_kittest::kittest::{NodeT, Queryable};
        let dir = tempfile::tempdir().unwrap();
        creature(dir.path(), "Wolf", false);
        let mut editor = CreatureEditor::open(dir.path(), "Wolf.md").unwrap();
        let mut harness = egui_kittest::Harness::new(move |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                Encounters::creature_pane(ui, &mut editor);
            });
        });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        harness.get_by_label("Done with this creature");
        let save = harness.get_by_label("Save creature");
        assert!(save.accesskit_node().is_disabled());
        let texts: Vec<_> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.text()),
                _ => None,
            })
            .collect();
        assert!(
            texts
                .iter()
                .any(|text| text.contains("No changes") && text.contains(icons::OK)),
            "iter8-B-FT5: shared clean status: {texts:?}"
        );
        assert!(
            !texts
                .iter()
                .any(|text| text.contains("No changes to save."))
        );
    }

    #[test]
    fn iter_2_b_changed_row_keeps_base_and_full_hand_change_link() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let detail = "Enemies · CR 2 · HP 43 · AC 15 · A very long source name from the Ashen Marches · as Ember Cultist";
                let word = "Changed · 1 to add by hand";
                for width in [280.0, 320.0, 356.0, 400.0] {
                    let fitted = changed_detail_parts(ui, detail, word, width, false).0;
                    assert!(fitted.contains("as Ember Cultist"), "{fitted}");
                    assert!(!fitted.contains("Ashen Marches"), "{fitted}");
                    assert!(!fitted.contains(word));
                    let font = Type::Small.font(ui);
                    let used = design::number_width(ui, &fitted, &font)
                        + design::number_width(ui, word, &font)
                        + design::number_width(ui, " · ", &font);
                    assert!(used <= width, "{used} exceeds {width}");
                }
            });
        });
    }
    #[test]
    fn roster_stats_use_the_search_order_and_keep_other_details_last() {
        assert_eq!(
            roster_detail("HP 11 · AC 12 · CR 1/4 · Temp 5 · as Guard"),
            format!("CR 1/4 · HP 11 {} +5 · AC 12 · as Guard", icons::TEMP_HP)
        );
        assert_eq!(roster_detail("HP 11 · AC 12"), "HP 11 · AC 12");
    }
    #[test]
    fn missing_roster_names_never_fall_back_to_reference_keys() {
        let mut draft = Draft::new();
        let id = Uuid::new_v4();
        draft.add(id, false);
        let slot = &mut draft.setup.roster[0];
        slot.key = "actor_wolf_1".into();
        slot.party_slot = Some("party_slot_internal".into());
        for reference in [
            EntryRef::id(id),
            EntryRef {
                id: None,
                key: Some("package:wolf".into()),
                link: None,
            },
        ] {
            slot.creature = Some(reference);
            let mut known = BTreeMap::new();
            let label = roster_name(slot, None, &known);
            assert_eq!(label, "Missing creature");
            for raw in [
                &slot.key,
                slot.party_slot.as_ref().unwrap(),
                &slot.creature.as_ref().unwrap().label(),
            ] {
                assert!(!label.contains(raw));
            }
            known.insert(
                slot.creature.as_ref().unwrap().label(),
                "Cinder Wolf 1".into(),
            );
            assert_eq!(roster_name(slot, None, &known), "Cinder Wolf 1");
        }
        slot.overrides = yaml("{name: Last known wolf}")
            .as_mapping()
            .unwrap()
            .clone();
        assert_eq!(roster_name(slot, None, &BTreeMap::new()), "Last known wolf");
    }

    #[test]
    fn edited_roster_rechecks_a_failed_start_and_undo_restores_its_reason() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let id = creature(dir.path(), "Cinder Wolf", false);
        let old = Snapshot::load(&active, &[]).unwrap();
        fs::remove_file(dir.path().join("Cinder Wolf.md")).unwrap();
        let mut snapshot = Snapshot::load(&active, &[]).unwrap();
        snapshot.keep_edited_settings(&old);
        let mut draft = Draft::new();
        draft.add(id, false);
        draft.add(id, false);
        let error = draft.validate(&snapshot).unwrap_err();
        assert!(
            error
                .text
                .starts_with("2 Cinder Wolf rows have no creature.")
        );
        let mut encounters = Encounters {
            snapshot: Some(snapshot),
            validation_path: Some(draft.path.clone()),
            draft: Some(draft),
            error: Some(error.text.clone()),
            validation_error: Some(error),
            refusal_watch: true,
            ..Default::default()
        };
        encounters.draft.as_mut().unwrap().setup.roster.pop();
        encounters.refresh_validation_error(false, true);
        assert!(
            encounters
                .error
                .as_ref()
                .unwrap()
                .starts_with("Cinder Wolf has no creature.")
        );
        let missing = encounters.draft.as_ref().unwrap().setup.roster.clone();
        encounters.draft.as_mut().unwrap().setup.roster.clear();
        encounters.refresh_validation_error(false, true);
        assert!(encounters.error.is_none());
        encounters.draft.as_mut().unwrap().setup.roster = missing;
        encounters.refresh_validation_error(false, true);
        assert!(
            encounters
                .error
                .as_ref()
                .unwrap()
                .starts_with("Cinder Wolf has no creature.")
        );

        // Round 24 review: drawn frames check the refused draft again only when it or the
        // campaign changes: not on unchanged frames or a resize; once after an edit, Discard and
        // Undo. The work is counted inside `Draft::validate`.
        use crate::test_support::{CachedWork, Counts};
        let settings = dir.path().join("settings.json");
        let mut store = crate::persistence::Store::at(settings.clone());
        let mut data = store.load().unwrap();
        data.active_campaign_id = Some(active.id);
        data.campaigns.push(active.clone());
        store.save(&data).unwrap();
        let campaigns = Campaigns::at(settings);
        encounters.active = Some(active.clone());
        let ctx = egui::Context::default();
        let frame = |encounters: &mut Encounters, size: egui::Vec2, events: Vec<egui::Event>| {
            let _ = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    events,
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        encounters.show(
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
        };
        let key = || {
            vec![egui::Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }]
        };
        let shows = |encounters: &Encounters, words: &str| {
            encounters
                .error
                .as_deref()
                .is_some_and(|error| error.starts_with(words))
        };
        let (wide, narrow) = (egui::vec2(1200.0, 800.0), egui::vec2(960.0, 640.0));
        let check = CachedWork::start("Encounter refusal check", &mut encounters, |e| Counts {
            reached: e.validation_reached,
            work: VALIDATIONS.with(std::cell::Cell::get),
        });
        frame(&mut encounters, wide, Vec::new());
        let shown = shows(&encounters, "Cinder Wolf has no creature.");
        check
            .warm(&mut encounters, shown, 1)
            .expect("refusal-check-warm");
        check
            .steady(&mut encounters, |encounters| {
                for size in [wide, wide, wide, narrow, wide] {
                    frame(encounters, size, Vec::new());
                }
                shows(encounters, "Cinder Wolf has no creature.")
            })
            .expect("refusal-check-steady");
        // An edit: the second row comes back, and the reason counts both.
        let row = encounters.draft.as_ref().unwrap().setup.roster[0].clone();
        check
            .refresh(&mut encounters, 1, |encounters| {
                encounters.draft.as_mut().unwrap().setup.roster.push(row);
                frame(encounters, wide, key());
                frame(encounters, wide, Vec::new());
                shows(encounters, "2 Cinder Wolf rows have no creature.")
            })
            .expect("refusal-check-edit");
        // Discard: a new draft has no saved encounter to go back to, so its refusal goes with it.
        check
            .refresh(&mut encounters, 0, |encounters| {
                encounters.discard_draft(&active);
                frame(encounters, wide, key());
                frame(encounters, wide, Vec::new());
                encounters.draft.is_none() && encounters.error.is_none()
            })
            .expect("refusal-check-discard");
        // Undo: the draft comes back and its reason with it.
        check
            .refresh(&mut encounters, 1, |encounters| {
                encounters.undo_discard();
                frame(encounters, wide, key());
                frame(encounters, wide, Vec::new());
                shows(encounters, "2 Cinder Wolf rows have no creature.")
            })
            .expect("refusal-check-undo");
        check
            .steady(&mut encounters, |encounters| {
                for size in [wide, narrow, wide] {
                    frame(encounters, size, Vec::new());
                }
                shows(encounters, "2 Cinder Wolf rows have no creature.")
            })
            .expect("refusal-check-steady-after-undo");
        // A check that passes clears the stored refusal.
        encounters.draft.as_mut().unwrap().setup.roster.clear();
        frame(&mut encounters, wide, key());
        frame(&mut encounters, wide, Vec::new());
        assert!(
            encounters.error.is_none() && encounters.validation_error.is_none(),
            "refusal-cleared-on-pass"
        );
    }

    #[test]
    fn a_refresh_keeps_the_name_of_a_deleted_creature() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let id = creature(dir.path(), "Cinder Wolf", false);
        let old = Snapshot::load(&active, &[]).unwrap();
        fs::remove_file(dir.path().join("Cinder Wolf.md")).unwrap();
        let mut refreshed = Snapshot::load(&active, &[]).unwrap();
        refreshed.keep_edited_settings(&old);
        let mut draft = Draft::new();
        draft.add(id, false);
        assert_eq!(
            roster_name(
                &draft.setup.roster[0],
                None,
                &refreshed.known_creature_names
            ),
            "Cinder Wolf"
        );
    }

    #[test]
    fn bulk_replacement_keeps_names_and_starting_values() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let wolf = creature(dir.path(), "Cinder Wolf", false);
        let guard = creature(dir.path(), "Guard", false);
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let mut draft = Draft::new();
        draft.add(wolf, false);
        draft.add(wolf, false);
        draft.setup.roster[0].initial.insert("hp".into(), 7.into());
        let before = draft.setup.roster.clone();
        replace_roster(&mut draft.setup.roster, 0, guard, true, &snapshot);
        assert!(
            draft
                .setup
                .roster
                .iter()
                .all(|slot| slot.creature == Some(EntryRef::id(guard)))
        );
        assert!(
            draft
                .setup
                .roster
                .iter()
                .all(|slot| slot.overrides["name"] == Value::String("Cinder Wolf".into()))
        );
        assert_eq!(draft.setup.roster[0].initial, before[0].initial);
        assert_eq!(draft.setup.roster[0].side, before[0].side);
    }

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
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Test campaign".into(),
            path: root.into(),
            dndbeyond_campaign_id: Some("123".into()),
            dndbeyond_campaign_name: None,
        };
        if campaign.path.is_dir() && !campaign.path.join("Campaign.md").exists() {
            crate::content_workspace::CampaignDocument::new(&campaign)
                .unwrap()
                .save()
                .unwrap();
        }
        campaign
    }
    fn creature(root: &std::path::Path, name: &str, party: bool) -> Uuid {
        let text = crate::library::note_text(name, "creature", None)
            .replace("| Hit Points |  |", "| Hit Points | 1 |")
            .replace("| Armor Class |  |", "| Armor Class | 10 |");
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
        let text = splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: world\nsplat_name: Rook Ambry\nsplat_world:\n  type: npc\n  creature: {{id: {stat}}}\n", Uuid::new_v4()), "").expect("schema-2 fixture");
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
        let result = snapshot
            .creatures
            .iter()
            .find(|creature| creature.id == npc)
            .unwrap();
        assert!(snapshot.already_on_roster(&draft, result));
        assert!(!snapshot.already_on_roster(&Draft::new(), result));
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
    fn iter2_d_duplicate_npc_keeps_its_name_classification() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let stat = creature(dir.path(), "Guard", false);
        let text = splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: world\nsplat_name: Rook Ambry\nsplat_world:\n  type: npc\n  creature: {{id: {stat}}}\n", Uuid::new_v4()), "").unwrap();
        write_note(dir.path(), "Rook.md", None, &text).unwrap();
        let overrides = yaml("{name: Rook Ambry}").as_mapping().unwrap().clone();
        for copies in 0..=2 {
            if copies > 0 {
                fs::write(dir.path().join(format!("Rook copy {copies}.md")), &text).unwrap();
            }
            let snapshot = Snapshot::load(&active, &[]).unwrap();
            let guard = &snapshot.catalogue.entries[&stat].entry;
            assert_eq!(
                snapshot.npc_name_for(guard, &overrides).as_deref(),
                Some("Rook Ambry"),
                "copy count: {copies}"
            );
            let other_name = yaml("{name: Someone else}").as_mapping().unwrap().clone();
            assert_eq!(snapshot.npc_name_for(guard, &other_name), None);
            let other_stat = snapshot
                .catalogue
                .entries
                .values()
                .find(|item| item.entry.kind == "creature" && item.entry.id != stat)
                .unwrap();
            assert_eq!(snapshot.npc_name_for(&other_stat.entry, &overrides), None);
            if copies > 0 {
                assert!(!snapshot.catalogue.duplicate_world.is_empty());
            }
        }
    }

    #[test]
    fn empty_search_puts_campaign_creatures_before_equal_book_names() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let own = creature(dir.path(), "Aboleth", false);
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let matches: Vec<_> = snapshot
            .creatures
            .iter()
            .filter(|creature| creature.name == "Aboleth")
            .collect();
        assert!(matches.len() >= 2);
        assert_eq!(matches[0].id, own);
        assert_eq!(matches[1].origin, crate::lookup::Origin::Book);
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
    fn the_loot_field_owns_a_plain_loot_list_and_leaves_any_other_alone() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let mut d = Draft::new();
        d.body = "Read this.".into();
        d.loot = vec!["30 gp".into(), "Roll 2× gems".into()];
        d.save(&campaign, &[]).unwrap();
        let saved = fs::read_to_string(dir.path().join(&d.path)).unwrap();
        assert!(
            saved.contains("Read this.\n\n## Loot\n\n- 30 gp\n- Roll 2× gems\n"),
            "{saved}"
        );
        let reopened = Draft::open(dir.path(), &d.path).unwrap();
        assert_eq!(
            (reopened.body.as_str(), reopened.loot.join("\n").as_str()),
            ("Read this.", "30 gp\nRoll 2× gems")
        );
        assert!(!reopened.dirty());
        // A section with more than a list stays in the notes, where nothing in it is lost.
        let mixed = saved.replace("- 30 gp\n", "In the chest:\n- 30 gp\n");
        fs::write(dir.path().join(&d.path), &mixed).unwrap();
        let mut reopened = Draft::open(dir.path(), &d.path).unwrap();
        assert!(!reopened.loot_owned);
        assert!(reopened.body.contains("In the chest:"));
        reopened.body.push_str("\nMore.");
        reopened.save(&campaign, &[]).unwrap();
        let saved = fs::read_to_string(dir.path().join(&d.path)).unwrap();
        assert!(
            saved.contains("In the chest:\n- 30 gp\n- Roll 2× gems\nMore."),
            "{saved}"
        );
        // A Loot section between others keeps its place, and a second one stays as written.
        let middle = saved
            .replace("In the chest:\n", "")
            .replace("\nMore.", "\n\n## Tactics\n\nHide.");
        fs::write(dir.path().join(&d.path), &middle).unwrap();
        let mut reopened = Draft::open(dir.path(), &d.path).unwrap();
        assert!(reopened.loot_owned);
        reopened.loot.push("A map".into());
        reopened.save(&campaign, &[]).unwrap();
        let saved = fs::read_to_string(dir.path().join(&d.path)).unwrap();
        assert!(
            saved.contains("## Loot\n\n- 30 gp\n- Roll 2× gems\n- A map\n\n## Tactics\n\nHide."),
            "{saved}"
        );
        let twice = saved.replace("Hide.", "Hide.\n\n## Loot\n\n- A second list");
        fs::write(dir.path().join(&d.path), &twice).unwrap();
        assert!(!Draft::open(dir.path(), &d.path).unwrap().loot_owned);
        // A Loot heading typed in the notes would be lost, so the save says so.
        fs::write(dir.path().join(&d.path), &saved).unwrap();
        let mut typed = Draft::open(dir.path(), &d.path).unwrap();
        typed.body = "## Loot\n\nIn the chest: a map".into();
        assert!(
            typed
                .save(&campaign, &[])
                .unwrap_err()
                .contains("Loot heading")
        );
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
    fn integrity_d8_a_moved_encounter_keeps_the_draft_and_identity() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = creature(dir.path(), "Monster", false);
        let mut draft = Draft::new();
        draft.add(id, false);
        draft.save(&campaign, &[]).unwrap();
        let old = draft.path.clone();
        let moved = "Encounters/Moved.md";
        fs::rename(dir.path().join(&old), dir.path().join(moved)).unwrap();
        draft.body = "My unsaved draft".into();
        draft.save(&campaign, &[]).unwrap();
        assert_eq!(draft.path, moved);
        assert!(!dir.path().join(old).exists());
        assert!(
            fs::read_to_string(dir.path().join(moved))
                .unwrap()
                .contains("My unsaved draft")
        );
    }

    #[test]
    fn integrity_d8_a_changed_encounter_compares_and_keeps_unknown_fields() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let mut draft = Draft::new();
        draft.add(creature(dir.path(), "Monster", false), false);
        draft.save(&campaign, &[]).unwrap();
        let path = dir.path().join(&draft.path);
        let external = fs::read_to_string(&path)
            .unwrap()
            .replace("splat_schema:", "tags: [DM]\nsplat_schema:");
        fs::write(&path, &external).unwrap();
        draft.body = "My words".into();
        assert!(draft.save(&campaign, &[]).is_err());
        assert_eq!(
            draft.recovery.change.as_ref().unwrap().text.as_deref(),
            Some(external.as_str())
        );
        let change = draft.recovery.accept().unwrap();
        draft.source = change.text.clone().unwrap();
        draft.baseline = change.text;
        draft.save(&campaign, &[]).unwrap();
        let saved = fs::read_to_string(&path).unwrap();
        assert!(saved.contains("tags: [DM]") && saved.contains("My words"));

        use egui_kittest::kittest::Queryable;
        // FF7: the file goes while the draft is open. The red choice says what it removes, its
        // result says what was lost, and Undo remove gives the draft back.
        fs::remove_file(&path).unwrap();
        draft.body = "My later words".into();
        assert!(draft.save(&campaign, &[]).is_err());
        assert!(draft.recovery.file_gone());
        let name = draft.name.clone();
        let editor = Encounters {
            draft: Some(draft),
            snapshot: Some(Snapshot::load(&campaign, &[]).unwrap()),
            ..Default::default()
        };
        let mut harness = egui_kittest::Harness::builder().build_state(
            |ctx, editor: &mut Encounters| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    editor.discard_undo(ui);
                    if !editor.encounter_disk_view(ui, &campaign, &[]) {}
                });
            },
            editor,
        );
        harness.run_steps(2);
        assert!(harness.query_by_label("Let it go").is_none());
        harness
            .get_by_label("Remove encounter and lose draft")
            .click();
        harness.run_steps(2);
        assert!(harness.state().draft.is_none());
        harness.get_by_label(&format!("{name} removed. Its draft was discarded."));
        harness.get_by_label("Undo remove").click();
        harness.run_steps(2);
        let back = harness
            .state()
            .draft
            .as_ref()
            .expect("Undo remove gives the draft back");
        assert_eq!(back.body, "My later words");
        assert!(harness.state().discarded.is_none());
        assert!(!path.exists(), "nothing was written");
    }

    #[test]
    fn i19_e_encounter_comparison_matches_disk_layout_and_overwrite() {
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let mut draft = Draft::new();
        draft.add(creature(dir.path(), "Monster", false), false);
        draft.save(&campaign, &[]).unwrap();
        let path = dir.path().join(&draft.path);
        let external = format!(
            "{}\nDM's outside account.\n",
            fs::read_to_string(&path)
                .unwrap()
                .replace("splat_schema:", "tags: [dawn]\nsplat_schema:")
        );
        fs::write(&path, &external).unwrap();
        draft.name = "Dawn".into();
        assert!(draft.save(&campaign, &[]).is_err());
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let editor = Encounters {
            draft: Some(draft),
            snapshot: Some(Snapshot::load(&campaign, &[]).unwrap()),
            ..Default::default()
        };
        let mut harness = egui_kittest::Harness::builder().build_state(
            |ctx, editor: &mut Encounters| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if !editor.encounter_disk_view(ui, &campaign, &[]) {}
                });
            },
            editor,
        );
        harness.run_steps(2);
        harness.get_by_label("1 line differs");
        let comparison = harness
            .state()
            .draft
            .as_ref()
            .unwrap()
            .comparison_text(&snapshot)
            .unwrap();
        assert_eq!(
            comparison,
            external.replace("splat_name: New encounter", "splat_name: Dawn")
        );
        let overwrite = splat::content::list_formats::write_references(
            &comparison,
            snapshot
                .catalogue
                .entries
                .values()
                .map(|entry| &entry.entry),
        )
        .unwrap();
        assert!(comparison.contains("tags: [dawn]"));
        assert!(comparison.contains("DM's outside account."));
        harness.get_by_label("Overwrite with my draft").click();
        harness.run_steps(2);
        assert_eq!(harness.state().error, None);
        assert_eq!(fs::read_to_string(&path).unwrap(), overwrite);
    }

    #[test]
    fn fmt_f2_creature_editor_preserves_bytes_outside_the_hp_cell() {
        let dir = tempfile::tempdir().unwrap();
        let text = crate::library::note_text("Wolf", "creature", None)
            .replace("| Hit Points |  |", "| Hit Points | 22 (4d8 + 4) |")
            .replace("| Armor Class |  |", "| Armor Class | 13 |")
            .replace("## Actions", "%% My comment %%\n\n## Actions")
            .replace('\n', "\r\n");
        fs::write(dir.path().join("Wolf.md"), &text).unwrap();
        let mut editor = CreatureEditor::open(dir.path(), "Wolf.md").unwrap();
        editor.fields.hp = "23".into();
        editor.save(dir.path()).unwrap();
        assert_eq!(
            fs::read_to_string(dir.path().join("Wolf.md")).unwrap(),
            text.replace("| Hit Points | 22", "| Hit Points | 23")
        );
    }

    #[test]
    fn integrity_d8_creature_drafts_follow_moves_and_compare_edits() {
        let dir = tempfile::tempdir().unwrap();
        creature(dir.path(), "Monster", false);
        let mut editor = CreatureEditor::open(dir.path(), "Monster.md").unwrap();
        fs::rename(dir.path().join("Monster.md"), dir.path().join("Moved.md")).unwrap();
        editor.fields.hp = "19".into();
        editor.save(dir.path()).unwrap();
        assert_eq!(editor.path, "Moved.md");
        let outside = format!(
            "{}\nDM's words",
            fs::read_to_string(dir.path().join("Moved.md")).unwrap()
        );
        fs::write(dir.path().join("Moved.md"), &outside).unwrap();
        editor.fields.hp = "20".into();
        assert!(editor.save(dir.path()).is_err());
        assert_eq!(
            editor.recovery.change.as_ref().unwrap().text.as_deref(),
            Some(outside.as_str())
        );
        assert_eq!(
            fs::read_to_string(dir.path().join("Moved.md")).unwrap(),
            outside
        );
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
        // iter-24-B AB1, FB1: Save encounter writes the notes to the encounter and, in the same
        // step, to the open run's Prep. The run's combat state (damage, a condition) and its
        // setup details stay, and the encounter keeps its roster.
        editor.run.as_mut().unwrap().hurt_for_test(0, 3, "Prone");
        let run = editor.run.as_ref().unwrap();
        let (stats, combat) = (run.participant_stats(0), run.combat_state_for_test(0));
        let technical = run.prep().1.map(str::to_owned);
        assert!(combat.0.is_some() && combat.1.contains(&"Prone".to_owned()));
        assert_ne!(run.prep().0, "The bridge groans.");
        editor.draft.as_mut().unwrap().body = "The bridge groans.".into();
        editor.save_draft(&egui::Context::default(), &campaign, &[]);
        let saved = fs::read_to_string(&path).unwrap();
        assert!(saved.contains("The bridge groans.") && saved.contains("Hero"));
        let run = editor.run.as_ref().unwrap();
        assert_eq!(run.prep().0, "The bridge groans.");
        assert_eq!(run.prep().1.map(str::to_owned), technical);
        assert_eq!(run.participant_stats(0), stats);
        assert_eq!(run.combat_state_for_test(0), combat);
        editor.run = None;
        assert!(!editor.dirty());
        fs::write(&path, format!("{original}External edit\n")).unwrap();
        assert!(editor.start_run(&campaign, &[]).is_err());
    }
    /// The missing-creature message takes its tone from the kind of fault, where `validate` finds it.
    #[test]
    fn a_missing_creature_message_takes_its_tone_from_the_fault_kind() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let id = creature(dir.path(), "Cinder Wolf", false);
        let old = Snapshot::load(&active, &[]).unwrap();
        let file = dir.path().join("Cinder Wolf.md");
        let original = fs::read_to_string(&file).unwrap();
        fs::remove_file(&file).unwrap();
        let mut deleted = Snapshot::load(&active, &[]).unwrap();
        deleted.keep_edited_settings(&old);
        let mut draft = Draft::new();
        draft.add(id, false);
        let invalid = draft.validate(&deleted).unwrap_err();
        assert!(invalid.text.ends_with(REPLACE_DELETED_ENTRY), "{invalid:?}");
        assert!(invalid.tone == Tone::Warning);
        // The row says the same fault in the same tone.
        assert!(invalid.tone == missing_creature_tone(None));
        let mut encounters = Encounters {
            snapshot: Some(deleted),
            validation_path: Some(draft.path.clone()),
            draft: Some(draft),
            error: Some(invalid.text.clone()),
            validation_error: Some(invalid.clone()),
            refusal_watch: true,
            ..Default::default()
        };
        assert!(encounters.error_tone() == Tone::Warning);
        // Another message with the same last sentence is not that fault, so it is an error.
        encounters.error = Some(format!("The file did not save. {REPLACE_DELETED_ENTRY}"));
        assert!(encounters.error_tone() == Tone::Error);
        encounters.validation_error = None;
        assert!(encounters.error_tone() == Tone::Error);
        // The fault ends when the row goes: no message stays behind.
        encounters.error = Some(invalid.text.clone());
        encounters.validation_error = Some(invalid);
        let roster = encounters.draft.as_ref().unwrap().setup.roster.clone();
        encounters.draft.as_mut().unwrap().setup.roster.clear();
        encounters.refresh_validation_error(false, true);
        assert!(encounters.error.is_none());
        let mut draft = encounters.draft.take().unwrap();
        draft.setup.roster = roster;
        // A file that cannot be read is an error, in the message and in the row.
        let broken = original.replace("splat_name: Cinder Wolf", "splat_name: [Cinder Wolf");
        assert_ne!(broken, original);
        fs::write(&file, broken).unwrap();
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let invalid = draft.validate(&snapshot).unwrap_err();
        assert!(
            snapshot
                .catalogue
                .unreadable_ref(draft.setup.roster[0].creature.as_ref().unwrap())
                .is_some(),
            "{invalid:?}"
        );
        assert!(invalid.tone == Tone::Error, "{invalid:?}");
        let unread = snapshot
            .catalogue
            .unreadable_ref(draft.setup.roster[0].creature.as_ref().unwrap());
        assert!(invalid.tone == missing_creature_tone(unread));
        assert_eq!(missing_creature_mark(unread).0, icons::ERROR);
        encounters.snapshot = Some(snapshot);
        encounters.draft = Some(draft);
        encounters.error = Some(invalid.text.clone());
        encounters.validation_error = Some(invalid);
        assert!(encounters.error_tone() == Tone::Error);
        // Round 23 AD2: two files that claim one identity are a warning, as Library shows them,
        // in the message and in the row. The fight stays blocked until the identity is clear.
        fs::write(&file, &original).unwrap();
        fs::write(dir.path().join("Cinder Wolf copy.md"), &original).unwrap();
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let draft = encounters.draft.as_ref().unwrap();
        let invalid = draft.validate(&snapshot).unwrap_err();
        let duplicate = snapshot
            .catalogue
            .unreadable_ref(draft.setup.roster[0].creature.as_ref().unwrap());
        assert!(
            duplicate.is_some_and(|note| note.duplicate
                && note
                    .word
                    .starts_with("These two files claim to be the same entry")),
            "{invalid:?}"
        );
        assert!(invalid.tone == Tone::Warning, "{invalid:?}");
        assert!(invalid.tone == missing_creature_tone(duplicate));
        assert_eq!(missing_creature_mark(duplicate).0, icons::WARN);
        encounters.snapshot = Some(snapshot);
        encounters.error = Some(invalid.text.clone());
        encounters.validation_error = Some(invalid);
        assert!(encounters.error_tone() == Tone::Warning);
    }

    /// FD7 family: an encounter's faults name the roster row as the DM sees it, never its key or a
    /// stored field name.
    #[test]
    fn encounter_faults_name_the_row_not_its_key() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let id = creature(dir.path(), "Cinder Wolf", false);
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let new_draft = || {
            let mut draft = Draft::new();
            draft.name = "Bridge".into();
            draft.add(id, false);
            draft.add(id, false);
            draft
        };
        let draft = new_draft();
        assert!(draft.validate(&snapshot).is_ok());
        assert!(draft.setup.roster[1].key.starts_with("actor_"));
        let fault = |change: &dyn Fn(&mut Draft)| {
            let mut draft = new_draft();
            change(&mut draft);
            let text = draft.validate(&snapshot).unwrap_err().text;
            assert!(!text.contains("actor_"), "{text}");
            text
        };
        assert_eq!(
            fault(&|draft| draft.setup.roster[1].side = "nowhere".into()),
            "Row 2 (Cinder Wolf): its side is not in this encounter."
        );
        assert_eq!(
            fault(&|draft| {
                draft.setup.roster[1]
                    .initial
                    .insert("hp".into(), Value::from(-3));
            }),
            "Row 2 (Cinder Wolf): its starting HP must be a whole number, 0 or more."
        );
        assert_eq!(
            fault(&|draft| {
                draft.setup.roster[1]
                    .initial
                    .insert("temporary_hp".into(), Value::from("lots"));
            }),
            "Row 2 (Cinder Wolf): its starting Temporary HP must be a whole number, 0 or more."
        );
        assert_eq!(
            fault(&|draft| draft.setup.roster[1].key = draft.setup.roster[0].key.clone()),
            format!(
                "Row 2 (Cinder Wolf) shares its identity with another row, or has none. {REPAIR_IN_ENTRIES}"
            )
        );
        assert_eq!(
            fault(&|draft| draft.setup.roster[1].creature = None),
            "Row 2 (Missing creature) needs one creature or one party member."
        );
        assert_eq!(
            fault(&|draft| {
                draft.setup.roster[1].creature = None;
                draft.setup.roster[1].party_slot = Some("fighter".into());
            }),
            PARTY_FROM_CAMPAIGN
        );
        assert_eq!(
            fault(&|draft| {
                for (_, side) in draft.setup.sides.iter_mut() {
                    side.as_mapping_mut().unwrap().remove("hostile_to");
                }
            }),
            "Each side needs a list of the sides it fights."
        );
        assert_eq!(
            fault(&|draft| {
                draft.setup.roster[1]
                    .overrides
                    .insert("name".into(), Value::from("Ash"));
                draft.setup.roster[1]
                    .overrides
                    .insert("side".into(), Value::from("nowhere"));
            }),
            "Row 2 (Ash): side nowhere is not a side of this encounter."
        );
        // A fault found when the fight starts names the row too.
        let mut editor = Encounters {
            active: Some(active.clone()),
            draft: Some(draft),
            ..Default::default()
        };
        editor.draft.as_mut().unwrap().setup.roster[1]
            .extra
            .insert("initial_presence".into(), Value::from("maybe"));
        editor.draft.as_mut().unwrap().save(&active, &[]).unwrap();
        let Err(error) = editor.start_run(&active, &[]) else {
            panic!("a fight with an unknown starting presence started");
        };
        assert_eq!(
            error,
            "Row 2 (Cinder Wolf): Splat cannot tell if it starts in the fight."
        );
    }

    #[test]
    fn the_add_pane_says_where_party_members_come_from_in_plain_words() {
        for code in ["local", "copies", "cop", "binding", "slot"] {
            assert!(!PARTY_SOURCE.contains(code), "{PARTY_SOURCE}");
            assert!(!PARTY_FROM_CAMPAIGN.contains(code), "{PARTY_FROM_CAMPAIGN}");
        }
        assert!(PARTY_FROM_CAMPAIGN.ends_with(&format!(
            "This editor adds party members {}",
            PARTY_SOURCE.strip_prefix("Party members come ").unwrap()
        )));
    }

    #[test]
    fn roster_faults_survive_a_row_with_missing_cells() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let wolf = stat_creature(dir.path(), "Wolf", 20);
        let mut draft = Draft::new();
        draft.add(wolf, false);
        draft.save(&active, &[]).unwrap();
        let path = draft.path.clone();
        let saved = fs::read_to_string(dir.path().join(&path)).unwrap();
        let table = splat::content::layout::Document::new(&saved)
            .table(splat::content::list_formats::ROSTER)
            .unwrap();
        // Cut the row back to two cells, as a hand edit in Obsidian can.
        let row = &table.rows[0];
        let short = format!("| {} | {} |", row[0].value, row[1].value);
        let start = saved[..row[0].range.start]
            .rfind('\n')
            .map_or(0, |at| at + 1);
        let end = saved[row[0].range.start..]
            .find('\n')
            .map_or(saved.len(), |at| row[0].range.start + at);
        let mut cut = saved.clone();
        cut.replace_range(start..end, &short);
        let entry = Entry::parse(&path, &cut).unwrap();
        let faults = roster_faults(&entry, &BTreeMap::new());
        assert_eq!(faults.len(), 1, "{cut}");
    }

    #[test]
    fn iter4_b_unread_roster_blocks_direct_and_plan_starts_without_writing() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let wolf = stat_creature(dir.path(), "Wolf", 20);
        let mut draft = Draft::new();
        draft.add(wolf, false);
        draft.add(wolf, false);
        draft.save(&active, &[]).unwrap();
        let path = draft.path.clone();
        let saved = fs::read_to_string(dir.path().join(&path)).unwrap();
        let table = splat::content::layout::Document::new(&saved)
            .table(splat::content::list_formats::ROSTER)
            .unwrap();
        let mut flagged = saved.clone();
        flagged.replace_range(table.rows[1][3].range.clone(), "HP lots");
        // A link can show a UUID. The warning still names the creature.
        flagged.replace_range(table.rows[1][1].range.clone(), &format!("[[{wolf}]]"));
        fs::write(dir.path().join(&path), &flagged).unwrap();
        let permissions = fs::metadata(dir.path().join(&path)).unwrap().permissions();
        let mut read_only = permissions.clone();
        read_only.set_readonly(true);
        fs::set_permissions(dir.path().join(&path), read_only).unwrap();
        let draft = Draft::open(dir.path(), &path).unwrap();
        // FC3: the reader leaves the unread row out of the roster; its fault says where it
        // stands, after the one row that reads, so the roster shows it in its place.
        let faults = roster_faults(&Entry::parse(&path, &flagged).unwrap(), &BTreeMap::new());
        assert_eq!(draft.setup.roster.len(), 1);
        assert_eq!(
            faults.iter().map(|fault| fault.before).collect::<Vec<_>>(),
            [1]
        );
        let mut editor = Encounters {
            draft: Some(draft),
            ..Default::default()
        };
        let error = editor
            .start_run(&active, &[])
            .err()
            .expect("An unread row must block Start fight");
        assert!(error.contains("Wolf") && error.contains("line"), "{error}");
        editor.run_path(&path, &active, &[]);
        assert!(editor.run.is_none());
        assert!(
            editor
                .error
                .as_ref()
                .is_some_and(|error| error.contains("Wolf"))
        );
        assert_eq!(fs::read_to_string(dir.path().join(&path)).unwrap(), flagged);
        fs::set_permissions(dir.path().join(&path), permissions).unwrap();
        fs::write(dir.path().join(&path), &saved).unwrap();
        editor.draft = Some(Draft::open(dir.path(), &path).unwrap());
        let run = editor.start_run(&active, &[]).unwrap();
        let saved: serde_json::Value =
            serde_json::from_slice(&fs::read(run.path().unwrap()).unwrap()).unwrap();
        assert_eq!(saved["state"]["participants"].as_array().unwrap().len(), 2);
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
        // History says why an older fight cannot go on, and keeps that answer only while the
        // same run is at this path. Outside Splat, the fight that was played last takes the
        // file; once the list is read again, History offers Continue fight for it.
        let fact = || {
            crate::encounter_run::Run::open(&path, campaign.id)
                .unwrap()
                .timeline_fact()
        };
        let mut later = fact();
        later.id = uuid::Uuid::new_v4();
        let mut played = crate::timeline::Book::default();
        played.open(Some(dir.path()));
        played.record_run(&fact(), None, true);
        played.record_run(&later, None, true);
        editor.snapshot = Some(Snapshot::load(&campaign, &[]).unwrap());
        editor.follow_continue_rule(&played);
        assert_eq!(
            editor.history_blocked,
            Some(crate::encounter_run::CONTINUE_LATER)
        );
        let mut taken = saved.clone();
        taken["id"] = serde_json::json!(later.id);
        fs::write(&path, serde_json::to_vec_pretty(&taken).unwrap()).unwrap();
        editor.read_runs_again(&campaign);
        editor.open_saved_run(&path, &campaign);
        editor.follow_continue_rule(&played);
        assert_eq!(
            editor.history_blocked, None,
            "the run now at this path is the fight played last"
        );
        fs::write(&path, &bytes).unwrap();
        editor.snapshot = None;

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

        // Continue fight is refused while another run is open, and changes nothing.
        editor.error = None;
        editor.continue_saved_run(&path, &campaign);
        assert_eq!(editor.error.as_deref(), Some(BLOCKED_BY_RUN));
        assert_eq!(editor.run().unwrap().live_info().unwrap().id, current);
        assert_eq!(fs::read(&path).unwrap(), bytes);
        // With the slot free, it opens that run, and the app answers by the rule with the
        // Timeline. A run opened this way has no answer yet, and that is not a yes: with no
        // event for this fight, the run says why and stays ended.
        editor.run = None;
        editor.go_to_run = false;
        editor.continue_saved_run(&path, &campaign);
        assert!(editor.error.is_none() && editor.go_to_run && editor.history_read.is_none());
        let mut timeline = crate::timeline::Book::default();
        timeline.open(Some(dir.path()));
        let answer = |editor: &mut Encounters, timeline: &mut crate::timeline::Book| {
            let run = editor.run_mut().unwrap();
            let act = run
                .wants_unend
                .take()
                .expect("Continue fight waits for the rule");
            run.unend(act, timeline);
        };
        answer(&mut editor, &mut timeline);
        let run = editor.run().unwrap();
        assert!(run.ended());
        assert!(
            run.continue_blocked
                .is_some_and(|reason| reason.starts_with("This fight is not on the Timeline"))
        );
        // On the Timeline as the fight that was played last, it goes on, on the round it
        // ended in.
        let fact = editor.run().unwrap().timeline_fact();
        timeline.record_run(&fact, None, true);
        editor.continue_saved_run(&path, &campaign);
        answer(&mut editor, &mut timeline);
        let run = editor.run().unwrap();
        assert_eq!(run.continue_blocked, None);
        assert_eq!(run.path(), Some(path.as_path()));
        assert_eq!(run.state_words(false), "Round 3");
        let steps = |editor: &mut Encounters| {
            editor.run_mut().unwrap().autosave();
            let saved: serde_json::Value =
                serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            saved["undo"]["steps"].as_array().unwrap().len()
        };
        assert_eq!(steps(&mut editor), 1, "one step, which Undo takes back");
        // The same action on the open run does nothing more to a fight that is live.
        editor.continue_saved_run(&path, &campaign);
        assert_eq!(steps(&mut editor), 1);
    }

    #[test]
    fn i19_p2_missing_run_folder_clears_resume_and_reports_one_issue() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let mut draft = Draft::new();
        draft.add(stat_creature(dir.path(), "Wolf", 20), false);
        draft.save(&campaign, &[]).unwrap();
        let mut editor = Encounters {
            draft: Some(draft),
            ..Default::default()
        };
        let run = editor.start_run(&campaign, &[]).unwrap();
        let folder = run.path().unwrap().parent().unwrap().to_owned();
        drop(run);
        editor.snapshot = Some(Snapshot::load(&campaign, &[]).unwrap());
        assert_eq!(editor.paused_run_count(), 1);
        editor.snapshot.as_mut().unwrap().options.folder = "Unsaved folder".into();
        editor.read_runs_again(&campaign);
        assert_eq!(
            editor.paused_run_count(),
            1,
            "an unsaved folder is not read"
        );
        fs::rename(&folder, dir.path().join("Moved runs")).unwrap();
        editor.read_runs_again(&campaign);
        editor.read_runs_again(&campaign);
        assert!(editor.saved_runs().is_empty());
        assert!(editor.paused_run().is_none());
        assert_eq!(editor.paused_run_count(), 0);
        assert!(editor.error.is_none());
        assert_eq!(
            editor
                .snapshot
                .as_ref()
                .unwrap()
                .issues
                .iter()
                .filter(|issue| issue.starts_with("Run folder not found"))
                .count(),
            1
        );
        fs::rename(dir.path().join("Moved runs"), folder).unwrap();
        editor.read_runs_again(&campaign);
        assert_eq!(editor.paused_run_count(), 1);
        assert!(
            !editor
                .snapshot
                .as_ref()
                .unwrap()
                .issues
                .iter()
                .any(|issue| issue.starts_with("Run folder not found"))
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
            &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {id}\nsplat_kind: creature\nsplat_name: {name}\nsplat_creature: {{}}\nsplat_stats:\n  max_hp: {{value: {hp}}}\n  armor_class: {{value: 13}}\n  initiative: {{value: 1}}\n"), "").expect("schema-2 fixture"),
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
        let edited = text.replace("HP +10", "HP +99");
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
        assert!(key.starts_with("actor_"));
        for (block, words) in [
            ("{hit_points: 3}", "unknown override"),
            ("{max_hp: {set: 0}}", "from 1 to"),
            ("{max_hp: {add: -20}}", "would become 0"),
            ("{attack_bonus: {set: 2}}", "attack_bonus"),
            ("{side: rivals}", "not a side of this encounter"),
        ] {
            slot_override(&mut d, 0, block);
            let error = d.save(&campaign, &[]).unwrap_err();
            // The row as the DM sees it, never its stored key.
            assert!(
                error.starts_with("Row 1 (Rival): ")
                    && !error.contains(&key)
                    && error.contains(words),
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
        let text = format!(
            "{}\n## Splat data\n\n<!-- splat:yaml path=splat_list_data -->\n\n```yaml\nsplat_encounter:\n  slots:\n    rival_one:\n      overrides: {{level: 3}}\n```\n",
            fs::read_to_string(&path).unwrap()
        );
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
