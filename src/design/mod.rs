//! The design system. Read `docs/design-system.md`. Tokens are the only source of
//! colour, type and size. Screens compose `components` and never style by hand.
pub mod components;
pub mod icons;
pub mod reader;
pub mod theme;
pub mod tokens;

#[cfg(test)]
pub mod gallery;
#[cfg(test)]
mod lint;

pub use components::*;
pub use tokens::*;
