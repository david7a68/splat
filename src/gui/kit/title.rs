//! A page's header (`design::page_header`, `TitleLine::page`): the title in `Title` type and an
//! optional muted `Small` context on its baseline, the actions at the right, a hairline, then an `L`
//! gap. The title and its context come first; the actions take the room they leave and wrap there.
//! Never repeat the campaign name in it.
use crate::gui::theme::{Density, L, LINE_GAP, S, Type, XS, colors};
use gpui::{AnyElement, Div, IntoElement, ParentElement, SharedString, Styled, div};

/// `page_header("Settings", None, Vec::new())`; actions are kit buttons (the page's primary last).
pub fn page_header(
    title: impl Into<SharedString>,
    context: Option<SharedString>,
    actions: Vec<AnyElement>,
) -> Div {
    let context = context.map(|context| {
        super::text::text(Type::Small, colors().muted, context)
            .truncate()
            .into_any_element()
    });
    page_header_with(title, context, actions)
}

/// `page_header` with a context that is an element (a `kit::status`: its icon, word and colour).
pub fn page_header_with(
    title: impl Into<SharedString>,
    context: Option<AnyElement>,
    actions: Vec<AnyElement>,
) -> Div {
    let c = colors();
    let mut heading = div()
        .flex()
        .flex_row()
        .items_baseline()
        .gap(S + XS)
        .flex_none()
        .child(super::text::text(Type::Title, c.text, title).whitespace_nowrap());
    if let Some(context) = context {
        heading = heading.child(div().min_w_0().child(context));
    }
    div()
        .flex()
        .flex_col()
        .w_full()
        .flex_none()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .min_h(Density::Scan.control())
                .gap(S)
                .child(heading)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .flex_1()
                        .justify_end()
                        .items_center()
                        .gap(S)
                        .children(actions),
                ),
        )
        .child(super::text::rule().mt(LINE_GAP))
        .child(super::text::gap(L))
}

/// Boxed elements for `page_header`'s actions.
pub fn actions(items: impl IntoIterator<Item = AnyElement>) -> Vec<AnyElement> {
    items.into_iter().collect()
}
