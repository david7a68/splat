//! The Timeline page: the whole campaign in story order, apart from any one session.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{
    Book, Event, EventDisplay, Kind, checks, clock, clock_now, clock_words, day_numbers, kind_icon,
    reference,
};
use crate::design::{
    self, Action, Density, Status, Type, colors, icons,
    outcome::{
        BodyMessage, Choice, Fault, FaultMessage, FileLine, KeptOutcome, MessageChoice, Messages,
        Notice, PaneFrame, Said, Sentence, Severity, UndoStep, Verb,
    },
};
use eframe::egui;
use std::time::{Duration, Instant};
use uuid::Uuid;

const ROW: f32 = 44.0;
const SPINE: f32 = 16.0;
const TEXT_AT: f32 = 34.0;
const TWO_COLUMNS_FROM: f32 = 640.0;
const FILTERS_ON_ONE_ROW_FROM: f32 = design::LIST_MEASURE;
const UNDO_ADD_FOR: Duration = Duration::from_secs(30);
const COPIED_FOR: Duration = Duration::from_secs(2);

const TIMELINE_SEARCH: design::FieldId =
    crate::field_id!("timeline_search").once(design::Once::PageSearch);
const EVENT_NOTES: design::FieldName = crate::field_id!("event_notes");
const EVENT_TITLE: design::FieldName = crate::field_id!("event_title");
const EVENT_PLACE: design::FieldName = crate::field_id!("event_place");
const EVENT_KIND: design::FieldName = crate::field_id!("timeline_event_kind");
const EVENT_SESSION: design::FieldName = crate::field_id!("timeline_event_session");
const NEW_EVENTS_SESSION: design::FieldId =
    crate::field_id!("timeline_new_events_session").once(design::Once::PageSearch);
const PLACE_AFTER: design::FieldName = crate::field_id!("timeline_place_after");
/// The page's own view choice: which session it shows. It is the page's state, not saved
/// object data.
const SESSION_FILTER: design::FieldId =
    crate::field_id!("timeline_session_filter").once(design::Once::PageSearch);

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Filter {
    #[default]
    All,
    Days,
    Encounters,
    Rests,
    Travel,
    Events,
}

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
struct ListView {
    flagged: std::collections::HashMap<String, u32>,
    days: Vec<u32>,
    rows: Vec<usize>,
    road: std::collections::HashMap<String, String>,
    now: Option<u64>,
}

/// The filters as one value with a revision, so the list view's `Memo` can key by them.
struct FilterKey(design::memo::Revised<(Filter, String, Option<Uuid>)>);

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
    filters: FilterKey,
    /// The list view, by the book's revision and the filters' (`ListView`).
    view: design::memo::Memo<
        (design::memo::Revision, design::memo::Revision),
        std::sync::Arc<ListView>,
    >,
    filter: Filter,
    search: String,
    session: Option<Uuid>,
    selected: Option<String>,
    copied: Option<(String, Instant)>,
    focus_title: bool,
    /// A modal is open over the app (the app's one test, `modal_open`): Ctrl+S waits for it.
    pub modal: bool,
    /// The open event's form opens at its top in the next draw: another event, a link, the page
    /// opened again. Never a filter or a search, which change only the list.
    reset_detail: bool,
    scroll_to: Option<usize>,
    /// An event just added with the buttons, with one step of undo.
    added: design::Results<super::TimelineFile, JustAdded>,
    /// Its footer line, kept as data across frames (ruling L9): made again only when the store,
    /// the book or the saved-run facts changed.
    added_view: design::results::Retained<super::TimelineFile, ListAct, AddedInputs>,
    /// The book's revision at the last check that the Add result's event is still there.
    added_seen: Option<design::memo::Revision>,
    /// The list opens where the story is now, at its end.
    opened: bool,
    /// One column: the detail hides the list, so the open event's results end its form.
    narrow: bool,
    /// The run open now, whose event must stay while it runs.
    pub open_run: Option<Uuid>,
    /// Saved runs from the same campaign snapshot used by the Session plan, and the revision of
    /// their facts (`Encounters::saved_runs_revision`): `set_saved_runs`.
    saved_runs: Vec<crate::encounter_run::SavedRunInfo>,
    saved_runs_revision: Option<design::memo::Revision>,
    /// The DM followed an open fight's status back to the run.
    pub wants_back_to_run: bool,
    /// Filters and the selection belong to one campaign.
    campaign: Option<Uuid>,
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
    focus_length: bool,
    /// Round 29 G142: a roll was made in this journey (its id) at this check (its offset): the
    /// journey's next draw scrolls least so that the check's result and its offer lines are in
    /// view, once. Another selection, another journey's draw or a reopened page drops it.
    reveal_checks: Option<(String, u64)>,
    /// What the DM typed in a journey's checks, and each journey's last result with its Undo.
    checks: checks::State,
    checks_done: checks::Results,
}

/// One button for each of these kinds of event. Returns the new event's ID.
pub fn quick_add(
    ui: &mut egui::Ui,
    book: &mut Book,
    session: Option<Uuid>,
    kinds: &[Kind],
    first: impl FnOnce(&mut egui::Ui),
) -> Option<String> {
    quick_add_with(ui, book, session, kinds, Vec::new(), first).and_then(|chosen| chosen.ok())
}

/// `quick_add`, with lines of the strip's More menu alone, after the kinds. A new event's ID
/// comes back as `Ok`, and a chosen line of `more` as `Err` with its index there.
pub fn quick_add_with(
    ui: &mut egui::Ui,
    book: &mut Book,
    session: Option<Uuid>,
    kinds: &[Kind],
    more: Vec<Action<'_>>,
    first: impl FnOnce(&mut egui::Ui),
) -> Option<Result<String, usize>> {
    let blocked = book.unreadable.is_some();
    let names: Vec<_> = kinds
        .iter()
        .map(|kind| format!("Add {}", kind.word()))
        .collect();
    let actions = kinds
        .iter()
        .zip(&names)
        .map(|(kind, name)| {
            Action::quiet(kind.word())
                .icon(kind_icon(*kind))
                .name(name)
                .density(Density::Read)
                .blocked_if(blocked, "The timeline file must be repaired first.")
        })
        .collect();
    // Fight and Long rest stay visible; the rarer kinds share More when space is short.
    let always_visible: Vec<usize> = kinds
        .iter()
        .enumerate()
        .filter(|(_, kind)| matches!(kind, Kind::Encounter | Kind::LongRest))
        .map(|(index, _)| index)
        .collect();
    design::hanging_add_strip(ui, first, actions, &always_visible, more).map(|index| {
        let Some(&kind) = kinds.get(index) else {
            return Err(index - kinds.len());
        };
        Ok(if kind == Kind::LongRest {
            book.add_long_rest(session)
        } else {
            book.add(kind, session)
        })
    })
}

/// What the DM chose in the Timeline list's message footer, done after the footer is drawn.
#[derive(Clone, Debug, PartialEq)]
enum ListAct {
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
    /// The event's name when it was added: the result's words and its Undo name it, so a
    /// rename while the result shows changes neither (review of PR 901).
    label: Option<String>,
}

