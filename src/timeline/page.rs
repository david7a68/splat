//! The Timeline page: the whole campaign in story order, apart from any one session.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{Book, Event, EventDisplay, Kind, checks, clock_now, clock_words, day_numbers};
use crate::design::{
    self, Status, colors, icons,
    outcome::{
        Choice, Fault, FaultMessage, FileLine, KeptOutcome, MessageChoice, Messages, Notice,
        PaneFrame, Said, Sentence, Severity, UndoStep, Verb,
    },
};
use crate::state_context::StateContext;
use std::time::{Duration, Instant};
use uuid::Uuid;

const UNDO_ADD_FOR: Duration = Duration::from_secs(30);

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Filter {
    #[default]
    All,
    Days,
    Encounters,
    Rests,
    Travel,
    Events,
}

/// The filter tabs, in their order, with their words.
pub const FILTERS: [(Filter, &str); 6] = [
    (Filter::All, "All"),
    (Filter::Days, "Days"),
    (Filter::Encounters, "Fights"),
    (Filter::Rests, "Rests"),
    (Filter::Travel, "Travel"),
    (Filter::Events, "Major events"),
];

impl Filter {
    fn includes(self, kind: Kind) -> bool {
        match self {
            Self::All => true,
            Self::Days => kind == Kind::Day,
            Self::Encounters => kind == Kind::Encounter,
            Self::Rests => matches!(kind, Kind::ShortRest | Kind::LongRest),
            Self::Travel => kind == Kind::Travel,
            Self::Events => kind == Kind::Event,
        }
    }
}

/// What the list and the header show, worked out from the events, the file's bytes and the
/// filters: the days the file wrote with another number, each event's day, the rows that pass
/// the filters, each road fight's day on its journey (round 27 FE10), and the clock now. Made
/// once per change of the book's revision or of the filters (`TimelinePage::view`).
pub struct ListView {
    pub flagged: std::collections::HashMap<String, u32>,
    pub days: Vec<u32>,
    pub rows: Vec<usize>,
    pub road: std::collections::HashMap<String, String>,
    pub now: Option<u64>,
}

/// The filters as one value with a revision, so the list view's `Memo` can key by them.
pub struct FilterKey(design::memo::Revised<(Filter, String, Option<Uuid>)>);

