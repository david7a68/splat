//! The run's dock: the Amount field with Damage, Heal and Temp HP, the dice field, and the dock's
//! second line of quiet actions.
use super::{
    Command, Run,
    view::{
        AMOUNT, DOCK_ALL_WORDS_FROM, DOCK_HEIGHT, dock_name_shown, presence_action, toggle,
        toggle_width,
    },
};
use crate::design::{self, Action, Density, Type, colors, icons};
use eframe::egui::{self, Align, Key, Layout, vec2};

/// How `Effect` fits beside the selected name.
#[derive(Clone, Copy, PartialEq)]
enum EffectFit {
    Word,
    Icon,
    None,
}

/// A roll's result has its place before there is one, so that nothing moves when it comes.
const DICE_RESULT_WIDTH: f32 = 32.0;

impl Run {
    /// The entry controls have one fixed home, labelled with the row they act on, so
    /// no row moves when the selection changes. Typing needs no pointer travel at all.
    pub(super) fn dock(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        let index = self.selected;
        let mut command = None;
        let mut open_conditions = false;
        let mut open_side = false;
        let mut open_effect = false;
        let mut in_dock = false;
        let frame = egui::Frame::new()
            .fill(c.selected)
            .inner_margin(egui::Margin::symmetric(design::S as i8, design::XS as i8));
        frame.show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_height(DOCK_HEIGHT - design::S);
            // The background takes the roster's width; the controls keep to one block, no wider
            // than a list, so the DM never crosses the screen between them.
            ui.scope(|ui| {
                ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
                ui.spacing_mut().item_spacing = vec2(design::XS, design::XS);
                let value = self.amount.parse::<i64>().ok();
                let empty = value.is_none().then_some("Type an amount first.");
                // The narrowest dock keeps every word by giving up the hint and an icon.
                let narrow = ui.available_width() < DOCK_ALL_WORDS_FROM;
                ui.horizontal(|ui| {
                    self.amount_field(ui, if narrow { 68.0 } else { 84.0 });
                    let damage = Action::secondary("Damage").density(Density::Glance);
                    let damage = if narrow { damage } else { damage.key("Enter") };
                    if damage
                        .hint("Apply damage. Temporary HP absorbs it first.")
                        .blocked(empty)
                        .show(ui)
                        .clicked()
                    {
                        command = value.map(|n| self.amount_command(n, Command::Damage));
                    }
                    ui.add_space(if narrow { design::XS } else { design::M });
                    if Action::quiet("Heal")
                        .density(Density::Glance)
                        .hint("Heal (Shift+Enter)")
                        .blocked(empty)
                        .show(ui)
                        .clicked()
                    {
                        command = value.map(|n| self.amount_command(n, Command::Heal));
                    }
                    if Action::quiet("Temp HP")
                        .icon(icons::TEMP_HP)
                        .density(Density::Glance)
                        .name("Replace temporary HP")
                        .hint("Replace temporary HP with this amount")
                        .blocked(empty)
                        .show(ui)
                        .clicked()
                    {
                        command = value.map(|n| self.amount_command(n, Command::TempHp));
                    }
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        self.dice(ui, narrow);
                    });
                });
                if !self.also.is_empty() {
                    self.effect_in_dock = false;
                    self.target_words(ui);
                    return;
                }
                // Line two becomes one field while the DM adds an effect to this row. The
                // field belongs to that row: another selection closes it, and its words wait.
                let key = &self.state.participants[index].key;
                if self.effect_open.as_ref().is_some_and(|open| open != key) {
                    self.effect_open = None;
                }
                if self.effect_open.is_some() {
                    command = self.effect_line(ui, index);
                    return;
                }
                let p = &self.state.participants[index];
                let death_saves = self.hide_panel && p.hp == Some(0);
                let conditions_key = if self.lookup_open { "" } else { "C" };
                // The selected name, `L`, then its controls: one block after the creature.
                let (presence, hint) = presence_action(p.present);
                let gap = ui.spacing().item_spacing.x;
                let controls = toggle_width(ui, presence, icons::ABSENT)
                    + gap
                    + design::S
                    + toggle_width(ui, "Conc", icons::CONCENTRATING)
                    + gap
                    + if death_saves {
                        Action::quiet("Death saves")
                            .density(Density::Read)
                            .natural_width(ui)
                            + gap
                    } else {
                        0.0
                    }
                    + Action::quiet("Conditions")
                        .density(Density::Read)
                        .key(conditions_key)
                        .natural_width(ui)
                    + gap;
                let height = Density::Read.control();
                let effect_key = if self.lookup_open || self.notes_open {
                    ""
                } else {
                    "E"
                };
                let running = self.state.phase == super::Phase::Running;
                let effect_word = Self::effect_action(effect_key).natural_width(ui);
                let effect_icon = Density::Read.control();
                let mut effect = EffectFit::None;
                // Left to right: the name, then exactly `L`, then its controls. The controls'
                // width only limits how long a name may be.
                ui.horizontal(|ui| {
                    ui.set_min_height(height);
                    if narrow && let Some((total, line)) = self.dock_roll() {
                        ui.label(Type::Strong.rich(ui, format!("= {total}")))
                            .on_hover_text(line);
                    }
                    let room = ui.available_width() - controls - design::L;
                    let (role, shown) = dock_name_shown(ui, &p.name, room);
                    // A name that lost a word or its size has no room to give.
                    let whole = role == Type::Heading && shown == p.name;
                    let response = ui
                        .label(role.rich(ui, shown).color(c.text))
                        .on_hover_text(&p.name);
                    design::named(ui, &response, &format!("Selected: {}", p.name));
                    ui.add_space(design::L - gap);
                    if Action::quiet("Conditions")
                        .density(Density::Read)
                        .name("Conditions for this participant")
                        .key(conditions_key)
                        .show(ui)
                        .clicked()
                    {
                        open_conditions = true;
                    }
                    // Notes hide the panel with the saves, so one word leads there.
                    if death_saves
                        && Action::quiet("Death saves")
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                    {
                        open_side = true;
                    }
                    if toggle(
                        ui,
                        "Conc",
                        icons::CONCENTRATING,
                        p.concentration,
                        "Concentrating",
                        Density::Read,
                    )
                    .on_hover_text("Mark concentration. It records your ruling only.")
                    .clicked()
                    {
                        command = Some(Command::Concentration(index, !p.concentration));
                    }
                    ui.add_space(design::S);
                    if toggle(
                        ui,
                        presence,
                        icons::ABSENT,
                        !p.present,
                        presence,
                        Density::Read,
                    )
                    .on_hover_text(hint)
                    .clicked()
                    {
                        command = Some(Command::Presence(index, !p.present));
                    }
                    // `Effect` takes only the room that is spare after Absent, `L` after it, so
                    // no control of the line moves and the name keeps all its room: with its
                    // word, else with its icon, else not at all.
                    let spare = ui.available_width() - design::L + gap;
                    if running && whole && spare >= effect_word {
                        effect = EffectFit::Word;
                    } else if running && whole && spare >= effect_icon {
                        effect = EffectFit::Icon;
                    }
                    if effect != EffectFit::None {
                        ui.add_space(design::L - gap);
                    }
                    let pressed = match effect {
                        EffectFit::Word => Self::effect_action(effect_key).show(ui).clicked(),
                        EffectFit::Icon => {
                            design::icon_button(ui, icons::PIN, "Effect: words for this row")
                                .clicked()
                        }
                        EffectFit::None => false,
                    };
                    if pressed {
                        open_effect = true;
                    }
                });
                in_dock = effect != EffectFit::None;
            });
        });
        self.effect_in_dock = in_dock;
        if open_conditions {
            self.open_conditions_tab();
        }
        if open_effect {
            self.open_effect_field();
        }
        if open_side {
            self.close_lookup = true;
        }
        if let Some(command) = command {
            let applied = matches!(
                command,
                Command::Damage(..) | Command::Heal(..) | Command::TempHp(..) | Command::Several(_)
            );
            self.run_command(command);
            if applied {
                self.amount.clear();
                self.clear_targets();
                self.follow_with_keyboard();
            }
        }
    }

    /// The quiet `Effect` of the dock's second line, with its key while the key works.
    fn effect_action(key: &str) -> Action<'_> {
        Action::quiet("Effect")
            .icon(icons::PIN)
            .density(Density::Read)
            .name("Effect: words for this row")
            .hint("Pin words to this row, such as Bless 10 rounds")
            .key(key)
    }

    /// The dock's second line as one field, for the row it opened for. Enter adds the words
    /// and Escape clears them; either closes the field and puts focus in `Amount`. Any other
    /// way out, such as `Next turn`, closes it, and its words wait for that row.
    fn effect_line(&mut self, ui: &mut egui::Ui, index: usize) -> Option<Command> {
        let p = &self.state.participants[index];
        let (key, hint) = (
            p.key.clone(),
            format!("Effect on {}, such as Bless 10 rounds", p.name),
        );
        let field = self.effect_fields.entry(key.clone()).or_default();
        let result = ui
            .horizontal(|ui| {
                ui.set_min_height(Density::Read.control());
                design::add_field(
                    ui,
                    field,
                    "Effect",
                    &hint,
                    0.0,
                    |_| None::<()>,
                    |_| Vec::new(),
                )
            })
            .inner;
        let escape = ui.input(|i| i.key_pressed(Key::Escape));
        let words = field.words.trim().to_owned();
        let mut command = None;
        if result.enter || (escape && result.field.lost_focus()) {
            if result.enter && !words.is_empty() {
                command = Some(Command::Condition(index, words));
            }
            field.clear();
            self.effect_open = None;
            self.focus_amount = true;
        } else if result.field.lost_focus() {
            self.effect_open = None;
        }
        if self.effect_open.is_none() {
            self.effect_fields
                .retain(|_, field| !field.words.is_empty());
        }
        command
    }

    /// In place of the one row's actions: how many take this amount, and how to give half. The
    /// choice is on each row, so that any number of targets stays in view.
    fn target_words(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_min_height(28.0);
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(
                            ui,
                            format!(
                                "{} targets. Press a row’s mark for half, rounded down.",
                                self.targets().len()
                            ),
                        )
                        .color(colors().muted),
                )
                .truncate(),
            );
        });
    }

    /// Beside the amount, so the stat block that asks for the roll stays in view. Laid out
    /// right to left: the result, then the button, then the field.
    fn dice(&mut self, ui: &mut egui::Ui, narrow: bool) {
        ui.spacing_mut().item_spacing.x = design::XS;
        if !narrow {
            ui.allocate_ui_with_layout(
                vec2(DICE_RESULT_WIDTH, Density::Read.control()),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.set_width(DICE_RESULT_WIDTH);
                    if let Some((total, line)) = self.dock_roll() {
                        ui.label(
                            Type::Heading
                                .rich(ui, format!("= {total}"))
                                .color(colors().text),
                        )
                        .on_hover_text(line);
                    }
                },
            );
        }
        let roll = Action::secondary("Roll");
        let roll = if narrow { roll } else { roll.icon(icons::DICE) };
        let pressed = roll
            .name("Roll dice")
            .density(Density::Read)
            .hint("Roll this expression. The result goes in the log.")
            .show(ui)
            .clicked();
        let width = if narrow { 60.0 } else { 72.0 };
        let field = design::text_input(ui, "Dice", &mut self.dice, "1d20", width);
        if field.gained_focus() {
            design::focus_and_select(ui, field.id, &self.dice);
        }
        if pressed || (field.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter))) {
            let expression = if self.dice.trim().is_empty() {
                "1d20".to_owned()
            } else {
                self.dice.trim().to_owned()
            };
            match self.roll_dice(&expression) {
                Err(error) => self.error = Some(error),
                Ok(()) => {
                    self.condition.clear();
                    self.note.clear();
                }
            }
            self.follow_with_keyboard();
        }
    }

    /// The dice field's last roll, beside it.
    fn dock_roll(&self) -> Option<(i64, String)> {
        let rolled = self.dock_roll.as_ref()?;
        Some((rolled.total, rolled.line()))
    }

    /// Beside Notes, a letter in Amount means the DM went on writing. It and the digits
    /// before it go to the note, and they never apply as an amount.
    fn take_words_for_note(&mut self, ui: &egui::Ui) {
        let id = design::focus_id(AMOUNT);
        if let Some(typed) =
            Self::words_in_number_field(self.notes_open, ui, id, |c| c.is_ascii_digit())
        {
            self.for_note = Some(format!("{}{typed}", std::mem::take(&mut self.amount)));
            self.return_focus = true;
        }
    }

    fn amount_field(&mut self, ui: &mut egui::Ui, width: f32) {
        self.take_words_for_note(ui);
        let response = design::number_field(
            ui,
            AMOUNT,
            &mut self.amount,
            AMOUNT,
            width,
            Density::Glance.control(),
        );
        self.amount.retain(|d| d.is_ascii_digit());
        // Typed, cleared or typed again: the amount is the DM's until a new roll.
        if response.changed() {
            self.amount_typed = true;
        }
        if std::mem::take(&mut self.focus_amount) {
            response.request_focus();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::kittest::Queryable;
    use std::{cell::RefCell, collections::BTreeSet, rc::Rc};

    fn dock(name: &str, width: f32) -> (Rc<RefCell<Run>>, egui_kittest::Harness<'static>) {
        let mut run = super::super::tests::run();
        for (index, total) in [20, 15, 10].into_iter().enumerate() {
            run.apply(Command::Initiative(index, Some(total))).unwrap();
        }
        run.apply(Command::Begin).unwrap();
        run.state.participants[0].name = name.into();
        let run = Rc::new(RefCell::new(run));
        let shown = run.clone();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(width, 200.0))
            .build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| shown.borrow_mut().dock(ui));
            });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        (run, harness)
    }

    /// The painted name, and its size.
    fn name_shown(harness: &egui_kittest::Harness<'_>, name: &str) -> Option<(String, f32)> {
        let first = name.split_whitespace().next().unwrap();
        harness
            .output()
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.text().starts_with(first) => Some((
                    text.galley.text().to_owned(),
                    text.galley.job.sections[0].format.font_id.size,
                )),
                _ => None,
            })
    }

    /// F2: `Effect` takes only the room after Absent: with its word, else its icon, else not
    /// at all. The name and today's controls keep their places at every width.
    #[test]
    fn effect_gives_up_its_place_before_the_name_loses_a_word_or_its_size() {
        let name = "Oren Thistlewick of the Southern Road";
        let (_, wide) = dock(name, 900.0);
        let (_, size) = name_shown(&wide, name).unwrap();
        let mut forms = BTreeSet::new();
        for width in (360..=760).step_by(8) {
            let (run, harness) = dock(name, width as f32);
            let effect = harness.query_by_label("Effect: words for this row");
            let word = harness.output().shapes.iter().any(|shape| {
                matches!(&shape.shape, egui::Shape::Text(text) if text.galley.text() == "Effect")
            });
            let form = match (effect.is_some(), word) {
                (true, true) => "word",
                (true, false) => "icon",
                (false, _) => "none",
            };
            forms.insert(form);
            assert_eq!(run.borrow().effect_in_dock, effect.is_some(), "{width}");
            if effect.is_some() {
                assert_eq!(
                    name_shown(&harness, name),
                    Some((name.to_owned(), size)),
                    "{width} {form}: the name keeps its size and its words"
                );
            }
            // Conditions, Conc and Absent stay where they are without Effect: in Setup.
            let absent = harness.get_by_label("Absent").rect();
            let (setup, harness) = dock(name, width as f32);
            setup.borrow_mut().state.phase = crate::encounter_run::Phase::Setup;
            let mut harness = harness;
            harness.run_steps(2);
            assert!(harness.query_by_label("Effect: words for this row").is_none());
            assert_eq!(harness.get_by_label("Absent").rect(), absent, "{width} {form}");
        }
        assert_eq!(
            forms,
            BTreeSet::from(["icon", "none", "word"]),
            "each form at some width"
        );
    }

    /// F2: the dock's field adds one effect to the row it opened for. Enter adds and puts
    /// focus in Amount; Escape clears; another selection closes it, and its words wait.
    #[test]
    fn the_effect_field_belongs_to_its_row() {
        let (run, mut harness) = dock("Guard", 900.0);
        let field = "Effect";
        harness.get_by_label("Effect: words for this row").click();
        harness.run_steps(3);
        harness.get_by_label(field).type_text("Bless 10 rounds");
        harness.run_steps(1);
        harness.key_press(egui::Key::Enter);
        harness.run_steps(2);
        {
            let run = run.borrow();
            assert!(
                run.state.participants[0]
                    .conditions
                    .iter()
                    .any(|c| c == "Bless 10")
            );
            assert_eq!(run.undo_label(), "Undo: effect Bless on Guard");
            assert!(run.effect_open.is_none() && run.effect_fields.is_empty());
        }
        assert!(harness.query_by_label(field).is_none(), "the field closed");
        // Escape clears the words and adds nothing.
        harness.get_by_label("Effect: words for this row").click();
        harness.run_steps(3);
        harness.get_by_label(field).type_text("Haste");
        harness.run_steps(1);
        harness.key_press(egui::Key::Escape);
        harness.run_steps(2);
        assert!(run.borrow().effect_fields.is_empty());
        assert_eq!(run.borrow().state.participants[0].conditions.len(), 2);
        // Next turn moves the selection: the field closes, and its words wait for its row.
        harness.get_by_label("Effect: words for this row").click();
        harness.run_steps(3);
        harness.get_by_label(field).type_text("Haste");
        harness.run_steps(1);
        run.borrow_mut().run_command(Command::Next);
        harness.run_steps(2);
        assert!(harness.query_by_label(field).is_none());
        let key = run.borrow().state.participants[0].key.clone();
        assert_eq!(run.borrow().effect_fields[&key].words, "Haste");
        run.borrow_mut().select(0);
        harness.run_steps(2);
        harness.get_by_label("Effect: words for this row").click();
        harness.run_steps(3);
        assert_eq!(
            harness.get_by_label(field).value().as_deref(),
            Some("Haste")
        );
    }
}
