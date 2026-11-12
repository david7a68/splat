//! The run's Undo steps, as the run file keeps them. Only this file writes or reads that form.
//!
//! A step holds the state before it. Whole states would repeat every creature and the whole
//! log for each step, so the file keeps each state as its differences from the state after the
//! step: the rows that differ and the log lines that differ. The rest of a state is small, and
//! is kept whole.
use super::{Participant, Phase, State, loot};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use splat::{
    content::{Entry, Fields},
    execution::dice::Random,
};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{Arc, LazyLock},
};

/// The most steps a run keeps, in memory and in its file.
pub(super) const LIMIT: usize = 100;
const VERSION: u32 = 1;
/// The most bytes the steps may take in the run file. A run file above 64 MiB does not open
/// again, so the steps must leave room for the run itself.
const SAVED_BYTES: usize = if cfg!(test) {
    1024 * 1024
} else {
    16 * 1024 * 1024
};
/// The most memory the steps read from a file may take, as `Step::weight` counts it. Tests
/// use smaller budgets for both, so that they can reach them with small files.
pub(super) const MEMORY_BYTES: usize = if cfg!(test) {
    32 * 1024 * 1024
} else {
    256 * 1024 * 1024
};
/// One slot of a collection: a value in a list or a map, with the spare room that a
/// collection can hold. A list of a million empty strings is small as text and large in memory.
const SLOT: usize = 2 * (std::mem::size_of::<serde_yaml_ng::Value>() + 16);
/// The most a step's text can grow when it is decoded: a few bytes of text name one slot.
const DECODED: usize = 64;

/// The state before each step, oldest first, with each step's name, whether a sheet made it,
/// and the words in its name that name the row it acted on (`Step::target`).
pub(super) type History = (Vec<State>, Vec<String>, Vec<bool>, Vec<Option<String>>);

/// The file's form. It is written from steps that are already text, and read as the newest
/// steps, still as text.
#[derive(Serialize, Deserialize)]
struct Saved<Steps> {
    version: u32,
    steps: Steps,
}

#[derive(Serialize, Deserialize)]
struct Step {
    label: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    from_sheet: bool,
    /// How many rows the state has.
    count: usize,
    /// The rows that differ from the same rows after the step, by place.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    rows: BTreeMap<usize, Participant>,
    log: Lines,
    order: Vec<usize>,
    cursor: usize,
    round: u64,
    phase: Phase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    last_change: Option<String>,
    random: Random,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    loot: BTreeMap<String, Vec<loot::Found>>,
}

/// A log as its difference from the log after the step: the same first `keep` lines, then
/// `own`, then the same last `tail` lines.
#[derive(Serialize, Deserialize)]
struct Lines {
    keep: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    own: Vec<String>,
    tail: usize,
}

/// In a saved row, this stands for the definition the same row has after the step. A run does
/// not change a creature's definition, so a saved row almost never needs its own copy.
static SAME_DEFINITION: LazyLock<Arc<Entry>> = LazyLock::new(|| {
    Arc::new(Entry {
        path: String::new(),
        id: uuid::Uuid::nil(),
        key: None,
        name: String::new(),
        kind: String::new(),
        fields: Fields::from(serde_yaml_ng::Mapping::new()),
        body: String::new(),
        body_line: 0,
        inventory: Vec::new(),
        inventory_containers: Vec::new(),
        flags: Vec::new(),
    })
});

pub(super) fn same_definition() -> Arc<Entry> {
    SAME_DEFINITION.clone()
}

/// True for a row that has no definition of its own. Only a saved Undo row may be one.
pub(super) fn is_same_definition(definition: &Arc<Entry>) -> bool {
    Arc::ptr_eq(definition, &SAME_DEFINITION)
}

