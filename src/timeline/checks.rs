//! A journey's encounter checks: one roll on a marked table for each day or watch of a Travel
//! event, kept in that event's lines of `Timeline.md` by its offset from the journey's start.
//! The plan's Travel row and Timeline's detail column draw the same block from here.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{Book, Check, DAY, Event, HOUR, Kind, clock_words, read_duration};
use crate::design::{self, Action, Density, Status, Tone, Type, colors, icons};
use crate::roll_table::{Per, Table};
use eframe::egui;
use splat::execution::dice::Random;
use std::collections::BTreeMap;

/// A journey with more checks than this lists none: it is too long to roll here.
const MOST_CHECKS: usize = 31;
/// The most `Plan “…”` offers under one result.
const MOST_OFFERS: usize = 3;

/// The "How long" field of an open event. Both of its forms (with checks to follow and without)
/// use this id, so the focus and the cursor stay when the event gains or loses its checks.
const LENGTH: design::FieldId = crate::field_id!("event_length");

/// The "How long" field of this event. egui keeps a field's cursor and text Undo under its id, so
/// each event has its own id: another event's length never gets this one's history.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
)]
pub(super) fn length_field(event: &Event) -> design::FieldId {
    LENGTH.with(design::TextKey::from_data(event.id.as_str()))
}

/// What the DM asked for in a journey's checks. One press in a frame.
#[derive(Clone, Debug, PartialEq)]
pub enum Act {
    /// `Roll for encounters`: the block, on the table that the earlier choices name.
    Start,
    /// A table from the menu, or `No checks`.
    Choose(Option<String>),
    /// `Roll` on one line, or `Roll all`: the offsets in hours.
    Roll(Vec<u64>),
    /// `Enter a roll`: a player's own roll for one line.
    Enter(u64, i64),
    /// `Clear` on a result: its label and its words.
    Clear(String, String),
    /// `Plan “<name>”` on a result that links an encounter or a group.
    Plan(Planned),
}

/// What `Plan “<name>”` acted on: the name, and the check the DM clicked it on, by its label
/// and words as read.
#[derive(Clone, Debug, PartialEq)]
pub struct Planned {
    pub name: String,
    pub check: (String, String),
}

impl Planned {
    /// The plan line's message.
    pub fn words(&self) -> String {
        plan_line_words(&self.name)
    }

    /// Whether this is the check the DM clicked.
    pub fn on(&self, check: &Check) -> bool {
        self.check.0 == check.label && self.check.1 == check.words
    }
}

/// The message of `Plan “<name>”`.
fn plan_line_words(name: &str) -> String {
    format!("“{name}” is in tonight’s plan.")
}

/// The mark that `Plan “<name>”` leaves at the end of its plan line, which no reader shows: the
/// journey, and the check as read (its offset, and the sum of its words). Starting the fight
/// reads it, so the fight is played at that check.
#[derive(Clone, Debug, PartialEq)]
pub struct RoadMark {
    journey: String,
    at: u64,
    sum: String,
}

const ROAD_MARK: &str = "<!-- splat:check ";

/// Why a fight planned from a check is not played at it, in the words of the run's notice.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OffRoad {
    /// The check, or its journey, changed or went since the plan line; or the mark is not read.
    Changed,
    /// The journey is another session's: the line was carried over or copied.
    OtherSession,
    /// The note or `Timeline.md` cannot be read now.
    NotRead,
}

impl OffRoad {
    pub fn words(self) -> &'static str {
        match self {
            Self::Changed => "Its travel check changed, so this fight follows the last event.",
            Self::OtherSession => {
                "Its journey is in an earlier session, so this fight follows the last event."
            }
            Self::NotRead => {
                "Splat cannot read its travel check now, so this fight follows the last event."
            }
        }
    }
}

/// A plan line read for its mark: the line's own words, with the mark taken out and the text
/// before and after it kept byte for byte, and its mark, if it has one. Half a comment is no
/// mark and stays in the words; a whole one whose terms cannot be read is taken out of the
/// words and is no mark either: an ordinary line, an ordinary fight, and no notice.
pub struct MarkedLine {
    pub words: String,
    pub mark: Option<RoadMark>,
}

/// The one reader of a plan line's mark.
pub fn marked(line: &str) -> MarkedLine {
    let whole = line
        .split_once(ROAD_MARK)
        .and_then(|(before, rest)| Some((before, rest.split_once("-->")?)));
    let Some((before, (inner, after))) = whole else {
        return MarkedLine {
            words: line.to_owned(),
            mark: None,
        };
    };
    let term = |key: &str| {
        inner
            .split_whitespace()
            .find_map(|term| term.strip_prefix(key)?.strip_prefix('='))
    };
    let mark = (|| {
        Some(RoadMark {
            journey: term("journey")?.to_owned(),
            at: term("at")?.parse().ok()?,
            sum: term("sum")?.to_owned(),
        })
    })();
    MarkedLine {
        words: format!("{before}{after}"),
        mark,
    }
}

/// A note's words as readers show them: each line's words without its mark, byte for byte.
pub fn without_marks(text: &str) -> String {
    text.split_inclusive('\n')
        .map(|line| marked(line).words)
        .collect()
}

impl RoadMark {
    /// The mark for a check of this journey, when its offset can be read.
    pub fn new(journey: &str, check: &Check) -> Option<Self> {
        Some(Self {
            journey: journey.to_owned(),
            at: check.at?,
            sum: words_sum(&check.words),
        })
    }

    /// The comment that ends the plan line.
    pub fn comment(&self) -> String {
        format!(
            "{ROAD_MARK}journey={} at={} sum={} -->",
            self.journey, self.at, self.sum
        )
    }

