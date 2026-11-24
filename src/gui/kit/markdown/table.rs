//! A reader table (egui `markdown_view::reader_table`): columns as wide as their words without
//! breaking a word, shrunk toward their longest word when the reader is narrower, a rule under
//! each row and a heavier look for the header, no stripes. A table that still does not fit says
//! so and scrolls sideways under its fixed first column, whose rule then runs through the header
//! and the rows. A rollable table keeps a slot for the dice mark before every row's range.
use super::parse::Table;
use crate::gui::icons;
use crate::gui::theme::{HAIRLINE_WIDTH, M, SCROLL_BAR_WIDTH, Type, TypeExt, XS, colors};
use gpui::{
    AnyElement, Div, ElementId, IntoElement, ParentElement, Pixels, SharedString, Styled, TextRun,
    Window, div, prelude::*, px,
};

/// The space between columns.
pub(crate) const TABLE_GAP: Pixels = M;

/// The hint over a table that scrolls sideways.
pub(crate) const MORE_COLUMNS: &str = "More columns: scroll sideways or use Shift+wheel.";

/// A table laid out for a width.
pub(crate) struct Laid {
    pub widths: Vec<Pixels>,
    pub heights: Vec<Pixels>,
    pub scrolls: bool,
    /// The dice mark's slot before the first column's words, for a rollable table.
    pub slot: Pixels,
}

/// One run of `role` over all of `text`.
fn run(text: &str, role: Type, family: &'static str) -> Vec<TextRun> {
    let mut font = gpui::font(family);
    font.weight = role.weight();
    vec![TextRun {
        len: text.len(),
        font,
        color: colors().text,
        background_color: None,
        underline: None,
        strikethrough: None,
    }]
}

/// The width `text` takes on one line in `role`.
fn line_width(text: &str, role: Type, window: &Window) -> Pixels {
    if text.is_empty() {
        return px(0.0);
    }
    window
        .text_system()
        .shape_line(
            SharedString::from(text.to_owned()),
            role.size(),
            &run(text, role, role.family()),
            None,
        )
        .width
        .ceil()
}

/// How many lines `text` takes in `role`, wrapped at `width`.
fn lines(text: &str, role: Type, width: Pixels, window: &Window) -> usize {
    if text.is_empty() {
        return 1;
    }
    window
        .text_system()
        .shape_text(
            SharedString::from(text.to_owned()),
            role.size(),
            &run(text, role, role.family()),
            Some(width.max(px(1.0))),
            None,
        )
        .map_or(1, |shaped| {
            shaped
                .iter()
                .map(|line| line.wrap_boundaries().len() + 1)
                .sum::<usize>()
                .max(1)
        })
}

