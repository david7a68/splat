//! Trackers (F4 of the session-cycle design), the slice between fights: house rule notes in
//! play, their trackers and marks, the ledger's replay into values and the Due list, and the
//! acts that write the ledger. Splat counts and shows; a mark raises a notice and never acts.
//!
//! One owner for each fact: a definition is in its house rule note, a value and a notice are
//! entries of `Trackers.md`, and nothing else remembers an act.
#[path = "trackers/history.rs"]
pub mod history;
#[path = "trackers/view.rs"]
pub mod view;

use crate::design::outcome::Sentence;
use ledger::{Entry, Fault, Ledger, NoticeRef, Op, Roll};
pub use splat_campaign::trackers::*;
use splat_campaign::{campaign::Campaign, timeline};
use splat_content::content::Entry as Note;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};
use uuid::Uuid;

/// The ledger is read again this often, so an edit in Obsidian shows.
const CHECK_DISK_EVERY: Duration = Duration::from_secs(1);
/// Today's `Undo add` of an event keeps its 30 seconds.
pub const UNDO_ADD_FOR: Duration = Duration::from_secs(30);

/// The part of the Timeline that determines when day marks are due.
pub struct Clock<'a> {
    pub changes: u64,
    pub conflict: bool,
    pub moved_under_edit: Option<&'a str>,
    pub unreadable: Option<&'a str>,
    pub events: &'a [timeline::Event],
}

/// The house rule notes in play as the files hold them now: the list in the campaign's manifest
/// (`Campaign.md`, at `manifest` in its folder), and each note of the campaign read from its
/// file, where the content found it. A package's note is the content's. This is the one read of
/// the definitions: when the content changes, and at each act, so that an act never uses a
/// definition that changed on disk. It reads only those files, never the whole folder.
pub fn rules_on_disk(campaign: &Campaign, manifest: &str, index: &dyn RuleSource) -> Rules {
    let read = || -> Result<Vec<Uuid>, String> {
        let text = std::fs::read_to_string(campaign.path.join(manifest))
            .map_err(|error| format!("{error}."))?;
        let entry = Note::parse(manifest, &text)?;
        if entry.kind != "campaign" || entry.id != campaign.id {
            return Err("it is not this campaign’s manifest.".into());
        }
        read_in_play(entry.fields.get("splat_in_play"))
    };
    let ids = match read() {
        Ok(ids) => ids,
        Err(error) if error.starts_with("splat_in_play") => {
            return Rules {
                problems: vec![(None, error)],
                ..Rules::default()
            };
        }
        Err(error) => {
            return Rules::cannot_read(
                RulesFile::Manifest,
                format!("{manifest} cannot be read: {error}"),
            );
        }
    };
    if ids.is_empty() {
        return Rules::default();
    }
    let mut notes = Vec::new();
    for id in ids {
        let Some((entry, is_campaign_note)) = index.note(id) else {
            notes.push(Err((
                id,
                format!(
                    "A house rule note in play is not found: {id}. Switch it off in Campaign → Rules and runs."
                ),
            )));
            continue;
        };
        if !is_campaign_note {
            notes.push(Ok(entry.clone()));
            continue;
        }
        let path = &entry.path;
        let moved = format!(
            "{} is not where it was. Splat reads the notes again when its window gets focus.",
            entry.name
        );
        match std::fs::read_to_string(campaign.path.join(path)) {
            Ok(text) => match Note::parse(path, &text) {
                Ok(entry) if entry.id == id => notes.push(Ok(entry)),
                Ok(_) => notes.push(Err((id, moved))),
                Err(error) => {
                    return Rules::cannot_read(
                        RulesFile::HouseRuleNote,
                        format!("{path} cannot be read: {error}"),
                    );
                }
            },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                notes.push(Err((id, moved)))
            }
            Err(error) => {
                return Rules::cannot_read(
                    RulesFile::HouseRuleNote,
                    format!("{path} cannot be read: {error}."),
                );
            }
        }
    }
    read_rules(index.loot_sources(), notes)
}

/// The one answer to "what is the day count now, or why is it not known": the view, the rail's
/// count and every act read it. Not known while `Timeline.md` cannot be read, while it changed
/// or cannot be read under an edit that waits to be saved, and while that waits for the DM's
/// choice; then no day notice shows, and the Timeline keeps the draft.
pub fn today(book: &Clock<'_>) -> Today {
    let why = book
        .unreadable
        .or(book.moved_under_edit)
        .or_else(|| book.conflict.then_some(timeline::CHANGED_UNDER_DRAFT));
    match why {
        Some(why) => Today::NotRead(why.to_owned()),
        None => Today::Day(timeline::day_now(book.events).unwrap_or(0)),
    }
}

