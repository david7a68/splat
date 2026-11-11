//! The run's three views: setup, live and ended. Prominence follows the brief:
//! round and current turn, then every row's HP and conditions, then the actions.
//! Rows never move when the selection changes: the entry controls have a fixed dock.
use super::{
    Command, MAX_FROM_SHEET, NOBODY_PRESENT, Panel, Participant, PartySync, Phase, Run,
    SAVED_HP_USED, SHEET_SOURCE, display, mode_label,
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
use splat::content::{Entry, Reference};
use std::{path::PathBuf, sync::Arc};
use uuid::Uuid;

/// What is held while an encounter group is dragged to the initiative list.
struct GroupDrag(Uuid);

const AMOUNT: &str = "Amount";
const READ_ALOUD: &str = "Read-aloud and notes";
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
/// A long name wraps to at most this many lines; the last one ends in `…`.
const NAME_LINES: usize = 3;
/// Run toggles use a selected surface; the turn action keeps the accent fill.
pub(crate) fn toggle(
    ui: &mut egui::Ui,
    label: &str,
    icon: &str,
    selected: bool,
    name: &str,
    density: Density,
) -> egui::Response {
    let button = egui::Button::new(
        Type::Body
            .rich(ui, format!("{icon} {label}"))
            .color(colors().text),
    )
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

/// Blank means roll; invalid text must never silently request a roll.
fn arrival_total(text: &str) -> Result<Option<i64>, &'static str> {
    let text = text.trim();
    if text.is_empty() {
        Ok(None)
    } else {
        text.parse()
            .map(Some)
            .map_err(|_| "Enter a whole number, or clear the field to roll.")
    }
}

fn arrival_add_word(group: &super::RunGroup) -> String {
    if group.warnings.is_empty() {
        "Add".to_owned()
    } else {
        format!("Add {}", group.joinable_count())
    }
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
            word: "Reading sheet…",
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
            icon: icons::SYNC_OFF,
            word: "Sync stopped",
            color: colors().sync.pending,
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

/// A panel beside the run keeps only a gutter from what it sits beside. It is `L`, because a
/// quiet action at its text edge (Prep's `Edit encounter`) reaches into it, never into the
/// splitter.
fn side_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(colors().page)
        .inner_margin(egui::Margin {
            left: design::L as i8,
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
const KEYS: [(&str, &str); 13] = [
    ("N", "Next turn"),
    ("Enter", "Damage the selected rows by the amount"),
    ("Shift+Enter", "Heal the selected rows by the amount"),
    ("Ctrl+click", "Add a row to the targets of one amount"),
    ("Escape", "Select the current turn again"),
    ("U or Ctrl+Z", "Undo"),
    ("Up, Down", "Select another row"),
    ("A", "Focus the amount"),
    ("C", "Conditions"),
    (
        "Ctrl+1 to 9",
        "Arm an action; click a row to target it; Escape stops",
    ),
    ("Ctrl+G", "Add to fight"),
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
        self.selection_left_save();
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
        // Escape in a row's initiative field closes the field and nothing else. The field
        // has lost focus by now, so the check above does not keep the key from here.
        if self.live_initiative.is_some()
            && ui.input_mut(|i| i.consume_key(Modifiers::NONE, Key::Escape))
        {
            self.live_initiative = None;
            return;
        }
        if self.disarm_on_escape(ui) {
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
        // Ctrl+1 to Ctrl+9 arm the panel's actions, as a game's action bar does.
        const NUMBERS: [Key; 9] = [
            Key::Num1,
            Key::Num2,
            Key::Num3,
            Key::Num4,
            Key::Num5,
            Key::Num6,
            Key::Num7,
            Key::Num8,
            Key::Num9,
        ];
        let number = ui.input_mut(|i| {
            NUMBERS
                .iter()
                .position(|key| i.consume_key(Modifiers::COMMAND, *key))
        });
        if let Some(at) = number {
            self.arm_by_number(at + 1);
        }
    }

    fn undo_action<'a>(&'a self, label: &'a str) -> Action<'a> {
        let action = Action::secondary(label)
            .icon(icons::UNDO)
            .density(Density::Read);
        let action = if matches!(self.state.phase, Phase::Running) && !self.lookup_open {
            action.key("U")
        } else {
            action
        };
        action.blocked_if(self.history.is_empty(), self.empty_undo_reason())
    }

    fn empty_undo_reason(&self) -> &str {
        if self.undo_stopped {
            self.undo_boundary
                .as_deref()
                .unwrap_or("Nothing to undo since the sheets changed")
        } else if self.reopened {
            "Nothing to undo since reopening"
        } else {
            "Nothing to undo"
        }
    }

    fn empty_undo(&self, ui: &mut egui::Ui) -> bool {
        let full = self.empty_undo_reason();
        let action = self.undo_action("Undo");
        let room = ui.available_width() - action.natural_width(ui) - design::S;
        let shown = if design::number_width(ui, full, &Type::Small.font(ui)) <= room {
            full
        } else {
            "Nothing to undo"
        };
        let clicked = action
            .name(&format!("Undo · {full}"))
            .hint(full)
            .show(ui)
            .clicked();
        if design::number_width(ui, shown, &Type::Small.font(ui)) <= room {
            let reason = ui.label(Type::Small.rich(ui, shown).color(colors().muted));
            design::named(ui, &reason, full);
            reason.on_hover_text(full);
        }
        clicked
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
        // The choices start at the message text, like every other message with choices.
        let choice = design::message_with_actions(ui, Tone::Error, super::DISK_CONFLICT, |ui| {
            ui.add_space(design::XS);
            ui.horizontal(|ui| {
                // Neither action is filled: the safe one comes first, and the red one keeps an
                // `L` gap.
                let save_new = Action::secondary("Save as a new run")
                    .density(Density::Read)
                    .hint("Keep the changed file as it is, and save this run beside it")
                    .show(ui)
                    .clicked();
                // egui adds its item spacing too, so the whole gap is exactly `L`.
                ui.add_space(design::L - ui.spacing().item_spacing.x);
                let replace = Action::destructive("Replace the file with this run")
                    .density(Density::Read)
                    .hint("The changes in the file on disk are lost")
                    .show(ui)
                    .clicked();
                (save_new, replace)
            })
            .inner
        });
        let outcome = match choice {
            (true, _) => Some(self.save_as_new_run()),
            (_, true) => Some(self.keep_this_run()),
            _ => None,
        };
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
                    let roster_width = size.x;
                    ui.spacing_mut().item_spacing.y = 0.0;
                    // The rows' scale decides the header's size, and needs only a close
                    // idea of the room the header will leave. The bar of actions above
                    // the dock does not change it, so Lookup does not resize the rows.
                    let scale = RunScale::fit(
                        ui.available_height()
                            - header_height(design::RUN_REGULAR)
                            - 32.0
                            - DOCK_HEIGHT
                            - self.footer_height(),
                        ui.available_width(),
                        self.state.order.len(),
                    );
                    let sheet_notice_in_header = self.live_header(ui, scale);
                    let stat_error = self
                        .error
                        .as_ref()
                        .filter(|error| error.contains("stat block could not be read"))
                        .cloned();
                    if stat_error.is_none() {
                        self.error_line(ui);
                    }
                    let room = ui.available_height()
                        - DOCK_HEIGHT
                        - self.strip_height()
                        - self.footer_height();
                    let error_room = if stat_error.is_some() && !self.has_disk_conflict() {
                        Density::Read.control() + design::XS
                    } else {
                        0.0
                    };
                    let room = room - error_room;
                    // A message goes in the room under the list, so it moves no row while
                    // the DM's eye is on them. The next action clears it. A long one wraps.
                    let notice = self
                        .notice
                        .as_ref()
                        .filter(|_| !sheet_notice_in_header)
                        .map(|detail| {
                            let summary = self
                                .notice_summary
                                .clone()
                                .unwrap_or_else(|| detail.clone());
                            let boundaries: Vec<_> = self
                                .state
                                .participants
                                .iter()
                                .filter(|p| {
                                    detail.contains(&format!("Undo for {} stops here", p.name))
                                })
                                .map(|p| {
                                    format!(
                                        "Undo for {} stops here",
                                        p.name.split_whitespace().next().unwrap_or(&p.name)
                                    )
                                })
                                .collect();
                            let full = format!("{summary} · {}", boundaries.join(" · "));
                            if !boundaries.is_empty()
                                && design::number_width(ui, &full, &Type::Small.font(ui))
                                    <= ui.available_width() - design::L * 6.0
                            {
                                full
                            } else {
                                summary
                            }
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
                    let notice_height = if compact || self.notice_summary.is_some() {
                        MESSAGE_LINE
                    } else {
                        notice_height
                    };
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
                            Layout::left_to_right(if compact || self.notice_summary.is_some() {
                                Align::Center
                            } else {
                                Align::TOP
                            }),
                            |ui| {
                                ui.set_min_height(notice_height);
                                let words = self
                                    .notice_summary
                                    .clone()
                                    .filter(|summary| {
                                        design::number_width(ui, summary, &Type::Small.font(ui))
                                            <= ui.available_width() - design::L * 6.0
                                    })
                                    .unwrap_or_else(|| {
                                        format!(
                                            "{} changed",
                                            design::count(self.notice_changes, "sheet", "sheets")
                                        )
                                    });
                                let shown = if self.notice_summary.is_some()
                                    || (compact
                                        && design::number_width(ui, &notice, &Type::Small.font(ui))
                                            > ui.available_width() - design::L * 6.0)
                                {
                                    words.as_str()
                                } else {
                                    notice.as_str()
                                };
                                // Centre a compact message with Log and Dismiss. A wrapped
                                // notice keeps its top alignment and its reserved height.
                                // small_message draws its bar and icon before the words.
                                let lead = design::S
                                    + design::HAIRLINE_WIDTH
                                    + design::number_width(ui, icons::WARN, &Type::Small.font(ui))
                                    - design::XS;
                                let text_room =
                                    ui.available_width() - design::L * 6.0 - design::S - lead;
                                let fitted = sheet_notice_words(
                                    ui,
                                    self.notice_summary.as_deref().unwrap_or(shown),
                                    &Type::Small.font(ui),
                                    text_room,
                                );
                                let shown = if self.notice_summary.is_some() {
                                    fitted.as_str()
                                } else {
                                    shown
                                };
                                let response = ui
                                    .allocate_ui_with_layout(
                                        vec2(
                                            text_room,
                                            if compact || self.notice_summary.is_some() {
                                                Type::Small.line()
                                            } else {
                                                notice_height
                                            },
                                        ),
                                        Layout::left_to_right(Align::TOP),
                                        |ui| {
                                            if self.notice_summary.is_some() {
                                                design::small_message(ui, Tone::Warning, shown)
                                            } else {
                                                ui.horizontal_top(|ui| {
                                                    ui.label(
                                                        Type::Small
                                                            .rich(ui, icons::SUCCESS)
                                                            .color(colors().result.success),
                                                    );
                                                    ui.add(
                                                        egui::Label::new(
                                                            Type::Small
                                                                .rich(ui, shown)
                                                                .color(colors().result.success),
                                                        )
                                                        .wrap(),
                                                    )
                                                })
                                                .inner
                                            }
                                        },
                                    )
                                    .inner;
                                let detail = self.notice.as_deref().unwrap_or(shown);
                                design::named(ui, &response, detail);
                                response.on_hover_text(detail);
                                let log = log_link(ui);
                                design::named(ui, &log, "Log");
                                if log.clicked() {
                                    self.panel = Panel::Log;
                                    self.close_lookup = true;
                                }
                                let dismiss =
                                    design::icon_button(ui, icons::CLOSE, "Dismiss this message")
                                        .clicked();
                                if dismiss {
                                    self.notice = None;
                                    self.return_focus = self.lookup_open;
                                }
                            },
                        );
                    }
                    if let Some(error) = stat_error.as_ref().filter(|_| error_room > 0.0) {
                        let first = error
                            .split_once(" Its row")
                            .map_or(error.as_str(), |(first, _)| first);
                        ui.horizontal(|ui| {
                            let tone = colors().result.error;
                            ui.label(Type::Small.rich(ui, icons::ERROR).color(tone));
                            let summary = format!(
                                "{} · Repair in Campaign → Packages",
                                first.trim_end_matches('.')
                            );
                            let detail = format!(
                                "{first} Its row still works. Repair in Campaign → Packages."
                            );
                            let fitted = design::words_that_fit(
                                ui,
                                &summary,
                                &Type::Small.font(ui),
                                ui.available_width() - Density::Read.control() - design::S,
                            );
                            let response = design::small_link(ui, &fitted);
                            design::named(ui, &response, &detail);
                            if response.on_hover_text(&detail).clicked() {
                                self.wants_packages = true;
                            }
                            if design::icon_button(ui, icons::CLOSE, "Dismiss this error").clicked()
                            {
                                self.error = None;
                            }
                        });
                    }
                    // Each control band starts with fresh bounds. A child above can widen
                    // its parent's minimum rect, which set_width cannot shrink again.
                    // With the panel hidden, its action tiles keep one line above the dock.
                    if self.hide_panel {
                        let strip_rect = Rect::from_min_size(
                            ui.cursor().min,
                            vec2(roster_width, super::rolling::STRIP_HEIGHT),
                        );
                        ui.scope_builder(egui::UiBuilder::new().max_rect(strip_rect), |ui| {
                            ui.set_min_height(super::rolling::STRIP_HEIGHT);
                            self.compact_actions(ui);
                        });
                    }
                    let dock_rect =
                        Rect::from_min_size(ui.cursor().min, vec2(roster_width, DOCK_HEIGHT));
                    ui.scope_builder(egui::UiBuilder::new().max_rect(dock_rect), |ui| {
                        self.dock(ui)
                    });
                    let footer_rect = Rect::from_min_size(
                        ui.cursor().min,
                        vec2(roster_width, self.footer_height()),
                    );
                    closed = ui
                        .scope_builder(egui::UiBuilder::new().max_rect(footer_rect), |ui| {
                            self.live_footer(ui)
                        })
                        .inner;
                });
            });
        closed
    }

    /// The round and the turn are the strongest things on the page, above any row, and
    /// they grow with the rows. Next up is the only name that may be off the roster's view.
    fn live_header(&mut self, ui: &mut egui::Ui, scale: RunScale) -> bool {
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
                // As the strip pairs Back to run with Next turn: one height, one small gap.
                ui.add_space(design::S);
                // A narrow roster keeps its room for the turn's name.
                let words = ui.max_rect().width() >= DOCK_ALL_WORDS_FROM;
                if self.add_to_fight(words, Density::Glance).show(ui).clicked() {
                    self.show_arrivals();
                }
                ui.add_space(design::S);
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    let response = ui.label(
                        Type::Round
                            .rich(ui, self.state_words(false))
                            .font(Type::Round.sized(ui, turn_size))
                            .color(c.text),
                    );
                    design::named(
                        ui,
                        &response,
                        &format!("Run · {} · Round {}", self.name, self.state.round),
                    );
                    ui.add_space(design::S);
                    // Drop the turn suffix before shortening the identifying name.
                    let room = ui.available_width() - design::HAIRLINE_WIDTH;
                    let least = (scale.name + design::HAIRLINE_WIDTH).min(turn_size);
                    let full = format!("{name}’s turn");
                    let font = Type::Name.sized(ui, turn_size);
                    let fit = |text: &str| {
                        (turn_size * room / design::number_width(ui, text, &font)).min(turn_size)
                    };
                    let (text, size) = if fit(&full) >= least {
                        (full.clone(), fit(&full))
                    } else if fit(name) >= least {
                        (name.to_owned(), fit(name))
                    } else {
                        let font = Type::Name.sized(ui, least);
                        (dock_name(ui, name, &font, room), least)
                    };
                    let response = ui.add(
                        egui::Label::new(
                            Type::Name
                                .rich(ui, text)
                                .font(Type::Name.sized(ui, size))
                                .color(c.turn.current),
                        )
                        .truncate(),
                    );
                    design::named(ui, &response, &full);
                    response.on_hover_text(&full);
                });
            },
        );
        ui.add_space(6.0);
        // Measure the notice first; Undo takes the remaining room on line two.
        let undo_label = self.undo_label();
        let mut undo = false;
        let mut review_conflict = false;
        let party_sync_off = self.party_sync_off();
        let needs_connection = strip.sync_stopped || (self.sync_needs_sign_in() && !party_sync_off);
        let offline_color = self
            .account_status
            .map_or(c.connection.offline, |status| status.color);
        let sync_alert = if let Some(name) = &strip.conflict {
            Some(SyncAlert::new(
                icons::WARN,
                &format!("Sync conflict: {name}"),
                &format!("Conflict: {name}"),
                c.sync.conflict,
            ))
        } else if strip.sync_stopped {
            Some(SyncAlert::new(
                icons::OFFLINE,
                "Signed out of D&D Beyond",
                "Signed out",
                offline_color,
            ))
        } else if self.sync_needs_sign_in() && !party_sync_off {
            Some(SyncAlert::new(
                icons::SYNC_OFF,
                "Not connected",
                "Not connected",
                offline_color,
            ))
        } else if !strip.unsent.is_empty() {
            Some(SyncAlert::new(
                icons::WARN,
                "Changes kept in Splat, not sent",
                "Not sent",
                c.result.warning,
            ))
        } else if let Some(name) = &strip.sync_error {
            Some(SyncAlert::new(
                icons::ERROR,
                &format!("Sync error: {name}"),
                "Sync error",
                c.result.error,
            ))
        } else if party_sync_off {
            Some(SyncAlert::new(
                icons::SYNC_OFF,
                "Sync off for the party",
                "Sync off for the party",
                c.muted,
            ))
        } else {
            None
        };
        let mut sheet_notice_in_header = false;
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), Density::Read.control()),
            Layout::right_to_left(Align::Center),
            |ui| {
                let font = Type::Small.font(ui);
                let notice = if self.recording_notice.is_some() && strip.conflict.is_none() {
                    format!(
                        "{} Recording silence · {}",
                        icons::WARN,
                        sync_alert.as_ref().map_or("", |alert| alert.short.as_str())
                    )
                } else if let Some(alert) = &sync_alert {
                    // Measure only what is painted: no space after a notice with no link.
                    let link = if needs_connection {
                        "Connect to sync"
                    } else if self.hide_panel && strip.conflict.is_some() {
                        "Review"
                    } else {
                        ""
                    };
                    if link.is_empty() {
                        alert.text.clone()
                    } else {
                        format!("{} {link}", alert.text)
                    }
                } else if nobody_present {
                    format!("{} No participants are present", icons::WARN)
                } else {
                    strip
                        .next
                        .as_ref()
                        .map_or(String::new(), |next| format!("Next: {next}"))
                };
                let action_font = Type::Strong.font(ui);
                let from_sheet = self.undo_from_sheet();
                let fitted = undo_words(
                    ui,
                    &undo_label,
                    from_sheet,
                    &self.state.participants,
                    &action_font,
                    f32::INFINITY,
                );
                let mut minimum = fitted.clone();
                if let Some(target) = self
                    .state
                    .participants
                    .iter()
                    .filter(|p| minimum.contains(&p.name))
                    .max_by_key(|p| p.name.len())
                {
                    minimum = minimum.replacen(
                        &target.name,
                        &dock_name(ui, &target.name, &action_font, design::L * 5.0),
                        1,
                    );
                }
                // The shortest target Undo keeps: a sheet's step also gives up its source.
                let shortest = minimum
                    .strip_prefix("Undo: ")
                    .and_then(|action| action.strip_prefix(SHEET_SOURCE))
                    .filter(|_| from_sheet)
                    .map_or_else(|| minimum.clone(), |action| format!("Undo: {action}"));
                let target_reserve = self.undo_action(&shortest).natural_width(ui) + design::XS;
                let minimum = self.undo_action(&minimum).natural_width(ui)
                    + design::XS
                    + if self.history.is_empty() {
                        design::number_width(ui, "Nothing to undo", &font) + design::S
                    } else {
                        0.0
                    };
                // A state notice stays whole; Undo then shortens its target, and shows the word
                // Undo alone only when no target fits. Only the Next notice gives way to Undo's
                // shortened target. Undo keeps its full accessible name and hover text.
                let state_notice =
                    sync_alert.is_some() || self.recording_notice.is_some() || nobody_present;
                let silence_beside_sync_off = party_sync_off && self.recording_notice.is_some();
                let beside_state = state_notice && !self.history.is_empty();
                let minimum = if silence_beside_sync_off || beside_state {
                    self.undo_action("Undo").natural_width(ui) + design::XS
                } else {
                    minimum
                };
                // A shortened Undo target comes before the sheet-change notice, which can go
                // below the roster.
                let sheet_reserve = if beside_state && !silence_beside_sync_off {
                    target_reserve
                } else {
                    minimum
                };
                let notice_width = design::number_width(ui, &notice, &font) + design::S;
                let sheet_room =
                    (ui.available_width() - sheet_reserve - notice_width - design::L).max(0.0);
                let sheet_width = |summary: &str| {
                    design::number_width(ui, &format!("{} {summary}", icons::WARN), &font)
                        + design::number_width(ui, "Log", &Type::Small.font(ui))
                        + Density::Read.control()
                        + design::L
                        + design::S * 3.0
                };
                let sheet = self
                    .notice_summary
                    .as_ref()
                    .filter(|_| self.notice.is_some() && self.recording_notice.is_none())
                    .and_then(|summary| {
                        if sheet_width(summary) <= sheet_room {
                            return Some(summary.clone());
                        }
                        let compact = summary
                            .split(", ")
                            .next()
                            .unwrap_or(summary)
                            .replace(&format!("{} ", super::SHEET_CHANGED), ": ");
                        (sheet_width(&compact) <= sheet_room).then_some(compact)
                    });
                let sheet_width = sheet.as_ref().map_or(0.0, |summary| sheet_width(summary));
                sheet_notice_in_header = sheet.is_some();
                let notice_width = (notice_width + if sheet.is_some() { sheet_width } else { 0.0 })
                    .min((ui.available_width() - minimum - design::L).max(0.0));
                let slot = ui.available_width() - notice_width - design::L;
                let mut keyless = false;
                let shown = if party_sync_off
                    && self.recording_notice.is_some()
                    && self.undo_action(&fitted).natural_width(ui) > slot
                {
                    "Undo".to_owned()
                } else {
                    // The key letter goes before the target does; the Keys list still has U.
                    let mut shown = "Undo".to_owned();
                    for drop_key in [false, true] {
                        let width_of = |words: &str| {
                            let action = self.undo_action(words);
                            let action = if drop_key { action.key("") } else { action };
                            action.natural_width(ui)
                        };
                        let words = undo_words(
                            ui,
                            &undo_label,
                            from_sheet,
                            &self.state.participants,
                            &Type::Strong.font(ui),
                            slot - width_of("") - design::XS,
                        );
                        if width_of(&words) <= slot && keeps_names(&words) {
                            shown = words;
                            keyless = drop_key;
                            break;
                        }
                    }
                    shown
                };
                ui.allocate_ui_with_layout(
                    vec2(slot, Density::Read.control()),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        undo = if self.history.is_empty() {
                            self.empty_undo(ui)
                        } else {
                            let action = self.undo_action(&shown);
                            let action = if keyless { action.key("") } else { action };
                            action
                                .name(&undo_label)
                                .hint(&undo_label)
                                .show(ui)
                                .clicked()
                        };
                    },
                );
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = design::XS;
                    let font = Type::Small.font(ui);
                    if let Some(notice) = self.recording_notice.filter(|_| strip.conflict.is_none())
                    {
                        design::status(
                            ui,
                            Status {
                                icon: icons::WARN,
                                word: "Recording silence",
                                color: c.result.warning,
                            },
                        )
                        .on_hover_text(notice);
                        if sync_alert.is_some() {
                            ui.label(Type::Small.rich(ui, "·").color(c.muted));
                        }
                    }
                    if nobody_present && self.recording_notice.is_none() && strip.conflict.is_none()
                    {
                        // A warning has its icon, as `2 tied` does.
                        ui.label(
                            Type::Small
                                .rich(ui, format!("{} No participants are present", icons::WARN))
                                .color(c.result.warning),
                        )
                        .on_hover_text(NOBODY_PRESENT);
                    } else if let Some(alert) = &sync_alert {
                        let link = if strip.conflict.is_some() && self.hide_panel {
                            "Review"
                        } else if needs_connection {
                            "Connect to sync"
                        } else {
                            ""
                        };
                        let full = &alert.text;
                        let link_width = design::number_width(ui, link, &font) + design::S;
                        let fits_link = !link.is_empty()
                            && design::number_width(ui, &alert.short, &font)
                                + link_width
                                + design::XS
                                <= ui.available_width();
                        let text_width = ui.available_width()
                            - if sheet.is_some() { sheet_width } else { 0.0 }
                            - if fits_link {
                                link_width + design::XS
                            } else {
                                0.0
                            };
                        let full_fits = design::number_width(ui, full, &font) <= text_width;
                        let words = if full_fits { full } else { &alert.short };
                        ui.allocate_ui_with_layout(
                            vec2(text_width, Density::Read.control()),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                if strip.conflict.is_some() {
                                    let word = words
                                        .split_once(' ')
                                        .map_or(words.as_str(), |(_, word)| word);
                                    if design::status_link(
                                        ui,
                                        Status {
                                            icon: icons::WARN,
                                            word,
                                            color: alert.color,
                                        },
                                    )
                                    .on_hover_text(full)
                                    .clicked()
                                    {
                                        review_conflict = true;
                                    }
                                } else {
                                    ui.add(
                                        egui::Label::new(
                                            Type::Small.rich(ui, words).color(alert.color),
                                        )
                                        .truncate(),
                                    )
                                    .on_hover_text(full);
                                }
                            },
                        );
                        if !link.is_empty() && fits_link && design::small_link(ui, link).clicked() {
                            if link == "Review" {
                                review_conflict = true;
                            } else {
                                self.wants_sign_in = true;
                            }
                        }
                    } else if self.recording_notice.is_none() {
                        let text = if nobody_present {
                            NOBODY_PRESENT.to_owned()
                        } else {
                            strip
                                .next
                                .as_ref()
                                .map_or(String::new(), |next| format!("Next: {next}"))
                        };
                        // Undo is shortened first. Then the next name goes whole, because
                        // the roster marks NEXT on its row.
                        let shown = if nobody_present {
                            design::words_that_fit(ui, &text, &font, ui.available_width())
                        } else if design::number_width(ui, &text, &font) <= ui.available_width() {
                            text
                        } else {
                            String::new()
                        };
                        ui.label(Type::Small.rich(ui, shown).color(if nobody_present {
                            c.result.warning
                        } else {
                            c.turn.next
                        }));
                    }
                    if let Some(summary) = &sheet {
                        ui.add_space(design::S);
                        // The compact message's own top alignment is useful in stacked
                        // notices. Here its text-height box is centred by the control row.
                        let width =
                            design::number_width(ui, &format!("{} {summary}", icons::WARN), &font)
                                + design::S
                                + design::HAIRLINE_WIDTH
                                + design::XS;
                        let response = ui
                            .allocate_ui_with_layout(
                                vec2(width, Type::Small.line()),
                                Layout::left_to_right(Align::Center),
                                |ui| {
                                    ui.spacing_mut().item_spacing.x = design::S;
                                    design::small_message(ui, Tone::Warning, summary)
                                },
                            )
                            .inner;
                        let detail = self.notice.as_deref().unwrap_or(summary);
                        design::named(ui, &response, detail);
                        response.on_hover_text(detail);
                        let log = log_link(ui);
                        design::named(ui, &log, "Log");
                        if log.clicked() {
                            self.panel = Panel::Log;
                            self.close_lookup = true;
                        }
                        if design::icon_button(ui, icons::CLOSE, "Dismiss this message").clicked() {
                            self.notice = None;
                            self.notice_summary = None;
                        }
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
        sheet_notice_in_header
    }

    /// The sync state a row shows, if this participant has one.
    pub(super) fn sync_word(&self, index: usize) -> Option<SyncState> {
        self.syncs
            .get(&index)
            .map(crate::combat_sync::CombatSync::state)
    }

    fn party_sync_off(&self) -> bool {
        let mut linked = self
            .state
            .participants
            .iter()
            .enumerate()
            .filter(|(_, p)| super::remote_identity(p).is_some())
            .peekable();
        linked.peek().is_some()
            && linked.all(|(index, participant)| {
                self.sync_word(index) == Some(SyncState::Off)
                    || (!self.syncs.contains_key(&index)
                        && !self.keep_party_synced
                        && !self.sync_to_restore.contains(&participant.key))
            })
    }

    fn row_sync_word(&self, index: usize) -> Option<SyncState> {
        // A lost connection stops sync, and it starts again by itself: not the DM's Sync off.
        self.sync_word(index).filter(|state| {
            *state != SyncState::Stopped && !(*state == SyncState::Off && self.party_sync_off())
        })
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
                    .with_badge(ui, self.row_badge(index).as_deref())
            })
            .collect()
    }

    /// The height of this list at its most compressed, in this room.
    fn tightest_roster(&self, ui: &egui::Ui, width: f32, room: f32) -> f32 {
        let scale = RunScale::fit(room, width, self.state.order.len());
        self.roster_layouts(ui, width, scale, RowFit::Summary)
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
            layouts = self.roster_layouts(ui, width, scale, RowFit::Natural);
        } else {
            layouts = self.roster_layouts(ui, width, scale, RowFit::Compressed);
            // What is left goes back to the rows, shortest first, so the pitch evens out.
            let heights: Vec<f32> = layouts.iter().map(|l| l.height(scale)).collect();
            let cap = heights.iter().copied().fold(scale.row, f32::max);
            let even = even_height(&heights, room - 2.0, cap);
            layouts.iter_mut().for_each(|l| l.even = even);
        }
        if total(&layouts, scale) > room {
            layouts = self.roster_layouts(ui, width, scale, RowFit::Summary);
            let heights: Vec<_> = layouts.iter().map(|layout| layout.height(scale)).collect();
            let cap = heights.iter().copied().fold(scale.row, f32::max);
            let even = even_height(&heights, room - design::HAIRLINE_WIDTH, cap);
            layouts.iter_mut().for_each(|layout| layout.even = even);
        }
        // A list longer than its room keeps a line under it, to say what is out of view.
        let overflow = total(&layouts, scale) > room;
        if overflow {
            // Too tall even at its tightest. It scrolls, at that tight pitch.
            layouts = self.roster_layouts(ui, width - SCROLL_BAR, scale, RowFit::Summary);
        }
        #[cfg(test)]
        for (&index, layout) in self.state.order.iter().zip(&layouts) {
            let first = &layouts[0];
            assert_eq!(layout.initiative_right, first.initiative_right);
            assert_eq!(layout.name_left, first.name_left);
            assert_eq!(layout.picture, first.picture);
            assert!(layout.fit != RowFit::Natural || layout.height(scale) >= scale.row);
            assert!(
                layout
                    .second_line
                    .as_ref()
                    .is_none_or(|line| line.rows.len() <= 1),
                "live row detail wraps at {width} for {}: {:?}",
                self.state.participants[index].name,
                layout.second_line.as_ref().map(|line| (
                    line.text(),
                    line.job.wrap.max_width,
                    line.rows.iter().map(|row| row.size).collect::<Vec<_>>()
                ))
            );
        }
        let list_height = if overflow { room - OVERFLOW_LINE } else { room };
        let mut clicked = None;
        let mut row_choice = None;
        let mut rule = None;
        let mut toggle_half = None;
        // A changed window size must not leave the selected row out of view. The bar of
        // actions that Lookup shows above the dock is not a new size.
        let window_room = room + self.strip_height();
        let resized = (window_room - self.roster_height).abs() > 1.0;
        self.roster_height = window_room;
        let scroll = std::mem::take(&mut self.scroll_to_selected) || resized;
        let mut spans = Vec::with_capacity(count);
        let mut edit_initiative = None;
        let mut set_initiative = None;
        let mut close_initiative = false;
        let order = self.state.order.clone();
        let output = egui::ScrollArea::vertical()
            .id_salt("run_roster")
            .auto_shrink([false, false])
            .max_height(list_height)
            .show(ui, |ui| {
                let clip = ui.clip_rect().intersect(Rect::from_min_size(
                    ui.cursor().min,
                    vec2(width, list_height),
                ));
                ui.set_clip_rect(clip);
                ui.spacing_mut().item_spacing.y = 0.0;
                let origin = ui.cursor().min.y;
                let mut selected_rect = None;
                let mut next_rect = None;
                for (&index, layout) in order.iter().zip(&layouts) {
                    let original_clip = ui.clip_rect();
                    let row_top = ui.cursor().top();
                    let whole_row = row_top >= original_clip.top() - design::HAIRLINE_WIDTH
                        && row_top + layout.height(scale)
                            <= original_clip.bottom() + design::HAIRLINE_WIDTH;
                    if overflow && !whole_row {
                        ui.set_clip_rect(Rect::NOTHING);
                    }
                    let selected = self.selected == index || self.also.contains(&index);
                    let p = &self.state.participants[index];
                    let sync = self.row_sync_word(index);
                    let save = self.save_on_row(index);
                    let badge = self.row_badge(index);
                    let turn = TurnMark::of(index, current, next_up);
                    let mut name = row_name(p, turn, sync);
                    if let Some((ability, total)) = &save {
                        name.push_str(&format!(", {ability} save {total}"));
                    }
                    if let Some(badge) = &badge {
                        name.push_str(&format!(", {badge}"));
                    }
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
                            if selected {
                                ui.painter().rect_filled(
                                    rect,
                                    design::RADIUS_CONTROL,
                                    colors().selected,
                                );
                            } else if ui.rect_contains_pointer(rect) {
                                ui.painter().rect_filled(
                                    rect,
                                    design::RADIUS_CONTROL,
                                    colors().hover,
                                );
                            }
                            let response = ui
                                .scope_builder(egui::UiBuilder::new().max_rect(target), |ui| {
                                    design::row(ui, rect.height(), selected, &name, |ui, _| {
                                        paint_row(
                                            ui,
                                            rect,
                                            p,
                                            turn,
                                            sync,
                                            scale,
                                            layout,
                                            save.as_ref(),
                                            badge.as_deref(),
                                        );
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
                                    paint_row(
                                        ui,
                                        rect,
                                        p,
                                        turn,
                                        sync,
                                        scale,
                                        layout,
                                        save.as_ref(),
                                        badge.as_deref(),
                                    );
                                },
                            );
                            let rect = response.rect;
                            (response, rect)
                        }
                    });
                    let (response, rect) = response.inner;
                    // A name cut to `…` is whole in hover, as it is in the row's name.
                    let response = if layout.name_cut {
                        response.on_hover_text(format!("{}\n{}", p.name, p.side))
                    } else {
                        response.on_hover_text(&p.side)
                    };
                    ui.set_clip_rect(original_clip);
                    spans.push((rect.top() - origin, rect.bottom() - origin, index));
                    if response.clicked() {
                        clicked = Some((index, ui.input(|i| i.modifiers.command)));
                    }
                    // Right-click: how to roll against this row, or a question it waits on.
                    if let Some(choice) = self.row_menu(&response, index) {
                        row_choice = Some((index, choice));
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
                            pos2(
                                word.center().x,
                                word.center().y.clamp(
                                    rect.top() + Density::Read.control() / 2.0,
                                    rect.bottom() - Density::Read.control() / 2.0,
                                ),
                            ),
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
                    // In combat the number itself opens a field to change it.
                    // With several targets, this place holds the half-damage mark.
                    if self.state.phase == Phase::Running
                        && self.live_initiative != Some(index)
                        && (!selected || self.also.is_empty())
                    {
                        // Inside the row, as its second target, like the missing-total link.
                        // A short summary row keeps it to the row, with no clamp to panic.
                        let middle = (rect.top() + layout.line_y(scale, rect.height()))
                            .min(rect.bottom() - Density::Read.control() / 2.0)
                            .max(rect.top() + Density::Read.control() / 2.0);
                        let target = Rect::from_min_max(
                            pos2(
                                rect.left() + design::S,
                                middle - Density::Read.control() / 2.0,
                            ),
                            pos2(
                                rect.left() + layout.initiative_right + design::XS,
                                middle + Density::Read.control() / 2.0,
                            ),
                        )
                        .intersect(rect);
                        let number = ui
                            .interact(
                                target,
                                ui.id().with(("change_initiative", &p.key)),
                                egui::Sense::click(),
                            )
                            .on_hover_text("Change initiative")
                            .on_hover_cursor(egui::CursorIcon::PointingHand);
                        number.widget_info(|| {
                            egui::WidgetInfo::labeled(
                                egui::WidgetType::Link,
                                true,
                                format!("Change initiative for {}", p.name),
                            )
                        });
                        if number.clicked() {
                            edit_initiative = Some(index);
                            clicked = None;
                        }
                        if number.has_focus() {
                            design::focus_ring(ui, target);
                        }
                    }
                    if self.live_initiative == Some(index) {
                        let field = Rect::from_min_size(
                            pos2(
                                rect.left() + design::S,
                                rect.top() + layout.line_y(scale, rect.height())
                                    - Density::Read.control() / 2.0,
                            ),
                            vec2(layout.initiative_right - design::S, Density::Read.control()),
                        );
                        ui.scope_builder(egui::UiBuilder::new().max_rect(field), |ui| {
                            let name = format!("Initiative for {}", p.name);
                            let text = self.setup_inputs.entry(index).or_default();
                            let response = design::number_field(
                                ui,
                                &name,
                                text,
                                "",
                                field.width(),
                                Density::Read.control(),
                            );
                            if self.focus_field == Some(index) {
                                // Typing replaces the total that is there.
                                design::focus_and_select(ui, response.id, text);
                                self.focus_field = None;
                            }
                            let enter = ui.input(|i| i.key_pressed(Key::Enter));
                            if enter
                                && (response.has_focus() || response.lost_focus())
                                && let Ok(total) = text.parse()
                            {
                                set_initiative = Some((index, total));
                            } else if response.lost_focus() {
                                // Escape, a click away, or Enter with no number leaves the
                                // total as it was.
                                close_initiative = true;
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
        if close_initiative {
            self.live_initiative = None;
        }
        if let Some(index) = edit_initiative {
            self.live_initiative = Some(index);
            self.focus_field = Some(index);
            let total = self.state.participants[index].initiative;
            self.setup_inputs
                .insert(index, total.map_or_else(String::new, |n| n.to_string()));
        }
        if let Some((index, total)) = set_initiative {
            // The same total again changes nothing, and is no step to undo.
            if self.state.participants[index].initiative != Some(total) {
                self.run_command(Command::Initiative(index, Some(total)));
            }
            self.live_initiative = None;
        }
        if let Some(index) = toggle_half {
            if !self.halved.remove(&index) {
                self.halved.insert(index);
            }
            self.focus_amount = true;
        }
        // While an action is armed, a row is its target: Shift+click rolls with advantage,
        // Ctrl+click with disadvantage. Otherwise a click selects, and Ctrl+click adds.
        if let Some((index, ctrl)) = clicked {
            let shift = ui.input(|i| i.modifiers.shift);
            let mode = if ctrl {
                Some(crate::rolls::Mode::Disadvantage)
            } else if shift {
                Some(crate::rolls::Mode::Advantage)
            } else {
                None
            };
            if !self.target(index, mode) {
                if ctrl {
                    self.toggle_target(index);
                } else {
                    self.select(index);
                }
            }
        }
        if let Some((index, choice)) = row_choice {
            self.row_choice(index, choice);
        }
        if rule == Some(usize::MAX) || rule == Some(usize::MAX - 2) {
            self.panel = Panel::Conditions;
        } else if rule == Some(usize::MAX - 1) {
            self.panel = Panel::Reference;
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
            if self.state.phase == Phase::Setup {
                text.push_str(&self.setup_hidden_attention(rows));
            }
            Some(text)
        };
        let text = [side("above", &above), side("below", &below)]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join(" · ");
        let (rect, _) = ui.allocate_exact_size(
            vec2(ui.available_width(), OVERFLOW_LINE),
            egui::Sense::hover(),
        );
        ui.painter().rect_filled(rect, 0.0, colors().page);
        ui.painter().hline(
            rect.x_range(),
            rect.top(),
            Stroke::new(design::HAIRLINE_WIDTH, colors().hairline),
        );
        ui.painter().text(
            rect.left_center(),
            Align2::LEFT_CENTER,
            text,
            Type::Small.font(ui),
            colors().muted,
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
                    ui.add_space(design::S);
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
                        if narrow && let Some((total, line)) = self.dock_roll() {
                            ui.label(Type::Strong.rich(ui, format!("= {total}")))
                                .on_hover_text(line);
                        }
                        // Beside the actions at the minimum window it gives up size, not letters.
                        let role = if design::number_width(ui, &p.name, &Type::Heading.font(ui))
                            < ui.available_width()
                        {
                            Type::Heading
                        } else {
                            Type::Small
                        };
                        let shown = dock_name(
                            ui,
                            &p.name,
                            &role.font(ui),
                            ui.available_width() - design::S,
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
        // Typed, cleared or typed again: the amount is the DM's until a new roll.
        if response.changed() {
            self.amount_typed = true;
        }
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
                let columns = if ui.available_width() >= design::CONDITION_THREE_COLUMNS_FROM { 3 } else { 2 };
                egui::Grid::new("official_conditions")
                    .num_columns(columns)
                    .min_col_width((ui.available_width() / columns as f32 - design::XS).max(design::CONDITION_COLUMN_MIN))
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
                    let start = exhaustion_level(&p.conditions[at]);
                    // A drag shows its level here and commits one step when it ends, from the
                    // level it started at. A drag that the run no longer matches is dropped.
                    let frame = ui.ctx().cumulative_frame_nr();
                    let drag = self.exhaustion_drag.take().filter(|drag| {
                        (drag.index, drag.at, drag.start) == (index, at, start)
                            && drag.frame + 1 >= frame
                    });
                    let mut level = drag.map_or(i64::from(start), |drag| drag.level);
                    let stepper = ui.horizontal(|ui| {
                        ui.label(Type::Body.rich(ui, "Exhaustion level").color(colors().text));
                        design::stepper_with_density(
                            ui,
                            "Exhaustion level",
                            &mut level,
                            0..=EXHAUSTION_MAX,
                            Density::Scan,
                            Some(EXHAUSTION_AT_LOW),
                            Some(EXHAUSTION_AT_HIGH),
                        )
                    });
                    let dragging = ui.ctx().dragged_id().is_some()
                        && ui
                            .input(|i| i.pointer.press_origin())
                            .is_some_and(|origin| stepper.response.rect.contains(origin));
                    // A level above the usual top (from a file or a sync) stays until the DM
                    // changes it.
                    let level = level.clamp(0, EXHAUSTION_MAX.max(i64::from(start)));
                    if dragging {
                        self.exhaustion_drag = Some(ExhaustionDrag {
                            index,
                            at,
                            start,
                            level,
                            frame,
                        });
                    } else if (stepper.inner || drag.is_some()) && level != i64::from(start) {
                        command = Some(if level == 0 {
                            Command::RemoveCondition(index, at)
                        } else {
                            set_exhaustion(index, at, level as u8)
                        });
                    }
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
                                design::row_menu(ui, condition, |ui| {
                                    if ui.button("Remove").clicked() {
                                        command = Some(Command::RemoveCondition(index, i));
                                        ui.close();
                                    }
                                });
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
                let note = close_saved_words(
                    !self.history.is_empty(),
                    self.syncs.values().any(|sync| sync.enabled),
                );
                closed = self.close_confirmation(ui, &note);
                return;
            }
            // Five actions share the narrowest row. None of them may lose a letter.
            let compact = ui.available_width() < DOCK_ALL_WORDS_FROM;
            ui.spacing_mut().item_spacing.x = 0.0;
            let mut closing = false;
            let mut ending = false;
            let blocked = self.close_block();
            let close_word = if blocked.is_some() {
                "Close after sync"
            } else {
                "Close run"
            };
            let status = self.save_status_value();
            let saved_word = self.copy_label.as_ref().map_or_else(
                || status.word.to_owned(),
                |copy| format!("{} · {copy}", status.word),
            );
            let status_width = design::number_width(
                ui,
                &format!("{} {saved_word}", status.icon),
                &Type::Small.font(ui),
            );
            let keys = if compact {
                Action::quiet("Keys")
            } else {
                Action::quiet("Keys").icon(icons::KEYS)
            }
            .density(Density::Read);
            let right_width = status_width
                + keys.natural_width(ui)
                + self
                    .party_sync_action()
                    .map_or(0.0, |action| action.natural_width(ui));
            let close_width = Action::quiet(close_word)
                .density(Density::Read)
                .natural_width(ui)
                + Action::quiet("End encounter…")
                    .density(Density::Read)
                    .natural_width(ui)
                + right_width
                + design::L;
            let end_word = if ui.available_width() < close_width {
                "End…"
            } else {
                "End encounter…"
            };
            let short_width = close_width
                - design::number_width(ui, "End encounter…", &Type::Strong.font(ui))
                + design::number_width(ui, "End…", &Type::Strong.font(ui));
            let menu_needed = ui.available_width() < short_width;
            let end_menu = menu_needed && self.copy_label.is_some();
            let mut controls = |ui: &mut egui::Ui| {
                let blocked = self.close_block();
                closing = Action::quiet(if blocked.is_some() {
                    "Close after sync"
                } else {
                    "Close run"
                })
                .name("Close run")
                .density(Density::Read)
                .blocked(blocked)
                .show(ui)
                .clicked();
                if !end_menu {
                    ending = Action::quiet(end_word)
                        .name("End encounter…")
                        .density(Density::Read)
                        .hint("End encounter… Set an outcome. You can undo this.")
                        .show(ui)
                        .clicked();
                }
            };
            if menu_needed && !end_menu {
                let menu = ui.menu_button(format!("Close {}", icons::DOWN), |ui| controls(ui));
                design::named(ui, &menu.response, "Close menu");
            } else {
                controls(ui);
            }
            if end_menu {
                design::row_menu(ui, &self.name, |ui| {
                    if Action::quiet("End encounter…").show(ui).clicked() {
                        ending = true;
                        ui.close();
                    }
                });
            }
            if closing {
                closed = self.request_close();
            }
            self.ending |= ending;
            self.focus_outcome |= ending;
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                self.save_status(ui);
                let keys = if compact {
                    Action::quiet("Keys")
                } else {
                    Action::quiet("Keys").icon(icons::KEYS)
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
        closed
    }

    /// `Saved` or `Not saved`, in every phase.
    fn save_status(&self, ui: &mut egui::Ui) {
        let status = self.save_status_value();
        let base_word = status.word;
        let path = self.path().map(|path| path.display().to_string());
        let word = self
            .copy_label
            .as_ref()
            .map(|copy| format!("{} · {copy}", status.word));
        let status = Status {
            word: word.as_deref().unwrap_or(status.word),
            ..status
        };
        let response = design::status(ui, status);
        let full = self
            .copy_label
            .as_ref()
            .map(|copy| format!("{base_word} · {copy}"));
        if let Some(full) = &full {
            design::named(ui, &response, full);
        }
        if let Some(path) = path {
            response.on_hover_text(full.map_or(path.clone(), |full| format!("{full}\n{path}")));
        }
    }

    fn save_status_value(&self) -> Status<'static> {
        if self.strip().not_saved {
            Status {
                icon: icons::ERROR,
                word: "Not saved",
                color: colors().document.invalid,
            }
        } else if !self.dirty {
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
        let conflict = self.first_sync_in(SyncState::Conflict);
        let mut review = false;
        let title = if cannot_save {
            "This run is not saved"
        } else if conflict.is_some() {
            "Sync conflict"
        } else if self.sync_needs_sign_in() && !stranded.is_empty() {
            "Changes not sent to D&D Beyond"
        } else if !stranded.is_empty() || block.is_some() {
            "Sync has not finished"
        } else {
            "Close this run?"
        };
        let title = conflict.filter(|_| !cannot_save).map_or_else(
            || title.to_owned(),
            |i| format!("{} has a sync conflict", self.state.participants[i].name),
        );
        let changes = self.unsaved_summary();
        design::dialog(ui.ctx(), "close_run", &title, |ui| {
            if cannot_save {
                ui.add(
                    egui::Label::new(format!(
                        "{}Closing loses {}:",
                        if self.has_disk_conflict() {
                            "The file changed on disk, so saving stopped. "
                        } else {
                            ""
                        },
                        if changes.len() == 1 {
                            "this change"
                        } else {
                            "these changes"
                        }
                    ))
                    .wrap(),
                );
                for change in changes.iter().take(3) {
                    ui.add(egui::Label::new(change).wrap());
                }
                if changes.len() > 3 {
                    ui.label(format!("and {} more", changes.len() - 3));
                }
            } else if conflict.is_some() {
                ui.label("Choose which HP to keep, or close and choose next time.");
            } else if stranded.is_empty() {
                ui.add(egui::Label::new(note).wrap());
            } else {
                ui.add(egui::Label::new(format!("Changes for {} were not sent to D&D Beyond. The saved run keeps HP, death saves and conditions for the next sync. Closing ends Undo for this run.", stranded.join(", "))).wrap());
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
            let chosen = close_recovery_actions(
                ui,
                conflict.is_some(),
                self.sync_needs_sign_in() && !stranded.is_empty(),
            );
            review = chosen.review;
            if chosen.connect {
                self.wants_sign_in = true;
                keep = true;
            }
            let (lose, ()) = design::dialog_actions(
                ui,
                |ui| {
                    (cannot_save || !lost.is_empty())
                        && Action::destructive("Close and lose changes")
                            .blocked(block)
                            .show(ui)
                            .clicked()
                },
                |ui| {
                    if cannot_save {
                        save_new = Action::primary("Save as a new run and close")
                            .blocked(block)
                            .show(ui)
                            .clicked();
                    } else if lost.is_empty() {
                        close |= Action::secondary("Close now")
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
                },
            );
            close |= lose;
        });
        if review {
            self.review_conflict();
            self.panel = Panel::Reference;
            keep = true;
        }
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

    fn unsaved_summary(&self) -> Vec<String> {
        let saved_lines = self
            .storage
            .as_ref()
            .and_then(|s| s.baseline.as_ref())
            .and_then(|bytes| serde_json::from_slice::<super::SavedRun>(bytes).ok())
            .map_or(0, |saved| saved.state.log.len());
        let changes: Vec<_> = self
            .history
            .iter()
            .zip(&self.history_labels)
            .filter(|(state, _)| state.log.len() >= saved_lines)
            .map(|(_, label)| Self::change_with_result(&self.state, label))
            .collect();
        if changes.is_empty() {
            vec!["Everything since the last save.".to_owned()]
        } else {
            changes
        }
    }

    fn request_close(&mut self) -> bool {
        if self.strip().not_saved
            || !self.stranded_names().is_empty()
            || !self.unsent_only_here_names().is_empty()
        {
            self.closing = true;
            false
        } else {
            self.close_run()
        }
    }

    pub(super) fn close_run(&mut self) -> bool {
        if let Some(reason) = self.close_block() {
            self.error = Some(reason.to_owned());
            return false;
        }
        self.mark_closed();
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
    fn party_sync_action(&self) -> Option<Action<'static>> {
        if self.linked_participants().is_empty() {
            return None;
        }
        let stopping = matches!(self.party_sync_plan(), PartySync::Stop(_));
        Some(Action::quiet(if stopping { "Stop sync" } else { "Sync party" })
            .icon(icons::SYNC_ON)
            .density(Density::Read)
            .name(if stopping {
                "Stop party sync"
            } else {
                "Sync party"
            })
            .hint("Read every linked character’s sheet. A value you changed stays, and you choose.")
            .blocked_if(
                !self.connected && !self.simulate_connection,
                "Connect to D&D Beyond in Settings to sync.",
            ))
    }

    fn sync_party_button(&mut self, ui: &mut egui::Ui) {
        if self
            .party_sync_action()
            .is_some_and(|action| action.show(ui).clicked())
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
            ui.add_space(design::L);
            let Some(sync) = self.syncs.get_mut(&index) else {
                continue;
            };
            let conflict = sync.state() == SyncState::Conflict;
            ui.push_id(index, |ui| {
                if conflict {
                    sync.conflicts_for(ui, &name);
                } else {
                    ui.horizontal(|ui| {
                        ui.label(Type::Strong.rich(ui, &name).color(colors().text));
                        design::status(ui, sync_status(sync.state()));
                    });
                }
                if Action::quiet(&format!("Stop sync for {name}"))
                    .density(Density::Read)
                    .line_start()
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
        // While an action is armed, its actor's stat block stays; the other tabs follow the row.
        let index = if self.panel == Panel::Reference {
            self.panel_row()
        } else {
            self.selected
        };
        let p = &self.state.participants[index];
        let (successes, failures) = (p.successes, p.failures);
        let death_saves = p.shows_death_saves();
        let is_sheet = p.definition.fields.contains_key("splat_external");
        let title = if matches!(self.panel, Panel::Log | Panel::Encounter | Panel::Groups) {
            self.fight_panel_title()
        } else {
            &p.name
        };
        let shown = dock_name(ui, title, &Type::Title.font(ui), ui.available_width());
        let response = ui.add(egui::Label::new(
            Type::Title.rich(ui, shown).color(colors().text),
        ));
        design::named(ui, &response, title);
        response.on_hover_text(title);

        let participant_tab = !matches!(self.panel, Panel::Log | Panel::Encounter | Panel::Groups);
        let reference = if is_sheet { "Sheet" } else { "Stat block" };
        self.panel_tabs(ui, Some(reference), true);
        design::rule(ui);
        if death_saves
            && !(self.panel == Panel::Reference
                && self.sheet_for == Some(index)
                && self.sheet.is_some())
        {
            self.death_saves(ui, index, successes, failures);
        }
        if let Some(sync) = self
            .syncs
            .get_mut(&index)
            .filter(|s| participant_tab && s.state() == SyncState::Conflict)
        {
            sync.conflicts(ui);
        }
        self.panel_body(ui, index, self.panel);
    }

    /// Add to fight opens Arrivals. It is beside the fight's own controls, not among the
    /// panel's tabs, so that the DM finds it when something arrives. It takes the height of
    /// the controls in its row. Narrow layouts keep the icon and the word Add.
    fn add_to_fight(&self, words: bool, density: Density) -> Action<'static> {
        let action = if words {
            Action::secondary("Add to fight").icon(icons::ADD)
        } else {
            Action::secondary("Add").icon(icons::ADD)
        };
        action
            .name("Add to fight")
            .hint("Add an encounter group or a creature (Ctrl+G)")
            .density(density)
            .selected(self.panel == Panel::Groups && !self.hide_panel)
    }

    /// One row, with participant controls on the left and fight controls on the right.
    fn fight_panel_title(&self) -> &'static str {
        match self.panel {
            Panel::Log => "This fight · Log",
            Panel::Encounter => "This fight · Prep",
            Panel::Groups => "This fight · Arrivals",
            _ => "This fight",
        }
    }

    fn panel_tabs(&mut self, ui: &mut egui::Ui, reference: Option<&str>, conditions: bool) {
        let mut participant = Vec::new();
        if let Some(reference) = reference {
            participant.push((Panel::Reference, reference));
        }
        if conditions {
            participant.push((Panel::Conditions, "Conditions"));
        }
        if reference.is_some() {
            participant.push((Panel::Record, "Correct"));
        }
        // Arrivals has no tab. Add to fight, beside the fight's controls, opens it.
        let fight = [(Panel::Log, "Log"), (Panel::Encounter, "Prep")];
        let width = |items: &[(Panel, &str)]| {
            items
                .iter()
                .map(|(_, label)| {
                    design::number_width(ui, label, &Type::Strong.font(ui)) + design::L
                })
                .sum::<f32>()
        };
        let participant_width = width(&participant);
        let fight_width = width(&fight);
        let fits = participant_width + fight_width + design::L <= ui.available_width();
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
                    let current = fight
                        .iter()
                        .find(|(panel, _)| *panel == self.panel)
                        .map(|(_, label)| *label);
                    let button = design::menu_button(ui, "Fight", false);
                    egui::Popup::menu(&button).show(|ui| {
                        for (panel, label) in fight {
                            let action = egui::Button::new(label).selected(self.panel == panel);
                            if ui.add(action).clicked() {
                                self.panel = panel;
                                ui.close();
                            }
                        }
                    });
                    design::named(ui, &button, "Fight menu");
                    if current.is_some() {
                        ui.painter().hline(
                            button.rect.x_range(),
                            button.rect.bottom(),
                            egui::Stroke::new(design::HAIRLINE_WIDTH, colors().accent),
                        );
                    }
                }
            });
        });
    }

    fn panel_body(&mut self, ui: &mut egui::Ui, index: usize, panel: Panel) {
        match panel {
            Panel::Reference => {
                self.reference_tab(ui, index);
                self.armed_tiles_end(ui.ctx());
            }
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
                        |ui| {
                            if let Some(item) = self.content.as_ref().and_then(|index| {
                                index.items.iter().find(|item| {
                                    Some(item.entry.id) == self.source
                                        && item.entry.kind == "encounter"
                                })
                            }) && Action::quiet("Edit encounter")
                                .line_start()
                                .show(ui)
                                .clicked()
                            {
                                ui.ctx().data_mut(|data| {
                                    data.insert_temp(
                                        egui::Id::new("edit_run_encounter"),
                                        item.entry.path.clone(),
                                    )
                                });
                            }
                        },
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
                if self
                    .groups
                    .as_ref()
                    .is_some_and(|groups| !groups.is_empty())
                {
                    ui.add(
                        egui::Label::new(
                            Type::Small
                                .rich(ui, "Drag a group name to the list, or use Add.")
                                .color(colors().muted),
                        )
                        .wrap(),
                    );
                }
                self.add_creature_form(ui);
                if let Some(previous) = self.history.last().filter(|_| {
                    self.ad_hoc.query.trim().is_empty()
                        && self.state.log.last().is_some_and(|line| {
                            line.starts_with(super::GROUP_ADDED)
                                || line.starts_with(super::ONE_ADDED)
                        })
                }) {
                    let start = previous.participants.len();
                    let tied = self
                        .state
                        .participants
                        .iter()
                        .enumerate()
                        .skip(start)
                        .find_map(|(first, arrival)| {
                            let total = arrival.initiative?;
                            self.state
                                .order
                                .iter()
                                .copied()
                                .rev()
                                .find(|i| {
                                    *i < start
                                        && self.state.participants[*i].present
                                        && self.state.participants[*i].initiative == Some(total)
                                })
                                .map(|before| (first, Self::roll_block_first(&self.state, before)))
                        });
                    let group_line = self.group_arrival_notice(start);
                    let line = group_line
                        .as_deref()
                        .unwrap_or_else(|| self.state.log.last().map_or("", String::as_str));
                    if let Some((first, before)) = tied {
                        let name =
                            Self::roll_block_name(&self.state, &self.state.participants[before]);
                        let waits = self.state.phase == Phase::Running
                            && self
                                .state
                                .order
                                .iter()
                                .position(|i| *i == before)
                                .is_some_and(|at| at <= self.state.cursor);
                        let total = self.state.participants[first].initiative.unwrap();
                        let tie = if group_line.is_some() {
                            format!(" Tied with {name} at\u{a0}{total}, placed after.")
                        } else {
                            String::new()
                        };
                        let line = format!(
                            "{line}{tie}{}",
                            if waits {
                                " ·\u{a0}acts next round if moved before"
                            } else {
                                ""
                            }
                        );
                        let arrival =
                            Self::roll_block_name(&self.state, &self.state.participants[first]);
                        let full = format!("Move {arrival} before {name}");
                        let font = Type::Strong.font(ui);
                        design::message(ui, Tone::Info, &line);
                        ui.horizontal(|ui| {
                            let icon = design::number_width(ui, icons::INFO, &Type::Body.font(ui));
                            let gap = ui.spacing().item_spacing.x;
                            let inset =
                                design::S + design::HAIRLINE_WIDTH + icon + gap * 2.0 - design::XS;
                            ui.add_space((inset - gap).max(0.0));
                            let shown = undo_words(
                                ui,
                                &full,
                                false,
                                &self.state.participants,
                                &font,
                                ui.available_width() - design::M * 2.0,
                            );
                            if Action::secondary(&shown)
                                .name(&full)
                                .hint(&full)
                                .density(Density::Read)
                                .show(ui)
                                .clicked()
                            {
                                self.run_command(Command::MoveArrivalBefore(first, before));
                            }
                        });
                    } else {
                        design::message(ui, Tone::Success, line);
                    }
                } else if let Some(line) = self
                    .state
                    .log
                    .last()
                    .filter(|line| line.contains(" moved before "))
                {
                    design::message(ui, Tone::Success, line);
                }
                design::rule(ui);
                add = self.group_list(ui);
            });
        if let Some(id) = add {
            self.add_group_from_panel(id);
        }
    }

    /// The roster shows members; this notice keeps just the group and its totals.
    /// Roll details and member names remain in the unmodified Log.
    fn group_arrival_notice(&self, start: usize) -> Option<String> {
        let added = self.state.participants.get(start..)?;
        let origin = added.first()?.added_from.as_ref()?;
        if origin.group.is_empty() {
            return None;
        }
        let totals: std::collections::BTreeSet<_> =
            added.iter().filter_map(|p| p.initiative).collect();
        let totals = totals
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        Some(format!("{} joined at\u{a0}{totals}.", origin.group))
    }

    /// Returns the group the DM chooses to add. Those in tonight's plan come first.
    fn group_list(&mut self, ui: &mut egui::Ui) -> Option<Uuid> {
        let c = colors();
        let Some(groups) = self.groups.clone() else {
            design::group_heading(ui, "Encounter groups", 0);
            ui.add(
                egui::Label::new(
                    Type::Body
                        .rich(
                            ui,
                            "This campaign’s groups have not loaded yet. If this stays, the Encounters page says why.",
                        )
                        .color(c.muted),
                )
                .wrap(),
            );
            return None;
        };
        if groups.is_empty() {
            design::group_heading(ui, "Encounter groups", 0);
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    Type::Small
                        .rich(ui, "No groups yet. Make one on Encounters with")
                        .color(c.muted),
                );
                if design::link(ui, "New group").clicked() {
                    self.wants_new_group = true;
                }
            });
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
        if !labelled {
            design::group_heading(ui, "Encounter groups", offered.any.len());
        }
        let add_width = offered
            .own
            .iter()
            .chain(&planned)
            .chain(&any)
            .chain(&offered.other)
            .map(|group| {
                Action::secondary(&arrival_add_word(group))
                    .density(Density::Read)
                    .natural_width(ui)
            })
            .fold(0.0, f32::max);
        let mut add = None;
        for (heading, list) in [
            ("For this encounter", offered.own.as_slice()),
            ("In tonight’s plan", planned.as_slice()),
            ("For any encounter", any.as_slice()),
            ("For one encounter", offered.other.as_slice()),
        ] {
            if labelled && !list.is_empty() {
                design::group_heading(ui, heading, list.len());
            }
            if heading == "For one encounter" && !list.is_empty() {
                ui.add(egui::Label::new(Type::Small.rich(ui,
                    "This run was saved before groups had an encounter, so all groups show.").color(c.muted)).wrap());
            }
            for group in list {
                ui.push_id(group.id, |ui| {
                    if self.group_row(ui, group, add_width) {
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
    fn group_row(&mut self, ui: &mut egui::Ui, group: &super::RunGroup, add_width: f32) -> bool {
        let c = colors();
        let name = &group.name;
        let changed = group
            .members
            .iter()
            .any(|member| member.overrides.changes_creature());
        let change_hint = if changed { " Changed." } else { "" };
        let added = self
            .state
            .participants
            .iter()
            .filter_map(|p| p.added_from.as_ref())
            .filter(|from| from.group_id == Some(group.id))
            .map(|from| from.round)
            .max();
        let mut status = if let Some(round) = added {
            if round == 0 {
                "Added before combat".to_owned()
            } else {
                format!("Added in round {round}")
            }
        } else if let Some(name) = group
            .encounter_name
            .as_ref()
            .filter(|_| group.encounter != self.source)
        {
            format!("For {name}")
        } else if group.encounter.is_some() && group.encounter != self.source {
            "For a deleted encounter".to_owned()
        } else {
            String::new()
        };
        if changed {
            if !status.is_empty() {
                status.push_str(" · ");
            }
            status.push_str(&format!("{} Changed", icons::CHANGED));
        }
        let grip = ui.allocate_response(
            vec2(ui.available_width(), Density::Read.control()),
            egui::Sense::DRAG,
        );
        // A pointer's way to do what the Add button does, so the keyboard does not stop here.
        grip.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Other,
                true,
                format!(
                    "Drag {name} to the initiative list{}",
                    if changed { ". Changed." } else { "" }
                ),
            )
        });
        let totals_text = format!(
            "{} · {} · {}",
            style::human_label(&group.side),
            group.count_label(),
            group.xp_label()
        );
        let heading_width = design::number_width(
            ui,
            &format!("{} {name}", icons::DRAG),
            &Type::Strong.font(ui),
        );
        let totals_in_heading = heading_width
            + design::L
            + design::number_width(ui, &totals_text, &Type::Small.font(ui))
            <= grip.rect.width();
        let status_in_heading = totals_in_heading
            && !status.is_empty()
            && heading_width
                + design::L
                + design::number_width(
                    ui,
                    &format!("{status} · {totals_text}"),
                    &Type::Small.font(ui),
                )
                <= grip.rect.width();
        let totals = ui.painter().text(
            grip.rect.right_center(),
            Align2::RIGHT_CENTER,
            if totals_in_heading {
                totals_text.clone()
            } else {
                String::new()
            },
            Type::Small.font(ui),
            c.muted,
        );
        if status_in_heading {
            ui.painter().text(
                pos2(
                    grip.rect.left() + heading_width + design::L,
                    grip.rect.center().y,
                ),
                Align2::LEFT_CENTER,
                &status,
                Type::Small.font(ui),
                c.muted,
            );
        }
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
            "{name}.{change_hint} {context}{} {}",
            group.notes.trim(),
            self.initiative.grouping.arrival_rule()
        ));
        if group.problem.is_none()
            && arrival_total(self.group_totals.get(&group.id).map_or("", String::as_str)).is_ok()
        {
            grip.dnd_set_drag_payload(GroupDrag(group.id));
        }
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
        if !totals_in_heading {
            ui.add(egui::Label::new(Type::Small.rich(ui, totals_text).color(c.muted)).wrap());
        }
        let members = group.member_summary_wrapped();
        let mut add = false;
        let add_word = arrival_add_word(group);
        // Initiative and Add end at the panel's inner right edge in every group, so they share
        // one column. The summary takes what is left.
        let right = ui.max_rect().right();
        ui.horizontal(|ui| {
            let gap = ui.spacing().item_spacing.x;
            // The field's margins draw it a little wider than it asks for; XS covers that, and
            // the space before Add takes up whatever is left, so Add ends at the edge.
            let controls = design::number_width(ui, "Initiative", &Type::Small.font(ui))
                + gap
                + ARRIVAL_FIELD_WIDTH
                + design::XS
                + gap
                + add_width;
            let width = (right - ui.cursor().left() - controls - gap).max(design::XXL);
            ui.allocate_ui(vec2(width, Density::Read.control()), |ui| {
                ui.set_min_width(width);
                ui.set_max_width(width);
                let summary = arrival_summary(
                    ui,
                    if status_in_heading { "" } else { &status },
                    &members,
                    width,
                );
                let response =
                    ui.add(egui::Label::new(Type::Small.rich(ui, &summary).color(c.muted)).wrap());
                let name = if status.is_empty() {
                    members.clone()
                } else {
                    format!("{status} · {members}")
                };
                design::named(ui, &response, &name);
            });
            ui.add_space((right - controls - ui.cursor().left()).max(0.0));
            let field = format!("Initiative total for {name}");
            ui.label(Type::Small.rich(ui, "Initiative").color(c.muted));
            let text = self.group_totals.entry(group.id).or_default();
            // By id, because two groups can have one name.
            let response = design::locked(ui, group.problem.as_deref(), |ui| {
                design::number_field_with_id(
                    ui,
                    design::focus_id(&field).with(group.id),
                    &field,
                    text,
                    "Roll",
                    ARRIVAL_FIELD_WIDTH,
                    Density::Read.control(),
                )
            });
            response.clone().on_hover_text("Blank rolls initiative");
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_description("Blank rolls initiative")
            });
            let parsed = arrival_total(text);
            let total = parsed.as_ref().ok().copied().flatten();
            let enter = response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
            let leader = group.leader();
            let may_roll = !leader
                .as_ref()
                .is_some_and(|leader| self.joins_side(&leader.side));
            let blocked = group
                .problem
                .as_deref()
                .or(parsed.as_ref().err().copied())
                .or((total.is_none()
                    && may_roll
                    && leader.is_some_and(|leader| leader.initiative_bonus.is_none()))
                .then_some("The first member has no initiative bonus. Type a total."));
            ui.add_space((right - add_width - ui.cursor().left()).max(0.0));
            add = ui
                .allocate_ui_with_layout(
                    vec2(add_width, Density::Read.control()),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.set_width(add_width);
                        Action::secondary(&add_word)
                            .min_width(add_width)
                            .density(Density::Read)
                            .name(&format!("Add {name} to the initiative list"))
                            .blocked(blocked)
                            .show(ui)
                            .clicked()
                    },
                )
                .inner
                || (enter && blocked.is_none());
        });
        if let Err(reason) =
            arrival_total(self.group_totals.get(&group.id).map_or("", String::as_str))
        {
            // The same warning form as the group's faults below: a mark, and words in text colour.
            design::small_message(ui, Tone::Warning, reason);
        }
        let warnings = &group.warnings;
        if !warnings.is_empty() {
            ui.label(Type::Small.rich(ui, "Cannot join").color(c.muted));
        }
        // The members' faults come first in `problem`; the lines after them are the group's own
        // reason, such as no member that can join.
        let own: Vec<super::MemberFault> = group
            .own_problems()
            .into_iter()
            .map(|line| super::MemberFault::new(super::FaultKind::CannotJoin, line.into()))
            .collect();
        if !warnings.is_empty() || !own.is_empty() {
            for fault in warnings.iter().chain(&own) {
                if group_fault(ui, fault) {
                    if fault.kind == super::FaultKind::NotInPackages {
                        self.wants_packages = true;
                    } else {
                        self.wants_group_editor = Some(group.path.clone());
                    }
                }
            }
            // Every fault gives one link; when all point to packages, the group editor
            // stays one click away.
            if warnings
                .iter()
                .chain(&own)
                .all(|fault| fault.kind == super::FaultKind::NotInPackages)
                && design::status_link(
                    ui,
                    Status {
                        icon: icons::EDIT,
                        word: "Edit group",
                        color: colors().accent,
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
        let total = match arrival_total(self.group_totals.get(&id).map_or("", String::as_str)) {
            Ok(total) => total,
            Err(reason) => {
                self.error = Some(reason.into());
                return;
            }
        };
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
        design::group_heading(ui, "Death saves", 0);
        let mut command = None;
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Read.control());
            ui.spacing_mut().item_spacing.x = design::XS;
            ui.label(
                Type::Small
                    .rich(
                        ui,
                        format!(
                            "{successes} {} · {failures} {}",
                            if successes == 1 {
                                "success"
                            } else {
                                "successes"
                            },
                            if failures == 1 { "failure" } else { "failures" }
                        ),
                    )
                    .color(c.text),
            );
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
            // The roll records nothing. The DM adds the success or failure it gives.
            crate::rolls::set_roller(ui.ctx(), Some(index));
            crate::rolls::set_entry(ui.ctx(), &self.state.participants[index].name);
            crate::rolls::link(
                ui,
                "Roll",
                Type::Body,
                &splat::content::dice_text::RollLink::d20(
                    splat::content::dice_text::D20::DeathSave,
                    0,
                    "",
                ),
            );
            crate::rolls::set_roller(ui.ctx(), None);
        });
        if let Some(command) = command {
            self.run_command(command);
        }
    }

    fn reference_tab(&mut self, ui: &mut egui::Ui, index: usize) {
        self.card_loot(ui, index);
        let p = &self.state.participants[index];
        let definition = &p.definition;
        if definition.path == "Made up in this fight" {
            ui.label(
                Type::Body
                    .rich(ui, "Made up in this fight. It has no stat block.")
                    .color(colors().muted),
            );
            ui.horizontal(|ui| {
                for (label, value) in [
                    ("ARMOR CLASS", display(p.armor_class)),
                    (
                        "HIT POINTS",
                        format!("{} / {}", display(p.hp), display(p.max_hp)),
                    ),
                    ("INITIATIVE", display(p.initiative)),
                ] {
                    if label == "ARMOR CLASS" && p.armor_class.is_none() {
                        continue;
                    }
                    ui.vertical(|ui| {
                        ui.label(Type::Label.rich(ui, label).color(colors().muted));
                        ui.label(Type::Prose.rich(ui, value).color(colors().text));
                    });
                    ui.add_space(design::L);
                }
            });
            return;
        }
        // A linked character shows the same sheet as the Party page, with its age.
        if let Some(record) = self.sheet.clone().filter(|_| self.sheet_for == Some(index)) {
            let live = crate::party::LiveValues {
                hp: p.hp,
                max_hp: p.max_hp,
                temporary_hp: p.temporary_hp,
                conditions: p.conditions.clone(),
                death_saves: splat::character::DeathSaves {
                    successes: u64::from(p.successes),
                    failures: u64::from(p.failures),
                },
                death_saves_shown: p.shows_death_saves(),
            };
            changes_block(ui, p, false);
            let freshness = crate::party::freshness(&record, self.connected);
            // The sheet is older than what the DM just set here, so say which is which.
            // While sync reads this sheet, the copy's age says nothing about these values.
            let ready = self.syncs.get(&index).is_some_and(|sync| sync.ready());
            let (word, age) = if ready {
                ("Connected", "")
            } else {
                freshness
                    .word
                    .split_once(" · ")
                    .unwrap_or((&freshness.word, ""))
            };
            let facts = if ready {
                "· Sync reads this sheet · HP from this run".to_owned()
            } else {
                format!("· {age} · HP from this run")
            };
            // Beside an open conflict, green words would say the opposite of what matters.
            if !self
                .syncs
                .get(&index)
                .is_some_and(|sync| sync.state() == SyncState::Conflict)
            {
                // The portrait has a column of its own: a wrapped status line starts below
                // the status, not below the portrait.
                ui.horizontal(|ui| {
                    if let Some(path) = self.storage.as_ref().and_then(|storage| {
                        crate::party_sync::portrait_path(storage.campaign, &record)
                    }) {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::Vec2::splat(design::XXL),
                            egui::Sense::empty(),
                        );
                        crate::art::portrait(
                            ui,
                            rect,
                            Some(&path),
                            &record.definition.splat_name,
                            None,
                        );
                    }
                    ui.vertical(|ui| {
                        ui.horizontal_wrapped(|ui| {
                            design::status(
                                ui,
                                Status {
                                    icon: freshness.icon,
                                    word,
                                    color: freshness.color,
                                },
                            );
                            ui.label(Type::Small.rich(ui, facts).color(colors().muted));
                        });
                    });
                });
            }
            if p.shows_death_saves() {
                ui.add_space(design::L);
                self.death_saves(ui, index, p.successes, p.failures);
            }
            let spells = crate::party::has_spells(&record);
            if !spells && self.sheet_page == 2 {
                self.sheet_page = 0;
            }
            let pages: Vec<_> = crate::party::SHEET_PAGES
                .iter()
                .copied()
                .filter(|(page, _)| *page != 2 || spells)
                .collect();
            design::overflow_tabs_small(ui, &mut self.sheet_page, &pages);
            let page = self.sheet_page;
            super::rolling::gutter_scroll(
                ui,
                egui::ScrollArea::vertical()
                    .id_salt(("run_sheet", record.definition.splat_id, page))
                    .auto_shrink([false, false]),
                |ui| {
                    crate::rolls::set_roller(ui.ctx(), Some(index));
                    self.armed_tiles_begin(ui.ctx());
                    crate::party::sheet_page(ui, &record, page, "", Some(&live));
                    crate::rolls::set_roller(ui.ctx(), None);
                },
            );
            return;
        }
        let mut link = None;
        let mut notice = None;
        if let Some(content) = &self.content {
            let at = content.position_of(&Reference::Id(definition.id));
            let current = at.map(|at| &content.items[at].entry);
            let unreadable = content
                .errors
                .iter()
                .any(|error| error.contains(&definition.path));
            notice = snapshot_notice(definition, current, unreadable);
            if let Some(message) = &notice {
                ui.label(Type::Small.rich(ui, message).color(colors().muted));
                if let Some(at) = at
                    && Action::quiet("Read the new version")
                        .line_start()
                        .show(ui)
                        .clicked()
                {
                    self.lookup_request = Some(at);
                }
            }
        }
        if definition.fields.contains_key("splat_external") {
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(ui, snapshot_values_line(notice.is_some()))
                        .color(colors().document.read_only),
                )
                .wrap(),
            );
        }
        super::rolling::gutter_scroll(
            ui,
            egui::ScrollArea::vertical()
                // By participant: copies of one creature can differ, and each card opens on its
                // own changes, not at the place the last copy was left.
                .id_salt(("run_reference", &p.key))
                .auto_shrink([false, false]),
            |ui| {
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
                        current_hp: p.hp,
                        initiative: p.initiative_bonus,
                        speed: p.changes.as_ref().and_then(|changes| {
                            changes
                                .overrides
                                .rows(&changes.base)
                                .into_iter()
                                .find(|row| row.label == "Speed")
                                .map(|row| row.changed)
                        }),
                    };
                    crate::rolls::set_roller(ui.ctx(), Some(index));
                    self.armed_tiles_begin(ui.ctx());
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
                    crate::rolls::set_roller(ui.ctx(), None);
                }
            },
        );
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
                if p.hp == Some(0) { ui.add_space(design::L); }
                design::group_heading(ui, "Correct HP", 0);
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
                ui.add(
                    egui::Label::new(Type::Small.rich(ui,
                        "You resolve conditions, Concentration, death saves and recovery. Zero HP does not remove a turn."
                    ).color(colors().muted)).wrap(),
                );
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
                    } else if self.current() == Some(index) {
                        // Moving it now would skip others, or give them a second turn.
                        (
                            "The turn stays. If the new total moves this row, it moves when its turn ends.",
                            colors().muted,
                        )
                    } else {
                        (
                            "This row moves. The current turn stays.",
                            colors().muted,
                        )
                    };
                    let had = p.initiative;
                    ui.add_space(design::L);
                    design::group_heading(ui, "Correct initiative", 0);
                    let field = format!("Initiative for {}", p.name);
                    let text = self.setup_inputs.entry(index).or_default();
                    let total = text.trim().parse::<i64>().ok();
                    let mut reason_below = false;
                    ui.horizontal(|ui| {
                        correction_label(ui, "Initiative");
                        let response = design::number_field(
                            ui,
                            &field,
                            text,
                            "Total",
                            72.0,
                            Density::Read.control(),
                        );
                        let enter = response.lost_focus()
                            && ui.input(|i| i.key_pressed(Key::Enter));
                        let pressed = Action::secondary("Set initiative")
                            .density(Density::Read)
                            .blocked_if(total.is_none(), "Type a number first.")
                            .show(ui)
                            .clicked();
                        let reason = total.is_none().then_some("Type a number first.");
                        reason_below = reason.is_some_and(|reason| design::number_width(ui, reason, &Type::Small.font(ui)) > ui.available_width());
                        if !reason_below { design::blocked_reason(ui, reason); }
                        if (pressed || enter)
                            && let Some(total) = total
                        {
                            // The same total again changes nothing, and is no step to undo.
                            if had != Some(total) {
                                command = Some(Command::Initiative(index, Some(total)));
                            }
                            text.clear();
                            self.return_focus = self.lookup_open;
                        }
                    });
                    if reason_below {
                        ui.horizontal(|ui| {
                            correction_label(ui, "");
                            design::blocked_reason(ui, Some("Type a number first."));
                        });
                    }
                    ui.add(egui::Label::new(Type::Small.rich(ui, words).color(color)).wrap());
                }
                ui.add_space(design::L);
                design::group_heading(ui, "DM notes", 0);
                let mut note_reason_below = false;
                ui.horizontal(|ui| {
                    correction_label(ui, "Note");
                    let reason_width = design::number_width(ui, "Type a note first.", &Type::Small.font(ui)) + design::S;
                    let button_width = Action::secondary("Add note").density(Density::Read).natural_width(ui);
                    let empty = self.note.trim().is_empty();
                    note_reason_below = empty && ui.available_width() < button_width + reason_width + design::XXL * 3.0;
                    let reserved = if empty && !note_reason_below { reason_width } else { 0.0 };
                    design::text_input(
                        ui,
                        "DM note",
                        &mut self.note,
                        "What happened",
                        (ui.available_width() - button_width - reserved - ui.spacing().item_spacing.x).max(design::XXL),
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
                    if empty && !note_reason_below { design::blocked_reason(ui, Some("Type a note first.")); }
                });
                if note_reason_below {
                    ui.horizontal(|ui| {
                        correction_label(ui, "");
                        design::blocked_reason(ui, Some("Type a note first."));
                    });
                }
                if !p.notes.is_empty() {
                    ui.add(
                        egui::Label::new(Type::Body.rich(ui, &p.notes).color(colors().text)).wrap(),
                    );
                }
                if p.definition.fields.contains_key("splat_external") {
                    ui.add_space(design::L);
                    design::group_heading(ui, "D&D Beyond combat sync", 0);
                    // Viewing Correct is not a request to stop this character's sync.
                    // Keep a new worker only when the DM enables it.
                    let mut unstarted = crate::combat_sync::CombatSync::default();
                    let sync = self.syncs.get_mut(&index).unwrap_or(&mut unstarted);
                    let was = sync.enabled;
                    ui.push_id(index, |ui| {
                        if self.connected || self.simulate_connection { sync.show(ui); } else {
                            ui.horizontal_wrapped(|ui| {
                                ui.add_enabled(false, egui::Checkbox::new(&mut sync.enabled, "Sync combat stats with D&D Beyond")).on_disabled_hover_text("Connect to sync");
                                if design::link(ui, "Connect to sync").clicked() { self.wants_sign_in = true; }
                            });
                        }
                    });
                    self.dirty |= was != sync.enabled;
                    if unstarted.enabled {
                        self.syncs.insert(index, unstarted);
                    }
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
        self.refresh_roster_faults();
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
        let blocked = if let Some(reason) = self.roster_blocked() {
            Some(reason.to_owned())
        } else if missing_hp > 0 {
            Some(format!("{missing_hp} need valid HP."))
        } else if missing_initiative > 0 {
            Some(format!("{missing_initiative} need initiative."))
        } else {
            None
        };
        let mut command = None;
        let default_roll = self.initiative.grouping;
        let default_hint = format!("Campaign default: {}", default_roll.label());
        let roll_choices = self.roll_choices();
        let setup_origin = ui.cursor().min;
        let setup_width = ui.available_width();
        let setup_header_height = if self.roster_faults.is_empty() {
            Density::Glance.control() + Density::Read.control() + design::XS
        } else {
            Density::Glance.control() + design::M
        };
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), setup_header_height),
            Layout::right_to_left(Align::TOP),
            |ui| {
                ui.set_min_height(setup_header_height);
                ui.allocate_ui_with_layout(
                    vec2(design::S * 18.0, setup_header_height),
                    Layout::top_down(Align::RIGHT),
                    |ui| {
                        if Action::primary("Begin combat")
                            .density(Density::Glance)
                            .blocked(blocked.as_deref())
                            .show(ui)
                            .clicked()
                        {
                            command = Some(Command::Begin);
                        }
                        if let Some(reason) = &blocked {
                            ui.label(Type::Small.rich(ui, reason).color(colors().muted));
                        }
                    },
                );
                ui.allocate_ui_with_layout(
                    vec2(design::S * 20.0, setup_header_height),
                    Layout::top_down(Align::RIGHT),
                    |ui| {
                        let roll = ui
                            .horizontal(|ui| {
                                design::split_action(
                                    ui,
                                    Action::secondary("Roll NPCs")
                                        .name("Roll NPC initiative")
                                        .icon(icons::DICE)
                                        .density(Density::Glance)
                                        .hint(&default_hint),
                                    "Roll NPC initiative another way",
                                    |ui| {
                                        ui.label(
                                            Type::Small
                                                .rich(ui, "This roll only")
                                                .color(colors().muted),
                                        );
                                        for mode in &roll_choices {
                                            let item = ui.selectable_label(
                                                *mode == default_roll,
                                                mode.label(),
                                            );
                                            if item.clicked() {
                                                command = Some(Command::RollNpcs(*mode));
                                            }
                                        }
                                    },
                                )
                            })
                            .inner;
                        if roll.clicked() {
                            command = Some(Command::RollNpcs(default_roll));
                        }
                    },
                );
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    self.run_title(ui, &self.state_words(false));
                    let tied = self.tied_count();
                    if tied > 0 {
                        ui.label(
                            Type::Small
                                .rich(
                                    ui,
                                    format!(
                                        "{} {}",
                                        icons::WARN,
                                        design::count(tied, "tied", "tied")
                                    ),
                                )
                                .color(colors().result.warning),
                        );
                    }
                });
            },
        );
        if blocked.is_none() {
            let first = self
                .state
                .order
                .iter()
                .copied()
                .find(|i| self.state.participants[*i].present);
            let text = first.map_or_else(
                || "Everyone is ready".to_owned(),
                |i| {
                    format!(
                        "Everyone is ready · {} goes first",
                        self.state.participants[i].name
                    )
                },
            );
            let rect = Rect::from_min_size(
                setup_origin + vec2(0.0, Density::Glance.control() + design::XS),
                vec2(setup_width, Density::Read.control()),
            );
            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(rect)
                    .layout(Layout::right_to_left(Align::Center)),
                |ui| {
                    ui.add(
                        egui::Label::new(
                            Type::Small
                                .rich(ui, format!("{} {text}", icons::OK))
                                .color(colors().muted),
                        )
                        .wrap()
                        .halign(Align::RIGHT),
                    );
                },
            );
        }
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), Density::Read.control()),
            Layout::right_to_left(Align::Center),
            |ui| {
                // Add to fight keeps its room beside Undo, which shortens first.
                let add = self.add_to_fight(true, Density::Read);
                let undo_room = ui.available_width() - add.natural_width(ui) - design::S;
                let label = self.undo_label();
                let undo = ui
                    .allocate_ui_with_layout(
                        vec2(undo_room, Density::Read.control()),
                        Layout::right_to_left(Align::Center),
                        |ui| {
                            if self.history.is_empty() {
                                self.empty_undo(ui)
                            } else {
                                self.undo_action(&label)
                                    .name(&label)
                                    .hint(&label)
                                    .show(ui)
                                    .clicked()
                            }
                        },
                    )
                    .inner;
                ui.add_space(design::S);
                if add.show(ui).clicked() {
                    self.show_arrivals();
                }
                if undo {
                    self.undo();
                    self.return_focus = self.lookup_open;
                }
            },
        );
        design::rule(ui);
        if let Some(path) = &self.roster_source {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();
            let last = self.roster_faults.len().saturating_sub(1);
            for (index, fault) in self.roster_faults.iter().enumerate() {
                let words = fault.words();
                // One file names every unread row, so its name and actions show once, after
                // the last warning about it.
                if index < last {
                    design::small_message(ui, Tone::Warning, &words);
                    continue;
                }
                ui.scope(|ui| {
                    ui.spacing_mut().item_spacing.y = design::XS;
                    let gap = ui.spacing().item_spacing.x;
                    let inset = design::S
                        + design::HAIRLINE_WIDTH
                        + design::number_width(ui, icons::WARN, &Type::Body.font(ui))
                        + gap * 2.0
                        - design::XS;
                    let message_width =
                        design::number_width(ui, &words, &Type::Body.font(ui)) + inset;
                    let file_width = design::number_width(ui, &filename, &Type::Small.font(ui))
                        + design::number_width(ui, "Edit in Obsidian", &Type::Body.font(ui))
                        + design::number_width(ui, "Show file", &Type::Body.font(ui))
                        + gap * 2.0;
                    if message_width + design::L + file_width <= ui.available_width() {
                        ui.horizontal_top(|ui| {
                            ui.allocate_ui_with_layout(
                                vec2(message_width, 0.0),
                                Layout::top_down(Align::Min),
                                |ui| {
                                    design::small_message(ui, Tone::Warning, &words);
                                },
                            );
                            ui.add_space(design::L - gap);
                            crate::world::entry_file_parts(
                                ui,
                                path.parent().unwrap_or(path),
                                &filename,
                                true,
                            );
                        });
                    } else {
                        // The file line starts at the warning's words, as every message's
                        // actions do, and its name wraps whole rather than being cut.
                        let words_left =
                            design::small_message(ui, Tone::Warning, &words).rect.left();
                        let inset = words_left - ui.cursor().left();
                        ui.horizontal_top(|ui| {
                            ui.add_space(inset);
                            ui.vertical(|ui| {
                                crate::encounters::file_line(
                                    ui,
                                    path.parent().unwrap_or(path),
                                    &filename,
                                );
                            });
                        });
                    }
                });
            }
        }
        self.error_line(ui);
        if let Some(words) = self.carry_missing() {
            design::message(ui, Tone::Warning, &words);
        }
        if self.rest_filled_saved_hp() {
            if design::message_with_action(
                ui,
                Tone::Info,
                "Party HP is full after the long rest.",
                "Use the saved HP",
            ) && let Err(e) = self.use_saved_hp()
            {
                self.error = Some(e);
            }
        } else if self.saved_hp_used() {
            // The same slot, so the DM sees the result where they pressed, and can take it back.
            if design::undo_message(ui, Tone::Success, SAVED_HP_USED, "Undo") {
                self.undo();
            }
        } else if let Some((from, back)) = self.back_to_sheets()
            && design::message_with_action(
                ui,
                Tone::Info,
                &format!("Party HP carries over from {from}."),
                "Use the sheets’ HP",
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
                let mut spans = Vec::new();
                let output = egui::ScrollArea::vertical()
                    .id_salt("run_setup_rows")
                    .max_height((room - OVERFLOW_LINE).max(0.0))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        // Include the heading when comparing rows with the scroll viewport.
                        let origin = ui.cursor().min.y;
                        ui.spacing_mut().item_spacing.y = 0.0;
                        let (_, right) = self.setup_slots(ui, false);
                        let number = design::L * 3.0;
                        ui.horizontal(|ui| {
                            ui.add_space((ui.available_width() - right - number).max(0.0));
                            ui.label(Type::Label.rich(ui, "INITIATIVE").color(colors().muted));
                        });
                        // Roster order. Rows hold still while the DM types (T4.3).
                        for index in 0..self.state.participants.len() {
                            let row = ui.push_id(index, |ui| {
                                if let Some(picked) = self.setup_row(ui, index) {
                                    command = Some(picked);
                                }
                            });
                            spans.push((
                                row.response.rect.top() - origin,
                                row.response.rect.bottom() - origin,
                                index,
                            ));
                        }
                        let mut shared: std::collections::BTreeMap<&str, Vec<Rect>> =
                            std::collections::BTreeMap::new();
                        for p in &self.state.participants {
                            if let Some(key) = p.shared_roll.as_deref()
                                && let Some(field) = ui.ctx().read_response(design::focus_id(
                                    &format!("Initiative for {}", p.name),
                                ))
                            {
                                shared.entry(key).or_default().push(field.rect);
                            }
                        }
                        for fields in shared.values().filter(|fields| fields.len() > 1) {
                            let top = fields.iter().map(Rect::top).fold(f32::INFINITY, f32::min);
                            let bottom = fields
                                .iter()
                                .map(Rect::bottom)
                                .fold(f32::NEG_INFINITY, f32::max);
                            let x = fields[0].left() - design::XS;
                            let stroke = egui::Stroke::new(design::HAIRLINE_WIDTH, colors().muted);
                            for field in fields {
                                ui.painter().vline(x, field.y_range(), stroke);
                            }
                            ui.painter().hline(x..=x + design::XS, top, stroke);
                            ui.painter().hline(x..=x + design::XS, bottom, stroke);
                        }
                    });
                self.overflow_note(ui, &output, &spans);
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
                    closed = self.request_close();
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

    /// Place has a slot from the start. Only the tie slot changes on blur.
    /// Roll precedes initiative so party and NPC totals share one column.
    /// When `Roll NPCs` will give two NPCs one roll, their tie slot is there from the start,
    /// so the roll moves no field. Typed ties still wait for blur.
    fn setup_slots(&self, ui: &egui::Ui, npc: bool) -> ((f32, f32, f32), f32) {
        let grouped_roll =
            !self.initiative.roster_ties && self.rolls_a_group(self.initiative.grouping);
        let tie = if grouped_roll
            || self
                .state
                .order
                .iter()
                .any(|&i| self.setup_tie_directions(i).is_some())
        {
            Density::Read.control() * 2.0
        } else {
            0.0
        };
        let place = design::XL;
        // Freeze only the tie slot while an initiative field has focus. Place numbers can
        // appear immediately without moving the total under the cursor. Tab to Absent or a
        // menu is a blur, so the tie slot settles then.
        let slot_width = egui::Id::new("setup_tie_slot_width");
        let in_initiative = ui.memory(|memory| memory.focused()).is_some_and(|focused| {
            self.state
                .participants
                .iter()
                .any(|p| focused == design::focus_id(&format!("Initiative for {}", p.name)))
        });
        let tie = if in_initiative {
            ui.data(|data| data.get_temp::<f32>(slot_width))
                .unwrap_or(tie)
        } else {
            ui.data_mut(|data| data.insert_temp(slot_width, tie));
            tie
        };
        let roll = if npc { design::L * 4.0 } else { 0.0 };
        let presence = design::number_width(
            ui,
            &format!("{} Absent", icons::ABSENT),
            &Type::Body.font(ui),
        ) + ui.spacing().button_padding.x * 2.0
            + design::S;
        (
            (tie, place, roll),
            tie + place + design::S + Density::Read.control() + presence,
        )
    }

    /// Controls share the name line. Ties reserve a column only while tied rows exist.
    /// Enter moves to the next missing total; rare controls wait in the row menu.
    fn setup_row(&mut self, ui: &mut egui::Ui, index: usize) -> Option<Command> {
        let mut command = None;
        let p = self.state.participants[index].clone();
        // What the DM weighs before a fight: who is hurt, and what is still on them.
        let side_icon = match p.side.as_str() {
            "party" => icons::PARTY,
            "enemies" => icons::ENCOUNTERS,
            _ => icons::GROUP,
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
        if let Some((icon, word, _)) = hp_word(&p) {
            numbers.push_str(&format!(" · {icon} {word}"));
        }
        let selected = self.panel == Panel::Record && self.selected == index;
        let (name, npc, mode, bonus, initiative, present) = (
            p.name.clone(),
            p.npc,
            p.initiative_mode,
            p.initiative_bonus,
            p.initiative,
            p.present,
        );
        let expanded = self.setup_expanded == Some(index);
        let mut enter = false;
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            // Keep the worded toggle in every width; party rows give up their per-row Roll.
            let presence_slot = design::number_width(
                ui,
                &format!("{} Absent", icons::ABSENT),
                &Type::Body.font(ui),
            ) + ui.spacing().button_padding.x * 2.0
                + design::S;
            let ((tie_slot, place_slot, roll_slot), right) = self.setup_slots(ui, npc);
            let number_slot = design::L * 3.0;
            let actions = number_slot + roll_slot + right;
            let name_width = (ui.available_width() - actions).max(design::XL);
            let side = "";
            let plain = format!("{absent}{side}{numbers}");
            // Changed values keep their mark beside the setup numbers.
            let detail = if !p.stat_changes() {
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
            let width = name_width - 2.0 * design::S;
            let title =
                ui.painter()
                    .layout(name.clone(), Type::Body.font(ui), colors().text, width);
            let first_line = Density::Scan.control().max(title.size().y);
            let detail_width = ui.available_width() - 2.0 * design::S;
            let mut detail = detail;
            if p.npc_identity() {
                detail.push_str(&format!(" · as {}", p.definition.name));
            }
            for (at, condition) in p.conditions.iter().enumerate() {
                let next = format!("{detail} · {condition}");
                let remaining = p.conditions.len() - at - 1;
                let reserve = if remaining > 0 {
                    format!(" · {remaining} more")
                } else {
                    String::new()
                };
                if at > 0
                    && design::number_width(ui, &format!("{next}{reserve}"), &Type::Small.font(ui))
                        > detail_width
                {
                    detail.push_str(&format!(" · {} more", p.conditions.len() - at));
                    break;
                }
                detail = next;
            }
            let row_name = format!("{name} · {detail}");
            let detail = detail
                .split(" · ")
                .map(|part| part.replace(' ', "\u{a0}"))
                .collect::<Vec<_>>()
                .join(" ·\u{a0}");
            let status =
                hp_word(&p).map(|(icon, word, color)| (format!("{icon}\u{a0}{word}"), color));
            let detail = design::status_in_text(
                ui,
                &detail,
                status.as_ref().map(|(word, color)| (word.as_str(), *color)),
                detail_width,
            );
            let row = first_line + detail.size().y + design::XS;
            let padding = (row - first_line - detail.size().y) / 2.0;
            let control_top = padding + (first_line - Density::Scan.control()) / 2.0;
            if selected {
                ui.painter().rect_filled(
                    Rect::from_min_size(ui.cursor().min, vec2(ui.available_width(), row)),
                    0.0,
                    colors().selected,
                );
            }
            let response = ui
                .allocate_ui(vec2(name_width, row), |ui| {
                    design::row(ui, row, selected, &row_name, |ui, rect| {
                        let top = rect.top() + padding;
                        ui.painter().galley(
                            pos2(
                                rect.left() + design::S,
                                top + (first_line - title.size().y) / 2.0,
                            ),
                            title.clone(),
                            colors().text,
                        );
                        ui.painter().galley(
                            pos2(rect.left() + design::S, top + first_line),
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
            if npc {
                ui.allocate_ui_with_layout(
                    vec2(roll_slot, Density::Scan.control()),
                    Layout::top_down(Align::LEFT),
                    |ui| {
                        ui.set_width(roll_slot);
                        ui.add_space(control_top);
                        if Action::quiet("Roll")
                            .plain()
                            .icon(icons::DICE)
                            .density(Density::Scan)
                            .name(&format!("Roll for {name}"))
                            .blocked_if(
                                bonus.is_none(),
                                "Set an initiative bonus, or type a total.",
                            )
                            .show(ui)
                            .clicked()
                        {
                            command = Some(Command::RollInitiative(index));
                        }
                    },
                );
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
            ui.allocate_ui_with_layout(
                vec2(number_slot, row),
                Layout::top_down(Align::LEFT),
                |ui| {
                    ui.set_width(number_slot);
                    ui.spacing_mut().item_spacing.y = 0.0;
                    ui.add_space(control_top);
                    let text = self
                        .setup_inputs
                        .entry(index)
                        .or_insert_with(|| initiative.map_or_else(String::new, |n| n.to_string()));
                    let response = design::number_field(
                        ui,
                        &field_name,
                        text,
                        "",
                        number_slot,
                        Density::Scan.control(),
                    );
                    let parsed = text.parse::<i64>().ok();
                    if response.changed() {
                        if parsed != initiative {
                            command = Some(Command::Initiative(index, parsed));
                        }
                    } else if !response.has_focus() && parsed != initiative {
                        *text = initiative.map_or_else(String::new, |n| n.to_string());
                    }
                    enter = response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
                    if self.focus_field == Some(index) {
                        self.focus_field = None;
                        response.request_focus();
                    }
                },
            );
            if tie_slot > 0.0 {
                ui.allocate_ui_with_layout(
                    vec2(tie_slot, row),
                    Layout::top_down(Align::LEFT),
                    |ui| {
                        ui.set_width(tie_slot);
                        ui.add_space(control_top);
                        if let Some(picked) = self.setup_tie_controls(ui, index) {
                            command = Some(picked);
                        }
                    },
                );
            }
            if place_slot > 0.0 {
                ui.allocate_ui_with_layout(
                    vec2(place_slot, row),
                    Layout::top_down(Align::LEFT),
                    |ui| {
                        ui.add_space(control_top);
                        let place = (present
                            && self
                                .state
                                .participants
                                .iter()
                                .all(|p| !p.present || p.initiative.is_some()))
                        .then(|| {
                            initiative
                                .and_then(|_| self.state.order.iter().position(|i| *i == index))
                        })
                        .flatten()
                        .map(|place| ordinal(place + 1))
                        .unwrap_or_default();
                        let response = ui.add_sized(
                            vec2(place_slot, Density::Scan.control()),
                            egui::Label::new(Type::Small.rich(ui, &place).color(colors().muted)),
                        );
                        design::named(ui, &response, &format!("Place for {name}: {place}"));
                    },
                );
            }
            ui.add_space(design::S);
            let (word, hint) = presence_action(present);
            let accessible = format!("{word}: {name}");
            let pressed = ui
                .allocate_ui_with_layout(
                    vec2(presence_slot, Density::Scan.control()),
                    Layout::top_down(Align::LEFT),
                    |ui| {
                        ui.set_width(presence_slot);
                        ui.add_space(control_top);
                        let button = egui::Button::new(
                            Type::Body
                                .rich(ui, format!("{} {word}", icons::ABSENT))
                                .color(if present {
                                    colors().muted
                                } else {
                                    colors().text
                                }),
                        )
                        .wrap_mode(egui::TextWrapMode::Extend)
                        .selected(!present)
                        .fill(if present {
                            colors().page
                        } else {
                            colors().selected
                        })
                        .frame(!present)
                        .min_size(vec2(presence_slot, Density::Scan.control()));
                        let response = ui.add(button).on_hover_text(hint);
                        design::named(ui, &response, &accessible);
                        response.clicked()
                    },
                )
                .inner;
            if pressed {
                command = Some(Command::Presence(index, !present));
            }
            ui.allocate_ui_with_layout(
                vec2(Density::Read.control(), row),
                Layout::top_down(Align::LEFT),
                |ui| {
                    ui.add_space(padding + (first_line - Density::Read.control()) / 2.0);
                    design::row_menu(ui, &name, |ui| {
                        if Action::quiet("Correct…").show(ui).clicked() {
                            self.selected = index;
                            self.panel = Panel::Record;
                            self.close_lookup = true;
                            ui.close();
                        }
                        if Action::quiet("NPC and roll mode").show(ui).clicked() {
                            self.setup_expanded = if expanded { None } else { Some(index) };
                            self.return_focus = self.lookup_open;
                            ui.close();
                        }
                    });
                },
            );
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
            self.fight_panel_title()
        } else {
            &self.state.participants[self.selected].name
        };
        ui.label(Type::Title.rich(ui, title).color(colors().text));
        let reference = if self.state.participants[self.selected]
            .definition
            .fields
            .contains_key("splat_external")
        {
            "Sheet"
        } else {
            "Stat block"
        };
        self.panel_tabs(ui, Some(reference), true);
        design::rule(ui);
        self.sync_block(ui);
        match self.panel {
            Panel::Reference => self.reference_tab(ui, self.selected),
            Panel::Conditions => self.conditions_tab(ui, self.selected),
            Panel::Keys => self.keys_tab(ui),
            Panel::Record => {
                let p = &self.state.participants[self.selected];
                changes_block(ui, p, false);
                self.record_tab(ui, self.selected);
            }
            Panel::Log => self.log_tab(ui),
            Panel::Encounter => self.notes_tab(ui),
            Panel::Groups => self.groups_tab(ui),
        }
    }

    /// How many the DM must order, when Notes hide the panel that says so.
    pub(super) fn tied_count(&self) -> usize {
        (0..self.state.participants.len())
            .filter(|&index| self.setup_tied(index))
            .count()
    }

    fn setup_tied(&self, index: usize) -> bool {
        let p = &self.state.participants[index];
        !self.initiative.roster_ties
            && p.initiative.is_some()
            && self
                .state
                .participants
                .iter()
                .enumerate()
                .any(|(other, q)| {
                    index != other
                        && p.initiative == q.initiative
                        && !self.shared_initiative(index, other)
                })
    }

    fn setup_hidden_attention(&self, rows: &[usize]) -> String {
        let missing = rows
            .iter()
            .filter(|&&i| {
                self.state.participants[i].present
                    && self.state.participants[i].initiative.is_none()
            })
            .count();
        let tied = rows.iter().filter(|&&i| self.setup_tied(i)).count();
        let mut words = String::new();
        if missing > 0 {
            words.push_str(&format!(
                ", {missing} {} initiative",
                if missing == 1 { "needs" } else { "need" }
            ));
        }
        if tied > 0 {
            words.push_str(&format!(", {}", design::count(tied, "tied", "tied")));
        }
        words
    }

    fn shared_initiative(&self, a: usize, b: usize) -> bool {
        let p = &self.state.participants[a];
        let q = &self.state.participants[b];
        p.shared_roll.is_some() && p.shared_roll == q.shared_roll
    }

    fn setup_tie_directions(&self, index: usize) -> Option<(usize, bool, bool)> {
        let position = self.state.order.iter().position(|i| *i == index)?;
        let p = &self.state.participants[index];
        let tied = |other: Option<&usize>| {
            !self.initiative.roster_ties
                && p.initiative.is_some()
                && other.is_some_and(|i| self.state.participants[*i].initiative == p.initiative)
        };
        let up = position > 0 && tied(self.state.order.get(position - 1));
        let down = tied(self.state.order.get(position + 1));
        (up || down).then_some((position, up, down))
    }

    /// Tied rows alone have two fixed arrow slots under Init.
    fn setup_tie_controls(&self, ui: &mut egui::Ui, index: usize) -> Option<Command> {
        let (position, up, down) = self.setup_tie_directions(index)?;
        let p = &self.state.participants[index];
        let mut command = None;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            for (allowed, icon, destination, relation) in [
                (up, icons::MOVE_UP, position.saturating_sub(1), "before"),
                (down, icons::MOVE_DOWN, position + 1, "after"),
            ] {
                if allowed {
                    let name = format!(
                        "Move {} {relation} {}",
                        p.name, self.state.participants[self.state.order[destination]].name
                    );
                    if design::icon_button(ui, icon, &name).clicked() {
                        command = Some(Command::Move(position, destination));
                    }
                } else {
                    ui.allocate_space(vec2(Density::Read.control(), Density::Read.control()));
                }
            }
        });
        command
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
        let (header_rect, _) = ui.allocate_exact_size(
            vec2(ui.available_width(), design::L * 4.0),
            egui::Sense::hover(),
        );
        design::rule(ui);
        self.error_line(ui);
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
                    // A quiet action at the text edge (`Stop sync for …`) reaches `S` into the
                    // panel's gutter. The scroll area starts there too, so it shows the whole
                    // target, and its margin keeps the text where it was.
                    let area = ui.available_rect_before_wrap();
                    ui.scope_builder(
                        egui::UiBuilder::new().max_rect(area.with_min_x(area.min.x - design::S)),
                        |ui| {
                            egui::ScrollArea::vertical()
                                .id_salt("run_ended_sync")
                                .max_height(room)
                                .auto_shrink([false, true])
                                .show(ui, |ui| {
                                    egui::Frame::new()
                                        .inner_margin(egui::Margin {
                                            left: design::S as i8,
                                            ..egui::Margin::ZERO
                                        })
                                        .show(ui, |ui| {
                                            self.sync_block(ui);
                                            if !self.setup_details.is_empty() {
                                                design::section(ui, "Prep", |ui| {
                                                    self.notes_tab(ui);
                                                });
                                            }
                                        });
                                });
                        },
                    );
                    if choosing {
                        design::section(ui, "Combat log", |ui| self.log_tab(ui));
                    } else {
                        design::group_label(ui, "Combat log");
                        self.log_tab(ui);
                    }
                });
        }
        let mut roster_right = ui.available_rect_before_wrap().left();
        egui::CentralPanel::default()
            // The panel's drag handle lies on this edge, so the rows stop short of it.
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                right: design::S as i8,
                ..egui::Margin::ZERO
            }))
            .show_inside(ui, |ui| {
                let scale = RunScale::fit(
                    ui.available_height(),
                    ui.available_width(),
                    self.state.order.len(),
                );
                egui::ScrollArea::vertical()
                    .id_salt("run_final_state")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_max_width(ui.available_width().min(scale.max_width));
                        // What the party takes comes first, then how everyone ended.
                        self.loot_section(ui);
                        design::group_label(ui, "Final state");
                        ui.spacing_mut().item_spacing.y = 0.0;
                        roster_right = ui.available_rect_before_wrap().right();
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
                            let response =
                                design::row(ui, layout.height(scale), false, &name, |ui, rect| {
                                    paint_row(
                                        ui,
                                        rect,
                                        p,
                                        TurnMark::None,
                                        sync,
                                        scale,
                                        &layout,
                                        None,
                                        None,
                                    );
                                });
                            if layout.name_cut {
                                response.on_hover_text(&p.name);
                            }
                        }
                    });
            });
        let mut header_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt("ended_header")
                .max_rect(header_rect),
        );
        header_ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.set_min_height(60.0);
            let close_reason = self.close_block().map(|_| {
                let conflicts = self.conflict_count();
                let pending = self
                    .syncs
                    .values()
                    .filter(|sync| {
                        sync.unconfirmed()
                            && sync.state() != SyncState::Conflict
                            && !sync.stranded()
                    })
                    .count();
                let mut parts = Vec::new();
                if conflicts > 0 {
                    parts.push(design::count(conflicts, "conflict", "conflicts"));
                }
                if pending > 0 {
                    parts.push(design::count(pending, "pending change", "pending changes"));
                }
                format!("Settle {} first.", parts.join(" and "))
            });
            let close_width = close_reason.as_ref().map_or_else(
                || {
                    Action::primary("Close run")
                        .density(Density::Glance)
                        .natural_width(ui)
                },
                |reason| {
                    design::number_width(ui, reason, &Type::Small.font(ui))
                        .min(ui.available_width() / 2.0)
                },
            );
            ui.allocate_ui_with_layout(
                vec2(
                    close_width,
                    Density::Glance.control()
                        + if close_reason.is_some() {
                            Type::Small.line() + ui.spacing().item_spacing.y
                        } else {
                            0.0
                        },
                ),
                Layout::top_down(Align::RIGHT),
                |ui| {
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
                        closed = self.request_close();
                    }
                    design::blocked_reason(ui, close_reason.as_deref());
                },
            );
            ui.add_space((ui.available_rect_before_wrap().right() - roster_right).max(0.0));
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
                self.run_title(ui, &self.state_words(false));
                ui.label(
                    Type::Small
                        .rich(ui, format!("Outcome: {outcome}"))
                        .color(c.muted),
                );
            });
        });

        closed
    }
}