/// Every field is named, so that a new field cannot be left out of the comparison. Two
/// definitions are the same only when they are one copy; an equal second copy is saved again.
fn same_row(a: &Participant, b: &Participant) -> bool {
    let Participant {
        key,
        npc,
        initiative_group,
        initiative_mode,
        name,
        side,
        definition,
        initial,
        max_hp,
        hp,
        temporary_hp,
        armor_class,
        initiative_bonus,
        initiative,
        shared_roll,
        present,
        conditions,
        concentration,
        successes,
        failures,
        notes,
        combat_values,
        started,
        changes,
        added_from,
    } = a;
    Arc::ptr_eq(definition, &b.definition)
        && hp == &b.hp
        && temporary_hp == &b.temporary_hp
        && max_hp == &b.max_hp
        && initiative == &b.initiative
        && present == &b.present
        && concentration == &b.concentration
        && successes == &b.successes
        && failures == &b.failures
        && npc == &b.npc
        && initiative_mode == &b.initiative_mode
        && armor_class == &b.armor_class
        && initiative_bonus == &b.initiative_bonus
        && conditions == &b.conditions
        && notes == &b.notes
        && key == &b.key
        && name == &b.name
        && side == &b.side
        && initiative_group == &b.initiative_group
        && shared_roll == &b.shared_roll
        && added_from == &b.added_from
        && started == &b.started
        && combat_values == &b.combat_values
        && changes == &b.changes
        && initial == &b.initial
}

impl Lines {
    fn of(before: &[String], after: &[String]) -> Self {
        let keep = before.iter().zip(after).take_while(|(a, b)| a == b).count();
        let tail = before[keep..]
            .iter()
            .rev()
            .zip(after[keep..].iter().rev())
            .take_while(|(a, b)| a == b)
            .count();
        Self {
            keep,
            own: before[keep..before.len() - tail].to_vec(),
            tail,
        }
    }

    fn before(self, after: &[String]) -> Option<Vec<String>> {
        let tail_from = after.len().checked_sub(self.tail)?;
        if self.keep > tail_from {
            return None;
        }
        let mut log = after[..self.keep].to_vec();
        log.extend(self.own);
        log.extend_from_slice(&after[tail_from..]);
        Some(log)
    }
}

impl Step {
    /// The row this step acted on, as its own data says: the one row it changed, by its
    /// place, named as the run names that row (what a row is stays the run's, as in `before`),
    /// when the step's name names it at its start or end (`7 damage to Guard 1`, `Guard 1
    /// absent`). The file keeps no more; a step on no row, or on several, has none.
    fn target(&self, after: &State) -> Option<String> {
        let [at] = self.rows.keys().collect::<Vec<_>>()[..] else {
            return None;
        };
        let name = &after.participants.get(*at)?.name;
        (self.label.ends_with(name.as_str()) || self.label.starts_with(name.as_str()))
            .then(|| name.clone())
    }
    fn of(before: &State, after: &State, label: &str, from_sheet: bool) -> Self {
        // Every field is named, so that a new field of a state is a decision here. The two
        // left out are never saved: the run file does not keep them for its own state either.
        let State {
            participants,
            order,
            cursor,
            round,
            phase,
            log,
            last_change,
            random,
            loot,
            roll_results: _,
            damage: _,
            cast: _,
        } = before;
        let rows = participants
            .iter()
            .enumerate()
            .filter_map(|(at, row)| {
                let kept = after.participants.get(at);
                if kept.is_some_and(|kept| same_row(row, kept)) {
                    return None;
                }
                // The definition is the run's, by row. A step never holds one.
                let mut row = row.clone();
                row.definition = same_definition();
                Some((at, row))
            })
            .collect();
        Self {
            label: label.to_owned(),
            from_sheet,
            count: participants.len(),
            rows,
            log: Lines::of(log, &after.log),
            order: order.clone(),
            cursor: *cursor,
            round: *round,
            phase: phase.clone(),
            last_change: last_change.clone(),
            random: random.clone(),
            loot: loot.clone(),
        }
    }

