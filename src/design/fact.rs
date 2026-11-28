//! A sheet's labelled fact (ruling M5, R40): one form, `Label: value`, its values joined by the
//! middle dot (`Coins: 12 cp · 40 sp`). The part owns the colon, the separator and how a joined
//! line wraps; a caller gives a label and its values, never a separator.
//!
//! - [`line`]: a fact on a line of its own. A line breaks only at a space that does not follow a
//!   middle dot, so every dot stays with the word after it, also a dot inside a value; a word too
//!   long for a line breaks between whole characters (graphemes), never right after a dot. So no
//!   line ends at a dot (round 31 AM15). An empty value is left out with its dot; a value is
//!   trimmed and its runs of spaces are one space. It is a label whose text is the plain words:
//!   the DM can select and copy them exactly, a screen reader can focus it, and its accessible
//!   value is those words.
//! - [`tile_label`]: a stat tile's label that holds its own number (`Str: 8` over `+1`), in the
//!   same form (round 31 SL12).
//! - [`roll_tile`]: the rolling stat tile, a [`super::stat_tile`] whose value rolls.
//!
//! ```
//! use splat_design::fact::{tile_label, words};
//! assert_eq!(words("Coins", ["12 cp", "40 sp"]), "Coins: 12 cp · 40 sp");
//! assert_eq!(words("Resistance", ["Fire", ""]), "Resistance: Fire");
//! assert_eq!(words("Resistance", ["  Fire   and  Cold "]), "Resistance: Fire and Cold");
//! assert_eq!(words("Resistance", [" "]), "Resistance: —");
//! assert_eq!(tile_label("Str", 8), "Str: 8");
//! ```

use std::fmt::Display;

/// The separator the DM reads between two values.
const SEPARATOR: &str = " · ";

/// What a fact with no value says: the sheet's mark for a value it does not have.
const NONE: &str = "—";

/// The values that have words, each trimmed with its runs of spaces made one; an empty value is
/// left out, and so is its separator.
fn present(values: impl IntoIterator<Item = impl Display>) -> Vec<String> {
    values
        .into_iter()
        .map(|value| {
            value
                .to_string()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|value| !value.is_empty())
        .collect()
}

/// `Resistance: Fire · Cold`: the one form of a fact, with no empty value (`Resistance: —`
/// when none is left). It is also the text, the copy and the accessible value of [`line`].
pub fn words(label: &str, values: impl IntoIterator<Item = impl Display>) -> String {
    let values = present(values);
    if values.is_empty() {
        format!("{label}: {NONE}")
    } else {
        format!("{label}: {}", values.join(SEPARATOR))
    }
}

/// The label of a stat tile that holds its own number: `Str: 8`, over the tile's value (`+1`).
pub fn tile_label(label: &str, value: impl Display) -> String {
    words(label, [value])
}
