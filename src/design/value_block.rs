//! One value of a settings pane, as one block (ruling R24, round 29): its heading row, its
//! field, its save row, then its own messages `S` below the save row. The order is the type's:
//! each slot is a method of the stage before it, so a block cannot draw its save row above its
//! field, or its warning anywhere but under its save row.
//!
//! The heading row has one form for every block ([`HeadingRow`]): the group heading, then an
//! optional note on the same line (a status, a muted line, or a spinner with a word), then an
//! optional quiet action at the row's right edge. The row is one control high whatever it holds,
//! so two blocks' headings sit at the same distance from their rows' tops.
//!
//! ```
//! use splat::value_block::{HeadingRow, ValueBlock};
//! fn f(ui: &mut eframe::egui::Ui) {
//!     let (block, _) = ValueBlock::new(ui, HeadingRow::new("Session folder"));
//!     let (block, _) = block.field(|ui| ui.label("Sessions"));
//!     let (block, _) = block.save_row(|ui| ui.label("No changes"));
//!     block.end();
//! }
//! ```
//!
//! A save row before the field does not compile:
//!
//! ```compile_fail,E0599
//! use splat::value_block::{HeadingRow, ValueBlock};
//! fn f(ui: &mut eframe::egui::Ui) {
//!     let (block, _) = ValueBlock::new(ui, HeadingRow::new("Session folder"));
//!     let (block, _) = block.save_row(|ui| ui.label("No changes"));
//!     block.end();
//! }
//! ```
//!
//! Nor a second field after the save row:
//!
//! ```compile_fail,E0599
//! use splat::value_block::{HeadingRow, ValueBlock};
//! fn f(ui: &mut eframe::egui::Ui) {
//!     let (block, _) = ValueBlock::new(ui, HeadingRow::new("Session folder"));
//!     let (block, _) = block.field(|ui| ui.label("Sessions"));
//!     let (block, _) = block.save_row(|ui| ui.label("No changes"));
//!     let (block, _) = block.field(|ui| ui.label("Sessions"));
//!     block.end();
//! }
//! ```
use super::{
    Action, Density, FORM_WIDTH, Heading, Status, Type, components::spinner, components::status,
    extras::space, heading, outcome::RowMessages, tokens::S,
};
use eframe::egui::{self, Rect, Response, Ui};
use std::marker::PhantomData;

/// What a heading row says beside its heading, on the same line.
pub enum HeadingNote<'a> {
    /// A state with its icon and colour (`Connected`).
    Status(Status<'a>),
    /// A muted line (`Not connected · showing the name saved on this computer`).
    Muted(&'a str),
    /// Work that runs now: the spinner and its word (`Refreshing…`).
    Busy(&'a str),
}

/// The one form of a block's heading row: the heading, its note, its action at the right.
#[must_use = "give it to `ValueBlock::new`"]
pub struct HeadingRow<'a> {
    title: &'a str,
    note: Option<HeadingNote<'a>>,
    action: Option<Action<'a>>,
}

impl<'a> HeadingRow<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            note: None,
            action: None,
        }
    }

    pub fn note(mut self, note: HeadingNote<'a>) -> Self {
        self.note = Some(note);
        self
    }

    /// A quiet action at the row's right edge (`Edit details`), at `Density::Read`.
    pub fn action(mut self, action: Action<'a>) -> Self {
        self.action = Some(action.density(Density::Read));
        self
    }

    fn show(self, ui: &mut Ui) -> (Rect, Option<Response>) {
        let Self {
            title,
            note,
            action,
        } = self;
        let height = Density::Read.control();
        let width = ui.available_width();
        let mut pressed = None;
        let row = ui.allocate_ui_with_layout(
            egui::vec2(width, height),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.set_min_height(height);
                let title = heading(ui, Heading::Group { title, count: 0 });
                match note {
                    Some(HeadingNote::Status(state)) => {
                        status(ui, state);
                    }
                    Some(HeadingNote::Muted(words)) => {
                        ui.weak(words);
                    }
                    Some(HeadingNote::Busy(words)) => {
                        spinner(ui);
                        ui.weak(words);
                    }
                    None => {}
                }
                if let Some(action) = action {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        pressed = Some(action.show(ui));
                    });
                }
                title.rect
            },
        );
        let _ = row.inner;
        (row.response.rect, pressed)
    }
}

/// Typestate of a [`ValueBlock`]: its heading row is drawn; its field follows.
pub struct AfterHeading;
/// Typestate of a [`ValueBlock`]: its field is drawn; a save row may follow.
pub struct AfterField;
/// Typestate of a [`ValueBlock`]: its save row is drawn; its own messages may follow.
pub struct AfterSave;

