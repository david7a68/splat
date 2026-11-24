//! Campaign encounter templates. Runtime state is deliberately separate.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
#[cfg(test)]
use crate::content_workspace::patch_fields;
use crate::design::memo::Revised;
use crate::design::text_draft::{KeptCompare, TextDraft};
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue, EntryRef},
    design::{self, Tone, Type, colors, icons},
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

/// Where the Add participants pane's party members come from (the GPUI editor's help).
pub const PARTY_SOURCE: &str = "Party members come from the sheets Splat saved last.";
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
    /// F3, R37: a required part of the encounter is missing (its name), as against a change
    /// that cannot stand. Set where the fault is found (`Invalid::missing`), never read back from
    /// the words; with the draft's origin it gives the save row's word.
    missing: bool,
    /// Roster rows whose creature is gone or cannot be read, as the roster names them: the
    /// typed cause of a refused Start (`Invalid::start_refusal`). Set where `validate` finds
    /// them, never read back from the words.
    rows: Option<RowCause>,
}

/// Why roster rows have no creature, by the rows the roster names (`Draft::deleted_rows`,
/// `Draft::unreadable_rows`), or by the creature's own name and package where the next step
/// is about the creature, not the row (issue #530).
#[derive(Clone, Debug, PartialEq)]
enum RowCause {
    Deleted(Vec<String>),
    /// The roster rows, and the Library name of each row's unreadable file, in roster order.
    Unreadable(Vec<String>, Vec<String>),
    /// Two files claim the creature: its own name.
    Duplicate(String),
    /// The creature's package is not loaded: the creature (as the row names it) and the package.
    PackageMissing {
        creature: String,
        package: String,
    },
    /// Start's own checks (issue #530, item 4), each the typed cause of one fault `validate`
    /// finds, by the row's shown name where a row is at fault.
    EncounterUnreadable(String),
    RosterUnreadable(String),
    Unnamed,
    LootHeading,
    SideUnnamed,
    HostileSideMissing,
    SideWithoutFoes,
    KeyShared(String),
    SideMissing(String),
    Unavailable(String),
    PartyFromCampaign,
    WithoutCreature(usize),
    StartingNotWhole {
        row: String,
        what: &'static str,
    },
    ChangedSideMissing {
        row: String,
        side: String,
    },
    HpAboveMax {
        row: String,
        hp: i64,
        max: i64,
    },
    ChangesInvalid(String),
}

impl Invalid {
    /// Round 31 SE3, AE4 (start-fight-result-first, ruling M7): the answer to a refused Start
    /// fight from `encounter`, when its cause is a row's creature: no fight started, why, and the
    /// one next step. `None` for any other cause, which keeps its own words, and for creatures of
    /// several unreadable files, which the part's cause names one at a time.
    fn start_refusal(&self, encounter: &str) -> Option<design::results::Refusal> {
        use design::results::{Cause, FailedAct, Name, Recovery, Refusal, Rows, Step};
        let act = FailedAct::NotStarted {
            encounter: Name::new(encounter)?,
        };
        match self.rows.as_ref()? {
            RowCause::Deleted(names) => {
                let rows = Rows::new(names.iter().filter_map(|name| Name::new(name.as_str())))?;
                Some(Refusal::new(
                    act,
                    Cause::CreatureDeleted(rows.clone()),
                    Recovery::First(Step::ReplaceRows(rows)),
                ))
            }
            RowCause::Unreadable(_, files) => {
                // Issue #530: the next step names the Library entry (the file), not a row's
                // changed name.
                let mut creatures: Vec<&String> = files.iter().collect();
                creatures.sort();
                creatures.dedup();
                let [creature] = creatures[..] else {
                    return None;
                };
                Some(Refusal::new(
                    act,
                    Cause::CreatureUnreadable(Name::new(creature.as_str())?),
                    Recovery::First(Step::FixInLibrary(Name::new(creature.as_str())?)),
                ))
            }
            RowCause::Duplicate(creature) => Some(Refusal::new(
                act,
                Cause::CreatureDuplicate(Name::new(creature.as_str())?),
                Recovery::First(Step::FixInLibrary(Name::new(creature.as_str())?)),
            )),
            RowCause::PackageMissing { creature, package } => Some(Refusal::new(
                act,
                Cause::PackageNotFound {
                    creature: Name::new(creature.as_str())?,
                    package: Name::new(package.as_str())?,
                },
                Recovery::First(Step::AddPackage(Name::new(package.as_str())?)),
            )),
            // Item 4: every other fault Start's check finds, in its own typed cause.
            other => {
                let row = |name: &str| Rows::new([Name::new(name)?]);
                let (cause, step) = match other {
                    RowCause::EncounterUnreadable(file) => (
                        Cause::EncounterUnreadable(Name::new(file.as_str())?),
                        Step::RepairInEntries,
                    ),
                    RowCause::RosterUnreadable(file) => (
                        Cause::RosterUnreadable(Name::new(file.as_str())?),
                        Step::FixRosterRows,
                    ),
                    RowCause::Unnamed => (Cause::EncounterUnnamed, Step::EnterEncounterName),
                    RowCause::LootHeading => (Cause::LootHeadingInNotes, Step::MoveLootLines),
                    RowCause::SideUnnamed => (Cause::SideUnnamed, Step::RepairInEntries),
                    RowCause::HostileSideMissing => {
                        (Cause::HostileSideMissing, Step::RepairInEntries)
                    }
                    RowCause::SideWithoutFoes => (Cause::SideWithoutFoes, Step::RepairInEntries),
                    RowCause::KeyShared(name) => {
                        (Cause::RowKeyShared(row(name)?), Step::RepairInEntries)
                    }
                    RowCause::SideMissing(name) => {
                        (Cause::RowSideMissing(row(name)?), Step::RepairInEntries)
                    }
                    RowCause::Unavailable(name) => (
                        Cause::CreatureUnavailable(row(name)?),
                        Step::ReplaceRows(row(name)?),
                    ),
                    RowCause::PartyFromCampaign => (Cause::PartyFromCampaign, Step::EditPartyRows),
                    RowCause::WithoutCreature(index) => (
                        Cause::RowWithoutCreature(Name::new(format!("Row {}", index + 1))?),
                        Step::RepairInEntries,
                    ),
                    RowCause::StartingNotWhole { row: name, what } => (
                        Cause::StartingNotWhole {
                            rows: row(name)?,
                            what: Name::new(*what)?,
                        },
                        Step::FixStartingValues(row(name)?),
                    ),
                    RowCause::ChangedSideMissing { row: name, side } => (
                        Cause::ChangedSideMissing {
                            rows: row(name)?,
                            side: Name::new(side.as_str())?,
                        },
                        Step::FixChanges(row(name)?),
                    ),
                    RowCause::HpAboveMax { row: name, hp, max } => (
                        Cause::StartingHpAboveMax {
                            rows: row(name)?,
                            hp: *hp,
                            max: *max,
                        },
                        Step::FixStartingValues(row(name)?),
                    ),
                    RowCause::ChangesInvalid(name) => (
                        Cause::ChangesInvalid(row(name)?),
                        Step::FixChanges(row(name)?),
                    ),
                    RowCause::Deleted(_)
                    | RowCause::Unreadable(..)
                    | RowCause::Duplicate(_)
                    | RowCause::PackageMissing { .. } => return None,
                };
                Some(Refusal::new(act, cause, Recovery::First(step)))
            }
        }
    }

    /// The typed cause of this fault, for Start's answer (issue #530).
    fn because(mut self, cause: RowCause) -> Self {
        self.rows = Some(cause);
        self
    }

    /// A required part is missing.
    fn missing(text: &str) -> Self {
        Self {
            missing: true,
            // R37: a missing part's tone, the one its diagnosis gives the save row's word.
            tone: Tone::Warning,
            ..Self::from(text)
        }
    }

    /// The save row's typed diagnosis of this fault. Its tone is the fault's (R41): a deleted
    /// creature is a thing that is gone, a warning; an unreadable one or a change that cannot
    /// stand is an error.
    fn diagnosis(&self) -> design::save_row::Diagnosis<'static> {
        if self.missing {
            design::save_row::Diagnosis::incomplete(self.text.clone())
        } else if self.tone == Tone::Warning {
            design::save_row::Diagnosis::missing(self.text.clone())
        } else {
            design::save_row::Diagnosis::refused(self.text.clone())
        }
    }
}

/// A group's refusal, with its kind: the encounter page keeps it as its last refusal, and the
/// save row and the footer give it the refusal's tone (R41).
impl From<&groups::Refusal> for Invalid {
    fn from(refusal: &groups::Refusal) -> Self {
        Self {
            text: refusal.text().to_owned(),
            tone: refusal.tone(),
            missing: refusal.diagnosis().only_incomplete(),
            rows: None,
        }
    }
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
            missing: false,
            rows: None,
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

#[path = "encounters/disk.rs"]
mod disk;
/// The creature pane, replacement and narrow tabs as plain data, for the GPUI page.
#[path = "encounters/forms.rs"]
pub mod forms;
#[path = "encounters/groups.rs"]
mod groups;
#[path = "encounters/history.rs"]
mod history;
/// The page as plain data and its acts, for the GPUI page (`src/gui/pages/encounters/`).
#[path = "encounters/model.rs"]
pub mod model;
#[path = "encounters/results_key.rs"]
mod results_key;
#[path = "encounters/run_settings_view.rs"]
mod run_settings_view;
// The one member-fault form and its row, for Arrivals in the run (Contract B, R).
use groups::GroupDraft;
pub(crate) use groups::{
    EDIT_GROUP, ENCOUNTER_DELETED, NO_MAX_HP_CAUSE, PACKAGES_LINK, member_fault,
};
#[cfg(any(test, feature = "test-support"))]
pub use groups::{change_copies, group_checks, member_comparisons};
pub use history::{Template, template_of};

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
pub struct RosterFault {
    pub name: String,
    pub line: usize,
    pub problem: String,
    /// How many read rows come before this one, so the roster shows it in its place (FC3).
    pub before: usize,
}

impl RosterFault {
    /// `<name> · line <n>: <problem>`: the one form that the encounter editor, the run setup and
    /// Save's reason give an unread roster row.
    pub fn words(&self) -> String {
        row_fault_words(&self.name, self.line, &self.problem)
    }
}

/// Save's checks of an encounter's changed groups (`owned_groups_found`): one ranked
/// diagnosis, the origin of the first group that only lacks a part, and the reason the footer
/// paints, if any (F2, R35): the first refusal that no group row or field already says.
#[derive(Clone)]
struct OwnedGroupsFound {
    found: design::save_row::Diagnosis<'static>,
    origin: Option<design::save_row::DraftOrigin>,
    footer: Option<EncounterFault>,
    /// N4: the cause that the footer's next step answers, in the words Save says it (`Group
    /// <name>: <cause>`), so the footer can leave out a stored failed-action message that only
    /// repeats it, and keep any other.
    cause: Option<String>,
}

/// N1: Save's checks of an encounter's changed groups, ranked: the first refused change outranks
/// the first missing part, whose group's origin comes with it (R37).
fn owned_groups_found(
    groups: &[GroupDraft],
    snapshot: &Snapshot,
    encounter: Uuid,
) -> OwnedGroupsFound {
    use design::save_row::Diagnosis;
    let mut incomplete = None;
    let mut incomplete_cause = None;
    // Lead ruling 1 (s30 B11 F1): the most severe refusal across the groups by the diagnosis
    // rank, then group order (`Diagnosis::outranks`; equals keep the first). Its diagnosis, its
    // typed footer and its cause are chosen together, so none comes from another group.
    let mut chosen: Option<(Diagnosis<'static>, EncounterFault, String)> = None;
    for group in groups.iter().filter(|group| group.dirty()) {
        let Err(refusal) = group.check(snapshot, Some(encounter)) else {
            continue;
        };
        if refusal.diagnosis().only_incomplete() {
            let words = if group.is_empty() {
                empty_group_words(&group.name())
            } else {
                named_group_words(&group.name(), refusal.text())
            };
            incomplete_cause.get_or_insert(named_group_words(&group.name(), refusal.text()));
            incomplete.get_or_insert((words, group.origin()));
        } else {
            let words = named_group_words(&group.name(), refusal.text());
            // Save's words for it, with the refusal's kind and so its tone.
            let found = refusal.named(words.clone());
            // N2 (lead ruling, R35): a fault with a place of its own (a member's row, Belongs
            // to) keeps its cause there, and the footer always names the next step, so a blocked
            // Save has painted words whether that place is in view or not; never the cause twice.
            let footer = match refusal.place() {
                groups::RefusalPlace::Footer | groups::RefusalPlace::SaveRow => words.clone(),
                groups::RefusalPlace::Member(..) => {
                    fix_group_row(refusal.row().unwrap_or("marked"), &group.name())
                }
                groups::RefusalPlace::BelongsTo => format!(
                    "Choose who {} belongs to before you save.",
                    group_words(&group.name())
                ),
            };
            if chosen
                .as_ref()
                .is_none_or(|(kept, ..)| found.outranks(kept))
            {
                // R41: the footer says the fault in the refusal's tone.
                let fault = EncounterFault::refusal(footer, &found);
                chosen = Some((found, fault, words));
            }
        }
    }
    let origin = incomplete.as_ref().map(|(_, origin)| *origin);
    let (refused, footer, cause) = match chosen {
        Some((found, footer, cause)) => (Some(found), Some(footer), Some(cause)),
        None => (
            None,
            incomplete.as_ref().map(|(words, _)| {
                EncounterFault::refusal(words.clone(), &Diagnosis::incomplete(words.as_str()))
            }),
            incomplete_cause,
        ),
    };
    let found = incomplete
        .map_or_else(Diagnosis::valid, |(words, _)| Diagnosis::incomplete(words))
        .and(refused.unwrap_or_else(Diagnosis::valid));
    OwnedGroupsFound {
        found,
        origin,
        footer,
        cause,
    }
}

/// F5: the key of the owned-group check: the campaign's read, the encounter draft, its number
/// of groups and the count of group revisions issued. Constant work per frame: no group is
/// read. Every edit of a group issues a revision (`GroupDraft::touch`), so a changed group
/// changes the key; a group added or removed changes the number.
fn owned_groups_key(snapshot: &Snapshot, draft: &Draft) -> (Uuid, Uuid, usize, u64) {
    (
        snapshot.revision,
        draft.id,
        draft.groups.len(),
        groups::revisions_issued(),
    )
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// (frames that asked for the owned-group check, groups that the check or its key read)
    static OWNED_GROUP_WORK: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((0, 0)) };
}
/// The counts of the owned-group check, for `CachedWork` (F5).
#[cfg(any(test, feature = "test-support"))]
pub fn owned_group_work() -> (usize, usize) {
    OWNED_GROUP_WORK.with(std::cell::Cell::get)
}

/// The next step for a group's member row that blocks Save (N2): `Fix the Cinder Wolf row of
/// Ash Hounds before you save.`
fn fix_group_row(row: &str, group: &str) -> String {
    format!(
        "Fix the {row} row of {} before you save.",
        group_words(group)
    )
}

/// A group as a sentence names it: its name, or `the new group` while it has none.
fn group_words(name: &str) -> String {
    match name.trim() {
        "" => "the new group".to_owned(),
        name => name.to_owned(),
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// The error the next encounter write gives instead of writing (`fail_next_encounter_write`).
    static FAIL_NEXT_WRITE: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

/// A test makes the next Save of an encounter fail at its write, as a full disk would.
#[cfg(any(test, feature = "test-support"))]
pub fn fail_next_encounter_write(error: &str) {
    FAIL_NEXT_WRITE.with(|next| *next.borrow_mut() = Some(error.to_owned()));
}

/// A group's refusal as the encounter's Save says it: `Group <name>: <words>`.
fn named_group_words(name: &str, words: &str) -> String {
    match name.trim() {
        "" => words.to_owned(),
        name => format!("Group {name}: {words}"),
    }
}

/// An empty group's reason in the encounter editor: `Add a creature to <name> first.`
fn empty_group_words(name: &str) -> String {
    match name.trim() {
        "" => "Add a creature to the new group first.".to_owned(),
        name => format!("Add a creature to {name} first."),
    }
}

/// Where the editor leaves the roster's unread rows, for the roster to show in their places.
fn roster_fault_rows_id(draft: Uuid) -> egui::Id {
    egui::Id::new(("roster_fault_rows", draft))
}

/// Save's reason for the encounter's unread roster rows (`reason`), as its typed diagnosis.
/// Lead ruling (s30 B11, R41): an unread roster row is a row of a file that is there and cannot
/// be read, so it is an error wherever the editor says it: Save's reason (`Invalid`), the
/// message above the roster and the row's own mark. This is the one value they read; a roster
/// row whose creature file is missing stays a warning (`missing_creature_tone`).
fn unread_rows_found(reason: String) -> design::save_row::Diagnosis<'static> {
    design::save_row::Diagnosis::refused(reason)
}

/// The tone of every place that says an unread roster row: its diagnosis's. Run setup reads it
/// too.
pub(crate) fn unread_row_tone() -> Tone {
    unread_rows_found(String::new())
        .tone()
        .unwrap_or(Tone::Error)
}

/// A roster row's fault at its place: the row's name and the file's line. Its words are the
/// shared located form (`outcome::fault_words`: `<name> · line <n>: <cause>`), as every typed
/// place gives a fault of the DM's files.
struct RowFault<'a> {
    name: &'a str,
    line: usize,
    problem: &'a str,
}

impl design::outcome::Fault for RowFault<'_> {
    /// An unread row (R41, `unread_row_tone`).
    fn severity(&self) -> design::outcome::Severity {
        design::outcome::Severity::CannotRead
    }
    fn place(&self) -> Option<design::outcome::FaultPlace<'_>> {
        Some(design::outcome::FaultPlace {
            name: self.name.into(),
            line: u32::try_from(self.line).ok(),
        })
    }
    /// The parser's words, as the reader gives them (`cause_words`).
    fn cause(&self) -> design::outcome::Sentence {
        outcome_words(&splat::content::layout::cause_words(self.problem))
    }
}

fn row_fault_words(name: &str, line: usize, problem: &str) -> String {
    design::outcome::fault_words(&RowFault {
        name,
        line,
        problem,
    })
    .as_str()
    .to_owned()
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
) -> Vec<(String, Tone)> {
    flags
        .iter()
        .map(|flag| {
            let row = faults.iter().find(|fault| fault.line == flag.line);
            // R41: a flag of an unread roster row in that fault's tone; any other a warning.
            let tone = if row.is_some() {
                unread_row_tone()
            } else {
                Tone::Warning
            };
            (unread_note(path, flag, row, faults), tone)
        })
        .collect()
}

/// One flag's words (`unread_notes`).
fn unread_note(
    path: &str,
    flag: &splat::content::layout::Flag,
    row: Option<&RosterFault>,
    faults: &[RosterFault],
) -> String {
    // A flag read back from a saved copy has only its sentence, which names its line: it
    // follows the file or the row, and no line is added.
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
}

pub use splat_content_ui::content_actions::content_issue_count;

pub use splat_content_ui::content_actions::issue_count_title;

pub use splat_content_ui::content_actions::file_line;

/// The encounter file's flags (`unread_notes`), the unread roster rows among them (`faults`) and
/// the file's path in the campaign.
struct UnreadRows {
    notes: Vec<(String, Tone)>,
    faults: usize,
    path: String,
}

/// One flag as a footer fault, and the file line the last one holds.
struct UnreadFault {
    fault: EncounterFault,
    file: Option<String>,
}

impl UnreadFault {
    /// The fault, with its file's line (`Edit in Obsidian`, `Show file`) under `root`.
    fn file_in(&self, root: &std::path::Path) -> design::outcome::FaultMessage<FooterPress> {
        let message = design::outcome::FaultMessage::new(&self.fault);
        match &self.file {
            Some(path) => message.file(design::outcome::FileLine::new(root, path).edit().show()),
            None => message,
        }
    }
}

impl UnreadRows {
    /// One fault each: an unread roster row in its error tone (`unread_row_tone`, R41), any
    /// other flag as a warning. With unread roster rows, the last fault says what to do and
    /// holds the file's line, whose path and links start at its words.
    fn faults(self) -> Vec<UnreadFault> {
        let Self {
            mut notes,
            faults,
            path,
        } = self;
        let Some((last, last_tone)) = notes.pop() else {
            return Vec::new();
        };
        let mut found: Vec<UnreadFault> = notes
            .into_iter()
            .map(|(note, tone)| UnreadFault {
                fault: EncounterFault::FileFlag(note, tone),
                file: None,
            })
            .collect();
        found.push(if faults == 0 {
            UnreadFault {
                fault: EncounterFault::FileFlag(last, last_tone),
                file: None,
            }
        } else {
            UnreadFault {
                fault: EncounterFault::FileFlag(
                    format!("{last} {}", fix_roster_rows(faults)),
                    unread_row_tone(),
                ),
                file: Some(path),
            }
        });
        found
    }
}

