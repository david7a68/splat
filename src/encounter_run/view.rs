//! The run's three views: setup, live and ended. Prominence follows the brief:
//! round and current turn, then every row's HP and conditions, then the actions.
//! Rows never move when the selection changes: the entry controls have a fixed dock.
use super::{
    Command, LastRoll, MAX_FROM_SHEET, NOBODY_PRESENT, Panel, Participant, PartySync, Phase, Run,
    display, mode_label,
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
use splat::content::overrides::{BY_HAND, Row};
use std::{path::PathBuf, sync::Arc};
use uuid::Uuid;

/// What is held while an encounter group is dragged to the initiative list.
struct GroupDrag(Uuid);

const AMOUNT: &str = "Amount";
const READ_ALOUD: &str = "Read-aloud and notes";
const SHEET_PAGES: [(usize, &str); 5] = [
    (0, "Overview"),
    (1, "Actions"),
    (2, "Spells"),
    (3, "Gear"),
    (4, "Features"),
];
/// Two lines of controls in a frame.
const DOCK_HEIGHT: f32 = 36.0 + 4.0 + 28.0 + 8.0;
/// The dock's width from which Damage has its key hint, and Temp HP and Roll their full forms.
const DOCK_ALL_WORDS_FROM: f32 = 500.0;
/// A roll's result has its place before there is one, so that nothing moves when it comes.
const DICE_RESULT_WIDTH: f32 = 32.0;
const FOOTER_HEIGHT: f32 = 28.0 + 7.0;
const GUTTER: f32 = design::M;
/// Room for the note under a long roster, past the scroll area's clip margin.
const OVERFLOW_LINE: f32 = 22.0;
/// Kept free under a list that fits, for a message of up to two lines.
const MESSAGE_ROOM: f32 = 44.0;
/// A message's height when the list needs the room.
const MESSAGE_LINE: f32 = 28.0;
const SCROLL_BAR: f32 = 12.0;
/// A name-only row when the list has no other way to fit. Still a 24 px target and more.
const TIGHT_ROW: f32 = 25.0;
/// The name column a row needs, in units of its name size, to keep a state column too.
const NAME_ROOM: f32 = 8.0;
/// Run toggles use a selected surface; the turn action keeps the accent fill.
pub(crate) fn toggle(
    ui: &mut egui::Ui,
    label: &str,
    icon: &str,
    selected: bool,
    name: &str,
    density: Density,
) -> egui::Response {
    let button = egui::Button::new(Type::Body.rich(ui, format!("{icon} {label}")))
        .selected(selected)
        .min_size(vec2(0.0, density.control()));
    let button = if selected {
        button.fill(colors().selected)
    } else {
        button.frame(false)
    };
    let response = ui.add(button);
    design::named(ui, &response, name);
    response
}

/// What sync needs from the DM, on the line above the roster.
struct SyncAlert {
    text: String,
    /// For when `text` does not fit beside a long Undo.
    short: String,
    color: egui::Color32,
}
impl SyncAlert {
    fn new(icon: &str, text: &str, short: &str, color: egui::Color32) -> Self {
        Self {
            text: format!("{icon} {text}"),
            short: format!("{icon} {short}"),
            color,
        }
    }
}

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
        SyncState::NotSent => Status {
            icon: icons::WARN,
            word: "Not sent",
            color: colors().result.warning,
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

/// Until the DM drags it, the panel leaves the roster its capped width and uses the rest.
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
const KEYS: [(&str, &str); 12] = [
    ("N", "Next turn"),
    ("Enter", "Damage the selected rows by the amount"),
    ("Shift+Enter", "Heal the selected rows by the amount"),
    ("Ctrl+click", "Add a row to the targets of one amount"),
    ("Escape", "Select the current turn again"),
    ("U or Ctrl+Z", "Undo"),
    ("Up, Down", "Select another row"),
    ("A", "Focus the amount"),
    ("C", "Conditions"),
    ("Ctrl+G", "Arrivals"),
    ("Ctrl+K", "Open or close Lookup. Escape also closes it"),
    ("Ctrl+J", "Open or close Notes. Escape also closes them"),
];

impl Run {
    /// Returns true only after the DM chooses to close the run.
    pub fn show(&mut self, ui: &mut egui::Ui) -> bool {
        if !matches!(self.state.phase, Phase::Ended { .. })
            && ui.ctx().memory(|memory| memory.top_modal_layer()).is_none()
            && ui.input_mut(|i| i.consume_key(Modifiers::COMMAND, Key::G))
        {
            self.panel = Panel::Groups;
            self.close_lookup = true;
        }
        self.refresh_portraits();
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

    pub(super) fn run_command(&mut self, command: Command) {
        let follow = matches!(command, Command::Next | Command::Begin);
        // A field that reports each key keeps the keyboard, or the next digit lands in the
        // note. The roll goes on to the first empty field.
        let keeps_keyboard = matches!(
            command,
            Command::Initiative(..)
                | Command::Health(..)
                | Command::Bonus(..)
                | Command::RollNpcs(_)
        );
        match self.apply(command) {
            Err(error) => self.error = Some(error),
            Ok(()) => {
                self.condition.clear();
                self.note.clear();
                // Beside a panel, every step gives the keyboard back, or words typed next
                // would go nowhere.
                if follow || (self.lookup_open && !keeps_keyboard) {
                    self.follow_with_keyboard();
                }
                if follow {
                    self.scroll_to_selected = true;
                    self.amount.clear();
                    self.clear_targets();
                    self.last_roll = None;
                    if self.panel == Panel::Conditions {
                        self.panel = Panel::Reference;
                    }
                }
            }
        }
    }

    fn clear_targets(&mut self) {
        self.also.clear();
        self.halved.clear();
    }

    /// Add or take back one row, for one amount on several targets.
    pub(super) fn toggle_target(&mut self, index: usize) {
        if index == self.selected {
            if let Some(next) = self.also.pop_first() {
                self.selected = next;
            }
            self.halved.remove(&index);
        } else if !self.also.remove(&index) {
            self.also.insert(index);
        } else {
            self.halved.remove(&index);
        }
        // One target has no half to choose, and a chip that is gone must not still count.
        if self.also.is_empty() {
            self.halved.clear();
        }
        self.amount.clear();
        self.focus_amount = true;
    }

    /// The selected row and every other target, in turn order, as the log reads.
    pub(super) fn targets(&self) -> Vec<usize> {
        let mut targets: Vec<usize> = self
            .also
            .iter()
            .copied()
            .filter(|index| *index != self.selected)
            .collect();
        targets.push(self.selected);
        targets.sort_by_key(|index| self.state.order.iter().position(|at| at == index));
        targets
    }

    /// The same step on the selected row and on every other target, as one step of undo. Half
    /// is for damage, and it rounds down.
    pub(super) fn amount_command(&self, amount: i64, make: fn(usize, i64) -> Command) -> Command {
        if self.also.is_empty() {
            return make(self.selected, amount);
        }
        let halves = matches!(make(0, 0), Command::Damage(..));
        Command::Several(
            self.targets()
                .into_iter()
                .map(|index| {
                    let half = halves && self.halved.contains(&index);
                    make(index, if half { amount / 2 } else { amount })
                })
                .collect(),
        )
    }

    pub(super) fn select(&mut self, index: usize) {
        self.clear_targets();
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
        // Beside a panel, the digits of an earlier number must not join this one.
        if self.lookup_open {
            self.amount.clear();
        }
        self.focus_amount = true;
    }

    /// After a step, the keyboard goes to Amount, for the next number. Beside a panel, the
    /// DM may be typing there, and a digit and Enter must never become damage. Only a click
    /// on a row or on Amount itself takes the keyboard from a panel, for one number.
    pub(super) fn follow_with_keyboard(&mut self) {
        if self.lookup_open {
            self.amount.clear();
            self.return_focus = true;
        } else {
            self.focus_amount = true;
        }
    }

    /// A footer's second step waits for an answer, and Escape belongs to it.
    pub fn footer_step_open(&self) -> bool {
        self.closing || self.ending
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
            self.return_focus = self.lookup_open;
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
        // Back to whose turn it is, after a look at someone else.
        if let Some(current) = self.current()
            && (self.selected != current || !self.also.is_empty())
            && ui.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape))
        {
            self.select(current);
            self.panel = Panel::Reference;
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
            self.follow_with_keyboard();
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
            let make = if heal { Command::Heal } else { Command::Damage };
            self.run_command(self.amount_command(amount, make));
            self.amount.clear();
            self.clear_targets();
            self.follow_with_keyboard();
        }
        if conditions {
            self.open_conditions_tab();
        }
    }

    fn undo_action<'a>(&self, label: &'a str) -> Action<'a> {
        let action = Action::secondary(label)
            .icon(icons::UNDO)
            .density(Density::Read);
        let action = if matches!(self.state.phase, Phase::Running) && !self.lookup_open {
            action.key("U")
        } else {
            action
        };
        action.blocked_if(
            self.history.is_empty(),
            if self.undo_stopped {
                "Undo stopped at sheet update. See Log."
            } else if self.reopened {
                "Nothing to undo since this run reopened"
            } else {
                "Nothing to undo yet."
            },
        )
    }

    fn undo_reason(&self, ui: &mut egui::Ui) {
        if self.history.is_empty() {
            let reason = if self.undo_stopped {
                "Undo stopped at sheet update"
            } else if self.reopened {
                "Nothing to undo since this run reopened"
            } else {
                "Nothing to undo yet"
            };
            ui.label(Type::Small.rich(ui, reason).color(colors().muted));
        }
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
            // The action that loses nothing is the filled one.
            if Action::secondary("Save as a new run")
                .density(Density::Read)
                .hint("Keep the changed file as it is, and save this run beside it")
                .show(ui)
                .clicked()
            {
                outcome = Some(self.save_as_new_run());
            }
            if Action::destructive("Replace the file with this run")
                .density(Density::Read)
                .hint("The changes in the file on disk are lost")
                .show(ui)
                .clicked()
            {
                outcome = Some(self.keep_this_run());
            }
        });
        if let Some(Err(error)) = outcome {
            self.error = Some(error);
        }
        ui.add_space(design::XS);
    }

    /// Beside a panel the roster reaches it, wherever the DM drags it. Alone, it keeps its cap.
    fn roster_width(&self, ui: &egui::Ui) -> f32 {
        let room = ui.available_width() - GUTTER;
        if self.hide_panel {
            room.min(design::RUN_LARGE.max_width)
        } else {
            room
        }
    }

    // ---- Live -------------------------------------------------------------------

    fn live_view(&mut self, ui: &mut egui::Ui) -> bool {
        // Before the keys, which give way to the keyboard beside a panel.
        self.back_out_on_escape(ui);
        self.live_keys(ui);
        let mut closed = false;
        let width = panel_width(ui);
        if !self.hide_panel {
            design::Column::panel("run_panel", width)
                .page_keeps(design::RUN_LIST_KEEPS)
                .frame(side_frame())
                .show_inside(ui, |ui| self.side_panel(ui));
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| {
                // A panel's own width is fixed, so the cap needs an inner layout.
                let size = vec2(self.roster_width(ui), ui.available_height());
                ui.allocate_ui_with_layout(size, Layout::top_down(Align::LEFT), |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    // The rows' scale decides the header's size, and needs only a close
                    // idea of the room the header will leave.
                    let scale = RunScale::fit(
                        ui.available_height()
                            - header_height(design::RUN_REGULAR)
                            - 32.0
                            - DOCK_HEIGHT
                            - self.footer_height(),
                        ui.available_width(),
                        self.state.order.len(),
                    );
                    self.live_header(ui, scale);
                    self.error_line(ui);
                    let room = ui.available_height() - DOCK_HEIGHT - self.footer_height();
                    // A message goes in the room under the list, so it moves no row while
                    // the DM's eye is on them. The next action clears it. A long one wraps.
                    let notice = self.notice.as_ref().map(|_| {
                        format!(
                            "{}; see Log.",
                            design::count(self.notice_changes, "sheet change", "sheet changes")
                        )
                    });
                    let width = ui.available_width();
                    let notice_height = notice.as_ref().map_or(0.0, |notice| {
                        let font = Type::Small.font(ui);
                        ui.fonts_mut(|fonts| {
                            fonts.layout(
                                notice.clone(),
                                font,
                                egui::Color32::PLACEHOLDER,
                                width - 32.0,
                            )
                        })
                        .size()
                        .y
                        .max(MESSAGE_ROOM)
                    });
                    // A list that would have to scroll to make room gets it back: the message
                    // keeps its point on one line, with the whole of it on hover and in the log.
                    let tightest = self.tightest_roster(ui, width, room);
                    let compact = notice.is_some() && tightest > room - notice_height;
                    let notice_height = if compact { MESSAGE_LINE } else { notice_height };
                    // With an error line up too, one more line would scroll even a compressed
                    // list. The error comes first, and the message returns when it is dismissed.
                    let notice = notice.filter(|_| tightest <= room - notice_height);
                    let notice_height = if notice.is_some() { notice_height } else { 0.0 };
                    let list = room - notice_height;
                    let roster = ui.allocate_ui_with_layout(
                        vec2(ui.available_width(), list),
                        Layout::top_down(Align::LEFT),
                        |ui| {
                            ui.set_min_height(list);
                            self.roster(ui, list, notice_height);
                        },
                    );
                    self.group_drop_zone(ui, roster.response.rect);
                    if let Some(notice) = notice {
                        // All the height the list gave up, so the dock stays where it was.
                        ui.allocate_ui_with_layout(
                            vec2(ui.available_width(), notice_height),
                            Layout::left_to_right(Align::TOP),
                            |ui| {
                                ui.set_min_height(notice_height);
                                let dismiss =
                                    design::icon_button(ui, icons::CLOSE, "Dismiss this message")
                                        .clicked();
                                let shown = notice.as_str();
                                let response = ui.add(
                                    egui::Label::new(
                                        Type::Small.rich(ui, shown).color(colors().result.warning),
                                    )
                                    .wrap(),
                                );
                                let detail = self.notice.as_deref().unwrap_or(shown);
                                design::named(ui, &response, detail);
                                response.on_hover_text(detail);
                                if dismiss {
                                    self.notice = None;
                                    self.return_focus = self.lookup_open;
                                }
                            },
                        );
                    }
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
                    .key(if self.lookup_open { "" } else { "N" })
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
                    // The first answer on the screen, and larger than any row's name. It gives
                    // up a little size, then the later words of a long name. The row, the panel
                    // and the dock's hover have the whole name.
                    let room = ui.available_width() - 1.0;
                    let suffix = "’s turn";
                    let full = format!("{name}{suffix}");
                    let least = (scale.name + 1.0).min(turn_size);
                    let font = Type::Name.sized(ui, turn_size);
                    let fit = |text: &str| {
                        let width = design::number_width(ui, text, &font);
                        (turn_size * room / width).min(turn_size)
                    };
                    let full_size = fit(&full);
                    let (text, size) = if full_size >= least {
                        (full, full_size)
                    } else {
                        let words = design::words_that_fit(
                            ui,
                            name,
                            &font,
                            room - design::number_width(ui, suffix, &font),
                        );
                        let short = format!("{words}{suffix}");
                        let size = fit(&short).max(least);
                        (short, size)
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
        // What sync needs from the DM. These stay until the cause is gone. A stopped sync
        // comes first, because nothing else can be resolved without a connection.
        let kept = match strip.unsent.as_slice() {
            [] => None,
            [name] => Some(format!("{name}: kept in Splat, not sent")),
            names => Some(format!(
                "{} kept in Splat, not sent",
                design::count(names.len(), "change", "changes")
            )),
        };
        let sync_alert = if strip.sync_stopped {
            Some(match &kept {
                None => SyncAlert::new(
                    icons::OFFLINE,
                    "Signed out of D&D Beyond",
                    "Signed out",
                    c.muted,
                ),
                Some(_) => SyncAlert::new(
                    icons::OFFLINE,
                    &format!(
                        "Signed out of D&D Beyond. {}: not sent",
                        strip.unsent.join(", ")
                    ),
                    "Signed out",
                    c.result.error,
                ),
            })
        } else if let Some(name) = &strip.conflict {
            Some(SyncAlert::new(
                icons::WARN,
                &format!("Sync conflict: {name}"),
                &format!("Conflict: {name}"),
                c.sync.conflict,
            ))
        } else if let Some(kept) = &kept {
            Some(SyncAlert::new(icons::WARN, kept, kept, c.result.warning))
        } else {
            strip.sync_error.as_ref().map(|name| {
                SyncAlert::new(
                    icons::ERROR,
                    &format!("Sync error: {name}"),
                    &format!("Error: {name}"),
                    c.result.error,
                )
            })
        };
        let mut review_conflict = false;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 28.0),
            Layout::right_to_left(Align::Center),
            |ui| {
                ui.set_min_height(28.0);
                undo = self
                    .undo_action(&undo_label)
                    .hint(if self.lookup_open {
                        "Undo the last step"
                    } else {
                        "Undo the last step (U)"
                    })
                    .show(ui)
                    .clicked();
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    if let Some(SyncAlert { text, short, color }) = &sync_alert {
                        // Notes hide the panel with the conflict's controls, so one word leads there.
                        let review = self.hide_panel && strip.conflict.is_some();
                        let link = if review {
                            design::number_width(ui, "Review", &Type::Body.font(ui)) + design::S
                        } else if strip.sync_stopped {
                            design::number_width(ui, "Sign in again", &Type::Body.font(ui))
                                + design::S
                        } else {
                            0.0
                        };
                        // Next to a long Undo it gives up size, then the word Sync, never the name.
                        let room = ui.available_width() - link;
                        let (role, text) =
                            if design::number_width(ui, text, &Type::Strong.font(ui)) < room {
                                (Type::Strong, text)
                            } else if design::number_width(ui, text, &Type::Small.font(ui)) < room {
                                (Type::Small, text)
                            } else {
                                (Type::Small, short)
                            };
                        ui.label(role.rich(ui, text.as_str()).color(*color));
                        review_conflict = review && design::link(ui, "Review").clicked();
                        if strip.sync_stopped && design::link(ui, "Sign in again").clicked() {
                            self.wants_sign_in = true;
                        }
                    } else if self.history.is_empty() {
                        self.undo_reason(ui);
                    } else if let Some(notice) = self.recording_notice {
                        // Beside a long Undo, the state stays and its advice goes.
                        let fits = design::number_width(ui, notice, &Type::Small.font(ui))
                            < ui.available_width();
                        let words = if fits { notice } else { "Recording silence" };
                        ui.label(Type::Small.rich(ui, words).color(c.result.warning));
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
        design::rule(ui);
        if review_conflict {
            self.review_conflict();
        }
        if undo {
            self.undo();
            self.follow_with_keyboard();
        }
        if next_turn {
            self.run_command(Command::Next);
        }
    }

    /// The sync state a row shows, if this participant has one.
    pub(super) fn sync_word(&self, index: usize) -> Option<SyncState> {
        self.syncs
            .get(&index)
            .filter(|sync| sync.enabled || sync.error.is_some() || sync.stranded())
            .map(crate::combat_sync::CombatSync::state)
    }

    fn row_sync_word(&self, index: usize) -> Option<SyncState> {
        self.sync_word(index)
            .filter(|state| *state != SyncState::Stopped)
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
                RowLayout::new(ui, width, p, self.row_sync_word(index), scale, fit, longest)
                    .with_art(self.row_art(ui.ctx(), p))
            })
            .collect()
    }

    /// The height of this list at its most compressed, in this room.
    fn tightest_roster(&self, ui: &egui::Ui, width: f32, room: f32) -> f32 {
        let scale = RunScale::fit(room, width, self.state.order.len());
        self.roster_layouts(ui, width, scale, RowFit::Compressed)
            .iter()
            .map(|layout| layout.height(scale))
            .sum()
    }

    /// `room` is what the list has now, and `lent` what a message under it has taken. A list
    /// that fits leaves that much free at all times, so a message moves none of its rows.
    fn roster(&mut self, ui: &mut egui::Ui, room: f32, lent: f32) {
        let current = self.current();
        let next_up = self.next_up();
        let count = self.state.order.len();
        let mut scale = RunScale::fit(room + lent, ui.available_width(), count);
        let width = ui.available_width();
        let total = |layouts: &[RowLayout], scale: RunScale| -> f32 {
            layouts.iter().map(|l| l.height(scale)).sum()
        };
        let mut layouts = self.roster_layouts(ui, width, scale, RowFit::Natural);
        let natural = total(&layouts, scale);
        if natural <= room {
            // Spare room goes to the rows, so a short list leaves no gap under it.
            // Never more than the list has now, or the rows overflow it and compress.
            let spare = (room + lent - MESSAGE_ROOM).min(room) - natural - 2.0;
            let share = spare.max(0.0) / count.max(1) as f32;
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
        let mut rule = None;
        let mut toggle_half = None;
        // A changed window size must not leave the selected row out of view.
        let resized = (room - self.roster_height).abs() > 1.0;
        self.roster_height = room;
        let scroll = std::mem::take(&mut self.scroll_to_selected) || resized;
        let mut spans = Vec::with_capacity(count);
        let mut edit_initiative = None;
        let mut set_initiative = None;
        let order = self.state.order.clone();
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
                    let selected = self.selected == index || self.also.contains(&index);
                    let p = &self.state.participants[index];
                    let sync = self.row_sync_word(index);
                    let turn = TurnMark::of(index, current, next_up);
                    let name = row_name(p, turn, sync);
                    let response = ui.push_id(&p.key, |ui| {
                        if self.live_initiative == Some(index) {
                            let (rect, _) = ui.allocate_exact_size(
                                vec2(ui.available_width(), layout.height(scale)),
                                egui::Sense::hover(),
                            );
                            // The number field owns the initiative gutter while it is open.
                            let target = Rect::from_min_max(
                                pos2(
                                    rect.left() + layout.initiative_right + design::XS,
                                    rect.top(),
                                ),
                                rect.max,
                            );
                            let response = ui
                                .scope_builder(egui::UiBuilder::new().max_rect(target), |ui| {
                                    design::row(ui, rect.height(), selected, &name, |ui, _| {
                                        paint_row(ui, rect, p, turn, sync, scale, layout);
                                    })
                                })
                                .inner;
                            (response, rect)
                        } else {
                            let response = design::row(
                                ui,
                                layout.height(scale),
                                selected,
                                &name,
                                |ui, rect| {
                                    paint_row(ui, rect, p, turn, sync, scale, layout);
                                },
                            );
                            let rect = response.rect;
                            (response, rect)
                        }
                    });
                    let (response, rect) = response.inner;
                    let response = response.on_hover_text(&p.side);
                    spans.push((rect.top() - origin, rect.bottom() - origin, index));
                    if response.clicked() {
                        clicked = Some((index, ui.input(|i| i.modifiers.command)));
                    }
                    if let (Some(span), Some(galley)) =
                        (&layout.initiative_span, &layout.second_line)
                    {
                        let start = galley.pos_from_cursor(egui::text::CCursor::new(span.start));
                        let end = galley.pos_from_cursor(egui::text::CCursor::new(span.end));
                        let origin = vec2(
                            rect.left() + layout.second_left,
                            layout.second_y(scale, rect),
                        );
                        let word = Rect::from_min_max(start.min, pos2(end.left(), start.bottom()))
                            .translate(origin);
                        let target = Rect::from_center_size(
                            word.center(),
                            vec2(word.width(), Density::Read.control()),
                        );
                        let link = ui
                            .interact(
                                target,
                                ui.id().with(("missing_initiative", &p.key)),
                                egui::Sense::click(),
                            )
                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                        link.widget_info(|| {
                            egui::WidgetInfo::labeled(
                                egui::WidgetType::Link,
                                true,
                                format!("No initiative for {}", p.name),
                            )
                        });
                        if link.clicked() {
                            edit_initiative = Some(index);
                            clicked = None;
                        }
                        if link.has_focus() {
                            design::focus_ring(ui, target);
                        }
                    }
                    if self.live_initiative == Some(index) {
                        let field = Rect::from_min_size(
                            pos2(rect.left(), rect.top() + design::XS),
                            vec2(layout.initiative_right, Density::Read.control()),
                        );
                        ui.scope_builder(egui::UiBuilder::new().max_rect(field), |ui| {
                            let name = format!("Initiative for {}", p.name);
                            let text = self.setup_inputs.entry(index).or_default();
                            let response = design::number_field(
                                ui,
                                &name,
                                text,
                                "Init",
                                field.width(),
                                Density::Read.control(),
                            );
                            if self.focus_field == Some(index) {
                                response.request_focus();
                                self.focus_field = None;
                            }
                            if ui.input(|i| i.key_pressed(Key::Enter))
                                && (response.has_focus() || response.lost_focus())
                                && let Ok(total) = text.parse()
                            {
                                set_initiative = Some((index, total));
                            }
                        });
                    }
                    // Whoever the DM asks about is the one they are about to change.
                    if let Some(position) =
                        condition_links(ui, rect, &p.key, layout, scale, self.content.as_deref())
                    {
                        rule = Some(position);
                        clicked = Some((index, false));
                    }
                    // More than a fill says a row is one of several targets, and the mark
                    // says whether it took half for a save.
                    if selected && !self.also.is_empty() {
                        let half = self.halved.contains(&index);
                        let target = Rect::from_center_size(
                            pos2(rect.left() + design::L, rect.center().y),
                            vec2(24.0, 24.0),
                        );
                        let id = ui.id().with(("half_damage", &p.key));
                        let mark = ui
                            .interact(target, id, egui::Sense::click())
                            .on_hover_text("This target saved, so it takes half, rounded down.")
                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                        let name = format!("Half damage: {}", p.name);
                        mark.widget_info(|| {
                            egui::WidgetInfo::selected(
                                egui::WidgetType::Checkbox,
                                ui.is_enabled(),
                                half,
                                &name,
                            )
                        });
                        ui.painter().text(
                            target.center(),
                            Align2::CENTER_CENTER,
                            if half { "½" } else { icons::OK },
                            Type::Heading.font(ui),
                            colors().accent,
                        );
                        if mark.has_focus() {
                            design::focus_ring(ui, target);
                        }
                        if mark.clicked() {
                            toggle_half = Some(index);
                        }
                    }
                    if self.selected == index {
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
        if let Some(index) = edit_initiative {
            self.live_initiative = Some(index);
            self.focus_field = Some(index);
            self.setup_inputs.insert(index, String::new());
        }
        if let Some((index, total)) = set_initiative {
            self.run_command(Command::Initiative(index, Some(total)));
            self.live_initiative = None;
        }
        if let Some(index) = toggle_half {
            if !self.halved.remove(&index) {
                self.halved.insert(index);
            }
            self.focus_amount = true;
        }
        match clicked {
            Some((index, true)) => self.toggle_target(index),
            Some((index, false)) => self.select(index),
            None => {}
        }
        if rule == Some(usize::MAX) {
            self.panel = Panel::Conditions;
        } else if rule.is_some() {
            self.lookup_request = rule;
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
                (syncs_in(SyncState::NotSent), "Not sent"),
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
        let mut open_side = false;
        let frame = egui::Frame::new()
            .fill(c.selected)
            .inner_margin(egui::Margin::symmetric(design::S as i8, design::XS as i8));
        frame.show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.set_height(DOCK_HEIGHT - design::S);
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
            ui.horizontal(|ui| {
                ui.set_min_height(28.0);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let (presence, hint) = presence_action(p.present);
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
                    // Notes hide the panel with the saves, so one word leads there.
                    if self.hide_panel
                        && p.hp == Some(0)
                        && Action::quiet("Death saves")
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                    {
                        open_side = true;
                    }
                    if Action::quiet("Conditions")
                        .density(Density::Read)
                        .name("Conditions for this participant")
                        .key(if self.lookup_open { "" } else { "C" })
                        .show(ui)
                        .clicked()
                    {
                        open_conditions = true;
                    }
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        if narrow && let Some(roll) = &self.last_roll {
                            ui.label(Type::Strong.rich(ui, format!("= {}", roll.total)))
                                .on_hover_text(&roll.line);
                        }
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
                                "{} targets. Press a row's mark for half, rounded down.",
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
                    if let Some(roll) = &self.last_roll {
                        ui.label(
                            Type::Heading
                                .rich(ui, format!("= {}", roll.total))
                                .color(colors().text),
                        )
                        .on_hover_text(&roll.line);
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
            let before = self.state.log.len();
            self.run_command(Command::Roll(expression));
            self.last_roll = (self.state.log.len() > before)
                .then(|| self.state.log.last().cloned())
                .flatten()
                .map(|line| LastRoll {
                    total: line
                        .rsplit(" = ")
                        .next()
                        .unwrap_or_default()
                        .trim_end_matches('.')
                        .to_owned(),
                    line,
                });
            self.follow_with_keyboard();
        }
    }

    fn open_conditions_tab(&mut self) {
        self.panel = Panel::Conditions;
        self.close_lookup = true;
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

    /// What was typed in this field beside Notes, when it cannot be a number.
    fn words_in_number_field(
        notes_open: bool,
        ui: &egui::Ui,
        id: egui::Id,
        number: impl Fn(char) -> bool,
    ) -> Option<String> {
        if !notes_open || !ui.ctx().memory(|memory| memory.has_focus(id)) {
            return None;
        }
        ui.input_mut(|i| {
            let typed: String = i
                .events
                .iter()
                .filter_map(|event| match event {
                    egui::Event::Text(text) => Some(text.as_str()),
                    _ => None,
                })
                .collect();
            let words = typed.chars().any(|c| !number(c));
            if words {
                i.events
                    .retain(|event| !matches!(event, egui::Event::Text(_)));
            }
            words.then_some(typed)
        })
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
                            if toggle(ui, name, if found.is_some() { icons::OK } else { icons::ADD }, found.is_some(), name, Density::Read).clicked()
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
                // One level, not a pile of lines. Each step replaces the last.
                let exhausted = p.conditions.iter().position(|c| c.starts_with("Exhaustion"));
                if let Some(at) = exhausted {
                    let level = exhaustion_level(&p.conditions[at]);
                    ui.horizontal(|ui| {
                        ui.label(
                            Type::Body
                                .rich(ui, format!("Exhaustion level {level}"))
                                .color(colors().text),
                        );
                        if Action::quiet("−")
                            .name("Less exhaustion")
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                        {
                            command = Some(if level > 1 {
                                set_exhaustion(index, at, level - 1)
                            } else {
                                Command::RemoveCondition(index, at)
                            });
                        }
                        if Action::quiet("+")
                            .name("More exhaustion")
                            .density(Density::Read)
                            .blocked_if(level >= 6, "Level 6 is the highest.")
                            .show(ui)
                            .clicked()
                        {
                            command = Some(set_exhaustion(index, at, level + 1));
                        }
                    });
                }
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
                            let menu_width = Density::Read.control() + design::S;
                            ui.allocate_ui(vec2((ui.available_width() - menu_width).max(design::L), 0.0), |ui| {
                                ui.add(egui::Label::new(Type::Body.rich(ui, condition).color(colors().text)).wrap());
                            });
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                let menu = ui.menu_button("⋯", |ui| {
                                    if ui.button("Remove").clicked() {
                                        command = Some(Command::RemoveCondition(index, i));
                                        ui.close();
                                    }
                                });
                                design::named(ui, &menu.response, &format!("More actions for {condition}"));
                            });
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
                        // Typed as a level, it is still the one Exhaustion a creature has.
                        let typed = self.condition.trim();
                        command = Some(match exhausted.filter(|_| typed.starts_with("Exhaustion")) {
                            Some(at) => set_exhaustion(index, at, exhaustion_level(typed)),
                            None => Command::Condition(index, self.condition.clone()),
                        });
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

    fn sync_needs_sign_in(&self) -> bool {
        !self.connected && !self.simulate_connection && !self.linked_participants().is_empty()
    }

    fn footer_height(&self) -> f32 {
        FOOTER_HEIGHT
            + if self.sync_needs_sign_in() {
                Type::Small.line()
            } else {
                0.0
            }
    }

    /// Sync and the closing controls. These are kept far from Next turn.
    fn live_footer(&mut self, ui: &mut egui::Ui) -> bool {
        let mut closed = false;
        ui.add_space(design::XS);
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Read.control());
            if self.ending {
                let field =
                    design::text_input(ui, "Outcome", &mut self.outcome, "Outcome, if any", 160.0);
                if std::mem::take(&mut self.focus_outcome) {
                    field.request_focus();
                }
                let enter = field.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
                if Action::destructive("End encounter")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                    || enter
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
                    self.return_focus = self.lookup_open;
                }
                return;
            }
            if self.closing {
                closed = self.close_confirmation(ui, "The run stays saved. Undo and sync stop.");
                return;
            }
            // Five actions share the narrowest row. None of them may lose a letter.
            let compact = ui.available_width() < DOCK_ALL_WORDS_FROM;
            ui.spacing_mut().item_spacing.x = 0.0;
            // Off while sync has work to finish, and the label says so without a hover.
            let blocked = self.close_block();
            if Action::quiet(if blocked.is_some() {
                "Close after sync"
            } else {
                "Close run"
            })
            .name("Close run")
            .density(Density::Read)
            .blocked(blocked)
            .show(ui)
            .clicked()
            {
                self.closing = true;
            }
            if Action::quiet("End encounter…")
                .density(Density::Read)
                .hint("Set an outcome. You can undo this.")
                .show(ui)
                .clicked()
            {
                self.ending = true;
                self.focus_outcome = true;
            }
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                self.save_status(ui);
                let keys = Action::quiet("Keys");
                let keys = if compact {
                    keys
                } else {
                    keys.icon(icons::KEYS)
                };
                if keys
                    .density(Density::Read)
                    .name("Keyboard commands")
                    .selected(self.panel == Panel::Keys)
                    .show(ui)
                    .clicked()
                {
                    // Hidden behind Notes, the DM has not seen them yet.
                    self.panel = if self.panel == Panel::Keys && !self.hide_panel {
                        Panel::Reference
                    } else {
                        Panel::Keys
                    };
                    self.close_lookup = true;
                }
                self.sync_party_button(ui);
            });
        });
        if self.sync_needs_sign_in() {
            ui.label(
                Type::Small
                    .rich(ui, "Sign in to sync")
                    .color(colors().muted),
            );
        }
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
        self.close_dialog(ui, note)
    }

    /// All close paths share the same safe default and separate loss from saving.
    fn close_dialog(&mut self, ui: &egui::Ui, note: &str) -> bool {
        let cannot_save = self.strip().not_saved;
        let stranded = self.stranded_names();
        let lost = self.unsent_only_here_names();
        let block = self.close_block();
        let mut keep = ui.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape));
        let mut close = false;
        let mut save_new = false;
        let title = if cannot_save {
            "This run could not be saved"
        } else if !stranded.is_empty() || block.is_some() {
            "Sync has not finished"
        } else {
            "Close this run?"
        };
        let changes = self.unsaved_summary();
        design::dialog(ui.ctx(), "close_run", title, |ui| {
            if cannot_save {
                ui.add(
                    egui::Label::new(format!("Closing loses unsaved changes: {changes}")).wrap(),
                );
            } else if stranded.is_empty() {
                ui.add(egui::Label::new(note).wrap());
            } else {
                ui.add(egui::Label::new(format!("Changes for {} were not sent to D&D Beyond. The saved run keeps HP, death saves and conditions for the next sync. Undo stops here.", stranded.join(", "))).wrap());
            }
            if !lost.is_empty() {
                ui.add(
                    egui::Label::new(format!(
                        "Spell slots and limited uses not sent for {} are lost.",
                        lost.join(", ")
                    ))
                    .wrap(),
                );
            }
            ui.add_space(design::S);
            ui.horizontal(|ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if cannot_save {
                        save_new = Action::primary("Save as a new run and close")
                            .blocked(block)
                            .show(ui)
                            .clicked();
                    } else if lost.is_empty() {
                        close = Action::secondary("Close now")
                            .blocked(block)
                            .show(ui)
                            .clicked();
                    }
                    let response = Action::secondary("Keep running").show(ui);
                    let id = ui.id().with("close_default_focused");
                    if !ui.data(|data| data.get_temp::<bool>(id).unwrap_or(false)) {
                        response.request_focus();
                        ui.data_mut(|data| data.insert_temp(id, true));
                    }
                    keep |= response.clicked();
                });
            });
            if cannot_save || !lost.is_empty() {
                ui.add_space(design::L);
                close |= Action::destructive("Close and lose changes")
                    .blocked(block)
                    .show(ui)
                    .clicked();
            }
        });
        if keep {
            self.closing = false;
            return false;
        }
        if save_new {
            return match self.save_as_new_run() {
                Ok(()) => self.close_run(),
                Err(error) => {
                    self.error = Some(error);
                    false
                }
            };
        }
        close && (cannot_save || self.close_run())
    }

    fn unsaved_summary(&self) -> String {
        let saved_lines = self
            .storage
            .as_ref()
            .and_then(|s| s.baseline.as_ref())
            .and_then(|bytes| serde_json::from_slice::<super::SavedRun>(bytes).ok())
            .map_or(0, |saved| saved.state.log.len());
        let changes = self.state.log.get(saved_lines..).unwrap_or(&[]);
        match changes {
            [] => "everything since the last save.".to_owned(),
            [one] => one.clone(),
            [first, rest @ ..] => format!("{first} And {} more.", rest.len()),
        }
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
            .hint("Read every linked character's sheet. A value you changed stays, and you choose.")
            .blocked_if(
                !self.connected && !self.simulate_connection,
                "Connect to D&D Beyond in Settings to sync.",
            )
            .show(ui)
            .clicked()
        {
            self.toggle_party_sync();
            self.return_focus = self.lookup_open;
        }
    }

    pub(super) fn toggle_party_sync(&mut self) {
        let plan = self.party_sync_plan();
        self.keep_party_synced = matches!(plan, PartySync::Start(_));
        self.dirty = true;
        match plan {
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
            .filter(|(_, sync)| sync.enabled && sync.unconfirmed())
            .filter_map(|(index, _)| {
                Some((*index, self.state.participants.get(*index)?.name.clone()))
            })
            .collect();
        if names.is_empty() {
            return;
        }
        design::group_label(ui, "D&D Beyond sync");
        ui.add(
            egui::Label::new(
                Type::Small
                    .rich(
                        ui,
                        "Splat and D&D Beyond differ. Choose which values to keep.",
                    )
                    .color(colors().muted),
            )
            .wrap(),
        );
        for (index, name) in names {
            let Some(sync) = self.syncs.get_mut(&index) else {
                continue;
            };
            let conflict = sync.state() == SyncState::Conflict;
            ui.push_id(index, |ui| {
                let word = sync_status(sync.state()).word;
                if conflict {
                    sync.conflicts_for(ui, &name);
                } else {
                    ui.label(
                        Type::Strong
                            .rich(ui, format!("{name} · {word}"))
                            .color(colors().text),
                    );
                }
                if Action::quiet("Stop sync")
                    .density(Density::Read)
                    .name(&format!("Stop sync for {name}"))
                    .hint("Sync stops, and changes not yet confirmed are not sent")
                    .show(ui)
                    .clicked()
                {
                    sync.stop();
                    self.dirty = true;
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
        let title = if matches!(self.panel, Panel::Log | Panel::Encounter | Panel::Groups) {
            "This fight"
        } else {
            &p.name
        };
        ui.add(egui::Label::new(Type::Title.rich(ui, title).color(colors().text)).wrap());
        let participant_tab = !matches!(self.panel, Panel::Log | Panel::Encounter | Panel::Groups);
        if participant_tab && hp == Some(0) {
            self.death_saves(ui, index, successes, failures);
        }
        if let Some(sync) = self
            .syncs
            .get_mut(&index)
            .filter(|s| participant_tab && s.state() == SyncState::Conflict)
        {
            sync.conflicts(ui);
        }
        let reference = if is_sheet { "Sheet" } else { "Stat block" };
        self.panel_tabs(ui, Some(reference), true);
        design::rule(ui);
        self.panel_body(ui, index, self.panel);
    }

    /// One row, with participant controls on the left and fight controls on the right.
    fn panel_tabs(&mut self, ui: &mut egui::Ui, reference: Option<&str>, conditions: bool) {
        let mut participant = Vec::new();
        if let Some(reference) = reference {
            participant.push((Panel::Reference, reference));
        }
        if conditions {
            participant.push((Panel::Conditions, "Conditions"));
        }
        participant.push((Panel::Record, "Correct"));
        let fight = [
            (Panel::Log, "Log"),
            (Panel::Encounter, "Prep"),
            (Panel::Groups, "Arrivals"),
        ];
        let width = |items: &[(Panel, &str)]| {
            items
                .iter()
                .map(|(_, label)| {
                    design::number_width(ui, label, &Type::Strong.font(ui)) + design::L
                })
                .sum::<f32>()
        };
        let fits = width(&participant) + width(&fight) + design::L <= ui.available_width();
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            design::segmented(ui, &mut self.panel, &participant);
            ui.add_space(design::L);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if fits {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        design::segmented(ui, &mut self.panel, &fight);
                    });
                } else {
                    let menu = ui.menu_button(format!("Fight {}", icons::DOWN), |ui| {
                        for (panel, label) in fight {
                            let action = egui::Button::new(label).selected(self.panel == panel);
                            let action = if panel == Panel::Groups {
                                action.shortcut_text("Ctrl+G")
                            } else {
                                action
                            };
                            if ui.add(action).clicked() {
                                self.panel = panel;
                                ui.close();
                            }
                        }
                    });
                    design::named(ui, &menu.response, "Fight menu");
                }
            });
        });
    }

    fn panel_body(&mut self, ui: &mut egui::Ui, index: usize, panel: Panel) {
        match panel {
            Panel::Reference => self.reference_tab(ui, index),
            Panel::Conditions => self.conditions_tab(ui, index),
            Panel::Record => self.record_tab(ui, index),
            Panel::Log => self.log_tab(ui),
            Panel::Keys => self.keys_tab(ui),
            Panel::Encounter => self.notes_tab(ui),
            Panel::Groups => self.groups_tab(ui),
        }
    }

    fn notes_tab(&self, ui: &mut egui::Ui) {
        let (prep, technical) = self.prep();
        egui::ScrollArea::vertical()
            .id_salt("run_encounter_notes")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if prep.is_empty() {
                    design::empty_state(
                        ui,
                        icons::SESSIONS,
                        "No prep notes yet",
                        "Add read-aloud text or notes when you edit the encounter.",
                        |_| {},
                    );
                } else {
                    design::group_label(ui, READ_ALOUD);
                    style::markdown(ui, prep);
                }
                if let Some(technical) = technical {
                    ui.add_space(design::M);
                    design::quiet_section(ui, "Technical details", |ui| {
                        style::markdown(ui, technical);
                    });
                }
            });
    }

    /// The campaign's encounter groups. A group is dragged to the initiative list, or added
    /// with its button, by the rule the campaign chose for initiative.
    /// Whatever joins a fight after it starts: one creature, or a prepared group.
    fn groups_tab(&mut self, ui: &mut egui::Ui) {
        let mut add = None;
        egui::ScrollArea::vertical()
            .id_salt("run_groups")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = design::XS;
                ui.add(
                    egui::Label::new(
                        Type::Small
                            .rich(ui, "Drag a group name to the list, or use Add.")
                            .color(colors().muted),
                    )
                    .wrap(),
                );
                self.just_added(ui);
                self.add_creature_form(ui);
                design::rule(ui);
                add = self.group_list(ui);
            });
        if let Some(id) = add {
            self.add_group_from_panel(id);
        }
    }

    /// The roll and where it placed them. Undo or a later step takes this away.
    fn just_added(&self, ui: &mut egui::Ui) {
        let added = self
            .history_labels
            .last()
            .and_then(|label| label.strip_prefix("add "))
            .map(|group| format!("{}{group} added: ", super::GROUP_ADDED));
        let Some(start) = added else {
            return;
        };
        if let Some(line) = self
            .state
            .log
            .last()
            .filter(|line| line.starts_with(&start) || line.starts_with(super::ONE_ADDED))
        {
            let short: String = line.chars().take(super::GROUP_LINE_LIMIT).collect();
            let shown = if short.len() < line.len() {
                format!("{short}… The Log has the rest.")
            } else {
                short
            };
            design::message(ui, Tone::Success, &shown);
        }
    }

    /// Returns the group the DM chooses to add. Those in tonight's plan come first.
    fn group_list(&mut self, ui: &mut egui::Ui) -> Option<Uuid> {
        let c = colors();
        ui.label(Type::Heading.rich(ui, "Encounter groups").color(c.text));
        let Some(groups) = self.groups.clone() else {
            ui.add(
                egui::Label::new(
                    Type::Body
                        .rich(
                            ui,
                            "This campaign's groups have not loaded yet. If this stays, the Encounters page says why.",
                        )
                        .color(c.muted),
                )
                .wrap(),
            );
            return None;
        };
        if groups.is_empty() {
            design::empty_state(
                ui,
                icons::GROUP,
                "No encounter groups yet",
                "Assemble one in a session's plan, or on Encounters with New group.",
                |_| {},
            );
            return None;
        }
        design::rule(ui);
        let offered = super::OfferedGroups::of(&groups, self.source);
        let in_plan = |group: &Arc<super::RunGroup>| {
            self.planned
                .iter()
                .any(|name| name.eq_ignore_ascii_case(&group.name))
        };
        let (planned, any): (Vec<_>, Vec<_>) = offered.any.iter().cloned().partition(in_plan);
        // A campaign with only groups for any encounter, none of them planned, has nothing to
        // tell apart.
        let labelled = !offered.own.is_empty() || !planned.is_empty() || !offered.other.is_empty();
        let mut add = None;
        for (heading, list) in [
            ("For this encounter", offered.own.as_slice()),
            ("In tonight's plan", planned.as_slice()),
            ("For any encounter", any.as_slice()),
            ("For one encounter", offered.other.as_slice()),
        ] {
            if labelled && !list.is_empty() {
                design::group_label(ui, heading);
            }
            for group in list {
                ui.push_id(group.id, |ui| {
                    if self.group_row(ui, group) {
                        add = Some(group.id);
                    }
                });
                design::rule(ui);
            }
        }
        add
    }

    /// By side, a creature joins its side's initiative with no roll, when the side has one.
    pub(super) fn joins_side(&self, side: &str) -> bool {
        self.initiative.grouping == Grouping::BySide
            && self
                .state
                .participants
                .iter()
                .any(|p| p.npc && p.side == side && p.initiative.is_some())
    }

    /// Returns true when the DM chooses to add this group.
    fn group_row(&mut self, ui: &mut egui::Ui, group: &super::RunGroup) -> bool {
        let c = colors();
        let name = &group.name;
        let creatures = group.creature_count();
        let grip = ui.allocate_response(
            vec2(ui.available_width(), Density::Read.control()),
            egui::Sense::DRAG,
        );
        // A pointer's way to do what the Add button does, so the keyboard does not stop here.
        grip.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Other,
                true,
                format!("Drag {name} to the initiative list"),
            )
        });
        let totals = ui.painter().text(
            grip.rect.right_center(),
            Align2::RIGHT_CENTER,
            format!(
                "{} · {} · {}",
                style::human_label(&group.side),
                design::count(creatures as usize, "member", "members"),
                group.xp_label()
            ),
            Type::Small.font(ui),
            c.muted,
        );
        // A long name stops short of the totals, and its row shows it whole on hover.
        let title = egui::WidgetText::from(
            Type::Strong
                .rich(ui, format!("{} {name}", icons::DRAG))
                .color(c.text),
        )
        .into_galley(
            ui,
            Some(egui::TextWrapMode::Truncate),
            (totals.left() - grip.rect.left() - design::S).max(0.0),
            Type::Strong.font(ui),
        );
        ui.painter().galley(
            pos2(
                grip.rect.left(),
                grip.rect.center().y - title.size().y / 2.0,
            ),
            title,
            c.text,
        );
        let grip = grip.on_hover_cursor(egui::CursorIcon::Grab);
        let context = group
            .encounter_name
            .as_ref()
            .filter(|_| group.encounter != self.source)
            .map_or_else(String::new, |name| format!("For {name}. "));
        let grip = grip.on_hover_text(format!(
            "{name}. {context}{} {}",
            group.notes.trim(),
            self.initiative.grouping.arrival_rule()
        ));
        grip.dnd_set_drag_payload(GroupDrag(group.id));
        if grip.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
            if let Some(pointer) = ui.input(|i| i.pointer.latest_pos()) {
                // On a surface of its own, because the list under it is full of words.
                let painter = ui
                    .ctx()
                    .layer_painter(egui::LayerId::new(egui::Order::Tooltip, grip.id));
                let galley = painter.layout_no_wrap(
                    format!("{} {name}", icons::ADD),
                    Type::Strong.font(ui),
                    c.accent,
                );
                let place = pointer + vec2(design::M, -galley.size().y / 2.0);
                painter.rect_filled(
                    Rect::from_min_size(place, galley.size()).expand(design::XS),
                    design::RADIUS_CONTROL,
                    c.raised,
                );
                painter.galley(place, galley, c.accent);
            }
        }
        let members = group.member_summary();
        let added = self
            .state
            .participants
            .iter()
            .filter_map(|p| p.added_from.as_ref())
            .filter(|from| from.group_id == Some(group.id))
            .map(|from| from.round)
            .max();
        let summary = if let Some(round) = added {
            format!("Added in round {round} · {members}")
        } else if let Some(name) = group
            .encounter_name
            .as_ref()
            .filter(|_| group.encounter != self.source)
        {
            format!("For {name} · {members}")
        } else {
            members
        };
        let mut add = false;
        ui.horizontal(|ui| {
            let controls = design::number_width(ui, "Total", &Type::Small.font(ui))
                + 72.0
                + design::XS
                + design::number_width(ui, "Add", &Type::Strong.font(ui))
                + 2.0 * design::M
                + 3.0 * ui.spacing().item_spacing.x;
            let width = (ui.available_width() - controls).max(design::XXL);
            ui.allocate_ui(vec2(width, Density::Read.control()), |ui| {
                ui.set_min_width(width);
                ui.set_max_width(width);
                ui.add(egui::Label::new(Type::Small.rich(ui, &summary).color(c.muted)).wrap());
            });
            let field = format!("Initiative total for {name}");
            ui.label(Type::Small.rich(ui, "Total").color(c.muted));
            let text = self.group_totals.entry(group.id).or_default();
            // By id, because two groups can have one name.
            let response = design::number_field_with_id(
                ui,
                design::focus_id(&field).with(group.id),
                &field,
                text,
                "Roll",
                72.0,
                Density::Read.control(),
            );
            response.clone().on_hover_text("Blank rolls initiative");
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_description("Blank rolls initiative")
            });
            let total = text.parse::<i64>().ok();
            let enter = response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
            let leader = group.leader();
            let may_roll = !leader
                .as_ref()
                .is_some_and(|leader| self.joins_side(&leader.side));
            let blocked = group.problem.as_deref().or((total.is_none()
                && may_roll
                && leader.is_some_and(|leader| leader.initiative_bonus.is_none()))
            .then_some("The first member has no initiative bonus. Type a total."));
            add = Action::secondary("Add")
                .density(Density::Read)
                .name(&format!("Add {name} to the initiative list"))
                .blocked(blocked)
                .show(ui)
                .clicked()
                || (enter && blocked.is_none());
        });
        if let Some(problem) = &group.problem {
            for fault in problem.lines() {
                ui.add(
                    egui::Label::new(
                        Type::Small
                            .rich(ui, format!("{} {fault}", icons::WARN))
                            .color(c.result.warning),
                    )
                    .wrap(),
                );
            }
            if design::status_link(
                ui,
                Status {
                    icon: icons::EDIT,
                    word: "Edit group",
                    color: c.accent,
                },
            )
            .clicked()
            {
                self.wants_group_editor = Some(group.path.clone());
            }
        }
        add
    }

    /// The same step for the button and for a drop: the total typed beside it, or a roll.
    pub(super) fn add_group_from_panel(&mut self, id: Uuid) {
        let Some(group) = self
            .groups
            .iter()
            .flat_map(|groups| groups.iter())
            .find(|group| group.id == id)
            .cloned()
        else {
            return;
        };
        let total = self
            .group_totals
            .get(&id)
            .and_then(|text| text.parse::<i64>().ok());
        self.run_command(Command::AddGroup(group, total));
        if self.error.is_none() {
            self.group_totals.remove(&id);
            self.return_focus = self.lookup_open;
        }
    }

    /// While a group is held, the initiative list says it can take it.
    fn group_drop_zone(&mut self, ui: &mut egui::Ui, rect: Rect) {
        let Some(held) = egui::DragAndDrop::payload::<GroupDrag>(ui.ctx()) else {
            return;
        };
        ui.painter().rect_stroke(
            rect,
            design::RADIUS_CONTROL,
            Stroke::new(design::FOCUS_WIDTH, colors().accent),
            StrokeKind::Inside,
        );
        let over = ui
            .input(|i| i.pointer.latest_pos())
            .is_some_and(|pointer| rect.contains(pointer));
        if over && ui.input(|i| i.pointer.any_released()) {
            egui::DragAndDrop::clear_payload(ui.ctx());
            self.add_group_from_panel(held.0);
        }
    }

    fn keys_tab(&self, ui: &mut egui::Ui) {
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
                        if self.lookup_open {
                            "Letters do nothing while Notes or Lookup is open. Use the buttons."
                        } else {
                            "Letters work when the amount has focus, or nothing has."
                        },
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
        if definition.path == "Made up in this fight" {
            ui.label(
                Type::Body
                    .rich(ui, "Made up in this fight. It has no stat block.")
                    .color(colors().muted),
            );
            ui.horizontal(|ui| {
                design::stat_tile(
                    ui,
                    "Armor class",
                    &p.armor_class
                        .map_or_else(|| "?".into(), |ac| ac.to_string()),
                );
                design::stat_tile(
                    ui,
                    "Hit points",
                    &p.hp.map_or_else(|| "?".into(), |hp| hp.to_string()),
                );
            });
            return;
        }
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
            if let Some(path) = self
                .storage
                .as_ref()
                .and_then(|storage| crate::party_sync::portrait_path(storage.campaign, record))
            {
                let (rect, _) = ui.allocate_exact_size(
                    egui::Vec2::splat(crate::party::SHEET_PORTRAIT),
                    egui::Sense::empty(),
                );
                crate::art::portrait(ui, rect, Some(&path), &record.definition.splat_name, None);
            }
            changes_block(ui, p, false);
            let freshness = crate::party::freshness(record, self.connected);
            // The sheet is older than what the DM just set here, so say which is which.
            // While sync reads this sheet, the copy's age says nothing about these values.
            let word = if self.syncs.get(&index).is_some_and(|sync| sync.ready()) {
                "Connected · Sync reads this sheet · HP from this run".to_owned()
            } else {
                format!("{} · HP from this run", freshness.word)
            };
            // Beside an open conflict, green words would say the opposite of what matters.
            if !self
                .syncs
                .get(&index)
                .is_some_and(|sync| sync.state() == SyncState::Conflict)
            {
                design::status(
                    ui,
                    Status {
                        icon: freshness.icon,
                        word: &word,
                        color: freshness.color,
                    },
                );
            }
            let spells = crate::party::has_spells(record);
            if !spells && self.sheet_page == 2 {
                self.sheet_page = 0;
            }
            let pages: Vec<_> = SHEET_PAGES
                .iter()
                .copied()
                .filter(|(page, _)| *page != 2 || spells)
                .collect();
            design::segmented(ui, &mut self.sheet_page, &pages);
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
            // By participant: copies of one creature can differ, and each card opens on its own
            // changes, not at the place the last copy was left.
            .id_salt(("run_reference", &p.key))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                changes_block(ui, p, true);
                let links: &dyn view::Links = match &self.content {
                    Some(index) => index.as_ref(),
                    None => &view::NoLinks,
                };
                if definition.path == "Made up in this fight" {
                    design::empty_state(
                        ui,
                        icons::INFO,
                        "Made up in this fight",
                        "It has no stat block.",
                        |_| {},
                    );
                    ui.horizontal(|ui| {
                        design::stat_tile(ui, "AC", &display(p.armor_class));
                        design::stat_tile(ui, "HP", &display(p.hp));
                    });
                } else {
                    let values = view::RunValues {
                        armor_class: p.armor_class,
                        hp: p.max_hp,
                        initiative: p.initiative_bonus,
                        by_hand: p.changes.as_ref().map_or_else(Vec::new, |changes| {
                            changes
                                .overrides
                                .table_bonuses(&changes.base)
                                .into_iter()
                                .filter(|line| {
                                    line.starts_with("Attack") || line.starts_with("Damage")
                                })
                                .collect()
                        }),
                    };
                    link = view::show_run(
                        ui,
                        definition,
                        &view::Origin {
                            path: &definition.path,
                            owner: "Starting snapshot, saved with this run",
                            builtin: false,
                        },
                        links,
                        &values,
                    );
                }
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
                let sheet_decides = self.syncs.get(&index).is_some_and(|sync| sync.ready());
                optional_number(ui, "Current HP", &mut hp, 0..=1_000_000);
                design::locked(ui, sheet_decides.then_some(MAX_FROM_SHEET), |ui| {
                    optional_number(ui, "Maximum HP", &mut max, 1..=1_000_000);
                });
                if sheet_decides {
                    ui.add(
                        egui::Label::new(
                            Type::Small.rich(ui, MAX_FROM_SHEET).color(colors().muted),
                        )
                        .wrap(),
                    );
                }
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
                    // Absent at the start, so no roll was asked for, or a wrong number typed
                    // under pressure. The DM says what it is, and whose turn it is does not change.
                    let (words, color) = if p.initiative.is_none() {
                        (
                            "No initiative yet: last in the order until you set it. The current turn stays.",
                            colors().result.warning,
                        )
                    } else {
                        (
                            "Correct initiative. This row moves, and the current turn stays.",
                            colors().muted,
                        )
                    };
                    ui.add(egui::Label::new(Type::Small.rich(ui, words).color(color)).wrap());
                    let field = format!("Initiative for {}", p.name);
                    let text = self.setup_inputs.entry(index).or_default();
                    let total = text.trim().parse::<i64>().ok();
                    ui.horizontal(|ui| {
                        let response = design::number_field(
                            ui,
                            &field,
                            text,
                            "Init",
                            72.0,
                            Density::Read.control(),
                        );
                        let enter = response.lost_focus()
                            && ui.input(|i| i.key_pressed(Key::Enter));
                        let pressed = Action::secondary("Set initiative")
                            .density(Density::Read)
                            .blocked_if(total.is_none(), "Type a total first.")
                            .show(ui)
                            .clicked();
                        if (pressed || enter)
                            && let Some(total) = total
                        {
                            command = Some(Command::Initiative(index, Some(total)));
                            text.clear();
                            self.return_focus = self.lookup_open;
                        }
                    });
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
                    if Action::secondary("Add note")
                        .name("Add note")
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
                    let sync = self.syncs.entry(index).or_default();
                    let was = sync.enabled;
                    ui.push_id(index, |ui| sync.show(ui));
                    self.dirty |= was != sync.enabled;
                }
            });
        if let Some(command) = command {
            self.run_command(command);
        }
    }

    fn log_tab(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        egui::ScrollArea::vertical()
            .id_salt("run_log")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if self.state.log.is_empty() {
                    design::empty_state(
                        ui,
                        icons::SESSIONS,
                        "No log entries yet",
                        "Begin combat or add a note to record what happens.",
                        |_| {},
                    );
                }
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
            design::Column::panel("run_setup_panel", width)
                .page_keeps(design::RUN_LIST_KEEPS)
                .frame(side_frame())
                .show_inside(ui, |ui| self.setup_panel(ui));
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show_inside(ui, |ui| {
                let size = vec2(self.roster_width(ui), ui.available_height());
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
            .filter(|p| p.initiative.is_none() && p.present)
            .count();
        let missing_hp = self
            .state
            .participants
            .iter()
            .filter(|p| !matches!((p.hp, p.max_hp), (Some(hp), Some(max)) if max > 0 && hp >= 0 && hp <= max))
            .count();
        let blocked = if missing_hp > 0 {
            Some(if self.hide_panel {
                format!("{missing_hp} need valid HP.")
            } else {
                format!("{missing_hp} need valid HP. Open Correct to set it.")
            })
        } else if missing_initiative > 0 {
            Some(format!("{missing_initiative} need initiative."))
        } else {
            None
        };
        let mut command = None;
        let default_roll = self.initiative.grouping;
        let default_hint = format!("Campaign default: {}", default_roll.label());
        let roll_choices = self.roll_choices();
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
                let roll = design::split_action(
                    ui,
                    Action::secondary("Roll NPCs")
                        .name("Roll NPC initiative")
                        .icon(icons::DICE)
                        .density(Density::Glance)
                        .hint(&default_hint),
                    "Roll NPC initiative another way",
                    |ui| {
                        ui.label(Type::Small.rich(ui, "This roll only").color(colors().muted));
                        for mode in &roll_choices {
                            let item = ui.selectable_label(*mode == default_roll, mode.label());
                            if item.clicked() {
                                command = Some(Command::RollNpcs(*mode));
                            }
                        }
                    },
                );
                if roll.clicked() {
                    command = Some(Command::RollNpcs(default_roll));
                }
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    self.run_title(ui, "Setup");
                });
            },
        );
        // Always one line, so the rows below never move during entry (T4.3).
        ui.horizontal(|ui| {
            let tied = self.tied_with_panel_hidden();
            ui.label(match &blocked {
                Some(blocked) => Type::Small
                    .rich(ui, format!("{} {blocked}", icons::WARN))
                    .color(colors().document.unsaved),
                None if tied > 0 => Type::Small
                    .rich(ui, format!("Everyone is ready. {tied} tied."))
                    .color(colors().document.unsaved),
                None => Type::Small
                    .rich(ui, "Everyone is ready.")
                    .color(colors().result.success),
            });
            // Notes hide corrections and tie order, so one link leads there.
            if self.hide_panel && design::link(ui, "Open Correct").clicked() {
                self.panel = Panel::Record;
                self.close_lookup = true;
            }
        });
        design::rule(ui);
        self.error_line(ui);
        if let Some(words) = self.carry_missing() {
            design::message(ui, Tone::Warning, &words);
        }
        if let Some((from, back)) = self.back_to_sheets()
            && design::message_with_action(
                ui,
                Tone::Info,
                &format!("Party HP carries over from {from}."),
                "Use the sheets' HP",
            )
        {
            command = Some(back);
        }
        // Opening beside a panel, the DM can be typing there, and a digit is not an initiative.
        if std::mem::take(&mut self.focus_first_missing) && !self.lookup_open {
            self.focus_field = self
                .state
                .participants
                .iter()
                .position(|p| p.initiative.is_none() && p.present);
        }
        let room = ui.available_height() - self.footer_height() - design::M;
        let rows = ui.allocate_ui_with_layout(
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
        self.group_drop_zone(ui, rows.response.rect);
        design::rule(ui);
        ui.add_space(design::XS);
        ui.horizontal(|ui| {
            if self.closing {
                let note = if self.untouched_setup() {
                    "Nothing was done, so no run is kept."
                } else {
                    "The run stays saved, in setup."
                };
                closed = self.close_confirmation(ui, note);
            } else {
                if Action::quiet(if self.close_block().is_some() {
                    "Close after sync"
                } else {
                    "Close run"
                })
                .name("Close run")
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
                    self.return_focus = self.lookup_open;
                }
                self.undo_reason(ui);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    self.save_status(ui)
                });
            }
        });
        if let Some(command) = command {
            if matches!(command, Command::Begin) {
                self.panel = Panel::Reference;
            }
            let rolled = matches!(command, Command::RollNpcs(_));
            self.run_command(command);
            // Next is the first player still without a number.
            if rolled {
                self.focus_field = self
                    .state
                    .participants
                    .iter()
                    .position(|p| p.initiative.is_none() && p.present);
                if self.focus_field.is_none() {
                    self.return_focus = self.lookup_open;
                }
            }
        }
        closed
    }

    /// Name and initiative first. Enter moves to the next missing total. The rarely
    /// used controls wait behind a row expander.
    fn setup_row(&mut self, ui: &mut egui::Ui, index: usize) -> Option<Command> {
        let mut command = None;
        let p = &self.state.participants[index];
        // What the DM weighs before a fight: who is hurt, and what is still on them.
        let side_icon = match p.side.as_str() {
            "party" => "■",
            "enemies" => "◆",
            _ => "•",
        };
        let mut numbers = format!(
            "{side_icon} {} · HP {}/{}",
            style::human_label(&p.side),
            display(p.hp),
            display(p.max_hp)
        );
        let absent = if p.present {
            String::new()
        } else {
            format!("{} Absent · ", icons::ABSENT)
        };
        if p.temporary_hp > 0 {
            numbers.push_str(&format!(" {} +{}", icons::TEMP_HP, p.temporary_hp));
        }
        // A changed creature's AC may be part of what changed.
        numbers.push_str(&format!(" · AC {}", display(p.armor_class)));
        if let Some((_, word, _)) = hp_word(p) {
            numbers.push_str(&format!(" · {word}"));
        }
        // A changed row has its change to say, and no room left for these.
        if p.changes.is_none() {
            for condition in &p.conditions {
                numbers.push_str(&format!(" · {condition}"));
            }
        }
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
            // Keep the worded toggle in every width; party rows give up their per-row Roll.
            let presence_slot = design::L * 6.0;
            let actions =
                72.0 + if npc { 76.0 } else { 0.0 } + 28.0 + presence_slot + 4.0 * design::XS;
            let name_width = (ui.available_width() - actions).max(110.0);
            let side = "";
            let plain = format!("{absent}{side}{numbers}");
            // Changed values keep their mark beside the setup numbers.
            let detail = if p.changes.is_none() {
                plain
            } else {
                let changed = format!("{} Changed", icons::CHANGED);
                let both = format!("{absent}{side}{changed} · {numbers}");
                let font = Type::Small.font(ui);
                if design::number_width(ui, &both, &font) + 2.0 * design::S <= name_width {
                    both
                } else {
                    format!("{absent}{changed} · {numbers}")
                }
            };
            let row_name = format!("{name} · {detail}");
            let response = ui
                .allocate_ui(vec2(name_width, row), |ui| {
                    let width = name_width - 2.0 * design::S;
                    let title = ui.painter().layout(
                        name.clone(),
                        Type::Body.font(ui),
                        colors().text,
                        width,
                    );
                    let detail =
                        ui.painter()
                            .layout(detail, Type::Small.font(ui), colors().muted, width);
                    let height = row.max(title.size().y + detail.size().y + design::S);
                    design::row(ui, height, selected, &row_name, |ui, rect| {
                        let top = rect.top() + design::XS;
                        ui.painter().galley(
                            pos2(rect.left() + design::S, top),
                            title.clone(),
                            colors().text,
                        );
                        ui.painter().galley(
                            pos2(rect.left() + design::S, top + title.size().y),
                            detail,
                            colors().muted,
                        );
                    })
                })
                .inner;
            if response.clicked() {
                self.selected = index;
                self.return_focus = self.lookup_open;
            }
            let field_name = format!("Initiative for {name}");
            // Beside Notes, a letter goes to the note, and the number stays as it is.
            let in_field = design::focus_id(&field_name);
            if let Some(typed) = Self::words_in_number_field(self.notes_open, ui, in_field, |c| {
                c.is_ascii_digit() || c == '-'
            }) {
                self.for_note = Some(typed);
                self.return_focus = true;
            }
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
            if npc
                && Action::quiet("Roll")
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
            let accessible = format!("{word}: {name}");
            let pressed = toggle(
                ui,
                word,
                icons::ABSENT,
                !present,
                &accessible,
                Density::Scan,
            )
            .on_hover_text(hint)
            .clicked();
            if pressed {
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
                self.return_focus = self.lookup_open;
            }
        });
        if enter {
            // The next row without a total, after this one, then from the top.
            let count = self.state.participants.len();
            self.focus_field = (1..count).map(|step| (index + step) % count).find(|i| {
                let p = &self.state.participants[*i];
                p.initiative.is_none() && p.present
            });
            if self.focus_field.is_none() {
                self.return_focus = self.lookup_open;
            }
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
        let title = if matches!(self.panel, Panel::Log | Panel::Encounter | Panel::Groups) {
            "This fight"
        } else {
            &self.state.participants[self.selected].name
        };
        ui.label(Type::Title.rich(ui, title).color(colors().text));
        self.panel_tabs(ui, None, false);
        design::rule(ui);
        self.sync_block(ui);
        match self.panel {
            Panel::Reference | Panel::Conditions | Panel::Keys => {
                self.notes_tab(ui);
            }
            Panel::Record => {
                let p = &self.state.participants[self.selected];
                ui.label(
                    Type::Small
                        .rich(ui, format!("Side: {}", style::human_label(&p.side)))
                        .color(colors().muted),
                );
                changes_block(ui, p, false);
                design::quiet_section(ui, "Resolve ties", |ui| self.turn_order_preview(ui));
                self.record_tab(ui, self.selected);
            }
            Panel::Log => self.log_tab(ui),
            Panel::Encounter => self.notes_tab(ui),
            Panel::Groups => self.groups_tab(ui),
        }
    }

    /// How many the DM must order, when Notes hide the panel that says so.
    fn tied_with_panel_hidden(&self) -> usize {
        if !self.hide_panel || self.initiative.roster_ties {
            return 0;
        }
        let initiative = |at: usize| {
            self.state
                .order
                .get(at)
                .and_then(|index| self.state.participants[*index].initiative)
        };
        (0..self.state.order.len())
            .filter(|&at| {
                initiative(at).is_some()
                    && ((at > 0 && initiative(at - 1) == initiative(at))
                        || initiative(at + 1) == initiative(at))
            })
            .count()
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
                            "Default NPC roll: {} · {}",
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
        if !self.initiative.roster_ties
            && self.state.order.windows(2).any(|pair| {
                let a = self.state.participants[pair[0]].initiative;
                a.is_some() && a == self.state.participants[pair[1]].initiative
            })
        {
            ui.label(Type::Small.rich(ui, "Tie: choose the order").color(c.muted));
        }
        egui::ScrollArea::vertical()
            .id_salt("run_order_preview")
            .max_height(design::READER_MEASURE / 3.0)
            .auto_shrink([false, true])
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
                    if !up && !down {
                        continue;
                    }
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
                            if !p.present {
                                design::status(
                                    ui,
                                    Status {
                                        icon: icons::ABSENT,
                                        word: "Absent",
                                        color: c.participant.absent,
                                    },
                                );
                            }
                            ui.add(
                                egui::Label::new(Type::Body.rich(ui, &p.name).color(c.text))
                                    .truncate(),
                            );
                            if !self.initiative.roster_ties && (up || down) {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    // Two fixed places, so that a second click on an arrow
                                    // cannot land on the other one and take back the first.
                                    let slot = egui::Vec2::splat(Density::Read.control());
                                    if down {
                                        if design::icon_button(
                                            ui,
                                            icons::MOVE_DOWN,
                                            &format!(
                                                "Move {} after {}",
                                                p.name,
                                                self.state.participants[order[position + 1]].name
                                            ),
                                        )
                                        .clicked()
                                        {
                                            command = Some(Command::Move(position, position + 1));
                                        }
                                    } else {
                                        ui.allocate_space(slot);
                                    }
                                    if up {
                                        if design::icon_button(
                                            ui,
                                            icons::MOVE_UP,
                                            &format!(
                                                "Move {} before {}",
                                                p.name,
                                                self.state.participants[order[position - 1]].name
                                            ),
                                        )
                                        .clicked()
                                        {
                                            command = Some(Command::Move(position, position - 1));
                                        }
                                    } else {
                                        ui.allocate_space(slot);
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
                if Action::primary(if self.close_block().is_some() {
                    "Close after sync"
                } else {
                    "Close run"
                })
                .name("Close run")
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
                        self.return_focus = self.lookup_open;
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
            let conflicts = self.conflict_count();
            let text = if conflicts > 0 {
                format!(
                    "{} to settle. {reason}",
                    design::count(conflicts, "conflict", "conflicts")
                )
            } else {
                reason.to_owned()
            };
            ui.add(egui::Label::new(Type::Small.rich(ui, text).color(c.muted)).wrap());
        }
        if self.closing {
            ui.horizontal(|ui| {
                closed = self.close_confirmation(ui, "The run stays saved.");
            });
        }
        // Beside a panel, the final state comes first.
        if !self.hide_panel {
            let width = design::side_panel_width(ui.ctx().content_rect().width());
            design::Column::panel("run_ended_log", width)
                .page_keeps(design::RUN_LIST_KEEPS)
                .frame(side_frame())
                .show_inside(ui, |ui| {
                    // With choices to make, they have the column, and the log waits behind a
                    // heading. Never a pair of choices cut by a second scroll.
                    let choosing = self.conflict_count() > 0;
                    let room = if choosing {
                        ui.available_height() - Density::Read.control() - design::M
                    } else {
                        ui.available_height() * 0.6
                    };
                    egui::ScrollArea::vertical()
                        .id_salt("run_ended_sync")
                        .max_height(room)
                        .auto_shrink([false, true])
                        .show(ui, |ui| {
                            self.sync_block(ui);
                            if !self.setup_details.is_empty() {
                                design::section(ui, "Prep", |ui| self.notes_tab(ui));
                            }
                        });
                    if choosing {
                        design::section(ui, "Combat log", |ui| self.log_tab(ui));
                    } else {
                        design::group_label(ui, "Combat log");
                        self.log_tab(ui);
                    }
                });
        }
        egui::CentralPanel::default()
            // The panel's drag handle lies on this edge, so the rows stop short of it.
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                right: design::S as i8,
                ..egui::Margin::ZERO
            }))
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
                            let sync = self.row_sync_word(*index);
                            let name = row_name(p, TurnMark::None, sync);
                            let layout = RowLayout::new(
                                ui,
                                ui.available_width(),
                                p,
                                sync,
                                scale,
                                RowFit::Natural,
                                0.0,
                            )
                            .with_art(self.row_art(ui.ctx(), p));
                            design::row(ui, layout.height(scale), false, &name, |ui, rect| {
                                paint_row(ui, rect, p, TurnMark::None, sync, scale, &layout);
                            });
                        }
                    });
            });
        closed
    }
}

