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
use eframe::egui::{self, Align2, Response, Sense, Ui, WidgetInfo, WidgetType, pos2, vec2};
use std::sync::Arc;

// ------------------------------------------------------------------------------------------------
// Space (N3)

/// A gap from the spacing scale. No other amount exists, so a caller cannot nudge a gap by a few
/// pixels. The space above a heading and after a field's help are not gaps: the heading and the
/// field add them.
///
/// ```
/// fn f(ui: &mut eframe::egui::Ui) {
///     splat::extras::gap(ui, splat::extras::Gap::L);
/// }
/// ```
///
/// A raw number is not a gap:
///
/// ```compile_fail,E0308
/// fn f(ui: &mut eframe::egui::Ui) {
///     splat::extras::gap(ui, 10.0);
/// }
/// ```
///
/// The design's own raw space is not public:
///
/// ```compile_fail,E0603
/// fn f(ui: &mut eframe::egui::Ui) {
///     splat::extras::space(ui, 10.0);
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

/// The design's own space: any amount, for the parts that measure it. Only `src/design/` may use
/// it, so a caller's space is always a [`Gap`].
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub(in crate::design) fn space(ui: &mut Ui, amount: f32) {
    ui.add_space(amount);
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
}

impl Heading<'_> {
    /// The space between the item above and the heading's top. It is always more than the space
    /// under the heading, so a heading belongs to what follows it.
    pub const fn above(self) -> f32 {
        match self {
            Self::Group { .. } | Self::Label(_) => L,
            Self::Section(_) => XL,
        }
    }

    /// The space between the heading (its hairline, for a section) and what follows it.
    pub const fn below(self) -> f32 {
        match self {
            Self::Group { .. } | Self::Label(_) => XS,
            Self::Section(_) => XS,
        }
    }
}

/// A heading, with its own space above it: `Heading::above` from the item before it, none when it
/// is the first thing in its column. The space under it is `Heading::below`. A caller adds no
/// space before or after a heading (Clippy refuses a raw `add_space`; a `gap` there is a fault a
/// review sees).
///
/// ```
/// fn f(ui: &mut eframe::egui::Ui) {
///     splat::extras::heading(ui, splat::extras::Heading::Group { title: "Ready", count: 2 });
/// }
/// ```
///
/// The space is the heading's, not the caller's:
///
/// ```compile_fail,E0061
/// fn f(ui: &mut eframe::egui::Ui) {
///     splat::extras::heading(ui, splat::extras::Heading::Label("Party"), 12.0);
/// }
/// ```
pub fn heading(ui: &mut Ui, heading: Heading<'_>) -> Response {
    let first = ui.cursor().top() <= ui.max_rect().top() + 0.5;
    // egui puts the item gap after every item; the heading's own space replaces it.
    let spacing = ui.spacing().item_spacing.y;
    if !first {
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
    };
    space(ui, heading.below() - spacing);
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
/// takes its words' width, at most the rest of the line. In an auto-sized menu, egui first
/// measures the menu (a sizing pass, inside the room egui gives it: a `ComboBox`'s or a
/// `Popup::width`'s width, else egui's default menu room, at most the screen; `menu_popup` gives
/// the room to its pane's edge): there the choice asks for its words' width, at most that room,
/// so the menu is as wide as its widest choice and never wider than its room. Then each choice
/// takes the menu's width.
///
/// **Words (X2).** The words are the app's own fixed words (`&'static str`). Words from the
/// user's data (a name, a path, a title) can be any length, so they go through
/// [`choice_wrapped`], which wraps them inside the row and never cuts them.
///
/// ```
/// fn f(ui: &mut eframe::egui::Ui, scope: &mut u8) {
///     if splat::extras::choice(ui, *scope == 1, "This campaign").clicked() {
///         *scope = 1;
///     }
/// }
/// ```
///
/// Whether it is selected is said, never left out:
///
/// ```compile_fail,E0061
/// fn f(ui: &mut eframe::egui::Ui) {
///     let _ = splat::extras::choice(ui, "This campaign");
/// }
/// ```
///
/// A name from the user's data is not the app's fixed words; it goes through `choice_wrapped`:
///
/// ```compile_fail,E0308
/// fn f(ui: &mut eframe::egui::Ui, name: String) {
///     let _ = splat::extras::choice(ui, true, name);
/// }
/// ```
///
/// ```compile_fail,E0621
/// fn f(ui: &mut eframe::egui::Ui, name: &str) {
///     let _ = splat::extras::choice(ui, true, name);
/// }
/// ```
///
/// The compiling twin for data words:
///
/// ```
/// fn f(ui: &mut eframe::egui::Ui, name: &str) {
///     let _ = splat::extras::choice_wrapped(ui, true, name, None);
/// }
/// ```
pub fn choice(ui: &mut Ui, selected: bool, label: &'static str) -> Response {
    choice_row(ui, selected, label, Lines::One)
}

/// The width a choice row takes: never more than its column or line gives (X2). `natural` is
/// the row's width with its words on one line.
fn choice_width(ui: &Ui, natural: f32) -> f32 {
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
    let words = number_width(ui, label, &font).max(
        blocked.map_or(0.0, |reason| number_width(ui, reason, &Type::Small.font(ui))),
    );
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
/// fn f(ui: &mut eframe::egui::Ui, at: &mut u8, name: String) {
///     let _ = splat::extras::choice_value(ui, at, 1, name);
/// }
/// ```
///
/// ```
/// fn f(ui: &mut eframe::egui::Ui, at: &mut u8) {
///     let _ = splat::extras::choice_value(ui, at, 1, "First");
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
        ui.fonts_mut(|fonts| fonts.layout(words.to_owned(), font, color, room))
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
/// look, its words wrapped inside the row (never cut), so two names that differ at their end
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
    use eframe::egui::Context;

    fn run(add: impl FnMut(&mut Ui)) {
        let ctx = Context::default();
        let mut add = add;
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| add(ui));
        });
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
            });
        }
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
        assert!(words.len() >= 5, "the rows painted their words: {}", words.len());
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
