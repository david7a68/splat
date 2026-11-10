//! The campaign's timeline: every encounter, rest, travel interval, day and major event,
//! in story order. It stands apart from session notes. A note refers to an event by its
//! stable ID, and the same link works in Obsidian, because each event is a block in
//! `Timeline.md`. Version 2 keeps the events in the body. Old layouts need conversion.
pub(crate) mod formats;
pub mod page;

use crate::design::icons;
use crate::encounter_run::{SavedRunInfo, Standing};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use uuid::Uuid;

/// Saved, unfinished fights past Setup keep their HP after a long rest.
pub fn fights_that_keep_hp(
    events: &[Event],
    session: Option<Uuid>,
    saved_runs: &[SavedRunInfo],
) -> Vec<String> {
    let runs: Vec<_> = saved_runs
        .iter()
        .filter(|run| run.standing == Standing::Unfinished && !run.label.starts_with("Setup"))
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
                event.summary(None)
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

pub const FILE: &str = "Timeline.md";
const SAVE_AFTER: Duration = Duration::from_secs(1);
const CHECK_DISK_EVERY: Duration = Duration::from_secs(1);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Day,
    Encounter,
    ShortRest,
    LongRest,
    Travel,
    Event,
}

impl Kind {
    pub const ALL: [Self; 6] = [
        Self::Day,
        Self::Encounter,
        Self::LongRest,
        Self::ShortRest,
        Self::Travel,
        Self::Event,
    ];

    /// The name in files and in transcript markers.
    pub fn wire(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Encounter => "encounter",
            Self::ShortRest => "short_rest",
            Self::LongRest => "long_rest",
            Self::Travel => "travel",
            Self::Event => "event",
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Self::Day => "Day",
            Self::Encounter => "Fight",
            Self::ShortRest => "Short rest",
            Self::LongRest => "Long rest",
            Self::Travel => "Travel",
            Self::Event => "Major event",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Self::Day => icons::DAY,
            Self::Encounter => icons::ENCOUNTERS,
            Self::ShortRest => icons::SHORT_REST,
            Self::LongRest => icons::LONG_REST,
            Self::Travel => icons::TRAVEL,
            Self::Event => icons::MAJOR_EVENT,
        }
    }
}

fn is_zero(number: &u64) -> bool {
    *number == 0
}

fn is_empty(text: &str) -> bool {
    text.is_empty()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub kind: Kind,
    #[serde(default, skip_serializing_if = "is_empty")]
    pub title: String,
    #[serde(default, skip_serializing_if = "is_empty")]
    pub duration: String,
    #[serde(default, skip_serializing_if = "is_empty")]
    pub place: String,
    #[serde(default, skip_serializing_if = "is_empty")]
    pub notes: String,
    /// What the encounter run recorded. Splat keeps this up to date, not the DM.
    #[serde(default, skip_serializing_if = "is_empty")]
    pub result: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run: Option<Uuid>,
    /// The marker this came from, so that a rename does not offer it again.
    #[serde(default, skip_serializing_if = "is_empty")]
    pub marker: String,
    /// The DM said this long rest brought everyone back to full. It records their ruling.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub restores: bool,
    /// A fight that went on in a later session belongs to that one too.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continued_in: Option<Uuid>,
    /// When this was last played or ruled on, among the others. Zero is before any of them.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub played_order: u64,
    #[serde(flatten)]
    pub extra: serde_yaml_ng::Mapping,
}

impl Event {
    /// Does this event belong to this session, or go on in it?
    pub fn is_in(&self, session: Option<Uuid>) -> bool {
        self.session == session || (session.is_some() && self.continued_in == session)
    }

    /// The name people read. A day has its number, and it can also have a name.
    pub fn label(&self, day: u32) -> String {
        match (self.kind, self.title.trim()) {
            (Kind::Day, "") => format!("Day {day}"),
            (Kind::Day, title) => format!("Day {day}: {title}"),
            (kind, "") => kind.word().to_owned(),
            (_, title) => title.to_owned(),
        }
    }

