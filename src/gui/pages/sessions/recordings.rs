//! The session's recordings (`Sessions::recordings_tab`).
use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, IntoElement, Window, div};

pub(super) fn render(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    div().into_any_element()
}

pub(super) fn delete_dialog(
    _root: &mut SplatRoot,
    _cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    None
}
