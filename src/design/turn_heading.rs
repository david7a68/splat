//! The run's turn heading (mechanism M5, ruling R22): `Round 2`, the current turn's name,
//! `Add` and `Next turn` on one line, fitted and painted by this one part.
//!
//! A caller gives data: the run's name, the round, the current turn's whole name (a
//! [`TurnName`], made from the participant's own name field), the run's scale,
//! whether `Add` is selected, why `Next turn` is blocked, and whether the keys work. It gets
//! back presses. It cannot measure the controls, paint the heading's words, or give a width:
//! the part reads each control's width from the same [`Action`] that it paints, finds the
//! name's real room between the round and `Add`, chooses ONE complete candidate (the words
//! and the controls together) from a private ordered list, and paints that candidate.
//!
//! **Key policy of a name line (R22).** In a line that holds a name, the key letter `N` gives
//! up its room before the name is cut: a candidate with the letter is tried only while every
//! name of the roster is whole beside it at the least size, so pressing `Next turn` never
//! changes the letter and no control moves under the pointer (G1b). With the keyboard elsewhere (Lookup open) the letter is never a
//! candidate. A line with no name keeps its controls' places by its own layout (the dock's keys are
//! plain `Action` keys).
//!
//! **Kept result.** [`KeptTurnHeading`] holds the one chosen candidate, its galleys and its
//! words. Its owner is the run view (`Run::turn_heading`, made with the run); it ends with the
//! run. A frame whose data and metrics (the heading's width, the pixels per point, the fonts of
//! both control forms, the theme's font generation, the allowed name sizes and the colours)
//! equal the kept key builds no string and measures no candidate in this part. Any change of
//! one of them chooses again. (The run's second header line and its `Strip` are outside this
//! claim.)
//!
//! **The name is never wholly hidden (lead's ruling 1).** Every shortened candidate keeps at
//! least the first letter of the name and its copy mark (`T… 12`, never `… 12`). Where not even
//! that fits, no candidate fits: the part paints that least form clipped to its room. The run
//! list keeps 454 px (`RUN_LIST_KEEPS`), and there a candidate always fits (the unit sweep
//! proves it for a three-digit round, the large scale and a long numbered name).
//!
//! The type proves construction and paint ownership (rung 1). It does not prove that the
//! glyphs are inside the room; the run's visual checks do (rung 4).

/// From this width of a run's line, `Add` reads `Add to fight`; below it, `Add`. The dock
/// below the roster uses the same width for its own words (`DOCK_ALL_WORDS_FROM`).
pub const ADD_WORDS_FROM: f32 = 500.0;

/// [`whole_words_name`] with any measure of a line's width (a view that does not draw with
/// egui).
pub fn whole_words_name_by(name: &str, width: f32, measure: impl Fn(&str) -> f32) -> String {
    let fitted = super::components::words_that_fit_by(name, width, &measure);
    if fitted == name {
        return fitted;
    }
    let fitted = fitted.trim_end_matches('…');
    let last = name.split_whitespace().last().unwrap_or(name);
    if !last.chars().all(|c| c.is_ascii_digit()) && last.chars().count() != 1 {
        let mut words: Vec<_> = fitted.split_whitespace().collect();
        while words
            .last()
            .is_some_and(|word| ["of", "the", "de", "von"].contains(word))
        {
            words.pop();
        }
        while words.len() > 1 && measure(&format!("{}…", words.join(" "))) > width {
            words.pop();
        }
        return format!("{}…", words.join(" "));
    }
    let prefix_room = width - measure(&format!("… {last}"));
    let prefix = super::components::words_that_fit_by(name, prefix_room, &measure);
    let prefix = prefix.trim_end_matches('…');
    let candidate = format!("{prefix}… {last}");
    if measure(&candidate) <= width {
        candidate
    } else {
        format!("… {last}")
    }
}