impl Default for FilterKey {
    fn default() -> Self {
        Self(design::memo::Revised::new(Default::default()))
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many times this thread worked out a list view: the test counts the work itself.
    static VIEWS_MADE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

impl ListView {
    fn make(book: &Book, (filter, search, session): &(Filter, String, Option<Uuid>)) -> Self {
        #[cfg(any(test, feature = "test-support"))]
        VIEWS_MADE.with(|made| made.set(made.get() + 1));
        let days = day_numbers(&book.events);
        let rows = (0..book.events.len())
            .filter(|&index| matches(*filter, search, *session, &book.events[index], days[index]))
            .collect();
        Self {
            flagged: book
                .baseline()
                .and_then(|bytes| std::str::from_utf8(bytes).ok())
                .map(super::formats::flagged_days)
                .unwrap_or_default()
                .into_iter()
                .collect(),
            days,
            rows,
            road: super::road_days(&book.events),
            now: clock_now(&book.events),
        }
    }
}

/// Does this event pass the filters?
fn matches(filter: Filter, search: &str, session: Option<Uuid>, event: &Event, day: u32) -> bool {
    if !filter.includes(event.kind) {
        return false;
    }
    if session.is_some() && event.session != session {
        return false;
    }
    let search = search.trim().to_lowercase();
    if search.is_empty() {
        return true;
    }
    [
        event.label(day).as_str(),
        event.place.as_str(),
        event.notes.as_str(),
        event.result.as_str(),
        event.kind.word(),
    ]
    .iter()
    .any(|text| text.to_lowercase().contains(&search))
}

/// Row 123: the campaign's fights and groups by name, which a check result on Timeline can offer
/// to the plan of its journey's session. The app gives them each frame, as it gives the Session
/// page its `Plan`.
/// One copy for each read of the encounters (`revision`); a clone shares it.
#[derive(Clone, Debug, Default)]
pub struct PlanNames {
    /// The read of the campaign's encounters these names come from (`Encounters::plan_revision`).
    pub revision: Option<Uuid>,
    pub fights: std::sync::Arc<[String]>,
    pub groups: std::sync::Arc<[String]>,
}

impl PlanNames {
    pub fn new(revision: Option<Uuid>, fights: Vec<String>, groups: Vec<String>) -> Self {
        #[cfg(any(test, feature = "test-support"))]
        PLAN_NAMES_MADE.with(|made| made.set(made.get() + 1));
        Self {
            revision,
            fights: fights.into(),
            groups: groups.into(),
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many times the plan's names were copied from the encounters (row 123, point 4).
    pub static PLAN_NAMES_MADE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// How many times a session plan was read for Timeline's offers or a plan line's write.
    pub static JOURNEY_PLAN_READS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// How many times a plan line's result was checked against its note's words.
    pub static PLAN_LINE_CHECKS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub use super::SessionLink;

#[derive(Default)]
pub struct TimelinePage {
    /// The filters, with a revision.
    pub filters: FilterKey,
    /// The list view, by the book's revision and the filters' (`ListView`).
    pub view: design::memo::Memo<
        (design::memo::Revision, design::memo::Revision),
        std::sync::Arc<ListView>,
    >,
    pub filter: Filter,
    pub search: String,
    pub session: Option<Uuid>,
    pub selected: Option<String>,
    pub copied: Option<(String, Instant)>,
    pub focus_title: bool,
    /// A modal is open over the app (the app's one test, `modal_open`): Ctrl+S waits for it.
    pub modal: bool,
    /// The open event's form opens at its top in the next draw: another event, a link, the page
    /// opened again. Never a filter or a search, which change only the list.
    pub reset_detail: bool,
    pub scroll_to: Option<usize>,
    /// An event just added with the buttons, with one step of undo.
    pub added: design::Results<super::TimelineFile, JustAdded>,
    /// Its footer line, kept as data across frames (ruling L9): made again only when the store,
    /// the book or the saved-run facts changed.
    pub added_view: design::results::Retained<super::TimelineFile, ListAct, AddedInputs>,
    /// The book's revision at the last check that the Add result's event is still there.
    pub added_seen: Option<design::memo::Revision>,
    /// The list opens where the story is now, at its end.
    pub opened: bool,
    /// One column: the detail hides the list, so the open event's results end its form.
    pub narrow: bool,
    /// The run open now, whose event must stay while it runs.
    pub open_run: Option<Uuid>,
    /// Saved runs from the same campaign snapshot used by the Session plan, and the revision of
    /// their facts (`Encounters::saved_runs_revision`): `set_saved_runs`.
    pub saved_runs: Vec<crate::encounter_run::SavedRunInfo>,
    pub saved_runs_revision: Option<design::memo::Revision>,
    /// The DM followed an open fight's status back to the run.
    pub wants_back_to_run: bool,
    /// Filters and the selection belong to one campaign.
    pub campaign: Option<Uuid>,
    /// The DM asked to open this session's page.
    pub open_session: Option<Uuid>,
    /// The DM chose where new events go.
    pub new_events_to: Option<Option<Uuid>>,
    /// The session new events go to looks like an old one, in words.
    pub dated: Option<String>,
    /// The DM is playing that session after all.
    pub play_current: bool,
    /// The campaign's locations and their other names, to suggest for an event's place.
    pub places: Vec<(Uuid, Vec<String>)>,
    /// The DM asked to read this location.
    pub open_place: Option<Uuid>,
    /// The campaign's marked tables, for a journey's encounter checks.
    pub tables: Option<std::sync::Arc<splat_campaign::loot::Sources>>,
    /// Row 123: the campaign's fights and groups by name, for a check result's plan offer.
    pub plan_names: PlanNames,
    pub focus_length: bool,
    /// Round 29 G142: a roll was made in this journey (its id) at this check (its offset): the
    /// journey's next draw scrolls least so that the check's result and its offer lines are in
    /// view, once. Another selection, another journey's draw or a reopened page drops it.
    pub reveal_checks: Option<(String, u64)>,
    /// What the DM typed in a journey's checks, and each journey's last result with its Undo.
    pub checks: checks::State,
    pub checks_done: checks::Results,
}

/// The Timeline page's Add strip: every kind, fights first.
pub const ADD_KINDS: [Kind; 6] = [
    Kind::Encounter,
    Kind::Day,
    Kind::LongRest,
    Kind::ShortRest,
    Kind::Travel,
    Kind::Event,
];

/// What the DM chose in the Timeline list's message footer, done after the footer is drawn.
#[derive(Clone, Debug, PartialEq)]
pub enum ListAct {
    UndoLinks,
    UpdateLinks,
    UndoRuling,
    UndoRemove,
    UndoAdd,
    UndoPlanLine,
    File(FileAct),
}

pub enum RestRulingAction {
    Set(bool),
}

impl RestRulingAction {
    pub fn apply(self, book: &mut Book, id: &str) {
        match self {
            Self::Set(restores) => book.set_restores(id, restores),
        }
    }
}

/// The Timeline page's add result: the event the buttons added, until the DM opens another
/// event, the event is gone from the book, or `UNDO_ADD_FOR` passes on the frame clock, so it
/// cannot undo the wrong event. The page's owner update (`settle_added`) ends it, before any
/// footer reads it.
pub struct JustAdded {
    file: super::TimelineFile,
    id: String,
}

impl design::results::Kept for JustAdded {
    type Owner = super::TimelineFile;
    const ENDS: design::results::Ends = design::results::Ends {
        // The event's removal here ends it; another selection does too (`select`).
        owner_changed: design::results::OwnerChanged::Ends,
        // A change of the book (a reload, a file event, an edit) that lost its event ends it.
        disk_changed: design::results::DiskChanged::Checked,
        press: design::results::Press::Undo,
    };
    const LIFE: Option<Duration> = Some(UNDO_ADD_FOR);
    fn owner(&self) -> super::TimelineFile {
        self.file.clone()
    }
}

/// The inputs of the Add result's words besides its store: the book's revision (the event's
/// label, its day, its rest ruling) and the saved-run facts' (which fights keep their HP).
type AddedInputs = (design::memo::Revision, design::memo::Revision);

/// A fault of the Timeline: a save that failed, a removal that notes still link to, a flag of
/// the file, or a second timeline file (in the footer). The kind gives the tone.
enum FileFault {
    /// Notes still link to the removed event: the DM checks them.
    Linked(Sentence),
    NotSaved(Sentence),
    /// A line of the file that Splat read its own way (`Line 9: “Day 5” is day 1 in order. …`).
    Flag(Sentence),
    /// Another timeline file in the Timeline's folder.
    Copy,
}

impl Fault for FileFault {
    fn severity(&self) -> Severity {
        match self {
            Self::Linked(_) | Self::Flag(_) | Self::Copy => Severity::NeedsFixing,
            Self::NotSaved(_) => Severity::NotSaved,
        }
    }
    fn cause(&self) -> Sentence {
        match self {
            Self::Linked(words) | Self::NotSaved(words) | Self::Flag(words) => words.clone(),
            Self::Copy => crate::sentence!(
                "Another timeline file exists. Compare its events with Timeline.md before removing either file."
            ),
        }
    }
}

/// Words made elsewhere as text: a file's flag, an error of the system, a result's words that
/// the Timeline builds from the events. The one place this file takes them as a sentence.
#[allow(
    clippy::disallowed_methods,
    reason = "words of a file, of the system or of the events, as text (design::results::STAGED)"
)]
pub fn as_sentence(words: &str) -> Sentence {
    Sentence::outside(words).unwrap_or(crate::sentence!("Something changed."))
}

/// Both pages use the same ruling words, icon and colour.
pub fn long_rest_status(restores: bool) -> Status<'static> {
    Status {
        icon: if restores {
            icons::OK
        } else {
            icons::LONG_REST
        },
        word: if restores {
            "Everyone at full"
        } else {
            "Not at full"
        },
        color: if restores {
            colors().document.saved
        } else {
            colors().muted
        },
    }
}

/// The ruling's result in a pane's typed footer, while its rest is the one open: a finished
/// change, so it has the success tone in both directions. It says the ruling now in force, and
/// what it does to the next fight. Both pages draw it from the book's store.
pub fn ruling_line<'f, A: 'static>(
    frame: &'f PaneFrame,
    messages: &mut Messages<'f, A>,
    book: &Book,
    open: &str,
    undo: A,
) {
    let Some(shown) = book
        .results
        .ruling
        .get(&book.file())
        .filter(|shown| shown.value().rest().id == open && book.rest_undo_id() == Some(open))
    else {
        return;
    };
    messages.kept(shown.outcome(frame, |ruling| {
        let words = if ruling.rest().restores {
            crate::sentence!("Everyone starts the next fight at full.")
        } else {
            crate::sentence!("The next fight starts from the HP the party has now.")
        };
        Said::new(words)
            .undo(UndoStep::of(Verb::Change, "ruling").pressed_as(undo))
            .into()
    }));
}

/// Timeline's removal notice: the plan's words, for the removed event's own session, then how
/// many notes still link to it.
fn timeline_removal_words(book: &Book, label: &str, mentions: usize) -> String {
    let session = book.removed_event().and_then(|event| event.session);
    let days = day_numbers(&book.events);
    let mut words = super::removed_event_words(book, session, &days, label, &[]);
    if mentions > 0 {
        words.push_str(&format!(
            " {} still {} to it.",
            design::count(mentions, "note", "notes"),
            if mentions == 1 { "links" } else { "link" }
        ));
    }
    words
}

/// What a restoring rest does to the next fight. Adding a rest and marking it say the same.
const REST_RESTORES_WORDS: &str = "Everyone starts the next fight at full.";

/// The result of every Add: the event, then the session it went to when the DM added it inside
/// one (`Session 1`). A rest that restores HP then says what that does to the next fight, and
/// which saved fights keep their HP.
pub fn added_message(label: &str, session: Option<&str>, restores: Option<Vec<String>>) -> String {
    let to = session.map_or_else(String::new, |session| format!(" to {session}"));
    let mut words = format!("{label} added{to}.");
    if let Some(saved_fights) = restores {
        words.push_str(&format!(" {REST_RESTORES_WORDS}"));
        for name in saved_fights {
            words.push_str(&format!(" {name} keeps its saved HP."));
        }
    }
    words
}

/// A Timeline flag's words. The file line below the warning holds the path, so the words name
/// only the line.
fn flag_words(flag: &splat::content::layout::Flag) -> String {
    flag.message.clone()
}

/// What the DM chose in a file notice in a pane's footer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileAct {
    UndoReload,
    SaveAgain,
}

/// The reload result's Undo: `Undo reload Timeline.md`, the file by the name the app gives it
/// everywhere (`Splat cannot read Timeline.md`), as World's repair form names an unreadable note
/// (`Undo reload Broken.md`, `Undo discard of Broken.md`); a readable note's Undo names the note
/// (a session note, a World note: M0438). A book with no file gives `Undo reload` (M0548).
fn reload_step(book: &Book) -> UndoStep {
    named_step(
        Verb::Reload,
        book.file()
            .0
            .file_name()
            .map(|name| name.to_string_lossy().into_owned()),
    )
}

/// An Undo that names its object, the one place that falls back to the bare verb when there is
/// none (a book with no file, an added event no longer in the book).
fn named_step(verb: Verb, object: Option<String>) -> UndoStep {
    match object.filter(|object| !object.trim().is_empty()) {
        Some(object) => UndoStep::of(verb, object),
        None => UndoStep::verb(verb),
    }
}

/// The file's notices that block nothing, in a pane's typed footer: the reload result with its
/// Undo (from the book's store), then the failed save with `Try to save again`, the flags and a
/// second timeline file. They arrive from outside edits and autosave, so every pane that shows
/// the Timeline's events (the Timeline page, a session's plan) puts them in its footer. `wrap`
/// gives the pane's own press for each file act.
pub fn file_messages<'f, A: 'static>(
    frame: &'f PaneFrame,
    messages: &mut Messages<'f, A>,
    book: &Book,
    wrap: impl Fn(FileAct) -> A,
) {
    if let Some(shown) = book.results.reloaded.get(&book.file()) {
        let undo = wrap(FileAct::UndoReload);
        messages.kept(shown.outcome(frame, |reloaded| {
            KeptOutcome::Notice(
                Notice::new(crate::sentence!(
                    "Reloaded. Your version, with {}, is gone unless you undo.",
                    design::count(reloaded.count(), "event", "events")
                )),
                reload_step(book).pressed_as(undo),
            )
        }));
    }
    if book.unreadable.is_none()
        && !book.conflict
        && let Some(error) = &book.error
    {
        messages.outcome(
            FaultMessage::new(&FileFault::NotSaved(as_sentence(error))).choice(Choice::new(
                MessageChoice::Choice("Try to save again"),
                wrap(FileAct::SaveAgain),
            )),
        );
    }
    // The flags, one warning each, with the file's line (its path, `Edit in Obsidian` and `Show
    // file`) under the last; then a second timeline file with its own line. They are footer
    // messages, so they stay in view over long content and move no row (PR 340's rule), in the
    // footer's height-limited region, which scrolls when they fill it.
    let folder = book.path().and_then(std::path::Path::parent);
    for (at, flag) in book.flags.iter().enumerate() {
        let fault = FaultMessage::new(&FileFault::Flag(as_sentence(&flag_words(flag))));
        messages.outcome(match folder {
            Some(folder) if at + 1 == book.flags.len() => {
                fault.file(FileLine::new(folder, super::FILE).edit().show())
            }
            _ => fault,
        });
    }
    if let Some(copy) = &book.duplicate {
        let fault = FaultMessage::new(&FileFault::Copy);
        let named = copy.parent().zip(copy.file_name());
        messages.outcome(match named {
            Some((folder, name)) => fault.file(
                FileLine::new(folder, name.to_string_lossy().into_owned())
                    .edit()
                    .show(),
            ),
            None => fault,
        });
    }
}

/// Do what the DM chose in a footer's file notice; an Undo acts on the entry its ticket names.
pub fn file_act(book: &mut Book, act: FileAct, ticket: Option<design::results::Ticket>) {
    match act {
        FileAct::UndoReload => {
            if let Some(ticket) = ticket {
                book.undo_reload(ticket);
            }
        }
        FileAct::SaveAgain => {
            book.error = None;
            book.save();
        }
    }
}

impl TimelinePage {
    /// Show one event, whatever the filters were hiding.
    pub fn reveal(&mut self, id: &str, book: &Book) {
        self.filter = Filter::All;
        self.search.clear();
        self.session = None;
        self.reset_detail = true;
        self.select(Some(id.to_owned()));
        self.scroll_to = book.events.iter().position(|event| event.id == id);
    }