/// The problem's words, at the line that leaves the actions out.
pub fn repeat_words(repeat: &Repeat) -> String {
    match repeat {
        Repeat::Line { word, .. } => crate::sentence!(
            "Two lines are named “{}”, and an action asks for an amount. Give each line its own name.",
            word
        )
        .to_string(),
        Repeat::Label { label, .. } => crate::sentence!(
            "Two actions are named “{}”, and one asks for an amount. Give each action its own name.",
            label
        )
        .to_string(),
    }
}

/// `"1d6: [4]"` reads as `Rolled 4 on 1d6`.
fn rolled(text: &str) -> String {
    match text.split_once(": [") {
        Some((dice, faces)) => {
            let total: i64 = faces
                .trim_end_matches(']')
                .split(',')
                .filter_map(|face| face.trim().parse::<i64>().ok())
                .sum();
            format!("Rolled {total} on {dice}")
        }
        None => format!("Rolled {text}"),
    }
}

/// The result line's words for one act, from the entries alone, so they read the same after a
/// restart.
pub fn result_words(rules: &Rules, ledger: &Ledger, state: &State, entries: &[Entry]) -> Sentence {
    let mut words = act_words(rules, ledger, state, entries);
    // Every way a notice leaves the Due list by a fall says so once: a press, a linked change,
    // or a ticked change of `Done`.
    for entry in entries {
        for at in state.lowered(&entry.id) {
            let name = short_name(
                rules,
                entry.tracker.as_deref().unwrap_or_default(),
                entry.instance.as_deref(),
                *at,
            );
            words = words.then(crate::sentence!("{} is no longer due.", name));
        }
    }
    words
}

/// The act's own words: the notice it ended or rolled on, or the values it left.
#[allow(
    clippy::disallowed_methods,
    reason = "a roll table’s row is the DM’s own words"
)]
fn act_words(rules: &Rules, ledger: &Ledger, state: &State, entries: &[Entry]) -> Sentence {
    let label = label(rules, ledger, entries);
    // An act on a notice says what happened in a sentence; `Undo: <label>` keeps the label.
    if let Some(entry) = head(entries).filter(|e| e.notice.is_some())
        && let Some(notice) = &entry.notice
    {
        let short = notice_name(rules, ledger, notice);
        let ends = entries.iter().filter(|e| e.dismissed).count();
        if let (NoticeRef::Day { mark, .. }, true) = (notice, entry.dismissed && ends > 1) {
            return crate::sentence!("{} is dismissed for {ends} days.", rules.day_title(mark));
        }
        if entry.done {
            return crate::sentence!("{} is done.", short);
        }
        if entry.dismissed {
            return crate::sentence!("{} is dismissed.", short);
        }
        if let Some(Roll::Table(roll)) = &entry.roll {
            let name = roll
                .name
                .as_ref()
                .map_or(String::new(), |n| format!(" for {n}"));
            // The row's words are the DM's own table text.
            let table = rules.table_name(&roll.table);
            return Sentence::outside(format!(
                "Rolled {} on {table}{name}: {}",
                roll.value,
                roll.row.trim()
            ))
            .unwrap_or_else(|| crate::sentence!("Rolled {} on {table}{name}.", roll.value));
        }
    }
    // Each value it left is a sentence of its own; `Undo: <label>` below names the act.
    let values = entries
        .iter()
        .filter(|entry| entry.notice.is_none())
        .filter_map(|entry| {
            let after = state.value_after(&entry.id)?;
            Some(crate::sentence!(
                "{} changed to {after}.",
                line_name(rules, entry.tracker.as_deref()?, entry.instance.as_deref())
            ))
        })
        .reduce(Sentence::then);
    let mut words = values.unwrap_or_else(|| crate::sentence!("{}.", label));
    for entry in entries {
        if let Some(Roll::Dice(text)) = &entry.roll {
            words = words.then(match &entry.row {
                // The row's words are the DM's own table text.
                Some(row) => Sentence::outside(format!("{}: {row}", rolled(text)))
                    .unwrap_or_else(|| crate::sentence!("{}.", rolled(text))),
                None => crate::sentence!("{}.", rolled(text)),
            });
        }
    }
    let raised: Vec<(String, i64)> = entries
        .iter()
        .flat_map(|entry| {
            state.raised_by(&entry.id).iter().map(|at| {
                let name = short_name(
                    rules,
                    entry.tracker.as_deref().unwrap_or_default(),
                    entry.instance.as_deref(),
                    *at,
                );
                (name, *at)
            })
        })
        .collect();
    // Two notices with one name say their marks: `Bell at 20 and Bell at 40 are due.`
    let due: Vec<String> = raised
        .iter()
        .map(|(name, at)| {
            if raised.iter().filter(|(other, _)| other == name).count() > 1 {
                format!("{name} at {at}")
            } else {
                name.clone()
            }
        })
        .collect();
    match due.as_slice() {
        [] => {}
        [one] => words = words.then(crate::sentence!("{} is due.", one)),
        [many @ .., last] => {
            words = words.then(crate::sentence!("{} and {last} are due.", many.join(", ")));
        }
    }
    words
}

