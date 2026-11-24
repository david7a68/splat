//! What the GPUI Session page (`src/gui/pages/sessions/`) reads from the page's state and the
//! acts it asks for. Toolkit-free: no egui `Ui` here. Each act is the same state change the egui
//! page makes for the same press, by the same methods, so the two UIs share one behaviour. The
//! GPUI view holds no copy of the page's logic; what it needs from a private field is read here.
use eframe::egui;
use super::{
    DraftState, Form, Note, Plan, ProcessingSettings, SessionAct, Sessions, Tab, Template, View,
    form_reason,
};
use crate::design::CreateReasons as _;
use crate::timeline::EventDisplay as _;
use crate::design::outcome::{PaneFrame, Press, plain};
use crate::timeline::Book;
use std::path::PathBuf;
use uuid::Uuid;

/// The New session form (or Session details) as the view draws it.
pub struct FormView {
    pub editing: bool,
    pub number: String,
    pub title: String,
    pub date: String,
    pub template: String,
    pub error: Option<String>,
    /// Why Create waits, in its words.
    pub reason: Option<&'static str>,
    /// `File: Session 1.md`, once nothing blocks Create.
    pub preview: Option<String>,
    /// Whether Create is the screen's primary (not while a run is open).
    pub primary: bool,
    /// Ask for the keyboard in the title (once).
    pub focus_title: bool,
}

/// Which of the form's words the DM changed.
#[derive(Clone, Copy, Debug)]
pub enum FormField {
    Number,
    Title,
    Date,
    Template,
}

/// A press in a message footer, kept until the view acts on it. Not `Clone`: one press acts once.
pub struct FooterPress(Press<SessionAct>);

/// One control of a footer line.
pub struct FooterChoice {
    pub words: String,
    pub kind: plain::Kind,
    pub blocked: Option<String>,
    pub hover: Option<String>,
    pub press: FooterPress,
}

/// One footer line, in the footer's order.
pub struct FooterLine {
    pub tone: crate::design::Tone,
    pub words: String,
    pub hover: Option<String>,
    pub file: Option<String>,
    pub help: Vec<String>,
    pub undo: Option<FooterChoice>,
    pub choices: Vec<FooterChoice>,
}

fn footer(messages: crate::design::outcome::Messages<'_, SessionAct>) -> Vec<FooterLine> {
    let choice = |choice: plain::Choice<SessionAct>| FooterChoice {
        words: choice.words,
        kind: choice.kind,
        blocked: choice.blocked,
        hover: choice.hover,
        press: FooterPress(choice.press),
    };
    messages
        .into_plain()
        .into_iter()
        .map(|line| FooterLine {
            tone: line.tone,
            words: line.words,
            hover: line.hover,
            file: line.file,
            help: line.help,
            undo: line.undo.map(choice),
            choices: line.choices.into_iter().map(choice).collect(),
        })
        .collect()
}

/// One row of the session list.
pub struct ListRow {
    pub id: Uuid,
    pub title: String,
    /// The note's file name, when another row has the same title and date.
    pub file: Option<String>,
    /// The second line: the date or a recording state, then `Playing` for the current session.
    pub detail: String,
    /// The state's icon, word and colour (a live recording, a processed result), if any.
    pub state: Option<super::RecordingStatus>,
    pub playing: bool,
    pub selected: bool,
    /// The full words of a short state, for the row's name and hover.
    pub spoken: Option<String>,
}

impl Sessions {
    // The page and its header.

    pub fn gv_templates_shown(&self) -> bool {
        self.view == View::Templates
    }

    pub fn gv_show_templates(&mut self) {
        self.view = View::Templates;
    }

    pub fn gv_show_notes(&mut self) {
        self.view = View::Notes;
    }

    pub fn gv_notes(&self) -> &[Note] {
        &self.notes
    }

    pub fn gv_templates(&self) -> &[Template] {
        &self.templates
    }

    pub fn gv_selected(&self) -> Option<Uuid> {
        self.selected
    }

    pub fn gv_selected_note(&self) -> Option<&Note> {
        self.selected
            .and_then(|id| self.notes.iter().find(|note| note.id == id))
    }

    pub fn gv_form_open(&self) -> bool {
        self.form.is_some()
    }

    pub fn gv_new_template_open(&self) -> bool {
        self.new_template.is_some()
    }

    /// Other sessions wait until the DM chooses what to keep of the open note.
    pub fn gv_locked(&self) -> bool {
        self.draft.as_ref().is_some_and(super::Draft::needs_attention)
    }

    pub fn gv_offers_list_toggle(&self, wide: bool) -> bool {
        self.offers_list_toggle(wide)
    }

    pub fn gv_list_hides(&self) -> bool {
        self.list_hides()
    }

    /// The list stands beside the session (the last drawn page's width test), for `Ctrl+\`.
    pub fn gv_set_list_beside(&mut self, beside: bool) {
        self.list_beside = beside;
    }

    pub fn gv_list_only(&self) -> bool {
        self.list_only
    }

    pub fn gv_set_list_only(&mut self, only: bool) {
        self.list_only = only;
    }

    pub fn gv_open_run(&self) -> bool {
        self.open_run.is_some()
    }

    /// The header's `New template`.
    pub fn gv_new_template(&mut self) {
        self.templates_list_only = false;
        self.new_template = Some(String::new());
        self.template_error = None;
    }

    /// `Stop recording`, from the header or a record control.
    pub fn gv_stop_recording(&mut self, ctx: &egui::Context) {
        self.finish_recording(ctx);
    }

    /// `New session` (the header, the empty state).
    pub fn gv_open_form(&mut self) {
        self.open_form();
    }

    /// The words of the page that replaces everything while the folder cannot be used.
    pub fn gv_folder_fault(&self) -> String {
        self.folder_error.as_ref().map_or_else(
            || "The session folder is not available.".to_owned(),
            |fault| String::from(fault.clone()),
        )
    }

    /// The page's notice ends when the DM moves to another session or view (`show`).
    pub fn gv_follow_context(&mut self) {
        let context = (self.selected, self.view);
        if self
            .notice_context
            .is_some_and(|previous| previous != context)
        {
            self.clear_notice();
        }
        self.notice_context = Some(context);
    }

    // The form.

    pub fn gv_form(&mut self) -> Option<FormView> {
        let focus_title = std::mem::take(&mut self.focus_form_title);
        let form = self.form.as_ref()?;
        let reason = form_reason(form, &self.templates).map(|reason| reason.words());
        let preview = match (&form.editing, &self.folder) {
            (None, Some(folder)) => {
                let path = super::super::note_path(
                    folder,
                    form.number.parse().unwrap_or(1),
                    &form.title,
                );
                let shown = path.strip_prefix(folder).unwrap_or(&path);
                Some(format!("File: {}", shown.display()))
            }
            _ => None,
        };
        Some(FormView {
            editing: form.editing.is_some(),
            number: form.number.clone(),
            title: form.title.clone(),
            date: form.date.clone(),
            template: form.template.clone(),
            error: form.error.clone(),
            reason,
            preview,
            primary: self.open_run.is_none(),
            focus_title,
        })
    }

    pub fn gv_form_set(&mut self, field: FormField, words: &str) {
        let Some(form) = &mut self.form else {
            return;
        };
        let slot = match field {
            FormField::Number => &mut form.number,
            FormField::Title => &mut form.title,
            FormField::Date => &mut form.date,
            FormField::Template => &mut form.template,
        };
        words.clone_into(slot);
    }

    pub fn gv_form_create(&mut self) {
        if let Some(form) = self.form.take() {
            self.form = self.press_create(form);
        }
    }

    pub fn gv_form_cancel(&mut self) {
        self.form = None;
    }

