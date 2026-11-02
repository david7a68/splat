//! Manual DM controls. These record rulings; they do not execute creature rules.
mod view;
use crate::encounter_settings::{Grouping, Initiative};
use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_yaml_ng::{Mapping, Value};
use splat::{
    content::Entry,
    execution::dice::{Dice, Random},
};
use std::{collections::BTreeMap, sync::Arc};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;
pub(crate) use view::sync_status;

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
}
fn stat(entry: &Entry, name: &str) -> Option<i64> {
    let stat = entry.fields.get("splat_stats")?.get(name)?;
    stat.get("value").and_then(Value::as_i64)
}
impl Participant {
    pub fn new(
        key: String,
        side: String,
        entry: Entry,
        overrides: &Mapping,
        present: bool,
    ) -> Result<Self, String> {
        let mut initial = entry
            .fields
            .get("splat_initial")
            .and_then(Value::as_mapping)
            .cloned()
            .unwrap_or_default();
        initial.extend(overrides.clone());
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
        let max_hp = stat(&entry, "max_hp").filter(|n| *n > 0);
        let hp = number("hp")?.or(max_hp);
        let temporary_hp = number("temporary_hp")?.unwrap_or(0);
        let conditions: Vec<String> = initial
            .get("conditions")
            .and_then(Value::as_sequence)
            .map(|items| {
                items
                    .iter()
                    .map(|v| {
                        v.as_str()
                            .or_else(|| v.get("name").and_then(Value::as_str))
                            .map(str::to_owned)
                            .unwrap_or_else(|| {
                                serde_yaml_ng::to_string(v)
                                    .unwrap_or_default()
                                    .trim()
                                    .to_owned()
                            })
                    })
                    .collect()
            })
            .unwrap_or_default();
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
            name: entry.name.clone(),
            side,
            max_hp,
            hp,
            temporary_hp,
            armor_class: stat(&entry, "armor_class"),
            initiative_bonus: stat(&entry, "initiative"),
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
            definition: Arc::new(entry),
            initial,
        })
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
pub(crate) struct Run {
    pub name: String,
    id: Uuid,
    initiative: Initiative,
    storage: Option<Storage>,
    dirty: bool,
    state: State,
    history: Vec<State>,
    /// What each history step did, for a named Undo.
    history_labels: Vec<String>,
    selected: usize,
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
    connected: bool,
    /// Test builds have no session. This lets the party-sync control be exercised.
    simulate_connection: bool,
    scroll_to_selected: bool,
    /// What sync changed. It stays until the DM dismisses it, whatever they do next.
    notice: Option<Notice>,
    /// The list leaves no room for the notice, so the header or the Log tab says it instead.
    notice_hidden: bool,
    /// The row marked NEXT is scrolled out of the roster's view.
    next_out_of_view: bool,
    /// `Mark present` opened Record for a total, and its field takes the keyboard.
    focus_initiative: bool,
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
}
#[derive(Clone, Copy, Default, PartialEq)]
enum Panel {
    #[default]
    Reference,
    Conditions,
    Record,
    Log,
    Encounter,
    Keys,
}
const DISK_CONFLICT: &str = "This run changed on disk, perhaps from another copy of Splat or a sync tool. Your run is safe in memory, and saving has stopped until you choose.";
const MAX_FROM_SHEET: &str =
    "Maximum HP comes from the D&D Beyond sheet while sync is on. Stop sync to change it here.";
pub(crate) const NOBODY_PRESENT: &str =
    "No participants are present. Mark one present or end the encounter.";