    /// Where the fight is played: in this session's journey at the check, while the journey
    /// still covers the check and the check is as the plan line read it. Else why not: the
    /// fight goes after the session's last event, as any fight does.
    pub fn place(
        &self,
        events: &[Event],
        session: Option<uuid::Uuid>,
    ) -> Result<super::Within, OffRoad> {
        let journey = events
            .iter()
            .find(|event| event.id == self.journey && event.kind == Kind::Travel)
            .ok_or(OffRoad::Changed)?;
        if journey.session != session {
            return Err(OffRoad::OtherSession);
        }
        let same = journey
            .checks
            .iter()
            .any(|check| check.at == Some(self.at) && words_sum(&check.words) == self.sum);
        (covers(journey, self.at) && same)
            .then(|| super::Within {
                journey: self.journey.clone(),
                at: self.at,
            })
            .ok_or(OffRoad::Changed)
    }

    /// The day the fight is played on, for its plan row: `Day 2 of Road to the ford`, counted
    /// as the check's name is. `starts` is the clock of `events`.
    pub fn day_words(
        &self,
        events: &[Event],
        starts: &[Option<u64>],
        session: Option<uuid::Uuid>,
    ) -> Option<String> {
        let within = self.place(events, session).ok()?;
        let at = events.iter().position(|event| event.id == within.journey)?;
        let minute = starts[at].unwrap_or(0) + within.at * HOUR;
        Some(format!(
            "Day {} of {}",
            minute / DAY + 1,
            events[at].label(0)
        ))
    }
}

/// The sum of a check's words, so a mark tells a check rolled again or edited from the one the
/// DM planned from: 64 bits of SHA-256, short enough that the mark stays one line in the note
/// and long enough that two different words do not meet by chance. Two rolls with the same
/// words are the same check.
fn words_sum(words: &str) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(words.as_bytes()))[..16].to_owned()
}

/// Whether the journey covers a check at this offset, in hours: before its end, or the one
/// check of a journey whose length the clock cannot read. The checks block, Begin combat and
/// the clock all ask this.
pub fn covers(journey: &Event, at: u64) -> bool {
    fits(at, 1, minutes(&journey.duration))
}

/// Whether this journey shows its checks: the DM chose `Roll for encounters` on it.
pub fn shown(event: &Event) -> bool {
    event.kind == Kind::Travel && (event.checks_table.is_some() || !event.checks.is_empty())
}

/// Whether a row's menu offers `Roll for encounters`: a journey with no checks yet, in a
/// campaign that has a marked table.
pub fn offered(event: &Event, tables: &[Table]) -> bool {
    event.kind == Kind::Travel && !shown(event) && !tables.is_empty()
}

/// The table a journey's checks start on. One table: that one. Several: the table of the
/// journey with checks that is latest on the Timeline, while that table is there. Else no
/// table, and the DM chooses. Never a key's name or a file's order.
pub fn first_table(events: &[Event], id: &str, tables: &[Table]) -> String {
    if let [table] = tables {
        return table.key.clone();
    }
    events
        .iter()
        .rev()
        .filter(|event| event.id != id && event.kind == Kind::Travel)
        .find_map(|event| event.checks_table.as_ref().filter(|key| !key.is_empty()))
        .filter(|key| tables.iter().any(|table| table.key == **key))
        .cloned()
        .unwrap_or_default()
}

/// The chosen table, when the campaign still has it.
fn chosen<'a>(event: &Event, tables: &'a [Table]) -> Option<&'a Table> {
    let key = event
        .checks_table
        .as_deref()
        .filter(|key| !key.is_empty())?;
    tables.iter().find(|table| table.key == key)
}

/// The length in minutes, when the clock can read it. Zero is a length.
fn minutes(length: &str) -> Option<u64> {
    read_duration(length.trim())
}

/// Whether a check at this offset is one of the journey's: a whole interval from the start,
/// and before the end.
fn fits(at: u64, every: u64, length: Option<u64>) -> bool {
    at.is_multiple_of(every) && length.map_or(at == 0, |minutes| at * HOUR < minutes)
}

/// The offsets of a journey's checks, in hours: one at each whole interval from the start, and
/// none at the end, so a length of zero has none. An empty length, or one the clock cannot read,
/// has one. Too many is a count.
pub fn offsets(length: &str, every: u64) -> Result<Vec<u64>, usize> {
    let Some(minutes) = minutes(length) else {
        return Ok(vec![0]);
    };
    let count = minutes.div_ceil(every * HOUR);
    if count > MOST_CHECKS as u64 {
        return Err(count as usize);
    }
    Ok((0..count).map(|at| at * every).collect())
}

/// A check's name, read from the clock: `Day 3` for a day, `Day 3, 08:00` for a watch or for
/// hours. A journey before the first day counts its own days from one.
pub fn name(start: Option<u64>, at: u64, per: &Per) -> String {
    let minute = start.unwrap_or(0) + at * HOUR;
    match per {
        Per::Day | Per::NotRead(_) => format!("Day {}", minute / DAY + 1),
        Per::Watch | Per::Hours(_) => clock_words(minute),
    }
}

/// The result of a Timeline act, with its Undo. It lasts while the event is as the act left
/// it, with no time limit; the page drops it when the DM leaves.
#[derive(Clone, Debug, PartialEq)]
pub struct Done {
    pub id: String,
    before: Event,
    after: Event,
    pub words: String,
    pub undo: &'static str,
}

impl Done {
    pub fn live(&self, book: &Book) -> bool {
        book.events.iter().find(|event| event.id == self.id) == Some(&self.after)
    }

