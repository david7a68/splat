//! The Encounters page (egui: `src/encounters.rs`, `src/encounters/`, `src/difficulty.rs`,
//! `src/creature_filter.rs`, `src/loot_editor.rs`, `src/encounters_ui/`).
use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, Window};

/// This area's GPUI-only state, kept in `root.pages.encounters`.
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
    super::placeholder("Encounters", "The Encounters page")
}
