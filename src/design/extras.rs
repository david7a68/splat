//! Package X: design extras. Each part decides a thing that callers used to decide one by one and
//! sometimes got wrong: the space around headings and help (N3), the check on a selected choice
//! (N1). `clippy.toml` (section "Package X") refuses the raw egui calls and the old parts these
//! replace; each staged use is one row of `design::results::STAGED_ALLOWS` (`Ban::Extras(n)`).
//!
//! What is enforced for space: `Ui::add_space` and `Ui::allocate_space` are refused outside
//! `src/design/` (a raw amount becomes a `Gap` token), the heading adds the space above it, and a
//! field adds `L` after its help. Other raw spacing (`ui.spacing_mut().item_spacing.y = …`, 54
//! assignments in 20 files) is a reviewer duty; the gate's alignment and stillness checks see its
//! effects. A caller of `choice` gives words and whether the choice is the selected one.
use super::{components::*, icons, tokens::*};
use egui::{self, Align2, Response, Sense, Ui, WidgetInfo, WidgetType, pos2, vec2};
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Space (N3)

/// A gap from the spacing scale. No other amount exists, so a caller cannot nudge a gap by a few
/// pixels. The space above a heading and after a field's help are not gaps: the heading and the
/// field add them.
///
/// ```
/// fn f(ui: &mut egui::Ui) {
///     splat_design::extras::gap(ui, splat_design::extras::Gap::L);
/// }
/// ```
///
/// A raw number is not a gap:
///
/// ```compile_fail,E0308
/// fn f(ui: &mut egui::Ui) {
///     splat_design::extras::gap(ui, 10.0);
/// }
/// ```
///
/// The design's own raw space is not public:
///
/// ```compile_fail,E0603
/// fn f(ui: &mut egui::Ui) {
///     splat_design::extras::space(ui, 10.0);
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gap {
    XS,
    S,
    M,
    L,
    XL,
    XXL,
}

impl Gap {
    pub const fn size(self) -> f32 {
        match self {
            Self::XS => XS,
            Self::S => S,
            Self::M => M,
            Self::L => L,
            Self::XL => XL,
            Self::XXL => XXL,
        }
    }
}

/// Space between two things in a column or on a line: one step of the scale.
pub fn gap(ui: &mut Ui, gap: Gap) {
    space(ui, gap.size());
}

/// How far one level of a block quote is set in from the text edge (`quote`).
pub const QUOTE_INDENT: f32 = L;

/// A block quote's words at its `depth` (1 for a quote, 2 for a quote in a quote): set in by
/// `QUOTE_INDENT` a level from the text edge, on one line with nothing between (layout PR 10c,
/// inventory M0409: the reader names the depth, the part owns the indent).
pub fn quote<R>(
    ui: &mut Ui,
    depth: usize,
    add: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<R> {
    indent(ui, depth, Gap::L, add)
}

/// `add` set in by `levels` steps of `step` from the text edge, on one line with nothing between
/// (a tree's rows by depth, a quote by level): the caller names the level, the part owns the
/// indent, and no item spacing adds to it.
pub fn indent<R>(
    ui: &mut Ui,
    levels: usize,
    step: Gap,
    add: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<R> {
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        space(ui, step.size() * levels as f32);
        ui.vertical(add).inner
    })
}

/// `add` drawn `drop` below the top of its slot. The slot is `size` wide and still holds the
/// drop, so what follows starts below the block, as when a space came first; the block is placed
/// by its rect, so a slot taller than the block never moves it.
pub fn dropped<R>(ui: &mut Ui, size: egui::Vec2, drop: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    let column = egui::Layout::top_down(egui::Align::Min);
    ui.allocate_ui_with_layout(size, column, |ui| {
        let mut room = ui.available_rect_before_wrap();
        room.min.y += drop;
        room.max.y = room.max.y.max(room.min.y);
        ui.scope_builder(egui::UiBuilder::new().max_rect(room).layout(column), add)
            .inner
    })
    .inner
}

/// The first line of a row, from the row's top. Every control that stands on it has its centre on
/// the line's centre, as the first line of the row's name does (`FirstLine::slot`).
#[derive(Clone, Copy, Debug)]
pub struct FirstLine {
    /// The line's top, from the row's top.
    pub top: f32,
    pub height: f32,
}