    /// How far this event moves the campaign's clock, in minutes.
    fn minutes(&self) -> u64 {
        match (self.kind, self.duration.trim()) {
            (Kind::Day, _) => 0,
            (Kind::LongRest, "") => 8 * HOUR,
            (Kind::ShortRest, "") => HOUR,
            (_, text) => read_duration(text).unwrap_or(0),
        }
    }

    /// The DM wrote a length that Splat cannot count, so the clock did not move.
    pub fn time_not_read(&self) -> bool {
        let text = self.duration.trim();
        self.kind != Kind::Day && !text.is_empty() && read_duration(text).is_none()
    }

    /// The second line of a row: what is known, with the session if one is given.
    pub fn summary(&self, session: Option<&str>) -> String {
        self.summary_with_result(session, &self.result)
    }

    fn summary_with_result(&self, session: Option<&str>, result: &str) -> String {
        let mut parts = Vec::new();
        // An untitled event already shows its kind as its name.
        if self.kind != Kind::Day
            && !self.title.trim().is_empty()
            && self.title.trim() != self.kind.word()
        {
            parts.push(self.kind.word());
        }
        for part in [self.duration.as_str(), self.place.as_str(), result] {
            if !part.trim().is_empty() {
                parts.push(part);
            }
        }
        if self.time_not_read() {
            parts.push("time not read");
        }
        if let Some(session) = session {
            parts.push(session);
        }
        parts.join(" · ")
    }
}

/// What the party's state comes from in one session, as far as the timeline can say.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PartyState {
    pub rest_order: u64,
    /// A long rest the DM marked brought everyone back.
    pub rested: bool,
    /// The fights since then, oldest first: each run, and its name. A player not in the last
    /// fight stands where an earlier one left them.
    pub fights: Vec<(Uuid, String)>,
}

/// One session's events in order. A marked rest wipes out what came before it.
pub fn party_state(events: &[Event], session: Option<Uuid>) -> PartyState {
    let mut state = PartyState::default();
    if session.is_none() {
        return state;
    }
    let days = day_numbers(events);
    // In the order they were played. A fight resumed late is the latest word on the party,
    // wherever the story puts it. The sort keeps list order among events that have no order.
    let mut played: Vec<(&Event, u32)> = events
        .iter()
        .zip(days)
        .filter(|(event, _)| event.is_in(session))
        .collect();
    played.sort_by_key(|(event, _)| event.played_order);
    for (event, day) in played {
        if let Some(run) = event.run {
            state.fights.push((run, event.label(day)));
        } else if event.kind == Kind::LongRest && event.restores {
            state = PartyState {
                rest_order: event.played_order,
                rested: true,
                fights: Vec::new(),
            };
        }
    }
    state
}

/// What a run tells the timeline. A new value of this means something changed.
#[derive(Clone, Debug, PartialEq)]
pub struct RunFact {
    pub id: Uuid,
    pub copied_from: Option<Uuid>,
    pub name: String,
    pub round: u64,
    pub state: RunState,
    /// How many steps the DM has taken since this run opened. It only grows while it stays open.
    pub steps: usize,
}

