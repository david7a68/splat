//! The Timeline page: the whole campaign in story order, apart from any one session.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{Book, Event, Kind, checks, clock, clock_now, clock_words, day_numbers, reference};
use crate::design::{
    self, Action, Density, Status, Tone, Type, colors, icons,
    outcome::{
        Choice, Fault, FaultMessage, KeptOutcome, MessageChoice, Messages, Notice, PaneFrame, Said,
        Sentence, Severity, UndoStep, Verb,
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

#[cfg(test)]
thread_local! {
    /// How many times this thread worked out a list view: the test counts the work itself.
    static VIEWS_MADE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

impl ListView {
    fn make(book: &Book, (filter, search, session): &(Filter, String, Option<Uuid>)) -> Self {
        #[cfg(test)]
        VIEWS_MADE.with(|made| made.set(made.get() + 1));
        let days = day_numbers(&book.events);
        let rows = (0..book.events.len())
            .filter(|&index| matches(*filter, search, *session, &book.events[index], days[index]))
            .collect();
        Self {
            flagged: book
                .baseline
                .as_deref()
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

/// A session, as the timeline knows it.
#[derive(Clone, Debug)]
pub struct SessionLink {
    pub id: Uuid,
    pub label: String,
    /// The events its note links to.
    pub mentions: Vec<String>,
}

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
    reset_detail: bool,
    scroll_to: Option<usize>,
    /// An event just added with the buttons, with one step of undo.
    added: design::Results<super::TimelineFile, JustAdded>,
    /// The list opens where the story is now, at its end.
    opened: bool,
    /// One column: the detail hides the list, so the open event's results end its form.
    narrow: bool,
    /// The run open now, whose event must stay while it runs.
    pub open_run: Option<Uuid>,
    /// Saved runs from the same campaign snapshot used by the Session plan.
    pub saved_runs: Vec<crate::encounter_run::SavedRunInfo>,
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
    pub tables: Option<std::sync::Arc<crate::loot_editor::Sources>>,
    focus_length: bool,
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
                .icon(kind.icon())
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

/// The Timeline page's add result: the event the buttons added, until the DM moves on, opens
/// another event, or `UNDO_ADD_FOR` passes, so it cannot undo the wrong event.
pub struct JustAdded {
    file: super::TimelineFile,
    id: String,
    at: Instant,
}

impl design::results::Kept for JustAdded {
    type Owner = super::TimelineFile;
    const ENDS: design::results::Ends = design::results::Ends {
        // Another selection, or the time running out, ends it (`list_messages`).
        owner_changed: design::results::OwnerChanged::Ends,
        disk_changed: design::results::DiskChanged::Stays,
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> super::TimelineFile {
        self.file.clone()
    }
}

/// A footer fault of the Timeline: a save that failed, or a removal that notes still link to.
/// The kind gives the tone.
enum FileFault {
    /// Notes still link to the removed event: the DM checks them.
    Linked(Sentence),
    NotSaved(Sentence),
}

impl Fault for FileFault {
    fn severity(&self) -> Severity {
        match self {
            Self::Linked(_) => Severity::NeedsFixing,
            Self::NotSaved(_) => Severity::NotSaved,
        }
    }
    fn cause(&self) -> Sentence {
        match self {
            Self::Linked(words) | Self::NotSaved(words) => words.clone(),
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
                UndoStep::verb(Verb::Reload).pressed_as(undo),
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
}

/// The file's flags and a second timeline file, each with the file's line (its path, `Edit in
/// Obsidian` and `Show file`) under the last, at the bottom of the room a pane would take, outside
/// its scroll: on the footer's side, so they stay in view over long content and move no row. A
/// typed footer message has no file line yet (a gap of the message part, sent to its owner), so
/// they are not footer messages. Returns the room left above them for the pane (all of it with
/// no flag).
pub fn file_flags_below(ui: &mut egui::Ui, book: &Book) -> egui::Rect {
    let room = ui.available_rect_before_wrap();
    if book.flags.is_empty() && book.duplicate.is_none() {
        return room;
    }
    // Their height first, in a pass that paints nothing, then the flags at the room's bottom.
    let layout = egui::Layout::top_down(egui::Align::Min);
    let mut measure = ui.new_child(
        egui::UiBuilder::new()
            .id_salt("file_flags_measure")
            .max_rect(room)
            .layout(layout)
            .sizing_pass()
            .invisible(),
    );
    file_flags(&mut measure, book);
    let height = measure.min_rect().height().min(room.height());
    let at = egui::Rect::from_min_max(egui::pos2(room.left(), room.bottom() - height), room.max);
    let mut below = ui.new_child(
        egui::UiBuilder::new()
            .id_salt("file_flags")
            .max_rect(at)
            .layout(layout),
    );
    file_flags(&mut below, book);
    room.with_max_y((at.top() - design::S).max(room.top()))
}

fn file_flags(ui: &mut egui::Ui, book: &Book) {
    let folder = book.path.as_deref().and_then(std::path::Path::parent);
    for (at, flag) in book.flags.iter().enumerate() {
        let text = flag_words(flag);
        match folder {
            Some(folder) if at + 1 == book.flags.len() => {
                design::message_with_actions(ui, Tone::Warning, &text, |ui| {
                    crate::encounters::file_line(ui, folder, super::FILE);
                });
            }
            _ => {
                design::message(ui, Tone::Warning, &text);
            }
        }
    }
    if let Some(warning) = &book.duplicate {
        design::message(ui, Tone::Warning, warning);
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
pub fn problems(ui: &mut egui::Ui, book: &mut Book) -> bool {
    if let Some(error) = book.unreadable.clone() {
        // Message choices start at the message text. The old-format notice draws its own.
        let choices = |ui: &mut egui::Ui, book: &mut Book| {
            ui.horizontal_wrapped(|ui| {
                if Action::secondary("Read the file again").show(ui).clicked() {
                    book.reload();
                }
                if book.gone
                    && Action::secondary("Save this timeline there again")
                        .show(ui)
                        .clicked()
                {
                    book.save_again();
                }
            });
        };
        design::message_with_actions(ui, Tone::Error, &error, |ui| choices(ui, book));
    } else if book.conflict {
        design::message_with_actions(
            ui,
            Tone::Warning,
            crate::timeline::CHANGED_UNDER_DRAFT,
            |ui| {
                ui.horizontal_wrapped(|ui| {
                    if Action::secondary("Overwrite with my timeline")
                        .show(ui)
                        .clicked()
                    {
                        book.overwrite();
                    }
                    // One `L` gap before the red choice: the line's item gap (`S`) and this `S`.
                    design::gap(ui, design::Gap::S);
                    if Action::destructive("Reload and lose my timeline")
                        .show(ui)
                        .clicked()
                    {
                        book.reload_keeping_undo();
                    }
                });
            },
        );
        if book.conflict {
            match book.conflict_comparison() {
                Ok((draft, disk)) => {
                    let face = egui::TextStyle::Monospace.resolve(ui.style());
                    let height = ui.fonts_mut(|fonts| fonts.row_height(&face));
                    let id = ("timeline_conflict", &book.path, &disk);
                    design::draft_comparison_styled(ui, &draft, &disk, id, face, height);
                }
                Err(error) => {
                    design::message(ui, Tone::Error, &error);
                }
            }
            return true;
        }
    }
    false
}

impl TimelinePage {
    /// Show one event, whatever the filters were hiding.
    pub fn reveal(&mut self, id: &str, book: &Book) {
        self.filter = Filter::All;
        self.search.clear();
        self.session = None;
        self.reset_detail = true;
        self.selected = Some(id.to_owned());
        self.scroll_to = book.events.iter().position(|event| event.id == id);
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
    /// Kept on `egui::ComboBox`: its narrow control shows the chosen session's short form
    /// (`Session 1`) while its list shows the whole names, and `design::Dropdown` shows only the
    /// chosen row's own words (a gap sent to the part's owner).
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn new_events_line(
        &mut self,
        ui: &mut egui::Ui,
        sessions: &[SessionLink],
        current: Option<Uuid>,
    ) {
        ui.label(Type::Small.rich(ui, "Adds to").color(colors().muted));
        let label = current
            .and_then(|id| sessions.iter().find(|session| session.id == id))
            .map_or("No session", |session| short_session(&session.label));
        let mut chosen = current;
        let inner = egui::ComboBox::from_id_salt("timeline_new_events_session")
            .truncate()
            .selected_text(label)
            .width(design::READER_MEASURE / 4.0)
            .icon(design::combo_icon)
            .show_ui(ui, |ui| {
                design::choice_value(ui, &mut chosen, None, "No session");
                for session in sessions {
                    design::choice_wrapped_value(ui, &mut chosen, Some(session.id), &session.label);
                }
            });
        design::named(ui, &inner.response, "Session for new events");
        inner.response.on_hover_text(label);
        if chosen != current {
            self.new_events_to = Some(chosen);
        }
    }

    fn filters(&mut self, ui: &mut egui::Ui, sessions: &[SessionLink], current: Option<Uuid>) {
        let before = (self.filter, self.search.clone(), self.session);
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
            // `Clear` changes `self.search`, which `before` below sees; nothing else to reset.
            let _ = TIMELINE_SEARCH.search(
                ui,
                "Search the timeline",
                &mut self.search,
                "Search",
                160.0,
            );
            design::Dropdown::new(SESSION_FILTER, "Show one session")
                .choice(None, "All sessions")
                .choices(
                    sessions
                        .iter()
                        .map(|session| (Some(session.id), session.label.as_str())),
                )
                .width(150.0)
                .show(ui, &mut self.session);
            self.new_events_line(ui, sessions, current);
        });
        if before != (self.filter, self.search.clone(), self.session) {
            self.reset_detail = true;
        }
    }

    pub fn reopen(&mut self) {
        self.reset_detail = true;
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
                event.kind.icon(),
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
                let second = second_line(ui, &detail, &spans, width);
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
                        ui.add(
                            egui::Label::new(Type::Small.rich(ui, after).color(c.muted)).truncate(),
                        );
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
            self.selected = Some(event.id.clone());
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
    /// draws the rows, the empty state, or (narrow) the open event's form, as the pane's content.
    fn list_pane<R>(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        notes: &mut crate::sessions::page::Sessions,
        open: Option<&str>,
        body: impl FnOnce(
            &mut Self,
            &mut egui::Ui,
            &mut Book,
            &mut crate::sessions::page::Sessions,
        ) -> R,
    ) -> R {
        let open = open.filter(|id| book.events.iter().any(|event| event.id == *id));
        let offer = open.and_then(|id| notes.event_link_offer(book, id));
        let frame = PaneFrame::new(ui);
        let messages = self.list_messages(&frame, ui, book, notes, open, offer.as_ref());
        let full = ui.available_rect_before_wrap();
        let room = file_flags_below(ui, book);
        let mut pane_ui = ui.new_child(egui::UiBuilder::new().max_rect(room).layout(*ui.layout()));
        let paned = messages.pane(&mut pane_ui, design::Body::Content, |ui| {
            body(self, ui, book, notes)
        });
        ui.advance_cursor_after_rect(full);
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
                        self.selected = None;
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
        notes: &crate::sessions::page::Sessions,
        open: Option<&str>,
        offer: Option<&crate::sessions::page::LinkOffer>,
    ) -> Messages<'f, ListAct> {
        let mut messages = Messages::new(frame, "timeline_list_footer", ui.available_width());
        if let Some(line) = notes.event_link_line(frame, ListAct::UndoLinks) {
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
            let words = as_sentence(&timeline_removal_words(
                book,
                shown.value().label(),
                mentions,
            ));
            messages.kept(shown.outcome(frame, |_| {
                let undo = UndoStep::verb(Verb::Remove).pressed_as(ListAct::UndoRemove);
                // A finished change has the success tone; links left to it make it a warning.
                if mentions > 0 {
                    KeptOutcome::Fault(FaultMessage::new(&FileFault::Linked(words)), undo)
                } else {
                    Said::new(words).undo(undo).into()
                }
            }));
        }
        // It goes when the DM moves on, or soon, so that it cannot undo the wrong event.
        let file = book.file();
        if self.added.get(&file).is_some_and(|shown| {
            let added = shown.value();
            added.at.elapsed() >= UNDO_ADD_FOR || self.selected.as_ref() != Some(&added.id)
        }) {
            self.added.owner_changed(&file);
        }
        if let Some(shown) = self.added.get(&file) {
            ui.ctx().request_repaint_after(UNDO_ADD_FOR);
            let days = day_numbers(&book.events);
            let id = shown.value().id.clone();
            let label = book
                .events
                .iter()
                .position(|event| event.id == id)
                .map_or_else(String::new, |at| book.events[at].label(days[at]));
            let event = book.events.iter().find(|event| event.id == id);
            let restores = event
                .filter(|event| event.kind == Kind::LongRest && event.restores)
                .map(|event| {
                    super::fights_that_keep_hp(&book.events, event.session, &self.saved_runs)
                });
            // Timeline lists every session, so its result names only the event.
            let words = as_sentence(&added_message(&label, None, restores));
            messages.kept(shown.outcome(frame, |_| {
                Said::new(words)
                    .undo(UndoStep::verb(Verb::Add).pressed_as(ListAct::UndoAdd))
                    .into()
            }));
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
        // The row before the target comes to the top, as the list opened before.
        if let Some(target) = self.scroll_to.take()
            && let Some(row) = rows.iter().position(|&index| index == target)
        {
            ui.scroll_to_rect(place(row.saturating_sub(1)), Some(egui::Align::Min));
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
    fn detail(&mut self, ui: &mut egui::Ui, book: &mut Book, sessions: &[SessionLink]) {
        let c = colors();
        let index = self
            .selected
            .as_ref()
            .and_then(|id| book.events.iter().position(|event| event.id == *id));
        checks::settle_length(book, index);
        let Some(index) = index else {
            ui.label(Type::Heading.rich(ui, "An event").color(c.text));
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
            });
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
        let icon_only = event.kind == Kind::Day || event.own_title().is_none();
        let kind = header_kind(event);
        let needed = design::number_width(ui, &title, &Type::Heading.font(ui))
            + design::number_width(ui, &kind, &Type::Small.font(ui))
            + design::number_width(ui, "Saving blocked", &Type::Small.font(ui))
            + design::number_width(ui, icons::OK, &Type::Small.font(ui))
            + Action::secondary("Copy link")
                .icon(icons::COPY)
                .density(Density::Read)
                .natural_width(ui)
            + Density::Read.control()
            + design::S * 5.0;
        let title_line = |ui: &mut egui::Ui| {
            let kind_label = ui.label(Type::Small.rich(ui, &kind).color(c.muted));
            if icon_only {
                design::named(ui, &kind_label, event.kind.word());
                kind_label.on_hover_text(event.kind.word());
            }
            let shown =
                design::words_that_fit(ui, &title, &Type::Heading.font(ui), ui.available_width());
            ui.label(Type::Heading.rich(ui, shown).color(c.text))
                .on_hover_text(&title);
        };
        let mut actions = |ui: &mut egui::Ui| {
            let (icon, word, color) = if blocked.is_some() {
                (
                    icons::WARN,
                    if unsaved { "Unsaved" } else { "Saving blocked" },
                    c.document.external,
                )
            } else if unsaved {
                (icons::BUSY, "Saving…", c.muted)
            } else {
                (icons::OK, "Saved", c.document.clean)
            };

            let more = design::icon_button(
                ui,
                icons::EXPAND,
                &format!("More actions for {}", event.label(days[index])),
            );
            design::menu_popup(ui, &more).show(|ui| {
                if checks::offered(event, tables)
                    && Action::quiet("Roll for encounters").show(ui).clicked()
                {
                    checks_act = Some(checks::Act::Start);
                    ui.close();
                }
                let open_run = event.run.is_some() && event.run == self.open_run;
                removed = Action::quiet("Remove")
                    .name("Remove this event")
                    .blocked_if(open_run, "This run is open. Close it first.")
                    .show(ui)
                    .clicked();
                if open_run {
                    ui.label(
                        Type::Small
                            .rich(ui, "This run is open. Close it first.")
                            .color(c.muted),
                    );
                }
                if removed {
                    ui.close();
                }
            });
            let copied = self
                .copied
                .as_ref()
                .is_some_and(|(copied, _)| *copied == id);
            if Action::secondary(if copied { "Copied" } else { "Copy link" })
                .icon(if copied { icons::OK } else { icons::COPY })
                .density(Density::Read)
                .show(ui)
                .clicked()
            {
                ui.ctx().copy_text(link.clone());
                self.copied = Some((id.clone(), Instant::now()));
            }
            design::status(ui, design::Status { icon, word, color });
        };
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
        ui.allocate_ui_with_layout(
            egui::vec2(header_width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                if needed <= ui.available_width() {
                    ui.horizontal(|ui| {
                        title_line(ui);
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            &mut actions,
                        );
                    });
                } else {
                    ui.horizontal(title_line);
                    ui.allocate_ui_with_layout(
                        egui::vec2(ui.available_width(), Density::Read.control()),
                        egui::Layout::right_to_left(egui::Align::Center),
                        actions,
                    );
                }
            },
        );
        if let Some(reason) = blocked {
            ui.label(Type::Small.rich(ui, reason).color(c.muted));
        }
        design::rule(ui);
        ui.scope(|ui| {
            ui.set_max_width(header_width);
            let event = &mut book.events[index];
            design::gap(ui, design::Gap::XS);
            let hint = (event.kind == Kind::Day)
                .then_some("Optional. Days number themselves, so a name is extra.");
            let title = EVENT_TITLE.of(design::TextKey::of(&*event));
            let title = design::labelled_field(ui, title, "Title", &mut event.title, hint, None);
            changed |= title.changed();
            if std::mem::take(&mut self.focus_title) {
                title.request_focus();
            }
        });
        design::gap(ui, design::Gap::S);
        design::rule(ui);
        let mut ruling = None;
        let mut area = egui::ScrollArea::vertical()
            .id_salt(("timeline_detail", &id))
            .auto_shrink([false, false]);
        if std::mem::take(&mut self.reset_detail) {
            area = area.vertical_scroll_offset(0.0);
        }
        let clip = ui.available_rect_before_wrap().intersect(ui.clip_rect());
        // Round 27 FL1: the form leaves the scroll bar's lane free whether or not the bar shows,
        // so its fields and the header's `⋯` end at one x for every event.
        let lane_width = ui.available_width() - ui.spacing().scroll.allocated_width();
        area.show(ui, |ui| {
            ui.set_clip_rect(ui.clip_rect().intersect(clip));
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
                    let focus = std::mem::take(&mut self.focus_length);
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
                    planned: &|_| None,
                };
                checks_act = checks::header(ui, &view, &mut self.checks).or(checks_act.take());
                // The act's result keeps its line under the table, so it moves no field.
                let done = self.checks_done.get(&id);
                let result = done.map(|done| (done.words.as_str(), done.undo));
                undo_checks |= design::undo_line(ui, Tone::Success, result);
                checks_act = checks::lines(ui, &view, &mut self.checks).or(checks_act.take());
            }
            design::gap(ui, design::Gap::M);
            let place = EVENT_PLACE.of(design::TextKey::of(&*event));
            changed |=
                design::labelled_field(ui, place, "Where", &mut event.place, None, None).changed();
            changed |= self.place_help(ui, event);
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
                self.open_session = Some(session);
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
                                    event.run.is_some() && event.run == self.open_run,
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
                            self.open_session = Some(session.id);
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
            match checks::apply(book, &id, &act, tables) {
                Ok(Some(done)) => self.checks_done.keep(done),
                Ok(None) => {}
                Err(error) => self.checks.failed(&act, error),
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
            self.selected = None;
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
            *self = Self {
                open_run: self.open_run,
                saved_runs: std::mem::take(&mut self.saved_runs),
                ..Default::default()
            };
        }
        self.campaign = Some(campaign_id);
        design::page_header(
            ui,
            "Timeline",
            Some(&if book.events.is_empty() {
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
            }),
            |_| {},
        );
        if problems(ui, book) {
            return;
        }
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)) {
            book.save();
        }
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
            self.added.record(JustAdded {
                file: book.file(),
                id,
                at: Instant::now(),
            });
        }
        if let Some(words) = self.dated.clone()
            && design::message_with_action(ui, Tone::Warning, &words, "Play this one")
        {
            self.play_current = true;
        }
        design::gap(ui, design::Gap::XS);
        if book.events.is_empty() && book.unreadable.is_none() {
            let open_run = self.open_run.is_some();
            let first = self.list_pane(ui, book, notes, None, |_, ui, _, _| {
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
                self.added.record(JustAdded {
                    file: book.file(),
                    id,
                    at: Instant::now(),
                });
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
            self.filters(ui, sessions, current);
        }
        design::gap(ui, design::Gap::XS);
        // Beside Lookup or Notes at a small window, two columns would crush the list.
        if narrow {
            if event_open {
                if Action::quiet("All events")
                    .icon(icons::BACK)
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.selected = None;
                }
                // With the list hidden, the open event's results are in the form's own footer,
                // so they move no field.
                notes.expire_event_link_undo(true, book);
                let open = self.selected.clone();
                self.list_pane(ui, book, notes, open.as_deref(), |page, ui, book, _| {
                    page.detail(ui, book, sessions);
                });
            } else {
                notes.expire_event_link_undo(true, book);
                self.list_pane(ui, book, notes, None, |page, ui, book, _| {
                    page.list(ui, book, sessions);
                });
            }
            return;
        }
        if !event_open {
            self.list_pane(ui, book, notes, None, |page, ui, book, _| {
                page.list(ui, book, sessions);
            });
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
                    .show_inside(ui, |ui| self.detail(ui, book, sessions));
                egui::CentralPanel::default()
                    // The panel's drag handle lies on this edge, so the rows stop short of it.
                    .frame(egui::Frame::new().inner_margin(egui::Margin {
                        right: design::S as i8,
                        ..Default::default()
                    }))
                    .show_inside(ui, |ui| {
                        notes.expire_event_link_undo(true, book);
                        let open = self.selected.clone();
                        self.list_pane(ui, book, notes, open.as_deref(), |page, ui, book, _| {
                            page.list(ui, book, sessions);
                        });
                    });
            },
        );
    }
}

/// The session's number alone. The row's own title is the event's.
fn short_session(label: &str) -> &str {
    label.split(':').next().unwrap_or(label)
}

#[cfg(test)]
pub fn check_long_rest_add(kinds: &'static [Kind]) {
    use egui_kittest::kittest::Queryable;
    let session = Some(Uuid::new_v4());
    let mut harness = egui_kittest::Harness::builder().build_state(
        move |ctx, book: &mut Book| {
            egui::CentralPanel::default().show(ctx, |ui| {
                quick_add(ui, book, session, kinds, |_| {});
            });
        },
        Book::default(),
    );
    design::theme::install(&harness.ctx);
    harness.run_steps(2);
    harness.get_by_label("Add Long rest").click();
    harness.run_steps(2);
    assert_eq!(harness.state().events.len(), 1);
    assert!(super::party_state(&harness.state().events, session).rested);
    let id = harness.state().events[0].id.clone();
    harness.state_mut().remove(&id);
    assert!(!super::party_state(&harness.state().events, session).rested);
}

/// A row's second line on one line, muted, with the rest ruling from `at` in its own colour.
/// A row's second line: muted, with each status part (the rest ruling, a length the clock
/// cannot count) in its own colour. `spans` are byte ranges in `detail`, in order.
fn second_line(
    ui: &egui::Ui,
    detail: &str,
    spans: &[(std::ops::Range<usize>, egui::Color32)],
    width: f32,
) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::default();
    let mut at = 0;
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

fn header_kind(event: &Event) -> String {
    if event.kind == Kind::Day || event.own_title().is_none() {
        event.kind.icon().to_owned()
    } else {
        format!("{} {}", event.kind.icon(), event.kind.word())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter3_c_closed_long_rest_rows_show_both_rulings() {
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
            let mut harness = egui_kittest::Harness::builder().build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    TimelinePage::default().row(ui, &event, 0, None, None, None);
                });
            });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            harness.get_by_label(&expected);
        }
    }

    #[test]
    fn iter1_c_day_appears_only_when_filters_hide_day_headings() {
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
            let mut harness = egui_kittest::Harness::builder().build(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut page = TimelinePage::default();
                    if filtered {
                        page.filter = Filter::Rests;
                    }
                    page.row(ui, &event, 1, None, None, None);
                });
            });
            design::theme::install(&harness.ctx);
            harness.run_steps(2);
            harness.get_by_label(&expected);
        }
    }

    #[test]
    fn iter1_c_rest_ruling_actions_name_the_change_and_keep_undo() {
        use egui_kittest::kittest::Queryable;
        let mut book = Book::default();
        book.add(Kind::LongRest, None);
        book.events[0].restores = true;
        let event_id = book.events[0].id.clone();
        // The ruling line in the row, and its result in the pane's footer, as the pages draw them.
        let mut harness = egui_kittest::Harness::builder().build_state(
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
        design::theme::install(&harness.ctx);
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
        harness.get_by_label("Undo change ruling").click();
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
        let mut book = Book::default();
        for i in 0..30 {
            let id = book.add(Kind::Event, None);
            if let Some(event) = book.events.iter_mut().find(|event| event.id == id) {
                event.title = format!("Event {i}");
            }
        }
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(600.0, 400.0))
            .build_ui_state(
                |ui, (page, book): &mut (TimelinePage, Book)| {
                    design::ScrollColumn::new("timeline_list_test")
                        .show(ui, |ui| page.list(ui, book, &[]));
                },
                (TimelinePage::default(), book),
            );
        design::theme::install(&harness.ctx);
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
                .build_state(
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
            design::theme::install(&harness.ctx);
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
            .build_state(
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
        design::theme::install(&harness.ctx);
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
        harness.get_by_label("Undo remove").click();
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
