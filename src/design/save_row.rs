//! The save row: a draft's state word and quiet `Discard` at the left, `Save` at the right.
//!
//! One function decides each part from one [`DraftState`]: the word ([`DraftState::word`]),
//! Save's weight ([`DraftState::save_weight`]), why Save waits ([`DraftState::save_blocked`]),
//! whether `Discard` shows, and the campaign-switch reason ([`DraftState::lock_reason`]). The
//! caller says what is true of the draft (new or saved, changed or not, what Save's check found
//! and why, changed on disk, a failed Save, who owns the primary); it never picks a word, a
//! weight, a tone or a reason.
//!
//! The draft's origin ([`DraftOrigin`]) and Save's check ([`Diagnosis`]) are facts, and the word
//! comes from them (R37, Q-m): a new draft that lacks a required part says `New` in a neutral
//! tone, and a saved draft that an edit left without one is a refused change, `Invalid` in the
//! warning tone of its cause. A refused change and unreadable data say `Invalid` in the error
//! tone. An empty list alone never says `New`: only the origin does, and every draft states it.
//! When a draft has several faults, one rule ranks them, whatever order the caller reports them
//! in: unreadable data, then a refused change, then a missing part.
//! `docs/design-system.md` (Buttons, the save row paragraph) is the rule; this is its one copy.
//!
//! Caller duties left: route `Ctrl+S` through [`DraftState::can_save`], so the key saves exactly
//! when the button can, and show a refusal's or a failure's sentence in the pane's footer. A
//! caller that paints a word itself (`SaveWord::status`) holds the rule only when it takes that
//! word from [`DraftState::word`].
//!
//! # What does not compile
//!
//! Each case has a twin that compiles and differs only in the marked line.
//!
//! ```
//! use splat::ownership::{Changed, Diagnosis, DraftOrigin, DraftState, Primary, SaveWeight, SaveWord};
//! let draft = DraftState::new(DraftOrigin::New, Changed::Yes, Primary::Draft) // <-
//!     .check(Diagnosis::incomplete("Add a creature first.")); // <-
//! assert_eq!((draft.word(), draft.save_weight()), (SaveWord::New, SaveWeight::Secondary));
//! ```
//!
//! The word comes from the state; a caller cannot set it, or build a state by hand:
//!
//! ```compile_fail
//! # use splat::ownership::{Changed, DraftOrigin, DraftState, Primary, SaveWord};
//! let draft = DraftState {
//!     word: SaveWord::Unsaved, // <-
//!     ..DraftState::new(DraftOrigin::New, Changed::Yes, Primary::Draft)
//! };
//! ```
//!
//! Every draft states its origin; none is assumed:
//!
//! ```compile_fail
//! # use splat::ownership::{Changed, DraftState, Primary};
//! let draft = DraftState::new(Changed::Yes, Primary::Draft); // <- no origin
//! ```
//!
//! Whether the draft changed is a named state, not a `bool` that can be passed in the wrong
//! place:
//!
//! ```compile_fail
//! # use splat::ownership::{DraftOrigin, DraftState, Primary};
//! let draft = DraftState::new(DraftOrigin::New, true, Primary::Draft); // <- a bare bool
//! ```
//!
//! Save's check is a typed diagnosis: text alone is no refusal, and a diagnosis is made only by
//! its named constructors:
//!
//! ```compile_fail
//! # use splat::ownership::{Changed, DraftOrigin, DraftState, Primary};
//! let draft = DraftState::new(DraftOrigin::New, Changed::Yes, Primary::Draft)
//!     .check("Add a creature first."); // <- text
//! ```
//!
//! ```compile_fail
//! # use splat::ownership::Diagnosis;
//! let found = Diagnosis { refused: None, incomplete: None, unreadable: true }; // <- by hand
//! ```
use super::{Action, Density, Status, Tone, colors, components::status, icons};
use eframe::egui::{self, Ui};
use std::borrow::Cow;