impl FirstLine {
    /// A slot of the row (`size`, from the row's top) holding one control `control` high, placed
    /// by its rect on the line (`dropped`).
    pub fn slot<R>(
        self,
        ui: &mut Ui,
        size: egui::Vec2,
        control: f32,
        add: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        dropped(ui, size, self.top + (self.height - control) / 2.0, add)
    }
}

/// `add`'s block, which takes no room, and leaves no item gap, when it draws nothing. Says
/// whether it drew. (A `Ui::scope` always allocates, and its item gap with it.)
pub fn unless_empty<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> (R, bool) {
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(ui.available_rect_before_wrap())
            .layout(*ui.layout()),
    );
    let inner = add(&mut child);
    let rect = child.min_rect();
    let drew = rect.height() > 0.0;
    if drew {
        ui.allocate_rect(rect, Sense::hover());
    }
    (inner, drew)
}

/// Exactly one step of the scale between the item above (or before) and the next, whatever the
/// item spacing is: the space is the step less the spacing egui already leaves between two items,
/// along the layout's main axis, and never less than none. `gap` adds a step on top of that
/// spacing; this keeps the step the whole distance. A caller no longer reads `item_spacing`.
pub fn gap_to_next(ui: &mut Ui, gap: Gap) {
    let spacing = ui.spacing().item_spacing;
    let along = if ui.layout().main_dir().is_horizontal() {
        spacing.x
    } else {
        spacing.y
    };
    space(ui, (gap.size() - along).max(0.0));
}

/// `add`'s block, and room after it so that what follows starts at least `height` below where
/// the block starts: a line that keeps one height while what it shows changes (a control, then
/// a status in its place; layout step 11c, M0452), so what follows never moves. `height` comes
/// from tokens (a control's height and a gap), never from the content.
pub fn at_least<R>(ui: &mut Ui, height: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    let top = ui.cursor().top();
    let inner = add(ui);
    let used = ui.cursor().top() - top;
    space(ui, (height - used).max(0.0));
    inner
}

/// The room of one line of `kind` with no words in it (a line kept for words still to come; a
/// marker's placement line before the marker is added, layout step 11c, M0478): as tall as a
/// label of that type, with the item gap after it, so what follows stands where it will stand
/// when the words come. It draws nothing and has no accessible node.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn empty_line(ui: &mut Ui, kind: super::Type) {
    let height = egui::WidgetText::from(kind.rich(ui, "M"))
        .into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            egui::TextStyle::Body,
        )
        .size()
        .y;
    ui.allocate_space(egui::vec2(0.0, height));
}

/// The design's own space: any amount, for the parts that measure it. Only `src/design/` may use
/// it, so a caller's space is always a [`Gap`].
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub(in crate::design) fn space(ui: &mut Ui, amount: f32) {
    ui.add_space(amount);
}

#[cfg(feature = "test-support")]
pub fn test_space(ui: &mut Ui, amount: f32) {
    space(ui, amount);
}

