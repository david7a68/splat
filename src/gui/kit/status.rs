//! A status: an icon, a word and a colour (`design::Status`). Never a pill badge, never colour
//! alone. `status` shows it; `status_link` is one that opens where the DM can act on it.
use super::{OnClick, button::element_name, target, tooltip::tip};
use crate::gui::theme::{FOCUS_WIDTH, MIN_TARGET, RADIUS_CONTROL, Type, TypeExt, XS, colors};
use gpui::{
    App, ClickEvent, Div, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
};
use std::rc::Rc;

/// What a status says.
#[derive(Clone, Debug)]
pub struct Status {
    /// Its icon (`gui::icons`).
    pub icon: SharedString,
    /// Its word.
    pub word: SharedString,
    /// Its colour token.
    pub color: Hsla,
    /// The icon's own colour when the status's kind gives it one (R5: a live recording paints
    /// only its record icon in the recording colour).
    pub icon_color: Option<Hsla>,
}

impl Status {
    pub fn new(icon: impl Into<SharedString>, word: impl Into<SharedString>, color: Hsla) -> Self {
        Self {
            icon: icon.into(),
            word: word.into(),
            color,
            icon_color: None,
        }
    }

    /// The icon in its own colour.
    pub fn icon_color(mut self, color: Hsla) -> Self {
        self.icon_color = Some(color);
        self
    }
}

/// The icon and the word in `Small` type and the status colour. Wrapped lines start at the word,
/// never under the icon.
pub fn status(status: &Status) -> Div {
    div()
        .flex()
        .flex_row()
        .items_start()
        .gap(XS)
        .text_color(status.color)
        .child(
            div()
                .icon_font(Type::Small)
                .flex_none()
                .text_color(status.icon_color.unwrap_or(status.color))
                .child(status.icon.clone()),
        )
        .child(div().ty(Type::Small).min_w_0().child(status.word.clone()))
}

/// A status that opens where the DM can act on it: the status with a hover fill, a 24 px target
/// and hover words.
#[derive(IntoElement)]
pub struct StatusLink {
    id: ElementId,
    status: Status,
    hint: Option<SharedString>,
    on_click: Option<OnClick>,
}

/// A status link (`design::status_link`).
pub fn status_link(id: impl Into<ElementId>, state: Status) -> StatusLink {
    StatusLink {
        id: id.into(),
        status: state,
        hint: None,
        on_click: None,
    }
}

impl StatusLink {
    /// Hover words (the long form of its word).
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

impl RenderOnce for StatusLink {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = colors();
        let mut link = div()
            .id(self.id.clone())
            .flex()
            .items_center()
            .min_h(MIN_TARGET)
            .px(XS)
            .rounded(RADIUS_CONTROL)
            .cursor_pointer()
            .hover(|s| s.bg(c.hover))
            .focusable()
            .tab_stop(true)
            .role(gpui::Role::Link)
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(
                self.hint
                    .clone()
                    .unwrap_or_else(|| self.status.word.clone()),
            )
            .child(status(&self.status));
        if let Some(hint) = self.hint {
            link = link.tooltip(tip(hint));
        }
        if let Some(on_click) = self.on_click {
            link = link.on_click(move |event, window, cx| on_click(event, window, cx));
        }
        let link = target(link, self.status.word.clone());
        target(link, element_name(&self.id))
    }
}
