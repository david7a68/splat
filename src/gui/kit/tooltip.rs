//! Words on hover. Use a tooltip for a hint that says more than a control's name (its key, what it
//! opens) or for a blocked control's reason. Never put something the DM needs only there.
use crate::gui::theme::{
    HAIRLINE_WIDTH, RADIUS_LAYER, S, TOOLTIP_WIDTH, Type, TypeExt, XS, colors,
};
use gpui::{
    AnyView, App, AppContext, Context, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Window, div,
};

/// The hover layer's own view.
pub struct Tip(SharedString);

impl Render for Tip {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        tip_box(self.0.clone())
    }
}

/// The tooltip's box with `words`, for a part that shows the same words while it holds keyboard
/// focus (GPUI's tooltip shows on hover only).
pub fn tip_box(words: SharedString) -> gpui::Div {
    let c = colors();
    div()
        .max_w(TOOLTIP_WIDTH)
        .px(S)
        .py(XS)
        .bg(c.raised)
        .border(HAIRLINE_WIDTH)
        .border_color(c.rule)
        .rounded(RADIUS_LAYER)
        .ty(Type::Small)
        .text_color(c.text)
        .child(words)
}

/// Build a tooltip view of `words` (for `StatefulInteractiveElement::tooltip`).
pub fn tip(words: SharedString) -> impl Fn(&mut Window, &mut App) -> AnyView + 'static {
    move |_, cx| cx.new(|_| Tip(words.clone())).into()
}

/// `.with_tooltip(words)` on any element with an id.
pub trait WithTooltip: StatefulInteractiveElement + Sized {
    /// Show `words` on hover.
    fn with_tooltip(self, words: impl Into<SharedString>) -> Self {
        self.tooltip(tip(words.into()))
    }
}

impl<T: StatefulInteractiveElement> WithTooltip for T {}
