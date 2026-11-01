//! The run's three views: setup, live and ended. Prominence follows the brief:
//! round and current turn, then every row's HP and conditions, then the actions.
//! Rows never move when the selection changes: the entry controls have a fixed dock.
use super::{Command, Panel, Participant, Phase, Run, display, mode_label};
use crate::{
    combat_sync::SyncState,
    design::{
        self, Action, Density, HpFonts, RunScale, Status, Tone, Type, colors, icons,
        reader as style,
    },
    encounter_settings::Grouping,
    library::view,
};
use eframe::egui::{
    self, Align, Align2, Galley, Key, Layout, Modifiers, Rect, Stroke, StrokeKind, pos2, vec2,
};
use std::sync::Arc;

const AMOUNT: &str = "Amount";
const SHEET_PAGES: [(usize, &str); 5] = [
    (0, "Overview"),
    (1, "Actions"),
    (2, "Spells"),
    (3, "Gear"),
    (4, "Features"),
];
/// Two lines of controls in a frame.
const DOCK_HEIGHT: f32 = 36.0 + 4.0 + 28.0 + 8.0;
const FOOTER_HEIGHT: f32 = 28.0 + 7.0;
const GUTTER: f32 = design::M;
/// Room for the note under a long roster, past the scroll area's clip margin.
const OVERFLOW_LINE: f32 = 22.0;
/// The name column a row needs, in units of its name size, to keep a state column too.
const NAME_ROOM: f32 = 11.0;
/// A panel this wide shows the reference and a tab side by side.
const PANEL_TWO_COLUMNS: f32 = 820.0;

pub(super) fn sync_status(state: SyncState) -> Status<'static> {
    let s = colors().sync;
    match state {
        SyncState::Off => Status {
            icon: icons::SYNC_OFF,
            word: "Sync off",
            color: s.off,
        },
        SyncState::FirstRead => Status {
            icon: icons::BUSY,
            word: "First read",
            color: s.first_read,
        },
        SyncState::Matching => Status {
            icon: icons::OK,
            word: "Synced",
            color: s.matching,
        },
        SyncState::Pending => Status {
            icon: icons::REFRESH,
            word: "Sync pending",
            color: s.pending,
        },
        SyncState::Conflict => Status {
            icon: icons::WARN,
            word: "Conflict",
            color: s.conflict,
        },
        SyncState::ReadOnly => Status {
            icon: icons::READ_ONLY,
            word: "Sync read-only",
            color: s.read_only,
        },
        SyncState::Error => Status {
            icon: icons::ERROR,
            word: "Sync error",
            color: s.error,
        },
    }
}

/// A panel beside the run keeps only a gutter from what it sits beside.
fn side_frame() -> egui::Frame {
    egui::Frame::new().inner_margin(egui::Margin {
        left: GUTTER as i8,
        ..egui::Margin::ZERO
    })
}

fn side_color(side: &str) -> egui::Color32 {
    let c = colors().side;
    match side {
        "party" => c.party,
        "enemies" => c.enemies,
        _ => c.other,
    }
}

/// The side's shape carries the meaning on a row. The dock and tooltip give the word.
fn side_mark(painter: &egui::Painter, center: egui::Pos2, side: &str) {
    let color = side_color(side);
    match side {
        "party" => {
            painter.rect_filled(Rect::from_center_size(center, vec2(8.0, 8.0)), 0.0, color);
        }
        "enemies" => {
            let points = vec![
                center + vec2(0.0, -5.0),
                center + vec2(5.0, 0.0),
                center + vec2(0.0, 5.0),
                center + vec2(-5.0, 0.0),
            ];
            painter.add(egui::Shape::convex_polygon(points, color, Stroke::NONE));
        }
        _ => {
            painter.circle_filled(center, 4.0, color);
        }
    }
}

/// The run's keyboard commands.
const KEYS: [(&str, &str); 8] = [
    ("N", "Next turn"),
    ("Enter", "Damage the selected row by the amount"),
    ("Shift+Enter", "Heal the selected row by the amount"),
    ("U or Ctrl+Z", "Undo"),
    ("Up, Down", "Select another row"),
    ("A", "Focus the amount"),
    ("C", "Conditions"),
    ("Ctrl+K", "Open or close Lookup. Escape also closes it"),
];

impl Run {
    /// Returns true only after the DM chooses to close the run.
    pub fn show(&mut self, ui: &mut egui::Ui) -> bool {
        let closed = match self.state.phase {
            Phase::Setup => self.setup_view(ui),
            Phase::Running => self.live_view(ui),
            Phase::Ended { .. } => self.ended_view(ui),
        };
        if closed {
            return true;
        }
        self.autosave();
        false
    }

    fn run_command(&mut self, command: Command) {
        let follow = matches!(command, Command::Next | Command::Begin);
        match self.apply(command) {
            Err(error) => self.error = Some(error),
            Ok(()) => {
                self.notice = None;
                self.condition.clear();
                self.note.clear();
                if follow {
                    self.focus_amount = true;
                    self.scroll_to_selected = true;
                    self.amount.clear();
                    if self.panel == Panel::Conditions {
                        self.panel = Panel::Reference;
                    }
                }
            }
        }
    }

    fn select(&mut self, index: usize) {
        if self.selected != index {
            self.selected = index;
            self.condition.clear();
            self.note.clear();
            self.amount.clear();
            self.scroll_to_selected = true;
            if self.panel == Panel::Conditions {
                self.panel = Panel::Reference;
            }
        }
        self.focus_amount = true;
    }

    fn step_selection(&mut self, delta: isize) {
        let order = &self.state.order;
        let Some(position) = order.iter().position(|i| *i == self.selected) else {
            return;
        };
        let next = position
            .saturating_add_signed(delta)
            .min(order.len().saturating_sub(1));
        self.select(order[next]);
    }

    /// Single-letter commands work when no text field has focus, and in the amount
    /// field, which accepts only digits.
    fn live_keys(&mut self, ui: &egui::Ui) {
        // A dialog above the run takes the keyboard.
        if ui.ctx().memory(|memory| memory.top_modal_layer()).is_some() {
            return;
        }
        let amount_id = design::focus_id(AMOUNT);
        let focused = ui.ctx().memory(|memory| memory.focused());
        if focused.is_some_and(|id| id != amount_id) && ui.ctx().wants_keyboard_input() {
            return;
        }
        let plain = Modifiers::NONE;
        let (next, undo, up, down, focus, enter, heal, conditions, typed) = ui.input_mut(|i| {
            let typed: String = if focused.is_none() {
                i.events
                    .iter()
                    .filter_map(|event| match event {
                        egui::Event::Text(text) => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<String>()
                    .chars()
                    .filter(char::is_ascii_digit)
                    .collect()
            } else {
                String::new()
            };
            (
                i.consume_key(plain, Key::N),
                i.consume_key(plain, Key::U) || i.consume_key(Modifiers::COMMAND, Key::Z),
                i.consume_key(plain, Key::ArrowUp),
                i.consume_key(plain, Key::ArrowDown),
                i.consume_key(plain, Key::A),
                focused == Some(amount_id) && i.consume_key(plain, Key::Enter),
                focused == Some(amount_id) && i.consume_key(Modifiers::SHIFT, Key::Enter),
                i.consume_key(plain, Key::C),
                typed,
            )
        });
        if next {
            self.run_command(Command::Next);
        }
        if undo {
            self.undo();
            self.focus_amount = true;
        }
        if up {
            self.step_selection(-1);
        }
        if down {
            self.step_selection(1);
        }
        if focus {
            self.focus_amount = true;
        }
        if !typed.is_empty() {
            self.amount.push_str(&typed);
            self.focus_amount = true;
        }
        if let Some(amount) = self.amount.parse::<i64>().ok().filter(|_| enter || heal) {
            let index = self.selected;
            self.run_command(if heal {
                Command::Heal(index, amount)
            } else {
                Command::Damage(index, amount)
            });
            self.amount.clear();
            self.focus_amount = true;
        }
        if conditions {
            self.open_conditions_tab();
        }
    }

    fn undo_action<'a>(&self, label: &'a str) -> Action<'a> {
        Action::secondary(label)
            .icon(icons::UNDO)
            .density(Density::Read)
            .key("U")
            .blocked_if(self.history.is_empty(), "Nothing to undo yet.")
    }

