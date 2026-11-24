//! What the run's header reads (the GPUI header is `gui::pages::run::header`): the current
//! turn's reminders, and the shorter forms of a sheet's summary.
use super::{Phase, Run};

impl Run {
    /// The current turn's reminders, from the effects with a time: all of them whole, their
    /// count, and the whole words for the hover and the accessible name.
    pub fn turn_reminders(&self) -> Option<(String, String, String)> {
        if self.state.phase != Phase::Running {
            return None;
        }
        let p = self.state.participants.get(self.current()?)?;
        let reminders: Vec<String> = p
            .conditions
            .iter()
            .filter_map(super::Condition::reminder)
            .collect();
        if reminders.is_empty() {
            return None;
        }
        let whole = reminders.join(" · ");
        let full = format!("Reminders for {}: {whole}", p.name);
        Some((whole, reminders.len().to_string(), full))
    }
}

/// The sheet-change notice's forms on the notice line, longest first: the whole summary,
/// then its first changed value with the name (`Brannoch Vale: HP 58 → 54`). A count of
/// removed Undo steps stays in both (FJ1, X47): with no room for it, the notice goes below
/// the roster, where it keeps the count too.
pub fn header_sheet_forms(summary: &str) -> [String; 2] {
    let (changes, removed) = summary
        .rsplit_once(" · ")
        .filter(|(_, removed)| removed.ends_with(" removed"))
        .map_or((summary, String::new()), |(changes, removed)| {
            (changes, format!(" · {removed}"))
        });
    let compact = changes
        .split(", ")
        .next()
        .unwrap_or(changes)
        .replace(&format!("{} ", super::SHEET_CHANGED), ": ");
    [summary.to_owned(), format!("{compact}{removed}")]
}