    /// The saved runs and the revision of their facts, from the campaign's encounters.
    pub fn set_saved_runs(
        &mut self,
        runs: Vec<crate::encounter_run::SavedRunInfo>,
        revision: design::memo::Revision,
    ) {
        self.saved_runs = runs;
        self.saved_runs_revision = Some(revision);
    }

    /// Open `selected`: another event than the Add result's ends that result at once, so it
    /// cannot undo the wrong event.
    pub fn select(&mut self, selected: Option<String>) {
        self.selected = selected;
        self.end_add_of_other_selection();
    }

    /// An Add result whose event is not the open one can never be undone again: it ends.
    fn end_add_of_other_selection(&mut self) {
        let selected = &self.selected;
        self.added
            .end_unreachable(|added| selected.as_ref() == Some(&added.id));
    }

    /// The Add result's owner update, before any footer reads it (ruling L9): its life on the
    /// frame clock ends it (`Results::expire`, which asks for a repaint at the end); a change of
    /// the book that lost its event ends it, checked only when the book's revision changed (an
    /// unreadable book keeps the events it had, so its Add stands until its life ends; a book
    /// of another file is another owner, so its Add shows no more); another selection ends it.
    pub fn settle_added(&mut self, ctx: &StateContext, book: &Book) {
        self.added.expire(ctx);
        let revision = book.revision();
        if self.added_seen != Some(revision) {
            self.added_seen = Some(revision);
            if !self.added.is_empty() {
                self.added
                    .disk_changed(|added| book.events.iter().any(|event| event.id == added.id));
            }
        }
        self.end_add_of_other_selection();
    }