/// The level in `Exhaustion (2)`. Plain `Exhaustion` is the first.
pub(super) fn exhaustion_level(condition: &str) -> u8 {
    condition
        .trim()
        .strip_prefix("Exhaustion (")
        .and_then(|rest| rest.strip_suffix(')'))
        .and_then(|level| level.trim().parse().ok())
        .unwrap_or(1)
}

/// The width of a group's Initiative total field in the Arrivals panel.
const ARRIVAL_FIELD_WIDTH: f32 = 72.0;

/// The highest exhaustion level. The run's stepper stops here and says why.
const EXHAUSTION_MAX: i64 = 6;
const EXHAUSTION_AT_LOW: &str = "Exhaustion is already 0.";
const EXHAUSTION_AT_HIGH: &str = "Exhaustion stops at 6.";

/// An exhaustion drag in progress: the level it started at, and the level shown now.
#[derive(Clone, Copy, Debug)]
pub(super) struct ExhaustionDrag {
    index: usize,
    at: usize,
    start: u8,
    level: i64,
    /// The frame that last drew this drag. A drag the panel stopped drawing is dropped, so it
    /// cannot commit later without the DM.
    frame: u64,
}

/// One step of undo: the old level goes, and the new one comes.
fn set_exhaustion(index: usize, at: usize, level: u8) -> Command {
    Command::Several(vec![
        Command::RemoveCondition(index, at),
        Command::Condition(index, format!("Exhaustion ({level})")),
    ])
}

