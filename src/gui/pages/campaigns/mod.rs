//! The Campaign page's tabs (egui: `src/campaigns.rs`, `src/campaign_content.rs`,
//! `src/campaign_content/`). The shell draws the page's header and tabs (`SplatRoot::campaign_page`).
use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, Window};

/// This area's GPUI-only state, kept in `root.pages.campaigns`.
#[derive(Default)]
pub struct State {}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Once per frame, before anything draws.
pub(crate) fn poll(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Campaign → Packages.
pub(crate) fn render_packages(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    super::placeholder("", "Packages")
}

/// Campaign → Rules and runs (`Encounters::run_settings`).
pub(crate) fn render_run_settings(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    super::placeholder("", "Rules and runs")
}

/// Campaign → Link and folder.
pub(crate) fn render_link(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    super::placeholder("", "Link and folder")
}
