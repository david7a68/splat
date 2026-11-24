//! The run's message footer as data, for a view in any toolkit: which messages show, in which
//! order, in which tone, with which choices; and what a choice does (`press_message`). The egui
//! view draws its typed form (`view::Run::run_messages`) and goes through `press_message` for its
//! presses too.
use super::{
    Command, Run,
    view::{RunPress, STAT_BLOCK_UNREAD, close_saved_words},
};

/// A message's tone, as the design system's `Tone`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageTone {
    Success,
    Info,
    Warning,
    Error,
}

/// One choice under a message's words.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageChoice {
    pub words: &'static str,
    pub press: RunPress,
    /// The choice loses something (the file's changes): it comes last, after `L`.
    pub loses: bool,
    /// The message's close icon, not words.
    pub dismiss: bool,
}

impl MessageChoice {
    fn choice(words: &'static str, press: RunPress) -> Self {
        Self {
            words,
            press,
            loses: false,
            dismiss: false,
        }
    }
    fn dismiss(press: RunPress) -> Self {
        Self {
            words: "Dismiss",
            press,
            loses: false,
            dismiss: true,
        }
    }
}

/// One message of the run's footer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunMessage {
    pub tone: MessageTone,
    pub words: String,
    pub choices: Vec<MessageChoice>,
}

impl RunMessage {
    fn new(tone: MessageTone, words: impl Into<String>) -> Self {
        Self {
            tone,
            words: words.into(),
            choices: Vec::new(),
        }
    }
    fn with(mut self, choice: MessageChoice) -> Self {
        self.choices.push(choice);
        self
    }
}

impl Run {
    /// The run's footer messages, in the order `run_messages` shows them (FI2): the step's
    /// result, a recharge's words, the disk conflict, the run's error, a refused Continue
    /// fight, unread Undo steps, and in setup the carry-over and rest offers.
    pub fn message_list(&self, setup: bool) -> Vec<RunMessage> {
        let mut messages = Vec::new();
        let disk = self.has_disk_conflict();
        let rest = setup && self.rest_filled_saved_hp();
        let used = setup && !rest && self.saved_hp_used();
        if used && self.newest_step_result().is_some() {
            messages.push(
                RunMessage::new(MessageTone::Success, super::SAVED_HP_USED)
                    .with(MessageChoice::choice("Undo", RunPress::UndoStep)),
            );
        }
        if !setup
            && let Some(recharged) = self.recharge_result()
            && self.newest_step_result().is_some()
        {
            messages.push(RunMessage::new(
                MessageTone::Success,
                recharged.words.clone(),
            ));
        }
        if disk {
            messages.push(
                RunMessage::new(MessageTone::Error, super::DISK_CONFLICT)
                    .with(MessageChoice::choice(
                        "Save as a new run",
                        RunPress::SaveAsNew,
                    ))
                    .with(MessageChoice {
                        loses: true,
                        ..MessageChoice::choice(
                            "Replace the file with this run",
                            RunPress::Replace,
                        )
                    }),
            );
        }
        if let Some(error) = self.error.as_deref().filter(|_| !disk) {
            let repair = error.contains(STAT_BLOCK_UNREAD);
            let words = if repair {
                let first = error
                    .split_once(" Its row")
                    .map_or(error, |(first, _)| first);
                format!("{first} Its row still works.")
            } else {
                error.to_owned()
            };
            let mut message = RunMessage::new(MessageTone::Error, words);
            if repair {
                message = message.with(MessageChoice::choice(
                    "Campaign → Packages",
                    RunPress::ToPackages,
                ));
            }
            messages.push(message.with(MessageChoice::dismiss(RunPress::DismissError)));
        }
        if let Some((_, _, refusal)) = self.unend_refused.as_ref().filter(|_| !disk) {
            let tone = match refusal.tone() {
                crate::design::Tone::Error => MessageTone::Error,
                _ => MessageTone::Warning,
            };
            messages.push(
                RunMessage::new(tone, refusal.words().to_string())
                    .with(MessageChoice::dismiss(RunPress::DismissRefusal)),
            );
        }
        if !disk && self.undo_unread {
            messages.push(
                RunMessage::new(MessageTone::Warning, super::UNDO_UNREAD)
                    .with(MessageChoice::dismiss(RunPress::DismissUnread)),
            );
        }
        if setup && let Some(words) = self.carry_missing() {
            messages.push(RunMessage::new(MessageTone::Warning, words));
        }
        if rest {
            messages.push(
                RunMessage::new(MessageTone::Info, super::REST_LOG)
                    .with(MessageChoice::choice("Use the saved HP", RunPress::UseSaved)),
            );
        }
        if setup
            && !rest
            && !used
            && let Some((from, _)) = self.back_to_sheets()
        {
            messages.push(
                RunMessage::new(
                    MessageTone::Info,
                    format!("Party HP carries over from {from}."),
                )
                .with(MessageChoice::choice(
                    "Use the sheets’ HP",
                    RunPress::UseSheets,
                )),
            );
        }
        messages
    }

    /// What a footer choice does, but the step's Undo, which goes by its ticket
    /// (`run_press`, `undo_newest_result`). A command it gives the caller runs as the view's
    /// other commands do.
    pub fn press_message(&mut self, press: RunPress) -> Option<Command> {
        match press {
            RunPress::UndoStep => {
                self.undo_newest_result();
                None
            }
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

    /// The Undo of the newest step's shown result: Undo, while that result is the newest
    /// step's (`newest_step_result` shows only the newest step's). Undo ends the result.
    pub fn undo_newest_result(&mut self) {
        let newest = self.history_steps.last().copied();
        if self
            .newest_step_result()
            .is_some_and(|shown| Some(shown.value().owner.step) == newest)
        {
            self.undo();
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
