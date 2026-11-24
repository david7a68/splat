//! A segmented choice (`design::segmented`): words with an accent underline under the chosen one,
//! no pill. Each segment is as wide as its words in their widest state (`Strong`), and its words
//! start `S` from its left edge, so nothing moves when the choice changes. Use it for a page's
//! tabs (Settings, Campaign) and for a small exclusive choice (Touchpad speed).
use super::{button::element_name, target};
use crate::gui::theme::{
    Density, FOCUS_WIDTH, RADIUS_CONTROL, S, Type, TypeExt, UNDERLINE_WIDTH, colors,
};
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};
use std::rc::Rc;

type OnSelect = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// `Segmented::new("settings-tabs", ["Account and campaigns", …], chosen)
///     .on_select(cx.listener(|root, index: &usize, _, cx| …))`.
#[derive(IntoElement)]
pub struct Segmented {
    id: ElementId,
    options: Vec<SharedString>,
    chosen: usize,
    on_select: Option<OnSelect>,
}

impl Segmented {
    pub fn new(
        id: impl Into<ElementId>,
        options: impl IntoIterator<Item = impl Into<SharedString>>,
        chosen: usize,
    ) -> Self {
        Self {
            id: id.into(),
            options: options.into_iter().map(Into::into).collect(),
            chosen,
            on_select: None,
        }
    }

    /// Called with the index of a segment the DM chose that was not already chosen. Its
    /// signature fits `cx.listener(|root, index: &usize, window, cx| …)`.
    pub fn on_select(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(move |index, window, cx| {
            handler(&index, window, cx)
        }));
        self
    }
}

impl RenderOnce for Segmented {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = colors();
        let segments = self.options.iter().enumerate().map(|(index, words)| {
            let chosen = index == self.chosen;
            let id = ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), words.clone());
            // The widest state's words keep the room (`Strong`), drawn invisibly under the shown
            // words, so a segment is as wide chosen or not.
            let room = div()
                .ty(Type::Strong)
                .text_color(gpui::transparent_black())
                .h(px(0.0))
                .overflow_hidden()
                .child(words.clone());
            let mut segment = div()
                .id(id.clone())
                .relative()
                .flex()
                .flex_col()
                .justify_center()
                .h(Density::Read.control())
                .px(S)
                .rounded(RADIUS_CONTROL)
                .cursor_pointer()
                .hover(|s| s.bg(c.hover))
                .focusable()
                .focus_visible(|s| s.border(FOCUS_WIDTH).border_color(c.focus))
                .aria_label(words.clone())
                .aria_selected(chosen)
                .child(
                    div()
                        .ty(if chosen { Type::Strong } else { Type::Body })
                        .text_color(if chosen { c.text } else { c.muted })
                        .whitespace_nowrap()
                        .child(words.clone()),
                )
                .child(room)
                .when(chosen, |segment| {
                    segment.child(
                        div()
                            .absolute()
                            .bottom_0()
                            .left(px(2.0))
                            .right(px(2.0))
                            .h(UNDERLINE_WIDTH)
                            .bg(c.accent),
                    )
                });
            if let Some(on_select) = self.on_select.clone().filter(|_| !chosen) {
                segment = segment.on_click(move |_, window, cx| on_select(index, window, cx));
            }
            let segment = target(segment, words.clone());
            target(segment, element_name(&id))
        });
        div()
            .id(self.id.clone())
            .flex()
            .flex_row()
            .flex_wrap()
            .children(segments)
    }
}
