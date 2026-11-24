//! A sheet's labelled fact (`design::fact`, ruling M5): one form, `Label: value`, its values
//! joined by the middle dot (`Coins: 12 cp · 40 sp`). The part owns the colon, the separator and
//! how a joined line wraps; a caller gives a label and its values, never a separator.
//!
//! - [`fact_line`]: a fact on a line of its own. A line breaks only at a space that does not follow
//!   a middle dot, so every dot stays with the word after it; a word too long for a line breaks
//!   between whole characters (graphemes). So no line ends at a dot.
//! - [`words`] and [`tile_label`]: the same form as words (`Str: 8`), from `design::fact`.
//! - [`roll_tile`]: a stat tile whose value rolls: its muted label line over the caller's link.
use crate::gui::theme::{Type, TypeExt, colors};
use gpui::{Div, IntoElement, ParentElement, Styled, div};
use std::fmt::Display;

pub use crate::design::fact::{tile_label, words};

/// The no-break space that keeps a middle dot with the word after it.
const AFTER_DOT: &str = "·\u{a0}";

/// `Resistance: Fire · Cold` on a line of its own, in `Body`, wrapped so no line ends at a dot.
pub fn fact_line(label: &str, values: impl IntoIterator<Item = impl Display>) -> Div {
    let shown = words(label, values).replace("· ", AFTER_DOT);
    div().ty(Type::Body).text_color(colors().text).child(shown)
}

/// A stat tile whose value rolls: the label line (sentence-case muted `Small` words, cut to the
/// tile) above the caller's roll link.
pub fn roll_tile(label: &str, link: impl IntoElement) -> Div {
    div()
        .flex()
        .flex_col()
        .child(
            div()
                .ty(Type::Small)
                .text_color(colors().muted)
                .truncate()
                .child(label.to_owned()),
        )
        .child(link)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_dot_stays_with_the_word_after_it() {
        let shown = words("Coins", ["12 cp", "40 sp"]).replace("· ", AFTER_DOT);
        assert_eq!(shown, "Coins: 12 cp ·\u{a0}40 sp");
        assert_eq!(words("Resistance", [" "]), "Resistance: —");
    }
}