/// Fit columns to their words without breaking a word; a table that cannot fit those widths
/// scrolls (egui `column_widths`).
pub(crate) fn lay_out(table: &Table, available: Pixels, window: &Window) -> Laid {
    let count = table.columns.len();
    if count == 0 || table.rows.is_empty() {
        return Laid {
            widths: Vec::new(),
            heights: Vec::new(),
            scrolls: false,
            slot: px(0.0),
        };
    }
    let measure = |text: &str| line_width(text, Type::StatLabel, window) + XS;
    let mut natural = vec![px(1.0); count];
    let mut floor = vec![px(1.0); count];
    for row in 0..table.rows.len() {
        for (position, (natural, floor)) in natural.iter_mut().zip(&mut floor).enumerate() {
            let cell = table.cell(row, position);
            *natural = (*natural).max(measure(cell));
            if row == 0 {
                *floor = (*floor).max(measure(cell));
            }
            for word in cell.split_whitespace() {
                *floor = (*floor).max(measure(word));
            }
        }
    }
    let gaps = TABLE_GAP * count.saturating_sub(1) as f32;
    let room = (available - gaps - XS).max(px(1.0));
    let total: Pixels = natural.iter().copied().fold(px(0.0), |a, b| a + b);
    let floors: Pixels = floor.iter().copied().fold(px(0.0), |a, b| a + b);
    let mut widths = if total <= room {
        natural
    } else if floors >= room {
        floor
    } else {
        let share = (room - floors) / (total - floors).max(px(1.0));
        natural
            .iter()
            .zip(&floor)
            .map(|(natural, floor)| *floor + (*natural - *floor) * share)
            .collect()
    };
    // A rollable table's first column holds the dice mark's slot before its words.
    let slot = if table.rolls.is_some() {
        let mut font = gpui::font(icons::FAMILY);
        font.weight = gpui::FontWeight::NORMAL;
        let glyph = window
            .text_system()
            .shape_line(
                SharedString::new_static(icons::DICE),
                Type::Stat.size(),
                &[TextRun {
                    len: icons::DICE.len(),
                    font,
                    color: colors().text,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
                None,
            )
            .width;
        (glyph + line_width(" ", Type::Stat, window).max(px(3.0))).ceil()
    } else {
        px(0.0)
    };
    if slot > px(0.0) {
        for row in 0..table.rows.len() {
            let role = if row == 0 {
                Type::StatLabel
            } else {
                Type::Stat
            };
            widths[0] = widths[0].max(line_width(table.cell(row, 0), role, window) + slot + XS);
        }
    }
    let heights = (0..table.rows.len())
        .map(|row| {
            let role = if row == 0 {
                Type::StatLabel
            } else {
                Type::Stat
            };
            let tallest = (0..count)
                .map(|position| {
                    let width = if position == 0 {
                        widths[0] - slot
                    } else {
                        widths[position]
                    };
                    lines(table.cell(row, position), role, width, window)
                })
                .max()
                .unwrap_or(1);
            role.line() * tallest as f32 + XS * 2.0 + HAIRLINE_WIDTH
        })
        .collect();
    let width = widths.iter().copied().fold(px(0.0), |a, b| a + b) + gaps;
    Laid {
        widths,
        heights,
        scrolls: width > available,
        slot,
    }
}

/// The table, drawn from its layout. `matched` is the rolled row (0 is the header).
pub(crate) fn draw(
    id: ElementId,
    table: &Table,
    laid: &Laid,
    matched: Option<usize>,
) -> AnyElement {
    let c = colors();
    if laid.widths.is_empty() {
        return div().into_any_element();
    }
    let cell = |row: usize, position: usize| -> Div {
        let role = if row == 0 {
            Type::StatLabel
        } else {
            Type::Stat
        };
        let words = table.cell(row, position).to_owned();
        if position == 0 && laid.slot > px(0.0) {
            // The rolled row's mark sits in the slot; every row's range starts after it.
            return div()
                .w(laid.widths[0])
                .flex_none()
                .flex()
                .flex_row()
                .child(
                    div()
                        .w(laid.slot)
                        .flex_none()
                        .when(matched == Some(row) && row > 0, |slot| {
                            slot.child(super::super::icon(icons::DICE, Type::Stat, c.text))
                        }),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .ty(role)
                        .text_color(c.text)
                        .child(words),
                );
        }
        div()
            .w(laid.widths[position])
            .flex_none()
            .ty(role)
            .text_color(c.text)
            .child(words)
    };
    let row = |row: usize, positions: std::ops::Range<usize>| -> Div {
        div()
            .flex()
            .flex_row()
            .gap(TABLE_GAP)
            .h(laid.heights[row])
            .py(XS)
            .border_b(HAIRLINE_WIDTH)
            .border_color(c.hairline)
            .when(matched == Some(row) && row > 0, |line| line.bg(c.selected))
            .children(positions.map(|position| cell(row, position)))
    };
    let count = laid.widths.len();
    let rows = 0..table.rows.len();
    if !laid.scrolls {
        return div()
            .flex()
            .flex_col()
            .children(rows.map(|at| row(at, 0..count)))
            .into_any_element();
    }
    // The first column stays; the others scroll sideways under its rule. The scroll bar has its
    // own lane below the last row, so it never covers a cell's words.
    let lane = SCROLL_BAR_WIDTH + XS;
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(super::super::target(
            super::super::text(Type::Small, c.muted, MORE_COLUMNS),
            MORE_COLUMNS,
        ))
        .child(
            div()
                .flex()
                .flex_row()
                .w_full()
                .child(
                    div()
                        .flex_none()
                        .flex()
                        .flex_col()
                        .pb(lane)
                        .border_r(HAIRLINE_WIDTH)
                        .border_color(c.hairline)
                        .children(rows.clone().map(|at| row(at, 0..1))),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .pl(TABLE_GAP)
                        .child(super::super::scroll::sideways(
                            id,
                            div()
                                .flex()
                                .flex_col()
                                .pb(lane)
                                .children(rows.map(|at| row(at, 1..count))),
                        )),
                ),
        )
        .into_any_element()
}