    /// Put the event back as it was before the act, while nothing else changed it.
    pub fn undo(&self, book: &mut Book) {
        if let Some(event) = book
            .events
            .iter_mut()
            .find(|event| event.id == self.id)
            .filter(|event| **event == self.after)
        {
            event.clone_from(&self.before);
            book.touch();
            book.save();
        }
    }
}

/// The last result of each journey's act on one page, with its Undo, kept by the event's ID. Each
/// lasts while its event is as the act left it; the page drops them all when the DM leaves it.
#[derive(Default)]
pub struct Results(BTreeMap<String, Done>);

impl Results {
    pub fn keep(&mut self, done: Done) {
        self.0.insert(done.id.clone(), done);
    }

    /// Drop each result whose event changed since its act.
    pub fn follow(&mut self, book: &Book) {
        self.0.retain(|_, done| done.live(book));
    }

    pub fn get(&self, id: &str) -> Option<&Done> {
        self.0.get(id)
    }

    /// Undo this journey's last act.
    pub fn undo(&mut self, id: &str, book: &mut Book) {
        if let Some(done) = self.0.remove(id) {
            done.undo(book);
        }
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

/// Change one journey's checks, and save the file. `Plan` is the session note's act, not this
/// one's. The words come back when the act cannot
/// happen; a result comes back when the act has one to show.
pub fn apply(
    book: &mut Book,
    id: &str,
    act: &Act,
    tables: &[Table],
) -> Result<Option<Done>, String> {
    let first = first_table(&book.events, id, tables);
    let event = book
        .events
        .iter_mut()
        .find(|event| event.id == id)
        .ok_or("This journey is gone from the Timeline.")?;
    let before = event.clone();
    let mut random = Random::seeded(crate::hermetic::new_uuid().as_u128() as u64);
    let mut add = |event: &mut Event, at: u64, typed: Option<i64>| -> Result<(), String> {
        let table = chosen(event, tables).ok_or("Choose a table first.")?;
        let (value, row) = table.roll(typed)?.draw(&mut random)?;
        // The words as the table has them, links and all, so the result can still name them.
        let row = table.linked_words(value).map_or(row, str::to_owned);
        let place = event
            .checks
            .iter()
            .position(|check| check.at.is_some_and(|other| other > at))
            .unwrap_or(event.checks.len());
        event
            .checks
            .insert(place, Check::new(at, format!("{value} · {row}")));
        Ok(())
    };
    let result = match act {
        Act::Start => {
            event.checks_table = Some(first);
            Some((
                "Encounter checks added to this journey.".to_owned(),
                "Undo checks",
            ))
        }
        Act::Choose(Some(key)) => {
            event.checks_table = Some(key.clone());
            None
        }
        Act::Choose(None) if !event.checks.is_empty() => {
            return Err("Clear the results first.".to_owned());
        }
        Act::Choose(None) => {
            event.checks_table = None;
            None
        }
        Act::Roll(offsets) => {
            for at in offsets {
                if let Err(error) = add(event, *at, None) {
                    event.clone_from(&before);
                    return Err(error);
                }
            }
            let rolled = design::count(offsets.len(), "travel check", "travel checks");
            Some((format!("{rolled} rolled."), "Undo roll"))
        }
        Act::Enter(at, value) => {
            add(event, *at, Some(*value))?;
            Some(("1 travel check rolled.".to_owned(), "Undo roll"))
        }
        Act::Clear(label, words) => {
            let at = event
                .checks
                .iter()
                .position(|check| check.label == *label && check.words == *words)
                .ok_or("That check changed on disk.")?;
            event.checks.remove(at);
            Some(("Check cleared.".to_owned(), "Undo clear"))
        }
        Act::Plan(_) => return Ok(None),
    };
    let after = event.clone();
    if after == before {
        return Ok(None);
    }
    book.touch();
    book.save();
    Ok(result.map(|(words, undo)| Done {
        id: id.to_owned(),
        before,
        after,
        words,
        undo,
    }))
}

/// What one open block needs from its page.
pub struct View<'a> {
    pub event: &'a Event,
    /// The clock's minute at the journey's start, when a day sets it.
    pub start: Option<u64>,
    /// The length as the DM typed it, saved or not.
    pub length: &'a str,
    pub tables: &'a [Table],
    /// For a name that a result links: nothing when it is not an encounter or a group, else
    /// whether the plan has a row still to come for it.
    pub planned: &'a dyn Fn(&str) -> Option<bool>,
}

/// What the DM typed in one block: a roll for each line, and the last line's fault.
#[derive(Default)]
pub struct State {
    event: String,
    typed: BTreeMap<u64, String>,
    problem: Option<(Option<u64>, String)>,
}

impl State {
    /// Keep what was typed for this journey only.
    fn follow(&mut self, id: &str) {
        if self.event != id {
            *self = Self {
                event: id.to_owned(),
                ..Default::default()
            };
        }
    }

    /// An act that could not happen says why on its line.
    pub fn failed(&mut self, act: &Act, error: String) {
        let at = match act {
            Act::Enter(at, _) => Some(*at),
            Act::Roll(offsets) if offsets.len() == 1 => Some(offsets[0]),
            _ => None,
        };
        self.problem = Some((at, error));
    }

