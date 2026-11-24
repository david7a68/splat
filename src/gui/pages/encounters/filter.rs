//! The creature filter (egui: `creature_filter::filter_bar`): a Filter menu with each fact's
//! values, a removable chip for each chosen value, and Clear filters. The filter lives in the
//! bridge's egui context under the egui page's id, so the roster and a group share it.
use super::editor::FILTER_ID;
use crate::gui::theme::{Density, S, Type, XS, colors};
use crate::gui::{SplatRoot, icons, kit};
use eframe::egui;
use gpui::{Context, Div, ElementId, ParentElement, Styled, div};
use splat_content::content::creature_filter::{Facet, Filter, challenge_value};

/// The page's filter, changed by `change`.
fn change(root: &mut SplatRoot, change: impl FnOnce(&mut Filter)) {
    let ctx = root.bridge.ctx().clone();
    let id = egui::Id::new(FILTER_ID);
    let mut filter: Filter = ctx.data(|data| data.get_temp(id)).unwrap_or_default();
    change(&mut filter);
    ctx.data_mut(|data| data.insert_temp(id, filter));
}

/// The words of a challenge range: `CR 1 to 5`.
fn range_words((low, high): (Option<f64>, Option<f64>)) -> String {
    let words = splat_content_ui::creature_filter::challenge_words;
    match (low, high) {
        (Some(low), Some(high)) if low == high => format!("CR {}", words(low)),
        (Some(low), Some(high)) => format!("CR {} to {}", words(low), words(high)),
        (Some(low), None) => format!("CR {} or more", words(low)),
        (None, Some(high)) => format!("CR {} or less", words(high)),
        (None, None) => String::new(),
    }
}

/// The Filter button, its menu, and the chips.
/// The facts' choices for the open menu (`Encounters::filter_choices`).
pub(super) type Choices = std::collections::BTreeMap<Facet, Vec<(String, usize)>>;

/// The id of the Filter menu.
pub(super) fn menu_id() -> ElementId {
    ElementId::Name("creature-filter".into())
}

pub(super) fn bar(filter: &Filter, choices: Option<&Choices>, cx: &mut Context<SplatRoot>) -> Div {
    let count = filter.count();
    let word = if count == 0 {
        "Filter".to_owned()
    } else {
        format!("Filter ({count})")
    };
    let id = menu_id();
    let menu = match choices {
        Some(choices) => menu(filter, choices, cx),
        None => div(),
    };
    let trigger = kit::Button::quiet("creature-filter-button", format!("{word} {}", icons::DOWN))
        .line_start()
        .selected(count > 0)
        .name("Filter creatures");
    let mut bar = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(S)
        .child(kit::Popover::new(id, trigger, menu));
    let chosen: Vec<(Facet, String)> = filter
        .chosen
        .iter()
        .flat_map(|(facet, values)| values.iter().map(move |v| (*facet, v.clone())))
        .collect();
    for (at, (facet, value)) in chosen.into_iter().enumerate() {
        let label = format!("{}: {value}", facet.label());
        bar = bar.child(
            kit::Button::quiet(
                ElementId::Name(format!("filter-chip:{at}").into()),
                label.clone(),
            )
            .icon(icons::CLOSE)
            .density(Density::Scan)
            .selected(true)
            .name(format!("Remove filter {label}"))
            .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                change(root, |filter| filter.toggle(facet, &value));
                cx.notify();
            })),
        );
    }
    if filter.challenge != (None, None) {
        bar = bar.child(
            kit::Button::quiet("filter-chip-challenge", range_words(filter.challenge))
                .icon(icons::CLOSE)
                .density(Density::Scan)
                .selected(true)
                .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                    change(root, |filter| filter.challenge = (None, None));
                    cx.notify();
                })),
        );
    }
    if count > 1 {
        bar = bar.child(
            kit::Button::quiet("filter-clear", "Clear filters")
                .plain()
                .on_click(cx.listener(|root: &mut SplatRoot, _, _, cx| {
                    change(root, |filter| *filter = Filter::default());
                    cx.notify();
                })),
        );
    }
    bar
}

/// The menu: each fact's values as check boxes, and the challenge's lowest and highest.
fn menu(filter: &Filter, choices: &Choices, cx: &mut Context<SplatRoot>) -> Div {
    let c = colors();
    let mut menu = div().flex().flex_col().gap(XS).p(S).max_h(gpui::px(420.0));
    if choices.is_empty() {
        return menu.child(kit::text(Type::Small, c.muted, "No creatures to filter."));
    }
    for facet in Facet::ALL {
        let Some(values) = choices.get(&facet) else {
            continue;
        };
        menu = menu.child(kit::text(Type::Label, c.text, facet.label().to_uppercase()));
        if facet == Facet::Challenge {
            for (title, high) in [("Lowest", false), ("Highest", true)] {
                let mut row = div().flex().flex_row().flex_wrap().gap(XS);
                for (value, _) in values {
                    let Some(number) = challenge_value(value) else {
                        continue;
                    };
                    let bound = if high {
                        filter.challenge.1
                    } else {
                        filter.challenge.0
                    };
                    row = row.child(
                        kit::Button::quiet(
                            ElementId::Name(format!("filter-cr:{title}:{value}").into()),
                            value.clone(),
                        )
                        .density(Density::Read)
                        .selected(bound == Some(number))
                        .name(format!("{title} CR {value}"))
                        .on_click(cx.listener(
                            move |root: &mut SplatRoot, _, _, cx| {
                                change(root, |filter| {
                                    let bound = if high {
                                        &mut filter.challenge.1
                                    } else {
                                        &mut filter.challenge.0
                                    };
                                    *bound = (*bound != Some(number)).then_some(number);
                                });
                                cx.notify();
                            },
                        )),
                    );
                }
                menu = menu
                    .child(kit::text(Type::Small, c.muted, title))
                    .child(row);
            }
            continue;
        }
        for (value, count) in values {
            let chosen = filter.is_chosen(facet, value);
            let value = value.clone();
            menu = menu.child(
                kit::Checkbox::new(
                    ElementId::Name(format!("filter:{}:{value}", facet.label()).into()),
                    format!("{value} ({count})"),
                    chosen,
                )
                .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                    change(root, |filter| filter.toggle(facet, &value));
                    cx.notify();
                })),
            );
        }
    }
    menu_scroll(menu)
}

fn menu_scroll(menu: Div) -> Div {
    div()
        .child(kit::scroll_column("creature-filter-menu", menu))
        .h(gpui::px(420.0))
        .w(gpui::px(280.0))
}
