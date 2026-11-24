//! A link in running words (`design::link`, `small_link`): the accent colour, underlined, a 24 px
//! target with a hover fill. Use it for a quiet act that reads as words (`Show file`, `Edit in
//! Obsidian`, a file of a fault list), never for a page's commit.
use super::{OnClick, button::element_name, target, tooltip::tip};
use crate::gui::theme::{
    FOCUS_WIDTH, HAIRLINE_WIDTH, MIN_TARGET, RADIUS_CONTROL, Type, TypeExt, XS, colors,
};
use gpui::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, UnderlineStyle, Window, div,
};
use std::rc::Rc;

#[derive(IntoElement)]
pub struct Link {
    id: ElementId,
    words: SharedString,
    small: bool,
    hint: Option<SharedString>,
    on_click: Option<OnClick>,
}

/// `link(id, "Show file").on_click(…)`.
pub fn link(id: impl Into<ElementId>, words: impl Into<SharedString>) -> Link {
    Link {
        id: id.into(),
        words: words.into(),
        small: false,
        hint: None,
        on_click: None,
    }
}

impl Link {
    /// `Small` words (a link in a list of faults).
    pub fn small(mut self) -> Self {
        self.small = true;
        self
    }

    /// Words on hover (a whole path the link opens).
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
            .flex_none()
            .items_center()
            .min_h(MIN_TARGET)
            .px(XS)
            .rounded(RADIUS_CONTROL)
            .ty(if self.small { Type::Small } else { Type::Body })
            .text_color(c.accent)
            .text_decoration_1()
            .text_decoration_color(c.accent)
            .whitespace_nowrap()
            .cursor_pointer()
            .hover(|s| s.bg(c.hover))
            .focusable()
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(self.words.clone())
            .child(self.words.clone());
        let _ = (HAIRLINE_WIDTH, UnderlineStyle::default());
        if let Some(hint) = self.hint {
            link = link.tooltip(tip(hint));
        }
        if let Some(on_click) = self.on_click {
            link = link.on_click(move |event, window, cx| on_click(event, window, cx));
        }
        let link = target(link, self.words.clone());
        target(link, element_name(&self.id))
    }
}
