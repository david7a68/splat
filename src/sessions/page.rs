//! The Sessions page: notes, templates and recordings, one campaign at a time. The
//! recorder and the processing job live here, and keep going while the DM uses other pages.
use super::{
    DEFAULT_FOLDER, Draft, DraftState, NewNote, Note, Recording, Template, body_of, create_note,
    create_template, delete_recording, folder as session_folder, folder_setting, list_notes,
    list_recordings, list_templates, new_recording_path, save_folder, update_details,
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
        page::{SessionLink, problems, quick_add},
    },
};
use eframe::egui;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use uuid::Uuid;

const LIST_WIDTH: f32 = 216.0;
const CHECK_DISK_EVERY: Duration = Duration::from_secs(1);

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
            Self::Review => "Review notes",
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
    mentions: BTreeMap<Uuid, Vec<String>>,
    selected: Option<Uuid>,
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
    job: Option<(Job, Uuid)>,
    reading: Option<(PathBuf, Made)>,
    confirm_delete: Option<PathBuf>,
    last_check: Option<Instant>,
    /// The campaign's manifest changed, so what holds a copy of it must reload.
    pub manifest_changed: Option<PathBuf>,
    pub wants_processing_settings: bool,
    pub wants_event: Option<String>,
}

fn clock(seconds: u64) -> String {
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        (seconds / 60) % 60,
        seconds % 60
    )
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
        if let Some(live) = &mut self.live {
            live.recorder.poll();
            if let Health::Stopped(words) = live.recorder.health().clone() {
                self.notice = Some((
                    Tone::Error,
                    format!("The recording stopped by itself. What it captured is kept. {words}"),
                ));
                self.finish_recording();
            } else {
                ctx.request_repaint_after(Duration::from_secs(1));
            }
        }
        if let Some((job, session)) = &mut self.job {
            let session = *session;
            if let Some(result) = job.poll() {
                self.job = None;
                self.notice = Some(match result {
                    Ok(None) => (
                        Tone::Success,
                        "Processing finished. Read the result, and add the events you want to the timeline."
                            .to_owned(),
                    ),
                    Ok(Some(warning)) => (Tone::Warning, warning),
                    Err(error) => (Tone::Error, error),
                });
                if self.selected == Some(session) {
                    self.refresh_recordings();
                }
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
            for draft in [&mut self.draft, &mut self.template_draft]
                .into_iter()
                .flatten()
            {
                draft.follow_disk();
            }
        }
        book.session_links = self
            .notes
            .iter()
            .map(|note| (note.id, note.link()))
            .collect();
        book.poll(ctx);
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
        self.selected = session;
        self.draft = session
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
            .and_then(|note| Draft::open(&note.path).ok());
        self.reading = None;
        self.confirm_record = false;
        self.refresh_recordings();
    }

    /// The session open now. New events and recordings belong to it.
    pub fn current(&self) -> Option<Uuid> {
        self.selected
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
        self.live
            .as_ref()
            .map(|live| format!("Rec {}", clock(live.recorder.seconds())))
    }

    pub fn recording_label(&self) -> Option<&str> {
        self.live.as_ref().map(|live| live.label.as_str())
    }

    pub fn processing(&self) -> bool {
        self.job.is_some()
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
        if let Some(live) = self.live.take() {
            let path = live.recorder.stop();
            if self.notice.is_none() {
                self.notice = Some((
                    Tone::Success,
                    format!(
                        "Recording saved: {}.",
                        path.file_stem().unwrap_or_default().to_string_lossy()
                    ),
                ));
            }
            self.refresh_recordings();
        }
    }

    fn start_recording(&mut self, settings: &ProcessingSettings) {
        let (Some(folder), Some(session), Some(tool)) =
            (&self.folder, self.selected, settings.tool())
        else {
            return;
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
                self.live = Some(Live {
                    session,
                    label,
                    recorder,
                });
            }
            Err(error) => self.notice = Some((Tone::Error, error)),
        }
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
                if request.speech_key.is_none() {
                    request.speech_key = credentials::load_key(KeySlot::Speech)?;
                }
                if request.llm_key.is_none()
                    && request.config.provider != processing::Provider::CodexCli
                {
                    request.llm_key = credentials::load_key(KeySlot::Llm)?;
                }
                Ok(())
            },
        );
        self.job = Some((job, session));
    }

    fn record_control(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings) {
        let session = self.selected;
        if self.live.as_ref().map(|live| live.session) == session && session.is_some() {
            if Action::destructive("Stop recording")
                .icon(icons::STOP_RECORDING)
                .show(ui)
                .clicked()
            {
                self.finish_recording();
            }
            return;
        }
        let other = self
            .live
            .as_ref()
            .map(|live| format!("Stop the recording of {} first.", live.label));
        let reason = other.as_deref().or(if settings.probing() {
            Some("Looking for a recorder.")
        } else if settings.tool().is_none() {
            Some("No recorder found. See Settings.")
        } else if settings.input_missing() {
            Some("The input chosen in Settings is not connected.")
        } else {
            None
        });
        if Action::secondary("Record")
            .icon(icons::RECORD)
            .name("Record this session")
            .blocked(reason)
            .show(ui)
            .clicked()
        {
            self.confirm_record = true;
        }
    }

    /// The live state, or the step before it. Always in the same place.
    fn record_line(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings) {
        let c = colors();
        if let Some(live) = self
            .live
            .as_ref()
            .filter(|live| Some(live.session) == self.selected)
        {
            let (word, color) = match live.recorder.health() {
                Health::Starting => ("Waiting for sound", c.muted),
                Health::Hearing => ("Hearing sound", c.recording),
                Health::Silent => ("Silent. Check the input", c.result.warning),
                Health::Stalled => ("The file stopped growing", c.result.error),
                Health::Stopped(_) => ("Stopped", c.result.error),
            };
            ui.horizontal_wrapped(|ui| {
                design::status(
                    ui,
                    Status {
                        icon: icons::RECORD,
                        word: &format!("Recording {}", clock(live.recorder.seconds())),
                        color: c.recording,
                    },
                );
                ui.label(Type::Small.rich(ui, word).color(color));
            });
        } else if self.confirm_record {
            design::message(
                ui,
                Tone::Info,
                "Tell everyone at the table before you record. The audio stays in the campaign folder until you select Process.",
            );
            ui.horizontal(|ui| {
                if Action::primary("Start recording").show(ui).clicked() {
                    self.confirm_record = false;
                    self.start_recording(settings);
                }
                if Action::quiet("Not now").show(ui).clicked() {
                    self.confirm_record = false;
                }
            });
        }
    }

    fn draft_status(ui: &mut egui::Ui, draft: &Draft) {
        let c = colors();
        let (icon, word, color) = match &draft.state {
            DraftState::Clean => (icons::OK, "Saved", c.document.clean),
            DraftState::Saved => (icons::OK, "Saved", c.document.saved),
            DraftState::Unsaved => (icons::DRAFT, "Unsaved", c.document.unsaved),
            DraftState::Conflict => (icons::WARN, "Changed on disk", c.document.external),
            DraftState::Failed(_) => (icons::ERROR, "Not saved", c.document.invalid),
        };
        design::status(ui, Status { icon, word, color });
    }

    fn draft_problem(ui: &mut egui::Ui, draft: &mut Draft) {
        match draft.state.clone() {
            DraftState::Conflict => {
                design::message(
                    ui,
                    Tone::Warning,
                    "This file changed on disk while you were editing. Nothing is saved until you choose.",
                );
                ui.horizontal(|ui| {
                    if Action::secondary("Overwrite with my draft")
                        .show(ui)
                        .clicked()
                    {
                        draft.overwrite();
                    }
                    if Action::quiet("Reload and lose my draft").show(ui).clicked() {
                        let _ = draft.reload();
                    }
                });
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
        let number = self.notes.iter().map(|note| note.number).max().unwrap_or(0) + 1;
        let template = self
            .templates
            .iter()
            .find(|template| template.path.is_some())
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
        design::text_field(
            ui,
            "Session title",
            &mut form.title,
            Some("Optional. You can name it after you play."),
            None,
        );
        design::text_field(ui, "Date", &mut form.date, None, None);
        if form.editing.is_none() {
            design::field(ui, "Template", None, None, |ui| {
                let inner = egui::ComboBox::from_id_salt("session_template")
                    .selected_text(&form.template)
                    .width(ui.available_width().min(design::FORM_WIDTH) - design::S)
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
        let mut done = false;
        let mut cancel = false;
        ui.horizontal(|ui| {
            let label = if form.editing.is_some() {
                "Save details"
            } else {
                "Create session"
            };
            done = Action::primary(label).show(ui).clicked();
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
                    self.tab = Tab::Write;
                    return;
                }
                Err(error) => form.error = Some(error),
            }
        }
        self.form = Some(form);
    }

    fn submit_form(&mut self, form: &Form) -> Result<Uuid, String> {
        let number: u32 = form
            .number
            .trim()
            .parse()
            .map_err(|_| "The number must be a whole number.".to_owned())?;
        if let Some(id) = form.editing {
            let note = self
                .notes
                .iter()
                .find(|note| note.id == id)
                .cloned()
                .ok_or("This session is gone from the folder.")?;
            self.flush();
            update_details(&note, &form.title, &form.date, number)?;
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
                        let detail = if live == Some(note.id) {
                            format!("{} · Recording", note.date)
                        } else {
                            note.date.clone()
                        };
                        if design::two_line_row(
                            ui,
                            Density::Glance.control() + design::S,
                            self.selected == Some(note.id),
                            &note.label(),
                            &detail,
                        )
                        .clicked()
                        {
                            choose = Some(note.id);
                        }
                    }
                });
            });
        if let Some(id) = choose {
            self.form = None;
            self.select(Some(id));
        }
    }

    fn write_tab(&mut self, ui: &mut egui::Ui, book: &mut Book) {
        let Some(draft) = &mut self.draft else {
            design::message(ui, Tone::Error, "This note could not be opened.");
            return;
        };
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)) {
            draft.save();
        }
        Self::draft_problem(ui, draft);
        egui::ScrollArea::vertical()
            .id_salt(("session_editor", self.selected))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let response = ui.add(
                    egui::TextEdit::multiline(&mut draft.body)
                        .font(egui::TextStyle::Monospace)
                        .margin(egui::vec2(design::S, design::S))
                        .desired_width(ui.available_width())
                        .min_size(egui::vec2(0.0, ui.available_height())),
                );
                design::named(ui, &response, "Session notes");
                if response.changed() {
                    draft.edited();
                    if let Some(session) = self.selected {
                        self.mentions
                            .insert(session, timeline::referenced_ids(&draft.body));
                    }
                }
            });
        let _ = book;
    }

    fn read_tab(&mut self, ui: &mut egui::Ui, book: &Book) {
        let Some(draft) = &self.draft else {
            return;
        };
        let text = timeline::for_reading(&draft.body, &book.events);
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
                        .filter(|(event, _)| event.session == session)
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
        // Stay here. At the table, a rest is one press, and its details can wait.
        quick_add(ui, book, session, None);
        ui.add_space(design::XS);
        let days = timeline::day_numbers(&book.events);
        let mut insert = None;
        egui::ScrollArea::vertical()
            .id_salt(("session_events", session))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let mut any = false;
                for (event, day) in book.events.iter().zip(&days) {
                    if event.session != session {
                        continue;
                    }
                    any = true;
                    ui.horizontal(|ui| {
                        if Action::quiet("Add to notes")
                            .name(&format!("Add to notes: {}", event.label(*day)))
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                        {
                            insert = Some(timeline::reference(event, *day));
                        }
                        ui.label(Type::Body.rich(ui, event.kind.icon()).color(c.muted));
                        if design::link(ui, &event.label(*day)).clicked() {
                            self.wants_event = Some(event.id.clone());
                        }
                        ui.add(
                            egui::Label::new(
                                Type::Small.rich(ui, event.summary(None)).color(c.muted),
                            )
                            .truncate(),
                        );
                    });
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
            if !draft.body.ends_with('\n') {
                draft.body.push('\n');
            }
            draft.body.push_str(&format!("\n{link}\n"));
            draft.edited();
            self.mentions
                .insert(session, timeline::referenced_ids(&draft.body));
            self.notice = Some((Tone::Success, "Added to the end of the notes.".to_owned()));
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
            .map_or_else(|| "Unknown length".to_owned(), clock);
        ui.label(Type::Strong.rich(ui, &recording.name).color(c.text));
        let state = if live {
            "Recording now"
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
                if Action::quiet("Cancel")
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
        let mut process = false;
        let mut delete = false;
        let mut open = None;
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
                process = Action::secondary(label)
                    .name(&format!("{label}: {}", recording.name))
                    .density(Density::Read)
                    .blocked(reason.as_deref())
                    .show(ui)
                    .clicked();
            }
            for (made, exists) in [
                (Made::Summary, recording.summary),
                (Made::Marked, recording.marked),
                (Made::Review, recording.review),
                (Made::Transcript, recording.transcript),
            ] {
                if exists
                    && Action::quiet(made.word())
                        .name(&format!("{}: {}", made.word(), recording.name))
                        .density(Density::Read)
                        .selected(self.reading == Some((recording.path.clone(), made)))
                        .show(ui)
                        .clicked()
                {
                    open = Some(made);
                }
            }
            ui.add_space(design::L);
            delete = Action::quiet("Delete")
                .name(&format!("Delete recording: {}", recording.name))
                .density(Density::Read)
                .blocked_if(live || job_here, "This recording is in use.")
                .show(ui)
                .clicked();
        });
        if process {
            let path = recording.path.clone();
            self.start_processing(ui.ctx(), &path, settings, book);
        }
        if delete {
            self.confirm_delete = Some(recording.path.clone());
        }
        if let Some(made) = open {
            let this = (recording.path.clone(), made);
            self.reading = (self.reading.as_ref() != Some(&this)).then_some(this);
        }
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
        for (index, marker) in recording.markers.iter().enumerate() {
            let title = if marker.title.is_empty() {
                marker.kind.word()
            } else {
                marker.title.as_str()
            };
            let added = book.has_marker(marker, session);
            let mut facts = vec![marker.kind.word()];
            for fact in [&marker.at, &marker.duration, &marker.place] {
                if !fact.is_empty() {
                    facts.push(fact);
                }
            }
            ui.horizontal(|ui| {
                if added {
                    design::status(
                        ui,
                        Status {
                            icon: icons::OK,
                            word: "On the timeline",
                            color: c.document.saved,
                        },
                    );
                } else if Action::secondary("Add")
                    .name(&format!("Add to timeline: {title} ({index})"))
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    book.add_marker(marker, session);
                }
                ui.label(Type::Body.rich(ui, marker.kind.icon()).color(c.muted));
                ui.label(Type::Body.rich(ui, title).color(c.text));
                ui.add(
                    egui::Label::new(Type::Small.rich(ui, facts.join(" · ")).color(c.muted))
                        .truncate(),
                );
            });
        }
    }

    fn reading_pane(&mut self, ui: &mut egui::Ui, recording: &Path, made: Made) {
        let text = fs::read_to_string(made.path(recording)).unwrap_or_default();
        let body = timeline::markers_for_reading(body_of(&text));
        ui.add_space(design::XS);
        design::group_label(ui, made.word());
        if made == Made::Summary
            && Action::secondary("Add to notes").show(ui).clicked()
            && let Some(draft) = &mut self.draft
        {
            draft.body.push_str(&format!("\n\n{}\n", body.trim()));
            draft.edited();
            self.notice = Some((Tone::Success, "Added to the end of the notes.".to_owned()));
        }
        design::reader::measure(ui, |ui| markdown_view::show_reader(ui, &body));
    }

    fn recordings_tab(
        &mut self,
        ui: &mut egui::Ui,
        settings: &ProcessingSettings,
        book: &mut Book,
    ) {
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
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(
                            ui,
                            format!(
                                "Process sends the audio to {} for a transcript. Then the transcript and this session's notes go to {llm} twice: once to clean them, and once to check that work against the raw words.",
                                processing::host(&config.speech_url),
                            ),
                        )
                        .color(colors().muted),
                )
                .wrap(),
            );
        }
        let recordings = self.recordings.clone();
        if recordings.is_empty() {
            ui.add_space(design::M);
            ui.add(
                egui::Label::new(
                    Type::Body
                        .rich(
                            ui,
                            "No recordings of this session. Select Record above to make one.",
                        )
                        .color(colors().muted),
                )
                .wrap(),
            );
            return;
        }
        egui::ScrollArea::vertical()
            .id_salt(("session_recordings", self.selected))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for recording in &recordings {
                    ui.add_space(design::S);
                    self.recording_row(ui, recording, settings, book);
                    self.markers(ui, recording, book);
                    if let Some((path, made)) = self.reading.clone()
                        && path == recording.path
                    {
                        self.reading_pane(ui, &path, made);
                    }
                    design::rule(ui);
                }
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
                    path.file_stem().unwrap_or_default().to_string_lossy()
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
            return;
        };
        ui.horizontal(|ui| {
            ui.add(egui::Label::new(Type::Title.rich(ui, note.label()).color(c.text)).truncate());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
                    Self::draft_status(ui, draft);
                }
            });
        });
        ui.label(Type::Small.rich(ui, &note.date).color(c.muted));
        self.record_line(ui, settings);
        if let Some((tone, words)) = self.notice.clone() {
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
                }
            });
        }
        let recordings = format!("Recordings ({})", self.recordings.len());
        design::segmented(
            ui,
            &mut self.tab,
            &[
                (Tab::Write, "Write"),
                (Tab::Read, "Read"),
                (Tab::Events, "Events"),
                (Tab::Recordings, recordings.as_str()),
            ],
        );
        ui.add_space(design::XS);
        match self.tab {
            Tab::Write => self.write_tab(ui, book),
            Tab::Read => self.read_tab(ui, book),
            Tab::Events => self.events_tab(ui, book),
            Tab::Recordings => self.recordings_tab(ui, settings, book),
        }
    }

    fn notes_view(&mut self, ui: &mut egui::Ui, settings: &ProcessingSettings, book: &mut Book) {
        if self.notes.is_empty() && self.form.is_none() {
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
        egui::SidePanel::left("session_list_panel")
            .resizable(false)
            .exact_width(LIST_WIDTH)
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
                    self.session_pane(ui, settings, book);
                }
            });
    }

    fn select_template(&mut self, name: &str) {
        self.flush();
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
        egui::SidePanel::left("template_list_panel")
            .resizable(false)
            .exact_width(LIST_WIDTH)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                right: design::S as i8,
                ..Default::default()
            }))
            .show_inside(ui, |ui| {
                start_new = Action::secondary("New template")
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
                                choose = Some(template.name.clone());
                            }
                        }
                    });
            });
        if start_new {
            self.new_template = Some(String::new());
            self.template_error = None;
        }
        if let Some(name) = choose {
            self.select_template(&name);
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
                if self.new_template.is_some() {
                    self.new_template_form(ui);
                }
                self.template_editor(ui);
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
                            .desired_width(ui.available_width())
                            .min_size(egui::vec2(0.0, ui.available_height())),
                    );
                    design::named(ui, &response, "Template text");
                    if response.changed() {
                        draft.edited();
                    }
                } else {
                    ui.label(Type::Mono.rich(ui, &template.body).color(c.muted));
                }
            });
    }

    fn folder_view(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        let Some(campaign) = self.campaign.clone() else {
            return;
        };
        ui.add_space(design::XS);
        design::text_field(
            ui,
            "Session folder",
            &mut self.folder_text,
            Some(
                "Inside the campaign folder, unless the path is absolute. Keep your own numbering if you use it.",
            ),
            self.folder_error.as_deref(),
        );
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
        } else {
            None
        };
        if Action::primary("Save folder")
            .blocked(blocked)
            .show(ui)
            .clicked()
        {
            self.flush();
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
        design::page_header(ui, "Sessions", Some(&campaign), |ui| {
            if self.view == View::Notes && !in_form {
                new_session = Action::primary("New session")
                    .icon(icons::ADD)
                    .show(ui)
                    .clicked();
            }
        });
        if new_session {
            self.open_form();
        }
        design::segmented(
            ui,
            &mut self.view,
            &[
                (View::Notes, "Notes"),
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
            View::Folder => self.folder_view(ui),
        }
        self.delete_dialog(ui.ctx());
    }
}

#[cfg(test)]
impl Sessions {
    pub fn busy(&self) -> bool {
        self.job.is_some()
    }

    pub fn simulate_recorder_health(&mut self, health: Health) {
        if let Some(live) = &mut self.live {
            live.recorder.simulate_health(health);
        }
    }
}
