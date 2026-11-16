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
//! use splat::fact::{tile_label, words};
//! assert_eq!(words("Coins", ["12 cp", "40 sp"]), "Coins: 12 cp · 40 sp");
//! assert_eq!(words("Resistance", ["Fire", ""]), "Resistance: Fire");
//! assert_eq!(words("Resistance", ["  Fire   and  Cold "]), "Resistance: Fire and Cold");
//! assert_eq!(words("Resistance", [" "]), "Resistance: —");
//! assert_eq!(tile_label("Str", 8), "Str: 8");
//! ```

use std::fmt::Display;
use std::ops::Range;
use std::sync::Arc;

use eframe::egui::{self, Response, Ui};
use splat::content::dice_text::RollLink;
use unicode_segmentation::UnicodeSegmentation;

use super::{Type, colors};

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

/// Where `text` may break a line: after each space that does not follow a middle dot (the byte
/// after the space). The space stays at the end of the line before, as a wrapped label keeps
/// it, so the lines together are the text.
fn breaks(text: &str) -> Vec<usize> {
    let mut breaks = Vec::new();
    let mut before = None;
    for (at, c) in text.char_indices() {
        if c == ' ' && before.is_some_and(|before| before != '·') {
            breaks.push(at + 1);
        }
        before = Some(c);
    }
    breaks
}

/// Whether a line's words end at a middle dot.
fn ends_at_dot(words: &str) -> bool {
    words.trim_end().ends_with('·')
}

/// The lines of `text` at most `width` wide (`wide` measures), as byte ranges that together are
/// `text`: each ends at the farthest break whose words fit with the space after them (a line is
/// as wide as its words and that space). A first word wider than a line
/// breaks between whole characters, at least one to a line and never right after a dot.
fn rows(text: &str, width: f32, wide: impl Fn(&str) -> f32) -> Vec<Range<usize>> {
    let breaks = breaks(text);
    let mut rows = Vec::new();
    let mut start = 0;
    while start < text.len() {
        if wide(&text[start..]) <= width {
            rows.push(start..text.len());
            break;
        }
        let fit = breaks
            .iter()
            .copied()
            .filter(|&end| end > start)
            .take_while(|&end| wide(&text[start..end]) <= width)
            .last();
        let end = fit.unwrap_or_else(|| {
            let word = breaks
                .iter()
                .copied()
                .find(|&end| end > start)
                .unwrap_or(text.len());
            let mut end = start;
            for (offset, grapheme) in text[start..word].grapheme_indices(true) {
                let next = start + offset + grapheme.len();
                if end > start
                    && wide(&text[start..next]) > width
                    && !ends_at_dot(&text[start..end])
                {
                    break;
                }
                end = next;
            }
            end
        });
        rows.push(start..end);
        start = end;
    }
    rows
}

/// One galley whose text is `text`, laid out on `rows`: each line laid out by itself, then
/// joined with no line break of its own, so a selection and a copy read `text` as it is.
fn galley(ui: &Ui, text: &str, rows: &[Range<usize>]) -> Arc<egui::Galley> {
    let lay = |words: &str| {
        egui::WidgetText::from(words).into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            egui::TextStyle::Body,
        )
    };
    let parts: Vec<Arc<egui::Galley>> = rows.iter().map(|row| lay(&text[row.clone()])).collect();
    let Some(first) = parts.first() else {
        return lay(text);
    };
    let mut job = (*first.job).clone();
    job.text = text.to_owned();
    if let Some(section) = job.sections.first_mut() {
        section.byte_range = 0..text.len();
        job.sections.truncate(1);
    }
    let mut joined = egui::Galley::concat(Arc::new(job), &parts, ui.ctx().pixels_per_point());
    for row in &mut joined.rows {
        Arc::make_mut(&mut row.row).ends_with_newline = false;
    }
    Arc::new(joined)
}

/// A fact on a line of its own, broken at the room it has where no dot is left at a line's end.
/// A label of the plain words: selectable and copied as they are, and focusable for a screen
/// reader, whose value they are.
pub fn line(ui: &mut Ui, label: &str, values: impl IntoIterator<Item = impl Display>) -> Response {
    let text = words(label, values);
    let wide = |words: &str| {
        egui::WidgetText::from(words)
            .into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                egui::TextStyle::Body,
            )
            .size()
            .x
    };
    let rows = rows(&text, ui.available_width(), wide);
    let galley = galley(ui, &text, &rows);
    ui.add(egui::Label::new(galley))
}

/// The label of a stat tile that holds its own number: `Str: 8`, over the tile's value (`+1`).
pub fn tile_label(label: &str, value: impl Display) -> String {
    words(label, [value])
}

