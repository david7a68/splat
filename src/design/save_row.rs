//! The save row: a draft's state word and quiet `Discard` at the left, `Save` at the right.
//!
//! One function decides each part from one [`DraftState`]: the word ([`DraftState::word`]),
//! Save's weight ([`DraftState::save_weight`]), why Save waits ([`DraftState::save_blocked`]),
//! whether `Discard` shows, and the campaign-switch reason ([`DraftState::lock_reason`]). The
//! caller says what is true of the draft (new or saved, changed or not, what Save's check found
//! and why, changed on disk, a failed Save, who owns the primary, a repair that a clean draft's
//! Save writes, a wait outside the draft); it never picks a word, a weight, a tone or a reason.
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
//! use splat_design::ownership::{Changed, Diagnosis, DraftOrigin, DraftState, Primary, SaveWeight, SaveWord};
//! let draft = DraftState::new(DraftOrigin::New, Changed::Yes, Primary::Draft) // <-
//!     .check(Diagnosis::incomplete("Add a creature first.")); // <-
//! assert_eq!((draft.word(), draft.save_weight()), (SaveWord::New, SaveWeight::Secondary));
//! ```
//!
//! The word comes from the state; a caller cannot set it, or build a state by hand:
//!
//! ```compile_fail
//! # use splat_design::ownership::{Changed, DraftOrigin, DraftState, Primary, SaveWord};
//! let draft = DraftState {
//!     word: SaveWord::Unsaved, // <-
//!     ..DraftState::new(DraftOrigin::New, Changed::Yes, Primary::Draft)
//! };
//! ```
//!
//! Every draft states its origin; none is assumed:
//!
//! ```compile_fail
//! # use splat_design::ownership::{Changed, DraftState, Primary};
//! let draft = DraftState::new(Changed::Yes, Primary::Draft); // <- no origin
//! ```
//!
//! Whether the draft changed is a named state, not a `bool` that can be passed in the wrong
//! place:
//!
//! ```compile_fail
//! # use splat_design::ownership::{DraftOrigin, DraftState, Primary};
//! let draft = DraftState::new(DraftOrigin::New, true, Primary::Draft); // <- a bare bool
//! ```
//!
//! Save's check is a typed diagnosis: text alone is no refusal, and a diagnosis is made only by
//! its named constructors:
//!
//! ```compile_fail
//! # use splat_design::ownership::{Changed, DraftOrigin, DraftState, Primary};
//! let draft = DraftState::new(DraftOrigin::New, Changed::Yes, Primary::Draft)
//!     .check("Add a creature first."); // <- text
//! ```
//!
//! ```compile_fail
//! # use splat_design::ownership::Diagnosis;
//! let found = Diagnosis { refused: None, missing: None, incomplete: None, unreadable: true }; // <- by hand
//! ```
use super::{Status, Tone, colors, icons};
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
/// - [`Diagnosis::missing`]: a change that names a thing that is gone (a deleted encounter, a
///   creature this campaign cannot find): `Invalid` in the warning tone (R41), and Save waits
///   with the reason. It ranks after a refused change and before a missing part.
/// - [`Diagnosis::unreadable`]: the draft or its file cannot be read (a repair's problem):
///   `Invalid`, but Save is not blocked by it, since Save can repair it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnosis<'a> {
    refused: Option<Cow<'a, str>>,
    missing: Option<Cow<'a, str>>,
    incomplete: Option<Cow<'a, str>>,
    unreadable: bool,
}