/// The line under a character's card. The notice above it, when it shows, already says the
/// card is the copy from the start of the fight, so the line does not say it again.
fn snapshot_values_line(notice_shown: bool) -> &'static str {
    if notice_shown {
        "The roster has the current values."
    } else {
        "Saved when this run began. The roster has the current values."
    }
}

/// The action names what it does, so its label cannot be read as a state.
/// What an encounter or group changed on this creature: the value it replaced and the new
/// one, each marked. It stays out in the open, above the stat block that keeps its own values.
fn snapshot_notice(snapshot: &Entry, current: Option<&Entry>, unreadable: bool) -> Option<String> {
    let name = std::path::Path::new(&snapshot.path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    // The card says first what it shows: the copy the run took at its start.
    let lead = format!(
        "This {} uses the copy from the start of the fight.",
        if snapshot.fields.contains_key("splat_external") {
            "sheet"
        } else {
            "stat block"
        }
    );
    let Some(current) = current else {
        if !unreadable
            && let Some(path) = snapshot
                .fields
                .get("splat_run_source_path")
                .and_then(|value| value.as_str())
            && std::path::Path::new(path).is_file()
        {
            let owner = snapshot
                .fields
                .get("splat_run_source_owner")
                .and_then(|value| value.as_str())
                .unwrap_or("This package");
            return Some(format!("{lead} {owner} is not used in this campaign now."));
        }
        return Some(format!(
            "{lead} {name} {}.",
            if unreadable {
                "cannot be read now"
            } else if snapshot.fields.contains_key("splat_run_source_path") {
                "was deleted"
            } else {
                "is not available in this campaign now"
            }
        ));
    };
    let fields = |entry: &Entry| {
        entry
            .fields
            .iter()
            .filter(|(key, _)| {
                !key.as_str()
                    .is_some_and(|key| key == "splat_schema" || key.starts_with("splat_run_"))
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<serde_yaml_ng::Mapping>()
    };
    // Normalization ignores schema/run metadata. With the same model and kind,
    // identical source bodies therefore have identical normalized bodies too.
    (fields(snapshot) != fields(current)
        || ((snapshot.body != current.body || snapshot.kind != current.kind)
            && splat::content::stat_blocks::comparison_body(snapshot)
                != splat::content::stat_blocks::comparison_body(current)))
    .then(|| format!("{lead} {name} has changed."))
}

fn change_origin(p: &Participant) -> Option<String> {
    let mut parts = Vec::new();
    if let Some(added) = &p.added_from
        && (!p.stat_changes() || added.group.is_empty())
    {
        parts.push(if added.group.is_empty() {
            "Added by hand".into()
        } else {
            format!("Arrived with {}", added.group)
        });
    }
    let has_name_row = p.changes.as_ref().is_some_and(|changes| {
        changes
            .overrides
            .rows(&changes.base)
            .iter()
            .any(|row| row.label == "Name")
    });
    if p.named_stat_block() && (p.npc_identity() || !has_name_row) {
        parts.push(format!("as {}", p.definition.name));
    }
    (!parts.is_empty()).then(|| parts.join(" · "))
}

fn changes_block(ui: &mut egui::Ui, p: &Participant, above_stat_block: bool) {
    let c = colors();
    let heading = |ui: &mut egui::Ui| {
        if let Some(origin) = change_origin(p) {
            ui.add(egui::Label::new(Type::Small.rich(ui, origin).color(c.muted)).wrap());
        }
        if p.stat_changes() {
            let word = p
                .added_from
                .as_ref()
                .filter(|added| !added.group.is_empty())
                .map_or_else(
                    || "Changed".to_owned(),
                    |added| format!("Changed · arrived with {}", added.group),
                );
            design::status(
                ui,
                Status {
                    icon: icons::CHANGED,
                    word: &word,
                    color: c.participant.changed,
                },
            );
        }
    };
    let Some(changes) = p.changes.as_ref().filter(|_| p.stat_changes()) else {
        heading(ui);
        return;
    };
    let rows: Vec<_> = changes
        .overrides
        .rows(&changes.base)
        .into_iter()
        .filter(|row| {
            !above_stat_block || !matches!(row.label, "Maximum HP" | "AC" | "Initiative bonus")
        })
        .filter(|row| !p.npc_identity() || row.label != "Name")
        .collect();
    if above_stat_block {
        let by_hand = rows
            .iter()
            .filter(|row| row.by_hand)
            .map(|row| {
                let value = row.changed.split(" on each").next().unwrap_or(&row.changed);
                let value = value.split(" (").next().unwrap_or(value);
                format!(
                    "{} {value}",
                    row.label.trim_end_matches(" bonus").to_lowercase()
                )
            })
            .collect::<Vec<_>>();
        if !by_hand.is_empty() {
            ui.add(
                egui::Label::new(
                    Type::Body
                        .rich(ui, format!("Add by hand: {}", by_hand.join(", ")))
                        .color(c.participant.changed),
                )
                .wrap(),
            );
        }
        heading(ui);
        change_rows(
            ui,
            &rows
                .iter()
                .filter(|row| !row.by_hand)
                .cloned()
                .collect::<Vec<_>>(),
        );
    } else {
        heading(ui);
        change_rows(ui, &rows);
    }
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
    let separate = !by_hand.is_empty() && !applied.is_empty();
    let mut was_by_hand = false;
    for row in by_hand.into_iter().chain(applied) {
        if separate && was_by_hand && !row.by_hand {
            ui.add_space(design::S);
        }
        was_by_hand = row.by_hand;
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
        add(&change_preview(row), c.participant.changed);
        ui.add(egui::Label::new(job).wrap());
    }
}

pub(crate) fn change_preview(row: &Row) -> String {
    fn numbers(text: &str) -> String {
        let mut result = String::new();
        let mut number = String::new();
        for c in text.chars().chain(std::iter::once('\0')) {
            if c.is_ascii_digit() || (c == '-' && number.is_empty()) {
                number.push(c);
            } else {
                if !number.is_empty() {
                    result.push_str(&super::grouped_number(&number));
                    number.clear();
                }
                if c != '\0' {
                    result.push(c);
                }
            }
        }
        result
    }
    format!(
        "{} → {}{}",
        numbers(if row.label == "Speed" {
            row.base.trim().trim_end_matches('.')
        } else {
            &row.base
        }),
        numbers(&row.changed),
        if row.by_hand {
            " · add by hand"
        } else if row.in_rolls {
            " · in Splat’s rolls"
        } else {
            ""
        }
    )
}

/// Keep a member's repair at the end of that member's fault, with no overlapping targets.
/// The fault's tone and link come from its kind, so it looks the same on every screen.
pub(crate) fn group_fault(ui: &mut egui::Ui, fault: &super::MemberFault) -> bool {
    let packages = fault.kind == super::FaultKind::NotInPackages;
    let tone = fault.tone();
    let status = Status {
        icon: if packages {
            icons::CAMPAIGN
        } else {
            icons::EDIT
        },
        word: if packages {
            "Campaign → Packages"
        } else {
            "Edit group"
        },
        color: colors().accent,
    };
    let gap = ui.spacing().item_spacing.x;
    let inset = design::S
        + design::HAIRLINE_WIDTH
        + design::number_width(ui, icons::WARN, &Type::Body.font(ui))
        + gap * 2.0
        - design::XS;
    let text_width = design::number_width(ui, fault, &Type::Body.font(ui)) + inset;
    let link_width = design::number_width(
        ui,
        &format!("{} {}", status.icon, status.word),
        &Type::Small.font(ui),
    );
    if text_width + design::L + link_width <= ui.available_width() {
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(vec2(text_width, 0.0), Layout::top_down(Align::Min), |ui| {
                design::message(ui, tone, fault);
            });
            ui.add_space(design::L - gap);
            design::status_link(ui, status).clicked()
        })
        .inner
    } else {
        // A notice action: the link's words start at the warning text.
        design::message_with_actions(ui, tone, fault, |ui| {
            hanging_link(ui, status.icon, |ui| {
                design::status_link(ui, status).clicked()
            })
        })
    }
}

/// A repair link under a message. Its words start at the message text, and its icon hangs in
/// the gutter before them.
pub(crate) fn hanging_link(
    ui: &mut egui::Ui,
    icon: &str,
    link: impl FnOnce(&mut egui::Ui) -> bool,
) -> bool {
    let hang = design::number_width(ui, &format!("{icon} "), &Type::Small.font(ui));
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(-hang);
        link(ui)
    })
    .inner
}