    /// The list view for this frame: worked out again only when the book or the filters
    /// changed. The filters' value is compared, not copied, on a frame with no change.
    pub fn view(&mut self, book: &Book) -> std::sync::Arc<ListView> {
        let (filter, search, session) = &*self.filters.0;
        if *filter != self.filter || *search != self.search || *session != self.session {
            self.filters
                .0
                .set((self.filter, self.search.clone(), self.session));
        }
        let key = (book.revision(), self.filters.0.revision());
        let filters = &self.filters;
        self.view
            .get_or(key, || {
                std::sync::Arc::new(ListView::make(book, &filters.0))
            })
            .clone()
    }

    pub fn reopen(&mut self) {
        self.reset_detail = true;
        self.reveal_checks = None;
        // The journeys' results and their Undo end when the DM leaves the page.
        self.checks_done.clear();
    }

    /// A row's words: its title, its second line with each status part's span and tint, the
    /// flag of a day heading written out of order, and its accessible name.
    pub fn row_words(
        &self,
        event: &Event,
        day: u32,
        session: Option<&str>,
        written_as: Option<u32>,
        on_road: Option<&str>,
    ) -> RowWords {
        let title = event.label(day);
        let open = event.run.is_some() && event.run == self.open_run;
        let summary = super::played_fight_summary(event, open);
        let mut detail = event.summary_with_result(session, &summary);
        // Round 27 FE10: a fight on the road says its day of the journey, as its plan row does.
        if let Some(on_road) = on_road {
            if !detail.is_empty() {
                detail.push_str(" · ");
            }
            detail.push_str(on_road);
        }
        // Repeat the day only when a filter or search can hide its heading.
        if event.kind != Kind::Day
            && day > 0
            && (self.filter != Filter::All
                || !self.search.trim().is_empty()
                || self.session.is_some())
        {
            detail = if detail.is_empty() {
                format!("Day {day}")
            } else {
                format!("Day {day} · {detail}")
            };
        }
        // The ruling is a status, in its own colour, as on the plan row and in the detail.
        let mut ruling = None;
        if event.kind == Kind::LongRest {
            if !detail.is_empty() {
                detail.push_str(" · ");
            }
            let status = long_rest_status(event.restores);
            ruling = Some((detail.len(), RowTint::Ruling(event.restores)));
            detail.push_str(&format!("{} {}", status.icon, status.word));
        }
        // Round 26 FG4, FD7: a length the clock cannot count is a warning part, in its colour.
        let mut spans = Vec::new();
        let warning = event.time_warning();
        if let Some(warning) = &warning
            && let Some(start) = detail.find(warning.as_str())
        {
            spans.push((start..start + warning.len(), RowTint::Warning));
        }
        if let Some((start, tint)) = ruling {
            spans.push((start..detail.len(), tint));
        }
        // A day heading written out of order keeps its Day icon and says what it was written as.
        let flag = written_as.map(|day| format!("Written as Day {day}"));
        let mut name = if detail.is_empty() {
            title.clone()
        } else {
            format!("{title} · {detail}")
        };
        if let Some(flag) = &flag {
            name.push_str(&format!(" · {flag}"));
        }
        RowWords {
            title,
            detail,
            spans,
            flag,
            name,
            open,
            summary,
            warning: warning.is_some(),
        }
    }

    /// What the typed place could be: the one location it names (its first name), or up to five
    /// names that contain it. The place stays text.
    pub fn place_help_of(&self, place: &str) -> PlaceHelp {
        let typed = place.trim().to_lowercase();
        if typed.is_empty() {
            return PlaceHelp::None;
        }
        let exact: Vec<_> = self
            .places
            .iter()
            .filter(|(_, names)| names.iter().any(|name| name.to_lowercase() == typed))
            .collect();
        if let [(id, names)] = exact.as_slice() {
            return PlaceHelp::Exact(*id, names[0].clone());
        }
        let close: Vec<String> = self
            .places
            .iter()
            .filter_map(|(_, names)| {
                names
                    .iter()
                    .find(|name| name.to_lowercase().contains(&typed))
            })
            .take(5)
            .cloned()
            .collect();
        if close.is_empty() {
            PlaceHelp::None
        } else {
            PlaceHelp::Close(close)
        }
    }

    /// The header's words after `Timeline`: the count of events (and how many show while a filter
    /// or a search hides some) and the clock, once a day sets it.
    pub fn header_context(&mut self, book: &Book) -> String {
        if book.events.is_empty() {
            return String::new();
        }
        // The clock shows once a day sets it; before that, there is nothing to name.
        let view = self.view(book);
        let now = view
            .now
            .map_or_else(String::new, |now| format!(" · Now: {}", clock_words(now)));
        format!(
            "{}{now}",
            if self.filtered() {
                let shown = view.rows.len();
                format!(
                    "{} of {}",
                    if shown == 0 {
                        "0".into()
                    } else {
                        design::count(shown, "", "")
                    },
                    design::count(book.events.len(), "event", "events")
                )
            } else {
                design::count(book.events.len(), "event", "events")
            },
        )
    }

    /// Whether a filter, a search or a session hides some events.
    pub fn filtered(&self) -> bool {
        self.filter != Filter::All || !self.search.trim().is_empty() || self.session.is_some()
    }

    /// `Show all events`: the filters, the search and the session filter clear.
    pub fn show_all(&mut self) {
        self.search.clear();
        self.filter = Filter::All;
        self.session = None;
    }

    /// An event the DM just added (with the buttons or `Start with day 1`): it opens, its Title
    /// takes the keyboard, and its result keeps its Undo for `UNDO_ADD_FOR`.
    pub fn record_added(&mut self, id: String, book: &Book, ctx: &StateContext) {
        self.reveal(&id, book);
        self.focus_title = true;
        self.added.record_timed(
            JustAdded {
                file: book.file(),
                id,
            },
            ctx,
        );
    }