/// Where each slot of a block was drawn, for the proofs (test builds keep it per pass).
#[derive(Clone, Debug, PartialEq)]
pub struct BlockRecord {
    pub title: String,
    /// The heading row: one control high.
    pub heading: Rect,
    pub field: Rect,
    pub save_row: Option<Rect>,
    /// The block's own messages, `S` below its save row, when it shows any.
    pub messages: Option<Rect>,
}

#[cfg(test)]
const BLOCK_RECORDS: &str = "design_value_blocks";

/// Each block drawn in the last pass, in drawing order. Only test builds keep them.
#[cfg(test)]
pub fn block_records(ctx: &egui::Context) -> Vec<BlockRecord> {
    let last = ctx.cumulative_pass_nr().saturating_sub(1);
    let now = ctx.cumulative_pass_nr();
    ctx.data(|data| data.get_temp::<(u64, Vec<BlockRecord>)>(egui::Id::new(BLOCK_RECORDS)))
        .filter(|(pass, _)| *pass == last || *pass == now)
        .map(|(_, blocks)| blocks)
        .unwrap_or_default()
}

/// One value's block, at most `FORM_WIDTH` wide: see the module's text.
#[must_use = "draw its field"]
pub struct ValueBlock<'u, Stage> {
    ui: &'u mut Ui,
    record: BlockRecord,
    stage: PhantomData<Stage>,
}

impl<'u> ValueBlock<'u, AfterHeading> {
    /// The block's heading row, with its own space above it (none when it is the first thing
    /// in its column), and the press of its action.
    pub fn new(ui: &'u mut Ui, row: HeadingRow<'_>) -> (Self, Option<Response>) {
        ui.set_max_width(ui.available_width().min(FORM_WIDTH));
        let first = ui.cursor().top() <= ui.max_rect().top() + 0.5;
        let spacing = ui.spacing().item_spacing.y;
        if !first {
            // The heading's words, not its row, start `Heading::above` below the item before
            // (row 93): the row centres them, so its top comes up by their inset in the row.
            let inset = (Density::Read.control() - Type::Label.line()) / 2.0;
            space(
                ui,
                Heading::Group {
                    title: "",
                    count: 0,
                }
                .above()
                    - spacing
                    - inset,
            );
        }
        let title = row.title.to_owned();
        let (heading, pressed) = row.show(ui);
        space(
            ui,
            Heading::Group {
                title: "",
                count: 0,
            }
            .below()
                - spacing,
        );
        (
            Self {
                ui,
                record: BlockRecord {
                    title,
                    heading,
                    field: Rect::NOTHING,
                    save_row: None,
                    messages: None,
                },
                stage: PhantomData,
            },
            pressed,
        )
    }

    /// The value's field and what belongs to it (its help, its current value).
    pub fn field<R>(mut self, add: impl FnOnce(&mut Ui) -> R) -> (ValueBlock<'u, AfterField>, R) {
        let response = self.ui.vertical(add);
        self.record.field = response.response.rect;
        (self.next(), response.inner)
    }
}

impl<'u> ValueBlock<'u, AfterField> {
    /// The value's save row, under its field.
    pub fn save_row<R>(mut self, add: impl FnOnce(&mut Ui) -> R) -> (ValueBlock<'u, AfterSave>, R) {
        let response = self.ui.vertical(add);
        self.record.save_row = Some(response.response.rect);
        (self.next(), response.inner)
    }

    /// A block with no save row (its value saves elsewhere, or is read only).
    pub fn end(self) {
        self.keep();
    }
}

impl<'u> ValueBlock<'u, AfterSave> {
    /// The save row's own messages (its fault, the warning before its act, its kept result),
    /// `S` below the save row; nothing, not even the gap, when there is none. A press comes back
    /// as the caller's id.
    pub fn messages<A: 'static>(
        mut self,
        messages: RowMessages<'_, A>,
    ) -> Option<crate::design::outcome::Press<A>> {
        let mut pressed = None;
        if !messages.is_empty() {
            space(self.ui, S - self.ui.spacing().item_spacing.y);
            let response = self.ui.vertical(|ui| messages.show(ui));
            self.record.messages = Some(response.response.rect);
            pressed = response.inner;
        }
        self.keep();
        pressed
    }

    /// A save row with no messages of its own.
    pub fn end(self) {
        self.keep();
    }
}

impl<'u, Stage> ValueBlock<'u, Stage> {
    fn next<Next>(self) -> ValueBlock<'u, Next> {
        ValueBlock {
            ui: self.ui,
            record: self.record,
            stage: PhantomData,
        }
    }

    fn keep(self) {
        #[cfg(test)]
        super::components::keep_for_pass(self.ui, BLOCK_RECORDS, self.record);
        #[cfg(not(test))]
        let _ = (self.ui, self.record);
    }
}
