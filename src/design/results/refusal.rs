//! A refused act's answer, typed (mechanism M1 of the structural phase): what was not done to
//! which named thing, why, and what the DM can do next. Every part is required and none is free
//! text a caller writes. The thing, the file, the act, the cause and the recovery are bounded
//! enums whose words this part writes, and a name is never blank. The tone comes from the cause,
//! in one place ([`Refusal::tone`], ruling R41). A caller that needs a cause from outside the app
//! adds a bounded diagnosis here, with its tone.
//!
//! Ruling M7 (migration 32): an attempted act that failed is a refusal, whatever page it is on:
//! Start fight and Continue fight, Carry over, the plan's scenes, a tracker's acts and its
//! history, World's and Lookup's Add to fight, and the plan's links and run files. Each has its act, its causes and its
//! next steps here. A standing fault or a notice is not a refusal: it uses `FaultMessage` or
//! `Notice`.
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
//! use splat_design::ownership::{Cause, Change, FailedAct, File, Kind, Name, Recovery, Refusal, Thing};
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
//! use splat_design::ownership::{Cause, FailedAct, File, Kind, Name, Refusal, Thing};
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
//! use splat_design::ownership::{File, Recovery};
//! let next = Recovery::ByHand { file: File::SessionNote, change: "" }; // <- a change from text
//! ```
//! ```compile_fail
//! use splat_design::ownership::Cause;
//! let cause = Cause::Changed(""); // <- a file from text
//! ```
//! ```compile_fail
//! use splat_design::ownership::{Name, Thing};
//! let thing = Thing::new("plan line", Name::new("Bell").unwrap()); // <- a kind from text
//! ```
//! ```compile_fail
//! use splat_design::ownership::Recovery;
//! let next: Recovery = "Try again.".into(); // <- a recovery from text
//! ```
//! ```compile_fail
//! use splat_design::ownership::Name;
//! let name: Name = String::from("").into(); // <- a name from text, unchecked
//! ```
//! ```compile_fail
//! use splat_design::ownership::Name;
//! let name = Name::fallback("this session"); // <- a fallback from text
//! ```
//! ```compile_fail
//! use splat_design::ownership::Refusal;
//! let refusal = Refusal::default(); // <- a refusal with nothing in it
//! ```
//! ```compile_fail
//! use splat_design::ownership::{Cause, Change, FailedAct, File, Kind, Name, Recovery, Refusal, Thing};
//! let session = Name::new("Session 1").unwrap();
//! let fight = Name::new("Bell").unwrap();
//! let refusal = Refusal { // <- built by hand, around its constructor
//!     act: FailedAct::NotRemoved { what: Thing::new(Kind::PlanLine, fight), from: session },
//!     cause: Cause::Changed(File::SessionNote),
//!     recovery: Recovery::ByHand { file: File::SessionNote, change: Change::RemovePlanLine },
//! };
//! ```
//!
//! M7's acts, causes and next steps take only checked names, never words. The twin of the cases
//! below:
//!
//! ```
//! use splat_design::ownership::{Cause, FailedAct, Name, Recovery, Refusal, Rows, Step};
//! let rows = Rows::new([Name::new("Cinder Wolf").unwrap()]).unwrap();
//! let refusal = Refusal::new(
//!     FailedAct::NotStarted { encounter: Name::new("Shrine of Cinders").unwrap() },
//!     Cause::CreatureDeleted(rows.clone()),
//!     Recovery::First(Step::ReplaceRows(rows)),
//! );
//! assert_eq!(
//!     refusal.words().as_str(),
//!     "No fight started from Shrine of Cinders. The creature of the Cinder Wolf row was \
//!      deleted. Choose a replacement for the Cinder Wolf row first.",
//! );
//! ```
//! ```compile_fail
//! use splat_design::ownership::Step;
//! let step: Step = "Check the note".into(); // <- a next step from text
//! ```
//! ```compile_fail
//! use splat_design::ownership::Rows;
//! let rows = Rows::new(["Cinder Wolf"]); // <- rows from text, unchecked
//! ```
//! ```compile_fail
//! use splat_design::ownership::Cause;
//! let cause = Cause::DependsOn("Wandering roll"); // <- a name from text
//! ```
//! ```compile_fail
//! use splat_design::ownership::Rows;
//! let rows = Rows::default(); // <- rows with no row in them
//! ```
//!
//! Ruling M13: a refusal at the tracker's own limit ends at its cause, and only
//! `Refusal::at_limit` can end a refusal with no next step. The twin:
//!
//! ```
//! use splat_design::ownership::{FailedAct, Limit, Name, Refusal};
//! let refusal = Refusal::at_limit(
//!     FailedAct::NotRecorded { act: Name::new("Lose favor").unwrap() },
//!     Limit::Min { tracker: Name::new("Crowd Favor").unwrap(), min: 0 },
//! );
//! assert_eq!(
//!     refusal.words().as_str(),
//!     "Nothing was recorded for Lose favor. Crowd Favor cannot go below 0.",
//! );
//! ```
//! ```compile_fail
//! use splat_design::ownership::{AtLimit, Cause, FailedAct, File, Name, Recovery, Refusal};
//! let refusal = Refusal::new(
//!     FailedAct::NotRecorded { act: Name::new("Lose favor").unwrap() },
//!     Cause::Changed(File::Trackers),
//!     Recovery::AtLimit(AtLimit(())), // <- no next step for another cause
//! );
//! ```
//!
//! A blank name is no name; the caller gives a typed fallback instead:
//!
//! ```
//! use splat_design::ownership::{Fallback, Name};
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
    /// `this scene`: a plan scene whose words are blank (M-N1.4).
    ThisScene,
    /// `that tracker`: a tracker's act, label or name that has no words (M-S0.3)
    ThatTracker,
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
                Fallback::ThisScene => "this scene",
                Fallback::ThatTracker => "that tracker",
            }
            .to_owned(),
        )
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The rows of an encounter an act or a cause is about, each by its creature's name: never
/// empty. The part writes them: `the Cinder Wolf row`, `the 2 Cinder Wolf rows`, `the Cinder Wolf
/// row and the Ember Cultist row`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rows(Vec<(Name, usize)>);

impl Rows {
    /// One row for each name, in their order; rows of one creature are counted together.
    /// `None` when there is no row.
    pub fn new(rows: impl IntoIterator<Item = Name>) -> Option<Self> {
        let mut counted: Vec<(Name, usize)> = Vec::new();
        for name in rows {
            match counted.iter_mut().find(|(seen, _)| *seen == name) {
                Some((_, count)) => *count += 1,
                None => counted.push((name, 1)),
            }
        }
        (!counted.is_empty()).then_some(Self(counted))
    }

    /// How many rows, of every creature.
    fn count(&self) -> usize {
        self.0.iter().map(|(_, count)| count).sum()
    }

    /// Whether the rows are of more than one creature.
    fn creatures(&self) -> usize {
        self.0.len()
    }

    /// `the Cinder Wolf row`, `the 2 Cinder Wolf rows and the Ember Cultist row`.
    fn words(&self) -> String {
        let rows: Vec<String> = self
            .0
            .iter()
            .map(|(name, count)| match count {
                1 => format!("the {} row", name.as_str()),
                count => format!("the {count} {} rows", name.as_str()),
            })
            .collect();
        match rows.as_slice() {
            [rest @ .., last] if !rest.is_empty() => format!("{} and {last}", rest.join(", ")),
            _ => rows.concat(),
        }
    }
}

/// The kind of thing an act was on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A session plan's line for a fight: `plan line for Shrine of Cinders`.
    PlanLine,
    /// A session's recording: `recording for Session 1`.
    Recording,
    /// A scene of tonight's plan, by its words: `scene “Talk to Hobb”`.
    Scene,
    /// A creature from a stat block: `creature Cinder Wolf`.
    Creature,
    /// An NPC: `NPC Rook Ambry`.
    Npc,
    /// An entry of a tracker's history in `Trackers.md`, by its label: `entry “Take seal”`.
    Entry,
    /// A saved run's file, by its fight: `run of Bridge Ambush`.
    Run,
}

