//! The creature filter control: a Filter menu with a submenu for each fact, and a
//! removable chip for each chosen value. Library and the creature search share it.
use crate::design::{self, Action, Density, Type, colors, icons};
use eframe::egui;
use splat::content::creature_filter::{Facet, Facets, Filter, challenge_value, choices};

/// The words for a challenge rating: 0.25 is `1/4`.
pub fn challenge_words(value: f64) -> String {
    [(0.125, "1/8"), (0.25, "1/4"), (0.5, "1/2")]
        .into_iter()
        .find(|(fraction, _)| *fraction == value)
        .map_or_else(|| format!("{value}"), |(_, words)| words.to_owned())
}

/// The words of a challenge range's chip: `CR 1 to 5`.
pub fn range_words((low, high): (Option<f64>, Option<f64>)) -> String {
    match (low.map(challenge_words), high.map(challenge_words)) {
        (Some(low), Some(high)) if low == high => format!("CR {low}"),
        (Some(low), Some(high)) => format!("CR {low} to {high}"),
        (Some(low), None) => format!("CR {low} or more"),
        (None, Some(high)) => format!("CR {high} or less"),
        (None, None) => String::new(),
    }
}

/// The Filter button's words: `Filter`, or `Filter (2)` with two values chosen.
pub fn filter_word(filter: &Filter) -> String {
    match filter.count() {
        0 => "Filter".to_owned(),
        count => format!("Filter ({count})"),
    }
}

/// A fact's row in the Filter menu: its label, with how many of its values are chosen.
pub fn facet_word(filter: &Filter, facet: Facet) -> String {
    let chosen = if facet == Facet::Challenge {
        usize::from(filter.challenge != (None, None))
    } else {
        filter.chosen.get(&facet).map_or(0, |v| v.len())
    };
    if chosen > 0 {
        format!("{} ({chosen})", facet.label())
    } else {
        facet.label().to_owned()
    }
}

/// The chosen values, each as its chip shows it: `Habitat: Underdark`.
pub fn chips(filter: &Filter) -> Vec<(Facet, String, String)> {
    filter
        .chosen
        .iter()
        .flat_map(|(facet, values)| {
            values
                .iter()
                .map(move |v| (*facet, v.clone(), format!("{}: {v}", facet.label())))
        })
        .collect()
}

/// A press on a challenge rating in the lowest (`high` false) or highest row: it becomes that
/// bound, and a second press on the same rating removes the bound.
pub fn choose_bound(filter: &mut Filter, high: bool, number: f64) {
    let bound = if high {
        &mut filter.challenge.1
    } else {
        &mut filter.challenge.0
    };
    *bound = (*bound != Some(number)).then_some(number);
}

/// The Filter menu and the chips. `creatures` gives the creatures that the menu counts;
/// it runs only while the menu is open. Returns true when the filter changed.
pub fn filter_bar<'a>(
    ui: &mut egui::Ui,
    filter: &mut Filter,
    creatures: impl FnOnce() -> Vec<&'a Facets>,
) -> bool {
    let before = filter.clone();
    ui.horizontal_wrapped(|ui| {
        let count = filter.count();
        let response = design::menu_button_at_line_start(ui, &filter_word(filter), count > 0);
        design::named(ui, &response, "Filter creatures");
        design::menu_popup(&*ui, &response)
            .show_submenus(|ui, submenus| menu(ui, submenus, filter, &choices(creatures())));
        for (facet, value, label) in chips(filter) {
            if chip(ui, &label) {
                filter.toggle(facet, &value);
            }
        }
        if filter.challenge != (None, None) && chip(ui, &range_words(filter.challenge)) {
            filter.challenge = (None, None);
        }
        if count > 1 && design::small_link(ui, "Clear filters").clicked() {
            *filter = Filter::default();
        }
    });
    *filter != before
}

fn chip(ui: &mut egui::Ui, label: &str) -> bool {
    Action::quiet(label)
        .icon(icons::CLOSE)
        .density(Density::Scan)
        .selected(true)
        .name(&format!("Remove filter {label}"))
        .show(ui)
        .clicked()
}

