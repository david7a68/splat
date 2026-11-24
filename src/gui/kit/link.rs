//! A link (`design::link`, `small_link`): underlined words in the link colour that open a page or
//! a place. Use it for a way out that reads as words (`Open D&D Beyond sign-in`); an action that
//! changes something is a `kit::Button`.
use super::{button::element_name, target};
use crate::gui::theme::{FOCUS_WIDTH, MIN_TARGET, RADIUS_CONTROL, Type, TypeExt, colors};
use gpui::{
    App, ClickEvent, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, prelude::*,
};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct Link {
    id: ElementId,
    words: SharedString,
    small: bool,
    on_click: Option<super::OnClick>,
}

impl Link {
    pub fn new(id: impl Into<ElementId>, words: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            words: words.into(),
            small: false,
            on_click: None,
        }
    }

    /// `Small` words, in a line of help.
    pub fn small(mut self) -> Self {
        self.small = true;
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
        let ty = if self.small { Type::Small } else { Type::Body };
        let mut link = div()
            .id(self.id.clone())
            .flex()
            .flex_none()
            .items_center()
            .min_h(if self.small { ty.line() } else { MIN_TARGET })
            .rounded(RADIUS_CONTROL)
            .ty(ty)
            .text_color(c.accent)
            .underline()
            .cursor_pointer()
            .focusable()
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(self.words.clone())
            .hover(|s| s.text_color(c.text))
            .child(self.words.clone());
        if let Some(on_click) = self.on_click {
            link = link.on_click(move |event, window, cx| on_click(event, window, cx));
        }
        let link = target(link, self.words.clone());
        target(link, element_name(&self.id))
    }
}