    /// Check this step against the state after it, and say what each row of the state before
    /// it weighs. Nothing is built yet. A step changes rows and never names a new one: each
    /// of its rows is a row the state after it has, in the same place, with the same key. A
    /// row the state before it lacks is one that arrived later. Anything else is not a step
    /// this version wrote.
    fn row_weights(&self, after: &State, after_weights: &[usize]) -> Option<Vec<usize>> {
        let arrived = after.participants.get(self.count..)?;
        if !arrived.iter().all(|row| row.added_from.is_some()) {
            return None;
        }
        let mut weights = after_weights.get(..self.count)?.to_vec();
        for (at, row) in &self.rows {
            let kept = after.participants.get(*at).filter(|_| *at < self.count)?;
            if row.key != kept.key {
                return None;
            }
            weights[*at] = row_weight(row, kept);
        }
        Some(weights)
    }

    /// What the state before this step weighs when it is kept: its rows, as `row_weights`
    /// gives them, and everything else it owns. This is the one count that the memory budget
    /// uses, and it is made before the state is built.
    fn weight(&self, after: &State, rows: &[usize]) -> usize {
        // The log is the log after the step, less the lines the step added, with its own.
        let lines = after
            .log
            .iter()
            .chain(&self.log.own)
            .map(|line| SLOT + line.len());
        rows.iter()
            .copied()
            .chain(lines)
            .chain([
                self.label.len(),
                SLOT * self.order.len(),
                owned(&self.phase),
                owned(&self.last_change),
                owned(&self.random),
                owned(&self.loot),
            ])
            .fold(0, usize::saturating_add)
    }

    /// The state before this step, given the state after it. `row_weights` has checked it.
    /// A step can change what a row has now. What a row is stays the run's: its name, side,
    /// AC, definition, starting values, changes and arrival come from the row after the step,
    /// whatever the file says.
    fn before(self, after: &State) -> Option<State> {
        let mut rows = self.rows;
        let participants = after
            .participants
            .get(..self.count)?
            .iter()
            .enumerate()
            .map(|(at, kept)| match rows.remove(&at) {
                Some(row) => Participant {
                    key: kept.key.clone(),
                    name: kept.name.clone(),
                    side: kept.side.clone(),
                    armor_class: kept.armor_class,
                    definition: kept.definition.clone(),
                    initial: kept.initial.clone(),
                    changes: kept.changes.clone(),
                    added_from: kept.added_from.clone(),
                    ..row
                },
                None => kept.clone(),
            })
            .collect();
        let state = State {
            participants,
            order: self.order,
            cursor: self.cursor,
            round: self.round,
            phase: self.phase,
            log: self.log.before(&after.log)?,
            last_change: self.last_change,
            random: self.random,
            loot: self.loot,
            roll_results: BTreeMap::new(),
            damage: BTreeMap::new(),
            cast: None,
        };
        super::valid_state(&state).then_some(state)
    }
}

/// The steps for the run file, oldest first. `now` is the run's current state, which the file
/// holds whole. When the steps are more than the file may hold, the oldest stay out of it.
pub(super) fn save(
    history: &[State],
    labels: &[String],
    from_sheet: &[bool],
    now: &State,
) -> Result<Box<RawValue>, String> {
    let mut steps = Vec::new();
    let mut bytes = 0;
    // The newest step is the one the DM needs first, so the budget is spent from that end.
    for at in (0..history.len()).rev() {
        let after = history.get(at + 1).unwrap_or(now);
        let step = Step::of(&history[at], after, &labels[at], from_sheet[at]);
        let step = serde_json::value::to_raw_value(&step).map_err(|e| e.to_string())?;
        bytes += step.get().len();
        if bytes > SAVED_BYTES {
            break;
        }
        steps.push(step);
    }
    steps.reverse();
    serde_json::value::to_raw_value(&Saved {
        version: VERSION,
        steps,
    })
    .map_err(|e| e.to_string())
}

/// The newest steps of a file's list, each still as text. A file can list more steps than this
/// version keeps. The older ones are passed over, and none of them is decoded.
struct Newest<'a>(VecDeque<&'a RawValue>);

impl<'de: 'a, 'a> Deserialize<'de> for Newest<'a> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Steps<'a>(std::marker::PhantomData<&'a ()>);
        impl<'de: 'a, 'a> serde::de::Visitor<'de> for Steps<'a> {
            type Value = Newest<'a>;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a list of Undo steps")
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut list: A,
            ) -> Result<Self::Value, A::Error> {
                let mut newest = VecDeque::with_capacity(LIMIT);
                while let Some(step) = list.next_element::<&'a RawValue>()? {
                    if newest.len() == LIMIT {
                        newest.pop_front();
                    }
                    newest.push_back(step);
                }
                Ok(Newest(newest))
            }
        }
        deserializer.deserialize_seq(Steps(std::marker::PhantomData))
    }
}

