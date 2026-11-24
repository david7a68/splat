//! The design system. Read `docs/design-system.md`. Tokens are the only source of
//! colour, type and size. Screens compose `components` and never style by hand.
mod comparison;
pub mod components;
pub mod fact;
pub mod fitting;
pub mod icons;
pub mod layout;
pub mod memo;
pub mod outcome;
pub mod reader;
pub mod results;
pub mod save_row;
pub mod status;
pub mod text_draft;
pub mod theme;
pub mod tokens;
pub mod turn_heading;

#[cfg(test)]
mod lint;

pub use comparison::{Aligned, aligned};
pub use components::*;
#[allow(unused_imports)]
pub use layout::{
    ACTION_GAP, BackTo, CreateReasons, DraftOwner, EditorHead, EditorKind, MAX_WORD_CHARS,
    TitlePlacement, Unsaved, Word, first_unsaved, title_lead_in, title_placement,
};
pub use results::Results;
pub use save_row::DraftState;
pub use status::StatusKind;
pub use tokens::*;
