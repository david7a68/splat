//! The Sessions page: notes, templates and recordings, one campaign at a time. The
//! recorder and the processing job live here, and keep going while the DM uses other pages.
use super::{
    Apply, DEFAULT_FOLDER, Draft, DraftState, Dropped, NewNote, Note, NoteUndo, Recording,
    Template, body_of, create_note, create_template, folder_setting, insert_in_note,
    insert_in_plan, linked_names, list_notes, list_recordings, list_templates, new_recording_path,
    parts_of, plain_links, recording_title, rename_links, rename_links_in_file, save_folder,
    section_text, unreadable_notes, update_details, without_empty_sections, without_own_heading,
};
use crate::design::Color;
use crate::design::text_draft::TextDraft;
use crate::state_context::StateContext;
use crate::{
    credentials::{self, KeySlot},
    design::{
        self, CreateReasons, Status, Tone, colors, icons,
        outcome::{
            Choice, Fault, FaultMessage, KeptOutcome, Lost, MessageChoice, Messages, Notice,
            PaneFrame, Press, Said, Sentence, Severity, UndoStep, Verb,
        },
        results::ByTicket,
    },
    encounters::PlanFight,
    lookup::{Written, WrittenIn},
    persistence::Campaign,
    processing::{self, Job, Request, settings::ProcessingSettings},
    recording::{Health, Recorder},
    timeline::{self, Book, EventDisplay, page::SessionLink},
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use uuid::Uuid;

#[path = "carry.rs"]
mod carry;
#[path = "folder_view.rs"]
mod folder_view;
/// What the GPUI view reads and does (`src/gui/pages/sessions/`).
#[path = "gpui_facade.rs"]
pub mod gpui_facade;
#[path = "kept.rs"]
mod kept;
#[path = "plan.rs"]
mod plan;

const MOVE_FILES: &str = "Move files and save";
const SAVED_WORD_FOR: Duration = Duration::from_secs(8);
/// The header's toggle of the session list, by what a press does.
/// The session list's toggle: one name in both states; its pressed state says whether the list
/// is shown (review of PR 618).
pub const LIST_TOGGLE: &str = "Session list";
const CHECK_DISK_EVERY: Duration = Duration::from_secs(1);
const SCAN_EVERY: Duration = Duration::from_secs(5);
const MISSING_INPUT_ERROR: &str = "no such device";
const PROCESSING_CANCELLED: &str = "Processing cancelled";

crate::create_reasons! {
    /// Why the New session form's Create (or the details form's Save details) waits.
    enum FormReason {
        Number => "Enter the session number as a whole number.",
        Date => "Enter the date as YYYY-MM-DD.",
        TemplateGone => "The chosen template is gone from the folder. Choose another.",
    }
}

/// Why `form` cannot be made now, judged on its words and on the templates the page lists.
fn form_reason(form: &Form, templates: &[Template]) -> Option<FormReason> {
    if form.number.trim().parse::<u32>().is_err() {
        return Some(FormReason::Number);
    }
    // A date's order is how Splat tells an old session from tonight's.
    let date = form.date.trim().as_bytes();
    if date.len() != 10
        || date.iter().enumerate().any(|(at, c)| {
            if at == 4 || at == 7 {
                *c != b'-'
            } else {
                !c.is_ascii_digit()
            }
        })
    {
        return Some(FormReason::Date);
    }
    // A new session starts from a template the folder still holds.
    (form.editing.is_none()
        && !templates.is_empty()
        && !templates
            .iter()
            .any(|template| template.name == form.template))
    .then_some(FormReason::TemplateGone)
}

#[derive(Clone, Copy, Default, PartialEq)]
enum View {
    #[default]
    Notes,
    Templates,
}

/// What one listing of the current session folder found, or why it could not list it. `None`:
/// no folder.
type FolderFiles = Result<Option<super::StorageCounts>, String>;

/// The session folder's listing, kept for the folder it lists. Its owner is that folder; it
/// ends at each disk scan (`end_folder_listing`, also while the New session form is open), a
/// campaign switch and a folder save, so the Link and folder page lists the folder once per
/// scan, not once a frame. It decides only what is drawn: an act on the disk lists the folder
/// at the act (`move_storage`, and the check before Save folder).
struct FolderScan {
    folder: PathBuf,
    listed: FolderFiles,
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// The session folder listing's end events (`refresh`) on this thread.
    static FOLDER_SCAN_ENDS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub use splat_content_ui::session_notes::FolderFault;

/// The Session folder's acts: `Save folder` in its save row, and the warning's two choices.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FolderAct {
    Save,
    Move,
    WithoutMoving,
}

/// Why the Session folder cannot be saved now, by its own state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FolderWait {
    Recording,
    Processing,
    Unsaved,
    CannotRead,
    NoChanges,
}

impl FolderWait {
    fn words(self) -> &'static str {
        match self {
            Self::Recording => "Stop the recording first.",
            Self::Processing => "Wait for processing to finish.",
            Self::Unsaved => "Choose what to do with the unsaved file first.",
            Self::CannotRead => "Could not read the session files. Check the current folder first.",
            Self::NoChanges => "No changes to save.",
        }
    }

    fn sentence(self) -> Sentence {
        match self {
            Self::Recording => crate::sentence!("Stop the recording first."),
            Self::Processing => crate::sentence!("Wait for processing to finish."),
            Self::Unsaved => crate::sentence!("Choose what to do with the unsaved file first."),
            Self::CannotRead => crate::sentence!(
                "Could not read the session files. Check the current folder first."
            ),
            Self::NoChanges => crate::sentence!("No changes to save."),
        }
    }
}

/// `The current folder keeps <counts>. …`: what Save without moving would hide.
fn folder_keeps(counts: &str) -> Sentence {
    crate::sentence!(
        "The current folder keeps {}. If you save without moving, Splat no longer shows these files.",
        counts
    )
}

/// The counts of one listing in words (`1 note, 4 events and 2 templates`), with the Timeline's
/// events; `None` when it lists no file.
fn storage_words(counts: super::StorageCounts, events: usize) -> Option<String> {
    if counts.files == 0 {
        return None;
    }
    let mut words = vec![
        design::count(counts.notes, "note", "notes"),
        design::count(events, "event", "events"),
        design::count(counts.templates, "template", "templates"),
        design::count(counts.recordings, "recording", "recordings"),
    ]
    .into_iter()
    .filter(|count| !count.is_empty())
    .collect::<Vec<_>>();
    let last = words.pop().unwrap_or_default();
    let words = if words.is_empty() {
        last
    } else {
        format!("{} and {last}", words.join(", "))
    };
    Some(if words.is_empty() {
        "Timeline.md".to_owned()
    } else {
        words
    })
}

/// `Moved <counts> to <folder>.`, from the store's own value.
fn folder_moved<A>(moved: &kept::FolderMoved) -> design::outcome::Done<A> {
    design::outcome::Done::new(Verb::Move, format!("{} to {}", moved.counts, moved.typed))
}

/// Round 30 `recording-result-owner` (FG1, SG3): `Recording saved: <title>.`, kept by the
/// session whose recording it names. It shows in that session's footer only, and only while its
/// file stands: every listing of the recordings and the disk check once a second
/// (`CHECK_DISK_EVERY`, also while nothing records) ask the disk, and a look that no longer finds
/// the file ends it, whichever session is open. Leaving the tab the DM was on (`clear_success`),
/// its `Dismiss`, a recording that starts in the same session (`start_recording`) and a campaign
/// change (`open_campaign`) end it too.
struct RecordingSaved {
    owner: kept::OfSession,
    path: PathBuf,
}

impl design::results::Kept for RecordingSaved {
    type Owner = kept::OfSession;
    const ENDS: design::results::Ends = design::results::Ends {
        // Leaving the tab ends it, as every success of the page (`clear_success`).
        owner_changed: design::results::OwnerChanged::Ends,
        // It holds while its file stands (`refresh_recordings`).
        disk_changed: design::results::DiskChanged::Checked,
        // `Dismiss` uses it up.
        press: design::results::Press::Use,
    };
    fn owner(&self) -> kept::OfSession {
        self.owner.clone()
    }
}

/// What the DM chose in the session page's message footer, done after the footer is drawn.
#[derive(Clone, Debug, PartialEq)]
pub enum SessionAct {
    MoveRunHere(Uuid),
    UndoMove,
    Dismiss,
    UndoIdentity,
    PlayThisOne,
    Open(Option<Uuid>),
    OpenSettings,
    UndoCarry(Uuid),
    UndoMarker,
    /// `Dismiss` of `Recording saved: …` (`RecordingSaved`).
    DismissSaved,
    /// `Undo reload` or `Undo discard` of the draft's dropped words.
    UndoDrop,
    /// `Undo delete` of a deleted template.
    UndoDeleteTemplate,
    /// The notes panel's `Play it tonight` and `New session`.
    PlayTonight,
    NewSession,
    /// A press in the plan's part of the footer: the plan acts on it.
    Plan(plan::FooterAct),
}

#[derive(Clone, Copy, Default, PartialEq)]
enum Tab {
    #[default]
    Plan,
    /// In a narrow page, the note has its own tab. In a wide one, it stands beside the plan.
    Note,
    Recordings,
}

/// The Encounters store's line of a plan group's discard, for one frame's footer.
pub type GroupDiscardLine<'a> = dyn for<'f> Fn(&'f design::outcome::PaneFrame) -> Option<design::outcome::KeptLine<'f, ()>>
    + 'a;

/// What the rest of the app tells this page about fights.
pub struct Plan<'a> {
    pub fights: &'a [PlanFight],
    pub saved_runs: &'a [crate::encounter_run::SavedRunInfo],
    /// The revision of the saved runs' facts (`Encounters::saved_runs_revision`): the plan's Add
    /// result names the fights that keep their HP by them.
    pub saved_runs_revision: design::memo::Revision,
    /// The revision of the campaign's fights as `fights` lists them
    /// (`Encounters::plan_fights_revision`): a fight read again, renamed, added or removed is a
    /// new one, so what the plan keeps by its fights (the planned fights' days) is worked out again.
    pub fights_revision: design::memo::Revision,
    pub groups: &'a [std::sync::Arc<crate::encounter_run::RunGroup>],
    /// Why no group can be assembled now.
    pub group_blocked: Option<&'a str>,
    /// A group being assembled takes the plan's place, beside the note.
    pub group_open: bool,
    /// A new group started in the plan was discarded there: the Encounters store's own line
    /// of that discard for this frame's footer (`Encounters::group_discard_line`), with the
    /// store's words, order and ticket.
    pub group_discard: &'a GroupDiscardLine<'a>,
    /// A fight is open that something can still join.
    pub arrivals_open: bool,
    /// Why no fight can start now.
    pub blocked: Option<&'a str>,
    pub run_directory: Option<PathBuf>,
    pub open_run: Option<Uuid>,
    pub open_state: String,
    pub open_encounter: Option<&'a str>,
    /// The party's identities in the party bar's order.
    pub party: &'a [Uuid],
    /// The campaign's marked tables, for a journey's encounter checks.
    pub tables: &'a [crate::roll_table::Table],
}

/// What the previous session left, for the top of this one.
struct LastTime {
    id: Uuid,
    label: String,
    what_happened: Option<String>,
    /// What the session that carry-over reads left, or why its note cannot be read. `None`
    /// when this note carried from a session that is gone.
    source: Option<Result<carry::Source, String>>,
    /// What `Carry over` would add, for the draft's revision it was read from.
    offer: Option<(u64, Option<carry::Offer>)>,
}

/// The new-session form, or the same fields for a session that exists.
struct Form {
    editing: Option<Uuid>,
    number: String,
    title: String,
    date: String,
    template: String,
    error: Option<String>,
}

#[derive(Clone, Copy, PartialEq)]
enum Made {
    Transcript,
    Marked,
    Review,
    Summary,
}

impl Made {
    fn word(self) -> &'static str {
        match self {
            Self::Transcript => "Raw transcript",
            Self::Marked => "Clean transcript",
            Self::Review => "What the check changed",
            Self::Summary => "Summary",
        }
    }

    fn path(self, recording: &Path) -> PathBuf {
        match self {
            Self::Transcript => processing::transcript_path(recording),
            Self::Marked => processing::marked_path(recording),
            Self::Review => processing::review_path(recording),
            Self::Summary => processing::summary_path(recording),
        }
    }
}

/// A text from a recording, read once, one part at a time.
struct Reading {
    recording: PathBuf,
    name: String,
    made: Made,
    parts: Vec<(String, String)>,
    part: usize,
    /// A raw transcript that stops early: parts done, and parts in all.
    short_of: Option<(u64, u64)>,
    /// The shown part as the reader draws it, made once per part (`ShownPart`).
    shown: Option<ShownPart>,
}

/// The shown part's words as the reader draws them (the draft's warning line out; with several
/// parts, the part's own heading out) (review of PR 862, P3): made once per part, so a quiet frame copies and scans nothing. The parts are read
/// once when the text opens, so the part is the words' revision.
struct ShownPart {
    part: usize,
    body: String,
}

#[cfg(test)]
thread_local! {
    /// How many times a shown part was made (tests).
    static SHOWN_PARTS_MADE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

impl ShownPart {
    fn of(reading: &Reading) -> Self {
        #[cfg(test)]
        SHOWN_PARTS_MADE.with(|made| made.set(made.get() + 1));
        let (time, text) = &reading.parts[reading.part];
        let cleaned = text.replace(
            "> This is a draft from an LLM. Check it against your own memory.\n",
            "",
        );
        let heading = format!("## {time}");
        let body = if reading.parts.len() > 1 {
            cleaned
                .trim_start()
                .strip_prefix(&heading)
                .filter(|rest| rest.starts_with('\n'))
                .map_or_else(|| cleaned.clone(), |rest| rest.trim_start().to_owned())
        } else {
            cleaned
        };
        Self {
            part: reading.part,
            body,
        }
    }
}

/// An event just added, with its one step of undo. It goes at the next thing the DM does.
/// The timed recording words (`Recording saved`, `Hearing sound`) as of the last owner update,
/// with a revision that changes only when one comes or goes.
struct TimedWords(design::memo::Revised<(bool, bool)>);

impl Default for TimedWords {
    fn default() -> Self {
        Self(design::memo::Revised::new((false, false)))
    }
}

/// Something failed where the DM may not be looking. It stays until they dismiss it.
struct Alert {
    word: &'static str,
    words: String,
    session: Uuid,
    /// The recording stopped, so one press can start it again.
    can_record: bool,
    retry: Option<PathBuf>,
    /// The recorder's own words, for hover, when `words` puts them plainly.
    said: Option<String>,
}

impl Alert {
    /// The input is gone, so the DM must choose another before recording again.
    fn missing_input(&self) -> bool {
        self.said
            .as_deref()
            .unwrap_or(&self.words)
            .to_lowercase()
            .contains(MISSING_INPUT_ERROR)
    }
}

struct Live {
    session: Uuid,
    label: String,
    recorder: Recorder,
}

/// Whether a session is being played (W14), and which: the answer of `played_session`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Played {
    /// No session is being played.
    No,
    /// A fight is open or a recording is live, and no session note is current.
    NoNote,
    /// This session's note is being played.
    Session(Uuid),
}

/// One note a link update wrote: the draft before, the words after, and the whole file and
/// session id it held right after the write. Undo writes only into that same file.
struct LinkWrite {
    before: Draft,
    after: String,
    bytes: String,
    id: Option<Uuid>,
}

impl LinkWrite {
    /// The session the update wrote, by the note's own words (`Session 2: The Road`).
    fn within(&self) -> design::results::Name {
        use design::results::{Fallback, Name};
        super::parse_note(&self.before.path, &self.bytes)
            .and_then(|note| Name::new(note.label()))
            .unwrap_or(Name::fallback(Fallback::ThatSession))
    }
}

/// Why the Undo of a link update left one note as it was (M7, lane N0): the note's session, the
/// typed cause, which also gives the tone (R41), and the one next step. The result's line opens
/// with its refusal.
#[derive(Clone, Debug, PartialEq)]
struct LinkUndoRefused {
    within: design::results::Name,
    cause: design::results::Cause,
    recovery: design::results::Recovery,
}

impl LinkUndoRefused {
    /// The refusal of the Undo of the links that now say `title`. A blank title names no
    /// event, so the links are named by their session.
    fn refusal(self, title: &str) -> design::results::Refusal {
        use design::results::{FailedAct, Name, Refusal};
        let to = Name::new(title).unwrap_or_else(|| self.within.clone());
        Refusal::new(
            FailedAct::LinksNotPutBack {
                to,
                within: self.within,
            },
            self.cause,
            self.recovery,
        )
    }
}

/// Tests: what runs after a link update writes a note.
#[cfg(any(test, feature = "test-support"))]
type LinkWriteHook = Box<dyn FnMut(&Path)>;

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Tests: runs after a link update writes a note, to change the next one under it.
    static AFTER_LINK_WRITE: std::cell::RefCell<Option<LinkWriteHook>> =
        std::cell::RefCell::new(None);
    /// Tests: runs right before Undo reads a note it wrote, to replace it under the read.
    static BEFORE_UNDO_READ: std::cell::RefCell<Option<LinkWriteHook>> =
        std::cell::RefCell::new(None);
    /// Tests: how many notes `note_bodies` read.
    static NOTE_READS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}
/// A rename's offer to update the links in the notes: what it says, its action, and what it
/// would write.
pub struct LinkOffer {
    changes: Vec<(Draft, String)>,
    title: String,
    /// The old names the links say, and how many links would change.
    old: Vec<String>,
    count: usize,
}

impl LinkOffer {
    /// The old names the offer changes the links from.
    #[cfg(any(test, feature = "test-support"))]
    pub fn old(&self) -> &[String] {
        &self.old
    }

    /// The offer in a pane's typed footer: what changes, and the action that does it.
    pub fn notice<A>(&self, update: A) -> Notice<A> {
        Notice::new(crate::sentence!(
            "Change links from “{}” to “{}”.",
            self.old.join("” or “"),
            self.title
        ))
        .choice(Choice::named(
            crate::choice!("Update {}", design::count(self.count, "link", "links")),
            update,
        ))
    }
}

struct LinkUndoState {
    notes: Vec<(PathBuf, String)>,
    events: Vec<timeline::Event>,
}

#[derive(Default)]
pub struct Sessions {
    /// `<file> is now a separate session.` and its Undo.
    separate: design::Results<kept::Page, kept::Separate>,
    /// The DM showed or hid the party bar. Otherwise it shows while a session is played.
    pub party_bar: Option<bool>,
    /// The party has members, and the bar does not show, so the header offers it.
    pub party_hidden: bool,
    /// The DM hid the session list. It is the DM's choice for the app, saved with the window's
    /// state, and only the header's toggle (or `Ctrl+\`) changes it. The list still shows where
    /// it is the only thing to show (`list_hides`).
    pub list_hidden: bool,
    pub run_open: bool,
    campaign: Option<Campaign>,
    folder: Option<PathBuf>,
    /// Why the folder cannot be used, or why the last write to it failed.
    folder_error: Option<FolderFault>,
    /// The current folder's listing, for the Session folder form and its footer. See
    /// `FolderScan`.
    folder_scan: Option<FolderScan>,
    /// `Moved <counts> to <folder>.`, one per campaign (`kept::FolderMoved`).
    folder_moves: design::Results<kept::FolderLink, kept::FolderMoved>,
    /// Review of PR 274, item 1: the plan row the DM just added or acted on, until the row
    /// after it is in view above the footer (or a few draws pass), the draws it took, and the
    /// pass it was last drawn in.
    pub(super) row_reveal: Option<(String, u8, u64)>,
    /// When the link update's result was made.
    /// The link update's result, with its Undo.
    link_updates: design::Results<kept::Page, kept::LinkUpdate>,
    /// The plan's results whose data its own fields hold (a scene act, the removed group, the
    /// deleted run), by kind: their footer order and press tickets (`kept::PlanMark`).
    plan_marks: design::Results<kept::PlanOwner, kept::PlanMark>,
    folder_text: String,
    view: View,
    tab: Tab,
    /// The list of sessions stands beside this one, and it already gives the date.
    list_beside: bool,
    /// The note shows as a reader. A new or current session opens for writing.
    reading_note: bool,
    /// The event open in place in the plan.
    open_event: Option<String>,
    reveal_event: bool,
    open_planned: Option<String>,
    show_fight_log: bool,
    /// A saved run the DM deleted from the plan, with the removal action it replaces in the
    /// plan (the book's removal of that run's own event when the deletion happened). The
    /// deletion's result owns it: its Undo, or the plan's next note or event edit, ends it.
    deleted_run: Option<(crate::encounter_run::DeletedRun, Option<u64>)>,
    updated_links: Option<Vec<LinkWrite>>,
    link_undo_state: Option<LinkUndoState>,
    link_problem: Option<String>,
    /// A partial update or refused Undo keeps its result and remaining writes through the usual
    /// expiry so the DM can retry.
    link_partial: bool,
    removed_fight: Option<(Option<Uuid>, usize, String, NoteUndo)>,
    /// A journey's encounter checks in the plan.
    travel: plan::Travel,
    plan_notice_state: Option<(String, Vec<timeline::Event>)>,
    removed_event_in_plan: bool,
    /// What the removal's owner note looked like; a later edit of it ends the removal.
    removal_note: Option<RemovalNote>,
    /// How often the owner's note was looked for, which only a change may cause.
    #[cfg(any(test, feature = "test-support"))]
    removal_scans: usize,
    removed_group: Option<(Option<Uuid>, usize, String, String, f32, String)>,
    /// The plan's scenes: the Scene field and the results of scene acts.
    scenes: plan::Scenes,
    /// The rewards record (F7): each `Add rewards` result by its note and fight.
    rewards: plan::Rewards,
    /// The note's plan as rows, built when the note, the content or the encounters change.
    plan_model: plan::PlanModelCache,
    /// What the plan's closed rows read from their data, kept by its inputs' revisions.
    plan_projection: plan::ProjectionCache,
    /// The fight behind the open event, read once.
    /// The open played fight's summary, kept with the run's file as last found (or why not):
    /// a different answer from the folder reads it again (R41, round 30 review F2).
    fight: Option<plan::KeptFight>,
    /// What the session before the selected one left, read once for each selection.
    last_time: Option<(Uuid, Option<LastTime>)>,
    /// The results of carry-over acts, by the session each changed.
    carries: carry::Carries,
    notes: Vec<Note>,
    unreadable: Vec<String>,
    mentions: BTreeMap<Uuid, Vec<String>>,
    selected: Option<Uuid>,
    today: Option<String>,
    /// In a narrow page, the list stands in for the session.
    list_only: bool,
    templates_list_only: bool,
    /// Where new events and runs go, if the DM chose. Otherwise the newest session.
    events_to: Option<Option<Uuid>>,
    /// The session chosen when the app last closed, until its campaign opens.
    pub restored_session: Option<(Uuid, Option<Uuid>)>,
    fight_search: String,
    draft: Option<Draft>,
    form: Option<Form>,
    templates: Vec<Template>,
    template: Option<String>,
    template_draft: Option<Draft>,
    /// The kept answer of the open note's compare with a move result's words (layout PR 9b-C).
    same_moved: design::text_draft::KeptCompare,
    new_template: Option<String>,
    template_error: Option<String>,
    /// A deleted template's words, for its `Undo delete`.
    deleted_templates: design::Results<kept::Page, kept::DeletedTemplate>,
    /// The words a draft's warning dropped, as their result (`Undo reload`, `Undo discard`).
    dropped: design::Results<kept::DraftFile, kept::Dropped>,
    recordings: Vec<Recording>,
    live: Option<Live>,
    confirm_record: bool,
    /// The page's one notice (#597): what the last act or a job said, in its tone, the session
    /// it is about and whether it arrived by itself (`kept::PageNotice`). The store issues its
    /// order; a new notice replaces it.
    notices: design::Results<kept::Page, kept::PageNotice>,
    /// A job ended while the DM may be elsewhere.
    job_done: bool,
    /// The latest result stays after its completion notice is acknowledged.
    processed_session: Option<Uuid>,
    /// The recording that result came from, so deleting it clears the result.
    processed_recording: Option<PathBuf>,
    /// The latest result has something the DM must look at.
    job_warned: bool,
    alerts: Vec<Alert>,
    /// The plan's Add result (`kept::PlanAdded`), with one step of undo.
    added: design::Results<kept::OfSession, kept::PlanAdded>,
    /// Its footer line, kept as data across frames (ruling L9): made again only when the store,
    /// the book or the saved-run facts changed.
    added_view: design::results::Retained<kept::OfSession, SessionAct, plan::AddedInputs>,
    /// The book's revision at the last check that the events hold what the Add left.
    added_seen: Option<design::memo::Revision>,
    /// A marker's `Add` result, by the session whose recording holds the marker.
    markers: design::Results<kept::OfSession, kept::MarkerAdd>,
    /// `Recording saved: …`, by the session whose recording it names.
    saved: design::Results<kept::OfSession, RecordingSaved>,
    notice_context: Option<(Option<Uuid>, View)>,
    job: Option<(Job, Uuid)>,
    reading: Option<Reading>,
    confirm_delete: Option<PathBuf>,
    confirm_again: Option<PathBuf>,
    pub notes_open: bool,
    focus_notes: bool,
    /// Back to the note, with its caret where it was.
    refocus_note: bool,
    focus_form_title: bool,
    /// The open run's name, and the session its event is in.
    pub open_run: Option<(String, Uuid)>,
    /// Words typed elsewhere that belong at the note's caret.
    words_for_note: String,
    words_new_line: bool,
    /// A step of the run came between. The next word typed starts a line of its own.
    break_before_words: bool,
    move_run_here: Option<(Uuid, String)>,
    /// The open run moved to another session, with `Undo move`.
    moves: design::Results<kept::Page, kept::Moved>,
    /// When a recording last stopped, and the campaign it was recorded in: the owner, so the
    /// rail says it was kept only while that campaign is open (another campaign's rail never
    /// says it). The owner update (`settle_words`) ends it `SAVED_WORD_FOR` after, on the frame
    /// clock (`crate::frame_clock`); no reader reads the clock.
    saved_at: Option<(Option<Uuid>, f64)>,
    /// The timed recording words as of the last owner update (`Recording saved` shown, `Hearing
    /// sound` shown), with their revision.
    timed_words: TimedWords,
    note_had_focus: bool,
    /// The caret to bring into view, on the frame after the editor has laid out.
    caret_to_show: Option<usize>,
    /// The Notes panel's slice of the open note (`PanelSlice`).
    panel_slice: Option<PanelSlice>,
    note_scroll_to_top: bool,
    last_check: Option<Instant>,
    last_scan: Option<Instant>,
    /// Row 123: a count that changes when a listing of the folder (`refresh`) finds other notes
    /// (an id, a file or a label), so a plan line's owner is checked against the notes only then.
    pub(super) notes_revision: u64,
    /// The campaign's manifest changed, so what holds a copy of it must reload.
    pub manifest_changed: Option<PathBuf>,
    /// The Sessions page is showing, so a button there need not lead here.
    pub on_page: bool,
    /// A modal is open over the app (the close prompt or the copy dialog, set by the frame
    /// logic). Ctrl+S waits for it, as every shortcut does.
    pub modal: bool,
    pub wants_processing_settings: bool,
    pub wants_speech_key: bool,
    pub wants_recording_input: bool,
    /// The DM asked to see the session this is about.
    pub wants_sessions_page: bool,
    pub wants_event: Option<String>,
    /// The encounter to start, by its path.
    pub wants_fight: Option<String>,
    /// A planned fight's `Edit encounter`: its encounter, by its ID, opens in the editor, whose
    /// way back is this session.
    pub wants_edit_encounter: Option<Uuid>,
    pub wants_campaign_link: bool,
    pub wants_runs_refresh: bool,
    pub wants_new_fight: bool,
    pub wants_new_group: bool,
    /// The plan footer's `Undo discard` of a plan group, by its line's ticket
    /// (`Encounters::undo_group_discard_in_plan`).
    pub wants_group_undo: Option<design::results::Ticket>,
    /// The session whose plan a group is being assembled in. It is the one that gets the link.
    group_session: Option<Uuid>,
    /// B4 review F1: the plan's group whose name the DM pressed while an unsaved draft refuses
    /// to open it: the session and the group's stable id (B4b N2: not its path, which another
    /// group can take). The plan paints the refusing reason under that group's line. It ends at
    /// the next press of a group's name; when the plan's groups are read and that group is not
    /// among them (removed, renamed away, its file replaced by another group); and, in
    /// `lib.rs` while it is stored, when nothing unsaved refuses any more, another session is
    /// selected, or a new read of the campaign lost the group (B4c; that check is cached by
    /// `Encounters::plan_refusal_stands`, so an unchanged frame compares no draft).
    pub plan_group_refused: Option<(Option<Uuid>, Uuid)>,
    /// Notes that kept a group's old name, because they could not be written.
    rename_problem: Option<String>,
    /// The open fight's Arrivals panel, for a planned group that has come.
    pub wants_arrivals: bool,
    /// The path of a group to edit in the plan.
    pub wants_group: Option<String>,
    group_search: String,
    /// What tonight's note links, kept while another session's note is open.
    tonight_links: Option<(Uuid, Vec<String>)>,
    pub wants_back_to_run: bool,
    /// A saved run to open, to read in full.
    pub wants_saved_run: Option<PathBuf>,
    /// A name in a note's link to read in Lookup.
    pub wants_lookup: Option<String>,
    /// A scene's NPC or creature, by its ID, to start a fight that nobody planned with.
    pub wants_start_fight: Option<crate::world::StartFight>,
    /// A done scene's entry, by its ID, to open in Lookup with its Log field and these words.
    pub wants_log: Option<(Uuid, String, String)>,
    /// A command wrote a file, so Lookup must read the campaign again.
    pub wants_content_refresh: bool,
    /// The session notes came, went or moved: Lookup's index reads them again.
    pub wants_index_refresh: bool,
}

