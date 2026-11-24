//! The Sessions page: notes, templates and recordings, one campaign at a time. The
//! recorder and the processing job live here, and keep going while the DM uses other pages.
use super::{
    Apply, DEFAULT_FOLDER, Draft, DraftState, Dropped, NewNote, Note, NoteUndo, Recording,
    Template, body_of, create_note, create_template, delete_recording, folder_setting,
    fresh_line_in_note, insert_in_note, insert_in_plan, linked_names, list_notes, list_recordings,
    list_templates, nested_in_note, new_recording_path, parts_of, plain_links, recording_title,
    rename_links, rename_links_in_file, save_folder, section_text, unreadable_notes,
    update_details, without_empty_sections, without_own_heading,
};
use crate::design::text_draft::TextDraft;
use crate::{
    credentials::{self, KeySlot},
    design::{
        self, Action, CreateReasons, Density, Status, Tone, Type, colors, icons,
        outcome::{
            BodyMessage, Choice, Fault, FaultMessage, KeptOutcome, Lost, MessageChoice, Messages,
            Notice, PagePlaceholder, PaneFrame, Press, RowMessages, Said, Sentence, Severity,
            UndoStep, Verb,
        },
        results::ByTicket,
    },
    encounters::PlanFight,
    lookup::{Written, WrittenIn},
    markdown_view,
    persistence::Campaign,
    processing::{self, Job, Request, settings::ProcessingSettings},
    recording::{Health, Recorder},
    timeline::{self, Book, EventDisplay, kind_icon, page::SessionLink},
};
use eframe::egui::{self, Color32};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use uuid::Uuid;

#[path = "carry.rs"]
mod carry;
#[path = "kept.rs"]
mod kept;
#[path = "plan.rs"]
mod plan;
/// What the GPUI view reads and does (`src/gui/pages/sessions/`, `docs/gpui-port.md`).
#[path = "gpui_facade.rs"]
pub mod gpui_facade;
#[cfg(any(test, feature = "test-support"))]
pub use plan::scene_fit_counts;

const MOVE_FILES: &str = "Move files and save";
const SAVED_WORD_FOR: Duration = Duration::from_secs(8);
const LIST_WIDTH: f32 = 216.0;
const LIST_WIDTHS: std::ops::RangeInclusive<f32> = 170.0..=420.0;
const TWO_COLUMNS_FROM: f32 = 640.0;
/// The note's column beside the plan: the DM drags its edge into the room beside it.
const NOTE_COLUMN: &str = "session_note_frame";
/// The note frame's padding and hairline, on both sides.
const NOTE_FRAME_EDGES: f32 = 2.0 * (design::S + design::HAIRLINE_WIDTH);
/// The header's toggle of the session list, by what a press does.
/// The session list's toggle: one name in both states; its pressed state says whether the list
/// is shown (review of PR 618).
pub const LIST_TOGGLE: &str = "Session list";
/// The page header's actions (M0498).
const NEW_SESSION: design::Word = crate::word!("New session");
/// The template editor's title line (M0493).
const MORE: design::Word = crate::word!("More");
const CUSTOMIZE: design::Word = crate::word!("Customize");
const NEW_TEMPLATE: design::Word = crate::word!("New template");
const STOP_RECORDING: design::Word = crate::word!("Stop recording");
const TEMPLATES: design::Word = crate::word!("Templates");
const SHOW_PARTY: design::Word = crate::word!("Show party");
/// The plan and the note stand side by side from here.
const PLAN_BESIDE_NOTE_FROM: f32 = 720.0;
const PLAN_WIDTH: f32 = 330.0;
const PLAN_MAX_WIDTH: f32 = 480.0;
const CHECK_DISK_EVERY: Duration = Duration::from_secs(1);
const SCAN_EVERY: Duration = Duration::from_secs(5);
const MISSING_INPUT_ERROR: &str = "no such device";
const PROCESSING_CANCELLED: &str = "Processing cancelled";
const SESSION_TITLE: design::FieldName = crate::field_id!("session_title");
const SESSION_DATE: design::FieldName = crate::field_id!("session_date");
const TEMPLATE_NAME: design::FieldId =
    crate::field_id!("template_name").once(design::Once::CreateForm);
const SESSION_FOLDER: design::FieldName = crate::field_id!("session_folder");
const SESSION_TEMPLATE: design::FieldName = crate::field_id!("session_template");
const CREATE_SESSION: design::Word = crate::word!("Create session");
const SAVE_DETAILS: design::Word = crate::word!("Save details");
const CANCEL: design::Word = crate::word!("Cancel");
/// A session note's text: on the Sessions page, and in the Notes panel beside another page.
const NOTE_ON_PAGE: design::FieldName = crate::field_id!("session_note_text");
const NOTE_IN_PANEL: design::FieldName = crate::field_id!("session_note_text_panel");
/// A template's text in the template editor.
const TEMPLATE_TEXT: design::FieldName = crate::field_id!("session_template_text");
const CREATE_TEMPLATE: design::Word = crate::word!("Create template");

crate::create_reasons! {
    /// Why the New session form's Create (or the details form's Save details) waits.
    enum FormReason {
        Number => "Enter the session number as a whole number.",
        Date => "Enter the date as YYYY-MM-DD.",
        TemplateGone => "The chosen template is gone from the folder. Choose another.",
    }
}

crate::create_reasons! {
    /// Why the New template form's Create waits.
    enum TemplateReason {
        NoName => "Type a name first.",
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
const RECORDING_VIEW: design::FieldName = crate::field_id!("recording_view");
const RECORDING_PART: design::FieldName = crate::field_id!("recording_part");

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

/// How often `refresh` ended the session folder's listing on this thread.
#[cfg(any(test, feature = "test-support"))]
pub fn folder_scan_ends() -> usize {
    FOLDER_SCAN_ENDS.with(std::cell::Cell::get)
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

/// What one recording row holds of the tab's single places: the primary action, the
/// destination's help line, and `Record again` (`record_again_row`).
#[derive(Clone, Copy)]
struct RowRoles {
    primary: bool,
    destination_here: bool,
    record_again_here: bool,
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
    /// A press in the plan's part of the footer: the plan acts on it (`plan_pane`).
    Plan(plan::FooterAct),
}

/// Which footer holds the session page's own results and notices (`session_messages`): those
/// of the session, of its note and of its recordings. None of them is the plan's, so the plan's
/// pane holds them only where it is the one pane in view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PageFooter {
    /// The narrow Plan tab: the plan is all the pane shows, and its one footer holds the plan's
    /// results and the page's.
    Plan,
    /// The session pane's own footer, below all it shows: plan and note side by side (across
    /// both, below the taller), the note, the recordings, a conflict.
    Session,
}

impl SessionView {
    /// Round 30 `session-pane-footer-owner` (FF3, FM9, FN5, SN3, AN4): a result stands in the
    /// pane of its object. The page's results belong to the session, its note or a recording,
    /// so they stand in the footer of the pane that holds the whole session, never in the plan's
    /// column beside the note. The one place that decides it: `session_pane` and
    /// `session_content` both ask here.
    fn page_footer(&self) -> PageFooter {
        match self {
            Self::Tab {
                shown: Tab::Plan,
                beside: false,
                ..
            } => PageFooter::Plan,
            Self::Tab { .. } | Self::Conflict => PageFooter::Session,
        }
    }
}

/// What the session page's pane shows, as its header worked it out.
enum SessionView {
    /// The note's editor, to choose between the draft and the file.
    Conflict,
    Tab {
        shown: Tab,
        beside: bool,
        editing_group_here: bool,
        /// The edges the header was drawn in, for the body (`SessionLayout`).
        edges: design::PaneEdges,
    },
}

/// Layout step 7: what the session pane shows and the edges that its title line, its toolbar and
/// its body share, decided before the title line paints (`Sessions::session_layout`).
struct SessionLayout {
    conflict: bool,
    beside: bool,
    editing_group_here: bool,
    shown: Tab,
    /// Beside the note: the plan, a gutter and the note's column, whose edge the DM drags. Else
    /// one column as wide as the note's frame (also for the narrow Plan tab and a conflict), or
    /// the whole room (Recordings, and a group being edited in the narrow Plan tab).
    edges: design::PaneEdges,
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
    pub group_editor: std::cell::RefCell<&'a mut dyn FnMut(&mut egui::Ui)>,
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
    /// The jump to the marker's line, when a marker opened this, until it lands (`MarkJump`).
    mark: Option<MarkJump>,
    /// The marker that opened this, with its key, so it can be added from here.
    marker: Option<(crate::timeline::Marker, String)>,
    /// A raw transcript that stops early: parts done, and parts in all.
    short_of: Option<(u64, u64)>,
    /// The shown part as the reader draws it, made once per part (`ShownPart`).
    shown: Option<ShownPart>,
}

/// The jump that opens a text at its marker's line (M0481; review of PR 862). The line's top is
/// measured again in every draw until the jump lands, and the column's typed jump is asked again
/// from that measure (ruling M18), so a width change before the column moves keeps it right. It
/// ends when the line is at the view's top, when the column moved no further since the last ask
/// (the text ends too soon), when the DM's own input moves the column (`reveal::dm_scrolls`,
/// the column's one cancel rule), or after `MarkJump::ASKS` asks.
struct MarkJump {
    /// The marker's words in the text.
    words: String,
    /// Asks left.
    left: u8,
    /// The line's distance below the view's top when it last asked.
    last: Option<f32>,
}

impl MarkJump {
    const ASKS: u8 = 4;

    fn new(words: String) -> Self {
        Self {
            words,
            left: Self::ASKS,
            last: None,
        }
    }

    /// Whether to ask the column again, for the line `below` the view's top in this draw; the
    /// DM's own input in the column this pass is `moved`.
    fn ask(&mut self, below: f32, moved: bool) -> bool {
        let landed = below.abs() <= 0.5;
        let stuck = self.last.is_some_and(|last| (last - below).abs() <= 0.5);
        if moved || landed || stuck || self.left == 0 {
            return false;
        }
        self.left -= 1;
        self.last = Some(below);
        true
    }
}

/// The shown part's words as the reader draws them (the draft's warning line out; with several
/// parts, the part's own heading out) and where the marker's line starts in them (review of PR
/// 862, P3): made once per part, so a quiet frame copies and scans nothing. The parts are read
/// once when the text opens, so the part is the words' revision.
struct ShownPart {
    part: usize,
    body: String,
    at: Option<usize>,
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
        let at = reading
            .marker
            .as_ref()
            .map(|(marker, _)| marker.reading_words())
            .or_else(|| reading.mark.as_ref().map(|mark| mark.words.clone()))
            .and_then(|words| body.find(&words));
        Self {
            part: reading.part,
            body,
            at,
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
    /// G118: the plan's kept lines, one per planned fight (`plan::PlanLines`), and the
    /// planned fights they were last trimmed to. Ended by `forget_plan_lines` (plan change,
    /// session change, campaign change); a new content key replaces an entry.
    plan_lines: plan::PlanLines,
    /// The planned fights whose lines `plan_lines` keeps, each with its day: its revision keys
    /// those lines with each roster's (ruling M6), so a new day fits them again.
    plan_lines_seen: crate::design::memo::Revised<Vec<(plan::PlanLineId, Option<String>)>>,
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
    /// The plan's closed rows as their recipe laid them, kept by their inputs (layout 11c A2c).
    plan_rows: plan::PlanRowsCache,
    /// The plan's measured region, while its body is one (layout 11c A2c-3): its offset, its
    /// requests and its mask.
    plan_region: Option<crate::design::RegionState>,
    /// Whether the plan's body was its measured region in the last frame (A2c-3).
    pub(super) plan_measured: bool,
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
    focus_fight_search: bool,
    draft: Option<Draft>,
    form: Option<Form>,
    templates: Vec<Template>,
    template: Option<String>,
    template_draft: Option<Draft>,
    /// The kept answer of the open note's compare with a move result's words (layout PR 9b-C).
    same_moved: design::text_draft::KeptCompare,
    /// The shown built-in template's name and its words as a document (layout PR 9b).
    builtin_text: Option<(String, design::text_draft::TextDraft)>,
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
    /// The note editor drawn last, and whose note it edits. Read it through
    /// `note_editor_id`, which gives it only for the open note.
    last_note_editor: Option<NoteEditor>,
    move_run_here: Option<(Uuid, String)>,
    /// The open run moved to another session, with `Undo move`.
    moves: design::Results<kept::Page, kept::Moved>,
    /// When a recording last stopped, and the campaign it was recorded in: the owner, so the
    /// rail says it was kept only while that campaign is open (another campaign's rail never
    /// says it). The owner update (`settle_words`) ends it `SAVED_WORD_FOR` after, on the frame
    /// clock (`crate::frame_clock`); no reader reads the clock.
    saved_at: Option<(Option<Uuid>, f64)>,
    /// The timed recording words as of the last owner update (`Recording saved` shown, `Hearing
    /// sound` shown), with their revision (`recording_revision`).
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
    /// A modal is open over the app (its dialogs, the full-window image, any egui modal: the
    /// app's one test, `art::modal_open`). Ctrl+S waits for it, as every shortcut does.
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
    focus_group_search: bool,
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
    /// The `/` menu in the note, and what its command has open.
    slash: super::slash::Slash,
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

/// A pane body that manages its own room, for one of the pane's closed reasons.
#[allow(
    clippy::disallowed_methods,
    reason = "the one named escape of the pane: `Fills::because` (design::results::STAGED_ALLOWS)"
)]
fn fills_body(reason: design::pane::FillsReason) -> design::pane::Body {
    design::pane::Body::Fills(design::pane::Fills::because(reason))
}

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
    One(Color32),
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

    pub fn icon_color(self) -> Color32 {
        match self.look {
            RecordingLook::Kind(kind) => kind.icon_color(),
            RecordingLook::One(color) => color,
        }
    }

    pub fn word_color(self) -> Color32 {
        match self.look {
            RecordingLook::Kind(kind) => kind.word_color(),
            RecordingLook::One(color) => color,
        }
    }

    /// What a status painter paints: this status's icon and colours, with `words` (its word, or
    /// its word and time) in place of its word.
    pub fn tinted(self, words: &str) -> design::Tinted<'_> {
        match self.look {
            RecordingLook::Kind(kind) => design::KindStatus::new(kind, words).into(),
            RecordingLook::One(color) => Status {
                icon: self.icon,
                word: words,
                color,
            }
            .into(),
        }
    }
}

/// Words that wrap at the width they are given.
fn plan_summary(summary: &str) -> String {
    summary
        .split(" · ")
        .map(|part| part.replace(' ', "\u{a0}"))
        .collect::<Vec<_>>()
        .join(" ·\u{a0}")
        .replace("Ended ·\u{a0}Round", "Ended\u{a0}·\u{a0}Round")
}

/// The recording count belongs to the tab's label and click target.
fn recordings_tab(ui: &mut egui::Ui, shown: &mut Tab, count: usize) -> bool {
    design::counted_tab(
        ui,
        shown,
        Tab::Recordings,
        "Recordings",
        count,
        ("recording", "recordings"),
    )
}

fn wrapped_words(ui: &mut egui::Ui, kind: Type, words: impl Into<String>, color: Color32) {
    ui.add(egui::Label::new(kind.rich(ui, words).color(color)).wrap());
}

/// A title and its date wrap together, and expose the full title to assistive tools.
fn note_title(ui: &mut egui::Ui, title: &str, date: &str) {
    let mut job = egui::text::LayoutJob::default();
    job.append(
        title,
        0.0,
        egui::TextFormat {
            font_id: Type::Title.font(ui),
            color: colors().text,
            ..Default::default()
        },
    );
    job.append(
        &format!("  {date}"),
        0.0,
        egui::TextFormat {
            font_id: Type::Small.font(ui),
            color: colors().muted,
            ..Default::default()
        },
    );
    let response = ui.add(egui::Label::new(job).wrap());
    design::named(ui, &response, &format!("{title} · {date}"));
}

/// One column of the wide page, the full height of the page.
/// The note's column, whose edge the DM drags. Until then it has the reader's measure and its
/// frame, and the room beside it stays empty.
fn note_column() -> design::Column {
    design::Column::left(
        NOTE_COLUMN,
        design::READER_MEASURE + NOTE_FRAME_EDGES,
        design::PANEL_MIN_WIDTH..=f32::INFINITY,
    )
    .page_keeps(0.0)
}

/// The note frame's width, as the note's column has it: dragged, or the reader's measure. Write
/// and Read share it, in both layouts, so a switch moves no line end.
fn note_frame_width(ctx: &egui::Context) -> f32 {
    design::column_width(ctx, NOTE_COLUMN).unwrap_or(design::READER_MEASURE + NOTE_FRAME_EDGES)
}

