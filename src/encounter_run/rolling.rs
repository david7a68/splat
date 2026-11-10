//! Rolls from dice links during a run: the roll strip above the panel, its follow-ups, and
//! calls for a save. A roll changes no participant. It goes to the Log, and Undo keeps it.
use super::{Panel, Phase, Run};
use crate::design::{self, Action, Density, Type, colors};
use crate::rolls::{self, Ask, Mode, Rolled, Shown};
use eframe::egui::{self, Align, Layout};
use splat::content::Entry;
use splat::content::dice_text::{self, D20, DamagePart, RollLink};
use splat::execution::dice::Random;
use std::collections::BTreeMap;

/// The last roll of this turn, above the panel's tabs. It belongs to its roller, not to the
/// selected row, so it stays when the DM chooses a target.
#[derive(Clone, Debug)]
pub(super) struct Strip {
    /// The row that rolled. A roll from Lookup or the dice field has none.
    pub roller: Option<usize>,
    pub rolled: Rolled,
}

/// A call for a save: its own targets, until `Use` puts them in the run's selection.
#[derive(Clone, Debug)]
pub(super) struct Call {
    pub caster: Option<usize>,
    pub who: String,
    pub label: String,
    pub ability: String,
    pub dc: i64,
    pub half: bool,
    pub damage: Vec<DamagePart>,
    /// The feature's whole paragraph, so its failure and success can be read.
    pub paragraph: Option<String>,
    pub rows: BTreeMap<usize, CallRow>,
}

