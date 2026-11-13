//! The save row: a draft's state word and quiet `Discard` at the left, `Save` at the right.
//!
//! One function decides each part from one [`DraftState`]: the word ([`DraftState::word`]),
//! Save's weight ([`DraftState::save_weight`]), why Save waits ([`DraftState::save_blocked`]),
//! whether `Discard` shows, and the campaign-switch reason ([`DraftState::lock_reason`]). The
//! caller says what is true of the draft (changed or not, refused and why, changed on disk, a
//! failed Save, who owns the primary); it never picks a word, a weight or a reason.
//! `docs/design-system.md` (Buttons, the save row paragraph) is the rule; this is its one copy.
//!
//! Caller duties left: route `Ctrl+S` through [`DraftState::can_save`], so the key saves exactly
//! when the button can, and show a refusal's or a failure's sentence in the pane's footer.
//!
//! # What does not compile
//!
//! Each case has a twin that compiles and differs only in the marked line.
//!
//! ```
//! use splat::ownership::{Changed, DraftState, Primary, SaveWeight, SaveWord};
//! let draft = DraftState::new(Changed::Yes, Primary::Draft).refused("Another entry has this id.");
//! assert_eq!((draft.word(), draft.save_weight()), (SaveWord::Invalid, SaveWeight::Secondary));
//! ```
//!
//! The word comes from the state; a caller cannot set it, or build a state by hand:
//!
//! ```compile_fail
//! # use splat::ownership::{Changed, DraftState, Primary, SaveWord};
//! let draft = DraftState { word: SaveWord::Unsaved, ..DraftState::new(Changed::Yes, Primary::Draft) }; // <-
//! ```
//!
//! Whether the draft changed is a named state, not a `bool` that can be passed in the wrong
//! place:
//!
//! ```compile_fail
//! # use splat::ownership::{DraftState, Primary};
//! let draft = DraftState::new(true, Primary::Draft); // <- a bare bool
//! ```
use super::{Action, Density, Status, colors, components::status, icons};
use eframe::egui::{self, Ui};
use std::borrow::Cow;

/// Whether the draft differs from the saved text: the same type as a revised value's change.
pub use super::memo::Changed;

/// Save's own refusal of the draft. One field holds its mode and its reason, so the last call
/// decides both.
#[derive(Clone, Debug)]
enum Refusal<'a> {
    None,
    /// `Invalid`, changed or not.
    Always(Cow<'a, str>),
    /// `Invalid` only while the draft has changes (the encounter draft's exception).
    WhenChanged(Cow<'a, str>),
}

/// Who owns the one accent-filled primary action in view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Primary {
    /// This draft's Save, when it would write.
    Draft,
    /// Something else: an open run's `Next turn`, or another draft that was edited last.
    Elsewhere,
}

/// The state word of a save row, in the order the rules check them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SaveWord {
    /// Save's own check refuses the draft, changed or not, or the draft has a fault that Save
    /// does not fix by itself.
    Invalid,
    /// The file changed on disk; the comparison owns the pane.
    ChangedOnDisk,
    /// The disk refused the last Save of this draft.
    NotSaved,
    Unsaved,
    /// The draft on screen was saved and has not changed since.
    Saved,
    NoChanges,
}