impl RunFact {
    /// Whether the DM played this run after `old`. A run just opened has only been opened,
    /// even when it opens over its own older copy.
    pub fn played_since(&self, old: &RunFact) -> bool {
        self.id == old.id && self.steps > old.steps
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RunState {
    Setup,
    Running,
    Ended(String),
}

const HOUR: u64 = 60;
const DAY: u64 = 24 * HOUR;
const MONTH: u64 = 30 * DAY;
const YEAR: u64 = 12 * MONTH;

/// A length in the closed form the clock can count, in minutes: `3 days`, `an hour`.
pub fn read_duration(text: &str) -> Option<u64> {
    let mut words = text.split_whitespace();
    let number = match words.next()?.to_lowercase().as_str() {
        "a" | "an" => 1.0,
        number => number.parse::<f64>().ok()?,
    };
    let unit = match words.next()?.to_lowercase().trim_end_matches('s') {
        "minute" => 1,
        "hour" => HOUR,
        "day" => DAY,
        "week" => 7 * DAY,
        "month" => MONTH,
        "year" => YEAR,
        _ => return None,
    };
    if words.next().is_some() || !number.is_finite() || number < 0.0 {
        return None;
    }
    let minutes = number * unit as f64;
    (minutes < u64::MAX as f64 / 2.0).then(|| minutes.round() as u64)
}

/// The campaign's one clock: the minute each event starts, in list order. It is computed and
/// never stored, so moving or removing an event corrects it. `None` means before the first day.
pub fn clock(events: &[Event]) -> Vec<Option<u64>> {
    let mut now = None;
    events
        .iter()
        .map(|event| {
            if event.kind == Kind::Day {
                // Always the next boundary, so forgotten durations cannot make the count drift.
                now = Some(now.map_or(0, |minute: u64| (minute / DAY + 1) * DAY));
            }
            let start = now;
            now = now.map(|minute| minute.saturating_add(event.minutes()));
            start
        })
        .collect()
}

/// The clock after every event, for the page to say where the story is now.
pub fn clock_now(events: &[Event]) -> Option<u64> {
    clock(events)
        .last()
        .copied()
        .flatten()
        .zip(events.last())
        .map(|(start, event)| start.saturating_add(event.minutes()))
}

/// The day the story is on now. Nothing, before the first day.
pub fn day_now(events: &[Event]) -> Option<u64> {
    clock_now(events).map(|minute| minute / DAY + 1)
}

/// A moment on the clock in the words a DM reads: the day, then hours and minutes, so that two
/// events within one hour read apart. Whole hours read the same way (`Day 2, 01:00`).
pub fn clock_words(minute: u64) -> String {
    let within = minute % DAY;
    format!(
        "Day {}, {:02}:{:02}",
        minute / DAY + 1,
        within / HOUR,
        within % HOUR
    )
}

/// The day each event falls on, in list order. Zero means before the first day.
pub fn day_numbers(events: &[Event]) -> Vec<u32> {
    clock(events)
        .into_iter()
        .map(|start| start.map_or(0, |minute| (minute / DAY) as u32 + 1))
        .collect()
}

/// A link to one event. Obsidian follows it to the event's block in `Timeline.md`.
pub fn reference_label(event: &Event, day: u32) -> String {
    let label: String = event
        .label(day)
        .chars()
        .filter(|c| !matches!(c, '[' | ']' | '|' | '#' | '^'))
        .collect();
    label.trim().to_owned()
}

pub fn reference(event: &Event, day: u32) -> String {
    format!("[[Timeline#^{}|{}]]", event.id, reference_label(event, day))
}

/// Call `found` with each timeline link in a text: the ID and the label, if it has one.
pub(crate) fn each_reference<'a>(
    text: &'a str,
    mut found: impl FnMut(usize, usize, &'a str, Option<&'a str>),
) {
    const START: &str = "[[Timeline#^";
    let mut from = 0;
    while let Some(start) = text[from..].find(START).map(|at| at + from) {
        let inner = start + START.len();
        let Some(end) = text[inner..].find("]]").map(|at| at + inner) else {
            return;
        };
        let (id, label) = match text[inner..end].split_once('|') {
            Some((id, label)) => (id, Some(label)),
            None => (&text[inner..end], None),
        };
        found(start, end + 2, id, label);
        from = end + 2;
    }
}

pub fn referenced_ids(text: &str) -> Vec<String> {
    let mut ids = Vec::new();
    each_reference(text, |_, _, id, _| {
        if !ids.iter().any(|known| known == id) {
            ids.push(id.to_owned());
        }
    });
    ids
}

/// An event an LLM found in a transcript. It is only a suggestion until the DM adds it.
#[derive(Clone, Debug, PartialEq)]
pub struct Marker {
    pub kind: Kind,
    pub title: String,
    pub at: String,
    pub place: String,
    pub duration: String,
}

impl Marker {
    /// How this marker reads in the Reader.
    pub fn reading_words(&self) -> String {
        let title = if self.title.is_empty() {
            self.kind.word()
        } else {
            &self.title
        };
        format!("**{}: {title}**", self.kind.word())
    }

