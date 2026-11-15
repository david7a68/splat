//! A refused act's answer, typed (mechanism M1 of the structural phase): what was not done to
//! which named thing, why, and what the DM can do next. Every part is required and none is free
//! text a caller writes: the act and its recovery are enums with real targets, the names are never
//! blank, and only an outside cause (a system error) is a [`Sentence`] from outside the app.
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
//! use splat::ownership::{Cause, FailedAct, Name, Opens, Recovery, Refusal, Thing};
//! let fight = Name::new("Shrine of Cinders").unwrap();
//! let session = Name::new("Session 1: The Crossing").unwrap();
//! let refusal = Refusal::new(
//!     FailedAct::NotRemoved { what: Thing::new("plan line", fight), from: session },
//!     Cause::Changed("session note"),
//!     Recovery::ByHand { open: Opens::SessionNotes, change: "remove the plan line" },
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
//! use splat::ownership::{Cause, FailedAct, Name, Refusal, Thing};
//! let fight = Name::new("Shrine of Cinders").unwrap();
//! let session = Name::new("Session 1: The Crossing").unwrap();
//! let refusal = Refusal::new( // <- no recovery
//!     FailedAct::NotRemoved { what: Thing::new("plan line", fight), from: session },
//!     Cause::Changed("session note"),
//! );
//! ```
//!
//! A recovery, a name or a refusal from free text:
//!
//! ```compile_fail
//! use splat::ownership::Recovery;
//! let next: Recovery = "Try again.".into(); // <- a recovery from text
//! ```
//! ```compile_fail
//! use splat::ownership::Name;
//! let name: Name = String::from("").into(); // <- a name from text, unchecked
//! ```
//! ```compile_fail
//! use splat::ownership::Refusal;
//! let refusal = Refusal::default(); // <- a refusal with nothing in it
//! ```
//! ```compile_fail
//! use splat::ownership::{Cause, FailedAct, Name, Opens, Recovery, Refusal, Thing};
//! let session = Name::new("Session 1").unwrap();
//! let fight = Name::new("Bell").unwrap();
//! let refusal = Refusal { // <- built by hand, around its constructor
//!     act: FailedAct::NotRemoved { what: Thing::new("plan line", fight), from: session },
//!     cause: Cause::Changed("session note"),
//!     recovery: Recovery::ByHand { open: Opens::SessionNotes, change: "remove the plan line" },
//! };
//! ```
//!
//! A blank name is no name; the caller gives a typed fallback noun instead:
//!
//! ```
//! use splat::ownership::Name;
//! assert!(Name::new("  ").is_none());
//! let name = Name::new("").unwrap_or(Name::fallback("this session"));
//! assert_eq!(name.as_str(), "this session");
//! ```
use crate::design::outcome::Sentence;

/// A name the DM sees (a fight, a session), never blank. Made by [`Name::new`], which refuses
/// blank words, or by [`Name::fallback`], a fixed noun for a thing with no name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    /// The name, trimmed; `None` when it is blank.
    pub fn new(words: impl Into<String>) -> Option<Self> {
        let words: String = words.into();
        let words = words.trim();
        (!words.is_empty()).then(|| Self(words.to_owned()))
    }

    /// A fixed noun for a thing whose name is blank (`this session`).
    pub fn fallback(noun: &'static str) -> Self {
        assert!(!noun.trim().is_empty(), "a fallback noun is not blank");
        Self(noun.to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A named thing an act was on: a kind of thing (`plan line`) and the name that tells it apart
/// (`for Shrine of Cinders`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Thing {
    kind: &'static str,
    name: Name,
}

impl Thing {
    /// `kind` is a fixed noun (`plan line`), never blank.
    pub fn new(kind: &'static str, name: Name) -> Self {
        assert!(!kind.trim().is_empty(), "a thing's kind is not blank");
        Self { kind, name }
    }
}

/// What the refused act did not do, to which thing, where. A new kind of act is a new variant,
/// whose words are made here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FailedAct {
    /// `The plan line for Shrine of Cinders was not removed from Session 1: The Crossing.`
    NotRemoved { what: Thing, from: Name },
    /// The act's change is in the open draft of `place`, and the draft was not saved:
    /// `The plan line for Bell was taken out of Session 1’s draft, and the draft was not saved.`
    NotSaved { what: Thing, place: Name },
}