/// The level in `Exhaustion (2)`. Plain `Exhaustion` is the first.
fn exhaustion_level(condition: &str) -> u8 {
    condition
        .trim()
        .strip_prefix("Exhaustion (")
        .and_then(|rest| rest.strip_suffix(')'))
        .and_then(|level| level.trim().parse().ok())
        .unwrap_or(1)
}

/// One step of undo: the old level goes, and the new one comes.
fn set_exhaustion(index: usize, at: usize, level: u8) -> Command {
    Command::Several(vec![
        Command::RemoveCondition(index, at),
        Command::Condition(index, format!("Exhaustion ({level})")),
    ])
}

/// The action names what it does, so its label cannot be read as a state.
/// What an encounter or group changed on this creature: the value it replaced and the new
/// one, each marked. It stays out in the open, above the stat block that keeps its own values.
fn changes_block(ui: &mut egui::Ui, p: &Participant, above_stat_block: bool) {
    let c = colors();
    if let Some(added) = &p.added_from {
        ui.add(
            egui::Label::new(
                Type::Small
                    .rich(
                        ui,
                        if added.group.is_empty() {
                            "Added by hand".to_owned()
                        } else {
                            format!("Arrived with {}", added.group)
                        },
                    )
                    .color(c.muted),
            )
            .wrap(),
        );
    }
    let Some(changes) = &p.changes else {
        return;
    };
    design::status(
        ui,
        Status {
            icon: icons::CHANGED,
            word: &p
                .added_from
                .as_ref()
                .filter(|a| !a.group.is_empty())
                .map_or_else(
                    || "Changed".to_owned(),
                    |a| format!("Changed in {}", a.group),
                ),
            color: c.participant.changed,
        },
    );
    let rows = changes.overrides.rows(&changes.base);
    change_rows(ui, &rows);
    let note = (!above_stat_block && rows.iter().any(|row| row.by_hand)).then_some(BY_HAND);
    if let Some(note) = note {
        ui.add(egui::Label::new(Type::Small.rich(ui, note).color(c.muted)).wrap());
    }
    ui.add_space(design::S);
}