    /// Which marker this is, across the times a recording's list is read.
    pub fn key(&self, recording: &str) -> String {
        format!(
            "{recording} {} {} {}",
            self.at,
            self.kind.wire(),
            self.title
        )
    }
}

const MARKER_START: &str = "<!-- splat:event ";
const MARKER_END: &str = "-->";

fn attribute(text: &str, name: &str) -> String {
    let start = format!("{name}=\"");
    text.match_indices(&start)
        // Not the end of a longer name, such as `format=`.
        .find(|(at, _)| *at == 0 || text[..*at].ends_with(char::is_whitespace))
        .and_then(|(at, _)| {
            let rest = &text[at + start.len()..];
            rest.find('"').map(|end| rest[..end].trim().to_owned())
        })
        .unwrap_or_default()
}

/// Each marker in order, with where it is. One of an unknown kind is left in the text.
fn each_marker(text: &str, mut found: impl FnMut(usize, usize, Marker)) {
    let mut from = 0;
    while let Some(start) = text[from..].find(MARKER_START).map(|at| at + from) {
        let inner = start + MARKER_START.len();
        let Some(end) = text[inner..].find(MARKER_END).map(|at| at + inner) else {
            return;
        };
        let body = &text[inner..end];
        let kind = attribute(body, "kind");
        if let Some(kind) = Kind::ALL.into_iter().find(|known| known.wire() == kind) {
            found(
                start,
                end + MARKER_END.len(),
                Marker {
                    kind,
                    title: attribute(body, "title"),
                    at: attribute(body, "at"),
                    place: attribute(body, "place"),
                    duration: attribute(body, "duration"),
                },
            );
        }
        from = end + MARKER_END.len();
    }
}

pub fn parse_markers(text: &str) -> Vec<Marker> {
    let mut markers = Vec::new();
    each_marker(text, |_, _, marker| markers.push(marker));
    markers
}

/// A marked transcript for the Reader: a comment would not show, so each becomes a line.
pub fn markers_for_reading(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut copied = 0;
    each_marker(text, |start, end, marker| {
        out.push_str(&text[copied..start]);
        // A blank line follows, so that the words stay apart from the marker's line.
        out.push_str(&format!("{}\n", marker.reading_words()));
        copied = end;
    });
    out.push_str(&text[copied..]);
    out
}

fn render(events: &[Event], sessions: &BTreeMap<Uuid, String>) -> Result<String, String> {
    Ok(formats::render(events, sessions))
}

pub(crate) fn parse(text: &str) -> Result<Vec<Event>, String> {
    let (front, _) = crate::sessions::split_note(text).ok_or("Missing timeline properties.")?;
    let fields: serde_yaml_ng::Value = serde_yaml_ng::from_str(
        front
            .trim()
            .trim_start_matches("---")
            .trim_end_matches("---"),
    )
    .map_err(|e| format!("Invalid YAML: {e}"))?;
    match fields
        .get("timeline_version")
        .and_then(serde_yaml_ng::Value::as_u64)
    {
        Some(2) => {}
        Some(1) => return Err(splat::content::OLD_LAYOUT.into()),
        _ => return Err("Unsupported timeline version (expected 2).".into()),
    }
    Ok(formats::parse(text).0)
}

fn new_id() -> String {
    let id = crate::hermetic::new_uuid().simple().to_string();
    format!("t-{}", &id[id.len() - 10..])
}

/// A campaign's events, kept in step with its file. Edits save shortly after they stop.
#[derive(Default)]
pub struct Book {
    path: Option<PathBuf>,
    pub events: Vec<Event>,
    pub flags: Vec<splat::content::layout::Flag>,
    run_write: bool,
    pub run_open: bool,
    baseline: Option<Vec<u8>>,
    /// The generated body last written, even while frontmatter is edited elsewhere.
    pending_run: Option<(RunFact, Option<Uuid>, bool)>,
    pub duplicate: Option<String>,
    saved_events: Vec<Event>,
    pub renamed_links: BTreeMap<String, Vec<String>>,
    link_titles: BTreeMap<String, (String, String)>,
    reordered: Vec<String>,
    dirty: bool,
    last_edit: Option<Instant>,
    last_check: Option<Instant>,
    /// The file changed on disk during an edit. Saving stops until the DM chooses.
    pub conflict: bool,
    pub merged_outside: bool,
    /// The file cannot be read as a timeline. It is left as it is, and nothing saves.
    pub unreadable: Option<String>,
    /// The file was here, and it is gone. Its events stay in memory until the DM chooses.
    pub gone: bool,
    pub error: Option<String>,
    /// What was removed, where it was, and the name it had there.
    removed: Option<(usize, Event, String)>,
    /// What a reload replaced, until the next edit.
    dropped: Option<Vec<Event>>,
    /// One ruling change, until the next edit.
    rest_undo: Option<(Event, Event)>,
    /// Each session's note name, for links in the file's body.
    pub session_links: BTreeMap<Uuid, String>,
}

impl Book {
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
        *self = Self {
            path,
            session_links: std::mem::take(&mut self.session_links),
            ..Default::default()
        };
        self.reload();
    }