impl JustAdded {
    /// The result of the event `id` just added to `book`, with the name it has now.
    fn of(book: &Book, id: String) -> Self {
        let label = book
            .events
            .iter()
            .position(|event| event.id == id)
            .map(|at| book.events[at].label(day_numbers(&book.events)[at]));
        Self {
            file: book.file(),
            id,
            label,
        }
    }
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
/// the file, or a second timeline file (in the footer); the file that cannot be read or that
/// changed under the draft (in the page's place, `problems`). The kind gives the tone.
enum FileFault {
    /// Notes still link to the removed event: the DM checks them.
    Linked(Sentence),
    NotSaved(Sentence),
    /// Timeline.md cannot be read, is gone, or its comparison cannot be made.
    Unread(Sentence),
    /// Timeline.md changed on disk under the DM's unsaved edit.
    Changed,
    /// A line of the file that Splat read its own way (`Line 9: “Day 5” is day 1 in order. …`).
    Flag(Sentence),
    /// Another timeline file in the Timeline's folder.
    Copy,
}

impl Fault for FileFault {
    fn severity(&self) -> Severity {
        match self {
            Self::Linked(_) | Self::Flag(_) | Self::Copy | Self::Changed => Severity::NeedsFixing,
            Self::NotSaved(_) => Severity::NotSaved,
            Self::Unread(_) => Severity::CannotRead,
        }
    }
    fn cause(&self) -> Sentence {
        match self {
            Self::Linked(words)
            | Self::NotSaved(words)
            | Self::Flag(words)
            | Self::Unread(words) => words.clone(),
            Self::Changed => as_sentence(crate::timeline::CHANGED_UNDER_DRAFT),
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
    reason = "words of a file, of the system or of the events, as text (design::results::STAGED_ALLOWS)"
)]
fn as_sentence(words: &str) -> Sentence {
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

pub fn rest_ruling(ui: &mut egui::Ui, event: &Event) -> Option<RestRulingAction> {
    rest_ruling_line(ui, event)
}

/// The ruling now in force and the action that changes it. The action changes data, so it is a
/// quiet button after the status, never a link.
pub fn rest_ruling_line(ui: &mut egui::Ui, event: &Event) -> Option<RestRulingAction> {
    if event.kind != Kind::LongRest {
        return None;
    }
    let mut action = None;
    ui.horizontal_wrapped(|ui| {
        design::status(ui, long_rest_status(event.restores));
        // The line's item gap (`S`, the theme's) and this `S` make `L`.
        design::gap(ui, design::Gap::S);
        if Action::quiet(if event.restores {
            "Mark not at full"
        } else {
            "Mark everyone at full"
        })
        .density(Density::Read)
        .show(ui)
        .clicked()
        {
            action = Some(RestRulingAction::Set(!event.restores));
        }
    });
    action
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

/// What stops the timeline from saving. The DM must choose, so this cannot be dismissed.
/// Returns true when the comparison needs the rest of the pane.
#[allow(
    clippy::disallowed_methods,
    reason = "ruling M20: Timeline.md’s problems in the page’s place, typed (`Ban::Body`)"
)]
pub fn problems(ui: &mut egui::Ui, book: &mut Book) -> bool {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Repair {
        ReadAgain,
        SaveAgain,
        Overwrite,
        Reload,
    }
    if let Some(error) = book.unreadable.clone() {
        // Its choices start at its words; a gone file can also be saved there again.
        let mut message =
            FaultMessage::new(&FileFault::Unread(as_sentence(&error))).choice(Choice::new(
                MessageChoice::Choice("Read the file again"),
                Repair::ReadAgain,
            ));
        if book.gone {
            message = message.choice(Choice::new(
                MessageChoice::Choice("Save this timeline there again"),
                Repair::SaveAgain,
            ));
        }
        match BodyMessage::from(message).show(ui) {
            Some(Repair::ReadAgain) => book.reload(),
            Some(Repair::SaveAgain) => book.save_again(),
            _ => {}
        }
    } else if book.conflict {
        // The safe choice first, then the red one `L` after it (the part's).
        let pressed = BodyMessage::from(
            FaultMessage::new(&FileFault::Changed)
                .choice(Choice::new(
                    MessageChoice::Choice("Overwrite with my timeline"),
                    Repair::Overwrite,
                ))
                .choice(Choice::new(
                    MessageChoice::Lose("Reload and lose my timeline"),
                    Repair::Reload,
                )),
        )
        .show(ui);
        match pressed {
            Some(Repair::Overwrite) => book.overwrite(),
            Some(Repair::Reload) => book.reload_keeping_undo(),
            _ => {}
        }
        if book.conflict {
            match book.conflict_comparison() {
                Ok((draft, disk)) => {
                    let face = egui::TextStyle::Monospace.resolve(ui.style());
                    let height = ui.fonts_mut(|fonts| fonts.row_height(&face));
                    let id = ("timeline_conflict", book.path(), &disk);
                    design::draft_comparison_styled(ui, &draft, &disk, id, face, height);
                }
                Err(error) => {
                    let fault = FileFault::Unread(as_sentence(&error));
                    let _ = BodyMessage::<()>::from(FaultMessage::new(&fault)).show(ui);
                }
            }
            return true;
        }
    }
    false
}

/// The current session's note is dated before today (`dated_before_today`): the choice to play
/// this one. True when the DM pressed it.
#[allow(
    clippy::disallowed_methods,
    reason = "ruling M20: the session date notice above the list, typed (`Ban::Body`)"
)]
fn dated_notice(ui: &mut egui::Ui, words: &str) -> bool {
    BodyMessage::from(
        Notice::warning(as_sentence(words))
            .choice(Choice::new(MessageChoice::Choice("Play this one"), ())),
    )
    .show(ui)
    .is_some()
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
    fn select(&mut self, selected: Option<String>) {
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
    fn settle_added(&mut self, ctx: &egui::Context, book: &Book) {
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
    fn view(&mut self, book: &Book) -> std::sync::Arc<ListView> {
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

    /// Where a new event goes, said before it happens. An encounter run goes there too.
    ///
    /// The closed control keeps the short session name; its menu and accessible value keep
    /// the whole name. The Dropdown part owns the regular weight.
    fn new_events_line(
        &mut self,
        ui: &mut egui::Ui,
        sessions: &[SessionLink],
        current: Option<Uuid>,
        room: egui::Rect,
    ) {
        ui.label(Type::Small.rich(ui, "Adds to").color(colors().muted));
        let mut chosen = current;
        let mut dropdown = design::Dropdown::new(NEW_EVENTS_SESSION, "Session for new events")
            .choice(None, "No session");
        for session in sessions {
            dropdown = dropdown.choice_with_short_label(
                Some(session.id),
                session.label.as_str(),
                short_session(&session.label),
            );
        }
        // This header's room starts on its row: its menu opens below, never above it.
        let mut room = room;
        room.min.y = room.top().max(ui.cursor().top());
        let shown = dropdown
            .within(room)
            .width(design::READER_MEASURE / 4.0)
            .show(ui, &mut chosen);
        design::named(ui, &shown.response, "Session for new events");
        if chosen != current {
            self.new_events_to = Some(chosen);
        }
    }

    fn filters(
        &mut self,
        ui: &mut egui::Ui,
        sessions: &[SessionLink],
        current: Option<Uuid>,
        room: egui::Rect,
    ) {
        let tabs = |ui: &mut egui::Ui, filter: &mut Filter| {
            design::segmented(
                ui,
                filter,
                &[
                    (Filter::All, "All"),
                    (Filter::Days, "Days"),
                    (Filter::Encounters, "Fights"),
                    (Filter::Rests, "Rests"),
                    (Filter::Travel, "Travel"),
                    (Filter::Events, "Major events"),
                ],
            );
        };
        let wide = ui.available_width() >= FILTERS_ON_ONE_ROW_FROM;
        if !wide {
            tabs(ui, &mut self.filter);
        }
        ui.horizontal_wrapped(|ui| {
            if wide {
                tabs(ui, &mut self.filter);
            }
            // `Clear` changes `self.search` only. A filter or a search changes the list, not the
            // open event: its form keeps its place, also when the search hides its row (review
            // of #817).
            let _ = TIMELINE_SEARCH.search(
                ui,
                "Search the timeline",
                &mut self.search,
                "Search",
                160.0,
            );
            let mut menu_room = room;
            menu_room.min.y = menu_room.top().max(ui.cursor().top());
            design::Dropdown::new(SESSION_FILTER, "Show one session")
                .within(menu_room)
                .choice(None, "All sessions")
                .choices(
                    sessions
                        .iter()
                        .map(|session| (Some(session.id), session.label.as_str())),
                )
                .width(150.0)
                .show(ui, &mut self.session);
            self.new_events_line(ui, sessions, current, room);
        });
    }

    /// Tests: a reveal left pending by a roll in `journey` at `at`, as when the DM leaves that
    /// journey before its next draw.
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_pending_reveal(&mut self, journey: &str, at: u64) {
        self.reveal_checks = Some((journey.to_owned(), at));
    }

    pub fn reopen(&mut self) {
        self.reset_detail = true;
        self.reveal_checks = None;
        // The journeys' results and their Undo end when the DM leaves the page.
        self.checks_done.clear();
    }

    fn row(
        &mut self,
        ui: &mut egui::Ui,
        event: &Event,
        day: u32,
        session: Option<&str>,
        written_as: Option<u32>,
        on_road: Option<&str>,
    ) {
        let c = colors();
        let selected = self.selected.as_deref() == Some(event.id.as_str());
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
            ruling = Some((detail.len(), status.color));
            detail.push_str(&format!("{} {}", status.icon, status.word));
        }
        // Round 26 FG4, FD7: a length the clock cannot count is a warning part, in its colour.
        let mut spans = Vec::new();
        let warning = event.time_warning();
        if let Some(warning) = &warning
            && let Some(start) = detail.find(warning.as_str())
        {
            spans.push((start..start + warning.len(), c.result.warning));
        }
        if let Some((start, color)) = ruling {
            spans.push((start..detail.len(), color));
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
        let response = design::row(ui, ROW, selected, &name, |ui, rect| {
            let painter = ui.painter();
            let spine = rect.left() + SPINE;
            let width = rect.width() - TEXT_AT - design::S;
            let icon_y = rect.center().y;
            if event.kind != Kind::Day {
                for (from, to) in [(rect.top(), icon_y - 11.0), (icon_y + 11.0, rect.bottom())] {
                    painter.vline(
                        spine,
                        egui::Rangef::new(from, to),
                        egui::Stroke::new(design::HAIRLINE_WIDTH, c.rule),
                    );
                }
            }
            painter.text(
                egui::pos2(spine, icon_y),
                egui::Align2::CENTER_CENTER,
                kind_icon(event.kind),
                Type::Name.font(ui),
                if event.kind == Kind::Day {
                    c.text
                } else {
                    c.muted
                },
            );
            // The warning icon and a muted word after the heading, in `Small`.
            let flag = flag.as_ref().map(|flag| {
                let mut job = egui::text::LayoutJob::default();
                for (text, color) in [
                    (icons::WARN.to_owned(), c.result.warning),
                    (format!(" {flag}"), c.muted),
                ] {
                    job.append(
                        &text,
                        0.0,
                        egui::TextFormat::simple(Type::Small.font(ui), color),
                    );
                }
                painter.layout_job(job)
            });
            let flag_room = flag.as_ref().map_or(0.0, |flag| flag.size().x + design::S);
            let galley = if event.kind == Kind::Day {
                let mut job = design::group_heading_text(ui, &title, 0);
                job.wrap.max_width = (width - flag_room).max(0.0);
                job.wrap.max_rows = 1;
                painter.layout_job(job)
            } else {
                let font = if selected { Type::Strong } else { Type::Body };
                design::one_line(ui, &title, font.font(ui), c.text, width)
            };
            let lines = if detail.is_empty() {
                galley.size().y
            } else {
                galley.size().y + Type::Small.line()
            };
            let top = rect.center().y - lines / 2.0;
            let x = rect.left() + TEXT_AT;
            if let Some(flag) = flag {
                let at = egui::pos2(
                    x + galley.size().x + design::S,
                    top + (galley.size().y - flag.size().y) / 2.0,
                );
                painter.galley(at, flag, c.muted);
            }
            painter.galley(egui::pos2(x, top), galley, c.text);
            if !detail.is_empty() && !open {
                let second = second_line(ui, &detail, 0, &spans, width);
                painter.galley(
                    egui::pos2(x, top + lines - Type::Small.line()),
                    second,
                    c.muted,
                );
            }
        });
        if open && !summary.is_empty() {
            // The link uses the same second-line position as every other row.
            let title_height = ui.fonts_mut(|fonts| fonts.row_height(&Type::Body.font(ui)));
            let top = response.rect.center().y - (title_height + Type::Small.line()) / 2.0;
            let rect = egui::Rect::from_min_max(
                egui::pos2(response.rect.left() + TEXT_AT, top + title_height),
                egui::pos2(
                    response.rect.right() - design::S,
                    top + title_height + Type::Small.line(),
                ),
            );
            ui.scope_builder(
                egui::UiBuilder::new()
                    .max_rect(rect)
                    .accessibility_parent(response.id),
                |ui| {
                    ui.spacing_mut().interact_size.y = 0.0;
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        let (before, after) = detail.split_once(&summary).unwrap_or(("", ""));
                        ui.label(Type::Small.rich(ui, before).color(c.muted));
                        if design::inline_status_link(
                            ui,
                            Status {
                                icon: icons::RUN,
                                word: &summary,
                                color: c.text,
                            },
                        )
                        .clicked()
                        {
                            self.wants_back_to_run = true;
                        }
                        // The words after the link fit as the closed row's do (M0552): whole
                        // words to the row's end, in their own colours.
                        let from = detail.len() - after.len();
                        let width = ui.available_width();
                        ui.label(second_line(ui, &detail, from, &spans, width));
                    });
                },
            );
        }
        let response = if warning.is_some() {
            response.on_hover_text(format!(
                "{}. {}",
                super::TIME_NOT_COUNTED,
                super::TIME_REPAIR
            ))
        } else {
            response
        };
        if response.clicked() {
            self.reset_detail = self.selected.as_ref() != Some(&event.id);
            self.select(Some(event.id.clone()));
            self.focus_title = false;
        }
    }

    /// Locations the typed place could be, or a link to the one it is. The place stays text.
    fn place_help(&mut self, ui: &mut egui::Ui, event: &mut Event) -> bool {
        let typed = event.place.trim().to_lowercase();
        if typed.is_empty() {
            return false;
        }
        let exact: Vec<_> = self
            .places
            .iter()
            .filter(|(_, names)| names.iter().any(|name| name.to_lowercase() == typed))
            .collect();
        if let [(id, names)] = exact.as_slice() {
            if design::link(ui, &format!("Read {}", names[0])).clicked() {
                self.open_place = Some(*id);
            }
            return false;
        }
        let close: Vec<_> = self
            .places
            .iter()
            .filter_map(|(_, names)| {
                names
                    .iter()
                    .find(|name| name.to_lowercase().contains(&typed))
            })
            .take(5)
            .collect();
        if close.is_empty() {
            return false;
        }
        let mut changed = false;
        ui.horizontal_wrapped(|ui| {
            for name in close {
                if design::link(ui, name).clicked() {
                    event.place = name.clone();
                    changed = true;
                }
            }
        });
        changed
    }

    /// Round 26: the list column's results, reasons and notices sit in its message footer, so a
    /// result never moves a row: the link update after a rename and its Undo, the open event's
    /// rest ruling, the file's notices, a removal and an add. Each comes from its own store (the
    /// book's, the session page's, this page's) and a press acts on the entry its ticket names;
    /// the footer only places them. Round 28 (AL1, FL1, AM1): the column is one pane, so the
    /// footer comes `S` below the last row, pinned only when the rows fill the column. `body`
    /// draws the rows, the empty state, or (narrow) the open event's form, as the pane's content;
    /// its column behaves as `scroll` says (the open event's form opens at its top: M0560).
    fn list_pane<R>(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        notes: &mut crate::sessions::page::Sessions,
        footer: PaneFooter<'_>,
        scroll: design::pane::ContentScroll,
        body: impl FnOnce(
            &mut Self,
            &mut egui::Ui,
            &mut Book,
            &mut crate::sessions::page::Sessions,
        ) -> R,
    ) -> R {
        let open = match footer {
            PaneFooter::Page(open) => open,
            PaneFooter::Empty => None,
        };
        let open = open.filter(|id| book.events.iter().any(|event| event.id == *id));
        let offer = open.and_then(|id| notes.event_link_offer(book, id));
        let frame = PaneFrame::new(ui);
        let messages = match footer {
            PaneFooter::Page(_) => {
                self.list_messages(&frame, ui, book, notes, open, offer.as_ref())
            }
            PaneFooter::Empty => {
                Messages::new(&frame, "timeline_detail_footer", ui.available_width())
            }
        };
        let paned = messages.pane_content(ui, scroll, |ui| body(self, ui, book, notes));
        if let Some(press) = paned.pressed {
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
        paned.body
    }

    /// The Timeline page's footer messages for this frame (`list_pane`).
    fn list_messages<'f>(
        &mut self,
        frame: &'f PaneFrame,
        ui: &egui::Ui,
        book: &Book,
        notes: &mut crate::sessions::page::Sessions,
        open: Option<&str>,
        offer: Option<&crate::sessions::page::LinkOffer>,
    ) -> Messages<'f, ListAct> {
        let mut messages = Messages::new(frame, "timeline_list_footer", ui.available_width());
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
                // The name it had when it was added, kept with the result.
                let label = shown.value().label.clone();
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

    /// The rows, in the list pane's scroll column. The column is as tall as every row, and only
    /// the rows in view draw, so a long timeline costs no more per frame than a short one.
    fn list(&mut self, ui: &mut egui::Ui, book: &mut Book, sessions: &[SessionLink]) {
        let view = self.view(book);
        let (flagged, days, rows, road) = (&view.flagged, &view.days, &view.rows, &view.road);
        if rows.is_empty() {
            // An unreadable file has its own message, and no filter caused it.
            if book.unreadable.is_none() {
                design::empty_state(
                    ui,
                    icons::TIMELINE,
                    "No matching events",
                    "Clear the search and filters to see the whole timeline.",
                    |ui| {
                        if design::link(ui, "Show all events").clicked() {
                            self.search.clear();
                            self.filter = Filter::All;
                            self.session = None;
                        }
                    },
                );
            }
            return;
        }
        let gap = ui.spacing().item_spacing.y;
        let step = ROW + gap;
        let width = ui.available_width();
        let (column, _) = ui.allocate_exact_size(
            egui::vec2(width, rows.len() as f32 * step - gap),
            egui::Sense::hover(),
        );
        let place = |row: usize| {
            egui::Rect::from_min_size(
                egui::pos2(column.left(), column.top() + row as f32 * step),
                egui::vec2(width, ROW),
            )
        };
        // The row before the target comes to the top, as the list opened before: the list's
        // column writes it as its own offset at its next pass (M0556, `reveal::jump`).
        if let Some(target) = self.scroll_to.take()
            && let Some(row) = rows.iter().position(|&index| index == target)
        {
            design::reveal::jump(
                ui,
                place(row.saturating_sub(1)).top(),
                design::reveal::JumpTo::ListedRow,
            );
        }
        let shown = ui.clip_rect();
        let first = ((shown.top() - column.top()) / step).floor().max(0.0) as usize;
        let last =
            (((shown.bottom() - column.top()) / step).ceil().max(0.0) as usize + 1).min(rows.len());
        for (row, &index) in rows.iter().enumerate().take(last).skip(first) {
            let event = &book.events[index];
            let session = event
                .session
                .and_then(|id| sessions.iter().find(|session| session.id == id))
                .map(|session| short_session(&session.label));
            let on_road = road.get(&event.id);
            // The row's ids come from its event, never from its place in view: when the list
            // scrolls, a focused row keeps its event (review of PR 334). egui makes a child's
            // ids from the parent's running count of children, which here is the slot in view;
            // a global scope gives the child exactly this id: the list's own and the event's key.
            let rect = place(row);
            let builder = egui::UiBuilder::new()
                .id_salt(ui.id().with(design::TextKey::of(event)))
                .global_scope(true)
                .max_rect(rect);
            ui.scope_builder(builder, |ui| {
                self.row(
                    ui,
                    event,
                    days[index],
                    session,
                    flagged.get(&event.id).copied(),
                    on_road.map(String::as_str),
                );
            });
        }
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn detail(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        sessions: &[SessionLink],
        notes: &mut crate::sessions::page::Sessions,
        narrow: bool,
    ) {
        let c = colors();
        let index = self
            .selected
            .as_ref()
            .and_then(|id| book.events.iter().position(|event| event.id == *id));
        checks::settle_length(book, index);
        let Some(index) = index else {
            let empty = |ui: &mut egui::Ui| {
                design::heading(ui, design::Heading::Panel("An event"));
                ui.add(
                    egui::Label::new(
                        Type::Small
                            .rich(
                                ui,
                                "Select an event to name it, place it or note what it means. Each event has its own link, for a session note or an Obsidian note.",
                            )
                            .color(c.muted),
                    )
                    .wrap(),
                );
            };
            if narrow {
                // The detail's own pane, never the list's: its short content would clamp the
                // list's saved place (review of #817).
                ui.push_id("timeline_detail_pane", |ui| {
                    self.list_pane(
                        ui,
                        book,
                        notes,
                        PaneFooter::Page(None),
                        design::pane::ContentScroll::default(),
                        |_, ui, _, _| empty(ui),
                    );
                });
            } else {
                empty(ui);
            }
            return;
        };
        let id = book.events[index].id.clone();
        let days = day_numbers(&book.events);
        let link = reference(&book.events[index], days[index]);
        let unsaved = book.event_unsaved(&id);
        let blocked = book
            .unreadable
            .as_deref()
            .or(book.error.as_deref())
            .or(if book.conflict {
                Some("Changed on disk. Choose what to keep above.")
            } else {
                None
            })
            .map(str::to_owned);
        let mut changed = false;
        let mut removed = false;
        let mut placed: Option<Option<String>> = None;
        let sources = self.tables.clone();
        let tables = sources.as_ref().map_or(&[][..], |sources| &sources.tables);
        let mut checks_act = None;
        let mut undo_checks = false;
        let mut length_taken = checks::Taken::Nothing;
        self.checks_done.follow(book);
        let labels: Vec<String> = book
            .events
            .iter()
            .zip(&days)
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
            .collect();
        let ids: Vec<String> = book.events.iter().map(|event| event.id.clone()).collect();
        let starts = clock(&book.events)[index];
        self.copied = self
            .copied
            .take()
            .filter(|(_, at)| at.elapsed() < COPIED_FOR);
        if self.copied.is_some() {
            ui.ctx().request_repaint_after(COPIED_FOR);
        }
        let event = &book.events[index];
        let title = event.label(days[index]);
        let kind = header_kind(event);
        // M0558 (M-P0.26): the header is a title line: the kind before the title (its icon alone
        // for a day or an untitled event, named by its word), Copy link, the save status in the
        // room of the widest of its four words, and the `⋯` menu at the line's right end.
        let saving = [
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
        ];
        let now = if blocked.is_some() {
            saving[if unsaved { 2 } else { 3 }]
        } else if unsaved {
            saving[1]
        } else {
            saving[0]
        };
        let copied = self
            .copied
            .as_ref()
            .is_some_and(|(copied, _)| *copied == id);
        let copy = if copied { COPIED } else { COPY_LINK };
        let more = format!("More actions for {}", event.label(days[index]));
        let width_id = ui
            .id()
            .with(("timeline_form_width", ui.available_width().to_bits()));
        let header_width = ui
            .ctx()
            .data(|data| data.get_temp::<f32>(width_id))
            .unwrap_or_else(|| {
                (ui.available_width() - ui.spacing().scroll.allocated_width())
                    .min(design::FORM_WIDTH)
            });
        let mut ruling = None;
        // M0560 (M-P0.30): another event, a link or a changed filter opens the form at its top.
        let from_top = std::mem::take(&mut self.reset_detail);
        // Round 27 FL1: the form leaves the scroll bar's lane free whether or not the bar shows,
        // so its fields and the header's `⋯` end at one x for every event.
        let lane_width = ui.available_width() - ui.spacing().scroll.allocated_width();
        // This journey's pending reveal, used once; another journey's is dropped.
        let reveal = self
            .reveal_checks
            .take()
            .filter(|(journey, _)| *journey == id)
            .map(|(_, at)| at);
        // Row 123: a check result's offer asks the plan of the journey's session.
        let journey_plan = checks::shown(&book.events[index])
            .then(|| notes.journey_plan(book, &id, &self.plan_names));
        // Review of #817 (P3): the header stays above the form only while it leaves the form at
        // least one control row; below that it scrolls with the form, so the form never has no
        // room (the header's height is the last draw's, at this width).
        let header_key = width_id.with("timeline_detail_header");
        // Review of #827 (P3): the pane's footer (its lines and the gap above them) counts too: the
        // last draw's pane height less its body's room.
        let footer_key = header_key.with("footer");
        let footer_room = ui
            .ctx()
            .data(|data| data.get_temp::<f32>(footer_key))
            .unwrap_or(0.0);
        let header_scrolls = ui
            .ctx()
            .data(|data| data.get_temp::<f32>(header_key))
            .is_some_and(|header| {
                ui.available_height() - header - footer_room < design::Density::Read.control()
            });
        // M0559 (M-P0.30): the header, then the form, which scrolls in the detail's own pane.
        let mut draw = |page: &mut Self, ui: &mut egui::Ui, book: &mut Book, part: DetailPart| {
            if part == DetailPart::Header {
                let top = ui.cursor().top();
                let event = &book.events[index];
                let mut copy_pressed = false;
                ui.allocate_ui_with_layout(
                    egui::vec2(header_width, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        let shown = design::TitleLine::new(&title)
                            .role(design::TitleType::Panel)
                            .kind(&kind, event.kind.word())
                            .actions_at_end()
                            .action(design::TitleAction::secondary(copy).icon(if copied {
                                icons::OK
                            } else {
                                icons::COPY
                            }))
                            .state(now, &saving)
                            .more(&more, |ui| {
                                if checks::offered(event, tables)
                                    && Action::quiet("Roll for encounters").show(ui).clicked()
                                {
                                    checks_act = Some(checks::Act::Start);
                                    ui.close();
                                }
                                let open_run = event.run.is_some() && event.run == page.open_run;
                                removed = Action::quiet("Remove")
                                    .name("Remove this event")
                                    .blocked_if(open_run, "This run is open. Close it first.")
                                    .show(ui)
                                    .clicked();
                                // A note under its choice, at its words (M-P0.26).
                                design::choice_note(
                                    ui,
                                    false,
                                    open_run.then_some("This run is open. Close it first."),
                                );
                                if removed {
                                    ui.close();
                                }
                            })
                            .show(ui);
                        copy_pressed = shown.clicked(copy);
                    },
                );
                if copy_pressed {
                    ui.ctx().copy_text(link.clone());
                    page.copied = Some((id.clone(), Instant::now()));
                }
                if let Some(reason) = &blocked {
                    ui.label(Type::Small.rich(ui, reason.as_str()).color(c.muted));
                }
                design::rule(ui);
                ui.scope(|ui| {
                    ui.set_max_width(header_width);
                    let event = &mut book.events[index];
                    design::gap(ui, design::Gap::XS);
                    let hint = (event.kind == Kind::Day)
                        .then_some("Optional. Days number themselves, so a name is extra.");
                    let title = EVENT_TITLE.of(design::TextKey::of(&*event));
                    let title =
                        design::labelled_field(ui, title, "Title", &mut event.title, hint, None);
                    changed |= title.changed();
                    if std::mem::take(&mut page.focus_title) {
                        title.request_focus();
                    }
                });
                design::gap(ui, design::Gap::S);
                design::rule(ui);
                let height = ui.cursor().top() - top;
                ui.ctx()
                    .data_mut(|data| data.insert_temp(header_key, height));
                return;
            }
            let form_width = lane_width.min(ui.available_width()).min(design::FORM_WIDTH);
            ui.ctx()
                .data_mut(|data| data.insert_temp(width_id, form_width));
            if (form_width - header_width).abs() > design::HAIRLINE_WIDTH {
                ui.ctx().request_repaint();
            }
            ui.set_max_width(form_width);
            let event = &mut book.events[index];
            let mut length = event.duration.clone();
            design::gap(ui, design::Gap::M);
            let day_start = starts
                .filter(|_| event.kind == Kind::Day)
                .map(|start| format!("Starts: {}", clock_words(start)));
            let kind_id = EVENT_KIND.of(design::TextKey::of(&*event));
            design::field(ui, "Kind", day_start.as_deref(), None, |ui| {
                let [first, rest @ ..] = Kind::ALL;
                changed |= design::Dropdown::new(kind_id, "Kind of event")
                    .choice(first, first.word())
                    .choices(rest.into_iter().map(|kind| (kind, kind.word())))
                    .show(ui, &mut event.kind)
                    .changed;
            });
            if event.kind != Kind::Day {
                design::gap(ui, design::Gap::M);
                design::field(ui, "How long", Some(super::LENGTH_HELP), None, |ui| {
                    let hint = match event.kind {
                        Kind::LongRest => "8 hours",
                        Kind::ShortRest => "1 hour",
                        _ => "",
                    };
                    let focus = std::mem::take(&mut page.focus_length);
                    if checks::shown(event) {
                        let (words, taken) =
                            checks::length_input(ui, &mut book.length, event, hint, focus);
                        length = words;
                        length_taken = taken;
                    } else {
                        // With no checks to follow it, the event changes as the DM types.
                        let width = ui.available_width();
                        changed |= design::line_field(
                            ui,
                            checks::length_field(event),
                            "How long",
                            &mut event.duration,
                            hint,
                            width,
                        )
                        .changed();
                    }
                });
            }
            ruling = rest_ruling_line(ui, event);
            if let Some(start) = starts.filter(|_| event.kind != Kind::Day) {
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        Type::Small
                            .rich(ui, format!("Starts: {}", clock_words(start)))
                            .color(c.muted),
                    );
                    // Round 26 FG4: a length the clock cannot count leaves Ends at Starts, so
                    // the warning takes Ends' place on the line that is always there.
                    if let Some(warning) = event.time_warning() {
                        ui.label(Type::Small.rich(ui, warning).color(c.result.warning))
                            .on_hover_text(super::TIME_REPAIR);
                    } else if event.kind != Kind::Day {
                        ui.label(
                            Type::Small
                                .rich(
                                    ui,
                                    format!(
                                        "Ends: {}",
                                        clock_words(start.saturating_add(event.minutes()))
                                    ),
                                )
                                .color(c.muted),
                        );
                    }
                });
            }
            // A journey's encounter checks, under its own **How long**.
            if checks::shown(event) {
                design::gap(ui, design::Gap::M);
                let view = checks::View {
                    event,
                    start: starts,
                    length: &length,
                    tables,
                    planned: &|name| journey_plan.as_ref().and_then(|plan| plan.get(name)),
                    reveal,
                };
                checks_act = checks::header(ui, &view, &mut page.checks).or(checks_act.take());
                // The act's result keeps its line under the table, so it moves no field.
                // M0561 (M-P0.27): a typed result line that keeps its room.
                let result = page.checks_done.get(&id).map(|done| {
                    design::outcome::PlacedResult::new(
                        done.words.clone(),
                        done.undo.clone().pressed_as(()),
                    )
                });
                undo_checks |= design::outcome::PlacedResult::show_line(result, ui).is_some();
                checks_act = checks::lines(ui, &view, &mut page.checks).or(checks_act.take());
            }
            design::gap(ui, design::Gap::M);
            let place = EVENT_PLACE.of(design::TextKey::of(&*event));
            changed |=
                design::labelled_field(ui, place, "Where", &mut event.place, None, None).changed();
            changed |= page.place_help(ui, event);
            design::gap(ui, design::Gap::M);
            let session_id = EVENT_SESSION.of(design::TextKey::of(&*event));
            design::field(ui, "Session", None, None, |ui| {
                changed |= design::Dropdown::new(session_id, "Session of this event")
                    .choice(None, "No session")
                    .choices(
                        sessions
                            .iter()
                            .map(|session| (Some(session.id), session.label.as_str())),
                    )
                    .show(ui, &mut event.session)
                    .changed;
            });
            // A session whose note has gone has nothing to open.
            if let Some(session) = event.session
                && sessions.iter().any(|known| known.id == session)
                && design::link(ui, "Open this session").clicked()
            {
                page.open_session = Some(session);
            }
            if !event.result.is_empty() {
                design::gap(ui, design::Gap::M);
                design::field(ui, "From the run", None, None, |ui| {
                    ui.label(
                        Type::Body
                            .rich(
                                ui,
                                super::played_fight_summary(
                                    event,
                                    event.run.is_some() && event.run == page.open_run,
                                ),
                            )
                            .color(c.text),
                    );
                });
            }
            design::gap(ui, design::Gap::M);
            design::field(ui, "Notes", None, None, |ui| {
                let width = ui.available_width() - 2.0 * design::S;
                let notes = EVENT_NOTES.of(design::TextKey::of(event));
                let response = notes.multi_line(ui, &mut event.notes, |edit| {
                    edit.desired_width(width)
                        .desired_rows(3)
                        .margin(egui::vec2(design::S, design::S))
                });
                design::named(ui, &response, "Notes on this event");
                changed |= response.changed();
            });
            for (label, value) in &event.extra {
                if let (Some(label), Some(value)) = (label.as_str(), value.as_str()) {
                    ui.label(
                        Type::Body
                            .rich(ui, format!("{label}: {value}"))
                            .color(c.muted),
                    );
                }
            }
            design::quiet_section(ui, "Link text", |ui| {
                ui.add(egui::Label::new(Type::Mono.rich(ui, &link).color(c.muted)).wrap());
            });
            let mentions: Vec<&SessionLink> = sessions
                .iter()
                .filter(|session| session.mentions.contains(&id))
                .collect();
            if !mentions.is_empty() {
                design::field(ui, "Mentioned in", None, None, |ui| {
                    for session in mentions {
                        if design::link(ui, &session.label).clicked() {
                            page.open_session = Some(session.id);
                        }
                    }
                });
            }
            design::gap(ui, design::Gap::XS);
            design::gap(ui, design::Gap::M);
            let place_after = PLACE_AFTER.of(design::TextKey::of(&*event));
            design::field(ui, "Place after", None, None, |ui| {
                // The event before this one; the list opens on it.
                let before = index.checked_sub(1).map(|at| ids[at].clone());
                let mut after = before.clone();
                design::Dropdown::new(place_after, "Place this event after")
                    .choice(None, "Start of the timeline")
                    .choices(
                        labels
                            .iter()
                            .enumerate()
                            .filter(|(at, _)| *at != index)
                            .map(|(at, label)| (Some(ids[at].clone()), label.as_str())),
                    )
                    .show(ui, &mut after);
                if after != before {
                    placed = Some(after);
                }
            });
        };
        // Narrow, the pane holds the open event's footer; wide, its results stay in the
        // list's footer, so the detail's pane has none. Its own pane either way, so the
        // list's column keeps its place for the way back.
        let footer = if narrow {
            PaneFooter::Page(Some(&id))
        } else {
            PaneFooter::Empty
        };
        if !header_scrolls {
            draw(self, ui, book, DetailPart::Header);
        }
        let pane_bottom = ui.max_rect().bottom();
        ui.push_id("timeline_detail_pane", |ui| {
            self.list_pane(
                ui,
                book,
                notes,
                footer,
                design::pane::ContentScroll::default().from_top(from_top),
                |page, ui, book, _| {
                    if let Some(body) = design::pane::body_room(ui) {
                        let room = (pane_bottom - body.bottom()).max(0.0);
                        ui.ctx().data_mut(|data| data.insert_temp(footer_key, room));
                    }
                    if header_scrolls {
                        draw(page, ui, book, DetailPart::Header);
                    }
                    draw(page, ui, book, DetailPart::Form);
                },
            );
        });
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

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        campaign_id: Uuid,
        sessions: &[SessionLink],
        current: Option<Uuid>,
        notes: &mut crate::sessions::page::Sessions,
    ) {
        let page_width = ui.available_width();
        // Only a real change: the first show can already have an event to reveal.
        if self
            .campaign
            .is_some_and(|campaign| campaign != campaign_id)
        {
            // The app set `modal` for this frame before the page drew: the reset keeps it, so
            // Ctrl+S waits under a modal on the first frame after a campaign change too.
            *self = Self {
                open_run: self.open_run,
                saved_runs: std::mem::take(&mut self.saved_runs),
                saved_runs_revision: self.saved_runs_revision,
                modal: self.modal,
                ..Default::default()
            };
        }
        self.campaign = Some(campaign_id);
        self.settle_added(ui.ctx(), book);
        // A pending reveal is its journey's: a changed selection drops it before any draw.
        if self
            .reveal_checks
            .as_ref()
            .is_some_and(|(journey, _)| self.selected.as_deref() != Some(journey.as_str()))
        {
            self.reveal_checks = None;
        }
        // M0562: the header is the shared title line: the title, then the count of events (and
        // how many show) and the clock, once a day sets it.
        let context = if book.events.is_empty() {
            String::new()
        } else {
            // The clock shows once a day sets it; before that, there is nothing to name.
            let view = self.view(book);
            let now = view
                .now
                .map_or_else(String::new, |now| format!(" · Now: {}", clock_words(now)));
            format!(
                "{}{now}",
                if self.filter != Filter::All
                    || !self.search.trim().is_empty()
                    || self.session.is_some()
                {
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
        };
        let _ = design::TitleLine::new("Timeline")
            .context((!context.is_empty()).then_some(context.as_str()))
            .page()
            .show(ui);
        if problems(ui, book) {
            return;
        }
        // Ctrl+S is taken here, before the fields, and saves after they took this frame's
        // input (issue #513): words typed in the same frame are saved with it. Under a modal it
        // waits, as every shortcut does.
        let save = design::save_row::save_key(ui, !self.modal);
        // The page under its header and the file's problems: the quick add, the list and the
        // open event. An early end leaves only this closure, so the save after it always runs.
        let mut body = || {
            if let Some(id) = quick_add(
                ui,
                book,
                current,
                &[
                    Kind::Encounter,
                    Kind::Day,
                    Kind::LongRest,
                    Kind::ShortRest,
                    Kind::Travel,
                    Kind::Event,
                ],
                |_| {},
            ) {
                self.reveal(&id, book);
                self.focus_title = true;
                self.added.record_timed(JustAdded::of(book, id), ui.ctx());
            }
            if let Some(words) = self.dated.clone() {
                self.play_current |= dated_notice(ui, &words);
            }
            design::gap(ui, design::Gap::XS);
            if book.events.is_empty() && book.unreadable.is_none() {
                let open_run = self.open_run.is_some();
                let first = self.list_pane(ui, book, notes, PaneFooter::Page(None), design::pane::ContentScroll::default(), |_, ui, _, _| {
                let mut first = false;
                design::empty_state(
                    ui,
                    icons::TIMELINE,
                    "Nothing on the timeline yet",
                    "Add the first day above. Each encounter run appears here by itself, in the session it belongs to.",
                    |ui| {
                        first = if open_run {
                            Action::secondary("Start with day 1")
                        } else {
                            Action::primary("Start with day 1")
                        }
                        .show(ui)
                        .clicked();
                    },
                );
                first
            });
                if first {
                    let id = book.add(Kind::Day, current);
                    self.reveal(&id, book);
                    self.focus_title = true;
                    self.added.record_timed(JustAdded::of(book, id), ui.ctx());
                }
                return;
            }
            if !std::mem::replace(&mut self.opened, true) && self.scroll_to.is_none() {
                self.scroll_to = book.events.len().checked_sub(1);
            }
            // One column, with an event open: the filters would only push it down.
            let narrow = page_width < TWO_COLUMNS_FROM;
            self.narrow = narrow;
            let event_open = self
                .selected
                .as_ref()
                .is_some_and(|id| book.events.iter().any(|event| event.id == *id));
            if !(narrow && event_open) {
                // The header is drawn before the body splits. Use the app's current width
                // decision, including pending keyboard and pointer resize, rather than the
                // previous pass's panel.
                let mut room = ui.clip_rect();
                // The header knows its current clip, not the list footer drawn later. Long
                // menus keep the part's height cap and inner scroll, as the old ComboBox did.
                if !narrow && event_open {
                    let natural = design::side_panel_width(page_width);
                    let content_width = page_width.min(design::LIST_MEASURE + natural);
                    let detail = design::Column::panel("timeline_detail_panel", natural)
                        .width_within(
                            ui.ctx(),
                            egui::Rect::from_min_size(
                                ui.max_rect().min,
                                egui::vec2(content_width, ui.available_height()),
                            ),
                        );
                    room.max.x = room.right().min(
                        ui.max_rect().left() + content_width
                            - detail
                            - ui.style().interaction.resize_grab_radius_side,
                    );
                }
                self.filters(ui, sessions, current, room);
            }
            design::gap(ui, design::Gap::XS);
            // Beside Lookup or Notes at a small window, two columns would crush the list.
            if narrow {
                if event_open {
                    if design::TitleLine::back(crate::word!("All events"), design::BackEdge::Text)
                        .show(ui)
                        .clicked()
                    {
                        // Review of #817 (P2): the list comes in the next pass, at its place;
                        // this pass draws no empty detail.
                        self.select(None);
                        ui.ctx().request_discard("timeline: back to all events");
                        ui.ctx().request_repaint();
                        return;
                    }
                    // With the list hidden, the open event's results are in the form's own footer,
                    // so they move no field.
                    notes.expire_event_link_undo(true, book);
                    // M0559 (M-P0.30): the form's header, then its pane.
                    self.detail(ui, book, sessions, notes, true);
                } else {
                    notes.expire_event_link_undo(true, book);
                    self.list_pane(
                        ui,
                        book,
                        notes,
                        PaneFooter::Page(None),
                        design::pane::ContentScroll::default(),
                        |page, ui, book, _| {
                            page.list(ui, book, sessions);
                        },
                    );
                }
                return;
            }
            if !event_open {
                self.list_pane(
                    ui,
                    book,
                    notes,
                    PaneFooter::Page(None),
                    design::pane::ContentScroll::default(),
                    |page, ui, book, _| {
                        page.list(ui, book, sessions);
                    },
                );
                return;
            }
            let width = design::side_panel_width(ui.available_width());
            let content_width = ui.available_width().min(design::LIST_MEASURE + width);
            ui.allocate_ui_with_layout(
                egui::vec2(content_width, ui.available_height()),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    design::Column::panel("timeline_detail_panel", width)
                        .frame(egui::Frame::new().inner_margin(egui::Margin {
                            left: design::S as i8,
                            ..Default::default()
                        }))
                        .show_inside(ui, |ui| self.detail(ui, book, sessions, notes, false));
                    egui::CentralPanel::default()
                        // The panel's drag handle lies on this edge, so the rows stop short of it.
                        .frame(egui::Frame::new().inner_margin(egui::Margin {
                            right: design::S as i8,
                            ..Default::default()
                        }))
                        .show_inside(ui, |ui| {
                            notes.expire_event_link_undo(true, book);
                            let open = self.selected.clone();
                            self.list_pane(
                                ui,
                                book,
                                notes,
                                PaneFooter::Page(open.as_deref()),
                                design::pane::ContentScroll::default(),
                                |page, ui, book, _| {
                                    page.list(ui, book, sessions);
                                },
                            );
                        });
                },
            );
        };
        body();
        if save {
            book.save();
        }
    }
}

