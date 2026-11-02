//! The run's three views: setup, live and ended. Prominence follows the brief:
//! round and current turn, then every row's HP and conditions, then the actions.
//! Rows never move when the selection changes: the entry controls have a fixed dock.
use super::{
    Command, NOBODY_PRESENT, Panel, Participant, PartySync, Phase, Run, display, mode_label,
};
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
const SCROLL_BAR: f32 = 12.0;
/// A name-only row when the list has no other way to fit. Still a 24 px target and more.
const TIGHT_ROW: f32 = 25.0;
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
        SyncState::Stopped => Status {
            icon: icons::OFFLINE,
            word: "Sync stopped",
            color: colors().result.error,
        },
        SyncState::Error => Status {
            icon: icons::ERROR,
            word: "Sync error",
            color: s.error,
        },
    }
}

/// The roster keeps its capped width. The panel uses the rest deliberately.
fn panel_width(ui: &egui::Ui) -> f32 {
    design::side_panel_width(ui.ctx().content_rect().width())
        .max(ui.available_width() - design::RUN_LARGE.max_width - GUTTER)
}

/// A panel beside the run keeps only a gutter from what it sits beside.
fn side_frame() -> egui::Frame {
    egui::Frame::new().inner_margin(egui::Margin {
        left: GUTTER as i8,
        ..egui::Margin::ZERO
    })
}