    fn run_title(&self, ui: &mut egui::Ui, what: &str) {
        ui.add(
            egui::Label::new(
                Type::Small
                    .rich(ui, format!("Run · {}", self.name))
                    .color(colors().muted),
            )
            .truncate(),
        );
        ui.add(egui::Label::new(Type::Title.rich(ui, what).color(colors().text)).truncate());
    }

    /// An error can be long and must be read in full. It is rare, so it may push the list.
    fn error_line(&mut self, ui: &mut egui::Ui) {
        if let Some(error) = self.error.clone() {
            ui.horizontal_top(|ui| {
                let width = ui.available_width() - 36.0;
                ui.scope(|ui| {
                    ui.set_max_width(width);
                    design::message(ui, Tone::Error, &error);
                });
                if design::icon_button(ui, icons::CLOSE, "Dismiss this message").clicked() {
                    self.error = None;
                }
            });
        }
    }

    // ---- Live -------------------------------------------------------------------

    fn panel_width(&self, ui: &egui::Ui) -> f32 {
        // The roster keeps its capped width. The panel uses the rest deliberately.
        design::side_panel_width(ui.ctx().content_rect().width())
            .max(ui.available_width() - design::RUN_LARGE.max_width - GUTTER)
    }

    fn live_view(&mut self, ui: &mut egui::Ui) -> bool {
        self.live_keys(ui);
        let mut closed = false;
        let width = self.panel_width(ui);
        if !self.hide_panel {
            egui::SidePanel::right("run_panel")
                .resizable(false)
                .exact_width(width)
                .frame(side_frame())
                .show_inside(ui, |ui| self.side_panel(ui));
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| {
                // A panel's own width is fixed, so the cap needs an inner layout.
                let size = vec2(
                    (ui.available_width() - GUTTER).min(design::RUN_LARGE.max_width),
                    ui.available_height(),
                );
                ui.allocate_ui_with_layout(size, Layout::top_down(Align::LEFT), |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    self.live_header(ui);
                    self.error_line(ui);
                    let room = ui.available_height() - DOCK_HEIGHT - FOOTER_HEIGHT;
                    ui.allocate_ui_with_layout(
                        vec2(ui.available_width(), room),
                        Layout::top_down(Align::LEFT),
                        |ui| {
                            ui.set_min_height(room);
                            self.roster(ui, room);
                        },
                    );
                    self.dock(ui);
                    closed = self.live_footer(ui);
                });
            });
        closed
    }

    fn live_header(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        let current = self.current();
        let name = current.map_or_else(String::new, |i| self.state.participants[i].name.clone());
        let next_up = self
            .next_up()
            .map(|i| self.state.participants[i].name.clone());
        let mut next_turn = false;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 36.0),
            Layout::right_to_left(Align::Center),
            |ui| {
                ui.set_min_height(36.0);
                next_turn = Action::primary("Next turn")
                    .icon(icons::NEXT)
                    .density(Density::Glance)
                    .key("N")
                    .show(ui)
                    .clicked();
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    let response = ui.label(
                        Type::Round
                            .rich(ui, format!("Round {}", self.state.round))
                            .color(c.text),
                    );
                    design::named(
                        ui,
                        &response,
                        &format!("Run · {} · Round {}", self.name, self.state.round),
                    );
                    ui.add_space(design::XS);
                    ui.add(
                        egui::Label::new(
                            Type::Title
                                .rich(ui, format!("{name}’s turn"))
                                .color(c.turn.current),
                        )
                        .truncate(),
                    );
                });
            },
        );
        ui.add_space(2.0);
        // Undo has this line's first claim, so it can always name the action in full.
        let undo_label = self.undo_label();
        let mut undo = false;
        let mut dismiss = false;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 28.0),
            Layout::right_to_left(Align::Center),
            |ui| {
                ui.set_min_height(28.0);
                undo = self
                    .undo_action(&undo_label)
                    .hint("Undo the last step (U)")
                    .show(ui)
                    .clicked();
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    let conflict = self.strip().conflict;
                    if let Some(notice) = &self.notice {
                        dismiss =
                            design::icon_button(ui, icons::CLOSE, "Dismiss this message").clicked();
                        ui.add(
                            egui::Label::new(Type::Small.rich(ui, notice).color(c.result.warning))
                                .truncate(),
                        );
                    } else if let Some(conflict) = conflict {
                        design::status(
                            ui,
                            Status {
                                icon: icons::WARN,
                                word: &format!("Sync conflict: {conflict}"),
                                color: c.sync.conflict,
                            },
                        );
                    } else if let Some(next) = &next_up {
                        ui.add(
                            egui::Label::new(
                                Type::Small
                                    .rich(ui, format!("Next: {next}"))
                                    .color(c.turn.next),
                            )
                            .truncate(),
                        );
                    }
                });
            },
        );
        ui.add_space(2.0);
        design::rule(ui);
        if dismiss {
            self.notice = None;
        }
        if undo {
            self.undo();
            self.focus_amount = true;
        }
        if next_turn {
            self.run_command(Command::Next);
        }
    }

    fn roster(&mut self, ui: &mut egui::Ui, room: f32) {
        let current = self.current();
        let next_up = self.next_up();
        let order = &self.state.order;
        let scale = RunScale::fit(room, ui.available_width(), order.len());
        // A list longer than its room keeps a line under it, to say what is out of view.
        let overflow = scale.row * order.len() as f32 > room;
        let list_height = if overflow { room - OVERFLOW_LINE } else { room };
        let mut clicked = None;
        // A changed window size must not leave the selected row out of view.
        let resized = (room - self.roster_height).abs() > 1.0;
        self.roster_height = room;
        let scroll = std::mem::take(&mut self.scroll_to_selected) || resized;
        let mut spans = Vec::with_capacity(order.len());
        let output = egui::ScrollArea::vertical()
            .id_salt("run_roster")
            .auto_shrink([false, false])
            .max_height(list_height)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_max_width(ui.available_width().min(scale.max_width));
                let origin = ui.cursor().min.y;
                for &index in order {
                    let selected = self.selected == index;
                    let p = &self.state.participants[index];
                    let sync = self
                        .syncs
                        .get(&index)
                        .filter(|sync| sync.enabled || sync.error.is_some())
                        .map(crate::combat_sync::CombatSync::state);
                    let is_current = current == Some(index);
                    let is_next = next_up == Some(index);
                    let name = row_name(p, is_current, is_next, sync);
                    let layout = RowLayout::new(ui, ui.available_width(), p, sync, scale);
                    let response = ui.push_id(&p.key, |ui| {
                        design::row(ui, layout.height(scale), selected, &name, |ui, rect| {
                            paint_row(ui, rect, p, is_current, is_next, sync, scale, &layout);
                        })
                    });
                    let response = response.inner.on_hover_text(&p.side);
                    let rect = response.rect;
                    spans.push((rect.top() - origin, rect.bottom() - origin, index));
                    if response.clicked() {
                        clicked = Some(index);
                    }
                    if scroll && selected {
                        // A row's whole height, past the area's clip margin at each end.
                        ui.scroll_to_rect(rect.expand2(vec2(0.0, 4.0)), None);
                    }
                }
            });
        self.overflow_note(ui, &output, &spans);
        if let Some(index) = clicked {
            self.select(index);
        }
    }

    /// In a long roster, say what is out of view, and whether any of it needs attention.
    fn overflow_note(
        &self,
        ui: &mut egui::Ui,
        output: &egui::scroll_area::ScrollAreaOutput<()>,
        spans: &[(f32, f32, usize)],
    ) {
        let (top, bottom) = (
            output.state.offset.y,
            output.state.offset.y + output.inner_rect.height(),
        );
        let hidden: Vec<usize> = spans
            .iter()
            .filter(|(start, end, _)| *end <= top + 1.0 || *start >= bottom - 1.0)
            .map(|(.., index)| *index)
            .collect();
        if hidden.is_empty() {
            return;
        }
        let hurt = hidden
            .iter()
            .filter(|index| {
                matches!(
                    design::hp_state(
                        self.state.participants[**index].hp,
                        self.state.participants[**index].max_hp
                    ),
                    design::HpState::Bloodied | design::HpState::Zero
                )
            })
            .count();
        let text = if hurt > 0 {
            format!(
                "{} rows out of view · {hurt} Bloodied or at 0 HP",
                hidden.len()
            )
        } else {
            format!("{} rows out of view", hidden.len())
        };
        ui.add_space(6.0);
        ui.add_sized(
            [ui.available_width(), OVERFLOW_LINE - 6.0],
            egui::Label::new(Type::Small.rich(ui, text).color(colors().muted)).truncate(),
        );
    }

    /// The entry controls have one fixed home, labelled with the row they act on, so
    /// no row moves when the selection changes. Typing needs no pointer travel at all.
    fn dock(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        let index = self.selected;
        let mut command = None;
        let mut open_conditions = false;
        let frame = egui::Frame::new()
            .fill(c.selected)
            .inner_margin(egui::Margin::symmetric(design::S as i8, design::XS as i8));
        frame.show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_height(DOCK_HEIGHT - design::S);
            ui.spacing_mut().item_spacing = vec2(design::XS, design::XS);
            let value = self.amount.parse::<i64>().ok();
            let empty = value.is_none().then_some("Type an amount first.");
            ui.horizontal(|ui| {
                self.amount_field(ui);
                if Action::secondary("Damage")
                    .density(Density::Glance)
                    .key("Enter")
                    .hint("Apply damage. Temporary HP absorbs it first.")
                    .blocked(empty)
                    .show(ui)
                    .clicked()
                {
                    command = value.map(|n| Command::Damage(index, n));
                }
                ui.add_space(design::M);
                if Action::quiet("Heal")
                    .density(Density::Glance)
                    .hint("Heal (Shift+Enter)")
                    .blocked(empty)
                    .show(ui)
                    .clicked()
                {
                    command = value.map(|n| Command::Heal(index, n));
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
                    command = value.map(|n| Command::TempHp(index, n));
                }
            });
            let p = &self.state.participants[index];
            ui.horizontal(|ui| {
                ui.set_min_height(28.0);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if Action::quiet("Absent")
                        .icon(icons::ABSENT)
                        .density(Density::Read)
                        .hint("Skip this participant's turns until marked present")
                        .selected(!p.present)
                        .show(ui)
                        .clicked()
                    {
                        command = Some(Command::Presence(index, !p.present));
                    }
                    if Action::quiet("Conc")
                        .icon(icons::CONCENTRATING)
                        .density(Density::Read)
                        .name("Concentrating")
                        .hint("Mark concentration. It records your ruling only.")
                        .selected(p.concentration)
                        .show(ui)
                        .clicked()
                    {
                        command = Some(Command::Concentration(index, !p.concentration));
                    }
                    if Action::quiet("Conditions")
                        .icon(icons::NEXT)
                        .density(Density::Read)
                        .name("Conditions for this participant")
                        .key("C")
                        .show(ui)
                        .clicked()
                    {
                        open_conditions = true;
                    }
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        ui.add(
                            egui::Label::new(Type::Heading.rich(ui, &p.name).color(c.text))
                                .truncate(),
                        );
                    });
                });
            });
        });
        if open_conditions {
            self.open_conditions_tab();
        }
        if let Some(command) = command {
            let applied = matches!(
                command,
                Command::Damage(..) | Command::Heal(..) | Command::TempHp(..)
            );
            self.run_command(command);
            if applied {
                self.amount.clear();
                self.focus_amount = true;
            }
        }
    }

    fn open_conditions_tab(&mut self) {
        self.panel = Panel::Conditions;
        self.close_lookup = true;
    }

    fn amount_field(&mut self, ui: &mut egui::Ui) {
        let response = design::number_field(
            ui,
            AMOUNT,
            &mut self.amount,
            AMOUNT,
            84.0,
            Density::Glance.control(),
        );
        self.amount.retain(|d| d.is_ascii_digit());
        if std::mem::take(&mut self.focus_amount) {
            response.request_focus();
        }
    }

    /// The official conditions, one click each. A second click removes one. This sits
    /// beside the roster and needs no typing, so a mistyped name cannot lose its sync.
    fn conditions_tab(&mut self, ui: &mut egui::Ui, index: usize) {
        let mut command = None;
        let p = &self.state.participants[index];
        egui::ScrollArea::vertical()
            .id_salt("run_conditions")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = vec2(design::XS, design::XS);
                let columns = if ui.available_width() >= 400.0 { 3 } else { 2 };
                egui::Grid::new("official_conditions")
                    .num_columns(columns)
                    .min_col_width((ui.available_width() / columns as f32 - design::XS).max(90.0))
                    .spacing([design::XS, design::XS])
                    .show(ui, |ui| {
                        for (position, (_, name)) in
                            crate::combat_sync::CONDITIONS.iter().enumerate()
                        {
                            let found = p.conditions.iter().position(|c| {
                                c.eq_ignore_ascii_case(name)
                                    || (*name == "Exhaustion" && c.starts_with("Exhaustion"))
                            });
                            if Action::quiet(name)
                                .icon(if found.is_some() {
                                    icons::OK
                                } else {
                                    icons::ADD
                                })
                                .density(Density::Read)
                                .selected(found.is_some())
                                .show(ui)
                                .clicked()
                            {
                                command = Some(found.map_or_else(
                                    || Command::Condition(index, (*name).to_owned()),
                                    |i| Command::RemoveCondition(index, i),
                                ));
                            }
                            if position % columns == columns - 1 {
                                ui.end_row();
                            }
                        }
                    });
                ui.add_space(design::S);
                let official = |c: &String| super::recognized_condition(c).is_some();
                let custom: Vec<_> = p
                    .conditions
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| !official(c))
                    .collect();
                if !custom.is_empty() {
                    design::group_label(ui, "Other effects");
                    for (i, condition) in custom {
                        ui.horizontal(|ui| {
                            if design::icon_button(ui, icons::CLOSE, &format!("Remove {condition}"))
                                .clicked()
                            {
                                command = Some(Command::RemoveCondition(index, i));
                            }
                            ui.add(
                                egui::Label::new(
                                    Type::Body.rich(ui, condition).color(colors().text),
                                )
                                .wrap(),
                            );
                        });
                    }
                }
                ui.horizontal(|ui| {
                    let response = design::text_input(
                        ui,
                        "Other effect",
                        &mut self.condition,
                        "Other effect…",
                        (ui.available_width() - 80.0).max(100.0),
                    );
                    let enter = response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
                    if (Action::secondary("Add")
                        .density(Density::Read)
                        .blocked_if(self.condition.trim().is_empty(), "Type an effect first.")
                        .show(ui)
                        .clicked()
                        || enter)
                        && !self.condition.trim().is_empty()
                    {
                        command = Some(Command::Condition(index, self.condition.clone()));
                    }
                });
                ui.add(
                    egui::Label::new(
                        Type::Small
                            .rich(ui, "These record your ruling. No number changes by itself. Other effects stay local and do not sync.")
                            .color(colors().muted),
                    )
                    .wrap(),
                );
            });
        if let Some(command) = command {
            self.run_command(command);
        }
    }

    /// Sync and the closing controls. These are kept far from Next turn.
    fn live_footer(&mut self, ui: &mut egui::Ui) -> bool {
        let mut closed = false;
        ui.add_space(design::XS);
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Read.control());
            if self.ending {
                design::text_input(ui, "Outcome", &mut self.outcome, "Outcome", 120.0);
                if Action::destructive("End encounter")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    let outcome = if self.outcome.trim().is_empty() {
                        "Undecided".to_owned()
                    } else {
                        self.outcome.trim().to_owned()
                    };
                    self.ending = false;
                    self.run_command(Command::End(outcome));
                }
                if Action::quiet("Keep playing")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.ending = false;
                }
                return;
            }
            if self.closing {
                closed = self.close_confirmation(ui, "The run stays saved. Undo and sync stop.");
                return;
            }
            let pending = self
                .syncs
                .values()
                .any(|s| s.enabled && (s.busy() || s.dirty()));
            // Five actions share the narrowest row. None of them may lose a letter.
            ui.spacing_mut().item_spacing.x = 0.0;
            if Action::quiet("Close run")
                .density(Density::Read)
                .blocked_if(
                    pending,
                    "Wait for pending sync changes, resolve conflicts, or turn off combat sync before closing.",
                )
                .show(ui)
                .clicked()
            {
                self.closing = true;
            }
            if Action::quiet("End encounter…")
                .density(Density::Read)
                .hint("Record an outcome. You can undo this.")
                .show(ui)
                .clicked()
            {
                self.ending = true;
            }
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                let status = if !self.dirty {
                    Status {
                        icon: icons::OK,
                        word: "Saved",
                        color: colors().document.saved,
                    }
                } else {
                    Status {
                        icon: icons::DRAFT,
                        word: "Not saved",
                        color: colors().document.unsaved,
                    }
                };
                let path = self.path().map(|path| path.display().to_string());
                let response = design::status(ui, status);
                if let Some(path) = path {
                    response.on_hover_text(path);
                }
                if Action::quiet("Keys")
                    .icon(icons::KEYS)
                    .density(Density::Read)
                    .name("Keyboard commands")
                    .selected(self.panel == Panel::Keys)
                    .show(ui)
                    .clicked()
                {
                    self.panel = if self.panel == Panel::Keys {
                        Panel::Reference
                    } else {
                        Panel::Keys
                    };
                    self.close_lookup = true;
                }
                self.sync_party_button(ui);
            });
        });
        closed
    }

    /// The second step of closing. Returns true when the run has closed.
    fn close_confirmation(&mut self, ui: &mut egui::Ui, note: &str) -> bool {
        ui.label(Type::Small.rich(ui, note).color(colors().muted));
        if Action::quiet("Keep running")
            .density(Density::Read)
            .show(ui)
            .clicked()
        {
            self.closing = false;
        }
        Action::destructive("Close now")
            .density(Density::Read)
            .show(ui)
            .clicked()
            && self.close_run()
    }

    fn close_run(&mut self) -> bool {
        match self.save() {
            Ok(()) => {
                self.closing = false;
                self.confirm_close = true;
                true
            }
            Err(error) => {
                self.error = Some(error);
                false
            }
        }
    }

    /// One action for the whole party (T3.3). The run works fully without it.
    fn sync_party_button(&mut self, ui: &mut egui::Ui) {
        let linked = self.linked_participants();
        if linked.is_empty() {
            return;
        }
        let all_on = linked
            .iter()
            .all(|i| self.syncs.get(i).is_some_and(|s| s.enabled));
        let simulated = self.simulate_connection;
        if Action::quiet(if all_on { "Stop sync" } else { "Sync party" })
            .icon(icons::SYNC_ON)
            .density(Density::Read)
            .name(if all_on {
                "Stop party sync"
            } else {
                "Sync party"
            })
            .hint("Read and update every linked character on D&D Beyond")
            .blocked_if(
                !self.connected && !simulated,
                "Connect to D&D Beyond in Settings to sync.",
            )
            .show(ui)
            .clicked()
        {
            for index in linked {
                let sync = self.syncs.entry(index).or_default();
                if all_on {
                    sync.stop();
                } else {
                    sync.start();
                    if simulated {
                        sync.keep_without_connection();
                    }
                }
            }
        }
    }

    // ---- Side panel -------------------------------------------------------------

    fn side_panel(&mut self, ui: &mut egui::Ui) {
        let index = self.selected;
        let p = &self.state.participants[index];
        let (hp, successes, failures) = (p.hp, p.successes, p.failures);
        let is_sheet = p.definition.fields.contains_key("splat_external");
        ui.add(egui::Label::new(Type::Title.rich(ui, &p.name).color(colors().text)).truncate());
        if hp == Some(0) {
            self.death_saves(ui, index, successes, failures);
        }
        if let Some(sync) = self
            .syncs
            .get_mut(&index)
            .filter(|s| s.state() == SyncState::Conflict)
        {
            sync.conflicts(ui);
        }
        let reference = if is_sheet { "Sheet" } else { "Stat block" };
        if ui.available_width() >= PANEL_TWO_COLUMNS {
            // Room for both: the reference stays open, with the DM's records beside it.
            let tab = if self.panel == Panel::Reference {
                Panel::Log
            } else {
                self.panel
            };
            let mut picked = tab;
            let column = (ui.available_width() - design::XL) / 2.0;
            let height = ui.available_height();
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = design::XL;
                ui.allocate_ui_with_layout(
                    vec2(column, height),
                    Layout::top_down(Align::LEFT),
                    |ui| {
                        ui.set_min_height(height);
                        ui.set_max_width(column);
                        design::group_label(ui, reference);
                        design::rule(ui);
                        self.reference_tab(ui, index);
                    },
                );
                ui.allocate_ui_with_layout(
                    vec2(column, height),
                    Layout::top_down(Align::LEFT),
                    |ui| {
                        ui.set_min_height(height);
                        ui.set_max_width(column);
                        design::segmented(
                            ui,
                            &mut picked,
                            &[
                                (Panel::Conditions, "Conditions"),
                                (Panel::Record, "Record"),
                                (Panel::Log, "Log"),
                                (Panel::Encounter, "Notes"),
                            ],
                        );
                        design::rule(ui);
                        self.panel_body(ui, index, picked);
                    },
                );
            });
            if picked != tab {
                self.panel = picked;
            }
        } else {
            design::segmented(
                ui,
                &mut self.panel,
                &[
                    (Panel::Reference, reference),
                    (Panel::Conditions, "Conditions"),
                    (Panel::Record, "Record"),
                    (Panel::Log, "Log"),
                    (Panel::Encounter, "Notes"),
                ],
            );
            design::rule(ui);
            self.panel_body(ui, index, self.panel);
        }
    }

    fn panel_body(&mut self, ui: &mut egui::Ui, index: usize, panel: Panel) {
        match panel {
            Panel::Reference => self.reference_tab(ui, index),
            Panel::Conditions => self.conditions_tab(ui, index),
            Panel::Record => self.record_tab(ui, index),
            Panel::Log => self.log_tab(ui),
            Panel::Keys => Self::keys_tab(ui),
            Panel::Encounter => self.notes_tab(ui),
        }
    }

    fn notes_tab(&self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .id_salt("run_encounter_notes")
            .auto_shrink([false, false])
            .show(ui, |ui| style::markdown(ui, &self.setup_details, false));
    }

    fn keys_tab(ui: &mut egui::Ui) {
        let c = colors();
        ui.add_space(design::XS);
        design::group_label(ui, "Keyboard commands");
        for (key, what) in KEYS {
            ui.horizontal_top(|ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(96.0, 20.0), egui::Sense::hover());
                ui.painter().text(
                    rect.left_top(),
                    Align2::LEFT_TOP,
                    key,
                    Type::Heading.font(ui),
                    c.text,
                );
                ui.add(egui::Label::new(Type::Body.rich(ui, what).color(c.muted)).wrap());
            });
        }
        ui.add_space(design::XS);
        ui.add(
            egui::Label::new(
                Type::Small
                    .rich(
                        ui,
                        "Letters work when the amount has focus, or nothing has.",
                    )
                    .color(c.muted),
            )
            .wrap(),
        );
    }

    /// Recording only. Colour says nothing here, because nothing is wrong.
    fn death_saves(&mut self, ui: &mut egui::Ui, index: usize, successes: u8, failures: u8) {
        let c = colors();
        let mut command = None;
        ui.horizontal(|ui| {
            ui.label(Type::Heading.rich(ui, "Death saves").color(c.text));
            ui.label(
                Type::Body
                    .rich(ui, format!("{successes} successes · {failures} failures"))
                    .color(c.muted),
            );
        });
        ui.horizontal(|ui| {
            if Action::quiet("+ Success")
                .name("Add a death save success")
                .density(Density::Read)
                .blocked_if(successes >= 3, "Three successes are recorded.")
                .show(ui)
                .clicked()
            {
                command = Some(Command::DeathSaves(index, successes + 1, failures));
            }
            if Action::quiet("+ Failure")
                .name("Add a death save failure")
                .density(Density::Read)
                .blocked_if(failures >= 3, "Three failures are recorded.")
                .show(ui)
                .clicked()
            {
                command = Some(Command::DeathSaves(index, successes, failures + 1));
            }
        });
        if let Some(command) = command {
            self.run_command(command);
        }
    }

    fn reference_tab(&mut self, ui: &mut egui::Ui, index: usize) {
        let p = &self.state.participants[index];
        let definition = &p.definition;
        // A linked character shows the same sheet as the Party page, with its age.
        if let Some(record) = self
            .sheet
            .as_ref()
            .filter(|_| self.sheet_for == Some(index))
        {
            let live = crate::party::LiveValues {
                hp: p.hp,
                max_hp: p.max_hp,
                temporary_hp: p.temporary_hp,
                conditions: p.conditions.clone(),
            };
            design::status(
                ui,
                crate::party::freshness(record, self.connected).as_status(),
            );
            design::segmented(ui, &mut self.sheet_page, &SHEET_PAGES);
            let page = self.sheet_page;
            egui::ScrollArea::vertical()
                .id_salt(("run_sheet", definition.id, page))
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    crate::party::sheet_page(ui, record, page, "", Some(&live));
                });
            return;
        }
        let mut link = None;
        if definition.fields.contains_key("splat_external") {
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(
                            ui,
                            "Saved when this run began. The roster has the current values.",
                        )
                        .color(colors().document.read_only),
                )
                .wrap(),
            );
        }
        egui::ScrollArea::vertical()
            .id_salt(("run_reference", definition.id))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let links: &dyn view::Links = match &self.content {
                    Some(index) => index.as_ref(),
                    None => &view::NoLinks,
                };
                link = view::show(
                    ui,
                    &definition.name,
                    Some(definition),
                    &definition.body,
                    &view::Origin {
                        path: &definition.path,
                        owner: "Starting snapshot, saved with this run",
                    },
                    false,
                    links,
                );
            });
        if link.is_some() {
            self.lookup_request = link;
        }
    }

    fn record_tab(&mut self, ui: &mut egui::Ui, index: usize) {
        let mut command = None;
        egui::ScrollArea::vertical()
            .id_salt("run_record")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = design::S;
                let p = &self.state.participants[index];
                let mut hp = p.hp;
                let mut max = p.max_hp;
                design::group_label(ui, "Correct HP");
                ui.horizontal_wrapped(|ui| {
                    optional_number(ui, "Current HP", &mut hp, 0..=1_000_000);
                    optional_number(ui, "Maximum HP", &mut max, 1..=1_000_000);
                });
                if hp != p.hp || max != p.max_hp {
                    command = Some(Command::Health(index, hp, max));
                }
                if self.state.phase == Phase::Setup {
                    let mut bonus = p.initiative_bonus;
                    optional_number(ui, "Initiative bonus", &mut bonus, -1000..=1000);
                    if bonus != p.initiative_bonus {
                        command = Some(Command::Bonus(index, bonus));
                    }
                } else {
                    ui.label(
                        Type::Small
                            .rich(ui, "Initiative is fixed once combat begins.")
                            .color(colors().muted),
                    );
                }
                design::group_label(ui, "Death saves (manual)");
                ui.horizontal(|ui| {
                    let (mut successes, mut failures) = (p.successes, p.failures);
                    ui.label("Successes");
                    let a = ui.add(egui::DragValue::new(&mut successes).range(0..=3));
                    design::named(ui, &a, "Death save successes");
                    ui.label("Failures");
                    let b = ui.add(egui::DragValue::new(&mut failures).range(0..=3));
                    design::named(ui, &b, "Death save failures");
                    if successes != p.successes || failures != p.failures {
                        command = Some(Command::DeathSaves(index, successes, failures));
                    }
                });
                ui.add(
                    egui::Label::new(
                        Type::Small
                            .rich(
                                ui,
                                "You resolve conditions, Concentration, death saves and recovery. Zero HP does not remove a turn.",
                            )
                            .color(colors().muted),
                    )
                    .wrap(),
                );
                design::group_label(ui, "DM notes");
                ui.horizontal(|ui| {
                    design::text_input(
                        ui,
                        "DM note",
                        &mut self.note,
                        "Resources, targets, rulings…",
                        (ui.available_width() - 90.0).max(120.0),
                    );
                    if Action::secondary("Record")
                        .name("Record note")
                        .density(Density::Read)
                        .blocked_if(self.note.trim().is_empty(), "Type a note first.")
                        .show(ui)
                        .clicked()
                    {
                        command = Some(Command::Note(index, self.note.clone()));
                    }
                });
                if !p.notes.is_empty() {
                    ui.add(
                        egui::Label::new(Type::Body.rich(ui, &p.notes).color(colors().text)).wrap(),
                    );
                }
                if p.definition.fields.contains_key("splat_external") {
                    design::group_label(ui, "D&D Beyond combat sync");
                    ui.push_id(index, |ui| self.syncs.entry(index).or_default().show(ui));
                }
            });
        if let Some(command) = command {
            self.run_command(command);
        }
    }

    fn log_tab(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        if self.state.phase == Phase::Running {
            let mut roll = false;
            ui.horizontal(|ui| {
                design::text_input(ui, "Dice", &mut self.dice, "1d20", 72.0);
                roll = Action::secondary("Roll")
                    .icon(icons::DICE)
                    .name("Roll dice")
                    .density(Density::Read)
                    .hint("Roll this expression. The result goes in the log.")
                    .show(ui)
                    .clicked();
            });
            if roll {
                let dice = self.dice.clone();
                self.run_command(Command::Roll(dice));
            }
        }
        egui::ScrollArea::vertical()
            .id_salt("run_log")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for line in self.state.log.iter().rev() {
                    ui.add(egui::Label::new(Type::Small.rich(ui, line).color(c.text)).wrap());
                }
            });
    }

    // ---- Setup ------------------------------------------------------------------

    fn setup_view(&mut self, ui: &mut egui::Ui) -> bool {
        let mut closed = false;
        let width = design::side_panel_width(ui.ctx().content_rect().width());
        if !self.hide_panel {
            egui::SidePanel::right("run_setup_panel")
                .resizable(false)
                .exact_width(width)
                .frame(side_frame())
                .show_inside(ui, |ui| self.setup_panel(ui));
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| {
                let size = vec2(
                    (ui.available_width() - GUTTER).min(design::RUN_LARGE.max_width),
                    ui.available_height(),
                );
                ui.allocate_ui_with_layout(size, Layout::top_down(Align::LEFT), |ui| {
                    closed = self.setup_body(ui);
                });
            });
        closed
    }

    fn setup_body(&mut self, ui: &mut egui::Ui) -> bool {
        let mut closed = false;
        let missing_initiative = self
            .state
            .participants
            .iter()
            .filter(|p| p.initiative.is_none())
            .count();
        let missing_hp = self
            .state
            .participants
            .iter()
            .filter(|p| !matches!((p.hp, p.max_hp), (Some(hp), Some(max)) if max > 0 && hp >= 0 && hp <= max))
            .count();
        let blocked = if missing_hp > 0 {
            Some(format!(
                "{missing_hp} need valid HP. Open Record to set it."
            ))
        } else if missing_initiative > 0 {
            Some(format!("{missing_initiative} need initiative."))
        } else {
            None
        };
        let mut command = None;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 44.0),
            Layout::right_to_left(Align::Center),
            |ui| {
                ui.set_min_height(44.0);
                if Action::primary("Begin combat")
                    .density(Density::Glance)
                    .blocked(blocked.as_deref())
                    .show(ui)
                    .clicked()
                {
                    command = Some(Command::Begin);
                }
                if Action::secondary("Roll NPC initiative")
                    .icon(icons::DICE)
                    .density(Density::Glance)
                    .show(ui)
                    .clicked()
                {
                    command = Some(Command::RollNpcs);
                }
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    self.run_title(ui, "Setup");
                });
            },
        );
        // Always one line, so the rows below never move during entry (T4.3).
        ui.label(match &blocked {
            Some(blocked) => Type::Small
                .rich(ui, blocked)
                .color(colors().document.unsaved),
            None => Type::Small
                .rich(ui, "Everyone is ready.")
                .color(colors().result.success),
        });
        design::rule(ui);
        self.error_line(ui);
        if std::mem::take(&mut self.focus_first_missing) {
            self.focus_field = self
                .state
                .participants
                .iter()
                .position(|p| p.initiative.is_none());
        }
        let room = ui.available_height() - FOOTER_HEIGHT - design::S;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), room),
            Layout::top_down(Align::LEFT),
            |ui| {
                ui.set_min_height(room);
                egui::ScrollArea::vertical()
                    .id_salt("run_setup_rows")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        // Roster order. Rows hold still while the DM types (T4.3).
                        for index in 0..self.state.participants.len() {
                            ui.push_id(index, |ui| {
                                if let Some(picked) = self.setup_row(ui, index) {
                                    command = Some(picked);
                                }
                            });
                        }
                    });
            },
        );
        design::rule(ui);
        ui.add_space(design::XS);
        ui.horizontal(|ui| {
            if self.closing {
                closed = self.close_confirmation(ui, "The run stays saved as Preparing.");
            } else {
                if Action::quiet("Close run")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.closing = true;
                }
                let label = self.undo_label();
                if self.undo_action(&label).show(ui).clicked() {
                    self.undo();
                }
            }
        });
        if let Some(command) = command {
            if matches!(command, Command::Begin) {
                self.panel = Panel::Reference;
            }
            self.run_command(command);
        }
        closed
    }

    /// Name and initiative first. Enter moves to the next missing total. The rarely
    /// used controls wait behind a row expander.
    fn setup_row(&mut self, ui: &mut egui::Ui, index: usize) -> Option<Command> {
        let mut command = None;
        let p = &self.state.participants[index];
        let detail = format!(
            "{} · HP {}/{} · AC {}",
            p.side,
            display(p.hp),
            display(p.max_hp),
            display(p.armor_class)
        );
        let selected = self.panel == Panel::Record && self.selected == index;
        let (name, npc, mode, bonus, initiative) = (
            &p.name,
            p.npc,
            p.initiative_mode,
            p.initiative_bonus,
            p.initiative,
        );
        let expanded = self.setup_expanded == Some(index);
        let row = Density::Scan.row() + design::XS;
        let mut enter = false;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = design::XS;
            let actions = 72.0 + 76.0 + 28.0 + 3.0 * design::XS;
            let name_width = (ui.available_width() - actions).max(110.0);
            let response = ui
                .allocate_ui(vec2(name_width, row), |ui| {
                    design::two_line_row(ui, row, selected, name, &detail)
                })
                .inner;
            if response.clicked() {
                self.selected = index;
            }
            let field_name = format!("Initiative for {name}");
            let text = self
                .setup_inputs
                .entry(index)
                .or_insert_with(|| initiative.map_or_else(String::new, |n| n.to_string()));
            let response =
                design::number_field(ui, &field_name, text, "Init", 72.0, Density::Scan.control());
            let parsed = text.parse::<i64>().ok();
            if response.changed() {
                if parsed != initiative {
                    command = Some(Command::Initiative(index, parsed));
                }
            } else if !response.has_focus() && parsed != initiative {
                // A roll or undo changed it. The field follows.
                *text = initiative.map_or_else(String::new, |n| n.to_string());
            }
            enter = response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
            if self.focus_field == Some(index) {
                self.focus_field = None;
                response.request_focus();
            }
            if Action::quiet("Roll")
                .icon(icons::DICE)
                .density(Density::Scan)
                .name(&format!("Roll for {name}"))
                .blocked_if(bonus.is_none(), "Set an initiative bonus, or type a total.")
                .show(ui)
                .clicked()
            {
                command = Some(Command::RollInitiative(index));
            }
            if design::icon_button(
                ui,
                icons::EXPAND,
                &format!("More options for {name}: NPC and roll mode"),
            )
            .clicked()
            {
                self.setup_expanded = if expanded { None } else { Some(index) };
            }
        });
        if enter {
            // The next row without a total, after this one, then from the top.
            let count = self.state.participants.len();
            self.focus_field = (1..count)
                .map(|step| (index + step) % count)
                .find(|i| self.state.participants[*i].initiative.is_none());
        }
        if expanded {
            ui.horizontal(|ui| {
                ui.add_space(design::S);
                let mut is_npc = npc;
                if ui
                    .checkbox(&mut is_npc, "NPC")
                    .on_hover_text("NPC initiative rolls include this participant")
                    .changed()
                {
                    command = Some(Command::Npc(index, is_npc));
                }
                let mut picked = mode;
                let combo = egui::ComboBox::from_id_salt("initiative_mode")
                    .icon(design::combo_icon)
                    .width(170.0)
                    .selected_text(mode_label(mode))
                    .show_ui(ui, |ui| {
                        for m in [-1, 0, 1] {
                            ui.selectable_value(&mut picked, m, mode_label(m));
                        }
                    });
                design::named_combo(ui, &combo, &format!("Initiative roll for {name}"));
                if picked != mode {
                    command = Some(Command::RollMode(index, picked));
                }
            });
        }
        design::rule(ui);
        command
    }

    fn setup_panel(&mut self, ui: &mut egui::Ui) {
        design::segmented(
            ui,
            &mut self.panel,
            &[
                (Panel::Reference, "Turn order"),
                (Panel::Record, "Record"),
                (Panel::Log, "Log"),
                (Panel::Encounter, "Notes"),
            ],
        );
        design::rule(ui);
        match self.panel {
            Panel::Reference | Panel::Conditions | Panel::Keys => self.turn_order_preview(ui),
            Panel::Record => {
                ui.label(
                    Type::Heading
                        .rich(ui, &self.state.participants[self.selected].name)
                        .color(colors().text),
                );
                self.record_tab(ui, self.selected);
            }
            Panel::Log => self.log_tab(ui),
            Panel::Encounter => self.notes_tab(ui),
        }
    }

    fn turn_order_preview(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        let mut command = None;
        ui.add(
            egui::Label::new(
                Type::Small
                    .rich(
                        ui,
                        format!(
                            "{} · {}",
                            self.initiative.grouping.label(),
                            if self.initiative.roster_ties {
                                "Roster order breaks ties"
                            } else {
                                "You resolve ties"
                            }
                        ),
                    )
                    .color(c.muted),
            )
            .wrap(),
        );
        if matches!(
            self.initiative.grouping,
            Grouping::BySide | Grouping::Explicit
        ) {
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(
                            ui,
                            "Each group uses the first NPC in roster order for its bonus and roll mode.",
                        )
                        .color(c.muted),
                )
                .wrap(),
            );
        }
        egui::ScrollArea::vertical()
            .id_salt("run_order_preview")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                let order = &self.state.order;
                for (position, index) in order.iter().enumerate() {
                    let p = &self.state.participants[*index];
                    let tied = |other: Option<&usize>| {
                        p.initiative.is_some()
                            && other.is_some_and(|o| {
                                self.state.participants[*o].initiative == p.initiative
                            })
                    };
                    let up = position > 0 && tied(order.get(position - 1));
                    let down = tied(order.get(position + 1));
                    ui.push_id(index, |ui| {
                        ui.horizontal(|ui| {
                            ui.set_min_height(Density::Read.control());
                            let (rect, _) = ui.allocate_exact_size(
                                vec2(26.0, Density::Read.control()),
                                egui::Sense::hover(),
                            );
                            design::number(
                                ui.painter(),
                                rect.right(),
                                rect.center().y,
                                &p.initiative.map_or_else(|| "—".into(), |n| n.to_string()),
                                &Type::Numeral.font(ui),
                                if p.initiative.is_some() {
                                    c.text
                                } else {
                                    c.faint
                                },
                            );
                            ui.add(
                                egui::Label::new(Type::Body.rich(ui, &p.name).color(c.text))
                                    .truncate(),
                            );
                            if !self.initiative.roster_ties && (up || down) {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    if down
                                        && design::icon_button(
                                            ui,
                                            icons::DOWN,
                                            &format!("Move {} after the tied participant", p.name),
                                        )
                                        .clicked()
                                    {
                                        command = Some(Command::Move(position, position + 1));
                                    }
                                    if up
                                        && design::icon_button(
                                            ui,
                                            icons::UP,
                                            &format!("Move {} before the tied participant", p.name),
                                        )
                                        .clicked()
                                    {
                                        command = Some(Command::Move(position, position - 1));
                                    }
                                    ui.label(Type::Small.rich(ui, "Tie").color(c.document.unsaved));
                                });
                            }
                        });
                    });
                    design::rule(ui);
                }
            });
        if let Some(command) = command {
            self.run_command(command);
        }
    }

    // ---- Ended ------------------------------------------------------------------

    fn ended_view(&mut self, ui: &mut egui::Ui) -> bool {
        let c = colors();
        let mut closed = false;
        let outcome = match &self.state.phase {
            Phase::Ended { outcome } => outcome.clone(),
            _ => String::new(),
        };
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 60.0),
            Layout::right_to_left(Align::Center),
            |ui| {
                ui.set_min_height(60.0);
                if Action::primary("Close run")
                    .density(Density::Glance)
                    .show(ui)
                    .clicked()
                {
                    closed = self.close_run();
                }
                if !self.history.is_empty() {
                    let label = self.undo_label();
                    if self
                        .undo_action(&label)
                        .density(Density::Glance)
                        .show(ui)
                        .clicked()
                    {
                        self.undo();
                    }
                } else {
                    ui.label(
                        Type::Small
                            .rich(ui, "Reopened, so the ending cannot be undone.")
                            .color(c.muted),
                    );
                }
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    self.run_title(ui, &format!("Ended: {outcome}"));
                    ui.label(
                        Type::Small
                            .rich(ui, format!("Ended manually in round {}", self.state.round))
                            .color(c.muted),
                    );
                });
            },
        );
        design::rule(ui);
        self.error_line(ui);
        let width = design::side_panel_width(ui.ctx().content_rect().width());
        egui::SidePanel::right("run_ended_log")
            .resizable(false)
            .exact_width(width)
            .frame(side_frame())
            .show_inside(ui, |ui| {
                design::group_label(ui, "Combat log");
                self.log_tab(ui);
            });
        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| {
                design::group_label(ui, "Final state");
                let scale = RunScale::fit(
                    ui.available_height(),
                    ui.available_width(),
                    self.state.order.len(),
                );
                egui::ScrollArea::vertical()
                    .id_salt("run_final_state")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.set_max_width(ui.available_width().min(scale.max_width));
                        for index in &self.state.order {
                            let p = &self.state.participants[*index];
                            let name = row_name(p, false, false, None);
                            let layout = RowLayout::new(ui, ui.available_width(), p, None, scale);
                            design::row(ui, layout.height(scale), false, &name, |ui, rect| {
                                paint_row(ui, rect, p, false, false, None, scale, &layout);
                            });
                        }
                    });
            });
        closed
    }
}

