//! The page areas, one folder each. The shell (`gui::SplatRoot`) calls each area through the
//! hooks below.
//!
//! Every area has:
//! - `State`: its GPUI-only state (scroll handles, focus handles, which row a menu is open on),
//!   kept in `SplatRoot::pages`. The app's own state stays in `root.app` (`crate::Splat`).
//! - `init(root, window, cx)`: once, when the root is built.
//! - `poll(root, window, cx)`: once per frame, before anything draws (after the background polls,
//!   before `Splat::route`).
//! - its render hooks, each `fn(&mut SplatRoot, &mut Window, &mut Context<SplatRoot>) ->
//!   AnyElement`: the page (`render`), and the bands and panels the shell places for it.
use super::SplatRoot;
use crate::Page;
use gpui::{AnyElement, Context, IntoElement, Window, div};

pub mod campaigns;
pub mod encounters;
pub mod library;
pub mod lookup;
pub mod party;
pub mod reader;
pub mod run;
pub mod sessions;
pub mod settings;
pub mod timeline;
pub mod trackers;
pub mod world;

/// Every area's GPUI-only state.
#[derive(Default)]
#[allow(
    dead_code,
    reason = "each page keeps its GPUI state here; some fields wait for their page"
)]
pub struct States {
    pub campaigns: campaigns::State,
    pub encounters: encounters::State,
    pub library: library::State,
    pub lookup: lookup::State,
    pub party: party::State,
    pub reader: reader::State,
    pub run: run::State,
    pub sessions: sessions::State,
    pub settings: settings::State,
    pub timeline: timeline::State,
    pub trackers: trackers::State,
    pub world: world::State,
}

/// Each area's `init`, once, when the root is built.
pub(crate) fn init(root: &mut SplatRoot, window: &mut Window, cx: &mut Context<SplatRoot>) {
    campaigns::init(root, window, cx);
    encounters::init(root, window, cx);
    library::init(root, window, cx);
    lookup::init(root, window, cx);
    party::init(root, window, cx);
    reader::init(root, window, cx);
    run::init(root, window, cx);
    sessions::init(root, window, cx);
    settings::init(root, window, cx);
    timeline::init(root, window, cx);
    trackers::init(root, window, cx);
    world::init(root, window, cx);
}

/// Each area's `poll`, once per frame before anything draws.
pub(crate) fn poll(root: &mut SplatRoot, window: &mut Window, cx: &mut Context<SplatRoot>) {
    campaigns::poll(root, window, cx);
    encounters::poll(root, window, cx);
    library::poll(root, window, cx);
    lookup::poll(root, window, cx);
    party::poll(root, window, cx);
    reader::poll(root, window, cx);
    run::poll(root, window, cx);
    sessions::poll(root, window, cx);
    settings::poll(root, window, cx);
    timeline::poll(root, window, cx);
    trackers::poll(root, window, cx);
    world::poll(root, window, cx);
}

/// The page shown, by its area's `render`. Settings and Campaign keep their header and tabs in
/// the shell and call their areas per tab (`SplatRoot::settings_page`, `campaign_page`); a page
/// that needs a campaign shows the shell's no-campaign state while none is open.
pub(crate) fn render_page(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    match root.app.page {
        Page::Run => run::render(root, window, cx),
        Page::Encounters => encounters::render(root, window, cx),
        Page::Sessions => sessions::render(root, window, cx),
        Page::Timeline => timeline::render(root, window, cx),
        Page::World => world::render(root, window, cx),
        Page::Party => party::render(root, window, cx),
        Page::Library => library::render(root, window, cx),
        // Drawn by the shell; listed so the match stays whole.
        Page::Campaign | Page::Settings => div().into_any_element(),
    }
}

/// Ctrl+S on the page shown (the root's one owner, `SplatRoot::save`, calls it after the Notes
/// panel and Lookup's edit form had their turn): each area's `save_key` saves exactly what its own Save would
/// (`DraftState::can_save`). A page that needs a
/// campaign saves nothing while none is open (it shows the no-campaign state).
pub(crate) fn save_key(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) {
    let needs_campaign = !matches!(root.app.page, Page::Library | Page::Settings | Page::Run);
    if needs_campaign && root.app.campaigns.active_campaign().is_none() {
        return;
    }
    match root.app.page {
        Page::Encounters => encounters::save_key(root, cx),
        Page::Sessions => sessions::save_key(root, cx),
        Page::Timeline => timeline::save_key(root, cx),
        Page::World => world::save_key(root, cx),
        Page::Library => library::save_key(root, cx),
        Page::Campaign => campaigns::save_key(root, cx),
        // No draft that Ctrl+S saves.
        Page::Run | Page::Party | Page::Settings => {}
    }
}