/// Whether the draft differs from the saved text: the same type as a revised value's change.
pub use super::memo::Changed;

/// Whether the draft's file was ever saved: a fact its editor stores with the draft, not one
/// read from the draft's contents or from whether a file is on disk now. It becomes `Saved` at
/// the first successful Save; a failed write, a lost file or an accepted recovery leaves it as
/// it was; Discard and Undo carry it with the draft.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftOrigin {
    /// Never saved: the file was never written.
    New,
    /// Read from, or saved to, its file at least once.
    Saved,
}

/// What Save's own check found wrong with a draft: the typed validation result. It is made only
/// by its named constructors and joined with [`Diagnosis::and`]. It can hold all three kinds at
/// once, and the save row ranks them by one rule, in any order of report: unreadable data, then
/// a refused change, then a missing part.
///
/// - [`Diagnosis::incomplete`]: a required part is missing (`Add a creature first.`). On a new
///   draft this is its normal way to start, `New`; on a saved draft an edit removed it, a
///   refused change. Save waits with the reason.
/// - [`Diagnosis::refused`]: a change that cannot stand (`Another entry has this id.`):
///   `Invalid`, and Save waits with the reason.
/// - [`Diagnosis::unreadable`]: the draft or its file cannot be read (a repair's problem):
///   `Invalid`, but Save is not blocked by it, since Save can repair it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnosis<'a> {
    refused: Option<Cow<'a, str>>,
    incomplete: Option<Cow<'a, str>>,
    unreadable: bool,
}

impl<'a> Diagnosis<'a> {
    /// Nothing wrong.
    pub fn valid() -> Self {
        Self {
            refused: None,
            incomplete: None,
            unreadable: false,
        }
    }

    pub fn incomplete(why: impl Into<Cow<'a, str>>) -> Self {
        Self {
            incomplete: Some(why.into()),
            ..Self::valid()
        }
    }

    pub fn refused(why: impl Into<Cow<'a, str>>) -> Self {
        Self {
            refused: Some(why.into()),
            ..Self::valid()
        }
    }

    pub fn unreadable() -> Self {
        Self {
            unreadable: true,
            ..Self::valid()
        }
    }

    /// Both diagnoses. Each kind keeps its own reason (the later report's, when both name one),
    /// so no kind hides another, and the rank does not depend on the order of the calls.
    pub fn and(self, other: Self) -> Self {
        Self {
            refused: other.refused.or(self.refused),
            incomplete: other.incomplete.or(self.incomplete),
            unreadable: self.unreadable || other.unreadable,
        }
    }

    /// The reason Save waits, by the rank: a refused change before a missing part. Unreadable
    /// data does not block Save.
    pub fn blocking(&self) -> Option<&str> {
        self.refused.as_deref().or(self.incomplete.as_deref())
    }

    /// Whether the one fault is a missing part (no refused change, no unreadable data).
    pub fn only_incomplete(&self) -> bool {
        self.incomplete.is_some() && self.refused.is_none() && !self.unreadable
    }

    pub fn into_owned(self) -> Diagnosis<'static> {
        Diagnosis {
            refused: self.refused.map(|why| Cow::Owned(why.into_owned())),
            incomplete: self.incomplete.map(|why| Cow::Owned(why.into_owned())),
            unreadable: self.unreadable,
        }
    }
}