    pub fn clear_problem(&mut self) {
        self.problem = None;
    }
}

/// The block's field: the label `Encounter checks` above a line of the table menu and the dice
/// with the interval.
pub fn header(ui: &mut egui::Ui, view: &View<'_>, state: &mut State) -> Option<Act> {
    state.follow(&view.event.id);
    let c = colors();
    let table = chosen(view.event, view.tables);
    let mut act = None;
    design::field(ui, "Encounter checks", None, None, |ui| {
        ui.horizontal_wrapped(|ui| table_line(ui, view, table, &mut act))
    });
    if let Some(fault) = table.and_then(Table::fault) {
        design::status(
            ui,
            Status {
                icon: icons::WARN,
                word: "Cannot roll",
                color: c.document.external,
            },
        )
        .on_hover_text(fault);
    } else if let Some(warning) = table.and_then(|table| table.per.warning()) {
        design::small_message(ui, Tone::Warning, &warning);
    }
    act
}

/// The table menu, then `· 1d20 each day`. Dice words that move to a line of their own start
/// with the words, not with `·`.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn table_line(ui: &mut egui::Ui, view: &View<'_>, table: Option<&Table>, act: &mut Option<Act>) {
    let c = colors();
    ui.spacing_mut().item_spacing.x = design::S;
    let word = match (table, view.event.checks_table.as_deref()) {
        (Some(table), _) => table.name(),
        (None, Some(key)) if !key.is_empty() => "Table not found",
        _ => "Choose a table",
    };
    let button = design::menu_button(ui, word, false);
    egui::Popup::menu(&button).show(|ui| {
        let mut names: Vec<&Table> = view.tables.iter().collect();
        names.sort_by(|a, b| a.name().cmp(b.name()).then(a.key.cmp(&b.key)));
        for option in names {
            let selected = table.is_some_and(|table| table.key == option.key);
            if design::choice(ui, selected, option.name()).clicked() {
                *act = Some(Act::Choose(Some(option.key.clone())));
                ui.close();
            }
        }
        ui.separator();
        let results = !view.event.checks.is_empty();
        if Action::quiet("No checks")
            .blocked_if(results, "Clear the results first.")
            .show(ui)
            .clicked()
        {
            *act = Some(Act::Choose(None));
            ui.close();
        }
        if results {
            design::blocked_reason(ui, Some("Clear the results first."));
        }
    });
    let Some(table) = table else {
        return;
    };
    let words = format!("{} {}", table.dice, table.per.words());
    let after = format!("· {words}");
    let fits = design::number_width(ui, &after, &Type::Small.font(ui))
        <= ui.available_size_before_wrap().x;
    if !fits {
        ui.end_row();
    }
    ui.add(
        egui::Label::new(
            Type::Small
                .rich(ui, if fits { after } else { words })
                .color(c.muted),
        )
        .wrap_mode(egui::TextWrapMode::Extend),
    );
}

/// The span of the journey on the clock, for the line under **How long**.
pub fn span(start: Option<u64>, length: &str) -> Option<String> {
    let start = start?;
    let end = start + minutes(length).filter(|minutes| *minutes > 0)?;
    Some(format!("{} to {}", clock_words(start), clock_words(end)))
}

/// One line for each check, then `Roll all`, then results that are no longer on a line.
pub fn lines(ui: &mut egui::Ui, view: &View<'_>, state: &mut State) -> Option<Act> {
    state.follow(&view.event.id);
    let c = colors();
    let table = chosen(view.event, view.tables);
    let per = table.map_or(Per::Day, |table| table.per.clone());
    let every = per.hours();
    let length = minutes(view.length);
    let blocked = match table {
        None => Some("Choose a table first."),
        Some(table) if table.problem.is_some() => Some("Cannot roll"),
        Some(_) => None,
    };
    let named = |at: u64| {
        if length.is_none() && at == 0 {
            "This journey".to_owned()
        } else {
            name(view.start, at, &per)
        }
    };
    let schedule = offsets(view.length, every);
    let listed = schedule.as_deref().unwrap_or_default();
    let placed = |check: &Check| {
        check.at.is_some_and(|at| match &schedule {
            Ok(listed) => listed.contains(&at),
            Err(_) => fits(at, every, length),
        })
    };
    // Each result on its line, or a line to roll, in the journey's order.
    let mut rows: Vec<(String, Option<&Check>, u64)> = Vec::new();
    for at in listed {
        let results: Vec<&Check> = view
            .event
            .checks
            .iter()
            .filter(|check| check.at == Some(*at))
            .collect();
        if results.is_empty() {
            rows.push((named(*at), None, *at));
        }
        rows.extend(
            results
                .into_iter()
                .map(|check| (named(*at), Some(check), *at)),
        );
    }
    if schedule.is_err() {
        let mut far: Vec<&Check> = view.event.checks.iter().filter(|c| placed(c)).collect();
        far.sort_by_key(|check| check.at);
        rows.extend(far.into_iter().map(|check| {
            let at = check.at.unwrap_or_default();
            (named(at), Some(check), at)
        }));
    }
    let outside: Vec<(String, &Check)> = view
        .event
        .checks
        .iter()
        .filter(|check| !placed(check))
        .map(|check| {
            let label = check
                .at
                .map_or_else(|| check.label.clone(), |at| name(view.start, at, &per));
            (label, check)
        })
        .collect();
    let font = Type::Small.font(ui);
    let column = rows
        .iter()
        .map(|(label, _, _)| label)
        .chain(outside.iter().map(|(label, _)| label))
        .map(|label| design::number_width(ui, label, &font))
        .fold(0.0, f32::max)
        + design::S;
    let mut act = None;
    let mut line = |ui: &mut egui::Ui, label: &str, check: Option<&Check>, at: Option<u64>| {
        let mut act = None;
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = design::S;
            ui.allocate_ui_with_layout(
                egui::vec2(column, Density::Read.control()),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.set_min_size(egui::vec2(column, Density::Read.control()));
                    ui.label(Type::Small.rich(ui, label).color(c.muted));
                },
            );
            match check {
                Some(check) => {
                    if let Some(found) = result(ui, label, check, view) {
                        act = Some(found);
                    }
                }
                None => {
                    let at = at.unwrap_or_default();
                    if Action::secondary("Roll")
                        .icon(icons::DICE)
                        .name(&format!("Roll for {label}"))
                        .density(Density::Read)
                        .blocked(blocked)
                        .show(ui)
                        .clicked()
                    {
                        act = Some(Act::Roll(vec![at]));
                    }
                    let typed = state.typed.entry(at).or_default();
                    let field = ui
                        .add_enabled_ui(blocked.is_none(), |ui| {
                            design::number_field(
                                ui,
                                &format!("Enter a roll for {label}"),
                                typed,
                                "Enter a roll",
                                design::XXL * 3.0,
                                Density::Read.control(),
                            )
                        })
                        .inner;
                    if field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        match typed.trim().parse::<i64>() {
                            Ok(value) => act = Some(Act::Enter(at, value)),
                            Err(_) if typed.trim().is_empty() => {}
                            Err(_) => {
                                state.problem = Some((Some(at), "Type a whole number.".into()));
                            }
                        }
                        typed.clear();
                    }
                }
            }
        });
        if let Some((Some(place), problem)) = &state.problem
            && check.is_none()
            && at == Some(*place)
        {
            design::small_message(ui, Tone::Warning, problem);
        }
        act
    };
    for (label, check, at) in &rows {
        act = line(ui, label, *check, Some(*at)).or(act);
    }
    if table.is_none() {
        design::blocked_reason(ui, blocked);
    }
    if let Err(count) = schedule {
        design::small_message(
            ui,
            Tone::Info,
            &format!(
                "This journey has {count} checks. Split it into shorter journeys to roll them here."
            ),
        );
    }
    let waiting: Vec<u64> = rows
        .iter()
        .filter(|(_, check, _)| check.is_none())
        .map(|(_, _, at)| *at)
        .collect();
    if waiting.len() > 1 {
        design::gap(ui, design::Gap::XS);
        if Action::secondary(&format!("Roll all {}", waiting.len()))
            .icon(icons::DICE)
            .density(Density::Read)
            .blocked(blocked)
            .show(ui)
            .clicked()
        {
            act = Some(Act::Roll(waiting));
        }
    }
    if !outside.is_empty() {
        design::gap(ui, design::Gap::XS);
        ui.label(
            Type::Small
                .rich(ui, "Outside this journey now")
                .color(c.muted),
        );
        for (label, check) in &outside {
            act = line(ui, label, Some(check), None).or(act);
        }
    }
    if let Some((None, problem)) = &state.problem {
        design::small_message(ui, Tone::Warning, problem);
    }
    act
}