fn roster_fault_reason(path: &str, faults: &[RosterFault]) -> Option<String> {
    (!faults.is_empty()).then(|| {
        format!(
            "{}: {}. {}",
            path,
            faults
                .iter()
                .map(|fault| {
                    // The rows join into one sentence: each without its end mark.
                    row_fault_words(&fault.name, fault.line, &fault.problem)
                        .trim_end_matches('.')
                        .to_owned()
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
    /// Whether this encounter was ever saved (R37): `New` from `Draft::new`, `Saved` from
    /// `Draft::open` and the first successful save. Its own fact, not `baseline`.
    origin: design::save_row::DraftOrigin,
    baseline: Option<String>,
    source: String,
    name: String,
    setup: Setup,
    /// What the DM reads to the table, and their notes: the note's own text, a document of its
    /// own (layout PR 9b).
    body: TextDraft,
    saved_name: String,
    saved_setup: Setup,
    /// The saved text, with its revision (layout PR 9b-C): `dirty` compares two revisions on a
    /// quiet frame.
    saved_body: Revised<String>,
    /// The last answer of the body's compare with the saved one.
    same_body: KeptCompare,
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
    /// Results just made (a group detached from this draft, a replacement of missing roster
    /// rows), each with what its Undo gives back. The draft keeps none: the page moves each into
    /// its store (`Encounters::record_editor_results`), which issues its order and holds it with
    /// its ticket, so an Undo applies only its own data.
    new_results: Vec<DraftUndo>,
    /// A Save ended this draft's results (FC3: after a Save a replacement's Undo would make a
    /// draft that differs from the file while it looks like it; a removed group is written).
    results_ended: bool,
    /// This draft's content, as the owner of its results: a fresh one at each open, kept by a
    /// clone, so a discarded draft and the clean draft that replaces it (same note, same `id`)
    /// never share a result.
    results_key: results_key::ResultsKey,
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
            origin: design::save_row::DraftOrigin::New,
            baseline: None,
            source,
            name: name.clone(),
            saved_name: name,
            setup: Setup::default(),
            body: TextDraft::default(),
            saved_setup: Setup::default(),
            saved_body: Revised::default(),
            same_body: KeptCompare::default(),
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
            new_results: Vec::new(),
            results_ended: false,
            results_key: results_key::ResultsKey::new(),
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
            origin: design::save_row::DraftOrigin::Saved,
            baseline: Some(text.clone()),
            source: text,
            saved_name: entry.name.clone(),
            name: entry.name.clone(),
            saved_setup: setup.clone(),
            setup,
            saved_body: Revised::new(body.clone()),
            same_body: KeptCompare::default(),
            body: TextDraft::new(body),
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
            new_results: Vec::new(),
            results_ended: false,
            results_key: results_key::ResultsKey::new(),
        })
    }
    /// Whether the text is the saved text: compared once per pair of revisions.
    fn body_saved(&self) -> bool {
        self.body.same_as(
            self.saved_body.revision(),
            &self.saved_body,
            &self.same_body,
        )
    }

    fn dirty(&self) -> bool {
        self.baseline.is_none()
            || self.name != self.saved_name
            || self.setup != self.saved_setup
            || !self.body_saved()
            || self.loot != self.saved_loot
            || self.groups.iter().any(GroupDraft::dirty)
            || !self.deleted_groups.is_empty()
    }
    /// The results made since the page last looked, handed to the page's store.
    fn take_new_results(&mut self) -> Vec<DraftUndo> {
        std::mem::take(&mut self.new_results)
    }
    /// Undo one of this draft's results, from the data its store holds with its ticket: the
    /// roster, or the removed group.
    fn undo_result(&mut self, undo: &DraftUndo) {
        match undo {
            DraftUndo::Replacement { before, .. } => self.setup.roster = before.clone(),
            DraftUndo::RemovedGroup {
                index,
                group,
                delete,
            } => self.undo_removed_group(*index, group, *delete),
        }
    }
    /// Undo a group's removal: it comes back in its place, with its draft.
    fn undo_removed_group(&mut self, index: usize, removed: &GroupDraft, delete: bool) {
        let path = removed.path().to_owned();
        if let Some(group) = self.groups.iter_mut().find(|group| group.path() == path) {
            // A move changed only who owns the group: Undo gives that back, and the group
            // keeps its members, edits and member results made since.
            group.take_owner_from(removed);
            return;
        }
        // A deleted group comes back as it was deleted; a group whose file went comes back from
        // the copy kept with its result.
        let back = self
            .deleted_groups
            .iter()
            .position(|group| delete && group.path() == path)
            .map_or_else(|| removed.clone(), |at| self.deleted_groups.remove(at));
        let at = index.min(self.groups.len());
        self.groups.insert(at, back);
    }
    /// AB1: Save's reason while one of this encounter's changed groups has no members: Save
    /// checks every changed group, and refuses an empty one.
    fn empty_group_reason(&self) -> Option<String> {
        self.groups
            .iter()
            .find(|group| group.is_empty() && group.dirty())
            .map(|group| empty_group_words(&group.name()))
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
        let source = if self.body_saved() && self.loot == self.saved_loot {
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
            self.body.as_str().to_owned()
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
            ("encounter", &self.name),
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
        #[cfg(any(test, feature = "test-support"))]
        if let Some(error) = FAIL_NEXT_WRITE.with(std::cell::RefCell::take) {
            // A test's failed write: the real save path, refused at its write, before the
            // baseline moves.
            return Err(error);
        }
        crate::file_batch::apply(&changes)?;
        self.source = text.clone();
        self.baseline = Some(text);
        self.origin = design::save_row::DraftOrigin::Saved;
        self.saved_name = self.name.clone();
        self.saved_setup = self.setup.clone();
        self.saved_body = Revised::new(self.body.as_str().to_owned());
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
        self.new_results.clear();
        self.results_ended = true;
        Ok(())
    }
    fn validate(&self, snapshot: &Snapshot) -> Result<(), Invalid> {
        #[cfg(any(test, feature = "test-support"))]
        VALIDATIONS.with(|count| count.set(count.get() + 1));
        // Issue #530 (item 4): each fault carries its typed cause, for Start's answer.
        let file = self
            .path
            .rsplit('/')
            .next()
            .unwrap_or(&self.path)
            .to_owned();
        let entry = Entry::parse(&self.path, &self.source).map_err(|error| {
            Invalid::from(error).because(RowCause::EncounterUnreadable(file.clone()))
        })?;
        if let Some(reason) = roster_fault_reason(
            &self.path,
            &roster_faults(&entry, &snapshot.known_creature_names),
        ) {
            // R41: in the tone of its diagnosis, as the message above the roster says it.
            return Err(Invalid {
                tone: unread_row_tone(),
                ..Invalid::from(reason).because(RowCause::RosterUnreadable(file))
            });
        }
        if self.name.trim().is_empty() {
            return Err(Invalid::missing("Enter an encounter name.").because(RowCause::Unnamed));
        }
        // The Loot list writes that section, so a second one typed in the notes would be lost.
        if self.loot_owned && splat::content::loot::has_section(&self.body) {
            return Err(Invalid::from("Read-aloud and notes has a Loot heading. Put those lines in the Loot list, or rename the heading.").because(RowCause::LootHeading));
        }
        let mut keys = BTreeSet::new();
        for (side, data) in &self.setup.sides {
            if side.as_str().is_none_or(|s| s.trim().is_empty()) {
                return Err(Invalid::from("Sides need names.").because(RowCause::SideUnnamed));
            }
            let hostile = data
                .get("hostile_to")
                .and_then(Value::as_sequence)
                .ok_or_else(|| {
                    Invalid::from("Each side needs a list of the sides it fights.")
                        .because(RowCause::SideWithoutFoes)
                })?;
            if hostile.iter().any(|s| !self.setup.sides.contains_key(s)) {
                return Err(Invalid::from("A hostile side is missing.")
                    .because(RowCause::HostileSideMissing));
            }
        }
        for (index, slot) in self.setup.roster.iter().enumerate() {
            let row = || roster_row(index, slot, snapshot);
            // The row's shown name, for Start's typed answer (`Rows`).
            let shown = || shown_row_name(slot, snapshot);
            if slot.key.trim().is_empty() || !keys.insert(&slot.key) {
                return Err(Invalid::from(format!(
                    "{} shares its identity with another row, or has none. {REPAIR_IN_ENTRIES}",
                    row()
                ))
                .because(RowCause::KeyShared(shown())));
            }
            if !self
                .setup
                .sides
                .contains_key(Value::from(slot.side.clone()))
            {
                return Err(Invalid::from(format!(
                    "{}: its side is not in this encounter.",
                    row()
                ))
                .because(RowCause::SideMissing(shown())));
            }
            match (&slot.creature, &slot.party_slot) {
                (Some(reference), None) => {
                    let id = snapshot.catalogue.resolve_ref(reference).map_err(|_| {
                        // The kind of fault sets the tone, here where the message is made.
                        let unreadable = snapshot.catalogue.unreadable_ref(reference);
                        // Issue #530: two files that claim the creature, and a package that is
                        // not loaded, are their own causes, never "unreadable" or "deleted".
                        let package = snapshot.catalogue.missing_package_of(reference);
                        Invalid {
                            missing: false,
                            rows: Some(match (unreadable, package) {
                                (Some(note), _) if note.duplicate => RowCause::Duplicate(
                                    roster_base_name(slot, None, &snapshot.known_creature_names),
                                ),
                                (Some(_), _) => RowCause::Unreadable(
                                    self.unreadable_rows(snapshot),
                                    self.unreadable_files(snapshot),
                                ),
                                (None, Some(package)) => RowCause::PackageMissing {
                                    creature: roster_base_name(
                                        slot,
                                        None,
                                        &snapshot.known_creature_names,
                                    ),
                                    package,
                                },
                                (None, None) => RowCause::Deleted(self.deleted_rows(snapshot)),
                            }),
                            tone: missing_creature_tone(unreadable),
                            // G125: each row says its deleted entry (`MISSING_CREATURE`), so the
                            // message says only the next act, for every such row.
                            // SC7, FC3: an unreadable entry too: each row says its cause and
                            // its `Open in …`, so the message says only the next act, once.
                            text: if unreadable.is_some() {
                                fix_unreadable_rows(&self.unreadable_rows(snapshot))
                            } else {
                                replace_deleted_rows(&self.deleted_rows(snapshot))
                            },
                        }
                    })?;
                    if !snapshot.creatures.iter().any(|c| c.id == id) {
                        return Err(Invalid::from(format!(
                            "{}: this creature is unavailable in campaign content.",
                            row()
                        ))
                        .because(RowCause::Unavailable(shown())));
                    }
                    self.validate_overrides(
                        (&row(), &shown()),
                        slot,
                        &snapshot.catalogue.entries[&id].entry,
                    )?;
                }
                (None, Some(_)) => {
                    return Err(
                        Invalid::from(PARTY_FROM_CAMPAIGN).because(RowCause::PartyFromCampaign)
                    );
                }
                _ => {
                    return Err(Invalid::from(format!(
                        "{} needs one creature or one party member.",
                        row()
                    ))
                    .because(RowCause::WithoutCreature(index)));
                }
            }
            for (field, words) in [("hp", "HP"), ("temporary_hp", "Temporary HP")] {
                if slot
                    .initial
                    .get(field)
                    .is_some_and(|v| v.as_u64().is_none())
                {
                    return Err(Invalid::from(format!(
                        "{}: its starting {words} must be a whole number, 0 or more.",
                        row()
                    ))
                    .because(RowCause::StartingNotWhole {
                        row: shown(),
                        what: words,
                    }));
                }
            }
        }
        Ok(())
    }
    /// The names of the roster rows whose package entry was deleted, in roster order, as each
    /// row shows its name: a changed name too (`roster_name`, as the row draws it).
    fn deleted_rows(&self, snapshot: &Snapshot) -> Vec<String> {
        self.setup
            .roster
            .iter()
            .filter(|slot| {
                slot.creature.as_ref().is_some_and(|reference| {
                    snapshot.catalogue.resolve_ref(reference).is_err()
                        && snapshot.catalogue.unreadable_ref(reference).is_none()
                })
            })
            .map(|slot| roster_name(slot, None, &snapshot.known_creature_names))
            .collect()
    }
    /// The names of the roster rows whose creature's file cannot be read now, in roster order,
    /// as each row shows its name.
    fn unreadable_rows(&self, snapshot: &Snapshot) -> Vec<String> {
        self.setup
            .roster
            .iter()
            .filter(|slot| {
                slot.creature.as_ref().is_some_and(|reference| {
                    snapshot.catalogue.resolve_ref(reference).is_err()
                        && snapshot.catalogue.unreadable_ref(reference).is_some()
                })
            })
            .map(|slot| roster_name(slot, None, &snapshot.known_creature_names))
            .collect()
    }
    /// The creature's own name (its Library entry's) for each roster row whose creature's file
    /// cannot be read, in roster order (issue #530: `Fix … in Library` names the entry, not a
    /// row's changed name).
    fn unreadable_files(&self, snapshot: &Snapshot) -> Vec<String> {
        self.setup
            .roster
            .iter()
            .filter(|slot| {
                slot.creature.as_ref().is_some_and(|reference| {
                    snapshot.catalogue.resolve_ref(reference).is_err()
                        && snapshot.catalogue.unreadable_ref(reference).is_some()
                })
            })
            .map(|slot| roster_base_name(slot, None, &snapshot.known_creature_names))
            .collect()
    }
    /// A change to a stat must leave it valid for this slot's creature and starting state.
    /// `row` is the row as a message names it (`Row 2 (Cinder Wolf)`) and as Start's typed
    /// answer names it (its shown name).
    fn validate_overrides(
        &self,
        (row, shown): (&str, &str),
        slot: &Slot,
        entry: &Entry,
    ) -> Result<(), Invalid> {
        let named = |error: String| {
            Invalid::from(format!("{row}: {error}"))
                .because(RowCause::ChangesInvalid(shown.to_owned()))
        };
        let overrides = Overrides::parse(&slot.overrides).map_err(named)?;
        let resolved = overrides.resolve_for(entry, &slot.side).map_err(named)?;
        if let Some(side) = &overrides.side
            && !self.setup.sides.contains_key(side.trim())
        {
            return Err(Invalid::from(format!(
                "{row}: side {side} is not a side of this encounter."
            ))
            .because(RowCause::ChangedSideMissing {
                row: shown.to_owned(),
                side: side.trim().to_owned(),
            }));
        }
        if overrides.max_hp.is_some()
            && let (Some(hp), Some(max)) = (
                slot.initial.get("hp").and_then(Value::as_i64),
                resolved.max_hp,
            )
            && hp > max
        {
            return Err(Invalid::from(format!(
                "{row}: starting HP {hp} is above the changed maximum of {max}."
            ))
            .because(RowCause::HpAboveMax {
                row: shown.to_owned(),
                hp,
                max,
            }));
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

/// What the DM did with the creature open beside the roster.
enum CreatureStep {
    Close,
}

/// What the DM can change on a creature, as they typed it.
#[derive(Clone)]
struct CreatureFields {
    name: String,
    hp: String,
    armor_class: String,
    /// The creature's text, a document of its own (layout PR 9b).
    body: TextDraft,
}

/// A creature the DM made their own, open beside the roster.
struct CreatureEditor {
    recovery: disk::Recovery,
    path: String,
    baseline: String,
    fields: CreatureFields,
    saved: CreatureFields,
    /// The last answer of the text's compare with the saved text (layout PR 9b-C): by the two
    /// drafts' revisions, so a quiet frame compares two numbers.
    same_body: KeptCompare,
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
            body: TextDraft::new(entry.body.trim()),
            name: entry.name,
        };
        Ok(Self {
            recovery: Default::default(),
            path: path.to_owned(),
            baseline: text,
            saved: fields.clone(),
            fields,
            same_body: KeptCompare::default(),
            error: None,
            focus: true,
        })
    }
    fn dirty(&self) -> bool {
        let (saved, now) = (&self.saved, &self.fields);
        saved.name != now.name
            || saved.hp != now.hp
            || saved.armor_class != now.armor_class
            || !self.body_saved()
    }

    /// Whether the text is the saved text: compared once per pair of revisions.
    fn body_saved(&self) -> bool {
        let saved = &self.saved.body;
        self.fields
            .body
            .same_as(saved.revision(), saved, &self.same_body)
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
            let text = if self.body_saved() {
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
        self.recovery.check(
            root,
            &mut self.path,
            Some(&self.baseline),
            ("creature", &self.fields.name),
        )?;
        let text = self.text()?;
        Entry::parse(&self.path, &text)?;
        write_note(root, &self.path, Some(&self.baseline), &text)?;
        let body = Entry::parse(&self.path, &text)?.body.trim().to_owned();
        let _ = self.fields.body.replace(body);
        self.baseline = text;
        self.saved = self.fields.clone();
        self.recovery = Default::default();
        Ok(())
    }
}

/// The page's Ctrl+S, taken from this frame's input (`Encounters::save_shortcut`).
pub(super) fn take_save_key(ctx: &egui::Context) -> bool {
    design::save_row::save_key_before_drawing(ctx, true)
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
/// A result's stats, each in its own place, so that they can be compared down the list. The
/// first place holds a party copy's word, which has no challenge rating. The package follows
/// if the row has room for it.
/// `hp_missing` comes from the creature's stats: the HP fault is data, never found in the words.
pub fn detail_columns(ui: &mut egui::Ui, detail: &str, source: Option<&str>, hp_missing: bool) {
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
            design::gap(ui, design::Gap::XS);
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
            design::gap(ui, design::Gap::XS);
            ui.label(Type::Small.rich(ui, source).color(colors().muted));
        }
    });
}

const STILL_READING: &str = "Still reading this campaign.";
const ADD_PARTICIPANTS_FIRST: &str = "Add participants first.";

pub use splat_content_ui::content_actions::creature_name_link;

/// `changed` holds this creature's values when a slot or member changes them.
/// The creature's hit points, after its changes. None is the `HP not set` fault.
pub fn max_hp(entry: &Entry, changed: Option<&Resolved>) -> Option<i64> {
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

/// The campaign's party for F1, by the ID of each copy's note, with its level from the saved
/// sheet now. A copy that cannot be read is still a member, with a level not known: its file is
/// one of this campaign's notes and holds a character sheet, but Splat cannot read its level.
fn campaign_party(
    campaign: &Campaign,
    catalogue: &Catalogue,
    creatures: &[Creature],
) -> BTreeMap<Uuid, Option<u64>> {
    let readable = creatures.iter().filter(|c| c.party).map(|creature| {
        let entry = &catalogue.entries[&creature.id].entry;
        let level = crate::difficulty::character_level(entry).flatten();
        (creature.id, level)
    });
    let unreadable = catalogue
        .unreadable
        .iter()
        .filter(|(_, note)| !note.duplicate && note.path.starts_with(&campaign.path))
        .filter(|(_, note)| {
            fs::read_to_string(&note.path)
                .is_ok_and(|text| text.contains("splat_character") && text.contains("dndbeyond"))
        })
        .map(|(id, _)| (*id, None));
    readable.chain(unreadable).collect()
}

/// The roster's difficulty: each character on the party's side by level, and each creature
/// against the party by XP. Allies on the party's side count for neither. A participant whose
/// entry is gone counts as not known. An input that is not known is named as the roster's
/// summary names it, so an opened row can say whose it is.
fn roster_difficulty(
    setup: &Setup,
    catalogue: &Catalogue,
    known: &BTreeMap<String, String>,
    scale: &crate::difficulty::Scale,
) -> crate::difficulty::Difficulty {
    let (mut party, mut party_names) = (Vec::new(), Vec::new());
    let (mut foes, mut foe_names) = (Vec::new(), Vec::new());
    for slot in &setup.roster {
        let side = slot
            .overrides
            .get("side")
            .and_then(Value::as_str)
            .unwrap_or(&slot.side);
        let entry = slot
            .creature
            .as_ref()
            .and_then(|reference| catalogue.resolve_ref(reference).ok())
            .and_then(|id| catalogue.entries.get(&id))
            .map(|entry| &entry.entry);
        if side.trim() == "party" {
            match entry {
                Some(entry) => {
                    if let Some(level) = crate::difficulty::character_level(entry) {
                        party.push(level);
                        party_names.push(Some(entry.name.clone()));
                    }
                }
                // A sheet that cannot be read keeps its file's name, as the campaign's party
                // does, so the reason can say whose level is not known.
                None => {
                    party.push(None);
                    party_names.push(
                        slot.creature
                            .as_ref()
                            .and_then(|reference| catalogue.unreadable_ref(reference))
                            .and_then(|note| {
                                Some(note.path.file_stem()?.to_string_lossy().into_owned())
                            }),
                    );
                }
            }
        } else {
            foes.push(entry.and_then(crate::difficulty::creature_xp));
            foe_names.push(roster_name(slot, entry, known));
        }
    }
    let roster_party = !party.is_empty();
    scale.weigh(&party, foes).named(|what, at| match what {
        crate::difficulty::Unknown::Xp => foe_names.get(at).cloned(),
        crate::difficulty::Unknown::Level if roster_party => party_names.get(at).cloned()?,
        // The campaign's party stands in: its copy's name, or its file's for a copy
        // that cannot be read.
        crate::difficulty::Unknown::Level => {
            let id = scale.party.keys().nth(at)?;
            catalogue
                .entries
                .get(id)
                .map(|entry| entry.entry.name.clone())
                .or_else(|| {
                    let path = &catalogue.unreadable.get(id)?.path;
                    Some(path.file_stem()?.to_string_lossy().into_owned())
                })
        }
    })
}

fn roster_summary(
    setup: &Setup,
    catalogue: &Catalogue,
    known: &BTreeMap<String, String>,
) -> crate::difficulty::Summary {
    let mut party = 0;
    // By the structured parts (shown name, base name, XP state), never the printed words: a
    // creature named `Wolf (as Bat)` and `Bat` renamed `Wolf` stay two members.
    let mut creatures = std::collections::BTreeMap::<(String, String, bool), usize>::new();
    for slot in &setup.roster {
        let side = slot
            .overrides
            .get("side")
            .and_then(Value::as_str)
            .unwrap_or(&slot.side);
        if side.trim() == "party" {
            party += 1;
        } else {
            let entry = slot
                .creature
                .as_ref()
                .and_then(|reference| catalogue.resolve_ref(reference).ok())
                .and_then(|id| catalogue.entries.get(&id))
                .map(|entry| &entry.entry);
            // R31: from the source `roster_difficulty` weighs, so the member knows itself
            // whether its XP is missing; no name decides it.
            let no_xp = entry.and_then(crate::difficulty::creature_xp).is_none();
            // FC3: a creature whose entry is gone keeps the name its row shows.
            let name = roster_base_name(slot, entry, known);
            // FL2: a renamed creature names its base, as a group's member summary does.
            let shown = groups::shown_name(&slot.overrides, name.clone());
            *creatures.entry((shown, name, no_xp)).or_default() += 1;
        }
    }
    // G118: typed parts, never built words, so the difficulty line knows which members the
    // summary shows. FL2: the words are a group's form, `2 × Cinder Wolf, 1 × Ember Cultist`
    // (`Summary::text`): every member with its count, joined by commas, so ` · ` separates
    // facts only.
    // G118: a renamed member is CALLED by its shown name, the name `roster_name` gives the
    // difficulty's reason, so the line matches it and does not name it again.
    let members = creatures
        .into_iter()
        .map(|((shown, name, no_xp), count)| {
            if shown == name {
                crate::difficulty::Member::new(count, name, no_xp)
            } else {
                crate::difficulty::Member::renamed(count, &shown, &name, no_xp)
            }
        })
        .collect();
    // Loot alone does not change a creature.
    let changed = setup.roster.iter().any(|slot| {
        Overrides::parse(&slot.overrides)
            .map_or(!slot.overrides.is_empty(), |o| o.changes_creature())
    });
    crate::difficulty::Summary {
        party,
        members,
        facts: changed
            .then_some(crate::difficulty::Fact::Changed)
            .into_iter()
            .collect(),
    }
}

/// The detail part of a creature without hit points.
const HP_NOT_SET: &str = "HP not set";

/// One part of a creature's detail line, and whether it is the `HP not set` fault.
pub(crate) struct DetailPart<'a> {
    pub words: &'a str,
    pub(crate) hp_fault: bool,
}

impl DetailPart<'_> {
    /// The fault has the warning status colour, icon and words; every other part is muted.
    pub fn color(&self) -> egui::Color32 {
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

fn stat_line(text: &str) -> String {
    text.split(" · ")
        .map(|part| part.replace(' ', "\u{a0}"))
        .collect::<Vec<_>>()
        .join(" · ")
}

const MISSING_CREATURE: &str = "Not found. Its package entry was deleted.";

/// The message for roster rows whose package entry was deleted: only the next act, naming or
/// counting the rows by creature. The rows themselves say the cause ([`MISSING_CREATURE`]).
/// `the 2 Cinder Wolf rows and the Goblin row`: rows named as the roster shows them, counted.
fn named_rows(names: &[String]) -> String {
    let mut counted: Vec<(&str, usize)> = Vec::new();
    for name in names {
        match counted.iter_mut().find(|(known, _)| known == name) {
            Some((_, count)) => *count += 1,
            None => counted.push((name, 1)),
        }
    }
    let rows: Vec<String> = counted
        .iter()
        .map(|(name, count)| match count {
            1 => format!("the {name} row"),
            count => format!("the {count} {name} rows"),
        })
        .collect();
    match rows.as_slice() {
        [] => "those rows".to_owned(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

fn replace_deleted_rows(names: &[String]) -> String {
    let rows = named_rows(names);
    // SC5, AC4: the sentence says when the replacement is needed: Save and a fight's start
    // both wait for it.
    if names.len() == 1 {
        format!("Choose a replacement for {rows} before you save or start a fight.")
    } else {
        format!("Choose replacements for {rows} before you save or start a fight.")
    }
}

/// The next act for rows whose creature's file cannot be read: the rows say the cause and the
/// way to the file, so this says only what waits.
fn fix_unreadable_rows(names: &[String]) -> String {
    let rows = named_rows(names);
    // Rows of one creature share its file.
    let mut creatures: Vec<&String> = names.iter().collect();
    creatures.sort();
    creatures.dedup();
    if creatures.len() == 1 {
        format!("Fix the file of {rows} before you save or start a fight.")
    } else {
        format!("Fix the files of {rows} before you save or start a fight.")
    }
}

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

/// R41 (lead ruling, s30 B11): whether a creature reference that does not resolve names a file
/// that is there and cannot be read (unreadable, an error), as against a creature that is gone (a
/// warning). The roster's one classification (`missing_creature_tone`), which a group's members
/// share.
pub(super) fn creature_unreadable(
    catalogue: &crate::content_workspace::Catalogue,
    reference: &crate::content_workspace::EntryRef,
) -> bool {
    missing_creature_tone(catalogue.unreadable_ref(reference)) == Tone::Error
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

/// A roster row's shown name: its creature's, the last name Splat knew for an entry that is
/// gone, or `Missing creature`. The roster's rows, its summary and its difficulty reason (FC3)
/// all name a row by this one function.
fn roster_name(
    slot: &Slot,
    definition: Option<&Entry>,
    known: &BTreeMap<String, String>,
) -> String {
    groups::shown_name(&slot.overrides, roster_base_name(slot, definition, known))
}

/// The creature's own name under a row's shown name (see `roster_name`).
fn roster_base_name(
    slot: &Slot,
    definition: Option<&Entry>,
    known: &BTreeMap<String, String>,
) -> String {
    definition
        .map(|entry| entry.name.clone())
        .or_else(|| {
            slot.creature
                .as_ref()
                .and_then(|reference| known.get(&reference.label()))
                .cloned()
        })
        .unwrap_or_else(|| "Missing creature".into())
}

/// A roster row's shown name (its changed name, else its creature's), as `roster_row` gives it
/// after the row's place.
fn shown_row_name(slot: &Slot, snapshot: &Snapshot) -> String {
    let definition = slot
        .creature
        .as_ref()
        .and_then(|reference| snapshot.catalogue.resolve_ref(reference).ok())
        .and_then(|id| snapshot.catalogue.entries.get(&id))
        .map(|indexed| &indexed.entry);
    roster_name(slot, definition, &snapshot.known_creature_names)
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

/// FF8: the creature search's separation of a copied note (`Rook Ambry copy.md is now a
/// separate note.`), kept with its Undo in the page's `separations` store. Its owner is the
/// campaign and the note's new identity, and its order is issued by the store at the
/// separation. Both editors show it through the store's view. It ends when its Undo works, when
/// the note no longer holds that identity on disk (deleted, or changed by another program), and
/// a campaign switch hides it. The change's note text is behind an `Arc`: a draw copies none of
/// it.
struct Separation {
    owner: SeparationOwner,
    change: Arc<IdentityChange>,
    /// The note's path in its campaign, read again by the disk check.
    path: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SeparationOwner {
    root: std::path::PathBuf,
    note: Uuid,
}

impl design::results::Owner for SeparationOwner {
    type Scope = std::path::Path;
    fn scope(&self) -> &std::path::Path {
        &self.root
    }
}

impl design::results::Kept for Separation {
    type Owner = SeparationOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        // The note was separated again: the newer separation replaces it (`record`).
        owner_changed: design::results::OwnerChanged::Ends,
        // It holds while the note at its path still has the identity the separation gave it.
        disk_changed: design::results::DiskChanged::Checked,
        // Its Undo writes the note back; it leaves only when that worked.
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> SeparationOwner {
        self.owner.clone()
    }
}

impl Separation {
    /// Whether the separation still holds: false only when the note on disk is read and has
    /// another identity (another program changed it). A note that cannot be read now (moved
    /// away for a moment, locked) keeps its result, so its Undo can be tried again.
    fn holds(&self) -> bool {
        #[cfg(any(test, feature = "test-support"))]
        SEPARATION_READS.with(|reads| reads.set(reads.get() + 1));
        match fs::read_to_string(self.owner.root.join(&self.path)) {
            Ok(text) => crate::content_workspace::note_id(&text) == Some(self.owner.note),
            Err(_) => true,
        }
    }

    /// The copy's file name, which tells two separations apart: `Rook copy.md`.
    fn copy_name(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or(&self.path)
    }

    /// Its Undo: `Undo separate Rook copy.md`, whose accessible name names the copy.
    fn undo_step(&self) -> design::outcome::UndoStep {
        design::outcome::UndoStep::of(design::outcome::Verb::Separate, self.copy_name())
    }
}

/// Why the last separation, or its Undo, failed: kept with its campaign and its copy, and shown
/// only in that campaign.
#[derive(Clone, Debug)]
struct SeparationFailure {
    root: std::path::PathBuf,
    path: String,
    words: String,
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How often the separations' disk check read a note.
    static SEPARATION_READS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// A note's identity changed (a separation or its Undo): the shell reads the campaign again.
fn note_identity_changed(ctx: &egui::Context) {
    ctx.data_mut(|data| data.insert_temp(egui::Id::new("encounter_identity_changed"), true));
}

/// A separation's change, with the note's text before and after it.
struct IdentityChange(crate::world::Change);

impl Clone for IdentityChange {
    fn clone(&self) -> Self {
        #[cfg(any(test, feature = "test-support"))]
        IDENTITY_COPIES.with(|copies| copies.set(copies.get() + 1));
        Self(self.0.clone())
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How often a separation's change, with its note text, was copied.
    static IDENTITY_COPIES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The replacement picker's five results for the lower-case `query`: the creatures that the
/// search finds as typed (no typo), best first in the one creature order (M3,
/// `matching::Typed::creature`) and then by origin, as the encounter search ranks them. They are
/// ranked before the five are taken, so the exact name is never left out. With no words, the
/// list's first five.
#[cfg(test)]
fn replacement_matches<'a>(creatures: &'a [Creature], query: &str) -> Vec<&'a Creature> {
    replacement_indices(creatures, query)
        .into_iter()
        .map(|at| &creatures[at])
        .collect()
}

/// `replacement_matches` as the places of the five in `creatures`. With no words it stops at
/// the fifth creature it can offer; with words it ranks as typed only, so no typo is worked
/// out (review of PR 450).
fn replacement_indices(creatures: &[Creature], query: &str) -> Vec<usize> {
    let offered = creatures
        .iter()
        .enumerate()
        .filter(|(_, c)| !c.linked)
        .inspect(|_| {
            #[cfg(any(test, feature = "test-support"))]
            REPLACEMENT_NAMES.with(|names| names.set(names.get() + 1));
        });
    if query.is_empty() {
        return offered.map(|(at, _)| at).take(5).collect();
    }
    let typed = crate::matching::Typed::new(query);
    let mut found: Vec<(crate::matching::Rank, crate::lookup::Origin, usize)> = offered
        .filter_map(|(at, c)| {
            let rank = typed.creature_as_typed(&c.lower, crate::matching::words(&c.lower))?;
            Some((rank, c.origin, at))
        })
        .collect();
    // Stable: the list's order stays inside one rank and origin.
    found.sort_by_key(|&(rank, origin, _)| (rank, origin));
    found.into_iter().map(|(.., at)| at).take(5).collect()
}

// Creatures the replacement picker visited on this thread, for a test that a repaint visits
// none.
#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static REPLACEMENT_NAMES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

// Kept pickers the replacement pickers' sweep visited on this thread, for a test that it runs
// once per frame (review of PR 473, P3).
#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static REPLACEMENT_SWEPT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
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

/// The creatures the roster's rows name, as a search's ranking reads them (issue #518): an
/// encounter's slots, read in place. A repaint compares them with the kept ranking's copy in one walk that allocates nothing; a copy is made
/// only when the ranking is made again.
#[derive(Clone, Copy)]
enum OnRoster<'a> {
    Slots(&'a [Slot]),
}

impl OnRoster<'_> {
    fn slot_creature(slot: &Slot) -> Option<Uuid> {
        slot.creature.as_ref().and_then(|reference| reference.id)
    }

    /// Whether the roster names `kept`, in order.
    fn names(self, kept: &[Uuid]) -> bool {
        match self {
            Self::Slots(slots) => slots
                .iter()
                .filter_map(Self::slot_creature)
                .eq(kept.iter().copied()),
        }
    }

    fn to_vec(self) -> Vec<Uuid> {
        #[cfg(any(test, feature = "test-support"))]
        ROSTER_COPIES.with(|copies| copies.set(copies.get() + 1));
        match self {
            Self::Slots(slots) => slots.iter().filter_map(Self::slot_creature).collect(),
        }
    }
}

/// The creatures a search offers: the campaign's read (`Snapshot::creatures`, by its revision),
/// all of them or only those no NPC links (a group's members).
#[derive(Clone, Copy)]
struct SearchList<'a> {
    creatures: &'a [Creature],
    unlinked: bool,
    revision: Uuid,
}

impl<'a> SearchList<'a> {
    fn creatures(self) -> impl Iterator<Item = &'a Creature> {
        self.creatures
            .iter()
            .filter(move |c| !self.unlinked || !c.linked)
    }
}

/// Everything a search's ranking reads. The same key ranks the same.
#[derive(Clone, PartialEq)]
struct SearchKey {
    list: Uuid,
    unlinked: bool,
    query: String,
    filter: Filter,
}

/// A search's ranking, kept for its view (the field's id) until its key changes: the creatures
/// found, in the one order, by their place in the list; and how many match as typed (no typo).
struct SearchRanked {
    key: SearchKey,
    /// The roster's creatures this ranking read (`OnRoster::names` checks them on a repaint).
    on_roster: Vec<Uuid>,
    found: Arc<Vec<usize>>,
    as_typed: usize,
}

impl SearchRanked {
    fn rank(
        key: SearchKey,
        list: SearchList<'_>,
        filter: &Filter,
        already: &dyn Fn(&Creature) -> bool,
        on_roster: Vec<Uuid>,
    ) -> Self {
        let parsed = Query::parse(&key.query);
        let typed = crate::matching::Typed::new(&parsed.words);
        let query = &key.query;
        let mut ranked: Vec<(crate::matching::Rank, usize)> = list
            .creatures
            .iter()
            .enumerate()
            .filter(|(_, c)| !list.unlinked || !c.linked)
            .filter(|(_, c)| {
                #[cfg(any(test, feature = "test-support"))]
                SEARCH_RANKED.with(|ranked| ranked.set(ranked.get() + 1));
                let facets = c.facets.as_ref();
                (!parsed.names_facts() || facets.is_some_and(|f| parsed.matches_facts(f)))
                    && (filter.is_empty() || facets.is_some_and(|f| filter.matches(f)))
                    && (!query.is_empty() || !c.party || !already(c))
            })
            .filter_map(|(at, c)| {
                if parsed.words.is_empty() {
                    return Some((crate::matching::Rank::typed(0), at));
                }
                let facts = c
                    .facets
                    .as_ref()
                    .is_some_and(|f| parsed.words_in(&c.name, f));
                // FI5, M3: the one creature order, as Lookup and Arrivals rank its names.
                let name = typed.creature(&c.lower, crate::matching::words(&c.lower));
                Some((crate::matching::with_other(name, facts)?, at))
            })
            .collect();
        // The plain name first, so "wolf" finds Wolf before Dire Wolf. A fact match comes
        // after every name match as typed, and a typo in the name comes last. The sort is
        // stable, so the list's order stays inside each rank.
        if !parsed.words.is_empty() {
            ranked.sort_by_key(|&(rank, at)| (rank, list.creatures[at].origin));
        }
        let as_typed = ranked.iter().take_while(|(rank, _)| !rank.typo).count();
        let found: Vec<usize> = ranked.into_iter().map(|(_, at)| at).collect();
        Self {
            key,
            on_roster,
            found: Arc::new(found),
            as_typed,
        }
    }
}

// Creatures the encounter search's ranking visited on this thread, for a test that a repaint
// visits none.
#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static SEARCH_RANKED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// How many times the search's results columns were measured on this thread (#565).
    static RESULT_COLUMNS_MEASURED: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// How many times a search copied the roster's creatures for its ranking (issue #518).
    static ROSTER_COPIES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

struct Snapshot {
    revision: Uuid,
    catalogue: Catalogue,
    known_creature_names: BTreeMap<String, String>,
    creatures: Vec<Creature>,
    templates: Vec<Template>,
    issues: Vec<String>,
    options: crate::encounter_settings::Options,
    saved_options: crate::encounter_settings::Options,
    campaign_text: String,
    runs: Vec<crate::encounter_run::SavedRunInfo>,
    /// Each read of `runs` (`take_run_list`): a new revision, for `RunFacts`.
    runs_read: crate::design::memo::Revised<()>,
    /// AB1, R41: the run list could not be read, and why. The footer says it in its tone, and
    /// In progress and History say it instead of their empty states. It ends when a read
    /// succeeds.
    runs_unread: Option<crate::encounter_run::RunFolderFault>,
    /// The run folder is not there and nothing names a run, so it may never have been made. The
    /// page turns this into `runs_unread` when it has seen this campaign's folder hold runs.
    runs_missing: Option<crate::encounter_run::RunFolderFault>,
    groups: Arc<Vec<Arc<crate::encounter_run::RunGroup>>>,
    run_creatures: Arc<Vec<Arc<crate::encounter_run::RunCreature>>>,
    /// The campaign's XP budget table and its party's levels, for each fight's difficulty.
    scale: Arc<crate::difficulty::Scale>,
}
/// Load what the Encounters page shows, for the load benchmark.
#[cfg(any(test, feature = "test-support"))]
pub fn load_for_bench(campaign: &Campaign, modules: &[Module]) -> Result<(), String> {
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
    /// The creatures a search offers from this read: all, or only those no NPC links.
    fn search_list(&self, unlinked: bool) -> SearchList<'_> {
        SearchList {
            creatures: &self.creatures,
            unlinked,
            revision: self.revision,
        }
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
        let scale = crate::difficulty::Scale {
            budgets: crate::encounter_settings::xp_budgets(campaign, &catalogue, &document),
            party: campaign_party(campaign, &catalogue, &creatures),
        };
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
                let difficulty = setup
                    .as_ref()
                    .map(|setup| {
                        roster_difficulty(setup, &catalogue, &known_creature_names, &scale)
                    })
                    .unwrap_or_default();
                Template {
                    path: e.entry.path.clone(),
                    id: e.entry.id,
                    name: e.entry.name.clone(),
                    summary: setup.as_ref().map(|setup| {
                        history::Roster::new(
                            roster_summary(setup, &catalogue, &known_creature_names),
                            &difficulty,
                        )
                    }),
                    difficulty,
                    repeatable: setup.is_some_and(|setup| setup.repeatable),
                }
            })
            .collect();
        templates.sort_by_key(|template| template.name.to_lowercase());
        let issues: Vec<String> = catalogue
            .errors
            .iter()
            .chain(&closure.errors)
            .cloned()
            .collect();
        let options = crate::encounter_settings::Options::load(campaign, &catalogue)?;
        let directory = options.directory(campaign)?;
        let read = crate::encounter_run::Run::list(&directory, campaign.id);
        // A folder the timeline names runs in is not one that was never made.
        let missing = matches!(read, Err(crate::encounter_run::RunFolderFault::Missing(_)))
            && !crate::sessions::folder(campaign)
                .ok()
                .and_then(|folder| fs::read_to_string(folder.join("Timeline.md")).ok())
                .is_some_and(|text| {
                    text.lines()
                        .any(|line| line.trim_start().starts_with("run:"))
                });
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
        let mut snapshot = Self {
            revision: Uuid::new_v4(),
            groups,
            known_creature_names,
            run_creatures,
            scale: Arc::new(scale),
            options: options.clone(),
            saved_options: options,
            campaign_text: document.text,
            runs: Vec::new(),
            runs_read: crate::design::memo::Revised::new(()),
            runs_unread: None,
            runs_missing: None,
            catalogue,
            creatures,
            templates,
            issues,
        };
        snapshot.take_run_list(read, missing);
        Ok(snapshot)
    }
    /// AB1: the one classification of a run list read, for the first load, a reload and every
    /// refresh. Runs that read replace the list. A folder known not to be there may never have
    /// been made, so it is `runs_missing` until the page knows it held runs. Any other failure
    /// (an unreadable folder or parent) is `runs_unread` until a read succeeds. The fault is
    /// said by the footer and the two lists, never again as a content issue.
    fn take_run_list(
        &mut self,
        read: Result<Vec<crate::encounter_run::SavedRunInfo>, crate::encounter_run::RunFolderFault>,
        missing: bool,
    ) {
        self.runs_missing = None;
        self.runs_unread = None;
        self.runs_read.revise(|_| crate::design::memo::Changed::Yes);
        match read {
            Ok(runs) => self.runs = runs,
            Err(error) => {
                self.runs.clear();
                if missing {
                    self.runs_missing = Some(error);
                } else {
                    self.runs_unread = Some(error);
                }
            }
        }
    }
    /// The encounters a group can belong to, in the list's order.
    fn encounter_choices(&self) -> impl Iterator<Item = (Uuid, &str)> {
        self.templates
            .iter()
            .map(|template| (template.id, template.name.as_str()))
    }
}
/// One of a draft's own results in the editor's footer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DraftResult {
    RemovedGroup,
    Replacement,
}

/// Whose editor result it is: the campaign, the encounter draft's content (`results_key`), and
/// its kind (one of each per draft; a newer one of a kind replaces it).
#[derive(Clone, Debug, PartialEq, Eq)]
struct EditorOwner {
    root: std::path::PathBuf,
    draft: Uuid,
    kind: DraftResult,
}

impl design::results::Owner for EditorOwner {
    type Scope = std::path::Path;
    fn scope(&self) -> &std::path::Path {
        &self.root
    }
}

/// An encounter draft's result in the page's store: its owner and what its Undo gives back. The
/// data lives here, with the ticket, never beside the store in the draft: a result the body makes
/// in the frame of a press cannot be what an older result's Undo restores.
struct EditorMark {
    owner: EditorOwner,
    /// Its draft's key: the result ends when no copy of its draft is left.
    draft: results_key::KeyWatch,
    undo: DraftUndo,
}

/// What an encounter draft's result's Undo gives back, held in the page's store with its ticket.
#[derive(Clone)]
enum DraftUndo {
    /// A group detached from this draft (FB7), until the next save: its place, the group, and
    /// whether it is deleted (a move keeps it in the draft, owned by no encounter).
    RemovedGroup {
        index: usize,
        group: Box<GroupDraft>,
        delete: bool,
    },
    /// The replacement of missing roster rows (FA2): the roster before and after, and its words.
    Replacement {
        before: Vec<Slot>,
        after: Vec<Slot>,
        words: String,
        /// The replaced creature, as the rows named it: its Undo names it.
        replaced: String,
    },
}

impl DraftUndo {
    fn kind(&self) -> DraftResult {
        match self {
            Self::RemovedGroup { .. } => DraftResult::RemovedGroup,
            Self::Replacement { .. } => DraftResult::Replacement,
        }
    }
    /// Whether it still holds: a replacement ends at an edit of the replaced rows.
    fn holds(&self, draft: &Draft) -> bool {
        match self {
            Self::Replacement { after, .. } => *after == draft.setup.roster,
            Self::RemovedGroup { .. } => true,
        }
    }
    /// Its footer line, from its own value. FF7: a group removed with its draft after its file
    /// was gone says what was lost. FB7: nothing is on disk until Save, so a removed group is a
    /// notice; a replacement is a finished change.
    fn outcome(&self) -> design::outcome::KeptOutcome<FooterPress> {
        use design::outcome::{KeptOutcome, Notice, Said, UndoStep, Verb};
        match self {
            Self::RemovedGroup { group, delete, .. } => {
                let (words, step) = if group.file_gone() {
                    (
                        disk::removed_with_draft(&group.name()),
                        UndoStep::of(Verb::Remove, group.name()),
                    )
                } else if *delete {
                    (
                        "Group deleted. Save deletes its file.".to_owned(),
                        UndoStep::of(Verb::Remove, "group"),
                    )
                } else {
                    (
                        format!("{} moves to any encounter when you save.", group.name()),
                        UndoStep::verb(Verb::Move),
                    )
                };
                KeptOutcome::Notice(
                    Notice::new(outcome_words(&words)),
                    step.pressed_as(FooterPress::UndoEditor),
                )
            }
            Self::Replacement {
                words, replaced, ..
            } => Said::new(outcome_words(words))
                .undo(
                    UndoStep::of(Verb::Replace, replaced.clone())
                        .pressed_as(FooterPress::UndoEditor),
                )
                .into(),
        }
    }
}

impl design::results::Kept for EditorMark {
    type Owner = EditorOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        // Its data went: a Save, a Discard, an edit of the replaced rows, a newer result.
        owner_changed: design::results::OwnerChanged::Ends,
        // Nothing is on disk until Save.
        disk_changed: design::results::DiskChanged::Stays,
        // Its Undo gives the draft its rows or its group back.
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> EditorOwner {
        self.owner.clone()
    }
}

/// A kept result with its own Undo, on the Encounters page or in an editor's footer. Each is
/// owned by the state that holds its data (`discarded`, `discarded_group`, `deleted_run`,
/// `removed_saved_group`, `removed_encounter`) and ends with it (FI1, FJ4). Its footer order is
/// issued when its action is done (`stamp_kept`).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum KeptResult {
    Discarded,
    GroupDiscarded,
    RunDeleted,
    GroupRemoved,
    EncounterRemoved,
    /// `Encounter saved.` or `Group saved.` (`message`), with no Undo.
    Saved,
    /// `Rules and runs draft discarded.` with its Undo (`discarded_options`): only Rules and runs
    /// shows it (`run_settings`).
    OptionsDiscarded,
}

/// Whose kept result it is: the campaign and its kind. One of each kind per campaign, as each
/// kind's data is held once (`discarded`, `deleted_run`, …). A campaign switch hides it.
#[derive(Clone, Debug, PartialEq, Eq)]
struct KeptOwner {
    root: std::path::PathBuf,
    kind: KeptResult,
}

impl design::results::Owner for KeptOwner {
    type Scope = std::path::Path;
    fn scope(&self) -> &std::path::Path {
        &self.root
    }
}

/// A kept result in the page's store: its owner, for the footer's order and the press's ticket.
/// Its words come from the data its kind holds (`Encounters::kept_outcome`), which ends it.
struct KeptMark(KeptOwner);

impl design::results::Kept for KeptMark {
    type Owner = KeptOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        // Its data was dropped by a later action (a new draft, a save, another removal).
        owner_changed: design::results::OwnerChanged::Ends,
        // A reload keeps the data it undoes (a removed file comes back only by its Undo).
        disk_changed: design::results::DiskChanged::Stays,
        // Its Undo puts its data back; it leaves only when that worked.
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> KeptOwner {
        self.0.clone()
    }
}

/// A discard's kept line: `<name> draft discarded.` with `Undo discard of <name>`, or, for a draft
/// whose file was gone, `<name> removed. Its draft was discarded.` (FF7: it was removed with its
/// file, and its Undo says so). One form for the editor's footer and the Session plan's.
fn discard_outcome<A>(name: String, gone: bool, undo: A) -> design::outcome::KeptOutcome<A> {
    use design::outcome::{KeptOutcome, Notice, UndoStep, Verb};
    if gone {
        KeptOutcome::Notice(
            Notice::new(crate::sentence!(
                "{} removed. Its draft was discarded.",
                name
            )),
            UndoStep::of(Verb::Remove, name.clone()).pressed_as(undo),
        )
    } else {
        KeptOutcome::Notice(
            Notice::new(crate::sentence!("{} draft discarded.", name)),
            UndoStep::of(Verb::Discard, name).pressed_as(undo),
        )
    }
}

/// A fault on an Encounters pane, by kind: its severity and its words in one place.
#[derive(Clone, Debug, PartialEq)]
enum EncounterFault {
    /// An act failed: the system's or the check's words.
    Failed(String),
    /// A deleted run had changes that never reached D&D Beyond: the result says so, in the
    /// warning tone, and keeps its Undo.
    RunDeletedUnsent(String),
    /// R41: the campaign's run folder is not there, cannot be read, or is not set: the DM
    /// chooses another. Its words and tone come from the fault.
    RunFolder(crate::encounter_run::RunFolderFault),
    /// A copied note's separation, or its Undo, failed: the reason.
    SeparationFailed(String),
    /// A warning of a group's file that no member row states.
    FileWarning(String),
    /// Rules and runs (ruling M4): the saved runs a folder change leaves, which is why `Save run
    /// settings` waits; it carries the two move choices. A reason, so an act's answer below it
    /// never moves the choice the DM pressed.
    RunsLeft(String),
    /// A flag of the encounter's file (`unread_notes`) in its tone: an unread roster row is an
    /// error (R41, `unread_row_tone`), any other flag a warning.
    FileFlag(String, Tone),
    /// The draft cannot be saved as it is: a missing creature, an empty group. The DM fixes
    /// it in the editor.
    Invalid(String),
    /// The Encounters page waits for the previous campaign's open encounter (`show_page`).
    PreviousCampaignDraft,
    /// Start fight was refused (ruling M7): no fight started, why, and the next step, in the
    /// tone of its cause.
    StartRefused(design::results::Refusal),
}

impl EncounterFault {
    /// Save's refusal `words` in the tone of its diagnosis `found` (R41): the one place that
    /// turns a refusal into a footer fault, so the footer and the save row's word agree.
    fn refusal(words: String, found: &design::save_row::Diagnosis<'_>) -> Self {
        match found.tone() {
            Some(Tone::Error) => Self::Failed(words),
            _ => Self::Invalid(words),
        }
    }
}

impl design::outcome::Fault for EncounterFault {
    fn severity(&self) -> design::outcome::Severity {
        match self {
            Self::Failed(_) => design::outcome::Severity::NotSaved,
            Self::FileFlag(_, Tone::Error) => design::outcome::Severity::CannotRead,
            Self::RunFolder(fault) => fault.severity(),
            Self::RunDeletedUnsent(_)
            | Self::SeparationFailed(_)
            | Self::FileWarning(_)
            | Self::RunsLeft(_)
            | Self::FileFlag(..)
            | Self::Invalid(_)
            | Self::PreviousCampaignDraft => design::outcome::Severity::NeedsFixing,
            Self::StartRefused(refusal) => match refusal.tone() {
                Tone::Error => design::outcome::Severity::CannotRead,
                _ => design::outcome::Severity::NeedsFixing,
            },
        }
    }
    #[allow(
        clippy::disallowed_methods,
        reason = "the faults carry the system’s and the run’s words"
    )]
    fn cause(&self) -> design::outcome::Sentence {
        match self {
            Self::Failed(words) => design::outcome::Sentence::outside(words.as_str())
                .unwrap_or(crate::sentence!("The act did not work.")),
            Self::RunDeletedUnsent(words) => design::outcome::Sentence::outside(words.as_str())
                .unwrap_or(crate::sentence!("The run was deleted.")),
            Self::RunFolder(fault) => design::outcome::Sentence::outside(fault.message().as_str())
                .unwrap_or(crate::sentence!("Splat cannot list the run folder.")),
            Self::SeparationFailed(words) => design::outcome::Sentence::outside(words.as_str())
                .unwrap_or(crate::sentence!("The note was not changed.")),
            Self::FileWarning(words) => design::outcome::Sentence::outside(words.as_str())
                .unwrap_or(crate::sentence!("The group’s file has a problem.")),
            Self::RunsLeft(words) => outcome_words(words),
            Self::FileFlag(words, _) => design::outcome::Sentence::outside(words.as_str())
                .unwrap_or(crate::sentence!("The encounter’s file has a problem.")),
            Self::Invalid(words) => design::outcome::Sentence::outside(words.as_str())
                .unwrap_or(crate::sentence!("Save cannot write this draft yet.")),
            Self::PreviousCampaignDraft => {
                crate::sentence!("Save or discard the previous campaign’s encounter first.")
            }
            Self::StartRefused(refusal) => refusal.words(),
        }
    }
}

/// Words the page keeps as a string (a result's words made by the app, with the names it
/// shows) as a footer sentence: the named escape for words that are not one template.
#[allow(
    clippy::disallowed_methods,
    reason = "a kept result’s words are made by the app as a string"
)]
fn outcome_words(words: &str) -> design::outcome::Sentence {
    design::outcome::Sentence::outside(words).unwrap_or(crate::sentence!("Done."))
}