/// The session's number alone. The row's own title is the event's.
fn short_session(label: &str) -> &str {
    label.split(':').next().unwrap_or(label)
}

#[cfg(test)]
pub(crate) fn check_long_rest_add(kinds: &'static [Kind]) {
    use crate::design::test_harness::Themed as _;
    use egui_kittest::kittest::Queryable;
    let session = Some(Uuid::new_v4());
    let mut harness = egui_kittest::Harness::builder().themed_state(
        move |ctx, book: &mut Book| {
            egui::CentralPanel::default().show(ctx, |ui| {
                quick_add(ui, book, session, kinds, |_| {});
            });
        },
        Book::default(),
    );
    harness.run_steps(2);
    harness.get_by_label("Add Long rest").click();
    harness.run_steps(2);
    assert_eq!(harness.state().events.len(), 1);
    assert!(super::party_state(&harness.state().events, session).rested);
    let id = harness.state().events[0].id.clone();
    harness.state_mut().remove(&id);
    assert!(!super::party_state(&harness.state().events, session).rested);
}

/// A row's second line, from the byte `from` of `detail` (the open row's words after its run
/// link start there): muted, with each status part (the rest ruling, a length the clock cannot
/// count) in its own colour. `spans` are byte ranges in `detail`, in order.
fn second_line(
    ui: &egui::Ui,
    detail: &str,
    from: usize,
    spans: &[(std::ops::Range<usize>, egui::Color32)],
    width: f32,
) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::default();
    let mut at = from.min(detail.len());
    let mut append = |words: &str, color| {
        job.append(
            &words.replace(['\n', '\r'], " "),
            0.0,
            egui::TextFormat::simple(Type::Small.font(ui), color),
        );
    };
    for (range, color) in spans {
        let (start, end) = (
            range.start.max(at).min(detail.len()),
            range.end.min(detail.len()),
        );
        if start >= end {
            continue;
        }
        append(&detail[at..start], colors().muted);
        append(&detail[start..end], *color);
        at = end;
    }
    append(&detail[at..], colors().muted);
    job.wrap.max_width = width.max(1.0);
    job.wrap.max_rows = 1;
    job.wrap.break_anywhere = true;
    ui.fonts_mut(|fonts| fonts.layout_job(job))
}

