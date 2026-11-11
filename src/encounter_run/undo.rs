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
/// The most memory the steps read from a file may take, as `weight` counts it. Tests use
/// smaller budgets for both, so that they can reach them with small files.
const MEMORY_BYTES: usize = if cfg!(test) {
    4 * 1024 * 1024
} else {
    256 * 1024 * 1024
};

/// The state before each step, oldest first, with each step's name and whether a sheet made it.
pub(super) type History = (Vec<State>, Vec<String>, Vec<bool>);

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
                let mut row = row.clone();
                if kept.is_some_and(|kept| Arc::ptr_eq(&kept.definition, &row.definition)) {
                    row.definition = same_definition();
                }
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

    /// The state before this step, given the state after it, and what each of its rows weighs.
    /// A step changes rows and never names a new one: each of its rows is a row the state
    /// after it has, in the same place, with the same key. Anything else is not a step this
    /// version wrote, and no definition is taken for it.
    fn before(self, after: &State, after_weights: &[usize]) -> Option<(State, Vec<usize>)> {
        let mut rows = self.rows;
        let mut weights = Vec::new();
        let mut participants = Vec::new();
        for at in 0..self.count {
            let kept = after.participants.get(at)?;
            match rows.remove(&at) {
                Some(mut row) => {
                    if row.key != kept.key {
                        return None;
                    }
                    let own_definition = !is_same_definition(&row.definition);
                    if !own_definition {
                        row.definition = kept.definition.clone();
                    }
                    weights.push(row_weight(&row, own_definition));
                    participants.push(row);
                }
                None => {
                    weights.push(*after_weights.get(at)?);
                    participants.push(kept.clone());
                }
            }
        }
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
            cast: None,
        };
        // A row for a place the state does not have is not a step this version wrote.
        (rows.is_empty() && super::valid_state(&state)).then_some((state, weights))
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
/// One step is decoded at a time, and only while its text and the state it gives fit.
pub(super) fn read(saved: &RawValue, now: &State) -> Option<History> {
    let saved: Saved<Newest<'_>> = serde_json::from_str(saved.get()).ok()?;
    if saved.version != VERSION {
        return None;
    }
    let mut history = (Vec::new(), Vec::new(), Vec::new());
    let mut memory: usize = 0;
    // The run owns its current rows and their definitions. A state read here is charged for
    // its own copy of each row.
    let mut weights: Vec<usize> = now
        .participants
        .iter()
        .map(|row| row_weight(row, false))
        .collect();
    // The newest step is next to the current state, so the states are rebuilt from the end.
    for text in saved.steps.0.iter().rev() {
        if memory.saturating_add(text.get().len()) > MEMORY_BYTES {
            break;
        }
        let mut step: Step = serde_json::from_str(text.get()).ok()?;
        let label = std::mem::take(&mut step.label);
        let from_sheet = step.from_sheet;
        let (state, rows) = step.before(history.0.last().unwrap_or(now), &weights)?;
        memory = memory.saturating_add(weight(&state, &rows));
        if memory > MEMORY_BYTES {
            break;
        }
        history.0.push(state);
        history.1.push(label);
        history.2.push(from_sheet);
        weights = rows;
    }
    history.0.reverse();
    history.1.reverse();
    history.2.reverse();
    Some(history)
}

/// How many bytes a value takes as JSON: the measure of the text and numbers it owns.
fn json_len<T: Serialize + ?Sized>(value: &T) -> usize {
    struct Count(usize);
    impl std::io::Write for Count {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0 += bytes.len();
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut count = Count(0);
    // A value that cannot be measured counts as too large to keep.
    serde_json::to_writer(&mut count, value).map_or(usize::MAX, |()| count.0)
}

/// What one row of a kept state weighs. Every field is named, so that a new field is counted.
/// A definition shared with the state after is one allocation, which that state has.
fn row_weight(row: &Participant, own_definition: bool) -> usize {
    let Participant {
        key,
        npc: _,
        initiative_group,
        initiative_mode: _,
        name,
        side,
        definition,
        initial,
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
        changes,
        added_from,
    } = row;
    [
        std::mem::size_of::<Participant>(),
        key.len() + name.len() + side.len() + notes.len(),
        json_len(initiative_group),
        json_len(shared_roll),
        json_len(conditions),
        json_len(initial),
        json_len(combat_values),
        json_len(started),
        json_len(changes),
        json_len(added_from),
        if own_definition {
            json_len(definition)
        } else {
            0
        },
    ]
    .into_iter()
    .fold(0, usize::saturating_add)
}

/// What one kept state weighs: its rows, as `row_weight` counts them, and everything else it
/// owns. This is the one count that the memory budget uses.
fn weight(state: &State, rows: &[usize]) -> usize {
    let State {
        participants: _,
        order,
        cursor: _,
        round: _,
        phase,
        log,
        last_change,
        random,
        loot,
        roll_results: _,
        cast: _,
    } = state;
    let lines = log
        .iter()
        .map(|line| line.len() + std::mem::size_of::<String>());
    rows.iter()
        .copied()
        .chain(lines)
        .chain([
            std::mem::size_of_val(order.as_slice()),
            json_len(phase),
            json_len(last_change),
            json_len(random),
            json_len(loot),
        ])
        .fold(0, usize::saturating_add)
}
