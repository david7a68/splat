use super::*;

impl Trackers {
    pub fn test_settle(&mut self) {
        self.settle();
    }

    pub fn test_kept(&self) -> Option<(crate::design::ResultOrder, Vec<String>)> {
        let shown = self.results.get(&self.played_owner()?)?;
        Some((shown.order_for_caller_test(), shown.value().1.clone()))
    }

    pub fn test_has_played_owner(&self) -> bool {
        self.played_owner().is_some()
    }

    pub fn test_result(&self) -> Option<(String, Sentence)> {
        self.result.clone()
    }

    pub fn test_history(&self) -> &HashMap<String, Vec<history::Row>> {
        &self.history
    }

    pub fn test_history_reason(&self) -> Option<(Uuid, Unrecorded)> {
        self.history_reason.clone()
    }

    pub fn test_removed(&self) -> Option<history::Removed> {
        self.removed
            .newest_first(self.campaign.as_ref()?.path.as_path())
            .next()
            .map(|shown| shown.value().clone())
    }

    /// Whether the history footer of `note` draws the last Remove's kept line, with its Undo
    /// (`history::footer` draws exactly this selection).
    pub fn test_removed_drawn(&self, note: Uuid) -> bool {
        self.removed_in(note).is_some()
    }

    /// The ticket of the last Remove's line, as its Undo's press carries it.
    pub fn test_removed_ticket(&self) -> Option<crate::design::results::Ticket> {
        self.removed
            .newest_first(self.campaign.as_ref()?.path.as_path())
            .next()
            .map(|shown| shown.ticket_for_caller_test())
    }

    /// The press of the last Remove's Undo, with its line's ticket.
    pub fn test_undo_remove(&mut self) {
        let ticket = self.test_removed_ticket();
        self.undo_remove(ticket);
    }

    /// What the last Remove's Undo names (`Undo remove <name>`); empty with no Remove.
    pub fn test_removed_name(&self) -> String {
        self.test_removed()
            .as_ref()
            .map(history::Removed::name)
            .unwrap_or_default()
    }

    pub fn test_today(&self) -> &Today {
        &self.today
    }

    pub fn test_problems(&self) -> &[(Option<Uuid>, String)] {
        &self.problems
    }

    pub fn test_typed(&self) -> Option<&str> {
        self.view.typed()
    }

    pub fn test_measured(&self) -> usize {
        self.view.measured
    }

    /// How many times this thread's view made a Due row's parts (layout 11c scope C).
    pub fn test_due_parts_made() -> usize {
        super::view::due_parts_made()
    }
}
