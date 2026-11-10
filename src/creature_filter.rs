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

fn range_words((low, high): (Option<f64>, Option<f64>)) -> String {
    match (low.map(challenge_words), high.map(challenge_words)) {
        (Some(low), Some(high)) if low == high => format!("CR {low}"),
        (Some(low), Some(high)) => format!("CR {low} to {high}"),
        (Some(low), None) => format!("CR {low} or more"),
        (None, Some(high)) => format!("CR {high} or less"),
        (None, None) => String::new(),
    }
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
        let word = if count == 0 {
            "Filter".to_owned()
        } else {
            format!("Filter ({count})")
        };
        let response = design::menu_button(ui, &word, count > 0);
        design::named(ui, &response, "Filter creatures");
        egui::Popup::menu(&response)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .show(|ui| menu(ui, filter, &choices(creatures())));
        let chosen: Vec<(Facet, String)> = filter
            .chosen
            .iter()
            .flat_map(|(facet, values)| values.iter().map(move |v| (*facet, v.clone())))
            .collect();
        for (facet, value) in chosen {
            let label = format!("{}: {value}", facet.label());
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

fn menu(
    ui: &mut egui::Ui,
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
    for facet in Facet::ALL {
        let Some(values) = choices.get(&facet) else {
            continue;
        };
        let chosen = if facet == Facet::Challenge {
            usize::from(filter.challenge != (None, None))
        } else {
            filter.chosen.get(&facet).map_or(0, |v| v.len())
        };
        let label = if chosen > 0 {
            format!("{} ({chosen})", facet.label())
        } else {
            facet.label().to_owned()
        };
        ui.menu_button(label, |ui| {
            egui::ScrollArea::vertical()
                .max_height(ui.ctx().content_rect().height() * 0.6)
                .show(ui, |ui| {
                    if facet == Facet::Challenge {
                        challenge_menu(ui, filter, values);
                    } else {
                        for (value, count) in values {
                            let mut on = filter.is_chosen(facet, value);
                            let response = ui.checkbox(&mut on, format!("{value} ({count})"));
                            design::named(
                                ui,
                                &response,
                                &format!("Filter {}: {value}", facet.label()),
                            );
                            if response.changed() {
                                filter.toggle(facet, value);
                            }
                        }
                    }
                });
        });
    }
}

/// Two rows of challenge ratings: the lowest and the highest. A second click on a bound
/// removes it.
fn challenge_menu(ui: &mut egui::Ui, filter: &mut Filter, values: &[(String, usize)]) {
    ui.set_max_width(design::number_width(ui, "1/8", &Type::Body.font(ui)) * 9.0);
    for (title, high) in [("Lowest", false), ("Highest", true)] {
        ui.label(Type::Small.rich(ui, title).color(colors().muted));
        ui.horizontal_wrapped(|ui| {
            for (value, _) in values {
                let Some(number) = challenge_value(value) else {
                    continue;
                };
                let bound = if high {
                    &mut filter.challenge.1
                } else {
                    &mut filter.challenge.0
                };
                let response = ui.selectable_label(*bound == Some(number), value);
                design::named(ui, &response, &format!("{title} CR {value}"));
                if response.clicked() {
                    *bound = (*bound != Some(number)).then_some(number);
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
