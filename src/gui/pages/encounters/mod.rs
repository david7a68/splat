//! The Encounters page (egui: `src/encounters.rs`, `src/encounters/`, `src/difficulty.rs`,
//! `src/creature_filter.rs`, `src/loot_editor.rs`, `src/encounters_ui/`).
//!
//! The page's state and logic stay in `Encounters` (`root.app.encounters`); its plain-data views
//! and acts are in `encounters::model`, shared with the egui page. This folder draws them: the
//! list (`list`), the encounter editor (`editor`) and the group editor (`group`).
use crate::gui::SplatRoot;
use gpui::{
    AnyElement, Context, InteractiveElement, IntoElement, KeyBinding, ParentElement, Styled,
    Window, actions, div,
};
use splat_encounters_ui::encounters::model::PageShown;

mod common;
mod editor;
mod filter;
mod group;
mod list;
#[cfg(test)]
mod tests;

actions!(
    encounters,
    [
        /// Ctrl+S: save the open creature, encounter or group.
        Save,
    ]
);

/// This area's GPUI-only state, kept in `root.pages.encounters`.
#[derive(Default)]
pub struct State {
    /// The list's Groups section is closed.
    pub groups_closed: bool,
    /// The list's History section is closed (an ended run read there opens it).
    pub history_closed: bool,
    /// The words typed in the loot list's add field.
    pub loot_typed: String,
    /// The roster row whose side is being typed (`Other…`), by its key.
    pub other_side: Option<String>,
    /// The page's own keyboard focus, so its keys (Ctrl+S) reach it when the control that had
    /// the keyboard went away (a footer's Undo).
    pub focus: Option<gpui::FocusHandle>,
}

/// Once, when the root is built.
pub(crate) fn init(root: &mut SplatRoot, _window: &mut Window, cx: &mut Context<SplatRoot>) {
    cx.bind_keys([KeyBinding::new("ctrl-s", Save, Some("Encounters"))]);
    root.pages.encounters.focus = Some(cx.focus_handle());
}

/// Once per frame, before anything draws.
pub(crate) fn poll(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Ctrl+S on this page: the open creature while it has changes, else the encounter or group
/// (`Encounters::save_shortcut`); never under a modal.
fn save(root: &mut SplatRoot, _: &Save, _: &mut Window, cx: &mut Context<SplatRoot>) {
    if root.modal_open(cx) {
        return;
    }
    common::with_page(root, |page, ctx, active, campaigns| {
        page.save_by_key_now(ctx, active, campaigns.modules());
    });
    cx.notify();
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
                common::tone(fault.tone),
                fault.words,
            ))
            .into_any_element(),
        Some(PageShown::List) => list::render(root, window, cx),
        Some(PageShown::Encounter) => editor::render(root, window, cx),
        Some(PageShown::Group) => group::render(root, window, cx),
    };
    let mut page = div().key_context("Encounters");
    if let Some(focus) = root.pages.encounters.focus.clone() {
        let lost = window.focused(cx).is_none_or(|focused| focused == root.focus);
        if lost {
            window.focus(&focus, cx);
        }
        page = page.track_focus(&focus);
    }
    page.on_action(cx.listener(save))
        .size_full()
        .child(body)
        .into_any_element()
}
