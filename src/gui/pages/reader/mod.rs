//! The Reader: entries and notes shown as reference (egui: `src/markdown_view.rs`,
//! `src/markdown_source.rs`, `src/content_ui/`). Its Markdown view is `kit::markdown`; the
//! readers that pages show (Library, World, Lookup) build on what this area adds.
pub mod entry;

use crate::gui::SplatRoot;
use gpui::{Context, Window};

/// This area's GPUI-only state, kept in `root.pages.reader`.
#[derive(Default)]
pub struct State {
    /// Tests only: a reader part drawn in place of the page, to capture it.
    #[cfg(test)]
    pub(crate) preview: Option<Preview>,
}

/// A test's reader part, drawn in place of the page.
#[cfg(test)]
pub(crate) type Preview =
    std::rc::Rc<dyn Fn(&mut SplatRoot, &mut Window, &mut Context<SplatRoot>) -> gpui::AnyElement>;

/// Once, when the root is built: readers find the app's egui context (the link index, the art)
/// and leave pressed links there for the frame logic, as egui's readers do.
pub(crate) fn init(root: &mut SplatRoot, _window: &mut Window, cx: &mut Context<SplatRoot>) {
    cx.set_global(crate::gui::kit::markdown::Host(root.bridge.ctx().clone()));
}

/// Once per frame, before anything draws.
pub(crate) fn poll(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Tests only: the test's reader part in place of the page.
#[cfg(test)]
pub(crate) fn render_preview(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> Option<gpui::AnyElement> {
    let preview = root.pages.reader.preview.clone()?;
    Some(preview(root, window, cx))
}

#[cfg(test)]
mod tests;
