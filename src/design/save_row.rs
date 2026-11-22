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
//! when the button can (taken through [`save_key`], the one rule for every page's key), and show
//! a refusal's or a failure's sentence in the pane's footer. A
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
use super::{Action, Density, Status, Tone, colors, components::status, icons};
use egui::{self, Ui};
use std::borrow::Cow;

/// The page's `Ctrl+S`, one rule for every page (issues #513, #514): it is taken from this
/// frame's input only while `ready` (the draft can be saved; the editor is the one the key is
/// for), the place is enabled, and no modal is open ([`set_modal`]). A page takes it before its
/// fields, so no field reads it, and saves after them, so the words typed in that frame are in
/// the save. A raw take of the key is counted (`design::results::STAGED`).
pub fn save_key(ui: &Ui, ready: bool) -> bool {
    ready && ui.is_enabled() && take_save_key(ui.ctx())
}

/// [`save_key`] for a page that takes the key before it draws (it has no `Ui` yet); its caller
/// knows whether the page is enabled.
pub fn save_key_before_drawing(ctx: &egui::Context, ready: bool) -> bool {
    ready && take_save_key(ctx)
}

fn take_save_key(ctx: &egui::Context) -> bool {
    if modal_open(ctx) {
        return false;
    }
    ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S))
}

fn modal_id() -> egui::Id {
    egui::Id::new("design_save_key_modal")
}

/// The app says, once each pass before any page draws, whether a modal is open (its own dialogs,
/// the full-window image, any egui modal: `art::modal_open`). No page's `Ctrl+S` acts beneath
/// it ([`save_key`]).
pub fn set_modal(ctx: &egui::Context, open: bool) {
    let pass = ctx.cumulative_pass_nr();
    ctx.data_mut(|data| data.insert_temp(modal_id(), (pass, open)));
}

fn modal_open(ctx: &egui::Context) -> bool {
    let pass = ctx.cumulative_pass_nr();
    ctx.data(|data| data.get_temp::<(u64, bool)>(modal_id()))
        .is_some_and(|(set, open)| open && set == pass)
}

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
/// The campaign-switch reason's short forms on a draft head's narrow save row (ruling M24).
pub const SAVE_OR_DISCARD_SHORT: &str = "Save or discard to switch.";
pub const DISCARD_SHORT: &str = "Discard to switch.";

/// The short form of a campaign-switch reason (ruling M24), for a row too narrow for it.
pub fn switch_short(full: &str) -> &'static str {
    if full == DISCARD_TO_SWITCH {
        DISCARD_SHORT
    } else {
        SAVE_OR_DISCARD_SHORT
    }
}
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
        }
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

    /// `Cancel` shows: a form draft with no changes, the comparison not owning the pane.
    pub fn cancel_shown(&self) -> bool {
        self.cancels.is_some() && !self.changed() && !self.on_disk
    }

    /// The form's reason after `Cancel` while it shows.
    fn cancel_reason(&self) -> Option<&'a str> {
        self.cancels.flatten().filter(|_| self.cancel_shown())
    }

    /// Every sentence this draft's row can paint after its controls, whatever it holds now:
    /// the widest decides where a form draft's reasons stand.
    fn reasons(&self) -> Vec<&'a str> {
        let locks: &[&'a str] = if self.locks_campaign {
            &[SAVE_OR_DISCARD_TO_SWITCH, DISCARD_TO_SWITCH]
        } else {
            &[]
        };
        self.holds
            .into_iter()
            .chain(locks.iter().copied())
            .chain(self.cancels.flatten())
            .collect()
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

    /// The sentence the row paints after its controls now: the caller's while something else
    /// waits for the draft too (`holds`), else the campaign-switch reason, else a clean form's
    /// reason after `Cancel`.
    fn row_reason(&self) -> Option<&'a str> {
        self.holds
            .filter(|_| self.changed())
            .or(self.lock_reason())
            .or(self.cancel_reason())
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
}

/// The state words of a draft editor that its `DraftState` can reach when it never says
/// `changed_on_disk`, `not_saved` or `repairs` (the group editor): new, unsaved, invalid in each
/// cause, saved and no changes.
pub const EDIT_WORDS: [SaveWord; 8] = [
    SaveWord::Invalid(InvalidCause::Incomplete),
    SaveWord::Invalid(InvalidCause::Refused),
    SaveWord::Invalid(InvalidCause::Missing),
    SaveWord::Invalid(InvalidCause::Unreadable),
    SaveWord::New,
    SaveWord::Unsaved,
    SaveWord::Saved,
    SaveWord::NoChanges,
];

