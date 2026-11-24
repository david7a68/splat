//! A link (`design::link`, `small_link`, `wrapped_link`): words in the accent colour with an
//! underline, the focus colour on hover, and a target at least 24 px tall. Use it to open a note,
//! a file or a place, never for an act that changes data (that is a button).
use super::{OnClick, button::element_name, target, tooltip::tip};
use crate::gui::theme::{FOCUS_WIDTH, MIN_TARGET, RADIUS_CONTROL, Type, TypeExt, colors};
use gpui::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*,
};
use std::rc::Rc;

/// `Link::new(("open", id), "Ashfall Keep").on_click(…)`.
#[derive(IntoElement)]
pub struct Link {
    id: ElementId,
    words: SharedString,
    name: Option<SharedString>,
    ty: Type,
    hint: Option<SharedString>,
    on_click: Option<OnClick>,
}

impl Link {
    pub fn new(id: impl Into<ElementId>, words: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            words: words.into(),
            name: None,
            ty: Type::Body,
            hint: None,
            on_click: None,
        }
    }

    /// `Small` words (a file line's links).
    pub fn small(mut self) -> Self {
        self.ty = Type::Small;
        self
    }

    /// The accessible name, when it says more than the words.
    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Hover words (a file's whole path).
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
        let name = self.name.clone().unwrap_or_else(|| self.words.clone());
        let mut link = div()
            .id(self.id.clone())
            .flex()
            .items_center()
            .min_h(MIN_TARGET)
            .rounded(RADIUS_CONTROL)
            .cursor_pointer()
            .ty(self.ty)
            .text_color(c.accent)
            .underline()
            .hover(|s| s.text_color(c.focus))
            .focusable()
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(name.clone())
            .child(self.words.clone());
        if let Some(hint) = self.hint {
            link = link.tooltip(tip(hint));
        }
        if let Some(on_click) = self.on_click {
            link = link.on_click(move |event, window, cx| on_click(event, window, cx));
        }
        let link = target(link, name);
        let link = target(link, self.words);
        target(link, element_name(&self.id))
    }
}
