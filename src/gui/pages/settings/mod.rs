//! Settings → Account and campaigns, and Recording and processing (egui: `src/settings.rs`,
//! the Settings branch of `Splat::page`). The shell draws the header, the tabs and Touchpad
//! (`SplatRoot::settings_page`).
use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, Window};

/// This area's GPUI-only state, kept in `root.pages.settings`.
#[derive(Default)]
pub struct State {}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Once per frame, before anything draws.
pub(crate) fn poll(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Settings → Account and campaigns.
pub(crate) fn render_account(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    super::placeholder("", "Account and campaigns")
}

/// Settings → Recording and processing (`ProcessingSettings::pane`).
pub(crate) fn render_recording(
    _root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    super::placeholder("", "Recording and processing")
}

/// The add-campaign form that the Campaign page shows when there is no campaign
/// (`Campaigns::show_add_form`, `form_error_inline`).
pub(crate) fn render_add_campaign_form(
    _root: &mut SplatRoot,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    super::placeholder("", "The add-campaign form")
}