fn hp_word(p: &Participant) -> Option<(&'static str, &'static str, egui::Color32)> {
    let hp = colors().hp;
    match design::hp_state(p.hp, p.max_hp) {
        design::HpState::Zero => Some((icons::ZERO_HP, "0 HP", hp.zero)),
        design::HpState::Bloodied => Some((icons::BLOODIED, "Bloodied", hp.bloodied)),
        design::HpState::Unknown => Some((icons::WARN, "HP unknown", colors().muted)),
        _ => None,
    }
}

/// Everything on a row, for assistive technology and for the tests.
fn row_name(p: &Participant, current: bool, next: bool, sync: Option<SyncState>) -> String {
    let mut parts = vec![
        p.name.clone(),
        p.side.clone(),
        format!("HP {}/{}", display(p.hp), display(p.max_hp)),
        format!("Temp {}", p.temporary_hp),
        format!("AC {}", display(p.armor_class)),
        format!("Initiative {}", display(p.initiative)),
    ];
    if current {
        parts.push("Current turn".into());
    }
    if next {
        parts.push("Next up".into());
    }
    if !p.present {
        parts.push("Absent".into());
    }
    if let Some((_, word, _)) = hp_word(p) {
        parts.push(word.into());
    }
    if p.concentration {
        parts.push("Concentrating".into());
    }
    parts.extend(p.conditions.iter().cloned());
    if let Some(sync) = sync {
        parts.push(sync_status(sync).word.into());
    }
    parts.join(", ")
}