impl Kind {
    /// The thing in the words of an act: its kind, and its name.
    fn phrase(self, name: &Name) -> String {
        let name = name.as_str();
        match self {
            Self::PlanLine => format!("plan line for {name}"),
            Self::Recording => format!("recording for {name}"),
            Self::Scene => format!("scene “{name}”"),
            Self::Creature => format!("creature {name}"),
            Self::Npc => format!("NPC {name}"),
            Self::Entry => format!("entry “{name}”"),
            Self::Run => format!("run of {name}"),
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

    fn phrase(&self) -> String {
        self.kind.phrase(&self.name)
    }
}

/// The kind of file a cause or a recovery is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum File {
    /// A session's note.
    SessionNote,
    /// A session's recording file.
    Recording,
    /// The note of the session before this one, which Carry over reads.
    LastSessionNote,
    /// `Timeline.md`, which says which fight was played last.
    Timeline,
    /// `Trackers.md`, the trackers' history.
    Trackers,
    /// A saved run's file.
    RunFile,
    /// The house rule notes in play, which define the trackers.
    HouseRuleNote,
    /// `Campaign.md`, which lists the house rule notes in play (M-S0.5).
    CampaignNote,
}

impl File {
    /// The file as a sentence's first words: `The session note`, `Timeline.md`.
    fn subject(self) -> &'static str {
        match self {
            Self::SessionNote => "The session note",
            Self::Recording => "The recording file",
            Self::LastSessionNote => "The last session’s note",
            Self::Timeline => "Timeline.md",
            Self::Trackers => "Trackers.md",
            Self::RunFile => "The run file",
            Self::HouseRuleNote => "A house rule note",
            Self::CampaignNote => "Campaign.md",
        }
    }

    /// The file inside a sentence: `the session note`, `Timeline.md`.
    fn object(self) -> &'static str {
        match self {
            Self::SessionNote => "the session note",
            Self::Recording => "the recording file",
            Self::LastSessionNote => "the last session’s note",
            Self::Timeline => "Timeline.md",
            Self::Trackers => "Trackers.md",
            Self::RunFile => "the run file",
            Self::HouseRuleNote => "the house rule notes",
            Self::CampaignNote => "Campaign.md",
        }
    }

    /// What the DM opens: `that session’s notes`.
    fn opens(self) -> &'static str {
        match self {
            Self::SessionNote => "that session’s notes",
            Self::Recording => "that session’s recordings folder",
            Self::LastSessionNote => "the last session’s notes",
            Self::Timeline => "the Timeline page",
            Self::Trackers => "Trackers.md",
            Self::RunFile => "that session’s runs folder",
            Self::HouseRuleNote => "the house rule notes",
            Self::CampaignNote => "Campaign.md",
        }
    }
}

/// A folder an act writes into.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Folder {
    /// The campaign's run folder, where a saved run's file lives.
    Runs,
}

impl Folder {
    /// The folder inside a sentence: `the run folder`.
    fn object(self) -> &'static str {
        match self {
            Self::Runs => "the run folder",
        }
    }
}

/// A part of a tracker or of its history that an act needs as it was read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Part {
    /// `That entry`: the history entry the act is on.
    Entry,
    /// `The entry before it`: where an entry goes back.
    EntryBefore,
    /// `That notice`: the due notice the DM pressed, in `Trackers.md`.
    Notice,
    /// `That name`: a name of a tracker's table, in its house rule note.
    TrackerName,
    /// `That action`: a tracker's action, in its house rule note.
    Action,
    /// `That mark`: a mark of a tracker, in its house rule note.
    Mark,
    /// `That table`: a roll table, in its house rule note.
    Table,
    /// `The day count`: the days of the story, from the Timeline.
    DayCount,
}

impl Part {
    /// The part as a sentence's first words.
    fn subject(self) -> &'static str {
        match self {
            Self::Entry => "That entry",
            Self::EntryBefore => "The entry before it",
            Self::Notice => "That notice",
            Self::TrackerName => "That name",
            Self::Action => "That action",
            Self::Mark => "That mark",
            Self::Table => "That table",
            Self::DayCount => "The day count",
        }
    }

    /// Where it changed: on disk, or in its note.
    fn changed(self) -> &'static str {
        match self {
            Self::Entry | Self::EntryBefore | Self::Notice => "changed on disk",
            Self::TrackerName | Self::Action | Self::Mark | Self::Table => "changed in its note",
            Self::DayCount => "changed",
        }
    }
}

/// A limit the DM cannot change (rulings M13, M15): the DM's only choice is not to do the act,
/// so its answer ends at its cause, with no next step. Only these kinds can end so
/// (`Refusal::at_limit`); a fault in the tracker rules is a `Cause` with a real step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Limit {
    /// `Crowd Favor cannot go below 0.`: the tracker's `min`.
    Min { tracker: Name, min: i64 },
    /// `Each name has its roll.`: every name of the table was rolled.
    EveryNameRolled,
    /// M15: Continue fight, or the Undo of an ending, refused by the run rule after the fight's
    /// encounter was deleted, so no new run can start from it: `A later fight or rest was
    /// played, and its encounter was deleted.` Only while the encounter is gone; while it exists
    /// the refusal is `Refusal::new(…, Cause::LaterPlayed | Cause::NotOnTimeline,
    /// Recovery::StartNewRun)`.
    EncounterGone(RunRule),
}

/// The run rule that refused Continue fight or the Undo of an ending: Continue fight is only
/// for the fight played last, and only for a fight on the Timeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunRule {
    /// A later fight or rest was played.
    LaterPlayed,
    /// The fight is not on the Timeline.
    NotOnTimeline,
}

/// The empty next step of a refusal at a tracker's own limit. Only [`Refusal::at_limit`] makes
/// one, so no other cause can end with no step.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtLimit(());

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
    /// Nothing changed: `The recording for Session 1 was not deleted from Session 1: The
    /// Crossing.`
    NotDeleted { what: Thing, from: Name },
    /// Start fight: `No fight started from Shrine of Cinders.`
    NotStarted { encounter: Name },
    /// Continue fight: `Bridge Ambush did not go on.`
    NotContinued { fight: Name },
    /// The Undo of an ending, which also makes the fight live again: `The ending of Bridge
    /// Ambush was not undone.`
    EndingNotUndone { fight: Name },
    /// Carry over: `Nothing was carried over from Session 1 into Session 2.`
    NotCarried { from: Name, into: Name },
    /// The Undo of Carry over: `The carry-over from Session 1 was not undone in Session 2.`
    CarryNotUndone { from: Name, into: Name },
    /// A scene, a creature or an NPC: `The scene “Talk to Hobb” was not added to Session 2.`,
    /// `The creature Cinder Wolf was not added to Bridge Ambush.`
    NotAdded { what: Thing, to: Name },
    /// A scene ticked: `The scene “Talk to Hobb” was not ticked in Session 2.`
    NotTicked { what: Thing, within: Name },
    /// A scene's tick cleared: `The scene “Talk to Hobb” was not unticked in Session 2.`
    NotUnticked { what: Thing, within: Name },
    /// The Undo of a removal: `The entry “Take seal” was not put back into Trackers.md.`
    NotPutBack { what: Thing, into: Name },
    /// The plan's links: `No links to Hobb were changed.`
    LinksNotChanged { to: Name },
    /// The Undo of a links update: `The links to Hobb were not put back in Session 1.`
    LinksNotPutBack { to: Name, within: Name },
    /// A tracker's act: `Nothing was recorded for Take seal.`
    NotRecorded { act: Name },
    /// A tracker's roll: `Nothing was rolled on Wandering.`
    NotRolled { table: Name },
    /// The Undo of a tracker's act: `Nothing was undone for Take seal.`
    NotUndone { act: Name },
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
    /// `Splat could not change the run file.` An error: the system refused the write.
    NotChanged(File),
    /// `That line of the session note changed.` A warning: in Splat or on disk.
    LineChanged(File),
    /// `The session note changed after the act.` A warning: an Undo would lose that change.
    ChangedAfter(File),
    /// `What Carry over adds changed after it was shown.` A warning: the last session's note,
    /// its recordings or the plan changed.
    OfferChanged,
    /// `Splat cannot find the run folder.` A warning (R41: missing).
    FolderGone(Folder),
    /// `Splat cannot write in the run folder.` An error: the system refused the write.
    FolderNotWritable(Folder),
    /// `Another file now stands where the run file was.` A warning: Splat never writes over it.
    Occupied(File),
    /// `That entry changed on disk.` A warning.
    PartChanged(Part),
    /// `The entry before it is no longer in Trackers.md.` A warning.
    PartGone(Part, File),
    /// `Wandering roll depends on it.` A warning: another entry needs this one.
    DependsOn(Name),
    /// `The fight has ended.` A warning.
    FightEnded,
    /// `Cinder Wolf has no maximum HP.` A warning.
    NoMaxHp(Name),
    /// `Cinder Wolf has no initiative bonus.` A warning.
    NoInitiative(Name),
    /// `A later fight or rest was played.` A warning: Continue fight is for the last fight.
    LaterPlayed,
    /// `It is not on the Timeline.` A warning.
    NotOnTimeline,
    /// `The creature of the Cinder Wolf row was deleted.` A warning.
    CreatureDeleted(Rows),
    /// `Splat cannot read the file of Cinder Wolf.` An error (R41: unreadable).
    CreatureUnreadable(Name),
    /// `Two files claim the creature Cinder Wolf.` A warning, as Library shows it (issue #530).
    CreatureDuplicate(Name),
    /// `The package Ashen Marches of Cinder Wolf was not found.` A warning (R41: missing): the
    /// creature's file may still exist (issue #530).
    PackageNotFound { creature: Name, package: Name },
    /// Start's own checks of an encounter (issue #530, item 4), each in the words the editor
    /// already gives it. All are warnings (a rule of the app the act did not meet), except the
    /// roster rows Splat cannot read (an error, R41).
    /// `The encounter has no name.`
    EncounterUnnamed,
    /// `Read-aloud and notes has a Loot heading.`
    LootHeadingInNotes,
    /// `A side has no name.`
    SideUnnamed,
    /// `A hostile side is missing.`
    HostileSideMissing,
    /// `A side has no list of the sides it fights.`
    SideWithoutFoes,
    /// `The key of the Cinder Wolf row is missing or shared with another row.`
    RowKeyShared(Rows),
    /// `The side of the Cinder Wolf row is not in this encounter.`
    RowSideMissing(Rows),
    /// `The creature of the Cinder Wolf row is unavailable in campaign content.`
    CreatureUnavailable(Rows),
    /// `This encounter fills some rows from the campaign’s party.`
    PartyFromCampaign,
    /// `Row 3 needs one creature or one party member.`
    RowWithoutCreature(Name),
    /// `The starting HP of the Cinder Wolf row must be a whole number, 0 or more.`
    StartingNotWhole { rows: Rows, what: Name },
    /// `The changed side Raiders of the Cinder Wolf row is not a side of this encounter.`
    ChangedSideMissing { rows: Rows, side: Name },
    /// `The starting HP 40 of the Cinder Wolf row is above the changed maximum of 30.`
    StartingHpAboveMax { rows: Rows, hp: i64, max: i64 },
    /// `The changes of the Cinder Wolf row cannot stand.` Each change says why on its row.
    ChangesInvalid(Rows),
    /// `Splat cannot read the roster rows of Bridge Ambush.md.` An error (R41: unreadable).
    RosterUnreadable(Name),
    /// `Splat cannot read Bridge Ambush.md.` An error (R41: unreadable).
    EncounterUnreadable(Name),
    /// `Hobb’s note changed on disk or waits for a choice.` A warning.
    NeedsChoice(Name),
    /// `More than one note can answer to Hobb.` A warning.
    Ambiguous(Name),
    /// `Session 1 changed after its links were updated.` A warning: the Undo would lose that
    /// change.
    EditedSince(Name),
    /// `That notice is no longer due.` A warning.
    NoLongerDue,
    /// A fault in the tracker rules: `Rivals has no line for Party.` A warning.
    NoLine { tracker: Name, line: Name },
    /// A fault in the tracker rules: `No tracker in play has the key “rivals”.` A warning.
    UnknownTracker(Name),
    /// A fault in the tracker rules: `A change of Rivals has no add or set.` A warning.
    NoAmount(Name),
    /// A fault in the tracker rules: `Splat cannot read an amount of Rivals.` A warning.
    BadAmount(Name),
    /// A fault in the tracker rules: `No roll table has the key “gems”.` A warning.
    UnknownTable(Name),
    /// `The roll table “gems” cannot roll.` A warning: its dice or rows have a fault, which
    /// the problems list names.
    TableCannotRoll(Name),
    /// `The roll table “gems” has no row for 7.` A warning.
    NoRow { table: Name, total: i64 },
    /// `No amount was typed.` A warning: an action that asks for its amount got none.
    NoNumber,
    /// The tracker's own limit (M13): `Crowd Favor cannot go below 0.` A warning. With a step it
    /// is only extra words; with none, only `Refusal::at_limit` builds it.
    AtLimit(Limit),
}

