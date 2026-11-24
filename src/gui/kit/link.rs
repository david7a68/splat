//! A text link (egui `design::link`, `small_link`): accent words with an underline, on one line,
//! whose press target is at least `XL` wide and `XL` high. Use it for a link among words or in a
//! list of names (a related entry, a spell's description); use a `Button` for an action.
//!
//! ```ignore
//! kit::Link::new(("related", index), name).on_click(cx.listener(move |root, _, _, cx| …))
//! ```
use super::OnClick;
use crate::gui::theme::{HAIRLINE_WIDTH, Type, TypeExt, XL, colors};
use gpui::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*,
};
use std::rc::Rc;

/// A one-line text link.
#[derive(IntoElement)]
pub struct Link {
    id: ElementId,
    words: SharedString,
    ty: Type,
    hint: Option<SharedString>,
    on_click: Option<OnClick>,
}

impl Link {
    /// A `Body` link.
    pub fn new(id: impl Into<ElementId>, words: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            words: words.into(),
            ty: Type::Body,
            hint: None,
            on_click: None,
        }
    }

    /// A `Small` link, with the same target.
    pub fn small(mut self) -> Self {
        self.ty = Type::Small;
        self
    }

    /// Words on hover (where the link goes).
    pub fn hint(mut self, words: impl Into<SharedString>) -> Self {
        self.hint = Some(words.into());
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Link {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = colors();
        let mut link = div()
            .id(self.id)
            .flex()
            .flex_row()
            .flex_none()
            .items_center()
            .min_w(XL)
            .h(XL)
            .cursor_pointer()
            .ty(self.ty)
            .text_color(c.accent)
            .whitespace_nowrap()
            .aria_label(self.words.clone())
            .child(
                div()
                    .border_b(HAIRLINE_WIDTH)
                    .border_color(c.accent)
                    .child(self.words.clone()),
            )
            .hover(move |s| s.text_color(c.focus));
        if let Some(words) = self.hint {
            link = link.tooltip(super::tooltip::tip(words));
        }
        if let Some(on_click) = self.on_click {
            link = link.on_click(move |event, window, cx| on_click(event, window, cx));
        }
        super::target(link, self.words)
    }
}