fn arrival_summary(ui: &egui::Ui, status: &str, members: &str, width: f32) -> String {
    if status.is_empty() {
        return members.to_owned();
    }
    let status = status.replace(' ', "\u{a0}");
    let first = members.split(", ").next().unwrap_or(members);
    let joined = format!("{status}\u{a0}·\u{a0}{first}");
    if design::number_width(ui, &joined, &Type::Small.font(ui)) <= width {
        format!("{status}\u{a0}·\u{a0}{members}")
    } else if design::number_width(ui, &status, &Type::Small.font(ui)) <= width {
        format!("{status}\n{members}")
    } else {
        format!(
            "{}\n{members}",
            status.split("\u{a0}·\u{a0}").collect::<Vec<_>>().join("\n")
        )
    }
}

fn ordinal(place: usize) -> String {
    let suffix = if (11..=13).contains(&(place % 100)) {
        "th"
    } else {
        match place % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };
    format!("{place}{suffix}")
}

/// What the DM chose among a close dialog's recovery actions.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(crate) struct CloseRecovery {
    pub review: bool,
    pub connect: bool,
}

/// The recovery actions that the app-close and run-close dialogs offer above their action line.
/// Both dialogs use this one function, so the actions have one order and one place: quiet,
/// on one line, `Connect to sync` before `Review conflict`, and the first action starts at the
/// dialog text.
pub(crate) fn close_recovery_actions(
    ui: &mut egui::Ui,
    review: bool,
    connect: bool,
) -> CloseRecovery {
    let mut chosen = CloseRecovery::default();
    if !review && !connect {
        return chosen;
    }
    ui.horizontal(|ui| {
        let mut first = true;
        let mut action = |ui: &mut egui::Ui, label: &str| {
            let action = Action::quiet(label);
            let action = if std::mem::take(&mut first) {
                action.line_start()
            } else {
                action
            };
            action.show(ui).clicked()
        };
        if connect {
            chosen.connect = action(ui, "Connect to sync");
        }
        if review {
            chosen.review = action(ui, "Review conflict");
        }
    });
    chosen
}