/// The fixed columns of a run row, and its second line, which grows rather than cuts.
struct RowLayout {
    width: f32,
    second_line: Option<Arc<Galley>>,
    /// The row has no room for a state column. Its state and sync words open the second
    /// line, so that the name stays whole.
    inline_state: bool,
    name_right: f32,
    ac_right: f32,
    temp_right: f32,
    hp_right: f32,
}

const INITIATIVE_WIDTH: f32 = 42.0;
const NAME_LEFT: f32 = INITIATIVE_WIDTH + 22.0;

impl RowLayout {
    fn new(
        ui: &egui::Ui,
        width: f32,
        p: &Participant,
        sync: Option<SyncState>,
        scale: RunScale,
    ) -> Self {
        let c = colors();
        // Fixed columns from the left of the numbers. A name stays near its HP.
        let hp_width = scale.hp * 1.9 + scale.hp_max * 2.0 + 4.0;
        let state_width = (scale.mark * 7.0).clamp(84.0, 110.0);
        let temp_width = scale.stat * 2.6 + 12.0;
        let ac_width = scale.stat * 1.5 + 22.0;
        let numbers = NAME_LEFT + ac_width + temp_width + hp_width + design::S;
        let inline_state = width - numbers - state_width < scale.name * NAME_ROOM;
        let name_room = (width - numbers - if inline_state { 0.0 } else { state_width }).max(90.0);
        let name_width = name_room.min(scale.name * 12.5);
        let name_right = NAME_LEFT + name_width;
        let ac_right = name_right + ac_width;
        let temp_right = ac_right + temp_width;
        let hp_right = temp_right + hp_width;

        let mut job = egui::text::LayoutJob::default();
        // The line wraps before the HP column, so that it never runs over the HP bar.
        job.wrap.max_width = (temp_right - NAME_LEFT).max(1.0);
        let font = Type::Body.sized(ui, scale.condition);
        let format = |color| egui::TextFormat {
            font_id: font.clone(),
            color,
            ..Default::default()
        };
        let mut first = true;
        let mut add = |text: &str, color: egui::Color32| {
            if !first {
                job.append(" · ", 0.0, format(c.muted));
            }
            first = false;
            job.append(text, 0.0, format(color));
        };
        if inline_state {
            if let Some((icon, word, color)) = hp_word(p) {
                add(&format!("{icon} {word}"), color);
            }
            if let Some(sync) = sync {
                let status = sync_status(sync);
                add(&format!("{} {}", status.icon, status.word), status.color);
            }
        }
        if !p.present {
            add(&format!("{} Absent", icons::ABSENT), c.muted);
        }
        if p.concentration {
            add(
                &format!("{} Conc", icons::CONCENTRATING),
                c.participant.concentrating,
            );
        }
        for condition in &p.conditions {
            add(condition, c.participant.condition);
        }
        let second_line = (!job.is_empty()).then(|| ui.fonts_mut(|fonts| fonts.layout_job(job)));
        Self {
            width,
            second_line,
            inline_state,
            name_right,
            ac_right,
            temp_right,
            hp_right,
        }
    }

