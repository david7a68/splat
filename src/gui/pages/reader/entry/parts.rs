//! The Reader's typographic parts (egui `design::reader`): a title and its subtitle, the heavy
//! rule that opens and closes a stat block, an identity line, labelled detail rows, a table of
//! facts, a section heading with its hairline, and a block of rules prose with its run-in titles.
//!
//! egui leaves `LINE_GAP` after every item of a column; these parts end with that gap themselves
//! where egui's item did, so a column of them keeps egui's pitch with no gap of its own.
use crate::design::reader::{human_label, run_in_markdown};
use crate::gui::kit;
use crate::gui::theme::{LINE_GAP, S, Type, TypeExt, XL, XS, colors};
use gpui::{
    AnyElement, Div, ElementId, IntoElement, ParentElement, Pixels, SharedString, Styled, TextRun,
    Window, div, px,
};

/// A detail row's label column.
pub const LABEL_WIDTH: Pixels = px(112.0);

/// About 70 characters of a fact's value, in stat type.
pub const DETAIL_MEASURE: Pixels = px(330.0);

/// An item of a column: `content`, then the gap egui leaves after an item.
pub fn item(content: impl IntoElement) -> Div {
    div().flex().flex_col().pb(LINE_GAP).child(content)
}

/// An entry's name, in `Display`.
pub fn title(text: impl Into<SharedString>) -> Div {
    item(kit::text(Type::Display, colors().text, text))
}

/// The line under an entry's name, in muted `Small`.
pub fn subtitle(text: impl Into<SharedString>) -> Div {
    item(kit::text(Type::Small, colors().muted, text))
}

/// A heavy rule that opens and closes a stat block.
pub fn heavy_rule() -> Div {
    item(div().w_full().h(px(2.0)).flex_none().bg(colors().rule))
}

/// A hairline across the column.
pub fn rule() -> Div {
    item(kit::rule())
}

/// An italic muted identity line (`Huge · Dragon (Chromatic) · Chaotic Evil`), then `S`.
pub fn identity(text: impl Into<SharedString>) -> Div {
    item(kit::text(Type::ProseItalic, colors().muted, text)).pb(LINE_GAP + S)
}

/// A reader's section heading (egui `Heading::Section`): `XL` above it unless it is `first`, its
/// words, a hairline 2 px under them, then `XS`.
pub fn section_heading(title: impl Into<SharedString>, first: bool) -> Div {
    div()
        .flex()
        .flex_col()
        .when_not_first(first, XL)
        .child(kit::text(Type::Section, colors().text, title))
        .child(kit::gap(px(2.0)))
        .child(kit::rule())
        .child(kit::gap(XS))
}

trait Above: Styled + Sized {
    fn when_not_first(self, first: bool, above: Pixels) -> Self {
        if first { self } else { self.pt(above) }
    }
}
impl<T: Styled> Above for T {}

/// Muted `Small` help words at most the reader's measure wide (egui `design::help`).
pub fn help(text: impl Into<SharedString>) -> Div {
    item(kit::text(Type::Small, colors().muted, text))
}

/// The width `text` takes on one line in `role`.
fn width(text: &str, role: Type, window: &Window) -> Pixels {
    let mut font = gpui::font(role.family());
    font.weight = role.weight();
    window
        .text_system()
        .shape_line(
            SharedString::from(text.to_owned()),
            role.size(),
            &[TextRun {
                len: text.len(),
                font,
                color: colors().text,
                background_color: None,
                underline: None,
                strikethrough: None,
            }],
            None,
        )
        .width
}

/// One detail row: a `StatLabel` label in its own column, then `value` wrapped at the detail
/// measure. A label too long for its column takes its own line; a label is never cut. `line` is
/// the value's line height, which the label shares so the two stand on one baseline.
pub fn detail_row(label: &str, line: Pixels, value: impl IntoElement, window: &Window) -> Div {
    let c = colors();
    let label = human_label(label);
    let words = || {
        div()
            .ty(Type::StatLabel)
            .line_height(line)
            .text_color(c.text)
            .child(label.clone())
    };
    if width(&label, Type::StatLabel, window) > LABEL_WIDTH - XS {
        return div()
            .flex()
            .flex_col()
            .pb(px(2.0))
            .child(item(words()))
            .child(div().max_w(DETAIL_MEASURE).child(value));
    }
    div()
        .flex()
        .flex_row()
        .gap(S)
        .pb(px(2.0))
        .child(div().w(LABEL_WIDTH).flex_none().child(words()))
        .child(div().flex_1().min_w_0().max_w(DETAIL_MEASURE).child(value))
}

/// One row of a reference table: a label column, then a wrapped `Stat` value.
pub fn detail(label: &str, value: impl Into<SharedString>, window: &Window) -> Div {
    let line = Type::Stat.line();
    detail_row(
        label,
        line,
        div()
            .ty(Type::Stat)
            .text_color(colors().text)
            .child(value.into()),
        window,
    )
}

/// One detail row of a stat block (Speed, Senses, …): every line has the link pitch (`XL`), as
/// the lines of a row with dice links do.
pub fn stat_detail(
    label: &str,
    value: impl Into<SharedString>,
    color: gpui::Hsla,
    window: &Window,
) -> Div {
    detail_row(
        label,
        XL,
        div()
            .ty(Type::Stat)
            .line_height(XL)
            .text_color(color)
            .child(value.into()),
        window,
    )
}

/// A tight table of facts, then `S`. Rulebooks use rows for this, not tiles.
pub fn facts(facts: &[(String, String)], window: &Window) -> Option<Div> {
    if facts.is_empty() {
        return None;
    }
    Some(
        div()
            .flex()
            .flex_col()
            .pb(S)
            .children(facts.iter().map(|(label, value)| detail(label, value.clone(), window))),
    )
}

/// Imported rules text: each paragraph's run-in title (`Bite.`) in bold, in the reader's prose.
pub fn source_prose(id: impl Into<ElementId>, text: &str, document: Option<uuid::Uuid>) -> AnyElement {
    let text = text
        .split("\n\n")
        .map(run_in_markdown)
        .collect::<Vec<_>>()
        .join("\n\n");
    kit::Markdown::new(id, text)
        .document(document)
        .into_any_element()
}

/// The child id `name` under `id`.
pub fn child(id: &ElementId, name: impl Into<SharedString>) -> ElementId {
    ElementId::NamedChild(std::sync::Arc::new(id.clone()), name.into())
}
