//! The run's three views: setup, live and ended. Prominence follows the brief:
//! round and current turn, then every row's HP and conditions, then the actions.
//! Rows never move when the selection changes: the entry controls have a fixed dock.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{
    Command, END_STEP, MAX_FROM_SHEET, Panel, Participant, PartySync, Phase, Run, SAVED_HP_USED,
    SHEET_SOURCE, Unend, display, effect::Time, mode_label,
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

pub(super) const AMOUNT: &str = "Amount";
const OTHER_EFFECT_FIELD: design::FieldName = crate::field_id!("run_other_effect");
const OUTCOME_FIELD: design::FieldName = crate::field_id!("run_outcome");
const DM_NOTE_FIELD: design::FieldName = crate::field_id!("run_dm_note");
const READ_ALOUD: &str = "Read-aloud and notes";
/// FI10: Arrivals says what the fight becomes when a group joins.
pub(crate) const FIGHT_AFTER: &str = "Fight after: ";
/// Two lines of controls in a frame.
pub(super) const DOCK_HEIGHT: f32 = 36.0 + 4.0 + 28.0 + 8.0;
/// The dock's width from which Damage has its key hint, and Temp HP and Roll their full forms.
pub(super) const DOCK_ALL_WORDS_FROM: f32 = 500.0;
/// FK1 (iter-27-R): the dock's name slot, one width in every fight and for every row, so the
/// controls after it keep one x. A name that `Heading` does not hold on one line wraps in it.
pub(super) const DOCK_NAME_WIDTH: f32 = 8.0 * design::L;
/// The lines a dock name may take. Two share the height of one control.
const DOCK_NAME_LINES: usize = 2;
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
/// X8: the Conditions field's words while the dock has no room for `Effect`.
const OTHER_EFFECT_TIMED: &str = "Other effect, such as Bless 10 rounds";
/// X7: what the Conditions tab records, and what Splat does with a time the DM types.
const CONDITIONS_HELP: &str = "No HP, AC or roll changes by itself. A time you type counts down at the end of that creature’s turn: Bless 10 rounds goes from 10 to 9. Other effects stay local and do not sync.";
/// A creature's effects beyond the official conditions, in the run's Conditions tab and in a
/// change form alike: the `Other effects` label, then each in Body text with its row menu.
/// Returns the position in `effects` of the one the DM removed. `field` is the width of the
/// `Other effect…` field below: each row ends where that field's Add ends, so the form has one
/// right edge (FH2).
pub(crate) fn other_effects_list<'a>(
    ui: &mut egui::Ui,
    effects: impl IntoIterator<Item = &'a str>,
    field: f32,
) -> Option<usize> {
    let effects: Vec<_> = effects.into_iter().collect();
    if effects.is_empty() {
        return None;
    }
    let mut removed = None;
    design::heading(ui, design::Heading::Label("Other effects"));
    let row = (field + ui.spacing().item_spacing.x + other_effect_add().natural_width(ui))
        .min(ui.available_width());
    for (at, effect) in effects.into_iter().enumerate() {
        // The menu first, at the row's right edge, then the words in the rest of the row, both
        // centred on the row's line (FH2: an empty-height slot put the words below the menu).
        let size = vec2(row, Density::Read.control());
        ui.allocate_ui_with_layout(size, Layout::right_to_left(Align::Center), |ui| {
            ui.set_max_width(row);
            design::row_menu(ui, effect, |ui| {
                if ui.button("Remove").clicked() {
                    removed = Some(at);
                    ui.close();
                }
            });
            design::gap(ui, design::Gap::S);
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.add(egui::Label::new(Type::Body.rich(ui, effect).color(colors().text)).wrap());
            });
        });
    }
    removed
}

/// The other-effect field's Add, measured by `other_effects_list` for the form's right edge.
fn other_effect_add() -> Action<'static> {
    Action::secondary("Add").density(Density::Read)
}

/// The field that adds an other effect, and its Add. True when the DM adds the typed words;
/// then the field's response. `owner` is the key of the object whose draft it is (a run and
/// its participant, a group's member): egui keeps the field's cursor and its text Undo under
/// its id, so no caller leaves the owner out.
pub(crate) fn other_effect_field(
    ui: &mut egui::Ui,
    owner: impl design::RowKey,
    text: &mut String,
    (width, placeholder): (f32, &str),
    focus: bool,
) -> (bool, egui::Response) {
    let id = OTHER_EFFECT_FIELD.of(owner);
    ui.horizontal(|ui| {
        let response = design::line_field(ui, id, "Other effect", text, placeholder, width);
        if focus {
            response.request_focus();
        }
        let enter = response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
        let added = (other_effect_add()
            .blocked_if(text.trim().is_empty(), "Type an effect first.")
            .show(ui)
            .clicked()
            || enter)
            && !text.trim().is_empty();
        (added, response)
    })
    .inner
}

/// Run toggles use a selected surface; the turn action keeps the accent fill.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
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
    // FH1, FI3: an off toggle keeps the frame's padding, with nothing painted, so turning it
    // on moves no word and no neighbour.
    let button = if selected {
        button.fill(colors().selected)
    } else {
        button
            .frame(true)
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::NONE)
    };
    let response = ui.add(button);
    design::named(ui, &response, name);
    response
}

/// Armor class, hit points and initiative under capital labels, in the stat block's columns
/// (`library::creature::vitals`): each column as wide as its label or value and `S`, `L`
/// apart. So the trio keeps one place when the DM selects a made-up creature after a stat
/// block.
fn vital_columns(ui: &mut egui::Ui, columns: &[(&str, String)]) {
    ui.with_layout(
        Layout::left_to_right(Align::Min).with_main_wrap(true),
        |ui| {
            ui.spacing_mut().item_spacing = vec2(design::L, 2.0);
            for (label, value) in columns {
                let width = design::number_width(ui, label, &Type::Label.font(ui))
                    .max(design::number_width(ui, value, &Type::Prose.font(ui)))
                    + design::S;
                ui.allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    ui.label(Type::Label.rich(ui, *label).color(colors().muted));
                    ui.label(Type::Prose.rich(ui, value).color(colors().text));
                });
            }
        },
    );
}

/// The width `toggle` takes, on or off: its words and the frame's padding.
pub(super) fn toggle_width(ui: &egui::Ui, label: &str, icon: &str) -> f32 {
    #[cfg(test)]
    CONDITION_MEASURES.with(|count| count.set(count.get() + 1));
    let words = egui::WidgetText::from(Type::Body.rich(ui, format!("{icon} {label}")))
        .into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            egui::TextStyle::Button,
        )
        .size()
        .x;
    let v = &ui.visuals().widgets;
    let expansion = v
        .inactive
        .expansion
        .max(v.hovered.expansion)
        .max(v.active.expansion);
    words + 2.0 * (ui.spacing().button_padding.x + expansion)
}

#[cfg(test)]
thread_local! {
    /// How many times a group row scanned the run's participants for that group's joins.
    static GROUP_SCANS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
thread_local! {
    /// How often the Arrivals summaries and the Arrivals reasons were worked out (Astra 2): an
    /// unchanged frame works out none of them.
    pub(super) static PREPARED: std::cell::Cell<(usize, usize)> =
        const { std::cell::Cell::new((0, 0)) };
}

#[cfg(test)]
thread_local! {
    /// How many toggle widths the conditions grid measured.
    static CONDITION_MEASURES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Each column's width: its widest name in the selected (bordered) form. Measured once per
/// font size and padding, never from what is drawn, so no toggle moves its neighbours.
fn condition_column_widths(ui: &egui::Ui, columns: usize) -> Vec<f32> {
    let font = Type::Body.font(ui);
    let key = egui::Id::new("condition_column_widths")
        .with(font.size.to_bits())
        .with(ui.spacing().button_padding.x.to_bits())
        .with(ui.ctx().pixels_per_point().to_bits())
        .with(columns);
    if let Some(widths) = ui.data(|data| data.get_temp::<Vec<f32>>(key)) {
        return widths;
    }
    let mut widths = vec![0.0_f32; columns];
    for (at, (_, name)) in crate::combat_sync::CONDITIONS.iter().enumerate() {
        let width = toggle_width(ui, name, icons::OK);
        widths[at % columns] = widths[at % columns].max(width);
    }
    ui.data_mut(|data| data.insert_temp(key, widths.clone()));
    widths
}

/// The official conditions as toggles in fixed columns, `S` apart: the run's Conditions tab
/// and a group member's Starting conditions. `on` says whether a condition is on. Returns
/// the condition the DM pressed.
pub(crate) fn conditions_grid(
    ui: &mut egui::Ui,
    salt: &str,
    on: impl Fn(&str) -> bool,
) -> Option<&'static str> {
    conditions_grid_counted(ui, salt, &|name| on(name).then(|| name.to_owned())).0
}

/// The grid of `conditions_grid`, where `label` gives an on condition's words. X11: a
/// condition with a time shows its count on its tick, `✓ Frightened 10`, while every column
/// still fits the room; its column grows by the count. Else each tick shows its name only.
/// Returns the pressed condition, and whether the counts show.
fn conditions_grid_counted(
    ui: &mut egui::Ui,
    salt: &str,
    label: &dyn Fn(&str) -> Option<String>,
) -> (Option<&'static str>, bool) {
    let columns = if ui.available_width() >= design::CONDITION_THREE_COLUMNS_FROM {
        3
    } else {
        2
    };
    let mut widths = condition_column_widths(ui, columns);
    let mut counted = widths.clone();
    for (at, (_, name)) in crate::combat_sync::CONDITIONS.iter().enumerate() {
        if let Some(words) = label(name).filter(|words| words != name) {
            counted[at % columns] = counted[at % columns].max(toggle_width(ui, &words, icons::OK));
        }
    }
    let gaps = design::S * (columns - 1) as f32;
    let counts = counted.iter().sum::<f32>() + gaps <= ui.available_width();
    if counts {
        widths = counted;
    }
    let height = Density::Read.control();
    let mut pressed = None;
    ui.push_id(salt, |ui| {
        ui.spacing_mut().item_spacing.y = design::XS;
        for row in crate::combat_sync::CONDITIONS.chunks(columns) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = design::S;
                for ((_, name), width) in row.iter().zip(&widths) {
                    let size = vec2(*width, height);
                    ui.allocate_ui_with_layout(size, Layout::left_to_right(Align::Center), |ui| {
                        ui.set_min_size(size);
                        let words = label(name);
                        let selected = words.is_some();
                        let icon = if selected { icons::OK } else { icons::ADD };
                        let shown = words.filter(|_| counts);
                        let shown = shown.as_deref().unwrap_or(name);
                        if toggle(ui, shown, icon, selected, name, Density::Read).clicked() {
                            pressed = Some(*name);
                        }
                    });
                }
            });
        }
    });
    (pressed, counts)
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

