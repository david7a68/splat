//! The Campaign page's tabs (egui: `src/campaigns.rs`, `src/campaign_content.rs`,
//! `src/campaign_content/`). The shell draws the page's header and tabs (`SplatRoot::campaign_page`).
mod link;
mod packages;
mod run_settings;
#[cfg(test)]
mod tests;

use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, KeyBinding, Window, actions};

actions!(
    campaigns,
    [
        /// Ctrl+S on Packages: the open repair's Save, else `Save selections`.
        Save,
    ]
);

/// This area's GPUI-only state, kept in `root.pages.campaigns`.
#[derive(Default)]
pub struct State {
    /// Link and folder's `Technical details` is open.
    pub(crate) details_open: bool,
    /// Packages' issue list is open.
    pub(crate) issues_open: bool,
    /// The repairs whose editor is open, by their files.
    pub(crate) repairs_open: std::collections::BTreeSet<String>,
    /// Packages' keyboard focus, so Ctrl+S reaches the tab while nothing else holds the
    /// keyboard.
    pub(crate) focus: Option<gpui::FocusHandle>,
}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, cx: &mut Context<SplatRoot>) {
    cx.bind_keys([KeyBinding::new("ctrl-s", Save, Some("CampaignPackages"))]);
}

/// Once per frame, before anything draws.
pub(crate) fn poll(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Campaign → Packages.
pub(crate) fn render_packages(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    packages::render(root, window, cx)
}

/// Campaign → Rules and runs (`Encounters::run_settings`).
pub(crate) fn render_run_settings(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    run_settings::render(root, window, cx)
}

/// Campaign → Link and folder.
pub(crate) fn render_link(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    link::render(root, window, cx)
}
