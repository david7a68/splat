//! The Encounters page (state: `src/encounters.rs`, `src/encounters/`, `src/difficulty.rs`,
//! `src/creature_filter.rs`, `src/loot_editor.rs`, `src/encounters_ui/`).
//!
//! The page's state and logic stay in `Encounters` (`root.app.encounters`); its plain-data views
//! and acts are in `encounters::model`. This folder draws them: the
//! list (`list`), the encounter editor (`editor`) and the group editor (`group`).
use crate::gui::SplatRoot;
use gpui::{
    AnyElement, Context, InteractiveElement, IntoElement, ParentElement, Styled, Window, div,
};
use splat_encounters_ui::encounters::model::PageShown;

mod changes;
mod common;
mod creature;
mod disk;
mod editor;
mod filter;
mod group;
mod list;
mod loot;
#[cfg(test)]
pub(crate) mod tests;

/// This area's GPUI-only state, kept in `root.pages.encounters`.
#[derive(Default)]
pub struct State {
    /// The list's Groups section is closed.
    pub groups_closed: bool,
    /// The list's content issues are open (they start closed).
    pub issues_open: bool,
    /// The list's History section is closed (an ended run read there opens it).
    pub history_closed: bool,
    /// The side choice whose `Other…` field is open, by its id.
    pub other_side: Option<String>,
    /// Each loot list's kept state, by its key (`loot::list`).
    pub loot: std::collections::HashMap<String, loot::LootState>,
    /// Each Changes form's Other effect being typed, by the form's key.
    pub other_effects: std::collections::HashMap<String, String>,
    /// The width the open editor is laid out at (`common::room_probe`).
    pub room: std::rc::Rc<std::cell::Cell<Option<gpui::Pixels>>>,
    /// The search results' scroll (`kit::virtual_list::uniform_column`).
    pub results_scroll: gpui::UniformListScrollHandle,
    /// The results' facts width, by the ranking it was measured for (`editor::results_facts`).
    pub results_facts: Option<((usize, usize), gpui::Pixels)>,
    /// Each open replacement picker's words and its `Replace all`, by its row's key.
    pub replacing: std::collections::HashMap<String, (String, bool)>,
    /// The page's own keyboard focus, so its keys (Ctrl+S) reach it when the control that had
    /// the keyboard went away (a footer's Undo).
    pub focus: Option<gpui::FocusHandle>,
}

/// Once, when the root is built.
pub(crate) fn init(root: &mut SplatRoot, _window: &mut Window, cx: &mut Context<SplatRoot>) {
    root.pages.encounters.focus = Some(cx.focus_handle());
}

/// Once per frame, before anything draws.
pub(crate) fn poll(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Ctrl+S on this page (the root's one owner calls it, `pages::save_key`): the open creature
/// while it has changes, else the encounter or group (`Encounters::save_shortcut`). The Session
/// page's plan calls it for a group open there.
pub(crate) fn save_key(root: &mut SplatRoot, _: &mut Context<SplatRoot>) {
    common::with_page(root, |page, ctx, active, campaigns| {
        page.save_by_key_now(ctx, active, campaigns.modules());
    });
}

/// The group editor drawn in place of a session's plan while a group is assembled there
///. The Sessions page calls it; its Ctrl+S reaches
/// `save_key` through the Session page's (`sessions::save_key`).
pub(crate) fn render_group_in_plan(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let mut editor = div().key_context("Encounters");
    if let Some(focus) = root.pages.encounters.focus.clone() {
        if window
            .focused(cx)
            .is_none_or(|focused| focused == root.focus)
        {
            window.focus(&focus, cx);
        }
        editor = editor.track_focus(&focus);
    }
    editor
        .size_full()
        .child(group::render(root, window, cx))
        .into_any_element()
}

/// The page.
pub(crate) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let shown = root.app.encounters.page_frame(&root.app.campaigns);
    let body = match shown {
        None => div().into_any_element(),
        Some(PageShown::OtherCampaign(fault)) => div()
            .child(crate::gui::kit::Message::new(
                crate::gui::kit::messages::tone(fault.tone),
                fault.words,
            ))
            .into_any_element(),
        Some(PageShown::List) => list::render(root, window, cx),
        Some(PageShown::Encounter) => editor::render(root, window, cx),
        Some(PageShown::Group) => group::render(root, window, cx),
    };
    let mut page = div().key_context("Encounters");
    if let Some(focus) = root.pages.encounters.focus.clone() {
        let lost = window
            .focused(cx)
            .is_none_or(|focused| focused == root.focus);
        if lost {
            window.focus(&focus, cx);
        }
        page = page.track_focus(&focus);
    }
    page.size_full().child(body).into_any_element()
}