/// FL4: the line under a pending sync block's heading in the ended view. The heading above it
/// names the character, and `Stop sync for …` under it is the way to close now, so the line
/// says only what waits and that it needs no step (two lines at 960, not three).
const PENDING_WORDS: &str =
    "Its changes have not reached D&D Beyond yet. Splat tries again by itself.";

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
        let follow = matches!(command, Command::Next | Command::Begin | Command::Continue);
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

    pub(super) fn clear_targets(&mut self) {
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
        // An open menu (a row's, a tile's, a split button's) takes Escape first: it closes,
        // and the armed action, the selection and the panel stay.
        if egui::Popup::is_any_open(ui.ctx()) && ui.input(|i| i.key_pressed(Key::Escape)) {
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
        let effect = letters
            && !(ui.input(|i| i.modifiers.shift || i.modifiers.alt))
            && ui.input_mut(|i| i.consume_key(Modifiers::NONE, Key::E));
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
        if effect {
            self.open_effect_field();
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

    pub(super) fn undo_action<'a>(&'a self, label: &'a str) -> Action<'a> {
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

    pub(super) fn empty_undo_reason(&self) -> &str {
        if self.undo_stopped {
            self.undo_boundary
                .as_deref()
                .unwrap_or("Nothing to undo since the sheets changed")
        } else if let Some(reason) = self.no_saved_undo {
            reason
        } else {
            "Nothing to undo"
        }
    }

    /// The words beside an empty Undo, before any fitting. While the unread-steps warning
    /// shows, it says the cause, so the header says only `Nothing to undo` (AI3).
    pub(super) fn empty_undo_words(&self) -> &str {
        if self.undo_unread {
            "Nothing to undo"
        } else {
            self.empty_undo_reason()
        }
    }

    pub(super) fn empty_undo(&self, ui: &mut egui::Ui) -> bool {
        let full = self.empty_undo_reason();
        let words = self.empty_undo_words();
        let action = self.undo_action("Undo");
        let room = ui.available_width() - action.natural_width(ui) - design::S;
        let shown = if design::number_width(ui, words, &Type::Small.font(ui)) <= room {
            words
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

    /// FI2: the run's one message footer. The notices that come by themselves (a changed file,
    /// an error, Undo steps that could not be read) and, in setup, the rest and carry-over
    /// offers and their result, sit in it, pinned below the roster, so a row the DM aims at
    /// never moves when one comes or goes. It is drawn in a child of `ui` over `rect`, at its
    /// bottom, and the y where the body above it ends is returned. A stat-block error is one
    /// of them: its whole fault wraps there, with its repair place under its words.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn message_footer(
        &mut self,
        ui: &mut egui::Ui,
        rect: Rect,
        setup: bool,
        command: &mut Option<Command>,
    ) -> f32 {
        use std::cell::Cell;
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::top_down(Align::LEFT)),
        );
        let save_new = Cell::new(false);
        let replace = Cell::new(false);
        let dismiss_error = Cell::new(false);
        let dismiss_unread = Cell::new(false);
        let use_saved = Cell::new(false);
        let undo_saved = Cell::new(false);
        let use_sheets = Cell::new(false);
        let disk = self.has_disk_conflict();
        let error = self.error.clone().filter(|_| !disk);
        let to_packages = Cell::new(false);
        // Astra 3 (FI11, AI1): a stat-block error says its whole fault, wrapped, with its repair
        // place as a bordered button under its words. Nothing is cut, so nothing hides in hover.
        let repair = error
            .as_deref()
            .filter(|error| error.contains(STAT_BLOCK_UNREAD))
            .map(|error| {
                let first = error
                    .split_once(" Its row")
                    .map_or(error, |(first, _)| first);
                format!("{first} Its row still works.")
            });
        let unread = !disk && self.undo_unread;
        let carry = setup.then(|| self.carry_missing()).flatten();
        let rest = setup && self.rest_filled_saved_hp();
        let used = setup && !rest && self.saved_hp_used();
        let back = (setup && !rest && !used)
            .then(|| self.back_to_sheets())
            .flatten();
        let back_words = back
            .as_ref()
            .map(|(from, _)| format!("Party HP carries over from {from}."));
        let recharged = self
            .recharge_result()
            .filter(|_| !setup)
            .map(|recharged| recharged.words.clone());
        // Both results belong to the newest step, so they take its order (issued when the step
        // was done, `newest_step_order`).
        let step_order = self.newest_step_order();
        // After the words its messages borrow, so it is dropped first.
        let mut footer = design::Footer::new(("run_messages", self.id), rect.width());
        if used && let Some(order) = step_order {
            // The result of the DM's own press, with its Undo: it leads the footer.
            footer.result(order, SAVED_HP_USED, true, |ui| {
                if design::undo_message(ui, Tone::Success, SAVED_HP_USED, "Undo use saved HP") {
                    undo_saved.set(true);
                }
            });
        }
        // X4: a recharge roll's result, until the next act. Its Undo is the header's.
        if let (Some(words), Some(order)) = (recharged.as_deref(), step_order) {
            footer.result(order, words, false, |ui| {
                design::message(ui, Tone::Info, words);
            });
        }
        if disk {
            // The run cannot save until the DM chooses, so this cannot be dismissed. The
            // choices start at the message text, like every other message with choices.
            footer.reason(super::DISK_CONFLICT, |ui| {
                let choice =
                    design::message_with_actions(ui, Tone::Error, super::DISK_CONFLICT, |ui| {
                        design::gap(ui, design::Gap::XS);
                        ui.horizontal(|ui| {
                            // Neither action is filled: the safe one comes first, and the red one
                            // keeps an `L` gap.
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
                save_new.set(choice.0);
                replace.set(choice.1);
            });
        }
        if let Some(error) = &error {
            // A stat-block error keeps its own name, apart from the sheet notice's.
            let close = if repair.is_some() {
                "Dismiss this error"
            } else {
                "Dismiss this message"
            };
            footer.reason(error.as_str(), |ui| {
                let dismissed = dismissable(ui, close, |ui| {
                    if let Some(words) = &repair {
                        // Top down, so the repair goes under the words.
                        let clicked = ui
                            .vertical(|ui| {
                                design::message_with_actions(ui, Tone::Error, words, |ui| {
                                    Action::secondary("Campaign → Packages")
                                        .name("Repair in Campaign → Packages")
                                        .density(Density::Read)
                                        .show(ui)
                                        .clicked()
                                })
                            })
                            .inner;
                        to_packages.set(clicked);
                    } else {
                        design::message(ui, Tone::Error, error);
                    }
                });
                dismiss_error.set(dismissed);
            });
        }
        // Said once: the Log keeps the line, and the next save leaves the steps out.
        if unread {
            footer.notice(super::UNDO_UNREAD, |ui| {
                let dismissed = dismissable(ui, "Dismiss the Undo message", |ui| {
                    design::message(ui, Tone::Warning, super::UNDO_UNREAD);
                });
                dismiss_unread.set(dismissed);
            });
        }
        if let Some(words) = &carry {
            footer.notice(words.as_str(), |ui| {
                design::message(ui, Tone::Warning, words);
            });
        }
        const REST_FULL: &str = "Party HP is full after the long rest.";
        if rest {
            footer.notice(REST_FULL, |ui| {
                if design::message_with_action(ui, Tone::Info, REST_FULL, "Use the saved HP") {
                    use_saved.set(true);
                }
            });
        }
        if let Some(words) = &back_words {
            footer.notice(words.as_str(), |ui| {
                if design::message_with_action(ui, Tone::Info, words, "Use the sheets’ HP") {
                    use_sheets.set(true);
                }
            });
        }
        let bottom = footer.show_by_hand(&mut child);
        let outcome = if save_new.get() {
            Some(self.save_as_new_run())
        } else if replace.get() {
            Some(self.keep_this_run())
        } else {
            None
        };
        if let Some(Err(error)) = outcome {
            self.error = Some(error);
        }
        if dismiss_error.get() {
            self.error = None;
        }
        if to_packages.get() {
            self.wants_packages = true;
        }
        if dismiss_unread.get() {
            self.undo_unread = false;
        }
        if use_saved.get()
            && let Err(e) = self.use_saved_hp()
        {
            self.error = Some(e);
        }
        if undo_saved.get() {
            self.undo();
        }
        if use_sheets.get()
            && let Some((_, back)) = back
        {
            *command = Some(back);
        }
        bottom
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

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
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
                    let room = ui.available_height()
                        - DOCK_HEIGHT
                        - self.strip_height()
                        - self.footer_height();
                    // FI2: notices that come by themselves sit in the message footer at the
                    // bottom of the roster's room, so the roster keeps its top.
                    let region = Rect::from_min_size(
                        ui.cursor().min,
                        vec2(ui.available_width(), room.max(0.0)),
                    );
                    let body_bottom = self.message_footer(ui, region, false, &mut None);
                    let footer_room = (region.bottom() - body_bottom).max(0.0);
                    let room = room - footer_room;
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
                            self.roster(ui, list, notice_height, footer_room);
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
                                // FJ4: the words get `text_room`; the message's bar, icon and
                                // their gaps take room before them. A fitted notice then keeps
                                // one line, and its band the height the list gave up.
                                let message_width =
                                    text_room + lead + ui.spacing().item_spacing.x * 2.0;
                                let response = ui
                                    .allocate_ui_with_layout(
                                        vec2(
                                            message_width,
                                            if compact || self.notice_summary.is_some() {
                                                Type::Small.line()
                                            } else {
                                                notice_height
                                            },
                                        ),
                                        Layout::left_to_right(Align::TOP),
                                        |ui| {
                                            // One form for every result: the tone marks
                                            // the icon and bar, the words keep text colour.
                                            let tone = if self.notice_summary.is_some() {
                                                Tone::Warning
                                            } else {
                                                Tone::Success
                                            };
                                            design::small_message(ui, tone, shown)
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
                    // Past the message footer, which the roster's room left free.
                    ui.add_space(footer_room);
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

    /// The sync state a row shows, if this participant has one.
    pub(super) fn sync_word(&self, index: usize) -> Option<SyncState> {
        self.syncs
            .get(&index)
            .map(crate::combat_sync::CombatSync::state)
    }

    pub(super) fn party_sync_off(&self) -> bool {
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

    /// A row marked Conc that the newest step damaged: the DC of its save, 10 or half the
    /// damage dealt if that is more, and at most 30. The step keeps its own damage in memory,
    /// so the DC lasts until the next step; the run file gains no field, and after a restart
    /// the row has no DC. Splat does not roll it.
    pub(super) fn conc_save_dc(&self, index: usize) -> Option<u32> {
        let p = self.state.participants.get(index)?;
        // The newest step's own damage to this row, as dealt: nothing is read from words.
        let damage = *self.state.damage.get(&index)?;
        (p.concentration && damage > 0 && self.history_from_sheet.last() != Some(&true))
            .then(|| (damage / 2).clamp(10, 30) as u32)
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
        let current = self.current();
        self.state
            .order
            .iter()
            .map(|&index| {
                let p = &self.state.participants[index];
                let row = (
                    self.row_sync_word(index),
                    self.conc_save_dc(index),
                    current == Some(index),
                );
                RowLayout::fitted(ui, width, p, row, scale, fit, longest)
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
    /// `hidden` is the room the message footer covers (FI2): the rows are sized as if it were
    /// free, so a footer that comes or goes moves no row; only the list's visible height
    /// shrinks, and the rows behind the footer scroll into view.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn roster(&mut self, ui: &mut egui::Ui, visible: f32, lent: f32, hidden: f32) {
        let room = visible + hidden;
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
        // Rows that reach behind the message footer are out of view too: the line that counts
        // them keeps its place inside the visible list.
        let behind = !overflow && total(&layouts, scale) > visible;
        let list_height = (if overflow || behind {
            room - OVERFLOW_LINE
        } else {
            room
        }) - hidden;
        let mut clicked = None;
        let mut row_choice = None;
        let mut rule = None;
        let mut toggle_half = None;
        let mut effect_act = None;
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
            // A list that fits its whole room keeps its columns: rows behind the message
            // footer scroll by wheel and by the turn's own scroll, with no bar over the HP.
            .scroll_bar_visibility(if overflow {
                egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded
            } else {
                egui::scroll_area::ScrollBarVisibility::AlwaysHidden
            })
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
                    // A row cut by the list's edge or by the message footer is not drawn: the
                    // list ends at a whole row, and `1 below` counts the rest. It keeps its place,
                    // and neither it nor any control of it is a target, a hover or a Tab stop.
                    let hidden = (overflow || behind) && !whole_row;
                    if hidden {
                        ui.set_clip_rect(Rect::NOTHING);
                    }
                    let selected = self.selected == index || self.also.contains(&index);
                    let p = &self.state.participants[index];
                    let sync = self.row_sync_word(index);
                    let save = self.save_on_row(index);
                    let badge = self.row_badge(index);
                    let turn = TurnMark::of(index, current, next_up);
                    let mut name = row_name(p, turn, sync);
                    let conc_dc = self
                        .conc_save_dc(index)
                        .map(|dc| format!("Conc save DC {dc}"));
                    if let Some(dc) = &conc_dc {
                        name.push_str(&format!(", {dc}"));
                    }
                    if let Some((ability, total)) = &save {
                        name.push_str(&format!(", {ability} save {total}"));
                    }
                    if let Some(badge) = &badge {
                        name.push_str(&format!(", {badge}"));
                    }
                    let response = ui.push_id(&p.key, |ui| {
                        if hidden {
                            ui.set_invisible();
                        }
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
                    // The whole words of the effects, and the save's DC, are in the hover.
                    let effects: Vec<String> = p
                        .conditions
                        .iter()
                        .filter(|c| c.is_effect())
                        .map(super::Condition::words)
                        .chain(conc_dc)
                        .collect();
                    let side = if effects.is_empty() {
                        p.side.clone()
                    } else {
                        format!("{}\n{}", p.side, effects.join(" · "))
                    };
                    let response = if layout.name_cut {
                        response.on_hover_text(format!("{}\n{side}", p.name))
                    } else {
                        response.on_hover_text(side)
                    };
                    ui.set_clip_rect(original_clip);
                    // Every other control of a hidden row (initiative, conditions, effects, the
                    // half-damage mark) is built in one part that is not shown, takes no press or
                    // hover, and is no Tab stop: one decision for the whole row.
                    let mut hidden_part = hidden.then(|| {
                        let mut part = ui
                            .new_child(egui::UiBuilder::new().max_rect(ui.max_rect()).invisible());
                        part.set_clip_rect(Rect::NOTHING);
                        part
                    });
                    let ui = match hidden_part.as_mut() {
                        Some(part) => part,
                        None => &mut *ui,
                    };
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
                            // The field's id is its owner's identity (this run, this
                            // participant), not its name: a rename or a row with the same
                            // name never takes over its text, cursor or selection.
                            let response = design::number_field_with_id(
                                ui,
                                egui::Id::new(("run_initiative", self.id, &p.key)),
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
                    // An ended run shows its effects as words: nothing counts, and a click
                    // does nothing.
                    if self.state.phase == Phase::Running
                        && let Some((act, _)) = effect_links(ui, rect, p, layout, scale)
                    {
                        clicked = Some((index, false));
                        effect_act = Some(match act {
                            EffectAct::Open => None,
                            EffectAct::Remove(at) => Some(Command::RemoveCondition(index, at)),
                            EffectAct::Spend(at) => Some(Command::Spend(index, at)),
                            EffectAct::Recharge(at) => Some(Command::Recharge(index, at)),
                        });
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
        match effect_act {
            Some(Some(command)) => self.run_command(command),
            Some(None) => self.open_conditions_tab(),
            None => {}
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

    pub(super) fn open_conditions_tab(&mut self) {
        self.panel = Panel::Conditions;
        self.close_lookup = true;
    }

    /// `E`: the dock's effect field for the selected row; when `Effect` has no room there, or
    /// several rows are selected, the Conditions tab with focus in its `Other effect…` field.
    pub(super) fn open_effect_field(&mut self) {
        if self.state.phase != Phase::Running {
            return;
        }
        if self.effect_in_dock && self.also.is_empty() {
            let key = self.state.participants[self.selected].key.clone();
            self.effect_fields.entry(key.clone()).or_default().focus = true;
            self.effect_open = Some(key);
        } else {
            // The Conditions tab's field shows the same draft, the participant's.
            self.open_conditions_tab();
            self.focus_other_effect = true;
        }
    }

    /// What was typed in this field beside Notes, when it cannot be a number.
    pub(super) fn words_in_number_field(
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

    /// The official conditions, one click each. A second click removes one. This sits
    /// beside the roster and needs no typing, so a mistyped name cannot lose its sync.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn conditions_tab(&mut self, ui: &mut egui::Ui, index: usize) {
        let mut command = None;
        let mut added = false;
        let p = &self.state.participants[index];
        let key = p.key.clone();
        let run_id = self.id;
        egui::ScrollArea::vertical()
            .id_salt("run_conditions")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // A form, not the pane: the grid, the effects and Add stay within its width.
                ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
                ui.spacing_mut().item_spacing = vec2(design::XS, design::XS);
                let found = |name: &str| {
                    p.conditions.iter().position(|c| {
                        c.same(name) || (name == "Exhaustion" && c.name().starts_with("Exhaustion"))
                    })
                };
                // Fixed columns, `S` apart: never spread across the form, never moved by a toggle.
                // X11: a condition with a count shows it on its tick, `✓ Frightened 10`.
                let label = |name: &str| {
                    found(name).map(|at| {
                        let c = &p.conditions[at];
                        c.count().map_or_else(|| name.to_owned(), |_| c.words())
                    })
                };
                let (pressed, counts) = conditions_grid_counted(ui, "official_conditions", &label);
                if let Some(name) = pressed {
                    command = Some(found(name).map_or_else(
                        || Command::Condition(index, name.to_owned()),
                        |i| Command::RemoveCondition(index, i),
                    ));
                }
                // One level, not a pile of lines. Each step replaces the last.
                let exhausted = p
                    .conditions
                    .iter()
                    .position(|c| c.name().starts_with("Exhaustion"));
                if let Some(at) = exhausted {
                    let start = exhaustion_level(p.conditions[at].name());
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
                            set_exhaustion(index, level as u8)
                        });
                    }
                }
                // Each other effect with its whole words. An official condition's count is on
                // its tick; it is here only when the tick cannot show it.
                let listed = |c: &super::Condition| {
                    super::recognized_condition(c.name()).is_none()
                        || (c.is_effect() && !(counts && c.count().is_some()))
                };
                let custom: Vec<_> = p
                    .conditions
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| listed(c))
                    .map(|(at, c)| (at, c.words()))
                    .collect();
                // The list's heading adds its own space above it; an empty list has no heading.
                if custom.is_empty() {
                    design::gap(ui, design::Gap::S);
                }
                let width = (ui.available_width() - 80.0).max(100.0);
                if let Some(at) =
                    other_effects_list(ui, custom.iter().map(|(_, words)| words.as_str()), width)
                {
                    command = Some(Command::RemoveCondition(index, custom[at].0));
                }
                let focus = std::mem::take(&mut self.focus_other_effect);
                // The participant's one draft: the dock's field shows the same words.
                let draft = self.effect_fields.entry(p.key.clone()).or_default();
                // X8: where the dock has no room for `Effect`, this field is the way, and its
                // words say that a time counts.
                let placeholder = if self.state.phase == Phase::Running && !self.effect_in_dock {
                    OTHER_EFFECT_TIMED
                } else {
                    "Other effect…"
                };
                // The draft is this participant's, in this run: its field keeps its own cursor
                // and Undo (two runs can reuse a roster slot's key).
                let owner = design::TextKey::from_data(format!("{}:{}", run_id, p.key));
                let (add, field) =
                    other_effect_field(ui, owner, &mut draft.words, (width, placeholder), focus);
                // Escape clears the participant's draft, as in the dock's field.
                if field.lost_focus() && ui.input(|i| i.key_pressed(Key::Escape)) {
                    draft.words.clear();
                }
                if add {
                    // Typed as a level, it is still the one Exhaustion a creature has.
                    // Words with a time go through the one reader, which keeps the level.
                    let typed = draft.words.trim().to_owned();
                    let level = typed.starts_with("Exhaustion")
                        && !super::Condition::read(&typed).is_effect();
                    command = Some(match exhausted.filter(|_| level) {
                        Some(_) => set_exhaustion(index, exhaustion_level(&typed)),
                        None => Command::Condition(index, typed),
                    });
                    added = true;
                }
                design::help(ui, CONDITIONS_HELP);
            });
        if let Some(command) = command {
            self.run_command(command);
            // Added words leave the draft; words the run refused stay, with the reason. In a
            // fight, focus goes back to Amount, as after the dock's field.
            if added && self.error.is_none() {
                self.effect_fields.remove(&key);
                self.focus_amount = self.state.phase == Phase::Running;
            }
        }
    }

    pub(super) fn sync_needs_sign_in(&self) -> bool {
        !self.connected && !self.simulate_connection && !self.linked_participants().is_empty()
    }

    fn footer_height(&self) -> f32 {
        FOOTER_HEIGHT
    }

    /// Sync and the closing controls. These are kept far from Next turn.
    fn live_footer(&mut self, ui: &mut egui::Ui) -> bool {
        let mut closed = false;
        design::gap(ui, design::Gap::XS);
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Read.control());
            if self.ending {
                let field = design::line_field(
                    ui,
                    OUTCOME_FIELD.of(self.id),
                    "Outcome",
                    &mut self.outcome,
                    "Outcome, if any",
                    160.0,
                );
                if std::mem::take(&mut self.focus_outcome) {
                    field.request_focus();
                    // An outcome that is already there is selected, so that Enter keeps it
                    // and typing replaces it.
                    let mut text =
                        egui::TextEdit::load_state(ui.ctx(), field.id).unwrap_or_default();
                    text.cursor
                        .set_char_range(Some(egui::text::CCursorRange::two(
                            egui::text::CCursor::new(0),
                            egui::text::CCursor::new(self.outcome.chars().count()),
                        )));
                    text.store(ui.ctx(), field.id);
                }
                let enter = field.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter));
                if Action::destructive("End encounter")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                    || enter
                {
                    let outcome = if self.outcome.trim().is_empty() {
                        super::NO_OUTCOME.to_owned()
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
                // FK7 (iter-27-R): the dialog takes the input, and the line behind it stays
                // as it was, with its save state, so the page behind a dialog does not change.
                let note = close_saved_words(self.syncs.values().any(|sync| sync.enabled));
                closed = self.close_confirmation(ui, &note);
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
                let menu = design::menu_button(ui, "Close", false);
                design::named(ui, &menu, "Close menu");
                design::menu_popup(&menu).show(|ui| controls(ui));
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
            // The End step opens. With no outcome typed yet, it starts from the outcome of the
            // ending that Continue fight took away.
            if ending
                && self.outcome.is_empty()
                && let Some(outcome) = self.outcome_before_continue()
            {
                self.outcome = outcome.to_owned();
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
    /// Why a close waits, as one exact sentence: `Settle 2 conflicts and 1 pending change
    /// first.` The ended view and the close dialog both say it, so one block has one form.
    fn close_reason(&self) -> Option<String> {
        self.close_block().map(|general| {
            let conflicts = self.conflict_count();
            let pending = self
                .syncs
                .values()
                .filter(|sync| {
                    sync.unconfirmed() && sync.state() != SyncState::Conflict && !sync.stranded()
                })
                .count();
            let mut parts = Vec::new();
            if conflicts > 0 {
                parts.push(design::count(conflicts, "conflict", "conflicts"));
            }
            if pending > 0 {
                parts.push(design::count(pending, "pending change", "pending changes"));
            }
            if parts.is_empty() {
                general.to_owned()
            } else {
                format!("Settle {} first.", parts.join(" and "))
            }
        })
    }

    fn close_confirmation(&mut self, ui: &mut egui::Ui, note: &str) -> bool {
        self.close_dialog(ui, note)
    }

    /// All close paths share the same safe default and separate loss from saving.
    fn close_dialog(&mut self, ui: &egui::Ui, note: &str) -> bool {
        let cannot_save = self.strip().not_saved;
        let stranded = self.stranded_names();
        let lost = self.unsent_only_here_names();
        let block = self.close_reason();
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
                if let Some(words) = conflict_close_words(block.is_some()) {
                    ui.add(egui::Label::new(words).wrap());
                }
            } else if stranded.is_empty() {
                ui.add(egui::Label::new(note).wrap());
            } else {
                ui.add(egui::Label::new(format!("Changes for {} were not sent to D&D Beyond. The saved run keeps HP, death saves and conditions for the next sync.", stranded.join(", "))).wrap());
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
            design::gap(ui, design::Gap::S);
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
            // Every close action here waits for the same cause. Round 27 FN4: it shows once,
            // `S` left of the blocked close on the right, or after the red one when that is
            // the only close.
            let right_close = cannot_save || lost.is_empty();
            let (lose, ()) = design::dialog_actions(
                ui,
                |ui| {
                    let lose = (cannot_save || !lost.is_empty())
                        && Action::destructive("Close and lose changes")
                            .blocked(block.as_deref())
                            .show(ui)
                            .clicked();
                    if !right_close {
                        design::blocked_reason(ui, block.as_deref());
                    }
                    lose
                },
                |ui| {
                    ui.spacing_mut().item_spacing.x = design::S;
                    if cannot_save {
                        save_new = Action::primary("Save as a new run and close")
                            .blocked(block.as_deref())
                            .show(ui)
                            .clicked();
                    } else if lost.is_empty() {
                        close |= Action::secondary("Close now")
                            .blocked(block.as_deref())
                            .show(ui)
                            .clicked();
                    }
                    if right_close && block.is_some() {
                        design::blocked_reason(ui, block.as_deref());
                        design::gap(ui, design::Gap::S);
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
            .enumerate()
            .filter(|(_, (state, _))| state.log.len() >= saved_lines)
            .map(|(at, (_, label))| {
                let target = self.history_targets.get(at).and_then(Option::as_deref);
                Self::change_with_result(&self.state, label, target)
            })
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
        design::heading(ui, design::Heading::Label("D&D Beyond sync"));
        // FL4: `Choose which values to keep` only when a block offers values to choose.
        let any_conflict = names.iter().any(|(index, _)| {
            self.syncs
                .get(index)
                .is_some_and(|sync| sync.state() == SyncState::Conflict)
        });
        ui.add(
            egui::Label::new(
                Type::Small
                    .rich(
                        ui,
                        if any_conflict {
                            "Splat and D&D Beyond differ. Choose which values to keep."
                        } else {
                            "Splat and D&D Beyond differ."
                        },
                    )
                    .color(colors().muted),
            )
            .wrap(),
        );
        for (index, name) in names {
            design::gap(ui, design::Gap::L);
            let Some(sync) = self.syncs.get_mut(&index) else {
                continue;
            };
            let conflict = sync.state() == SyncState::Conflict;
            ui.push_id(index, |ui| {
                if conflict {
                    sync.conflicts_for(ui, &name);
                } else {
                    let status = sync_status(sync.state());
                    crate::combat_sync::sync_heading(
                        ui,
                        &name,
                        &format!("{} {}", status.icon, status.word),
                        status.color,
                    );
                    if sync.state() == SyncState::Pending {
                        // FL4: say what waits, that it needs no step, and the way to close now.
                        design::help(ui, PENDING_WORDS);
                    }
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
        let title = if matches!(
            self.panel,
            Panel::Log | Panel::Encounter | Panel::Groups | Panel::Keys
        ) {
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

        let participant_tab = !matches!(
            self.panel,
            Panel::Log | Panel::Encounter | Panel::Groups | Panel::Keys
        );
        let reference = if is_sheet { "Sheet" } else { "Stat block" };
        self.panel_tabs(ui, Some(reference), true);
        design::rule(ui);
        // FJ5: one place on every tab, directly below the tab row, so the counts and their
        // actions do not move when the DM changes tab. Sheet's status line follows them.
        if death_saves && participant_tab {
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
    pub(super) fn add_to_fight(&self, words: bool, density: Density) -> Action<'static> {
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
            // FE14: the title names what the panel shows, in the form of the other fight panels.
            Panel::Keys => "This fight · Keys",
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
            design::gap(ui, design::Gap::L);
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
                    design::menu_popup(&button).show(|ui| {
                        for (panel, label) in fight {
                            if design::choice(ui, self.panel == panel, label).clicked() {
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

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
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
                    design::heading(ui, design::Heading::Label(READ_ALOUD));
                    style::markdown(ui, prep);
                }
                if let Some(technical) = technical {
                    design::gap(ui, design::Gap::M);
                    design::quiet_section(ui, "Technical details", |ui| {
                        style::markdown(ui, technical);
                    });
                }
            });
    }

    /// The campaign's encounter groups. A group is dragged to the initiative list, or added
    /// with its button, by the rule the campaign chose for initiative.
    /// Whatever joins a fight after it starts: one creature, or a prepared group.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state`, and raw egui calls (design::results::STAGED_ALLOWS)"
    )]
    fn groups_tab(&mut self, ui: &mut egui::Ui) {
        let mut add = None;
        // FB1: the newest arrival's result, and a tie's Move button, sit in a footer pinned
        // to the panel's bottom, below the scrolling list. The list keeps its top, so a result
        // that comes, grows with a tie, or goes moves no group row. Its owner is the newest
        // step: only a new step or Undo ends it. FJ2: typed search words do not, because the
        // footer takes no room from the search results and the DM often types the next arrival
        // while a tie's Move choice is still open.
        // FI5: a typed total that cannot be read gives its reason here too, after the result,
        // named by its group. The reason belongs to the typed words: it ends when they change.
        let reasons = self.arrival_reasons();
        let result = self.arrival_result_shown();
        // The shared message footer: as tall as what it holds, at most a third of the panel,
        // and it scrolls inside beyond that (pointer and keyboard), so ten bad totals at 960
        // neither clip nor push the list. The result's Undo is live in the header while its
        // step is the newest, so it leads; the reasons follow.
        // The message names itself; the footer's own name never takes the Log's words.
        let words = String::new();
        // The result belongs to the newest step, so it takes that step's order.
        let order = self.newest_step_order();
        let mut footer = design::Footer::new(("arrival_footer", self.id), ui.available_width());
        if result && let Some(order) = order {
            footer.result(order, words, true, |ui| self.arrival_result(ui));
        }
        for reason in reasons.iter() {
            footer.reason(reason.as_str(), move |ui| {
                // The group faults' form: a mark, and words in text colour.
                design::small_message(ui, Tone::Warning, reason);
            });
        }
        let body_bottom = footer.show_by_hand(ui);
        // FJ1: the list keeps the scroll bar's lane whether or not the bar shows, as the
        // Encounters list does, so a list that grows long moves no field and no Add.
        let lane = (ui.available_width() - ui.spacing().scroll.allocated_width()).max(0.0);
        egui::ScrollArea::vertical()
            .id_salt("run_groups")
            .auto_shrink([false, false])
            .max_height((body_bottom - ui.cursor().top()).max(0.0))
            .show(ui, |ui| {
                ui.set_max_width(lane);
                ui.spacing_mut().item_spacing.y = design::XS;
                if self
                    .groups
                    .as_ref()
                    .is_some_and(|groups| !groups.is_empty())
                {
                    design::help(ui, "Drag a group name to the list, or use Add.");
                }
                // Measured once: the search field and the group list share the block.
                let arrival = self.arrival_block(ui);
                self.add_creature_form(ui, arrival.map(|(_, block)| block));
                design::rule(ui);
                add = self.group_list(ui, arrival);
            });
        if let Some(id) = add {
            self.add_group_from_panel(id);
        }
    }

    /// The revision of the groups' read: it changes whenever `groups` holds another read than
    /// the one counted last, whatever page was open when it was replaced.
    fn groups_revision(&mut self) -> u64 {
        let same = match (&self.groups, &self.groups_seen) {
            (Some(now), Some(seen)) => Arc::ptr_eq(now, seen),
            (None, None) => true,
            _ => false,
        };
        if !same {
            self.groups_revision += 1;
            self.groups_seen = self.groups.clone();
        }
        self.groups_revision
    }

    /// The revision of the scale's read, as `groups_revision` is for the groups.
    fn scale_revision(&mut self) -> u64 {
        let same = match (&self.scale, &self.scale_seen) {
            (Some(now), Some(seen)) => Arc::ptr_eq(now, seen),
            (None, None) => true,
            _ => false,
        };
        if !same {
            self.scale_revision += 1;
            self.scale_seen = self.scale.clone();
        }
        self.scale_revision
    }

    /// Each offered group whose typed total cannot be read: `<group>: <reason>`, in list order.
    /// Worked out again only after an edit of the typed totals, a new read of the groups or
    /// another encounter (`totals_revision`, the groups' address, `source`).
    fn arrival_reasons(&mut self) -> Arc<[String]> {
        let groups_revision = self.groups_revision();
        let Some(groups) = self.groups.as_ref() else {
            return Arc::from([]);
        };
        let key = (self.totals_revision, groups_revision, self.source);
        if let Some((_, reasons)) = self.arrival_reasons.as_ref().filter(|(at, _)| *at == key) {
            return reasons.clone();
        }
        #[cfg(test)]
        {
            let (a, b) = PREPARED.with(std::cell::Cell::get);
            PREPARED.with(|count| count.set((a, b + 1)));
        }
        let offered = super::OfferedGroups::of(groups, self.source);
        let reasons: Arc<[String]> = offered
            .own
            .iter()
            .chain(&offered.any)
            .chain(&offered.other)
            .filter_map(|group| {
                let text = self.group_totals.get(&group.id)?;
                let reason = arrival_total(text).err()?;
                Some(format!("{}: {reason}", group.name))
            })
            .collect();
        self.arrival_reasons = Some((key, reasons.clone()));
        reasons
    }

    /// FI8: a group's member summary for its place, wrapped beside Initiative and Add, or on a
    /// full-width line above them when `Will add:` and its first member do not fit beside them.
    /// Worked out once for each read of the groups, width and type size; a frame that changes
    /// none of them reads the kept words.
    fn arrival_summary(
        &mut self,
        ui: &egui::Ui,
        group: &super::RunGroup,
        beside: f32,
        full: f32,
    ) -> super::ArrivalSummary {
        let font = Type::Small.font(ui);
        let key = (
            self.groups_revision(),
            beside.to_bits(),
            full.to_bits(),
            font.size.to_bits(),
        );
        if self.arrival_summaries.as_ref().map(|(at, _)| *at) != Some(key) {
            self.arrival_summaries = Some((key, std::collections::BTreeMap::new()));
        }
        let Some((_, kept)) = self.arrival_summaries.as_mut() else {
            unreachable!("set just above");
        };
        if let Some(summary) = kept.get(&group.id) {
            return summary.clone();
        }
        #[cfg(test)]
        {
            let (a, b) = PREPARED.with(std::cell::Cell::get);
            PREPARED.with(|count| count.set((a + 1, b)));
        }
        let (words, alone) = group.member_summary_fit(ui, &font, beside);
        let summary = if alone {
            super::ArrivalSummary {
                members: group.member_summary_wrapped(ui, &font, full).into(),
                above: true,
            }
        } else {
            super::ArrivalSummary {
                members: words.into(),
                above: false,
            }
        };
        kept.insert(group.id, summary.clone());
        summary
    }

    /// What each group would make the fight when it joins, for the groups whose XP is known. The
    /// party is the characters who are present; the fight is every present creature against them.
    /// Worked out again only after a step, a new read of the groups or of the scale.
    pub(super) fn arrival_words(&mut self) -> Option<&super::ArrivalWords> {
        let key = (self.revision, self.groups_revision(), self.scale_revision());
        let (groups, scale) = (self.groups.as_ref()?, self.scale.as_ref()?);
        #[cfg(test)]
        {
            self.arrival_counts.0 += 1;
        }
        if self.arrival_words.as_ref().map(|(at, _)| *at) != Some(key) {
            #[cfg(test)]
            {
                self.arrival_counts.1 += 1;
            }
            let (mut party, mut foes) = (Vec::new(), Vec::new());
            let mut characters = false;
            for p in &self.state.participants {
                if p.side.trim() == "party" {
                    let level = scale.level(&p.definition);
                    characters |= level.is_some();
                    if p.present {
                        party.extend(level);
                    }
                } else if p.present {
                    foes.push(crate::difficulty::creature_xp(&p.definition));
                }
            }
            // The campaign's party stands in only for a fight with no character at all. When
            // every character is absent, there is no party to weigh against.
            let nobody = characters && party.is_empty();
            let words = groups
                .iter()
                .filter_map(|group| {
                    // Each member joins on its own side; one on the party's side is an ally.
                    let added: Vec<_> = group
                        .members
                        .iter()
                        .filter(|member| {
                            member
                                .overrides
                                .side
                                .as_deref()
                                .unwrap_or(&group.side)
                                .trim()
                                != "party"
                        })
                        .flat_map(|member| {
                            let xp = crate::difficulty::creature_xp(&member.entry);
                            std::iter::repeat_n(xp, member.count as usize)
                        })
                        .collect();
                    if added.is_empty() || nobody {
                        return None;
                    }
                    let adds = added.iter().map(|xp| xp.unwrap_or(0)).sum();
                    let fight = scale.weigh(&party, foes.iter().copied().chain(added));
                    Some((group.id, (fight.rating?, adds)))
                })
                .collect();
            self.arrival_words = Some((key, words));
        }
        self.arrival_words.as_ref().map(|(_, words)| words)
    }

    /// Whether `arrival_result` has a result to show: the newest step added an arrival, or it
    /// settled a tie.
    fn arrival_result_shown(&self) -> bool {
        let last = self.state.log.last();
        (!self.history.is_empty()
            && last.is_some_and(|line| {
                line.starts_with(super::GROUP_ADDED) || line.starts_with(super::ONE_ADDED)
            }))
            || last.is_some_and(|line| line.contains(" moved before "))
    }

    /// The newest arrival's result: who joined and at what total, with `Move … before …` when
    /// it tied, or the move that settled a tie. `arrival_result_shown` says when it draws.
    fn arrival_result(&mut self, ui: &mut egui::Ui) {
        if let Some(previous) = self.history.last().filter(|_| {
            self.state.log.last().is_some_and(|line| {
                line.starts_with(super::GROUP_ADDED) || line.starts_with(super::ONE_ADDED)
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
                let name = Self::roll_block_name(&self.state, &self.state.participants[before]);
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
                // The name the `joined at` message uses, so one arrival has one name.
                let arrival = Self::arrival_name(&self.state, first, &name);
                let full = format!("Move {arrival} before {name}");
                let font = Type::Strong.font(ui);
                // A finished change, tie or not: the Move button shows the open choice.
                design::message(ui, Tone::Success, &line);
                design::message_choice_row(ui, Tone::Success, |ui| {
                    let room = ui.available_width() - design::M * 2.0;
                    let shown = move_words(ui, &arrival, &name, &font, room).unwrap_or_else(|| {
                        undo_words(
                            ui,
                            (&full, None),
                            false,
                            &self.state.participants,
                            &font,
                            room,
                        )
                    });
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
    }

    /// A group's side, size and worth: `Enemies · 3 members · XP 150`.
    fn group_totals_text(group: &super::RunGroup) -> String {
        format!(
            "{} · {} · {}",
            style::human_label(&group.side),
            group.count_label(),
            group.xp_label()
        )
    }

    /// The roster shows members; this notice keeps just the group and its totals.
    /// Roll details and member names remain in the unmodified Log.
    fn group_arrival_notice(&self, start: usize) -> Option<String> {
        let added = self.state.participants.get(start..)?;
        let origin = added.first()?.added_from.as_ref()?;
        // A creature added by hand has no group: the notice names the rows it added, in the
        // group's words. `entered by the DM` stays in the Log, which is not spoken to the DM.
        let who = if origin.group.is_empty() {
            let names: Vec<_> = added.iter().map(|p| p.name.as_str()).collect();
            match names.as_slice() {
                [] => return None,
                [one] => (*one).to_owned(),
                [first @ .., last] => format!("{} and {last}", first.join(", ")),
            }
        } else {
            origin.group.clone()
        };
        let totals: std::collections::BTreeSet<_> =
            added.iter().filter_map(|p| p.initiative).collect();
        let totals = totals
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        Some(format!("{who} joined at\u{a0}{totals}."))
    }

    /// Arrivals' one block for every group: as wide as its widest heading or summary with
    /// Initiative and Add, and no wider than a list, so the controls stay near the names they
    /// add, at one x in every group. Also the width of the Add slot. The search field above
    /// takes the block's width, so the panel has one right edge. None without groups.
    fn arrival_block(&self, ui: &egui::Ui) -> Option<(f32, f32)> {
        let groups = self.groups.as_ref()?;
        let offered = super::OfferedGroups::of(groups, self.source);
        let all: Vec<_> = offered
            .own
            .iter()
            .chain(&offered.any)
            .chain(&offered.other)
            .collect();
        if all.is_empty() {
            return None;
        }
        let add_width = all
            .iter()
            .map(|group| {
                Action::secondary(&arrival_add_word(group))
                    .density(Density::Read)
                    .natural_width(ui)
            })
            .fold(0.0, f32::max);
        // One block for every group, as wide as its widest heading or summary with Initiative
        // and Add, and no wider than a list: the controls stay near the names they add, at one
        // x in every group.
        let gap = ui.spacing().item_spacing.x;
        let controls = design::number_width(ui, "Initiative", &Type::Small.font(ui))
            + gap
            + ARRIVAL_FIELD_WIDTH
            + design::XS
            + gap
            + add_width;
        let block = all
            .iter()
            .map(|group| {
                let status = self.group_status(group, self.group_joined(group));
                let totals = Self::group_totals_text(group);
                let heading = design::number_width(
                    ui,
                    &format!("{} {}", icons::DRAG, group.name),
                    &Type::Strong.font(ui),
                ) + design::L
                    + design::number_width(
                        ui,
                        &if status.is_empty() {
                            totals
                        } else {
                            format!("{status} · {totals}")
                        },
                        &Type::Small.font(ui),
                    );
                let summary =
                    design::number_width(ui, &group.member_summary(), &Type::Small.font(ui))
                        + gap
                        + controls;
                heading.max(summary)
            })
            .fold(0.0, f32::max)
            .min(design::LIST_MEASURE)
            .min(ui.available_width());
        Some((add_width, block))
    }

    /// Returns the group the DM chooses to add. Those in tonight's plan come first.
    fn group_list(&mut self, ui: &mut egui::Ui, arrival: Option<(f32, f32)>) -> Option<Uuid> {
        let c = colors();
        let Some(groups) = self.groups.clone() else {
            design::heading(
                ui,
                design::Heading::Group {
                    title: "Encounter groups",
                    count: 0,
                },
            );
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
            design::heading(
                ui,
                design::Heading::Group {
                    title: "Encounter groups",
                    count: 0,
                },
            );
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    Type::Small
                        .rich(ui, "No groups yet. Make one on Encounters with")
                        .color(c.muted),
                );
                // FJ7: one sentence in one size, and its full stop follows the link's last
                // word. The link and its full stop wrap together.
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    if design::small_link(ui, "New group").clicked() {
                        self.wants_new_group = true;
                    }
                    ui.label(Type::Small.rich(ui, ".").color(c.muted));
                });
            });
            return None;
        }
        // SI2: `groups_tab` draws the one rule above the groups.
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
            design::heading(
                ui,
                design::Heading::Group {
                    title: "Encounter groups",
                    count: offered.any.len(),
                },
            );
        }
        let (add_width, block) = arrival.unwrap_or_default();
        // A child of its own: the pane's earlier rows already make this Ui as wide as the pane,
        // and a Ui cannot shrink below what it holds. A group's faults may use a list's width,
        // so a repair link stays on its fault's line; its name, members and controls keep to
        // the block.
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
            let mut add = None;
            for (heading, list) in [
                ("For this encounter", offered.own.as_slice()),
                ("In tonight’s plan", planned.as_slice()),
                ("For any encounter", any.as_slice()),
                ("For one encounter", offered.other.as_slice()),
            ] {
                if labelled && !list.is_empty() {
                    design::heading(
                        ui,
                        design::Heading::Group {
                            title: heading,
                            count: list.len(),
                        },
                    );
                }
                if heading == "For one encounter" && !list.is_empty() {
                    design::help(
                        ui,
                        "This run does not say which encounter it came from, so the groups of every encounter show.",
                    );
                }
                for group in list {
                    let row = ui.push_id(group.id, |ui| {
                        if self.group_row(ui, group, add_width, block) {
                            add = Some(group.id);
                        }
                    });
                    design::register_row(ui, row.response.rect);
                    design::rule(ui);
                }
            }
            add
        })
        .inner
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

    /// The newest round this group joined in (0 for setup), from one scan of the participants
    /// and no list. Its row's status and its layout both take it from here.
    fn group_joined(&self, group: &super::RunGroup) -> Option<u64> {
        #[cfg(test)]
        GROUP_SCANS.with(|count| count.set(count.get() + 1));
        self.state
            .participants
            .iter()
            .filter_map(|p| p.added_from.as_ref())
            .filter(|from| from.group_id == Some(group.id))
            .map(|from| from.round)
            .max()
    }

    /// What a group's row says about it before its members: when it joined (`joined`, from
    /// `group_joined`), whose encounter it is for, and `Changed`.
    fn group_status(&self, group: &super::RunGroup, joined: Option<u64>) -> String {
        let changed = group
            .members
            .iter()
            .any(|member| member.overrides.changes_creature());
        let mut status = if let Some(round) = joined {
            if round == 0 {
                // Short enough for the member slot at 960, in the run's own phase word, so a
                // first join in setup needs no line of its own (review 2).
                "Added in setup".to_owned()
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
            // FB10: the Groups list's words, short enough to sit beside the name.
            crate::encounters::ENCOUNTER_DELETED.to_owned()
        } else {
            String::new()
        };
        if changed {
            if !status.is_empty() {
                status.push_str(" · ");
            }
            status.push_str(&format!("{} Changed", icons::CHANGED));
        }
        status
    }

    /// Returns true when the DM chooses to add this group.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn group_row(
        &mut self,
        ui: &mut egui::Ui,
        group: &super::RunGroup,
        add_width: f32,
        block: f32,
    ) -> bool {
        let c = colors();
        let name = &group.name;
        let changed = group
            .members
            .iter()
            .any(|member| member.overrides.changes_creature());
        let change_hint = if changed { " Changed." } else { "" };
        let joined_round = self.group_joined(group);
        let status = self.group_status(group, joined_round);
        let rating = self
            .arrival_words()
            .and_then(|words| words.get(&group.id))
            .cloned();
        let (rating, adds) = rating.map_or((None, 0), |(rating, adds)| (Some(rating), adds));
        let grip = ui.allocate_response(
            vec2(block.min(ui.available_width()), Density::Read.control()),
            egui::Sense::DRAG,
        );
        let totals_text = Self::group_totals_text(group);
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
        // F1, FI10: what the fight becomes when the group joins, after the group's own XP, in
        // the one difficulty form: ` · Fight after: XP 450 · Low`. Only where the totals and
        // the status words are in the heading and stay there with it.
        let word = rating
            .as_ref()
            .map(|rating| format!(" · {FIGHT_AFTER}{}", rating.form(false)));
        let word = word.filter(|word| {
            let line = if status.is_empty() {
                format!("{totals_text}{word}")
            } else {
                format!("{status} · {totals_text}{word}")
            };
            totals_in_heading
                && (status.is_empty() || status_in_heading)
                && heading_width
                    + design::L
                    + design::number_width(ui, &line, &Type::Small.font(ui))
                    <= grip.rect.width()
        });
        let shown = rating.as_ref().filter(|_| word.is_some());
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
        if let Some(rating) = shown {
            let words = format!(
                "Adds {} XP. {FIGHT_AFTER}{}.",
                crate::encounter_run::grouped_number(adds),
                rating.form(true)
            );
            ui.ctx()
                .accesskit_node_builder(grip.id, |node| node.set_description(words));
        }
        // The word in the text colour; the rest muted, as every difficulty line.
        let mut job = egui::text::LayoutJob::default();
        if let Some(rating) = shown {
            crate::difficulty::append(
                &mut job,
                ui,
                Type::Small,
                &format!(" · {FIGHT_AFTER}"),
                c.muted,
            );
            crate::difficulty::append_form(&mut job, ui, Type::Small, &rating.parts(false));
        }
        let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
        let word_rect = Align2::RIGHT_CENTER.anchor_size(grip.rect.right_center(), galley.size());
        ui.painter().galley(word_rect.min, galley, c.text);
        let totals = ui.painter().text(
            pos2(word_rect.left(), grip.rect.center().y),
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
            "{name}.{change_hint} {context}{} {}{}",
            group.notes.trim(),
            self.initiative.grouping.arrival_rule(),
            shown.map_or_else(String::new, |rating| format!("\n{}", rating.budget_line()))
        ));
        // A group that cannot join carries no payload; its ghost still follows the pointer.
        let movable = group.problem.is_none()
            && arrival_total(self.group_totals.get(&group.id).map_or("", String::as_str)).is_ok();
        design::drag_source(
            ui,
            &grip,
            movable.then_some(GroupDrag(group.id)),
            design::Ghost::new(icons::ADD, name),
        );
        if !totals_in_heading {
            ui.add(egui::Label::new(Type::Small.rich(ui, &totals_text).color(c.muted)).wrap());
        }
        let mut add = false;
        let add_word = arrival_add_word(group);
        // Initiative and Add end at the group block's right edge in every group, so they share
        // one column. The summary takes what is left.
        let right = ui.max_rect().left() + block.min(ui.available_width());
        let gap = ui.spacing().item_spacing.x;
        // The field's margins draw it a little wider than it asks for; XS covers that, and
        // the space before Add takes up whatever is left, so Add ends at the edge.
        let controls = design::number_width(ui, "Initiative", &Type::Small.font(ui))
            + gap
            + ARRIVAL_FIELD_WIDTH
            + design::XS
            + gap
            + add_width;
        let beside = (right - ui.cursor().left() - controls - gap).max(design::XXL);
        // FI8: when `Will add:` and its first member do not fit beside Initiative and Add, the
        // summary takes a full-width line above them, so `Will add:` never stands alone.
        let full = (right - ui.cursor().left()).max(design::XXL);
        let super::ArrivalSummary {
            members,
            above: summary_above,
        } = self.arrival_summary(ui, group, beside, full);
        let width = if summary_above { full } else { beside };
        let members = members.to_string();
        let status_shown = if status_in_heading {
            ""
        } else {
            status.as_str()
        };
        // A status that cannot fit the member slot whole takes a full-width line above the
        // members and controls, where it wraps at its spaces. FB1: a group that has joined
        // decides by its widest join status too, so a second join that changes the words never
        // changes the row's shape.
        // A group that joined can show `Added in round N` again; its row keeps that room.
        let widest = format!(
            "Added in round {}{}",
            self.state.round.max(10),
            if changed {
                format!(" · {} Changed", icons::CHANGED)
            } else {
                String::new()
            }
        );
        let widest_in_heading = totals_in_heading
            && heading_width
                + design::L
                + design::number_width(
                    ui,
                    &format!("{widest} · {totals_text}"),
                    &Type::Small.font(ui),
                )
                <= grip.rect.width();
        // FI5: any group can join, so every row keeps the shape of the widest words it can show,
        // `Added in round NN` (or its own longer status). Joining then moves no row, and its
        // `Added in round N` sits on the members' line.
        let font = Type::Small.font(ui);
        let shape = if widest_in_heading
            || design::number_width(ui, status_shown, &font)
                > design::number_width(ui, &widest, &font)
        {
            status_shown
        } else {
            widest.as_str()
        };
        // FI8: a summary above the controls leaves their line's left part free: the status goes
        // there, never on a line of its own.
        let status_above = !summary_above
            && (arrival_summary(ui, status_shown, &members, width).is_none()
                || arrival_summary(ui, shape, &members, width).is_none());
        if status_above {
            let words = if status_shown.is_empty() {
                " "
            } else {
                status_shown
            };
            ui.add(egui::Label::new(Type::Small.rich(ui, words).color(c.muted)).wrap());
        }
        // FB1: room for a status line and a members line, so a group that joins (and gains
        // `Added in round 1`) moves no row below it.
        let two_lines = ui
            .fonts_mut(|fonts| {
                fonts.layout_no_wrap(
                    "Ag\nAg".into(),
                    Type::Small.font(ui),
                    egui::Color32::PLACEHOLDER,
                )
            })
            .size()
            .y;
        let layout_height = |ui: &egui::Ui, words: String, width: f32| {
            let job = egui::text::LayoutJob::simple(
                words,
                font.clone(),
                egui::Color32::PLACEHOLDER,
                width,
            );
            ui.fonts_mut(|fonts| fonts.layout_job(job)).size().y
        };
        // The lines the widest status would take with the members (beside the controls), or
        // alone (left of the controls, under a summary above them), so a join keeps the height.
        let shape_height = if status_above {
            0.0
        } else if summary_above {
            layout_height(ui, shape.to_owned(), beside)
        } else {
            arrival_summary(ui, shape, &members, width)
                .map_or(0.0, |words| layout_height(ui, words, width))
        };
        let summary_name = if status.is_empty() || status_above || summary_above {
            members.clone()
        } else {
            format!("{status} · {members}")
        };
        if summary_above {
            let response =
                ui.add(egui::Label::new(Type::Small.rich(ui, &members).color(c.muted)).wrap());
            design::named(ui, &response, &summary_name);
        }
        ui.horizontal(|ui| {
            let cell = if summary_above { beside } else { width };
            ui.allocate_ui(vec2(cell, Density::Read.control()), |ui| {
                ui.set_min_width(cell);
                ui.set_max_width(cell);
                if summary_above {
                    ui.set_min_height(shape_height.max(Density::Read.control()));
                    if !status_shown.is_empty() {
                        ui.add(
                            egui::Label::new(Type::Small.rich(ui, status_shown).color(c.muted))
                                .wrap(),
                        );
                    }
                    return;
                }
                ui.set_min_height(two_lines.max(shape_height).max(Density::Read.control()));
                let summary = arrival_summary(
                    ui,
                    if status_above { "" } else { status_shown },
                    &members,
                    width,
                )
                .unwrap_or_default();
                let response =
                    ui.add(egui::Label::new(Type::Small.rich(ui, &summary).color(c.muted)).wrap());
                design::named(ui, &response, &summary_name);
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
            // FI5: the reason's footer can take room from the list. While the field has focus
            // and its words cannot be read, a field that the footer hides scrolls into view.
            if response.has_focus()
                && arrival_total(text).is_err()
                && !ui.clip_rect().contains_rect(response.rect)
            {
                response.scroll_to_me(None);
            }
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_description("Blank rolls initiative")
            });
            let parsed = arrival_total(text);
            if response.changed() {
                self.totals_revision += 1;
            }
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
        // FI5: the total's reason is in the footer below the list (`groups_tab`), so it moves
        // no row when it comes or goes.
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
                // The one member-fault row (Contract B, R): the Encounters list draws it too.
                if crate::encounters::member_fault_row(ui, fault) {
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
                // The faults' actions start at the message words; this one does too.
                && design::message_choice_row(ui, Tone::Warning, |ui| {
                    Action::secondary("Edit group")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                })
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
            self.totals_revision += 1;
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
        // The panel's title names them (FE14), so the list has no label of its own.
        design::gap(ui, design::Gap::XS);
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
        design::gap(ui, design::Gap::XS);
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
        design::heading(
            ui,
            design::Heading::Group {
                title: "Death saves",
                count: 0,
            },
        );
        let mut command = None;
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Read.control());
            ui.spacing_mut().item_spacing.x = design::XS;
            // FJ1 (iter-27-R): the counts have the slot of their widest form, so a press
            // moves neither `+ Success` nor `+ Failure`.
            let font = Type::Small.font(ui);
            let slot = (0..=3)
                .flat_map(|s| (0..=3).map(move |f| (s, f)))
                .map(|(s, f)| design::number_width(ui, &death_save_words(s, f), &font))
                .fold(0.0_f32, f32::max);
            ui.allocate_ui_with_layout(
                vec2(slot, Density::Read.control()),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.set_width(slot);
                    ui.label(
                        Type::Small
                            .rich(ui, death_save_words(successes, failures))
                            .color(c.text),
                    );
                },
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

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
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
            let unknown =
                |value: Option<i64>| value.map_or_else(|| "Unknown".to_owned(), |n| n.to_string());
            vital_columns(
                ui,
                &[
                    ("ARMOR CLASS", unknown(p.armor_class)),
                    (
                        "HIT POINTS",
                        format!("{} / {}", unknown(p.hp), unknown(p.max_hp)),
                    ),
                    ("INITIATIVE", unknown(p.initiative)),
                ],
            );
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
            // The death saves are above, below the tabs (`side_panel`); the status follows.
            if p.shows_death_saves() {
                design::gap(ui, design::Gap::L);
            }
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
            // The facts start with their own blank, and the line adds none: the gap before
            // each `·` is one space, the status word's too.
            let facts = if ready {
                " · Sync reads this sheet · HP from this run".to_owned()
            } else {
                format!(" · {age} · HP from this run")
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
                        // One text, so the blank before each `·` is the same space.
                        let font = Type::Small.font(ui);
                        let mut job = egui::text::LayoutJob::default();
                        for (text, color) in [
                            (format!("{} {word}", freshness.icon), freshness.color),
                            (facts, colors().muted),
                        ] {
                            job.append(
                                &text,
                                0.0,
                                egui::TextFormat {
                                    font_id: font.clone(),
                                    color,
                                    ..Default::default()
                                },
                            );
                        }
                        ui.add(egui::Label::new(job).wrap());
                    });
                });
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
            notice = snapshot_notice(definition, current, unreadable).map(|words| {
                // FD3: a file that cannot be read now is an error, as in every other view.
                let tone = if unreadable {
                    Tone::Error
                } else {
                    Tone::Warning
                };
                (words, tone, at)
            });
        }
        // FD5 (iter-27-R): the notice comes and goes with the file, so it is in the panel's
        // footer and the stat block keeps its top. Its owner is this row's snapshot: the notice
        // is worked out again each frame from it and the campaign's read, and ends when the
        // file matches again or another row is selected. Base: view.rs:4230-4262 (a3d947bd).
        let read = std::cell::Cell::new(None);
        let mut footer =
            design::Footer::new(("run_reference_footer", self.id), ui.available_width());
        if let Some((words, tone, at)) = &notice {
            footer.notice(words.as_str(), |ui| match at {
                Some(at) => {
                    let pressed = design::message_with_actions(ui, *tone, words, |ui| {
                        // AJ1: a message action without an icon has the secondary border,
                        // and its border starts at the notice's words.
                        Action::secondary("Read the new version")
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                    });
                    if pressed {
                        read.set(Some(*at));
                    }
                }
                None => {
                    design::message(ui, *tone, words);
                }
            });
        }
        let body_bottom = footer.show_by_hand(ui);
        if let Some(at) = read.get() {
            self.lookup_request = Some(at);
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
                .auto_shrink([false, false])
                .max_height((body_bottom - ui.cursor().top()).max(0.0)),
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

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
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
                design::heading(ui, design::Heading::Group { title: "Correct HP", count: 0 });
                let sheet_decides = self.syncs.get(&index).is_some_and(|sync| sync.ready());
                optional_number(ui, "Current HP", &mut hp, 0..=1_000_000);
                design::locked(ui, sheet_decides.then_some(MAX_FROM_SHEET), |ui| {
                    optional_number(ui, "Maximum HP", &mut max, 1..=1_000_000);
                });
                if sheet_decides {
                    design::help(ui, MAX_FROM_SHEET);
                }
                if hp != p.hp || max != p.max_hp {
                    command = Some(Command::Health(index, hp, max));
                }
                design::help(
                    ui,
                    "You resolve conditions, Concentration, death saves and recovery. Zero HP does not remove a turn.",
                );
                if self.state.phase == Phase::Setup {
                    // FJ8 (iter-27-R): the bonus has its own heading, as live play's total has.
                    design::heading(ui, design::Heading::Group { title: "Correct initiative", count: 0 });
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
                    design::heading(ui, design::Heading::Group { title: "Correct initiative", count: 0 });
                    let field = format!("Initiative for {}", p.name);
                    let text = self.setup_inputs.entry(index).or_default();
                    let total = text.trim().parse::<i64>().ok();
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
                    // FM2 (iter-27-R): the help is the field's, `S` below it; Set initiative's
                    // reason keeps its reserved line after the help.
                    ui.add(egui::Label::new(Type::Small.rich(ui, words).color(color)).wrap());
                    correction_reason(ui, "Type a number first.", total.is_none());
                }
                design::heading(ui, design::Heading::Group { title: "DM notes", count: 0 });
                let empty = self.note.trim().is_empty();
                // A note is a sentence: its field takes the form's whole line, and Add note
                // starts the line below it, at the field. The reason has its own line below.
                ui.scope(|ui| {
                    ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
                    ui.horizontal(|ui| {
                        correction_label(ui, "Note");
                        design::line_field(
                            ui,
                            DM_NOTE_FIELD
                                .of(self.id)
                                .with(design::TextKey::from_data(p.key.clone())),
                            "DM note",
                            &mut self.note,
                            "What happened",
                            // `S` clear of the tab's scroll bar when the tab is tall.
                            (ui.available_width() - design::S).max(design::XXL),
                        );
                    });
                });
                ui.horizontal(|ui| {
                    ui.add_space(correction_label_width(ui) + ui.spacing().item_spacing.x);
                    if Action::secondary("Add note")
                        .name("Add note")
                        .density(Density::Read)
                        .blocked_if(empty, "Type a note first.")
                        .show(ui)
                        .clicked()
                    {
                        command = Some(Command::Note(index, self.note.clone()));
                    }
                });
                correction_reason(ui, "Type a note first.", empty);
                if !p.notes.is_empty() {
                    ui.add(
                        egui::Label::new(Type::Body.rich(ui, &p.notes).color(colors().text)).wrap(),
                    );
                }
                if p.definition.fields.contains_key("splat_external") {
                    design::heading(ui, design::Heading::Group { title: "D&D Beyond combat sync", count: 0 });
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

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
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

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
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
                                            // FJ4 (iter-27-R): the selected choice has a check,
                                            // not only its fill; every word keeps one x.
                                            let selected = *mode == default_roll;
                                            let item = ui.selectable_label(
                                                selected,
                                                checked_choice(ui, selected, mode.label()),
                                            );
                                            design::named(ui, &item, mode.label());
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
            Layout::left_to_right(Align::Center),
            |ui| {
                // Add to fight has a fixed place at the roster's left edge, so a longer Undo
                // never moves it. Undo takes the rest of the line, from the right.
                if self.add_to_fight(true, Density::Read).show(ui).clicked() {
                    self.show_arrivals();
                }
                design::gap(ui, design::Gap::S);
                let label = self.undo_label();
                let undo = ui
                    .allocate_ui_with_layout(
                        vec2(ui.available_width(), Density::Read.control()),
                        Layout::right_to_left(Align::Center),
                        |ui| {
                            if self.history.is_empty() {
                                self.empty_undo(ui)
                            } else {
                                // The one rule of Undo labels, as in the live and ended header.
                                let room = ui.available_width();
                                let words = self.undo_words_in(ui, &label, room, Density::Read);
                                self.undo_action(&words)
                                    .name(&label)
                                    .hint(&label)
                                    .show(ui)
                                    .clicked()
                            }
                        },
                    )
                    .inner;
                if undo {
                    self.undo();
                    self.return_focus = self.lookup_open;
                }
            },
        );
        design::rule(ui);
        if let Some((root, filename)) = &self.roster_source {
            // FA11: the path in the campaign, as the encounter editor names the same file.
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
                    let file_width = design::number_width(ui, filename, &Type::Small.font(ui))
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
                            crate::world::entry_file_parts(ui, root, filename, true);
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
                                crate::encounters::file_line(ui, root, filename);
                            });
                        });
                    }
                });
            }
        }
        // FI2: errors, the carry-over warning and the rest offers and their result are in the
        // message footer below the rows (`message_footer`).
        // Opening beside a panel, the DM can be typing there, and a digit is not an initiative.
        if std::mem::take(&mut self.focus_first_missing) && !self.lookup_open {
            self.focus_field = self
                .state
                .participants
                .iter()
                .position(|p| p.initiative.is_none() && p.present);
        }
        let room = ui.available_height() - self.footer_height() - design::M;
        let region =
            Rect::from_min_size(ui.cursor().min, vec2(ui.available_width(), room.max(0.0)));
        let body_bottom = self.message_footer(ui, region, true, &mut command);
        let footer_room = (region.bottom() - body_bottom).max(0.0);
        let room = room - footer_room;
        let rows = ui.allocate_ui_with_layout(
            vec2(ui.available_width(), room),
            Layout::top_down(Align::LEFT),
            |ui| {
                ui.set_min_height(room);
                let mut spans = Vec::new();
                // SJ1 (iter-27-R): the roster keeps the scroll bar's lane whether or not the
                // bar shows, as Arrivals does, so adding arrivals moves no field, Absent or menu.
                let lane = (ui.available_width() - ui.spacing().scroll.allocated_width()).max(0.0);
                let output = egui::ScrollArea::vertical()
                    .id_salt("run_setup_rows")
                    .max_height((room - OVERFLOW_LINE).max(0.0))
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_max_width(lane);
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
        // Past the message footer, which the rows' room left free.
        ui.add_space(footer_room);
        design::rule(ui);
        design::gap(ui, design::Gap::XS);
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
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
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
        // FF1: a row from an unread roster line marks its own fault first, in the warning tone,
        // so the DM need not match the notice's name by reading. The notice keeps the cause.
        let fault = self
            .roster_fault_line(&p.key)
            .map(|line| format!("{} Line {line} not read", icons::WARN));
        let absent = match &fault {
            Some(fault) => format!("{fault} · {absent}"),
            None => absent,
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
            // FK6 (iter-27-R): every row starts with its side, and changed values keep their
            // mark after the setup numbers, as the encounter editor's row puts `Changed` last.
            let detail = if p.stat_changes() {
                format!("{absent}{numbers} · {} Changed", icons::CHANGED)
            } else {
                format!("{absent}{numbers}")
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
                let next = format!("{detail} · {}", condition.words());
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
            let status = match &fault {
                Some(fault) => Some((fault.replace(' ', "\u{a0}"), colors().result.warning)),
                None => {
                    hp_word(&p).map(|(icon, word, color)| (format!("{icon}\u{a0}{word}"), color))
                }
            };
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
            design::gap(ui, design::Gap::S);
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
                        .min_size(vec2(presence_slot, Density::Scan.control()));
                        // FN1: as the shared `toggle`, the off state keeps the frame and its
                        // padding with nothing painted, so the icon and word keep one x when the
                        // DM marks a participant absent. Off words stay muted (setup's rule).
                        let button = if present {
                            button
                                .frame(true)
                                .fill(egui::Color32::TRANSPARENT)
                                .stroke(egui::Stroke::NONE)
                        } else {
                            button.frame(true).fill(colors().selected)
                        };
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
                        // A verb, so the choice says it opens the settings in the row.
                        let words = if expanded {
                            "Hide NPC and roll mode"
                        } else {
                            "Change NPC and roll mode"
                        };
                        if Action::quiet(words).show(ui).clicked() {
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
                design::gap(ui, design::Gap::S);
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
                            design::choice_value(ui, &mut picked, m, mode_label(m));
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
        let title = if matches!(
            self.panel,
            Panel::Log | Panel::Encounter | Panel::Groups | Panel::Keys
        ) {
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
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
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

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn ended_view(&mut self, ui: &mut egui::Ui) -> bool {
        self.back_out_on_escape(ui);
        let c = colors();
        let mut closed = false;
        let outcome = match &self.state.phase {
            Phase::Ended { outcome } => outcome.clone(),
            _ => String::new(),
        };
        // The title and the save word keep their words. When the actions cannot share their
        // line, as beside Lookup in a small window, Undo and Continue fight take a line below.
        let gap = ui.spacing().item_spacing.x;
        let title_width = self.ended_title_width(ui, &outcome);
        let undo_room = self.ended_undo_room(ui);
        let one_line =
            title_width + self.ended_actions_width(ui, undo_room) <= ui.available_width();
        let first_line = design::L * 4.0;
        // Beside the log panel the second line is the roster's: Undo stays at its right edge.
        let line_width = if self.hide_panel {
            ui.available_width()
        } else {
            ui.available_width()
                - design::side_panel_width(ui.ctx().content_rect().width())
                - design::S
        };
        let (reason_room, second_line) = self.ended_second_line(ui, line_width, undo_room);
        let (header_rect, _) = ui.allocate_exact_size(
            vec2(
                ui.available_width(),
                first_line
                    + if one_line {
                        0.0
                    } else {
                        second_line + design::S
                    },
            ),
            egui::Sense::hover(),
        );
        design::rule(ui);
        // FI2: errors and a changed file sit in the message footer at the bottom of the view,
        // below the final state and the panel, so neither moves when one comes or goes.
        let region = ui.available_rect_before_wrap();
        let body_bottom = self.message_footer(ui, region, false, &mut None);
        if body_bottom < region.bottom() {
            let top = ui.cursor().top();
            ui.set_max_height((body_bottom - top).max(0.0));
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
                    // The scroll area's margin, so `Combat log` starts where `Prep` does.
                    egui::Frame::new()
                        .inner_margin(egui::Margin {
                            left: design::S as i8,
                            ..egui::Margin::ZERO
                        })
                        .show(ui, |ui| {
                            if choosing {
                                design::section(ui, "Combat log", |ui| self.log_tab(ui));
                            } else {
                                design::heading(ui, design::Heading::Label("Combat log"));
                                self.log_tab(ui);
                            }
                        });
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
                        // SI1: how everyone ended comes first, so the whole party's HP is in
                        // view; what the party takes follows.
                        design::heading(ui, design::Heading::Label("Final state"));
                        let spacing = ui.spacing().item_spacing.y;
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
                        ui.spacing_mut().item_spacing.y = spacing;
                        design::gap(ui, design::Gap::L);
                        self.loot_section(ui);
                    });
            });
        let mut header_ui = ui.new_child(
            egui::UiBuilder::new()
                .id_salt("ended_header")
                .max_rect(header_rect.with_max_y(header_rect.top() + first_line)),
        );
        header_ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.set_min_height(60.0);
            let close_reason = self.close_reason();
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
            if one_line {
                self.ended_continue(ui, f32::INFINITY);
                let room = ui.available_width() - title_width - gap;
                // Undo stays at the roster's right edge, but never gives its target to it.
                let align = (ui.available_rect_before_wrap().right() - roster_right).max(0.0);
                let align = match self.ended_undo_whole(ui) {
                    Some(whole) => align.min((room - whole).max(0.0)),
                    None => align,
                };
                ui.add_space(align);
                self.ended_undo(ui, room - align);
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
        if !one_line {
            let mut second = ui.new_child(
                egui::UiBuilder::new()
                    .id_salt("ended_header_actions")
                    .max_rect(
                        header_rect
                            .with_min_y(header_rect.top() + first_line)
                            .with_max_x(if self.hide_panel {
                                header_rect.right()
                            } else {
                                roster_right
                            }),
                    ),
            );
            second.with_layout(Layout::right_to_left(Align::Min), |ui| {
                self.ended_continue(ui, reason_room);
                let room = ui.available_width();
                self.ended_undo(ui, room);
            });
        }

        closed
    }

    /// The width the ended header keeps for its title, its outcome and its save word.
    fn ended_title_width(&self, ui: &egui::Ui, outcome: &str) -> f32 {
        let small = Type::Small.font(ui);
        let title = design::number_width(ui, &self.state_words(false), &Type::Title.font(ui));
        // A long outcome wraps. A short one keeps its line.
        let outcome = design::number_width(ui, &format!("Outcome: {outcome}"), &small);
        // The widest save word, so a step that saves never changes the header's form (X38).
        let status = self.save_status_value();
        let word = "Not saved";
        let saved = match &self.copy_label {
            Some(copy) => format!("{} {word} · {copy}", status.icon),
            None => format!("{} {word}", status.icon),
        };
        title.max(outcome.min(title * 2.0))
            + ui.spacing().item_spacing.x
            + design::number_width(ui, &saved, &small)
    }

    /// The width of the ended header's actions on the title's line: Close run (or Close after
    /// sync), the room of Continue fight or its reason, and Undo's room. It is a function of the
    /// width alone: never of Undo's target, of the reason or of Continue fight showing yet, so
    /// no act changes the header's form (X38). Where the second line is needed at all, the
    /// header has it from the start, and the lone Undo there names its target (W6-D2). Undo is
    /// at the roster's right edge, so beside the log panel the actions to its right take at
    /// least the panel's width.
    fn ended_actions_width(&self, ui: &egui::Ui, undo_room: f32) -> f32 {
        let gap = ui.spacing().item_spacing.x;
        let glance = |action: Action<'_>| action.density(Density::Glance).natural_width(ui) + gap;
        let close =
            glance(Action::primary("Close after sync")).max(glance(Action::primary("Close run")));
        let mut actions = close + ended_continue_room(ui) + gap;
        if !self.hide_panel {
            let panel = design::side_panel_width(ui.ctx().content_rect().width());
            actions = actions.max(panel + design::S);
        }
        actions + undo_room + gap
    }

    /// The room the ended header keeps for Undo, the same for every step: `Undo: end
    /// encounter`, and at least `14 × L`, which holds a short loot step whole (`Undo: 2 loot
    /// rolls on gems`). A longer target shortens in it by the rule of every Undo label
    /// (`undo_forms`); it never takes room from the line.
    fn ended_undo_room(&self, ui: &egui::Ui) -> f32 {
        self.undo_action(&format!("Undo: {END_STEP}"))
            .density(Density::Glance)
            .natural_width(ui)
            .max(design::L * 14.0)
    }

    /// The header's second line, when the actions take it: the room for the sentence that
    /// stands where Continue fight would be, beside Undo's room, and the line's height. Both
    /// come from the width alone: the line is as tall as the tallest of the reasons at that
    /// room, so no reason, target or step changes its height (X38), and none is ever cut.
    fn ended_second_line(&self, ui: &egui::Ui, width: f32, undo_room: f32) -> (f32, f32) {
        let gap = ui.spacing().item_spacing.x;
        let room = ended_continue_room(ui)
            .min(width - undo_room - 2.0 * gap)
            .max(design::L * 6.0);
        let tallest = super::CONTINUE_REASONS
            .iter()
            .map(|reason| {
                ui.fonts_mut(|fonts| {
                    fonts
                        .layout(
                            (*reason).to_owned(),
                            Type::Small.font(ui),
                            colors().muted,
                            room,
                        )
                        .size()
                        .y
                })
            })
            .fold(Density::Glance.control(), f32::max);
        (room, tallest)
    }

    /// The width of Undo with its whole target, when the ended header shows it beside
    /// Continue fight or its reason. Then Undo never gives up its target.
    fn ended_undo_whole(&self, ui: &egui::Ui) -> Option<f32> {
        let continues = !self.undo_continues() || self.undo_blocked().is_some();
        (continues && !self.history.is_empty() && self.undo_blocked().is_none()).then(|| {
            self.undo_action(&self.undo_label())
                .density(Density::Glance)
                .natural_width(ui)
        })
    }

    /// One control takes the ending away. While the ending is the newest step, that is Undo.
    /// After a loot roll, or with no steps, it is Continue fight, beside Close run: the two
    /// ways out of an ended fight, with the one that changes nothing last.
    fn ended_continue(&mut self, ui: &mut egui::Ui, room: f32) {
        if self.undo_continues() && self.undo_blocked().is_none() {
            return;
        }
        // Continue fight, and Undo of an ending from the run file, are for the fight that was
        // played last. For another fight, the reason stands where the control would be.
        if let Some(reason) = self.continue_words() {
            let width = continue_reason_width(ui, reason).min(room);
            ui.allocate_ui_with_layout(
                vec2(width, Type::Small.line()),
                Layout::top_down(Align::LEFT),
                |ui| {
                    ui.add(
                        egui::Label::new(Type::Small.rich(ui, reason).color(colors().muted)).wrap(),
                    );
                },
            );
        } else if continue_action()
            .density(Density::Glance)
            .show(ui)
            .clicked()
        {
            // The app has the Timeline, so it answers this by the rule before the next frame.
            self.wants_unend = Some(Unend::Continue);
        }
    }

    /// Undo in the ended header. It gives up its target before a word beside it is cut, and
    /// keeps the target in hover and in its accessible name.
    fn ended_undo(&mut self, ui: &mut egui::Ui, room: f32) {
        if self.history.is_empty() || self.undo_blocked().is_some() {
            return;
        }
        let full = self.undo_label();
        let words = self.undo_words_in(ui, &full, room, Density::Glance);
        if self
            .undo_action(&words)
            .density(Density::Glance)
            .name(&full)
            .hint(&full)
            .show(ui)
            .clicked()
        {
            self.undo();
            self.return_focus = self.lookup_open;
        }
    }

    /// Undo's words as a header's line has `room` for them, in the ended and the setup view:
    /// whole when they fit, else the first form of the one rule (`undo_forms`) that fits, and
    /// `Undo` alone last.
    fn undo_words_in(&self, ui: &egui::Ui, full: &str, room: f32, density: Density) -> String {
        let fits =
            |words: &str| self.undo_action(words).density(density).natural_width(ui) <= room + 0.5;
        if fits(full) {
            return full.to_owned();
        }
        let forms = undo_forms(
            full,
            self.undo_target(),
            self.undo_from_sheet(),
            &self.state.participants,
        );
        forms
            .target
            .into_iter()
            .chain(forms.bare)
            .find(|words| fits(words))
            .unwrap_or_else(|| "Undo".to_owned())
    }
}

/// The room of Continue fight or the sentence in its place, on the title's line: the widest
/// of the control and every reason, so it is the same whichever shows.
fn ended_continue_room(ui: &egui::Ui) -> f32 {
    super::CONTINUE_REASONS
        .iter()
        .map(|reason| continue_reason_width(ui, reason))
        .fold(
            continue_action().density(Density::Glance).natural_width(ui),
            f32::max,
        )
}

/// The width of the sentence that stands where Continue fight would be. It wraps to the
/// lines of the ended header, so that Close run keeps its place.
fn continue_reason_width(ui: &egui::Ui, reason: &str) -> f32 {
    design::number_width(ui, reason, &Type::Small.font(ui)).min(design::L * 15.0)
}

/// The one control that takes an ending away, wherever it shows.
pub(crate) fn continue_action() -> Action<'static> {
    Action::secondary("Continue fight").icon(icons::RUN)
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
/// The one Exhaustion entry at another level. Its time stays.
fn set_exhaustion(index: usize, level: u8) -> Command {
    Command::Condition(index, format!("Exhaustion ({level})"))
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
/// A footer message with its dismiss cross at its end (AI1). The cross is a `Read` target,
/// taller than the message's first line, so the message starts lower by half the difference:
/// the cross is centred on the first line, not on the top of the words. True when the DM
/// dismisses it.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn dismissable(ui: &mut egui::Ui, close: &str, message: impl FnOnce(&mut egui::Ui)) -> bool {
    ui.horizontal_top(|ui| {
        let width = ui.available_width() - Density::Read.control() - design::S;
        let lift = ((Density::Read.control() - Type::Body.line()) / 2.0).max(0.0);
        ui.vertical(|ui| {
            ui.set_max_width(width);
            ui.add_space(lift);
            message(ui);
        });
        design::icon_button(ui, icons::CLOSE, close).clicked()
    })
    .inner
}

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
    design::gap(ui, design::Gap::S);
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
            design::gap(ui, design::Gap::S);
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

/// An Arrivals row's status and members for a slot `width` wide. None when a part of the status
/// cannot fit the slot whole: then the status takes a full-width line of its own.
fn arrival_summary(ui: &egui::Ui, status: &str, members: &str, width: f32) -> Option<String> {
    if status.is_empty() {
        return Some(members.to_owned());
    }
    let font = Type::Small.font(ui);
    let status = status.replace(' ', "\u{a0}");
    let first = members.split(", ").next().unwrap_or(members);
    let joined = format!("{status}\u{a0}·\u{a0}{first}");
    if design::number_width(ui, &joined, &font) <= width {
        Some(format!("{status}\u{a0}·\u{a0}{members}"))
    } else if design::number_width(ui, &status, &font) <= width {
        Some(format!("{status}\n{members}"))
    } else {
        let parts: Vec<_> = status.split("\u{a0}·\u{a0}").collect();
        parts
            .iter()
            .all(|part| design::number_width(ui, part, &font) <= width)
            .then(|| format!("{}\n{members}", parts.join("\n")))
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

/// The words of an error for a stat block that cannot be read, whose repair is in Campaign →
/// Packages.
const STAT_BLOCK_UNREAD: &str = "stat block could not be read";

/// What the DM chose among a close dialog's recovery actions.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(crate) struct CloseRecovery {
    pub review: bool,
    pub connect: bool,
}

/// The recovery actions that the app-close and run-close dialogs offer above their action line.
/// Both dialogs use this one function, so the actions have one order and one place: secondary
/// buttons on one line, `Connect to sync` before `Review conflict`, and the first one's border
/// starts at the dialog text. FK2: a message action without an icon has the secondary border, so
/// the safe action never reads as a label beside `Close now`.
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
        let action = |ui: &mut egui::Ui, label: &str| Action::secondary(label).show(ui).clicked();
        if connect {
            chosen.connect = action(ui, "Connect to sync");
        }
        if review {
            chosen.review = action(ui, "Review conflict");
        }
    });
    chosen
}

/// The close dialog's words for a sync conflict. They offer a close only when a close can happen.
/// Round 27 FN4: a blocked close has no words here. The title names the conflict, and the reason
/// beside the disabled `Close now` says what to do, once.
fn conflict_close_words(blocked: bool) -> Option<&'static str> {
    (!blocked).then_some("Choose which HP to keep, or close and choose next time.")
}

/// What a close keeps. A close keeps the run's Undo steps too (they are saved with the run), so
/// nothing here may say that closing ends Undo.
fn close_saved_words(sync: bool) -> String {
    if sync {
        "The run stays saved. Sync stops."
    } else {
        "The run stays saved. Resume it from Paused run on the left, or from Encounters."
    }
    .to_owned()
}

/// The tie button's words, `Move <arrival> before <name>`, when they fit `width`. FA6: the
/// button sits under the message that names its arrival whole, and it only ever moves that
/// arrival, so when the whole words do not fit, an arrival named by its first member,
/// `Shrine Reserve (Goblin Warrior 3)`, gives up the parenthesis, never just the member's name:
/// `Move Shrine Reserve before Brannoch Vale`. A bare `(… 3)` tells the DM nothing. Hover and
/// the accessible name keep the whole words. None when no form fits.
fn move_words(
    ui: &egui::Ui,
    arrival: &str,
    before: &str,
    font: &egui::FontId,
    width: f32,
) -> Option<String> {
    let full = format!("Move {arrival} before {before}");
    if design::number_width(ui, &full, font) <= width {
        return Some(full);
    }
    let group = arrival
        .strip_suffix(')')
        .and_then(|rest| rest.rsplit_once(" ("))
        .map_or(arrival, |(group, _)| group);
    if group == arrival {
        return None;
    }
    shortened_names(group, before).find_map(|(group, before)| {
        let short = format!("Move {group} before {before}");
        (design::number_width(ui, &short, font) <= width).then_some(short)
    })
}

/// Two names with their last words given up in turn, the other name's last: each pair from
/// both whole to both at their first word and `…`.
fn shortened_names<'a>(group: &'a str, before: &'a str) -> impl Iterator<Item = (String, String)> {
    let groups = name_words(group);
    name_words(before).into_iter().flat_map(move |before| {
        groups
            .clone()
            .into_iter()
            .map(move |group| (group, before.clone()))
    })
}

/// The forms of one Undo label, longest first: see `undo_forms`.
pub(super) struct UndoForms {
    /// Forms that name the act's target, from the whole label to its shortest telling form.
    pub(super) target: Vec<String>,
    /// Forms without the target, for an Undo too narrow for any of `target`:
    /// `Undo: 8 damage × 3`. After them, the caller shows `Undo` alone.
    pub(super) bare: Vec<String>,
}

/// The one rule of the run's Undo labels. A label names its act and its target, which
/// `describe_named` gives as `target`: never read back out of the label's words. A long
/// effect keeps four of its words, then two (`Undo: add effect Wis save… on Cinder Wolf 2`). Then
/// the target shortens by whole words and keeps the part that tells it from another row: its
/// copy number or letter (`Cinder… 2`), else its last word. A sheet's step gives up its source
/// before its target's short forms. When no telling form fits, the target goes (`bare`). No
/// form cuts a name to only `…` and its number. Hover and the accessible name keep the whole
/// label.
pub(super) fn undo_forms(
    label: &str,
    target: Option<&str>,
    from_sheet: bool,
    participants: &[Participant],
) -> UndoForms {
    let (prefix, action) = label
        .strip_prefix("Undo: ")
        .map_or(("", label), |action| ("Undo: ", action));
    let mut actions = vec![action];
    if from_sheet && let Some(rest) = action.strip_prefix(SHEET_SOURCE) {
        actions.push(rest);
    }
    let mut seen = std::collections::HashSet::new();
    let forms = actions
        .iter()
        .flat_map(|action| action_forms(action, target, participants))
        .filter(|words| keeps_names(words) && seen.insert(words.clone()))
        .map(|words| format!("{prefix}{words}"))
        .collect();
    let bare = if let Some((verb, ..)) = move_parts(action) {
        vec![verb.to_owned()]
    } else if let Some((before, _)) = target.and_then(|target| split_target(action, target)) {
        act_words(before)
    } else {
        undo_compact(label).into_iter().collect()
    };
    UndoForms {
        target: forms,
        bare: bare
            .into_iter()
            .map(|words| format!("{prefix}{}", words.trim_start_matches(prefix)))
            .collect(),
    }
}

/// The first of `undo_forms`' target forms that fits `width`, else its shortest one. The
/// caller measures it, and goes on to the bare forms, then `Undo` alone.
pub(super) fn undo_words(
    ui: &egui::Ui,
    (label, target): (&str, Option<&str>),
    from_sheet: bool,
    participants: &[Participant],
    font: &egui::FontId,
    width: f32,
) -> String {
    let forms = undo_forms(label, target, from_sheet, participants).target;
    forms
        .iter()
        .find(|words| design::number_width(ui, words, font) <= width)
        .or(forms.last())
        .map_or_else(|| label.to_owned(), Clone::clone)
}

/// An action's words before and after its target, which `describe_named` puts at the label's
/// end, or at its start (`Brannoch Vale absent`).
fn split_target<'a>(action: &'a str, target: &str) -> Option<(&'a str, &'a str)> {
    if target.is_empty() {
        return None;
    }
    action
        .strip_suffix(target)
        .map(|before| (before, ""))
        .or_else(|| action.strip_prefix(target).map(|after| ("", after)))
}

/// The act's own words without the target and its joining word: an effect's four, then two
/// (`add effect Wis save…`), or `7 damage`, `initiative`, `2 loot rolls`. None for a name-first
/// step.
fn act_words(before: &str) -> Vec<String> {
    if let Some((words, _)) = effect_words(before) {
        return words;
    }
    [" on ", " off ", " for ", " to ", " from "]
        .into_iter()
        .find_map(|joining| before.strip_suffix(joining))
        .map(str::trim)
        .filter(|words| !words.is_empty())
        .map(str::to_owned)
        .into_iter()
        .collect()
}

/// One action's forms with its target, longest first.
fn action_forms(action: &str, target: Option<&str>, participants: &[Participant]) -> Vec<String> {
    if let Some(forms) = move_forms(action) {
        return forms;
    }
    let Some((name, (before, after))) =
        target.and_then(|name| Some((name, split_target(action, name)?)))
    else {
        return vec![action.to_owned()];
    };
    let words: Vec<String> = match effect_words(before).filter(|_| after.is_empty()) {
        Some((words, joining)) => words
            .into_iter()
            .map(|words| format!("{words}{joining}"))
            .collect(),
        None => vec![before.to_owned()],
    };
    let shortest = words.last().expect("one form of the words");
    let mut forms: Vec<String> = words
        .iter()
        .map(|words| format!("{words}{name}{after}"))
        .collect();
    forms.extend(
        target_forms(name, participants)
            .into_iter()
            .skip(1)
            .map(|name| format!("{shortest}{name}{after}")),
    );
    forms
}

/// The words of an effect's step before its target (`add effect Wis save DC 15 (Hold Person)
/// on `) when they are more than four: four of them, then two, each with `…`, and the joining
/// word. The verb (`add effect`, `add`, `remove`) stays and is not counted. None for any other step's words,
/// which stay whole.
fn effect_words(before: &str) -> Option<(Vec<String>, &'static str)> {
    let (words, joining) = [" on ", " off ", " from "]
        .into_iter()
        .find_map(|joining| Some((before.strip_suffix(joining)?, joining)))?;
    // FK4 (iter-27-R): every effect step starts with its verb, `add effect`, `add` or `remove`.
    let (verb, words) = ["add effect ", "add ", "effect ", "remove "]
        .into_iter()
        .find_map(|verb| Some((verb.to_owned(), words.strip_prefix(verb)?)))
        .unwrap_or((String::new(), words));
    let words: Vec<&str> = words.split_whitespace().collect();
    (words.len() > 4).then(|| {
        let forms = [4, 2]
            .into_iter()
            .map(|n| format!("{verb}{}…", words[..n].join(" ")))
            .collect();
        (forms, joining)
    })
}

/// A row's name by whole words, longest first, each form telling it from every other row:
/// `Cinder Wolf 2` then `Cinder… 2`; `Brannoch Vale` then `Brannoch…`. A form that another
/// row's name also gives (`Rival…` beside `Rival Captain 2`) keeps the last word instead, or is
/// left out.
fn target_forms(name: &str, participants: &[Participant]) -> Vec<String> {
    let words: Vec<&str> = name.split_whitespace().collect();
    let mut forms = vec![name.to_owned()];
    let Some(last) = words.last().copied() else {
        return forms;
    };
    let copy =
        words.len() > 1 && (last.chars().all(|c| c.is_ascii_digit()) || last.chars().count() == 1);
    let tells = |kept: &str, end: Option<&str>| {
        !participants.iter().any(|p| {
            p.name != name
                && p.name.starts_with(&format!("{kept} "))
                && end.is_none_or(|end| p.name.ends_with(&format!(" {end}")))
        })
    };
    let longest = words.len().saturating_sub(if copy { 2 } else { 1 });
    for count in (1..=longest).rev() {
        let kept = words[..count].join(" ");
        if !copy && tells(&kept, None) {
            forms.push(format!("{kept}…"));
        } else if count + 1 < words.len() && tells(&kept, Some(last)) {
            forms.push(format!("{kept}… {last}"));
        }
    }
    forms
}

/// A move's verb, its arrival and the name it moved before: `move <arrival> before <name>`.
fn move_parts(action: &str) -> Option<(&'static str, &str, &str)> {
    let (verb, words) = action
        .strip_prefix("move ")
        .map(|words| ("move", words))
        .or_else(|| action.strip_prefix("Move ").map(|words| ("Move", words)))?;
    let (moved, before) = words.split_once(" before ")?;
    Some((verb, moved, before))
}

/// A move's forms with its target, longest first. SJ1: Undo is not beside the message that
/// names the arrival, so an arrival with a copy number keeps it. The moved arrival is what the
/// DM must know (iter-27-R FI8), so the place goes first, `move Rival Team (Rival Captain 2)`,
/// then the group, `move Rival Captain 2`; last the member's name gives way to `…`,
/// `move Rival Team (… 2)`, then `move Rival… (… 2)`.
/// An arrival with no copy number shortens with the other name by whole words. None for any
/// other step.
fn move_forms(action: &str) -> Option<Vec<String>> {
    let (verb, moved, before) = move_parts(action)?;
    let parts = moved
        .strip_suffix(')')
        .and_then(|rest| rest.rsplit_once(" ("));
    let copy = parts.and_then(|(group, member)| {
        let (_, last) = member.rsplit_once(' ')?;
        (!last.is_empty() && last.chars().all(|c| c.is_ascii_digit()))
            .then_some((group, member, last))
    });
    let mut forms = vec![action.to_owned()];
    match copy {
        Some((group, member, copy)) => {
            forms.push(format!("{verb} {moved}"));
            forms.push(format!("{verb} {member}"));
            forms.extend(
                name_words(group)
                    .into_iter()
                    .map(|group| format!("{verb} {group} (…\u{a0}{copy})")),
            );
        }
        None => {
            let moved = parts.map_or(moved, |(group, _)| group);
            forms.extend(
                shortened_names(moved, before)
                    .map(|(moved, before)| format!("{verb} {moved} before {before}")),
            );
        }
    }
    Some(forms)
}

/// A step on several rows without them, for an Undo too narrow for its words:
/// `Undo: 8 damage × 3` (a step on one row gives up its row by `act_words`). FI4: the row count follows `×`, never `to`, so it cannot be read as a copy number
/// (`to 2` beside `Cinder Wolf 2`). `None` when the words name no row to leave out.
pub(super) fn undo_compact(label: &str) -> Option<String> {
    let action = label.strip_prefix("Undo: ")?;
    let several = action.split(super::HALF_FOR).next().unwrap_or(action);
    let counted = several.strip_suffix(super::SEVERAL_TARGETS)?;
    let (words, count) = counted.rsplit_once(" to ")?;
    Some(format!("Undo: {words} × {count}"))
}

/// False when shortening left a name as only `…` and its copy number (`move … 2 before`,
/// `effect Wis… on … 2`): such a form tells the DM nothing, so `undo_forms` never gives it.
pub(super) fn keeps_names(words: &str) -> bool {
    !words.split(' ').any(|word| word == "…")
}

/// A name with its last words given up in turn, by whole words: from the whole name to its
/// first word and `…`.
fn name_words(name: &str) -> Vec<String> {
    let all: Vec<&str> = name.split_whitespace().collect();
    (1..=all.len())
        .rev()
        .map(|n| {
            let kept = all[..n].join(" ");
            if n == all.len() {
                kept
            } else {
                format!("{kept}…")
            }
        })
        .collect()
}

/// Keep the character's name; give up later values before shortening a sheet notice.
fn sheet_notice_words(ui: &egui::Ui, summary: &str, font: &egui::FontId, width: f32) -> String {
    // FJ1: a count of removed Undo steps stays as long as the character's name does.
    let (summary, removed) = summary
        .rsplit_once(" · ")
        .filter(|(_, removed)| removed.ends_with(" removed"))
        .map_or((summary, String::new()), |(summary, removed)| {
            (summary, format!(" · {removed}"))
        });
    let (first, more) = summary
        .rsplit_once(" and ")
        .filter(|(_, more)| more.ends_with("sheet") || more.ends_with("sheets"))
        .map_or((summary, String::new()), |(first, more)| {
            (first, format!(" and {more}"))
        });
    let values: Vec<_> = first.split(", ").collect();
    let fits = |words: &str| design::number_width(ui, words, font) <= width;
    // FJ4: then shorter forms, so the notice keeps one line and its band its height: the
    // first value alone, `<name>: <value>`, without the other sheets, then the name alone.
    let compact = values[0].replace(super::SHEET_CHANGED, ":");
    let name = values[0]
        .split_once(super::SHEET_CHANGED)
        .map_or(values[0], |(name, _)| name);
    let mut forms: Vec<String> = (1..=values.len())
        .rev()
        .map(|n| format!("{}{more}{removed}", values[..n].join(", ")))
        .collect();
    forms.push(format!("{compact}{more}{removed}"));
    forms.push(format!("{compact}{removed}"));
    if !removed.is_empty() {
        forms.push(format!("{name}{removed}"));
    }
    match forms.iter().find(|form| fits(form)) {
        Some(form) => form.clone(),
        None => design::words_that_fit(
            ui,
            forms.last().map_or(summary, String::as_str),
            font,
            width,
        ),
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

/// FK1 (iter-27-R): the dock's selected name in its slot `room` wide, in the text colour. Whole
/// on one line in `Heading`; else whole in `Small` on up to two lines that share one control's
/// height; else two `Small` lines of whole words that end in `…` and keep a copy number or
/// letter (`Ember Cultist of the… 12`); a form on one line takes the next word's first letters
/// onto the second (SK1: `Seraphina Thorn…`); else the first word's letters and `…`. Hover
/// and the accessible name hold it whole.
pub(super) fn dock_name_shown(ui: &egui::Ui, name: &str, room: f32) -> Arc<Galley> {
    let layout = |text: &str, font: egui::FontId, line_height: Option<f32>| {
        let mut job = egui::text::LayoutJob::single_section(
            text.to_owned(),
            egui::TextFormat {
                font_id: font,
                color: colors().text,
                line_height,
                ..Default::default()
            },
        );
        job.wrap.max_width = room.max(1.0);
        ui.fonts_mut(|fonts| fonts.layout_job(job))
    };
    let heading = Type::Heading.font(ui);
    if design::number_width(ui, name, &heading) <= room {
        return layout(name, heading, None);
    }
    let line = Some(Density::Read.control() / DOCK_NAME_LINES as f32);
    let fits = |text: &str| {
        let galley = layout(text, Type::Small.font(ui), line);
        (galley.rows.len() <= DOCK_NAME_LINES && galley.size().x <= room + 0.5).then_some(galley)
    };
    if let Some(galley) = fits(name) {
        return galley;
    }
    let words: Vec<&str> = name.split_whitespace().collect();
    let last = words.last().copied().unwrap_or(name);
    let numbered =
        words.len() > 1 && (last.chars().all(|c| c.is_ascii_digit()) || last.chars().count() == 1);
    let (body, end) = if numbered {
        (&words[..words.len() - 1], format!("…\u{a0}{last}"))
    } else {
        (&words[..], "…".to_owned())
    };
    for count in (1..body.len()).rev() {
        let mut kept = &body[..count];
        // A name does not end on a joining word or a comma.
        while kept.len() > 1
            && kept
                .last()
                .is_some_and(|word| ["of", "the", "and", "de", "von"].contains(word))
        {
            kept = &kept[..kept.len() - 1];
        }
        let shown = format!("{}{end}", kept.join(" ").trim_end_matches([',', ';', ':']));
        if let Some(galley) = fits(&shown) {
            // SK1: a form on one line, with only `…` on the second, or with the first word
            // alone shows no more than that word. While the next word's first
            // letters fit there, they show (`Seraphina Thorn…`), so the slot keeps more of
            // the name than its first word.
            let second_empty = galley.rows.len() < DOCK_NAME_LINES
                || kept.len() == 1
                || galley
                    .rows
                    .last()
                    .is_some_and(|row| row.text().trim() == end.trim());
            if second_empty && let Some(next) = body.get(kept.len()) {
                let letters: Vec<char> = next.chars().collect();
                let partial = (2..letters.len()).rev().find_map(|count| {
                    fits(&format!(
                        "{} {}{end}",
                        kept.join(" "),
                        letters[..count].iter().collect::<String>()
                    ))
                    .filter(|galley| galley.rows.len() == DOCK_NAME_LINES)
                });
                if let Some(partial) = partial {
                    return partial;
                }
            }
            return galley;
        }
    }
    // The first word's letters, with the copy number while it fits, then without it.
    let letters: Vec<char> = words.first().copied().unwrap_or(name).chars().collect();
    [end.as_str(), "…"]
        .into_iter()
        .find_map(|end| {
            (1..=letters.len()).rev().find_map(|count| {
                fits(&format!(
                    "{}{end}",
                    letters[..count].iter().collect::<String>()
                ))
            })
        })
        .unwrap_or_else(|| layout("…", Type::Small.font(ui), line))
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

pub(super) fn presence_action(present: bool) -> (&'static str, &'static str) {
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

/// A menu choice's words after a check slot: the check shows on the selected choice, and the
/// slot is kept on the others, so every choice's words start at one x (FJ4).
fn checked_choice(ui: &egui::Ui, selected: bool, words: &str) -> egui::text::LayoutJob {
    let font = Type::Body.font(ui);
    let c = colors();
    let mut job = egui::text::LayoutJob::default();
    for (text, color) in [
        (
            format!("{} ", icons::OK),
            if selected {
                c.text
            } else {
                egui::Color32::TRANSPARENT
            },
        ),
        (words.to_owned(), c.text),
    ] {
        job.append(
            &text,
            0.0,
            egui::TextFormat {
                font_id: font.clone(),
                color,
                ..Default::default()
            },
        );
    }
    job
}

/// The death-save counts in words: `1 success · 0 failures`.
fn death_save_words(successes: u8, failures: u8) -> String {
    format!(
        "{successes} {} · {failures} {}",
        if successes == 1 {
            "success"
        } else {
            "successes"
        },
        if failures == 1 { "failure" } else { "failures" }
    )
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
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
pub(super) fn log_link(ui: &mut egui::Ui) -> egui::Response {
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

/// Concentration, as a run row marks it and History's Final state repeats it: its icon and
/// `Conc`, in its own colour.
pub(crate) fn concentration_mark() -> (String, egui::Color32) {
    (
        format!("{} Conc", icons::CONCENTRATING),
        colors().participant.concentrating,
    )
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
    parts.extend(p.conditions.iter().map(super::Condition::words));
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
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
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

/// Each row's finished layout, by the hash of its whole input: `RowLayout::fitted`.
#[derive(Clone, Default)]
struct RowCache(std::collections::HashMap<u64, RowLayout>);

#[cfg(test)]
thread_local! {
    /// How often `fitted` was asked for a row, and how often a row was laid out.
    static ROW_FITS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static ROW_LAYOUTS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// What a row's layout shows of its participant: every word and state it reads.
#[derive(Clone, Hash)]
struct RowWords {
    name: String,
    side: String,
    hp_word: Option<(&'static str, &'static str, egui::Color32)>,
    present: bool,
    initiative: bool,
    concentration: bool,
    conditions: Vec<super::Condition>,
    /// `as <name>`: the stat block an NPC uses.
    as_name: Option<String>,
    /// The changed stats' bonuses, when the encounter or group changed the creature.
    bonuses: Option<Vec<String>>,
    loot: bool,
}

impl RowWords {
    fn of(p: &Participant) -> Self {
        Self {
            name: p.name.clone(),
            side: p.side.clone(),
            hp_word: hp_word(p),
            present: p.present,
            initiative: p.initiative.is_some(),
            concentration: p.concentration,
            conditions: p.conditions.clone(),
            as_name: p.npc_identity().then(|| p.definition.name.clone()),
            bonuses: p
                .changes
                .as_ref()
                .filter(|_| p.stat_changes())
                .map(|changes| changes.overrides.table_bonuses(&changes.base)),
            loot: p.loot_ready(),
        }
    }
}

/// A row layout's whole input. `RowLayout::measured` reads nothing else, and the hash names
/// every field, so the cache's key holds all of it.
#[derive(Clone)]
struct RowInput {
    words: RowWords,
    sync: Option<SyncState>,
    conc_dc: Option<u32>,
    /// The row has the turn: an effect due on it comes first (W1-D1).
    turn: bool,
    scale: RunScale,
    fit: RowFit,
    width: f32,
    longest_name: f32,
    pixels_per_point: f32,
}

impl std::hash::Hash for RowInput {
    fn hash<H: std::hash::Hasher>(&self, hash: &mut H) {
        let Self {
            words,
            sync,
            conc_dc,
            turn,
            scale,
            fit,
            width,
            longest_name,
            pixels_per_point,
        } = self;
        let RunScale {
            row,
            name,
            hp,
            hp_max,
            stat,
            condition,
            mark,
            max_width,
        } = scale;
        words.hash(hash);
        sync.hash(hash);
        conc_dc.hash(hash);
        turn.hash(hash);
        fit.hash(hash);
        for value in [
            width,
            longest_name,
            pixels_per_point,
            row,
            name,
            hp,
            hp_max,
            stat,
            condition,
            mark,
            max_width,
        ] {
            value.to_bits().hash(hash);
        }
    }
}

/// What a row's second line holds beyond today's words: its effects and a save's DC.
#[derive(Default)]
struct Extra {
    effects: EffectPart,
    conc_dc: Option<u32>,
    /// Each effect shows as its plain words: the row with each time taken away (X5, the
    /// ruling on PR 276), whose lines and height the row keeps.
    as_plain: bool,
}

/// The effects a row shows: the first ones with their words, each with its count, then the
/// count of the rest, `⚑ +2`, or of them all, `⚑ 3`.
#[derive(Clone, Default)]
struct EffectPart {
    /// Each shown effect's place in the row's conditions, and the words of its name.
    shown: Vec<(usize, String)>,
    rest: usize,
    /// The most plain conditions shown before `N more`, when the effects' words need their
    /// room. None: all of them, or at fewest one, as the row's folding of section 1.4 can
    /// leave them.
    plain_at_most: Option<usize>,
}

/// What a click on an effect's target does.
#[derive(Clone, Copy, Debug, PartialEq)]
enum EffectAct {
    /// Opens **Conditions**, where the effect is removed.
    Open,
    /// `time is up`: removes the effect.
    Remove(usize),
    Spend(usize),
    Recharge(usize),
}

/// The count after an effect's name, in its colours, and every form it can take, to measure
/// its slot. The first part is the gap after the name: `Bless 9`, `Bless · last round`.
fn count_slot(condition: &super::Condition) -> (Vec<(String, egui::Color32)>, Vec<String>) {
    let c = colors();
    let gap = " ·\u{a0}";
    let forms = condition
        .count_forms()
        .into_iter()
        .map(|form| match form.strip_prefix("· ") {
            Some(words) if words == "time is up" => format!("{gap}{} {words}", icons::WARN),
            Some(words) => format!("{gap}{words}"),
            None if form == "spent" => format!(" spent {}", icons::DICE),
            None => format!(" {form}"),
        })
        .collect();
    let Some(count) = condition.count() else {
        return (Vec::new(), forms);
    };
    let parts = match condition.time() {
        Some(Time::Rounds { left: 0, .. }) => vec![
            (gap.to_owned(), c.muted),
            (format!("{} ", icons::WARN), c.result.warning),
            (count, c.text),
        ],
        Some(Time::Rounds { left: 1, .. }) => {
            vec![(gap.to_owned(), c.muted), (count, c.participant.condition)]
        }
        Some(Time::Recharge { spent: true, .. }) => vec![
            (" ".to_owned(), c.muted),
            (format!("{count} {}", icons::DICE), c.participant.condition),
        ],
        _ => vec![(" ".to_owned(), c.muted), (count, c.participant.condition)],
    };
    (parts, forms)
}

#[derive(Clone)]
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
    /// Each target of the effects on the second line: its act, the effect's place in the
    /// row's conditions (none for the count of the rest), and where it is, in characters.
    effect_spans: Vec<(EffectAct, usize, std::ops::Range<usize>)>,
    /// The second line needed the small type to keep to one line.
    detail_small: bool,
    /// The same row with no effect and no save DC, whose lines and height this row keeps.
    without_effects: Option<Box<RowLayout>>,
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
    /// The input this layout was made from, so that a roll result can lay the row out again
    /// with fewer condition words (R8). Shared, so a cached layout's copy costs no work.
    input: Option<std::sync::Arc<RowInput>>,
}

pub(super) fn header_height(scale: RunScale) -> f32 {
    (scale.hp * 1.6).max(36.0)
}

impl RowLayout {
    /// The row's roll words, after its second line. FI2 (iter-27-R): a roll result does not
    /// change a row's height while it fits there. R8 (SL2): the result's two parts (the
    /// comparison or save outcome, and the damage) are never cut. In order: the result whole
    /// after the effects and the save DC; whole on the row without them; whole on that row
    /// with fewer condition words (the rest in its `N more`), at the row's height. Only when
    /// the two parts fit in none of these does the result take a line of its own, and the row
    /// grows by it. The hover and the Log keep it whole.
    fn with_badge(mut self, ui: &egui::Ui, badge: Option<&str>) -> Self {
        self.badge_below = false;
        let base = self
            .without_effects
            .take()
            .map(|base| base.with_badge(ui, badge));
        if let Some(badge) = badge {
            let font = Type::Small.font(ui);
            let parts: Vec<&str> = badge.split(" · ").collect();
            let result = parts[parts.len().saturating_sub(2)..].join(" · ");
            let whole = design::number_width(ui, &result, &font);
            let room = |layout: &Self| badge_place(layout, 0.0).2;
            if whole > room(&self) {
                if let Some(base) = base
                    .as_ref()
                    .filter(|base| !base.badge_below && whole <= room(base))
                {
                    return base.clone();
                }
                let plain = base.as_ref().unwrap_or(&self);
                if let Some(fewer) = plain.with_fewer_conditions(ui, |layout| whole <= room(layout))
                {
                    return Self {
                        art: self.art.clone(),
                        ..fewer
                    };
                }
                self.badge_below = true;
            }
        }
        self.without_effects = base.map(Box::new);
        self
    }

    /// R8: this row (with no effect and no save DC) with fewer condition words shown, the
    /// rest in its `N more`, and last with no words on its second line: the first form, from
    /// the most words down, that keeps the row's height and in which `fits` holds. Each form
    /// is laid out once and kept by its input.
    fn with_fewer_conditions(&self, ui: &egui::Ui, fits: impl Fn(&Self) -> bool) -> Option<Self> {
        let input = self.input.as_ref()?;
        let shown = self
            .condition_spans
            .iter()
            .filter(|(name, _)| !["Run conditions", "Run changes"].contains(&name.as_str()))
            .count();
        let scale = input.scale;
        // Keyed by this row's input and the form, so a frame that changes nothing lays out no
        // form and builds no input.
        let line_font = self
            .second_line
            .as_ref()
            .and_then(|line| line.job.sections.first())
            .map(|section| section.format.font_id.clone());
        let form = |count: Option<usize>| {
            let key = {
                use std::hash::{Hash, Hasher};
                let mut hash = std::collections::hash_map::DefaultHasher::new();
                ("fewer conditions", count).hash(&mut hash);
                input.hash(&mut hash);
                hash.finish()
            };
            let cache = egui::Id::new("run_row_layouts");
            let layout = ui
                .data_mut(|data| {
                    data.get_temp_mut_or_default::<RowCache>(cache)
                        .0
                        .get(&key)
                        .cloned()
                })
                .unwrap_or_else(|| {
                    let extra = Extra {
                        as_plain: true,
                        ..Extra::default()
                    };
                    // `None`: every word of the second line gives way (state words too); the
                    // hover and the accessible name keep them.
                    let input = match count {
                        Some(_) => Arc::clone(input),
                        None => Arc::new(RowInput {
                            words: RowWords {
                                hp_word: None,
                                initiative: true,
                                concentration: false,
                                conditions: Vec::new(),
                                as_name: None,
                                bonuses: None,
                                loot: false,
                                ..input.words.clone()
                            },
                            ..(**input).clone()
                        }),
                    };
                    let bare = count.is_none();
                    let mut layout =
                        Self::measured(ui, &input, (count.unwrap_or(0), BonusFit::All, &extra));
                    if bare {
                        // The result is this row's second line: the line keeps its height (an
                        // empty line of the row's own type), and the row is not name-only.
                        layout.tight = false;
                        if let Some(font) = line_font.clone() {
                            layout.second_line = Some(ui.fonts_mut(|fonts| {
                                fonts.layout_job(egui::text::LayoutJob::single_section(
                                    String::new(),
                                    egui::TextFormat::simple(font, colors().muted),
                                ))
                            }));
                        }
                    }
                    layout.input = Some(input);
                    ui.data_mut(|data| {
                        data.get_temp_mut_or_default::<RowCache>(cache)
                            .0
                            .insert(key, layout.clone());
                    });
                    layout
                });
            (layout.height(scale) == self.height(scale) && fits(&layout)).then_some(layout)
        };
        (0..shown)
            .rev()
            .map(Some)
            .chain(std::iter::once(None))
            .find_map(form)
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
        if let Some(base) = self.without_effects.take() {
            self.without_effects = Some(Box::new(base.with_art(art.clone())));
        }
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
        Self::fitted(ui, width, p, (sync, None, false), scale, fit, longest_name)
    }

    /// The row's rule of section 1.4, as one rule: the row's height and lines are those of
    /// the same row with each time taken away (each effect is its plain words, and an official
    /// condition with a time is that condition). The effects then take only the room that is
    /// spare there, in the first of these forms that keeps both and shows by its words each
    /// effect that row shows: each effect whole; the first ones whole and a count of the
    /// rest; the first one's words shortened from their end and a count; the count of them
    /// all. Else the row is that row: its effects show as their words without their times,
    /// and its hover has them whole. On the row that has the turn, an effect due on it comes
    /// first. The save DC then takes the room that the effects leave. A finished layout is
    /// kept by its whole input, so a frame that changes nothing in a row lays it out no more.
    fn fitted(
        ui: &egui::Ui,
        width: f32,
        p: &Participant,
        (sync, conc_dc, turn): (Option<SyncState>, Option<u32>, bool),
        scale: RunScale,
        fit: RowFit,
        longest_name: f32,
    ) -> Self {
        #[cfg(test)]
        ROW_FITS.with(|count| count.set(count.get() + 1));
        let input = RowInput {
            words: RowWords::of(p),
            sync,
            conc_dc,
            turn,
            scale,
            fit,
            width,
            longest_name,
            pixels_per_point: ui.ctx().pixels_per_point(),
        };
        let key = {
            use std::hash::{Hash, Hasher};
            let mut hash = std::collections::hash_map::DefaultHasher::new();
            input.hash(&mut hash);
            hash.finish()
        };
        let cache = egui::Id::new("run_row_layouts");
        if let Some(layout) = ui.data_mut(|data| {
            data.get_temp_mut_or_default::<RowCache>(cache)
                .0
                .get(&key)
                .cloned()
        }) {
            return layout;
        }
        let mut layout = Self::laid_out(ui, &input);
        let input = std::sync::Arc::new(input);
        layout.input = Some(input.clone());
        if let Some(base) = layout.without_effects.as_mut() {
            base.input = Some(input);
        }
        ui.data_mut(|data| {
            let layouts = &mut data.get_temp_mut_or_default::<RowCache>(cache).0;
            if layouts.len() > 512 {
                layouts.clear();
            }
            layouts.insert(key, layout.clone());
        });
        layout
    }

    /// `fitted`'s work, for its input.
    fn laid_out(ui: &egui::Ui, input: &RowInput) -> Self {
        let (conc_dc, conditions) = (input.conc_dc, &input.words.conditions);
        let measure = |extra: &Extra| {
            let plain = conditions
                .iter()
                .filter(|c| !c.is_effect() || extra.as_plain)
                .count();
            let plain = extra
                .effects
                .plain_at_most
                .map_or(plain, |most| plain.min(most));
            Self::measured(ui, input, (plain, BonusFit::All, extra))
        };
        // X5, the ruling on PR 276: the row with each time taken away. Each effect shows as
        // its plain words, and an official condition as the condition it is. Its lines, its
        // height and the effects it shows by their words stay.
        let base = measure(&Extra {
            as_plain: true,
            ..Extra::default()
        });
        let mut effects: Vec<(usize, &str)> = conditions
            .iter()
            .enumerate()
            .filter(|(_, c)| c.is_effect())
            .map(|(at, c)| (at, c.name()))
            .collect();
        // W1-D1: on the row that has the turn, an effect due on it comes first.
        if input.turn {
            effects.sort_by_key(|(at, _)| !conditions[*at].due());
        }
        if effects.is_empty() && conc_dc.is_none() {
            return base;
        }
        let (effects, dc) = Self::choose(ui, &base, &effects, input, measure);
        if effects.shown.is_empty() && effects.rest == 0 && dc.is_none() {
            return base;
        }
        let layout = measure(&Extra {
            effects,
            conc_dc: dc,
            ..Extra::default()
        });
        Self {
            without_effects: Some(Box::new(base)),
            ..layout
        }
    }

    /// The row's rule of section 1.4, as a choice: the effects to show and whether the save's
    /// DC shows. Each form must keep the lines and the height of `base`, and show each official
    /// condition with a time that `base` shows. The first that does wins; with none, the row
    /// is `base`, which shows the effects as their words without their times.
    fn choose(
        ui: &egui::Ui,
        base: &Self,
        effects: &[(usize, &str)],
        input: &RowInput,
        measure: impl Fn(&Extra) -> Self,
    ) -> (EffectPart, Option<u32>) {
        let names = |layout: &Self| -> Vec<String> {
            layout
                .condition_spans
                .iter()
                .map(|(name, _)| name.clone())
                .filter(|name| !["Run conditions", "Run changes"].contains(&name.as_str()))
                .collect()
        };
        // An official condition that the row without times shows stays in view: as its words
        // with the count, or as the plain condition it was. Other words show in a form by
        // their words or in its count. Plain conditions fold as today.
        let shown: Vec<String> = names(base)
            .into_iter()
            .filter(|name| super::recognized_condition(name).is_some())
            .filter(|name| effects.iter().any(|(_, effect)| effect == name))
            .collect();
        let keeps = |layout: &Self| {
            let now = names(layout);
            layout.keeps_place_of(base, input.scale) && shown.iter().all(|name| now.contains(name))
        };
        let mut forms: Vec<EffectPart> = (0..=effects.len())
            .rev()
            .map(|whole| EffectPart {
                shown: effects[..whole]
                    .iter()
                    .map(|(at, name)| (*at, (*name).to_owned()))
                    .collect(),
                rest: effects.len() - whole,
                plain_at_most: None,
            })
            .collect();
        // The first effect alone, shortened to the room that the line leaves it with the plain
        // conditions folded to `most`, since the effects come first.
        let shortened = |most: usize| {
            let &(at, name) = effects.first()?;
            let rest = effects.len() - 1;
            let bare = EffectPart {
                shown: vec![(at, String::new())],
                rest,
                plain_at_most: Some(most),
            };
            let measured = measure(&Extra {
                effects: bare.clone(),
                ..Extra::default()
            });
            let line = measured.second_line.as_ref().filter(|_| keeps(&measured))?;
            let font = line.job.sections[0].format.font_id.clone();
            let room = line.job.wrap.max_width - line.size().x;
            let cut = design::words_that_fit(ui, name, &font, room);
            (cut != name && cut != "…" && design::number_width(ui, &cut, &font) <= room).then(
                || EffectPart {
                    shown: vec![(at, cut)],
                    ..bare
                },
            )
        };
        // At fewest one plain condition shows, as the row's folding leaves them.
        if let Some(mut part) = shortened(1) {
            part.plain_at_most = None;
            forms.insert(forms.len() - 1, part);
        }
        // W1-D1: an effect due on this turn keeps its first words before the last plain
        // condition folds into `N more`.
        let due = input.turn
            && effects
                .first()
                .is_some_and(|(at, _)| input.words.conditions[*at].due());
        if due && let Some(part) = shortened(0) {
            forms.insert(forms.len() - 1, part);
        }
        // The count of them all is the last form. With no effect, the one form is no part.
        let Some(effects) = forms.into_iter().find(|part| {
            keeps(&measure(&Extra {
                effects: part.clone(),
                ..Extra::default()
            }))
        }) else {
            return (EffectPart::default(), None);
        };
        let line = |layout: &Self| {
            layout
                .second_line
                .as_ref()
                .map(|line| line.text().to_owned())
        };
        let without = line(&measure(&Extra {
            effects: effects.clone(),
            ..Extra::default()
        }));
        // The DC folds nothing that shows: the line without it is the start of the line with
        // it. Else it is in the hover and the accessible name only.
        let dc = input.conc_dc.filter(|dc| {
            let with = measure(&Extra {
                effects: effects.clone(),
                conc_dc: Some(*dc),
                ..Extra::default()
            });
            keeps(&with)
                && line(&with)
                    .zip(without.clone())
                    .is_some_and(|(with, without)| without.is_empty() || with.starts_with(&without))
        });
        (effects, dc)
    }

    /// This row has the lines and the height of `base`, the same row with no effect: nothing
    /// that an effect adds may wrap, shrink the type, move the HP bar or grow the row.
    fn keeps_place_of(&self, base: &Self, scale: RunScale) -> bool {
        let rows = |layout: &Self| {
            layout
                .second_line
                .as_ref()
                .map_or(0, |line| line.rows.len())
        };
        let height = self.height(scale);
        rows(self) <= rows(base).max(1)
            && !self.detail_small
            && !base.detail_small
            && self.bar_inset == base.bar_inset
            && self.bar_below_detail == base.bar_below_detail
            && self.tight == base.tight
            && (height - base.height(scale)).abs() < 0.01
            && (self.line_y(scale, height) - base.line_y(scale, height)).abs() < 0.01
    }

    fn measured(
        ui: &egui::Ui,
        input: &RowInput,
        (condition_count, bonus_fit, extra): (usize, BonusFit, &Extra),
    ) -> Self {
        #[cfg(test)]
        ROW_LAYOUTS.with(|count| count.set(count.get() + 1));
        // The layout reads only its input: the cache of `fitted` is keyed on all of it.
        let RowInput {
            words: w,
            sync,
            scale,
            fit,
            width,
            longest_name,
            ..
        } = input;
        let (sync, scale, fit, width, longest_name) = (*sync, *scale, *fit, *width, *longest_name);
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
        // X10: the count of the rest ends the effects' slot after a space, with no dot.
        let dot = std::cell::Cell::new(true);
        // One part of the line: its words after ` · `, in one or more colours, and the room
        // after them that keeps a count's slot as wide as its widest form.
        let mut add_parts = |parts: &[(&str, egui::Color32)], pad: f32| {
            if !first {
                job.append(
                    if dot.replace(true) { " ·\u{a0}" } else { " " },
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
            for (text, color) in parts {
                job.append(
                    text,
                    0.0,
                    egui::TextFormat {
                        font_id: font.clone(),
                        color: *color,
                        ..Default::default()
                    },
                );
            }
            let end = job.text.chars().count();
            if pad > 0.0 {
                job.append("", pad, egui::TextFormat::simple(font.clone(), c.muted));
            }
            start..end
        };
        if inline_state && let Some((icon, word, color)) = w.hp_word {
            add_parts(&[(&*format!("{icon} {word}"), color)], 0.0);
        }
        if !w.present {
            add_parts(&[(&*format!("{} Absent", icons::ABSENT), c.muted)], 0.0);
        }
        let initiative_span =
            (w.present && !w.initiative).then(|| add_parts(&[("No initiative", c.accent)], 0.0));
        let mut conc_end = None;
        if w.concentration {
            let (words, color) = concentration_mark();
            conc_end = Some(add_parts(&[(&*words, color)], 0.0).end);
        }
        // Effects take the conditions' place, before the plain conditions (section 1.4). Each
        // count has a slot as wide as its widest form, so that a change of count moves no word.
        let mut condition_spans = Vec::new();
        let mut effect_spans = Vec::new();
        let mut end_pad = 0.0;
        for (at, words) in &extra.effects.shown {
            let condition = &w.conditions[*at];
            let (slot, forms) = count_slot(condition);
            let width = |parts: &[(String, egui::Color32)]| {
                let text: String = parts.iter().map(|(text, _)| text.as_str()).collect();
                design::number_width(ui, &text, &font)
            };
            // The slot holds the widest form, and a whole target (`XL`) after the gap in each
            // form, so the count's target never reaches over its neighbours. FK2 (iter-27-R):
            // a count keeps the room of its widest number in place, so `10` to `9` moves no word;
            // the rest of the room of its widest form (`· ⚠ time is up`) goes to the line's
            // end, so no hole opens between the effect and what follows it. The words after it
            // move only at `last round` and `time is up`, when the DM must look.
            let form_width = |form: &String| {
                let gap = if form.starts_with(" ·") {
                    " ·\u{a0}"
                } else {
                    " "
                };
                design::number_width(ui, form, &font)
                    .max(design::number_width(ui, gap, &font) + design::XL)
            };
            let widest = forms.iter().map(form_width).fold(0.0, f32::max);
            let numbers = forms
                .iter()
                .filter(|form| !form.starts_with(" ·"))
                .map(form_width)
                .fold(0.0, f32::max);
            let shown = width(&slot);
            let worded = slot.first().is_some_and(|(text, _)| text.starts_with(" ·"));
            let here = if worded {
                0.0
            } else {
                (numbers - shown).max(0.0)
            };
            end_pad += (widest - shown - here).max(0.0);
            let mut parts = vec![(words.as_str(), c.participant.condition)];
            parts.extend(slot.iter().map(|(text, color)| (text.as_str(), *color)));
            let span = add_parts(&parts, here);
            let name_end = span.start + words.chars().count();
            let whole = words == condition.name();
            if whole && super::recognized_condition(condition.name()).is_some() {
                condition_spans.push((condition.name().to_owned(), span.start..name_end));
            }
            let separator = slot.first().map_or(0, |(text, _)| text.chars().count());
            let target = match condition.time() {
                Some(Time::Rounds { left: 0, .. }) => EffectAct::Remove(*at),
                Some(Time::Uses { .. }) => EffectAct::Spend(*at),
                Some(Time::Recharge { .. }) => EffectAct::Recharge(*at),
                _ => EffectAct::Open,
            };
            if slot.is_empty() {
                effect_spans.push((target, *at, span.start..name_end));
            } else {
                effect_spans.push((target, *at, name_end + separator..span.end));
            }
        }
        if extra.effects.rest > 0 {
            let words = if extra.effects.shown.is_empty() {
                format!("{}\u{a0}{}", icons::PIN, extra.effects.rest)
            } else {
                dot.set(false);
                format!("{}\u{a0}+{}", icons::PIN, extra.effects.rest)
            };
            let span = add_parts(&[(&*words, c.participant.condition)], 0.0);
            effect_spans.push((EffectAct::Open, usize::MAX, span));
        }
        let plain: Vec<&super::Condition> = w
            .conditions
            .iter()
            .filter(|c| !c.is_effect() || extra.as_plain)
            .collect();
        for condition in plain.iter().take(condition_count) {
            let condition = condition.name();
            // FD8: a long effect shows its first four words and `…`, so the cut cannot be
            // read as a sentence. Its last word is the link to the whole effect.
            if condition.split_whitespace().count() > 4 {
                let shown = format!(
                    "{}…",
                    condition
                        .split_whitespace()
                        .take(4)
                        .collect::<Vec<_>>()
                        .join(" ")
                );
                let span = add_parts(&[(&*shown, c.participant.condition)], 0.0);
                let last = shown.rsplit(' ').next().unwrap_or(&shown).chars().count();
                condition_spans.push((condition.to_owned(), span.end - last..span.end));
            } else {
                let span = add_parts(&[(condition, c.participant.condition)], 0.0);
                condition_spans.push((condition.to_owned(), span));
            }
        }
        // A side of its own needs its word: a name and a number can look like another side's.
        if plain.len() > condition_count {
            condition_spans.push((
                "Run conditions".into(),
                add_parts(
                    &[(
                        &*design::count(plain.len() - condition_count, "more", "more")
                            .replace(' ', "\u{a0}"),
                        c.participant.condition,
                    )],
                    0.0,
                ),
            ));
        }
        if !["party", "enemies"].contains(&w.side.as_str()) {
            add_parts(&[(&*style::human_label(&w.side), c.side.other)], 0.0);
        }
        // A change is never hidden. What the run cannot apply is on the row, for the roll.
        // A number stays with its word when the line wraps.
        if let Some(name) = &w.as_name {
            add_parts(&[(&*format!("as {name}"), c.muted)], 0.0);
        }
        let mut bonus_start = None;
        if let Some(bonuses) = &w.bonuses
            && bonus_fit == BonusFit::None
            && !bonuses.is_empty()
        {
            // FJ3: with no change shown, `N more` would follow another kind (a condition or
            // the side) and read as more of it. The count names its own kind and is the link.
            condition_spans.push((
                "Run changes".into(),
                add_parts(
                    &[(
                        &*format!(
                            "{}\u{a0}{}",
                            icons::CHANGED,
                            design::count(bonuses.len(), "change", "changes")
                        )
                        .replace(' ', "\u{a0}"),
                        c.participant.changed,
                    )],
                    0.0,
                ),
            ));
        } else if let Some(bonuses) = &w.bonuses {
            add_parts(
                &[(
                    &*format!("{}\u{a0}Changed", icons::CHANGED),
                    c.participant.changed,
                )],
                0.0,
            );
            for bonus in bonuses.iter().take(match bonus_fit {
                BonusFit::None => 0,
                BonusFit::One => 1,
                BonusFit::All => bonuses.len(),
            }) {
                let span = add_parts(
                    &[(&*bonus.replace(' ', "\u{a0}"), c.participant.changed)],
                    0.0,
                );
                bonus_start.get_or_insert(span.start);
            }
            if bonus_fit == BonusFit::One && bonuses.len() > 1 {
                // FI10: a hidden count keeps its number, after a shown change of its kind.
                condition_spans.push((
                    "Run changes".into(),
                    add_parts(
                        &[(
                            &*design::count(bonuses.len() - 1, "more", "more")
                                .replace(' ', "\u{a0}"),
                            c.participant.changed,
                        )],
                        0.0,
                    ),
                ));
            }
        }
        // Down, with something to take: the DM rolls it from the card or the ended view.
        if w.loot {
            add_parts(&[(&*format!("{}\u{a0}Loot", icons::LOOT), c.accent)], 0.0);
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
        // Damage to a row marked Conc: its save, until the next step, in the room left.
        if let Some(dc) = extra.conc_dc {
            // R7 (FL6): right after the row's own `Conc` mark the save says `save DC 10`; the
            // hover and the accessible name keep `Conc save DC 10`, as does a save that does
            // not stand beside the mark.
            let beside_mark = conc_end == Some(job.text.chars().count());
            if !job.is_empty() {
                job.append(
                    " ·\u{a0}",
                    0.0,
                    egui::TextFormat::simple(font.clone(), c.muted),
                );
            }
            job.append(
                &if beside_mark {
                    format!("save DC {dc}")
                } else {
                    format!("Conc save DC {dc}")
                },
                0.0,
                egui::TextFormat::simple(font.clone(), c.participant.concentrating),
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
        // FK2: the counts' spare room, after the line's last word.
        if end_pad > 0.0 && !job.is_empty() {
            job.append("", end_pad, egui::TextFormat::simple(font.clone(), c.muted));
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
            && w.bonuses.is_some()
            && second_line.as_ref().is_some_and(|line| line.rows.len() > 1)
        {
            return Self::measured(
                ui,
                input,
                (
                    condition_count,
                    if bonus_fit == BonusFit::All {
                        BonusFit::One
                    } else {
                        BonusFit::None
                    },
                    extra,
                ),
            );
        }
        if condition_count > 1
            && (conditions_wrap || second_line.as_ref().is_some_and(|line| line.rows.len() > 1))
        {
            return Self::measured(ui, input, (condition_count - 1, bonus_fit, extra));
        }
        let mut bar_below_detail = false;
        let mut detail_small = false;
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
                detail_small = true;
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
                w.name.clone(),
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
            (name, name_cut) = name_lines(ui, &w.name, &font, name_width);
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
            effect_spans,
            detail_small,
            without_effects: None,
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
            input: None,
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

/// The y of a link's underline in its galley: 2 px below the baseline of the link's first
/// letter, as a reader link has it.
fn link_underline_y(galley: &egui::Galley, at: usize) -> Option<f32> {
    let cursor = galley.layout_from_cursor(egui::text::CCursor::new(at));
    let row = galley.rows.get(cursor.row)?;
    let glyph = row.glyphs.get(cursor.column)?;
    Some(row.pos.y + glyph.pos.y + design::HAIRLINE_WIDTH * 2.0)
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
        // A count's link is its last word (`more`, or `changes` with no change shown); a long
        // effect's span is already its last word.
        let link_start = if changes || conditions {
            let words: String = galley
                .text()
                .chars()
                .skip(span.start)
                .take(span.len())
                .collect();
            let last = words.rsplit([' ', '\u{a0}']).next().unwrap_or(&words);
            span.end.saturating_sub(last.chars().count())
        } else {
            span.start
        };
        let start = galley.pos_from_cursor(egui::text::CCursor::new(link_start));
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
        // FL9: 2 px below the words' baseline, as reader links do; at the row's bottom the line
        // touched the next row's turn stroke.
        let underline = link_underline_y(galley, link_start).map_or(word.bottom(), |y| {
            ui.painter().round_to_pixel_center(origin.y + y)
        });
        ui.painter().hline(
            word.x_range(),
            underline,
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

/// The right edge of the letter before character `end`, on that letter's row, when the
/// words from the row's start to it are on one row.
fn last_letter_right(galley: &Galley, end: usize) -> Option<f32> {
    let mut at = 0;
    for row in &galley.rows {
        let count = row.glyphs.len();
        if end > at && end <= at + count {
            let glyph = &row.glyphs[end - at - 1];
            return Some(row.pos.x + glyph.pos.x + glyph.advance_width);
        }
        at += count + usize::from(row.ends_with_newline);
    }
    None
}

/// An effect target's words and its target: from the words' first letter, at least `XL`
/// wide, into the room that the count's slot keeps after them. It never starts before the
/// words, so the condition's name before a count keeps its own target.
fn effect_target(
    galley: &Galley,
    span: &std::ops::Range<usize>,
    origin: egui::Pos2,
    rect: Rect,
) -> Option<(Rect, Rect)> {
    let start = galley.pos_from_cursor(egui::text::CCursor::new(span.start));
    // The words end at their last letter, before the room that keeps a count's slot.
    let right = last_letter_right(galley, span.end)?;
    let word =
        Rect::from_min_max(start.min, pos2(right, start.bottom())).translate(origin.to_vec2());
    let bottom = (word.center().y + design::XL / 2.0).min(rect.bottom());
    let width = word.width().max(design::XL);
    let target = Rect::from_min_max(
        pos2(word.left(), bottom - design::XL),
        pos2(word.left() + width, bottom),
    );
    Some((word, target))
}

/// Each target of a row's effects: a count, words shown on each turn, or the count of the
/// rest. Each is its own target, at least `XL` wide, so the name beside it keeps its act.
/// Returns the act the DM asked for, and the effect's place.
fn effect_links(
    ui: &mut egui::Ui,
    rect: Rect,
    p: &Participant,
    layout: &RowLayout,
    scale: RunScale,
) -> Option<(EffectAct, usize)> {
    let galley = layout.second_line.as_ref()?;
    let origin = pos2(
        rect.left() + layout.second_left,
        layout.second_y(scale, rect),
    );
    let mut asked = None;
    for (act, at, span) in &layout.effect_spans {
        let condition = p.conditions.get(*at);
        let effect = condition.map_or("", super::Condition::name);
        let name = match (act, condition.and_then(super::Condition::time)) {
            (EffectAct::Remove(_), _) => format!("Remove {effect}"),
            (EffectAct::Spend(_), Some(Time::Uses { left: 0, .. })) => {
                format!("Give back every use of {effect} of {}", p.name)
            }
            (EffectAct::Spend(_), _) => format!("Spend one use of {effect} of {}", p.name),
            (EffectAct::Recharge(_), Some(Time::Recharge { spent: false, .. })) => {
                format!("Mark {effect} spent")
            }
            (EffectAct::Recharge(_), Some(Time::Recharge { recharge, .. })) => {
                let range = if recharge[0] == recharge[1] {
                    recharge[0].to_string()
                } else {
                    format!("{}–{}", recharge[0], recharge[1])
                };
                format!("Roll 1d6. {range} makes it ready")
            }
            // A running count, words shown on each turn, or the count of the rest (no place).
            _ => {
                let words = condition.map_or_else(
                    || {
                        p.conditions
                            .iter()
                            .filter(|c| c.is_effect())
                            .map(super::Condition::words)
                            .collect::<Vec<_>>()
                            .join(" · ")
                    },
                    super::Condition::words,
                );
                format!("{words}. Remove it in Conditions")
            }
        };
        let Some((word, target)) = effect_target(galley, span, origin, rect) else {
            continue;
        };
        let response = ui
            .interact(
                target,
                ui.id().with(("effect", &p.key, *at, span.start)),
                egui::Sense::click(),
            )
            .on_hover_text(&name)
            .on_hover_cursor(egui::CursorIcon::PointingHand);
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
            asked = Some((*act, *at));
        }
    }
    asked
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
            // R8: an empty line (every word gave way to the result) starts at its edge.
            if row.glyphs.is_empty() {
                (0.0, row.rect().top())
            } else {
                (row.rect().right() + design::S, row.rect().top())
            }
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
    let picture = layout.picture.min(rect.height() - 4.0);
    // FJ3: the picture centres on the first name line, as the initiative number does, on every
    // row with or without a second line; it only stays inside its row.
    let picture_y = line_y.clamp(
        rect.top() + 2.0 + picture / 2.0,
        (rect.bottom() - 2.0 - picture / 2.0).max(rect.top() + 2.0 + picture / 2.0),
    );
    crate::art::portrait(
        ui,
        Rect::from_center_size(
            pos2(name_left - PICTURE_GAP - layout.picture / 2.0, picture_y),
            egui::Vec2::splat(picture),
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
    let size = vec2(correction_label_width(ui), Density::Read.control());
    ui.allocate_ui_with_layout(size, egui::Layout::left_to_right(Align::Center), |ui| {
        ui.set_min_size(size);
        ui.label(Type::Body.rich(ui, label).color(colors().text));
    });
}

/// A Correct reason on its own line below its field, starting at the field. The line stays
/// while the reason is gone, so nothing below moves as the DM types.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn correction_reason(ui: &mut egui::Ui, reason: &str, shown: bool) {
    ui.horizontal(|ui| {
        ui.add_space(correction_label_width(ui) + ui.spacing().item_spacing.x);
        let words = Type::Small.rich(ui, reason).color(colors().muted);
        if shown {
            ui.add(egui::Label::new(words).wrap());
        } else {
            let size = egui::WidgetText::from(words)
                .into_galley(
                    ui,
                    Some(egui::TextWrapMode::Wrap),
                    ui.available_width(),
                    egui::TextStyle::Body,
                )
                .size();
            ui.allocate_exact_size(size, egui::Sense::hover());
        }
    });
}

/// Every label the Correct tab has, so they share one column as wide as the widest.
const CORRECTION_LABELS: [&str; 5] = [
    "Current HP",
    "Maximum HP",
    "Initiative bonus",
    "Initiative",
    "Note",
];

/// The Correct tab's label column: the widest label and an `S` gap, and never under `L * 6`.
fn correction_label_width(ui: &egui::Ui) -> f32 {
    let font = Type::Body.font(ui);
    CORRECTION_LABELS
        .iter()
        .map(|label| design::number_width(ui, label, &font) + design::S)
        .fold(design::L * 6.0, f32::max)
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

#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
)]
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
    /// V26, V27 and the review of PR 275: beside Continue fight on the ended header's second
    /// line, a target too long for the line stays inside the line, and the hover and the
    /// accessible name hold the whole words, as the one-line header does. X6: it shortens by
    /// the one rule of Undo labels, so a table name that is one long word goes whole and the
    /// act stays (`Undo: 2 loot rolls`), never cut inside a word.
    #[test]
    fn ended_undo_long_target_ends_in_view_with_its_whole_words_in_hover_and_name() {
        use egui_kittest::kittest::Queryable;
        let mut run = super::super::tests::run();
        run.state.phase = super::super::Phase::Ended {
            outcome: "Party won".into(),
        };
        run.state.round = 2;
        run.history.push(run.state.clone());
        let target =
            "2 loot rolls on treasure-from-the-ancient-dragon-vault-and-the-forgotten-kings-tomb";
        run.history_labels.push(target.into());
        run.history_from_sheet.push(false);
        // The step's target, as `describe_named` records it for one loot table.
        run.history_targets.push(Some(
            "treasure-from-the-ancient-dragon-vault-and-the-forgotten-kings-tomb".into(),
        ));
        run.lookup_open = true;
        run.hide_panel = true;
        let full = format!("Undo: {target}");
        let run = std::rc::Rc::new(std::cell::RefCell::new(run));
        let shown = run.clone();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(454.0, 640.0))
            .build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    shown.borrow_mut().ended_view(ui);
                });
            });
        crate::design::theme::install(&harness.ctx);
        harness.run_steps(4);
        let painted: Vec<_> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text.starts_with("Undo") => {
                    let glyphs: String = text
                        .galley
                        .rows
                        .iter()
                        .flat_map(|row| row.glyphs.iter().map(|g| g.chr))
                        .collect();
                    Some((glyphs, text.visual_bounding_rect(), shape.clip_rect))
                }
                _ => None,
            })
            .collect();
        assert_eq!(painted.len(), 1, "one Undo: {painted:?}");
        let (glyphs, rect, clip) = &painted[0];
        assert!(
            clip.contains_rect(*rect),
            "Undo is cut by its clip: {painted:?}"
        );
        assert_eq!(
            glyphs, "Undo: 2 loot rolls",
            "Undo keeps its act in the line's room: {glyphs:?}"
        );
        // The accessible name, and the hover, hold the whole words.
        harness.get_by_label(&full).hover();
        harness.run_steps(60);
        assert_eq!(
            harness.query_all_by_label(&full).count(),
            2,
            "Undo's hover holds its whole words"
        );
        // The setup view's Undo has the same rule: a long name shortens by whole words, never
        // inside a word, and the whole step is its accessible name.
        let mut setup = super::super::tests::run();
        let name = "Oren Thistlewick, Keeper of the Southern Road";
        setup.state.participants[0].name = name.into();
        setup.history.push(setup.state.clone());
        setup.history_labels.push(format!("initiative for {name}"));
        setup.history_targets.push(Some(name.into()));
        setup.history_from_sheet.push(false);
        setup.lookup_open = true;
        setup.hide_panel = true;
        let full = format!("Undo: initiative for {name}");
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(454.0, 640.0))
            .build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    setup.setup_view(ui);
                });
            });
        crate::design::theme::install(&harness.ctx);
        harness.run_steps(4);
        let painted: Vec<_> = harness
            .output()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text.starts_with("Undo") => {
                    Some((text.galley.job.text.clone(), text.galley.elided))
                }
                _ => None,
            })
            .collect();
        assert!(
            matches!(&painted[..], [(words, false)]
                if words.starts_with("Undo: initiative for Oren") && keeps_names(words)),
            "the setup Undo is one form of the rule: {painted:?}"
        );
        assert!(harness.query_by_label(&full).is_some(), "{full}");
        // X38 and review round 1 of PR 286: at a width, the ended header has one height for
        // every target, loot key and reason, the fight played last or not; its controls stay
        // whole and apart, and the reason touches none of them.
        let key = "treasure-from-the-ancient-dragon-vault-and-the-forgotten-kings-tomb";
        for width in [454.0, 608.0, 800.0, 1040.0, 1760.0] {
            let mut tops = Vec::new();
            for long in [false, true] {
                for older in [false, true] {
                    for action in [
                        "end encounter",
                        "2 loot rolls on gems",
                        "loot roll on coins",
                    ] {
                        let mut run = super::super::tests::run();
                        run.state.phase = super::super::Phase::Ended {
                            outcome: "Party won".into(),
                        };
                        run.state.round = 2;
                        if long {
                            run.setup_details = format!("## Loot\n- Roll 2× {key}\n");
                        }
                        run.history.push(run.state.clone());
                        run.history_labels.push(action.into());
                        run.history_from_sheet.push(false);
                        run.hide_panel = width <= 608.0;
                        run.lookup_open = width <= 608.0;
                        if older {
                            run.continue_blocked = Some(super::super::CONTINUE_LATER);
                            run.saved_steps = run.history.len();
                        }
                        let mut harness = egui_kittest::Harness::builder()
                            .with_size(egui::vec2(width, 800.0))
                            .build(move |ctx| {
                                egui::CentralPanel::default().show(ctx, |ui| {
                                    run.ended_view(ui);
                                });
                            });
                        crate::design::theme::install(&harness.ctx);
                        harness.run_steps(4);
                        let top = harness
                            .output()
                            .shapes
                            .iter()
                            .find_map(|shape| match &shape.shape {
                                egui::Shape::Text(text)
                                    if text.galley.job.text == "FINAL STATE" =>
                                {
                                    Some(text.pos.y)
                                }
                                _ => None,
                            })
                            .expect("FINAL STATE");
                        let full = format!("Undo: {action}");
                        let controls: Vec<_> = ["Close run", "Continue fight", full.as_str()]
                            .into_iter()
                            .filter_map(|label| harness.query_by_label(label).map(|n| n.rect()))
                            .collect();
                        assert!(
                            controls
                                .iter()
                                .all(|r| r.width() >= 24.0 && r.height() >= 24.0),
                            "a small header target at {width}: {controls:?}"
                        );
                        for (at, a) in controls.iter().enumerate() {
                            assert!(
                                controls[at + 1..].iter().all(|b| !a.intersects(*b)),
                                "header controls overlap at {width}: {controls:?}"
                            );
                        }
                        if older {
                            let reason = harness.get_by_label(super::super::CONTINUE_LATER).rect();
                            assert!(
                                controls.iter().all(|r| !r.intersects(reason)),
                                "the reason overlaps a control at {width}: {reason:?} {controls:?}"
                            );
                        }
                        tops.push((top, long, older, action));
                    }
                }
            }
            assert!(
                tops.iter().all(|(top, ..)| *top == tops[0].0),
                "the ended header's height at {width}: {tops:?}"
            );
        }
    }

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
                        let arrival = run.arrival_block(ui);
                        run.group_list(ui, arrival);
                    });
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            // iter-25-R review 2: a frame scans the participants once per group for the panel's
            // width and once per group for its row, and builds no list of rounds besides.
            let scans = || GROUP_SCANS.with(std::cell::Cell::get);
            let (before, passes) = (scans(), harness.ctx.cumulative_pass_nr());
            harness.run_steps(3);
            let passes = (harness.ctx.cumulative_pass_nr() - passes) as usize;
            assert!(passes > 0);
            assert_eq!(
                scans() - before,
                passes * 2 * 2,
                "two scans for each of 2 groups in each of {passes} passes"
            );
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
    fn iter10_b_arrival_controls_share_one_edge_and_bad_totals_warn_once() {
        use egui_kittest::kittest::Queryable;
        let names = ["Ash Hounds", "Shrine Reserve", "Lost Patrol"];
        let mut edges = Vec::new();
        for width in [440.0, 600.0, 760.0] {
            let run = super::super::tests::run();
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
            // Two faults at once: a second bad total, on the group below it.
            let also_bad = groups[2].id;
            // The whole Arrivals tab, so its footer is drawn too. `clean` has no bad total.
            let edge = std::rc::Rc::new(std::cell::Cell::new(0.0));
            let build = |mut run: Run, bad_total: bool| {
                run.groups = Some(Arc::new(groups.clone()));
                if bad_total {
                    run.group_totals.insert(bad, "1-2".into());
                    run.group_totals.insert(also_bad, "2-5".into());
                }
                let seen = edge.clone();
                let mut harness = egui_kittest::Harness::builder()
                    .with_size(vec2(width, 800.0))
                    .build(move |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            seen.set(ui.max_rect().right());
                            run.groups_tab(ui);
                        });
                    });
                design::theme::install(&harness.ctx);
                harness.run_steps(2);
                harness
            };
            let clean = build(super::super::tests::run(), false);
            let clean_field = clean
                .get_by_label("Initiative total for Lost Patrol")
                .rect();
            drop(clean);
            let mut harness = build(run, true);
            // Astra 2: an unchanged frame works out no summary and no reason again.
            let prepared = PREPARED.with(std::cell::Cell::get);
            harness.run_steps(3);
            let again = PREPARED.with(std::cell::Cell::get);
            assert!(
                prepared.0 > 0 && prepared.1 > 0 && again.0 == prepared.0 && again.1 == prepared.1,
                "iter-26-R-prepared at {width}: {prepared:?} then {again:?}"
            );
            let edge = edge.get();
            // FI5: the reason is in the footer, so the group below the bad total keeps its y.
            let field = harness
                .get_by_label("Initiative total for Lost Patrol")
                .rect();
            assert!(
                (field.top() - clean_field.top()).abs() <= 0.5,
                "iter-26-R-FI5 at {width}: {clean_field:?} then {field:?}"
            );
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
            edges.push(buttons[0].right());
            for (button, field) in buttons.iter().zip(&fields) {
                // AH1: one Add edge for every group, at the group block's edge, inside the pane.
                assert!(
                    (button.right() - buttons[0].right()).abs() <= 0.5
                        && button.right() <= edge + 0.5,
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
            // Once for each bad total: two groups, two reasons.
            assert_eq!(texts.len(), 2, "iter10-B-FT4-once at {width}");
            // FI5: in the footer, each reason names its group, in list order.
            let named: Vec<_> = texts.iter().map(|text| text.galley.text()).collect();
            assert!(
                named[0].starts_with("Shrine Reserve: Enter a whole number")
                    && named[1].starts_with("Lost Patrol: Enter a whole number")
                    && texts[0].pos.y < texts[1].pos.y,
                "iter-26-R-FI5-named at {width}: {named:?}"
            );
            for text in &texts {
                assert!(
                    text.galley
                        .job
                        .sections
                        .iter()
                        .all(|section| section.format.color == colors().text),
                    "iter10-B-FT4-text-colour at {width}"
                );
                assert!(
                    !text.galley.text().contains(icons::WARN),
                    "iter10-B-FT4-mark-apart at {width}"
                );
            }
        }
        // AH1: the block is as wide as its groups need, not the pane: in a pane wider than that,
        // Add stays where it was.
        assert!(
            (edges[1] - edges[2]).abs() <= 0.5,
            "iter-23-R-AH1: Add follows the group block, not the pane: {edges:?}"
        );
        // Astra 4: ten bad totals at 960x640. The reasons sit in the shared footer: at most a
        // third of the panel, and the last reason is reached by Tab and the arrows, and by the
        // wheel.
        let crowded = |_: ()| {
            let mut run = super::super::tests::run();
            let groups: Vec<_> = (1..=10)
                .map(|at| {
                    Arc::new(super::super::RunGroup {
                        id: Uuid::new_v4(),
                        path: format!("Group {at}.md"),
                        name: format!("Group {at}"),
                        side: "enemies".into(),
                        encounter: None,
                        encounter_name: None,
                        members: vec![super::super::GroupMember {
                            entry: run.state.participants[0].definition.clone(),
                            count: 1,
                            overrides: Default::default(),
                        }],
                        notes: String::new(),
                        xp: 0,
                        xp_unknown: vec![],
                        unavailable: 0,
                        unavailable_members: vec![],
                        problem: None,
                        warnings: vec![],
                    })
                })
                .collect();
            for group in &groups {
                run.group_totals.insert(group.id, "2-5".into());
            }
            run.groups = Some(Arc::new(groups));
            let focus = egui::Id::new(("arrival_footer", run.id)).with("focus");
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(440.0, 640.0))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| run.groups_tab(ui));
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(3);
            (harness, focus)
        };
        const LAST: &str = "Group 10: Enter a whole number, or clear the field to roll.";
        let footer = |harness: &egui_kittest::Harness<'_>, focus: egui::Id| {
            harness
                .ctx
                .read_response(focus)
                .map(|response| response.rect)
        };
        let shown = |harness: &egui_kittest::Harness<'_>, focus: egui::Id| {
            let footer = footer(harness, focus);
            harness
                .output()
                .shapes
                .iter()
                .any(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.job.text == LAST => {
                        let rect = Rect::from_min_size(text.pos, text.galley.size());
                        footer.is_some_and(|footer| {
                            shape
                                .clip_rect
                                .intersect(footer)
                                .expand(0.5)
                                .contains_rect(rect)
                        })
                    }
                    _ => false,
                })
        };
        let (mut keyboard, focus) = crowded(());
        let rect = footer(&keyboard, focus).expect("the reasons have a footer");
        assert!(
            rect.height() <= 640.0 / 3.0 + 1.0 && !shown(&keyboard, focus),
            "iter-26-R-crowded-cap: footer {rect:?}, last reason hidden before scrolling"
        );
        for _ in 0..80 {
            if keyboard.ctx.memory(|memory| memory.has_focus(focus)) {
                break;
            }
            keyboard.key_press(egui::Key::Tab);
            keyboard.run_steps(2);
        }
        for _ in 0..40 {
            if shown(&keyboard, focus) {
                break;
            }
            keyboard.key_press(egui::Key::ArrowDown);
            keyboard.run_steps(2);
        }
        assert!(
            keyboard.ctx.memory(|memory| memory.has_focus(focus)) && shown(&keyboard, focus),
            "iter-26-R-crowded-keyboard: Tab and the arrows reach the last reason"
        );
        let (mut wheel, focus) = crowded(());
        let over = footer(&wheel, focus)
            .expect("the reasons have a footer")
            .center();
        wheel.event(egui::Event::PointerMoved(over));
        for _ in 0..40 {
            if shown(&wheel, focus) {
                break;
            }
            wheel.event(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: vec2(0.0, -Type::Body.line()),
                modifiers: egui::Modifiers::NONE,
            });
            wheel.run_steps(2);
        }
        assert!(
            shown(&wheel, focus),
            "iter-26-R-crowded-wheel: the wheel reaches the last reason"
        );
        // Astra re-check: a reload replaces the groups while Arrivals is closed, several times
        // (each old read dropped). Reopened, Arrivals shows the current members and reasons, and
        // Add adds the current members.
        use std::{cell::RefCell, rc::Rc};
        let (pack, vanguard) = (Uuid::new_v4(), Uuid::new_v4());
        // A member with a challenge rating, so each read has XP and difficulty words.
        let rated = |run: &Run| {
            let mut entry = (*run.state.participants[0].definition).clone();
            entry.fields.insert(
                "splat_creature".into(),
                serde_yaml_ng::from_str("{type: humanoid, cr_text: '1'}").unwrap(),
            );
            Arc::new(entry)
        };
        let read = |rated: &Arc<Entry>, pack_name: &str, count: u64| {
            let group = |id: Uuid, name: &str| {
                Arc::new(super::super::RunGroup {
                    id,
                    path: format!("{name}.md"),
                    name: name.into(),
                    side: "enemies".into(),
                    encounter: None,
                    encounter_name: None,
                    members: vec![super::super::GroupMember {
                        entry: rated.clone(),
                        count,
                        overrides: Default::default(),
                    }],
                    notes: String::new(),
                    xp: 0,
                    xp_unknown: vec![],
                    unavailable: 0,
                    unavailable_members: vec![],
                    problem: None,
                    warnings: vec![],
                })
            };
            Arc::new(vec![group(pack, pack_name), group(vanguard, "Vanguard")])
        };
        let mut run = super::super::tests::run();
        // Every creature's XP is known, so the fight has a word.
        let creature = rated(&run);
        for p in &mut run.state.participants {
            p.definition = creature.clone();
        }
        run.groups = Some(read(&creature, "Pack A", 2));
        // A scale, so each read has difficulty words (#262), which must follow the read too.
        run.scale = Some(Arc::new(crate::difficulty::Scale {
            budgets: Some(crate::difficulty::builtin_budgets()),
            party: [(Uuid::new_v4(), Some(4))].into_iter().collect(),
        }));
        run.group_totals.insert(pack, "2-5".into());
        run.group_totals.insert(vanguard, "12".into());
        let shared = Rc::new(RefCell::new((run, true)));
        let drawn = shared.clone();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(760.0, 800.0))
            .build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let (run, open) = &mut *drawn.borrow_mut();
                    if *open {
                        run.groups_tab(ui);
                    }
                });
            });
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        let texts = |harness: &egui_kittest::Harness<'_>| {
            harness
                .output()
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                    _ => None,
                })
                .collect::<Vec<_>>()
        };
        assert!(
            texts(&harness)
                .iter()
                .any(|text| text.starts_with("Pack A: "))
                && texts(&harness)
                    .iter()
                    .any(|text| text.contains("2\u{a0}×\u{a0}")),
            "the first read shows: {:?}",
            texts(&harness)
        );
        let first_words = shared.borrow_mut().0.arrival_words().cloned();
        // Closed, then two reloads back to back, each dropping the read before: the last read
        // takes the first one's freed allocation, as a reload can. Nothing is allocated between.
        shared.borrow_mut().1 = false;
        harness.run_steps(2);
        {
            let (run, _) = &mut *shared.borrow_mut();
            run.groups = Some(read(&creature, "Pack B", 4));
            run.groups = Some(read(&creature, "Pack D", 3));
        }
        harness.run_steps(2);
        shared.borrow_mut().1 = true;
        harness.run_steps(3);
        // #262's difficulty words follow the current read: the kept words are the ones worked
        // out fresh for it, and they differ from the first read's.
        let (kept, fresh) = {
            let run = &mut shared.borrow_mut().0;
            let kept = run.arrival_words().cloned();
            run.arrival_words = None;
            (kept, run.arrival_words().cloned())
        };
        assert!(
            fresh.as_ref().is_some_and(|words| !words.is_empty())
                && fresh != first_words
                && kept == fresh,
            "iter-26-R-reload-words: first {first_words:?}, kept {kept:?}, fresh {fresh:?}"
        );
        let now = texts(&harness);
        let members: Vec<_> = now
            .iter()
            .filter(|text| text.contains("\u{a0}×\u{a0}"))
            .collect();
        assert!(
            members.len() == 2
                && members
                    .iter()
                    .all(|text| text.starts_with("3\u{a0}×\u{a0}"))
                && now.iter().any(|text| text.starts_with("Pack D: "))
                && !now.iter().any(|text| {
                    ["Pack A: ", "Pack B: "]
                        .iter()
                        .any(|old| text.starts_with(old))
                }),
            "iter-26-R-reload: reopened Arrivals shows the current read: {now:?}"
        );
        let before = shared.borrow().0.state.participants.len();
        harness
            .get_by_label("Add Vanguard to the initiative list")
            .click();
        harness.run_steps(3);
        let after = shared.borrow().0.state.participants.len();
        assert_eq!(
            after,
            before + 3,
            "iter-26-R-reload-add: Add adds the current 3 members"
        );
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
                .map(super::super::Condition::from)
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
                // F2, section 1.4: effects take the conditions' place, before the plain ones,
                // and a row keeps the lines and the height of the same row with each time taken
                // away (X5: each effect as its plain words). What does not fit is a count.
                use super::super::Condition;
                let mut q = p.clone();
                q.conditions = vec![
                    Condition::from("Poisoned"),
                    Condition::read("Bless 10 rounds"),
                    Condition::read("Wis save DC 15 (Hold Person) on its turn"),
                    Condition::read("Frightened 10 rounds"),
                ];
                let mut plain = q.clone();
                plain.conditions.retain(|c| !c.is_effect());
                let mut labels = q.clone();
                labels.conditions = q
                    .conditions
                    .iter()
                    .map(|c| Condition::from(c.name()))
                    .collect();
                let mut counted = 0;
                let row = |width, q, scale, fit, turn| {
                    RowLayout::fitted(ui, width, q, (None, None, turn), scale, fit, 0.0)
                };
                for (width, scale) in [
                    (442.0, design::RUN_REGULAR),
                    (596.0, design::RUN_REGULAR),
                    (442.0, design::RUN_COMPACT),
                    (1040.0, design::RUN_LARGE),
                ] {
                    for (fit, turn) in [RowFit::Natural, RowFit::Compressed, RowFit::Summary]
                        .into_iter()
                        .flat_map(|fit| [(fit, false), (fit, true)])
                    {
                        let base = RowLayout::new(ui, width, &labels, None, scale, fit, 0.0);
                        let layout = row(width, &q, scale, fit, turn);
                        assert_eq!(layout.height(scale), base.height(scale), "{width} {fit:?}");
                        let rows =
                            |l: &RowLayout| l.second_line.as_ref().map_or(0, |g| g.rows.len());
                        assert!(rows(&layout) <= rows(&base).max(1), "{width} {fit:?}");
                        let text = layout.second_line.as_ref().unwrap().text();
                        // The first effect leads, before the plain condition. W1-D1: on the
                        // row that has the turn, the effect due on it leads.
                        let (lead, other) = if turn {
                            ("Wis", "Bless")
                        } else {
                            ("Bless", "Wis")
                        };
                        if let Some(at) = text.find(other) {
                            assert!(text.find(lead).is_some_and(|lead| lead < at), "{text}");
                        }
                        if let (Some(effect), Some(poisoned)) =
                            (text.find(lead), text.find("Poisoned"))
                        {
                            assert!(effect < poisoned, "{text}");
                        }
                        if text.contains(icons::PIN) {
                            counted += 1;
                            assert!(!text.contains("Frightened 10"), "{text}");
                            // X10: a count of the rest follows its effects' slot after a
                            // space, with no dot that reads as a mark of the column above.
                            assert!(
                                !text.contains(&format!("·\u{a0}{}\u{a0}+", icons::PIN)),
                                "{width} {fit:?}: {text}"
                            );
                        }
                    }
                }
                assert!(counted > 0, "some width counts the effects that do not fit");
                // W1-D1: the effect due on the turn keeps its first words before the last plain
                // condition folds into `1 more`, at some width; a row without the turn never
                // folds its one plain condition for an effect.
                let mut folded = 0;
                for width in 300..700 {
                    for scale in [design::RUN_COMPACT, design::RUN_REGULAR] {
                        let line = |turn| {
                            let layout = row(width as f32, &q, scale, RowFit::Natural, turn);
                            layout.second_line.unwrap().text().to_owned()
                        };
                        let (turn, other) = (line(true), line(false));
                        assert!(other.contains("Poisoned"), "{width}: {other}");
                        let due = turn.find("Wis");
                        if due.is_some_and(|due| {
                            turn.find("1\u{a0}more").is_some_and(|more| due < more)
                        }) {
                            folded += 1;
                        }
                    }
                }
                assert!(
                    folded > 0,
                    "W1-D1: the due effect's words outrank a plain condition"
                );
                // FK2 (iter-27-R): a count keeps the room of its widest number in place, so `10`,
                // `9` and `2` move no word after it; the rest of its widest form's room is at the
                // line's end, so no hole sits between the count and the next word. The words
                // after it move only at `last round` and `time is up`, and the line keeps its
                // lines in every form.
                let mut timed = p.clone();
                timed.initiative = Some(10);
                let mut at = Vec::new();
                let mut lines = Vec::new();
                let font = Type::Strong.sized(ui, design::RUN_REGULAR.condition);
                let joint = design::number_width(ui, " ·\u{a0}", &font);
                for left in [10, 9, 2, 1, 0] {
                    let mut effect = Condition::read("Bless 10 rounds");
                    if let Condition::Effect(super::super::effect::Effect {
                        time: Time::Rounds { left: l, .. },
                        ..
                    }) = &mut effect
                    {
                        *l = left;
                    }
                    timed.conditions = vec![effect, Condition::from("Prone")];
                    let layout = RowLayout::new(
                        ui,
                        900.0,
                        &timed,
                        None,
                        design::RUN_REGULAR,
                        RowFit::Natural,
                        0.0,
                    );
                    let line = layout.second_line.unwrap();
                    let form = match left {
                        0 => "time is up".to_owned(),
                        1 => "last round".to_owned(),
                        n => n.to_string(),
                    };
                    assert!(line.text().starts_with("Bless"), "{}", line.text());
                    assert!(line.text().contains(&form), "{form}: {}", line.text());
                    assert!(line.text().ends_with("Prone"), "{}", line.text());
                    let prone = line.text().chars().count() - "Prone".chars().count();
                    at.push(line.pos_from_cursor(egui::text::CCursor::new(prone)).min.x);
                    lines.push(line.rows.len());
                    if left == 10 {
                        // No hole: from the end of `10` to `Prone` is the joint and, at most,
                        // the rest of the count's whole target (`XL`), not the room of
                        // `· ⚠ time is up`.
                        let count = line.text().find("10").unwrap();
                        // The right of the `0` glyph: a cursor after it stands after any room
                        // the line keeps there.
                        let zero = line.text()[..count].chars().count() + 1;
                        let end = line.pos_from_cursor(egui::text::CCursor::new(zero)).min.x
                            + design::number_width(ui, "0", &font);
                        assert!(
                            at[0] - end <= joint + design::XL,
                            "iter-27-R-FK2: {end} to {} is more than {joint}",
                            at[0]
                        );
                    }
                }
                // Whole pixels: a form measured alone and the same form in the line can round
                // apart by one pixel.
                assert!(at[..3].iter().all(|x| (x - at[0]).abs() <= 1.0), "{at:?}");
                assert!(lines.iter().all(|n| *n == lines[0]), "{lines:?}");
                // Rule P, as one rule (reviews of PR 276): a row's height and lines are those of
                // the same row with each time taken away: each effect is its plain words, and an
                // official condition with a time is that condition. So no change of a time moves
                // a row: none to a timed effect, a plain condition given a time, a count down,
                // `time is up`, a time removed. X5: so an effect shows wherever its plain words
                // show, also beside a name of three lines.
                let mut guard = p.clone();
                guard.name = "Guard".into();
                guard.initiative = Some(10);
                guard.hp = guard.max_hp;
                guard.temporary_hp = 0;
                let rounds = |words: &str, left: u32| {
                    let mut timed = Condition::read(words);
                    if let Condition::Effect(super::super::effect::Effect {
                        time: Time::Rounds { left: l, .. },
                        ..
                    }) = &mut timed
                    {
                        *l = left;
                    }
                    timed
                };
                let states: Vec<Vec<Condition>> = vec![
                    vec![rounds("Poisoned 10 rounds", 10)],
                    vec![rounds("Poisoned 10 rounds", 1)],
                    vec![rounds("Poisoned 10 rounds", 0)],
                    vec![rounds("Bless 10 rounds", 9)],
                    vec![rounds("Bless 10 rounds", 0), Condition::from("Prone")],
                    vec![
                        Condition::read("Wis save DC 15 (Hold Person) on its turn"),
                        rounds("Frightened 3 rounds", 2),
                        Condition::from("Prone"),
                    ],
                    vec![
                        Condition::read("Legendary actions 3 uses"),
                        Condition::read("Fire Breath recharge 5-6"),
                    ],
                ];
                let official =
                    |c: &Condition| super::super::recognized_condition(c.name()).is_some();
                let long = "Seraphina Thornwood-Ashgrave of the Ninth Bell";
                for (state, name) in states
                    .iter()
                    .flat_map(|state| [(state, "Guard"), (state, long)])
                {
                    let mut timed = guard.clone();
                    timed.name = name.into();
                    timed.conditions = state.clone();
                    let mut untimed = timed.clone();
                    untimed.conditions = state.iter().map(|c| Condition::from(c.name())).collect();
                    for width in [300.0, 442.0, 596.0, 900.0, 1040.0] {
                        for scale in [design::RUN_COMPACT, design::RUN_REGULAR] {
                            for fit in [RowFit::Natural, RowFit::Compressed, RowFit::Summary] {
                                let with = RowLayout::new(ui, width, &timed, None, scale, fit, 0.0);
                                let without =
                                    RowLayout::new(ui, width, &untimed, None, scale, fit, 0.0);
                                let rows = |l: &RowLayout| {
                                    l.second_line.as_ref().map_or(0, |g| g.rows.len())
                                };
                                let words = |l: &RowLayout| {
                                    l.second_line.as_ref().map(|g| g.text().to_owned())
                                };
                                assert_eq!(
                                    with.height(scale),
                                    without.height(scale),
                                    "{width} {fit:?} {:?}: {:?} / {:?}",
                                    scale.row,
                                    words(&without),
                                    words(&with)
                                );
                                assert!(rows(&with) <= rows(&without).max(1));
                                // An effect that shows without its time still shows: an official
                                // condition by its whole name, other words by their first word
                                // or in the row's count of effects. Never nothing.
                                for c in state.iter().filter(|c| c.is_effect()) {
                                    let first = c.name().split(' ').next().unwrap();
                                    if words(&without).is_some_and(|w| w.contains(first)) {
                                        let with = words(&with).unwrap_or_default();
                                        assert!(
                                            if official(c) {
                                                with.contains(c.name())
                                            } else {
                                                with.contains(first) || with.contains(icons::PIN)
                                            },
                                            "{name} {width} {fit:?}: {} in {with:?}",
                                            c.name()
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                let mut prone = p.clone();
                prone.initiative = Some(10);
                prone.conditions = vec![Condition::read("Prone 2 rounds")];
                // A count's target starts after its condition's
                // name, so the name keeps its own target.
                for scale in [design::RUN_REGULAR, design::RUN_COMPACT] {
                    let layout =
                        RowLayout::new(ui, 900.0, &prone, None, scale, RowFit::Natural, 0.0);
                    let line = layout.second_line.as_ref().unwrap();
                    let (_, name) = layout.condition_spans.first().expect("Prone's name");
                    let (_, _, count) = layout.effect_spans.first().expect("its count");
                    let name_end = last_letter_right(line, name.end).unwrap();
                    let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(900.0, 60.0));
                    let (_, target) = effect_target(line, count, pos2(0.0, 0.0), rect).unwrap();
                    assert!(
                        target.left() >= name_end,
                        "{name_end} {target:?}: {}",
                        line.text()
                    );
                    assert!(target.width() >= design::XL);
                    // iter-28-R FL9: the link's underline keeps 2 px from its line's bottom, so
                    // the turn stroke of the row below never meets it.
                    let underline = link_underline_y(line, name.start).expect("Prone's underline");
                    let row = &line.rows[0];
                    assert!(
                        underline <= row.pos.y + row.size.y - 2.0,
                        "iter-28-R-FL9: underline {underline}, line bottom {}",
                        row.pos.y + row.size.y
                    );
                }
                // Through `2`, `last round` and `time is up`, the same words show at every width
                // and the word after the slot keeps its line. iter-27-R FK2 changes this on
                // purpose: that word moves at `last round` and `time is up` (the slot's spare
                // room is at the line's end); the numbers' check below keeps its x for `10`, `9`
                // and `2`.
                for width in (400..1000).step_by(6) {
                    for scale in [design::RUN_REGULAR, design::RUN_COMPACT] {
                        let mut seen = Vec::new();
                        for left in [2, 1, 0] {
                            let mut row = prone.clone();
                            row.conditions = vec![
                                Condition::Effect(super::super::effect::Effect {
                                    name: "Bless".into(),
                                    time: Time::Rounds { rounds: 2, left },
                                }),
                                Condition::from("Prone"),
                            ];
                            let layout = RowLayout::new(
                                ui,
                                width as f32,
                                &row,
                                None,
                                scale,
                                RowFit::Natural,
                                0.0,
                            );
                            let line = layout.second_line.unwrap();
                            let text = line.text().to_owned();
                            let at = text.find("Prone").map(|byte| {
                                line.pos_from_cursor(egui::text::CCursor::new(
                                    text[..byte].chars().count(),
                                ))
                                .min
                            });
                            seen.push((text.starts_with("Bless"), at));
                        }
                        for next in &seen[1..] {
                            assert_eq!(seen[0].0, next.0, "{width}: {seen:?}");
                            if let (Some(a), Some(b)) = (seen[0].1, next.1) {
                                assert!(a.y == b.y, "{width}: {seen:?}");
                            }
                        }
                    }
                }
                // T17.6: a save's DC ends the line where it has room, and is never what makes
                // the row grow; with no room it is in the hover and the name only.
                let mut conc = plain.clone();
                conc.concentration = true;
                conc.initiative = Some(10);
                for (width, shows) in [(900.0, true), (300.0, false)] {
                    let scale = design::RUN_REGULAR;
                    let base = RowLayout::new(ui, width, &conc, None, scale, RowFit::Natural, 0.0);
                    let layout = RowLayout::fitted(
                        ui,
                        width,
                        &conc,
                        (None, Some(12), false),
                        scale,
                        RowFit::Natural,
                        0.0,
                    );
                    let text = layout.second_line.as_ref().unwrap().text().to_owned();
                    // iter-28-R R7: a save that does not stand beside the mark (a condition is
                    // between) keeps `Conc save DC 12`.
                    assert_eq!(text.ends_with("Conc save DC 12"), shows, "{width}: {text}");
                    assert_eq!(layout.height(scale), base.height(scale));
                }
                // iter-28-R R7 (FL6): beside the row's own `Conc` mark the save is `save DC 12`;
                // the word `Conc` is not written twice.
                let mut beside = conc.clone();
                beside.conditions.clear();
                let text = RowLayout::fitted(
                    ui,
                    900.0,
                    &beside,
                    (None, Some(12), false),
                    design::RUN_REGULAR,
                    RowFit::Natural,
                    0.0,
                )
                .second_line
                .map(|line| line.text().to_owned())
                .unwrap_or_default();
                assert!(
                    text.ends_with("Conc ·\u{a0}save DC 12") && text.matches("Conc").count() == 1,
                    "iter-28-R-R7: {text}"
                );
                // The DC folds no condition and no bonus that shows without it.
                let mut crowded = conc.clone();
                crowded.conditions = ["Poisoned", "Prone", "Restrained", "Blinded"]
                    .map(super::super::Condition::from)
                    .to_vec();
                for width in (300..=900).step_by(10) {
                    let width = width as f32;
                    let scale = design::RUN_REGULAR;
                    let line = |dc| {
                        RowLayout::fitted(
                            ui,
                            width,
                            &crowded,
                            (None, dc, false),
                            scale,
                            RowFit::Natural,
                            0.0,
                        )
                        .second_line
                        .map(|line| line.text().to_owned())
                        .unwrap_or_default()
                    };
                    let (without, with) = (line(None), line(Some(12)));
                    assert!(with.starts_with(&without), "{width}: {without} / {with}");
                }
            });
        });
    }

    /// Reviews of PR 276: a frame that changes nothing in a row lays it out no more, also
    /// across the roster's passes and a resize back to a width it had; a change of the row's
    /// input, here the same words with another kind of time, lays it out again and shows the
    /// new words. The work is counted inside `RowLayout::measured`.
    #[test]
    fn a_row_is_laid_out_again_only_when_its_input_changes() {
        use super::super::Condition;
        use crate::test_support::{CachedWork, Counts};
        let mut run = super::super::tests::run();
        for (index, total) in [20, 15, 10].into_iter().enumerate() {
            run.apply(Command::Initiative(index, Some(total))).unwrap();
        }
        run.apply(Command::Begin).unwrap();
        run.state.participants[0].conditions = vec![Condition::read("Bless 2 rounds")];
        run.state.participants[1].conditions = vec![Condition::read("Frightened 10 rounds")];
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let frame = |run: &mut Run, width: f32| -> Vec<String> {
            let mut lines = Vec::new();
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    // The roster's three passes; the natural one's words are the ones read.
                    for fit in [RowFit::Summary, RowFit::Compressed, RowFit::Natural] {
                        let layouts = run.roster_layouts(ui, width, design::RUN_REGULAR, fit);
                        lines = layouts
                            .iter()
                            .map(|layout| {
                                layout
                                    .second_line
                                    .as_ref()
                                    .map_or_else(String::new, |line| line.text().to_owned())
                            })
                            .collect();
                    }
                });
            });
            lines
        };
        let counts = |_: &mut Run| Counts {
            reached: ROW_FITS.with(std::cell::Cell::get),
            work: ROW_LAYOUTS.with(std::cell::Cell::get),
        };
        let shows =
            |lines: &[String], words: &str| lines.iter().any(|line| line.starts_with(words));
        let check = CachedWork::start("Run row layout", &mut run, counts);
        frame(&mut run, 442.0);
        let lines = frame(&mut run, 900.0);
        check
            .warm(&mut run, shows(&lines, "Bless 2"), 3)
            .expect("row-layout-warm");
        check
            .steady(&mut run, |run| {
                let mut lines = Vec::new();
                for width in [900.0, 900.0, 442.0, 900.0] {
                    lines = frame(run, width);
                }
                shows(&lines, "Bless 2")
            })
            .expect("row-layout-steady");
        check
            .refresh(&mut run, 1, |run| {
                run.state.participants[0].conditions = vec![Condition::read("Bless 2 on its turn")];
                let lines = frame(run, 900.0);
                lines.iter().any(|line| line == "Bless 2")
            })
            .expect("row-layout-refresh");
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
                // iter-26-R FD8: with room, a long effect shows four words and `…`, so the cut
                // cannot be read as a sentence; its link is the last shown word.
                let mut effect = p.clone();
                effect.present = true;
                effect.concentration = false;
                effect.hp = effect.max_hp;
                effect.conditions = vec!["Wreathed in the cinders of the old shrine".into()];
                let wide = RowLayout::new(
                    ui,
                    900.0,
                    &effect,
                    None,
                    design::RUN_COMPACT,
                    RowFit::Natural,
                    0.0,
                );
                let text = wide.second_line.as_ref().unwrap().text().to_owned();
                assert!(
                    text.contains("Wreathed in the cinders…") && !text.contains("more"),
                    "iter-26-R-FD8: {text}"
                );
                let (_, span) = wide
                    .condition_spans
                    .iter()
                    .find(|(name, _)| name.starts_with("Wreathed"))
                    .expect("the effect has a link");
                let linked: String = text.chars().skip(span.start).take(span.len()).collect();
                assert_eq!(linked, "cinders…", "iter-26-R-FD8-link");
                // iter-26-R FI10: hidden changes keep their count, `1 more`, never a bare `more`.
                let note = crate::library::note_text("Guard", "creature", None);
                let changed = Participant::with_overrides(
                    "guard".into(),
                    "Enemies".into(),
                    Entry::parse("Guard.md", &note).unwrap(),
                    &Mapping::new(),
                    &Overrides {
                        attack_bonus: Some(1),
                        damage_bonus: Some(1),
                        ..Default::default()
                    },
                    true,
                )
                .unwrap();
                let mut counted = 0;
                let mut bare = 0;
                for width in [140.0, 180.0, 220.0, 260.0, 300.0, 340.0] {
                    let layout = RowLayout::new(
                        ui,
                        width,
                        &changed,
                        None,
                        design::RUN_COMPACT,
                        RowFit::Summary,
                        0.0,
                    );
                    let Some((_, span)) = layout
                        .condition_spans
                        .iter()
                        .find(|(name, _)| name == "Run changes")
                    else {
                        continue;
                    };
                    let text = layout.second_line.as_ref().unwrap().text().to_owned();
                    let words: String = text.chars().take(span.end).collect();
                    let kind = if words.ends_with("more") {
                        "more"
                    } else {
                        "changes"
                    };
                    let before = words.trim_end_matches(kind).trim_end_matches('\u{a0}');
                    let hidden = before
                        .rsplit([' ', '\u{a0}'])
                        .next()
                        .unwrap_or_default()
                        .to_owned();
                    assert!(
                        words.ends_with(kind)
                            && hidden.parse::<usize>().is_ok_and(|n| (1..=2).contains(&n)),
                        "iter-26-R-FI10 at {width}: {text}"
                    );
                    // iter-28-R FJ3: `N more` follows a shown change, never the bare `Changed`
                    // word (after it, the DM reads more of the conditions before it). With no
                    // change shown, the link is `✎ N changes` and `Changed` is not written.
                    let before = before[..before.len() - hidden.len()]
                        .trim_end_matches([' ', '\u{a0}', '·']);
                    if kind == "more" {
                        assert!(
                            !before.ends_with("Changed"),
                            "iter-28-R-FJ3 at {width}: {text}"
                        );
                    } else {
                        assert!(
                            before.ends_with(icons::CHANGED) && !text.contains("Changed"),
                            "iter-28-R-FJ3 at {width}: {text}"
                        );
                        bare += 1;
                    }
                    counted += 1;
                }
                assert!(counted > 0, "iter-26-R-FI10: some width hides a change");
                assert!(bare > 0, "iter-28-R-FJ3: some width shows no change");
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

    /// Review repair 4: roll words always fit their place. iter-27-R FI2, iter-28-R R8: when a
    /// row's conditions fill the line, condition words give way (into `N more`) and the result
    /// stays whole on the second line, at the row's height. Only when its two parts alone do
    /// not fit do the words take a line of their own at the name's edge, and the row grows by
    /// that line.
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
                // R8: the conditions give way; the words stay whole on the second line.
                let kept = RowLayout::new(ui, 442.0, &p, None, scale, RowFit::Natural, 0.0)
                    .with_badge(ui, Some(badge));
                assert!(!kept.badge_below, "iter-28-R-R8");
                assert_eq!(kept.height(scale), plain.height(scale), "iter-28-R-R8");
                assert_eq!(
                    badge_words(ui, badge, &font, badge_place(&kept, 0.0).2).as_deref(),
                    Some(badge),
                    "iter-28-R-R8"
                );
                // Two parts that fit in no form of the row take a line of their own.
                let long = "84% to hit with the Greatsword of the Ember Hills · advantage from Help and Faerie Fire";
                let layout = RowLayout::new(ui, 442.0, &p, None, scale, RowFit::Natural, 0.0)
                    .with_badge(ui, Some(long));
                let (left, top, room) = badge_place(&layout, 0.0);
                let words = badge_words(ui, long, &font, room);
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
                // iter-27-R FI2: a Bloodied, concentrating row with its save DC at 960 keeps its
                // height when a save result comes: the result stays on the second line.
                let mut wolf = p.clone();
                wolf.conditions.clear();
                wolf.hp = Some(3);
                let row = |badge: Option<&str>| {
                    RowLayout::fitted(
                        ui,
                        442.0,
                        &wolf,
                        (None, Some(10), false),
                        scale,
                        RowFit::Natural,
                        0.0,
                    )
                    .with_badge(ui, badge)
                };
                // iter-28-R R8: the result is never cut (before R8 it was cut to `Saved…`);
                // the row's words give way, so the row keeps its height.
                let saved = row(Some("Saved · 1 damage (half)"));
                assert!(!saved.badge_below, "iter-27-R-FI2");
                assert_eq!(
                    saved.height(scale),
                    row(None).height(scale),
                    "iter-27-R-FI2"
                );
                let (_, _, room) = badge_place(&saved, 0.0);
                assert_eq!(
                    badge_words(ui, "Saved · 1 damage (half)", &font, room).as_deref(),
                    Some("Saved · 1 damage (half)"),
                    "iter-28-R-R8: the result is whole in {room}"
                );
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
                // iter-28-R R8 (SL2): with three conditions at 960 the result is whole and the
                // row keeps its height; condition words give way into the row's `N more`.
                let mut three = plain.clone();
                three.conditions = ["Poisoned", "Prone", "Restrained"]
                    .map(super::super::Condition::from)
                    .to_vec();
                let result = "11 vs AC 13 · 12 damage";
                let row = |badge| {
                    RowLayout::new(
                        ui,
                        442.0,
                        &three,
                        None,
                        design::RUN_COMPACT,
                        RowFit::Natural,
                        0.0,
                    )
                    .with_badge(ui, badge)
                };
                let (without, with) = (row(None), row(Some(result)));
                assert!(!with.badge_below, "iter-28-R-R8: no line of its own");
                assert_eq!(
                    with.height(design::RUN_COMPACT),
                    without.height(design::RUN_COMPACT),
                    "iter-28-R-R8: the row keeps its height"
                );
                assert_eq!(
                    badge_words(ui, result, &font, badge_place(&with, 0.0).2).as_deref(),
                    Some(result),
                    "iter-28-R-R8: the result is whole: {:?}",
                    with.second_line.as_ref().map(|line| line.text().to_owned())
                );
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
                            // iter-28-R R8 (SL2): the result's two parts are never cut.
                            assert_eq!(shown, full, "iter-28-R-R8 {at}: room {room}");
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
                let font = Type::Small.font(ui);
                for width in [90.0, 150.0, 180.0, 400.0] {
                    for status in ["Changed", "Added in round 2 · Changed", "Added in setup"] {
                        // iter-23 SA1: a status part that cannot fit the slot whole goes to a
                        // line of its own above the row; it never breaks inside a word here.
                        let Some(summary) =
                            arrival_summary(ui, status, "2 × Wolf, Captain Varra", width)
                        else {
                            assert!(
                                status.split(" · ").any(|part| {
                                    design::number_width(ui, &part.replace(' ', "\u{a0}"), &font)
                                        > width
                                }),
                                "{status} at {width}"
                            );
                            continue;
                        };
                        let galley =
                            ui.painter()
                                .layout(summary, font.clone(), colors().muted, width);
                        let rows: Vec<String> = galley
                            .rows
                            .iter()
                            .map(|row| row.glyphs.iter().map(|glyph| glyph.chr).collect())
                            .collect();
                        for word in status.split([' ', '·']).filter(|word| !word.is_empty()) {
                            assert!(
                                rows.iter().any(|row| row
                                    .split([' ', '\u{a0}'])
                                    .any(|shown| shown == word)),
                                "{word} is split in {rows:?} at {width}"
                            );
                        }
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
                // iter-23 FH4: a member wider than its line breaks first before `(as …)`, then
                // at its own spaces, never inside a word; one that fits stays whole.
                let member = "1 × Maximum HP would become −28 (as Goblin Warrior)";
                assert_eq!(
                    super::super::keep_whole(ui, &font, "2 × Wolf", 150.0),
                    "2\u{a0}×\u{a0}Wolf"
                );
                for width in [100.0, 150.0, 260.0] {
                    let shown = super::super::keep_whole(ui, &font, member, width);
                    let galley =
                        ui.painter()
                            .layout(shown.clone(), font.clone(), colors().muted, width);
                    let rows: Vec<String> = galley
                        .rows
                        .iter()
                        .map(|row| row.glyphs.iter().map(|glyph| glyph.chr).collect())
                        .collect();
                    for word in member.split(' ') {
                        assert!(
                            rows.iter()
                                .any(|row| row.split([' ', '\u{a0}']).any(|part| part == word)),
                            "{word} is split in {rows:?} at {width}"
                        );
                    }
                    assert!(rows.iter().all(|row| !row.trim_end().ends_with('(')));
                    if width == 260.0 {
                        assert!(shown.contains(" (as\u{a0}Goblin\u{a0}Warrior)"), "{shown}");
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
        assert_eq!(run.unsaved_summary()[1], format!("add Prone on {name}"));
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
                        (label, Some(p.name.as_str())),
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
                // iter-26-R FA6: the tie button, under the message that names its arrival
                // whole, gives up the whole parenthesis, never only the member's name: a bare
                // `(… 3)` tells the DM nothing. One pixel short of each form gives the next.
                let arrival = "Shrine Reserve (Goblin Warrior 3)";
                let whole = "Move Shrine Reserve (Goblin Warrior 3) before Brannoch Vale";
                let short = "Move Shrine Reserve before Brannoch Vale";
                let at = |width: f32| move_words(ui, arrival, "Brannoch Vale", &font, width);
                let whole_width = design::number_width(ui, whole, &font);
                let short_width = design::number_width(ui, short, &font);
                assert_eq!(at(whole_width).as_deref(), Some(whole));
                assert_eq!(at(whole_width - 1.0).as_deref(), Some(short));
                // Narrower, the group's last word goes, then the other name's.
                let shorter = "Move Shrine… before Brannoch Vale";
                assert_eq!(at(short_width - 1.0).as_deref(), Some(shorter));
                let shortest = "Move Shrine… before Brannoch…";
                let shortest_width = design::number_width(ui, shortest, &font);
                assert_eq!(at(shortest_width).as_deref(), Some(shortest));
                assert_eq!(
                    at(shortest_width - 1.0),
                    None,
                    "then the caller cuts the target"
                );
                for width in [whole_width - 1.0, short_width - 1.0, shortest_width] {
                    assert!(
                        !at(width).unwrap().contains("(…"),
                        "iter-26-R-FA6 at {width}"
                    );
                }
                // iter-26-R SJ1, iter-27-R FI8: Undo is not under that message, so it keeps the
                // copy number that tells two same-name groups apart. The moved arrival is what
                // the DM must know: the place goes first, then the group, and the member keeps
                // its whole name. One pixel short of each form gives the next.
                let rival = "Rival Team (Rival Captain 2)";
                let undo = format!("Undo: move {rival} before Rival Team");
                let order = [
                    undo.as_str(),
                    "Undo: move Rival Team (Rival Captain 2)",
                    "Undo: move Rival Captain 2",
                ];
                for pair in order.windows(2) {
                    let width = design::number_width(ui, pair[0], &font);
                    assert_eq!(
                        undo_words(ui, (&undo, None), false, &[], &font, width + 0.5),
                        pair[0],
                        "iter-27-R-SJ1"
                    );
                    assert_eq!(
                        undo_words(ui, (&undo, None), false, &[], &font, width - 1.0),
                        pair[1],
                        "iter-27-R-SJ1 one pixel short of {:?}",
                        pair[0]
                    );
                }
                // X46: narrower, the member's name gives way and its copy number stays; with
                // no room for that either, the target goes: never `move Rival… before Rival…`.
                let placeless = "Undo: move Rival… (…\u{a0}2)";
                let width = design::number_width(ui, placeless, &font) + 0.5;
                assert_eq!(
                    undo_words(ui, (&undo, None), false, &[], &font, width),
                    placeless
                );
                let forms = undo_forms(&undo, None, false, &[]);
                assert!(
                    forms.target.iter().all(|words| words.contains('2')),
                    "{:?}",
                    forms.target
                );
                assert_eq!(forms.bare, ["Undo: move"]);
                // An arrival named by its group alone has no member to give up.
                let plain = "Move Shrine Reserve before Brannoch Vale";
                let plain_width = design::number_width(ui, plain, &font);
                let alone =
                    |width: f32| move_words(ui, "Shrine Reserve", "Brannoch Vale", &font, width);
                assert_eq!(alone(plain_width).as_deref(), Some(plain));
                assert_eq!(alone(plain_width - 1.0), None);
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
                    (&effect, Some(p.name.as_str())),
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
                    ("add Frightened on ", ""),
                    ("remove Frightened from ", ""),
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
                    let shown = undo_words(
                        ui,
                        (&full, Some(&p.name)),
                        false,
                        std::slice::from_ref(&p),
                        &font,
                        width,
                    );
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
                    // X6: one form, `Undo: <words>`, never `Undo add:` or `Undo remove:`.
                    let expected = format!("Undo: Wreathed in… {direction} Cinder… 3");
                    let width = design::number_width(ui, &expected, &font);
                    assert_eq!(
                        undo_words(
                            ui,
                            (&full, Some(&p.name)),
                            false,
                            std::slice::from_ref(&p),
                            &font,
                            width
                        ),
                        expected
                    );
                }
                // X6: an added effect's Undo has one form, `Undo: add effect <words>`: four words,
                // then two, then the target by whole words with its copy number, then no
                // target. Never a name cut to `…` and its number.
                let effect = format!(
                    "Undo: add effect Wis save DC 15 (Hold Person) on {}",
                    p.name
                );
                let rows = std::slice::from_ref(&p);
                let forms = undo_forms(&effect, Some(&p.name), false, rows);
                assert_eq!(
                    forms.target,
                    [
                        "Undo: add effect Wis save DC 15… on Cinder Wolf 3",
                        "Undo: add effect Wis save… on Cinder Wolf 3",
                        "Undo: add effect Wis save… on Cinder… 3",
                    ]
                );
                assert_eq!(
                    forms.bare,
                    [
                        "Undo: add effect Wis save DC 15…",
                        "Undo: add effect Wis save…"
                    ]
                );
                let narrow = "Undo: add effect Wis save… on Cinder… 3";
                let width = design::number_width(ui, narrow, &font) + 0.5;
                assert_eq!(
                    undo_words(ui, (&effect, Some(&p.name)), false, rows, &font, width),
                    narrow
                );
                // A form that another row's name also gives does not tell the row apart.
                let mut plain = p.clone();
                plain.name = "Cinder Wolf".into();
                let both = [p.clone(), plain];
                assert_eq!(target_forms("Cinder Wolf", &both), ["Cinder Wolf"]);
                assert_eq!(target_forms(&p.name, &both), ["Cinder Wolf 3", "Cinder… 3"]);
            });
        });
        // SJ1, SH1: when no target fits, Undo keeps the step's words without the row. A step
        // that names no row in that place has no short form, so it shows `Undo` alone.
        // The row is the step's own target (`describe_named`), never found in the words.
        let compact = |label: &str| {
            undo_forms(label, Some(&p.name), false, std::slice::from_ref(&p))
                .bare
                .first()
                .cloned()
        };
        for (label, short) in [
            // FI4: the count follows `×`, so `2` is not read as Cinder Wolf 2.
            (
                "Undo: 3 damage to 2 participants, half for 1",
                Some("Undo: 3 damage × 2"),
            ),
            (
                "Undo: 8 damage to 3 participants",
                Some("Undo: 8 damage × 3"),
            ),
            (
                "Undo: a change to 3 participants",
                Some("Undo: a change × 3"),
            ),
            (
                "Undo: exhaustion 2 → 3 on Cinder Wolf 3",
                Some("Undo: exhaustion 2 → 3"),
            ),
            (
                "Undo: initiative for Cinder Wolf 3",
                Some("Undo: initiative"),
            ),
            ("Undo: 7 damage to Cinder Wolf 3", Some("Undo: 7 damage")),
            (
                "Undo: D&D Beyond HP for Cinder Wolf 3",
                Some("Undo: D&D Beyond HP"),
            ),
            // Lookalikes: the name first, a row not in the run, or no row at all.
            ("Undo: Cinder Wolf 3 absent", None),
            ("Undo: initiative for Cinder Wolf 4", None),
            ("Undo: next turn", None),
            ("Undo: add 2 × Wolf", None),
        ] {
            assert_eq!(compact(label).as_deref(), short, "{label}");
        }
        // Review round 1 of PR 286: the row is the step's own target, never found in its
        // words. A row named `2` is not the `2` of `DC 12`, so the effect keeps two words and
        // the row.
        let mut run = super::super::tests::run();
        run.state.participants[0].name = "2".into();
        let (label, target) = run.describe_named(&Command::Condition(
            0,
            "Wis save DC 12 Against the Ancient Archmage on its turn".into(),
        ));
        assert_eq!(
            (label.as_str(), target.as_deref()),
            (
                "add effect Wis save DC 12 Against the Ancient Archmage on 2",
                Some("2")
            )
        );
        let forms = undo_forms(
            &format!("Undo: {label}"),
            target.as_deref(),
            false,
            &run.state.participants,
        );
        assert!(
            forms
                .target
                .contains(&"Undo: add effect Wis save… on 2".to_owned()),
            "{:?}",
            forms.target
        );
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
                    crate::encounters::member_fault_row(ui, &shown);
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
            // iter-25-R FB5: the repair is a bordered secondary button, not an underlined
            // link, on the Encounters list and in Arrivals alike.
            use egui_kittest::kittest::Queryable;
            let repair = harness.get_by_label("Edit group").rect();
            let bordered = harness.output().shapes.iter().any(|shape| {
                matches!(&shape.shape, egui::Shape::Rect(rect)
                    if rect.stroke.width > 0.0 && rect.rect.expand(1.0).contains_rect(repair))
            });
            assert!(bordered, "{fault:?}: the repair {repair:?} has a border");
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
                    run.roster(ui, 600.0, 0.0, 0.0);
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
        // iter-28-R (number field owner): the open field belongs to its participant, not to
        // the words of its name. A rename while it is open keeps its focus and its selection.
        run.borrow_mut().state.participants[2].name = "Guard Captain".into();
        harness.run_steps(2);
        harness
            .get_by_label("Initiative for Guard Captain")
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
        // FI1, FJ1: the grid's columns are fixed. Turning on Frightened (middle column) leaves
        // the third column where it was, and the widths were measured once, not every frame.
        // The harness draws one frame before the theme sets its font size: two sizes, so
        // up to two measures of every name, and none after.
        let measured = CONDITION_MEASURES.with(std::cell::Cell::get);
        assert!(measured >= crate::combat_sync::CONDITIONS.len());
        harness.run_steps(3);
        assert_eq!(
            CONDITION_MEASURES.with(std::cell::Cell::get),
            measured,
            "iter-24-R-FI1-once: an unchanged frame measures nothing"
        );
        let third = |harness: &egui_kittest::Harness<'_>| {
            ["Deafened", "Grappled", "Paralyzed"]
                .map(|name| harness.get_by_label(name).rect().left())
        };
        let before = third(&harness);
        harness.get_by_label("Frightened").click();
        harness.run_steps(2);
        assert!(
            run.borrow().state.participants[0]
                .conditions
                .iter()
                .any(|c| c == "Frightened"),
            "iter-24-R-FI1: Frightened is on"
        );
        assert_eq!(third(&harness), before, "iter-24-R-FI1: column 3 stays");
        assert_eq!(
            CONDITION_MEASURES.with(std::cell::Cell::get),
            measured,
            "iter-24-R-FI1-once: a toggle measures nothing"
        );
        run.borrow_mut().undo();
        harness.run_steps(2);
        // Every toggle state: each condition on, then off, and no toggle in any column moves.
        let places = |harness: &egui_kittest::Harness<'_>| {
            crate::combat_sync::CONDITIONS.map(|(_, name)| harness.get_by_label(name).rect().left())
        };
        let start = places(&harness);
        // iter-25-R FI3: a toggle's words start at one x, on or off, and it takes one width.
        let words = |harness: &egui_kittest::Harness<'_>, name: &str| {
            let rect = harness.get_by_label(name).rect();
            let left = harness
                .output()
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text)
                        if text.galley.text().ends_with(name)
                            && rect.contains(text.pos + text.galley.rect.center().to_vec2()) =>
                    {
                        Some(text.pos.x + text.galley.rect.left())
                    }
                    _ => None,
                });
            (left.expect("the toggle paints its words"), rect.width())
        };
        for (_, name) in crate::combat_sync::CONDITIONS {
            let was = run.borrow().state.participants[0].conditions.clone();
            // Exhaustion's level stepper takes its words' place, so it has no on form here.
            let plain = name != "Exhaustion";
            let off = plain.then(|| words(&harness, name));
            harness.get_by_label(name).click();
            harness.run_steps(2);
            assert_ne!(
                run.borrow().state.participants[0].conditions,
                was,
                "iter-24-R-FJ1: {name} toggled"
            );
            assert_eq!(places(&harness), start, "iter-24-R-FJ1: {name} toggled");
            if let Some(off) = off {
                let on = words(&harness, name);
                assert!(
                    (on.0 - off.0).abs() <= 1.0 && (on.1 - off.1).abs() < 0.5,
                    "iter-25-R-FI3: {name} words and width off {off:?}, on {on:?}"
                );
            }
            // Exhaustion starts on at level 2; a second press would start it again at 1.
            if name == "Exhaustion" {
                run.borrow_mut().undo();
            } else {
                harness.get_by_label(name).click();
            }
            harness.run_steps(2);
            assert_eq!(run.borrow().state.participants[0].conditions, was);
            assert_eq!(places(&harness), start, "iter-24-R-FJ1: {name} back");
        }
        assert_eq!(
            CONDITION_MEASURES.with(std::cell::Cell::get),
            measured,
            "iter-24-R-FI1-once: toggles measure nothing"
        );
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
                egui::CentralPanel::default().show(ctx, |ui| {
                    let at = shown.borrow().selected;
                    shown.borrow_mut().conditions_tab(ui, at);
                });
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
            ("Undo add: effect Wis… on … 2", false),
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
        // F2: a level with a time is read by the one reader: the level stays, with its count.
        harness.get_by_label("Other effect").click();
        harness.run_steps(1);
        harness
            .get_by_label("Other effect")
            .type_text("Exhaustion (7) 10 rounds");
        harness.run_steps(1);
        harness.get_by_label("Add").click();
        harness.run_steps(2);
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            ["Exhaustion (7) 10".to_string()]
        );
        // One entry (review of PR 276): the stepper keeps its time, and another level typed
        // with a time changes the one entry.
        harness.get_by_label("Decrease Exhaustion level").click();
        harness.run_steps(2);
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            ["Exhaustion (6) 10".to_string()]
        );
        assert_eq!(
            run.borrow().undo_label(),
            "Undo: exhaustion 7 → 6 on Guard 1"
        );
        run.borrow_mut()
            .apply(Command::Condition(0, "Exhaustion (5) 8 rounds".into()))
            .unwrap();
        assert_eq!(
            run.borrow().state.participants[0].conditions,
            ["Exhaustion (5) 8".to_string()]
        );
        // Escape in `Other effect…` clears the participant's draft, as in the dock's field.
        harness.get_by_label("Other effect").click();
        harness.run_steps(1);
        harness.get_by_label("Other effect").type_text("Haste");
        harness.run_steps(1);
        let key = run.borrow().state.participants[0].key.clone();
        assert_eq!(
            run.borrow()
                .effect_fields
                .get(&key)
                .map(|f| f.words.clone())
                .as_deref(),
            Some("Haste")
        );
        // The draft belongs to the participant: edit A, switch to B, focus the field, Ctrl+Z.
        // B keeps its own (empty) draft and A's text is unchanged.
        // A has two undo states: time passes between its two edits.
        harness.run_steps(16);
        harness.get_by_label("Other effect").type_text("!");
        harness.run_steps(1);
        let other = run.borrow().state.participants[1].key.clone();
        run.borrow_mut().selected = 1;
        harness.run_steps(2);
        harness.get_by_label("Other effect").click();
        harness.run_steps(1);
        harness.key_press_modifiers(egui::Modifiers::COMMAND, Key::Z);
        harness.run_steps(2);
        assert!(
            run.borrow()
                .effect_fields
                .get(&other)
                .is_none_or(|f| f.words.is_empty()),
            "Ctrl+Z in B's field gave B the history of A"
        );
        assert_eq!(
            run.borrow()
                .effect_fields
                .get(&key)
                .map(|f| f.words.clone())
                .as_deref(),
            Some("Haste!"),
            "Ctrl+Z in B's field changed A"
        );
        // The draft is also its run's: another run can reuse the roster slot's key. Run B shows
        // the same participant key; Ctrl+Z in its field gives it nothing of run A's history.
        run.borrow_mut().selected = 0;
        let other_run = super::super::tests::run();
        assert_eq!(
            other_run.state.participants[0].key, key,
            "run B reuses A's key"
        );
        assert_ne!(other_run.id, run.borrow().id);
        let run_a = std::mem::replace(&mut *run.borrow_mut(), other_run);
        harness.run_steps(2);
        harness.get_by_label("Other effect").click();
        harness.run_steps(1);
        harness.key_press_modifiers(egui::Modifiers::COMMAND, Key::Z);
        harness.run_steps(2);
        assert!(
            run.borrow()
                .effect_fields
                .get(&key)
                .is_none_or(|f| f.words.is_empty()),
            "Ctrl+Z in run B's field gave B the history of run A"
        );
        *run.borrow_mut() = run_a;
        harness.run_steps(2);
        harness.get_by_label("Other effect").click();
        harness.run_steps(1);
        harness.key_press(Key::Escape);
        harness.run_steps(2);
        assert!(
            run.borrow()
                .effect_fields
                .get(&key)
                .is_none_or(|f| f.words.is_empty())
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
    fn iter23_s_am1_conflict_words_offer_a_close_only_when_it_can_happen() {
        // Round 27 FN4: a blocked close has no body words; its reason sits beside Close now.
        assert_eq!(conflict_close_words(true), None);
        // Lookalike outside the rule: a conflict with nothing blocking keeps the close offer.
        assert_eq!(
            conflict_close_words(false),
            Some("Choose which HP to keep, or close and choose next time.")
        );
    }

    #[test]
    fn close_names_only_the_live_losses() {
        assert_eq!(
            close_saved_words(false),
            "The run stays saved. Resume it from Paused run on the left, or from Encounters."
        );
        assert_eq!(close_saved_words(true), "The run stays saved. Sync stops.");
        // AL1: a close keeps Undo, so no close words say that it ends.
        for sync in [false, true] {
            assert!(!close_saved_words(sync).contains("Undo"));
        }
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
                // iter-27-R FK1: a name that `Heading` holds stays in it on one line; else it
                // wraps in `Small` to two lines, whole while they hold it.
                let heading = Type::Heading.font(ui);
                let whole = design::number_width(ui, long, &heading);
                let at = |room: f32| dock_name_shown(ui, long, room);
                let size = |galley: &Galley| galley.job.sections[0].format.font_id.size;
                assert!(at(whole).text() == long && at(whole).rows.len() == 1);
                assert_eq!(size(&at(whole)), heading.size);
                let one = at(whole - 1.0);
                assert_eq!((one.text(), one.rows.len()), (long, 1));
                assert_eq!(size(&one), font.size, "Small on one line while it holds the name");
                let two = at(design::number_width(ui, long, &font) - 1.0);
                assert_eq!((two.text(), two.rows.len()), (long, 2), "iter-27-R-FK1-wraps");
                assert_eq!(size(&two), font.size);
                // iter-25-R FJ4, FJ1: a sheet notice keeps one line by shorter forms, and a
                // count of removed Undo steps stays as long as the name does.
                let summary = "Brannoch Vale’s sheet changed: HP 58 → 54 and 2 more sheets · 1 Undo step removed";
                let notice = |width: f32| sheet_notice_words(ui, summary, &font, width);
                let wide = design::number_width(ui, summary, &font);
                assert_eq!(notice(wide), summary);
                let compact = "Brannoch Vale: HP 58 → 54 and 2 more sheets · 1 Undo step removed";
                assert_eq!(notice(design::number_width(ui, compact, &font)), compact);
                let named = "Brannoch Vale · 1 Undo step removed";
                let named_width = design::number_width(ui, named, &font);
                assert_eq!(notice(named_width), named);
                assert!(design::number_width(ui, &notice(named_width - 1.0), &font) <= named_width - 1.0);
                // iter-27-R FK1: at every room the dock's name keeps to its slot and to two
                // lines that share one control's height. It shows whole wherever two lines hold
                // it; else whole words and `…`, never ending on a joining word, and a copy
                // number or letter stays.
                for name in [
                    long,
                    "Ashfang, Eldest Wolf of the Southern Ridge",
                    "Vault Guard",
                    "Ember Cultist of the Southern Watch 12",
                ] {
                    let mut forms = std::collections::BTreeSet::new();
                    for room in 20..=320 {
                        let room = room as f32;
                        let shown = dock_name_shown(ui, name, room);
                        let text = shown.text().to_owned();
                        assert!(
                            shown.size().x <= room + 0.5
                                && shown.rows.len() <= DOCK_NAME_LINES
                                && shown.size().y <= Density::Read.control() + 0.5,
                            "{room}: {text:?} {:?}",
                            shown.size()
                        );
                        forms.insert(text.clone());
                        if text == name {
                            continue;
                        }
                        assert!(text.contains('…'), "{room}: {text}");
                        let (kept, end) = text.split_once('…').unwrap();
                        if name.ends_with(" 12") && room >= design::XL {
                            assert!(end == "\u{a0}12", "{room}: {text}");
                        }
                        let words: Vec<&str> = kept.split_whitespace().collect();
                        assert!(
                            words.last().is_none_or(|word| !["of", "the"].contains(word)),
                            "{room}: {text}"
                        );
                        // Whole words, unless the first word alone is cut by letters, or (SK1)
                        // the last word's letters fill the second line.
                        let written: Vec<&str> = name.split_whitespace().collect();
                        let whole = words.len().saturating_sub(usize::from(shown.rows.len() == 2));
                        assert!(
                            words.len() <= 1
                                || words.iter().zip(&written).take(whole).all(|(a, b)| {
                                    a.trim_end_matches(',') == b.trim_end_matches(',')
                                }),
                            "{room}: {text}"
                        );
                        assert!(
                            words.iter().zip(&written).all(|(a, b)| {
                                b.trim_end_matches(',').starts_with(a.trim_end_matches(','))
                            }),
                            "{room}: {text}"
                        );
                        // iter-28-R SK1: a cut name with more than one word never leaves the
                        // second line empty while a word's first letters fit there.
                        if words.len() == 1 && written.len() > 1 && !kept.is_empty() {
                            let next: Vec<char> = written[1].chars().take(2).collect();
                            let longer = format!(
                                "{} {}…{end}",
                                written[0],
                                next.iter().collect::<String>()
                            );
                            let two = {
                                let mut job = egui::text::LayoutJob::single_section(
                                    longer.clone(),
                                    egui::TextFormat {
                                        font_id: font.clone(),
                                        line_height: Some(
                                            Density::Read.control() / DOCK_NAME_LINES as f32,
                                        ),
                                        ..Default::default()
                                    },
                                );
                                job.wrap.max_width = room;
                                ui.fonts_mut(|fonts| fonts.layout_job(job))
                            };
                            assert!(
                                kept != written[0]
                                    || two.rows.len() > DOCK_NAME_LINES
                                    || two.size().x > room + 0.5,
                                "iter-28-R-SK1 {room}: {text}, but {longer:?} fits"
                            );
                        }
                    }
                    // The rule reaches each of its forms: whole, cut by words.
                    assert!(forms.contains(name), "{name}: {forms:?}");
                    if name != "Vault Guard" {
                        assert!(forms.iter().any(|form| form.contains('…')), "{name}");
                    }
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
                        && detail.text().contains(icons::CHANGED)
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
                assert!(
                    detail.text().contains("Blessed") && detail.text().contains(icons::CHANGED)
                );
                assert_eq!(detail.rows.len(), 1, "{}", detail.text());
            });
        });
    }
}
