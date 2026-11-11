//! Rolls during a run, as in a tactics game. A tile in the stat block or sheet arms its
//! action, and the panel stays on the actor while it is armed. Each row then shows the chance
//! to hit it. A click on a row targets it: a creature's attack rolls with its damage, which
//! goes into Amount for that row, and Enter applies it. A player rolls their own dice, so a
//! party member's attack only targets the row and the DM types what the player says. A save
//! adds each clicked row as a target and rolls its save, shown where the row's AC is. A roll
//! changes no participant by itself: it goes to the Log, and Undo keeps it.
use super::Run;
use crate::rolls::{self, ArmedAction, ArmedView, Ask, Chip, Mode, Outcome, Rolled};
use eframe::egui;
use splat::content::Entry;
use splat::content::dice_text::{self, D20, DamagePart, RollLink, When};
use splat::content::inspiration::Inspiration;
use splat::execution::dice::Random;
use std::collections::BTreeMap;

/// An armed action: whose, which, and how its d20 rolls.
#[derive(Clone, Debug)]
pub(super) struct Armed {
    pub actor: usize,
    pub link: RollLink,
    pub mode: Mode,
    /// The actor expends inspiration for advantage on the roll made for them.
    pub inspired: bool,
}

/// A target that waits for the DM to say which damage condition holds, as in `if the goat
/// moved 20+ feet straight toward the target`. Each choice is its words and its damage.
#[derive(Clone, Debug)]
pub(super) struct Asking {
    pub row: usize,
    pub mode: Mode,
    pub inspired: bool,
    /// What the DM is asked, such as `If the goat moved 20+ feet …?`
    pub question: String,
    pub choices: Vec<(String, Vec<DamagePart>)>,
}

/// A question for the DM, and its choices of damage.
type Question = (String, Vec<(String, Vec<DamagePart>)>);

/// The last roll the run made for a player, of which their inspiration can reroll one die.
#[derive(Clone, Debug)]
pub(super) struct Reroll {
    pub actor: usize,
    /// The row that shows the result: the target, or the actor for a check.
    pub shown: usize,
    pub outcome: Outcome,
    /// The save whose damage this is, rolled once for all of its targets.
    pub save: Option<Armed>,
}

/// What a row's right-click menu chose.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum RowChoice {
    /// Roll the armed action against the row, or the row's save, this way.
    Roll(Mode, bool),
    /// The damage of this choice, for the asking row.
    Damage(usize),
    /// Reroll this die of the row's result with inspiration.
    Reroll(rolls::Die),
    /// Close the question without rolling.
    Cancel,
}

/// `link` with this damage in place of its own.
fn with_damage(link: &RollLink, parts: Vec<DamagePart>) -> RollLink {
    match link.clone() {
        RollLink::D20 {
            what, bonus, label, ..
        } => RollLink::D20 {
            what,
            bonus,
            label,
            damage: parts,
        },
        RollLink::Damage { label, .. } => RollLink::Damage { label, parts },
        other => other,
    }
}

/// The damage parts of an attack, a save or a damage roll.
fn damage_of(link: &RollLink) -> &[DamagePart] {
    match link {
        RollLink::D20 { damage, .. } | RollLink::SaveDc { damage, .. } => damage,
        RollLink::Damage { parts, .. } => parts,
        RollLink::Dice { .. } => &[],
    }
}

/// `2d6+2 bludgeoning`, for a choice in the question.
fn parts_words(parts: &[DamagePart]) -> String {
    if parts.is_empty() {
        return "no dice".to_owned();
    }
    rolls::parts_text(parts)
}

/// The most dice a row's menu offers to reroll. A roll of many dice lists its lowest.
const REROLL_CHOICES: usize = 8;

/// The height of the action bar above the dock while Lookup or Notes hide the panel.
pub(super) const STRIP_HEIGHT: f32 = 28.0;

/// Why nothing rolls once the encounter has ended.
const ENDED: &str = "This encounter has ended. Undo the ending to continue.";

/// A save bonus from a stat block: its Save value, or else its ability modifier.
fn save_bonus(entry: &Entry, ability: &str) -> Option<i64> {
    let stat = |key: &str| {
        entry
            .fields
            .get("splat_stats")?
            .get(key)?
            .get("value")?
            .as_i64()
    };
    let ability = ability.to_lowercase();
    stat(&format!("save_{ability}"))
        .or_else(|| stat(&ability).map(|score| (score - 10).div_euclid(2)))
}

/// The chance that d20 + `bonus` meets `armor`, as a whole percent. A natural 1 always
/// misses and a natural 20 always hits; advantage rolls twice and keeps the higher.
pub(super) fn hit_chance(bonus: i64, armor: i64, mode: Mode) -> i64 {
    let faces = (21 - (armor - bonus)).clamp(1, 19) as f64;
    let one = faces / 20.0;
    let chance = match mode {
        Mode::Advantage => 1.0 - (1.0 - one) * (1.0 - one),
        Mode::Disadvantage => one * one,
        Mode::Normal | Mode::Critical => one,
    };
    (chance * 100.0).round() as i64
}

impl Run {
    /// A tile or a number in a participant's stat block or sheet. An action arms; a check,
    /// a save, a skill or plain dice rolls at once and shows on the actor's row.
    pub(crate) fn roll_link(&mut self, ask: Ask) {
        let Some(actor) = self.existing(ask.roller) else {
            return;
        };
        let inspired = ask.inspired && self.can_buy_advantage(actor);
        let result = match &ask.link {
            RollLink::D20 {
                what: D20::Attack, ..
            }
            | RollLink::SaveDc { .. }
            | RollLink::Damage { .. } => {
                self.arm(actor, ask.link, ask.mode, inspired);
                Ok(())
            }
            RollLink::D20 { .. } | RollLink::Dice { .. } => {
                self.roll_alone(actor, &ask.link, ask.mode, inspired)
            }
        };
        if let Err(error) = result {
            self.error = Some(error);
        }
    }

    /// The row, if it is still in the run.
    fn existing(&self, row: Option<usize>) -> Option<usize> {
        row.filter(|row| *row < self.state.participants.len())
    }

    /// Arm an action. Its rows' results from before go.
    fn arm(&mut self, actor: usize, link: RollLink, mode: Mode, inspired: bool) {
        self.badges.clear();
        self.saves.clear();
        self.asking = None;
        self.reroll = None;
        if self.selected != actor {
            self.select(actor);
        }
        self.armed = Some(Armed {
            actor,
            link,
            mode,
            inspired,
        });
    }

    /// Stop the armed action. Its rows' results stay until the next action or turn.
    pub(crate) fn disarm(&mut self) {
        self.armed = None;
        self.asking = None;
    }

    /// A check, a save, a skill or dice: rolled now, shown on the actor's row.
    fn roll_alone(
        &mut self,
        actor: usize,
        link: &RollLink,
        mode: Mode,
        inspired: bool,
    ) -> Result<(), String> {
        let who = self.state.participants[actor].name.clone();
        let mode = if inspired { Mode::Advantage } else { mode };
        let outcome = self.record(
            |random| rolls::make(&who, link, mode, (0, 0), None, random),
            |outcome: &Outcome| spent_lines(&who, inspired, outcome.lines()),
        )?;
        if inspired {
            self.spend_inspiration(actor);
        }
        if let Some(first) = &outcome.first {
            self.badges.insert(actor, first.headline());
        }
        self.keep_for_reroll(actor, actor, outcome, None);
        Ok(())
    }

