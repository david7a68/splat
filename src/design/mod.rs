//! The design system. Read `docs/design-system.md`. Tokens are the only source of
//! colour, type and size. Screens compose `components` and never style by hand.
//!
//! The design system owns the raw egui calls that `clippy.toml` disallows elsewhere.
#![allow(clippy::disallowed_methods)]
pub mod column;
mod comparison;
pub mod components;
pub mod icons;
pub mod layout;
pub mod pane;
pub mod reader;
pub mod theme;
pub mod tokens;

#[cfg(test)]
pub mod gallery;
#[cfg(test)]
mod lint;

pub use column::*;
pub use components::*;
// Wave 2 callers name the layout parts through `design`, as they name the components.
#[allow(unused_imports)]
pub use layout::{
    ACTION_GAP, Create, CreateChoice, CreateField, CreateReasons, FieldId, InlineCreate, KeptLine,
    MAX_WORD_CHARS, NARROWEST_PANE, NoReason, RowIndex, RowKey, ScrollColumn, Slot, SlotType,
    TitleAction, TitleLine, TitlePlacement, TitleType, Word, labelled_field, lane_width,
    line_field, menu_popup, sideways_scroll, status_slot, title_lead_in, title_placement,
};
// Wave 2 callers name the pane's escape through `design`.
#[allow(unused_imports)]
pub use pane::{Body, Fills, FillsReason, Placement};
pub use tokens::*;
