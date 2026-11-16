//! The design system. Read `docs/design-system.md`. Tokens are the only source of
//! colour, type and size. Screens compose `components` and never style by hand.
pub mod column;
mod comparison;
pub mod components;
pub mod extras;
pub mod fact;
pub(crate) mod fitting;
pub mod icons;
pub mod layout;
pub mod memo;
pub mod outcome;
pub mod pane;
pub mod popups;
pub mod reader;
pub mod results;
pub mod reveal;
pub mod save_row;
pub mod status;
pub mod theme;
pub mod tokens;
pub mod turn_heading;
pub mod value_block;

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
pub use extras::{
    Gap, Heading, choice, choice_value, choice_wrapped, choice_wrapped_value, gap, heading,
};
// Wave 2 callers name the layout parts through `design`, as they name the components.
#[allow(unused_imports)]
pub use layout::{
    ACTION_GAP, BackEdge, BackLink, BackTo, CREATE_FIELD_MIN, Create, CreateChoice, CreateDropdown,
    CreateField, CreateReasons, DraftOwner, Editor, EditorHead, EditorKind, FieldId, FieldName,
    InlineCreate, KeptLine, KeyName, MAX_WORD_CHARS, NARROWEST_PANE, NoReason, Once, RowIndex,
    RowKey, ScrollColumn, Selection, SideAction, Slot, SlotType, TextKey, TextOwner, TitleAction,
    TitleActionRow, TitleLine, TitlePlacement, TitleType, Unsaved, Word, first_unsaved,
    labelled_field, lane_width, line_field, menu_popup, path_help, register_title_row,
    search_min_row, sideways_scroll, status_slot, title_lead_in, title_placement,
};
// Wave 2 callers name the pane's escape through `design`.
#[allow(unused_imports)]
pub use pane::{Body, EditorRegion, Fills, FillsReason, Placement};
// Wave 2 callers name the popup parts through `design`, as they name the components.
#[allow(unused_imports)]
pub use popups::{Dropdown, DropdownResponse, Ghost, ImageSize, TipLine, drag_source, rich_tip};
pub use results::Results;
pub use save_row::{DraftState, save_row};
#[allow(unused_imports)]
pub use status::{KindStatus, StatusKind, Tinted};
pub use tokens::*;
// M5, ruling R22: the run's turn heading, one closed part.
pub use turn_heading::{KeptTurnHeading, TurnHeading, TurnKeys, TurnName};
// Ruling R24: one block per value of a settings pane.
pub use value_block::{HeadingNote, HeadingRow, ValueBlock};