    /// True when this row's character has inspiration they can expend now.
    pub(super) fn has_inspiration(&self, row: usize) -> bool {
        if self.inspiration.is_none()
            || !self.player(row)
            || self
                .spent_inspiration
                .contains(&self.state.participants[row].key)
        {
            return false;
        }
        match self
            .syncs
            .get(&row)
            .and_then(|sync| sync.values().get("inspiration"))
        {
            Some(group) => group["inspiration"] == true,
            None => self.sheet_inspiration.get(&row) == Some(&true),
        }
    }

    /// What a read of a character's sheet says about their inspiration. A sheet that shows
    /// none has caught up with what the run spent.
    pub(super) fn note_sheet_inspiration(&mut self, row: usize, has: bool) {
        self.sheet_inspiration.insert(row, has);
        if !has
            && self
                .spent_inspiration
                .remove(&self.state.participants[row].key)
        {
            self.dirty = true;
        }
    }

    /// Send what the run spent to each synced sheet that can take it now. The run remembers
    /// a spend, and saves it with the run, until D&D Beyond confirms that the sheet shows
    /// none: a value that waits to be sent, or cannot be sent, is not a confirmation.
    pub(super) fn settle_spent_inspiration(&mut self) {
        if self.spent_inspiration.is_empty() {
            return;
        }
        for (row, sync) in &mut self.syncs {
            let Some(p) = self.state.participants.get(*row) else {
                continue;
            };
            if !self.spent_inspiration.contains(&p.key) {
                continue;
            }
            let has = |groups: &BTreeMap<String, serde_json::Value>| {
                groups
                    .get("inspiration")
                    .map(|group| group["inspiration"] == true)
            };
            if sync.confirmed().and_then(has) == Some(false) {
                self.spent_inspiration.remove(&p.key);
                self.dirty = true;
            } else if sync.ready() && has(sync.values()) == Some(true) {
                sync.set_group("inspiration", serde_json::json!({"inspiration": false}));
            }
        }
    }

    /// True when this row's character can expend inspiration for advantage before a roll:
    /// only by a rule that says so, and only if they have it.
    fn can_buy_advantage(&self, row: usize) -> bool {
        self.inspiration == Some(Inspiration::Advantage) && self.has_inspiration(row)
    }

    /// The character expends inspiration. The run remembers it until their sheet shows none:
    /// a synced sheet is sent it now, or when its sync next can take it.
    fn spend_inspiration(&mut self, row: usize) {
        self.spent_inspiration
            .insert(self.state.participants[row].key.clone());
        self.dirty = true;
        self.settle_spent_inspiration();
    }

    /// Keep a roll made for a player for a reroll, by a rule that rerolls after the roll.
    fn keep_for_reroll(
        &mut self,
        actor: usize,
        shown: usize,
        outcome: Outcome,
        save: Option<Armed>,
    ) {
        self.reroll = (self.inspiration == Some(Inspiration::Reroll)
            && self.has_inspiration(actor))
        .then_some(Reroll {
            actor,
            shown,
            outcome,
            save,
        });
    }

    /// Reroll one die of the kept roll with inspiration. The new die stands.
    fn reroll_with_inspiration(&mut self, die: rolls::Die) -> Result<(), String> {
        let Some(kept) = self.reroll.take() else {
            return Ok(());
        };
        if !self.has_inspiration(kept.actor) {
            return Ok(());
        }
        let who = self.state.participants[kept.actor].name.clone();
        let changes = self.changes(kept.actor);
        let (outcome, ..) = self.record(
            |random| rolls::reroll(&who, &kept.outcome, die, changes, random),
            |(outcome, old, new): &(Outcome, i64, i64)| {
                let mut lines = vec![format!(
                    "{who} expends inspiration to reroll a die: {old} becomes {new}."
                )];
                // Only what the new die changed is said again.
                let changed = |new: &Option<Rolled>, old: &Option<Rolled>| {
                    new.as_ref().filter(|_| new != old).map(Rolled::line)
                };
                lines.extend(changed(&outcome.first, &kept.outcome.first));
                lines.extend(changed(&outcome.damage, &kept.outcome.damage));
                lines
            },
        )?;
        self.spend_inspiration(kept.actor);
        if let Some(save) = &kept.save {
            self.show_save_damage(save, &outcome);
        } else if kept.shown != kept.actor {
            self.show_attack(kept.shown, &outcome);
        } else if let Some(first) = &outcome.first {
            self.badges.insert(kept.shown, first.headline());
        }
        Ok(())
    }

    /// A run's change to the actor's attack and damage, from its encounter or group.
    fn changes(&self, actor: usize) -> (i64, i64) {
        self.state.participants[actor]
            .changes
            .as_ref()
            .map_or((0, 0), |changes| {
                (
                    changes.overrides.attack_bonus.unwrap_or(0),
                    changes.overrides.damage_bonus.unwrap_or(0),
                )
            })
    }

    /// Roll on the run's dice, and write the Log lines in every Undo step too, so Undo keeps
    /// them and the dice never repeat.
    pub(super) fn record<T>(
        &mut self,
        roll: impl FnOnce(&mut Random) -> Result<T, String>,
        lines: impl Fn(&T) -> Vec<String>,
    ) -> Result<T, String> {
        if self.ended() {
            return Err(ENDED.into());
        }
        let mut random = self.state.random.clone();
        let made = roll(&mut random)?;
        let lines = lines(&made);
        for state in self.history.iter_mut().chain([&mut self.state]) {
            state.log.extend(lines.iter().cloned());
            state.random = random.clone();
        }
        self.dm_steps += 1;
        self.dirty = true;
        self.error = None;
        Ok(made)
    }

    /// The panel's participant: the armed actor while an action is armed.
    pub(super) fn panel_row(&self) -> usize {
        self.armed
            .as_ref()
            .map(|armed| armed.actor)
            .filter(|row| *row < self.state.participants.len())
            .unwrap_or(self.selected)
    }

    /// True for a party member, whose player rolls their own dice.
    fn player(&self, row: usize) -> bool {
        let p = &self.state.participants[row];
        p.side == "party" || p.definition.fields.contains_key("splat_external")
    }

    /// A click on a row while an action is armed: the row becomes its target.
    /// Returns false when nothing is armed, so the click selects as usual.
    pub(super) fn target(&mut self, row: usize, mode: Option<Mode>) -> bool {
        let Some(armed) = self.armed.clone() else {
            return false;
        };
        if row == armed.actor {
            return true;
        }
        self.asking = None;
        let mode = mode.unwrap_or(armed.mode);
        let result = match &armed.link {
            RollLink::SaveDc { ability, .. } => {
                self.add_save_target(row, ability, mode);
                Ok(())
            }
            _ if self.player(armed.actor) => {
                // The player says the numbers; the DM types the damage.
                self.aim(row);
                self.badges
                    .insert(row, "Player rolls · type the damage".to_owned());
                Ok(())
            }
            link => self.attack(armed.actor, row, link, mode, false),
        };
        if let Err(error) = result {
            self.error = Some(error);
        }
        true
    }