    /// Rows in a list get taller, never shorter, when conditions need another line.
    fn height(&self, scale: RunScale) -> f32 {
        let lines = self.second_line.as_ref().map_or(1, |line| line.rows.len());
        scale.row + (lines.saturating_sub(1)) as f32 * (scale.condition + 3.0)
    }
}

#[allow(clippy::too_many_arguments)]
fn paint_row(
    ui: &egui::Ui,
    rect: Rect,
    p: &Participant,
    current: bool,
    next: bool,
    sync: Option<SyncState>,
    scale: RunScale,
    layout: &RowLayout,
) {
    let c = colors();
    let painter = ui.painter();
    let x = rect.left();
    let has_second_line = layout.second_line.is_some() || sync.is_some();
    let name_line = scale.name + 4.0;
    let second_height = layout
        .second_line
        .as_ref()
        .map_or(scale.condition + 2.0, |line| line.size().y);
    // One line sits in the middle. Two share the row, with the numbers on the first.
    let line_y = if has_second_line {
        rect.top() + (rect.height() - second_height - 3.0) / 2.0 - 1.0
    } else {
        rect.center().y - 1.0
    };
    // The current row uses a stronger fill, so its dim text is muted, never faint.
    let dim = if current {
        c.muted
    } else {
        c.participant.absent
    };
    let text = if p.present { c.text } else { dim };
    if current {
        painter.rect_filled(rect, 0.0, c.current_row);
        painter.rect_stroke(
            rect,
            2,
            Stroke::new(design::FOCUS_WIDTH, c.turn.current),
            StrokeKind::Inside,
        );
        painter.rect_filled(
            Rect::from_min_size(rect.min, vec2(6.0, rect.height())),
            0.0,
            c.turn.current,
        );
    }
    let numeral = Type::Numeral.sized(ui, scale.stat);
    let initiative_right = x + INITIATIVE_WIDTH - 4.0;
    let mark_y = rect.bottom() - scale.mark / 2.0 - 2.0;
    design::number(
        painter,
        initiative_right,
        if current || next {
            // Level with the name when the row has room, and always clear of the word.
            line_y.min(mark_y - (scale.mark + scale.stat) / 2.0)
        } else {
            rect.center().y - 1.0
        },
        &display(p.initiative),
        &numeral,
        if current { c.turn.current } else { c.muted },
    );
    if current || next {
        let (word, color) = if current {
            ("TURN", c.turn.current)
        } else {
            ("NEXT", c.turn.next)
        };
        painter.text(
            pos2(initiative_right, mark_y),
            Align2::RIGHT_CENTER,
            word,
            Type::Label.sized(ui, scale.mark),
            color,
        );
    }
    side_mark(painter, pos2(x + INITIATIVE_WIDTH + 8.0, line_y), &p.side);
    let name_left = x + NAME_LEFT;
    let name = design::one_line(
        ui,
        &p.name,
        Type::Name.sized(ui, scale.name),
        text,
        x + layout.name_right - name_left,
    );
    painter.galley(pos2(name_left, line_y - name_line / 2.0), name, text);
    // AC, then temporary HP, then HP, directly after the name.
    let ac_right = x + layout.ac_right;
    let left = design::number(
        painter,
        ac_right,
        line_y,
        &display(p.armor_class),
        &numeral,
        text,
    );
    painter.text(
        pos2(left - 4.0, line_y + 1.0),
        Align2::RIGHT_CENTER,
        "AC",
        Type::Small.font(ui),
        if current { c.muted } else { c.faint },
    );
    if p.temporary_hp > 0 {
        let left = design::number(
            painter,
            x + layout.temp_right - 4.0,
            line_y,
            &format!("+{}", p.temporary_hp),
            &numeral,
            c.hp.temp,
        );
        painter.text(
            pos2(left - 2.0, line_y),
            Align2::RIGHT_CENTER,
            icons::TEMP_HP,
            Type::Small.font(ui),
            c.hp.temp,
        );
    }
    let fonts = HpFonts {
        current: Type::NumeralLarge.sized(ui, scale.hp),
        max: Type::Numeral.sized(ui, scale.hp_max),
    };
    let hp_right = x + layout.hp_right;
    let hp_left = design::hp_readout(ui, painter, hp_right, line_y, p.hp, p.max_hp, &fonts);
    let bar = Rect::from_min_max(
        pos2(hp_left.min(hp_right - 60.0), rect.bottom() - 4.0),
        pos2(hp_right, rect.bottom() - 2.0),
    );
    design::hp_bar(painter, bar, p.hp, p.max_hp, p.temporary_hp);
    if let Some((icon, word, color)) = hp_word(p).filter(|_| !layout.inline_state) {
        let color = if current && word == "HP unknown" {
            c.muted
        } else {
            color
        };
        painter.text(
            pos2(hp_right + design::S, line_y),
            Align2::LEFT_CENTER,
            format!("{icon} {word}"),
            Type::Heading.sized(ui, scale.mark.max(12.0)),
            color,
        );
    }
    // Conditions have their own line, in their own colour, in full. Sync has its slot.
    let second_y = rect.bottom() - second_height - 4.0;
    if let Some(line) = &layout.second_line {
        painter.galley(
            pos2(name_left, second_y),
            line.clone(),
            c.participant.condition,
        );
    }
    if let Some(sync) = sync.filter(|_| !layout.inline_state) {
        let status = sync_status(sync);
        painter.text(
            pos2(x + layout.width - design::S, second_y + second_height / 2.0),
            Align2::RIGHT_CENTER,
            format!("{} {}", status.icon, status.word),
            Type::Small.font(ui),
            status.color,
        );
    }
    painter.hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(design::HAIRLINE_WIDTH, c.hairline),
    );
}

fn optional_number(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<i64>,
    range: std::ops::RangeInclusive<i64>,
) {
    let mut known = value.is_some();
    if ui.checkbox(&mut known, label).changed() {
        *value = known.then(|| *range.start().max(&0));
    }
    if let Some(n) = value {
        let response = ui.add(egui::DragValue::new(n).range(range));
        design::named(ui, &response, label);
    } else {
        ui.label(Type::Small.rich(ui, "Unknown").color(colors().muted));
    }
}
