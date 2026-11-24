//! A long list that draws only the rows in view (`design::BlockRegion`, egui's clipped lists):
//! GPUI's `uniform_list` in a column that keeps its scroll bar's lane, as `scroll_column` does.
//! Every row has one height. Keep the `UniformListScrollHandle` in the page's state to move the
//! list from code (`handle.scroll_to_item(row, ScrollStrategy::Nearest)`).
use crate::gui::theme::SCROLL_BAR_WIDTH;
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, Styled,
    UniformListScrollHandle, Window, div, uniform_list,
};
use gpui_component::scroll::ScrollableElement;
use std::ops::Range;

/// `count` rows, each made by `row` for the rows in view (`cx.processor(|root, range, window,
/// cx| …)` reads the page's state).
pub fn virtual_column<R: IntoElement + 'static>(
    id: impl Into<ElementId>,
    handle: &UniformListScrollHandle,
    count: usize,
    rows: impl Fn(Range<usize>, &mut Window, &mut App) -> Vec<R> + 'static,
) -> impl IntoElement {
    let id = id.into();
    div()
        .id(ElementId::NamedChild(
            std::sync::Arc::new(id.clone()),
            "column".into(),
        ))
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