    /// What a row's right-click menu chose.
    pub(super) fn row_choice(&mut self, row: usize, choice: RowChoice) {
        let result = match choice {
            RowChoice::Cancel => {
                self.asking = None;
                Ok(())
            }
            RowChoice::Reroll(die) => self.reroll_with_inspiration(die),
            RowChoice::Damage(index) => match self.asking.take() {
                Some(asking) if asking.row == row => self.answer(asking, index),
                _ => Ok(()),
            },
            RowChoice::Roll(mode, inspired) => {
                let Some(armed) = self.armed.clone() else {
                    return;
                };
                self.asking = None;
                match &armed.link {
                    RollLink::SaveDc { ability, .. } => {
                        self.add_save_target(row, ability, mode);
                        Ok(())
                    }
                    // For a player, the menu rolls for them.
                    link => {
                        let inspired = inspired && self.can_buy_advantage(armed.actor);
                        self.attack(armed.actor, row, link, mode, inspired)
                    }
                }
            }
        };
        if let Err(error) = result {
            self.error = Some(error);
        }
    }

    /// The right-click menu of a run row, and the question that waits on it. Shows nothing
    /// when the row has nothing to offer.
    pub(super) fn row_menu(&self, response: &egui::Response, row: usize) -> Option<RowChoice> {
        let mut chosen = None;
        if let Some(asking) = self.asking.as_ref().filter(|asking| asking.row == row) {
            let shown = egui::Popup::menu(response).open(true).show(|ui| {
                ui.set_width(320.0);
                let c = crate::design::colors();
                ui.add(
                    egui::Label::new(
                        crate::design::Type::Small
                            .rich(ui, &asking.question)
                            .color(c.text),
                    )
                    .wrap(),
                );
                for (index, (words, _)) in asking.choices.iter().enumerate() {
                    if rolls::menu_item(ui, words) {
                        chosen = Some(RowChoice::Damage(index));
                    }
                }
                if rolls::menu_item(ui, "Cancel") {
                    chosen = Some(RowChoice::Cancel);
                }
            });
            if chosen.is_none() && shown.is_some_and(|shown| shown.response.clicked_elsewhere()) {
                chosen = Some(RowChoice::Cancel);
            }
            return chosen;
        }
        let sections = self.row_menu_sections(row);
        if sections.is_empty() {
            return None;
        }
        // Under the row, so the row it is about stays in view.
        let open = if response.secondary_clicked() {
            Some(egui::SetOpenCommand::Bool(true))
        } else if response.clicked() {
            Some(egui::SetOpenCommand::Bool(false))
        } else {
            None
        };
        egui::Popup::menu(response).open_memory(open).show(|ui| {
            for (heading, items) in &sections {
                ui.label(
                    crate::design::Type::Small
                        .rich(ui, heading)
                        .color(crate::design::colors().muted),
                );
                for (words, choice) in items {
                    if rolls::menu_item(ui, words) {
                        chosen = Some(choice.clone());
                        ui.close();
                    }
                }
            }
        });
        chosen
    }

    /// What a row's right-click menu offers now: each section is a heading and its choices.
    fn row_menu_sections(&self, row: usize) -> Vec<(String, Vec<(String, RowChoice)>)> {
        let mut sections: Vec<(String, Vec<(String, RowChoice)>)> = Vec::new();
        if let Some(armed) = self.armed.as_ref().filter(|armed| armed.actor != row) {
            let name = &self.state.participants[row].name;
            let heading = match &armed.link {
                RollLink::SaveDc { label, ability, .. } => {
                    format!("{name}'s {ability} save against {label}")
                }
                link => format!("{} against {name}", rolls::link_label(link)),
            };
            let mut items = Vec::new();
            match &armed.link {
                RollLink::SaveDc { .. } if !self.player(row) => {
                    for (words, mode, _) in rolls::mode_choices("Roll its save", false) {
                        items.push((words, RowChoice::Roll(mode, false)));
                    }
                }
                RollLink::D20 { .. } => {
                    let verb = if self.player(armed.actor) {
                        "Roll for them"
                    } else {
                        "Roll"
                    };
                    let inspired = self.can_buy_advantage(armed.actor);
                    for (words, mode, inspired) in rolls::mode_choices(verb, inspired) {
                        items.push((words, RowChoice::Roll(mode, inspired)));
                    }
                }
                _ => {}
            }
            if !items.is_empty() {
                sections.push((heading, items));
            }
        }
        // A save's damage is one roll for all of its targets, so each of their rows offers it.
        if let Some(kept) = self.reroll.as_ref().filter(|kept| {
            (kept.shown == row || (kept.save.is_some() && self.save_targets().contains(&row)))
                && self.has_inspiration(kept.actor)
        }) {
            sections.push((
                "Inspiration: reroll one die".to_owned(),
                kept.outcome
                    .dice()
                    .into_iter()
                    .take(REROLL_CHOICES)
                    .map(|(words, die)| (words, RowChoice::Reroll(die)))
                    .collect(),
            ));
        }
        sections
    }

    /// Whether a damage condition holds, when the run can tell: the roll's mode, or whether
    /// the target or the actor is Bloodied. `None` when only the DM knows.
    fn condition_holds(&self, when: &When, mode: Mode, actor: usize, row: usize) -> Option<bool> {
        if let Some(holds) = rolls::condition_by_mode(when, mode) {
            return Some(holds);
        }
        let words = when.text.to_lowercase();
        if words.ends_with("is bloodied") {
            let who = if words.contains("the target") {
                row
            } else {
                actor
            };
            let p = &self.state.participants[who];
            return match crate::design::hp_state(p.hp, p.max_hp) {
                crate::design::HpState::Unknown => None,
                state => Some(matches!(
                    state,
                    crate::design::HpState::Bloodied | crate::design::HpState::Zero
                )),
            };
        }
        // `if the target has more than 20 Hit Points`, from a save's `Otherwise`.
        if let Some(limit) = words
            .strip_prefix("if the target has more than ")
            .and_then(|rest| rest.strip_suffix(" hit points"))
            .and_then(|number| number.parse::<i64>().ok())
        {
            return self.state.participants[row].hp.map(|hp| hp > limit);
        }
        None
    }

    /// Whether a condition holds for every one of `rows`. `None` when the run cannot tell for
    /// one of them, or when they differ: one roll of damage cannot be both.
    fn condition_holds_for(
        &self,
        when: &When,
        mode: Mode,
        actor: usize,
        rows: &[usize],
    ) -> Option<bool> {
        let mut answers = rows
            .iter()
            .map(|row| self.condition_holds(when, mode, actor, *row));
        let first = answers.next()??;
        answers.all(|answer| answer == Some(first)).then_some(first)
    }