/// A result: its number and words, `Clear` at the line's end, and an offer for each encounter
/// or group that it links.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn result(ui: &mut egui::Ui, label: &str, check: &Check, view: &View<'_>) -> Option<Act> {
    let planned = view.planned;
    let c = colors();
    let mut act = None;
    let clear_name = format!("Clear the check for {label}");
    let clear = Action::quiet("Clear")
        .name(&clear_name)
        .density(Density::Read);
    let width = (ui.available_width() - clear.natural_width(ui) - design::S).max(0.0);
    ui.allocate_ui_with_layout(
        egui::vec2(width, 0.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_width(width);
            ui.add_space((Density::Read.control() - Type::Body.size()) / 2.0 - design::XS);
            ui.add(
                egui::Label::new(
                    Type::Body
                        .rich(ui, crate::sessions::plain_links(&check.words))
                        .color(c.text),
                )
                .wrap(),
            );
            let mut names = crate::sessions::linked_names(&check.words);
            names.retain(|name| planned(name).is_some());
            // Each name keeps one place, one control high: its offer, then `In the plan`. The
            // offer's result and its Undo are in the plan's message footer (round 27 FE7).
            for name in names.into_iter().take(MOST_OFFERS) {
                let size = egui::vec2(ui.available_width(), Density::Read.control());
                let top = egui::Layout::top_down(egui::Align::Min);
                ui.allocate_ui_with_layout(size, top, |ui| {
                    ui.set_min_size(size);
                    if planned(&name) == Some(true) {
                        let line = egui::vec2(ui.available_width(), Density::Read.control());
                        let left = egui::Layout::left_to_right(egui::Align::Center);
                        ui.allocate_ui_with_layout(line, left, |ui| {
                            design::status(
                                ui,
                                Status {
                                    icon: icons::OK,
                                    word: "In the plan",
                                    color: c.document.saved,
                                },
                            );
                        });
                    } else if Action::quiet(&format!("Plan “{name}”"))
                        .line_start()
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                    {
                        act = Some(Act::Plan(Planned {
                            name: name.clone(),
                            check: (check.label.clone(), check.words.clone()),
                        }));
                    }
                });
            }
        },
    );
    if clear.show(ui).clicked() {
        act = Some(Act::Clear(check.label.clone(), check.words.clone()));
    }
    act
}

/// **How long** while it has focus: the event, the words typed, and the length the event had
/// when the typing began.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LengthDraft(Option<(String, String, String)>);

/// What a length's field did to its event this frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Taken {
    /// Nothing reached the event.
    Nothing,
    /// The event took the typed length.
    Length,
    /// The event took the typed length, but the file had changed its length meanwhile: saving
    /// waits for the DM's choice, as for any edit made on both sides.
    OverDisk,
}

impl LengthDraft {
    /// The words in view for this event: the typed ones while it has a draft.
    pub fn words(&self, event: &Event) -> String {
        match &self.0 {
            Some((id, words, _)) if *id == event.id => words.clone(),
            _ => event.duration.clone(),
        }
    }

