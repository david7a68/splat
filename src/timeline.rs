//! The campaign's timeline: every encounter, rest, travel interval, day and major event,
//! in story order. It stands apart from session notes. A note refers to an event by its
//! stable ID, and the same link works in Obsidian, because each event is a block in
//! `Timeline.md`. Version 2 keeps the events in the body.

#[path = "timeline/checks.rs"]
pub mod checks;

use crate::design::icons;
use crate::encounter_run::{SavedRunInfo, Standing};
pub use splat_campaign::timeline::*;
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use uuid::Uuid;

/// A session, as the timeline knows it.
#[derive(Clone, Debug)]
pub struct SessionLink {
    pub id: Uuid,
    pub label: String,
    /// The events its note links to.
    pub mentions: Vec<String>,
}

/// Saved, unfinished fights past Setup keep their HP after a long rest.
pub fn fights_that_keep_hp(
    events: &[Event],
    session: Option<Uuid>,
    saved_runs: &[SavedRunInfo],
) -> Vec<String> {
    let runs: Vec<_> = saved_runs
        .iter()
        .filter(|run| run.standing == Standing::Unfinished && !run.setup)
        .filter_map(|run| run.id)
        .collect();
    events
        .iter()
        .filter(|event| {
            event.kind == Kind::Encounter
                && event.is_in(session)
                && event.run.is_some_and(|id| runs.contains(&id))
        })
        .map(|event| event.title.clone())
        .collect()
}

/// The Timeline file of one campaign: the owner of the Timeline's kept results that both pages
/// show (the Timeline page and a session's plan). The book is not replaced by a reload, and keeps
/// its stores over a campaign switch (`Book::open`), so a result outlives both pages and shows
/// again when its campaign does.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineFile(pub PathBuf);

impl crate::design::results::Owner for TimelineFile {
    type Scope = Path;
    fn scope(&self) -> &Path {
        &self.0
    }
}

/// `Reloaded. Your version, with <n> events, is gone unless you undo.`: the events a reload
/// replaced, for `Undo reload`.
pub struct Reloaded {
    file: TimelineFile,
    events: Vec<Event>,
}

impl Reloaded {
    /// How many events the reload replaced.
    pub fn count(&self) -> usize {
        self.events.len()
    }
}

impl crate::design::results::Kept for Reloaded {
    type Owner = TimelineFile;
    const ENDS: crate::design::results::Ends = crate::design::results::Ends {
        // The next edit ends it (`touch`).
        owner_changed: crate::design::results::OwnerChanged::Ends,
        // Another reload makes a new one (`record`).
        disk_changed: crate::design::results::DiskChanged::Stays,
        press: crate::design::results::Press::Undo,
    };
    fn owner(&self) -> TimelineFile {
        self.file.clone()
    }
}

/// One change of a long rest's full-HP ruling, until the next edit or its Undo.
pub struct Ruling {
    file: TimelineFile,
    before: Event,
    after: Event,
}

impl Ruling {
    /// The rest it changed, as it is now.
    pub fn rest(&self) -> &Event {
        &self.after
    }
}

impl crate::design::results::Kept for Ruling {
    type Owner = TimelineFile;
    const ENDS: crate::design::results::Ends = crate::design::results::Ends {
        // The next edit or a reload of its file ends it (`touch`, `reload`); another
        // campaign's ruling stays.
        owner_changed: crate::design::results::OwnerChanged::Ends,
        disk_changed: crate::design::results::DiskChanged::Stays,
        press: crate::design::results::Press::Undo,
    };
    fn owner(&self) -> TimelineFile {
        self.file.clone()
    }
}

/// The Timeline's kept results, each kind in its own store, owned by the book's file.
#[derive(Default)]
pub struct BookResults {
    pub reloaded: crate::design::Results<TimelineFile, Reloaded>,
    pub removal: crate::design::Results<TimelineFile, Removal>,
    pub ruling: crate::design::Results<TimelineFile, Ruling>,
    /// The book's hold on the removal it recorded, which follows the run's updates (M0543).
    pub removal_hold: Option<crate::design::results::Hold>,
}

/// One removed event, until its session changes or Undo puts it back.
pub struct Removal {
    file: TimelineFile,
    /// This removal's own identity: a new removal, even of the same event, has another.
    action: u64,
    index: usize,
    /// The event that was before it, which it goes back after.
    after: Option<String>,
    event: Event,
    /// Its name where it was.
    label: String,
    /// Its session's events once it was gone. A difference is an edit of that session.
    owned: Vec<Event>,
    /// How many session notes still linked to it when it was removed: its result says so, for
    /// as long as the removal stands (also over a campaign switch and back).
    mentions: usize,
}

impl Removal {
    /// Its name where it was.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// How many session notes still linked to it when it was removed.
    pub fn mentions(&self) -> usize {
        self.mentions
    }
}

impl crate::design::results::Kept for Removal {
    type Owner = TimelineFile;
    const ENDS: crate::design::results::Ends = crate::design::results::Ends {
        // An edit of its event's session ends it (`end_removal_if_its_session_changed`); a
        // newer removal, in any session, replaces it (`record`).
        owner_changed: crate::design::results::OwnerChanged::Ends,
        // Events read from the file are checked as an edit is (`accept_events`).
        disk_changed: crate::design::results::DiskChanged::Stays,
        press: crate::design::results::Press::Undo,
    };
    fn owner(&self) -> TimelineFile {
        self.file.clone()
    }
}

/// Each removal action gets a number unique in this run of the app, across reloaded books.
static REMOVAL_ACTIONS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// The removal notice on the plan and on Timeline. It names which event went, by its result or
/// state, and says when another event with the same name stays in this session. The plan passes
/// its own fights in `planned`; Timeline passes the removed event's own session and none.
pub fn removed_event_words(
    book: &Book,
    session: Option<Uuid>,
    days: &[u32],
    label: &str,
    planned: &[(String, bool)],
) -> String {
    let detail = book
        .removed_event()
        .map(|event| {
            if event.run.is_some() {
                played_fight_summary(event, false)
            } else {
                event.facts()
            }
        })
        .filter(|detail| !detail.trim().is_empty());
    let mut words = match detail {
        Some(detail) => format!("Removed “{label}” ({detail})."),
        None => format!("Removed “{label}”."),
    };
    // "The other" only makes sense when the removed fight was in this session; a removal made on
    // the Timeline page from another session names no other fight here. An event with no session
    // has no "other": events with no session are not one group.
    if session.is_none()
        || !book
            .removed_event()
            .is_some_and(|event| event.is_in(session))
    {
        return words;
    }
    let same: Vec<_> = book
        .events
        .iter()
        .enumerate()
        .filter(|(at, event)| {
            event.is_in(session) && event.label(days.get(*at).copied().unwrap_or(0)) == label
        })
        .map(|(_, event)| !event.result.trim().is_empty())
        .chain(
            planned
                .iter()
                .filter(|(name, _)| name == label)
                .map(|(_, ended)| *ended),
        )
        .collect();
    match same.as_slice() {
        [] => {}
        [true] => words.push_str(&format!(" The other “{label}” and its result stay.")),
        [false] => words.push_str(&format!(" The other “{label}” stays.")),
        more if more.iter().any(|&result| result) => words.push_str(&format!(
            " {} other “{label}” events stay, with their results.",
            more.len()
        )),
        more => words.push_str(&format!(" {} other “{label}” events stay.", more.len())),
    }
    words
}

/// A played fight names its result and last round once, on the row's second line.
pub fn played_fight_summary(event: &Event, open: bool) -> String {
    let state = |round: &str, paused, ended| {
        round
            .parse()
            .ok()
            .map(|round| crate::encounter_run::state_words(round, paused, ended))
    };
    if let Some((outcome, round)) = event.result.rsplit_once(", round ") {
        state(round, false, true).map(|words| format!("{outcome} · {words}"))
    } else if let Some(round) = event.result.strip_prefix("Ended in round ") {
        state(round, false, true)
    } else if let Some(round) = event.result.strip_prefix("Round ") {
        state(round, !open, false)
    } else {
        None
    }
    .unwrap_or_else(|| event.result.clone())
}

const SAVE_AFTER: Duration = Duration::from_secs(1);
const CHECK_DISK_EVERY: Duration = Duration::from_secs(1);

pub fn kind_icon(kind: Kind) -> &'static str {
    match kind {
        Kind::Day => icons::DAY,
        Kind::Encounter => icons::ENCOUNTERS,
        Kind::ShortRest => icons::SHORT_REST,
        Kind::LongRest => icons::LONG_REST,
        Kind::Travel => icons::TRAVEL,
        Kind::Event => icons::MAJOR_EVENT,
    }
}

pub trait EventDisplay {
    fn time_warning(&self) -> Option<String>;
    fn summary(&self, session: Option<&str>) -> String;
    fn facts(&self) -> String;
    fn summary_with_result(&self, session: Option<&str>, result: &str) -> String;
}

impl EventDisplay for Event {
    /// Round 26 FG4, FD7: a length the clock cannot count is a warning with its icon, in the
    /// row's detail. Rows paint these words in the warning colour, and say `TIME_REPAIR` on
    /// hover; the event form says it below How long.
    fn time_warning(&self) -> Option<String> {
        self.time_not_read()
            .then(|| format!("{} {TIME_NOT_COUNTED}", icons::WARN))
    }

    /// The second line of a row: what is known, with the session if one is given.
    fn summary(&self, session: Option<&str>) -> String {
        self.summary_with_result(session, &self.result)
    }

    /// The row's facts without its warning, for words that name the event elsewhere: a
    /// removal's result and the list of what is written. Its state belongs to its rows.
    fn facts(&self) -> String {
        event_parts(self, None, &self.result, false)
    }

    fn summary_with_result(&self, session: Option<&str>, result: &str) -> String {
        event_parts(self, session, result, true)
    }
}

fn event_parts(event: &Event, session: Option<&str>, result: &str, warning: bool) -> String {
    let mut parts = Vec::new();
    // An untitled event already shows its kind as its name.
    if event.kind != Kind::Day && event.own_title().is_some() {
        parts.push(event.kind.word().to_owned());
    }
    for part in [event.duration.as_str(), event.place.as_str(), result] {
        if !part.trim().is_empty() {
            parts.push(part.to_owned());
        }
    }
    parts.extend(event.time_warning().filter(|_| warning));
    if let Some(session) = session {
        parts.push(session.to_owned());
    }
    parts.join(" · ")
}

/// A length the clock cannot count says so in these words, after the warning icon.
pub const TIME_NOT_COUNTED: &str = "Not counted in the day’s time";

/// How long's help, on Timeline and in the plan's open journey: the forms the clock counts.
pub const LENGTH_HELP: &str =
    "A number and a unit move the clock, such as 4 hours, 1 day or half a day.";

/// How to write a length the clock counts. The event form's help and the rows' hover say it.
pub const TIME_REPAIR: &str = "Write a number and a unit, such as 4 hours, 1 day or half a day.";

fn render(events: &[Event], sessions: &BTreeMap<Uuid, String>) -> Result<String, String> {
    Ok(formats::render(events, sessions))
}

fn new_id() -> String {
    let id = crate::hermetic::new_uuid().simple().to_string();
    format!("t-{}", &id[id.len() - 10..])
}

/// A campaign's events, kept in step with its file. Edits save shortly after they stop.
#[derive(Default)]
pub struct Book {
    /// The Timeline's kept results: a reload, a removal, a ruling.
    pub results: BookResults,
    /// While a run's update applies, a removal keeps its place (`apply_run_keeping_removal`).
    keep_removal: bool,
    path: Option<PathBuf>,
    pub events: Vec<Event>,
    pub flags: Vec<splat::content::layout::Flag>,
    run_write: bool,
    pub run_open: bool,
    baseline: Option<Vec<u8>>,
    /// The generated body last written, even while frontmatter is edited elsewhere.
    pending_run: Option<(RunFact, Option<Uuid>, bool, Option<Within>)>,
    /// Another timeline file in the Timeline's folder: the page warns, with its file line.
    pub duplicate: Option<PathBuf>,
    saved_events: Vec<Event>,
    pub renamed_links: BTreeMap<String, Vec<String>>,
    link_titles: BTreeMap<String, (String, String)>,
    reordered: Vec<String>,
    dirty: bool,
    last_edit: Option<Instant>,
    last_check: Option<Instant>,
    /// The file changed on disk during an edit. Saving stops until the DM chooses.
    pub conflict: bool,
    /// The file changed on disk, or cannot be read, under an edit that waits for its pause,
    /// as the last read found: why. The save, or a read that finds the file as it was, ends it.
    pub moved_under_edit: Option<String>,
    pub merged_outside: bool,
    /// The file cannot be read as a timeline. It is left as it is, and nothing saves.
    pub unreadable: Option<String>,
    /// The file was here, and it is gone. Its events stay in memory until the DM chooses.
    pub gone: bool,
    pub error: Option<String>,
    /// Each session's note name, for links in the file's body.
    pub session_links: BTreeMap<Uuid, String>,
    /// The length being typed in a journey's **How long**, on whichever page. One owner, so a
    /// draft on one page never outlives a newer length typed on the other.
    pub length: checks::LengthDraft,
    /// Goes up at each edit, read and write, so a reader can keep what it worked out from the
    /// events until they can have changed.
    changes: u64,
    /// The same changes, and each change of the file's bytes as read or written, as a
    /// `Revision` that a `Memo` can key by.
    revision: BookRevision,
}

/// [`Book`]'s revision: it advances with `changes` and when `baseline` changes.
#[derive(Clone, Debug)]
struct BookRevision(crate::design::memo::Revised<u64>);

impl Default for BookRevision {
    fn default() -> Self {
        Self(crate::design::memo::Revised::new(0))
    }
}

impl BookRevision {
    fn advance(&mut self) {
        self.0.revise(|n| {
            *n += 1;
            crate::design::memo::Changed::Yes
        });
    }
}