/// Each fact is a submenu of the Filter menu (`MenuPopup::show_submenus`, M0051): its choices
/// open under its row, in the menu, so none lies over the next fact's row, also in a narrow
/// pane; the menu scrolls its facts when the open ones are tall.
fn menu(
    ui: &mut egui::Ui,
    submenus: &mut design::Submenus,
    filter: &mut Filter,
    choices: &std::collections::BTreeMap<Facet, Vec<(String, usize)>>,
) {
    if choices.is_empty() {
        ui.label(
            Type::Small
                .rich(ui, "No creatures to filter.")
                .color(colors().muted),
        );
        return;
    }
    let facets: Vec<(Facet, &[(String, usize)])> = Facet::ALL
        .into_iter()
        .filter_map(|facet| Some((facet, choices.get(&facet)?.as_slice())))
        .collect();
    let words: Vec<String> = facets
        .iter()
        .map(|(facet, _)| facet_word(filter, *facet))
        .collect();
    // The widest fact's choices: the menu keeps one width whichever facts are open.
    let widest = facets
        .iter()
        .map(|(facet, values)| facet_width(ui, *facet, values))
        .fold(0.0, f32::max);
    submenus.show(ui, "creature filter facts", widest, &words, |ui, at| {
        let (facet, values) = facets[at];
        if facet == Facet::Challenge {
            challenge_menu(ui, filter, values);
        } else {
            for (value, count) in values {
                let mut on = filter.is_chosen(facet, value);
                let response = ui.checkbox(&mut on, format!("{value} ({count})"));
                design::named(ui, &response, &format!("Filter {}: {value}", facet.label()));
                if response.changed() {
                    filter.toggle(facet, value);
                }
            }
        }
    });
}

/// The width of a fact's choices: the challenge rows (`challenge_menu`), or the widest checkbox
/// with its words.
fn facet_width(ui: &egui::Ui, facet: Facet, values: &[(String, usize)]) -> f32 {
    if facet == Facet::Challenge {
        return challenge_width(ui);
    }
    let font = Type::Body.font(ui);
    let words = values
        .iter()
        .map(|(value, count)| design::number_width(ui, &format!("{value} ({count})"), &font))
        .fold(0.0, f32::max);
    ui.spacing().icon_width + ui.spacing().icon_spacing + words.ceil()
}

/// The width of the challenge rows: nine of the widest rating.
fn challenge_width(ui: &egui::Ui) -> f32 {
    design::number_width(ui, "1/8", &Type::Body.font(ui)) * 9.0
}

/// Two rows of challenge ratings: the lowest and the highest. A second click on a bound
/// removes it.
fn challenge_menu(ui: &mut egui::Ui, filter: &mut Filter, values: &[(String, usize)]) {
    // Never wider than the menu has: in the narrowest column the ratings wrap (review of PR 759).
    ui.set_max_width(challenge_width(ui).min(ui.available_width()));
    for (title, high) in [("Lowest", false), ("Highest", true)] {
        ui.label(Type::Small.rich(ui, title).color(colors().muted));
        ui.horizontal_wrapped(|ui| {
            for (value, _) in values {
                let Some(number) = challenge_value(value) else {
                    continue;
                };
                let bound = if high {
                    filter.challenge.1
                } else {
                    filter.challenge.0
                };
                let response = design::choice_wrapped(ui, bound == Some(number), value, None);
                design::named(ui, &response, &format!("{title} CR {value}"));
                if response.clicked() {
                    choose_bound(filter, high, number);
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge_ranges_read_as_words() {
        assert_eq!(range_words((Some(0.25), Some(0.25))), "CR 1/4");
        assert_eq!(range_words((Some(1.0), Some(5.0))), "CR 1 to 5");
        assert_eq!(range_words((Some(0.125), None)), "CR 1/8 or more");
        assert_eq!(range_words((None, Some(2.0))), "CR 2 or less");
    }
}