    /// The event takes the typed words.
    fn take(&mut self, event: &mut Event) -> Taken {
        let Some((_, words, base)) = self.0.take() else {
            return Taken::Nothing;
        };
        if words == event.duration {
            return Taken::Nothing;
        }
        let outside = event.duration != base;
        event.duration = words;
        if outside {
            Taken::OverDisk
        } else {
            Taken::Length
        }
    }
}

/// Note what a length's field did: a new length is an edit; one typed over a newer length
/// from the file waits for the DM's choice.
pub fn note_taken(book: &mut Book, taken: Taken) {
    match taken {
        Taken::Nothing => {}
        Taken::Length => book.touch(),
        Taken::OverDisk => book.edited_over_disk(),
    }
}

/// **How long**, the Timeline's field. The file takes the words on Enter or when the field
/// loses focus; until then the page shows what was typed. Returns the words in view, and what
/// reached the event.
pub fn length_input(
    ui: &mut egui::Ui,
    draft: &mut LengthDraft,
    event: &mut Event,
    hint: &str,
    focus: bool,
) -> (String, Taken) {
    let mut words = draft.words(event);
    let width = ui.available_width();
    let response = design::line_field(ui, length_field(event), "How long", &mut words, hint, width);
    if focus {
        response.request_focus();
    }
    let ours = draft.0.as_ref().is_some_and(|(id, _, _)| *id == event.id);
    if response.has_focus() {
        let base = match &draft.0 {
            Some((id, _, base)) if *id == event.id => base.clone(),
            _ => event.duration.clone(),
        };
        draft.0 = Some((event.id.clone(), words.clone(), base));
        (words, Taken::Nothing)
    } else if ours {
        // Enter, or focus gone elsewhere.
        (words, draft.take(event))
    } else {
        (words, Taken::Nothing)
    }
}