/// A tracker's line in a sentence: `Seals for Party`, or the tracker's name alone.
fn line_name(rules: &Rules, key: &str, instance: Option<&str>) -> String {
    let name = rules.def(key).map_or(key, |def| def.name.as_str());
    match instance {
        Some(instance) => format!("{name} for {instance}"),
        None => name.to_owned(),
    }
}

// ---------------------------------------------------------------------------------------------
// Rolls

/// The campaign and the session being played, which own the newest act's result. A campaign
/// switch hides the result; switching back shows it again.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Played {
    root: PathBuf,
    session: Option<Uuid>,
}

impl crate::design::results::Owner for Played {
    type Scope = std::path::Path;
    fn scope(&self) -> &std::path::Path {
        &self.root
    }
}

/// The newest act of the played session, kept for its footer order: its owner, and the ids of
/// its entries, which tell one act from another. Its words are worked out with the ledger
/// (`Trackers::result`).
pub(crate) struct NewestAct(Played, Vec<String>);

impl crate::design::results::Kept for NewestAct {
    type Owner = Played;
    const ENDS: crate::design::results::Ends = crate::design::results::Ends {
        // No newest act left (all undone): it ends.
        owner_changed: crate::design::results::OwnerChanged::Ends,
        // A read of the ledger works the newest act out again (`settle_result`).
        disk_changed: crate::design::results::DiskChanged::Stays,
        // Undo takes the act out of the ledger; the next act, if any, is recorded then.
        press: crate::design::results::Press::Undo,
    };
    fn owner(&self) -> Played {
        self.0.clone()
    }
}

/// A fault of the trackers' files or of a write, by kind. The kind decides its severity in one
/// `match` (`Fault::severity`), and its words in one place, so every view that draws it (the
/// Trackers view, a house rule note's reader, the footer) gives it one tone and one wording.
#[derive(Clone, Debug, PartialEq)]
pub enum TrackerFault {
    /// `Trackers.md` cannot be read: the system's words.
    LedgerUnreadable(String),
    /// `Campaign.md` or a house rule note in play cannot be read: the reader's words.
    NotesUnreadable(String),
    /// The day count is not known: the Timeline's words.
    ClockNotRead(String),
    /// A write of `Trackers.md` failed: the system's words.
    NotSaved(String),
    /// The Timeline did not save the event that `Add to Timeline` made: its words.
    TimelineNotSaved(String),
    /// A problem of the problems list: a member or a note the DM must fix before it counts.
    Problem(String),
    /// An act that wrote nothing (ruling M7): the act, its cause and the next step, in the tone
    /// of its cause.
    Refused(crate::design::results::Refusal),
}

