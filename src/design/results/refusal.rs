//! A refused act's answer, typed (mechanism M1 of the structural phase): what was not done to
//! which named thing, why, and what the DM can do next. Every part is required and none is free
//! text a caller writes. The thing, the file, the act, the cause and the recovery are bounded
//! enums whose words this part writes, and a name is never blank. The tone comes from the cause,
//! in one place ([`Refusal::tone`], ruling R41). A caller that needs a cause from outside the app
//! adds a bounded diagnosis here, with its tone.
//!
//! A [`Refusal`] reaches a footer only through a [`Results`](super::Results) store: an opt-in
//! recorded Undo ([`Results::undo_or_refuse`](super::Results::undo_or_refuse)) keeps it on the
//! entry it failed on, with the order of that attempt, and the entry's line then opens with the
//! refusal's words.
//!
//! # What does not compile
//!
//! Each case has a twin that compiles and differs only in the marked line. The twin:
//!
//! ```
//! use splat::ownership::{Cause, Change, FailedAct, File, Kind, Name, Recovery, Refusal, Thing};
//! let fight = Name::new("Shrine of Cinders").unwrap();
//! let session = Name::new("Session 1: The Crossing").unwrap();
//! let refusal = Refusal::new(
//!     FailedAct::NotRemoved { what: Thing::new(Kind::PlanLine, fight), from: session },
//!     Cause::Changed(File::SessionNote),
//!     Recovery::ByHand { file: File::SessionNote, change: Change::RemovePlanLine },
//! );
//! assert_eq!(
//!     refusal.words().as_str(),
//!     "The plan line for Shrine of Cinders was not removed from Session 1: The Crossing. \
//!      The session note changed on disk. Open that session’s notes and remove the plan line there.",
//! );
//! ```
//!
//! A refusal with no recovery:
//!
//! ```compile_fail
//! use splat::ownership::{Cause, FailedAct, File, Kind, Name, Refusal, Thing};
//! let fight = Name::new("Shrine of Cinders").unwrap();
//! let session = Name::new("Session 1: The Crossing").unwrap();
//! let refusal = Refusal::new( // <- no recovery
//!     FailedAct::NotRemoved { what: Thing::new(Kind::PlanLine, fight), from: session },
//!     Cause::Changed(File::SessionNote),
//! );
//! ```
//!
//! A recovery's change, a cause's file, a thing's kind, a recovery, a name or a refusal from
//! free text:
//!
//! ```compile_fail
//! use splat::ownership::{File, Recovery};
//! let next = Recovery::ByHand { file: File::SessionNote, change: "" }; // <- a change from text
//! ```
//! ```compile_fail
//! use splat::ownership::Cause;
//! let cause = Cause::Changed(""); // <- a file from text
//! ```
//! ```compile_fail
//! use splat::ownership::{Name, Thing};
//! let thing = Thing::new("plan line", Name::new("Bell").unwrap()); // <- a kind from text
//! ```
//! ```compile_fail
//! use splat::ownership::Recovery;
//! let next: Recovery = "Try again.".into(); // <- a recovery from text
//! ```
//! ```compile_fail
//! use splat::ownership::Name;
//! let name: Name = String::from("").into(); // <- a name from text, unchecked
//! ```
//! ```compile_fail
//! use splat::ownership::Name;
//! let name = Name::fallback("this session"); // <- a fallback from text
//! ```
//! ```compile_fail
//! use splat::ownership::Refusal;
//! let refusal = Refusal::default(); // <- a refusal with nothing in it
//! ```
//! ```compile_fail
//! use splat::ownership::{Cause, Change, FailedAct, File, Kind, Name, Recovery, Refusal, Thing};
//! let session = Name::new("Session 1").unwrap();
//! let fight = Name::new("Bell").unwrap();
//! let refusal = Refusal { // <- built by hand, around its constructor
//!     act: FailedAct::NotRemoved { what: Thing::new(Kind::PlanLine, fight), from: session },
//!     cause: Cause::Changed(File::SessionNote),
//!     recovery: Recovery::ByHand { file: File::SessionNote, change: Change::RemovePlanLine },
//! };
//! ```
//!
//! A blank name is no name; the caller gives a typed fallback instead:
//!
//! ```
//! use splat::ownership::{Fallback, Name};
//! assert!(Name::new("  ").is_none());
//! let name = Name::new("").unwrap_or(Name::fallback(Fallback::ThatSession));
//! assert_eq!(name.as_str(), "that session");
//! ```
use crate::design::{Tone, outcome::Sentence};

