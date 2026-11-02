//! The Sessions page: notes, templates and recordings, one campaign at a time. The
//! recorder and the processing job live here, and keep going while the DM uses other pages.
use super::{
    DEFAULT_FOLDER, Draft, DraftState, NewNote, Note, Recording, Template, body_of, create_note,
    create_template, delete_recording, folder as session_folder, folder_setting,
    fresh_line_in_note, insert_in_note, list_notes, list_recordings, list_templates,
    nested_in_note, new_recording_path, parts_of, recording_title, save_folder, unreadable_notes,
    update_details, without_own_heading,
};
use crate::{
    credentials::{self, KeySlot},
    design::{self, Action, Density, Status, Tone, Type, colors, icons},
    markdown_view,
    persistence::Campaign,
    processing::{self, Job, Request, settings::ProcessingSettings},
    recording::{Health, Recorder},
    timeline::{
        self, Book,
        page::{SessionLink, problems, quick_add, undo_removed},
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

const UNDO_ADD_FOR: Duration = Duration::from_secs(30);
const SAVED_WORD_FOR: Duration = Duration::from_secs(8);
const LIST_WIDTH: f32 = 216.0;
const LIST_WIDTHS: std::ops::RangeInclusive<f32> = 170.0..=420.0;
const STATUS_WIDTH: f32 = 128.0;
const TWO_COLUMNS_FROM: f32 = 640.0;
/// About 80 characters of source. A note keeps its shape in a wide window.
const EDITOR_WIDTH: f32 = 680.0;
const CHECK_DISK_EVERY: Duration = Duration::from_secs(1);
const SCAN_EVERY: Duration = Duration::from_secs(5);

#[derive(Clone, Copy, Default, PartialEq)]
enum View {
    #[default]
    Notes,
    Templates,
    Folder,
}

#[derive(Clone, Copy, Default, PartialEq)]
enum Tab {
    #[default]
    Write,
    Read,
    Events,
    Recordings,
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
}

struct Live {
    session: Uuid,
    label: String,
    recorder: Recorder,
}

#[derive(Default)]
pub struct Sessions {
    campaign: Option<Campaign>,
    folder: Option<PathBuf>,
    folder_error: Option<String>,
    folder_text: String,
    folder_saved: bool,
    view: View,
    tab: Tab,
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
    /// The session a notice is about. None means whichever is open now.
    notice_session: Option<Uuid>,
    /// A job ended while the DM may be elsewhere.
    job_done: bool,
    /// It finished with something the DM must look at.
    job_warned: bool,
    alerts: Vec<Alert>,
    added: Option<Added>,
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
    /// When a recording last stopped, so the rail can say it was kept.
    saved_at: Option<Instant>,
    note_had_focus: bool,
    /// The caret to bring into view, on the frame after the editor has laid out.
    caret_to_show: Option<usize>,
    last_check: Option<Instant>,
    last_scan: Option<Instant>,
    /// The campaign's manifest changed, so what holds a copy of it must reload.
    pub manifest_changed: Option<PathBuf>,
    /// The Sessions page is showing, so a button there need not lead here.
    pub on_page: bool,
    pub wants_processing_settings: bool,
    /// The DM asked to see the session this is about.
    pub wants_sessions_page: bool,
    pub wants_event: Option<String>,
}

/// The words for a note that looks like last week's. A forgotten `New session` puts
/// tonight's notes and audio there, and neither has a way to move afterward.
fn dated_before_today(note: &Note, today: &str) -> Option<String> {
    (note.date.as_str() < today).then(|| {
        format!(
            "This session is dated {}. If you play a new one tonight, start it first. Play this one changes its date to today.",
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

/// A list row whose title wraps, because a session's title is often long.
fn session_row(ui: &mut egui::Ui, selected: bool, title: &str, detail: &str) -> egui::Response {
    let c = colors();
    let width = ui.available_width() - 2.0 * design::S;
    let font = if selected { Type::Strong } else { Type::Body };
    let galley = ui
        .painter()
        .layout(title.to_owned(), font.font(ui), c.text, width);
    let lines = galley.size().y + Type::Small.line();
    let height = (lines + design::M).max(Density::Glance.control() + design::S);
    design::row(
        ui,
        height,
        selected,
        &format!("{title} · {detail}"),
        |ui, rect| {
            let top = rect.center().y - lines / 2.0;
            let x = rect.left() + design::S;
            let first = galley.size().y;
            ui.painter().galley(egui::pos2(x, top), galley, c.text);
            let second = design::one_line(ui, detail, Type::Small.font(ui), c.muted, width);
            ui.painter()
                .galley(egui::pos2(x, top + first), second, c.muted);
        },
    )
}

fn clock(seconds: u64) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        (seconds / 60) % 60,
        seconds % 60
    )
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
    /// Follow the active campaign, watch the recorder and the job, and save what is due.
    pub fn poll(&mut self, ctx: &egui::Context, active: Option<&Campaign>, book: &mut Book) {
        if self.campaign.as_ref().map(|c| (c.id, &c.path)) != active.map(|c| (c.id, &c.path)) {
            self.open_campaign(active);
        }
        book.open(self.folder.as_deref());
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
                    word: "Recording stopped",
                    words: format!(
                        "The recording of {label} stopped by itself. {kept} Check the microphone, then record again. The recorder said: {words}"
                    ),
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

    /// What cannot wait, on every page. It sits at the bottom, so that it never moves `Next
    /// turn` under a pointer parked on it. A silence takes no banner: it is often only a quiet
    /// table, and it shows in the rail and on the run's own line.
    pub fn banner(&mut self, ctx: &egui::Context, settings: &ProcessingSettings) {
        let stalled = self
            .live
            .as_ref()
            .filter(|live| matches!(live.recorder.health(), Health::Stalled | Health::Stopped(_)))
            .map(|live| {
                format!(
                    "{}: not saving audio after {}. What was saved is kept. Record again starts a new file.",
                    live.label,
                    spoken_length(live.recorder.seconds())
                )
            });
        if stalled.is_none() && self.alerts.is_empty() {
            return;
        }
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
                if let Some(words) = &stalled {
                    let session = self.live.as_ref().map(|live| live.session);
                    design::message(ui, Tone::Error, words);
                    ui.horizontal(|ui| {
                        if Action::primary("Record again").show(ui).clicked() {
                            record_again = session;
                        }
                        let here = self.on_page && self.selected == session;
                        if !here && Action::quiet("Open the session").show(ui).clicked() {
                            open = session;
                        }
                        ui.add_space(design::L);
                        stop = Action::quiet("Stop")
                            .name("Stop without recording again")
                            .show(ui)
                            .clicked();
                    });
                }
                let mut dismiss = None;
                for (index, alert) in self.alerts.iter().enumerate() {
                    design::message(ui, Tone::Error, &alert.words);
                    ui.horizontal(|ui| {
                        if alert.can_record
                            && self.live.is_none()
                            && Action::primary("Record again").show(ui).clicked()
                        {
                            record_again = Some(alert.session);
                        }
                        let here = self.on_page
                            && self.selected == Some(alert.session)
                            && self.tab == Tab::Recordings;
                        if !alert.can_record
                            && !here
                            && Action::secondary("Open the session").show(ui).clicked()
                        {
                            open = Some(alert.session);
                        }
                        ui.add_space(design::L);
                        if Action::quiet("Dismiss this error").show(ui).clicked() {
                            dismiss = Some(index);
                        }
                    });
                }
                if let Some(index) = dismiss {
                    self.alerts.remove(index);
                }
            });
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
            if !draft.unsaved() {
                match self.notes.iter().find(|note| note.id == id) {
                    Some(note) if note.path != draft.path => {
                        self.draft = None;
                        self.select(Some(id));
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
        self.job = None;
        let label = self
            .notes
            .iter()
            .find(|note| note.id == session)
            .map_or_else(|| "this session".to_owned(), Note::label);
        self.notice_session = Some(session);
        match result {
            Ok(None) => {
                self.job_done = true;
                self.job_warned = false;
                self.notice = Some((
                    Tone::Success,
                    format!(
                        "{label} is processed. Read the result, and add the events you want to the timeline."
                    ),
                ));
            }
            Ok(Some(warning)) => {
                self.job_done = true;
                self.job_warned = true;
                // The recording's row has the words, for as long as the result stands.
                let _ = warning;
                self.notice = Some((
                    Tone::Warning,
                    format!("{label} is processed, with a warning."),
                ));
            }
            Err(error) if error == processing::CANCELLED => {
                self.notice = Some((Tone::Info, error));
            }
            Err(error) => {
                self.notice = None;
                self.raise(Alert {
                    session,
                    can_record: false,
                    word: "Processing failed",
                    words: format!("Processing {label} failed. What is done is kept. {error}"),
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
        self.notice = None;
        self.campaign = active.cloned();
        self.folder_error = None;
        match active {
            Some(campaign) => {
                self.folder_text =
                    folder_setting(campaign).unwrap_or_else(|_| DEFAULT_FOLDER.to_owned());
                match session_folder(campaign) {
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
        self.events_to = None;
        self.alerts.clear();
        self.job_done = false;
        self.job_warned = false;
        self.deleted_template = None;
        self.folder_saved = false;
        self.notice_session = None;
        self.select(self.notes.first().map(|note| note.id));
    }

    pub fn refresh(&mut self) {
        let Some(folder) = &self.folder else {
            self.notes.clear();
            self.mentions.clear();
            self.templates.clear();
            return;
        };
        self.notes = list_notes(folder);
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

    pub fn select(&mut self, session: Option<Uuid>) {
        self.flush();
        self.list_only = false;
        // Words that could not be saved stay on screen, with the choice of what to do.
        if self.draft.as_ref().is_some_and(Draft::needs_attention) {
            self.tab = Tab::Write;
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
        self.selected = session;
        self.draft = session
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
            .and_then(|note| Draft::open(&note.path).ok());
        self.reading = None;
        self.added = None;
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

    /// Today, as of when the app started. A game that passes midnight is still tonight.
    fn today(&mut self) -> String {
        self.today
            .get_or_insert_with(crate::hermetic::today)
            .clone()
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
    fn not_being_played(&self, start: &str) -> Option<String> {
        if self.selected.is_none() || self.selected == self.current() {
            return None;
        }
        let number = |id: Option<Uuid>| {
            self.notes
                .iter()
                .find(|note| Some(note.id) == id)
                .map(|note| format!("Session {}", note.number))
        };
        let here = number(self.selected)?;
        Some(match number(self.current()) {
            Some(playing) => format!("{start} {here}, not the session being played ({playing})."),
            None => format!("{start} {here}."),
        })
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

    pub fn recording(&self) -> bool {
        self.live.is_some()
    }

    /// What the rail says while a microphone is live.
    pub fn recording_detail(&self) -> Option<String> {
        let live = self.live.as_ref()?;
        let time = clock(live.recorder.seconds());
        Some(match live.recorder.health() {
            Health::Stalled | Health::Stopped(_) => "Not saving".to_owned(),
            Health::Silent => format!("Silent {time}"),
            _ => format!("Rec {time}"),
        })
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
        let path = live.recorder.stop();
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
                    format!("Recording of {label} saved: {}.", recording_title(&path)),
                )
            } else {
                (Tone::Warning, format!("Nothing was recorded for {label}."))
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
        let Some(session) = self.selected else {
            return;
        };
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
            self.draft.as_ref().map_or("", |draft| draft.body.as_str()),
            events.join("\n")
        );
        let (speech_key, llm_key) = settings.held_keys();
        self.notice = None;
        self.notice_session = None;
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
        if let Some(live) = &self.live {
            Some(format!("Stop the recording of {} first.", live.label))
        } else if self
            .draft
            .as_ref()
            .is_some_and(|draft| draft.state == DraftState::Missing)
        {
            Some("Save your words first.".to_owned())
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
        if let Some(live) = &self.live {
            let name = if Some(live.session) == self.selected {
                "Stop recording".to_owned()
            } else {
                format!("Stop recording of {}", live.label)
            };
            if Action::destructive("Stop recording")
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
    }

    /// The live state, or the step before it. Always in the same place.
    fn record_line(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings) {
        // While a recording runs, the line keeps one height, so the note below never moves.
        let live = self.live.is_some();
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
                Health::Stalled | Health::Stopped(_) => {
                    design::message(
                        ui,
                        Tone::Error,
                        &format!(
                            "Not saving audio after {}. What was saved is kept.",
                            spoken_length(live.recorder.seconds())
                        ),
                    );
                }
                health => {
                    let (word, color) = match health {
                        Health::Starting => ("Waiting for sound", c.muted),
                        Health::Silent => ("Silent for over a minute", c.result.warning),
                        // Only what is true now. A table goes quiet, and that is no warning.
                        _ if live.recorder.heard_just_now() => ("Hearing sound", c.muted),
                        _ => ("Quiet", c.muted),
                    };
                    ui.horizontal_wrapped(|ui| {
                        // The first thing this page must say, large enough for a glance.
                        ui.label(
                            Type::Strong
                                .rich(ui, format!("{} Recording {time}", icons::RECORD))
                                .color(c.recording),
                        );
                        ui.label(Type::Small.rich(ui, word).color(color));
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
        } else if let (Some(live), Some((icon, word, color))) =
            (&self.live, self.recording_status())
        {
            let words = format!(
                "{word} {} in {}",
                clock(live.recorder.seconds()),
                live.label
            );
            design::status(
                ui,
                Status {
                    icon,
                    word: &words,
                    color,
                },
            )
            .on_hover_text(&words);
        } else if settings.input_missing() {
            let mut open = false;
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    Type::Small
                        .rich(ui, "The input chosen in Settings is not connected.")
                        .color(c.result.warning),
                );
                open = Action::quiet("Open Settings")
                    .density(Density::Read)
                    .show(ui)
                    .clicked();
            });
            if open {
                self.wants_processing_settings = true;
            }
        } else if self.confirm_record {
            let label = self
                .selected
                .and_then(|id| self.notes.iter().find(|note| note.id == id))
                .map_or_else(String::new, Note::label);
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
            design::message(
                ui,
                if other.is_empty() {
                    Tone::Info
                } else {
                    Tone::Warning
                },
                &format!(
                    "Record {label}.{other} Tell everyone at the table first. The audio stays in the campaign folder until you select Process."
                ),
            );
            ui.horizontal(|ui| {
                if dated.is_some()
                    && self.selected == self.current()
                    && Action::secondary("Play this one").show(ui).clicked()
                {
                    self.play_this_one();
                }
                if Action::primary("Start recording").show(ui).clicked() {
                    self.confirm_record = false;
                    if let Some(session) = self.selected
                        && let Err(error) = self.start_recording(settings, session)
                    {
                        self.notice_session = None;
                        self.notice = Some((Tone::Error, error));
                    }
                }
                if Action::quiet("Not now").show(ui).clicked() {
                    self.confirm_record = false;
                }
            });
        }
    }

    /// A note's state says it is the note, beside a control about audio.
    fn note_status(ui: &mut egui::Ui, draft: &Draft) {
        let c = colors();
        let (icon, word, color) = match &draft.state {
            DraftState::Clean => (icons::OK, "Notes saved", c.document.clean),
            DraftState::Saved => (icons::OK, "Notes saved", c.document.saved),
            DraftState::Unsaved => (icons::DRAFT, "Notes unsaved", c.document.unsaved),
            DraftState::Conflict => (icons::WARN, "Changed on disk", c.document.external),
            DraftState::Missing => (icons::WARN, "File gone", c.document.external),
            DraftState::Failed(_) => (icons::ERROR, "Notes not saved", c.document.invalid),
        };
        design::status(ui, Status { icon, word, color });
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

    fn draft_problem(ui: &mut egui::Ui, draft: &mut Draft) {
        if draft.dropped.is_some()
            && design::message_with_action(ui, Tone::Info, "Draft dropped.", "Undo reload")
        {
            draft.undo_reload();
        }
        match draft.state.clone() {
            DraftState::Conflict => {
                design::message(
                    ui,
                    Tone::Warning,
                    "This file changed on disk while you were editing. Nothing is saved until you choose, and other sessions open after that.",
                );
                ui.horizontal_wrapped(|ui| {
                    if Action::secondary("Overwrite with my draft")
                        .show(ui)
                        .clicked()
                    {
                        draft.overwrite();
                    }
                    let showing = draft.on_disk.is_some();
                    let label = if showing {
                        "Hide disk version"
                    } else {
                        "Show disk version"
                    };
                    if Action::quiet(label).show(ui).clicked() {
                        if showing {
                            draft.on_disk = None;
                        } else {
                            draft.show_disk();
                        }
                    }
                    if Action::quiet("Reload and lose my draft").show(ui).clicked()
                        && draft.reload_keeping_undo().is_err()
                    {
                        draft.state = DraftState::Missing;
                    }
                });
                if let Some(words) = &draft.on_disk {
                    design::group_label(ui, "In the file now");
                    egui::ScrollArea::vertical()
                        .id_salt("note_on_disk")
                        .max_height(160.0)
                        .show(ui, |ui| {
                            ui.add(
                                egui::Label::new(Type::Mono.rich(ui, words).color(colors().muted))
                                    .wrap(),
                            );
                        });
                }
            }
            DraftState::Missing => {
                design::message(
                    ui,
                    Tone::Warning,
                    "This file is gone from the folder. It may have been moved or renamed. Your words are still here. Its recordings and events stay with the old session, and do not move to a new file. Other sessions open after you save.",
                );
                if Action::secondary("Save my words as a new file")
                    .show(ui)
                    .clicked()
                {
                    draft.save_again();
                }
            }
            DraftState::Failed(error) => {
                design::message(ui, Tone::Error, &error);
                if Action::secondary("Try to save again").show(ui).clicked() {
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
        ui.label(Type::Title.rich(ui, heading).color(c.text));
        ui.add_space(design::XS);
        design::text_field(ui, "Number", &mut form.number, None, None);
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
            done |= Action::primary(label).show(ui).clicked();
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
                    self.tab = Tab::Write;
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
        let Some((name, session)) = self.open_run.clone().filter(|(_, session)| *session != new)
        else {
            return;
        };
        let Some(note) = self.notes.iter().find(|note| note.id == session) else {
            return;
        };
        self.notice = Some((
            Tone::Info,
            format!(
                "{name} stays in {}. To move it, change its Session on the Timeline.",
                note.label()
            ),
        ));
        self.notice_session = Some(new);
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
                        let result = self.notice_session.filter(|_| self.job_done);
                        let detail = if live == Some(note.id) {
                            let state = self
                                .recording_status()
                                .map_or("Recording", |(_, word, _)| word);
                            format!("{} · {state}", note.date)
                        } else if job == Some(note.id) {
                            format!("{} · Processing", note.date)
                        } else if result == Some(note.id) {
                            let word = if self.job_warned {
                                "Processed: check it"
                            } else {
                                "Processed"
                            };
                            format!("{} · {word}", note.date)
                        } else {
                            note.date.clone()
                        };
                        if session_row(ui, self.selected == Some(note.id), &note.label(), &detail)
                            .clicked()
                        {
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

    fn write_tab(&mut self, ui: &mut egui::Ui) {
        self.note_editor(ui, false);
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
        if in_panel {
            Self::draft_problem(ui, draft);
        }
        if draft.on_disk.is_some() {
            design::group_label(ui, "Your draft");
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
        egui::ScrollArea::vertical()
            .id_salt(("session_editor", self.selected, in_panel))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Beside a page, Tab indents. It must not walk to `Next turn`, where the next
                // space would press it. Escape still leaves the panel.
                let output = egui::TextEdit::multiline(&mut draft.body)
                    .lock_focus(in_panel)
                    .font(egui::TextStyle::Monospace)
                    .margin(egui::vec2(design::S, design::S))
                    .desired_width(ui.available_width().min(EDITOR_WIDTH))
                    .min_size(egui::vec2(0.0, ui.available_height()))
                    .show(ui);
                let response = output.response;
                design::named(ui, &response, "Note text");
                self.note_had_focus = response.has_focus();
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
                        .pos_from_cursor(egui::text::CCursor::new(caret))
                        .translate(output.galley_pos.to_vec2());
                    ui.scroll_to_rect(rect, Some(egui::Align::Center));
                }
                if focus {
                    response.request_focus();
                    let (caret, changed) = fresh_line_in_note(&mut draft.body);
                    if changed {
                        draft.edited();
                    }
                    self.caret_to_show = Some(caret);
                    let end = egui::text::CCursor::new(caret);
                    let mut state =
                        egui::TextEdit::load_state(ui.ctx(), response.id).unwrap_or_default();
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
        let in_page_editor = self.on_page && self.tab == Tab::Write && self.note_had_focus;
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
                self.tab = Tab::Write;
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
                if design::panel_header(ui, &title, "Close notes") {
                    self.notes_open = false;
                    return;
                }
                if self.selected.is_none() {
                    design::message(ui, Tone::Info, "No session yet.");
                    if Action::primary("New session").show(ui).clicked() {
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
                let today = self.today();
                let note = self
                    .selected
                    .and_then(|id| self.notes.iter().find(|note| note.id == id));
                let mut play = false;
                let mut start = false;
                if let Some(note) = note {
                    ui.label(Type::Small.rich(ui, &note.date).color(colors().muted));
                    if let Some(words) = dated_before_today(note, &today) {
                        design::message(ui, Tone::Warning, &words);
                        ui.horizontal(|ui| {
                            play = Action::secondary("Play this one")
                                .density(Density::Read)
                                .show(ui)
                                .clicked();
                            start = Action::secondary("New session")
                                .density(Density::Read)
                                .show(ui)
                                .clicked();
                        });
                    }
                }
                if play {
                    self.play_this_one();
                }
                if start {
                    self.start_new_session();
                }
                if let Some(draft) = &self.draft {
                    Self::note_status(ui, draft);
                }
                self.note_editor(ui, true);
            });
    }

    fn read_tab(&mut self, ui: &mut egui::Ui, book: &Book) {
        let Some(draft) = &self.draft else {
            return;
        };
        let note = self
            .selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id));
        let body = note.map_or_else(
            || draft.body.clone(),
            |note| without_own_heading(&draft.body, note),
        );
        let text = timeline::for_reading(&body, &book.events);
        let linked = timeline::referenced_ids(&draft.body);
        let days = timeline::day_numbers(&book.events);
        let session = self.selected;
        egui::ScrollArea::vertical()
            .id_salt(("session_reader", self.selected))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                design::reader::measure(ui, |ui| {
                    markdown_view::show_reader(ui, &text);
                    let events: Vec<_> = book
                        .events
                        .iter()
                        .zip(&days)
                        .filter(|(event, _)| event.session == session || linked.contains(&event.id))
                        .collect();
                    if !events.is_empty() {
                        ui.add_space(design::L);
                        design::group_label(ui, "On the timeline");
                        for (event, day) in events {
                            if design::link(ui, &event.label(*day)).clicked() {
                                self.wants_event = Some(event.id.clone());
                            }
                        }
                    }
                });
            });
    }

    fn events_tab(&mut self, ui: &mut egui::Ui, book: &mut Book) {
        let c = colors();
        let session = self.selected;
        problems(ui, book);
        if let Some(words) = self.not_being_played("These go to") {
            design::message(ui, Tone::Warning, &words);
        } else if let Some(words) = self.current_dated()
            && design::message_with_action(ui, Tone::Warning, &words, "Play this one")
        {
            self.play_this_one();
        }
        // Stay here. At the table, a rest is one press, and its details can wait.
        let added = quick_add(ui, book, session);
        let days = timeline::day_numbers(&book.events);
        if let Some(id) = &added {
            let index = book.events.iter().position(|event| event.id == *id);
            self.added = Some(Added {
                id: id.clone(),
                label: index.map_or_else(String::new, |at| book.events[at].label(days[at])),
                at: Instant::now(),
                count: book.events.len(),
            });
        }
        // It goes at the DM's next edit, or soon, so that it cannot undo the wrong thing.
        if let Some(just) = &self.added {
            let current = book
                .events
                .iter()
                .position(|event| event.id == just.id)
                .map(|at| book.events[at].label(days[at]));
            if just.at.elapsed() >= UNDO_ADD_FOR
                || just.count != book.events.len()
                || current.as_ref() != Some(&just.label)
            {
                self.added = None;
            } else {
                ui.ctx().request_repaint_after(UNDO_ADD_FOR);
            }
        }
        if let Some(just) = &self.added {
            let to = self
                .notes
                .iter()
                .find(|note| Some(note.id) == session)
                .map_or_else(String::new, |note| format!(" to Session {}", note.number));
            let (id, words) = (just.id.clone(), format!("{} added{to}.", just.label));
            if design::message_with_action(ui, Tone::Success, &words, "Undo add") {
                book.remove(&id);
                self.added = None;
            }
        } else if !undo_removed(ui, book) {
            // Its line stays reserved, so that the rows below do not move under the pointer.
            ui.add_space(Density::Read.control() + ui.spacing().item_spacing.y);
        }
        ui.add_space(design::XS);
        let days = timeline::day_numbers(&book.events);
        let mut insert = None;
        let body = self.draft.as_ref().map_or("", |draft| draft.body.as_str());
        egui::ScrollArea::vertical()
            .id_salt(("session_events", session))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let action = 104.0;
                let widths = [
                    ui.available_width().min(EDITOR_WIDTH) - action - design::XL,
                    action,
                ];
                let mut any = false;
                for (event, day) in book.events.iter().zip(&days) {
                    if event.session != session {
                        continue;
                    }
                    any = true;
                    let label = event.label(*day);
                    let linked = body.contains(&format!("^{}", event.id));
                    design::table_row(ui, &widths, design::S, |ui, cell| {
                        if cell == 0 {
                            ui.horizontal_top(|ui| {
                                ui.label(Type::Body.rich(ui, event.kind.icon()).color(c.muted));
                                ui.vertical(|ui| {
                                    if design::wrapped_link(ui, &label).clicked() {
                                        self.wants_event = Some(event.id.clone());
                                    }
                                    let summary = event.summary(None);
                                    if !summary.is_empty() {
                                        ui.add(
                                            egui::Label::new(
                                                Type::Small.rich(ui, summary).color(c.muted),
                                            )
                                            .wrap(),
                                        );
                                    }
                                });
                            });
                        } else {
                            // One height either way, so that the rows below never move.
                            ui.allocate_ui_with_layout(
                                egui::vec2(ui.available_width(), Density::Read.control()),
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if linked {
                                        design::status(
                                            ui,
                                            Status {
                                                icon: icons::OK,
                                                word: "In notes",
                                                color: c.document.saved,
                                            },
                                        );
                                    } else if Action::quiet("Add to notes")
                                        .name(&format!("Add to notes: {label}"))
                                        .density(Density::Read)
                                        .show(ui)
                                        .clicked()
                                    {
                                        insert = Some(timeline::reference(event, *day));
                                    }
                                },
                            );
                        }
                    });
                }
                if added.is_some() {
                    ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                }
                if !any {
                    ui.add(
                        egui::Label::new(
                            Type::Body
                                .rich(
                                    ui,
                                    "No event belongs to this session yet. Add one above. An encounter run adds itself.",
                                )
                                .color(c.muted),
                        )
                        .wrap(),
                    );
                }
            });
        if let (Some(link), Some(draft), Some(session)) = (insert, &mut self.draft, session) {
            insert_in_note(&mut draft.body, &link);
            draft.edited();
            self.added = None;
            self.mentions
                .insert(session, timeline::referenced_ids(&draft.body));
        }
    }

    fn recording_row(
        &mut self,
        ui: &mut egui::Ui,
        recording: &Recording,
        settings: &ProcessingSettings,
        book: &Book,
    ) {
        let c = colors();
        let live = self
            .live
            .as_ref()
            .is_some_and(|live| live.recorder.path() == recording.path);
        let length = recording
            .seconds
            .map_or_else(|| "Unknown length".to_owned(), spoken_length);
        ui.label(Type::Strong.rich(ui, &recording.title).color(c.text));
        let processing = self
            .job
            .as_ref()
            .is_some_and(|(job, _)| job.recording == recording.path);
        let state = if live {
            self.recording_status()
                .map_or("Recording", |(_, word, _)| word)
        } else if processing {
            "Processing"
        } else if recording.warning.is_some() {
            "Processed, with a warning"
        } else if recording.summary {
            "Processed"
        } else if recording.transcript {
            "Partly processed"
        } else {
            "Not processed"
        };
        ui.label(
            Type::Small
                .rich(
                    ui,
                    format!("{length} · {} · {state}", megabytes(recording.bytes)),
                )
                .color(c.muted),
        );
        if let Some(warning) = &recording.warning {
            design::message(ui, Tone::Warning, warning);
        }
        let job_here = self
            .job
            .as_ref()
            .is_some_and(|(job, _)| job.recording == recording.path);
        if job_here {
            let words = self
                .job
                .as_ref()
                .map(|(job, _)| job.progress.words())
                .unwrap_or_default();
            ui.horizontal(|ui| {
                design::spinner(ui);
                ui.label(Type::Small.rich(ui, words).color(c.muted));
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
            });
        }
        let mut open = None;
        let views: Vec<Made> = [
            (Made::Summary, recording.summary),
            (Made::Marked, recording.marked),
            (Made::Review, recording.review),
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
                }
            });
        }
        let mut process = false;
        let mut again = false;
        let mut delete = false;
        ui.horizontal_wrapped(|ui| {
            let reason = if live {
                Some("Stop recording first.".to_owned())
            } else if self.job.is_some() {
                Some("One recording is processed at a time.".to_owned())
            } else {
                settings.problem()
            };
            if !job_here {
                let label = if recording.summary {
                    "Process again"
                } else if recording.transcript {
                    "Continue processing"
                } else {
                    "Process"
                };
                let pressed = Action::secondary(label)
                    .name(&format!("{label}: {}", recording.name))
                    .density(Density::Read)
                    .blocked(reason.as_deref())
                    .show(ui)
                    .clicked();
                process = pressed && !recording.summary;
                again = pressed && recording.summary;
                ui.add_space(design::L);
            }
            delete = Action::quiet("Delete")
                .name(&format!("Delete recording: {}", recording.name))
                .density(Density::Read)
                .blocked_if(live || job_here, "This recording is in use.")
                .show(ui)
                .clicked();
        });
        if self.confirm_again.as_deref() == Some(recording.path.as_path()) {
            design::message(
                ui,
                Tone::Info,
                "This asks the LLM again, and replaces the summary, the clean transcript and what the check changed. The raw transcript stays. Events you added stay on the timeline, and a reworded suggestion can offer Add again.",
            );
            ui.horizontal(|ui| {
                process = Action::primary("Replace the results").show(ui).clicked();
                if Action::quiet("Not now").show(ui).clicked() {
                    self.confirm_again = None;
                }
            });
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
        design::group_label(
            ui,
            &format!(
                "Events marked in this recording: {}",
                recording.markers.len()
            ),
        );
        ui.label(
            Type::Small
                .rich(
                    ui,
                    "These are suggestions from the LLM. Only what you add goes on the timeline.",
                )
                .color(c.muted),
        );
        let action = 120.0;
        let widths = [ui.available_width() - action - design::XL, action];
        let mut read_at: Option<(String, crate::timeline::Marker, String)> = None;
        let days = timeline::day_numbers(&book.events);
        for (index, marker) in recording.markers.iter().enumerate() {
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
            design::table_row(ui, &widths, design::S, |ui, cell| {
                if cell == 0 {
                    ui.horizontal_top(|ui| {
                        ui.label(Type::Body.rich(ui, marker.kind.icon()).color(c.muted));
                        ui.vertical(|ui| {
                            ui.add(
                                egui::Label::new(Type::Body.rich(ui, title).color(c.text)).wrap(),
                            );
                            ui.horizontal_wrapped(|ui| {
                                if !marker.at.is_empty()
                                    && recording.marked
                                    && design::link(ui, &marker.at)
                                        .on_hover_text("Read the transcript here")
                                        .clicked()
                                {
                                    read_at =
                                        Some((marker.at.clone(), marker.clone(), key.clone()));
                                }
                                let mut facts = facts.join(" · ");
                                if let Some(id) = &added {
                                    let at = book.events.iter().position(|event| event.id == *id);
                                    let before = at.and_then(|at| at.checked_sub(1));
                                    facts.push_str(&before.map_or_else(
                                        || " · First on the timeline".to_owned(),
                                        |before| {
                                            format!(
                                                " · After {}",
                                                book.events[before].label(days[before])
                                            )
                                        },
                                    ));
                                }
                                ui.add(
                                    egui::Label::new(Type::Small.rich(ui, facts).color(c.muted))
                                        .wrap(),
                                );
                            });
                        });
                    });
                } else {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                        if let Some(id) = &added {
                            if design::link(ui, "On the timeline").clicked() {
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
                            book.add_marker(marker, &key, session);
                        }
                    });
                }
            });
        }
        if let Some((at, marker, key)) = read_at {
            self.open_reading(recording, Made::Marked, Some(&at), Some((marker, key)));
        }
    }

    /// One text from a recording, in the whole pane, one part at a time. A 5-hour transcript
    /// is about 30 parts of a size the Reader handles.
    fn reading_view(&mut self, ui: &mut egui::Ui, book: &mut Book) {
        let c = colors();
        let session = self.selected;
        let Some(reading) = &mut self.reading else {
            return;
        };
        let mut back = false;
        let mut add_to_notes = None;
        let in_notes = self.draft.as_ref().map_or("", |draft| draft.body.as_str());
        ui.horizontal(|ui| {
            back = Action::quiet("Recordings")
                .icon(icons::BACK)
                .name("Back to recordings")
                .density(Density::Read)
                .show(ui)
                .clicked();
            ui.label(Type::Heading.rich(ui, reading.made.word()).color(c.text));
            ui.label(Type::Small.rich(ui, &reading.name).color(c.muted));
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
        });
        if let Some((done, parts)) = reading.short_of {
            design::message(
                ui,
                Tone::Warning,
                &format!(
                    "Transcribed so far: {done} of {parts} parts. Continue processing adds the rest."
                ),
            );
        }
        if matches!(reading.made, Made::Marked | Made::Review) {
            ui.label(
                Type::Small
                    .rich(
                        ui,
                        "Written by an LLM. The raw transcript has the words as heard.",
                    )
                    .color(c.muted),
            );
        }
        if let Some((marker, key)) = &reading.marker {
            ui.horizontal_wrapped(|ui| {
                ui.label(Type::Body.rich(ui, marker.kind.icon()).color(c.muted));
                ui.label(
                    Type::Body
                        .rich(ui, marker.reading_words().replace('*', ""))
                        .color(c.text),
                );
                if book.event_of_marker(key).is_some() {
                    design::status(
                        ui,
                        Status {
                            icon: icons::OK,
                            word: "On the timeline",
                            color: c.document.saved,
                        },
                    );
                } else if Action::secondary("Add")
                    .name("Add this event to the timeline")
                    .density(Density::Read)
                    .blocked_if(
                        book.unreadable.is_some(),
                        "The timeline file must be repaired first.",
                    )
                    .show(ui)
                    .clicked()
                {
                    book.add_marker(marker, key, session);
                }
            });
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
                design::reader::measure(ui, |ui| {
                    let body = &reading.parts[reading.part].1;
                    let at = reading.mark.as_deref().and_then(|mark| body.find(mark));
                    match at {
                        Some(at) => {
                            markdown_view::show_reader(ui, &body[..at]);
                            ui.scroll_to_cursor(Some(egui::Align::TOP));
                            markdown_view::show_reader(ui, &body[at..]);
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
        if let Some(problem) = settings.problem() {
            if design::message_with_action(ui, Tone::Info, &problem, "Open Settings") {
                self.wants_processing_settings = true;
            }
        } else {
            let config = &settings.config;
            let llm = if config.provider == processing::Provider::CodexCli {
                format!("Codex ({})", config.llm_model.trim())
            } else {
                processing::host(&config.llm_url)
            };
            let width = ui.available_width().min(EDITOR_WIDTH);
            ui.allocate_ui_with_layout(
                egui::vec2(width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(
                            ui,
                            format!(
                                "Process sends the audio and the session and campaign names to {} for a transcript. Then the transcript, this session's name, notes and events go to {llm} twice: once to clean them, and once to check that work against the raw words.",
                                processing::host(&config.speech_url),
                            ),
                        )
                        .color(colors().muted),
                )
                .wrap(),
            );
                },
            );
        }
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
                format!("No recordings of this session. {label} is recording now.")
            } else if let Some(reason) = &reason {
                format!("No recordings of this session. {reason}")
            } else {
                "No recordings of this session. Select Record above to make one.".to_owned()
            };
            ui.add(egui::Label::new(Type::Body.rich(ui, words).color(colors().muted)).wrap());
            if !here
                && self.live.is_none()
                && reason.is_some()
                && Action::quiet("Open Settings").show(ui).clicked()
            {
                self.wants_processing_settings = true;
            }
            return;
        }
        egui::ScrollArea::vertical()
            .id_salt(("session_recordings", self.selected))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Words and their action stay near each other in a wide window.
                let width = ui.available_width().min(EDITOR_WIDTH);
                ui.allocate_ui_with_layout(
                    egui::vec2(width, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_width(width);
                        for recording in &recordings {
                            ui.add_space(design::S);
                            self.recording_row(ui, recording, settings, book);
                            self.markers(ui, recording, book);
                            // Each marker row ends in a rule already.
                            if recording.markers.is_empty() {
                                design::rule(ui);
                            }
                        }
                    },
                );
            });
    }

    fn delete_dialog(&mut self, ctx: &egui::Context) {
        let Some(path) = self.confirm_delete.clone() else {
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
            ui.horizontal(|ui| {
                keep |= Action::primary("Keep the recording").show(ui).clicked();
                ui.add_space(design::L);
                delete = Action::destructive("Delete recording").show(ui).clicked();
            });
        });
        if keep {
            self.confirm_delete = None;
        }
        if delete {
            self.confirm_delete = None;
            self.reading = None;
            self.notice_session = None;
            self.notice = Some(match delete_recording(&path) {
                Ok(()) => (Tone::Success, "Recording deleted.".to_owned()),
                Err(error) => (Tone::Error, format!("Not deleted. {error}")),
            });
            self.refresh_recordings();
        }
    }

    fn session_pane(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings, book: &mut Book) {
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
            if let Some(draft) = &mut self.draft {
                Self::draft_problem(ui, draft);
                self.note_editor(ui, false);
            }
            return;
        };
        // The controls first, and the title in what is left. A long title wraps.
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            self.record_control(ui, settings);
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
                // Centred on the buttons beside it, though the row aligns to the top.
                ui.allocate_ui_with_layout(
                    egui::vec2(STATUS_WIDTH, Density::Scan.control()),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| Self::note_status(ui, draft),
                );
            }
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                ui.add(egui::Label::new(Type::Title.rich(ui, note.label()).color(c.text)).wrap());
            });
        });
        ui.label(Type::Small.rich(ui, &note.date).color(c.muted));
        self.record_line(ui, settings);
        if let Some(draft) = &mut self.draft {
            Self::draft_problem(ui, draft);
        }
        let shown = self.notice_session.is_none() || self.notice_session == self.selected;
        if shown && self.job_done {
            self.job_done = false;
            self.job_warned = false;
        }
        if let Some((tone, words)) = self.notice.clone().filter(|_| shown) {
            ui.horizontal_top(|ui| {
                let button = Density::Read.control() + design::S;
                let width = ui.available_width() - button;
                ui.allocate_ui_with_layout(
                    egui::vec2(width, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| design::message(ui, tone, &words),
                );
                if design::icon_button(ui, icons::CLOSE, "Dismiss").clicked() {
                    self.notice = None;
                    self.notice_session = None;
                }
            });
        }
        if let Some(copy) = note.also.first() {
            design::message(
                ui,
                Tone::Warning,
                &format!(
                    "Another file claims this session: {}. Splat edits {}. Remove or rename the other one.",
                    copy.file_name().unwrap_or_default().to_string_lossy(),
                    note.path.file_name().unwrap_or_default().to_string_lossy()
                ),
            );
        }
        let today = self.today();
        if self.tab == Tab::Write
            && self.selected == self.current()
            && let Some(words) = dated_before_today(&note, &today)
            && design::message_with_action(ui, Tone::Warning, &words, "Play this one")
        {
            self.play_this_one();
        }
        if self.tab == Tab::Write
            && let Some(words) = self.not_being_played("You are writing in")
        {
            design::message(ui, Tone::Warning, &words);
        }
        let recordings = format!("Recordings ({})", self.recordings.len());
        let tab_changed = design::segmented(
            ui,
            &mut self.tab,
            &[
                (Tab::Write, "Write"),
                (Tab::Read, "Read"),
                (Tab::Events, "Events"),
                (Tab::Recordings, recordings.as_str()),
            ],
        );
        if tab_changed {
            self.clear_success();
        }
        ui.add_space(design::XS);
        match self.tab {
            Tab::Write => self.write_tab(ui),
            Tab::Read => self.read_tab(ui, book),
            Tab::Events => self.events_tab(ui, book),
            Tab::Recordings => self.recordings_tab(ui, settings, book),
        }
    }

    fn notes_view(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings, book: &mut Book) {
        if !self.unreadable.is_empty() {
            design::message(
                ui,
                Tone::Warning,
                &format!(
                    "Splat cannot read the properties of {}, so it is not listed. The file is safe. Repair it to bring it back.",
                    self.unreadable.join(", ")
                ),
            );
        }
        let holding = self.draft.as_ref().is_some_and(Draft::needs_attention);
        if self.notes.is_empty() && self.form.is_none() && !holding {
            let mut start = false;
            design::empty_state(
                ui,
                icons::SESSIONS,
                "No session notes yet",
                "Start a session from a template. Notes are Markdown files in the campaign folder.",
                |ui| {
                    start = Action::primary("New session").show(ui).clicked();
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
        if ui.available_width() < TWO_COLUMNS_FROM {
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
                    self.session_pane(ui, settings, book);
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
                    // Words and their actions stay together in a wide window.
                    let width = ui.available_width().min(EDITOR_WIDTH + design::XL);
                    let height = ui.available_height();
                    ui.allocate_ui_with_layout(
                        egui::vec2(width, height),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(width);
                            self.session_pane(ui, settings, book);
                        },
                    );
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
        let c = colors();
        if self.template.is_none()
            && let Some(first) = self.templates.first().map(|t| t.name.clone())
        {
            self.select_template(&first);
        }
        let mut choose = None;
        let mut start_new = false;
        // Beside Lookup at a small window, two columns would crush the editor.
        let narrow = ui.available_width() < TWO_COLUMNS_FROM;
        if narrow && self.templates_list_only {
            self.template_list(ui, &mut choose, &mut start_new);
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
                .show_inside(ui, |ui| self.template_list(ui, &mut choose, &mut start_new));
        }
        if start_new {
            self.templates_list_only = false;
            self.new_template = Some(String::new());
            self.template_error = None;
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
                ui.add(
                    egui::Label::new(
                        Type::Small
                            .rich(
                                ui,
                                "A new session starts from a template. Splat fills in {{number}}, {{title}}, {{date}} and {{campaign}}.",
                            )
                            .color(c.muted),
                    )
                    .wrap(),
                );
                if let Some((name, _)) = self.deleted_template.clone()
                    && design::message_with_action(
                        ui,
                        Tone::Info,
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
                if let Some(error) = &self.template_error {
                    design::message(ui, Tone::Error, error);
                }
                // One thing at a time, so that the DM does not type into another template.
                if self.new_template.is_some() {
                    self.new_template_form(ui);
                } else {
                    self.template_editor(ui);
                }
            });
    }

    fn template_list(
        &mut self,
        ui: &mut egui::Ui,
        choose: &mut Option<String>,
        start_new: &mut bool,
    ) {
        *start_new = Action::secondary("New template")
            .icon(icons::ADD)
            .density(Density::Read)
            .show(ui)
            .clicked();
        ui.add_space(design::XS);
        egui::ScrollArea::vertical()
            .id_salt("template_list")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for template in &self.templates {
                    let kind = if template.path.is_some() {
                        "This campaign"
                    } else {
                        "Built-in"
                    };
                    if design::two_line_row(
                        ui,
                        Density::Glance.control() + design::S,
                        self.template.as_deref() == Some(template.name.as_str()),
                        &template.name,
                        kind,
                    )
                    .clicked()
                    {
                        *choose = Some(template.name.clone());
                    }
                }
            });
    }

    fn new_template_form(&mut self, ui: &mut egui::Ui) {
        let Some(mut name) = self.new_template.take() else {
            return;
        };
        design::text_field(ui, "Template name", &mut name, None, None);
        let mut keep = true;
        ui.horizontal(|ui| {
            if Action::primary("Create template").show(ui).clicked()
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
                    delete = Action::quiet("Delete")
                        .name("Delete this template")
                        .density(Density::Read)
                        .show(ui)
                        .clicked();
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
        if let Some(draft) = &mut self.template_draft {
            if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)) {
                draft.save();
            }
            Self::draft_problem(ui, draft);
        }
        egui::ScrollArea::vertical()
            .id_salt(("template_editor", &template.name))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if let Some(draft) = &mut self.template_draft {
                    let response = ui.add(
                        egui::TextEdit::multiline(&mut draft.body)
                            .font(egui::TextStyle::Monospace)
                            .margin(egui::vec2(design::S, design::S))
                            .desired_width(ui.available_width().min(EDITOR_WIDTH))
                            .min_size(egui::vec2(0.0, ui.available_height())),
                    );
                    design::named(ui, &response, "Template text");
                    if response.changed() {
                        draft.edited();
                        self.deleted_template = None;
                    }
                } else {
                    ui.label(Type::Mono.rich(ui, &template.body).color(c.muted));
                }
            });
    }

    fn folder_view(&mut self, ui: &mut egui::Ui, book: &Book) {
        let c = colors();
        let Some(campaign) = self.campaign.clone() else {
            return;
        };
        ui.add_space(design::XS);
        let edited = design::text_field(
            ui,
            "Session folder",
            &mut self.folder_text,
            Some(
                "Inside the campaign folder, unless the path is absolute. A name such as 30 Sessions is fine.",
            ),
            self.folder_error.as_deref(),
        )
        .changed();
        if edited {
            self.folder_saved = false;
        }
        if let Some(folder) = &self.folder {
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(ui, format!("Now: {}", folder.display()))
                        .color(c.muted),
                )
                .wrap(),
            );
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(ui, "Notes, Timeline.md, Templates and Recordings are all here. Changing this does not move files.")
                        .color(c.muted),
                )
                .wrap(),
            );
        }
        ui.add_space(design::XS);
        let blocked = if self.live.is_some() {
            Some("Stop the recording first.")
        } else if self.job.is_some() {
            Some("Wait for processing to finish.")
        } else if self.busy_with_edit() || book.not_saved() {
            Some("Choose what to do with the unsaved file first.")
        } else {
            None
        };
        if Action::primary("Save folder")
            .blocked(blocked)
            .show(ui)
            .clicked()
        {
            self.flush();
            // The save just now can fail. Those words stay on screen, as `select` keeps them.
            if self.draft.as_ref().is_some_and(Draft::needs_attention) {
                self.folder_error = Some(
                    "Not changed. Your note could not be saved. Choose what to do with it first."
                        .to_owned(),
                );
                return;
            }
            match save_folder(&campaign, &self.folder_text) {
                Ok(folder) => {
                    self.folder = Some(folder);
                    self.folder_error = None;
                    self.folder_saved = true;
                    self.manifest_changed = Some(campaign.path.clone());
                    self.draft = None;
                    self.template_draft = None;
                    self.template = None;
                    self.refresh();
                    self.select(self.notes.first().map(|note| note.id));
                }
                Err(error) => {
                    self.folder_error = Some(error);
                    self.folder_saved = false;
                }
            }
        }
        if self.folder_saved {
            ui.label(Type::Small.rich(ui, "Saved").color(c.document.saved));
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings, book: &mut Book) {
        let campaign = self
            .campaign
            .as_ref()
            .map_or_else(String::new, |campaign| campaign.name.clone());
        let mut new_session = false;
        let in_form = self.form.is_some();
        let empty = self.notes.is_empty();
        let locked = self.draft.as_ref().is_some_and(Draft::needs_attention);
        let mut stop = false;
        design::page_header(ui, "Sessions", Some(&campaign), |ui| {
            let list_alone = self.list_only && ui.ctx().content_rect().width() < 1500.0;
            if self.live.is_some() && (self.view != View::Notes || in_form || list_alone || empty) {
                stop = Action::destructive("Stop recording")
                    .icon(icons::STOP_RECORDING)
                    .show(ui)
                    .clicked();
            }
            if self.view == View::Notes && !in_form && !empty {
                new_session = Action::secondary("New session")
                    .icon(icons::ADD)
                    .blocked_if(locked, "Choose what to do with this note first.")
                    .show(ui)
                    .clicked();
            }
        });
        if stop {
            self.finish_recording();
        }
        if new_session {
            self.open_form();
        }
        design::segmented(
            ui,
            &mut self.view,
            &[
                (View::Notes, "Session notes"),
                (View::Templates, "Templates"),
                (View::Folder, "Folder"),
            ],
        );
        ui.add_space(design::XS);
        if self.folder.is_none() && self.view != View::Folder {
            design::message(
                ui,
                Tone::Error,
                self.folder_error
                    .as_deref()
                    .unwrap_or("The session folder is not available."),
            );
            return;
        }
        match self.view {
            View::Notes => self.notes_view(ui, settings, book),
            View::Templates => self.templates_view(ui),
            View::Folder => self.folder_view(ui, book),
        }
        self.delete_dialog(ui.ctx());
    }
}

#[cfg(test)]
impl Sessions {
    pub fn busy(&self) -> bool {
        self.job.as_ref().is_some_and(|(job, _)| !job.is_held())
    }

    /// The open note's file is renamed outside Splat, as Obsidian can do.
    pub fn simulate_file_renamed(&mut self) {
        let draft = self.draft.as_ref().expect("an open note");
        let new = draft.path.with_file_name("Renamed in Obsidian.md");
        fs::rename(&draft.path, new).expect("the note");
    }

    /// The open note changes outside Splat, as when Obsidian edits the same file.
    pub fn simulate_disk_change(&mut self) {
        let draft = self.draft.as_ref().expect("an open note");
        let text = fs::read_to_string(&draft.path).expect("the note");
        fs::write(&draft.path, format!("{text}\nWritten in Obsidian.\n")).expect("the note");
    }

    /// A job that stays in progress, for a capture of that state.
    pub fn simulate_job(&mut self, progress: processing::Progress) {
        let session = self.selected.expect("an open session");
        let path = self.recordings[0].path.clone();
        self.job = Some((Job::held(path, progress), session));
    }

    /// A job that ends with a warning, stored as a real one is.
    pub fn simulate_job_end_warned(&mut self, words: &str) {
        let (job, session) = self.job.take().expect("a job");
        fs::write(processing::warning_path(&job.recording), words).expect("the warning");
        self.finish_job(Ok(Some(words.to_owned())), session);
    }

    pub fn simulate_job_end(&mut self, result: Result<Option<String>, String>) {
        let (_, session) = self.job.take().expect("a job");
        self.finish_job(result, session);
    }

    pub fn simulate_recorder_health(&mut self, health: Health) {
        if let Some(live) = &mut self.live {
            live.recorder.simulate_health(health);
        }
    }
}