    fn check_copies(&mut self) {
        let Some(path) = &self.path else {
            self.duplicate = None;
            return;
        };
        self.duplicate = path.parent().and_then(|folder| fs::read_dir(folder).ok()).into_iter().flatten()
            .filter_map(Result::ok).map(|file| file.path())
            .filter(|other| other != path && other.extension().is_some_and(|ext| ext == "md"))
            .find(|other| fs::read_to_string(other).is_ok_and(|text| text.lines().any(|line| line.starts_with("timeline_version:"))))
            .map(|other| format!("Another timeline file exists: {}. Compare its events with Timeline.md before removing either file.", other.display()));
    }

    /// Read the file, and drop any edit not yet saved.
    pub fn reload(&mut self) {
        self.rest_undo = None;
        self.dirty = false;
        self.reordered.clear();
        self.conflict = false;
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
                        self.events = events;
                    }
                    Err(error) => {
                        self.events.clear();
                        self.unreadable = Some(format!(
                            "{FILE} could not be read, so Splat did not change it. {error}"
                        ));
                    }
                }
                self.baseline = Some(bytes);
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
        self.run_write = false;
        self.merged_outside = false;
        self.rest_undo = None;
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
        self.removed = None;
        self.dropped = None;
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
        crate::sessions::sync_folder(parent)?;
        self.flags = formats::parse_with_sessions(&text, &self.session_links).1;
        self.baseline = Some(text.into_bytes());
        self.events = events;
        self.saved_events.clone_from(&self.events);
        self.merged_outside = outside;
        self.dirty = false;
        self.run_write = false;
        self.reordered.clear();
        self.conflict = false;
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
                        Ok(merged) => self.events = merged,
                        Err(error) => {
                            self.error = Some(error);
                            return;
                        }
                    }
                    self.baseline = Some(bytes);
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
        if let Err(error) = self.write() {
            self.error = Some(format!("The timeline was not saved. {error}"));
        }
    }

    /// An edit is waiting for its pause before it saves.
    pub fn unsaved(&self) -> bool {
        self.dirty
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
        self.dropped = Some(events);
    }

    pub fn dropped(&self) -> Option<usize> {
        self.dropped.as_ref().map(Vec::len)
    }

    pub fn undo_reload(&mut self) {
        if let Some(events) = self.dropped.take() {
            self.events = events;
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

    pub fn not_saved(&self) -> bool {
        self.dirty && (self.conflict || self.error.is_some())
    }

    pub fn poll(&mut self, ctx: &eframe::egui::Context) {
        let now = Instant::now();
        if self.dirty {
            // Test builds save only when told to, so a capture never depends on the clock.
            #[cfg(not(test))]
            if !self.conflict
                && self.error.is_none()
                && self
                    .last_edit
                    .is_none_or(|at| now.duration_since(at) >= SAVE_AFTER)
            {
                self.save();
            }
            #[cfg(test)]
            let _ = SAVE_AFTER;
            ctx.request_repaint_after(SAVE_AFTER);
        } else if self.path.is_some()
            && !self.gone
            && self
                .last_check
                .is_none_or(|at| now.duration_since(at) >= CHECK_DISK_EVERY)
        {
            self.last_check = Some(now);
            self.check_copies();
            let current = self.path.as_ref().and_then(|path| fs::read(path).ok());
            if current != self.baseline {
                self.reload();
                if self.unreadable.is_none()
                    && let Some((fact, session, played)) = self.pending_run.take()
                {
                    self.apply_run(&fact, session, played);
                }
            }
        }
    }

    /// Add after the session's last event, or at the end. A rest found in an old recording
    /// belongs with its session, not after what happened since.
    pub fn add(&mut self, kind: Kind, session: Option<Uuid>) -> String {
        let id = new_id();
        let at = session
            .and_then(|session| {
                self.events
                    .iter()
                    .rposition(|event| event.session == Some(session))
            })
            .map_or(self.events.len(), |index| index + 1);
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
        self.rest_undo = Some((before, after));
    }

    pub fn rest_undo_id(&self) -> Option<&str> {
        self.rest_undo
            .as_ref()
            .filter(|(_, after)| self.events.contains(after))
            .map(|(_, after)| after.id.as_str())
    }

    pub fn undo_restores(&mut self) {
        if self.rest_undo_id().is_none() {
            return;
        }
        if let Some((before, _)) = self.rest_undo.take() {
            let event = self.event_mut(&before.id).expect("the unchanged rest");
            *event = before;
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
        if let Some(index) = self.events.iter().position(|event| event.id == id) {
            let label = self.events[index].label(day_numbers(&self.events)[index]);
            let event = self.events.remove(index);
            self.touch();
            self.removed = Some((index, event, label));
        }
    }

    pub fn removed_row(&self) -> Option<(usize, String)> {
        self.removed
            .as_ref()
            .map(|(at, _, label)| (*at, label.clone()))
    }

    /// The event that Undo would put back.
    pub fn removed_event(&self) -> Option<&Event> {
        self.removed.as_ref().map(|(_, event, _)| event)
    }

    pub fn removed(&self) -> Option<&str> {
        self.removed.as_ref().map(|(.., label)| label.as_str())
    }

    pub fn undo_remove(&mut self) -> Option<String> {
        let (index, event, _) = self.removed.take()?;
        let id = event.id.clone();
        self.events.insert(index.min(self.events.len()), event);
        self.touch();
        Some(id)
    }

    /// Every run sits on the timeline without a step from the DM. The DM's words stay, and
    /// so does the DM's undo, because a new round is not an edit.
    /// What a run says now. `played` means it changed while open, and it was not only opened.
    pub fn record_run(&mut self, fact: &RunFact, session: Option<Uuid>, played: bool) {
        // It stays in place for this, because what comes next must follow its order.
        let removed = self.removed.clone();
        let dm_edit = self.dirty && !self.run_write;
        self.apply_run(fact, session, played);
        self.run_write = !dm_edit;
        self.removed = removed;
    }

    /// The order for what is played or ruled on next.
    pub fn next_order(&self) -> u64 {
        self.events
            .iter()
            .chain(self.removed.iter().map(|(_, event, _)| event))
            .map(|event| event.played_order)
            .max()
            .unwrap_or(0)
            + 1
    }

    /// The open run's facts changed, from `before`. A run that was already over when it opened
    /// is being read. It adds nothing to the story, whichever session is now current.
    pub fn record_change(
        &mut self,
        before: Option<&RunFact>,
        fact: &RunFact,
        session: Option<Uuid>,
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
        self.record_run(fact, session, played);
    }

    fn apply_run(&mut self, fact: &RunFact, session: Option<Uuid>, played: bool) {
        if self.unreadable.is_some() {
            self.pending_run = Some((fact.clone(), session, played));
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
                    let event = &mut self.events[index];
                    let moved = session.is_some()
                        && event.session != session
                        && event.continued_in != session;
                    if moved {
                        event.continued_in = session;
                    }
                    // Zero is from a file older than this order, and is never the latest.
                    if event.played_order == 0 || event.played_order + 1 != next {
                        event.played_order = next;
                        self.touch();
                    } else if moved {
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
            }
        }
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
        book.record_change(None, &fact(RunState::Running), None);
        let mut repaired = event(Kind::Travel, "1 hour");
        repaired.id = "t-repaired".into();
        fs::write(&path, render(&[repaired], &BTreeMap::new()).unwrap()).unwrap();
        book.poll(&eframe::egui::Context::default());
        assert!(book.unreadable.is_none());
        assert_eq!(book.events.len(), 2);
        assert_eq!(book.events[1].run, Some(fact(RunState::Running).id));
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
        assert!(
            book.duplicate
                .as_deref()
                .unwrap()
                .contains("Timeline (conflict).md")
        );
        assert!(copy.exists());
    }

    #[test]
    fn integrity_d22_a_recovery_copy_keeps_one_event_and_its_notes() {
        let mut book = Book::default();
        let original = fact(RunState::Running);
        book.record_change(None, &original, None);
        let event = book.events[0].id.clone();
        book.events[0].notes = "DM's account".into();
        let copy = RunFact {
            id: Uuid::new_v4(),
            copied_from: Some(original.id),
            ..original.clone()
        };
        book.record_change(Some(&original), &copy, None);
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
        timeline.record_change(None, &fact(RunState::Running), None);
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
        book.record_change(None, &ended, Some(Uuid::new_v4()));
        assert!(book.events.is_empty(), "it was over when it opened");

        // A run that ends while open is the story.
        let running = fact(RunState::Running);
        book.record_change(None, &running, None);
        assert_eq!(book.events.len(), 1);
        book.record_change(Some(&running), &ended, None);
        assert!(book.events[0].result.starts_with("Party won"));

        // Keep both, on a run that ended: the copy has a new ID, and it is the story too.
        let copy = RunFact {
            id: Uuid::new_v4(),
            ..ended.clone()
        };
        book.record_change(Some(&ended), &copy, None);
        assert_eq!(book.events.len(), 2);

        // Opened again, it keeps its event up to date.
        book.record_change(None, &ended, None);
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
            extra: Default::default(),
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
        let (fight, run) = hp_fight("Setup · Paused", Standing::Unfinished);
        assert!(
            fights_that_keep_hp(std::slice::from_ref(&fight), fight.session, &[run]).is_empty()
        );
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
        for words in ["a few hours", "soon", "", "3", "-1 day", "3 days and a bit"] {
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
        assert!(
            event(Kind::Travel, "a few hours")
                .summary(None)
                .contains("time not read")
        );
        assert!(!event(Kind::Travel, "3 days").time_not_read());
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
        book.undo_reload();
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
    fn old_timeline_is_flagged_and_not_loaded() {
        let dir = tempfile::tempdir().unwrap();
        let text = "---\ntimeline_version: 1\ntimeline: []\n---\n";
        assert_eq!(parse(text).unwrap_err(), splat::content::OLD_LAYOUT);
        fs::write(dir.path().join(FILE), text).unwrap();
        let book = book(dir.path());
        assert!(book.events.is_empty());
        assert!(
            book.unreadable
                .as_ref()
                .unwrap()
                .contains(splat::content::OLD_LAYOUT)
        );
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
        assert_eq!(book.undo_remove(), Some(rest));
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
        assert_eq!(book.undo_remove(), Some(rest));
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
        assert_eq!(book.undo_remove(), Some(rest.clone()));
        assert_eq!(book.events[1].id, rest);
        book.remove(&rest);
        book.add(Kind::Day, None);
        assert!(book.undo_remove().is_none());
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
        book.undo_restores();
        assert!(party_state(&book.events, session).rested);
        assert!(book.rest_undo_id().is_none());
        book.set_restores(&id, false);
        book.set_restores(&id, true);
        assert!(party_state(&book.events, session).rested);
        assert_eq!(book.event_mut(&id).unwrap().played_order, order);
        book.undo_restores();
        assert_eq!(party_state(&book.events, session), before);
        book.remove(&id);
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
        book.undo_restores();
        assert!(book.event_mut(&id).unwrap().restores);
        book.set_restores(&id, false);
        book.add(Kind::Travel, session);
        assert!(book.rest_undo_id().is_none());
        book.undo_restores();
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
