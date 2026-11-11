//! Manual DM controls. These record rulings; they do not execute creature rules.
mod ad_hoc;
mod loot;
pub(crate) mod view;
use crate::encounter_settings::{Grouping, Initiative};
pub(crate) use ad_hoc::RunCreature;
use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_yaml_ng::{Mapping, Value};
use splat::{
    content::{
        Entry,
        overrides::{Base, Overrides, Resolved, condition_names, same_condition},
    },
    execution::dice::{Dice, Random},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::{Duration, Instant},
};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;
pub(crate) use view::{change_preview, change_rows};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Participant {
    pub key: String,
    pub npc: bool,
    pub initiative_group: Option<String>,
    pub initiative_mode: i8,
    pub name: String,
    pub side: String,
    pub definition: Arc<Entry>,
    pub initial: Mapping,
    pub max_hp: Option<i64>,
    pub hp: Option<i64>,
    pub temporary_hp: i64,
    pub armor_class: Option<i64>,
    pub initiative_bonus: Option<i64>,
    pub initiative: Option<i64>,
    /// Participants that received one shared roll keep the same key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_roll: Option<String>,
    pub present: bool,
    pub conditions: Vec<String>,
    pub concentration: bool,
    pub successes: u8,
    pub failures: u8,
    pub notes: String,
    /// The last values D&D Beyond confirmed for this participant, never unsent ones: the next
    /// first read compares the run with these. Older saves may hold unsent values here.
    #[serde(default)]
    pub combat_values: BTreeMap<String, serde_json::Value>,
    /// What this participant began the run with. Until a first read there is no match, so a
    /// difference from these is the DM's. Older saves do not have it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    started: Option<SheetShadow>,
    /// What the encounter or group changed, with the values they replaced. It is frozen here,
    /// so a later edit of the encounter file does not change this run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<Changes>,
    /// The encounter group this participant arrived in, during the run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_from: Option<AddedFrom>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Changes {
    pub base: Base,
    pub overrides: Overrides,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AddedFrom {
    pub group: String,
    /// One addition. The same group added twice is two of these, each with its own roll.
    pub instance: String,
    #[serde(default)]
    pub round: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
}
/// One line of an encounter group, as a run adds it.
#[derive(Clone, Debug)]
pub(crate) struct GroupMember {
    pub entry: Arc<Entry>,
    pub count: u64,
    pub overrides: Overrides,
}
/// A campaign's encounter group, ready to add. Its members are resolved when it loads.
#[derive(Clone, Debug)]
pub(crate) struct RunGroup {
    pub id: Uuid,
    pub path: String,
    pub name: String,
    pub side: String,
    /// The one encounter this group is for. With none, it is for any encounter.
    pub encounter: Option<Uuid>,
    /// That encounter's name, while this campaign still has it.
    pub encounter_name: Option<String>,
    pub members: Vec<GroupMember>,
    pub notes: String,
    /// What this group's creatures are worth. Overrides do not change it.
    pub xp: u64,
    /// The creatures with no XP in their entry, so that the total is not read as complete.
    pub xp_unknown: Vec<String>,
    /// How many creatures the note names that this campaign cannot find.
    pub unavailable: u64,
    /// Missing members retain their position, count and display name for the summary.
    pub unavailable_members: Vec<(usize, u64, String)>,
    /// Every reason this group cannot be added now.
    pub problem: Option<String>,
    /// Unread rows do not block the readable members.
    pub warnings: Vec<String>,
}
impl RunGroup {
    /// The first member's bonus decides a shared roll, as it does for any initiative group.
    pub(crate) fn leader(&self) -> Option<Resolved> {
        let member = self.members.first()?;
        let base = Base::of(&member.entry, &self.side);
        member.overrides.resolve(&base).ok()
    }
    /// How many, on which side, and what they are worth, in one line.
    pub(crate) fn summary(&self) -> String {
        let mut summary = format!(
            "{} · {} · {}",
            crate::design::reader::human_label(&self.side),
            self.count_label(),
            self.xp_label()
        );
        if self
            .members
            .iter()
            .any(|member| member.overrides.changes_creature())
        {
            summary.push_str(" · Changed");
        }
        if self
            .members
            .iter()
            .any(|member| member.overrides.loot.is_some())
        {
            summary.push_str(" · Loot");
        }
        let members = self.member_summary();
        if !members.is_empty() {
            summary.push_str(&format!(" · {members}"));
        }
        summary
    }
    pub(crate) fn joinable_count(&self) -> u64 {
        self.members.iter().map(|member| member.count).sum()
    }
    pub(crate) fn count_label(&self) -> String {
        if self.unavailable_members.is_empty() {
            crate::design::count(self.creature_count() as usize, "member", "members")
        } else {
            format!(
                "{} of {} can join",
                self.joinable_count(),
                self.creature_count()
            )
        }
    }
    pub(crate) fn xp_label(&self) -> String {
        if self.xp_unknown.is_empty() {
            format!("XP {}", grouped_number(self.xp))
        } else if self.xp == 0 && self.members.len() == self.xp_unknown.len() {
            "XP not known".to_owned()
        } else {
            format!("XP {} + unknown", grouped_number(self.xp))
        }
    }
    pub(crate) fn member_summary(&self) -> String {
        let members: Vec<_> = self
            .members
            .iter()
            .map(|member| {
                let base = Base::of(&member.entry, &self.side);
                let name = member
                    .overrides
                    .resolve(&base)
                    .map_or(base.name, |resolved| resolved.name);
                let name = if member.overrides.name.is_some() {
                    format!("{name} (as {})", member.entry.name)
                } else {
                    name
                };
                format!(
                    "{} × {name}",
                    crate::design::count(member.count as usize, "", "").trim()
                )
            })
            .collect();
        let summary = members.join(", ");
        if self.unavailable > 0 {
            format!(
                "Will add: {}",
                if summary.is_empty() { "none" } else { &summary }
            )
        } else {
            summary
        }
    }

    pub(crate) fn member_summary_wrapped(&self) -> String {
        self.member_summary()
            .split(", ")
            .map(|member| member.replace(' ', "\u{a0}"))
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Every creature the note names, whether or not it can be found now.
    pub(crate) fn creature_count(&self) -> u64 {
        self.members
            .iter()
            .map(|member| member.count)
            .fold(self.unavailable, u64::saturating_add)
    }
}
pub(crate) fn grouped_number(number: impl ToString) -> String {
    let text = number.to_string();
    let digits = text.trim_start_matches('-');
    let mut out = if text.starts_with('-') {
        "−".to_owned()
    } else {
        String::new()
    };
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}
/// A campaign's groups, as one run sorts them.
#[derive(Debug, Default)]
pub(crate) struct OfferedGroups {
    pub own: Vec<Arc<RunGroup>>,
    pub any: Vec<Arc<RunGroup>>,
    /// Groups for an encounter, when the run cannot say which encounter it is.
    pub other: Vec<Arc<RunGroup>>,
}
impl OfferedGroups {
    pub(crate) fn of(groups: &[Arc<RunGroup>], encounter: Option<Uuid>) -> Self {
        let mut offered = Self::default();
        for group in groups {
            match group.encounter {
                None => offered.any.push(group.clone()),
                owner if owner == encounter => offered.own.push(group.clone()),
                Some(_) if encounter.is_none() => offered.other.push(group.clone()),
                Some(_) => {}
            }
        }
        offered
    }
}
#[cfg(test)]
fn stat(entry: &Entry, name: &str) -> Option<i64> {
    let stat = entry.fields.get("splat_stats")?.get(name)?;
    stat.get("value").and_then(Value::as_i64)
}
impl Participant {
    pub(super) fn named_stat_block(&self) -> bool {
        self.npc_identity()
            || self.changes.as_ref().is_some_and(|changes| {
                changes
                    .overrides
                    .name
                    .as_ref()
                    .is_some_and(|name| name != &self.definition.name)
            })
    }
    pub(super) fn npc_identity(&self) -> bool {
        self.definition.fields.contains_key("splat_run_npc_name")
    }
    pub(super) fn stat_changes(&self) -> bool {
        self.changes.as_ref().is_some_and(|changes| {
            let mut overrides = changes.overrides.clone();
            if self.npc_identity() {
                overrides.name = None;
            }
            overrides.changes_creature()
        })
    }
    /// Keep existing counts visible even when the participant is not on the party side.
    fn shows_death_saves(&self) -> bool {
        // A linked player sheet makes death saves on any side name the DM chose.
        self.hp == Some(0)
            && (self.side == "party"
                || self.definition.fields.contains_key("splat_external")
                || self.successes > 0
                || self.failures > 0)
    }

    #[cfg(test)]
    pub fn new(
        key: String,
        side: String,
        entry: Entry,
        starting: &Mapping,
        present: bool,
    ) -> Result<Self, String> {
        Self::with_overrides(key, side, entry, starting, &Overrides::default(), present)
    }
    /// `starting` is the slot's own starting state. A changed maximum starts the creature at
    /// that maximum, unless `starting` gives its HP. That value must then fit.
    pub fn with_overrides(
        key: String,
        side: String,
        entry: Entry,
        starting: &Mapping,
        overrides: &Overrides,
        present: bool,
    ) -> Result<Self, String> {
        let mut base = Base::of(&entry, &side);
        let resolved = overrides
            .resolve_for(&entry, &side)
            .map_err(|error| format!("{key} ({}): {error}", entry.name))?;
        let mut initial = entry
            .fields
            .get("splat_initial")
            .and_then(Value::as_mapping)
            .cloned()
            .unwrap_or_default();
        initial.extend(starting.clone());
        let number = |field: &str| -> Result<Option<i64>, String> {
            initial
                .get(field)
                .map(|v| {
                    v.as_i64()
                        .filter(|n| *n >= 0)
                        .ok_or_else(|| format!("{}: invalid starting {field}.", entry.name))
                })
                .transpose()
        };
        let max_hp = resolved.max_hp;
        let hp = if overrides.max_hp.is_some() && !starting.contains_key("hp") {
            max_hp
        } else {
            number("hp")?.or(max_hp)
        };
        if overrides.max_hp.is_some()
            && let (Some(hp), Some(max)) = (hp, max_hp)
            && hp > max
        {
            return Err(format!(
                "{key}: starting HP {hp} is above the changed maximum of {max}."
            ));
        }
        let temporary_hp = number("temporary_hp")?.unwrap_or(0);
        let mut conditions: Vec<String> = initial
            .get("conditions")
            .map(condition_names)
            .unwrap_or_default();
        base.conditions = conditions.clone();
        for condition in &overrides.conditions {
            if !conditions.iter().any(|c| same_condition(c, condition)) {
                conditions.push(condition.clone());
            }
        }
        let counter = |name: &str| {
            initial
                .get("death_saves")
                .and_then(|v| v.get(name))
                .and_then(Value::as_u64)
                .unwrap_or(0)
                .min(3) as u8
        };
        Ok(Self {
            npc: !entry.fields.contains_key("splat_external"),
            initiative_group: None,
            initiative_mode: 0,
            key,
            name: resolved.name,
            side: resolved.side,
            max_hp,
            hp,
            temporary_hp,
            armor_class: resolved.armor_class,
            initiative_bonus: resolved.initiative_bonus,
            initiative: None,
            shared_roll: None,
            present,
            conditions: conditions.clone(),
            concentration: false,
            successes: counter("successes"),
            failures: counter("failures"),
            notes: String::new(),
            combat_values: BTreeMap::new(),
            started: Some(SheetShadow {
                hp,
                temp: temporary_hp,
                successes: counter("successes"),
                failures: counter("failures"),
                conditions,
            }),
            changes: (!overrides.is_empty()).then(|| Changes {
                base,
                overrides: overrides.clone(),
            }),
            added_from: None,
            definition: Arc::new(entry),
            initial,
        })
    }
    /// One creature of an encounter group: at full HP, on the group's side unless it says
    /// otherwise. A run needs known HP once combat begins, so a creature with none is refused.
    fn group_member(
        key: String,
        group: &RunGroup,
        member: &GroupMember,
        instance: &str,
    ) -> Result<Self, String> {
        member
            .overrides
            .resolve_for(&member.entry, &group.side)
            .map_err(|error| format!("{} ({}): {error}", group.name, member.entry.name))?;
        let mut participant = Self::with_overrides(
            key,
            group.side.clone(),
            (*member.entry).clone(),
            &Mapping::new(),
            &member.overrides,
            true,
        )?;
        let max = participant.max_hp.ok_or_else(|| {
            format!(
                "{}: {} has no maximum HP. Set it in this member's overrides.",
                group.name, member.entry.name
            )
        })?;
        participant.hp = Some(max);
        if let Some(started) = &mut participant.started {
            started.hp = Some(max);
        }
        participant.initiative_group = Some(instance.to_owned());
        participant.added_from = Some(AddedFrom {
            group: group.name.clone(),
            instance: instance.to_owned(),
            round: 0,
            group_id: Some(group.id),
        });
        Ok(participant)
    }
    /// Begin from where the last fight left this player, not from their sheet. Returns what
    /// the sheet would have given, if that is different.
    pub fn carry_over(&mut self, carry: &Carry) -> Option<(Option<i64>, i64)> {
        if !self.definition.fields.contains_key("splat_external") {
            return None;
        }
        let from_a_fight = carry.values.get(&self.definition.id).copied();
        let (hp, temp) = match from_a_fight {
            Some(values) => values,
            None if carry.full => (self.max_hp?, 0),
            None => return None,
        };
        let hp = self.max_hp.map_or(hp, |max| hp.min(max));
        let sheet = (self.hp, self.temporary_hp);
        if sheet == (Some(hp), temp) {
            return None;
        }
        self.hp = Some(hp);
        self.temporary_hp = temp;
        // After a fight, what it began with, so that sync reads this as the start and not as the
        // DM's edit: a first read takes the sheet's value, as it does for any value left alone.
        // Full HP from a rest is the DM's ruling, so a sheet that differs is a conflict to settle.
        if from_a_fight.is_some()
            && let Some(started) = &mut self.started
        {
            started.hp = Some(hp);
            started.temp = temp;
        }
        Some(sheet)
    }
    fn health_known(&self) -> bool {
        matches!((self.hp, self.max_hp), (Some(hp), Some(max)) if max > 0 && hp >= 0 && hp <= max)
    }
    /// Ready to begin. Someone who is not here has no roll to make.
    fn ready(&self) -> bool {
        self.health_known() && (self.initiative.is_some() || !self.present)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Phase {
    Setup,
    Running,
    Ended { outcome: String },
}
#[derive(Clone, Serialize, Deserialize)]
struct State {
    participants: Vec<Participant>,
    order: Vec<usize>,
    cursor: usize,
    round: u64,
    phase: Phase,
    log: Vec<String>,
    #[serde(default)]
    last_change: Option<String>,
    random: Random,
    /// What each loot line rolled, by its place: `loot::place`. Saves from before loot have none.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    loot: BTreeMap<String, Vec<loot::Found>>,
}
/// Each carried player's place in the roster, and HP and temporary HP from their sheet.
type SheetValues = Vec<(usize, Option<i64>, i64)>;
/// HP that came from the last fight, not from the sheets.
struct CarriedHp {
    saved: bool,
    from: String,
    sheets: SheetValues,
    /// Fights whose runs could not be read, so their HP is not here.
    missing: Vec<String>,
}
/// The last roll, beside the dice.
struct LastRoll {
    total: String,
    /// The log line behind the total.
    line: String,
}
pub(crate) struct Run {
    pub name: String,
    id: Uuid,
    /// The encounter this run came from. Saves from before this was kept have none.
    source: Option<Uuid>,
    /// Recheck only this encounter when its file metadata changes. Never rebuild the catalogue per frame.
    roster_source: Option<PathBuf>,
    roster_stamp: Option<(Option<std::time::SystemTime>, u64)>,
    roster_faults: Vec<crate::encounters::RosterFault>,
    roster_names: BTreeMap<String, String>,
    roster_names_revision: Option<Uuid>,
    initiative: Initiative,
    storage: Option<Storage>,
    recovery: Recovery,
    /// Each character's cached portrait, by D&D Beyond ID.
    portraits: BTreeMap<String, Option<PathBuf>>,
    portraits_read: Option<Instant>,
    dirty: bool,
    state: State,
    history: Vec<State>,
    /// What each history step did, for a named Undo.
    history_labels: Vec<String>,
    /// Beside each label: true when a sheet made the step, not the DM. Only such a step may
    /// give up its source when Undo is tight.
    history_from_sheet: Vec<bool>,
    selected: usize,
    /// More targets for one amount, beside the selected row. Never saved.
    also: BTreeSet<usize>,
    /// The targets that take half of a damage amount, for a save.
    halved: BTreeSet<usize>,
    /// How many steps the DM has taken or taken back since this opened. A sheet's change is
    /// not one, and neither is opening a run to read it. Never saved.
    dm_steps: usize,
    /// Never saved.
    carried: Option<CarriedHp>,
    rest_order: u64,
    last_roll: Option<LastRoll>,
    /// Digits typed so far. Empty after each use, so a wrong number cannot repeat.
    amount: String,
    condition: String,
    note: String,
    dice: String,
    outcome: String,
    error: Option<String>,
    confirm_close: bool,
    setup_details: String,
    syncs: BTreeMap<usize, crate::combat_sync::CombatSync>,
    sync_shadows: BTreeMap<usize, SheetShadow>,
    panel: Panel,
    closing: bool,
    ending: bool,
    focus_amount: bool,
    roster_height: f32,
    sync_to_restore: Vec<String>,
    /// Sync starts by itself for each linked player, and again after a connection comes back.
    keep_party_synced: bool,
    connected: bool,
    /// Test builds have no session. This lets the party-sync control be exercised.
    simulate_connection: bool,
    scroll_to_selected: bool,
    /// What sync changed. The next run action or sync event clears it.
    notice: Option<String>,
    notice_changes: usize,
    notice_summary: Option<String>,
    live_initiative: Option<usize>,
    /// A value reached the run with no step of its own, so Undo stops there.
    undo_stopped: bool,
    undo_boundary: Option<String>,
    setup_inputs: BTreeMap<usize, String>,
    /// The participant whose initiative the last step set, to merge keystrokes.
    last_initiative: Option<usize>,
    /// An exhaustion drag shows its level and becomes one step when it ends.
    exhaustion_drag: Option<view::ExhaustionDrag>,
    /// Lookup is beside the run, so the run's own panel gives up its space.
    pub hide_panel: bool,
    /// The recording needs a look, shown where the run's notices go, so nothing moves.
    pub recording_notice: Option<&'static str>,
    pub copy_label: Option<String>,
    /// The account's one status function supplies the offline colour too.
    pub account_status: Option<crate::design::Status<'static>>,
    /// Lookup is open. Letters are then the next word of a search, never a command.
    pub lookup_open: bool,
    /// Opened from a save, so there is no undo history to go back through.
    reopened: bool,
    pub content: Option<Arc<crate::lookup::ContentIndex>>,
    /// A link the DM followed in the side panel.
    pub lookup_request: Option<usize>,
    /// The Party page's sheet for the selected participant, when there is one.
    sheet: Option<Arc<splat::character::EntitySnapshot>>,
    sheet_for: Option<usize>,
    sheet_page: usize,
    setup_expanded: Option<usize>,
    focus_field: Option<usize>,
    focus_first_missing: bool,
    /// The panel needs its space back, so Lookup should close.
    pub close_lookup: bool,
    pub wants_sign_in: bool,
    pub wants_packages: bool,
    pub wants_new_group: bool,
    pub wants_group_editor: Option<String>,
    /// A button took the keyboard from the panel beside the run, so it goes back.
    pub return_focus: bool,
    /// Whether the panel beside the run is Notes, where words typed in Amount belong.
    pub notes_open: bool,
    /// Words typed in Amount beside Notes. A letter is never the one number.
    pub for_note: Option<String>,
    /// The end step just opened, and its one field takes the keyboard.
    focus_outcome: bool,
    /// The file changed under this run. Saving stops until the DM chooses what to keep.
    disk_conflict: bool,
    /// The campaign's encounter groups, as they read from disk, once they have been read. A
    /// run does not save these.
    pub groups: Option<Arc<Vec<Arc<RunGroup>>>>,
    /// The campaign's creatures that can join a fight, once they have been read.
    pub creatures: Option<Arc<Vec<Arc<RunCreature>>>>,
    /// The names linked in tonight's plan. Those groups come first in the panel.
    pub planned: Vec<String>,
    ad_hoc: ad_hoc::AdHoc,
    /// The total typed beside each group in the panel, until it is used.
    group_totals: BTreeMap<Uuid, String>,
}
#[derive(Clone, Copy, Default, PartialEq)]
enum Panel {
    #[default]
    Reference,
    Conditions,
    Record,
    Log,
    Encounter,
    Groups,
    Keys,
}
/// How an added group's log line starts. The group panel shows that line while it is the last.
const GROUP_ADDED: &str = "Encounter group ";
/// How one creature's log line starts, with no group to name.
const ONE_ADDED: &str = "Added by hand: ";
/// The source at the start of an Undo step that a sheet made. A tight Undo gives it up before
/// its target.
const SHEET_SOURCE: &str = "D&D Beyond ";
/// What a saved run may hold. A larger one does not open again.
const MAX_PARTICIPANTS: usize = 2000;
/// Where a run's own text ends. Saved runs hold both parts in one string.
pub(crate) const SETUP_HEADING: &str = "## Setup (DM reference)";
const DISK_CONFLICT: &str = "This run changed on disk, perhaps from another copy of Splat or a sync tool. Your run is safe in memory, and saving has stopped until you choose.";
const MAX_FROM_SHEET: &str =
    "Maximum HP comes from the D&D Beyond sheet while sync is on. Stop sync to change it here.";
/// The Undo step a long rest makes when it fills a saved Setup's HP.
const REST_STEP: &str = "full HP after long rest";
/// The Undo step that puts back the HP a Setup had before the rest.
const SAVED_HP_STEP: &str = "use the saved HP";
/// The result of `Use the saved HP`, until the next step.
pub(crate) const SAVED_HP_USED: &str = "Party HP set to the saved values.";
pub(crate) const NOBODY_PRESENT: &str =
    "No participants are present. Mark one present or end the encounter.";
const CLOSE_BLOCKED: &str =
    "Wait for pending sync changes, resolve conflicts, or stop sync for them before closing.";
/// What the one party action does when pressed.
enum PartySync {
    Start(Vec<usize>),
    Stop(Vec<usize>),
}
/// What the run strip shows on other pages.
pub struct Strip {
    pub round: u64,
    pub current: Option<String>,
    pub next: Option<String>,
    pub conflict: Option<String>,
    /// A sync stopped when the session ended.
    pub sync_stopped: bool,
    /// Whose changes a stopped sync has not sent.
    pub unsent: Vec<String>,
    /// A sync reports an error.
    pub sync_error: Option<String>,
    /// The last save failed, so the run is only in memory.
    pub not_saved: bool,
    pub running: bool,
    pub name: String,
}
#[derive(Clone)]
enum Command {
    RollNpcs(Grouping),
    Npc(usize, bool),
    RollMode(usize, i8),
    Begin,
    Next,
    Move(usize, usize),
    MoveArrivalBefore(usize, usize),
    End(String),
    Health(usize, Option<i64>, Option<i64>),
    TempHp(usize, i64),
    Damage(usize, i64),
    Heal(usize, i64),
    Initiative(usize, Option<i64>),
    RollInitiative(usize),
    Bonus(usize, Option<i64>),
    Presence(usize, bool),
    Condition(usize, String),
    RemoveCondition(usize, usize),
    Concentration(usize, bool),
    DeathSaves(usize, u8, u8),
    Note(usize, String),
    /// Add every member of an encounter group, on this total, or by rolling.
    AddGroup(Arc<RunGroup>, Option<i64>),
    /// One line of creatures with no group behind them.
    AddCreature(Arc<RunGroup>, Option<i64>),
    /// One amount on several targets, as one step of undo.
    Several(Vec<Command>),
    /// A roll on a marked table in a note, on this run's dice, with its row in the log.
    TableRoll(crate::roll_table::TableRoll),
    /// Rolls for loot on campaign tables, for the encounter or its creatures, as one step. It
    /// still works once the fight has ended, because that is when the party searches.
    Loot(Vec<loot::LootRoll>),
}
/// A d20 for initiative, with the bonus and the roll mode. Returns the die and the total.
fn roll_initiative(random: &mut Random, bonus: i64, mode: i8) -> Result<(i64, i64), String> {
    let first = i64::from(random.die(20)?);
    let die = if mode == 0 {
        first
    } else {
        let second = i64::from(random.die(20)?);
        if mode > 0 {
            first.max(second)
        } else {
            first.min(second)
        }
    };
    Ok((die, die.checked_add(bonus).ok_or("Initiative overflow.")?))
}
/// The number in a name that is this base name, counting a plain name as the first.
fn number_in_name(name: &str, base: &str) -> Option<u64> {
    if name == base {
        return Some(1);
    }
    // A number this large is part of the name. Nothing can count on from it.
    name.strip_prefix(base)?
        .strip_prefix(' ')?
        .parse()
        .ok()
        .filter(|number: &u64| *number < 1 << 53)
}
/// Numbered names, as the run gives duplicates when it starts. Names already in the fight
/// stay as the table knows them, so new creatures go on from the highest number.
fn number_added(existing: &[Participant], added: &mut [Participant]) {
    let mut counts: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    for p in added.iter() {
        counts.entry(p.name.clone()).or_default().1 += 1;
    }
    let mut used: std::collections::BTreeSet<String> =
        existing.iter().map(|p| p.name.clone()).collect();
    for p in added.iter_mut() {
        let base = p.name.clone();
        let taken = existing
            .iter()
            .filter_map(|other| number_in_name(&other.name, &base))
            .max();
        let (given, total) = counts.get_mut(&base).expect("counted above");
        *given += 1;
        let mut number = taken.unwrap_or(0).saturating_add(*given);
        let mut name = if taken.is_some() || *total > 1 {
            format!("{base} {number}")
        } else {
            base.clone()
        };
        // A name can look like another's number, and a field's id comes from a name.
        while used.contains(&name) && number < u64::MAX {
            number += 1;
            name = format!("{base} {number}");
        }
        used.insert(name.clone());
        p.name = name;
    }
}
impl Run {
    pub fn new(
        name: String,
        mut participants: Vec<Participant>,
        setup_details: String,
    ) -> Result<Self, String> {
        if participants.is_empty() {
            return Err("Add at least one participant before running the encounter.".into());
        }
        let mut counts = BTreeMap::new();
        for p in &participants {
            *counts.entry(p.name.clone()).or_insert(0) += 1;
        }
        // A name that is already unique stays. A number never lands on one, because a field's
        // id comes from a name, and an override can name a creature `Goblin 2`.
        let mut used: std::collections::BTreeSet<String> = counts
            .iter()
            .filter(|(_, count)| **count == 1)
            .map(|(name, _)| name.clone())
            .collect();
        let mut seen = BTreeMap::new();
        for p in &mut participants {
            if counts[&p.name] > 1 {
                let count = seen.entry(p.name.clone()).or_insert(0);
                let mut name;
                loop {
                    *count += 1;
                    name = format!("{} {}", p.name, count);
                    if !used.contains(&name) {
                        break;
                    }
                }
                used.insert(name.clone());
                p.name = name;
            }
        }
        let seed = u64::from_le_bytes(
            crate::hermetic::new_uuid().as_bytes()[..8]
                .try_into()
                .expect("UUID seed"),
        );
        Ok(Self {
            name,
            id: Uuid::new_v4(),
            source: None,
            roster_source: None,
            roster_stamp: None,
            roster_faults: Vec::new(),
            roster_names: BTreeMap::new(),
            roster_names_revision: None,
            initiative: Initiative::default(),
            storage: None,
            recovery: Recovery::default(),
            portraits: BTreeMap::new(),
            portraits_read: None,
            dirty: true,
            state: State {
                order: (0..participants.len()).collect(),
                participants,
                cursor: 0,
                round: 0,
                phase: Phase::Setup,
                log: vec![
                    "DM run prepared from saved encounter. Rules are resolved by the DM.".into(),
                ],
                last_change: None,
                loot: BTreeMap::new(),
                random: Random::seeded(seed),
            },
            history: Vec::new(),
            history_labels: Vec::new(),
            history_from_sheet: Vec::new(),
            selected: 0,
            also: BTreeSet::new(),
            halved: BTreeSet::new(),
            dm_steps: 0,
            carried: None,
            rest_order: 0,
            last_roll: None,
            amount: String::new(),
            condition: String::new(),
            note: String::new(),
            dice: String::new(),
            outcome: String::new(),
            error: None,
            confirm_close: false,
            setup_details,
            syncs: BTreeMap::new(),
            sync_shadows: BTreeMap::new(),
            panel: Panel::Encounter,
            closing: false,
            ending: false,
            focus_amount: false,
            roster_height: 0.0,
            sync_to_restore: Vec::new(),
            keep_party_synced: false,
            connected: false,
            simulate_connection: false,
            scroll_to_selected: false,
            notice: None,
            notice_changes: 0,
            notice_summary: None,
            live_initiative: None,
            recording_notice: None,
            copy_label: None,
            account_status: None,
            undo_stopped: false,
            undo_boundary: None,
            setup_inputs: BTreeMap::new(),
            last_initiative: None,
            exhaustion_drag: None,
            hide_panel: false,
            lookup_open: false,
            reopened: false,
            content: None,
            lookup_request: None,
            close_lookup: false,
            wants_sign_in: false,
            wants_packages: false,
            wants_new_group: false,
            wants_group_editor: None,
            return_focus: false,
            notes_open: false,
            for_note: None,
            focus_outcome: false,
            disk_conflict: false,
            sheet: None,
            sheet_for: None,
            sheet_page: 0,
            setup_expanded: None,
            focus_field: None,
            focus_first_missing: true,
            groups: None,
            creatures: None,
            planned: Vec::new(),
            ad_hoc: ad_hoc::AdHoc::default(),
            group_totals: BTreeMap::new(),
        })
    }
    /// Opened for this fight and closed again with nothing done.
    pub fn untouched_setup(&self) -> bool {
        self.state.phase == Phase::Setup
            && self.history.is_empty()
            && !self.reopened
            && !self.undo_stopped
    }
    /// HP came from the last fight, not from the sheets, and the log says so.
    pub fn note_carry(&mut self, carry: &Carry, sheets: SheetValues) {
        self.rest_order = carry.rest_order;
        // A fight that cannot be read carries nothing, and the log must not say it did.
        if !sheets.is_empty() {
            self.state
                .log
                .push(format!("Party HP carried over from {}.", carry.from));
        }
        self.carried = Some(CarriedHp {
            saved: false,
            from: carry.from.clone(),
            sheets,
            missing: carry.missing.clone(),
        });
        self.dirty = true;
    }
    /// A Setup saved now has already seen this rest; reopening must keep its saved HP.
    pub fn remember_rest(&mut self, order: u64) {
        if self.state.phase == Phase::Setup && order > self.rest_order {
            self.rest_order = order;
            self.dirty = true;
        }
    }

    /// Restore a Setup saved before this rest. Combat keeps its saved values.
    pub fn rest_saved_setup(&mut self, order: u64) {
        if self.state.phase != Phase::Setup || order <= self.rest_order {
            return;
        }
        let before = self.state.clone();
        let mut saved = Vec::new();
        for (index, participant) in self.state.participants.iter_mut().enumerate() {
            if participant.definition.fields.contains_key("splat_external")
                && participant.max_hp.is_some()
            {
                saved.push((index, participant.hp, participant.temporary_hp));
                participant.hp = participant.max_hp;
                participant.temporary_hp = 0;
            }
        }
        self.history.push(before);
        self.history_labels.push(REST_STEP.into());
        self.history_from_sheet.push(false);
        self.state.last_change = Some(REST_STEP.into());
        self.state
            .log
            .push("Party HP is full after the long rest.".into());
        self.carried = Some(CarriedHp {
            saved: true,
            from: "the long rest".into(),
            sheets: saved,
            missing: Vec::new(),
        });
        self.rest_order = order;
        self.dirty = true;
    }
    /// The newest Undo step's name, if there is one.
    fn last_step(&self) -> Option<&str> {
        self.history_labels.last().map(String::as_str)
    }
    /// True while the long rest's full HP is the newest step on a saved Setup.
    fn rest_filled_saved_hp(&self) -> bool {
        self.carried
            .as_ref()
            .is_some_and(|carried| carried.saved && !carried.sheets.is_empty())
            && self.last_step() == Some(REST_STEP)
    }
    /// True while `Use the saved HP` is the newest step, so its result and Undo still apply.
    fn saved_hp_used(&self) -> bool {
        self.carried.as_ref().is_some_and(|carried| carried.saved)
            && self.last_step() == Some(SAVED_HP_STEP)
    }
    /// Put back the HP the Setup had before the rest, as one named step that Undo takes back.
    /// The full HP stays in history, so the DM can return to it.
    fn use_saved_hp(&mut self) -> Result<(), String> {
        if !self.rest_filled_saved_hp() {
            return Err("The long rest's HP is no longer the newest change.".into());
        }
        let mut commands = Vec::new();
        for (index, hp, temp) in self.carried.iter().flat_map(|carried| &carried.sheets) {
            let p = self
                .state
                .participants
                .get(*index)
                .ok_or("A hero with saved HP is no longer in this run.")?;
            commands.push(Command::Health(*index, *hp, p.max_hp));
            commands.push(Command::TempHp(*index, *temp));
        }
        let start = self.state.log.len();
        self.apply(Command::Several(commands))?;
        // One summary names the step before each hero's line. Undo takes the whole state back,
        // so the summary goes with it, like the long rest's own line.
        self.state.log.insert(start, SAVED_HP_USED.into());
        if let Some(label) = self.history_labels.last_mut() {
            *label = SAVED_HP_STEP.into();
        }
        self.state.last_change = Some(SAVED_HP_STEP.into());
        Ok(())
    }
    /// The carried values that still differ from the sheets, as one step back to the sheets.
    fn back_to_sheets(&self) -> Option<(String, Command)> {
        let carried = self.carried.as_ref()?;
        if carried.saved && self.last_step() != Some(REST_STEP) {
            return None;
        }
        let mut commands = Vec::new();
        for (index, hp, temp) in &carried.sheets {
            let p = self.state.participants.get(*index)?;
            // Sync is reading the sheet itself, and these values may be older than that.
            let reading = self.syncs.get(index).is_some_and(|sync| sync.ready());
            if !reading && (p.hp, p.temporary_hp) != (*hp, *temp) {
                commands.push(Command::Health(*index, *hp, p.max_hp));
                commands.push(Command::TempHp(*index, *temp));
            }
        }
        (!commands.is_empty()).then(|| (carried.from.clone(), Command::Several(commands)))
    }
    /// The fights whose HP could not be read, in words.
    fn carry_missing(&self) -> Option<String> {
        let carried = self.carried.as_ref()?;
        (!carried.missing.is_empty()).then(|| {
            format!(
                "Could not read the run of {}, so that fight's HP did not carry over.",
                carried.missing.join(", ")
            )
        })
    }
    /// The read-aloud text and notes, and the setup's technical details, if a run has them.
    fn prep(&self) -> (&str, Option<&str>) {
        match self.setup_details.split_once(SETUP_HEADING) {
            Some((prep, technical)) => (prep.trim(), Some(technical.trim())),
            None => (self.setup_details.trim(), None),
        }
    }
    fn current(&self) -> Option<usize> {
        (self.state.phase == Phase::Running).then(|| self.state.order[self.state.cursor])
    }
    fn sort_initiative(state: &mut State) {
        // Preserve the DM's chosen order for tied totals.
        state
            .order
            .sort_by_key(|i| std::cmp::Reverse(state.participants[*i].initiative));
    }
    /// NPCs with the same key share one `Roll NPCs` roll in this mode.
    fn roll_key(p: &Participant, grouping: Grouping) -> String {
        match grouping {
            // An encounter group that arrived together stays one roll. It joins
            // neither its like outside the group, nor another addition of itself.
            Grouping::Identical => match &p.added_from {
                Some(added) => added.instance.clone(),
                None => format!(
                    "{}:{}:{}:{}",
                    p.definition.id,
                    p.initiative_bonus.map_or(String::new(), |v| v.to_string()),
                    p.initiative_mode,
                    p.changes
                        .as_ref()
                        .map_or_else(String::new, |c| c.overrides.mechanics())
                ),
            },
            Grouping::Individual => p.key.clone(),
            Grouping::BySide => p.side.clone(),
            Grouping::Explicit => p.initiative_group.clone().unwrap_or_else(|| p.key.clone()),
        }
    }
    /// True when this mode gives two or more NPCs one shared roll. Their equal totals then
    /// show tie arrows, so setup can reserve the tie column before the roll.
    pub(crate) fn rolls_a_group(&self, grouping: Grouping) -> bool {
        let mut keys = std::collections::BTreeSet::new();
        self.state
            .participants
            .iter()
            .filter(|p| p.npc)
            .any(|p| !keys.insert(Self::roll_key(p, grouping)))
    }
    /// A shared roll is one block, including its first member.
    fn roll_block_first(state: &State, index: usize) -> usize {
        let p = &state.participants[index];
        state
            .order
            .iter()
            .copied()
            .find(|i| {
                let other = &state.participants[*i];
                *i == index
                    || (p.shared_roll.is_some()
                        && other.shared_roll == p.shared_roll
                        && other.initiative == p.initiative)
            })
            .unwrap_or(index)
    }

    fn roll_block_name(state: &State, p: &Participant) -> String {
        if p.shared_roll.is_some()
            && state
                .participants
                .iter()
                .filter(|other| {
                    other.shared_roll == p.shared_roll && other.initiative == p.initiative
                })
                .count()
                > 1
        {
            if let Some(origin) = &p.added_from {
                return origin.group.clone();
            }
            if p.shared_roll
                .as_deref()
                .is_some_and(|key| key.starts_with("side:"))
            {
                return crate::design::reader::human_label(&p.side);
            }
            return p.definition.name.clone();
        }
        p.name.clone()
    }

    /// A place for someone whose initiative was set during combat. The current turn stays
    /// current, and no one else moves relative to each other.
    /// Whether `a` acts before `b`: by total, then by roster when the campaign breaks ties
    /// that way. Otherwise a tie keeps the order the DM gave it.
    fn goes_before(state: &State, a: usize, b: usize, roster_ties: bool) -> bool {
        let (x, y) = (
            state.participants[a].initiative,
            state.participants[b].initiative,
        );
        x > y || (roster_ties && x == y && a < b)
    }
    /// Whether the row at `at` in the order is where its total puts it, beside the rows
    /// other than `skip`.
    fn in_place(state: &State, at: usize, roster_ties: bool, skip: Option<usize>) -> bool {
        let index = state.order[at];
        let others = |i: &&usize| Some(**i) != skip;
        let after_previous = state.order[..at]
            .iter()
            .rfind(others)
            .is_none_or(|previous| !Self::goes_before(state, index, *previous, roster_ties));
        let before_next = state.order[at + 1..]
            .iter()
            .find(others)
            .is_none_or(|next| !Self::goes_before(state, *next, index, roster_ties));
        after_previous && before_next
    }
    /// The current row whose total changed during its turn. It keeps its place until its
    /// turn ends, so that no one else is skipped or acts twice in this round.
    ///
    /// Only a total out of order says so. A tie is never out of order here: the DM can move
    /// an arrival within a tie, and the campaign's tie rule can change while a run is open.
    /// A row that only sits in another tie order does not wait for its turn to end.
    fn moves_after_turn(state: &State) -> Option<usize> {
        (state.phase == Phase::Running && !Self::in_place(state, state.cursor, false, None))
            .then(|| state.order[state.cursor])
    }
    /// Put a participant where its total goes, and keep the current turn on the same row.
    /// `waiting` is the current row that waits for its turn to end, found before this
    /// change: it is not where its total goes, so the place is found beside the others.
    fn take_place_in_order(
        state: &mut State,
        index: usize,
        roster_ties: bool,
        waiting: Option<usize>,
    ) {
        let current = state.order[state.cursor];
        state.order.retain(|i| *i != index);
        let place = state
            .order
            .iter()
            .position(|i| Some(*i) != waiting && Self::goes_before(state, index, *i, roster_ties))
            .unwrap_or(state.order.len());
        state.order.insert(place, index);
        if let Some(cursor) = state.order.iter().position(|i| *i == current) {
            state.cursor = cursor;
        }
    }
    /// What a total set in combat does to the turn order, in words.
    fn initiative_notice(
        state: &State,
        index: usize,
        total: i64,
        had: bool,
        acted: bool,
        roster_ties: bool,
    ) -> String {
        let name = &state.participants[index].name;
        let at = state.order.iter().position(|i| *i == index);
        if at == Some(state.cursor) {
            if Self::moves_after_turn(state).is_none() {
                return format!("{name} moves to\u{a0}{total}. The turn stays.");
            }
            if !state.participants[index].present {
                return format!("{name} moves to\u{a0}{total} when this turn ends.");
            }
            // Where the row goes when this turn ends, and whether that is in this round.
            let mut after = state.clone();
            let later = Self::end_turn(&mut after, roster_ties).is_ok()
                && after.round == state.round
                && after
                    .order
                    .iter()
                    .position(|i| *i == index)
                    .is_some_and(|at| at >= after.cursor);
            return format!(
                "{name} moves to\u{a0}{total} when this turn ends ·\u{a0}acts {}.",
                if later {
                    "later this round"
                } else {
                    "next round"
                }
            );
        }
        let passed = at.is_some_and(|at| at < state.cursor);
        // An absent row has no turn to speak of until it returns.
        let when = if !state.participants[index].present {
            ""
        } else if passed {
            " ·\u{a0}acts next round"
        } else if acted {
            " ·\u{a0}acts again this round"
        } else {
            ""
        };
        let verb = if had { "moves to" } else { "joins at" };
        format!("{name} {verb}\u{a0}{total}{when}.")
    }
    /// Give the turn to the next present participant. A current row whose total changed in
    /// its turn goes to its new place first: if that is still to come in this round, it acts
    /// again there; if it has passed, it acts there next round.
    fn end_turn(state: &mut State, roster_ties: bool) -> Result<(), String> {
        if !state.participants.iter().any(|p| p.present) {
            return Err(NOBODY_PRESENT.into());
        }
        // The rows still to come in this round start after the current one.
        let mut from = state.cursor + 1;
        if let Some(moved) = Self::moves_after_turn(state) {
            state.order.remove(state.cursor);
            let place = state
                .order
                .iter()
                .position(|i| Self::goes_before(state, moved, *i, roster_ties))
                .unwrap_or(state.order.len());
            state.order.insert(place, moved);
            // A place above the rows still to come pushes them down by one. A place among
            // them is one of them, after its turn here.
            if place > state.cursor {
                from = state.cursor;
            }
        }
        let present = |i: &usize| state.participants[*i].present;
        state.cursor = match state.order[from.min(state.order.len())..]
            .iter()
            .position(present)
        {
            Some(at) => from + at,
            None => {
                state.round = state.round.checked_add(1).ok_or("Round limit reached.")?;
                state.order.iter().position(present).ok_or(NOBODY_PRESENT)?
            }
        };
        Ok(())
    }
    /// The initiative of each new participant, and the words the log uses for it. They are
    /// new, so none of them is in `state` yet.
    fn group_initiative(
        state: &mut State,
        added: &mut [Participant],
        total: Option<i64>,
        grouping: Grouping,
    ) -> Result<String, String> {
        if let Some(total) = total {
            let shared = added[0].key.clone();
            added.iter_mut().for_each(|p| {
                p.initiative = Some(total);
                p.shared_roll = Some(shared.clone());
            });
            return Ok(format!("Initiative {total}, entered by the DM"));
        }
        fn roll(random: &mut Random, p: &Participant) -> Result<(i64, i64, i64), String> {
            let bonus = p.initiative_bonus.ok_or_else(|| {
                format!("{} has no initiative bonus. Type a total instead.", p.name)
            })?;
            let (die, total) = roll_initiative(random, bonus, p.initiative_mode)?;
            Ok((die, bonus, total))
        }
        let mut words = Vec::new();
        match grouping {
            Grouping::Identical | Grouping::Explicit => {
                let (die, bonus, total) = roll(&mut state.random, &added[0])?;
                let key = added[0].key.clone();
                added.iter_mut().for_each(|p| {
                    p.initiative = Some(total);
                    p.shared_roll = Some(key.clone());
                });
                words.push(format!(
                    "Shared roll {die} {} {} = {total}",
                    if bonus < 0 { "−" } else { "+" },
                    bonus.unsigned_abs()
                ));
            }
            Grouping::Individual => {
                for p in added.iter_mut() {
                    let (die, bonus, total) = roll(&mut state.random, p)?;
                    p.initiative = Some(total);
                    p.shared_roll = None;
                    words.push(format!("{}: {die} {bonus:+} = {total}", p.name));
                }
            }
            Grouping::BySide => {
                let mut sides: BTreeMap<String, i64> = BTreeMap::new();
                for p in added.iter_mut() {
                    if !sides.contains_key(&p.side) {
                        let held = state
                            .participants
                            .iter()
                            .filter(|other| other.npc && other.side == p.side)
                            .find_map(|other| other.initiative);
                        let total = match held {
                            Some(total) => {
                                words.push(format!("Joins {} at\u{a0}{total}", p.side));
                                total
                            }
                            None => {
                                let (die, bonus, total) = roll(&mut state.random, p)?;
                                words.push(format!("{} rolled {die} {bonus:+} = {total}", p.side));
                                total
                            }
                        };
                        sides.insert(p.side.clone(), total);
                    }
                    p.initiative = sides.get(&p.side).copied();
                    p.shared_roll = Some(format!("side:{}", p.side));
                }
            }
        }
        Ok(words.join("; "))
    }
    fn join_group(
        state: &mut State,
        group: &RunGroup,
        total: Option<i64>,
        rule: &Initiative,
    ) -> Result<(String, String), String> {
        let grouping = rule.grouping;
        let waiting = Self::moves_after_turn(state);
        if let Some(problem) = &group.problem {
            return Err(format!("{}: {problem}", group.name));
        }
        let instance = format!("group_{}", crate::hermetic::new_uuid().simple());
        let mut added = Vec::new();
        for member in &group.members {
            for _ in 0..member.count {
                let key = format!("{instance}_{}", added.len() + 1);
                added.push(Participant::group_member(key, group, member, &instance)?);
            }
        }
        if added.is_empty() {
            return Err(format!("{} has no members.", group.name));
        }
        if state.participants.len() + added.len() > MAX_PARTICIPANTS {
            return Err(format!(
                "A run can hold {MAX_PARTICIPANTS} participants, and {} would pass that.",
                group.name
            ));
        }
        number_added(&state.participants, &mut added);
        let mut initiative = Self::group_initiative(state, &mut added, total, grouping)?;
        if let Some((p, total)) = added.iter().find_map(|new| {
            let total = new.initiative?;
            state
                .order
                .iter()
                .map(|i| &state.participants[*i])
                .rev()
                .find(|p| p.present && p.initiative == Some(total))
                .map(|p| (p, total))
        }) {
            initiative.push_str(&format!(
                ". Tied with {} at\u{a0}{total}, placed after",
                Self::roll_block_name(state, p)
            ));
        }
        let names: Vec<_> = added.iter().map(|p| p.name.clone()).collect();
        let first = state.participants.len();
        state.participants.extend(added);
        for index in first..state.participants.len() {
            state.order.push(index);
            if state.phase == Phase::Running {
                Self::take_place_in_order(state, index, rule.roster_ties, waiting);
            }
        }
        if state.phase == Phase::Setup {
            Self::sort_initiative(state);
        }
        Ok((names.join(", "), initiative))
    }
    /// The dock's roll. It changes no game state, so it goes to the Log and is not an Undo
    /// step. Undo of an earlier step keeps its line, and the dice do not roll the same again.
    pub(super) fn roll_dice(&mut self, expression: &str) -> Result<(), String> {
        if matches!(self.state.phase, Phase::Ended { .. }) {
            return Err("This encounter has ended. Undo the ending to continue.".into());
        }
        let mut random = self.state.random.clone();
        let roll = Dice::parse(expression)?.roll(&mut random, false)?;
        let line = format!("DM roll {expression}: {:?} = {}.", roll.dice, roll.total);
        for state in self.history.iter_mut().chain([&mut self.state]) {
            state.log.push(line.clone());
            state.random = random.clone();
        }
        self.dm_steps += 1;
        self.dirty = true;
        self.error = None;
        Ok(())
    }
    /// Roll on a table in a note. The run's log keeps the result, and its Undo takes it back.
    pub(crate) fn roll_table(
        &mut self,
        roll: crate::roll_table::TableRoll,
    ) -> Result<String, String> {
        self.apply(Command::TableRoll(roll))?;
        Ok(self.state.log.last().cloned().unwrap_or_default())
    }
    /// An NPC or a group joins the fight from outside the run page.
    pub(crate) fn add_group(&mut self, group: Arc<RunGroup>, total: Option<i64>) {
        if let Err(error) = self.apply(Command::AddGroup(group, total)) {
            self.error = Some(error);
        }
    }
    /// This run's own id, the same in every frame and every save.
    pub(crate) fn id(&self) -> Uuid {
        self.id
    }
    pub(crate) fn ended(&self) -> bool {
        matches!(self.state.phase, Phase::Ended { .. })
    }
    /// The sides in this run, in the order they first appear.
    pub(crate) fn side_names(&self) -> Vec<String> {
        let mut sides = Vec::new();
        for participant in &self.state.participants {
            if !sides.contains(&participant.side) {
                sides.push(participant.side.clone());
            }
        }
        sides
    }
    pub(crate) fn arrivals_in_run(&self) -> BTreeMap<Uuid, (usize, Option<i64>)> {
        self.state
            .participants
            .iter()
            .enumerate()
            .filter_map(|(index, p)| {
                Some((p.added_from.as_ref()?.group_id?, (index, p.initiative)))
            })
            .collect()
    }
    pub(crate) fn select_arrival(&mut self, index: usize) {
        if index < self.state.participants.len() {
            self.select(index);
            self.panel = Panel::Reference;
            self.scroll_to_selected = true;
        }
    }
    fn apply(&mut self, command: Command) -> Result<(), String> {
        if matches!(command, Command::Begin) {
            self.roster_stamp = None;
            self.refresh_roster_faults();
            if let Some(reason) = self.roster_blocked() {
                return Err(reason.into());
            }
        }
        if matches!(self.state.phase, Phase::Ended { .. }) && !matches!(command, Command::Loot(_)) {
            return Err("This encounter has ended. Undo the ending to continue.".into());
        }
        // The sheet decides the maximum while sync reads it, and sync cannot write one. A value
        // that could not stay is refused here, with the reason, not replaced at the next tick.
        if let Command::Health(index, _, max) = &command
            && self
                .syncs
                .get(index)
                .is_some_and(|sync| sync.ready() && sync.max_hp() != *max)
        {
            return Err(MAX_FROM_SHEET.into());
        }
        // Merge typed digits of one initiative into one undo step.
        let initiative_edit = match &command {
            // Typed digits merge only in setup. In combat a total is set once, as one step.
            Command::Initiative(index, _) if self.state.phase == Phase::Setup => Some(*index),
            _ => None,
        };
        if initiative_edit.is_some() && initiative_edit == self.last_initiative {
            self.undo();
        }
        let undo_label = self.describe(&command);
        // Work on a copy so failed commands cannot partly change the run or dice stream.
        let began = matches!(command, Command::Begin);
        let follow_turn = matches!(command, Command::Begin | Command::Next);
        let roster_ties = self.initiative.roster_ties;
        // Who it was, and whether its turn this round had come, before the change.
        let live_initiative = match &command {
            Command::Initiative(index, Some(total)) if self.state.phase == Phase::Running => {
                let had = self
                    .state
                    .participants
                    .get(*index)
                    .is_some_and(|p| p.initiative.is_some());
                let acted = self
                    .state
                    .order
                    .iter()
                    .position(|i| i == index)
                    .is_some_and(|at| at < self.state.cursor);
                Some((*index, *total, had, acted))
            }
            _ => None,
        };
        let mut next = self.state.clone();
        let label = match command {
            Command::RollNpcs(grouping) => {
                if next.phase != Phase::Setup {
                    return Err("Initiative is fixed during combat.".into());
                }
                let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
                let mut group_order = Vec::new();
                for (i, p) in next.participants.iter().enumerate().filter(|(_, p)| p.npc) {
                    let key = Self::roll_key(p, grouping);
                    if !groups.contains_key(&key) {
                        group_order.push(key.clone());
                    }
                    groups.entry(key).or_default().push(i);
                }
                if groups.is_empty() {
                    return Err("No NPCs are marked for initiative.".into());
                }
                let mut results = Vec::new();
                for group in group_order {
                    let members = &groups[&group];
                    let leader = &next.participants[members[0]];
                    let bonus = leader
                        .initiative_bonus
                        .ok_or_else(|| format!("Set an initiative bonus for {}.", leader.name))?;
                    let (die, total) =
                        roll_initiative(&mut next.random, bonus, leader.initiative_mode)?;
                    let names: Vec<_> = members
                        .iter()
                        .map(|i| next.participants[*i].name.clone())
                        .collect();
                    results.push(format!("{}: {die} {bonus:+} = {total}", names.join(", ")));
                    let roll_key = next.participants[members[0]].key.clone();
                    for i in members {
                        next.participants[*i].shared_roll = Some(roll_key.clone());
                        next.participants[*i].initiative = Some(total);
                    }
                }
                if self.initiative.roster_ties {
                    next.order = (0..next.participants.len()).collect();
                }
                Self::sort_initiative(&mut next);
                format!(
                    "NPC initiative — {}. {}",
                    grouping.label(),
                    results.join("; ")
                )
            }
            Command::Begin => {
                if next.phase != Phase::Setup {
                    return Err("Combat has already started.".into());
                }
                if !next.participants.iter().all(Participant::ready) {
                    return Err("Set valid current and maximum HP for each participant, and initiative for every participant.".into());
                }
                if self.initiative.roster_ties {
                    next.order = (0..next.participants.len()).collect();
                }
                Self::sort_initiative(&mut next);
                next.cursor = next
                    .order
                    .iter()
                    .position(|i| next.participants[*i].present)
                    .ok_or("Mark at least one participant present.")?;
                next.round = 1;
                next.phase = Phase::Running;
                format!(
                    "Round 1: {}'s turn.",
                    next.participants[next.order[next.cursor]].name
                )
            }
            Command::Next => {
                if next.phase != Phase::Running {
                    return Err("Begin combat first.".into());
                }
                if !next.participants.iter().any(|p| p.present) {
                    return Err(NOBODY_PRESENT.into());
                }
                Self::end_turn(&mut next, self.initiative.roster_ties)?;
                format!(
                    "Round {}: {}'s turn.",
                    next.round, next.participants[next.order[next.cursor]].name
                )
            }
            Command::MoveArrivalBefore(first, before) => {
                if first >= next.participants.len()
                    || before >= first
                    || !next.participants[before].present
                    || next.participants[first].initiative != next.participants[before].initiative
                {
                    return Err("Choose a present participant with the same initiative.".into());
                }
                let before = Self::roll_block_first(&next, before);
                let target = Self::roll_block_name(&next, &next.participants[before]);
                let current = (next.phase == Phase::Running).then(|| next.order[next.cursor]);
                let waits = next.phase == Phase::Running
                    && next
                        .order
                        .iter()
                        .position(|i| *i == before)
                        .is_some_and(|at| at <= next.cursor);
                let instance = next
                    .participants
                    .get(first)
                    .and_then(|p| p.added_from.as_ref())
                    .map(|a| a.instance.clone())
                    .ok_or("This arrival is no longer here.")?;
                let members: Vec<_> = next
                    .order
                    .iter()
                    .copied()
                    .filter(|i| {
                        next.participants[*i]
                            .added_from
                            .as_ref()
                            .is_some_and(|a| a.instance == instance)
                            && next.participants[*i].initiative
                                == next.participants[before].initiative
                    })
                    .collect();
                next.order.retain(|i| !members.contains(i));
                let at = next
                    .order
                    .iter()
                    .position(|i| *i == before)
                    .ok_or("This participant is no longer here.")?;
                next.order.splice(at..at, members);
                if let Some(current) = current {
                    next.cursor = next
                        .order
                        .iter()
                        .position(|i| *i == current)
                        .unwrap_or(next.cursor);
                }
                format!(
                    "{} moved before {target} at\u{a0}{}{}.",
                    next.participants[first].name,
                    display(next.participants[before].initiative),
                    if waits {
                        " ·\u{a0}acts next round"
                    } else {
                        ""
                    }
                )
            }
            Command::Move(from, to) => {
                if self.initiative.roster_ties {
                    return Err("This campaign resolves initiative ties in roster order.".into());
                }
                if next.phase != Phase::Setup || from >= next.order.len() || to >= next.order.len()
                {
                    return Err("Set turn order before combat begins.".into());
                }
                let a = next.order[from];
                let b = next.order[to];
                if next.participants[a].initiative != next.participants[b].initiative {
                    return Err(
                        "Change initiative totals to move across different initiatives.".into(),
                    );
                }
                next.order.swap(from, to);
                "DM changed tied initiative order.".into()
            }
            Command::End(outcome) => {
                if next.phase != Phase::Running {
                    return Err("Begin combat before ending it.".into());
                }
                next.phase = Phase::Ended {
                    outcome: outcome.clone(),
                };
                format!(
                    "Ended manually in round {}. Outcome: {outcome}.",
                    next.round
                )
            }
            Command::AddGroup(group, total) => {
                let first = next.participants.len();
                let (added, initiative) =
                    Self::join_group(&mut next, &group, total, &self.initiative)?;
                for p in &mut next.participants[first..] {
                    if let Some(from) = &mut p.added_from {
                        from.round = next.round;
                    }
                }
                format!("{GROUP_ADDED}{} added: {added}. {initiative}.", group.name)
            }
            Command::AddCreature(group, total) => {
                let first = next.participants.len();
                let (added, initiative) =
                    Self::join_group(&mut next, &group, total, &self.initiative)?;
                // There is no group to have arrived with.
                for p in &mut next.participants[first..] {
                    if let Some(from) = &mut p.added_from {
                        from.group.clear();
                    }
                }
                format!("{ONE_ADDED}{added}. {initiative}.")
            }
            Command::Several(commands) => {
                let mut lines = commands
                    .into_iter()
                    .map(|command| Self::participant_command(&mut next, command, roster_ties))
                    .collect::<Result<Vec<_>, _>>()?;
                let last = lines.pop().ok_or("Choose a target first.")?;
                next.log.append(&mut lines);
                last
            }
            Command::TableRoll(roll) => roll.resolve(&mut next.random)?,
            Command::Loot(rolls) => {
                let mut lines = loot::roll(&mut next, rolls)?;
                let last = lines.pop().ok_or("Choose a loot roll first.")?;
                next.log.append(&mut lines);
                last
            }
            other => Self::participant_command(&mut next, other, roster_ties)?,
        };
        next.log.push(label);
        next.last_change = Some(Self::change_with_result(&next, &undo_label));
        if self.history.len() == 100 {
            self.history.remove(0);
            self.history_labels.remove(0);
            self.history_from_sheet.remove(0);
        }
        self.dm_steps += 1;
        self.notice = None;
        self.history.push(std::mem::replace(&mut self.state, next));
        if began {
            self.panel = Panel::Reference;
        }
        self.history_labels.push(undo_label);
        self.history_from_sheet.push(false);
        self.notice_summary = None;
        if let Some((index, total, had, acted)) = live_initiative {
            self.notice = Some(Self::initiative_notice(
                &self.state,
                index,
                total,
                had,
                acted,
                roster_ties,
            ));
        }
        self.last_initiative = initiative_edit;
        if follow_turn && let Some(current) = self.current() {
            self.selected = current;
            // Whichever way the turn moved, targets chosen for it no longer stand.
            self.also.clear();
            self.halved.clear();
        }
        self.dirty = true;
        self.error = None;
        Ok(())
    }
    fn participant_command(
        state: &mut State,
        command: Command,
        roster_ties: bool,
    ) -> Result<String, String> {
        // Found before the change, which can make a neighbour of the current row look out of
        // place.
        let waiting = Self::moves_after_turn(state);
        let index = match &command {
            Command::Npc(i, ..)
            | Command::RollMode(i, ..)
            | Command::Health(i, ..)
            | Command::TempHp(i, ..)
            | Command::Damage(i, ..)
            | Command::Heal(i, ..)
            | Command::Initiative(i, ..)
            | Command::RollInitiative(i)
            | Command::Bonus(i, ..)
            | Command::Presence(i, ..)
            | Command::Condition(i, ..)
            | Command::RemoveCondition(i, ..)
            | Command::Concentration(i, ..)
            | Command::DeathSaves(i, ..)
            | Command::Note(i, ..) => *i,
            _ => return Err("Invalid participant command.".into()),
        };
        let p = state
            .participants
            .get_mut(index)
            .ok_or("Participant is missing.")?;
        let change = match command {
            Command::Npc(_, value) => {
                if state.phase != Phase::Setup {
                    return Err("Set NPC control before combat.".into());
                }
                p.npc = value;
                format!("NPC control: {value}.")
            }
            Command::RollMode(_, value) => {
                if state.phase != Phase::Setup || !(-1..=1).contains(&value) {
                    return Err("Set initiative roll mode before combat.".into());
                }
                p.initiative_mode = value;
                format!("Initiative mode: {}.", mode_label(value))
            }
            Command::Health(_, hp, max) => {
                if hp.is_some_and(|n| n < 0)
                    || max.is_some_and(|n| n <= 0)
                    || matches!((hp, max), (Some(h), Some(m)) if h > m)
                    || (state.phase == Phase::Running && (hp.is_none() || max.is_none()))
                {
                    return Err("HP must be between zero and a positive maximum.".into());
                }
                p.hp = hp;
                p.max_hp = max;
                format!("HP set to {}, maximum {}.", display(hp), display(max))
            }
            Command::TempHp(_, amount) => {
                if amount < 0 {
                    return Err("Temporary HP cannot be negative.".into());
                }
                p.temporary_hp = amount;
                format!("Temporary HP replaced with {amount}.")
            }
            Command::Damage(_, amount) => {
                if amount < 0 {
                    return Err("Damage cannot be negative.".into());
                }
                let hp = p.hp.ok_or("Set current HP first.")?;
                let absorbed = amount.min(p.temporary_hp);
                p.temporary_hp -= absorbed;
                p.hp = Some(hp.saturating_sub(amount - absorbed).max(0));
                let taken_by_temp = if absorbed > 0 {
                    format!(" ({absorbed} absorbed by temporary HP)")
                } else {
                    String::new()
                };
                format!(
                    "{amount} damage applied{taken_by_temp}; {} HP remains.",
                    p.hp.unwrap()
                )
            }
            Command::Heal(_, amount) => {
                if amount < 0 {
                    return Err("Healing cannot be negative.".into());
                }
                let hp = p.hp.ok_or("Set current HP first.")?;
                let max = p.max_hp.ok_or("Set maximum HP first.")?;
                p.hp = Some(hp.saturating_add(amount).min(max));
                format!("{amount} healing applied; {} HP remains.", p.hp.unwrap())
            }
            Command::Initiative(_, total) => {
                // In combat, a total can be set or corrected, and never taken away.
                if state.phase != Phase::Setup && total.is_none() {
                    return Err("Enter an initiative total.".into());
                }
                p.initiative = total;
                p.shared_roll = None;
                let change = format!("Initiative set to {}.", display(total));
                if state.phase == Phase::Running {
                    let label = format!("{}: {change}", p.name);
                    let at = state.order.iter().position(|i| *i == index);
                    // The current row keeps its place until its turn ends. Another row moves
                    // only when its total puts it somewhere else, so a tie keeps its order.
                    if at.is_some_and(|at| {
                        at != state.cursor && !Self::in_place(state, at, roster_ties, waiting)
                    }) {
                        Self::take_place_in_order(state, index, roster_ties, waiting);
                    }
                    return Ok(label);
                }
                change
            }
            Command::Bonus(_, bonus) => {
                if state.phase != Phase::Setup {
                    return Err("Initiative is fixed during combat.".into());
                }
                p.initiative_bonus = bonus;
                format!("Initiative bonus set to {}.", display(bonus))
            }
            Command::RollInitiative(_) => {
                if state.phase != Phase::Setup {
                    return Err("Initiative is fixed during combat.".into());
                }
                let bonus = p
                    .initiative_bonus
                    .ok_or("Set an initiative bonus or enter a total.")?;
                let (die, total) = roll_initiative(&mut state.random, bonus, p.initiative_mode)?;
                p.initiative = Some(total);
                p.shared_roll = None;
                format!("Initiative: {die} + {bonus} = {total}.")
            }
            Command::Presence(_, present) => {
                p.present = present;
                if present && p.initiative.is_none() && state.phase != Phase::Setup {
                    "Marked present with no initiative: last in the order until you set it.".into()
                } else if present {
                    "Marked present; retains initiative and state.".into()
                } else {
                    "Marked absent; retains initiative and state.".into()
                }
            }
            Command::Condition(_, condition) => {
                let condition = condition.trim();
                if condition.is_empty() {
                    return Err("Enter a condition name.".into());
                }
                if p.conditions
                    .iter()
                    .any(|c| c.eq_ignore_ascii_case(condition))
                {
                    return Err("That condition is already recorded.".into());
                }
                p.conditions.push(condition.into());
                format!("Condition added: {condition}.")
            }
            Command::RemoveCondition(_, i) => {
                if i >= p.conditions.len() {
                    return Err("Condition is missing.".into());
                }
                format!("Condition removed: {}.", p.conditions.remove(i))
            }
            Command::Concentration(_, value) => {
                p.concentration = value;
                format!(
                    "Concentration {}.",
                    if value { "marked" } else { "cleared" }
                )
            }
            Command::DeathSaves(_, successes, failures) => {
                if successes > 3 || failures > 3 {
                    return Err("Death save counts must be from zero to three.".into());
                }
                p.successes = successes;
                p.failures = failures;
                {
                    let count = |n, single, plural| {
                        if n == 0 {
                            format!("0 {plural}")
                        } else {
                            crate::design::count(n, single, plural)
                        }
                    };
                    format!(
                        "Death saves: {}, {}.",
                        count(usize::from(successes), "success", "successes"),
                        count(usize::from(failures), "failure", "failures")
                    )
                }
            }
            Command::Note(_, note) => {
                if note.trim().is_empty() {
                    return Err("Enter a note.".into());
                }
                if !p.notes.is_empty() {
                    p.notes.push('\n');
                }
                p.notes.push_str(note.trim());
                format!("DM note: {}", note.trim())
            }
            _ => return Err("Invalid participant command.".into()),
        };
        let label = format!("{}: {change}", p.name);
        if state.phase == Phase::Setup {
            Self::sort_initiative(state);
        }
        Ok(label)
    }
    fn undo(&mut self) {
        if let Some(previous) = self.history.pop() {
            self.notice = None;
            self.dm_steps += 1;
            self.history_labels.pop();
            self.history_from_sheet.pop();
            // What D&D Beyond last confirmed is a fact, not a step that can be taken back.
            let confirmed: Vec<_> = self
                .state
                .participants
                .iter_mut()
                .map(|p| std::mem::take(&mut p.combat_values))
                .collect();
            self.state = previous;
            for (p, values) in self.state.participants.iter_mut().zip(confirmed) {
                p.combat_values = values;
            }
            self.forget_removed_participants();
            self.dirty = true;
            self.error = None;
            self.last_initiative = None;
        }
    }
    /// Undo can take away an encounter group, and these all hold a participant's place.
    fn forget_removed_participants(&mut self) {
        let count = self.state.participants.len();
        if self.selected >= count {
            self.selected = self.current().unwrap_or(0);
        }
        self.syncs.retain(|index, _| *index < count);
        self.sync_shadows.retain(|index, _| *index < count);
        self.setup_inputs.retain(|index, _| *index < count);
        for place in [
            &mut self.sheet_for,
            &mut self.setup_expanded,
            &mut self.focus_field,
        ] {
            *place = place.filter(|index| *index < count);
        }
    }
    /// The rules a single NPC roll can use. Initiative groups need a group to be different
    /// from individual rolls.
    fn roll_choices(&self) -> Vec<Grouping> {
        let has_group = self
            .state
            .participants
            .iter()
            .any(|p| p.npc && p.initiative_group.is_some());
        Grouping::ALL
            .into_iter()
            .filter(|mode| *mode != Grouping::Explicit || has_group)
            .collect()
    }
    /// The words a named Undo uses for this command.
    fn describe(&self, command: &Command) -> String {
        let name = |index: &usize| {
            self.state
                .participants
                .get(*index)
                .map_or("a participant", |p| p.name.as_str())
        };
        match command {
            Command::RollNpcs(_) => "NPC initiative roll".into(),
            Command::Npc(i, _) => format!("NPC control on {}", name(i)),
            Command::RollMode(i, _) => format!("roll mode on {}", name(i)),
            Command::Begin => "begin combat".into(),
            Command::Next => "next turn".into(),
            Command::Move(..) => "tie order".into(),
            Command::MoveArrivalBefore(first, before) => format!(
                "move {} before {}",
                name(first),
                Self::roll_block_name(&self.state, &self.state.participants[*before])
            ),
            Command::End(_) => "end encounter".into(),
            Command::Health(i, ..) => format!("HP correction on {}", name(i)),
            Command::TempHp(i, amount) => format!("{amount} temporary HP on {}", name(i)),
            Command::Damage(i, amount) => format!("{amount} damage to {}", name(i)),
            Command::Heal(i, amount) => format!("{amount} healing to {}", name(i)),
            Command::Initiative(i, _) | Command::RollInitiative(i) => {
                format!("initiative for {}", name(i))
            }
            Command::Bonus(i, _) => format!("initiative bonus on {}", name(i)),
            Command::Presence(i, present) => format!(
                "{} {}",
                name(i),
                if *present { "present" } else { "absent" }
            ),
            Command::Condition(i, condition) => format!("{} on {}", condition.trim(), name(i)),
            Command::RemoveCondition(i, at)
                if self
                    .state
                    .participants
                    .get(*i)
                    .and_then(|p| p.conditions.get(*at))
                    .is_some_and(|condition| condition.starts_with("Exhaustion")) =>
            {
                let old = view::exhaustion_level(&self.state.participants[*i].conditions[*at]);
                format!("exhaustion {old} → 0 on {}", name(i))
            }
            Command::RemoveCondition(i, at) => format!(
                "{} off {}",
                self.state
                    .participants
                    .get(*i)
                    .and_then(|p| p.conditions.get(*at))
                    .map_or("an effect", String::as_str),
                name(i)
            ),
            Command::Concentration(i, value) => format!(
                "{} concentration for {}",
                if *value { "start" } else { "stop" },
                name(i)
            ),
            Command::DeathSaves(i, ..) => format!("death saves on {}", name(i)),
            Command::Note(i, _) => format!("note on {}", name(i)),
            Command::AddGroup(group, _) | Command::AddCreature(group, _) => {
                format!("add {}", group.name)
            }
            Command::Several(commands) => {
                let word = match commands.first() {
                    Some(Command::Health(..)) => return "HP from the sheets".to_owned(),
                    Some(Command::RemoveCondition(i, at)) => {
                        let old = self
                            .state
                            .participants
                            .get(*i)
                            .and_then(|p| p.conditions.get(*at));
                        if let Some(old) = old.filter(|old| old.starts_with("Exhaustion")) {
                            let old = view::exhaustion_level(old);
                            let new = commands
                                .iter()
                                .find_map(|command| match command {
                                    Command::Condition(target, condition)
                                        if target == i && condition.starts_with("Exhaustion") =>
                                    {
                                        Some(view::exhaustion_level(condition))
                                    }
                                    _ => None,
                                })
                                .unwrap_or(0);
                            return format!("exhaustion {old} → {new} on {}", name(i));
                        }
                        "a change to".to_owned()
                    }
                    Some(Command::Heal(_, amount)) => format!("{amount} healing to"),
                    Some(Command::TempHp(_, amount)) => format!("{amount} temporary HP on"),
                    Some(Command::Damage(_, amount)) => {
                        // The largest amount, so a half does not misname the step.
                        let most = commands
                            .iter()
                            .filter_map(|command| match command {
                                Command::Damage(_, amount) => Some(*amount),
                                _ => None,
                            })
                            .max()
                            .unwrap_or(*amount);
                        format!("{most} damage to")
                    }
                    _ => "a change to".to_owned(),
                };
                let halves = commands
                    .iter()
                    .filter_map(|command| match command {
                        Command::Damage(_, amount) => Some(*amount),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
                let most = halves.iter().copied().max().unwrap_or(0);
                let half = halves.iter().filter(|amount| **amount < most).count();
                let saved = if half > 0 {
                    format!(", half for {half}")
                } else {
                    String::new()
                };
                format!("{word} {} participants{saved}", commands.len())
            }
            Command::TableRoll(roll) => format!("roll {}", roll.dice),
            // Short, because the ended view's header holds it beside the title.
            Command::Loot(rolls) => loot::describe(rolls),
        }
    }
    /// True when a sheet made the newest Undo step.
    fn undo_from_sheet(&self) -> bool {
        self.history_from_sheet.last().copied().unwrap_or(false)
    }
    fn undo_label(&self) -> String {
        self.history_labels
            .last()
            .map_or_else(|| "Undo".to_owned(), |label| format!("Undo: {label}"))
    }
    fn next_up(&self) -> Option<usize> {
        if self.state.phase != Phase::Running {
            return None;
        }
        let count = self.state.order.len();
        (1..=count)
            .map(|step| self.state.order[(self.state.cursor + step) % count])
            .find(|index| self.state.participants[*index].present)
            .filter(|index| Some(*index) != self.current())
    }
    fn linked_participants(&self) -> Vec<usize> {
        self.state
            .participants
            .iter()
            .enumerate()
            .filter(|(_, p)| remote_identity(p).is_some())
            .map(|(index, _)| index)
            .collect()
    }
    pub fn autosave(&mut self) {
        if self.dirty
            && self.storage.is_some()
            && !self.disk_conflict
            && let Err(e) = self.save()
        {
            self.error = Some(e);
        }
    }
    /// The first participant whose sync is in this state.
    fn first_sync_in(&self, state: crate::combat_sync::SyncState) -> Option<usize> {
        self.syncs
            .iter()
            .find(|(_, sync)| sync.state() == state)
            .map(|(index, _)| *index)
    }
    pub fn strip(&self) -> Strip {
        use crate::combat_sync::SyncState;
        let name = |index: usize| self.state.participants[index].name.clone();
        Strip {
            round: self.state.round,
            current: self.current().map(name),
            next: self.next_up().map(name),
            conflict: self.first_sync_in(SyncState::Conflict).map(name),
            sync_stopped: self
                .syncs
                .values()
                .any(crate::combat_sync::CombatSync::session_lost),
            unsent: self.stranded_names(),
            sync_error: self.first_sync_in(SyncState::Error).map(name),
            not_saved: self.disk_conflict || (self.dirty && self.error.is_some()),
            running: self.state.phase == Phase::Running,
            name: self.name.clone(),
        }
    }
    /// Each player in this fight as they are now, by the character's identity.
    pub fn party_now(&self) -> BTreeMap<Uuid, FightRow> {
        self.state
            .participants
            .iter()
            .filter(|p| is_player(p))
            .map(|p| (p.definition.id, FightRow::from(p)))
            .collect()
    }
    /// What the timeline records about this run.
    pub fn timeline_fact(&self) -> crate::timeline::RunFact {
        crate::timeline::RunFact {
            id: self.id,
            copied_from: self.recovery.copied_from,
            name: self.name.clone(),
            round: self.state.round,
            state: match &self.state.phase {
                Phase::Setup => crate::timeline::RunState::Setup,
                Phase::Running => crate::timeline::RunState::Running,
                Phase::Ended { outcome } => crate::timeline::RunState::Ended(outcome.clone()),
            },
            steps: self.dm_steps,
        }
    }
    /// The strip's Next turn, usable from any page.
    pub fn next_turn(&mut self) {
        if let Err(error) = self.apply(Command::Next) {
            self.error = Some(error);
        } else {
            self.amount.clear();
            self.scroll_to_selected = true;
            self.follow_with_keyboard();
        }
    }
    /// Show the first participant with a conflict, so it can be resolved.
    pub fn review_conflict(&mut self) {
        if let Some(index) = self.first_sync_in(crate::combat_sync::SyncState::Conflict) {
            self.selected = index;
            self.also.clear();
            self.halved.clear();
            self.scroll_to_selected = true;
            // Its controls are in the side panel, which Lookup covers.
            self.close_lookup = true;
        }
    }
    /// The character whose sheet the side panel wants, if the selection is linked.
    pub fn wanted_sheet(&self) -> Option<String> {
        remote_identity(self.state.participants.get(self.selected)?).map(|(_, id)| id)
    }
    /// A sheet the shell found for the selected participant, or none.
    pub fn set_sheet(&mut self, record: Option<&splat::character::EntitySnapshot>) {
        let unchanged = self.sheet_for == Some(self.selected)
            && self.sheet.as_deref().map(|r| &r.source.content_hash)
                == record.map(|r| &r.source.content_hash);
        if !unchanged {
            self.sheet = record.cloned().map(Arc::new);
            self.sheet_for = Some(self.selected);
        }
    }
    fn sync_names(&self, wanted: impl Fn(&crate::combat_sync::CombatSync) -> bool) -> Vec<String> {
        self.syncs
            .iter()
            .filter(|(_, sync)| wanted(sync))
            .filter_map(|(index, _)| self.state.participants.get(*index))
            .map(|p| p.name.clone())
            .collect()
    }
    /// The names of characters with sync changes not yet confirmed.
    pub fn pending_sync_names(&self) -> Vec<String> {
        self.sync_names(crate::combat_sync::CombatSync::unconfirmed)
    }
    pub fn pending_sync(&self) -> bool {
        self.syncs
            .values()
            .any(crate::combat_sync::CombatSync::unconfirmed)
    }
    /// How many participants have a conflict to settle.
    pub(crate) fn conflict_count(&self) -> usize {
        self.syncs
            .values()
            .filter(|sync| sync.state() == crate::combat_sync::SyncState::Conflict)
            .count()
    }
    /// Whose unsent values a close would lose: those the run does not hold itself.
    pub(crate) fn unsent_only_here_names(&self) -> Vec<String> {
        self.sync_names(crate::combat_sync::CombatSync::unsent_only_here)
    }
    /// Who has changes a lost session left unsent.
    pub(crate) fn stranded_names(&self) -> Vec<String> {
        self.sync_names(crate::combat_sync::CombatSync::stranded)
    }
    pub fn path(&self) -> Option<&Path> {
        self.storage.as_ref().map(|storage| storage.path.as_path())
    }
    /// Participants with sync on, to restore after a restart. Sync reads first.
    pub fn synced_keys(&self) -> Vec<String> {
        self.syncs
            .iter()
            .filter(|(_, sync)| sync.enabled)
            .filter_map(|(index, _)| self.state.participants.get(*index))
            .map(|p| p.key.clone())
            .chain(self.sync_to_restore.iter().cloned())
            .collect()
    }
    /// The session comes back after the run does, and sync without one stops itself,
    /// so these wait for the connection.
    pub fn restore_sync(&mut self, keys: &[String]) {
        self.sync_to_restore = keys.to_vec();
    }
    /// The encounter asks for the party to stay synced, or the DM asked for it in this run.
    pub fn keep_party_synced(&mut self) {
        self.keep_party_synced = true;
        self.dirty = true;
    }
    /// A linked player with no sync starts one, and one the lost connection stopped starts
    /// again. A sync the DM stopped for one player stays stopped. Each first read asks before it
    /// replaces a value the DM changed.
    fn start_party_sync(&mut self, linked: Option<&str>) {
        for index in self.linked_participants() {
            let in_campaign = remote_identity(&self.state.participants[index])
                .is_some_and(|(campaign, _)| linked == Some(campaign.as_str()));
            if in_campaign && self.syncs.get(&index).is_none_or(|s| s.session_lost()) {
                self.syncs.entry(index).or_default().start();
            }
        }
    }
    /// Players whose sync the DM stopped. A reopened run must not start these again.
    fn stopped_players(&self) -> Vec<String> {
        self.syncs
            .iter()
            .filter(|(_, sync)| !sync.enabled && !sync.session_lost() && sync.error.is_none())
            .filter_map(|(index, _)| self.state.participants.get(*index))
            .map(|p| p.key.clone())
            .collect()
    }
    fn resume_restored_sync(&mut self) {
        let keys = std::mem::take(&mut self.sync_to_restore);
        for index in self.linked_participants() {
            if keys.contains(&self.state.participants[index].key) {
                self.syncs.entry(index).or_default().start();
            }
        }
    }
}
fn mode_label(mode: i8) -> &'static str {
    match mode {
        -1 => "Disadvantage / surprised",
        1 => "Advantage",
        _ => "Normal roll",
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct SheetShadow {
    hp: Option<i64>,
    temp: i64,
    successes: u8,
    failures: u8,
    conditions: Vec<String>,
}
impl SheetShadow {
    fn of(p: &Participant) -> Self {
        Self {
            hp: p.hp,
            temp: p.temporary_hp,
            successes: p.successes,
            failures: p.failures,
            conditions: p.conditions.clone(),
        }
    }
}
/// How often an open run looks for portraits that have arrived.
const PORTRAIT_REREAD: Duration = Duration::from_secs(30);

fn remote_identity(p: &Participant) -> Option<(String, String)> {
    let external = p.definition.fields.get("splat_external")?;
    if external.get("provider").and_then(Value::as_str) != Some("dndbeyond") {
        return None;
    }
    Some((
        external.get("campaign_id")?.as_str()?.into(),
        external.get("character_id")?.as_str()?.into(),
    ))
}
/// The sheet groups a participant shows. A sheet has others, which the run only keeps.
#[derive(Clone, Copy, PartialEq)]
enum SheetGroup {
    Hp,
    Death,
    Conditions,
}
impl SheetGroup {
    /// Its name in the sheet's values, and so in a saved run.
    fn key(self) -> &'static str {
        match self {
            Self::Hp => "hp",
            Self::Death => "death",
            Self::Conditions => "conditions",
        }
    }
}
/// Show these sheet values on the participant.
fn apply_sheet_values(
    p: &mut Participant,
    values: &BTreeMap<String, serde_json::Value>,
    max_hp: Option<i64>,
) {
    if let Some(max) = max_hp
        && let Some(hp) = values.get(SheetGroup::Hp.key())
    {
        p.max_hp = Some(max);
        p.hp = Some((max - hp["removedHitPoints"].as_i64().unwrap_or(0)).max(0));
        p.temporary_hp = hp["temporaryHitPoints"].as_i64().unwrap_or(0);
    }
    if let Some(death) = values.get(SheetGroup::Death.key()) {
        p.successes = death["successCount"].as_u64().unwrap_or(0).min(3) as u8;
        p.failures = death["failCount"].as_u64().unwrap_or(0).min(3) as u8;
    }
    if let Some(conditions) = values
        .get(SheetGroup::Conditions.key())
        .and_then(serde_json::Value::as_object)
    {
        p.conditions.retain(|c| recognized_condition(c).is_none());
        for (id, name) in crate::combat_sync::CONDITIONS {
            if let Some(level) = conditions.get(&id.to_string()) {
                p.conditions.push(if id == 4 {
                    format!("Exhaustion ({})", level.as_u64().unwrap_or(1))
                } else {
                    name.into()
                });
            }
        }
    }
}

/// Which groups of these sheet values would change what this participant shows.
fn groups_that_differ(
    p: &Participant,
    values: &BTreeMap<String, serde_json::Value>,
    max_hp: Option<i64>,
) -> Vec<SheetGroup> {
    let mut shown = p.clone();
    apply_sheet_values(&mut shown, values, max_hp);
    shadow_differences(&SheetShadow::of(p), &SheetShadow::of(&shown))
}

/// What changed for this participant, from what to what.
fn change_line(
    name: &str,
    before: &SheetShadow,
    after: &SheetShadow,
    maximum: (Option<i64>, Option<i64>),
) -> String {
    let mut parts = Vec::new();
    if maximum.0 != maximum.1 {
        parts.push(format!(
            "maximum HP {} → {}",
            display(maximum.0),
            display(maximum.1)
        ));
    }
    if before.hp != after.hp {
        parts.push(format!("HP {} → {}", display(before.hp), display(after.hp)));
    }
    if before.temp != after.temp {
        parts.push(format!("temp HP {} → {}", before.temp, after.temp));
    }
    if (before.successes, before.failures) != (after.successes, after.failures) {
        parts.push(format!(
            "death saves {} and {} → {} and {}",
            before.successes, before.failures, after.successes, after.failures
        ));
    }
    let official = |shadow: &SheetShadow| -> Vec<String> {
        shadow
            .conditions
            .iter()
            .filter(|c| recognized_condition(c).is_some())
            .cloned()
            .collect()
    };
    let same = |a: &str, b: &str| recognized_condition(a) == recognized_condition(b);
    let (old, new) = (official(before), official(after));
    for gone in old.iter().filter(|c| !new.iter().any(|n| same(c, n))) {
        parts.push(format!("{gone} removed"));
    }
    for added in new.iter().filter(|c| !old.iter().any(|o| same(c, o))) {
        parts.push(format!("{added} added"));
    }
    format!("{name} {}", parts.join(", "))
}

/// Which groups hold different values in these two. Only conditions the sheet knows count, by
/// what they are: `poisoned` and `Poisoned` are one condition.
fn shadow_differences(now: &SheetShadow, then: &SheetShadow) -> Vec<SheetGroup> {
    let sorted = |conditions: &[String]| {
        let mut conditions: Vec<String> = conditions
            .iter()
            .filter_map(|c| recognized_condition(c))
            .map(|(id, level)| format!("{id}:{level}"))
            .collect();
        conditions.sort();
        conditions
    };
    let mut groups = Vec::new();
    if (now.hp, now.temp) != (then.hp, then.temp) {
        groups.push(SheetGroup::Hp);
    }
    if (now.successes, now.failures) != (then.successes, then.failures) {
        groups.push(SheetGroup::Death);
    }
    if sorted(&now.conditions) != sorted(&then.conditions) {
        groups.push(SheetGroup::Conditions);
    }
    groups
}

/// This group as the sheet would hold the participant's values.
fn local_group(
    p: &Participant,
    group: SheetGroup,
    template: &serde_json::Value,
    max_hp: Option<i64>,
) -> Option<serde_json::Value> {
    match group {
        SheetGroup::Hp => Some(serde_json::json!({
            "removedHitPoints": (max_hp? - p.hp?).max(0),
            "temporaryHitPoints": p.temporary_hp,
        })),
        SheetGroup::Death => Some(serde_json::json!({
            "successCount": p.successes,
            "failCount": p.failures,
        })),
        SheetGroup::Conditions => {
            let mut conditions = template.clone();
            let values = conditions.as_object_mut()?;
            for (id, _) in crate::combat_sync::CONDITIONS {
                values.remove(&id.to_string());
            }
            for condition in &p.conditions {
                if let Some((id, level)) = recognized_condition(condition) {
                    values.insert(id.to_string(), level);
                }
            }
            Some(conditions)
        }
    }
}

/// A first read. A value the DM recorded is never replaced without their say: where the run
/// differs from the last value D&D Beyond confirmed, or from where it began if there is none,
/// and the sheet differs too, they choose with the usual conflict controls. With no change,
/// the sheet's values stand.
fn hold_local_changes(p: &mut Participant, sync: &mut crate::combat_sync::CombatSync) {
    let matched = sync.take_matched().filter(|matched| !matched.is_empty());
    let last_read = matched
        .clone()
        .or_else(|| Some(p.combat_values.clone()).filter(|values| !values.is_empty()));
    let changed = match (matched, &p.started) {
        (Some(matched), _) => groups_that_differ(p, &matched, p.max_hp),
        (None, Some(started)) if p.combat_values.is_empty() => {
            shadow_differences(&SheetShadow::of(p), started)
        }
        (None, Some(_)) => groups_that_differ(p, &p.combat_values, p.max_hp),
        // A save from before starting values were kept. Its match can hold unsent values, so
        // nothing there shows what the DM changed. Every difference asks, this once.
        (None, None) => vec![SheetGroup::Hp, SheetGroup::Death, SheetGroup::Conditions],
    };
    if p.started.is_none() {
        p.started = Some(SheetShadow::of(p));
    }
    let sheet = sync.values().clone();
    let max = sync.max_hp();
    for group in groups_that_differ(p, &sheet, max) {
        // Above the sheet's maximum, a value can only be the sheet's own: the notice says so.
        if changed.contains(&group)
            && let Some(local) = sheet
                .get(group.key())
                .and_then(|t| local_group(p, group, t, max))
                .filter(|local| sheet.get(group.key()) != Some(local))
        {
            // Only the run changed, and the sheet cannot be sent to: there is nothing to settle.
            let sheet_same = last_read
                .as_ref()
                .is_some_and(|read| read.get(group.key()) == sheet.get(group.key()));
            if sync.read_only() && sheet_same {
                sync.note_while_stopped(group.key(), local);
            } else {
                sync.hold_conflict(group.key(), local);
            }
        }
    }
}

fn recognized_condition(text: &str) -> Option<(u64, serde_json::Value)> {
    for (id, name) in crate::combat_sync::CONDITIONS {
        if text.eq_ignore_ascii_case(name) {
            return Some((
                id,
                if id == 4 {
                    1.into()
                } else {
                    serde_json::Value::Null
                },
            ));
        }
        if id == 4
            && let Some(level) = text
                .strip_prefix("Exhaustion (")
                .and_then(|s| s.strip_suffix(')'))
                .and_then(|s| s.parse::<u64>().ok())
                .filter(|n| (1..=6).contains(n))
        {
            return Some((id, level.into()));
        }
    }
    None
}
impl Run {
    pub fn tick_sync(
        &mut self,
        ctx: &egui::Context,
        session: Option<crate::auth::Session>,
        epoch: u64,
        linked: Option<&str>,
    ) {
        self.connected = session.is_some();
        if !self.connected {
            self.syncs
                .values_mut()
                .for_each(crate::combat_sync::CombatSync::session_ended);
        }
        if self.connected && !self.sync_to_restore.is_empty() {
            self.resume_restored_sync();
        }
        if matches!(self.state.phase, Phase::Ended { .. }) {
            // What the run recorded is its result. A later change to a sheet is not part of it.
            // A sync with work to finish stays until it is done.
            self.syncs
                .retain(|_, sync| !sync.enabled || sync.unconfirmed());
        } else if self.connected && self.keep_party_synced {
            self.start_party_sync(linked);
        }
        let mut changed_participants = Vec::new();
        let mut sheet_indices = Vec::new();
        let mut identities = std::collections::BTreeSet::new();
        let mut changes = Vec::new();
        let mut from_sheet = Vec::new();
        let mut from_panel = Vec::new();
        let mut choice_indices = Vec::new();
        for (index, sync) in &mut self.syncs {
            if sync.updated {
                self.notice = None;
            }
            let p = &mut self.state.participants[*index];
            let Some((campaign, id)) = remote_identity(p) else {
                continue;
            };
            if linked != Some(campaign.as_str()) {
                sync.stop();
                sync.error = Some(
                    "Link this campaign to the character's D&D Beyond campaign to sync.".into(),
                );
                continue;
            }
            if sync.enabled && !identities.insert(id.clone()) {
                sync.stop();
                sync.error = Some("Another participant is already syncing this character.".into());
                continue;
            }
            // Recorded while sync was stopped: these cannot go yet, and they count.
            if sync.stopped_after_read() {
                let values = sync.values().clone();
                let max = sync.max_hp();
                for group in groups_that_differ(p, &values, max) {
                    if let Some(local) = values
                        .get(group.key())
                        .and_then(|t| local_group(p, group, t, max))
                    {
                        sync.note_while_stopped(group.key(), local);
                    }
                }
            }
            let before = SheetShadow::of(p);
            if sync.ready()
                && let Some(shadow) = self.sync_shadows.get(index)
            {
                if let Some(mut hp) = sync.values().get(SheetGroup::Hp.key()).cloned() {
                    if before.hp != shadow.hp
                        && let (Some(current), Some(max)) = (before.hp, sync.max_hp())
                    {
                        hp["removedHitPoints"] = (max - current).max(0).into();
                    }
                    if before.temp != shadow.temp {
                        hp["temporaryHitPoints"] = before.temp.into();
                    }
                    sync.set_group(SheetGroup::Hp.key(), hp);
                }
                if before.successes != shadow.successes || before.failures != shadow.failures {
                    sync.set_group(SheetGroup::Death.key(),serde_json::json!({"successCount":before.successes,"failCount":before.failures}));
                }
                let group = SheetGroup::Conditions;
                if before.conditions != shadow.conditions
                    && let Some(conditions) = sync
                        .values()
                        .get(group.key())
                        .and_then(|t| local_group(p, group, t, sync.max_hp()))
                {
                    sync.set_group(group.key(), conditions);
                }
            }
            sync.tick(ctx, session.clone(), epoch, &campaign, &id);
            let first_read = sync.take_first_read();
            if first_read {
                hold_local_changes(p, sync);
            }
            if sync.ready() {
                let participant_before = p.clone();
                let maximum_before = p.max_hp;
                apply_sheet_values(p, sync.values(), sync.max_hp());
                let after = SheetShadow::of(p);
                // Anything that reached the run here took no step of its own. Undo would put
                // the older value back and send it, so the steps before it end here.
                let changed =
                    !shadow_differences(&before, &after).is_empty() || maximum_before != p.max_hp;
                if changed {
                    changed_participants.push((*index, participant_before));
                    let line = change_line(&p.name, &before, &after, (maximum_before, p.max_hp));
                    if !sync.chose_remote && (sync.updated || first_read) {
                        sheet_indices.push(*index);
                        from_sheet.push(line);
                    } else {
                        from_panel.push(line);
                        choice_indices.push(*index);
                    }
                }
                // Only what D&D Beyond has confirmed. Saving unsent values here would make
                // them look matched after a close, and the next read would replace them.
                let confirmed = sync.confirmed().cloned().unwrap_or_default();
                if changed || p.combat_values != confirmed {
                    p.combat_values = confirmed;
                    changes.push(p.name.clone());
                }
                self.sync_shadows.insert(*index, after);
                sync.updated = false;
                sync.chose_remote = false;
            } else {
                self.sync_shadows.remove(index);
            }
        }
        if !changes.is_empty() {
            if !from_sheet.is_empty() || !from_panel.is_empty() {
                let mut before_sync = self.state.clone();
                for (index, participant) in changed_participants {
                    before_sync.participants[index] = participant;
                }
                self.reconcile_sync_history(&before_sync, &sheet_indices, &choice_indices);
                self.note_sync_changes(&from_sheet, &from_panel);
            }
            self.dirty = true;
            if self.storage.is_some()
                && let Err(e) = self.save()
            {
                self.error = Some(e);
            }
        }
    }
}
impl Run {
    fn participant_edit(participant: &Participant) -> serde_json::Value {
        let mut value = serde_json::to_value(participant).expect("a saved participant");
        // Read confirmations are facts, not changes made by the history step.
        if let Some(fields) = value.as_object_mut() {
            fields.remove("combat_values");
            fields.remove("started");
        }
        value
    }

    /// Snapshots include the whole run. Drop a step only if it changed this character,
    /// then carry the new sheet values through the snapshots that remain.
    fn reconcile_sync_history(&mut self, before: &State, indices: &[usize], choices: &[usize]) {
        let mut kept = Vec::new();
        let mut labels = Vec::new();
        let mut from_sheet = Vec::new();
        let mut dropped = Vec::new();
        // A multi-target step is atomic. If it also touched another participant, its
        // older steps cannot safely cross this boundary either. Say which steps were lost.
        let mut affected: BTreeSet<usize> = indices.iter().copied().collect();
        loop {
            let old = affected.len();
            for (at, previous) in self.history.iter().enumerate() {
                let next = self.history.get(at + 1).unwrap_or(before);
                let changed: Vec<_> = (0..next.participants.len().max(previous.participants.len()))
                    .filter(|index| {
                        match (
                            previous.participants.get(*index),
                            next.participants.get(*index),
                        ) {
                            (Some(a), Some(b)) => {
                                Self::participant_edit(a) != Self::participant_edit(b)
                            }
                            _ => true,
                        }
                    })
                    .collect();
                if changed.iter().any(|index| affected.contains(index)) {
                    affected.extend(changed);
                }
            }
            if old == affected.len() {
                break;
            }
        }
        let indices: Vec<_> = affected.into_iter().collect();
        for (at, previous) in self.history.iter().enumerate() {
            let next = self.history.get(at + 1).unwrap_or(before);
            let touches = indices.iter().any(|index| {
                match (
                    previous.participants.get(*index),
                    next.participants.get(*index),
                ) {
                    (Some(a), Some(b)) => Self::participant_edit(a) != Self::participant_edit(b),
                    _ => true,
                }
            });
            if touches {
                dropped.push(self.history_labels[at].clone());
                continue;
            }
            let mut previous = previous.clone();
            for index in &indices {
                previous.participants[*index] = self.state.participants[*index].clone();
            }
            kept.push(previous);
            labels.push(self.history_labels[at].clone());
            from_sheet.push(self.history_from_sheet[at]);
        }
        self.history = kept;
        self.history_labels = labels;
        self.history_from_sheet = from_sheet;
        if !dropped.is_empty() {
            let line = format!("Sheet update removed undo steps: {}.", dropped.join("; "));
            self.state.log.push(line.clone());
            for previous in &mut self.history {
                previous.log.push(line.clone());
            }
        }
        if !choices.is_empty() {
            let mut previous = before.clone();
            for index in &indices {
                previous.participants[*index] = self.state.participants[*index].clone();
            }
            self.history.push(previous);
            let label = if let [index] = choices {
                let old = &before.participants[*index];
                let now = &self.state.participants[*index];
                let what = if old.conditions == now.conditions
                    && old.successes == now.successes
                    && old.failures == now.failures
                {
                    "HP"
                } else {
                    "values"
                };
                format!("{SHEET_SOURCE}{what} for {}", now.name)
            } else {
                format!(
                    "{SHEET_SOURCE}values for {}",
                    crate::design::count(choices.len(), "character", "characters")
                )
            };
            self.state.last_change = Some(Self::change_with_result(&self.state, &label));
            self.history_labels.push(label);
            self.history_from_sheet.push(true);
            self.dm_steps += 1;
        }
    }

    /// The log keeps the complete change; the notice names each character's undo boundary.
    fn note_sync_changes(&mut self, from_sheet: &[String], from_panel: &[String]) {
        let mut what = Vec::new();
        if !from_sheet.is_empty() {
            what.push(format!("From D&D Beyond: {}.", from_sheet.join("; ")));
        }
        if !from_panel.is_empty() {
            what.push(format!("In the sync panel: {}.", from_panel.join("; ")));
        }
        let what = what.join(" ");
        let mut changes: Vec<String> = Vec::new();
        for line in from_sheet {
            for (index, part) in line.split(", ").enumerate() {
                if index == 0
                    || self
                        .state
                        .participants
                        .iter()
                        .any(|p| part.starts_with(&p.name))
                {
                    changes.push(part.to_owned());
                } else if let Some(last) = changes.last_mut() {
                    last.push_str(", ");
                    last.push_str(part);
                }
            }
        }
        self.notice_changes = changes.len();
        self.notice_summary = changes.first().map(|first| {
            let more = self.notice_changes.saturating_sub(1);
            let first = self
                .state
                .participants
                .iter()
                .find_map(|participant| {
                    first.strip_prefix(&participant.name).map(|rest| {
                        format!(
                            "{}'s sheet changed:{}",
                            participant.name,
                            rest.trim_start_matches(':')
                        )
                    })
                })
                .unwrap_or_else(|| first.clone());
            if more == 0 {
                first
            } else {
                format!(
                    "{first} and {}",
                    crate::design::count(more, "more sheet", "more sheets")
                )
            }
        });
        self.state.log.push(what.clone());
        for previous in &mut self.history {
            previous.log.push(what.clone());
        }
        let names: Vec<_> = self
            .state
            .participants
            .iter()
            .filter(|p| from_sheet.iter().any(|line| line.starts_with(&p.name)))
            .map(|p| format!("Undo for {} stops here", p.name))
            .collect();
        self.notice = (!from_sheet.is_empty()).then(|| {
            if names.is_empty() {
                what
            } else {
                format!("{what} {}.", names.join(". "))
            }
        });
        self.undo_stopped = true;
        let names: Vec<_> = self
            .state
            .participants
            .iter()
            .filter(|p| from_sheet.iter().any(|line| line.starts_with(&p.name)))
            .map(|p| p.name.as_str())
            .collect();
        self.undo_boundary = Some(match names.as_slice() {
            [name] => format!("Nothing to undo since {name}'s sheet changed"),
            _ => "Nothing to undo since the sheets changed".to_owned(),
        });
    }
    /// A sheet changed outside Splat, with these words for what changed.
    #[cfg(test)]
    pub(crate) fn note_sheet_update(&mut self, lines: &[String]) {
        let before = self.state.clone();
        let indices: Vec<_> = self
            .state
            .participants
            .iter()
            .enumerate()
            .filter(|(_, p)| lines.iter().any(|line| line.starts_with(&p.name)))
            .map(|(at, _)| at)
            .collect();
        self.reconcile_sync_history(&before, &indices, &[]);
        self.note_sync_changes(lines, &[]);
    }
}
#[derive(Clone, Default, Serialize, Deserialize)]
struct Recovery {
    #[serde(default)]
    closed_order: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    copied_from: Option<Uuid>,
    #[serde(default)]
    copy_number: u64,
}

impl Recovery {
    fn number(&self, name: &str) -> u64 {
        let legacy = name
            .split_once(" · saved ")
            .map(|(_, suffix)| {
                suffix
                    .strip_suffix(')')
                    .and_then(|suffix| suffix.rsplit_once(" ("))
                    .and_then(|(_, number)| number.parse().ok())
                    .unwrap_or(1)
            })
            .unwrap_or(0);
        self.copy_number.max(legacy)
    }
}

#[derive(Serialize, Deserialize)]
struct SavedRun {
    #[serde(default)]
    rest_order: u64,
    version: u32,
    #[serde(default)]
    recovery: Recovery,
    campaign: Uuid,
    id: Uuid,
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source: Option<Uuid>,
    initiative: Initiative,
    state: State,
    setup_details: String,
    /// Saves from before this choice load with it off. The DM can start sync from the run.
    #[serde(default)]
    keep_party_synced: bool,
    #[serde(default)]
    stopped_players: Vec<String>,
}
struct Storage {
    path: PathBuf,
    root: PathBuf,
    campaign: Uuid,
    baseline: Option<Vec<u8>>,
}

pub fn find_run(directory: &Path, campaign: Uuid, id: Uuid) -> Result<PathBuf, String> {
    let mut found = Vec::new();
    for file in fs::read_dir(directory).map_err(|e| format!("{}: {e}", directory.display()))? {
        let path = file.map_err(|e| e.to_string())?.path();
        if path.extension().is_some_and(|ext| ext == "json")
            && read_saved(&path, campaign).is_ok_and(|run| run.id == id)
        {
            found.push(path);
        }
    }
    match found.as_slice() {
        [path] => Ok(path.clone()),
        [] => Err(format!(
            "Run {id} was not found in {}.",
            directory.display()
        )),
        _ => Err(format!(
            "Several files use run {id}: {}. Keep one identity before carrying HP.",
            found
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}
impl Run {
    fn refresh_portraits(&mut self) {
        if self
            .portraits_read
            .is_some_and(|read| read.elapsed() < PORTRAIT_REREAD)
        {
            return;
        }
        self.portraits_read = Some(Instant::now());
        let Some(campaign) = self.storage.as_ref().map(|storage| storage.campaign) else {
            return;
        };
        let Ok(cache) = crate::hermetic::character_cache() else {
            return;
        };
        self.portraits = self
            .state
            .participants
            .iter()
            .filter_map(remote_identity)
            .map(|(remote, id)| {
                let portrait = cache.find_portrait(campaign, &remote, &id);
                (id, portrait)
            })
            .collect();
    }
    /// The picture for a participant's row: a character's portrait, or a creature's art.
    fn row_art(&self, ctx: &egui::Context, p: &Participant) -> Option<PathBuf> {
        match remote_identity(p) {
            Some((_, id)) => self.portraits.get(&id).cloned().flatten(),
            None => crate::art::entry_art(ctx, &p.definition),
        }
    }
    /// Shared run-state words for setup, live play and saved rows.
    pub fn state_words(&self, paused: bool) -> String {
        state_words(
            self.state.round,
            paused,
            matches!(self.state.phase, Phase::Ended { .. }),
        )
    }

    fn mark_closed(&mut self) {
        let Some(storage) = &self.storage else {
            return;
        };
        let latest = storage
            .path
            .parent()
            .and_then(|parent| fs::read_dir(parent).ok())
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .filter(|file| file.path().extension().is_some_and(|ext| ext == "json"))
            .filter_map(|file| read_saved(&file.path(), storage.campaign).ok())
            .map(|saved| saved.recovery.closed_order)
            .max()
            .unwrap_or(0);
        self.recovery.closed_order = latest.saturating_add(1);
        self.dirty = true;
    }

    pub fn needs_save(&self) -> bool {
        self.dirty
    }
    /// Say which encounter this run came from. Call it before the first save.
    pub fn came_from(&mut self, encounter: Uuid) {
        self.source = Some(encounter);
    }
    pub fn configure_storage(
        &mut self,
        campaign: Uuid,
        directory: &Path,
        initiative: Initiative,
    ) -> Result<(), String> {
        self.initiative = initiative;
        self.storage = Some(Storage {
            root: directory
                .ancestors()
                .find(|path| path.is_dir())
                .ok_or("Campaign folder not found.")?
                .to_owned(),
            path: directory.join(format!("{}.json", self.id)),
            campaign,
            baseline: None,
        });
        self.save()
    }
    pub fn guard_campaign(&mut self, root: &Path) {
        if let Some(storage) = &mut self.storage {
            storage.root = root.to_owned();
        }
        if self.state.phase == Phase::Setup
            && let Some(source) = self.source
            && let Ok((entries, _)) = splat::content::read_directory(root)
            && let Some(entry) = entries.iter().find(|entry| entry.id == source)
        {
            self.roster_source = Some(root.join(&entry.path));
            for participant in &self.state.participants {
                self.roster_names.insert(
                    participant.definition.id.to_string(),
                    participant.definition.name.clone(),
                );
            }
            for entry in &entries {
                self.roster_names
                    .insert(entry.id.to_string(), entry.name.clone());
                if let Some(key) = &entry.key {
                    self.roster_names.insert(key.clone(), entry.name.clone());
                }
            }
            self.refresh_roster_faults();
        }
    }
    pub(crate) fn set_roster_names(&mut self, revision: Uuid, names: &BTreeMap<String, String>) {
        if self.roster_names_revision != Some(revision) {
            self.roster_names.extend(names.clone());
            self.roster_names_revision = Some(revision);
            self.roster_stamp = None;
        }
    }
    fn refresh_roster_faults(&mut self) {
        let Some(path) = &self.roster_source else {
            return;
        };
        // A moved or deleted encounter file can no longer be repaired here; its old faults
        // must not block Begin. The run's own participants are complete.
        let Ok(metadata) = fs::metadata(path) else {
            self.roster_faults.clear();
            self.roster_stamp = None;
            return;
        };
        let stamp = (metadata.modified().ok(), metadata.len());
        if self.roster_stamp == Some(stamp) {
            return;
        }
        if let Ok(text) = fs::read_to_string(path)
            && let Ok(entry) = Entry::parse(&path.to_string_lossy(), &text)
        {
            self.roster_faults = crate::encounters::roster_faults(&entry, &self.roster_names);
            self.roster_stamp = Some(stamp);
        }
    }
    fn roster_blocked(&self) -> Option<&'static str> {
        (!self.roster_faults.is_empty()).then_some("Fix the unread row first.")
    }
    pub fn save(&mut self) -> Result<(), String> {
        let stopped_players = self.stopped_players();
        let Some(storage) = &mut self.storage else {
            return Err("This run has no save location.".into());
        };
        if !self.dirty {
            return Ok(());
        }
        let saved = SavedRun {
            rest_order: self.rest_order,
            version: 1,
            recovery: self.recovery.clone(),
            campaign: storage.campaign,
            id: self.id,
            name: self.name.clone(),
            source: self.source,
            initiative: self.initiative.clone(),
            state: self.state.clone(),
            setup_details: self.setup_details.clone(),
            keep_party_synced: self.keep_party_synced,
            stopped_players,
        };
        let bytes = serde_json::to_vec_pretty(&saved).map_err(|e| e.to_string())?;
        let parent = storage.path.parent().ok_or("Invalid run location")?;
        if !storage.root.is_dir() {
            return Err(format!(
                "Campaign or run folder is gone: {}. Choose folder before saving.",
                storage.root.display()
            ));
        }
        fs::create_dir_all(parent).map_err(|e| format!("Could not create run folder: {e}"))?;
        let current = match fs::read(&storage.path) {
            Ok(b) => Some(b),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => return Err(e.to_string()),
        };
        if current != storage.baseline {
            self.disk_conflict = true;
            return Err(DISK_CONFLICT.into());
        }
        let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
        temp.write_all(&bytes)
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|e| e.to_string())?;
        // Recheck after staging, just before atomic replacement.
        if fs::read(&storage.path).ok() != storage.baseline {
            self.disk_conflict = true;
            return Err(DISK_CONFLICT.into());
        }
        if storage.baseline.is_some() {
            temp.persist(&storage.path).map_err(|e| e.to_string())?;
        } else {
            temp.persist_noclobber(&storage.path)
                .map_err(|e| e.to_string())?;
        }
        storage.baseline = Some(bytes);
        self.dirty = false;
        Ok(())
    }
    /// The file changed under this run, and the DM keeps this run: it replaces the file.
    pub fn keep_this_run(&mut self) -> Result<(), String> {
        let storage = self
            .storage
            .as_mut()
            .ok_or("This run has no save location.")?;
        storage.baseline = match fs::read(&storage.path) {
            Ok(bytes) => Some(bytes),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => return Err(e.to_string()),
        };
        self.disk_conflict = false;
        self.dirty = true;
        self.error = None;
        self.save()
    }
    /// The file changed under this run, and the DM keeps both: this run gets a new file
    /// beside it, and the changed file stays as it is.
    pub fn save_as_new_run(&mut self) -> Result<(), String> {
        let storage = self
            .storage
            .as_mut()
            .ok_or("This run has no save location.")?;
        let id = crate::hermetic::new_uuid();
        let directory = storage.path.parent().ok_or("Invalid run location")?;
        let now = crate::hermetic::now();
        let time = calendar_time(now);
        let time = time.replace([' ', ':'], "-");
        let path = directory.join(format!("{time}-{:02}-UTC-{id}.json", now % 60));
        let base_name = self
            .name
            .split(" · saved ")
            .next()
            .unwrap_or(&self.name)
            .to_owned();
        let copy_number = fs::read_dir(directory)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
            .filter_map(|file| read_saved(&file.path(), storage.campaign).ok())
            .filter(|saved| saved.name.split(" · saved ").next() == Some(base_name.as_str()))
            .map(|saved| saved.recovery.number(&saved.name))
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        let old_path = std::mem::replace(&mut storage.path, path);
        let old_baseline = storage.baseline.take();
        let old_id = std::mem::replace(&mut self.id, id);
        let old_recovery = self.recovery.clone();
        self.recovery.copied_from = Some(old_id);
        self.recovery.copy_number = copy_number;
        let old_name = std::mem::replace(&mut self.name, base_name);
        self.dirty = true;
        match self.save() {
            Ok(()) => {
                self.disk_conflict = false;
                self.error = None;
                Ok(())
            }
            Err(error) => {
                let storage = self.storage.as_mut().expect("storage was set above");
                storage.path = old_path;
                storage.baseline = old_baseline;
                self.id = old_id;
                self.name = old_name;
                self.recovery = old_recovery;
                self.error = Some(error.clone());
                Err(error)
            }
        }
    }
    /// A run with this set cannot be saved as it stands.
    pub fn has_disk_conflict(&self) -> bool {
        self.disk_conflict
    }
    /// Why the run must stay open. Closing drops the worker, so anything it has not
    /// confirmed would be lost with no message.
    fn close_block(&self) -> Option<&'static str> {
        // Stranded changes warn instead. They cannot leave without a session, and that must
        // not trap the run.
        self.syncs
            .values()
            .any(|sync| sync.unconfirmed() && !sync.stranded())
            .then_some(CLOSE_BLOCKED)
    }
    /// What `Sync party` does now. A running sync is never restarted: starting one drops
    /// its pending writes and any open conflict.
    fn party_sync_plan(&self) -> PartySync {
        let linked = self.linked_participants();
        let on = |i: &usize| self.syncs.get(i).is_some_and(|s| s.enabled);
        let off: Vec<usize> = linked.iter().copied().filter(|i| !on(i)).collect();
        // A sync that stopped itself has said why. It is not started again by this action
        // while others run, or it would stop again at once.
        let startable: Vec<usize> = off
            .iter()
            .copied()
            .filter(|i| {
                self.syncs
                    .get(i)
                    .is_none_or(|s| s.error.is_none() || s.session_lost())
            })
            .collect();
        if !startable.is_empty() {
            PartySync::Start(startable)
        } else if linked.iter().any(&on) {
            PartySync::Stop(linked.into_iter().filter(|i| on(i)).collect())
        } else {
            PartySync::Start(off)
        }
    }
    pub fn open(path: &Path, campaign: Uuid) -> Result<Self, String> {
        if fs::metadata(path).map_err(|e| e.to_string())?.len() > 64 * 1024 * 1024 {
            return Err("Saved run exceeds the size limit.".into());
        }
        let bytes = fs::read(path).map_err(|e| e.to_string())?;
        if bytes.len() > 64 * 1024 * 1024 {
            return Err("Saved run exceeds the size limit.".into());
        }
        let saved: SavedRun =
            serde_json::from_slice(&bytes).map_err(|e| format!("Invalid saved run: {e}"))?;
        if saved.version != 1 || saved.campaign != campaign || saved.id.is_nil() {
            return Err("Unsupported run version or wrong campaign.".into());
        }
        let state = &saved.state;
        let count = state.participants.len();
        let order: std::collections::BTreeSet<_> = state.order.iter().copied().collect();
        if count == 0
            || count > MAX_PARTICIPANTS
            || state.order.len() != count
            || order != (0..count).collect()
            || state.cursor >= count
            || state.participants.iter().any(|p| {
                p.temporary_hp < 0
                    || p.successes > 3
                    || p.failures > 3
                    || !(-1..=1).contains(&p.initiative_mode)
            })
            || (state.phase != Phase::Setup
                && (!state.participants.iter().all(Participant::health_known) || state.round == 0))
        {
            return Err("Saved run state is invalid; its file has not been changed.".into());
        }
        let panel = if saved.state.phase == Phase::Setup {
            Panel::Encounter
        } else {
            Panel::Reference
        };
        let syncs = saved
            .state
            .participants
            .iter()
            .enumerate()
            .filter(|(_, p)| saved.stopped_players.contains(&p.key))
            .map(|(index, _)| (index, crate::combat_sync::CombatSync::default()))
            .collect();
        Ok(Self {
            id: saved.id,
            name: saved.name,
            source: saved.source,
            roster_source: None,
            roster_stamp: None,
            roster_faults: Vec::new(),
            roster_names: BTreeMap::new(),
            roster_names_revision: None,
            recovery: saved.recovery,
            initiative: saved.initiative,
            selected: match saved.state.phase {
                Phase::Running => saved.state.order[saved.state.cursor],
                _ => 0,
            },
            state: saved.state,
            setup_details: saved.setup_details,
            storage: Some(Storage {
                root: path.parent().ok_or("Invalid run location")?.to_owned(),
                path: path.into(),
                campaign,
                baseline: Some(bytes),
            }),
            portraits: BTreeMap::new(),
            portraits_read: None,
            dirty: false,
            history: Vec::new(),
            history_labels: Vec::new(),
            history_from_sheet: Vec::new(),
            also: BTreeSet::new(),
            halved: BTreeSet::new(),
            dm_steps: 0,
            carried: None,
            rest_order: saved.rest_order,
            last_roll: None,
            amount: String::new(),
            condition: String::new(),
            note: String::new(),
            dice: String::new(),
            outcome: String::new(),
            error: None,
            confirm_close: false,
            syncs,
            sync_shadows: BTreeMap::new(),
            panel,
            closing: false,
            ending: false,
            focus_amount: true,
            roster_height: 0.0,
            sync_to_restore: Vec::new(),
            keep_party_synced: saved.keep_party_synced,
            connected: false,
            simulate_connection: false,
            scroll_to_selected: true,
            notice: None,
            notice_changes: 0,
            notice_summary: None,
            live_initiative: None,
            recording_notice: None,
            copy_label: None,
            account_status: None,
            undo_stopped: false,
            undo_boundary: None,
            setup_inputs: BTreeMap::new(),
            last_initiative: None,
            exhaustion_drag: None,
            hide_panel: false,
            lookup_open: false,
            reopened: true,
            content: None,
            lookup_request: None,
            close_lookup: false,
            wants_sign_in: false,
            wants_packages: false,
            wants_new_group: false,
            wants_group_editor: None,
            return_focus: false,
            notes_open: false,
            for_note: None,
            focus_outcome: false,
            disk_conflict: false,
            sheet: None,
            sheet_for: None,
            sheet_page: 0,
            setup_expanded: None,
            focus_field: None,
            focus_first_missing: true,
            groups: None,
            creatures: None,
            planned: Vec::new(),
            ad_hoc: ad_hoc::AdHoc::default(),
            group_totals: BTreeMap::new(),
        })
    }
    pub fn list(directory: &Path, campaign: Uuid) -> Result<Vec<SavedRunInfo>, String> {
        if !directory.exists() {
            return Err(format!("Run folder not found: {}.", directory.display()));
        }
        let mut runs = Vec::new();
        let mut close_order = BTreeMap::new();
        let mut copy_numbers = BTreeMap::new();
        let mut changed_elsewhere = BTreeSet::new();
        for file in fs::read_dir(directory).map_err(|e| e.to_string())? {
            let path = file.map_err(|e| e.to_string())?.path();
            if path.extension().is_some_and(|e| e == "json") {
                let modified = crate::hermetic::file_time(&path);
                match Self::open(&path, campaign) {
                    Ok(run) => {
                        close_order.insert(path.clone(), run.recovery.closed_order);
                        copy_numbers.insert(path.clone(), run.recovery.number(&run.name));
                        if let Some(original) = run.recovery.copied_from {
                            changed_elsewhere.insert(original);
                        }
                        runs.push(run.info(path, modified, true));
                    }
                    Err(_) => {
                        let name = path.file_name().unwrap_or_default().to_string_lossy();
                        runs.push(SavedRunInfo {
                            label: format!("Unreadable run: {name}"),
                            id: None,
                            source: None,
                            encounter: String::new(),
                            standing: Standing::Unreadable,
                            saved_at: modified,
                            unsent: Vec::new(),
                            path,
                        });
                    }
                }
            }
        }
        let after_copy = |run: &SavedRunInfo| {
            copy_numbers
                .get(&run.path)
                .filter(|number| **number > 0)
                .map_or(0, |number| format!("Copy {number} · ").len())
        };
        for run in &mut runs {
            if run.id.is_some_and(|id| changed_elsewhere.contains(&id)) {
                run.label.insert_str(after_copy(run), "Changed on disk · ");
            }
        }
        let mut identities = BTreeMap::<Uuid, Vec<PathBuf>>::new();
        for run in &runs {
            if let Some(id) = run.id {
                identities.entry(id).or_default().push(run.path.clone());
            }
        }
        for run in &mut runs {
            if let Some(paths) = run
                .id
                .and_then(|id| identities.get(&id))
                .filter(|paths| paths.len() > 1)
            {
                run.label = format!(
                    "Duplicate identity: {} · {}",
                    paths
                        .iter()
                        .map(|path| path.display().to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    run.label
                );
            }
        }
        // The run closed last owns Resume, including recovery copies.
        runs.sort_by(|a, b| {
            close_order
                .get(&b.path)
                .cmp(&close_order.get(&a.path))
                .then_with(|| b.saved_at.cmp(&a.saved_at))
                .then_with(|| a.encounter.cmp(&b.encounter))
                .then_with(|| a.label.cmp(&b.label))
                .then_with(|| a.path.cmp(&b.path))
        });
        if let Some(run) = runs
            .iter_mut()
            .find(|run| run.standing == Standing::Unfinished)
            && close_order.get(&run.path).is_some_and(|order| *order > 0)
        {
            let duplicate = run
                .id
                .and_then(|id| identities.get(&id))
                .is_some_and(|paths| paths.len() > 1);
            let at = if duplicate { 0 } else { after_copy(run) };
            run.label.insert_str(at, "Closed last · ");
        }
        let all_recent = runs
            .iter()
            .filter(|run| run.standing == Standing::Unfinished)
            .all(|run| run.label.contains(" · Saved just now"));
        if all_recent {
            for run in &mut runs {
                if run.standing == Standing::Unfinished {
                    run.label = run.label.replace(" · Saved just now", "");
                }
            }
        }
        Ok(runs)
    }
    /// This run as a row on the Encounters page, as it is now and not as its file has it.
    pub fn live_info(&self) -> Option<SavedRunInfo> {
        let path = self.path()?.to_path_buf();
        let saved_at = crate::hermetic::file_time(&path);
        Some(self.info(path, saved_at, false))
    }
    fn change_with_result(state: &State, label: &str) -> String {
        state
            .participants
            .iter()
            .find(|p| label.ends_with(&format!("damage to {}", p.name)))
            .and_then(|p| p.hp)
            .map_or_else(|| label.to_owned(), |hp| format!("{label} ({hp} HP left)"))
    }

    fn info(&self, path: PathBuf, saved_at: Option<u64>, paused: bool) -> SavedRunInfo {
        let saved = saved_at.map_or_else(String::new, |at| {
            format!(
                " · Saved {}",
                crate::party::fetch_age(crate::hermetic::now().saturating_sub(at))
            )
        });
        let standing = if matches!(self.state.phase, Phase::Ended { .. }) {
            Standing::Ended
        } else {
            Standing::Unfinished
        };
        let mut label = self.state_words(paused);
        if let Phase::Ended { outcome } = &self.state.phase {
            if !outcome.trim().is_empty() {
                label = format!("{outcome} · {label}");
            }
        } else if paused
            && let Some(last) = self
                .state
                .last_change
                .as_ref()
                .or_else(|| self.state.log.last())
        {
            let round_prefix = format!("Round {}: ", self.state.round);
            let last = last.strip_prefix(&round_prefix).unwrap_or(last);
            label.push_str(&format!(" · Last: {last}"));
        }
        label.push_str(&saved);
        let legacy = self.name.split_once(" · saved ");
        let copy_number = self.recovery.number(&self.name);
        if copy_number > 0 {
            label.insert_str(0, &format!("Copy {copy_number} · "));
        }
        SavedRunInfo {
            path,
            id: Some(self.id),
            source: self.source,
            encounter: legacy
                .map_or(self.name.as_str(), |(name, _)| name)
                .to_owned(),
            label,
            standing,
            saved_at,
            unsent: self.unsent_in_save(),
        }
    }
    /// Who holds values that D&D Beyond did not confirm, in a run that keeps the party matched.
    /// With its file gone, nothing else has these values.
    fn unsent_in_save(&self) -> Vec<String> {
        if !self.keep_party_synced {
            return Vec::new();
        }
        self.state
            .participants
            .iter()
            .filter(|p| remote_identity(p).is_some())
            // As a first read would judge it. Before any match, a difference from the start
            // is the DM's.
            .filter(|p| match &p.started {
                Some(started) if p.combat_values.is_empty() => {
                    !shadow_differences(&SheetShadow::of(p), started).is_empty()
                }
                _ => {
                    !p.combat_values.is_empty()
                        && !groups_that_differ(p, &p.combat_values, p.max_hp).is_empty()
                }
            })
            .map(|p| p.name.clone())
            .collect()
    }
}

/// Where a saved run stands, as the Encounters page sorts it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Standing {
    Unfinished,
    Ended,
    /// The file is not a run this campaign can open. It can belong to another campaign that
    /// shares the folder, so the page offers nothing that would change it.
    Unreadable,
}

/// One saved run, as the Encounters page lists it.
#[derive(Clone)]
pub struct SavedRunInfo {
    pub path: PathBuf,
    pub id: Option<Uuid>,
    /// The encounter it came from, if the save says.
    pub source: Option<Uuid>,
    pub encounter: String,
    pub label: String,
    pub standing: Standing,
    pub saved_at: Option<u64>,
    /// Who holds values not sent to D&D Beyond.
    pub unsent: Vec<String>,
}

/// A run file the DM deleted, kept in memory for one step of undo.
pub struct DeletedRun {
    path: PathBuf,
    bytes: Vec<u8>,
    saved_at: Option<std::time::SystemTime>,
    pub encounter: String,
    pub unsent: Vec<String>,
}

impl DeletedRun {
    pub const UNDO_LABEL: &str = "Undo delete run";
    /// One deletion warning on every page, including the way to keep unsent changes.
    pub fn message(&self) -> String {
        let name = &self.encounter;
        if self.unsent.is_empty() {
            format!("Deleted the saved run of {name}.")
        } else {
            format!(
                "Deleted the saved run of {name}. It held changes for {} that D&D Beyond does not have. Undo to keep them.",
                self.unsent.join(", ")
            )
        }
    }

    /// Remove the run's file.
    pub fn delete(info: &SavedRunInfo) -> Result<Self, String> {
        let bytes = fs::read(&info.path).map_err(|e| e.to_string())?;
        let saved_at = fs::metadata(&info.path)
            .and_then(|metadata| metadata.modified())
            .ok();
        fs::remove_file(&info.path).map_err(|e| e.to_string())?;
        Ok(Self {
            path: info.path.clone(),
            bytes,
            saved_at,
            encounter: info.encounter.clone(),
            unsent: info.unsent.clone(),
        })
    }
    /// Put the file back as it was. A file that has since appeared there stays.
    pub fn restore(&self) -> Result<(), String> {
        let parent = self.path.parent().ok_or("Invalid run location")?;
        let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
        temp.write_all(&self.bytes)
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|e| e.to_string())?;
        if let Some(at) = self.saved_at {
            temp.as_file().set_modified(at).map_err(|e| e.to_string())?;
        }
        temp.persist_noclobber(&self.path)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// Where the party stands between fights in one session.
#[derive(Clone, Debug, PartialEq)]
pub struct Carry {
    pub rest_order: u64,
    /// The fight or the rest it comes from, in words.
    pub from: String,
    /// A long rest the DM said restored everyone. A later fight's values still come first.
    pub full: bool,
    /// HP and temporary HP, by the character's identity.
    pub values: BTreeMap<Uuid, (i64, i64)>,
    /// Fights whose runs could not be read.
    pub missing: Vec<String>,
}

/// One fight after the fact: how each participant ended, and what the log says.
pub struct FightSummary {
    /// Combat ended, so these are the final values. A run can also be closed mid-fight.
    pub ended: bool,
    pub end_reason: String,
    pub rows: Vec<FightRow>,
    pub log: Vec<String>,
}

pub struct FightRow {
    pub name: String,
    pub player: bool,
    pub hp: Option<i64>,
    pub max_hp: Option<i64>,
    pub temporary_hp: i64,
    pub conditions: Vec<String>,
    pub present: bool,
}

/// A player's character, rather than a creature.
fn is_player(p: &Participant) -> bool {
    p.definition.fields.contains_key("splat_external")
}

impl From<&Participant> for FightRow {
    fn from(p: &Participant) -> Self {
        Self {
            name: p.name.clone(),
            player: is_player(p),
            hp: p.hp,
            max_hp: p.max_hp,
            temporary_hp: p.temporary_hp,
            conditions: p.conditions.clone(),
            present: p.present,
        }
    }
}

/// Check the same saved-run model as normal loading, without opening a campaign.
pub(crate) fn check_saved_text(text: &str) -> Result<(), String> {
    let saved: SavedRun =
        serde_json::from_str(text).map_err(|e| format!("Invalid saved run: {e}"))?;
    if saved.version != 1 {
        return Err("Unsupported run version.".into());
    }
    Ok(())
}

/// A saved run's file, within the size limit.
fn read_saved(path: &Path, campaign: Uuid) -> Result<SavedRun, String> {
    if fs::metadata(path).map_err(|e| e.to_string())?.len() > 64 * 1024 * 1024 {
        return Err("Saved run exceeds the size limit.".into());
    }
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let saved: SavedRun =
        serde_json::from_slice(&bytes).map_err(|e| format!("Invalid saved run: {e}"))?;
    if saved.version != 1 || saved.campaign != campaign {
        return Err("Unsupported run version or wrong campaign.".into());
    }
    Ok(saved)
}

/// How the players stood when this run was last saved.
pub fn party_at_end(path: &Path, campaign: Uuid) -> Result<BTreeMap<Uuid, (i64, i64)>, String> {
    Ok(read_saved(path, campaign)?
        .state
        .participants
        .iter()
        .filter(|p| is_player(p))
        .filter_map(|p| Some((p.definition.id, (p.hp?, p.temporary_hp))))
        .collect())
}

/// A finished or interrupted fight, to read in place.
pub fn fight_summary(path: &Path, campaign: Uuid) -> Result<FightSummary, String> {
    let state = read_saved(path, campaign)?.state;
    Ok(FightSummary {
        ended: matches!(state.phase, Phase::Ended { .. }),
        end_reason: "Ended manually".to_owned(),
        rows: state
            .order
            .iter()
            .filter_map(|index| state.participants.get(*index))
            .map(FightRow::from)
            .collect(),
        log: state.log.iter().rev().cloned().collect(),
    })
}

/// One vocabulary for every place a run's state is shown. Round zero means setup.
pub fn state_words(round: u64, paused: bool, ended: bool) -> String {
    if ended {
        format!("Ended · Round {}", round.max(1))
    } else {
        let state = if round == 0 {
            "Setup".to_owned()
        } else {
            format!("Round {round}")
        };
        if paused {
            format!("{state} · Paused")
        } else {
            state
        }
    }
}

/// Recent saves use the same age words as sheets. Older saves use a short UTC date.
pub(crate) fn saved_time(at: u64, now: u64) -> String {
    if now / 86_400 <= at / 86_400 + 1 {
        crate::party::fetch_age(now.saturating_sub(at))
    } else {
        calendar_date(at)
    }
}

/// A UTC date and time, so two runs saved on one day can be told apart.
pub fn calendar_time(seconds: u64) -> String {
    format!(
        "{} {:02}:{:02}",
        calendar_date(seconds),
        (seconds / 3600) % 24,
        (seconds / 60) % 60
    )
}

/// A UTC date from seconds since 1970, by Howard Hinnant's `civil_from_days`.
pub fn calendar_date(seconds: u64) -> String {
    let days = (seconds / 86_400) as i64 + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

#[cfg(test)]
impl Run {
    /// A participant's name, maximum HP and presence, for a test of what reached the run.
    pub fn participant_stats(&self, index: usize) -> (String, Option<i64>, bool) {
        let p = &self.state.participants[index];
        (p.name.clone(), p.max_hp, p.present)
    }
    pub fn keeps_party_synced(&self) -> bool {
        self.keep_party_synced
    }
    pub fn hp_of(&self, index: usize) -> Option<i64> {
        self.state.participants[index].hp
    }
    pub fn simulate_connection(&mut self) {
        self.simulate_connection = true;
    }

    /// The live empty-history state before any reopen or sheet-update boundary.
    pub fn simulate_fresh_live_history(&mut self) {
        assert_eq!(self.state.phase, Phase::Running);
        self.history.clear();
        self.history_labels.clear();
        self.history_from_sheet.clear();
        self.reopened = false;
        self.undo_stopped = false;
    }
    pub fn simulate_rename(&mut self, name: &str, to: &str) {
        let participant = self
            .state
            .participants
            .iter_mut()
            .find(|p| p.name == name)
            .expect("participant in this run");
        participant.name = to.to_owned();
    }
    /// A read of this sheet with the damage it had at the last match, plus `more_damage`. Test
    /// builds have no live sheet, so this goes through the same comparison that a live read does.
    pub fn simulate_read(&mut self, name: &str, more_damage: i64) {
        let (index, participant) = self
            .state
            .participants
            .iter()
            .enumerate()
            .find(|(_, p)| p.name == name)
            .expect("participant in this run");
        let (_, character) = remote_identity(participant).expect("D&D Beyond participant");
        let max_hp = participant.max_hp.unwrap_or(1);
        let damage = participant
            .combat_values
            .get("hp")
            .and_then(|hp| hp["removedHitPoints"].as_i64())
            .unwrap_or(0);
        let sync = self.syncs.get_mut(&index).expect("a running sync");
        sync.stop_simulating();
        sync.simulate_read(
            character.parse().expect("numeric ID"),
            max_hp,
            damage + more_damage,
        );
    }
    pub fn stop_simulating_sync(&mut self) {
        self.syncs
            .values_mut()
            .for_each(crate::combat_sync::CombatSync::stop_simulating);
    }
    pub fn simulate_error(&mut self, message: &str) {
        self.error = (!message.is_empty()).then(|| message.to_owned());
    }
    /// Show one participant in a sync state test builds cannot reach by themselves.
    pub fn simulate_sync_state(&mut self, name: &str, state: crate::combat_sync::SyncState) {
        let (index, participant) = self
            .state
            .participants
            .iter()
            .enumerate()
            .find(|(_, p)| p.name == name)
            .expect("participant in this run");
        let (_, character) = remote_identity(participant).expect("D&D Beyond participant");
        let max_hp = participant.max_hp.unwrap_or(1);
        let damage = max_hp - participant.hp.unwrap_or(max_hp);
        self.syncs.insert(
            index,
            crate::combat_sync::CombatSync::simulated_state(
                state,
                character.parse().expect("numeric ID"),
                max_hp,
                damage,
            ),
        );
    }
    /// Give one participant a D&D Beyond conflict, as the sync panel would show it.
    pub fn simulate_sync_conflict(&mut self, name: &str) {
        let (index, participant) = self
            .state
            .participants
            .iter()
            .enumerate()
            .find(|(_, p)| p.name == name)
            .expect("participant in this run");
        let (_, character) = remote_identity(participant).expect("D&D Beyond participant");
        let max_hp = participant.max_hp.unwrap_or(1);
        let damage = max_hp - participant.hp.unwrap_or(max_hp);
        self.syncs.insert(
            index,
            crate::combat_sync::CombatSync::simulated_conflict(
                character.parse().expect("numeric ID"),
                max_hp,
                damage,
            ),
        );
    }
}

/// A saved run in its second round, written by the same commands the DM controls use.
#[cfg(test)]
pub(crate) fn write_in_progress_run(
    campaign: Uuid,
    directory: &Path,
    name: &str,
    participants: Vec<(Participant, i64)>,
) -> Result<(), String> {
    write_in_progress_run_with(
        campaign,
        directory,
        name,
        participants,
        &format!("# {name}\n\nPhases and completion are resolved manually."),
    )
}

/// As `write_in_progress_run`, with the encounter's own text.
#[cfg(test)]
pub(crate) fn write_in_progress_run_with(
    campaign: Uuid,
    directory: &Path,
    name: &str,
    participants: Vec<(Participant, i64)>,
    details: &str,
) -> Result<(), String> {
    let (participants, initiatives): (Vec<_>, Vec<_>) = participants.into_iter().unzip();
    let mut run = Run::new(name.into(), participants, details.to_owned())?;
    run.state.random = Random::seeded(20_260_101);
    for (index, total) in initiatives.into_iter().enumerate() {
        run.apply(Command::Initiative(index, Some(total)))?;
    }
    run.apply(Command::Begin)?;
    let first = run.state.order[0];
    run.apply(Command::Note(first, "Holding the bridge.".into()))?;
    run.apply(Command::Next)?;
    let enemy = run.state.order[run.state.cursor];
    run.apply(Command::Damage(enemy, 9))?;
    run.apply(Command::Condition(enemy, "Prone".into()))?;
    run.apply(Command::Next)?;
    let caster = run.state.order[run.state.cursor];
    run.apply(Command::Concentration(caster, true))?;
    for _ in 0..run.state.order.len() {
        run.apply(Command::Next)?;
    }
    run.configure_storage(campaign, directory, Initiative::default())
}

/// A run that has ended, for test scenes. It says what the timeline records about it.
#[cfg(test)]
pub(crate) fn write_ended_run(
    campaign: Uuid,
    directory: &Path,
    name: &str,
    source: Option<Uuid>,
    outcome: &str,
    participant: Participant,
) -> Result<crate::timeline::RunFact, String> {
    let mut run = Run::new(name.into(), vec![participant], format!("# {name}"))?;
    run.state.random = Random::seeded(20_260_101);
    run.source = source;
    run.apply(Command::Initiative(0, Some(10)))?;
    run.apply(Command::Begin)?;
    run.apply(Command::End(outcome.into()))?;
    run.configure_storage(campaign, directory, Initiative::default())?;
    Ok(run.timeline_fact())
}

fn display(value: Option<i64>) -> String {
    value.map_or_else(|| "?".into(), |n| n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrity_d11_a_missing_run_folder_is_reported() {
        let dir = tempfile::tempdir().unwrap();
        let error = Run::list(&dir.path().join("gone"), Uuid::nil())
            .err()
            .unwrap();
        assert!(error.contains("Run folder not found"));
    }

    #[test]
    fn integrity_d23_renamed_runs_are_found_and_duplicates_are_named() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Uuid::new_v4();
        let mut run = run();
        begin(&mut run);
        run.configure_storage(campaign, dir.path(), Initiative::default())
            .unwrap();
        let renamed = dir.path().join("Bridge fight.json");
        fs::rename(run.path().unwrap(), &renamed).unwrap();
        assert_eq!(find_run(dir.path(), campaign, run.id).unwrap(), renamed);
        fs::copy(&renamed, dir.path().join("conflict.json")).unwrap();
        let error = find_run(dir.path(), campaign, run.id).unwrap_err();
        assert!(error.contains("Bridge fight.json") && error.contains("conflict.json"));
        let rows = Run::list(dir.path(), campaign).unwrap();
        assert_eq!(rows.len(), 2);
        assert!(
            rows.iter()
                .all(|row| row.label.contains("Duplicate identity"))
        );
    }

    #[test]
    fn integrity_d25_a_moved_campaign_is_never_created_again() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = dir.path().join("Campaign");
        fs::create_dir_all(&campaign).unwrap();
        let mut run = run();
        begin(&mut run);
        run.configure_storage(
            Uuid::new_v4(),
            &campaign.join("Runs"),
            Initiative::default(),
        )
        .unwrap();
        fs::rename(&campaign, dir.path().join("Moved campaign")).unwrap();
        run.apply(Command::Damage(0, 1)).unwrap();
        assert!(run.save().unwrap_err().contains("folder is gone"));
        assert!(!campaign.exists());
        assert!(run.needs_save());
    }

    #[test]
    fn integrity_d25_an_external_run_folder_still_checks_the_campaign() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = dir.path().join("Campaign");
        fs::create_dir_all(&campaign).unwrap();
        let mut run = run();
        begin(&mut run);
        run.configure_storage(
            Uuid::new_v4(),
            &dir.path().join("External runs"),
            Initiative::default(),
        )
        .unwrap();
        run.guard_campaign(&campaign);
        fs::rename(&campaign, dir.path().join("Moved campaign")).unwrap();
        run.apply(Command::Damage(0, 1)).unwrap();
        assert!(run.save().is_err());
        assert!(!campaign.exists());
    }
    use crate::combat_sync::SyncState;
    fn creature() -> Entry {
        Entry::parse("monster.md", &splat::content::formats::convert("Fixture.md", &format!("---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: Guard\nsplat_creature: {{type: humanoid}}\nsplat_stats:\n  max_hp: {{value: 20}}\n  armor_class: {{value: 16}}\n  initiative: {{value: 2}}\nsplat_initial:\n  hp: 17\n  temporary_hp: 3\n  conditions: [Poisoned]\n  death_saves: {{successes: 1, failures: 0}}\n---\n\n## Actions\nSpear attack.\n", Uuid::new_v4())).expect("schema-2 fixture")).unwrap()
    }
    pub(super) fn run() -> Run {
        let entry = creature();
        Run::new(
            "Ambush".into(),
            (0..3)
                .map(|i| {
                    Participant::new(
                        format!("slot{i}"),
                        "enemies".into(),
                        entry.clone(),
                        &Mapping::new(),
                        true,
                    )
                    .unwrap()
                })
                .collect(),
            "DM notes".into(),
        )
        .unwrap()
    }
    #[test]
    fn iter4_b_resumed_setup_blocks_unread_rows_and_rereads_only_the_changed_file() {
        let dir = tempfile::tempdir().unwrap();
        let id = Uuid::new_v4();
        let text = format!(
            "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: encounter\nsplat_name: Ambush\n---\n\n## Roster\n\n| Side | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| Enemies | [[Guard]] | Scout | HP lots | Keep this row |\n| Enemies | [[Guard]] | Captain | AC lots | Second fault |\n"
        );
        fs::write(dir.path().join("Ambush.md"), &text).unwrap();
        let mut run = run();
        run.came_from(id);
        let campaign = Uuid::new_v4();
        run.configure_storage(campaign, &dir.path().join("Runs"), Initiative::default())
            .unwrap();
        let path = run.path().unwrap().to_path_buf();
        let mut restored = Run::open(&path, campaign).unwrap();
        restored.guard_campaign(dir.path());
        assert_eq!(restored.roster_faults.len(), 2);
        assert_eq!(restored.roster_faults[0].name, "Scout");
        assert_eq!(restored.roster_faults[1].name, "Captain");
        for (index, total) in [20, 15, 10].into_iter().enumerate() {
            restored
                .apply(Command::Initiative(index, Some(total)))
                .unwrap();
        }
        let state = serde_json::to_value(&restored.state).unwrap();
        assert!(restored.apply(Command::Begin).is_err());
        assert_eq!(serde_json::to_value(&restored.state).unwrap(), state);
        assert_eq!(
            fs::read_to_string(dir.path().join("Ambush.md")).unwrap(),
            text
        );
        // A moved or deleted source must not leave a stale fault on complete run snapshots.
        let source = dir.path().join("Ambush.md");
        let moved = dir.path().join("Moved.md");
        fs::rename(&source, &moved).unwrap();
        restored.refresh_roster_faults();
        assert!(
            restored.roster_faults.is_empty(),
            "a moved source clears stale faults"
        );
        assert_eq!(serde_json::to_value(&restored.state).unwrap(), state);
        fs::rename(&moved, &source).unwrap();
        restored.refresh_roster_faults();
        assert_eq!(restored.roster_faults.len(), 2);
        fs::remove_file(&source).unwrap();
        restored.refresh_roster_faults();
        assert!(
            restored.roster_faults.is_empty(),
            "a deleted source clears stale faults"
        );
        assert_eq!(serde_json::to_value(&restored.state).unwrap(), state);
        fs::write(&source, &text).unwrap();
        restored.refresh_roster_faults();
        assert_eq!(restored.roster_faults.len(), 2);
        // Repair one fault: the other still blocks. No state or outside prose is overwritten.
        let partial = text.replace("HP lots", "HP +5");
        fs::write(dir.path().join("Ambush.md"), &partial).unwrap();
        assert!(restored.apply(Command::Begin).is_err());
        assert_eq!(restored.roster_faults.len(), 1);
        let repaired = format!(
            "{}\nOutside note stays.\n",
            partial.replace("AC lots", "AC +2")
        );
        fs::write(dir.path().join("Ambush.md"), &repaired).unwrap();
        restored.apply(Command::Begin).unwrap();
        assert_eq!(restored.state.phase, Phase::Running);
        assert_eq!(restored.state.participants.len(), 3);
        assert_eq!(
            fs::read_to_string(dir.path().join("Ambush.md")).unwrap(),
            repaired
        );
    }

    fn begin(run: &mut Run) {
        for (index, total) in [20, 15, 10].into_iter().enumerate() {
            run.apply(Command::Initiative(index, Some(total))).unwrap();
        }
        run.apply(Command::Begin).unwrap();
    }

    #[test]
    fn remote_choice_undo_names_one_or_many_characters() {
        let mut run = run();
        let before = run.state.clone();
        run.state.participants[0].hp = Some(8);
        run.reconcile_sync_history(&before, &[], &[0]);
        assert_eq!(run.undo_label(), "Undo: D&D Beyond HP for Guard 1");
        run.undo();
        assert_eq!(run.state.participants[0].hp, before.participants[0].hp);
        run.reconcile_sync_history(&before, &[], &[0, 1]);
        assert_eq!(run.undo_label(), "Undo: D&D Beyond values for 2 characters");
    }
    #[test]
    fn resolved_starting_state_is_independent_for_each_placement() {
        let mut entry = creature();
        let mut overrides = Mapping::new();
        overrides.insert("hp".into(), 5.into());
        let p =
            Participant::new("a".into(), "allies".into(), entry.clone(), &overrides, true).unwrap();
        assert_eq!(
            (p.hp, p.max_hp, p.temporary_hp, p.armor_class),
            (Some(5), Some(20), 3, Some(16))
        );
        assert_eq!(p.conditions, vec!["Poisoned"]);
        assert_eq!(p.successes, 1);
        entry.fields.insert("splat_stats".into(), Value::Null);
        assert_eq!(stat(&p.definition, "max_hp"), Some(20));
        let mut run = run();
        let before = run.state.participants[1].hp;
        run.apply(Command::Damage(0, 8)).unwrap();
        assert_eq!(run.state.participants[0].hp, Some(12));
        assert_eq!(run.state.participants[1].hp, before);
        assert_ne!(
            run.state.participants[0].name,
            run.state.participants[1].name
        );
        assert_ne!(run.state.participants[0].key, run.state.participants[1].key);
    }
    #[test]
    fn damage_uses_temporary_hp_healing_caps_and_undo_restores_state() {
        let mut run = run();
        run.apply(Command::Damage(0, 8)).unwrap();
        assert_eq!(
            (
                run.state.participants[0].hp,
                run.state.participants[0].temporary_hp
            ),
            (Some(12), 0)
        );
        run.undo();
        assert_eq!(
            (
                run.state.participants[0].hp,
                run.state.participants[0].temporary_hp
            ),
            (Some(17), 3)
        );
        run.apply(Command::Heal(0, i64::MAX)).unwrap();
        assert_eq!(run.state.participants[0].hp, Some(20));
        run.apply(Command::TempHp(0, 9)).unwrap();
        run.apply(Command::TempHp(0, 4)).unwrap();
        assert_eq!(run.state.participants[0].temporary_hp, 4);
        run.apply(Command::Damage(0, i64::MAX)).unwrap();
        assert_eq!(run.state.participants[0].hp, Some(0));
        assert!(run.state.participants[0].present); // zero HP still needs a turn for death saves
        let log = run.state.log.clone();
        assert!(run.apply(Command::Damage(0, -1)).is_err());
        assert_eq!(run.state.log, log);
        assert_eq!(run.state.participants[0].successes, 1); // no guessed death-save ruling
    }
    #[test]
    fn turns_wrap_skip_absent_keep_identity_and_do_not_grant_returning_actor_extra_turn() {
        let mut run = run();
        begin(&mut run);
        assert_eq!(run.current(), Some(0));
        run.apply(Command::Presence(1, false)).unwrap();
        run.apply(Command::Next).unwrap();
        assert_eq!(run.current(), Some(2));
        assert_eq!(run.state.round, 1);
        run.apply(Command::Presence(1, true)).unwrap();
        run.apply(Command::Next).unwrap();
        assert_eq!(run.current(), Some(0));
        assert_eq!(run.state.round, 2);
        run.apply(Command::Next).unwrap();
        assert_eq!(run.current(), Some(1));
        run.undo();
        assert_eq!(run.current(), Some(0));
        assert_eq!(run.state.round, 2);
        for i in 0..3 {
            run.apply(Command::Presence(i, false)).unwrap();
        }
        assert!(run.apply(Command::Next).is_err());
        assert_eq!(run.state.round, 2);
    }
    #[test]
    fn a_dice_roll_stays_in_the_log_and_is_not_an_undo_step() {
        let mut run = run();
        begin(&mut run);
        let health = |run: &Run| {
            let p = &run.state.participants[0];
            (p.hp, p.temporary_hp)
        };
        let whole = health(&run);
        run.apply(Command::Damage(0, 3)).unwrap();
        assert_ne!(health(&run), whole);
        let label = run.undo_label();
        let steps = run.history.len();
        run.state.random = Random::controlled([4, 5]);
        run.roll_dice("1d6").unwrap();
        let line = run.state.log.last().cloned().unwrap();
        assert!(
            line.starts_with("DM roll 1d6:") && line.ends_with("= 4."),
            "{line}"
        );
        // The roll is not a step: Undo still names the damage.
        assert_eq!(run.history.len(), steps);
        assert_eq!(run.undo_label(), label);
        run.undo();
        assert_eq!(health(&run), whole, "U takes back the damage");
        assert!(run.state.log.contains(&line), "the roll stays in the Log");
        // The dice go on, so the next roll is not the same one again.
        run.roll_dice("1d6").unwrap();
        assert!(run.state.log.last().unwrap().ends_with("= 5."));
        // Other commands keep their Undo.
        run.apply(Command::Heal(0, 1)).unwrap();
        assert_eq!(
            run.undo_label(),
            format!("Undo: 1 healing to {}", run.state.participants[0].name)
        );
        // A bad expression changes nothing.
        let log = run.state.log.len();
        assert!(run.roll_dice("1d").is_err());
        assert_eq!(run.state.log.len(), log);
    }
    #[test]
    fn initiative_requires_input_preserves_dm_ties_and_rolls_use_supplied_bonus_once() {
        let mut run = run();
        assert!(run.apply(Command::Begin).is_err());
        run.state.random = Random::controlled([12]);
        run.apply(Command::RollInitiative(0)).unwrap();
        assert_eq!(run.state.participants[0].initiative, Some(14));
        run.undo();
        run.apply(Command::RollInitiative(0)).unwrap();
        assert_eq!(run.state.participants[0].initiative, Some(14));
        run.apply(Command::Initiative(1, Some(14))).unwrap();
        run.apply(Command::Initiative(2, Some(5))).unwrap();
        run.apply(Command::Move(1, 0)).unwrap();
        assert!(run.apply(Command::Move(0, 2)).is_err());
        run.apply(Command::Begin).unwrap();
        assert_eq!(run.current(), Some(1));
        assert!(run.apply(Command::RollInitiative(0)).is_err());
        // In combat a total can be corrected, and never taken away.
        assert!(run.apply(Command::Initiative(0, None)).is_err());
    }
    #[test]
    fn missing_or_inconsistent_health_requires_dm_input() {
        let mut run = run();
        run.state.participants[0].max_hp = None;
        for i in 0..3 {
            run.apply(Command::Initiative(i, Some(10))).unwrap();
        }
        assert!(run.apply(Command::Begin).is_err());
        assert!(run.apply(Command::Health(0, Some(21), Some(20))).is_err());
        run.apply(Command::Health(0, Some(17), Some(20))).unwrap();
        run.apply(Command::Begin).unwrap();
        assert!(run.apply(Command::Health(0, None, Some(20))).is_err());
    }
    #[test]
    fn conditions_concentration_notes_and_manual_ending_can_be_undone() {
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Condition(0, "Prone".into())).unwrap();
        assert!(run.apply(Command::Condition(0, "prone".into())).is_err());
        run.apply(Command::Concentration(0, true)).unwrap();
        run.apply(Command::Note(0, "Used a spell slot".into()))
            .unwrap();
        run.apply(Command::DeathSaves(0, 2, 1)).unwrap();
        assert!(run.state.participants[0].concentration);
        assert!(run.state.participants[0].notes.contains("spell slot"));
        run.apply(Command::RemoveCondition(0, 1)).unwrap();
        assert_eq!(run.state.participants[0].conditions, vec!["Poisoned"]);
        run.apply(Command::End("Enemies fled".into())).unwrap();
        assert!(matches!(run.state.phase, Phase::Ended { .. }));
        assert!(run.state.log.last().unwrap().contains("Ended manually"));
        assert!(run.apply(Command::Damage(0, 2)).is_err());
        run.undo();
        assert_eq!(run.state.phase, Phase::Running);
        run.apply(Command::Next).unwrap();
    }

    #[test]
    fn iter_2_b_concentration_undo_names_the_recorded_direction() {
        let mut run = run();
        let name = run.state.participants[0].name.clone();
        run.apply(Command::Concentration(0, true)).unwrap();
        assert_eq!(
            run.undo_label(),
            format!("Undo: start concentration for {name}")
        );
        run.apply(Command::Concentration(0, false)).unwrap();
        assert_eq!(
            run.undo_label(),
            format!("Undo: stop concentration for {name}")
        );
        run.undo();
        assert!(run.state.participants[0].concentration);
        run.undo();
        assert!(!run.state.participants[0].concentration);
    }
    #[test]
    fn render_setup_running_and_ended_views_without_changing_state() {
        let mut run = run();
        let ctx = egui::Context::default();
        for phase in 0..3 {
            if phase == 1 {
                begin(&mut run);
            }
            if phase == 2 {
                run.apply(Command::End("Undecided".into())).unwrap();
            }
            let log = run.state.log.clone();
            for size in [egui::vec2(700.0, 600.0), egui::vec2(1000.0, 760.0)] {
                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            assert!(!run.show(ui));
                        });
                    },
                );
                assert!(!output.shapes.is_empty());
                assert_eq!(run.state.log, log);
            }
        }
    }
    #[test]
    fn saved_run_roundtrip_keeps_combat_state_and_rejects_external_edits() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Uuid::new_v4();
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Damage(0, 5)).unwrap();
        run.apply(Command::Next).unwrap();
        run.configure_storage(campaign, dir.path(), Initiative::default())
            .unwrap();
        let path = run.storage.as_ref().unwrap().path.clone();
        let mut reopened = Run::open(&path, campaign).unwrap();
        assert_eq!(reopened.current(), run.current());
        assert_eq!(
            reopened.state.participants[0].hp,
            run.state.participants[0].hp
        );
        assert_eq!(reopened.state.log, run.state.log);
        assert!(!reopened.needs_save());
        reopened.apply(Command::Next).unwrap();
        reopened.save().unwrap();
        assert_eq!(Run::open(&path, campaign).unwrap().current(), Some(2));
        let external = fs::read(&path).unwrap();
        run.apply(Command::Next).unwrap();
        assert!(run.save().is_err());
        assert_eq!(fs::read(&path).unwrap(), external);
        assert!(Run::open(&path, Uuid::new_v4()).is_err());
    }
    #[test]
    fn an_old_run_copy_requires_explicit_conversion_and_keeps_combat_state() {
        fn downgrade(value: &mut serde_json::Value) {
            if value
                .get("fields")
                .is_some_and(|f| f.get("splat_schema").is_some())
            {
                value["fields"]["splat_schema"] = 1.into();
                value["body"] = "## Actions\nSpear attack.\n".into();
            } else {
                match value {
                    serde_json::Value::Object(map) => map.values_mut().for_each(downgrade),
                    serde_json::Value::Array(values) => values.iter_mut().for_each(downgrade),
                    _ => {}
                }
            }
        }
        let dir = tempfile::tempdir().unwrap();
        let campaign = Uuid::new_v4();
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Damage(0, 5)).unwrap();
        run.apply(Command::Next).unwrap();
        run.configure_storage(campaign, dir.path(), Initiative::default())
            .unwrap();
        let path = run.storage.as_ref().unwrap().path.clone();
        let mut old: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        downgrade(&mut old);
        let before = serde_json::to_string_pretty(&old).unwrap();
        fs::write(&path, &before).unwrap();
        assert!(
            Run::open(&path, campaign)
                .err()
                .unwrap()
                .contains(splat::content::OLD_LAYOUT)
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), before);
        let relative = path.strip_prefix(dir.path()).unwrap().to_str().unwrap();
        let file = crate::conversion::preview(dir.path(), relative, before, &[]).unwrap();
        crate::conversion::publish(&file).unwrap();
        let reopened = Run::open(&path, campaign).unwrap();
        assert_eq!(reopened.id, run.id);
        assert_eq!(reopened.current(), run.current());
        assert_eq!(reopened.state.log, run.state.log);
        assert_eq!(
            reopened.state.participants[0].hp,
            run.state.participants[0].hp
        );
    }

    /// Change the saved file behind the run's back, as a vault sync tool would.
    fn run_with_changed_file() -> (tempfile::TempDir, Run, PathBuf, Vec<u8>, Uuid) {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Uuid::new_v4();
        let mut run = run();
        begin(&mut run);
        run.configure_storage(campaign, dir.path(), Initiative::default())
            .unwrap();
        let path = run.path().unwrap().to_path_buf();
        let mut other = Run::open(&path, campaign).unwrap();
        other.apply(Command::Next).unwrap();
        other.save().unwrap();
        let external = fs::read(&path).unwrap();
        run.apply(Command::Damage(0, 5)).unwrap();
        assert!(run.save().is_err());
        (dir, run, path, external, campaign)
    }
    #[test]
    fn a_changed_file_stops_saving_and_autosave_does_not_retry() {
        let (_dir, mut run, path, external, _) = run_with_changed_file();
        assert!(run.has_disk_conflict());
        run.apply(Command::Damage(0, 1)).unwrap();
        run.autosave();
        assert_eq!(fs::read(&path).unwrap(), external);
        assert!(run.needs_save());
        assert!(run.strip().not_saved);
    }
    #[test]
    fn keeping_this_run_replaces_the_changed_file_and_saving_works_again() {
        let (_dir, mut run, path, _, campaign) = run_with_changed_file();
        let hp = run.state.participants[0].hp;
        run.keep_this_run().unwrap();
        assert!(!run.has_disk_conflict());
        assert!(!run.needs_save());
        assert!(!run.strip().not_saved);
        assert_eq!(
            Run::open(&path, campaign).unwrap().state.participants[0].hp,
            hp
        );
        // A later change saves by itself, as it did before the conflict.
        run.apply(Command::Damage(0, 1)).unwrap();
        run.autosave();
        assert!(!run.needs_save());
        assert_eq!(
            Run::open(&path, campaign).unwrap().state.participants[0].hp,
            hp.map(|hp| hp - 1)
        );
    }
    #[test]
    fn saving_as_a_new_run_keeps_both_files() {
        let (dir, mut run, path, external, campaign) = run_with_changed_file();
        let hp = run.state.participants[0].hp;
        let before = run.timeline_fact();
        let mut story = crate::timeline::Book::default();
        story.record_change(None, &before, None);
        run.save_as_new_run().unwrap();
        story.record_change(Some(&before), &run.timeline_fact(), None);
        assert_eq!(story.events.len(), 1);
        assert_eq!(story.events[0].run, Some(run.id));
        assert!(!run.has_disk_conflict());
        assert_eq!(fs::read(&path).unwrap(), external);
        let new_path = run.path().unwrap().to_path_buf();
        assert_ne!(new_path, path);
        assert_eq!(new_path.parent(), Some(dir.path()));
        let copy = Run::open(&new_path, campaign).unwrap();
        assert_eq!(copy.state.participants[0].hp, hp);
        assert_ne!(copy.id, Run::open(&path, campaign).unwrap().id);
        assert_eq!(copy.name, "Ambush");
        assert!(copy.live_info().unwrap().label.starts_with("Copy 1 · "));
    }
    #[test]
    fn recovery_copies_resume_the_last_closed_run_and_preserve_external_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Uuid::new_v4();
        let mut run = started_run_in(dir.path(), campaign);
        let original_id = run.id;
        let original = run.path().unwrap().to_path_buf();
        let bytes = fs::read(&original).unwrap();
        fs::write(&original, [bytes.as_slice(), b"\n"].concat()).unwrap();
        run.apply(Command::Damage(0, 1)).unwrap();
        assert!(run.save().is_err());
        run.save_as_new_run().unwrap();
        assert!(run.close_run());
        let latest = run.id;
        let copies = Run::list(dir.path(), campaign).unwrap();
        assert_eq!(copies[0].id, Some(latest));
        assert_eq!(copies[0].encounter, "Ambush");
        assert!(copies[0].label.starts_with("Copy 1 · Closed last · "));
        assert!(copies[0].label.contains(" · Last: "));
        let old = copies
            .iter()
            .find(|copy| copy.id == Some(original_id))
            .unwrap();
        assert!(old.label.starts_with("Changed on disk · "));
        assert_eq!(
            fs::read(&original).unwrap(),
            [bytes.as_slice(), b"\n"].concat()
        );
        let mut old = Run::open(&original, campaign).unwrap();
        assert!(old.close_run());
        assert_eq!(
            Run::list(dir.path(), campaign).unwrap()[0].id,
            Some(original_id)
        );
    }

    #[test]
    fn iter7_b_preserved_recovery_copy_starts_with_copy_number() {
        let (dir, mut run, _, _, campaign) = run_with_changed_file();
        run.save_as_new_run().unwrap();
        let first = run.id;
        let path = run.path().unwrap().to_path_buf();
        let bytes = fs::read(&path).unwrap();
        fs::write(&path, [bytes.as_slice(), b"\n"].concat()).unwrap();
        run.apply(Command::Damage(0, 1)).unwrap();
        assert!(run.save().is_err());
        run.save_as_new_run().unwrap();
        let rows = Run::list(dir.path(), campaign).unwrap();
        let preserved = rows.iter().find(|row| row.id == Some(first)).unwrap();
        assert!(
            preserved.label.starts_with("Copy 1 · Changed on disk · "),
            "iter7-B-FT2: {}",
            preserved.label
        );
        assert_eq!(fs::read(path).unwrap(), [bytes.as_slice(), b"\n"].concat());
    }

    #[test]
    fn shared_run_state_words_never_show_round_zero() {
        for (round, paused, ended, expected) in [
            (0, false, false, "Setup"),
            (0, true, false, "Setup · Paused"),
            (2, false, false, "Round 2"),
            (2, true, false, "Round 2 · Paused"),
            (2, false, true, "Ended · Round 2"),
        ] {
            assert_eq!(state_words(round, paused, ended), expected);
        }
    }

    #[test]
    fn close_is_refused_in_every_phase_while_sync_has_not_finished() {
        for phase in [
            Phase::Setup,
            Phase::Running,
            Phase::Ended {
                outcome: "Won".into(),
            },
        ] {
            let dir = tempfile::tempdir().unwrap();
            let mut run = run();
            begin(&mut run);
            run.state.phase = phase.clone();
            run.configure_storage(Uuid::new_v4(), dir.path(), Initiative::default())
                .unwrap();
            run.syncs.insert(
                0,
                crate::combat_sync::CombatSync::simulated_conflict(1, 10, 1),
            );
            assert_eq!(run.close_block(), Some(CLOSE_BLOCKED), "{phase:?}");
            assert!(!run.close_run(), "{phase:?}");
            assert_eq!(run.error.as_deref(), Some(CLOSE_BLOCKED), "{phase:?}");
            run.syncs.clear();
            run.error = None;
            assert!(run.close_run(), "{phase:?}");
        }
    }
    pub(super) fn linked(run: &mut Run, index: usize, character: &str) {
        let mut definition = (*run.state.participants[index].definition).clone();
        definition.fields.insert(
            "splat_external".into(),
            serde_yaml_ng::from_str(&format!(
                "{{provider: dndbeyond, campaign_id: '1', character_id: '{character}'}}"
            ))
            .unwrap(),
        );
        run.state.participants[index].definition = std::sync::Arc::new(definition);
    }
    #[test]
    fn sync_party_starts_only_those_that_are_off_and_keeps_a_conflict() {
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        linked(&mut run, 1, "22");
        run.syncs.insert(
            0,
            crate::combat_sync::CombatSync::simulated_conflict(11, 10, 1),
        );
        assert!(matches!(
            run.party_sync_plan(),
            PartySync::Start(ref start) if start == &vec![1]
        ));
        run.toggle_party_sync();
        assert!(run.syncs[&1].enabled);
        assert_eq!(
            run.syncs[&0].state(),
            crate::combat_sync::SyncState::Conflict,
            "the running sync keeps its conflict"
        );
    }
    #[test]
    fn a_run_that_keeps_the_party_synced_starts_by_itself_and_respects_a_stop() {
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        linked(&mut run, 1, "22");
        let (campaign, _) = remote_identity(&run.state.participants[0]).unwrap();
        let ctx = egui::Context::default();
        let session = || {
            let hour = std::time::Duration::from_secs(3600);
            Some(crate::auth::Session::for_test(hour, hour))
        };
        run.keep_party_synced();

        run.tick_sync(&ctx, None, 1, Some(&campaign));
        assert!(run.syncs.is_empty(), "no connection, so nothing to start");
        run.tick_sync(&ctx, session(), 1, Some("another campaign"));
        assert!(run.syncs.is_empty(), "only this campaign's players");

        run.tick_sync(&ctx, session(), 1, Some(&campaign));
        assert_eq!(run.syncs[&0].state(), SyncState::FirstRead);
        assert_eq!(run.syncs[&1].state(), SyncState::FirstRead);
        assert!(!run.syncs.contains_key(&2), "a creature has no sheet");

        // The DM stopped one player. A later frame does not start it again.
        run.syncs.get_mut(&1).unwrap().stop();
        run.tick_sync(&ctx, session(), 1, Some(&campaign));
        assert!(!run.syncs[&1].enabled);

        // A lost connection comes back, and only what it stopped starts again.
        run.tick_sync(&ctx, None, 1, Some(&campaign));
        assert!(run.syncs[&0].session_lost());
        run.tick_sync(&ctx, session(), 2, Some(&campaign));
        assert!(run.syncs[&0].enabled && !run.syncs[&0].session_lost());
        assert!(!run.syncs[&1].enabled);

        // Stop for the whole party holds, as far as a later frame goes.
        run.syncs.get_mut(&1).unwrap().start();
        run.toggle_party_sync();
        assert!(!run.keeps_party_synced());
        run.tick_sync(&ctx, session(), 2, Some(&campaign));
        assert!(!run.syncs[&0].enabled && !run.syncs[&1].enabled);
    }
    fn test_session() -> Option<crate::auth::Session> {
        let hour = std::time::Duration::from_secs(3600);
        Some(crate::auth::Session::for_test(hour, hour))
    }
    #[test]
    fn a_stop_for_one_player_lasts_after_the_run_reopens() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Uuid::new_v4();
        let mut first = run();
        linked(&mut first, 0, "11");
        linked(&mut first, 1, "22");
        let (remote, _) = remote_identity(&first.state.participants[0]).unwrap();
        first
            .configure_storage(campaign, dir.path(), Initiative::default())
            .unwrap();
        first.keep_party_synced();
        let ctx = egui::Context::default();
        first.tick_sync(&ctx, test_session(), 1, Some(&remote));
        first.syncs.get_mut(&1).unwrap().stop();
        first.save().unwrap();

        let path = first.path().unwrap().to_path_buf();
        let mut reopened = Run::open(&path, campaign).unwrap();
        reopened.tick_sync(&ctx, test_session(), 1, Some(&remote));
        assert!(reopened.syncs[&0].enabled, "still on for the other player");
        assert!(!reopened.syncs[&1].enabled, "the DM's stop holds");
    }
    #[test]
    fn a_finished_run_keeps_its_results_and_undoing_the_end_syncs_again() {
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        let (remote, _) = remote_identity(&run.state.participants[0]).unwrap();
        run.keep_party_synced();
        let ctx = egui::Context::default();
        run.tick_sync(&ctx, test_session(), 1, Some(&remote));
        assert!(run.syncs[&0].enabled);

        run.apply(Command::End("Won".into())).unwrap();
        run.tick_sync(&ctx, test_session(), 1, Some(&remote));
        assert!(run.syncs.is_empty(), "no read can change the result");

        run.undo();
        run.tick_sync(&ctx, test_session(), 1, Some(&remote));
        assert!(run.syncs[&0].enabled);
    }
    #[test]
    fn a_read_only_sheet_leaves_the_runs_value_alone_at_a_first_read() {
        use crate::combat_sync::CombatSync;
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        let (remote, _) = remote_identity(&run.state.participants[0]).unwrap();
        let sheet = serde_json::json!({
            "id": 11,
            "removedHitPoints": 3,
            "temporaryHitPoints": 3,
            "deathSaves": {"successCount": 1, "failCount": 0},
            "conditions": [{"id": 11, "level": null}],
        });
        let mut last = CombatSync::default();
        last.enabled = true;
        last.simulate_sheet(11, 20, &sheet);
        run.state.participants[0].combat_values = last.values().clone();
        run.state.participants[0].hp = Some(10);
        let mut sync = CombatSync::default();
        sync.enabled = true;
        sync.simulate_sheet(11, 20, &sheet);
        run.syncs.insert(0, sync);

        run.tick_sync(&egui::Context::default(), test_session(), 1, Some(&remote));

        assert_eq!(run.syncs[&0].state(), SyncState::ReadOnly);
        assert_eq!(run.state.participants[0].hp, Some(10));
        assert!(run.close_block().is_none());
    }
    #[test]
    fn keeping_the_party_synced_is_saved_and_older_saves_load_with_it_off() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Uuid::new_v4();
        let mut first = run();
        first
            .configure_storage(campaign, dir.path(), Initiative::default())
            .unwrap();
        first.keep_party_synced();
        first.save().unwrap();
        let path = first.path().unwrap().to_path_buf();
        assert!(Run::open(&path, campaign).unwrap().keeps_party_synced());

        let text = fs::read_to_string(&path).unwrap();
        let mut older: serde_json::Value = serde_json::from_str(&text).unwrap();
        older.as_object_mut().unwrap().remove("keep_party_synced");
        fs::write(&path, serde_json::to_vec(&older).unwrap()).unwrap();
        assert!(!Run::open(&path, campaign).unwrap().keeps_party_synced());
    }
    #[test]
    fn a_session_that_ends_stops_each_sync_says_so_and_keeps_unsent_changes_counted() {
        use crate::combat_sync::CombatSync;
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        linked(&mut run, 1, "12");
        let name = run.state.participants[0].name.clone();
        let (campaign, _) = remote_identity(&run.state.participants[0]).unwrap();
        run.syncs.insert(
            0,
            CombatSync::simulated_state(SyncState::Pending, 11, 10, 0),
        );
        run.stop_simulating_sync();
        assert!(run.close_block().is_some(), "on their way: wait for them");

        run.tick_sync(&egui::Context::default(), None, 1, Some(&campaign));

        let sync = &run.syncs[&0];
        assert!(!sync.enabled);
        // It has a change it could not send, so the row says that, not only that sync stopped.
        assert_eq!(sync.state(), SyncState::NotSent);
        assert_eq!(
            run.sync_word(0),
            Some(SyncState::NotSent),
            "the row says so"
        );
        assert!(run.strip().sync_stopped, "the header and strip say so");
        assert!(run.pending_sync(), "the changes still count");
        assert_eq!(run.stranded_names(), vec![name.clone()]);
        assert_eq!(run.pending_sync_names(), vec![name]);
        assert!(
            run.close_block().is_none(),
            "no session must not trap the run"
        );
        // Sync party starts it again, with the row that was never on.
        assert!(matches!(
            run.party_sync_plan(),
            PartySync::Start(ref start) if start == &vec![0, 1]
        ));
        run.syncs.get_mut(&0).unwrap().start();
        assert!(!run.syncs[&0].session_lost());
        assert!(!run.strip().sync_stopped);
    }
    /// A linked participant with sync on, showing a sheet with this much damage taken. Its
    /// maximum is the test creature's.
    fn matched_with_sheet(damage: i64) -> (Run, String, i64) {
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        let max = run.state.participants[0].max_hp.unwrap();
        let name = run.state.participants[0].name.clone();
        run.syncs.entry(0).or_default().start();
        connected_tick(&mut run);
        read_sheet(&mut run, max, damage);
        run.notice = None;
        (run, name, max)
    }
    fn connected_tick(run: &mut Run) {
        let session = crate::auth::Session::for_test(
            std::time::Duration::from_secs(3600),
            std::time::Duration::from_secs(7200),
        );
        run.tick_sync(&egui::Context::default(), Some(session), 1, Some("1"));
    }
    fn offline_tick(run: &mut Run) {
        run.tick_sync(&egui::Context::default(), None, 1, Some("1"));
    }
    fn read_sheet(run: &mut Run, max: i64, damage: i64) {
        run.syncs
            .get_mut(&0)
            .unwrap()
            .simulate_read(11, max, damage);
        connected_tick(run);
    }
    fn reconnect(run: &mut Run) {
        run.syncs.get_mut(&0).unwrap().start();
        connected_tick(run);
    }
    #[test]
    fn a_change_made_during_a_stop_survives_reconnect_as_a_conflict() {
        let (mut run, name, max) = matched_with_sheet(5);
        assert_eq!(run.state.participants[0].hp, Some(max - 5));
        offline_tick(&mut run);
        assert_eq!(run.sync_word(0), Some(SyncState::Stopped));
        run.apply(Command::Damage(0, 4)).unwrap();
        offline_tick(&mut run);
        assert_eq!(run.state.participants[0].hp, Some(max - 9));
        // Closing warns, and names who: the change has not gone anywhere.
        assert_eq!(run.stranded_names(), vec![name.clone()]);
        assert!(run.pending_sync());
        assert_eq!(run.pending_sync_names(), vec![name]);
        assert!(
            run.close_block().is_none(),
            "no session must not trap the run"
        );

        let undo = run.history.len();
        reconnect(&mut run);
        assert_eq!(run.sync_word(0), Some(SyncState::FirstRead));
        read_sheet(&mut run, max, 5);

        // The DM's number stands, the choice is theirs, and nothing was blamed on the sheet.
        assert_eq!(run.state.participants[0].hp, Some(max - 9));
        assert_eq!(run.sync_word(0), Some(SyncState::Conflict));
        assert_eq!(run.syncs[&0].conflict_keys(), vec!["hp"]);
        assert_eq!(run.syncs[&0].values()["hp"]["removedHitPoints"], 9);
        assert!(run.notice.is_none());
        assert_eq!(run.history.len(), undo, "undo is kept");
        assert!(
            run.close_block().is_some(),
            "while it is open, closing waits"
        );
        connected_tick(&mut run);
        assert_eq!(run.state.participants[0].hp, Some(max - 9));
    }
    #[test]
    fn a_change_not_yet_sent_when_the_session_ends_is_kept_too() {
        let (mut run, name, max) = matched_with_sheet(5);
        run.apply(Command::Damage(0, 4)).unwrap();
        connected_tick(&mut run);
        assert_eq!(
            run.state.participants[0].combat_values["hp"]["removedHitPoints"], 5,
            "what is saved as the match is what the sheet confirmed, never the unsent 9"
        );
        offline_tick(&mut run);
        assert_eq!(run.stranded_names(), vec![name]);
        reconnect(&mut run);
        read_sheet(&mut run, max, 5);
        assert_eq!(run.state.participants[0].hp, Some(max - 9));
        assert_eq!(run.syncs[&0].conflict_keys(), vec!["hp"]);
    }
    #[test]
    fn a_run_keeps_its_encounter_and_says_how_it_stands() {
        let dir = tempfile::tempdir().unwrap();
        let (campaign, encounter) = (Uuid::new_v4(), Uuid::new_v4());
        let mut run = started_run_in(dir.path(), campaign);
        run.came_from(encounter);
        run.dirty = true;
        run.save().unwrap();
        let saved = Run::list(dir.path(), campaign).unwrap();
        assert_eq!(saved[0].source, Some(encounter));
        assert_eq!(saved[0].standing, Standing::Unfinished);
        assert_eq!(saved[0].id, Some(run.id));
        assert!(saved[0].label.contains("Round 1 · Paused"));

        run.apply(Command::End("Party won".into())).unwrap();
        run.save().unwrap();
        let saved = Run::list(dir.path(), campaign).unwrap();
        assert_eq!(saved[0].standing, Standing::Ended);
        assert!(saved[0].label.starts_with("Party won · Ended · Round 1"));
        assert_eq!(run.live_info().unwrap().standing, Standing::Ended);
    }

    /// With no connection, nothing was ever confirmed, so these values are only in this file.
    #[test]
    fn a_delete_knows_about_changes_that_never_reached_dnd_beyond() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Uuid::new_v4();
        let mut run = run();
        linked(&mut run, 0, "1");
        run.keep_party_synced();
        for index in 0..3 {
            run.apply(Command::Initiative(index, Some(10))).unwrap();
        }
        run.apply(Command::Begin).unwrap();
        run.configure_storage(campaign, dir.path(), Initiative::default())
            .unwrap();
        let name = run.state.participants[0].name.clone();
        assert!(run.live_info().unwrap().unsent.is_empty(), "nothing yet");

        run.apply(Command::Damage(0, 1)).unwrap();
        run.apply(Command::Damage(1, 1)).unwrap();
        run.save().unwrap();
        let saved = Run::list(dir.path(), campaign).unwrap();
        assert_eq!(saved[0].unsent, vec![name], "only the linked participant");
    }

    /// Another campaign can share the folder, so its file is not this campaign's to change.
    #[test]
    fn a_file_this_campaign_cannot_open_is_unreadable() {
        let dir = tempfile::tempdir().unwrap();
        let other = Uuid::new_v4();
        started_run_in(dir.path(), other);
        let saved = Run::list(dir.path(), Uuid::new_v4()).unwrap();
        assert_eq!(saved[0].standing, Standing::Unreadable);
        assert_eq!(saved[0].id, None);
    }

    #[test]
    fn a_deleted_run_comes_back_as_it_was() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Uuid::new_v4();
        let run = started_run_in(dir.path(), campaign);
        let path = run.path().unwrap().to_path_buf();
        let before = fs::read(&path).unwrap();
        let time = fs::metadata(&path).unwrap().modified().unwrap();
        let saved = Run::list(dir.path(), campaign).unwrap();

        let deleted = DeletedRun::delete(&saved[0]).unwrap();
        assert!(Run::list(dir.path(), campaign).unwrap().is_empty());

        deleted.restore().unwrap();
        assert_eq!(fs::read(&path).unwrap(), before);
        assert_eq!(fs::metadata(&path).unwrap().modified().unwrap(), time);
        assert!(
            deleted.restore().is_err(),
            "a file now in its place stays as it is"
        );
    }

    fn started_run_in(directory: &Path, campaign: Uuid) -> Run {
        let mut run = run();
        for index in 0..3 {
            run.apply(Command::Initiative(index, Some(10))).unwrap();
        }
        run.apply(Command::Begin).unwrap();
        run.configure_storage(campaign, directory, Initiative::default())
            .unwrap();
        run
    }

    /// Through the file, as a close and reopen does. Anything only in memory is gone.
    fn close_and_reopen(run: &mut Run, campaign: Uuid) -> Run {
        run.dirty = true;
        run.save().unwrap();
        Run::open(run.path().unwrap(), campaign).unwrap()
    }
    fn stored(run: &mut Run, dir: &std::path::Path) -> Uuid {
        let campaign = Uuid::new_v4();
        run.configure_storage(campaign, dir, Initiative::default())
            .unwrap();
        campaign
    }
    fn first_sync(run: &mut Run, max: i64, damage: i64) {
        run.syncs.entry(0).or_default().start();
        connected_tick(run);
        read_sheet(run, max, damage);
    }
    #[test]
    fn damage_before_the_first_read_is_kept_as_a_conflict() {
        let dir = tempfile::tempdir().unwrap();
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        let max = run.state.participants[0].max_hp.unwrap();
        run.apply(Command::Damage(0, 4)).unwrap();
        let hp = run.state.participants[0].hp;
        // Sync party, with no match yet: the run's starting values are the comparison.
        let campaign = stored(&mut run, dir.path());
        let mut in_memory = close_and_reopen(&mut run, campaign);
        first_sync(&mut run, max, 0);
        assert_eq!(run.state.participants[0].hp, hp);
        assert_eq!(run.syncs[&0].conflict_keys(), vec!["hp"]);
        // The same after a close and reopen, because the starting values are saved.
        first_sync(&mut in_memory, max, 0);
        assert_eq!(in_memory.state.participants[0].hp, hp);
        assert_eq!(in_memory.syncs[&0].conflict_keys(), vec!["hp"]);
    }
    #[test]
    fn a_first_sync_with_no_change_is_ordinary_and_names_what_it_replaced() {
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        let max = run.state.participants[0].max_hp.unwrap();
        let hp = run.state.participants[0].hp.unwrap();
        first_sync(&mut run, max, 5);
        assert_eq!(run.state.participants[0].hp, Some(max - 5));
        assert!(run.syncs[&0].conflict_keys().is_empty());
        let notice = run.notice.clone().unwrap();
        let change = format!("HP {hp} → {}", max - 5);
        assert!(notice.contains(&change), "{notice}");
        assert!(run.state.log.iter().any(|line| line.contains(&change)));
    }
    #[test]
    fn unsent_when_the_session_ends_then_closed_still_asks() {
        let dir = tempfile::tempdir().unwrap();
        let (mut run, name, max) = matched_with_sheet(5);
        let campaign = stored(&mut run, dir.path());
        run.apply(Command::Damage(0, 4)).unwrap();
        connected_tick(&mut run);
        offline_tick(&mut run);
        assert_eq!(run.stranded_names(), vec![name], "the dialog's promise");

        let mut reopened = close_and_reopen(&mut run, campaign);
        assert_eq!(
            reopened.state.participants[0].combat_values["hp"]["removedHitPoints"],
            5
        );
        first_sync(&mut reopened, max, 5);
        assert_eq!(reopened.state.participants[0].hp, Some(max - 9));
        assert_eq!(reopened.syncs[&0].conflict_keys(), vec!["hp"]);
    }
    #[test]
    fn an_open_conflict_survives_stop_sync_and_a_close() {
        let dir = tempfile::tempdir().unwrap();
        let (mut run, name, max) = matched_with_sheet(5);
        let campaign = stored(&mut run, dir.path());
        offline_tick(&mut run);
        run.apply(Command::Damage(0, 4)).unwrap();
        offline_tick(&mut run);
        reconnect(&mut run);
        read_sheet(&mut run, max, 5);
        assert_eq!(run.sync_word(0), Some(SyncState::Conflict));

        // Stop sync lifts the close block, so the close now says what it leaves.
        run.syncs.get_mut(&0).unwrap().stop();
        connected_tick(&mut run);
        assert!(run.close_block().is_none());
        assert_eq!(run.stranded_names(), vec![name]);

        let mut reopened = close_and_reopen(&mut run, campaign);
        first_sync(&mut reopened, max, 5);
        assert_eq!(reopened.state.participants[0].hp, Some(max - 9));
        assert_eq!(reopened.syncs[&0].conflict_keys(), vec!["hp"]);
    }
    #[test]
    fn an_open_conflict_is_found_again_after_an_app_close_or_a_crash() {
        let dir = tempfile::tempdir().unwrap();
        let (mut run, _, max) = matched_with_sheet(5);
        let campaign = stored(&mut run, dir.path());
        offline_tick(&mut run);
        run.apply(Command::Damage(0, 4)).unwrap();
        offline_tick(&mut run);
        reconnect(&mut run);
        read_sheet(&mut run, max, 2);
        assert_eq!(run.sync_word(0), Some(SyncState::Conflict));
        // As last saved, with sync still on: nothing else is written at a crash.
        let keys = run.synced_keys();
        let mut reopened = close_and_reopen(&mut run, campaign);
        reopened.restore_sync(&keys);
        connected_tick(&mut reopened);
        read_sheet(&mut reopened, max, 2);
        assert_eq!(reopened.state.participants[0].hp, Some(max - 9));
        assert_eq!(reopened.syncs[&0].conflict_keys(), vec!["hp"]);
    }
    #[test]
    fn a_close_with_no_change_reads_as_usual_after_it() {
        let dir = tempfile::tempdir().unwrap();
        let (mut run, _, max) = matched_with_sheet(5);
        let campaign = stored(&mut run, dir.path());
        let mut reopened = close_and_reopen(&mut run, campaign);
        first_sync(&mut reopened, max, 8);
        assert_eq!(reopened.state.participants[0].hp, Some(max - 8));
        assert!(reopened.syncs[&0].conflict_keys().is_empty());
    }
    /// A save from before starting values were kept. New fields are optional.
    fn as_older_save(path: &Path, combat_values: Option<serde_json::Value>) {
        let mut saved: serde_json::Value =
            serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        for p in saved["state"]["participants"].as_array_mut().unwrap() {
            assert!(p.as_object_mut().unwrap().remove("started").is_some());
            if let Some(values) = &combat_values {
                p["combat_values"] = values.clone();
            }
        }
        fs::write(path, serde_json::to_vec(&saved).unwrap()).unwrap();
    }
    #[test]
    fn an_older_save_with_no_match_asks_rather_than_replaces() {
        let dir = tempfile::tempdir().unwrap();
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        let max = run.state.participants[0].max_hp.unwrap();
        let campaign = stored(&mut run, dir.path());
        run.dirty = true;
        run.save().unwrap();
        let path = run.path().unwrap().to_path_buf();
        as_older_save(&path, None);
        let mut reopened = Run::open(&path, campaign).unwrap();
        let hp = reopened.state.participants[0].hp;
        first_sync(&mut reopened, max, 3);
        assert_eq!(reopened.state.participants[0].hp, hp);
        assert!(
            reopened.syncs[&0]
                .conflict_keys()
                .contains(&"hp".to_owned())
        );
    }
    /// An older save can hold unsent values as its match. A missing `started` says the save is
    /// that old, so every difference asks, and only this once.
    #[test]
    fn an_older_save_whose_match_equals_the_run_asks_once() {
        let dir = tempfile::tempdir().unwrap();
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        let max = run.state.participants[0].max_hp.unwrap();
        let campaign = stored(&mut run, dir.path());
        run.apply(Command::Damage(0, 4)).unwrap();
        run.apply(Command::Condition(0, "Prone".into())).unwrap();
        run.save().unwrap();
        let path = run.path().unwrap().to_path_buf();
        let unsent = serde_json::json!({
            "hp": {"removedHitPoints": 4, "temporaryHitPoints": 0}
        });
        as_older_save(&path, Some(unsent));
        let mut reopened = Run::open(&path, campaign).unwrap();
        let hp = reopened.state.participants[0].hp;
        first_sync(&mut reopened, max, 0);
        assert_eq!(reopened.state.participants[0].hp, hp);
        let conflicts = reopened.syncs[&0].conflict_keys();
        assert!(conflicts.contains(&"hp".to_owned()));
        assert!(conflicts.contains(&"conditions".to_owned()));
        assert!(
            reopened.state.participants[0].started.is_some(),
            "it asks only once"
        );
    }
    #[test]
    fn a_conflict_choice_is_a_named_undo_step() {
        let (mut run, _, max) = matched_with_sheet(5);
        offline_tick(&mut run);
        run.apply(Command::Damage(0, 4)).unwrap();
        run.apply(Command::Next).unwrap();
        offline_tick(&mut run);
        reconnect(&mut run);
        read_sheet(&mut run, max, 2);
        run.syncs.get_mut(&0).unwrap().use_remote("hp");
        connected_tick(&mut run);
        assert_eq!(run.state.participants[0].hp, Some(max - 2));
        assert!(run.undo_label().contains("D&D Beyond"));
        run.undo();
        connected_tick(&mut run);
        assert_eq!(run.state.participants[0].hp, Some(max - 9));
        assert_eq!(run.syncs[&0].values()["hp"]["removedHitPoints"], 9);
        assert!(!run.undo_label().contains("D&D Beyond"));
        let cursor = run.state.cursor;
        run.undo();
        assert_ne!(run.state.cursor, cursor, "the prior turn step remains");
    }
    #[test]
    fn a_sync_panel_choice_keeps_earlier_steps() {
        let (mut run, _, max) = matched_with_sheet(5);
        run.apply(Command::Next).unwrap();
        connected_tick(&mut run);
        let count = run.history.len();
        run.syncs.get_mut(&0).unwrap().set_group(
            "hp",
            serde_json::json!({"removedHitPoints": 8, "temporaryHitPoints": 0}),
        );
        connected_tick(&mut run);
        assert_eq!(run.history.len(), count + 1);
        assert!(run.undo_label().contains("D&D Beyond"));
        run.undo();
        connected_tick(&mut run);
        assert_eq!(run.state.participants[0].hp, Some(max - 5));
    }
    #[test]
    fn a_named_step_undoes_as_before_with_sync_on() {
        let (mut run, _, max) = matched_with_sheet(5);
        run.apply(Command::Damage(0, 4)).unwrap();
        connected_tick(&mut run);
        assert_eq!(
            run.undo_label(),
            format!("Undo: 4 damage to {}", run.state.participants[0].name)
        );
        // The conditions come back in the sheet's order, which is no change.
        run.apply(Command::Condition(0, "prone".into())).unwrap();
        run.apply(Command::Condition(0, "Blinded".into())).unwrap();
        connected_tick(&mut run);
        connected_tick(&mut run);
        assert!(!run.history.is_empty(), "the DM's own steps stay");
        run.undo();
        run.undo();
        run.undo();
        connected_tick(&mut run);
        // Named, chosen by the DM, and sent as an ordinary edit.
        assert_eq!(run.state.participants[0].hp, Some(max - 5));
        assert_eq!(run.syncs[&0].values()["hp"]["removedHitPoints"], 5);
    }
    #[test]
    fn undo_after_a_stop_keeps_what_the_sheet_confirmed() {
        let dir = tempfile::tempdir().unwrap();
        let (mut run, _, max) = matched_with_sheet(5);
        let campaign = stored(&mut run, dir.path());
        run.apply(Command::Damage(0, 4)).unwrap();
        connected_tick(&mut run);
        read_sheet(&mut run, max, 9);
        assert_eq!(
            run.state.participants[0].combat_values["hp"]["removedHitPoints"],
            9
        );
        run.syncs.get_mut(&0).unwrap().stop();
        connected_tick(&mut run);
        // Undo takes the step back from the run. The sheet still holds 9.
        run.undo();
        assert_eq!(run.state.participants[0].hp, Some(max - 5));
        assert_eq!(
            run.state.participants[0].combat_values["hp"]["removedHitPoints"],
            9
        );
        let mut reopened = close_and_reopen(&mut run, campaign);
        first_sync(&mut reopened, max, 9);
        assert_eq!(reopened.state.participants[0].hp, Some(max - 5));
        assert!(
            reopened.syncs[&0]
                .conflict_keys()
                .contains(&"hp".to_owned())
        );
    }
    /// A sheet given in full. Poisoned is id 11, Prone is id 12, in D&D Beyond's own fields.
    fn sheet(damage: i64, conditions: &[u64]) -> serde_json::Value {
        serde_json::json!({
            "id": 11, "canEdit": true, "removedHitPoints": damage, "temporaryHitPoints": 0,
            "deathSaves": {"successCount": 0, "failCount": 0},
            "conditions": conditions.iter().map(|id| serde_json::json!({"id": id})).collect::<Vec<_>>(),
            "spellSlots": [{"level": 1, "used": 1}],
        })
    }
    fn read_full_sheet(run: &mut Run, max: i64, sheet: &serde_json::Value) {
        run.syncs
            .get_mut(&0)
            .unwrap()
            .simulate_sheet(11, max, sheet);
        connected_tick(run);
    }
    fn matched_with_conditions(conditions: &[u64]) -> (Run, i64) {
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        let max = run.state.participants[0].max_hp.unwrap();
        run.syncs.entry(0).or_default().start();
        connected_tick(&mut run);
        read_full_sheet(&mut run, max, &sheet(5, conditions));
        run.notice = None;
        (run, max)
    }
    #[test]
    fn a_condition_recorded_while_stopped_survives_a_close_as_a_conflict() {
        let dir = tempfile::tempdir().unwrap();
        let (mut run, max) = matched_with_conditions(&[]);
        let campaign = stored(&mut run, dir.path());
        offline_tick(&mut run);
        run.apply(Command::Condition(0, "Prone".into())).unwrap();
        offline_tick(&mut run);
        assert!(!run.stranded_names().is_empty());
        let mut reopened = close_and_reopen(&mut run, campaign);
        reopened.syncs.entry(0).or_default().start();
        connected_tick(&mut reopened);
        read_full_sheet(&mut reopened, max, &sheet(5, &[]));
        assert!(
            reopened.state.participants[0]
                .conditions
                .iter()
                .any(|c| c == "Prone")
        );
        assert!(
            reopened.syncs[&0]
                .conflict_keys()
                .contains(&"conditions".to_owned())
        );
    }
    #[test]
    fn a_condition_recorded_before_the_first_read_is_kept_as_a_conflict() {
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        let max = run.state.participants[0].max_hp.unwrap();
        run.apply(Command::Condition(0, "Prone".into())).unwrap();
        run.syncs.entry(0).or_default().start();
        connected_tick(&mut run);
        read_full_sheet(&mut run, max, &sheet(0, &[]));
        assert!(
            run.state.participants[0]
                .conditions
                .iter()
                .any(|c| c == "Prone")
        );
        assert!(
            run.syncs[&0]
                .conflict_keys()
                .contains(&"conditions".to_owned())
        );
    }
    #[test]
    fn removing_a_condition_the_sheet_holds_is_kept_too() {
        let (mut run, max) = matched_with_conditions(&[11]);
        let held = run.state.participants[0]
            .conditions
            .iter()
            .position(|c| c == "Poisoned")
            .unwrap();
        run.syncs.get_mut(&0).unwrap().stop();
        connected_tick(&mut run);
        run.apply(Command::RemoveCondition(0, held)).unwrap();
        connected_tick(&mut run);
        reconnect(&mut run);
        read_full_sheet(&mut run, max, &sheet(5, &[11]));
        assert!(
            !run.state.participants[0]
                .conditions
                .iter()
                .any(|c| c == "Poisoned")
        );
        assert!(
            run.syncs[&0]
                .conflict_keys()
                .contains(&"conditions".to_owned())
        );
    }
    #[test]
    fn a_sheet_update_names_conditions_and_temp_hp_from_what_to_what() {
        let (mut run, max) = matched_with_conditions(&[11]);
        let mut update = sheet(5, &[12]);
        update["temporaryHitPoints"] = 4.into();
        read_full_sheet(&mut run, max, &update);
        let notice = run.notice.clone().unwrap();
        assert!(notice.contains("Poisoned removed"), "{notice}");
        assert!(notice.contains("Prone added"), "{notice}");
        assert!(notice.contains("temp HP 0 → 4"), "{notice}");
        assert!(
            run.state
                .log
                .iter()
                .any(|line| line.contains("Prone added"))
        );
    }
    #[test]
    fn a_spell_slot_not_sent_asks_after_a_restart_of_sync_and_a_close_says_it_is_lost() {
        let dir = tempfile::tempdir().unwrap();
        let (mut run, max) = matched_with_conditions(&[]);
        let name = run.state.participants[0].name.clone();
        let campaign = stored(&mut run, dir.path());
        let mut slots = run.syncs[&0].values()["slots"].clone();
        slots["level1"] = 2.into();
        run.syncs.get_mut(&0).unwrap().set_group("slots", slots);
        offline_tick(&mut run);
        // A close would lose it, and the dialog says so.
        assert_eq!(run.unsent_only_here_names(), vec![name]);
        // Within the session it is not dropped: the next first read asks.
        reconnect(&mut run);
        read_full_sheet(&mut run, max, &sheet(5, &[]));
        assert_eq!(run.syncs[&0].conflict_keys(), vec!["slots"]);
        assert_eq!(run.syncs[&0].values()["slots"]["level1"], 2);
        // A reopened run has no such value, and no claim is made for it.
        let reopened = close_and_reopen(&mut run, campaign);
        assert!(reopened.unsent_only_here_names().is_empty());
    }
    #[test]
    fn an_absent_participant_needs_no_initiative_to_begin() {
        let mut run = run();
        run.apply(Command::Initiative(1, Some(14))).unwrap();
        run.apply(Command::Initiative(2, Some(9))).unwrap();
        assert!(
            run.apply(Command::Begin).is_err(),
            "still here, so a roll is due"
        );
        assert_eq!(
            run.state.order,
            vec![1, 2, 0],
            "setup keeps the order sorted"
        );
        run.apply(Command::Presence(0, false)).unwrap();
        run.apply(Command::Begin).unwrap();
        // Last in the order, and no turn is theirs while they are away.
        assert_eq!(run.state.order, vec![1, 2, 0]);
        assert_eq!(run.current(), Some(1));
        run.apply(Command::Next).unwrap();
        run.apply(Command::Next).unwrap();
        assert_eq!(run.current(), Some(1));
        assert_eq!(run.state.round, 2);
    }
    #[test]
    fn someone_back_with_no_initiative_is_last_until_the_dm_sets_it() {
        let dir = tempfile::tempdir().unwrap();
        let mut run = run();
        run.apply(Command::Initiative(1, Some(14))).unwrap();
        run.apply(Command::Initiative(2, Some(9))).unwrap();
        run.apply(Command::Presence(0, false)).unwrap();
        run.apply(Command::Begin).unwrap();
        run.apply(Command::Next).unwrap();
        run.apply(Command::Presence(0, true)).unwrap();
        assert!(run.state.log.last().unwrap().contains("no initiative"));
        // A run in this state saves and opens.
        let campaign = stored(&mut run, dir.path());
        let mut run = close_and_reopen(&mut run, campaign);
        assert_eq!(run.state.participants[0].initiative, None);
        // Their turn comes last, with no ruling on where they should be.
        run.apply(Command::Next).unwrap();
        assert_eq!(run.current(), Some(0));
        run.apply(Command::Next).unwrap();
        assert_eq!(run.current(), Some(1));
        // The DM sets it: they take their place, and the current turn stays current.
        run.apply(Command::Initiative(0, Some(12))).unwrap();
        assert_eq!(run.state.order, vec![1, 0, 2]);
        assert_eq!(run.current(), Some(1));
        // Once only, and a named step.
        assert!(run.apply(Command::Initiative(0, None)).is_err());
        assert!(run.undo_label().contains("initiative"));
        run.undo();
        assert_eq!(run.state.order, vec![1, 2, 0]);
        assert_eq!(run.state.participants[0].initiative, None);
    }
    #[test]
    fn a_maximum_the_sheet_decides_is_refused_with_the_reason_not_replaced() {
        let (mut run, _, max) = matched_with_sheet(5);
        let hp = run.state.participants[0].hp;
        let refused = run
            .apply(Command::Health(0, hp, Some(max + 8)))
            .unwrap_err();
        assert_eq!(refused, MAX_FROM_SHEET);
        assert_eq!(run.state.participants[0].max_hp, Some(max));
        // Current HP can still be corrected, and it goes out as an edit.
        run.apply(Command::Health(0, Some(max - 7), Some(max)))
            .unwrap();
        connected_tick(&mut run);
        assert_eq!(run.state.participants[0].hp, Some(max - 7));
        assert_eq!(run.syncs[&0].values()["hp"]["removedHitPoints"], 7);
    }
    #[test]
    fn a_maximum_set_while_sync_was_off_is_replaced_only_with_a_message() {
        let (mut run, _, max) = matched_with_sheet(0);
        run.syncs.get_mut(&0).unwrap().stop();
        connected_tick(&mut run);
        run.apply(Command::Health(0, Some(max + 3), Some(max + 8)))
            .unwrap();
        connected_tick(&mut run);
        assert_eq!(
            run.state.participants[0].max_hp,
            Some(max + 8),
            "kept while off"
        );
        reconnect(&mut run);
        read_sheet(&mut run, max, 0);
        // The sheet cannot hold it, so nothing is sent: it stands, and the words say what went.
        assert_eq!(run.state.participants[0].max_hp, Some(max));
        assert!(run.syncs[&0].conflict_keys().is_empty());
        assert_eq!(run.syncs[&0].values()["hp"]["removedHitPoints"], 0);
        let notice = run.notice.clone().unwrap();
        assert!(
            notice.contains(&format!("maximum HP {} → {max}", max + 8)),
            "{notice}"
        );
        assert!(
            notice.contains(&format!("HP {} → {max}", max + 3)),
            "{notice}"
        );
        assert!(run.state.log.iter().any(|line| line.contains("maximum HP")));
    }
    #[test]
    fn a_change_to_the_maximum_alone_still_says_so() {
        let (mut run, _, max) = matched_with_sheet(0);
        run.syncs.get_mut(&0).unwrap().stop();
        connected_tick(&mut run);
        run.apply(Command::Health(0, Some(max), Some(max + 8)))
            .unwrap();
        let correction = run.undo_label();
        connected_tick(&mut run);
        reconnect(&mut run);
        read_sheet(&mut run, max, 0);
        assert_eq!(run.state.participants[0].hp, Some(max));
        let notice = run.notice.clone().unwrap();
        assert!(
            notice.contains(&format!("maximum HP {} → {max}", max + 8)),
            "{notice}"
        );
        assert!(
            run.history_labels
                .iter()
                .all(|label| format!("Undo: {label}") != correction),
            "the stale maximum correction is removed"
        );
        while !run.history.is_empty() {
            run.undo();
            assert_eq!(run.state.participants[0].max_hp, Some(max));
        }
    }
    #[test]
    fn hp_above_the_sheets_maximum_never_sends_a_negative_value() {
        let (mut run, _, max) = matched_with_sheet(5);
        connected_tick(&mut run);
        // However it got there: an older step, for one.
        run.state.participants[0].hp = Some(max + 3);
        connected_tick(&mut run);
        let removed = run.syncs[&0].values()["hp"]["removedHitPoints"]
            .as_i64()
            .unwrap();
        assert_eq!(removed, 0);
        // And not in a conflict held at a first read.
        let (mut run, _, max) = matched_with_sheet(5);
        run.syncs.get_mut(&0).unwrap().stop();
        connected_tick(&mut run);
        run.state.participants[0].hp = Some(max + 3);
        run.state.participants[0].max_hp = Some(max + 3);
        reconnect(&mut run);
        read_sheet(&mut run, max, 5);
        for group in run.syncs[&0].values().values() {
            assert!(
                group["removedHitPoints"].as_i64().unwrap_or(0) >= 0,
                "{group}"
            );
        }
    }
    #[test]
    fn the_undo_notice_clears_on_the_next_run_action() {
        let (mut run, _, max) = matched_with_sheet(5);
        read_sheet(&mut run, max, 8);
        assert!(run.notice.as_ref().unwrap().contains("Undo for "));
        assert!(run.undo_stopped, "plain Undo gives this reason on hover");
        run.next_turn();
        run.apply(Command::Damage(0, 1)).unwrap();
        assert!(
            run.notice.is_none(),
            "the Log keeps the sheet update after a new action"
        );
    }
    #[test]
    fn the_existing_controls_settle_a_reconnect_conflict() {
        // Keep Splat values: the DM's number goes out as an ordinary edit.
        let (mut run, _, max) = matched_with_sheet(5);
        offline_tick(&mut run);
        run.apply(Command::Damage(0, 4)).unwrap();
        offline_tick(&mut run);
        reconnect(&mut run);
        read_sheet(&mut run, max, 5);
        run.syncs.get_mut(&0).unwrap().keep_local("hp");
        connected_tick(&mut run);
        assert_eq!(run.state.participants[0].hp, Some(max - 9));
        assert!(run.syncs[&0].conflict_keys().is_empty());
        assert!(run.syncs[&0].dirty(), "it still has to go to the sheet");
        assert_eq!(run.sync_word(0), Some(SyncState::Pending));

        // Use D&D Beyond values: the sheet's number replaces it, because the DM said so.
        let (mut run, _, max) = matched_with_sheet(5);
        offline_tick(&mut run);
        run.apply(Command::Damage(0, 4)).unwrap();
        offline_tick(&mut run);
        reconnect(&mut run);
        read_sheet(&mut run, max, 2);
        run.syncs.get_mut(&0).unwrap().use_remote("hp");
        connected_tick(&mut run);
        assert_eq!(run.state.participants[0].hp, Some(max - 2));
        assert!(!run.syncs[&0].dirty());
    }
    #[test]
    fn with_no_change_since_the_last_match_the_first_read_is_ordinary() {
        let (mut run, _, max) = matched_with_sheet(5);
        offline_tick(&mut run);
        assert!(run.stranded_names().is_empty(), "nothing to warn about");
        reconnect(&mut run);
        read_sheet(&mut run, max, 8);
        assert_eq!(run.state.participants[0].hp, Some(max - 8));
        assert_ne!(run.sync_word(0), Some(SyncState::Conflict));
        assert!(run.syncs[&0].conflict_keys().is_empty());
        let notice = run.notice.clone().unwrap();
        assert!(
            notice.contains("From D&D Beyond")
                && notice.contains(&format!("HP {} → {}", max - 5, max - 8)),
            "{notice}"
        );
        assert!(!notice.contains("changed on D&D Beyond"), "{notice}");
    }
    #[test]
    fn a_change_the_sheet_already_has_is_not_a_conflict() {
        let (mut run, _, max) = matched_with_sheet(5);
        offline_tick(&mut run);
        run.apply(Command::Damage(0, 4)).unwrap();
        offline_tick(&mut run);
        reconnect(&mut run);
        read_sheet(&mut run, max, 9);
        assert_eq!(run.state.participants[0].hp, Some(max - 9));
        assert!(run.syncs[&0].conflict_keys().is_empty());
    }
    #[test]
    fn a_change_made_after_stopping_sync_by_hand_is_kept_too() {
        let (mut run, _, max) = matched_with_sheet(5);
        run.syncs.get_mut(&0).unwrap().stop();
        connected_tick(&mut run);
        run.apply(Command::Damage(0, 4)).unwrap();
        connected_tick(&mut run);
        reconnect(&mut run);
        read_sheet(&mut run, max, 5);
        assert_eq!(run.state.participants[0].hp, Some(max - 9));
        assert_eq!(run.syncs[&0].conflict_keys(), vec!["hp"]);
    }
    #[test]
    fn after_a_restart_a_change_since_the_saved_values_is_kept() {
        let (mut run, _, max) = matched_with_sheet(5);
        let values = run.state.participants[0].combat_values.clone();
        assert!(!values.is_empty(), "the run saves what it last matched");
        // As a saved run comes back: no sync yet, and it waits for a session.
        run.syncs.clear();
        let key = run.state.participants[0].key.clone();
        run.restore_sync(std::slice::from_ref(&key));
        offline_tick(&mut run);
        run.apply(Command::Damage(0, 4)).unwrap();
        offline_tick(&mut run);
        connected_tick(&mut run);
        read_sheet(&mut run, max, 5);
        assert_eq!(run.state.participants[0].hp, Some(max - 9));
        assert_eq!(run.syncs[&0].conflict_keys(), vec!["hp"]);

        // No change while it was away: the read is the ordinary one.
        let (mut run, _, max) = matched_with_sheet(5);
        run.syncs.clear();
        run.restore_sync(&[key]);
        connected_tick(&mut run);
        read_sheet(&mut run, max, 8);
        assert_eq!(run.state.participants[0].hp, Some(max - 8));
        assert!(run.syncs[&0].conflict_keys().is_empty());
    }
    #[test]
    fn a_sync_stopped_as_a_duplicate_does_not_hide_stop_sync() {
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        linked(&mut run, 1, "11");
        let mut on = crate::combat_sync::CombatSync::default();
        on.start();
        run.syncs.insert(0, on);
        let mut stopped = crate::combat_sync::CombatSync::default();
        stopped.error = Some("Another participant is already syncing this character.".into());
        run.syncs.insert(1, stopped);
        assert!(matches!(
            run.party_sync_plan(),
            PartySync::Stop(ref stop) if stop == &vec![0]
        ));
    }
    #[test]
    fn shared_rolls_are_quiet_but_an_independent_equal_total_needs_a_ruling() {
        let mut run = run();
        run.state.participants[0].npc = false;
        run.apply(Command::Initiative(0, Some(19))).unwrap();
        run.state.random = Random::controlled([10]);
        run.apply(Command::RollNpcs(Grouping::Identical)).unwrap();
        assert_eq!(run.tied_count(), 0);
        assert_eq!(
            run.state.participants[1].shared_roll,
            run.state.participants[2].shared_roll
        );
        let dir = tempfile::tempdir().unwrap();
        let campaign = Uuid::new_v4();
        run.configure_storage(campaign, dir.path(), Initiative::default())
            .unwrap();
        let mut reopened = Run::open(run.path().unwrap(), campaign).unwrap();
        assert_eq!(reopened.tied_count(), 0);
        reopened.apply(Command::Initiative(0, Some(12))).unwrap();
        assert_eq!(reopened.tied_count(), 3);
        reopened.apply(Command::Initiative(1, Some(12))).unwrap();
        assert!(reopened.state.participants[1].shared_roll.is_none());
        reopened.undo();
        assert!(reopened.state.participants[1].shared_roll.is_some());
    }

    #[test]
    fn an_open_run_reports_when_its_file_was_saved() {
        let dir = tempfile::tempdir().unwrap();
        let mut run = run();
        run.configure_storage(Uuid::new_v4(), dir.path(), Initiative::default())
            .unwrap();
        let info = run.live_info().unwrap();
        assert!(
            info.saved_at.is_some(),
            "the open row must have a save time"
        );
        assert!(info.label.contains("Saved just now"));
    }

    #[test]
    fn iter_3_b_legacy_recovery_copy_keeps_its_number_and_file() {
        let (dir, mut run, _, _, campaign) = run_with_changed_file();
        run.keep_this_run().unwrap();
        run.name = "Ambush · saved 03:00:00 (2)".into();
        run.dirty = true;
        run.save().unwrap();
        let path = run.path().unwrap().to_path_buf();
        let bytes = fs::read(&path).unwrap();
        let rows = Run::list(dir.path(), campaign).unwrap();
        assert_eq!(rows[0].encounter, "Ambush");
        assert!(rows[0].label.starts_with("Copy 2 · "));
        assert_eq!(fs::read(&path).unwrap(), bytes);
        run.save_as_new_run().unwrap();
        assert!(run.live_info().unwrap().label.starts_with("Copy 3 · "));
        assert_eq!(fs::read(path).unwrap(), bytes);
    }

    #[test]
    fn recovery_copies_keep_the_name_and_number_each_copy() {
        let (_dir, mut run, _, _, campaign) = run_with_changed_file();
        let name = run.name.clone();
        run.save_as_new_run().unwrap();
        let first_path = run.path().unwrap().to_path_buf();
        let first_bytes = fs::read(&first_path).unwrap();
        assert_eq!(run.name, name);
        assert!(run.live_info().unwrap().label.starts_with("Copy 1 · "));
        run.save_as_new_run().unwrap();
        assert_eq!(run.name, name);
        assert_ne!(run.path().unwrap(), first_path);
        assert_eq!(fs::read(first_path).unwrap(), first_bytes);
        let reopened = Run::open(run.path().unwrap(), campaign).unwrap();
        assert_eq!(reopened.name, name);
        assert!(reopened.live_info().unwrap().label.starts_with("Copy 2 · "));
    }

    #[test]
    fn iter_3_b_exhaustion_undo_names_both_levels_and_restores_them() {
        let mut run = run();
        run.apply(Command::Condition(0, "Exhaustion (2)".into()))
            .unwrap();
        for (old, new) in [(2, 3), (3, 1), (1, 0)] {
            let command = if new == 0 {
                Command::RemoveCondition(0, 1)
            } else {
                Command::Several(vec![
                    Command::RemoveCondition(0, 1),
                    Command::Condition(0, format!("Exhaustion ({new})")),
                ])
            };
            let expected = format!(
                "exhaustion {old} → {new} on {}",
                run.state.participants[0].name
            );
            assert_eq!(run.describe(&command), expected);
            run.apply(command).unwrap();
            run.undo();
            assert_eq!(
                run.state.participants[0].conditions,
                vec!["Poisoned".to_owned(), format!("Exhaustion ({old})")]
            );
            if new > 0 {
                run.apply(Command::Several(vec![
                    Command::RemoveCondition(0, 1),
                    Command::Condition(0, format!("Exhaustion ({new})")),
                ]))
                .unwrap();
            }
        }
    }

    #[test]
    fn npc_rolls_follow_campaign_grouping_and_never_change_player_initiative() {
        let mut run = run();
        run.state.participants[0].npc = false;
        run.apply(Command::Initiative(0, Some(19))).unwrap();
        run.state.random = Random::controlled([10, 7]);
        run.apply(Command::RollNpcs(Grouping::Identical)).unwrap();
        assert_eq!(run.state.participants[0].initiative, Some(19));
        assert_eq!(run.state.participants[1].initiative, Some(12));
        assert_eq!(run.state.participants[2].initiative, Some(12));
        run.undo();
        run.apply(Command::RollNpcs(Grouping::Individual)).unwrap();
        assert_eq!(run.state.participants[1].initiative, Some(12));
        assert_eq!(run.state.participants[2].initiative, Some(9));
        run.undo();
        run.apply(Command::RollNpcs(Grouping::BySide)).unwrap();
        assert_eq!(run.state.participants[2].initiative, Some(12));
        run.undo();
        run.state.participants[1].initiative_group = Some("wave".into());
        run.state.participants[2].initiative_group = Some("wave".into());
        run.apply(Command::RollNpcs(Grouping::Explicit)).unwrap();
        assert_eq!(run.state.participants[2].initiative, Some(12));
    }
    #[test]
    fn one_npc_roll_can_use_another_rule_and_the_campaign_rule_stays() {
        let mut run = run();
        run.state.participants[0].npc = false;
        run.state.random = Random::controlled([10, 7]);
        assert_eq!(run.initiative.grouping, Grouping::Identical);
        run.apply(Command::RollNpcs(Grouping::Individual)).unwrap();
        assert_eq!(run.state.participants[1].initiative, Some(12));
        assert_eq!(run.state.participants[2].initiative, Some(9));
        assert_eq!(run.initiative.grouping, Grouping::Identical);
        assert!(run.state.log.last().unwrap().contains("Individually"));
    }
    #[test]
    fn initiative_groups_are_a_roll_choice_only_when_an_npc_has_a_group() {
        let mut run = run();
        assert!(!run.roll_choices().contains(&Grouping::Explicit));
        run.state.participants[1].initiative_group = Some("wave".into());
        assert!(run.roll_choices().contains(&Grouping::Explicit));
        run.state.participants[1].npc = false;
        assert!(!run.roll_choices().contains(&Grouping::Explicit));
    }
    #[test]
    fn undo_names_the_last_action_and_each_step_goes_back() {
        let mut run = run();
        begin(&mut run);
        assert_eq!(run.undo_label(), "Undo: begin combat");
        let name = run.state.participants[1].name.clone();
        run.apply(Command::Damage(1, 7)).unwrap();
        assert_eq!(run.undo_label(), format!("Undo: 7 damage to {name}"));
        run.undo();
        assert_eq!(run.undo_label(), "Undo: begin combat");
    }
    #[test]
    fn one_amount_on_several_targets_is_one_step_and_half_rounds_down() {
        let mut run = run();
        begin(&mut run);
        run.state
            .participants
            .iter_mut()
            .for_each(|p| p.temporary_hp = 0);
        let hp: Vec<_> = run.state.participants.iter().map(|p| p.hp).collect();
        run.selected = 0;
        run.also.insert(1);
        run.halved.insert(1);
        let steps = run.history.len();
        let lines = run.state.log.len();
        run.apply(run.amount_command(5, Command::Damage)).unwrap();
        assert_eq!(run.history.len(), steps + 1);
        assert_eq!(run.state.log.len(), lines + 2);
        assert_eq!(run.state.participants[0].hp, hp[0].map(|n| (n - 5).max(0)));
        assert_eq!(run.state.participants[1].hp, hp[1].map(|n| (n - 2).max(0)));
        assert_eq!(
            run.undo_label(),
            "Undo: 5 damage to 2 participants, half for 1"
        );
        run.undo();
        assert_eq!(run.state.participants[0].hp, hp[0]);
        assert_eq!(run.state.participants[1].hp, hp[1]);
        // Half is for a save. Healing goes to every target in full.
        run.also.insert(1);
        run.halved.insert(1);
        match run.amount_command(5, Command::Heal) {
            Command::Several(commands) => assert!(
                commands
                    .iter()
                    .all(|command| matches!(command, Command::Heal(_, 5)))
            ),
            _ => panic!("two targets are one step"),
        }
    }
    #[test]
    fn a_turn_that_moves_from_another_page_lets_go_of_the_targets() {
        let mut run = run();
        begin(&mut run);
        run.also.insert(1);
        run.halved.insert(1);
        run.next_turn();
        assert!(run.also.is_empty() && run.halved.is_empty());
        // Even if the two meet, one target takes one amount.
        run.selected = 1;
        run.also.insert(1);
        run.also.insert(0);
        assert_eq!(run.targets().iter().filter(|at| **at == 1).count(), 1);
    }
    #[test]
    fn live_initiative_says_whether_the_place_has_passed() {
        for (total, waits) in [(18, true), (8, false)] {
            let mut run = run();
            begin(&mut run);
            run.apply(Command::Next).unwrap();
            let current = run.current();
            run.apply(Command::Initiative(0, Some(total))).unwrap();
            let notice = run.notice.as_deref().unwrap();
            assert!(
                notice.contains(&format!("moves to\u{a0}{total}")),
                "{notice}"
            );
            assert_eq!(notice.contains("acts next round"), waits, "{notice}");
            // It acted at 20, and 8 is still to come in this round.
            assert_eq!(notice.contains("acts again this round"), !waits, "{notice}");
            assert_arrival_wraps(notice);
            assert_eq!(run.current(), current);
        }
    }

    /// The order by participant, and whose turn it is.
    fn turns(run: &Run) -> (Vec<usize>, Option<usize>) {
        (run.state.order.clone(), run.current())
    }

    #[test]
    fn a_change_to_another_row_moves_it_and_keeps_the_turn() {
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Next).unwrap();
        // The row below the current one passes it, and acts next round.
        run.apply(Command::Initiative(2, Some(18))).unwrap();
        assert_eq!(turns(&run), (vec![0, 2, 1], Some(1)));
        assert!(run.notice.as_deref().unwrap().contains("acts next round"));
        run.apply(Command::Next).unwrap();
        assert_eq!((run.current(), run.state.round), (Some(0), 2));
        // One undo step takes it back.
        let mut run = self::run();
        begin(&mut run);
        run.apply(Command::Initiative(2, Some(18))).unwrap();
        run.undo();
        assert_eq!(turns(&run), (vec![0, 1, 2], Some(0)));
    }

    #[test]
    fn a_tie_keeps_its_order_unless_the_campaign_uses_the_roster() {
        for (roster_ties, order) in [(false, vec![0, 2, 1]), (true, vec![0, 1, 2])] {
            let mut run = run();
            run.initiative.roster_ties = roster_ties;
            begin(&mut run);
            // The DM put the later roster row first in a tie of their own.
            run.state.participants[1].initiative = Some(12);
            run.state.order = vec![0, 2, 1];
            run.state.participants[2].initiative = Some(12);
            run.apply(Command::Initiative(2, Some(12))).unwrap();
            assert_eq!(run.state.order, order, "roster ties: {roster_ties}");
            assert_eq!(run.current(), Some(0));
        }
    }

    #[test]
    fn the_current_row_keeps_its_turn_and_moves_when_it_ends() {
        // Up: it has acted, and the row it passed has too, so the next turn is the one after.
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Next).unwrap();
        run.apply(Command::Next).unwrap();
        run.apply(Command::Initiative(2, Some(18))).unwrap();
        assert_eq!(turns(&run), (vec![0, 1, 2], Some(2)));
        let notice = run.notice.clone().unwrap();
        assert!(notice.contains("when this turn ends"), "{notice}");
        assert!(notice.contains("acts next round"), "{notice}");
        run.apply(Command::Next).unwrap();
        assert_eq!(turns(&run), (vec![0, 2, 1], Some(0)));
        assert_eq!(run.state.round, 2);
        // Down, as a delay: the next row acts, and then this one.
        let mut run = self::run();
        begin(&mut run);
        run.apply(Command::Initiative(0, Some(12))).unwrap();
        assert_eq!(turns(&run), (vec![0, 1, 2], Some(0)));
        let notice = run.notice.clone().unwrap();
        assert!(notice.contains("acts later this round"), "{notice}");
        run.apply(Command::Next).unwrap();
        assert_eq!(turns(&run), (vec![1, 0, 2], Some(1)));
        run.apply(Command::Next).unwrap();
        assert_eq!((run.current(), run.state.round), (Some(0), 1));
        // To the top from the last place: it opens the next round.
        let mut run = self::run();
        begin(&mut run);
        run.apply(Command::Next).unwrap();
        run.apply(Command::Next).unwrap();
        run.apply(Command::Initiative(2, Some(25))).unwrap();
        run.apply(Command::Next).unwrap();
        assert_eq!(turns(&run), (vec![2, 0, 1], Some(2)));
        assert_eq!(run.state.round, 2);
        // A total that keeps its place keeps the turn, and says so.
        let mut run = self::run();
        begin(&mut run);
        run.apply(Command::Initiative(0, Some(17))).unwrap();
        assert!(run.notice.as_deref().unwrap().contains("The turn stays"));
        run.apply(Command::Next).unwrap();
        assert_eq!(turns(&run), (vec![0, 1, 2], Some(1)));
    }

    /// Whose turn each Next gives, as (round, participant).
    fn next_turns(run: &mut Run, count: usize) -> Vec<(u64, usize)> {
        (0..count)
            .map(|_| {
                run.apply(Command::Next).unwrap();
                (run.state.round, run.current().unwrap())
            })
            .collect()
    }

    #[test]
    fn a_change_with_nobody_present_ends_and_says_so() {
        let mut run = run();
        begin(&mut run);
        for index in 0..3 {
            run.apply(Command::Presence(index, false)).unwrap();
        }
        run.apply(Command::Initiative(0, Some(1))).unwrap();
        assert!(
            run.notice
                .as_deref()
                .unwrap()
                .contains("when this turn ends")
        );
        assert_eq!(run.apply(Command::Next), Err(NOBODY_PRESENT.into()));
    }

    #[test]
    fn a_tie_out_of_roster_order_is_no_change_to_wait_for() {
        // An arrival moved within a tie, or a tie set by hand before the campaign chose
        // roster order. Each row still acts once a round.
        let mut run = run();
        begin(&mut run);
        run.state.participants[2].initiative = Some(15);
        run.state.order = vec![0, 2, 1];
        run.initiative.roster_ties = true;
        assert_eq!(
            next_turns(&mut run, 5),
            vec![(1, 2), (1, 1), (2, 0), (2, 2), (2, 1)]
        );
    }

    #[test]
    fn a_delay_past_an_absent_row_acts_later_this_round() {
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Presence(1, false)).unwrap();
        // 12 is above the 10 whose turn is next, so this row acts before it.
        run.apply(Command::Initiative(0, Some(12))).unwrap();
        let notice = run.notice.clone().unwrap();
        assert!(notice.contains("acts later this round"), "{notice}");
        assert_eq!(next_turns(&mut run, 3), vec![(1, 0), (1, 2), (2, 0)]);
        assert_eq!(run.state.order, vec![1, 0, 2]);
    }

    #[test]
    fn a_row_placed_while_the_current_row_waits_goes_beside_the_others() {
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Next).unwrap();
        // The current row waits at 1, where 15 was.
        run.apply(Command::Initiative(1, Some(1))).unwrap();
        // 5 goes after 10, not before the 1 that waits.
        run.apply(Command::Initiative(0, Some(5))).unwrap();
        assert_eq!(turns(&run), (vec![1, 2, 0], Some(1)));
        run.apply(Command::Next).unwrap();
        assert_eq!(turns(&run), (vec![2, 0, 1], Some(2)));
        run.apply(Command::Next).unwrap();
        assert_eq!(turns(&run), (vec![2, 0, 1], Some(0)));
        run.apply(Command::Next).unwrap();
        assert_eq!((run.current(), run.state.round), (Some(1), 1));
    }

    fn assert_arrival_wraps(message: &str) {
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for width in [240.0, 280.0, 320.0] {
                    let galley = ui.painter().layout(
                        message.to_owned(),
                        crate::design::Type::Small.font(ui),
                        crate::design::colors().text,
                        width,
                    );
                    for row in &galley.rows {
                        let text: String = row.glyphs.iter().map(|glyph| glyph.chr).collect();
                        assert!(!text.trim_end().ends_with('·'), "{text}");
                        assert!(!text.trim_end().ends_with(" at"), "{text}");
                    }
                }
            });
        });
    }

    #[test]
    fn saved_row_keeps_the_close_dialog_change_label() {
        let dir = tempfile::tempdir().unwrap();
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Damage(0, 3)).unwrap();
        let expected = Run::change_with_result(&run.state, run.history_labels.last().unwrap());
        let campaign = stored(&mut run, dir.path());
        let reopened = close_and_reopen(&mut run, campaign);
        assert!(reopened.history_labels.is_empty());
        let info = reopened.info(PathBuf::new(), None, true);
        assert!(
            info.label.contains(&format!(" · Last: {expected}")),
            "{}",
            info.label
        );
        assert!(expected.contains("HP left"));
    }

    #[test]
    fn panel_choice_does_not_claim_the_sheet_changed() {
        let mut run = run();
        let line = format!("{}: HP 20 → 18", run.state.participants[0].name);
        run.note_sync_changes(&[], std::slice::from_ref(&line));
        assert!(run.notice.is_none());
        assert!(run.notice_summary.is_none());
        assert!(run.state.log.last().unwrap().contains(&line));
    }

    #[test]
    fn a_wrong_initiative_moves_its_row_and_the_turn_stays_with_one_step_back() {
        let mut run = run();
        begin(&mut run);
        let current = run.current().unwrap();
        let other = (0..run.state.participants.len())
            .find(|index| *index != current)
            .unwrap();
        let order = run.state.order.clone();
        run.apply(Command::Initiative(other, Some(100))).unwrap();
        assert_eq!(run.state.order[0], other);
        assert_eq!(run.current(), Some(current));
        run.undo();
        assert_eq!(run.state.order, order);
        assert!(run.apply(Command::Initiative(other, None)).is_err());
    }
    #[test]
    fn only_the_dm_moves_a_run_on_the_timeline() {
        let mut run = run();
        begin(&mut run);
        let fact = run.timeline_fact();
        // A sheet's change reaches the log with no step of the DM's.
        run.state
            .log
            .push("Kestrel: HP changed on D&D Beyond.".into());
        assert_eq!(run.timeline_fact(), fact);
        run.apply(Command::Damage(0, 1)).unwrap();
        let hit = run.timeline_fact();
        assert!(hit.played_since(&fact));
        run.undo();
        assert!(run.timeline_fact().played_since(&hit));
    }

    #[test]
    fn a_setup_with_nothing_done_is_untouched_until_sync_changes_it() {
        let mut run = run();
        assert!(run.untouched_setup());
        run.undo_stopped = true;
        assert!(!run.untouched_setup());
    }
    #[test]
    fn a_ctrl_click_on_the_selected_row_gives_the_selection_to_another_target() {
        let mut run = run();
        run.selected = 0;
        run.toggle_target(1);
        assert!(run.also.contains(&1));
        run.toggle_target(0);
        assert_eq!(run.selected, 1);
        assert!(run.also.is_empty());
    }
    #[test]
    fn the_prep_tab_separates_a_runs_words_from_its_setup() {
        let mut run = run();
        run.setup_details = format!("Ash falls.\n\n{SETUP_HEADING}\n\n```yaml\nroster: []\n```");
        let (prep, technical) = run.prep();
        assert_eq!(prep, "Ash falls.");
        assert!(technical.unwrap().contains("roster"));
        // An older save, or a run with no setup, has only words.
        run.setup_details = "Only words.".into();
        assert_eq!(run.prep(), ("Only words.", None));
    }
    #[test]
    fn a_later_rest_restores_saved_setup_once_with_undo_but_keeps_combat_hp() {
        let folder = tempfile::tempdir().unwrap();
        let mut setup = run();
        linked(&mut setup, 0, "42");
        setup.state.participants[0].hp = Some(3);
        setup.state.participants[0].temporary_hp = 2;
        let campaign = stored(&mut setup, folder.path());
        let mut resumed = close_and_reopen(&mut setup, campaign);
        resumed.rest_saved_setup(1);
        assert_eq!(
            resumed.state.participants[0].hp,
            resumed.state.participants[0].max_hp
        );
        assert_eq!(resumed.state.participants[0].temporary_hp, 0);
        assert_eq!(resumed.history.len(), 1);
        assert_eq!(resumed.undo_label(), "Undo: full HP after long rest");
        assert_eq!(
            resumed.state.log.last().unwrap(),
            "Party HP is full after the long rest."
        );
        resumed.undo();
        assert_eq!(
            (
                resumed.state.participants[0].hp,
                resumed.state.participants[0].temporary_hp
            ),
            (Some(3), 2)
        );
        let mut reopened = close_and_reopen(&mut resumed, campaign);
        reopened.rest_saved_setup(1);
        assert_eq!(reopened.state.participants[0].hp, Some(3));
        reopened.rest_saved_setup(2);
        assert_eq!(reopened.history.len(), 1);
        begin(&mut reopened);
        reopened.state.participants[0].hp = Some(4);
        let mut combat = close_and_reopen(&mut reopened, campaign);
        combat.rest_saved_setup(3);
        assert_eq!(combat.state.participants[0].hp, Some(4));
        assert!(combat.history.is_empty());
        let mut setup = run();
        linked(&mut setup, 0, "42");
        setup.state.participants[0].hp = Some(3);
        setup.remember_rest(3);
        setup.rest_saved_setup(3);
        assert_eq!(setup.state.participants[0].hp, Some(3));
    }
    #[test]
    fn use_the_saved_hp_is_its_own_step_and_undo_gives_back_the_full_hp() {
        let folder = tempfile::tempdir().unwrap();
        let mut setup = run();
        linked(&mut setup, 0, "42");
        linked(&mut setup, 1, "43");
        setup.state.participants[0].hp = Some(3);
        setup.state.participants[0].temporary_hp = 2;
        setup.state.participants[1].hp = Some(5);
        setup.state.participants[1].temporary_hp = 0;
        // Slot 2 is not a hero: the rest does not touch it, and neither does the saved HP.
        setup.state.participants[2].hp = Some(1);
        let campaign = stored(&mut setup, folder.path());
        let mut run = close_and_reopen(&mut setup, campaign);
        let max = run.state.participants[0].max_hp;
        let hp = |run: &Run| {
            run.state
                .participants
                .iter()
                .map(|p| (p.hp, p.temporary_hp))
                .collect::<Vec<_>>()
        };
        let other = hp(&run)[2];
        let saved = vec![(Some(3), 2), (Some(5), 0), other];
        let full = vec![(max, 0), (max, 0), other];
        run.rest_saved_setup(1);
        assert_eq!(hp(&run), full);
        assert!(run.rest_filled_saved_hp() && !run.saved_hp_used());

        let before = run.state.log.len();
        run.use_saved_hp().unwrap();
        assert_eq!(
            hp(&run),
            saved,
            "Use the saved HP puts back every hero's HP"
        );
        assert_eq!(
            run.state
                .log
                .iter()
                .filter(|line| *line == SAVED_HP_USED)
                .count(),
            1,
            "The Log names the step once"
        );
        assert_eq!(
            run.state.log[before], SAVED_HP_USED,
            "before each hero's line"
        );
        // A named step, so the rest's full HP is still one Undo away.
        assert_eq!(run.history.len(), 2);
        assert_eq!(run.undo_label(), "Undo: use the saved HP");
        assert!(run.saved_hp_used() && !run.rest_filled_saved_hp());
        assert!(run.back_to_sheets().is_none());
        assert!(run.use_saved_hp().is_err(), "the press cannot repeat");

        run.undo();
        assert_eq!(hp(&run), full, "Undo gives back the full HP");
        assert!(!run.state.log.iter().any(|line| line == SAVED_HP_USED));
        assert_eq!(run.undo_label(), "Undo: full HP after long rest");
        assert!(run.rest_filled_saved_hp() && !run.saved_hp_used());
        run.undo();
        assert_eq!(
            hp(&run),
            saved,
            "Undo of the rest still restores the saved HP"
        );
        assert!(!run.rest_filled_saved_hp() && !run.saved_hp_used());

        // The result lasts until the next step.
        let mut again = close_and_reopen(&mut run, campaign);
        again.rest_saved_setup(2);
        again.use_saved_hp().unwrap();
        again
            .apply(Command::Initiative(0, Some(12)))
            .expect("a later step");
        assert!(!again.saved_hp_used());

        // With no hero, the rest has nothing to put back and offers nothing.
        let mut monsters = run_with_rest_and_no_hero(folder.path());
        assert!(!monsters.rest_filled_saved_hp());
        assert!(monsters.use_saved_hp().is_err());
    }
    fn run_with_rest_and_no_hero(dir: &std::path::Path) -> Run {
        let mut setup = run();
        let campaign = stored(&mut setup, dir);
        let mut run = close_and_reopen(&mut setup, campaign);
        run.rest_saved_setup(1);
        run
    }

    #[test]
    fn a_player_starts_from_the_last_fight_and_the_sheet_is_one_step_back() {
        let id = Uuid::new_v4();
        let entry = Entry::parse(
            "Hero.md",
            &splat::content::formats::convert("Fixture.md", &format!(
                "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: creature\nsplat_name: Hero\nsplat_creature: {{size: medium}}\nsplat_external: {{provider: test}}\nsplat_stats:\n  max_hp: {{value: 41}}\nsplat_initial:\n  hp: 11\n---\n"
            )).expect("schema-2 fixture"),
        )
        .unwrap();
        let mut hero =
            Participant::new("hero".into(), "party".into(), entry, &Mapping::new(), true).unwrap();
        let carry = Carry {
            rest_order: 0,
            from: "the bridge".into(),
            full: false,
            values: BTreeMap::from([(id, (7, 2))]),
            missing: Vec::new(),
        };
        assert_eq!(hero.carry_over(&carry), Some((Some(11), 0)));
        assert_eq!((hero.hp, hero.temporary_hp), (Some(7), 2));
        // A value the same as the sheet has nothing to say.
        assert_eq!(hero.carry_over(&carry), None);
        let rested = Carry {
            rest_order: 1,
            from: "the long rest".into(),
            full: true,
            values: BTreeMap::new(),
            missing: vec!["the shrine".into()],
        };
        hero.carry_over(&rested);
        assert_eq!((hero.hp, hero.temporary_hp), (Some(41), 0));
        // A ruling is not what the sheet began with, so sync treats it as the DM's value.
        assert_eq!(hero.started.as_ref().and_then(|s| s.hp), Some(7));

        let mut run = Run::new("Next".into(), vec![hero], String::new()).unwrap();
        run.note_carry(&rested, vec![(0, Some(11), 0)]);
        assert!(run.carry_missing().unwrap().contains("the shrine"));
        assert!(run.state.log.last().unwrap().contains("carried over"));
        let (from, back) = run.back_to_sheets().unwrap();
        assert_eq!(from, "the long rest");
        run.apply(back).unwrap();
        assert_eq!(run.state.participants[0].hp, Some(11));
        assert_eq!(run.undo_label(), "Undo: HP from the sheets");
        assert!(run.back_to_sheets().is_none());
        run.undo();
        assert_eq!(run.state.participants[0].hp, Some(41));
    }
    #[test]
    fn a_monster_does_not_carry_anything() {
        let id = Uuid::new_v4();
        let entry = Entry::parse(
            "Wolf.md",
            &splat::content::formats::convert("Fixture.md", &format!(
                "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: creature\nsplat_name: Wolf\nsplat_creature: {{size: medium}}\nsplat_stats:\n  max_hp: {{value: 11}}\n---\n"
            )).expect("schema-2 fixture"),
        )
        .unwrap();
        let mut wolf = Participant::new(
            "wolf".into(),
            "enemies".into(),
            entry,
            &Mapping::new(),
            true,
        )
        .unwrap();
        let carry = Carry {
            rest_order: 0,
            from: "the bridge".into(),
            full: true,
            values: BTreeMap::from([(id, (1, 0))]),
            missing: Vec::new(),
        };
        assert_eq!(wolf.carry_over(&carry), None);
        assert_eq!(wolf.hp, Some(11));
    }
    #[test]
    fn initiative_digits_merge_into_one_undo_step() {
        let mut run = run();
        run.apply(Command::Initiative(0, Some(1))).unwrap();
        run.apply(Command::Initiative(0, Some(15))).unwrap();
        assert_eq!(run.history.len(), 1);
        assert_eq!(run.state.participants[0].initiative, Some(15));
        run.apply(Command::Initiative(1, Some(3))).unwrap();
        assert_eq!(run.history.len(), 2);
    }
    #[test]
    fn a_sheet_update_keeps_unrelated_steps_and_never_restores_old_sheet_values() {
        let (mut run, _, max) = matched_with_sheet(5);
        let earlier = run.history.len();
        let unrelated_hp = run.state.participants[1].hp;
        let unrelated_temp = run.state.participants[1].temporary_hp;
        run.apply(Command::Damage(1, 5)).unwrap();
        run.apply(Command::Damage(0, 1)).unwrap();
        connected_tick(&mut run);
        read_sheet(&mut run, max, 6); // Acknowledge the local damage first.
        read_sheet(&mut run, max, 8);
        assert_eq!(
            run.history.len(),
            earlier + 1,
            "the unrelated step stays and the character's own step is removed: {:?}; HP {:?}; conflicts {:?}",
            run.history_labels,
            run.state.participants[0].hp,
            run.syncs[&0].conflict_keys()
        );
        run.undo();
        assert_eq!(run.state.participants[1].hp, unrelated_hp);
        assert_eq!(run.state.participants[1].temporary_hp, unrelated_temp);
        assert_eq!(run.state.participants[0].hp, Some(max - 8));
        assert!(
            run.state
                .log
                .iter()
                .any(|line| line.contains("removed undo steps"))
        );
    }
    #[test]
    fn a_sheet_update_drops_an_atomic_step_and_logs_its_unsafe_earlier_steps() {
        let (mut run, _, max) = matched_with_sheet(5);
        run.apply(Command::Damage(1, 5)).unwrap();
        run.apply(Command::Several(vec![
            Command::Damage(0, 1),
            Command::Damage(1, 2),
        ]))
        .unwrap();
        run.apply(Command::Note(2, "Still here".into())).unwrap();
        connected_tick(&mut run);
        read_sheet(&mut run, max, 6);
        read_sheet(&mut run, max, 8);
        assert!(
            run.history_labels
                .iter()
                .all(|label| !label.contains("damage"))
        );
        assert!(run.undo_label().contains("note"));
        let hp = run.state.participants[1].hp;
        run.undo();
        assert_eq!(run.state.participants[0].hp, Some(max - 8));
        assert_eq!(run.state.participants[1].hp, hp);
        assert!(!run.state.participants[2].notes.contains("Still here"));
        assert!(
            run.state
                .log
                .iter()
                .any(|line| line.contains("removed undo steps") && line.contains("damage"))
        );
    }
    #[test]
    fn next_up_skips_absent_and_dates_are_calendar_dates() {
        let mut run = run();
        begin(&mut run);
        assert_eq!(run.next_up(), Some(1));
        run.apply(Command::Presence(1, false)).unwrap();
        assert_eq!(run.next_up(), Some(2));
        assert_eq!(calendar_date(0), "1970-01-01");
        assert_eq!(calendar_date(1_767_236_400), "2026-01-01");
    }
    #[test]
    fn failed_npc_roll_is_atomic_and_surprise_uses_disadvantage() {
        let mut run = run();
        run.state.random = Random::controlled([18, 4, 10, 12]);
        run.state.participants[0].initiative_mode = -1;
        run.state.participants[2].initiative_bonus = None;
        assert!(run.apply(Command::RollNpcs(Grouping::Individual)).is_err());
        assert!(
            run.state
                .participants
                .iter()
                .all(|p| p.initiative.is_none())
        );
        run.state.participants[2].initiative_bonus = Some(2);
        run.apply(Command::RollNpcs(Grouping::Individual)).unwrap();
        assert_eq!(run.state.participants[0].initiative, Some(6));
    }
    // ---- Notes beside a run ------------------------------------------------------------
    #[test]
    fn a_table_roll_keeps_its_row_in_the_log_and_undo_takes_it_back() {
        let mut run = run();
        let table = |typed| crate::roll_table::TableRoll {
            key: "gifts".into(),
            dice: "1d6".into(),
            typed,
            rows: vec![crate::roll_table::RollRow {
                min: 1,
                max: 6,
                text: "A potion".into(),
            }],
        };
        let before = run.state.log.len();
        let line = run.roll_table(table(None)).unwrap();
        assert!(
            line.starts_with("Rolled ") && line.contains(" on 1d6:") && line.ends_with("A potion"),
            "{line}"
        );
        assert_eq!(run.state.log.len(), before + 1);
        run.undo();
        assert_eq!(run.state.log.len(), before);
        // A player's own dice take no draw from the run's stream.
        assert_eq!(
            run.roll_table(table(Some(3))).unwrap(),
            "Typed 3 on 1d6: A potion"
        );
    }

    #[test]
    fn loot_rolls_after_the_end_and_keeps_each_lines_results_apart() {
        let mut run = run();
        run.setup_details = format!(
            "Ash falls.\n\n## Loot\n\n- 30 gp\n- Roll 2× gems\n- Roll gems\n\n{SETUP_HEADING}\n\n```yaml\nroster: []\n```"
        );
        let carries = |loot: &str| {
            Some(Changes {
                base: Base::default(),
                overrides: Overrides {
                    loot: Some(loot.into()),
                    ..Overrides::default()
                },
            })
        };
        run.state.participants[1].changes = carries("Iron key; Roll gems");
        run.state.participants[2].changes = carries("Roll gems");
        assert!(!run.state.participants[1].stat_changes());
        begin(&mut run);
        let open = |run: &Run| {
            run.holders()
                .into_iter()
                .filter(|holder| holder.open)
                .map(|holder| holder.key)
                .collect::<Vec<_>>()
        };
        assert_eq!(open(&run), ["encounter"]);
        run.apply(Command::Damage(1, 1000)).unwrap();
        assert!(run.state.participants[1].loot_ready());
        assert!(view::row_words_for_test(&run.state.participants[1]).ends_with("Loot"));
        run.apply(Command::End("Won".into())).unwrap();
        assert_eq!(open(&run), ["encounter", "slot1"]);
        let holders: Vec<_> = run.holders().into_iter().filter(|h| h.open).collect();
        // Both of the encounter's rolls and the guard's, with the standing creature's left out.
        assert_eq!(run.unrolled(&holders).len(), 3);
        let gems = || crate::roll_table::TableRoll {
            key: "gems".into(),
            dice: "1d2".into(),
            typed: None,
            rows: vec![crate::roll_table::RollRow {
                min: 1,
                max: 2,
                text: "Ruby".into(),
            }],
        };
        let roll = |place: &str, times| loot::LootRoll {
            place: place.into(),
            owner: "Ambush".into(),
            roll: gems(),
            times,
        };
        let before = run.state.log.len();
        // Roll all is one step.
        run.apply(Command::Loot(vec![
            roll(&loot::place("encounter", 1), 2),
            roll(&loot::place("slot1", 1), 1),
        ]))
        .unwrap();
        assert_eq!(run.state.log.len(), before + 3);
        assert_eq!(run.state.loot[&loot::place("encounter", 1)].len(), 2);
        // The other line on the same table has no result of its own yet.
        assert!(!run.state.loot.contains_key(&loot::place("encounter", 2)));
        assert_eq!(run.unrolled(&holders).len(), 1);
        assert!(
            run.loot_markdown(&holders)
                .contains("- Ruby\n- Ruby\n- gems (not rolled)")
        );
        assert!(
            run.apply(Command::Loot(vec![roll(&loot::place("encounter", 1), 1)]))
                .is_err()
        );
        // Undo takes all of them back, and the run is still ended.
        assert!(run.apply(Command::Next).is_err());
        run.undo();
        assert!(run.state.loot.is_empty());
        assert_eq!(run.state.log.len(), before);
        assert!(run.ended());
    }

    #[test]
    fn arrival_identity_and_round_survive_a_restart_and_undo_clears_them() {
        let mut run = run();
        begin(&mut run);
        let group = guard_group("Reserve", 2);
        let id = group.id;
        run.apply(Command::AddGroup(group, Some(9))).unwrap();
        assert_eq!(
            run.state
                .participants
                .last()
                .unwrap()
                .added_from
                .as_ref()
                .unwrap()
                .round,
            1
        );
        assert!(run.arrivals_in_run().contains_key(&id));
        let dir = tempfile::tempdir().unwrap();
        let campaign = stored(&mut run, dir.path());
        let reopened = Run::open(run.path().unwrap(), campaign).unwrap();
        assert_eq!(run.arrivals_in_run(), reopened.arrivals_in_run());
        assert_eq!(
            reopened
                .state
                .participants
                .last()
                .unwrap()
                .added_from
                .as_ref()
                .unwrap()
                .round,
            1
        );
        run.undo();
        assert!(!run.arrivals_in_run().contains_key(&id));
    }

    #[test]
    fn an_npc_joins_the_fight_under_their_own_name_and_undo_removes_them() {
        let mut run = run();
        let before = run.state.participants.len();
        let mut npc = creature();
        npc.name = "Ashori Deepcut".into();
        let group = crate::world::npc_group(&npc, &creature(), "enemies");
        assert!(group.problem.is_none(), "{:?}", group.problem);
        run.add_group(group, Some(11));
        assert!(run.error.is_none(), "{:?}", run.error);
        let added = run.state.participants.last().unwrap();
        assert_eq!(added.name, "Ashori Deepcut");
        assert_eq!(added.side, "enemies");
        assert_eq!(added.initiative, Some(11));
        assert_eq!(run.state.participants.len(), before + 1);
        assert!(run.side_names().contains(&"enemies".to_owned()));
        let npc = run.state.participants.last().unwrap();
        let words = view::row_words_for_test(npc);
        assert!(
            words.contains("as Guard") && !words.contains("Changed"),
            "{words}"
        );
        run.undo();
        assert_eq!(run.state.participants.len(), before);
    }

    // ---- Encounter groups ------------------------------------------------------------
    fn guard_group(name: &str, count: u64) -> Arc<RunGroup> {
        group_of(
            name,
            "enemies",
            vec![(creature(), count, Overrides::default())],
        )
    }
    #[test]
    fn run_icons_have_glyphs_at_each_capture_size() {
        use crate::design::{self, Type, icons};
        for (width, height) in [(960.0, 640.0), (1200.0, 800.0), (1920.0, 1080.0)] {
            let ctx = egui::Context::default();
            design::theme::install(&ctx);
            let _ = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(width, height),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        for role in [
                            Type::Small,
                            Type::Body,
                            Type::Strong,
                            Type::Name,
                            Type::Label,
                        ] {
                            let font = role.font(ui);
                            for icon in [
                                icons::DOWN,
                                icons::TEMP_HP,
                                icons::ABSENT,
                                icons::CONCENTRATING,
                                icons::MOVE_UP,
                                icons::MOVE_DOWN,
                                icons::WARN,
                                icons::EDIT,
                                icons::DRAG,
                                icons::CHANGED,
                                icons::OK,
                                icons::ADD,
                                icons::NEXT,
                                icons::SESSIONS,
                                icons::GROUP,
                                icons::OFFLINE,
                                icons::ERROR,
                                icons::INFO,
                                icons::KEYS,
                                icons::DICE,
                                icons::UNDO,
                                icons::CLOSE,
                                icons::SYNC_ON,
                                "■",
                                "◆",
                                "•",
                            ] {
                                assert!(
                                    ui.fonts_mut(|fonts| fonts.has_glyphs(&font, icon)),
                                    "Missing icon {icon:?} at {width}×{height}"
                                );
                            }
                        }
                    });
                },
            );
        }
    }
    fn group_of(name: &str, side: &str, members: Vec<(Entry, u64, Overrides)>) -> Arc<RunGroup> {
        Arc::new(RunGroup {
            id: Uuid::new_v4(),
            path: format!("{name}.md"),
            name: name.into(),
            side: side.into(),
            encounter: None,
            encounter_name: None,
            members: members
                .into_iter()
                .map(|(entry, count, overrides)| GroupMember {
                    entry: Arc::new(entry),
                    count,
                    overrides,
                })
                .collect(),
            notes: String::new(),
            xp: 0,
            xp_unknown: Vec::new(),
            unavailable: 0,
            unavailable_members: Vec::new(),
            warnings: Vec::new(),
            problem: None,
        })
    }
    fn changed(text: &str) -> Overrides {
        Overrides::parse(&serde_yaml_ng::from_str(text).unwrap()).unwrap()
    }
    fn beast() -> Entry {
        Entry::parse("beast.md", &splat::content::formats::convert("Fixture.md", &format!("---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: Beast\nsplat_creature: {{type: beast}}\nsplat_stats:\n  max_hp: {{value: 40}}\n  armor_class: {{value: 12}}\n  initiative: {{value: -1}}\n  speed: {{value: 40}}\n---\n", Uuid::new_v4())).expect("schema-2 fixture")).unwrap()
    }
    #[test]
    fn group_summary_counts_members_and_changes() {
        let group = group_of(
            "Wave",
            "rivals",
            vec![(beast(), 3, changed("armor_class: {add: 1}"))],
        );
        assert_eq!(
            group.summary(),
            "Rivals · 3 members · XP 0 · Changed · 3 × Beast"
        );
    }

    fn names(run: &Run, range: std::ops::Range<usize>) -> Vec<String> {
        run.state.participants[range]
            .iter()
            .map(|p| p.name.clone())
            .collect()
    }
    #[test]
    fn a_group_added_in_setup_gets_fresh_keys_full_hp_and_one_roll() {
        let mut run = run();
        run.state.random = Random::controlled([10]);
        run.apply(Command::AddGroup(guard_group("Wave", 2), None))
            .unwrap();
        assert_eq!(run.state.participants.len(), 5);
        let added = &run.state.participants[3..];
        assert_eq!(names(&run, 3..5), vec!["Guard 4", "Guard 5"]);
        for p in added {
            assert_eq!(p.hp, p.max_hp, "full HP, not the creature's starting 17");
            assert_eq!(p.side, "enemies");
            assert_eq!(p.initiative, Some(12), "10 and the bonus of 2");
            assert!(p.npc && p.present);
            assert_eq!(p.added_from.as_ref().unwrap().group, "Wave");
        }
        assert_ne!(added[0].key, added[1].key);
        assert!(
            run.state.participants[..3]
                .iter()
                .all(|p| p.key != added[0].key)
        );
        // Setup keeps the order sorted, with no total last.
        assert_eq!(run.state.order[..2], [3, 4]);
        let log = run.state.log.last().unwrap();
        assert!(
            log.contains("Wave") && log.contains("Guard 4") && log.contains("10 + 2 = 12"),
            "{log}"
        );
        // Combat can begin with them in it.
        for i in 0..3 {
            run.apply(Command::Initiative(i, Some(5))).unwrap();
        }
        run.apply(Command::Begin).unwrap();
        assert_eq!(run.current(), Some(3));
    }
    #[test]
    fn a_group_added_mid_round_takes_its_place_and_the_current_turn_stays() {
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Next).unwrap();
        assert_eq!(run.current(), Some(1));
        run.apply(Command::AddGroup(guard_group("Later", 2), Some(12)))
            .unwrap();
        assert_eq!(run.state.order, vec![0, 1, 3, 4, 2]);
        assert_eq!(run.current(), Some(1), "the current turn stays current");
        assert_eq!(run.next_up(), Some(3), "they act this round");
        // A higher total is past this round's cursor: they wait for the next round.
        run.apply(Command::AddGroup(guard_group("Earlier", 1), Some(18)))
            .unwrap();
        assert_eq!(run.state.order, vec![0, 5, 1, 3, 4, 2]);
        assert_eq!(run.current(), Some(1));
        let seen: Vec<_> = (0..6)
            .map(|_| {
                run.apply(Command::Next).unwrap();
                (run.state.round, run.current().unwrap())
            })
            .collect();
        assert_eq!(
            seen,
            vec![(1, 3), (1, 4), (1, 2), (2, 0), (2, 5), (2, 1)],
            "each acts once a round, in place"
        );
    }
    #[test]
    fn a_tie_with_the_current_turn_goes_after_it_and_keeps_the_group_together() {
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Next).unwrap();
        run.apply(Command::AddGroup(guard_group("Tied", 2), Some(15)))
            .unwrap();
        assert_eq!(run.state.order, vec![0, 1, 3, 4, 2]);
        assert_eq!(run.next_up(), Some(3));
    }
    #[test]
    fn arrival_tie_can_move_before_and_undo_keeps_the_current_turn() {
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Next).unwrap();
        run.apply(Command::AddGroup(guard_group("Tied", 2), Some(15)))
            .unwrap();
        assert!(run.state.log.last().unwrap().contains("Tied with"));
        assert!(
            run.state
                .log
                .last()
                .unwrap()
                .contains("at\u{a0}15, placed after."),
            "iter6-B-FT4: the tie uses a comma"
        );
        assert!(!run.undo_label().contains("placed after"));
        let after = run.state.order.clone();
        let current = run.current();
        run.apply(Command::MoveArrivalBefore(3, 1)).unwrap();
        assert_eq!(
            run.undo_label(),
            format!(
                "Undo: move {} before {}",
                run.state.participants[3].name, run.state.participants[1].name
            )
        );
        assert!(
            run.state
                .log
                .last()
                .unwrap()
                .starts_with(&format!("{} moved before ", run.state.participants[3].name))
        );
        assert_eq!(run.state.order, vec![0, 3, 4, 1, 2]);
        assert_eq!(run.current(), current);
        run.undo();
        assert_eq!(run.state.order, after);
        assert_eq!(run.current(), current);
    }

    #[test]
    fn sheet_notice_counts_characters_in_a_combined_update() {
        let mut run = run();
        let first = run.state.participants[0].name.clone();
        let second = run.state.participants[1].name.clone();
        run.note_sync_changes(
            &[format!(
                "{first} HP 20 → 18, temp HP 2 → 0, {second} HP 20 → 10"
            )],
            &[],
        );
        assert_eq!(run.notice_changes, 2);
        assert_eq!(
            run.notice_summary.as_deref(),
            Some(
                format!("{first}'s sheet changed: HP 20 → 18, temp HP 2 → 0 and 1 more sheet")
                    .as_str()
            )
        );
        assert!(
            run.state
                .log
                .last()
                .unwrap()
                .contains(&format!("{second} HP 20 → 10"))
        );
    }

    #[test]
    fn moving_an_arrival_before_a_shared_roll_keeps_both_blocks_and_waits() {
        let mut run = run();
        begin(&mut run);
        run.apply(Command::AddGroup(guard_group("First wave", 2), Some(15)))
            .unwrap();
        run.apply(Command::AddGroup(guard_group("Second wave", 2), Some(15)))
            .unwrap();
        assert_arrival_wraps(&format!(
            "{} ·\u{a0}acts next round if moved before",
            run.state.log.last().unwrap()
        ));
        assert!(
            run.state
                .log
                .last()
                .unwrap()
                .contains("Tied with First wave")
        );
        // The last tied member used to split First wave when targeted.
        while run.current() != Some(3) {
            run.apply(Command::Next).unwrap();
        }
        let current = run.current();
        let order = run.state.order.clone();
        run.apply(Command::MoveArrivalBefore(5, 4)).unwrap();
        assert_arrival_wraps(run.state.log.last().unwrap());
        assert_eq!(run.state.order, vec![0, 1, 5, 6, 3, 4, 2]);
        assert_eq!(run.current(), current);
        assert!(
            run.state
                .log
                .last()
                .unwrap()
                .contains("moved before First wave at\u{a0}15 ·\u{a0}acts next round")
        );
        run.undo();
        assert_eq!(run.state.order, order);
        assert_eq!(run.current(), current);
    }

    #[test]
    fn the_same_group_can_be_added_twice_with_separate_creatures() {
        let mut run = run();
        begin(&mut run);
        let wave = guard_group("Wave", 2);
        run.state.random = Random::controlled([3, 9]);
        run.apply(Command::AddGroup(wave.clone(), None)).unwrap();
        run.apply(Command::AddGroup(wave, None)).unwrap();
        assert_eq!(run.state.participants.len(), 7);
        assert_eq!(
            names(&run, 3..7),
            vec!["Guard 4", "Guard 5", "Guard 6", "Guard 7"]
        );
        let keys: BTreeMap<_, _> = run
            .state
            .participants
            .iter()
            .map(|p| (&p.key, ()))
            .collect();
        assert_eq!(keys.len(), 7, "every key is its own");
        assert_eq!(run.state.participants[3].initiative, Some(5));
        assert_eq!(
            run.state.participants[5].initiative,
            Some(11),
            "its own roll"
        );
        run.apply(Command::Damage(3, 6)).unwrap();
        assert_eq!(
            run.state.participants[3].hp,
            Some(17),
            "3 temporary HP took some"
        );
        assert_eq!(run.state.participants[5].hp, Some(20), "its own HP");
    }
    #[test]
    fn a_display_name_cannot_share_a_number_given_to_a_duplicate() {
        let entry = creature();
        let slot = |key: &str, overrides: &Overrides| {
            Participant::with_overrides(
                key.into(),
                "enemies".into(),
                entry.clone(),
                &Mapping::new(),
                overrides,
                true,
            )
            .unwrap()
        };
        let plain = Overrides::default();
        let run = Run::new(
            "Clash".into(),
            vec![
                slot("a", &plain),
                slot("b", &plain),
                slot("c", &changed("{name: Guard 2}")),
            ],
            String::new(),
        )
        .unwrap();
        assert_eq!(names(&run, 0..3), vec!["Guard 1", "Guard 3", "Guard 2"]);
    }
    #[test]
    fn names_in_one_addition_never_match_each_other_and_huge_numbers_do_not_overflow() {
        let mut run = run();
        run.apply(Command::AddGroup(
            group_of(
                "Clash",
                "enemies",
                vec![
                    (creature(), 2, changed("{name: Rival}")),
                    (creature(), 1, changed("{name: Rival 2}")),
                ],
            ),
            Some(3),
        ))
        .unwrap();
        let added = names(&run, 3..6);
        let unique: std::collections::BTreeSet<_> = added.iter().collect();
        assert_eq!(unique.len(), 3, "{added:?}");
        run.apply(Command::AddGroup(
            group_of(
                "Huge",
                "enemies",
                vec![(creature(), 1, changed("{name: X 18446744073709551615}"))],
            ),
            Some(3),
        ))
        .unwrap();
        run.apply(Command::AddGroup(
            group_of(
                "After",
                "enemies",
                vec![(creature(), 1, changed("{name: X}"))],
            ),
            Some(3),
        ))
        .unwrap();
        assert_eq!(names(&run, 6..8), vec!["X 18446744073709551615", "X"]);
    }
    #[test]
    fn numbers_go_on_from_names_already_in_the_fight_and_a_lone_name_stays_plain() {
        let mut run = Run::new(
            "One".into(),
            vec![
                Participant::new("a".into(), "enemies".into(), beast(), &Mapping::new(), true)
                    .unwrap(),
            ],
            String::new(),
        )
        .unwrap();
        run.apply(Command::AddGroup(
            group_of("Pack", "enemies", vec![(beast(), 2, Overrides::default())]),
            Some(3),
        ))
        .unwrap();
        assert_eq!(names(&run, 0..3), vec!["Beast", "Beast 2", "Beast 3"]);
        run.apply(Command::AddGroup(
            group_of(
                "Alone",
                "enemies",
                vec![(creature(), 1, Overrides::default())],
            ),
            Some(3),
        ))
        .unwrap();
        assert_eq!(
            run.state.participants[3].name, "Guard",
            "no other Guard, so no number"
        );
        run.apply(Command::AddGroup(
            group_of(
                "Renamed",
                "enemies",
                vec![(creature(), 2, changed("{name: Rival}"))],
            ),
            Some(3),
        ))
        .unwrap();
        assert_eq!(names(&run, 4..6), vec!["Rival 1", "Rival 2"]);
    }
    #[test]
    fn undo_takes_a_group_back_whole_and_its_dice_with_it() {
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Next).unwrap();
        let before_log = run.state.log.clone();
        let before_order = run.state.order.clone();
        run.state.random = Random::controlled([10, 4]);
        run.apply(Command::AddGroup(guard_group("Wave", 2), None))
            .unwrap();
        assert_eq!(run.undo_label(), "Undo: add Wave");
        run.selected = 4;
        run.setup_inputs.insert(4, "7".into());
        run.syncs
            .insert(4, crate::combat_sync::CombatSync::default());
        run.sheet_for = Some(4);
        run.undo();
        assert_eq!(run.state.participants.len(), 3);
        assert_eq!(run.state.order, before_order);
        assert_eq!(run.state.log, before_log);
        assert_eq!(run.current(), Some(1));
        assert_eq!(
            run.selected, 1,
            "a row that went away moves to the current turn"
        );
        assert!(run.setup_inputs.is_empty() && run.syncs.is_empty());
        assert_eq!(run.sheet_for, None);
        assert_eq!(run.undo_label(), "Undo: next turn");
        // Nothing still points at the row, in any view.
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                run.show(ui);
            });
        });
        // Adding it again is a new step, with new keys.
        run.state.random = Random::controlled([10, 4]);
        run.apply(Command::AddGroup(guard_group("Wave", 1), None))
            .unwrap();
        assert_eq!(run.state.participants[3].initiative, Some(12));
        // Undo steps back through later steps first, in order.
        run.apply(Command::Next).unwrap();
        run.undo();
        run.undo();
        assert_eq!(run.state.participants.len(), 3);
    }
    fn known_wolf() -> Arc<RunCreature> {
        let mut entry = creature();
        entry.name = "Wolf".into();
        Arc::new(RunCreature {
            entry: Arc::new(entry),
            lower: "wolf".into(),
            detail: "A wolf".into(),
        })
    }
    fn form(query: &str, hp: &str) -> ad_hoc::AdHoc {
        ad_hoc::AdHoc {
            query: query.into(),
            hp: hp.into(),
            ..Default::default()
        }
    }
    fn add_from(run: &mut Run, form: &ad_hoc::AdHoc, known: &[Arc<RunCreature>]) {
        let checked = form.check(known).unwrap();
        run.apply(Command::AddCreature(form.group(checked), form.total()))
            .unwrap();
    }

    #[test]
    fn a_creature_made_up_at_the_table_joins_is_undone_and_stays_in_the_saved_run() {
        let dir = tempfile::tempdir().unwrap();
        let mut run = run();
        begin(&mut run);
        let before = run.state.participants.len();
        let mut ghost = form("  Ghost of the Mill ", "22");
        ghost.ac = "14".into();
        ghost.total = "9".into();
        ghost.count = "2".into();
        add_from(&mut run, &ghost, &[]);
        let p = &run.state.participants[before];
        assert_eq!(p.name, "Ghost of the Mill 1");
        assert_eq!(
            (p.hp, p.max_hp, p.armor_class),
            (Some(22), Some(22), Some(14))
        );
        assert_eq!(p.initiative, Some(9));
        assert_eq!(p.side, "enemies");
        assert!(p.changes.is_none(), "nothing here replaces a stat block");
        assert_eq!(p.added_from.as_ref().unwrap().group, "");
        assert!(run.state.log.last().unwrap().starts_with(ONE_ADDED));
        run.undo();
        assert_eq!(run.state.participants.len(), before);
        add_from(&mut run, &ghost, &[]);
        let campaign = stored(&mut run, dir.path());
        let reopened = close_and_reopen(&mut run, campaign);
        let p = &reopened.state.participants[before + 1];
        assert_eq!(p.name, "Ghost of the Mill 2");
        assert_eq!(p.max_hp, Some(22));
        assert_eq!(p.definition.name, "Ghost of the Mill");
    }

    #[test]
    fn a_creature_found_in_the_campaign_keeps_its_values_unless_the_dm_types_others() {
        let known = [known_wolf()];
        let mut run = run();
        begin(&mut run);
        let before = run.state.participants.len();
        // The whole name typed is the same as choosing it, and has what it needs.
        let mut wolf = form("WOLF", "");
        wolf.total = "5".into();
        add_from(&mut run, &wolf, &known);
        let p = &run.state.participants[before];
        assert_eq!(p.max_hp, Some(20));
        assert!(p.changes.is_none());
        let mut changed = form("", "31");
        changed.picked = Some(known[0].clone());
        changed.ac = "9".into();
        changed.total = "5".into();
        add_from(&mut run, &changed, &known);
        let p = run.state.participants.last().unwrap();
        assert_eq!((p.max_hp, p.armor_class), (Some(31), Some(9)));
        let changes = p.changes.as_ref().unwrap();
        assert_eq!(
            changes.base.max_hp,
            Some(20),
            "the change shows what it replaced"
        );
        // The same number the entry has is no change.
        let mut same = form("", "20");
        same.picked = Some(known[0].clone());
        same.total = "5".into();
        add_from(&mut run, &same, &known);
        assert!(run.state.participants.last().unwrap().changes.is_none());
    }

    #[test]
    fn a_search_shows_the_whole_name_first_then_names_that_start_with_it() {
        let named = |name: &str| {
            Arc::new(RunCreature {
                entry: Arc::new(creature()),
                lower: name.into(),
                detail: String::new(),
            })
        };
        let known = [
            named("cinder wolf"),
            named("dire wolf"),
            named("wolf spider"),
            named("wolf"),
        ];
        let found: Vec<_> = ad_hoc::matches_for(&known, "wolf")
            .iter()
            .map(|creature| creature.lower.as_str())
            .collect();
        assert_eq!(found, ["wolf", "wolf spider", "cinder wolf", "dire wolf"]);
    }

    #[test]
    fn an_ended_fight_takes_no_arrivals_and_an_undo_of_the_ending_does() {
        let mut run = run();
        assert!(run.takes_arrivals(), "setup can take them");
        begin(&mut run);
        assert!(run.takes_arrivals());
        run.apply(Command::End("Won".into())).unwrap();
        assert!(!run.takes_arrivals());
        run.undo();
        assert!(run.takes_arrivals());
    }

    #[test]
    fn a_creature_form_says_what_it_still_needs() {
        let known = [known_wolf()];
        let error = |form: ad_hoc::AdHoc| form.check(&known).err().unwrap();
        assert!(error(form("", "")).contains("Find or name"));
        assert!(error(form("Ghost", "")).contains("Type its HP"));
        assert!(error(form("Ghost", "0")).contains("HP must be"));
        let mut many = form("Ghost", "5");
        many.count = "0".into();
        assert!(error(many).contains("Count must be"));
        let mut armor = form("Ghost", "5");
        armor.ac = "1001".into();
        assert!(error(armor).contains("AC must be"));
        let mut blank = creature();
        blank.fields.clear();
        let unknown = [Arc::new(RunCreature {
            entry: Arc::new(blank),
            lower: "blank".into(),
            detail: String::new(),
        })];
        let mut no_hp = form("", "");
        no_hp.picked = Some(unknown[0].clone());
        let error = no_hp.check(&unknown).err().unwrap();
        assert!(error.contains("no maximum HP"), "{error}");
    }

    #[test]
    fn an_added_creature_counts_as_the_dms_step_and_follows_the_side_rule() {
        let mut run = run();
        run.initiative.grouping = Grouping::BySide;
        begin(&mut run);
        let side_total = run.state.participants[0].initiative;
        let fact = run.timeline_fact();
        add_from(&mut run, &form("Ghost", "5"), &[]);
        assert!(run.timeline_fact().played_since(&fact));
        assert_eq!(
            run.state.participants.last().unwrap().initiative,
            side_total,
            "by side, it takes its side's initiative with no roll"
        );
    }

    #[test]
    fn a_saved_run_reopens_with_added_groups_and_their_changes() {
        let dir = tempfile::tempdir().unwrap();
        let mut run = run();
        begin(&mut run);
        run.apply(Command::AddGroup(
            group_of(
                "Rivals",
                "rivals",
                vec![(
                    creature(),
                    1,
                    changed(
                        "{name: Captain, max_hp: {add: 10}, attack_bonus: {add: 1}, note: Sealed}",
                    ),
                )],
            ),
            Some(12),
        ))
        .unwrap();
        run.apply(Command::Next).unwrap();
        run.apply(Command::Damage(3, 4)).unwrap();
        let campaign = stored(&mut run, dir.path());
        let mut reopened = close_and_reopen(&mut run, campaign);
        let p = &reopened.state.participants[3];
        assert_eq!(p.name, "Captain");
        assert_eq!(p.side, "rivals");
        assert_eq!((p.hp, p.max_hp), (Some(29), Some(30)));
        assert_eq!(p.initiative, Some(12));
        assert_eq!(p.added_from.as_ref().unwrap().group, "Rivals");
        let changes = p.changes.as_ref().unwrap();
        assert_eq!(changes.base.max_hp, Some(20));
        assert_eq!(changes.overrides.attack_bonus, Some(1));
        assert_eq!(reopened.state.order, run.state.order);
        assert!(
            reopened
                .state
                .log
                .iter()
                .any(|line| line.contains("Rivals"))
        );
        // A step after the reopening saves and opens again.
        reopened.apply(Command::Next).unwrap();
        reopened.save().unwrap();
        let again = Run::open(reopened.path().unwrap(), campaign).unwrap();
        assert_eq!(again.state.participants[3].name, "Captain");
        assert_eq!(again.current(), reopened.current());
    }
    #[test]
    fn a_run_offers_its_encounters_groups_and_those_for_any_encounter() {
        let (arena, other) = (Uuid::new_v4(), Uuid::new_v4());
        let owned = |name: &str, owner: Uuid| {
            let mut group = (*guard_group(name, 1)).clone();
            group.encounter = Some(owner);
            Arc::new(group)
        };
        let groups = [
            guard_group("Free", 1),
            owned("Mine", arena),
            owned("Theirs", other),
        ];
        let names =
            |list: &[Arc<RunGroup>]| list.iter().map(|g| g.name.clone()).collect::<Vec<_>>();
        let offered = OfferedGroups::of(&groups, Some(arena));
        assert_eq!(names(&offered.own), ["Mine"]);
        assert_eq!(names(&offered.any), ["Free"]);
        assert!(
            offered.other.is_empty(),
            "another encounter's group is not offered"
        );
        // A save from before this cannot say which is its own, so it hides none.
        let offered = OfferedGroups::of(&groups, None);
        assert!(offered.own.is_empty());
        assert_eq!(names(&offered.any), ["Free"]);
        assert_eq!(names(&offered.other), ["Mine", "Theirs"]);
    }

    #[test]
    fn a_saved_run_keeps_its_encounter_and_an_older_save_has_none() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Uuid::new_v4();
        let encounter = Uuid::new_v4();
        let participant = Participant::new(
            "guard".into(),
            "enemies".into(),
            creature(),
            &Mapping::new(),
            true,
        )
        .unwrap();
        let mut run = Run::new("Arena".into(), vec![participant], String::new()).unwrap();
        run.came_from(encounter);
        run.configure_storage(campaign, dir.path(), Initiative::default())
            .unwrap();
        run.autosave();
        let path = run.storage.as_ref().unwrap().path.clone();
        assert_eq!(Run::open(&path, campaign).unwrap().source, Some(encounter));

        let mut saved: serde_json::Value =
            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        saved.as_object_mut().unwrap().remove("source");
        fs::write(&path, serde_json::to_vec(&saved).unwrap()).unwrap();
        assert_eq!(Run::open(&path, campaign).unwrap().source, None);
    }

    fn mixed_group() -> Arc<RunGroup> {
        group_of(
            "Mixed",
            "enemies",
            vec![
                (creature(), 1, Overrides::default()),
                (beast(), 2, Overrides::default()),
            ],
        )
    }
    #[test]
    fn identical_rolls_once_for_a_mixed_group_with_the_first_members_bonus() {
        let mut run = run();
        run.initiative.grouping = Grouping::Identical;
        // Only one die: a second roll would fail.
        run.state.random = Random::controlled([10]);
        run.apply(Command::AddGroup(mixed_group(), None)).unwrap();
        for p in &run.state.participants[3..] {
            assert_eq!(p.initiative, Some(12), "{}", p.name);
        }
    }
    #[test]
    fn identical_keeps_a_group_as_one_roll_when_npcs_are_rolled_later() {
        let mut run = run();
        run.apply(Command::AddGroup(mixed_group(), Some(1)))
            .unwrap();
        run.apply(Command::AddGroup(mixed_group(), Some(2)))
            .unwrap();
        // One for the three Guards of the roster, then one for each addition.
        run.state.random = Random::controlled([5, 8, 11]);
        run.apply(Command::RollNpcs(Grouping::Identical)).unwrap();
        let totals: Vec<_> = run
            .state
            .participants
            .iter()
            .map(|p| p.initiative.unwrap())
            .collect();
        assert_eq!(&totals[..3], [7, 7, 7], "the roster's identical Guards");
        assert_eq!(
            &totals[3..6],
            [10, 10, 10],
            "the first addition, not with its like"
        );
        assert_eq!(
            &totals[6..9],
            [13, 13, 13],
            "the second addition, apart from the first"
        );
    }
    #[test]
    fn individual_rolls_each_member_alone_and_a_typed_total_still_applies_to_all() {
        let mut run = run();
        run.initiative.grouping = Grouping::Individual;
        run.state.random = Random::controlled([10, 6, 15]);
        run.apply(Command::AddGroup(mixed_group(), None)).unwrap();
        let totals: Vec<_> = run.state.participants[3..]
            .iter()
            .map(|p| p.initiative)
            .collect();
        assert_eq!(totals, vec![Some(12), Some(5), Some(14)]);
        run.apply(Command::AddGroup(mixed_group(), Some(9)))
            .unwrap();
        assert!(
            run.state.participants[6..]
                .iter()
                .all(|p| p.initiative == Some(9))
        );
    }
    #[test]
    fn by_side_joins_the_side_or_rolls_once_for_a_side_with_none() {
        let mut run = run();
        run.initiative.grouping = Grouping::BySide;
        begin(&mut run);
        run.state.random = Random::controlled([]);
        run.apply(Command::AddGroup(mixed_group(), None)).unwrap();
        assert!(
            run.state.participants[3..]
                .iter()
                .all(|p| p.initiative == Some(20)),
            "the first total on its side, with no die"
        );
        run.state.random = Random::controlled([6]);
        run.apply(Command::AddGroup(
            group_of(
                "Split",
                "rivals",
                vec![
                    (creature(), 1, Overrides::default()),
                    (beast(), 1, Overrides::default()),
                    (creature(), 1, changed("{side: enemies}")),
                ],
            ),
            None,
        ))
        .unwrap();
        let p = &run.state.participants;
        assert_eq!(p[6].initiative, Some(8), "the new side rolled once");
        assert_eq!(p[7].initiative, Some(8));
        assert_eq!(p[8].initiative, Some(20), "this member's own side");
        assert!(
            run.state
                .log
                .last()
                .unwrap()
                .contains("Joins enemies at\u{a0}20")
        );
    }
    #[test]
    fn explicit_makes_each_addition_its_own_initiative_group() {
        let mut run = run();
        run.initiative.grouping = Grouping::Explicit;
        run.state.random = Random::controlled([10, 3]);
        run.apply(Command::AddGroup(mixed_group(), None)).unwrap();
        run.apply(Command::AddGroup(mixed_group(), None)).unwrap();
        let group = |i: usize| run.state.participants[i].initiative_group.clone();
        assert!(group(3).is_some());
        assert_eq!(group(3), group(5));
        assert_ne!(group(3), group(6));
        assert_eq!(run.state.participants[5].initiative, Some(12));
        assert_eq!(run.state.participants[6].initiative, Some(5));
        // The later roll keeps them so.
        run.state.participants[..3]
            .iter_mut()
            .for_each(|p| p.npc = false);
        run.state.random = Random::controlled([1, 2]);
        run.apply(Command::RollNpcs(Grouping::Explicit)).unwrap();
        assert_eq!(run.state.participants[3].initiative, Some(3));
        assert_eq!(run.state.participants[5].initiative, Some(3));
        assert_eq!(run.state.participants[8].initiative, Some(4));
    }
    #[test]
    fn a_typed_total_uses_no_dice_under_any_rule() {
        for grouping in Grouping::ALL {
            let mut run = run();
            run.initiative.grouping = grouping;
            run.state.random = Random::controlled([]);
            run.apply(Command::AddGroup(mixed_group(), Some(17)))
                .unwrap();
            assert!(
                run.state.participants[3..]
                    .iter()
                    .all(|p| p.initiative == Some(17)),
                "{grouping:?}"
            );
            assert!(run.state.log.last().unwrap().contains("entered by the DM"));
        }
    }
    #[test]
    fn a_group_that_cannot_join_leaves_the_run_as_it_was() {
        let check = |run: &mut Run, group: Arc<RunGroup>, total: Option<i64>, words: &str| {
            let (log, count) = (run.state.log.clone(), run.state.participants.len());
            let steps = run.history.len();
            let error = run.apply(Command::AddGroup(group, total)).unwrap_err();
            assert!(error.contains(words), "{error}");
            assert_eq!(run.state.log, log);
            assert_eq!(run.state.participants.len(), count);
            assert_eq!(run.history.len(), steps, "no step to undo");
        };
        let mut run = run();
        begin(&mut run);
        let mut stray = (*guard_group("Stray", 1)).clone();
        stray.problem = Some("Member 1: this creature is unavailable.".into());
        check(&mut run, Arc::new(stray), Some(4), "unavailable");
        let mut empty = (*guard_group("Empty", 1)).clone();
        empty.members.clear();
        check(&mut run, Arc::new(empty), Some(4), "no members");
        let unknown = Entry::parse("u.md", &splat::content::formats::convert("Fixture.md", &format!("---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: Unknown\nsplat_creature: {{}}\n---\n", Uuid::new_v4())).expect("schema-2 fixture")).unwrap();
        check(
            &mut run,
            group_of(
                "NoHp",
                "enemies",
                vec![(unknown.clone(), 1, Overrides::default())],
            ),
            Some(4),
            "no maximum HP",
        );
        // A max_hp override gives such a creature one. It then has no bonus to roll with.
        let given = group_of(
            "Given",
            "enemies",
            vec![(unknown, 1, changed("{max_hp: {set: 9}}"))],
        );
        check(&mut run, given.clone(), None, "no initiative bonus");
        run.apply(Command::AddGroup(given, Some(4))).unwrap();
        assert_eq!(run.state.participants[3].hp, Some(9));
        check(
            &mut run,
            group_of(
                "Weak",
                "enemies",
                vec![(creature(), 1, changed("{max_hp: {add: -20}}"))],
            ),
            Some(4),
            "Maximum HP",
        );
        // A saved run holds at most so many, so a run cannot grow past what it can reopen.
        check(&mut run, guard_group("Many", 1997), Some(4), "2000");
        // Linked characters have one sheet each.
        let mut linked = creature();
        linked.fields.insert(
            "splat_external".into(),
            serde_yaml_ng::from_str("{provider: dndbeyond}").unwrap(),
        );
        check(
            &mut run,
            group_of(
                "Linked",
                "party",
                vec![(linked, 1, changed("{conditions: [Prone]}"))],
            ),
            Some(4),
            "D&D Beyond",
        );
        run.apply(Command::End("Done".into())).unwrap();
        check(&mut run, guard_group("Late", 1), Some(4), "ended");
    }
    #[test]
    fn a_failed_roll_leaves_the_dice_where_they_were() {
        let mut run = run();
        begin(&mut run);
        run.state.random = Random::controlled([]);
        assert!(
            run.apply(Command::AddGroup(guard_group("Wave", 1), None))
                .is_err()
        );
        assert_eq!(run.state.participants.len(), 3);
    }

    // ---- Overrides -------------------------------------------------------------------
    fn overridden(text: &str, starting: &str) -> Result<Participant, String> {
        Participant::with_overrides(
            "slot".into(),
            "enemies".into(),
            creature(),
            &serde_yaml_ng::from_str(starting).unwrap(),
            &changed(text),
            true,
        )
    }
    #[test]
    fn no_overrides_behave_and_save_exactly_as_before() {
        let plain = Participant::new(
            "slot".into(),
            "enemies".into(),
            creature(),
            &Mapping::new(),
            true,
        )
        .unwrap();
        assert!(plain.changes.is_none() && plain.added_from.is_none());
        assert_eq!(
            (plain.hp, plain.max_hp, plain.armor_class),
            (Some(17), Some(20), Some(16))
        );
        let saved = serde_json::to_value(&plain).unwrap();
        assert!(saved.get("changes").is_none(), "a save has no new field");
        assert!(saved.get("added_from").is_none());
    }
    #[test]
    fn each_override_reaches_the_participant_and_the_entry_is_not_touched() {
        let p = overridden(
            "{name: Rival, max_hp: {add: 10}, armor_class: {set: 18}, initiative_bonus: {add: 3}, attack_bonus: {add: 1}, damage_bonus: {add: 1}, speed: {set: 40}, side: rivals, conditions: [Prone], note: Sealed}",
            "{}",
        )
        .unwrap();
        assert_eq!(p.name, "Rival");
        assert_eq!(p.definition.name, "Guard", "the entry keeps its own name");
        assert_eq!(p.max_hp, Some(30));
        assert_eq!(p.hp, Some(30), "a changed maximum starts at that maximum");
        assert_eq!(p.armor_class, Some(18));
        assert_eq!(p.initiative_bonus, Some(5));
        assert_eq!(p.side, "rivals");
        assert_eq!(p.conditions, vec!["Poisoned", "Prone"], "added to its own");
        assert_eq!(stat(&p.definition, "max_hp"), Some(20));
        let changes = p.changes.as_ref().unwrap();
        assert_eq!(
            changes.base.max_hp,
            Some(20),
            "the card keeps the old value"
        );
        assert_eq!(changes.base.side, "enemies");
        assert_eq!(
            changes.overrides.table_bonuses(&changes.base),
            vec!["Attack +1", "Damage +1", "Speed 40 ft"]
        );
        // What sync compares with is where the participant began.
        assert_eq!(p.started.as_ref().unwrap().hp, Some(30));
        assert_eq!(p.started.as_ref().unwrap().conditions.len(), 2);
    }
    #[test]
    fn a_slots_starting_hp_stands_only_when_it_fits_the_changed_maximum() {
        let p = overridden("{max_hp: {add: 10}}", "{hp: 12}").unwrap();
        assert_eq!((p.hp, p.max_hp), (Some(12), Some(30)));
        let error = overridden("{max_hp: {set: 10}}", "{hp: 12}").err().unwrap();
        assert!(error.contains("slot") && error.contains("above"), "{error}");
        // With no change to the maximum, starting HP keeps its old meaning.
        assert_eq!(
            overridden("{armor_class: {set: 3}}", "{}").unwrap().hp,
            Some(17)
        );
        // A duplicate condition is one condition.
        let p = overridden("{conditions: [poisoned]}", "{}").unwrap();
        assert_eq!(p.conditions, vec!["Poisoned"]);
    }
    #[test]
    fn a_changed_creature_is_no_longer_identical_to_its_like() {
        let entry = creature();
        let mut members: Vec<_> = (0..3)
            .map(|i| {
                Participant::new(
                    format!("s{i}"),
                    "enemies".into(),
                    entry.clone(),
                    &Mapping::new(),
                    true,
                )
                .unwrap()
            })
            .collect();
        members[0] = Participant::with_overrides(
            "s0".into(),
            "enemies".into(),
            entry,
            &Mapping::new(),
            &changed("{attack_bonus: {add: 1}}"),
            true,
        )
        .unwrap();
        let mut run = Run::new("Arena".into(), members, String::new()).unwrap();
        run.state.random = Random::controlled([3, 9]);
        run.apply(Command::RollNpcs(Grouping::Identical)).unwrap();
        let totals: Vec<_> = run
            .state
            .participants
            .iter()
            .map(|p| p.initiative)
            .collect();
        assert_eq!(totals, vec![Some(5), Some(11), Some(11)]);
    }
    #[test]
    fn a_saved_run_keeps_a_roster_slots_overrides() {
        let dir = tempfile::tempdir().unwrap();
        let p = overridden(
            "{max_hp: {add: 10}, damage_bonus: {add: 2}, side: rivals}",
            "{}",
        )
        .unwrap();
        let mut run = Run::new("Arena".into(), vec![p], String::new()).unwrap();
        run.apply(Command::Initiative(0, Some(9))).unwrap();
        run.apply(Command::Begin).unwrap();
        let campaign = stored(&mut run, dir.path());
        let reopened = close_and_reopen(&mut run, campaign);
        let p = &reopened.state.participants[0];
        assert_eq!(p.max_hp, Some(30));
        assert_eq!(p.side, "rivals");
        let changes = p.changes.as_ref().unwrap();
        assert_eq!(changes.overrides.damage_bonus, Some(2));
        assert_eq!(changes.base.max_hp, Some(20));
    }
    #[test]
    fn a_save_from_before_groups_and_overrides_still_opens() {
        let dir = tempfile::tempdir().unwrap();
        let mut run = run();
        begin(&mut run);
        let campaign = stored(&mut run, dir.path());
        let path = run.path().unwrap().to_path_buf();
        let saved: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        for p in saved["state"]["participants"].as_array().unwrap() {
            assert!(p.get("changes").is_none() && p.get("added_from").is_none());
        }
        let reopened = Run::open(&path, campaign).unwrap();
        assert!(
            reopened
                .state
                .participants
                .iter()
                .all(|p| p.changes.is_none())
        );
    }
    #[test]
    fn the_run_card_and_panel_draw_with_changes_and_groups_in_setup_and_in_combat() {
        let ctx = egui::Context::default();
        let mut run = run();
        run.groups = Some(Arc::new(vec![mixed_group(), guard_group("Wave", 2)]));
        run.apply(Command::AddGroup(
            group_of(
                "Rivals",
                "rivals",
                vec![(
                    creature(),
                    1,
                    changed(
                        "{name: Captain, attack_bonus: {add: 1}, speed: {set: 40}, note: Sealed}",
                    ),
                )],
            ),
            Some(30),
        ))
        .unwrap();
        run.selected = 3;
        for panel in [Panel::Groups, Panel::Record, Panel::Reference] {
            run.panel = panel;
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    run.show(ui);
                });
            });
        }
        for i in 0..3 {
            run.apply(Command::Initiative(i, Some(5))).unwrap();
        }
        run.apply(Command::Begin).unwrap();
        for panel in [Panel::Groups, Panel::Reference] {
            run.panel = panel;
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    run.show(ui);
                });
            });
        }
        let name = view::row_words_for_test(&run.state.participants[3]);
        assert!(
            name.contains("Changed") && name.contains("Attack +1"),
            "{name}"
        );
    }
    #[test]
    fn iter_1_b_saved_times_use_age_for_recent_calendar_days() {
        let now = 10 * 86_400 + 3_600;
        assert_eq!(saved_time(now, now), "just now");
        assert_eq!(saved_time(now - 120, now), "2 min ago");
        assert_eq!(saved_time(now - 7_200, now), "2 h ago");
        assert_eq!(saved_time(now - 86_400, now), "1 day ago");
        assert_eq!(saved_time(now - 90_001, now), calendar_date(now - 90_001));
        assert_eq!(
            saved_time(now - 3 * 86_400, now),
            calendar_date(now - 3 * 86_400)
        );
    }

    #[test]
    fn iter_1_b_invalid_arrival_totals_keep_text_and_run_unchanged() {
        for text in ["-", "--2", "9223372036854775808"] {
            let mut run = run();
            begin(&mut run);
            let wave = guard_group("Wave", 1);
            let id = wave.id;
            run.groups = Some(Arc::new(vec![wave]));
            run.group_totals.insert(id, text.into());
            let count = run.state.participants.len();
            let history = run.history.len();
            run.add_group_from_panel(id);
            assert_eq!(run.state.participants.len(), count, "{text}");
            assert_eq!(run.history.len(), history);
            assert_eq!(run.group_totals.get(&id).unwrap(), text);
            assert_eq!(
                run.error.as_deref(),
                Some("Enter a whole number, or clear the field to roll.")
            );
        }
    }

    #[test]
    fn a_drop_uses_the_total_typed_beside_the_group_and_a_failure_keeps_it() {
        let mut run = run();
        begin(&mut run);
        let wave = guard_group("Wave", 1);
        let id = wave.id;
        run.groups = Some(Arc::new(vec![wave]));
        run.group_totals.insert(id, "13".into());
        run.add_group_from_panel(id);
        assert_eq!(run.state.participants[3].initiative, Some(13));
        assert!(!run.group_totals.contains_key(&id), "used once");
        run.add_group_from_panel(Uuid::new_v4());
        assert_eq!(
            run.state.participants.len(),
            4,
            "a group no longer there adds nothing"
        );
        run.state.random = Random::controlled([]);
        run.add_group_from_panel(id);
        assert!(run.error.is_some());
    }
}
