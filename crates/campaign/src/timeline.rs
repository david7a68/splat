//! Timeline events, the campaign clock, and the order in which events were played.
pub mod formats;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const FILE: &str = "Timeline.md";

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
    /// A journey's encounter checks roll on the marked table with this key. An empty key holds
    /// the checks while no table is chosen; none means the DM never asked for checks here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks_table: Option<String>,
    /// The journey's rolled encounter checks, in the file's order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checks: Vec<Check>,
    /// A fight begun from a journey's check is played inside that journey, at that check.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub within: Option<Within>,
    #[serde(flatten)]
    pub extra: serde_yaml_ng::Mapping,
}

/// Where a fight inside a journey is played: the journey's event and the check's offset from
/// its start, in hours. The clock puts the fight there; the journey keeps its length.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Within {
    pub journey: String,
    pub at: u64,
}

/// One rolled encounter check of a journey: `- Check at 24h: "16 · [[Wolf Pack]] pick up the
/// trail."`. It belongs to its offset from the journey's start, never to a day's name.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Check {
    /// The line's label as the file has it. `Clear` finds the line by it.
    pub label: String,
    /// Hours from the journey's start, when the label can be read.
    pub at: Option<u64>,
    /// The result as the file holds it, unquoted: the number, `·`, and the row's words.
    pub words: String,
}

impl Check {
    pub fn new(at: u64, words: String) -> Self {
        Self {
            label: format!("Check at {at}h"),
            at: Some(at),
            words,
        }
    }

    /// A label from the file: `Check at 24h`, in any case. Its offset, when it can be read.
    pub fn read(label: &str, words: String) -> Option<Self> {
        let rest = label.to_lowercase();
        let offset = rest.strip_prefix("check at ")?;
        Some(Self {
            label: label.to_owned(),
            at: offset
                .trim()
                .strip_suffix('h')
                .and_then(|h| h.trim().parse().ok()),
            words,
        })
    }
}

/// What a heading or a link stores for an event: its kind, its id and its title exactly as the
/// DM or the plan wrote it. The file's writer and the link writer take this, never an
/// [`Event`], so they cannot reach the shown [`Event::label`], which drops a title that only says
/// the kind (a day titled `Day` must stay `## Day 1: Day` in the file, or the next save would
/// read it back with no title).
// Its fields are private: outside `timeline` the only way to get one is `Stored::of(&Event)`, so
// no caller can build it from a shown label (a doc test cannot name it: `timeline` is private).
#[derive(Clone, Copy)]
pub struct Stored<'a> {
    kind: Kind,
    id: &'a str,
    title: &'a str,
}

impl<'a> Stored<'a> {
    pub fn of(event: &'a Event) -> Self {
        Self {
            kind: event.kind,
            id: &event.id,
            title: &event.title,
        }
    }

    /// The stored name: a day's number, then the title exactly, when it has one.
    pub fn name(self, day: u32) -> String {
        match (self.kind, self.title.trim()) {
            (Kind::Day, "") => format!("Day {day}"),
            (Kind::Day, title) => format!("Day {day}: {title}"),
            (kind, "") => kind.word().to_owned(),
            (_, title) => title.to_owned(),
        }
    }
}

impl Event {
    /// Does this event belong to this session, or go on in it?
    pub fn is_in(&self, session: Option<Uuid>) -> bool {
        self.session == session || (session.is_some() && self.continued_in == session)
    }

    /// The event's own name: its title, unless that is empty or only says its kind (the plan
    /// gives a new event its kind as its title, `Day` or `Travel`). Every place that shows the
    /// kind beside the name asks this one question, so no line says the kind twice (round 28
    /// FL8, FN3, FH7).
    pub fn own_title(&self) -> Option<&str> {
        let title = self.title.trim();
        (!title.is_empty() && !title.eq_ignore_ascii_case(self.kind.word())).then_some(title)
    }

    /// The name people read on screen. A day has its number, and it can also have a name. It
    /// drops a title that only says the kind, so it is never written to a file or a link: the
    /// writers take [`Stored`], which holds the title exactly and has no shown label.
    pub fn label(&self, day: u32) -> String {
        match (self.kind, self.own_title()) {
            (Kind::Day, None) => format!("Day {day}"),
            (Kind::Day, Some(title)) => format!("Day {day}: {title}"),
            (kind, None) => kind.word().to_owned(),
            (_, Some(title)) => title.to_owned(),
        }
    }

