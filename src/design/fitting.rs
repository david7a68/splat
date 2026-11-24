//! The one ordered selector of the closed line recipes (M5): the run's turn heading and the
//! plan row. A recipe is a closed, ordered stream of COMPLETE candidates (the words and any
//! controls together), written once in its owner's code; the selector takes the first legal
//! one that fits. It never builds or cuts words itself.

/// The first candidate of `order` that `legal` allows and `measure` finds a fit for, with that
/// fit. Linear and in order: each legal candidate is measured at most once, and never by
/// bisection, because a later candidate can be wider than an earlier one (R31: dropping a
/// creature can add its name to the reason).
pub fn first_fit<C: Copy, R>(
    order: impl IntoIterator<Item = C>,
    legal: impl Fn(C) -> bool,
    mut measure: impl FnMut(C) -> Option<R>,
) -> Option<(C, R)> {
    order
        .into_iter()
        .filter(|candidate| legal(*candidate))
        .find_map(|candidate| measure(candidate).map(|fit| (candidate, fit)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The selector alone (not evidence for the plan row, whose recipe is a later pull request):
    /// an order whose later candidate is wider than an earlier one. It takes the first that fits,
    /// measures each legal candidate at most once in order, and never skips ahead by width.
    #[test]
    fn the_selector_takes_the_first_fit_in_order() {
        const ORDER: [u32; 5] = [70, 60, 66, 40, 12];
        for room in [80, 65, 61, 45, 10] {
            let mut measured = Vec::new();
            let chosen = first_fit(
                ORDER,
                |_| true,
                |width| {
                    measured.push(width);
                    (width <= room).then_some(())
                },
            );
            let first = ORDER.iter().position(|width| *width <= room);
            assert_eq!(
                chosen.map(|(width, ())| width),
                first.map(|at| ORDER[at]),
                "room {room}"
            );
            assert_eq!(
                measured,
                ORDER[..first.map_or(ORDER.len(), |at| at + 1)].to_vec(),
                "room {room}: each candidate measured once, in order"
            );
        }
        let skip = first_fit(
            ORDER,
            |width| width != 60,
            |width| (width <= 65).then_some(()),
        );
        assert_eq!(skip.map(|(width, ())| width), Some(40));
    }
}