/// A name the DM sees (a fight, a session), never blank. Made by [`Name::new`], which refuses
/// blank words, or by [`Name::fallback`], a fixed noun for a thing with no name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

/// The fixed noun for a thing whose name is blank.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fallback {
    /// `this fight`
    ThisFight,
    /// `that session`
    ThatSession,
}

impl Name {
    /// The name, trimmed; `None` when it is blank.
    pub fn new(words: impl Into<String>) -> Option<Self> {
        let words: String = words.into();
        let words = words.trim();
        (!words.is_empty()).then(|| Self(words.to_owned()))
    }

    pub fn fallback(fallback: Fallback) -> Self {
        Self(
            match fallback {
                Fallback::ThisFight => "this fight",
                Fallback::ThatSession => "that session",
            }
            .to_owned(),
        )
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The kind of thing an act was on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A session plan's line for a fight: `plan line`.
    PlanLine,
    /// A session's recording: `recording`.
    Recording,
}

impl Kind {
    fn noun(self) -> &'static str {
        match self {
            Self::PlanLine => "plan line",
            Self::Recording => "recording",
        }
    }
}

/// A named thing an act was on: its kind (`plan line`) and the name that tells it apart
/// (`for Shrine of Cinders`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Thing {
    kind: Kind,
    name: Name,
}

impl Thing {
    pub fn new(kind: Kind, name: Name) -> Self {
        Self { kind, name }
    }
}

/// The kind of file a cause or a recovery is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum File {
    /// A session's note.
    SessionNote,
    /// A session's recording file.
    Recording,
}

impl File {
    /// `session note`
    fn noun(self) -> &'static str {
        match self {
            Self::SessionNote => "session note",
            Self::Recording => "recording file",
        }
    }

    /// What the DM opens: `that session’s notes`.
    fn opens(self) -> &'static str {
        match self {
            Self::SessionNote => "that session’s notes",
            Self::Recording => "that session’s recordings folder",
        }
    }

    /// The file itself: `that session’s note file`.
    fn file(self) -> &'static str {
        match self {
            Self::SessionNote => "that session’s note file",
            Self::Recording => "that recording file",
        }
    }
}

/// A change the DM makes by hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Change {
    /// `remove the plan line`
    RemovePlanLine,
    /// `delete the recording file`
    DeleteRecording,
}

impl Change {
    fn words(self) -> &'static str {
        match self {
            Self::RemovePlanLine => "remove the plan line",
            Self::DeleteRecording => "delete the recording file",
        }
    }
}

/// What the refused act did not do, to which thing, where. A new kind of act is a new variant,
/// whose words are made here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FailedAct {
    /// Nothing changed: `The plan line for Shrine of Cinders was not removed from Session 1:
    /// The Crossing.`
    NotRemoved { what: Thing, from: Name },
    /// The change is in the open draft of `place`, and the draft was not saved: `The plan line
    /// for Bell was taken out of the draft of Session 1, and the draft was not saved.`
    LeftDraft { what: Thing, place: Name },
    /// Nothing changed: `The recording for Session 1 was not deleted from Session 1: The
    /// Crossing.`
    NotDeleted { what: Thing, from: Name },
}

/// Why the act was refused: a typed diagnosis, which also gives the tone (R41). The part writes
/// every cause; a new kind of cause is a new variant, with its tone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cause {
    /// `The session note changed on disk.` A warning.
    Changed(File),
    /// `The session note has unsaved words, and its file changed on disk.` A warning.
    Conflict(File),
    /// `The session note is gone from its folder.` A warning (R41: missing).
    Gone(File),
    /// `The file of the session note now holds another note.` A warning.
    Replaced(File),
    /// `Splat cannot read the session note.` An error (R41: unreadable).
    Unreadable(File),
    /// `The session note could not be saved.` An error.
    NotSaved(File),
    /// `Two files hold the session note.` A warning: Splat cannot tell which one to change.
    TwoFiles(File),
}

/// The DM's next step: a bounded operation on a real target, whose words this part writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recovery {
    /// `Open that session’s notes and remove the plan line there.`
    ByHand { file: File, change: Change },
    /// `Open that session’s notes and choose which words to keep.`
    Choose(File),
    /// The change waits in the open draft: `Open that session’s notes and save them.`
    SaveDraft(File),
    /// Nothing changed because the file could not be written: `Press Undo again when the folder
    /// can be written, or open that session’s notes and remove the plan line there.`
    RetryOrByHand { file: File, change: Change },
    /// `Put that session’s note file back in its folder, then press Undo again.`
    PutBack(File),
    /// `Check that session’s note file in its folder, then press Undo again.`
    CheckFile(File),
    /// `Keep one copy of that session’s note file in its folder, then press Undo again.`
    KeepOneFile(File),
}