/// A step in the app the DM takes before the act can work. The part writes its words.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step {
    /// `Choose which words to keep in the session note`
    ChooseWords(File),
    /// `Check the last session’s note`
    Check(File),
    /// `Fix Timeline.md on the Timeline page`
    FixTimeline,
    /// `Settle Timeline.md on the Timeline page`
    SettleTimeline,
    /// `Undo the ending`
    UndoEnding,
    /// `Choose a replacement for the Cinder Wolf row`
    ReplaceRows(Rows),
    /// `Fix Cinder Wolf in Library`
    FixInLibrary(Name),
    /// `Add the Ashen Marches package in Campaign → Packages`
    AddPackage(Name),
    /// `Enter an encounter name`
    EnterEncounterName,
    /// `Put those lines in the Loot list, or rename the heading`
    MoveLootLines,
    /// `Repair this encounter in Campaign → Campaign entries`
    RepairInEntries,
    /// `Edit those rows in Campaign → Campaign entries`
    EditPartyRows,
    /// `Fix the changes of the Cinder Wolf row in the roster`
    FixChanges(Rows),
    /// `Change the starting values of the Cinder Wolf row in the roster`
    FixStartingValues(Rows),
    /// `Fix the roster rows or remove them in Obsidian`
    FixRosterRows,
    /// `Remove Wandering roll`
    Remove(Name),
    /// `Resolve Hobb’s note`
    Resolve(Name),
    /// `Choose the group in each note`
    ChooseGroups,
    /// `Check what the Last time line offers now`
    CheckOffer,
    /// `Put the run folder back`
    PutFolderBack(Folder),
    /// `Fix the run folder outside Splat`
    FixOutside(Folder),
    /// `Move the other file away`
    MoveAside,
    /// A fault in the tracker rules, fixed where the tracker is defined: `Add Party to the per
    /// list of Rivals in its house rule note`
    AddLine { tracker: Name, line: Name },
    /// `Type the amount in the action’s field`
    TypeAmount,
}

impl Step {
    fn words(&self) -> String {
        match self {
            Self::ChooseWords(file) => format!("Choose which words to keep in {}", file.object()),
            Self::Check(file) => format!("Check {}", file.object()),
            Self::FixTimeline => "Fix Timeline.md on the Timeline page".to_owned(),
            Self::SettleTimeline => "Settle Timeline.md on the Timeline page".to_owned(),
            Self::UndoEnding => "Undo the ending".to_owned(),
            Self::ReplaceRows(rows) if rows.count() == 1 => {
                format!("Choose a replacement for {}", rows.words())
            }
            Self::ReplaceRows(rows) => format!("Choose replacements for {}", rows.words()),
            Self::FixInLibrary(name) => format!("Fix {} in Library", name.as_str()),
            Self::EnterEncounterName => "Enter an encounter name".to_owned(),
            Self::MoveLootLines => {
                "Put those lines in the Loot list, or rename the heading".to_owned()
            }
            Self::RepairInEntries => {
                "Repair this encounter in Campaign → Campaign entries".to_owned()
            }
            Self::EditPartyRows => "Edit those rows in Campaign → Campaign entries".to_owned(),
            Self::FixChanges(rows) => format!("Fix the changes of {} in the roster", rows.words()),
            Self::FixStartingValues(rows) => {
                format!(
                    "Change the starting values of {} in the roster",
                    rows.words()
                )
            }
            Self::FixRosterRows => "Fix the roster rows or remove them in Obsidian".to_owned(),
            Self::AddPackage(package) => {
                format!(
                    "Add the {} package in Campaign → Packages",
                    package.as_str()
                )
            }
            Self::Remove(name) => format!("Remove {}", name.as_str()),
            Self::Resolve(name) => format!("Resolve {}", name.as_str()),
            Self::ChooseGroups => "Choose the group in each note".to_owned(),
            Self::CheckOffer => "Check what the Last time line offers now".to_owned(),
            Self::PutFolderBack(folder) => format!("Put {} back", folder.object()),
            Self::FixOutside(folder) => format!("Fix {} outside Splat", folder.object()),
            Self::MoveAside => "Move the other file away".to_owned(),
            Self::AddLine { tracker, line } => format!(
                "Add {} to the per list of {} in its house rule note",
                line.as_str(),
                tracker.as_str()
            ),
            Self::TypeAmount => "Type the amount in the action’s field".to_owned(),
        }
    }
}

/// Another way to the same end, in the app.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Way {
    /// `Add it in Arrivals with its HP`
    ArrivalsWithHp,
    /// `Add it in Arrivals with an initiative total`
    ArrivalsWithInitiative,
}

impl Way {
    fn words(self) -> &'static str {
        match self {
            Self::ArrivalsWithHp => "Add it in Arrivals with its HP",
            Self::ArrivalsWithInitiative => "Add it in Arrivals with an initiative total",
        }
    }
}

