//! A sheet's labelled fact (ruling M5, R40): one form, `Label: value`, its values joined by the
//! middle dot (`Coins: 12 cp · 40 sp`). The part owns the colon, the separator and how a joined
//! line wraps; a caller gives a label and its values, never a separator.
//!
//! - [`line`]: a fact on a line of its own. Each value after the first is one unit with its dot
//!   (`· Elvish`), laid out whole: a line breaks only between units, so no line ends at a dot
//!   (round 31 AM15). An empty value is left out, with its dot. It is a label: the DM can select
//!   and copy it, and a screen reader can focus it; its accessible value is the plain words.
//! - [`tile_label`]: a stat tile's label that holds its own number (`Str: 8` over `+1`), in the
//!   same form (round 31 SL12).
//! - [`roll_tile`]: the rolling stat tile, a [`super::stat_tile`] whose value rolls.
//!
//! ```
//! use splat::fact::{tile_label, words};
//! assert_eq!(words("Coins", ["12 cp", "40 sp"]), "Coins: 12 cp · 40 sp");
//! assert_eq!(words("Resistance", ["Fire", ""]), "Resistance: Fire");
//! assert_eq!(words("Resistance", [" "]), "Resistance: —");
//! assert_eq!(tile_label("Str", 8), "Str: 8");
//! ```

use std::fmt::Display;

use eframe::egui::{self, Response, Ui};
use splat::content::dice_text::RollLink;

use super::{Type, colors};

/// The separator the DM reads between two values.
const SEPARATOR: &str = " · ";

/// What a fact with no value says: the sheet's mark for a value it does not have.
const NONE: &str = "—";

/// The values that have words; an empty value is left out, and so is its separator.
fn present(values: impl IntoIterator<Item = impl Display>) -> Vec<String> {
    values
        .into_iter()
        .map(|value| value.to_string())
        .filter(|value| !value.trim().is_empty())
        .collect()
}

/// `Resistance: Fire · Cold`: the one form of a fact, with no empty value (`Resistance: —`
/// when none is left). It is also the accessible value of [`line`].
pub fn words(label: &str, values: impl IntoIterator<Item = impl Display>) -> String {
    units(label, &present(values)).join(" ")
}

/// The fact's units: `Label: Common`, then `· Elvish` for each value after the first. A line
/// breaks only between units.
fn units(label: &str, values: &[String]) -> Vec<String> {
    match values.split_first() {
        None => vec![format!("{label}: {NONE}")],
        Some((first, rest)) => std::iter::once(format!("{label}: {first}"))
            .chain(
                rest.iter()
                    .map(|value| format!("{}{value}", SEPARATOR.trim_start())),
            )
            .collect(),
    }
}

/// The units on lines of at most `width`, broken here and nowhere else: a unit joins the line
/// when the line with it still fits, else it starts the next line. A unit wider than a line is
/// broken at its spaces, its dot kept with the first word after it; a word wider than a line is
/// broken between its letters, at least three to a line. So no line ends at a dot, whatever the
/// script (`epaint` may break before a CJK letter, so it never wraps these lines itself).
fn lines(ui: &Ui, units: &[String], width: f32) -> String {
    let wide = |text: &str| {
        egui::WidgetText::from(text)
            .into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                egui::TextStyle::Body,
            )
            .size()
            .x
    };
    let mut rows: Vec<String> = Vec::new();
    let mut row = String::new();
    // Puts `piece` on the line when it fits there, else starts the next line with it; says
    // whether it stands on a line now.
    let place = |rows: &mut Vec<String>, row: &mut String, piece: &str| {
        let joined = if row.is_empty() {
            piece.to_owned()
        } else {
            format!("{row} {piece}")
        };
        if wide(&joined) <= width {
            *row = joined;
            return true;
        }
        if !row.is_empty() {
            rows.push(std::mem::take(row));
        }
        if wide(piece) <= width {
            piece.clone_into(row);
            return true;
        }
        false
    };
    for unit in units {
        if place(&mut rows, &mut row, unit) {
            continue;
        }
        let mut words = unit.split(' ');
        let mut pieces: Vec<String> = Vec::new();
        if let Some(first) = words.next() {
            match (first, words.next()) {
                ("·", Some(second)) => pieces.push(format!("· {second}")),
                (first, second) => pieces.extend(
                    [Some(first), second]
                        .into_iter()
                        .flatten()
                        .map(str::to_owned),
                ),
            }
        }
        pieces.extend(words.map(str::to_owned));
        for piece in pieces {
            if place(&mut rows, &mut row, &piece) {
                continue;
            }
            for letter in piece.chars() {
                let joined = format!("{row}{letter}");
                if row.chars().count() < 3 || wide(&joined) <= width {
                    row = joined;
                } else {
                    rows.push(std::mem::take(&mut row));
                    row.push(letter);
                }
            }
        }
    }
    if !row.is_empty() {
        rows.push(row);
    }
    rows.join("\n")
}

