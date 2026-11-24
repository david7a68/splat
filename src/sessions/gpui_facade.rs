//! What the GPUI Session page (`src/gui/pages/sessions/`) reads from the page's state and the
//! acts it asks for. Toolkit-free. Each act calls the page's own methods, so a press makes one
//! state change wherever it comes from. The GPUI view holds no copy of the page's logic; what it needs from a private field is read here.
use super::{
    DraftState, Form, Note, Plan, ProcessingSettings, SessionAct, Sessions, Tab, Template, View,
    form_reason,
};
use crate::design::CreateReasons as _;
use crate::design::outcome::{PaneFrame, Press, plain};
use crate::state_context::StateContext;
use crate::timeline::Book;
use crate::timeline::EventDisplay as _;
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

/// A press in a message footer (`design::outcome::Press`): it carries a kept line's ticket.
pub type FooterPress = Press<SessionAct>;

/// One footer line, in the footer's order (`design::outcome::plain`).
pub type FooterLine = plain::PlainLine<SessionAct>;

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
        self.draft
            .as_ref()
            .is_some_and(super::Draft::needs_attention)
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
    pub fn gv_stop_recording(&mut self, ctx: &StateContext) {
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
                let path =
                    super::super::note_path(folder, form.number.parse().unwrap_or(1), &form.title);
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

    /// A press on a tab: 0 Plan, 1 Note, 2 Recordings.
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
        let frame = PaneFrame::new();
        let messages = self.session_messages(&frame, settings, book, plan, &note);
        messages.into_plain()
    }

    /// A press in the session page's footer (`session_press`).
    pub fn gv_session_press(&mut self, press: FooterPress, book: &mut Book, plan: &Plan<'_>) {
        self.session_press(press, book, plan);
    }

    /// The footer of a pane whose body is the draft's editor (`draft_messages`).
    pub fn gv_draft_footer(&mut self, template: bool) -> Vec<FooterLine> {
        let frame = PaneFrame::new();
        let messages = self.draft_messages(&frame, template);
        messages.into_plain()
    }

    pub fn gv_draft_press(&mut self, press: FooterPress, template: bool) {
        self.draft_press(press, template);
    }
}

/// The record control beside the title (`record_control`).
pub enum RecordControl {
    None,
    /// `Stop recording`, named for its session when it is another's.
    Stop {
        name: String,
    },
    /// `Record`, and why it waits (shown under it too).
    Record {
        blocked: Option<String>,
    },
}

/// The record line under the title.
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

/// The Last time line.
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

    /// The record line's state now.
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

    /// The Last time line, read once for each selection (`see_last_time`).
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
    /// writing.
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

    /// An Add menu's search, of the Group menu when `group`, else the Fight
    /// menu's: its words while the menu lists more than `FIGHT_SEARCH_FROM` names (`None`: no
    /// search), and the names it lists for them.
    pub fn gv_menu_search<'a>(
        &self,
        group: bool,
        names: &'a [String],
    ) -> (Option<String>, Vec<&'a str>) {
        let search = if group {
            &self.group_search
        } else {
            &self.fight_search
        };
        let field = (names.len() > super::plan::FIGHT_SEARCH_FROM).then(|| search.clone());
        let shown = super::plan::searched(names.iter().map(String::as_str), search);
        (field, shown)
    }

    /// The DM typed in an Add menu's search.
    pub fn gv_set_menu_search(&mut self, group: bool, words: &str) {
        let search = if group {
            &mut self.group_search
        } else {
            &mut self.fight_search
        };
        words.clone_into(search);
    }

    /// A name chosen in an Add menu: it joins the plan, and the menu's search empties.
    pub fn gv_menu_choose(&mut self, group: bool, name: &str) {
        self.add_to_plan(name);
        self.gv_set_menu_search(group, "");
    }
}

/// What a press in the session banner does (`draft_warning`).
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
    pub kind: plain::ChoiceForm,
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