impl SaveWord {
    pub fn words(self) -> &'static str {
        match self {
            Self::Invalid => "Invalid",
            Self::ChangedOnDisk => "Changed on disk",
            Self::NotSaved => "Not saved",
            Self::Unsaved => "Unsaved",
            Self::Saved => "Saved",
            Self::NoChanges => "No changes",
        }
    }

    pub fn status(self) -> Status<'static> {
        let d = colors().document;
        let (icon, color) = match self {
            Self::Invalid | Self::NotSaved => (icons::ERROR, d.invalid),
            Self::ChangedOnDisk => (icons::WARN, d.external),
            Self::Unsaved => (icons::DRAFT, d.unsaved),
            Self::Saved => (icons::OK, d.saved),
            Self::NoChanges => (icons::OK, d.clean),
        };
        Status {
            icon,
            word: self.words(),
            color,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SaveWeight {
    Primary,
    Secondary,
}

/// Why Save waits while the file changed on disk.
pub const CHANGED_ON_DISK: &str = "Choose what to do with the changed file first.";
/// Why Save waits with nothing to save. It belongs only to Save's hover text and accessible
/// description, never to the row.
pub const NO_CHANGES: &str = "No changes to save.";
/// The campaign-switch reason of a draft whose Save would write.
pub const SAVE_OR_DISCARD_TO_SWITCH: &str = "Save or discard to switch campaign.";
/// The campaign-switch reason of a draft whose Save waits: Discard is the one way out.
pub const DISCARD_TO_SWITCH: &str = "Discard to switch campaign.";

/// What is true of one draft. See the module comment.
#[derive(Clone, Debug)]
pub struct DraftState<'a> {
    changed: Changed,
    primary: Primary,
    refused: Refusal<'a>,
    invalid: bool,
    on_disk: bool,
    not_saved: bool,
    saved: bool,
    locks_campaign: bool,
}

impl<'a> DraftState<'a> {
    pub fn new(changed: Changed, primary: Primary) -> Self {
        Self {
            changed,
            primary,
            refused: Refusal::None,
            invalid: false,
            on_disk: false,
            not_saved: false,
            saved: false,
            locks_campaign: false,
        }
    }

    /// Save's own check refuses the draft: `Invalid`, changed or not; Save waits with this reason
    /// and is not primary.
    pub fn refused(mut self, why: impl Into<Cow<'a, str>>) -> Self {
        self.refused = Refusal::Always(why.into());
        self
    }

    /// The encounter draft's exception to [`Self::refused`]: it says `Invalid` only while it has
    /// changes, because a clean one has nothing for Save to refuse, and its rows name the faults.
    pub fn refused_when_changed(mut self, why: impl Into<Cow<'a, str>>) -> Self {
        self.refused = Refusal::WhenChanged(why.into());
        self
    }

    /// The draft or its file has a fault that is not Save's refusal (the file cannot be read, a
    /// repair's problem): `Invalid`. Save is not blocked by it.
    pub fn invalid(mut self) -> Self {
        self.invalid = true;
        self
    }

    /// The file changed on disk while the draft was open.
    pub fn changed_on_disk(mut self) -> Self {
        self.on_disk = true;
        self
    }

    /// The disk refused the last Save of this draft. Save stays open to try again, but is not
    /// primary.
    pub fn not_saved(mut self) -> Self {
        self.not_saved = true;
        self
    }

    /// The draft on screen was just saved.
    pub fn just_saved(mut self) -> Self {
        self.saved = true;
        self
    }

    /// This draft locks the campaign switch while it has changes.
    pub fn locks_campaign(mut self) -> Self {
        self.locks_campaign = true;
        self
    }

    fn changed(&self) -> bool {
        self.changed == Changed::Yes
    }

    fn refusal(&self) -> Option<&str> {
        match &self.refused {
            Refusal::None => None,
            Refusal::Always(why) => Some(why.as_ref()),
            Refusal::WhenChanged(why) => self.changed().then_some(why.as_ref()),
        }
    }

    pub fn word(&self) -> SaveWord {
        if self.refusal().is_some() || self.invalid {
            SaveWord::Invalid
        } else if self.on_disk {
            SaveWord::ChangedOnDisk
        } else if self.not_saved {
            SaveWord::NotSaved
        } else if self.changed() {
            SaveWord::Unsaved
        } else if self.saved {
            SaveWord::Saved
        } else {
            SaveWord::NoChanges
        }
    }

    /// Why Save waits, or `None` when Save would run.
    pub fn save_blocked(&self) -> Option<&str> {
        if self.on_disk {
            Some(CHANGED_ON_DISK)
        } else if !self.changed() {
            Some(NO_CHANGES)
        } else {
            self.refusal()
        }
    }

