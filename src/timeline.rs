//! The campaign's timeline: every encounter, rest, travel interval, day and major event,
//! in story order. It stands apart from session notes. A note refers to an event by its
//! stable ID, and the same link works in Obsidian, because each event is a block in
//! `Timeline.md`. The list in the frontmatter is the record. Splat writes the body from it.
pub mod page;

use crate::design::icons;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use uuid::Uuid;

pub const FILE: &str = "Timeline.md";
const VERSION: u32 = 1;
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
            Self::Encounter => "Encounter",
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
}

impl Event {
    /// The name people read. A day has its number, and it can also have a name.
    pub fn label(&self, day: u32) -> String {
        match (self.kind, self.title.trim()) {
            (Kind::Day, "") => format!("Day {day}"),
            (Kind::Day, title) => format!("Day {day}: {title}"),
            (kind, "") => kind.word().to_owned(),
            (_, title) => title.to_owned(),
        }
    }

    /// The second line of a row: what is known, with the session if one is given.
    pub fn summary(&self, session: Option<&str>) -> String {
        let mut parts = Vec::new();
        // An untitled event already shows its kind as its name.
        if self.kind != Kind::Day && !self.title.trim().is_empty() {
            parts.push(self.kind.word());
        }
        for part in [&self.duration, &self.place, &self.result] {
            if !part.trim().is_empty() {
                parts.push(part.as_str());
            }
        }
        if let Some(session) = session {
            parts.push(session);
        }
        parts.join(" · ")
    }
}

/// What a run tells the timeline. A new value of this means something changed.
#[derive(Clone, Debug, PartialEq)]
pub struct RunFact {
    pub id: Uuid,
    pub name: String,
    pub round: u64,
    pub state: RunState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RunState {
    Setup,
    Running,
    Ended(String),
}

#[derive(Serialize, Deserialize)]
struct Front {
    timeline_version: u32,
    #[serde(default)]
    timeline: Vec<Event>,
}

/// The day each event falls on, in list order. Zero means before the first day.
pub fn day_numbers(events: &[Event]) -> Vec<u32> {
    let mut day = 0;
    events
        .iter()
        .map(|event| {
            if event.kind == Kind::Day {
                day += 1;
            }
            day
        })
        .collect()
}

/// A link to one event. Obsidian follows it to the event's block in `Timeline.md`.
pub fn reference(event: &Event, day: u32) -> String {
    let label: String = event
        .label(day)
        .chars()
        .filter(|c| !matches!(c, '[' | ']' | '|' | '#' | '^'))
        .collect();
    format!("[[Timeline#^{}|{}]]", event.id, label.trim())
}

/// Call `found` with each timeline link in a text: the ID and the label, if it has one.
fn each_reference<'a>(
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

/// A note for the Reader: each link becomes the words it stands for.
pub fn for_reading(text: &str, events: &[Event]) -> String {
    let days = day_numbers(events);
    let mut out = String::with_capacity(text.len());
    let mut copied = 0;
    each_reference(text, |start, end, id, label| {
        out.push_str(&text[copied..start]);
        // The event's name now, not the name it had when the link was made.
        match events.iter().position(|event| event.id == id) {
            Some(index) => out.push_str(&events[index].label(days[index])),
            None => match label {
                Some(label) => out.push_str(&format!("{label} (removed from the timeline)")),
                None => out.push_str("(event not on the timeline)"),
            },
        }
        copied = end;
    });
    out.push_str(&text[copied..]);
    out
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
    let front = serde_yaml_ng::to_string(&Front {
        timeline_version: VERSION,
        timeline: events.to_vec(),
    })
    .map_err(|e| e.to_string())?;
    let mut text = format!(
        "---\n{front}---\n\n# Timeline\n\n\
         Splat writes this page from the list above. Edit the list, or use Splat.\n"
    );
    let days = day_numbers(events);
    let mut open_list = false;
    for (event, day) in events.iter().zip(days) {
        if event.kind == Kind::Day {
            text.push_str(&format!(
                "\n## {} ^{}\n\n",
                event.label(day).replace(['\n', '^'], " "),
                event.id
            ));
            open_list = true;
            continue;
        }
        if !open_list {
            text.push('\n');
            open_list = true;
        }
        let session = event.session.and_then(|id| sessions.get(&id));
        let summary = event.summary(None);
        let mut line = format!("- **{}**", event.label(day));
        if !summary.is_empty() {
            line.push_str(&format!(": {summary}"));
        }
        if let Some(session) = session {
            line.push_str(&format!(" ([[{session}]])"));
        }
        text.push_str(&line.replace('\n', " "));
        text.push_str(&format!(" ^{}\n", event.id));
    }
    Ok(text)
}

fn parse(text: &str) -> Result<Vec<Event>, String> {
    let text = text.trim_start_matches('\u{feff}').replace("\r\n", "\n");
    let rest = text
        .strip_prefix("---\n")
        .ok_or("It has no frontmatter list.")?;
    let end = rest
        .find("\n---\n")
        .or_else(|| rest.strip_suffix("\n---").map(str::len))
        .ok_or("Its frontmatter has no end.")?;
    let front: Front = serde_yaml_ng::from_str(&rest[..end]).map_err(|e| e.to_string())?;
    if front.timeline_version != VERSION {
        return Err(format!(
            "Timeline version {} is not supported.",
            front.timeline_version
        ));
    }
    let mut seen = std::collections::BTreeSet::new();
    for event in &front.timeline {
        if event.id.trim().is_empty() || !seen.insert(&event.id) {
            return Err(format!("The event ID “{}” is not unique.", event.id));
        }
    }
    Ok(front.timeline)
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
    baseline: Option<Vec<u8>>,
    dirty: bool,
    last_edit: Option<Instant>,
    last_check: Option<Instant>,
    /// The file changed on disk during an edit. Saving stops until the DM chooses.
    pub conflict: bool,
    /// The file cannot be read as a timeline. It is left as it is, and nothing saves.
    pub unreadable: Option<String>,
    /// The file was here, and it is gone. Its events stay in memory until the DM chooses.
    pub gone: bool,
    pub error: Option<String>,
    /// What was removed, where it was, and the name it had there.
    removed: Option<(usize, Event, String)>,
    /// What a reload replaced, until the next edit.
    dropped: Option<Vec<Event>>,
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
        *self = Self {
            path,
            session_links: std::mem::take(&mut self.session_links),
            ..Default::default()
        };
        self.reload();
    }