fn choice(words: &str, kind: plain::ChoiceForm, act: BannerAct) -> BannerChoice {
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
    use plain::ChoiceForm as Kind;
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
                choice(
                    "Overwrite with my draft",
                    Kind::Choice,
                    act(DraftAct::Overwrite),
                ),
                choice(
                    "Reload and lose my draft",
                    Kind::Lose,
                    act(DraftAct::Reload),
                ),
            ],
        )),
        DraftState::Missing if matches!(draft.dropped, Some(Dropped::Discarded(_))) => Some((
            Tone::Warning,
            words(super::DISCARDED_GONE),
            vec![choice(
                "Save these words as a new file",
                Kind::Choice,
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
                    Kind::Choice,
                    act(DraftAct::SaveMoved),
                ));
            }
            choices.push(choice(
                "Save my draft as a new file",
                Kind::Choice,
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
            vec![choice(
                "Try to save again",
                Kind::Choice,
                act(DraftAct::Save),
            )],
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
    /// The session banner, or `None` when it has nothing to say.
    pub fn gv_banner(&self, settings: &ProcessingSettings, run_open: bool) -> Option<Banner> {
        use crate::design::Tone;
        use plain::ChoiceForm as Kind;
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
                Kind::Choice
            };
            let mut choices = vec![choice("Record again", record, BannerAct::StalledRecord)];
            if !here {
                choices.push(choice(
                    "Open the session",
                    Kind::Choice,
                    BannerAct::StalledOpen,
                ));
            }
            choices.push(choice(
                "Stop recording",
                Kind::Choice,
                BannerAct::StalledStop,
            ));
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
                    Kind::Choice
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
                    Kind::Choice,
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
        // What arrived on its own.
        let notes = self.on_page && self.view == View::Notes && self.form.is_none();
        let other = if notes && self.other_recording_shown() {
            self.live
                .as_ref()
                .zip(self.recording_status())
                .map(|(live, status)| OtherRecording {
                    status,
                    words: format!("{} {}", status.word, super::clock(live.recorder.seconds())),
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
                        Kind::Choice,
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

    /// A press in the banner (`draft_warning`'s choices among them).
    pub fn gv_banner_act(
        &mut self,
        act: BannerAct,
        ctx: &StateContext,
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

/// One part of the note's Read view.
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

/// The note's Read view.
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

    /// Whether the note's change since the Notes panel last held it was only its own writes (an
    /// act, an insert, the DM's typing on the page): the panel's editor keeps its Undo history.
    /// Ask before `gv_note_words`, which marks the panel's hold.
    pub fn gv_note_own_write(&self) -> bool {
        self.draft
            .as_ref()
            .is_some_and(super::Draft::own_since_followed)
    }

    /// The DM's edit in the note's editor: its whole words after it.
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

    /// After a step of the run, what the DM types next is a new thought (`after_a_step_back_to_note`):
    /// the first typed words after the end of a line of words start a paragraph of their own.
    /// `before` and `words` are the editor's words around one edit, and `caret` its caret after
    /// it, as byte offsets. The words with the break and the caret after them, or `None` when the
    /// edit was not typing at the end of a line of words, or no break waits.
    pub fn gv_new_thought(
        &mut self,
        before: &str,
        words: &str,
        caret: usize,
    ) -> Option<(String, usize)> {
        if !self.break_before_words || words.len() <= before.len() {
            return None;
        }
        // The edit put words in at the caret and took none out.
        let start = caret.checked_sub(words.len() - before.len())?;
        if !words.is_char_boundary(start)
            || !words.is_char_boundary(caret)
            || !before.is_char_boundary(start)
            || words[..start] != before[..start]
            || words[caret..] != before[start..]
        {
            return None;
        }
        let ends_words = !before[..start].is_empty()
            && !before[..start].ends_with('\n')
            && before[start..].chars().next().is_none_or(|c| c == '\n');
        if !ends_words {
            return None;
        }
        let now = format!("{}\n\n{}{}", &before[..start], &words[start..caret], &before[start..]);
        Some((now, caret + 2))
    }

    /// A correction key in the note (Backspace, Delete, an arrow, Home, End): the DM goes on with
    /// the same line, so no new paragraph waits.
    pub fn gv_correcting(&mut self) {
        self.break_before_words = false;
    }

    /// Words typed while no control had the keyboard, beside the Notes panel: they belong in
    /// the note (egui `note_editor`: "A stray click leaves the keyboard on nothing").
    pub fn gv_stray_words(&mut self, words: &str) {
        self.words_for_note.push_str(words);
        self.words_new_line = self.break_before_words;
        self.refocus_note = true;
    }

    /// Whether the note's editor should ask for the keyboard now (the Notes panel opened, words
    /// typed elsewhere wait), without taking the request.
    pub fn gv_wants_focus(&self) -> bool {
        self.focus_notes || self.refocus_note
    }

    /// Whether the note had the keyboard in the last frame (the Notes panel's Ctrl+S waits for it).
    pub fn gv_note_had_focus(&self) -> bool {
        self.note_had_focus
    }

    /// Where the keyboard is: in the note's editor (`focused`), or in another control
    /// (`elsewhere`). Narrowing keeps whichever side the DM last worked in: the note while it has
    /// focus, the plan once focus moves to a plan control.
    pub fn gv_note_has_focus(&mut self, in_panel: bool, focused: bool, elsewhere: bool) {
        self.note_had_focus = focused;
        if in_panel {
            return;
        }
        if focused && self.tab == Tab::Plan {
            self.tab = Tab::Note;
        } else if !focused && elsewhere && self.tab == Tab::Note {
            self.tab = Tab::Plan;
        }
    }

    /// The note's Read view, prepared once per change of the note.
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

/// The Notes panel's head.
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
        let frame = PaneFrame::new();
        let messages = self.panel_messages(&frame);
        messages.into_plain()
    }

    pub fn gv_panel_press(&mut self, press: FooterPress) {
        self.panel_press(press.id);
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

/// An open event's details (a simple form of the plan's open row).
pub struct EventDetails {
    pub title: String,
    pub notes: String,
    pub place: String,
    /// A played fight's lines, as the Timeline says them.
    pub fight: Option<String>,
    /// A played fight's run: its block (`gv_fight_block`) shows how it ended. `None` while it
    /// runs (then `fight` says so).
    pub run: Option<Uuid>,
    /// The fight runs now: its final HP show when it ends.
    pub running: bool,
    /// A long rest's ruling can be set here.
    pub rest: Option<bool>,
    /// A journey: its encounter checks come first, under its row (`travel_block`).
    pub journey: bool,
}

/// The open journey's checks block in the plan (`Sessions::travel_block`): where it starts on the
/// clock, **How long** as the DM typed it with its help, whether that field takes the keyboard,
/// and what a check result can offer to this session's plan.
pub struct TravelView {
    pub id: String,
    pub index: usize,
    pub start: Option<u64>,
    pub length: String,
    pub help: Option<String>,
    pub focus_length: bool,
    pub plan: super::plan::JourneyPlan,
}

pub struct FightRow {
    pub position: usize,
    pub name: String,
    pub path: String,
    pub id: Uuid,
    /// The roster's words, then the difficulty's parts (`true`: the word, shown strong).
    pub roster: String,
    /// The roster with the party's size first (`5 party · …`), when the party has members: the
    /// line keeps it while the row has room for it.
    pub roster_with_party: Option<String>,
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
    /// The heading's `N scenes`, when the scenes fold or the DM opened them: the count, and
    /// whether they are folded now.
    pub fold: Option<(usize, bool)>,
    /// The scenes fold now: their rows are left out.
    pub folded: bool,
    /// A one-time reveal after an add or an act: the row after this one (or this one, when it
    /// is the last) scrolls into view (`Reveal`).
    pub reveal: Option<String>,
    /// The event row just opened: it comes to the plan's top (`reveal_event`).
    pub reveal_open: Option<String>,
}

pub use super::plan::{FightCell, log_notice, log_room};

/// How an open played fight ended (`Sessions::fight_block`): its players' final HP in the
/// party's order, `Log`, `Whole run` and `Timeline`, and the Log's rows while it is open; or
/// why it cannot show.
pub enum FightBlockView {
    Fault(plain::PlainLine<()>),
    Ended {
        cells: Vec<super::plan::FightCell>,
        /// The run's file, for `Whole run`.
        path: Option<PathBuf>,
        /// The Log's entries, newest first, while it is open.
        log: Option<Vec<String>>,
    },
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
    /// An act in an open journey's checks block (`travel_act`).
    Travel(String, crate::timeline::checks::Act),
    /// `N scenes` on the heading, pressed while the scenes were folded (`true`) or open.
    SceneFold(bool),
    /// A played fight's `Log`: its rows show or hide.
    FightLog,
    /// A played fight's `Whole run`: its saved run opens.
    WholeRun(PathBuf),
    /// `Undo` of the journey's last checks result (`travel.done.undo`).
    TravelUndo(String),
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
        let (folded, fold_line) = self.scene_fold(projection.scene_count);
        let reveal = self.row_reveal.take().map(|(id, _, _)| id);
        let mut reveal_open = None;
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
            let restores =
                (event.kind == crate::timeline::Kind::LongRest && saved.is_none() && !running)
                    .then(|| crate::timeline::page::long_rest_status(event.restores));
            if open && std::mem::take(&mut self.reveal_event) {
                reveal_open = Some(event.id.clone());
            }
            let details = open.then(|| EventDetails {
                title: event.title.clone(),
                notes: event.notes.clone(),
                place: event.place.clone(),
                fight: event
                    .run
                    .map(|_| crate::timeline::played_fight_summary(event, running)),
                run: event.run.filter(|_| !running),
                running,
                rest: (event.kind == crate::timeline::Kind::LongRest).then_some(event.restores),
                journey: event.run.is_none() && crate::timeline::checks::shown(event),
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
            if folded {
                return;
            }
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
                roster: crate::difficulty::Summary {
                    party: 0,
                    members: fight.summary.members.clone(),
                    facts: fight.summary.facts.clone(),
                }
                .text(),
                roster_with_party: (fight.summary.party > 0).then(|| fight.summary.text()),
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
                    .map(|warning| {
                        crate::design::outcome::fault_words(warning)
                            .as_str()
                            .to_owned()
                    })
                    .collect(),
                problems: group
                    .own_problems()
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
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
            fold: fold_line.then_some((projection.scene_count, folded)),
            folded,
            reveal,
            reveal_open,
        }
    }

    /// A played fight's Log is open.
    pub fn gv_fight_log_open(&self) -> bool {
        self.show_fight_log
    }

    /// What the GPUI plan measured in its last frame, for the scenes' fold (`note_fold_measure`).
    /// Returns whether the fold changes, so the plan draws again at once.
    pub fn gv_fold_measure(
        &mut self,
        folded: bool,
        viewport: f32,
        rows: (f32, f32, f32),
        next: Option<(f32, f32, f32)>,
    ) -> bool {
        self.note_fold_measure(folded, viewport, rows, next)
    }

    /// The open played fight's block (`fight_block`): `None` when the open row is no ended fight.
    pub fn gv_fight_block(
        &mut self,
        book: &Book,
        plan: &Plan<'_>,
    ) -> Option<(String, FightBlockView)> {
        let open = self.open_event.clone()?;
        let event = book.events.iter().find(|event| event.id == open)?;
        let run = event.run?;
        if plan.open_run == Some(run) {
            return None;
        }
        let path = self.keep_fight(run, plan);
        let (_, _, result, _) = self.fight.as_ref()?;
        let view = match result {
            Err(fault) => FightBlockView::Fault(super::plan::fight_fault_line(fault)),
            Ok(summary) => FightBlockView::Ended {
                cells: super::plan::fight_cells(summary, plan.party),
                path,
                log: self.show_fight_log.then(|| summary.log.clone()),
            },
        };
        Some((open, view))
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
        let (folded, _) = self.scene_fold(projection.scene_count);
        let rows = (!folded).then_some(projection.scenes.as_slice());
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
        let page = with_page
            .then_some(())
            .and(note.as_ref())
            .map(|note| (settings, note));
        let frame = PaneFrame::new();
        let messages = self.plan_footer_messages(
            &frame,
            book,
            plan,
            page,
            (shown, &projection.planned_results),
        );
        messages.into_plain()
    }

    /// A press in the plan pane's footer (`plan_footer_press`), with its scene and rewards acts.
    pub fn gv_plan_press(&mut self, press: FooterPress, book: &mut Book, plan: &Plan<'_>) {
        let session = self.selected;
        let (scene, rewards) = self.plan_footer_press(press, book, plan);
        if let Some(act) = scene {
            self.scene_act(act, session);
        }
        if let Some(rewards) = rewards {
            self.do_rewards_act(rewards, book, plan);
        }
    }

    /// The open journey's checks block, before it draws: the length that settled, the start on
    /// the clock, and the results that still hold. `None`
    /// when the open row is not a journey of this plan.
    pub fn gv_travel(
        &mut self,
        book: &mut Book,
        plan: &Plan<'_>,
        index: Option<&crate::lookup::ContentIndex>,
    ) -> Option<TravelView> {
        let open = self.open_event.clone()?;
        let (open_at, _) = self.journey_start(book, Some(&open));
        crate::timeline::checks::settle_length(book, open_at);
        let (at, start) = self.journey_start(book, Some(&open));
        self.travel.done.follow(book);
        let at = at?;
        let event = &book.events[at];
        if event.run.is_some()
            || !crate::timeline::checks::shown(event)
            || !event.is_in(self.selected)
        {
            return None;
        }
        let length = book.length.words(event);
        let help = crate::timeline::checks::span(start, &length).or_else(|| {
            crate::timeline::read_duration(&length)
                .is_none()
                .then(|| crate::timeline::LENGTH_HELP.to_owned())
        });
        let projection = self.plan_projection(book, self.selected, plan, index);
        let known = plan
            .fights
            .iter()
            .map(|fight| fight.name.clone())
            .chain(plan.groups.iter().map(|group| group.name.clone()))
            .collect();
        let coming = projection
            .planned
            .iter()
            .map(|&at| plan.fights[at].name.clone())
            .chain(
                projection
                    .planned_groups
                    .iter()
                    .map(|&at| plan.groups[at].name.clone()),
            )
            .collect();
        Some(TravelView {
            id: open,
            index: at,
            start,
            length,
            help,
            focus_length: std::mem::take(&mut self.travel.focus_length),
            plan: super::plan::JourneyPlan::of(known, coming),
        })
    }

    /// The open journey's checks block: what the DM typed and its last fault.
    pub fn gv_travel_block(&mut self) -> &mut crate::timeline::checks::State {
        &mut self.travel.block
    }

    /// The journey's last checks result, with its Undo.
    pub fn gv_travel_done(&self, id: &str) -> Option<&crate::timeline::checks::Done> {
        self.travel.done.get(id)
    }

    /// The DM's act in the plan's rows or Add strip.
    pub fn gv_plan_act(
        &mut self,
        act: PlanAct,
        book: &mut Book,
        plan: &Plan<'_>,
        index: Option<&crate::lookup::ContentIndex>,
        ctx: &StateContext,
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
                // The row after the new one comes into view.
                self.row_reveal = Some((id.clone(), 0, ctx.pass()));
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
                if session.is_some() && self.add_to_note(&link) {
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
            PlanAct::Travel(id, act) => {
                self.row_reveal = Some((id.clone(), 0, ctx.pass()));
                self.travel_act(book, session, &id, act, plan);
            }
            PlanAct::TravelUndo(id) => self.travel.done.undo(&id, book),
            PlanAct::SceneFold(folded) => self.press_scene_fold(folded),
            PlanAct::FightLog => self.show_fight_log = !self.show_fight_log,
            PlanAct::WholeRun(path) => self.wants_saved_run = Some(path),
            PlanAct::RollForEncounters(id) => {
                self.row_reveal = Some((id.clone(), 0, ctx.pass()));
                self.travel_act(
                    book,
                    session,
                    &id,
                    crate::timeline::checks::Act::Start,
                    plan,
                );
            }
        }
    }
}

/// What a recording row offers.
pub struct RecordingRow {
    pub path: PathBuf,
    pub title: String,
    pub name: String,
    /// `1 s · 0 MB`, or `Audio removed`.
    pub size: String,
    /// Its state; `None` while it is processed (then `processing` says how far).
    pub status: Option<super::RecordingStatus>,
    pub processing: Option<Processing>,
    /// The live recording's lines: its time and what it hears, in the state's colour.
    pub live_line: Option<(String, String)>,
    /// Sentences under the status, muted, with a hover (what the recorder said).
    pub sentences: Vec<(String, Option<String>)>,
    /// The texts it has: Summary, Clean transcript, What the check changed, Raw transcript.
    pub views: Vec<(u8, &'static str)>,
    /// Its menu: `Process again…` (with why it waits), and `Delete…` (with why it waits).
    pub process_again: Option<Option<&'static str>>,
    pub delete_blocked: Option<&'static str>,
    /// Its actions, in order.
    pub actions: Vec<RecordingChoice>,
    /// The reason Process waits, as a link that opens Settings (`true`: to save a speech key).
    pub reason_link: Option<(String, bool)>,
    /// What Process sends where (help under the row).
    pub destination: Option<String>,
    /// `Process again`'s confirmation.
    pub confirm_again: Option<String>,
    /// The events an LLM marked in it.
    pub markers: Vec<MarkerRow>,
}

pub struct Processing {
    pub words: String,
    pub cancelling: bool,
}

pub struct RecordingChoice {
    pub words: &'static str,
    pub kind: plain::ChoiceForm,
    pub blocked: Option<String>,
    pub hover: Option<String>,
    pub act: RecordingAct,
}

pub struct MarkerRow {
    pub index: usize,
    pub icon: &'static str,
    pub title: String,
    /// Its time, a link into the clean transcript (when there is one).
    pub at: Option<String>,
    pub facts: String,
    pub placement: String,
    /// On the timeline: the event's id.
    pub added: Option<String>,
}

/// What the DM did in the Recordings tab.
#[derive(Clone, Debug)]
pub enum RecordingAct {
    ProcessAgain,
    Delete,
    Stop,
    RecordAgain,
    ChooseInput,
    CheckInput,
    Process,
    OpenSettings(bool),
    Read(u8),
    AgainNotNow,
    AgainReplace,
    Cancel,
    AddMarker(usize),
    ReadMarker(usize),
    OpenEvent(String),
}

/// The text of a recording being read (`reading_view`).
pub struct ReadingView {
    pub name: String,
    /// The texts it has, and the one shown.
    pub views: Vec<(u8, &'static str)>,
    pub made: u8,
    /// A summary's `Add to notes`, or `In notes`.
    pub add_to_notes: Option<bool>,
    pub short_of: Option<(u64, u64)>,
    pub draft_note: bool,
    pub part: usize,
    pub times: Vec<String>,
    pub body: String,
}

fn made_of(index: u8) -> super::Made {
    match index {
        0 => super::Made::Summary,
        1 => super::Made::Marked,
        2 => super::Made::Review,
        _ => super::Made::Transcript,
    }
}

fn made_index(made: super::Made) -> u8 {
    match made {
        super::Made::Summary => 0,
        super::Made::Marked => 1,
        super::Made::Review => 2,
        super::Made::Transcript => 3,
    }
}

impl Sessions {
    /// The Recordings tab's empty state: its words, and whether `Open Settings` follows.
    pub fn gv_recordings_empty(&self, settings: &ProcessingSettings) -> Option<(String, bool)> {
        if !self.recordings.is_empty() {
            return None;
        }
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
        Some((words, !here && self.live.is_none() && reason.is_some()))
    }

    pub fn gv_open_settings(&mut self) {
        self.wants_processing_settings = true;
    }

    pub fn gv_reading(&self) -> bool {
        self.reading.is_some()
    }

    /// The Recordings tab's rows.
    pub fn gv_recording_rows(
        &mut self,
        settings: &ProcessingSettings,
        book: &Book,
    ) -> Vec<RecordingRow> {
        use crate::recording::Health;
        use plain::ChoiceForm as Kind;
        if self.processed_notice_read_here() {
            self.end_notice();
        }
        let c = crate::design::colors();
        let banner_primary = self.banner_has_primary(settings, self.run_open);
        let recordings = self.recordings.clone();
        let recovery = self.recording_recovery_row();
        let destination_at = recordings.iter().position(|recording| {
            !recording.summary
                && recording.path.exists()
                && self.job.is_none()
                && settings.problem().is_none()
                && self
                    .live
                    .as_ref()
                    .is_none_or(|live| live.recorder.path() != recording.path)
        });
        let primary_at = recovery.or(destination_at);
        let record_again_at = self.record_again_row();
        let days = crate::timeline::day_numbers(&book.events);
        recordings
            .iter()
            .enumerate()
            .map(|(index, recording)| {
                let primary =
                    primary_at == Some(index) && !self.confirm_record && !banner_primary;
                let record_again_here = record_again_at == Some(index);
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
                    super::spoken_length,
                );
                let size = if live {
                    super::megabytes(recording.bytes)
                } else if !recording.path.exists() {
                    "Audio removed".to_owned()
                } else {
                    format!("{length} · {}", super::megabytes(recording.bytes))
                };
                let job_here = self
                    .job
                    .as_ref()
                    .is_some_and(|(job, _)| job.recording == recording.path);
                let stored = &recording.status;
                let missing_input = settings.input_missing()
                    || stored.as_ref().is_some_and(|words| {
                        words.to_lowercase().contains(super::MISSING_INPUT_ERROR)
                    });
                let one = |icon, word, color| {
                    super::RecordingStatus::one(crate::design::Status { icon, word, color })
                };
                let status = if live {
                    self.recording_status()
                        .unwrap_or_else(super::RecordingStatus::live)
                } else if let Some(words) = stored.as_deref() {
                    let state = if words.starts_with("Failed") {
                        "Failed"
                    } else if words.starts_with("Processing cancelled") {
                        "Cancelled"
                    } else {
                        "Stopped"
                    };
                    if state == "Cancelled" {
                        one(crate::design::icons::WARN, state, c.result.warning)
                    } else {
                        one(crate::design::icons::ERROR, state, c.result.error)
                    }
                } else if recording.warning.is_some() {
                    super::RecordingStatus::one(super::processed_warning())
                } else if recording.summary {
                    super::RecordingStatus::one(super::processed_clean())
                } else if recording.transcript {
                    one(crate::design::icons::DRAFT, "Partly processed", c.muted)
                } else {
                    one(crate::design::icons::DOCUMENT, "Not processed", c.muted)
                };
                let processing = job_here.then(|| {
                    let (job, _) = self.job.as_ref().expect("the job here");
                    Processing {
                        words: format!("Processing · {}", job.progress.words()),
                        cancelling: job.cancelling(),
                    }
                });
                let health = self.live.as_ref().map(|live| live.recorder.health().clone());
                let stalled = live
                    && matches!(health, Some(Health::Stalled | Health::Stopped(_)));
                let silent = live && matches!(health, Some(Health::Silent));
                let mut sentences = Vec::new();
                let mut live_line = None;
                if let Some(rec) = self.live.as_ref().filter(|_| live) {
                    let time = super::clock(rec.recorder.seconds());
                    if silent {
                        sentences.push((
                            format!("{time} recorded · Silent for over a minute"),
                            None,
                        ));
                    } else if !stalled {
                        let sound = if *rec.recorder.health() == Health::Starting {
                            "Waiting for sound"
                        } else if rec.recorder.heard_just_now() {
                            "Hearing sound"
                        } else {
                            "Quiet"
                        };
                        live_line = Some((time, format!(" recorded · {sound}")));
                    } else {
                        sentences.push((
                            format!(
                                "Stopped saving after {}. What was saved is kept. Record again starts a new file.",
                                super::spoken_length(rec.recorder.seconds())
                            ),
                            None,
                        ));
                    }
                }
                if let Some(words) = stored.as_ref().filter(|_| !live && !job_here) {
                    let hover = words
                        .strip_prefix("Stopped by itself:")
                        .filter(|said| super::plain_recorder_words(said).is_some())
                        .map(super::recorder_said);
                    sentences.push((super::recording_detail(words, missing_input), hover));
                }
                if let Some(warning) = recording.warning.as_ref().filter(|_| !live && !job_here) {
                    sentences.push((warning.clone(), None));
                }
                let views = [
                    (super::Made::Summary, recording.summary),
                    (super::Made::Marked, recording.marked),
                    (
                        super::Made::Review,
                        recording.review && !recording.check_missing(),
                    ),
                    (super::Made::Transcript, recording.transcript),
                ]
                .into_iter()
                .filter_map(|(made, exists)| exists.then_some((made_index(made), made.word())))
                .collect();
                let process_again = recording.summary.then_some(if !recording.path.exists() {
                    Some("Audio removed. Restore the WAV file to process it.")
                } else if live {
                    Some("Stop recording first.")
                } else if self.job.is_some() {
                    Some("Wait for processing to finish.")
                } else {
                    None
                });
                let busy = live || job_here;
                let stopped_by_itself = stored
                    .as_ref()
                    .is_some_and(|words| words.starts_with("Stopped by itself:"));
                let mut actions = Vec::new();
                let mut reason_link = None;
                if !live && !job_here && stopped_by_itself {
                    if missing_input && record_again_here {
                        actions.push(RecordingChoice {
                            words: "Choose an input",
                            kind: if primary && !self.run_open {
                                Kind::Primary
                            } else {
                                Kind::Choice
                            },
                            blocked: None,
                            hover: None,
                            act: RecordingAct::ChooseInput,
                        });
                    }
                    if record_again_here {
                        let busy = self.live.is_some() || self.job.is_some();
                        actions.push(RecordingChoice {
                            words: "Record again",
                            kind: if primary && !self.run_open && !missing_input && !busy {
                                Kind::Primary
                            } else {
                                Kind::Choice
                            },
                            blocked: busy.then(|| {
                                "Stop recording or wait for processing to finish.".to_owned()
                            }),
                            hover: None,
                            act: RecordingAct::RecordAgain,
                        });
                    }
                }
                if live {
                    actions.push(RecordingChoice {
                        words: "Stop recording",
                        kind: Kind::Choice,
                        blocked: None,
                        hover: None,
                        act: RecordingAct::Stop,
                    });
                    if stalled && record_again_here {
                        actions.push(RecordingChoice {
                            words: "Record again",
                            kind: if self.run_open || !primary {
                                Kind::Choice
                            } else {
                                Kind::Primary
                            },
                            blocked: None,
                            hover: None,
                            act: RecordingAct::RecordAgain,
                        });
                    }
                    if silent {
                        actions.push(RecordingChoice {
                            words: "Check the input in Settings",
                            kind: Kind::Link,
                            blocked: None,
                            hover: None,
                            act: RecordingAct::CheckInput,
                        });
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
                    let label = if recording.transcript
                        || recording.status.as_ref().is_some_and(|words| {
                            words.starts_with("Failed") || words.starts_with("Processing cancelled")
                        }) {
                        "Continue processing"
                    } else {
                        "Process"
                    };
                    let quiet = self.run_open
                        || !primary
                        || stalled
                        || stopped_by_itself
                        || reason.is_some();
                    actions.push(RecordingChoice {
                        words: label,
                        kind: if quiet { Kind::Choice } else { Kind::Primary },
                        blocked: reason.clone(),
                        hover: Some(super::recording_destination(settings, recording)),
                        act: RecordingAct::Process,
                    });
                    if let Some(reason) = reason.as_ref().filter(|_| !live && self.job.is_none()) {
                        let speech = crate::processing::host(&settings.config.speech_url);
                        let key = reason.starts_with("Save an API key for this speech service");
                        let named = if key {
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
                        reason_link = Some((named, key));
                    }
                }
                let destination = (destination_at == Some(index))
                    .then(|| super::recording_destination(settings, recording));
                let confirm_again = (self.confirm_again.as_deref() == Some(recording.path.as_path()))
                    .then(|| {
                        format!(
                            "This sends the raw transcript, this session’s name, notes and events to {} twice. The audio is not sent again. It replaces the summary, the clean transcript and what the check changed. Events you added stay on the timeline.",
                            super::processing_llm(settings)
                        )
                    });
                let markers = recording
                    .markers
                    .iter()
                    .enumerate()
                    .map(|(index, marker)| {
                        let title = if marker.title.is_empty() {
                            marker.kind.word().to_owned()
                        } else {
                            marker.title.clone()
                        };
                        let key = marker.key(&recording.name);
                        let added = book.event_of_marker(&key).map(|event| event.id.clone());
                        let mut facts = vec![marker.kind.word()];
                        for fact in [&marker.duration, &marker.place] {
                            if !fact.is_empty() {
                                facts.push(fact);
                            }
                        }
                        let placement = added.as_ref().map_or_else(String::new, |id| {
                            let at = book.events.iter().position(|event| event.id == *id);
                            let before = at.and_then(|at| at.checked_sub(1));
                            before.map_or_else(
                                || "First on the timeline".to_owned(),
                                |before| {
                                    format!("After {}", book.events[before].label(days[before]))
                                },
                            )
                        });
                        MarkerRow {
                            index,
                            icon: crate::timeline::kind_icon(marker.kind),
                            title,
                            at: (!marker.at.is_empty() && recording.marked)
                                .then(|| marker.at.clone()),
                            facts: facts.join(" · "),
                            placement,
                            added,
                        }
                    })
                    .collect();
                RecordingRow {
                    path: recording.path.clone(),
                    title: recording.title.clone(),
                    name: recording.name.clone(),
                    size,
                    status: (!job_here).then_some(status),
                    processing,
                    live_line,
                    sentences,
                    views,
                    process_again,
                    delete_blocked: busy.then_some("This recording is in use."),
                    actions,
                    reason_link,
                    destination,
                    confirm_again,
                    markers,
                }
            })
            .collect()
    }

    /// A press in a recording's row.
    pub fn gv_recording_act(
        &mut self,
        path: &std::path::Path,
        act: RecordingAct,
        settings: &ProcessingSettings,
        book: &mut Book,
        ctx: &StateContext,
    ) {
        let Some(recording) = self
            .recordings
            .iter()
            .find(|recording| recording.path == path)
            .cloned()
        else {
            return;
        };
        let session = self.selected;
        match act {
            RecordingAct::ProcessAgain => self.confirm_again = Some(recording.path.clone()),
            RecordingAct::Delete => self.confirm_delete = Some(recording.path.clone()),
            RecordingAct::Stop => self.finish_recording(ctx),
            RecordingAct::RecordAgain => {
                if let Some(session) = session {
                    let stalled = self.live.as_ref().is_some_and(|live| {
                        live.recorder.path() == recording.path
                            && matches!(
                                live.recorder.health(),
                                crate::recording::Health::Stalled
                                    | crate::recording::Health::Stopped(_)
                            )
                    });
                    if stalled {
                        self.finish_recording(ctx);
                    }
                    if let Err(error) = self.start_recording(settings, session) {
                        self.say(crate::design::Tone::Error, error, None);
                    }
                }
            }
            RecordingAct::ChooseInput => self.wants_recording_input = true,
            RecordingAct::CheckInput => self.wants_processing_settings = true,
            RecordingAct::Process | RecordingAct::AgainReplace => {
                self.confirm_again = None;
                self.start_processing(ctx, &recording.path, settings, book);
            }
            RecordingAct::OpenSettings(key) => {
                self.wants_processing_settings = true;
                self.wants_speech_key = key;
            }
            RecordingAct::Read(made) => self.open_reading(&recording, made_of(made), None),
            RecordingAct::AgainNotNow => self.confirm_again = None,
            RecordingAct::Cancel => {
                if let Some((job, _)) = &self.job {
                    job.cancel();
                }
            }
            RecordingAct::AddMarker(index) => {
                if book.unreadable.is_some() {
                    return;
                }
                if let Some(marker) = recording.markers.get(index) {
                    let key = marker.key(&recording.name);
                    let id = book.add_marker(marker, &key, session);
                    book.save();
                    let title = if marker.title.is_empty() {
                        marker.kind.word()
                    } else {
                        marker.title.as_str()
                    };
                    self.markers.record(super::kept::MarkerAdd {
                        owner: super::kept::OfSession {
                            root: self.folder.clone().unwrap_or_default(),
                            session,
                        },
                        id,
                        title: title.to_owned(),
                    });
                }
            }
            RecordingAct::ReadMarker(index) => {
                if let Some(marker) = recording.markers.get(index) {
                    self.open_reading(&recording, super::Made::Marked, Some(&marker.at.clone()));
                }
            }
            RecordingAct::OpenEvent(id) => self.wants_event = Some(id),
        }
    }

    /// The recording text being read (`reading_view`).
    pub fn gv_reading_view(&mut self) -> Option<ReadingView> {
        let in_notes = self
            .draft
            .as_ref()
            .map_or(String::new(), |draft| draft.body.as_str().to_owned());
        let recordings = self.recordings.clone();
        let reading = self.reading.as_mut()?;
        let path = reading.recording.clone();
        let views = [
            super::Made::Summary,
            super::Made::Marked,
            super::Made::Review,
            super::Made::Transcript,
        ]
        .into_iter()
        .filter(|made| {
            made.path(&path).exists()
                && (*made != super::Made::Review
                    || recordings
                        .iter()
                        .find(|recording| recording.path == path)
                        .is_none_or(|recording| !recording.check_missing()))
        })
        .map(|made| (made_index(made), made.word()))
        .collect();
        if reading
            .shown
            .as_ref()
            .is_none_or(|shown| shown.part != reading.part)
        {
            reading.shown = Some(super::ShownPart::of(reading));
        }
        let add_to_notes = (reading.made == super::Made::Summary).then(|| {
            let summary = crate::sessions::nested_in_note(reading.parts[0].1.trim());
            in_notes.contains(summary.trim())
        });
        Some(ReadingView {
            name: reading.name.clone(),
            views,
            made: made_index(reading.made),
            add_to_notes,
            short_of: reading.short_of,
            draft_note: matches!(
                reading.made,
                super::Made::Summary | super::Made::Marked | super::Made::Review
            ),
            part: reading.part,
            times: reading.parts.iter().map(|(time, _)| time.clone()).collect(),
            body: reading
                .shown
                .as_ref()
                .map(|shown| shown.body.clone())
                .unwrap_or_default(),
        })
    }

    /// `Back to recordings`.
    pub fn gv_close_reading(&mut self) {
        self.reading = None;
    }

    /// Another text of the same recording.
    pub fn gv_read_made(&mut self, made: u8) {
        let Some(path) = self
            .reading
            .as_ref()
            .map(|reading| reading.recording.clone())
        else {
            return;
        };
        if let Some(recording) = self
            .recordings
            .iter()
            .find(|recording| recording.path == path)
            .cloned()
        {
            self.open_reading(&recording, made_of(made), None);
        }
    }

    /// Earlier, Later, or a time.
    pub fn gv_read_part(&mut self, part: usize) {
        if let Some(reading) = &mut self.reading
            && part < reading.parts.len()
        {
            reading.part = part;
        }
    }

    /// A summary's `Add to notes`.
    pub fn gv_summary_to_notes(&mut self) {
        let Some(summary) = self
            .reading
            .as_ref()
            .filter(|reading| reading.made == super::Made::Summary)
            .map(|reading| crate::sessions::nested_in_note(reading.parts[0].1.trim()))
        else {
            return;
        };
        let _ = self.add_to_note(&summary);
    }

    /// The delete dialog's recording, while it is open.
    pub fn gv_confirm_delete(&self) -> Option<String> {
        self.confirm_delete
            .as_deref()
            .map(crate::sessions::recording_title)
    }

    /// `Keep the recording`.
    pub fn gv_keep_recording(&mut self) {
        self.confirm_delete = None;
    }

    /// `Delete recording` (`delete_dialog`).
    pub fn gv_delete_recording(&mut self) {
        let Some(path) = self.confirm_delete.take() else {
            return;
        };
        self.reading = None;
        if self.processed_recording.as_deref() == Some(path.as_path()) {
            self.processed_session = None;
            self.processed_recording = None;
            self.job_done = false;
            self.job_warned = false;
        }
        let (tone, words) = match crate::sessions::delete_recording(&path) {
            Ok(()) => (
                crate::design::Tone::Success,
                "Recording deleted.".to_owned(),
            ),
            Err(error) => (crate::design::Tone::Error, error),
        };
        self.say(tone, words, None);
        self.refresh_recordings();
    }
}

/// One row of the templates list.
pub struct TemplateRow {
    pub name: String,
    /// `Campaign` or `Built-in`.
    pub kind: &'static str,
    pub selected: bool,
}

/// The shown template, as its editor draws it.
pub struct TemplateView {
    pub name: String,
    /// A campaign template: its file, its draft's words and state, and its menu's word
    /// (`Delete`, or `Go back to built-in`).
    pub file: Option<(PathBuf, String, DraftState, &'static str)>,
    /// A built-in template: its words, read-only, with `Customize`.
    pub built_in: Option<String>,
}

impl Sessions {
    pub fn gv_template_rows(&self) -> Vec<TemplateRow> {
        self.templates
            .iter()
            .map(|template| TemplateRow {
                name: template.name.clone(),
                kind: if template.path.is_some() {
                    "Campaign"
                } else {
                    "Built-in"
                },
                selected: self.new_template.is_none()
                    && self.template.as_deref() == Some(template.name.as_str()),
            })
            .collect()
    }

    pub fn gv_templates_list_only(&self) -> bool {
        self.templates_list_only
    }

    pub fn gv_set_templates_list_only(&mut self, only: bool) {
        self.templates_list_only = only;
    }

    /// A press on a template's row.
    pub fn gv_choose_template(&mut self, name: &str) {
        self.templates_list_only = false;
        self.new_template = None;
        self.select_template(name);
    }

    /// The shown template, choosing the first when none is.
    pub fn gv_template(&mut self) -> Option<TemplateView> {
        if self.template.is_none()
            && let Some(first) = self.templates.first().map(|t| t.name.clone())
        {
            self.select_template(&first);
        }
        let index = self
            .template
            .as_ref()
            .and_then(|name| self.templates.iter().position(|t| &t.name == name))?;
        let template = &self.templates[index];
        let built_in = crate::sessions::BUILT_IN_TEMPLATES
            .iter()
            .any(|(other, _)| *other == template.name);
        Some(TemplateView {
            name: template.name.clone(),
            file: template
                .path
                .clone()
                .zip(self.template_draft.as_ref())
                .map(|(path, draft)| {
                    (
                        path,
                        draft.body.as_str().to_owned(),
                        draft.state.clone(),
                        if built_in {
                            "Go back to built-in"
                        } else {
                            "Delete"
                        },
                    )
                }),
            built_in: template.path.is_none().then(|| template.body.clone()),
        })
    }

    /// The template's state, as its title line says it (`draft_status`).
    pub fn gv_template_status(state: &DraftState) -> crate::design::Status<'static> {
        Self::draft_status(state)
    }

    /// The DM's edit of the template's words.
    pub fn gv_template_edited(&mut self, words: &str) {
        if let Some(draft) = &mut self.template_draft
            && !draft.body.same_words(words)
        {
            let _ = draft.body.replace(words.to_owned());
            draft.edited();
            self.deleted_templates.owner_changed(&super::kept::Page {
                root: self.folder.clone().unwrap_or_default(),
            });
        }
    }

    /// Ctrl+S in the template.
    pub fn gv_save_template(&mut self) {
        if !self.modal
            && let Some(draft) = &mut self.template_draft
        {
            draft.save();
        }
    }

    /// `Customize`, `Delete` or `Go back to built-in` of the shown template.
    pub fn gv_customize_template(&mut self) {
        if let Some(index) = self
            .template
            .as_ref()
            .and_then(|name| self.templates.iter().position(|t| &t.name == name))
        {
            self.customize_template(index);
        }
    }

    pub fn gv_delete_template(&mut self) {
        if let Some(index) = self
            .template
            .as_ref()
            .and_then(|name| self.templates.iter().position(|t| &t.name == name))
        {
            self.delete_template(index);
        }
    }

    /// The New template form's name, while it is open.
    pub fn gv_new_template_name(&self) -> Option<&str> {
        self.new_template.as_deref()
    }

    pub fn gv_set_new_template_name(&mut self, name: &str) {
        if let Some(words) = &mut self.new_template {
            name.clone_into(words);
        }
    }

    /// The New template form's Create.
    pub fn gv_create_template(&mut self) {
        let Some(name) = self.new_template.clone() else {
            return;
        };
        if !name.trim().is_empty() && self.create_new_template(&name) {
            self.new_template = None;
        }
    }

    pub fn gv_cancel_new_template(&mut self) {
        self.new_template = None;
        self.template_error = None;
    }

    /// The templates list's footer (`list_messages`): a deleted template's Undo.
    pub fn gv_template_list_footer(&self) -> Vec<FooterLine> {
        let frame = PaneFrame::new();
        let messages = self.list_messages(&frame);
        messages.into_plain()
    }
}

impl Sessions {
    /// `Save` of a rolled NPC: its World entry, and the link the note takes; or why not.
    pub fn gv_save_npc(
        &mut self,
        generated: &crate::npc_generator::Generated,
        index: Option<&crate::lookup::ContentIndex>,
    ) -> Result<String, String> {
        let campaign = self.campaign.as_ref().ok_or("Open a campaign first.")?;
        crate::world::create_generated(campaign, generated)?;
        self.wants_content_refresh = true;
        Ok(crate::world::new_entry_link(index, &generated.name, "npc"))
    }
}

pub use crate::sessions::slash_plain as slash;

#[cfg(test)]
mod tests {
    use super::*;

    /// Round 30 review F4 of PR 400: every warning of the session note that the banner shows on
    /// any page names its note, with its cause: the conflict, the missing note, the discarded
    /// and missing note, and the failed save. A template's warning (no name) keeps `This note`.
    /// The banner's data (`draft_warning`).
    #[test]
    fn a_session_note_warning_names_its_note_in_every_state() {
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("Session 1 - Ash.md");
        std::fs::write(&path, "---\nsession_id: 1\n---\n# Ash\n").unwrap();
        const NAME: &str = "Session 1: Ash on the Road";
        let shows = |state: DraftState, discarded: bool, name: Option<&str>, words: &str| {
            let mut draft = super::super::Draft::open(&path).unwrap();
            draft.state = state;
            if discarded {
                draft.discard();
                draft.state = DraftState::Missing;
            }
            let said: Vec<String> = draft_warning(&draft, name, name.is_none())
                .into_iter()
                .map(|message| message.words)
                .collect();
            let named = said.iter().any(|said| said.contains(words));
            let unnamed = said
                .iter()
                .any(|said| said.contains("This note ") || said.contains("This file "));
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
}