fn close_saved_words(undo: bool, sync: bool) -> String {
    match (undo, sync) {
        (false, false) => {
            "The run stays saved. Resume it from Paused run on the left, or from Encounters."
        }
        (true, false) => "The run stays saved. Closing ends Undo for this run.",
        (false, true) => "The run stays saved. Sync stops.",
        (true, true) => "The run stays saved. Closing ends Undo for this run. Sync stops.",
    }
    .to_owned()
}

/// Cut only the target, keeping action words, amounts and copy numbers.
/// `from_sheet`: a sheet made this step, so its source can go before its target.
fn undo_words(
    ui: &egui::Ui,
    label: &str,
    from_sheet: bool,
    participants: &[Participant],
    font: &egui::FontId,
    width: f32,
) -> String {
    let mut shortened = label.to_owned();
    for direction in [" on ", " off "] {
        if let Some((effect, target)) = label
            .strip_prefix("Undo: ")
            .and_then(|s| s.rsplit_once(direction))
            && effect.split_whitespace().count() > 4
            && participants.iter().any(|p| p.name == target)
        {
            let effect = effect
                .split_whitespace()
                .take(4)
                .collect::<Vec<_>>()
                .join(" ");
            shortened = format!("Undo: {effect}…{direction}{target}");
            break;
        }
    }
    let label = shortened.as_str();
    if design::number_width(ui, label, font) <= width {
        return label.to_owned();
    }
    let compact_effect = [" on ", " off "].into_iter().find_map(|direction| {
        let (effect, target) = label.strip_prefix("Undo: ")?.rsplit_once(direction)?;
        if !effect.ends_with('…') || !participants.iter().any(|p| p.name == target) {
            return None;
        }
        let effect = effect
            .split_whitespace()
            .take(2)
            .collect::<Vec<_>>()
            .join(" ");
        let verb = if direction == " on " { "add" } else { "remove" };
        Some(format!(
            "Undo {verb}: {}…{direction}{target}",
            effect.trim_end_matches('…')
        ))
    });
    // The button always says Undo: the target is shortened, never the verb.
    let (prefix, action) = match &compact_effect {
        Some(compact) => ("", compact.as_str()),
        None => label
            .strip_prefix("Undo: ")
            .map_or(("", label), |action| ("Undo: ", action)),
    };
    let width = width - design::number_width(ui, prefix, font);
    let shown = shortened_action(ui, action, participants, font, width);
    // A sheet's step gives up its source before its target: `HP for Brannoch…`.
    if from_sheet
        && (design::number_width(ui, &shown, font) > width || !keeps_names(&shown))
        && let Some(rest) = action.strip_prefix(SHEET_SOURCE)
    {
        let without = shortened_action(ui, rest, participants, font, width);
        if design::number_width(ui, &without, font) <= width && keeps_names(&without) {
            return format!("{prefix}{without}");
        }
    }
    format!("{prefix}{shown}")
}

/// False when shortening left a name as only `…` and its copy number (`move … 2 before`):
/// then the target cannot fit, and Undo shows its word alone.
fn keeps_names(words: &str) -> bool {
    let mut parts = words.split("… ");
    let first = parts.next().unwrap_or_default();
    words == first || !(first.is_empty() || first.ends_with(' '))
}

/// An action's words shortened to `width`, its target name first.
fn shortened_action(
    ui: &egui::Ui,
    action: &str,
    participants: &[Participant],
    font: &egui::FontId,
    width: f32,
) -> String {
    if design::number_width(ui, action, font) <= width {
        return action.to_owned();
    }
    if let Some((moved, target)) = action
        .strip_prefix("move ")
        .or_else(|| action.strip_prefix("Move "))
        .and_then(|words| words.split_once(" before "))
    {
        let room = width - design::number_width(ui, "move  before ", font);
        let target_room = design::number_width(ui, target, font).min(room / 2.0);
        let moved = dock_name(ui, moved, font, room - target_room);
        let target = dock_name(
            ui,
            target,
            font,
            room - design::number_width(ui, &moved, font),
        );
        let verb = if action.starts_with("Move ") {
            "Move"
        } else {
            "move"
        };
        return format!("{verb} {moved} before {target}");
    }
    let target = participants
        .iter()
        .filter(|p| action.contains(&p.name))
        .max_by_key(|p| p.name.len());
    if let Some(target) = target {
        let (before, after) = action.split_once(&target.name).expect("target in action");
        let words: Vec<_> = target.name.split_whitespace().collect();
        let suffix = words
            .last()
            .copied()
            .filter(|last| last.chars().all(|c| c.is_ascii_digit()) || last.chars().count() == 1);
        for count in (1..words.len()).rev() {
            let name = if let Some(last) = suffix {
                format!("{}… {last}", words[..count].join(" "))
            } else {
                format!("{}…", words[..count].join(" "))
            };
            let candidate = format!("{before}{name}{after}");
            if design::number_width(ui, &candidate, font) <= width {
                return candidate;
            }
        }
        let room = width - design::number_width(ui, &format!("{before}{after}"), font);
        return format!("{before}{}{after}", dock_name(ui, &target.name, font, room));
    }
    action.to_owned()
}

/// Keep the character's name; give up later values before shortening a sheet notice.
fn sheet_notice_words(ui: &egui::Ui, summary: &str, font: &egui::FontId, width: f32) -> String {
    let (first, more) = summary
        .rsplit_once(" and ")
        .filter(|(_, more)| more.ends_with("sheet") || more.ends_with("sheets"))
        .map_or((summary, String::new()), |(first, more)| {
            (first, format!(" and {more}"))
        });
    let mut values: Vec<_> = first.split(", ").collect();
    loop {
        let shown = format!("{}{more}", values.join(", "));
        if design::number_width(ui, &shown, font) <= width || values.len() == 1 {
            return shown;
        }
        values.pop();
    }
}

/// Keep the copy suffix when a name needs fewer words.
/// A name in at most `NAME_LINES` lines of this width. A longer one keeps its first whole words
/// and ends in `…`, so it is never cut inside a word. Says whether it was cut.
fn name_lines(ui: &egui::Ui, name: &str, font: &egui::FontId, width: f32) -> (Arc<Galley>, bool) {
    let layout = |text: String, rows: usize| {
        let mut job =
            egui::text::LayoutJob::simple(text, font.clone(), egui::Color32::PLACEHOLDER, width);
        job.wrap.max_rows = rows;
        ui.fonts_mut(|fonts| fonts.layout_job(job))
    };
    // One layout in the usual case. A cut one says how many letters fit.
    let first = layout(name.to_owned(), NAME_LINES);
    if !first.elided {
        return (first, false);
    }
    let room: usize = first
        .rows
        .iter()
        .map(|row| row.char_count_including_newline())
        .sum();
    let mut words = Vec::new();
    let mut count = 0;
    for word in name.split_whitespace() {
        count += word.chars().count() + 1;
        if count > room {
            break;
        }
        words.push(word);
    }
    if words.is_empty() {
        words.extend(name.split_whitespace().next());
    }
    loop {
        // A name does not end on a joining word or a comma.
        while words.len() > 1
            && words
                .last()
                .is_some_and(|word| ["of", "the", "and", "de", "von"].contains(word))
        {
            words.pop();
        }
        let shown = format!("{}…", words.join(" ").trim_end_matches([',', ';', ':']));
        let galley = layout(shown, usize::MAX);
        if galley.rows.len() <= NAME_LINES {
            return (galley, true);
        }
        if words.len() <= 1 {
            // One word longer than the lines: cut it by letters.
            let mut letters: Vec<char> = name.chars().take(room.saturating_sub(1)).collect();
            loop {
                let galley = layout(format!("{}…", String::from_iter(&letters)), usize::MAX);
                if galley.rows.len() <= NAME_LINES || letters.len() <= 1 {
                    return (galley, true);
                }
                letters.pop();
            }
        }
        words.pop();
    }
}

pub(super) fn dock_name(ui: &egui::Ui, name: &str, font: &egui::FontId, width: f32) -> String {
    let fitted = design::words_that_fit(ui, name, font, width);
    if fitted == name {
        return fitted;
    }
    let fitted = fitted.trim_end_matches('…');
    let last = name.split_whitespace().last().unwrap_or(name);
    if !last.chars().all(|c| c.is_ascii_digit()) && last.chars().count() != 1 {
        let mut words: Vec<_> = fitted.split_whitespace().collect();
        while words
            .last()
            .is_some_and(|word| ["of", "the", "de", "von"].contains(word))
        {
            words.pop();
        }
        while words.len() > 1
            && design::number_width(ui, &format!("{}…", words.join(" ")), font) > width
        {
            words.pop();
        }
        return format!("{}…", words.join(" "));
    }
    let prefix_room = width - design::number_width(ui, &format!("… {last}"), font);
    let prefix = design::words_that_fit(ui, name, font, prefix_room);
    let prefix = prefix.trim_end_matches('…');
    let candidate = format!("{prefix}… {last}");
    if design::number_width(ui, &candidate, font) <= width {
        candidate
    } else {
        format!("… {last}")
    }
}

fn presence_action(present: bool) -> (&'static str, &'static str) {
    if present {
        (
            "Absent",
            "Skip this participant’s turns until marked present",
        )
    } else {
        ("Absent", "Give this participant turns again")
    }
}