    /// What the DM chose in the list's message footer, done after the footer is drawn: an Undo
    /// acts on the entry its ticket names.
    pub fn footer_act(
        &mut self,
        press: design::outcome::Press<ListAct>,
        book: &mut Book,
        notes: &mut crate::sessions::page::Sessions,
        offer: Option<crate::sessions::page::LinkOffer>,
    ) {
        match press.id {
            ListAct::UndoLinks => {
                if let Some(ticket) = press.ticket {
                    notes.undo_event_link_update(ticket);
                }
            }
            ListAct::UpdateLinks => {
                if let Some(offer) = offer {
                    notes.accept_event_link_offer(offer, book);
                }
            }
            ListAct::UndoRuling => {
                if let Some(ticket) = press.ticket {
                    book.undo_restores(ticket);
                    self.added.owner_changed(&book.file());
                }
            }
            ListAct::UndoRemove => {
                if let Some(ticket) = press.ticket {
                    book.undo_remove(ticket);
                }
            }
            ListAct::UndoAdd => {
                if let Some(ticket) = press.ticket
                    && let Ok((added, ())) = self.added.undo_by(ticket, |_| Ok::<(), ()>(()))
                {
                    book.undo_add(&added.id);
                    self.select(None);
                }
            }
            ListAct::UndoPlanLine => {
                // Row 123: a refused Undo stays on its entry, which both readers show.
                if let Some(ticket) = press.ticket {
                    notes.undo_plan_line(ticket);
                }
            }
            ListAct::File(act) => file_act(book, act, press.ticket),
        }
    }

    /// The Timeline page's footer messages for this frame (`list_pane`), `width` wide.
    pub fn list_messages<'f>(
        &mut self,
        frame: &'f PaneFrame,
        book: &Book,
        notes: &mut crate::sessions::page::Sessions,
        open: Option<&str>,
        offer: Option<&crate::sessions::page::LinkOffer>,
    ) -> Messages<'f, ListAct> {
        let mut messages = Messages::new();
        if let Some(line) = notes.event_link_line(frame, ListAct::UndoLinks) {
            messages.kept(line);
        }
        // Row 123: the plan line's result, or its refused Undo's answer, from the one store the
        // Session plan reads too.
        for line in notes.plan_line_lines(frame, ListAct::UndoPlanLine) {
            messages.kept(line);
        }
        if let Some(offer) = offer {
            messages.outcome(offer.notice(ListAct::UpdateLinks));
        }
        if let Some(problem) = notes.event_link_problem() {
            messages.outcome(design::outcome::Outcome::Refused(as_sentence(problem)));
        }
        if let Some(id) = open {
            ruling_line(frame, &mut messages, book, id, ListAct::UndoRuling);
        }
        file_messages(frame, &mut messages, book, ListAct::File);
        if let Some(shown) = book.results.removal.get(&book.file()) {
            // The removal keeps its own count, so a campaign switch and back keeps the warning.
            let mentions = shown.value().mentions();
            let label = shown.value().label().to_owned();
            let words = as_sentence(&timeline_removal_words(book, &label, mentions));
            messages.kept(shown.outcome(frame, |_| {
                // M-N1.7: the Undo names the event, as the Session plan's line of it does.
                let undo = UndoStep::of(Verb::Remove, label).pressed_as(ListAct::UndoRemove);
                // A finished change has the success tone; links left to it make it a warning.
                if mentions > 0 {
                    KeptOutcome::Fault(FaultMessage::new(&FileFault::Linked(words)), undo)
                } else {
                    Said::new(words).undo(undo).into()
                }
            }));
        }
        // The Add result: its owner update (`settle_added`) ended it if it is over, so this
        // frame finds its kept line, made again only when the store, the book or the saved-run
        // facts changed, and presents it.
        let file = book.file();
        let inputs = (
            book.revision(),
            self.saved_runs_revision.unwrap_or_else(|| book.revision()),
        );
        let saved_runs = &self.saved_runs;
        let projection = self
            .added_view
            .refresh(&self.added, &file, inputs, |shown| {
                let id = &shown.value().id;
                let at = book.events.iter().position(|event| event.id == *id);
                let label = at.map(|at| book.events[at].label(day_numbers(&book.events)[at]));
                let restores = at
                    .map(|at| &book.events[at])
                    .filter(|event| event.kind == Kind::LongRest && event.restores)
                    .map(|event| {
                        super::fights_that_keep_hp(&book.events, event.session, saved_runs)
                    });
                // Timeline lists every session, so its result names only the event.
                let words = as_sentence(&added_message(
                    label.as_deref().unwrap_or(""),
                    None,
                    restores,
                ));
                // The Undo names the added event, as its words do.
                Said::new(words)
                    .undo(named_step(Verb::Add, label).pressed_as(ListAct::UndoAdd))
                    .into()
            });
        if let Ok(Some(line)) = self.added.present(frame, projection, &file, inputs) {
            messages.kept(line);
        }
        if book.merged_outside {
            messages.outcome(Notice::new(crate::sentence!(
                "Timeline.md also changed outside Splat. Both changes are kept."
            )));
        }
        messages
    }

    /// What the open event's form did in one frame, applied after it drew: the edit, the
    /// length its field gave the event, a journey's checks act, its Undo, the rest ruling, the
    /// new place, and Remove.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw calls that a design part replaces (design::results::STAGED)"
    )]
    pub fn apply_detail(
        &mut self,
        book: &mut Book,
        notes: &mut crate::sessions::page::Sessions,
        sessions: &[SessionLink],
        id: &str,
        acts: DetailActs,
    ) {
        let DetailActs {
            changed,
            removed,
            placed,
            checks: checks_act,
            undo_checks,
            ruling,
            length_taken,
        } = acts;
        let id = id.to_owned();
        let Some(index) = book.events.iter().position(|event| event.id == id) else {
            return;
        };
        let sources = self.tables.clone();
        let tables = sources.as_ref().map_or(&[][..], |sources| &sources.tables);
        if changed {
            book.touch();
        }
        checks::note_taken(book, length_taken);
        if let Some(act) = checks_act {
            self.checks.clear_problem();
            if act == checks::Act::Start {
                self.focus_length = book.events[index].duration.trim().is_empty();
            }
            // Row 123: `Plan “<name>”` writes the plan of the journey's session, by the plan's
            // own act; its result and Undo are in the list's footer.
            if let checks::Act::Plan(chosen) = &act {
                if let Err(words) = notes.plan_from_timeline(book, &id, chosen, &self.plan_names) {
                    self.checks.failed(&act, words);
                }
            } else {
                match checks::apply(book, &id, &act, tables) {
                    Ok(Some(done)) => {
                        self.checks_done.keep(done);
                        self.reveal_checks = match &act {
                            checks::Act::Roll(offsets) => offsets.iter().min().copied(),
                            checks::Act::Enter(at, _) => Some(*at),
                            _ => None,
                        }
                        .map(|at| (id.clone(), at));
                    }
                    Ok(None) => {}
                    Err(error) => self.checks.failed(&act, error),
                }
            }
        }
        if undo_checks {
            self.checks_done.undo(&id, book);
        }
        if let Some(action) = ruling {
            action.apply(book, &id);
            self.added.owner_changed(&book.file());
        }
        if let Some(after) = placed {
            book.place_after(&id, after.as_deref());
            self.scroll_to = book.events.iter().position(|event| event.id == id);
        }
        if removed {
            self.added.owner_changed(&book.file());
            let mentions = sessions
                .iter()
                .filter(|session| session.mentions.contains(&id))
                .count();
            book.remove_linked(&id, mentions);
            self.select(None);
        }
    }
}

