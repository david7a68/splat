//! A checkbox (a yes-or-no setting with words: `Remember my D&D Beyond sign-in on this computer`)
//! and a switch (a setting that acts at once, such as Combat sync). The whole line is the target,
//! at least 24 px high.
use super::{OnClick, button::element_name, target, tooltip::tip};
use crate::gui::icons::phosphor;
use crate::gui::theme::{
    FOCUS_WIDTH, HAIRLINE_WIDTH, MIN_TARGET, RADIUS_CONTROL, S, Type, TypeExt, XS, colors,
};
use gpui::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};
use std::rc::Rc;

/// `Checkbox::new("remember", "Remember my sign-in", checked).on_click(...)`; the handler flips
/// the caller's value.
#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    label: SharedString,
    checked: bool,
    blocked: Option<SharedString>,
    on_click: Option<OnClick>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, checked: bool) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            checked,
            blocked: None,
            on_click: None,
        }
    }

    /// Disabled with a reason on hover.
    pub fn blocked(mut self, reason: Option<impl Into<SharedString>>) -> Self {
        self.blocked = reason.map(Into::into);
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

impl RenderOnce for Checkbox {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = colors();
        let size = px(16.0);
        let enabled = self.blocked.is_none();
        let mark = div()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(size)
            .rounded(RADIUS_CONTROL)
            .border(HAIRLINE_WIDTH)
            .border_color(c.field_border)
            .bg(c.well)
            .icon_font(Type::Small)
            .text_color(c.text)
            .when(self.checked, |mark| {
                mark.child(SharedString::new_static(phosphor::CHECK))
            });
        line(
            self.id,
            self.label,
            mark,
            self.blocked,
            enabled.then_some(self.on_click).flatten(),
            gpui::accesskit::Toggled::from(self.checked),
        )
    }
}

/// A switch: a track with a knob, on or off, then its words.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    label: SharedString,
    on: bool,
    blocked: Option<SharedString>,
    on_click: Option<OnClick>,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, on: bool) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            on,
            blocked: None,
            on_click: None,
        }
    }

    pub fn blocked(mut self, reason: Option<impl Into<SharedString>>) -> Self {
        self.blocked = reason.map(Into::into);
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

impl RenderOnce for Switch {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = colors();
        let enabled = self.blocked.is_none();
        let track = div()
            .flex()
            .flex_none()
            .items_center()
            .w(px(28.0))
            .h(px(16.0))
            .px(px(2.0))
            .rounded(px(8.0))
            .bg(if self.on {
                c.accent_fill
            } else {
                c.field_border
            })
            .when(self.on, |track| track.justify_end())
            .child(div().size(px(12.0)).rounded(px(6.0)).bg(if self.on {
                c.on_accent
            } else {
                c.text
            }));
        line(
            self.id,
            self.label,
            track,
            self.blocked,
            enabled.then_some(self.on_click).flatten(),
            gpui::accesskit::Toggled::from(self.on),
        )
    }
}

fn line(
    id: ElementId,
    label: SharedString,
    mark: gpui::Div,
    blocked: Option<SharedString>,
    on_click: Option<OnClick>,
    toggled: gpui::accesskit::Toggled,
) -> impl IntoElement {
    let c = colors();
    let enabled = blocked.is_none();
    let mut line = div()
        .id(id.clone())
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .min_h(MIN_TARGET)
        .pr(XS)
        .rounded(RADIUS_CONTROL)
        .focusable()
        .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
        .aria_label(label.clone())
        .aria_toggled(toggled)
        .child(mark)
        .child(div().ty(Type::Body).text_color(c.text).child(label.clone()));
    if let Some(reason) = blocked {
        line = line
            .opacity(super::button::DISABLED_OPACITY)
            .tooltip(tip(reason));
    }
    if enabled && let Some(on_click) = on_click {
        line = line
            .cursor_pointer()
            .on_click(move |event, window, cx| on_click(event, window, cx));
    }
    let line = target(line, label);
    target(line, element_name(&id))
}