impl<'a> Diagnosis<'a> {
    /// Nothing wrong.
    pub fn valid() -> Self {
        Self {
            refused: None,
            missing: None,
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

    /// A change that names a thing that is gone: a warning (R41), and Save waits.
    pub fn missing(why: impl Into<Cow<'a, str>>) -> Self {
        Self {
            missing: Some(why.into()),
            ..Self::valid()
        }
    }

    pub fn unreadable() -> Self {
        Self {
            unreadable: true,
            ..Self::valid()
        }
    }

    /// The kind that ranks first, by the one rule: unreadable data, a refused change, a thing
    /// that is gone, then a missing part. `None` when nothing is wrong.
    pub fn cause(&self) -> Option<InvalidCause> {
        if self.unreadable {
            Some(InvalidCause::Unreadable)
        } else if self.refused.is_some() {
            Some(InvalidCause::Refused)
        } else if self.missing.is_some() {
            Some(InvalidCause::Missing)
        } else if self.incomplete.is_some() {
            Some(InvalidCause::Incomplete)
        } else {
            None
        }
    }

    /// Whether this diagnosis's first-ranked fault outranks `other`'s, by the one rule of
    /// [`Self::cause`]. Equals do not outrank each other, so a caller that keeps the first of
    /// equals keeps the order of report among them (lead ruling, s30 B11: a group names its most
    /// severe member fault, then the first in member order).
    pub fn outranks(&self, other: &Diagnosis<'_>) -> bool {
        let rank = |cause: Option<InvalidCause>| match cause {
            None => 0,
            Some(InvalidCause::Incomplete) => 1,
            Some(InvalidCause::Missing) => 2,
            Some(InvalidCause::Refused) => 3,
            Some(InvalidCause::Unreadable) => 4,
        };
        rank(self.cause()) > rank(other.cause())
    }

    /// The tone of the first-ranked fault (R41): every place that says this fault (the save
    /// row's word, a field's message, the footer) paints it in this tone. `None` when nothing
    /// is wrong.
    pub fn tone(&self) -> Option<Tone> {
        self.cause().map(InvalidCause::tone)
    }

    /// Both diagnoses. Each kind keeps its own reason (the later report's, when both name one),
    /// so no kind hides another, and the rank does not depend on the order of the calls.
    pub fn and(self, other: Self) -> Self {
        Self {
            refused: other.refused.or(self.refused),
            missing: other.missing.or(self.missing),
            incomplete: other.incomplete.or(self.incomplete),
            unreadable: self.unreadable || other.unreadable,
        }
    }

    /// The reason Save waits, by the rank: a refused change, a thing that is gone, then a
    /// missing part. Unreadable data does not block Save.
    pub fn blocking(&self) -> Option<&str> {
        self.refused
            .as_deref()
            .or(self.missing.as_deref())
            .or(self.incomplete.as_deref())
    }

    /// Whether the one fault is a missing part (no refused change, nothing gone, no unreadable
    /// data).
    pub fn only_incomplete(&self) -> bool {
        self.incomplete.is_some()
            && self.refused.is_none()
            && self.missing.is_none()
            && !self.unreadable
    }

    pub fn into_owned(self) -> Diagnosis<'static> {
        Diagnosis {
            refused: self.refused.map(|why| Cow::Owned(why.into_owned())),
            missing: self.missing.map(|why| Cow::Owned(why.into_owned())),
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
    /// The change names a thing that is gone (a deleted encounter, a creature this campaign
    /// cannot find): the warning tone (R41).
    Missing,
}

impl InvalidCause {
    /// The one tone of this cause, wherever it is said (R41, R37): the status word takes it,
    /// and so does each message of the same fault.
    pub fn tone(self) -> Tone {
        match self {
            Self::Incomplete | Self::Missing => Tone::Warning,
            Self::Refused | Self::Unreadable => Tone::Error,
        }
    }
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
    /// A clean draft whose Save writes to repair its file (an unread choice): the row names the
    /// write ([`DraftState::status`]), in the warning tone.
    Repair,
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
            Self::Repair => "Repair",
            Self::NoChanges => "No changes",
        }
    }

    pub fn status(self) -> Status<'static> {
        let d = colors().document;
        let (icon, color) = match self {
            // The word takes its cause's tone (R37, R41): the one value that also paints the
            // cause's own message.
            Self::Invalid(cause) => match cause.tone() {
                Tone::Error => (icons::ERROR, d.invalid),
                tone => tone.parts(),
            },
            Self::NotSaved => (icons::ERROR, d.invalid),
            Self::New => (icons::DRAFT, d.clean),
            Self::ChangedOnDisk => (icons::WARN, d.external),
            Self::Unsaved => (icons::DRAFT, d.unsaved),
            Self::Saved => (icons::OK, d.saved),
            Self::Repair => Tone::Warning.parts(),
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
    /// The write a clean draft's Save makes to repair its file (`repairs`).
    repair: Option<Cow<'a, str>>,
    /// Why Save waits for something outside the draft (`waits`, `commits_elsewhere`).
    waits: Option<&'a str>,
    /// The wait is another commit that saves this draft (`commits_elsewhere`).
    elsewhere: bool,
    /// A form draft (`cancels`): clean, it shows `Cancel`, with this reason after it.
    cancels: Option<Option<&'a str>>,
    /// The lock sentence while something else waits for this draft too (`holds`).
    holds: Option<&'a str>,
    /// Discard's accessible name, when another draft's Discard can show on the same page
    /// (`discard_named`).
    discard_name: Option<&'a str>,
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
            repair: None,
            waits: None,
            elsewhere: false,
            cancels: None,
            holds: None,
            discard_name: None,
        }
    }

