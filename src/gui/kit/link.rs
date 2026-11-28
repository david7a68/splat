//! A text link (`design::link`, `small_link`, `wrapped_link`): accent words with an underline that
//! turn to the focus colour on hover, for a move to another place (`Open this session`, `Show all
//! events`, a note's name, a file). A link never changes data; an act that does is a quiet
//! `Button`. Its target is at least `XL` wide and `MIN_TARGET` high, so two short links side by
//! side never share a press area.
use super::{OnClick, button::element_name, target, tooltip::tip};
use crate::gui::theme::{FOCUS_WIDTH, MIN_TARGET, RADIUS_CONTROL, Type, TypeExt, XL, colors};
use gpui::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*,
};
use std::rc::Rc;

/// A link of `Body` words on one line (`.small()` for `Small`, `.wrap()` for words of unknown
/// length).
///
/// ```ignore
/// kit::Link::new("open-session", "Open this session")
///     .on_click(cx.listener(|root, _, _, cx| { …; cx.notify() }))
/// ```
#[derive(IntoElement)]
pub struct Link {
    id: ElementId,
    words: SharedString,
    ty: Type,
    wrap: bool,
    hint: Option<SharedString>,
    name: Option<SharedString>,
    on_click: Option<OnClick>,
}

/// `link(id, "Show file").on_click(…)`: the same as `Link::new`.
pub fn link(id: impl Into<ElementId>, words: impl Into<SharedString>) -> Link {
    Link::new(id, words)
}

impl Link {
    pub fn new(id: impl Into<ElementId>, words: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            words: words.into(),
            ty: Type::Body,
            wrap: false,
            hint: None,
            name: None,
            on_click: None,
        }
    }

    /// `Small` words (a link in a row's detail or a message's file line).
    pub fn small(mut self) -> Self {
        self.ty = Type::Small;
        self
    }

    /// The words wrap at the column's edge (`design::wrapped_link`): a note's name, a path, a
    /// check's line, whose length the app does not choose.
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }

    /// Words on hover (a path).
    pub fn hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// The accessible name, when it says more than the words. The harness finds the link by it
    /// too.
    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
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
            .min_w(XL)
            .rounded(RADIUS_CONTROL)
            .ty(self.ty)
            .text_color(c.accent)
            .underline()
            .cursor_pointer()
            .hover(|s| s.text_color(c.focus))
            .focusable()
            .tab_stop(true)
            .role(gpui::Role::Link)
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(name.clone());
        link = if self.wrap {
            // GPUI measures a text's smallest width as its whole line, so in a row a wrapped
            // link's words would never give way: the link and its words may shrink to the room
            // left beside the row's other parts, and the words wrap there.
            link.min_w_0()
                .flex_shrink_1()
                .child(div().min_w_0().child(self.words.clone()))
        } else {
            link.flex_none()
                .whitespace_nowrap()
                .child(self.words.clone())
        };
        if let Some(hint) = self.hint {
            link = link.tooltip(tip(hint));
        }
        if let Some(on_click) = self.on_click {
            link = link.on_click(move |event, window, cx| on_click(event, window, cx));
        }
        let link = match self.name.filter(|name| *name != self.words) {
            Some(name) => target(link, name),
            None => link,
        };
        let link = target(link, self.words);
        target(link, element_name(&self.id))
    }
}
