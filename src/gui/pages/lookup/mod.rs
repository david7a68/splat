//! Lookup, the search panel beside any page (egui: `src/lookup.rs`).
use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, Window};

/// This area's GPUI-only state, kept in `root.pages.lookup`.
#[derive(Default)]
pub struct State {}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Once per frame, before anything draws.
pub(crate) fn poll(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Lookup's panel while `root.app.lookup.open` (`Lookup::show`). The shell gives it the side
/// panel's width; Trackers is a view of it (`super::trackers::render_view`).
pub(crate) fn render_panel(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    if root.app.lookup.on_trackers() {
        return super::trackers::render_view(root, window, cx);
    }
    super::placeholder("", "Lookup")
}
