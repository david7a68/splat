//! The rail's items: `NavItem` (a page, with an optional second line) and `NavToggle` (a panel
//! beside the page, with its key). Only the shell's rail uses these.
use super::{OnClick, button::element_name, target, tooltip::tip};
use crate::gui::theme::{
    Density, FOCUS_WIDTH, MARK_WIDTH, RADIUS_CONTROL, S, Type, TypeExt, XS, colors,
};
use gpui::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};
use std::rc::Rc;

/// Where the icon's column starts, and where the words start (egui's `S + 2` and 28).
const ICON_LEFT: f32 = 10.0;
const WORDS_LEFT: f32 = 28.0;

/// One page in the rail: an icon and a word, the accent bar and selected fill when it is the
/// page shown, and an optional muted second line (`Run` / `No fight open`). With a second line it
/// is a `Glance` control and `XS` tall, so the Run slot keeps one height in every state.
#[derive(IntoElement)]
pub struct NavItem {
    id: ElementId,
    icon: &'static str,
    label: SharedString,
    detail: Option<SharedString>,
    selected: bool,
    hint: Option<SharedString>,
    name: Option<SharedString>,
    on_click: Option<OnClick>,
}

impl NavItem {
    pub fn new(
        id: impl Into<ElementId>,
        icon: &'static str,
        label: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            label: label.into(),
            detail: None,
            selected: false,
            hint: None,
            name: None,
            on_click: None,
        }
    }

    /// The muted second line.
    pub fn detail(mut self, detail: Option<impl Into<SharedString>>) -> Self {
        self.detail = detail.map(Into::into);
        self
    }

    /// The page shown.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Hover words.
    pub fn hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// The accessible name, when it differs from `label · detail`.
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

impl RenderOnce for NavItem {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = colors();
        let two_lines = self.detail.is_some();
        let height = if two_lines {
            Density::Glance.control() + XS
        } else {
            Density::Scan.control()
        };
        let color = if self.selected { c.text } else { c.muted };
        let name = self.name.clone().unwrap_or_else(|| match &self.detail {
            Some(detail) => format!("{} · {detail}", self.label).into(),
            None => self.label.clone(),
        });
        let words = div()
            .flex()
            .flex_col()
            .min_w_0()
            .flex_1()
            .child(
                div()
                    .ty(if self.selected {
                        Type::Strong
                    } else {
                        Type::Body
                    })
                    .truncate()
                    .child(self.label.clone()),
            )
            .when_some(self.detail.clone(), |words, detail| {
                words.child(
                    div()
                        .ty(Type::Small)
                        .text_color(c.muted)
                        .truncate()
                        .child(detail),
                )
            });
        let mut item = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(height)
            .flex_none()
            .rounded(RADIUS_CONTROL)
            .text_color(color)
            .when(self.selected, |item| item.bg(c.selected))
            .when(!self.selected, |item| item.hover(|s| s.bg(c.hover)))
            .cursor_pointer()
            .focusable()
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(name)
            .aria_selected(self.selected)
            .child(
                div()
                    .w(px(WORDS_LEFT))
                    .pl(px(ICON_LEFT))
                    .flex_none()
                    .icon_font(Type::Body)
                    .child(SharedString::new_static(self.icon)),
            )
            .child(words.pr(S))
            .when(self.selected, |item| {
                item.child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .h_full()
                        .w(MARK_WIDTH)
                        .rounded(px(1.0))
                        .bg(c.accent),
                )
            });
        if let Some(hint) = self.hint {
            item = item.tooltip(tip(hint));
        }
        if let Some(on_click) = self.on_click {
            item = item.on_click(move |event, window, cx| on_click(event, window, cx));
        }
        let item = target(item, self.label.clone());
        target(item, element_name(&self.id))
    }
}

/// A panel toggle in the rail (Lookup, Notes): its icon and word, then its key in faint `Small`
/// at the right, or `Open` in the text colour while its panel is open.
#[derive(IntoElement)]
pub struct NavToggle {
    id: ElementId,
    icon: &'static str,
    label: SharedString,
    key: SharedString,
    open: bool,
    hint: Option<SharedString>,
    on_click: Option<OnClick>,
}

impl NavToggle {
    pub fn new(
        id: impl Into<ElementId>,
        icon: &'static str,
        label: impl Into<SharedString>,
        key: impl Into<SharedString>,
        open: bool,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            label: label.into(),
            key: key.into(),
            open,
            hint: None,
            on_click: None,
        }
    }

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

impl RenderOnce for NavToggle {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = colors();
        let color = if self.open { c.text } else { c.muted };
        let mut item = div()
            .id(self.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(Density::Scan.control())
            .flex_none()
            .rounded(RADIUS_CONTROL)
            .text_color(color)
            .when(self.open, |item| item.bg(c.selected))
            .when(!self.open, |item| item.hover(|s| s.bg(c.hover)))
            .cursor_pointer()
            .focusable()
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(self.label.clone())
            .child(
                div()
                    .w(px(WORDS_LEFT))
                    .pl(px(ICON_LEFT))
                    .flex_none()
                    .icon_font(Type::Body)
                    .child(SharedString::new_static(self.icon)),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .ty(Type::Body)
                    .truncate()
                    .child(self.label.clone()),
            )
            .child(
                div()
                    .pr(XS)
                    .flex_none()
                    .ty(Type::Small)
                    .text_color(if self.open { c.text } else { c.faint })
                    .child(if self.open {
                        SharedString::new_static("Open")
                    } else {
                        self.key.clone()
                    }),
            );
        if let Some(hint) = self.hint {
            item = item.tooltip(tip(hint));
        }
        if let Some(on_click) = self.on_click {
            item = item.on_click(move |event, window, cx| on_click(event, window, cx));
        }
        let item = target(item, self.label.clone());
        target(item, element_name(&self.id))
    }
}