/// A page notice that is a fault: an act that failed, or one the DM must check. Its words are
/// the page's own sentence; the kind gives the tone.
enum PageFault {
    Failed(Sentence),
    Check(Sentence),
}

impl Fault for PageFault {
    fn severity(&self) -> Severity {
        match self {
            Self::Failed(_) => Severity::NotSaved,
            Self::Check(_) => Severity::NeedsFixing,
        }
    }
    fn cause(&self) -> Sentence {
        match self {
            Self::Failed(words) | Self::Check(words) => words.clone(),
        }
    }
}

use splat_content_ui::session_notes::notice_sentence;

/// The words a draft dropped (reloaded from disk, or discarded) and their Undo, in the footer
/// of the pane that holds the draft: the result its store keeps (M0437), with the store's order,
/// taken at the act, and its ticket. The Undo names the draft's note or template (`name`,
/// M0438, M0440).
fn add_dropped<'f>(
    frame: &'f PaneFrame,
    messages: &mut Messages<'f, SessionAct>,
    draft: &Draft,
    name: &str,
    dropped: &design::Results<kept::DraftFile, kept::Dropped>,
) {
    if draft.dropped.is_none() {
        return;
    }
    let Some(shown) = dropped.get(&kept::DraftFile {
        path: draft.path.clone(),
    }) else {
        return;
    };
    messages.kept(shown.outcome(frame, |dropped| {
        if dropped.reloaded {
            KeptOutcome::Notice(
                Notice::new(crate::sentence!(
                    "Reloaded from disk. Your previous draft is kept until you edit or the file changes again."
                )),
                UndoStep::of(Verb::Reload, name).pressed_as(SessionAct::UndoDrop),
            )
        } else {
            KeptOutcome::Notice(
                Notice::new(crate::sentence!("Draft discarded.")),
                UndoStep::of(Verb::Discard, name).pressed_as(SessionAct::UndoDrop),
            )
        }
    }));
}

/// After a draft's warning: a Reload or Discard that dropped the draft's words records their
/// result, at the act, so it lists as the newest.
fn keep_dropped(
    dropped: &mut design::Results<kept::DraftFile, kept::Dropped>,
    draft: &Draft,
    act: Option<DraftAct>,
) {
    if matches!(act, Some(DraftAct::Reload | DraftAct::Discard))
        && let Some(words) = &draft.dropped
    {
        dropped.record(kept::Dropped {
            file: kept::DraftFile {
                path: draft.path.clone(),
            },
            reloaded: matches!(words, Dropped::Reloaded(_)),
        });
    }
}

fn processing_llm(settings: &ProcessingSettings) -> String {
    let config = &settings.config;
    if config.provider == processing::Provider::CodexCli {
        format!("Codex ({})", config.llm_model.trim())
    } else {
        processing::host(&config.llm_url)
    }
}

fn processing_destination(settings: &ProcessingSettings) -> String {
    let config = &settings.config;
    let llm = processing_llm(settings);
    format!(
        "Process sends the audio and the session and campaign names to {} for a transcript. Then the transcript, this session’s name, notes and events go to {llm} twice: once to clean them, and once to check that work against the raw words.",
        processing::host(&config.speech_url)
    )
}

fn recording_destination(settings: &ProcessingSettings, recording: &Recording) -> String {
    if recording.transcript && processing::transcript_short_of(&recording.path).is_none() {
        format!(
            "Continue processing sends the raw transcript, this session’s name, notes and events to {}. The audio is not sent again.",
            processing_llm(settings)
        )
    } else {
        let destination = processing_destination(settings);
        if let Some((done, total)) = processing::transcript_short_of(&recording.path) {
            destination.replacen(
                "Process sends the audio",
                &format!(
                    "Continue processing sends the rest of the audio (parts {done} to {total})"
                ),
                1,
            )
        } else {
            destination
        }
    }
}

/// A clean result: the same status in the session list as on the recording's own row.
fn processed_clean() -> Status<'static> {
    Status {
        icon: icons::OK,
        word: "Processed",
        color: colors().result.success,
    }
}

pub fn processed_warning() -> Status<'static> {
    Status {
        icon: icons::WARN,
        word: "Processed, with a warning",
        color: colors().result.warning,
    }
}

/// R5: what a recording, its processing or its result shows now. One producer makes it
/// (`Sessions::recording_status`, and `Sessions::rail_recording_status` for the rail), and every
/// place that shows it (the rail, the list row, the Record line, the Recordings tab, the banner)
/// takes its colours from it: no screen picks the colour of a live recording. Red is a fault: a
/// live recording paints only its record icon in the `recording` token, its words in the text
/// colour (`StatusKind::Recording`).
#[derive(Clone, Copy, Debug)]
pub struct RecordingStatus {
    pub icon: &'static str,
    pub word: &'static str,
    look: RecordingLook,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RecordingLook {
    /// A status whose kind decides its icon's and its words' colours.
    Kind(design::StatusKind),
    /// The icon and the words in one colour: a fault, a warning, a result.
    One(Color),
}

impl RecordingStatus {
    /// A live recording that saves audio.
    fn live() -> Self {
        let kind = design::StatusKind::Recording;
        Self {
            icon: kind.icon(),
            word: "Recording",
            look: RecordingLook::Kind(kind),
        }
    }

    fn one(status: Status<'static>) -> Self {
        Self {
            icon: status.icon,
            word: status.word,
            look: RecordingLook::One(status.color),
        }
    }

    pub fn icon_color(self) -> Color {
        match self.look {
            RecordingLook::Kind(kind) => kind.icon_color(),
            RecordingLook::One(color) => color,
        }
    }

    pub fn word_color(self) -> Color {
        match self.look {
            RecordingLook::Kind(kind) => kind.word_color(),
            RecordingLook::One(color) => color,
        }
    }
}

fn words_to_read(body: &str, book: &Book) -> String {
    let _ = book;
    without_empty_sections(&crate::world::with_links(body, &Default::default()))
}

fn updated_event_links(body: &str, book: &Book) -> (String, usize) {
    let days = timeline::day_numbers(&book.events);
    let mut out = String::new();
    let mut copied = 0;
    let mut count = 0;
    timeline::each_reference(body, |start, end, id, label| {
        out.push_str(&body[copied..start]);
        if let Some((at, event)) = book
            .events
            .iter()
            .enumerate()
            .find(|(_, event)| event.id == id)
            && let Some(label) = label
            && book
                .renamed_links
                .get(id)
                .is_some_and(|old| old.iter().any(|old| old == label))
            && label != timeline::reference_label(event, days[at])
        {
            out.push_str(&timeline::reference(event, days[at]));
            count += 1;
        } else {
            out.push_str(&body[start..end]);
        }
        copied = end;
    });
    out.push_str(&body[copied..]);
    (out, count)
}

/// The lines of a section that hold prose, never a line made only of links.
fn prose(section: &str) -> Option<String> {
    let lines: Vec<_> = section
        .lines()
        .filter(|line| {
            let mut prose = line.trim().trim_start_matches(['-', '*', ' ']).to_owned();
            while let Some(start) = prose.find("[[") {
                let Some(end) = prose[start..].find("]]") else {
                    break;
                };
                prose.replace_range(start..start + end + 2, "");
            }
            !prose.trim().is_empty()
        })
        .collect();
    (!lines.is_empty()).then(|| plain_links(&lines.join("\n")))
}

/// What the Last time line reads of the last session: the prose under its `What happened`,
/// then its processed recap, the one source that carry-over reads too. When neither holds
/// words, the note's first written line, so the line still shows last week's words: also a
/// line before any section. `body` comes without the note's own title and date
/// (`without_own_heading`); the line is never a heading, the recap of the session before, or
/// the plan.
fn recap_preview(body: &str, processed: impl FnOnce() -> Option<String>) -> Option<String> {
    if let Some(words) = section_text(body, "What happened").and_then(|words| prose(&words)) {
        return Some(words);
    }
    if let Some(words) = processed() {
        return Some(words);
    }
    let mut heading = None;
    body.lines()
        .filter(|line| {
            if let Some(name) = line.strip_prefix("## ") {
                heading = Some(name.trim().to_owned());
                return false;
            }
            !line.starts_with('#')
                && heading
                    .as_deref()
                    .is_none_or(|name| name != "Recap of last time" && name != "Plan for tonight")
        })
        .find_map(|line| {
            let line = super::task_parts(line.trim()).map_or(line, |(_, words)| words);
            prose(line.trim_start_matches(['-', '*', '+', ' ']))
        })
}

/// Keep plan links in the file, but do not repeat fights, groups and scenes in Read. `scenes`
/// are the scenes' whole lines, as the plan model found them.
fn compact_plan_for_reading(
    body: &str,
    fights: &[PlanFight],
    groups: &[std::sync::Arc<crate::encounter_run::RunGroup>],
    scenes: &[&str],
    beside: bool,
) -> String {
    let Some(plan) = section_text(body, "Plan for tonight") else {
        return body.to_owned();
    };
    // A line planned from a journey's check ends with its mark, which no reader shows.
    let lines: Vec<_> = plan
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| crate::timeline::checks::marked(line).words)
        .collect();
    let only_planned_links = !lines.is_empty()
        && lines.iter().all(|line| {
            // A scene's line starts at the line's start; a nested copy of it is note text.
            if scenes.iter().any(|scene| *scene == line.trim_end()) {
                return true;
            }
            let link = line
                .trim()
                .strip_prefix("- ")
                .or_else(|| line.trim().strip_prefix("* "));
            link.and_then(|link| {
                link.strip_prefix("[[")
                    .and_then(|link| link.strip_suffix("]]"))
            })
            .is_some_and(|name| {
                let name = name.split('|').next().unwrap_or(name);
                fights
                    .iter()
                    .any(|fight| fight.name.eq_ignore_ascii_case(name))
                    || groups
                        .iter()
                        .any(|group| group.name.eq_ignore_ascii_case(name))
            })
        });
    if !only_planned_links {
        return body.to_owned();
    }
    let fight_count = lines
        .iter()
        .filter(|line| {
            let name = line
                .trim()
                .trim_start_matches(['-', '*', ' '])
                .trim_start_matches("[[")
                .trim_end_matches("]]")
                .split('|')
                .next()
                .unwrap_or("");
            fights
                .iter()
                .any(|fight| fight.name.eq_ignore_ascii_case(name))
        })
        .count();
    // The working plan lists a linked group once, even when its link is repeated.
    let group_count = linked_names(&plan)
        .iter()
        .filter(|name| {
            groups
                .iter()
                .any(|group| group.name.eq_ignore_ascii_case(name))
                && !fights
                    .iter()
                    .any(|fight| fight.name.eq_ignore_ascii_case(name))
        })
        .count();
    let mut counts = [
        (fight_count, "fight", "fights"),
        (group_count, "group", "groups"),
        (scenes.len(), "scene", "scenes"),
    ]
    .into_iter()
    .filter(|(count, _, _)| *count > 0)
    .map(|(count, one, many)| design::count(count, one, many))
    .collect::<Vec<_>>();
    let last = counts.pop().unwrap_or_default();
    let words = if counts.is_empty() {
        format!("{last}, in the plan")
    } else {
        format!("{} and {last}, in the plan", counts.join(", "))
    };
    let heading = body
        .split_inclusive('\n')
        .scan(0, |offset, line| {
            let at = *offset;
            *offset += line.len();
            Some((at, line))
        })
        .find(|(_, line)| line.trim_end() == "## Plan for tonight")
        .map_or(0, |(at, _)| at);
    let start = heading + body[heading..].find(&plan).expect("the plan section");
    let mut reading = body.to_owned();
    if beside {
        reading.replace_range(heading..start + plan.len(), "");
    } else {
        reading.replace_range(start..start + plan.len(), &words);
    }
    reading
}

fn has_written_text(body: &str) -> bool {
    body.lines().any(|line| {
        let line = line.trim();
        !line.is_empty() && !line.starts_with('#') && !matches!(line, "-" | "*" | "---")
    })
}

/// A session that is not tonight's: the session being played is dated before today, or the open
/// session is not the one being played. Nothing is lost while it waits for the DM's act, but a
/// forgotten `New session` files tonight's notes and audio in the wrong place, and the rail
/// already marks it as a warning; so every place that says it is a warning (R41; the design
/// system's "the warning that this is not the session being played or that its date has
/// passed"). This is the one value that the Session footer, the Notes panel's footer, the Record
/// step and the rail's `Old session` read.
pub struct NotTonight;

impl NotTonight {
    /// The tone of every place that says it.
    const TONE: Tone = Tone::Warning;

    /// A footer's notice of it, in its tone.
    fn notice<A>(words: Sentence) -> Notice<A> {
        match Self::TONE {
            Tone::Warning => Notice::warning(words),
            _ => Notice::new(words),
        }
    }

    /// The rail's mark while a run files itself under a session dated before today.
    pub fn rail() -> Status<'static> {
        let (icon, color) = match Self::TONE {
            Tone::Warning => (icons::WARN, colors().result.warning),
            _ => (icons::INFO, colors().result.info),
        };
        Status {
            icon,
            word: "Old session",
            color,
        }
    }
}

/// The words for a note that looks like last week's. A forgotten `New session` puts
/// tonight's notes and audio there, and neither has a way to move afterward.
fn dated_before_today(note: &Note, today: &str) -> Option<String> {
    (note.date.as_str() < today).then(|| {
        format!(
            "This session is dated {}. Play it tonight, or start a new one.",
            note.date
        )
    })
}

/// The Notes panel's slice of the open note (lead, choice a): the note from its `## What
/// happened` on, which the panel's editor edits; the part before it stays as it is in the note
/// (whole on the Sessions page). Kept with the note's key it was cut from, so a quiet frame
/// cuts nothing, and written back as `prefix` and the slice when the slice changes.
struct PanelSlice {
    session: Option<Uuid>,
    body: design::text_draft::DraftKey,
    prefix: String,
    tail: TextDraft,
}

impl PanelSlice {
    /// Where the panel's part of the note starts: its `## What happened`, else the note's start.
    fn start(text: &str) -> usize {
        text.find("## What happened").unwrap_or(0)
    }

    fn of(session: Option<Uuid>, body: &TextDraft) -> Self {
        let text = body.as_str();
        let at = Self::start(text);
        Self {
            session,
            body: body.key(),
            prefix: text[..at].to_owned(),
            tail: TextDraft::new(&text[at..]),
        }
    }
}

/// Session numbers can repeat after a missing note is saved as a new file.
fn session_warning_names(open: &Note, destination: &Note) -> (String, String) {
    let name = |note: &Note| {
        if open.number == destination.number {
            note.path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        } else {
            format!("Session {}", note.number)
        }
    };
    (name(open), name(destination))
}

/// After Discard, a note with no file left: the editor shows its last saved words.
const DISCARDED_GONE: &str = "This file is gone from the folder. The editor shows its last saved words. Save them as a new file to keep them.";

/// A choice of a draft's warning (`draft_warning`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftAct {
    Overwrite,
    Reload,
    SaveAgain,
    SaveMoved,
    Discard,
    Save,
}

/// What a press of the draft's warning does (both UIs).
fn apply_draft_act(draft: &mut Draft, act: &DraftAct) {
    match act {
        DraftAct::Overwrite => draft.overwrite(),
        DraftAct::Reload => {
            if draft.reload_keeping_undo().is_err() {
                draft.state = DraftState::Missing;
            }
        }
        DraftAct::SaveAgain => draft.save_again(),
        DraftAct::SaveMoved => draft.save_moved(),
        DraftAct::Discard => draft.discard(),
        DraftAct::Save => draft.save(),
    }
}

/// A missing note's words. A known new place says why it is offered; with none, the file is
/// only gone.
fn missing_note_words(moved_to: Option<String>) -> String {
    moved_to.map_or_else(
        || "This file is gone from the folder. Your draft is still here. Choose where to save it, or discard it.".to_owned(),
        |path| format!("This note was moved or renamed. It is now {path}. Your draft is still here."),
    )
}

/// Round 30 `note-conflict-cross-page-banner` (SM2) and review F4 of PR 400: a session note's
/// warning stands in the bottom banner on every page, so it names its note, not `This file`,
/// which beside another page's own file is a wrong picture. `name` is the note's (`note_name`);
/// a template's warning has none and keeps its words. Each cause keeps its words and its tone.
fn owned_words(name: Option<&str>, words: &str) -> String {
    let Some(name) = name else {
        return words.to_owned();
    };
    let subject = format!("The note of {name}");
    if let Some(rest) = words
        .strip_prefix("This note ")
        .or_else(|| words.strip_prefix("This file "))
    {
        format!("{subject} {rest}")
    } else if let Some(rest) = words.strip_prefix("Not saved. ") {
        format!("{subject} was not saved. {rest}")
    } else {
        format!("{subject}: {words}")
    }
}

fn recording_recovery(missing_input: bool) -> &'static str {
    if missing_input {
        "Choose an input, then record again."
    } else {
        "Check the microphone, then record again."
    }
}

/// Plain words for an error the recorder is known to give. Its own words stay in hover.
fn plain_recorder_words(said: &str) -> Option<&'static str> {
    let said = said.to_lowercase();
    [
        (MISSING_INPUT_ERROR, "the microphone is no longer connected"),
        (
            "device or resource busy",
            "another program is using the microphone",
        ),
        (
            "permission denied",
            "the system did not let Splat use the microphone",
        ),
    ]
    .into_iter()
    .find(|(known, _)| said.contains(known))
    .map(|(_, plain)| plain)
}

/// The hover for a plain recorder error: what the recorder itself said.
fn recorder_said(said: &str) -> String {
    format!(
        "The recorder said: {}.",
        said.trim().trim_end_matches(['.', '!', '?'])
    )
}

fn recording_detail(words: &str, missing_input: bool) -> String {
    if let Some(cancelled) = words.strip_prefix("Processing cancelled: ") {
        if let Some(tail) = cancelled.strip_prefix("Transcribing part ") {
            let part = tail.split('.').next().unwrap_or(tail);
            return format!("Cancelled at part {part}. Continue processing goes on from there.");
        }
        let step = cancelled.split(". Cancelled.").next().unwrap_or(cancelled);
        return format!("Cancelled during {step}. Continue processing goes on from there.");
    }
    if let Some(said) = words.strip_prefix("Stopped by itself:") {
        let said = said.trim().trim_end_matches('.');
        return format!(
            "Stopped by itself: {}. {}",
            plain_recorder_words(said).unwrap_or(said),
            recording_recovery(missing_input)
        );
    }
    let detail = words.strip_prefix("Failed: ").unwrap_or(words);
    if let Some((step, rest)) = detail.split_once(", ")
        && let Some((elapsed, error)) = rest.split_once(". ")
        && elapsed.ends_with(" s")
    {
        return format!(
            "Stopped at {}, after {elapsed}. {error}",
            step.replacen(": ", ", ", 1).to_lowercase()
        );
    }
    detail.to_owned()
}

fn clock(seconds: u64) -> String {
    if seconds < 3600 {
        format!("{}:{:02}", seconds / 60, seconds % 60)
    } else {
        format!(
            "{}:{:02}:{:02}",
            seconds / 3600,
            (seconds / 60) % 60,
            seconds % 60
        )
    }
}

/// A length that cannot be read as a time of day.
fn spoken_length(seconds: u64) -> String {
    match (seconds / 3600, (seconds / 60) % 60) {
        (0, 0) => format!("{seconds} s"),
        (0, minutes) => format!("{minutes} min"),
        (hours, minutes) => format!("{hours} h {minutes} min"),
    }
}

fn megabytes(bytes: u64) -> String {
    format!("{:.0} MB", bytes as f64 / 1_000_000.0)
}

impl Sessions {
    /// A removed event's result and Undo belong to the event's session. An edit of that
    /// session's note ends them, from any page; another note's edit does not. Book edits and
    /// combat that begins in that session end them in the book itself. This runs every frame,
    /// so it looks at the notes and the words only when the removal or the draft changed.
    /// A dropped-words result ends with the words: at an edit, at its Undo or at another change
    /// of the draft's file, the draft lets them go (`Draft::dropped`). Read on every frame, it
    /// changes the store only at that change.
    fn end_dropped(&mut self) {
        if self.dropped.is_empty() {
            return;
        }
        for draft in [self.draft.as_ref(), self.template_draft.as_ref()]
            .into_iter()
            .flatten()
        {
            let file = kept::DraftFile {
                path: draft.path.clone(),
            };
            if draft.dropped.is_none() && self.dropped.get(&file).is_some() {
                self.dropped.owner_changed(&file);
            }
        }
    }

    fn end_removal_on_note_edit(&mut self, book: &mut Book) {
        let Some(removed) = book.removed_event() else {
            self.removal_note = None;
            return;
        };
        let revision = self.draft.as_ref().map(Draft::revision);
        let same_removal = self
            .removal_note
            .as_ref()
            .is_some_and(|seen| seen.id == removed.id);
        if same_removal
            && self
                .removal_note
                .as_ref()
                .is_some_and(|seen| seen.revision == revision)
        {
            return;
        }
        #[cfg(any(test, feature = "test-support"))]
        {
            self.removal_scans += 1;
        }
        let owner = removed.session;
        let id = removed.id.clone();
        // The owner's words, while its note is the one open.
        let words = self.draft.as_ref().filter(|draft| {
            owner.is_some()
                && self
                    .notes
                    .iter()
                    .any(|note| note.path == draft.path && Some(note.id) == owner)
        });
        match self.removal_note.as_mut().filter(|_| same_removal) {
            Some(seen) => {
                match (&seen.words, words) {
                    (Some(before), Some(draft))
                        if !draft.body.same_as(before.revision(), before, &seen.same) =>
                    {
                        book.forget_removed();
                        self.removal_note = None;
                        return;
                    }
                    (None, Some(draft)) => {
                        seen.words =
                            Some(design::memo::Revised::new(draft.body.as_str().to_owned()));
                    }
                    _ => {}
                }
                seen.revision = revision;
            }
            None => {
                self.removal_note = Some(RemovalNote {
                    id,
                    revision,
                    words: words
                        .map(|draft| design::memo::Revised::new(draft.body.as_str().to_owned())),
                    same: design::text_draft::KeptCompare::default(),
                });
            }
        }
    }