impl Book {
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn baseline(&self) -> Option<&[u8]> {
        self.baseline.as_deref()
    }

    pub fn open(&mut self, folder: Option<&Path>) {
        let path = folder.map(|folder| folder.join(FILE));
        if self.path == path {
            return;
        }
        // An edit in its pause before saving goes with the file it belongs to.
        self.save();
        if self.unsaved() {
            self.error.get_or_insert_with(|| {
                "Choose what to do with the unsaved timeline before changing folders.".into()
            });
            return;
        }
        // The stores stay: each result is scoped by its file, so another campaign shows none,
        // and coming back shows them again.
        *self = Self {
            path,
            session_links: std::mem::take(&mut self.session_links),
            results: std::mem::take(&mut self.results),
            ..Default::default()
        };
        self.reload();
    }

    fn check_copies(&mut self) {
        let Some(path) = &self.path else {
            self.duplicate = None;
            return;
        };
        self.duplicate = path
            .parent()
            .and_then(|folder| fs::read_dir(folder).ok())
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .map(|file| file.path())
            .filter(|other| other != path && other.extension().is_some_and(|ext| ext == "md"))
            .find(|other| {
                fs::read_to_string(other).is_ok_and(|text| {
                    text.lines()
                        .any(|line| line.starts_with("timeline_version:"))
                })
            });
    }

    /// Read the file, and drop any edit not yet saved.
    pub fn reload(&mut self) {
        self.changes += 1;
        self.revision.advance();
        let file = self.file();
        self.results.ruling.owner_changed(&file);
        self.dirty = false;
        self.reordered.clear();
        self.conflict = false;
        self.moved_under_edit = None;
        self.merged_outside = false;
        self.error = None;
        self.unreadable = None;
        self.gone = false;
        self.check_copies();
        let Some(path) = &self.path else {
            self.events.clear();
            return;
        };
        match fs::read(path) {
            Ok(bytes) => {
                match std::str::from_utf8(&bytes)
                    .map_err(|e| e.to_string())
                    .and_then(parse)
                {
                    Ok(_) => {
                        let text = String::from_utf8_lossy(&bytes);
                        let (events, flags) =
                            formats::parse_with_sessions(&text, &self.session_links);
                        self.flags = flags;
                        let days = day_numbers(&events);
                        self.link_titles = events
                            .iter()
                            .enumerate()
                            .map(|(at, event)| {
                                (
                                    event.id.clone(),
                                    (event.title.clone(), reference_label(event, days[at])),
                                )
                            })
                            .collect();
                        self.saved_events.clone_from(&events);
                        self.accept_events(events);
                    }
                    Err(error) => {
                        self.events.clear();
                        self.unreadable = Some(format!(
                            "{FILE} could not be read, so Splat did not change it. {error}"
                        ));
                    }
                }
                self.baseline = Some(bytes);
                self.revision.advance();
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                if self.baseline.is_some() {
                    self.gone = true;
                    self.unreadable = Some(format!(
                        "{FILE} is gone from the folder. It may have been moved or renamed. Splat will not start another by itself."
                    ));
                } else {
                    self.events.clear();
                }
            }
            Err(e) => {
                self.events.clear();
                self.unreadable = Some(format!("{FILE} could not be read. {e}"));
            }
        }
    }

    /// Note an edit. It saves after a short pause, or at the next `save`.
    pub fn touch(&mut self) {
        self.changes += 1;
        self.revision.advance();
        self.run_write = false;
        self.merged_outside = false;
        let file = self.file();
        self.results.ruling.owner_changed(&file);
        self.end_removal_if_its_session_changed();
        let days = day_numbers(&self.events);
        for (at, event) in self.events.iter().enumerate() {
            let label = reference_label(event, days[at]);
            if let Some((title, old_label)) = self
                .link_titles
                .insert(event.id.clone(), (event.title.clone(), label))
                && title != event.title
            {
                let labels = self.renamed_links.entry(event.id.clone()).or_default();
                if !labels.contains(&old_label) {
                    labels.push(old_label);
                }
            }
        }
        self.dirty = true;
        self.last_edit = Some(Instant::now());
        self.results.reloaded.owner_changed(&file);
    }

    /// The book's file: the owner of its kept results.
    pub fn file(&self) -> TimelineFile {
        TimelineFile(self.path.clone().unwrap_or_default())
    }

    /// Events from the file, or merged with it, replace the ones in memory. A change there to
    /// the removal's own session (its event back from outside, say) ends it, as an edit does.
    fn accept_events(&mut self, events: Vec<Event>) {
        self.changes += 1;
        self.revision.advance();
        self.events = events;
        self.end_removal_if_its_session_changed();
    }

    /// A removal belongs to its event's session. An edit of that session's events ends its
    /// result and Undo; another session's edit leaves them.
    fn end_removal_if_its_session_changed(&mut self) {
        if self.keep_removal {
            return;
        }
        let file = self.file();
        if self.results.removal.get(&file).is_some_and(|shown| {
            let removal = shown.value();
            self.owned_events(removal.event.session) != removal.owned
        }) {
            self.results.removal.owner_changed(&file);
        }
    }

    /// The events of one session, in story order. Events with no session are their own group.
    fn owned_events(&self, session: Option<Uuid>) -> Vec<Event> {
        self.events
            .iter()
            .filter(|event| event.is_in(session))
            .cloned()
            .collect()
    }

    fn text_for(&self, events: &[Event], source: Option<&str>) -> Result<String, String> {
        if let Some(source) = source {
            parse(source)?;
            return formats::patch(source, events, &self.session_links, self.run_write);
        }
        render(events, &self.session_links)
    }

    /// Keep outside-only edits; require a choice only for an item edited on both sides.
    fn merged_events(&self, disk: &str, overwrite: bool) -> Result<Vec<Event>, String> {
        parse(disk)?;
        let disk = formats::parse_with_sessions(disk, &self.session_links).0;
        let mut merged = disk.clone();
        for old in &self.saved_events {
            let ours = self.events.iter().find(|e| e.id == old.id);
            let theirs = disk.iter().find(|e| e.id == old.id);
            if ours == Some(old) {
                continue;
            }
            if !overwrite && theirs != Some(old) && ours != theirs {
                return Err("The same timeline item changed on both sides.".into());
            }
            if ours.is_none() {
                merged.retain(|e| e.id != old.id);
            }
        }
        for (index, event) in self.events.iter().enumerate() {
            if self.saved_events.iter().any(|old| old == event) {
                continue;
            }
            let at = merged
                .iter()
                .position(|e| e.id == event.id)
                .unwrap_or_else(|| {
                    self.events[..index]
                        .iter()
                        .rev()
                        .find_map(|previous| merged.iter().position(|e| e.id == previous.id))
                        .map_or(0, |at| at + 1)
                });
            let existing = merged.get(at).is_some_and(|e| e.id == event.id);
            let mut event = event.clone();
            if overwrite
                && self
                    .saved_events
                    .iter()
                    .find(|old| old.id == event.id)
                    .is_some_and(|old| old.extra == event.extra)
                && let Some(theirs) = disk.iter().find(|e| e.id == event.id)
            {
                event.extra.clone_from(&theirs.extra);
            }
            if existing {
                merged[at] = event;
            } else {
                merged.insert(at, event);
            }
        }
        // Reordering changes the relationship between items. Keep outside order otherwise.
        let order = |events: &[Event]| {
            events
                .iter()
                .filter(|e| {
                    self.saved_events.iter().any(|old| old.id == e.id)
                        && self.events.iter().any(|ours| ours.id == e.id)
                        && disk.iter().any(|theirs| theirs.id == e.id)
                })
                .map(|e| e.id.clone())
                .collect::<Vec<_>>()
        };
        let baseline_order = order(&self.saved_events);
        let ours = order(&self.events);
        let theirs = order(&disk);
        if ours != baseline_order {
            if !overwrite && theirs != baseline_order && ours != theirs {
                return Err("The timeline order changed on both sides.".into());
            }
            let ordered: Vec<_> = self
                .events
                .iter()
                .filter_map(|e| merged.iter().find(|m| m.id == e.id).cloned())
                .collect();
            let mut ordered = ordered.into_iter();
            for event in &mut merged {
                if self.events.iter().any(|e| e.id == event.id) {
                    *event = ordered.next().unwrap();
                }
            }
        }
        Ok(merged)
    }

    pub fn conflict_comparison(&self) -> Result<(String, String), String> {
        let path = self.path.as_ref().ok_or("No campaign is open.")?;
        let disk = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let events = self.merged_events(&disk, true)?;
        let draft = self.text_for(&events, Some(&disk))?;
        Ok((draft, disk))
    }

    fn write(&mut self) -> Result<(), String> {
        let path = self.path.as_ref().ok_or("No campaign is open.")?;
        let current = fs::read(path).ok();
        let outside = current != self.baseline;
        let mut events = self.events.clone();
        if outside {
            let disk = current
                .as_deref()
                .and_then(|bytes| std::str::from_utf8(bytes).ok());
            if let Some(disk) = disk.filter(|text| formats::is_v2(text)) {
                events = self
                    .merged_events(disk, false)
                    .inspect_err(|_| self.conflict = true)?;
            } else {
                self.conflict = true;
                return Err("Timeline.md changed on disk. Compare it before saving.".into());
            }
        }
        let source = current
            .as_deref()
            .map(std::str::from_utf8)
            .transpose()
            .map_err(|e| e.to_string())?;
        let text = self.text_for(&events, source)?;
        let parent = path.parent().ok_or("Invalid timeline location.")?;
        if !parent.is_dir() {
            return Err(format!(
                "Session folder is gone: {}. Choose folder in Sessions.",
                parent.display()
            ));
        }
        let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
        temp.write_all(text.as_bytes())
            .and_then(|()| temp.as_file().sync_all())
            .map_err(|e| e.to_string())?;
        if fs::read(path).ok() != current {
            self.conflict = true;
            return Err("Timeline.md changed again on disk. Compare it before saving.".into());
        }
        if current.is_some() {
            temp.persist(path)
        } else {
            temp.persist_noclobber(path)
        }
        .map_err(|e| e.to_string())?;
        splat_campaign::file_batch::sync_folder(parent).map_err(|error| error.to_string())?;
        self.flags = formats::parse_with_sessions(&text, &self.session_links).1;
        self.baseline = Some(text.into_bytes());
        self.revision.advance();
        self.accept_events(events);
        self.saved_events.clone_from(&self.events);
        self.merged_outside = outside;
        self.dirty = false;
        self.run_write = false;
        self.reordered.clear();
        self.conflict = false;
        self.moved_under_edit = None;
        self.error = None;
        Ok(())
    }

    pub fn save(&mut self) {
        if !self.dirty || self.conflict || self.unreadable.is_some() || self.path.is_none() {
            return;
        }
        if let Err(error) = self.write() {
            self.error = Some(format!("The timeline was not saved. {error}"));
        }
    }

    /// An edit typed against an older version of an event meets a newer one from the file:
    /// saving waits for the DM's choice, as for any edit made on both sides.
    pub fn edited_over_disk(&mut self) {
        self.touch();
        self.conflict = true;
    }

    /// The file changed on disk, and the DM keeps this version.
    pub fn overwrite(&mut self) {
        if let Some(path) = &self.path
            && let Ok(bytes) = fs::read(path)
        {
            match std::str::from_utf8(&bytes)
                .map_err(|e| e.to_string())
                .and_then(parse)
            {
                Ok(_) => {
                    match self.merged_events(std::str::from_utf8(&bytes).unwrap(), true) {
                        Ok(merged) => self.accept_events(merged),
                        Err(error) => {
                            self.error = Some(error);
                            return;
                        }
                    }
                    self.baseline = Some(bytes);
                    self.revision.advance();
                }
                Err(error) => {
                    self.error = Some(format!("Timeline.md could not be read. {error}"));
                    return;
                }
            }
        }
        self.run_write = false;
        if let Err(error) = self.write() {
            self.error = Some(format!("The timeline was not saved. {error}"));
        }
    }

    /// The file went, and the DM keeps these events: they go where it was.
    pub fn save_again(&mut self) {
        self.gone = false;
        self.unreadable = None;
        self.baseline = None;
        self.revision.advance();
        // Before the file is written, so that it holds what the open run said meanwhile.
        if let Some((fact, session, played, road)) = self.pending_run.take() {
            self.apply_run(&fact, session, played, road.as_ref());
        }
        if let Err(error) = self.write() {
            self.error = Some(format!("The timeline was not saved. {error}"));
        }
    }

    /// An edit is waiting for its pause before it saves.
    pub fn unsaved(&self) -> bool {
        self.dirty
    }

    /// The count of edits, reads and writes so far. The events are as they were while it stays.
    /// What a `Memo` keys the events and the file's bytes by.
    pub fn revision(&self) -> crate::design::memo::Revision {
        self.revision.0.revision()
    }

    pub fn changes(&self) -> u64 {
        self.changes
    }

    /// Only edits to this event affect its own save word.
    pub fn event_unsaved(&self, id: &str) -> bool {
        self.unsaved()
            && (self.reordered.iter().any(|event| event == id)
                || self.events.iter().find(|event| event.id == id)
                    != self.saved_events.iter().find(|event| event.id == id))
    }

