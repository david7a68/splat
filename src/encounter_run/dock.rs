//! What the run's dock reads (the GPUI dock is `gui::pages::run::dock`): the last roll beside
//! the dice field, and the words while several rows are targets.
use super::Run;
use crate::design::icons;

impl Run {
    /// The dice field's last roll: its total and its Log line.
    /// The last roll beside the dice, as its total, its hover and its accessible name: a
    /// recharge's while its act is the newest (X4), named by its participant and effect; else
    /// the dice field's.
    pub fn dock_roll(&self) -> Option<(i64, String, String)> {
        match (self.recharge_result(), &self.dock_roll) {
            (Some(recharged), _) => Some((
                recharged.die,
                recharged.words.clone(),
                recharged.name.clone(),
            )),
            (None, rolled) => rolled.as_ref().map(|rolled| {
                (
                    rolled.total,
                    rolled.line(),
                    format!("Rolled {}", rolled.total),
                )
            }),
        }
    }
}

/// The dock's words while several rows are targets: the mark the rows paint, and what it does.
pub fn target_hint() -> String {
    format!(
        "Press a target’s {} to give it half, rounded down.",
        icons::OK
    )
}
