//! The templates view (`Sessions::templates_view`).
use crate::gui::SplatRoot;
use gpui::{AnyElement, App, Context, IntoElement, Window, div};

#[derive(Default)]
pub(crate) struct State {}

pub(super) fn init(_cx: &mut App) {}

pub(super) fn render(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    div().into_any_element()
}
