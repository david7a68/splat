//! The empty state of a pane or page (`design::empty_state`): its icon, a serif title with no full
//! stop, what the place is for, and one next step, at the top of the pane with normal padding.
use crate::gui::theme::{READER_MEASURE, S, Type, XS, colors};
use gpui::{Div, IntoElement, ParentElement, SharedString, Styled, div};

/// `empty_state(icons::CAMPAIGN, "No campaign yet", "A campaign keeps…", Some(button))`.
pub fn empty_state(
    icon: &'static str,
    title: impl Into<SharedString>,
    body: impl Into<SharedString>,
    action: Option<impl IntoElement>,
) -> Div {
    let c = colors();
    let column = div()
        .flex()
        .flex_col()
        .items_start()
        .max_w(READER_MEASURE)
        .gap(S)
        .child(super::text::icon(icon, Type::Title, c.muted))
        .child(super::text::text(Type::Title, c.text, title))
        .child(super::text::text(Type::Body, c.muted, body));
    match action {
        Some(action) => column.child(div().pt(XS).child(action)),
        None => column,
    }
}
