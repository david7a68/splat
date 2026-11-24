//! Trackers, a view of Lookup (egui: `src/trackers/`). Lookup's panel draws it while
//! `root.app.lookup.on_trackers()`.
use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, Window};

/// This area's GPUI-only state, kept in `root.pages.trackers`.
#[derive(Default)]
pub struct State {}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Once per frame, before anything draws.
pub(crate) fn poll(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Trackers in Lookup's panel.
pub(crate) fn render_view(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    super::placeholder("", "Trackers")
}