    /// `Details`: the same fields for this session.
    pub fn gv_edit_details(&mut self) {
        if let Some(note) = self.gv_selected_note().cloned() {
            self.form = Some(Form {
                editing: Some(note.id),
                number: note.number.to_string(),
                title: note.title.clone(),
                date: note.date.clone(),
                template: String::new(),
                error: None,
            });
        }
    }

    // The list.

    /// The session list's rows, as `list_pane` words them.
    pub fn gv_list_rows(&self) -> Vec<ListRow> {
        let live = self.live.as_ref().map(|live| live.session);
        let job = self.job.as_ref().map(|(_, session)| *session);
        let result = self.processed_session;
        let current = self.current();
        self.notes
            .iter()
            .map(|note| {
                let state = if live == Some(note.id) || job == Some(note.id) {
                    self.rail_recording_status()
                } else {
                    self.alerts
                        .iter()
                        .rev()
                        .find(|alert| alert.session == note.id)
                        .map(|alert| {
                            let cancelled = alert.word == super::PROCESSING_CANCELLED;
                            let c = crate::design::colors();
                            super::RecordingStatus::one(crate::design::Status {
                                icon: if cancelled {
                                    crate::design::icons::WARN
                                } else {
                                    crate::design::icons::ERROR
                                },
                                word: if cancelled {
                                    "Cancelled"
                                } else if alert.can_record {
                                    "Stopped"
                                } else {
                                    "Failed"
                                },
                                color: if cancelled {
                                    c.result.warning
                                } else {
                                    c.result.error
                                },
                            })
                        })
                };
                let warned = state.is_none() && result == Some(note.id) && self.job_warned;
                let clean = state.is_none() && result == Some(note.id) && !warned;
                let state = if warned {
                    Some(super::RecordingStatus::one(crate::design::Status {
                        word: "Processed",
                        ..super::processed_warning()
                    }))
                } else if clean {
                    Some(super::RecordingStatus::one(super::processed_clean()))
                } else {
                    state
                };
                let spoken = if warned {
                    Some(super::processed_warning().word.to_owned())
                } else if clean {
                    Some(super::processed_clean().word.to_owned())
                } else {
                    None
                };
                let duplicate = self.notes.iter().any(|other| {
                    other.id != note.id && other.label() == note.label() && other.date == note.date
                });
                ListRow {
                    id: note.id,
                    title: note.label(),
                    file: duplicate.then(|| {
                        note.path
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned()
                    }),
                    detail: if state.is_some() {
                        String::new()
                    } else {
                        note.date.clone()
                    },
                    state,
                    playing: current == Some(note.id),
                    selected: self.selected == Some(note.id),
                    spoken,
                }
            })
            .collect()
    }

    /// A press on a row of the list.
    pub fn gv_choose(&mut self, id: Uuid) {
        self.form = None;
        self.list_only = false;
        self.select(Some(id));
    }

    // The session pane.

    pub fn gv_tab(&self) -> u8 {
        match self.tab {
            Tab::Plan => 0,
            Tab::Note => 1,
            Tab::Recordings => 2,
        }
    }

    /// A press on a tab: 0 Plan, 1 Note, 2 Recordings (`session_header`).
    pub fn gv_set_tab(&mut self, tab: u8) {
        let shown = match tab {
            0 => Tab::Plan,
            1 => Tab::Note,
            _ => Tab::Recordings,
        };
        if shown == self.tab {
            return;
        }
        if self.tab == Tab::Recordings || shown == Tab::Recordings {
            self.clear_notice();
        }
        self.tab = shown;
        self.clear_success();
    }

    pub fn gv_recording_count(&self) -> usize {
        self.recordings.len()
    }

    pub fn gv_reading_note(&self) -> bool {
        self.reading_note
    }

    pub fn gv_toggle_reading(&mut self) {
        self.reading_note = !self.reading_note;
    }

    /// The note's draft state, while a draft is open.
    pub fn gv_draft_state(&self) -> Option<&DraftState> {
        self.draft.as_ref().map(|draft| &draft.state)
    }

    /// The open draft's file name, for the header of a note whose file is gone.
    pub fn gv_draft_name(&self) -> Option<String> {
        self.draft.as_ref().map(|draft| {
            draft
                .path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        })
    }

    /// The note status beside `Details` (`note_save_status`).
    pub fn gv_note_status(&self) -> Option<crate::design::Status<'static>> {
        self.draft
            .as_ref()
            .map(|draft| Self::note_state_status(&draft.state))
    }

    pub fn gv_has_draft(&self) -> bool {
        self.draft.is_some()
    }

    pub fn gv_conflict(&self) -> bool {
        self.draft
            .as_ref()
            .is_some_and(|draft| draft.state == DraftState::Conflict)
    }

    /// The session page's own footer (`session_messages`), as plain lines.
    pub fn gv_session_footer(
        &mut self,
        settings: &ProcessingSettings,
        book: &Book,
        plan: &Plan<'_>,
    ) -> Vec<FooterLine> {
        let Some(note) = self.gv_selected_note().cloned() else {
            return Vec::new();
        };
        let frame = PaneFrame::headless();
        let messages = self.session_messages(&frame, settings, book, plan, &note, 0.0);
        footer(messages)
    }

    /// A press in the session page's footer (`session_press`).
    pub fn gv_session_press(&mut self, press: FooterPress, book: &mut Book, plan: &Plan<'_>) {
        self.session_press(press.0, book, plan);
    }

    /// The footer of a pane whose body is the draft's editor (`draft_messages`).
    pub fn gv_draft_footer(&mut self, template: bool) -> Vec<FooterLine> {
        let frame = PaneFrame::headless();
        let messages = self.draft_messages(&frame, "gpui_draft_footer", template, 0.0);
        footer(messages)
    }

    pub fn gv_draft_press(&mut self, press: FooterPress, template: bool) {
        self.draft_press(press.0, template);
    }

    /// Where the session's folder is, if one is set.
    pub fn gv_folder(&self) -> Option<&PathBuf> {
        self.folder.as_ref()
    }
}

/// The record control beside the title (`record_control`).
pub enum RecordControl {
    None,
    /// `Stop recording`, named for its session when it is another's.
    Stop { name: String },
    /// `Record`, and why it waits (shown under it too).
    Record { blocked: Option<String> },
}

/// The record line under the title (`record_line_body`).
pub enum RecordLine {
    None,
    /// This session's live recording: its state, time and what it hears.
    Live {
        status: super::RecordingStatus,
        /// The state and its time, in the state's colours.
        words: String,
        /// What it hears, muted.
        detail: String,
        /// Silent for a minute: offer the input check.
        silent: bool,
    },
    /// The step before recording: what it records, and the choices.
    Confirm {
        words: String,
        tone: crate::design::Tone,
        play_this_one: bool,
        start_primary: bool,
    },
}

/// The Last time line (`last_time_line`).
pub struct LastTimeView {
    pub id: Uuid,
    pub label: String,
    /// The first written line of what happened.
    pub preview: String,
    /// The whole words, for hover.
    pub full: String,
    /// `Carry over`, while this session is being prepared: its hover, or why it waits.
    pub carry: Option<Result<String, String>>,
}

impl Sessions {
    /// The record control's state now (`record_control`).
    pub fn gv_record_control(&self, settings: &ProcessingSettings) -> RecordControl {
        use crate::recording::Health;
        if self.live.as_ref().is_some_and(|live| {
            matches!(live.recorder.health(), Health::Stalled | Health::Stopped(_))
        }) {
            return RecordControl::None;
        }
        if let Some(live) = &self.live {
            if self.tab == Tab::Recordings
                && self.reading.is_none()
                && self.selected == Some(live.session)
                && !self.recordings.is_empty()
            {
                return RecordControl::None;
            }
            let name = if Some(live.session) == self.selected {
                "Stop recording".to_owned()
            } else {
                format!("Stop recording of {}", live.label)
            };
            return RecordControl::Stop { name };
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
            return RecordControl::None;
        }
        RecordControl::Record {
            blocked: self.record_blocked(settings),
        }
    }

