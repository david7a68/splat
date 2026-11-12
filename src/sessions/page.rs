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
use crate::{
    credentials::{self, KeySlot},
    design::{self, Action, Density, Status, Tone, Type, colors, icons},
    encounter_run::FightSummary,
    encounters::PlanFight,
    lookup::{Written, WrittenIn},
    markdown_view,
    persistence::Campaign,
    processing::{self, Job, Request, settings::ProcessingSettings},
    recording::{Health, Recorder},
    timeline::{
        self, Book,
        page::{LiveUndos, SessionLink, added_message, body_above},
    },
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
#[path = "plan.rs"]
mod plan;

const MOVE_FILES: &str = "Move files and save";
const SAVED_WORD_FOR: Duration = Duration::from_secs(8);
const LIST_WIDTH: f32 = 216.0;
const LIST_WIDTHS: std::ops::RangeInclusive<f32> = 170.0..=420.0;
const STATUS_WIDTH: f32 = 128.0;
const TWO_COLUMNS_FROM: f32 = 640.0;
/// The plan and the note stand side by side from here.
const PLAN_BESIDE_NOTE_FROM: f32 = 720.0;
const PLAN_WIDTH: f32 = 330.0;
const PLAN_MAX_WIDTH: f32 = 480.0;
const CHECK_DISK_EVERY: Duration = Duration::from_secs(1);
const SCAN_EVERY: Duration = Duration::from_secs(5);
const MISSING_INPUT_ERROR: &str = "no such device";
const PROCESSING_CANCELLED: &str = "Processing cancelled";

#[derive(Clone, Copy, Default, PartialEq)]
enum View {
    #[default]
    Notes,
    Templates,
}

/// The current session folder's files, or why they could not be listed.
type FolderFiles = Result<Option<Vec<PathBuf>>, String>;

/// What the DM chose in the session page's message footer, done after the footer is drawn.
#[derive(Clone, Copy, Debug, PartialEq)]
enum SessionAct {
    MoveRunHere(Uuid),
    UndoMove,
    Dismiss,
    UndoIdentity,
    PlayThisOne,
    Open(Option<Uuid>),
    OpenSettings,
    UndoCarry(Uuid),
}

#[derive(Clone, Copy, Default, PartialEq)]
enum Tab {
    #[default]
    Plan,
    /// In a narrow page, the note has its own tab. In a wide one, it stands beside the plan.
    Note,
    Recordings,
}

/// What the rest of the app tells this page about fights.
pub struct Plan<'a> {
    pub fights: &'a [PlanFight],
    pub saved_runs: &'a [crate::encounter_run::SavedRunInfo],
    pub groups: &'a [std::sync::Arc<crate::encounter_run::RunGroup>],
    /// Why no group can be assembled now.
    pub group_blocked: Option<&'a str>,
    /// A group being assembled takes the plan's place, beside the note.
    pub group_open: bool,
    /// A new group started in the plan was discarded there.
    pub group_discarded: bool,
    pub group_discarded_name: String,
    /// When that discard was made, for the plan footer's newest-first order.
    pub group_discarded_order: u64,
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
    /// The words to bring into view, once, when a marker opened this.
    mark: Option<String>,
    /// The marker that opened this, with its key, so it can be added from here.
    marker: Option<(crate::timeline::Marker, String)>,
    /// A raw transcript that stops early: parts done, and parts in all.
    short_of: Option<(u64, u64)>,
}

/// An event just added, with its one step of undo. It goes at the next thing the DM does.
struct Added {
    id: String,
    label: String,
    at: Instant,
    count: usize,
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

/// Tests: what runs after a link update writes a note.
#[cfg(test)]
type LinkWriteHook = Box<dyn FnMut(&Path)>;

#[cfg(test)]
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
    pub words: String,
    pub action: String,
    changes: Vec<(Draft, String)>,
    title: String,
}

struct LinkUndoState {
    notes: Vec<(PathBuf, String)>,
    events: Vec<timeline::Event>,
    /// The event's name that the links now say. It ends with this state.
    title: String,
}

#[derive(Default)]
pub struct Sessions {
    identity_undo: Option<super::IdentityUndo>,
    /// The DM showed or hid the party bar. Otherwise it shows while a session is played.
    pub party_bar: Option<bool>,
    /// The party has members, and the bar does not show, so the header offers it.
    pub party_hidden: bool,
    pub run_open: bool,
    campaign: Option<Campaign>,
    folder: Option<PathBuf>,
    folder_error: Option<String>,
    /// The current folder's files as `folder_messages` scanned them this frame, with the scan's
    /// error, for the form drawn after it: one scan a frame, as before the footer.
    folder_files: Option<FolderFiles>,
    /// Review of PR 274, item 1: the plan row the DM just added or acted on, until the row
    /// after it is in view above the footer (or a few draws pass), the draws it took, and the
    /// pass it was last drawn in.
    pub(super) row_reveal: Option<(String, u8, u64)>,
    /// When the link update's result was made (`timeline::next_result_order`).
    link_order: u64,
    /// When each kept result in this page's footers was made, so live Undo results go newest
    /// first whatever their kind (review of PR 274, item 2).
    pub(super) result_orders: crate::timeline::page::ResultOrder,
    folder_text: String,
    folder_notice: Option<String>,
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
    row_notice: Option<(Option<Uuid>, usize)>,
    updated_links: Option<Vec<LinkWrite>>,
    link_undo_state: Option<LinkUndoState>,
    link_problem: Option<String>,
    /// A link update that could not be put back whole: some notes hold the new links. Its Undo
    /// and its error stay through the usual expiry until Undo puts them back or a new update.
    link_partial: bool,
    removed_fight: Option<(Option<Uuid>, usize, String, NoteUndo)>,
    /// A journey's encounter checks in the plan.
    travel: plan::Travel,
    plan_notice_state: Option<(String, Vec<timeline::Event>)>,
    removed_event_in_plan: bool,
    /// What the removal's owner note looked like; a later edit of it ends the removal.
    removal_note: Option<RemovalNote>,
    /// How often the owner's note was looked for, which only a change may cause.
    #[cfg(test)]
    removal_scans: usize,
    removed_group: Option<(Option<Uuid>, usize, String, String, f32, String)>,
    /// The plan's scenes: the Scene field and the results of scene acts.
    scenes: plan::Scenes,
    /// The rewards record (F7): each `Add rewards` result by its note and fight.
    rewards: plan::Rewards,
    /// The note's plan as rows, built when the note, the content or the encounters change.
    plan_model: plan::PlanModelCache,
    /// The fight behind the open event, read once.
    fight: Option<(Uuid, Result<FightSummary, String>)>,
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
    new_template: Option<String>,
    template_error: Option<String>,
    deleted_template: Option<(String, String)>,
    recordings: Vec<Recording>,
    live: Option<Live>,
    confirm_record: bool,
    notice: Option<(Tone, String)>,
    /// A copy of a notice that arrived on its own (processing finished). While `notice` still
    /// equals it, the notice sits in the bottom banner; any later notice replaces it and keeps
    /// its own place above the tabs, as the result of the DM's action.
    arrived_notice: Option<(Tone, String)>,
    /// The session a notice is about. None means whichever is open now.
    notice_session: Option<Uuid>,
    /// A job ended while the DM may be elsewhere.
    job_done: bool,
    /// The latest result stays after its completion notice is acknowledged.
    processed_session: Option<Uuid>,
    /// The recording that result came from, so deleting it clears the result.
    processed_recording: Option<PathBuf>,
    /// The latest result has something the DM must look at.
    job_warned: bool,
    alerts: Vec<Alert>,
    added: Option<Added>,
    recording_added: Option<String>,
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
    note_editor_id: Option<egui::Id>,
    move_run_here: Option<(Uuid, String)>,
    moved_run: Option<MovedRun>,
    /// When a recording last stopped, so the rail can say it was kept.
    saved_at: Option<Instant>,
    note_had_focus: bool,
    /// The caret to bring into view, on the frame after the editor has laid out.
    caret_to_show: Option<usize>,
    note_scroll_to_top: bool,
    last_check: Option<Instant>,
    last_scan: Option<Instant>,
    /// The campaign's manifest changed, so what holds a copy of it must reload.
    pub manifest_changed: Option<PathBuf>,
    /// The Sessions page is showing, so a button there need not lead here.
    pub on_page: bool,
    pub wants_processing_settings: bool,
    pub wants_speech_key: bool,
    pub wants_recording_input: bool,
    /// The DM asked to see the session this is about.
    pub wants_sessions_page: bool,
    pub wants_event: Option<String>,
    /// The encounter to start, by its path.
    pub wants_fight: Option<String>,
    pub wants_campaign_link: bool,
    pub wants_runs_refresh: bool,
    pub wants_new_fight: bool,
    pub wants_new_group: bool,
    pub wants_group_undo: bool,
    /// The session whose plan a group is being assembled in. It is the one that gets the link.
    group_session: Option<Uuid>,
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
    /// The `/` menu in the note, and what its command has open.
    slash: super::slash::Slash,
    /// A command wrote a file, so Lookup must read the campaign again.
    pub wants_content_refresh: bool,
}

struct MovedRun {
    before: timeline::Event,
    open_run: Option<(String, Uuid)>,
    body: String,
    events: Vec<timeline::Event>,
    words: String,
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
fn page_column(ui: &mut egui::Ui, width: f32, height: f32, add: impl FnOnce(&mut egui::Ui)) {
    ui.allocate_ui_with_layout(
        egui::vec2(width, height),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_width(width);
            ui.set_min_height(height);
            add(ui);
        },
    );
}

/// A note's words as a reader sees them: links as words, and no empty headings.
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
    let lines: Vec<_> = plan
        .lines()
        .filter(|line| !line.trim().is_empty())
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
/// a line of their own, and with no words only that break goes in.
fn insert_at_caret(
    ctx: &egui::Context,
    id: egui::Id,
    draft: &mut Draft,
    words: &str,
    new_line: bool,
) {
    let mut state = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    let Some(range) = state.cursor.char_range() else {
        if !words.is_empty() {
            insert_in_note(&mut draft.body, words);
            draft.edited();
        }
        return;
    };
    let at = range.primary.index;
    let chars: Vec<char> = draft.body.chars().collect();
    let ends_words = at > 0
        && chars.get(at - 1).is_some_and(|c| *c != '\n')
        && chars.get(at).is_none_or(|c| *c == '\n');
    let added = if new_line && ends_words {
        format!("\n\n{words}")
    } else {
        words.to_owned()
    };
    if added.is_empty() {
        return;
    }
    let byte = draft
        .body
        .char_indices()
        .nth(at)
        .map_or(draft.body.len(), |(byte, _)| byte);
    draft.body.insert_str(byte, &added);
    draft.edited();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::one(
            egui::text::CCursor::new(at + added.chars().count()),
        )));
    egui::TextEdit::store_state(ctx, id, state);
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
    state: Option<Status<'_>>,
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
            ui.painter()
                .galley(egui::pos2(x, top + first), second, c.muted);
        },
    )
}

fn session_detail(
    ui: &egui::Ui,
    file: Option<&str>,
    detail: &str,
    state: Option<Status<'_>>,
    width: f32,
) -> std::sync::Arc<egui::Galley> {
    // Summary parts stay whole. A file name is words: it wraps at a space, and its next line
    // starts where the first did.
    let detail = file
        .into_iter()
        .map(str::to_owned)
        .chain(detail.lines().map(plan_summary))
        .collect::<Vec<_>>()
        .join("\n");
    let word = state.map(|state| {
        (
            plan_summary(&format!("{} {}", state.icon, state.word)),
            state.color,
        )
    });
    design::status_in_text(
        ui,
        &detail,
        word.as_ref().map(|(word, color)| (word.as_str(), *color)),
        width,
    )
}

/// After Discard, a note with no file left: the editor shows its last saved words.
const DISCARDED_GONE: &str = "This file is gone from the folder. The editor shows its last saved words. Save them as a new file to keep them.";

/// A missing note's words. A known new place says why it is offered; with none, the file is
/// only gone.
fn missing_note_words(moved_to: Option<String>) -> String {
    moved_to.map_or_else(
        || "This file is gone from the folder. Your draft is still here. Choose where to save it, or discard it.".to_owned(),
        |path| format!("This note was moved or renamed. It is now {path}. Your draft is still here."),
    )
}

