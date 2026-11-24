//! The Library page and the copy dialog (egui: `src/library.rs`, `src/library/`,
//! `src/processing.rs`, `src/processing/`, `src/dndbeyond.rs`, `src/credentials.rs`,
//! `src/copy_dialog.rs`, `src/library_page/`).
use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, ParentElement as _, Window};

/// This area's GPUI-only state, kept in `root.pages.library`.
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
    super::placeholder("Library", "The Library page")
}

/// The copy dialog (`copy_dialog::CopyDialog`), while `root.app.copy_dialog` is open. Draw it
/// with `kit::Dialog`; it covers the window.
pub(crate) fn render_copy_dialog(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    gpui::IntoElement::into_any_element(crate::gui::kit::Dialog::new("copy-dialog", "Copy").child(
        crate::gui::kit::text(
            crate::gui::theme::Type::Body,
            crate::gui::theme::colors().muted,
            "The copy dialog is not in the GPUI app yet.",
        ),
    ))
}