    /// `Record`: the step before recording.
    pub fn gv_ask_to_record(&mut self) {
        self.confirm_record = true;
    }

    /// The record line's state now (`record_line_body`).
    pub fn gv_record_line(&mut self) -> RecordLine {
        use crate::recording::Health;
        if self.tab == Tab::Recordings
            && self.reading.is_none()
            && self
                .live
                .as_ref()
                .is_some_and(|live| Some(live.session) == self.selected)
        {
            return RecordLine::None;
        }
        if let Some(live) = self
            .live
            .as_ref()
            .filter(|live| Some(live.session) == self.selected)
        {
            let time = super::clock(live.recorder.seconds());
            let status = self.recording_status();
            return match (live.recorder.health(), status) {
                (Health::Stalled | Health::Stopped(_), _) | (_, None) => RecordLine::None,
                (health, Some(status)) => {
                    let word = match health {
                        Health::Starting => "Waiting for sound",
                        Health::Silent => "Silent for over a minute",
                        _ if live.recorder.heard_just_now() => "Hearing sound",
                        _ => "Quiet",
                    };
                    let silent = *health == Health::Silent;
                    RecordLine::Live {
                        status,
                        words: if silent {
                            status.word.to_owned()
                        } else {
                            format!("{} {time}", status.word)
                        },
                        detail: if silent {
                            format!("{word} · {time} recorded")
                        } else {
                            word.to_owned()
                        },
                        silent,
                    }
                }
            };
        }
        if self.live.is_some() || !self.confirm_record {
            return RecordLine::None;
        }
        let note = self.gv_selected_note().cloned();
        let label = note
            .as_ref()
            .map_or_else(String::new, |note| format!("“{}”", note.label()));
        let today = self.today();
        let dated = note
            .as_ref()
            .and_then(|note| super::dated_before_today(note, &today));
        let other = if self.selected != self.current() {
            " This is not the session being played.".to_owned()
        } else {
            dated
                .as_ref()
                .map_or_else(String::new, |words| format!(" {words}"))
        };
        RecordLine::Confirm {
            words: format!(
                "Record {label}.{other} Tell everyone at the table first. The audio stays in the campaign folder until you select Process."
            ),
            tone: if other.is_empty() {
                crate::design::Tone::Info
            } else {
                super::NotTonight::TONE
            },
            play_this_one: dated.is_some() && self.selected == self.current(),
            start_primary: !self.run_open,
        }
    }

    /// The record line's `Play this one`.
    pub fn gv_play_this_one(&mut self) {
        self.play_this_one();
    }

    /// The record line's `Start recording`.
    pub fn gv_start_recording(&mut self, settings: &ProcessingSettings) {
        self.confirm_record = false;
        if let Some(session) = self.selected
            && let Err(error) = self.start_recording(settings, session)
        {
            self.say(crate::design::Tone::Error, error, None);
        }
    }

    /// The record line's `Not now`.
    pub fn gv_not_now(&mut self) {
        self.confirm_record = false;
    }

    /// The Last time line, read once for each selection (`see_last_time`, `last_time_line`).
    /// Nothing while a group being assembled here needs the height.
    pub fn gv_last_time(&mut self, book: &Book, plan: &Plan<'_>) -> Option<LastTimeView> {
        if plan.group_open && self.group_session == self.selected {
            return None;
        }
        let session = self.see_last_time(book, plan)?;
        let carry = if self.preparing(session, book) {
            self.carry_offer()
        } else {
            None
        };
        let (_, Some(last)) = self.last_time.as_ref()? else {
            return None;
        };
        let full = format!(
            "Last time: {} — {}",
            last.label,
            last.what_happened
                .as_deref()
                .unwrap_or("Nothing written yet.")
        );
        let preview = last
            .what_happened
            .as_deref()
            .and_then(|words| words.lines().find(|line| !line.trim().is_empty()))
            .unwrap_or("")
            .to_owned();
        Some(LastTimeView {
            id: last.id,
            label: last.label.clone(),
            preview,
            full,
            carry,
        })
    }

    /// The Last time line's link: the session before.
    pub fn gv_open_session(&mut self, id: Uuid) {
        self.select(Some(id));
    }

    /// `Carry over`.
    pub fn gv_carry_over(&mut self, previous: Uuid, plan: &Plan<'_>, book: &Book) {
        if let Some(session) = self.selected {
            self.carry_over(session, previous, plan, book);
        }
    }

    /// The Read view of the open note is worth offering: it holds written words, or the DM is
    /// writing (`session_header`).
    pub fn gv_offers_read(
        &mut self,
        plan: &Plan<'_>,
        index: Option<&crate::lookup::ContentIndex>,
        beside: bool,
    ) -> bool {
        if !self.reading_note {
            return true;
        }
        let model = self.plan_model(index, plan);
        super::has_written_text(&super::compact_plan_for_reading(
            &self.note_body(),
            plan.fights,
            plan.groups,
            &model.scene_lines(),
            beside,
        ))
    }

    /// A group is being assembled in this session's plan: it takes the plan's place.
    pub fn gv_editing_group_here(&self, plan: &Plan<'_>) -> bool {
        plan.group_open && self.group_session == self.selected
    }
}

/// What a press in the session banner does (`banner`, `banner_notices`, `draft_warning`).
#[derive(Clone, Debug)]
pub enum BannerAct {
    StalledRecord,
    StalledOpen,
    StalledStop,
    /// An alert's choice, by its index in the alerts.
    AlertInput(usize),
    AlertRetry(usize),
    AlertRecord(usize),
    AlertOpen(usize),
    AlertDismiss(usize),
    /// The notice that arrived by itself: its close icon.
    NoticeDismiss,
    /// `Make <file> a separate session`.
    Separate(PathBuf),
    /// The note's warning's choices; `true` for the template's.
    Draft(bool, super::DraftAct),
}

/// One banner message: its tone, words, the recorder's own words on hover, and its choices.
pub struct BannerMessage {
    pub tone: crate::design::Tone,
    pub words: String,
    pub hover: Option<String>,
    pub choices: Vec<BannerChoice>,
}

pub struct BannerChoice {
    pub words: String,
    pub kind: plain::Kind,
    pub blocked: Option<String>,
    pub act: BannerAct,
}

/// Another session's live recording, as the banner's status line says it.
pub struct OtherRecording {
    pub status: super::RecordingStatus,
    pub words: String,
    pub detail: String,
}

/// The banner: another recording's line, then its messages in order.
pub struct Banner {
    pub other: Option<OtherRecording>,
    pub messages: Vec<BannerMessage>,
}

fn choice(words: &str, kind: plain::Kind, act: BannerAct) -> BannerChoice {
    BannerChoice {
        words: words.to_owned(),
        kind,
        blocked: None,
        act,
    }
}