impl crate::design::outcome::Fault for TrackerFault {
    fn severity(&self) -> crate::design::outcome::Severity {
        use crate::design::outcome::Severity;
        match self {
            Self::LedgerUnreadable(_) | Self::NotesUnreadable(_) | Self::ClockNotRead(_) => {
                Severity::CannotRead
            }
            Self::NotSaved(_) | Self::TimelineNotSaved(_) => Severity::NotSaved,
            Self::Problem(_) => Severity::NeedsFixing,
            Self::Refused(refusal) => match refusal.tone() {
                crate::design::Tone::Error => Severity::CannotRead,
                _ => Severity::NeedsFixing,
            },
        }
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "the faults carry the system’s and the Timeline’s words"
    )]
    fn cause(&self) -> Sentence {
        use crate::sentence;
        // Each kind's words come from outside the app; blank words fall back to the app's own.
        let outside = |words: String, blank: Sentence| Sentence::outside(words).unwrap_or(blank);
        match self {
            Self::LedgerUnreadable(error) => outside(
                format!("{} cannot be read: {error}", ledger::FILE),
                sentence!("{} cannot be read.", ledger::FILE),
            )
            .then(sentence!(
                "The history in the file is safe. Splat writes nothing to it until it can be read."
            )),
            Self::NotesUnreadable(error) => outside(
                error.clone(),
                sentence!("A house rule note cannot be read."),
            )
            .then(sentence!(
                "Splat shows no tracker and records nothing until it can be read. It reads the notes again when its window gets focus."
            )),
            Self::ClockNotRead(why) => outside(
                why.clone(),
                sentence!("The Timeline cannot be read."),
            )
            .then(view::DAY_MARKS_WAIT),
            Self::NotSaved(error) => outside(
                format!("{} was not saved: {}", ledger::FILE, error.trim_end_matches('.')),
                sentence!("{} was not saved.", ledger::FILE),
            )
            .then(sentence!("Nothing was recorded.")),
            Self::TimelineNotSaved(words) => {
                outside(words.clone(), sentence!("The timeline was not saved."))
            }
            Self::Problem(problem) => {
                outside(problem.clone(), sentence!("A tracker cannot be used."))
            }
            Self::Refused(refusal) => refusal.words(),
        }
    }
}

/// Why the last act wrote nothing. It shows on the result line, at the bottom edge of the view,
/// until the next act or until the ledger is read again: never beside a row that can go away.
#[derive(Clone, Debug, PartialEq)]
pub enum Unrecorded {
    /// The act does not fit the files on disk now: what was not done, why, and the next step.
    Refused(crate::design::results::Refusal),
    /// The save failed. Nothing waits: the DM presses again.
    NotSaved(String),
}

/// A tracker name, label or key as the refusal names it; one with no words is `that tracker`.
pub fn named(words: &str) -> crate::design::results::Name {
    use crate::design::results::{Fallback, Name};
    Name::new(words).unwrap_or_else(|| Name::fallback(Fallback::ThatTracker))
}

/// The bounded answer to an act that wrote nothing (rulings M7, M13): `act` with the domain's
/// cause, and the one next step. At the tracker's own limit it ends at its cause; a fault in the
/// tracker rules sends the DM to the house rule notes; a stale file to the file it read.
pub fn refusal(
    act: crate::design::results::FailedAct,
    cause: splat_campaign::trackers::Refused,
) -> crate::design::results::Refusal {
    use crate::design::results::{Cause, File, Limit, Part as P, Recovery, Refusal, Step};
    use splat_campaign::trackers::{Part, Refused};
    let check = |file| Recovery::First(Step::Check(file));
    let rules = || check(File::HouseRuleNote);
    let part = |part: Part| match part {
        Part::Notice => (P::Notice, File::Trackers),
        Part::Mark => (P::Mark, File::HouseRuleNote),
        Part::Table => (P::Table, File::HouseRuleNote),
        Part::Name => (P::TrackerName, File::HouseRuleNote),
        Part::Action => (P::Action, File::HouseRuleNote),
        Part::Entry => (P::Entry, File::Trackers),
        Part::EntryBefore => (P::EntryBefore, File::Trackers),
        Part::DayCount => (P::DayCount, File::Timeline),
    };
    let (cause, recovery) = match cause {
        Refused::BelowMin { tracker, min } => {
            return Refusal::at_limit(
                act,
                Limit::Min {
                    tracker: named(&tracker),
                    min,
                },
            );
        }
        Refused::EveryNameRolled => return Refusal::at_limit(act, Limit::EveryNameRolled),
        Refused::NoLine {
            tracker,
            line: Some(line),
        } => (
            Cause::NoLine {
                tracker: named(&tracker),
                line: named(&line),
            },
            Recovery::First(Step::AddLine {
                tracker: named(&tracker),
                line: named(&line),
            }),
        ),
        Refused::NoLine { line: None, .. } => (Cause::PartChanged(P::Action), rules()),
        Refused::UnknownTracker(key) => (Cause::UnknownTracker(named(&key)), rules()),
        Refused::NoAmount(tracker) => (Cause::NoAmount(named(&tracker)), rules()),
        Refused::BadAmount(tracker) => (Cause::BadAmount(named(&tracker)), rules()),
        Refused::UnknownTable(key) => (Cause::UnknownTable(named(&key)), rules()),
        Refused::TableCannotRoll(key) => (Cause::TableCannotRoll(named(&key)), rules()),
        Refused::NoRow { table, total } => (
            Cause::NoRow {
                table: named(&table),
                total,
            },
            rules(),
        ),
        Refused::NoNumber => (Cause::NoNumber, Recovery::First(Step::TypeAmount)),
        Refused::Changed(changed) => {
            let (changed, file) = part(changed);
            (Cause::PartChanged(changed), check(file))
        }
        Refused::NoLongerDue => (Cause::NoLongerDue, check(File::Trackers)),
        Refused::Gone(gone) => (
            Cause::PartGone(part(gone).0, File::Trackers),
            check(File::Trackers),
        ),
        Refused::LedgerChanged => (Cause::Changed(File::Trackers), check(File::Trackers)),
        Refused::RulesChanged => (Cause::Changed(File::HouseRuleNote), rules()),
        // The file that cannot be read, and the next step there (PR 661 review, P2).
        Refused::RulesUnreadable(RulesFile::Manifest) => (
            Cause::Unreadable(File::CampaignNote),
            check(File::CampaignNote),
        ),
        Refused::RulesUnreadable(RulesFile::HouseRuleNote) => {
            (Cause::Unreadable(File::HouseRuleNote), rules())
        }
        Refused::TimelineUnreadable => (
            Cause::Unreadable(File::Timeline),
            Recovery::First(Step::FixTimeline),
        ),
        Refused::DependsOn(name) => (
            Cause::DependsOn(named(&name)),
            Recovery::First(Step::Remove(named(&name))),
        ),
    };
    Refusal::new(act, cause, recovery)
}