/// Why the act was refused. The file's noun (`session note`) is fixed; only an error of the
/// system comes from outside, checked by [`Sentence::outside`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cause {
    /// `The session note changed on disk.`
    Changed(&'static str),
    /// `The session note has unsaved words, and its file changed on disk.`
    Conflict(&'static str),
    /// `The session note is gone from its folder.`
    Gone(&'static str),
    /// `The file of the session note now holds another note.`
    Replaced(&'static str),
    /// `Splat cannot read the session note.`
    Unreadable(&'static str),
    /// `The session note could not be saved.`
    NotSaved(&'static str),
    /// An error of the system, as it came (a failed save).
    Outside(Sentence),
}

/// What the DM opens to recover: a place that exists for the refused thing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opens {
    /// `that session’s notes`: the notes of the session the act was for.
    SessionNotes,
}

impl Opens {
    fn words(self) -> &'static str {
        match self {
            Self::SessionNotes => "that session’s notes",
        }
    }

    /// The file behind it, for a recovery that names the file.
    fn file(self) -> &'static str {
        match self {
            Self::SessionNotes => "that session’s note file",
        }
    }
}

/// The DM's next step, with its real target. No variant is free text: `change` is a fixed phrase
/// that starts with its verb (`remove the plan line`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Recovery {
    /// `Open that session’s notes and remove the plan line there.`
    ByHand { open: Opens, change: &'static str },
    /// `Open that session’s notes and choose which words to keep.`
    Choose { open: Opens },
    /// `Open that session’s notes and save them.`
    Save { open: Opens },
    /// `Put that session’s note file back in its folder, then press Undo again.`
    PutBack { open: Opens },
    /// `Check that session’s note file in its folder, then press Undo again.`
    CheckFile { open: Opens },
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

    /// The whole answer: the act not done, the cause, the next step.
    pub fn words(&self) -> Sentence {
        let act = match &self.act {
            FailedAct::NotRemoved { what, from } => crate::sentence!(
                "The {} for {} was not removed from {}.",
                what.kind,
                what.name.as_str(),
                from.as_str()
            ),
            FailedAct::NotSaved { what, place } => crate::sentence!(
                "The {} for {} was taken out of the draft of {}, and the draft was not saved.",
                what.kind,
                what.name.as_str(),
                place.as_str()
            ),
        };
        let cause = match &self.cause {
            Cause::Changed(file) => crate::sentence!("The {} changed on disk.", file),
            Cause::Conflict(file) => crate::sentence!(
                "The {} has unsaved words, and its file changed on disk.",
                file
            ),
            Cause::Gone(file) => crate::sentence!("The {} is gone from its folder.", file),
            Cause::Replaced(file) => {
                crate::sentence!("The file of the {} now holds another note.", file)
            }
            Cause::Unreadable(file) => crate::sentence!("Splat cannot read the {}.", file),
            Cause::NotSaved(file) => crate::sentence!("The {} could not be saved.", file),
            Cause::Outside(words) => words.clone(),
        };
        let next = match self.recovery {
            Recovery::ByHand { open, change } => {
                crate::sentence!("Open {} and {} there.", open.words(), change)
            }
            Recovery::Choose { open } => {
                crate::sentence!("Open {} and choose which words to keep.", open.words())
            }
            Recovery::Save { open } => crate::sentence!("Open {} and save them.", open.words()),
            Recovery::PutBack { open } => crate::sentence!(
                "Put {} back in its folder, then press Undo again.",
                open.file()
            ),
            Recovery::CheckFile { open } => crate::sentence!(
                "Check {} in its folder, then press Undo again.",
                open.file()
            ),
        };
        act.then(cause).then(next)
    }
}
