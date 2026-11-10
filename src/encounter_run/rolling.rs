//! Rolls from dice links during a run. A roll changes no participant: it goes to the Log, and
//! Undo keeps it. A roll from a stat block holds the panel on its roller while its result is
//! open, so a click on a target's row chooses the target without hiding the attack's text.
//! A save DC starts a call: rows chosen on the roster save, and their results show on them.
use super::{Phase, Run};
use crate::design::{self, Type, colors};
use crate::rolls::{self, Ask, Mode, Rolled, Shown};
use eframe::egui;
use splat::content::Entry;
use splat::content::dice_text::{D20, DamagePart, RollLink};
use splat::execution::dice::Random;
use std::collections::{BTreeMap, BTreeSet};

/// The last roll of this turn. It belongs to its roller, not to the selected row.
#[derive(Clone, Debug)]
pub(super) struct LastRoll {
    /// The row that rolled. A roll from Lookup or the dice field has none.
    pub roller: Option<usize>,
    pub rolled: Rolled,
}

/// A call for a save. Its targets are the run's own selection; its results are by row.
#[derive(Clone, Debug)]
pub(super) struct Call {
    pub caster: Option<usize>,
    pub who: String,
    pub label: String,
    pub ability: String,
    pub dc: i64,
    /// True when the feature's text says a success takes half damage.
    pub half: bool,
    pub damage: Vec<DamagePart>,
    /// Each rolled save, by row.
    pub totals: BTreeMap<usize, i64>,
    /// Rows that take half. One target has no run half mark, so the call keeps them too.
    pub halves: BTreeSet<usize>,
}

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

impl Run {
    /// Make the roll a dice link asked for. A save DC starts a call for the save instead.
    /// Returns the result to show beside the link.
    pub(crate) fn roll_link(&mut self, ask: Ask) -> Result<Option<Shown>, String> {
        let result = self.try_roll_link(ask);
        if let Err(error) = &result {
            self.error = Some(error.clone());
        }
        result
    }

    fn try_roll_link(&mut self, ask: Ask) -> Result<Option<Shown>, String> {
        if self.ended() {
            return Err(ENDED.into());
        }
        let live = matches!(self.state.phase, Phase::Running);
        let roller = self.existing(ask.roller);
        let who = match roller {
            Some(row) => self.state.participants[row].name.clone(),
            None => ask.entry,
        };
        if let RollLink::SaveDc {
            label,
            ability,
            dc,
            half,
            damage,
        } = ask.link
        {
            // Targets and their results live on the roster, which setup does not have yet.
            if !live {
                return Ok(None);
            }
            self.call = Some(Call {
                caster: roller,
                who,
                label,
                ability,
                dc,
                half,
                damage,
                totals: BTreeMap::new(),
                halves: BTreeSet::new(),
            });
            self.also.clear();
            self.halved.clear();
            if let Some(row) = roller {
                self.select(row);
            }
            self.held = roller;
            return Ok(self.call_shown(ask.at));
        }
        let change = self.change(roller, &ask.link);
        let rolled =
            self.record(|random| rolls::roll(&who, &ask.link, ask.mode, change, random))?;
        Ok(Some(self.after_roll(roller, rolled, ask.at)))
    }

    /// The roller's row, if it is still in the run.
    fn existing(&self, roller: Option<usize>) -> Option<usize> {
        roller.filter(|row| *row < self.state.participants.len())
    }

    /// A run change to the roller's attack or damage, from its encounter or group.
    fn change(&self, roller: Option<usize>, link: &RollLink) -> i64 {
        let Some(overrides) = roller
            .and_then(|row| self.state.participants[row].changes.as_ref())
            .map(|changes| &changes.overrides)
        else {
            return 0;
        };
        match link {
            RollLink::D20 {
                what: D20::Attack, ..
            } => overrides.attack_bonus.unwrap_or(0),
            RollLink::Damage { .. } => overrides.damage_bonus.unwrap_or(0),
            _ => 0,
        }
    }