impl Call {
    /// Tick a row, or take its tick away.
    fn tick(&mut self, row: usize) {
        if self.rows.remove(&row).is_none() {
            self.rows.insert(row, CallRow::default());
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct CallRow {
    pub total: Option<i64>,
    /// A player's own roll, as typed.
    pub typed: String,
    pub half: bool,
}

/// A linked player sheet rolls its own saves.
fn character(entry: &Entry) -> bool {
    entry.fields.contains_key("splat_external")
}

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

/// The paragraph whose run-in title starts with `label`, from a stat block's text.
fn feature_paragraph(entry: &Entry, label: &str) -> Option<String> {
    if label.is_empty() {
        return None;
    }
    entry
        .body
        .split("\n\n")
        .map(str::trim)
        .find(|paragraph| {
            design::reader::run_in_title(paragraph).is_some_and(|title| {
                title == label
                    || title
                        .strip_prefix(label)
                        .is_some_and(|rest| rest.starts_with(" ("))
            })
        })
        .map(str::to_owned)
}

impl Run {
    /// Make the roll a dice link asked for. A save DC opens a call for the save instead.
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
            return Err("This encounter has ended. Undo the ending to continue.".into());
        }
        let roller = self.existing(ask.roller);
        let who = roller.map_or_else(
            || ask.entry.clone(),
            |row| self.state.participants[row].name.clone(),
        );
        if let RollLink::SaveDc {
            label,
            ability,
            dc,
            half,
            damage,
        } = ask.link
        {
            let paragraph = roller.and_then(|row| {
                feature_paragraph(&self.state.participants[row].definition, &label)
            });
            self.open_call(Call {
                caster: roller,
                who,
                label,
                ability,
                dc,
                half,
                damage,
                paragraph,
                rows: BTreeMap::new(),
            });
            return Ok(None);
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
            return Err("This encounter has ended. Undo the ending to continue.".into());
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

    fn after_roll(&mut self, roller: Option<usize>, rolled: Rolled, at: egui::Pos2) -> Shown {
        let live = matches!(self.state.phase, Phase::Running);
        if live {
            self.strip = Some(Strip {
                roller,
                rolled: rolled.clone(),
            });
            // The run's keys work again, so D, T and Enter can follow.
            self.follow_with_keyboard();
        }
        Shown {
            armor: self.armor_words(roller, &rolled),
            use_blocked: self.use_blocked(roller, &rolled).map(str::to_owned),
            actions: live && self.hide_panel,
            rolled,
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

    /// `Use` must not put a feature's damage on the creature that dealt it by mistake.
    fn use_blocked(&self, roller: Option<usize>, rolled: &Rolled) -> Option<&'static str> {
        (rolled.feature_damage() && roller == Some(self.selected) && self.also.is_empty())
            .then_some("Choose a target first.")
    }

    /// `Roll damage` after an attack: the same feature's damage, by the roller's values.
    /// A natural 20 rolls it as a critical hit.
    /// `at` places the result beside its link, when the side panel is hidden.
    pub(super) fn roll_strip_damage(&mut self, at: Option<egui::Pos2>) -> Option<Shown> {
        let strip = self.strip.clone()?;
        let damage = strip.rolled.damage().to_vec();
        if !strip.rolled.attack() || damage.is_empty() {
            return None;
        }
        let label = strip.rolled.feature().unwrap_or_default().to_owned();
        let link = RollLink::Damage {
            label,
            parts: damage,
        };
        let mode = if strip.rolled.natural == Some(20) {
            Mode::Critical
        } else {
            Mode::Normal
        };
        let roller = self.existing(strip.roller);
        let change = self.change(roller, &link);
        let who = strip.rolled.who.clone();
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

    /// `Use`: put the strip's total in Amount, for the selected targets.
    pub(super) fn use_strip_total(&mut self) {
        let Some(strip) = &self.strip else {
            return;
        };
        if !strip.rolled.usable() || self.use_blocked(strip.roller, &strip.rolled).is_some() {
            return;
        }
        self.amount = strip.rolled.total.max(0).to_string();
        self.focus_amount = true;
    }

    /// What a result beside a link asked for, with the side panel hidden.
    pub(crate) fn rolled_action(&mut self, action: rolls::Action, at: egui::Pos2) -> Option<Shown> {
        match action {
            rolls::Action::RollDamage => self.roll_strip_damage(Some(at)),
            rolls::Action::Use => {
                self.use_strip_total();
                None
            }
        }
    }

    pub(crate) fn has_strip(&self) -> bool {
        self.strip.is_some()
    }

    /// The strip's result again, as the selection has changed since the roll.
    pub(crate) fn refreshed(&self, shown: &Shown) -> Shown {
        let roller = self.strip.as_ref().and_then(|strip| strip.roller);
        Shown {
            armor: self.armor_words(roller, &shown.rolled),
            use_blocked: self.use_blocked(roller, &shown.rolled).map(str::to_owned),
            ..shown.clone()
        }
    }

    /// Two lines above the panel's tabs: the last roll, then what can follow it.
    pub(super) fn strip_view(&mut self, ui: &mut egui::Ui) {
        let Some(strip) = self.strip.clone() else {
            return;
        };
        let c = colors();
        let rolled = &strip.rolled;
        let roller = self.existing(strip.roller);
        ui.spacing_mut().item_spacing.y = design::XS;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = design::S;
            let headline = ui.label(Type::Strong.rich(ui, rolled.headline()).color(c.text));
            design::named(ui, &headline, &rolled.line());
            if let Some(armor) = self.armor_words(roller, rolled) {
                ui.label(Type::Body.rich(ui, armor).color(c.text));
            }
            if let Some(words) = rolled.natural_words() {
                ui.label(
                    Type::Body
                        .rich(ui, format!("{} {words}", design::icons::INFO))
                        .color(c.text),
                );
            }
            // The dice go first, then the feature; the roller's name stays.
            let name = rolled.name();
            let mut full = format!("{name} · {}", rolled.parts());
            if let Some(words) = rolled.changed_words() {
                full.push_str(&format!(" · {words}"));
            }
            let font = Type::Small.font(ui);
            let room = ui.available_width();
            let context = [full.as_str(), name.as_str(), rolled.who.as_str()]
                .into_iter()
                .find(|words| design::number_width(ui, words, &font) <= room)
                .map(str::to_owned);
            if let Some(context) = context {
                ui.label(Type::Small.rich(ui, context).color(c.muted))
                    .on_hover_text(rolled.line());
            }
        });
        let mut damage = false;
        let mut use_total = false;
        let mut back = None;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = design::S;
            if self.call.is_none() || self.panel != Panel::Call {
                match rolls::actions(ui, rolled, self.use_blocked(roller, rolled)) {
                    Some(rolls::Action::RollDamage) => damage = true,
                    Some(rolls::Action::Use) => use_total = true,
                    None => {}
                }
            }
            if let Some(row) = roller
                && Some(row) != self.current()
                && row != self.selected
            {
                let name = &self.state.participants[row].name;
                let words = format!("Back to {name}");
                // After the buttons, in the room they leave, so it never passes the edge.
                let font = Type::Body.font(ui);
                let room =
                    ui.available_width() - design::number_width(ui, "Back to ", &font) - design::S;
                let shown = format!("Back to {}", super::view::dock_name(ui, name, &font, room));
                let response = design::link(ui, &shown);
                design::named(ui, &response, &words);
                if response
                    .on_hover_text(format!("Select {name} again, for its next roll."))
                    .clicked()
                {
                    back = Some(row);
                }
            }
        });
        if damage {
            self.roll_strip_damage(None);
        }
        if use_total {
            self.use_strip_total();
        }
        if let Some(row) = back {
            self.select(row);
            self.panel = Panel::Reference;
        }
        design::rule(ui);
    }

    fn open_call(&mut self, call: Call) {
        // The same DC opens the same call again, with its ticks.
        let same = self.call.as_ref().is_some_and(|open| {
            open.caster == call.caster
                && open.label == call.label
                && open.dc == call.dc
                && open.ability == call.ability
        });
        if !same {
            self.call = Some(call);
            self.also.clear();
            self.halved.clear();
        }
        self.panel = Panel::Call;
        self.close_lookup = true;
    }

    /// Ctrl+click adds a target: to the open call, or else to the run's amount.
    pub(super) fn ctrl_click(&mut self, row: usize) {
        if !self.call_toggle(row) {
            self.toggle_target(row);
        }
    }

    /// A Ctrl+click on a roster row while the call is open ticks that row in the call.
    pub(super) fn call_toggle(&mut self, row: usize) -> bool {
        let Some(call) = self.call.as_mut().filter(|_| self.panel == Panel::Call) else {
            return false;
        };
        if Some(row) != call.caster {
            call.tick(row);
        }
        true
    }

    /// The call's ticked creatures that have not saved yet and can roll here.
    fn unrolled_creatures(&self, call: &Call) -> Vec<usize> {
        call.rows
            .iter()
            .filter(|(row, entry)| {
                let definition = &self.state.participants[**row].definition;
                entry.total.is_none()
                    && !character(definition)
                    && save_bonus(definition, &call.ability).is_some()
            })
            .map(|(row, _)| *row)
            .collect()
    }

    fn roll_save(&mut self, row: usize) {
        let Some(call) = self.call.clone() else {
            return;
        };
        let p = &self.state.participants[row];
        let Some(bonus) = save_bonus(&p.definition, &call.ability) else {
            return;
        };
        let link = RollLink::d20(D20::Save, bonus, call.ability.clone());
        let name = p.name.clone();
        match self.record(|random| rolls::roll(&name, &link, Mode::Normal, 0, random)) {
            Ok(rolled) => {
                if let Some(entry) = self.call.as_mut().and_then(|call| call.rows.get_mut(&row)) {
                    entry.total = Some(rolled.total);
                }
            }
            Err(error) => self.error = Some(error),
        }
    }

    /// `Use 28 on 3 targets`: the call's ticks become the run's targets, then Damage applies.
    fn use_call(&mut self, total: i64) {
        let Some(call) = &self.call else {
            return;
        };
        let rows: Vec<usize> = call.rows.keys().copied().collect();
        let halved: Vec<usize> = call
            .rows
            .iter()
            .filter(|(_, row)| row.half)
            .map(|(at, _)| *at)
            .collect();
        let Some(&first) = rows.first() else {
            return;
        };
        self.select(first);
        if rows.len() == 1 {
            // One target has no half set, so the halved number goes in Amount.
            let amount = if halved.contains(&first) {
                total / 2
            } else {
                total
            };
            self.amount = amount.max(0).to_string();
        } else {
            self.also = rows[1..].iter().copied().collect();
            self.halved = halved.into_iter().collect();
            self.amount = total.max(0).to_string();
        }
        self.focus_amount = true;
    }

    /// The call view: the feature, its targets with their saves, then the damage and `Use`.
    pub(super) fn call_tab(&mut self, ui: &mut egui::Ui) {
        let Some(call) = self.call.clone() else {
            self.panel = Panel::Reference;
            return;
        };
        let c = colors();
        let mut close = false;
        ui.horizontal(|ui| {
            ui.label(
                Type::Heading
                    .rich(
                        ui,
                        format!("{} · DC {} {} save", call.label, call.dc, call.ability),
                    )
                    .color(c.text),
            );
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if Action::quiet("Close")
                    .density(Density::Read)
                    .hint("Close the call. Its ticks stay until Next turn.")
                    .show(ui)
                    .clicked()
                {
                    close = true;
                }
            });
        });
        if close {
            self.panel = Panel::Reference;
            return;
        }
        let mut actions = Vec::new();
        egui::ScrollArea::vertical()
            .id_salt("run_call")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = design::XS;
                if let Some(paragraph) = &call.paragraph {
                    rolls::set_roller(ui.ctx(), call.caster);
                    rolls::set_entry(ui.ctx(), &call.who);
                    design::reader::measure(ui, |ui| {
                        design::reader::source_prose(
                            ui,
                            &dice_text::with_links(paragraph, &call.label),
                        );
                    });
                    rolls::set_roller(ui.ctx(), None);
                }
                ui.add_space(design::S);
                design::group_heading(ui, "Targets", call.rows.len());
                ui.label(
                    Type::Small
                        .rich(ui, "Tick each target. Ctrl+click on a row also ticks it.")
                        .color(c.muted),
                );
                for &row in &self.state.order {
                    let p = &self.state.participants[row];
                    if Some(row) == call.caster || !p.present {
                        continue;
                    }
                    let ticked = call.rows.get(&row);
                    let character = character(&p.definition);
                    let bonus = save_bonus(&p.definition, &call.ability);
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = design::S;
                        let mut on = ticked.is_some();
                        if ui.checkbox(&mut on, &p.name).changed() {
                            actions.push(CallStep::Tick(row));
                        }
                        let Some(entry) = ticked else {
                            return;
                        };
                        if character {
                            let mut typed = entry.typed.clone();
                            let field = design::text_input(
                                ui,
                                &format!("Save of {}", p.name),
                                &mut typed,
                                "Total",
                                52.0,
                            );
                            if field.changed() {
                                actions.push(CallStep::Typed(row, typed));
                            }
                        }
                        if bonus.is_some() && entry.total.is_none() {
                            let word = if character { "Roll for them" } else { "Roll" };
                            if Action::secondary(word)
                                .density(Density::Read)
                                .show(ui)
                                .clicked()
                            {
                                actions.push(CallStep::Roll(row));
                            }
                        }
                        if let Some(total) = entry.total {
                            let made = total >= call.dc;
                            ui.label(Type::Strong.rich(ui, total.to_string()).color(c.text));
                            ui.label(
                                Type::Small
                                    .rich(ui, if made { "At or above DC" } else { "Below DC" })
                                    .color(c.muted),
                            );
                        }
                        let mut half = entry.half;
                        if ui.checkbox(&mut half, "Half").changed() {
                            actions.push(CallStep::Half(row, half));
                        }
                    });
                }
                ui.add_space(design::S);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = design::S;
                    if !self.unrolled_creatures(&call).is_empty()
                        && Action::secondary("Roll all creatures")
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                    {
                        actions.push(CallStep::RollAll);
                    }
                    if call.half
                        && call
                            .rows
                            .values()
                            .any(|row| row.total.is_some_and(|t| t >= call.dc))
                        && Action::secondary("Half for those at or above DC")
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                    {
                        actions.push(CallStep::HalfMade);
                    }
                });
                if !call.damage.is_empty() {
                    ui.add_space(design::S);
                    let link = RollLink::Damage {
                        label: call.label.clone(),
                        parts: call.damage.clone(),
                    };
                    ui.horizontal_wrapped(|ui| {
                        ui.label(Type::Body.rich(ui, "Damage").color(c.text));
                        rolls::set_roller(ui.ctx(), call.caster);
                        rolls::set_entry(ui.ctx(), &call.who);
                        rolls::link(ui, &rolls::parts_text(&call.damage), Type::Body, &link);
                        rolls::set_roller(ui.ctx(), None);
                    });
                }
                let rolled = self.strip.as_ref().map(|strip| &strip.rolled).filter(|rolled| {
                    matches!(&rolled.link, RollLink::Damage { label, .. } if *label == call.label)
                });
                let targets = call.rows.len();
                let total = rolled.map(|rolled| rolled.total);
                let half_one = targets == 1 && call.rows.values().all(|row| row.half);
                let word = match total {
                    Some(total) if half_one => {
                        let name = call
                            .rows
                            .keys()
                            .next()
                            .map(|row| self.state.participants[*row].name.clone())
                            .unwrap_or_default();
                        format!("Use {} (half) on {name}", total / 2)
                    }
                    Some(total) => format!(
                        "Use {total} on {targets} {}",
                        if targets == 1 { "target" } else { "targets" }
                    ),
                    None => "Use".to_owned(),
                };
                let blocked = if targets == 0 {
                    Some("Tick a target first.")
                } else if total.is_none() {
                    Some("Roll the damage first.")
                } else {
                    None
                };
                ui.add_space(design::S);
                if Action::primary(&word)
                    .density(Density::Read)
                    .hint("Put the total in Amount for the ticked targets. Then press Damage.")
                    .blocked(blocked)
                    .show(ui)
                    .clicked()
                    && let Some(total) = total
                {
                    actions.push(CallStep::Use(total));
                }
            });
        for step in actions {
            self.call_step(step);
        }
    }

    fn call_step(&mut self, step: CallStep) {
        match step {
            CallStep::Tick(row) => {
                if let Some(call) = &mut self.call {
                    call.tick(row);
                }
            }
            CallStep::Typed(row, typed) => {
                if let Some(entry) = self.call.as_mut().and_then(|call| call.rows.get_mut(&row)) {
                    entry.total = typed.trim().parse().ok();
                    entry.typed = typed;
                }
            }
            CallStep::Roll(row) => self.roll_save(row),
            CallStep::RollAll => {
                let rows = self
                    .call
                    .as_ref()
                    .map_or_else(Vec::new, |call| self.unrolled_creatures(call));
                for row in rows {
                    self.roll_save(row);
                }
            }
            CallStep::Half(row, half) => {
                if let Some(entry) = self.call.as_mut().and_then(|call| call.rows.get_mut(&row)) {
                    entry.half = half;
                }
            }
            CallStep::HalfMade => {
                if let Some(call) = &mut self.call {
                    let dc = call.dc;
                    for entry in call.rows.values_mut() {
                        if entry.total.is_some_and(|total| total >= dc) {
                            entry.half = true;
                        }
                    }
                }
            }
            CallStep::Use(total) => self.use_call(total),
        }
    }

    /// Escape closes the call view, and keeps its ticks.
    pub(super) fn call_escape(&mut self, ui: &egui::Ui) -> bool {
        if self.panel == Panel::Call
            && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        {
            self.panel = Panel::Reference;
            return true;
        }
        false
    }

    /// Next turn ends this turn's roll and its call.
    pub(super) fn end_turn_rolls(&mut self) {
        self.strip = None;
        self.call = None;
        if self.panel == Panel::Call {
            self.panel = Panel::Reference;
        }
    }

    /// Undo can take away rows. A roll keeps its result, but not a roller that is gone.
    pub(super) fn forget_removed_rollers(&mut self) {
        let rows = self.state.participants.len();
        if let Some(strip) = &mut self.strip
            && strip.roller.is_some_and(|row| row >= rows)
        {
            strip.roller = None;
        }
        if let Some(call) = &mut self.call {
            call.rows.retain(|row, _| *row < rows);
            if call.caster.is_some_and(|row| row >= rows) {
                call.caster = None;
            }
        }
    }
}

