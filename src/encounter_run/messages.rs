//! What a choice of the run's message footer does (`press_message`), for any view: the footer
//! itself is `view::Run::run_messages` (typed lines, which the GPUI view reads with
//! `Messages::into_plain`), and both views press through `run_press`, which calls this.
use super::{
    Command, Run,
    view::{RunPress, close_saved_words},
};

impl Run {
    /// What a footer choice does, but the step's Undo, which goes by its ticket (`run_press`
    /// undoes it before it calls this). A command it gives the caller runs as the view's other
    /// commands do.
    pub fn press_message(&mut self, press: RunPress) -> Option<Command> {
        match press {
            RunPress::UndoStep => None,
            RunPress::SaveAsNew => {
                if let Err(error) = self.save_as_new_run() {
                    self.error = Some(error);
                }
                None
            }
            RunPress::Replace => {
                if let Err(error) = self.keep_this_run() {
                    self.error = Some(error);
                }
                None
            }
            RunPress::DismissError => {
                self.error = None;
                None
            }
            RunPress::DismissRefusal => {
                self.unend_refused = None;
                None
            }
            RunPress::ToPackages => {
                self.wants_packages = true;
                None
            }
            RunPress::DismissUnread => {
                self.undo_unread = false;
                None
            }
            RunPress::UseSaved => {
                if let Err(error) = self.use_saved_hp() {
                    self.error = Some(error);
                }
                None
            }
            RunPress::UseSheets => self.back_to_sheets().map(|(_, back)| back),
        }
    }

    /// What Begin combat waits for, if anything: an unread roster, HP, or initiative.
    pub fn setup_blocked(&self) -> Option<String> {
        let missing_initiative = self
            .state
            .participants
            .iter()
            .filter(|p| p.initiative.is_none() && p.present)
            .count();
        let missing_hp = self
            .state
            .participants
            .iter()
            .filter(|p| {
                !matches!((p.hp, p.max_hp), (Some(hp), Some(max)) if max > 0 && hp >= 0 && hp <= max)
            })
            .count();
        if let Some(reason) = self.roster_blocked() {
            Some(reason.to_owned())
        } else if missing_hp > 0 {
            Some(format!("{missing_hp} need valid HP."))
        } else if missing_initiative > 0 {
            Some(format!("{missing_initiative} need initiative."))
        } else {
            None
        }
    }

    /// The close step's note about what stays saved, in each phase.
    pub fn close_note(&self) -> String {
        match self.state.phase {
            super::Phase::Setup if self.untouched_setup() => {
                "Nothing was done, so no run is kept.".to_owned()
            }
            super::Phase::Setup => "The run stays saved, in setup.".to_owned(),
            super::Phase::Running => {
                close_saved_words(self.syncs.values().any(|sync| sync.enabled))
            }
            super::Phase::Ended { .. } => "The run stays saved.".to_owned(),
        }
    }
}