    /// Roll on the run's dice, and write the Log line in every Undo step too, so Undo keeps
    /// it and the dice never repeat.
    pub(super) fn record(
        &mut self,
        roll: impl FnOnce(&mut Random) -> Result<Rolled, String>,
    ) -> Result<Rolled, String> {
        if self.ended() {
            return Err(ENDED.into());
        }
        let mut random = self.state.random.clone();
        let rolled = roll(&mut random)?;
        let line = rolled.line();
        for state in self.history.iter_mut().chain([&mut self.state]) {
            state.log.push(line.clone());
            state.random = random.clone();
        }
        self.dm_steps += 1;
        self.dirty = true;
        self.error = None;
        Ok(rolled)
    }

    /// Keep the roll, hold the panel on its roller, and put damage in Amount when it has a
    /// target that is not the roller.
    fn after_roll(&mut self, roller: Option<usize>, rolled: Rolled, at: egui::Pos2) -> Shown {
        let live = matches!(self.state.phase, Phase::Running);
        if live {
            self.last = Some(LastRoll {
                roller,
                rolled: rolled.clone(),
            });
            if roller.is_some() {
                self.held = roller;
            }
            if rolled.feature_damage() && self.use_blocked(roller).is_none() {
                self.use_last_total();
            }
            // The run's keys work again, so D and Enter can follow.
            self.follow_with_keyboard();
        }
        Shown {
            armor: self.armor_words(roller, &rolled),
            held: live && roller.is_some(),
            rolled: Some(rolled),
            call: None,
            at,
        }
    }

    /// The selected row's AC beside an attack, when the selected row is not the roller.
    fn armor_words(&self, roller: Option<usize>, rolled: &Rolled) -> Option<String> {
        if !rolled.attack() || roller == Some(self.selected) {
            return None;
        }
        let p = self.state.participants.get(self.selected)?;
        Some(format!("AC {} {}", p.armor_class?, p.name))
    }