const CLOSE_BLOCKED: &str =
    "Wait for pending sync changes, resolve conflicts, or select Stop sync before closing.";
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
    pub ended: bool,
    pub name: String,
}
/// What sync changed, in the forms that have room to say it.
#[derive(Clone)]
struct Notice {
    /// From what to what. It goes under the list when the list has the room.
    full: String,
    /// Who changed, for one line under a full list.
    short: String,
}
/// Said in place of a short notice that does not fit its one line.
const NOTICE_LEAST: &str = "Sheet update. Earlier Undo steps ended. See the Log.";
/// Said on the header's line when the list has no room for the notice at all.
const NOTICE_IN_HEADER: &str = "Earlier Undo steps ended. See the Log.";
const NOTICE_IN_HEADER_SHORT: &str = "Undo ended. See the Log.";
#[derive(Clone)]
enum Command {
    RollNpcs,
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
        let mut seen = BTreeMap::new();
        for p in &mut participants {
            let count = seen.entry(p.name.clone()).or_insert(0);
            *count += 1;
            if counts[&p.name] > 1 {
                p.name = format!("{} {}", p.name, count);
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
            initiative: Initiative::default(),
            storage: None,
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
            amount: String::new(),
            condition: String::new(),
            note: String::new(),
            dice: "1d20".into(),
            outcome: "Undecided".into(),
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
            connected: false,
            simulate_connection: false,
            scroll_to_selected: false,
            notice: None,
            notice_hidden: false,
            next_out_of_view: false,
            focus_initiative: false,
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
        })
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
            Command::RollNpcs => {
                if next.phase != Phase::Setup {
                    return Err("Initiative is fixed during combat.".into());
                }
                let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
                let mut group_order = Vec::new();
                for (i, p) in next.participants.iter().enumerate().filter(|(_, p)| p.npc) {
                    let key = match self.initiative.grouping {
                        Grouping::Identical => format!(
                            "{}:{}:{}",
                            p.definition.id,
                            p.initiative_bonus.map_or(String::new(), |v| v.to_string()),
                            p.initiative_mode
                        ),
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
                    let first = i64::from(next.random.die(20)?);
                    let die = if leader.initiative_mode == 0 {
                        first
                    } else {
                        let second = i64::from(next.random.die(20)?);
                        if leader.initiative_mode > 0 {
                            first.max(second)
                        } else {
                            first.min(second)
                        }
                    };
                    let total = die.checked_add(bonus).ok_or("Initiative overflow.")?;
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
                    self.initiative.grouping.label(),
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
                format!("Ended in round {}. Outcome: {outcome}.", next.round)
            }
            Command::Roll(expression) => {
                let roll = Dice::parse(&expression)?.roll(&mut next.random, false)?;
                format!("DM roll {expression}: {:?} = {}.", roll.dice, roll.total)
            }
            other => Self::participant_command(&mut next, other)?,
        };
        next.log.push(label);
        if self.history.len() == 100 {
            self.history.remove(0);
            self.history_labels.remove(0);
        }
        self.history.push(std::mem::replace(&mut self.state, next));
        self.history_labels.push(undo_label);
        self.last_initiative = initiative_edit;
        if follow_turn && let Some(current) = self.current() {
            self.selected = current;
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
                // In combat, only for someone who came back without one.
                if state.phase != Phase::Setup && (p.initiative.is_some() || total.is_none()) {
                    return Err("Initiative is fixed during combat.".into());
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
                let first = i64::from(state.random.die(20)?);
                let die = if p.initiative_mode == 0 {
                    first
                } else {
                    let second = i64::from(state.random.die(20)?);
                    if p.initiative_mode > 0 {
                        first.max(second)
                    } else {
                        first.min(second)
                    }
                };
                let total = die.checked_add(bonus).ok_or("Initiative overflow.")?;
                p.initiative = Some(total);
                format!("Initiative: {die} + {bonus} = {total}.")
            }
            Command::Presence(_, present) => {
                p.present = present;
                match (present, p.initiative.is_some()) {
                    (true, false) if state.phase != Phase::Setup => {
                        "Marked present with no initiative: last in the order until you set it in Record.".into()
                    }
                    (true, false) => "Marked present; keeps state. No initiative yet.".into(),
                    (true, true) => "Marked present; keeps initiative and state.".into(),
                    (false, true) => "Marked absent; keeps initiative and state.".into(),
                    (false, false) => "Marked absent; keeps state. No initiative yet.".into(),
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
            self.dirty = true;
            self.error = None;
            self.last_initiative = None;
        }
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
            Command::RollNpcs => "NPC initiative roll".into(),
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
        }
    }
    /// What Undo would do, or, with no step to take back, why there is none. The words are
    /// on the button because at the table nobody hovers.
    fn undo_label(&self) -> String {
        self.history_labels.last().map_or_else(
            || {
                if self.undo_stopped {
                    "No Undo since sync update".to_owned()
                } else if self.reopened {
                    "No Undo since reopening".to_owned()
                } else {
                    "Undo".to_owned()
                }
            },
            |label| format!("Undo: {label}"),
        )
    }
    /// The same action with no name on it, for a line that has no room for both.
    fn undo_label_short(&self) -> Option<String> {
        let Some(label) = self.history_labels.last() else {
            return (self.undo_stopped || self.reopened).then(|| "No Undo".to_owned());
        };
        [" to ", " on ", " off ", " for "]
            .iter()
            .filter_map(|word| label.find(word))
            .min()
            .map(|end| format!("Undo: {}", &label[..end]))
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
            ended: matches!(self.state.phase, Phase::Ended { .. }),
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
            // The digits of an amount meant for another row must not apply to this one.
            if self.selected != index {
                self.amount.clear();
            }
            self.selected = index;
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
    /// Whose unsent values closing the app would lose, with sync on or off.
    pub(crate) fn lost_on_app_close_names(&self) -> Vec<String> {
        self.sync_names(crate::combat_sync::CombatSync::loses_on_close)
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
            sync.hold_conflict(group.key(), local);
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
                let maximum_before = p.max_hp;
                apply_sheet_values(p, sync.values(), sync.max_hp());
                let after = SheetShadow::of(p);
                // Anything that reached the run here took no step of its own. Undo would put
                // the older value back and send it, so the steps before it end here.
                let changed =
                    !shadow_differences(&before, &after).is_empty() || maximum_before != p.max_hp;
                if changed {
                    let line = change_line(&p.name, &before, &after, (maximum_before, p.max_hp));
                    if sync.updated || first_read {
                        from_sheet.push((p.name.clone(), line));
                    } else {
                        from_panel.push((p.name.clone(), line));
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
            } else {
                self.sync_shadows.remove(index);
            }
        }
        if !changes.is_empty() {
            if !from_sheet.is_empty() || !from_panel.is_empty() {
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
    /// Values reached the run with no step of their own: earlier steps can no longer be undone
    /// safely. The log and the notice each say what changed, from what to what, and that
    /// Undo ended: the log may be the only place the DM reads it.
    fn note_sync_changes(
        &mut self,
        from_sheet: &[(String, String)],
        from_panel: &[(String, String)],
    ) {
        self.history.clear();
        self.history_labels.clear();
        let lines = |changes: &[(String, String)]| {
            changes
                .iter()
                .map(|(_, line)| line.as_str())
                .collect::<Vec<_>>()
                .join("; ")
        };
        let mut what = Vec::new();
        if !from_sheet.is_empty() {
            what.push(format!("From D&D Beyond: {}.", lines(from_sheet)));
        }
        if !from_panel.is_empty() {
            what.push(format!("In Record: {}.", lines(from_panel)));
        }
        let full = format!("{} Earlier Undo steps ended.", what.join(" "));
        self.state.log.push(full.clone());
        let source = if from_sheet.is_empty() {
            "In Record"
        } else {
            "Sheet update"
        };
        let mut names = from_sheet.iter().chain(from_panel).map(|(name, _)| name);
        let first = names.next().map_or("", String::as_str);
        let who = match names.count() {
            0 => first.to_owned(),
            more => format!("{first} and {more} more"),
        };
        self.notice = Some(Notice {
            full,
            short: format!("{source}: {who}. Earlier Undo steps ended."),
        });
        self.undo_stopped = true;
    }
    /// A sheet changed outside Splat: who, and the words for what changed.
    #[cfg(test)]
    pub(crate) fn note_sheet_update(&mut self, changes: &[(String, String)]) {
        self.note_sync_changes(changes, &[]);
    }
}
#[derive(Serialize, Deserialize)]
struct SavedRun {
    version: u32,
    campaign: Uuid,
    id: Uuid,
    name: String,
    initiative: Initiative,
    state: State,
    setup_details: String,
}
struct Storage {
    path: PathBuf,
    campaign: Uuid,
    baseline: Option<Vec<u8>>,
}
impl Run {
    pub fn needs_save(&self) -> bool {
        self.dirty
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
            initiative: self.initiative.clone(),
            state: self.state.clone(),
            setup_details: self.setup_details.clone(),
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
    /// Why a stopped row's conflict cannot be settled, and what to do, in the words on screen.
    fn why_choices_stopped(&self) -> &'static str {
        if !self.connected && !self.simulate_connection {
            crate::combat_sync::STOPPED_CONNECT_FIRST
        } else if matches!(self.party_sync_plan(), PartySync::Stop(_)) {
            crate::combat_sync::STOPPED_STOP_THEN_SYNC
        } else {
            crate::combat_sync::STOPPED_SYNC_AGAIN
        }
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
            || count > 2000
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
        Ok(Self {
            id: saved.id,
            name: saved.name,
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
            dirty: false,
            history: Vec::new(),
            history_labels: Vec::new(),
            amount: String::new(),
            condition: String::new(),
            note: String::new(),
            dice: "1d20".into(),
            outcome: "Undecided".into(),
            error: None,
            confirm_close: false,
            syncs: BTreeMap::new(),
            sync_shadows: BTreeMap::new(),
            panel: Panel::default(),
            closing: false,
            ending: false,
            focus_amount: true,
            roster_height: 0.0,
            sync_to_restore: Vec::new(),
            connected: false,
            simulate_connection: false,
            scroll_to_selected: true,
            notice: None,
            notice_hidden: false,
            next_out_of_view: false,
            focus_initiative: false,
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
                    Ok(run) => {
                        let (status, finished) = match run.state.phase {
                            Phase::Setup => ("Preparing", false),
                            Phase::Running => ("In progress", false),
                            Phase::Ended { .. } => ("Ended", true),
                        };
                        // The turn it stopped on, so a DM can tell two saves apart at a glance.
                        let turn = run
                            .current()
                            .filter(|_| !finished)
                            .map(|i| format!(" · {}’s turn", run.state.participants[i].name))
                            .unwrap_or_default();
                        runs.push(SavedRunInfo {
                            path,
                            encounter: run.name.clone(),
                            label: format!(
                                "Round {} · {status}{turn}{}",
                                run.state.round,
                                modified.map_or_else(String::new, |at| format!(
                                    " · Saved\u{a0}{}",
                                    // The phrase stays on one line when the label wraps.
                                    crate::party::fetch_age(
                                        crate::hermetic::now().saturating_sub(at)
                                    )
                                    .replace(' ', "\u{a0}")
                                )),
                            ),
                            finished,
                            readable: true,
                        });
                    }
                    Err(_) => {
                        let name = path.file_name().unwrap_or_default().to_string_lossy();
                        runs.push(SavedRunInfo {
                            label: format!("Unreadable run: {name}"),
                            encounter: String::new(),
                            finished: false,
                            readable: false,
                            path,
                        });
                    }
                }
            }
        }
        runs.sort_by(|a, b| a.encounter.cmp(&b.encounter).then(a.label.cmp(&b.label)));
        Ok(runs)
    }
}

/// One saved run, as the Encounters page lists it.
pub struct SavedRunInfo {
    pub path: PathBuf,
    pub encounter: String,
    pub label: String,
    pub finished: bool,
    pub readable: bool,
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
    /// The index and D&D Beyond identity of a linked participant.
    fn simulated_member(&self, name: &str) -> (usize, u64, i64) {
        let (index, participant) = self
            .state
            .participants
            .iter()
            .enumerate()
            .find(|(_, p)| p.name == name)
            .expect("participant in this run");
        let (_, character) = remote_identity(participant).expect("D&D Beyond participant");
        (
            index,
            character.parse().expect("numeric ID"),
            participant.max_hp.unwrap_or(1),
        )
    }
    /// A sheet that holds what this participant holds now, with this much damage. Temporary HP,
    /// death saves and conditions stay as they are, as the simulated change is only to HP.
    fn simulated_sheet(
        p: &Participant,
        character: u64,
        removed: i64,
        can_edit: bool,
    ) -> serde_json::Value {
        let conditions: Vec<_> = p
            .conditions
            .iter()
            .filter_map(|condition| recognized_condition(condition))
            .map(|(id, level)| serde_json::json!({"id": id, "level": level}))
            .collect();
        serde_json::json!({
            "id": character,
            "canEdit": can_edit,
            "removedHitPoints": removed,
            "temporaryHitPoints": p.temporary_hp,
            "deathSaves": {"successCount": p.successes, "failCount": p.failures},
            "conditions": conditions,
        })
    }
    /// A read of this sheet with the damage it had at the last match, plus `more_damage`. Test
    /// builds have no live sheet, so this goes through the same comparison that a live read does.
    pub fn simulate_read(&mut self, name: &str, more_damage: i64) {
        let (index, character, max_hp) = self.simulated_member(name);
        let participant = &self.state.participants[index];
        let damage = participant
            .combat_values
            .get("hp")
            .and_then(|hp| hp["removedHitPoints"].as_i64())
            .unwrap_or(0);
        let sheet = Self::simulated_sheet(participant, character, damage + more_damage, true);
        let sync = self.syncs.get_mut(&index).expect("a running sync");
        sync.stop_simulating();
        sync.simulate_sheet(character, max_hp, &sheet);
    }
    pub fn stop_simulating_sync(&mut self) {
        self.syncs
            .values_mut()
            .for_each(crate::combat_sync::CombatSync::stop_simulating);
    }
    pub fn simulate_error(&mut self, message: &str) {
        self.error = (!message.is_empty()).then(|| message.to_owned());
    }
    /// Show one participant in a sync state test builds cannot reach by themselves. A state
    /// with a change of the DM's first makes that change as a real step, so that what the run
    /// holds and what it tells the DM are what they would be in play.
    pub fn simulate_sync_state(&mut self, name: &str, state: crate::combat_sync::SyncState) {
        use crate::combat_sync::{CombatSync, SyncState};
        let (index, character, max_hp) = self.simulated_member(name);
        let participant = &self.state.participants[index];
        let hp = participant.hp.unwrap_or(max_hp);
        let sheet = Self::simulated_sheet(
            participant,
            character,
            max_hp - hp,
            state != SyncState::ReadOnly,
        );
        let change = match state {
            SyncState::Conflict => 4,
            SyncState::Pending => 1,
            _ => 0,
        };
        if change > 0 {
            self.apply(Command::Health(
                index,
                Some((hp - change).max(0)),
                Some(max_hp),
            ))
            .expect("the DM's change");
        }
        let participant = &self.state.participants[index];
        let local = serde_json::json!({
            "removedHitPoints": max_hp - participant.hp.unwrap_or(max_hp),
            "temporaryHitPoints": participant.temporary_hp,
        });
        let shadow = SheetShadow::of(participant);
        self.syncs.insert(
            index,
            CombatSync::simulated_from(state, character, max_hp, &sheet, local),
        );
        self.sync_shadows.insert(index, shadow);
    }
    /// Give one participant a D&D Beyond conflict.
    pub fn simulate_sync_conflict(&mut self, name: &str) {
        self.simulate_sync_state(name, crate::combat_sync::SyncState::Conflict);
    }
    /// A spell slot spent on this sheet and not sent.
    pub fn simulate_slot_edit(&mut self, name: &str) {
        let (index, ..) = self.simulated_member(name);
        self.syncs
            .get_mut(&index)
            .expect("a running sync")
            .simulate_slot_edit();
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
    // A caster, as the fixture's beasts only bite. With no caster here, the current row.
    let caster = run
        .state
        .participants
        .iter()
        .position(|p| p.name.contains("Iselle"))
        .unwrap_or(run.state.order[run.state.cursor]);
    run.apply(Command::Concentration(caster, true))?;
    for _ in 0..run.state.order.len() {
        run.apply(Command::Next)?;
    }
    run.configure_storage(campaign, directory, Initiative::default())
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
        assert!(run.apply(Command::Initiative(0, Some(30))).is_err());
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
        assert!(run.state.log.last().unwrap().contains("Ended in round"));
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
        let notice = run.notice.clone().unwrap().full;
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
    fn undo_cannot_bring_back_a_value_after_a_conflict_is_settled() {
        let (mut run, _, max) = matched_with_sheet(5);
        offline_tick(&mut run);
        run.apply(Command::Damage(0, 4)).unwrap();
        run.apply(Command::Next).unwrap();
        offline_tick(&mut run);
        reconnect(&mut run);
        read_sheet(&mut run, max, 2);
        assert_eq!(run.state.participants[0].hp, Some(max - 9));
        assert!(
            !run.history.is_empty(),
            "the conflict keeps the steps before it"
        );
        run.syncs.get_mut(&0).unwrap().use_remote("hp");
        connected_tick(&mut run);
        assert_eq!(run.state.participants[0].hp, Some(max - 2));
        // Nothing to take back under another name, and the notice says what happened.
        assert!(run.history.is_empty());
        assert_eq!(run.undo_label(), "No Undo since sync update");
        let notice = run.notice.clone().unwrap().full;
        let change = format!("HP {} → {}", max - 9, max - 2);
        assert!(notice.contains(&change), "{notice}");
        assert!(notice.contains("From D&D Beyond"), "{notice}");
        assert!(run.state.log.iter().any(|line| line.contains(&change)));
        // Undo does nothing, so nothing older reaches the sheet.
        run.undo();
        connected_tick(&mut run);
        assert_eq!(run.state.participants[0].hp, Some(max - 2));
        assert_eq!(run.syncs[&0].values()["hp"]["removedHitPoints"], 2);
        assert!(!run.syncs[&0].dirty(), "nothing older goes to D&D Beyond");
    }
    #[test]
    fn an_edit_in_the_sync_panel_ends_the_steps_before_it_and_says_so() {
        let (mut run, _, max) = matched_with_sheet(5);
        run.apply(Command::Next).unwrap();
        connected_tick(&mut run);
        assert!(!run.history.is_empty());
        let edit = serde_json::json!({"removedHitPoints": 8, "temporaryHitPoints": 0});
        run.syncs.get_mut(&0).unwrap().set_group("hp", edit);
        connected_tick(&mut run);
        assert_eq!(run.state.participants[0].hp, Some(max - 8));
        assert!(run.history.is_empty());
        let notice = run.notice.clone().unwrap().full;
        assert!(notice.contains("In Record"), "{notice}");
        assert!(
            notice.contains(&format!("HP {} → {}", max - 5, max - 8)),
            "{notice}"
        );
        run.undo();
        connected_tick(&mut run);
        assert_eq!(run.syncs[&0].values()["hp"]["removedHitPoints"], 8);
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
        let notice = run.notice.clone().unwrap().full;
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
        assert!(run.apply(Command::Initiative(0, Some(20))).is_err());
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
        let notice = run.notice.clone().unwrap().full;
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
        connected_tick(&mut run);
        reconnect(&mut run);
        read_sheet(&mut run, max, 0);
        assert_eq!(run.state.participants[0].hp, Some(max));
        let notice = run.notice.clone().unwrap().full;
        assert!(
            notice.contains(&format!("maximum HP {} → {max}", max + 8)),
            "{notice}"
        );
        assert!(run.history.is_empty(), "the named step no longer matches");
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
        assert!(
            run.notice
                .as_ref()
                .unwrap()
                .full
                .contains("Earlier Undo steps ended")
        );
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
        let notice = run.notice.clone().unwrap().full;
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
    /// `Review` moves the selection, so a typed amount must not land on the new row.
    #[test]
    fn review_clears_an_amount_typed_for_another_row() {
        let mut run = run();
        begin(&mut run);
        run.selected = 1;
        run.amount = "12".to_owned();
        run.syncs.insert(
            0,
            crate::combat_sync::CombatSync::simulated_conflict(1, 10, 1),
        );
        run.review_conflict();
        assert_eq!(run.selected, 0);
        assert!(run.amount.is_empty());
        // On the row already selected, the number was meant for this row.
        run.amount = "7".to_owned();
        run.review_conflict();
        assert_eq!(run.amount, "7");
    }
    /// A stopped row's conflict says what starts its sync again, from the case.
    #[test]
    fn a_stopped_conflict_says_what_to_do_in_each_case() {
        use crate::combat_sync::{
            CombatSync, STOPPED_CONNECT_FIRST, STOPPED_STOP_THEN_SYNC, STOPPED_SYNC_AGAIN,
        };
        let mut run = run();
        begin(&mut run);
        linked(&mut run, 0, "11");
        linked(&mut run, 1, "11");
        assert_eq!(run.why_choices_stopped(), STOPPED_CONNECT_FIRST);
        run.connected = true;
        assert_eq!(run.why_choices_stopped(), STOPPED_SYNC_AGAIN);
        // One row runs, and the other stopped with an error: the button says Stop sync.
        let mut on = CombatSync::default();
        on.start();
        run.syncs.insert(0, on);
        let mut stopped = CombatSync::default();
        stopped.error = Some("Another participant is already syncing this character.".into());
        run.syncs.insert(1, stopped);
        assert_eq!(run.why_choices_stopped(), STOPPED_STOP_THEN_SYNC);
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
        run.apply(Command::RollNpcs).unwrap();
        assert_eq!(run.state.participants[0].initiative, Some(19));
        assert_eq!(run.state.participants[1].initiative, Some(12));
        assert_eq!(run.state.participants[2].initiative, Some(12));
        run.undo();
        run.initiative.grouping = Grouping::Individual;
        run.apply(Command::RollNpcs).unwrap();
        assert_eq!(run.state.participants[1].initiative, Some(12));
        assert_eq!(run.state.participants[2].initiative, Some(9));
        run.undo();
        run.initiative.grouping = Grouping::BySide;
        run.apply(Command::RollNpcs).unwrap();
        assert_eq!(run.state.participants[2].initiative, Some(12));
        run.undo();
        run.initiative.grouping = Grouping::Explicit;
        run.state.participants[1].initiative_group = Some("wave".into());
        run.state.participants[2].initiative_group = Some("wave".into());
        run.apply(Command::RollNpcs).unwrap();
        assert_eq!(run.state.participants[2].initiative, Some(12));
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
    fn a_sheet_update_clears_undo_and_says_so() {
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Next).unwrap();
        run.note_sheet_update(&[("Guard 1".to_owned(), "Guard 1 HP 20 → 17".to_owned())]);
        assert!(run.history.is_empty());
        assert_eq!(run.undo_label(), "No Undo since sync update");
        assert!(
            run.notice
                .as_ref()
                .unwrap()
                .full
                .contains("Earlier Undo steps ended")
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
        run.initiative.grouping = Grouping::Individual;
        run.state.random = Random::controlled([18, 4, 10, 12]);
        run.state.participants[0].initiative_mode = -1;
        run.state.participants[2].initiative_bonus = None;
        assert!(run.apply(Command::RollNpcs).is_err());
        assert!(
            run.state
                .participants
                .iter()
                .all(|p| p.initiative.is_none())
        );
        run.state.participants[2].initiative_bonus = Some(2);
        run.apply(Command::RollNpcs).unwrap();
        assert_eq!(run.state.participants[0].initiative, Some(6));
    }
    /// M9-1, L9-2: who changed fits the one line, and the whole stays for the log and hover.
    #[test]
    fn a_notice_says_who_changed_in_each_form() {
        let mut run = run();
        begin(&mut run);
        let change = |name: &str, line: &str| (name.to_owned(), line.to_owned());
        run.note_sync_changes(
            &[
                change("Guard 1", "Guard 1 HP 20 → 17"),
                change("Guard 2", "Guard 2 HP 9 → 8"),
            ],
            &[change("Guard 3", "Guard 3 HP 5 → 4")],
        );
        let notice = run.notice.clone().unwrap();
        assert!(
            notice
                .full
                .contains("From D&D Beyond: Guard 1 HP 20 → 17; Guard 2 HP 9 → 8.")
        );
        assert!(notice.full.contains("In Record: Guard 3 HP 5 → 4."));
        assert!(notice.full.ends_with("Earlier Undo steps ended."));
        assert_eq!(
            notice.short,
            "Sheet update: Guard 1 and 2 more. Earlier Undo steps ended."
        );
        run.note_sync_changes(&[], &[change("Guard 3", "Guard 3 HP 4 → 3")]);
        assert_eq!(
            run.notice.unwrap().short,
            "In Record: Guard 3. Earlier Undo steps ended."
        );
    }
    /// L6-1: with no step to take back, the button itself says why, in each case.
    #[test]
    fn undo_with_no_step_says_why() {
        let dir = tempfile::tempdir().unwrap();
        let mut run = run();
        begin(&mut run);
        let campaign = stored(&mut run, dir.path());
        let path = run.path().unwrap().to_path_buf();
        let mut reopened = Run::open(&path, campaign).unwrap();
        assert_eq!(reopened.undo_label(), "No Undo since reopening");
        assert_eq!(reopened.undo_label_short().as_deref(), Some("No Undo"));
        reopened.apply(Command::Damage(0, 1)).unwrap();
        assert!(reopened.undo_label().starts_with("Undo: 1 damage to"));
        assert_eq!(
            reopened.undo_label_short().as_deref(),
            Some("Undo: 1 damage")
        );
        reopened.note_sheet_update(&[("Guard 1".to_owned(), "Guard 1 HP 20 → 17".to_owned())]);
        assert_eq!(reopened.undo_label(), "No Undo since sync update");
    }
    /// L8-5: the log says what is true of a participant with no initiative.
    #[test]
    fn presence_words_follow_the_initiative() {
        let mut run = run();
        run.apply(Command::Presence(0, false)).unwrap();
        assert!(run.state.log.last().unwrap().contains("No initiative yet"));
        run.apply(Command::Presence(0, true)).unwrap();
        run.apply(Command::Initiative(0, Some(3))).unwrap();
        run.apply(Command::Presence(0, false)).unwrap();
        assert!(run.state.log.last().unwrap().contains("keeps initiative"));
    }
}
