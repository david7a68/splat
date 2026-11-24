//! Long lists that build only the rows in view (`design::BlockRegion`, egui's clipped lists), in a
//! column that keeps its scroll bar's lane as `scroll_column` does. Two forms, one per row shape:
//!
//! - `uniform_column`: every row has one height (Library's entries, Timeline's events). GPUI's
//!   `uniform_list`; keep its `UniformListScrollHandle` in the page's GPUI state to move the list
//!   from code (`handle.scroll_to_item(row, ScrollStrategy::Nearest)`).
//! - `measured_column`: rows of their own heights (World's headings and two-line notes). GPUI's
//!   `list`; keep its `ListState` in the page's GPUI state, and `splice` it when the rows change
//!   (it measures each row once, as it comes into view).
use crate::gui::theme::SCROLL_BAR_WIDTH;
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ListState, ParentElement, Styled,
    UniformListScrollHandle, Window, div, uniform_list,
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

/// The column's own id, under the list's.
fn column_id(id: &ElementId) -> ElementId {
    ElementId::NamedChild(std::sync::Arc::new(id.clone()), "column".into())
}