    /// Follow the active campaign, watch the recorder and the job, and save what is due.
    pub fn poll(&mut self, ctx: &StateContext, active: Option<&Campaign>, book: &mut Book) {
        if self.campaign.as_ref().map(|c| (c.id, &c.path)) != active.map(|c| (c.id, &c.path)) {
            self.open_campaign(active);
        }
        book.session_links = self
            .notes
            .iter()
            .map(|note| (note.id, note.link()))
            .collect();
        book.open(self.folder.as_deref());
        self.end_removal_on_note_edit(book);
        self.end_dropped();
        if let Some(live) = &mut self.live {
            live.recorder.poll();
            if let Health::Stopped(words) = live.recorder.health().clone() {
                let label = live.label.clone();
                let session = live.session;
                let kept = if self.stop_recorder(false) {
                    "What it captured is kept."
                } else {
                    "Nothing was captured."
                };
                self.raise(Alert {
                    session,
                    can_record: true,
                    retry: None,
                    word: "Recording stopped",
                    words: match plain_recorder_words(&words) {
                        Some(plain) => {
                            format!("The recording of “{label}” stopped by itself: {plain}. {kept}")
                        }
                        None => format!(
                            "The recording of “{label}” stopped by itself. {kept} The recorder said: {}.",
                            words.trim_end_matches(['.', '!', '?'])
                        ),
                    },
                    said: plain_recorder_words(&words).map(|_| recorder_said(&words)),
                });
            } else {
                ctx.request_repaint_after(Duration::from_secs(1));
            }
        }
        if let Some((job, session)) = &mut self.job {
            let session = *session;
            if let Some(result) = job.poll() {
                self.finish_job(result, session);
            } else {
                ctx.request_repaint_after(Duration::from_millis(250));
            }
        }
        for draft in [&mut self.draft, &mut self.template_draft]
            .into_iter()
            .flatten()
        {
            if draft.state == DraftState::Unsaved {
                #[cfg(not(any(test, feature = "test-support")))]
                if draft.due_at(Instant::now()) {
                    draft.save();
                }
                ctx.request_repaint_after(Duration::from_millis(500));
            }
        }
        let now = Instant::now();
        if self
            .last_check
            .is_none_or(|at| now.duration_since(at) >= CHECK_DISK_EVERY)
        {
            self.last_check = Some(now);
            // A saved recording's result ends with its file, also while nothing records: one
            // look at each kept path at this bounded interval, never on every frame.
            self.saved.disk_changed(|saved| saved.path.exists());
            // The live file's length and size change, and it can appear after the first look.
            if self.live.as_ref().map(|live| live.session) == self.selected
                && self.selected.is_some()
            {
                self.refresh_recordings();
            }
            for draft in [&mut self.draft, &mut self.template_draft]
                .into_iter()
                .flatten()
            {
                draft.follow_disk();
            }
        }
        // Round 30 re-check N1 of PR 400: a kept `Recording saved: …` must end when its file goes
        // also with no input, so while one is kept the app wakes for the next disk check. Only
        // then: with nothing kept, this asks for no frame.
        if !self.saved.is_empty() {
            let since = self
                .last_check
                .map_or(CHECK_DISK_EVERY, |at| now.duration_since(at));
            ctx.request_repaint_after(CHECK_DISK_EVERY.saturating_sub(since));
        }
        if let Some(id) = self.draft.as_mut().and_then(|draft| draft.restored.take()) {
            self.refresh();
            self.selected = Some(id);
            self.refresh_recordings();
        }
        // Notes can be renamed or moved in Obsidian, so the list follows the folder.
        if self
            .last_scan
            .is_none_or(|at| now.duration_since(at) >= SCAN_EVERY)
        {
            self.last_scan = Some(now);
            self.scan_folder();
        }
        book.session_links = self
            .notes
            .iter()
            .map(|note| (note.id, note.link()))
            .collect();
        // Row 123: the note's own events end a plan line's result, and its journey's removal
        // ends a refused Undo's answer, on every page.
        self.follow_plan_line();
        self.end_plan_answers_of_lost_journeys(book);
        book.poll(ctx);
        // After the book's last update this frame, so the Add result is checked against the
        // book its footer will read.
        self.settle_timed(ctx, book);
    }

    fn recording_row_here(&self, session: Uuid) -> bool {
        self.on_page
            && self.selected == Some(session)
            && self.tab == Tab::Recordings
            && self.view == View::Notes
            && self.form.is_none()
            && (self.list_beside || !self.list_only)
            && self.reading.is_none()
    }

    fn stalled_elsewhere(&self) -> Option<&Live> {
        self.live.as_ref().filter(|live| {
            !self.recording_row_here(live.session)
                && matches!(live.recorder.health(), Health::Stalled | Health::Stopped(_))
        })
    }

    /// Round 30 `record-again-repeat` (FG8, SG6, AG5): the one row that offers `Record again`.
    /// One press starts one new file, so the act stands once: on the newest row whose recording
    /// stopped by itself (live and stalled, or stored as stopped). An older stopped row keeps its
    /// state, its words and `Process`, and does not look tied to the new file.
    fn record_again_row(&self) -> Option<usize> {
        self.recordings.iter().position(|recording| {
            let live = self
                .live
                .as_ref()
                .filter(|live| live.recorder.path() == recording.path);
            match live {
                Some(live) => {
                    matches!(live.recorder.health(), Health::Stalled | Health::Stopped(_))
                }
                None => {
                    recording
                        .status
                        .as_deref()
                        .is_some_and(|words| words.starts_with("Stopped by itself:"))
                        && self
                            .job
                            .as_ref()
                            .is_none_or(|(job, _)| job.recording != recording.path)
                }
            }
        })
    }

    fn recording_recovery_row(&self) -> Option<usize> {
        self.recordings.iter().position(|recording| {
            let stalled = self.live.as_ref().is_some_and(|live| {
                live.recorder.path() == recording.path
                    && matches!(live.recorder.health(), Health::Stalled | Health::Stopped(_))
            });
            stalled
                || (self.live.is_none()
                    && self.job.is_none()
                    && recording
                        .status
                        .as_deref()
                        .is_some_and(|words| words.starts_with("Stopped by itself:")))
        })
    }

    /// One enabled recovery action owns primary across the composed screen.
    fn banner_primary_owner(&self, settings: &ProcessingSettings, run_open: bool) -> Option<usize> {
        if run_open || self.confirm_record {
            return None;
        }
        if self.stalled_elsewhere().is_some() {
            return Some(usize::MAX);
        }
        let visible = |alert: &&Alert| !self.recording_row_here(alert.session);
        self.alerts
            .iter()
            .enumerate()
            .find(|(_, alert)| {
                visible(alert)
                    && alert.can_record
                    && (settings.input_missing()
                        || alert.missing_input()
                        || (self.live.is_none() && self.job.is_none()))
            })
            .or_else(|| {
                if self
                    .selected
                    .is_some_and(|session| self.recording_row_here(session))
                    && self.recording_recovery_row().is_some()
                {
                    return None;
                }
                self.alerts.iter().enumerate().find(|(_, alert)| {
                    visible(alert)
                        && alert.retry.as_ref().is_some_and(|path| path.exists())
                        && self.live.is_none()
                        && self.job.is_none()
                        && settings.problem().is_none()
                })
            })
            .map(|(index, _)| index)
    }

    /// The notice that arrived on its own is still the one to show.
    fn notice_arrived(&self) -> bool {
        self.notice().is_some_and(|notice| notice.arrived)
    }

    /// Round 26 FM7: the session's own Recordings tab shows the processed row, so the DM has
    /// read the `is processed` notice there. Its banner does not show, and the view ends it.
    fn processed_notice_read_here(&self) -> bool {
        self.on_page
            && self.view == View::Notes
            && self.form.is_none()
            && self.tab == Tab::Recordings
            && self.notice_arrived()
            && self.notice_session() == self.selected
    }