/// What a press in an Encounters footer asks for. An Undo names its line by the press's ticket.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FooterPress {
    Undo,
    /// The Undo of the encounter draft's own result (`editor_results`).
    UndoEditor,
    ChooseFolder,
    /// The Undo of a group draft's member result (`member_results`).
    UndoMember,
    /// The Undo of a copied note's separation (`separations`).
    UndoSeparation,
    /// Rules and runs: the run folder warning's choices (ruling M4). `Save run settings` waits
    /// in its place while the folder change leaves saved runs.
    MoveRuns,
    SaveWithoutMoving,
}

/// Whose member result it is: the campaign and the group draft's content (`results_key`). A
/// draft has one member result at a time (a newer one replaces it).
#[derive(Clone, Debug, PartialEq, Eq)]
struct MemberOwner {
    root: std::path::PathBuf,
    draft: Uuid,
}

impl design::results::Owner for MemberOwner {
    type Scope = std::path::Path;
    fn scope(&self) -> &std::path::Path {
        &self.root
    }
}

/// A group draft's member result in the page's store: its owner and what its Undo gives back.
/// The data lives here, with the ticket, never beside the store in the draft: a member removed
/// by the body in the frame of a press cannot be what an older result's Undo restores.
struct MemberMark {
    owner: MemberOwner,
    /// Its group draft's key: the result ends when no copy of its draft is left.
    draft: results_key::KeyWatch,
    undo: groups::MemberUndo,
}

