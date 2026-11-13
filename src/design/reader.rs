//! Typographic parts of the one Reader. `library::view` composes them.
use super::{components::rule, extras::space, tokens::*};
use eframe::egui::{self, Ui, vec2};

/// Prose in a side panel is one step smaller than prose at full width.
pub fn prose_type(ui: &Ui) -> Type {
    prose_type_at(ui.available_width())
}

/// The Reader's prose type in a column this wide.
pub fn prose_type_at(width: f32) -> Type {
    if width < READER_MEASURE - 60.0 {
        Type::ProseCompact
    } else {
        Type::Prose
    }
}

/// The fill behind code among words, in the Reader and where a note is typed.
pub fn inline_code_fill() -> egui::Color32 {
    colors().hover
}

/// Keep rules text at a comfortable measure, however wide the window is.
pub fn measure<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(READER_MEASURE));
        add(ui)
    })
    .inner
}

pub fn title(ui: &mut Ui, text: &str) {
    ui.add(egui::Label::new(Type::Display.rich(ui, text).color(colors().text)).wrap());
}

pub fn subtitle(ui: &mut Ui, text: &str) {
    ui.add(egui::Label::new(Type::Small.rich(ui, text).color(colors().muted)).wrap());
}

pub fn section_heading(ui: &mut Ui, text: &str) {
    space(ui, M);
    ui.add(egui::Label::new(Type::Section.rich(ui, text).color(colors().text)).wrap());
    space(ui, 2.0);
    rule(ui);
    space(ui, XS);
}

/// A heavy rule that opens and closes a stat block.
pub fn heavy_rule(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 2.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, colors().rule);
}

const LABEL_WIDTH: f32 = 112.0;

/// About 70 characters of a fact's value, in stat type.
const DETAIL_MEASURE: f32 = 330.0;

/// One row of a reference table: a label column, then a wrapped value.
pub fn detail(ui: &mut Ui, label: &str, value: &str) {
    let color = colors().text;
    detail_row(ui, label, |ui| {
        ui.add(egui::Label::new(Type::Stat.rich(ui, value).color(color)).wrap());
    });
}

/// One detail row of a stat block (Speed, Senses, Languages, …). Every line has the link pitch
/// (`XL`), as the lines of a row with dice links do, so all detail rows of a stat block have one
/// pitch and no row looks grouped with another. The extra height goes below the words.
pub fn stat_detail(ui: &mut Ui, label: &str, value: &str, color: egui::Color32) {
    detail_row(ui, label, |ui| {
        ui.add(
            egui::Label::new(
                Type::Stat
                    .rich(ui, value)
                    .color(color)
                    .line_height(Some(XL)),
            )
            .wrap(),
        );
    });
}

/// A reference row whose value has dice links. A linked line is taller than a stat line, and
/// the label sits on the baseline of the value's first line, as in every other row.
pub fn linked_detail(ui: &mut Ui, label: &str, value: impl FnOnce(&mut Ui)) {
    detail_row(ui, label, value);
}

fn detail_row(ui: &mut Ui, label: &str, value: impl FnOnce(&mut Ui)) {
    let c = colors();
    let label = human_label(label);
    let font = Type::StatLabel.font(ui);
    let galley = ui.painter().layout_no_wrap(label.clone(), font, c.text);
    // A long label takes its own line. A label is never cut.
    if galley.size().x > LABEL_WIDTH - XS {
        ui.label(Type::StatLabel.rich(ui, label).color(c.text));
        value(ui);
        return;
    }
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = S;
        let (rect, _) =
            ui.allocate_exact_size(vec2(LABEL_WIDTH, Type::Stat.line()), egui::Sense::hover());
        // The label is painted after the value, on the baseline of the value's first painted
        // words. A linked line puts its words where its own layout puts them, so the label reads
        // that place instead of guessing it.
        let slot = ui.painter().add(egui::Shape::Noop);
        // A long value keeps the reader's measure in this smaller type.
        let room = vec2(ui.available_width().min(DETAIL_MEASURE), 0.0);
        ui.allocate_ui(room, value);
        let own = first_baseline(&galley).unwrap_or(0.0);
        // The move can be up as well as down: a value's words can sit higher in their line than
        // the label's own words do.
        let drop = first_painted_baseline(ui, slot, rect.top())
            .map_or(0.0, |baseline| baseline - rect.top() - own);
        ui.painter().set(
            slot,
            egui::Shape::galley(rect.min + vec2(0.0, drop), galley, c.text),
        );
    });
}