    fn has_banner_notices(&self) -> bool {
        let notes = self.on_page && self.view == View::Notes && self.form.is_none();
        let selected = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id));
        (notes && self.other_recording_shown())
            || (notes
                && self.notice_arrived()
                && !self.processed_notice_read_here()
                && (self.notice_session().is_none() || self.notice_session() == self.selected))
            || (notes && selected.is_some_and(|note| !note.also.is_empty()))
            || (notes && !self.unreadable.is_empty())
            || self.draft.as_ref().is_some_and(Draft::needs_attention)
            || (self.on_page
                && self.view == View::Templates
                && self
                    .template_draft
                    .as_ref()
                    .is_some_and(Draft::needs_attention))
    }

    /// The session note's name for its warning: `Session 1: The Bridge`, or its file's name
    /// when the note is no longer listed.
    fn note_name(&self) -> Option<String> {
        let draft = self.draft.as_ref()?;
        Some(
            self.notes
                .iter()
                .find(|note| Some(note.id) == self.selected)
                .map_or_else(
                    || {
                        draft
                            .path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned()
                    },
                    Note::label,
                ),
        )
    }

    fn banner_has_primary(&self, settings: &ProcessingSettings, run_open: bool) -> bool {
        self.banner_primary_owner(settings, run_open).is_some()
    }

    fn scan_folder(&mut self) {
        // The folder's listing follows the disk also while the New session form is open; the
        // form and its unsent words are not refreshed.
        self.end_folder_listing();
        if self.form.is_some() {
            // The form's choices follow the folder, so a template removed outside shows its
            // reason before the press; the form's own words stay.
            if let Some(folder) = &self.folder {
                self.templates = list_templates(folder);
            }
            return;
        }
        self.refresh();
        if let (Some(id), Some(draft)) = (self.selected, &self.draft) {
            self.mentions
                .insert(id, timeline::referenced_ids(&draft.body));
            // A discarded or reloaded draft keeps its Undo until the DM uses it or types. The
            // scan does not follow the note to a new file under it, or the Undo would go.
            if !draft.unsaved() && draft.dropped.is_none() {
                match self.notes.iter().find(|note| note.id == id) {
                    Some(note) if note.path != draft.path => {
                        // The one note with this ID moved, so its Undo goes with it. A copy
                        // that claims the ID too is not followed.
                        let moved = note
                            .also
                            .is_empty()
                            .then(|| (draft.path.clone(), note.path.clone()));
                        self.draft = None;
                        self.select(Some(id));
                        if let Some((from, to)) = moved {
                            if let Some((.., undo)) = &mut self.removed_fight {
                                undo.moved(&from, &to);
                            }
                            self.scenes.moved(&from, &to);
                            self.carries.moved(&from, &to);
                        }
                    }
                    None => {
                        self.draft = None;
                        self.select(self.notes.first().map(|note| note.id));
                    }
                    Some(_) => {}
                }
            }
        }
    }

    fn finish_job(&mut self, result: Result<Option<String>, String>, session: Uuid) {
        let job = self.job.take();
        let failed_recording = job.as_ref().map(|(job, _)| job.recording.clone());
        let progress = job
            .as_ref()
            .map(|(job, _)| job.progress.words())
            .unwrap_or_default();
        if let Some(path) = &failed_recording {
            let words = match &result {
                Err(error) => Some(format!(
                    "{}: {progress}. {error}",
                    if error == processing::CANCELLED {
                        "Processing cancelled"
                    } else {
                        "Failed"
                    }
                )),
                _ => None,
            };
            if let Some(words) = words {
                let _ = super::replace_file(&path.with_extension("state.txt"), &words);
            } else {
                let _ = fs::remove_file(path.with_extension("state.txt"));
            }
        }
        let label = self
            .notes
            .iter()
            .find(|note| note.id == session)
            .map_or_else(|| "this session".to_owned(), Note::label);
        match result {
            Ok(None) => {
                self.processed_session = Some(session);
                self.processed_recording = failed_recording.clone();
                self.job_done = true;
                self.job_warned = false;
                self.say_arrived(
                    Tone::Success,
                    format!(
                        "“{label}” is processed. Read the result, and add the events you want to the timeline."
                    ),
                    Some(session),
                );
            }
            Ok(Some(warning)) => {
                self.processed_session = Some(session);
                self.processed_recording = failed_recording.clone();
                self.job_done = true;
                self.job_warned = true;
                // The recording's row has the words, for as long as the result stands.
                let _ = warning;
                self.end_notice();
            }
            Err(error) if error == processing::CANCELLED => {
                self.end_notice();
                self.raise(Alert {
                    session,
                    can_record: false,
                    retry: failed_recording,
                    said: None,
                    word: PROCESSING_CANCELLED,
                    words: format!("Processing cancelled. {}", processing::CONTINUE_ADVICE),
                });
            }
            Err(error) => {
                self.end_notice();
                self.raise(Alert {
                    session,
                    can_record: false,
                    retry: failed_recording,
                    said: None,
                    word: "Processing failed",
                    words: format!("Processing “{label}” failed. What is done is kept. {error}"),
                });
            }
        }
        if self.selected == Some(session) {
            self.refresh_recordings();
        }
    }

    fn open_campaign(&mut self, active: Option<&Campaign>) {
        self.flush();
        // A live recording blocks a campaign switch, so none is lost here.
        self.draft = None;
        self.template_draft = None;
        self.selected = None;
        self.template = None;
        self.reading = None;
        self.form = None;
        self.end_notice();
        self.deleted_run = None;
        self.removed_group = None;
        self.plan_notice_state = None;
        self.removed_event_in_plan = false;
        self.updated_links = None;
        self.link_updates.owner_changed(&kept::Page {
            root: self.folder.clone().unwrap_or_default(),
        });
        self.link_undo_state = None;
        self.removed_fight = None;
        self.scenes = plan::Scenes::default();
        self.rewards = plan::Rewards::default();
        // Their marks are the old campaign's: they end with its results (review of PR 629, P2).
        self.plan_marks = design::Results::new();
        // Row 123: a journey's acts, results and plan line are the campaign's: the change ends
        // them here, as it happens.
        self.travel = plan::Travel::default();
        // A saved recording's result is its campaign's: the change ends it, not only hides it.
        self.saved = design::Results::new();
        self.campaign = active.cloned();
        self.folder_error = None;
        match active {
            Some(campaign) => {
                self.folder_text =
                    folder_setting(campaign).unwrap_or_else(|_| DEFAULT_FOLDER.to_owned());
                match super::open_folder_typed(campaign) {
                    Ok(folder) => self.folder = Some(folder),
                    Err(fault) => {
                        self.folder = None;
                        self.folder_error = Some(fault);
                    }
                }
            }
            None => self.folder = None,
        }
        self.refresh();
        self.events_to = self
            .restored_session
            .take()
            .filter(|(campaign, _)| Some(*campaign) == active.map(|c| c.id))
            .map(|(_, session)| session);
        self.alerts.clear();
        self.processed_session = None;
        self.processed_recording = None;
        self.job_done = false;
        self.job_warned = false;
        // A folder move's result stays its campaign's (`folder_moves`): this campaign's scope
        // hides it, and coming back shows it again. The page's notice does not: the change
        // ends it (`end_notice` above), and coming back does not bring it back.
        self.select(self.current().or(self.notes.first().map(|note| note.id)));
    }

    /// A count that changes when a listing of the folder finds other notes (an id, a file or a
    /// label): what `links` says changes only with it, but for the mentions.
    pub fn notes_revision(&self) -> u64 {
        self.notes_revision
    }

    /// The session the DM chose to play, when they chose.
    pub fn chosen_session(&self) -> Option<Option<Uuid>> {
        self.events_to
    }

    /// The end event of the folder's listing (`FolderScan`): the next draw lists the folder again.
    fn end_folder_listing(&mut self) {
        #[cfg(any(test, feature = "test-support"))]
        FOLDER_SCAN_ENDS.with(|ends| ends.set(ends.get() + 1));
        self.folder_scan = None;
    }

    pub fn refresh(&mut self) {
        // The disk event of the folder's listing and of a move's result: the next draw lists
        // the folder again, and a result whose folder is gone ends.
        self.end_folder_listing();
        self.folder_moves.disk_changed(|moved| moved.to.is_dir());
        let Some(folder) = &self.folder else {
            if !self.notes.is_empty() {
                self.notes_revision += 1;
            }
            self.notes.clear();
            self.mentions.clear();
            self.templates.clear();
            return;
        };
        if !folder.is_dir() {
            self.folder_error = Some(FolderFault::Missing(folder.clone()));
            return;
        }
        self.folder_error = None;
        let notes = list_notes(folder);
        // A new, renamed or removed note: the campaign's content holds session notes too, so
        // Lookup and Read's links find it. A new number or title too: Lookup and World name a
        // session by its label (round 28 FC8).
        let place = |notes: &[Note]| -> Vec<(Uuid, PathBuf, String)> {
            notes
                .iter()
                .map(|note| (note.id, note.path.clone(), note.label()))
                .collect()
        };
        let changed = place(&notes) != place(&self.notes);
        self.wants_index_refresh |= changed;
        if changed {
            // Row 123: a plan line's owners are checked again only now.
            self.notes_revision += 1;
        }
        self.notes = notes;
        // A new, renamed or removed note can change which titles are shared.
        self.last_time = None;
        self.carries.scanned();
        if let Err(error) = super::remember_notes(folder, &self.notes) {
            self.folder_error = Some(FolderFault::NotSaved(error));
        }
        self.unreadable = unreadable_notes(folder);
        self.templates = list_templates(folder);
        self.mentions = self
            .notes
            .iter()
            .filter_map(|note| {
                let text = fs::read_to_string(&note.path).ok()?;
                Some((note.id, timeline::referenced_ids(&text)))
            })
            .collect();
    }

    fn refresh_recordings(&mut self) {
        self.recordings = match (&self.folder, self.selected) {
            (Some(folder), Some(session)) => list_recordings(folder, session),
            _ => Vec::new(),
        };
        // A saved recording's result ends with its file, in whichever session it is.
        self.saved.disk_changed(|saved| saved.path.exists());
    }

    /// Save what can be saved. Called on the way to anything that can lose a draft.
    pub fn flush(&mut self) {
        for draft in [&mut self.draft, &mut self.template_draft]
            .into_iter()
            .flatten()
        {
            draft.save();
        }
    }

    pub fn folder_missing(&self) -> bool {
        matches!(self.folder_error, Some(FolderFault::Missing(_)))
            || self.folder.as_ref().is_some_and(|folder| !folder.is_dir())
    }

    /// The plan that assembles a group, on its Plan tab: where the rail's `Group draft` link
    /// takes a group that the plan owns (FF2), so that its Save still links it there.
    pub fn show_group_plan(&mut self) {
        if self.group_session.is_some() {
            self.select(self.group_session);
            self.tab = Tab::Plan;
        }
    }
    pub fn select(&mut self, session: Option<Uuid>) {
        self.flush();
        self.list_only = false;
        // Words that could not be saved stay on screen, with the choice of what to do.
        if self.draft.as_ref().is_some_and(Draft::needs_attention) {
            self.write_in_note();
            return;
        }
        // Already open, so there is nothing to read again.
        if session == self.selected && self.draft.is_some() {
            return;
        }
        // The panel is for the session being played. It must not follow the page to an
        // older note, or the next words typed at the table go there.
        if self.notes_open && session != self.current() {
            self.notes_open = false;
        }
        // The plan's scene results are the session's: another session ends them.
        if self.selected != session {
            self.scenes = plan::Scenes::default();
        }
        // The plan's Add result is the session's open plan's: a select ends it.
        self.end_plan_add();
        self.selected = session;
        self.caret_to_show = None;
        self.note_scroll_to_top = true;
        self.draft = session
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
            .and_then(|note| Draft::open(&note.path).ok());
        self.reading = None;
        self.open_event = None;
        self.fight = None;
        self.reading_note = session != self.current();
        if self.notice().is_some_and(|notice| notice.session.is_none()) {
            self.end_notice();
        }
        self.confirm_record = false;
        self.confirm_again = None;
        self.refresh_recordings();
    }

    /// Where new events and runs go: the newest session, unless the DM chose. Reading an
    /// older note does not change it, or tonight's run would file under last week.
    pub fn current(&self) -> Option<Uuid> {
        match self.events_to {
            Some(Some(id)) if self.notes.iter().any(|note| note.id == id) => Some(id),
            Some(None) => None,
            _ => self.notes.first().map(|note| note.id),
        }
    }

    /// A link to the note of the session being played, as a world entry's Log writes it. So
    /// what the DM changes between sessions, with no fight or recording, is not put on the
    /// last session.
    pub fn current_link(&mut self) -> Option<String> {
        let Played::Session(played) = self.played_session() else {
            return None;
        };
        let note = self.notes.iter().find(|note| note.id == played)?;
        note.path
            .file_stem()
            .map(|stem| format!("[[{}]]", stem.to_string_lossy()))
    }

    /// The session being played (W14): a fight is open, a recording is live, or the current
    /// session is not dated before today. Every reader of "the session being played" asks this
    /// one function.
    pub fn played_session(&mut self) -> Played {
        let today = self.today();
        let current = self.current();
        let dated = current
            .and_then(|current| self.notes.iter().find(|note| note.id == current))
            .is_some_and(|note| note.date.as_str() >= today.as_str());
        if !(self.run_open || self.live.is_some() || dated) {
            return Played::No;
        }
        current.map_or(Played::NoNote, Played::Session)
    }

    /// Today, as of when the app started. A game that passes midnight is still tonight.
    fn today(&mut self) -> String {
        self.today
            .get_or_insert_with(crate::hermetic::today)
            .clone()
    }

    /// The party bar shows: the DM said so, or a session is being played. That is an open
    /// fight, a live recording, or a current session that is not dated before today.
    pub fn party_bar_shows(&mut self) -> bool {
        if let Some(chosen) = self.party_bar {
            return chosen;
        }
        self.played_session() != Played::No
    }

    /// A session is open in the pane: the selected note is in the folder's list.
    fn session_open(&self) -> bool {
        self.selected
            .is_some_and(|id| self.notes.iter().any(|note| note.id == id))
    }

    /// The wide page leaves the list out: the DM hid it, and a session stands in its place.
    /// With no session open, the list is the only thing to show, so it shows; the DM's
    /// choice stays.
    fn list_hides(&self) -> bool {
        self.list_hidden && self.session_open()
    }

    /// The header offers the list's toggle: the notes view of a wide page, with a session open
    /// and the folder there. `wide` is the page's width test (`TWO_COLUMNS_FROM`).
    fn offers_list_toggle(&self, wide: bool) -> bool {
        wide && self.view == View::Notes
            && self.session_open()
            && self.folder.as_ref().is_some_and(|folder| folder.is_dir())
    }

    /// `Ctrl+\`: hide or show the session list, where the header offers its toggle. The width
    /// test is the last drawn page's. Returns whether the key does something here.
    pub fn list_toggle_key(&self) -> bool {
        self.offers_list_toggle(self.list_beside)
    }

    pub fn toggle_list(&mut self) {
        self.list_hidden = !self.list_hidden;
    }

    /// The words if new events go to a session that looks like an old one.
    pub fn current_dated(&mut self) -> Option<String> {
        let today = self.today();
        let current = self.current()?;
        let note = self.notes.iter().find(|note| note.id == current)?;
        dated_before_today(note, &today)
    }

    pub fn play_current(&mut self) {
        let current = self.current();
        if current != self.selected {
            self.select(current);
        }
        if current == self.selected {
            self.play_this_one();
        }
    }

    /// From anywhere: to the form for a new session.
    fn start_new_session(&mut self) {
        self.view = View::Notes;
        self.open_form();
        self.wants_sessions_page = true;
    }

    /// The DM prepared this session earlier, and is playing it now.
    fn play_this_one(&mut self) {
        let today = self.today();
        let confirming = self.confirm_record;
        let Some(note) = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
            .cloned()
        else {
            return;
        };
        self.flush();
        if self.draft.as_ref().is_some_and(Draft::needs_attention) {
            return;
        }
        match update_details(&note, &note.title, &today, note.number) {
            Ok(_) => {
                self.draft = None;
                self.refresh();
                let selected = self.selected;
                self.select(selected);
                // Pressed in the step before recording, that step is still what comes next.
                self.confirm_record = confirming;
                self.focus_notes = self.notes_open;
            }
            Err(error) => {
                self.say(Tone::Error, error, None);
            }
        }
    }

    /// The words for working in a session other than the one being played. Round 30
    /// `session-not-played-repeat` (FF9, FN15, SF4, AF6): each fact once, the destination first.
    /// The file that takes what the DM writes is named once, at the start; the session being
    /// played follows, by its file name only when the two share a number.
    fn not_being_played(&self) -> Option<Sentence> {
        if self.selected.is_none() || self.selected == self.current() {
            return None;
        }
        let note = self
            .notes
            .iter()
            .find(|note| Some(note.id) == self.selected)?;
        let played = self
            .notes
            .iter()
            .find(|note| Some(note.id) == self.current());
        let destination = note.path.file_name().unwrap_or_default().to_string_lossy();
        Some(match played {
            Some(played) => crate::sentence!(
                "What you write and add on this page goes into {}. The session being played is {}.",
                destination,
                session_warning_names(note, played).1
            ),
            None => crate::sentence!(
                "What you write and add on this page goes into {}. It is not the session being played.",
                destination
            ),
        })
    }

    /// `not_being_played`, or, while a group is being assembled in another session, where.
    fn not_being_played_words(&self, note: &Note, plan: &Plan<'_>) -> Option<Sentence> {
        let words = self.not_being_played()?;
        if plan.group_open
            && let Some(group) = self
                .notes
                .iter()
                .find(|note| Some(note.id) == self.group_session)
        {
            let (open, group) = session_warning_names(note, group);
            return Some(crate::sentence!(
                "{} is not the session being played. A group is being assembled in {}.",
                open,
                group
            ));
        }
        Some(words)
    }

    /// The DM chose where new events and runs go.
    pub fn set_current(&mut self, session: Option<Uuid>) {
        self.events_to = Some(session);
        if self.notes_open && session != self.selected {
            self.notes_open = false;
        }
    }

    pub fn links(&self) -> Vec<SessionLink> {
        self.notes
            .iter()
            .map(|note| SessionLink {
                id: note.id,
                label: note.label(),
                mentions: self.mentions.get(&note.id).cloned().unwrap_or_default(),
            })
            .collect()
    }

    pub fn open(&mut self, session: Uuid) {
        self.view = View::Notes;
        self.form = None;
        self.select(Some(session));
    }

    pub fn open_live_recording(&mut self) {
        if let Some(session) = self
            .live
            .as_ref()
            .map(|live| live.session)
            .or_else(|| self.job.as_ref().map(|(_, session)| *session))
            .or_else(|| self.alerts.last().map(|alert| alert.session))
            .or(self.processed_session)
            .or(self.notice_session())
            .or_else(|| {
                let root = self.folder.clone().unwrap_or_default();
                self.saved
                    .newest_first(&root)
                    .next()
                    .and_then(|saved| saved.value().owner.session)
            })
        {
            self.open(session);
            self.tab = Tab::Recordings;
        }
    }

    /// The session an Add result names (`Session 2`), from the session's own note.
    fn session_name(&self, session: Option<Uuid>) -> Option<String> {
        self.notes
            .iter()
            .find(|note| Some(note.id) == session)
            .map(|note| format!("Session {}", note.number))
    }

    /// Only this explicit action changes a note after an event rename.
    /// Round 26: a rename's link offer as words, its action's name and the changes it would
    /// write, for the Timeline's message footer. It reads the notes once, when it is asked.
    pub fn event_link_offer(&self, book: &Book, event: &str) -> Option<LinkOffer> {
        if book.renamed_links.is_empty() && self.updated_links.is_none() {
            return None;
        }
        let renamed = book.renamed_links.get(event)?;
        let current = book.events.iter().find(|item| item.id == event)?;
        let mut scoped = Book::default();
        scoped.events = book.events.clone();
        scoped
            .renamed_links
            .insert(event.to_owned(), renamed.clone());
        let mut changes = Vec::new();
        let mut count = 0;
        // The old names that the notes' links say: a title typed letter by letter passes many
        // names, and the offer names only those a link holds.
        let mut said = std::collections::BTreeSet::new();
        for note in &self.notes {
            let draft =
                if let Some(draft) = self.draft.as_ref().filter(|draft| draft.path == note.path) {
                    draft.clone()
                } else if let Ok(draft) = Draft::open(&note.path) {
                    draft
                } else {
                    continue;
                };
            let (body, found) = updated_event_links(&draft.body, &scoped);
            if found > 0 {
                count += found;
                timeline::each_reference(&draft.body, |_, _, id, label| {
                    if id == event
                        && let Some(label) = label
                    {
                        said.insert(label.to_owned());
                    }
                });
                changes.push((draft, body));
            }
        }
        // Issue #569: the result and a refused Undo name the links' own words (the reference
        // label, never blank), not the event's title, which a rename can leave blank.
        let days = timeline::day_numbers(&book.events);
        let day = book
            .events
            .iter()
            .position(|item| item.id == event)
            .map_or(0, |at| days[at]);
        (count > 0).then(|| LinkOffer {
            changes,
            title: timeline::reference_label(current, day),
            old: renamed
                .iter()
                .filter(|old| said.contains(*old))
                .cloned()
                .collect(),
            count,
        })
    }

    /// The DM took the link offer: write the notes, and keep what Undo needs.
    pub fn accept_event_link_offer(&mut self, offer: LinkOffer, book: &Book) {
        self.update_event_link_notes(offer.changes);
        if self.updated_links.is_some() {
            self.link_updates.record(kept::LinkUpdate {
                page: self.page(),
                title: offer.title.clone(),
            });
        }
        self.link_undo_state = Some(LinkUndoState {
            notes: self.note_bodies(),
            events: book.events.clone(),
        });
    }

    /// `Undo link update`, pressed on the result its ticket names. The result stays, with its
    /// Undo, while some notes keep the new links; it then opens with the typed refusal of the
    /// first note Undo left as it was (M7, lane N0): what was not put back, where, why, and the
    /// next step.
    pub fn undo_event_link_update(&mut self, ticket: design::results::Ticket) {
        if self
            .link_updates
            .get_by(&ticket)
            .is_none_or(|shown| shown.value().page != self.page())
        {
            return;
        }
        let refused = self.undo_event_link_notes();
        let _ = self
            .link_updates
            .undo_or_refuse(ticket, |update| match refused {
                None => Ok(()),
                Some(refused) => Err(refused.refusal(&update.title)),
            });
    }

    /// The link update's result with its Undo, from its store, for a pane's typed footer.
    pub fn event_link_line<'f, A: 'static>(
        &self,
        frame: &'f PaneFrame,
        undo: A,
    ) -> Option<design::outcome::KeptLine<'f, A>> {
        self.updated_links.as_ref()?;
        let shown = self.link_updates.get(&self.page())?;
        Some(shown.outcome(frame, |update| {
            Said::new(crate::sentence!("Links now say “{}”.", update.title))
                .undo(crate::library::LINK_UPDATE_UNDO.pressed_as(undo))
                .into()
        }))
    }

    /// Why the last link update or its Undo did not finish.
    pub fn event_link_problem(&self) -> Option<&str> {
        self.link_problem.as_deref()
    }

    fn note_bodies(&self) -> Vec<(PathBuf, String)> {
        self.notes
            .iter()
            .filter_map(|note| {
                #[cfg(any(test, feature = "test-support"))]
                NOTE_READS.with(|reads| reads.set(reads.get() + 1));
                let body = self
                    .draft
                    .as_ref()
                    .filter(|draft| draft.path == note.path)
                    .map(|draft| draft.body.as_str().to_owned())
                    .or_else(|| {
                        fs::read_to_string(&note.path)
                            .ok()
                            .map(|text| body_of(&text).to_owned())
                    })?;
                let disk = fs::read_to_string(&note.path).unwrap_or_default();
                Some((note.path.clone(), format!("{body}\0{disk}")))
            })
            .collect()
    }

    pub fn expire_event_link_undo(&mut self, on_timeline: bool, book: &Book) {
        // A partial update or refused Undo keeps its result while notes still need Undo.
        // Check those notes when the DM presses Undo.
        if self.link_partial && self.updated_links.is_some() {
            return;
        }
        let ends = !on_timeline || self.draft.as_ref().is_some_and(Draft::needs_attention);
        let changed = !ends
            && self.link_undo_state.as_ref().is_some_and(|state| {
                state.events != book.events || state.notes != self.note_bodies()
            });
        if ends || changed {
            self.updated_links = None;
            self.link_updates.owner_changed(&kept::Page {
                root: self.folder.clone().unwrap_or_default(),
            });
            self.link_undo_state = None;
            self.link_problem = None;
        }
    }

    fn update_event_link_notes(&mut self, changes: Vec<(Draft, String)>) {
        // A new update ends the last one's result: its warning, its Undo and its hold.
        self.link_problem = None;
        self.updated_links = None;
        self.link_updates.owner_changed(&kept::Page {
            root: self.folder.clone().unwrap_or_default(),
        });
        self.link_partial = false;
        // Every target is checked before the first write: its draft, the open draft of the same
        // note (its owner here), and its file on disk. One blocked target writes nothing.
        let blocked = changes.iter().find(|(before, _)| {
            before.needs_attention()
                || self
                    .draft
                    .as_ref()
                    .is_some_and(|open| open.path == before.path && open.needs_attention())
                || !before.file_unchanged()
        });
        if let Some((before, _)) = blocked {
            self.link_problem = Some(format!(
                "No links changed. {} changed or needs a choice first; resolve it and try again.",
                before
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
            ));
            // As before, a new update ends the last one's Undo.
            self.updated_links = None;
            self.link_updates.owner_changed(&kept::Page {
                root: self.folder.clone().unwrap_or_default(),
            });
            return;
        }
        // Each note is written by its own save, which checks the file once more right before
        // writing. A failure part way is not rolled back: the result names every note already
        // written, with the error, and Undo puts back exactly those.
        self.link_partial = false;
        let mut written = Vec::new();
        let mut failed = None;
        for (before, body) in changes {
            let mut next = before.clone();
            let _ = next.body.replace(body);
            next.edited();
            next.save();
            if next.unsaved() {
                let name = next
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                let why = match &next.state {
                    DraftState::Failed(error) => error.clone(),
                    _ => "It changed on disk.".to_owned(),
                };
                failed = Some(format!("{name} was not written. {why}"));
                break;
            }
            if let Some(open) = self.draft.as_ref().filter(|draft| draft.path == next.path) {
                // The act wrote a copy of the open note: the Notes panel follows it as the note's
                // own write.
                let mut copy = next.clone();
                copy.take_over_from(open, before.body.as_str());
                self.draft = Some(copy);
            }
            let bytes = next.file_text().unwrap_or_default().to_owned();
            written.push(LinkWrite {
                id: super::note_identity(&next.path, &bytes),
                before,
                after: next.body.as_str().to_owned(),
                bytes,
            });
            #[cfg(any(test, feature = "test-support"))]
            AFTER_LINK_WRITE.with(|hook| {
                if let Some(hook) = hook.borrow_mut().as_mut() {
                    hook(&next.path);
                }
            });
        }
        if let Some(failed) = failed {
            if written.is_empty() {
                self.link_problem = Some(format!("No links changed. {failed}"));
            } else {
                let names: Vec<String> = written
                    .iter()
                    .map(|write| {
                        write
                            .before
                            .path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned()
                    })
                    .collect();
                self.link_problem = Some(format!(
                    "Links changed only in {}. {failed} Undo link update puts {} back.",
                    names.join(", "),
                    if names.len() == 1 { "it" } else { "them" }
                ));
                self.link_partial = true;
            }
        }
        self.updated_links = (!written.is_empty()).then_some(written);
        self.last_time = None;
    }

    /// Put back what the last link update wrote, note by note. A note it cannot put back keeps
    /// the new links and its Undo; the first such note's typed reason comes back (lane N0).
    fn undo_event_link_notes(&mut self) -> Option<LinkUndoRefused> {
        use design::results::{Cause, File, Recovery, Step};
        let mut remaining = Vec::new();
        let mut refused = None;
        for write in self.updated_links.take().unwrap_or_default() {
            // Undo writes only into the file it wrote: the draft it saves through must hold the
            // same bytes and the same session, and `save` checks the file against them again
            // right before writing.
            #[cfg(any(test, feature = "test-support"))]
            BEFORE_UNDO_READ.with(|hook| {
                if let Some(hook) = hook.borrow_mut().as_mut() {
                    hook(&write.before.path);
                }
            });
            let current = self
                .draft
                .as_ref()
                .filter(|draft| draft.path == write.before.path)
                .cloned()
                .or_else(|| Draft::open(&write.before.path).ok());
            let same_file = |draft: &Draft| {
                draft.file_text() == Some(write.bytes.as_str())
                    && draft
                        .file_text()
                        .and_then(|text| super::note_identity(&draft.path, text))
                        == write.id
            };
            // The note's state picks the cause, and so the tone (R41): a save the system refused
            // is an error; an outside change, a missing file, another note or an edit since the
            // update is a warning.
            let by_state = |draft: &Draft| match &draft.state {
                DraftState::Failed(_) => Some((
                    Cause::NotSaved(File::SessionNote),
                    Recovery::First(Step::Check(File::SessionNote)),
                )),
                DraftState::Conflict => Some((
                    Cause::Conflict(File::SessionNote),
                    Recovery::First(Step::ChooseWords(File::SessionNote)),
                )),
                DraftState::Missing => Some((
                    Cause::Gone(File::SessionNote),
                    Recovery::First(Step::Check(File::SessionNote)),
                )),
                _ => None,
            };
            let why = match current {
                None if !write.before.path.exists() => Some((
                    Cause::Gone(File::SessionNote),
                    Recovery::First(Step::Check(File::SessionNote)),
                )),
                None => Some((
                    Cause::Unreadable(File::SessionNote),
                    Recovery::First(Step::Check(File::SessionNote)),
                )),
                Some(draft) if draft.needs_attention() => by_state(&draft),
                Some(draft)
                    if draft
                        .file_text()
                        .and_then(|text| super::note_identity(&draft.path, text))
                        != write.id =>
                {
                    Some((
                        Cause::Replaced(File::SessionNote),
                        Recovery::First(Step::Check(File::SessionNote)),
                    ))
                }
                Some(draft) if !same_file(&draft) || !draft.body.same_words(&write.after) => {
                    let within = write.within();
                    Some((
                        Cause::EditedSince(within.clone()),
                        Recovery::First(Step::Resolve(within)),
                    ))
                }
                Some(mut current) => {
                    let _ = current.body.replace(write.before.body.as_str());
                    current.edited();
                    current.save();
                    // A save that did not finish says why in the draft's state.
                    let why = current.unsaved().then(|| {
                        by_state(&current).unwrap_or((
                            Cause::NotSaved(File::SessionNote),
                            Recovery::First(Step::Check(File::SessionNote)),
                        ))
                    });
                    if let Some(open) = self
                        .draft
                        .as_ref()
                        .filter(|draft| draft.path == current.path)
                    {
                        // The Undo wrote a copy of the open note that held the update's words:
                        // the Notes panel follows it as the note's own write.
                        current.take_over_from(open, &write.after);
                        self.draft = Some(current);
                    }
                    why
                }
            };
            if let Some((cause, recovery)) = why {
                refused.get_or_insert(LinkUndoRefused {
                    within: write.within(),
                    cause,
                    recovery,
                });
                remaining.push(write);
            }
        }
        let still_written = !remaining.is_empty();
        if self.link_partial || still_written {
            // The Undo ends the update's warning. Any refusal is on the result itself.
            self.link_problem = None;
        }
        self.link_partial = still_written;
        self.updated_links = still_written.then_some(remaining);
        self.last_time = None;
        refused
    }

    pub fn recording(&self) -> bool {
        self.live.is_some()
    }

    /// The rail's recording status: the live state, else processing, a failure or a result.
    pub fn rail_recording_status(&self) -> Option<RecordingStatus> {
        let c = colors();
        if let Some(status) = self.recording_status() {
            return Some(status);
        }
        if self.processing() {
            return Some(RecordingStatus::one(Status {
                icon: icons::AUDIO,
                word: "Processing",
                color: c.connection.busy,
            }));
        }
        if let Some(word) = self.alert_words().last() {
            return Some(RecordingStatus::one(Status {
                icon: if word == PROCESSING_CANCELLED {
                    icons::WARN
                } else {
                    icons::ERROR
                },
                word,
                color: if word == PROCESSING_CANCELLED {
                    c.result.warning
                } else {
                    c.result.error
                },
            }));
        }
        if self.job_warned() {
            return Some(RecordingStatus::one(Status {
                word: "Processed",
                ..processed_warning()
            }));
        }
        if self.processed_session.is_some() {
            return Some(RecordingStatus::one(processed_clean()));
        }
        self.just_saved().then_some(RecordingStatus::one(Status {
            icon: icons::OK,
            word: "Recording saved",
            color: c.document.saved,
        }))
    }

    /// The same timed recording words on the rail and the session page.
    pub fn recording_detail(&self) -> Option<String> {
        let Some(live) = self.live.as_ref() else {
            return self
                .rail_recording_status()
                .map(|status| status.word.to_owned());
        };
        let word = self.recording_status()?.word;
        Some(
            if matches!(word, "Not saving audio" | "Recording silence") {
                word.to_owned()
            } else {
                format!("{} {}", word, clock(live.recorder.seconds()))
            },
        )
    }

    pub fn recording_hover(&self) -> Option<String> {
        if let Some((job, _)) = &self.job {
            return Some(format!("Processing · {}", job.progress.words()));
        }
        if let Some(live) = &self.live
            && *live.recorder.health() == Health::Silent
        {
            return Some(format!(
                "Recording silence · {} recorded",
                clock(live.recorder.seconds())
            ));
        }
        self.recording_detail()
    }

    /// The word the rail keeps in view. A failed recording cannot look like a live one.
    /// The live recording's state: the one producer of what every screen shows of it (R5).
    pub fn recording_status(&self) -> Option<RecordingStatus> {
        let c = colors();
        self.live.as_ref().map(|live| match live.recorder.health() {
            Health::Stalled | Health::Stopped(_) => RecordingStatus::one(Status {
                icon: icons::ERROR,
                word: "Not saving audio",
                color: c.result.error,
            }),
            Health::Silent => RecordingStatus::one(Status {
                icon: icons::WARN,
                word: "Recording silence",
                color: c.result.warning,
            }),
            Health::Starting | Health::Hearing => RecordingStatus::live(),
        })
    }

    /// A word for the run's own notice line. It takes no room of its own, so nothing moves.
    pub fn run_notice(&self) -> Option<&'static str> {
        self.live
            .as_ref()
            .filter(|live| *live.recorder.health() == Health::Silent)
            .map(|_| "Recording silence. Check the input in Settings.")
    }

    /// A recording stopped a moment ago, and its file is kept. Its owner update
    /// (`settle_words`) ends it on the frame clock; this reads no clock.
    pub fn just_saved(&self) -> bool {
        self.saved_at.is_some_and(|(campaign, _)| {
            campaign == self.campaign.as_ref().map(|campaign| campaign.id)
        })
    }

    /// The owner update of the page's timed results and words (ruling L9), at the end of
    /// `poll`, after the book's last update and before any page or rail reads them: each ends
    /// here, by the frame clock, and asks for a repaint at its next end, so it ends with no
    /// input. No reader reads the clock.
    fn settle_timed(&mut self, ctx: &StateContext, book: &Book) {
        self.settle_plan_add(ctx, book);
        self.settle_words(ctx);
    }

    /// The plan's Add result: its life ends it (`Results::expire`); so does a book whose events
    /// no longer hold what the add left (their count, its label), checked only when the book's
    /// revision changed (a reload, a file event, an edit), never on an unchanged frame.
    fn settle_plan_add(&mut self, ctx: &StateContext, book: &Book) {
        self.added.expire(ctx);
        let revision = book.revision();
        if self.added_seen == Some(revision) {
            return;
        }
        self.added_seen = Some(revision);
        if self.added.is_empty() {
            return;
        }
        let days = crate::timeline::day_numbers(&book.events);
        self.added.disk_changed(|added| {
            added.count == book.events.len()
                && book
                    .events
                    .iter()
                    .position(|event| event.id == added.id)
                    .is_some_and(|at| book.events[at].label(days[at]) == added.label)
        });
    }

    /// The plan's Add result ends: the DM's next act on the plan or its note, or another
    /// session.
    pub(super) fn end_plan_add(&mut self) {
        self.added.owner_changed(&self.of_session(self.selected));
    }

    /// `Recording saved` ends `SAVED_WORD_FOR` after the stop, and `Hearing sound`
    /// `recording::HEARD_FOR` after the last sound, on the frame clock; each asks for a repaint
    /// at its end while it stands. The revision of the two words changes only when one comes or
    /// goes.
    fn settle_words(&mut self, ctx: &StateContext) {
        if let Some((_, at)) = self.saved_at {
            if crate::frame_clock::passed(at, SAVED_WORD_FOR) {
                self.saved_at = None;
            } else {
                let left = SAVED_WORD_FOR.as_secs_f64() - (crate::frame_clock::now() - at);
                ctx.request_repaint_after(Duration::from_secs_f64(left.max(0.0)));
            }
        }
        let hearing = self.live.as_mut().is_some_and(|live| {
            if let Some(left) = live.recorder.expire() {
                ctx.request_repaint_after(left);
            }
            live.recorder.heard_just_now()
        });
        // The word as its readers show it: also its campaign's (`just_saved`).
        self.timed_words.0.set((self.just_saved(), hearing));
    }

    /// A job finished, and the DM has not looked at it yet.
    pub fn job_done(&self) -> bool {
        self.job_done
    }

    pub fn job_warned(&self) -> bool {
        self.job_warned
    }

    /// What failed, for the rail, until the DM dismisses it or tries again.
    pub fn alert_words(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.alerts.iter().map(|alert| alert.word)
    }

    /// One alert of each kind, so that a later failure cannot hide a stopped recording.
    fn raise(&mut self, alert: Alert) {
        self.alerts.retain(|old| old.can_record != alert.can_record);
        self.alerts.push(alert);
    }

    pub fn recording_label(&self) -> Option<&str> {
        self.live.as_ref().map(|live| live.label.as_str())
    }

    pub fn processing(&self) -> bool {
        self.job.is_some()
    }

    /// The session being processed, by name.
    pub fn processing_label(&self) -> Option<String> {
        let (_, session) = self.job.as_ref()?;
        self.notes
            .iter()
            .find(|note| note.id == *session)
            .map(Note::label)
    }

    /// Each draft that closing the app would lose, by name.
    pub fn not_saved(&self) -> Vec<String> {
        let mut items = Vec::new();
        if let Some(draft) = self.draft.as_ref().filter(|draft| draft.needs_attention()) {
            items.push(format!(
                "Notes in “{}” are not saved.",
                draft.path.file_stem().unwrap_or_default().to_string_lossy()
            ));
        }
        if let Some(draft) = self
            .template_draft
            .as_ref()
            .filter(|draft| draft.needs_attention())
        {
            items.push(format!(
                "Template “{}” is not saved.",
                draft.path.file_stem().unwrap_or_default().to_string_lossy()
            ));
        }
        items
    }

    pub fn notes_changed_on_disk(&self) -> bool {
        self.draft
            .as_ref()
            .is_some_and(|draft| draft.state == DraftState::Conflict)
    }

    pub fn busy_with_edit(&self) -> bool {
        !self.not_saved().is_empty()
    }

    /// Stop the recording, and arm the end of its `Recording saved` (`settle_words`), so a stop
    /// made after this frame's owner update still ends with no input.
    fn finish_recording(&mut self, ctx: &StateContext) {
        self.stop_recorder(true);
        self.settle_words(ctx);
    }

    /// Stop, and say what is true of the file. Returns whether it holds any audio.
    fn stop_recorder(&mut self, tell: bool) -> bool {
        let Some(live) = self.live.take() else {
            return false;
        };
        let (session, label) = (live.session, live.label.clone());
        let stopped = match live.recorder.health() {
            Health::Stopped(words) => Some(format!(
                "Stopped by itself: {}.",
                words.trim_end_matches(['.', '!', '?'])
            )),
            _ => None,
        };
        let path = live.recorder.stop();
        if let Some(words) = stopped {
            let _ = super::replace_file(&path.with_extension("state.txt"), &words);
        }
        let has_audio =
            crate::recording::Layout::read(&path).is_ok_and(|layout| layout.data_length > 0);
        if !has_audio {
            // An empty file is not a recording, and it must not look like one.
            let _ = fs::remove_file(&path);
        }
        if tell && has_audio {
            self.saved_at = Some((
                self.campaign.as_ref().map(|campaign| campaign.id),
                crate::frame_clock::now(),
            ));
            // The result is its recording's (`RecordingSaved`), not the page's one notice.
            self.saved.record(RecordingSaved {
                owner: self.of_session(Some(session)),
                path: path.clone(),
            });
        } else if tell {
            self.say(
                Tone::Warning,
                format!("Nothing was recorded for “{label}”."),
                Some(session),
            );
        }
        self.refresh_recordings();
        has_audio
    }

    fn start_recording(
        &mut self,
        settings: &ProcessingSettings,
        session: Uuid,
    ) -> Result<(), String> {
        let (Some(folder), Some(tool)) = (&self.folder, settings.tool()) else {
            return Err("No recorder is ready.".to_owned());
        };
        let label = self
            .notes
            .iter()
            .find(|note| note.id == session)
            .map_or_else(|| "this session".to_owned(), Note::label);
        let path = new_recording_path(folder, session);
        match Recorder::start(tool, settings.input(), path) {
            Ok(recorder) => {
                self.processed_session = None;
                self.processed_recording = None;
                self.job_done = false;
                self.job_warned = false;
                self.end_notice();
                // A new recording answers an old failure of this session.
                self.alerts
                    .retain(|alert| !(alert.can_record && alert.session == session));
                // And it ends the session's last `Recording saved: …` (`RecordingSaved`).
                self.saved.owner_changed(&self.of_session(Some(session)));
                self.live = Some(Live {
                    session,
                    label,
                    recorder,
                });
                self.refresh_recordings();
            }
            Err(error) => return Err(error),
        }
        Ok(())
    }

    fn start_processing(
        &mut self,
        ctx: &StateContext,
        recording: &Path,
        settings: &ProcessingSettings,
        book: &Book,
    ) {
        if let Some(session) = self.selected {
            self.process_recording(ctx, recording, settings, book, session);
        }
    }

    fn process_recording(
        &mut self,
        ctx: &StateContext,
        recording: &Path,
        settings: &ProcessingSettings,
        book: &Book,
        session: Uuid,
    ) {
        if !recording.exists() {
            self.say(
                Tone::Warning,
                "Audio removed. Restore the WAV file to process it.".into(),
                None,
            );
            return;
        }
        let note = self
            .notes
            .iter()
            .find(|note| note.id == session)
            .map_or_else(String::new, Note::label);
        let days = timeline::day_numbers(&book.events);
        let events: Vec<String> = book
            .events
            .iter()
            .zip(&days)
            .filter(|(event, _)| event.session == Some(session))
            .map(|(event, day)| format!("- {}", timeline::Stored::of(event).name(*day)))
            .collect();
        let context = format!(
            "{note}\n\n{}\n\nEvents the DM already recorded:\n{}",
            if self.selected == Some(session) {
                self.draft
                    .as_ref()
                    .map_or_else(String::new, |draft| draft.body.as_str().to_owned())
            } else {
                self.notes
                    .iter()
                    .find(|note| note.id == session)
                    .and_then(|note| fs::read_to_string(&note.path).ok())
                    .map(|text| body_of(&text).to_owned())
                    .unwrap_or_default()
            },
            events.join("\n")
        );
        let (speech_key, llm_key) = settings.held_keys();
        self.end_notice();
        let _ = fs::remove_file(recording.with_extension("state.txt"));
        self.processed_session = None;
        self.processed_recording = None;
        self.job_done = false;
        self.job_warned = false;
        self.alerts
            .retain(|alert| alert.can_record || alert.session != session);
        let job = processing::start(
            ctx,
            Request {
                recording: recording.to_owned(),
                config: settings.config.clone(),
                context,
                speech_hint: format!(
                    "A Dungeons & Dragons session at the table. {note}, in the campaign {}.",
                    self.campaign.as_ref().map_or("", |c| c.name.as_str())
                ),
                speech_key,
                llm_key,
                codex_program: "codex".into(),
            },
            |request| {
                // A key goes only to the service it was saved for.
                if request.speech_key.is_none() && request.config.speech_key_fits() {
                    request.speech_key = credentials::load_key(KeySlot::Speech)?;
                }
                if request.llm_key.is_none()
                    && request.config.provider != processing::Provider::CodexCli
                    && request.config.llm_key_fits()
                {
                    request.llm_key = credentials::load_key(KeySlot::Llm)?;
                }
                Ok(())
            },
        );
        self.job = Some((job, session));
    }

    /// Why `Record` cannot start now, in words.
    fn record_blocked(&self, settings: &ProcessingSettings) -> Option<String> {
        if self.folder.as_ref().is_none_or(|folder| !folder.is_dir()) {
            Some("Session folder is gone. Choose folder before recording.".into())
        } else if let Some(live) = &self.live {
            Some(format!("Stop the recording of {} first.", live.label))
        } else if settings.probing() {
            Some("Looking for a recorder.".to_owned())
        } else if settings.tool().is_none() {
            Some("No recorder found. See Settings.".to_owned())
        } else if settings.input_missing() {
            Some("The input chosen in Settings is not connected.".to_owned())
        } else {
            None
        }
    }

    /// Whether another session's recording state shows in the bottom banner: its recording
    /// runs, and the banner does not already say that it stopped saving audio (round 27 FE8).
    fn other_recording_shown(&self) -> bool {
        self.live
            .as_ref()
            .is_some_and(|live| Some(live.session) != self.selected)
            && self.recording_status().is_some()
            && self.stalled_elsewhere().is_none()
    }

    /// A note's state says it is the note, beside a control about audio.
    fn note_save_status(draft: &Draft) -> Status<'static> {
        Self::note_state_status(&draft.state)
    }

    fn note_state_status(state: &DraftState) -> Status<'static> {
        let c = colors();
        let (icon, word, color) = match state {
            // A saved note is the calm, usual state: muted, never green, in every view.
            DraftState::Clean | DraftState::Saved => (icons::OK, "Notes saved", c.document.clean),
            DraftState::Unsaved => (icons::BUSY, "Saving…", c.muted),
            DraftState::Conflict => (icons::WARN, "Changed on disk", c.document.external),
            // A moved or gone note keeps the draft safe and offers a choice, so it is a warning
            // here, in the rail and in its banner alike. A failed write stays an error (FM5).
            DraftState::Missing => (icons::WARN, "Notes not saved", c.document.external),
            DraftState::Failed(_) => (icons::ERROR, "Notes not saved", c.document.invalid),
        };
        Status { icon, word, color }
    }

    pub fn attention_status(&self) -> Option<Status<'static>> {
        self.draft
            .as_ref()
            .filter(|draft| draft.needs_attention())
            .map(Self::note_save_status)
    }

    /// A template draft's state, for its title line (`TitleLine::state`).
    fn draft_status(state: &DraftState) -> Status<'static> {
        let c = colors();
        let (icon, word, color) = match state {
            DraftState::Clean => (icons::OK, "Saved", c.document.clean),
            DraftState::Saved => (icons::OK, "Saved", c.document.saved),
            DraftState::Unsaved => (icons::DRAFT, "Unsaved", c.document.unsaved),
            DraftState::Conflict => (icons::WARN, "Changed on disk", c.document.external),
            DraftState::Missing => (icons::WARN, "File gone", c.document.external),
            DraftState::Failed(_) => (icons::ERROR, "Not saved", c.document.invalid),
        };
        Status { icon, word, color }
    }

    /// The message footer of a pane whose body is one draft's editor: the draft's own result
    /// with its Undo, and for a template, the template action's error. Returns where the body
    /// ends.
    fn draft_messages<'f>(&self, frame: &'f PaneFrame, template: bool) -> Messages<'f, SessionAct> {
        let mut messages = Messages::new();
        let draft = if template {
            self.template_draft.as_ref()
        } else {
            self.draft.as_ref()
        };
        if let Some(draft) = draft {
            let name = if template {
                self.template.clone()
            } else {
                self.note_name()
            };
            add_dropped(
                frame,
                &mut messages,
                draft,
                &name.unwrap_or_default(),
                &self.dropped,
            );
        }
        if template {
            if let Some(shown) = self.deleted_templates.get(&self.page()) {
                messages.kept(shown.outcome(frame, |deleted| {
                    Said::new(crate::sentence!("Deleted “{}”.", deleted.name))
                        .undo(
                            UndoStep::of(Verb::Delete, deleted.name.clone())
                                .pressed_as(SessionAct::UndoDeleteTemplate),
                        )
                        .into()
                }));
            }
            if let Some(error) = &self.template_error {
                messages.outcome(FaultMessage::new(&PageFault::Failed(notice_sentence(
                    error,
                ))));
            }
        }
        messages
    }

    /// A press in a draft's footer: the Undo of its dropped words, or of a template's delete.
    fn draft_press(&mut self, press: Press<SessionAct>, template: bool) {
        match press.id {
            SessionAct::UndoDrop => {
                let draft = if template {
                    self.template_draft.as_mut()
                } else {
                    self.draft.as_mut()
                };
                if let (Some(draft), Some(ticket)) = (draft, press.ticket) {
                    let _ = self.dropped.undo_by(ticket, |_| {
                        draft.undo_dropped();
                        Ok::<(), ()>(())
                    });
                }
            }
            SessionAct::UndoDeleteTemplate => {
                if let Some(ticket) = press.ticket {
                    self.undo_delete_template(ticket);
                }
            }
            _ => {}
        }
    }

    fn open_form(&mut self) {
        self.list_only = false;
        self.focus_form_title = true;
        // A template edited a moment ago is the one a new session starts from.
        self.flush();
        self.refresh();
        let number = self.notes.iter().map(|note| note.number).max().unwrap_or(0) + 1;
        let template = self
            .templates
            .iter()
            // The standard one, or a customized copy of it. A name that only sorts first,
            // such as a downtime template, is not what a new session wants.
            .find(|template| template.name == super::BUILT_IN_TEMPLATES[0].0)
            .or(self.templates.first())
            .map_or_else(String::new, |template| template.name.clone());
        self.form = Some(Form {
            editing: None,
            number: number.to_string(),
            title: String::new(),
            date: crate::hermetic::today(),
            template,
            error: None,
        });
    }

    /// The form's Create (or Save details): made when nothing blocks it, else the form stays
    /// with its reason or its fault. Returns the form that stays open. Both UIs call it.
    fn press_create(&mut self, mut form: Form) -> Option<Form> {
        // A press reads the templates as the folder holds them now: one changed or removed
        // outside Splat since the form opened is never made from the listing's copy. The press
        // is judged on that and on the form after this frame's typing; a refused press leaves
        // the reason on the kept line.
        if form.editing.is_none()
            && let Some(folder) = &self.folder
        {
            self.templates = list_templates(folder);
        }
        if form_reason(&form, &self.templates).is_some() {
            return Some(form);
        }
        match self.submit_form(&form) {
            Ok(id) => {
                self.refresh();
                self.select(Some(id));
                if form.editing.is_none() {
                    self.say_where_the_run_stays(id);
                }
                self.write_in_note();
                // A new session is for typing in, under What happened.
                self.focus_notes = form.editing.is_none();
                None
            }
            Err(error) => {
                form.error = Some(error);
                // The act failed and the form stays: the keyboard goes back to it.
                self.focus_form_title = true;
                Some(form)
            }
        }
    }

    /// The warning about an old session has gone, and the open run's event has not moved.
    fn say_where_the_run_stays(&mut self, new: Uuid) {
        self.moves.owner_changed(&self.page());
        let Some((name, session)) = self.open_run.clone().filter(|(_, session)| *session != new)
        else {
            return;
        };
        let Some(note) = self.notes.iter().find(|note| note.id == session) else {
            return;
        };
        let words = format!("{name} stays in {}.", note.label());
        self.move_run_here = Some((new, words.clone()));
        self.say(Tone::Info, words, Some(new));
    }

    /// `Move it here`: the open run's event moves to `session`, and its result with `Undo move`
    /// is kept in `moves`, the act's own entry.
    fn move_run_to_session(&mut self, book: &mut Book, run: Uuid, session: Uuid) {
        if let Some(moved) = self.move_run(book, run, session) {
            self.moves.record(moved);
        }
    }

    /// The move itself, and what its Undo needs.
    fn move_run(&mut self, book: &mut Book, run: Uuid, session: Uuid) -> Option<kept::Moved> {
        let event = book
            .events
            .iter_mut()
            .find(|event| event.run == Some(run))?;
        let before = event.clone();
        event.session = Some(session);
        event.continued_in = None;
        book.touch();
        let open_run = self.open_run.clone();
        let name = open_run.as_ref().map_or("Fight", |(name, _)| name.as_str());
        let number = self
            .notes
            .iter()
            .find(|note| note.id == session)
            .map_or(1, |note| note.number);
        let name = name.to_owned();
        // The `stays in` notice is answered: the result takes its place.
        self.end_notice();
        let moved = kept::Moved {
            page: self.page(),
            session,
            before,
            open_run,
            body: design::memo::Revised::new(
                self.draft
                    .as_ref()
                    .map_or_else(String::new, |draft| draft.body.as_str().to_owned()),
            ),
            events: book.events.clone(),
            name,
            number,
        };
        self.open_run = self.open_run.take().map(|(name, _)| (name, session));
        self.move_run_here = None;
        Some(moved)
    }

    /// A move's result ends when the Timeline or the note changed since (`Moved`'s end event).
    fn expire_move(&mut self, book: &Book) {
        let page = self.page();
        let changed = self.moves.get(&page).is_some_and(|shown| {
            let moved = shown.value();
            moved.events != book.events
                || self.draft.as_ref().is_none_or(|draft| {
                    !draft
                        .body
                        .same_as(moved.body.revision(), &moved.body, &self.same_moved)
                })
        });
        if changed {
            self.moves.owner_changed(&page);
        }
    }

    /// `Undo move`: the run's event goes back, and the run's session with it.
    fn undo_moved(&mut self, book: &mut Book, moved: kept::Moved) {
        if let Some(event) = book
            .events
            .iter_mut()
            .find(|event| event.id == moved.before.id)
        {
            *event = moved.before;
            book.touch();
        }
        self.open_run = moved.open_run;
    }

    /// The Session page of this campaign's folder: the owner of the page's kept results.
    fn page(&self) -> kept::Page {
        kept::Page {
            root: self.folder.clone().unwrap_or_default(),
        }
    }

    /// The page's one notice, while it stands (#597).
    fn notice(&self) -> Option<&kept::PageNotice> {
        self.notices.get(&self.page()).map(|shown| shown.value())
    }

    /// The session the page's notice is about: `None` for whichever is open, or no notice.
    fn notice_session(&self) -> Option<Uuid> {
        self.notice().and_then(|notice| notice.session)
    }

    /// Say `words` in `tone`, about `session` (`None`: whichever is open). It replaces the
    /// page's notice and takes a new order, as the newest result.
    fn say(&mut self, tone: Tone, words: String, session: Option<Uuid>) {
        let page = self.page();
        self.notices.record(kept::PageNotice {
            page,
            tone,
            words,
            session,
            arrived: false,
        });
    }

    /// What a job said when it finished while the DM was elsewhere: it stands in the bottom
    /// banner until the DM reads it where it happened.
    fn say_arrived(&mut self, tone: Tone, words: String, session: Option<Uuid>) {
        let page = self.page();
        self.notices.record(kept::PageNotice {
            page,
            tone,
            words,
            session,
            arrived: true,
        });
    }

    /// The page's notice ends.
    fn end_notice(&mut self) {
        let page = self.page();
        self.notices.owner_changed(&page);
    }

    /// One session of this campaign's folder.
    fn of_session(&self, session: Option<Uuid>) -> kept::OfSession {
        kept::OfSession {
            root: self.folder.clone().unwrap_or_default(),
            session,
        }
    }

    fn submit_form(&mut self, form: &Form) -> Result<Uuid, String> {
        let number: u32 = form
            .number
            .trim()
            .parse()
            .map_err(|_| FormReason::Number.words().to_owned())?;
        if let Some(reason) = form_reason(form, &self.templates) {
            return Err(reason.words().to_owned());
        }
        if let Some(id) = form.editing {
            let note = self
                .notes
                .iter()
                .find(|note| note.id == id)
                .cloned()
                .ok_or("This session is gone from the folder.")?;
            self.flush();
            if self.draft.as_ref().is_some_and(Draft::needs_attention) {
                return Err("Choose what to do with the unsaved notes first.".to_owned());
            }
            update_details(&note, &form.title, &form.date, number)?;
            // The file changed, so the draft reads it again.
            self.draft = None;
            return Ok(id);
        }
        let folder = self.folder.as_ref().ok_or("No session folder.")?;
        let campaign = self.campaign.as_ref().ok_or("No campaign.")?;
        let template = self
            .templates
            .iter()
            .find(|template| template.name == form.template)
            .map_or("", |template| template.body.as_str());
        let note = create_note(
            folder,
            &NewNote {
                number,
                title: &form.title,
                date: form.date.trim(),
                campaign: &campaign.name,
                template,
            },
        )?;
        // Whatever was chosen before, a new session is the one being played now.
        self.events_to = None;
        Ok(note.id)
    }

    pub fn fresh_focus_in_panel(&mut self) {
        self.focus_notes = self.notes_open;
    }

    /// A button took the keyboard. It goes back to the panel, or to the page's editor if
    /// the DM was typing there.
    /// Words the page adds to the open note (a reading's summary, a plan's link). It is the
    /// note's own edit, so the note's Undo history stays. False with no note open.
    pub(super) fn add_to_note(&mut self, words: &str) -> bool {
        let Some(draft) = &mut self.draft else {
            return false;
        };
        let before = draft.body.key();
        draft.body.edit(|body| insert_in_note(body, words));
        draft.own_write(before);
        draft.edited();
        true
    }

    /// Words the DM typed in the run's Amount, going on with the note.
    pub fn words_for_note(&mut self, words: String) {
        self.words_for_note = words;
        self.words_new_line = true;
    }

    /// After a step of the run, what is typed next is a new thought. Nothing changes in the
    /// note until the DM types it, so a click alone leaves the note as it was.
    pub fn after_a_step_back_to_note(&mut self, notes_gave_way: bool) {
        self.return_focus_to_note();
        self.break_before_words = self.refocus_note || notes_gave_way;
    }

    pub fn return_focus_to_note(&mut self) {
        let in_page_editor = self.on_page
            && !self.reading_note
            && self.tab != Tab::Recordings
            && self.note_had_focus;
        self.refocus_note = self.notes_open || in_page_editor;
    }

    /// Open or close the note beside the current page. It opens on the session that new
    /// events go to, because that is the one being played. The caret starts on a fresh line.
    pub fn toggle_notes(&mut self, on_this_page: bool) {
        // A form open here holds typed words, and it comes first.
        if on_this_page && self.form.is_some() {
            return;
        }
        let open = on_this_page || !self.notes_open;
        if !on_this_page {
            self.notes_open = open;
        }
        if open {
            let current = self.current();
            if current.is_some() && current != self.selected {
                self.select(current);
            }
            // Words that could not be saved came first, and they are on the page.
            if current.is_some() && self.selected != current {
                self.notes_open = false;
                self.wants_sessions_page = true;
                return;
            }
            self.focus_notes = true;
            if on_this_page {
                self.view = View::Notes;
                self.write_in_note();
            }
        }
    }

    /// Back after Lookup, with the caret where it was, on the session being played.
    pub fn notes_come_back(&mut self) {
        if self.selected.is_some() && self.current() == self.selected {
            self.notes_open = true;
            self.refocus_note = true;
        } else if !self.on_page {
            self.toggle_notes(false);
        }
    }

    /// The Notes panel's footer: the draft's dropped words with their Undo, and, for a session
    /// dated before today, `Play it tonight` and `New session` (both UIs).
    fn panel_messages<'f>(&mut self, frame: &'f PaneFrame) -> Messages<'f, SessionAct> {
        let today = self.today();
        let dated = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
            .filter(|note| note.date.as_str() < today.as_str())
            .map(|note| note.date.clone());
        let mut messages = Messages::new();
        if let Some(draft) = &self.draft {
            let name = self.note_name().unwrap_or_default();
            add_dropped(frame, &mut messages, draft, &name, &self.dropped);
        }
        if let Some(date) = dated {
            messages.outcome(
                NotTonight::notice(crate::sentence!(
                    "This session is dated {}. Play it tonight, or start a new one.",
                    date
                ))
                .choice(Choice::new(
                    MessageChoice::Choice("Play it tonight"),
                    SessionAct::PlayTonight,
                ))
                .choice(Choice::new(
                    MessageChoice::Choice("New session"),
                    SessionAct::NewSession,
                )),
            );
        }
        messages
    }

    /// A press in the Notes panel's footer (both UIs).
    fn panel_press(&mut self, act: SessionAct) {
        match act {
            SessionAct::UndoDrop => {
                if let Some(draft) = &mut self.draft {
                    draft.undo_dropped();
                }
            }
            SessionAct::PlayTonight => self.play_this_one(),
            SessionAct::NewSession => self.start_new_session(),
            _ => {}
        }
    }

    fn note_body(&self) -> String {
        let Some(draft) = &self.draft else {
            return String::new();
        };
        let note = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id));
        note.map_or_else(
            || draft.body.as_str().to_owned(),
            |note| without_own_heading(&draft.body, note),
        )
    }

    /// A timeline reference opens its event. Any other note link opens in Lookup.
    fn open_note_link(&mut self, target: String) {
        if let Some(id) = target.strip_prefix("Timeline#^") {
            self.wants_event = Some(id.to_owned());
        } else {
            self.wants_lookup = Some(target);
        }
    }

    /// Show the note, ready for words.
    fn write_in_note(&mut self) {
        self.tab = Tab::Note;
        self.reading_note = false;
        self.caret_to_show = None;
        self.note_scroll_to_top = true;
    }

    /// A run closed, so what was read of any fight may be out of date.
    pub fn forget_fight(&mut self) {
        self.fight = None;
    }

    /// A note's link that no entry answers to. Saying so is better than a link that does nothing.
    pub fn no_entry_named(&mut self, name: &str) {
        self.say(Tone::Info, format!("No entry is named {name}."), None);
    }

    /// Words at the end of the plan in this session's note, so that the file holds the plan.
    pub fn add_to_plan(&mut self, name: &str) {
        let (Some(draft), Some(session)) = (&mut self.draft, self.selected) else {
            return;
        };
        let line = format!("- [[{name}]]");
        let _ = draft.apply(session, Apply::WithTyping, |words| {
            let mut words = words.to_owned();
            insert_in_plan(&mut words, &line);
            Some(words)
        });
        self.mentions
            .insert(session, timeline::referenced_ids(&draft.body));
    }

    /// A saved group has a new name. Every note's link to it follows, so that no plan loses it.
    pub fn rename_links(&mut self, old: &str, new: &str) {
        if let Some(campaign) = &self.campaign {
            let other_entry =
                splat::content::read_directory(&campaign.path).is_ok_and(|(entries, _)| {
                    entries
                        .iter()
                        .any(|entry| entry.name.eq_ignore_ascii_case(old))
                });
            let session_folder = self.folder.as_deref().unwrap_or(&campaign.path);
            let other_note = crate::world::scan(&campaign.path, session_folder)
                .notes
                .iter()
                .any(|note| note.name.eq_ignore_ascii_case(old));
            if other_entry || other_note {
                self.rename_problem = Some(format!(
                    "More than one note can answer to {old}. Links were kept. Choose the group in each note before renaming its link."
                ));
                return;
            }
        }
        if let (Some(draft), Some(session)) = (&mut self.draft, self.selected) {
            let before = draft.body.key();
            let renamed = draft.body.edit(|body| rename_links(body, old, new));
            draft.own_write(before);
            if renamed {
                draft.edited();
                self.mentions
                    .insert(session, timeline::referenced_ids(&draft.body));
            }
        }
        if let Some((_, names)) = &mut self.tonight_links {
            for name in names
                .iter_mut()
                .filter(|name| name.eq_ignore_ascii_case(old))
            {
                new.clone_into(name);
            }
        }
        let kept: Vec<String> = self
            .notes
            .iter()
            .filter(|note| Some(note.id) != self.selected)
            .filter(|note| rename_links_in_file(&note.path, old, new).is_err())
            .map(Note::label)
            .collect();
        self.rename_problem = (!kept.is_empty()).then(|| {
            format!(
                "{} could not be changed, and still links {old}. Change the link to {new}.",
                kept.join(", ")
            )
        });
    }

    /// The names tonight's note links, for the run. The note on screen gives its words not yet
    /// saved. Another session's note on screen does not change what tonight's plan holds.
    pub fn tonight_links(&mut self) -> Vec<String> {
        let tonight = self.current();
        if self.selected == tonight
            && let (Some(id), Some(draft)) = (tonight, &self.draft)
        {
            self.tonight_links = Some((id, linked_names(&draft.body)));
        }
        self.tonight_links
            .as_ref()
            .filter(|(id, _)| Some(*id) == tonight)
            .map_or_else(Vec::new, |(_, names)| names.clone())
    }

    /// Everything the DM wrote, for Lookup. The open note gives the words not yet saved.
    pub fn written(&self, book: &Book) -> Vec<Written> {
        let mut written: Vec<Written> = self
            .notes
            .iter()
            .filter_map(|note| {
                let body = if self.selected == Some(note.id) {
                    self.draft
                        .as_ref()
                        .map(|draft| draft.body.as_str().to_owned())
                } else {
                    fs::read_to_string(&note.path)
                        .ok()
                        .map(|text| body_of(&text).to_owned())
                }?;
                let body = without_own_heading(&body, note);
                Some(Written {
                    title: note.label(),
                    detail: format!("Session note · {}", note.date),
                    // Search shows found lines as they read: a travel check's mark is no word.
                    text: words_to_read(&crate::timeline::checks::without_marks(&body), book),
                    place: WrittenIn::Session(note.id),
                })
            })
            .collect();
        let days = timeline::day_numbers(&book.events);
        written.extend(book.events.iter().zip(&days).map(|(event, day)| {
            let summary = event.facts();
            Written {
                title: event.label(*day),
                detail: if summary.is_empty() {
                    "Timeline".to_owned()
                } else {
                    format!("Timeline · {summary}")
                },
                text: [
                    event.place.as_str(),
                    event.result.as_str(),
                    event.notes.as_str(),
                ]
                .into_iter()
                .filter(|part| !part.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n\n"),
                place: WrittenIn::Event(event.id.clone()),
            }
        }));
        written
    }

    /// Read what the session before the selected one left, once for each selection. Returns
    /// the selected session. Both UIs call it before they draw the Last time line.
    fn see_last_time(&mut self, book: &Book, plan: &Plan<'_>) -> Option<Uuid> {
        let selected = self.selected?;
        if self.last_time.as_ref().map(|(id, _)| *id) != Some(selected) {
            let previous = self
                .notes
                .iter()
                .position(|note| note.id == selected)
                .and_then(|at| self.notes.get(at + 1));
            let found = previous.map(|note| {
                let text = fs::read_to_string(&note.path).unwrap_or_default();
                let body = body_of(&text);
                // Two notes with one title: the link names its file, as the list's second line
                // does, so it cannot read as a link to the open session.
                let label = note.label();
                let shared = self
                    .notes
                    .iter()
                    .any(|other| other.id != note.id && other.label() == label);
                let label = match note.path.file_name() {
                    Some(file) if shared => format!("{label} ({})", file.to_string_lossy()),
                    _ => label,
                };
                let source = self.carry_source(note, plan, book);
                let what_happened =
                    recap_preview(&without_own_heading(body, note), || match &source {
                        Some(Ok(source)) if source.id == note.id => {
                            source.processed().map(str::to_owned)
                        }
                        _ => carry::processed_recap(self.folder.as_deref()?, note.id),
                    });
                LastTime {
                    id: note.id,
                    label,
                    what_happened,
                    source,
                    offer: None,
                }
            });
            self.last_time = Some((selected, found));
        }
        Some(selected)
    }

    /// A message of success has done its work once the DM has moved on.
    fn clear_success(&mut self) {
        self.saved.owner_changed(&self.of_session(self.selected));
        // News of another session's job waits until the DM has seen it.
        if self
            .notice()
            .is_some_and(|notice| notice.tone == Tone::Success)
            && (self.notice_session().is_none() || self.notice_session() == self.selected)
        {
            self.end_notice();
        }
    }

    /// Read a text once, and open it at a time, or at its start.
    fn open_reading(&mut self, recording: &Recording, made: Made, at: Option<&str>) {
        let text = fs::read_to_string(made.path(&recording.path)).unwrap_or_default();
        let body = timeline::markers_for_reading(body_of(&text));
        let parts = parts_of(&body);
        self.clear_success();
        // The last part that starts at or before the time. A time is not always a heading.
        let part = at
            .and_then(|at| parts.iter().rposition(|(time, _)| time.as_str() <= at))
            .unwrap_or(0);
        self.reading = Some(Reading {
            recording: recording.path.clone(),
            name: recording.title.clone(),
            made,
            parts,
            part,
            short_of: matches!(made, Made::Transcript)
                .then(|| processing::transcript_short_of(&recording.path))
                .flatten(),
            shown: None,
        });
    }

    /// Round 26 FM4, FD3: the session page's results and notices that come and go sit in its
    /// message footer, so the title's line, Last time, the tabs and the recordings never move
    /// when one comes. Made before the body draws (`Messages` hold no borrow of the page), so
    /// the body may change anything; `session_press` acts on the footer's press after it.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: the notice slot of the page keeps its own order (`outcome_at`, design::results::STAGED)"
    )]
    fn session_messages<'f>(
        &mut self,
        frame: &'f PaneFrame,
        settings: &ProcessingSettings,
        book: &Book,
        plan: &Plan<'_>,
        note: &Note,
    ) -> Messages<'f, SessionAct> {
        self.expire_move(book);
        self.end_carry_results();
        let shown = self.notice_session().is_none() || self.notice_session() == self.selected;
        if shown && self.job_done() {
            self.job_done = false;
        }
        let mut messages = Messages::new();
        let dismiss = || Choice::new(MessageChoice::Dismiss(Lost::Nothing), SessionAct::Dismiss);
        if let Some(draft) = &self.draft {
            let name = self.note_name().unwrap_or_default();
            add_dropped(frame, &mut messages, draft, &name, &self.dropped);
        }
        if let Some((tone, words)) = self
            .notice()
            .filter(|_| shown && !self.notice_arrived())
            .map(|notice| (notice.tone, notice.words.clone()))
        {
            let here = self
                .move_run_here
                .as_ref()
                .filter(|(session, notice)| {
                    Some(*session) == self.selected && *notice == words && plan.open_run.is_some()
                })
                .map(|(session, _)| *session);
            let sentence = notice_sentence(&words);
            if let Some(session) = here {
                messages.outcome(Notice::new(sentence).choice(Choice::new(
                    MessageChoice::Choice("Move it here"),
                    SessionAct::MoveRunHere(session),
                )));
            } else {
                match tone {
                    Tone::Error => {
                        messages.outcome(
                            FaultMessage::new(&PageFault::Failed(sentence)).choice(dismiss()),
                        );
                    }
                    Tone::Warning => {
                        messages.outcome(
                            FaultMessage::new(&PageFault::Check(sentence)).choice(dismiss()),
                        );
                    }
                    Tone::Success => {
                        // A success is a kept line: its order and its ticket are the store's.
                        if let Some(shown) = self.notices.get(&self.page()) {
                            messages.kept(
                                shown.outcome(frame, |_| {
                                    Said::new(sentence).choice(dismiss()).into()
                                }),
                            );
                        }
                    }
                    Tone::Info => {
                        messages.outcome(Notice::new(sentence).choice(dismiss()));
                    }
                }
            }
        }
        self.kept_lines(frame, book, &mut messages);
        let today = self.today();
        let writing = self.tab != Tab::Recordings && !self.reading_note;
        if writing && self.selected == self.current() && note.date.as_str() < today.as_str() {
            messages.outcome(
                NotTonight::notice(crate::sentence!(
                    "This session is dated {}. Play it tonight, or start a new one.",
                    note.date
                ))
                .choice(Choice::new(
                    MessageChoice::Choice("Play this one"),
                    SessionAct::PlayThisOne,
                )),
            );
        }
        if let Some(words) = self.not_being_played_words(note, plan) {
            let current = if plan.group_open {
                self.group_session
            } else {
                self.current()
            };
            let next = self
                .notes
                .iter()
                .any(|note| Some(note.id) == current)
                .then(|| {
                    if current == self.current() {
                        "Return to the session being played"
                    } else {
                        "Go to it"
                    }
                });
            let mut notice = NotTonight::notice(words);
            if let Some(next) = next {
                notice = notice.choice(Choice::new(
                    MessageChoice::Choice(next),
                    SessionAct::Open(current),
                ));
            }
            messages.outcome(notice);
        }
        // The input went away by itself: a notice, not the recording's state line.
        if self.live.is_none() && !self.confirm_record && settings.input_missing() {
            messages.outcome(
                Notice::new(crate::sentence!(
                    "The input chosen in Settings is not connected."
                ))
                .choice(Choice::new(
                    MessageChoice::Choice("Open Settings"),
                    SessionAct::OpenSettings,
                )),
            );
        }
        self.carry_in_messages(frame, &mut messages);
        messages
    }

    /// The page's kept results with their Undo, each from its own store, in the footer of the
    /// session it belongs to: a separate session, the open run moved here, a marker's event.
    fn kept_lines<'f>(
        &self,
        frame: &'f PaneFrame,
        book: &Book,
        messages: &mut Messages<'f, SessionAct>,
    ) {
        let page = self.page();
        // A copy that claims this session arrives from the folder, so it sits in the bottom
        // banner; its result is here.
        if let Some(shown) = self.separate.get(&page) {
            messages.kept(shown.outcome(frame, |separate| {
                let filename = separate.undo.file_name();
                Said::new(crate::sentence!("{} is now a separate session.", filename))
                    .undo(
                        UndoStep::of(Verb::Separate, "session")
                            .pressed_as(SessionAct::UndoIdentity),
                    )
                    .into()
            }));
        }
        if let Some(shown) = self
            .moves
            .get(&page)
            .filter(|shown| Some(shown.value().session) == self.selected)
        {
            messages.kept(shown.outcome(frame, |moved| {
                Said::new(crate::sentence!(
                    "{} moved to Session {}.",
                    moved.name,
                    moved.number
                ))
                .undo(UndoStep::of(Verb::Move, moved.name.clone()).pressed_as(SessionAct::UndoMove))
                .into()
            }));
        }
        let mine = self.of_session(self.selected);
        // Round 30 `recording-result-owner`: a saved recording's result, in its own session's
        // footer while its file stands (`RecordingSaved`).
        if let Some(shown) = self.saved.get(&mine) {
            messages.kept(shown.outcome(frame, |saved| {
                Said::new(crate::sentence!(
                    "Recording saved: {}.",
                    recording_title(&saved.path)
                ))
                .choice(Choice::new(
                    MessageChoice::Dismiss(Lost::Nothing),
                    SessionAct::DismissSaved,
                ))
                .into()
            }));
        }
        // Round 27 FI3, FN2: a marker's Add result, in its own session's footer while its event
        // stands. The marker row keeps `✓ On the timeline`.
        if let Some(shown) = self
            .markers
            .get(&mine)
            .filter(|shown| book.events.iter().any(|event| event.id == shown.value().id))
        {
            let session = self.session_name(self.selected);
            messages.kept(shown.outcome(frame, |added| {
                let words = match &session {
                    Some(session) => crate::sentence!("{} added to {}.", added.title, session),
                    None => crate::sentence!("{} added.", added.title),
                };
                Said::new(words)
                    .undo(
                        UndoStep::of(Verb::Add, added.title.clone())
                            .pressed_as(SessionAct::UndoMarker),
                    )
                    .into()
            }));
        }
    }

    /// A press on one of `kept_lines`: the store acts on the entry the ticket names. Gives back
    /// any other press.
    fn kept_press(
        &mut self,
        press: Press<SessionAct>,
        book: &mut Book,
    ) -> Option<Press<SessionAct>> {
        match press.id {
            SessionAct::UndoMove => {
                self.expire_move(book);
                if let Some(ticket) = press.ticket
                    && let Ok((moved, ())) = self.moves.undo_by(ticket, |_| Ok::<(), ()>(()))
                {
                    self.undo_moved(book, moved);
                }
            }
            SessionAct::UndoIdentity => {
                if let Some(ticket) = press.ticket {
                    match self
                        .separate
                        .undo_by(ticket, |separate| separate.undo.undo())
                    {
                        Ok(_) => self.refresh(),
                        // The result stays, with its Undo, beside the reason.
                        Err(ByTicket::Failed(error)) => {
                            self.say(Tone::Error, error, None);
                        }
                        Err(ByTicket::Gone) => {}
                    }
                }
            }
            SessionAct::UndoMarker => {
                if let Some(ticket) = press.ticket
                    && let Ok((added, ())) = self.markers.undo_by(ticket, |_| Ok::<(), ()>(()))
                {
                    book.undo_add(&added.id);
                }
            }
            SessionAct::DismissSaved => {
                if let Some(ticket) = press.ticket {
                    let _ = self.saved.use_press_by(ticket);
                }
            }
            _ => return Some(press),
        }
        None
    }

    /// What a press in the session page's footer does. A kept result's Undo acts on the entry
    /// its ticket names; an entry replaced or ended since is gone, and nothing happens.
    fn session_press(&mut self, press: Press<SessionAct>, book: &mut Book, plan: &Plan<'_>) {
        let Some(press) = self.kept_press(press, book) else {
            return;
        };
        match press.id {
            SessionAct::MoveRunHere(session) => {
                if let Some(run) = plan.open_run {
                    self.move_run_to_session(book, run, session);
                }
            }
            // The kept results' presses (`kept_press`).
            SessionAct::UndoMove
            | SessionAct::UndoIdentity
            | SessionAct::UndoMarker
            | SessionAct::DismissSaved => {}
            // A success's close icon uses its kept line up, by its ticket; another notice's ends
            // the page's notice.
            SessionAct::Dismiss => match press.ticket {
                Some(ticket) => {
                    let _ = self.notices.use_press_by(ticket);
                }
                None => self.end_notice(),
            },
            SessionAct::PlayThisOne => self.play_this_one(),
            SessionAct::Open(session) => self.select(session),
            SessionAct::OpenSettings => self.wants_processing_settings = true,
            SessionAct::UndoCarry(session) => self.undo_carry(session, press.ticket),
            SessionAct::UndoDrop => {
                if let (Some(draft), Some(ticket)) = (&mut self.draft, press.ticket) {
                    let _ = self.dropped.undo_by(ticket, |_| {
                        draft.undo_dropped();
                        Ok::<(), ()>(())
                    });
                }
            }
            SessionAct::PlayTonight => self.play_this_one(),
            SessionAct::NewSession => self.start_new_session(),
            // The template pane's own press (`draft_press`), and the plan's.
            SessionAct::UndoDeleteTemplate | SessionAct::Plan(_) => {}
        }
    }

    fn select_template(&mut self, name: &str) {
        self.flush();
        if self
            .template_draft
            .as_ref()
            .is_some_and(Draft::needs_attention)
        {
            return;
        }
        self.template = Some(name.to_owned());
        self.template_draft = self
            .templates
            .iter()
            .find(|template| template.name == name)
            .and_then(|template| template.path.as_deref())
            .and_then(|path| Draft::open(path).ok());
    }

    /// The template list's footer, with the list alone on screen: the delete result.
    fn list_messages<'f>(&self, frame: &'f PaneFrame) -> Messages<'f, SessionAct> {
        let mut messages = Messages::new();
        if let Some(shown) = self.deleted_templates.get(&self.page()) {
            messages.kept(shown.outcome(frame, |deleted| {
                Said::new(crate::sentence!("Deleted “{}”.", deleted.name))
                    .undo(
                        UndoStep::of(Verb::Delete, deleted.name.clone())
                            .pressed_as(SessionAct::UndoDeleteTemplate),
                    )
                    .into()
            }));
        }
        messages
    }

    /// `Undo delete`: the deleted template's file comes back with the words it had.
    fn undo_delete_template(&mut self, ticket: design::results::Ticket) {
        let Some(folder) = self.folder.clone() else {
            return;
        };
        // It leaves the store only when the file is written again; else it stays, with its
        // Undo, beside the reason.
        match self.deleted_templates.undo_by(ticket, |deleted| {
            create_template(&folder, &deleted.name, &deleted.body)
        }) {
            Ok((deleted, _)) => {
                self.refresh();
                self.select_template(&deleted.name);
            }
            Err(ByTicket::Failed(error)) => self.template_error = Some(error),
            Err(ByTicket::Gone) => {}
        }
    }

    /// The New template form's Create: a template that starts with the title line only.
    /// Returns whether it was made (both UIs).
    fn create_new_template(&mut self, name: &str) -> bool {
        let Some(folder) = &self.folder else {
            return false;
        };
        let body = "# Session {{number}}: {{title}}\n\n";
        match create_template(folder, name, body) {
            Ok(template) => {
                self.template_error = None;
                self.refresh();
                self.select_template(&template.name);
                true
            }
            Err(error) => {
                self.template_error = Some(error);
                false
            }
        }
    }

    /// `Customize` of a built-in template: a copy in the folder to edit (both UIs).
    fn customize_template(&mut self, index: usize) {
        let Some(folder) = &self.folder else {
            return;
        };
        let name = self.templates[index].name.clone();
        match create_template(folder, &name, &self.templates[index].body) {
            Ok(created) => {
                self.refresh();
                self.select_template(&created.name);
            }
            Err(error) => self.template_error = Some(error),
        }
    }

    /// `Delete` (or `Go back to built-in`) of a campaign template, with `Undo delete` (both UIs).
    fn delete_template(&mut self, index: usize) {
        let Some(path) = self.templates[index].path.clone() else {
            return;
        };
        let name = self.templates[index].name.clone();
        match super::remove_file(&path) {
            Ok(()) => {
                let body = self.template_draft.take().map_or_else(
                    || self.templates[index].body.as_str().to_owned(),
                    |draft| draft.body.into_string(),
                );
                self.deleted_templates.record(kept::DeletedTemplate {
                    page: kept::Page {
                        root: self.folder.clone().unwrap_or_default(),
                    },
                    name,
                    body,
                });
                self.template = None;
                self.refresh();
            }
            Err(error) => self.template_error = Some(super::not_deleted(&path, &error)),
        }
    }

    /// Why the folder cannot be saved now, by the folder's own state; `None` when it can.
    fn folder_wait(&self, book: &Book, changed: bool, files: &FolderFiles) -> Option<FolderWait> {
        if self.live.is_some() {
            Some(FolderWait::Recording)
        } else if self.job.is_some() {
            Some(FolderWait::Processing)
        } else if self.busy_with_edit() || book.not_saved() {
            Some(FolderWait::Unsaved)
        } else if files.is_err() {
            Some(FolderWait::CannotRead)
        } else if !changed {
            Some(FolderWait::NoChanges)
        } else {
            None
        }
    }

    /// The DM's act on the folder: Save folder (no files offered), or one of the warning's two
    /// choices. Each reads the disk at the act.
    fn save_folder_act(&mut self, campaign: &Campaign, book: &Book, act: FolderAct) {
        let move_files = act == FolderAct::Move;
        self.flush();
        // The save just now can fail. Those words stay on screen, as `select` keeps them.
        if self.draft.as_ref().is_some_and(Draft::needs_attention) {
            self.folder_error = Some(FolderFault::NoteNotSaved);
            return;
        }
        // The rule: an act commits only what the DM was shown. The row and the warning were
        // drawn from the kept listing, which can be older than the disk, so every act reads the
        // current folder now. Save folder (offered with no files) stops when files came; Save
        // without moving stops when the files differ from the counts its warning named; a folder
        // that cannot be read stops all three. Move files and save moves what is there at the
        // act, and its result names what it moved (`move_storage`), never the counts shown.
        let shown = {
            let drawn = self.folder_files();
            self.folder_move_counts(book, &drawn)
        };
        self.end_folder_listing();
        let now = self.folder_files();
        if let Err(error) = &now {
            self.folder_error = Some(FolderFault::CannotRead(error.clone()));
            return;
        }
        match act {
            FolderAct::Save
                if now
                    .as_ref()
                    .is_ok_and(|now| now.is_some_and(|now| now.files > 0)) =>
            {
                self.folder_error = Some(FolderFault::FilesCame);
                return;
            }
            // The refusals of Save without moving end at the draft's next edit or Discard,
            // and when their cause goes: the warning
            // gone for `FilesChanged`, files back for `FilesGone`.
            FolderAct::WithoutMoving if self.folder_move_counts(book, &now) != shown => {
                self.folder_error = Some(if self.folder_move_counts(book, &now).is_none() {
                    FolderFault::FilesGone
                } else {
                    FolderFault::FilesChanged
                });
                return;
            }
            _ => {}
        }
        let campaign = campaign.clone();
        let link = kept::FolderLink {
            campaign: campaign.path.clone(),
        };
        // A save or a failed save of the folder ends the last move's result.
        self.folder_moves.owner_changed(&link);
        let events = book.events.len();
        let saved = if move_files {
            // What it moved, listed at the act: never the listing drawn.
            super::move_storage(&campaign, &self.folder_text)
                .map(|(folder, moved)| (folder, storage_words(moved, events)))
        } else {
            save_folder(&campaign, &self.folder_text).map(|folder| (folder, None))
        };
        match saved {
            Ok((folder, moved)) => {
                if let Some(counts) = moved {
                    self.folder_moves.record(kept::FolderMoved {
                        link,
                        counts,
                        typed: self.folder_text.clone(),
                        to: folder.clone(),
                    });
                }
                self.folder = Some(folder);
                self.folder_error = None;
                self.manifest_changed = Some(campaign.path.clone());
                self.draft = None;
                self.template_draft = None;
                self.template = None;
                self.refresh();
                self.select(self.notes.first().map(|note| note.id));
            }
            Err(error) => {
                self.folder_error = Some(FolderFault::NotSaved(error));
            }
        }
    }

    /// The Session folder form's owner in the results store: the open campaign.
    fn folder_link(&self) -> Option<kept::FolderLink> {
        self.campaign.as_ref().map(|campaign| kept::FolderLink {
            campaign: campaign.path.clone(),
        })
    }

    /// The current folder's listing: kept for its folder (`folder_scan`) until `refresh` ends
    /// it, so a frame with the form in view does not list the folder again.
    fn folder_files(&mut self) -> FolderFiles {
        let Some(folder) = &self.folder else {
            return Ok(None);
        };
        if self
            .folder_scan
            .as_ref()
            .is_none_or(|scan| &scan.folder != folder)
        {
            let listed = super::storage_files(folder)
                .map(|files| Some(super::StorageCounts::of(folder, &files)));
            self.folder_scan = Some(FolderScan {
                folder: folder.clone(),
                listed,
            });
        }
        self.folder_scan
            .as_ref()
            .map_or(Ok(None), |scan| scan.listed.clone())
    }

    /// What a Move files and save would move from the current folder, in words, when the typed
    /// folder differs and the listing drawn finds files. The form's warning says it; the move's
    /// result says what `move_storage` moved (`storage_words`).
    fn folder_move_counts(&self, book: &Book, files: &FolderFiles) -> Option<String> {
        let campaign = self.campaign.as_ref()?;
        let changed = super::resolve(campaign, &self.folder_text).ok() != self.folder.clone();
        self.folder.as_ref().filter(|_| changed)?;
        storage_words(*files.as_ref().ok()?.as_ref()?, book.events.len())
    }

    /// Round 26 FN2, FN5, ruling R24: the Session folder form's messages, `S` below its save row,
    /// so a typed letter, a refusal or a result never moves the save row: why
    /// the folder cannot be used or a write failed (`FolderFault`), what Save without moving
    /// would hide (`Notice::warning`, with the two choices `Move files and save` and `Save
    /// without moving`, none while the page is locked), and the open campaign's move result from
    /// its store; a choice's press comes back from `ValueBlock::messages`. The folder's listing is read once per disk scan
    /// (`folder_scan`).
    fn folder_messages<'f>(
        &mut self,
        frame: &'f PaneFrame,
        messages: &mut impl design::outcome::plain::Sink<'f, FolderAct>,
        book: &Book,
        page_locked: bool,
    ) {
        let files = self.folder_files();
        if let Some(fault) = &self.folder_error {
            messages.outcome(FaultMessage::new(fault).into());
        }
        if let Some(counts) = self.folder_move_counts(book, &files) {
            // A warning before the act, in the warning tone, with its two choices: they are the
            // folder's commit when it keeps files. While the page is locked (a run), the save
            // row says why, and no choice is offered.
            let mut warning = Notice::warning(folder_keeps(&counts));
            if !page_locked {
                let campaign = self.campaign.as_ref();
                let changed = campaign.is_some_and(|campaign| {
                    super::resolve(campaign, &self.folder_text).ok() != self.folder.clone()
                });
                let wait = self.folder_wait(book, changed, &files);
                for (words, act) in [
                    (MOVE_FILES, FolderAct::Move),
                    ("Save without moving", FolderAct::WithoutMoving),
                ] {
                    let mut choice = Choice::new(MessageChoice::Choice(words), act);
                    if let Some(wait) = wait {
                        choice = choice.locked_when(true, wait.sentence());
                    }
                    warning = warning.choice(choice);
                }
            }
            messages.outcome(warning.into());
        }
        if let Some(shown) = self.folder_link().and_then(|link| {
            self.folder_moves
                .get(&link)
                .map(|shown| shown.outcome(frame, |moved| folder_moved(moved).into()))
        }) {
            messages.kept(shown);
        }
    }

    pub fn clear_notice(&mut self) {
        self.end_plan_add();
        // Leaving the Recordings tab ends the marker's result (`MarkerAdd`'s owner event).
        self.markers.owner_changed(&self.of_session(self.selected));
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Sessions {
    pub fn busy(&self) -> bool {
        self.job.as_ref().is_some_and(|(job, _)| !job.is_held())
    }

    /// A processing failure for the open session, as the job would raise it.
    pub fn simulate_alert(&mut self, words: &str) {
        let session = self.selected.expect("an open session");
        self.raise(Alert {
            session,
            can_record: false,
            retry: None,
            said: None,
            word: "Processing failed",
            words: words.to_owned(),
        });
    }

    /// Processing of the open session finishes now, as the background job would.
    pub fn simulate_job_finished(&mut self) {
        let session = self.selected.expect("an open session");
        self.finish_job(Ok(None), session);
    }

    /// A job that stays in progress, for a capture of that state.
    pub fn simulate_job(&mut self, progress: processing::Progress) {
        self.end_notice();
        self.processed_session = None;
        self.processed_recording = None;
        self.job_done = false;
        self.job_warned = false;
        let session = self.selected.expect("an open session");
        let path = self.recordings[0].path.clone();
        if let processing::Progress::Transcribing { part, of } = &progress {
            fs::write(
                processing::transcript_path(&path),
                format!(
                    "---\ntranscript_parts_done: {part}\ntranscript_parts: {of}\n---\n\nThe party held the bridge.\n"
                ),
            )
            .expect("a partial raw transcript before cancellation");
            self.refresh_recordings();
        }
        if matches!(progress, processing::Progress::Asking { .. }) {
            fs::write(
                processing::transcript_path(&path),
                "---\ntranscript_parts_done: 1\ntranscript_parts: 1\n---\n\nThe party held the bridge.\n",
            )
            .expect("a complete raw transcript before the LLM pass");
            self.refresh_recordings();
        }
        self.job = Some((Job::held(path, progress), session));
    }

    /// A job that ends with a warning, stored as a real one is.
    pub fn simulate_job_end_warned(&mut self, words: &str) {
        let (job, session) = self.job.as_ref().expect("a job");
        let session = *session;
        fs::write(processing::warning_path(&job.recording), words).expect("the warning");
        self.finish_job(Ok(Some(words.to_owned())), session);
    }

    /// The held job ends with this result, as the background job would.
    pub fn simulate_job_end(&mut self, result: Result<Option<String>, String>) {
        let session = self.job.as_ref().expect("a job").1;
        self.finish_job(result, session);
    }

    /// The open note's file changes outside Splat while the DM has unsaved words.
    pub fn simulate_conflict(&mut self) {
        self.simulate_disk_change();
        let draft = self.draft.as_mut().expect("an open note");
        draft.body.push_str("\nTyped before the outside edit.\n");
        draft.edited();
        draft.save();
    }

    /// The open note changes outside Splat, as when Obsidian edits the same file.
    pub fn simulate_disk_change(&mut self) {
        let draft = self.draft.as_ref().expect("an open note");
        let text = fs::read_to_string(&draft.path).expect("the note");
        fs::write(&draft.path, format!("{text}\nWritten in Obsidian.\n")).expect("the note");
    }

    pub fn simulate_recorder_health(&mut self, health: Health) {
        if let Some(live) = &mut self.live {
            live.recorder.simulate_health(health);
        }
    }
}

/// The removal's owner note as it was first seen open, and the draft revision last looked at.
struct RemovalNote {
    /// The removed event.
    id: String,
    revision: Option<u64>,
    /// The note's words when the result began, with their revision (layout PR 9b-C), and the
    /// kept answer of the draft's compare with them: a quiet frame compares two revisions.
    words: Option<design::memo::Revised<String>>,
    same: design::text_draft::KeptCompare,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timeline::{Kind, removed_event_words};

    #[test]
    fn iter10_c_known_recorder_errors_use_plain_words() {
        assert_eq!(
            recording_detail("Stopped by itself: no such device.", true),
            "Stopped by itself: the microphone is no longer connected. Choose an input, then record again."
        );
        assert_eq!(
            recording_detail("Stopped by itself: Device or resource busy", false),
            "Stopped by itself: another program is using the microphone. Check the microphone, then record again."
        );
        // An error Splat does not know keeps the recorder's own words.
        assert_eq!(
            recording_detail("Stopped by itself: stream error 42.", false),
            "Stopped by itself: stream error 42. Check the microphone, then record again."
        );
        assert_eq!(
            recorder_said("no such device."),
            "The recorder said: no such device."
        );
        // The plain words still lead to Choose an input.
        let alert = Alert {
            word: "Recording stopped",
            words: "The recording of Session 1 stopped by itself: the microphone is no longer connected."
                .to_owned(),
            session: Uuid::nil(),
            can_record: true,
            retry: None,
            said: Some(recorder_said("no such device")),
        };
        assert!(alert.missing_input());
        let unknown = Alert {
            said: None,
            words: "The recording stopped by itself. The recorder said: stream error 42."
                .to_owned(),
            ..alert
        };
        assert!(!unknown.missing_input());
    }

    #[test]
    fn iter10_c_removal_notice_names_the_removed_fight() {
        let session = Some(Uuid::new_v4());
        let fight = |id: &str, result: &str| timeline::Event {
            id: id.to_owned(),
            kind: Kind::Encounter,
            title: "Bridge Ambush".to_owned(),
            duration: String::new(),
            place: String::new(),
            notes: String::new(),
            result: result.to_owned(),
            session,
            run: Some(Uuid::new_v4()),
            marker: String::new(),
            restores: false,
            continued_in: None,
            played_order: 0,
            checks_table: None,
            checks: Vec::new(),
            within: None,
            extra: Default::default(),
        };
        let mut book = Book::default();
        book.events = vec![
            fight("a", "Round 2"),
            fight("b", "Held the bridge, round 1"),
        ];
        book.remove("a");
        let days = timeline::day_numbers(&book.events);
        assert_eq!(
            removed_event_words(&book, session, &days, "Bridge Ambush", &[]),
            "Removed “Bridge Ambush” (Round 2 · Paused). The other “Bridge Ambush” and its result stay."
        );
        // A same-name fight in another session is not in this plan.
        assert_eq!(
            removed_event_words(&book, Some(Uuid::new_v4()), &days, "Bridge Ambush", &[]),
            "Removed “Bridge Ambush” (Round 2 · Paused)."
        );
        // A removal from another session (made on the Timeline page) is not "the other" fight of
        // a same-name fight in the session shown.
        let other = Some(Uuid::new_v4());
        book.events[0].session = other;
        assert_eq!(
            removed_event_words(&book, other, &days, "Bridge Ambush", &[]),
            "Removed “Bridge Ambush” (Round 2 · Paused).",
            "iter10-lead-other-session"
        );
        book.events[0].session = session;
        // A remaining fight with no result yet stays, with no result to name.
        crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove);
        book.events[1].result.clear();
        book.remove("a");
        assert_eq!(
            removed_event_words(&book, session, &days, "Bridge Ambush", &[]),
            "Removed “Bridge Ambush” (Round 2 · Paused). The other “Bridge Ambush” stays."
        );
        // A planned fight of the same name counts too, with its ended run as its result.
        book.events.remove(0);
        assert_eq!(
            removed_event_words(
                &book,
                session,
                &days,
                "Bridge Ambush",
                &[
                    ("Bridge Ambush".to_owned(), true),
                    ("Shrine".to_owned(), true)
                ]
            ),
            "Removed “Bridge Ambush” (Round 2 · Paused). The other “Bridge Ambush” and its result stay."
        );
        // iter-23-S FK2: the removal's result and Undo belong to the removed event's session.
        // With B's note open, A's event goes from Timeline: an edit to B keeps A's Undo, and
        // Undo brings back A's event and its words together. An edit to A's note ends it.
        let dir = tempfile::tempdir().unwrap();
        let (a, b) = (Uuid::new_v4(), Uuid::new_v4());
        let note = |id: Uuid, name: &str| {
            let path = dir.path().join(format!("{name}.md"));
            fs::write(&path, format!("# {name}\n")).unwrap();
            Note {
                id,
                number: 1,
                title: name.to_owned(),
                date: "2026-01-01".into(),
                path,
                also: Vec::new(),
            }
        };
        let (note_a, note_b) = (note(a, "A"), note(b, "B"));
        let mut book = Book::default();
        book.events = vec![
            timeline::Event {
                session: Some(a),
                ..fight("a", "Round 2")
            },
            timeline::Event {
                session: Some(b),
                ..fight("b", "")
            },
        ];
        let mut sessions = Sessions {
            draft: Some(Draft::open(&note_b.path).unwrap()),
            notes: vec![note_a.clone(), note_b],
            ..Default::default()
        };
        book.remove("a");
        let days = timeline::day_numbers(&book.events);
        let words = removed_event_words(&book, Some(a), &days, "Bridge Ambush", &[]);
        assert_eq!(words, "Removed “Bridge Ambush” (Round 2 · Paused).");
        sessions.end_removal_on_note_edit(&mut book);
        // Unchanged polls do not look at the notes or the words again.
        let scans = sessions.removal_scans;
        for _ in 0..3 {
            sessions.end_removal_on_note_edit(&mut book);
        }
        assert_eq!(sessions.removal_scans, scans, "unchanged polls do no work");
        let draft = sessions.draft.as_mut().unwrap();
        draft.body.push_str("B’s words.\n");
        draft.edited();
        sessions.end_removal_on_note_edit(&mut book);
        assert_eq!(
            sessions.removal_scans,
            scans + 1,
            "an edit is looked at once"
        );
        sessions.end_removal_on_note_edit(&mut book);
        assert_eq!(sessions.removal_scans, scans + 1);
        assert_eq!(
            book.removed(),
            Some("Bridge Ambush"),
            "an edit to B keeps A’s Undo"
        );
        assert_eq!(
            crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove)
                .as_deref(),
            Some("a")
        );
        assert_eq!(book.events[0].id, "a");
        assert_eq!(book.events[0].session, Some(a));
        assert!(book.removed().is_none(), "the words go with the Undo");
        book.remove("a");
        sessions.end_removal_on_note_edit(&mut book);
        sessions.draft = Some(Draft::open(&note_a.path).unwrap());
        sessions.end_removal_on_note_edit(&mut book);
        assert_eq!(
            book.removed(),
            Some("Bridge Ambush"),
            "opening A is not an edit"
        );
        let draft = sessions.draft.as_mut().unwrap();
        draft.body.push_str("[[Bridge Ambush]]\n");
        draft.edited();
        sessions.end_removal_on_note_edit(&mut book);
        assert!(book.removed().is_none(), "an edit to A ends it");
        assert!(
            crate::timeline::press_footer_undo(&mut book, crate::timeline::TestUndo::Remove)
                .is_none()
        );
    }

    #[test]
    fn iter23_s_fl2_a_missing_note_names_its_new_place_only_when_known() {
        assert_eq!(
            missing_note_words(Some("Renamed in Obsidian.md".to_owned())),
            "This note was moved or renamed. It is now Renamed in Obsidian.md. Your draft is still here."
        );
        // Outside the rule: with no new place, the file is only gone.
        assert_eq!(
            missing_note_words(None),
            "This file is gone from the folder. Your draft is still here. Choose where to save it, or discard it."
        );
    }

    #[test]
    fn iter8_c_st2_warning_names_duplicate_number_files_and_keeps_ordinary_numbers() {
        let played = Note {
            id: Uuid::new_v4(),
            number: 1,
            title: "The Bridge".into(),
            date: "2026-01-01".into(),
            path: PathBuf::from("Session 1 - The Bridge.md"),
            also: Vec::new(),
        };
        let mut open = played.clone();
        open.id = Uuid::new_v4();
        open.path = PathBuf::from("Session 1 - The Bridge 2.md");
        let mut sessions = Sessions {
            notes: vec![played.clone(), open.clone()],
            selected: Some(open.id),
            events_to: Some(Some(played.id)),
            ..Default::default()
        };
        assert_eq!(
            sessions.not_being_played().as_ref().map(Sentence::as_str),
            Some(
                "What you write and add on this page goes into Session 1 - The Bridge 2.md. The session being played is Session 1 - The Bridge.md."
            ),
            "iter-8-C-ST2-files"
        );
        sessions.notes[1].number = 2;
        assert_eq!(
            sessions.not_being_played().as_ref().map(Sentence::as_str),
            Some(
                "What you write and add on this page goes into Session 1 - The Bridge 2.md. The session being played is Session 1."
            )
        );
        sessions.selected = Some(played.id);
        assert!(sessions.not_being_played().is_none());
        sessions.selected = None;
        assert!(sessions.not_being_played().is_none());
    }

    #[test]
    fn iter1_c_completed_processing_uses_processed_in_the_rail() {
        let mut page = Sessions {
            processed_session: Some(Uuid::new_v4()),
            job_done: true,
            ..Default::default()
        };
        let clean = page.rail_recording_status().unwrap();
        assert_eq!(clean.word, "Processed");
        // One status, one icon: the rail uses the list's and the row's clean status.
        assert_eq!(clean.icon, processed_clean().icon);
        assert_eq!(clean.icon, icons::OK);
        assert_eq!(clean.icon_color(), processed_clean().color);
        assert_eq!(clean.word_color(), processed_clean().color);
        page.job_warned = true;
        let status = page.rail_recording_status().unwrap();
        assert_eq!(status.word, "Processed");
        assert_eq!(status.icon, icons::WARN);
        assert_eq!(status.icon_color(), colors().result.warning);
        assert_eq!(status.word_color(), colors().result.warning);
    }

    #[test]
    fn integrity_d9_identity_undo_stays_with_its_campaign() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Copy.md");
        let before = format!("---\nsession_id: {}\n---\nDM's words\n", Uuid::new_v4());
        fs::write(&path, &before).unwrap();
        // Round 30 B7e: a refused write says plain words, never the system's, and leaves the
        // file's bytes as they were.
        let failing = |code: Option<i32>| super::super::FAILING_WRITES.with(|c| c.set(code));
        failing(Some(13));
        assert_eq!(
            super::super::new_identity(&path).err().as_deref(),
            Some(
                "Not saved. No permission to save. Give your user account permission to change the file and its folder."
            )
        );
        failing(None);
        assert_eq!(fs::read_to_string(&path).unwrap(), before);
        let undo = super::super::new_identity(&path).unwrap();
        let after = fs::read(&path).unwrap();
        failing(Some(28));
        assert_eq!(
            undo.undo().err().as_deref(),
            Some("Not saved. Disk is full. Free some space on the disk, then try again.")
        );
        failing(None);
        assert_eq!(fs::read(&path).unwrap(), after);
        let mut sessions = Sessions {
            folder: Some(dir.path().into()),
            ..Default::default()
        };
        sessions.separate.record(kept::Separate {
            page: sessions.page(),
            undo,
        });
        let here = sessions.page();
        sessions.open_campaign(None);
        // Another campaign shows none of it; the file keeps the new identity.
        assert!(sessions.separate.get(&sessions.page()).is_none());
        assert!(sessions.separate.get(&here).is_some());
        assert_eq!(fs::read(path).unwrap(), after);
    }

    #[test]
    fn integrity_d16_a_same_named_world_note_keeps_ambiguous_links() {
        let dir = tempfile::tempdir().unwrap();
        let id = Uuid::new_v4();
        let path = dir.path().join("Session.md");
        fs::write(
            &path,
            format!("---\nsession_id: {id}\n---\n[[Ash Pack#Members]]\n"),
        )
        .unwrap();
        fs::write(
            dir.path().join("Ash Pack.md"),
            "# Ash Pack\nA place, not the group.",
        )
        .unwrap();
        let mut sessions = Sessions {
            campaign: Some(Campaign {
                id: Uuid::new_v4(),
                name: "Campaign".into(),
                path: dir.path().into(),
                dndbeyond_campaign_id: None,
                dndbeyond_campaign_name: None,
            }),
            folder: Some(dir.path().into()),
            selected: Some(id),
            draft: Some(Draft::open(&path).unwrap()),
            ..Default::default()
        };
        sessions.rename_links("Ash Pack", "Cinder Pack");
        assert_eq!(
            sessions.draft.as_ref().unwrap().body.as_str(),
            "[[Ash Pack#Members]]\n"
        );
        assert!(sessions.rename_problem.is_some());
        fs::remove_file(dir.path().join("Ash Pack.md")).unwrap();
        sessions.rename_links("Ash Pack", "Cinder Pack");
        assert_eq!(
            sessions.draft.as_ref().unwrap().body.as_str(),
            "[[Cinder Pack#Members]]\n"
        );
    }

    /// Round 26 FM5: a moved or gone note is a warning in the header and the rail, as its
    /// banner is. A failed write, which looks the same in words, stays an error.
    #[test]
    fn a_moved_note_warns_and_a_failed_write_stays_an_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Session 1.md");
        fs::write(&path, "# Session 1\n").unwrap();
        let mut draft = Draft::open(&path).unwrap();
        draft.state = DraftState::Missing;
        let moved = Sessions::note_save_status(&draft);
        assert_eq!(
            (moved.icon, moved.word, moved.color),
            (icons::WARN, "Notes not saved", colors().document.external)
        );
        draft.state = DraftState::Failed("This file is read only.".into());
        let failed = Sessions::note_save_status(&draft);
        assert_eq!(
            (failed.icon, failed.word, failed.color),
            (icons::ERROR, "Notes not saved", colors().document.invalid)
        );
    }

    #[test]
    fn link_update_undo_expires_on_navigation_note_edits_and_event_edits() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(
            dir.path(),
            &NewNote {
                number: 1,
                title: "",
                date: "2026-11-03",
                campaign: "Test",
                template: "Original words.",
            },
        )
        .unwrap();
        let mut sessions = Sessions {
            notes: vec![note.clone()],
            ..Default::default()
        };
        let mut book = Book::default();
        for edit in 0..3 {
            sessions.updated_links = Some(Vec::new());
            sessions.link_undo_state = Some(LinkUndoState {
                notes: sessions.note_bodies(),
                events: book.events.clone(),
            });
            sessions.expire_event_link_undo(true, &book);
            assert!(sessions.updated_links.is_some());
            match edit {
                0 => sessions.expire_event_link_undo(false, &book),
                1 => {
                    fs::write(&note.path, "Changed outside Splat.").unwrap();
                    sessions.expire_event_link_undo(true, &book);
                }
                _ => {
                    book.add(Kind::Day, None);
                    sessions.expire_event_link_undo(true, &book);
                }
            }
            assert!(sessions.updated_links.is_none());
        }
    }

    #[test]
    fn event_link_updates_save_and_undo_as_one_batch_without_overwriting_external_edits() {
        let dir = tempfile::tempdir().unwrap();
        let mut book = Book::default();
        let id = book.add(Kind::LongRest, None);
        let body = format!(
            "## What happened\n\n[[Timeline#^{id}|Long rest]] and [[Timeline#^{id}|our rest]].\n"
        );
        let notes: Vec<_> = (1..=2)
            .map(|number| {
                create_note(
                    dir.path(),
                    &NewNote {
                        number,
                        title: "",
                        date: "2026-01-01",
                        campaign: "Test",
                        template: &body,
                    },
                )
                .unwrap()
            })
            .collect();
        let original: Vec<_> = notes
            .iter()
            .map(|note| fs::read(&note.path).unwrap())
            .collect();
        book.events[0].title = "Rest at the mill".into();
        book.touch();
        // Merely renaming never writes a note.
        for (note, before) in notes.iter().zip(&original) {
            assert_eq!(fs::read(&note.path).unwrap(), *before);
        }
        let changes = notes
            .iter()
            .map(|note| {
                let draft = Draft::open(&note.path).unwrap();
                let (updated, count) = updated_event_links(&draft.body, &book);
                assert_eq!(count, 1);
                (draft, updated)
            })
            .collect();
        let mut sessions = Sessions::default();
        sessions.update_event_link_notes(changes);
        assert_eq!(sessions.updated_links.as_ref().unwrap().len(), 2);
        for note in &notes {
            let saved = fs::read_to_string(&note.path).unwrap();
            assert!(saved.contains("|Rest at the mill]]") && saved.contains("|our rest]]"));
        }
        sessions.undo_event_link_notes();
        assert!(sessions.updated_links.is_none());
        for (note, before) in notes.iter().zip(&original) {
            assert_eq!(fs::read(&note.path).unwrap(), *before);
        }
        // An external edit after the preview cannot be overwritten by accepting it.
        let draft = Draft::open(&notes[0].path).unwrap();
        let updated = updated_event_links(&draft.body, &book).0;
        let outside = format!(
            "{}External edit.\n",
            fs::read_to_string(&notes[0].path).unwrap()
        );
        fs::write(&notes[0].path, &outside).unwrap();
        sessions.update_event_link_notes(vec![(draft, updated)]);
        assert_eq!(fs::read_to_string(&notes[0].path).unwrap(), outside);
        assert!(sessions.updated_links.is_none());
        // Review of PR 255, item 4: a blocked second target stops the whole update. First the
        // second note's open draft is in conflict, then its file changed after the preview.
        for (note, before) in notes.iter().zip(&original) {
            fs::write(&note.path, before).unwrap();
        }
        let prepare = || -> Vec<(Draft, String)> {
            notes
                .iter()
                .map(|note| {
                    let draft = Draft::open(&note.path).unwrap();
                    let updated = updated_event_links(&draft.body, &book).0;
                    (draft, updated)
                })
                .collect()
        };
        let mut changes = prepare();
        changes[1].0.state = DraftState::Conflict;
        let mut sessions = Sessions::default();
        sessions.update_event_link_notes(changes);
        for (note, before) in notes.iter().zip(&original) {
            assert_eq!(fs::read(&note.path).unwrap(), *before);
        }
        assert!(sessions.updated_links.is_none() && sessions.link_problem.is_some());
        let changes = prepare();
        let outside = format!(
            "{}External edit.\n",
            fs::read_to_string(&notes[1].path).unwrap()
        );
        fs::write(&notes[1].path, &outside).unwrap();
        let mut sessions = Sessions::default();
        sessions.update_event_link_notes(changes);
        assert_eq!(fs::read(&notes[0].path).unwrap(), original[0]);
        assert_eq!(fs::read_to_string(&notes[1].path).unwrap(), outside);
        assert!(sessions.updated_links.is_none() && sessions.link_problem.is_some());
        // Re-check 2 of PR 255. Note A sits beside the others, note B in its own folder.
        use std::os::unix::fs::PermissionsExt;
        for (note, before) in notes.iter().zip(&original) {
            fs::write(&note.path, before).unwrap();
        }
        let folder_b = dir.path().join("Folder B");
        fs::create_dir(&folder_b).unwrap();
        let note_a = notes[0].path.clone();
        let note_b = folder_b.join(notes[1].path.file_name().unwrap());
        fs::rename(&notes[1].path, &note_b).unwrap();
        let open = |path: &std::path::Path| {
            let draft = Draft::open(path).unwrap();
            let updated = updated_event_links(&draft.body, &book).0;
            (draft, updated)
        };
        let temporary_files = || -> Vec<String> {
            [dir.path(), folder_b.as_path()]
                .iter()
                .flat_map(|folder| fs::read_dir(folder).unwrap().flatten())
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .filter(|name| name.starts_with(".tmp"))
                .collect()
        };
        // (a) After A is written, B is written outside. B is refused; A's result, error and
        // Undo stay, also through the expiry.
        let b_outside = format!(
            "{}Written outside.\n",
            String::from_utf8_lossy(&original[1])
        );
        let b_path = note_b.clone();
        let b_text = b_outside.clone();
        super::AFTER_LINK_WRITE.with(|hook| {
            *hook.borrow_mut() = Some(Box::new(move |written: &std::path::Path| {
                if written.file_name() != b_path.file_name() {
                    fs::write(&b_path, &b_text).unwrap();
                }
            }));
        });
        let mut sessions = Sessions::default();
        sessions.update_event_link_notes(vec![open(&note_a), open(&note_b)]);
        super::AFTER_LINK_WRITE.with(|hook| *hook.borrow_mut() = None);
        assert_eq!(fs::read_to_string(&note_b).unwrap(), b_outside);
        let a_written = fs::read_to_string(&note_a).unwrap();
        assert!(a_written.contains("|Rest at the mill]]"));
        sessions.expire_event_link_undo(false, &book);
        assert_eq!(sessions.updated_links.as_ref().map(Vec::len), Some(1));
        let problem = sessions.link_problem.clone().unwrap_or_default();
        let a_name = note_a.file_name().unwrap().to_string_lossy().into_owned();
        assert!(problem.contains(&a_name), "{problem}");
        // (d) While that result is held, with the notes listed, a frame away from Timeline
        // reads no note.
        sessions.notes = list_notes(dir.path());
        assert!(!sessions.notes.is_empty());
        sessions.link_undo_state = Some(LinkUndoState {
            notes: Vec::new(),
            events: book.events.clone(),
        });
        super::NOTE_READS.with(|reads| reads.set(0));
        sessions.expire_event_link_undo(false, &book);
        sessions.expire_event_link_undo(false, &book);
        assert_eq!(super::NOTE_READS.with(std::cell::Cell::get), 0);
        // (b) A is replaced by a note with another session id and the same words. Undo refuses
        // it, names it, and leaves it byte for byte.
        let replaced = a_written.replace(&notes[0].id.to_string(), &Uuid::new_v4().to_string());
        assert_ne!(replaced, a_written);
        fs::write(&note_a, &replaced).unwrap();
        let refused = sessions.undo_event_link_notes();
        assert_eq!(fs::read_to_string(&note_a).unwrap(), replaced);
        // Lane N0: the typed cause of each refusal, which also gives its tone (R41).
        use design::results::{Cause, File, Name, Recovery, Step};
        let replaced_in_a = |refused: Option<LinkUndoRefused>| {
            refused.is_some_and(|refused| {
                refused.cause == Cause::Replaced(File::SessionNote)
                    && refused.within.as_str() == "Session 1"
            })
        };
        assert!(replaced_in_a(refused.clone()), "{refused:?}");
        // (e) Re-check 3: A holds the written bytes again, and is replaced right before Undo
        // reads it. Undo refuses the replacement and names A.
        fs::write(&note_a, &a_written).unwrap();
        let a_path = note_a.clone();
        let swap = replaced.clone();
        super::BEFORE_UNDO_READ.with(|hook| {
            *hook.borrow_mut() = Some(Box::new(move |_: &std::path::Path| {
                fs::write(&a_path, &swap).unwrap();
            }));
        });
        let refused = sessions.undo_event_link_notes();
        super::BEFORE_UNDO_READ.with(|hook| *hook.borrow_mut() = None);
        assert_eq!(fs::read_to_string(&note_a).unwrap(), replaced);
        assert!(replaced_in_a(refused.clone()), "{refused:?}");
        // (c) After A is written, B's folder takes no new file. B is not written, A's result
        // stays, and no temporary file is left in either folder.
        fs::write(&note_a, &original[0]).unwrap();
        fs::write(&note_b, &original[1]).unwrap();
        let locked = folder_b.clone();
        super::AFTER_LINK_WRITE.with(|hook| {
            *hook.borrow_mut() = Some(Box::new(move |_: &std::path::Path| {
                fs::set_permissions(&locked, fs::Permissions::from_mode(0o555)).unwrap();
            }));
        });
        let mut sessions = Sessions::default();
        sessions.update_event_link_notes(vec![open(&note_a), open(&note_b)]);
        super::AFTER_LINK_WRITE.with(|hook| *hook.borrow_mut() = None);
        let left = temporary_files();
        fs::set_permissions(&folder_b, fs::Permissions::from_mode(0o755)).unwrap();
        assert_eq!(left, Vec::<String>::new());
        assert_eq!(fs::read(&note_b).unwrap(), original[1]);
        assert!(
            fs::read_to_string(&note_a)
                .unwrap()
                .contains("|Rest at the mill]]")
        );
        assert_eq!(sessions.updated_links.as_ref().map(Vec::len), Some(1));
        // Re-check 3: the retry writes B. The old "only A" warning ends, and Undo now puts back
        // only B; A keeps its links.
        let a_linked = fs::read(&note_a).unwrap();
        sessions.update_event_link_notes(vec![open(&note_b)]);
        assert!(
            sessions.link_problem.is_none(),
            "{:?}",
            sessions.link_problem
        );
        assert!(
            fs::read_to_string(&note_b)
                .unwrap()
                .contains("|Rest at the mill]]")
        );
        sessions.expire_event_link_undo(true, &book);
        assert_eq!(sessions.updated_links.as_ref().map(Vec::len), Some(1));
        sessions.undo_event_link_notes();
        assert_eq!(fs::read(&note_b).unwrap(), original[1]);
        assert_eq!(fs::read(&note_a).unwrap(), a_linked);
        assert!(sessions.updated_links.is_none() && sessions.link_problem.is_none());
        // (f) Review of PR 459, item 4 (the M7 caller map): a partial Undo whose failure is a
        // save, not an edit. A and B are written; then B's folder takes no new file, and B
        // itself does not change. Undo puts A back; B's save fails, so B stays byte for byte as
        // the update wrote it, and only B's Undo waits. B was not edited: its typed cause is
        // `Cause::NotSaved(File::SessionNote)`, an error, never `Cause::EditedSince` (lane N0).
        fs::write(&note_a, &original[0]).unwrap();
        fs::write(&note_b, &original[1]).unwrap();
        let mut sessions = Sessions {
            folder: Some(dir.path().to_path_buf()),
            notes: vec![
                notes[0].clone(),
                Note {
                    path: note_b.clone(),
                    ..notes[1].clone()
                },
            ],
            ..Default::default()
        };
        let offer = sessions.event_link_offer(&book, &id).expect("link offer");
        sessions.accept_event_link_offer(offer, &book);
        sessions.expire_event_link_undo(true, &book);
        assert_eq!(sessions.updated_links.as_ref().map(Vec::len), Some(2));
        let owner = sessions.page();
        let result = sessions.link_updates.get(&owner).unwrap();
        let ticket = result.ticket_for_caller_test();
        let stale_ticket = result.ticket_for_caller_test();
        let b_written = fs::read(&note_b).unwrap();
        assert_ne!(b_written, original[1], "the update wrote B");
        fs::set_permissions(&folder_b, fs::Permissions::from_mode(0o555)).unwrap();
        sessions.undo_event_link_update(ticket);
        let left = temporary_files();
        let refusal = sessions
            .link_updates
            .get(&owner)
            .and_then(|shown| shown.refusal().cloned());
        sessions.expire_event_link_undo(true, &book);
        fs::set_permissions(&folder_b, fs::Permissions::from_mode(0o755)).unwrap();
        assert_eq!(left, Vec::<String>::new());
        assert_eq!(fs::read(&note_a).unwrap(), original[0], "A is put back");
        assert_eq!(
            fs::read(&note_b).unwrap(),
            b_written,
            "B did not change: no edit is the cause"
        );
        let waiting: Option<Vec<_>> = sessions.updated_links.as_ref().map(|writes| {
            writes
                .iter()
                .map(|write| write.before.path.clone())
                .collect()
        });
        assert_eq!(
            waiting,
            Some(vec![note_b.clone()]),
            "only B's Undo waits after the next expiry check"
        );
        // Lane N0 (n0-link-undo-cause): B's folder took no file, an error with its own cause,
        // words and tone, never `EditedSince`.
        let refusal = refusal.expect("B's Undo is refused");
        let expected = LinkUndoRefused {
            within: Name::new("Session 2").unwrap(),
            cause: Cause::NotSaved(File::SessionNote),
            recovery: Recovery::First(Step::Check(File::SessionNote)),
        }
        .refusal("Rest at the mill");
        assert_eq!(refusal, expected);
        assert_eq!(
            (refusal.words().as_str(), refusal.tone()),
            (
                "The links to Rest at the mill were not put back in Session 2. The session note \
                 could not be saved. Check the session note first.",
                Tone::Error
            ),
            "n0-link-undo-cause: a locked folder"
        );
        let result = sessions.link_updates.get(&owner).unwrap();
        assert_eq!(result.refusal(), Some(&refusal));
        sessions.undo_event_link_update(result.ticket_for_caller_test());
        assert_eq!(fs::read(&note_a).unwrap(), original[0]);
        assert_eq!(fs::read(&note_b).unwrap(), original[1], "retry restores B");
        assert!(sessions.updated_links.is_none());
        assert!(sessions.link_updates.get(&owner).is_none());

        let offer = sessions
            .event_link_offer(&book, &id)
            .expect("new link offer");
        sessions.accept_event_link_offer(offer, &book);
        sessions.undo_event_link_update(stale_ticket);
        assert_eq!(
            fs::read(&note_b).unwrap(),
            b_written,
            "stale Undo leaves B alone"
        );
        assert_eq!(sessions.updated_links.as_ref().map(Vec::len), Some(2));
        // (g) Lane N0: B is the open note, and its file changes outside Splat after the update.
        // Undo puts A back; B's save finds the outside change and keeps it: a conflict, a
        // warning, never `EditedSince`.
        fs::write(&note_a, &original[0]).unwrap();
        fs::write(&note_b, &original[1]).unwrap();
        let mut sessions = Sessions::default();
        sessions.update_event_link_notes(vec![open(&note_a), open(&note_b)]);
        assert_eq!(sessions.updated_links.as_ref().map(Vec::len), Some(2));
        sessions.draft = Some(Draft::open(&note_b).unwrap());
        let b_outside = format!("{}Written outside.\n", fs::read_to_string(&note_b).unwrap());
        fs::write(&note_b, &b_outside).unwrap();
        let refused = sessions
            .undo_event_link_notes()
            .expect("B's Undo is refused");
        assert_eq!(fs::read(&note_a).unwrap(), original[0], "A is put back");
        assert_eq!(fs::read_to_string(&note_b).unwrap(), b_outside);
        assert_eq!(refused.cause, Cause::Conflict(File::SessionNote));
        let refusal = refused.refusal("Rest at the mill");
        assert_eq!(
            (refusal.words().as_str(), refusal.tone()),
            (
                "The links to Rest at the mill were not put back in Session 2. The session note \
                 has unsaved words, and its file changed on disk. Choose which words to keep in \
                 the session note first.",
                Tone::Warning
            ),
            "n0-link-undo-cause: an outside change of the open B"
        );
        // Issue #569: renamed to a blank title, the offer (and so its result, `Links now say …`,
        // and a refused Undo) names the links' own words, the reference label, never “”.
        fs::write(
            &note_a,
            format!("## What happened\n\n[[Timeline#^{id}|Rest at the mill]]\n"),
        )
        .unwrap();
        book.events[0].title = String::new();
        book.touch();
        let sessions = Sessions {
            notes: vec![Note {
                path: note_a.clone(),
                ..notes[0].clone()
            }],
            ..Default::default()
        };
        let offer = sessions
            .event_link_offer(&book, &id)
            .expect("a blank title still offers to update the links");
        let label = timeline::reference_label(&book.events[0], 0);
        assert!(!label.is_empty(), "the reference label is never blank");
        assert_eq!(offer.title, label, "the offer names the links' words");
    }

    /// #597: the page's notice is one entry of its store. A newer notice replaces the older one
    /// (also its arrived flag), and a success about one session ends when the DM moves on in
    /// that session, never in another.
    #[test]
    fn the_page_notice_is_one_entry_of_its_store() {
        let folder = tempfile::tempdir().unwrap();
        let (a, b) = (Uuid::from_u128(1), Uuid::from_u128(2));
        let mut sessions = Sessions {
            folder: Some(folder.path().to_path_buf()),
            ..Sessions::default()
        };
        let notice = |sessions: &Sessions| {
            sessions
                .notice()
                .map(|notice| (notice.tone, notice.words.clone(), notice.session))
        };
        sessions.say_arrived(Tone::Success, "“Session 1” is processed.".into(), Some(a));
        assert!(
            sessions.notice_arrived(),
            "a job's notice arrived by itself"
        );
        sessions.say(Tone::Error, "Not saved.".into(), None);
        assert_eq!(
            notice(&sessions),
            Some((Tone::Error, "Not saved.".to_owned(), None)),
            "n0-notice-one-entry: the newer notice replaces the older"
        );
        assert!(
            !sessions.notice_arrived(),
            "n0-notice-one-entry: the newer notice is the page's, not the job's"
        );
        sessions.say(Tone::Success, "Recording deleted.".into(), Some(a));
        sessions.selected = Some(b);
        sessions.clear_success();
        assert!(notice(&sessions).is_some(), "another session keeps it");
        sessions.selected = Some(a);
        sessions.clear_success();
        assert_eq!(notice(&sessions), None, "its own session ends it");
    }

    #[test]
    fn event_link_update_changes_old_titles_and_keeps_custom_aliases() {
        let mut book = Book::default();
        let id = book.add(Kind::LongRest, None);
        book.events[0].title = "Rest at the mill".into();
        book.renamed_links
            .insert(id.clone(), vec!["Long rest".into()]);
        let original = format!(
            "## What happened\n\n[[Timeline#^{id}|Long rest]] and [[Timeline#^{id}|Long rest]].\n[[Timeline#^{id}|our rest]]\n"
        );
        let (updated, count) = updated_event_links(&original, &book);
        assert_eq!(count, 2);
        assert_eq!(updated.matches("|Rest at the mill]]").count(), 2);
        assert!(updated.contains("|our rest]]"));
        assert_eq!(updated_event_links(&updated, &book).1, 0);
    }

    #[test]
    fn recap_uses_prose_and_skips_link_only_lines() {
        let body = "## Recap of last time\n\n## What happened\n\n- [[Timeline#^t-rest|Long rest]]\n\nWe reached the mill.\n";
        let none = || None;
        assert_eq!(
            recap_preview(body, none).as_deref(),
            Some("We reached the mill.")
        );
        assert_eq!(
            recap_preview("## What happened\n- [[Bridge]]\n", none),
            None
        );
        // Row 16: `What happened` first, as carry-over reads it, then the processed recap.
        let both = "## Recap of last time\nWe won.\n\n## What happened\nWe ran.";
        assert_eq!(recap_preview(both, none).as_deref(), Some("We ran."));
        let processed = || Some("The wolves came.".to_owned());
        assert_eq!(recap_preview(both, processed).as_deref(), Some("We ran."));
        let recap_only = "## Recap of last time\nWe won.\n\n## What happened\n";
        assert_eq!(
            recap_preview(recap_only, processed).as_deref(),
            Some("The wolves came.")
        );
        // With neither, the note's first written line, never its title, date, older recap or plan.
        let template = "# Session 1: The Road\n\n2026-01-01, Emberfall\n\n## Recap of last time\n\nWe won.\n\n\
                        ## Plan for tonight\n\n- [ ] Meet [[Hobb]]\n\n## What happened\n\n## Open threads\n\n\
                        - [ ] The miller Hobb told the party about the toll.\n";
        // The Last time line reads the note without its own title and date, as Read does.
        let note = Note {
            id: uuid::Uuid::nil(),
            number: 1,
            title: "The Road".into(),
            date: "2026-01-01".into(),
            path: PathBuf::from("Session 1.md"),
            also: Vec::new(),
        };
        let preview = |text: &str| recap_preview(&without_own_heading(text, &note), none);
        assert_eq!(
            preview(template).as_deref(),
            Some("The miller Hobb told the party about the toll.")
        );
        assert_eq!(recap_preview(recap_only, none), None);
        // Words before any section are written words too (review of PR 277, finding 4).
        assert_eq!(
            preview(
                "# Session 1: The Road\n\n2026-01-01, Marches\n\nWe found the missing ferry.\n"
            )
            .as_deref(),
            Some("We found the missing ferry.")
        );
    }

    #[test]
    fn read_plan_summary_keeps_mixed_notes_and_repeated_fights_in_the_file() {
        let fights = [PlanFight {
            saved: None,
            id: uuid::Uuid::nil(),
            name: "Bridge Ambush".into(),
            path: String::new(),
            summary: Default::default(),
            revision: None,
            difficulty: Default::default(),
        }];
        let source = "## Plan for tonight\n\n- [[Bridge Ambush]]\n- [[Bridge Ambush]]\n\n## What happened\n\nWe crossed the bridge.\n";
        let compact = compact_plan_for_reading(
            source,
            &fights,
            &[],
            &plan::PlanModel::build(source, &fights, &[], None).scene_lines(),
            true,
        );
        assert!(!compact.contains("Plan for tonight"));
        assert!(!compact.contains("[[Bridge Ambush]]"));
        assert!(compact.contains("We crossed the bridge."));
        assert_eq!(source.matches("[[Bridge Ambush]]").count(), 2);
        let tab = compact_plan_for_reading(
            source,
            &fights,
            &[],
            &plan::PlanModel::build(source, &fights, &[], None).scene_lines(),
            false,
        );
        assert!(tab.contains("2 fights, in the plan"));
        assert!(!tab.contains("at left"));
        let empty = source.replace("We crossed the bridge.", "");
        assert!(!has_written_text(&compact_plan_for_reading(
            &empty,
            &fights,
            &[],
            &plan::PlanModel::build(&empty, &fights, &[], None).scene_lines(),
            true
        )));
        let repeated = format!(
            "## Recap of last time\n\n- [[Bridge Ambush]]\n- [[Bridge Ambush]]\n\n{source}"
        );
        let compact = compact_plan_for_reading(
            &repeated,
            &fights,
            &[],
            &plan::PlanModel::build(&repeated, &fights, &[], None).scene_lines(),
            true,
        );
        assert_eq!(compact.matches("[[Bridge Ambush]]").count(), 2);
        assert!(compact.starts_with("## Recap of last time\n\n- [[Bridge Ambush]]"));
        let mixed = source.replace("- [[Bridge Ambush]]\n\n##", "- Ask the ferryman.\n\n##");
        assert_eq!(
            compact_plan_for_reading(
                &mixed,
                &fights,
                &[],
                &plan::PlanModel::build(&mixed, &fights, &[], None).scene_lines(),
                true
            ),
            mixed
        );
    }

    #[test]
    fn iter_2_c_read_plan_compacts_groups_and_keeps_prose_and_unknown_links() {
        let groups = [std::sync::Arc::new(crate::encounter_run::RunGroup {
            id: Uuid::new_v4(),
            path: "Group.md".into(),
            name: "Ash Pack".into(),
            side: "enemies".into(),
            encounter: None,
            encounter_name: None,
            members: vec![],
            notes: String::new(),
            xp: 0,
            xp_unknown: vec![],
            unavailable: 0,
            unavailable_members: vec![],
            problem: None,
            warnings: vec![],
        })];
        let fights = [PlanFight {
            saved: None,
            id: uuid::Uuid::nil(),
            name: "Bridge Ambush".into(),
            path: String::new(),
            summary: Default::default(),
            revision: None,
            difficulty: Default::default(),
        }];
        for (links, count) in [
            ("- [[ash pack|The wolves]]", "1 group"),
            ("- [[Ash Pack]]\n- [[Bridge Ambush]]", "1 fight and 1 group"),
            ("- [[Ash Pack]]\n- [[Ash Pack]]", "1 group"),
            // A scene is a row beside the note too (T12.10).
            (
                "- [[Bridge Ambush]]\n- [ ] [[Captain Varra]]\n- [x] The miller is the informer\n- [[Ash Pack]]",
                "1 fight, 1 group and 2 scenes",
            ),
            ("- [ ] Meet Hobb", "1 scene"),
        ] {
            let source = format!(
                "## Plan for tonight\n\n{links}\n\n## What happened\n\nWe reached the mill.\n"
            );
            let before = source.clone();
            let compact = compact_plan_for_reading(
                &source,
                &fights,
                &groups,
                &plan::PlanModel::build(&source, &fights, &groups, None).scene_lines(),
                true,
            );
            assert!(!compact.contains("Plan for tonight"));
            assert!(compact.contains("We reached the mill."));
            assert!(
                compact_plan_for_reading(
                    &source,
                    &fights,
                    &groups,
                    &plan::PlanModel::build(&source, &fights, &groups, None).scene_lines(),
                    false
                )
                .contains(&format!("{count}, in the plan"))
            );
            assert_eq!(source, before);
        }
        for lines in [
            "- [[Ash Pack]]\nAsk the ferryman.",
            "- [[Unknown group]]",
            "- [[Ash Pack]] extra words",
            // A nested task line is not a scene, and a task line with a fight is not hidden.
            "- [ ] Meet Hobb\n  - [ ] Bring the coin",
            "- [ ] Meet Hobb\n  - [ ] Meet Hobb",
            "- [ ] [[Bridge Ambush]]",
        ] {
            let source = format!("## Plan for tonight\n\n{lines}\n");
            assert_eq!(
                compact_plan_for_reading(
                    &source,
                    &fights,
                    &groups,
                    &plan::PlanModel::build(&source, &fights, &groups, None).scene_lines(),
                    true
                ),
                source
            );
        }
    }

    #[test]
    fn read_view_keeps_inline_note_and_event_links_without_rewriting_the_note() {
        let mut book = Book::default();
        let id = book.add(Kind::LongRest, None);
        book.events[0].title = "Rest at the mill".into();
        let source = format!("A [[Bridge Ambush]] before [[Timeline#^{id}|Old name]].");
        let rendered = words_to_read(&source, &book);
        assert!(rendered.contains("[Bridge Ambush](<splat-note:Bridge Ambush>)"));
        assert!(rendered.contains(&format!("[Old name](<splat-note:Timeline#^{id}>)")));
        assert!(source.contains("|Old name]]"));
    }

    /// W14's one test for the session being played. Its callers are the Log stamp
    /// (`current_link`) and the party bar, and each gives the same answer in each case.
    #[test]
    fn the_session_being_played_is_one_test_for_the_log_stamp_and_the_party_bar() {
        let dir = tempfile::tempdir().unwrap();
        let note = |date: &str| Note {
            id: Uuid::new_v4(),
            number: 3,
            title: String::new(),
            date: date.to_owned(),
            path: dir.path().join("Session 3.md"),
            also: Vec::new(),
        };
        let link = Some("[[Session 3]]".to_owned());
        // (case, the current note's date, a fight open, a recording live, played)
        let cases = [
            ("dated today", Some("2026-05-10"), false, false, true),
            (
                "dated before today",
                Some("2026-05-09"),
                false,
                false,
                false,
            ),
            // The old tests disagreed here: the bar showed, and the Log stamp was "dated today".
            (
                "prep for a later date",
                Some("2026-05-17"),
                false,
                false,
                true,
            ),
            (
                "a fight on an older note",
                Some("2026-05-09"),
                true,
                false,
                true,
            ),
            (
                "a recording on an older note",
                Some("2026-05-09"),
                false,
                true,
                true,
            ),
            ("a fight with no session note", None, true, false, true),
        ];
        for (case, date, fight, recording, played) in cases {
            let mut sessions = Sessions {
                today: Some("2026-05-10".to_owned()),
                notes: date.map(note).into_iter().collect(),
                run_open: fight,
                ..Sessions::default()
            };
            let current = sessions.current();
            let expected = match (played, current) {
                (false, _) => Played::No,
                (true, None) => Played::NoNote,
                (true, Some(id)) => Played::Session(id),
            };
            if recording {
                let path = dir.path().join("live.wav");
                sessions.live = Some(Live {
                    session: Uuid::nil(),
                    label: "Session 3".to_owned(),
                    recorder: crate::recording::Recorder::start(
                        crate::recording::Tool::Simulated,
                        None,
                        path,
                    )
                    .unwrap(),
                });
            }
            assert_eq!(sessions.played_session(), expected, "{case}");
            assert_eq!(sessions.party_bar_shows(), played, "{case}: the party bar");
            assert_eq!(
                sessions.current_link(),
                if played && current.is_some() {
                    link.clone()
                } else {
                    None
                },
                "{case}: the Log stamp"
            );
            // The DM's own choice moves only the bar.
            sessions.party_bar = Some(!played);
            assert_eq!(
                sessions.party_bar_shows(),
                !played,
                "{case}: the DM's choice"
            );
            assert_eq!(sessions.played_session(), expected, "{case}");
        }
    }

    /// The DM's hidden list leaves the wide page only while a session stands in its place, and
    /// the header offers the toggle only where it does something. The choice outlives both.
    #[test]
    fn a_hidden_list_stays_out_only_while_a_session_is_open() {
        let dir = tempfile::tempdir().unwrap();
        let note = Note {
            id: Uuid::new_v4(),
            number: 1,
            title: String::new(),
            date: "2026-05-10".to_owned(),
            path: dir.path().join("Session 1.md"),
            also: Vec::new(),
        };
        let mut sessions = Sessions {
            folder: Some(dir.path().to_path_buf()),
            notes: vec![note.clone()],
            selected: Some(note.id),
            ..Sessions::default()
        };
        assert!(
            !sessions.list_hides(),
            "the list shows until the DM hides it"
        );
        assert!(sessions.offers_list_toggle(true));
        assert!(
            !sessions.offers_list_toggle(false),
            "a narrow page has its own back link"
        );
        sessions.toggle_list();
        assert!(sessions.list_hides());
        // No session open: the list is all there is to show, and the choice stays.
        for (case, selected) in [
            ("none chosen", None),
            ("its file went", Some(Uuid::new_v4())),
        ] {
            sessions.selected = selected;
            assert!(!sessions.list_hides(), "{case}");
            assert!(!sessions.offers_list_toggle(true), "{case}");
            assert!(sessions.list_hidden, "{case}");
        }
        sessions.selected = Some(note.id);
        assert!(
            sessions.list_hides(),
            "the session open again: the list stays hidden"
        );
        sessions.view = View::Templates;
        assert!(
            !sessions.offers_list_toggle(true),
            "Templates has its own list"
        );
        sessions.view = View::Notes;
        sessions.folder = Some(dir.path().join("gone"));
        assert!(
            !sessions.offers_list_toggle(true),
            "a missing folder shows its fault alone"
        );
        sessions.folder = Some(dir.path().to_path_buf());
        sessions.toggle_list();
        assert!(!sessions.list_hides());
    }

    /// Round 28 K4a, the New session form's facts: the chosen template is read from the folder
    /// at the press (changed or removed outside Splat since the form opened), a refused press
    /// keeps the form and its words, and the form belongs to its campaign (a switch ends it, and
    /// B's form never offers A's template).
    #[test]
    fn the_new_session_form_reads_its_template_at_the_press_and_keeps_to_its_campaign() {
        let dirs = [tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap()];
        let [a, b] = [0, 1].map(|at| {
            let campaign = Campaign {
                id: Uuid::from_u128(at + 1),
                name: format!("Campaign {at}"),
                path: dirs[at as usize].path().into(),
                dndbeyond_campaign_id: None,
                dndbeyond_campaign_name: None,
            };
            crate::content_workspace::CampaignDocument::new(&campaign)
                .unwrap()
                .save()
                .unwrap();
            let folder = super::super::open_folder(&campaign).unwrap();
            if at == 0 {
                fs::create_dir_all(folder.join(super::super::TEMPLATES)).unwrap();
                fs::write(folder.join(super::super::TEMPLATES).join("Mine.md"), "Mine").unwrap();
            }
            create_note(
                &folder,
                &NewNote {
                    number: 1,
                    title: "One",
                    date: "2026-11-01",
                    campaign: &campaign.name,
                    template: "",
                },
            )
            .unwrap();
            campaign
        });
        let mine = a
            .path
            .join("Sessions")
            .join(super::super::TEMPLATES)
            .join("Mine.md");
        let ctx = StateContext::new();
        let mut book = Book::default();
        let mut sessions = Sessions::default();
        sessions.poll(&ctx, Some(&a), &mut book);
        let open = |sessions: &mut Sessions, title: &str| {
            sessions.open_form();
            sessions.gv_form_set(gpui_facade::FormField::Template, "Mine");
            sessions.gv_form_set(gpui_facade::FormField::Title, title);
        };
        // Changed outside before Create: the note holds the template as the folder has it.
        open(&mut sessions, "Unsent");
        assert_eq!(
            sessions.gv_form().map(|form| form.template),
            Some("Mine".to_owned())
        );
        fs::write(&mine, "Changed outside").unwrap();
        sessions.gv_form_create();
        assert!(sessions.form.is_none(), "the session is made");
        let made = sessions
            .notes
            .iter()
            .find(|note| note.title == "Unsent")
            .map(|note| fs::read_to_string(&note.path).unwrap())
            .unwrap_or_default();
        assert!(
            made.contains("Changed outside"),
            "made from the folder's template: {made:?}"
        );
        // Removed outside before Create: the press is refused, the form keeps its words and
        // says why.
        open(&mut sessions, "Second");
        let before = sessions.notes.len();
        fs::remove_file(&mine).unwrap();
        sessions.gv_form_create();
        assert_eq!(
            sessions.notes.len(),
            before,
            "no session from a template that is gone"
        );
        let form = sessions.gv_form().expect("the form stays");
        assert_eq!(form.title, "Second", "the form keeps its words");
        assert!(
            form.reason == Some(FormReason::TemplateGone.words())
                || form.error.as_deref() == Some(FormReason::TemplateGone.words()),
            "the form says why: {:?} {:?}",
            form.reason,
            form.error
        );
        // Another campaign and back: the switch ends A's form; B's form offers B's templates.
        sessions.poll(&ctx, Some(&b), &mut book);
        assert!(sessions.form.is_none(), "a switch ends the form");
        sessions.open_form();
        assert_eq!(
            sessions.gv_form().map(|form| form.template),
            Some(super::super::BUILT_IN_TEMPLATES[0].0.to_owned())
        );
        assert!(
            sessions
                .gv_templates()
                .iter()
                .all(|template| template.name != "Mine"),
            "B's form never offers A's template"
        );
        sessions.poll(&ctx, Some(&a), &mut book);
        assert!(sessions.form.is_none(), "back in A, the form is not B's");
    }

    /// The run moved to this session keeps `Undo move <run>` in the page's footer, owned by this
    /// campaign's page: its Undo puts the event back, and an edit of the note or a change to the
    /// Timeline ends it, in the plain footer lines.
    #[test]
    fn move_run_undo_restores_the_event_and_expires_after_an_edit() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("note.md");
        fs::write(&path, "## What happened\n").unwrap();
        let old = Uuid::from_u128(1);
        let new = Uuid::from_u128(2);
        let run = Uuid::from_u128(3);
        let mut page = Sessions {
            folder: Some(folder.path().into()),
            draft: Some(Draft::open(&path).unwrap()),
            open_run: Some(("Bridge Ambush".into(), old)),
            selected: Some(new),
            ..Default::default()
        };
        let mut book = Book::default();
        book.add(Kind::Encounter, Some(old));
        book.events[0].run = Some(run);
        book.events[0].continued_in = Some(new);
        let before = book.events.clone();
        page.move_run_to_session(&mut book, run, new);
        assert_eq!(book.events[0].session, Some(new));
        assert_eq!(book.events[0].continued_in, None);
        // The act's result is kept in its store, owned by this campaign's page.
        assert!(page.moves.get(&page.page()).is_some());
        // The footer's own Undo, by its ticket. M0483: the Undo names the run it moves back.
        let frame = PaneFrame::new();
        let mut messages = Messages::new();
        page.kept_lines(&frame, &book, &mut messages);
        let lines = messages.into_plain();
        let undo = lines
            .iter()
            .find(|line| {
                line.undo
                    .as_ref()
                    .is_some_and(|undo| undo.label == "Undo move Bridge Ambush")
            })
            .and_then(gpui_facade::FooterLine::press_undo)
            .expect("the footer offers Undo move Bridge Ambush");
        let _ = page.kept_press(undo, &mut book);
        assert_eq!(book.events, before);
        assert_eq!(page.open_run.as_ref().unwrap().1, old);
        assert!(page.moves.is_empty());
        // An edit of the note ends a kept move.
        page.move_run_to_session(&mut book, run, new);
        assert!(!page.moves.is_empty());
        page.draft.as_mut().unwrap().body.push_str("A note.");
        page.expire_move(&book);
        assert!(page.moves.is_empty());
        // So does a change to the Timeline.
        page.move_run_to_session(&mut book, run, old);
        assert!(!page.moves.is_empty());
        book.add(Kind::Travel, Some(new));
        page.expire_move(&book);
        assert!(page.moves.is_empty());
    }
}
