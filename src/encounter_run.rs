//! Manual DM controls. These record rulings; they do not execute creature rules.
use crate::encounter_settings::{Grouping, Initiative};
use crate::markdown_view;
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
    selected: usize,
    amount: i64,
    condition: String,
    note: String,
    dice: String,
    outcome: String,
    error: Option<String>,
    confirm_close: bool,
    setup_details: String,
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
            Uuid::new_v4().as_bytes()[..8]
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
            selected: 0,
            amount: 1,
            condition: String::new(),
            note: String::new(),
            dice: "1d20".into(),
            outcome: "Undecided".into(),
            error: None,
            confirm_close: false,
            setup_details,
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
        }
        self.history.push(std::mem::replace(&mut self.state, next));
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
            self.state = previous;
            self.dirty = true;
            self.error = None;
        }
    }
    /// Returns true only after the user chooses to discard the in-memory run.
    pub fn show(&mut self, ui: &mut egui::Ui) -> bool {
        ui.visuals_mut().override_text_color = Some(egui::Color32::from_gray(215));
        ui.visuals_mut().weak_text_color = Some(egui::Color32::from_gray(165));
        ui.heading(format!("DM view · {}", self.name));
        ui.label(
            "You resolve attacks and rules. Apply final damage after resistance and other effects.",
        );
        ui.weak("Changes are saved to this run. Templates and character records stay separate.");
        if let Some(storage) = &self.storage {
            ui.weak(if self.dirty {
                "Unsaved run changes"
            } else {
                "Run saved automatically"
            })
            .on_hover_text(storage.path.display().to_string());
        }
        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
        let mut command = None;
        ui.horizontal_wrapped(|ui| {
            match &self.state.phase {
                Phase::Setup => {
                    if ui.button("Roll NPC initiative").clicked() {
                        command = Some(Command::RollNpcs);
                    }
                    if ui.button("Begin combat").clicked() {
                        command = Some(Command::Begin);
                    }
                }
                Phase::Running => {
                    ui.strong(format!("Round {}", self.state.round));
                    if ui.button("Next turn").clicked() {
                        command = Some(Command::Next);
                    }
                }
                Phase::Ended { outcome } => {
                    ui.strong(format!(
                        "Ended manually · {outcome} · Round {}",
                        self.state.round
                    ));
                }
            }
            if ui
                .add_enabled(!self.history.is_empty(), egui::Button::new("Undo"))
                .clicked()
            {
                self.undo();
            }
            if ui.button("Save and close run").clicked() {
                match self.save() {
                    Ok(()) => self.confirm_close = true,
                    Err(e) => self.error = Some(e),
                }
            }
            if ui.button("Save now").clicked()
                && let Err(e) = self.save()
            {
                self.error = Some(e);
            }
        });
        if self.confirm_close {
            return true;
        }
        if let Some(error) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
        if self.state.phase == Phase::Setup {
            ui.label(format!(
                "Campaign initiative: {} · {}",
                self.initiative.grouping.label(),
                if self.initiative.roster_ties {
                    "Roster order breaks ties"
                } else {
                    "DM / players resolve ties"
                }
            ));
            if matches!(
                self.initiative.grouping,
                Grouping::BySide | Grouping::Explicit
            ) {
                ui.weak(
                    "Each group uses the first NPC in roster order for its bonus and roll mode.",
                );
            }
            ui.label("Enter initiative totals or roll each participant. Use the same total for a group. Resolve ties with Up and Down before combat begins.");
        }
        let ended = matches!(self.state.phase, Phase::Ended { .. });
        let current = self.current();
        egui::ScrollArea::vertical().scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible).id_salt("dm_run").show(ui, |ui| {
            for (position, &index) in self.state.order.iter().enumerate() {
                let p = &self.state.participants[index];
                ui.push_id(&p.key, |ui| { ui.group(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        if current == Some(index) { ui.colored_label(egui::Color32::LIGHT_BLUE, "CURRENT TURN"); }
                        if ui.selectable_label(self.selected == index, egui::RichText::new(&p.name).strong()).clicked() { self.selected = index; self.condition.clear(); self.note.clear(); }
                        ui.weak(&p.side);
                        ui.label(format!("HP {}/{} · Temp {} · AC {}", display(p.hp), display(p.max_hp), p.temporary_hp, display(p.armor_class)));
                        if self.state.phase != Phase::Setup { ui.label(format!("Initiative {}", display(p.initiative))); }
                        if !p.present { ui.weak("Absent"); }
                        if p.hp == Some(0) { ui.colored_label(egui::Color32::YELLOW, "0 HP · resolve death or unconsciousness"); }
                        if p.concentration { ui.label("Concentrating"); }
                        if !p.conditions.is_empty() { ui.label(p.conditions.join(", ")); }
                    });
                    if self.state.phase == Phase::Setup { ui.horizontal_wrapped(|ui| {
                        let mut npc = p.npc; if ui.checkbox(&mut npc, "NPC").changed() { command = Some(Command::Npc(index, npc)); }
                        let mut total = p.initiative; optional_number(ui, "Initiative", &mut total, -1000..=1000);
                        if total != p.initiative { command = Some(Command::Initiative(index, total)); }
                        let mut mode = p.initiative_mode;
                        egui::ComboBox::from_id_salt("initiative_mode").selected_text(mode_label(mode)).show_ui(ui, |ui| { for m in [-1, 0, 1] { ui.selectable_value(&mut mode, m, mode_label(m)); } });
                        if mode != p.initiative_mode { command = Some(Command::RollMode(index, mode)); }
                        if ui.add_enabled(p.initiative_bonus.is_some(), egui::Button::new("Roll")).clicked() { command = Some(Command::RollInitiative(index)); }
                        if ui.add_enabled(position > 0 && !self.initiative.roster_ties, egui::Button::new("Up")).clicked() { command = Some(Command::Move(position, position - 1)); }
                        if ui.add_enabled(position + 1 < self.state.order.len() && !self.initiative.roster_ties, egui::Button::new("Down")).clicked() { command = Some(Command::Move(position, position + 1)); }
                    }); }
                }); });
            }
            ui.separator();
            let p = &self.state.participants[self.selected]; let index = self.selected;
            ui.heading(&p.name);
            ui.add_enabled_ui(!ended, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let mut present = p.present; if ui.checkbox(&mut present, "Present in turn order").changed() { command = Some(Command::Presence(index, present)); }
                    let mut concentrating = p.concentration; if ui.checkbox(&mut concentrating, "Concentrating").changed() { command = Some(Command::Concentration(index, concentrating)); }
                });
                ui.horizontal_wrapped(|ui| {
                    ui.label("Amount"); ui.add(egui::DragValue::new(&mut self.amount).range(0..=1_000_000));
                    if ui.button("Apply damage").clicked() { command = Some(Command::Damage(index, self.amount)); }
                    if ui.button("Heal").clicked() { command = Some(Command::Heal(index, self.amount)); }
                    if ui.button("Replace temporary HP").clicked() { command = Some(Command::TempHp(index, self.amount)); }
                });
                ui.collapsing("Adjust HP and initiative bonus", |ui| {
                    let mut hp = p.hp; let mut max = p.max_hp;
                    ui.horizontal_wrapped(|ui| { optional_number(ui, "Current HP", &mut hp, 0..=1_000_000); optional_number(ui, "Maximum HP", &mut max, 1..=1_000_000); });
                    if hp != p.hp || max != p.max_hp { command = Some(Command::Health(index, hp, max)); }
                    if self.state.phase == Phase::Setup {
                        let mut bonus = p.initiative_bonus; optional_number(ui, "Initiative bonus", &mut bonus, -1000..=1000);
                        if bonus != p.initiative_bonus { command = Some(Command::Bonus(index, bonus)); }
                    }
                });
                ui.horizontal_wrapped(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.condition).hint_text("Condition or effect"));
                    if ui.button("Add condition").clicked() { command = Some(Command::Condition(index, self.condition.clone())); }
                });
                for (i, condition) in p.conditions.iter().enumerate() { ui.horizontal(|ui| { ui.label(condition); if ui.small_button("Remove").clicked() { command = Some(Command::RemoveCondition(index, i)); } }); }
                ui.horizontal_wrapped(|ui| {
                    ui.label("Death saves (manual)"); let mut successes = p.successes; let mut failures = p.failures;
                    ui.label("Successes"); ui.add(egui::DragValue::new(&mut successes).range(0..=3)); ui.label("Failures"); ui.add(egui::DragValue::new(&mut failures).range(0..=3));
                    if successes != p.successes || failures != p.failures { command = Some(Command::DeathSaves(index, successes, failures)); }
                });
                ui.weak("Conditions, Concentration checks, death saves, and recovery are resolved by the DM. Zero HP does not remove a turn.");
                ui.horizontal_wrapped(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.note).hint_text("DM note: resources, targets, rulings…"));
                    if ui.button("Record note").clicked() { command = Some(Command::Note(index, self.note.clone())); }
                });
            });
            if !p.notes.is_empty() { ui.label(&p.notes); }
            ui.collapsing("Creature details (starting snapshot)", |ui| {
                markdown_view::show(ui, &p.definition.body);
                ui.collapsing("Structured stats and starting state", |ui| {
                    ui.monospace(serde_yaml_ng::to_string(&p.definition.fields).unwrap_or_default());
                    ui.monospace(serde_yaml_ng::to_string(&p.initial).unwrap_or_default());
                });
            });
            ui.collapsing("Encounter setup and DM notes", |ui| { markdown_view::show(ui, &self.setup_details); });
            ui.separator();
            ui.add_enabled_ui(!ended, |ui| {
                ui.horizontal_wrapped(|ui| { ui.label("Dice"); ui.text_edit_singleline(&mut self.dice); if ui.button("Roll dice").clicked() { command = Some(Command::Roll(self.dice.clone())); } });
                if self.state.phase == Phase::Running {
                    ui.horizontal_wrapped(|ui| { ui.label("Outcome"); ui.text_edit_singleline(&mut self.outcome);
                        if ui.button("End encounter").clicked() { command = Some(Command::End(if self.outcome.trim().is_empty() { "Undecided".into() } else { self.outcome.trim().into() })); }
                    });
                }
            });
            ui.heading("Combat log");
            for line in self.state.log.iter().rev() { ui.label(line); }
        });
        if let Some(command) = command {
            if let Err(error) = self.apply(command) {
                self.error = Some(error);
            } else {
                self.condition.clear();
                self.note.clear();
            }
        }
        if self.dirty
            && self.storage.is_some()
            && let Err(e) = self.save()
        {
            self.error = Some(e);
        }
        false
    }
}
fn mode_label(mode: i8) -> &'static str {
    match mode {
        -1 => "Disadvantage / surprised",
        1 => "Advantage",
        _ => "Normal roll",
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
            state: saved.state,
            setup_details: saved.setup_details,
            storage: Some(Storage {
                path: path.into(),
                campaign,
                baseline: Some(bytes),
            }),
            dirty: false,
            history: Vec::new(),
            selected: 0,
            amount: 1,
            condition: String::new(),
            note: String::new(),
            dice: "1d20".into(),
            outcome: "Undecided".into(),
            error: None,
            confirm_close: false,
        })
    }
    pub fn list(directory: &Path, campaign: Uuid) -> Result<Vec<(PathBuf, String)>, String> {
        if !directory.exists() {
            return Ok(Vec::new());
        }
        let mut runs = Vec::new();
        for file in fs::read_dir(directory).map_err(|e| e.to_string())? {
            let path = file.map_err(|e| e.to_string())?.path();
            if path.extension().is_some_and(|e| e == "json") {
                match Self::open(&path, campaign) {
                    Ok(run) => runs.push((
                        path,
                        format!(
                            "{} · Round {} · {}",
                            run.name,
                            run.state.round,
                            match run.state.phase {
                                Phase::Setup => "Preparing",
                                Phase::Running => "In progress",
                                Phase::Ended { .. } => "Ended",
                            }
                        ),
                    )),
                    Err(_) => runs.push((
                        path.clone(),
                        format!(
                            "Unreadable run: {}",
                            path.file_name().unwrap_or_default().to_string_lossy()
                        ),
                    )),
                }
            }
        }
        runs.sort_by(|a, b| a.1.cmp(&b.1));
        Ok(runs)
    }
}

