//! The creature filter (egui: `creature_filter::filter_bar`), as the kit's `FilterBar`: the
//! filter lives in the bridge's egui context under the egui page's id, so the roster and a group
//! share it.
use super::editor::FILTER_ID;
use crate::gui::{SplatRoot, kit};
use eframe::egui;
use gpui::{Context, ElementId, IntoElement};
use splat_content::content::creature_filter::Filter;

/// The facts' choices for the open menu (`Encounters::filter_choices`).
pub(super) type Choices = kit::creature_filter::Choices;

/// The bar's id.
fn bar_id() -> ElementId {
    ElementId::Name("creature-filter".into())
}

/// The id of the Filter menu.
pub(super) fn menu_id() -> ElementId {
    kit::FilterBar::menu_id(&bar_id())
}

/// The Filter button, its menu, and the chips.
pub(super) fn bar(
    filter: &Filter,
    choices: Option<&Choices>,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    let choices = choices.cloned().unwrap_or_default();
    kit::FilterBar::new(bar_id(), filter, move || choices, cx).on_change(cx.listener(
        |root: &mut SplatRoot, filter: &Filter, _, cx| {
            let id = egui::Id::new(FILTER_ID);
            root.bridge
                .ctx()
                .data_mut(|data| data.insert_temp(id, filter.clone()));
            cx.notify();
        },
    ))
}