    /// The question a target waits on: one choice without the unknown conditions, then one
    /// with each. `None` when the run can tell every condition, with the damage to roll.
    fn damage_for(
        &self,
        link: &RollLink,
        mode: Mode,
        actor: usize,
        rows: &[usize],
    ) -> Result<Vec<DamagePart>, Question> {
        let parts = damage_of(link);
        let unknown: Vec<&When> = dice_text::conditions(parts)
            .into_iter()
            .filter(|when| self.condition_holds_for(when, mode, actor, rows).is_none())
            .collect();
        let settle = |yes: Option<&When>| {
            dice_text::resolve(parts, |when| {
                self.condition_holds_for(when, mode, actor, rows)
                    .unwrap_or(Some(when) == yes)
            })
        };
        if unknown.is_empty() {
            return Ok(settle(None));
        }
        // One condition is a yes or no; several are lettered in the question.
        let sentence = |when: &When| {
            let mut words = when.text.clone();
            if let Some(first) = words.get_mut(..1) {
                first.make_ascii_uppercase();
            }
            words
        };
        let without = settle(None);
        if let [when] = unknown.as_slice() {
            let with = settle(Some(when));
            return Err((
                format!("{}?", sentence(when)),
                vec![
                    (format!("Yes · {}", parts_words(&with)), with),
                    (format!("No · {}", parts_words(&without)), without),
                ],
            ));
        }
        let mut question = vec!["Which applies?".to_owned()];
        let mut choices = Vec::new();
        for (at, when) in unknown.iter().enumerate() {
            let letter = char::from(b'A' + at as u8);
            question.push(format!("{letter}: {}", sentence(when)));
            let parts = settle(Some(when));
            choices.push((format!("{letter} · {}", parts_words(&parts)), parts));
        }
        choices.push((format!("None · {}", parts_words(&without)), without));
        Err((question.join("\n"), choices))
    }

    /// Roll the asking target with the chosen damage.
    fn answer(&mut self, asking: Asking, index: usize) -> Result<(), String> {
        let Some(armed) = self.armed.clone() else {
            return Ok(());
        };
        let Some((_, parts)) = asking.choices.get(index) else {
            return Ok(());
        };
        if matches!(armed.link, RollLink::SaveDc { .. }) {
            return self.roll_save_damage(&armed, parts);
        }
        let link = with_damage(&armed.link, parts.clone());
        self.roll_attack(armed.actor, asking.row, &link, asking.mode, asking.inspired)
    }

    /// Make `row` the dock's only target, keeping the panel on the actor.
    fn aim(&mut self, row: usize) {
        self.selected = row;
        self.also.clear();
        self.halved.clear();
        self.amount.clear();
        self.focus_amount = true;
    }

    /// Roll an attack, or damage alone, against `row`. Its damage goes into Amount for it.
    /// A damage condition the run cannot tell waits for the DM's choice first.
    fn attack(
        &mut self,
        actor: usize,
        row: usize,
        link: &RollLink,
        mode: Mode,
        inspired: bool,
    ) -> Result<(), String> {
        let mode = if inspired { Mode::Advantage } else { mode };
        match self.damage_for(link, mode, actor, &[row]) {
            Ok(parts) => self.roll_attack(actor, row, &with_damage(link, parts), mode, inspired),
            Err((question, choices)) => {
                self.asking = Some(Asking {
                    row,
                    mode,
                    inspired,
                    question,
                    choices,
                });
                Ok(())
            }
        }
    }

    /// Roll an attack whose damage is settled against `row`.
    fn roll_attack(
        &mut self,
        actor: usize,
        row: usize,
        link: &RollLink,
        mode: Mode,
        inspired: bool,
    ) -> Result<(), String> {
        let who = self.state.participants[actor].name.clone();
        let changes = self.changes(actor);
        let outcome = self.record(
            |random| rolls::make(&who, link, mode, changes, None, random),
            |outcome: &Outcome| spent_lines(&who, inspired, outcome.lines()),
        )?;
        if inspired {
            self.spend_inspiration(actor);
        }
        self.show_attack(row, &outcome);
        self.keep_for_reroll(actor, row, outcome, None);
        Ok(())
    }

    /// An attack's result on its target's row, and its damage in Amount.
    fn show_attack(&mut self, row: usize, outcome: &Outcome) {
        self.aim(row);
        let armor = self.state.participants[row].armor_class;
        let mut words = Vec::new();
        if let Some(first) = &outcome.first {
            let natural = first
                .natural_words()
                .map_or_else(String::new, |words| format!("{words} · "));
            words.push(match armor {
                Some(armor) => format!("{natural}{} vs AC {armor}", first.total),
                None => format!("{natural}{} to hit", first.total),
            });
        }
        if let Some(damage) = &outcome.damage {
            words.push(format!("{} damage", damage.total));
            self.amount = damage.total.max(0).to_string();
        }
        self.badges.insert(row, words.join(" · "));
    }

    /// Add `row` to a save's targets, and roll its save if it is a creature.
    fn add_save_target(&mut self, row: usize, ability: &str, mode: Mode) {
        let actor = self.armed.as_ref().map(|armed| armed.actor);
        if Some(self.selected) == actor || self.selected == row {
            self.aim(row);
        } else if !self.also.remove(&row) {
            self.also.insert(row);
        } else {
            self.halved.remove(&row);
            self.saves.remove(&row);
            return;
        }
        if self.player(row) {
            return;
        }
        let p = &self.state.participants[row];
        let Some(bonus) = save_bonus(&p.definition, ability) else {
            return;
        };
        let save = RollLink::d20(D20::Save, bonus, ability);
        let name = p.name.clone();
        match self.record(
            |random| rolls::roll(&name, &save, mode, 0, random),
            |rolled: &Rolled| vec![rolled.line()],
        ) {
            Ok(rolled) => {
                self.saves.insert(row, rolled.total);
            }
            Err(error) => self.error = Some(error),
        }
    }

    /// What the line under the armed tile offers: the action's state, and its next steps.
    fn armed_view(&self) -> Option<ArmedView> {
        let armed = self.armed.as_ref()?;
        let (status, mut actions) = match &armed.link {
            RollLink::SaveDc { half, damage, .. } => {
                let targets = self.save_targets().len();
                let status = match targets {
                    0 => "Click each target. Escape stops.".to_owned(),
                    1 => "1 target. Click more, then roll the damage.".to_owned(),
                    n => format!("{n} targets. Click more, then roll the damage."),
                };
                let mut actions = Vec::new();
                if targets > 0 && !damage.is_empty() {
                    actions.push(("Roll damage", ArmedAction::RollDamage));
                }
                if *half && !self.saves.is_empty() && targets > 1 {
                    actions.push(("Half for those at or above DC", ArmedAction::HalfMade));
                }
                (status, actions)
            }
            _ if self.player(armed.actor) => {
                let mut actions = Vec::new();
                if self.selected != armed.actor {
                    actions.push(("Roll for them", ArmedAction::RollForThem));
                }
                (
                    "Click a target, then type the player's damage. Escape stops.".to_owned(),
                    actions,
                )
            }
            _ => (
                "Click a target to roll against it. Enter applies the damage. Escape stops."
                    .to_owned(),
                Vec::new(),
            ),
        };
        actions.push(("Stop", ArmedAction::Stop));
        Some(ArmedView {
            link: armed.link.clone(),
            status,
            actions,
        })
    }

