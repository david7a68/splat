//! A sheet's labelled fact (ruling M5, R40): one form, `Label: value`, its values joined by the
//! middle dot (`Coins: 12 cp · 40 sp`). The part owns the colon, the separator and how a joined
//! line wraps; a caller gives a label and its values, never a separator.
//!
//! - [`line`]: a fact on a line of its own. Where the line wraps, each middle dot moves with the
//!   value after it, so no line ends at a dot (round 31 AM15). The accessible name is the plain
//!   words.
//! - [`tile_label`]: a stat tile's label that holds its own number (`Str: 8` over `+1`), in the
//!   same form (round 31 SL12).
//! - [`roll_tile`]: the rolling stat tile, a [`super::stat_tile`] whose value rolls.
//!
//! ```
//! use splat::fact::{tile_label, words};
//! assert_eq!(words("Coins", ["12 cp", "40 sp"]), "Coins: 12 cp · 40 sp");
//! assert_eq!(tile_label("Str", 8), "Str: 8");
//! ```

use std::fmt::Display;

use eframe::egui::{self, Response, Ui};
use splat::content::dice_text::RollLink;

use super::{Type, colors};

/// The separator the DM reads between two values, also in the accessible name.
const SEPARATOR: &str = " · ";

/// The separator as painted: a no-break space after the dot keeps the dot with the value after
/// it (`epaint` breaks a line at a space, never at a no-break space).
const PAINTED: &str = " ·\u{a0}";

/// `Resistance: Fire · Cold`: the one form of a fact. It is also the accessible name of
/// [`line`].
pub fn words(label: &str, values: impl IntoIterator<Item = impl Display>) -> String {
    joined(label, values, SEPARATOR)
}

fn joined(label: &str, values: impl IntoIterator<Item = impl Display>, separator: &str) -> String {
    let values: Vec<String> = values.into_iter().map(|value| value.to_string()).collect();
    format!("{label}: {}", values.join(separator))
}

/// A fact on a line of its own, wrapped at the room it has. At a break the middle dot moves with
/// the value after it, so no line ends at a dot; the accessible name is [`words`].
pub fn line(ui: &mut Ui, label: &str, values: impl IntoIterator<Item = impl Display>) -> Response {
    let values: Vec<String> = values.into_iter().map(|value| value.to_string()).collect();
    // Laid out and painted here, as a label is, so the one accessible name is the plain words:
    // the painted text keeps its no-break spaces out of the name (a label would expose them).
    let galley = egui::WidgetText::from(joined(label, &values, PAINTED)).into_galley(
        ui,
        Some(egui::TextWrapMode::Wrap),
        ui.available_width(),
        egui::TextStyle::Body,
    );
    let (rect, response) = ui.allocate_exact_size(galley.size(), egui::Sense::hover());
    let name = words(label, &values);
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &name));
    if ui.is_rect_visible(rect) {
        ui.painter()
            .galley(rect.min, galley, ui.style().visuals.text_color());
    }
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

    /// M5: one form for every fact, in words and in the accessible name: the colon, then the
    /// values joined by the middle dot. A tile label that holds its own number takes the same
    /// form. Drawn, a fact keeps each middle dot with the value after it at every width, so no
    /// painted line ends at a dot; at a wide width it is one line.
    #[test]
    fn a_fact_says_label_colon_values_and_never_ends_a_line_at_a_dot() {
        const WORDS: &str = "Languages: Common · Elvish · Dwarvish · Sylvan · Thieves’ Cant";
        let values = ["Common", "Elvish", "Dwarvish", "Sylvan", "Thieves’ Cant"];
        assert_eq!(words("Languages", values), WORDS);
        assert_eq!(words("Darkvision", ["60 ft."]), "Darkvision: 60 ft.");
        assert_eq!(tile_label("Str", 8), "Str: 8");
        let mut wraps = 0;
        for width in (80..=520).step_by(4) {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(egui::vec2(width as f32, 300.0))
                .build_ui(move |ui| {
                    let _ = line(ui, "Languages", values);
                });
            super::super::theme::install(&harness.ctx);
            harness.run();
            let names: Vec<(String, Option<String>)> = harness
                .root()
                .children_recursive()
                .map(|node| {
                    let node = node.accesskit_node();
                    (node.label().unwrap_or_default(), node.value())
                })
                .filter(|(label, value)| {
                    label.starts_with("Languages")
                        || value.as_deref().is_some_and(|v| v.starts_with("Languages"))
                })
                .collect();
            assert_eq!(
                names,
                [(String::new(), Some(WORDS.to_owned()))],
                "at {width}: one name, with plain spaces"
            );
            let rows: Vec<String> = harness
                .output()
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) if text.galley.job.text.starts_with("Languages") => {
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
                .collect();
            assert!(!rows.is_empty(), "the fact is painted at {width}");
            for row in &rows {
                assert!(
                    !row.trim_end().ends_with('·'),
                    "at {width}, a line ends at a dot: {rows:?}"
                );
            }
            wraps += usize::from(rows.len() > 1);
            if width == 520 {
                assert_eq!(rows.len(), 1, "wide: one line: {rows:?}");
            }
        }
        assert!(wraps > 0, "some widths wrap the fact");
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
