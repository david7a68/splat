//! Long lists that build only the rows in view (`design::BlockRegion`), in a
//! column that keeps its scroll bar's lane as `scroll_column` does. Two forms, one per row shape:
//!
//! - `uniform_column`: every row has one height (Library's entries, Timeline's events). GPUI's
//!   `uniform_list`; keep its `UniformListScrollHandle` in the page's GPUI state to move the list
//!   from code (`handle.scroll_to_item(row, ScrollStrategy::Nearest)`).
//! - `measured_column`: rows of their own heights (World's headings and two-line notes). GPUI's
//!   `list`; keep its `ListState` in the page's GPUI state, and `splice` it when the rows change
//!   (it measures each row once, as it comes into view).
//! - `placed_rows`: rows whose heights the page has already worked out (the Run roster, which
//!   fits every row to its room before it draws). The content of a `scroll_column_with`: the
//!   page keeps its `ScrollHandle` and the rows' places, so it scrolls to a row and says what is
//!   out of view from them, as for a list that builds every row.
use crate::gui::theme::SCROLL_BAR_WIDTH;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ListState, ParentElement,
    ScrollHandle, Styled, UniformListScrollHandle, Window, div, px, uniform_list,
};
use gpui_component::scroll::ScrollableElement;
use std::ops::Range;

/// `count` rows of one height, each made by `rows` for the rows in view (`cx.processor(|root,
/// range, window, cx| …)` reads the page's state). The list's element id is `id`.
pub fn uniform_column<R: IntoElement + 'static>(
    id: impl Into<ElementId>,
    handle: &UniformListScrollHandle,
    count: usize,
    rows: impl Fn(Range<usize>, &mut Window, &mut App) -> Vec<R> + 'static,
) -> impl IntoElement {
    let id = id.into();
    div()
        .id(column_id(&id))
        .size_full()
        .min_h_0()
        .child(
            uniform_list(id, count, rows)
                .track_scroll(handle)
                .size_full()
                .pr(SCROLL_BAR_WIDTH),
        )
        .vertical_scrollbar(handle)
}

/// The rows of `state` (its item count), each made by `row` when it comes into view, at its own
/// height.
pub fn measured_column(
    id: impl Into<ElementId>,
    state: &ListState,
    row: impl FnMut(usize, &mut Window, &mut App) -> AnyElement + 'static,
) -> impl IntoElement {
    div()
        .id(column_id(&id.into()))
        .size_full()
        .min_h_0()
        .child(
            gpui::list(state.clone(), row)
                .size_full()
                .pr(SCROLL_BAR_WIDTH),
        )
        .vertical_scrollbar(state)
}

/// The rows of a `scroll_column_with` on `handle` whose tops and bottoms are `spans` (in order,
/// from 0): only those within one view's height of the view are made, by `row` with their place
/// in `spans`; the rest are blank space of their heights, so the column keeps its whole height
/// and every row its place. `view` is the column's height until it has been laid out once.
pub fn placed_rows(
    spans: &[(f32, f32)],
    handle: &ScrollHandle,
    view: f32,
    mut row: impl FnMut(usize) -> AnyElement,
) -> AnyElement {
    let shown = f32::from(handle.bounds().size.height);
    let shown = if shown > 1.0 { shown } else { view };
    let top = -f32::from(handle.offset().y);
    let range = in_view(spans, top - shown, top + 2.0 * shown);
    let above = range.clone().next().map_or(0.0, |first| spans[first].0);
    let end = spans.last().map_or(0.0, |last| last.1);
    let below = range
        .clone()
        .last()
        .map_or(end - above, |last| end - spans[last].1);
    div()
        .flex()
        .flex_col()
        .w_full()
        .child(div().flex_none().h(px(above)))
        .children(range.map(&mut row))
        .child(div().flex_none().h(px(below)))
        .into_any_element()
}

/// The rows of `spans` that reach into `from..to`.
fn in_view(spans: &[(f32, f32)], from: f32, to: f32) -> Range<usize> {
    let first = spans.partition_point(|(_, bottom)| *bottom <= from);
    let last = spans.partition_point(|(top, _)| *top < to);
    first..last.max(first)
}

/// The column's own id, under the list's.
fn column_id(id: &ElementId) -> ElementId {
    ElementId::NamedChild(std::sync::Arc::new(id.clone()), "column".into())
}

#[cfg(test)]
mod tests {
    #[test]
    fn only_rows_that_reach_into_the_range_are_in_view() {
        let spans: Vec<(f32, f32)> = (0..100)
            .map(|at| (at as f32 * 10.0, at as f32 * 10.0 + 10.0))
            .collect();
        assert_eq!(super::in_view(&spans, -50.0, 25.0), 0..3);
        assert_eq!(super::in_view(&spans, 100.0, 130.0), 10..13);
        assert_eq!(super::in_view(&spans, 995.0, 2000.0), 99..100);
        assert_eq!(super::in_view(&spans, 2000.0, 3000.0), 100..100);
        assert_eq!(super::in_view(&[], 0.0, 10.0), 0..0);
    }
}
