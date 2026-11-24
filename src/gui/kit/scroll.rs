//! A vertical scroll column (`design::ScrollColumn`): it keeps the scroll bar's lane whether or not
//! the bar shows, so nothing moves when the bar comes. The bar is gpui-component's, coloured by the
//! theme (`theme::install`). No scroll the app makes is animated (R43).
//!
//! Use `scroll_column` for a pane's body; give it a `ScrollHandle` you keep (in the page's GPUI
//! state) with `scroll_column_with` when the page scrolls it itself (a reveal, a jump to a row).
use crate::gui::theme::SCROLL_BAR_WIDTH;
use gpui::{
    ElementId, InteractiveElement, IntoElement, ParentElement, ScrollHandle,
    StatefulInteractiveElement, Styled, div,
};
use gpui_component::scroll::ScrollableElement;

/// A column that fills its parent and scrolls its `content` up and down.
pub fn scroll_column(id: impl Into<ElementId>, content: impl IntoElement) -> impl IntoElement {
    div()
        .id(id.into())
        .size_full()
        .min_h_0()
        .overflow_y_scrollbar()
        .child(lane(content))
}

/// A scroll column whose offset the page keeps and moves (`handle.scroll_to_item`,
/// `handle.set_offset`).
pub fn scroll_column_with(
    id: impl Into<ElementId>,
    handle: &ScrollHandle,
    content: impl IntoElement,
) -> impl IntoElement {
    div()
        .id(id.into())
        .size_full()
        .min_h_0()
        .overflow_y_scroll()
        .track_scroll(handle)
        .child(lane(content))
        .vertical_scrollbar(handle)
}

/// The content, one bar's lane narrower than the column.
fn lane(content: impl IntoElement) -> impl IntoElement {
    div().w_full().pr(SCROLL_BAR_WIDTH).child(content)
}

/// A strip that scrolls sideways (a wide table in a reader).
pub fn sideways(id: impl Into<ElementId>, content: impl IntoElement) -> impl IntoElement {
    div()
        .id(id.into())
        .w_full()
        .overflow_x_scrollbar()
        .child(content)
}

/// A virtual list (`gpui::uniform_list`, only the rows in view are built) with the kit's scroll
/// bar in its lane: a long list whose rows have one height (Timeline's events).
pub fn virtual_list(
    list: gpui::UniformList,
    handle: &gpui::UniformListScrollHandle,
) -> impl IntoElement {
    div()
        .size_full()
        .min_h_0()
        .child(list.track_scroll(handle).size_full().pr(SCROLL_BAR_WIDTH))
        .vertical_scrollbar(handle)
}