/// A draft's own result (Discard or Reload, with its Undo) in a pane's message footer. The
/// draft owns it until an edit or the file's next change; `undo` says the DM pressed its Undo.
fn draft_result_in<'a>(undos: &mut LiveUndos<'a>, draft: &Draft, undo: &'a std::cell::Cell<bool>) {
    // The drop's own order, taken when the DM dropped the words, every time.
    let order = || draft.dropped_order;
    match draft.dropped {
        Some(Dropped::Reloaded(_)) => {
            undos.push(order(), "Reloaded from disk.".to_owned(), move |ui| {
                if design::reloaded_draft(ui) {
                    undo.set(true);
                }
            });
        }
        Some(Dropped::Discarded(_)) => {
            undos.push(order(), "Draft discarded.".to_owned(), move |ui| {
                if design::undo_message(ui, Tone::Info, "Draft discarded.", "Undo discard") {
                    undo.set(true);
                }
            });
        }
        None => {}
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
        #[cfg(test)]
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
                    (Some(before), Some(draft)) if *before != draft.body => {
                        book.forget_removed();
                        self.removal_note = None;
                        return;
                    }
                    (None, Some(draft)) => seen.words = Some(draft.body.clone()),
                    _ => {}
                }
                seen.revision = revision;
            }
            None => {
                self.removal_note = Some(RemovalNote {
                    id,
                    revision,
                    words: words.map(|draft| draft.body.clone()),
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
        if self.just_saved() {
            ctx.request_repaint_after(SAVED_WORD_FOR);
        }
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
                #[cfg(not(test))]
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
        book.poll(ctx);
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
        self.notice.is_some() && self.notice == self.arrived_notice
    }

    /// The session's own banner notices are in view: the notes, or the Notes panel.
    fn notes_in_view(&self) -> bool {
        (self.on_page && self.view == View::Notes && self.form.is_none()) || self.notes_open
    }

    /// Round 26 FM7: the session's own Recordings tab shows the processed row, so the DM has
    /// read the `is processed` notice there. Its banner does not show, and the view ends it.
    fn processed_notice_read_here(&self) -> bool {
        self.on_page
            && self.view == View::Notes
            && self.form.is_none()
            && self.tab == Tab::Recordings
            && self.notice_arrived()
            && self.notice_session == self.selected
    }

    fn has_banner_notices(&self) -> bool {
        let notes = self.on_page && self.view == View::Notes && self.form.is_none();
        let selected = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id));
        (notes
            && self.notice_arrived()
            && !self.processed_notice_read_here()
            && (self.notice_session.is_none() || self.notice_session == self.selected))
            || (notes && selected.is_some_and(|note| !note.also.is_empty()))
            || (notes && !self.unreadable.is_empty())
            || (self.notes_in_view() && self.draft.as_ref().is_some_and(Draft::needs_attention))
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
    fn banner_notices(&mut self, ui: &mut egui::Ui) {
        let notes = self.on_page && self.view == View::Notes && self.form.is_none();
        let width = ui.available_width().min(design::LIST_MEASURE);
        if notes
            && self.notice_arrived()
            && !self.processed_notice_read_here()
            && (self.notice_session.is_none() || self.notice_session == self.selected)
            && let Some((tone, words)) = self.notice.clone()
        {
            ui.horizontal_top(|ui| {
                let room = width - Density::Read.control() - design::S;
                ui.allocate_ui_with_layout(
                    egui::vec2(room, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| design::message(ui, tone, &words),
                );
                if design::icon_button(ui, icons::CLOSE, "Dismiss").clicked() {
                    self.notice = None;
                    self.notice_session = None;
                }
            });
        }
        if notes
            && let Some(note) = self
                .selected
                .and_then(|id| self.notes.iter().find(|note| note.id == id))
                .cloned()
        {
            for copy in &note.also {
                let filename = copy.file_name().unwrap_or_default().to_string_lossy();
                if design::message_with_action(
                    ui,
                    Tone::Warning,
                    &format!(
                        "{} and {filename} claim to be the same session.",
                        note.path.file_name().unwrap_or_default().to_string_lossy()
                    ),
                    &format!("Make {filename} a separate session"),
                ) {
                    match super::new_identity(copy) {
                        Ok(undo) => {
                            self.result_orders.renew("identity");
                            self.identity_undo = Some(undo);
                            self.refresh();
                        }
                        Err(error) => self.notice = Some((Tone::Error, error)),
                    }
                }
            }
        }
        if notes && !self.unreadable.is_empty() {
            design::message(
                ui,
                Tone::Warning,
                &format!(
                    "Splat cannot read the properties of {}, so it is not listed. The file is safe. Repair it to bring it back.",
                    self.unreadable.join(", ")
                ),
            );
        }
        if self.notes_in_view()
            && let Some(draft) = self.draft.as_mut().filter(|draft| draft.needs_attention())
        {
            Self::draft_warning(ui, draft);
        }
        if self.on_page
            && self.view == View::Templates
            && let Some(draft) = self
                .template_draft
                .as_mut()
                .filter(|draft| draft.needs_attention())
        {
            Self::draft_warning(ui, draft);
        }
    }

    fn banner_has_primary(&self, settings: &ProcessingSettings, run_open: bool) -> bool {
        self.banner_primary_owner(settings, run_open).is_some()
    }

    /// What cannot wait, on every page. It sits at the bottom, so that it never moves `Next
    /// turn` under a pointer parked on it. A silence takes no banner: it is often only a quiet
    /// table, and it shows in the rail and on the run's own line.
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
                egui::ScrollArea::vertical()
                    .id_salt("session_banner_scroll")
                    .max_height(most)
                    // Short content takes its own height; long content takes the whole third.
                    .min_scrolled_height(most)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        if let Some(words) = &stalled {
                            let session = self.live.as_ref().map(|live| live.session);
                            // The choices start at the message text, as every message's choices do.
                            design::message_with_actions(ui, Tone::Error, words, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    if (if primary_owner == Some(usize::MAX) {
                                        Action::primary("Record again")
                                    } else {
                                        Action::secondary("Record again")
                                    })
                                    .show(ui)
                                    .clicked()
                                    {
                                        record_again = session;
                                    }
                                    let here = self.on_page && self.selected == session;
                                    if !here
                                        && Action::secondary("Open the session").show(ui).clicked()
                                    {
                                        open = session;
                                    }
                                    ui.add_space(design::L - ui.spacing().item_spacing.x);
                                    stop = Action::secondary("Stop recording")
                                        .name("Stop recording")
                                        .show(ui)
                                        .clicked();
                                });
                            });
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
                            let retry_blocked = if audio_gone {
                                Some("Audio removed. Restore the WAV file to process it.")
                            } else if self.job.is_some() || self.live.is_some() {
                                Some("Stop recording or wait for processing to finish.")
                            } else {
                                None
                            };
                            // The choices start at the message text, and wrap there too. The words keep
                            // their hover: what the recorder itself said. Dismiss is the close icon at the
                            // message end, as every dismiss is.
                            let top = ui.cursor().min;
                            let mut words_end = top.y;
                            let here = self.on_page
                                && self.selected == Some(alert.session)
                                && self.tab == Tab::Recordings;
                            let can_record = alert.can_record && self.live.is_none();
                            let has_choices =
                                missing_input || alert.retry.is_some() || can_record || !here;
                            let message_width = ui.available_width().min(design::LIST_MEASURE);
                            ui.horizontal_top(|ui| {
                                let room = message_width - Density::Read.control() - design::S;
                                ui.allocate_ui_with_layout(
                                    egui::vec2(room, 0.0),
                                    egui::Layout::top_down(egui::Align::Min),
                                    |ui| {
                                        if !has_choices {
                                            design::message(ui, Tone::Error, &words);
                                            words_end = ui.min_rect().bottom();
                                            return;
                                        }
                                        design::message_with_actions(
                                            ui,
                                            Tone::Error,
                                            &words,
                                            |ui| {
                                                words_end = ui.min_rect().top()
                                                    - ui.spacing().item_spacing.y;
                                                ui.horizontal_wrapped(|ui| {
                                                    if missing_input
                                                        && (if banner_primary {
                                                            Action::primary("Choose an input")
                                                        } else {
                                                            Action::secondary("Choose an input")
                                                        })
                                                        .show(ui)
                                                        .clicked()
                                                    {
                                                        self.wants_recording_input = true;
                                                    }
                                                    if let Some(path) = &alert.retry
                                                        && (if banner_primary {
                                                            Action::primary("Continue processing")
                                                        } else {
                                                            Action::secondary("Continue processing")
                                                        })
                                                        .blocked(retry_blocked)
                                                        .show(ui)
                                                        .clicked()
                                                    {
                                                        retry = Some((alert.session, path.clone()));
                                                    }
                                                    // One blocked control says why beside it, then `L` to the next.
                                                    let mut gap = false;
                                                    if alert.retry.is_some()
                                                        && retry_blocked.is_some()
                                                    {
                                                        design::blocked_reason(ui, retry_blocked);
                                                        gap = true;
                                                    }
                                                    let mut gap_before = |ui: &mut egui::Ui| {
                                                        if std::mem::take(&mut gap) {
                                                            ui.add_space(
                                                                design::L
                                                                    - ui.spacing().item_spacing.x,
                                                            );
                                                        }
                                                    };
                                                    if can_record {
                                                        gap_before(ui);
                                                        // With no input, Choose an input is the one primary.
                                                        if (if banner_primary && !missing_input {
                                                            Action::primary("Record again")
                                                        } else {
                                                            Action::secondary("Record again")
                                                        })
                                                        .show(ui)
                                                        .clicked()
                                                        {
                                                            record_again = Some(alert.session);
                                                        }
                                                    }
                                                    if !here {
                                                        gap_before(ui);
                                                        if Action::secondary("Open the session")
                                                            .show(ui)
                                                            .clicked()
                                                        {
                                                            open = Some(alert.session);
                                                        }
                                                    }
                                                });
                                            },
                                        );
                                    },
                                );
                                if design::icon_button(ui, icons::CLOSE, "Dismiss this error")
                                    .clicked()
                                {
                                    dismiss = Some(index);
                                }
                            });
                            if let Some(said) = &alert.said {
                                let width = message_width - Density::Read.control() - design::S;
                                let words_rect = egui::Rect::from_min_max(
                                    top,
                                    egui::pos2(top.x + width, words_end),
                                );
                                ui.interact(
                                    words_rect,
                                    ui.id().with(("alert_said", index)),
                                    egui::Sense::hover(),
                                )
                                .on_hover_text(said);
                            }
                        }
                        if let Some(index) = dismiss {
                            self.alerts.remove(index);
                        }
                        // After the recording alerts, in the banner's order: what arrived on its own.
                        self.banner_notices(ui);
                        // A control that gains keyboard focus scrolls into view.
                        let focus = ui.ctx().memory(|memory| memory.focused());
                        let seen_id = egui::Id::new("session_banner_focus");
                        let seen: Option<egui::Id> =
                            ui.data(|data| data.get_temp(seen_id)).flatten();
                        if focus != seen {
                            if let Some(response) = focus.and_then(|id| ui.ctx().read_response(id))
                                && response.layer_id == ui.layer_id()
                                && ui.min_rect().contains_rect(response.rect)
                            {
                                ui.scroll_to_rect(response.rect, None);
                            }
                            ui.data_mut(|data| data.insert_temp(seen_id, focus));
                        }
                    });
            });
        if let Some((session, path)) = retry {
            self.process_recording(ctx, &path, settings, book, session);
        }
        if stop {
            self.finish_recording();
        }
        if let Some(session) = record_again {
            self.finish_recording();
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
        if self.form.is_some() {
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
        self.notice_session = Some(session);
        match result {
            Ok(None) => {
                self.processed_session = Some(session);
                self.processed_recording = failed_recording.clone();
                self.job_done = true;
                self.job_warned = false;
                self.notice = Some((
                    Tone::Success,
                    format!(
                        "“{label}” is processed. Read the result, and add the events you want to the timeline."
                    ),
                ));
                self.arrived_notice.clone_from(&self.notice);
            }
            Ok(Some(warning)) => {
                self.processed_session = Some(session);
                self.processed_recording = failed_recording.clone();
                self.job_done = true;
                self.job_warned = true;
                // The recording's row has the words, for as long as the result stands.
                let _ = warning;
                self.notice = None;
            }
            Err(error) if error == processing::CANCELLED => {
                self.notice = None;
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
                self.notice = None;
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
        self.identity_undo = None;
        // A live recording blocks a campaign switch, so none is lost here.
        self.draft = None;
        self.template_draft = None;
        self.selected = None;
        self.template = None;
        self.reading = None;
        self.form = None;
        self.notice = None;
        self.deleted_run = None;
        self.row_notice = None;
        self.removed_group = None;
        self.plan_notice_state = None;
        self.removed_event_in_plan = false;
        self.updated_links = None;
        self.link_undo_state = None;
        self.removed_fight = None;
        self.scenes = plan::Scenes::default();
        self.rewards = plan::Rewards::default();
        self.campaign = active.cloned();
        self.folder_error = None;
        match active {
            Some(campaign) => {
                self.folder_text =
                    folder_setting(campaign).unwrap_or_else(|_| DEFAULT_FOLDER.to_owned());
                match super::open_folder(campaign) {
                    Ok(folder) => self.folder = Some(folder),
                    Err(error) => {
                        self.folder = None;
                        self.folder_error = Some(error);
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
        self.deleted_template = None;
        self.folder_notice = None;
        self.notice_session = None;
        self.select(self.current().or(self.notes.first().map(|note| note.id)));
    }

    /// The session the DM chose to play, when they chose.
    pub fn chosen_session(&self) -> Option<Option<Uuid>> {
        self.events_to
    }

    pub fn refresh(&mut self) {
        let Some(folder) = &self.folder else {
            self.notes.clear();
            self.mentions.clear();
            self.templates.clear();
            return;
        };
        if !folder.is_dir() {
            self.folder_error = Some(super::missing_folder("Session", folder));
            return;
        }
        self.folder_error = None;
        self.notes = list_notes(folder);
        // A new, renamed or removed note can change which titles are shared.
        self.last_time = None;
        self.carries.scanned();
        if let Err(error) = super::remember_notes(folder, &self.notes) {
            self.folder_error = Some(error);
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
        self.folder_error
            .as_ref()
            .is_some_and(|error| error.starts_with("Session folder not found"))
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
        self.selected = session;
        self.caret_to_show = None;
        self.note_scroll_to_top = true;
        self.draft = session
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
            .and_then(|note| Draft::open(&note.path).ok());
        self.reading = None;
        self.added = None;
        self.open_event = None;
        self.fight = None;
        self.reading_note = session != self.current();
        if self.notice_session.is_none() {
            self.notice = None;
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
                self.notice_session = None;
                self.notice = Some((Tone::Error, error));
            }
        }
    }

    /// The words for working in a session other than the one being played.
    fn not_being_played(&self) -> Option<String> {
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
        Some(
            if let Some(played) = played.filter(|played| played.number == note.number) {
                let (open, played) = session_warning_names(note, played);
                format!(
                    "{open} is not the session being played. The session being played is {played}. What you write and add on this page goes into {destination}."
                )
            } else {
                format!(
                    "Session {} is not the session being played. What you write and add on this page goes into {destination}.",
                    note.number
                )
            },
        )
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
            .or(self.notice_session)
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
        let old: Vec<&str> = renamed.iter().map(String::as_str).collect();
        let (words, action) =
            crate::content_workspace::link_offer_words(&old, &current.title, count);
        (count > 0).then(|| LinkOffer {
            words,
            action,
            changes,
            title: current.title.clone(),
        })
    }

    /// The DM took the link offer: write the notes, and keep what Undo needs.
    pub fn accept_event_link_offer(&mut self, offer: LinkOffer, book: &Book) {
        self.update_event_link_notes(offer.changes);
        self.link_order = crate::timeline::next_result_order();
        self.link_undo_state = Some(LinkUndoState {
            notes: self.note_bodies(),
            events: book.events.clone(),
            title: offer.title,
        });
    }

    /// The link update's result, while its Undo is live: the Library's words for the same
    /// action, the name the links now say.
    pub fn event_link_result(&self) -> Option<String> {
        self.updated_links.as_ref()?;
        self.link_undo_state
            .as_ref()
            .map(|state| format!("Links now say “{}”.", state.title))
    }

    pub fn undo_event_link_update(&mut self) {
        self.undo_event_link_notes();
    }

    /// A reveal that waits for its next draw, as one does just after an add or a travel act.
    #[cfg(test)]
    pub fn simulate_pending_reveal(&mut self, row: &str, pass: u64) {
        self.row_reveal = Some((row.to_owned(), 0, pass));
    }

    /// When the link update's result was made, for the footer's newest-first order.
    pub fn event_link_order(&self) -> u64 {
        self.link_order
    }

    /// Why the last link update or its Undo did not finish.
    pub fn event_link_problem(&self) -> Option<&str> {
        self.link_problem.as_deref()
    }

    fn note_bodies(&self) -> Vec<(PathBuf, String)> {
        self.notes
            .iter()
            .filter_map(|note| {
                #[cfg(test)]
                NOTE_READS.with(|reads| reads.set(reads.get() + 1));
                let body = self
                    .draft
                    .as_ref()
                    .filter(|draft| draft.path == note.path)
                    .map(|draft| draft.body.clone())
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
        // A partial update keeps its result, its error and its Undo until Undo or a new update,
        // and reads no note meanwhile: Undo checks its notes when the DM presses it.
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
            self.link_undo_state = None;
            self.link_problem = None;
        }
    }

    fn update_event_link_notes(&mut self, changes: Vec<(Draft, String)>) {
        // A new update ends the last one's result: its warning, its Undo and its hold.
        self.link_problem = None;
        self.updated_links = None;
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
            next.body = body;
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
            let bytes = next.file_text().unwrap_or_default().to_owned();
            written.push(LinkWrite {
                id: super::note_identity(&next.path, &bytes),
                before,
                after: next.body.clone(),
                bytes,
            });
            if self
                .draft
                .as_ref()
                .is_some_and(|draft| draft.path == next.path)
            {
                self.draft = Some(next.clone());
            }
            #[cfg(test)]
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

    fn undo_event_link_notes(&mut self) {
        let mut remaining = Vec::new();
        let mut refused = Vec::new();
        for write in self.updated_links.take().unwrap_or_default() {
            // Undo writes only into the file it wrote: the draft it saves through must hold the
            // same bytes and the same session, and `save` checks the file against them again
            // right before writing.
            #[cfg(test)]
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
            match current.filter(|draft| {
                same_file(draft) && draft.body == write.after && !draft.needs_attention()
            }) {
                Some(mut current) => {
                    current.body = write.before.body.clone();
                    current.edited();
                    current.save();
                    let undone = !current.unsaved();
                    if self
                        .draft
                        .as_ref()
                        .is_some_and(|draft| draft.path == current.path)
                    {
                        self.draft = Some(current);
                    }
                    if !undone {
                        refused.push(write.before.path.clone());
                        remaining.push(write);
                    }
                }
                None => {
                    refused.push(write.before.path.clone());
                    remaining.push(write);
                }
            }
        }
        if refused.is_empty() {
            if self.link_partial {
                self.link_problem = None;
            }
            self.link_partial = false;
        } else {
            let names: Vec<String> = refused
                .iter()
                .map(|path| {
                    path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned()
                })
                .collect();
            self.link_problem = Some(format!(
                "Undo did not change {}: it changed after its links were updated. Keep or resolve that edit first.",
                names.join(", ")
            ));
        }
        self.updated_links = (!remaining.is_empty()).then_some(remaining);
        self.last_time = None;
    }

    pub fn recording(&self) -> bool {
        self.live.is_some()
    }

    pub fn rail_recording_status(&self) -> Option<Status<'static>> {
        let c = colors();
        if let Some((icon, word, color)) = self.recording_status() {
            return Some(Status { icon, word, color });
        }
        if self.processing() {
            return Some(Status {
                icon: icons::AUDIO,
                word: "Processing",
                color: c.connection.busy,
            });
        }
        if let Some(word) = self.alert_words().last() {
            return Some(Status {
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
            });
        }
        if self.job_warned() {
            return Some(Status {
                word: "Processed",
                ..processed_warning()
            });
        }
        if self.processed_session.is_some() {
            return Some(processed_clean());
        }
        self.just_saved().then_some(Status {
            icon: icons::OK,
            word: "Recording saved",
            color: c.document.saved,
        })
    }

    /// The same timed recording words on the rail and the session page.
    pub fn recording_detail(&self) -> Option<String> {
        let Some(live) = self.live.as_ref() else {
            return self
                .rail_recording_status()
                .map(|status| status.word.to_owned());
        };
        let (_, word, _) = self.recording_status()?;
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
    pub fn recording_status(&self) -> Option<(&'static str, &'static str, Color32)> {
        let c = colors();
        self.live.as_ref().map(|live| match live.recorder.health() {
            Health::Stalled | Health::Stopped(_) => {
                (icons::ERROR, "Not saving audio", c.result.error)
            }
            Health::Silent => (icons::WARN, "Recording silence", c.result.warning),
            Health::Starting | Health::Hearing => (icons::RECORD, "Recording", c.recording),
        })
    }

    /// A word for the run's own notice line. It takes no room of its own, so nothing moves.
    pub fn run_notice(&self) -> Option<&'static str> {
        self.live
            .as_ref()
            .filter(|live| *live.recorder.health() == Health::Silent)
            .map(|_| "Recording silence. Check the input in Settings.")
    }

    /// A recording stopped a moment ago, and its file is kept.
    pub fn just_saved(&self) -> bool {
        self.saved_at
            .is_some_and(|at| at.elapsed() < SAVED_WORD_FOR)
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

    fn finish_recording(&mut self) {
        self.stop_recorder(true);
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
        if tell {
            self.notice_session = Some(session);
            self.notice = Some(if has_audio {
                self.saved_at = Some(Instant::now());
                (
                    Tone::Success,
                    format!("Recording saved: {}.", recording_title(&path)),
                )
            } else {
                (
                    Tone::Warning,
                    format!("Nothing was recorded for “{label}”."),
                )
            });
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
                self.notice = None;
                self.notice_session = None;
                // A new recording answers an old failure of this session.
                self.alerts
                    .retain(|alert| !(alert.can_record && alert.session == session));
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
            self.notice = Some((
                Tone::Warning,
                "Audio removed. Restore the WAV file to process it.".into(),
            ));
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
            .map(|(event, day)| format!("- {}", event.label(*day)))
            .collect();
        let context = format!(
            "{note}\n\n{}\n\nEvents the DM already recorded:\n{}",
            if self.selected == Some(session) {
                self.draft
                    .as_ref()
                    .map_or_else(String::new, |draft| draft.body.clone())
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
        self.notice = None;
        self.notice_session = None;
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
        // Stop is here whichever session is open. It can be urgent, if a player asks.
        if self.live.as_ref().is_some_and(|live| {
            matches!(live.recorder.health(), Health::Stalled | Health::Stopped(_))
        }) {
            return;
        }
        if let Some(live) = &self.live {
            if self.tab == Tab::Recordings
                && self.reading.is_none()
                && self.selected == Some(live.session)
                && !self.recordings.is_empty()
            {
                return;
            }
            let name = if Some(live.session) == self.selected {
                "Stop recording".to_owned()
            } else {
                format!("Stop recording of {}", live.label)
            };
            if Action::secondary("Stop recording")
                .icon(icons::STOP_RECORDING)
                .name(&name)
                .hint(&name)
                .show(ui)
                .clicked()
            {
                self.finish_recording();
            }
            return;
        }
        if self.confirm_record
            || (self.tab == Tab::Recordings
                && self.reading.is_none()
                && self.recordings.iter().any(|recording| {
                    recording
                        .status
                        .as_deref()
                        .is_some_and(|words| words.starts_with("Stopped by itself:"))
                }))
        {
            return;
        }
        let reason = self.record_blocked(settings);
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
            ui.add_space(design::L);
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
        let top = ui.cursor().top();
        self.record_line_body(ui, settings);
        if live && !self.confirm_record {
            let used = ui.cursor().top() - top;
            ui.add_space((Density::Read.control() + design::XS - used).max(0.0));
        }
    }

    fn record_line_body(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings) {
        let c = colors();
        if let Some(live) = self
            .live
            .as_ref()
            .filter(|live| Some(live.session) == self.selected)
        {
            let time = clock(live.recorder.seconds());
            match live.recorder.health() {
                Health::Stalled | Health::Stopped(_) => {}
                health => {
                    let (word, color) = match health {
                        Health::Starting => ("Waiting for sound", c.muted),
                        Health::Silent => ("Silent for over a minute", c.muted),
                        // Only what is true now. A table goes quiet, and that is no warning.
                        _ if live.recorder.heard_just_now() => ("Hearing sound", c.muted),
                        _ => ("Quiet", c.muted),
                    };
                    ui.horizontal_wrapped(|ui| {
                        // The first thing this page must say, large enough for a glance.
                        ui.label(
                            Type::Strong
                                .rich(
                                    ui,
                                    format!(
                                        "{} {}",
                                        if *health == Health::Silent {
                                            icons::WARN
                                        } else {
                                            icons::RECORD
                                        },
                                        if *health == Health::Silent {
                                            "Recording silence".to_owned()
                                        } else {
                                            format!("Recording {time}")
                                        }
                                    ),
                                )
                                .color(if *health == Health::Silent {
                                    c.result.warning
                                } else {
                                    c.recording
                                }),
                        );
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
                Tone::Warning
            };
            // The choices start at the message text.
            design::message_with_actions(ui, tone, &words, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if dated.is_some()
                        && self.selected == self.current()
                        && Action::secondary("Play this one").show(ui).clicked()
                    {
                        self.play_this_one();
                    }
                    if (if self.run_open {
                        Action::secondary("Start recording")
                    } else {
                        Action::primary("Start recording")
                    })
                    .show(ui)
                    .clicked()
                    {
                        self.confirm_record = false;
                        if let Some(session) = self.selected
                            && let Err(error) = self.start_recording(settings, session)
                        {
                            self.notice_session = None;
                            self.notice = Some((Tone::Error, error));
                        }
                    }
                    if Action::secondary("Not now").show(ui).clicked() {
                        self.confirm_record = false;
                    }
                });
            });
        }
    }

    /// A note's state says it is the note, beside a control about audio.
    fn note_save_status(draft: &Draft) -> Status<'static> {
        let c = colors();
        let (icon, word, color) = match &draft.state {
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

    fn draft_status(ui: &mut egui::Ui, draft: &Draft) {
        let c = colors();
        let (icon, word, color) = match &draft.state {
            DraftState::Clean => (icons::OK, "Saved", c.document.clean),
            DraftState::Saved => (icons::OK, "Saved", c.document.saved),
            DraftState::Unsaved => (icons::DRAFT, "Unsaved", c.document.unsaved),
            DraftState::Conflict => (icons::WARN, "Changed on disk", c.document.external),
            DraftState::Missing => (icons::WARN, "File gone", c.document.external),
            DraftState::Failed(_) => (icons::ERROR, "Not saved", c.document.invalid),
        };
        design::status(ui, Status { icon, word, color });
    }

    /// The message footer of a pane whose body is one draft's editor: the draft's own result
    /// with its Undo, and for a template, the template action's error. Returns where the body
    /// ends.
    fn draft_footer(&mut self, ui: &mut egui::Ui, id: &str, template: bool) -> f32 {
        let undo = std::cell::Cell::new(false);
        let mut footer = design::Footer::new((id, self.selected), ui.available_width());
        let draft = if template {
            self.template_draft.as_ref()
        } else {
            self.draft.as_ref()
        };
        if let Some(draft) = draft {
            let mut undos = LiveUndos::new();
            draft_result_in(&mut undos, draft, &undo);
            undos.into_footer(&mut footer);
        }
        if template && let Some(error) = self.template_error.clone() {
            footer.reason(error.clone(), move |ui| {
                design::message(ui, Tone::Error, &error);
            });
        }
        let bottom = footer.show(ui);
        let draft = if template {
            self.template_draft.as_mut()
        } else {
            self.draft.as_mut()
        };
        if undo.get()
            && let Some(draft) = draft
        {
            draft.undo_dropped();
        }
        bottom
    }

    /// What the file did to a draft: changed on disk, gone or moved, or not saved. It arrives on
    /// its own, so it is drawn in the bottom banner, never above the editor.
    fn draft_warning(ui: &mut egui::Ui, draft: &mut Draft) {
        // A new file could not be made: why, above the choices that are still open.
        if draft.state == DraftState::Missing
            && let Some(error) = &draft.save_again_error
        {
            design::message(ui, Tone::Error, error);
        }
        // Every choice below starts at its message's text. The red choice keeps an `L` gap.
        match draft.state.clone() {
            DraftState::Conflict => {
                design::message_with_actions(
                    ui,
                    Tone::Warning,
                    "This note changed on disk. Your draft is safe until you choose. Other sessions open after that.",
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            if Action::secondary("Overwrite with my draft")
                                .show(ui)
                                .clicked()
                            {
                                draft.overwrite();
                            }
                            ui.add_space(design::L - ui.spacing().item_spacing.x);
                            if Action::destructive("Reload and lose my draft")
                                .show(ui)
                                .clicked()
                                && draft.reload_keeping_undo().is_err()
                            {
                                draft.state = DraftState::Missing;
                            }
                        });
                    },
                );
            }
            // After Discard with no file left, the editor holds the last saved words, not a
            // draft: the one way on is a new file.
            DraftState::Missing if matches!(draft.dropped, Some(Dropped::Discarded(_))) => {
                let save = design::message_with_actions(ui, Tone::Warning, DISCARDED_GONE, |ui| {
                    Action::secondary("Save these words as a new file")
                        .show(ui)
                        .clicked()
                });
                if save {
                    draft.save_again();
                }
            }
            DraftState::Missing => {
                let words = missing_note_words(draft.moved_to.as_ref().map(|path| {
                    path.strip_prefix(&draft.search_folder)
                        .unwrap_or(path)
                        .display()
                        .to_string()
                }));
                design::message_with_actions(ui, Tone::Warning, &words, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.y = design::M;
                        if let Some(path) = &draft.moved_to {
                            let label = format!(
                                "Save my draft to {}",
                                path.strip_prefix(&draft.search_folder)
                                    .unwrap_or(path)
                                    .display()
                            );
                            if Action::secondary(&label).show(ui).clicked() {
                                draft.save_moved();
                            }
                        }
                        if Action::secondary("Save my draft as a new file")
                            .show(ui)
                            .clicked()
                        {
                            draft.save_again();
                        }
                        ui.add_space(design::L - ui.spacing().item_spacing.x);
                        if Action::destructive("Discard").show(ui).clicked() {
                            draft.discard();
                        }
                    });
                });
            }
            DraftState::Failed(error) => {
                let again = design::message_with_actions(ui, Tone::Error, &error, |ui| {
                    Action::secondary("Try to save again").show(ui).clicked()
                });
                if again {
                    draft.save();
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

    fn show_form(&mut self, ui: &mut egui::Ui) {
        let Some(mut form) = self.form.take() else {
            return;
        };
        let heading = if form.editing.is_some() {
            "Session details"
        } else {
            "New session"
        };
        let c = colors();
        design::group_heading(ui, heading, 0);
        ui.add_space(design::XS);
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
        let title = design::text_field(
            ui,
            "Session title",
            &mut form.title,
            Some("Optional. You can name it after you play."),
            None,
        );
        // The one field a DM usually types, often with people waiting.
        if std::mem::take(&mut self.focus_form_title) {
            title.request_focus();
        }
        let title_entered = title.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
        design::text_field(ui, "Date", &mut form.date, Some("As YYYY-MM-DD."), None);
        if form.editing.is_none() {
            design::field(ui, "Template", None, None, |ui| {
                let inner = egui::ComboBox::from_id_salt("session_template")
                    .selected_text(&form.template)
                    .width(ui.available_width().min(design::FORM_WIDTH))
                    .icon(design::combo_icon)
                    .show_ui(ui, |ui| {
                        for template in &self.templates {
                            ui.selectable_value(
                                &mut form.template,
                                template.name.clone(),
                                &template.name,
                            );
                        }
                    });
                design::named(ui, &inner.response, "Session template");
            });
        }
        if form.editing.is_none()
            && let Some(folder) = &self.folder
        {
            let path = super::note_path(folder, form.number.parse().unwrap_or(1), &form.title);
            let shown = path.strip_prefix(folder).unwrap_or(&path);
            wrapped_words(
                ui,
                Type::Small,
                format!("File: {}", shown.display()),
                c.muted,
            );
        }
        if let Some(error) = &form.error {
            design::message(ui, Tone::Error, error);
        }
        let mut done = title_entered;
        let mut cancel = false;
        ui.horizontal(|ui| {
            let label = if form.editing.is_some() {
                "Save details"
            } else {
                "Create session"
            };
            done |= if self.open_run.is_some() {
                Action::secondary(label)
            } else {
                Action::primary(label)
            }
            .show(ui)
            .clicked();
            cancel = Action::quiet("Cancel").show(ui).clicked();
        });
        if cancel {
            return;
        }
        if done {
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
                    return;
                }
                Err(error) => form.error = Some(error),
            }
        }
        self.form = Some(form);
    }

    /// The warning about an old session has gone, and the open run's event has not moved.
    fn say_where_the_run_stays(&mut self, new: Uuid) {
        self.moved_run = None;
        let Some((name, session)) = self.open_run.clone().filter(|(_, session)| *session != new)
        else {
            return;
        };
        let Some(note) = self.notes.iter().find(|note| note.id == session) else {
            return;
        };
        self.notice = Some((Tone::Info, format!("{name} stays in {}.", note.label())));
        self.notice_session = Some(new);
        self.move_run_here = self.notice.as_ref().map(|(_, words)| (new, words.clone()));
    }

    fn move_run_to_session(&mut self, book: &mut Book, run: Uuid, session: Uuid) {
        let Some(event) = book.events.iter_mut().find(|event| event.run == Some(run)) else {
            return;
        };
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
        let words = format!("{name} moved to Session {number}.");
        self.notice = Some((Tone::Success, words.clone()));
        self.notice_session = Some(session);
        self.result_orders.renew("move");
        self.moved_run = Some(MovedRun {
            before,
            open_run,
            body: self
                .draft
                .as_ref()
                .map_or_else(String::new, |draft| draft.body.clone()),
            events: book.events.clone(),
            words,
        });
        self.open_run = self.open_run.take().map(|(name, _)| (name, session));
        self.move_run_here = None;
    }

    fn expire_move(&mut self, book: &Book) {
        if self.moved_run.as_ref().is_some_and(|moved| {
            self.notice
                .as_ref()
                .is_none_or(|(_, words)| *words != moved.words)
        }) {
            self.moved_run = None;
        }
        if self.moved_run.as_ref().is_some_and(|moved| {
            moved.events != book.events
                || self
                    .draft
                    .as_ref()
                    .is_none_or(|draft| draft.body != moved.body)
        }) {
            self.moved_run = None;
            self.notice = None;
            self.notice_session = None;
        }
    }

    fn undo_move(&mut self, book: &mut Book) {
        self.expire_move(book);
        if let Some(moved) = self.moved_run.take() {
            if let Some(event) = book
                .events
                .iter_mut()
                .find(|event| event.id == moved.before.id)
            {
                *event = moved.before;
                book.touch();
            }
            self.open_run = moved.open_run;
            self.notice = None;
            self.notice_session = None;
        }
    }

    fn submit_form(&mut self, form: &Form) -> Result<Uuid, String> {
        let number: u32 = form
            .number
            .trim()
            .parse()
            .map_err(|_| "The number must be a whole number.".to_owned())?;
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
            return Err("Enter the date as YYYY-MM-DD.".to_owned());
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
        egui::ScrollArea::vertical()
            .id_salt("session_list")
            .auto_shrink([false, false])
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
                                    Status {
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
                                    }
                                })
                        };
                        // A warned result is a status like its siblings: icon, word and colour.
                        // The full words stay in hover and in the row's name.
                        // A clean result is a status too, in the success colour.
                        let warned = state.is_none() && result == Some(note.id) && self.job_warned;
                        let clean = state.is_none() && result == Some(note.id) && !warned;
                        let state = if warned {
                            Some(Status {
                                word: "Processed",
                                ..processed_warning()
                            })
                        } else if clean {
                            Some(processed_clean())
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
    fn note_editor(&mut self, ui: &mut egui::Ui, in_panel: bool) {
        let Some(draft) = &mut self.draft else {
            design::message(ui, Tone::Error, "This note could not be opened.");
            return;
        };
        // Beside a page with its own draft, Ctrl+S goes to where the keyboard is.
        if (!in_panel || self.note_had_focus)
            && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S))
        {
            draft.save();
        }
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
        if self.break_before_words
            && let Some(id) = self.note_editor_id
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
                insert_at_caret(ui.ctx(), id, draft, "", true);
            }
        }
        let heading_id = ui.id().with(("note_heading_offset", self.selected));
        let heading_offset = if in_panel {
            ui.data(|data| data.get_temp::<f32>(heading_id).unwrap_or(0.0))
        } else {
            0.0
        };
        let reset_scroll = !in_panel && std::mem::take(&mut self.note_scroll_to_top);
        let frame = egui::Frame::new()
            .fill(colors().well)
            .stroke(egui::Stroke::new(
                design::HAIRLINE_WIDTH,
                colors().field_border,
            ))
            .corner_radius(design::RADIUS_CONTROL)
            .inner_margin(design::S);
        self.slash.take_keys(ui, self.note_editor_id);
        let index = ui.ctx().data(|data| {
            data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                "reader_link_index",
            ))
        });
        let mut pressed_link = None;
        // ScrollArea derives its ID from its Ui. Give that Ui a global session ID,
        // so tab and side-by-side layouts share the same scroll state.
        ui.scope_builder(
            egui::UiBuilder::new().id(("session_editor_scope", self.selected, in_panel)),
            |ui| {
                ui.set_max_width(
                    ui.available_width()
                        .min(design::READER_MEASURE + frame.total_margin().sum().x),
                );
                frame.show(ui, |ui| {
                    let visible_height = ui.available_height().max(design::XL);
                    let mut scroll = egui::ScrollArea::vertical().id_salt((
                        "session_editor",
                        self.selected,
                        in_panel,
                    ));
                    // A view at the note's end stays there when it gets shorter (a bottom banner
                    // arrives), so the line being typed stays in view. Only a shorter view does
                    // this; a note that grows keeps its place.
                    let view_id = egui::Id::new(("session_editor_view", self.selected, in_panel));
                    let before: Option<(f32, bool, f32)> = ui.data(|data| data.get_temp(view_id));
                    if reset_scroll {
                        scroll = scroll.vertical_scroll_offset(0.0);
                    } else if let Some((height, at_end, content)) = before
                        && at_end
                        && visible_height < height - 0.5
                    {
                        scroll = scroll.vertical_scroll_offset((content - visible_height).max(0.0));
                    }
                    let output = scroll
                        .max_height(visible_height)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            // Beside a page, Tab indents. It must not walk to `Next turn`, where the next
                            // space would press it. Escape still leaves the panel.
                            let mut layouter = crate::markdown_source::layouter;
                            let output = egui::TextEdit::multiline(&mut draft.body)
                                .id(egui::Id::new((
                                    "session_note_text",
                                    self.selected,
                                    in_panel,
                                )))
                                .lock_focus(in_panel)
                                .frame(false)
                                .font(design::reader::prose_type(ui).font(ui))
                                .layouter(&mut layouter)
                                .margin(egui::Vec2::ZERO)
                                .desired_width(ui.available_width().min(design::READER_MEASURE))
                                .min_size(egui::vec2(0.0, visible_height + heading_offset))
                                .show(ui);
                            crate::markdown_source::paint_section_rules(ui, &output);
                            pressed_link =
                                crate::markdown_source::pressed_note_link(ui, &output, None);
                            let place = super::slash::Place {
                                session: self.selected,
                                campaign: self.campaign.as_ref(),
                                index: index.as_deref(),
                                run_open: self.run_open,
                            };
                            let done = self.slash.show(ui, &output, &mut draft.body, &place);
                            self.wants_content_refresh |= done.created;
                            if done.edited {
                                draft.edited();
                                if let Some(session) = self.selected {
                                    self.mentions
                                        .insert(session, timeline::referenced_ids(&draft.body));
                                }
                            }
                            if in_panel && let Some(at) = draft.body.find("## What happened") {
                                // Even a short note must scroll its repeated heading out of the panel.
                                let heading = draft.body[..at].chars().count();
                                let offset = output
                                    .galley
                                    .pos_from_cursor(egui::text::CCursor::new(heading))
                                    .top();
                                ui.data_mut(|data| data.insert_temp(heading_id, offset.max(0.0)));
                            }
                            let response = output.response;
                            design::named(ui, &response, "Note text");
                            self.note_had_focus = response.has_focus();
                            // Narrowing keeps whichever side the DM last worked in: the note
                            // while it has focus, the plan once focus moves to a plan field.
                            if !in_panel && response.has_focus() {
                                self.tab = Tab::Note;
                            } else if !in_panel
                                && self.tab == Tab::Note
                                && ui.memory(|memory| memory.focused().is_some())
                            {
                                self.tab = Tab::Plan;
                            }
                            self.note_editor_id = Some(response.id);
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
                                    insert_at_caret(ui.ctx(), response.id, draft, &words, new_line);
                                }
                            }
                            if let Some(caret) = self.caret_to_show.take() {
                                let rect = output
                                    .galley
                                    .pos_from_cursor(egui::text::CCursor::new(if in_panel {
                                        draft
                                            .body
                                            .find("## What happened")
                                            .map_or(caret, |at| draft.body[..at].chars().count())
                                    } else {
                                        caret
                                    }))
                                    .translate(output.galley_pos.to_vec2());
                                if in_panel {
                                    ui.scroll_to_rect(rect, Some(egui::Align::TOP));
                                } else {
                                    let clip = ui.clip_rect();
                                    if rect.top() < clip.top() || rect.bottom() > clip.bottom() {
                                        let wanted = (rect.bottom() - clip.height()).max(0.0);
                                        let top = output
                                            .galley
                                            .rows
                                            .iter()
                                            .map(|row| row.pos.y + output.galley_pos.y)
                                            .find(|top| *top >= wanted)
                                            .unwrap_or(rect.top());
                                        let visible = egui::Rect::from_min_size(
                                            egui::pos2(rect.left(), top),
                                            egui::vec2(rect.width(), clip.height()),
                                        );
                                        ui.scroll_to_rect(visible, Some(egui::Align::TOP));
                                    }
                                }
                            }
                            if focus {
                                response.request_focus();
                                let (caret, changed) = fresh_line_in_note(&mut draft.body);
                                if changed {
                                    draft.edited();
                                }
                                self.caret_to_show = Some(caret);
                                let end = egui::text::CCursor::new(caret);
                                let mut state = egui::TextEdit::load_state(ui.ctx(), response.id)
                                    .unwrap_or_default();
                                state
                                    .cursor
                                    .set_char_range(Some(egui::text::CCursorRange::one(end)));
                                egui::TextEdit::store_state(ui.ctx(), response.id, state);
                            }
                            if response.changed() {
                                draft.edited();
                                if let Some(session) = self.selected {
                                    self.mentions
                                        .insert(session, timeline::referenced_ids(&draft.body));
                                }
                            }
                        });
                    // Only a note that scrolls can be at its end.
                    let room = output.content_size.y - output.inner_rect.height();
                    let at_end = room > 1.0 && output.state.offset.y >= room - 1.0;
                    ui.data_mut(|data| {
                        data.insert_temp(view_id, (visible_height, at_end, output.content_size.y));
                    });
                    #[cfg(test)]
                    ui.data_mut(|data| {
                        data.insert_temp(
                            egui::Id::new("session_editor_proof"),
                            (self.note_editor_id.unwrap(), output.state.offset),
                        )
                    });
                });
            },
        );
        if let Some(target) = pressed_link {
            self.open_note_link(target);
        }
    }

    pub fn fresh_focus_in_panel(&mut self) {
        self.focus_notes = self.notes_open;
    }

    /// A button took the keyboard. It goes back to the panel, or to the page's editor if
    /// the DM was typing there.
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
                let date = self.selected.and_then(|id| self.notes.iter().find(|note| note.id == id)).map_or("", |note| note.date.as_str());
                let mut close = false;
                if self.selected.is_none() {
                    close = design::panel_header(ui, &title, "Close notes");
                } else {
                    ui.horizontal_top(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                            close = design::icon_button(ui, icons::CLOSE, "Close notes").clicked();
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| note_title(ui, &title, date));
                        });
                    });
                }
                if let Some(draft) = &self.draft { Self::note_status(ui, draft); }
                design::rule(ui);
                if close { self.notes_open = false; return; }
                if self.selected.is_none() {
                    if design::message_with_actions(ui, Tone::Info, "No session yet.", |ui| {
                        Action::primary("New session").show(ui).clicked()
                    }) {
                        self.start_new_session();
                    }
                    return;
                }
                if self.current().is_none() {
                    design::message(
                        ui,
                        Tone::Info,
                        "New events go to no session, so no note is open here. Choose a session on the Timeline page.",
                    );
                    return;
                }
                // Round 26: the panel's notice and its draft's result sit in its message footer,
                // so the note the DM types in never moves when one comes or goes.
                let today = self.today();
                let note = self
                    .selected
                    .and_then(|id| self.notes.iter().find(|note| note.id == id));
                let choice = std::cell::Cell::new((false, false));
                let undo = std::cell::Cell::new(false);
                let mut footer = design::Footer::new("notes_panel_footer", ui.available_width());
                if let Some(draft) = &self.draft {
                    let mut undos = LiveUndos::new();
            draft_result_in(&mut undos, draft, &undo);
            undos.into_footer(&mut footer);
                }
                if let Some(words) = note.and_then(|note| dated_before_today(note, &today)) {
                    let choice = &choice;
                    footer.notice(words.clone(), move |ui| {
                        // The choices start at the warning's text.
                        design::message_with_actions(ui, Tone::Warning, &words, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                let play = Action::secondary("Play it tonight")
                                    .density(Density::Read)
                                    .show(ui)
                                    .clicked();
                                let start = Action::secondary("New session")
                                    .density(Density::Read)
                                    .show(ui)
                                    .clicked();
                                choice.set((play, start));
                            });
                        });
                    });
                }
                let bottom = footer.show(ui);
                let (play, start) = choice.get();
                if undo.get()
                    && let Some(draft) = &mut self.draft
                {
                    draft.undo_dropped();
                }
                if play {
                    self.play_this_one();
                }
                if start {
                    self.start_new_session();
                }
                body_above(ui, bottom, |ui| self.note_editor(ui, true));
            });
    }

    fn note_body(&self) -> String {
        let Some(draft) = &self.draft else {
            return String::new();
        };
        let note = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id));
        note.map_or_else(
            || draft.body.clone(),
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
        egui::ScrollArea::vertical()
            .id_salt(("session_reader", self.selected))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.scope(|ui| {
                    ui.set_max_width(ui.available_width().min(design::READER_MEASURE));
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
                        self.show_prepared(ui, &prepared, false);
                        if let Some(summary) = &prepared.summary {
                            ui.label(Type::Small.rich(ui, summary).color(colors().muted));
                            self.show_prepared(ui, &prepared, true);
                        }
                    }
                    self.world_changes(ui);
                });
            });
        if let Some(target) = markdown_view::take_note_link(ui.ctx()) {
            self.open_note_link(target);
        }
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
        design::reader::section_heading(ui, "World changes");
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
            self.note_editor(ui, false);
        }
    }

    /// A run closed, so what was read of any fight may be out of date.
    pub fn forget_fight(&mut self) {
        self.fight = None;
    }

    /// A note's link that no entry answers to. Saying so is better than a link that does nothing.
    pub fn no_entry_named(&mut self, name: &str) {
        self.notice = Some((Tone::Info, format!("No entry is named {name}.")));
        self.notice_session = None;
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
        if let (Some(draft), Some(session)) = (&mut self.draft, self.selected)
            && rename_links(&mut draft.body, old, new)
        {
            draft.edited();
            self.mentions
                .insert(session, timeline::referenced_ids(&draft.body));
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
                    self.draft.as_ref().map(|draft| draft.body.clone())
                } else {
                    fs::read_to_string(&note.path)
                        .ok()
                        .map(|text| body_of(&text).to_owned())
                }?;
                let body = without_own_heading(&body, note);
                Some(Written {
                    title: note.label(),
                    detail: format!("Session note · {}", note.date),
                    text: words_to_read(&body, book),
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
        let Some(selected) = self.selected else {
            return;
        };
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
        self.last_time_line(ui, selected, plan, book);
    }

    fn recording_row(
        &mut self,
        ui: &mut egui::Ui,
        recording: &Recording,
        settings: &ProcessingSettings,
        book: &Book,
        primary: bool,
        destination_here: bool,
    ) {
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
        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        delete = Action::quiet("Delete…")
                            .name(&format!("Delete recording: {}", recording.name))
                            .density(Density::Read)
                            .blocked_if(busy, "This recording is in use.")
                            .show(ui)
                            .clicked();
                        if busy {
                            ui.label(
                                Type::Small
                                    .rich(ui, "This recording is in use.")
                                    .color(c.muted),
                            );
                        }
                    });
                    if delete {
                        ui.close();
                    }
                });
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.add(
                        egui::Label::new(Type::Strong.rich(ui, &recording.title).color(c.text))
                            .wrap(),
                    );
                });
            });
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
        let (icon, word, color) = if live {
            self.recording_status()
                .unwrap_or((icons::RECORD, "Recording", c.recording))
        } else if processing {
            (icons::BUSY, "Processing", c.connection.busy)
        } else if let Some(words) = stored.as_deref() {
            let state = if words.starts_with("Failed") {
                "Failed"
            } else if words.starts_with("Processing cancelled") {
                "Cancelled"
            } else {
                "Stopped"
            };
            if state == "Cancelled" {
                (icons::WARN, state, c.result.warning)
            } else {
                (icons::ERROR, state, c.result.error)
            }
        } else if recording.warning.is_some() {
            {
                let status = processed_warning();
                (status.icon, status.word, status.color)
            }
        } else if recording.summary {
            let status = processed_clean();
            (status.icon, status.word, status.color)
        } else if recording.transcript {
            (icons::DRAFT, "Partly processed", c.muted)
        } else {
            (icons::DOCUMENT, "Not processed", c.muted)
        };
        if !processing {
            design::status(ui, Status { icon, word, color }).on_hover_text(word);
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
            ui.label(
                Type::Small
                    .rich(
                        ui,
                        format!("{} recorded · {sound}", clock(live.recorder.seconds())),
                    )
                    .color(c.muted),
            );
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
            || job_here
            || !recording.summary
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
                if missing_input
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
                record_again = (if primary
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
                if stalled {
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
            if job_here {
                let words = self
                    .job
                    .as_ref()
                    .map(|(job, _)| job.progress.words())
                    .unwrap_or_default();
                {
                    design::spinner(ui);
                    ui.label(
                        Type::Small
                            .rich(ui, format!("Processing · {words}"))
                            .color(c.muted),
                    );
                    let cancelling = self.job.as_ref().is_some_and(|(job, _)| job.cancelling());
                    if cancelling {
                        ui.label(
                            Type::Small
                                .rich(ui, "Cancelling. This ends after the current step.")
                                .color(c.muted),
                        );
                    } else if Action::quiet("Cancel")
                        .name("Cancel processing")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                        && let Some((job, _)) = &self.job
                    {
                        job.cancel();
                    }
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
                ui.add_space(design::L);
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
                // The choices start at the message text; the red one keeps an `L` gap.
                design::message_with_actions(ui, Tone::Info, &words, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        if Action::secondary("Not now").show(ui).clicked() {
                            self.confirm_again = None;
                        }
                        ui.add_space(design::L - ui.spacing().item_spacing.x);
                        process = Action::destructive("Replace the results")
                            .show(ui)
                            .clicked();
                    });
                });
            });
        }
        if stop {
            self.finish_recording();
        }
        if record_again && let Some(session) = self.selected {
            if stalled {
                self.finish_recording();
            }
            if let Err(error) = self.start_recording(settings, session) {
                self.notice = Some((Tone::Error, error));
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

    /// A message of success has done its work once the DM has moved on.
    fn clear_success(&mut self) {
        // News of another session's job waits until the DM has seen it.
        if matches!(self.notice, Some((Tone::Success, _)))
            && (self.notice_session.is_none() || self.notice_session == self.selected)
        {
            self.notice = None;
            self.notice_session = None;
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
        let mark = marker.as_ref().map(|(marker, _)| marker.reading_words());
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
        });
    }

    fn markers(&mut self, ui: &mut egui::Ui, recording: &Recording, book: &mut Book) {
        if recording.markers.is_empty() {
            return;
        }
        let c = colors();
        let session = self.selected;
        // One `L` gap between the recording's last line and the events heading.
        ui.add_space(design::L - ui.spacing().item_spacing.y);
        design::group_heading(
            ui,
            "Events marked in this recording",
            recording.markers.len(),
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
                measure(marker.kind.icon(), Type::Body)
                    + design::S
                    + measure(title, Type::Body).max(time + measure(facts, Type::Small))
            })
            .fold(0.0, f32::max)
            .min(design::READER_MEASURE)
            .min((ui.available_width() - action - design::L).max(0.0));
        let widths = [text, action];
        // The rows and their rules take the combined width. A row's result message uses the
        // list measure, so its Undo follows the words on one line when they fit.
        let message_width = ui.available_width().min(design::LIST_MEASURE);
        ui.scope(|ui| {
            for (index, (marker, title, key, added, facts, placement)) in rows.iter().enumerate() {
                ui.scope(|ui| {
                    ui.set_width(text + design::L + action);
                    design::table_row(ui, &widths, design::L, |ui, cell| {
                        ui.spacing_mut().item_spacing.x = design::S;
                        if cell == 0 {
                            ui.horizontal_top(|ui| {
                                ui.label(Type::Body.rich(ui, marker.kind.icon()).color(c.muted));
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
                                    if !placement.is_empty() {
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
                                    self.recording_added = Some(id);
                                }
                            });
                        }
                    });
                });
                if added.as_ref() == self.recording_added.as_ref()
                    && let Some(id) = &added
                {
                    let words = added_message(title, self.session_name(session).as_deref(), None);
                    let undo = ui
                        .scope(|ui| {
                            ui.set_max_width(message_width);
                            design::undo_message(ui, Tone::Success, &words, "Undo add")
                        })
                        .inner;
                    if undo {
                        book.undo_add(id);
                        self.recording_added = None;
                    }
                }
            }
        });
        if let Some((at, marker, key)) = read_at {
            self.open_reading(recording, Made::Marked, Some(&at), Some((marker, key)));
        }
    }

    /// One text from a recording, in the whole pane, one part at a time. A 5-hour transcript
    /// is about 30 parts of a size the Reader handles.
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
                    back = Action::quiet("Recordings")
                        .icon(icons::BACK)
                        .name("Back to recordings")
                        .density(Density::Read)
                        .show(ui)
                        .clicked();
                    ui.label(Type::Small.rich(ui, &reading.name).color(c.muted));
                });
            });
        });
        let switched = if segments_width + design::XS <= ui.available_width() {
            design::segmented(ui, &mut made, &options)
        } else {
            let inner = egui::ComboBox::from_id_salt("recording_view")
                .selected_text(made.word())
                .show_ui(ui, |ui| {
                    for (option, label) in &options {
                        ui.selectable_value(&mut made, *option, *label);
                    }
                });
            design::named(ui, &inner.response, "Recording view");
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
        let added_to = self.session_name(self.selected);
        let Some(reading) = &mut self.reading else {
            return;
        };
        if let Some((done, parts)) = reading.short_of {
            design::message(
                ui,
                Tone::Warning,
                &format!(
                    "Transcribed so far: {done} of {parts} parts. Continue processing adds the rest."
                ),
            );
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
                let inner = egui::ComboBox::from_id_salt("recording_part")
                    .selected_text(&reading.parts[reading.part].0)
                    .width(120.0)
                    .icon(design::combo_icon)
                    .show_ui(ui, |ui| {
                        for (at, (time, _)) in reading.parts.iter().enumerate() {
                            ui.selectable_value(&mut reading.part, at, time);
                        }
                    });
                design::named(ui, &inner.response, "Go to a time");
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
        ui.add_space(design::XS);
        egui::ScrollArea::vertical()
            .id_salt((
                "recording_text",
                &reading.recording,
                reading.made.word(),
                reading.part,
            ))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                note_width(ui, |ui| {
                    let body = &reading.parts[reading.part].1;
                    let body = body.replace(
                        "> This is a draft from an LLM. Check it against your own memory.\n",
                        "",
                    );
                    let heading = format!("## {}", reading.parts[reading.part].0);
                    let body = if count > 1 {
                        body.trim_start()
                            .strip_prefix(&heading)
                            .filter(|rest| rest.starts_with('\n'))
                            .map(str::trim_start)
                            .unwrap_or(&body)
                    } else {
                        &body
                    };
                    let at = reading
                        .marker
                        .as_ref()
                        .map(|(marker, _)| marker.reading_words())
                        .as_deref()
                        .or(reading.mark.as_deref())
                        .and_then(|mark| body.find(mark));
                    match at {
                        Some(at) => {
                            markdown_view::show_reader(ui, &body[..at]);
                            if reading.mark.is_some() {
                                ui.scroll_to_cursor(Some(egui::Align::TOP));
                            }
                            if let Some((marker, key)) = &reading.marker {
                                let line_end =
                                    body[at..].find('\n').map_or(body.len(), |end| at + end);
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
                                        self.recording_added = Some(id);
                                    }
                                });
                                if let Some(id) = &self.recording_added
                                    && book
                                        .event_of_marker(key)
                                        .is_some_and(|event| event.id == *id)
                                {
                                    let id = id.clone();
                                    let words =
                                        added_message(&marker.title, added_to.as_deref(), None);
                                    if design::undo_message(ui, Tone::Success, &words, "Undo add") {
                                        book.undo_add(&id);
                                        self.recording_added = None;
                                    }
                                }
                                markdown_view::show_reader(ui, &body[line_end..]);
                            } else {
                                markdown_view::show_reader(ui, &body[at..]);
                            }
                        }
                        None => markdown_view::show_reader(ui, body),
                    }
                });
                reading.mark = None;
                // A part fills several screens, so the way on is also where the reading ends.
                ui.add_space(design::S);
                if reading.part + 1 < count
                    && Action::secondary("Go on to the next part")
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                {
                    reading.part += 1;
                }
            });
        if back {
            self.reading = None;
        }
        if let Some(body) = add_to_notes
            && let Some(draft) = &mut self.draft
        {
            insert_in_note(&mut draft.body, &body);
            draft.edited();
            self.notice_session = None;
            self.notice = Some((Tone::Success, "Added to the notes.".to_owned()));
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
            ui.add_space(design::M);
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
        egui::ScrollArea::vertical()
            .id_salt(("session_recordings", self.selected))
            .auto_shrink([false, false])
            .show(ui, |ui| {
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
                for (index, recording) in recordings.iter().enumerate() {
                    ui.add_space(design::S);
                    ui.scope_builder(
                        egui::UiBuilder::new().id(("recording_row", &recording.path)),
                        |ui| {
                            self.recording_row(
                                ui,
                                recording,
                                settings,
                                book,
                                primary == Some(index) && !self.confirm_record && !banner_primary,
                                destination == Some(index),
                            );
                        },
                    );
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
                    "This removes {} and what was made from it from the campaign folder. You cannot undo this. The timeline stays as it is.",
                    recording_title(&path)
                ))
                .wrap(),
            );
            ui.add_space(design::S);
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
            self.notice_session = None;
            // A deleted recording takes its result with it, so the rail stops saying Processed.
            if self.processed_recording.as_deref() == Some(path.as_path()) {
                self.processed_session = None;
                self.processed_recording = None;
                self.job_done = false;
                self.job_warned = false;
            }
            self.notice = Some(match delete_recording(&path) {
                Ok(()) => (Tone::Success, "Recording deleted.".to_owned()),
                Err(error) => (Tone::Error, format!("Not deleted. {error}")),
            });
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
    /// message footer, pinned at the pane's bottom, so the title's line, Last time, the tabs and
    /// the recordings never move when one comes. Returns where the body ends.
    fn session_footer(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
        plan: &Plan<'_>,
        note: &Note,
    ) -> f32 {
        self.expire_move(book);
        self.end_carry_results();
        let shown = self.notice_session.is_none() || self.notice_session == self.selected;
        if shown && self.job_done() {
            self.job_done = false;
        }
        let act = std::cell::Cell::new(None);
        let undo_dropped = std::cell::Cell::new(false);
        let mut footer =
            design::Footer::new(("session_footer", self.selected), ui.available_width());
        let mut undos = LiveUndos::new();
        if let Some(draft) = &self.draft {
            draft_result_in(&mut undos, draft, &undo_dropped);
        }
        if let Some((tone, words)) = self
            .notice
            .clone()
            .filter(|_| shown && !self.notice_arrived())
        {
            let act = &act;
            if let Some(session) = self
                .move_run_here
                .as_ref()
                .filter(|(session, notice)| {
                    Some(*session) == self.selected && *notice == words && plan.open_run.is_some()
                })
                .map(|(session, _)| *session)
            {
                footer.result(words.clone(), false, move |ui| {
                    if design::message_with_action(ui, tone, &words, "Move it here") {
                        act.set(Some(SessionAct::MoveRunHere(session)));
                    }
                });
            } else if self.moved_run.is_some() {
                let order = self.result_orders.of("move", &words);
                undos.push(order, words.clone(), move |ui| {
                    if design::undo_message(ui, tone, &words, "Undo move") {
                        act.set(Some(SessionAct::UndoMove));
                    }
                });
            } else {
                let draw = move |ui: &mut egui::Ui| {
                    ui.horizontal_top(|ui| {
                        let button = Density::Read.control() + design::S;
                        let width = ui.available_width() - button;
                        ui.allocate_ui_with_layout(
                            egui::vec2(width, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| design::message(ui, tone, &words),
                        );
                        if design::icon_button(ui, icons::CLOSE, "Dismiss").clicked() {
                            act.set(Some(SessionAct::Dismiss));
                        }
                    });
                };
                let name = self
                    .notice
                    .as_ref()
                    .map(|(_, words)| words.clone())
                    .unwrap_or_default();
                if tone == Tone::Error {
                    footer.reason(name, draw);
                } else {
                    footer.result(name, false, draw);
                }
            }
        }
        // A copy that claims this session arrives from the folder, so it sits in the bottom
        // banner (`banner_notices`); its result is here.
        if let Some(undo) = &self.identity_undo {
            let filename = undo.path.file_name().unwrap_or_default().to_string_lossy();
            let words = format!("{filename} is now a separate session.");
            let act = &act;
            let order = self.result_orders.of("identity", &undo.path);
            undos.push(order, words.clone(), move |ui| {
                if design::undo_message(ui, Tone::Success, &words, "Undo separate session") {
                    act.set(Some(SessionAct::UndoIdentity));
                }
            });
        }
        let today = self.today();
        let writing = self.tab != Tab::Recordings && !self.reading_note;
        if writing
            && self.selected == self.current()
            && let Some(words) = dated_before_today(note, &today)
        {
            let act = &act;
            footer.notice(words.clone(), move |ui| {
                if design::message_with_action(ui, Tone::Warning, &words, "Play this one") {
                    act.set(Some(SessionAct::PlayThisOne));
                }
            });
        }
        if let Some(mut words) = self.not_being_played() {
            let current = if plan.group_open {
                self.group_session
            } else {
                self.current()
            };
            if plan.group_open
                && let Some(group) = self
                    .notes
                    .iter()
                    .find(|note| Some(note.id) == self.group_session)
            {
                let (open, group) = session_warning_names(note, group);
                words = format!(
                    "{open} is not the session being played. A group is being assembled in {group}."
                );
            }
            let next = self
                .notes
                .iter()
                .find(|note| Some(note.id) == current)
                .map(|destination| {
                    if current == self.current() {
                        "Return to the session being played".to_owned()
                    } else {
                        let (_, destination) = session_warning_names(note, destination);
                        format!("Open {destination}")
                    }
                });
            let act = &act;
            footer.notice(words.clone(), move |ui| {
                if let Some(next) = next {
                    if design::message_with_action(ui, Tone::Warning, &words, &next) {
                        act.set(Some(SessionAct::Open(current)));
                    }
                } else {
                    design::message(ui, Tone::Warning, &words);
                }
            });
        }
        // Another session's recording: its state, time and title, as a notice. As on the
        // recording row, the state's colour is only on its icon and word.
        if let (Some(live), Some((icon, word, color))) = (&self.live, self.recording_status())
            && Some(live.session) != self.selected
        {
            let detail = format!(" {} in {}", clock(live.recorder.seconds()), live.label);
            let (icon, word) = (icon.to_owned(), word.to_owned());
            footer.notice(format!("{word}{detail}"), move |ui| {
                let format = |color| egui::TextFormat {
                    font_id: Type::Small.font(ui),
                    color,
                    ..Default::default()
                };
                let mut job = egui::text::LayoutJob::default();
                job.append(&icon, 0.0, format(color));
                job.append(" ", 0.0, format(color));
                job.append(&word, 0.0, format(color));
                job.append(&detail, 0.0, format(colors().muted));
                ui.add(egui::Label::new(job).wrap())
                    .on_hover_text(format!("{word}{detail}"));
            });
        }
        // The input went away by itself: a notice, not the recording's state line.
        if self.live.is_none() && !self.confirm_record && settings.input_missing() {
            const MISSING: &str = "The input chosen in Settings is not connected.";
            let act = &act;
            footer.notice(MISSING, move |ui| {
                if design::message_with_action(ui, Tone::Warning, MISSING, "Open Settings") {
                    act.set(Some(SessionAct::OpenSettings));
                }
            });
        }
        self.carry_in_footer(&mut undos, &mut footer, &act);
        undos.into_footer(&mut footer);
        let bottom = footer.show(ui);
        if undo_dropped.get()
            && let Some(draft) = &mut self.draft
        {
            draft.undo_dropped();
        }
        match act.take() {
            Some(SessionAct::MoveRunHere(session)) => {
                if let Some(run) = plan.open_run {
                    self.move_run_to_session(book, run, session);
                }
            }
            Some(SessionAct::UndoMove) => self.undo_move(book),
            Some(SessionAct::Dismiss) => {
                self.notice = None;
                self.notice_session = None;
            }
            Some(SessionAct::UndoIdentity) => {
                if let Some(undo) = &self.identity_undo {
                    match undo.undo() {
                        Ok(()) => {
                            self.identity_undo = None;
                            self.refresh();
                        }
                        Err(error) => self.notice = Some((Tone::Error, error)),
                    }
                }
            }
            Some(SessionAct::PlayThisOne) => self.play_this_one(),
            Some(SessionAct::Open(session)) => self.select(session),
            Some(SessionAct::OpenSettings) => self.wants_processing_settings = true,
            Some(SessionAct::UndoCarry(session)) => self.undo_carry(session),
            None => {}
        }
        bottom
    }

    fn session_pane(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
        plan: &Plan<'_>,
    ) {
        let c = colors();
        let Some(note) = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
            .cloned()
        else {
            // Its file went from the folder, and the words are still here. The same
            // header as a note, so that the page does not jump at the next scan.
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                self.record_control(ui, settings);
                if let Some(draft) = &self.draft {
                    ui.allocate_ui_with_layout(
                        egui::vec2(STATUS_WIDTH, Density::Scan.control()),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| Self::note_status(ui, draft),
                    );
                    let name = draft.path.file_stem().unwrap_or_default().to_string_lossy();
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                        ui.add(egui::Label::new(Type::Title.rich(ui, name).color(c.text)).wrap());
                    });
                }
            });
            self.record_line(ui, settings);
            if self.draft.is_some() {
                let bottom = self.draft_footer(ui, "session_gone_footer", false);
                body_above(ui, bottom, |ui| self.note_editor(ui, false));
            }
            return;
        };
        let title_on_own_line = ui.available_width() < TWO_COLUMNS_FROM;
        ui.horizontal(|ui| {
            // One height whether Record, Stop recording or nothing stands beside the title, so
            // the line and the tabs below keep their y when a recording starts or stops.
            ui.set_min_height(Density::Scan.control());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                self.record_control(ui, settings);
                if !title_on_own_line
                    || self
                        .draft
                        .as_ref()
                        .is_some_and(|draft| draft.state == DraftState::Conflict)
                {
                    self.session_details_action(ui, &note);
                }
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    note_title(ui, &note.label(), &note.date);
                });
            });
        });
        self.record_line(ui, settings);
        // The body (Last time, the tabs and their pane) ends above the footer: its own `Ui`
        // ends there, so an editor or a list that fills its height stops above the footer.
        let bottom = self.session_footer(ui, settings, book, plan, &note);
        body_above(ui, bottom, |ui| {
            self.session_body(ui, settings, book, plan, &note, title_on_own_line);
        });
    }

    /// The session page below its title and state line, above its message footer.
    fn session_body(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
        plan: &Plan<'_>,
        note: &Note,
        title_on_own_line: bool,
    ) {
        // A group being assembled needs the height, and last time can wait for it.
        if !(plan.group_open && self.group_session == self.selected) {
            self.last_time_block(ui, book, plan);
        }
        if self
            .draft
            .as_ref()
            .is_some_and(|draft| draft.state == DraftState::Conflict)
        {
            self.note_editor(ui, false);
            return;
        }
        let editing_group_here = plan.group_open && self.group_session == self.selected;
        let beside_from = if editing_group_here {
            design::LIST_MEASURE + design::L + design::READER_MEASURE
        } else {
            PLAN_BESIDE_NOTE_FROM
        };
        let beside = ui.available_width() >= beside_from;
        // In a wide page, the note is on the plan's tab, so there is no second one to choose.
        let mut shown = if beside && self.tab == Tab::Note {
            Tab::Plan
        } else {
            self.tab
        };
        let tabs: &[(Tab, &str)] = if beside {
            &[(Tab::Plan, "Plan and notes")]
        } else {
            &[(Tab::Plan, "Plan"), (Tab::Note, "Note")]
        };
        let plan_width = if editing_group_here {
            design::LIST_MEASURE
        } else {
            (ui.available_width() * 0.4).clamp(PLAN_WIDTH, PLAN_MAX_WIDTH)
        };
        let note_start = if beside { plan_width + design::L } else { 0.0 };
        let toolbar_width = ui
            .available_width()
            .min(note_start + design::READER_MEASURE + 2.0 * (design::S + design::HAIRLINE_WIDTH));
        ui.horizontal(|ui| {
            if shown != Tab::Recordings {
                ui.set_max_width(toolbar_width);
            }
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
        });
        ui.add_space(design::XS);
        match shown {
            Tab::Plan if beside => {
                let height = ui.available_height();
                let plan_width = if editing_group_here {
                    design::LIST_MEASURE
                } else {
                    (ui.available_width() * 0.4).clamp(PLAN_WIDTH, PLAN_MAX_WIDTH)
                };
                let note_width = ui.available_width() - plan_width - design::L;
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = design::L;
                    page_column(ui, plan_width, height, |ui| {
                        fixed_scope(ui, editing_group_here, "session_plan_pane", |ui| {
                            self.plan_pane(ui, book, plan)
                        });
                    });
                    page_column(ui, note_width, height, |ui| {
                        fixed_scope(ui, editing_group_here, "session_note_pane", |ui| {
                            self.note_pane(ui, book, plan, beside)
                        })
                    });
                });
            }
            // The same ids in both layouts: crossing the width keeps typed text, focus and scroll.
            // The plan's rows end at the tab row's right edge, as the note's frame does, so the
            // pane has one right edge; the scroll bar's lane lies beyond it. A group being edited
            // keeps its own width.
            Tab::Plan => {
                ui.scope(|ui| {
                    if !editing_group_here {
                        ui.set_max_width(toolbar_width + ui.spacing().scroll.allocated_width());
                    }
                    fixed_scope(ui, editing_group_here, "session_plan_pane", |ui| {
                        self.plan_pane(ui, book, plan)
                    })
                })
                .inner
            }
            Tab::Note => fixed_scope(ui, editing_group_here, "session_note_pane", |ui| {
                self.note_pane(ui, book, plan, beside)
            }),
            Tab::Recordings => {
                if self.processed_notice_read_here() {
                    self.notice = None;
                    self.notice_session = None;
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
            egui::ScrollArea::vertical()
                .id_salt("session_form")
                .auto_shrink([false, false])
                .show(ui, |ui| self.show_form(ui));
            return;
        }
        // Beside Lookup at a small window, two columns would crush the session.
        self.list_beside = ui.available_width() >= TWO_COLUMNS_FROM;
        if !self.list_beside {
            if self.list_only && self.form.is_none() {
                self.list_pane(ui);
            } else {
                if self.form.is_none()
                    && Action::quiet("All sessions")
                        .icon(icons::BACK)
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                {
                    self.list_only = true;
                }
                if self.form.is_some() {
                    egui::ScrollArea::vertical()
                        .id_salt("session_form")
                        .auto_shrink([false, false])
                        .show(ui, |ui| self.show_form(ui));
                } else {
                    self.session_pane(ui, settings, book, plan);
                }
            }
            return;
        }
        design::Column::left("session_list_panel", LIST_WIDTH, LIST_WIDTHS)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                right: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| self.list_pane(ui));
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                if self.form.is_some() {
                    egui::ScrollArea::vertical()
                        .id_salt("session_form")
                        .auto_shrink([false, false])
                        .show(ui, |ui| self.show_form(ui));
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
            self.template_list(ui, &mut choose);
        } else if narrow {
            if Action::quiet("All templates")
                .icon(icons::BACK)
                .density(Density::Read)
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
                let bottom = self.draft_footer(ui, "template_footer", true);
                body_above(ui, bottom, |ui| {
                    // One thing at a time, so that the DM does not type into another template.
                    if self.new_template.is_some() {
                        self.new_template_form(ui);
                    } else {
                        self.template_editor(ui);
                    }
                });
            });
    }

    fn template_list(&mut self, ui: &mut egui::Ui, choose: &mut Option<String>) {
        if Action::quiet("Sessions")
            .icon(icons::BACK)
            .show(ui)
            .clicked()
        {
            self.view = View::Notes;
        }
        ui.add_space(design::S);
        egui::ScrollArea::vertical()
            .id_salt("template_list")
            .auto_shrink([false, false])
            .show(ui, |ui| {
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
                if let Some((name, _)) = self.deleted_template.clone()
                    && design::undo_message(
                        ui,
                        Tone::Success,
                        &format!("Deleted “{name}”."),
                        "Undo delete",
                    )
                {
                    let (name, body) = self.deleted_template.take().expect("just read");
                    if let Some(folder) = &self.folder {
                        match create_template(folder, &name, &body) {
                            Ok(_) => {
                                self.refresh();
                                self.select_template(&name);
                            }
                            Err(error) => self.template_error = Some(error),
                        }
                    }
                }
            });
    }

    fn new_template_form(&mut self, ui: &mut egui::Ui) {
        let Some(mut name) = self.new_template.take() else {
            return;
        };
        ui.add_space(design::L);
        design::group_label(ui, "New template");
        ui.add_space(design::M);
        design::text_field(
            ui,
            "Template name",
            &mut name,
            Some("Starts with the title line only."),
            None,
        );
        let mut keep = true;
        ui.horizontal(|ui| {
            if if self.open_run.is_some() {
                Action::secondary("Create template")
            } else {
                Action::primary("Create template")
            }
            .show(ui)
            .clicked()
                && let Some(folder) = &self.folder
            {
                let body = "# Session {{number}}: {{title}}\n\n";
                match create_template(folder, &name, body) {
                    Ok(template) => {
                        keep = false;
                        self.template_error = None;
                        self.refresh();
                        self.select_template(&template.name);
                    }
                    Err(error) => self.template_error = Some(error),
                }
            }
            if Action::quiet("Cancel").show(ui).clicked() {
                keep = false;
                self.template_error = None;
            }
        });
        if keep {
            self.new_template = Some(name);
        }
    }

    fn template_editor(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        let Some(template) = self
            .template
            .as_ref()
            .and_then(|name| self.templates.iter().find(|t| &t.name == name))
            .cloned()
        else {
            return;
        };
        let mut customize = false;
        let mut delete = false;
        ui.horizontal(|ui| {
            ui.label(Type::Heading.rich(ui, &template.name).color(c.text));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if template.path.is_some() {
                    let more = design::icon_button(
                        ui,
                        icons::EXPAND,
                        &format!("More actions for {}", template.name),
                    );
                    egui::Popup::menu(&more).show(|ui| {
                        let built_in = super::BUILT_IN_TEMPLATES
                            .iter()
                            .any(|(name, _)| *name == template.name);
                        delete = Action::quiet(if built_in {
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
                        if delete {
                            ui.close();
                        }
                    });
                    if let Some(draft) = &self.template_draft {
                        Self::draft_status(ui, draft);
                    }
                } else {
                    customize = Action::secondary("Customize")
                        .name("Customize this template")
                        .density(Density::Read)
                        .show(ui)
                        .clicked();
                    design::status(
                        ui,
                        Status {
                            icon: icons::READ_ONLY,
                            word: "Read-only",
                            color: c.document.read_only,
                        },
                    );
                }
            });
        });
        if customize && let Some(folder) = &self.folder {
            match create_template(folder, &template.name, &template.body) {
                Ok(created) => {
                    self.refresh();
                    self.select_template(&created.name);
                }
                Err(error) => self.template_error = Some(error),
            }
            return;
        }
        if delete && let Some(path) = &template.path {
            match fs::remove_file(path) {
                Ok(()) => {
                    let body = self
                        .template_draft
                        .take()
                        .map_or_else(|| template.body.clone(), |draft| draft.body);
                    self.deleted_template = Some((template.name.clone(), body));
                    self.template = None;
                    self.refresh();
                }
                Err(error) => self.template_error = Some(format!("Not deleted. {error}")),
            }
            return;
        }
        if let Some(draft) = &mut self.template_draft
            && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S))
        {
            draft.save();
        }
        let mut pressed_link = None;
        egui::ScrollArea::vertical()
            .id_salt(("template_editor", &template.name))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if let Some(draft) = &mut self.template_draft {
                    let mut layouter = crate::markdown_source::layouter;
                    let output = egui::TextEdit::multiline(&mut draft.body)
                        .font(design::reader::prose_type(ui).font(ui))
                        .layouter(&mut layouter)
                        .margin(egui::vec2(design::S, design::S))
                        .desired_width(ui.available_width())
                        .min_size(egui::vec2(0.0, ui.available_height()))
                        .show(ui);
                    crate::markdown_source::paint_section_rules(ui, &output);
                    pressed_link = crate::markdown_source::pressed_note_link(ui, &output, None);
                    let response = output.response;
                    design::named(ui, &response, "Template text");
                    if response.changed() {
                        draft.edited();
                        self.deleted_template = None;
                    }
                } else {
                    let mut body = template.body.clone();
                    let mut layouter = crate::markdown_source::layouter;
                    let output = egui::TextEdit::multiline(&mut body)
                        .font(design::reader::prose_type(ui).font(ui))
                        .layouter(&mut layouter)
                        .desired_width(ui.available_width())
                        .min_size(egui::vec2(0.0, ui.available_height()))
                        .frame(false)
                        .interactive(false)
                        .show(ui);
                    design::named(ui, &output.response, "Built-in template text");
                    crate::markdown_source::paint_section_rules(ui, &output);
                }
            });
        if let Some(target) = pressed_link {
            self.open_note_link(target);
        }
    }

    /// `details_open`: the campaign details form, with its own primary save, is in view above.
    pub fn folder_view(
        &mut self,
        ui: &mut egui::Ui,
        book: &Book,
        locked: Option<&str>,
        details_open: bool,
    ) {
        // Earlier sections can have a wider minimum rect. Give this form its own column.
        let rect = egui::Rect::from_min_size(
            ui.cursor().min,
            egui::vec2(
                ui.available_width().min(design::FORM_WIDTH),
                ui.available_height(),
            ),
        );
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            self.folder_form(ui, book, locked, details_open);
        });
    }

    fn folder_form(
        &mut self,
        ui: &mut egui::Ui,
        book: &Book,
        locked: Option<&str>,
        details_open: bool,
    ) {
        let Some(campaign) = self.campaign.clone() else {
            return;
        };
        ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
        ui.add_space(design::XS);
        design::group_heading(ui, "Session folder", 0);
        let response = ui
            .add_enabled_ui(locked.is_none(), |ui| {
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.folder_text)
                        .desired_width(ui.available_width() - 2.0 * design::S)
                        .min_size(egui::vec2(0.0, Density::Read.control()))
                        .margin(egui::vec2(design::S, design::XS)),
                );
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
            self.folder_notice = None;
        }
        if let Some(folder) = &self.folder {
            design::help(ui, &format!("Now: {}", folder.display()));
            design::help(
                ui,
                "Notes, Timeline.md, Templates and Recordings are all here.",
            );
        }
        let changed = super::resolve(&campaign, &self.folder_text).ok() != self.folder.clone();
        // `folder_messages` scanned the folder this frame, before the pane's body.
        let files = self
            .folder_files
            .take()
            .unwrap_or_else(|| self.scan_folder_files());
        let move_counts = self.folder_move_counts(book, &files).unwrap_or_default();
        let has_files = !move_counts.is_empty();
        ui.add_space(design::XS);
        let blocked = if self.live.is_some() {
            Some("Stop the recording first.")
        } else if self.job.is_some() {
            Some("Wait for processing to finish.")
        } else if self.busy_with_edit() || book.not_saved() {
            Some("Choose what to do with the unsaved file first.")
        } else if files.is_err() {
            Some("Could not read the session files. Check the current folder first.")
        } else if !changed {
            Some("No changes to save.")
        } else {
            None
        };
        let mut move_files = false;
        let save = ui
            .horizontal_wrapped(|ui| {
                if changed {
                    design::status(
                        ui,
                        Status {
                            icon: icons::DRAFT,
                            word: "Unsaved",
                            color: colors().document.unsaved,
                        },
                    );
                    if Action::quiet("Discard").show(ui).clicked() {
                        if let Ok(saved) = folder_setting(&campaign) {
                            self.folder_text = saved;
                        }
                        self.folder_error = None;
                    }
                }
                if !changed {
                    design::status(
                        ui,
                        Status {
                            icon: icons::OK,
                            word: "No changes",
                            color: colors().document.clean,
                        },
                    );
                }
                if locked.is_none() && changed {
                    design::blocked_reason(ui, blocked);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if has_files {
                        // One primary in view: the open details form owns its own save.
                        // While a run is open, `reserve_primary` keeps Next turn the only one.
                        move_files = (if details_open {
                            Action::secondary(MOVE_FILES)
                        } else {
                            Action::primary(MOVE_FILES)
                        })
                        .blocked(blocked)
                        .show(ui)
                        .clicked();
                        Action::quiet("Save without moving")
                            .blocked(blocked)
                            .show(ui)
                            .clicked()
                            || move_files
                    } else {
                        Action::secondary("Save folder")
                            .blocked(blocked)
                            .show(ui)
                            .clicked()
                    }
                })
                .inner
            })
            .inner;
        if save {
            self.flush();
            // The save just now can fail. Those words stay on screen, as `select` keeps them.
            if self.draft.as_ref().is_some_and(Draft::needs_attention) {
                self.folder_error = Some(
                    "Not changed. Your note could not be saved. Choose what to do with it first."
                        .to_owned(),
                );
                return;
            }
            let saved = if move_files {
                super::move_storage(&campaign, &self.folder_text)
            } else {
                save_folder(&campaign, &self.folder_text)
            };
            match saved {
                Ok(folder) => {
                    self.folder = Some(folder);
                    self.folder_error = None;
                    self.folder_notice =
                        move_files.then(|| format!("Moved {move_counts} to {}.", self.folder_text));
                    self.manifest_changed = Some(campaign.path.clone());
                    self.draft = None;
                    self.template_draft = None;
                    self.template = None;
                    self.refresh();
                    self.select(self.notes.first().map(|note| note.id));
                }
                Err(error) => {
                    self.folder_error = Some(error);
                    self.folder_notice = None;
                }
            }
        }
    }

    /// The current folder's files, or why they could not be listed. One scan.
    fn scan_folder_files(&self) -> FolderFiles {
        self.folder
            .as_ref()
            .map(|folder| super::storage_files(folder))
            .transpose()
    }

    /// What a Move files and save would move from the current folder, in words, when the typed
    /// folder differs and the current one keeps files. The form's warning and its result say it.
    fn folder_move_counts(&self, book: &Book, files: &FolderFiles) -> Option<String> {
        let campaign = self.campaign.as_ref()?;
        let changed = super::resolve(campaign, &self.folder_text).ok() != self.folder.clone();
        let folder = self.folder.as_ref().filter(|_| changed)?;
        let files = files.as_ref().ok()?.as_ref()?;
        if files.is_empty() {
            return None;
        }
        let notes = self.notes.len();
        let templates = files
            .iter()
            .filter(|path| path.starts_with(folder.join(super::TEMPLATES)))
            .count();
        let recordings = files
            .iter()
            .filter(|path| path.extension().is_some_and(|ext| ext == "wav"))
            .count();
        let mut counts = vec![
            design::count(notes, "note", "notes"),
            design::count(book.events.len(), "event", "events"),
            design::count(templates, "template", "templates"),
            design::count(recordings, "recording", "recordings"),
        ]
        .into_iter()
        .filter(|count| !count.is_empty())
        .collect::<Vec<_>>();
        let last = counts.pop().unwrap_or_default();
        let counts = if counts.is_empty() {
            last
        } else {
            format!("{} and {last}", counts.join(", "))
        };
        Some(if counts.is_empty() {
            "Timeline.md".to_owned()
        } else {
            counts
        })
    }

    /// Round 26 FN2, FN5: the Session folder form's messages, in the message footer of the pane
    /// that holds the form, so a typed letter, a refusal or a result never moves its save row.
    pub fn folder_messages<'a>(&mut self, footer: &mut design::Footer<'a>, book: &Book) {
        let files = self.scan_folder_files();
        let counts = self.folder_move_counts(book, &files);
        self.folder_files = Some(files);
        if let Some(error) = self.folder_error.clone() {
            footer.reason(error.clone(), move |ui| {
                design::message(ui, Tone::Error, &error);
            });
        }
        if let Some(counts) = counts {
            let words = format!(
                "The current folder keeps {counts}. If you save without moving, Splat no longer shows these files."
            );
            footer.notice(words.clone(), move |ui| {
                design::message(ui, Tone::Warning, &words);
            });
        }
        if let Some(words) = self.folder_notice.clone() {
            footer.result(words.clone(), false, move |ui| {
                design::message(ui, Tone::Success, &words);
            });
        }
    }

    pub fn clear_notice(&mut self) {
        self.added = None;
        self.recording_added = None;
    }

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
        let mut new_session = false;
        let in_form = self.form.is_some();
        let empty = self.notes.is_empty();
        let locked = self.draft.as_ref().is_some_and(Draft::needs_attention);
        let mut stop = false;
        design::page_header(ui, page_title, Some(&context), |ui| {
            let list_alone = self.list_only && ui.ctx().content_rect().width() < 1500.0;
            if self.view == View::Notes && !in_form && !empty && !folder_missing {
                new_session = Action::secondary("New session")
                    .icon(icons::ADD)
                    .blocked_if(
                        folder_missing,
                        "Choose the session folder before making a session.",
                    )
                    .blocked_if(locked, "Other sessions wait until you choose what to keep.")
                    .show(ui)
                    .clicked();
            }
            if self.view == View::Templates
                && self.new_template.is_none()
                && Action::secondary("New template")
                    .icon(icons::ADD)
                    .show(ui)
                    .clicked()
            {
                self.templates_list_only = false;
                self.new_template = Some(String::new());
                self.template_error = None;
            }
            if self.live.is_some() && (self.view != View::Notes || in_form || list_alone || empty) {
                stop = Action::quiet("Stop recording")
                    .icon(icons::STOP_RECORDING)
                    .show(ui)
                    .clicked();
            }
            if self.view == View::Notes && Action::quiet("Templates").show(ui).clicked() {
                self.view = View::Templates;
            }
            if self.party_hidden
                && Action::quiet("Show party")
                    .icon(icons::PARTY)
                    .show(ui)
                    .clicked()
            {
                self.party_bar = Some(true);
            }
        });
        if stop {
            self.finish_recording();
        }
        if new_session {
            self.open_form();
        }
        if folder_missing {
            // The only way out of this page, so it has the bordered recovery action.
            if design::message_with_action(
                ui,
                Tone::Warning,
                self.folder_error
                    .as_deref()
                    .unwrap_or("The session folder is not available."),
                "Choose folder",
            ) {
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

#[cfg(test)]
impl Sessions {
    pub fn busy(&self) -> bool {
        self.job.as_ref().is_some_and(|(job, _)| !job.is_held())
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

    /// Processing of the open session finishes now, as the background job would.
    pub fn simulate_job_finished(&mut self) {
        let session = self.selected.expect("an open session");
        self.finish_job(Ok(None), session);
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
        self.notice = None;
        self.notice_session = None;
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
    words: Option<String>,
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
        book.undo_remove();
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
        assert_eq!(book.undo_remove().as_deref(), Some("a"));
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
        assert!(book.undo_remove().is_none());
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
            sessions.not_being_played().as_deref(),
            Some(
                "Session 1 - The Bridge 2.md is not the session being played. The session being played is Session 1 - The Bridge.md. What you write and add on this page goes into Session 1 - The Bridge 2.md."
            ),
            "iter-8-C-ST2-files"
        );
        sessions.notes[1].number = 2;
        assert_eq!(
            sessions.not_being_played().as_deref(),
            Some(
                "Session 2 is not the session being played. What you write and add on this page goes into Session 1 - The Bridge 2.md."
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
                        let rows: Vec<String> = galley
                            .rows
                            .iter()
                            .map(|row| row.glyphs.iter().map(|g| g.chr).collect())
                            .collect();
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
                    let rows: Vec<(String, f32)> = galley
                        .rows
                        .iter()
                        .map(|row| {
                            (
                                row.glyphs.iter().map(|g| g.chr).collect(),
                                row.pos.x + row.glyphs.first().map_or(0.0, |g| g.pos.x),
                            )
                        })
                        .collect();
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
        assert_eq!(clean.color, processed_clean().color);
        page.job_warned = true;
        let status = page.rail_recording_status().unwrap();
        assert_eq!(status.word, "Processed");
        assert_eq!(status.icon, icons::WARN);
        assert_eq!(status.color, colors().result.warning);
    }

    #[test]
    fn integrity_d9_identity_undo_stays_with_its_campaign() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Copy.md");
        let before = format!("---\nsession_id: {}\n---\nDM's words\n", Uuid::new_v4());
        fs::write(&path, before).unwrap();
        let undo = super::super::new_identity(&path).unwrap();
        let after = fs::read(&path).unwrap();
        let mut sessions = Sessions {
            identity_undo: Some(undo),
            ..Default::default()
        };
        sessions.open_campaign(None);
        assert!(sessions.identity_undo.is_none());
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
            sessions.draft.as_ref().unwrap().body,
            "[[Ash Pack#Members]]\n"
        );
        assert!(sessions.rename_problem.is_some());
        fs::remove_file(dir.path().join("Ash Pack.md")).unwrap();
        sessions.rename_links("Ash Pack", "Cinder Pack");
        assert_eq!(
            sessions.draft.as_ref().unwrap().body,
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
    fn message_words_and_undo_keep_their_place_whatever_the_spacing() {
        use egui_kittest::kittest::Queryable;
        let words = "Removed “Shrine of Cinders” from tonight.";
        let place = |spacing: f32| {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(egui::vec2(260.0, 200.0))
                .build(move |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.spacing_mut().item_spacing.x = spacing;
                        design::undo_message(ui, Tone::Info, words, "Undo remove");
                    });
                });
            design::theme::install(&harness.ctx);
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
                title: String::new(),
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
            title: "Rest at the mill".into(),
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
        sessions.undo_event_link_notes();
        assert_eq!(fs::read_to_string(&note_a).unwrap(), replaced);
        let problem = sessions.link_problem.clone().unwrap_or_default();
        assert!(problem.contains(&a_name), "{problem}");
        // (e) Re-check 3: A holds the written bytes again, and is replaced right before Undo
        // reads it. Undo refuses the replacement and names A.
        fs::write(&note_a, &a_written).unwrap();
        sessions.link_problem = None;
        let a_path = note_a.clone();
        let swap = replaced.clone();
        super::BEFORE_UNDO_READ.with(|hook| {
            *hook.borrow_mut() = Some(Box::new(move |_: &std::path::Path| {
                fs::write(&a_path, &swap).unwrap();
            }));
        });
        sessions.undo_event_link_notes();
        super::BEFORE_UNDO_READ.with(|hook| *hook.borrow_mut() = None);
        assert_eq!(fs::read_to_string(&note_a).unwrap(), replaced);
        let problem = sessions.link_problem.clone().unwrap_or_default();
        assert!(problem.contains(&a_name), "{problem}");
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
            name: "Bridge Ambush".into(),
            path: String::new(),
            summary: String::new(),
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
            name: "Bridge Ambush".into(),
            path: String::new(),
            summary: String::new(),
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
    fn move_run_undo_restores_the_event_and_expires_after_an_edit() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("note.md");
        fs::write(&path, "## What happened\n").unwrap();
        let old = Uuid::from_u128(1);
        let new = Uuid::from_u128(2);
        let run = Uuid::from_u128(3);
        let mut sessions = Sessions {
            draft: Some(Draft::open(&path).unwrap()),
            open_run: Some(("Bridge Ambush".into(), old)),
            ..Default::default()
        };
        let mut book = Book::default();
        book.add(Kind::Encounter, Some(old));
        book.events[0].run = Some(run);
        book.events[0].continued_in = Some(new);
        let before = book.events.clone();
        sessions.move_run_to_session(&mut book, run, new);
        assert_eq!(book.events[0].session, Some(new));
        assert_eq!(book.events[0].continued_in, None);
        sessions.undo_move(&mut book);
        assert_eq!(book.events, before);
        assert_eq!(sessions.open_run.as_ref().unwrap().1, old);
        sessions.move_run_to_session(&mut book, run, new);
        sessions.draft.as_mut().unwrap().body.push_str("A note.");
        sessions.expire_move(&book);
        assert!(sessions.moved_run.is_none() && sessions.notice.is_none());
        sessions.move_run_to_session(&mut book, run, old);
        book.add(Kind::Travel, Some(new));
        sessions.expire_move(&book);
        assert!(sessions.moved_run.is_none() && sessions.notice.is_none());
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
}