    /// The rows a save targets: the dock's targets, without the caster.
    fn save_targets(&self) -> Vec<usize> {
        let actor = self.armed.as_ref().map(|armed| armed.actor);
        self.targets()
            .into_iter()
            .filter(|row| Some(*row) != actor)
            .collect()
    }

    /// Publish the armed action for the tiles drawn next, and whether the panel's character
    /// can expend inspiration for advantage.
    pub(super) fn armed_tiles_begin(&self, ctx: &egui::Context) {
        rolls::set_armed(ctx, self.armed_view());
        rolls::set_inspired(ctx, self.can_buy_advantage(self.panel_row()));
    }

    /// Handle what was pressed under the armed tile.
    pub(super) fn armed_tiles_end(&mut self, ctx: &egui::Context) {
        rolls::set_inspired(ctx, false);
        let Some(action) = rolls::take_armed_action(ctx) else {
            return;
        };
        let Some(armed) = self.armed.clone() else {
            return;
        };
        let result = match action {
            ArmedAction::Stop => {
                self.disarm();
                Ok(())
            }
            ArmedAction::RollForThem => {
                let row = self.selected;
                let inspired = armed.inspired && self.has_inspiration(armed.actor);
                self.attack(armed.actor, row, &armed.link, armed.mode, inspired)
            }
            ArmedAction::RollDamage => self.save_damage(&armed),
            ArmedAction::HalfMade => {
                self.half_made(&armed);
                Ok(())
            }
        };
        if let Err(error) = result {
            self.error = Some(error);
        }
    }

    /// A save's damage, rolled once for every target, into Amount. Halves follow the marks.
    /// A damage condition is settled first: by the targets, when the run can tell and they
    /// agree, or else by a question under the first target's row.
    fn save_damage(&mut self, armed: &Armed) -> Result<(), String> {
        let targets = self.save_targets();
        match self.damage_for(&armed.link, Mode::Normal, armed.actor, &targets) {
            Ok(parts) => self.roll_save_damage(armed, &parts),
            Err((question, choices)) => {
                self.asking = targets.first().map(|row| Asking {
                    row: *row,
                    mode: Mode::Normal,
                    inspired: false,
                    question,
                    choices,
                });
                Ok(())
            }
        }
    }

    /// Roll a save's settled damage once, into Amount.
    fn roll_save_damage(&mut self, armed: &Armed, parts: &[DamagePart]) -> Result<(), String> {
        let RollLink::SaveDc { label, .. } = &armed.link else {
            return Ok(());
        };
        if parts.is_empty() {
            // Its condition gives these targets another effect, in the rules text. An amount
            // from before is not this save's damage, so Enter must not apply it.
            self.amount.clear();
            self.reroll = None;
            for row in self.save_targets() {
                self.badges
                    .insert(row, "No damage · see the text".to_owned());
            }
            return Ok(());
        }
        let link = RollLink::Damage {
            label: label.clone(),
            parts: parts.to_vec(),
        };
        let who = self.state.participants[armed.actor].name.clone();
        let changes = self.changes(armed.actor);
        let outcome = self.record(
            |random| rolls::make(&who, &link, Mode::Normal, changes, None, random),
            Outcome::lines,
        )?;
        self.show_save_damage(armed, &outcome);
        let shown = self.selected;
        self.keep_for_reroll(armed.actor, shown, outcome, Some(armed.clone()));
        Ok(())
    }

    /// A save's damage in Amount and on each target's row. The targets and their half marks
    /// stay as they are.
    fn show_save_damage(&mut self, armed: &Armed, outcome: &Outcome) {
        let (RollLink::SaveDc { dc, half, .. }, Some(rolled)) = (&armed.link, &outcome.damage)
        else {
            return;
        };
        let total = rolled.total.max(0);
        // One target has no half mark, so its half goes straight into Amount.
        let lone_half = *half
            && self.also.is_empty()
            && self
                .saves
                .get(&self.selected)
                .is_some_and(|save| save >= dc);
        self.amount = if lone_half { total / 2 } else { total }.to_string();
        self.focus_amount = true;
        for row in self.save_targets() {
            self.badges.insert(row, format!("{total} damage"));
        }
    }

    /// Mark the save targets at or above the DC to take half.
    fn half_made(&mut self, armed: &Armed) {
        let RollLink::SaveDc { dc, .. } = armed.link else {
            return;
        };
        for row in self.save_targets() {
            if self.saves.get(&row).is_some_and(|total| *total >= dc) {
                self.halved.insert(row);
            }
        }
    }

    /// What a row shows while an action is armed or after it: the chance to hit, or the
    /// result against it.
    pub(super) fn row_badge(&self, row: usize) -> Option<String> {
        if let Some(words) = self.badges.get(&row) {
            return Some(words.clone());
        }
        let armed = self.armed.as_ref()?;
        let RollLink::D20 {
            what: D20::Attack,
            bonus,
            ..
        } = armed.link
        else {
            return None;
        };
        let p = &self.state.participants[row];
        if row == armed.actor || !p.present || self.player(armed.actor) {
            return None;
        }
        let bonus = bonus + self.changes(armed.actor).0;
        Some(format!(
            "{}% to hit",
            hit_chance(bonus, p.armor_class?, armed.mode)
        ))
    }

    /// A save result in place of AC on a row, such as `DEX` and 19.
    pub(super) fn save_on_row(&self, row: usize) -> Option<(String, i64)> {
        let total = *self.saves.get(&row)?;
        let Some(RollLink::SaveDc { ability, .. }) = self.armed.as_ref().map(|armed| &armed.link)
        else {
            return None;
        };
        Some((
            ability.chars().take(3).collect::<String>().to_uppercase(),
            total,
        ))
    }

    /// The actions of a row: from its linked sheet, or else from its stat block's text.
    pub(super) fn chips(&self, row: usize) -> Vec<Chip> {
        if let Some(record) = self.sheet.as_ref().filter(|_| self.sheet_for == Some(row)) {
            return crate::party::sheet_chips(record);
        }
        rolls::chips(&self.state.participants[row].definition.body)
    }

    /// The room above the dock for the action bar: only while the panel is hidden.
    pub(super) fn strip_height(&self) -> f32 {
        if self.hide_panel { STRIP_HEIGHT } else { 0.0 }
    }

    /// The panel's action tiles, as one line above the dock while the panel is hidden.
    pub(super) fn compact_actions(&mut self, ui: &mut egui::Ui) {
        let row = self.panel_row();
        let chips = self.chips(row);
        rolls::set_roller(ui.ctx(), Some(row));
        rolls::set_entry(ui.ctx(), &self.state.participants[row].name.clone());
        self.armed_tiles_begin(ui.ctx());
        rolls::compact_tiles(ui, &chips);
        rolls::set_roller(ui.ctx(), None);
        rolls::set_inspired(ui.ctx(), false);
    }