    /// Whether Save (and `Ctrl+S`) would run now.
    pub fn can_save(&self) -> bool {
        self.save_blocked().is_none()
    }

    /// Save is primary while the draft has changes that it can save, the last Save did not fail,
    /// and nothing else owns the primary; secondary otherwise.
    pub fn save_weight(&self) -> SaveWeight {
        if self.can_save() && !self.not_saved && self.primary == Primary::Draft {
            SaveWeight::Primary
        } else {
            SaveWeight::Secondary
        }
    }

    /// `Discard` shows while the draft has changes and the comparison does not own the pane.
    pub fn discard_shown(&self) -> bool {
        self.changed() && !self.on_disk
    }

    /// The campaign-switch reason after `Discard`, for a draft that locks the switch.
    pub fn lock_reason(&self) -> Option<&'static str> {
        (self.locks_campaign && self.changed()).then(|| {
            if self.can_save() {
                SAVE_OR_DISCARD_TO_SWITCH
            } else {
                DISCARD_TO_SWITCH
            }
        })
    }
}

/// What the DM pressed in a save row.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[must_use]
pub struct SaveRow {
    pub save: bool,
    pub discard: bool,
}

/// The save row of `state`, as wide as the `Ui`: the word, `Discard` and any lock reason at the
/// left, `save` (its words, such as `Save` or `Save changes`) at the right with `Ctrl+S`.
pub fn save_row(ui: &mut Ui, state: &DraftState<'_>, save: &str) -> SaveRow {
    let mut pressed = SaveRow::default();
    ui.horizontal(|ui| {
        ui.set_min_height(Density::Scan.control());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            pressed.save = match state.save_weight() {
                SaveWeight::Primary => Action::primary(save),
                SaveWeight::Secondary => Action::secondary(save),
            }
            .density(Density::Read)
            .blocked(state.save_blocked())
            .key("Ctrl+S")
            .show(ui)
            .clicked();
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                status(ui, state.word().status());
                if state.discard_shown() {
                    pressed.discard = Action::quiet("Discard").show(ui).clicked();
                }
                if let Some(reason) = state.lock_reason() {
                    super::blocked_reason(ui, Some(reason));
                }
            });
        });
    });
    pressed
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The rule as a table: each row is one state and the word, weight, reason and Discard it
    /// gives. Rows name the round that ruled them.
    #[test]
    fn one_function_decides_the_word_the_weight_and_the_reason() {
        let yes = || DraftState::new(Changed::Yes, Primary::Draft);
        let no = || DraftState::new(Changed::No, Primary::Draft);
        let rows = [
            (
                "clean",
                no(),
                SaveWord::NoChanges,
                SaveWeight::Secondary,
                Some(NO_CHANGES),
                false,
            ),
            (
                "changed",
                yes(),
                SaveWord::Unsaved,
                SaveWeight::Primary,
                None,
                true,
            ),
            (
                "saved",
                no().just_saved(),
                SaveWord::Saved,
                SaveWeight::Secondary,
                Some(NO_CHANGES),
                false,
            ),
            // R27 FN5: Invalid follows the refusal, changed or not.
            (
                "refused",
                yes().refused("Bad id."),
                SaveWord::Invalid,
                SaveWeight::Secondary,
                Some("Bad id."),
                true,
            ),
            (
                "refused clean",
                no().refused("Bad id."),
                SaveWord::Invalid,
                SaveWeight::Secondary,
                Some(NO_CHANGES),
                false,
            ),
            // The encounter draft's exception.
            (
                "encounter clean",
                no().refused_when_changed("Add a creature first."),
                SaveWord::NoChanges,
                SaveWeight::Secondary,
                Some(NO_CHANGES),
                false,
            ),
            (
                "encounter changed",
                yes().refused_when_changed("Add a creature first."),
                SaveWord::Invalid,
                SaveWeight::Secondary,
                Some("Add a creature first."),
                true,
            ),
            // A fault that Save does not refuse: Invalid, and Save still writes.
            (
                "fault",
                yes().invalid(),
                SaveWord::Invalid,
                SaveWeight::Primary,
                None,
                true,
            ),
            (
                "on disk",
                yes().changed_on_disk(),
                SaveWord::ChangedOnDisk,
                SaveWeight::Secondary,
                Some(CHANGED_ON_DISK),
                false,
            ),
            // World repair: a failed Save stays open to try again, but is not primary.
            (
                "not saved",
                yes().not_saved(),
                SaveWord::NotSaved,
                SaveWeight::Secondary,
                None,
                true,
            ),
            // R22, R23: an open run's Next turn stays the only primary.
            (
                "run",
                DraftState::new(Changed::Yes, Primary::Elsewhere),
                SaveWord::Unsaved,
                SaveWeight::Secondary,
                None,
                true,
            ),
            // Review of PR 301: the last refusal decides its mode too.
            (
                "when changed, then always",
                no().refused_when_changed("Add a creature first.")
                    .refused("Bad id."),
                SaveWord::Invalid,
                SaveWeight::Secondary,
                Some(NO_CHANGES),
                false,
            ),
            (
                "refused on disk",
                yes().refused("Bad id.").changed_on_disk(),
                SaveWord::Invalid,
                SaveWeight::Secondary,
                Some(CHANGED_ON_DISK),
                false,
            ),
        ];
        for (name, state, word, weight, blocked, discard) in rows {
            assert_eq!(
                (
                    state.word(),
                    state.save_weight(),
                    state.save_blocked(),
                    state.discard_shown()
                ),
                (word, weight, blocked, discard),
                "{name}"
            );
            assert_eq!(
                state.can_save(),
                blocked.is_none(),
                "{name}: Ctrl+S saves exactly when Save can"
            );
        }
    }

    #[test]
    fn the_lock_reason_names_save_only_while_save_would_run() {
        let state = |changed| DraftState::new(changed, Primary::Draft).locks_campaign();
        assert_eq!(state(Changed::No).lock_reason(), None);
        assert_eq!(
            state(Changed::Yes).lock_reason(),
            Some(SAVE_OR_DISCARD_TO_SWITCH)
        );
        assert_eq!(
            state(Changed::Yes).refused("Bad id.").lock_reason(),
            Some(DISCARD_TO_SWITCH)
        );
        assert_eq!(
            DraftState::new(Changed::Yes, Primary::Draft).lock_reason(),
            None,
            "a draft that does not lock the switch says nothing"
        );
    }

    /// The row draws what the state decides: the word, Discard only with changes, and Save with
    /// its weight's type. One press reaches only its own control.
    #[test]
    fn the_row_draws_the_state() {
        use egui_kittest::kittest::Queryable;
        for (state, word, discard) in [
            (
                DraftState::new(Changed::Yes, Primary::Draft),
                "Unsaved",
                true,
            ),
            (
                DraftState::new(Changed::No, Primary::Draft),
                "No changes",
                false,
            ),
        ] {
            let pressed = std::cell::Cell::new(SaveRow::default());
            let mut harness = egui_kittest::Harness::builder()
                .with_size(egui::vec2(600.0, 80.0))
                .build_ui(|ui| {
                    let row = save_row(ui, &state, "Save");
                    if row.save || row.discard {
                        pressed.set(row);
                    }
                });
            crate::design::theme::install(&harness.ctx);
            harness.run();
            assert!(harness.query_by_label_contains(word).is_some(), "{word}");
            assert_eq!(
                harness.query_by_label("Discard").is_some(),
                discard,
                "{word}"
            );
            if discard {
                harness.get_by_label("Discard").click();
                harness.run();
                assert_eq!(
                    pressed.get(),
                    SaveRow {
                        save: false,
                        discard: true
                    }
                );
                harness.get_by_label("Save").click();
                harness.run();
                assert_eq!(
                    pressed.get(),
                    SaveRow {
                        save: true,
                        discard: false
                    }
                );
            }
        }
    }
}
