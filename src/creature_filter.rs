//! The creature filter control: a Filter menu with a submenu for each fact, and a
//! removable chip for each chosen value. Library and the creature search share it.
use splat::content::creature_filter::{Facet, Filter};

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