/// The DM's next step: a bounded operation on a real target, whose words this part writes. A
/// refused act has one next step (ruling R45): the act it answers is not offered again; the DM
/// takes the step, and then the act's own control.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Recovery {
    /// `Open that session’s notes and remove the plan line there.`
    ByHand { file: File, change: Change },
    /// `Undo the ending first.`
    First(Step),
    /// `Add it in Arrivals with its HP instead.`
    Instead(Way),
    /// `Start a new run to play it again.` Continue fight is for the last fight played.
    StartNewRun,
    /// No next step, at the tracker's own limit (M13): only `Refusal::at_limit` makes it.
    AtLimit(AtLimit),
}

/// A refused act's answer: what was not done, why, and the next step. Its fields are private, it
/// has no `Default` and no conversion from text: [`Refusal::new`] takes all three parts. They
/// are held in one box, so a `Result` that carries a refusal stays one pointer wide whatever the
/// vocabulary grows to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal(Box<Parts>);

// A `Result` that carries a refusal stays small (Clippy's `result_large_err`).
const _: () = assert!(std::mem::size_of::<Refusal>() == std::mem::size_of::<usize>());

#[derive(Clone, Debug, PartialEq, Eq)]
struct Parts {
    act: FailedAct,
    cause: Cause,
    recovery: Recovery,
}

impl Refusal {
    pub fn new(act: FailedAct, cause: Cause, recovery: Recovery) -> Self {
        Self(Box::new(Parts {
            act,
            cause,
            recovery,
        }))
    }

    /// A refusal at the tracker's own limit (ruling M13): `Nothing was recorded for Lose favor.
    /// Crowd Favor cannot go below 0.` It ends at its cause: the DM's only choice is not to do
    /// the act. Its uses are counted (`design::results::STAGED`).
    pub fn at_limit(act: FailedAct, limit: Limit) -> Self {
        Self(Box::new(Parts {
            act,
            cause: Cause::AtLimit(limit),
            recovery: Recovery::AtLimit(AtLimit(())),
        }))
    }

    /// M15 (issue #507): Continue fight, or the Undo of an ending, refused by the run `rule`.
    /// While the fight's encounter exists, a new run from it plays the fight again
    /// (`Recovery::StartNewRun`); once the encounter is deleted no run can start from it, so the
    /// answer ends at its cause (`Limit::EncounterGone`). The one place that builds that limit, so
    /// it never stands while the encounter exists.
    pub fn run_rule(act: FailedAct, rule: RunRule, encounter_exists: bool) -> Self {
        if encounter_exists {
            let cause = match rule {
                RunRule::LaterPlayed => Cause::LaterPlayed,
                RunRule::NotOnTimeline => Cause::NotOnTimeline,
            };
            Self::new(act, cause, Recovery::StartNewRun)
        } else {
            Self::at_limit(act, Limit::EncounterGone(rule))
        }
    }

    /// The tone of its answer, from its diagnosis (R41): a file that cannot be read or written
    /// is an error; a file that changed, went or holds another note, and a rule of the app that
    /// the act did not meet, are a warning.
    pub fn tone(&self) -> Tone {
        match self.0.cause {
            Cause::Changed(_)
            | Cause::Conflict(_)
            | Cause::Gone(_)
            | Cause::Replaced(_)
            | Cause::LineChanged(_)
            | Cause::ChangedAfter(_)
            | Cause::OfferChanged
            | Cause::FolderGone(_)
            | Cause::Occupied(_)
            | Cause::PartChanged(_)
            | Cause::PartGone(..)
            | Cause::DependsOn(_)
            | Cause::FightEnded
            | Cause::NoMaxHp(_)
            | Cause::NoInitiative(_)
            | Cause::LaterPlayed
            | Cause::NotOnTimeline
            | Cause::CreatureDeleted(_)
            | Cause::CreatureDuplicate(_)
            | Cause::PackageNotFound { .. }
            | Cause::EncounterUnnamed
            | Cause::LootHeadingInNotes
            | Cause::SideUnnamed
            | Cause::HostileSideMissing
            | Cause::SideWithoutFoes
            | Cause::RowKeyShared(_)
            | Cause::RowSideMissing(_)
            | Cause::CreatureUnavailable(_)
            | Cause::PartyFromCampaign
            | Cause::RowWithoutCreature(_)
            | Cause::StartingNotWhole { .. }
            | Cause::ChangedSideMissing { .. }
            | Cause::StartingHpAboveMax { .. }
            | Cause::ChangesInvalid(_)
            | Cause::NeedsChoice(_)
            | Cause::Ambiguous(_)
            | Cause::EditedSince(_)
            | Cause::NoLongerDue
            | Cause::NoLine { .. }
            | Cause::UnknownTracker(_)
            | Cause::NoAmount(_)
            | Cause::BadAmount(_)
            | Cause::UnknownTable(_)
            | Cause::TableCannotRoll(_)
            | Cause::NoRow { .. }
            | Cause::NoNumber
            | Cause::AtLimit(_) => Tone::Warning,
            Cause::Unreadable(_)
            | Cause::NotSaved(_)
            | Cause::NotChanged(_)
            | Cause::FolderNotWritable(_)
            | Cause::CreatureUnreadable(_)
            | Cause::RosterUnreadable(_)
            | Cause::EncounterUnreadable(_) => Tone::Error,
        }
    }

    /// The whole answer: the act not done, the cause, the next step.
    pub fn words(&self) -> Sentence {
        self.words_told_apart(None)
    }