    /// Read the file, and drop any edit not yet saved.
    pub fn reload(&mut self) {
        self.dirty = false;
        self.conflict = false;
        self.error = None;
        self.unreadable = None;
        self.gone = false;
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
                    Ok(events) => self.events = events,
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
        self.dirty = true;
        self.last_edit = Some(Instant::now());
        self.removed = None;
        self.dropped = None;
    }

    fn write(&mut self) -> Result<(), String> {
        let path = self.path.as_ref().ok_or("No campaign is open.")?;
        let text = render(&self.events, &self.session_links)?;
        let parent = path.parent().ok_or("Invalid timeline location.")?;
        fs::create_dir_all(parent).map_err(|e| format!("Could not create the folder: {e}"))?;
        let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
        temp.write_all(text.as_bytes())
            .and_then(|()| temp.as_file().sync_all())
            .map_err(|e| e.to_string())?;
        temp.persist(path).map_err(|e| e.to_string())?;
        self.baseline = Some(text.into_bytes());
        self.dirty = false;
        self.conflict = false;
        self.error = None;
        Ok(())
    }

    pub fn save(&mut self) {
        if !self.dirty || self.conflict || self.unreadable.is_some() {
            return;
        }
        let Some(path) = &self.path else {
            return;
        };
        let current = fs::read(path).ok();
        if current != self.baseline {
            self.conflict = true;
            return;
        }
        if let Err(error) = self.write() {
            self.error = Some(format!("The timeline was not saved. {error}"));
        }
    }