/// The label line of every stat tile: sentence-case muted `Small` words, cut to the tile.
pub(super) fn tile_label_line(ui: &mut Ui, label: &str) {
    ui.add(egui::Label::new(Type::Small.rich(ui, label).color(colors().muted)).truncate());
}

/// A stat tile whose value rolls: the label line of [`super::stat_tile`] over the value as a
/// roll link in the large numeral type. The press rolls `link`.
pub fn roll_tile(ui: &mut Ui, label: &str, value: &str, link: &RollLink) -> Response {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        tile_label_line(ui, label);
        crate::rolls::link(ui, value, Type::NumeralLarge, link)
    })
    .inner
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::kittest::NodeT;
    use splat::content::dice_text::D20;

    /// The text shapes a frame painted that start with `start`, row by row.
    fn painted_rows(harness: &egui_kittest::Harness<'_>, start: &str) -> Vec<String> {
        harness
            .output()
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text)
                    if text.galley.job.text.replace('\n', "").starts_with(start) =>
                {
                    Some(
                        text.galley
                            .rows
                            .iter()
                            .map(|row| row.text())
                            .collect::<Vec<_>>(),
                    )
                }
                _ => None,
            })
            .flatten()
            .collect()
    }

    /// The accessible values of the nodes whose value starts with `start`.
    fn node_values(harness: &egui_kittest::Harness<'_>, start: &str) -> Vec<String> {
        harness
            .root()
            .children_recursive()
            .filter(|node| node.accesskit_node().role() == egui::accesskit::Role::Label)
            .filter_map(|node| node.accesskit_node().value())
            .filter(|value| value.starts_with(start))
            .collect()
    }

    /// Presses the primary button at `from`, moves to `to` and lets go, then copies: the copied
    /// texts.
    fn drag_copy(
        harness: &mut egui_kittest::Harness<'_>,
        from: egui::Pos2,
        to: egui::Pos2,
    ) -> Vec<String> {
        let button = |pos, pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        harness.hover_at(from);
        harness.step();
        harness.event(button(from, true));
        harness.step();
        harness.hover_at(to);
        harness.step();
        harness.event(button(to, false));
        harness.step();
        harness.event(egui::Event::Copy);
        harness.step();
        harness
            .output()
            .platform_output
            .commands
            .iter()
            .filter_map(|command| match command {
                egui::OutputCommand::CopyText(text) => Some(text.clone()),
                _ => None,
            })
            .collect()
    }

    /// M5: one form for every fact, in words and in the accessible value: the colon, then the
    /// values joined by the middle dot, an empty value left out with its dot, spaces made one.
    /// A tile label that holds its own number takes the same form. Drawn at every width, the
    /// painted lines are the words, and no line ends at a dot: also before a CJK value (which
    /// `epaint` may break before), with an empty value, a dot inside a value, a value with
    /// leading spaces, one long value and words longer than a line. A word breaks only between
    /// whole characters (graphemes), also inside an emoji sequence. The fact is a label: dragged
    /// across, it copies exactly its words; with a screen reader, Tab focuses it.
    #[test]
    fn a_fact_says_label_colon_values_and_never_ends_a_line_at_a_dot() {
        const WORDS: &str = "Languages: Common · Elvish · Dwarvish · Sylvan · Thieves’ Cant";
        let languages = ["Common", "Elvish", "Dwarvish", "Sylvan", "Thieves’ Cant"];
        assert_eq!(words("Languages", languages), WORDS);
        assert_eq!(words("Darkvision", ["60 ft."]), "Darkvision: 60 ft.");
        assert_eq!(words("Resistance", ["Fire", ""]), "Resistance: Fire");
        assert_eq!(words("Resistance", ["", " "]), "Resistance: —");
        assert_eq!(
            words(
                "Resistance",
                ["Fire", "  Supercalifragilisticexpialidocious"]
            ),
            "Resistance: Fire · Supercalifragilisticexpialidocious"
        );
        assert_eq!(tile_label("Str", 8), "Str: 8");
        const LONG: &str = "Speaks with every creature it can see within sixty feet, also through \
            a wall of stone or a closed door";
        let owned = |values: &[&str]| values.iter().map(ToString::to_string).collect::<Vec<_>>();
        // Each case, and whether it fits one line at 640 px.
        let cases: [(&str, Vec<String>, bool); 9] = [
            ("Languages", owned(&languages), true),
            ("Resistance", owned(&["Fire", "火焰"]), true),
            ("Resistance", owned(&["Fire", ""]), true),
            ("Resistance", owned(&["Fire · Cold · Lightning"]), true),
            (
                "Resistance",
                owned(&["Fire", "  Supercalifragilisticexpialidocious"]),
                true,
            ),
            ("Telepathy", owned(&[LONG]), false),
            ("Mood", vec!["👍🏽".repeat(16)], false),
            ("Family", vec!["👨‍👩‍👧‍👦".repeat(12)], false),
            ("Love", vec!["❤️".repeat(16)], false),
        ];
        for (label, values, one_line) in cases {
            let name = words(label, &values);
            let starts: Vec<usize> = name.grapheme_indices(true).map(|(at, _)| at).collect();
            let mut wraps = 0;
            for width in (80..=640).step_by(4) {
                let shown = values.clone();
                let mut harness = egui_kittest::Harness::builder()
                    .with_size(egui::vec2(width as f32, 400.0))
                    .build_ui(move |ui| {
                        let _ = line(ui, label, &shown);
                    });
                super::super::theme::install(&harness.ctx);
                harness.run();
                assert_eq!(
                    node_values(&harness, label),
                    std::slice::from_ref(&name),
                    "at {width}: one value, the plain words"
                );
                let rows = painted_rows(&harness, label);
                assert_eq!(rows.concat(), name, "at {width}: the lines are the words");
                let mut at = 0;
                for row in &rows {
                    assert!(
                        !row.trim_end().ends_with('·'),
                        "at {width}, a line ends at a dot: {rows:?}"
                    );
                    assert!(
                        starts.contains(&at),
                        "at {width}, a line starts inside a character: {rows:?}"
                    );
                    at += row.len();
                }
                wraps += usize::from(rows.len() > 1);
                if width == 640 && one_line {
                    assert_eq!(rows.len(), 1, "wide: one line: {rows:?}");
                }
            }
            assert!(wraps > 0, "{label}: some widths wrap the fact");
        }

        // Dragged across and copied, the fact gives exactly its words, and a drag over its last
        // line exactly that line's words; Tab with a screen reader focuses it.
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(300.0, 200.0))
            .build_ui(|ui| {
                let _ = line(ui, "Languages", languages);
            });
        super::super::theme::install(&harness.ctx);
        harness.run();
        let rect = harness
            .root()
            .children_recursive()
            .find(|node| node.accesskit_node().value().as_deref() == Some(WORDS))
            .map(|node| node.rect())
            .expect("the fact has a node");
        let rows = painted_rows(&harness, "Languages");
        assert!(rows.len() > 1, "300 px wraps the fact: {rows:?}");
        let last_row = rect.height() / rows.len() as f32;
        let end = rect.right_bottom() - egui::vec2(1.0, 4.0);
        assert_eq!(
            drag_copy(&mut harness, rect.left_top() + egui::vec2(1.0, 4.0), end),
            [WORDS],
            "a drag across the fact copies exactly its words"
        );
        let last = egui::pos2(rect.left() + 1.0, rect.bottom() - last_row / 2.0);
        assert_eq!(
            drag_copy(&mut harness, last, end),
            rows.last()
                .map(String::as_str)
                .into_iter()
                .collect::<Vec<_>>(),
            "a drag over the last line copies exactly its words"
        );
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(300.0, 200.0))
            .build_ui(|ui| {
                let _ = line(ui, "Languages", languages);
            });
        super::super::theme::install(&harness.ctx);
        harness
            .ctx
            .options_mut(|options| options.screen_reader = true);
        harness.run();
        harness.key_press(egui::Key::Tab);
        harness.run();
        let focused = harness
            .root()
            .children_recursive()
            .find(|node| node.accesskit_node().is_focused())
            .and_then(|node| node.accesskit_node().value());
        assert_eq!(focused.as_deref(), Some(WORDS), "Tab focuses the fact");

        // The rolling tile has the stat tile's label line and value type; only its value rolls.
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(400.0, 200.0))
            .build_ui(|ui| {
                ui.horizontal(|ui| {
                    super::super::stat_tile(ui, &tile_label("Str", 8), "−1");
                    let link = RollLink::d20(D20::Check, 2, "Dexterity");
                    let _ = roll_tile(ui, &tile_label("Dex", 14), "+2", &link);
                });
            });
        super::super::theme::install(&harness.ctx);
        harness.run();
        let style = |words: &str| {
            harness
                .output()
                .shapes
                .iter()
                .find_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) if text.galley.job.text == words => {
                        let format = &text.galley.job.sections[0].format;
                        Some((format.font_id.clone(), text.fallback_color))
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("{words} is painted"))
        };
        assert_eq!(style("Str: 8"), style("Dex: 14"), "one label line");
        assert_eq!(style("−1").0, style("+2").0, "one value type");
    }
}
