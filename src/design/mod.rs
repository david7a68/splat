//! The design system. Read `docs/design-system.md`. Tokens are the only source of
//! colour, type and size. Screens compose `components` and never style by hand.
pub mod column;
mod comparison;
pub mod components;
pub mod icons;
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
pub use results::Results;
pub use save_row::{DraftState, save_row};
pub use tokens::*;
