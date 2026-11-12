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
                    self.target_words(ui);
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
                // SL1: in a narrow dock the dice total has a reserved place at the line's end,
                // under the dice and Roll, so it never joins the selected name.
                let result_slot = if narrow {
                    design::number_width(
                        ui,
                        &format!("{} 888", icons::DICE),
                        &Type::Strong.font(ui),
                    ) + gap
                } else {
                    0.0
                };
                // FK1: the name slot has one width for the whole fight: the widest roster name
                // in `Heading`, at most `12 × L`, so Conditions, Conc and Absent keep one x
                // whichever row is selected. Measured as the roster measures its names, each
                // frame through the galley cache; nothing is kept.
                let heading = Type::Heading.font(ui);
                let widest = self
                    .state
                    .participants
                    .iter()
                    .map(|p| design::number_width(ui, &p.name, &heading))
                    .fold(0.0_f32, f32::max);
                // Left to right: the name slot, then exactly `L`, then its controls. The
                // controls' width only limits how wide the slot may be.
                ui.horizontal(|ui| {
                    ui.set_min_height(height);
                    let room = ui.available_width() - controls - design::L - result_slot;
                    let slot = widest.min(12.0 * design::L).min(room).max(0.0);
                    let (role, shown) = dock_name_shown(ui, &p.name, slot);
                    ui.allocate_ui_with_layout(
                        vec2(slot, height),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.set_width(slot);
                            let response = ui
                                .label(role.rich(ui, shown).color(c.text))
                                .on_hover_text(&p.name);
                            design::named(ui, &response, &format!("Selected: {}", p.name));
                        },
                    );
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
                    if narrow {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if let Some((total, line)) = self.dock_roll() {
                                let response = ui
                                    .label(
                                        Type::Strong
                                            .rich(ui, format!("{} {total}", icons::DICE))
                                            .color(c.text),
                                    )
                                    .on_hover_text(&line);
                                design::named(ui, &response, &format!("Rolled {total}"));
                            }
                        });
                    }
                });
            });
        });
        if open_conditions {
            self.open_conditions_tab();
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

    /// In place of the one row's actions: how to give half. The choice is on each row, so that
    /// any number of targets stays in view. FK5: the words show the mark that the rows paint
    /// (`icons::OK`), and leave the count to the panel's hint, so it shows once.
    fn target_words(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_min_height(28.0);
            ui.add(
                egui::Label::new(Type::Small.rich(ui, target_hint()).color(colors().muted))
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

/// The dock's words while several rows are targets: the mark the rows paint, and what it does.
pub(super) fn target_hint() -> String {
    format!(
        "Press a target’s {} to give it half, rounded down.",
        icons::OK
    )
}
