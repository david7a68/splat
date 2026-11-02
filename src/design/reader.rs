//! Typographic parts of the one Reader. `library::view` composes them.
use super::{components::rule, tokens::*};
use eframe::egui::{self, Ui, vec2};

/// Prose in a side panel is one step smaller than prose at full width.
pub fn prose_type(ui: &Ui) -> Type {
    if ui.available_width() < READER_MEASURE - 60.0 {
        Type::ProseCompact
    } else {
        Type::Prose
    }
}

pub const READER_MEASURE_WIDTH: f32 = READER_MEASURE;

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
    ui.add_space(M);
    ui.add(egui::Label::new(Type::Section.rich(ui, text).color(colors().text)).wrap());
    ui.add_space(2.0);
    rule(ui);
    ui.add_space(XS);
}

/// A heavy rule that opens and closes a stat block.
pub fn heavy_rule(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 2.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, colors().rule);
}

const LABEL_WIDTH: f32 = 112.0;

/// One row of a reference table: a label column, then a wrapped value.
pub fn detail(ui: &mut Ui, label: &str, value: &str) {
    let c = colors();
    let label = human_label(label);
    let font = Type::StatLabel.font(ui);
    let galley = ui
        .painter()
        .layout_no_wrap(label.clone(), font.clone(), c.text);
    // A long label takes its own line. A label is never cut.
    if galley.size().x > LABEL_WIDTH - XS {
        ui.label(Type::StatLabel.rich(ui, label).color(c.text));
        ui.add(egui::Label::new(Type::Stat.rich(ui, value).color(c.text)).wrap());
        return;
    }
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = S;
        let (rect, _) =
            ui.allocate_exact_size(vec2(LABEL_WIDTH, Type::Stat.line()), egui::Sense::hover());
        ui.painter().galley(rect.min, galley, c.text);
        ui.add(egui::Label::new(Type::Stat.rich(ui, value).color(c.text)).wrap());
    });
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
    ui.add_space(S);
}

pub fn identity(ui: &mut Ui, text: &str) {
    ui.add(egui::Label::new(Type::ProseItalic.rich(ui, text).color(colors().muted)).wrap());
    ui.add_space(S);
}

/// Scoped to readers: editors and other workspaces keep their own colours.
pub fn markdown(ui: &mut Ui, text: &str, compact: bool) {
    crate::markdown_view::show_reader(ui, text, compact);
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
    markdown(ui, &text, false);
}

pub fn human_label(value: &str) -> String {
    let value = value.replace('_', " ");
    let mut chars = value.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + chars.as_str()
    })
}
