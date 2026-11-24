//! A quiet section (`design::quiet_section`): a caret and `Small` muted words that open or close
//! the section's content under them. It keeps its own open state under its id, as egui's
//! collapsing header does, and starts closed. Use it for what most readers skip: Technical
//! details, Source and identifiers, a form that waits (`New location…`).
use super::{button::element_name, target};
use crate::gui::icons;
use crate::gui::theme::{FOCUS_WIDTH, MIN_TARGET, RADIUS_CONTROL, S, Type, TypeExt, XS, colors};
use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*,
};

/// `QuietSection::new(("tech", id), "Technical details").child(…)`.
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

    /// Start open (a short list), until the DM closes it.
    pub fn default_open(mut self, open: bool) -> Self {
        self.open = open;
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
        let open = window.use_keyed_state(
            ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), "open".into()),
            cx,
            move |_, _| first,
        );
        let is_open = *open.read(cx);
        let header = div()
            .id(self.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .gap(XS)
            .min_h(MIN_TARGET)
            .px(XS)
            .ml(-XS)
            .rounded(RADIUS_CONTROL)
            .cursor_pointer()
            .hover(|s| s.bg(c.hover))
            .focusable()
            .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
            .aria_label(self.title.clone())
            .on_click(move |_, window, cx| {
                open.update(cx, |open, _| *open = !*open);
                window.refresh();
            })
            .child(
                div()
                    .icon_font(Type::Small)
                    .text_color(c.muted)
                    .child(SharedString::new_static(if is_open {
                        icons::DOWN
                    } else {
                        icons::phosphor::CARET_RIGHT
                    })),
            )
            .child(super::text::text(Type::Small, c.muted, self.title.clone()));
        let header = target(header, self.title.clone());
        let header = target(header, element_name(&self.id));
        div()
            .flex()
            .flex_col()
            .child(header)
            .when(is_open, |section| {
                section.child(div().flex().flex_col().gap(S).pl(S).children(self.children))
            })
    }
}