/// What a heading is. Each kind has its type, its colour and the space above it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Heading<'a> {
    /// A list group: capitals in the text colour, its nonzero count muted, no rule.
    Group { title: &'a str, count: usize },
    /// A small group label in muted capitals (a form's group, a column's name).
    Label(&'a str),
    /// A reader's or stat block's serif section heading over a hairline.
    Section(&'a str),
    /// A panel's, a form's or a band's own title, in the heading type and the text colour: what
    /// the block under it is (`Short rest`, the run strip's `Round 3`).
    Panel(&'a str),
    /// A heading that says a state in its colour, its icon first, as a status does: colour is
    /// never the only signal (`⚠ Hit points conflict`). Its accessible name is its word.
    Status(Status<'a>),
    /// A sheet's or an entry's own name, in the display type and the text colour, beside its
    /// picture (Party's sheet).
    Name(&'a str),
}

impl Heading<'_> {
    /// The space between the item above and the heading's top. It is always more than the space
    /// under the heading, so a heading belongs to what follows it.
    pub const fn above(self) -> f32 {
        match self {
            Self::Group { .. } | Self::Label(_) | Self::Panel(_) | Self::Status(_) => L,
            Self::Section(_) | Self::Name(_) => XL,
        }
    }

    /// The space between the heading (its hairline, for a section) and what follows it.
    pub const fn below(self) -> f32 {
        match self {
            Self::Group { .. } | Self::Label(_) | Self::Panel(_) | Self::Status(_) => XS,
            Self::Section(_) | Self::Name(_) => XS,
        }
    }
}

/// A heading, with its own space above it: `Heading::above` from the item before it, none when it
/// is the first thing in its column. The space under it is `Heading::below`. In a row (a
/// horizontal layout) it adds no space at all: a column's space would become space at its side
/// there; the row's owner gives the space above the row. A caller adds no
/// space before or after a heading (Clippy refuses a raw `add_space`; a `gap` there is a fault a
/// review sees).
///
/// ```
/// fn f(ui: &mut egui::Ui) {
///     splat_design::extras::heading(ui, splat_design::extras::Heading::Group { title: "Ready", count: 2 });
/// }
/// ```
///
/// The space is the heading's, not the caller's:
///
/// ```compile_fail,E0061
/// fn f(ui: &mut egui::Ui) {
///     splat_design::extras::heading(ui, splat_design::extras::Heading::Label("Party"), 12.0);
/// }
/// ```
pub fn heading(ui: &mut Ui, heading: Heading<'_>) -> Response {
    let first = ui.cursor().top() <= ui.max_rect().top() + 0.5;
    let in_row = ui.layout().main_dir().is_horizontal();
    // egui puts the item gap after every item; the heading's own space replaces it.
    let spacing = ui.spacing().item_spacing.y;
    if !first && !in_row {
        space(ui, heading.above() - spacing);
    }
    let response = match heading {
        Heading::Group { title, count } => {
            let job = group_heading_text(ui, title, count);
            ui.label(job)
        }
        Heading::Label(text) => ui.label(
            Type::Label
                .rich(ui, text.to_uppercase())
                .color(colors().muted),
        ),
        Heading::Section(text) => {
            let response =
                ui.add(egui::Label::new(Type::Section.rich(ui, text).color(colors().text)).wrap());
            space(ui, 2.0);
            rule(ui);
            response
        }
        Heading::Panel(text) => ui.label(Type::Heading.rich(ui, text).color(colors().text)),
        Heading::Status(status) => {
            let response = ui.label(
                Type::Heading
                    .rich(ui, format!("{} {}", status.icon, status.word))
                    .color(status.color),
            );
            named(ui, &response, status.word);
            response
        }
        Heading::Name(text) => ui.label(Type::Display.rich(ui, text).color(colors().text)),
    };
    if !in_row {
        space(ui, heading.below() - spacing);
    }
    response
}

// ------------------------------------------------------------------------------------------------
// Choices (N1)

/// One choice in a menu, a drop-down or a list, of which one is the selected one (N1).
/// Clippy refuses egui's other ways to draw one outside `src/design/`: `Ui::selectable_value`,
/// `Ui::selectable_label`, `Button::selectable`, `Button::selected` and `SelectableLabel::new`
/// (staged, `Ban::Extras`). Selected states that are NOT this part, each with its own part: a
/// radio (one of a few shown at once), a checkbox (on or off), the toggle (`Action::selected`, a
/// button that stays pressed), and tabs (`segmented`, `counted_tab`, `overflow_tabs`). The selected
/// choice shows the check (`icons::OK`) in a fixed column at its left, so every choice's words
/// start at one x whether or not it is selected, and its accessible state is selected.
/// `clicked()` chooses it.
///
/// **Width (X2).** A choice never takes more width than its column gives. In a column it takes
/// the column's width, and its words are cut with an ellipsis when they do not fit. On a line it
/// takes its words' width, at most the rest of the line. In a menu from `menu_popup`, the
/// menu decides its width on every draw from its choices' words, at most its room in the pane,
/// and each choice takes that width. In a raw egui popup (staged), egui measures the menu only
/// on its first frame (there the choice asks for its words' width, at most egui's room) and then
/// keeps that size; a choice then fills it.
///
/// **Words (X2).** The words are the app's own fixed words (`&'static str`). Words from the
/// user's data (a name, a path, a title) can be any length, so they go through
/// [`choice_wrapped`], which wraps them inside the row and never cuts them.
///
/// ```
/// fn f(ui: &mut egui::Ui, scope: &mut u8) {
///     if splat_design::extras::choice(ui, *scope == 1, "This campaign").clicked() {
///         *scope = 1;
///     }
/// }
/// ```
///
/// Whether it is selected is said, never left out:
///
/// ```compile_fail,E0061
/// fn f(ui: &mut egui::Ui) {
///     let _ = splat_design::extras::choice(ui, "This campaign");
/// }
/// ```
///
/// A name from the user's data is not the app's fixed words; it goes through `choice_wrapped`:
///
/// ```compile_fail,E0308
/// fn f(ui: &mut egui::Ui, name: String) {
///     let _ = splat_design::extras::choice(ui, true, name);
/// }
/// ```
///
/// ```compile_fail,E0621
/// fn f(ui: &mut egui::Ui, name: &str) {
///     let _ = splat_design::extras::choice(ui, true, name);
/// }
/// ```
///
/// The compiling twin for data words:
///
/// ```
/// fn f(ui: &mut egui::Ui, name: &str) {
///     let _ = splat_design::extras::choice_wrapped(ui, true, name, None);
/// }
/// ```
pub fn choice(ui: &mut Ui, selected: bool, label: &'static str) -> Response {
    choice_row(ui, selected, label, Lines::One)
}

/// The width a choice row takes: never more than its column or line gives (X2). `natural` is
/// the row's width with its words on one line.
fn choice_width(ui: &Ui, natural: f32) -> f32 {
    super::layout::note_choice_width(ui, natural);
    let room = ui.available_width().max(0.0);
    if ui.layout().is_vertical() && !ui.is_sizing_pass() {
        // A column: its width, whatever the words need.
        room
    } else {
        // A line, or a menu that egui is measuring: the words' width, inside the room.
        natural.min(room)
    }
}

/// How a choice row lays out its words.
#[derive(Clone, Copy)]
enum Lines<'a> {
    /// One line, cut with an ellipsis; the row is as wide as its words in a sideways layout.
    One,
    /// Wrapped inside the row's full width, with a blocked choice's reason below.
    Wrap { blocked: Option<&'a str> },
}

/// The one choice row behind [`choice`] and [`choice_wrapped`]: the check column, the fill on
/// hover and press, the words and the focus ring.
fn choice_row(ui: &mut Ui, selected: bool, label: &str, lines: Lines<'_>) -> Response {
    let c = colors();
    let font = Type::Body.font(ui);
    let blocked = match lines {
        Lines::One => None,
        Lines::Wrap { blocked } => blocked,
    };
    let color = if blocked.is_some() { c.muted } else { c.text };
    let words = number_width(ui, label, &font).max(blocked.map_or(0.0, |reason| {
        number_width(ui, reason, &Type::Small.font(ui))
    }));
    let width = choice_width(ui, S + ICON_SLOT + words + S);
    let height = match lines {
        Lines::One => Density::Read.control(),
        Lines::Wrap { blocked } => choice_wrapped_height(ui, label, blocked, width),
    };
    let sense = if blocked.is_some() {
        Sense::hover()
    } else {
        Sense::click()
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), sense);
    response.widget_info(|| {
        WidgetInfo::selected(
            WidgetType::SelectableLabel,
            blocked.is_none() && ui.is_enabled(),
            selected,
            label,
        )
    });
    if let Some(reason) = blocked {
        ui.ctx()
            .accesskit_node_builder(response.id, |node| node.set_description(reason));
    }
    if ui.is_rect_visible(rect) {
        let fill = if blocked.is_some() {
            None
        } else if response.is_pointer_button_down_on() {
            Some(c.selected)
        } else if response.hovered() {
            Some(c.hover)
        } else {
            None
        };
        if let Some(fill) = fill {
            ui.painter()
                .rect_filled(rect, egui::CornerRadius::same(RADIUS_CONTROL as u8), fill);
        }
        let left = rect.left() + S + ICON_SLOT;
        // The first line's middle: the row's middle for one line, else `row_pad` below its top.
        let (first, words, reason) = match lines {
            Lines::One => {
                let galley = one_line(ui, label, font, c.text, rect.width() - 2.0 * S - ICON_SLOT);
                (rect.center().y, galley, None)
            }
            Lines::Wrap { blocked } => {
                let (words, reason) = row_galleys(ui, label, blocked, width, color);
                (
                    rect.top() + row_pad() + Type::Body.line() / 2.0,
                    words,
                    reason,
                )
            }
        };
        if selected {
            ui.painter().text(
                pos2(rect.left() + S, first),
                Align2::LEFT_CENTER,
                icons::OK,
                Type::Body.font(ui),
                c.accent,
            );
        }
        let top = match lines {
            Lines::One => first - words.size().y / 2.0,
            Lines::Wrap { .. } => rect.top() + row_pad(),
        };
        let below = words.size().y;
        ui.painter().galley(pos2(left, top), words, color);
        if let Some(reason) = reason {
            ui.painter()
                .galley(pos2(left, top + below), reason, c.muted);
        }
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response
}

/// [`choice`] for a value: a click sets `current` to `value`. Its words are the app's fixed
/// words, as [`choice`]'s are; a value named by the user's data uses [`choice_wrapped`].
///
/// ```compile_fail,E0308
/// fn f(ui: &mut egui::Ui, at: &mut u8, name: String) {
///     let _ = splat_design::extras::choice_value(ui, at, 1, name);
/// }
/// ```
///
/// ```
/// fn f(ui: &mut egui::Ui, at: &mut u8) {
///     let _ = splat_design::extras::choice_value(ui, at, 1, "First");
/// }
/// ```
pub fn choice_value<T: PartialEq>(
    ui: &mut Ui,
    current: &mut T,
    value: T,
    label: &'static str,
) -> Response {
    let mut response = choice(ui, *current == value, label);
    if response.clicked() && *current != value {
        *current = value;
        response.mark_changed();
    }
    response
}

/// The space above and below a row's words, so a one-line row is a `Read` control.
fn row_pad() -> f32 {
    ((Density::Read.control() - Type::Body.line()) / 2.0).max(XS)
}

/// A row's words, wrapped inside `width`: the label in `Body`, and a blocked choice's reason in
/// `Small` below it.
fn row_galleys(
    ui: &Ui,
    label: &str,
    reason: Option<&str>,
    width: f32,
    color: egui::Color32,
) -> (Arc<egui::Galley>, Option<Arc<egui::Galley>>) {
    let c = colors();
    let room = (width - 2.0 * S - ICON_SLOT).max(1.0);
    let wrap = |words: &str, font: egui::FontId, color| {
        // Egui's fallback splits an overwide word. Shorten only such words first, so the
        // ordinary wrapping path can break at word boundaries and fitting text stays exact.
        let mut fitted = None;
        let mut start = 0;
        let mut copied = 0;
        for token in words.split_inclusive(char::is_whitespace) {
            let word = token.trim_end_matches(char::is_whitespace);
            if number_width(ui, word, &font) > room {
                let short = one_line(ui, word, font.clone(), color, room);
                let short: String = short
                    .rows
                    .iter()
                    .flat_map(|row| row.glyphs.iter().map(|glyph| glyph.chr))
                    .collect();
                let value = fitted.get_or_insert_with(String::new);
                value.push_str(&words[copied..start]);
                value.push_str(&short);
                copied = start + word.len();
            }
            start += token.len();
        }
        if let Some(value) = &mut fitted {
            value.push_str(&words[copied..]);
        }
        ui.fonts_mut(|fonts| {
            fonts.layout(
                fitted.unwrap_or_else(|| words.to_owned()),
                font,
                color,
                room,
            )
        })
    };
    (
        wrap(label, Type::Body.font(ui), color),
        reason.map(|reason| wrap(reason, Type::Small.font(ui), c.muted)),
    )
}

/// A [`choice_wrapped`] row's height in `width`: a `Read` control, or its wrapped words and pad.
pub(super) fn choice_wrapped_height(ui: &Ui, label: &str, reason: Option<&str>, width: f32) -> f32 {
    let (label, reason) = row_galleys(ui, label, reason, width, colors().text);
    let words = label.size().y + reason.map_or(0.0, |reason| reason.size().y);
    (words + 2.0 * row_pad()).max(Density::Read.control())
}

/// [`choice`] in a list whose rows wrap (the drop-down's, `Dropdown`): the same check column and
/// look, its words wrapped at word boundaries inside the row (an overwide word ends in an ellipsis), so two names that differ at their end
/// stay apart; a blocked choice shows its reason below its words and takes no press and no
/// keyboard focus. This is the form for words from the user's data (X2): it takes `&str` of any
/// length. Its width follows [`choice`]'s rule; only its words wrap instead of being cut.
pub fn choice_wrapped(ui: &mut Ui, selected: bool, label: &str, blocked: Option<&str>) -> Response {
    choice_row(ui, selected, label, Lines::Wrap { blocked })
}

/// [`choice_wrapped`] for a value named by the user's data: a click sets `current` to `value`,
/// as [`choice_value`] does.
pub fn choice_wrapped_value<T: PartialEq>(
    ui: &mut Ui,
    current: &mut T,
    value: T,
    label: &str,
) -> Response {
    let mut response = choice_wrapped(ui, *current == value, label, None);
    if response.clicked() && *current != value {
        *current = value;
        response.mark_changed();
    }
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::Context;

    fn run(add: impl FnMut(&mut Ui)) {
        let ctx = Context::default();
        let mut add = add;
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| add(ui));
        });
    }

    /// `indent`: the block starts `levels` steps in with no item spacing added; `dropped`: it starts
    /// `drop` below the slot's top and the slot holds the drop; `unless_empty`: an empty block leaves
    /// the next item where it would be with no block, a block that draws leaves its gap;
    /// `at_least` holds a short block's line to its height; `empty_line` is a label's line.
    #[test]
    fn indent_dropped_and_unless_empty_place_by_rect() {
        run(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 6.0);
            let left = ui.max_rect().left();
            let at = indent(ui, 3, Gap::S, |ui| ui.label("x").rect.left()).inner;
            assert!((at - (left + 3.0 * S)).abs() < 0.01, "indent: {at}");
            let top = ui.cursor().top();
            let seen = dropped(ui, egui::vec2(100.0, 40.0), 10.0, |ui| {
                ui.label("y").rect.top()
            });
            assert!((seen - (top + 10.0)).abs() < 0.01, "drop: {seen}");
            assert!(
                ui.cursor().top() >= top + 10.0,
                "the slot holds the drop: {} from {top}",
                ui.cursor().top()
            );
            let before = ui.cursor().top();
            let ((), drew) = unless_empty(ui, |_| ());
            assert!(
                !drew && (ui.cursor().top() - before).abs() < 0.01,
                "empty block moved the cursor"
            );
            let (_, drew) = unless_empty(ui, |ui| ui.label("z"));
            assert!(
                drew && ui.cursor().top() > before,
                "a block that draws has its gap"
            );
            // `at_least` (layout step 11c): a short block, then room, so the next item starts
            // `height` below the block's top; a block taller than `height` adds none.
            let top = ui.cursor().top();
            let _ = at_least(ui, 60.0, |ui| ui.label("short"));
            let held = ui.cursor().top() - top;
            assert!(
                (held - 60.0).abs() < 0.01,
                "at_least: room up to 60, got {held}"
            );
            let top = ui.cursor().top();
            let _ = at_least(ui, 5.0, |ui| ui.label("taller than five"));
            let tall = ui.cursor().top() - top;
            let top = ui.cursor().top();
            let _ = ui.label("taller than five");
            let alone = ui.cursor().top() - top;
            assert!(
                (tall - alone).abs() < 0.01,
                "at_least: a taller block adds nothing: {tall} vs the block alone {alone}"
            );
            // `empty_line`: as tall as a label of its type, with the label's gap after it.
            let top = ui.cursor().top();
            empty_line(ui, super::super::Type::Small);
            let empty = ui.cursor().top() - top;
            let top = ui.cursor().top();
            let _ = ui.label(super::super::Type::Small.rich(ui, "After"));
            let label = ui.cursor().top() - top;
            assert!(
                empty > 0.0 && (empty - label).abs() < 0.01,
                "empty_line: {empty}, a Small label: {label}"
            );
        });
    }

    /// `gap_to_next`: the whole distance between two items is one step, at the default item
    /// spacing and at another one, down a column and along a line. `gap` leaves a step plus the
    /// spacing.
    #[test]
    fn gap_to_next_leaves_exactly_one_step_whatever_the_spacing() {
        for spacing in [6.0_f32, 8.0] {
            for step in [Gap::S, Gap::L] {
                for horizontal in [false, true] {
                    let mut measured = None;
                    run(|ui| {
                        ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);
                        let mut between = |ui: &mut Ui| {
                            let first = ui.label("first").rect;
                            gap_to_next(ui, step);
                            let second = ui.label("second").rect;
                            measured = Some(if horizontal {
                                second.left() - first.right()
                            } else {
                                second.top() - first.bottom()
                            });
                        };
                        if horizontal {
                            ui.horizontal(|ui| between(ui));
                        } else {
                            between(ui);
                        }
                    });
                    let distance = measured.expect("the labels were laid out");
                    assert!(
                        (distance - step.size()).abs() < 0.01,
                        "{step:?} at spacing {spacing}, horizontal {horizontal}: {distance}"
                    );
                }
            }
        }
    }

    /// N3: a heading adds its own space above it (none when it is first in its column), and the
    /// space above is more than the space below.
    #[test]
    fn a_heading_adds_the_space_above_it_and_less_below() {
        for kind in [
            Heading::Group {
                title: "Ready",
                count: 2,
            },
            Heading::Label("Party"),
            Heading::Section("Actions"),
            Heading::Panel("Short rest"),
            Heading::Status(Status {
                icon: icons::WARN,
                word: "Hit points conflict",
                color: colors().sync.conflict,
            }),
            Heading::Name("Kaya Brightwater"),
        ] {
            run(|ui| {
                let top = ui.max_rect().top();
                let first = heading(ui, kind);
                assert!(
                    (first.rect.top() - top).abs() < 0.5,
                    "{kind:?}: the first heading starts at its column's top"
                );
                let before = ui.label("A row");
                let next = heading(ui, kind);
                let after = ui.label("The next row");
                let above = next.rect.top() - before.rect.bottom();
                assert!(
                    (above - kind.above()).abs() < 0.5,
                    "{kind:?}: {above} above, not {}",
                    kind.above()
                );
                let below = after.rect.top() - next.rect.bottom();
                assert!(above > below, "{kind:?}: {above} above, {below} below");
                // In a row the heading adds no space at its sides (row 93): it starts where
                // the row starts, and the next item is one item gap after it. (A serif
                // `Section` heading has its own rule and is never drawn in a row.)
                if matches!(kind, Heading::Section(_)) {
                    return;
                }
                let gap = ui.spacing().item_spacing.x;
                ui.label("A row");
                let row = ui.horizontal(|ui| {
                    let start = ui.cursor().left();
                    let heading = heading(ui, kind).rect;
                    let status = ui.label("Connected").rect;
                    (start, heading, status)
                });
                let (start, heading, status) = row.inner;
                assert!(
                    (heading.left() - start).abs() < 0.5
                        && (status.left() - heading.right() - gap).abs() < 0.5,
                    "{kind:?}: in a row, starts at {} (row {start}), next item {} after it",
                    heading.left(),
                    status.left() - heading.right()
                );
            });
        }
        // M0413, M0041, M0042: each new role's type and colour; a status heading paints its
        // icon first in its colour and is named by its word.
        let ctx = Context::default();
        crate::design::theme::install(&ctx);
        ctx.enable_accesskit();
        let conflict = colors().sync.conflict;
        let output = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                heading(ui, Heading::Panel("Short rest"));
                heading(
                    ui,
                    Heading::Status(Status {
                        icon: icons::WARN,
                        word: "Hit points conflict",
                        color: conflict,
                    }),
                );
                heading(ui, Heading::Name("Kaya Brightwater"));
            });
        });
        let painted: Vec<(String, f32, egui::Color32)> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some((
                    text.galley.text().to_owned(),
                    text.galley.job.sections[0].format.font_id.size,
                    text.galley.job.sections[0].format.color,
                )),
                _ => None,
            })
            .collect();
        let find = |words: &str| {
            painted
                .iter()
                .find(|(text, ..)| text == words)
                .cloned()
                .unwrap_or_else(|| panic!("{words:?} painted: {painted:?}"))
        };
        let status_words = format!("{} Hit points conflict", icons::WARN);
        assert_eq!(
            [
                find("Short rest"),
                find(&status_words),
                find("Kaya Brightwater")
            ]
            .map(|(_, size, color)| (size, color)),
            [
                (Type::Heading.size(), colors().text),
                (Type::Heading.size(), conflict),
                (Type::Display.size(), colors().text),
            ],
            "m-p0-heading-roles: Panel, Status and Name in their type and colour"
        );
        let named = output
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| {
                update
                    .nodes
                    .iter()
                    .any(|(_, node)| node.label() == Some("Hit points conflict"))
            });
        assert!(
            named,
            "m-p0-heading-roles: the status heading is named by its word"
        );
    }

    /// X2: a choice never takes more width than its column gives. In a 100 px column, a label
    /// that needs about 300 px keeps each form's row, and every word it paints, inside the
    /// column: the one-line form cuts its words, the wrapped form wraps them. On a line in that
    /// column, the row ends at the column's edge too. A short label in a wide column still takes
    /// the column's width (a menu's rows stay one width).
    #[test]
    fn a_choice_stays_inside_a_narrow_column() {
        const LONG: &str = "A choice whose words need far more room than its narrow column has";
        let ctx = Context::default();
        let mut column = egui::Rect::NOTHING;
        let mut rows = Vec::new();
        let output = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let words = number_width(ui, LONG, &Type::Body.font(ui));
                assert!(words >= 300.0, "the label needs {words} px");
                column = egui::Rect::from_min_size(ui.cursor().min, vec2(100.0, 400.0));
                ui.scope_builder(egui::UiBuilder::new().max_rect(column), |ui| {
                    rows.push(choice(ui, true, LONG).rect);
                    rows.push(choice_wrapped(ui, false, LONG, None).rect);
                    rows.push(choice_wrapped(ui, false, LONG, Some(LONG)).rect);
                    rows.push(ui.horizontal(|ui| choice(ui, false, LONG)).inner.rect);
                    let short = choice(ui, false, "All");
                    assert_eq!(short.rect.width(), 100.0, "a short choice fills its column");
                });
            });
        });
        assert_eq!(rows.len(), 4);
        for (at, row) in rows.iter().enumerate() {
            assert!(
                row.left() >= column.left() - 0.5 && row.right() <= column.right() + 0.5,
                "row {at} {row:?} is inside the column {column:?}"
            );
        }
        let mut words = Vec::new();
        for clipped in &output.shapes {
            if let egui::Shape::Text(text) = &clipped.shape {
                words.push(text.visual_bounding_rect());
            }
        }
        assert!(
            words.len() >= 5,
            "the rows painted their words: {}",
            words.len()
        );
        for word in words {
            assert!(
                word.right() <= column.right() + 0.5,
                "words {word:?} stay inside the column {column:?}"
            );
        }
    }

    /// N1: the selected choice's words start where an unselected choice's words start, and only
    /// the selected one says it is selected.
    #[test]
    fn a_choice_keeps_its_words_at_one_x_and_says_it_is_selected() {
        run(|ui| {
            let on = choice(ui, true, "This campaign");
            let off = choice(ui, false, "Built in");
            assert_eq!(on.rect.left(), off.rect.left());
            assert_eq!(on.rect.width(), off.rect.width(), "a column's width");
            assert!(on.rect.height() >= 24.0);
            let mut value = 0;
            let response = choice_value(ui, &mut value, 1, "One");
            assert!(!response.changed());
        });
        let ctx = Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = choice(ui, true, "This campaign");
                let _ = choice(ui, false, "Built in");
            });
        });
        let output = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = choice(ui, true, "This campaign");
                let _ = choice(ui, false, "Built in");
            });
        });
        let checks = output
            .shapes
            .iter()
            .filter(|clipped| {
                matches!(&clipped.shape, egui::Shape::Text(text) if text.galley.job.text == icons::OK)
            })
            .count();
        assert_eq!(checks, 1, "one check, on the selected choice");

        // Changing the selection moves no word and keeps the menu's width. On a line each choice
        // takes its own width (a menu that sizes itself to its choices measures the same widths),
        // so each word's x and the line's width are the same whichever choice is selected.
        let menu = |selected: usize| {
            let ctx = Context::default();
            let mut width = 0.0;
            let mut output = None;
            for _ in 0..2 {
                let out = ctx.run(egui::RawInput::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.horizontal_top(|ui| {
                            let column = ui.horizontal(|ui| {
                                for (at, word) in
                                    ["This campaign", "Built in", "All"].iter().enumerate()
                                {
                                    let _ = choice(ui, at == selected, word);
                                }
                            });
                            width = column.response.rect.width();
                        });
                    });
                });
                output = Some(out);
            }
            let words: Vec<f32> = output
                .unwrap()
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) if text.galley.job.text != icons::OK => {
                        Some(text.pos.x)
                    }
                    _ => None,
                })
                .collect();
            (words, width)
        };
        let first = menu(0);
        assert_eq!(first.0.len(), 3, "three words painted");
        for selected in [1, 2] {
            assert_eq!(
                menu(selected),
                first,
                "n1-still: selection {selected} moved a word or the menu"
            );
        }
    }
}