fn hp_word(p: &Participant) -> Option<(&'static str, &'static str, egui::Color32)> {
    design::hp_word(p.hp, p.max_hp)
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
/// The notice's `Log` link. The underline covers only the word, and the word starts where the
/// item gap puts it. The 24 px target is the link's own minimum size: a word narrower than that
/// gets the extra width evenly on both sides, inside the gaps, so nothing beside it moves.
fn log_link(ui: &mut egui::Ui) -> egui::Response {
    const WORD: &str = "Log";
    let c = colors();
    let galley = ui
        .painter()
        .layout_no_wrap(WORD.to_owned(), Type::Small.font(ui), c.accent);
    let (id, word) = ui.allocate_space(vec2(galley.size().x, galley.size().y.max(design::XL)));
    let target = word.expand2(vec2(((design::XL - word.width()) / 2.0).max(0.0), 0.0));
    let response = ui.interact(target, id, egui::Sense::click());
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Link, ui.is_enabled(), WORD));
    if ui.is_rect_visible(target) {
        let top = word.center().y - galley.size().y / 2.0;
        let color = if response.hovered() {
            c.focus
        } else {
            c.accent
        };
        ui.painter().hline(
            word.x_range(),
            top + galley.size().y - 1.0,
            egui::Stroke::new(design::HAIRLINE_WIDTH, color.gamma_multiply(0.6)),
        );
        ui.painter().galley(pos2(word.left(), top), galley, color);
        if response.has_focus() {
            design::focus_ring(ui, target);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

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
    if p.named_stat_block() {
        parts.push(format!("as {}", p.definition.name));
    }
    if let Some(changes) = p.changes.as_ref().filter(|_| p.stat_changes()) {
        parts.push("Changed".into());
        parts.extend(changes.overrides.table_bonuses(&changes.base));
    }
    if p.loot_ready() {
        parts.push("Loot".into());
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
    /// After name-only rows shrink, keep the first table bonus and a link to the rest.
    Summary,
}

#[derive(Clone, Copy, PartialEq)]
enum BonusFit {
    All,
    One,
    None,
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
    /// The name needed more than `NAME_LINES`, so its last line ends in `…`.
    name_cut: bool,
    second_line: Option<Arc<Galley>>,
    /// Where each condition is on the second line, in characters.
    condition_spans: Vec<(String, std::ops::Range<usize>)>,
    initiative_span: Option<std::ops::Range<usize>>,
    /// The row has no room for a state column. Its state and sync words open the second
    /// line, so that the name stays whole.
    inline_state: bool,
    /// A name-only row in a list that needs the height.
    tight: bool,
    /// The row's roll words have no room after its second line, so they take a line of
    /// their own under it, and the row grows by that line.
    badge_below: bool,
    fit: RowFit,
    bar_inset: f32,
    bar_below_detail: bool,
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
    /// The row's roll words, if any, decide whether they need a line of their own.
    fn with_badge(mut self, ui: &egui::Ui, badge: Option<&str>) -> Self {
        self.badge_below = false;
        if let Some(badge) = badge {
            let (_, _, room) = badge_place(&self, 0.0);
            self.badge_below = badge_words(ui, badge, &Type::Small.font(ui), room).is_none();
        }
        self
    }

    /// The second line and, when they have their own, the roll words' line under it.
    fn second_block(&self, scale: RunScale) -> f32 {
        let second = self
            .second_line
            .as_ref()
            .map_or(scale.condition + 2.0, |line| line.size().y);
        second
            + if self.badge_below {
                Type::Small.line()
            } else {
                0.0
            }
    }

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
        Self::measured(
            ui,
            width,
            p,
            sync,
            scale,
            (fit, p.conditions.len(), BonusFit::All),
            longest_name,
        )
    }

    fn measured(
        ui: &egui::Ui,
        width: f32,
        p: &Participant,
        sync: Option<SyncState>,
        scale: RunScale,
        fit_and_count: (RowFit, usize, BonusFit),
        longest_name: f32,
    ) -> Self {
        let (fit, condition_count, bonus_fit) = fit_and_count;
        let c = colors();
        // Fixed columns from the left of the numbers. A name stays near its HP.
        // The smallest scale has one set of columns, compressed or not, so that a list that
        // starts to compress does not move its names sideways.
        let narrow = fit != RowFit::Natural || scale.row <= design::RUN_COMPACT.row;
        // The current-row bar, the gap after it, and room for the word `TURN` at this size. In
        // narrow columns the word can sit on the number's line, so that a row's height never
        // depends on whose turn it is.
        let word = scale.mark * 3.2;
        let initiative_right = 6.0 + design::XS + if narrow { word + scale.stat } else { word };
        let gap = if narrow { 6.0 } else { 12.0 };
        // Its bar starts later, still under the digits, so its second line has the width that
        // the turn word took and a condition does not wrap again.
        let mut bar_inset = if narrow { 8.0 } else { 0.0 };
        // Compressed rows have a roster to fit, so the picture gives up what it can.
        let picture = if narrow {
            scale.name * 1.25
        } else {
            (scale.row - 10.0).min(scale.name * 1.75)
        };
        let name_left = initiative_right + gap + scale.name * 0.7 + picture + PICTURE_GAP;
        let hp_width = scale.hp * 1.9 + scale.hp_max * 2.0 + 4.0;
        let state_font = Type::Heading.sized(ui, scale.mark.max(12.0));
        let state_width = [
            SyncState::FirstRead,
            SyncState::ReadOnly,
            SyncState::Pending,
            SyncState::Stopped,
        ]
        .into_iter()
        .map(|state| {
            let status = sync_status(state);
            design::number_width(ui, &format!("{} {}", status.icon, status.word), &state_font)
                + design::S
        })
        .fold((scale.mark * 8.5).clamp(84.0, 118.0), f32::max);
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
        // Every scale keeps it aligned with the name.
        let second_left = name_left;
        job.wrap.max_width = (temp_right + bar_inset - second_left).max(1.0);
        // With HP in what a DM looks for: as strong as the state words, and stronger than AC.
        // The compact scale keeps body weight, so a wrapped line costs no row its place.
        let role = if narrow { Type::Body } else { Type::Strong };
        let font = role.sized(ui, scale.condition);
        let mut first = true;
        let mut add = |text: &str, color: egui::Color32| {
            if !first {
                job.append(
                    " ·\u{a0}",
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
        if inline_state && let Some((icon, word, color)) = hp_word(p) {
            add(&format!("{icon} {word}"), color);
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
        let mut condition_spans: Vec<_> = p
            .conditions
            .iter()
            .take(condition_count)
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
        if p.conditions.len() > condition_count {
            condition_spans.push((
                "Run conditions".into(),
                add(
                    &design::count(p.conditions.len() - condition_count, "more", "more")
                        .replace(' ', "\u{a0}"),
                    c.participant.condition,
                ),
            ));
        }
        if !["party", "enemies"].contains(&p.side.as_str()) {
            add(&style::human_label(&p.side), c.side.other);
        }
        // A change is never hidden. What the run cannot apply is on the row, for the roll.
        // A number stays with its word when the line wraps.
        if p.npc_identity() {
            add(&format!("as {}", p.definition.name), c.muted);
        }
        let mut bonus_start = None;
        if let Some(changes) = p.changes.as_ref().filter(|_| p.stat_changes()) {
            add(
                &format!("{}\u{a0}Changed", icons::CHANGED),
                c.participant.changed,
            );
            let bonuses = changes.overrides.table_bonuses(&changes.base);
            for bonus in bonuses.iter().take(match bonus_fit {
                BonusFit::None => 0,
                BonusFit::One => 1,
                BonusFit::All => bonuses.len(),
            }) {
                let span = add(&bonus.replace(' ', "\u{a0}"), c.participant.changed);
                bonus_start.get_or_insert(span.start);
            }
            if !bonuses.is_empty()
                && (bonus_fit == BonusFit::None
                    || (bonus_fit == BonusFit::One && bonuses.len() > 1))
            {
                condition_spans.push(("Run changes".into(), add("more", c.participant.changed)));
            }
        }
        // Down, with something to take: the DM rolls it from the card or the ended view.
        if p.loot_ready() {
            add(&format!("{}\u{a0}Loot", icons::LOOT), c.accent);
        }
        if inline_state && let Some(sync) = sync {
            let status = sync_status(sync);
            // Alone on the line, the state starts at the line's edge, like Bloodied.
            let alone = job.is_empty();
            if !alone {
                job.append(
                    " ",
                    0.0,
                    egui::TextFormat {
                        font_id: font.clone(),
                        ..Default::default()
                    },
                );
            }
            job.append(
                &format!("{} {}", status.icon, status.word).replace(' ', "\u{a0}"),
                if alone { 0.0 } else { design::XS },
                egui::TextFormat {
                    font_id: Type::Small.sized(ui, scale.mark),
                    color: status.color,
                    underline: if sync == SyncState::Conflict {
                        Stroke::new(design::HAIRLINE_WIDTH, status.color)
                    } else {
                        Stroke::NONE
                    },
                    ..Default::default()
                },
            );
        }
        if let Some(span) = &initiative_span {
            let start = job
                .text
                .char_indices()
                .nth(span.start)
                .map_or(job.text.len(), |(at, _)| at);
            for section in &mut job.sections {
                if section.byte_range.start == start {
                    section.format.underline = Stroke::new(design::HAIRLINE_WIDTH, c.accent);
                }
            }
        }
        let mut conditions = job.clone();
        if let Some(start) = bonus_start {
            let end = conditions
                .text
                .char_indices()
                .nth(start)
                .map_or(conditions.text.len(), |(at, _)| at);
            conditions.text.truncate(end);
            conditions.text = conditions
                .text
                .trim_end_matches([' ', '·', '\u{a0}'])
                .to_owned();
            let end = conditions.text.len();
            conditions
                .sections
                .retain(|section| section.byte_range.start < end);
            for section in &mut conditions.sections {
                section.byte_range.end = section.byte_range.end.min(end);
            }
        }
        let conditions_wrap = ui
            .fonts_mut(|fonts| fonts.layout_job(conditions))
            .rows
            .len()
            > 1;
        let mut second_line =
            (!job.is_empty()).then(|| ui.fonts_mut(|fonts| fonts.layout_job(job.clone())));
        if bonus_fit != BonusFit::None
            && p.stat_changes()
            && second_line.as_ref().is_some_and(|line| line.rows.len() > 1)
        {
            return Self::measured(
                ui,
                width,
                p,
                sync,
                scale,
                (
                    fit,
                    condition_count,
                    if bonus_fit == BonusFit::All {
                        BonusFit::One
                    } else {
                        BonusFit::None
                    },
                ),
                longest_name,
            );
        }
        if condition_count > 1
            && (conditions_wrap || second_line.as_ref().is_some_and(|line| line.rows.len() > 1))
        {
            return Self::measured(
                ui,
                width,
                p,
                sync,
                scale,
                (fit, condition_count - 1, bonus_fit),
                longest_name,
            );
        }
        let mut bar_below_detail = false;
        // At the narrowest width, mandatory state words can need the unused left
        // part of the HP bar. Keep the bar under HP, with at least an XL of length.
        if second_line.as_ref().is_some_and(|line| line.rows.len() > 1) {
            let old_width = job.wrap.max_width;
            let mut unwrapped = job.clone();
            unwrapped.wrap.max_width = f32::INFINITY;
            let needed = ui.fonts_mut(|fonts| fonts.layout_job(unwrapped)).size().x;
            let extra = (needed - old_width + design::HAIRLINE_WIDTH)
                .min((hp_right - temp_right - bar_inset - design::XL - design::XS).max(0.0));
            bar_inset += extra;
            job.wrap.max_width += extra;
            second_line = Some(ui.fonts_mut(|fonts| fonts.layout_job(job.clone())));
            if second_line.as_ref().is_some_and(|line| line.rows.len() > 1) {
                for section in &mut job.sections {
                    section.format.font_id = Type::Small.sized(ui, scale.mark);
                }
                second_line = Some(ui.fonts_mut(|fonts| fonts.layout_job(job.clone())));
                if second_line.as_ref().is_some_and(|line| line.rows.len() > 1) {
                    bar_below_detail = true;
                    job.wrap.max_width = (width - second_left - design::XS).max(1.0);
                    second_line = Some(ui.fonts_mut(|fonts| fonts.layout_job(job)));
                }
            }
        }
        let mut name = ui.fonts_mut(|fonts| {
            fonts.layout(
                p.name.clone(),
                Type::Name.sized(ui, scale.name),
                egui::Color32::PLACEHOLDER,
                name_width,
            )
        });
        // The numeric columns never move: a long name wraps in the shared name column at the
        // compact size, and a fourth line is cut to `…`. Hover and the row's name keep it whole.
        let mut name_cut = false;
        if name.rows.len() > 2 {
            let font = Type::Name.sized(ui, design::RUN_COMPACT.name.min(scale.name));
            (name, name_cut) = name_lines(ui, &p.name, &font, name_width);
        }
        let tight = fit != RowFit::Natural
            && second_line.is_none()
            && sync.is_none()
            && name.rows.len() == 1;
        Self {
            art: None,
            picture,
            second_left,
            name,
            name_cut,
            even: 0.0,
            bar_inset,
            bar_below_detail,
            second_line,
            condition_spans,
            initiative_span,
            inline_state,
            tight,
            badge_below: false,
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
                RowFit::Compressed | RowFit::Summary => 2.0,
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
        let second = self.second_block(scale);
        let name_line = self.name_line(scale);
        if self.fit != RowFit::Natural {
            return (name_line + self.name_extra() + second + design::HAIRLINE_WIDTH)
                .max(self.even)
                + if self.bar_below_detail {
                    design::S
                } else {
                    0.0
                };
        }
        let top = ((scale.row - name_line - second_height) / 2.0).max(1.0);
        scale
            .row
            .max(top + name_line + self.name_extra() + second + 3.0)
            + if self.bar_below_detail {
                design::S
            } else {
                0.0
            }
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
        if self.fit != RowFit::Natural && self.second_line.is_some() {
            let content = name_line + self.name_extra() + self.second_block(scale);
            return ((height - content) / 2.0).max(design::HAIRLINE_WIDTH / 2.0) + name_line / 2.0;
        }
        // A row with one line centres it until there is room for two. Then every row
        // shares one place, with or without a second line, so adding one moves no number.
        let basis = match self.fit {
            RowFit::Natural => scale.row,
            RowFit::Compressed | RowFit::Summary => self
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
        let changes = condition == "Run changes";
        let conditions = condition == "Run conditions";
        let custom = changes || conditions || condition.split_whitespace().count() > 4;
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
        let bottom = (word.center().y + design::XL / 2.0).min(rect.bottom());
        let width = word.width().max(design::XL);
        let left = (word.center().x - width / 2.0)
            .max(rect.left())
            .min(rect.right() - width);
        let target =
            Rect::from_min_max(pos2(left, bottom - design::XL), pos2(left + width, bottom));
        let id = ui.id().with(("condition_rule", key, position));
        let response = ui
            .interact(target, id, egui::Sense::click())
            .on_hover_text(format!("Read the rule for {condition}"))
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        let name = if conditions {
            "Read all conditions".to_owned()
        } else if changes {
            "Read all changes".to_owned()
        } else if custom {
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
            opened = Some(if conditions {
                usize::MAX - 2
            } else if changes {
                usize::MAX - 1
            } else {
                entry.unwrap_or(usize::MAX)
            });
        }
    }
    opened
}

/// Where a row's roll words go: their left edge, their top below the second line's top, and
/// their room. After the last line of the row's second line, or at its start when it has none;
/// they end before the HP column, as the second line does.
fn badge_place(layout: &RowLayout, x: f32) -> (f32, f32, f32) {
    if layout.badge_below {
        let left = x + layout.second_left;
        let top = layout
            .second_line
            .as_ref()
            .map_or(0.0, |line| line.size().y);
        return (left, top, x + layout.temp_right - left - design::S);
    }
    let (after, top) = layout
        .second_line
        .as_ref()
        .and_then(|line| line.rows.last())
        .map_or((0.0, 0.0), |row| {
            (row.rect().right() + design::S, row.rect().top())
        });
    let left = x + layout.second_left + after;
    (left, top, x + layout.temp_right - left - design::S)
}

/// The words of `badge` that fit `room`: the longest form that fits whole, dropping its first
/// parts but never its last two, or else its last form cut by whole words. `None` when not
/// one word fits.
fn badge_words(ui: &egui::Ui, badge: &str, font: &egui::FontId, room: f32) -> Option<String> {
    let parts: Vec<&str> = badge.split(" · ").collect();
    let kept = parts.len().min(2);
    let forms: Vec<String> = (0..=parts.len() - kept)
        .map(|from| parts[from..].join(" · "))
        .collect();
    if let Some(words) = forms
        .iter()
        .find(|words| design::number_width(ui, words, font) <= room)
    {
        return Some(words.clone());
    }
    // `words_that_fit` keeps a first word even where it does not fit.
    let words = design::words_that_fit(ui, forms.last()?, font, room);
    (!words.is_empty() && design::number_width(ui, &words, font) <= room).then_some(words)
}

/// `save` is a save's result for this row, such as `DEX` and 19, shown where AC is, and
/// `badge` what the current action says about the row, such as `60% to hit`.
#[allow(clippy::too_many_arguments)]
fn paint_row(
    ui: &egui::Ui,
    rect: Rect,
    p: &Participant,
    turn: TurnMark,
    sync: Option<SyncState>,
    scale: RunScale,
    layout: &RowLayout,
    save: Option<&(String, i64)>,
    badge: Option<&str>,
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
            RowFit::Compressed | RowFit::Summary => pos2(initiative_left - design::XS, line_y),
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
                if layout.name.rows.len() > 1 || layout.second_left < layout.name_left {
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
    // During a call for a save, a target's save stands where its AC is: AC does not
    // matter to a save, and the result sits beside the HP it will change.
    let ac_right = x + layout.ac_right;
    let (value, label, strong) = match save {
        Some((ability, total)) => (total.to_string(), ability.as_str(), true),
        None => (display(p.armor_class), "AC", false),
    };
    let left = design::number(
        painter,
        ac_right,
        line_y,
        &value,
        &numeral,
        if strong || !p.present { text } else { c.muted },
    );
    painter.text(
        pos2(left - 4.0, line_y + 1.0),
        Align2::RIGHT_CENTER,
        label,
        Type::Small.font(ui),
        if current || strong { c.muted } else { c.faint },
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
    let bar_top = if layout.bar_below_detail {
        rect.bottom() - bar_height - design::HAIRLINE_WIDTH
    } else {
        (line_y + scale.hp * 0.5 + 3.0).min(rect.bottom() - bar_height - 1.0)
    };
    let bar = Rect::from_min_max(
        pos2(x + layout.temp_right + layout.bar_inset + 4.0, bar_top),
        pos2(hp_right, bar_top + bar_height),
    );
    design::hp_bar(painter, bar, p.hp, p.max_hp, p.temporary_hp);
    // The current action's words on this row: the chance to hit, or the result. They have one
    // place on every row and at every size: the line under the name, after its conditions
    // (and after the state word where the row has no state column). The longest form that fits
    // whole, dropping its first parts; the result itself (its last two parts: an attack's
    // comparison and damage, a save's result and damage, a chance and its mode) never drops,
    // and is cut by whole words only where the line has no room for it. The row's accessible
    // name has the whole text.
    if let Some(badge) = badge {
        let font = Type::Small.font(ui);
        let (left, top, room) = badge_place(layout, x);
        if let Some(words) = badge_words(ui, badge, &font, room) {
            painter.text(
                pos2(left, second_y + top),
                Align2::LEFT_TOP,
                words,
                font,
                c.accent,
            );
        }
    }
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
        let text = Type::Heading
            .rich(ui, format!("{} {}", status.icon, status.word))
            .font(state_font.clone())
            .color(status.color);
        let text = if sync == SyncState::Conflict {
            text.underline()
        } else {
            text
        };
        let galley = egui::WidgetText::from(text).into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            Type::Heading.font(ui),
        );
        let at = pos2(
            hp_right + design::S,
            if hp_word(p).is_some() {
                second_y + second_height / 2.0
            } else {
                line_y
            },
        );
        painter.galley(
            pos2(at.x, at.y - galley.size().y / 2.0),
            galley,
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
fn correction_label(ui: &mut egui::Ui, label: &str) {
    let size = vec2(design::L * 6.0, Density::Read.control());
    ui.allocate_ui_with_layout(size, egui::Layout::left_to_right(Align::Center), |ui| {
        ui.set_min_size(size);
        ui.label(Type::Body.rich(ui, label).color(colors().text));
    });
}

fn signed_bonus(value: i64) -> String {
    if value < 0 {
        format!("−{}", value.unsigned_abs())
    } else {
        format!("+{value}")
    }
}

fn parse_bonus(text: &str) -> Option<f64> {
    text.trim().replace('−', "-").parse().ok()
}

fn optional_number(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<i64>,
    range: std::ops::RangeInclusive<i64>,
) {
    // Its words stay on one line, in any width of panel.
    ui.horizontal(|ui| {
        correction_label(ui, label);
        if let Some(n) = value {
            let mut field = egui::DragValue::new(n).range(range);
            if label == "Initiative bonus" {
                field = field
                    .custom_formatter(|value, _| signed_bonus(value as i64))
                    .custom_parser(parse_bonus);
            }
            let response = ui.add(field);
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

#[cfg(test)]
mod tests {
    #[test]
    fn iter8_b_setup_places_show_while_typing_and_fields_hold_still() {
        use egui_kittest::kittest::Queryable;
        for width in [450.0, 960.0, 1920.0] {
            let mut run = super::super::tests::run();
            run.initiative.roster_ties = false;
            // The guards are identical NPCs. Individual rolls give them no shared roll, so no
            // tie slot is reserved and this test sees a typed tie wait for blur.
            run.initiative.grouping = crate::encounter_settings::Grouping::Individual;
            for (index, total) in [Some(20), Some(15), None].into_iter().enumerate() {
                run.state.participants[index].initiative = total;
                run.state.participants[index].conditions.clear();
            }
            let run = std::rc::Rc::new(std::cell::RefCell::new(run));
            let shown = run.clone();
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 640.0))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let mut run = shown.borrow_mut();
                        for i in 0..3 {
                            let picked = ui.push_id(i, |ui| run.setup_row(ui, i)).inner;
                            if let Some(command) = picked {
                                run.apply(command).unwrap();
                            }
                        }
                    });
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            let label = "Initiative for Guard 3";
            harness.get_by_label(label).click();
            harness.run_steps(2);
            let before = harness.get_by_label(label).rect().min;
            harness.event(egui::Event::Text("10".into()));
            harness.run_steps(2);
            assert_eq!(run.borrow().state.participants[2].initiative, Some(10));
            assert!(harness.ctx.memory(|m| m.has_focus(design::focus_id(label))));
            assert!(
                harness
                    .query(
                        egui_kittest::kittest::By::new().predicate(|node| node.label().as_deref()
                            == Some("Place for Guard 3: 3rd")
                            && node.value().as_deref() == Some("3rd"))
                    )
                    .is_some(),
                "iter8-B-FT2: the last focused total shows its place at {width}"
            );
            assert_eq!(
                harness.get_by_label(label).rect().min,
                before,
                "iter8-B-FT1: last total keeps x and y"
            );
            // Replace 10 by 15, tied with Guard 2, without leaving the field.
            harness.key_press(Key::Backspace);
            harness.event(egui::Event::Text("5".into()));
            harness.run_steps(2);
            assert_eq!(run.borrow().state.participants[2].initiative, Some(15));
            assert_eq!(
                harness.get_by_label(label).rect().min,
                before,
                "iter8-B-FT1: a typed tie keeps x and y"
            );
            assert!(
                harness
                    .query_by_label("Move Guard 3 before Guard 2")
                    .is_none(),
                "iter8-B-tie: the new tie slot waits for blur"
            );
            harness.key_press(Key::Backspace);
            harness.run_steps(2);
            assert_eq!(run.borrow().state.participants[2].initiative, Some(1));
            assert_eq!(
                harness.get_by_label(label).rect().min,
                before,
                "iter8-B-FT1: backspace keeps x and y"
            );
            harness
                .ctx
                .memory_mut(|m| m.surrender_focus(design::focus_id(label)));
            harness.run_steps(2);
            assert_eq!(
                harness.get_by_label(label).rect().min,
                before,
                "iter8-B-FT1: leaving an untied field keeps x and y"
            );
            for name in ["Guard 1", "Guard 2", "Guard 3"] {
                let field = harness
                    .get_by_label(&format!("Initiative for {name}"))
                    .rect();
                let absent = harness.get_by_label(&format!("Absent: {name}")).rect();
                assert!(
                    absent.left() - field.right() >= design::S,
                    "iter8-B-FT1: S gap before Absent for {name}"
                );
            }
            // A tie that is kept does expose its ordering actions after blur.
            harness.get_by_label(label).click();
            harness.event(egui::Event::Text("5".into()));
            harness.run_steps(2);
            assert_eq!(run.borrow().state.participants[2].initiative, Some(15));
            // Tab to the next control is a blur of the field too.
            harness.key_press(Key::Tab);
            harness.run_steps(2);
            assert!(
                !harness.ctx.memory(|m| m.has_focus(design::focus_id(label))),
                "iter8-B-tie: Tab leaves the field"
            );
            assert!(
                harness
                    .query_by_label("Move Guard 3 before Guard 2")
                    .is_some(),
                "iter8-B-tie: a kept tie shows its controls after Tab"
            );
            harness.ctx.memory_mut(|m| {
                if let Some(id) = m.focused() {
                    m.surrender_focus(id);
                }
            });
            harness.run_steps(2);
            assert!(
                harness
                    .query_by_label("Move Guard 3 before Guard 2")
                    .is_some(),
                "iter8-B-tie: a kept tie shows its controls on blur"
            );
            // The tie slot opens on blur, once, after the DM has left the field: the row
            // keeps its line, and the field moves left by the slot only then.
            assert_eq!(
                harness.get_by_label(label).rect().min.y,
                before.y,
                "iter8-B-tie: a kept tie keeps the row on its line after blur"
            );
            assert_eq!(
                harness.get_by_label(label).rect().min.x,
                before.x - Density::Read.control() * 2.0,
                "iter18-B-FT3: a typed tie opens its slot only after blur"
            );
        }
    }

    /// FT3: a grouped `Roll NPCs` moves no field, because its tie slot is there from the
    /// start. With no group to roll, or individual rolls, setup shows no empty tie slot.
    #[test]
    fn iter18_b_grouped_roll_reserves_the_tie_slot_from_the_start() {
        use crate::encounter_settings::Grouping;
        use egui_kittest::kittest::Queryable;
        let slot = Density::Read.control() * 2.0;
        let lefts = |run: super::Run| {
            let run = std::rc::Rc::new(std::cell::RefCell::new(run));
            let shown = run.clone();
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(1200.0, 640.0))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let mut run = shown.borrow_mut();
                        for i in 0..run.state.participants.len() {
                            ui.push_id(i, |ui| run.setup_row(ui, i));
                        }
                    });
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            let read = |harness: &egui_kittest::Harness<'_>| {
                ["Guard 1", "Guard 2", "Guard 3"].map(|name| {
                    harness
                        .get_by_label(&format!("Initiative for {name}"))
                        .rect()
                        .left()
                })
            };
            let before = read(&harness);
            let grouping = run.borrow().initiative.grouping;
            run.borrow_mut().state.random = splat::execution::dice::Random::controlled([7, 9, 11]);
            run.borrow_mut().apply(Command::RollNpcs(grouping)).unwrap();
            harness.run_steps(2);
            let arrows = harness
                .query_all(egui_kittest::kittest::By::new().predicate(|node| {
                    node.label()
                        .is_some_and(|label| label.starts_with("Move Guard"))
                }))
                .count();
            (before, read(&harness), arrows)
        };
        let fresh = |grouping: Grouping, distinct: bool| {
            let mut run = super::super::tests::run();
            run.initiative.roster_ties = false;
            run.initiative.grouping = grouping;
            for p in &mut run.state.participants {
                p.initiative = None;
                p.conditions.clear();
            }
            // Guard 1 is a player, so the roll leaves its field empty.
            run.state.participants[0].npc = false;
            if distinct {
                run.state.participants[2].initiative_bonus = Some(5);
            }
            run
        };
        // Two identical NPCs share the default roll: the slot is there before it.
        let (grouped, rolled, arrows) = lefts(fresh(Grouping::Identical, false));
        assert!(arrows > 0, "iter18-B-FT3: the shared roll shows its arrows");
        assert_eq!(grouped, rolled, "iter18-B-FT3: Roll NPCs moves no field");
        // No group to roll, or individual rolls: no empty slot, before or after.
        for (case, grouping, distinct) in [
            ("no group", Grouping::Identical, true),
            ("individual rolls", Grouping::Individual, false),
        ] {
            let (before, after, arrows) = lefts(fresh(grouping, distinct));
            assert_eq!(arrows, 0, "iter18-B-FT3: {case} has no tie");
            for at in 0..3 {
                assert_eq!(
                    before[at] - grouped[at],
                    slot,
                    "iter18-B-FT3: {case} reserves no tie slot (row {at})"
                );
                assert_eq!(
                    before[at], after[at],
                    "iter18-B-FT3: {case} roll moves no field"
                );
            }
        }
    }

    #[test]
    fn iter8_b_arrival_fields_share_a_column_for_add_and_add_ten() {
        use egui_kittest::kittest::Queryable;
        for width in [440.0, 760.0] {
            let mut run = super::super::tests::run();
            let groups: Vec<_> = [("Scouts", false), ("Lost Patrol", true)]
                .into_iter()
                .map(|(name, partial)| {
                    Arc::new(super::super::RunGroup {
                        id: Uuid::new_v4(),
                        path: format!("{name}.md"),
                        name: name.into(),
                        side: "enemies".into(),
                        encounter: None,
                        encounter_name: None,
                        members: vec![super::super::GroupMember {
                            entry: run.state.participants[0].definition.clone(),
                            count: 10,
                            overrides: Default::default(),
                        }],
                        notes: String::new(),
                        xp: 0,
                        xp_unknown: vec![],
                        unavailable: 0,
                        unavailable_members: vec![],
                        problem: None,
                        warnings: if partial {
                            vec!["One unread member".into()]
                        } else {
                            vec![]
                        },
                    })
                })
                .collect();
            run.groups = Some(Arc::new(groups));
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 800.0))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        run.group_list(ui);
                    });
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            let fields: Vec<_> = ["Scouts", "Lost Patrol"]
                .map(|name| {
                    harness
                        .get_by_label(&format!("Initiative total for {name}"))
                        .rect()
                })
                .to_vec();
            let buttons = ["Scouts", "Lost Patrol"].map(|name| {
                harness
                    .get_by_label(&format!("Add {name} to the initiative list"))
                    .rect()
            });
            assert!(
                (buttons[0].right() - buttons[1].right()).abs() <= 1.0,
                "iter9-B-FT6-edge: {buttons:?}"
            );
            assert!(
                (buttons[0].width() - buttons[1].width()).abs() <= 1.0,
                "iter9-B-FT6-width: {buttons:?}"
            );
            assert!(
                (fields[0].left() - fields[1].left()).abs() <= 1.0,
                "iter8-B-FT3: shared field column at {width}: {fields:?}"
            );
        }
    }

    #[test]
    fn iter10_b_arrival_controls_end_at_the_panel_edge_and_bad_totals_warn_once() {
        use egui_kittest::kittest::Queryable;
        let names = ["Ash Hounds", "Shrine Reserve", "Lost Patrol"];
        for width in [440.0, 600.0, 760.0] {
            let mut run = super::super::tests::run();
            let groups: Vec<_> = [
                ("Ash Hounds", 1, false),
                ("Shrine Reserve", 2, false),
                ("Lost Patrol", 10, true),
            ]
            .into_iter()
            .map(|(name, count, partial)| {
                Arc::new(super::super::RunGroup {
                    id: Uuid::new_v4(),
                    path: format!("{name}.md"),
                    name: name.into(),
                    side: "enemies".into(),
                    encounter: None,
                    encounter_name: None,
                    members: vec![super::super::GroupMember {
                        entry: run.state.participants[0].definition.clone(),
                        count,
                        overrides: Default::default(),
                    }],
                    notes: String::new(),
                    xp: 0,
                    xp_unknown: vec![],
                    unavailable: 0,
                    unavailable_members: vec![],
                    problem: None,
                    warnings: if partial {
                        vec!["One unread member".into()]
                    } else {
                        vec![]
                    },
                })
            })
            .collect();
            let bad = groups[1].id;
            run.groups = Some(Arc::new(groups));
            run.group_totals.insert(bad, "1-2".into());
            let edge = std::rc::Rc::new(std::cell::Cell::new(0.0));
            let seen = edge.clone();
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 800.0))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        seen.set(ui.max_rect().right());
                        run.group_list(ui);
                    });
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            let edge = edge.get();
            let fields = names.map(|name| {
                harness
                    .get_by_label(&format!("Initiative total for {name}"))
                    .rect()
            });
            let buttons = names.map(|name| {
                harness
                    .get_by_label(&format!("Add {name} to the initiative list"))
                    .rect()
            });
            for (button, field) in buttons.iter().zip(&fields) {
                assert!(
                    (button.right() - edge).abs() <= 0.5,
                    "iter10-B-SD5-edge at {width}: {buttons:?} vs {edge}"
                );
                assert!(
                    (field.left() - fields[0].left()).abs() <= 0.5,
                    "iter10-B-SD5-column at {width}: {fields:?}"
                );
                // Must stay true (FT-6): one Add width, so one straight Add edge.
                assert!((button.width() - buttons[0].width()).abs() <= 0.5);
            }
            // FT-4: the bad total is one warning in the group fault form: text-colour words.
            let texts: Vec<_> = harness
                .output()
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text)
                        if text.galley.text().contains("Enter a whole number") =>
                    {
                        Some(text.clone())
                    }
                    _ => None,
                })
                .collect();
            assert_eq!(texts.len(), 1, "iter10-B-FT4-once at {width}");
            assert!(
                texts[0]
                    .galley
                    .job
                    .sections
                    .iter()
                    .all(|section| section.format.color == colors().text),
                "iter10-B-FT4-text-colour at {width}"
            );
            assert!(
                !texts[0].galley.text().contains(icons::WARN),
                "iter10-B-FT4-mark-apart at {width}"
            );
        }
    }

    #[test]
    fn iter8_b_condition_counts_keep_more_with_sync() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut p = super::super::tests::run().state.participants[0].clone();
                p.conditions = [
                    "Poisoned",
                    "Prone",
                    "Restrained",
                    "Blinded",
                    "Deafened",
                    "Frightened",
                ]
                .map(str::to_owned)
                .to_vec();
                for sync in [SyncState::Off, SyncState::Conflict] {
                    let layout = RowLayout::new(
                        ui,
                        442.0,
                        &p,
                        Some(sync),
                        design::RUN_COMPACT,
                        RowFit::Summary,
                        0.0,
                    );
                    let line = layout.second_line.unwrap();
                    assert!(line.text().contains("more"), "iter8-B-FT7: {}", line.text());
                    assert!(!line.text().contains('+'), "iter8-B-FT7: {}", line.text());
                    assert!(line.text().contains("Poisoned"));
                    assert!(
                        line.text()
                            .contains(sync_status(sync).word.replace(' ', "\u{a0}").as_str())
                    );
                }
            });
        });
    }

    #[test]
    fn iter8_b_sheet_notice_centres_with_next_and_log_and_has_a_gap() {
        use egui_kittest::kittest::Queryable;
        for width in [1200.0, 1920.0] {
            let mut run = super::super::tests::run();
            for (index, total) in [20, 15, 10].into_iter().enumerate() {
                run.apply(Command::Initiative(index, Some(total))).unwrap();
            }
            run.apply(Command::Begin).unwrap();
            run.notice = Some("Guard 1’s sheet changed: HP 17 → 15. Undo stops here.".into());
            run.notice_summary = Some("Guard 1: HP 17 → 15".into());
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 800.0))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        run.live_header(ui, design::RUN_REGULAR);
                    });
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            let log = harness.get_by_label("Log").rect();
            let text = |prefix: &str| {
                harness
                    .output()
                    .shapes
                    .iter()
                    .find_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) if text.galley.text().starts_with(prefix) => {
                            Some(text)
                        }
                        _ => None,
                    })
                    .unwrap_or_else(|| panic!("Required text: {prefix}"))
            };
            let next = text("Next:");
            let notice = text("Guard 1: HP");
            let next_rect = Rect::from_min_size(next.pos, next.galley.size());
            let notice_rect = Rect::from_min_size(notice.pos, notice.galley.size());
            let icon = text(icons::WARN);
            let icon_rect = Rect::from_min_size(icon.pos, icon.galley.size());
            assert!(
                (notice_rect.left() - icon_rect.right() - design::XS).abs() <= 1.0,
                "iter9-B-FT7-icon-gap: {icon_rect:?}, {notice_rect:?}"
            );
            let bar = harness
                .output()
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    egui::Shape::Rect(rect) if rect.fill == colors().result.warning => {
                        Some(rect.rect)
                    }
                    _ => None,
                })
                .expect("Required warning bar");
            assert!(
                (notice_rect.center().y - next_rect.center().y).abs() <= 1.0
                    && (notice_rect.center().y - log.center().y).abs() <= 1.0,
                "iter8-B-AT2-centre: {notice_rect:?}, {next_rect:?}, {log:?}"
            );
            assert!(
                bar.left() - next_rect.right() >= design::S,
                "iter8-B-AT2-gap: {bar:?}, {next_rect:?}"
            );
            assert!(
                (bar.center().y - notice_rect.center().y).abs() <= 1.0,
                "iter8-B-AT2-bar: {bar:?}, {notice_rect:?}"
            );
        }
    }

    #[test]
    fn iter7_b_setup_counts_partly_hidden_rows_from_scroll_content_top() {
        use egui_kittest::kittest::Queryable;
        let mut checked = 0;
        for height in (300..640).step_by(4) {
            let mut run = super::super::tests::run();
            run.state.participants[2].name = "Last guard".into();
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(600.0, height as f32))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| run.setup_body(ui));
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            let row = harness.get_by_label_contains("Last guard ·").rect();
            let Some(clip) = harness
                .output()
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.text() == "Last guard" => {
                        Some(shape.clip_rect)
                    }
                    _ => None,
                })
            else {
                continue;
            };
            if row.top() < clip.bottom() && row.bottom() > clip.bottom() + 1.0 {
                checked += 1;
                assert!(harness.output().shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.text().contains("below"))),
                    "iter7-B-AD1: partly clipped row must count at height {height}: row {row:?}, clip {clip:?}");
            }
        }
        assert!(checked > 0, "The probe must exercise a partly clipped row");
    }

    #[test]
    fn iter7_b_death_saves_follow_party_and_recorded_counts() {
        use egui_kittest::kittest::Queryable;
        for (side, linked, successes, failures, shown) in [
            ("party", false, 0, 0, true),
            ("enemies", false, 0, 0, false),
            ("enemies", false, 1, 0, true),
            ("other", false, 0, 1, true),
            // A linked player sheet on a side the DM named makes death saves too.
            ("heroes", true, 0, 0, true),
        ] {
            let mut run = super::super::tests::run();
            run.selected = 0;
            run.panel = Panel::Reference;
            let p = &mut run.state.participants[0];
            if linked {
                let mut definition = (*p.definition).clone();
                definition
                    .fields
                    .insert("splat_external".into(), "dndbeyond:1".into());
                p.definition = std::sync::Arc::new(definition);
            }
            p.side = side.into();
            p.hp = Some(0);
            p.successes = successes;
            p.failures = failures;
            let mut harness = egui_kittest::Harness::new(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| run.side_panel(ui));
            });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            assert_eq!(
                harness.query_by_label("Add a death save success").is_some(),
                shown,
                "iter7-B-FT1: {side}, {successes} successes, {failures} failures"
            );
        }
    }

    #[test]
    fn iter7_b_setup_names_fit_and_initiative_fields_align() {
        use egui_kittest::kittest::Queryable;
        let mut run = super::super::tests::run();
        run.state.participants.truncate(2);
        run.state.order = vec![0, 1];
        for (p, name, npc) in run
            .state
            .participants
            .iter_mut()
            .zip([("Kestrel Dunmoor", false), ("Ember Cultist", true)])
            .map(|(p, (name, npc))| (p, name, npc))
        {
            p.name = name.into();
            p.npc = npc;
            p.initiative = None;
        }
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(450.0, 640.0))
            .build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    for i in 0..2 {
                        ui.push_id(i, |ui| run.setup_row(ui, i));
                    }
                });
            });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        let a = harness
            .get_by_label("Initiative for Kestrel Dunmoor")
            .rect();
        let b = harness.get_by_label("Initiative for Ember Cultist").rect();
        assert!(
            a.left() > 240.0,
            "iter7-B-FD1: unused slots must return room to the name: {a:?}"
        );
        assert!(
            (a.left() - b.left()).abs() < 1.0,
            "iter7-B-FD1 aligned fields: {a:?}, {b:?}"
        );
        let names: Vec<_> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.text() == "Kestrel Dunmoor" => Some(text),
                _ => None,
            })
            .collect();
        assert!(!names.is_empty());
        assert!(
            names.iter().all(|text| text.galley.rows.len() == 1),
            "iter7-B-FD1: party names fit one line"
        );
    }

    #[test]
    fn iter6_b_turn_heading_keeps_each_of_five_copy_numbers() {
        for number in 1..=5 {
            for width in [442.0, 760.0] {
                let mut run = super::super::tests::run();
                let base = run.state.participants[0].clone();
                run.state.participants = (1..=5)
                    .map(|n| {
                        let mut p = base.clone();
                        p.name = format!("Cinder Wolf {n}");
                        p.key = format!("wolf-{n}");
                        p.initiative = Some(20 - n);
                        p
                    })
                    .collect();
                run.state.order = (0..5).collect();
                run.state.cursor = number - 1;
                run.state.phase = Phase::Running;
                run.state.round = 1;
                let mut harness = egui_kittest::Harness::builder()
                    .with_size(vec2(width, 640.0))
                    .build(move |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            run.live_header(ui, design::RUN_REGULAR);
                        });
                    });
                design::theme::install(&harness.ctx);
                harness.run_steps(2);
                let texts: Vec<_> = harness
                    .output()
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => Some(text),
                        _ => None,
                    })
                    .collect();
                let heading = texts
                    .iter()
                    .find(|text| {
                        text.galley
                            .job
                            .sections
                            .iter()
                            .any(|section| section.format.color == colors().turn.current)
                    })
                    .expect("turn heading");
                assert!(
                    heading.galley.text().contains(&number.to_string()),
                    "iter6-B-FT1: {}",
                    heading.galley.text()
                );
                assert!(
                    !heading.galley.elided,
                    "iter6-B-FT1: the fitted heading stays whole"
                );
            }
        }
    }

    #[test]
    fn iter6_b_narrow_header_keeps_add_word() {
        let mut run = super::super::tests::run();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(442.0, 640.0))
            .build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    run.live_header(ui, design::RUN_REGULAR);
                });
            });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        assert!(
            harness
                .output()
                .shapes
                .iter()
                .any(|shape| matches!(&shape.shape,
            egui::Shape::Text(text) if text.galley.text().contains("Add") && !text.galley.elided)),
            "iter6-B-AT2: Add keeps a whole word"
        );
    }

    #[test]
    fn iter6_b_sheet_notice_keeps_text_colour_and_warning_bar() {
        let words = "Brannoch Vale’s sheet changed: HP 58 → 54, temp HP 5 → 0";
        let mut harness = egui_kittest::Harness::new(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                design::small_message(ui, Tone::Warning, words);
            });
        });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        let texts: Vec<_> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.text() == words => Some(text),
                _ => None,
            })
            .collect();
        assert!(
            !texts.is_empty(),
            "iter6-B-FT3: the sheet notice is painted"
        );
        assert!(
            texts.iter().all(|text| text
                .galley
                .job
                .sections
                .iter()
                .all(|section| section.format.color == colors().text)),
            "iter6-B-FT3: words use text colour"
        );
        assert!(harness.output().shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Rect(rect) if rect.fill == colors().result.warning)), "iter6-B-FT3: warning bar");
    }

    use super::*;
    use serde_yaml_ng::Mapping;
    use splat::content::{Entry, overrides::Overrides};

    #[test]
    fn iter8_b_crowded_row_keeps_worded_count_and_states_within_its_bounds() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut p = super::super::tests::run().state.participants[0].clone();
                p.hp = Some(5);
                p.present = false;
                p.conditions = vec![
                    "Poisoned".into(),
                    "Wreathed in the cinders of the old shrine".into(),
                ];
                for concentration in [false, true] {
                    p.concentration = concentration;
                    let layout = RowLayout::new(
                        ui,
                        442.40625,
                        &p,
                        Some(SyncState::Off),
                        design::RUN_COMPACT,
                        RowFit::Summary,
                        0.0,
                    );
                    let line = layout.second_line.as_ref().unwrap();
                    assert!(
                        line.rows
                            .iter()
                            .all(|row| row.size.x <= line.job.wrap.max_width),
                        "iter8-B-FT7-bounds: {}",
                        line.text()
                    );
                    assert!(
                        layout.height(design::RUN_COMPACT)
                            >= layout.name_line(design::RUN_COMPACT) + line.size().y,
                        "iter8-B-FT7-height: the row includes its full state text"
                    );
                    assert!(
                        line.text().replace('\u{a0}', " ").contains("1 more"),
                        "iter8-B-FT7-words: {}",
                        line.text()
                    );
                    for word in ["Bloodied", "Absent", "Poisoned", "Sync", "off"] {
                        assert!(line.text().contains(word));
                    }
                    assert!(
                        layout
                            .condition_spans
                            .iter()
                            .any(|(name, _)| name == "Run conditions")
                    );
                }
            });
        });
    }

    #[test]
    fn run_snapshot_notice_ignores_metadata_but_keeps_model_and_prose_changes() {
        let source =
            splat::content::definitions::source_file("definitions/creatures/wolf.md").unwrap();
        let current = Entry::parse("Wolf.md", source).unwrap();
        let mut snapshot = current.clone();
        snapshot.fields.insert("splat_schema".into(), 2.into());
        snapshot
            .fields
            .insert("splat_run_source_owner".into(), "Ashen Marches".into());
        assert!(snapshot_notice(&snapshot, Some(&current), false).is_none());

        let mut changed = current.clone();
        changed.fields["splat_stats"]["max_hp"]["value"] = 12.into();
        assert!(snapshot_notice(&snapshot, Some(&changed), false).is_some());

        let mut changed = current.clone();
        changed.body.push_str("\n\nA new attack.\n");
        assert!(snapshot_notice(&snapshot, Some(&changed), false).is_some());

        let mut whitespace = current.clone();
        whitespace.body.push_str("\n\n");
        assert!(snapshot_notice(&snapshot, Some(&whitespace), false).is_none());
    }

    #[test]
    fn run_copy_ignores_an_unchanged_source_but_reports_hp_and_source_removal() {
        let source =
            splat::content::definitions::source_file("definitions/creatures/wolf.md").unwrap();
        let mut snapshot = Entry::parse("Wolf.md", source).unwrap();
        let current = Entry::parse("Wolf.md", source).unwrap();
        assert!(snapshot_notice(&snapshot, Some(&current), false).is_none());
        let changed = Entry::parse(
            "Wolf.md",
            &source.replace("| Hit Points | 11", "| Hit Points | 12"),
        )
        .unwrap();
        assert!(
            snapshot_notice(&snapshot, Some(&changed), false)
                .unwrap()
                .contains("has changed.")
        );
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("Wolf.md");
        std::fs::write(&path, source).unwrap();
        snapshot.fields.insert(
            "splat_run_source_path".into(),
            path.to_string_lossy().to_string().into(),
        );
        snapshot
            .fields
            .insert("splat_run_source_owner".into(), "Ashen Marches".into());
        assert!(
            snapshot_notice(&snapshot, None, false)
                .unwrap()
                .contains("Ashen Marches is not used in this campaign now")
        );
        std::fs::remove_file(path).unwrap();
        assert!(
            snapshot_notice(&snapshot, None, false)
                .unwrap()
                .contains("was deleted")
        );
    }

    #[test]
    fn card_origin_does_not_repeat_a_name_change_and_joins_group_and_npc() {
        let text = crate::library::note_text("Guard", "creature", None);
        let mut p = Participant::with_overrides(
            "guard".into(),
            "Enemies".into(),
            Entry::parse("Guard.md", &text).unwrap(),
            &Mapping::new(),
            &Overrides {
                name: Some("Captain".into()),
                ..Default::default()
            },
            true,
        )
        .unwrap();
        assert_eq!(change_origin(&p), None);
        p.added_from = Some(super::super::AddedFrom {
            group: "Rival Team".into(),
            instance: "arrival".into(),
            round: 1,
            group_id: None,
        });
        assert_eq!(change_origin(&p).as_deref(), None);
        use egui_kittest::kittest::Queryable;
        let card = p.clone();
        let mut harness = egui_kittest::Harness::new(move |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| changes_block(ui, &card, true));
        });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        harness.get_by_label(&format!(
            "{} Changed · arrived with Rival Team",
            icons::CHANGED
        ));
        assert!(harness.query_by_label("Arrived with Rival Team").is_none());
        let changed = p.changes.take();
        assert_eq!(
            change_origin(&p).as_deref(),
            Some("Arrived with Rival Team")
        );
        p.changes = changed;
        Arc::make_mut(&mut p.definition)
            .fields
            .insert("splat_run_npc_name".into(), "Captain".into());
        assert_eq!(
            change_origin(&p).as_deref(),
            Some("Arrived with Rival Team · as Guard")
        );
        p.added_from = None;
        assert_eq!(change_origin(&p).as_deref(), Some("as Guard"));
    }

    /// iter-20-R-FT6, ST1: the notice says first what the card shows, with the card's own
    /// word, and the line under a character's card does not say it again.
    #[test]
    fn the_snapshot_notice_names_the_copy_once() {
        let text = crate::library::note_text("Cinder Wolf", "creature", None);
        let snapshot = Entry::parse("Cinder Wolf.md", &text).unwrap();
        let mut current = snapshot.clone();
        current.body.push_str("The pack howls.");
        assert_eq!(
            snapshot_notice(&snapshot, Some(&current), false).as_deref(),
            Some(
                "This stat block uses the copy from the start of the fight. Cinder Wolf.md has changed."
            )
        );
        assert_eq!(
            snapshot_notice(&snapshot, None, true).as_deref(),
            Some(
                "This stat block uses the copy from the start of the fight. Cinder Wolf.md cannot be read now."
            )
        );
        let mut sheet = snapshot.clone();
        sheet
            .fields
            .insert("splat_external".into(), serde_yaml_ng::Value::Bool(true));
        assert!(
            snapshot_notice(&sheet, Some(&current), false)
                .unwrap()
                .starts_with("This sheet uses the copy from the start of the fight.")
        );
        assert_eq!(
            snapshot_values_line(true),
            "The roster has the current values."
        );
        // Must stay true: with no notice above it, the line still says when it was saved.
        assert_eq!(
            snapshot_values_line(false),
            "Saved when this run began. The roster has the current values."
        );
    }

    /// Review repair 4: roll words always fit their place. When a row's conditions leave no
    /// room for one word, the words take a line of their own at the name's edge, and the row
    /// grows by that line.
    #[test]
    fn roll_words_take_their_own_line_when_the_conditions_fill_it() {
        let mut p = super::super::tests::run().state.participants[0].clone();
        p.initiative = Some(17);
        p.hp = Some(22);
        p.max_hp = Some(22);
        p.concentration = true;
        p.conditions = vec!["Grappled".into(), "Poisoned".into(), "Restrained".into()];
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let font = Type::Small.font(ui);
                let badge = "84% to hit · advantage";
                let scale = design::RUN_COMPACT;
                let plain = RowLayout::new(ui, 442.0, &p, None, scale, RowFit::Natural, 0.0);
                let layout = RowLayout::new(ui, 442.0, &p, None, scale, RowFit::Natural, 0.0)
                    .with_badge(ui, Some(badge));
                let (left, top, room) = badge_place(&layout, 0.0);
                let words = badge_words(ui, badge, &font, room);
                assert!(
                    words
                        .as_ref()
                        .is_some_and(|words| design::number_width(ui, words, &font) <= room),
                    "{words:?} in {room} px at {left} after {:?}",
                    layout
                        .second_line
                        .as_ref()
                        .map(|line| line.text().to_owned())
                );
                // On a line of its own at the name's edge, under the conditions: the row grows
                // by that one line, and a row without roll words does not.
                assert_eq!(left, layout.second_left);
                assert_eq!(top, plain.second_line.as_ref().unwrap().size().y);
                assert_eq!(
                    layout.height(scale),
                    plain.height(scale) + Type::Small.line()
                );
                let none = RowLayout::new(ui, 442.0, &p, None, scale, RowFit::Natural, 0.0)
                    .with_badge(ui, None);
                assert_eq!(none.height(scale), plain.height(scale));
                // Must stay true: words that fit after the conditions stay there.
                let mut fewer = p.clone();
                fewer.conditions.truncate(1);
                let room = RowLayout::new(ui, 442.0, &fewer, None, scale, RowFit::Natural, 0.0)
                    .with_badge(ui, Some(badge));
                assert!(!room.badge_below);
            });
        });
    }

    /// iter-20-R-FT4: a row's roll words have one place at every size: the line under the
    /// name, after its conditions, never the state column. Long words fit whole or are cut by
    /// whole words.
    #[test]
    fn roll_words_sit_under_the_name_after_its_conditions_at_every_size() {
        let mut base = super::super::tests::run().state.participants[0].clone();
        base.initiative = Some(17);
        let mut plain = base.clone();
        plain.hp = Some(22);
        plain.max_hp = Some(22);
        let mut conc = plain.clone();
        conc.concentration = true;
        let mut bloodied = conc.clone();
        bloodied.hp = Some(4);
        bloodied.conditions = vec!["Poisoned".into()];
        let mut long = plain.clone();
        long.name = "Cinder Wolf of the Long Ash Road Beyond the Ember Hills".into();
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let font = Type::Small.font(ui);
                for (scale, width) in [
                    (design::RUN_COMPACT, 442.0),
                    (design::RUN_REGULAR, 596.0),
                    (design::RUN_LARGE, 760.0),
                ] {
                    for (p, full) in [&plain, &conc, &bloodied, &long].into_iter().flat_map(|p| {
                        ["Saved · 10 damage (half)", "84% to hit · advantage"].map(|full| (p, full))
                    }) {
                        let layout =
                            RowLayout::new(ui, width, p, None, scale, RowFit::Natural, 0.0)
                                .with_badge(ui, Some(full));
                        let (left, top, room) = badge_place(&layout, 0.0);
                        let at = format!("{} at {width}", p.name);
                        assert!(left >= layout.name_left, "{at}");
                        assert!(
                            left + room < layout.hp_right,
                            "{at}: never in the state column"
                        );
                        match &layout.second_line {
                            // No room after the conditions: a line of their own under them.
                            Some(line) if layout.badge_below => {
                                assert_eq!(left, layout.second_left, "{at}");
                                assert_eq!(top, line.size().y, "{at}");
                            }
                            Some(line) => {
                                let last = line.rows.last().unwrap().rect();
                                assert!(left >= layout.second_left + last.right(), "{at}");
                                assert_eq!(top, last.top(), "{at}");
                            }
                            None => assert_eq!(left, layout.second_left, "{at}"),
                        }
                        // Under the name's last line, also when a long name wraps.
                        let rect =
                            Rect::from_min_size(pos2(0.0, 0.0), vec2(width, layout.height(scale)));
                        let name_top =
                            layout.line_y(scale, rect.height()) - layout.name_line(scale) / 2.0;
                        // A name's galley holds its line spacing under the last line.
                        let spacing = (layout.name.rows[0].rect().height()
                            - layout.name_line(scale))
                        .max(0.0);
                        assert!(
                            layout.second_y(scale, rect) + top
                                >= name_top + layout.name.size().y - spacing - 1.0,
                            "{at}: the words start under the name's last line"
                        );
                        {
                            let shown = badge_words(ui, full, &font, room).unwrap();
                            let kept = shown.trim_end_matches('…');
                            assert!(
                                shown == full
                                    || (shown.ends_with('…')
                                        && full.starts_with(kept)
                                        && full[kept.len()..].starts_with(' ')),
                                "{at}: {shown:?} is {full:?} or cut by whole words"
                            );
                            // With at most `Conc` before them, they are whole from 1200 up.
                            if scale.row >= design::RUN_REGULAR.row && p.conditions.is_empty() {
                                assert_eq!(
                                    shown,
                                    full,
                                    "{at}: whole where the line has room; room {room}, second {:?}",
                                    layout.second_line.as_ref().map(|l| l.text().to_owned())
                                );
                            }
                        }
                    }
                }
            });
        });
    }

    #[test]
    fn run_snapshot_notice_distinguishes_unchanged_changed_unreadable_and_deleted() {
        let text = crate::library::note_text("Wolf", "creature", None);
        let snapshot = Entry::parse("Wolf.md", &text).unwrap();
        assert!(snapshot_notice(&snapshot, Some(&snapshot), false).is_none());
        let mut current = snapshot.clone();
        current.body.push_str("A new attack.");
        assert!(
            snapshot_notice(&snapshot, Some(&current), false)
                .unwrap()
                .contains("has changed.")
        );
        assert!(
            snapshot_notice(&snapshot, None, true)
                .unwrap()
                .contains("cannot be read now")
        );
        assert!(
            snapshot_notice(&snapshot, None, false)
                .unwrap()
                .contains("is not available in this campaign now")
        );
    }

    #[test]
    fn a_replaced_creature_keeps_its_stat_block_name_in_run_rows() {
        let text = crate::library::note_text("Guard", "creature", None)
            .replace("max_hp: {}", "max_hp: {value: 11}");
        let entry = Entry::parse("Guard.md", &text).unwrap();
        let mut p = Participant::with_overrides(
            "wolf".into(),
            "Enemies".into(),
            entry,
            &Mapping::new(),
            &Overrides {
                name: Some("Cinder Wolf".into()),
                ..Default::default()
            },
            true,
        )
        .unwrap();
        p.name = "Cinder Wolf 1".into();
        assert!(p.named_stat_block());
        assert!(row_name(&p, TurnMark::None, None).contains("as Guard"));
    }

    #[test]
    fn initiative_bonus_formats_and_parses_signed_values() {
        for (value, shown) in [(1, "+1"), (0, "+0"), (-1, "−1")] {
            assert_eq!(signed_bonus(value), shown);
            assert_eq!(parse_bonus(shown), Some(value as f64));
            assert_eq!(parse_bonus(&value.to_string()), Some(value as f64));
        }
    }

    #[test]
    fn arrival_status_wraps_without_a_trailing_separator() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for width in [150.0, 180.0, 400.0] {
                    for status in [
                        "Changed",
                        "Added in round 2 · Changed",
                        "Added before combat",
                    ] {
                        let summary = arrival_summary(ui, status, "2 × Wolf, Captain Varra", width);
                        let galley = ui.painter().layout(
                            summary,
                            Type::Small.font(ui),
                            colors().muted,
                            width,
                        );
                        for row in &galley.rows {
                            assert_ne!(row.glyphs.last().map(|glyph| glyph.chr), Some('·'));
                            assert!(
                                !row.glyphs
                                    .first()
                                    .is_some_and(|glyph| { glyph.chr.is_ascii_digit() })
                                    || row
                                        .glyphs
                                        .iter()
                                        .map(|glyph| glyph.chr)
                                        .collect::<String>()
                                        .starts_with("2 ×")
                            );
                            assert!(row.size.x <= width + 1.0);
                        }
                        if width == 400.0 && status.contains("round") {
                            assert!(galley.rows.iter().any(|row| {
                                row.glyphs
                                    .iter()
                                    .map(|glyph| glyph.chr)
                                    .collect::<String>()
                                    .replace('\u{a0}', " ")
                                    .trim()
                                    .contains("Added in round 2 · Changed")
                            }));
                        }
                        if width == 400.0 {
                            assert_eq!(galley.rows.len(), 1);
                        }
                    }
                }
            });
        });
    }

    #[test]
    fn hidden_shared_roll_of_three_has_no_tie_warning() {
        let mut run = super::super::tests::run();
        for p in &mut run.state.participants[..3] {
            p.initiative = Some(17);
            p.shared_roll = Some("goblins".into());
        }
        assert_eq!(run.tied_count(), 0);
        assert_eq!(run.setup_hidden_attention(&[0, 1, 2]), "");
        assert!(run.setup_tie_directions(1).is_some());
    }

    #[test]
    fn hidden_independent_equal_rolls_say_two_tied() {
        let mut run = super::super::tests::run();
        run.state.participants[0].initiative = Some(17);
        run.state.participants[1].initiative = Some(17);
        run.state.participants[2].initiative = Some(12);
        assert_eq!(run.tied_count(), 2);
        assert_eq!(run.setup_hidden_attention(&[0, 1]), ", 2 tied");
        run.initiative.roster_ties = true;
        assert_eq!(run.setup_hidden_attention(&[0, 1]), "");
    }

    #[test]
    fn saving_wait_and_save_failure_have_distinct_statuses() {
        let mut run = super::super::tests::run();
        run.dirty = true;
        let status = run.save_status_value();
        assert_eq!(status.icon, icons::DRAFT);
        assert_eq!(status.color, colors().document.unsaved);
        run.disk_conflict = true;
        let status = run.save_status_value();
        assert_eq!(status.word, "Not saved");
        assert_eq!(status.icon, icons::ERROR);
        assert_eq!(status.color, colors().document.invalid);
    }

    #[test]
    fn closing_lists_changes_with_their_targets() {
        let mut run = super::super::tests::run();
        run.state.participants[0].hp = Some(11);
        run.state.participants[0].temporary_hp = 0;
        let name = run.state.participants[0].name.clone();
        run.apply(Command::Damage(0, 3)).unwrap();
        assert_eq!(
            run.unsaved_summary(),
            vec![format!("3 damage to {name} (8 HP left)")]
        );
        run.apply(Command::Condition(0, "Prone".into())).unwrap();
        assert_eq!(run.unsaved_summary().len(), 2);
        assert_eq!(run.unsaved_summary()[1], format!("Prone on {name}"));
        run.undo();
        assert_eq!(run.unsaved_summary().len(), 1);
    }

    #[test]
    fn iter_11_b_a_sheet_step_gives_up_its_source_before_its_target() {
        let entry = Entry::parse("vale.md", &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: creature\nsplat_name: Brannoch Vale\nsplat_creature: {{type: humanoid}}\nsplat_stats: {{max_hp: {{value: 62}}}}\n", Uuid::new_v4()), "").expect("schema-2 fixture")).unwrap();
        let p =
            Participant::new("vale".into(), "party".into(), entry, &Mapping::new(), true).unwrap();
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let font = Type::Strong.font(ui);
                // One pixel over each form's own width: the parts are measured apart.
                let words = |label: &str, width: f32| {
                    undo_words(
                        ui,
                        label,
                        true,
                        std::slice::from_ref(&p),
                        &font,
                        width + 1.0,
                    )
                };
                let full = format!("Undo: {SHEET_SOURCE}HP for {}", p.name);
                // Room for all of it: nothing changes.
                let wide = design::number_width(ui, &full, &font);
                assert_eq!(words(&full, wide), full);
                // The name shortens first, while the source fits.
                let named = format!("Undo: {SHEET_SOURCE}HP for Brannoch…");
                assert_eq!(words(&full, design::number_width(ui, &named, &font)), named);
                // Then the source goes, and the target stays.
                let short = "Undo: HP for Brannoch…";
                assert_eq!(words(&full, design::number_width(ui, short, &font)), short);
                // A count of characters has no name to shorten.
                let many = format!("Undo: {SHEET_SOURCE}values for 2 characters");
                let target = "Undo: values for 2 characters";
                assert_eq!(
                    words(&many, design::number_width(ui, target, &font)),
                    target
                );
                // iter11-B2: a DM's effect that starts with the same words keeps them.
                let effect = format!("Undo: {SHEET_SOURCE}boon on {}", p.name);
                let tight = design::number_width(ui, "Undo: boon on Brannoch…", &font);
                let shown = undo_words(
                    ui,
                    &effect,
                    false,
                    std::slice::from_ref(&p),
                    &font,
                    tight + 1.0,
                );
                assert!(
                    shown.starts_with(&format!("Undo: {SHEET_SOURCE}boon on ")),
                    "{shown}"
                );
                // A step from the DM keeps its words, however tight.
                let damage = format!("Undo: 3 damage to {}", p.name);
                assert!(words(&damage, 10.0).starts_with("Undo: 3 damage to "));
            });
        });
    }

    #[test]
    fn iter_2_b_undo_keeps_actions_directions_amounts_and_copy_numbers() {
        let entry = Entry::parse("wolf.md", &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: creature\nsplat_name: Cinder Wolf 3\nsplat_creature: {{type: beast}}\nsplat_stats: {{max_hp: {{value: 33}}}}\n", Uuid::new_v4()), "").expect("schema-2 fixture")).unwrap();
        let p = Participant::new(
            "wolf".into(),
            "enemies".into(),
            entry,
            &Mapping::new(),
            true,
        )
        .unwrap();
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let font = Type::Strong.font(ui);
                for (before, after) in [
                    ("7 damage to ", ""),
                    ("Frightened on ", ""),
                    ("Frightened off ", ""),
                    ("", " present"),
                    ("", " absent"),
                    ("initiative for ", ""),
                    ("start concentration for ", ""),
                    ("stop concentration for ", ""),
                    ("exhaustion 2 → 3 on ", ""),
                    ("exhaustion 3 → 1 on ", ""),
                ] {
                    let full = format!("Undo: {before}{}{after}", p.name);
                    let width =
                        design::number_width(ui, &format!("Undo: {before}Cinder… 3{after}"), &font);
                    let shown =
                        undo_words(ui, &full, false, std::slice::from_ref(&p), &font, width);
                    // iter10-B-FT2: the word Undo stays; the target is shortened.
                    assert!(
                        shown.starts_with(&format!("Undo: {before}")) && shown.ends_with(after),
                        "{shown}"
                    );
                    assert!(shown.contains("… 3"), "{shown}");
                    assert!(design::number_width(ui, &shown, &font) <= width, "{shown}");
                }
                for direction in ["on", "off"] {
                    let full = format!(
                        "Undo: Wreathed in the cinders of the old shrine {direction} {}",
                        p.name
                    );
                    let verb = if direction == "on" { "add" } else { "remove" };
                    let expected = format!("Undo {verb}: Wreathed in… {direction} Cinder… 3");
                    let width = design::number_width(ui, &expected, &font);
                    assert_eq!(
                        undo_words(ui, &full, false, std::slice::from_ref(&p), &font, width),
                        expected
                    );
                }
            });
        });
    }

    /// FT3: a long name wraps in the shared name column, and AC, temporary HP and HP keep the
    /// short row's x. Past three lines the name ends in `…`, and the row is tall enough for it.
    #[test]
    fn iter_17_b_long_names_keep_the_numeric_columns() {
        let short = super::super::tests::run().state.participants[0].clone();
        let mut long = short.clone();
        long.name = "Seraphina Thornwood-Ashgrave of the Ninth Bell".into();
        let mut longest = short.clone();
        longest.name = "Seraphina Thornwood-Ashgrave of the Ninth Bell, Warden of the Cinder \
                        Road and the Ashen Gate"
            .into();
        // Edge content: one word longer than the three lines.
        let mut one_word = short.clone();
        one_word.name = "Thornwood".repeat(10);
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for (scale, width) in [
                    (design::RUN_COMPACT, 442.40625),
                    (design::RUN_REGULAR, 596.0),
                    (design::RUN_LARGE, 760.0),
                ] {
                    // As the roster measures it: the longest name in the list.
                    let font = Type::Name.sized(ui, scale.name);
                    let widest = design::number_width(ui, &longest.name, &font) + 1.0;
                    for fit in [RowFit::Natural, RowFit::Compressed, RowFit::Summary] {
                        let layout = |p: &Participant| {
                            RowLayout::new(ui, width, p, None, scale, fit, widest)
                        };
                        let base = layout(&short);
                        for p in [&long, &longest, &one_word] {
                            let row = layout(p);
                            assert_eq!(
                                (row.ac_right, row.temp_right, row.hp_right),
                                (base.ac_right, base.temp_right, base.hp_right),
                                "{} moves the columns at {width} and scale {}",
                                p.name,
                                scale.row
                            );
                            assert!(row.name.rows.len() <= NAME_LINES, "{}", p.name);
                            // Must stay true: the row grows for every name line, and the
                            // second line stays inside the row.
                            let height = row.height(scale);
                            let rect = Rect::from_min_size(egui::Pos2::ZERO, vec2(width, height));
                            let name_top = row.line_y(scale, height) - row.name_line(scale) / 2.0;
                            let second = row.second_line.as_ref().map_or(0.0, |line| line.size().y);
                            assert!(
                                name_top >= 0.0
                                    && name_top + row.name.size().y <= height + 1.0
                                    && row.second_y(scale, rect) + second <= height + 1.0,
                                "the name or detail leaves the row at {width} and scale {}: \
                                 height {height}, name {name_top} + {}, second {} + {second}",
                                scale.row,
                                row.name.size().y,
                                row.second_y(scale, rect),
                            );
                        }
                    }
                }
                // The compact column at 960x640 cuts the longest name at three lines.
                let row = RowLayout::new(
                    ui,
                    442.40625,
                    &longest,
                    None,
                    design::RUN_COMPACT,
                    RowFit::Natural,
                    0.0,
                );
                let last = row.name.rows.last().and_then(|line| line.glyphs.last());
                assert!(row.name_cut && !row.name.elided && row.name.rows.len() == NAME_LINES);
                assert_eq!(last.map(|glyph| glyph.chr), Some('…'));
                assert!(row.name.text().starts_with("Seraphina Thornwood-Ashgrave"));
                // The fixture's long name fits in three lines, so it stays whole.
                let row = RowLayout::new(
                    ui,
                    442.40625,
                    &long,
                    None,
                    design::RUN_COMPACT,
                    RowFit::Natural,
                    0.0,
                );
                assert!(!row.name_cut && row.name.text() == long.name);
                let row = RowLayout::new(
                    ui,
                    442.40625,
                    &one_word,
                    None,
                    design::RUN_COMPACT,
                    RowFit::Natural,
                    0.0,
                );
                assert!(row.name_cut && row.name.text().ends_with('…'));
            });
        });
    }

    #[test]
    fn iter_15_b_inline_conflict_starts_at_the_second_line_edge() {
        let base = super::super::tests::run().state.participants[0].clone();
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                // The x where the state's icon starts on the second line.
                let icon_x = |participant: &Participant, sync: Option<SyncState>| {
                    let layout = RowLayout::new(
                        ui,
                        442.40625,
                        participant,
                        sync,
                        design::RUN_COMPACT,
                        RowFit::Summary,
                        0.0,
                    );
                    assert!(layout.inline_state, "the compact row puts its state inline");
                    let line = layout.second_line.as_ref().unwrap();
                    let text = line.text();
                    let word = if sync.is_some() {
                        "Conflict"
                    } else {
                        "Bloodied"
                    };
                    let at = text.find(word).expect("the state is on the second line");
                    let icon = text[..at].chars().count() - 2;
                    let before = text[..at].chars().take(icon).collect::<String>();
                    let x = line.pos_from_cursor(egui::text::CCursor::new(icon)).left();
                    (x, before)
                };
                let mut alone = base.clone();
                alone.side = "party".into();
                alone.conditions.clear();
                alone.hp = alone.max_hp;
                alone.initiative = Some(14);
                let mut bloodied = alone.clone();
                bloodied.hp = alone.max_hp.map(|max| max / 3);
                let (conflict, before) = icon_x(&alone, Some(SyncState::Conflict));
                let (hurt, _) = icon_x(&bloodied, None);
                assert_eq!(before, "", "nothing comes before a conflict that is alone");
                assert!(
                    (conflict - hurt).abs() < 0.5,
                    "Conflict starts at {conflict}, Bloodied at {hurt}"
                );
                // Must stay true: after other words, the state keeps its space from them.
                let mut prone = alone.clone();
                prone.conditions = vec!["Prone".into()];
                let (after, before) = icon_x(&prone, Some(SyncState::Conflict));
                assert!(
                    before.ends_with(' ') && before.contains("Prone"),
                    "{before:?}"
                );
                let prone_end = {
                    let layout = RowLayout::new(
                        ui,
                        442.40625,
                        &prone,
                        Some(SyncState::Conflict),
                        design::RUN_COMPACT,
                        RowFit::Summary,
                        0.0,
                    );
                    let line = layout.second_line.unwrap();
                    let text = line.text();
                    let end = text[..text.find("Prone").unwrap()].chars().count() + "Prone".len();
                    line.pos_from_cursor(egui::text::CCursor::new(end)).left()
                };
                assert!(after >= prone_end + design::XS, "{after} after {prone_end}");
            });
        });
    }

    #[test]
    fn iter_1_b_reading_sheet_fits_beside_a_condition_and_overflow_count() {
        let mut participant = super::super::tests::run().state.participants[0].clone();
        participant.side = "party".into();
        participant.hp = Some(10);
        participant.max_hp = Some(41);
        participant.conditions = vec!["Poisoned".into(), "Restrained".into()];
        participant.initiative = Some(14);
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for (scale, width) in [
                    (design::RUN_COMPACT, 442.40625),
                    (design::RUN_REGULAR, 596.0),
                    (design::RUN_LARGE, 760.0),
                ] {
                    let layout = RowLayout::new(
                        ui,
                        width,
                        &participant,
                        Some(SyncState::FirstRead),
                        scale,
                        RowFit::Summary,
                        0.0,
                    );
                    let line = layout.second_line.as_ref().unwrap();
                    if layout.inline_state {
                        assert!(
                            line.text()
                                .replace('\u{a0}', " ")
                                .contains("Reading sheet…")
                        );
                        assert!(line.text().replace('\u{a0}', " ").contains("1 more"));
                        assert!(
                            layout
                                .condition_spans
                                .iter()
                                .any(|(name, _)| name == "Run conditions")
                        );
                    } else {
                        let status = sync_status(SyncState::FirstRead);
                        let state_width = design::number_width(
                            ui,
                            &format!("{} {}", status.icon, status.word),
                            &Type::Heading.sized(ui, scale.mark),
                        );
                        assert!(layout.hp_right + design::S + state_width <= width);
                    }
                    assert!(line.text().contains("Poisoned"));
                    assert_eq!(line.rows.len(), 1, "{}: {}", scale.row, line.text());
                }
            });
        });
    }

    #[test]
    fn iter5_b_arrival_fault_uses_text_colour_and_warning_mark() {
        // iter-22-B FT1: the mark's tone comes from the fault's kind. An unread change is a
        // warning; an invalid maximum HP is an error here, as it is in the group editor.
        for (fault, tone, other) in [
            (
                super::super::MemberFault::from("Member 2, Cinder Wolf: Changes: HP lots not read"),
                colors().result.warning,
                colors().result.error,
            ),
            (
                super::super::MemberFault::new(
                    super::super::FaultKind::InvalidChange,
                    "Member 1, Cinder Wolf: Maximum HP would become −28. It must be 1 or more."
                        .into(),
                ),
                colors().result.error,
                colors().result.warning,
            ),
        ] {
            let shown = fault.clone();
            let mut harness = egui_kittest::Harness::new(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    group_fault(ui, &shown);
                });
            });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            let texts: Vec<_> = harness
                .output()
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.text().contains(&*fault) => Some(text),
                    _ => None,
                })
                .collect();
            assert!(!texts.is_empty());
            for text in texts {
                assert!(
                    text.galley
                        .job
                        .sections
                        .iter()
                        .all(|section| section.format.color == colors().text)
                );
            }
            let fills: Vec<_> = harness
                .output()
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Rect(rect) => Some(rect.fill),
                    _ => None,
                })
                .collect();
            assert!(fills.contains(&tone), "{fault:?}: {fills:?}");
            assert!(!fills.contains(&other), "{fault:?}: {fills:?}");
        }
    }

    #[test]
    fn iter_1_b_saved_footer_closes_in_one_step() {
        use egui_kittest::kittest::Queryable;
        let run = std::rc::Rc::new(std::cell::RefCell::new(super::super::tests::run()));
        let directory = tempfile::tempdir().unwrap();
        run.borrow_mut()
            .configure_storage(Uuid::new_v4(), directory.path(), Default::default())
            .unwrap();
        let closed = std::rc::Rc::new(std::cell::Cell::new(false));
        let shown = run.clone();
        let result = closed.clone();
        let mut harness = egui_kittest::Harness::new(move |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                if shown.borrow_mut().live_footer(ui) {
                    result.set(true);
                }
            });
        });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        harness.get_by_label("Close run").click();
        harness.run_steps(2);
        assert!(closed.get());
        assert!(!run.borrow().closing);
    }

    #[test]
    fn iter_2_b_sheet_notice_uses_header_room_and_keeps_undo() {
        use egui_kittest::kittest::Queryable;
        for (width, in_header) in [(1920.0, true), (442.0, false)] {
            let mut run = super::super::tests::run();
            for (index, total) in [20, 15, 10].into_iter().enumerate() {
                run.apply(Command::Initiative(index, Some(total))).unwrap();
            }
            run.apply(Command::Begin).unwrap();
            run.notice =
                Some("Guard 1’s sheet changed: HP 17 → 15. Undo for Guard 1 stops here.".into());
            run.notice_summary = Some("Guard 1’s sheet changed: HP 17 → 15".into());
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 640.0))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        run.live_header(ui, design::RUN_REGULAR);
                    });
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            assert_eq!(
                harness
                    .query_all_by_label_contains("Guard 1’s sheet changed: HP 17 → 15")
                    .count()
                    > 0,
                in_header,
                "{width}: {:?}",
                harness.root()
            );
            assert!(harness.query_by_label("Undo: begin combat").is_some());
        }
    }

    #[test]
    fn iter_2_b_tie_arrows_keep_their_columns() {
        use egui_kittest::kittest::Queryable;
        let mut run = super::super::tests::run();
        run.initiative.roster_ties = false;
        run.state.order = vec![0, 1];
        for p in &mut run.state.participants {
            p.initiative = Some(10);
        }
        let first = run.state.participants[0].name.clone();
        let last = run.state.participants[1].name.clone();
        let mut harness = egui_kittest::Harness::new(move |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for i in [0, 1] {
                    ui.push_id(i, |ui| {
                        run.setup_tie_controls(ui, i);
                    });
                }
            });
        });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        let down = harness
            .get_by_label(&format!("Move {first} after {last}"))
            .rect();
        let up = harness
            .get_by_label(&format!("Move {last} before {first}"))
            .rect();
        assert!(down.left() >= up.right(), "down {down:?}, up {up:?}");
    }

    #[test]
    fn a_combat_row_initiative_opens_a_field_to_change_it() {
        use egui_kittest::kittest::Queryable;
        let mut run = super::super::tests::run();
        for (index, total) in [20, 15, 10].into_iter().enumerate() {
            run.apply(Command::Initiative(index, Some(total))).unwrap();
        }
        run.apply(Command::Begin).unwrap();
        let run = std::rc::Rc::new(std::cell::RefCell::new(run));
        let shown = run.clone();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(1280.0, 720.0))
            .build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut run = shown.borrow_mut();
                    run.live_keys(ui);
                    run.roster(ui, 600.0, 0.0);
                });
            });
        design::theme::install(&harness.ctx);
        run.borrow_mut().select(1);
        harness.run_steps(2);
        // Escape leaves the total as it was, and the DM's look at another row stays.
        harness
            .get_by_label("Change initiative for Guard 3")
            .click();
        harness.run_steps(3);
        assert_eq!(run.borrow().live_initiative, Some(2));
        harness.key_press(Key::Escape);
        harness.run_steps(3);
        assert_eq!(run.borrow().live_initiative, None);
        assert_eq!(run.borrow().state.participants[2].initiative, Some(10));
        assert_eq!(run.borrow().selected, 1);
        // Enter with no number closes the field too.
        harness
            .get_by_label("Change initiative for Guard 3")
            .click();
        harness.run_steps(3);
        harness.key_press(Key::Backspace);
        harness.run_steps(1);
        harness.key_press(Key::Enter);
        harness.run_steps(3);
        assert_eq!(run.borrow().live_initiative, None);
        assert_eq!(run.borrow().state.participants[2].initiative, Some(10));
        // The field opens on the total, and what is typed replaces it.
        harness
            .get_by_label("Change initiative for Guard 3")
            .click();
        harness.run_steps(3);
        assert_eq!(
            run.borrow().setup_inputs.get(&2).map(String::as_str),
            Some("10")
        );
        harness
            .get_by_label("Initiative for Guard 3")
            .type_text("18");
        harness.run_steps(1);
        harness.key_press(Key::Enter);
        harness.run_steps(3);
        let run = run.borrow();
        assert_eq!(run.state.participants[2].initiative, Some(18));
        assert_eq!(run.state.order, vec![0, 2, 1]);
        assert_eq!(run.current(), Some(0));
        assert_eq!(run.live_initiative, None);
    }

    #[test]
    fn iter4_b_party_sync_off_is_once_in_header_and_mixed_rows_keep_their_state() {
        let mut run = super::super::tests::run();
        while run.state.participants.len() < 6 {
            run.state
                .participants
                .push(run.state.participants[0].clone());
        }
        for index in 0..5 {
            super::super::tests::linked(&mut run, index, &index.to_string());
        }
        assert!(
            run.party_sync_off(),
            "An encounter with sync disabled keeps its shared header notice"
        );
        run.keep_party_synced = true;
        assert!(
            !run.party_sync_off(),
            "Waiting for the connection is not a disabled party"
        );
        run.keep_party_synced = false;
        for index in 0..5 {
            assert_eq!(
                run.sync_word(index),
                None,
                "Untouched linked rows have no sync word"
            );
        }
        // Only workers the DM stopped supply Off. When all are stopped, the header
        // carries their shared state once, as before.
        for index in 0..5 {
            run.syncs
                .insert(index, crate::combat_sync::CombatSync::default());
        }
        assert!(run.party_sync_off());
        for index in 0..5 {
            assert_eq!(run.sync_word(index), Some(SyncState::Off));
            assert_eq!(run.row_sync_word(index), None);
        }
        assert_eq!(run.row_sync_word(5), None);
        run.syncs.insert(
            0,
            crate::combat_sync::CombatSync::simulated_conflict(1, 10, 1),
        );
        assert_eq!(run.row_sync_word(0), Some(SyncState::Conflict));
        assert_eq!(run.row_sync_word(1), Some(SyncState::Off));
        let mut stopped = crate::combat_sync::CombatSync::default();
        stopped.start();
        stopped.session_ended();
        run.syncs.insert(1, stopped);
        // A lost connection stopped it; sync starts again by itself, so the row is not Sync off.
        assert_eq!(run.row_sync_word(1), None);
        // Rows that wait for the connection, or a party kept in sync, are not Sync off either.
        run.syncs.clear();
        run.sync_to_restore = vec![run.state.participants[2].key.clone()];
        assert_eq!(run.row_sync_word(2), None);
        assert_eq!(run.row_sync_word(3), None);
        run.keep_party_synced = true;
        assert_eq!(run.row_sync_word(3), None);
    }

    #[test]
    fn iter9_b_correct_does_not_change_sync_and_explicit_stop_stays() {
        use std::{cell::RefCell, rc::Rc};
        for connected in [false, true] {
            for explicitly_stopped in [false, true] {
                let mut run = super::super::tests::run();
                for index in 0..2 {
                    super::super::tests::linked(&mut run, index, &index.to_string());
                }
                run.keep_party_synced();
                run.connected = connected;
                if explicitly_stopped {
                    run.syncs
                        .insert(0, crate::combat_sync::CombatSync::default());
                }
                let expected = explicitly_stopped.then_some(SyncState::Off);
                assert_eq!(run.row_sync_word(0), expected);
                let stopped_before = run.stopped_players();
                let run = Rc::new(RefCell::new(run));
                let shown = run.clone();
                let mut harness = egui_kittest::Harness::builder()
                    .with_size(vec2(960.0, 1080.0))
                    .build(move |ctx| {
                        egui::CentralPanel::default()
                            .show(ctx, |ui| shown.borrow_mut().record_tab(ui, 0));
                    });
                design::theme::install(&harness.ctx);
                harness.run_steps(3);
                let run = run.borrow();
                assert_eq!(
                    run.row_sync_word(0),
                    expected,
                    "iter9-B-FT8-no-side-effect connected={connected}"
                );
                assert_eq!(
                    run.stopped_players(),
                    stopped_before,
                    "Viewing Correct must not persist a stop"
                );
                assert_eq!(
                    run.row_sync_word(1),
                    None,
                    "An untouched second player stays unchanged"
                );
            }
        }
    }

    #[test]
    fn iter9_b_exhaustion_stepper_uses_scan_and_keeps_undo() {
        use egui_kittest::kittest::Queryable;
        use std::{cell::RefCell, rc::Rc};
        let mut run = super::super::tests::run();
        run.state.participants[0].conditions = vec!["Exhaustion (2)".into()];
        let run = Rc::new(RefCell::new(run));
        let shown = run.clone();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(560.0, 640.0))
            .build(move |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| shown.borrow_mut().conditions_tab(ui, 0));
            });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        for name in ["Decrease Exhaustion level", "Increase Exhaustion level"] {
            let rect = harness.get_by_label(name).rect();
            assert_eq!(
                rect.height(),
                Density::Scan.control(),
                "iter9-B-FT12-scan: {name}"
            );
        }
        harness.get_by_label("Increase Exhaustion level").click();
        harness.run_steps(2);
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            ["Exhaustion (3)"]
        );
        assert!(
            run.borrow()
                .undo_label()
                .contains("exhaustion 2 → 3 on Guard 1")
        );
        run.borrow_mut().undo();
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            ["Exhaustion (2)"]
        );
        harness.run_steps(2);
        for _ in 0..2 {
            harness.get_by_label("Decrease Exhaustion level").click();
            harness.run_steps(2);
        }
        assert!(
            run.borrow().state.participants[0].conditions.is_empty(),
            "Level zero removes exhaustion"
        );
    }

    /// The conditions tab for one creature at one exhaustion level, shared with its harness.
    fn exhaustion_harness(
        level: &str,
    ) -> (
        std::rc::Rc<std::cell::RefCell<Run>>,
        egui_kittest::Harness<'static>,
    ) {
        use std::{cell::RefCell, rc::Rc};
        let mut run = super::super::tests::run();
        run.state.participants[0].conditions = vec![level.into()];
        let run = Rc::new(RefCell::new(run));
        let shown = run.clone();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(560.0, 640.0))
            .build(move |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| shown.borrow_mut().conditions_tab(ui, 0));
            });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        (run, harness)
    }

    /// The exhaustion value field (its label has the same words).
    fn exhaustion_field<'h>(harness: &'h egui_kittest::Harness<'static>) -> egui_kittest::Node<'h> {
        use egui_kittest::kittest::{NodeT, Queryable};
        let mut fields = harness
            .query_all_by_label("Exhaustion level")
            .filter(|node| node.accesskit_node().role() == egui::accesskit::Role::SpinButton);
        let field = fields.next().expect("an exhaustion value field");
        assert!(fields.next().is_none(), "one exhaustion value field");
        field
    }

    /// Press the exhaustion value and move the pointer along `path` (x offsets in px; the
    /// stepper's speed is 0.1, so 10 px is one level). The pointer stays down.
    fn drag_exhaustion(harness: &mut egui_kittest::Harness<'static>, path: &[f32]) -> egui::Pos2 {
        let start = exhaustion_field(harness).rect().center();
        harness.hover_at(start);
        harness.run_steps(1);
        harness.drag_at(start);
        harness.run_steps(1);
        let mut at = start;
        for x in path {
            at = start + vec2(*x, 0.0);
            harness.hover_at(at);
            harness.run_steps(1);
        }
        // One more frame, so the accessibility tree shows the level the field shows.
        harness.run_steps(1);
        at
    }

    #[test]
    fn iter10_b_undo_falls_back_to_its_word_when_a_name_is_lost() {
        for (words, keeps) in [
            ("Undo: move Rival… 2 before Rival Team", true),
            ("Undo: D&D Beyond HP for Brannoch…", true),
            ("Undo: 7 damage to Cinder… 3", true),
            ("Undo: next turn", true),
            ("Undo: move … 2 before Rival Team", false),
            ("Undo: … 3", false),
            ("… 2", false),
        ] {
            assert_eq!(keeps_names(words), keeps, "iter10-B-FT2-lost-name: {words}");
        }
    }

    #[test]
    fn iter10_b_one_exhaustion_drag_is_one_undo_step() {
        use egui_kittest::kittest::Queryable;
        let (run, mut harness) = exhaustion_harness("Exhaustion (2)");
        let steps = run.borrow().history.len();
        let end = drag_exhaustion(&mut harness, &[4.0, 10.0, 20.0, 30.0, 40.0]);
        // Mid-drag, the field shows the level, but the run has no step yet.
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            ["Exhaustion (2)"],
            "iter10-B-ST1-mid-drag"
        );
        assert_eq!(
            run.borrow().history.len(),
            steps,
            "iter10-B-ST1-no-step-yet"
        );
        // The drag passed through at least two levels on its way.
        let shown: u8 = exhaustion_field(&harness).value().unwrap().parse().unwrap();
        assert!(shown >= 4, "iter10-B-ST1-shown: {shown}");
        harness.drop_at(end);
        harness.run_steps(2);
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            [format!("Exhaustion ({shown})")],
            "iter10-B-ST1-committed"
        );
        assert_eq!(
            run.borrow().history.len(),
            steps + 1,
            "iter10-B-ST1-one-step"
        );
        assert!(
            run.borrow()
                .undo_label()
                .contains(&format!("exhaustion 2 → {shown} on Guard 1")),
            "iter10-B-ST1-label: {}",
            run.borrow().undo_label()
        );
        run.borrow_mut().undo();
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            ["Exhaustion (2)"],
            "iter10-B-ST1-one-undo"
        );

        // A drag through 0 keeps the condition until it ends, then sets the end level once.
        harness.run_steps(2);
        let low = drag_exhaustion(&mut harness, &[-4.0, -10.0, -20.0, -30.0, -40.0]);
        assert_eq!(exhaustion_field(&harness).value().as_deref(), Some("0"));
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            ["Exhaustion (2)"],
            "iter10-B-ST1-through-zero: level 0 mid-drag does not remove exhaustion"
        );
        // Back up past the start level, in the same drag.
        let mut end = low;
        let mut shown = 0;
        for step in 1..=12 {
            end = low + vec2(10.0 * step as f32, 0.0);
            harness.hover_at(end);
            harness.run_steps(2);
            shown = exhaustion_field(&harness).value().unwrap().parse().unwrap();
            if shown >= 3 {
                break;
            }
        }
        assert!(shown >= 3, "iter10-B-ST1-back-up: {shown}");
        harness.drop_at(end);
        harness.run_steps(2);
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            [format!("Exhaustion ({shown})")],
            "iter10-B-ST1-through-zero-end"
        );
        assert_eq!(run.borrow().history.len(), steps + 1);

        // Must stay true: each − or + press is its own step.
        let before = run.borrow().history.len();
        for _ in 0..2 {
            harness.get_by_label("Increase Exhaustion level").click();
            harness.run_steps(2);
        }
        assert_eq!(
            run.borrow().history.len(),
            before + 2,
            "iter10-B-ST1-presses"
        );
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            [format!("Exhaustion ({})", (shown + 2).min(6))]
        );
        // A drag that goes out and comes back to where it began adds nothing.
        let level = exhaustion_field(&harness).value();
        // The level is at the top here, so the drag goes down and back.
        let end = drag_exhaustion(&mut harness, &[-4.0, -30.0]);
        assert_ne!(
            exhaustion_field(&harness).value(),
            level,
            "iter10-B-ST1-no-change: the drag must change the shown level partway"
        );
        let end = {
            let back = end + vec2(30.0, 0.0);
            harness.hover_at(back);
            harness.run_steps(2);
            assert_eq!(
                exhaustion_field(&harness).value(),
                level,
                "iter10-B-ST1-back"
            );
            back
        };
        harness.drop_at(end);
        harness.run_steps(2);
        assert_eq!(
            run.borrow().history.len(),
            before + 2,
            "iter10-B-ST1-no-change"
        );
    }

    #[test]
    fn iter10_lead_exhaustion_above_six_and_stale_drag() {
        use egui_kittest::kittest::Queryable;
        // A level above 6 (a hand-edited run or a sync) steps down by one, not to 6.
        let (run, mut harness) = exhaustion_harness("Exhaustion (8)");
        harness.get_by_label("Decrease Exhaustion level").click();
        harness.run_steps(2);
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            ["Exhaustion (7)".to_string()],
            "iter10-lead-above-six"
        );
        // A drag that the panel stopped drawing is dropped, not committed later.
        let steps = run.borrow().history.len();
        run.borrow_mut().exhaustion_drag = Some(ExhaustionDrag {
            index: 0,
            at: 0,
            start: 7,
            level: 3,
            frame: 0,
        });
        harness.run_steps(3);
        assert_eq!(run.borrow().history.len(), steps, "iter10-lead-stale-drag");
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            ["Exhaustion (7)".to_string()]
        );
    }

    #[test]
    fn iter10_b_exhaustion_limit_says_why() {
        use egui_kittest::kittest::{NodeT, Queryable};
        let (run, mut harness) = exhaustion_harness("Exhaustion (6)");
        let up = harness.get_by_label("Increase Exhaustion level");
        assert!(up.accesskit_node().is_disabled(), "iter10-B-ST2-disabled");
        assert_eq!(
            up.accesskit_node().description().as_deref(),
            Some("Exhaustion stops at 6."),
            "iter10-B-ST2-high"
        );
        let down = harness.get_by_label("Decrease Exhaustion level");
        assert!(!down.accesskit_node().is_disabled());
        assert_eq!(down.accesskit_node().description(), None);
        // The reason shows on hover.
        harness.get_by_label("Increase Exhaustion level").hover();
        harness.run_steps(60);
        let tooltips = harness
            .query_all_by_label_contains("Exhaustion stops at 6.")
            .count();
        assert_eq!(tooltips, 1, "iter10-B-ST2-hover: one tooltip");
        // Must stay true: a press at the limit changes nothing.
        let steps = run.borrow().history.len();
        harness.get_by_label("Increase Exhaustion level").click();
        harness.run_steps(2);
        assert_eq!(run.borrow().history.len(), steps);
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            ["Exhaustion (6)"]
        );
        // Mid-drag at 0, the decrease says why.
        let end = drag_exhaustion(&mut harness, &[-4.0, -20.0, -40.0, -60.0, -70.0]);
        assert_eq!(
            harness
                .get_by_label("Decrease Exhaustion level")
                .accesskit_node()
                .description()
                .as_deref(),
            Some("Exhaustion is already 0."),
            "iter10-B-ST2-low"
        );
        harness.drop_at(end);
        harness.run_steps(2);
        assert!(
            run.borrow().state.participants[0].conditions.is_empty(),
            "A drag to 0 removes exhaustion in one step"
        );
        assert_eq!(run.borrow().history.len(), steps + 1);
    }

    #[test]
    fn iter9_b_below_roster_notice_centres_compact_and_top_aligns_wrapped() {
        use egui_kittest::kittest::Queryable;
        for (width, height) in [(960.0, 640.0), (1200.0, 800.0), (1920.0, 1080.0)] {
            for wrapped in [false, true] {
                let mut run = super::super::tests::run();
                for (index, total) in [20, 15, 10].into_iter().enumerate() {
                    run.apply(Command::Initiative(index, Some(total))).unwrap();
                }
                run.apply(Command::Begin).unwrap();
                run.recording_notice = Some("Recording stopped");
                let summary = "Guard 1: HP 17 → 15";
                run.notice = Some(if wrapped {
                    "Guard 1 and Guard 2 have changed. ".repeat(8)
                } else {
                    summary.into()
                });
                run.notice_summary = (!wrapped).then(|| summary.into());
                let mut harness = egui_kittest::Harness::builder()
                    .with_size(vec2(width, height))
                    .build(move |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            run.live_view(ui);
                        });
                    });
                design::theme::install(&harness.ctx);
                harness.run_steps(3);
                let log = harness
                    .query_all_by_label("Log")
                    .map(|n| n.rect())
                    .max_by(|a, b| a.bottom().total_cmp(&b.bottom()))
                    .expect("Required notice Log link");
                let dismiss = harness.get_by_label("Dismiss this message").rect();
                let text = harness
                    .output()
                    .shapes
                    .iter()
                    .find_map(|shape| match &shape.shape {
                        egui::Shape::Text(text)
                            if text.galley.text().starts_with(if wrapped {
                                "Guard 1 and"
                            } else {
                                "Guard 1: HP"
                            }) =>
                        {
                            Some(text)
                        }
                        _ => None,
                    })
                    .expect("Required below-roster message text");
                let rect = Rect::from_min_size(text.pos, text.galley.size());
                if wrapped {
                    assert!(rect.height() > Type::Small.line(), "Required wrapped text");
                    assert!(
                        (rect.top() - dismiss.top()).abs() <= 1.0,
                        "iter9-B-AT2-wrap: {rect:?}, {dismiss:?}"
                    );
                } else {
                    assert!(
                        (rect.center().y - log.center().y).abs() <= 1.0
                            && (rect.center().y - dismiss.center().y).abs() <= 1.0,
                        "iter9-B-AT2-centre: {rect:?}, {log:?}, {dismiss:?}"
                    );
                }
            }
        }
    }

    /// Both `Log` links (below the roster, and inline in the control row) take their 24 px
    /// target from their own size: the painted word has no spaces, starts one item gap after the
    /// message, and its underline covers only the word.
    #[test]
    fn iter14_b_log_link_word_starts_at_its_gap_and_underline_covers_only_it() {
        use egui_kittest::kittest::Queryable;
        let summary = "Guard 1: HP 17 → 15";
        let mut paths = std::collections::BTreeSet::new();
        for (width, height) in [(960.0, 640.0), (1200.0, 800.0), (1920.0, 1080.0)] {
            for below in [true, false] {
                let mut run = super::super::tests::run();
                for (index, total) in [20, 15, 10].into_iter().enumerate() {
                    run.apply(Command::Initiative(index, Some(total))).unwrap();
                }
                run.apply(Command::Begin).unwrap();
                if below {
                    run.recording_notice = Some("Recording stopped");
                }
                run.notice = Some(summary.into());
                run.notice_summary = Some(summary.into());
                let mut harness = egui_kittest::Harness::builder()
                    .with_size(vec2(width, height))
                    .build(move |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            run.live_view(ui);
                        });
                    });
                design::theme::install(&harness.ctx);
                harness.run_steps(3);
                let gap = harness.ctx.style().spacing.item_spacing.x;
                let target = harness
                    .query_all_by_label("Log")
                    .map(|n| n.rect())
                    .max_by(|a, b| a.bottom().total_cmp(&b.bottom()))
                    .expect("Required notice Log link");
                let mut word = None;
                let mut message = None;
                let mut lines = Vec::new();
                for shape in &harness.output().shapes {
                    match &shape.shape {
                        egui::Shape::Text(text) if text.galley.text().trim() == "Log" => {
                            let rect = Rect::from_min_size(text.pos, text.galley.size());
                            if target.intersects(rect) {
                                word = Some((text.galley.text().to_owned(), rect));
                            }
                        }
                        egui::Shape::Text(text)
                            if text.galley.text().starts_with("Guard 1: HP") =>
                        {
                            message = Some(Rect::from_min_size(text.pos, text.galley.size()));
                        }
                        egui::Shape::LineSegment { points, .. }
                            if (points[0].y - points[1].y).abs() < 0.1 =>
                        {
                            lines.push((
                                points[0].x.min(points[1].x),
                                points[0].x.max(points[1].x),
                                points[0].y,
                            ));
                        }
                        _ => {}
                    }
                }
                let (painted, word) = word.expect("Required painted Log word");
                let message = message.expect("Required notice words");
                let underline = lines
                    .iter()
                    .find(|(left, right, y)| {
                        *y > word.center().y
                            && *y <= word.bottom() + 0.5
                            && *left < word.right()
                            && *right > word.left()
                    })
                    .copied();
                paths.insert(below);
                let context = format!(
                    "{width}x{height}, below {below}: painted {painted:?} at {word:?}, target {target:?}, \
                     message {message:?}, underline {underline:?}, gap {gap}"
                );
                assert_eq!(painted, "Log", "iter14-B-log-words: {context}");
                assert!(
                    underline.is_some_and(|(left, right, _)| (left - word.left()).abs() < 0.5
                        && (right - word.right()).abs() < 0.5),
                    "iter14-B-log-underline: {context}"
                );
                assert!(
                    target.width() >= design::XL - 0.01 && target.height() >= design::XL - 0.01,
                    "iter14-B-log-target: {context}"
                );
                // The word starts at the message's last word plus the row's item gap: the
                // default gap below the roster, `XS` in the control row.
                let separator = if below { gap } else { design::XS };
                assert!(
                    (word.left() - message.right() - separator).abs() < 0.5,
                    "iter14-B-log-gap: {context}"
                );
                // The two paths sit in different places: below the roster, or in the top row.
                assert_eq!(
                    word.top() > height / 2.0,
                    below,
                    "iter14-B-log-path: {context}"
                );
            }
        }
        assert_eq!(paths.len(), 2, "Both Log paths are reached");
    }

    #[test]
    fn iter_1_b_unsaved_close_still_asks_and_keeps_the_run() {
        let mut run = super::super::tests::run();
        run.disk_conflict = true;
        assert!(!run.request_close());
        assert!(run.closing);
        assert!(!run.confirm_close);

        // Spell slots kept only here are lost on close, so the close asks first.
        let mut run = super::super::tests::run();
        let mut sync = crate::combat_sync::CombatSync::default();
        sync.note_while_stopped("spells", serde_json::json!({"1": 1}));
        run.syncs.insert(0, sync);
        assert!(!run.unsent_only_here_names().is_empty());
        assert!(!run.request_close());
        assert!(run.closing);
        assert!(!run.confirm_close);
    }

    #[test]
    fn iter_1_b_arrival_total_accepts_blank_and_signed_whole_numbers() {
        for text in ["", "  "] {
            assert_eq!(arrival_total(text), Ok(None));
        }
        for number in [i64::MIN, -1, 0, 13, i64::MAX] {
            assert_eq!(arrival_total(&number.to_string()), Ok(Some(number)));
        }
        assert!(arrival_total("1.5").is_err());
    }

    #[test]
    fn close_names_only_the_live_losses() {
        assert_eq!(
            close_saved_words(false, false),
            "The run stays saved. Resume it from Paused run on the left, or from Encounters."
        );
        assert!(!close_saved_words(false, true).contains("Undo"));
        assert!(!close_saved_words(true, false).contains("Sync stops"));
        assert!(close_saved_words(true, true).contains("Undo for this run. Sync stops."));
    }

    #[test]
    fn dock_names_keep_copy_suffixes_and_change_previews_keep_units() {
        let mut harness = egui_kittest::Harness::builder().build(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let font = Type::Small.font(ui);
                let long = "Seraphina Thornwood-Ashgrave of the Ninth Bell";
                let room = design::number_width(ui, "Seraphina Thornwood-Ashgrave of", &font);
                let fitted = dock_name(ui, long, &font, room);
                assert_eq!(fitted, "Seraphina Thornwood-Ashgrave…");
                assert!(design::number_width(ui, &fitted, &font) <= room);
                for name in ["Cinder Wolf 12", "Cinder Wolf 2", "Goblin Warrior B"] {
                    let fitted = dock_name(ui, name, &font, 70.0);
                    assert!(
                        fitted.ends_with(name.split_whitespace().last().unwrap()),
                        "{fitted}"
                    );
                    assert!(design::number_width(ui, &fitted, &font) <= 70.0, "{fitted}");
                }
            });
        });
        harness.run_steps(2);
        assert_eq!(
            change_preview(&Row {
                label: "Speed",
                base: "30 ft.".into(),
                changed: "40 ft".into(),
                by_hand: true,
                in_rolls: false,
            }),
            "30 ft → 40 ft · add by hand"
        );
        assert_eq!(
            change_preview(&Row {
                label: "Attack bonus",
                base: "As printed".into(),
                changed: "+1 on each attack".into(),
                by_hand: false,
                in_rolls: true,
            }),
            "As printed → +1 on each attack · in Splat’s rolls"
        );
        assert_eq!(
            change_preview(&Row {
                label: "Maximum HP",
                base: "1000000".into(),
                changed: "-28".into(),
                by_hand: false,
                in_rolls: false,
            }),
            "1,000,000 → −28"
        );
        assert_eq!(ordinal(12), "12th");
        assert_eq!(ordinal(21), "21st");
    }

    #[test]
    fn wrapped_row_details_stay_above_the_rule_at_every_scale() {
        let entry = Entry::parse("captain.md", &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: creature\nsplat_name: Rival Captain\nsplat_creature: {{type: humanoid}}\nsplat_stats:\n  max_hp: {{value: 33}}\n  armor_class: {{value: 12}}\n", Uuid::new_v4()), "").expect("schema-2 fixture")).unwrap();
        let overrides = Overrides::parse(
            &serde_yaml_ng::from_str(
                "{attack_bonus: {add: 1}, damage_bonus: {add: 1}, speed: {set: 40}}",
            )
            .unwrap(),
        )
        .unwrap();
        let mut participant = Participant::with_overrides(
            "captain".into(),
            "rivals".into(),
            entry,
            &Mapping::new(),
            &overrides,
            true,
        )
        .unwrap();
        participant.conditions.push("Blessed".into());
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for scale in [design::RUN_COMPACT, design::RUN_REGULAR, design::RUN_LARGE] {
                    for fit in [RowFit::Natural, RowFit::Compressed, RowFit::Summary] {
                        for width in [400.0, 560.0, 760.0] {
                            let layout =
                                RowLayout::new(ui, width, &participant, None, scale, fit, 0.0);
                            let rect = Rect::from_min_size(
                                egui::Pos2::ZERO,
                                vec2(width, layout.height(scale)),
                            );
                            let detail = layout.second_line.as_ref().unwrap();
                            assert!(
                                detail.text().contains("Blessed"),
                                "first condition missing: {}",
                                detail.text()
                            );
                            assert!(
                                layout.second_y(scale, rect) + detail.size().y < rect.bottom(),
                                "detail crosses row rule at width {width} and scale {}",
                                scale.row
                            );
                            assert_eq!(layout.second_left, layout.name_left);
                        }
                    }
                }
                // F1: a changed live creature with concentration and one condition,
                // at the 1200x800 roster width beside its stat block.
                let mut live = participant.clone();
                live.side = "enemies".into();
                live.concentration = true;
                live.initiative = Some(14);
                let layout = RowLayout::new(
                    ui,
                    596.0,
                    &live,
                    None,
                    design::RUN_REGULAR,
                    RowFit::Natural,
                    0.0,
                );
                let detail = layout.second_line.as_ref().unwrap();
                assert!(
                    detail.text().contains("Conc")
                        && detail.text().contains("Blessed")
                        && detail.text().contains("Changed")
                );
                assert_eq!(detail.rows.len(), 1, "{}", detail.text());
                Arc::make_mut(&mut live.definition).name = "Ember Cultist".into();
                let layout = RowLayout::new(
                    ui,
                    442.40625,
                    &live,
                    None,
                    design::RUN_REGULAR,
                    RowFit::Summary,
                    0.0,
                );
                let detail = layout.second_line.as_ref().unwrap();
                assert!(detail.text().contains("Blessed") && detail.text().contains("Changed"));
                assert_eq!(detail.rows.len(), 1, "{}", detail.text());
            });
        });
    }
}