/// What the open event's form did in one frame (`TimelinePage::apply_detail`).
pub struct DetailActs {
    /// A field changed the event.
    pub changed: bool,
    /// `Remove` in the header's menu.
    pub removed: bool,
    /// `Place after`: the event it goes after (`None`: the start).
    pub placed: Option<Option<String>>,
    /// An act of a journey's checks.
    pub checks: Option<checks::Act>,
    /// The Undo of the journey's last checks act.
    pub undo_checks: bool,
    /// The rest ruling's action.
    pub ruling: Option<RestRulingAction>,
    /// What **How long** gave the event (a journey with checks takes it on Enter or focus loss).
    pub length_taken: checks::Taken,
}

impl Default for DetailActs {
    fn default() -> Self {
        Self {
            changed: false,
            removed: false,
            placed: None,
            checks: None,
            undo_checks: false,
            ruling: None,
            length_taken: checks::Taken::Nothing,
        }
    }
}

/// Why the open event cannot save now: the file cannot be read, a save failed, or it changed on
/// disk under the draft.
pub fn saving_blocked(book: &Book) -> Option<String> {
    book.unreadable
        .as_deref()
        .or(book.error.as_deref())
        .or(if book.conflict {
            Some("Changed on disk. Choose what to keep above.")
        } else {
            None
        })
        .map(str::to_owned)
}

/// The open event's save state's four words, in the room of the widest: Saved, Saving…,
/// Unsaved and Saving blocked.
pub fn saving_states() -> [Status<'static>; 4] {
    let c = colors();
    [
        Status {
            icon: icons::OK,
            word: "Saved",
            color: c.document.clean,
        },
        Status {
            icon: icons::BUSY,
            word: "Saving…",
            color: c.muted,
        },
        Status {
            icon: icons::WARN,
            word: "Unsaved",
            color: c.document.external,
        },
        Status {
            icon: icons::WARN,
            word: "Saving blocked",
            color: c.document.external,
        },
    ]
}

/// Which of `saving_states` the event `id` is in now.
pub fn saving_now(book: &Book, id: &str) -> usize {
    let unsaved = book.event_unsaved(id);
    if saving_blocked(book).is_some() {
        if unsaved { 2 } else { 3 }
    } else if unsaved {
        1
    } else {
        0
    }
}

/// Each event's name in `Place after`: its label, its day and its session's number.
pub fn event_labels(book: &Book, days: &[u32], sessions: &[SessionLink]) -> Vec<String> {
    book.events
        .iter()
        .zip(days)
        .map(|(event, day)| {
            let session = event
                .session
                .and_then(|id| sessions.iter().find(|session| session.id == id));
            let mut label = event.label(*day);
            if event.kind != Kind::Day && *day > 0 {
                label.push_str(&format!(" · Day {day}"));
            }
            if let Some(session) = session {
                label.push_str(&format!(" · {}", short_session(&session.label)));
            }
            label
        })
        .collect()
}

/// What the typed place of an event could be (`TimelinePage::place_help_of`).
pub enum PlaceHelp {
    None,
    /// The one location it names, by its first name: a link reads it.
    Exact(Uuid, String),
    /// Up to five names that contain it: a link puts one in the field.
    Close(Vec<String>),
}

/// A row's words (`TimelinePage::row_words`).
pub struct RowWords {
    pub title: String,
    /// The second line; empty when the row has none.
    pub detail: String,
    /// Byte ranges of `detail` with their own tint, in order.
    pub spans: Vec<(std::ops::Range<usize>, RowTint)>,
    /// `Written as Day 5`, after a day heading written out of order.
    pub flag: Option<String>,
    /// The row's accessible name: its title and its second line.
    pub name: String,
    /// The run of this fight is open: its result is a link back to the run.
    pub open: bool,
    /// The played fight's result and last round (the open run's link words).
    pub summary: String,
    /// The length cannot be counted: the row's hover says how to write it.
    pub warning: bool,
}

/// A status part's tint on a row's second line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowTint {
    /// A length the clock cannot count.
    Warning,
    /// A long rest's ruling: everyone at full, or not.
    Ruling(bool),
}

impl RowTint {}

/// The session's number alone. The row's own title is the event's.
pub fn short_session(label: &str) -> &str {
    label.split(':').next().unwrap_or(label)
}