/// The steps from a run file. None when this version cannot read them: another version wrote
/// them, or they do not fit the run's current state. A file can hold more than this version
/// keeps, or states that would take too much memory: then the newest steps that fit are read.
/// One step is decoded at a time, and only while its text can fit. Its state is built only
/// after its weight is known to fit.
pub(super) fn read(saved: &RawValue, now: &State) -> Option<History> {
    let saved: Saved<Newest<'_>> = serde_json::from_str(saved.get()).ok()?;
    if saved.version != VERSION {
        return None;
    }
    let mut history = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut memory: usize = 0;
    let mut weights: Vec<usize> = now
        .participants
        .iter()
        .map(|row| row_weight(row, row))
        .collect();
    // The newest step is next to the current state, so the states are rebuilt from the end.
    for text in saved.steps.0.iter().rev() {
        let decoded = text.get().len().saturating_mul(DECODED);
        if memory.saturating_add(decoded) > MEMORY_BYTES {
            break;
        }
        let mut step: Step = serde_json::from_str(text.get()).ok()?;
        let after = history.0.last().unwrap_or(now);
        let rows = step.row_weights(after, &weights)?;
        memory = memory.saturating_add(step.weight(after, &rows));
        if memory > MEMORY_BYTES {
            break;
        }
        history.3.push(step.target(after));
        history.1.push(std::mem::take(&mut step.label));
        history.2.push(step.from_sheet);
        let state = step.before(after)?;
        history.0.push(state);
        weights = rows;
    }
    history.0.reverse();
    history.1.reverse();
    history.3.reverse();
    history.2.reverse();
    Some(history)
}

/// The memory a value owns outside itself: its text, and a slot for each item of each list
/// and map in it. It is counted from the value's own parts, not from its size as text.
fn owned<T: Serialize + ?Sized>(value: &T) -> usize {
    fn parts(value: &serde_json::Value) -> usize {
        match value {
            serde_json::Value::String(text) => text.len(),
            serde_json::Value::Array(items) => items
                .iter()
                .map(|item| SLOT.saturating_add(parts(item)))
                .fold(0, usize::saturating_add),
            serde_json::Value::Object(fields) => fields
                .iter()
                .map(|(key, value)| (2 * SLOT + key.len()).saturating_add(parts(value)))
                .fold(0, usize::saturating_add),
            _ => 0,
        }
    }
    // A value that cannot be measured counts as too large to keep.
    serde_json::to_value(value).map_or(usize::MAX, |value| parts(&value))
}

/// What one row of a kept state weighs. `row` gives what a step can change, and `kept` gives
/// what a row is, which `Step::before` takes from the row after the step. Every field is
/// named, so that a new field is counted. The definition is shared with the run's own row.
fn row_weight(row: &Participant, kept: &Participant) -> usize {
    let Participant {
        key: _,
        npc: _,
        initiative_group,
        initiative_mode: _,
        name: _,
        side: _,
        definition: _,
        initial: _,
        max_hp: _,
        hp: _,
        temporary_hp: _,
        armor_class: _,
        initiative_bonus: _,
        initiative: _,
        shared_roll,
        present: _,
        conditions,
        concentration: _,
        successes: _,
        failures: _,
        notes,
        combat_values,
        started,
        changes: _,
        added_from: _,
    } = row;
    [
        std::mem::size_of::<Participant>(),
        kept.key.len() + kept.name.len() + kept.side.len() + notes.len(),
        owned(&kept.initial),
        owned(&kept.changes),
        owned(&kept.added_from),
        owned(initiative_group),
        owned(shared_roll),
        owned(conditions),
        owned(combat_values),
        owned(started),
    ]
    .into_iter()
    .fold(0, usize::saturating_add)
}