    /// Ctrl+1 to Ctrl+9 arm the panel's participant's actions, in tile order.
    pub(super) fn arm_by_number(&mut self, number: usize) {
        let row = self.panel_row();
        if let Some(chip) = self.chips(row).into_iter().nth(number.saturating_sub(1)) {
            self.arm(row, chip.link, Mode::Normal, false);
        }
    }

    /// Escape stops an armed action first.
    pub(super) fn disarm_on_escape(&mut self, ui: &egui::Ui) -> bool {
        // A question waiting on a target closes first.
        if self.asking.is_some()
            && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        {
            self.asking = None;
            return true;
        }
        if self.armed.is_some()
            && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        {
            self.disarm();
            return true;
        }
        false
    }

    /// Next turn ends the armed action and its rows' results.
    pub(super) fn end_turn_rolls(&mut self) {
        self.armed = None;
        self.asking = None;
        self.reroll = None;
        self.badges.clear();
        self.saves.clear();
    }

    /// Undo can take away rows. An action whose actor is gone ends.
    pub(super) fn forget_removed_rollers(&mut self) {
        let rows = self.state.participants.len();
        if self.armed.as_ref().is_some_and(|armed| armed.actor >= rows) {
            self.armed = None;
        }
        if self
            .asking
            .as_ref()
            .is_some_and(|asking| asking.row >= rows)
        {
            self.asking = None;
        }
        if self
            .reroll
            .as_ref()
            .is_some_and(|kept| kept.actor >= rows || kept.shown >= rows)
        {
            self.reroll = None;
        }
        self.sheet_inspiration.retain(|row, _| *row < rows);
        self.badges.retain(|row, _| *row < rows);
        self.saves.retain(|row, _| *row < rows);
    }
}

/// A roll's Log lines, after a line that says the roller expended inspiration for it.
fn spent_lines(who: &str, inspired: bool, lines: Vec<String>) -> Vec<String> {
    if !inspired {
        return lines;
    }
    let mut all = vec![format!("{who} expends inspiration for advantage.")];
    all.extend(lines);
    all
}

#[cfg(test)]
mod tests {
    use super::super::Command;
    use super::*;
    use splat::content::dice_text::DamagePart;

    fn live() -> Run {
        let mut run = super::super::tests::run();
        for (index, total) in [20, 15, 10].into_iter().enumerate() {
            run.apply(Command::Initiative(index, Some(total))).unwrap();
        }
        run.apply(Command::Begin).unwrap();
        run
    }

    fn attack() -> RollLink {
        RollLink::D20 {
            what: D20::Attack,
            bonus: 4,
            label: "Spear".into(),
            damage: vec![DamagePart {
                dice: "2d6+2".into(),
                kind: Some("Piercing".into()),
                when: None,
            }],
        }
    }

    fn ask(link: RollLink, roller: usize) -> Ask {
        Ask {
            link,
            mode: Mode::Normal,
            roller: Some(roller),
            entry: "Guard".into(),
            inspired: false,
        }
    }

    /// Row `row` as a party member whose synced sheet says they have inspiration.
    fn inspired_player(run: &mut Run, row: usize) {
        run.state.participants[row].side = "party".into();
        inspired_sync(run, row);
        assert!(run.inspiration.is_none() || run.has_inspiration(row));
    }

    /// A sync for `row` whose read says the character has inspiration.
    fn inspired_sync(run: &mut Run, row: usize) {
        use crate::combat_sync::{CombatSync, SyncState};
        let mut sync = CombatSync::simulated_state(SyncState::Matching, 7, 30, 0);
        sync.simulate_sheet(
            7,
            30,
            &serde_json::json!({"id":7,"canEdit":true,"removedHitPoints":0,"temporaryHitPoints":0,"deathSaves":{"successCount":0,"failCount":0},"conditions":[],"inspiration":true}),
        );
        run.syncs.insert(row, sync);
    }

    #[test]
    fn spent_inspiration_stays_with_the_run_until_the_sheet_shows_none() {
        let mut run = live();
        run.inspiration = Some(Inspiration::Reroll);
        run.state.participants[0].side = "party".into();
        // No sync: the last read of the sheet says they have it.
        run.note_sheet_inspiration(0, true);
        assert!(run.has_inspiration(0));
        run.spend_inspiration(0);
        assert!(!run.has_inspiration(0));
        let key = run.state.participants[0].key.clone();
        assert!(run.spent_inspiration.contains(&key), "saved with the run");
        // A read that still says they have it is older than the spend.
        run.note_sheet_inspiration(0, true);
        assert!(!run.has_inspiration(0));
        // The sheet caught up; a later grant counts again.
        run.note_sheet_inspiration(0, false);
        assert!(run.spent_inspiration.is_empty());
        run.note_sheet_inspiration(0, true);
        assert!(run.has_inspiration(0));
    }

    #[test]
    fn a_sync_that_starts_later_is_sent_the_spent_inspiration() {
        let mut run = live();
        run.inspiration = Some(Inspiration::Reroll);
        run.state.participants[0].side = "party".into();
        run.note_sheet_inspiration(0, true);
        run.spend_inspiration(0);
        // The sync's first read still shows inspiration on the sheet.
        inspired_sync(&mut run, 0);
        assert!(
            !run.has_inspiration(0),
            "the run's spend wins over the read"
        );
        run.settle_spent_inspiration();
        let sync = &run.syncs[&0];
        assert_eq!(sync.values()["inspiration"]["inspiration"], false);
        assert!(sync.dirty(), "the change waits to be sent");
        // A value that waits to be sent is not a confirmation: a run saved and reopened now
        // must still know the spend, or the old sheet would give the inspiration back.
        run.settle_spent_inspiration();
        run.settle_spent_inspiration();
        assert_eq!(run.spent_inspiration.len(), 1, "kept until confirmed");
        assert!(!run.has_inspiration(0));
        // D&D Beyond confirms that the sheet shows none.
        run.syncs.get_mut(&0).unwrap().simulate_sheet(
            7,
            30,
            &serde_json::json!({"id":7,"canEdit":true,"removedHitPoints":0,"temporaryHitPoints":0,"deathSaves":{"successCount":0,"failCount":0},"conditions":[],"inspiration":false}),
        );
        run.settle_spent_inspiration();
        assert!(run.spent_inspiration.is_empty(), "the sheet has it now");
    }

    fn log_has(run: &Run, words: &str) -> bool {
        run.state.log.iter().any(|line| line.contains(words))
    }

    fn check() -> RollLink {
        RollLink::d20(D20::Check, 2, "Strength")
    }

    #[test]
    fn without_an_inspiration_rule_nobody_has_inspiration() {
        let mut run = live();
        inspired_player(&mut run, 0);
        assert!(!run.has_inspiration(0));
        run.roll_link(Ask {
            inspired: true,
            ..ask(check(), 0)
        });
        assert!(!log_has(&run, "inspiration"));
    }

    #[test]
    fn a_rule_of_advantage_spends_inspiration_before_the_roll() {
        let mut run = live();
        run.inspiration = Some(Inspiration::Advantage);
        inspired_player(&mut run, 0);
        assert!(run.has_inspiration(0));
        run.roll_link(Ask {
            inspired: true,
            ..ask(check(), 0)
        });
        assert!(log_has(&run, "expends inspiration for advantage"));
        assert!(log_has(&run, "2d20kh1"), "{:?}", run.state.log);
        assert!(!run.has_inspiration(0), "it is spent");
        assert!(run.reroll.is_none(), "this rule has no reroll");
    }

