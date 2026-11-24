//! Lookup, the search panel beside any page. Ctrl+K opens and closes it;
//! Escape closes it (the shell's `Dismiss`); Alt+Left and Alt+Right go Back and Forward. Its
//! acts are `Lookup`'s own (`lookup::view`).
mod entry;
mod panel;
#[cfg(test)]
mod tests;

use crate::Page;
use crate::gui::SplatRoot;
use crate::history::Step;
use gpui::{
    AnyElement, Context, FocusHandle, InteractiveElement, IntoElement, KeyBinding, MouseButton,
    NavigationDirection, ParentElement, Styled, Window, actions, div,
};

actions!(
    lookup,
    [
        /// Alt+Left: Lookup's Back, unless the Library page has the keyboard.
        Back,
        /// Alt+Right: Lookup's Forward.
        Forward,
    ]
);

/// This area's GPUI-only state, kept in `root.pages.lookup`.
#[derive(Default)]
pub struct State {
    /// The reader's scroll and its heading jump.
    pub(crate) jump: crate::gui::pages::world::reader::Jump,
    /// The panel's keyboard place, so Back by key knows whether the keyboard is in Lookup.
    pub(super) panel_focus: Option<FocusHandle>,
}

/// Once, when the root is built. Alt+Left and Alt+Right are the root's, so they reach Lookup
/// from any page; the Library binds them in its own, deeper context first.
pub(crate) fn init(root: &mut SplatRoot, _window: &mut Window, cx: &mut Context<SplatRoot>) {
    cx.bind_keys([
        KeyBinding::new("alt-left", Back, Some("SplatRoot")),
        KeyBinding::new("alt-right", Forward, Some("SplatRoot")),
    ]);
    root.pages.lookup.panel_focus = Some(cx.focus_handle());
}

/// Alt+Left at the root (`history_input`): Lookup goes Back while it is open, unless the
/// Library page is shown and the keyboard is not in Lookup; then the Library goes Back.
pub(crate) fn back(
    root: &mut SplatRoot,
    _: &Back,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) {
    step(root, Step::Back, window, cx);
}

/// Alt+Right at the root, as `back`.
pub(crate) fn forward(
    root: &mut SplatRoot,
    _: &Forward,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) {
    step(root, Step::Forward, window, cx);
}

/// A side button over the panel: Lookup's, on any page.
fn press_step(root: &mut SplatRoot, step: Step, cx: &mut Context<SplatRoot>) {
    if !root.modal_open(cx) {
        root.app.lookup.go(step);
        cx.stop_propagation();
        cx.notify();
    }
}

fn step(root: &mut SplatRoot, step: Step, window: &mut Window, cx: &mut Context<SplatRoot>) {
    if root.modal_open(cx) {
        return;
    }
    let in_panel = root
        .pages
        .lookup
        .panel_focus
        .as_ref()
        .is_some_and(|focus| focus.contains_focused(window, cx));
    if root.app.lookup.open && (root.app.page != Page::Library || in_panel) {
        root.app.lookup.go(step);
    } else if root.app.page == Page::Library {
        match step {
            Step::Back => root.app.library.back(),
            Step::Forward => root.app.library.forward(),
        }
    } else {
        cx.propagate();
        return;
    }
    cx.notify();
}

/// Once per frame, before anything draws: the found notes are read again when the panel opens
/// (`Lookup::wants_written`).
pub(crate) fn poll(root: &mut SplatRoot, _: &mut Window, _: &mut Context<SplatRoot>) {
    // A press on a result row, or Lookup closing under its field, leaves nothing with the
    // keyboard: the root takes it back in that draw (`SplatRoot::watch_keys`).
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
    let body = if root.app.lookup.on_trackers() {
        super::trackers::render_view(root, window, cx)
    } else {
        panel::render(root, window, cx)
    };
    // A press in the panel gives it the keyboard, so Alt+Left goes to Lookup beside the
    // Library too; the mouse's side buttons over the panel are Lookup's (`history_input`).
    let mut panel = div().id("lookup-panel").size_full();
    if let Some(focus) = &root.pages.lookup.panel_focus {
        panel = panel.track_focus(focus);
    }
    panel
        .on_mouse_down(
            MouseButton::Navigate(NavigationDirection::Back),
            cx.listener(|root, _, _, cx| press_step(root, Step::Back, cx)),
        )
        .on_mouse_down(
            MouseButton::Navigate(NavigationDirection::Forward),
            cx.listener(|root, _, _, cx| press_step(root, Step::Forward, cx)),
        )
        .child(body)
        .into_any_element()
}
