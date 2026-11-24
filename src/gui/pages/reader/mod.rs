//! The Reader: entries and notes shown as reference (egui: `src/markdown_view.rs`,
//! `src/markdown_source.rs`, `src/content_ui/`). Its Markdown view is `kit::markdown`; the
//! readers that pages show (Library, World, Lookup) build on what this area adds.
use crate::gui::SplatRoot;
use gpui::{Context, Window};

/// This area's GPUI-only state, kept in `root.pages.reader`.
#[derive(Default)]
pub struct State {}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Once per frame, before anything draws.
pub(crate) fn poll(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}