/// A refused act's answer: what was not done, why, and the next step. Its fields are private, it
/// has no `Default` and no conversion from text: [`Refusal::new`] takes all three parts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    act: FailedAct,
    cause: Cause,
    recovery: Recovery,
}

impl Refusal {
    pub fn new(act: FailedAct, cause: Cause, recovery: Recovery) -> Self {
        Self {
            act,
            cause,
            recovery,
        }
    }

    /// The tone of its answer, from its diagnosis (R41): a file that cannot be read or written
    /// is an error; a file that changed, went or holds another note is a warning.
    pub fn tone(&self) -> Tone {
        match self.cause {
            Cause::Changed(_)
            | Cause::Conflict(_)
            | Cause::Gone(_)
            | Cause::Replaced(_)
            | Cause::TwoFiles(_) => Tone::Warning,
            Cause::Unreadable(_) | Cause::NotSaved(_) => Tone::Error,
        }
    }

    /// Its next step, for the owner's end events: a recovery that succeeded ends it
    /// (`Results::end_recovered`).
    pub fn recovery(&self) -> Recovery {
        self.recovery
    }

    /// The whole answer: the act not done, the cause, the next step.
    pub fn words(&self) -> Sentence {
        self.words_told_apart(None)
    }

    /// The whole answer, its place told apart from another answer's by `apart` (a file and a
    /// journey: `Session 1 (Road.md, Ford crossing)`), when two answers would read alike.
    pub fn words_told_apart(&self, apart: Option<&Name>) -> Sentence {
        let place = |name: &Name| match apart {
            Some(apart) => format!("{} ({})", name.as_str(), apart.as_str()),
            None => name.as_str().to_owned(),
        };
        let act = match &self.act {
            FailedAct::NotRemoved { what, from } => crate::sentence!(
                "The {} for {} was not removed from {}.",
                what.kind.noun(),
                what.name.as_str(),
                place(from)
            ),
            FailedAct::LeftDraft { what, place: at } => crate::sentence!(
                "The {} for {} was taken out of the draft of {}, and the draft was not saved.",
                what.kind.noun(),
                what.name.as_str(),
                place(at)
            ),
            FailedAct::NotDeleted { what, from } => crate::sentence!(
                "The {} for {} was not deleted from {}.",
                what.kind.noun(),
                what.name.as_str(),
                place(from)
            ),
        };
        let cause = match &self.cause {
            Cause::Changed(file) => crate::sentence!("The {} changed on disk.", file.noun()),
            Cause::Conflict(file) => crate::sentence!(
                "The {} has unsaved words, and its file changed on disk.",
                file.noun()
            ),
            Cause::Gone(file) => crate::sentence!("The {} is gone from its folder.", file.noun()),
            Cause::Replaced(file) => {
                crate::sentence!("The file of the {} now holds another note.", file.noun())
            }
            Cause::Unreadable(file) => crate::sentence!("Splat cannot read the {}.", file.noun()),
            Cause::NotSaved(file) => {
                crate::sentence!("The {} could not be saved.", file.noun())
            }
            Cause::TwoFiles(file) => crate::sentence!("Two files hold the {}.", file.noun()),
        };
        let next = match self.recovery {
            Recovery::ByHand { file, change } => {
                crate::sentence!("Open {} and {} there.", file.opens(), change.words())
            }
            Recovery::Choose(file) => {
                crate::sentence!("Open {} and choose which words to keep.", file.opens())
            }
            Recovery::SaveDraft(file) => crate::sentence!("Open {} and save them.", file.opens()),
            Recovery::RetryOrByHand { file, change } => crate::sentence!(
                "Press Undo again when the folder can be written, or open {} and {} there.",
                file.opens(),
                change.words()
            ),
            Recovery::PutBack(file) => crate::sentence!(
                "Put {} back in its folder, then press Undo again.",
                file.file()
            ),
            Recovery::CheckFile(file) => crate::sentence!(
                "Check {} in its folder, then press Undo again.",
                file.file()
            ),
            Recovery::KeepOneFile(file) => crate::sentence!(
                "Keep one copy of {} in its folder, then press Undo again.",
                file.file()
            ),
        };
        act.then(cause).then(next)
    }
}