enum CallStep {
    Tick(usize),
    Typed(usize, String),
    Roll(usize),
    RollAll,
    Half(usize, bool),
    HalfMade,
    Use(i64),
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
    fn attack_then_target_then_critical_damage_then_use() {
        let mut run = live();
        let roller = run.current().unwrap();
        run.state.random = Random::controlled([20, 3, 4, 5, 6]);
        let steps = run.history.len();
        let shown = run.roll_link(ask(attack(), Some(roller))).unwrap().unwrap();
        assert_eq!(shown.rolled.total, 24);
        assert_eq!(shown.rolled.natural_words(), Some("Natural 20"));
        assert_eq!(shown.armor, None, "the roller is still selected");
        assert_eq!(run.history.len(), steps, "a roll is not an Undo step");
        assert!(
            run.state
                .log
                .last()
                .unwrap()
                .contains("Spear: to hit 1d20 + 4")
        );
        let target = (0..3).find(|row| *row != roller).unwrap();
        run.select(target);
        let strip = run.strip.clone().unwrap();
        let p = &run.state.participants[target];
        assert_eq!(
            run.armor_words(strip.roller, &strip.rolled),
            Some(format!("AC {} {}", p.armor_class.unwrap(), p.name))
        );
        // A natural 20 doubles the dice and adds the bonus once.
        run.roll_strip_damage(None);
        assert_eq!(run.strip.as_ref().unwrap().rolled.total, 3 + 4 + 5 + 6 + 2);
        run.use_strip_total();
        assert_eq!(run.amount, "20");
        assert!(run.focus_amount);
    }

