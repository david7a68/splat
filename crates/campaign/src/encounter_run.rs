//! Saved encounter runs and the rules that change their combat state.
pub mod effect;
pub mod undo;

use crate::content::{
    Entry,
    inspiration::Inspiration,
    overrides::{Base, Overrides, condition_names, same_condition},
};
use effect::Condition;
use serde::{Deserialize, Serialize};
use serde_yaml_ng::{Mapping, Value};
use splat_content::dice::Random;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
    sync::Arc,
};
use uuid::Uuid;

pub const MAX_PARTICIPANTS: usize = 2000;

pub const NOBODY_PRESENT: &str =
    "No participants are present. Mark one present or end the encounter.";

/// The words after a name in the log line of a new turn.
const TURN: &str = "’s turn.";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Grouping {
    #[default]
    Identical,
    Individual,
    BySide,
    Explicit,
}
impl Grouping {
    pub const ALL: [Self; 4] = [
        Self::Identical,
        Self::Individual,
        Self::BySide,
        Self::Explicit,
    ];
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Initiative {
    pub grouping: Grouping,
    pub roster_ties: bool,
}
impl Default for Initiative {
    fn default() -> Self {
        Self {
            grouping: Grouping::Identical,
            roster_ties: false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Participant {
    pub key: String,
    pub npc: bool,
    pub initiative_group: Option<String>,
    pub initiative_mode: i8,
    pub name: String,
    pub side: String,
    /// Only a row of a saved Undo step may leave this out: `undo::same_definition`.
    #[serde(
        default = "undo::same_definition",
        skip_serializing_if = "undo::is_same_definition"
    )]
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
    pub conditions: Vec<Condition>,
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
    pub started: Option<SheetShadow>,
    /// What the encounter or group changed, with the values they replaced. It is frozen here,
    /// so a later edit of the encounter file does not change this run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<Changes>,
    /// The encounter group this participant arrived in, during the run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_from: Option<AddedFrom>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Changes {
    pub base: Base,
    pub overrides: Overrides,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddedFrom {
    pub group: String,
    /// One addition. The same group added twice is two of these, each with its own roll.
    pub instance: String,
    #[serde(default)]
    pub round: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Uuid>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SheetShadow {
    pub hp: Option<i64>,
    pub temp: i64,
    pub successes: u8,
    pub failures: u8,
    pub conditions: Vec<Condition>,
}
impl SheetShadow {
    pub fn of(p: &Participant) -> Self {
        Self {
            hp: p.hp,
            temp: p.temporary_hp,
            successes: p.successes,
            failures: p.failures,
            conditions: p.conditions.clone(),
        }
    }
}

impl Participant {
    pub fn named_stat_block(&self) -> bool {
        self.npc_identity()
            || self.changes.as_ref().is_some_and(|changes| {
                changes
                    .overrides
                    .name
                    .as_ref()
                    .is_some_and(|name| name != &self.definition.name)
            })
    }
    pub fn npc_identity(&self) -> bool {
        self.definition.fields.contains_key("splat_run_npc_name")
    }
    pub fn stat_changes(&self) -> bool {
        self.changes.as_ref().is_some_and(|changes| {
            let mut overrides = changes.overrides.clone();
            if self.npc_identity() {
                overrides.name = None;
            }
            overrides.changes_creature()
        })
    }
    /// Keep existing counts visible even when the participant is not on the party side.
    pub fn shows_death_saves(&self) -> bool {
        // A linked player sheet makes death saves on any side name the DM chose.
        self.hp == Some(0)
            && (self.side == "party"
                || self.definition.fields.contains_key("splat_external")
                || self.successes > 0
                || self.failures > 0)
    }

    #[cfg(any(test, feature = "test-support"))]
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
            .map_err(|error| format!("{}: {error}", entry.name))?;
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
                    v.as_i64().filter(|n| *n >= 0).ok_or_else(|| {
                        let words = if field == "hp" { "HP" } else { "Temporary HP" };
                        format!(
                            "{}: its starting {words} must be a whole number, 0 or more.",
                            entry.name
                        )
                    })
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
        // Free words from an encounter stay labels: nothing typed in prep starts to count.
        let conditions: Vec<Condition> = conditions.into_iter().map(Condition::Label).collect();
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
    pub fn health_known(&self) -> bool {
        matches!((self.hp, self.max_hp), (Some(hp), Some(max)) if max > 0 && hp >= 0 && hp <= max)
    }

    pub fn set_health(
        &mut self,
        hp: Option<i64>,
        max: Option<i64>,
        running: bool,
    ) -> Result<(), &'static str> {
        if hp.is_some_and(|n| n < 0)
            || max.is_some_and(|n| n <= 0)
            || matches!((hp, max), (Some(h), Some(m)) if h > m)
            || (running && (hp.is_none() || max.is_none()))
        {
            return Err("HP must be between zero and a positive maximum.");
        }
        self.hp = hp;
        self.max_hp = max;
        Ok(())
    }

    pub fn set_temporary_hp(&mut self, amount: i64) -> Result<(), &'static str> {
        if amount < 0 {
            return Err("Temporary HP cannot be negative.");
        }
        self.temporary_hp = amount;
        Ok(())
    }

    /// Apply damage and return the amount absorbed by temporary HP and the remaining HP.
    pub fn damage(&mut self, amount: i64) -> Result<(i64, i64), &'static str> {
        if amount < 0 {
            return Err("Damage cannot be negative.");
        }
        let hp = self.hp.ok_or("Set current HP first.")?;
        let absorbed = amount.min(self.temporary_hp);
        let remaining = hp.saturating_sub(amount - absorbed).max(0);
        self.temporary_hp -= absorbed;
        self.hp = Some(remaining);
        Ok((absorbed, remaining))
    }

