//! The design system. Read `docs/design-system.md`. Tokens are the only source of
//! colour, type and size. Screens compose `components` and never style by hand.
pub mod column;
mod comparison;
pub mod components;
pub mod extras;
pub mod icons;
pub mod layout;
pub mod memo;
pub mod outcome;
pub mod reader;
pub mod results;
pub mod save_row;
pub mod theme;
pub mod tokens;

#[cfg(test)]
pub(crate) mod attributes;
#[cfg(test)]
pub mod gallery;
#[cfg(test)]
mod lint;

pub use column::*;
pub use components::*;
// Wave 2 callers name the extras through `design`, as they name the components.
#[allow(unused_imports)]
pub use extras::{Gap, Heading, choice, choice_value, gap, heading};
// Wave 2 callers name the layout parts through `design`, as they name the components.
#[allow(unused_imports)]
pub use layout::{
    ACTION_GAP, Create, CreateChoice, CreateField, CreateReasons, FieldId, InlineCreate, KeptLine,
    MAX_WORD_CHARS, NARROWEST_PANE, NoReason, RowIndex, RowKey, ScrollColumn, Slot, SlotType,
    TextKey, TitleAction, TitleLine, TitlePlacement, TitleType, Word, labelled_field, lane_width,
    line_field, menu_popup, sideways_scroll, status_slot, title_lead_in, title_placement,
};
pub use results::Results;
pub use save_row::{DraftState, save_row};
pub use tokens::*;