    /// Back to the file, keeping what was here for one step of undo.
    pub fn reload_keeping_undo(&mut self) {
        let events = self.events.clone();
        self.reload();
        self.results.reloaded.record(Reloaded {
            file: self.file(),
            events,
        });
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn dropped(&self) -> Option<usize> {
        self.results
            .reloaded
            .get(&self.file())
            .map(|shown| shown.value().count())
    }

    /// `Undo reload`, pressed on the result its ticket names.
    pub fn undo_reload(&mut self, ticket: crate::design::results::Ticket) {
        if let Ok((reloaded, ())) = self.results.reloaded.undo_by(ticket, |_| Ok::<(), ()>(())) {
            self.events = reloaded.events;
            self.touch();
        }
    }

    pub fn attention_status(&self) -> Option<crate::design::Status<'static>> {
        self.not_saved().then(|| crate::design::Status {
            icon: if self.conflict {
                icons::WARN
            } else {
                icons::ERROR
            },
            word: if self.conflict {
                "Timeline changed on disk"
            } else {
                "Timeline not saved"
            },
            color: if self.conflict {
                crate::design::colors().document.external
            } else {
                crate::design::colors().document.invalid
            },
        })
    }

    /// Why this event is not in `Timeline.md`, in the words the Timeline page shows, or none.
    pub fn event_not_saved(&self, id: &str) -> Option<String> {
        if !self.not_saved() || !self.event_unsaved(id) {
            return None;
        }
        Some(match &self.error {
            Some(error) if !self.conflict => error.clone(),
            _ => "Timeline changed on disk.".to_owned(),
        })
    }

    pub fn not_saved(&self) -> bool {
        self.dirty && (self.conflict || self.error.is_some())
    }

    pub fn poll(&mut self, ctx: &eframe::egui::Context) {
        let now = Instant::now();
        if self.dirty {
            // Test builds save only when told to, so a capture never depends on the clock.
            #[cfg(not(any(test, feature = "test-support")))]
            if !self.conflict
                && self.error.is_none()
                && self
                    .last_edit
                    .is_none_or(|at| now.duration_since(at) >= SAVE_AFTER)
            {
                self.save();
            }
            #[cfg(any(test, feature = "test-support"))]
            let _ = SAVE_AFTER;
            ctx.request_repaint_after(SAVE_AFTER);
            // A dirty or conflicted book still follows the folder's other timeline file, so its
            // warning never names a file that moved or went (M4 review).
            if self
                .last_check
                .is_none_or(|at| now.duration_since(at) >= CHECK_DISK_EVERY)
            {
                self.last_check = Some(now);
                self.check_copies();
            }
        } else if self
            .last_check
            .is_none_or(|at| now.duration_since(at) >= CHECK_DISK_EVERY)
        {
            self.last_check = Some(now);
            self.read_if_changed();
        }
    }

    /// Read the file again if it changed on disk. The poll does this once a second, and an act
    /// that needs the newest events does it first. An edit waiting for its pause is newer, so
    /// the file is not read over it: false says that the file changed under that edit, and
    /// the events may lack what it holds until the save settles it.
    pub fn read_if_changed(&mut self) -> bool {
        if self.dirty {
            return self.check_under_edit();
        }
        // A file that went is looked for again only once it is back.
        if !self
            .path
            .as_ref()
            .is_some_and(|path| !self.gone || path.exists())
        {
            return true;
        }
        self.check_copies();
        let current = self.path.as_ref().and_then(|path| fs::read(path).ok());
        // A file that is back is read again, also when it is the file that went.
        if current != self.baseline || self.gone {
            self.reload();
            if self.unreadable.is_none()
                && let Some((fact, session, played, road)) = self.pending_run.take()
            {
                self.apply_run_keeping_removal(&fact, session, played, road.as_ref());
            }
        }
        true
    }

    /// While an edit waits for its pause: does the file still hold what Splat last read? When
    /// it changed or cannot be read, `moved_under_edit` says so, and the events are the draft's.
    fn check_under_edit(&mut self) -> bool {
        let read = self.path.as_ref().map(fs::read);
        let same = read.as_ref().and_then(|bytes| bytes.as_ref().ok()) == self.baseline.as_ref();
        self.moved_under_edit = match read {
            _ if same => None,
            Some(Err(error)) if error.kind() != std::io::ErrorKind::NotFound => {
                Some(format!("{FILE} could not be read. {error}"))
            }
            _ => Some(CHANGED_UNDER_DRAFT.to_owned()),
        };
        same
    }

    /// Add after the session's last event, or at the end. A rest found in an old recording
    /// belongs with its session, not after what happened since.
    /// Where a new event of this session goes: after the session's last event.
    fn session_end(&self, session: Option<Uuid>) -> usize {
        session
            .and_then(|session| {
                self.events
                    .iter()
                    .rposition(|event| event.session == Some(session))
            })
            .map_or(self.events.len(), |index| index + 1)
    }

    /// The events as `Timeline.md` holds them on disk now, when it can be read.
    pub fn on_disk(&self) -> Option<Vec<Event>> {
        let text = fs::read_to_string(self.path.as_ref()?).ok()?;
        parse(&text).ok()?;
        Some(formats::parse_with_sessions(&text, &self.session_links).0)
    }

    /// A run's fight that was placed in a journey goes after its session's last event, as any
    /// fight does, and is saved: its check changed in the merge of the save that placed it.
    pub fn leave_journey(&mut self, run: Uuid) {
        let Some(from) = self.events.iter().position(|event| event.run == Some(run)) else {
            return;
        };
        let mut event = self.events.remove(from);
        event.within = None;
        let at = self.session_end(event.session);
        self.events.insert(at, event);
        self.touch();
        self.save();
    }

    pub fn add(&mut self, kind: Kind, session: Option<Uuid>) -> String {
        let id = new_id();
        let at = self.session_end(session);
        self.events.insert(
            at,
            Event {
                id: id.clone(),
                kind,
                title: String::new(),
                duration: String::new(),
                place: String::new(),
                notes: String::new(),
                result: String::new(),
                session,
                run: None,
                marker: String::new(),
                restores: false,
                continued_in: None,
                played_order: 0,
                checks_table: None,
                checks: Vec::new(),
                within: None,
                extra: Default::default(),
            },
        );
        self.touch();
        id
    }

    pub fn undo_add(&mut self, id: &str) {
        self.remove(id);
    }

    /// A rest added by the DM on either page includes the full-HP ruling.
    pub fn add_long_rest(&mut self, session: Option<Uuid>) -> String {
        let order = self.next_order();
        let id = self.add(Kind::LongRest, session);
        let event = self.event_mut(&id).expect("the rest just added");
        event.restores = true;
        event.played_order = order;
        id
    }

    pub fn set_restores(&mut self, id: &str, restores: bool) {
        let Some(event) = self
            .event_mut(id)
            .filter(|event| event.kind == Kind::LongRest)
        else {
            return;
        };
        if event.restores == restores {
            return;
        }
        let before = event.clone();
        event.restores = restores;
        let after = event.clone();
        self.touch();
        self.results.ruling.record(Ruling {
            file: self.file(),
            before,
            after,
        });
    }

    pub fn rest_undo_id(&self) -> Option<&str> {
        self.results
            .ruling
            .get(&self.file())
            .map(|shown| shown.value())
            .filter(|ruling| self.events.contains(&ruling.after))
            .map(|ruling| ruling.after.id.as_str())
    }

    /// `Undo ruling`, pressed on the result its ticket names: the rest as it was, while it is
    /// still as the ruling left it.
    pub fn undo_restores(&mut self, ticket: crate::design::results::Ticket) {
        let events = &self.events;
        if let Ok((ruling, ())) = self.results.ruling.undo_by(ticket, |ruling| {
            if events.contains(&ruling.after) {
                Ok(())
            } else {
                Err(())
            }
        }) {
            let event = self
                .event_mut(&ruling.before.id)
                .expect("the unchanged rest");
            *event = ruling.before;
            self.touch();
        }
    }

    /// Put an event straight after another, or first. The list is the story's order.
    pub fn place_after(&mut self, id: &str, after: Option<&str>) {
        let Some(index) = self.events.iter().position(|event| event.id == id) else {
            return;
        };
        let event = self.events.remove(index);
        let at = after
            .and_then(|after| self.events.iter().position(|event| event.id == after))
            .map_or(0, |index| index + 1);
        self.events.insert(at, event);
        if at != index && !self.reordered.iter().any(|event| event == id) {
            self.reordered.push(id.to_owned());
        }
        self.touch();
    }

    /// One press and one step of undo, not a dialog.
    pub fn remove(&mut self, id: &str) {
        self.remove_linked(id, 0);
    }

    /// `remove`, of an event that `mentions` session notes still link to: the removal keeps the
    /// count, so its result can say so for as long as it stands.
    pub fn remove_linked(&mut self, id: &str, mentions: usize) {
        if let Some(index) = self.events.iter().position(|event| event.id == id) {
            let label = self.events[index].label(day_numbers(&self.events)[index]);
            let after = index
                .checked_sub(1)
                .map(|before| self.events[before].id.clone());
            let event = self.events.remove(index);
            self.touch();
            // One Undo remove at a time: a newer removal, in any session, replaces it.
            let removal = Removal {
                file: self.file(),
                action: REMOVAL_ACTIONS.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                index,
                after,
                owned: self.owned_events(event.session),
                event,
                label,
                mentions,
            };
            self.results.removal_hold = Some(self.results.removal.record_held(removal));
        }
    }

    fn removal(&self) -> Option<&Removal> {
        self.results
            .removal
            .get(&self.file())
            .map(|shown| shown.value())
    }

    /// The identity of the removal that Undo would reverse. A result that follows this action
    /// keeps it, so a later removal of the same event is told apart.
    pub fn removal_action(&self) -> Option<u64> {
        self.removal().map(|removal| removal.action)
    }