    /// How far this event moves the campaign's clock, in minutes.
    pub fn minutes(&self) -> u64 {
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

pub const HOUR: u64 = 60;
pub const DAY: u64 = 24 * HOUR;
const MONTH: u64 = 30 * DAY;
const YEAR: u64 = 12 * MONTH;

/// A length in the closed form the clock can count, in minutes: `3 days`, `an hour`.
pub fn read_duration(text: &str) -> Option<u64> {
    let mut words = text.split_whitespace();
    let number = match words.next()?.to_lowercase().as_str() {
        "a" | "an" => 1.0,
        // Round 26 FD7: `half a day` and `half an hour` are lengths a DM writes.
        "half" => match words.next()?.to_lowercase().as_str() {
            "a" | "an" => 0.5,
            _ => return None,
        },
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
    walk(events).0
}

/// Each event's start, and the clock after them all.
fn walk(events: &[Event]) -> (Vec<Option<u64>>, Option<u64>) {
    let mut now = None;
    let mut starts: Vec<Option<u64>> = Vec::with_capacity(events.len());
    for event in events {
        if let Some(start) = inside_journey(&events[..starts.len()], &starts, event) {
            starts.push(start);
            // The clock after a journey with fights in it is the later of the journey's end
            // and each fight's end, so a long fight never ends after `Now`.
            now = now.max(start.map(|start| start.saturating_add(event.minutes())));
            continue;
        }
        if event.kind == Kind::Day {
            // Always the next boundary, so forgotten durations cannot make the count drift.
            now = Some(now.map_or(0, |minute: u64| (minute / DAY + 1) * DAY));
        }
        starts.push(now);
        now = now.map(|minute| minute.saturating_add(event.minutes()));
    }
    (starts, now)
}

/// A fight inside a journey starts at its check: while it is a fight that names a journey
/// before it in the list, and that journey still covers the check. Any other event, or a fight
/// whose journey no longer holds it, is an ordinary event in the sequence.
/// Whether the journey covers a check at this offset, in hours: before its end, or the one
/// check of a journey whose length the clock cannot read. The checks block, Begin combat and
/// the clock all ask this.
pub fn covers(journey: &Event, at: u64) -> bool {
    check_fits(at, 1, read_duration(journey.duration.trim()))
}

/// Whether a check at this offset is one of the journey's: a whole interval from the start,
/// and before the end.
pub fn check_fits(at: u64, every: u64, length: Option<u64>) -> bool {
    at.is_multiple_of(every) && length.map_or(at == 0, |minutes| at * HOUR < minutes)
}

fn inside_journey(before: &[Event], starts: &[Option<u64>], event: &Event) -> Option<Option<u64>> {
    let within = event
        .within
        .as_ref()
        .filter(|_| event.kind == Kind::Encounter)?;
    let at = before
        .iter()
        .position(|journey| journey.id == within.journey && journey.kind == Kind::Travel)?;
    let offset = within.at.checked_mul(HOUR)?;
    covers(&before[at], within.at).then(|| starts[at].map(|start| start.saturating_add(offset)))
}

/// Round 27 FE10: each fight played at a journey's check, by its ID, with its day on the road in
/// the plan row's words (`Day 2 of Travel`), while its journey is before it and covers the check.
pub fn road_days(events: &[Event]) -> std::collections::HashMap<String, String> {
    let starts = clock(events);
    events
        .iter()
        .enumerate()
        .filter_map(|(index, event)| {
            let within = event
                .within
                .as_ref()
                .filter(|_| event.kind == Kind::Encounter)?;
            let at = events[..index]
                .iter()
                .position(|journey| journey.id == within.journey && journey.kind == Kind::Travel)?;
            if !covers(&events[at], within.at) {
                return None;
            }
            let minute = starts[at]
                .unwrap_or(0)
                .saturating_add(within.at.saturating_mul(HOUR));
            Some((
                event.id.clone(),
                format!("Day {} of {}", minute / DAY + 1, events[at].label(0)),
            ))
        })
        .collect()
}

/// The clock after every event, for the page to say where the story is now.
pub fn clock_now(events: &[Event]) -> Option<u64> {
    walk(events).1
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
    // A link's words are written into notes and compared with the file's names, so they are
    // the stored name.
    let label: String = Stored::of(event)
        .name(day)
        .chars()
        .filter(|c| !matches!(c, '[' | ']' | '|' | '#' | '^'))
        .collect();
    label.trim().to_owned()
}

pub fn reference(event: &Event, day: u32) -> String {
    format!("[[Timeline#^{}|{}]]", event.id, reference_label(event, day))
}

/// Call `found` with each timeline link in a text: the ID and the label, if it has one.
pub fn each_reference<'a>(
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

pub fn parse(text: &str) -> Result<Vec<Event>, String> {
    let (front, _) = crate::note::split_note(text).ok_or("Missing timeline properties.")?;
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
        _ => return Err("Unsupported timeline version (expected 2).".into()),
    }
    Ok(formats::parse(text).0)
}

/// What prevents a closed fight from continuing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContinueFault {
    Unreadable,
    MissingEvent,
    LaterPlay,
}

/// Only the last fight played may continue. A later long rest counts even if it restored no HP.
pub fn can_continue(events: Option<&[Event]>, run: Uuid) -> Result<(), ContinueFault> {
    let events = events.ok_or(ContinueFault::Unreadable)?;
    if !events.iter().any(|event| event.run == Some(run)) {
        return Err(ContinueFault::MissingEvent);
    }
    let last = events
        .iter()
        .enumerate()
        .filter(|(_, event)| event.run.is_some() || event.kind == Kind::LongRest)
        .max_by_key(|(at, event)| (event.played_order, *at))
        .and_then(|(_, event)| event.run);
    if last == Some(run) {
        Ok(())
    } else {
        Err(ContinueFault::LaterPlay)
    }
}

/// The next place in the campaign's order of play.
pub fn next_order<'a>(events: impl IntoIterator<Item = &'a Event>) -> u64 {
    events
        .into_iter()
        .map(|event| event.played_order)
        .max()
        .unwrap_or(0)
        + 1
}
