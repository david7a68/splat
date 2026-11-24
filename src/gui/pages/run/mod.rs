//! The live run: the Run page and the run strip on other pages (egui: `src/encounter_run.rs`,
//! `src/encounter_run/`, `src/combat_sync.rs`, `src/rolls.rs`, `src/roll_table.rs`).
use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, Window};

/// This area's GPUI-only state, kept in `root.pages.run`.
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
    super::placeholder("Run", "The Run page")
}

/// The run strip above every other page while a run is open (`Splat::run_strip`, `TurnStrip`).
pub(crate) fn render_strip(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    super::placeholder("", "The run strip")
}