/// `Add to Timeline` made this event for this entry. Its `Undo add` keeps today's 30 seconds.
#[derive(Clone, Debug)]
pub struct Added {
    pub entry: String,
    pub event: String,
    /// The event's title: what its `Undo add` names.
    pub title: String,
    pub at: Instant,
    /// The Timeline's words while this event is not saved in `Timeline.md`.
    pub not_saved: Option<String>,
}

/// The trackers of the active campaign: the definitions in play, the ledger as read, and the
/// acts' results. The shell owns one, beside Lookup, and the rail reads its count.
#[derive(Default)]
pub struct Trackers {
    campaign: Option<Campaign>,
    /// Where the campaign's manifest is, as the content found it, or why it cannot be read.
    manifest: Option<Result<String, String>>,
    /// The content the definitions were read from.
    index: Option<Arc<dyn RuleSource>>,
    next_id: Option<fn() -> Uuid>,
    pub rules: Rules,
    pub ledger: Ledger,
    /// Why `Trackers.md` cannot be read. Then no value and no notice shows.
    pub unreadable: Option<String>,
    /// The bytes read or written last; `None` before the first read.
    baseline: Option<Option<Vec<u8>>>,
    pub state: State,
    /// The entries that `Undo` takes back now: worked out when the ledger or the session changes.
    newest: Vec<Entry>,
    /// The newest act's label and the result line's words, worked out with `newest`.
    pub(crate) result: Option<(String, Sentence)>,
    /// The newest act as a kept result, for the footer's order: recorded when `newest` changes,
    /// so the store issues its order then and never at a draw.
    pub(crate) results: crate::design::Results<Played, NewestAct>,
    last_check: Option<Instant>,
    /// Why the last act wrote nothing.
    pub reason: Option<Unrecorded>,
    pub added: Option<Added>,
    /// The session being played, for the stamp and for Undo.
    pub played: Option<Uuid>,
    /// `Add to Timeline` for this entry, with this title: the shell has the Timeline.
    pub wants_timeline: Option<(String, String)>,
    /// The campaign's day count, as the Timeline held it when its count of changes was
    /// `clock_seen`. The shell reads it again at each act, so an act uses the day at the click.
    today: Today,
    clock_seen: Option<(u64, bool, Option<String>)>,
    /// `Undo add` of this event.
    pub wants_undo_add: Option<String>,
    pub(crate) view: view::ViewState,
    /// Counts each `settle`; its revision moves on with it, so the view measures its columns
    /// and fits its names again only when a value or a definition changed.
    pub(crate) revision: crate::design::memo::Revised<u64>,
    /// `Campaign → Rules and runs` in the empty state: the shell shows that page.
    pub wants_rules: bool,
    /// Each tracker's history, by key: worked out when the ledger or the definitions change.
    history: HashMap<String, Vec<history::Row>>,
    /// The problems list, each with the house rule note it belongs to: worked out with the
    /// history.
    problems: Vec<(Option<Uuid>, String)>,
    /// The last `Remove` in a tracker's history, for its one Undo, by its entries' IDs.
    removed: crate::design::Results<history::HistoryOwner, history::Removed>,
    /// Why the last `Remove` or its Undo wrote nothing, for the reader of that note.
    history_reason: Option<(Uuid, Unrecorded)>,
}

