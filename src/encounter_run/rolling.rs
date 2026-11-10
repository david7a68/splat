//! Rolls during a run. The selected row has an action strip under it: a chip for each of its
//! actions, a bonus field, and the last result. An attack rolls its damage with it; a save
//! rolls its damage and every creature's save, shown where each row's AC is. Damage waits
//! until the DM clicks a target's HP, which applies it as one step of Undo. A roll changes
//! no participant by itself: it goes to the Log, and Undo keeps it.
use super::{Command, Run};
use crate::rolls::{self, Ask, Chip, LineEvent, LineResult, Mode, Outcome, Rolled};
use eframe::egui;
use splat::content::Entry;
use splat::content::dice_text::{D20, RollLink};
use splat::execution::dice::Random;
use std::collections::{BTreeMap, BTreeSet};

/// The selected row's last action, and where its damage has gone.
#[derive(Clone, Debug)]
pub(super) struct Turn {
    pub actor: usize,
    pub outcome: Outcome,
    /// For a save: whether a HP click applies half.
    pub half: bool,
    /// Rows that took this damage.
    pub applied: BTreeSet<usize>,
    /// For a save: each creature's total, by row.
    pub saves: BTreeMap<usize, i64>,
}

impl Turn {
    /// The damage a HP click applies, full or half, never below 0.
    fn amount(&self) -> Option<i64> {
        let total = self.outcome.damage.as_ref()?.total.max(0);
        Some(if self.half { total / 2 } else { total })
    }

    /// An attack's damage lands once; a save's lands once per target.
    fn landed(&self, row: usize) -> bool {
        if self.outcome.many_targets() {
            self.applied.contains(&row)
        } else {
            !self.applied.is_empty()
        }
    }
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
    /// Make the roll a number in a participant's stat block or sheet asked for.
    pub(crate) fn roll_link(&mut self, ask: Ask) {
        let Some(actor) = self.existing(ask.roller) else {
            return;
        };
        if let Err(error) = self.roll_action(actor, &ask.link, ask.mode) {
            self.error = Some(error);
        }
    }

    fn roll_action(&mut self, actor: usize, link: &RollLink, mode: Mode) -> Result<(), String> {
        if self.ended() {
            return Err(ENDED.into());
        }
        let who = self.state.participants[actor].name.clone();
        let changes = self.changes(actor);
        let extra = self
            .bonuses
            .get(&actor)
            .and_then(|text| rolls::bonus_dice(text).ok().flatten());
        let outcome = self.record(
            |random| rolls::make(&who, link, mode, changes, extra.as_deref(), random),
            Outcome::lines,
        )?;
        let mut saves = BTreeMap::new();
        if let RollLink::SaveDc { ability, .. } = link {
            for row in self.state.order.clone() {
                let p = &self.state.participants[row];
                if row == actor || !p.present || p.definition.fields.contains_key("splat_external")
                {
                    continue;
                }
                let Some(bonus) = save_bonus(&p.definition, ability) else {
                    continue;
                };
                let save = RollLink::d20(D20::Save, bonus, ability);
                let name = p.name.clone();
                let rolled = self.record(
                    |random| rolls::roll(&name, &save, Mode::Normal, 0, random),
                    |rolled: &Rolled| vec![rolled.line()],
                )?;
                saves.insert(row, rolled.total);
            }
        }
        self.turn = Some(Turn {
            actor,
            outcome,
            half: false,
            applied: BTreeSet::new(),
            saves,
        });
        Ok(())
    }