    /// Discard's accessible name, when another draft's Discard can show on the same page
    /// (issue #553: the creature pane beside the encounter, `Discard creature changes`), so a
    /// screen reader tells the two apart. Its visible word stays `Discard`.
    pub fn discard_named(mut self, name: &'a str) -> Self {
        self.discard_name = Some(name);
        self
    }

    /// A clean draft whose Save writes to repair its file (Rules and runs' unread choice): the
    /// row names that write (`write`, such as `Save replaces the unread choice on line 23 with
    /// these settings`) in the warning tone, Save is open (and `Ctrl+S`), and there is no
    /// Discard, since nothing changed. A changed draft says `Unsaved`: its Save repairs too.
    pub fn repairs(mut self, write: impl Into<Cow<'a, str>>) -> Self {
        self.repair = Some(write.into());
        self
    }

    /// Save waits for something outside the draft, not a fault of it (an open run, a folder
    /// Splat cannot read): Save is blocked with `why`, the word stays the draft's, and Discard
    /// stays; the campaign switch says Discard is the way out.
    pub fn waits(mut self, why: &'a str) -> Self {
        self.waits = Some(why);
        self.elsewhere = false;
        self
    }

    /// Save waits because another commit takes its place now (a folder move's two choices):
    /// blocked with `why`, as [`Self::waits`], but those choices save the draft, so the campaign
    /// switch still says Save or discard.
    pub fn commits_elsewhere(mut self, why: &'a str) -> Self {
        self.waits = Some(why);
        self.elsewhere = true;
        self
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

    /// An open form's draft (World's edit form): while it has no changes, quiet `Cancel`
    /// stands where `Discard` would and closes the form, with `reason` after it when something
    /// waits for the form to close (`Cancel to add a Log line.`). Its reasons take the row
    /// when the widest one fits there, else a line under the row that keeps its room in every
    /// state, so the first edit moves nothing.
    pub fn cancels(mut self, reason: Option<&'a str>) -> Self {
        self.cancels = Some(reason);
        self
    }

    /// The lock sentence while something else waits for this draft too, in place of the
    /// campaign switch's (World's Log line: `Save or discard to add a Log line or switch
    /// campaign.`). It holds while the draft has changes.
    pub fn holds(mut self, words: &'a str) -> Self {
        self.holds = Some(words);
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

    fn missing(&self) -> Option<&str> {
        self.kind(|found| found.missing.as_deref())
    }

    fn incomplete(&self) -> Option<&str> {
        self.kind(|found| found.incomplete.as_deref())
    }

    /// Why Save's check blocks Save, by the rank: a refused change, a thing that is gone, then
    /// a missing part.
    fn refusal(&self) -> Option<&str> {
        self.refused()
            .or_else(|| self.missing())
            .or_else(|| self.incomplete())
    }

    pub fn word(&self) -> SaveWord {
        if self.unreadable() {
            SaveWord::Invalid(InvalidCause::Unreadable)
        } else if self.refused().is_some() {
            SaveWord::Invalid(InvalidCause::Refused)
        } else if self.missing().is_some() {
            SaveWord::Invalid(InvalidCause::Missing)
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
        } else if self.repair.is_some() {
            SaveWord::Repair
        } else if self.saved {
            SaveWord::Saved
        } else {
            SaveWord::NoChanges
        }
    }

    /// The row's state as it paints: [`Self::word`]'s status, and for a repair the write it names.
    pub fn status(&self) -> Status<'_> {
        match (self.word(), &self.repair) {
            (SaveWord::Repair, Some(write)) => Status {
                word: write,
                ..SaveWord::Repair.status()
            },
            (word, _) => word.status(),
        }
    }

    /// Why Save waits, or `None` when Save would run.
    pub fn save_blocked(&self) -> Option<&str> {
        if self.on_disk {
            Some(CHANGED_ON_DISK)
        } else if let Some(why) = self.waits {
            Some(why)
        } else if !self.changed() && self.repair.is_none() {
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

    /// The draft's file changed on disk: a comparison or fault body owns the pane. The typed
    /// fact, never read from a sentence.
    pub fn file_changed(&self) -> bool {
        self.on_disk
    }

    /// `Discard` shows while the draft has changes and the comparison does not own the pane.
    pub fn discard_shown(&self) -> bool {
        self.changed() && !self.on_disk
    }

    /// The campaign-switch reason after `Discard`, for a draft that locks the switch.
    pub fn lock_reason(&self) -> Option<&'static str> {
        (self.locks_campaign && self.changed()).then(|| {
            if self.can_save() || self.elsewhere {
                SAVE_OR_DISCARD_TO_SWITCH
            } else {
                DISCARD_TO_SWITCH
            }
        })
    }

    /// A form draft's quiet `Cancel` (`cancels`): it stands where `Discard` would while the
    /// draft has no changes and the comparison does not own the pane. It is never blocked.
    pub fn cancel_shown(&self) -> bool {
        self.cancels.is_some() && !self.changed() && !self.on_disk
    }

    /// The one reason the row says after its state word and `Discard` or `Cancel`: with changes,
    /// the lock sentence (`holds`, else [`Self::lock_reason`]); with none, the reason after
    /// `Cancel` (`cancels`).
    pub fn row_reason(&self) -> Option<&str> {
        if self.on_disk {
            None
        } else if self.changed() {
            self.holds
                .filter(|_| self.locks_campaign)
                .or_else(|| self.lock_reason())
        } else {
            self.cancels.flatten()
        }
    }
}

/// One reason's place on a draft head's save row (layout step 6): every form the reason can take
/// in any state, and the words that can take its place after an icon (G122's substitution of a
/// fault said in its own place). Its room on the row, and its line below the row when it has no
/// room there, are measured by the widest and tallest of all of them, so the place is the same
/// whatever the draft holds.
#[derive(Clone, Copy, Debug)]
pub struct ReasonSlot<'a> {
    pub forms: &'a [&'a str],
    pub substitutes: &'a [&'a str],
    /// The short forms the row shows in every state when the forms do not fit beside the widest
    /// word and Discard (ruling M24: `Save or discard to switch.`), the full reason in the hover
    /// text and the accessible description (`short_reason`). Empty: no short form.
    pub short: &'a [&'a str],
}