/// A fact on a line of its own, broken between its units at the room it has, so no line ends at
/// a dot. A label: selectable, and focusable for a screen reader. Its accessible value is
/// [`words`], the plain words with no line break.
pub fn line(ui: &mut Ui, label: &str, values: impl IntoIterator<Item = impl Display>) -> Response {
    let values = present(values);
    let text = lines(ui, &units(label, &values), ui.available_width());
    // The lines are broken above; the label draws them as they are.
    let response = ui.add(egui::Label::new(text).extend());
    let name = words(label, &values);
    // A label's value comes from its painted lines; the fact's is its words on one line.
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_value(name));
    response
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

    /// M5: one form for every fact, in words and in the accessible value: the colon, then the
    /// values joined by the middle dot, an empty value left out with its dot. A tile label that
    /// holds its own number takes the same form. Drawn at every width, a line breaks only
    /// between a value's units, so no painted line ends at a dot: also before a CJK value (which
    /// `epaint` may break before), with an empty value, and with one long value. The fact is a
    /// label: dragged across, it selects and copies; with a screen reader, Tab focuses it.
    #[test]
    fn a_fact_says_label_colon_values_and_never_ends_a_line_at_a_dot() {
        const WORDS: &str = "Languages: Common · Elvish · Dwarvish · Sylvan · Thieves’ Cant";
        let languages = ["Common", "Elvish", "Dwarvish", "Sylvan", "Thieves’ Cant"];
        assert_eq!(words("Languages", languages), WORDS);
        assert_eq!(words("Darkvision", ["60 ft."]), "Darkvision: 60 ft.");
        assert_eq!(words("Resistance", ["Fire", ""]), "Resistance: Fire");
        assert_eq!(words("Resistance", ["", " "]), "Resistance: —");
        assert_eq!(tile_label("Str", 8), "Str: 8");
        const LONG: &str = "Speaks with every creature it can see within sixty feet, also through \
            a wall of stone or a closed door";
        let cases: [(&str, Vec<&str>); 4] = [
            ("Languages", languages.to_vec()),
            ("Resistance", vec!["Fire", "火焰"]),
            ("Resistance", vec!["Fire", ""]),
            ("Telepathy", vec![LONG]),
        ];
        for (label, values) in cases {
            let name = words(label, &values);
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
                    [name.clone()],
                    "at {width}: one value, the plain words"
                );
                let rows = painted_rows(&harness, label);
                assert!(!rows.is_empty(), "{label} is painted at {width}");
                for row in &rows {
                    assert!(
                        !row.trim_end().ends_with('·'),
                        "at {width}, a line ends at a dot: {rows:?}"
                    );
                }
                wraps += usize::from(rows.len() > 1);
                if width == 640 && label != "Telepathy" {
                    assert_eq!(rows.len(), 1, "wide: one line: {rows:?}");
                }
            }
            assert!(wraps > 0, "{label}: some widths wrap the fact");
        }

        // Dragged across and copied, the fact gives its words; Tab with a screen reader focuses it.
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
        let (from, to) = (
            rect.left_top() + egui::vec2(1.0, 4.0),
            rect.right_bottom() - egui::vec2(1.0, 4.0),
        );
        harness.hover_at(from);
        harness.step();
        for (pos, pressed) in [(from, true), (to, true), (to, false)] {
            harness.hover_at(pos);
            harness.event(egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
            harness.step();
        }
        harness.event(egui::Event::Copy);
        harness.step();
        let copied: Vec<String> = harness
            .output()
            .platform_output
            .commands
            .iter()
            .filter_map(|command| match command {
                egui::OutputCommand::CopyText(text) => Some(text.clone()),
                _ => None,
            })
            .collect();
        assert!(
            copied
                .iter()
                .any(|text| text.contains("Common") && text.contains("Cant")),
            "a drag selects the fact and Copy gives its words: {copied:?}"
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