/// The draft's warning as data (`draft_warning`).
fn draft_warning(draft: &super::Draft, name: Option<&str>, template: bool) -> Vec<BannerMessage> {
    use super::{DraftAct, Dropped};
    use crate::design::Tone;
    use plain::Kind;
    let words = |text: &str| super::owned_words(name, text);
    let mut messages = Vec::new();
    if draft.state == DraftState::Missing
        && let Some(error) = &draft.save_again_error
    {
        messages.push(BannerMessage {
            tone: Tone::Error,
            words: words(error),
            hover: None,
            choices: Vec::new(),
        });
    }
    let act = |act| BannerAct::Draft(template, act);
    let message = match draft.state.clone() {
        DraftState::Conflict => Some((
            Tone::Warning,
            words(
                "This note changed on disk. Your draft is safe until you choose. Other sessions open after that.",
            ),
            vec![
                choice("Overwrite with my draft", Kind::Safe, act(DraftAct::Overwrite)),
                choice("Reload and lose my draft", Kind::Lose, act(DraftAct::Reload)),
            ],
        )),
        DraftState::Missing if matches!(draft.dropped, Some(Dropped::Discarded(_))) => Some((
            Tone::Warning,
            words(super::DISCARDED_GONE),
            vec![choice(
                "Save these words as a new file",
                Kind::Safe,
                act(DraftAct::SaveAgain),
            )],
        )),
        DraftState::Missing => {
            let moved = draft.moved_to.as_ref().map(|path| {
                path.strip_prefix(&draft.search_folder)
                    .unwrap_or(path)
                    .display()
                    .to_string()
            });
            let mut choices = Vec::new();
            if let Some(moved) = &moved {
                choices.push(choice(
                    &format!("Save my draft to {moved}"),
                    Kind::Safe,
                    act(DraftAct::SaveMoved),
                ));
            }
            choices.push(choice(
                "Save my draft as a new file",
                Kind::Safe,
                act(DraftAct::SaveAgain),
            ));
            choices.push(choice("Discard", Kind::Lose, act(DraftAct::Discard)));
            Some((
                Tone::Warning,
                words(&super::missing_note_words(moved)),
                choices,
            ))
        }
        DraftState::Failed(error) => Some((
            Tone::Error,
            words(&error),
            vec![choice("Try to save again", Kind::Safe, act(DraftAct::Save))],
        )),
        _ => None,
    };
    if let Some((tone, words, choices)) = message {
        messages.push(BannerMessage {
            tone,
            words,
            hover: None,
            choices,
        });
    }
    messages
}

impl Sessions {
    /// The session banner, or `None` when it has nothing to say (`banner`).
    pub fn gv_banner(&self, settings: &ProcessingSettings, run_open: bool) -> Option<Banner> {
        use crate::design::Tone;
        use plain::Kind;
        let primary_owner = self.banner_primary_owner(settings, run_open);
        let stalled = self.stalled_elsewhere().map(|live| {
            format!(
                "The recording of “{}” stopped saving audio. The first {} are kept. Record again starts a new file.",
                live.label,
                super::spoken_length(live.recorder.seconds())
            )
        });
        if stalled.is_none()
            && self
                .alerts
                .iter()
                .all(|alert| self.recording_row_here(alert.session))
            && !self.has_banner_notices()
        {
            return None;
        }
        let mut messages = Vec::new();
        if let Some(words) = stalled {
            let session = self.live.as_ref().map(|live| live.session);
            let here = self.on_page && self.selected == session;
            let record = if primary_owner == Some(usize::MAX) {
                Kind::Primary
            } else {
                Kind::Safe
            };
            let mut choices = vec![choice("Record again", record, BannerAct::StalledRecord)];
            if !here {
                choices.push(choice(
                    "Open the session",
                    Kind::Safe,
                    BannerAct::StalledOpen,
                ));
            }
            choices.push(choice("Stop recording", Kind::Safe, BannerAct::StalledStop));
            messages.push(BannerMessage {
                tone: Tone::Error,
                words,
                hover: None,
                choices,
            });
        }
        for (index, alert) in self.alerts.iter().enumerate() {
            if self.recording_row_here(alert.session) {
                continue;
            }
            let banner_primary = primary_owner == Some(index);
            let missing_input =
                alert.can_record && (settings.input_missing() || alert.missing_input());
            let words = if alert.word == "Recording stopped" {
                format!(
                    "{} {}",
                    alert.words,
                    super::recording_recovery(missing_input)
                )
            } else {
                alert.words.clone()
            };
            let audio_gone = alert.retry.as_ref().is_some_and(|path| !path.exists());
            let words = if audio_gone {
                crate::processing::without_retry_advice(&words)
            } else {
                words
            };
            let here =
                self.on_page && self.selected == Some(alert.session) && self.tab == Tab::Recordings;
            let can_record = alert.can_record && self.live.is_none();
            #[derive(PartialEq)]
            enum Primary {
                Input,
                Retry,
                Record,
            }
            let primary = if !banner_primary {
                None
            } else if missing_input {
                Some(Primary::Input)
            } else if alert.retry.is_some() {
                Some(Primary::Retry)
            } else if can_record {
                Some(Primary::Record)
            } else {
                None
            };
            let kind = |which: Primary| {
                if primary == Some(which) {
                    Kind::Primary
                } else {
                    Kind::Safe
                }
            };
            let mut choices = Vec::new();
            if missing_input {
                choices.push(choice(
                    "Choose an input",
                    kind(Primary::Input),
                    BannerAct::AlertInput(index),
                ));
            }
            if alert.retry.is_some() {
                let mut retry = choice(
                    "Continue processing",
                    kind(Primary::Retry),
                    BannerAct::AlertRetry(index),
                );
                retry.blocked = if audio_gone {
                    Some("Audio removed. Restore the WAV file to process it.".to_owned())
                } else if self.job.is_some() || self.live.is_some() {
                    Some("Stop recording or wait for processing to finish.".to_owned())
                } else {
                    None
                };
                choices.push(retry);
            }
            if can_record {
                choices.push(choice(
                    "Record again",
                    kind(Primary::Record),
                    BannerAct::AlertRecord(index),
                ));
            }
            if !here {
                choices.push(choice(
                    "Open the session",
                    Kind::Safe,
                    BannerAct::AlertOpen(index),
                ));
            }
            choices.push(choice(
                crate::design::outcome::DISMISS_NAME,
                Kind::Dismiss,
                BannerAct::AlertDismiss(index),
            ));
            messages.push(BannerMessage {
                tone: Tone::Error,
                words,
                hover: alert.said.clone(),
                choices,
            });
        }
        // What arrived on its own (`banner_notices`).
        let notes = self.on_page && self.view == View::Notes && self.form.is_none();
        let other = if notes && self.other_recording_shown() {
            self.live
                .as_ref()
                .zip(self.recording_status())
                .map(|(live, status)| OtherRecording {
                    status,
                    words: format!(
                        "{} {}",
                        status.word,
                        super::clock(live.recorder.seconds())
                    ),
                    detail: format!(" in {}", live.label),
                })
        } else {
            None
        };
        if notes
            && self.notice_arrived()
            && !self.processed_notice_read_here()
            && (self.notice_session().is_none() || self.notice_session() == self.selected)
            && let Some(notice) = self.notice()
        {
            messages.push(BannerMessage {
                tone: notice.tone,
                words: notice.words.clone(),
                hover: None,
                choices: vec![choice(
                    crate::design::outcome::DISMISS_NAME,
                    Kind::Dismiss,
                    BannerAct::NoticeDismiss,
                )],
            });
        }
        if notes && let Some(note) = self.gv_selected_note() {
            for copy in &note.also {
                let filename = copy.file_name().unwrap_or_default().to_string_lossy();
                messages.push(BannerMessage {
                    tone: Tone::Warning,
                    words: format!(
                        "{} and {filename} claim to be the same session.",
                        note.path.file_name().unwrap_or_default().to_string_lossy()
                    ),
                    hover: None,
                    choices: vec![choice(
                        &format!("Make {filename} a separate session"),
                        Kind::Safe,
                        BannerAct::Separate(copy.clone()),
                    )],
                });
            }
        }
        if notes && !self.unreadable.is_empty() {
            messages.push(BannerMessage {
                tone: Tone::Warning,
                words: format!(
                    "Splat cannot read the properties of {}, so it is not listed. The file is safe. Repair it to bring it back.",
                    self.unreadable.join(", ")
                ),
                hover: None,
                choices: Vec::new(),
            });
        }
        let name = self.note_name();
        if let Some(draft) = self.draft.as_ref().filter(|draft| draft.needs_attention()) {
            messages.extend(draft_warning(draft, name.as_deref(), false));
        }
        if self.on_page
            && self.view == View::Templates
            && let Some(draft) = self
                .template_draft
                .as_ref()
                .filter(|draft| draft.needs_attention())
        {
            messages.extend(draft_warning(draft, None, true));
        }
        Some(Banner { other, messages })
    }