impl Trackers {
    pub fn with_ids(next_id: fn() -> Uuid) -> Self {
        Self {
            next_id: Some(next_id),
            ..Self::default()
        }
    }

    /// Follow the campaign, its content and the ledger on disk. Called each frame; it reads the
    /// manifest only when the content changes, and the ledger at most once a second, and only
    /// while a note is in play.
    pub fn poll(
        &mut self,
        index: Option<Arc<dyn RuleSource>>,
        campaign: Option<&Campaign>,
        played: Option<Uuid>,
        timeline: Clock<'_>,
    ) {
        if self.played != played {
            self.played = played;
            self.settle_result();
        }
        let key = |campaign: &Campaign| (campaign.id, campaign.path.clone());
        if campaign.map(key) != self.campaign.as_ref().map(key) {
            *self = Self {
                campaign: campaign.cloned(),
                played,
                next_id: self.next_id,
                // The kept acts are scoped by campaign: another campaign does not see them, and
                // switching back finds each with its order.
                results: std::mem::take(&mut self.results),
                ..Self::default()
            };
        }
        let Some(campaign) = campaign else { return };
        self.follow_clock(timeline);
        if let Some(index) = index
            && self
                .index
                .as_ref()
                .is_none_or(|old| old.generation() != index.generation())
        {
            self.manifest = Some(index.manifest(campaign));
            self.index = Some(index);
            self.rules = self.rules_now();
            self.settle();
            self.last_check = None;
        }
        if self.rules.in_play()
            && self
                .last_check
                .is_none_or(|at| at.elapsed() >= CHECK_DISK_EVERY)
        {
            self.last_check = Some(Instant::now());
            self.read_disk();
        }
        if self
            .added
            .as_ref()
            .is_some_and(|added| added.at.elapsed() >= UNDO_ADD_FOR)
        {
            self.added = None;
        }
    }

    /// Follow the campaign's day count. The Timeline's count of changes says when it can have
    /// moved, so the day is worked out again only then, never in each frame.
    pub fn follow_clock(&mut self, timeline: Clock<'_>) {
        let seen = (
            timeline.changes,
            timeline.conflict,
            timeline.moved_under_edit.map(str::to_owned),
        );
        if self.clock_seen.as_ref() == Some(&seen) {
            return;
        }
        self.clock_seen = Some(seen);
        let today = today(&timeline);
        if today != self.today {
            self.today = today;
            self.settle();
        }
    }

    fn path(&self) -> Option<PathBuf> {
        self.campaign
            .as_ref()
            .map(|campaign| ledger::path(&campaign.path))
    }

    /// Take the file's text when it changed. A file that cannot be read shows nothing old.
    pub fn read_disk(&mut self) {
        let Some(path) = self.path() else { return };
        let bytes = match ledger::read_bytes(&path) {
            Ok(bytes) => bytes,
            Err(error) => {
                self.unreadable = Some(error);
                self.baseline = None;
                return;
            }
        };
        if self.baseline.as_ref() == Some(&bytes) {
            return;
        }
        // The file changed: the words of the last act that wrote nothing are old.
        self.reason = None;
        self.history_reason = None;
        match ledger::parse_bytes(bytes.as_deref()) {
            Ok(ledger) => {
                self.ledger = ledger;
                self.unreadable = None;
            }
            Err(error) => {
                self.ledger = Ledger::default();
                self.unreadable = Some(error);
            }
        }
        self.baseline = Some(bytes);
        self.settle();
    }