/// A note's words as a reader sees them: links as words, and no empty headings.
/// A Markdown note's face: styled by `markdown_source::layouter` in the prose `font`, with its
/// folds opened at the caret, laid out once per change (layout PR 9b).
fn markdown_face(font: egui::FontId) -> design::pane::Face {
    design::pane::Face::Laid {
        layout: crate::markdown_source::layouter,
        font,
        folds: true,
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

/// The campaign's content index, as every reader of the page finds it.
fn reader_index(ui: &egui::Ui) -> Option<std::sync::Arc<crate::lookup::ContentIndex>> {
    ui.ctx().data(|data| {
        data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
            "reader_link_index",
        ))
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

/// Recording results follow the column with the same text inset as Write, and keep the reader
/// measure: prose and its rules never fill a wide pane.
fn note_width<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    ui.scope(|ui| {
        ui.set_max_width((ui.available_width() - 2.0 * design::S).min(design::READER_MEASURE));
        add(ui)
    })
    .inner
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

/// Words at the note's caret. For a new thought after the end of a line of words, they start
/// a line of their own, and with no words only that break goes in. It is the note's own edit,
/// so the note's Undo history stays.
fn insert_at_caret(
    ctx: &egui::Context,
    id: egui::Id,
    text: &mut TextDraft,
    words: &str,
    new_line: bool,
) -> bool {
    let mut state = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    let Some(range) = state.cursor.char_range() else {
        if !words.is_empty() {
            design::edit_in_place(ctx, id, text, |body| insert_in_note(body, words));
            return true;
        }
        return false;
    };
    let at = range.primary.index;
    let chars: Vec<char> = text.chars().collect();
    let ends_words = at > 0
        && chars.get(at - 1).is_some_and(|c| *c != '\n')
        && chars.get(at).is_none_or(|c| *c == '\n');
    let added = if new_line && ends_words {
        format!("\n\n{words}")
    } else {
        words.to_owned()
    };
    if added.is_empty() {
        return false;
    }
    let byte = text
        .char_indices()
        .nth(at)
        .map_or(text.len(), |(byte, _)| byte);
    design::edit_in_place(ctx, id, text, |body| body.insert_str(byte, &added));
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::one(
            egui::text::CCursor::new(at + added.chars().count()),
        )));
    egui::TextEdit::store_state(ctx, id, state);
    true
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

/// Words the page adds to the open note (a reading's summary, a plan's link). It is the note's
/// own edit, so the note's Undo history stays for when the DM goes back to it.
/// A note editor's id and its owner, the session whose note it edits (`Sessions::selected`
/// when it drew). The page keeps the last one drawn.
#[derive(Clone, Copy)]
struct NoteEditor {
    session: Option<Uuid>,
    id: egui::Id,
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

/// A list row whose title wraps, because a session's title is often long.
fn session_row(
    ui: &mut egui::Ui,
    selected: bool,
    title: &str,
    // The note's file name, when another row has the same title and date.
    file: Option<&str>,
    detail: &str,
    // The detail in full words, when the drawn one is short.
    spoken: Option<&str>,
    state: Option<RecordingStatus>,
) -> egui::Response {
    let c = colors();
    let width = ui.available_width() - 2.0 * design::S;
    let font = if selected { Type::Strong } else { Type::Body };
    let galley = ui
        .painter()
        .layout(title.to_owned(), font.font(ui), c.text, width);
    let second = session_detail(ui, file, detail, state, width);
    let lines = galley.size().y + second.size().y;
    let height = (lines + design::M).max(Density::Glance.control() + design::S);
    let spoken = spoken.unwrap_or(detail);
    design::row(
        ui,
        height,
        selected,
        &match file {
            Some(file) => format!("{title} · {file}\n{spoken}"),
            None => format!("{title} · {spoken}"),
        },
        |ui, rect| {
            let top = rect.center().y - lines / 2.0;
            let x = rect.left() + design::S;
            let first = galley.size().y;
            ui.painter().galley(egui::pos2(x, top), galley, c.text);
            // The second line holds the list lock's dimming itself, and a live recording's
            // status keeps its own colours under it (R5, `design::StatusText`).
            second.paint(ui, egui::pos2(x, top + first));
        },
    )
}

fn session_detail(
    ui: &egui::Ui,
    file: Option<&str>,
    detail: &str,
    state: Option<RecordingStatus>,
    width: f32,
) -> design::StatusText {
    // Summary parts stay whole. A file name is words: it wraps at a space, and its next line
    // starts where the first did.
    let detail = file
        .into_iter()
        .map(str::to_owned)
        .chain(detail.lines().map(plan_summary))
        .collect::<Vec<_>>()
        .join("\n");
    // The status's own look: one colour, or a kind's (R5: a live recording's icon alone is red,
    // its word in the text colour).
    let word = state.map(|state| {
        let look = match state.look {
            RecordingLook::One(color) => design::InTextLook::One(color),
            RecordingLook::Kind(kind) => design::InTextLook::Kind(kind),
        };
        (
            plan_summary(&format!("{} {}", state.icon, state.word)),
            look,
        )
    });
    design::status_in_text(
        ui,
        &detail,
        word.as_ref().map(|(word, look)| (word.as_str(), *look)),
        width,
    )
}

/// After Discard, a note with no file left: the editor shows its last saved words.
const DISCARDED_GONE: &str = "This file is gone from the folder. The editor shows its last saved words. Save them as a new file to keep them.";

/// A choice of the record line's warning (`record_line_body`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RecordAct {
    PlayThisOne,
    Start,
    NotNow,
}

/// A choice of a recording alert in the bottom banner (`banner`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AlertAct {
    Input,
    Retry,
    Record,
    Open,
    Stop,
    Dismiss,
}

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

/// A recording row's sentence wraps at the reader's line length, as its destination does. The
/// row's rule and menu keep the list's width.
fn row_sentence(ui: &mut egui::Ui, words: egui::RichText) -> egui::Response {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(design::READER_MEASURE));
        ui.add(egui::Label::new(words).wrap())
    })
    .inner
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
    pub fn poll(&mut self, ctx: &egui::Context, active: Option<&Campaign>, book: &mut Book) {
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

    /// Lead ruling, round 25: a session notice that arrives on its own (processing finished,
    /// a file changed, moved or copied elsewhere, a save that failed) sits in the bottom banner,
    /// so it never moves the tabs or the note the DM types in. The result of the DM's own
    /// action keeps its place above the tabs. Each one wraps at `LIST_MEASURE`.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn banner_notices(&mut self, ui: &mut egui::Ui) {
        let notes = self.on_page && self.view == View::Notes && self.form.is_none();
        if notes && self.other_recording_shown() {
            self.other_recording_line(ui);
        }
        if notes
            && self.notice_arrived()
            && !self.processed_notice_read_here()
            && (self.notice_session().is_none() || self.notice_session() == self.selected)
            && let Some((tone, words)) = self
                .notice()
                .map(|notice| (notice.tone, notice.words.clone()))
        {
            // N6: the close icon on the words' first line, placed by the message part (M0441).
            // Work that finished by itself (`is processed`) is a result said where it happened,
            // with no way back (`BodyMessage::result`); a fault keeps its tone.
            let sentence = notice_sentence(&words);
            let dismiss = Choice::new(MessageChoice::Dismiss(Lost::Nothing), ());
            // In the banner's padding, as every banner message with a close icon.
            let message = match tone {
                Tone::Error => BodyMessage::from(
                    FaultMessage::new(&PageFault::Failed(sentence)).choice(dismiss),
                ),
                Tone::Warning => BodyMessage::from(
                    FaultMessage::new(&PageFault::Check(sentence)).choice(dismiss),
                ),
                Tone::Success => BodyMessage::result(sentence, [dismiss]),
                Tone::Info => BodyMessage::from(Notice::new(sentence).choice(dismiss)),
            };
            let closed = message.in_padding().show(ui);
            if closed.is_some() {
                self.end_notice();
            }
        }
        if notes
            && let Some(note) = self
                .selected
                .and_then(|id| self.notes.iter().find(|note| note.id == id))
                .cloned()
        {
            for copy in &note.also {
                let filename = copy.file_name().unwrap_or_default().to_string_lossy();
                let fault = PageFault::Check(notice_sentence(&format!(
                    "{} and {filename} claim to be the same session.",
                    note.path.file_name().unwrap_or_default().to_string_lossy()
                )));
                if BodyMessage::from(FaultMessage::new(&fault).choice(Choice::named(
                    crate::choice!("Make {} a separate session", filename),
                    (),
                )))
                .show(ui)
                .is_some()
                {
                    match super::new_identity(copy) {
                        Ok(undo) => {
                            self.separate.record(kept::Separate {
                                page: self.page(),
                                undo,
                            });
                            self.refresh();
                        }
                        Err(error) => self.say(Tone::Error, error, None),
                    }
                }
            }
        }
        if notes && !self.unreadable.is_empty() {
            let fault = PageFault::Check(notice_sentence(&format!(
                "Splat cannot read the properties of {}, so it is not listed. The file is safe. Repair it to bring it back.",
                self.unreadable.join(", ")
            )));
            let _ = BodyMessage::<()>::from(FaultMessage::new(&fault)).show(ui);
        }
        // Round 30 `note-conflict-cross-page-banner` (SM2): the session note's warning and its
        // choices stand on every page, not only where the note is in view, so the DM can choose
        // where the rail's `Notes changed` is read. Its words name the note, from the one value
        // that every page uses (`note_name`).
        let name = self.note_name();
        if let Some(draft) = self.draft.as_mut().filter(|draft| draft.needs_attention()) {
            let act = Self::draft_warning(ui, draft, name.as_deref());
            keep_dropped(&mut self.dropped, draft, act);
        }
        if self.on_page
            && self.view == View::Templates
            && let Some(draft) = self
                .template_draft
                .as_mut()
                .filter(|draft| draft.needs_attention())
        {
            let act = Self::draft_warning(ui, draft, None);
            keep_dropped(&mut self.dropped, draft, act);
        }
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

    /// What cannot wait, on every page. It sits at the bottom, so that it never moves `Next
    /// turn` under a pointer parked on it. A silence takes no banner: it is often only a quiet
    /// table, and it shows in the rail and on the run's own line.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    pub fn banner(
        &mut self,
        ctx: &egui::Context,
        settings: &ProcessingSettings,
        book: &Book,
        run_open: bool,
    ) {
        let primary_owner = self.banner_primary_owner(settings, run_open);
        let stalled = self.stalled_elsewhere().map(|live| {
            format!(
                "The recording of “{}” stopped saving audio. The first {} are kept. Record again starts a new file.",
                live.label,
                spoken_length(live.recorder.seconds())
            )
        });
        if stalled.is_none()
            && self
                .alerts
                .iter()
                .all(|alert| self.recording_row_here(alert.session))
            && !self.has_banner_notices()
        {
            return;
        }
        let mut retry = None;
        let mut record_again = None;
        let mut open = None;
        let mut stop = false;
        // A pane above the banner ends its body at a whole row, as at a footer (N12).
        design::pane::note_bottom_banner(ctx);
        egui::TopBottomPanel::bottom("session_banner")
            .frame(
                egui::Frame::new()
                    .fill(colors().raised)
                    .inner_margin(egui::Margin::symmetric(design::M as i8, design::XS as i8)),
            )
            .show(ctx, |ui| {
                // Review of PR 255: many notices at once never push a recovery choice out of the
                // window. The banner is at most a third of it, and scrolls; the recording alerts
                // stay first, at its top.
                let most = ui.ctx().content_rect().height() / 3.0;
                // A choice that gains keyboard focus comes whole into view (M0450): the
                // column's one focus arbiter, by the pane's rules.
                design::ScrollColumn::new("session_banner_scroll")
                    .follows_focus()
                    .max_height(most)
                    // Short content takes its own height; long content takes the whole third.
                    .min_scrolled_height(most)
                    .fit_height()
                    .show(ui, |ui| {
                        if let Some(words) = &stalled {
                            let session = self.live.as_ref().map(|live| live.session);
                            let here = self.on_page && self.selected == session;
                            // The banner's one primary, when it is this alert's (M0445).
                            let record = if primary_owner == Some(usize::MAX) {
                                MessageChoice::Primary("Record again")
                            } else {
                                MessageChoice::Choice("Record again")
                            };
                            let fault = PageFault::Failed(notice_sentence(words));
                            let mut message = FaultMessage::new(&fault)
                                .choice(Choice::new(record, AlertAct::Record));
                            if !here {
                                message = message.choice(Choice::new(
                                    MessageChoice::Choice("Open the session"),
                                    AlertAct::Open,
                                ));
                            }
                            message = message.choice(Choice::new(
                                MessageChoice::Choice("Stop recording"),
                                AlertAct::Stop,
                            ));
                            match BodyMessage::from(message).in_padding().show(ui) {
                                Some(AlertAct::Record) => record_again = session,
                                Some(AlertAct::Open) => open = session,
                                Some(AlertAct::Stop) => stop = true,
                                _ => {}
                            }
                        }
                        let mut dismiss = None;
                        for (index, alert) in self.alerts.iter().enumerate() {
                            if self.recording_row_here(alert.session) {
                                continue;
                            }
                            let banner_primary = primary_owner == Some(index);
                            let missing_input = alert.can_record
                                && (settings.input_missing() || alert.missing_input());
                            let words = if alert.word == "Recording stopped" {
                                format!("{} {}", alert.words, recording_recovery(missing_input))
                            } else {
                                alert.words.clone()
                            };
                            // With the audio gone, no advice to process again is true. The reason
                            // stands beside the blocked action instead.
                            let audio_gone =
                                alert.retry.as_ref().is_some_and(|path| !path.exists());
                            let words = if audio_gone {
                                processing::without_retry_advice(&words)
                            } else {
                                words
                            };
                            let here = self.on_page
                                && self.selected == Some(alert.session)
                                && self.tab == Tab::Recordings;
                            let can_record = alert.can_record && self.live.is_none();
                            // One primary at most (M0447): with no input, Choose an input; else
                            // Continue processing; else Record again. The banner gives it to one
                            // alert (`primary_owner`).
                            let primary = if !banner_primary {
                                None
                            } else if missing_input {
                                Some(AlertAct::Input)
                            } else if alert.retry.is_some() {
                                Some(AlertAct::Retry)
                            } else if can_record {
                                Some(AlertAct::Record)
                            } else {
                                None
                            };
                            let word = |word: &'static str, act: AlertAct| {
                                let what = if primary == Some(act) {
                                    MessageChoice::Primary(word)
                                } else {
                                    MessageChoice::Choice(word)
                                };
                                Choice::new(what, act)
                            };
                            // The words keep their hover: what the recorder itself said. The
                            // choices start at the words; a blocked one says why under the
                            // message; Dismiss is the close icon on the words' first line.
                            let fault = PageFault::Failed(notice_sentence(&words));
                            let mut message = FaultMessage::new(&fault);
                            if let Some(said) = &alert.said {
                                message = message.hover(said.clone());
                            }
                            if missing_input {
                                message = message.choice(word("Choose an input", AlertAct::Input));
                            }
                            if alert.retry.is_some() {
                                let retry = word("Continue processing", AlertAct::Retry);
                                message = message.choice(if audio_gone {
                                    retry.blocked(crate::sentence!(
                                        "Audio removed. Restore the WAV file to process it."
                                    ))
                                } else if self.job.is_some() || self.live.is_some() {
                                    retry.blocked(crate::sentence!(
                                        "Stop recording or wait for processing to finish."
                                    ))
                                } else {
                                    retry
                                });
                            }
                            if can_record {
                                message = message.choice(word("Record again", AlertAct::Record));
                            }
                            if !here {
                                message = message.choice(word("Open the session", AlertAct::Open));
                            }
                            message = message.choice(Choice::new(
                                MessageChoice::Dismiss(Lost::Nothing),
                                AlertAct::Dismiss,
                            ));
                            match BodyMessage::from(message).in_padding().show(ui) {
                                Some(AlertAct::Input) => self.wants_recording_input = true,
                                Some(AlertAct::Retry) => {
                                    if let Some(path) = &alert.retry {
                                        retry = Some((alert.session, path.clone()));
                                    }
                                }
                                Some(AlertAct::Record) => record_again = Some(alert.session),
                                Some(AlertAct::Open) => open = Some(alert.session),
                                Some(AlertAct::Dismiss) => dismiss = Some(index),
                                Some(AlertAct::Stop) | None => {}
                            }
                        }
                        if let Some(index) = dismiss {
                            self.alerts.remove(index);
                        }
                        // After the recording alerts, in the banner's order: what arrived on its own.
                        self.banner_notices(ui);
                    });
            });
        if let Some((session, path)) = retry {
            self.process_recording(ctx, &path, settings, book, session);
        }
        if stop {
            self.finish_recording(ctx);
        }
        if let Some(session) = record_again {
            self.finish_recording(ctx);
            if Some(session) != self.selected {
                self.select(Some(session));
            }
            // Pressed where the notice cannot show, so a failure stays in the banner.
            if let Err(error) = self.start_recording(settings, session) {
                self.raise(Alert {
                    word: "Recording stopped",
                    words: format!("The recording did not start again. {error}"),
                    session,
                    can_record: true,
                    retry: None,
                    said: None,
                });
            }
        }
        if let Some(session) = open {
            self.wants_sessions_page = true;
            self.view = View::Notes;
            self.form = None;
            self.tab = Tab::Recordings;
            if Some(session) != self.selected {
                self.select(Some(session));
            }
        }
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
        self.forget_plan_lines(Vec::new());
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

    /// The pane's body kind. The Recordings tab is rows, a reader or a form: the pane's scroll
    /// column, so its footer follows short content. The note's editor fills its room (lead,
    /// P2), and the plan keeps its own footer and scroll until it moves to this footer.
    fn session_body_kind(&self, view: &SessionView) -> design::pane::Body {
        use design::pane::{Body, FillsReason};
        match view {
            SessionView::Conflict => fills_body(FillsReason::NoteEditor),
            SessionView::Tab {
                shown: Tab::Recordings,
                ..
            } => Body::Content,
            // Read is a read view: content (review of PR 335).
            SessionView::Tab {
                shown: Tab::Note, ..
            } if self.reading_note => Body::Content,
            SessionView::Tab {
                shown: Tab::Note, ..
            } => fills_body(FillsReason::NoteEditor),
            SessionView::Tab { .. } => fills_body(FillsReason::Columns),
        }
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
            old: renamed.clone(),
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

    /// A reveal that waits for its next draw, as one does just after an add or a travel act.
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_pending_reveal(&mut self, row: &str, pass: u64) {
        self.row_reveal = Some((row.to_owned(), 0, pass));
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

    /// The revision of the timed recording words (`Recording saved`, `Hearing sound`): it
    /// changes when one of them comes or goes, as of the last owner update, for a reader that
    /// keeps them. The rail and the page read the words each frame from constant state and
    /// keep no line yet, so only the tests read it; the retained footer (PR 2b) is its reader.
    #[cfg(test)]
    pub(crate) fn recording_revision(&self) -> design::memo::Revision {
        self.timed_words.0.revision()
    }

    /// The owner update of the page's timed results and words (ruling L9), at the end of
    /// `poll`, after the book's last update and before any page or rail reads them: each ends
    /// here, by the frame clock, and asks for a repaint at its next end, so it ends with no
    /// input. No reader reads the clock.
    fn settle_timed(&mut self, ctx: &egui::Context, book: &Book) {
        self.settle_plan_add(ctx, book);
        self.settle_words(ctx);
    }

    /// The plan's Add result: its life ends it (`Results::expire`); so does a book whose events
    /// no longer hold what the add left (their count, its label), checked only when the book's
    /// revision changed (a reload, a file event, an edit), never on an unchanged frame.
    fn settle_plan_add(&mut self, ctx: &egui::Context, book: &Book) {
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
    fn settle_words(&mut self, ctx: &egui::Context) {
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
    fn finish_recording(&mut self, ctx: &egui::Context) {
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
        ctx: &egui::Context,
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
        ctx: &egui::Context,
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

    fn record_control(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings) {
        // Stop is here whichever session is open. It can be urgent, if a player asks. The
        // state is the one both UIs draw (`gv_record_control`).
        match self.gv_record_control(settings) {
            gpui_facade::RecordControl::None => {}
            gpui_facade::RecordControl::Stop { name } => {
                if Action::secondary("Stop recording")
                    .icon(icons::STOP_RECORDING)
                    .name(&name)
                    .hint(&name)
                    .show(ui)
                    .clicked()
                {
                    self.finish_recording(ui.ctx());
                }
            }
            gpui_facade::RecordControl::Record { blocked: reason } => {
                if Action::secondary("Record")
                    .icon(icons::RECORD)
                    .name("Record this session")
                    .blocked(reason.as_deref())
                    .show(ui)
                    .clicked()
                {
                    self.confirm_record = true;
                }
                if let Some(reason) = reason {
                    ui.label(Type::Small.rich(ui, reason).color(colors().muted));
                    design::gap(ui, design::Gap::L);
                }
            }
        }
    }

    /// The live state, or the step before it. Always in the same place.
    fn record_line(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings) {
        if self.tab == Tab::Recordings
            && self.reading.is_none()
            && self
                .live
                .as_ref()
                .is_some_and(|live| Some(live.session) == self.selected)
        {
            return;
        }
        // While this session's recording runs, the line keeps one height, so the note below
        // never moves, also when the recording stalls or stops by itself. Another session's
        // recording is a notice in the message footer (round 26 FD3).
        let live = self
            .live
            .as_ref()
            .is_some_and(|live| Some(live.session) == self.selected);
        // The body asks to confirm a recording only while none runs, so this holds for its pass.
        let height = if live && !self.confirm_record {
            Density::Read.control() + design::XS
        } else {
            0.0
        };
        design::at_least(ui, height, |ui| self.record_line_body(ui, settings));
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

    /// Another session's recording: its state, time and session, in the bottom banner, so it
    /// never moves the title line, the tabs or the plan (round 26 FD3). As on the recording
    /// row, the state's colours are only on its icon and word (round 24 FC7), and they are the
    /// status's own: a live recording's icon in the `recording` token, its word in the text
    /// colour (R5). It stays out of the typed message footer until that footer's notice can
    /// colour a state word (a gap of the message part, sent to its owner).
    fn other_recording_line(&self, ui: &mut egui::Ui) {
        let (Some(live), Some(status)) = (&self.live, self.recording_status()) else {
            return;
        };
        let word = status.word;
        // The time is the state's (R5: in the word's colour); the session it is in is detail.
        let time = format!(" {}", clock(live.recorder.seconds()));
        let detail = format!(" in {}", live.label);
        let format = |color| egui::TextFormat {
            font_id: Type::Small.font(ui),
            color,
            ..Default::default()
        };
        let mut job = egui::text::LayoutJob::default();
        job.append(status.icon, 0.0, format(status.icon_color()));
        job.append(" ", 0.0, format(status.word_color()));
        job.append(word, 0.0, format(status.word_color()));
        job.append(&time, 0.0, format(status.word_color()));
        job.append(&detail, 0.0, format(colors().muted));
        ui.add(egui::Label::new(job).wrap())
            .on_hover_text(format!("{word}{time}{detail}"));
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "the record line’s warning stands where recording starts, with its choices: a typed body message (M20)"
    )]
    fn record_line_body(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings) {
        let c = colors();
        if let Some(live) = self
            .live
            .as_ref()
            .filter(|live| Some(live.session) == self.selected)
        {
            let time = clock(live.recorder.seconds());
            let status = self.recording_status();
            match (live.recorder.health(), status) {
                (Health::Stalled | Health::Stopped(_), _) | (_, None) => {}
                (health, Some(status)) => {
                    let (word, color) = match health {
                        Health::Starting => ("Waiting for sound", c.muted),
                        Health::Silent => ("Silent for over a minute", c.muted),
                        // Only what is true now. A table goes quiet, and that is no warning.
                        _ if live.recorder.heard_just_now() => ("Hearing sound", c.muted),
                        _ => ("Quiet", c.muted),
                    };
                    ui.horizontal_wrapped(|ui| {
                        // The first thing this page must say, large enough for a glance. Its
                        // colours are the status's (R5): a live recording's icon only is red.
                        let words = if *health == Health::Silent {
                            status.word.to_owned()
                        } else {
                            format!("{} {time}", status.word)
                        };
                        let mut job = egui::text::LayoutJob::default();
                        for (part, color) in [
                            (status.icon, status.icon_color()),
                            (" ", status.word_color()),
                            (words.as_str(), status.word_color()),
                        ] {
                            Type::Strong.rich(ui, part).color(color).append_to(
                                &mut job,
                                ui.style(),
                                egui::FontSelection::Default,
                                egui::Align::Center,
                            );
                        }
                        ui.label(job);
                        let detail = if *health == Health::Silent {
                            format!("{word} · {time} recorded")
                        } else {
                            word.to_owned()
                        };
                        ui.label(Type::Small.rich(ui, detail).color(color));
                        if *health == Health::Silent
                            && Action::quiet("Check the input in Settings")
                                .density(Density::Read)
                                .show(ui)
                                .clicked()
                        {
                            self.wants_processing_settings = true;
                        }
                    });
                }
            }
        } else if self.live.is_none() && self.confirm_record {
            let label = self
                .selected
                .and_then(|id| self.notes.iter().find(|note| note.id == id))
                .map_or_else(String::new, |note| format!("“{}”", note.label()));
            let today = self.today();
            let dated = self
                .selected
                .and_then(|id| self.notes.iter().find(|note| note.id == id))
                .and_then(|note| dated_before_today(note, &today));
            let other = if self.selected != self.current() {
                " This is not the session being played.".to_owned()
            } else {
                dated
                    .as_ref()
                    .map_or_else(String::new, |words| format!(" {words}"))
            };
            let words = format!(
                "Record {label}.{other} Tell everyone at the table first. The audio stays in the campaign folder until you select Process."
            );
            let tone = if other.is_empty() {
                Tone::Info
            } else {
                NotTonight::TONE
            };
            // The choices start at the message text (M0453): `Start recording` is the screen's
            // one primary unless a run is open (its `Next turn` is).
            let sentence = notice_sentence(&words);
            let notice = if tone == Tone::Info {
                Notice::new(sentence)
            } else {
                NotTonight::notice(sentence)
            };
            let mut notice = notice;
            if dated.is_some() && self.selected == self.current() {
                notice = notice.choice(Choice::new(
                    MessageChoice::Choice("Play this one"),
                    RecordAct::PlayThisOne,
                ));
            }
            let start = if self.run_open {
                MessageChoice::Choice("Start recording")
            } else {
                MessageChoice::Primary("Start recording")
            };
            let pressed =
                BodyMessage::from(notice.choice(Choice::new(start, RecordAct::Start)).choice(
                    Choice::new(MessageChoice::Choice("Not now"), RecordAct::NotNow),
                ))
                .show(ui);
            match pressed {
                Some(RecordAct::PlayThisOne) => self.play_this_one(),
                Some(RecordAct::Start) => {
                    self.confirm_record = false;
                    if let Some(session) = self.selected
                        && let Err(error) = self.start_recording(settings, session)
                    {
                        self.say(Tone::Error, error, None);
                    }
                }
                Some(RecordAct::NotNow) => self.confirm_record = false,
                None => {}
            }
        }
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

    fn note_status(ui: &mut egui::Ui, draft: &Draft) {
        design::status(ui, Self::note_save_status(draft));
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

    /// Every state a draft can be in, for the statuses a title line can show (its stable room).
    fn every_draft_state() -> [DraftState; 6] {
        [
            DraftState::Clean,
            DraftState::Saved,
            DraftState::Unsaved,
            DraftState::Conflict,
            DraftState::Missing,
            DraftState::Failed(String::new()),
        ]
    }

    /// The message footer of a pane whose body is one draft's editor: the draft's own result
    /// with its Undo, and for a template, the template action's error. Returns where the body
    /// ends.
    fn draft_messages<'f>(
        &self,
        frame: &'f PaneFrame,
        id: &str,
        template: bool,
        width: f32,
    ) -> Messages<'f, SessionAct> {
        let mut messages = Messages::new(frame, (id, self.selected), width);
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

    /// What the file did to a draft: changed on disk, gone or moved, or not saved. It arrives on
    /// its own, so it is drawn in the bottom banner, never above the editor.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    /// A draft's warning and its choices. `name` names a session note (`note_name`); a
    /// template's warning says `This note`.
    fn draft_warning(ui: &mut egui::Ui, draft: &mut Draft, name: Option<&str>) -> Option<DraftAct> {
        let words = |text: &str| notice_sentence(&owned_words(name, text));
        // A new file could not be made: why, above the choices that are still open.
        if draft.state == DraftState::Missing
            && let Some(error) = &draft.save_again_error
        {
            let fault = PageFault::Failed(words(error));
            let _ = BodyMessage::<()>::from(FaultMessage::new(&fault)).show(ui);
        }
        // Every choice starts at its message's text; a red one keeps `L` before it (the part's).
        let choice = |word, act| Choice::new(MessageChoice::Choice(word), act);
        let lose = |word, act| Choice::new(MessageChoice::Lose(word), act);
        let pressed = match draft.state.clone() {
            DraftState::Conflict => {
                let fault = PageFault::Check(words(
                    "This note changed on disk. Your draft is safe until you choose. Other sessions open after that.",
                ));
                BodyMessage::from(
                    FaultMessage::new(&fault)
                        .choice(choice("Overwrite with my draft", DraftAct::Overwrite))
                        .choice(lose("Reload and lose my draft", DraftAct::Reload)),
                )
                .show(ui)
            }
            // After Discard with no file left, the editor holds the last saved words, not a
            // draft: the one way on is a new file.
            DraftState::Missing if matches!(draft.dropped, Some(Dropped::Discarded(_))) => {
                let fault = PageFault::Check(words(DISCARDED_GONE));
                BodyMessage::from(FaultMessage::new(&fault).choice(choice(
                    "Save these words as a new file",
                    DraftAct::SaveAgain,
                )))
                .show(ui)
            }
            DraftState::Missing => {
                let moved = draft.moved_to.as_ref().map(|path| {
                    path.strip_prefix(&draft.search_folder)
                        .unwrap_or(path)
                        .display()
                        .to_string()
                });
                let fault = PageFault::Check(words(&missing_note_words(moved.clone())));
                let mut message = FaultMessage::new(&fault);
                if let Some(moved) = &moved {
                    message = message.choice(Choice::named(
                        crate::choice!("Save my draft to {}", moved),
                        DraftAct::SaveMoved,
                    ));
                }
                BodyMessage::from(
                    message
                        .choice(choice("Save my draft as a new file", DraftAct::SaveAgain))
                        .choice(lose("Discard", DraftAct::Discard)),
                )
                .show(ui)
            }
            DraftState::Failed(error) => {
                let fault = PageFault::Failed(words(&error));
                BodyMessage::from(
                    FaultMessage::new(&fault).choice(choice("Try to save again", DraftAct::Save)),
                )
                .show(ui)
            }
            _ => None,
        };
        if let Some(act) = &pressed {
            apply_draft_act(draft, act);
        }
        pressed
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

    #[allow(
        clippy::disallowed_methods,
        reason = "the New session form’s act fault stands under the form: a typed body message (M20)"
    )]
    fn show_form(&mut self, ui: &mut egui::Ui) {
        let Some(mut form) = self.form.take() else {
            return;
        };
        let heading = if form.editing.is_some() {
            "Session details"
        } else {
            "New session"
        };
        design::heading(
            ui,
            design::Heading::Group {
                title: heading,
                count: 0,
            },
        );
        design::field(ui, "Number", None, None, |ui| {
            design::number_field(
                ui,
                "Number",
                &mut form.number,
                "",
                design::L * 5.0,
                Density::Read.control(),
            );
        });
        let reason = form_reason(&form, &self.templates);
        // The file the form makes, on the kept line once nothing blocks Create.
        let preview = match (&form.editing, &self.folder) {
            (None, Some(folder)) => {
                let path = super::note_path(folder, form.number.parse().unwrap_or(1), &form.title);
                let shown = path.strip_prefix(folder).unwrap_or(&path);
                Some(format!("File: {}", shown.display()))
            }
            _ => None,
        };
        // Round 28 K4a: the one inline creation form. The title, the date and, for a new
        // session, the Template drop-down; Create and Cancel on the action line after it; the
        // reason or the file on the kept line under them. Enter in a field creates, Escape
        // cancels.
        let word = if form.editing.is_some() {
            SAVE_DETAILS
        } else {
            CREATE_SESSION
        };
        let create = if self.open_run.is_some() {
            design::Create::secondary(word)
        } else {
            design::Create::primary(word)
        }
        .blocked(reason);
        let focus = std::mem::take(&mut self.focus_form_title);
        let fields = design::InlineCreate::new()
            .field(
                design::CreateField::new(
                    SESSION_TITLE.of(form.editing),
                    "Session title",
                    &mut form.title,
                )
                .help("Optional. You can name it after you play.")
                // The one field a DM usually types, often with people waiting.
                .focus(focus),
            )
            .field(
                design::CreateField::new(SESSION_DATE.of(form.editing), "Date", &mut form.date)
                    .help("As YYYY-MM-DD."),
            );
        let kept = design::KeptLine::new()
            .cancel(CANCEL, true)
            .preview(preview.as_deref());
        let mut names = self
            .templates
            .iter()
            .map(|template| (template.name.clone(), template.name.as_str()));
        let first = names.next().filter(|_| form.editing.is_none());
        let shown = match first {
            Some((first, label)) => fields
                .dropdown(design::CreateDropdown::new(
                    "Template",
                    design::layout::Selection::new(
                        SESSION_TEMPLATE.once(design::Once::CreateForm),
                        "Session template",
                        first,
                        label,
                    )
                    .choices(names),
                    &mut form.template,
                ))
                .create(create),
            None => fields.create(create),
        }
        .kept_line(kept)
        .show(ui);
        // A fault of the act itself, under the form, so it moves nothing above it.
        if let Some(error) = &form.error {
            let fault = PageFault::Failed(notice_sentence(error));
            let _ = BodyMessage::<()>::from(FaultMessage::new(&fault)).show(ui);
        }
        if shown.cancel {
            return;
        }
        if shown.create {
            self.form = self.press_create(form);
        } else {
            self.form = Some(form);
        }
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

    fn list_pane(&mut self, ui: &mut egui::Ui) {
        let locked = self.draft.as_ref().is_some_and(Draft::needs_attention);
        let mut choose = None;
        let live = self.live.as_ref().map(|live| live.session);
        design::ScrollColumn::new("session_list")
            .full_width()
            .show(ui, |ui| {
                ui.add_enabled_ui(!locked, |ui| {
                    for note in &self.notes {
                        let job = self.job.as_ref().map(|(_, session)| *session);
                        let result = self.processed_session;
                        let state = if live == Some(note.id) || job == Some(note.id) {
                            self.rail_recording_status()
                        } else {
                            self.alerts
                                .iter()
                                .rev()
                                .find(|alert| alert.session == note.id)
                                .map(|alert| {
                                    let cancelled = alert.word == PROCESSING_CANCELLED;
                                    RecordingStatus::one(Status {
                                        icon: if cancelled { icons::WARN } else { icons::ERROR },
                                        word: if cancelled {
                                            "Cancelled"
                                        } else if alert.can_record {
                                            "Stopped"
                                        } else {
                                            "Failed"
                                        },
                                        color: if cancelled {
                                            colors().result.warning
                                        } else {
                                            colors().result.error
                                        },
                                    })
                                })
                        };
                        // A warned result is a status like its siblings: icon, word and colour.
                        // The full words stay in hover and in the row's name.
                        // A clean result is a status too, in the success colour.
                        let warned = state.is_none() && result == Some(note.id) && self.job_warned;
                        let clean = state.is_none() && result == Some(note.id) && !warned;
                        let state = if warned {
                            Some(RecordingStatus::one(Status {
                                word: "Processed",
                                ..processed_warning()
                            }))
                        } else if clean {
                            Some(RecordingStatus::one(processed_clean()))
                        } else {
                            state
                        };
                        let detail = if let Some(state) = state {
                            format!("{} {}", state.icon, state.word)
                        } else {
                            note.date.clone()
                        };
                        let detail = if self.current() == Some(note.id) {
                            let together = format!("{detail} · {} Playing", icons::RUN);
                            let width = ui.available_width() - 2.0 * design::S;
                            if ui
                                .painter()
                                .layout_no_wrap(
                                    together.clone(),
                                    Type::Small.font(ui),
                                    colors().muted,
                                )
                                .size()
                                .x
                                <= width
                            {
                                together
                            } else {
                                format!("{detail}\n{} Playing", icons::RUN)
                            }
                        } else {
                            detail
                        };
                        let duplicate = self.notes.iter().any(|other| {
                            other.id != note.id
                                && other.label() == note.label()
                                && other.date == note.date
                        });
                        let file = duplicate.then(|| {
                            note.path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .into_owned()
                        });
                        // The name speaks words, not the icon's private character.
                        let full = if warned {
                            let short = format!("{} Processed", icons::WARN);
                            Some(detail.replacen(&short, processed_warning().word, 1))
                        } else if clean {
                            let short = format!("{} Processed", icons::OK);
                            Some(detail.replacen(&short, processed_clean().word, 1))
                        } else {
                            None
                        };
                        let row = session_row(
                            ui,
                            self.selected == Some(note.id),
                            &note.label(),
                            file.as_deref(),
                            &detail,
                            full.as_deref(),
                            state,
                        );
                        let row = if warned {
                            row.on_hover_text(processed_warning().word)
                        } else {
                            row
                        };
                        if row.clicked() {
                            choose = Some(note.id);
                        }
                    }
                });
            });
        if let Some(id) = choose {
            self.form = None;
            self.list_only = false;
            self.select(Some(id));
        }
    }

    /// The one editor of a session's note, on the page and in the panel beside any page.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    /// `region`: the pane's editor region, when the pane gives one (the Notes panel, layout
    /// step 11c); the page's note keeps `editor_view` (`ViewHeight::Fills`) and its named jumps.
    fn note_editor(
        &mut self,
        ui: &mut egui::Ui,
        in_panel: bool,
        region: Option<design::pane::EditorRegion<'_>>,
    ) {
        // The cached editor, only while it is this note's.
        let editor_id = self.note_editor_id();
        let Some(draft) = &mut self.draft else {
            let fault = PageFault::Failed(crate::sentence!("This note could not be opened."));
            let _ = PagePlaceholder::<()>::new(&fault).show(ui);
            return;
        };
        // Beside a page with its own draft, Ctrl+S goes to where the keyboard is. The key is
        // taken here, before the field, and the save runs after the field took this frame's
        // input (issue #513): words typed in the same frame are saved with it.
        let save =
            design::save_row::save_key(ui, (!in_panel || self.note_had_focus) && !self.modal);
        if draft.state == DraftState::Conflict {
            if draft.on_disk.is_none() {
                draft.show_disk();
            }
            if let Some(disk) = &draft.on_disk {
                design::grouped_draft_comparison(ui, &draft.body, disk, (&draft.path, disk), false);
                return;
            }
        }
        let focus = std::mem::take(&mut self.focus_notes);
        // A stray click leaves the keyboard on nothing, and the DM's eyes can be on the table.
        // With no field to go to, what they type still belongs in the note.
        if in_panel
            && !egui::Popup::is_any_open(ui.ctx())
            && ui
                .ctx()
                .memory(|memory| memory.focused().is_none() && memory.top_modal_layer().is_none())
        {
            let typed: String = ui.input_mut(|i| {
                let mut typed = String::new();
                i.events.retain(|event| match event {
                    egui::Event::Text(text) | egui::Event::Paste(text) => {
                        typed.push_str(text);
                        false
                    }
                    _ => true,
                });
                typed
            });
            if !typed.is_empty() {
                self.words_for_note = typed;
                self.words_new_line = self.break_before_words;
                self.refocus_note = true;
            }
        }
        // M0465–M0470: the note is an editor viewport of its own (`pane::editor_view`): its
        // fixed frame, its text scroll inside it, and the caret follow and reveal are the part's.
        // In the Notes panel the editor holds the note from its `## What happened` on (the
        // panel's own slice, written back into the note when it changes; lead, choice a): the
        // panel is for notes during play, and its title line names the session.
        let field = if in_panel {
            NOTE_IN_PANEL
        } else {
            NOTE_ON_PAGE
        }
        .of(self.selected);
        if in_panel {
            let now = draft.body.key();
            let session = self.selected;
            match self
                .panel_slice
                .as_mut()
                .filter(|slice| slice.session == session)
            {
                Some(slice) if slice.body == now => {}
                Some(slice) => {
                    // Only the note's own writes since the panel held the note (an act, the
                    // page's insert, a renamed link): the note is cut again at What happened, and
                    // the panel's part takes its new words under the panel's typing history, so
                    // Ctrl+Z there takes back what the DM typed and leaves the app's words
                    // (reviews of PRs 799 and 825).
                    match draft.own_since(slice.body) {
                        true => {
                            let text = draft.body.as_str();
                            let (prefix, tail) = text.split_at(PanelSlice::start(text));
                            if !slice.tail.same_words(tail) {
                                design::write_under_history(
                                    ui.ctx(),
                                    field.id(),
                                    &mut slice.tail,
                                    tail,
                                );
                            }
                            prefix.clone_into(&mut slice.prefix);
                            slice.body = now;
                        }
                        false => {
                            // A change the panel did not make (a reload of an outside edit, a
                            // discard): the slice is cut again and its typing history ends, so
                            // Ctrl+Z there never puts back words from before it.
                            if let Some(mut state) =
                                egui::TextEdit::load_state(ui.ctx(), field.id())
                            {
                                state.clear_undoer();
                                egui::TextEdit::store_state(ui.ctx(), field.id(), state);
                            }
                            *slice = PanelSlice::of(session, &draft.body);
                        }
                    }
                }
                None => self.panel_slice = Some(PanelSlice::of(session, &draft.body)),
            }
            draft.mark_followed();
        }
        // The body before the page editor's own edits in this pass.
        let before_edit = draft.body.key();
        let (prefix_chars, tail_before) = match &self.panel_slice {
            Some(slice) if in_panel => (slice.prefix.chars().count(), Some(slice.tail.key())),
            _ => (0, None),
        };
        // A newly selected note opens at its top: the selection owns the reset (`select`), once
        // per change of note, never on a reload of the same note.
        if !in_panel && std::mem::take(&mut self.note_scroll_to_top) {
            design::pane::jump_editor_view(ui.ctx(), field, design::pane::EditorJump::Top);
        }
        let text: &mut TextDraft = match &mut self.panel_slice {
            Some(slice) if in_panel => &mut slice.tail,
            _ => &mut draft.body,
        };
        let mut edited = false;
        if self.break_before_words
            && let Some(id) = editor_id
            && ui.ctx().memory(|memory| memory.has_focus(id))
        {
            let (typed, correcting) = ui.input(|i| {
                let typed = i.events.iter().any(|e| matches!(e, egui::Event::Text(_)));
                let correcting = i.events.iter().any(|e| {
                    matches!(e, egui::Event::Key { key, pressed: true, .. } if matches!(
                        key,
                        egui::Key::Backspace
                            | egui::Key::Delete
                            | egui::Key::ArrowLeft
                            | egui::Key::ArrowRight
                            | egui::Key::ArrowUp
                            | egui::Key::ArrowDown
                            | egui::Key::Home
                            | egui::Key::End
                    ))
                });
                (typed, correcting)
            });
            // A correction goes on with the same line.
            if typed || correcting {
                self.break_before_words = false;
            }
            if typed && !correcting {
                edited |= insert_at_caret(ui.ctx(), id, text, "", true);
            }
        }
        self.slash.take_keys(ui, editor_id);
        let index = ui.ctx().data(|data| {
            data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                "reader_link_index",
            ))
        });
        let mut pressed_link = None;
        // The viewport's scroll takes its id from its `Ui`: one id per session and place, so the
        // tab and the side-by-side layouts share one scroll (a breakpoint keeps the DM's place).
        ui.scope_builder(
            egui::UiBuilder::new().id(("session_editor_scope", self.selected, in_panel)),
            |ui| {
                // On the page, the note's column sets its width (`note_frame_width`); in the panel,
                // the reader's measure and the frame's edges.
                let width = if in_panel {
                    design::READER_MEASURE + 2.0 * (design::S + design::HAIRLINE_WIDTH)
                } else {
                    note_frame_width(ui.ctx())
                };
                ui.set_max_width(ui.available_width().min(width));
                let font = design::reader::prose_type(ui).font(ui);
                let face = markdown_face(font.clone());
                // Beside a page, Tab indents. It must not walk to `Next turn`, where the next space
                // would press it. Escape still leaves the panel.
                let output = match region {
                    // The Notes panel's pane region: the frame grows with the text up to the
                    // room above the panel's footer, then the text scrolls inside it.
                    Some(region) => region.editor(ui, field, text, face, |edit| {
                        edit.lock_focus(in_panel).font(font)
                    }),
                    None => design::pane::editor_view(
                        ui,
                        design::pane::ViewHeight::Fills,
                        field,
                        text,
                        face,
                        |edit| edit.lock_focus(in_panel).font(font),
                    ),
                };
                crate::markdown_source::paint_section_rules(ui, &output, Some(text.key()));
                pressed_link = crate::markdown_source::pressed_note_link(ui, &output, None);
                let place = super::slash::Place {
                    session: self.selected,
                    campaign: self.campaign.as_ref(),
                    index: index.as_deref(),
                    run_open: self.run_open,
                };
                let done = self.slash.show(ui, &output, text, &place);
                self.wants_content_refresh |= done.created;
                edited |= done.edited;
                let response = &output.response;
                design::named(ui, response, "Note text");
                self.note_had_focus = response.has_focus();
                // Narrowing keeps whichever side the DM last worked in: the note while it has
                // focus, the plan once focus moves to a plan field.
                if !in_panel && response.has_focus() {
                    self.tab = Tab::Note;
                } else if !in_panel
                    && self.tab == Tab::Note
                    && ui.memory(|memory| memory.focused().is_some())
                {
                    self.tab = Tab::Plan;
                }
                self.last_note_editor = Some(NoteEditor {
                    session: self.selected,
                    id: response.id,
                });
                // A click chooses the place, so what is typed goes exactly there.
                if focus || response.clicked() {
                    self.break_before_words = false;
                }
                if std::mem::take(&mut self.refocus_note) {
                    response.request_focus();
                    let words = std::mem::take(&mut self.words_for_note);
                    if !words.is_empty() {
                        // These words had their break, and what follows goes on from them.
                        self.break_before_words = false;
                        let new_line = std::mem::take(&mut self.words_new_line);
                        edited |= insert_at_caret(ui.ctx(), response.id, text, &words, new_line);
                    }
                }
                // A place another act wrote to (the plan's rewards) comes into view, focus or not:
                // the viewport's named jump, in the note's own text (in the panel, past its slice's
                // start).
                if let Some(caret) = self.caret_to_show.take()
                    && let Some(at) = caret.checked_sub(prefix_chars)
                {
                    let rect = output.galley.pos_from_cursor(egui::text::CCursor::new(at));
                    design::pane::jump_editor_view(
                        ui.ctx(),
                        field,
                        design::pane::EditorJump::Show {
                            top: rect.top(),
                            bottom: rect.bottom(),
                        },
                    );
                    ui.ctx().request_repaint();
                }
                // Asked for the note: a fresh line at the end of `What happened`, the caret on it.
                // The part reveals a caret that moved, so the line comes into view.
                if focus {
                    response.request_focus();
                    let (caret, changed) = text.edit(fresh_line_in_note);
                    edited |= changed;
                    let end = egui::text::CCursor::new(caret);
                    let mut state =
                        egui::TextEdit::load_state(ui.ctx(), response.id).unwrap_or_default();
                    state
                        .cursor
                        .set_char_range(Some(egui::text::CCursorRange::one(end)));
                    egui::TextEdit::store_state(ui.ctx(), response.id, state);
                }
                edited |= response.changed();
                #[cfg(any(test, feature = "test-support"))]
                ui.data_mut(|data| {
                    data.insert_temp(egui::Id::new("session_editor_proof"), response.id)
                });
            },
        );
        // The panel's slice changed: the note is the part before it and the slice.
        if let Some(slice) = &mut self.panel_slice
            && in_panel
            && Some(slice.tail.key()) != tail_before
        {
            let _ = draft
                .body
                .replace(format!("{}{}", slice.prefix, slice.tail.as_str()));
            slice.body = draft.body.key();
            draft.mark_followed();
        }
        // The page editor's own edits are the note's own writes: the Notes panel follows them.
        if !in_panel && draft.body.key() != before_edit {
            draft.own_write(before_edit);
        }
        if edited {
            draft.edited();
            if let Some(session) = self.selected {
                self.mentions
                    .insert(session, timeline::referenced_ids(&draft.body));
            }
        }
        if save && let Some(draft) = &mut self.draft {
            draft.save();
        }
        if let Some(target) = pressed_link {
            self.open_note_link(target);
        }
    }

    pub fn fresh_focus_in_panel(&mut self) {
        self.focus_notes = self.notes_open;
    }

    /// A button took the keyboard. It goes back to the panel, or to the page's editor if
    /// the DM was typing there.
    /// The last drawn note editor's id, only while it edits the open note (`selected`). A
    /// cached editor of another note (the DM moved to a note whose editor has not drawn yet)
    /// is none, so this note's edit never moves another note's text guard (review of PR 594).
    fn note_editor_id(&self) -> Option<egui::Id> {
        self.last_note_editor
            .filter(|editor| editor.session == self.selected)
            .map(|editor| editor.id)
    }

    /// Words the page adds to the open note (a reading's summary, a plan's link). It is the
    /// note's own edit, so the note's Undo history stays: its own editor's guard moves with it,
    /// and no other note's (`note_editor_id`). False with no note open.
    pub(super) fn add_to_note(&mut self, ctx: &egui::Context, words: &str) -> bool {
        let editor = self.note_editor_id();
        let Some(draft) = &mut self.draft else {
            return false;
        };
        let before = draft.body.key();
        match editor {
            Some(id) => {
                design::edit_in_place(ctx, id, &mut draft.body, |body| insert_in_note(body, words));
            }
            None => draft.body.edit(|body| insert_in_note(body, words)),
        }
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

    #[allow(
        clippy::disallowed_methods,
        reason = "the Notes panel with no session to write in says so in place of the editor: a typed body message (M20)"
    )]
    pub fn notes_panel(&mut self, ctx: &egui::Context) {
        let width = design::side_panel_width(ctx.content_rect().width());
        design::Column::panel("session_notes", width)
            .page_keeps(design::PANEL_LEAVES_PAGE)
            .frame(
                egui::Frame::new()
                    .fill(colors().page)
                    .stroke(egui::Stroke::new(design::HAIRLINE_WIDTH, colors().rule))
                    .inner_margin(egui::Margin::same(design::M as i8)),
            )
            .show(ctx, |ui| {
                let title = self
                    .selected
                    .and_then(|id| self.notes.iter().find(|note| note.id == id))
                    .map_or_else(|| "Notes".to_owned(), Note::label);
                // M0471: the panel's title line is the shared part: the note's title, its date
                // after it (muted `Small`), and the close icon at the line's right end.
                let date = self
                    .selected
                    .and_then(|id| self.notes.iter().find(|note| note.id == id))
                    .map(|note| note.date.as_str())
                    .filter(|date| !date.is_empty());
                let close = design::TitleLine::new(&title)
                    .role(design::TitleType::Section)
                    .context(date)
                    .close("Close notes")
                    .show(ui)
                    .closed;
                if let Some(draft) = &self.draft { Self::note_status(ui, draft); }
                design::rule(ui);
                if close { self.notes_open = false; return; }
                if self.selected.is_none() {
                    // M0472: in place of the editor, with the panel's one primary.
                    if BodyMessage::from(
                        Notice::new(crate::sentence!("No session yet."))
                            .choice(Choice::new(MessageChoice::Primary("New session"), ())),
                    )
                    .show(ui)
                    .is_some()
                    {
                        self.start_new_session();
                    }
                    return;
                }
                if self.current().is_none() {
                    let _ = BodyMessage::<()>::from(Notice::new(crate::sentence!(
                        "New events go to no session, so no note is open here. Choose a session on the Timeline page."
                    )))
                    .show(ui);
                    return;
                }
                // Round 26: the panel's notice and its draft's result sit in its message footer,
                // so the note the DM types in never moves when one comes or goes. The editor
                // fills its room (lead, P2).
                let frame = PaneFrame::new(ui);
                let messages = self.panel_messages(&frame, ui.available_width());
                let paned = messages.pane_editor(ui, |ui, region| {
                    self.note_editor(ui, true, Some(region));
                });
                if let Some(press) = paned.pressed {
                    self.panel_press(press.id);
                }
            });
    }

    /// The Notes panel's footer: the draft's dropped words with their Undo, and, for a session
    /// dated before today, `Play it tonight` and `New session` (both UIs).
    fn panel_messages<'f>(&mut self, frame: &'f PaneFrame, width: f32) -> Messages<'f, SessionAct> {
        let today = self.today();
        let dated = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
            .filter(|note| note.date.as_str() < today.as_str())
            .map(|note| note.date.clone());
        let mut messages = Messages::new(frame, "notes_panel_footer", width);
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

    fn read_tab(&mut self, ui: &mut egui::Ui, book: &Book, plan: &Plan<'_>, beside: bool) {
        if self.draft.is_none() {
            return;
        }
        let index = reader_index(ui);
        let model = self.plan_model(index.as_deref(), plan);
        let Some(prepared) = self.prepared_read(index.as_deref(), &model, plan, beside, book)
        else {
            return;
        };
        // Beside the plan, the reader is a column with its own scroll; alone, it is the pane's
        // content, and the pane scrolls it (review of PR 335: a read view is content).
        if beside {
            design::ScrollColumn::new(("session_reader", self.selected))
                .show(ui, |ui| self.read_body(ui, &prepared));
        } else {
            self.read_body(ui, &prepared);
        }
        if let Some(target) = markdown_view::take_note_link(ui.ctx()) {
            self.open_note_link(target);
        }
    }

    /// The words Read shows, as `read_tab` prepared them.
    fn read_body(&mut self, ui: &mut egui::Ui, prepared: &carry::Prepared) {
        ui.scope(|ui| {
            let measure = note_frame_width(ui.ctx()) - NOTE_FRAME_EDGES;
            ui.set_max_width(ui.available_width().min(measure));
            if !prepared.written {
                ui.horizontal(|ui| {
                    ui.label(
                        Type::Small
                            .rich(ui, "Nothing written yet.")
                            .color(colors().muted),
                    );
                    if design::link(ui, "Write").clicked() {
                        self.write_in_note();
                        self.focus_notes = true;
                    }
                });
            } else {
                self.show_prepared(ui, prepared, false);
                if let Some(summary) = &prepared.summary {
                    ui.label(Type::Small.rich(ui, summary).color(colors().muted));
                    self.show_prepared(ui, prepared, true);
                }
            }
            self.world_changes(ui);
        });
    }

    /// What play changed in the world in this session, from each entry's Log. The entry holds
    /// the line, so this list needs no store of its own.
    fn world_changes(&mut self, ui: &mut egui::Ui) {
        let Some(index) = ui.ctx().data(|data| {
            data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                "reader_link_index",
            ))
        }) else {
            return;
        };
        let Some(note) = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
        else {
            return;
        };
        let changes = index.world_changes(&note.path);
        if changes.is_empty() {
            return;
        }
        let session = note
            .path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        design::heading(ui, design::Heading::Section("World changes"));
        let mut open = None;
        for (source, line) in changes {
            let words = line.words();
            // Every line here is from this session, so its name is not said again.
            let words = words.strip_prefix(&session).map_or(words.as_str(), |rest| {
                rest.trim_start_matches([',', ':', ' '])
            });
            ui.horizontal_wrapped(|ui| {
                if design::link(ui, &index.items[source].entry.name).clicked() {
                    open = Some(source);
                }
                ui.label(Type::Small.rich(ui, words).color(colors().muted));
            });
        }
        if let Some(source) = open {
            let link = index.link_to(source);
            self.wants_lookup = Some(link.trim_matches(['[', ']']).to_owned());
        }
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

    /// The note, as one view: words to write, or the same words to read.
    fn note_pane(&mut self, ui: &mut egui::Ui, book: &Book, plan: &Plan<'_>, beside: bool) {
        if self.reading_note {
            self.read_tab(ui, book, plan, beside);
        } else {
            self.note_editor(ui, false, None);
        }
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

    /// What the session before this one left: where the DM picks up the thread.
    fn last_time_block(&mut self, ui: &mut egui::Ui, book: &Book, plan: &Plan<'_>) {
        let Some(selected) = self.see_last_time(book, plan) else {
            return;
        };
        self.last_time_line(ui, selected, plan, book);
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

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn recording_row(
        &mut self,
        ui: &mut egui::Ui,
        recording: &Recording,
        settings: &ProcessingSettings,
        book: &Book,
        roles: RowRoles,
    ) {
        let RowRoles {
            primary,
            destination_here,
            record_again_here,
        } = roles;
        let c = colors();
        let live = self
            .live
            .as_ref()
            .is_some_and(|live| live.recorder.path() == recording.path);
        let length = recording.seconds.map_or_else(
            || {
                if recording.path.exists() {
                    "Unknown length"
                } else {
                    "Audio removed"
                }
                .to_owned()
            },
            spoken_length,
        );
        let mut delete = false;
        let mut again = false;
        // Round 27 FO1: the `⋯` follows its date-time title an `L` gap after it, on the title's
        // line, so it belongs to this row and never floats at the pane's edge. Round 28 FB8: the
        // title's column is as tall as the `⋯`, so the title's line and the `⋯` share one centre.
        ui.horizontal(|ui| {
            let menu = Density::Read.control() + design::L;
            ui.allocate_ui_with_layout(
                egui::vec2(
                    (ui.available_width() - menu).max(0.0),
                    Density::Read.control(),
                ),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.add(
                        egui::Label::new(Type::Strong.rich(ui, &recording.title).color(c.text))
                            .wrap(),
                    );
                },
            );
            design::gap_to_next(ui, design::Gap::L);
            {
                design::row_menu(ui, &recording.title, |ui| {
                    if recording.summary
                        && Action::quiet("Process again…")
                            .name(&format!("Process again: {}", recording.name))
                            .blocked(if !recording.path.exists() {
                                Some("Audio removed. Restore the WAV file to process it.")
                            } else if live {
                                Some("Stop recording first.")
                            } else if self.job.is_some() {
                                Some("Wait for processing to finish.")
                            } else {
                                None
                            })
                            .show(ui)
                            .clicked()
                    {
                        again = true;
                        ui.close();
                    }
                    let busy = live
                        || self
                            .job
                            .as_ref()
                            .is_some_and(|(job, _)| job.recording == recording.path);
                    delete = Action::quiet("Delete…")
                        .name(&format!("Delete recording: {}", recording.name))
                        .density(Density::Read)
                        .blocked_if(busy, "This recording is in use.")
                        .show(ui)
                        .clicked();
                    // A note under its choice, at its words, as every menu's (M-P0.27).
                    design::choice_note(ui, false, busy.then_some("This recording is in use."));
                    if delete {
                        ui.close();
                    }
                });
            }
        });
        let processing = self
            .job
            .as_ref()
            .is_some_and(|(job, _)| job.recording == recording.path);
        ui.label(
            Type::Small
                .rich(
                    ui,
                    if live {
                        megabytes(recording.bytes)
                    } else if !recording.path.exists() {
                        "Audio removed".to_owned()
                    } else {
                        format!("{length} · {}", megabytes(recording.bytes))
                    },
                )
                .color(c.muted),
        );
        let stored = &recording.status;
        let missing_input = settings.input_missing()
            || stored
                .as_ref()
                .is_some_and(|words| words.to_lowercase().contains(MISSING_INPUT_ERROR));
        let one = |icon, word, color| RecordingStatus::one(Status { icon, word, color });
        let status = if live {
            self.recording_status()
                .unwrap_or_else(RecordingStatus::live)
        } else if processing {
            one(icons::BUSY, "Processing", c.connection.busy)
        } else if let Some(words) = stored.as_deref() {
            let state = if words.starts_with("Failed") {
                "Failed"
            } else if words.starts_with("Processing cancelled") {
                "Cancelled"
            } else {
                "Stopped"
            };
            if state == "Cancelled" {
                one(icons::WARN, state, c.result.warning)
            } else {
                one(icons::ERROR, state, c.result.error)
            }
        } else if recording.warning.is_some() {
            RecordingStatus::one(processed_warning())
        } else if recording.summary {
            RecordingStatus::one(processed_clean())
        } else if recording.transcript {
            one(icons::DRAFT, "Partly processed", c.muted)
        } else {
            one(icons::DOCUMENT, "Not processed", c.muted)
        };
        if processing {
            // Round 28 FM10: the spinner line takes the status line's place, so every state keeps
            // one order: size, status, detail, result links, actions.
            self.processing_lines(ui);
        } else {
            design::status(ui, status.tinted(status.word)).on_hover_text(status.word);
        }
        let stalled = live
            && self.live.as_ref().is_some_and(|live| {
                matches!(live.recorder.health(), Health::Stalled | Health::Stopped(_))
            });
        let silent = live
            && self
                .live
                .as_ref()
                .is_some_and(|live| *live.recorder.health() == Health::Silent);
        if silent && let Some(live) = &self.live {
            ui.label(
                Type::Small
                    .rich(
                        ui,
                        // The time keeps the line's start in every live state (FM8).
                        format!(
                            "{} recorded · Silent for over a minute",
                            clock(live.recorder.seconds())
                        ),
                    )
                    .color(c.muted),
            );
        }
        if live
            && !silent
            && !stalled
            && let Some(live) = &self.live
        {
            let sound = if *live.recorder.health() == Health::Starting {
                "Waiting for sound"
            } else if live.recorder.heard_just_now() {
                "Hearing sound"
            } else {
                "Quiet"
            };
            // R5: the live time is the state's, in its word's colour; the sound is detail.
            let mut job = egui::text::LayoutJob::default();
            for (part, color) in [
                (clock(live.recorder.seconds()), status.word_color()),
                (format!(" recorded · {sound}"), c.muted),
            ] {
                Type::Small.rich(ui, part).color(color).append_to(
                    &mut job,
                    ui.style(),
                    egui::FontSelection::Default,
                    egui::Align::Center,
                );
            }
            ui.label(job);
        }
        if stalled && let Some(live) = &self.live {
            let words = format!(
                "Stopped saving after {}. What was saved is kept. Record again starts a new file.",
                spoken_length(live.recorder.seconds())
            );
            row_sentence(ui, Type::Small.rich(ui, words).color(c.muted));
        }
        if let Some(words) = stored.as_ref().filter(|_| !live && !processing) {
            let shown = row_sentence(
                ui,
                Type::Small
                    .rich(ui, recording_detail(words, missing_input))
                    .color(c.muted),
            );
            if let Some(said) = words
                .strip_prefix("Stopped by itself:")
                .filter(|said| plain_recorder_words(said).is_some())
            {
                shown.on_hover_text(recorder_said(said));
            }
        }
        let warning_shown = !live && !processing;
        if let Some(warning) = recording.warning.as_ref().filter(|_| warning_shown) {
            row_sentence(ui, Type::Small.rich(ui, warning).color(c.muted));
        }
        let job_here = self
            .job
            .as_ref()
            .is_some_and(|(job, _)| job.recording == recording.path);
        let mut open = None;
        let views: Vec<Made> = [
            (Made::Summary, recording.summary),
            (Made::Marked, recording.marked),
            (Made::Review, recording.review && !recording.check_missing()),
            (Made::Transcript, recording.transcript),
        ]
        .into_iter()
        .filter_map(|(made, exists)| exists.then_some(made))
        .collect();
        if !views.is_empty() {
            ui.horizontal_wrapped(|ui| {
                for made in views {
                    if design::link(ui, made.word())
                        .on_hover_text(format!("Read this for {}", recording.name))
                        .clicked()
                    {
                        open = Some(made);
                    }
                    // While processing hides the warning line, the old results still need
                    // their reason, in the same words. Never both at once.
                    if made == Made::Transcript && !warning_shown && recording.check_missing() {
                        ui.label(
                            Type::Small
                                .rich(ui, processing::NO_CHECK_LABEL)
                                .color(c.muted),
                        );
                    }
                }
            });
        }
        let mut process = false;
        let mut stop = false;
        let mut record_again = false;
        // An empty row would still take a line's height, and push the marked events away
        // from this recording's links. Draw the row only when it holds something.
        let has_actions = stalled
            || live
            || (!job_here && !recording.summary)
            || stored
                .as_ref()
                .is_some_and(|words| words.starts_with("Stopped by itself:"));
        let mut actions = |ui: &mut egui::Ui| {
            if !live
                && !job_here
                && stored
                    .as_ref()
                    .is_some_and(|words| words.starts_with("Stopped by itself:"))
            {
                // The recovery of the newest stopped recording, with `Record again`.
                if missing_input
                    && record_again_here
                    && (if primary && !self.run_open {
                        Action::primary("Choose an input")
                    } else {
                        Action::secondary("Choose an input")
                    })
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.wants_recording_input = true;
                }
                record_again = record_again_here
                    && (if primary
                        && !self.run_open
                        && !missing_input
                        && self.live.is_none()
                        && self.job.is_none()
                    {
                        Action::primary("Record again")
                    } else {
                        Action::secondary("Record again")
                    })
                    .density(Density::Read)
                    .blocked_if(
                        self.live.is_some() || self.job.is_some(),
                        "Stop recording or wait for processing to finish.",
                    )
                    .show(ui)
                    .clicked();
            }
            // Stop recording keeps the first slot for as long as the recording is live. A stall
            // or silence comes by itself, so its actions go after Stop, never before it (FM2).
            if live {
                stop = Action::secondary("Stop recording")
                    .density(Density::Read)
                    .show(ui)
                    .clicked();
                if stalled && record_again_here {
                    record_again = (if self.run_open || !primary {
                        Action::secondary("Record again")
                    } else {
                        Action::primary("Record again")
                    })
                    .show(ui)
                    .clicked();
                }
                if silent
                    && Action::quiet("Check the input in Settings")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                {
                    self.wants_processing_settings = true;
                }
            }
            let reason = if !recording.path.exists() {
                Some("Audio removed. Restore the WAV file to process it.".to_owned())
            } else if live {
                Some("Stop recording first.".to_owned())
            } else if self.job.is_some() {
                Some("One recording is processed at a time.".to_owned())
            } else {
                settings.problem()
            };
            if !job_here && !recording.summary {
                let label = if recording.summary {
                    "Process again"
                } else if recording.transcript
                    || recording.status.as_ref().is_some_and(|words| {
                        words.starts_with("Failed") || words.starts_with("Processing cancelled")
                    })
                {
                    "Continue processing"
                } else {
                    "Process"
                };
                let destination = recording_destination(settings, recording);
                let response = (if self.run_open
                    || !primary
                    || stalled
                    || stored
                        .as_ref()
                        .is_some_and(|words| words.starts_with("Stopped by itself:"))
                    || reason.is_some()
                {
                    Action::secondary(label)
                } else {
                    Action::primary(label)
                })
                .name(&format!("{label}: {}", recording.name))
                .density(Density::Read)
                .blocked(reason.as_deref())
                .show(ui);
                ui.ctx().accesskit_node_builder(response.id, |node| {
                    node.set_description(reason.as_deref().unwrap_or(&destination))
                });
                let pressed = response.on_hover_text(&destination).clicked();
                if live {
                    design::blocked_reason(ui, Some("Stop recording first."));
                }
                if let Some(reason) = reason.as_ref().filter(|_| !live && self.job.is_none()) {
                    let speech = processing::host(&settings.config.speech_url);
                    let named = if reason.starts_with("Save an API key for this speech service") {
                        format!(
                            "Save an {} key in Settings",
                            if speech == "api.openai.com" {
                                "OpenAI"
                            } else {
                                &speech
                            }
                        )
                    } else {
                        reason.clone()
                    };
                    if design::link(ui, &named).clicked() {
                        self.wants_processing_settings = true;
                        self.wants_speech_key =
                            reason.starts_with("Save an API key for this speech service");
                    }
                }
                process = pressed && !recording.summary;
                design::gap(ui, design::Gap::L);
            }
        };
        if has_actions {
            ui.horizontal_wrapped(|ui| actions(ui));
        }
        if destination_here {
            design::help(ui, &recording_destination(settings, recording));
        }
        if self.confirm_again.as_deref() == Some(recording.path.as_path()) {
            // Processing help keeps the reader's measure, as the destination line above does.
            // Not now and Replace the results stay together below the message.
            ui.scope(|ui| {
                ui.set_max_width(ui.available_width().min(design::READER_MEASURE));
                let words = format!(
                    "This sends the raw transcript, this session’s name, notes and events to {} twice. The audio is not sent again. It replaces the summary, the clean transcript and what the check changed. Events you added stay on the timeline.",
                    processing_llm(settings)
                );
                // The choices start at the message text; the red one keeps an `L` gap (the part's).
                let pressed = BodyMessage::from(
                    Notice::new(notice_sentence(&words))
                        .choice(Choice::new(MessageChoice::Choice("Not now"), false))
                        .choice(Choice::new(MessageChoice::Lose("Replace the results"), true)),
                )
                .show(ui);
                match pressed {
                    Some(false) => self.confirm_again = None,
                    Some(true) => process = true,
                    None => {}
                }
            });
        }
        if stop {
            self.finish_recording(ui.ctx());
        }
        if record_again && let Some(session) = self.selected {
            if stalled {
                self.finish_recording(ui.ctx());
            }
            if let Err(error) = self.start_recording(settings, session) {
                self.say(Tone::Error, error, None);
            }
        }
        if again {
            self.confirm_again = Some(recording.path.clone());
        }
        if process {
            self.confirm_again = None;
            let path = recording.path.clone();
            self.start_processing(ui.ctx(), &path, settings, book);
        }
        if delete {
            self.confirm_delete = Some(recording.path.clone());
        }
        if let Some(made) = open {
            self.open_reading(recording, made, None, None);
        }
    }

    /// The status line of the recording being processed: the spinner and the progress words, and
    /// under the words quiet `Cancel` on its own line (round 27 AN1), so a stage's longer words
    /// never move it under the pointer. Its words start at the progress words' left edge (round
    /// 28 FM7): the spinner keeps its own column, and `Cancel` hangs its padding into it.
    fn processing_lines(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        let Some((job, _)) = &self.job else {
            return;
        };
        let words = job.progress.words();
        let cancelling = job.cancelling();
        let mut cancel = false;
        ui.horizontal_top(|ui| {
            design::spinner(ui);
            ui.vertical(|ui| {
                ui.label(
                    Type::Small
                        .rich(ui, format!("Processing · {words}"))
                        .color(c.muted),
                );
                if cancelling {
                    ui.label(
                        Type::Small
                            .rich(ui, "Cancelling. This ends after the current step.")
                            .color(c.muted),
                    );
                } else {
                    cancel = Action::quiet("Cancel")
                        .name("Cancel processing")
                        .density(Density::Read)
                        .line_start()
                        .show(ui)
                        .clicked();
                }
            });
        });
        if cancel && let Some((job, _)) = &self.job {
            job.cancel();
        }
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
    fn open_reading(
        &mut self,
        recording: &Recording,
        made: Made,
        at: Option<&str>,
        marker: Option<(crate::timeline::Marker, String)>,
    ) {
        let mark = marker
            .as_ref()
            .map(|(marker, _)| MarkJump::new(marker.reading_words()));
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
            mark,
            marker,
            short_of: matches!(made, Made::Transcript)
                .then(|| processing::transcript_short_of(&recording.path))
                .flatten(),
            shown: None,
        });
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn markers(&mut self, ui: &mut egui::Ui, recording: &Recording, book: &mut Book) {
        if recording.markers.is_empty() {
            return;
        }
        let c = colors();
        let session = self.selected;
        // The heading adds the `L` gap between the recording's last line and itself.
        design::heading(
            ui,
            design::Heading::Group {
                title: "Events marked in this recording",
                count: recording.markers.len(),
            },
        );
        ui.label(
            Type::Small
                .rich(
                    ui,
                    "These events were suggested from the transcript. Only events you add go on the timeline.",
                )
                .color(c.muted),
        );
        let mut read_at: Option<(String, crate::timeline::Marker, String)> = None;
        let days = timeline::day_numbers(&book.events);
        let rows: Vec<_> = recording
            .markers
            .iter()
            .map(|marker| {
                let title = if marker.title.is_empty() {
                    marker.kind.word()
                } else {
                    marker.title.as_str()
                };
                let key = marker.key(&recording.name);
                let added = book.event_of_marker(&key).map(|event| event.id.clone());
                let mut facts = vec![marker.kind.word()];
                for fact in [&marker.duration, &marker.place] {
                    if !fact.is_empty() {
                        facts.push(fact);
                    }
                }
                let facts = facts.join(" · ");
                let mut placement = String::new();
                if let Some(id) = &added {
                    let at = book.events.iter().position(|event| event.id == *id);
                    let before = at.and_then(|at| at.checked_sub(1));
                    placement = before.map_or_else(
                        || "First on the timeline".to_owned(),
                        |before| format!("After {}", book.events[before].label(days[before])),
                    );
                }
                (marker, title, key, added, facts, placement)
            })
            .collect();
        let measure = |text: &str, role: Type| {
            ui.painter()
                .layout_no_wrap(text.to_owned(), role.font(ui), c.text)
                .size()
                .x
        };
        let action = measure(&format!("{} On the timeline", icons::OK), Type::Small)
            .max(Action::secondary("Add").natural_width(ui));
        let text = rows
            .iter()
            .map(|(marker, title, _, _, facts, _)| {
                let time = if recording.marked && !marker.at.is_empty() {
                    measure(&marker.at, Type::Body) + design::S
                } else {
                    0.0
                };
                measure(kind_icon(marker.kind), Type::Body)
                    + design::S
                    + measure(title, Type::Body).max(time + measure(facts, Type::Small))
            })
            .fold(0.0, f32::max)
            .min(design::READER_MEASURE)
            .min((ui.available_width() - action - design::L).max(0.0));
        let widths = [text, action];
        // The rows and their rules take the combined width. A row's Add result is in the
        // session pane's footer (round 27 FI3).
        let row_count = rows.len();
        ui.scope(|ui| {
            for (index, (marker, title, key, added, facts, placement)) in rows.iter().enumerate() {
                let row = ui.scope(|ui| {
                    ui.set_width(text + design::L + action);
                    design::table_row(ui, &widths, design::L, |ui, cell| {
                        ui.spacing_mut().item_spacing.x = design::S;
                        if cell == 0 {
                            ui.horizontal_top(|ui| {
                                ui.label(
                                    Type::Body.rich(ui, kind_icon(marker.kind)).color(c.muted),
                                );
                                ui.vertical(|ui| {
                                    ui.add(
                                        egui::Label::new(Type::Body.rich(ui, *title).color(c.text))
                                            .wrap(),
                                    );
                                    ui.horizontal_wrapped(|ui| {
                                        if !marker.at.is_empty()
                                            && recording.marked
                                            && design::link(ui, &marker.at)
                                                .on_hover_text("Read the transcript here")
                                                .clicked()
                                        {
                                            read_at = Some((
                                                marker.at.clone(),
                                                (*marker).clone(),
                                                key.clone(),
                                            ));
                                        }
                                        ui.add(
                                            egui::Label::new(
                                                Type::Small.rich(ui, facts).color(c.muted),
                                            )
                                            .wrap(),
                                        );
                                    });
                                    // Review of PR 296, item 2: the placement line is kept
                                    // on every row with a row below it, empty until the marker
                                    // is added, so an Add moves no row below it. The last row
                                    // keeps none (review of PR 335): nothing below it moves,
                                    // and the footer follows the list `S` below it.
                                    if placement.is_empty() && index + 1 < row_count {
                                        // As tall as the label it stands for.
                                        design::empty_line(ui, Type::Small);
                                    } else {
                                        ui.add(
                                            egui::Label::new(
                                                Type::Small.rich(ui, placement).color(c.muted),
                                            )
                                            .wrap(),
                                        );
                                    }
                                });
                            });
                        } else {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                                if let Some(id) = &added {
                                    if design::status_link(
                                        ui,
                                        Status {
                                            icon: icons::OK,
                                            word: "On the timeline",
                                            color: c.result.success,
                                        },
                                    )
                                    .clicked()
                                    {
                                        self.wants_event = Some(id.clone());
                                    }
                                } else if Action::secondary("Add")
                                    .name(&format!("Add to timeline: {title} ({index})"))
                                    .density(Density::Read)
                                    .blocked_if(
                                        book.unreadable.is_some(),
                                        "The timeline file must be repaired first.",
                                    )
                                    .show(ui)
                                    .clicked()
                                {
                                    let id = book.add_marker(marker, key, session);
                                    book.save();
                                    self.markers.record(kept::MarkerAdd {
                                        owner: kept::OfSession {
                                            root: self.folder.clone().unwrap_or_default(),
                                            session,
                                        },
                                        id,
                                        title: (*title).to_owned(),
                                    });
                                }
                            });
                        }
                    });
                });
                design::register_row(ui, row.response.rect);
            }
        });
        if let Some((at, marker, key)) = read_at {
            self.open_reading(recording, Made::Marked, Some(&at), Some((marker, key)));
        }
    }

    /// One text from a recording, in the whole pane, one part at a time. A 5-hour transcript
    /// is about 30 parts of a size the Reader handles.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state`, and raw egui calls (design::results::STAGED_ALLOWS)"
    )]
    fn reading_view(&mut self, ui: &mut egui::Ui, book: &mut Book) {
        let c = colors();
        let session = self.selected;
        let Some(reading) = &self.reading else {
            return;
        };
        let path = reading.recording.clone();
        let mut made = reading.made;
        let options: Vec<_> = [Made::Summary, Made::Marked, Made::Review, Made::Transcript]
            .into_iter()
            .filter(|made| {
                made.path(&path).exists()
                    && (*made != Made::Review
                        || self
                            .recordings
                            .iter()
                            .find(|recording| recording.path == path)
                            .is_none_or(|recording| !recording.check_missing()))
            })
            .map(|made| (made, made.word()))
            .collect();
        let mut back = false;
        let mut add_to_notes = None;
        let in_notes = self.draft.as_ref().map_or("", |draft| draft.body.as_str());
        // Reserve each segment's full width before choosing the control. Its row never wraps.
        let segments_width: f32 =
            options
                .iter()
                .map(|(_, label)| {
                    design::number_width(ui, label, &Type::Strong.font(ui))
                        .max(design::number_width(ui, label, &Type::Body.font(ui)))
                        + 2.0 * design::S
                })
                .sum();
        // The viewer is one block: the reader measure, or the view choices when they are wider.
        // Add to notes ends at its right edge, near the date, not at the far edge of the pane.
        let block = ui
            .available_width()
            .min(design::READER_MEASURE.max(segments_width + design::XS));
        // The back link's word lines up with the view choices below it: their words, or the
        // drop-down's edge (the same choice as below).
        let back_edge = if segments_width + design::XS <= ui.available_width() {
            design::BackEdge::Segments
        } else {
            design::BackEdge::Text
        };
        ui.horizontal(|ui| {
            ui.set_max_width(block);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if reading.made == Made::Summary {
                    let summary = nested_in_note(reading.parts[0].1.trim());
                    if in_notes.contains(summary.trim()) {
                        design::status(
                            ui,
                            Status {
                                icon: icons::OK,
                                word: "In notes",
                                color: c.document.saved,
                            },
                        );
                    } else if Action::secondary("Add to notes")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                    {
                        add_to_notes = Some(summary);
                    }
                }
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    back = design::TitleLine::back(crate::word!("Back to recordings"), back_edge)
                        .context(&reading.name)
                        .show(ui)
                        .clicked();
                });
            });
        });
        let switched = if segments_width + design::XS <= ui.available_width() {
            design::segmented(ui, &mut made, &options)
        } else {
            // The reader belongs to the selected session's page; one is open at a time.
            let mut views = options.iter().copied();
            if let Some((first, label)) = views.next() {
                let _ = design::Dropdown::new(RECORDING_VIEW.of(self.selected), "Recording view")
                    .choice(first, label)
                    .choices(views)
                    .show(ui, &mut made);
            }
            made != reading.made
        };
        if switched
            && let Some(recording) = self
                .recordings
                .iter()
                .find(|recording| recording.path == path)
                .cloned()
        {
            self.open_reading(&recording, made, None, None);
        }
        // A marker's Add result names the session that gets the event.
        let selected = self.selected;
        let Some(reading) = &mut self.reading else {
            return;
        };
        if let Some((done, parts)) = reading.short_of {
            let _ = BodyMessage::<()>::from(Notice::warning(crate::sentence!(
                "Transcribed so far: {} of {} parts. Continue processing adds the rest.",
                done,
                parts
            )))
            .show(ui);
        }
        if matches!(reading.made, Made::Summary | Made::Marked | Made::Review) {
            ui.label(
                Type::Small
                    .rich(
                        ui,
                        "This draft was written automatically. Check it against your memory.",
                    )
                    .color(c.muted),
            );
        }
        let count = reading.parts.len();
        if count > 1 {
            ui.horizontal(|ui| {
                if Action::quiet("Earlier")
                    .icon(icons::UP)
                    .name("Earlier part")
                    .density(Density::Read)
                    .line_start()
                    .blocked_if(reading.part == 0, "This is the first part.")
                    .show(ui)
                    .clicked()
                {
                    reading.part -= 1;
                }
                ui.label(
                    Type::Small
                        .rich(ui, format!("Part {} of {count}", reading.part + 1))
                        .color(c.muted),
                );
                let mut times = reading
                    .parts
                    .iter()
                    .enumerate()
                    .map(|(at, (time, _))| (at, time.as_str()));
                if let Some((first, label)) = times.next() {
                    let _ = design::Dropdown::new(RECORDING_PART.of(selected), "Go to a time")
                        .choice(first, label)
                        .choices(times)
                        .width(120.0)
                        .show(ui, &mut reading.part);
                }
                if Action::quiet("Later")
                    .icon(icons::DOWN)
                    .name("Later part")
                    .density(Density::Read)
                    .blocked_if(reading.part + 1 == count, "This is the last part.")
                    .show(ui)
                    .clicked()
                {
                    reading.part += 1;
                }
            });
        }
        design::gap(ui, design::Gap::XS);
        if reading
            .shown
            .as_ref()
            .is_none_or(|shown| shown.part != reading.part)
        {
            reading.shown = Some(ShownPart::of(reading));
        }
        let mut landed = false;
        // The text is the session page's pane content: the pane is its scroll column.
        ui.scope(|ui| {
            note_width(ui, |ui| {
                let Some(shown) = &reading.shown else {
                    return;
                };
                let body = shown.body.as_str();
                match shown.at {
                    Some(at) => {
                        markdown_view::show_reader(ui, &body[..at]);
                        // Opened at a timeline marker: its line at the view's top, by the
                        // column's typed jump (M0481, ruling M18), measured again in each draw
                        // until it lands (`MarkJump`).
                        if let Some(mark) = &mut reading.mark {
                            let top = ui.cursor().top();
                            let below = top - ui.clip_rect().top();
                            if mark.ask(below, design::reveal::dm_scrolls(ui)) {
                                design::reveal::jump(
                                    ui,
                                    top,
                                    design::reveal::JumpTo::LinkedHeading,
                                );
                            } else {
                                landed = true;
                            }
                        }
                        if let Some((marker, key)) = &reading.marker {
                            let line_end = body[at..].find('\n').map_or(body.len(), |end| at + end);
                            ui.horizontal_wrapped(|ui| {
                                markdown_view::show_reader(ui, &body[at..line_end]);
                                if let Some(event) = book.event_of_marker(key) {
                                    if design::status_link(
                                        ui,
                                        Status {
                                            icon: icons::OK,
                                            word: "On the timeline",
                                            color: c.result.success,
                                        },
                                    )
                                    .clicked()
                                    {
                                        self.wants_event = Some(event.id.clone());
                                    }
                                } else if Action::secondary("Add")
                                    .name("Add this event to the timeline")
                                    .blocked_if(
                                        book.unreadable.is_some(),
                                        "The timeline file must be repaired first.",
                                    )
                                    .show(ui)
                                    .clicked()
                                {
                                    let id = book.add_marker(marker, key, session);
                                    book.save();
                                    let title = if marker.title.is_empty() {
                                        marker.kind.word()
                                    } else {
                                        marker.title.as_str()
                                    };
                                    self.markers.record(kept::MarkerAdd {
                                        owner: kept::OfSession {
                                            root: self.folder.clone().unwrap_or_default(),
                                            session,
                                        },
                                        id,
                                        title: title.to_owned(),
                                    });
                                }
                            });
                            markdown_view::show_reader(ui, &body[line_end..]);
                        } else {
                            markdown_view::show_reader(ui, &body[at..]);
                        }
                    }
                    None => markdown_view::show_reader(ui, body),
                }
            });
            // Landed, ended, or nothing to land on in this part.
            if landed
                || reading
                    .shown
                    .as_ref()
                    .is_some_and(|shown| shown.at.is_none())
            {
                reading.mark = None;
            }
            // A part fills several screens, so the way on is also where the reading ends. With
            // no part after it, nothing follows the words: the footer comes `S` below them.
            if reading.part + 1 < count {
                design::gap(ui, design::Gap::S);
                if Action::secondary("Go on to the next part")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    reading.part += 1;
                }
            }
        });
        if back {
            self.reading = None;
        }
        if let Some(body) = add_to_notes
            && self.add_to_note(ui.ctx(), &body)
        {
            self.say(Tone::Success, "Added to the notes.".to_owned(), None);
        }
    }

    fn recordings_tab(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
    ) {
        if self.reading.is_some() {
            self.reading_view(ui, book);
            return;
        }
        let banner_primary = self.banner_has_primary(settings, self.run_open);
        let recordings = self.recordings.clone();
        if recordings.is_empty() {
            design::gap(ui, design::Gap::M);
            let here = self
                .live
                .as_ref()
                .is_some_and(|live| Some(live.session) == self.selected);
            let reason = self.record_blocked(settings);
            let elsewhere = self.live.as_ref().map(|live| live.label.clone());
            let words = if here {
                "The recorder is starting. Its file appears here in a moment.".to_owned()
            } else if let Some(label) = elsewhere {
                format!("No recordings of this session. “{label}” is recording now.")
            } else if let Some(reason) = &reason {
                format!("No recordings of this session. {reason}")
            } else {
                "No recordings of this session. Select Record above to make one.".to_owned()
            };
            design::empty_state(ui, icons::RECORD, "Session recordings", &words, |_| {});
            if !here
                && self.live.is_none()
                && reason.is_some()
                && Action::quiet("Open Settings").show(ui).clicked()
            {
                self.wants_processing_settings = true;
            }
            return;
        }
        // The rows leave the scroll bar's lane free whether or not the bar shows, so a row's menu
        // keeps its x when the list grows long enough to scroll.
        let lane_width = ui.available_width() - ui.spacing().scroll.allocated_width();
        // The rows are the session page's pane content: the pane is their scroll column.
        ui.scope(|ui| {
            ui.set_max_width(
                lane_width
                    .min(ui.available_width())
                    .min(design::LIST_MEASURE),
            );
            let recovery = self.recording_recovery_row();
            let destination = recordings.iter().position(|recording| {
                !recording.summary
                    && recording.path.exists()
                    && self.job.is_none()
                    && settings.problem().is_none()
                    && self
                        .live
                        .as_ref()
                        .is_none_or(|live| live.recorder.path() != recording.path)
            });
            let primary = recovery.or(destination);
            let record_again = self.record_again_row();
            for (index, recording) in recordings.iter().enumerate() {
                design::gap(ui, design::Gap::S);
                let row = ui.scope_builder(
                    egui::UiBuilder::new().id(("recording_row", &recording.path)),
                    |ui| {
                        self.recording_row(
                            ui,
                            recording,
                            settings,
                            book,
                            RowRoles {
                                primary: primary == Some(index)
                                    && !self.confirm_record
                                    && !banner_primary,
                                destination_here: destination == Some(index),
                                record_again_here: record_again == Some(index),
                            },
                        );
                    },
                );
                design::register_row(ui, row.response.rect);
                self.markers(ui, recording, book);
                // Each marker row ends in a rule already.
                if recording.markers.is_empty() {
                    design::rule(ui);
                }
            }
        });
    }

    fn delete_dialog(&mut self, ctx: &egui::Context) {
        let focus_id = egui::Id::new("delete_recording_default_focused");
        let Some(path) = self.confirm_delete.clone() else {
            ctx.data_mut(|data| data.remove::<bool>(focus_id));
            return;
        };
        let mut keep = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let mut delete = false;
        design::dialog(ctx, "delete_recording", "Delete this recording?", |ui| {
            ui.add(
                egui::Label::new(format!(
                    "This deletes the recording of {}, its transcripts and its summary. You cannot undo this. The timeline stays as it is.",
                    recording_title(&path)
                ))
                .wrap(),
            );
            design::gap(ui, design::Gap::S);
            design::dialog_actions(
                ui,
                |ui| {
                    delete = Action::destructive("Delete recording").show(ui).clicked();
                },
                |ui| {
                    let response = Action::secondary("Keep the recording").show(ui);
                    if !ui.data(|data| data.get_temp::<bool>(focus_id).unwrap_or(false)) {
                        response.request_focus();
                        ui.data_mut(|data| data.insert_temp(focus_id, true));
                    }
                    keep |= response.clicked();
                },
            );
        });
        if keep {
            self.confirm_delete = None;
            ctx.data_mut(|data| data.remove::<bool>(focus_id));
        }
        if delete {
            self.confirm_delete = None;
            ctx.data_mut(|data| data.remove::<bool>(focus_id));
            self.reading = None;
            // A deleted recording takes its result with it, so the rail stops saying Processed.
            if self.processed_recording.as_deref() == Some(path.as_path()) {
                self.processed_session = None;
                self.processed_recording = None;
                self.job_done = false;
                self.job_warned = false;
            }
            let (tone, words) = match delete_recording(&path) {
                Ok(()) => (Tone::Success, "Recording deleted.".to_owned()),
                // `delete_recording` says it in plain words (`not_deleted`).
                Err(error) => (Tone::Error, error),
            };
            self.say(tone, words, None);
            self.refresh_recordings();
        }
    }

    fn session_details_action(&mut self, ui: &mut egui::Ui, note: &Note) {
        if Action::quiet("Details")
            .name("Edit session details")
            .show(ui)
            .clicked()
        {
            self.form = Some(Form {
                editing: Some(note.id),
                number: note.number.to_string(),
                title: note.title.clone(),
                date: note.date.clone(),
                template: String::new(),
                error: None,
            });
        }
        if let Some(draft) = &self.draft {
            Self::note_status(ui, draft);
        }
    }

    /// Round 26 FM4, FD3: the session page's results and notices that come and go sit in its
    /// message footer, so the title's line, Last time, the tabs and the recordings never move
    /// when one comes. Made before the body draws (`Messages` hold no borrow of the page), so
    /// the body may change anything; `session_press` acts on the footer's press after it.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: the notice slot of the page keeps its own order (`outcome_at`, design::results::STAGED_ALLOWS)"
    )]
    fn session_messages<'f>(
        &mut self,
        frame: &'f PaneFrame,
        settings: &ProcessingSettings,
        book: &Book,
        plan: &Plan<'_>,
        note: &Note,
        width: f32,
    ) -> Messages<'f, SessionAct> {
        self.expire_move(book);
        self.end_carry_results();
        let shown = self.notice_session().is_none() || self.notice_session() == self.selected;
        if shown && self.job_done() {
            self.job_done = false;
        }
        let mut messages = Messages::new(frame, ("session_footer", self.selected), width);
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
        // banner (`banner_notices`); its result is here.
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
            // The template pane's own press (`draft_press`), and the plan's (`plan_pane`).
            SessionAct::UndoDeleteTemplate | SessionAct::Plan(_) => {}
        }
    }

    fn session_pane(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
        plan: &Plan<'_>,
    ) {
        let Some(note) = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
            .cloned()
        else {
            // Its file went from the folder, and the words are still here. The same
            // header as a note, so that the page does not jump at the next scan: it ends at the
            // note frame's edge, as the editor below does (layout step 7).
            let edges = design::PaneColumns::shared_task("session_gone_columns")
                .fixed(0.0, note_frame_width(ui.ctx()))
                .edges(ui);
            edges.header(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    // M0485: the record control stays beside the title line; the title (the
                    // file's name) and the draft's state are the shared part's.
                    self.record_control(ui, settings);
                    if let Some(draft) = &self.draft {
                        let name = draft.path.file_stem().unwrap_or_default().to_string_lossy();
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                            let _ = design::TitleLine::new(&name)
                                .state(
                                    Self::note_save_status(draft),
                                    &Self::every_draft_state()
                                        .map(|state| Self::note_state_status(&state)),
                                )
                                .show(ui);
                        });
                    }
                })
            });
            self.record_line(ui, settings);
            if self.draft.is_some() {
                // The editor fills its room (lead, P2): its results and notices are the pane's.
                let frame = PaneFrame::new(ui);
                let messages =
                    self.draft_messages(&frame, "session_gone_footer", false, ui.available_width());
                let paned = messages.pane(
                    ui,
                    fills_body(design::pane::FillsReason::NoteEditor),
                    |ui| {
                        self.note_editor(ui, false, None);
                    },
                );
                if let Some(press) = paned.pressed {
                    self.draft_press(press, false);
                }
            }
            return;
        };
        // Layout step 7 (SH11, AH8, AP4, SO9, AO10, SQ6, AQ4, SR6, AR5): the title line, the
        // toolbar and the body share one set of edges, decided now, before anything paints. The
        // title's actions end at the session's content edge, never at the window's.
        let layout = self.session_layout(ui, plan);
        let title_on_own_line = layout.edges.outer(ui).span() < TWO_COLUMNS_FROM;
        layout.edges.header(ui, |ui| {
            ui.horizontal(|ui| {
                // One height whether Record, Stop recording or nothing stands beside the title,
                // so the line and the tabs below keep their y when a recording starts or stops.
                ui.set_min_height(Density::Scan.control());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    self.record_control(ui, settings);
                    if !title_on_own_line || layout.conflict {
                        self.session_details_action(ui, &note);
                    }
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                        note_title(ui, &note.label(), &note.date);
                    });
                });
            });
        });
        self.record_line(ui, settings);
        // The body (Last time, the tabs and their pane) is the pane's; the part places the
        // footer below it.
        let view = self.session_header(ui, book, plan, &note, layout, title_on_own_line);
        // The narrow Plan tab: the plan's pane holds the page's messages too, in its one footer,
        // which follows the plan's rows (`SessionView::page_footer`).
        if view.page_footer() == PageFooter::Plan {
            self.session_content(ui, settings, book, plan, view, &note);
            return;
        }
        let frame = PaneFrame::new(ui);
        let messages =
            self.session_messages(&frame, settings, book, plan, &note, ui.available_width());
        let body = self.session_body_kind(&view);
        let paned = messages.pane(ui, body, |ui| {
            self.session_content(ui, settings, book, plan, view, &note);
        });
        if let Some(press) = paned.pressed {
            self.session_press(press, book, plan);
        }
    }

    /// What the session pane shows, and the edges its title line, toolbar and body share,
    /// before any of them paints (layout step 7): from the pane's room, the shown tab and this
    /// pass's drag of the note's edge. Nothing here draws or changes the page.
    fn session_layout(&self, ui: &egui::Ui, plan: &Plan<'_>) -> SessionLayout {
        let conflict = self
            .draft
            .as_ref()
            .is_some_and(|draft| draft.state == DraftState::Conflict);
        let editing_group_here = plan.group_open && self.group_session == self.selected;
        let beside_from = if editing_group_here {
            design::LIST_MEASURE + design::L + design::READER_MEASURE
        } else {
            PLAN_BESIDE_NOTE_FROM
        };
        let beside = ui.available_width() >= beside_from;
        // In a wide page, the note is on the plan's tab, so there is no second one to choose.
        let shown = if beside && self.tab == Tab::Note {
            Tab::Plan
        } else {
            self.tab
        };
        let columns = design::PaneColumns::shared_task("session_columns");
        let columns = if conflict {
            columns.fixed(0.0, note_frame_width(ui.ctx()))
        } else if shown == Tab::Recordings || (shown == Tab::Plan && !beside && editing_group_here)
        {
            // Rows and readers of the whole room; a group being edited keeps its own width.
            columns.rest(0.0)
        } else if beside {
            let plan_width = if editing_group_here {
                design::LIST_MEASURE
            } else {
                (ui.available_width() * 0.4).clamp(PLAN_WIDTH, PLAN_MAX_WIDTH)
            };
            // The DM drags the note's edge into the room beside it.
            columns
                .fixed(0.0, plan_width)
                .resizable(design::L, note_column())
        } else {
            columns.fixed(0.0, note_frame_width(ui.ctx()))
        };
        SessionLayout {
            conflict,
            beside,
            editing_group_here,
            shown,
            edges: columns.edges(ui),
        }
    }

    /// The session page below its title and state line, above its pane: Last time, the tabs
    /// and the toolbar. They never scroll. Returns what the pane shows.
    fn session_header(
        &mut self,
        ui: &mut egui::Ui,
        book: &mut Book,
        plan: &Plan<'_>,
        note: &Note,
        layout: SessionLayout,
        title_on_own_line: bool,
    ) -> SessionView {
        // A group being assembled needs the height, and last time can wait for it.
        if !(plan.group_open && self.group_session == self.selected) {
            self.last_time_block(ui, book, plan);
        }
        if layout.conflict {
            return SessionView::Conflict;
        }
        let SessionLayout {
            beside,
            editing_group_here,
            mut shown,
            edges,
            ..
        } = layout;
        let tabs: &[(Tab, &str)] = if beside {
            &[(Tab::Plan, "Plan and notes")]
        } else {
            &[(Tab::Plan, "Plan"), (Tab::Note, "Note")]
        };
        // The tab row ends where the shown child ends: the note's frame (beside the plan, the
        // note's column, this pass's drag included), or the whole room for Recordings.
        let last = if beside && shown != Tab::Recordings {
            1
        } else {
            0
        };
        let mut changed_tab = false;
        edges.child_header(last, ui, |ui| {
            ui.horizontal(|ui| {
                let changed = ui
                    .horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        let changed = design::segmented(ui, &mut shown, tabs);
                        recordings_tab(ui, &mut shown, self.recordings.len()) || changed
                    })
                    .inner;
                if changed {
                    if self.tab == Tab::Recordings || shown == Tab::Recordings {
                        self.clear_notice();
                    }
                    self.tab = shown;
                    self.clear_success();
                    changed_tab = true;
                }
                let model = self.plan_model(reader_index(ui).as_deref(), plan);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if (shown == Tab::Note || (shown == Tab::Plan && beside))
                        && (!self.reading_note
                            || has_written_text(&compact_plan_for_reading(
                                &self.note_body(),
                                plan.fights,
                                plan.groups,
                                &model.scene_lines(),
                                beside,
                            )))
                    {
                        let (word, icon) = if self.reading_note {
                            ("Write", icons::EDIT)
                        } else {
                            ("Read", icons::DOCUMENT)
                        };
                        if Action::quiet(word)
                            .icon(icon)
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                        {
                            self.reading_note = !self.reading_note;
                        }
                    }
                    if title_on_own_line {
                        self.session_details_action(ui, note);
                    }
                });
            })
        });
        design::gap(ui, design::Gap::XS);
        // A new tab has other edges: the body takes them now, and the pass is drawn again so
        // that the title line has them too.
        let edges = if changed_tab {
            ui.ctx()
                .request_discard("the session tab changed the pane edges");
            self.session_layout(ui, plan).edges
        } else {
            edges
        };
        SessionView::Tab {
            shown,
            beside,
            editing_group_here,
            edges,
        }
    }

    /// What the pane shows: the note's editor in a conflict, or the shown tab.
    fn session_content(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
        plan: &Plan<'_>,
        view: SessionView,
        note: &Note,
    ) {
        // The page's messages go to the plan's footer only where `page_footer` says so; beside
        // the note, the plan's column keeps the plan's own results alone.
        let page = (view.page_footer() == PageFooter::Plan).then_some((settings, note));
        let SessionView::Tab {
            shown,
            beside,
            editing_group_here,
            edges,
        } = view
        else {
            self.note_editor(ui, false, None);
            return;
        };
        match shown {
            // Layout step 7: the plan and the note's column in the edges the header was drawn in.
            Tab::Plan if beside => edges.show_children(ui, |ui, index| {
                if index == 0 {
                    fixed_scope(ui, editing_group_here, "session_plan_pane", |ui| {
                        self.plan_pane(ui, book, plan, page)
                    });
                } else {
                    fixed_scope(ui, editing_group_here, "session_note_pane", |ui| {
                        self.note_pane(ui, book, plan, beside)
                    });
                }
            }),
            // The same ids in both layouts: crossing the width keeps typed text, focus and scroll.
            // The plan's rows end at the tab row's right edge, as the note's frame does, so the
            // pane has one right edge; the scroll bar's lane lies beyond it. A group being edited
            // keeps its own width.
            Tab::Plan => {
                ui.scope(|ui| {
                    if !editing_group_here {
                        let width = edges.child(ui, 0).span();
                        ui.set_max_width(width + ui.spacing().scroll.allocated_width());
                    }
                    fixed_scope(ui, editing_group_here, "session_plan_pane", |ui| {
                        self.plan_pane(ui, book, plan, page)
                    })
                })
                .inner
            }
            Tab::Note => fixed_scope(ui, editing_group_here, "session_note_pane", |ui| {
                self.note_pane(ui, book, plan, beside)
            }),
            Tab::Recordings => {
                if self.processed_notice_read_here() {
                    self.end_notice();
                }
                self.recordings_tab(ui, settings, book);
            }
        }
    }

    fn notes_view(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
        plan: &Plan<'_>,
    ) {
        // An unreadable note arrives from the folder: its warning is in the bottom banner.
        let holding = self.draft.as_ref().is_some_and(Draft::needs_attention);
        if self.notes.is_empty() && self.form.is_none() && !holding {
            let mut start = false;
            design::empty_state(
                ui,
                icons::SESSIONS,
                "No session notes yet",
                "Start a session from a template. It holds tonight’s plan and your notes.",
                |ui| {
                    start = if self.open_run.is_some() {
                        Action::secondary("New session")
                    } else {
                        Action::primary("New session")
                    }
                    .show(ui)
                    .clicked();
                },
            );
            if start {
                self.open_form();
            }
            return;
        }
        // The first session's form has no list to stand beside.
        if self.notes.is_empty() && !holding {
            design::ScrollColumn::new("session_form").show(ui, |ui| self.show_form(ui));
            return;
        }
        // Beside Lookup at a small window, two columns would crush the session.
        self.list_beside = ui.available_width() >= TWO_COLUMNS_FROM;
        if !self.list_beside {
            if self.list_only && self.form.is_none() {
                self.list_pane(ui);
            } else {
                if self.form.is_none()
                    && design::TitleLine::back(crate::word!("All sessions"), design::BackEdge::Text)
                        .show(ui)
                        .clicked()
                {
                    self.list_only = true;
                }
                if self.form.is_some() {
                    design::ScrollColumn::new("session_form").show(ui, |ui| self.show_form(ui));
                } else {
                    self.session_pane(ui, settings, book, plan);
                }
            }
            return;
        }
        // A hidden list keeps its dragged width for when it shows again: the width is the
        // column's, not the list's choice.
        let list_hides = self.list_hides();
        if !list_hides {
            design::Column::left("session_list_panel", LIST_WIDTH, LIST_WIDTHS)
                .frame(egui::Frame::new().inner_margin(egui::Margin {
                    right: design::S as i8,
                    ..Default::default()
                }))
                .show_inside(ui, |ui| self.list_pane(ui));
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: if list_hides { 0 } else { design::S as i8 },
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                if self.form.is_some() {
                    design::ScrollColumn::new("session_form").show(ui, |ui| self.show_form(ui));
                } else {
                    self.session_pane(ui, settings, book, plan);
                }
            });
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

    fn templates_view(&mut self, ui: &mut egui::Ui) {
        if self.template.is_none()
            && let Some(first) = self.templates.first().map(|t| t.name.clone())
        {
            self.select_template(&first);
        }
        let mut choose = None;
        // Beside Lookup at a small window, two columns would crush the editor.
        let narrow = ui.available_width() < TWO_COLUMNS_FROM;
        if narrow && self.templates_list_only {
            // Round 27 FI4: with the list alone on screen, its footer holds the delete result.
            // The list is the pane's scroll column: the footer follows a short list.
            self.template_list_head(ui);
            let frame = PaneFrame::new(ui);
            let messages = self.list_messages(&frame, "template_list_footer", ui.available_width());
            let paned = messages.pane(ui, design::pane::Body::Content, |ui| {
                self.template_rows(ui, &mut choose);
            });
            if let Some(press) = paned.pressed {
                self.draft_press(press, true);
            }
        } else if narrow {
            if design::TitleLine::back(crate::word!("All templates"), design::BackEdge::Text)
                .show(ui)
                .clicked()
            {
                self.templates_list_only = true;
            }
        } else {
            design::Column::left("template_list_panel", LIST_WIDTH, LIST_WIDTHS)
                .frame(egui::Frame::new().inner_margin(egui::Margin {
                    right: design::S as i8,
                    ..Default::default()
                }))
                .show_inside(ui, |ui| self.template_list(ui, &mut choose));
        }
        if let Some(name) = choose {
            self.templates_list_only = false;
            self.select_template(&name);
        }
        if narrow && self.templates_list_only {
            return;
        }
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                // Round 26 AN1, FN4, SN2, AL1: help keeps the reader measure.
                if self.new_template.is_none() {
                    design::help(
                        ui,
                        "Splat fills {{number}}, {{title}}, {{date}} and {{campaign}}. Obsidian’s core Templates plugin also fills {{date}} and {{title}}.",
                    );
                }
                // Round 26: the template pane's error and its draft's result sit in its message
                // footer, so the editor never moves when one comes.
                // The new template's form is short: the pane's scroll column. The editor is the
                // pane's editor region (layout step 11c, M0436): it grows with its text up to the
                // room left above the footer, then scrolls inside its frame.
                let frame = PaneFrame::new(ui);
                let messages =
                    self.draft_messages(&frame, "template_footer", true, ui.available_width());
                // One thing at a time, so that the DM does not type into another template. The
                // new template's form does not take the region: the pane is then a content pane.
                let paned = messages.pane_editor(ui, |ui, region| {
                    if self.new_template.is_some() {
                        // Round 27: `Cancel` starts its line, and its target reaches into the
                        // gutter: an `L` gutter keeps it clear of the column's splitter.
                        egui::Frame::new()
                            .inner_margin(egui::Margin {
                                left: (design::L - design::S) as i8,
                                ..Default::default()
                            })
                            .show(ui, |ui| self.new_template_form(ui));
                    } else {
                        self.template_editor(ui, region);
                    }
                });
                if let Some(press) = paned.pressed {
                    self.draft_press(press, true);
                }
            });
    }

    /// The template list's footer, with the list alone on screen: the delete result.
    fn list_messages<'f>(
        &self,
        frame: &'f PaneFrame,
        id: &str,
        width: f32,
    ) -> Messages<'f, SessionAct> {
        let mut messages = Messages::new(frame, id, width);
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

    /// The template list's head: the way back to the sessions. It never scrolls.
    fn template_list_head(&mut self, ui: &mut egui::Ui) {
        if design::TitleLine::back(crate::word!("Sessions"), design::BackEdge::Text)
            .show(ui)
            .clicked()
        {
            self.view = View::Notes;
        }
        design::gap(ui, design::Gap::S);
    }

    /// The template list in the wide layout: its head, then its rows in their own column.
    fn template_list(&mut self, ui: &mut egui::Ui, choose: &mut Option<String>) {
        self.template_list_head(ui);
        design::ScrollColumn::new("template_list").show(ui, |ui| {
            self.template_rows(ui, choose);
        });
    }

    /// One row for each template. In the narrow layout the pane is their scroll column.
    fn template_rows(&self, ui: &mut egui::Ui, choose: &mut Option<String>) {
        for template in &self.templates {
            let kind = if template.path.is_some() {
                "Campaign"
            } else {
                "Built-in"
            };
            if design::two_line_row(
                ui,
                Density::Glance.control() + design::S,
                self.new_template.is_none()
                    && self.template.as_deref() == Some(template.name.as_str()),
                &template.name,
                kind,
            )
            .clicked()
            {
                *choose = Some(template.name.clone());
            }
        }
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

    fn new_template_form(&mut self, ui: &mut egui::Ui) {
        let Some(mut name) = self.new_template.take() else {
            return;
        };
        design::heading(ui, design::Heading::Label("New template"));
        // The one inline creation form (round 31, `inline-create-action-order`, N0 M0490): the
        // name and its help, then `Create template` and quiet `Cancel` together on the action
        // line under the field, then the kept line with the reason. Enter in the field creates,
        // Escape cancels.
        let reason = name.trim().is_empty().then_some(TemplateReason::NoName);
        let create = if self.open_run.is_some() {
            design::Create::secondary(CREATE_TEMPLATE)
        } else {
            design::Create::primary(CREATE_TEMPLATE)
        }
        .blocked(reason);
        let shown = design::InlineCreate::new()
            .field(
                design::CreateField::new(TEMPLATE_NAME, "Template name", &mut name)
                    .help("Starts with the title line only."),
            )
            .create(create)
            .kept_line(design::KeptLine::new().cancel(CANCEL, true))
            .actions_under()
            .show(ui);
        let mut keep = !shown.cancel;
        if shown.cancel {
            self.template_error = None;
        }
        // A press is judged on the name after this frame's typing.
        let create = shown.create && !name.trim().is_empty();
        if create && self.create_new_template(&name) {
            keep = false;
        }
        if keep {
            self.new_template = Some(name);
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

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state`, and raw egui calls (design::results::STAGED_ALLOWS)"
    )]
    fn template_editor(&mut self, ui: &mut egui::Ui, region: design::pane::EditorRegion<'_>) {
        let c = colors();
        let Some(index) = self
            .template
            .as_ref()
            .and_then(|name| self.templates.iter().position(|t| &t.name == name))
        else {
            return;
        };
        // Its name and place only (layout PR 9b-C): its words stay in `self.templates`, read
        // there by the event that needs them, so a frame copies none of them.
        let (name, path) = (
            self.templates[index].name.clone(),
            self.templates[index].path.clone(),
        );
        // M0493: the template's title line, the shared part: `More` (its menu: Delete, or Go
        // back to built-in) and the draft's state for a template file; `Customize` and
        // `Read-only` for a built-in one.
        let mut delete = false;
        let more_name = format!("More actions for {}", name);
        let mut line = design::TitleLine::new(&name).role(design::TitleType::Section);
        if path.is_some() {
            let built_in = super::BUILT_IN_TEMPLATES
                .iter()
                .any(|(other, _)| *other == name);
            let delete = &mut delete;
            line = line.menu(
                design::TitleAction::quiet(MORE).name(&more_name),
                move |ui| {
                    *delete = Action::quiet(if built_in {
                        "Go back to built-in"
                    } else {
                        "Delete"
                    })
                    .name(if built_in {
                        "Go back to built-in"
                    } else {
                        "Delete this template"
                    })
                    .density(Density::Read)
                    .show(ui)
                    .clicked();
                    if *delete {
                        ui.close();
                    }
                },
            );
            if let Some(draft) = &self.template_draft {
                line = line.state(
                    Self::draft_status(&draft.state),
                    &Self::every_draft_state().map(|state| Self::draft_status(&state)),
                );
            }
        } else {
            let read_only = Status {
                icon: icons::READ_ONLY,
                word: "Read-only",
                color: c.document.read_only,
            };
            line = line
                .action(design::TitleAction::secondary(CUSTOMIZE).name("Customize this template"))
                .state(read_only, &[read_only]);
        }
        let customize = line.show(ui).clicked(CUSTOMIZE);
        if customize {
            self.customize_template(index);
            return;
        }
        if delete {
            self.delete_template(index);
            return;
        }
        // Ctrl+S is taken before the field and saves after it (issue #513), so words typed in
        // the same frame are saved with it.
        let save = design::save_row::save_key(ui, self.template_draft.is_some() && !self.modal);
        let mut pressed_link = None;
        // The template's text field is its file's (M0495, M0496). A template has no typed owner
        // key yet: its path is the key (`TextKey::from_data`, counted).
        let field = TEMPLATE_TEXT.of(design::TextKey::from_data(
            path.as_deref()
                .map_or_else(|| name.clone(), |path| path.display().to_string()),
        ));
        if let Some(draft) = &mut self.template_draft {
            // The pane's editor region (layout PR 9, step 11c): its fixed frame keeps all four
            // edges in view, grows with the text up to the room above the footer, and the text
            // and its lane scroll inside it.
            let font = design::reader::prose_type(ui).font(ui);
            let output = region.editor(
                ui,
                field,
                &mut draft.body,
                markdown_face(font.clone()),
                |edit| edit.font(font),
            );
            crate::markdown_source::paint_section_rules(ui, &output, Some(draft.body.key()));
            pressed_link = crate::markdown_source::pressed_note_link(ui, &output, None);
            let response = output.response;
            design::named(ui, &response, "Template text");
            if response.changed() {
                draft.edited();
                self.deleted_templates.owner_changed(&kept::Page {
                    root: self.folder.clone().unwrap_or_default(),
                });
            }
        } else {
            // A built-in template reads in the pane's scroll column: it is not an editor.
            // Its words are a document kept while it shows (layout PR 9b): a built-in template
            // never changes, so it is made again only for another template.
            if self
                .builtin_text
                .as_ref()
                .is_none_or(|(other, _)| *other != name)
            {
                self.builtin_text = Some((
                    name.clone(),
                    design::text_draft::TextDraft::new(self.templates[index].body.as_str()),
                ));
            }
            // The pane gave its editor region, which a built-in template does not use: the pane
            // is then a content pane, and the text reads in its scroll column.
            let _ = region;
            let body = &mut self.builtin_text.as_mut().expect("kept above").1;
            let width = ui.available_width();
            let font = design::reader::prose_type(ui).font(ui);
            let output = field.editor(ui, body, markdown_face(font.clone()), |edit| {
                edit.font(font)
                    .desired_width(width)
                    .frame(false)
                    .interactive(false)
            });
            design::named(ui, &output.response, "Built-in template text");
            crate::markdown_source::paint_section_rules(ui, &output, Some(body.key()));
        }
        if save && let Some(draft) = &mut self.template_draft {
            draft.save();
        }
        if let Some(target) = pressed_link {
            self.open_note_link(target);
        }
    }

    /// The Session folder's block on Campaign's Link and folder page (ruling R24): its heading
    /// row, its field with its help, its save row, then its own messages `S` below the save row
    /// (`folder_messages`). `locked`: why the page's links and folders cannot change now (a run).
    ///
    /// The save row's commit place holds `Save folder` in every state. When the current folder
    /// keeps files, that Save is disabled with its reason, and the two choices (`Move files and
    /// save`, `Save without moving`) are the warning's, under the save row: a second press where
    /// `Save folder` was (after a refusal found files) never moves files.
    #[allow(
        clippy::disallowed_methods,
        reason = "R24: a value block’s save row shows its own messages"
    )]
    pub fn folder_block(&mut self, ui: &mut egui::Ui, book: &Book, locked: Option<&str>) {
        let Some(campaign) = self.campaign.clone() else {
            return;
        };
        let frame = PaneFrame::new(ui);
        let (block, _) = design::ValueBlock::new(ui, design::HeadingRow::new("Session folder"));
        let (block, ()) =
            block.field(|ui| design::locked(ui, locked, |ui| self.folder_fields(ui, locked)));
        let (block, save) = block
            .save_row(|ui| design::locked(ui, locked, |ui| self.folder_save_row(ui, book, locked)));
        // A refusal of Save without moving ends when its cause goes (`save_folder_act`).
        if let Some(FolderFault::FilesChanged | FolderFault::FilesGone) = &self.folder_error {
            let files = self.folder_files();
            let offered = self.folder_move_counts(book, &files).is_some();
            let gone = matches!(self.folder_error, Some(FolderFault::FilesGone));
            if offered == gone {
                self.folder_error = None;
            }
        }
        let mut messages = RowMessages::new(&frame);
        self.folder_messages(&frame, &mut messages, book, locked.is_some());
        let pressed = block.messages(messages).map(|press| press.id);
        // One act per frame: Save folder, or one of the warning's two choices. Each reads the
        // disk again at the act (`save_folder_act`).
        let act = if save { Some(FolderAct::Save) } else { pressed };
        if let Some(act) = act {
            self.save_folder_act(&campaign, book, act);
        }
    }

    fn folder_fields(&mut self, ui: &mut egui::Ui, locked: Option<&str>) {
        ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
        let response = ui
            .add_enabled_ui(locked.is_none(), |ui| {
                let width = ui.available_width() - 2.0 * design::S;
                let campaign = self.campaign.as_ref().map(|campaign| campaign.id);
                let response =
                    SESSION_FOLDER
                        .of(campaign)
                        .single_line(ui, &mut self.folder_text, |edit| {
                            edit.desired_width(width)
                                .min_size(egui::vec2(0.0, Density::Read.control()))
                                .margin(egui::vec2(design::S, design::XS))
                        });
                design::named(ui, &response, "Folder");
                response
            })
            .inner;
        design::help(
            ui,
            "Inside the campaign folder, unless the path is absolute. A name such as 30 Sessions is fine.",
        );
        if let Some(reason) = locked {
            ui.ctx()
                .accesskit_node_builder(response.id, |node| node.set_description(reason));
            response.clone().on_disabled_hover_text(reason);
        }
        if response.changed() {
            // An edit of the draft ends a refusal of Save without moving (`save_folder_act`).
            if matches!(
                self.folder_error,
                Some(FolderFault::FilesChanged | FolderFault::FilesGone)
            ) {
                self.folder_error = None;
            }
            if let Some(link) = self.folder_link() {
                // The DM's next edit of the folder ends its move's result (`kept::FolderMoved`).
                self.folder_moves.owner_changed(&link);
            }
        }
        if let Some(folder) = &self.folder {
            // One line for every path (the path rule), so a save never moves the save row.
            design::path_help(ui, "session_folder_now", "Now: ", folder);
            design::help(
                ui,
                "Notes, Timeline.md, Templates and Recordings are all here.",
            );
        }
    }

    /// The Session folder form's save row (M0497): the shared save row (`save_row`) of the
    /// folder's draft, its word (`Unsaved`, `No changes`), Discard and `Save folder` with
    /// `Ctrl+S` at the form's right edge, and why Save waits under it while the folder has
    /// changes. True when the DM pressed Save, or `Ctrl+S`, and Save can run.
    fn folder_save_row(&mut self, ui: &mut egui::Ui, book: &Book, locked: Option<&str>) -> bool {
        let Some(campaign) = self.campaign.clone() else {
            return false;
        };
        ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
        let changed = super::resolve(&campaign, &self.folder_text).ok() != self.folder.clone();
        // The listing the footer read this frame (`folder_scan`, kept until the next scan).
        let files = self.folder_files();
        let has_files = self.folder_move_counts(book, &files).is_some();
        // One rule for the row (`DraftState`): the word, Discard, Save's weight and why it waits.
        let mut state = design::DraftState::new(
            design::save_row::DraftOrigin::Saved,
            if changed {
                design::save_row::Changed::Yes
            } else {
                design::save_row::Changed::No
            },
            design::save_row::Primary::Draft,
        );
        // The page's lock first: while it holds, the warning offers no choice, so Save never
        // points at choices that are not there. With no changes the row says so itself.
        let wait = self
            .folder_wait(book, changed, &files)
            .filter(|wait| *wait != FolderWait::NoChanges)
            .map(FolderWait::words);
        if let Some(why) = locked.or(wait) {
            state = state.waits(why);
        } else if has_files {
            // The warning's two choices under the row save the folder: Save waits in its place.
            state = state.commits_elsewhere("Choose below to move the files or not.");
        }
        // `Ctrl+S` saves exactly when the button it labels can, not under a modal (the app's
        // one test) nor the page's lock.
        let save_key = design::save_row::save_key(ui, state.can_save() && !self.modal);
        // The commit place: `Save folder` in every state, so the place the DM pressed never
        // turns into another act.
        let row = design::save_row(ui, &state, "Save folder");
        if row.discard {
            if let Ok(saved) = folder_setting(&campaign) {
                self.folder_text = saved;
            }
            self.folder_error = None;
        }
        if locked.is_none() && changed {
            design::blocked_reason(ui, state.save_blocked());
        }
        (row.save || save_key) && state.can_save()
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
            // The refusals of Save without moving end at the draft's next edit or Discard
            // (`folder_fields`, Discard), and when their cause goes (`folder_block`): the warning
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

    /// Round 26 FN2, FN5, ruling R24: the Session folder form's messages, `S` below its save row
    /// (`folder_block`), so a typed letter, a refusal or a result never moves the save row: why
    /// the folder cannot be used or a write failed (`FolderFault`), what Save without moving
    /// would hide (`Notice::warning`, with the two choices `Move files and save` and `Save
    /// without moving`, none while the page is locked), and the open campaign's move result from
    /// its store; a choice's press comes back from `ValueBlock::messages`. The folder's listing is read once per disk scan
    /// (`folder_scan`).
    fn folder_messages<'f>(
        &mut self,
        frame: &'f PaneFrame,
        messages: &mut RowMessages<'f, FolderAct>,
        book: &Book,
        page_locked: bool,
    ) {
        let files = self.folder_files();
        if let Some(fault) = &self.folder_error {
            messages.outcome(FaultMessage::new(fault));
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
            messages.outcome(warning);
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

    #[allow(
        clippy::disallowed_methods,
        reason = "with no session folder, the page’s fault and its way out stand in place of the page: a page placeholder (M20)"
    )]
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
        plan: &Plan<'_>,
    ) {
        let context = (self.selected, self.view);
        if self
            .notice_context
            .is_some_and(|previous| previous != context)
        {
            self.clear_notice();
        }
        self.notice_context = Some(context);

        let folder_missing = self.folder.as_ref().is_none_or(|folder| !folder.is_dir());
        let (page_title, count, singular, plural) = if self.view == View::Templates {
            ("Templates", self.templates.len(), "template", "templates")
        } else {
            ("Session", self.notes.len(), "session", "sessions")
        };
        let context = if count == 0 || folder_missing {
            String::new()
        } else {
            design::count(count, singular, plural)
        };
        let in_form = self.form.is_some();
        let empty = self.notes.is_empty();
        let locked = self.draft.as_ref().is_some_and(Draft::needs_attention);
        // The same width test as the page below the header (`notes_view`).
        let offers_list_toggle = self.offers_list_toggle(ui.available_width() >= TWO_COLUMNS_FROM);
        let list_alone = self.list_only && ui.ctx().content_rect().width() < 1500.0;
        // M0498: the page's header is the shared title line. The list's show and hide is its
        // view toggle, at the line's right end whatever comes and goes among the actions.
        let mut line = design::TitleLine::new(page_title)
            .context((!context.is_empty()).then_some(context.as_str()))
            .page();
        if offers_list_toggle {
            let hint = if self.list_hidden {
                "Show the session list (Ctrl+\\)"
            } else {
                "Hide the session list (Ctrl+\\)"
            };
            line = line.toggle(icons::SESSION_LIST, LIST_TOGGLE, hint, !self.list_hidden);
        }
        if self.view == View::Notes && !in_form && !empty && !folder_missing {
            line = line.action(
                design::TitleAction::secondary(NEW_SESSION)
                    .icon(icons::ADD)
                    .blocked(
                        locked.then_some("Other sessions wait until you choose what to keep."),
                    ),
            );
        }
        if self.view == View::Templates && self.new_template.is_none() {
            line = line.action(design::TitleAction::secondary(NEW_TEMPLATE).icon(icons::ADD));
        }
        if self.live.is_some() && (self.view != View::Notes || in_form || list_alone || empty) {
            line =
                line.action(design::TitleAction::quiet(STOP_RECORDING).icon(icons::STOP_RECORDING));
        }
        if self.view == View::Notes {
            line = line.action(design::TitleAction::quiet(TEMPLATES));
        }
        if self.party_hidden {
            line = line.action(design::TitleAction::quiet(SHOW_PARTY).icon(icons::PARTY));
        }
        let header = line.show(ui);
        if header.toggled {
            self.toggle_list();
        }
        if header.clicked(NEW_TEMPLATE) {
            self.templates_list_only = false;
            self.new_template = Some(String::new());
            self.template_error = None;
        }
        if header.clicked(TEMPLATES) {
            self.view = View::Templates;
        }
        if header.clicked(SHOW_PARTY) {
            self.party_bar = Some(true);
        }
        let (stop, new_session) = (header.clicked(STOP_RECORDING), header.clicked(NEW_SESSION));
        if stop {
            self.finish_recording(ui.ctx());
        }
        if new_session {
            self.open_form();
        }
        if folder_missing {
            // The only way out of this page, so it has the bordered recovery action. The words
            // are the fault's, as the Session folder form's footer says them.
            let words = self.folder_error.as_ref().map_or_else(
                || "The session folder is not available.".to_owned(),
                |fault| String::from(fault.clone()),
            );
            let fault = PageFault::Check(notice_sentence(&words));
            if PagePlaceholder::new(&fault)
                .choice(Choice::new(MessageChoice::Choice("Choose folder"), ()))
                .show(ui)
                .is_some()
            {
                self.wants_campaign_link = true;
            }
            return;
        }
        match self.view {
            View::Notes => self.notes_view(ui, settings, book, plan),
            View::Templates => self.templates_view(ui),
        }
        self.delete_dialog(ui.ctx());
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Sessions {
    pub fn busy(&self) -> bool {
        self.job.as_ref().is_some_and(|(job, _)| !job.is_held())
    }

    /// The disk check that otherwise runs once a second of wall time (`CHECK_DISK_EVERY`): the
    /// next `poll` runs it.
    pub fn simulate_disk_check(&mut self) {
        self.last_check = None;
    }

    /// The folder scan that otherwise runs every few seconds of wall time.
    pub fn simulate_folder_scan(&mut self) {
        self.scan_folder();
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

    /// The open note's file changes outside Splat while the DM has unsaved words.
    pub fn simulate_conflict(&mut self) {
        self.simulate_disk_change();
        let draft = self.draft.as_mut().expect("an open note");
        draft.body.push_str("\nTyped before the outside edit.\n");
        draft.edited();
        draft.save();
    }

    /// The disk refuses the open note's writes, as a full disk would: the DM has typed, and every
    /// write fails at its result with the system's full-disk error (code 28), before anything is
    /// written, until `simulate_failed_save_ends`. A direct retry fails too.
    pub fn simulate_failed_save(&mut self) {
        let draft = self.draft.as_mut().expect("an open note");
        draft.failing_write = Some(28);
        draft.body.push_str("\nTyped while the disk was full.\n");
        draft.edited();
        draft.save();
    }

    /// The disk takes writes again.
    pub fn simulate_failed_save_ends(&mut self) {
        if let Some(draft) = &mut self.draft {
            draft.failing_write = None;
        }
    }

    /// The open note's file leaves the folder outside Splat while the DM has unsaved words,
    /// and the next save finds it gone. Returns where the file went, to put it back.
    pub fn simulate_note_gone(&mut self) -> (PathBuf, PathBuf) {
        let draft = self.draft.as_mut().expect("an open note");
        let path = draft.path.clone();
        let folder = path.parent().expect("the session folder");
        let away = folder
            .parent()
            .expect("the parent of the folder")
            .join("note-gone-for-a-while.md");
        fs::rename(&path, &away).expect("the note moves away");
        draft.body.push_str("\nTyped after the file went.\n");
        draft.edited();
        draft.save();
        (path, away)
    }

    /// Processing of the open session finishes now, as the background job would.
    pub fn simulate_job_finished(&mut self) {
        let session = self.selected.expect("an open session");
        self.finish_job(Ok(None), session);
    }

    /// The open note's name, as its warnings and its draft's Undo name it.
    pub fn open_note_name(&self) -> Option<String> {
        self.note_name()
    }

    /// The open note's draft, to put back after a step that a scenario proves aside.
    pub fn draft_snapshot(&self) -> Option<Draft> {
        self.draft.clone()
    }

    pub fn restore_draft(&mut self, draft: Draft) {
        self.draft = Some(draft);
    }

    /// The open note's file is renamed outside Splat, as Obsidian can do.
    pub fn simulate_file_renamed(&mut self) {
        let draft = self.draft.as_ref().expect("an open note");
        let new = draft.path.with_file_name("Renamed in Obsidian.md");
        fs::rename(&draft.path, new).expect("the note");
    }

    /// The open note's file, for a scenario that reads what an act wrote.
    pub fn selected_path(&self) -> Option<&std::path::Path> {
        self.draft.as_ref().map(|draft| draft.path.as_path())
    }

    /// The open note changes outside Splat, as when Obsidian edits the same file.
    pub fn simulate_disk_change(&mut self) {
        let draft = self.draft.as_ref().expect("an open note");
        let text = fs::read_to_string(&draft.path).expect("the note");
        fs::write(&draft.path, format!("{text}\nWritten in Obsidian.\n")).expect("the note");
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
            fs::write(processing::transcript_path(&path), format!(
                "---\ntranscript_parts_done: {part}\ntranscript_parts: {of}\n---\n\nThe party held the bridge.\n"
            )).expect("a partial raw transcript before cancellation");
            self.refresh_recordings();
        }
        if matches!(progress, processing::Progress::Asking { .. }) {
            let transcript = processing::transcript_path(&path);
            fs::write(transcript,
                "---\ntranscript_parts_done: 1\ntranscript_parts: 1\n---\n\nThe party held the bridge.\n")
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

    pub fn simulate_job_end(&mut self, result: Result<Option<String>, String>) {
        let session = self.job.as_ref().expect("a job").1;
        self.finish_job(result, session);
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

/// Keep the note and group editor under stable IDs across the side-by-side breakpoint.
/// Other panes keep their usual IDs.
fn fixed_scope<R>(
    ui: &mut egui::Ui,
    fixed: bool,
    id: &str,
    add: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    if fixed {
        ui.scope_builder(egui::UiBuilder::new().id(egui::Id::new(id)), add)
            .inner
    } else {
        add(ui)
    }
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

    /// Round 30 review F4 of PR 400: every warning of the session note that the banner shows on
    /// any page names its note, with its cause and its tone: the conflict, the missing note,
    /// the discarded and missing note, and the failed save. A template's warning (no name)
    /// keeps `This note`.
    #[test]
    fn a_session_note_warning_names_its_note_in_every_state() {
        use egui_kittest::kittest::Queryable;
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("Session 1 - Ash.md");
        fs::write(&path, "---\nsession_id: 1\n---\n# Ash\n").unwrap();
        const NAME: &str = "Session 1: Ash on the Road";
        let shows = |state: DraftState, discarded: bool, name: Option<&str>, words: &str| {
            let mut draft = Draft::open(&path).unwrap();
            draft.state = state;
            if discarded {
                draft.discard();
                draft.state = DraftState::Missing;
            }
            let mut harness = egui_kittest::Harness::builder()
                .with_size(egui::vec2(700.0, 300.0))
                .build_ui_state(
                    move |ui, draft: &mut Draft| {
                        let _ = Sessions::draft_warning(ui, draft, name);
                    },
                    draft,
                );
            harness.run_steps(2);
            let named = harness.query_all_by_label_contains(words).next().is_some();
            let unnamed = harness
                .query_all_by_label_contains("This note ")
                .next()
                .is_some()
                || harness
                    .query_all_by_label_contains("This file ")
                    .next()
                    .is_some();
            (named, unnamed)
        };
        for (state, discarded, cause) in [
            (
                DraftState::Conflict,
                false,
                "The note of Session 1: Ash on the Road changed on disk.",
            ),
            (
                DraftState::Missing,
                false,
                "The note of Session 1: Ash on the Road is gone from the folder.",
            ),
            (
                DraftState::Missing,
                true,
                "The note of Session 1: Ash on the Road is gone from the folder.",
            ),
            (
                DraftState::Failed("Not saved. The disk is full.".to_owned()),
                false,
                "The note of Session 1: Ash on the Road was not saved. The disk is full.",
            ),
        ] {
            let (named, unnamed) = shows(state.clone(), discarded, Some(NAME), cause);
            assert!(
                named && !unnamed,
                "{state:?} (discarded {discarded}) names its note: {cause}"
            );
        }
        let (named, unnamed) = shows(
            DraftState::Conflict,
            false,
            None,
            "This note changed on disk.",
        );
        assert!(named && unnamed, "a template's warning keeps its words");
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
    fn iter5_c_session_detail_keeps_separators_and_warning_together() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for status in ["Processed", "Processed, with a warning"] {
                    let detail = format!("2026-01-01 · {status} · {} Playing", icons::RUN);
                    for width in [170.0, 200.0, 216.0, 330.0] {
                        let galley = session_detail(ui, None, &detail, None, width);
                        let rows: Vec<String> =
                            galley.lines().into_iter().map(|(row, _)| row).collect();
                        for row in &rows {
                            assert!(!row.trim_end().ends_with('·'), "{width}: {rows:?}");
                        }
                        assert!(
                            rows.iter()
                                .any(|row| row.replace('\u{a0}', " ").contains(status)),
                            "State stays whole: {width}: {rows:?}"
                        );
                    }
                }
                // iter-23-S FA11: a file name wraps at a space, and each of its lines starts
                // where the first does, with no blank before it.
                let file = "Session 1 - A long meeting name.md";
                for width in [100.0, 140.0, 170.0] {
                    let galley = session_detail(ui, Some(file), "2026-01-01", None, width);
                    let rows = galley.lines();
                    assert!(rows.len() > 2, "{width}: the name wraps: {rows:?}");
                    for (words, x) in &rows {
                        assert!(
                            !words.starts_with(char::is_whitespace) && x.abs() <= 0.5,
                            "{width}: {rows:?}"
                        );
                    }
                }
            });
        });
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

    /// Round 13 FT4: the plan column has wider item spacing than other callers. A message's words
    /// still start `XS` after its icon, and a wrapped Undo still lines up with the words.
    #[test]
    #[allow(
        deprecated,
        reason = "the test of the old Undo message under the plan’s spacing"
    )]
    fn message_words_and_undo_keep_their_place_whatever_the_spacing() {
        use crate::design::test_harness::ThemedPage as _;
        use egui_kittest::kittest::Queryable;
        let words = "Removed “Shrine of Cinders” from tonight.";
        let place = |spacing: f32| {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(egui::vec2(260.0, 200.0))
                .themed(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.spacing_mut().item_spacing.x = spacing;
                        design::undo_message(ui, Tone::Info, words, "Undo remove");
                    });
                });
            harness.run_steps(2);
            let message = harness.get_by_label(words).rect();
            let undo = harness.get_by_label("Undo remove").rect();
            (message, undo)
        };
        let (message, undo) = place(design::S);
        // The narrow pane puts Undo below the words.
        assert!(undo.top() >= message.bottom(), "{message:?} {undo:?}");
        for spacing in [design::XS, design::L, design::XL] {
            let (other, other_undo) = place(spacing);
            assert!(
                (other.left() - message.left()).abs() < 0.5,
                "words at {spacing}: {other:?} against {message:?}"
            );
            assert!(
                (other_undo.left() - undo.left()).abs() < 0.5,
                "Undo at {spacing}: {other_undo:?} against {undo:?}"
            );
        }
    }

    #[test]
    fn narrow_plan_summaries_never_end_a_line_with_a_separator() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for summary in [
                    "Held the bridge · Ended · Round 1",
                    "5 party · 2 × Cinder Wolf · Ember Cultist",
                ] {
                    for width in [140.0, 180.0, 220.0, 330.0] {
                        let galley = ui.painter().layout(
                            plan_summary(summary),
                            Type::Small.font(ui),
                            colors().muted,
                            width,
                        );
                        for row in &galley.rows {
                            let text: String = row.glyphs.iter().map(|glyph| glyph.chr).collect();
                            assert!(!text.trim_end().ends_with('·'), "{width}: {text}");
                        }
                    }
                }
            });
        });
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

    /// M0495 (lane N0): the template editor's field is its template's. Words typed and erased
    /// in template A do not come back with Ctrl+Z in template B, drawn at the same place, even
    /// when both hold the same words (the case the field's text guard leaves to the owner key).
    #[test]
    fn a_template_field_keeps_its_own_text_history() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let a = create_template(dir.path(), "Downtime", "## Plan\n").unwrap();
        let b = create_template(dir.path(), "Travel", "## Plan\n").unwrap();
        let mut sessions = Sessions {
            templates: vec![a.clone(), b.clone()],
            ..Sessions::default()
        };
        sessions.select_template(&a.name);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(800.0, 600.0))
            .themed_state(
                |ctx, sessions: &mut Sessions| {
                    egui::CentralPanel::default().show(ctx, |ui| template_pane(ui, sessions));
                },
                sessions,
            );
        harness.run_steps(2);
        harness.get_by_label("Template text").focus();
        harness.run_steps(2);
        // Layout PR 9b: the focused template's quiet frames lay out and hash none of its text.
        let work = design::pane::text_work(|| harness.run_steps(4));
        assert!(
            work.is_quiet(4),
            "9b-quiet: the template editor's quiet frames: {work:?}"
        );
        harness.get_by_label("Template text").type_text("Dusk");
        harness.run_steps(16);
        harness.get_by_label("Template text").type_text("!");
        harness.run_steps(1);
        for _ in 0.."Dusk!".len() {
            harness.key_press(egui::Key::Backspace);
        }
        harness.run_steps(1);
        let body = |harness: &egui_kittest::Harness<'_, Sessions>| {
            harness
                .state()
                .template_draft
                .as_ref()
                .map(|draft| draft.body.as_str().to_owned())
        };
        assert_eq!(body(&harness).as_deref(), Some("## Plan\n"));
        harness.state_mut().select_template(&b.name);
        harness.run_steps(2);
        harness.get_by_label("Template text").focus();
        harness.run_steps(2);
        harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::Z);
        harness.run_steps(2);
        assert_eq!(
            body(&harness).as_deref(),
            Some("## Plan\n"),
            "n0-template-owner: Ctrl+Z in Travel gave it the words typed in Downtime"
        );
    }

    /// Review of PR 594 (Astra, second P1): words typed and erased in note A come back with
    /// Ctrl+Z in A after the page added words to note B, whose body was A's and whose editor
    /// had not drawn. The page's cached editor was still A's; B's edit must not move A's text
    /// guard, and B keeps its words.
    #[test]
    fn a_note_keeps_its_undo_when_the_page_adds_words_to_another_note() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let new = |number| {
            create_note(
                dir.path(),
                &NewNote {
                    number,
                    title: "",
                    date: "2026-01-01",
                    campaign: "Test",
                    template: "## What happened\n",
                },
            )
            .unwrap()
        };
        let (a, b) = (new(1), new(2));
        let mut sessions = Sessions {
            folder: Some(dir.path().to_path_buf()),
            notes: vec![a.clone(), b.clone()],
            ..Sessions::default()
        };
        sessions.select(Some(a.id));
        // The harness draws the open note's editor, or no editor.
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(800.0, 600.0))
            .themed_state(
                |ctx, (sessions, draw, panel): &mut (Sessions, bool, bool)| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        if *draw {
                            sessions.note_editor(ui, *panel, None);
                        }
                    });
                },
                (sessions, true, false),
            );
        let body = |harness: &egui_kittest::Harness<'_, (Sessions, bool, bool)>| {
            harness
                .state()
                .0
                .draft
                .as_ref()
                .map(|draft| draft.body.as_str().to_owned())
                .unwrap_or_default()
        };
        harness.run_steps(2);
        let left = body(&harness);
        harness.get_by_label("Note text").focus();
        harness.run_steps(2);
        // Layout PR 9b: the focused note's quiet frames lay out and hash none of its text.
        let work = design::pane::text_work(|| harness.run_steps(4));
        assert!(
            work.is_quiet(4),
            "9b-quiet: the note editor's quiet frames: {work:?}"
        );
        harness.get_by_label("Note text").type_text("Lost words");
        harness.run_steps(8);
        for _ in 0.."Lost words".len() {
            harness.key_press(egui::Key::Backspace);
        }
        harness.run_steps(2);
        assert_eq!(body(&harness), left, "A's words are typed and erased");
        // To B without drawing its editor; B's body is what A's is now.
        harness.state_mut().1 = false;
        harness.state_mut().0.select(Some(b.id));
        harness.run_steps(1);
        assert_eq!(body(&harness), left, "the case: B's body is A's");
        let ctx = harness.ctx.clone();
        assert!(harness.state_mut().0.add_to_note(&ctx, "B’s summary."));
        // Back to A, and Ctrl+Z in its editor.
        harness.state_mut().0.select(Some(a.id));
        harness.state_mut().1 = true;
        harness.run_steps(2);
        harness.get_by_label("Note text").focus();
        harness.run_steps(2);
        let mut undone = Vec::new();
        while undone.len() < 4 && !body(&harness).contains("Lost words") {
            harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::Z);
            harness.run_steps(2);
            undone.push(body(&harness));
        }
        assert!(
            body(&harness).contains("Lost words"),
            "n0-note-owner: B’s edit kept A’s Undo history: {undone:?}"
        );
        assert!(
            fs::read_to_string(&b.path)
                .unwrap()
                .contains("B’s summary."),
            "n0-note-owner: B keeps its words"
        );
        // M0466: a newly selected note opens at its top, once per change of note; selecting the
        // open note again keeps its place. M0469, M0470: a place the plan wrote to comes into
        // view without focus. Both are the editor viewport's named jump (`jump_editor_view`).
        let offset = |harness: &egui_kittest::Harness<'_, (Sessions, bool, bool)>| {
            let field = NOTE_ON_PAGE.of(harness.state().0.selected).id();
            design::pane::editor_frames(&harness.ctx)
                .into_iter()
                .find(|frame| frame.field() == field)
                .map_or(-1.0, |frame| frame.offset())
        };
        let long: String = (0..150).map(|line| format!("Line {line}\n")).collect();
        {
            let draft = harness.state_mut().0.draft.as_mut().unwrap();
            let _ = draft.body.replace(format!("## What happened\n{long}"));
            draft.edited();
        }
        harness.run_steps(2);
        let view = harness.get_by_label("Note text").rect();
        harness.hover_at(egui::pos2(view.center().x, 300.0));
        harness.event(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, -900.0),
            modifiers: egui::Modifiers::NONE,
        });
        harness.run_steps(3);
        let scrolled = offset(&harness);
        assert!(scrolled > 100.0, "the wheel scrolls the note: {scrolled}");
        harness.state_mut().0.select(Some(a.id));
        harness.run_steps(2);
        assert!(
            (offset(&harness) - scrolled).abs() < 1.0,
            "n0-note-reset: selecting the open note again keeps its place: {} from {scrolled}",
            offset(&harness)
        );
        harness.state_mut().0.select(Some(b.id));
        harness.run_steps(2);
        harness.state_mut().0.select(Some(a.id));
        harness.run_steps(3);
        assert!(
            offset(&harness).abs() < 0.5,
            "n0-note-reset: a newly selected note opens at its top: {}",
            offset(&harness)
        );
        let late = harness.state().0.draft.as_ref().unwrap().body.as_str()[..]
            .find("Line 140")
            .map(|at| {
                harness.state().0.draft.as_ref().unwrap().body.as_str()[..at]
                    .chars()
                    .count()
            })
            .unwrap();
        harness.ctx.memory_mut(|memory| {
            if let Some(focused) = memory.focused() {
                memory.surrender_focus(focused);
            }
        });
        harness.state_mut().0.caret_to_show = Some(late);
        harness.run_steps(3);
        assert!(
            offset(&harness) > 2000.0,
            "n0-note-show: the plan's place at line 140 comes into view, no focus: {}",
            offset(&harness)
        );
        // Lead, choice a: in the Notes panel the editor holds the note from its `## What
        // happened` on; what is typed there goes into that part, and the part before it stays.
        {
            let draft = harness.state_mut().0.draft.as_mut().unwrap();
            let _ = draft
                .body
                .replace("# Session 1\n\nRecap words.\n\n## Plan for tonight\n\n## What happened\nNight one.\n");
            draft.edited();
        }
        harness.state_mut().2 = true;
        harness.run_steps(2);
        let shown = harness
            .get_by_label("Note text")
            .value()
            .unwrap_or_default();
        assert!(
            shown.starts_with("## What happened") && !shown.contains("Recap words."),
            "n0-panel-slice: the panel's editor starts at What happened: {shown:?}"
        );
        harness.get_by_label("Note text").focus();
        harness.run_steps(2);
        harness.get_by_label("Note text").type_text(" More.");
        harness.run_steps(2);
        let note = body(&harness);
        assert!(
            note.starts_with(
                "# Session 1\n\nRecap words.\n\n## Plan for tonight\n\n## What happened"
            ) && note.contains("Night one.")
                && note.contains(" More."),
            "n0-panel-slice: what is typed in the panel goes into the note, its start kept: {note:?}"
        );
        // Reviews of PRs 799 and 825 (Sol 2-009): a pair from the same saved note. The panel's
        // words, and the note's part from What happened on as saved.
        let path = harness.state().0.draft.as_ref().unwrap().path.clone();
        let panel = |harness: &egui_kittest::Harness<'_, (Sessions, bool, bool)>| {
            harness
                .get_by_label("Note text")
                .value()
                .unwrap_or_default()
        };
        let saved = |harness: &mut egui_kittest::Harness<'_, (Sessions, bool, bool)>| {
            harness.state_mut().0.draft.as_mut().unwrap().save();
            let file = fs::read_to_string(&path).unwrap();
            file.split_once("## What happened")
                .map(|(_, tail)| tail.to_owned())
                .unwrap_or_default()
        };
        let undo = |harness: &mut egui_kittest::Harness<'_, (Sessions, bool, bool)>,
                    until_gone: &str| {
            let mut undone = Vec::new();
            while undone.len() < 6 && body(harness).contains(until_gone) {
                harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::Z);
                harness.run_steps(2);
                undone.push(body(harness));
            }
            undone
        };
        let _ = saved(&mut harness);
        // (a) The app's own whole-note write (a reading's summary through `add_to_note`) keeps
        // the panel's Undo: Ctrl+Z there takes back what the DM typed and leaves the app's words.
        let ctx = harness.ctx.clone();
        harness
            .get_by_label("Note text")
            .type_text(" Typed in play.");
        harness.run_steps(2);
        assert!(harness.state_mut().0.add_to_note(&ctx, "A summary."));
        harness.run_steps(2);
        assert!(
            panel(&harness).contains("A summary.") && panel(&harness).contains(" Typed in play."),
            "the summary is in the panel beside the typing: {:?}",
            panel(&harness)
        );
        // An act through the note's one write path (`Draft::apply`, as a plan's rewards): a plan
        // line, in the part before What happened.
        harness.state_mut().0.add_to_plan("Old mill");
        harness.run_steps(2);
        let undone = undo(&mut harness, " Typed in play.");
        let (words, file) = (panel(&harness), saved(&mut harness));
        assert!(
            !words.contains(" Typed in play.")
                && words.contains("A summary.")
                && !file.contains(" Typed in play.")
                && file.contains("A summary.")
                && body(&harness).contains("- [[Old mill]]"),
            "n0-panel-undo: after the app's summary and plan line, Ctrl+Z in the panel takes back \
             the typing and keeps the app's words: panel {words:?}, saved {file:?}, note {:?} \
             ({undone:?})",
            body(&harness)
        );
        // Review of PR 825 (P1 825-b; Sol 2-012): the app's write is placed in each older Undo
        // step, or the panel's history ends at the write. Either way no Ctrl+Z takes the app's
        // words out. A note with a section below What happened:
        {
            let draft = harness.state_mut().0.draft.as_mut().unwrap();
            let words = format!("{}\n## Open threads\n- The bell.\n", draft.body.as_str());
            let _ = draft.body.replace(words);
            draft.edited();
        }
        harness.run_steps(2);
        harness.get_by_label("Note text").focus();
        harness.run_steps(2);
        let field = NOTE_IN_PANEL.of(harness.state().0.selected).id();
        // The panel's caret at a place in its words (characters), then words typed there.
        let type_at = |harness: &mut egui_kittest::Harness<'_, (Sessions, bool, bool)>,
                       at: usize,
                       words: &str| {
            let mut state = egui::TextEdit::load_state(&harness.ctx, field).unwrap_or_default();
            state
                .cursor
                .set_char_range(Some(egui::text::CCursorRange::one(
                    egui::text::CCursor::new(at),
                )));
            egui::TextEdit::store_state(&harness.ctx, field, state);
            harness.run_steps(2);
            harness.get_by_label("Note text").type_text(words);
            harness.run_steps(2);
        };
        let presses = |harness: &mut egui_kittest::Harness<'_, (Sessions, bool, bool)>| {
            let mut undone = Vec::new();
            for _ in 0..4 {
                harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::Z);
                harness.run_steps(2);
                undone.push(body(harness));
            }
            undone
        };
        // Refused: typed in the thread, then a summary at the end of What happened, above the
        // typing. The step before the typing has no place for it: the history ends.
        let end = panel(&harness).chars().count();
        type_at(&mut harness, end, " Thread typed.");
        assert!(harness.state_mut().0.add_to_note(&ctx, "A summary 2."));
        harness.run_steps(2);
        let note = body(&harness);
        assert!(
            note.contains("A summary 2.\n\n## Open threads")
                && note.find("## Open threads") < note.find(" Thread typed."),
            "the summary is above the thread typed in its section: {note:?}"
        );
        let undone = presses(&mut harness);
        let (words, file) = (panel(&harness), saved(&mut harness));
        assert!(
            words.contains("A summary 2.")
                && words.contains(" Thread typed.")
                && file.contains("A summary 2.")
                && file.contains(" Thread typed."),
            "n0-panel-refused: the summary could not reach the step before the typed thread, so \
             the panel's Undo ended and Ctrl+Z changes nothing: panel {words:?}, saved {file:?} \
             ({undone:?})"
        );
        // Placed, after that refusal: typed at the end of What happened, then a summary below
        // the typing. Ctrl+Z takes back only the typing; neither summary goes, also not the one
        // whose write ended the history before.
        let at = panel(&harness)
            .find("\n\n## Open threads")
            .map(|at| panel(&harness)[..at].chars().count())
            .unwrap();
        type_at(&mut harness, at, " Night typed.");
        assert!(harness.state_mut().0.add_to_note(&ctx, "A summary 4."));
        harness.run_steps(2);
        assert!(
            body(&harness).contains(" Night typed.\n\nA summary 4.\n\n## Open threads"),
            "the later summary is below the typing: {:?}",
            body(&harness)
        );
        let undone = presses(&mut harness);
        let (words, file) = (panel(&harness), saved(&mut harness));
        assert!(
            !words.contains(" Night typed.")
                && words.contains("A summary 2.")
                && words.contains("A summary 4.")
                && !file.contains(" Night typed.")
                && file.contains("A summary 2.")
                && file.contains("A summary 4."),
            "n0-panel-placed-after-refused: Ctrl+Z takes back only the typing after a placed \
             summary, and the earlier refused write stays: panel {words:?}, saved {file:?} \
             ({undone:?})"
        );
        // The reviewer's sequence: a renamed link and a summary before Notes draws again.
        let end = panel(&harness).chars().count();
        type_at(&mut harness, end, " Typed [[Old place]].");
        harness.state_mut().0.rename_links("Old place", "New place");
        assert!(harness.state_mut().0.add_to_note(&ctx, "A summary 3."));
        harness.run_steps(2);
        let undone = presses(&mut harness);
        let (words, file) = (panel(&harness), saved(&mut harness));
        assert!(
            words.contains("[[New place]]")
                && words.contains("A summary 3.")
                && !words.contains("[[Old place]]")
                && file.contains("[[New place]]")
                && file.contains("A summary 3.")
                && !file.contains("[[Old place]]"),
            "n0-panel-refused-rename: Ctrl+Z after a renamed link and a summary keeps both: \
             panel {words:?}, saved {file:?} ({undone:?})"
        );
        // (b) Typed and saved, Notes hidden; an outside edit of the note's end, which the disk
        // check reloads; the app's summary before Notes draws again. The reload ends the panel's
        // Undo: Ctrl+Z there takes out neither the outside words nor the app's words.
        harness
            .get_by_label("Note text")
            .type_text(" Before the reload.");
        harness.run_steps(2);
        let _ = saved(&mut harness);
        harness.state_mut().1 = false;
        harness.run_steps(2);
        let text = fs::read_to_string(&path).unwrap();
        fs::write(&path, format!("{text}\nOutside words.\n")).unwrap();
        harness.state_mut().0.draft.as_mut().unwrap().follow_disk();
        assert!(
            body(&harness).contains("Outside words."),
            "the disk check reloaded the outside edit"
        );
        assert!(harness.state_mut().0.add_to_note(&ctx, "A second summary."));
        harness.state_mut().1 = true;
        harness.run_steps(2);
        harness.get_by_label("Note text").focus();
        harness.run_steps(2);
        let reloaded = body(&harness);
        let mut undone = Vec::new();
        for _ in 0..4 {
            harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::Z);
            harness.run_steps(2);
            undone.push(body(&harness));
        }
        let (words, file) = (panel(&harness), saved(&mut harness));
        assert!(
            words.contains("Outside words.")
                && words.contains("A second summary.")
                && file.contains("Outside words.")
                && file.contains("A second summary.")
                && undone.iter().all(|note| *note == reloaded),
            "n0-panel-reload: the reload ended the panel's Undo, so Ctrl+Z in Notes changes \
             nothing and keeps the outside words and the app's words: panel {words:?}, saved \
             {file:?} ({undone:?})"
        );
    }

    /// Issue #513: Ctrl+S is taken before the editor and saves after it, in the note editor and
    /// in the template editor. Words typed and Ctrl+S in ONE input frame are saved together.
    #[test]
    fn ctrl_s_saves_the_words_typed_in_its_frame() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(
            dir.path(),
            &NewNote {
                number: 1,
                title: "",
                date: "2026-01-01",
                campaign: "Test",
                template: "## What happened\n",
            },
        )
        .unwrap();
        let template = create_template(dir.path(), "Short", "## Plan\n").unwrap();
        let mut sessions = Sessions {
            draft: Some(Draft::open(&note.path).unwrap()),
            templates: vec![template.clone()],
            ..Sessions::default()
        };
        sessions.select_template(&template.name);
        assert!(sessions.template_draft.is_some());
        // The harness draws the note's editor, or the template's.
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(800.0, 600.0))
            .themed_state(
                |ctx, (sessions, template): &mut (Sessions, bool)| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        if *template {
                            template_pane(ui, sessions);
                        } else {
                            sessions.note_editor(ui, false, None);
                        }
                    });
                },
                (sessions, false),
            );
        for (template, field, path) in [
            (false, "Note text", note.path.clone()),
            (true, "Template text", template.path.clone().unwrap()),
        ] {
            harness.state_mut().1 = template;
            harness.run_steps(2);
            harness.get_by_label(field).focus();
            harness.run_steps(2);
            let ctrl_s = egui::Event::Key {
                key: egui::Key::S,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            };
            // Review of PR 587 (P1): under a modal (the copy dialog, the app's `modal_open`),
            // Ctrl+S saves nothing in the background.
            harness.state_mut().0.modal = true;
            harness
                .input_mut()
                .events
                .extend([egui::Event::Text("Wait.".into()), ctrl_s.clone()]);
            harness.step();
            let saved = fs::read_to_string(&path).unwrap();
            assert!(
                !saved.contains("Wait."),
                "n0-ctrl-s-modal: {field}: Ctrl+S under a modal saved: {saved:?}"
            );
            harness.state_mut().0.modal = false;
            harness.run_steps(1);
            // Both events in ONE frame's raw input (`harness.event` gives each its own frame).
            let frame = harness.ctx.cumulative_frame_nr();
            harness
                .input_mut()
                .events
                .extend([egui::Event::Text("Dusk.".into()), ctrl_s]);
            harness.step();
            assert_eq!(
                harness.ctx.cumulative_frame_nr() - frame,
                1,
                "one input frame"
            );
            let saved = fs::read_to_string(&path).unwrap();
            assert!(
                saved.contains("Dusk."),
                "n0-ctrl-s-one-frame: {field}: Ctrl+S saves the words typed in its frame: {saved:?}"
            );
        }
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

    /// The template editor in a pane, as `templates_view` draws it: the editor takes the pane's
    /// editor region (layout step 11c).
    fn template_pane(ui: &mut egui::Ui, sessions: &mut Sessions) {
        let frame = PaneFrame::new(ui);
        let messages = Messages::<()>::new(&frame, "template_test", ui.available_width());
        let _ = messages.pane_editor(ui, |ui, region| sessions.template_editor(ui, region));
    }

    /// The page's kept results as the session footer draws them, with the press acted on as
    /// the page does (`kept_lines`, `kept_press`).
    fn kept_harness(
        sessions: Sessions,
        book: Book,
    ) -> egui_kittest::Harness<'static, (Sessions, Book)> {
        use crate::design::test_harness::Themed as _;
        egui_kittest::Harness::builder()
            .with_size(egui::vec2(800.0, 300.0))
            .themed_state(
                |ctx, (page, book): &mut (Sessions, Book)| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let frame = PaneFrame::new(ui);
                        let mut messages = Messages::new(&frame, "kept", ui.available_width());
                        page.kept_lines(&frame, book, &mut messages);
                        let paned = messages.pane(ui, design::pane::Body::Content, |_| ());
                        if let Some(press) = paned.pressed {
                            let _ = page.kept_press(press, book);
                        }
                    });
                },
                (sessions, book),
            )
    }

    #[test]
    fn move_run_undo_restores_the_event_and_expires_after_an_edit() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("note.md");
        fs::write(&path, "## What happened\n").unwrap();
        let old = Uuid::from_u128(1);
        let new = Uuid::from_u128(2);
        let run = Uuid::from_u128(3);
        let sessions = Sessions {
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
        use egui_kittest::kittest::Queryable;
        let mut harness = kept_harness(sessions, book);
        {
            let (page, book) = harness.state_mut();
            page.move_run_to_session(book, run, new);
            assert_eq!(book.events[0].session, Some(new));
            assert_eq!(book.events[0].continued_in, None);
            // The act's result is kept in its store, owned by this campaign's page.
            assert!(page.moves.get(&page.page()).is_some());
        }
        harness.run_steps(2);
        // The footer's own Undo, by its ticket.
        // M0483: the Undo names the run it moves back.
        harness.get_by_label("Undo move Bridge Ambush").click();
        harness.run_steps(2);
        {
            let (page, book) = harness.state_mut();
            assert_eq!(book.events, before);
            assert_eq!(page.open_run.as_ref().unwrap().1, old);
            assert!(page.moves.is_empty());
            // An edit of the note ends a kept move.
            page.move_run_to_session(book, run, new);
            assert!(!page.moves.is_empty());
            page.draft.as_mut().unwrap().body.push_str("A note.");
            page.expire_move(book);
            assert!(page.moves.is_empty());
            // So does a change to the Timeline.
            page.move_run_to_session(book, run, old);
            assert!(!page.moves.is_empty());
            book.add(Kind::Travel, Some(new));
            page.expire_move(book);
            assert!(page.moves.is_empty());
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

    /// M0481 (ruling M18; review of PR 862): a recording's text opened at a timeline marker opens
    /// with the marker's line at the view's top, by the column's typed jump, measured again in
    /// each draw until it lands. Each case opens the text in a fresh page that draws one pass a
    /// frame, so the jump asked in the first frame lands in the next one, and input or a resize
    /// can come before it lands.
    #[test]
    fn a_text_opened_at_a_marker_shows_the_marker_line_at_the_top() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        type Page = egui_kittest::Harness<'static, (Sessions, Book)>;
        let dir = tempfile::tempdir().unwrap();
        let recording = dir.path().join("Session 1.wav");
        fs::write(Made::Marked.path(&recording), "").unwrap();
        let before: String = (1..=60)
            .map(|line| {
                format!(
                    "Line {line} of what the table said before, with enough words that it wraps \
                     at another width.\n\n"
                )
            })
            .collect();
        let reading = || Reading {
            recording: recording.clone(),
            name: "Session 1".to_owned(),
            made: Made::Marked,
            parts: vec![
                (
                    "00:10:00".to_owned(),
                    format!("## 00:10:00\n{before}We rest at the mill.\n\n{before}"),
                ),
                (
                    "00:20:00".to_owned(),
                    "## 00:20:00\nThe second part.\n".to_owned(),
                ),
            ],
            part: 0,
            mark: Some(MarkJump::new("We rest at the mill.".to_owned())),
            marker: None,
            short_of: None,
            shown: None,
        };
        // The text's pane on the left; another scroll column, with a button, on the right.
        let page = || -> Page {
            let mut page = egui_kittest::Harness::builder()
                .with_size(egui::vec2(1000.0, 400.0))
                .themed_state(
                    |ctx, (page, book): &mut (Sessions, Book)| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            let full = ui.available_rect_before_wrap();
                            let (left, right) =
                                full.split_left_right_at_x(full.left() + full.width() * 0.7);
                            ui.scope_builder(egui::UiBuilder::new().max_rect(right), |ui| {
                                design::ScrollColumn::new("other column").show(ui, |ui| {
                                    let _ = Action::secondary("Other button").show(ui);
                                    for line in 0..80 {
                                        ui.label(Type::Body.rich(ui, format!("Other {line}")));
                                    }
                                });
                            });
                            ui.scope_builder(egui::UiBuilder::new().max_rect(left), |ui| {
                                let frame = PaneFrame::new(ui);
                                let messages =
                                    Messages::<()>::new(&frame, "reading", ui.available_width());
                                let _ = messages.pane(ui, design::pane::Body::Content, |ui| {
                                    page.reading_view(ui, book);
                                });
                            });
                        });
                    },
                    (Sessions::default(), Book::default()),
                );
            page.ctx
                .options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
            page.run_steps(2);
            page
        };
        // Open the text: its first draw asks for the jump, which lands in the next frame.
        let open = |page: &mut Page| {
            page.state_mut().0.reading = Some(reading());
            page.step();
        };
        let line = |page: &Page| page.get_by_label("We rest at the mill.").rect();
        let at_top = |page: &Page| (0.0..40.0).contains(&line(page).top());
        // A scroll of `by` (up, or down when negative) with the pointer `at`, in touchpad steps
        // of less than 8 points, which egui applies in their own frame: nothing moves after it.
        let wheel = |page: &mut Page, at: egui::Pos2, by: f32| {
            page.input_mut().events.push(egui::Event::PointerMoved(at));
            let steps = (by.abs() / 6.0).ceil() as usize;
            for _ in 0..steps {
                page.input_mut().events.push(egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: egui::vec2(0.0, by / steps as f32),
                    modifiers: egui::Modifiers::NONE,
                });
            }
            page.step();
        };
        // The text's column, and the other column after the resize to 640 px (its 30 % on the
        // right starts at 448 px).
        let (text, other) = (egui::pos2(350.0, 200.0), egui::pos2(600.0, 200.0));
        // Opened, it lands with the line at the top; the part is made once for all its frames.
        let mut one = page();
        SHOWN_PARTS_MADE.with(|made| made.set(0));
        open(&mut one);
        one.run_steps(6);
        assert!(
            at_top(&one),
            "m0481-jump: the marker's line {:?} is at the view's top",
            line(&one)
        );
        let made = SHOWN_PARTS_MADE.with(std::cell::Cell::get);
        assert_eq!(
            made, 1,
            "m0481-made-once: the shown part is made once, not in each frame"
        );
        // After it landed, the DM's wheel moves the line, and nothing pulls it back.
        let landed = line(&one);
        wheel(&mut one, text, 120.0);
        one.run_steps(6);
        assert!(
            line(&one).top() > landed.top() + 60.0,
            "m0481-landed-wheel: the wheel moved the line from {landed:?} to {:?}, and it stays",
            line(&one)
        );
        // Another part is made once more, and then not again.
        one.state_mut().0.reading.as_mut().unwrap().part = 1;
        one.run_steps(6);
        let made = SHOWN_PARTS_MADE.with(std::cell::Cell::get);
        assert_eq!(made, 2, "m0481-made-per-part: another part is made once");
        // A resize before it lands: the line is measured again in the new layout.
        let mut resized = page();
        open(&mut resized);
        resized.set_size(egui::vec2(640.0, 400.0));
        resized.run_steps(8);
        assert!(
            at_top(&resized),
            "m0481-resize: after a resize before it landed, the line {:?} is at the view's top",
            line(&resized)
        );
        // The DM's wheel in the text's own column before it lands ends it, also with a resize in
        // the same frame (which moves the line, so the jump would be asked again).
        let mut own = page();
        open(&mut own);
        own.set_size(egui::vec2(640.0, 400.0));
        wheel(&mut own, text, -120.0);
        own.run_steps(6);
        assert!(
            line(&own).top() > 400.0,
            "m0481-own-wheel: the wheel in the text's column ended the jump; the line {:?} stays \
             below the view",
            line(&own)
        );
        // A wheel in another column does not end it: with a resize in the same frame, the jump
        // asked before it is stale, and only the jump asked again lands the line.
        let mut wheeled = page();
        open(&mut wheeled);
        wheeled.set_size(egui::vec2(640.0, 400.0));
        wheel(&mut wheeled, other, 120.0);
        wheeled.run_steps(6);
        assert!(
            at_top(&wheeled),
            "m0481-other-wheel: a wheel in another column leaves the jump: the line {:?}",
            line(&wheeled)
        );
        // A key in another column (the focus there) does not end it, also with a resize.
        let mut keyed = page();
        keyed.get_by_label("Other button").focus();
        keyed.run_steps(2);
        open(&mut keyed);
        keyed.set_size(egui::vec2(640.0, 400.0));
        keyed.input_mut().events.push(egui::Event::Key {
            key: egui::Key::ArrowDown,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        });
        keyed.step();
        keyed.run_steps(6);
        assert!(
            at_top(&keyed),
            "m0481-other-key: a key in another column leaves the jump: the line {:?}",
            line(&keyed)
        );
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

    /// The Session folder form's messages, as another page's typed pane shows them
    /// (`folder_messages`), drawn as the Link and folder page draws the form: the campaign
    /// switch (`poll`), the save row, then the pane with the fields. A move's result is its
    /// campaign's: A → B → A shows each campaign its own; a disk scan ends A's when its folder
    /// is gone, and A's folder coming back does not bring it back; the DM's edit of the folder
    /// ends B's only. The missing folder is a typed fault in a sentence (routing row 136).
    #[test]
    fn a_folder_move_result_is_its_campaigns_over_a_switch_and_a_lost_folder() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::{NodeT, Queryable};
        let dirs = [
            tempfile::tempdir().unwrap(),
            tempfile::tempdir().unwrap(),
            tempfile::tempdir().unwrap(),
        ];
        let [a, b, c] = [0, 1, 2].map(|at| {
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
            if at == 2 {
                // C keeps no session file yet: its form offers Save folder.
                return campaign;
            }
            if at == 0 {
                fs::create_dir_all(folder.join(super::super::TEMPLATES)).unwrap();
                for name in ["One", "Two"] {
                    fs::write(
                        folder
                            .join(super::super::TEMPLATES)
                            .join(format!("{name}.md")),
                        name,
                    )
                    .unwrap();
                }
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
        // The fourth value is the page's lock (a run), as the Link and folder page passes it.
        type State = (Sessions, Book, Campaign, Option<&'static str>);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(900.0, 700.0))
            .themed_state(
                |ctx, (sessions, book, campaign, lock): &mut State| {
                    sessions.poll(ctx, Some(campaign), book);
                    egui::CentralPanel::default().show(ctx, |ui| {
                        sessions.folder_block(ui, book, *lock);
                    });
                },
                (Sessions::default(), Book::default(), a.clone(), None),
            );
        harness.run_steps(2);
        let shows = |harness: &egui_kittest::Harness<'_, State>, words: &str| {
            harness.query_all_by_label_contains(words).next().is_some()
        };
        let move_to = |harness: &mut egui_kittest::Harness<'_, State>, typed: &str| {
            harness.state_mut().0.folder_text = typed.to_owned();
            harness.run_steps(2);
            harness.get_by_label(MOVE_FILES).click();
            harness.run_steps(3);
        };
        let switch = |harness: &mut egui_kittest::Harness<'_, State>, to: &Campaign| {
            harness.state_mut().2 = to.clone();
            harness.run_steps(3);
        };
        let ctrl_s = |harness: &mut egui_kittest::Harness<'_, State>| {
            harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::S);
            harness.run_steps(3);
        };
        const A_MOVED: &str = "Moved 1 note and 1 template to Moved A.";
        const B_MOVED: &str = "Moved 1 note to Moved B.";
        const MISSING: &str = "Splat cannot find the session folder ";
        // The rail's `Recording saved` is its campaign's (row 100 review, point 5): a recording
        // stopped in A says so in A; B's rail says nothing of it; back in A it says so again,
        // until `SAVED_WORD_FOR` ends it.
        let saved = |harness: &egui_kittest::Harness<'_, State>| {
            harness
                .state()
                .0
                .rail_recording_status()
                .map(|status| status.word)
        };
        let audio = tempfile::tempdir().unwrap();
        harness.state_mut().0.live = Some(Live {
            session: Uuid::nil(),
            label: "Session 1".to_owned(),
            recorder: crate::recording::Recorder::start(
                crate::recording::Tool::Simulated,
                None,
                audio.path().join("live.wav"),
            )
            .unwrap(),
        });
        let ctx = harness.ctx.clone();
        harness.state_mut().0.finish_recording(&ctx);
        harness.run_steps(2);
        assert_eq!(saved(&harness), Some("Recording saved"), "A's own rail");
        // Review F1 of PR 444: the timed words' revision changes with each visible change of the
        // word (here a campaign switch and back), and not on an idle poll.
        let in_a = harness.state().0.recording_revision();
        harness.step();
        assert_eq!(
            harness.state().0.recording_revision(),
            in_a,
            "an idle poll is no change"
        );
        // Round 30 review of PR 400 (F3): the footer's `Recording saved: …` is kept by A's
        // session until the campaign changes; the change ends it, and coming back does not
        // bring it back (only the rail's word, its own fact, returns).
        assert!(
            !harness.state().0.saved.is_empty(),
            "A keeps its saved result"
        );
        switch(&mut harness, &b);
        assert!(
            harness.state().0.saved.is_empty(),
            "the campaign change ends A's saved result"
        );
        assert_eq!(
            saved(&harness),
            None,
            "B's rail says nothing of A's recording"
        );
        let in_b = harness.state().0.recording_revision();
        assert_ne!(in_b, in_a, "the word went: a new revision");
        switch(&mut harness, &a);
        assert!(
            harness.state().0.saved.is_empty(),
            "back in A, the ended result stays ended"
        );
        assert_eq!(saved(&harness), Some("Recording saved"), "back in A");
        assert_ne!(
            harness.state().0.recording_revision(),
            in_b,
            "the word came back: a new revision"
        );
        // Round 30 re-check N1 of PR 400: with no input, a kept `Recording saved: …` still ends
        // with its file. X and Y record in two sessions; the idle frame asks to wake by the next
        // disk check; X's file goes; that wake ends X, and Y stays.
        let (x, y) = (Uuid::new_v4(), Uuid::new_v4());
        for (session, file) in [(x, "x.wav"), (y, "y.wav")] {
            harness.state_mut().0.live = Some(Live {
                session,
                label: "Session".to_owned(),
                recorder: crate::recording::Recorder::start(
                    crate::recording::Tool::Simulated,
                    None,
                    audio.path().join(file),
                )
                .unwrap(),
            });
            let ctx = harness.ctx.clone();
            harness.state_mut().0.finish_recording(&ctx);
        }
        let wakes = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let heard = wakes.clone();
        harness.ctx.set_request_repaint_callback(move |info| {
            heard.lock().unwrap().push(info.delay);
        });
        // The disk check ran in this frame: the next one is due `CHECK_DISK_EVERY` later.
        harness.state_mut().0.last_check = None;
        harness.step();
        let soonest = wakes.lock().unwrap().iter().min().copied();
        assert!(
            soonest.is_some_and(|delay| delay <= CHECK_DISK_EVERY),
            "an idle frame with a kept result asks to wake by the next disk check: {soonest:?}"
        );
        std::fs::remove_file(audio.path().join("x.wav")).unwrap();
        // The wake: the next frame, once the check is due.
        harness.state_mut().0.last_check = Some(Instant::now() - CHECK_DISK_EVERY);
        harness.step();
        let kept = |harness: &egui_kittest::Harness<'_, State>, session: Uuid| {
            let sessions = &harness.state().0;
            sessions
                .saved
                .get(&sessions.of_session(Some(session)))
                .is_some()
        };
        assert!(
            !kept(&harness, x) && kept(&harness, y),
            "the wake ends X with its file, and Y stays"
        );
        harness.ctx.set_request_repaint_callback(|_| {});
        // Review of PR 346, item 1: the form shows two templates; one is deleted outside Splat
        // before the next scan, and the move's result says what it moved, not what was drawn.
        harness.state_mut().0.folder_text = "Moved A".to_owned();
        harness.run_steps(2);
        assert!(shows(
            &harness,
            "The current folder keeps 1 note and 2 templates."
        ));
        // The warning before the move is a warning (`Notice::warning`, PR 350), as the untyped
        // footer draws it: the part registers it in the warning tone, and paints the warning's
        // tone bar inside it, as tall as its words (the driver's `has_tone_bar` rule: from the
        // words' top to their bottom; one line in the value block, where the words are wider than
        // in the old footer).
        let words = harness
            .get_by_label_contains("The current folder keeps 1 note and 2 templates.")
            .rect();
        let has_tone_bar = |shape: &egui::Shape, message: egui::Rect, color: Color32| -> bool {
            fn bars(shape: &egui::Shape, out: &mut Vec<egui::epaint::RectShape>) {
                match shape {
                    egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| bars(shape, out)),
                    egui::Shape::Rect(bar) => out.push(bar.clone()),
                    _ => {}
                }
            }
            let mut found = Vec::new();
            bars(shape, &mut found);
            found.iter().any(|bar| {
                bar.fill == color
                    && bar.rect.width() <= design::XS
                    && message.expand(1.0).contains_rect(bar.rect)
                    && (bar.rect.top() - words.top()).abs() <= 1.0
                    && bar.rect.bottom() >= words.bottom() - 1.0
            })
        };
        let warnings: Vec<egui::Rect> = design::outcome::message_rects(&harness.ctx)
            .into_iter()
            .filter(|(_, _, tone)| *tone == Tone::Warning)
            .map(|(rect, _, _)| rect)
            .collect();
        assert!(
            warnings.len() == 1
                && harness.output().shapes.iter().any(|clipped| has_tone_bar(
                    &clipped.shape,
                    warnings[0],
                    colors().result.warning
                )),
            "the folder warning has the warning tone and its bar: {warnings:?}"
        );
        let templates_a = a.path.join("Sessions").join(super::super::TEMPLATES);
        fs::remove_file(templates_a.join("Two.md")).unwrap();
        move_to(&mut harness, "Moved A");
        assert!(shows(&harness, A_MOVED), "the move in A says what it moved");
        switch(&mut harness, &b);
        assert!(
            !shows(&harness, A_MOVED),
            "the page of B does not show the result of A"
        );
        move_to(&mut harness, "Moved B");
        assert!(shows(&harness, B_MOVED), "the move in B says what it moved");
        switch(&mut harness, &a);
        assert!(
            shows(&harness, A_MOVED) && !shows(&harness, B_MOVED),
            "the result of A came back with A, and the result of B stays with B"
        );
        // The folder of A is moved away outside Splat: the disk scan ends the result of A and
        // says why; the folder back, the next scan ends the fault, and the result stays ended.
        let moved_a = a.path.join("Moved A");
        let aside = a.path.join("Kept aside");
        let scan = |harness: &mut egui_kittest::Harness<'_, State>| {
            harness.state_mut().0.simulate_folder_scan();
            harness.run_steps(3);
        };
        fs::rename(&moved_a, &aside).unwrap();
        scan(&mut harness);
        assert!(
            !shows(&harness, A_MOVED),
            "the result of A ends with its folder"
        );
        assert!(
            shows(&harness, MISSING) && shows(&harness, "It may have been moved or renamed."),
            "the missing folder is a sentence"
        );
        assert!(harness.state().0.folder_missing());
        fs::rename(&aside, &moved_a).unwrap();
        scan(&mut harness);
        assert!(
            !shows(&harness, MISSING) && !shows(&harness, A_MOVED),
            "the folder is back by the scan, and the ended result stays ended"
        );
        // Gone again, then a switch to B and back: the result of B stays, and the folder of A,
        // back before the return, opens with no fault.
        fs::rename(&moved_a, &aside).unwrap();
        scan(&mut harness);
        switch(&mut harness, &b);
        assert!(
            shows(&harness, B_MOVED),
            "the result of B stays over the lost folder of A"
        );
        fs::rename(&aside, &moved_a).unwrap();
        switch(&mut harness, &a);
        assert!(
            !shows(&harness, MISSING) && !shows(&harness, A_MOVED),
            "the folder is back by the switch, and the ended result stays ended"
        );
        // The edit of the folder of B ends the result of B, and only that one.
        move_to(&mut harness, "Moved A2");
        const A2_MOVED: &str = "Moved 1 note and 1 template to Moved A2.";
        assert!(shows(&harness, A2_MOVED));
        switch(&mut harness, &b);
        harness.get_by_label("Folder").click();
        harness.run_steps(2);
        harness.get_by_label("Folder").type_text("x");
        harness.run_steps(3);
        assert!(
            !shows(&harness, B_MOVED),
            "an edit of the folder ends its result"
        );
        switch(&mut harness, &a);
        assert!(
            shows(&harness, A2_MOVED),
            "the edit in B leaves the result of A"
        );
        // Review of PR 346, item 2 and the question: C keeps no file, and the New session form
        // is open with unsent words. Save folder was drawn from the listing; a template that
        // came outside Splat stops it at the press, and the folder setting stays. A scan while
        // the form is open ends the listing, and the next draw counts what is on the disk.
        switch(&mut harness, &c);
        {
            let sessions = &mut harness.state_mut().0;
            sessions.open_form();
            sessions.form.as_mut().unwrap().title = "Unsent words".to_owned();
            sessions.folder_text = "Elsewhere".to_owned();
        }
        // The page's own list of note paths is the one file; with it gone, a scan while the
        // form is open ends the listing (no refresh writes it again), and Save folder shows.
        fs::remove_file(c.path.join("Sessions").join(super::super::NOTE_PATHS)).unwrap();
        harness.state_mut().0.simulate_folder_scan();
        harness.run_steps(2);
        assert!(shows(&harness, "Save folder") && !shows(&harness, MOVE_FILES));
        let templates_c = c.path.join("Sessions").join(super::super::TEMPLATES);
        fs::create_dir_all(&templates_c).unwrap();
        fs::write(templates_c.join("Late.md"), "Late").unwrap();
        // The press is the pointer's, at Save folder's place, so a second press there is the
        // DM's double click.
        let save_at = harness.get_by_label("Save folder").rect();
        let press = |harness: &mut egui_kittest::Harness<'_, State>| {
            for down in [true, false] {
                harness.event(egui::Event::PointerMoved(save_at.center()));
                harness.event(egui::Event::PointerButton {
                    pos: save_at.center(),
                    button: egui::PointerButton::Primary,
                    pressed: down,
                    modifiers: egui::Modifiers::NONE,
                });
                harness.step();
            }
            harness.run_steps(3);
        };
        press(&mut harness);
        assert!(
            shows(&harness, "The current folder now keeps session files."),
            "Save folder reads the disk at the press"
        );
        assert_eq!(super::super::folder_setting(&c).unwrap(), DEFAULT_FOLDER);
        assert!(
            shows(&harness, MOVE_FILES) && shows(&harness, "The current folder keeps 1 template."),
            "after the refused press the form shows what is on the disk"
        );
        // Review of the early commit, point 1: the refused Save keeps its place, disabled with
        // its reason, and the move choices are outside its former hit area; a second press
        // there moves nothing.
        let save = harness.get_by_label("Save folder");
        assert!(
            save.rect() == save_at && save.accesskit_node().is_disabled(),
            "the refused Save keeps its place, disabled: {:?}",
            save.rect()
        );
        assert!(
            shows(&harness, "Choose below to move the files or not."),
            "the disabled Save says why"
        );
        for choice in [MOVE_FILES, "Save without moving"] {
            let rect = harness.get_by_label(choice).rect();
            assert!(
                !rect.intersects(save_at),
                "{choice} is outside Save folder’s place: {rect:?}"
            );
        }
        press(&mut harness);
        assert!(
            templates_c.join("Late.md").is_file()
                && super::super::folder_setting(&c).unwrap() == DEFAULT_FOLDER,
            "a second press where Save folder was moves nothing and saves nothing"
        );
        fs::write(templates_c.join("Later.md"), "Later").unwrap();
        harness.state_mut().0.simulate_folder_scan();
        harness.run_steps(3);
        assert!(
            shows(&harness, "The current folder keeps 2 templates."),
            "a scan with the New session form open ends the listing"
        );
        assert_eq!(
            harness
                .state()
                .0
                .form
                .as_ref()
                .map(|form| form.title.as_str()),
            Some("Unsent words"),
            "the form keeps its unsent words"
        );
        // Astra FINAL review of 83e0fd5f, P2 1: an act commits only what the DM was shown.
        // A template comes outside Splat after the warning was drawn (no scan): Save without
        // moving reads the folder at the press, refuses, and the warning names the new counts.
        fs::write(templates_c.join("Third.md"), "Third").unwrap();
        harness.get_by_label("Save without moving").click();
        harness.run_steps(3);
        assert_eq!(super::super::folder_setting(&c).unwrap(), DEFAULT_FOLDER);
        assert!(
            shows(
                &harness,
                "Not changed. The files in the current folder changed."
            ) && shows(&harness, "The current folder keeps 3 templates."),
            "Save without moving refuses files its warning did not name"
        );
        // Re-check of da245d42, P2 1: the refusal ends at the draft's next edit.
        harness.get_by_label("Folder").click();
        harness.run_steps(2);
        harness.get_by_label("Folder").type_text("2");
        harness.run_steps(3);
        assert!(
            !shows(&harness, "Check the new counts") && shows(&harness, MOVE_FILES),
            "an edit of the folder ends the refusal; the warning and its choices stay"
        );
        // The folder cannot be read at the press: no choice commits.
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&templates_c, fs::Permissions::from_mode(0o000)).unwrap();
            harness.get_by_label("Save without moving").click();
            harness.run_steps(3);
            let fault = harness.state().0.folder_error.clone();
            fs::set_permissions(&templates_c, fs::Permissions::from_mode(0o755)).unwrap();
            assert_eq!(super::super::folder_setting(&c).unwrap(), DEFAULT_FOLDER);
            assert!(
                matches!(fault, Some(FolderFault::CannotRead(_))),
                "an unreadable folder stops Save without moving: {fault:?}"
            );
        }
        harness.state_mut().0.simulate_folder_scan();
        harness.run_steps(3);
        // Re-check of da245d42, P2 1 a: the refusal ends when its cause goes. Refused again,
        // then the original folder typed back: No changes, and no sentence pointing at nothing.
        fs::write(templates_c.join("Fourth.md"), "Fourth").unwrap();
        harness.get_by_label("Save without moving").click();
        harness.run_steps(3);
        assert!(shows(&harness, "Check the new counts"));
        harness.state_mut().0.folder_text = DEFAULT_FOLDER.to_owned();
        harness.run_steps(3);
        assert!(
            shows(&harness, "No changes") && !shows(&harness, "Check the new counts"),
            "the folder typed back ends the refusal with its warning"
        );
        // b: at the refused press the folder has no file left. The words name the control that
        // is there (Save folder), and they end when files come back.
        harness.state_mut().0.folder_text = "Elsewhere".to_owned();
        harness.run_steps(3);
        assert!(shows(&harness, "Save without moving"));
        fs::remove_dir_all(&templates_c).unwrap();
        harness.get_by_label("Save without moving").click();
        harness.run_steps(3);
        assert_eq!(super::super::folder_setting(&c).unwrap(), DEFAULT_FOLDER);
        assert!(
            shows(
                &harness,
                "Not changed. The current folder has no files now. Use Save folder."
            ) && !shows(&harness, "Check the new counts")
                && !shows(&harness, MOVE_FILES)
                && !harness
                    .get_by_label("Save folder")
                    .accesskit_node()
                    .is_disabled(),
            "an empty folder at the press points at Save folder, which is there"
        );
        fs::create_dir_all(&templates_c).unwrap();
        fs::write(templates_c.join("Back.md"), "Back").unwrap();
        harness.state_mut().0.simulate_folder_scan();
        harness.run_steps(3);
        assert!(
            !shows(&harness, "has no files now") && shows(&harness, MOVE_FILES),
            "files back end the refusal"
        );
        // M0497: with no file to move (C's templates gone again), Ctrl+S saves the folder (the
        // shared save row's key), but not under a modal. After the refusal's own checks, so
        // nothing here ends it before they look. C is not used again.
        fs::remove_dir_all(&templates_c).unwrap();
        harness.state_mut().0.simulate_folder_scan();
        harness.run_steps(3);
        assert!(!shows(&harness, MOVE_FILES), "no file to move");
        harness.state_mut().0.modal = true;
        ctrl_s(&mut harness);
        assert_eq!(
            super::super::folder_setting(&c).unwrap(),
            DEFAULT_FOLDER,
            "n0-folder-ctrl-s: under a modal Ctrl+S saves nothing"
        );
        harness.state_mut().0.modal = false;
        ctrl_s(&mut harness);
        assert_eq!(
            super::super::folder_setting(&c).unwrap(),
            "Elsewhere",
            "n0-folder-ctrl-s: Ctrl+S saves the folder"
        );
        // Early-pass packet, point 3: the current folder's `Now:` line is one line, so a save to
        // a long folder and back to a short one never moves the save row under it.
        switch(&mut harness, &b);
        let save_top = |harness: &egui_kittest::Harness<'_, State>| {
            harness.get_by_label("Save folder").rect().top()
        };
        let clean = save_top(&harness);
        let long = "A folder with a name long enough to wrap the current path line twice over at this width";
        move_to(&mut harness, long);
        assert!(shows(&harness, "Moved 1 note to A folder with a name long"));
        let after_long = save_top(&harness);
        move_to(&mut harness, "B");
        let after_short = save_top(&harness);
        assert!(
            after_long == clean && after_short == clean,
            "the save row stays: clean {clean}, after the long path {after_long}, after the short one {after_short}"
        );
        let now = harness.get_by_label_contains("Now: ");
        assert!(
            now.rect().height() <= Type::Small.line() + 1.0,
            "the Now: line is one line: {:?}",
            now.rect()
        );
        // Astra FINAL review of 83e0fd5f, P2 3: the shortened line is kept; an unchanged
        // redraw does no shortening work.
        let cuts = crate::design::layout::path_cuts();
        harness.run_steps(4);
        assert_eq!(
            crate::design::layout::path_cuts(),
            cuts,
            "an unchanged redraw shortens no path"
        );
        // Validation of bbadb689, point 1: while the page is locked (a run) and the folder keeps
        // files, the warning offers no choice, and the disabled Save's reason is the lock's,
        // never a pointer at choices that are not there.
        const LOCK: &str = "Close the run to change the folder, the session folder or the link.";
        harness.state_mut().0.folder_text = "Elsewhere".to_owned();
        harness.state_mut().3 = Some(LOCK);
        harness.run_steps(3);
        let save = harness.get_by_label("Save folder");
        assert!(
            save.accesskit_node().is_disabled()
                && save.accesskit_node().description().as_deref() == Some(LOCK),
            "the locked Save says the lock’s reason: {:?}",
            save.accesskit_node().description()
        );
        assert!(
            shows(&harness, "The current folder keeps 1 note.")
                && !shows(&harness, "Choose below")
                && !shows(&harness, MOVE_FILES)
                && !shows(&harness, "Save without moving"),
            "under the lock the warning stays, with no choice and no pointer to one"
        );
        // The hover says the lock's reason alone (a tooltip waits half a second).
        harness.hover_at(harness.get_by_label("Save folder").rect().center());
        harness.run_steps(8);
        assert!(
            shows(&harness, LOCK) && !shows(&harness, "Choose below"),
            "the locked Save’s hover names the lock, not the choices"
        );
        harness.remove_cursor();
        harness.state_mut().3 = None;
        harness.run_steps(3);
        assert!(
            shows(&harness, MOVE_FILES)
                && shows(&harness, "Choose below to move the files or not."),
            "the lock gone, the choices come back"
        );
        // M0497: the folder's save row is the shared one (`save_row`). While the move choices
        // take Save's place, Ctrl+S saves nothing; Discard puts the saved folder back.
        let saved_folder = super::super::folder_setting(&b).unwrap();
        ctrl_s(&mut harness);
        assert!(
            super::super::folder_setting(&b).unwrap() == saved_folder
                && !shows(&harness, "The current folder now keeps session files."),
            "n0-folder-row: Ctrl+S presses nothing while the move choices take Save's place"
        );
        assert!(
            shows(&harness, "Unsaved") && shows(&harness, "Discard"),
            "n0-folder-row: the changed folder says Unsaved, with Discard"
        );
        harness.get_by_label("Discard").click();
        harness.run_steps(3);
        assert!(
            shows(&harness, "No changes") && harness.state().0.folder_text == saved_folder,
            "n0-folder-row: Discard puts the saved folder back"
        );
        // Round 30 B7d: the word ends by the frame clock (`crate::frame_clock`), not the wall
        // clock, so a test's frames decide when it ends: 7.5 s of frames after the stop it
        // stands; 8.5 s after, it has ended.
        // Last in this test, because it moves the frame clock on.
        harness.state_mut().0.live = Some(Live {
            session: Uuid::nil(),
            label: "Session 1".to_owned(),
            recorder: crate::recording::Recorder::start(
                crate::recording::Tool::Simulated,
                None,
                audio.path().join("clock.wav"),
            )
            .unwrap(),
        });
        let ctx = harness.ctx.clone();
        harness.state_mut().0.finish_recording(&ctx);
        harness.step();
        assert_eq!(saved(&harness), Some("Recording saved"), "the stop says it");
        let stopped = harness.state().0.saved_at.map(|(_, at)| at).unwrap();
        // Ruling L9: the owner update (`poll`'s `settle_timed`) ends the word, and until then
        // asks for a repaint at the time left; the rail reads no clock. The words' revision
        // changes at the end, not on a frame with no change.
        let said = harness.state().0.recording_revision();
        crate::frame_clock::set(stopped + 7.5);
        harness.step();
        assert_eq!(
            saved(&harness),
            Some("Recording saved"),
            "7.5 s of frames later"
        );
        // egui asks for the repaint one predicted frame early, so it lands at the end.
        let dt = f64::from(harness.ctx.input(|input| input.predicted_dt));
        let delay = harness.output().viewport_output[&egui::ViewportId::ROOT]
            .repaint_delay
            .as_secs_f64();
        assert!(
            (delay - (0.5 - dt)).abs() < 1e-3,
            "a repaint is asked for at the word's end, 0.5 s on, not {delay}"
        );
        assert_eq!(harness.state().0.recording_revision(), said);
        crate::frame_clock::set(stopped + 8.0);
        harness.step();
        assert_eq!(
            saved(&harness),
            None,
            "8 s of frames later the word has ended"
        );
        assert_ne!(
            harness.state().0.recording_revision(),
            said,
            "its end is a change of the timed words"
        );
    }

    /// Round 28 K4a, the New session form's facts: the chosen template is read from the folder
    /// at the press (changed or removed outside Splat since the form opened), a refused press
    /// keeps the form and its words, and the form belongs to its campaign (a switch ends it, and
    /// B's form never offers A's template).
    #[test]
    fn the_new_session_form_reads_its_template_at_the_press_and_keeps_to_its_campaign() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
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
        type State = (Sessions, Book, Campaign);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(900.0, 700.0))
            .themed_state(
                |ctx, (sessions, book, campaign): &mut State| {
                    sessions.poll(ctx, Some(campaign), book);
                    egui::CentralPanel::default().show(ctx, |ui| sessions.show_form(ui));
                },
                (Sessions::default(), Book::default(), a.clone()),
            );
        harness.run_steps(2);
        let shows = |harness: &egui_kittest::Harness<'_, State>, words: &str| {
            harness.query_all_by_label_contains(words).next().is_some()
        };
        let open = |harness: &mut egui_kittest::Harness<'_, State>, title: &str| {
            let sessions = &mut harness.state_mut().0;
            sessions.open_form();
            let form = sessions.form.as_mut().unwrap();
            form.template = "Mine".to_owned();
            form.title = title.to_owned();
            harness.run_steps(2);
        };
        let notes = |harness: &egui_kittest::Harness<'_, State>| harness.state().0.notes.len();
        // Changed outside before Create: the note holds the template as the folder has it.
        open(&mut harness, "Unsent");
        assert!(shows(&harness, "Session template: Mine"));
        fs::write(&mine, "Changed outside").unwrap();
        harness.get_by_label("Create session").click();
        harness.run_steps(3);
        assert!(harness.state().0.form.is_none(), "the session is made");
        let made = harness
            .state()
            .0
            .notes
            .iter()
            .find(|note| note.title == "Unsent")
            .map(|note| fs::read_to_string(&note.path).unwrap())
            .unwrap_or_default();
        assert!(
            made.contains("Changed outside"),
            "made from the folder's template: {made:?}"
        );
        // Removed outside before Create: the press is refused, the reason shows on the kept
        // line once, and the form keeps its words.
        open(&mut harness, "Second");
        let before = notes(&harness);
        fs::remove_file(&mine).unwrap();
        harness.get_by_label("Create session").click();
        harness.run_steps(3);
        assert_eq!(
            notes(&harness),
            before,
            "no session from a template that is gone"
        );
        assert_eq!(
            harness
                .state()
                .0
                .form
                .as_ref()
                .map(|form| form.title.as_str()),
            Some("Second"),
            "the form keeps its words"
        );
        assert_eq!(
            harness
                .query_all_by_label_contains(FormReason::TemplateGone.words())
                .count(),
            1,
            "the reason shows once"
        );
        // Another campaign and back: the switch ends A's form; B's form offers B's templates.
        harness.state_mut().2 = b.clone();
        harness.run_steps(3);
        assert!(harness.state().0.form.is_none(), "a switch ends the form");
        harness.state_mut().0.open_form();
        harness.run_steps(2);
        assert_eq!(
            harness
                .state()
                .0
                .form
                .as_ref()
                .map(|form| form.template.as_str()),
            Some(super::super::BUILT_IN_TEMPLATES[0].0)
        );
        assert!(!shows(&harness, "Mine") && !shows(&harness, "Second"));
        harness.state_mut().2 = a.clone();
        harness.run_steps(3);
        assert!(
            harness.state().0.form.is_none(),
            "back in A, the form is not B's"
        );
    }
}
