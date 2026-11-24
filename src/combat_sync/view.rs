//! The combat sync panel's reads and acts, free of any toolkit, so the egui panel
//! (`CombatSync::show`) and the GPUI Party page (`src/gui/pages/party/`) share them.
use super::CombatSync;

impl CombatSync {
    /// `Sync this sheet with D&D Beyond`: on starts reading the live sheet; off stops.
    pub fn set_enabled(&mut self, enabled: bool) {
        let was = self.enabled;
        self.enabled = enabled;
        if was && !enabled {
            self.stop();
        }
        if !was && enabled {
            self.start();
        }
    }

    /// `Check now`: the next read starts at once.
    pub fn check_now(&mut self) {
        self.refresh_only = true;
        self.next = None;
    }

    /// A live sheet has been read.
    pub fn has_remote(&self) -> bool {
        self.remote.is_some()
    }

    /// The sign-in can edit the live sheet.
    pub fn remote_writable(&self) -> Option<bool> {
        self.remote.as_ref().map(|remote| remote.can_edit)
    }
}
