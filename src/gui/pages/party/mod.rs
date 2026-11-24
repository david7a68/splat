//! The Party page and the party bar on the Session page (egui: `src/party*.rs`,
//! `src/party_support/`, `src/party_page/`, `Splat::party_bar`).
use crate::gui::SplatRoot;
use gpui::{AnyElement, Context, Window};

/// This area's GPUI-only state, kept in `root.pages.party`.
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
    super::placeholder("Party", "The Party page")
}

/// The party bar on the Session page while a session is played (`Splat::party_bar`), or `None`.
/// It also sets `sessions.party_hidden` as the egui bar does.
pub(crate) fn render_bar(
    root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let app = &mut root.app;
    let here = app.page == crate::Page::Sessions
        && app.campaigns.active_campaign().is_some()
        && app.party.has_members();
    let shows = here && app.sessions.party_bar_shows();
    app.sessions.party_hidden = here && !shows;
    None
}
