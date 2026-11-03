//! Manual DM controls. These record rulings; they do not execute creature rules.
mod ad_hoc;
mod view;
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
pub(crate) use view::change_rows;

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
    /// Every reason this group cannot be added now.
    pub problem: Option<String>,
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
            "{} · {} · XP {}",
            crate::design::count(self.creature_count() as usize, "creature", "creatures"),
            crate::design::reader::human_label(&self.side),
            self.xp
        );
        if !self.xp_unknown.is_empty() || self.unavailable > 0 {
            summary.push_str(" or more");
        }
        if self
            .members
            .iter()
            .any(|member| !member.overrides.is_empty())
        {
            summary.push_str(" · Changed");
        }
        summary
    }
    /// Every creature the note names, whether or not it can be found now.
    pub(crate) fn creature_count(&self) -> u64 {
        self.members
            .iter()
            .map(|member| member.count)
            .fold(self.unavailable, u64::saturating_add)
    }
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
    random: Random,
}
/// Each carried player's place in the roster, and HP and temporary HP from their sheet.
type SheetValues = Vec<(usize, Option<i64>, i64)>;
/// HP that came from the last fight, not from the sheets.
struct CarriedHp {
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
    initiative: Initiative,
    storage: Option<Storage>,
    /// Each character's cached portrait, by D&D Beyond ID.
    portraits: BTreeMap<String, Option<PathBuf>>,
    portraits_read: Option<Instant>,
    dirty: bool,
    state: State,
    history: Vec<State>,
    /// What each history step did, for a named Undo.
    history_labels: Vec<String>,
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
    /// What sync changed. It stays until the DM dismisses it, whatever they do next.
    notice: Option<String>,
    /// A value reached the run with no step of its own, so Undo stops there.
    undo_stopped: bool,
    setup_inputs: BTreeMap<usize, String>,
    /// The participant whose initiative the last step set, to merge keystrokes.
    last_initiative: Option<usize>,
    /// Lookup is beside the run, so the run's own panel gives up its space.
    pub hide_panel: bool,
    /// The recording needs a look, shown where the run's notices go, so nothing moves.
    pub recording_notice: Option<&'static str>,
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
/// What the group panel says of an addition before it points to the Log.
const GROUP_LINE_LIMIT: usize = 200;
/// What a saved run may hold. A larger one does not open again.
const MAX_PARTICIPANTS: usize = 2000;
/// Where a run's own text ends. Saved runs hold both parts in one string.
pub(crate) const SETUP_HEADING: &str = "## Setup (DM reference)";
const DISK_CONFLICT: &str = "This run changed on disk, perhaps from another copy of Splat or a sync tool. Your run is safe in memory, and saving has stopped until you choose.";
const MAX_FROM_SHEET: &str =
    "Maximum HP comes from the D&D Beyond sheet while sync is on. Stop sync to change it here.";
pub(crate) const NOBODY_PRESENT: &str =
    "No participants are present. Mark one present or end the encounter.";
const CLOSE_BLOCKED: &str =
    "Wait for pending sync changes, resolve conflicts, or turn off combat sync before closing.";
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
    Roll(String),
    /// Add every member of an encounter group, on this total, or by rolling.
    AddGroup(Arc<RunGroup>, Option<i64>),
    /// One line of creatures with no group behind them.
    AddCreature(Arc<RunGroup>, Option<i64>),
    /// One amount on several targets, as one step of undo.
    Several(Vec<Command>),
    /// A roll on a marked table in a note, on this run's dice, with its row in the log.
    TableRoll(crate::roll_table::TableRoll),
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
            initiative: Initiative::default(),
            storage: None,
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
                random: Random::seeded(seed),
            },
            history: Vec::new(),
            history_labels: Vec::new(),
            selected: 0,
            also: BTreeSet::new(),
            halved: BTreeSet::new(),
            dm_steps: 0,
            carried: None,
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
            panel: Panel::default(),
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
            recording_notice: None,
            undo_stopped: false,
            setup_inputs: BTreeMap::new(),
            last_initiative: None,
            hide_panel: false,
            lookup_open: false,
            reopened: false,
            content: None,
            lookup_request: None,
            close_lookup: false,
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
        // A fight that cannot be read carries nothing, and the log must not say it did.
        if !sheets.is_empty() {
            self.state
                .log
                .push(format!("Party HP carried over from {}.", carry.from));
        }
        self.carried = Some(CarriedHp {
            from: carry.from.clone(),
            sheets,
            missing: carry.missing.clone(),
        });
        self.dirty = true;
    }
    /// The carried values that still differ from the sheets, as one step back to the sheets.
    fn back_to_sheets(&self) -> Option<(String, Command)> {
        let carried = self.carried.as_ref()?;
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
    /// A place for someone whose initiative was set during combat. The current turn stays
    /// current, and no one else moves relative to each other.
    fn take_place_in_order(state: &mut State, index: usize) {
        let current = state.order[state.cursor];
        state.order.retain(|i| *i != index);
        let total = state.participants[index].initiative;
        let place = state
            .order
            .iter()
            .position(|i| state.participants[*i].initiative < total)
            .unwrap_or(state.order.len());
        state.order.insert(place, index);
        if let Some(cursor) = state.order.iter().position(|i| *i == current) {
            state.cursor = cursor;
        }
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
            added.iter_mut().for_each(|p| p.initiative = Some(total));
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
                added.iter_mut().for_each(|p| p.initiative = Some(total));
                words.push(format!("Shared roll {die} {bonus:+} = {total}"));
            }
            Grouping::Individual => {
                for p in added.iter_mut() {
                    let (die, bonus, total) = roll(&mut state.random, p)?;
                    p.initiative = Some(total);
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
                                words.push(format!("Joins {} at {total}", p.side));
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
                }
            }
        }
        Ok(words.join("; "))
    }
    fn join_group(
        state: &mut State,
        group: &RunGroup,
        total: Option<i64>,
        grouping: Grouping,
    ) -> Result<(String, String), String> {
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
        let initiative = Self::group_initiative(state, &mut added, total, grouping)?;
        let names: Vec<_> = added.iter().map(|p| p.name.clone()).collect();
        let first = state.participants.len();
        state.participants.extend(added);
        for index in first..state.participants.len() {
            state.order.push(index);
            if state.phase == Phase::Running {
                Self::take_place_in_order(state, index);
            }
        }
        if state.phase == Phase::Setup {
            Self::sort_initiative(state);
        }
        Ok((names.join(", "), initiative))
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
    fn apply(&mut self, command: Command) -> Result<(), String> {
        if matches!(self.state.phase, Phase::Ended { .. }) {
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
        let follow_turn = matches!(command, Command::Begin | Command::Next);
        let mut next = self.state.clone();
        let label = match command {
            Command::RollNpcs(grouping) => {
                if next.phase != Phase::Setup {
                    return Err("Initiative is fixed during combat.".into());
                }
                let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
                let mut group_order = Vec::new();
                for (i, p) in next.participants.iter().enumerate().filter(|(_, p)| p.npc) {
                    let key = match grouping {
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
                        Grouping::Explicit => {
                            p.initiative_group.clone().unwrap_or_else(|| p.key.clone())
                        }
                    };
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
                    for i in members {
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
                loop {
                    next.cursor += 1;
                    if next.cursor == next.order.len() {
                        next.cursor = 0;
                        next.round = next.round.checked_add(1).ok_or("Round limit reached.")?;
                    }
                    if next.participants[next.order[next.cursor]].present {
                        break;
                    }
                }
                format!(
                    "Round {}: {}'s turn.",
                    next.round, next.participants[next.order[next.cursor]].name
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
            Command::Roll(expression) => {
                let roll = Dice::parse(&expression)?.roll(&mut next.random, false)?;
                format!("DM roll {expression}: {:?} = {}.", roll.dice, roll.total)
            }
            Command::AddGroup(group, total) => {
                let (added, initiative) =
                    Self::join_group(&mut next, &group, total, self.initiative.grouping)?;
                format!("{GROUP_ADDED}{} added: {added}. {initiative}.", group.name)
            }
            Command::AddCreature(group, total) => {
                let first = next.participants.len();
                let (added, initiative) =
                    Self::join_group(&mut next, &group, total, self.initiative.grouping)?;
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
                    .map(|command| Self::participant_command(&mut next, command))
                    .collect::<Result<Vec<_>, _>>()?;
                let last = lines.pop().ok_or("Choose a target first.")?;
                next.log.append(&mut lines);
                last
            }
            Command::TableRoll(roll) => roll.resolve(&mut next.random)?,
            other => Self::participant_command(&mut next, other)?,
        };
        next.log.push(label);
        if self.history.len() == 100 {
            self.history.remove(0);
            self.history_labels.remove(0);
        }
        self.dm_steps += 1;
        self.history.push(std::mem::replace(&mut self.state, next));
        self.history_labels.push(undo_label);
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
    fn participant_command(state: &mut State, command: Command) -> Result<String, String> {
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
                let change = format!("Initiative set to {}.", display(total));
                if state.phase == Phase::Running {
                    let label = format!("{}: {change}", p.name);
                    Self::take_place_in_order(state, index);
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
                format!("Death saves: {successes} successes, {failures} failures.")
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
            self.dm_steps += 1;
            self.history_labels.pop();
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
            self.last_roll = None;
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
            // A long effect stays readable on the panel, and the label stays whole.
            Command::Condition(i, condition) if condition.trim().chars().count() > 24 => {
                format!("effect on {}", name(i))
            }
            Command::Condition(i, condition) => format!("{} on {}", condition.trim(), name(i)),
            Command::RemoveCondition(i, at) => format!(
                "{} off {}",
                self.state
                    .participants
                    .get(*i)
                    .and_then(|p| p.conditions.get(*at))
                    .filter(|condition| condition.trim().chars().count() <= 24)
                    .map_or("an effect", String::as_str),
                name(i)
            ),
            Command::Concentration(i, _) => format!("concentration on {}", name(i)),
            Command::DeathSaves(i, ..) => format!("death saves on {}", name(i)),
            Command::Note(i, _) => format!("note on {}", name(i)),
            Command::Roll(expression) => format!("{expression} roll"),
            Command::AddGroup(group, _) | Command::AddCreature(group, _) => {
                format!("add {}", group.name)
            }
            Command::Several(commands) => {
                let word = match commands.first() {
                    Some(Command::Health(..)) => return "HP from the sheets".to_owned(),
                    Some(Command::RemoveCondition(i, _)) => {
                        return format!("Exhaustion on {}", name(i));
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
        }
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
    /// What the timeline records about this run.
    pub fn timeline_fact(&self) -> crate::timeline::RunFact {
        crate::timeline::RunFact {
            id: self.id,
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
        for (index, sync) in &mut self.syncs {
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
                self.reconcile_sync_history(&before_sync, &sheet_indices, &from_panel);
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
    fn reconcile_sync_history(&mut self, before: &State, indices: &[usize], choices: &[String]) {
        let mut kept = Vec::new();
        let mut labels = Vec::new();
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
        }
        self.history = kept;
        self.history_labels = labels;
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
            self.history_labels
                .push(format!("{} (D&D Beyond)", choices.join("; ")));
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
        self.notice = Some(if names.is_empty() {
            what
        } else {
            format!("{what} {}.", names.join(". "))
        });
        self.undo_stopped = true;
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
#[derive(Serialize, Deserialize)]
struct SavedRun {
    version: u32,
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
    campaign: Uuid,
    baseline: Option<Vec<u8>>,
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
            path: directory.join(format!("{}.json", self.id)),
            campaign,
            baseline: None,
        });
        self.save()
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
            version: 1,
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
        let path = storage
            .path
            .parent()
            .ok_or("Invalid run location")?
            .join(format!("{id}.json"));
        let old_path = std::mem::replace(&mut storage.path, path);
        let old_baseline = storage.baseline.take();
        let old_id = std::mem::replace(&mut self.id, id);
        let new_name = format!("{} (copy)", self.name);
        let old_name = std::mem::replace(&mut self.name, new_name);
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
            initiative: saved.initiative,
            selected: match saved.state.phase {
                Phase::Running => saved.state.order[saved.state.cursor],
                _ => 0,
            },
            state: saved.state,
            setup_details: saved.setup_details,
            storage: Some(Storage {
                path: path.into(),
                campaign,
                baseline: Some(bytes),
            }),
            portraits: BTreeMap::new(),
            portraits_read: None,
            dirty: false,
            history: Vec::new(),
            history_labels: Vec::new(),
            also: BTreeSet::new(),
            halved: BTreeSet::new(),
            dm_steps: 0,
            carried: None,
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
            panel: Panel::default(),
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
            recording_notice: None,
            undo_stopped: false,
            setup_inputs: BTreeMap::new(),
            last_initiative: None,
            hide_panel: false,
            lookup_open: false,
            reopened: true,
            content: None,
            lookup_request: None,
            close_lookup: false,
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
            return Ok(Vec::new());
        }
        let mut runs = Vec::new();
        for file in fs::read_dir(directory).map_err(|e| e.to_string())? {
            let path = file.map_err(|e| e.to_string())?.path();
            if path.extension().is_some_and(|e| e == "json") {
                let modified = crate::hermetic::file_time(&path);
                match Self::open(&path, campaign) {
                    Ok(run) => runs.push(run.info(path, modified)),
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
        // Newest first. The names decide a tie, so that a list always comes out the same.
        runs.sort_by(|a, b| {
            b.saved_at
                .cmp(&a.saved_at)
                .then_with(|| a.encounter.cmp(&b.encounter))
                .then_with(|| a.label.cmp(&b.label))
                .then_with(|| a.path.cmp(&b.path))
        });
        Ok(runs)
    }
    /// This run as a row on the Encounters page, as it is now and not as its file has it.
    pub fn live_info(&self) -> Option<SavedRunInfo> {
        Some(self.info(self.path()?.to_path_buf(), None))
    }
    fn info(&self, path: PathBuf, saved_at: Option<u64>) -> SavedRunInfo {
        let round = self.state.round;
        let saved = saved_at.map_or_else(String::new, |at| {
            format!(
                " · Saved {}",
                crate::party::fetch_age(crate::hermetic::now().saturating_sub(at))
            )
        });
        let (standing, label) = match &self.state.phase {
            Phase::Ended { outcome } => {
                let outcome = Some(outcome.trim())
                    .filter(|outcome| !outcome.is_empty())
                    .unwrap_or("Ended");
                (Standing::Ended, format!("{outcome} · Round {round}{saved}"))
            }
            phase => {
                let status = if *phase == Phase::Setup {
                    "Preparing"
                } else {
                    "In progress"
                };
                // The turn it stopped on, so a DM can tell two saves apart at a glance.
                let turn = self
                    .current()
                    .map(|i| format!(" · {}’s turn", self.state.participants[i].name))
                    .unwrap_or_default();
                (
                    Standing::Unfinished,
                    format!("Round {round} · {status}{turn}{saved}"),
                )
            }
        };
        SavedRunInfo {
            path,
            id: Some(self.id),
            source: self.source,
            encounter: self.name.clone(),
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
        .filter(|p| p.definition.fields.contains_key("splat_external"))
        .filter_map(|p| Some((p.definition.id, (p.hp?, p.temporary_hp))))
        .collect())
}

/// A finished or interrupted fight, to read in place.
pub fn fight_summary(path: &Path, campaign: Uuid) -> Result<FightSummary, String> {
    let state = read_saved(path, campaign)?.state;
    Ok(FightSummary {
        ended: matches!(state.phase, Phase::Ended { .. }),
        rows: state
            .order
            .iter()
            .filter_map(|index| state.participants.get(*index))
            .map(|p| FightRow {
                name: p.name.clone(),
                player: p.definition.fields.contains_key("splat_external"),
                hp: p.hp,
                max_hp: p.max_hp,
                temporary_hp: p.temporary_hp,
                conditions: p.conditions.clone(),
                present: p.present,
            })
            .collect(),
        log: state.log.iter().rev().cloned().collect(),
    })
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
    let (participants, initiatives): (Vec<_>, Vec<_>) = participants.into_iter().unzip();
    let mut run = Run::new(
        name.into(),
        participants,
        format!("# {name}\n\nPhases and completion are resolved manually."),
    )?;
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
    use crate::combat_sync::SyncState;
    fn creature() -> Entry {
        Entry::parse("monster.md", &format!("---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: Guard\nsplat_creature: {{type: humanoid}}\nsplat_stats:\n  max_hp: {{value: 20}}\n  armor_class: {{value: 16}}\n  initiative: {{value: 2}}\nsplat_initial:\n  hp: 17\n  temporary_hp: 3\n  conditions: [Poisoned]\n  death_saves: {{successes: 1, failures: 0}}\n---\n\n## Actions\nSpear attack.\n", Uuid::new_v4())).unwrap()
    }
    fn run() -> Run {
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
    fn begin(run: &mut Run) {
        for (index, total) in [20, 15, 10].into_iter().enumerate() {
            run.apply(Command::Initiative(index, Some(total))).unwrap();
        }
        run.apply(Command::Begin).unwrap();
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
        run.save_as_new_run().unwrap();
        assert!(!run.has_disk_conflict());
        assert_eq!(fs::read(&path).unwrap(), external);
        let new_path = run.path().unwrap().to_path_buf();
        assert_ne!(new_path, path);
        assert_eq!(new_path.parent(), Some(dir.path()));
        let copy = Run::open(&new_path, campaign).unwrap();
        assert_eq!(copy.state.participants[0].hp, hp);
        assert_ne!(copy.id, Run::open(&path, campaign).unwrap().id);
        assert_eq!(copy.name, "Ambush (copy)");
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
    fn linked(run: &mut Run, index: usize, character: &str) {
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
        assert!(saved[0].label.contains("In progress"));

        run.apply(Command::End("Party won".into())).unwrap();
        run.save().unwrap();
        let saved = Run::list(dir.path(), campaign).unwrap();
        assert_eq!(saved[0].standing, Standing::Ended);
        assert!(saved[0].label.starts_with("Party won · Round "));
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
    fn the_undo_notice_stays_until_the_dm_dismisses_it() {
        let (mut run, _, max) = matched_with_sheet(5);
        read_sheet(&mut run, max, 8);
        assert!(run.notice.as_ref().unwrap().contains("Undo for "));
        assert!(run.undo_stopped, "plain Undo gives this reason on hover");
        run.next_turn();
        run.apply(Command::Damage(0, 1)).unwrap();
        assert!(run.notice.is_some(), "still there after the next steps");
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
    fn a_player_starts_from_the_last_fight_and_the_sheet_is_one_step_back() {
        let id = Uuid::new_v4();
        let entry = Entry::parse(
            "Hero.md",
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: creature\nsplat_name: Hero\nsplat_creature: {{size: medium}}\nsplat_external: {{provider: test}}\nsplat_stats:\n  max_hp: {{value: 41}}\nsplat_initial:\n  hp: 11\n---\n"
            ),
        )
        .unwrap();
        let mut hero =
            Participant::new("hero".into(), "party".into(), entry, &Mapping::new(), true).unwrap();
        let carry = Carry {
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
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: creature\nsplat_name: Wolf\nsplat_creature: {{size: medium}}\nsplat_stats:\n  max_hp: {{value: 11}}\n---\n"
            ),
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
            problem: None,
        })
    }
    fn changed(text: &str) -> Overrides {
        Overrides::parse(&serde_yaml_ng::from_str(text).unwrap()).unwrap()
    }
    fn beast() -> Entry {
        Entry::parse("beast.md", &format!("---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: Beast\nsplat_creature: {{type: beast}}\nsplat_stats:\n  max_hp: {{value: 40}}\n  armor_class: {{value: 12}}\n  initiative: {{value: -1}}\n  speed: {{value: 40}}\n---\n", Uuid::new_v4())).unwrap()
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
            log.contains("Wave") && log.contains("Guard 4") && log.contains("10 +2 = 12"),
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
                .contains("Joins enemies at 20")
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
        let unknown = Entry::parse("u.md", &format!("---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: Unknown\nsplat_creature: {{}}\n---\n", Uuid::new_v4())).unwrap();
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
