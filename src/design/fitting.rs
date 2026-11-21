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