/// The baseline of a galley's first glyph, from the galley's top.
fn first_baseline(galley: &egui::Galley) -> Option<f32> {
    galley
        .rows
        .iter()
        .find_map(|row| row.glyphs.first().map(|glyph| row.pos.y + glyph.pos.y))
}

/// The baseline of the first words painted on this layer after `slot`. Only the shapes after the
/// slot are read, so the cost follows the value, not the page. Words whose top is more than one
/// control height from the row's `top` are not the value's first line (for example a wrapped value
/// that a scroll area has moved), so they give no baseline.
fn first_painted_baseline(ui: &Ui, slot: egui::layers::ShapeIdx, top: f32) -> Option<f32> {
    ui.ctx().graphics(|layers| {
        let list = layers.get(ui.layer_id())?;
        let (text_top, baseline) =
            list.all_entries()
                .skip(slot.0 + 1)
                .find_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) => first_baseline(&text.galley)
                        .map(|baseline| (text.pos.y, text.pos.y + baseline)),
                    _ => None,
                })?;
        ((text_top - top).abs() <= Density::Read.control()).then_some(baseline)
    })
}

/// A tight table of facts. Rulebooks use rows for this, not tiles.
pub fn facts(ui: &mut Ui, facts: &[(String, String)]) {
    if facts.is_empty() {
        return;
    }
    ui.spacing_mut().item_spacing.y = 2.0;
    for (label, value) in facts {
        detail(ui, label, value);
    }
    space(ui, S);
}

pub fn identity(ui: &mut Ui, text: &str) {
    ui.add(egui::Label::new(Type::ProseItalic.rich(ui, text).color(colors().muted)).wrap());
    space(ui, S);
}

/// Scoped to readers: editors and other workspaces keep their own colours.
pub fn markdown(ui: &mut Ui, text: &str) {
    crate::markdown_view::show_reader(ui, text);
}

/// The same, as compact field rows.
pub fn compact_markdown(ui: &mut Ui, text: &str) {
    crate::markdown_view::show_compact(ui, text);
}

/// A named run-in paragraph's title: `Rend.`, `Acid Breath (Recharge 5–6).` Title case,
/// a few words, and an optional qualifier in brackets. A sentence does not match, so it
/// never turns bold.
pub fn run_in_title(line: &str) -> Option<&str> {
    let mut depth = 0i32;
    for (index, c) in line.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            '.' if depth == 0 => {
                let rest = &line[index + 1..];
                let title = &line[..index];
                return (rest.is_empty() || rest.starts_with(' '))
                    .then_some(title)
                    .filter(|title| title_words(title));
            }
            _ => {}
        }
    }
    None
}

fn title_words(title: &str) -> bool {
    let name = match title.split_once(" (") {
        Some((name, qualifier)) if qualifier.ends_with(')') => name,
        Some(_) => return false,
        None => title,
    };
    let words: Vec<&str> = name.split_whitespace().collect();
    let small = |word: &str| {
        matches!(
            word,
            "a" | "an"
                | "and"
                | "at"
                | "by"
                | "for"
                | "from"
                | "in"
                | "of"
                | "on"
                | "or"
                | "the"
                | "to"
                | "with"
        )
    };
    (1..=7).contains(&words.len())
        && words[0].starts_with(char::is_uppercase)
        && !small(words[words.len() - 1])
        && words.iter().all(|word| {
            (word.starts_with(|c: char| c.is_uppercase() || c.is_ascii_digit()) || small(word))
                && word
                    .chars()
                    .all(|c| c.is_alphanumeric() || "'\u{2019}-/".contains(c))
        })
}