    /// A press in the banner (the end of `banner`, `banner_notices` and `draft_warning`).
    pub fn gv_banner_act(
        &mut self,
        act: BannerAct,
        ctx: &egui::Context,
        settings: &ProcessingSettings,
        book: &Book,
    ) {
        let mut record_again = None;
        let mut open = None;
        match act {
            BannerAct::StalledRecord => record_again = self.live.as_ref().map(|live| live.session),
            BannerAct::StalledOpen => open = self.live.as_ref().map(|live| live.session),
            BannerAct::StalledStop => self.finish_recording(ctx),
            BannerAct::AlertInput(_) => self.wants_recording_input = true,
            BannerAct::AlertRetry(index) => {
                if let Some((session, path)) = self
                    .alerts
                    .get(index)
                    .and_then(|alert| Some((alert.session, alert.retry.clone()?)))
                {
                    self.process_recording(ctx, &path, settings, book, session);
                }
            }
            BannerAct::AlertRecord(index) => {
                record_again = self.alerts.get(index).map(|alert| alert.session);
            }
            BannerAct::AlertOpen(index) => open = self.alerts.get(index).map(|alert| alert.session),
            BannerAct::AlertDismiss(index) => {
                if index < self.alerts.len() {
                    self.alerts.remove(index);
                }
            }
            BannerAct::NoticeDismiss => self.end_notice(),
            BannerAct::Separate(copy) => match crate::sessions::new_identity(&copy) {
                Ok(undo) => {
                    self.separate.record(super::kept::Separate {
                        page: self.page(),
                        undo,
                    });
                    self.refresh();
                }
                Err(error) => self.say(crate::design::Tone::Error, error, None),
            },
            BannerAct::Draft(template, act) => {
                let draft = if template {
                    self.template_draft.as_mut()
                } else {
                    self.draft.as_mut()
                };
                if let Some(draft) = draft {
                    super::apply_draft_act(draft, &act);
                    super::keep_dropped(&mut self.dropped, draft, Some(act));
                }
            }
        }
        if let Some(session) = record_again {
            self.finish_recording(ctx);
            if Some(session) != self.selected {
                self.select(Some(session));
            }
            if let Err(error) = self.start_recording(settings, session) {
                self.raise(super::Alert {
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
}

/// One part of the note's Read view (`show_prepared`).
pub enum ReadPart {
    /// Markdown to read.
    Words(String),
    /// An embedded heading of another session's note, under `From <session>`.
    Embed {
        from: Uuid,
        label: String,
        words: String,
    },
    /// An embed whose source is gone: what Read says in its place.
    Gone(String),
}

/// The note's Read view (`read_tab`, `read_body`).
pub struct ReadView {
    pub written: bool,
    pub before: Vec<ReadPart>,
    /// The plan's count line in the tab layout, and what follows it.
    pub summary: Option<String>,
    pub after: Vec<ReadPart>,
}

impl Sessions {
    /// The note's words as its editor shows them: the whole note on the page; in the Notes panel,
    /// its slice from `## What happened` on (`PanelSlice`).
    pub fn gv_note_words(&mut self, in_panel: bool) -> Option<String> {
        let draft = self.draft.as_mut()?;
        if !in_panel {
            return Some(draft.body.as_str().to_owned());
        }
        let now = draft.body.key();
        let session = self.selected;
        match self
            .panel_slice
            .as_mut()
            .filter(|slice| slice.session == session)
        {
            Some(slice) if slice.body == now => {}
            Some(slice) => {
                let text = draft.body.as_str();
                let (prefix, tail) = text.split_at(super::PanelSlice::start(text));
                if !slice.tail.same_words(tail) {
                    let _ = slice.tail.replace(tail.to_owned());
                }
                prefix.clone_into(&mut slice.prefix);
                slice.body = now;
            }
            None => self.panel_slice = Some(super::PanelSlice::of(session, &draft.body)),
        }
        draft.mark_followed();
        self.panel_slice
            .as_ref()
            .map(|slice| slice.tail.as_str().to_owned())
    }

    /// The DM's edit in the note's editor: its whole words after it (`note_editor`).
    pub fn gv_note_edited(&mut self, in_panel: bool, words: &str) {
        let Some(draft) = &mut self.draft else {
            return;
        };
        if in_panel {
            let Some(slice) = self
                .panel_slice
                .as_mut()
                .filter(|slice| slice.session == self.selected)
            else {
                return;
            };
            if slice.tail.same_words(words) {
                return;
            }
            let _ = slice.tail.replace(words.to_owned());
            let _ = draft
                .body
                .replace(format!("{}{}", slice.prefix, slice.tail.as_str()));
            slice.body = draft.body.key();
            draft.mark_followed();
        } else {
            if draft.body.same_words(words) {
                return;
            }
            let before = draft.body.key();
            let _ = draft.body.replace(words.to_owned());
            draft.own_write(before);
        }
        draft.edited();
        if let Some(session) = self.selected {
            self.mentions
                .insert(session, crate::timeline::referenced_ids(&draft.body));
        }
        self.break_before_words = false;
    }

    /// Ctrl+S in the note: save now.
    pub fn gv_save_note(&mut self) {
        if !self.modal
            && let Some(draft) = &mut self.draft
        {
            draft.save();
        }
    }

    /// The note asked for the keyboard (a new session, `Write`, the Notes panel): a fresh line at
    /// the end of `What happened`, and the caret there, as a byte offset of the editor's words.
    pub fn gv_take_focus(&mut self, in_panel: bool) -> Option<usize> {
        if !std::mem::take(&mut self.focus_notes) {
            return None;
        }
        let draft = self.draft.as_mut()?;
        self.break_before_words = false;
        let (caret, changed) = draft.body.edit(crate::sessions::fresh_line_in_note);
        if changed {
            draft.edited();
        }
        let text = draft.body.as_str();
        let byte = text
            .char_indices()
            .nth(caret)
            .map_or(text.len(), |(byte, _)| byte);
        if in_panel {
            let start = super::PanelSlice::start(text);
            return Some(byte.saturating_sub(start));
        }
        Some(byte)
    }

    /// Words typed elsewhere that belong at the note's caret (`words_for_note`), with the editor's
    /// caret: the new words and the caret after them, or `None` when nothing waits.
    pub fn gv_take_words(&mut self, words: &str, caret: usize) -> Option<(String, usize)> {
        if !std::mem::take(&mut self.refocus_note) {
            return None;
        }
        let added = std::mem::take(&mut self.words_for_note);
        let new_line = std::mem::take(&mut self.words_new_line);
        if added.is_empty() {
            return Some((words.to_owned(), caret));
        }
        self.break_before_words = false;
        let at = caret.min(words.len());
        let before = &words[..at];
        let ends_words = !before.is_empty()
            && !before.ends_with('\n')
            && words[at..].chars().next().is_none_or(|c| c == '\n');
        let added = if new_line && ends_words {
            format!("\n\n{added}")
        } else {
            added
        };
        let mut text = words.to_owned();
        text.insert_str(at, &added);
        Some((text, at + added.len()))
    }

    /// Whether the note's editor should ask for the keyboard now (the Notes panel opened, words
    /// typed elsewhere wait), without taking the request.
    pub fn gv_wants_focus(&self) -> bool {
        self.focus_notes || self.refocus_note
    }

    /// The editor took the keyboard: in a narrow page, the note's tab stays.
    pub fn gv_note_has_focus(&mut self, in_panel: bool, focused: bool) {
        self.note_had_focus = focused;
        if !in_panel && focused && self.tab == Tab::Plan {
            // Narrowing keeps whichever side the DM last worked in.
            self.tab = Tab::Note;
        }
    }

    /// The note's Read view, prepared once per change of the note (`read_tab`).
    pub fn gv_read(
        &mut self,
        index: Option<&crate::lookup::ContentIndex>,
        plan: &Plan<'_>,
        beside: bool,
        book: &Book,
    ) -> Option<ReadView> {
        self.draft.as_ref()?;
        let model = self.plan_model(index, plan);
        let prepared = self.prepared_read(index, &model, plan, beside, book)?;
        Some(ReadView {
            written: prepared.written,
            before: Self::read_parts(&prepared, false),
            summary: prepared.summary.clone(),
            after: Self::read_parts(&prepared, true),
        })
    }

    /// Read's `Write`: the note, ready for words.
    pub fn gv_write(&mut self) {
        self.write_in_note();
        self.focus_notes = true;
    }

    /// A link pressed in the note: an event opens in the Timeline, a name in Lookup.
    pub fn gv_open_link(&mut self, target: &str) {
        self.open_note_link(target.to_owned());
    }

    /// A conflict's two texts: the draft and the file as it is now (`show_disk`).
    pub fn gv_conflict_texts(&mut self) -> Option<(String, String)> {
        let draft = self.draft.as_mut()?;
        if draft.state != DraftState::Conflict {
            return None;
        }
        if draft.on_disk.is_none() {
            draft.show_disk();
        }
        let disk = draft.on_disk.clone()?;
        Some((draft.body.as_str().to_owned(), disk))
    }
}

/// The Notes panel's head (`notes_panel`).
pub enum PanelState {
    /// No session yet: `New session` in place of the editor.
    NoSession,
    /// New events go to no session.
    NoCurrent,
    /// The note, with its title and date.
    Note { title: String, date: Option<String> },
}

impl Sessions {
    pub fn gv_panel(&self) -> PanelState {
        let note = self.gv_selected_note();
        if self.selected.is_none() {
            return PanelState::NoSession;
        }
        if self.current().is_none() {
            return PanelState::NoCurrent;
        }
        PanelState::Note {
            title: note.map_or_else(|| "Notes".to_owned(), Note::label),
            date: note
                .map(|note| note.date.clone())
                .filter(|date| !date.is_empty()),
        }
    }

    pub fn gv_panel_title(&self) -> (String, Option<String>) {
        let note = self.gv_selected_note();
        (
            note.map_or_else(|| "Notes".to_owned(), Note::label),
            note.map(|note| note.date.clone())
                .filter(|date| !date.is_empty()),
        )
    }

    /// The panel's `New session`, from anywhere.
    pub fn gv_start_new_session(&mut self) {
        self.start_new_session();
    }

    pub fn gv_panel_footer(&mut self) -> Vec<FooterLine> {
        let frame = PaneFrame::headless();
        let messages = self.panel_messages(&frame, 0.0);
        footer(messages)
    }

    pub fn gv_panel_press(&mut self, press: FooterPress) {
        self.panel_press(press.0.id);
    }
}

/// One row of the plan, in the plan's order: the session's events, then each place's scenes
/// before its planned fight, the scenes after the last, then the planned groups.
pub enum PlanRow {
    Event(EventRow),
    Fight(FightRow),
    Group(GroupRow),
    Scene(SceneRow),
}

pub struct EventRow {
    pub id: String,
    pub icon: &'static str,
    pub label: String,
    /// Its summary, and the part of it that is a warning (a length the clock cannot count).
    pub summary: String,
    pub warning: Option<String>,
    /// A long rest's ruling: whether it restores, as a status after the summary.
    pub restores: Option<crate::design::Status<'static>>,
    pub open: bool,
    /// Its fight runs now: the summary leads back to the run.
    pub running: bool,
    /// A paused run to resume, and the fight it was started from.
    pub saved: Option<crate::encounter_run::SavedRunInfo>,
    pub saved_fight: Option<String>,
    /// An ended fight's run, whose rewards the row offers.
    pub ended: Option<Uuid>,
    /// Its rewards are in the note.
    pub listed: bool,
    /// The note links it (`In notes`).
    pub linked: bool,
    /// `Add to notes`: the link it adds.
    pub reference: Option<String>,
    /// `Roll for encounters` is offered (a journey with marked tables).
    pub rolls: bool,
    /// The open row's details: the fight's lines, or the event's fields.
    pub details: Option<EventDetails>,
}

/// An open event's details (a simpler form of the egui plan's open row).
pub struct EventDetails {
    pub title: String,
    pub notes: String,
    pub place: String,
    /// A played fight's lines, as the Timeline says them.
    pub fight: Option<String>,
    /// A long rest's ruling can be set here.
    pub rest: Option<bool>,
}

pub struct FightRow {
    pub position: usize,
    pub name: String,
    pub path: String,
    pub id: Uuid,
    /// The roster's words, then the difficulty's parts (`true`: the word, shown strong).
    pub roster: String,
    pub difficulty: Vec<(String, bool)>,
    pub running: bool,
    pub saved: Option<crate::encounter_run::SavedRunInfo>,
    pub open: bool,
}

pub struct GroupRow {
    pub position: usize,
    pub name: String,
    pub path: String,
    pub summary: String,
    pub edit_word: &'static str,
    pub joins_fight: bool,
    pub warnings: Vec<String>,
    pub problems: Vec<String>,
    /// The DM pressed its name while an unsaved draft refuses the open: why.
    pub refused: Option<String>,
}

pub struct SceneRow {
    pub slot: (usize, usize),
    pub line: usize,
    pub done: bool,
    pub words: String,
    pub detail: Option<String>,
    pub not_found: bool,
    /// `Start fight` with its NPC or creature, while no run is open.
    pub start: Option<String>,
    /// A done scene's `Add to Log`.
    pub log: bool,
}

/// The plan as the view draws it.
pub struct PlanView {
    /// `Tonight` or `This session`, and its count.
    pub heading: &'static str,
    pub count: usize,
    pub rows: Vec<PlanRow>,
    /// The Timeline file cannot be read: adds wait for its repair.
    pub add_blocked: Option<&'static str>,
    /// Why no fight can start, why no group can be assembled.
    pub fight_blocked: Option<String>,
    pub group_blocked: Option<String>,
    /// The fights and groups the Add menus list.
    pub fights: Vec<String>,
    pub groups: Vec<String>,
    /// The Scene field is open, with its words.
    pub scene_field: Option<String>,
    /// The first primary action is free (no run open, no recording step).
    pub primary_free: bool,
    pub open_run: bool,
}

/// What the DM did in the plan.
#[derive(Clone, Debug)]
pub enum PlanAct {
    /// An Add strip kind: Day, Long rest, Short rest, Travel, Event.
    Add(crate::timeline::Kind),
    /// `Scene`: the Scene field opens.
    OpenScene,
    SceneWords(String),
    SceneAdd,
    SceneCancel,
    /// A fight or group from the Add menus, by name.
    PlanName(String),
    NewFight,
    NewGroup,
    ToggleEvent(String),
    TogglePlanned(String),
    RemoveEvent(String),
    AddToNotes(String),
    StartFight(String),
    Resume(PathBuf),
    DeleteRun(Uuid),
    RemoveFight(usize, String),
    EditEncounter(Uuid),
    RemoveGroup(usize, String),
    EditGroup(String),
    ReadGroup(String),
    AddToFight,
    SceneTick(usize),
    SceneRemove(usize),
    SceneStart(usize),
    SceneLog(usize),
    AddRewards(String, Uuid),
    BackToRun,
    ReadEncounter(String),
    OpenEventOnTimeline(String),
    EventTitle(String, String),
    EventNotes(String, String),
    EventPlace(String, String),
    RestRuling(String, bool),
    RollForEncounters(String),
}

impl Sessions {
    /// The plan of the selected session, as rows and what its Add strip offers.
    pub fn gv_plan(
        &mut self,
        book: &mut Book,
        plan: &Plan<'_>,
        index: Option<&crate::lookup::ContentIndex>,
    ) -> PlanView {
        let session = self.selected;
        self.end_plan_results_after_edits(book);
        self.scenes.end_what_changed(self.draft.as_ref());
        self.follow_plan_line();
        self.rewards.see(self.draft.as_ref(), session, book);
        let projection = self.plan_projection(book, session, plan, index);
        let days = &projection.days;
        let mut rows = Vec::new();
        for (at, event) in book.events.iter().enumerate() {
            if !event.is_in(session) {
                continue;
            }
            let day = days.get(at).copied().unwrap_or(0);
            let label = event.label(day);
            let open = self.open_event.as_ref() == Some(&event.id);
            let running = event.run.is_some() && event.run == plan.open_run;
            let saved = super::plan::planned_event_run(event, plan, running).cloned();
            let ended = event.run.filter(|_| !running && saved.is_none());
            let saved_fight = plan
                .fights
                .iter()
                .find(|fight| {
                    fight
                        .saved
                        .as_ref()
                        .is_some_and(|s| s.id == event.run && event.run.is_some())
                })
                .map(|fight| fight.path.clone());
            let summary = if running {
                plan.open_state.clone()
            } else if let Some(saved) = &saved {
                crate::encounters::run_state_word(saved, true)
            } else if event.run.is_some() {
                crate::timeline::played_fight_summary(event, running)
            } else {
                event.summary(None)
            };
            let warning = event.time_warning().filter(|_| event.run.is_none());
            let restores = (event.kind == crate::timeline::Kind::LongRest && saved.is_none() && !running)
                .then(|| crate::timeline::page::long_rest_status(event.restores));
            let details = open.then(|| EventDetails {
                title: event.title.clone(),
                notes: event.notes.clone(),
                place: event.place.clone(),
                fight: event.run.map(|_| crate::timeline::played_fight_summary(event, running)),
                rest: (event.kind == crate::timeline::Kind::LongRest).then_some(event.restores),
            });
            rows.push(PlanRow::Event(EventRow {
                id: event.id.clone(),
                icon: crate::timeline::kind_icon(event.kind),
                label,
                summary,
                warning,
                restores,
                open,
                running,
                saved,
                saved_fight,
                ended,
                listed: self.rewards.listed(&event.id),
                linked: projection.linked.contains(&event.id),
                reference: (!running && event.run.is_none())
                    .then(|| crate::timeline::reference(event, day)),
                rolls: crate::timeline::checks::offered(event, plan.tables),
                details,
            }));
        }
        let scene_rows = |rows: &mut Vec<PlanRow>, place: usize| {
            for (index, &line) in projection.scenes[place].iter().enumerate() {
                let scene = projection.model.scene(line);
                let entry = scene.entry.as_ref();
                rows.push(PlanRow::Scene(SceneRow {
                    slot: (place, index),
                    line,
                    done: scene.task.done,
                    words: scene.task.words.clone(),
                    detail: entry.and_then(|entry| entry.detail.clone()),
                    not_found: scene.not_found,
                    start: entry
                        .and_then(|entry| entry.start.as_ref())
                        .filter(|_| plan.open_run.is_none())
                        .map(|fight| fight.name.clone()),
                    log: scene.task.done && entry.and_then(|e| e.log.as_ref()).is_some(),
                }));
            }
        };
        for (position, &at) in projection.planned.iter().enumerate() {
            scene_rows(&mut rows, position);
            let fight = &plan.fights[at];
            let running =
                plan.open_run.is_some() && plan.open_encounter == Some(fight.name.as_str());
            let saved =
                super::plan::planned_saved_run(fight, plan.fights, plan.saved_runs).cloned();
            rows.push(PlanRow::Fight(FightRow {
                position,
                name: fight.name.clone(),
                path: fight.path.clone(),
                id: fight.id,
                roster: fight.summary.text(),
                difficulty: fight.difficulty.parts().unwrap_or_default(),
                running,
                saved,
                open: self.open_planned.as_ref() == Some(&fight.path),
            }));
        }
        scene_rows(&mut rows, projection.planned.len());
        for (position, &at) in projection.planned_groups.iter().enumerate() {
            let group = &plan.groups[at];
            let refused = plan
                .group_blocked
                .filter(|_| self.plan_group_refused == Some((session, group.id)))
                .map(str::to_owned);
            rows.push(PlanRow::Group(GroupRow {
                position,
                name: group.name.clone(),
                path: group.path.clone(),
                summary: group.summary(),
                edit_word: if group.warnings.is_empty() {
                    "Edit"
                } else {
                    "Edit group"
                },
                joins_fight: plan.arrivals_open && !group.members.is_empty(),
                warnings: group
                    .warnings
                    .iter()
                    .map(|warning| crate::design::outcome::fault_words(warning).as_str().to_owned())
                    .collect(),
                problems: group.own_problems().into_iter().map(str::to_owned).collect(),
                refused,
            }));
        }
        PlanView {
            heading: if session == self.current() {
                "Tonight"
            } else {
                "This session"
            },
            count: projection.events_here
                + projection.planned.len()
                + projection.planned_groups.len()
                + projection.scene_count,
            rows,
            add_blocked: book
                .unreadable
                .is_some()
                .then_some("The timeline file must be repaired first."),
            fight_blocked: plan.blocked.map(str::to_owned),
            group_blocked: plan.group_blocked.map(str::to_owned),
            fights: plan.fights.iter().map(|fight| fight.name.clone()).collect(),
            groups: plan.groups.iter().map(|group| group.name.clone()).collect(),
            scene_field: self.scenes.field.as_ref().map(|field| field.words.clone()),
            primary_free: plan.open_run.is_none() && !self.confirm_record,
            open_run: plan.open_run.is_some(),
        }
    }

    /// The plan pane's footer (`plan_footer_messages`): with the page's own messages when the
    /// plan is all the pane shows (the narrow Plan tab).
    pub fn gv_plan_footer(
        &mut self,
        settings: &ProcessingSettings,
        book: &mut Book,
        plan: &Plan<'_>,
        index: Option<&crate::lookup::ContentIndex>,
        with_page: bool,
    ) -> Vec<FooterLine> {
        let session = self.selected;
        let projection = self.plan_projection(book, session, plan, index);
        let removed_slot = self
            .scenes
            .removed
            .as_ref()
            .and_then(|(line, ..)| projection.line_place.get(*line).copied());
        let rows = Some(projection.scenes.as_slice());
        let shown = super::plan::Shown {
            added: self
                .scenes
                .added
                .as_ref()
                .map(|_| self.shown_at(super::plan::Place::Field, rows)),
            removed: self.scenes.removed.as_ref().map(|_| {
                self.shown_at(
                    removed_slot.map_or(super::plan::Place::Footer, super::plan::Place::Row),
                    rows,
                )
            }),
            fault: self
                .scenes
                .fault
                .as_ref()
                .map(|fault| self.shown_at(fault.place, rows)),
        };
        let note = self.gv_selected_note().cloned();
        let page = with_page.then_some(()).and(note.as_ref()).map(|note| (settings, note));
        let frame = PaneFrame::headless();
        let messages = self.plan_footer_messages(
            &frame,
            book,
            plan,
            page,
            (shown, &projection.planned_results),
            0.0,
        );
        footer(messages)
    }

    /// A press in the plan pane's footer (`plan_footer_press`), with its scene and rewards acts.
    pub fn gv_plan_press(&mut self, press: FooterPress, book: &mut Book, plan: &Plan<'_>) {
        let session = self.selected;
        let (scene, rewards) = self.plan_footer_press(press.0, book, plan);
        if let Some(act) = scene {
            self.scene_act(act, session);
        }
        if let Some(rewards) = rewards {
            self.do_rewards_act(rewards, book, plan);
        }
    }

    /// The DM's act in the plan's rows or Add strip.
    pub fn gv_plan_act(
        &mut self,
        act: PlanAct,
        book: &mut Book,
        plan: &Plan<'_>,
        index: Option<&crate::lookup::ContentIndex>,
        ctx: &egui::Context,
    ) {
        use super::plan::{RewardsAct, SceneAct};
        let session = self.selected;
        let scene_of = |this: &mut Self, book: &mut Book, line: usize| {
            let projection = this.plan_projection(book, session, plan, index);
            let place = projection
                .scenes
                .iter()
                .enumerate()
                .find_map(|(place, lines)| {
                    lines
                        .iter()
                        .position(|&at| at == line)
                        .map(|index| (place, index))
                })?;
            let scene = projection.model.scene(line);
            Some((
                place,
                scene.task.clone(),
                scene.entry.as_ref().and_then(|e| e.start.clone()),
                scene.entry.as_ref().and_then(|e| e.log.clone()),
            ))
        };
        match act {
            PlanAct::Add(kind) => {
                if book.unreadable.is_some() {
                    return;
                }
                let id = if kind == crate::timeline::Kind::LongRest {
                    book.add_long_rest(session)
                } else {
                    book.add(kind, session)
                };
                self.record_plan_add(&id, book, ctx);
            }
            PlanAct::OpenScene => {
                let mut field = crate::design::AddField::default();
                field.focus = true;
                self.scenes.field = Some(field);
            }
            PlanAct::SceneWords(words) => {
                if let Some(field) = &mut self.scenes.field {
                    field.words = words;
                }
            }
            PlanAct::SceneAdd => {
                let typed = self
                    .scenes
                    .field
                    .as_ref()
                    .map(|field| field.words.trim().to_owned())
                    .unwrap_or_default();
                if !typed.is_empty() {
                    self.scene_act(SceneAct::Add(typed), session);
                }
            }
            PlanAct::SceneCancel => self.scenes.field = None,
            PlanAct::PlanName(name) => self.add_to_plan(&name),
            PlanAct::NewFight => self.wants_new_fight = true,
            PlanAct::NewGroup => {
                self.wants_new_group = true;
                self.group_session = self.selected;
            }
            PlanAct::ToggleEvent(id) => self.toggle_plan_event(id),
            PlanAct::TogglePlanned(path) => {
                self.open_planned = (self.open_planned.as_ref() != Some(&path)).then_some(path);
            }
            PlanAct::RemoveEvent(id) => self.remove_plan_event(&id, book),
            PlanAct::AddToNotes(link) => {
                if session.is_some() && self.add_to_note(ctx, &link) {
                    let body = self.draft.as_ref().map_or("", |draft| draft.body.as_str());
                    if let Some(session) = session {
                        self.mentions
                            .insert(session, crate::timeline::referenced_ids(body));
                    }
                    self.end_plan_add();
                }
            }
            PlanAct::StartFight(path) => self.wants_fight = Some(path),
            PlanAct::Resume(path) => self.wants_saved_run = Some(path),
            PlanAct::DeleteRun(id) => {
                if let Some(saved) = plan.saved_runs.iter().find(|saved| saved.id == Some(id)) {
                    self.delete_plan_run(&saved.clone(), book);
                }
            }
            PlanAct::RemoveFight(position, name) => {
                let projection = self.plan_projection(book, session, plan, index);
                self.remove_planned_fight((position, name), book, plan, &projection.planned);
            }
            PlanAct::EditEncounter(id) => self.wants_edit_encounter = Some(id),
            PlanAct::RemoveGroup(position, name) => {
                self.remove_planned_group((position, name, 0.0), book);
            }
            PlanAct::EditGroup(path) => {
                self.wants_group = Some(path);
                self.group_session = self.selected;
            }
            PlanAct::ReadGroup(path) => {
                if plan.group_blocked.is_some() {
                    if let Some(group) = plan.groups.iter().find(|group| group.path == path) {
                        self.plan_group_refused = Some((self.selected, group.id));
                    }
                } else {
                    self.plan_group_refused = None;
                    self.wants_group = Some(path);
                    self.group_session = self.selected;
                }
            }
            PlanAct::AddToFight => self.wants_arrivals = true,
            PlanAct::SceneTick(line) => {
                if let Some((slot, task, _, _)) = scene_of(self, book, line) {
                    self.scene_act(SceneAct::Tick(slot, task), session);
                }
            }
            PlanAct::SceneRemove(line) => {
                if let Some((slot, task, _, _)) = scene_of(self, book, line) {
                    self.scene_act(SceneAct::Remove(slot, line, task), session);
                }
            }
            PlanAct::SceneStart(line) => {
                if let Some((_, _, Some(fight), _)) = scene_of(self, book, line) {
                    self.scene_act(SceneAct::StartFight(fight), session);
                }
            }
            PlanAct::SceneLog(line) => {
                if let Some((_, task, _, Some((id, name)))) = scene_of(self, book, line) {
                    self.scene_act(
                        SceneAct::AddToLog(id, name, super::plan::words_outside_links(&task.words)),
                        session,
                    );
                }
            }
            PlanAct::AddRewards(event, run) => {
                self.do_rewards_act((RewardsAct::Add(event, run), None), book, plan);
            }
            PlanAct::BackToRun => self.wants_back_to_run = true,
            PlanAct::ReadEncounter(name) => self.wants_lookup = Some(name),
            PlanAct::OpenEventOnTimeline(id) => self.wants_event = Some(id),
            PlanAct::EventTitle(id, words) => {
                if let Some(event) = book.events.iter_mut().find(|event| event.id == id) {
                    event.title = words;
                    book.touch();
                }
            }
            PlanAct::EventNotes(id, words) => {
                if let Some(event) = book.events.iter_mut().find(|event| event.id == id) {
                    event.notes = words;
                    book.touch();
                }
            }
            PlanAct::EventPlace(id, words) => {
                if let Some(event) = book.events.iter_mut().find(|event| event.id == id) {
                    event.place = words;
                    book.touch();
                }
            }
            PlanAct::RestRuling(id, restores) => {
                crate::timeline::page::RestRulingAction::Set(restores).apply(book, &id);
                self.end_plan_add();
            }
            PlanAct::RollForEncounters(id) => {
                self.row_reveal = Some((id.clone(), 0, ctx.cumulative_pass_nr()));
                self.travel_act(book, session, &id, crate::timeline::checks::Act::Start, plan);
            }
        }
    }
}
