//! One value of a settings pane, as one block (ruling R24): its
//! heading row, its field, its save row, then its own messages `S` below the save row. The order
//! is the type's: each slot is a method of the stage before it, so a block cannot put its save row
//! above its field, or its warning anywhere but under its save row.
//!
//! ```ignore
//! kit::ValueBlock::new(kit::HeadingRow::new("Session folder").action(edit), first)
//!     .field(folder_field)
//!     .save_row(save_row)
//!     .messages(vec![message])
//! ```
//!
//! The heading row has one form for every block: the group heading, an optional note on the same
//! line (a status, a muted line, or a spinner with a word), and an optional quiet action at the
//! row's right edge. The row is one `Density::Read` control high whatever it holds.
use super::{Button, Message, Status, label, status};
use crate::gui::theme::{Density, FORM_WIDTH, L, S, Type, TypeExt, XS, colors};
use gpui::{
    AnyElement, Div, IntoElement, ParentElement, RenderOnce, SharedString, Styled, div, prelude::*,
};
use std::marker::PhantomData;

/// What a heading row says beside its heading, on the same line.
pub enum HeadingNote {
    /// A state with its icon and colour (`Connected`).
    Status(Status),
    /// A muted line (`Not connected · showing the name saved on this computer`).
    Muted(SharedString),
    /// Work that runs now: the spinner and its word (`Refreshing…`).
    Busy(SharedString),
}

/// The one form of a block's heading row: the heading, its note, its action at the right.
#[must_use = "give it to `ValueBlock::new`"]
pub struct HeadingRow {
    title: SharedString,
    note: Option<HeadingNote>,
    action: Option<Button>,
}

impl HeadingRow {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            note: None,
            action: None,
        }
    }

    pub fn note(mut self, note: HeadingNote) -> Self {
        self.note = Some(note);
        self
    }

    /// A quiet action at the row's right edge (`Edit details`), at `Density::Read`.
    pub fn action(mut self, action: Button) -> Self {
        self.action = Some(action.density(Density::Read));
        self
    }

    fn row(self) -> Div {
        let c = colors();
        let note: Option<AnyElement> = self.note.map(|note| match note {
            HeadingNote::Status(state) => status(&state).into_any_element(),
            HeadingNote::Muted(words) => div()
                .ty(Type::Body)
                .text_color(c.muted)
                .child(words)
                .into_any_element(),
            HeadingNote::Busy(words) => div()
                .flex()
                .flex_row()
                .items_center()
                .gap(XS)
                .child(gpui_component::spinner::Spinner::new().color(c.muted))
                .child(div().ty(Type::Body).text_color(c.muted).child(words))
                .into_any_element(),
        });
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .h(Density::Read.control())
            .child(label(self.title, 0))
            .children(note)
            .child(div().flex_1())
            .children(self.action)
    }
}

/// Typestate of a [`ValueBlock`]: its heading row is drawn; its field follows.
pub struct AfterHeading;
/// Typestate of a [`ValueBlock`]: its field is drawn; a save row may follow.
pub struct AfterField;
/// Typestate of a [`ValueBlock`]: its save row is drawn; its own messages may follow.
pub struct AfterSave;

/// One value's block, at most `FORM_WIDTH` wide: see the module's text.
#[must_use = "draw its field"]
pub struct ValueBlock<Stage> {
    block: Div,
    stage: PhantomData<Stage>,
}

impl ValueBlock<AfterHeading> {
    /// The block's heading row, with its own space above it (`L`, none when it is the `first`
    /// thing in its column) and `XS` under it.
    pub fn new(row: HeadingRow, first: bool) -> Self {
        // The heading's words, not its row, start `L` below the item before: the row centres
        // them, so its top comes up by their inset in the row.
        let inset = (Density::Read.control() - Type::Label.line()) / 2.0;
        let block = div()
            .flex()
            .flex_col()
            .w_full()
            .max_w(FORM_WIDTH)
            .when(!first, |block| block.pt(L - inset))
            .child(row.row())
            .child(super::gap(XS));
        Self {
            block,
            stage: PhantomData,
        }
    }

    /// The value's field and what belongs to it (its help, its current value).
    pub fn field(self, field: impl IntoElement) -> ValueBlock<AfterField> {
        self.next(field)
    }
}

impl ValueBlock<AfterField> {
    /// The value's save row, under its field.
    pub fn save_row(self, row: impl IntoElement) -> ValueBlock<AfterSave> {
        self.next(row)
    }

    /// A block with no save row (its value saves elsewhere, or is read only).
    pub fn end(self) -> Block {
        Block(self.block)
    }
}

impl ValueBlock<AfterSave> {
    /// The save row's own messages (its fault, the warning before its act, its kept result),
    /// `S` below the save row; nothing, not even the gap, when there is none.
    pub fn messages(self, messages: Vec<Message>) -> Block {
        if messages.is_empty() {
            return Block(self.block);
        }
        Block(
            self.block
                .child(div().flex().flex_col().pt(S).gap(S).children(messages)),
        )
    }

    /// A save row with no messages of its own.
    pub fn end(self) -> Block {
        Block(self.block)
    }
}

impl<Stage> ValueBlock<Stage> {
    fn next<Next>(self, slot: impl IntoElement) -> ValueBlock<Next> {
        ValueBlock {
            block: self.block.child(slot),
            stage: PhantomData,
        }
    }
}

/// A finished value block, to put in a column.
#[derive(IntoElement)]
pub struct Block(Div);

impl RenderOnce for Block {
    fn render(self, _: &mut gpui::Window, _: &mut gpui::App) -> impl IntoElement {
        self.0
    }
}