    #[test]
    fn a_rule_of_reroll_rerolls_the_d20_after_the_roll() {
        let mut run = live();
        run.inspiration = Some(Inspiration::Reroll);
        inspired_player(&mut run, 0);
        // Asking for it before the roll does nothing under this rule.
        run.state.random = Random::controlled([3, 17]);
        run.roll_link(Ask {
            inspired: true,
            ..ask(check(), 0)
        });
        assert!(!log_has(&run, "expends"));
        assert_eq!(
            run.badges.get(&0).map(String::as_str),
            Some("5 Strength check")
        );
        let kept = run.reroll.clone().unwrap();
        assert_eq!(kept.shown, 0);
        let dice = kept.outcome.dice();
        assert_eq!(dice.len(), 1);
        assert_eq!(dice[0].0, "3 on the d20");
        run.row_choice(0, RowChoice::Reroll(dice[0].1));
        assert!(log_has(
            &run,
            "expends inspiration to reroll a die: 3 becomes 17."
        ));
        assert_eq!(
            run.badges.get(&0).map(String::as_str),
            Some("19 Strength check")
        );
        assert!(!run.has_inspiration(0), "it is spent");
        assert!(run.reroll.is_none());
    }

    #[test]
    fn a_rule_of_reroll_rerolls_any_die_of_a_roll_made_for_a_player() {
        let mut run = live();
        run.inspiration = Some(Inspiration::Reroll);
        let actor = run.current().unwrap();
        let target = (0..3).find(|row| *row != actor).unwrap();
        inspired_player(&mut run, actor);
        run.roll_link(ask(attack(), actor));
        // A player's tile only targets; `Roll for them` rolls: d20 11, damage 1 and 5.
        run.target(target, None);
        assert!(run.reroll.is_none());
        run.state.random = Random::controlled([11, 1, 5, 6]);
        run.row_choice(target, RowChoice::Roll(Mode::Normal, false));
        assert_eq!(run.amount, "8");
        let words: Vec<String> = run
            .reroll
            .as_ref()
            .unwrap()
            .outcome
            .dice()
            .into_iter()
            .map(|(words, _)| words)
            .collect();
        assert_eq!(
            words,
            [
                "11 on the d20",
                "1 on 2d6+2 piercing",
                "5 on 2d6+2 piercing"
            ]
        );
        let lines = run.state.log.len();
        let die = run.reroll.as_ref().unwrap().outcome.dice()[1].1;
        run.row_choice(target, RowChoice::Reroll(die));
        assert!(log_has(&run, "reroll a die: 1 becomes 6."));
        assert_eq!(
            run.state.log.len(),
            lines + 2,
            "only the damage is said again"
        );
        assert_eq!(run.amount, "13", "the new damage is in Amount");
        assert!(run.row_badge(target).unwrap().ends_with("13 damage"));
        assert!(!run.has_inspiration(actor));
    }

    #[test]
    fn applied_damage_ends_the_chance_to_reroll() {
        let mut run = live();
        run.inspiration = Some(Inspiration::Reroll);
        let actor = run.current().unwrap();
        let target = (0..3).find(|row| *row != actor).unwrap();
        inspired_player(&mut run, actor);
        run.roll_link(ask(attack(), actor));
        run.target(target, None);
        run.row_choice(target, RowChoice::Roll(Mode::Normal, false));
        assert!(run.reroll.is_some());
        // Enter applies the amount. A reroll after it would put damage in Amount again.
        let amount = run.amount.parse().unwrap();
        run.apply(Command::Damage(target, amount)).unwrap();
        assert!(run.reroll.is_none());
        assert!(
            run.row_menu_sections(target)
                .iter()
                .all(|(heading, _)| { !heading.starts_with("Inspiration") })
        );
    }

    #[test]
    fn a_players_save_damage_can_reroll_a_die_for_all_of_its_targets() {
        let mut run = live();
        run.inspiration = Some(Inspiration::Reroll);
        let actor = run.current().unwrap();
        let others: Vec<usize> = (0..3).filter(|row| *row != actor).collect();
        inspired_player(&mut run, actor);
        let save = RollLink::SaveDc {
            label: "Burning Hands".into(),
            ability: "Dexterity".into(),
            dc: 14,
            half: true,
            damage: vec![DamagePart {
                dice: "3d6".into(),
                kind: Some("Fire".into()),
                when: None,
            }],
        };
        run.roll_link(ask(save, actor));
        run.target(others[0], None);
        run.target(others[1], None);
        run.saves.insert(others[0], 15);
        run.saves.insert(others[1], 9);
        let armed = run.armed.clone().unwrap();
        run.half_made(&armed);
        run.state.random = Random::controlled([1, 2, 3, 6]);
        run.save_damage(&armed).unwrap();
        assert_eq!(run.amount, "6");
        // Each target's row offers the one roll's dice.
        for row in &others {
            let sections = run.row_menu_sections(*row);
            let (_, items) = sections
                .iter()
                .find(|(heading, _)| heading.starts_with("Inspiration"))
                .expect("a reroll section");
            assert_eq!(items[0].0, "1 on 3d6 fire");
        }
        let die = run.reroll.as_ref().unwrap().outcome.dice()[0].1;
        run.row_choice(others[1], RowChoice::Reroll(die));
        assert_eq!(run.amount, "11");
        for row in &others {
            assert_eq!(run.badges.get(row).map(String::as_str), Some("11 damage"));
        }
        assert_eq!(
            run.halved.iter().copied().collect::<Vec<_>>(),
            vec![others[0]],
            "the half marks stay"
        );
        let mut targets = run.save_targets();
        targets.sort_unstable();
        assert_eq!(targets, others, "the targets stay");
        assert!(!run.has_inspiration(actor));
    }

    fn bloodied_bite() -> RollLink {
        let text = "Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Piercing damage, or 6 (1d8 + 2) Piercing damage if the target is Bloodied.";
        dice_text::find(text, "Bite")
            .into_iter()
            .find_map(|span| matches!(span.link, RollLink::D20 { .. }).then_some(span.link))
            .unwrap()
    }

    #[test]
    fn a_condition_the_run_can_tell_picks_the_damage() {
        let mut run = live();
        let actor = run.current().unwrap();
        let target = (0..3).find(|row| *row != actor).unwrap();
        run.roll_link(ask(bloodied_bite(), actor));
        run.target(target, None);
        assert!(log_has(&run, "1d4"), "{:?}", run.state.log);
        let max = run.state.participants[target].max_hp.unwrap();
        run.state.participants[target].hp = Some(max / 2);
        run.target(target, None);
        assert!(log_has(&run, "1d8"), "{:?}", run.state.log);
        assert!(run.asking.is_none());
    }

