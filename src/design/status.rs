//! A status whose icon and words take different colours (package K2, gap K-7; ruling R5).
//!
//! `Status` has one colour for its icon and its word. A [`StatusKind`] decides both colours and
//! the icon: a closed set, so no caller passes a free colour.
use super::Color;

use super::{colors, icons};

/// The statuses whose icon and words differ in colour. Each decides its icon and both colours.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusKind {
    /// A live recording: only its record icon in the `recording` token; its words (and its
    /// time) in the text colour (R5).
    Recording,
}

impl StatusKind {
    pub fn icon(self) -> &'static str {
        match self {
            Self::Recording => icons::RECORD,
        }
    }

    pub fn icon_color(self) -> Color {
        match self {
            Self::Recording => colors().recording,
        }
    }

    pub fn word_color(self) -> Color {
        match self {
            Self::Recording => colors().text,
        }
    }
}
