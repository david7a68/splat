//! The Campaign page's tabs (state: `src/campaigns.rs`, `src/campaign_content.rs`,
//! `src/campaign_content/`). The shell draws the page's header and tabs (`SplatRoot::campaign_page`).
mod link;
mod packages;
mod run_settings;
#[cfg(test)]
mod tests;

use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, Window};

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

/// Ctrl+S on the Campaign page (`pages::save_key`): the shown tab's save, exactly when its button
/// can (`Save selections`, `Save run settings`, `Save folder`).
pub(crate) fn save_key(root: &mut SplatRoot, _: &mut Context<SplatRoot>) {
    match root.app.campaign_tab {
        crate::CampaignTab::Packages => {
            if let Some(active) = root.app.campaigns.active_campaign() {
                packages::save_key(root, &active);
            }
        }
        crate::CampaignTab::RunSettings => {
            let Some(active) = root.app.campaigns.active_campaign() else {
                return;
            };
            if root
                .app
                .encounters
                .run_row(&active)
                .is_some_and(|row| row.state.can_save())
            {
                let SplatRoot { app, ctx, .. } = root;
                app.encounters
                    .save_run_settings(ctx, &active, &app.campaigns);
            }
        }
        crate::CampaignTab::Link => {
            let lock = crate::campaigns::Campaigns::link_lock(root.app.blocked_switch());
            let crate::Splat {
                sessions, timeline, ..
            } = &mut root.app;
            if sessions
                .folder_row(timeline, lock)
                .is_some_and(|(state, _)| state.can_save())
            {
                sessions.folder_act(timeline, crate::sessions::page::FolderAct::Save);
            }
        }
    }
}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

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
