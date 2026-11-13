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
/// start at one x whether or not it is selected, and its accessible state is selected. In a
/// column it takes the column's width; on a line, its own. `clicked()` chooses it.
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
pub fn choice(ui: &mut Ui, selected: bool, label: &str) -> Response {
    let c = colors();
    let font = Type::Body.font(ui);
    let natural = S + ICON_SLOT + number_width(ui, label, &font) + S;
    let width = if ui.layout().is_vertical() {
        natural.max(ui.available_width())
    } else {
        natural
    };
    let height = Density::Read.control();
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());
    response.widget_info(|| {
        WidgetInfo::selected(
            WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        )
    });
    if ui.is_rect_visible(rect) {
        let fill = if response.is_pointer_button_down_on() {
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
        if selected {
            ui.painter().text(
                pos2(rect.left() + S, rect.center().y),
                Align2::LEFT_CENTER,
                icons::OK,
                Type::Body.font(ui),
                c.accent,
            );
        }
        let galley = one_line(ui, label, font, c.text, rect.width() - 2.0 * S - ICON_SLOT);
        ui.painter().galley(
            pos2(
                rect.left() + S + ICON_SLOT,
                rect.center().y - galley.size().y / 2.0,
            ),
            galley,
            c.text,
        );
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response
}

/// [`choice`] for a value: a click sets `current` to `value`.
pub fn choice_value<T: PartialEq>(ui: &mut Ui, current: &mut T, value: T, label: &str) -> Response {
    let mut response = choice(ui, *current == value, label);
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

        // Changing the selection moves no word and keeps the menu's width: in a menu that takes
        // its width from its choices (a shrinking area), each word's x and the menu's width are
        // the same whichever choice is selected.
        let menu = |selected: usize| {
            let ctx = Context::default();
            let mut width = 0.0;
            let mut output = None;
            for _ in 0..2 {
                let out = ctx.run(egui::RawInput::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.horizontal_top(|ui| {
                            let column = ui.vertical(|ui| {
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