    /// The row, if it is still in the run.
    fn existing(&self, row: Option<usize>) -> Option<usize> {
        row.filter(|row| *row < self.state.participants.len())
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

    /// The damage a click on a row's HP applies now, if any: none after an attack's damage
    /// has landed, and none on the actor itself.
    pub(super) fn waiting_damage(&self, row: usize) -> Option<i64> {
        let turn = self.turn.as_ref()?;
        let amount = turn.amount()?;
        if row == turn.actor || turn.landed(row) || !self.state.participants.get(row)?.present {
            return None;
        }
        Some(amount)
    }

    /// A click on a row's HP while damage waits: apply it there, as one step of Undo.
    pub(super) fn apply_waiting(&mut self, row: usize) {
        let Some(amount) = self.waiting_damage(row) else {
            return;
        };
        self.run_command(Command::Damage(row, amount));
        if let Some(turn) = &mut self.turn {
            turn.applied.insert(row);
        }
    }

    /// A save result in place of AC on a row, such as `DEX` and 19.
    pub(super) fn save_on_row(&self, row: usize) -> Option<(String, i64)> {
        let turn = self.turn.as_ref()?;
        let RollLink::SaveDc { ability, .. } = &turn.outcome.link else {
            return None;
        };
        let total = *turn.saves.get(&row)?;
        Some((
            ability.chars().take(3).collect::<String>().to_uppercase(),
            total,
        ))
    }

    /// The actions of a row: from its linked sheet, or else from its stat block's text.
    fn chips(&self, row: usize) -> Vec<Chip> {
        if let Some(record) = self.sheet.as_ref().filter(|_| self.sheet_for == Some(row)) {
            return crate::party::sheet_chips(record);
        }
        rolls::chips(&self.state.participants[row].definition.body)
    }

    /// The strip under the selected row, in live play.
    pub(super) fn action_strip(&mut self, ui: &mut egui::Ui, row: usize) {
        let chips = self.chips(row);
        let turn = self.turn.as_ref().filter(|turn| turn.actor == row);
        let apply = turn.and_then(|turn| {
            let amount = turn.amount()?;
            Some(
                if !turn.outcome.many_targets() && !turn.applied.is_empty() {
                    "Applied.".to_owned()
                } else {
                    format!("Click a row's HP to apply {amount}.")
                },
            )
        });
        let result = LineResult {
            outcome: turn.map(|turn| &turn.outcome),
            half: turn
                .filter(|turn| turn.outcome.many_targets() && turn.outcome.damage.is_some())
                .map(|turn| turn.half),
            apply,
        };
        let mut bonus = self.bonuses.get(&row).cloned().unwrap_or_default();
        let event = rolls::action_line(ui, ("run", row), &chips, &result, &mut bonus);
        if bonus.is_empty() {
            self.bonuses.remove(&row);
        } else {
            self.bonuses.insert(row, bonus);
        }
        let last = turn.map(|turn| turn.outcome.link.clone());
        let result = match event {
            Some(LineEvent::Roll(link, mode)) => self.roll_action(row, &link, mode),
            Some(LineEvent::Again(mode)) => match last {
                Some(link) => self.roll_action(row, &link, mode),
                None => Ok(()),
            },
            Some(LineEvent::Half(half)) => {
                if let Some(turn) = &mut self.turn {
                    turn.half = half;
                }
                Ok(())
            }
            None => Ok(()),
        };
        if let Err(error) = result {
            self.error = Some(error);
        }
    }

    /// The selection moved: the action belongs to the row it was rolled for.
    pub(super) fn forget_turn_unless(&mut self, row: usize) {
        if self.turn.as_ref().is_some_and(|turn| turn.actor != row) {
            self.turn = None;
        }
    }

    /// Next turn ends this turn's action.
    pub(super) fn end_turn_rolls(&mut self) {
        self.turn = None;
    }

    /// Undo can take away rows. An action whose actor is gone ends.
    pub(super) fn forget_removed_rollers(&mut self) {
        let rows = self.state.participants.len();
        if self.turn.as_ref().is_some_and(|turn| turn.actor >= rows) {
            self.turn = None;
        }
        if let Some(turn) = &mut self.turn {
            turn.applied.retain(|row| *row < rows);
            turn.saves.retain(|row, _| *row < rows);
        }
        self.bonuses.retain(|row, _| *row < rows);
    }

    /// The hover words for a row's HP while damage waits.
    pub(super) fn apply_words(&self, row: usize) -> Option<String> {
        let amount = self.waiting_damage(row)?;
        Some(format!(
            "Apply {amount} damage to {}",
            self.state.participants[row].name
        ))
    }
}

/// The strip's height above the dock, which the roster's room leaves for it.
pub(super) const STRIP_HEIGHT: f32 = rolls::LINE_HEIGHT;

#[cfg(test)]
mod tests {
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
            }],
        }
    }

    #[test]
    fn an_attack_rolls_its_damage_and_a_click_on_a_targets_hp_applies_it_once() {
        let mut run = live();
        let actor = run.current().unwrap();
        run.state.random = Random::controlled([20, 3, 4, 5, 6]);
        let steps = run.history.len();
        run.roll_action(actor, &attack(), Mode::Normal).unwrap();
        let turn = run.turn.clone().unwrap();
        assert_eq!(turn.outcome.first.as_ref().unwrap().total, 24);
        // A natural 20 doubles the dice and adds the bonus once.
        assert_eq!(
            turn.outcome.damage.as_ref().unwrap().total,
            3 + 4 + 5 + 6 + 2
        );
        assert_eq!(run.history.len(), steps, "a roll is not an Undo step");
        assert!(
            run.state
                .log
                .iter()
                .any(|line| line.contains("Spear: to hit 1d20 + 4"))
        );
        assert_eq!(run.waiting_damage(actor), None, "never on the attacker");
        let target = (0..3).find(|row| *row != actor).unwrap();
        let hp = run.state.participants[target].hp.unwrap();
        run.apply_waiting(target);
        assert_eq!(run.state.participants[target].hp, Some((hp - 20).max(0)));
        assert_eq!(run.history.len(), steps + 1, "applying is one step");
        let other = (0..3).find(|row| *row != actor && *row != target).unwrap();
        assert_eq!(
            run.waiting_damage(other),
            None,
            "an attack's damage lands once"
        );
        run.undo();
        assert!(run.turn.is_some(), "Undo keeps the roll");
        run.apply(Command::Next).unwrap();
        assert!(run.turn.is_none(), "Next turn ends it");
    }

    #[test]
    fn a_save_damages_each_target_once_full_or_half_and_selection_ends_it() {
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
            }],
        };
        run.state.random = Random::controlled([1, 2, 3, 4]);
        run.roll_action(actor, &save, Mode::Normal).unwrap();
        let others: Vec<usize> = (0..3).filter(|row| *row != actor).collect();
        assert_eq!(run.waiting_damage(others[0]), Some(10));
        run.apply_waiting(others[0]);
        assert_eq!(run.waiting_damage(others[0]), None);
        run.turn.as_mut().unwrap().half = true;
        assert_eq!(run.waiting_damage(others[1]), Some(5), "half rounds down");
        run.forget_turn_unless(others[1]);
        assert!(run.turn.is_none(), "selecting another row ends the action");
    }

    #[test]
    fn a_bonus_adds_to_the_d20_and_a_save_falls_back_to_the_modifier() {
        let mut run = live();
        let actor = run.current().unwrap();
        run.bonuses.insert(actor, "1d4".into());
        run.state.random = Random::controlled([10, 3, 1, 1, 1]);
        run.roll_action(actor, &attack(), Mode::Normal).unwrap();
        let first = run.turn.unwrap().outcome.first.unwrap();
        assert_eq!(first.total, 17);
        assert_eq!(
            first.natural,
            Some(10),
            "the bonus die is never the natural d20"
        );
        let source =
            splat::content::definitions::source_file("definitions/creatures/aboleth.md").unwrap();
        let entry = Entry::parse("Aboleth.md", source).unwrap();
        assert!(save_bonus(&entry, "Dexterity").is_some());
    }
}