/// Every state word a save row can show.
pub const ALL_WORDS: [SaveWord; 10] = [
    SaveWord::Invalid(InvalidCause::Incomplete),
    SaveWord::Invalid(InvalidCause::Refused),
    SaveWord::Invalid(InvalidCause::Missing),
    SaveWord::Invalid(InvalidCause::Unreadable),
    SaveWord::New,
    SaveWord::ChangedOnDisk,
    SaveWord::NotSaved,
    SaveWord::Unsaved,
    SaveWord::Saved,
    SaveWord::NoChanges,
];

/// Layout step 6 (`DraftHead`): what a draft editor's head is given to plan from. Widths only,
/// never the draft's state: the plan is the same in every state the draft can reach.
#[derive(Clone, Copy, Debug)]
pub struct HeadSpec<'a> {
    /// Every state word the draft can reach (`EDIT_WORDS` for a draft that is never changed on
    /// disk, left unsaved by a failed write or repaired): the row is measured by the widest.
    pub words: &'a [SaveWord],
    /// The room from the name field's start to its owning column's edge.
    pub column: f32,
    /// The room from the name field's start to the end of the title line.
    pub line: f32,
    /// The title line's actions after the name (Save in its widest weight, and any others), with
    /// the gaps between them.
    pub actions: f32,
    /// The save row's room, from its start to its reasons' edge, less anything that stands at its
    /// end (a run's state).
    pub row: f32,
    /// Each reason slot of the save row, in row order (Save's own reason first, then the
    /// campaign-switch reason).
    pub reasons: &'a [ReasonSlot<'a>],
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

    /// The widest of `words` on one line.
    fn widest(ui: &Ui, words: &[&str]) -> f32 {
        let small = super::Type::Small.font(ui);
        words
            .iter()
            .map(|words| super::number_width(ui, words, &small))
            .fold(0.0, f32::max)
    }
}

/// A reason in its short form on the save row (ruling M24): the short words, muted, with the
/// full reason in the hover text and the accessible description.
pub fn short_reason(ui: &mut Ui, short: &str, full: &str) {
    let response = ui.label(super::Type::Small.rich(ui, short).color(colors().muted));
    super::named(ui, &response, full);
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_description(full));
    let _ = response.on_hover_text(full);
}

/// The height of `slot`'s own line under the save row, `width` wide: the tallest of its forms
/// and of its substitutes after their icon, so it is the same in every state (layout step 6).
pub fn slot_height(ui: &Ui, slot: &ReasonSlot<'_>, width: f32) -> f32 {
    let small = super::Type::Small.font(ui);
    let tall = |words: &str, width: f32| {
        ui.painter()
            .layout(
                words.to_owned(),
                small.clone(),
                colors().muted,
                width.max(0.0),
            )
            .size()
            .y
    };
    let forms = slot
        .forms
        .iter()
        .map(|words| tall(words, width))
        .fold(0.0, f32::max);
    slot.substitutes
        .iter()
        .map(|words| tall(words, width - super::tokens::ICON_SLOT - super::tokens::S))
        .fold(forms, f32::max)
}

/// Where a draft editor's head puts its parts (layout step 6, I7 and I8), from [`head_plan`].
#[derive(Clone, Debug, PartialEq)]
pub struct HeadPlan {
    /// The name field's width; its margins (`S` each side) are outside it.
    pub name_width: f32,
    /// The actions stand on the name's line, `L` after the field; else on a line of their own
    /// under it, from the text edge.
    pub actions_beside: bool,
    /// For each reason of [`HeadSpec::reasons`]: on the save row, or on a line of its own under
    /// it, kept in every state.
    pub on_row: Vec<bool>,
    /// For each reason on the row: in its short form (ruling M24), in every state.
    pub short: Vec<bool>,
}

/// The narrowest a name field gets so that the actions stand beside it; narrower, the actions
/// take their own line and the name its full width.
pub const NAME_LEAST: f32 = 160.0;

