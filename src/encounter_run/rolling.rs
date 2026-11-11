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
use splat::content::dice_text::{D20, RollLink};
use splat::execution::dice::Random;

/// An armed action: whose, which, and how its d20 rolls.
#[derive(Clone, Debug)]
pub(super) struct Armed {
    pub actor: usize,
    pub link: RollLink,
    pub mode: Mode,
}

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
        let result = match &ask.link {
            RollLink::D20 {
                what: D20::Attack, ..
            }
            | RollLink::SaveDc { .. }
            | RollLink::Damage { .. } => {
                self.arm(actor, ask.link, ask.mode);
                Ok(())
            }
            RollLink::D20 { .. } | RollLink::Dice { .. } => {
                self.roll_alone(actor, &ask.link, ask.mode)
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
    fn arm(&mut self, actor: usize, link: RollLink, mode: Mode) {
        self.badges.clear();
        self.saves.clear();
        if self.selected != actor {
            self.select(actor);
        }
        self.armed = Some(Armed { actor, link, mode });
    }

    /// Stop the armed action. Its rows' results stay until the next action or turn.
    pub(crate) fn disarm(&mut self) {
        self.armed = None;
    }

    /// A check, a save, a skill or dice: rolled now, shown on the actor's row.
    fn roll_alone(&mut self, actor: usize, link: &RollLink, mode: Mode) -> Result<(), String> {
        let who = self.state.participants[actor].name.clone();
        let outcome = self.record(
            |random| rolls::make(&who, link, mode, (0, 0), None, random),
            Outcome::lines,
        )?;
        if let Some(first) = &outcome.first {
            self.badges.insert(actor, first.headline());
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
        let mode = mode.unwrap_or(armed.mode);
        let result = match &armed.link {
            RollLink::SaveDc { ability, .. } => {
                self.add_save_target(row, ability);
                Ok(())
            }
            _ if self.player(armed.actor) => {
                // The player says the numbers; the DM types the damage.
                self.aim(row);
                self.badges
                    .insert(row, "Player rolls · type the damage".to_owned());
                Ok(())
            }
            link => self.attack(armed.actor, row, link, mode),
        };
        if let Err(error) = result {
            self.error = Some(error);
        }
        true
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
    fn attack(
        &mut self,
        actor: usize,
        row: usize,
        link: &RollLink,
        mode: Mode,
    ) -> Result<(), String> {
        let who = self.state.participants[actor].name.clone();
        let changes = self.changes(actor);
        let outcome = self.record(
            |random| rolls::make(&who, link, mode, changes, None, random),
            Outcome::lines,
        )?;
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
        Ok(())
    }

    /// Add `row` to a save's targets, and roll its save if it is a creature.
    fn add_save_target(&mut self, row: usize, ability: &str) {
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
            |random| rolls::roll(&name, &save, Mode::Normal, 0, random),
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

    /// Publish the armed action for the tiles drawn next.
    pub(super) fn armed_tiles_begin(&self, ctx: &egui::Context) {
        rolls::set_armed(ctx, self.armed_view());
    }

    /// Handle what was pressed under the armed tile.
    pub(super) fn armed_tiles_end(&mut self, ctx: &egui::Context) {
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
                self.attack(armed.actor, row, &armed.link, armed.mode)
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
    fn save_damage(&mut self, armed: &Armed) -> Result<(), String> {
        let RollLink::SaveDc {
            label,
            damage,
            dc,
            half,
            ..
        } = &armed.link
        else {
            return Ok(());
        };
        let link = RollLink::Damage {
            label: label.clone(),
            parts: damage.clone(),
        };
        let who = self.state.participants[armed.actor].name.clone();
        let changes = self.changes(armed.actor);
        let outcome = self.record(
            |random| rolls::make(&who, &link, Mode::Normal, changes, None, random),
            Outcome::lines,
        )?;
        if let Some(rolled) = &outcome.damage {
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
        Ok(())
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
    }

    /// Ctrl+1 to Ctrl+9 arm the panel's participant's actions, in tile order.
    pub(super) fn arm_by_number(&mut self, number: usize) {
        let row = self.panel_row();
        if let Some(chip) = self.chips(row).into_iter().nth(number.saturating_sub(1)) {
            self.arm(row, chip.link, Mode::Normal);
        }
    }

    /// Escape stops an armed action first.
    pub(super) fn disarm_on_escape(&mut self, ui: &egui::Ui) -> bool {
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
        self.badges.clear();
        self.saves.clear();
    }

    /// Undo can take away rows. An action whose actor is gone ends.
    pub(super) fn forget_removed_rollers(&mut self) {
        let rows = self.state.participants.len();
        if self.armed.as_ref().is_some_and(|armed| armed.actor >= rows) {
            self.armed = None;
        }
        self.badges.retain(|row, _| *row < rows);
        self.saves.retain(|row, _| *row < rows);
    }
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
            }],
        }
    }

    fn ask(link: RollLink, roller: usize) -> Ask {
        Ask {
            link,
            mode: Mode::Normal,
            roller: Some(roller),
            entry: "Guard".into(),
        }
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