fn display(value: Option<i64>) -> String {
    value.map_or_else(|| "?".into(), |n| n.to_string())
}
fn optional_number(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<i64>,
    range: std::ops::RangeInclusive<i64>,
) {
    let mut known = value.is_some();
    if ui.checkbox(&mut known, label).changed() {
        *value = if known {
            Some(*range.start().max(&0))
        } else {
            None
        };
    }
    if let Some(n) = value {
        ui.add(egui::DragValue::new(n).range(range));
    } else {
        ui.weak("Unknown");
    }
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

    #[test]
    #[ignore = "Creates synthetic campaign data for screenshot review in SPLAT_UI_FIXTURE"]
    fn create_screenshot_fixture() {
        let root = PathBuf::from(
            std::env::var("SPLAT_UI_FIXTURE").expect("Set SPLAT_UI_FIXTURE to a temporary folder"),
        );
        fs::create_dir_all(root.join("campaign")).unwrap();
        let campaign = crate::persistence::Campaign {
            id: Uuid::new_v4(),
            name: "Northreach · Sample campaign".into(),
            path: root.join("campaign"),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let mut document = crate::content_workspace::CampaignDocument::load(&campaign).unwrap();
        document.save().unwrap();
        let data = crate::persistence::UserData {
            active_campaign_id: Some(campaign.id),
            campaigns: vec![campaign.clone()],
            ..Default::default()
        };
        let mut store = crate::persistence::Store::at(root.join("config/splat/settings.json"));
        store.load().unwrap();
        store.save(&data).unwrap();
        let mut run = run();
        run.name = "Ambush at the Old Bridge".into();
        run.state.participants[0].name = "Mira · Ranger".into();
        run.state.participants[0].npc = false;
        run.state.participants[0].side = "party".into();
        run.state.participants[0].conditions.clear();
        run.state.participants[1].name = "Bridge guard 1".into();
        run.state.participants[2].name = "Bridge guard 2".into();
        run.apply(Command::Initiative(0, Some(17))).unwrap();
        run.configure_storage(
            campaign.id,
            &campaign.path.join("Encounter Runs"),
            Initiative::default(),
        )
        .unwrap();
    }
}
