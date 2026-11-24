//! Lookup, the search panel beside any page (egui: `src/lookup.rs`). Ctrl+K opens and closes it;
//! Escape closes it (the shell's `Dismiss`). Its acts are `Lookup`'s own (`lookup::view`).
mod entry;
mod panel;
#[cfg(test)]
mod tests;

use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, Window};

/// This area's GPUI-only state, kept in `root.pages.lookup`.
#[derive(Default)]
pub struct State {
    /// The panel was open in the last frame: when it closes, its field's keyboard goes back to
    /// the root.
    was_open: bool,
    /// The reader's scroll and its heading jump.
    pub(crate) jump: crate::gui::pages::world::reader::Jump,
}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Once per frame, before anything draws: the found notes are read again when the panel opens
/// (`Lookup::wants_written`, as the egui shell passes them in).
pub(crate) fn poll(root: &mut SplatRoot, window: &mut Window, cx: &mut Context<SplatRoot>) {
    // A press on a result row, or Lookup closing under its field, leaves nothing with the
    // keyboard: the root takes it back, so Escape, Ctrl+K and the page's keys still reach it.
    let open = root.app.lookup.open;
    if window.focused(cx).is_none() || (root.pages.lookup.was_open && !open) {
        window.focus(&root.focus, cx);
    }
    root.pages.lookup.was_open = open;
    let app = &mut root.app;
    if app.lookup.open && app.lookup.wants_written() {
        let written = app.sessions.written(&app.timeline);
        app.lookup.set_written(written);
    }
}

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
    panel::render(root, window, cx)
}