    pub fn heal(&mut self, amount: i64) -> Result<i64, &'static str> {
        if amount < 0 {
            return Err("Healing cannot be negative.");
        }
        let hp = self.hp.ok_or("Set current HP first.")?;
        let max = self.max_hp.ok_or("Set maximum HP first.")?;
        let remaining = hp.saturating_add(amount).min(max);
        self.hp = Some(remaining);
        Ok(remaining)
    }

    /// Ready to begin. Someone who is not here has no roll to make.
    pub fn ready(&self) -> bool {
        self.health_known() && (self.initiative.is_some() || !self.present)
    }
}

/// What one roll found.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Found {
    pub value: i64,
    pub dice: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Phase {
    Setup,
    Running,
    Ended { outcome: String },
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct State<C = ()> {
    pub participants: Vec<Participant>,
    pub order: Vec<usize>,
    pub cursor: usize,
    pub round: u64,
    pub phase: Phase,
    pub log: Vec<String>,
    #[serde(default)]
    pub last_change: Option<String>,
    pub random: Random,
    /// What each loot line rolled, by its place: `loot::place`. Saves from before loot have none.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub loot: BTreeMap<String, Vec<Found>>,
    /// What a save's damage step gave each target row, such as `Failed · 3 damage`. It
    /// belongs to the step, so Undo takes it back with the HP. Not saved.
    #[serde(skip)]
    pub roll_results: BTreeMap<usize, String>,
    /// The damage the step dealt each row, before temporary HP or 0 HP took any of it: a
    /// concentrating row's save DC comes from it. It belongs to the step. Not saved.
    #[serde(skip)]
    pub damage: BTreeMap<usize, i64>,
    /// The save as it stood before the next step applied its damage, so Undo of that step
    /// arms the save again with its results. Only the state before a save's damage step has
    /// one. Not saved.
    #[serde(skip)]
    pub cast: Option<C>,
}

impl<C> State<C> {
    pub fn begin(&mut self, roster_ties: bool) -> Result<String, String> {
        if self.phase != Phase::Setup {
            return Err("Combat has already started.".into());
        }
        if !self.participants.iter().all(Participant::ready) {
            return Err("Set valid current and maximum HP for each participant, and initiative for every participant.".into());
        }
        if !self.participants.iter().any(|p| p.present) {
            return Err("Mark at least one participant present.".into());
        }
        if roster_ties {
            self.order = (0..self.participants.len()).collect();
        }
        State::sort_initiative(self);
        self.cursor = self
            .order
            .iter()
            .position(|i| self.participants[*i].present)
            .ok_or("Mark at least one participant present.")?;
        self.round = 1;
        self.phase = Phase::Running;
        Ok(format!(
            "Round 1: {}{TURN}",
            self.participants[self.order[self.cursor]].name
        ))
    }

    pub fn next_turn(&mut self, roster_ties: bool) -> Result<String, String> {
        if self.phase != Phase::Running {
            return Err("Begin combat first.".into());
        }
        if !self.participants.iter().any(|p| p.present) {
            return Err(NOBODY_PRESENT.into());
        }
        let ending = self.order[self.cursor];
        Self::end_turn(self, roster_ties)?;
        self.log
            .extend(effect::count_down(&mut self.participants[ending]));
        Ok(format!(
            "Round {}: {}{TURN}",
            self.round, self.participants[self.order[self.cursor]].name
        ))
    }

    pub fn end(&mut self, outcome: String) -> Result<String, String> {
        if self.phase != Phase::Running {
            return Err("Begin combat before ending it.".into());
        }
        self.phase = Phase::Ended {
            outcome: outcome.clone(),
        };
        Ok(format!(
            "Ended manually in round {}. Outcome: {outcome}.",
            self.round
        ))
    }

    pub fn continue_fight(&mut self) -> Result<String, String> {
        if !matches!(self.phase, Phase::Ended { .. }) {
            return Err("This fight has not ended.".into());
        }
        self.phase = Phase::Running;
        Ok(format!(
            "Fight continued. Round {}: {}{TURN}",
            self.round, self.participants[self.order[self.cursor]].name
        ))
    }

    pub fn sort_initiative(state: &mut Self) {
        // Preserve the DM's chosen order for tied totals.
        state
            .order
            .sort_by_key(|i| std::cmp::Reverse(state.participants[*i].initiative));
    }
    /// A place for someone whose initiative was set during combat. The current turn stays
    /// current, and no one else moves relative to each other.
    /// Whether `a` acts before `b`: by total, then by roster when the campaign breaks ties
    /// that way. Otherwise a tie keeps the order the DM gave it.
    pub fn goes_before(state: &Self, a: usize, b: usize, roster_ties: bool) -> bool {
        let (x, y) = (
            state.participants[a].initiative,
            state.participants[b].initiative,
        );
        x > y || (roster_ties && x == y && a < b)
    }
    /// Whether the row at `at` in the order is where its total puts it, beside the rows
    /// other than `skip`.
    pub fn in_place(state: &Self, at: usize, roster_ties: bool, skip: Option<usize>) -> bool {
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
    pub fn moves_after_turn(state: &Self) -> Option<usize> {
        (state.phase == Phase::Running && !Self::in_place(state, state.cursor, false, None))
            .then(|| state.order[state.cursor])
    }
    /// Put a participant where its total goes, and keep the current turn on the same row.
    /// `waiting` is the current row that waits for its turn to end, found before this
    /// change: it is not where its total goes, so the place is found beside the others.
    pub fn take_place_in_order(
        state: &mut Self,
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
    /// Give the turn to the next present participant. A current row whose total changed in
    /// its turn goes to its new place first: if that is still to come in this round, it acts
    /// again there; if it has passed, it acts there next round.
    pub fn end_turn(state: &mut Self, roster_ties: bool) -> Result<(), String> {
        if !state.participants.iter().any(|p| p.present) {
            return Err(NOBODY_PRESENT.into());
        }
        // The rows still to come in this round start after the current one.
        let mut from = state.cursor + 1;
        let original_order = (state.round == u64::MAX).then(|| state.order.clone());
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
                let Some(round) = state.round.checked_add(1) else {
                    state.order = original_order.expect("order kept at the round limit");
                    return Err("Round limit reached.".into());
                };
                state.round = round;
                state.order.iter().position(present).ok_or(NOBODY_PRESENT)?
            }
        };
        Ok(())
    }
}

/// A d20 for initiative, with the bonus and the roll mode. Returns the die and the total.
pub fn roll_initiative(random: &mut Random, bonus: i64, mode: i8) -> Result<(i64, i64), String> {
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

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Recovery {
    #[serde(default)]
    pub closed_order: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copied_from: Option<Uuid>,
    #[serde(default)]
    pub copy_number: u64,
}

impl Recovery {
    pub fn number(&self, name: &str) -> u64 {
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
#[serde(bound = "")]
pub struct SavedRun<C = ()> {
    #[serde(default)]
    pub rest_order: u64,
    pub version: u32,
    #[serde(default)]
    pub recovery: Recovery,
    pub campaign: Uuid,
    pub id: Uuid,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Uuid>,
    pub initiative: Initiative,
    /// Saves from before this have none, and offer no inspiration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inspiration: Option<Inspiration>,
    pub state: State<C>,
    pub setup_details: String,
    /// Saves from before this choice load with it off. The DM can start sync from the run.
    #[serde(default)]
    pub keep_party_synced: bool,
    #[serde(default)]
    pub stopped_players: Vec<String>,
    /// Participants whose spent inspiration their sheet does not show yet.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub spent_inspiration: BTreeSet<String>,
    /// The Undo steps, in the form `undo` writes and reads. Saves from before this have none.
    /// It stays as text here, so that a list of runs does not read steps it will not use, and
    /// so that steps this version cannot read do not stop the run from loading.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undo: Option<Box<serde_json::value::RawValue>>,
}
impl<C> SavedRun<C> {
    /// A run file's text as a run: of this campaign, or of any campaign when a folder is
    /// checked with no campaign open. Every reader of a run file comes through here.
    fn parse(bytes: &[u8], campaign: Option<Uuid>) -> Result<Self, String> {
        let saved: Self =
            serde_json::from_slice(bytes).map_err(|e| format!("Invalid saved run: {e}"))?;
        if campaign.is_none() && saved.version != 1 {
            return Err("Unsupported run version.".into());
        }
        if saved.version != 1 || campaign.is_some_and(|campaign| saved.campaign != campaign) {
            return Err("Unsupported run version or wrong campaign.".into());
        }
        // Only a row of an Undo step may leave its definition out.
        if saved
            .state
            .participants
            .iter()
            .any(|p| undo::is_same_definition(&p.definition))
        {
            return Err("Invalid saved run: a participant has no definition.".into());
        }
        Ok(saved)
    }

    /// Read a run that can be reopened, of the given campaign when one is open.
    pub fn reopen(bytes: &[u8], campaign: Option<Uuid>) -> Result<Self, String> {
        let saved = Self::parse(bytes, campaign)?;
        if saved.id.is_nil() {
            return Err("Unsupported run version or wrong campaign.".into());
        }
        if !valid_state(&saved.state) {
            return Err("Saved run state is invalid; its file has not been changed.".into());
        }
        Ok(saved)
    }
}
/// What `Run::open` requires of a saved state, and of each state an Undo step gives back.
fn valid_state<C>(state: &State<C>) -> bool {
    let count = state.participants.len();
    let order: BTreeSet<_> = state.order.iter().copied().collect();
    count > 0
        && count <= MAX_PARTICIPANTS
        && state.order.len() == count
        && order == (0..count).collect()
        && state.cursor < count
        && state.participants.iter().all(|p| {
            p.temporary_hp >= 0
                && p.successes <= 3
                && p.failures <= 3
                && (-1..=1).contains(&p.initiative_mode)
        })
        && (state.phase == Phase::Setup
            || (state.participants.iter().all(Participant::health_known) && state.round > 0))
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
    /// The participant's identity: a player's matches its party sheet.
    pub id: Uuid,
    pub name: String,
    pub player: bool,
    pub hp: Option<i64>,
    pub max_hp: Option<i64>,
    pub temporary_hp: i64,
    pub conditions: Vec<Condition>,
    pub present: bool,
    /// Concentration was marked when the run was saved.
    pub concentration: bool,
}

/// A player's character, rather than a creature.
pub fn is_player(p: &Participant) -> bool {
    p.definition.fields.contains_key("splat_external")
}

impl From<&Participant> for FightRow {
    fn from(p: &Participant) -> Self {
        Self {
            id: p.definition.id,
            name: p.name.clone(),
            player: is_player(p),
            hp: p.hp,
            max_hp: p.max_hp,
            temporary_hp: p.temporary_hp,
            conditions: p.conditions.clone(),
            present: p.present,
            concentration: p.concentration,
        }
    }
}

/// Check the same saved-run model as normal loading, without opening a campaign.
pub fn check_saved_text(text: &str) -> Result<(), String> {
    SavedRun::<()>::reopen(text.as_bytes(), None).map(drop)
}

/// A saved run's file, within the size limit, including a file that grows during the read.
pub fn read_saved_bytes(path: &Path) -> Result<Vec<u8>, String> {
    const LIMIT: u64 = 64 * 1024 * 1024;
    let too_large = "Saved run exceeds the size limit.";
    let file = fs::File::open(path).map_err(|error| error.to_string())?;
    if file.metadata().map_err(|error| error.to_string())?.len() > LIMIT {
        return Err(too_large.into());
    }
    let mut bytes = Vec::new();
    file.take(LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() as u64 > LIMIT {
        return Err(too_large.into());
    }
    Ok(bytes)
}

pub fn read_saved<C>(path: &Path, campaign: Uuid) -> Result<SavedRun<C>, String> {
    SavedRun::reopen(&read_saved_bytes(path)?, Some(campaign))
}

/// How the players stood when this run was last saved.
pub fn party_at_end(path: &Path, campaign: Uuid) -> Result<BTreeMap<Uuid, (i64, i64)>, String> {
    Ok(read_saved::<()>(path, campaign)?
        .state
        .participants
        .iter()
        .filter(|p| is_player(p))
        .filter_map(|p| Some((p.definition.id, (p.hp?, p.temporary_hp))))
        .collect())
}

/// A finished or interrupted fight, to read in place.
pub fn fight_summary(path: &Path, campaign: Uuid) -> Result<FightSummary, String> {
    let state = read_saved::<()>(path, campaign)?.state;
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

impl Participant {
    /// What this creature carries, one line each.
    pub fn loot_lines(&self) -> Vec<String> {
        self.changes
            .as_ref()
            .and_then(|changes| changes.overrides.loot.as_deref())
            .map(crate::content::loot::parts)
            .unwrap_or_default()
    }

    /// The creature is down and has loot to take.
    pub fn loot_ready(&self) -> bool {
        self.hp == Some(0)
            && self
                .changes
                .as_ref()
                .is_some_and(|changes| changes.overrides.loot.is_some())
    }
}