    /// Is a house rule note in play? With none, nothing about trackers shows anywhere.
    pub fn in_play(&self) -> bool {
        self.rules.in_play()
    }

    /// The open notices, or none while the ledger or the definitions cannot be read.
    pub fn due(&self) -> usize {
        if self.in_play() && self.unreadable.is_none() && self.rules.unreadable.is_none() {
            self.state.due.len()
        } else {
            0
        }
    }

    /// The newest act of the session being played, with the entries one Undo takes back.
    pub fn newest(&self) -> &[Entry] {
        &self.newest
    }

    /// A new settle: the count, and so its revision, moves on.
    pub(crate) fn count_settle(&mut self) {
        self.revision.revise(|settles| {
            *settles += 1;
            crate::design::memo::Changed::Yes
        });
    }

    /// Work the values, the notices, the histories and the newest act out of the ledger again.
    fn settle(&mut self) {
        self.count_settle();
        self.state = replay(&self.rules, &self.ledger, &self.today);
        self.history = history::rows(&self.rules, &self.ledger, &self.state);
        self.problems = history::problems(&self.rules, &self.state);
        self.settle_result();
    }

    /// The newest act of the session being played, and the result line's words for it.
    fn settle_result(&mut self) {
        self.newest = undo_target(&self.ledger, self.played);
        self.result = (!self.newest.is_empty()).then(|| {
            (
                label(&self.rules, &self.ledger, &self.newest),
                result_words(&self.rules, &self.ledger, &self.state, &self.newest),
            )
        });
        let Some(owner) = self.played_owner() else {
            return;
        };
        // Only a new act records, which issues its order. Selecting a session or a campaign again
        // finds its act kept, with the order it had.
        let act: Vec<String> = self.newest.iter().map(|entry| entry.id.clone()).collect();
        if self.result.is_none() {
            // Only a ledger as read can say the session has no act left; before the first read
            // (a campaign switched back) the kept act stays.
            if self.baseline.is_some() {
                self.results.owner_changed(&owner);
            }
        } else if self
            .results
            .get(&owner)
            .is_none_or(|shown| shown.value().1 != act)
        {
            self.results.record(NewestAct(owner, act));
        }
    }

    /// The campaign and the session being played: the owner of the newest act's result.
    pub(crate) fn played_owner(&self) -> Option<Played> {
        self.campaign.as_ref().map(|campaign| Played {
            root: campaign.path.clone(),
            session: self.played,
        })
    }

    /// An act built from the values and notices of the file on disk. Why it wrote nothing
    /// shows on the view's result line.
    fn act(&mut self, act: Act<'_>) {
        let today = self.today.clone();
        let session = self.played;
        let next_id = self.next_id.unwrap_or(Uuid::new_v4);
        // What the act was, for its refusal: the action, the table, or the notice pressed.
        let failed = match &act {
            Act::Press { action, .. } => crate::design::results::FailedAct::NotRecorded {
                act: named(action.label),
            },
            Act::Roll { table, .. } => crate::design::results::FailedAct::NotRolled {
                table: named(self.rules.table_name(table)),
            },
            Act::Done { drawn, .. } | Act::Dismiss { drawn } | Act::DismissAll { drawn } => {
                crate::design::results::FailedAct::NotRecorded {
                    act: named(&drawn.name(&self.rules)),
                }
            }
        };
        if let Err(Some(reason)) = self.commit(failed, |rules, ledger| {
            act.build(rules, ledger, &today, session, next_id)
        }) {
            self.reason = Some(reason);
        }
    }

    /// The definitions in play as the files hold them now.
    fn rules_now(&self) -> Rules {
        match (&self.campaign, &self.manifest, &self.index) {
            (Some(campaign), Some(Ok(manifest)), Some(index)) => {
                rules_on_disk(campaign, manifest, index.as_ref())
            }
            (_, Some(Err(error)), _) => Rules::cannot_read(
                RulesFile::Manifest,
                format!("Campaign.md cannot be read: {error}"),
            ),
            _ => Rules::default(),
        }
    }