    /// Damage must not land on the creature that dealt it by mistake.
    fn use_blocked(&self, roller: Option<usize>) -> Option<&'static str> {
        (roller == Some(self.selected) && self.also.is_empty()).then_some("Choose a target first.")
    }

    /// The panel's participant: the roller while its roll is open, or else the selected row.
    pub(super) fn panel_row(&self) -> usize {
        match self.held {
            Some(row) if row < self.state.participants.len() => row,
            _ => self.selected,
        }
    }

    /// True while a roll or a call holds the panel.
    pub(crate) fn holding(&self) -> bool {
        self.held.is_some() || self.call.is_some()
    }

    /// The result beside the link has closed: the panel follows the selection again.
    pub(crate) fn release(&mut self) {
        self.held = None;
        self.call = None;
    }

    /// `Roll damage` after an attack, or the call's damage. A natural 20 rolls it as a
    /// critical hit. Damage with a target goes into Amount.
    pub(super) fn roll_damage(&mut self, at: Option<egui::Pos2>) -> Option<Shown> {
        let (link, mode, roller, who) = if let Some(call) = &self.call {
            (
                RollLink::Damage {
                    label: call.label.clone(),
                    parts: call.damage.clone(),
                },
                Mode::Normal,
                call.caster,
                call.who.clone(),
            )
        } else {
            let last = self.last.as_ref()?;
            let attack = &last.rolled;
            if !attack.attack() {
                return None;
            }
            let mode = if attack.natural == Some(20) {
                Mode::Critical
            } else {
                Mode::Normal
            };
            (
                RollLink::Damage {
                    label: attack.feature().unwrap_or_default().to_owned(),
                    parts: attack.damage().to_vec(),
                },
                mode,
                last.roller,
                attack.who.clone(),
            )
        };
        if matches!(&link, RollLink::Damage { parts, .. } if parts.is_empty()) {
            return None;
        }
        let roller = self.existing(roller);
        let change = self.change(roller, &link);
        match self.record(|random| rolls::roll(&who, &link, mode, change, random)) {
            Ok(rolled) => {
                let shown = self.after_roll(roller, rolled, at.unwrap_or_default());
                at.map(|_| shown)
            }
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    /// `Use`: put the last total in Amount, for the selected targets. In a call, one target
    /// that takes half takes the halved number.
    pub(super) fn use_last_total(&mut self) {
        let Some(last) = &self.last else {
            return;
        };
        if !last.rolled.usable()
            || (last.rolled.feature_damage() && self.use_blocked(last.roller).is_some())
        {
            return;
        }
        let total = last.rolled.total.max(0);
        let half = self.also.is_empty()
            && self
                .call
                .as_ref()
                .is_some_and(|call| call.halves.contains(&self.selected));
        self.amount = if half { total / 2 } else { total }.to_string();
        self.focus_amount = true;
    }

    /// What a result beside a link asked for.
    pub(crate) fn rolled_action(&mut self, action: rolls::Action, at: egui::Pos2) -> Option<Shown> {
        match action {
            rolls::Action::RollDamage => self.roll_damage(Some(at)),
            rolls::Action::RollSaves => {
                self.roll_saves();
                self.call_shown(at)
            }
            rolls::Action::HalfMade => {
                self.half_made();
                self.call_shown(at)
            }
        }
    }

    /// The result beside its link again, as the selection and the call change under it.
    pub(crate) fn refreshed(&self, shown: &Shown) -> Shown {
        if shown.call.is_some() {
            return self.call_shown(shown.at).unwrap_or_else(|| shown.clone());
        }
        let roller = self.last.as_ref().and_then(|last| last.roller);
        // A held result follows the run's last roll, such as damage rolled with D.
        let rolled = match &self.last {
            Some(last) if shown.held => Some(last.rolled.clone()),
            _ => shown.rolled.clone(),
        };
        Shown {
            armor: rolled
                .as_ref()
                .and_then(|rolled| self.armor_words(roller, rolled)),
            rolled,
            ..shown.clone()
        }
    }

    /// The call's card beside its DC: how many targets, and what can follow.
    fn call_shown(&self, at: egui::Pos2) -> Option<Shown> {
        let call = self.call.as_ref()?;
        let targets = self.call_targets();
        Some(Shown {
            rolled: None,
            call: Some(rolls::CallCard {
                who: call.who.clone(),
                label: call.label.clone(),
                ability: call.ability.clone(),
                dc: call.dc,
                targets: targets.len(),
                can_roll: !self.unrolled_creatures().is_empty(),
                can_half: call.half
                    && targets
                        .iter()
                        .any(|row| call.totals.get(row).is_some_and(|total| *total >= call.dc)),
                damage: !call.damage.is_empty(),
            }),
            armor: None,
            held: true,
            at,
        })
    }

    /// The call's targets: the run's selection, without the caster.
    fn call_targets(&self) -> Vec<usize> {
        let caster = self.call.as_ref().and_then(|call| call.caster);
        self.targets()
            .into_iter()
            .filter(|row| Some(*row) != caster)
            .collect()
    }

    /// Targets with a save bonus and no save yet. A linked sheet's player rolls their own.
    fn unrolled_creatures(&self) -> Vec<usize> {
        let Some(call) = &self.call else {
            return Vec::new();
        };
        self.call_targets()
            .into_iter()
            .filter(|row| {
                let definition = &self.state.participants[*row].definition;
                !call.totals.contains_key(row)
                    && !definition.fields.contains_key("splat_external")
                    && save_bonus(definition, &call.ability).is_some()
            })
            .collect()
    }

    /// `Roll saves`: each creature target rolls its save, on the run's dice.
    fn roll_saves(&mut self) {
        let Some(ability) = self.call.as_ref().map(|call| call.ability.clone()) else {
            return;
        };
        for row in self.unrolled_creatures() {
            let p = &self.state.participants[row];
            let Some(bonus) = save_bonus(&p.definition, &ability) else {
                continue;
            };
            let link = RollLink::d20(D20::Save, bonus, &ability);
            let name = p.name.clone();
            match self.record(|random| rolls::roll(&name, &link, Mode::Normal, 0, random)) {
                Ok(rolled) => {
                    if let Some(call) = &mut self.call {
                        call.totals.insert(row, rolled.total);
                    }
                }
                Err(error) => self.error = Some(error),
            }
        }
    }

    /// `Half for those at or above DC`: their rows take half of the damage that follows.
    fn half_made(&mut self) {
        let targets = self.call_targets();
        let Some(call) = &mut self.call else {
            return;
        };
        for row in targets {
            if call.totals.get(&row).is_some_and(|total| *total >= call.dc) {
                call.halves.insert(row);
                if !self.also.is_empty() {
                    self.halved.insert(row);
                }
            }
        }
    }

    /// A click on a roster row. During a call it chooses the call's targets; the caster
    /// stays out. Otherwise it selects the row, and a held panel stays on its roller.
    pub(super) fn row_click(&mut self, row: usize) {
        if let Some(call) = &self.call {
            if Some(row) == call.caster {
                return;
            }
            if Some(self.selected) == call.caster {
                self.select(row);
            } else {
                self.toggle_target(row);
            }
            return;
        }
        if self.held == Some(row) {
            self.held = None;
        }
        self.select(row);
    }

    /// A save result in place of AC on a row, such as `DEX` and 19, while its call is open.
    pub(super) fn save_on_row(&self, row: usize) -> Option<(String, i64)> {
        let call = self.call.as_ref()?;
        let total = *call.totals.get(&row)?;
        Some((
            call.ability
                .chars()
                .take(3)
                .collect::<String>()
                .to_uppercase(),
            total,
        ))
    }

    /// Under the panel's title while it is held: why, and a way to see the selected row.
    /// Returns true when the DM asked to see it.
    pub(super) fn held_line(&self, ui: &mut egui::Ui) -> bool {
        let Some(held) = self.held.filter(|_| self.panel_row() != self.selected) else {
            return false;
        };
        let held = &self.state.participants[held].name;
        let selected = &self.state.participants[self.selected].name;
        let mut show = false;
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = design::XS;
            ui.label(
                Type::Small
                    .rich(ui, format!("Held on {held} while its roll is open ·"))
                    .color(colors().muted),
            );
            let words = format!("Show {selected}");
            let response = design::small_link(ui, &words);
            design::named(ui, &response, &words);
            show = response
                .on_hover_text("Close the roll and show the selected row. Escape also does it.")
                .clicked();
        });
        show
    }

    /// Damage was applied from the dock: a call is done, and the panel follows again.
    pub(super) fn after_amount(&mut self) {
        if self.call.is_some() {
            self.release();
        }
    }

    /// Next turn ends this turn's roll, its hold and its call.
    pub(super) fn end_turn_rolls(&mut self) {
        self.last = None;
        self.release();
    }

    /// Undo can take away rows. A roll keeps its result, but not a roller that is gone.
    pub(super) fn forget_removed_rollers(&mut self) {
        let rows = self.state.participants.len();
        if let Some(last) = &mut self.last
            && last.roller.is_some_and(|row| row >= rows)
        {
            last.roller = None;
        }
        if self.held.is_some_and(|row| row >= rows) {
            self.held = None;
        }
        if let Some(call) = &mut self.call {
            call.totals.retain(|row, _| *row < rows);
            call.halves.retain(|row| *row < rows);
            if call.caster.is_some_and(|row| row >= rows) {
                call.caster = None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encounter_run::Command;

    fn live() -> Run {
        let mut run = super::super::tests::run();
        for (index, total) in [20, 15, 10].into_iter().enumerate() {
            run.apply(Command::Initiative(index, Some(total))).unwrap();
        }
        run.apply(Command::Begin).unwrap();
        run
    }

    fn ask(link: RollLink, roller: Option<usize>) -> Ask {
        Ask {
            link,
            mode: Mode::Normal,
            at: egui::Pos2::ZERO,
            roller,
            entry: "Guard".into(),
        }
    }

    fn attack() -> RollLink {
        RollLink::D20 {
            what: D20::Attack,
            bonus: 4,
            label: "Spear".into(),
            damage: vec![DamagePart {
                dice: "2d6+2".into(),
                kind: Some("Piercing".into()),
            }],
        }
    }

    #[test]
    fn a_held_panel_keeps_the_attacker_while_a_target_is_chosen_and_damage_fills_amount() {
        let mut run = live();
        let roller = run.current().unwrap();
        run.state.random = Random::controlled([20, 3, 4, 5, 6]);
        let steps = run.history.len();
        let shown = run.roll_link(ask(attack(), Some(roller))).unwrap().unwrap();
        let rolled = shown.rolled.unwrap();
        assert_eq!(rolled.total, 24);
        assert_eq!(rolled.natural_words(), Some("Natural 20"));
        assert!(
            shown.held && shown.armor.is_none(),
            "the roller is still selected"
        );
        assert_eq!(run.history.len(), steps, "a roll is not an Undo step");
        assert!(
            run.state
                .log
                .last()
                .unwrap()
                .contains("Spear: to hit 1d20 + 4")
        );
        let target = (0..3).find(|row| *row != roller).unwrap();
        run.row_click(target);
        assert_eq!(run.selected, target);
        assert_eq!(run.panel_row(), roller, "the panel stays on the attacker");
        let last = run.last.clone().unwrap();
        let p = &run.state.participants[target];
        assert_eq!(
            run.armor_words(last.roller, &last.rolled),
            Some(format!("AC {} {}", p.armor_class.unwrap(), p.name))
        );
        // A natural 20 doubles the dice and adds the bonus once; the total fills Amount.
        run.roll_damage(None);
        assert_eq!(run.last.as_ref().unwrap().rolled.total, 3 + 4 + 5 + 6 + 2);
        assert_eq!(run.amount, "20");
        assert!(run.focus_amount);
        run.release();
        assert_eq!(
            run.panel_row(),
            target,
            "closing the result shows the selection"
        );
    }

    #[test]
    fn damage_waits_for_a_target_undo_keeps_the_roll_and_next_turn_ends_it() {
        let mut run = live();
        let roller = run.current().unwrap();
        run.state.random = Random::controlled([2, 3]);
        let damage = RollLink::Damage {
            label: "Spear".into(),
            parts: vec![DamagePart {
                dice: "1d6+1".into(),
                kind: None,
            }],
        };
        run.roll_link(ask(damage, Some(roller))).unwrap();
        assert!(
            run.amount.is_empty(),
            "no damage lands on its roller by mistake"
        );
        run.use_last_total();
        assert!(run.amount.is_empty());
        // Plain dice can be used on anyone.
        run.roll_dice("1d4").unwrap();
        run.use_last_total();
        assert_eq!(run.amount, "3");
        run.amount.clear();
        run.apply(Command::Damage(roller, 1)).unwrap();
        run.undo();
        assert!(run.last.is_some(), "Undo keeps the roll");
        run.apply(Command::Next).unwrap();
        assert!(run.last.is_none() && !run.holding(), "Next turn ends it");
    }

    #[test]
    fn a_call_chooses_rows_on_the_roster_and_halves_those_at_or_above_the_dc() {
        let mut run = live();
        let caster = run.current().unwrap();
        let save = RollLink::SaveDc {
            label: "Fire Breath".into(),
            ability: "Dexterity".into(),
            dc: 14,
            half: true,
            damage: vec![DamagePart {
                dice: "4d6".into(),
                kind: Some("Fire".into()),
            }],
        };
        let shown = run.roll_link(ask(save, Some(caster))).unwrap().unwrap();
        assert!(shown.call.is_some() && run.holding());
        let others: Vec<usize> = (0..3).filter(|row| *row != caster).collect();
        run.row_click(caster);
        assert_eq!(run.selected, caster, "the caster is never a target");
        run.row_click(others[0]);
        run.row_click(others[1]);
        let mut targets = run.call_targets();
        targets.sort_unstable();
        assert_eq!(targets, others);
        assert_eq!(run.panel_row(), caster, "the panel stays on the caster");
        // The fixture has no ability scores, so the DM records the saves on the call.
        let call = run.call.as_mut().unwrap();
        call.totals.insert(others[0], 15);
        call.totals.insert(others[1], 9);
        assert_eq!(run.save_on_row(others[0]), Some(("DEX".into(), 15)));
        run.half_made();
        assert_eq!(
            run.halved.iter().copied().collect::<Vec<_>>(),
            vec![others[0]]
        );
        run.state.random = Random::controlled([1, 2, 3, 4]);
        run.roll_damage(None);
        assert_eq!(run.amount, "10", "the damage fills Amount for both targets");
        run.after_amount();
        assert!(!run.holding(), "applied damage ends the call");
    }

    #[test]
    fn a_save_falls_back_to_the_modifier() {
        let source =
            splat::content::definitions::source_file("definitions/creatures/aboleth.md").unwrap();
        let entry = Entry::parse("Aboleth.md", source).unwrap();
        assert!(save_bonus(&entry, "Dexterity").is_some());
    }
}
