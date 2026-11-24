//! The Session page, the notes panel beside other pages, and the session banner (egui:
//! `src/sessions/`, `src/sessions.rs`, `src/sessions_ui/`, `src/session_records/`).
use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, Window};

/// This area's GPUI-only state, kept in `root.pages.sessions`.
#[derive(Default)]
pub struct State {}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Once per frame, before anything draws.
pub(crate) fn poll(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// The page.
pub(crate) fn render(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    super::placeholder("Session", "The Session page")
}

/// The notes panel beside another page (`Sessions::notes_panel`), while `notes_open` and the
/// page is not Session. The shell gives it the side panel's width.
pub(crate) fn render_notes_panel(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    super::placeholder("", "The notes panel")
}

/// The session banner above every page (`Sessions::banner`: recording and processing notices),
/// or `None` when it has nothing to say.
pub(crate) fn render_banner(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    None
}