impl MemberMark {
    /// Its footer line, from its own value.
    fn outcome(&self) -> design::outcome::KeptOutcome<FooterPress> {
        design::outcome::Said::new(outcome_words(&self.undo.words()))
            .undo(self.undo.step().pressed_as(FooterPress::UndoMember))
            .into()
    }
}

impl design::results::Kept for MemberMark {
    type Owner = MemberOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        // Its data went: its member removed or changed, a newer result, a Save of the draft.
        owner_changed: design::results::OwnerChanged::Ends,
        // Nothing is on disk until Save.
        disk_changed: design::results::DiskChanged::Stays,
        // Its Undo puts the member back in the draft.
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> MemberOwner {
        self.owner.clone()
    }
}

/// What the saved runs say that a kept result reads (`timeline::fights_that_keep_hp`: which
/// fights keep their saved HP): which read of the run list it is, and the open run's own facts
/// over it (its id, and whether it is running). Its revision changes when either changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RunFacts {
    listed: Option<crate::design::memo::Revision>,
    open: Option<(Uuid, bool)>,
}

/// [`RunFacts`] with its revision, kept by the page.
struct RunFactsRevised(crate::design::memo::Revised<RunFacts>);

impl Default for RunFactsRevised {
    fn default() -> Self {
        Self(crate::design::memo::Revised::new(RunFacts {
            listed: None,
            open: None,
        }))
    }
}

/// The Session plan's fights and groups, shared, not copied (`Encounters::plan_kept`).
#[derive(Clone, Default)]
pub struct PlanKept {
    pub fights: Arc<[PlanFight]>,
    pub groups: Arc<[Arc<crate::encounter_run::RunGroup>]>,
}

#[derive(Default)]
pub struct Encounters {
    active: Option<Campaign>,
    snapshot: Option<Snapshot>,
    /// The saved-run facts that the Session plan's and the Timeline's Add results read, with
    /// their revision (`saved_runs_revision`). Noted at the end of each `poll`.
    run_facts: RunFactsRevised,
    /// The snapshot that `plan_fights` lists, as its revision: a new read or a save of an
    /// encounter makes a new snapshot revision, and so a new revision here.
    plan_fights_read: crate::design::memo::Revised<Option<Uuid>>,
    /// The Session plan's fights and groups, as `plan_fights` and `plan_groups` list them, kept
    /// by that revision (layout 11c A2a): a quiet frame copies none.
    plan_kept: crate::design::memo::Memo<
        (crate::design::memo::Revision, crate::design::memo::Revision),
        PlanKept,
    >,
    /// How often the plan's lists were listed again (a test counts them).
    plan_kept_builds: usize,
    /// G118: the Ready rows' difficulty lines, by template id, each under its `Roster::key`,
    /// room, font and scale. The page owns them; a new key replaces an entry, and a template
    /// that leaves the snapshot loses its entry at the next snapshot revision.
    ready_lines: crate::difficulty::Kept<Uuid>,
    /// The snapshot revision that `ready_lines` was last trimmed to.
    ready_lines_seen: Option<Uuid>,
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
    /// The typed answer to the last refused Start fight (`Invalid::start_refusal`): the draft
    /// Start checked, the words of that check, and the answer. Owner: that draft's last Start.
    /// It shows only while the draft is the one Start checked (the same draft, unchanged: Start
    /// runs only on a clean draft) and `error` still holds the check's words. So an edit hides it
    /// and an Undo back to that draft shows it again; a new check's record (a Save, another
    /// Start: `refused`) drops it, and a reload or another campaign (`error` cleared) ends it.
    start_refused: Option<(Uuid, String, design::results::Refusal)>,
    validation_path: Option<String>,
    /// A Save or a start was refused for the draft at `validation_path`, so its check runs again
    /// when that draft or the campaign changes. It ends with another draft.
    refusal_watch: bool,
    /// B4c: the plan refusal's check, keyed by the snapshot revision and `plan_epoch`: whether
    /// the refusal for that group still stands. An unchanged frame does not compare the drafts.
    plan_refusal_check: Option<(PlanRefusalKey, bool)>,
    /// Bumped on each frame with an editing input and the frame after it, while a plan refusal
    /// is stored (as `edit_epoch`, for every page).
    plan_epoch: u64,
    plan_edited_last_frame: bool,
    /// Test only: while set, `poll` takes no new read of the campaign (a load in flight waits),
    /// so a proof's window of unchanged frames keeps one snapshot revision.
    #[cfg(any(test, feature = "test-support"))]
    pub hold_reads: bool,
    /// Bumped on each frame that can change a draft (an editing input) and the frame after it,
    /// because a click's change lands late in its frame.
    edit_epoch: u64,
    edited_last_frame: bool,
    /// The snapshot revision, edit epoch and draft kind of the last check. An unchanged frame
    /// with the same key does not check the draft again.
    validated: Option<(Uuid, u64, bool)>,
    /// How often frames reached that key's guard.
    #[cfg(any(test, feature = "test-support"))]
    validation_reached: usize,
    /// Save's check of the open group draft (FN5, AI2), with the snapshot revision and the
    /// draft's own revision it was made for. A frame with no change does not check again.
    group_check: Option<((Uuid, u64), Option<groups::Refusal>)>,
    /// N1: Save's checks of the open encounter's changed groups, as one ranked diagnosis with
    /// the origin of the first group that only lacks a part. Keyed by the snapshot revision and
    /// the changed groups' own revisions, so an unchanged frame does not check them again.
    owned_group_check: Option<((Uuid, Uuid, usize, u64), OwnedGroupsFound)>,
    /// The open draft's difficulty, with the snapshot revision, edit epoch and draft it was
    /// worked out for.
    difficulty: Option<((Uuid, u64, Uuid), Arc<crate::difficulty::Difficulty>)>,
    /// How often frames asked for the draft's difficulty, and how often it was worked out.
    #[cfg(any(test, feature = "test-support"))]
    difficulty_counts: (usize, usize),
    message: Option<String>,
    /// The draft `Discard changes` last dropped, for one step of undo.
    discarded: Option<Draft>,
    /// The Rules and runs draft that Discard set aside; its result is in `kept`
    /// (`KeptResult::OptionsDiscarded`), which issued its order at the discard.
    discarded_options: Option<crate::encounter_settings::Options>,
    /// Rules and runs' `Saved` (round 31 SD7, save-result-lifetime): the campaign and the
    /// settings that its last Save wrote (Save run settings, a repair of an unread choice, or
    /// one of the folder move's two choices). Its save row says `Saved` while the form and the
    /// file still hold exactly these settings. It ends at the form's next edit, at a campaign
    /// switch, and when the file's settings change outside Splat (`run_settings`).
    options_saved: Option<(Uuid, crate::encounter_settings::Options)>,
    pub saved: bool,
    run: Option<crate::encounter_run::Run>,
    /// The encounter editor's last frame found that Save would refuse its changed draft, so
    /// Ctrl+S is blocked as Save is. The editor sets it each frame it draws.
    save_refused: bool,
    /// History is a read-only snapshot and never owns the open-run slot.
    history_read: Option<(std::path::PathBuf, crate::encounter_run::FightSummary)>,
    /// A run just opened in History: the list scrolls once, so its Final state is in view.
    history_reveal: bool,
    /// Why the fight read in History cannot go on (`continue_blocked`), when it cannot.
    history_blocked: Option<&'static str>,
    /// What the rule's answers were worked out from: the Timeline's count of changes, the
    /// open run if it is ended, the ID of the fight read in History, and whether an edit waits
    /// over a file that changed on disk. Not the fight's path: another
    /// run can take the file at that path.
    continue_key: Option<(u64, Option<uuid::Uuid>, Option<uuid::Uuid>, bool)>,
    /// A run just opened, or the DM asked to go back to it from its row in the list, so the
    /// shell goes to the Run page.
    pub go_to_run: bool,
    /// A creature to read beside the draft, in Lookup.
    pub peek: Option<Uuid>,
    focus_name: bool,
    focus_search: bool,
    /// Ctrl+S taken this frame by `save_shortcut`. The save is done after the fields it writes
    /// have taken this frame's input (the creature's pane, the editor's `then`, the group
    /// editor), so a key typed in the frame of Ctrl+S is saved too. `show_page` ends it.
    save_key: bool,
    editor_pane: EditorPane,
    expanded: Option<String>,
    group_draft: Option<GroupDraft>,
    /// The group draft `Discard changes` last dropped, for one step of undo.
    discarded_group: Option<GroupDraft>,
    /// When that discard was made, for the plan's footer order.
    /// A creature made the campaign's own, with its text open beside the roster.
    creature: Option<CreatureEditor>,

    /// The session the open encounter came from, which its way back returns to. The route ends
    /// when the DM leaves the editor by another way (`end_session_route`).
    pub from_session: Option<Uuid>,
    /// A new encounter joins the session's plan when it is first saved.
    plan_on_save: bool,
    pub saved_for_plan: Option<String>,
    /// A group's old and new name, for the notes that link it.
    pub renamed_group: Option<(String, String)>,
    /// The names linked in tonight's plan, for an open run.
    pub planned_groups: Vec<String>,
    /// The open group draft goes back to the session's plan when it is saved.
    group_in_plan: bool,
    pub wants_session: Option<Uuid>,
    pub wants_run_folder: bool,
    /// The run just opened began on the Session page, by way of this editor.
    pub run_from_session: bool,
    /// Where the party stands after the last fight of the session being played.
    pub carry: Option<crate::encounter_run::Carry>,
    /// The saved run `Delete` last removed, for one step of undo.
    deleted_run: Option<crate::encounter_run::DeletedRun>,
    removed_saved_group: Option<groups::RemovedNote>,
    /// The encounter `Remove` last removed, for `Undo remove encounter`.
    removed_encounter: Option<groups::RemovedNote>,
    /// Each kept result's owner and order, recorded when its action is done (`stamp_kept`); the
    /// footers show them newest first. The store itself lives as long as the page; a result
    /// leaves it when its data goes (`kept_results`): a full reload (`reload`) keeps the store
    /// but drops the removal data (a deleted run, a removed group or encounter), so those lines
    /// end; a campaign change also drops the discarded drafts and settings; an edit of a new
    /// draft drops the discard of its kind (`expire_discards`); a successful Undo takes its data
    /// back. A campaign switch hides the other campaign's lines (the owner's scope).
    kept: design::Results<KeptOwner, KeptMark>,
    /// FF8: the creature search's separations of copied notes, each with its Undo (`Separation`).
    /// It lives with the page, so both editors show it and a reload does not drop it; the disk
    /// check ends one whose note changed (`refresh_separations`).
    separations: design::Results<SeparationOwner, Separation>,
    /// Why the last separation, or its Undo, failed, with its campaign and copy. A new
    /// separation or a working Undo ends it; it shows only in its campaign.
    separation_error: Option<SeparationFailure>,
    /// The count of dropped results keys at the last end of dead results (`end_dead_results`).
    dead_keys_seen: u64,
    /// Each group draft's member result (`MemberMark`), for both editors. It lives with the page,
    /// so a Discard and its Undo, which move the draft, keep the order.
    member_results: design::Results<MemberOwner, MemberMark>,
    /// Each encounter draft's own results (a removed group, a replacement), for its editor.
    editor_results: design::Results<EditorOwner, EditorMark>,
    /// The GPUI page's creature search: its ranking and its rows, kept until the ranking's key
    /// or the roster it read changes (`model::Encounters::add_view`).
    search_ranked: Option<(Arc<SearchRanked>, Arc<Vec<model::ResultRow>>)>,
}
pub use splat_session_records::encounters::PlanFight;
/// What the page needs from the story: where each run's event is, and which sessions exist.
pub struct Story<'a> {
    pub events: &'a [crate::timeline::Event],
    pub sessions: &'a [crate::timeline::SessionLink],
    /// What the two say, as their sources' revisions: the Timeline's events
    /// (`Timeline::revision`) and the Sessions folder's listing (`Sessions::notes_revision`).
    /// The ledger keeps In progress by it (review of #875, #589). `None`: a story with no
    /// sources (a test's fixed lists), so what reads it is worked out on each frame.
    pub revision: Option<(crate::design::memo::Revision, u64)>,
}
/// What the DM asked for from a row of the list. One press in a frame, so one of these.
#[derive(Clone, Debug)]
pub enum ListAction {
    Resume(std::path::PathBuf),
    ReadHistory(std::path::PathBuf),
    ContinueFight(std::path::PathBuf),
    BackToRun,
    Delete(std::path::PathBuf),
    Edit(String),
    Run(String),
    OpenGroup(String),
    RemoveGroup(String),
    RemoveEncounter(String),
}
#[derive(Clone, Copy, Default, PartialEq)]
enum EditorPane {
    #[default]
    Roster,
    Add,
}
const START_FIGHT: &str = "Start fight";

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How often this thread ran an encounter draft's full check.
    static VALIDATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// How many result keys this thread checked (`KeyWatch::alive`).
    static KEY_CHECKS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
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
/// The open encounter's draft holds the page (the same words as its back line's reason).
const BLOCKED_BY_DRAFT: &str = design::Unsaved::Encounter.reason();
/// The rail's words for this page's drafts; the rail opens Encounters for each (FF2).
pub const ENCOUNTER_DRAFT: &str = "Encounter draft";
pub const GROUP_DRAFT: &str = "Group draft";
pub const CREATURE_DRAFT: &str = "Creature draft";
pub const RUN_SETTINGS_DRAFT: &str = "Run settings draft";
const BLOCKED_BY_CREATURE: &str = design::Unsaved::Creature.reason();
/// The key of the plan refusal's cached check: the campaign read's revision, the edit epoch and
/// the refused group's id.
type PlanRefusalKey = (Option<Uuid>, u64, Uuid);

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// B4b N1, B4c F1: how often a draft was compared for being unsaved (`DraftOwner::unsaved`),
    /// on this thread.
    pub static PLAN_BLOCK_CHECKS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// What a plan or run route that opens a draft got (B4 review F1): the caller gives the draft
/// its host only when it is `Opened`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlanOpen {
    Opened,
    /// An unsaved draft refuses it, with that draft's reason; every draft is kept.
    Refused(&'static str),
    /// The file cannot be read; Encounters says why.
    Failed,
}