/// The side's shape carries the meaning on a row. The dock and tooltip give the word.
fn side_mark(painter: &egui::Painter, center: egui::Pos2, side: &str, size: f32) {
    let c = colors().side;
    let half = size / 2.0;
    match side {
        "party" => {
            let square = Rect::from_center_size(center, vec2(size, size));
            painter.rect_filled(square, 0.0, c.party);
        }
        "enemies" => {
            let reach = half * 1.25;
            let points = vec![
                center + vec2(0.0, -reach),
                center + vec2(reach, 0.0),
                center + vec2(0.0, reach),
                center + vec2(-reach, 0.0),
            ];
            painter.add(egui::Shape::convex_polygon(points, c.enemies, Stroke::NONE));
        }
        _ => {
            painter.circle_filled(center, half, c.other);
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

    /// Escape backs out of a footer's second step, in every phase.
    fn back_out_on_escape(&mut self, ui: &egui::Ui) -> bool {
        let modal = ui.ctx().memory(|memory| memory.top_modal_layer()).is_some();
        if !modal
            && (self.closing || self.ending)
            && ui.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape))
        {
            self.closing = false;
            self.ending = false;
            return true;
        }
        false
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
        // With Lookup open, a letter is the next word of a search, never a command. A DM
        // typing "cone" must not advance the turn.
        if self.lookup_open && focused != Some(amount_id) {
            return;
        }
        if self.back_out_on_escape(ui) {
            return;
        }
        // Next turn puts the focus in Amount, and the DM may still be typing a search.
        let letters = !self.lookup_open;
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
            // Shift first. A plain shortcut also matches with Shift held, so the other
            // order turns every heal into damage.
            let heal = focused == Some(amount_id) && i.consume_key(Modifiers::SHIFT, Key::Enter);
            let enter = focused == Some(amount_id) && i.consume_key(plain, Key::Enter);
            // The other commands are the plain letter only, so a held Shift does nothing.
            if i.modifiers.shift || i.modifiers.alt {
                return (false, false, false, false, false, enter, heal, false, typed);
            }
            (
                letters && i.consume_key(plain, Key::N),
                (letters && i.consume_key(plain, Key::U))
                    || i.consume_key(Modifiers::COMMAND, Key::Z),
                i.consume_key(plain, Key::ArrowUp),
                i.consume_key(plain, Key::ArrowDown),
                letters && i.consume_key(plain, Key::A),
                enter,
                heal,
                letters && i.consume_key(plain, Key::C),
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
        let action = Action::secondary(label)
            .icon(icons::UNDO)
            .density(Density::Read);
        let action = if matches!(self.state.phase, Phase::Running) {
            action.key("U")
        } else {
            action
        };
        action.blocked_if(
            self.history.is_empty(),
            if self.reopened {
                "Undo does not survive a restart. Later steps can be undone."
            } else {
                "Nothing to undo yet."
            },
        )
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
        if self.has_disk_conflict() {
            self.disk_conflict_line(ui);
            return;
        }
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

    /// The run cannot save until the DM chooses, so this cannot be dismissed.
    fn disk_conflict_line(&mut self, ui: &mut egui::Ui) {
        design::message(ui, Tone::Error, super::DISK_CONFLICT);
        ui.add_space(design::XS);
        let mut outcome = None;
        ui.horizontal(|ui| {
            if Action::primary("Keep this run")
                .density(Density::Read)
                .hint("Replace the file on disk with this run")
                .show(ui)
                .clicked()
            {
                outcome = Some(self.keep_this_run());
            }
            if Action::secondary("Save as a new run")
                .density(Density::Read)
                .hint("Keep the changed file as it is, and save this run beside it")
                .show(ui)
                .clicked()
            {
                outcome = Some(self.save_as_new_run());
            }
        });
        if let Some(Err(error)) = outcome {
            self.error = Some(error);
        }
        ui.add_space(design::XS);
    }

    // ---- Live -------------------------------------------------------------------

    fn live_view(&mut self, ui: &mut egui::Ui) -> bool {
        self.live_keys(ui);
        let mut closed = false;
        let width = panel_width(ui);
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
                    // The rows' scale decides the header's size, and needs only a close
                    // idea of the room the header will leave.
                    let scale = RunScale::fit(
                        ui.available_height()
                            - header_height(design::RUN_REGULAR)
                            - 32.0
                            - DOCK_HEIGHT
                            - FOOTER_HEIGHT,
                        ui.available_width(),
                        self.state.order.len(),
                    );
                    self.live_header(ui, scale);
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

    /// The round and the turn are the strongest things on the page, above any row, and
    /// they grow with the rows. Next up is the only name that may be off the roster's view.
    fn live_header(&mut self, ui: &mut egui::Ui, scale: RunScale) {
        let c = colors();
        let strip = self.strip();
        let name = strip.current.as_deref().unwrap_or_default();
        let mut next_turn = false;
        let nobody_present = !self.state.participants.iter().any(|p| p.present);
        // As large as the largest HP, in the row's own face: nothing outweighs the header.
        let turn_size = scale.hp;
        let height = header_height(scale);
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), height),
            Layout::right_to_left(Align::Center),
            |ui| {
                ui.set_min_height(height);
                next_turn = Action::primary("Next turn")
                    .icon(icons::NEXT)
                    .density(Density::Glance)
                    .key("N")
                    .blocked(nobody_present.then_some(NOBODY_PRESENT))
                    .show(ui)
                    .clicked();
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    let response = ui.label(
                        Type::Round
                            .rich(ui, format!("Round {}", self.state.round))
                            .font(Type::Round.sized(ui, turn_size))
                            .color(c.text),
                    );
                    design::named(
                        ui,
                        &response,
                        &format!("Run · {} · Round {}", self.name, self.state.round),
                    );
                    ui.add_space(design::S);
                    // The first answer on the screen. It gives up size, then a word, before
                    // it gives up a letter of the name.
                    let room = ui.available_width() - 1.0;
                    let full = format!("{name}’s turn");
                    let least = Type::Name.font(ui).size.min(turn_size);
                    let fit = |text: &str| {
                        let width =
                            design::number_width(ui, text, &Type::Name.sized(ui, turn_size));
                        (turn_size * room / width).min(turn_size)
                    };
                    let (full_size, name_size) = (fit(&full), fit(name));
                    let (text, size) = if full_size >= least {
                        (full, full_size)
                    } else if name_size >= least {
                        (name.to_owned(), name_size)
                    } else {
                        // The row, the panel and the dock's hover have the whole name.
                        let font = Type::Name.sized(ui, least);
                        (design::words_that_fit(ui, name, &font, room), least)
                    };
                    ui.add(
                        egui::Label::new(
                            Type::Name
                                .rich(ui, text)
                                .font(Type::Name.sized(ui, size))
                                .color(c.turn.current),
                        )
                        .truncate(),
                    );
                });
            },
        );
        ui.add_space(6.0);
        // Undo has this line's first claim, so it can always name the action in full.
        let undo_label = self.undo_label();
        let mut undo = false;
        let mut dismiss = false;
        // What sync needs from the DM. These stay until the cause is gone. A stopped sync
        // comes first, because nothing else can be resolved without a connection.
        let sync_alert = if strip.sync_stopped {
            Some((
                format!("{} Sync stopped: connect in Settings", icons::OFFLINE),
                format!("{} Sync stopped", icons::OFFLINE),
                c.result.error,
            ))
        } else if let Some(name) = &strip.conflict {
            Some((
                format!("{} Sync conflict: {name}", icons::WARN),
                format!("{} Conflict: {name}", icons::WARN),
                c.sync.conflict,
            ))
        } else {
            strip.sync_error.as_ref().map(|name| {
                (
                    format!("{} Sync error: {name}", icons::ERROR),
                    format!("{} Error: {name}", icons::ERROR),
                    c.result.error,
                )
            })
        };
        // A notice too long for this line takes its own, below, in full.
        let notice_font = Type::Small.font(ui);
        let undo_room = design::number_width(ui, &undo_label, &Type::Strong.font(ui)) + 72.0;
        let notice_fits = self.notice.as_ref().is_none_or(|notice| {
            design::number_width(ui, notice, &notice_font) + undo_room + 32.0 < ui.available_width()
        });
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
                    if let Some((text, short, color)) = &sync_alert {
                        // Next to a long Undo it gives up size, then the word Sync, never the name.
                        let room = ui.available_width();
                        let (role, text) =
                            if design::number_width(ui, text, &Type::Strong.font(ui)) < room {
                                (Type::Strong, text)
                            } else if design::number_width(ui, text, &Type::Small.font(ui)) < room {
                                (Type::Small, text)
                            } else {
                                (Type::Small, short)
                            };
                        ui.label(role.rich(ui, text.as_str()).color(*color));
                    } else if let Some(notice) = self.notice.as_ref().filter(|_| notice_fits) {
                        dismiss =
                            design::icon_button(ui, icons::CLOSE, "Dismiss this message").clicked();
                        ui.label(Type::Small.rich(ui, notice).color(c.result.warning));
                    } else if nobody_present {
                        // Its first sentence, beside a long Undo. Next turn's hover has the rest.
                        let first = NOBODY_PRESENT
                            .split_inclusive('.')
                            .next()
                            .unwrap_or_default();
                        ui.label(Type::Small.rich(ui, first).color(c.result.warning));
                    } else if let Some(next) = &strip.next {
                        let text = format!("Next: {next}");
                        let role = if design::number_width(ui, &text, &Type::Body.font(ui))
                            < ui.available_width()
                        {
                            Type::Body
                        } else {
                            Type::Small
                        };
                        // The row marked NEXT has the whole of a name too long for this line.
                        let text =
                            design::words_that_fit(ui, &text, &role.font(ui), ui.available_width());
                        ui.label(role.rich(ui, text).color(c.turn.next));
                    }
                });
            },
        );
        ui.add_space(2.0);
        // A conflict outranks a notice, and a long notice has no room above.
        if let Some(notice) = self
            .notice
            .as_ref()
            .filter(|_| sync_alert.is_some() || !notice_fits)
        {
            ui.horizontal(|ui| {
                dismiss = design::icon_button(ui, icons::CLOSE, "Dismiss this message").clicked();
                ui.add(
                    egui::Label::new(Type::Small.rich(ui, notice).color(c.result.warning)).wrap(),
                );
            });
        }
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

    /// The sync state a row shows, if this participant has one.
    pub(super) fn sync_word(&self, index: usize) -> Option<SyncState> {
        self.syncs
            .get(&index)
            .filter(|sync| sync.enabled || sync.error.is_some())
            .map(crate::combat_sync::CombatSync::state)
    }

    /// Every row's layout at this width. Rows with only a name can give up height when
    /// the list would otherwise scroll, so a wrapped condition costs no other row its view.
    fn roster_layouts(
        &self,
        ui: &egui::Ui,
        width: f32,
        scale: RunScale,
        fit: RowFit,
    ) -> Vec<RowLayout> {
        let font = Type::Name.sized(ui, scale.name);
        let longest = self.state.participants.iter().fold(0.0_f32, |longest, p| {
            longest.max(design::number_width(ui, &p.name, &font) + 1.0)
        });
        self.state
            .order
            .iter()
            .map(|&index| {
                let p = &self.state.participants[index];
                RowLayout::new(ui, width, p, self.sync_word(index), scale, fit, longest)
            })
            .collect()
    }

    fn roster(&mut self, ui: &mut egui::Ui, room: f32) {
        let current = self.current();
        let next_up = self.next_up();
        let count = self.state.order.len();
        let mut scale = RunScale::fit(room, ui.available_width(), count);
        let width = ui.available_width();
        let total = |layouts: &[RowLayout], scale: RunScale| -> f32 {
            layouts.iter().map(|l| l.height(scale)).sum()
        };
        let mut layouts = self.roster_layouts(ui, width, scale, RowFit::Natural);
        let natural = total(&layouts, scale);
        if natural <= room {
            // Spare room goes to the rows, so a short list leaves no gap under it.
            let share = (room - natural - 2.0).max(0.0) / count.max(1) as f32;
            scale.row += share.min(scale.row * 0.4);
        } else {
            layouts = self.roster_layouts(ui, width, scale, RowFit::Compressed);
            // What is left goes back to the rows, shortest first, so the pitch evens out.
            let heights: Vec<f32> = layouts.iter().map(|l| l.height(scale)).collect();
            let even = even_height(&heights, room - 2.0, scale.row);
            layouts.iter_mut().for_each(|l| l.even = even);
        }
        // A list longer than its room keeps a line under it, to say what is out of view.
        let overflow = total(&layouts, scale) > room;
        if overflow {
            // Too tall even at its tightest. It scrolls, at that tight pitch.
            layouts = self.roster_layouts(ui, width - SCROLL_BAR, scale, RowFit::Compressed);
        }
        let list_height = if overflow { room - OVERFLOW_LINE } else { room };
        let mut clicked = None;
        // A changed window size must not leave the selected row out of view.
        let resized = (room - self.roster_height).abs() > 1.0;
        self.roster_height = room;
        let scroll = std::mem::take(&mut self.scroll_to_selected) || resized;
        let mut spans = Vec::with_capacity(count);
        let order = &self.state.order;
        let output = egui::ScrollArea::vertical()
            .id_salt("run_roster")
            .auto_shrink([false, false])
            .max_height(list_height)
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                let origin = ui.cursor().min.y;
                let mut selected_rect = None;
                let mut next_rect = None;
                for (&index, layout) in order.iter().zip(&layouts) {
                    let selected = self.selected == index;
                    let p = &self.state.participants[index];
                    let sync = self.sync_word(index);
                    let turn = TurnMark::of(index, current, next_up);
                    let name = row_name(p, turn, sync);
                    let response = ui.push_id(&p.key, |ui| {
                        design::row(ui, layout.height(scale), selected, &name, |ui, rect| {
                            paint_row(ui, rect, p, turn, sync, scale, layout);
                        })
                    });
                    let response = response.inner.on_hover_text(&p.side);
                    let rect = response.rect;
                    spans.push((rect.top() - origin, rect.bottom() - origin, index));
                    if response.clicked() {
                        clicked = Some(index);
                    }
                    if selected {
                        selected_rect = Some(rect);
                    }
                    if turn == TurnMark::Next {
                        next_rect = Some(rect);
                    }
                }
                if scroll && let Some(rect) = selected_rect {
                    // The next row too when it follows, so the DM sees who is coming. Past
                    // the area's clip margin at each end, so neither row is cut.
                    let wanted = next_rect
                        .filter(|next| {
                            next.top() >= rect.top()
                                && next.bottom() - rect.top() + 8.0 < list_height
                        })
                        .map_or(rect, |next| rect.union(next));
                    ui.scroll_to_rect(wanted.expand2(vec2(0.0, 4.0)), None);
                }
            });
        self.overflow_note(ui, &output, &spans);
        if let Some(index) = clicked {
            self.select(index);
        }
    }

    /// In a long roster, say what is out of view, on which side, and what needs attention
    /// there. A row cut by the edge cannot be read, so it counts as out of view.
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
        let above: Vec<usize> = spans
            .iter()
            .filter(|(start, ..)| *start < top - 1.0)
            .map(|(.., index)| *index)
            .collect();
        let below: Vec<usize> = spans
            .iter()
            .filter(|(_, end, _)| *end > bottom + 1.0)
            .map(|(.., index)| *index)
            .collect();
        if above.is_empty() && below.is_empty() {
            return;
        }
        let side = |word: &str, rows: &[usize]| -> Option<String> {
            if rows.is_empty() {
                return None;
            }
            let count = |test: &dyn Fn(usize) -> bool| rows.iter().filter(|i| test(**i)).count();
            let state = |i: usize| {
                design::hp_state(
                    self.state.participants[i].hp,
                    self.state.participants[i].max_hp,
                )
            };
            let syncs_in = |wanted: SyncState| {
                count(&|i| {
                    self.syncs
                        .get(&i)
                        .is_some_and(|sync| sync.state() == wanted)
                })
            };
            let mut text = format!("{} {word}", rows.len());
            for (n, label) in [
                (syncs_in(SyncState::Conflict), "Conflict"),
                (syncs_in(SyncState::Stopped), "Sync stopped"),
                (syncs_in(SyncState::Error), "Sync error"),
                (count(&|i| state(i) == design::HpState::Zero), "0 HP"),
                (
                    count(&|i| state(i) == design::HpState::Bloodied),
                    "Bloodied",
                ),
            ] {
                if n > 0 {
                    text.push_str(&format!(", {n} {label}"));
                }
            }
            Some(text)
        };
        let text = [side("above", &above), side("below", &below)]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" · ");
        ui.add_space(6.0);
        ui.add_sized(
            [ui.available_width(), OVERFLOW_LINE - 6.0],
            egui::Label::new(Type::Small.rich(ui, text).color(colors().muted)),
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
                    let (presence, hint) = presence_action(p.present);
                    if Action::quiet(presence)
                        .icon(icons::ABSENT)
                        .density(Density::Read)
                        .hint(hint)
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
                        .density(Density::Read)
                        .name("Conditions for this participant")
                        .key("C")
                        .show(ui)
                        .clicked()
                    {
                        open_conditions = true;
                    }
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        // Beside the actions at the minimum window it gives up size, not letters.
                        let role = if design::number_width(ui, &p.name, &Type::Heading.font(ui))
                            < ui.available_width()
                        {
                            Type::Heading
                        } else {
                            Type::Small
                        };
                        // The panel's title has the whole of a name too long for this line.
                        let shown = design::words_that_fit(
                            ui,
                            &p.name,
                            &role.font(ui),
                            ui.available_width(),
                        );
                        let response = ui
                            .label(role.rich(ui, shown).color(c.text))
                            .on_hover_text(&p.name);
                        design::named(ui, &response, &format!("Selected: {}", p.name));
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
            // Five actions share the narrowest row. None of them may lose a letter.
            ui.spacing_mut().item_spacing.x = 0.0;
            if Action::quiet("Close run")
                .density(Density::Read)
                .blocked(self.close_block())
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
                self.save_status(ui);
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

    /// `Saved` or `Not saved`, in every phase.
    fn save_status(&self, ui: &mut egui::Ui) {
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
    }

    /// The second step of closing. Returns true when the run has closed. The words follow
    /// the save state: a run that cannot save must not say that it stays saved.
    fn close_confirmation(&mut self, ui: &mut egui::Ui, note: &str) -> bool {
        let cannot_save = self.strip().not_saved;
        let stranded = self.stranded_names();
        let c = colors();
        let (note, color) = match (cannot_save, stranded.as_slice()) {
            (true, _) => ("Closing loses unsaved changes.".to_owned(), c.result.error),
            (false, []) => (note.to_owned(), c.muted),
            // Short: it shares the footer with both buttons at the minimum size.
            (false, [name]) => (format!("Not sent: {name}."), c.result.error),
            (false, [name, others @ ..]) => (
                format!("Not sent: {name} and {} more.", others.len()),
                c.result.error,
            ),
        };
        ui.label(Type::Small.rich(ui, note).color(color));
        if Action::quiet("Keep running")
            .density(Density::Read)
            .show(ui)
            .clicked()
        {
            self.closing = false;
        }
        if cannot_save {
            return Action::destructive("Close anyway")
                .density(Density::Read)
                .blocked(self.close_block())
                .show(ui)
                .clicked();
        }
        Action::destructive(if stranded.is_empty() {
            "Close now"
        } else {
            "Close anyway"
        })
        .density(Density::Read)
        .show(ui)
        .clicked()
            && self.close_run()
    }

    pub(super) fn close_run(&mut self) -> bool {
        if let Some(reason) = self.close_block() {
            self.error = Some(reason.to_owned());
            return false;
        }
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
        if self.linked_participants().is_empty() {
            return;
        }
        let stopping = matches!(self.party_sync_plan(), PartySync::Stop(_));
        if Action::quiet(if stopping { "Stop sync" } else { "Sync party" })
            .icon(icons::SYNC_ON)
            .density(Density::Read)
            .name(if stopping {
                "Stop party sync"
            } else {
                "Sync party"
            })
            .hint("Read and update every linked character on D&D Beyond")
            .blocked_if(
                !self.connected && !self.simulate_connection,
                "Connect to D&D Beyond in Settings to sync.",
            )
            .show(ui)
            .clicked()
        {
            self.toggle_party_sync();
        }
    }

    pub(super) fn toggle_party_sync(&mut self) {
        match self.party_sync_plan() {
            PartySync::Stop(indexes) => {
                for index in indexes {
                    self.syncs.entry(index).or_default().stop();
                }
            }
            PartySync::Start(indexes) => {
                let simulated = self.simulate_connection;
                for index in indexes {
                    let sync = self.syncs.entry(index).or_default();
                    sync.start();
                    if simulated {
                        sync.keep_without_connection();
                    }
                }
            }
        }
    }

    /// Setup and the ended view have no row panel, but a sync can still be running. Its
    /// conflicts, and a way to stop it, must be here or the run cannot close.
    fn sync_block(&mut self, ui: &mut egui::Ui) {
        let names: Vec<(usize, String)> = self
            .syncs
            .iter()
            .filter(|(_, sync)| sync.enabled && (sync.busy() || sync.dirty()))
            .filter_map(|(index, _)| {
                Some((*index, self.state.participants.get(*index)?.name.clone()))
            })
            .collect();
        if names.is_empty() {
            return;
        }
        design::group_label(ui, "D&D Beyond sync");
        for (index, name) in names {
            let Some(sync) = self.syncs.get_mut(&index) else {
                continue;
            };
            let conflict = sync.state() == SyncState::Conflict;
            ui.push_id(index, |ui| {
                let word = sync_status(sync.state()).word;
                ui.label(
                    Type::Strong
                        .rich(ui, format!("{name}: {word}"))
                        .color(colors().text),
                );
                if conflict {
                    sync.conflicts(ui);
                }
                if Action::quiet("Stop sync")
                    .density(Density::Read)
                    .name(&format!("Stop sync for {name}"))
                    .hint("Sync stops, and changes not yet confirmed are not sent")
                    .show(ui)
                    .clicked()
                {
                    sync.stop();
                }
            });
        }
        design::rule(ui);
    }

    // ---- Side panel -------------------------------------------------------------

    fn side_panel(&mut self, ui: &mut egui::Ui) {
        let index = self.selected;
        let p = &self.state.participants[index];
        let (hp, successes, failures) = (p.hp, p.successes, p.failures);
        let is_sheet = p.definition.fields.contains_key("splat_external");
        ui.add(egui::Label::new(Type::Title.rich(ui, &p.name).color(colors().text)).wrap());
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
            .show(ui, |ui| style::markdown(ui, &self.setup_details));
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
            let freshness = crate::party::freshness(record, self.connected);
            // The sheet is older than what the DM just set here, so say which is which.
            // While sync reads this sheet, the copy's age says nothing about these values.
            let word = if self.syncs.get(&index).is_some_and(|sync| sync.ready()) {
                "Connected · Sync reads this sheet · HP from this run".to_owned()
            } else {
                format!("{} · HP from this run", freshness.word)
            };
            design::status(
                ui,
                Status {
                    icon: freshness.icon,
                    word: &word,
                    color: freshness.color,
                },
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
                        builtin: false,
                    },
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
        self.back_out_on_escape(ui);
        let mut closed = false;
        let width = panel_width(ui);
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
        let room = ui.available_height() - FOOTER_HEIGHT - design::M;
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
                    .blocked(self.close_block())
                    .show(ui)
                    .clicked()
                {
                    self.closing = true;
                }
                let label = self.undo_label();
                if self.undo_action(&label).show(ui).clicked() {
                    self.undo();
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    self.save_status(ui)
                });
            }
        });
        if let Some(command) = command {
            if matches!(command, Command::Begin) {
                self.panel = Panel::Reference;
            }
            let rolled = matches!(command, Command::RollNpcs);
            self.run_command(command);
            // Next is the first player still without a number.
            if rolled {
                self.focus_first_missing = true;
            }
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
        let (name, npc, mode, bonus, initiative, present) = (
            &p.name,
            p.npc,
            p.initiative_mode,
            p.initiative_bonus,
            p.initiative,
            p.present,
        );
        let expanded = self.setup_expanded == Some(index);
        let row = Density::Scan.row() + design::XS;
        let mut enter = false;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = design::XS;
            let actions = 72.0 + 76.0 + 28.0 + 78.0 + 4.0 * design::XS;
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
                .plain()
                .icon(icons::DICE)
                .density(Density::Scan)
                .name(&format!("Roll for {name}"))
                .blocked_if(bonus.is_none(), "Set an initiative bonus, or type a total.")
                .show(ui)
                .clicked()
            {
                command = Some(Command::RollInitiative(index));
            }
            let (word, hint) = presence_action(present);
            if Action::quiet(word)
                .plain()
                .icon(icons::ABSENT)
                .density(Density::Scan)
                .name(&format!("{word}: {name}"))
                .hint(hint)
                .selected(!present)
                .show(ui)
                .clicked()
            {
                command = Some(Command::Presence(index, !present));
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
                design::named(ui, &combo.response, &format!("Initiative roll for {name}"));
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
        self.sync_block(ui);
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
        self.back_out_on_escape(ui);
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
                    .blocked(self.close_block())
                    .show(ui)
                    .clicked()
                {
                    if self.strip().not_saved || !self.stranded_names().is_empty() {
                        self.closing = true;
                    } else {
                        closed = self.close_run();
                    }
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
                self.save_status(ui);
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
        // Close run is this view's main action, so the reason it is off is not left to a hover.
        if let Some(reason) = self.close_block() {
            ui.add(egui::Label::new(Type::Small.rich(ui, reason).color(c.muted)).wrap());
        }
        if self.closing {
            ui.horizontal(|ui| {
                closed = self.close_confirmation(ui, "The run stays saved.");
            });
        }
        let width = design::side_panel_width(ui.ctx().content_rect().width());
        egui::SidePanel::right("run_ended_log")
            .resizable(false)
            .exact_width(width)
            .frame(side_frame())
            .show_inside(ui, |ui| {
                // Several conflicts can be long, and the log must keep some room.
                let room = ui.available_height() * 0.6;
                egui::ScrollArea::vertical()
                    .id_salt("run_ended_sync")
                    .max_height(room)
                    .auto_shrink([false, true])
                    .show(ui, |ui| self.sync_block(ui));
                if !self.setup_details.is_empty() {
                    design::section(ui, "Notes", |ui| self.notes_tab(ui));
                }
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
                            let sync = self.sync_word(*index);
                            let name = row_name(p, TurnMark::None, sync);
                            let layout = RowLayout::new(
                                ui,
                                ui.available_width(),
                                p,
                                sync,
                                scale,
                                RowFit::Natural,
                                0.0,
                            );
                            design::row(ui, layout.height(scale), false, &name, |ui, rect| {
                                paint_row(ui, rect, p, TurnMark::None, sync, scale, &layout);
                            });
                        }
                    });
            });
        closed
    }
}

