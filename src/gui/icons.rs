//! Named icon glyphs for the GPUI UI: `design::icons`, and the icon font's family name. They are
//! drawn from the bundled Phosphor Regular font (`assets/fonts/Phosphor-Regular.ttf`, MIT;
//! `theme::install` loads it). Draw one with `kit::icon(icons::X)`. Icons support words: a
//! control never shows an icon without its name (`docs/design-system.md`, Icons).
pub use crate::design::icons::*;

/// The family name of the icon font.
pub const FAMILY: &str = "Phosphor";
