//! A quiet section (egui `design::quiet_section`): a closed-by-default part of a page for what is
//! rarely wanted (where an entry comes from, technical details). Its header is muted `Small` words
//! after a caret, one `Read` control high; a press opens or closes it. Whether it is open is kept
//! under its id, so the caller holds no flag.
//!
//! ```ignore
//! kit::QuietSection::new(("source", entry.id), "Source and identifiers").child(details)
//! ```
use crate::gui::icons;
use crate::gui::theme::{Density, S, Type, TypeExt, XS, colors};
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*,
};

/// A section that opens on a press.
#[derive(IntoElement)]
pub struct QuietSection {
    id: ElementId,
    title: SharedString,
    open: bool,
    children: Vec<AnyElement>,
}

impl QuietSection {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            open: false,
            children: Vec::new(),
        }
    }

    /// Open when it first shows.
    pub fn open_at_first(mut self) -> Self {
        self.open = true;
        self
    }
}

impl ParentElement for QuietSection {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuietSection {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = colors();
        let first = self.open;
        let open = window.use_keyed_state(self.id.clone(), cx, move |_, _| first);
        let is_open = *open.read(cx);
        let header = div()
            .id(self.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .gap(XS)
            .h(Density::Read.control())
            .cursor_pointer()
            .ty(Type::Small)
            .text_color(c.muted)
            .hover(move |s| s.text_color(c.text))
            .aria_label(self.title.clone())
            .child(super::icon(
                if is_open {
                    icons::phosphor::CARET_DOWN
                } else {
                    icons::phosphor::CARET_RIGHT
                },
                Type::Small,
                c.muted,
            ))
            .child(self.title.clone())
            .on_click(move |_, _, cx| {
                open.update(cx, |open, cx| {
                    *open = !*open;
                    cx.notify();
                });
            });
        div()
            .flex()
            .flex_col()
            .child(super::target(header, self.title))
            .when(is_open, |section| {
                section.child(div().flex().flex_col().pl(S + XS).children(self.children))
            })
    }
}