/// A part of the open event's detail (`TimelinePage::detail`): its header (the title line and
/// the Title field) or its form.
#[derive(Clone, Copy, PartialEq, Eq)]
enum DetailPart {
    Header,
    Form,
}

/// What a Timeline pane's footer holds (`TimelinePage::list_pane`).
#[derive(Clone, Copy)]
enum PaneFooter<'a> {
    /// The page's messages, with the open event's (`list_messages`).
    Page(Option<&'a str>),
    /// None: the wide detail's form, whose results are in the list's footer (M0559).
    Empty,
}

/// The Copy link action's words, before and after a copy (M0558).
const COPY_LINK: design::Word = crate::word!("Copy link");
const COPIED: design::Word = crate::word!("Copied");

fn header_kind(event: &Event) -> String {
    if event.kind == Kind::Day || event.own_title().is_none() {
        kind_icon(event.kind).to_owned()
    } else {
        format!("{} {}", kind_icon(event.kind), event.kind.word())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Issue #513: Ctrl+S is taken before the fields and saves after them, so words typed in
    /// ONE input frame with it are saved with it. Under a modal it saves nothing.
    #[test]
    fn ctrl_s_saves_the_words_typed_in_its_frame() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let mut book = Book::default();
        book.open(Some(dir.path()));
        let id = book.add(Kind::Event, None);
        book.events[0].title = "Toll".into();
        book.save();
        let mut page = TimelinePage::default();
        page.reveal(&id, &book);
        type State = (TimelinePage, Book, crate::sessions::page::Sessions);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(1200.0, 800.0))
            .themed_state(
                |ctx, (page, book, notes): &mut State| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        page.show(ui, book, Uuid::from_u128(1), &[], None, notes);
                    });
                },
                (page, book, crate::sessions::page::Sessions::default()),
            );
        harness.run_steps(2);
        harness
            .get_by_role_and_label(egui::accesskit::Role::TextInput, "Title")
            .focus();
        harness.run_steps(2);
        let on_disk = |harness: &egui_kittest::Harness<'_, State>| {
            std::fs::read_to_string(harness.state().1.path().unwrap()).unwrap()
        };
        let ctrl_s = egui::Event::Key {
            key: egui::Key::S,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        };
        // Typing and Ctrl+S in one frame's raw input (kittest gives each `event` a frame).
        let type_and_save = |harness: &mut egui_kittest::Harness<'_, State>, words: &str| {
            harness
                .input_mut()
                .events
                .extend([egui::Event::Text(words.into()), ctrl_s.clone()]);
            harness.step();
        };
        harness.state_mut().0.modal = true;
        type_and_save(&mut harness, " gate");
        assert_eq!(harness.state().1.events[0].title, "Toll gate");
        assert!(
            !on_disk(&harness).contains("Toll gate"),
            "under a modal, Ctrl+S saves nothing"
        );
        harness.state_mut().0.modal = false;
        type_and_save(&mut harness, "s");
        assert!(
            on_disk(&harness).contains("Toll gates"),
            "the words typed in Ctrl+S's frame are saved with it: {}",
            on_disk(&harness)
        );
        // Review of PR 640 (P2): on the first frame after a campaign change the page resets
        // itself, and it keeps the app's modal state, so Ctrl+S still waits under a modal.
        let state = harness.state_mut();
        state.1.events[0].title = "Moved camp".into();
        state.1.touch();
        state.0.campaign = Some(Uuid::from_u128(2));
        state.0.modal = true;
        harness.input_mut().events.push(ctrl_s.clone());
        harness.step();
        assert!(
            !on_disk(&harness).contains("Moved camp"),
            "under a modal, Ctrl+S saves nothing on the campaign change's frame"
        );
    }

    /// M0552: the open row's words after its run link are fitted by the closed row's own line
    /// (`second_line` from the link's end): cut with an ellipsis at the row's end, a status part
    /// in its own colour (the raw label's were all muted), and the whole words in the line's text.
    #[test]
    fn the_open_rows_words_after_its_link_fit_as_a_closed_rows() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let mut seen = None;
        // New fonts apply from the next frame.
        for _ in 0..2 {
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let detail = "Session 4 · 3 rounds · Time not counted";
                    let warning = detail.find("Time").unwrap();
                    let spans = [(warning..detail.len(), colors().result.warning)];
                    // After a link that ends at "rounds".
                    let after = detail.find(" · Time").unwrap();
                    let whole = second_line(ui, detail, after, &spans, f32::INFINITY);
                    let cut = second_line(ui, detail, after, &spans, whole.size().x / 2.0);
                    let shown = |galley: &egui::Galley| -> String {
                        galley.rows[0]
                            .glyphs
                            .iter()
                            .map(|glyph| glyph.chr)
                            .collect()
                    };
                    let warned = whole
                        .job
                        .sections
                        .iter()
                        .filter(|section| section.format.color == colors().result.warning)
                        .map(|section| &whole.job.text[section.byte_range.clone()])
                        .collect::<String>();
                    seen = Some((shown(&whole), shown(&cut), cut.job.text.clone(), warned));
                });
            });
        }
        let (whole, cut, text, warned) = seen.unwrap();
        assert_eq!(
            whole, " · Time not counted",
            "the line starts at the link's end"
        );
        assert_eq!(warned, "Time not counted", "the warning keeps its colour");
        assert!(
            cut.ends_with('…') && cut.len() < whole.len(),
            "cut to the row: {cut:?}"
        );
        assert_eq!(text, whole, "the line's text keeps the whole words");
    }

    /// M0556: the list opens at its last event: the list's column writes the jump
    /// (`reveal::jump`), so the last event is painted whole inside the column's clip and the
    /// first one is not drawn (only the rows in view draw). At a wide window (two columns' width)
    /// and at a small one. The theme is the app's before the page's first frame: the harness draws
    /// until the page settles before the test's own code runs, and a jump measured at egui's
    /// default sizes is clamped short of the end once the theme's sizes apply.
    #[test]
    fn the_list_opens_at_its_last_event() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        for size in [egui::vec2(1200.0, 600.0), egui::vec2(960.0, 640.0)] {
            let dir = tempfile::tempdir().unwrap();
            let mut book = Book::default();
            book.open(Some(dir.path()));
            for at in 0..60 {
                book.add(Kind::Event, None);
                book.events.last_mut().unwrap().title = format!("Stop {at:02}");
            }
            book.save();
            type State = (TimelinePage, Book, crate::sessions::page::Sessions);
            let mut harness = egui_kittest::Harness::builder()
                .with_size(size)
                // The theme is installed in the harness's first frame; the page draws from the
                // next (issue #810, `design::test_harness`).
                .themed_state(
                    |ctx, (page, book, notes): &mut State| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            page.show(ui, book, Uuid::from_u128(1), &[], None, notes);
                        });
                    },
                    (
                        TimelinePage::default(),
                        book,
                        crate::sessions::page::Sessions::default(),
                    ),
                );
            harness.run_steps(6);
            // The column's clip ends at the panel's content edge (the list has no footer).
            let bottom = size.y
                - f32::from(
                    egui::Frame::central_panel(&harness.ctx.style())
                        .inner_margin
                        .bottom,
                );
            let rect = |words: &str| {
                harness
                    .query_all_by_label_contains(words)
                    .next()
                    .map(|node| node.rect())
            };
            let last =
                rect("Stop 59").unwrap_or_else(|| panic!("{size:?}: the last event is drawn"));
            let before =
                rect("Stop 58").unwrap_or_else(|| panic!("{size:?}: the event before it is drawn"));
            assert!(
                last.bottom() <= bottom + 0.5 && last.top() > before.bottom(),
                "{size:?}: the last event {last:?} is whole above the clip's end {bottom}"
            );
            assert!(
                rect("Stop 00").is_none(),
                "{size:?}: the list moved: the first event is out of view"
            );
        }
    }

    /// M0559, M0560 (M-P0.30): the open event's form scrolls in the detail's own pane, under its
    /// header, which stays; and an event opened again opens its form at the top. Wide (the
    /// detail's panel), then narrow (the form is the pane's content, its header drawn before the
    /// pane).
    #[test]
    fn the_detail_form_scrolls_under_its_header() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::{NodeT, Queryable};
        let dir = tempfile::tempdir().unwrap();
        let mut book = Book::default();
        book.open(Some(dir.path()));
        for at in 0..60 {
            book.add(Kind::Event, None);
            book.events.last_mut().unwrap().title = format!("Stop {at:02}");
        }
        book.save();
        type State = (TimelinePage, Book, crate::sessions::page::Sessions);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(1200.0, 600.0))
            .themed_state(
                |ctx, (page, book, notes): &mut State| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        page.show(ui, book, Uuid::from_u128(1), &[], None, notes);
                    });
                },
                (
                    TimelinePage::default(),
                    book,
                    crate::sessions::page::Sessions::default(),
                ),
            );
        harness.run_steps(6);
        // M0559, M0560 (M-P0.30, `m-p0-detail-scroll`): the open event's form scrolls in the
        // detail's own column, under its header, which stays; and an event opened again opens
        // its form at the top. Wide (the detail's panel), then narrow (the form is the pane's
        // content, its header drawn before the pane).
        let ids: Vec<String> = harness
            .state()
            .1
            .events
            .iter()
            .map(|event| event.id.clone())
            .collect();
        let open = |harness: &mut egui_kittest::Harness<'_, State>, at: usize| {
            let (page, book, _) = harness.state_mut();
            page.reveal(&ids[at], book);
            harness.run_steps(4);
        };
        let top = |harness: &egui_kittest::Harness<'_, State>, role, name: &str| {
            harness.get_by_role_and_label(role, name).rect().top()
        };
        let title = |harness: &egui_kittest::Harness<'_, State>| {
            top(harness, egui::accesskit::Role::TextInput, "Title")
        };
        let kind = |harness: &egui_kittest::Harness<'_, State>| {
            harness
                .query_by_label_contains("Kind of event: ")
                .expect("the form's Kind")
                .rect()
        };
        let wheel = |harness: &mut egui_kittest::Harness<'_, State>, at: egui::Pos2| {
            harness.event(egui::Event::PointerMoved(at));
            harness.event(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, -200.0),
                modifiers: egui::Modifiers::NONE,
            });
            harness.run_steps(4);
        };
        for (size, case) in [(1200.0, "wide"), (600.0, "narrow")] {
            harness.set_size(egui::vec2(size, 600.0));
            open(&mut harness, 57);
            let (opened_title, opened) = (title(&harness), kind(&harness));
            wheel(&mut harness, opened.center());
            assert!(
                (title(&harness) - opened_title).abs() < 0.5,
                "m-p0-detail-scroll ({case}): the header stays ({opened_title} -> {})",
                title(&harness)
            );
            assert!(
                kind(&harness).top() < opened.top() - 50.0,
                "m-p0-detail-scroll ({case}): the form scrolls under it ({} -> {})",
                opened.top(),
                kind(&harness).top()
            );
            if case == "wide" {
                // Review of #817 (P3, `m-p0-search-keeps-form`): a search that hides the open
                // event's row changes the list only; its form keeps its place.
                let scrolled = kind(&harness).top();
                harness
                    .get_by_role_and_label(egui::accesskit::Role::TextInput, "Search the timeline")
                    .focus();
                harness.run_steps(2);
                harness
                    .input_mut()
                    .events
                    .push(egui::Event::Text("Stop 1".into()));
                harness.run_steps(4);
                assert_eq!(harness.state().0.search, "Stop 1");
                assert!(
                    (kind(&harness).top() - scrolled).abs() < 0.5,
                    "m-p0-search-keeps-form: the form keeps its place ({scrolled} -> {})",
                    kind(&harness).top()
                );
            }
            // Another event, then this one again: each opens its form at the top.
            open(&mut harness, 56);
            open(&mut harness, 57);
            assert!(
                (kind(&harness).top() - opened.top()).abs() < 0.5,
                "m-p0-detail-scroll ({case}): opened again, the form is at its top ({} -> {})",
                opened.top(),
                kind(&harness).top()
            );
        }
        // Review of #817 (P2, `m-p0-back-keeps-list`): narrow, an event opened from a row far
        // down the list, then All events: the list is back at its place.
        let row = |harness: &egui_kittest::Harness<'_, State>, name: &str| {
            harness
                .query_all_by_label_contains(name)
                .find(|node| node.accesskit_node().role() == egui::accesskit::Role::Button)
                .map(|node| node.rect())
        };
        harness.get_by_label("All events").click();
        harness.run_steps(4);
        let place = row(&harness, "Stop 55").expect("the row of Stop 55, in view");
        harness
            .query_all_by_label_contains("Stop 55")
            .find(|node| node.accesskit_node().role() == egui::accesskit::Role::Button)
            .expect("the row of Stop 55")
            .click();
        harness.run_steps(4);
        assert_eq!(harness.state().0.selected.as_ref(), Some(&ids[55]));
        harness.get_by_label("All events").click();
        harness.run_steps(4);
        assert_eq!(
            row(&harness, "Stop 55"),
            Some(place),
            "m-p0-back-keeps-list: the list is back at its place"
        );
        // Review of #817 (P3, `m-p0-short-detail`): a window so short that the header would
        // leave the form no room. The header then scrolls with the form, so the form can be
        // reached: a wheel over the detail brings its Kind into view.
        let short = egui::vec2(1200.0, 300.0);
        harness.set_size(short);
        open(&mut harness, 57);
        // Small wheel steps until the Kind is whole in the window, or 30 steps.
        let inside = |rect: egui::Rect| rect.top() >= 0.0 && rect.bottom() <= short.y;
        let mut steps = 0;
        while !inside(kind(&harness)) && steps < 30 {
            harness.event(egui::Event::PointerMoved(egui::pos2(
                short.x - 150.0,
                short.y - 40.0,
            )));
            harness.event(egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta: egui::vec2(0.0, -20.0),
                modifiers: egui::Modifiers::NONE,
            });
            harness.run_steps(2);
            steps += 1;
        }
        let reached = kind(&harness);
        assert!(
            inside(reached),
            "m-p0-short-detail: the form's Kind comes into the {short:?} window: {reached:?}"
        );
        // Review of #827 (P3, `m-p0-short-detail-footer`): narrow, with the Add result and its
        // Undo in the detail's footer. At every height of the sweep, the header and the footer
        // leave the form room, or the header scrolls with the form: wheel steps over the form
        // bring its Kind whole into view above the footer.
        harness.set_size(egui::vec2(600.0, 600.0));
        harness.run_steps(4);
        harness.get_by_label("All events").click();
        harness.run_steps(4);
        harness.get_by_label("Add Major event").click();
        harness.run_steps(4);
        let undo = |harness: &egui_kittest::Harness<'_, State>| {
            harness
                .query_by_label("Undo add Major event")
                .map(|node| node.rect())
        };
        assert!(undo(&harness).is_some(), "the footer holds the Add result");
        for height in (260..=420).step_by(8) {
            let size = egui::vec2(600.0, height as f32);
            harness.set_size(size);
            harness.run_steps(4);
            let footer = undo(&harness)
                .unwrap_or_else(|| panic!("{size:?}: the footer holds the Add result"))
                .top();
            // Whole in the form's room: under the back line and the header (the Title field
            // ends it, also when the header scrolls with the form) and above the footer.
            let title = |harness: &egui_kittest::Harness<'_, State>| {
                harness
                    .get_by_role_and_label(egui::accesskit::Role::TextInput, "Title")
                    .rect()
                    .bottom()
            };
            let back = harness.get_by_label("All events").rect().bottom();
            let whole = |harness: &egui_kittest::Harness<'_, State>| {
                let rect = kind(harness);
                rect.top() >= title(harness).max(back) && rect.bottom() <= footer
            };
            let mut steps = 0;
            while !whole(&harness) && steps < 80 {
                // Over the form's room, towards the Kind: under the back line when the header
                // scrolls with the form, under the Title when it stays.
                let top = if steps % 2 == 0 {
                    back
                } else {
                    title(&harness).max(back)
                };
                let at = egui::pos2(size.x / 2.0, (top.min(footer) + footer) / 2.0);
                let by = if kind(&harness).top() < title(&harness).max(back) {
                    20.0
                } else {
                    -20.0
                };
                harness.event(egui::Event::PointerMoved(at));
                harness.event(egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, by),
                    modifiers: egui::Modifiers::NONE,
                });
                harness.run_steps(2);
                steps += 1;
            }
            let reached = kind(&harness);
            assert!(
                whole(&harness),
                "m-p0-short-detail-footer: at {size:?} the form's Kind comes whole above the \
                 footer at {footer}: {reached:?}"
            );
        }
    }

    /// Round 30 B7d, ruling L9: the Add result ends in the page's owner update
    /// (`settle_added`), before the footer presents its kept line: by the frame clock (the
    /// pane drawn unchanged across the end, with no input: until it, a repaint is asked for
    /// at the time left; at it, one frame drops the words, `Undo add` and the footer's room);
    /// when a reload of the book lost its event (A gone, B unchanged; an edit that keeps it is
    /// the control); and at once when another event is opened.
    #[test]
    fn the_add_result_ends_by_the_frame_clock() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let mut book = Book::default();
        book.open(Some(dir.path()));
        let b = book.add(Kind::Event, None);
        book.events[0].title = "Toll gate".into();
        book.save();
        type State = (TimelinePage, Book, crate::sessions::page::Sessions);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(1200.0, 800.0))
            .themed_state(
                |ctx, (page, book, notes): &mut State| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        page.show(ui, book, Uuid::from_u128(1), &[], None, notes);
                    });
                },
                (
                    TimelinePage::default(),
                    book,
                    crate::sessions::page::Sessions::default(),
                ),
            );
        harness.run_steps(2);
        let footer = |harness: &egui_kittest::Harness<'_, State>| {
            design::pane::footer_rect(&harness.ctx, egui::Id::new("timeline_list_footer"))
        };
        let repaint = |harness: &egui_kittest::Harness<'_, State>| {
            harness.output().viewport_output[&egui::ViewportId::ROOT].repaint_delay
        };
        let shows = |harness: &egui_kittest::Harness<'_, State>| {
            (
                harness.query_by_label("Undo add Major event").is_some(),
                harness
                    .query_all_by_label_contains(" added.")
                    .next()
                    .is_some(),
            )
        };
        let add = |harness: &mut egui_kittest::Harness<'_, State>, at: f64| {
            crate::frame_clock::set(at);
            harness.get_by_label("Add Major event").click();
            harness.run_steps(3);
            assert_eq!(
                shows(harness),
                (true, true),
                "the add says so, with its Undo"
            );
            harness
                .state()
                .0
                .selected
                .clone()
                .expect("the new event is open")
        };
        let a = add(&mut harness, 100.0);
        let start = crate::design::results::projection_work();
        // egui asks for the repaint one predicted frame early, so it lands at the end.
        let dt = f64::from(harness.ctx.input(|input| input.predicted_dt));
        for (at, left) in [(129.0, 1.0), (129.5, 0.5)] {
            crate::frame_clock::set(at);
            harness.step();
            assert_eq!(shows(&harness), (true, true), "{at}: it stands");
            let delay = repaint(&harness).as_secs_f64();
            assert!(
                (delay - (left - dt)).abs() < 1e-3,
                "{at}: a repaint is asked for at the end, {left} s on, not {delay}"
            );
        }
        let held = footer(&harness).expect("the result's footer is drawn");
        let quiet = crate::design::results::projection_work();
        assert_eq!(
            (quiet.built - start.built, quiet.bytes - start.bytes),
            (0, 0),
            "frames with no change build the kept line no more"
        );
        crate::frame_clock::set(130.0);
        harness.step();
        assert_eq!(
            shows(&harness),
            (false, false),
            "30 s of frames later the words and the Undo are gone in one frame"
        );
        assert!(
            footer(&harness).is_none_or(|now| (now.height() - held.height()).abs() > 1.0),
            "in that frame the footer gives its room back: {held:?} -> {:?}",
            footer(&harness)
        );
        // F2 of the early review: a reload of the book that lost the added event ends its
        // result on the first frame that shows the reload; an edit that keeps it does not.
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
        let a2 = add(&mut harness, 200.0);
        // Review of PR 901: a rename while the result shows changes neither its words nor its
        // Undo: the result keeps the name the event had when it was added.
        let at = harness
            .state()
            .1
            .events
            .iter()
            .position(|event| event.id == a2)
            .unwrap();
        harness.state_mut().1.events[at].title = "Renamed stop".into();
        harness.state_mut().1.touch();
        harness.step();
        assert!(
            harness.query_by_label("Undo add Major event").is_some()
                && harness.query_by_label("Undo add Renamed stop").is_none()
                && harness
                    .query_all_by_label_contains("Major event added.")
                    .next()
                    .is_some(),
            "the result keeps the added event's name"
        );
        harness.state_mut().1.save();
        edit_on_disk(&|_| true, Some("Toll bridge"));
        assert!(harness.state_mut().1.read_if_changed());
        harness.step();
        assert_eq!(
            shows(&harness),
            (true, true),
            "an edit on disk that keeps the added event keeps its result"
        );
        let a2_id = a2.clone();
        edit_on_disk(&move |event| event.id != a2_id, None);
        assert!(harness.state_mut().1.read_if_changed());
        harness.step();
        assert_eq!(
            shows(&harness),
            (false, false),
            "a reload that lost the added event ends its result on the first frame"
        );
        assert!(
            harness.state().1.events.iter().any(|event| event.id == b)
                && harness.query_by_label_contains("Toll bridge").is_some(),
            "the unrelated event B stays"
        );
        assert_ne!(a, a2);
        // Another event opened ends it at once, in the frame of the press.
        let a3 = add(&mut harness, 300.0);
        harness.state_mut().0.select(Some(b.clone()));
        harness.step();
        assert_eq!(shows(&harness), (false, false), "another selection ends it");
        assert!(!harness.state().1.events.iter().all(|event| event.id != a3));
        // M0548: the reload result's Undo names its file as the app does everywhere, with its
        // extension, as World's repair form names an unreadable note.
        harness.state_mut().1.reload_keeping_undo();
        harness.run_steps(2);
        assert!(
            harness.query_by_label("Undo reload Timeline.md").is_some(),
            "the reload's Undo names its file"
        );
    }

    #[test]
    fn iter3_c_closed_long_rest_rows_show_both_rulings() {
        use crate::design::test_harness::ThemedPage as _;
        use egui_kittest::kittest::Queryable;
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
            let expected = format!("{} · {icon} {word}", event.label(0));
            let mut harness = egui_kittest::Harness::builder().themed(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    TimelinePage::default().row(ui, &event, 0, None, None, None);
                });
            });
            harness.run_steps(2);
            harness.get_by_label(&expected);
        }
    }

    #[test]
    fn iter1_c_day_appears_only_when_filters_hide_day_headings() {
        use crate::design::test_harness::ThemedPage as _;
        use egui_kittest::kittest::Queryable;
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
            let mut harness = egui_kittest::Harness::builder().themed(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut page = TimelinePage::default();
                    if filtered {
                        page.filter = Filter::Rests;
                    }
                    page.row(ui, &event, 1, None, None, None);
                });
            });
            harness.run_steps(2);
            harness.get_by_label(&expected);
        }
    }

    #[test]
    fn iter1_c_rest_ruling_actions_name_the_change_and_keep_undo() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        let mut book = Book::default();
        book.add(Kind::LongRest, None);
        book.events[0].restores = true;
        let event_id = book.events[0].id.clone();
        // The ruling line in the row, and its result in the pane's footer, as the pages draw them.
        let mut harness = egui_kittest::Harness::builder().themed_state(
            move |ctx, book: &mut Book| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let frame = PaneFrame::new(ui);
                    let mut messages = Messages::new(&frame, "ruling_test", ui.available_width());
                    ruling_line(&frame, &mut messages, book, &event_id, ());
                    let mut action = None;
                    let paned = messages.pane(ui, design::Body::Content, |ui| {
                        action = rest_ruling(ui, &book.events[0]);
                    });
                    if let Some(action) = action {
                        action.apply(book, &event_id);
                    }
                    if let Some(ticket) = paned.pressed.and_then(|press| press.ticket) {
                        book.undo_restores(ticket);
                    }
                });
            },
            book,
        );
        harness.run_steps(2);
        harness.get_by_label("Mark not at full").click();
        harness.run_steps(2);
        assert!(!harness.state().events[0].restores);
        // iter-23-S FL1: each direction says the ruling now in force.
        assert!(
            harness
                .query_all_by_label("The next fight starts from the HP the party has now.")
                .next()
                .is_some()
        );
        harness.get_by_label("Mark everyone at full").click();
        harness.run_steps(2);
        assert!(harness.state().events[0].restores);
        assert!(
            harness
                .query_all_by_label(REST_RESTORES_WORDS)
                .next()
                .is_some()
        );
        harness.get_by_label("Undo change to ruling").click();
        harness.run_steps(2);
        assert!(!harness.state().events[0].restores);
    }

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
    fn long_rest_added_on_timeline_restores_and_undo_removes_its_ruling() {
        check_long_rest_add(&Kind::ALL);
    }

    /// Row 7 of the plain evening's contract: a line of More alone moves no action of the strip.
    /// A strip that needs no More has none of those lines; one that has More lists them last.
    /// A list of 30 events in a scroll column, drawn twice.
    fn long_list() -> egui_kittest::Harness<'static, (TimelinePage, Book)> {
        use crate::design::test_harness::Themed as _;
        let mut book = Book::default();
        for i in 0..30 {
            let id = book.add(Kind::Event, None);
            if let Some(event) = book.events.iter_mut().find(|event| event.id == id) {
                event.title = format!("Event {i}");
            }
        }
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(600.0, 400.0))
            .themed_ui_state(
                |ui, (page, book): &mut (TimelinePage, Book)| {
                    design::ScrollColumn::new("timeline_list_test")
                        .show(ui, |ui| page.list(ui, book, &[]));
                },
                (TimelinePage::default(), book),
            );
        harness.run_steps(2);
        harness
    }

    /// Review of PR 334: the list view is worked out once for frames with an unchanged book and
    /// filters (the count is inside `ListView::make`).
    #[test]
    fn a_long_list_works_out_its_view_once() {
        let made = VIEWS_MADE.with(std::cell::Cell::get);
        let mut harness = long_list();
        harness.run_steps(10);
        assert_eq!(
            VIEWS_MADE.with(std::cell::Cell::get) - made,
            1,
            "twelve frames with an unchanged book and filters work out the list view once"
        );
        // Must stay true: a change of the filters works it out again, once.
        harness.state_mut().0.filter = Filter::Events;
        harness.run_steps(10);
        assert_eq!(
            VIEWS_MADE.with(std::cell::Cell::get) - made,
            2,
            "a new filter, one new view"
        );
    }

    /// Review of PR 334: the list draws only the rows in view, and each row's ids come from its
    /// event. A focused row keeps its event when the list scrolls, and Enter opens that event.
    #[test]
    fn a_focused_row_keeps_its_event_when_the_list_scrolls() {
        use egui_kittest::kittest::NodeT;
        let mut harness = long_list();
        let focused = |harness: &egui_kittest::Harness<'_, (TimelinePage, Book)>| {
            harness
                .root()
                .children_recursive()
                .find(|node| node.is_focused())
                .and_then(|node| node.accesskit_node().label())
        };
        let top_of = |harness: &egui_kittest::Harness<'_, (TimelinePage, Book)>, name: &str| {
            harness
                .root()
                .children_recursive()
                .find(|node| {
                    node.accesskit_node()
                        .label()
                        .is_some_and(|label| label.starts_with(name))
                })
                .and_then(|node| node.accesskit_node().bounding_box())
                .map(|rect| rect.y0)
        };
        for _ in 0..10 {
            if focused(&harness).is_some_and(|label| label.starts_with("Event 3 ")) {
                break;
            }
            harness.key_press(egui::Key::Tab);
            harness.run_steps(2);
        }
        assert!(
            focused(&harness).is_some_and(|label| label.starts_with("Event 3 ")),
            "Tab reaches Event 3: {:?}",
            focused(&harness)
        );
        let before = top_of(&harness, "Event 3 ");
        harness.event(egui::Event::PointerMoved(egui::pos2(300.0, 200.0)));
        harness.event(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -(ROW + 16.0)),
            modifiers: Default::default(),
        });
        harness.run_steps(20);
        let after = top_of(&harness, "Event 3 ");
        assert!(
            before
                .zip(after)
                .is_some_and(|(before, after)| after < before - f64::from(ROW)),
            "the list scrolled by more than a row: {before:?} then {after:?}"
        );
        assert!(
            focused(&harness).is_some_and(|label| label.starts_with("Event 3 ")),
            "the focused row still names Event 3 after the scroll: {:?}",
            focused(&harness)
        );
        harness.key_press(egui::Key::Enter);
        harness.run_steps(2);
        let opened = harness.state().0.selected.clone();
        let event_3 = harness
            .state()
            .1
            .events
            .iter()
            .find(|event| event.title == "Event 3")
            .map(|event| event.id.clone());
        assert_eq!(opened, event_3, "Enter opens the focused event, Event 3");
    }

    #[test]
    fn a_line_of_more_alone_moves_no_action() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        const KINDS: [Kind; 5] = [
            Kind::Day,
            Kind::LongRest,
            Kind::ShortRest,
            Kind::Travel,
            Kind::Event,
        ];
        let draw = |width: f32, scene: bool| {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(egui::vec2(width, 300.0))
                .themed_state(
                    move |ctx, (book, chosen): &mut (Book, Option<Result<String, usize>>)| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            let more = if scene {
                                vec![
                                    Action::quiet("Scene")
                                        .icon(icons::CHECK_SQUARE)
                                        .name("Add Scene"),
                                ]
                            } else {
                                Vec::new()
                            };
                            if let Some(picked) =
                                quick_add_with(ui, book, None, &KINDS, more, |ui| {
                                    let _ = Action::quiet("Fight").icon(icons::ENCOUNTERS).show(ui);
                                })
                            {
                                *chosen = Some(picked);
                            }
                        });
                    },
                    (Book::default(), None),
                );
            harness.run_steps(2);
            harness
        };
        let shown = |harness: &egui_kittest::Harness<'_, _>| {
            [
                "Add Day",
                "Add Long rest",
                "Add Short rest",
                "Add Travel",
                "Add Major event",
                "More Add actions",
                "Add Scene",
            ]
            .map(|name| harness.query_by_label(name).map(|node| node.rect()))
        };
        for width in [360.0, 1200.0] {
            let without = draw(width, false);
            let mut with = draw(width, true);
            assert_eq!(shown(&without), shown(&with), "at {width} px");
            let more = with.query_by_label("More Add actions").is_some();
            assert_eq!(more, width < 1000.0, "at {width} px");
            if more {
                with.get_by_label("More Add actions").click();
                with.run_steps(2);
                let scene = with.get_by_label("Add Scene").rect();
                for name in ["Add Day", "Add Short rest", "Add Travel", "Add Major event"] {
                    assert!(
                        with.get_by_label(name).rect().bottom() <= scene.top(),
                        "{name}"
                    );
                }
                with.get_by_label("Add Scene").click();
                with.run_steps(2);
                assert_eq!(with.state().1, Some(Err(0)));
                assert!(with.state().0.events.is_empty());
            }
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

    #[test]
    /// Review of PR 340: a removal that notes link to keeps its warning when the DM opens
    /// another campaign's Timeline, works there, and comes back: through the page's own `show`
    /// and the book's campaign switch. B's file is not touched by A's Undo.
    fn a_removal_keeps_its_linked_note_warning_over_a_campaign_switch() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        let (a, b) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let mut book = Book::default();
        book.open(Some(a.path()));
        let id = book.add(Kind::Event, None);
        book.events[0].title = "Toll gate".into();
        book.save();
        let (campaign_a, campaign_b) = (Uuid::from_u128(1), Uuid::from_u128(2));
        let links = vec![SessionLink {
            id: Uuid::from_u128(9),
            label: "Session 1".into(),
            mentions: vec![id.clone()],
        }];
        type State = (
            TimelinePage,
            Book,
            crate::sessions::page::Sessions,
            Uuid,
            Vec<SessionLink>,
        );
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(1200.0, 800.0))
            .themed_state(
                |ctx, (page, book, notes, campaign, links): &mut State| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        page.show(ui, book, *campaign, links, None, notes);
                    });
                },
                (
                    TimelinePage::default(),
                    book,
                    crate::sessions::page::Sessions::default(),
                    campaign_a,
                    links.clone(),
                ),
            );
        harness.run_steps(2);
        harness.state_mut().0.selected = Some(id.clone());
        harness.run_steps(2);
        harness.get_by_label("More actions for Toll gate").click();
        harness.run_steps(2);
        harness.get_by_label("Remove this event").click();
        harness.run_steps(3);
        let warned = |harness: &egui_kittest::Harness<'_, State>| {
            harness
                .query_all_by_label_contains("1 note still links to it")
                .next()
                .is_some()
        };
        assert!(warned(&harness), "A's removal says a note links to it");
        // Campaign B: its own Timeline, and a day added there.
        {
            let state = harness.state_mut();
            state.3 = campaign_b;
            state.4 = Vec::new();
            state.1.open(Some(b.path()));
            state.1.add(Kind::Day, None);
            state.1.save();
        }
        harness.run_steps(3);
        // B shows no Undo of A's removal (review of PR 720: a wrong owner's label fails).
        assert!(
            harness
                .query_all_by_label_contains("Undo remove ")
                .next()
                .is_none(),
            "campaign B shows no Undo of A's removal"
        );
        let b_bytes = std::fs::read(b.path().join(super::super::FILE)).unwrap();
        // Back to A: the removal and its warning are still A's.
        {
            let state = harness.state_mut();
            state.3 = campaign_a;
            state.4 = links;
            state.1.open(Some(a.path()));
        }
        harness.run_steps(3);
        assert!(warned(&harness), "A's warning came back with A's removal");
        // The Undo names A's event, exactly.
        harness.get_by_label("Undo remove Toll gate").click();
        harness.run_steps(3);
        assert!(
            harness.state().1.events.iter().any(|event| event.id == id),
            "the Undo puts back A's event"
        );
        assert_eq!(
            std::fs::read(b.path().join(super::super::FILE)).unwrap(),
            b_bytes
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
}