/// R38: the drafts that hold an Encounters editor's back line, from the same values that Save
/// and the replacement guards read.
impl design::DraftOwner for Encounters {
    fn unsaved(&self, draft: design::Unsaved) -> bool {
        #[cfg(any(test, feature = "test-support"))]
        PLAN_BLOCK_CHECKS.with(|checks| checks.set(checks.get() + 1));
        match draft {
            design::Unsaved::Group => self.group_draft.as_ref().is_some_and(GroupDraft::dirty),
            design::Unsaved::Encounter => self.draft.as_ref().is_some_and(Draft::dirty),
            design::Unsaved::Creature => self.creature_dirty(),
        }
    }
}

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
        #[cfg(any(test, feature = "test-support"))]
        {
            self.validation_reached += 1;
        }
        let key = (snapshot.revision, self.edit_epoch, group);
        if self.validated == Some(key) {
            return;
        }
        self.validated = Some(key);
        let (invalid, shown) = if group {
            // G122: a group's refusal keeps its place: the footer says it only when no other
            // place does, as Save's own refusal (`refuse_group_save`).
            let refusal = self
                .group_draft
                .as_ref()
                .and_then(|draft| draft.check(snapshot, None).err());
            let shown = refusal
                .as_ref()
                .and_then(groups::Refusal::footer)
                .map(str::to_owned);
            (
                refusal.map(|refusal| Invalid::from(refusal.text().to_owned())),
                shown,
            )
        } else {
            let invalid = self
                .draft
                .as_ref()
                .and_then(|draft| draft.validate(snapshot).err());
            let shown = invalid.as_ref().map(|invalid| invalid.text.clone());
            (invalid, shown)
        };
        self.error = shown;
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
        self.start_refused = None;
    }
    /// A refused Start's record: the check's, then its typed answer when its cause has one.
    fn start_refused(&mut self, invalid: Option<Invalid>) {
        let answer = invalid.as_ref().and_then(|invalid| {
            let draft = self.draft.as_ref()?;
            Some((
                draft.id,
                invalid.text.clone(),
                invalid.start_refusal(&draft.name)?,
            ))
        });
        self.refused(invalid);
        self.start_refused = answer;
    }
    /// N1: Save's checks of the open encounter's changed groups as one ranked diagnosis: the
    /// first refused change (named `Group <name>: …`, as Save says it) outranks the first missing
    /// part, whose group's origin comes with it. Checked again only when the campaign's read or a
    /// changed group's own revision changes.
    /// F5 (B3b): the owned-group check's key inputs now, and whether a campaign read is still
    /// pending: a test holds them fixed and names the input that changed.
    #[cfg(any(test, feature = "test-support"))]
    pub fn owned_group_inputs(&self) -> Option<(Uuid, usize, u64, bool)> {
        let (snapshot, draft) = (self.snapshot.as_ref()?, self.draft.as_ref()?);
        Some((
            snapshot.revision,
            draft.groups.len(),
            groups::revisions_issued(),
            self.pending.is_some(),
        ))
    }
    fn owned_groups_diagnosis(&mut self) -> OwnedGroupsFound {
        let (Some(snapshot), Some(draft)) = (self.snapshot.as_ref(), self.draft.as_ref()) else {
            return OwnedGroupsFound {
                found: design::save_row::Diagnosis::valid(),
                origin: None,
                footer: None,
                cause: None,
            };
        };
        #[cfg(any(test, feature = "test-support"))]
        OWNED_GROUP_WORK.with(|work| work.set((work.get().0 + 1, work.get().1)));
        let key = owned_groups_key(snapshot, draft);
        if let Some((cached, found)) = &self.owned_group_check
            && *cached == key
        {
            return found.clone();
        }
        #[cfg(any(test, feature = "test-support"))]
        OWNED_GROUP_WORK.with(|work| work.set((work.get().0, work.get().1 + draft.groups.len())));
        let found = owned_groups_found(&draft.groups, snapshot, draft.id);
        self.owned_group_check = Some((key, found.clone()));
        found
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
    #[cfg(any(test, feature = "test-support"))]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    /// Block until the load in flight has its result (its completion signal, never a clock),
    /// and hand that result to the next `poll`, which takes it as it takes any. A test's wait.
    #[cfg(any(test, feature = "test-support"))]
    pub fn wait_for_load(&mut self) {
        if let Some(rx) = self.pending.take() {
            let (tx, ready) = mpsc::channel();
            if let Ok(result) = rx.recv() {
                let _ = tx.send(result);
            }
            self.pending = Some(ready);
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_sync_conflict(&mut self, name: &str) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_sync_conflict(name);
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_connection(&mut self) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_connection();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_sync_state(&mut self, name: &str, state: crate::combat_sync::SyncState) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_sync_state(name, state);
    }

    /// Every row with each time taken away, each effect as its plain words (rule P, X5).
    /// Returns the conditions as they were, for `simulate_conditions`.
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_untimed(&mut self) -> Vec<Vec<crate::encounter_run::Condition>> {
        self.run.as_mut().expect("an open run").simulate_untimed()
    }

    /// Give each row back the conditions that `simulate_untimed` returned.
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_conditions(&mut self, conditions: Vec<Vec<crate::encounter_run::Condition>>) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_conditions(conditions);
    }

    /// A participant under another name. No fixture has a name this long.
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_rename(&mut self, name: &str, to: &str) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_rename(name, to);
    }

    /// A read of this participant's sheet, holding its last matched damage plus `more_damage`.
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_read(&mut self, name: &str, more_damage: i64) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_read(name, more_damage);
    }

    /// Running syncs react to the session from now on, as real ones do.
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_session_end(&mut self) {
        self.run
            .as_mut()
            .expect("an open run")
            .stop_simulating_sync();
    }

    /// Show an error line. An empty message clears it.
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_run_error(&mut self, message: &str) {
        self.run
            .as_mut()
            .expect("an open run")
            .simulate_error(message);
    }

    #[cfg(any(test, feature = "test-support"))]
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
    /// The open draft's difficulty. Worked out again only when the draft, an edit or the
    /// campaign's read changes.
    fn draft_difficulty(&mut self) -> Option<Arc<crate::difficulty::Difficulty>> {
        let (snapshot, draft) = (self.snapshot.as_ref()?, self.draft.as_ref()?);
        let key = (snapshot.revision, self.edit_epoch, draft.id);
        #[cfg(any(test, feature = "test-support"))]
        {
            self.difficulty_counts.0 += 1;
        }
        if self.difficulty.as_ref().map(|(at, _)| *at) != Some(key) {
            #[cfg(any(test, feature = "test-support"))]
            {
                self.difficulty_counts.1 += 1;
            }
            let difficulty = roster_difficulty(
                &draft.setup,
                &snapshot.catalogue,
                &snapshot.known_creature_names,
                &snapshot.scale,
            );
            self.difficulty = Some((key, Arc::new(difficulty)));
        }
        self.difficulty
            .as_ref()
            .map(|(_, difficulty)| Arc::clone(difficulty))
    }
    /// How often frames asked for the open draft's difficulty, and how often it was worked out.
    #[cfg(any(test, feature = "test-support"))]
    pub fn difficulty_counts(&self) -> (usize, usize) {
        self.difficulty_counts
    }
    /// The encounter editor is what the Encounters page shows.
    pub fn editor_open(&self) -> bool {
        self.group_draft.is_none() && self.draft.is_some()
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
    /// Row 123: which read of the campaign's encounters and groups this is. It changes when
    /// they are read again or a saved encounter changes them, so a copy of their names is made
    /// once for each.
    pub fn plan_revision(&self) -> Option<Uuid> {
        self.snapshot.as_ref().map(|snapshot| snapshot.revision)
    }
    /// Row 123: the names of the fights and the groups the plan can hold (`plan_fights`,
    /// `plan_groups`), with no look at the runs.
    pub fn plan_names(&self) -> (Vec<String>, Vec<String>) {
        self.snapshot
            .as_ref()
            .map_or_else(Default::default, |snapshot| {
                (
                    snapshot
                        .templates
                        .iter()
                        .map(|template| template.name.clone())
                        .collect(),
                    snapshot
                        .groups
                        .iter()
                        .filter(|group| group.encounter.is_none())
                        .map(|group| group.name.clone())
                        .collect(),
                )
            })
    }
    /// The revision of the fights `plan_fights` lists: it changes with each new read of the
    /// encounters and each save of one, never on a quiet frame.
    pub fn plan_fights_revision(&mut self) -> crate::design::memo::Revision {
        self.plan_fights_read
            .set(self.snapshot.as_ref().map(|snapshot| snapshot.revision));
        self.plan_fights_read.revision()
    }

    /// The Session plan's fights and groups (`plan_fights`, `plan_groups`), listed again only when
    /// the snapshot changed (`plan_fights_revision`; both come from it) or the saved runs did (a
    /// fight lists its unfinished run: `saved_runs_revision`), never on a quiet frame.
    pub fn plan_kept(&mut self) -> PlanKept {
        let revision = (self.plan_fights_revision(), self.saved_runs_revision());
        if self.plan_kept.get(revision).is_none() {
            self.plan_kept_builds += 1;
            let built = PlanKept {
                fights: self.plan_fights().into(),
                groups: self.plan_groups().into(),
            };
            let _ = self.plan_kept.get_or(revision, || built);
        }
        self.plan_kept
            .get(revision)
            .expect("the plan lists were just kept")
            .clone()
    }

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
                    id: template.id,
                    name: template.name.clone(),
                    path: template.path.clone(),
                    summary: template
                        .summary
                        .as_ref()
                        .map(|roster| roster.parts.clone())
                        .unwrap_or_default(),
                    revision: template.summary.as_ref().map(|roster| roster.revision),
                    difficulty: template.difficulty.clone(),
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
                self.history_reveal = true;
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
        // An edit that waits for its save over a file that changed on disk: the rule cannot say
        // which fight was played last, as `Run::unend` finds (review of PR 666).
        let unsettled = timeline.unsaved() && timeline.moved_under_edit.is_some();
        let key = (timeline.changes(), ended, read, unsettled);
        if self.continue_key == Some(key) {
            // A restored or deleted encounter changes no key: the answer checks it each frame.
            if let Some(run) = self.run.as_mut() {
                run.answer_still_stands();
            }
            return;
        }
        let rule = |id| {
            if unsettled {
                return Some(crate::encounter_run::Run::UNSETTLED);
            }
            crate::encounter_run::continue_blocked(
                &timeline.events,
                timeline.unreadable.is_none(),
                id,
            )
        };
        if let Some(run) = self.run.as_mut().filter(|run| run.ended()) {
            run.continue_blocked = rule(run.id());
            run.answer_still_stands();
        }
        self.history_blocked = read.and_then(rule);
        self.continue_key = Some(key);
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
    /// B4, E1: an unsaved draft (encounter, group or creature) holds the open drafts: a plan or
    /// run route that would replace one is refused with that draft's reason, and every draft is
    /// kept. The reason when refused.
    fn replacement_held(&mut self) -> Option<&'static str> {
        let reason = design::first_unsaved(self)?.reason();
        self.error = Some(reason.to_owned());
        Some(reason)
    }
    /// Assemble a group where the plan is, with the session's note still beside it. Refused,
    /// with every draft kept, while one is unsaved.
    pub fn new_group_in_plan(&mut self) -> PlanOpen {
        if let Some(reason) = self.replacement_held() {
            return PlanOpen::Refused(reason);
        }
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
        PlanOpen::Opened
    }
    /// Open a group in the plan. `Failed` when it cannot be read (Encounters says why);
    /// `Refused` while a draft is unsaved, with that draft's reason, every draft kept. The caller
    /// gives the group its host only on `Opened`.
    pub fn open_group_in_plan(&mut self, path: &str, active: &Campaign) -> PlanOpen {
        if let Some(reason) = self.replacement_held() {
            return PlanOpen::Refused(reason);
        }
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
                PlanOpen::Opened
            }
            Err(error) => {
                self.error = Some(error);
                PlanOpen::Failed
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
    /// The discard the plan shows, as the line of this page's kept store: the store's own
    /// words, Undo and ticket, so the Session plan's footer orders it by the order the store
    /// issued at the discard and its press reaches only that entry
    /// (`undo_group_discard_in_plan`). Its id is `()`: the plan maps it to its own press.
    pub fn group_discard_line<'f>(
        &self,
        frame: &'f design::outcome::PaneFrame,
    ) -> Option<design::outcome::KeptLine<'f, ()>> {
        if !self.group_discarded_in_plan() {
            return None;
        }
        let root = self.active.as_ref()?.path.clone();
        self.kept
            .get(&KeptOwner {
                root,
                kind: KeptResult::GroupDiscarded,
            })
            .map(|shown| {
                shown.outcome(frame, |_| {
                    let draft = self.discarded_group.as_ref();
                    discard_outcome(
                        draft.map_or_else(String::new, GroupDraft::name),
                        draft.is_some_and(GroupDraft::file_gone),
                        (),
                    )
                })
            })
    }
    /// The plan footer's `Undo discard` of the line `group_discard_line` made, by its ticket: a
    /// discard replaced or ended since is gone, and nothing changes.
    pub fn undo_group_discard_in_plan(&mut self, ticket: design::results::Ticket) {
        let mut kept = std::mem::take(&mut self.kept);
        let _ = kept.undo_by(ticket, |mark| {
            if mark.0.kind == KeptResult::GroupDiscarded && self.group_discarded_in_plan() {
                self.group_draft = self.discarded_group.take();
                self.message = None;
                self.error = None;
                Ok(())
            } else {
                Err(())
            }
        });
        self.kept = kept;
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
    /// B4c: whether the plan's refusal for `group` still stands: a draft is unsaved and the
    /// campaign still has the group. Read only while a refusal is stored; the drafts are
    /// compared again only when the campaign was read again or a frame had an editing input
    /// (or follows one: a click's change lands late in its frame).
    pub fn plan_refusal_stands(&mut self, ctx: &egui::Context, group: Uuid) -> bool {
        let edited = edit_input(ctx);
        let edited_before = std::mem::replace(&mut self.plan_edited_last_frame, edited);
        if edited || edited_before {
            self.plan_epoch += 1;
        }
        let key = (
            self.snapshot.as_ref().map(|snapshot| snapshot.revision),
            self.plan_epoch,
            group,
        );
        if let Some((cached, stands)) = self.plan_refusal_check
            && cached == key
        {
            return stands;
        }
        let stands = self.plan_group_blocked().is_some()
            && self
                .snapshot
                .as_ref()
                .is_some_and(|snapshot| snapshot.groups.iter().any(|g| g.id == group));
        self.plan_refusal_check = Some((key, stands));
        stands
    }
    /// Test only: the campaign read's revision, which keys the plan refusal's check.
    #[cfg(any(test, feature = "test-support"))]
    pub fn snapshot_revision(&self) -> Option<Uuid> {
        self.snapshot.as_ref().map(|snapshot| snapshot.revision)
    }
    /// Test only: whether the campaign read offers a creature of this name to the encounter
    /// search (the creatures the campaign's package choices use).
    #[cfg(any(test, feature = "test-support"))]
    pub fn searchable_creature(&self, name: &str) -> bool {
        self.snapshot
            .as_ref()
            .is_some_and(|snapshot| snapshot.creatures.iter().any(|c| c.name == name))
    }
    /// No plan refusal is stored: its check is forgotten, so a new one compares afresh.
    pub fn forget_plan_refusal(&mut self) {
        self.plan_refusal_check = None;
    }
    /// The stable id of the plan's group at `path`, if the campaign has it.
    pub fn plan_group_id(&self, path: &str) -> Option<Uuid> {
        self.snapshot
            .as_ref()?
            .groups
            .iter()
            .find(|group| group.path == path)
            .map(|group| group.id)
    }
    /// Why a group cannot be assembled from the plan now.
    /// The unsaved draft that refuses it says which it is, from the back lines' recipe (R38):
    /// the encounter, its group or a creature, each with its own reason.
    pub fn plan_group_blocked(&self) -> Option<&'static str> {
        #[cfg(any(test, feature = "test-support"))]
        PLAN_BLOCK_CHECKS.with(|checks| checks.set(checks.get() + 1));
        design::first_unsaved(self)
            .map(design::Unsaved::reason)
            .or(self
                .snapshot
                .is_none()
                .then_some("Still reading this campaign."))
    }
    /// A new fight from the plan. Refused, with every draft kept, while one is unsaved.
    pub fn new_from_plan(&mut self, session: Option<Uuid>) -> PlanOpen {
        if let Some(reason) = self.replacement_held() {
            return PlanOpen::Refused(reason);
        }
        self.start_new_draft();
        self.from_session = session;
        self.plan_on_save = true;
        PlanOpen::Opened
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
        self.from_session = None;
        self.plan_on_save = false;
        self.creature = None;
    }

    /// A planned fight's `Edit encounter` (W1-D1): that encounter, found by its ID, opens in the
    /// editor read from its file at the click, and the editor's way back is the session. The plan
    /// blocks the act while the editor holds unsaved words; a group left open with nothing
    /// unsaved gives way, so the page shows this encounter. When the encounter is gone, or its
    /// file is gone or now holds another encounter, nothing opens and the words say why, for the
    /// plan. `session` is where the editor's way back leads.
    pub fn edit_from_plan(
        &mut self,
        id: Uuid,
        session: Option<Uuid>,
        active: &Campaign,
    ) -> Result<(), String> {
        if let Some(reason) = self.plan_group_blocked() {
            return Err(reason.to_owned());
        }
        let gone = || "This encounter is no longer in the campaign. Open it from Encounters.";
        let path = self
            .snapshot
            .as_ref()
            .and_then(|snapshot| {
                snapshot
                    .templates
                    .iter()
                    .find(|template| template.id == id)
                    .map(|template| template.path.clone())
            })
            .ok_or_else(|| gone().to_owned())?;
        // A file moved or deleted since the campaign was last read.
        if !active.path.join(&path).is_file() {
            return Err(gone().to_owned());
        }
        let draft = Draft::open(&active.path, &path)?;
        if draft.id != id {
            return Err(gone().to_owned());
        }
        self.clear_notice();
        self.search.clear();
        self.group_draft = None;
        self.group_in_plan = false;
        self.draft = Some(draft);
        self.from_session = session;
        self.plan_on_save = false;
        self.error = None;
        self.message = None;
        Ok(())
    }

    /// The DM left the editor by another way than its own way back (the rail, another page):
    /// `Back to session` would no longer lead to the session the DM came from.
    pub fn end_session_route(&mut self) {
        self.from_session = None;
    }

    /// The file the editor holds, for the visual tests.
    #[cfg(any(test, feature = "test-support"))]
    pub fn draft_path(&self) -> Option<&str> {
        self.draft.as_ref().map(|draft| draft.path.as_str())
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
    /// `Start fight` in a reader or in a scene's menu: what the editor's `Start fight` does for a
    /// new encounter, in one act. It saves `Fight with <name>` with the party and the creature,
    /// then starts its run, which saves at once. When an encounter of that name, or of that name
    /// with a number, was never run and holds what this would save, that one starts, so a wrong
    /// click leaves no second encounter. An NPC with no stat block opens the run's form for a
    /// creature made up at the table, with its name filled in.
    pub fn start_fight_with(
        &mut self,
        fight: &crate::world::StartFight,
        active: &Campaign,
        modules: &[Module],
    ) {
        self.clear_notice();
        if let Some(reason) = self.plan_blocked() {
            self.error = Some(reason.to_owned());
            return;
        }
        // The campaign as it is at the click: a party sheet or an encounter written a moment
        // ago, before the next content poll, is part of what the fight saves or reuses.
        self.error = None;
        self.reload_snapshot(active, modules);
        if self.error.is_some() {
            return;
        }
        let Some(snapshot) = &self.snapshot else {
            return;
        };
        let mut draft = Draft::new();
        for creature in snapshot.creatures.iter().filter(|creature| creature.party) {
            draft.add(creature.id, true);
        }
        if let Some(id) = fight.creature {
            draft.add(id, false);
            if fight.npc {
                draft
                    .setup
                    .roster
                    .last_mut()
                    .expect("added slot")
                    .overrides
                    .insert("name".into(), fight.name.clone().into());
            }
        }
        // What a click saves: every slot as written, apart from its key and the form of its
        // creature's reference, and the rest of the setup.
        let roster = |setup: &Setup| {
            let mut slots: Vec<_> = setup
                .roster
                .iter()
                .map(|slot| {
                    let text = |value: &Value| serde_yaml_ng::to_string(value).unwrap_or_default();
                    (
                        slot.creature
                            .as_ref()
                            .and_then(|reference| snapshot.catalogue.resolve_ref(reference).ok()),
                        slot.party_slot.clone(),
                        slot.side.clone(),
                        text(&slot.controller),
                        text(&Value::Mapping(slot.initial.clone())),
                        text(&Value::Mapping(slot.overrides.clone())),
                        text(&Value::Mapping(slot.extra.clone())),
                    )
                })
                .collect();
            slots.sort();
            let mut rest = setup.clone();
            rest.roster.clear();
            (slots, serde_yaml_ng::to_string(&rest).unwrap_or_default())
        };
        let wanted = roster(&draft.setup);
        let base = format!("Fight with {}", fight.name);
        let mut existing = None;
        for number in 1.. {
            let name = if number == 1 {
                base.clone()
            } else {
                format!("{base} {number}")
            };
            let Some(template) = snapshot
                .templates
                .iter()
                .find(|template| template.name.eq_ignore_ascii_case(&name))
            else {
                draft.name = name;
                break;
            };
            let run = snapshot.runs.iter().any(|run| {
                run.source == Some(template.id)
                    || (run.source.is_none() && run.encounter.eq_ignore_ascii_case(&template.name))
            });
            if !run
                && Draft::open(&active.path, &template.path)
                    .is_ok_and(|other| roster(&other.setup) == wanted)
            {
                existing = Some(template.path.clone());
                break;
            }
        }
        if let Some(path) = existing {
            self.run_path(&path, active, modules);
        } else {
            if let Err(error) = draft.save(active, modules) {
                self.error = Some(error);
                return;
            }
            self.draft = Some(draft);
            self.open_run(active, modules);
            self.draft = None;
            self.reload_snapshot(active, modules);
        }
        if fight.creature.is_none()
            && let Some(run) = self.run.as_mut()
        {
            run.made_up(&fight.name);
        }
    }
    /// Add a freshly copied creature to the open draft.
    pub fn add_to_draft(&mut self, id: Uuid) {
        if let Some(draft) = self.draft.as_mut() {
            draft.add_chosen(id, false);
        } else if let Some(draft) = self.group_draft.as_mut() {
            draft.add(id);
        }
    }
    /// What the rail calls the open draft.
    pub fn draft_word(&self) -> &'static str {
        if self.group_draft.is_some() {
            GROUP_DRAFT
        } else {
            ENCOUNTER_DRAFT
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
        // A reload is a disk event too: a separated copy changed by another program ends.
        self.refresh_separations();
        self.creature = None;
        self.history_read = None;

        // A discarded draft belongs to its campaign. Undo must not save it into another.
        if self.active.as_ref().is_none_or(|old| old.id != campaign.id) {
            self.discarded = None;
            self.discarded_group = None;
            self.discarded_options = None;
            self.options_saved = None;
        }
        // Its path can be out of date after this, so undo could put it where nothing lists it.
        self.deleted_run = None;
        self.removed_saved_group = None;
        self.removed_encounter = None;
        self.active = Some(campaign.clone());
        self.draft = None;
        self.group_draft = None;
        self.group_in_plan = false;
        self.snapshot = None;
        self.forget_ready_lines();
        self.error = None;
        self.refreshing = false;
        self.start_load(ctx, campaign, modules);
    }
    /// G118: the Ready rows' kept lines end with the snapshot they were fitted for: at a
    /// reload or a campaign switch, and when a load fails.
    fn forget_ready_lines(&mut self) {
        self.ready_lines = crate::difficulty::Kept::default();
        self.ready_lines_seen = None;
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
            // A disk change: a separated copy changed by another program ends its result.
            self.refresh_separations();
            self.refreshing = true;
            self.start_load(ctx, active, campaigns.modules().to_vec());
        }
    }
    /// Keep the snapshot current. The shell calls this every frame.
    pub fn poll(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        self.poll_reads(ctx, campaigns);
        self.note_run_facts();
    }

    /// The revision of the saved-run facts (`RunFacts`): it changes when the run list is read
    /// again (a deleted or restored run, a reload) or the open run starts, ends or leaves Setup,
    /// as of this frame's `poll`. A kept result whose words name the fights that keep their
    /// saved HP keys its words by it (`sessions::page::Plan::saved_runs_revision`, the Timeline's
    /// `set_saved_runs`).
    pub fn saved_runs_revision(&self) -> crate::design::memo::Revision {
        self.run_facts.0.revision()
    }

    /// Note the saved-run facts: a revision and the open run's id and running flag, constant
    /// work with no string copied (`Run::keeps_saved_hp`), never the run list.
    fn note_run_facts(&mut self) {
        let facts = RunFacts {
            listed: self
                .snapshot
                .as_ref()
                .map(|snapshot| snapshot.runs_read.revision()),
            open: self
                .run
                .as_ref()
                .map(crate::encounter_run::Run::keeps_saved_hp),
        };
        self.run_facts.0.set(facts);
    }

    fn poll_reads(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        #[cfg(any(test, feature = "test-support"))]
        if self.hold_reads {
            return;
        }
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
                        Err(e) => {
                            self.forget_ready_lines();
                            self.error = Some(e);
                        }
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.forget_ready_lines();
                    self.error = Some("Loading stopped. Reload to retry.".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        // Only a dropped key can leave a dead result. Ending a result can drop the last copy of
        // another draft (a removed group's Undo holds that group), so it looks again.
        while self.dead_keys_seen != results_key::dead_keys() {
            self.dead_keys_seen = results_key::dead_keys();
            self.end_dead_results();
        }
    }
    /// The member and editor results whose draft no longer exists end. A result watches its
    /// draft's key (`results_key`), which every copy of the draft shares: the open draft, a
    /// discard that Undo discard restores, a group a removed group's Undo holds. A draft that is
    /// dropped (closed, discarded for good, read again, another campaign's) drops its key with
    /// it, so nothing marks that end. It runs only after a key was dropped, and costs the
    /// number of stored results. The editor results go first: a removed group's Undo holds that group's key.
    fn end_dead_results(&mut self) {
        self.editor_results
            .end_unreachable(|mark| mark.draft.alive());
        self.member_results
            .end_unreachable(|mark| mark.draft.alive());
    }
    /// Ctrl+S on this page saves the open creature while it has changes, else the open
    /// encounter or group. The key is taken here, before the page draws; the save is done
    /// after the fields it writes have taken this frame's input (`save_key`): the creature's
    /// in its pane's `then`, the encounter's in the editor's `then`, a group's after its
    /// editor's fields. A digit typed in the frame of Ctrl+S is saved with it (review of PR 499).
    /// `modal`: the app's one test (`crate::art::modal_open`: its dialogs, the full-window image, any
    /// egui modal). While a modal is open the key is left alone, so nothing saves beneath it
    /// (reviews of PR 499 and PR 525).
    pub fn save_shortcut(&mut self, ctx: &egui::Context, modal: bool, campaigns: &Campaigns) {
        // Taken for the open editor whether or not it has changes yet: the first edit can come
        // in the same frame. Its owner saves only if it has changes after its fields ran.
        if !modal
            && (self.creature.is_some() || self.draft.is_some() || self.group_draft.is_some())
            && campaigns.active_campaign().is_some()
        {
            self.save_key |= take_save_key(ctx);
        }
    }
    /// The page's Ctrl+S that no pane took after its fields (the creature's pane was not drawn
    /// this frame, a narrow window's other tab): saved now, as nothing of it was drawn.
    fn save_by_key(&mut self, ctx: &egui::Context, active: &Campaign, modules: &[Module]) {
        if !std::mem::take(&mut self.save_key) {
            return;
        }
        if self.creature.as_ref().is_some_and(CreatureEditor::dirty) {
            self.save_creature(active, modules);
        } else if self.group_draft.is_some() {
            self.save_group_draft(ctx, active, modules);
        } else if self.draft_dirty() && !self.save_refused {
            // FC1: blocked as Save is, while the editor says Invalid.
            self.save_draft(ctx, active, modules);
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
                Self::record_kept(&mut self.kept, self.active.as_ref(), KeptResult::Saved);
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
                    // A saved encounter can have a new name: what is read from the list is new.
                    s.revision = Uuid::new_v4();
                    let difficulty = roster_difficulty(
                        &draft.setup,
                        &s.catalogue,
                        &s.known_creature_names,
                        &s.scale,
                    );
                    s.templates.push(Template {
                        path: draft.path.clone(),
                        id: entry.id,
                        name: draft.name.clone(),
                        summary: Some(history::Roster::new(
                            roster_summary(&draft.setup, &s.catalogue, &s.known_creature_names),
                            &difficulty,
                        )),
                        difficulty,
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
                // Every new snapshot, however it was read, says the same about the run folder.
                self.note_run_folder(&mut snapshot);
                self.snapshot = Some(snapshot);
            }
            Err(e) => {
                self.forget_ready_lines();
                self.error = Some(e);
            }
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
            Ok(()) => self.creature_saved(active, modules),
            Err(e) => editor.error = Some(e),
        }
    }
    /// The open creature was saved: it closes and the campaign's creatures are read again.
    fn creature_saved(&mut self, active: &Campaign, modules: &[Module]) {
        self.creature = None;
        self.clear_creature_block();
        self.reload_snapshot(active, modules);
        self.saved = true;
        self.focus_search = true;
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
    /// The Run page's egui view (the Run area's strip removes it). Returns true when the run
    /// has just closed. Its `run_reads` come first.
    pub fn show_run(&mut self, ui: &mut egui::Ui, active: &Campaign) -> bool {
        let Some(run) = &mut self.run else {
            return false;
        };
        if !run.show(ui) {
            return false;
        }
        self.close_shown_run(active);
        true
    }
    /// What the run reads from this page before the Run page draws (egui and GPUI): the
    /// campaign's creature names, groups, scale and creatures, and tonight's planned groups.
    pub fn run_reads(&mut self) {
        let Some(run) = &mut self.run else {
            return;
        };
        if let Some(snapshot) = &self.snapshot {
            run.set_roster_names(snapshot.revision, &snapshot.known_creature_names);
            run.set_reads(snapshot.revision, &snapshot.groups, &snapshot.scale);
            run.creatures = Some(snapshot.run_creatures.clone());
        }
        run.planned.clone_from(&self.planned_groups);
    }
    /// The DM closed the run on the Run page (egui and GPUI): it goes, and the list reads its
    /// runs again.
    pub fn close_shown_run(&mut self, active: &Campaign) {
        let Some(run) = &self.run else {
            return;
        };
        // A setup nobody touched holds nothing. It would only be a line to remove later.
        if run.untouched_setup()
            && let Some(path) = run.path()
        {
            let _ = fs::remove_file(path);
        }
        self.run = None;
        self.read_runs_again(active);
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
            snapshot.runs_unread = Some(error);
        }
        if !snapshot.runs.is_empty() || snapshot.runs_unread.is_some() {
            self.runs_known = campaign;
        }
    }
    /// The synchronous refresh that Make it yours, a creature save and a group removal run,
    /// for the scenarios.
    #[cfg(any(test, feature = "test-support"))]
    pub fn refresh_now(&mut self, active: &Campaign, modules: &[Module]) {
        self.reload_snapshot(active, modules);
    }
    pub fn read_runs_again(&mut self, active: &Campaign) {
        let Some(mut s) = self.snapshot.take() else {
            return;
        };
        let directory = s.saved_options.directory(active);
        let read = directory
            .map_err(crate::encounter_run::RunFolderFault::Setting)
            .and_then(|dir| crate::encounter_run::Run::list(&dir, active.id));
        let missing = matches!(read, Err(crate::encounter_run::RunFolderFault::Missing(_)));
        s.take_run_list(read, missing);
        self.note_run_folder(&mut s);
        self.snapshot = Some(s);
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
            self.from_session = None;
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
        self.stamp_kept(KeptResult::Discarded);
    }
    /// The undo of `discard_draft`.
    fn undo_discard(&mut self) {
        self.draft = self.discarded.take();
        self.message = None;
        self.error = None;
    }
    /// A discard's result ends when the draft of its kind has changes again: a creature's
    /// changes and a group's are not this encounter's, and leave its undo alone.
    fn expire_discards(&mut self) {
        if self.draft.as_ref().is_some_and(Draft::dirty) {
            self.discarded = None;
        }
        if self.group_draft.as_ref().is_some_and(GroupDraft::dirty) {
            self.discarded_group = None;
        }
    }

    /// A kept result of `kind` was just made: it is now the newest.
    fn stamp_kept(&mut self, kind: KeptResult) {
        Self::record_kept(&mut self.kept, self.active.as_ref(), kind);
    }

    /// `stamp_kept` on the store alone, for an action that holds another part of the page.
    fn record_kept(
        kept: &mut design::Results<KeptOwner, KeptMark>,
        active: Option<&Campaign>,
        kind: KeptResult,
    ) {
        if let Some(root) = active.map(|active| active.path.clone()) {
            kept.record(KeptMark(KeptOwner { root, kind }));
        }
    }

    /// The kept results that show now, newest first, each with its owner in the page's store.
    /// `page` is the Encounters list, which shows every kind; an editor shows only the discard
    /// of its own kind. A result whose data was dropped (a new draft, a save, another removal)
    /// leaves the store here: its owner changed.
    fn kept_results(&mut self, page: bool) -> Vec<KeptResult> {
        self.expire_discards();
        // Every kind with whether its data is still held, and whether this view shows it.
        let alive = [
            (
                KeptResult::Discarded,
                self.discarded.is_some(),
                self.encounter_undo_ready(),
            ),
            (
                KeptResult::GroupDiscarded,
                self.discarded_group.is_some(),
                self.group_undo_ready(),
            ),
            (KeptResult::RunDeleted, self.deleted_run.is_some(), page),
            (
                KeptResult::GroupRemoved,
                self.removed_saved_group.is_some(),
                page,
            ),
            (
                KeptResult::EncounterRemoved,
                self.removed_encounter.is_some(),
                page,
            ),
            (KeptResult::Saved, self.message.is_some(), page),
            // Only Rules and runs shows it (`run_settings`).
            (
                KeptResult::OptionsDiscarded,
                self.discarded_options.is_some(),
                false,
            ),
        ];
        let Some(root) = self.active.as_ref().map(|active| active.path.clone()) else {
            return Vec::new();
        };
        for (kind, held, _) in alive {
            if !held {
                self.kept.owner_changed(&KeptOwner {
                    root: root.clone(),
                    kind,
                });
            }
        }
        self.kept
            .newest_first(root.as_path())
            .map(|shown| shown.value().0.kind)
            .filter(|kind| alive.iter().any(|(k, _, here)| k == kind && *here))
            .collect()
    }

    /// A kept result as a typed footer line, from the data its kind holds: a discard is a
    /// notice with its Undo, a removal and a deleted run are results, a deleted run with changes
    /// that never reached D&D Beyond is a warning that keeps its Undo, a save has no Undo.
    #[allow(
        clippy::disallowed_methods,
        reason = "the run’s and the save’s words are kept as words"
    )]
    fn kept_outcome(&self, kind: KeptResult) -> design::outcome::KeptOutcome<FooterPress> {
        use design::outcome::{KeptOutcome, Said, Sentence, UndoStep, Verb};
        let discard = |name, gone| discard_outcome(name, gone, FooterPress::Undo);
        match kind {
            KeptResult::Discarded => self.discarded.as_ref().map_or_else(
                || discard(String::new(), false),
                |draft| discard(draft.name.clone(), draft.recovery.file_gone()),
            ),
            KeptResult::GroupDiscarded => self.discarded_group.as_ref().map_or_else(
                || discard(String::new(), false),
                |draft| discard(draft.name(), draft.file_gone()),
            ),
            KeptResult::RunDeleted => {
                let undo = UndoStep::of(Verb::Delete, "run").pressed_as(FooterPress::Undo);
                match &self.deleted_run {
                    Some(deleted) if !deleted.unsent.is_empty() => KeptOutcome::Fault(
                        design::outcome::FaultMessage::new(&EncounterFault::RunDeletedUnsent(
                            deleted.message(),
                        )),
                        undo,
                    ),
                    deleted => Said::new(
                        deleted
                            .as_ref()
                            .and_then(|deleted| Sentence::outside(deleted.message()))
                            .unwrap_or(crate::sentence!("Run deleted.")),
                    )
                    .undo(undo)
                    .into(),
                }
            }
            KeptResult::GroupRemoved => Said::new(crate::sentence!(
                "Removed group {}.",
                self.removed_saved_group
                    .as_ref()
                    .map_or("", |removed| removed.name.as_str())
            ))
            .undo(UndoStep::of(Verb::Remove, "group").pressed_as(FooterPress::Undo))
            .into(),
            // V6: a finished change has the success tone, as a removed group's result has.
            KeptResult::EncounterRemoved => Said::new(crate::sentence!(
                "Removed encounter {}.",
                self.removed_encounter
                    .as_ref()
                    .map_or("", |removed| removed.name.as_str())
            ))
            .undo(UndoStep::of(Verb::Remove, "encounter").pressed_as(FooterPress::Undo))
            .into(),
            KeptResult::Saved => Said::new(
                self.message
                    .as_deref()
                    .and_then(Sentence::outside)
                    .unwrap_or(crate::sentence!("Saved.")),
            )
            .into(),
            KeptResult::OptionsDiscarded => options_discarded(),
        }
    }

    /// Each kept result of `kinds` in the page's store, as a footer line for this frame.
    fn kept_lines<'f>(
        &self,
        frame: &'f design::outcome::PaneFrame,
        kinds: &[KeptResult],
    ) -> Vec<design::outcome::KeptLine<'f, FooterPress>> {
        let Some(root) = self.active.as_ref().map(|active| active.path.clone()) else {
            return Vec::new();
        };
        kinds
            .iter()
            .filter_map(|kind| {
                self.kept
                    .get(&KeptOwner {
                        root: root.clone(),
                        kind: *kind,
                    })
                    .map(|shown| shown.outcome(frame, |_| self.kept_outcome(*kind)))
            })
            .collect()
    }

    /// The list's footer lines, for both views: the kept results with their tickets, then why
    /// the last act failed, then an unreadable run folder with `Choose folder`.
    fn list_lines<'f>(
        &mut self,
        messages: &mut design::outcome::Messages<'f, FooterPress>,
        frame: &'f design::outcome::PaneFrame,
    ) {
        let kinds = self.kept_results(true);
        for line in self.kept_lines(frame, &kinds) {
            messages.kept(line);
        }
        if let Some(error) = &self.error {
            let fault = design::outcome::FaultMessage::new(&EncounterFault::Failed(error.clone()));
            messages.outcome(fault);
        }
        if let Some(folder) = self
            .snapshot
            .as_ref()
            .and_then(|snapshot| snapshot.runs_unread.clone())
        {
            messages.outcome(
                design::outcome::FaultMessage::new(&EncounterFault::RunFolder(folder)).choice(
                    design::outcome::Choice::new(
                        design::outcome::MessageChoice::Choice("Choose folder"),
                        FooterPress::ChooseFolder,
                    ),
                ),
            );
        }
    }

    /// The Undo of the kept result a footer press names by its ticket. A result that was
    /// replaced or ended since is `Gone`: nothing happens. A failed Undo keeps its result, with
    /// its order, and says why.
    fn undo_by_ticket(
        &mut self,
        ticket: design::results::Ticket,
        campaign: Option<(&Campaign, &[Module])>,
    ) {
        let mut kept = std::mem::take(&mut self.kept);
        let _ = kept.undo_by(ticket, |mark| {
            if self.undo_kept(mark.0.kind, campaign) {
                Ok(())
            } else {
                Err(())
            }
        });
        self.kept = kept;
    }

    /// Move the encounter draft's new results into the store, with their data (the order is
    /// issued now, in the frame of the act), and end those whose data went: a Save, or an edit
    /// of the replaced rows. A discarded draft's results stay in the store, hidden (its
    /// `results_key` is not the clean draft's), and show again with Undo discard (review of
    /// PR 339).
    fn record_editor_results(&mut self) {
        let Some(root) = self.active.as_ref().map(|active| active.path.clone()) else {
            return;
        };
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        let key = draft.results_key.id();
        let watch = draft.results_key.clone();
        let owner = |kind| EditorOwner {
            root: root.clone(),
            draft: key,
            kind,
        };
        // A Save ends the results from before it; a result made after it is new.
        if std::mem::take(&mut draft.results_ended) {
            for kind in [DraftResult::RemovedGroup, DraftResult::Replacement] {
                self.editor_results.owner_changed(&owner(kind));
            }
        }
        for undo in draft.take_new_results() {
            self.editor_results.record(EditorMark {
                owner: owner(undo.kind()),
                draft: watch.watch(),
                undo,
            });
        }
        // A replacement ends at an edit of its rows (made by the body or by another Undo).
        for kind in [DraftResult::RemovedGroup, DraftResult::Replacement] {
            let owner = owner(kind);
            if self
                .editor_results
                .get(&owner)
                .is_some_and(|shown| !shown.value().undo.holds(draft))
            {
                self.editor_results.owner_changed(&owner);
            }
        }
    }

    /// The encounter draft's results in the store, as footer lines for this frame.
    fn editor_result_lines<'f>(
        &self,
        frame: &'f design::outcome::PaneFrame,
    ) -> Vec<design::outcome::KeptLine<'f, FooterPress>> {
        let (Some(root), Some(draft)) = (self.active.as_ref(), self.draft.as_ref()) else {
            return Vec::new();
        };
        [DraftResult::RemovedGroup, DraftResult::Replacement]
            .into_iter()
            .filter_map(|kind| {
                self.editor_results
                    .get(&EditorOwner {
                        root: root.path.clone(),
                        draft: draft.results_key.id(),
                        kind,
                    })
                    .map(|shown| shown.outcome(frame, |mark| mark.undo.outcome()))
            })
            .collect()
    }

    /// The editor footer's lines, from the state as it is now (`editor_view` reads them after
    /// the panes' acts): the results it records now, then the reasons, then the file's flags.
    fn editor_lines<'f>(
        &mut self,
        messages: &mut design::outcome::Messages<'f, FooterPress>,
        frame: &'f design::outcome::PaneFrame,
        root: &std::path::Path,
        groups_reason: Option<(EncounterFault, String)>,
        unread: Option<UnreadRows>,
    ) {
        use design::outcome::FaultMessage;
        self.record_member_results();
        self.record_editor_results();
        for line in self.editor_result_lines(frame) {
            messages.kept(line);
        }
        let kinds = self.kept_results(false);
        for line in self.kept_lines(frame, &kinds) {
            messages.kept(line);
        }
        for line in self.separation_lines(frame, root) {
            messages.kept(line);
        }
        let members: Vec<MemberOwner> = self
            .draft
            .iter()
            .flat_map(|draft| draft.groups.iter())
            .map(|group| MemberOwner {
                root: root.to_owned(),
                draft: group.results_key(),
            })
            .collect();
        for owner in members {
            if let Some(shown) = self.member_results.get(&owner) {
                messages.kept(shown.outcome(frame, MemberMark::outcome));
            }
        }
        if let Some(error) = self.separation_failure() {
            messages.outcome(FaultMessage::new(&EncounterFault::SeparationFailed(error)));
        }
        // N4 (lead rule): the reason that disables Save now is always painted. A stored
        // failed-action message stays as its own line while it holds, unless it only repeats
        // the cause that the owned groups' next step answers.
        let (groups_reason, groups_cause) = groups_reason.unzip();
        if let Some(error) = self.error.clone()
            && groups_cause.as_deref() != Some(error.as_str())
        {
            // A deleted entry and an unreadable file are stated in the rows' tone too. A refused
            // Start says first that no fight started (SE3, AE4), while the check it answers holds.
            let start = self.start_refused.as_ref().filter(|(draft, answers, _)| {
                *answers == error
                    && self.shows_validation_error()
                    && self
                        .draft
                        .as_ref()
                        .is_some_and(|open| open.id == *draft && !open.dirty())
            });
            let fault = if let Some((_, _, refusal)) = start {
                EncounterFault::StartRefused(refusal.clone())
            } else if self.error_tone() == Tone::Warning {
                EncounterFault::Invalid(error)
            } else {
                EncounterFault::Failed(error)
            };
            messages.outcome(FaultMessage::new(&fault));
        }
        if let Some(reason) = groups_reason {
            // F2, R35: the owned groups' chosen reason, as its next step where a row or field
            // says the cause (an empty group, a blank name or side, a member's row), in its own
            // typed tone (R41).
            messages.outcome(FaultMessage::new(&reason));
        }
        // The encounter file's flags last, one fault each; the last one says what to do and
        // holds the file's line with its links.
        if let Some(unread) = unread {
            for fault in unread.faults() {
                messages.outcome(fault.file_in(root));
            }
        }
    }

    /// A press in the encounter editor's footer: each Undo reaches only the entry its line came
    /// from, by its ticket. A result that was replaced or ended since is `Gone`: nothing happens.
    fn editor_press(
        &mut self,
        ctx: &egui::Context,
        press: design::outcome::Press<FooterPress>,
        active: &Campaign,
        modules: &[Module],
    ) {
        let Some(ticket) = press.ticket else {
            return;
        };
        match press.id {
            FooterPress::Undo => self.undo_by_ticket(ticket, Some((active, modules))),
            FooterPress::UndoSeparation => self.undo_separation(ctx, ticket, active),
            FooterPress::UndoMember => self.undo_member(ticket),
            FooterPress::UndoEditor => self.undo_editor(ticket),
            // Rules and runs' own choices (`run_settings`).
            FooterPress::ChooseFolder | FooterPress::MoveRuns | FooterPress::SaveWithoutMoving => {}
        }
    }

    /// The Undo of the draft result a footer press names by its ticket: the draft gets its
    /// roster or its removed group back, and its other edits stay.
    fn undo_editor(&mut self, ticket: design::results::Ticket) {
        let mut results = std::mem::take(&mut self.editor_results);
        let _ = results.undo_by(ticket, |mark| match self.draft.as_mut() {
            // The check runs at the press, on the draft as the body left it: a roster change
            // made in this pass (a move, a removal, an added row) refuses the restore, so the
            // newer change stays.
            Some(draft) if draft.results_key.id() == mark.owner.draft && mark.undo.holds(draft) => {
                draft.undo_result(&mark.undo);
                Ok(())
            }
            _ => Err(()),
        });
        self.editor_results = results;
        // An open form is keyed by place; the place may now hold another row or member.
        self.expanded = None;
    }

    /// Move each group draft's new member result into the store, with its data (its order is
    /// issued now, in the frame of the act), and end a replacement whose member was removed or
    /// no longer holds its creature (a move keeps it: it names its member by identity). A discarded draft's result stays in the store, hidden (its
    /// `results_key` is not the clean draft's), and shows again with Undo discard (review of
    /// PR 339).
    fn record_member_results(&mut self) {
        let Some(root) = self.active.as_ref().map(|active| active.path.clone()) else {
            return;
        };
        let drafts = self.group_draft.iter_mut().chain(
            self.draft
                .iter_mut()
                .flat_map(|draft| draft.groups.iter_mut()),
        );
        for draft in drafts {
            let owner = MemberOwner {
                root: root.clone(),
                draft: draft.results_key(),
            };
            if let Some(undo) = draft.take_member_new() {
                self.member_results.record(MemberMark {
                    owner: owner.clone(),
                    draft: draft.results_watch(),
                    undo,
                });
            }
            if self
                .member_results
                .get(&owner)
                .is_some_and(|shown| !shown.value().undo.holds(draft))
            {
                self.member_results.owner_changed(&owner);
            }
        }
    }

    /// Each separation of this campaign as a footer line, newest first, with its Undo.
    fn separation_lines<'f>(
        &self,
        frame: &'f design::outcome::PaneFrame,
        root: &std::path::Path,
    ) -> Vec<design::outcome::KeptLine<'f, FooterPress>> {
        self.separations
            .newest_first(root)
            .map(|shown| {
                shown.outcome(frame, |separation| {
                    design::outcome::Said::new(outcome_words(separation.change.0.message()))
                        .undo(
                            separation
                                .undo_step()
                                .pressed_as(FooterPress::UndoSeparation),
                        )
                        .into()
                })
            })
            .collect()
    }

    /// The Undo of the member result a footer press names by its ticket: the group draft that
    /// owns it gets its member back. `Gone` does nothing.
    fn undo_member(&mut self, ticket: design::results::Ticket) {
        let mut results = std::mem::take(&mut self.member_results);
        let _ = results.undo_by(ticket, |mark| {
            let draft = self
                .group_draft
                .iter_mut()
                .chain(
                    self.draft
                        .iter_mut()
                        .flat_map(|draft| draft.groups.iter_mut()),
                )
                .find(|draft| draft.results_key() == mark.owner.draft);
            draft.ok_or(())?.undo_member_result(&mark.undo)
        });
        self.member_results = results;
        self.expanded = None;
    }

    /// The disk end event of the separations: one whose note has another identity now ends.
    /// It runs when the page hears of a disk change (`refresh_content`, `reload`), never per
    /// frame: each check reads a note.
    fn refresh_separations(&mut self) {
        self.separations.disk_changed(Separation::holds);
    }

    /// A separation of this copy, or its Undo, worked: that copy's failure ends (another copy's
    /// stays).
    fn end_separation_failure(&mut self, root: &std::path::Path, path: &str) {
        if self
            .separation_error
            .as_ref()
            .is_some_and(|failure| failure.root == root && failure.path == path)
        {
            self.separation_error = None;
        }
    }

    /// Why the last separation, or its Undo, failed, when it was in the active campaign.
    fn separation_failure(&self) -> Option<String> {
        let root = &self.active.as_ref()?.path;
        self.separation_error
            .as_ref()
            .filter(|failure| failure.root == *root)
            .map(|failure| failure.words.clone())
    }

    /// The Undo of the separation a footer press names by its ticket. `Gone` does nothing; a
    /// failed Undo keeps the separation and says why.
    fn undo_separation(
        &mut self,
        ctx: &egui::Context,
        ticket: design::results::Ticket,
        active: &Campaign,
    ) {
        match self.separations.undo_by(ticket, |separation| {
            separation
                .change
                .0
                .undo(active)
                .map(|()| separation.path.clone())
                .map_err(|error| (separation.path.clone(), error))
        }) {
            Ok((_, path)) => {
                self.end_separation_failure(&active.path, &path);
                note_identity_changed(ctx);
            }
            Err(design::results::ByTicket::Failed((path, error))) => {
                self.separation_error = Some(SeparationFailure {
                    root: active.path.clone(),
                    path,
                    words: error,
                });
            }
            Err(design::results::ByTicket::Gone) => {}
        }
    }

    /// Undo one kept result. A result that writes files back needs the campaign.
    /// Returns true when it worked; a failed Undo says why and keeps its data.
    fn undo_kept(&mut self, kind: KeptResult, campaign: Option<(&Campaign, &[Module])>) -> bool {
        match kind {
            KeptResult::Saved => false,
            KeptResult::OptionsDiscarded => {
                match (self.discarded_options.take(), self.snapshot.as_mut()) {
                    (Some(options), Some(snapshot)) => {
                        snapshot.options = options;
                        true
                    }
                    (options, _) => {
                        self.discarded_options = options;
                        false
                    }
                }
            }
            KeptResult::Discarded => {
                self.undo_discard();
                true
            }
            KeptResult::GroupDiscarded => {
                self.group_draft = self.discarded_group.take();
                self.message = None;
                self.error = None;
                true
            }
            KeptResult::RunDeleted => {
                let Some(deleted) = &self.deleted_run else {
                    return false;
                };
                match deleted.restore() {
                    Ok(()) => {
                        self.deleted_run = None;
                        self.error = None;
                        if let Some((active, _)) = campaign {
                            self.read_runs_again(active);
                        }
                        true
                    }
                    Err(error) => {
                        self.error = Some(format!("The run did not come back: {error}"));
                        false
                    }
                }
            }
            KeptResult::GroupRemoved => {
                let Some(removed) = &self.removed_saved_group else {
                    return false;
                };
                match removed.restore() {
                    Ok(()) => {
                        self.removed_saved_group = None;
                        if let Some((active, modules)) = campaign {
                            self.reload_snapshot(active, modules);
                        }
                        self.saved = true;
                        true
                    }
                    Err(error) => {
                        self.error = Some(format!("The group did not come back: {error}"));
                        false
                    }
                }
            }
            KeptResult::EncounterRemoved => {
                let Some(removed) = &self.removed_encounter else {
                    return false;
                };
                match removed.restore() {
                    Ok(()) => {
                        self.removed_encounter = None;
                        if let Some((active, modules)) = campaign {
                            self.reload_snapshot(active, modules);
                        }
                        self.saved = true;
                        true
                    }
                    Err(error) => {
                        self.error = Some(format!("The encounter did not come back: {error}"));
                        false
                    }
                }
            }
        }
    }

    fn encounter_undo_ready(&self) -> bool {
        self.discarded.is_some() && self.group_draft.is_none()
    }

    fn group_undo_ready(&self) -> bool {
        self.discarded_group.is_some() && self.draft.is_none()
    }

    pub fn clear_notice(&mut self) {}

    /// A row action of the list (`ListAction`), for both views: the egui list and the GPUI page.
    pub fn list_act(&mut self, action: ListAction, active: &Campaign, campaigns: &Campaigns) {
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
            ListAction::RemoveGroup(path) => {
                match groups::RemovedNote::remove(active, &path, "encounter_group") {
                    Ok(removed) => {
                        self.removed_saved_group = Some(removed);
                        self.removed_encounter = None;
                        self.stamp_kept(KeptResult::GroupRemoved);
                        self.reload_snapshot(active, campaigns.modules());
                        self.saved = true;
                    }
                    Err(error) => self.error = Some(error),
                }
            }
            ListAction::RemoveEncounter(path) => {
                match groups::RemovedNote::remove(active, &path, "encounter") {
                    Ok(removed) => {
                        self.removed_encounter = Some(removed);
                        self.removed_saved_group = None;
                        self.stamp_kept(KeptResult::EncounterRemoved);
                        self.reload_snapshot(active, campaigns.modules());
                        self.saved = true;
                    }
                    Err(error) => self.error = Some(error),
                }
            }
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
                        self.stamp_kept(KeptResult::RunDeleted);
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
                    self.from_session = None;
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
            self.start_refused(Some(error));
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
                self.start_refused(invalid);
                self.error = Some(error);
            }
        }
    }

    /// The encounter file's flags and unread roster rows, from its source and the campaign's
    /// creature names. FC3: the roster shows each row that blocks Start fight in its place: the
    /// faults move there (kept for this draft in `ctx`); nothing is copied.
    fn unread_rows(&self, ctx: &egui::Context) -> Option<UnreadRows> {
        let no_names = BTreeMap::new();
        let names = self
            .snapshot
            .as_ref()
            .map(|snapshot| &snapshot.known_creature_names)
            .unwrap_or(&no_names);
        let draft = self.draft.as_ref()?;
        let faults = Entry::parse(&draft.path, &draft.source)
            .map(|entry| roster_faults(&entry, names))
            .unwrap_or_default();
        let notes = unread_notes(&draft.path, &draft.flags, &faults);
        let count = faults.len();
        let rows: Arc<Vec<RosterFault>> = Arc::new(faults);
        ctx.data_mut(|data| data.insert_temp(roster_fault_rows_id(draft.id), rows));
        (!notes.is_empty()).then(|| UnreadRows {
            notes,
            faults: count,
            path: draft.path.clone(),
        })
    }

    /// Rules and runs' footer (`run_settings`'s, and the GPUI tab's): a commit's effects first
    /// (a save reloads the settings, a failure says why), then each line from its owner as the
    /// acts left it: the discard's result with its Undo, each unread rule choice's warning, the
    /// runs a folder change leaves with the folder's two choices, the error, and the open draft.
    #[allow(
        clippy::too_many_arguments,
        reason = "the footer reads the frame’s acts (commit, the form’s run files) and its locks"
    )]
    fn run_settings_footer<'f>(
        &mut self,
        ctx: &egui::Context,
        frame: &'f design::outcome::PaneFrame,
        messages: &mut design::outcome::Messages<'f, FooterPress>,
        active: &Campaign,
        modules: &[Module],
        commit: Option<Result<Option<String>, String>>,
        form_left: Option<(String, Result<Vec<std::path::PathBuf>, String>)>,
        run_open: bool,
        draft_open: bool,
    ) {
        // The acts' own effects first: a save reloads the settings (which ends the
        // error), and a failed attempt says why.
        if let Some(commit) = commit {
            self.options_committed(ctx, active, modules.to_vec(), commit);
        }
        // Then the footer's lines, each from its owner as the acts left it. The
        // discard's result is the page's store's (`KeptResult::OptionsDiscarded`, Undo by
        // its ticket): it ends when its draft was dropped (an edit, a save, another
        // campaign), and shows while the form is unchanged.
        let discarded = KeptOwner {
            root: active.path.clone(),
            kind: KeptResult::OptionsDiscarded,
        };
        if self.discarded_options.is_none() {
            self.kept.owner_changed(&discarded);
        }
        let snapshot = self.snapshot.as_ref();
        if snapshot.is_some_and(|snapshot| snapshot.options == snapshot.saved_options)
            && let Some(shown) = self.kept.get(&discarded)
        {
            messages.kept(shown.outcome(frame, |_| options_discarded()));
        }
        // Each unread choice's warning names its line; the file's path and links follow
        // the last one.
        let rule_words: Vec<String> = snapshot
            .map(|snapshot| rule_choices(snapshot, active.id))
            .unwrap_or_default()
            .into_iter()
            .map(|(_, words)| words)
            .collect();
        if let Some((last, before)) = rule_words.split_last() {
            for words in before {
                messages.outcome(design::outcome::FaultMessage::new(
                    &EncounterFault::FileWarning(words.clone()),
                ));
            }
            messages.outcome(
                design::outcome::FaultMessage::new(&EncounterFault::FileWarning(last.clone()))
                    .file(
                        design::outcome::FileLine::new(&active.path, "Campaign.md")
                            .edit()
                            .show(),
                    ),
            );
        }
        // The run files the form read, unless an act changed the folder after it
        // (Discard puts the saved one back).
        let left = snapshot.zip(form_left).map(|(snapshot, (folder, left))| {
            if snapshot.options.folder == folder {
                left
            } else {
                snapshot
                    .options
                    .runs_left_in(&snapshot.saved_options, active)
            }
        });
        if let Some(snapshot) = snapshot
            && let Some(Ok(left)) = &left
            && !left.is_empty()
        {
            let run_files = left.len();
            let runs: Vec<_> = left
                .iter()
                .filter_map(|path| crate::encounter_run::Run::open(path, active.id).ok())
                .filter(|run| !run.state_words(true).starts_with("Ended"))
                .collect();
            let words = if run_files == 1
                && let Some(run) = runs.first()
            {
                format!(
                    "{} ({}) stays in {}. From the new folder, Splat will not show it or resume it.",
                    run.name,
                    run.state_words(true),
                    snapshot.saved_options.folder
                )
            } else {
                format!(
                    "{} {} in {}. {} are unfinished. From the new folder, Splat will not show these runs or resume them.",
                    design::count(run_files, "saved run", "saved runs"),
                    if run_files == 1 { "stays" } else { "stay" },
                    snapshot.saved_options.folder,
                    runs.len()
                )
            };
            // M4: the warning's two choices are the folder's commit while it leaves
            // runs; none while a run or the open encounter locks the form (the save row
            // says why).
            let mut warning = design::outcome::FaultMessage::new(&EncounterFault::RunsLeft(words));
            if !run_open && !draft_open {
                for (words, press) in [
                    ("Move runs and save", FooterPress::MoveRuns),
                    ("Save without moving", FooterPress::SaveWithoutMoving),
                ] {
                    warning = warning.choice(design::outcome::Choice::new(
                        design::outcome::MessageChoice::Choice(words),
                        press,
                    ));
                }
            }
            messages.outcome(warning);
        }
        // An act's answer (a failed save or move) after the standing reasons, so it never
        // moves the choice the DM pressed.
        if let Some(e) = &self.error {
            messages.outcome(design::outcome::FaultMessage::new(&EncounterFault::Failed(
                e.clone(),
            )));
        }
        if self.draft_dirty() {
            messages.outcome(design::outcome::Notice::new(crate::sentence!(
                "Save or discard the open encounter to change run settings."
            )));
        }
    }

    /// The effects of one commit of Rules and runs (`commit_options`): a write records its
    /// settings for the save row's `Saved` (`options_saved`), tells the app the file changed
    /// and reads the settings again (which ends the last error); a move's warning or a failure
    /// is the footer's error.
    fn options_committed(
        &mut self,
        ctx: &egui::Context,
        active: &Campaign,
        modules: Vec<Module>,
        result: Result<Option<String>, String>,
    ) {
        match result {
            Ok(warning) => {
                // The settings as the file holds them (issue #519): the reload that follows reads
                // them back, so `Saved` stays.
                self.options_saved = self
                    .snapshot
                    .as_ref()
                    .map(|snapshot| (active.id, snapshot.options.as_saved()));
                self.saved = true;
                self.reload(ctx, active.clone(), modules);
                self.error = warning;
            }
            Err(e) => self.error = Some(e),
        }
    }
}