    /// Every act passes here. It reads the definitions in play and the ledger as the files hold
    /// them now. When a definition changed since the view drew it, or cannot be read, the act
    /// writes nothing and says why, and the view draws what the files hold. `Err(None)`: the
    /// ledger cannot be read, which the view and the reader say in place of the history.
    fn commit(
        &mut self,
        failed: crate::design::results::FailedAct,
        build: impl FnOnce(&Rules, &Ledger) -> Result<Op, Refused>,
    ) -> Result<(), Option<Unrecorded>> {
        let Some(campaign) = self.campaign.clone() else {
            return Err(None);
        };
        let rules = self.rules_now();
        let changed = !rules.same_definitions(&self.rules);
        let result = ledger::commit(&ledger::path(&campaign.path), |ledger| {
            if rules.unreadable.is_some() {
                return Err(Refused::RulesUnreadable(
                    rules.unreadable_file.unwrap_or(RulesFile::HouseRuleNote),
                ));
            }
            if changed {
                return Err(Refused::RulesChanged);
            }
            build(&rules, ledger)
        });
        let done = self.finish(failed, result);
        if changed {
            self.rules = rules;
            self.settle();
        }
        done
    }

    fn finish(
        &mut self,
        failed: crate::design::results::FailedAct,
        result: Result<(Vec<u8>, Ledger, Op), Fault>,
    ) -> Result<(), Option<Unrecorded>> {
        match result {
            Ok((bytes, ledger, _)) => {
                self.baseline = Some(Some(bytes));
                self.ledger = ledger;
                self.unreadable = None;
                self.settle();
                // The newest act's words replace why an older one wrote nothing.
                self.reason = None;
                self.history_reason = None;
                Ok(())
            }
            Err(Fault::NotRead(error)) => {
                self.unreadable = Some(error);
                self.baseline = None;
                Err(None)
            }
            Err(Fault::Refused(cause)) => {
                // The view draws again from the file, and says why nothing was recorded.
                self.read_disk();
                Err(Some(Unrecorded::Refused(refusal(failed, cause))))
            }
            // Nothing waits: the value, the Due rows and the result line show what the file
            // holds now, also an edit made in Obsidian a moment before.
            Err(Fault::NotSaved(error)) => {
                self.read_disk();
                Err(Some(Unrecorded::NotSaved(error)))
            }
        }
    }

    /// A quick action, on the line of `instance`, named by its label alone: the tests' press.
    #[cfg(feature = "test-support")]
    pub fn press(&mut self, key: &str, instance: Option<&str>, label: &str, asked: Option<i64>) {
        self.press_at(key, instance, label.into(), asked);
    }

    /// A quick action, named by its label and, from a drawn button, its place in `quick`.
    pub fn press_at(
        &mut self,
        key: &str,
        instance: Option<&str>,
        action: QuickRef<'_>,
        asked: Option<i64>,
    ) {
        let label = action.label;
        self.act(Act::Press {
            key,
            instance,
            action,
            asked,
        });
        // A typed number of an act that recorded nothing stays in its field.
        if let Some(amount) = asked
            && self.reason.is_some()
        {
            self.view.ask_again(key, instance, label, amount);
        }
    }

    /// A notice's `Roll <table>`: for the next name when the table names its rolls.
    pub fn roll_notice(&mut self, drawn: &Notice, table: &str) {
        self.act(Act::Roll { drawn, table });
    }

    /// `Done`: the ticked proposed changes and the notice's end, as one act.
    pub fn done(&mut self, drawn: &Notice, ticked: &[bool]) {
        self.act(Act::Done { drawn, ticked });
    }

    /// `Dismiss`: the notice ends, and nothing is applied.
    pub fn dismiss(&mut self, drawn: &Notice) {
        self.act(Act::Dismiss { drawn });
    }

    /// `Dismiss all` on a day mark's row: each of its open days ends, as one act, and nothing is
    /// applied. The days must be the ones the row showed.
    pub fn dismiss_all(&mut self, drawn: &Notice) {
        self.act(Act::DismissAll { drawn });
    }

    /// `Undo: <label>`: these entries, exactly as the line named them, leave the ledger. When a
    /// later entry about a notice would then count for nothing, it writes nothing and names it.
    pub fn undo(&mut self, expected: &[Entry]) {
        let session = self.played;
        let today = self.today.clone();
        let failed = crate::design::results::FailedAct::NotUndone {
            act: named(&label(&self.rules, &self.ledger, expected)),
        };
        if let Err(Some(reason)) = self.commit(failed, |rules, ledger| {
            undo_op(rules, &today, ledger, session, expected)
        }) {
            self.reason = Some(reason);
        }
    }
}

#[cfg(feature = "test-support")]
#[path = "trackers/test_support.rs"]
mod test_support;
