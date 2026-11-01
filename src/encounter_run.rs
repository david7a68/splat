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
    #[serde(default)]
    pub combat_values: BTreeMap<String, serde_json::Value>,
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
        let conditions = initial
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
            conditions,
            concentration: false,
            successes: counter("successes"),
            failures: counter("failures"),
            notes: String::new(),
            combat_values: BTreeMap::new(),
            definition: Arc::new(entry),
            initial,
        })
    }
    fn ready(&self) -> bool {
        matches!((self.hp, self.max_hp), (Some(hp), Some(max)) if max > 0 && hp >= 0 && hp <= max)
            && self.initiative.is_some()
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
    notice: Option<String>,
    setup_inputs: BTreeMap<usize, String>,
    /// The participant whose initiative the last step set, to merge keystrokes.
    last_initiative: Option<usize>,
    /// Lookup is beside the run, so the run's own panel gives up its space.
    pub hide_panel: bool,
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
/// What the run strip shows on other pages.
pub struct Strip {
    pub round: u64,
    pub current: Option<String>,
    pub next: Option<String>,
    pub conflict: Option<String>,
    pub running: bool,
    pub name: String,
}
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
            setup_inputs: BTreeMap::new(),
            last_initiative: None,
            hide_panel: false,
            content: None,
            lookup_request: None,
            close_lookup: false,
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
    fn apply(&mut self, command: Command) -> Result<(), String> {
        if matches!(self.state.phase, Phase::Ended { .. }) {
            return Err("This encounter has ended. Undo the ending to continue.".into());
        }
        // Merge typed digits of one initiative into one undo step.
        let initiative_edit = match &command {
            Command::Initiative(index, _) => Some(*index),
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
                    return Err(
                        "No participants are present. Mark one present or end the encounter."
                            .into(),
                    );
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
                format!(
                    "{amount} damage applied ({absorbed} absorbed by temporary HP); {} HP remains.",
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
                if state.phase != Phase::Setup {
                    return Err("Initiative is fixed during combat.".into());
                }
                p.initiative = total;
                format!("Initiative set to {}.", display(total))
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
                if present {
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
            self.history_labels.pop();
            self.state = previous;
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
            && let Err(e) = self.save()
        {
            self.error = Some(e);
        }
    }
    pub fn strip(&self) -> Strip {
        let name = |index: usize| self.state.participants[index].name.clone();
        let conflict = self
            .syncs
            .iter()
            .find(|(_, sync)| sync.state() == crate::combat_sync::SyncState::Conflict)
            .map(|(index, _)| name(*index));
        Strip {
            round: self.state.round,
            current: self.current().map(name),
            next: self.next_up().map(name),
            conflict,
            running: self.state.phase == Phase::Running,
            name: self.name.clone(),
        }
    }
    /// The strip's Next turn, usable from any page.
    pub fn next_turn(&mut self) {
        if let Err(error) = self.apply(Command::Next) {
            self.error = Some(error);
        } else {
            self.amount.clear();
            self.scroll_to_selected = true;
            self.focus_amount = true;
        }
    }
    /// Show the first participant with a conflict, so it can be resolved.
    pub fn review_conflict(&mut self) {
        if let Some(index) = self
            .syncs
            .iter()
            .find(|(_, sync)| sync.state() == crate::combat_sync::SyncState::Conflict)
            .map(|(index, _)| *index)
        {
            self.selected = index;
            self.scroll_to_selected = true;
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
    /// The names of characters with sync changes not yet confirmed.
    pub fn pending_sync_names(&self) -> Vec<String> {
        self.syncs
            .iter()
            .filter(|(_, sync)| sync.enabled && (sync.busy() || sync.dirty()))
            .filter_map(|(index, _)| self.state.participants.get(*index))
            .map(|p| p.name.clone())
            .collect()
    }
    pub fn pending_sync(&self) -> bool {
        self.syncs
            .values()
            .any(|sync| sync.enabled && (sync.busy() || sync.dirty()))
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

#[derive(Clone, PartialEq)]
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
        if self.connected && !self.sync_to_restore.is_empty() {
            self.resume_restored_sync();
        }
        let mut identities = std::collections::BTreeSet::new();
        let mut changes = Vec::new();
        let mut external_change = false;
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
            let before = SheetShadow::of(p);
            if sync.ready()
                && let Some(shadow) = self.sync_shadows.get(index)
            {
                if let Some(mut hp) = sync.values().get("hp").cloned() {
                    if before.hp != shadow.hp
                        && let (Some(current), Some(max)) = (before.hp, sync.max_hp())
                    {
                        hp["removedHitPoints"] = (max - current).into();
                    }
                    if before.temp != shadow.temp {
                        hp["temporaryHitPoints"] = before.temp.into();
                    }
                    sync.set_group("hp", hp);
                }
                if before.successes != shadow.successes || before.failures != shadow.failures {
                    sync.set_group("death",serde_json::json!({"successCount":before.successes,"failCount":before.failures}));
                }
                if before.conditions != shadow.conditions
                    && let Some(mut conditions) = sync.values().get("conditions").cloned()
                {
                    if let Some(values) = conditions.as_object_mut() {
                        for (id, _) in crate::combat_sync::CONDITIONS {
                            values.remove(&id.to_string());
                        }
                        for condition in &before.conditions {
                            if let Some((id, level)) = recognized_condition(condition) {
                                values.insert(id.to_string(), level);
                            }
                        }
                    }
                    sync.set_group("conditions", conditions);
                }
            }
            sync.tick(ctx, session.clone(), epoch, &campaign, &id);
            if sync.ready() {
                if let Some(max) = sync.max_hp()
                    && let Some(hp) = sync.values().get("hp")
                {
                    p.max_hp = Some(max);
                    p.hp = Some((max - hp["removedHitPoints"].as_i64().unwrap_or(0)).max(0));
                    p.temporary_hp = hp["temporaryHitPoints"].as_i64().unwrap_or(0);
                }
                if let Some(death) = sync.values().get("death") {
                    p.successes = death["successCount"].as_u64().unwrap_or(0).min(3) as u8;
                    p.failures = death["failCount"].as_u64().unwrap_or(0).min(3) as u8;
                }
                if let Some(conditions) = sync
                    .values()
                    .get("conditions")
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
                if sync.updated && before != SheetShadow::of(p) {
                    external_change = true;
                }
                if before != SheetShadow::of(p) || p.combat_values != *sync.values() {
                    p.combat_values = sync.values().clone();
                    changes.push(p.name.clone());
                }
                self.sync_shadows.insert(*index, SheetShadow::of(p));
                sync.updated = false;
            } else {
                self.sync_shadows.remove(index);
            }
        }
        if !changes.is_empty() {
            self.state.log.push(format!(
                "Combat sheet values updated: {}. See sync status for confirmation or conflicts.",
                changes.join(", ")
            ));
            if external_change {
                self.note_sheet_update(&changes);
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
    /// A sheet changed outside Splat, so earlier steps can no longer be undone safely.
    pub(crate) fn note_sheet_update(&mut self, names: &[String]) {
        self.history.clear();
        self.history_labels.clear();
        self.notice = Some(format!(
            "Undo history was cleared: D&D Beyond changed {}.",
            names.join(", ")
        ));
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
            return Err("This run changed on disk. Your current run is kept in memory; close the other instance before saving.".into());
        }
        let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
        temp.write_all(&bytes)
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|e| e.to_string())?;
        // Recheck after staging, just before atomic replacement.
        if fs::read(&storage.path).ok() != storage.baseline {
            return Err("Run changed while saving. The external file was kept.".into());
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
                && (!state.participants.iter().all(Participant::ready) || state.round == 0))
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
            setup_inputs: BTreeMap::new(),
            last_initiative: None,
            hide_panel: false,
            content: None,
            lookup_request: None,
            close_lookup: false,
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
                let modified = fs::metadata(&path)
                    .and_then(|metadata| metadata.modified())
                    .ok()
                    .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|elapsed| elapsed.as_secs());
                match Self::open(&path, campaign) {
                    Ok(run) => {
                        let (status, finished) = match run.state.phase {
                            Phase::Setup => ("Preparing", false),
                            Phase::Running => ("In progress", false),
                            Phase::Ended { .. } => ("Ended", true),
                        };
                        runs.push(SavedRunInfo {
                            path,
                            encounter: run.name.clone(),
                            label: format!(
                                "Round {} · {status}{}",
                                run.state.round,
                                modified.map_or_else(String::new, |at| format!(
                                    " · {} UTC",
                                    calendar_time(at)
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

fn display(value: Option<i64>) -> String {
    value.map_or_else(|| "?".into(), |n| n.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
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
    fn resume_selects_the_current_participant() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Uuid::new_v4();
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Next).unwrap();
        run.apply(Command::Next).unwrap();
        run.configure_storage(campaign, dir.path(), Initiative::default())
            .unwrap();
        let path = run.storage.as_ref().unwrap().path.clone();
        let reopened = Run::open(&path, campaign).unwrap();
        assert_eq!(Some(reopened.selected), reopened.current());
    }
    #[test]
    fn a_sheet_update_clears_undo_and_says_so() {
        let mut run = run();
        begin(&mut run);
        run.apply(Command::Next).unwrap();
        run.note_sheet_update(&["Guard 1".to_owned()]);
        assert!(run.history.is_empty());
        assert_eq!(run.undo_label(), "Undo");
        assert!(
            run.notice
                .as_ref()
                .unwrap()
                .contains("Undo history was cleared")
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
}