/// A length typed for a journey whose held field is no longer in view: its event takes it, as
/// the field would have when it lost focus. `open` is the place in the list of the event whose
/// form is in view, as the page resolved it; a page that goes out of view settles with none.
/// Only a journey that shows its checks holds a typed length, so a draft for an event without
/// them settles too. A frame that keeps the draft reads that one event, never the list.
pub fn settle_length(book: &mut Book, open: Option<usize>) {
    let held = open
        .and_then(|at| book.events.get(at))
        .filter(|event| shown(event))
        .map(|event| event.id.as_str());
    let Some(id) = book
        .length
        .0
        .as_ref()
        .map(|(id, _, _)| id.as_str())
        .filter(|id| Some(*id) != held)
        .map(str::to_owned)
    else {
        return;
    };
    let taken = match book.events.iter_mut().find(|event| event.id == id) {
        Some(event) => book.length.take(event),
        None => {
            book.length.0 = None;
            Taken::Nothing
        }
    };
    note_taken(book, taken);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::roll_table::RollRow;

    /// F9, W5-D1: the plan line's mark names the journey and the check as read. It places the
    /// fight only while that journey of this session still covers that check, unchanged; else
    /// it says why. The one reader keeps the line's words before and after the mark.
    #[test]
    fn w5_d1_a_road_mark_places_the_fight_only_at_the_check_it_read() {
        let session = Some(uuid::Uuid::from_u128(1));
        let mut road = journey("t-road", Some("road"));
        road.session = session;
        road.duration = "2 days".into();
        road.checks = vec![Check::new(
            24,
            "16 · [[Wolf Pack]] wait near mile 53628.".into(),
        )];
        let mark = RoadMark::new("t-road", &road.checks[0]).unwrap();
        let line = format!("- [[Wolf Pack]] {}", mark.comment());
        let read = marked(&line);
        assert_eq!(
            (read.words.as_str(), read.mark),
            ("- [[Wolf Pack]] ", Some(mark.clone()))
        );
        // Review of PR 284, finding 1: the DM's words after the mark are the line's words.
        let read = marked(&format!("{line} Bring silver weapons."));
        assert_eq!(read.words, "- [[Wolf Pack]]  Bring silver weapons.");
        assert_eq!(read.mark, Some(mark.clone()));
        // Half a mark is no mark: the line's words as they are.
        let half = "- [[Wolf Pack]] <!-- splat:check journey=t-road at=24";
        assert_eq!(
            (marked(half).words.as_str(), marked(half).mark),
            (half, None)
        );
        assert_eq!(marked("- [[Wolf Pack]]").mark, None);
        let unread = marked("- [[Wolf Pack]] <!-- splat:check at=x -->");
        assert_eq!(
            (unread.words.as_str(), unread.mark),
            ("- [[Wolf Pack]] ", None),
            "a whole mark that cannot be read is no mark, and no reader shows it"
        );
        let within = super::super::Within {
            journey: "t-road".into(),
            at: 24,
        };
        let day = event_with(Kind::Day);
        let events = |road: &Event| vec![day.clone(), road.clone()];
        let starts = |road: &Event| super::super::clock(&events(road));
        assert_eq!(mark.place(&events(&road), session), Ok(within));
        assert_eq!(
            mark.day_words(&events(&road), &starts(&road), session)
                .as_deref(),
            Some("Day 2 of Travel")
        );
        // Rolled again, or edited in Obsidian: other words. Finding 4: these two words share
        // the first eight digits of their sum.
        let mut changed = road.clone();
        changed.checks[0].words = "16 · [[Wolf Pack]] wait near mile 92050.".into();
        // Cleared.
        let mut cleared = road.clone();
        cleared.checks.clear();
        // Shortened, so the check is outside the journey now.
        let mut short = road.clone();
        short.duration = "1 day".into();
        // Finding 4: a check at offset zero of a journey made zero long.
        let mut zero = road.clone();
        zero.duration = "0 hours".into();
        zero.checks = vec![Check::new(0, "16 · [[Wolf Pack]] wait.".into())];
        let zero_mark = RoadMark::new("t-road", &zero.checks[0]).unwrap();
        assert!(!covers(&zero, 0) && covers(&road, 0) && !covers(&road, 48));
        let mut unknown = zero.clone();
        unknown.duration = String::new();
        assert!(
            covers(&unknown, 0),
            "the one check of a journey of no known length"
        );
        assert!(zero_mark.place(&events(&unknown), session).is_ok());
        // Another session's journey: a fight carried over is played tonight.
        let mut other = road.clone();
        other.session = Some(uuid::Uuid::from_u128(2));
        for (case, mark, road, why) in [
            ("rolled again", &mark, changed, OffRoad::Changed),
            ("cleared", &mark, cleared, OffRoad::Changed),
            ("shortened", &mark, short, OffRoad::Changed),
            ("zero long", &zero_mark, zero, OffRoad::Changed),
            ("another session", &mark, other, OffRoad::OtherSession),
        ] {
            assert_eq!(mark.place(&events(&road), session), Err(why), "{case}");
            assert_eq!(
                mark.day_words(&events(&road), &starts(&road), session),
                None,
                "{case}"
            );
        }
        assert_eq!(
            mark.place(&[day], session),
            Err(OffRoad::Changed),
            "the journey is gone"
        );
    }

    fn event_with(kind: Kind) -> Event {
        Event {
            kind,
            ..journey("t-day", None)
        }
    }

    fn table(key: &str, heading: &str, per: Per) -> Table {
        Table {
            key: key.into(),
            dice: "1d20".into(),
            path: "Road.md".into(),
            heading: Some(heading.into()),
            per,
            problem: None,
            rows: vec![
                RollRow {
                    min: 1,
                    max: 14,
                    text: "Nothing stirs.".into(),
                },
                RollRow {
                    min: 15,
                    max: 20,
                    text: "Wolf Pack pick up the trail.".into(),
                },
            ],
            markup: vec![
                "Nothing stirs.".into(),
                "[[Wolf Pack]] pick up the trail.".into(),
            ],
        }
    }

    fn journey(id: &str, key: Option<&str>) -> Event {
        Event {
            id: id.into(),
            kind: Kind::Travel,
            title: String::new(),
            duration: String::new(),
            place: String::new(),
            notes: String::new(),
            result: String::new(),
            session: None,
            run: None,
            marker: String::new(),
            restores: false,
            continued_in: None,
            played_order: 0,
            checks_table: key.map(str::to_owned),
            checks: Vec::new(),
            within: None,
            extra: Default::default(),
        }
    }

    #[test]
    fn t11_9_one_check_at_each_whole_interval_and_none_at_the_end() {
        assert_eq!(offsets("2 days", 24), Ok(vec![0, 24]));
        assert_eq!(offsets("2 days", 8), Ok(vec![0, 8, 16, 24, 32, 40]));
        assert_eq!(offsets("36 hours", 24), Ok(vec![0, 24]));
        for length in ["", "a while"] {
            assert_eq!(offsets(length, 24), Ok(vec![0]), "{length}");
        }
        // Shorter than one interval: the check at its start. Zero: none, as none is at the end.
        assert_eq!(offsets("1 hour", 24), Ok(vec![0]));
        assert_eq!(offsets("0 days", 24), Ok(vec![]));
        assert_eq!(offsets("10 days", 1), Err(240));
        assert_eq!(name(Some(2 * DAY + 8 * HOUR), 24, &Per::Day), "Day 4");
        assert_eq!(
            name(Some(2 * DAY + 8 * HOUR), 8, &Per::Watch),
            "Day 3, 16:00"
        );
        assert_eq!(name(None, 24, &Per::Day), "Day 2");
        assert_eq!(
            span(Some(2 * DAY + 8 * HOUR), "2 days").as_deref(),
            Some("Day 3, 08:00 to Day 5, 08:00")
        );
    }

    #[test]
    fn t11_9_a_length_typed_over_a_newer_one_from_disk_waits_for_a_choice() {
        let dir = tempfile::tempdir().unwrap();
        let mut book = Book::default();
        book.open(Some(dir.path()));
        let id = book.add(Kind::Travel, None);
        book.save();
        let draft = LengthDraft(Some((id.clone(), "2 days".into(), String::new())));
        // Typed against the length the event had: an edit.
        let taken = draft.clone().take(&mut book.events[0].clone());
        book.length = draft;
        assert_eq!(taken, Taken::Length);
        // The file gave the event a length meanwhile: the typed one does not overwrite it
        // silently, and saving waits for the DM's choice.
        book.events[0].duration = "3 days".into();
        settle_length(&mut book, None);
        assert_eq!(book.length, LengthDraft::default());
        assert!(book.conflict && book.events[0].duration == "2 days");
        // With its form open, a journey that shows its checks holds the typed length. One whose
        // checks went (an edit in Obsidian) holds none: its field changes the event as typed.
        let typed = LengthDraft(Some((id.clone(), "4 days".into(), "2 days".into())));
        book.events[0].checks_table = Some("road".into());
        book.length = typed.clone();
        settle_length(&mut book, Some(0));
        assert_eq!(book.length, typed);
        book.events[0].checks_table = None;
        settle_length(&mut book, Some(0));
        assert_eq!(book.length, LengthDraft::default());
        assert_eq!(book.events[0].duration, "4 days");
    }

    /// V4, V22: the label is above the table menu; the dice words share the menu's line after
    /// `·`, and a line of their own starts with the words.
    #[test]
    fn t11_9_the_dice_words_never_start_a_line_with_the_separator() {
        use egui_kittest::{Harness, kittest::Queryable};
        let tables = [table("road", "On the road", Per::Day)];
        let event = journey("journey", Some("road"));
        for (width, words) in [(400.0, "· 1d20 each day"), (160.0, "1d20 each day")] {
            let mut harness = Harness::builder()
                .with_size(egui::vec2(480.0, 200.0))
                .build_ui(|ui| {
                    ui.set_max_width(width);
                    let view = View {
                        event: &event,
                        start: None,
                        length: "",
                        tables: &tables,
                        planned: &|_| None,
                    };
                    header(ui, &view, &mut State::default());
                });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            let label = harness.get_by_label("Encounter checks").rect();
            let menu = harness.get_by_label("On the road").rect();
            let dice = harness.get_by_label(words).rect();
            assert!(label.bottom() <= menu.top(), "{width}: {label:?} {menu:?}");
            if width > 200.0 {
                assert!(
                    menu.y_range().contains(dice.center().y),
                    "{dice:?} {menu:?}"
                );
            } else {
                assert!(dice.top() >= menu.bottom() && (dice.left() - menu.left()).abs() < 1.0);
            }
        }
    }

    #[test]
    fn t11_10_the_first_table_follows_the_four_cases_never_a_key_or_file_order() {
        let road = table("road", "On the road", Per::Day);
        let sea = table("sea", "At sea", Per::Watch);
        let alone = [road.clone()];
        let both = [road.clone(), sea.clone()];
        let earlier = [
            journey("a", Some("sea")),
            journey("b", Some("road")),
            journey("c", Some("")),
            journey("this", None),
        ];
        // One marked table: that one, whatever came before.
        assert_eq!(first_table(&earlier, "this", &alone), "road");
        // Several: the latest journey on the Timeline with a chosen table.
        assert_eq!(first_table(&earlier, "this", &both), "road");
        let mut moved = earlier.clone();
        moved.swap(0, 1);
        assert_eq!(first_table(&moved, "this", &both), "sea");
        // Several and no earlier choice, or an earlier choice whose table is gone: none.
        assert_eq!(first_table(&earlier[2..], "this", &both), "");
        assert_eq!(
            first_table(&earlier, "this", &[sea, table("x", "X", Per::Day)]),
            ""
        );
    }

    #[test]
    fn t11_9_acts_keep_results_through_a_new_length_and_undo_their_own_step() {
        let dir = tempfile::tempdir().unwrap();
        let mut book = Book::default();
        book.open(Some(dir.path()));
        let id = book.add(Kind::Travel, None);
        let other = book.add(Kind::Travel, None);
        let tables = [table("road", "On the road", Per::Day)];
        let started = apply(&mut book, &id, &Act::Start, &tables)
            .unwrap()
            .unwrap();
        assert_eq!(started.words, "Encounter checks added to this journey.");
        assert_eq!(book.events[0].checks_table.as_deref(), Some("road"));
        crate::hermetic::closed::repeatable_uuids(5);
        let rolled = apply(&mut book, &id, &Act::Roll(vec![0, 24]), &tables)
            .unwrap()
            .unwrap();
        assert_eq!(rolled.words, "2 travel checks rolled.");
        assert!(!started.live(&book), "a later act ends the earlier Undo");
        let saved = std::fs::read_to_string(dir.path().join(super::super::FILE)).unwrap();
        assert!(saved.contains("  - Check at 24h: \""), "{saved}");
        // A typed roll that no row has says so, and changes nothing.
        let typed = apply(&mut book, &id, &Act::Enter(48, 25), &tables);
        assert_eq!(typed, Err("1d20 rolled 25, and no row has it.".to_owned()));
        let entered = apply(&mut book, &id, &Act::Enter(48, 16), &tables)
            .unwrap()
            .unwrap();
        assert_eq!(
            book.events[0].checks[2].words,
            "16 · [[Wolf Pack]] pick up the trail."
        );
        // An unrelated act B on another journey, then Undo of A: only A's step goes.
        apply(&mut book, &other, &Act::Start, &tables).unwrap();
        assert!(entered.live(&book));
        entered.undo(&mut book);
        assert_eq!(book.events[0].checks.len(), 2);
        assert_eq!(book.events[1].checks_table.as_deref(), Some("road"));
        // A shorter journey deletes no result, also through the file; the result past its end
        // is no longer one of its lines. `No checks` waits until the results are cleared.
        book.events[0].duration = "1 day".into();
        book.touch();
        book.save();
        book.reload();
        assert_eq!(
            (
                book.events[0].duration.as_str(),
                book.events[0].checks.len()
            ),
            ("1 day", 2)
        );
        assert_eq!(offsets("1 day", 24), Ok(vec![0]));
        let refused = apply(&mut book, &id, &Act::Choose(None), &tables);
        assert_eq!(refused, Err("Clear the results first.".to_owned()));
        for check in book.events[0].checks.clone() {
            let cleared = apply(
                &mut book,
                &id,
                &Act::Clear(check.label, check.words),
                &tables,
            )
            .unwrap()
            .unwrap();
            assert_eq!(
                (cleared.words.as_str(), cleared.undo),
                ("Check cleared.", "Undo clear")
            );
        }
        apply(&mut book, &id, &Act::Choose(None), &tables).unwrap();
        assert!(!shown(&book.events[0]));
        assert!(offered(&book.events[0], &tables) && !offered(&book.events[0], &[]));
    }
}