/// Each change, with the value it replaces. The editors and the run say it the same way.
pub(crate) fn change_rows(ui: &mut egui::Ui, rows: &[Row]) {
    let c = colors();
    // What the DM must do comes first, in larger type. The rest the run has done.
    let (by_hand, applied): (Vec<&Row>, Vec<&Row>) = rows.iter().partition(|row| row.by_hand);
    for row in by_hand.into_iter().chain(applied) {
        let font = if row.by_hand {
            Type::Body.font(ui)
        } else {
            Type::Small.font(ui)
        };
        let mut job = egui::text::LayoutJob::default();
        let mut add = |text: &str, color: egui::Color32| {
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
        add(&format!("{}: ", row.label), c.muted);
        add(&row.base, c.muted);
        add(" → ", c.muted);
        add(&row.changed, c.participant.changed);
        if row.by_hand {
            add(" ·\u{a0}add\u{a0}by\u{a0}hand", c.result.warning);
        }
        ui.add(egui::Label::new(job).wrap());
    }
}

fn presence_action(present: bool) -> (&'static str, &'static str) {
    if present {
        (
            "Absent",
            "Skip this participant's turns until marked present",
        )
    } else {
        ("Absent", "Give this participant turns again")
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
    if let Some(changes) = &p.changes {
        parts.push("Changed".into());
        parts.extend(changes.overrides.table_bonuses(&changes.base));
    }
    if let Some(sync) = sync {
        parts.push(sync_status(sync).word.into());
    }
    parts.join(", ")
}

#[cfg(test)]
pub(super) fn row_words_for_test(p: &Participant) -> String {
    row_name(p, TurnMark::None, None)
}

/// How much height a row may use.
#[derive(Clone, Copy, PartialEq)]
enum RowFit {
    Natural,
    /// The list needs the height, so a row gives up its padding too.
    Compressed,
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
const PICTURE_GAP: f32 = 6.0;

struct RowLayout {
    /// The participant's picture. A row with none shows the first letter of the name.
    art: Option<PathBuf>,
    /// The width of the picture's column.
    picture: f32,
    /// Where the second line starts.
    second_left: f32,
    /// The name at this row's width. A long name wraps and the row grows.
    name: Arc<Galley>,
    second_line: Option<Arc<Galley>>,
    /// Where each condition is on the second line, in characters.
    condition_spans: Vec<(String, std::ops::Range<usize>)>,
    initiative_span: Option<std::ops::Range<usize>>,
    /// The row has no room for a state column. Its state and sync words open the second
    /// line, so that the name stays whole.
    inline_state: bool,
    /// A name-only row in a list that needs the height.
    tight: bool,
    fit: RowFit,
    bar_inset: f32,
    /// The compressed list's shared row height, once any spare room is shared out.
    even: f32,
    initiative_right: f32,
    name_left: f32,
    ac_right: f32,
    temp_right: f32,
    hp_right: f32,
}

fn header_height(scale: RunScale) -> f32 {
    (scale.hp * 1.6).max(36.0)
}

impl RowLayout {
    fn with_art(mut self, art: Option<PathBuf>) -> Self {
        self.art = art;
        self
    }

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
        // The smallest scale has one set of columns, compressed or not, so that a list that
        // starts to compress does not move its names sideways.
        let narrow = fit == RowFit::Compressed || scale.row <= design::RUN_COMPACT.row;
        // The current-row bar, the gap after it, and room for the word `TURN` at this size. In
        // narrow columns the word can sit on the number's line, so that a row's height never
        // depends on whose turn it is.
        let word = scale.mark * 3.2;
        let initiative_right = 6.0 + design::XS + if narrow { word + scale.stat } else { word };
        let gap = if narrow { 6.0 } else { 12.0 };
        // Its bar starts later, still under the digits, so its second line has the width that
        // the turn word took and a condition does not wrap again.
        let bar_inset = if narrow { 8.0 } else { 0.0 };
        // Compressed rows have a roster to fit, so the picture gives up what it can.
        let picture = if narrow {
            scale.name * 1.25
        } else {
            (scale.row - 10.0).min(scale.name * 1.75)
        };
        let name_left = initiative_right + gap + scale.name * 0.7 + picture + PICTURE_GAP;
        let hp_width = scale.hp * 1.9 + scale.hp_max * 2.0 + 4.0;
        let state_width = (scale.mark * 8.5).clamp(84.0, 118.0);
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
        // In a compressed list it starts under the picture, as the row has no height to spare.
        let second_left = if narrow {
            name_left - picture - PICTURE_GAP
        } else {
            name_left
        };
        job.wrap.max_width = (temp_right + bar_inset - second_left).max(1.0);
        // With HP in what a DM looks for: as strong as the state words, and stronger than AC.
        // The compact scale keeps body weight, so a wrapped line costs no row its place.
        let role = if narrow { Type::Body } else { Type::Strong };
        let font = role.sized(ui, scale.condition);
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
            let start = job.text.chars().count();
            job.append(
                text,
                0.0,
                egui::TextFormat {
                    font_id: font.clone(),
                    color,
                    ..Default::default()
                },
            );
            start..job.text.chars().count()
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
        let initiative_span =
            (p.present && p.initiative.is_none()).then(|| add("No initiative", c.accent));
        if p.concentration {
            add(
                &format!("{} Conc", icons::CONCENTRATING),
                c.participant.concentrating,
            );
        }
        let condition_spans = p
            .conditions
            .iter()
            .map(|condition| {
                let shown = if condition.split_whitespace().count() > 4 {
                    format!(
                        "{} more",
                        condition
                            .split_whitespace()
                            .take(4)
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                } else {
                    condition.clone()
                };
                (condition.clone(), add(&shown, c.participant.condition))
            })
            .collect();
        // A side of its own needs its word: a name and a number can look like another side's.
        if !["party", "enemies"].contains(&p.side.as_str()) {
            add(&style::human_label(&p.side), c.side.other);
        }
        // A change is never hidden. What the run cannot apply is on the row, for the roll.
        // A number stays with its word when the line wraps.
        if let Some(changes) = &p.changes {
            add(
                &format!("{}\u{a0}Changed", icons::CHANGED),
                c.participant.changed,
            );
            for bonus in changes.overrides.table_bonuses(&changes.base) {
                add(&bonus.replace(' ', "\u{a0}"), c.participant.changed);
            }
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
            art: None,
            picture,
            second_left,
            name,
            even: 0.0,
            bar_inset,
            second_line,
            condition_spans,
            initiative_span,
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

    /// The top of the second line in a row at this place.
    fn second_y(&self, scale: RunScale, rect: Rect) -> f32 {
        rect.top()
            + self.line_y(scale, rect.height())
            + self.name_line(scale) / 2.0
            + self.name_extra()
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
            return (name_line + self.name_extra() + second + design::HAIRLINE_WIDTH)
                .max(self.even);
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

/// Each official condition on a row opens its rule, beside the run. Returns the entry the
/// DM asked for. Words the DM typed for another effect have no rule to open.
fn condition_links(
    ui: &mut egui::Ui,
    rect: Rect,
    key: &str,
    layout: &RowLayout,
    scale: RunScale,
    content: Option<&crate::lookup::ContentIndex>,
) -> Option<usize> {
    let galley = layout.second_line.as_ref()?;
    let origin = pos2(
        rect.left() + layout.second_left,
        layout.second_y(scale, rect),
    );
    let mut opened = None;
    for (position, (condition, span)) in layout.condition_spans.iter().enumerate() {
        let entry = content.and_then(|content| content.condition_rule(condition));
        let custom = condition.split_whitespace().count() > 4;
        // Wrapped official conditions remain readable; their rules still open in Lookup.
        if !custom && galley.rows.len() > 1 {
            continue;
        }
        if entry.is_none() && !custom {
            continue;
        }
        let start = galley.pos_from_cursor(egui::text::CCursor::new(if custom {
            span.end.saturating_sub(4)
        } else {
            span.start
        }));
        let end = galley.pos_from_cursor(egui::text::CCursor::new(span.end));
        if start.top() != end.top() {
            continue;
        }
        let word = Rect::from_min_max(start.min, pos2(end.left(), start.bottom()))
            .translate(origin.to_vec2());
        // The words are smaller than a target may be, so the target grows, inside its row.
        let bottom = (word.center().y + 12.0).min(rect.bottom());
        let target =
            Rect::from_min_max(pos2(word.left(), bottom - 24.0), pos2(word.right(), bottom));
        let id = ui.id().with(("condition_rule", key, position));
        let response = ui
            .interact(target, id, egui::Sense::click())
            .on_hover_text(format!("Read the rule for {condition}"))
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        let name = if custom {
            format!("More about {condition}")
        } else {
            format!("Rule: {condition}")
        };
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Link, ui.is_enabled(), &name)
        });
        ui.painter().hline(
            word.x_range(),
            word.bottom(),
            Stroke::new(
                1.0,
                if response.hovered() {
                    colors().focus
                } else {
                    colors().participant.condition.gamma_multiply(0.5)
                },
            ),
        );
        if response.has_focus() {
            design::focus_ring(ui, target);
        }
        if response.clicked() {
            opened = Some(entry.unwrap_or(usize::MAX));
        }
    }
    opened
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
    let second_y = layout.second_y(scale, rect);
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
        &p.initiative
            .map_or_else(|| "—".to_owned(), |n| n.to_string()),
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
        pos2(
            x + layout.name_left - layout.picture - PICTURE_GAP - scale.name * 0.6,
            line_y,
        ),
        &p.side,
        scale.name * 0.55,
    );
    let name_left = x + layout.name_left;
    // A compressed row can be shorter than the picture's column is wide.
    crate::art::portrait(
        ui,
        Rect::from_center_size(
            pos2(
                name_left - PICTURE_GAP - layout.picture / 2.0,
                if layout.second_left < layout.name_left {
                    line_y
                } else {
                    rect.center().y
                },
            ),
            egui::Vec2::splat(layout.picture.min(rect.height() - 4.0)),
        ),
        layout.art.as_deref(),
        &p.name,
        (!p.present).then_some(c.participant.absent),
    );
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
        if p.present { c.muted } else { text },
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
        pos2(x + layout.temp_right + layout.bar_inset + 4.0, bar_top),
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
            pos2(x + layout.second_left, second_y),
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

/// A number that can be unknown. `Mark unknown` clears it, away from the value,
/// so that a click meant for the number cannot lose it.
fn optional_number(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<i64>,
    range: std::ops::RangeInclusive<i64>,
) {
    // Its words stay on one line, in any width of panel.
    ui.horizontal(|ui| {
        ui.add_sized(
            vec2(design::L * 6.0, Density::Read.control()),
            egui::Label::new(Type::Body.rich(ui, label).color(colors().text)),
        );
        if let Some(n) = value {
            let response = ui.add(egui::DragValue::new(n).range(range));
            design::named(ui, &response, label);
            if Action::quiet("Mark unknown")
                .name(&format!("Mark {label} unknown"))
                .density(Density::Read)
                .show(ui)
                .clicked()
            {
                *value = None;
            }
        } else {
            ui.label(Type::Small.rich(ui, "Unknown").color(colors().muted));
            if Action::quiet("Enter")
                .name(&format!("Enter {label}"))
                .density(Density::Read)
                .show(ui)
                .clicked()
            {
                *value = Some(*range.start().max(&0));
            }
        }
    });
}