/// The action names what it does, so its label cannot be read as a state.
fn presence_action(present: bool) -> (&'static str, &'static str) {
    if present {
        (
            "Absent",
            "Skip this participant's turns until marked present",
        )
    } else {
        ("Mark present", "Give this participant turns again")
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

/// A row's place in the turn order. The next participant is never the current one.
#[derive(Clone, Copy, PartialEq)]
enum TurnMark {
    Current,
    Next,
    None,
}

impl TurnMark {
    fn of(index: usize, current: Option<usize>, next_up: Option<usize>) -> Self {
        if current == Some(index) {
            Self::Current
        } else if next_up == Some(index) {
            Self::Next
        } else {
            Self::None
        }
    }
}

/// Everything on a row, for assistive technology and for the tests.
fn row_name(p: &Participant, turn: TurnMark, sync: Option<SyncState>) -> String {
    let mut parts = vec![
        p.name.clone(),
        p.side.clone(),
        format!("HP {}/{}", display(p.hp), display(p.max_hp)),
        format!("Temp {}", p.temporary_hp),
        format!("AC {}", display(p.armor_class)),
        format!("Initiative {}", display(p.initiative)),
    ];
    match turn {
        TurnMark::Current => parts.push("Current turn".into()),
        TurnMark::Next => parts.push("Next up".into()),
        TurnMark::None => {}
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

/// How much height a row may use.
#[derive(Clone, Copy, PartialEq)]
enum RowFit {
    Natural,
    /// The list needs the height, so a row gives up its padding too.
    Compressed,
}

impl RowFit {
    /// A compressed row's bar starts later, still under the digits, so its second line has
    /// the width that the turn word took and a condition does not wrap again.
    fn bar_inset(self) -> f32 {
        match self {
            RowFit::Natural => 0.0,
            RowFit::Compressed => 8.0,
        }
    }
}

/// The height that makes these rows, none shorter than it is now, fill `room`, up to `cap`.
/// Rows taller than that keep their own height. Zero if the rows already fill `room`.
fn even_height(heights: &[f32], room: f32, cap: f32) -> f32 {
    let mut sorted = heights.to_vec();
    sorted.sort_by(f32::total_cmp);
    let mut rest = room;
    let mut left = sorted.len();
    for &height in sorted.iter().rev() {
        if height <= rest / left as f32 {
            break;
        }
        rest -= height;
        left -= 1;
    }
    if left == 0 || rest <= 0.0 {
        return 0.0;
    }
    (rest / left as f32).min(cap)
}

/// The fixed columns of a run row, and its second line, which grows rather than cuts.
struct RowLayout {
    /// The name at this row's width. A long name wraps and the row grows.
    name: Arc<Galley>,
    second_line: Option<Arc<Galley>>,
    /// The row has no room for a state column. Its state and sync words open the second
    /// line, so that the name stays whole.
    inline_state: bool,
    /// A name-only row in a list that needs the height.
    tight: bool,
    fit: RowFit,
    /// The compressed list's shared row height, once any spare room is shared out.
    even: f32,
    initiative_right: f32,
    name_left: f32,
    ac_right: f32,
    temp_right: f32,
    hp_right: f32,
}

/// The current-row bar, the gap after it, and room for the word `TURN` at this size.
fn initiative_width(scale: RunScale, fit: RowFit) -> f32 {
    let word = scale.mark * 3.2;
    6.0 + design::XS
        + match fit {
            RowFit::Natural => word,
            // The word sits on the number's line, so that a row's height never depends on
            // whose turn it is.
            RowFit::Compressed => word + scale.stat,
        }
}

fn header_height(scale: RunScale) -> f32 {
    (scale.hp * 1.6).max(36.0)
}

impl RowLayout {
    fn new(
        ui: &egui::Ui,
        width: f32,
        p: &Participant,
        sync: Option<SyncState>,
        scale: RunScale,
        fit: RowFit,
        longest_name: f32,
    ) -> Self {
        let c = colors();
        // Fixed columns from the left of the numbers. A name stays near its HP.
        let initiative_right = initiative_width(scale, fit);
        let gap = if fit == RowFit::Compressed { 6.0 } else { 12.0 };
        let name_left = initiative_right + gap + scale.name * 0.7;
        let hp_width = scale.hp * 1.9 + scale.hp_max * 2.0 + 4.0;
        let state_width = (scale.mark * 7.0).clamp(84.0, 110.0);
        let temp_width = scale.stat * 2.6 + 12.0;
        let ac_width = scale.stat * 1.5 + 22.0;
        let numbers = name_left + ac_width + temp_width + hp_width + design::S;
        let inline_state = width - numbers - state_width < scale.name * NAME_ROOM;
        let name_room = (width - numbers - if inline_state { 0.0 } else { state_width }).max(90.0);
        // Near its HP in a wide window, unless a name in this list needs more.
        let name_width = name_room.min((scale.name * 12.5).max(longest_name));
        let ac_right = name_left + name_width + ac_width;
        let temp_right = ac_right + temp_width;
        let hp_right = temp_right + hp_width;

        let mut job = egui::text::LayoutJob::default();
        // The line wraps before the HP column, so that it never runs over the HP bar.
        job.wrap.max_width = (temp_right + fit.bar_inset() - name_left).max(1.0);
        let font = Type::Body.sized(ui, scale.condition);
        let mut first = true;
        let mut add = |text: &str, color: egui::Color32| {
            if !first {
                job.append(
                    " · ",
                    0.0,
                    egui::TextFormat {
                        font_id: font.clone(),
                        color: c.muted,
                        ..Default::default()
                    },
                );
            }
            first = false;
            job.append(
                text,
                0.0,
                egui::TextFormat {
                    font_id: font.clone(),
                    color,
                    ..Default::default()
                },
            );
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
        let name = ui.fonts_mut(|fonts| {
            fonts.layout(
                p.name.clone(),
                Type::Name.sized(ui, scale.name),
                egui::Color32::PLACEHOLDER,
                name_width,
            )
        });
        let tight = fit == RowFit::Compressed
            && second_line.is_none()
            && sync.is_none()
            && name.rows.len() == 1;
        Self {
            name,
            even: 0.0,
            second_line,
            inline_state,
            tight,
            fit,
            initiative_right,
            name_left,
            ac_right,
            temp_right,
            hp_right,
        }
    }

    fn name_line(&self, scale: RunScale) -> f32 {
        scale.name
            + match self.fit {
                RowFit::Compressed => 2.0,
                RowFit::Natural => 4.0,
            }
    }

    /// Rows in a list get taller, never shorter, when conditions need another line.
    fn height(&self, scale: RunScale) -> f32 {
        if self.tight {
            return TIGHT_ROW.min(scale.row).max(self.even);
        }
        // The row's own text decides, with the bar and its hairline clear below it.
        let second_height = scale.condition + 2.0;
        let second = self
            .second_line
            .as_ref()
            .map_or(second_height, |line| line.size().y);
        let name_line = self.name_line(scale);
        if self.fit == RowFit::Compressed {
            return (1.0 + name_line + self.name_extra() + second + 1.0).max(self.even);
        }
        let top = ((scale.row - name_line - second_height) / 2.0).max(1.0);
        scale
            .row
            .max(top + name_line + self.name_extra() + second + 3.0)
    }

    /// The height a wrapped name adds under its first line.
    fn name_extra(&self) -> f32 {
        let lines = self.name.rows.len().max(1) as f32;
        self.name.size().y * (lines - 1.0) / lines
    }

    /// The centre of the name line, down from the row's top.
    fn line_y(&self, scale: RunScale, height: f32) -> f32 {
        let name_line = self.name_line(scale);
        let second_height = scale.condition + 2.0;
        // A row with one line centres it until there is room for two. Then every row
        // shares one place, with or without a second line, so adding one moves no number.
        let basis = match self.fit {
            RowFit::Natural => scale.row,
            RowFit::Compressed => self
                .even
                .max(height.min(1.0 + name_line + second_height + 1.0)),
        };
        if basis < 1.0 + name_line + second_height {
            return height / 2.0 - 2.0;
        }
        ((basis - name_line - second_height) / 2.0).max(1.0) + name_line / 2.0
    }
}

fn paint_row(
    ui: &egui::Ui,
    rect: Rect,
    p: &Participant,
    turn: TurnMark,
    sync: Option<SyncState>,
    scale: RunScale,
    layout: &RowLayout,
) {
    let c = colors();
    let painter = ui.painter();
    let current = turn == TurnMark::Current;
    let x = rect.left();
    let name_line = layout.name_line(scale);
    let second_height = scale.condition + 2.0;
    let line_y = rect.top() + layout.line_y(scale, rect.height());
    let second_y = line_y + name_line / 2.0 + layout.name_extra();
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
    let initiative_right = x + layout.initiative_right;
    let initiative_left = design::number(
        painter,
        initiative_right,
        line_y,
        &display(p.initiative),
        &numeral,
        if current { c.turn.current } else { c.muted },
    );
    let mark = match turn {
        TurnMark::Current => Some(("TURN", c.turn.current)),
        TurnMark::Next => Some(("NEXT", c.turn.next)),
        TurnMark::None => None,
    };
    if let Some((word, color)) = mark {
        let place = match layout.fit {
            RowFit::Natural => pos2(
                initiative_right,
                line_y + name_line / 2.0 + second_height / 2.0,
            ),
            RowFit::Compressed => pos2(initiative_left - design::XS, line_y),
        };
        painter.text(
            place,
            Align2::RIGHT_CENTER,
            word,
            Type::Label.sized(ui, scale.mark),
            color,
        );
    }
    side_mark(
        painter,
        pos2(x + layout.name_left - scale.name * 0.6, line_y),
        &p.side,
        scale.name * 0.55,
    );
    let name_left = x + layout.name_left;
    painter.galley(
        pos2(name_left, line_y - name_line / 2.0),
        layout.name.clone(),
        text,
    );
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
    design::hp_readout(ui, painter, hp_right, line_y, p.hp, p.max_hp, &fonts);
    // Under its own number, and thick enough to read across a table.
    let bar_height = if rect.height() < 30.0 {
        2.0
    } else {
        (scale.row / 14.0).clamp(3.0, 5.0)
    };
    // Close under the digits, and clear of the hairline.
    let bar_top = (line_y + scale.hp * 0.5 + 3.0).min(rect.bottom() - bar_height - 1.0);
    let bar = Rect::from_min_max(
        pos2(
            x + layout.temp_right + layout.fit.bar_inset() + 4.0,
            bar_top,
        ),
        pos2(hp_right, bar_top + bar_height),
    );
    design::hp_bar(painter, bar, p.hp, p.max_hp, p.temporary_hp);
    let state_font = Type::Heading.sized(ui, scale.mark.max(12.0));
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
            state_font.clone(),
            color,
        );
    }
    // Conditions have their own line, in their own colour, in full. Sync has its slot, as
    // strong as Bloodied, because a conflict must be seen at a glance.
    if let Some(line) = &layout.second_line {
        painter.galley(
            pos2(name_left, second_y),
            line.clone(),
            c.participant.condition,
        );
    }
    if let Some(sync) = sync.filter(|_| !layout.inline_state) {
        let status = sync_status(sync);
        // In the state column, under Bloodied, so the two are read together.
        painter.text(
            pos2(hp_right + design::S, second_y + second_height / 2.0),
            Align2::LEFT_CENTER,
            format!("{} {}", status.icon, status.word),
            state_font,
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