/// Why a save row says `Invalid`. It gives the word's tone, so `Invalid` has the tone of its
/// cause and no caller chooses one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvalidCause {
    /// A saved draft lost a required part: the warning tone.
    Incomplete,
    /// Save refuses a change: the error tone.
    Refused,
    /// The draft or its file cannot be read: the error tone.
    Unreadable,
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
    /// does not fix by itself. Its tone is its cause's.
    Invalid(InvalidCause),
    /// A new draft that lacks a required part: nothing is wrong yet (R37).
    New,
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
            Self::Invalid(_) => "Invalid",
            Self::New => "New",
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
            Self::Invalid(InvalidCause::Incomplete) => Tone::Warning.parts(),
            Self::Invalid(InvalidCause::Refused | InvalidCause::Unreadable) | Self::NotSaved => {
                (icons::ERROR, d.invalid)
            }
            Self::New => (icons::DRAFT, d.clean),
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
    origin: DraftOrigin,
    changed: Changed,
    primary: Primary,
    /// What Save's check found, changed or not.
    found: Diagnosis<'a>,
    /// What counts only while the draft has changes (the encounter draft's exception).
    found_when_changed: Diagnosis<'a>,
    on_disk: bool,
    not_saved: bool,
    saved: bool,
    locks_campaign: bool,
}

impl<'a> DraftState<'a> {
    /// A draft, with the origin its editor stores.
    pub fn new(origin: DraftOrigin, changed: Changed, primary: Primary) -> Self {
        Self {
            origin,
            changed,
            primary,
            found: Diagnosis::valid(),
            found_when_changed: Diagnosis::valid(),
            on_disk: false,
            not_saved: false,
            saved: false,
            locks_campaign: false,
        }
    }

    /// What Save's own check found, changed or not. It joins what was reported before
    /// ([`Diagnosis::and`]); the word comes from the rank and the origin.
    pub fn check(mut self, found: Diagnosis<'a>) -> Self {
        self.found = self.found.and(found);
        self
    }