/// Bold a paragraph's run-in title, if it has one.
pub fn run_in_markdown(paragraph: &str) -> String {
    let paragraph = paragraph.trim_start();
    match run_in_title(paragraph) {
        Some(title) => {
            let rest = &paragraph[title.len() + 1..];
            format!("**{title}.**{rest}")
        }
        None => paragraph.to_owned(),
    }
}

/// Imported rules use short named paragraphs for actions and benefits.
pub fn source_prose(ui: &mut Ui, text: &str) {
    let text = text
        .split("\n\n")
        .map(run_in_markdown)
        .collect::<Vec<_>>()
        .join("\n\n");
    markdown(ui, &text);
}

pub fn human_label(value: &str) -> String {
    let value = value.replace('_', " ");
    let mut chars = value.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + chars.as_str()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use splat::content::dice_text::{D20, RollLink};

    /// The baselines of the label and of the value's first word, as painted.
    fn baselines(row: impl Fn(&mut Ui), label: &str, first_word: &str) -> (f32, f32) {
        let context = egui::Context::default();
        super::super::theme::install(&context);
        let mut found = (None, None);
        // The second pass draws with the first pass's layout memory, as the app does.
        for _ in 0..2 {
            let output = context.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| row(ui));
            });
            found = (None, None);
            for shape in &output.shapes {
                let egui::Shape::Text(text) = &shape.shape else {
                    continue;
                };
                for row in &text.galley.rows {
                    let words: String = row.glyphs.iter().map(|glyph| glyph.chr).collect();
                    let baseline = |at: usize| text.pos.y + row.pos.y + row.glyphs[at].pos.y;
                    if words == label && found.0.is_none() {
                        found.0 = Some(baseline(0));
                    } else if let Some(at) = words.find(first_word)
                        && found.1.is_none()
                    {
                        found.1 = Some(baseline(words[..at].chars().count()));
                    }
                }
            }
        }
        (
            found.0.expect("the label is painted"),
            found.1.expect("the first value word is painted"),
        )
    }

    #[test]
    fn a_linked_row_label_shares_its_first_value_baseline() {
        let skills = format!(
            "{}, {}",
            RollLink::d20(D20::Skill, 11, "Perception").markdown("Perception +11"),
            RollLink::d20(D20::Skill, 3, "Stealth").markdown("Stealth +3"),
        );
        let (label, value) = baselines(
            |ui| {
                linked_detail(ui, "Skills", |ui| {
                    crate::markdown_view::show_compact(ui, &skills);
                });
            },
            "Skills",
            "Perception",
        );
        assert!(
            (label - value).abs() <= 0.5,
            "the Skills label baseline {label} differs from Perception's {value}"
        );
    }

    #[test]
    fn words_far_below_the_row_do_not_pull_the_label_down() {
        // A value whose first painted words start more than a control height below the row's
        // top, as a scrolled or wrapped value can: the label stays on its own line.
        let (label, value) = baselines(
            |ui| {
                linked_detail(ui, "Skills", |ui| {
                    ui.vertical(|ui| {
                        space(ui, Density::Read.control() * 3.0);
                        ui.label(Type::Stat.rich(ui, "Perception +11"));
                    });
                });
            },
            "Skills",
            "Perception",
        );
        assert!(
            value - label > Density::Read.control() * 2.0,
            "the Skills label {label} followed words {value} that are not on its line"
        );
    }

    #[test]
    fn a_plain_row_label_shares_its_value_baseline() {
        let (label, value) = baselines(
            |ui| detail(ui, "Senses", "Darkvision 60 ft., Passive Perception 15"),
            "Senses",
            "Darkvision",
        );
        assert!(
            (label - value).abs() <= 0.5,
            "the Senses label baseline {label} differs from its value's {value}"
        );
    }
}
