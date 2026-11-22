//! The design system. Read `docs/design-system.md`. Tokens are the only source of
//! colour, type and size. Screens compose `components` and never style by hand.
/// Measured fact/action rows (layout PR 4).
pub mod action_rows;
pub mod column;
mod comparison;
pub mod components;
pub mod extras;
pub mod fact;
pub mod fitting;
pub mod icons;
pub mod layout;
pub mod line_end;
pub mod memo;
pub mod outcome;
pub mod pane;
pub mod popups;
pub mod reader;
pub mod results;
pub mod reveal;
pub mod save_row;
pub mod status;
#[cfg(any(test, feature = "test-support"))]
pub mod test_harness;
pub mod text_draft;
pub mod theme;
pub mod tokens;
pub mod turn_heading;
pub mod value_block;

#[cfg(test)]
pub(crate) mod attributes;
#[cfg(test)]
mod lint;

pub use column::*;
pub use components::*;
// Wave 2 callers name the extras through `design`, as they name the components.
#[allow(unused_imports)]
pub use extras::{
    FirstLine, Gap, Heading, QUOTE_INDENT, at_least, choice, choice_value, choice_wrapped,
    choice_wrapped_value, dropped, empty_line, gap, gap_to_next, heading, indent, quote,
    unless_empty,
};
// Wave 2 callers name the layout parts through `design`, as they name the components.
pub use action_rows::{Columns, RowAction, action_row};
#[allow(unused_imports)]
pub use layout::{
    ACTION_GAP, BackEdge, BackLink, BackTo, CREATE_FIELD_MIN, Create, CreateChoice, CreateDropdown,
    CreateField, CreateReasons, DraftOwner, Editor, EditorHead, EditorKind, FieldId, FieldName,
    InlineCreate, KeptLine, KeyName, MAX_WORD_CHARS, MenuPopup, NARROWEST_PANE, NoReason, Once,
    ParticipantKey, RowIndex, RowKey, ScrollColumn, Selection, SideAction, Sideways, Slot,
    SlotType, Submenus, TextKey, TextOwner, TitleAction, TitleActionRow, TitleLine, TitlePlacement,
    TitleType, Unsaved, Word, edit_in_place, first_unsaved, labelled_field, lane_width, line_field,
    menu_popup, menu_popup_at, path_help, register_title_row, search_min_row, sideways_scroll,
    status_slot, title_lead_in, title_placement, write_under_history,
};
// Wave 2 callers name the pane's escape through `design`.
#[cfg(any(test, feature = "test-support"))]
pub use pane::notices_laid;
#[allow(unused_imports)]
pub use pane::{
    ACTION_ROW, ActionLine, Block, BlockRegion, Body, ENTRY_ROW_HEIGHT, EditorRegion, Fills,
    FillsReason, Laid, LaidUi, LineColumns, Navigation, NoticeRoll, NoticeRow, PaneColumns,
    PaneEdges, Placement, PlanColumns, PlanRow, RegionState, Row, SummaryPart, check_height,
    draw_laid, line_slot, notice_slot, plan_rule, plan_slot,
};
// Wave 2 callers name the popup parts through `design`, as they name the components.
#[allow(unused_imports)]
pub use popups::{Dropdown, DropdownResponse, Ghost, ImageSize, TipLine, drag_source, rich_tip};
pub use results::Results;
pub use save_row::{DraftState, save_row};
#[allow(unused_imports)]
pub use status::{KindStatus, StatusKind, Tinted};
pub use tokens::*;
// M5, ruling R22: the run's turn heading, one closed part.
pub use turn_heading::{KeptTurnHeading, TurnHeading, TurnKeys, TurnName, TurnStrip};
// Ruling R24: one block per value of a settings pane.
pub use value_block::{HeadingNote, HeadingRow, ValueBlock};