/// `Save run settings` waits for the folder warning's choices (ruling M4), as `Save folder` does.
const CHOOSE_MOVE: &str = "Choose below to move the runs or not.";

/// How Rules and runs commits its settings (ruling M4): `Save run settings`, or one of the folder
/// warning's two choices while the folder change leaves saved runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OptionsCommit {
    Save,
    MoveRuns,
    WithoutMoving,
}

/// Writes the shown settings: `Ok` with a move's warning when it wrote, `Err` with why it did
/// not. A save that leaves the old folder reads it again at the act, and stops when it cannot.
fn commit_options(
    snapshot: &Snapshot,
    active: &Campaign,
    commit: OptionsCommit,
) -> Result<Option<String>, String> {
    let options = &snapshot.options;
    let saved = &snapshot.saved_options;
    if commit == OptionsCommit::MoveRuns {
        return options.move_runs_and_save(saved, active, &snapshot.campaign_text);
    }
    if options.directory(active).ok() != saved.directory(active).ok() {
        options.runs_left_in(saved, active)?;
    }
    options.save(active, &snapshot.campaign_text).map(|()| None)
}

/// The unread rule choices Campaign sends to Rules and runs with their repair link, by their kind,
/// never their words: each one's line and warning.
fn rule_choices(snapshot: &Snapshot, campaign: Uuid) -> Vec<(String, String)> {
    snapshot
        .catalogue
        .entries
        .get(&campaign)
        .into_iter()
        .flat_map(|item| &item.entry.flags)
        .filter(|flag| flag.kind == splat::content::layout::FlagKind::RuleChoice)
        .map(|flag| (flag.line.to_string(), flag.message.clone()))
        .collect()
}