    #[test]
    fn a_condition_only_the_dm_knows_asks_before_the_roll() {
        let mut run = live();
        let actor = run.current().unwrap();
        let target = (0..3).find(|row| *row != actor).unwrap();
        let text = "Melee Attack Roll: +4, reach 5 ft. Hit: 9 (2d6 + 2) Bludgeoning damage, or 11 (2d8 + 2) Bludgeoning damage if the goat moved 20+ feet straight toward the target immediately before the hit.";
        let link = dice_text::find(text, "Ram")
            .into_iter()
            .find_map(|span| matches!(span.link, RollLink::D20 { .. }).then_some(span.link))
            .unwrap();
        run.roll_link(ask(link, actor));
        let lines = run.state.log.len();
        run.target(target, None);
        assert_eq!(
            run.state.log.len(),
            lines,
            "nothing rolls before the answer"
        );
        let asking = run.asking.clone().unwrap();
        assert_eq!(
            asking.question,
            "If the goat moved 20+ feet straight toward the target immediately before the hit?"
        );
        assert_eq!(asking.choices[0].0, "Yes · 2d8+2 bludgeoning");
        assert_eq!(asking.choices[1].0, "No · 2d6+2 bludgeoning");
        run.row_choice(target, RowChoice::Damage(0));
        assert!(log_has(&run, "2d8"), "{:?}", run.state.log);
        assert!(run.asking.is_none());
        assert!(!run.amount.is_empty());
    }

    #[test]
    fn the_chance_to_hit_counts_natural_ones_and_twenties_and_advantage() {
        assert_eq!(hit_chance(4, 13, Mode::Normal), 60);
        assert_eq!(hit_chance(4, 30, Mode::Normal), 5);
        assert_eq!(hit_chance(14, 10, Mode::Normal), 95);
        assert_eq!(hit_chance(4, 13, Mode::Advantage), 84);
        assert_eq!(hit_chance(4, 13, Mode::Disadvantage), 36);
    }

    #[test]
    fn an_armed_attack_rolls_against_each_clicked_row_and_its_damage_fills_amount() {
        let mut run = live();
        let actor = run.current().unwrap();
        run.roll_link(ask(attack(), actor));
        assert_eq!(run.panel_row(), actor);
        let target = (0..3).find(|row| *row != actor).unwrap();
        assert!(run.row_badge(target).unwrap().ends_with("% to hit"));
        run.state.random = Random::controlled([20, 3, 4, 5, 6]);
        let steps = run.history.len();
        assert!(run.target(target, None));
        assert_eq!(run.history.len(), steps, "a roll is not an Undo step");
        assert_eq!(run.selected, target, "the target takes the dock's amount");
        assert_eq!(run.panel_row(), actor, "the panel stays on the attacker");
        // A natural 20 doubles the dice and adds the bonus once.
        assert_eq!(run.amount, "20");
        assert!(
            run.row_badge(target)
                .unwrap()
                .starts_with("Natural 20 · 24 vs AC")
        );
        assert!(
            run.armed.is_some(),
            "the action stays armed for the next attack"
        );
        run.disarm();
        assert_eq!(run.panel_row(), target);
        run.apply(Command::Next).unwrap();
        assert!(run.badges.is_empty() && run.armed.is_none());
    }

    #[test]
    fn a_save_rolls_each_targets_save_and_its_damage_once() {
        let mut run = live();
        let actor = run.current().unwrap();
        let save = RollLink::SaveDc {
            label: "Fire Breath".into(),
            ability: "Dexterity".into(),
            dc: 14,
            half: true,
            damage: vec![DamagePart {
                dice: "4d6".into(),
                kind: Some("Fire".into()),
                when: None,
            }],
        };
        run.roll_link(ask(save, actor));
        let others: Vec<usize> = (0..3).filter(|row| *row != actor).collect();
        run.target(others[0], None);
        run.target(others[1], None);
        let mut targets = run.save_targets();
        targets.sort_unstable();
        assert_eq!(targets, others);
        run.saves.insert(others[0], 15);
        run.saves.insert(others[1], 9);
        let armed = run.armed.clone().unwrap();
        run.half_made(&armed);
        assert_eq!(
            run.halved.iter().copied().collect::<Vec<_>>(),
            vec![others[0]]
        );
        run.state.random = Random::controlled([1, 2, 3, 4]);
        run.save_damage(&armed).unwrap();
        assert_eq!(run.amount, "10");
    }

    #[test]
    fn a_saves_damage_follows_its_condition_or_asks_when_the_targets_differ() {
        let glare = "Death Glare. Wisdom Saving Throw: DC 11, one Frightened creature. Failure: If the target has 20 Hit Points or fewer, it drops to 0 Hit Points. Otherwise, the target takes 13 (3d8) Psychic damage.";
        let link = dice_text::find(glare, "Death Glare")
            .into_iter()
            .find_map(|span| matches!(span.link, RollLink::SaveDc { .. }).then_some(span.link))
            .unwrap();
        let mut run = live();
        let actor = run.current().unwrap();
        let others: Vec<usize> = (0..3).filter(|row| *row != actor).collect();
        run.roll_link(ask(link, actor));
        let armed = run.armed.clone().unwrap();
        // One target above the limit: the damage rolls.
        run.state.participants[others[0]].hp = Some(30);
        run.target(others[0], None);
        run.state.random = Random::controlled([2, 3, 4]);
        run.save_damage(&armed).unwrap();
        assert_eq!(run.amount, "9");
        // At the limit or under it: no damage rolls, and the row says so.
        run.state.participants[others[0]].hp = Some(20);
        let lines = run.state.log.len();
        run.save_damage(&armed).unwrap();
        assert_eq!(run.state.log.len(), lines);
        assert_eq!(run.amount, "", "the 9 from before must not wait for Enter");
        assert_eq!(
            run.badges.get(&others[0]).map(String::as_str),
            Some("No damage · see the text")
        );
        // Two targets that differ: one roll cannot be both, so the DM says.
        run.state.participants[others[1]].hp = Some(30);
        run.target(others[1], None);
        run.save_damage(&armed).unwrap();
        let asking = run.asking.clone().unwrap();
        assert_eq!(
            asking.question,
            "If the target has more than 20 Hit Points?"
        );
        assert_eq!(asking.choices[0].0, "Yes · 3d8 psychic");
        assert_eq!(asking.choices[1].0, "No · no dice");
        run.state.random = Random::controlled([1, 1, 1]);
        run.row_choice(asking.row, RowChoice::Damage(0));
        assert_eq!(run.amount, "3");
        assert!(run.asking.is_none());
    }

    #[test]
    fn a_check_rolls_at_once_onto_the_actors_row_and_a_save_falls_back_to_the_modifier() {
        let mut run = live();
        let actor = run.current().unwrap();
        run.state.random = Random::controlled([12]);
        run.roll_link(ask(RollLink::d20(D20::Check, 2, "Dexterity"), actor));
        assert_eq!(
            run.badges.get(&actor).map(String::as_str),
            Some("14 Dexterity check")
        );
        assert!(run.armed.is_none());
        let source =
            splat::content::definitions::source_file("definitions/creatures/aboleth.md").unwrap();
        let entry = Entry::parse("Aboleth.md", source).unwrap();
        assert!(save_bonus(&entry, "Dexterity").is_some());
    }
}