    /// The encounter draft's exception to [`Self::check`]: what Save's check found counts only
    /// while the draft has changes, because a clean one has nothing for Save to refuse, and its
    /// rows name the faults.
    pub fn check_when_changed(mut self, found: Diagnosis<'a>) -> Self {
        self.found_when_changed = self.found_when_changed.and(found);
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

    /// One kind's report: the one that holds changed or not, then the one that holds only
    /// while the draft has changes.
    fn kind<'s>(&'s self, of: impl Fn(&'s Diagnosis<'a>) -> Option<&'s str>) -> Option<&'s str> {
        of(&self.found).or_else(|| {
            self.changed()
                .then(|| of(&self.found_when_changed))
                .flatten()
        })
    }

    fn unreadable(&self) -> bool {
        self.found.unreadable || (self.changed() && self.found_when_changed.unreadable)
    }

    fn refused(&self) -> Option<&str> {
        self.kind(|found| found.refused.as_deref())
    }

    fn incomplete(&self) -> Option<&str> {
        self.kind(|found| found.incomplete.as_deref())
    }

    /// Why Save's check blocks Save, by the rank: a refused change before a missing part.
    fn refusal(&self) -> Option<&str> {
        self.refused().or_else(|| self.incomplete())
    }

    pub fn word(&self) -> SaveWord {
        if self.unreadable() {
            SaveWord::Invalid(InvalidCause::Unreadable)
        } else if self.refused().is_some() {
            SaveWord::Invalid(InvalidCause::Refused)
        } else if self.incomplete().is_some() {
            match self.origin {
                DraftOrigin::Saved => SaveWord::Invalid(InvalidCause::Incomplete),
                DraftOrigin::New => SaveWord::New,
            }
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
    save_row_with(ui, state, save, None)
}

/// Why other actions of the pane wait for this draft (M2, the reader toolbar's draft reason):
/// a long and a short whole form, both the toolbar part's own words, and whether it holds.
#[derive(Clone, Copy, Debug)]
pub(in crate::design) struct RowReason<'r> {
    pub long: &'r str,
    pub short: &'r str,
    pub locked: bool,
}

/// `save_row`, with `reason` in the room between the left parts and Save: the long form when it
/// fits whole, else the short one, measured before paint from the fonts with `Discard` and the
/// widest word, so the choice does not change when the draft becomes dirty. Painted while it
/// holds; nothing while open. Only a row too narrow for the short form keeps a slot under it,
/// in both states (R29), so nothing moves while the DM types.
pub(in crate::design) fn save_row_with(
    ui: &mut Ui,
    state: &DraftState<'_>,
    save: &str,
    reason: Option<RowReason<'_>>,
) -> SaveRow {
    let save_action = || {
        match state.save_weight() {
            SaveWeight::Primary => Action::primary(save),
            SaveWeight::Secondary => Action::secondary(save),
        }
        .density(Density::Read)
        .blocked(state.save_blocked())
        .key("Ctrl+S")
    };
    let gap = ui.spacing().item_spacing.x;
    let small = super::Type::Small.font(ui);
    let on_line = reason.and_then(|reason| {
        // The widest word and its icon, whatever the draft holds.
        let word = [
            SaveWord::Invalid(InvalidCause::Incomplete),
            SaveWord::Invalid(InvalidCause::Refused),
            SaveWord::Invalid(InvalidCause::Unreadable),
            SaveWord::New,
            SaveWord::ChangedOnDisk,
            SaveWord::NotSaved,
            SaveWord::Unsaved,
            SaveWord::Saved,
            SaveWord::NoChanges,
        ]
        .into_iter()
        .map(|word| {
            let status = word.status();
            super::number_width(ui, &format!("{} {}", status.icon, status.word), &small)
        })
        .fold(0.0, f32::max);
        let lock = state
            .lock_reason()
            .map_or(0.0, |words| super::number_width(ui, words, &small) + gap);
        let room = ui.available_width()
            - save_action().natural_width(ui)
            - word
            - Action::quiet("Discard").natural_width(ui)
            - lock
            - 4.0 * gap;
        let fits = |words: &str| super::number_width(ui, words, &small) <= room;
        if fits(reason.long) {
            Some(reason.long)
        } else if fits(reason.short) {
            Some(reason.short)
        } else {
            None
        }
    });
    let mut pressed = SaveRow::default();
    ui.horizontal(|ui| {
        ui.set_min_height(Density::Scan.control());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            pressed.save = save_action().show(ui).clicked();
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                status(ui, state.word().status());
                if state.discard_shown() {
                    pressed.discard = Action::quiet("Discard").show(ui).clicked();
                }
                if let Some(reason) = state.lock_reason() {
                    super::blocked_reason(ui, Some(reason));
                }
                if let Some(reason) = reason
                    && reason.locked
                    && let Some(words) = on_line
                {
                    let response =
                        ui.label(super::Type::Small.rich(ui, words).color(colors().muted));
                    if words != reason.long {
                        super::named(ui, &response, reason.long);
                        let _ = response.on_hover_text(reason.long);
                    }
                }
            });
        });
    });
    // Too narrow for the short form: a slot under the row, its room kept while open.
    if let Some(reason) = reason
        && on_line.is_none()
    {
        if reason.locked {
            super::blocked_reason(ui, Some(reason.long));
        } else {
            ui.scope_builder(egui::UiBuilder::new().invisible(), |ui| {
                super::blocked_reason(ui, Some(reason.long));
            });
        }
    }
    pressed
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The rule as a table: each row is one state and the word, weight, reason and Discard it
    /// gives. Rows name the round that ruled them.
    #[test]
    fn one_function_decides_the_word_the_weight_and_the_reason() {
        let yes = || DraftState::new(DraftOrigin::Saved, Changed::Yes, Primary::Draft);
        let no = || DraftState::new(DraftOrigin::Saved, Changed::No, Primary::Draft);
        let new = || DraftState::new(DraftOrigin::New, Changed::Yes, Primary::Draft);
        let incomplete = || Diagnosis::incomplete("Add a creature first.");
        let rows = [
            // R37, Q-m: a new draft that lacks a required part is New, and Save waits with its
            // reason.
            (
                "new incomplete",
                new().check(incomplete()),
                SaveWord::New,
                SaveWeight::Secondary,
                Some("Add a creature first."),
                true,
            ),
            // R37: a saved draft that an edit left without one is a refused change.
            (
                "saved incomplete",
                yes().check(incomplete()),
                SaveWord::Invalid(InvalidCause::Incomplete),
                SaveWeight::Secondary,
                Some("Add a creature first."),
                true,
            ),
            // The origin alone does not make a draft New: a refused change of a new draft is
            // Invalid, and a complete new draft is Unsaved.
            (
                "new refused",
                new()
                    .check(incomplete())
                    .check(Diagnosis::refused("Bad id.")),
                SaveWord::Invalid(InvalidCause::Refused),
                SaveWeight::Secondary,
                Some("Bad id."),
                true,
            ),
            (
                "new complete",
                new(),
                SaveWord::Unsaved,
                SaveWeight::Primary,
                None,
                true,
            ),
            // One rank, in either order of report: a refused change before a missing part.
            (
                "incomplete, then refused",
                new()
                    .check(incomplete())
                    .check(Diagnosis::refused("Bad id.")),
                SaveWord::Invalid(InvalidCause::Refused),
                SaveWeight::Secondary,
                Some("Bad id."),
                true,
            ),
            (
                "refused, then incomplete",
                new()
                    .check(Diagnosis::refused("Bad id."))
                    .check(incomplete()),
                SaveWord::Invalid(InvalidCause::Refused),
                SaveWeight::Secondary,
                Some("Bad id."),
                true,
            ),
            (
                "refused and incomplete in one diagnosis",
                new().check(incomplete().and(Diagnosis::refused("Bad id."))),
                SaveWord::Invalid(InvalidCause::Refused),
                SaveWeight::Secondary,
                Some("Bad id."),
                true,
            ),
            // Unreadable data ranks first, but does not block Save; a blocking refusal stays.
            (
                "unreadable and refused",
                yes()
                    .check(Diagnosis::refused("Bad id."))
                    .check(Diagnosis::unreadable()),
                SaveWord::Invalid(InvalidCause::Unreadable),
                SaveWeight::Secondary,
                Some("Bad id."),
                true,
            ),
            (
                "unreadable, then refused",
                yes()
                    .check(Diagnosis::unreadable())
                    .check(Diagnosis::refused("Bad id.")),
                SaveWord::Invalid(InvalidCause::Unreadable),
                SaveWeight::Secondary,
                Some("Bad id."),
                true,
            ),
            (
                "new unreadable",
                new().check(incomplete()).check(Diagnosis::unreadable()),
                SaveWord::Invalid(InvalidCause::Unreadable),
                SaveWeight::Secondary,
                Some("Add a creature first."),
                true,
            ),
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
                yes().check(Diagnosis::refused("Bad id.")),
                SaveWord::Invalid(InvalidCause::Refused),
                SaveWeight::Secondary,
                Some("Bad id."),
                true,
            ),
            (
                "refused clean",
                no().check(Diagnosis::refused("Bad id.")),
                SaveWord::Invalid(InvalidCause::Refused),
                SaveWeight::Secondary,
                Some(NO_CHANGES),
                false,
            ),
            // The encounter draft's exception.
            (
                "encounter clean",
                no().check_when_changed(Diagnosis::refused("Bad roster.")),
                SaveWord::NoChanges,
                SaveWeight::Secondary,
                Some(NO_CHANGES),
                false,
            ),
            (
                "encounter changed",
                yes().check_when_changed(Diagnosis::refused("Bad roster.")),
                SaveWord::Invalid(InvalidCause::Refused),
                SaveWeight::Secondary,
                Some("Bad roster."),
                true,
            ),
            // R37 in the encounter editor: a new group with only its name, then a saved group
            // an edit emptied.
            (
                "encounter new group",
                new().check_when_changed(incomplete()),
                SaveWord::New,
                SaveWeight::Secondary,
                Some("Add a creature first."),
                true,
            ),
            (
                "encounter emptied group",
                yes().check_when_changed(incomplete()),
                SaveWord::Invalid(InvalidCause::Incomplete),
                SaveWeight::Secondary,
                Some("Add a creature first."),
                true,
            ),
            // A fault that Save does not refuse: Invalid, and Save still writes.
            (
                "fault",
                yes().check(Diagnosis::unreadable()),
                SaveWord::Invalid(InvalidCause::Unreadable),
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
                DraftState::new(DraftOrigin::Saved, Changed::Yes, Primary::Elsewhere),
                SaveWord::Unsaved,
                SaveWeight::Secondary,
                None,
                true,
            ),
            // Review of PR 301: a refusal that holds changed or not still holds on a clean draft,
            // whatever was reported only for changes.
            (
                "when changed, then always",
                no().check_when_changed(Diagnosis::refused("Bad roster."))
                    .check(Diagnosis::refused("Bad id.")),
                SaveWord::Invalid(InvalidCause::Refused),
                SaveWeight::Secondary,
                Some(NO_CHANGES),
                false,
            ),
            (
                "refused on disk",
                yes().check(Diagnosis::refused("Bad id.")).changed_on_disk(),
                SaveWord::Invalid(InvalidCause::Refused),
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

    /// `Invalid` takes its cause's tone; `New` is no fault (R37).
    #[test]
    fn the_word_takes_its_causes_tone() {
        let warning = Tone::Warning.parts();
        let error = Tone::Error.parts();
        let tone = |word: SaveWord| (word.status().icon, word.status().color);
        assert_eq!(tone(SaveWord::Invalid(InvalidCause::Incomplete)), warning);
        assert_eq!(tone(SaveWord::Invalid(InvalidCause::Refused)).0, error.0);
        assert_eq!(tone(SaveWord::Invalid(InvalidCause::Unreadable)).0, error.0);
        for fault in [warning, error] {
            assert_ne!(tone(SaveWord::New).0, fault.0, "New has no fault icon");
            assert_ne!(tone(SaveWord::New).1, fault.1, "New has no fault colour");
        }
    }

    /// A missing part says `New` only by the origin a caller states, never by the missing part
    /// alone.
    #[test]
    fn only_the_origin_makes_a_draft_new() {
        let incomplete = || Diagnosis::incomplete("Add a creature first.");
        for (origin, word) in [
            (DraftOrigin::New, SaveWord::New),
            (
                DraftOrigin::Saved,
                SaveWord::Invalid(InvalidCause::Incomplete),
            ),
        ] {
            for changed in [Changed::Yes, Changed::No] {
                let state = DraftState::new(origin, changed, Primary::Draft).check(incomplete());
                assert_eq!(state.word(), word, "{origin:?}, {changed:?}");
            }
        }
    }

    #[test]
    fn the_lock_reason_names_save_only_while_save_would_run() {
        let state =
            |changed| DraftState::new(DraftOrigin::Saved, changed, Primary::Draft).locks_campaign();
        assert_eq!(state(Changed::No).lock_reason(), None);
        assert_eq!(
            state(Changed::Yes).lock_reason(),
            Some(SAVE_OR_DISCARD_TO_SWITCH)
        );
        assert_eq!(
            state(Changed::Yes)
                .check(Diagnosis::refused("Bad id."))
                .lock_reason(),
            Some(DISCARD_TO_SWITCH)
        );
        assert_eq!(
            DraftState::new(DraftOrigin::Saved, Changed::Yes, Primary::Draft).lock_reason(),
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
                DraftState::new(DraftOrigin::Saved, Changed::Yes, Primary::Draft),
                "Unsaved",
                true,
            ),
            (
                DraftState::new(DraftOrigin::Saved, Changed::No, Primary::Draft),
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