/// `Rules and runs draft discarded.` with `Undo discard of Rules and runs`.
fn options_discarded() -> design::outcome::KeptOutcome<FooterPress> {
    design::outcome::KeptOutcome::Notice(
        design::outcome::Notice::new(crate::sentence!("Rules and runs draft discarded.")),
        design::outcome::UndoStep::of(design::outcome::Verb::Discard, "Rules and runs")
            .pressed_as(FooterPress::Undo),
    )
}

#[path = "encounters/list.rs"]
mod list;
pub use list::run_state_word;

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

#[cfg(any(test, feature = "test-support"))]
pub use splat_content_ui::content_actions::{blocked_draft_switch_reason, draft_switch_reason};

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

    /// M3, F1 of the review of PR 450: the replacement picker ranks in the one creature order
    /// before it takes its five, so the exact name comes first, also after five names that hold
    /// it; with no words it keeps the list's first five.
    #[test]
    fn the_replacement_picker_ranks_before_it_takes_five() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        for name in [
            "Ancient Fenstalker",
            "Bog Fenstalker",
            "Cave Fenstalker",
            "Dread Fenstalker",
            "Elder Fenstalker",
            "Fenstalker",
        ] {
            creature(dir.path(), name, false);
        }
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let names = |query: &str| -> Vec<String> {
            replacement_matches(&snapshot.creatures, query)
                .into_iter()
                .map(|c| c.name.clone())
                .collect()
        };
        assert_eq!(
            names("fenstalker"),
            [
                "Fenstalker",
                "Ancient Fenstalker",
                "Bog Fenstalker",
                "Cave Fenstalker",
                "Dread Fenstalker",
            ]
        );
        let listed: Vec<String> = snapshot
            .creatures
            .iter()
            .filter(|c| !c.linked)
            .take(5)
            .map(|c| c.name.clone())
            .collect();
        assert_eq!(listed.len(), 5, "fixture: five creatures listed");
        assert_eq!(names(""), listed);
        // M-E0 (review of M-Q0.2): the encounter search keeps its ranking for its view
        // (`add_view`, as the GPUI page builds it). An unchanged frame ranks no creature; new
        // words and a change of the roster each rank again.
        let ranked = || SEARCH_RANKED.with(std::cell::Cell::get);
        let first = snapshot.creatures[0].id;
        let mut page = Encounters {
            draft: Some(Draft::new()),
            snapshot: Some(snapshot),
            ..Default::default()
        };
        page.set_search("fenstalker");
        let before = ranked();
        assert!(page.add_view(Filter::default()).is_some());
        let seen = ranked();
        assert!(seen > before, "the words ranked");
        let _ = page.add_view(Filter::default());
        assert_eq!(ranked(), seen, "an unchanged frame ranks no creature");
        page.set_search("fen");
        let _ = page.add_view(Filter::default());
        assert!(ranked() > seen, "new words rank again");
        let seen = ranked();
        page.draft.as_mut().unwrap().add(first, false);
        let _ = page.add_view(Filter::default());
        assert!(ranked() > seen, "a change of the roster ranks again");
    }

    #[test]
    fn iter_2_b_npc_with_deleted_stat_block_stays_in_search_and_cannot_be_added() {
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
        // The search lists it with why it cannot be added (the GPUI result shows these words
        // and no Add).
        let watchkeeper = snapshot
            .search_list(false)
            .creatures()
            .find(|c| c.name == "Watchkeeper")
            .expect("the search lists the NPC");
        assert_eq!(
            watchkeeper.unavailable.as_deref(),
            Some("Watchkeeper Guard stat block not found")
        );
    }

    /// FD3 edge content: unread rows that are not roster rows and roster rows with a long path
    /// each get the right words and tone (the GPUI footer draws them as typed lines).
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
        // R41 (lead ruling, s30 B11): an unread roster row's note is an error; another flag of
        // the file is a warning.
        assert_eq!(
            unread_notes(
                "Encounters/Shrine.md",
                &[saved(3, "Line 3: the date was not read.")],
                &[]
            ),
            [(
                "Encounters/Shrine.md: Line 3: the date was not read.".to_owned(),
                Tone::Warning
            )]
        );
        assert_eq!(
            unread_notes(
                "Encounters/Shrine.md",
                &[saved(20, "Line 20: the change was not read.")],
                &[row("Ember Cultist", 20)]
            ),
            [(
                "Ember Cultist: Line 20: the change was not read.".to_owned(),
                Tone::Error
            )]
        );
        for (flags, rows, path, expected, sentence) in [
            (
                vec![flag(3, "the date was not read.")],
                vec![],
                "Encounters/Shrine.md",
                vec!["Encounters/Shrine.md, line 3: The date was not read."],
                None,
            ),
            (
                vec![flag(20, "the change was not read.")],
                vec![row("Ember Cultist", 20)],
                "Encounters/Shrine.md",
                vec!["Ember Cultist · line 20: The change was not read."],
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
                    "Ember Cultist · line 20: The change “HP lots” was not read.",
                    "Row on line 21 · line 21: The creature cell is empty.",
                    "Line 4: The date was not read.",
                ],
                Some(fix_roster_rows(2)),
            ),
        ] {
            let notes = unread_notes(path, &flags, &rows);
            let (words, tones): (Vec<String>, Vec<Tone>) = notes.iter().cloned().unzip();
            assert_eq!(words, expected);
            // R41: each roster row's note in the error tone of Save's reason for it.
            for (flag, tone) in flags.iter().zip(&tones) {
                let roster_row = rows.iter().any(|row| row.line == flag.line);
                assert_eq!(
                    *tone,
                    if roster_row {
                        unread_rows_found(String::new()).tone().unwrap()
                    } else {
                        Tone::Warning
                    },
                    "line {}",
                    flag.line
                );
            }
            assert_eq!(unread_row_tone(), Tone::Error);
            // The run setup and Save's reason give each row the editor's words (iter-23 FA4).
            for row in &rows {
                let flag = flags.iter().find(|flag| flag.line == row.line).unwrap();
                let fault = RosterFault {
                    problem: flag.words().unwrap(),
                    ..row.clone()
                };
                assert!(words.contains(&fault.words()), "{}", fault.words());
                let reason = roster_fault_reason(path, std::slice::from_ref(&fault)).unwrap();
                assert!(
                    reason.contains(fault.words().trim_end_matches('.')),
                    "{reason}"
                );
            }
            // Save's reason for all the rows ends with the one sentence that says what to do.
            let faults: Vec<RosterFault> = rows
                .iter()
                .map(|row| RosterFault {
                    problem: flags
                        .iter()
                        .find(|flag| flag.line == row.line)
                        .and_then(|flag| flag.words())
                        .unwrap(),
                    ..row.clone()
                })
                .collect();
            let reason = roster_fault_reason(path, &faults);
            assert_eq!(reason.is_some(), sentence.is_some(), "{reason:?}");
            if let (Some(reason), Some(sentence)) = (reason, sentence) {
                assert!(reason.ends_with(sentence), "{reason}");
            }
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
        let goblin = creature(dir.path(), "Goblin", false);
        let guard = creature(dir.path(), "Guard", false);
        let old = Snapshot::load(&active, &[]).unwrap();
        fs::remove_file(dir.path().join("Cinder Wolf.md")).unwrap();
        fs::remove_file(dir.path().join("Goblin.md")).unwrap();
        let mut snapshot = Snapshot::load(&active, &[]).unwrap();
        snapshot.keep_edited_settings(&old);
        // G125: the rows say their entry was deleted; the message says only the next act, and
        // names and counts every such row, among rows whose creature is there.
        let mut mixed = Draft::new();
        mixed.add(id, false);
        mixed.add(guard, false);
        mixed.add(goblin, false);
        mixed.add(id, false);
        assert_eq!(
            mixed.validate(&snapshot).unwrap_err().text,
            "Choose replacements for the 2 Cinder Wolf rows and the Goblin row before you save or start a fight."
        );
        // A row with a changed name is named as the roster shows it, also beside a plain row of
        // the same creature.
        mixed.setup.roster[3].overrides = yaml("{name: Ash}").as_mapping().unwrap().clone();
        assert_eq!(
            mixed.validate(&snapshot).unwrap_err().text,
            "Choose replacements for the Cinder Wolf row, the Goblin row and the Ash row before you save or start a fight."
        );
        let mut draft = Draft::new();
        draft.add(id, false);
        draft.add(id, false);
        let error = draft.validate(&snapshot).unwrap_err();
        assert_eq!(
            error.text,
            "Choose replacements for the 2 Cinder Wolf rows before you save or start a fight."
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
        assert!(encounters.error.as_ref().unwrap().starts_with(
            "Choose a replacement for the Cinder Wolf row before you save or start a fight."
        ));
        let missing = encounters.draft.as_ref().unwrap().setup.roster.clone();
        encounters.draft.as_mut().unwrap().setup.roster.clear();
        encounters.refresh_validation_error(false, true);
        assert!(encounters.error.is_none());
        encounters.draft.as_mut().unwrap().setup.roster = missing;
        encounters.refresh_validation_error(false, true);
        assert!(encounters.error.as_ref().unwrap().starts_with(
            "Choose a replacement for the Cinder Wolf row before you save or start a fight."
        ));

        // The editor's frame (`editor_frame`, as the GPUI page draws it) checks the refused
        // draft again: after an edit, Discard and Undo, the reason follows the draft.
        encounters.active = Some(active.clone());
        let shows = |encounters: &Encounters, words: &str| {
            encounters
                .error
                .as_deref()
                .is_some_and(|error| error.starts_with(words))
        };
        let one = "Choose a replacement for the Cinder Wolf row before you save or start a fight.";
        let two =
            "Choose replacements for the 2 Cinder Wolf rows before you save or start a fight.";
        let _ = encounters.editor_frame(&active);
        assert!(shows(&encounters, one), "refusal-check-shown");
        // An edit: the second row comes back, and the reason counts both.
        let row = encounters.draft.as_ref().unwrap().setup.roster[0].clone();
        encounters.draft.as_mut().unwrap().setup.roster.push(row);
        let _ = encounters.editor_frame(&active);
        assert!(shows(&encounters, two), "refusal-check-edit");
        // Discard: a new draft has no saved encounter to go back to, so its refusal goes with it
        // (the page shows the list, so no editor frame runs).
        encounters.discard_draft(&active);
        assert!(
            encounters.draft.is_none() && encounters.error.is_none(),
            "refusal-check-discard"
        );
        // Undo: the draft comes back and its reason with it.
        encounters.undo_discard();
        let _ = encounters.editor_frame(&active);
        assert!(shows(&encounters, two), "refusal-check-undo");
        // A check that passes clears the stored refusal.
        encounters.draft.as_mut().unwrap().setup.roster.clear();
        let _ = encounters.editor_frame(&active);
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
        encounters.expire_discards();
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

        // Issue #519: a folder saved with outer spaces keeps its `Saved`. Save writes it
        // trimmed, and the save row's record is the settings as the file holds them, so the
        // reload that follows (which reads them back) does not end it.
        let mut encounters = Encounters {
            active: Some(active.clone()),
            snapshot: Some(Snapshot::load(&active, &[]).unwrap()),
            ..Default::default()
        };
        encounters.snapshot.as_mut().unwrap().options.folder = "Runs  ".into();
        let result = commit_options(
            encounters.snapshot.as_ref().unwrap(),
            &active,
            OptionsCommit::Save,
        );
        assert!(result.is_ok(), "the folder saves: {result:?}");
        encounters.options_committed(&egui::Context::default(), &active, Vec::new(), result);
        let reread = Snapshot::load(&active, &[]).unwrap();
        assert_eq!(
            encounters.options_saved.map(|(_, saved)| saved),
            Some(reread.saved_options),
            "#519: the record is what the file holds, so the reload keeps Saved"
        );
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
    /// T12.12: a reader's or a scene's `Start fight` saves `Fight with <name>` with the party and
    /// the creature and starts its run. After the untouched run is closed, also after a restart,
    /// it starts that encounter again; once that one was run, a new one takes the next number.
    /// An NPC with no stat block opens the made-up form with its name. It is blocked as the
    /// editor's `Start fight` is, and then writes nothing.
    #[test]
    fn start_fight_with_saves_one_encounter_and_starts_it_again() {
        use crate::world::StartFight;
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let party = creature(dir.path(), "Kestrel Dunmoor", true);
        let bandit = creature(dir.path(), "Bandit", false);
        let fixture = |name: &str, creature: &str| {
            splat::content::formats::fixture(
                "Fixture.md",
                &format!(
                    "splat_id: {}\nsplat_kind: world\nsplat_name: {name}\nsplat_world:\n  type: npc\n{creature}",
                    Uuid::new_v4()
                ),
                "",
            )
            .unwrap()
        };
        write_note(dir.path(), "Varra.md", None, &fixture("Captain Varra", "")).unwrap();
        let fresh = || Encounters {
            snapshot: Some(Snapshot::load(&active, &[]).unwrap()),
            active: Some(active.clone()),
            ..Default::default()
        };
        let encounter = |name: &str| dir.path().join(format!("Encounters/{name}.md"));
        let with_bandit = StartFight {
            name: "Bandit".into(),
            creature: Some(bandit),
            npc: false,
        };
        let mut encounters = fresh();
        encounters.start_fight_with(&with_bandit, &active, &[]);
        let run = encounters.run.take().expect("the run starts");
        let source = run.source();
        let saved = Draft::open(dir.path(), "Encounters/Fight with Bandit.md").unwrap();
        let mut ids: Vec<_> = saved
            .setup
            .roster
            .iter()
            .filter_map(|slot| slot.creature.as_ref()?.id)
            .collect();
        ids.sort();
        let mut wanted = vec![party, bandit];
        wanted.sort();
        assert_eq!(ids, wanted, "the party and the creature");
        assert_eq!(source, Some(saved.id));
        // Close run on an untouched Setup removes its file, and the app restarts.
        fs::remove_file(run.path().unwrap()).unwrap();
        let mut encounters = fresh();
        encounters.start_fight_with(&with_bandit, &active, &[]);
        assert_eq!(
            encounters.run.as_ref().and_then(|run| run.source()),
            Some(saved.id)
        );
        assert!(
            !encounter("Fight with Bandit 2").exists(),
            "no second encounter"
        );
        // That one was run now, so the next fight is a new encounter.
        encounters.run = None;
        let mut encounters = fresh();
        encounters.start_fight_with(&with_bandit, &active, &[]);
        assert!(encounters.run.is_some());
        assert!(encounter("Fight with Bandit 2").exists());

        // An NPC with no stat block: the party, and the made-up form with its name.
        let mut encounters = fresh();
        encounters.start_fight_with(
            &StartFight {
                name: "Captain Varra".into(),
                creature: None,
                npc: true,
            },
            &active,
            &[],
        );
        let run = encounters.run.as_ref().expect("the run starts");
        assert_eq!(run.made_up_name(), Some("Captain Varra"));
        let saved = Draft::open(dir.path(), "Encounters/Fight with Captain Varra.md").unwrap();
        assert_eq!(saved.setup.roster.len(), 1);

        // An encounter of that name that holds other values is not what the click saves: a
        // starting HP on a slot, or another setting of the fight.
        for (name, change) in [("Scout", 0), ("Lookout", 1)] {
            let mut other = Draft::new();
            other.name = format!("Fight with {name}");
            other.add(party, true);
            other.add(bandit, false);
            if change == 0 {
                other.setup.roster[1].initial.insert("hp".into(), 3.into());
            } else {
                other.setup.repeatable = true;
            }
            other.save(&active, &[]).unwrap();
            let mut encounters = fresh();
            encounters.start_fight_with(
                &StartFight {
                    name: name.into(),
                    creature: Some(bandit),
                    npc: false,
                },
                &active,
                &[],
            );
            assert!(encounters.run.is_some());
            assert!(
                encounter(&format!("Fight with {name} 2")).exists(),
                "{name}: a new encounter, not the one with other values"
            );
        }

        // Review of PR 261: a party sheet written a moment before the click, before the next
        // content poll, is in the fight that the click saves.
        let mut encounters = fresh();
        let late = creature(dir.path(), "Late Arrival", true);
        encounters.start_fight_with(
            &StartFight {
                name: "Ruffian".into(),
                creature: Some(bandit),
                npc: false,
            },
            &active,
            &[],
        );
        assert!(encounters.run.is_some(), "the run starts");
        let saved = Draft::open(dir.path(), "Encounters/Fight with Ruffian.md").unwrap();
        assert!(
            saved
                .setup
                .roster
                .iter()
                .any(|slot| slot.creature.as_ref().and_then(|r| r.id) == Some(late)),
            "Start fight must include the party on disk at the click"
        );

        // Blocked as the editor's Start fight is, with its words, and nothing is written.
        let mut encounters = fresh();
        encounters.draft = Some(Draft::new());
        let before = fs::read_dir(dir.path().join("Encounters")).unwrap().count();
        encounters.start_fight_with(&with_bandit, &active, &[]);
        assert!(encounters.run.is_none());
        assert_eq!(encounters.error.as_deref(), Some(BLOCKED_BY_DRAFT));
        assert_eq!(
            fs::read_dir(dir.path().join("Encounters")).unwrap().count(),
            before
        );

        // B4, E1: the plan's and the run's routes that would replace the open drafts are refused
        // in the owner while one is unsaved, each with that draft's reason, and keep every draft.
        let kept = encounters.draft.as_ref().map(|draft| draft.id);
        encounters.error = None;
        encounters.new_from_plan(None);
        assert_eq!(
            encounters.draft.as_ref().map(|d| d.id),
            kept,
            "New fight keeps it"
        );
        assert_eq!(encounters.error.as_deref(), Some(BLOCKED_BY_DRAFT));
        encounters.error = None;
        assert_eq!(
            encounters.new_group_in_plan(),
            PlanOpen::Refused(BLOCKED_BY_DRAFT)
        );
        assert!(encounters.group_draft.is_none(), "New group is refused");
        assert_eq!(encounters.error.as_deref(), Some(BLOCKED_BY_DRAFT));
        encounters.error = None;
        assert_eq!(
            encounters.open_group_in_plan("Groups/Any.md", &active),
            PlanOpen::Refused(BLOCKED_BY_DRAFT)
        );
        assert!(
            encounters.group_draft.is_none(),
            "a group is not opened over it"
        );
        assert_eq!(encounters.draft.as_ref().map(|d| d.id), kept);
        assert_eq!(encounters.error.as_deref(), Some(BLOCKED_BY_DRAFT));
        // A changed group holds them too, with the group's own reason.
        let mut encounters = fresh();
        encounters.group_draft = Some(GroupDraft::new());
        let group = encounters.group_draft.as_ref().map(GroupDraft::id);
        let group_reason = PlanOpen::Refused(design::Unsaved::Group.reason());
        assert_eq!(
            encounters.open_group_in_plan("Groups/Any.md", &active),
            group_reason
        );
        assert_eq!(encounters.new_group_in_plan(), group_reason);
        assert_eq!(encounters.new_from_plan(None), group_reason);
        assert_eq!(encounters.group_draft.as_ref().map(GroupDraft::id), group);
        assert!(
            encounters.draft.is_none(),
            "no new fight over the changed group"
        );
        assert_eq!(
            encounters.error.as_deref(),
            Some(design::Unsaved::Group.reason())
        );
        // With nothing unsaved, New fight opens its draft.
        let mut encounters = fresh();
        encounters.new_from_plan(None);
        assert!(
            encounters.draft.is_some(),
            "a new fight opens with nothing unsaved"
        );
        assert_eq!(encounters.error, None);
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
        // F1: a roster with no character weighs against the campaign's party: every party copy,
        // also one whose note cannot be read. Two saved sheets at level 3 give 300 / 450 / 800.
        let sheet = serde_json::to_string(&splat::content::character::Sheet {
            level: 3,
            ..Default::default()
        })
        .unwrap();
        for name in ["Hero", "Other"] {
            let id = if name == "Hero" { pc } else { Uuid::new_v4() };
            let text = splat::content::formats::fixture(
                &format!("{name}.md"),
                &format!(
                    "splat_id: {id}\nsplat_kind: creature\nsplat_name: {name}\nsplat_external: {{provider: dndbeyond, campaign_id: '123'}}\nsplat_character: {sheet}\nsplat_creature: {{}}\n"
                ),
                "",
            )
            .unwrap();
            fs::write(dir.path().join(format!("{name}.md")), text).unwrap();
        }
        let weigh = || {
            Snapshot::load(&campaign, &[])
                .unwrap()
                .scale
                .weigh(&[], [Some(250)])
        };
        assert_eq!(weigh().rating.unwrap().budgets, [300, 450, 800]);
        // One sheet cannot be read: the party is not one character smaller, its level is not
        // known, so the word does not show.
        let path = dir.path().join("Hero.md");
        let text = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            text.replacen("splat_schema: 2", "splat_schema: [", 1),
        )
        .unwrap();
        let unreadable = weigh();
        assert_eq!(
            (unreadable.xp, unreadable.rating.clone()),
            (250, None),
            "{unreadable:?}"
        );
    }
    /// X2, review 290: a typed reason with no name known still says what is not known.
    #[test]
    fn review290_unnamed_missing_level_still_has_a_reason() {
        let missing = crate::difficulty::Missing {
            what: crate::difficulty::Unknown::Level,
            at: vec![0],
            names: Vec::new(),
        };
        let difficulty = crate::difficulty::Difficulty {
            missing: Some(missing),
            ..Default::default()
        };
        let shown =
            crate::difficulty::Line::roster(&difficulty, crate::difficulty::Summary::default())
                .fit(|_| true)
                .map(|fitted| fitted.text());
        assert_eq!(
            shown.as_deref(),
            Some("Level not known"),
            "A typed missing-level result must not disappear when its name is unavailable"
        );
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

        // FF7: the file goes while the draft is open. The red choice says what it removes, its
        // result says what was lost, and Undo remove gives the draft back (through the model
        // the GPUI page draws: `encounter_disk`, `encounter_disk_choice`, the list's footer).
        fs::remove_file(&path).unwrap();
        draft.body = "My later words".into();
        assert!(draft.save(&campaign, &[]).is_err());
        assert!(draft.recovery.file_gone());
        let name = draft.name.clone();
        let mut editor = Encounters {
            // The kept results are the campaign's (`KeptOwner`).
            active: Some(campaign.clone()),
            draft: Some(draft),
            snapshot: Some(Snapshot::load(&campaign, &[]).unwrap()),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        let view = editor.encounter_disk().expect("the gone file's conflict");
        assert!(view.disk.is_none());
        assert_eq!(view.lose, "Remove encounter and lose draft");
        editor.encounter_disk_choice(&ctx, &campaign, &[], false);
        assert!(editor.draft.is_none());
        let lines = editor.list_footer();
        let removed = lines
            .iter()
            .find(|line| line.words == format!("{name} removed. Its draft was discarded."))
            .expect("the result names the encounter and what was lost");
        let undo = removed.press_undo().expect("its Undo");
        assert_eq!(
            removed.undo.as_ref().map(|undo| undo.label.as_str()),
            Some(format!("Undo remove {name}").as_str())
        );
        editor.footer_press(&ctx, undo, &campaign, &Campaigns::default());
        let back = editor
            .draft
            .as_ref()
            .expect("Undo remove gives the draft back");
        assert_eq!(back.body.as_str(), "My later words");
        assert!(editor.discarded.is_none());
        assert!(!path.exists(), "nothing was written");
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
        assert_eq!(
            invalid.text,
            "Choose a replacement for the Cinder Wolf row before you save or start a fight."
        );
        assert!(invalid.tone == Tone::Warning);
        // The row says the same fault in the same tone, and so does the save row's word (R41).
        assert!(invalid.tone == missing_creature_tone(None));
        assert_eq!(invalid.diagnosis().tone(), Some(invalid.tone));
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
        encounters.error = Some(format!("The file did not save. {}", invalid.text));
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
        assert_eq!(invalid.diagnosis().tone(), Some(invalid.tone));
        assert_eq!(missing_creature_mark(unread).0, icons::ERROR);
        // Issue #530 (2): a row renamed "Guard" keeps the next step on its Library entry. The
        // page keeps the creature's known name across reads, as for a deleted one.
        let mut snapshot = snapshot;
        snapshot.keep_edited_settings(&old);
        let mut draft = draft;
        draft.setup.roster[0]
            .overrides
            .insert(Value::from("name"), Value::from("Guard"));
        let renamed = draft.validate(&snapshot).unwrap_err();
        let refusal = renamed
            .start_refusal("Bridge Ambush")
            .expect("a typed answer");
        assert_eq!(
            refusal.words().as_str(),
            "No fight started from Bridge Ambush. Splat cannot read the file of Cinder Wolf. Fix \
             Cinder Wolf in Library first.",
            "#530: the next step names the Library entry, not the row's name"
        );
        encounters.snapshot = Some(snapshot);
        encounters.draft = Some(draft);
        encounters.error = Some(invalid.text.clone());
        encounters.validation_error = Some(invalid);
        assert!(encounters.error_tone() == Tone::Error);
        // Round 23 AD2: two files that claim one identity are a warning, as Library shows them,
        // in the message and in the row. The fight stays blocked until the identity is clear.
        fs::write(&file, &original).unwrap();
        fs::write(dir.path().join("Cinder Wolf copy.md"), &original).unwrap();
        let mut snapshot = Snapshot::load(&active, &[]).unwrap();
        snapshot.keep_edited_settings(&old);
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
        assert_eq!(invalid.diagnosis().tone(), Some(invalid.tone));
        assert_eq!(missing_creature_mark(duplicate).0, icons::WARN);
        // Issue #530 (1): Start's answer says the duplicate in its own warning, not as a file
        // that cannot be read (an error).
        let refusal = invalid
            .start_refusal("Bridge Ambush")
            .expect("a typed answer");
        assert_eq!(
            (refusal.words().as_str(), refusal.tone()),
            (
                "No fight started from Bridge Ambush. Two files claim the creature Cinder Wolf. Fix \
                 Cinder Wolf in Library first.",
                Tone::Warning
            ),
            "#530: two files that claim the creature"
        );
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
        // Issue #530 (item 4): every fault that refuses Start has its typed answer, in the
        // `No fight started from …` form.
        let answers = std::cell::RefCell::new(Vec::new());
        let fault = |change: &dyn Fn(&mut Draft)| {
            let mut draft = new_draft();
            change(&mut draft);
            let invalid = draft.validate(&snapshot).unwrap_err();
            let text = invalid.text.clone();
            assert!(!text.contains("actor_"), "{text}");
            let answer = invalid
                .start_refusal(&draft.name)
                .unwrap_or_else(|| panic!("#530: no typed Start answer for {text:?}"));
            assert!(
                answer
                    .words()
                    .as_str()
                    .starts_with("No fight started from Bridge. "),
                "#530: {:?}",
                answer.words()
            );
            answers
                .borrow_mut()
                .push(answer.words().as_str().to_owned());
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
        let answers = answers.into_inner();
        for expected in [
            "No fight started from Bridge. The starting HP of the Cinder Wolf row must be a whole \
             number, 0 or more. Change the starting values of the Cinder Wolf row in the roster \
             first.",
            "No fight started from Bridge. The changed side nowhere of the Ash row is not a side \
             of this encounter. Fix the changes of the Ash row in the roster first.",
        ] {
            assert!(
                answers.iter().any(|words| words == expected),
                "{answers:#?}"
            );
        }
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
        // Ruling L9, early review F1 of PR 444: the revision of the saved-run facts, which the
        // Add results' words key by, holds on a poll with no change, and moves when the run
        // list is read again or a run opens.
        editor.note_run_facts();
        let facts = editor.saved_runs_revision();
        editor.note_run_facts();
        assert_eq!(
            editor.saved_runs_revision(),
            facts,
            "no change, no new revision"
        );
        editor.follow_continue_rule(&played);
        assert_eq!(
            editor.history_blocked,
            Some(crate::encounter_run::CONTINUE_LATER)
        );
        let mut taken = saved.clone();
        taken["id"] = serde_json::json!(later.id);
        fs::write(&path, serde_json::to_vec_pretty(&taken).unwrap()).unwrap();
        editor.read_runs_again(&campaign);
        editor.note_run_facts();
        assert_ne!(
            editor.saved_runs_revision(),
            facts,
            "the run list read again is new facts"
        );
        editor.open_saved_run(&path, &campaign);
        editor.follow_continue_rule(&played);
        assert_eq!(
            editor.history_blocked, None,
            "the run now at this path is the fight played last"
        );
        fs::write(&path, &bytes).unwrap();
        editor.snapshot = None;

        editor.note_run_facts();
        let no_run = editor.saved_runs_revision();
        editor.run = Some(editor.start_run(&campaign, &[]).unwrap());
        editor.note_run_facts();
        assert_ne!(
            editor.saved_runs_revision(),
            no_run,
            "an open run's facts are over the list"
        );
        // Review F2 of PR 444: an unchanged poll copies no timeline fact (no run name).
        let open = editor.saved_runs_revision();
        let made = crate::encounter_run::TIMELINE_FACTS.with(std::cell::Cell::get);
        for _ in 0..3 {
            editor.note_run_facts();
        }
        assert_eq!(
            (
                editor.saved_runs_revision(),
                crate::encounter_run::TIMELINE_FACTS.with(std::cell::Cell::get)
            ),
            (open, made),
            "an unchanged poll keeps the revision and copies no fact of the run"
        );
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
        assert!(run.continue_blocked.is_some_and(|reason| {
            reason.starts_with("This fight cannot go on: it is not on the Timeline")
        }));
        // Review of PR 666 (P1): with an edit that waits for its save over a Timeline.md that
        // changed on disk, the refusal says so, and the app's next working-out (the new run
        // changed its key) keeps it until the save settles the file.
        let reason = |editor: &Encounters| {
            editor
                .run()
                .unwrap()
                .unend_refused
                .as_ref()
                .map(|(reason, _, _)| *reason)
        };
        let event = timeline.add(crate::timeline::Kind::Event, None);
        timeline.save();
        let mut other = crate::timeline::Book::default();
        other.open(Some(dir.path()));
        other.add(crate::timeline::Kind::Event, None);
        other.save();
        timeline.event_mut(&event).unwrap().notes = "Waiting to save".into();
        timeline.touch();
        editor.continue_saved_run(&path, &campaign);
        answer(&mut editor, &mut timeline);
        assert_eq!(reason(&editor), Some(crate::encounter_run::Run::UNSETTLED));
        editor.follow_continue_rule(&timeline);
        assert_eq!(
            reason(&editor),
            Some(crate::encounter_run::Run::UNSETTLED),
            "the refresh keeps the refusal while the edit waits over the changed file"
        );
        timeline.save();
        editor.follow_continue_rule(&timeline);
        assert_eq!(
            reason(&editor),
            None,
            "the save settles it, and the answer ends"
        );
        // Review of PR 733 (P2): with the content index the app gives the run, the answer says
        // the encounter was deleted while no encounter has its id. A change to another
        // encounter keeps it; the encounter restored ends it, on the app's every-frame check
        // with the rule's key unchanged.
        let fight = uuid::Uuid::new_v4();
        let note = |id: uuid::Uuid, name: &str| {
            format!(
                "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: encounter\nsplat_name: {name}\n---\n\n## Roster\n\n| Side | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n"
            )
        };
        let index = |campaign: &crate::persistence::Campaign| {
            Some(std::sync::Arc::new(
                crate::lookup::ContentIndex::load(&[], Some(campaign)).unwrap(),
            ))
        };
        editor.continue_saved_run(&path, &campaign);
        let run = editor.run_mut().unwrap();
        run.came_from(fight);
        run.content = index(&campaign);
        answer(&mut editor, &mut timeline);
        let words = |editor: &Encounters| {
            editor
                .run()
                .unwrap()
                .unend_refused
                .as_ref()
                .map(|(_, _, refusal)| refusal.words().as_str().to_owned())
        };
        assert!(
            words(&editor).is_some_and(|words| words.contains("its encounter was deleted")),
            "{:?}",
            words(&editor)
        );
        editor.follow_continue_rule(&timeline);
        fs::write(
            dir.path().join("Other fight.md"),
            note(uuid::Uuid::new_v4(), "Other fight"),
        )
        .unwrap();
        editor.run_mut().unwrap().content = index(&campaign);
        editor.follow_continue_rule(&timeline);
        assert!(
            words(&editor).is_some_and(|words| words.contains("its encounter was deleted")),
            "a change to another encounter keeps the answer"
        );
        fs::write(dir.path().join("Ambush.md"), note(fight, "Ambush")).unwrap();
        editor.run_mut().unwrap().content = index(&campaign);
        editor.follow_continue_rule(&timeline);
        assert_eq!(
            words(&editor),
            None,
            "the restored encounter ends the deleted answer, with the rule's key unchanged"
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
            active: Some(campaign.clone()),
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
        // R41: the snapshot holds the typed cause, a folder that is not there; it is said by the
        // footer and the lists, never again as a content issue.
        let snapshot = editor.snapshot.as_ref().unwrap();
        assert!(
            matches!(
                snapshot.runs_unread,
                Some(crate::encounter_run::RunFolderFault::Missing(_))
            ),
            "{:?}",
            snapshot.runs_unread
        );
        assert!(
            !snapshot
                .issues
                .iter()
                .any(|issue| issue.contains("run folder") || issue.contains("Run folder")),
            "{:?}",
            snapshot.issues
        );
        fs::rename(dir.path().join("Moved runs"), &folder).unwrap();
        editor.read_runs_again(&campaign);
        assert_eq!(editor.paused_run_count(), 1);
        assert!(editor.snapshot.as_ref().unwrap().runs_unread.is_none());

        // AB1 (Astra review of PR 259): on a first load, with nothing known about earlier runs,
        // a run folder whose parent cannot be looked in is unread, not "never made"; a folder
        // that is simply not there is still the empty case.
        #[cfg(unix)]
        {
            let aside = dir.path().join("Runs aside");
            fs::rename(&folder, &aside).unwrap();
            let blocker = dir.path().join("Not a folder");
            fs::write(&blocker, "").unwrap();
            std::os::unix::fs::symlink(blocker.join("runs"), &folder).unwrap();
            let snapshot = Snapshot::load(&campaign, &[]).unwrap();
            assert!(
                matches!(
                    snapshot.runs_unread,
                    Some(crate::encounter_run::RunFolderFault::Unreadable { .. })
                ) && snapshot.runs_missing.is_none(),
                "{:?}",
                snapshot.runs_unread
            );
            fs::remove_file(&folder).unwrap();
            let snapshot = Snapshot::load(&campaign, &[]).unwrap();
            assert!(snapshot.runs_unread.is_none() && snapshot.runs_missing.is_some());
            fs::rename(&aside, &folder).unwrap();
        }
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
        let _ = editor
            .draft
            .as_mut()
            .unwrap()
            .body
            .replace("Keep this draft");
        editor.open_saved_run(&path, &campaign);
        assert!(editor.has_run());
        assert!(editor.go_to_run);
        assert!(editor.paused_run().is_none());
        assert_eq!(
            editor.draft.as_ref().unwrap().body.as_str(),
            "Keep this draft"
        );
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
        let summary = |d: &Draft| {
            let snapshot = Snapshot::load(&campaign, &[]).unwrap();
            roster_summary(
                &d.setup,
                &snapshot.catalogue,
                &snapshot.known_creature_names,
            )
            .text()
        };
        assert_eq!(summary(&d), "1 party · Changed");
        slot_override(&mut d, 0, "{side: ' party '}");
        assert_eq!(summary(&d), "1 party · Changed");
        // iter-26-B FL2: the members in a group's form: each with its count, also one, joined
        // by commas, so ` · ` separates facts only; a renamed member names its base.
        let wolf = stat_creature(dir.path(), "Wolf", 11);
        d.add(wolf, false);
        d.add(wolf, false);
        d.add(id, false);
        d.add(wolf, false);
        slot_override(&mut d, 4, "{name: Alpha}");
        assert_eq!(
            summary(&d),
            "1 party · 1 × Alpha (as Wolf), 1 × Rival, 2 × Wolf · Changed"
        );
        // G118: the Ready row's line from the roster as the list loads it. No creature here has
        // a known XP; each is shown, the renamed one too, so none is named after the reason.
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let (catalogue, known) = (&snapshot.catalogue, &snapshot.known_creature_names);
        // This campaign has no party, so the reason is put in by hand, with each foe named as
        // `roster_difficulty` names it (`roster_name`): the renamed wolf is `Alpha`.
        let names: Vec<String> = d
            .setup
            .roster
            .iter()
            .filter(|slot| {
                let side = slot.overrides.get("side").and_then(Value::as_str);
                side.unwrap_or(&slot.side).trim() != "party"
            })
            .map(|slot| {
                let entry = slot
                    .creature
                    .as_ref()
                    .and_then(|reference| catalogue.resolve_ref(reference).ok())
                    .and_then(|id| catalogue.entries.get(&id))
                    .map(|entry| &entry.entry);
                roster_name(slot, entry, known)
            })
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        assert_eq!(names, ["Alpha", "Rival", "Wolf"]);
        let difficulty = crate::difficulty::Difficulty {
            unknown: 4,
            missing: Some(crate::difficulty::Missing {
                what: crate::difficulty::Unknown::Xp,
                at: vec![0, 1, 2, 3],
                names,
            }),
            ..Default::default()
        };
        let parts = roster_summary(&d.setup, catalogue, known);
        let line = |room: usize| {
            crate::difficulty::Line::roster(&difficulty, &parts)
                .fit(|text| text.chars().count() <= room)
                .unwrap()
                .text()
        };
        assert_eq!(
            line(200),
            "1 party · 1 × Alpha (as Wolf), 1 × Rival, 2 × Wolf · Changed · XP not known"
        );
        // Where the summary gave way, the members it no longer shows follow the reason.
        assert_eq!(
            line(56),
            "1 × Alpha (as Wolf), 1 × Rival, … · XP not known: Wolf"
        );
        // R31 groups by structured parts, never printed words (re-check of PR 390): `Bat`
        // renamed `Wolf` and a creature named `Wolf (as Bat)` print alike but stay two members,
        // so where both gave way and `A` stays, the reason names both.
        let mut twins = Draft::new();
        for name in ["A", "Bat", "Wolf (as Bat)"] {
            twins.add(stat_creature(dir.path(), name, 9), false);
        }
        slot_override(&mut twins, 1, "{name: Wolf}");
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let (catalogue, known) = (&snapshot.catalogue, &snapshot.known_creature_names);
        let parts = roster_summary(&twins.setup, catalogue, known);
        assert_eq!(
            parts.text(),
            "1 × A, 1 × Wolf (as Bat), 1 × Wolf (as Bat) · Changed"
        );
        let names: Vec<String> = twins
            .setup
            .roster
            .iter()
            .map(|slot| {
                let entry = slot
                    .creature
                    .as_ref()
                    .and_then(|reference| catalogue.resolve_ref(reference).ok())
                    .and_then(|id| catalogue.entries.get(&id))
                    .map(|entry| &entry.entry);
                roster_name(slot, entry, known)
            })
            .collect();
        let difficulty = crate::difficulty::Difficulty {
            unknown: 3,
            missing: Some(crate::difficulty::Missing {
                what: crate::difficulty::Unknown::Xp,
                at: vec![0, 1, 2],
                names,
            }),
            ..Default::default()
        };
        let line = crate::difficulty::Line::roster(&difficulty, &parts);
        let cut: Vec<String> = (20..80)
            .filter_map(|room| line.fit(|text| text.chars().count() <= room))
            .map(|fitted| fitted.text())
            .filter(|text| text.starts_with("1 × A, …"))
            .collect();
        assert!(
            !cut.is_empty() && cut.iter().all(|text| text.ends_with("and 1 more")),
            "{cut:?}"
        );
        d.setup.roster.truncate(1);
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