    /// The file changed on disk, and the DM keeps this version.
    pub fn overwrite(&mut self) {
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
            && self.unreadable.is_none()
            && self
                .last_check
                .is_none_or(|at| now.duration_since(at) >= CHECK_DISK_EVERY)
        {
            self.last_check = Some(now);
            let current = self.path.as_ref().and_then(|path| fs::read(path).ok());
            if current != self.baseline {
                self.reload();
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
            },
        );
        self.touch();
        id
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

    /// Move an event up or down the list. Returns false at either end.
    pub fn move_by(&mut self, id: &str, by: isize) -> bool {
        let Some(index) = self.events.iter().position(|event| event.id == id) else {
            return false;
        };
        let Some(target) = index
            .checked_add_signed(by)
            .filter(|target| *target < self.events.len())
        else {
            return false;
        };
        self.events.swap(index, target);
        self.touch();
        true
    }

    /// Every run sits on the timeline without a step from the DM. The DM's words stay, and
    /// so does the DM's undo, because a new round is not an edit.
    pub fn record_run(&mut self, fact: &RunFact, session: Option<Uuid>) {
        let removed = self.removed.take();
        self.apply_run(fact, session);
        self.removed = removed;
    }

    /// The open run's facts changed, from `before`. A run that was already over when it opened
    /// is being read. It adds nothing to the story, whichever session is now current.
    pub fn record_change(
        &mut self,
        before: Option<&RunFact>,
        fact: &RunFact,
        session: Option<Uuid>,
    ) {
        let opened = before.is_none_or(|before| before.id != fact.id);
        let known = self.events.iter().any(|event| event.run == Some(fact.id));
        if opened && !known && matches!(fact.state, RunState::Ended(_)) {
            return;
        }
        self.record_run(fact, session);
    }

    fn apply_run(&mut self, fact: &RunFact, session: Option<Uuid>) {
        if self.unreadable.is_some() {
            return;
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
            }
            None => {
                let id = self.add(Kind::Encounter, session);
                let event = self
                    .events
                    .iter_mut()
                    .find(|event| event.id == id)
                    .expect("the event just added");
                event.title = fact.name.clone();
                event.run = Some(fact.id);
                event.result = result;
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

    fn book(folder: &Path) -> Book {
        let mut book = Book::default();
        book.open(Some(folder));
        book
    }

    fn fact(state: RunState) -> RunFact {
        RunFact {
            id: Uuid::from_u128(7),
            name: "Bridge Ambush".into(),
            round: 3,
            state,
        }
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

        // Opened again, it keeps its event up to date.
        book.record_change(None, &ended, None);
        assert_eq!(book.events.len(), 1);
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
            text.contains(&format!("## Day 1: Harvest Eve ^{day}")),
            "{text}"
        );
        assert!(text.contains(&format!("- **Long rest** ^{rest}")), "{text}");
        assert!(text.contains("3 days"), "{text}");
        // The catalogue scan treats any `splat_` frontmatter line as an entry.
        assert!(!text.lines().any(|line| line.starts_with("splat_")));
        assert_eq!(book(dir.path()).events, first.events);
    }

    #[test]
    fn days_number_in_list_order_and_events_keep_the_day_they_follow() {
        let mut book = Book::default();
        book.add(Kind::Event, None);
        let first = book.add(Kind::Day, None);
        book.add(Kind::LongRest, None);
        book.add(Kind::Day, None);
        assert_eq!(day_numbers(&book.events), [0, 1, 1, 2]);
        assert!(book.move_by(&first, -1));
        assert_eq!(day_numbers(&book.events), [1, 1, 1, 2]);
        assert!(!book.move_by(&book.events[0].id.clone(), -1));
    }

    #[test]
    fn a_reference_resolves_and_reads_as_its_words() {
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
        assert_eq!(
            for_reading(&note, &book.events),
            "We fought at Bridge [Ambush], on Day 1. Then (event not on the timeline)."
        );
        // The name now, and a removed event says so.
        book.event_mut(&id).unwrap().title = "The bridge".into();
        assert_eq!(for_reading(&link, &book.events), "The bridge");
        let gone = "[[Timeline#^gone|Old words]]";
        assert_eq!(
            for_reading(gone, &book.events),
            "Old words (removed from the timeline)"
        );
        assert_eq!(
            for_reading("no links [[Other]]", &book.events),
            "no links [[Other]]"
        );
    }

    #[test]
    fn a_change_on_disk_stops_saving_until_the_dm_chooses() {
        let dir = tempfile::tempdir().unwrap();
        let mut book = book(dir.path());
        book.add(Kind::Day, None);
        book.save();
        let path = dir.path().join(FILE);
        let outside = "---\ntimeline_version: 1\ntimeline:\n- id: t-outside\n  kind: event\n---\n";
        fs::write(&path, outside).unwrap();
        book.add(Kind::LongRest, None);
        book.save();
        assert!(book.conflict && book.not_saved());
        assert_eq!(fs::read_to_string(&path).unwrap(), outside);
        book.reload();
        assert!(!book.conflict);
        assert_eq!(book.events.len(), 1);
        assert_eq!(book.events[0].id, "t-outside");
        book.add(Kind::Day, None);
        fs::write(&path, outside.replace("t-outside", "t-again")).unwrap();
        book.save();
        book.overwrite();
        assert!(!book.conflict);
        assert_eq!(
            self::parse(&fs::read_to_string(&path).unwrap())
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn a_reload_in_a_conflict_has_one_step_of_undo() {
        let dir = tempfile::tempdir().unwrap();
        let mut book = book(dir.path());
        book.add(Kind::Day, None);
        book.save();
        let outside = "---\ntimeline_version: 1\ntimeline: []\n---\n";
        fs::write(dir.path().join(FILE), outside).unwrap();
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
        book.record_run(&fact(RunState::Running), None);
        assert!(book.events.is_empty());
        book.add(Kind::Day, None);
        book.save();
        assert_eq!(fs::read_to_string(&path).unwrap(), "written by hand");
    }

    #[test]
    fn a_duplicate_id_is_refused_when_read() {
        let text = "---\ntimeline_version: 1\ntimeline:\n- {id: a, kind: day}\n- {id: a, kind: day}\n---\n";
        assert!(parse(text).unwrap_err().contains("not unique"));
    }

    #[test]
    fn a_run_adds_one_event_keeps_the_dms_words_and_undo_takes_it_back() {
        let session = Uuid::from_u128(1);
        let mut book = Book::default();
        book.record_run(&fact(RunState::Setup), Some(session));
        assert!(book.events.is_empty());

        book.record_run(&fact(RunState::Running), Some(session));
        assert_eq!(book.events.len(), 1);
        assert_eq!(book.events[0].kind, Kind::Encounter);
        assert_eq!(book.events[0].session, Some(session));
        assert_eq!(book.events[0].result, "Round 3");

        book.events[0].title = "The bridge at dusk".into();
        book.record_run(&fact(RunState::Ended("Party won".into())), Some(session));
        assert_eq!(book.events.len(), 1);
        assert_eq!(book.events[0].title, "The bridge at dusk");
        assert_eq!(book.events[0].result, "Party won, round 3");

        book.record_run(&fact(RunState::Setup), Some(session));
        assert!(book.events.is_empty());

        book.record_run(&fact(RunState::Running), Some(session));
        book.events[0].notes = "The ogre fled.".into();
        book.record_run(&fact(RunState::Setup), Some(session));
        assert_eq!(book.events.len(), 1);
        assert_eq!(book.events[0].result, "");
    }

    #[test]
    fn a_new_round_is_not_an_edit_so_undo_stays() {
        let mut book = Book::default();
        let rest = book.add(Kind::LongRest, None);
        book.record_run(&fact(RunState::Running), None);
        book.remove(&rest);
        let next = RunFact {
            round: 4,
            ..fact(RunState::Running)
        };
        book.record_run(&next, None);
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
        book.record_run(&fact(RunState::Running), None);
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