/// The kind's word before the open event's title, after its icon: none for a day or an untitled
/// event, whose label already says its kind, so the header says the kind once.
pub fn header_kind(event: &Event) -> &'static str {
    if event.kind == Kind::Day || event.own_title().is_none() {
        ""
    } else {
        event.kind.word()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_untitled_rest_names_its_kind_once_in_the_header() {
        let mut book = Book::default();
        book.add(Kind::LongRest, None);
        let mut event = book.events.remove(0);
        assert_eq!(
            format!("{} {}", header_kind(&event), event.label(1))
                .matches("Long rest")
                .count(),
            1
        );
        event.title = "Rest at the mill".into();
        assert!(header_kind(&event).contains("Long rest"));
        assert_eq!(event.label(1), "Rest at the mill");
        // Round 28 FL8, FN3, FH7: the plan gives a new event its kind as its title. The header,
        // the label and the row's second line then say the kind once.
        for (kind, word) in [(Kind::Travel, "Travel"), (Kind::Day, "Day")] {
            let mut book = Book::default();
            book.add(kind, None);
            let mut event = book.events.remove(0);
            event.title = word.into();
            let header = format!("{} {}", header_kind(&event), event.label(1));
            assert_eq!(header.matches(word).count(), 1, "{header}");
            assert!(!event.facts().contains(word), "{}", event.facts());
        }
    }

    #[test]
    fn iter12_c_ft4_timeline_removal_names_the_fight_from_its_own_session() {
        let (here, elsewhere) = (Some(Uuid::new_v4()), Some(Uuid::new_v4()));
        let mut book = Book::default();
        let mut fight = |session, result: &str| {
            let id = book.add(Kind::Encounter, session);
            let event = book.events.iter_mut().find(|event| event.id == id).unwrap();
            event.title = "Bridge Ambush".to_owned();
            event.result = result.to_owned();
            event.run = Some(Uuid::new_v4());
            id
        };
        let paused = fight(here, "Round 2");
        fight(here, "Held the bridge, round 1");
        // A fight of the same name in another session is not "the other" one.
        let other = fight(elsewhere, "Round 3");
        book.remove(&paused);
        assert_eq!(
            timeline_removal_words(&book, "Bridge Ambush", 0),
            "Removed “Bridge Ambush” (Round 2 · Paused). The other “Bridge Ambush” and its result stay."
        );
        // The notes that still link to it stay last.
        assert_eq!(
            timeline_removal_words(&book, "Bridge Ambush", 2),
            "Removed “Bridge Ambush” (Round 2 · Paused). The other “Bridge Ambush” and its result \
             stay. 2 notes still link to it."
        );
        assert_eq!(
            timeline_removal_words(&book, "Bridge Ambush", 1),
            "Removed “Bridge Ambush” (Round 2 · Paused). The other “Bridge Ambush” and its result \
             stay. 1 note still links to it."
        );
        // The session comes from the removed event: the one in the other session stands alone.
        crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove);
        book.remove(&other);
        assert_eq!(
            timeline_removal_words(&book, "Bridge Ambush", 0),
            "Removed “Bridge Ambush” (Round 3 · Paused)."
        );
    }

    /// An event with no session has no "other" event of its name: the sentence is left out.
    #[test]
    fn iter12_c2_removal_without_a_session_names_no_other_event() {
        let mut book = Book::default();
        let mut fight = |result: &str| {
            let id = book.add(Kind::Encounter, None);
            let event = book.events.iter_mut().find(|event| event.id == id).unwrap();
            event.title = "Bridge Ambush".to_owned();
            event.result = result.to_owned();
            event.run = Some(Uuid::new_v4());
            id
        };
        let paused = fight("Round 2");
        fight("Held the bridge, round 1");
        book.remove(&paused);
        assert_eq!(
            timeline_removal_words(&book, "Bridge Ambush", 1),
            "Removed “Bridge Ambush” (Round 2 · Paused). 1 note still links to it."
        );
    }

    /// The list footer's lines, as the page shows them under the list.
    fn footer(
        page: &mut TimelinePage,
        book: &Book,
        notes: &mut crate::sessions::page::Sessions,
        open: Option<&str>,
    ) -> Vec<design::outcome::plain::PlainLine<ListAct>> {
        let frame = PaneFrame::new();
        page.list_messages(&frame, book, notes, open, None)
            .into_plain()
    }

    /// The Undo of the footer line whose Undo has this label, pressed.
    fn press_undo(
        page: &mut TimelinePage,
        book: &mut Book,
        notes: &mut crate::sessions::page::Sessions,
        open: Option<&str>,
        label: &str,
    ) {
        let press = footer(page, book, notes, open)
            .iter()
            .find(|line| line.undo.as_ref().is_some_and(|undo| undo.label == label))
            .and_then(|line| line.press_undo())
            .unwrap_or_else(|| panic!("the footer has {label}"));
        page.footer_act(press, book, notes, None);
    }

    /// Round 30 B7d, ruling L9: the Add result ends in the page's owner update
    /// (`settle_added`), before the footer presents its kept line: by the frame clock (until it,
    /// the words and `Undo add` stand; at it, both go); when a reload of the book lost its event
    /// (A gone, B unchanged; an edit that keeps it is the control); and at once when another event
    /// is opened. Unchanged settles build the kept line no more.
    #[test]
    fn the_add_result_ends_by_the_frame_clock() {
        let ctx = StateContext::new();
        let dir = tempfile::tempdir().unwrap();
        let mut book = Book::default();
        book.open(Some(dir.path()));
        let b = book.add(Kind::Event, None);
        book.events[0].title = "Toll gate".into();
        book.save();
        let mut page = TimelinePage::default();
        let mut notes = crate::sessions::page::Sessions::default();
        let shows =
            |page: &mut TimelinePage, book: &Book, notes: &mut crate::sessions::page::Sessions| {
                let lines = footer(page, book, notes, None);
                (
                    lines.iter().any(|line| {
                        line.undo
                            .as_ref()
                            .is_some_and(|undo| undo.label == "Undo add Major event")
                    }),
                    lines.iter().any(|line| line.words.contains(" added.")),
                )
            };
        let add = |page: &mut TimelinePage, book: &mut Book, at: f64| {
            crate::frame_clock::set(at);
            let id = book.add(Kind::Event, None);
            page.record_added(id.clone(), book, &ctx);
            page.settle_added(&ctx, book);
            assert_eq!(page.selected.as_deref(), Some(id.as_str()), "it opens");
            id
        };
        let a = add(&mut page, &mut book, 100.0);
        assert_eq!(
            shows(&mut page, &book, &mut notes),
            (true, true),
            "the add says so, with its Undo"
        );
        let start = crate::design::results::projection_work();
        for at in [129.0, 129.5] {
            crate::frame_clock::set(at);
            page.settle_added(&ctx, &book);
            assert_eq!(
                shows(&mut page, &book, &mut notes),
                (true, true),
                "{at}: it stands"
            );
        }
        let quiet = crate::design::results::projection_work();
        assert_eq!(
            (quiet.built - start.built, quiet.bytes - start.bytes),
            (0, 0),
            "settles with no change build the kept line no more"
        );
        crate::frame_clock::set(130.0);
        page.settle_added(&ctx, &book);
        assert_eq!(
            shows(&mut page, &book, &mut notes),
            (false, false),
            "30 s later the words and the Undo are gone together"
        );
        // F2 of the early review: a reload of the book that lost the added event ends its
        // result at the next settle; an edit that keeps it does not.
        let edit_on_disk = |keep: &dyn Fn(&Event) -> bool, title: Option<&str>| {
            let mut other = Book::default();
            other.open(Some(dir.path()));
            other.events.retain(|event| keep(event));
            if let Some(title) = title {
                other.events[0].title = title.into();
            }
            other.touch();
            other.save();
        };
        let a2 = add(&mut page, &mut book, 200.0);
        book.save();
        edit_on_disk(&|_| true, Some("Toll bridge"));
        assert!(book.read_if_changed());
        page.settle_added(&ctx, &book);
        assert_eq!(
            shows(&mut page, &book, &mut notes),
            (true, true),
            "an edit on disk that keeps the added event keeps its result"
        );
        let a2_id = a2.clone();
        edit_on_disk(&move |event| event.id != a2_id, None);
        assert!(book.read_if_changed());
        page.settle_added(&ctx, &book);
        assert_eq!(
            shows(&mut page, &book, &mut notes),
            (false, false),
            "a reload that lost the added event ends its result"
        );
        assert!(
            book.events
                .iter()
                .any(|event| event.id == b && event.title == "Toll bridge"),
            "the unrelated event B stays"
        );
        assert_ne!(a, a2);
        // Another event opened ends it at once.
        let a3 = add(&mut page, &mut book, 300.0);
        page.select(Some(b.clone()));
        page.settle_added(&ctx, &book);
        assert_eq!(
            shows(&mut page, &book, &mut notes),
            (false, false),
            "another selection ends it"
        );
        assert!(book.events.iter().any(|event| event.id == a3));
        // M0548: the reload result's Undo names its file as the app does everywhere, with its
        // extension.
        book.reload_keeping_undo();
        assert!(
            footer(&mut page, &book, &mut notes, None)
                .iter()
                .any(|line| line
                    .undo
                    .as_ref()
                    .is_some_and(|undo| undo.label == "Undo reload Timeline.md")),
            "the reload's Undo names its file"
        );
    }

    /// A closed long rest's row says the ruling in force, either way, after its label.
    #[test]
    fn iter3_c_closed_long_rest_rows_show_both_rulings() {
        for restores in [false, true] {
            let mut book = Book::default();
            book.add(Kind::LongRest, None);
            let mut event = book.events.remove(0);
            event.restores = restores;
            let word = if restores {
                "Everyone at full"
            } else {
                "Not at full"
            };
            let icon = if restores {
                icons::OK
            } else {
                icons::LONG_REST
            };
            let words = TimelinePage::default().row_words(&event, 0, None, None, None);
            assert_eq!(words.name, format!("{} · {icon} {word}", event.label(0)));
            assert!(
                words
                    .spans
                    .iter()
                    .any(|(at, tint)| *tint == RowTint::Ruling(restores)
                        && words.detail[at.clone()] == format!("{icon} {word}")),
                "the ruling has its own tint: {:?}",
                words.spans
            );
        }
    }

    /// A row repeats its day only when a filter can hide the day's heading.
    #[test]
    fn iter1_c_day_appears_only_when_filters_hide_day_headings() {
        for filtered in [false, true] {
            let mut book = Book::default();
            book.add(Kind::ShortRest, None);
            let event = book.events.remove(0);
            let detail = event.summary_with_result(None, "");
            let expected_detail = if filtered {
                if detail.is_empty() {
                    "Day 1".into()
                } else {
                    format!("Day 1 · {detail}")
                }
            } else {
                detail
            };
            let expected = if expected_detail.is_empty() {
                event.label(1)
            } else {
                format!("{} · {expected_detail}", event.label(1))
            };
            let mut page = TimelinePage::default();
            if filtered {
                page.filter = Filter::Rests;
            }
            assert_eq!(page.row_words(&event, 1, None, None, None).name, expected);
        }
    }

    /// iter-23-S FL1: each direction of a rest's ruling says the ruling now in force in the list's
    /// footer, while its rest is open, and its Undo puts the ruling back.
    #[test]
    fn iter1_c_rest_ruling_actions_name_the_change_and_keep_undo() {
        let mut book = Book::default();
        book.add(Kind::LongRest, None);
        book.events[0].restores = true;
        let id = book.events[0].id.clone();
        let mut page = TimelinePage::default();
        let mut notes = crate::sessions::page::Sessions::default();
        let says = |page: &mut TimelinePage,
                    book: &Book,
                    notes: &mut crate::sessions::page::Sessions,
                    words: &str| {
            footer(page, book, notes, Some(&id))
                .iter()
                .any(|line| line.words == words)
        };
        RestRulingAction::Set(false).apply(&mut book, &id);
        assert!(!book.events[0].restores);
        assert!(says(
            &mut page,
            &book,
            &mut notes,
            "The next fight starts from the HP the party has now."
        ));
        RestRulingAction::Set(true).apply(&mut book, &id);
        assert!(book.events[0].restores);
        assert!(says(&mut page, &book, &mut notes, REST_RESTORES_WORDS));
        assert!(
            footer(&mut page, &book, &mut notes, None)
                .iter()
                .all(|line| line.words != REST_RESTORES_WORDS),
            "another event open: no ruling line"
        );
        press_undo(
            &mut page,
            &mut book,
            &mut notes,
            Some(&id),
            "Undo change to ruling",
        );
        assert!(!book.events[0].restores, "Undo puts the ruling back");
    }

    /// Review of PR 334: the list view is worked out once for an unchanged book and filters (the
    /// count is inside `ListView::make`); a change of the filters works it out again, once.
    #[test]
    fn a_long_list_works_out_its_view_once() {
        let mut book = Book::default();
        for i in 0..30 {
            let id = book.add(Kind::Event, None);
            if let Some(event) = book.events.iter_mut().find(|event| event.id == id) {
                event.title = format!("Event {i}");
            }
        }
        let mut page = TimelinePage::default();
        let made = VIEWS_MADE.with(std::cell::Cell::get);
        for _ in 0..12 {
            let _ = page.view(&book);
        }
        assert_eq!(
            VIEWS_MADE.with(std::cell::Cell::get) - made,
            1,
            "twelve reads with an unchanged book and filters work out the list view once"
        );
        page.filter = Filter::Events;
        for _ in 0..12 {
            let _ = page.view(&book);
        }
        assert_eq!(
            VIEWS_MADE.with(std::cell::Cell::get) - made,
            2,
            "a new filter, one new view"
        );
    }

    /// Review of PR 340: a removal that notes link to keeps its warning when the DM opens
    /// another campaign's Timeline, works there, and comes back. B shows no Undo of A's removal
    /// (review of PR 720), and B's file is not touched by A's Undo.
    #[test]
    fn a_removal_keeps_its_linked_note_warning_over_a_campaign_switch() {
        let (a, b) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let mut book = Book::default();
        book.open(Some(a.path()));
        let id = book.add(Kind::Event, None);
        book.events[0].title = "Toll gate".into();
        book.save();
        let mut page = TimelinePage::default();
        let mut notes = crate::sessions::page::Sessions::default();
        book.remove_linked(&id, 1);
        let warned =
            |page: &mut TimelinePage, book: &Book, notes: &mut crate::sessions::page::Sessions| {
                footer(page, book, notes, None)
                    .iter()
                    .any(|line| line.words.contains("1 note still links to it"))
            };
        assert!(
            warned(&mut page, &book, &mut notes),
            "A's removal says a note links to it"
        );
        // Campaign B: its own Timeline, and a day added there.
        book.open(Some(b.path()));
        book.add(Kind::Day, None);
        book.save();
        assert!(
            footer(&mut page, &book, &mut notes, None)
                .iter()
                .all(|line| line
                    .undo
                    .as_ref()
                    .is_none_or(|undo| !undo.label.starts_with("Undo remove "))),
            "campaign B shows no Undo of A's removal"
        );
        let b_bytes = std::fs::read(b.path().join(super::super::FILE)).unwrap();
        // Back to A: the removal and its warning are still A's.
        book.open(Some(a.path()));
        assert!(
            warned(&mut page, &book, &mut notes),
            "A's warning came back with A's removal"
        );
        press_undo(
            &mut page,
            &mut book,
            &mut notes,
            None,
            "Undo remove Toll gate",
        );
        assert!(
            book.events.iter().any(|event| event.id == id),
            "the Undo puts back A's event"
        );
        assert_eq!(
            std::fs::read(b.path().join(super::super::FILE)).unwrap(),
            b_bytes
        );
    }
}
