//! A link (`design::link`, `wrapped_link`, `small_link`): words in the accent, underlined, that go
//! somewhere or open something (a session, an entry, a row's details). Its target is at least
//! `MIN_TARGET` tall; its words wrap. For an act that changes data, use a quiet `Button`.
use super::{OnClick, button::element_name, target, tooltip::tip};
use crate::gui::theme::{HAIRLINE_WIDTH, MIN_TARGET, Type, TypeExt, colors};
use gpui::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*,
};
use std::rc::Rc;

/// `kit::Link::new("last-time", "Session 7").on_click(..)`.
#[derive(IntoElement)]
pub struct Link {
    id: ElementId,
    words: SharedString,
    ty: Type,
    hint: Option<SharedString>,
    on_click: Option<OnClick>,
}

impl Link {
    pub fn new(id: impl Into<ElementId>, words: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            words: words.into(),
            ty: Type::Body,
            hint: None,
            on_click: None,
        }
    }

    /// Its type (default `Body`; `Small` for a link in a detail line).
    pub fn ty(mut self, ty: Type) -> Self {
        self.ty = ty;
        self
    }

    /// Words on hover (the whole words of a cut link).
    pub fn hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Link {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = colors();
        let mut link = div()
            .id(self.id.clone())
            .flex()
            .items_center()
            .min_h(MIN_TARGET)
            .min_w_0()
            .cursor_pointer()
            .ty(self.ty)
            .text_color(c.accent)
            .hover(|s| s.text_color(c.accent_hover))
            .child(
                div()
                    .underline()
                    .text_decoration_1()
                    .text_decoration_color(c.accent)
                    .child(self.words.clone()),
            )
            .aria_label(self.words.clone());
        let _ = HAIRLINE_WIDTH;
        if let Some(hint) = self.hint.clone() {
            link = link.tooltip(tip(hint));
        }
        if let Some(on_click) = self.on_click {
            link = link.on_click(move |event, window, cx| on_click(event, window, cx));
        }
        let link = target(link, self.words.clone());
        target(link, element_name(&self.id))
    }
}