    #[test]
    fn use_waits_for_a_target_undo_keeps_the_strip_and_next_turn_ends_it() {
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
        let shown = run.roll_link(ask(damage, Some(roller))).unwrap().unwrap();
        assert_eq!(shown.use_blocked.as_deref(), Some("Choose a target first."));
        run.use_strip_total();
        assert!(
            run.amount.is_empty(),
            "no damage lands on its roller by mistake"
        );
        // Plain dice can be used on anyone.
        run.roll_dice("1d4").unwrap();
        run.use_strip_total();
        assert_eq!(run.amount, "3");
        run.amount.clear();
        run.apply(Command::Damage(roller, 1)).unwrap();
        run.undo();
        assert!(run.strip.is_some(), "Undo keeps the roll");
        run.apply(Command::Next).unwrap();
        assert!(run.strip.is_none(), "Next turn ends it");
    }

    #[test]
    fn a_call_ticks_its_own_targets_then_use_sets_the_run_selection_and_half() {
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
        assert!(
            run.roll_link(ask(save.clone(), Some(caster)))
                .unwrap()
                .is_none()
        );
        assert!(run.panel == Panel::Call);
        let others: Vec<usize> = (0..3).filter(|row| *row != caster).collect();
        run.select(others[0]);
        assert!(run.panel == Panel::Call, "a roster click keeps the call");
        assert!(run.call_toggle(others[0]));
        assert!(run.call_toggle(others[1]));
        assert!(
            run.also.is_empty(),
            "Ctrl+click ticks the call, not the run"
        );
        run.call_step(CallStep::Typed(others[0], "15".into()));
        run.call_step(CallStep::Typed(others[1], "9".into()));
        run.call_step(CallStep::HalfMade);
        let call = run.call.clone().unwrap();
        assert!(call.rows[&others[0]].half && !call.rows[&others[1]].half);
        // Escape closes the view; the same DC opens it again with its ticks.
        run.panel = Panel::Reference;
        run.roll_link(ask(save, Some(caster))).unwrap();
        assert_eq!(run.call.as_ref().unwrap().rows.len(), 2);
        run.call_step(CallStep::Use(10));
        assert_eq!(run.amount, "10");
        let mut targets = run.targets();
        targets.sort_unstable();
        assert_eq!(targets, others);
        assert_eq!(
            run.halved.iter().copied().collect::<Vec<_>>(),
            vec![others[0]]
        );
        // One target with Half takes the halved number in Amount.
        run.call_step(CallStep::Tick(others[1]));
        run.call_step(CallStep::Use(11));
        assert_eq!(run.amount, "5");
        assert!(run.also.is_empty());
    }

    #[test]
    fn a_save_falls_back_to_the_modifier_and_a_feature_is_found_by_its_title() {
        let source =
            splat::content::definitions::source_file("definitions/creatures/aboleth.md").unwrap();
        let entry = Entry::parse("Aboleth.md", source).unwrap();
        assert!(save_bonus(&entry, "Dexterity").is_some());
        let found = feature_paragraph(&entry, "Tentacle").unwrap_or_default();
        assert!(found.starts_with("Tentacle."), "{found}");
    }
}