    /// The event that Undo would put back.
    pub fn removed_event(&self) -> Option<&Event> {
        self.removal().map(|removal| &removal.event)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn removed(&self) -> Option<&str> {
        self.removal().map(|removal| removal.label.as_str())
    }

    /// The event goes back after the event that was before it, so edits in other sessions since
    /// do not move it. An event that came back another way is not added twice.
    pub fn undo_remove(&mut self, ticket: crate::design::results::Ticket) -> Option<String> {
        let events = &self.events;
        // An event that came back another way is not added twice: the entry ends.
        let (removal, back) = match self.results.removal.undo_by(ticket, |removal| {
            Ok::<bool, ()>(events.iter().any(|event| event.id == removal.event.id))
        }) {
            Ok(done) => done,
            Err(_) => return None,
        };
        let id = removal.event.id.clone();
        if back {
            return None;
        }
        let at = match &removal.after {
            None => 0,
            Some(after) => self
                .events
                .iter()
                .position(|event| event.id == *after)
                .map_or(removal.index.min(self.events.len()), |at| at + 1),
        };
        self.events.insert(at, removal.event);
        self.touch();
        Some(id)
    }

    /// Every run sits on the timeline without a step from the DM. The DM's words stay, and
    /// so does the DM's undo, because a new round is not an edit.
    /// What a run says now. `played` means it changed while open, and it was not only opened.
    #[cfg(any(test, feature = "test-support"))]
    pub fn record_run(&mut self, fact: &RunFact, session: Option<Uuid>, played: bool) {
        self.record_run_at(fact, session, played, None);
    }

    /// `road`: the journey and check that a fight entering now is played at.
    fn record_run_at(
        &mut self,
        fact: &RunFact,
        session: Option<Uuid>,
        played: bool,
        road: Option<&Within>,
    ) {
        let dm_edit = self.dirty && !self.run_write;
        self.apply_run_keeping_removal(fact, session, played, road);
        self.run_write = !dm_edit;
    }

    /// A run's update is not an edit: a removal stays in place, because what comes next must
    /// follow its order. When combat begins, the run's event enters the timeline; in the
    /// removal's own session that begins the next part of its story, so the result and Undo
    /// end there. A Setup adds no event and ends nothing.
    fn apply_run_keeping_removal(
        &mut self,
        fact: &RunFact,
        session: Option<Uuid>,
        played: bool,
        road: Option<&Within>,
    ) {
        let on_timeline = |events: &[Event]| {
            events.iter().any(|event| {
                event.run == Some(fact.id)
                    || (fact.copied_from.is_some() && event.run == fact.copied_from)
            })
        };
        // A run that waits for an unreadable file changes nothing yet, and the cleared events
        // are not the session's state.
        if self.unreadable.is_some() {
            self.apply_run(fact, session, played, road);
            return;
        }
        let before = on_timeline(&self.events);
        self.keep_removal = true;
        self.apply_run(fact, session, played, road);
        self.keep_removal = false;
        let entered = (!before)
            .then(|| self.events.iter().find(|event| event.run == Some(fact.id)))
            .flatten()
            .cloned();
        let file = self.file();
        let Some((session, run)) = self
            .removal()
            .map(|removal| (removal.event.session, removal.event.run))
        else {
            return;
        };
        // The removed event's own run back on the timeline also ends it: its Undo would give
        // that run a second event.
        if entered
            .as_ref()
            .is_some_and(|event| event.is_in(session) || (event.run.is_some() && event.run == run))
        {
            self.results.removal.owner_changed(&file);
            return;
        }
        // The run's own changes are the session's new state, not an edit after it.
        self.removal_follows_run(session);
    }

    /// The kept removal takes its session's events after a run's update as the state an edit
    /// is told from. The run's update is no press: the book revises the removal it recorded by
    /// its hold (M0543), so a newer removal is never changed by it.
    fn removal_follows_run(&mut self, session: Option<Uuid>) {
        let owned = self.owned_events(session);
        if let Some(hold) = &self.results.removal_hold {
            let _ = self
                .results
                .removal
                .revise_held(hold, |removal| removal.owned = owned);
        }
    }

    /// The removal's result and Undo end: its session's note was edited after it.
    pub fn forget_removed(&mut self) {
        let file = self.file();
        self.results.removal.owner_changed(&file);
    }

    /// The order for what is played or ruled on next.
    pub fn next_order(&self) -> u64 {
        next_order(
            self.events
                .iter()
                .chain(self.removal().map(|removal| &removal.event)),
        )
    }

    /// The open run's facts changed, from `before`. A run that was already over when it opened
    /// is being read. It adds nothing to the story, whichever session is now current. `road`:
    /// the journey and the check where a fight that enters now is played.
    pub fn record_change(
        &mut self,
        before: Option<&RunFact>,
        fact: &RunFact,
        session: Option<Uuid>,
        road: Option<&Within>,
    ) {
        // A new ID with a run already open is that run's copy, which keeps its place in the story.
        let known = self.events.iter().any(|event| {
            event.run == Some(fact.id)
                || (fact.copied_from.is_some() && event.run == fact.copied_from)
        });
        if before.is_none() && !known && matches!(fact.state, RunState::Ended(_)) {
            return;
        }
        let played = before.is_some_and(|old| fact.played_since(old));
        self.record_run_at(fact, session, played, road);
    }

    fn apply_run(
        &mut self,
        fact: &RunFact,
        session: Option<Uuid>,
        played: bool,
        road: Option<&Within>,
    ) {
        if self.unreadable.is_some() {
            self.pending_run = Some((fact.clone(), session, played, road.cloned()));
            return;
        }
        if !self.events.iter().any(|event| event.run == Some(fact.id))
            && let Some(original) = fact.copied_from
            && let Some(event) = self
                .events
                .iter_mut()
                .find(|event| event.run == Some(original))
        {
            event.run = Some(fact.id);
            self.touch();
        }
        let index = self
            .events
            .iter()
            .position(|event| event.run == Some(fact.id));
        let result = match &fact.state {
            RunState::Setup => {
                // Undo of Begin combat. An event the DM wrote on stays, without a result.
                if let Some(index) = index {
                    let event = &self.events[index];
                    if event.notes.trim().is_empty()
                        && event.duration.trim().is_empty()
                        && event.place.trim().is_empty()
                    {
                        self.events.remove(index);
                        self.touch();
                    } else if !event.result.is_empty() {
                        self.events[index].result.clear();
                        self.touch();
                    }
                }
                return;
            }
            RunState::Running => format!("Round {}", fact.round),
            RunState::Ended(outcome) if outcome.trim().is_empty() => {
                format!("Ended in round {}", fact.round)
            }
            RunState::Ended(outcome) => format!("{outcome}, round {}", fact.round),
        };
        match index {
            Some(index) => {
                if self.events[index].result != result {
                    self.events[index].result = result;
                    self.touch();
                }
                // Opening a fight to read it is not playing it.
                if played {
                    let next = self.next_order();
                    if self.events[index].record_play(session, next) {
                        self.touch();
                    }
                }
            }
            None => {
                let order = self.next_order();
                let id = self.add(Kind::Encounter, session);
                let event = self
                    .events
                    .iter_mut()
                    .find(|event| event.id == id)
                    .expect("the event just added");
                event.title = fact.name.clone();
                event.run = Some(fact.id);
                event.result = result;
                event.played_order = order;
                if let Some(within) = road {
                    self.place_in_journey(&id, within);
                }
            }
        }
    }

    /// A fight begun from a journey's check goes after that journey and the fights in it at
    /// the same or an earlier check, and is played at its check. With the journey gone, it
    /// stays where it is.
    fn place_in_journey(&mut self, id: &str, within: &Within) {
        let Some(from) = self.events.iter().position(|event| event.id == id) else {
            return;
        };
        let mut event = self.events.remove(from);
        let at = self
            .events
            .iter()
            .position(|journey| journey.id == within.journey)
            .map(|journey| {
                journey
                    + 1
                    + self.events[journey + 1..]
                        .iter()
                        .take_while(|other| {
                            other.within.as_ref().is_some_and(|other| {
                                other.journey == within.journey && other.at <= within.at
                            })
                        })
                        .count()
            });
        if at.is_some() {
            event.within = Some(within.clone());
        }
        self.events.insert(at.unwrap_or(from), event);
    }

    /// The event added from this marker, however it was renamed since.
    pub fn event_of_marker(&self, key: &str) -> Option<&Event> {
        self.events.iter().find(|event| event.marker == key)
    }

    /// The DM accepts a suggestion from a transcript.
    pub fn add_marker(&mut self, marker: &Marker, key: &str, session: Option<Uuid>) -> String {
        let id = self.add(marker.kind, session);
        let event = self.event_mut(&id).expect("the event just added");
        event.title.clone_from(&marker.title);
        event.place.clone_from(&marker.place);
        event.duration.clone_from(&marker.duration);
        event.marker = key.to_owned();
        if !marker.at.is_empty() {
            event.notes = format!("Near {} in the recording.", marker.at);
        }
        id
    }

    pub fn event_mut(&mut self, id: &str) -> Option<&mut Event> {
        self.events.iter_mut().find(|event| event.id == id)
    }
}

#[cfg(test)]
#[path = "timeline/test_support.rs"]
mod test_support;
#[cfg(test)]
pub use test_support::{TestUndo, press_footer_undo};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i20_s1_separate_edits_keep_both_changes_without_a_conflict() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(FILE);
        let mut book = book(dir.path());
        let first = book.add(Kind::Travel, None);
        let second = book.add(Kind::Travel, None);
        book.save();
        let baseline = fs::read_to_string(&path).unwrap();
        book.event_mut(&first).unwrap().title = "Splat road".into();
        book.touch();
        let mut disk_events = parse(&baseline).unwrap();
        disk_events[1].title = "Outside road".into();
        let disk = formats::patch(&baseline, &disk_events, &BTreeMap::new(), false).unwrap();
        fs::write(&path, format!("{disk}\nOutside prose.\n")).unwrap();
        book.save();
        assert!(!book.conflict && !book.unsaved(), "{:?}", book.error);
        let saved = fs::read_to_string(&path).unwrap();
        let events = parse(&saved).unwrap();
        assert_eq!(
            events.iter().find(|e| e.id == first).unwrap().title,
            "Splat road"
        );
        assert_eq!(
            events.iter().find(|e| e.id == second).unwrap().title,
            "Outside road"
        );
        assert!(saved.contains("Outside prose."));
        assert_eq!(book.events, events);
        // iter-23-S FK2: A's event goes and is saved; B is edited; A's event comes back from
        // outside; the save of B merges A back. B's edit stays, and A's notice and Undo end,
        // because A's session changed.
        let (a, b) = (Uuid::from_u128(1), Uuid::from_u128(2));
        book.session_links = BTreeMap::from([(a, "Session A".into()), (b, "Session B".into())]);
        let a_event = book.add(Kind::Travel, Some(a));
        let b_event = book.add(Kind::Travel, Some(b));
        book.save();
        let with_a = fs::read_to_string(&path).unwrap();
        book.remove(&a_event);
        book.save();
        book.event_mut(&b_event).unwrap().title = "B road".into();
        book.touch();
        assert!(book.removed().is_some(), "B's edit keeps A's Undo");
        fs::write(&path, &with_a).unwrap();
        book.save();
        assert!(!book.conflict && !book.unsaved(), "{:?}", book.error);
        assert!(
            book.events.iter().any(|event| event.id == a_event),
            "A came back"
        );
        assert_eq!(
            book.events.iter().find(|e| e.id == b_event).unwrap().title,
            "B road"
        );
        assert!(book.removed().is_none(), "A's notice ends");
        assert!(
            crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove)
                .is_none(),
            "and its Undo"
        );
    }

    #[test]
    fn i20_s1_same_event_conflicts_and_overwrite_keeps_other_outside_edits() {
        let dir = tempfile::tempdir().unwrap();
        let mut book = book(dir.path());
        book.add(Kind::Travel, None);
        book.add(Kind::Travel, None);
        book.save();
        let path = dir.path().join(FILE);
        let before = fs::read_to_string(&path).unwrap();
        book.events[0].title = "My road".into();
        book.touch();
        let mut disk_events = parse(&before).unwrap();
        disk_events[0].title = "Their road".into();
        disk_events[1].notes = "Outside notes".into();
        let disk = formats::patch(&before, &disk_events, &BTreeMap::new(), false).unwrap();
        fs::write(&path, &disk).unwrap();
        book.save();
        assert!(book.conflict && book.not_saved());
        let (draft, compared_disk) = book.conflict_comparison().unwrap();
        assert_eq!(compared_disk, disk);
        assert_eq!(
            draft
                .lines()
                .zip(disk.lines())
                .filter(|(a, b)| a != b)
                .count(),
            1
        );
        assert!(draft.contains("Outside notes"));
        book.overwrite();
        assert!(!book.conflict && !book.unsaved());
        assert_eq!(fs::read_to_string(&path).unwrap(), draft);
        // iter-23-S FK2: Overwrite merges A's event back from outside too, so its notice and
        // Undo end there, while the DM's own B edit wins.
        let (a, b) = (Uuid::from_u128(1), Uuid::from_u128(2));
        book.session_links = BTreeMap::from([(a, "Session A".into()), (b, "Session B".into())]);
        let a_event = book.add(Kind::Travel, Some(a));
        let b_event = book.add(Kind::Travel, Some(b));
        book.save();
        let with_a = fs::read_to_string(&path).unwrap();
        book.remove(&a_event);
        book.save();
        book.event_mut(&b_event).unwrap().title = "My B road".into();
        book.touch();
        let mut outside = parse(&with_a).unwrap();
        outside.iter_mut().find(|e| e.id == b_event).unwrap().title = "Their B road".into();
        fs::write(
            &path,
            formats::patch(&with_a, &outside, &book.session_links, false).unwrap(),
        )
        .unwrap();
        book.save();
        assert!(book.conflict, "the same B event changed on both sides");
        assert!(book.removed().is_some());
        // The merge itself ends A's notice, even when the write after it fails.
        use std::os::unix::fs::PermissionsExt;
        let open = fs::metadata(dir.path()).unwrap().permissions();
        fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o555)).unwrap();
        book.overwrite();
        fs::set_permissions(dir.path(), open).unwrap();
        assert!(book.error.is_some(), "the write after the merge failed");
        assert!(
            book.events.iter().any(|event| event.id == a_event),
            "A came back"
        );
        assert!(
            book.removed().is_none()
                && crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove)
                    .is_none()
        );
        book.overwrite();
        assert!(!book.conflict, "{:?}", book.error);
        assert_eq!(
            book.events.iter().find(|e| e.id == b_event).unwrap().title,
            "My B road"
        );
    }

    #[test]
    fn i20_s1_outside_add_remove_and_reorder_survive_a_run_result_save() {
        let dir = tempfile::tempdir().unwrap();
        let mut book = book(dir.path());
        for _ in 0..3 {
            book.add(Kind::Travel, None);
        }
        book.save();
        let path = dir.path().join(FILE);
        let before = fs::read_to_string(&path).unwrap();
        let mut disk_events = parse(&before).unwrap();
        disk_events.remove(1);
        disk_events.swap(0, 1);
        let mut added = disk_events[0].clone();
        added.id = "t-outside".into();
        added.title = "Outside addition".into();
        disk_events.push(added);
        let disk = formats::patch(&before, &disk_events, &BTreeMap::new(), false).unwrap();
        fs::write(&path, format!("{disk}Outside prose.\n")).unwrap();
        book.events[0].result = "Round 2".into();
        book.dirty = true;
        book.run_write = true;
        book.save();
        assert!(!book.conflict && !book.unsaved(), "{:?}", book.error);
        disk_events[1].result = "Round 2".into();
        assert_eq!(book.events, disk_events);
        assert!(fs::read_to_string(path).unwrap().contains("Outside prose."));
    }

    #[test]
    fn fight_summary_uses_run_state_words_without_changing_stored_result() {
        let mut event = event(Kind::Encounter, "");
        for (stored, open, shown) in [
            ("Round 2", true, "Round 2"),
            ("Round 2", false, "Round 2 · Paused"),
            ("Ended in round 3", false, "Ended · Round 3"),
            ("Ended in round 3", true, "Ended · Round 3"),
            (
                "Held the bridge, round 1",
                false,
                "Held the bridge · Ended · Round 1",
            ),
            (
                "Held the bridge, round 1",
                true,
                "Held the bridge · Ended · Round 1",
            ),
            ("A custom result", false, "A custom result"),
            ("A custom result", true, "A custom result"),
        ] {
            event.result = stored.into();
            assert_eq!(played_fight_summary(&event, open), shown);
            assert!(
                event
                    .summary_with_result(None, &played_fight_summary(&event, open))
                    .contains(shown)
            );
            assert!(event.summary(None).contains(stored));
            assert!(
                render(&[event.clone()], &BTreeMap::new())
                    .unwrap()
                    .contains(stored)
            );
            assert_eq!(event.result, stored);
        }
    }

    fn book(folder: &Path) -> Book {
        let mut book = Book::default();
        book.open(Some(folder));
        book
    }

    fn fact(state: RunState) -> RunFact {
        RunFact {
            id: Uuid::from_u128(7),
            copied_from: None,
            name: "Bridge Ambush".into(),
            round: 3,
            state,
            steps: 1,
        }
    }

    #[test]
    fn integrity_d1_unknown_properties_and_event_fields_survive() {
        let dir = tempfile::tempdir().unwrap();
        let mut book = book(dir.path());
        book.add(Kind::Travel, None);
        book.run_write = true;
        book.save();
        let path = dir.path().join(FILE);
        let text = fs::read_to_string(&path).unwrap().replacen(
            "timeline_version:",
            "# DM comment\ntags: [journey]\ntimeline_version:",
            1,
        ) + "  - weather: rain\n";
        fs::write(&path, text).unwrap();
        book.reload();
        book.events[0].title = "Road".into();
        book.touch();
        book.run_write = true;
        book.save();
        let saved = fs::read_to_string(path).unwrap();
        assert!(saved.contains("tags:"));
        assert!(saved.contains("weather: rain"));
        assert!(saved.contains("# DM comment"));
    }

    #[test]
    fn integrity_d17_a_repaired_timeline_is_polled_again() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(FILE);
        fs::write(&path, "broken").unwrap();
        let mut book = book(dir.path());
        assert!(book.unreadable.is_some());
        book.record_change(None, &fact(RunState::Running), None, None);
        let mut repaired = event(Kind::Travel, "1 hour");
        repaired.id = "t-repaired".into();
        fs::write(&path, render(&[repaired], &BTreeMap::new()).unwrap()).unwrap();
        book.poll(&eframe::egui::Context::default());
        assert!(book.unreadable.is_none());
        assert_eq!(book.events.len(), 2);
        assert_eq!(book.events[1].run, Some(fact(RunState::Running).id));
        // iter-23-S FK2: a reload that leaves the removal's session alone keeps its Undo, and
        // so does a round that waited while the file could not be read. An outside change to
        // that session's events ends it.
        let rest = book.add(Kind::LongRest, None);
        book.save();
        book.remove(&rest);
        book.save();
        let saved = fs::read_to_string(&path).unwrap();
        fs::write(&path, "broken").unwrap();
        book.reload();
        assert!(book.unreadable.is_some());
        let later = RunFact {
            round: 5,
            ..fact(RunState::Running)
        };
        book.record_change(Some(&fact(RunState::Running)), &later, None, None);
        fs::write(&path, &saved).unwrap();
        book.last_check = None;
        book.poll(&eframe::egui::Context::default());
        assert!(book.unreadable.is_none());
        assert_eq!(
            book.events[1].result, "Round 5",
            "the waiting round arrived"
        );
        assert_eq!(book.removed(), Some("Long rest"));
        book.save();
        let saved = fs::read_to_string(&path).unwrap();
        fs::write(&path, saved.replace("1 hour", "2 hours")).unwrap();
        book.reload();
        assert_ne!(
            book.events[0].duration, "1 hour",
            "the outside change arrived"
        );
        assert!(book.removed().is_none());
    }

    #[test]
    fn integrity_d18_a_timeline_copy_is_named_and_kept() {
        let dir = tempfile::tempdir().unwrap();
        let mut book = book(dir.path());
        book.add(Kind::Travel, None);
        book.save();
        let copy = dir.path().join("Timeline (conflict).md");
        fs::copy(dir.path().join(FILE), &copy).unwrap();
        book.poll(&eframe::egui::Context::default());
        assert!(book.duplicate.as_deref() == Some(copy.as_path()));
        assert!(copy.exists());
        // M4 review: the copy is renamed outside, then removed, while the book is dirty (an
        // edit waits): the warning follows it, then ends, so its file line never opens an old
        // path.
        book.add(Kind::Travel, None);
        assert!(book.dirty);
        let renamed = dir.path().join("Timeline (old).md");
        fs::rename(&copy, &renamed).unwrap();
        book.last_check = None;
        book.poll(&eframe::egui::Context::default());
        assert_eq!(book.duplicate.as_deref(), Some(renamed.as_path()));
        fs::remove_file(&renamed).unwrap();
        book.last_check = None;
        book.poll(&eframe::egui::Context::default());
        assert_eq!(book.duplicate, None);
    }

    #[test]
    fn integrity_d22_a_recovery_copy_keeps_one_event_and_its_notes() {
        let mut book = Book::default();
        let original = fact(RunState::Running);
        book.record_change(None, &original, None, None);
        let event = book.events[0].id.clone();
        book.events[0].notes = "DM's account".into();
        let copy = RunFact {
            id: Uuid::new_v4(),
            copied_from: Some(original.id),
            ..original.clone()
        };
        book.record_change(Some(&original), &copy, None, None);
        assert_eq!(book.events.len(), 1);
        assert_eq!(book.events[0].id, event);
        assert_eq!(book.events[0].notes, "DM's account");
        assert_eq!(book.events[0].run, Some(copy.id));
    }

    #[test]
    fn integrity_d19_a_folder_change_keeps_a_blocked_draft() {
        let old = tempfile::tempdir().unwrap();
        let new = tempfile::tempdir().unwrap();
        let mut book = book(old.path());
        book.add(Kind::Travel, None);
        fs::write(old.path().join(FILE), "disk change").unwrap();
        book.open(Some(new.path()));
        assert_eq!(book.path, Some(old.path().join(FILE)));
        assert!(book.not_saved());
        assert_eq!(book.events.len(), 1);
    }

    #[test]
    fn a_rename_offers_old_link_words_even_before_the_first_save() {
        let mut book = Book::default();
        let id = book.add(Kind::LongRest, None);
        book.events[0].title = "Rest at [the mill]".into();
        book.touch();
        assert_eq!(book.renamed_links[&id], ["Long rest"]);
        assert_eq!(
            reference(&book.events[0], 0),
            format!("[[Timeline#^{id}|Rest at the mill]]")
        );
        book.touch();
        assert_eq!(book.renamed_links[&id], ["Long rest"]);
    }

    #[test]
    fn a_run_update_does_not_mark_an_unchanged_event_unsaved() {
        let dir = tempfile::tempdir().unwrap();
        let mut timeline = book(dir.path());
        let rest = timeline.add(Kind::LongRest, None);
        let event = timeline.add(Kind::Event, None);
        timeline.save();
        assert!(!timeline.event_unsaved(&rest));
        timeline.record_change(None, &fact(RunState::Running), None, None);
        assert!(timeline.unsaved());
        assert!(!timeline.event_unsaved(&rest));
        timeline.event_mut(&event).unwrap().notes = "Changed here".into();
        timeline.touch();
        assert!(timeline.event_unsaved(&event));
        timeline.save();
        assert!(!timeline.event_unsaved(&event));
        timeline.place_after(&rest, Some(&event));
        assert!(timeline.event_unsaved(&rest));
        assert!(!timeline.event_unsaved(&event));
        timeline.save();
        assert!(!timeline.event_unsaved(&rest));
        timeline.event_mut(&event).unwrap().notes = "A second edit".into();
        timeline.touch();
        assert!(timeline.event_unsaved(&event));
        timeline.reload();
        assert!(!timeline.event_unsaved(&event));
        assert_eq!(
            timeline
                .events
                .iter()
                .find(|item| item.id == event)
                .unwrap()
                .notes,
            "Changed here"
        );
    }

    #[test]
    fn reading_an_ended_run_adds_nothing_to_the_story() {
        let mut book = Book::default();
        let ended = fact(RunState::Ended("Party won".into()));
        book.record_change(None, &ended, Some(Uuid::new_v4()), None);
        assert!(book.events.is_empty(), "it was over when it opened");

        // A run that ends while open is the story.
        let running = fact(RunState::Running);
        book.record_change(None, &running, None, None);
        assert_eq!(book.events.len(), 1);
        book.record_change(Some(&running), &ended, None, None);
        assert!(book.events[0].result.starts_with("Party won"));

        // Keep both, on a run that ended: the copy has a new ID, and it is the story too.
        let copy = RunFact {
            id: Uuid::new_v4(),
            ..ended.clone()
        };
        book.record_change(Some(&ended), &copy, None, None);
        assert_eq!(book.events.len(), 2);

        // Opened again, it keeps its event up to date.
        book.record_change(None, &ended, None, None);
        assert_eq!(book.events.len(), 2);
    }

    #[test]
    fn events_round_trip_and_the_body_names_each_as_a_block() {
        let dir = tempfile::tempdir().unwrap();
        let mut first = book(dir.path());
        let day = first.add(Kind::Day, None);
        first.event_mut(&day).unwrap().title = "Harvest Eve".into();
        let rest = first.add(Kind::LongRest, None);
        let travel = first.add(Kind::Travel, None);
        first.event_mut(&travel).unwrap().duration = "3 days".into();
        first.save();
        assert!(!first.not_saved());

        let text = fs::read_to_string(dir.path().join(FILE)).unwrap();
        assert!(
            text.contains(&format!("## Day 1: Harvest Eve\n%% splat id={day} %%")),
            "{text}"
        );
        assert!(text.contains(&format!("- Long rest ^{rest}")), "{text}");
        assert!(text.contains("3 days"), "{text}");
        // The catalogue scan treats any `splat_` frontmatter line as an entry.
        assert!(!text.lines().any(|line| line.starts_with("splat_")));
        assert_eq!(book(dir.path()).events, first.events);
    }

    fn event(kind: Kind, duration: &str) -> Event {
        Event {
            id: String::new(),
            kind,
            title: String::new(),
            duration: duration.to_owned(),
            place: String::new(),
            notes: String::new(),
            result: String::new(),
            session: None,
            run: None,
            marker: String::new(),
            restores: false,
            continued_in: None,
            played_order: 0,
            checks_table: None,
            checks: Vec::new(),
            within: None,
            extra: Default::default(),
        }
    }

    #[test]
    fn fmt_f3_dm_move_clears_continuation_and_keeps_next_run_save_working() {
        let dir = tempfile::tempdir().unwrap();
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();
        let run = Uuid::new_v4();
        let sessions = BTreeMap::from([(first, "Session 1".into()), (second, "Session 2".into())]);
        let mut e = event(Kind::Encounter, "");
        e.id = "t-fight".into();
        e.title = "Bridge".into();
        e.session = Some(first);
        e.continued_in = Some(second);
        e.run = Some(run);
        e.result = "Round 2".into();
        e.played_order = 2;
        let path = dir.path().join(FILE);
        std::fs::write(
            &path,
            format!("{}\nDM prose.\n", formats::render(&[e], &sessions)),
        )
        .unwrap();
        let mut book = Book {
            session_links: sessions.clone(),
            ..Default::default()
        };
        book.open(Some(dir.path()));
        book.run_open = true;
        book.events[0].session = Some(second);
        book.events[0].continued_in = None;
        book.touch();
        book.record_run(
            &RunFact {
                id: run,
                copied_from: None,
                name: "Bridge".into(),
                round: 3,
                state: RunState::Running,
                steps: 3,
            },
            Some(second),
            true,
        );
        book.save();
        assert!(book.error.is_none(), "{:?}", book.error);
        book.reload();
        assert_eq!(book.events[0].session, Some(second));
        assert_eq!(book.events[0].continued_in, None);
        assert_eq!(book.events[0].result, "Round 3");
        assert!(std::fs::read_to_string(path).unwrap().contains("DM prose."));
    }

    #[test]
    fn fmt_f3_repeated_run_save_changes_only_result_line() {
        let dir = tempfile::tempdir().unwrap();
        let run = Uuid::new_v4();
        let mut event = event(Kind::Encounter, "");
        event.id = "t-fight".into();
        event.title = "Bridge".into();
        event.run = Some(run);
        event.result = "Round 1".into();
        let text = format!(
            "{}\nDM prose.\n",
            formats::render(&[event], &BTreeMap::new())
        );
        let path = dir.path().join(FILE);
        std::fs::write(&path, &text).unwrap();
        let mut book = Book::default();
        book.open(Some(dir.path()));
        for round in [2, 3] {
            let before = std::fs::read_to_string(&path).unwrap();
            book.record_run(
                &RunFact {
                    id: run,
                    copied_from: None,
                    name: "Bridge".into(),
                    round,
                    state: RunState::Running,
                    steps: round as usize,
                },
                None,
                true,
            );
            book.save();
            assert!(book.error.is_none(), "{:?}", book.error);
            let after = std::fs::read_to_string(&path).unwrap();
            let noise = |raw: &str| {
                raw.lines()
                    .filter(|l| !l.starts_with("  - Result:"))
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            };
            assert_eq!(noise(&before), noise(&after));
            assert_ne!(before, after);
        }
    }
    fn hp_fight(label: &str, standing: Standing) -> (Event, SavedRunInfo) {
        let mut fight = event(Kind::Encounter, "");
        fight.title = "Bridge Ambush".into();
        fight.session = Some(Uuid::new_v4());
        fight.run = Some(Uuid::new_v4());
        let run = SavedRunInfo {
            path: PathBuf::new(),
            id: fight.run,
            source: None,
            encounter: fight.title.clone(),
            label: label.into(),
            standing,
            // The fixture's phase, as `Run::info` sets it from the run's state.
            setup: label.split(" · ").any(|part| part == "Setup"),
            saved_at: None,
            unsent: Vec::new(),
        };
        (fight, run)
    }

    #[test]
    fn fights_that_keep_hp_with_no_saved_runs() {
        let (fight, _) = hp_fight("Round 2 · Paused", Standing::Unfinished);
        assert!(fights_that_keep_hp(std::slice::from_ref(&fight), fight.session, &[]).is_empty());
    }

    #[test]
    fn fights_that_keep_hp_excludes_setup() {
        // A recovery copy's or a changed run's label starts with its mark, not its state: the
        // filter reads the typed phase, never the label's first word.
        for label in [
            "Setup · Paused",
            "Copy 2 · Setup · Paused",
            "Changed on disk · Setup · Paused",
        ] {
            let (fight, run) = hp_fight(label, Standing::Unfinished);
            assert!(
                fights_that_keep_hp(std::slice::from_ref(&fight), fight.session, &[run]).is_empty(),
                "{label}"
            );
        }
    }

    #[test]
    fn fights_that_keep_hp_includes_paused_round_two_in_its_session() {
        let (fight, run) = hp_fight("Round 2 · Paused", Standing::Unfinished);
        let events = std::slice::from_ref(&fight);
        let runs = std::slice::from_ref(&run);
        assert_eq!(
            fights_that_keep_hp(events, fight.session, runs),
            ["Bridge Ambush"]
        );
        assert!(fights_that_keep_hp(events, Some(Uuid::new_v4()), runs).is_empty());
        let continued = Uuid::new_v4();
        let mut fight = fight.clone();
        fight.continued_in = Some(continued);
        assert_eq!(
            fights_that_keep_hp(&[fight], Some(continued), runs),
            ["Bridge Ambush"]
        );
    }

    #[test]
    fn fights_that_keep_hp_excludes_ended() {
        let (fight, run) = hp_fight("Ended · Round 1", Standing::Ended);
        assert!(
            fights_that_keep_hp(std::slice::from_ref(&fight), fight.session, &[run]).is_empty()
        );
    }

    #[test]
    fn fights_that_keep_hp_includes_the_open_run() {
        let (fight, run) = hp_fight("Round 2", Standing::Unfinished);
        assert_eq!(
            fights_that_keep_hp(std::slice::from_ref(&fight), fight.session, &[run]),
            ["Bridge Ambush"]
        );
    }

    #[test]
    fn durations_are_read_in_one_closed_form() {
        assert_eq!(read_duration("8 hours"), Some(480));
        assert_eq!(read_duration("An hour"), Some(60));
        assert_eq!(read_duration("1.5 hours"), Some(90));
        assert_eq!(read_duration("2 weeks"), Some(14 * 1440));
        assert_eq!(read_duration("1 month"), Some(30 * 1440));
        assert_eq!(read_duration("1 year"), Some(360 * 1440));
        assert_eq!(read_duration("half a day"), Some(720));
        assert_eq!(read_duration("Half an hour"), Some(30));
        for words in [
            "a few hours",
            "soon",
            "",
            "3",
            "-1 day",
            "3 days and a bit",
            "half day",
            "half",
            "half a day more",
        ] {
            assert_eq!(read_duration(words), None, "{words}");
        }
    }

    #[test]
    fn the_clock_adds_exact_durations_and_a_day_moves_to_the_next_boundary() {
        use Kind::*;
        let days = |list: &[(Kind, &str)]| {
            let events: Vec<_> = list.iter().map(|(k, d)| event(*k, d)).collect();
            (day_numbers(&events), clock_now(&events))
        };
        // Before the first day there is no clock, and its durations do not count.
        assert_eq!(
            days(&[(Travel, "3 days"), (Day, "")]),
            (vec![0, 1], Some(0))
        );
        assert_eq!(days(&[(Day, ""), (Day, "")]).0, [1, 2]);
        // A journey moves the clock, so adding a day after it counts another.
        assert_eq!(
            days(&[(Day, ""), (Travel, "1 day"), (Day, "")]),
            (vec![1, 1, 3], Some(2880))
        );
        // Forgotten or short durations cannot make the count drift.
        assert_eq!(days(&[(Day, ""), (LongRest, ""), (Day, "")]).0, [1, 1, 2]);
        assert_eq!(
            days(&[(Day, ""), (Travel, "30 hours"), (Day, "")]).0,
            [1, 1, 3]
        );
        assert_eq!(days(&[(Day, ""), (ShortRest, "")]).1, Some(60));
        assert_eq!(days(&[(Day, ""), (LongRest, "2 hours")]).1, Some(120));
        // Splitting a journey does not change where it ends.
        let split = days(&[
            (Day, ""),
            (Travel, "20 minutes"),
            (Travel, "20 minutes"),
            (Travel, "20 minutes"),
        ]);
        assert_eq!(split.1, Some(60));
        assert_eq!(days(&[(Day, ""), (Encounter, "")]).1, Some(0));
        assert_eq!(clock_words(1500), "Day 2, 01:00");
        // AD1: minutes show, so a ten-minute event ends at a different time from its start.
        assert_eq!(clock_words(0), "Day 1, 00:00");
        assert_eq!(clock_words(10), "Day 1, 00:10");
        assert_eq!(clock_words(DAY - 1), "Day 1, 23:59");
        assert_eq!(clock_words(DAY), "Day 2, 00:00");
    }

    #[test]
    fn a_length_the_clock_cannot_read_says_so() {
        assert!(event(Kind::Travel, "a few hours").time_not_read());
        // Round 26 FG4, FD7: the part is a warning with its icon, not a muted fact.
        assert_eq!(
            event(Kind::Travel, "a few hours").summary(Some("Session 8")),
            format!(
                "a few hours · {} Not counted in the day’s time · Session 8",
                icons::WARN
            )
        );
        assert!(!event(Kind::Travel, "half a day").time_not_read());
        assert_eq!(event(Kind::Travel, "half a day").time_warning(), None);
        assert!(!event(Kind::Travel, "3 days").time_not_read());
        // A day has no length, so words there are no warning.
        assert_eq!(event(Kind::Day, "a few hours").time_warning(), None);
        assert!(!event(Kind::Travel, "").time_not_read());
        assert_eq!(
            clock_now(&[event(Kind::Day, ""), event(Kind::Travel, "soon")]),
            Some(0)
        );
    }

    #[test]
    fn days_number_in_list_order_and_events_keep_the_day_they_follow() {
        let mut book = Book::default();
        book.add(Kind::Event, None);
        let first = book.add(Kind::Day, None);
        book.add(Kind::LongRest, None);
        book.add(Kind::Day, None);
        assert_eq!(day_numbers(&book.events), [0, 1, 1, 2]);
        book.place_after(&first, None);
        assert_eq!(day_numbers(&book.events), [1, 1, 1, 2]);
        let order: Vec<_> = book.events.iter().map(|event| event.id.clone()).collect();
        book.place_after(&first, None);
        assert_eq!(
            book.events
                .iter()
                .map(|event| &event.id)
                .collect::<Vec<_>>(),
            order.iter().collect::<Vec<_>>()
        );
    }

    #[test]
    fn a_reference_keeps_its_identity_and_written_alias() {
        let mut book = Book::default();
        let day = book.add(Kind::Day, None);
        let id = book.add(Kind::Encounter, None);
        book.event_mut(&id).unwrap().title = "Bridge [Ambush]".into();
        let days = day_numbers(&book.events);
        let link = reference(&book.events[1], days[1]);
        assert_eq!(link, format!("[[Timeline#^{id}|Bridge Ambush]]"));
        let note = format!("We fought at {link}, on [[Timeline#^{day}]]. Then [[Timeline#^gone]].");
        assert_eq!(
            referenced_ids(&note),
            [id.clone(), day.clone(), "gone".into()]
        );
        book.event_mut(&id).unwrap().title = "The bridge".into();
        assert!(link.ends_with("|Bridge Ambush]]"));
        assert_eq!(referenced_ids(&link), [id]);
    }

    #[test]
    fn a_change_on_disk_stops_saving_until_the_dm_chooses() {
        let dir = tempfile::tempdir().unwrap();
        let mut book = book(dir.path());
        let day = book.add(Kind::Day, None);
        book.save();
        let path = dir.path().join(FILE);
        let mut outside_events = book.events.clone();
        outside_events[0].title = "Outside edit".into();
        let outside = render(&outside_events, &BTreeMap::new()).unwrap();
        fs::write(&path, &outside).unwrap();
        book.event_mut(&day).unwrap().title = "Our edit".into();
        book.add(Kind::LongRest, None);
        book.save();
        assert!(book.conflict && book.not_saved());
        assert_eq!(fs::read_to_string(&path).unwrap(), outside);
        book.reload();
        assert!(!book.conflict);
        assert_eq!(book.events.len(), 1);
        assert_eq!(book.events[0].title, "Outside edit");
        book.event_mut(&day).unwrap().title = "Our next edit".into();
        book.add(Kind::Day, None);
        fs::write(&path, outside.replace("Outside edit", "Their next edit")).unwrap();
        book.save();
        assert!(book.conflict);
        book.overwrite();
        assert!(!book.conflict);
        let saved = self::parse(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(saved.len(), 2);
        assert_eq!(saved[0].title, "Our next edit");
    }

    #[test]
    fn a_reload_in_a_conflict_has_one_step_of_undo() {
        let dir = tempfile::tempdir().unwrap();
        let mut book = book(dir.path());
        let day = book.add(Kind::Day, None);
        book.save();
        let outside = "---\ntimeline_version: 2\n---\n";
        fs::write(dir.path().join(FILE), outside).unwrap();
        book.event_mut(&day).unwrap().title = "Our edit".into();
        let rest = book.add(Kind::LongRest, None);
        book.save();
        assert!(book.conflict);
        book.reload_keeping_undo();
        assert!(book.events.is_empty());
        assert_eq!(book.dropped(), Some(2));
        crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Reload);
        assert_eq!(book.events[1].id, rest);
        assert_eq!(book.dropped(), None);
    }

    #[test]
    fn an_unreadable_file_is_left_alone_and_nothing_saves() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(FILE);
        fs::write(&path, "written by hand").unwrap();
        let mut book = book(dir.path());
        assert!(book.unreadable.is_some());
        book.record_run(&fact(RunState::Running), None, true);
        assert!(book.events.is_empty());
        book.add(Kind::Day, None);
        book.save();
        assert_eq!(fs::read_to_string(&path).unwrap(), "written by hand");
    }

    #[test]
    fn version_one_timeline_is_unreadable_and_kept() {
        let dir = tempfile::tempdir().unwrap();
        let text = "---\ntimeline_version: 1\ntimeline: []\n---\n";
        let unsupported = "Unsupported timeline version (expected 2).";
        assert_eq!(parse(text).unwrap_err(), unsupported);
        fs::write(dir.path().join(FILE), text).unwrap();
        let book = book(dir.path());
        assert!(book.events.is_empty());
        assert!(book.unreadable.as_ref().unwrap().contains(unsupported));
        assert_eq!(fs::read_to_string(dir.path().join(FILE)).unwrap(), text);
    }

    #[test]
    fn a_run_adds_one_event_keeps_the_dms_words_and_undo_takes_it_back() {
        let session = Uuid::from_u128(1);
        let mut book = Book::default();
        book.record_run(&fact(RunState::Setup), Some(session), true);
        assert!(book.events.is_empty());

        book.record_run(&fact(RunState::Running), Some(session), true);
        assert_eq!(book.events.len(), 1);
        assert_eq!(book.events[0].kind, Kind::Encounter);
        assert_eq!(book.events[0].session, Some(session));
        assert_eq!(book.events[0].result, "Round 3");

        book.events[0].title = "The bridge at dusk".into();
        book.record_run(
            &fact(RunState::Ended("Party won".into())),
            Some(session),
            true,
        );
        assert_eq!(book.events.len(), 1);
        assert_eq!(book.events[0].title, "The bridge at dusk");
        assert_eq!(book.events[0].result, "Party won, round 3");

        book.record_run(&fact(RunState::Setup), Some(session), true);
        assert!(book.events.is_empty());

        book.record_run(&fact(RunState::Running), Some(session), true);
        book.events[0].notes = "The ogre fled.".into();
        book.record_run(&fact(RunState::Setup), Some(session), true);
        assert_eq!(book.events.len(), 1);
        assert_eq!(book.events[0].result, "");
    }

    /// F9, W5-D1: a fight begun from a journey's day-two check is played at that check, inside
    /// the journey, and the journey keeps its length and checks. Its one write is the fight's
    /// event in `Timeline.md`; the run's `Undo: begin combat` takes it back to the byte.
    #[test]
    fn w5_d1_a_fight_from_a_check_is_played_at_it_and_undo_restores_the_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(FILE);
        let session = Some(Uuid::from_u128(1));
        let mut book = book(dir.path());
        book.add(Kind::Day, session);
        let journey = book.add(Kind::Travel, session);
        let rest = book.add(Kind::ShortRest, session);
        {
            let travel = book.event_mut(&journey).unwrap();
            travel.duration = "2 days".into();
            travel.checks_table = Some("road".into());
            travel.checks = vec![
                Check::new(0, "3 · Nothing stirs.".into()),
                Check::new(24, "16 · [[Bridge Ambush]] waits.".into()),
            ];
        }
        book.save();
        let journey_before = book.event_mut(&journey).unwrap().clone();
        // An edit on disk a moment before: the rest gets a place in Obsidian.
        let mut outside = parse(&fs::read_to_string(&path).unwrap()).unwrap();
        outside.iter_mut().find(|e| e.id == rest).unwrap().place = "The ford".into();
        let text = formats::patch(
            &fs::read_to_string(&path).unwrap(),
            &outside,
            &book.session_links,
            false,
        )
        .unwrap();
        fs::write(&path, &text).unwrap();
        book.read_if_changed();
        let before = fs::read_to_string(&path).unwrap();

        // Act A: combat begins, from the check at 24 hours.
        let road = Within {
            journey: journey.clone(),
            at: 24,
        };
        let running = fact(RunState::Running);
        book.record_change(Some(&fact(RunState::Setup)), &running, session, Some(&road));
        let kinds: Vec<_> = book.events.iter().map(|e| e.kind).collect();
        assert_eq!(
            kinds,
            [Kind::Day, Kind::Travel, Kind::Encounter, Kind::ShortRest],
            "the fight is in the journey, before what follows it"
        );
        let starts = clock(&book.events);
        assert_eq!(starts[2], Some(DAY), "the fight starts at Day 2, 00:00");
        assert_eq!(
            starts[3],
            Some(2 * DAY),
            "the journey still ends at Day 3, 00:00"
        );
        assert_eq!(clock_now(&book.events), Some(2 * DAY + HOUR));
        assert_eq!(day_numbers(&book.events)[2], 2);
        assert_eq!(book.events[1], journey_before, "the journey is as it was");

        // A failed save keeps the fight in memory and says so; the next save writes it.
        use std::os::unix::fs::PermissionsExt;
        let open = fs::metadata(dir.path()).unwrap().permissions();
        fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o555)).unwrap();
        book.save();
        fs::set_permissions(dir.path(), open).unwrap();
        assert!(book.error.is_some(), "the save failed and says so");
        assert_eq!(fs::read_to_string(&path).unwrap(), before);
        book.save();
        let written = fs::read_to_string(&path).unwrap();
        assert!(
            written.contains(&format!("journey={journey} at=24")),
            "{written}"
        );
        assert!(
            written.contains("- Place: The ford"),
            "the outside edit stays"
        );
        // After a restart the file gives the same day.
        let again = self::book(dir.path());
        assert_eq!(again.events, book.events);
        assert_eq!(clock(&again.events)[2], Some(DAY));

        // Act B, unrelated: the DM names the rest. Then Undo of A takes only A's event.
        book.event_mut(&rest).unwrap().title = "Breather".into();
        book.touch();
        book.save();
        book.record_change(Some(&running), &fact(RunState::Setup), session, None);
        book.save();
        let undone = fs::read_to_string(&path).unwrap();
        assert_eq!(
            undone,
            before.replace("- Short rest ^", "- Short rest: **Breather** ^")
        );
        assert_eq!(book.events[1], journey_before);
    }

    /// Review round 2, finding 5: the road fight's Undo gives back the file to the byte, with LF
    /// or CRLF and with or without a final line break, after a restart, an act B and an outside
    /// edit.
    #[test]
    fn pr284_r2_undo_matrix() {
        let mut failures = Vec::new();
        for crlf in [false, true] {
            for final_lf in [false, true] {
                let dir = tempfile::tempdir().unwrap();
                let mut b = book(dir.path());
                let session = Some(Uuid::from_u128(1));
                b.add(Kind::Day, session);
                let road = b.add(Kind::Travel, session);
                b.event_mut(&road).unwrap().duration = "2 days".into();
                b.save();
                let path = dir.path().join(FILE);
                let mut before = fs::read_to_string(&path).unwrap();
                if !final_lf {
                    before = before.trim_end_matches('\n').to_owned();
                }
                if crlf {
                    before = before.replace('\n', "\r\n");
                }
                fs::write(&path, &before).unwrap();
                b = book(dir.path());
                let run = fact(RunState::Running);
                let within = Within {
                    journey: road.clone(),
                    at: 24,
                };
                b.record_change(Some(&fact(RunState::Setup)), &run, session, Some(&within));
                b.save();
                assert!(b.error.is_none() && b.events.iter().any(|e| e.run == Some(run.id)));
                // Restart; act B names the journey; an outside edit gives it a place.
                b = book(dir.path());
                b.event_mut(&road).unwrap().title = "Northern road".into();
                b.touch();
                b.save();
                let disk = fs::read_to_string(&path).unwrap();
                let mut outside = parse(&disk).unwrap();
                outside.iter_mut().find(|e| e.id == road).unwrap().place = "The ford".into();
                let text = formats::patch(&disk, &outside, &b.session_links, false).unwrap();
                fs::write(&path, text).unwrap();
                b.record_change(Some(&run), &fact(RunState::Setup), session, None);
                b.save();
                assert!(b.error.is_none(), "{:?}", b.error);
                let mut expected = parse(&before).unwrap();
                let journey = expected.iter_mut().find(|e| e.id == road).unwrap();
                journey.title = "Northern road".into();
                journey.place = "The ford".into();
                let expected = formats::patch(&before, &expected, &b.session_links, false).unwrap();
                let actual = fs::read_to_string(&path).unwrap();
                if actual != expected {
                    failures.push(format!(
                        "crlf={crlf}, final_lf={final_lf}: expected {expected:?}, got {actual:?}"
                    ));
                }
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    /// Review of PR 284, finding 2: Begin decides from the file on disk, and a change that lands
    /// after that read meets the check again after the save's merge. Either way the fight is an
    /// ordinary one, never at the old day.
    #[test]
    fn pr284_review_stale_check_at_begin() {
        let dir = tempfile::tempdir().unwrap();
        let mut b = book(dir.path());
        let session = Some(Uuid::from_u128(1));
        b.add(Kind::Day, session);
        let road = b.add(Kind::Travel, session);
        let e = b.event_mut(&road).unwrap();
        e.duration = "2 days".into();
        e.checks = vec![Check::new(24, "16 · [[Bridge Ambush]] waits.".into())];
        let mark = checks::RoadMark::new(&road, &e.checks[0]).unwrap();
        b.save();
        let path = dir.path().join(FILE);
        let disk = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            disk.replace("16 · [[Bridge Ambush]] waits.", "17 · Nothing happens."),
        )
        .unwrap();
        assert_eq!(
            mark.place(&b.on_disk().unwrap(), session),
            Err(checks::OffRoad::Changed),
            "the file on disk has another check"
        );
        // The change lands after Begin read the file: the save merges it, and the check after
        // the merge sends the fight after the session's last event.
        let within = mark.place(&b.events, session).unwrap();
        let running = fact(RunState::Running);
        b.record_change(
            Some(&fact(RunState::Setup)),
            &running,
            session,
            Some(&within),
        );
        b.save();
        assert!(b.error.is_none(), "the outside edit merged");
        assert_eq!(
            mark.place(&b.events, session),
            Err(checks::OffRoad::Changed)
        );
        b.leave_journey(running.id);
        let fight = b.events.iter().position(|e| e.run.is_some()).unwrap();
        assert_eq!(
            clock(&b.events)[fight],
            Some(2 * DAY),
            "Day 3, not the old Day 2"
        );
        let text = fs::read_to_string(path).unwrap();
        assert!(text.contains("17 · Nothing happens.") && !text.contains("journey="));
    }

    /// Review finding 3: the place inside a journey holds only for a fight; an event whose kind
    /// changed is an ordinary event in the sequence.
    #[test]
    fn pr284_review_changed_kind_uses_ordinary_clock() {
        let mut road = event(Kind::Travel, "2 days");
        road.id = "t-road".into();
        let mut changed = event(Kind::Day, "");
        changed.within = Some(Within {
            journey: road.id.clone(),
            at: 24,
        });
        let events = [event(Kind::Day, ""), road, changed];
        assert_eq!(clock(&events)[2], Some(3 * DAY));
    }

    /// Review finding 3: the clock after a journey is the later of its end and each fight's
    /// end inside it, so a fight never ends after `Now`.
    #[test]
    fn pr284_review_duration_beyond_journey() {
        let mut road = event(Kind::Travel, "2 days");
        road.id = "t-road".into();
        let mut fight = event(Kind::Encounter, "2 days");
        fight.within = Some(Within {
            journey: road.id.clone(),
            at: 24,
        });
        let mut events = vec![event(Kind::Day, ""), road, fight];
        assert_eq!(clock(&events)[2], Some(DAY));
        assert_eq!(
            clock_now(&events),
            Some(3 * DAY),
            "the fight ends Day 4, 00:00"
        );
        events[2].duration = "1 hour".into();
        assert_eq!(clock_now(&events), Some(2 * DAY), "the journey ends later");
    }

    /// Review finding 5: the road fight's Undo gives back a valid file with no blank line
    /// between events to the byte, as it does a file with blank lines (the test above).
    #[test]
    fn pr284_review_compact_file_exact_undo() {
        let dir = tempfile::tempdir().unwrap();
        let mut b = book(dir.path());
        let session = Some(Uuid::from_u128(1));
        b.add(Kind::Day, session);
        let road = b.add(Kind::Travel, session);
        b.event_mut(&road).unwrap().duration = "2 days".into();
        b.add(Kind::ShortRest, session);
        b.save();
        let path = dir.path().join(FILE);
        let before = fs::read_to_string(&path).unwrap().replace("\n\n- ", "\n- ");
        fs::write(&path, &before).unwrap();
        b = book(dir.path());
        let run = fact(RunState::Running);
        let road = Within {
            journey: road,
            at: 24,
        };
        b.record_change(Some(&fact(RunState::Setup)), &run, session, Some(&road));
        b.save();
        assert!(b.events.iter().any(|e| e.run == Some(run.id)));
        b.record_change(Some(&run), &fact(RunState::Setup), session, None);
        b.save();
        assert_eq!(fs::read_to_string(path).unwrap(), before);
    }

    /// Review finding 5: an ordinary add and its Undo write the bytes that `main` writes. The
    /// removal leaves the line break that the add brought; only a road fight takes its own.
    #[test]
    fn pr284_review_plain_add_undo_bytes() {
        let dir = tempfile::tempdir().unwrap();
        let mut b = book(dir.path());
        b.add(Kind::Day, None);
        b.save();
        let path = dir.path().join(FILE);
        let before = fs::read_to_string(&path).unwrap();
        let id = b.add(Kind::ShortRest, None);
        b.save();
        b.undo_add(&id);
        b.save();
        assert_eq!(fs::read_to_string(&path).unwrap(), format!("{before}\n"));
    }

    /// A fight inside a journey that the DM later shortened, or moved before it, follows the
    /// event before it, as any fight does: never a day the journey no longer covers.
    #[test]
    fn w5_d1_a_fight_outside_its_journey_now_follows_the_event_before_it() {
        let mut travel = event(Kind::Travel, "1 day");
        travel.id = "t-road".into();
        let mut fight = event(Kind::Encounter, "");
        fight.within = Some(Within {
            journey: "t-road".into(),
            at: 24,
        });
        let events = [event(Kind::Day, ""), travel.clone(), fight.clone()];
        assert_eq!(clock(&events)[2], Some(DAY));
        let events = [event(Kind::Day, ""), fight, travel];
        assert_eq!(clock(&events)[1], Some(0));
    }

    #[test]
    fn a_fight_played_on_under_a_later_session_goes_on_there_and_reading_it_does_not() {
        let temp = tempfile::tempdir().unwrap();
        let mut book = book(temp.path());
        let (first, later) = (Some(Uuid::new_v4()), Some(Uuid::new_v4()));
        let run = fact(RunState::Running);
        book.record_run(&run, first, true);
        assert_eq!(book.events[0].continued_in, None);
        // Opened under the later session, and closed again: nothing was played.
        book.record_run(&run, later, false);
        assert_eq!(book.events[0].continued_in, None);
        assert_eq!(party_state(&book.events, later), PartyState::default());
        book.record_run(&run, later, true);
        assert_eq!(book.events[0].continued_in, later);
        assert_eq!(book.events[0].session, first);
    }

    #[test]
    fn a_fight_resumed_late_is_the_latest_word_on_the_party() {
        let temp = tempfile::tempdir().unwrap();
        let mut book = book(temp.path());
        let session = Some(Uuid::new_v4());
        let mut bridge = fact(RunState::Running);
        book.record_run(&bridge, session, true);
        let shrine = RunFact {
            id: Uuid::from_u128(8),
            name: "Shrine".into(),
            ..fact(RunState::Running)
        };
        book.record_run(&shrine, session, true);
        let order = |book: &Book| -> Vec<Uuid> {
            party_state(&book.events, session)
                .fights
                .iter()
                .map(|fight| fight.0)
                .collect()
        };
        assert_eq!(order(&book), vec![bridge.id, shrine.id]);
        // Opened and not played: the order stays.
        book.record_run(&bridge, session, false);
        assert_eq!(order(&book), vec![bridge.id, shrine.id]);
        bridge.steps += 1;
        book.record_run(&bridge, session, true);
        assert_eq!(order(&book), vec![shrine.id, bridge.id]);
        // A rest marked after that wipes both, wherever it sits in the list.
        let rest = book.add(Kind::LongRest, session);
        let next = book.next_order();
        let event = book.event_mut(&rest).unwrap();
        event.restores = true;
        event.played_order = next;
        let state = party_state(&book.events, session);
        assert!(state.rested && state.fights.is_empty());
    }

    #[test]
    fn a_run_opened_over_its_own_older_copy_was_not_played() {
        let before = RunFact {
            steps: 4,
            ..fact(RunState::Running)
        };
        let opened = RunFact {
            steps: 0,
            ..fact(RunState::Running)
        };
        assert!(!opened.played_since(&before));
        let hit = RunFact {
            steps: 1,
            ..fact(RunState::Running)
        };
        assert!(hit.played_since(&opened));
        let other = RunFact {
            id: Uuid::from_u128(8),
            ..hit.clone()
        };
        assert!(!other.played_since(&opened));
    }

    #[test]
    fn a_fight_resumed_from_an_older_file_is_still_the_latest_word() {
        let temp = tempfile::tempdir().unwrap();
        let mut book = book(temp.path());
        let session = Some(Uuid::new_v4());
        let mut bridge = fact(RunState::Running);
        book.record_run(&bridge, session, true);
        let shrine = RunFact {
            id: Uuid::from_u128(8),
            ..fact(RunState::Running)
        };
        book.record_run(&shrine, session, true);
        // As a file written before fights had an order.
        book.events
            .iter_mut()
            .for_each(|event| event.played_order = 0);
        bridge.steps += 1;
        book.record_run(&bridge, session, true);
        let fights = party_state(&book.events, session).fights;
        assert_eq!(fights.last().map(|fight| fight.0), Some(bridge.id));
    }

    #[test]
    fn a_rest_put_back_stays_before_what_was_played_since() {
        let temp = tempfile::tempdir().unwrap();
        let mut book = book(temp.path());
        let session = Some(Uuid::new_v4());
        let mut bridge = fact(RunState::Running);
        book.record_run(&bridge, session, true);
        let rest = book.add(Kind::LongRest, session);
        let next = book.next_order();
        let event = book.event_mut(&rest).unwrap();
        event.restores = true;
        event.played_order = next;
        book.remove(&rest);
        bridge.steps += 1;
        book.record_run(&bridge, session, true);
        assert_eq!(
            crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove),
            Some(rest)
        );
        let state = party_state(&book.events, session);
        assert!(state.rested);
        assert_eq!(state.fights.len(), 1, "the fight came after the rest");
    }

    #[test]
    fn a_new_round_is_not_an_edit_so_undo_stays() {
        let mut book = Book::default();
        let rest = book.add(Kind::LongRest, None);
        book.record_run(&fact(RunState::Running), None, true);
        book.remove(&rest);
        let next = RunFact {
            round: 4,
            ..fact(RunState::Running)
        };
        book.record_run(&next, None, true);
        assert_eq!(book.events[0].result, "Round 4");
        // iter-23-S FK2: the round is the session's state now, so another session's edit after
        // it still keeps the Undo.
        book.add(Kind::Travel, Some(Uuid::from_u128(1)));
        assert_eq!(book.removed(), Some("Long rest"));
        assert_eq!(
            crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove),
            Some(rest.clone())
        );
        // iter-23-S FK2: opening a run in Setup puts nothing on the timeline, so the removal's
        // result and Undo stay. Combat that begins in the removal's session ends them; combat
        // in another session does not.
        book.remove(&rest);
        let started = RunFact {
            id: Uuid::from_u128(8),
            ..fact(RunState::Setup)
        };
        book.record_run(&started, None, true);
        assert!(
            !book
                .events
                .iter()
                .any(|event| event.run == Some(started.id))
        );
        assert_eq!(book.removed(), Some("Long rest"), "Setup keeps Undo");
        let elsewhere = RunFact {
            id: Uuid::from_u128(9),
            ..fact(RunState::Running)
        };
        book.record_run(&elsewhere, Some(Uuid::from_u128(1)), true);
        assert!(
            book.events
                .iter()
                .any(|event| event.run == Some(elsewhere.id))
        );
        assert_eq!(
            book.removed(),
            Some("Long rest"),
            "another session's combat"
        );
        let begun = RunFact {
            state: RunState::Running,
            ..started
        };
        book.record_run(&begun, None, true);
        assert!(book.events.iter().any(|event| event.run == Some(begun.id)));
        assert!(book.removed().is_none());
        assert!(
            crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove)
                .is_none()
        );
        assert!(!book.events.iter().any(|event| event.id == rest));
        // A removed fight whose run comes back in another session: its Undo would give the run
        // a second event, so it ends.
        let fight = book
            .events
            .iter()
            .find(|event| event.run == Some(elsewhere.id))
            .unwrap()
            .id
            .clone();
        book.remove(&fight);
        book.record_run(&elsewhere, Some(Uuid::from_u128(2)), true);
        assert!(book.removed().is_none());
        assert_eq!(
            book.events
                .iter()
                .filter(|event| event.run == Some(elsewhere.id))
                .count(),
            1
        );
    }

    #[test]
    fn a_file_that_goes_keeps_the_events_and_starts_nothing_by_itself() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(FILE);
        let mut book = book(dir.path());
        let rest = book.add(Kind::LongRest, None);
        book.save();
        fs::rename(&path, dir.path().join("Moved.md")).unwrap();
        book.reload();
        assert!(book.gone && book.unreadable.is_some());
        assert_eq!(book.events[0].id, rest);
        book.record_run(&fact(RunState::Running), None, true);
        assert_eq!(book.events.len(), 1);
        assert!(!path.exists());
        book.save_again();
        assert!(!book.gone && book.unreadable.is_none());
        assert!(fs::read_to_string(&path).unwrap().contains(&rest));
        // What the run said while the file was gone is recorded, and is in the file.
        assert_eq!(book.events[1].result, "Round 3");
        assert!(fs::read_to_string(&path).unwrap().contains("Round 3"));
        assert!(!book.not_saved());
        // The file goes again, and comes back from outside Splat. The next look finds it,
        // and what the run said meanwhile is recorded.
        let valid = fs::read(&path).unwrap();
        fs::remove_file(&path).unwrap();
        book.reload();
        assert!(book.gone);
        let next = RunFact {
            round: 4,
            ..fact(RunState::Running)
        };
        book.record_run(&next, None, true);
        book.last_check = None;
        book.poll(&eframe::egui::Context::default());
        assert!(book.gone, "nothing to find yet");
        fs::write(&path, valid).unwrap();
        // A file that went is looked for once each pause between looks, not on each frame.
        book.poll(&eframe::egui::Context::default());
        assert!(book.gone, "the next look is after the pause");
        book.last_check = None;
        book.poll(&eframe::egui::Context::default());
        assert!(!book.gone && book.unreadable.is_none());
        assert_eq!(book.events[1].result, "Round 4");
    }

    #[test]
    fn remove_has_one_step_of_undo_until_the_next_edit() {
        let mut book = Book::default();
        book.add(Kind::Day, None);
        let rest = book.add(Kind::LongRest, None);
        book.add(Kind::Travel, None);
        book.remove(&rest);
        assert_eq!(book.removed(), Some("Long rest"));
        assert_eq!(book.events.len(), 2);
        assert_eq!(
            crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove),
            Some(rest.clone())
        );
        assert_eq!(book.events[1].id, rest);
        book.remove(&rest);
        book.add(Kind::Day, None);
        assert!(
            crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove)
                .is_none()
        );
        // iter-23-S FK2: a removal belongs to its event's session. Another session's edit keeps
        // it, and Undo puts the event back after its old neighbour; an edit in its own session
        // ends it.
        let (one, two) = (Some(Uuid::from_u128(1)), Some(Uuid::from_u128(2)));
        let mut book = Book::default();
        let first = book.add(Kind::Travel, one);
        let rest = book.add(Kind::LongRest, one);
        book.remove(&rest);
        let other = book.add(Kind::Day, two);
        book.place_after(&other, None);
        assert_eq!(book.events[0].id, other, "B's edit happened");
        assert_eq!(book.removed(), Some("Long rest"), "B's edit keeps A's Undo");
        assert_eq!(
            crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove),
            Some(rest.clone())
        );
        let order: Vec<_> = book.events.iter().map(|event| event.id.clone()).collect();
        assert_eq!(order, [other, first.clone(), rest.clone()]);
        assert!(book.removed().is_none(), "the words go with the Undo");
        book.remove(&rest);
        book.add(Kind::Travel, one);
        assert!(book.removed().is_none(), "A's edit ends it");
        // M0543 (M-P0.28): a run's update in the removal's own session is that session's new
        // state, not an edit: the removal follows it (by its hold), so a later edit in another
        // session keeps the Undo.
        let mut book = Book::default();
        book.add(Kind::Travel, one);
        book.record_change(None, &fact(RunState::Running), one, None);
        let rest = book.add(Kind::LongRest, one);
        book.remove(&rest);
        let later = RunFact {
            round: 5,
            ..fact(RunState::Running)
        };
        book.record_change(Some(&fact(RunState::Running)), &later, one, None);
        book.add(Kind::Day, two);
        assert_eq!(
            book.removed(),
            Some("Long rest"),
            "m-p0-hold-follows: the run's round in A's session is no edit; B's edit keeps A's Undo"
        );
    }

    #[test]
    fn markers_are_read_from_a_transcript_and_read_as_words() {
        let text = "## 01:00:00\n\n<!-- splat:event kind=\"long_rest\" at=\"01:00:00\" title=\"Rest at the mill\" duration=\"8 hours\" place=\"Old mill\" -->\nWe sleep.\n<!-- splat:event kind=\"dance\" title=\"Not a kind\" -->\n<!-- other comment -->\n<!-- splat:event kind=\"day\" -->\nMorning.\n";
        let markers = parse_markers(text);
        assert_eq!(markers.len(), 2);
        assert_eq!(
            markers[0],
            Marker {
                kind: Kind::LongRest,
                title: "Rest at the mill".into(),
                at: "01:00:00".into(),
                place: "Old mill".into(),
                duration: "8 hours".into(),
            }
        );
        assert_eq!(markers[1].kind, Kind::Day);
        assert_eq!(markers[1].title, "");
        let reading = markers_for_reading(text);
        assert!(
            reading.contains("**Long rest: Rest at the mill**\n\nWe sleep."),
            "{reading}"
        );
        assert!(reading.contains("**Day: Day**\n\nMorning."), "{reading}");
        assert!(!reading.contains("kind=\"long_rest\""));
        assert!(parse_markers("<!-- splat:event kind=\"day\"").is_empty());
    }

    #[test]
    fn a_marker_becomes_an_event_only_when_the_dm_adds_it() {
        let session = Some(Uuid::from_u128(2));
        let marker = parse_markers(
            "<!-- splat:event kind=\"travel\" at=\"00:10:00\" title=\"To Emberfall\" duration=\"3 days\" -->",
        )
        .remove(0);
        let key = marker.key("night");
        let mut book = Book::default();
        assert!(book.event_of_marker(&key).is_none());
        let id = book.add_marker(&marker, &key, session);
        let event = book.event_mut(&id).unwrap();
        assert_eq!(
            (event.kind, event.title.as_str(), event.duration.as_str()),
            (Kind::Travel, "To Emberfall", "3 days")
        );
        assert_eq!(event.notes, "Near 00:10:00 in the recording.");
        // A rename does not bring the offer back, and another event does not answer for it.
        event.title = "The road".into();
        assert_eq!(
            book.event_of_marker(&key).map(|event| event.id.as_str()),
            Some(id.as_str())
        );
        let other = book.add(Kind::Travel, session);
        book.event_mut(&other).unwrap().title = "To Emberfall".into();
        assert!(book.event_of_marker(&marker.key("another night")).is_none());
    }

    #[test]
    fn long_rest_ruling_can_be_turned_off_on_and_undone() {
        let session = Some(Uuid::new_v4());
        let mut book = Book::default();
        book.record_run(&fact(RunState::Running), session, true);
        let before = party_state(&book.events, session);
        let id = book.add_long_rest(session);
        let order = book.event_mut(&id).unwrap().played_order;
        assert!(order > 0);
        assert!(party_state(&book.events, session).rested);
        book.set_restores(&id, false);
        assert_eq!(party_state(&book.events, session), before);
        crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Ruling);
        assert!(party_state(&book.events, session).rested);
        assert!(book.rest_undo_id().is_none());
        book.set_restores(&id, false);
        book.set_restores(&id, true);
        assert!(party_state(&book.events, session).rested);
        assert_eq!(book.event_mut(&id).unwrap().played_order, order);
        crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Ruling);
        assert_eq!(party_state(&book.events, session), before);
        book.remove(&id);
        assert_eq!(party_state(&book.events, session), before);
        // A Long rest from the Add strip restores; removing it takes the ruling back.
        let added = book.add_long_rest(session);
        assert!(party_state(&book.events, session).rested);
        book.remove(&added);
        assert_eq!(party_state(&book.events, session), before);
    }

    #[test]
    fn rest_ruling_undo_expires_at_the_next_edit_and_keeps_play_order() {
        let session = Some(Uuid::new_v4());
        let mut book = Book::default();
        let id = book.add_long_rest(session);
        book.record_run(&fact(RunState::Running), session, true);
        book.set_restores(&id, false);
        book.set_restores(&id, true);
        assert_eq!(party_state(&book.events, session).fights.len(), 1);
        book.event_mut(&id).unwrap().notes = "Later edit".into();
        assert!(book.rest_undo_id().is_none());
        crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Ruling);
        assert!(book.event_mut(&id).unwrap().restores);
        book.set_restores(&id, false);
        book.add(Kind::Travel, session);
        assert!(book.rest_undo_id().is_none());
        crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Ruling);
        assert!(!book.event_mut(&id).unwrap().restores);
        // Recording markers remain proposals, without an automatic HP ruling.
        let marker = book.add(Kind::LongRest, session);
        assert!(!book.event_mut(&marker).unwrap().restores);
    }

    #[test]
    fn the_partys_state_follows_the_last_fight_or_a_rest_after_it() {
        let session = Some(Uuid::new_v4());
        let event = |kind, run, restores| Event {
            id: new_id(),
            kind,
            title: "A".into(),
            duration: String::new(),
            place: String::new(),
            notes: String::new(),
            result: String::new(),
            session,
            run,
            marker: String::new(),
            restores,
            continued_in: None,
            played_order: 0,
            checks_table: None,
            checks: Vec::new(),
            within: None,
            extra: Default::default(),
        };
        let first = Uuid::new_v4();
        let mut events = vec![event(Kind::ShortRest, None, false)];
        assert_eq!(party_state(&events, session), PartyState::default());
        events.push(event(Kind::Encounter, Some(first), false));
        assert_eq!(party_state(&events, session).fights.len(), 1);
        // A rest that restored nobody changes nothing.
        events.push(event(Kind::LongRest, None, false));
        assert_eq!(party_state(&events, session).fights.len(), 1);
        // A marked rest wipes out the fights before it, and later fights count from it.
        events.push(event(Kind::LongRest, None, true));
        let rested = party_state(&events, session);
        assert!(rested.rested && rested.fights.is_empty());
        let second = Uuid::new_v4();
        events.push(event(Kind::Encounter, Some(second), false));
        let state = party_state(&events, session);
        assert!(state.rested);
        assert_eq!(state.fights[0].0, second);
        // A fight that went on under a later session counts there too.
        let later = Some(Uuid::new_v4());
        assert_eq!(party_state(&events, later), PartyState::default());
        events.last_mut().unwrap().continued_in = later;
        assert_eq!(party_state(&events, later).fights[0].0, second);
        // Another session's fight, or none, has nothing to say about this one.
        assert_eq!(party_state(&events, None), PartyState::default());
        assert_eq!(
            party_state(&events, Some(Uuid::new_v4())),
            PartyState::default()
        );
    }

    #[test]
    fn an_event_goes_after_its_sessions_last_event() {
        let (first, second) = (Some(Uuid::from_u128(1)), Some(Uuid::from_u128(2)));
        let mut book = Book::default();
        book.add(Kind::Day, first);
        book.add(Kind::Day, second);
        let late = book.add(Kind::LongRest, first);
        assert_eq!(book.events[1].id, late);
        let new = book.add(Kind::Travel, Some(Uuid::from_u128(3)));
        assert_eq!(book.events[3].id, new);
    }

    #[test]
    fn an_event_can_be_placed_after_any_other_or_first() {
        let mut book = Book::default();
        let first = book.add(Kind::Day, None);
        let second = book.add(Kind::Day, None);
        let third = book.add(Kind::LongRest, None);
        book.place_after(&third, Some(&first));
        assert_eq!(book.events[1].id, third);
        book.place_after(&second, None);
        assert_eq!(book.events[0].id, second);
        // Named after itself, or after what is gone, it goes first and is not lost.
        book.place_after(&first, Some("gone"));
        assert_eq!(book.events.len(), 3);
    }
}