    /// The whole answer, its place told apart from another answer's by `apart`, joined by the
    /// middle dot as every told-apart place is (`Session 1: Road · Travel · 2`; R45: no brackets),
    /// when two answers would read alike.
    pub fn words_told_apart(&self, apart: Option<&Name>) -> Sentence {
        let place = |name: &Name| match apart {
            Some(apart) => format!("{} · {}", name.as_str(), apart.as_str()),
            None => name.as_str().to_owned(),
        };
        let act = match &self.0.act {
            FailedAct::NotRemoved { what, from } => crate::sentence!(
                "The {} was not removed from {}.",
                what.phrase(),
                place(from)
            ),
            FailedAct::NotDeleted { what, from } => crate::sentence!(
                "The {} was not deleted from {}.",
                what.phrase(),
                place(from)
            ),
            FailedAct::NotStarted { encounter } => {
                crate::sentence!("No fight started from {}.", place(encounter))
            }
            FailedAct::NotContinued { fight } => {
                crate::sentence!("{} did not go on.", place(fight))
            }
            FailedAct::EndingNotUndone { fight } => {
                crate::sentence!("The ending of {} was not undone.", place(fight))
            }
            FailedAct::NotCarried { from, into } => crate::sentence!(
                "Nothing was carried over from {} into {}.",
                from.as_str(),
                place(into)
            ),
            FailedAct::CarryNotUndone { from, into } => crate::sentence!(
                "The carry-over from {} was not undone in {}.",
                from.as_str(),
                place(into)
            ),
            FailedAct::NotAdded { what, to } => {
                crate::sentence!("The {} was not added to {}.", what.phrase(), place(to))
            }
            FailedAct::NotTicked { what, within } => {
                crate::sentence!("The {} was not ticked in {}.", what.phrase(), place(within))
            }
            FailedAct::NotUnticked { what, within } => crate::sentence!(
                "The {} was not unticked in {}.",
                what.phrase(),
                place(within)
            ),
            FailedAct::NotPutBack { what, into } => crate::sentence!(
                "The {} was not put back into {}.",
                what.phrase(),
                place(into)
            ),
            FailedAct::LinksNotChanged { to } => {
                crate::sentence!("No links to {} were changed.", place(to))
            }
            FailedAct::LinksNotPutBack { to, within } => crate::sentence!(
                "The links to {} were not put back in {}.",
                to.as_str(),
                place(within)
            ),
            FailedAct::NotRecorded { act } => {
                crate::sentence!("Nothing was recorded for {}.", place(act))
            }
            FailedAct::NotRolled { table } => {
                crate::sentence!("Nothing was rolled on {}.", place(table))
            }
            FailedAct::NotUndone { act } => {
                crate::sentence!("Nothing was undone for {}.", place(act))
            }
        };
        let cause = match &self.0.cause {
            Cause::Changed(file) => crate::sentence!("{} changed on disk.", file.subject()),
            Cause::Conflict(file) => crate::sentence!(
                "{} has unsaved words, and its file changed on disk.",
                file.subject()
            ),
            Cause::Gone(file) => crate::sentence!("{} is gone from its folder.", file.subject()),
            Cause::Replaced(file) => {
                crate::sentence!("The file of {} now holds another note.", file.object())
            }
            Cause::Unreadable(file) => crate::sentence!("Splat cannot read {}.", file.object()),
            Cause::NotSaved(file) => crate::sentence!("{} could not be saved.", file.subject()),
            Cause::NotChanged(file) => {
                crate::sentence!("Splat could not change {}.", file.object())
            }
            Cause::LineChanged(file) => {
                crate::sentence!("That line of {} changed.", file.object())
            }
            Cause::ChangedAfter(file) => {
                crate::sentence!("{} changed after the act.", file.subject())
            }
            Cause::OfferChanged => {
                crate::sentence!("What Carry over adds changed after it was shown.")
            }
            Cause::FolderGone(folder) => {
                crate::sentence!("Splat cannot find {}.", folder.object())
            }
            Cause::FolderNotWritable(folder) => {
                crate::sentence!("Splat cannot write in {}.", folder.object())
            }
            Cause::Occupied(file) => {
                crate::sentence!("Another file now stands where {} was.", file.object())
            }
            Cause::PartChanged(part) => {
                crate::sentence!("{} {}.", part.subject(), part.changed())
            }
            Cause::PartGone(part, file) => {
                crate::sentence!("{} is no longer in {}.", part.subject(), file.object())
            }
            Cause::DependsOn(other) => crate::sentence!("{} depends on it.", other.as_str()),
            Cause::FightEnded => crate::sentence!("The fight has ended."),
            Cause::NoMaxHp(creature) => {
                crate::sentence!("{} has no maximum HP.", creature.as_str())
            }
            Cause::NoInitiative(creature) => {
                crate::sentence!("{} has no initiative bonus.", creature.as_str())
            }
            Cause::LaterPlayed => crate::sentence!("A later fight or rest was played."),
            Cause::NotOnTimeline => crate::sentence!("It is not on the Timeline."),
            Cause::CreatureDeleted(rows) if rows.creatures() == 1 => {
                crate::sentence!("The creature of {} was deleted.", rows.words())
            }
            Cause::CreatureDeleted(rows) => {
                crate::sentence!("The creatures of {} were deleted.", rows.words())
            }
            Cause::CreatureUnreadable(creature) => {
                crate::sentence!("Splat cannot read the file of {}.", creature.as_str())
            }
            Cause::CreatureDuplicate(creature) => {
                crate::sentence!("Two files claim the creature {}.", creature.as_str())
            }
            Cause::PackageNotFound { creature, package } => crate::sentence!(
                "The package {} of {} was not found.",
                package.as_str(),
                creature.as_str()
            ),
            Cause::EncounterUnnamed => crate::sentence!("The encounter has no name."),
            Cause::LootHeadingInNotes => {
                crate::sentence!("Read-aloud and notes has a Loot heading.")
            }
            Cause::SideUnnamed => crate::sentence!("A side has no name."),
            Cause::HostileSideMissing => crate::sentence!("A hostile side is missing."),
            Cause::SideWithoutFoes => {
                crate::sentence!("A side has no list of the sides it fights.")
            }
            Cause::RowKeyShared(rows) => crate::sentence!(
                "The key of {} is missing or shared with another row.",
                rows.words()
            ),
            Cause::RowSideMissing(rows) => {
                crate::sentence!("The side of {} is not in this encounter.", rows.words())
            }
            Cause::CreatureUnavailable(rows) => crate::sentence!(
                "The creature of {} is unavailable in campaign content.",
                rows.words()
            ),
            Cause::PartyFromCampaign => {
                crate::sentence!("This encounter fills some rows from the campaign’s party.")
            }
            Cause::RowWithoutCreature(row) => {
                crate::sentence!("{} needs one creature or one party member.", row.as_str())
            }
            Cause::StartingNotWhole { rows, what } => crate::sentence!(
                "The starting {} of {} must be a whole number, 0 or more.",
                what.as_str(),
                rows.words()
            ),
            Cause::ChangedSideMissing { rows, side } => crate::sentence!(
                "The changed side {} of {} is not a side of this encounter.",
                side.as_str(),
                rows.words()
            ),
            Cause::StartingHpAboveMax { rows, hp, max } => crate::sentence!(
                "The starting HP {} of {} is above the changed maximum of {}.",
                hp,
                rows.words(),
                max
            ),
            Cause::ChangesInvalid(rows) => {
                crate::sentence!("The changes of {} cannot stand.", rows.words())
            }
            Cause::RosterUnreadable(file) => {
                crate::sentence!("Splat cannot read the roster rows of {}.", file.as_str())
            }
            Cause::EncounterUnreadable(file) => {
                crate::sentence!("Splat cannot read {}.", file.as_str())
            }
            Cause::NeedsChoice(note) => {
                crate::sentence!("{} changed on disk or waits for a choice.", note.as_str())
            }
            Cause::Ambiguous(name) => {
                crate::sentence!("More than one note can answer to {}.", name.as_str())
            }
            Cause::EditedSince(note) => {
                crate::sentence!("{} changed after its links were updated.", note.as_str())
            }
            Cause::NoLongerDue => crate::sentence!("That notice is no longer due."),
            Cause::NoLine { tracker, line } => {
                crate::sentence!("{} has no line for {}.", tracker.as_str(), line.as_str())
            }
            Cause::UnknownTracker(key) => {
                crate::sentence!("No tracker in play has the key “{}”.", key.as_str())
            }
            Cause::NoAmount(tracker) => {
                crate::sentence!("A change of {} has no add or set.", tracker.as_str())
            }
            Cause::BadAmount(tracker) => {
                crate::sentence!("Splat cannot read an amount of {}.", tracker.as_str())
            }
            Cause::UnknownTable(key) => {
                crate::sentence!("No roll table has the key “{}”.", key.as_str())
            }
            Cause::TableCannotRoll(key) => {
                crate::sentence!("The roll table “{}” cannot roll.", key.as_str())
            }
            Cause::NoRow { table, total } => {
                crate::sentence!(
                    "The roll table “{}” has no row for {}.",
                    table.as_str(),
                    total
                )
            }
            Cause::NoNumber => crate::sentence!("No amount was typed."),
            Cause::AtLimit(Limit::Min { tracker, min }) => {
                crate::sentence!("{} cannot go below {}.", tracker.as_str(), min)
            }
            Cause::AtLimit(Limit::EveryNameRolled) => crate::sentence!("Each name has its roll."),
            Cause::AtLimit(Limit::EncounterGone(RunRule::LaterPlayed)) => {
                crate::sentence!("A later fight or rest was played, and its encounter was deleted.")
            }
            Cause::AtLimit(Limit::EncounterGone(RunRule::NotOnTimeline)) => {
                crate::sentence!("It is not on the Timeline, and its encounter was deleted.")
            }
        };
        let next = match &self.0.recovery {
            Recovery::ByHand { file, change } => {
                crate::sentence!("Open {} and {} there.", file.opens(), change.words())
            }
            Recovery::First(step) => crate::sentence!("{} first.", step.words()),
            Recovery::Instead(way) => crate::sentence!("{} instead.", way.words()),
            Recovery::StartNewRun => crate::sentence!("Start a new run to play it again."),
            // M13: the answer ends at its cause.
            Recovery::AtLimit(_) => return act.then(cause),
        };
        act.then(cause).then(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(words: &str) -> Name {
        Name::new(words).unwrap()
    }

    /// Ruling M7: each family's act, causes and next step, in the part's words, with the tone
    /// of its cause (R41). One refusal per row; the plan line's own words are the doc test's.
    #[test]
    fn each_refused_act_says_what_was_not_done_why_and_the_next_step() {
        let session = || name("Session 2: The Crossing");
        let cases: Vec<(Refusal, &str, Tone)> = vec![
            // Start fight (E0).
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::CreatureDeleted(
                        Rows::new([
                            name("Cinder Wolf"),
                            name("Cinder Wolf"),
                            name("Ember Cultist"),
                        ])
                        .unwrap(),
                    ),
                    Recovery::First(Step::ReplaceRows(
                        Rows::new([name("Cinder Wolf"), name("Cinder Wolf")]).unwrap(),
                    )),
                ),
                "No fight started from Shrine of Cinders. The creatures of the 2 Cinder Wolf rows \
                 and the Ember Cultist row were deleted. Choose replacements for the 2 Cinder \
                 Wolf rows first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::CreatureUnreadable(name("Cinder Wolf")),
                    Recovery::First(Step::FixInLibrary(name("Cinder Wolf"))),
                ),
                "No fight started from Shrine of Cinders. Splat cannot read the file of Cinder \
                 Wolf. Fix Cinder Wolf in Library first.",
                Tone::Error,
            ),
            // Issue #530: two files claim the creature (a warning), and a package that is not
            // loaded (a warning, never "deleted").
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::CreatureDuplicate(name("Cinder Wolf")),
                    Recovery::First(Step::FixInLibrary(name("Cinder Wolf"))),
                ),
                "No fight started from Shrine of Cinders. Two files claim the creature Cinder \
                 Wolf. Fix Cinder Wolf in Library first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::PackageNotFound {
                        creature: name("Cinder Wolf"),
                        package: name("Ashen Marches"),
                    },
                    Recovery::First(Step::AddPackage(name("Ashen Marches"))),
                ),
                "No fight started from Shrine of Cinders. The package Ashen Marches of Cinder \
                 Wolf was not found. Add the Ashen Marches package in Campaign → Packages first.",
                Tone::Warning,
            ),
            // Issue #530 (item 4): Start's own checks, each a typed cause in the editor's words.
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::EncounterUnnamed,
                    Recovery::First(Step::EnterEncounterName),
                ),
                "No fight started from Shrine of Cinders. The encounter has no name. Enter an encounter name first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::LootHeadingInNotes,
                    Recovery::First(Step::MoveLootLines),
                ),
                "No fight started from Shrine of Cinders. Read-aloud and notes has a Loot heading. Put those lines in the Loot list, or rename the heading first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::SideUnnamed,
                    Recovery::First(Step::RepairInEntries),
                ),
                "No fight started from Shrine of Cinders. A side has no name. Repair this encounter in Campaign → Campaign entries first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::HostileSideMissing,
                    Recovery::First(Step::RepairInEntries),
                ),
                "No fight started from Shrine of Cinders. A hostile side is missing. Repair this encounter in Campaign → Campaign entries first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::SideWithoutFoes,
                    Recovery::First(Step::RepairInEntries),
                ),
                "No fight started from Shrine of Cinders. A side has no list of the sides it fights. Repair this encounter in Campaign → Campaign entries first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::RowKeyShared(Rows::new([name("Cinder Wolf")]).unwrap()),
                    Recovery::First(Step::RepairInEntries),
                ),
                "No fight started from Shrine of Cinders. The key of the Cinder Wolf row is missing or shared with another row. Repair this encounter in Campaign → Campaign entries first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::RowSideMissing(Rows::new([name("Cinder Wolf")]).unwrap()),
                    Recovery::First(Step::RepairInEntries),
                ),
                "No fight started from Shrine of Cinders. The side of the Cinder Wolf row is not in this encounter. Repair this encounter in Campaign → Campaign entries first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::CreatureUnavailable(Rows::new([name("Cinder Wolf")]).unwrap()),
                    Recovery::First(Step::ReplaceRows(Rows::new([name("Cinder Wolf")]).unwrap())),
                ),
                "No fight started from Shrine of Cinders. The creature of the Cinder Wolf row is unavailable in campaign content. Choose a replacement for the Cinder Wolf row first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::PartyFromCampaign,
                    Recovery::First(Step::EditPartyRows),
                ),
                "No fight started from Shrine of Cinders. This encounter fills some rows from the campaign’s party. Edit those rows in Campaign → Campaign entries first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::RowWithoutCreature(name("Row 3")),
                    Recovery::First(Step::RepairInEntries),
                ),
                "No fight started from Shrine of Cinders. Row 3 needs one creature or one party member. Repair this encounter in Campaign → Campaign entries first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::StartingNotWhole {
                        rows: Rows::new([name("Cinder Wolf")]).unwrap(),
                        what: name("HP"),
                    },
                    Recovery::First(Step::FixStartingValues(
                        Rows::new([name("Cinder Wolf")]).unwrap(),
                    )),
                ),
                "No fight started from Shrine of Cinders. The starting HP of the Cinder Wolf row must be a whole number, 0 or more. Change the starting values of the Cinder Wolf row in the roster first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::ChangedSideMissing {
                        rows: Rows::new([name("Cinder Wolf")]).unwrap(),
                        side: name("Raiders"),
                    },
                    Recovery::First(Step::FixChanges(Rows::new([name("Cinder Wolf")]).unwrap())),
                ),
                "No fight started from Shrine of Cinders. The changed side Raiders of the Cinder Wolf row is not a side of this encounter. Fix the changes of the Cinder Wolf row in the roster first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::StartingHpAboveMax {
                        rows: Rows::new([name("Cinder Wolf")]).unwrap(),
                        hp: 40,
                        max: 30,
                    },
                    Recovery::First(Step::FixStartingValues(
                        Rows::new([name("Cinder Wolf")]).unwrap(),
                    )),
                ),
                "No fight started from Shrine of Cinders. The starting HP 40 of the Cinder Wolf row is above the changed maximum of 30. Change the starting values of the Cinder Wolf row in the roster first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::ChangesInvalid(Rows::new([name("Cinder Wolf")]).unwrap()),
                    Recovery::First(Step::FixChanges(Rows::new([name("Cinder Wolf")]).unwrap())),
                ),
                "No fight started from Shrine of Cinders. The changes of the Cinder Wolf row cannot stand. Fix the changes of the Cinder Wolf row in the roster first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::RosterUnreadable(name("Bridge Ambush.md")),
                    Recovery::First(Step::FixRosterRows),
                ),
                "No fight started from Shrine of Cinders. Splat cannot read the roster rows of Bridge Ambush.md. Fix the roster rows or remove them in Obsidian first.",
                Tone::Error,
            ),
            (
                Refusal::new(
                    FailedAct::NotStarted {
                        encounter: name("Shrine of Cinders"),
                    },
                    Cause::EncounterUnreadable(name("Bridge Ambush.md")),
                    Recovery::First(Step::RepairInEntries),
                ),
                "No fight started from Shrine of Cinders. Splat cannot read Bridge Ambush.md. Repair this encounter in Campaign → Campaign entries first.",
                Tone::Error,
            ),
            // Continue fight (R0).
            (
                Refusal::new(
                    FailedAct::NotContinued {
                        fight: name("Bridge Ambush"),
                    },
                    Cause::Conflict(File::Timeline),
                    Recovery::First(Step::SettleTimeline),
                ),
                "Bridge Ambush did not go on. Timeline.md has unsaved words, and its file changed \
                 on disk. Settle Timeline.md on the Timeline page first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::EndingNotUndone {
                        fight: name("Bridge Ambush"),
                    },
                    Cause::LaterPlayed,
                    Recovery::StartNewRun,
                ),
                "The ending of Bridge Ambush was not undone. A later fight or rest was played. \
                 Start a new run to play it again.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotContinued {
                        fight: name("Bridge Ambush"),
                    },
                    Cause::Unreadable(File::Timeline),
                    Recovery::First(Step::FixTimeline),
                ),
                "Bridge Ambush did not go on. Splat cannot read Timeline.md. Fix Timeline.md on \
                 the Timeline page first.",
                Tone::Error,
            ),
            (
                Refusal::new(
                    FailedAct::NotContinued {
                        fight: name("Bridge Ambush"),
                    },
                    Cause::LaterPlayed,
                    Recovery::StartNewRun,
                ),
                "Bridge Ambush did not go on. A later fight or rest was played. Start a new run \
                 to play it again.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotContinued {
                        fight: name("Bridge Ambush"),
                    },
                    Cause::NotOnTimeline,
                    Recovery::StartNewRun,
                ),
                "Bridge Ambush did not go on. It is not on the Timeline. Start a new run to play \
                 it again.",
                Tone::Warning,
            ),
            // Carry over (N1).
            (
                Refusal::new(
                    FailedAct::NotCarried {
                        from: name("Session 1: The Road"),
                        into: session(),
                    },
                    Cause::OfferChanged,
                    Recovery::First(Step::CheckOffer),
                ),
                "Nothing was carried over from Session 1: The Road into Session 2: The Crossing. \
                 What Carry over adds changed after it was shown. Check what the Last time line \
                 offers now first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::CarryNotUndone {
                        from: name("Session 1: The Road"),
                        into: session(),
                    },
                    Cause::ChangedAfter(File::SessionNote),
                    Recovery::First(Step::Check(File::SessionNote)),
                ),
                "The carry-over from Session 1: The Road was not undone in Session 2: The \
                 Crossing. The session note changed after the act. Check the session note first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotCarried {
                        from: name("Session 1: The Road"),
                        into: session(),
                    },
                    Cause::Unreadable(File::LastSessionNote),
                    Recovery::First(Step::Check(File::LastSessionNote)),
                ),
                "Nothing was carried over from Session 1: The Road into Session 2: The Crossing. \
                 Splat cannot read the last session’s note. Check the last session’s note first.",
                Tone::Error,
            ),
            (
                Refusal::new(
                    FailedAct::NotCarried {
                        from: name("Session 1: The Road"),
                        into: session(),
                    },
                    Cause::Conflict(File::SessionNote),
                    Recovery::First(Step::ChooseWords(File::SessionNote)),
                ),
                "Nothing was carried over from Session 1: The Road into Session 2: The Crossing. \
                 The session note has unsaved words, and its file changed on disk. Choose which \
                 words to keep in the session note first.",
                Tone::Warning,
            ),
            // The plan's scenes (N1).
            (
                Refusal::new(
                    FailedAct::NotAdded {
                        what: Thing::new(Kind::Scene, name("Talk to Hobb")),
                        to: session(),
                    },
                    Cause::Gone(File::SessionNote),
                    Recovery::First(Step::Check(File::SessionNote)),
                ),
                "The scene “Talk to Hobb” was not added to Session 2: The Crossing. The session \
                 note is gone from its folder. Check the session note first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotTicked {
                        what: Thing::new(Kind::Scene, name("Talk to Hobb")),
                        within: session(),
                    },
                    Cause::LineChanged(File::SessionNote),
                    Recovery::First(Step::Check(File::SessionNote)),
                ),
                "The scene “Talk to Hobb” was not ticked in Session 2: The Crossing. That line of \
                 the session note changed. Check the session note first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotUnticked {
                        what: Thing::new(Kind::Scene, name("Talk to Hobb")),
                        within: session(),
                    },
                    Cause::LineChanged(File::SessionNote),
                    Recovery::First(Step::Check(File::SessionNote)),
                ),
                "The scene “Talk to Hobb” was not unticked in Session 2: The Crossing. That line \
                 of the session note changed. Check the session note first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRemoved {
                        what: Thing::new(Kind::Scene, name("Talk to Hobb")),
                        from: session(),
                    },
                    Cause::Replaced(File::SessionNote),
                    Recovery::First(Step::Check(File::SessionNote)),
                ),
                "The scene “Talk to Hobb” was not removed from Session 2: The Crossing. The file \
                 of the session note now holds another note. Check the session note first.",
                Tone::Warning,
            ),
            // A tracker's history (S0).
            (
                Refusal::new(
                    FailedAct::NotRemoved {
                        what: Thing::new(Kind::Entry, name("Take seal")),
                        from: name("Trackers.md"),
                    },
                    Cause::DependsOn(name("Wandering, 0:00 to 0:39 roll")),
                    Recovery::First(Step::Remove(name("Wandering, 0:00 to 0:39 roll"))),
                ),
                "The entry “Take seal” was not removed from Trackers.md. Wandering, 0:00 to 0:39 \
                 roll depends on it. Remove Wandering, 0:00 to 0:39 roll first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRemoved {
                        what: Thing::new(Kind::Entry, name("Take seal")),
                        from: name("Trackers.md"),
                    },
                    Cause::PartChanged(Part::Entry),
                    Recovery::First(Step::Check(File::Trackers)),
                ),
                "The entry “Take seal” was not removed from Trackers.md. That entry changed on \
                 disk. Check Trackers.md first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotPutBack {
                        what: Thing::new(Kind::Entry, name("Take seal")),
                        into: name("Trackers.md"),
                    },
                    Cause::PartGone(Part::EntryBefore, File::Trackers),
                    Recovery::First(Step::Check(File::Trackers)),
                ),
                "The entry “Take seal” was not put back into Trackers.md. The entry before it is \
                 no longer in Trackers.md. Check Trackers.md first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotPutBack {
                        what: Thing::new(Kind::Entry, name("Take seal")),
                        into: name("Trackers.md"),
                    },
                    Cause::PartChanged(Part::EntryBefore),
                    Recovery::First(Step::Check(File::Trackers)),
                ),
                "The entry “Take seal” was not put back into Trackers.md. The entry before it \
                 changed on disk. Check Trackers.md first.",
                Tone::Warning,
            ),
            // World's and Lookup's Add to fight (W0).
            (
                Refusal::new(
                    FailedAct::NotAdded {
                        what: Thing::new(Kind::Creature, name("Cinder Wolf")),
                        to: name("Bridge Ambush"),
                    },
                    Cause::NoMaxHp(name("Cinder Wolf")),
                    Recovery::Instead(Way::ArrivalsWithHp),
                ),
                "The creature Cinder Wolf was not added to Bridge Ambush. Cinder Wolf has no \
                 maximum HP. Add it in Arrivals with its HP instead.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotAdded {
                        what: Thing::new(Kind::Npc, name("Rook Ambry")),
                        to: name("Bridge Ambush"),
                    },
                    Cause::NoInitiative(name("Rook Ambry")),
                    Recovery::Instead(Way::ArrivalsWithInitiative),
                ),
                "The NPC Rook Ambry was not added to Bridge Ambush. Rook Ambry has no initiative \
                 bonus. Add it in Arrivals with an initiative total instead.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotAdded {
                        what: Thing::new(Kind::Creature, name("Cinder Wolf")),
                        to: name("Bridge Ambush"),
                    },
                    Cause::FightEnded,
                    Recovery::First(Step::UndoEnding),
                ),
                "The creature Cinder Wolf was not added to Bridge Ambush. The fight has ended. \
                 Undo the ending first.",
                Tone::Warning,
            ),
            // The plan's links and run files (N0, N1).
            (
                Refusal::new(
                    FailedAct::LinksNotChanged { to: name("Hobb") },
                    Cause::NeedsChoice(name("Session 1: The Road")),
                    Recovery::First(Step::Resolve(name("Session 1: The Road"))),
                ),
                "No links to Hobb were changed. Session 1: The Road changed on disk or waits for \
                 a choice. Resolve Session 1: The Road first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::LinksNotChanged { to: name("Hobb") },
                    Cause::Ambiguous(name("Hobb")),
                    Recovery::First(Step::ChooseGroups),
                ),
                "No links to Hobb were changed. More than one note can answer to Hobb. Choose the \
                 group in each note first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::LinksNotPutBack {
                        to: name("Hobb"),
                        within: name("Session 1: The Road"),
                    },
                    Cause::EditedSince(name("Session 1: The Road")),
                    Recovery::First(Step::Resolve(name("Session 1: The Road"))),
                ),
                "The links to Hobb were not put back in Session 1: The Road. Session 1: The Road \
                 changed after its links were updated. Resolve Session 1: The Road first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::LinksNotPutBack {
                        to: name("Hobb"),
                        within: name("Session 1: The Road"),
                    },
                    Cause::NotSaved(File::SessionNote),
                    Recovery::First(Step::Check(File::SessionNote)),
                ),
                "The links to Hobb were not put back in Session 1: The Road. The session note \
                 could not be saved. Check the session note first.",
                Tone::Error,
            ),
            (
                Refusal::new(
                    FailedAct::LinksNotPutBack {
                        to: name("Hobb"),
                        within: name("Session 1: The Road"),
                    },
                    Cause::Gone(File::SessionNote),
                    Recovery::First(Step::Check(File::SessionNote)),
                ),
                "The links to Hobb were not put back in Session 1: The Road. The session note is \
                 gone from its folder. Check the session note first.",
                Tone::Warning,
            ),
            // The save itself found an outside change: a conflict, a warning (review of
            // 1faf0142); a file that cannot be read is an error.
            (
                Refusal::new(
                    FailedAct::LinksNotPutBack {
                        to: name("Hobb"),
                        within: name("Session 1: The Road"),
                    },
                    Cause::Conflict(File::SessionNote),
                    Recovery::First(Step::ChooseWords(File::SessionNote)),
                ),
                "The links to Hobb were not put back in Session 1: The Road. The session note has \
                 unsaved words, and its file changed on disk. Choose which words to keep in the \
                 session note first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::LinksNotPutBack {
                        to: name("Hobb"),
                        within: name("Session 1: The Road"),
                    },
                    Cause::Unreadable(File::SessionNote),
                    Recovery::First(Step::Check(File::SessionNote)),
                ),
                "The links to Hobb were not put back in Session 1: The Road. Splat cannot read \
                 the session note. Check the session note first.",
                Tone::Error,
            ),
            (
                Refusal::new(
                    FailedAct::NotDeleted {
                        what: Thing::new(Kind::Run, name("Bridge Ambush")),
                        from: session(),
                    },
                    Cause::NotChanged(File::RunFile),
                    Recovery::First(Step::Check(File::RunFile)),
                ),
                "The run of Bridge Ambush was not deleted from Session 2: The Crossing. Splat \
                 could not change the run file. Check the run file first.",
                Tone::Error,
            ),
            (
                Refusal::new(
                    FailedAct::NotPutBack {
                        what: Thing::new(Kind::Run, name("Bridge Ambush")),
                        into: session(),
                    },
                    Cause::FolderGone(Folder::Runs),
                    Recovery::First(Step::PutFolderBack(Folder::Runs)),
                ),
                "The run of Bridge Ambush was not put back into Session 2: The Crossing. Splat \
                 cannot find the run folder. Put the run folder back first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotPutBack {
                        what: Thing::new(Kind::Run, name("Bridge Ambush")),
                        into: session(),
                    },
                    Cause::Occupied(File::RunFile),
                    Recovery::First(Step::MoveAside),
                ),
                "The run of Bridge Ambush was not put back into Session 2: The Crossing. Another \
                 file now stands where the run file was. Move the other file away first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotPutBack {
                        what: Thing::new(Kind::Run, name("Bridge Ambush")),
                        into: session(),
                    },
                    Cause::FolderNotWritable(Folder::Runs),
                    Recovery::First(Step::FixOutside(Folder::Runs)),
                ),
                "The run of Bridge Ambush was not put back into Session 2: The Crossing. Splat \
                 cannot write in the run folder. Fix the run folder outside Splat first.",
                Tone::Error,
            ),
            // A tracker's act, roll and Undo (S0, ruling M13). At the tracker's own limit the
            // answer ends at its cause; a fault in the tracker rules gets where to fix it.
            (
                Refusal::at_limit(
                    FailedAct::NotRecorded {
                        act: name("Lose favor"),
                    },
                    Limit::Min {
                        tracker: name("Crowd Favor"),
                        min: 0,
                    },
                ),
                "Nothing was recorded for Lose favor. Crowd Favor cannot go below 0.",
                Tone::Warning,
            ),
            (
                Refusal::at_limit(
                    FailedAct::NotRolled {
                        table: name("Wandering"),
                    },
                    Limit::EveryNameRolled,
                ),
                "Nothing was rolled on Wandering. Each name has its roll.",
                Tone::Warning,
            ),
            // M15 (#507): the run rule after the fight's encounter was deleted ends at its cause
            // too; while the encounter exists, a new run is the next step. `run_rule` decides.
            (
                Refusal::run_rule(
                    FailedAct::NotContinued {
                        fight: name("Bridge Ambush"),
                    },
                    RunRule::LaterPlayed,
                    false,
                ),
                "Bridge Ambush did not go on. A later fight or rest was played, and its \
                 encounter was deleted.",
                Tone::Warning,
            ),
            (
                Refusal::run_rule(
                    FailedAct::EndingNotUndone {
                        fight: name("Bridge Ambush"),
                    },
                    RunRule::NotOnTimeline,
                    false,
                ),
                "The ending of Bridge Ambush was not undone. It is not on the Timeline, and its \
                 encounter was deleted.",
                Tone::Warning,
            ),
            (
                Refusal::run_rule(
                    FailedAct::NotContinued {
                        fight: name("Bridge Ambush"),
                    },
                    RunRule::LaterPlayed,
                    true,
                ),
                "Bridge Ambush did not go on. A later fight or rest was played. Start a new run \
                 to play it again.",
                Tone::Warning,
            ),
            (
                Refusal::run_rule(
                    FailedAct::EndingNotUndone {
                        fight: name("Bridge Ambush"),
                    },
                    RunRule::NotOnTimeline,
                    true,
                ),
                "The ending of Bridge Ambush was not undone. It is not on the Timeline. Start a \
                 new run to play it again.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRecorded {
                        act: name("Rivals +1"),
                    },
                    Cause::NoLine {
                        tracker: name("Rivals"),
                        line: name("Party"),
                    },
                    Recovery::First(Step::AddLine {
                        tracker: name("Rivals"),
                        line: name("Party"),
                    }),
                ),
                "Nothing was recorded for Rivals +1. Rivals has no line for Party. Add Party to \
                 the per list of Rivals in its house rule note first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRecorded {
                        act: name("Rivals +1"),
                    },
                    Cause::UnknownTracker(name("rivals")),
                    Recovery::First(Step::Check(File::HouseRuleNote)),
                ),
                "Nothing was recorded for Rivals +1. No tracker in play has the key “rivals”. \
                 Check the house rule notes first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRecorded {
                        act: name("Rivals +1"),
                    },
                    Cause::NoAmount(name("Rivals")),
                    Recovery::First(Step::Check(File::HouseRuleNote)),
                ),
                "Nothing was recorded for Rivals +1. A change of Rivals has no add or set. Check \
                 the house rule notes first.",
                Tone::Warning,
            ),
            // M-S0.5 (PR 661 review): `Campaign.md` that cannot be read, named as itself.
            (
                Refusal::new(
                    FailedAct::NotRecorded {
                        act: name("Take seal"),
                    },
                    Cause::Unreadable(File::CampaignNote),
                    Recovery::First(Step::Check(File::CampaignNote)),
                ),
                "Nothing was recorded for Take seal. Splat cannot read Campaign.md. Check \
                 Campaign.md first.",
                Tone::Error,
            ),
            // M-S0.3 (ruling M21): the tracker causes that had only free words.
            (
                Refusal::new(
                    FailedAct::NotRecorded {
                        act: name("Rivals +1d4"),
                    },
                    Cause::BadAmount(name("Rivals")),
                    Recovery::First(Step::Check(File::HouseRuleNote)),
                ),
                "Nothing was recorded for Rivals +1d4. Splat cannot read an amount of Rivals. \
                 Check the house rule notes first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRolled {
                        table: name("gems"),
                    },
                    Cause::UnknownTable(name("gems")),
                    Recovery::First(Step::Check(File::HouseRuleNote)),
                ),
                "Nothing was rolled on gems. No roll table has the key “gems”. Check the house \
                 rule notes first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRolled {
                        table: name("gems"),
                    },
                    Cause::TableCannotRoll(name("gems")),
                    Recovery::First(Step::Check(File::HouseRuleNote)),
                ),
                "Nothing was rolled on gems. The roll table “gems” cannot roll. Check the house \
                 rule notes first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRolled {
                        table: name("gems"),
                    },
                    Cause::NoRow {
                        table: name("gems"),
                        total: 7,
                    },
                    Recovery::First(Step::Check(File::HouseRuleNote)),
                ),
                "Nothing was rolled on gems. The roll table “gems” has no row for 7. Check the \
                 house rule notes first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRecorded { act: name("Bet") },
                    Cause::NoNumber,
                    Recovery::First(Step::TypeAmount),
                ),
                "Nothing was recorded for Bet. No amount was typed. Type the amount in the \
                 action’s field first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRecorded {
                        act: name("Feast day"),
                    },
                    Cause::NoLongerDue,
                    Recovery::First(Step::Check(File::Trackers)),
                ),
                "Nothing was recorded for Feast day. That notice is no longer due. Check \
                 Trackers.md first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRecorded {
                        act: name("Feast day"),
                    },
                    Cause::PartChanged(Part::Mark),
                    Recovery::First(Step::Check(File::HouseRuleNote)),
                ),
                "Nothing was recorded for Feast day. That mark changed in its note. Check the \
                 house rule notes first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRecorded {
                        act: name("Feast day"),
                    },
                    Cause::PartChanged(Part::DayCount),
                    Recovery::First(Step::Check(File::Timeline)),
                ),
                "Nothing was recorded for Feast day. The day count changed. Check Timeline.md \
                 first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotRecorded {
                        act: name("Feast day"),
                    },
                    Cause::Unreadable(File::Timeline),
                    Recovery::First(Step::FixTimeline),
                ),
                "Nothing was recorded for Feast day. Splat cannot read Timeline.md. Fix \
                 Timeline.md on the Timeline page first.",
                Tone::Error,
            ),
            (
                Refusal::new(
                    FailedAct::NotRecorded {
                        act: name("Take seal"),
                    },
                    Cause::Changed(File::HouseRuleNote),
                    Recovery::First(Step::Check(File::HouseRuleNote)),
                ),
                "Nothing was recorded for Take seal. A house rule note changed on disk. Check the \
                 house rule notes first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotUndone {
                        act: name("Take seal"),
                    },
                    Cause::Changed(File::Trackers),
                    Recovery::First(Step::Check(File::Trackers)),
                ),
                "Nothing was undone for Take seal. Trackers.md changed on disk. Check Trackers.md \
                 first.",
                Tone::Warning,
            ),
            (
                Refusal::new(
                    FailedAct::NotUndone {
                        act: name("Take seal"),
                    },
                    Cause::DependsOn(name("Wandering, 0:40 to 1:19 roll")),
                    Recovery::First(Step::Remove(name("Wandering, 0:40 to 1:19 roll"))),
                ),
                "Nothing was undone for Take seal. Wandering, 0:40 to 1:19 roll depends on it. \
                 Remove Wandering, 0:40 to 1:19 roll first.",
                Tone::Warning,
            ),
        ];
        // Every row is checked, and every wrong row is named, so one change shows all it breaks.
        let wrong: Vec<String> = cases
            .iter()
            .filter(|(refusal, words, tone)| {
                refusal.words().as_str() != *words || refusal.tone() != *tone
            })
            .map(|(refusal, words, tone)| {
                format!(
                    "{words:?} ({tone:?}): said {:?} ({:?})",
                    refusal.words().as_str(),
                    refusal.tone()
                )
            })
            .collect();
        assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    }
}