impl<'a> ReasonSlot<'a> {
    /// A reason with these forms and nothing in its place.
    pub const fn of(forms: &'a [&'a str]) -> Self {
        Self {
            forms,
            substitutes: &[],
            short: &[],
        }
    }
}

/// The narrowest a name field gets so that the actions stand beside it; narrower, the actions
/// take their own line and the name its full width.
pub const NAME_LEAST: f32 = 160.0;

/// How wide a draft's name field may be, given the room to its column's edge (its margins
/// outside). It ends at `FORM_WIDTH` or at the column, whichever comes first. Within `XL` of the
/// column it ends at the column, so the two edges are the same or clearly apart.
pub fn name_width(column: f32) -> f32 {
    use super::tokens::{FORM_WIDTH, XL};
    if column - FORM_WIDTH < XL {
        column
    } else {
        FORM_WIDTH
    }
}

/// What the DM pressed in a save row.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[must_use]
pub struct SaveRow {
    pub save: bool,
    pub discard: bool,
    /// A form draft's `Cancel` (`DraftState::cancels`).
    pub cancel: bool,
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
            // R41: a change that names a thing that is gone is Invalid in any origin (a new
            // draft too), and it ranks after a refused change and before a missing part.
            (
                "new missing",
                new()
                    .check(incomplete())
                    .check(Diagnosis::missing("Its encounter was deleted.")),
                SaveWord::Invalid(InvalidCause::Missing),
                SaveWeight::Secondary,
                Some("Its encounter was deleted."),
                true,
            ),
            (
                "missing, then refused",
                yes()
                    .check(Diagnosis::missing("Its encounter was deleted."))
                    .check(Diagnosis::refused("Bad id.")),
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
            // M-E0 (Rules and runs): a clean draft whose Save repairs its file names the write,
            // and Save (and Ctrl+S) is open, with nothing to discard.
            (
                "repair",
                no().repairs("Save replaces the unread choice on line 23 with these settings"),
                SaveWord::Repair,
                SaveWeight::Primary,
                None,
                false,
            ),
            // A changed draft says Unsaved; its Save repairs too.
            (
                "repair changed",
                yes().repairs("Save replaces …"),
                SaveWord::Unsaved,
                SaveWeight::Primary,
                None,
                true,
            ),
            // A wait that is not the draft's: Save waits with it, the word stays the draft's,
            // and Discard stays. It outranks No changes.
            (
                "waits changed",
                yes().waits("Close the open run to save run settings."),
                SaveWord::Unsaved,
                SaveWeight::Secondary,
                Some("Close the open run to save run settings."),
                true,
            ),
            (
                "waits clean",
                no().waits("Close the open run to save run settings."),
                SaveWord::NoChanges,
                SaveWeight::Secondary,
                Some("Close the open run to save run settings."),
                false,
            ),
            (
                "commits elsewhere",
                yes().commits_elsewhere("Choose below to move the runs or not."),
                SaveWord::Unsaved,
                SaveWeight::Secondary,
                Some("Choose below to move the runs or not."),
                true,
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
        assert_eq!(tone(SaveWord::Invalid(InvalidCause::Missing)), warning);
        // One value: the diagnosis gives its cause, and the word and every message of the fault
        // take that cause's tone (R41).
        for (found, cause) in [
            (Diagnosis::incomplete("a"), InvalidCause::Incomplete),
            (Diagnosis::missing("a"), InvalidCause::Missing),
            (Diagnosis::refused("a"), InvalidCause::Refused),
            (Diagnosis::unreadable(), InvalidCause::Unreadable),
            (
                Diagnosis::missing("a").and(Diagnosis::incomplete("b")),
                InvalidCause::Missing,
            ),
            (
                Diagnosis::missing("a").and(Diagnosis::refused("b")),
                InvalidCause::Refused,
            ),
        ] {
            assert_eq!(found.cause(), Some(cause), "{found:?}");
            assert_eq!(found.tone(), Some(cause.tone()), "{found:?}");
            assert_eq!(
                tone(SaveWord::Invalid(cause)).0,
                cause.tone().parts().0,
                "{cause:?}: the word's icon is its cause's"
            );
        }
        assert_eq!(Diagnosis::valid().tone(), None);
        // The rank of `cause` orders whole diagnoses too; equals do not outrank each other.
        let ranked = [
            Diagnosis::valid(),
            Diagnosis::incomplete("a"),
            Diagnosis::missing("a"),
            Diagnosis::refused("a"),
            Diagnosis::unreadable(),
        ];
        for (i, low) in ranked.iter().enumerate() {
            for (j, high) in ranked.iter().enumerate() {
                assert_eq!(high.outranks(low), j > i, "{high:?} over {low:?}");
            }
        }
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
        // A wait outside the draft leaves Discard the way out; another commit that saves the
        // draft keeps Save in the sentence.
        assert_eq!(
            state(Changed::Yes).waits("Close the run.").lock_reason(),
            Some(DISCARD_TO_SWITCH)
        );
        assert_eq!(
            state(Changed::Yes)
                .commits_elsewhere("Choose below.")
                .lock_reason(),
            Some(SAVE_OR_DISCARD_TO_SWITCH)
        );
    }
}
