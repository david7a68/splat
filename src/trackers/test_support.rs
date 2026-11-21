use super::*;

impl Trackers {
    pub fn test_settle(&mut self) {
        self.settle();
    }

    pub fn test_kept(&self) -> Option<(crate::design::ResultOrder, Vec<String>)> {
        let shown = self.results.get(&self.played_owner()?)?;
        Some((shown.order(), shown.value().1.clone()))
    }

    pub fn test_campaign(&self) -> Option<&Campaign> {
        self.campaign.as_ref()
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
        self.revision += 1;
    }
}