/// The widest state word the save row can show, with its icon.
fn widest_word(ui: &Ui, words: &[SaveWord]) -> f32 {
    let small = super::Type::Small.font(ui);
    words
        .iter()
        .map(|word| {
            let status = word.status();
            super::number_width(ui, &format!("{} {}", status.icon, status.word), &small)
        })
        .fold(0.0, f32::max)
}

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

/// A draft editor's head, planned before it paints from the widths and fonts only (layout step
/// 6). The name field is no wider than `FORM_WIDTH`, and ends at its column's edge when that is
/// within `XL`; the actions stand `L` after it while the name keeps at least `NAME_LEAST`, else
/// on their own line. Each reason takes the save row while it fits beside the widest state word
/// and `Discard`, whether or not either shows now: so no state the draft reaches (clean, dirty,
/// refused, new) moves a reason, the row's height or the first field.
pub fn head_plan(ui: &Ui, spec: &HeadSpec<'_>) -> HeadPlan {
    use super::tokens::{L, S};
    let gap = ui.spacing().item_spacing.x;
    let margins = 2.0 * S;
    let least = Density::Scan.control();
    let column = (spec.column - margins).max(least);
    let full = name_width(column);
    let beside = spec.line - margins - L - spec.actions;
    let (name_width, actions_beside) = if beside >= full.min(NAME_LEAST) {
        (full.min(beside).max(least), true)
    } else {
        (full, false)
    };
    let mut room = spec.row
        - widest_word(ui, spec.words)
        - Action::quiet("Discard").natural_width(ui)
        - 2.0 * gap;
    let (on_row, short) = spec
        .reasons
        .iter()
        .map(|slot| {
            // Its forms, else its short forms (M24), else a line of its own below the row.
            for (words, short) in [(slot.forms, false), (slot.short, true)] {
                if words.is_empty() {
                    continue;
                }
                let width = ReasonSlot::widest(ui, words) + gap;
                if width <= room {
                    room -= width;
                    return (true, short);
                }
            }
            (false, false)
        })
        .unzip();
    HeadPlan {
        name_width,
        actions_beside,
        on_row,
        short,
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
    let save_action = |width: f32| {
        match state.save_weight() {
            SaveWeight::Primary => Action::primary(save),
            SaveWeight::Secondary => Action::secondary(save),
        }
        .density(Density::Read)
        .blocked(state.save_blocked())
        .key("Ctrl+S")
        // Its widest weight's width in every state, so a change of weight moves nothing.
        .min_width(width)
    };
    let gap = ui.spacing().item_spacing.x;
    let small = super::Type::Small.font(ui);
    // Save's widest form (primary or secondary weight) and the widest lock sentence the draft can
    // show (`locks_campaign`), whatever the draft holds now: the form depends on the width only.
    let save_width = [Action::primary(save), Action::secondary(save)]
        .into_iter()
        .map(|action| {
            action
                .density(Density::Read)
                .key("Ctrl+S")
                .natural_width(ui)
        })
        .fold(0.0, f32::max);
    let lock_width = if state.locks_campaign {
        [SAVE_OR_DISCARD_TO_SWITCH, DISCARD_TO_SWITCH]
            .into_iter()
            .map(|words| super::number_width(ui, words, &small) + gap)
            .fold(0.0, f32::max)
    } else {
        0.0
    };
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
        let room = ui.available_width()
            - save_width
            - word
            - Action::quiet("Discard").natural_width(ui)
            - lock_width
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
    // A form draft's reasons (the lock sentence while it has changes, Cancel's while clean)
    // stand on the row when the widest fits beside the widest word, the widest of Discard and
    // Cancel, and Save; else on a line under the row that keeps its room in every state.
    let form = state.cancels.is_some();
    let below = form && {
        let word = [SaveWord::Unsaved, SaveWord::NoChanges]
            .into_iter()
            .map(|word| {
                let status = word.status();
                super::number_width(ui, &format!("{} {}", status.icon, status.word), &small)
            })
            .fold(0.0, f32::max);
        let room = ui.available_width()
            - save_width
            - word
            - Action::quiet("Discard")
                .natural_width(ui)
                .max(Action::quiet("Cancel").natural_width(ui))
            - 4.0 * gap;
        state
            .reasons()
            .iter()
            .any(|words| super::number_width(ui, words, &small) > room)
    };
    let mut pressed = SaveRow::default();
    ui.horizontal(|ui| {
        ui.set_min_height(Density::Scan.control());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            pressed.save = save_action(save_width).show(ui).clicked();
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                status(ui, state.status());
                if state.discard_shown() {
                    pressed.discard = Action::quiet("Discard").show(ui).clicked();
                }
                if state.cancel_shown() {
                    pressed.cancel = Action::quiet("Cancel").show(ui).clicked();
                }
                if !below && let Some(reason) = state.row_reason() {
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
    // A form draft's reasons under the row: the one that holds now, or the widest kept
    // unseen, so the row's height is the same in every state.
    if below {
        match state.row_reason() {
            Some(reason) => super::blocked_reason(ui, Some(reason)),
            None => {
                let widest = state.reasons().into_iter().max_by(|a, b| {
                    super::number_width(ui, a, &small)
                        .total_cmp(&super::number_width(ui, b, &small))
                });
                ui.scope_builder(egui::UiBuilder::new().invisible(), |ui| {
                    super::blocked_reason(ui, widest);
                });
            }
        }
    }
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

    /// Issues #513 and #514: every page's `Ctrl+S` is `save_key`, one rule. It is taken once,
    /// and only while the page is ready, its place is enabled, and the app says no modal is open
    /// (`set_modal`); a key it refuses stays in the input for no one.
    #[test]
    fn ctrl_s_is_taken_only_by_a_ready_enabled_page_with_no_modal() {
        use crate::design::test_harness::Themed as _;
        #[derive(Default)]
        struct Page {
            ready: bool,
            enabled: bool,
            modal: bool,
            /// The takes of this frame: the page's, then a second asker's.
            taken: (bool, bool),
        }
        let mut h = egui_kittest::Harness::builder().themed_ui_state(
            |ui, page: &mut Page| {
                set_modal(ui.ctx(), page.modal);
                if !page.enabled {
                    ui.disable();
                }
                page.taken = (save_key(ui, page.ready), save_key(ui, true));
            },
            Page {
                ready: true,
                enabled: true,
                ..Page::default()
            },
        );
        // The theme's frame first: the page draws from the next one.
        h.run();
        let press = |h: &mut egui_kittest::Harness<'_, Page>| {
            h.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::S);
            h.step();
            h.state().taken
        };
        assert_eq!(press(&mut h), (true, false), "a ready page takes it, once");
        h.state_mut().modal = true;
        assert_eq!(
            press(&mut h),
            (false, false),
            "#514: beneath a modal no page takes it"
        );
        h.state_mut().modal = false;
        h.state_mut().enabled = false;
        assert_eq!(
            press(&mut h),
            (false, false),
            "a disabled page takes it not"
        );
        h.state_mut().enabled = true;
        h.state_mut().ready = false;
        assert_eq!(
            press(&mut h),
            (false, true),
            "a page that is not ready leaves it"
        );
    }

    /// M2 (B2 packet 3, F2): the save row with the toolbar's draft reason chooses its form
    /// (long or short on its line, or the slot under it) from the width only. Across widths on
    /// both fit boundaries, walking clean → valid dirty → invalid → clean (with and without the
    /// campaign-switch lock), the room the reason adds is the same in every state, so it moves
    /// nothing. (The row's own campaign-switch sentence wraps at very narrow widths and grows
    /// the row with or without the reason; that is the row's, measured apart here.)
    #[test]
    fn the_reason_form_depends_on_the_width_only() {
        let reason = |locked| RowReason {
            long: "Save or discard to copy, add an entry or change packages.",
            short: "Save or discard first.",
            locked,
        };
        let states = |locks: bool| {
            let base = |changed| {
                let state = DraftState::new(DraftOrigin::Saved, changed, Primary::Draft);
                if locks { state.locks_campaign() } else { state }
            };
            [
                (base(Changed::No), false),
                (base(Changed::Yes), true),
                (
                    base(Changed::Yes).check(Diagnosis::refused("Bad id.")),
                    true,
                ),
                (base(Changed::No), false),
            ]
        };
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let mut forms = std::collections::BTreeSet::new();
        for locks in [false, true] {
            for width in (240..=900).step_by(4) {
                let mut tops = Vec::new();
                for (state, locked) in states(locks) {
                    let mut top = 0.0;
                    let mut plain = 0.0;
                    for _ in 0..2 {
                        let _ = ctx.run(egui::RawInput::default(), |ctx| {
                            egui::CentralPanel::default().show(ctx, |ui| {
                                let rect = egui::Rect::from_min_size(
                                    ui.cursor().min,
                                    egui::vec2(width as f32, 1_000.0),
                                );
                                ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                                    let start = ui.cursor().top();
                                    let _ = save_row_with(ui, &state, "Save", Some(reason(locked)));
                                    top = ui.label("After").rect.top() - start;
                                });
                                let rect = egui::Rect::from_min_size(
                                    ui.cursor().min,
                                    egui::vec2(width as f32, 1_000.0),
                                );
                                ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                                    let start = ui.cursor().top();
                                    let _ = save_row_with(ui, &state, "Save", None);
                                    plain = ui.label("After").rect.top() - start;
                                });
                            });
                        });
                    }
                    tops.push(top - plain);
                }
                forms.insert(tops[0] as i64);
                assert!(
                    tops.iter().all(|top| (top - tops[0]).abs() < 0.5),
                    "at {width} (campaign lock {locks}) the control after the row moved: {tops:?}"
                );
            }
        }
        assert!(
            forms.len() >= 2,
            "the sweep reached the slot under the row: {forms:?}"
        );
        // Layout step 6 (`head_plan`, I8; SL4): a draft head's reasons are planned for the
        // complete state set. At every width, each state the draft reaches (clean, new and empty,
        // dirty, refused, just saved) draws its word, its Discard and every reason the plan puts
        // on the row in its longest form, and the row keeps one height in every state (a reason
        // with no room would wrap and grow it) and stays inside its room: so a state change never
        // pushes a reason off the row, and nothing under it moves.
        // The group editor's two slots, with G122's substitutions in the switch reason's place.
        let reasons = [
            ReasonSlot::of(&["Add a creature first."]),
            ReasonSlot {
                forms: &[SAVE_OR_DISCARD_TO_SWITCH, DISCARD_TO_SWITCH],
                substitutes: &[
                    "Its encounter was deleted. See Belongs to.",
                    "A creature here cannot be saved. See its row.",
                ],
                short: &[SAVE_OR_DISCARD_SHORT, DISCARD_SHORT],
            },
        ];
        let head_states = [
            DraftState::new(DraftOrigin::Saved, Changed::No, Primary::Draft).locks_campaign(),
            DraftState::new(DraftOrigin::New, Changed::Yes, Primary::Draft)
                .locks_campaign()
                .check(Diagnosis::incomplete("Add a creature first.")),
            DraftState::new(DraftOrigin::Saved, Changed::Yes, Primary::Draft).locks_campaign(),
            DraftState::new(DraftOrigin::Saved, Changed::Yes, Primary::Draft)
                .locks_campaign()
                .check(Diagnosis::refused("Bad id.")),
            DraftState::new(DraftOrigin::Saved, Changed::No, Primary::Draft)
                .locks_campaign()
                .just_saved(),
        ];
        let mut placed = std::collections::BTreeSet::new();
        let mut short_seen = false;
        for width in (240..=900).step_by(4) {
            let width = width as f32;
            let mut heights = Vec::new();
            for state in &head_states {
                let mut right = 0.0;
                let mut height = 0.0;
                let mut plan = None;
                let _ = ctx.run(egui::RawInput::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let rect =
                            egui::Rect::from_min_size(ui.cursor().min, egui::vec2(width, 1_000.0));
                        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                            let head = head_plan(
                                ui,
                                &HeadSpec {
                                    words: &EDIT_WORDS,
                                    column: width,
                                    line: width,
                                    actions: 120.0,
                                    row: width,
                                    reasons: &reasons,
                                },
                            );
                            let row = ui.horizontal(|ui| {
                                // As the editors' save rows: one control high in every state.
                                ui.set_min_height(Density::Scan.control());
                                status(ui, state.status());
                                if state.discard_shown() {
                                    let _ = Action::quiet("Discard").show(ui);
                                }
                                for ((slot, on_row), short) in
                                    reasons.iter().zip(&head.on_row).zip(&head.short)
                                {
                                    if *on_row && *short {
                                        short_reason(ui, slot.short[0], slot.forms[0]);
                                    } else if *on_row {
                                        super::super::blocked_reason(ui, Some(slot.forms[0]));
                                    }
                                }
                            });
                            right = row.response.rect.right() - rect.left();
                            height = row.response.rect.height();
                            // Each slot under the row is reserved `slot_height` high; every form
                            // and every substitute (after its icon) fits in it, at this width.
                            for (slot, on_row) in reasons.iter().zip(&head.on_row) {
                                if *on_row {
                                    continue;
                                }
                                let kept = slot_height(ui, slot, width);
                                let drawn = slot
                                    .forms
                                    .iter()
                                    .map(|words| (*words, 0.0))
                                    .chain(slot.substitutes.iter().map(|words| {
                                        (*words, super::super::tokens::ICON_SLOT + super::super::tokens::S)
                                    }))
                                    .map(|(words, icon)| {
                                        let rect = egui::Rect::from_min_size(
                                            ui.cursor().min + egui::vec2(icon, 0.0),
                                            egui::vec2(width - icon, 1_000.0),
                                        );
                                        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                                            super::super::blocked_reason(ui, Some(words));
                                        })
                                        .response
                                        .rect
                                        .height()
                                    })
                                    .fold(0.0, f32::max);
                                assert!(
                                    drawn <= kept + 0.5,
                                    "at {width} a form of {slot:?} is {drawn} tall in its slot of {kept}"
                                );
                            }
                            plan = Some(head);
                        });
                    });
                });
                let plan = plan.expect("the head was planned");
                placed.insert(plan.on_row.clone());
                short_seen |= plan.short.iter().any(|short| *short);
                assert!(
                    right <= width + 0.5,
                    "at {width} the row of {:?} is {right} wide, past its room: {plan:?}",
                    state.word()
                );
                heights.push((state.word(), height));
            }
            assert!(
                heights.iter().all(|(_, h)| (h - heights[0].1).abs() < 0.5),
                "at {width} the row's height changes with the state: {heights:?}"
            );
        }
        assert!(
            placed.len() >= 2 && short_seen,
            "the sweep reached a reason under the row and a short form on it (M24): {placed:?}, {short_seen}"
        );
    }

    /// Ruling M24 (Astra's P3 on PR 668): a short reason on the row paints its short words and
    /// gives the full reason as its accessible description (and its name and hover text).
    #[test]
    fn a_short_reason_describes_the_full_reason() {
        use egui_kittest::kittest::{NodeT, Queryable};
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(400.0, 100.0))
            .build_ui(|ui| short_reason(ui, SAVE_OR_DISCARD_SHORT, SAVE_OR_DISCARD_TO_SWITCH));
        harness.run_steps(2);
        let node = harness
            .query_by_label(SAVE_OR_DISCARD_TO_SWITCH)
            .or_else(|| harness.query_by_label(SAVE_OR_DISCARD_SHORT))
            .expect("the short reason is an accessible node");
        assert_eq!(
            node.accesskit_node().description(),
            Some(SAVE_OR_DISCARD_TO_SWITCH.to_owned()),
            "the short reason's accessible description is the full reason"
        );
    }

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

    /// The row draws what the state decides: the word, Discard only with changes, and Save with
    /// its weight's type. One press reaches only its own control.
    #[test]
    fn the_row_draws_the_state() {
        use crate::design::test_harness::ThemedPage as _;
        use egui_kittest::kittest::Queryable;
        // M-W0.3: an open form's draft (`cancels`, World's edit form): clean, `Cancel` and its
        // reason; changed, `Discard` and the caller's sentence (`holds`).
        const CANCEL_WHY: &str = "Cancel to add a Log line.";
        const HOLDS: &str = "Save or discard to add a Log line or switch campaign.";
        let form = |changed| {
            DraftState::new(DraftOrigin::Saved, changed, Primary::Draft)
                .locks_campaign()
                .cancels(Some(CANCEL_WHY))
                .holds(HOLDS)
        };
        for (state, word, discard, cancel, reason) in [
            (
                DraftState::new(DraftOrigin::Saved, Changed::Yes, Primary::Draft),
                "Unsaved",
                true,
                false,
                None,
            ),
            (
                DraftState::new(DraftOrigin::Saved, Changed::No, Primary::Draft),
                "No changes",
                false,
                false,
                None,
            ),
            // A repair's row names its write.
            (
                DraftState::new(DraftOrigin::Saved, Changed::No, Primary::Draft)
                    .repairs("Save replaces the unread choice on line 23 with these settings"),
                "Save replaces the unread choice on line 23",
                false,
                false,
                None,
            ),
            (
                form(Changed::No),
                "No changes",
                false,
                true,
                Some(CANCEL_WHY),
            ),
            (form(Changed::Yes), "Unsaved", true, false, Some(HOLDS)),
        ] {
            let pressed = std::cell::Cell::new(SaveRow::default());
            let mut harness = egui_kittest::Harness::builder()
                .with_size(egui::vec2(600.0, 80.0))
                .themed_ui(|ui| {
                    let row = save_row(ui, &state, "Save");
                    if row.save || row.discard || row.cancel {
                        pressed.set(row);
                    }
                });
            harness.run();
            assert!(harness.query_by_label_contains(word).is_some(), "{word}");
            assert_eq!(
                harness.query_by_label("Discard").is_some(),
                discard,
                "{word}"
            );
            assert_eq!(harness.query_by_label("Cancel").is_some(), cancel, "{word}");
            if let Some(reason) = reason {
                assert!(
                    harness.query_by_label(reason).is_some(),
                    "{word}: the row says {reason:?}"
                );
            }
            if cancel {
                harness.get_by_label("Cancel").click();
                harness.run();
                assert_eq!(
                    pressed.get(),
                    SaveRow {
                        save: false,
                        discard: false,
                        cancel: true
                    }
                );
            }
            if discard {
                harness.get_by_label("Discard").click();
                harness.run();
                assert_eq!(
                    pressed.get(),
                    SaveRow {
                        save: false,
                        discard: true,
                        cancel: false
                    }
                );
                harness.get_by_label("Save").click();
                harness.run();
                assert_eq!(
                    pressed.get(),
                    SaveRow {
                        save: true,
                        discard: false,
                        cancel: false
                    }
                );
            }
        }
        // A narrow form: its reasons go on a line under the row, and the row with that line is
        // as tall clean as changed, so the first edit moves nothing; a wide one keeps them on
        // the row.
        // A form with nothing waiting says nothing while clean: the line under the row keeps
        // its room unseen.
        let plain = |changed| {
            DraftState::new(DraftOrigin::Saved, changed, Primary::Draft)
                .locks_campaign()
                .cancels(None)
        };
        for (width, under, waits) in [
            (300.0, true, true),
            (800.0, false, true),
            (300.0, true, false),
        ] {
            let mut heights = Vec::new();
            for changed in [Changed::No, Changed::Yes] {
                let state = if waits { form(changed) } else { plain(changed) };
                let (after, save, words) = (
                    std::cell::Cell::new(0.0),
                    std::cell::Cell::new(egui::Rect::NOTHING),
                    std::cell::Cell::new(egui::Rect::NOTHING),
                );
                let mut harness = egui_kittest::Harness::builder()
                    .with_size(egui::vec2(width + 20.0, 200.0))
                    .themed_ui(|ui| {
                        ui.set_max_width(width);
                        let start = ui.cursor().top();
                        let _ = save_row(ui, &state, "Save changes");
                        after.set(ui.label("After").rect.top() - start);
                    });
                harness.run();
                save.set(harness.get_by_label("Save changes").rect());
                let reason = match (changed, waits) {
                    (Changed::Yes, true) => Some(HOLDS),
                    (Changed::Yes, false) => Some(SAVE_OR_DISCARD_TO_SWITCH),
                    (Changed::No, true) => Some(CANCEL_WHY),
                    (Changed::No, false) => None,
                };
                if let Some(reason) = reason {
                    words.set(harness.get_by_label(reason).rect());
                    assert_eq!(
                        words.get().top() >= save.get().bottom(),
                        under,
                        "{width}: {reason:?} {:?} under the row (Save {:?}): {under}",
                        words.get(),
                        save.get()
                    );
                }
                heights.push(after.get());
            }
            assert!(
                (heights[0] - heights[1]).abs() < 0.5,
                "{width}: the row keeps its height clean and changed: {heights:?}"
            );
        }
    }
}
