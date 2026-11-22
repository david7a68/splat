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
        self.removed.clone()
    }

    /// What the last Remove's Undo names (`Undo remove <name>`); empty with no Remove.
    pub fn test_removed_name(&self) -> String {
        self.removed
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

    pub fn test_remeasure(&mut self) {
        self.count_settle();
    }
}
