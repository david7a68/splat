//! Settings for recording and processing: which input, which speech service, which LLM.
//! The choices live in a small file beside the settings file. Keys live in the keyring.
use super::{
    ANTHROPIC_MODEL, ANTHROPIC_URL, CODEX_MODEL, Config, OPENAI_SPEECH_MODEL, OPENAI_URL, Provider,
};
use crate::{
    credentials::{self, KeySlot},
    design::outcome::{
        Fault, FaultMessage, Messages, NoPress, Notice, PaneFrame, Sentence, Severity,
    },
    design::pane::Body,
    design::{self, Action, Density, Type, colors, icons},
    recording::{self, Probe},
};
use eframe::egui;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::mpsc,
};
use zeroize::{Zeroize, Zeroizing};

/// A fault of the recording settings, the recorder or a key, by kind. The kind decides its
/// severity, so each one has one tone wherever it shows.
enum SettingsFault<'a> {
    /// The settings file cannot be read, so the defaults are in use: the reader's words.
    Unreadable(&'a str),
    /// A change was not saved: the system's words.
    NotSaved(&'a str),
    /// No recorder is installed on this computer.
    NoRecorder,
    /// The keyring did not keep or remove a key: its words. Splat holds the key until it closes.
    KeyNotKept(&'a str),
    /// The saved key is for another service, so Splat does not send it.
    KeyForAnother { owner: &'a str, target: &'a str },
}

impl Fault for SettingsFault<'_> {
    fn severity(&self) -> Severity {
        match self {
            Self::Unreadable(_) => Severity::CannotRead,
            Self::NotSaved(_) | Self::KeyNotKept(_) => Severity::NotSaved,
            Self::NoRecorder | Self::KeyForAnother { .. } => Severity::NeedsFixing,
        }
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "the faults carry the settings reader’s, the system’s and the keyring’s words"
    )]
    fn cause(&self) -> Sentence {
        use crate::sentence;
        match self {
            Self::Unreadable(error) => Sentence::outside(*error).unwrap_or(sentence!(
                "The settings file could not be read, so these are the defaults."
            )),
            Self::NotSaved(error) => {
                let said = sentence!("These settings were not saved.");
                match Sentence::outside(*error) {
                    Some(error) => said.then(error),
                    None => said,
                }
            }
            Self::NoRecorder => sentence!(
                "No recorder found. Install pw-record (PipeWire), parecord (PulseAudio) or arecord (ALSA). Notes and the timeline work without one."
            ),
            Self::KeyNotKept(error) => Sentence::outside(*error)
                .unwrap_or(sentence!("The key was not saved on this computer."))
                .then(sentence!("Splat keeps the key until it closes.")),
            Self::KeyForAnother { owner, target } => sentence!(
                "The saved key is for {}, so Splat will not send it to {}.",
                owner,
                target
            ),
        }
    }
}

/// Which text setting a [`ProcessingSettings::setting_field`] edits.
#[derive(Clone, Copy)]
enum Text {
    SpeechUrl,
    SpeechModel,
    LlmUrl,
    LlmModel,
}

/// Common services, to fill the address. The DM can still enter any other.
const LLM_CHOICES: [(&str, Provider, &str, &str); 6] = [
    (
        "Codex CLI, GPT-6.1 Sol",
        Provider::CodexCli,
        "",
        CODEX_MODEL,
    ),
    (
        "Anthropic",
        Provider::Anthropic,
        ANTHROPIC_URL,
        ANTHROPIC_MODEL,
    ),
    ("OpenAI", Provider::OpenAiCompatible, OPENAI_URL, ""),
    (
        "Google Gemini",
        Provider::OpenAiCompatible,
        "https://generativelanguage.googleapis.com/v1beta/openai",
        "",
    ),
    (
        "OpenRouter",
        Provider::OpenAiCompatible,
        "https://openrouter.ai/api/v1",
        "",
    ),
    (
        "Ollama on this computer",
        Provider::OpenAiCompatible,
        "http://localhost:11434/v1",
        "",
    ),
];
const SPEECH_CHOICES: [(&str, &str, &str); 3] = [
    ("OpenAI", OPENAI_URL, OPENAI_SPEECH_MODEL),
    ("Groq", "https://api.groq.com/openai/v1", ""),
    (
        "whisper.cpp on this computer",
        "http://localhost:8080/v1",
        "",
    ),
];
const OTHER_SERVICE: &str = "Another service";

const SPEECH_KEY: design::FieldId = crate::field_id!("speech_api_key").once(design::Once::Settings);
const LLM_KEY: design::FieldId = crate::field_id!("llm_api_key").once(design::Once::Settings);
const CLEAN_PROMPT: design::FieldId = crate::field_id!("clean_prompt").once(design::Once::Settings);
const REVIEW_PROMPT: design::FieldId =
    crate::field_id!("review_prompt").once(design::Once::Settings);
const INPUT: design::FieldId = crate::field_id!("recording_input").once(design::Once::Settings);
const SPEECH_SERVICE: design::FieldId =
    crate::field_id!("speech_service").once(design::Once::Settings);
const SPEECH_URL: design::FieldId = crate::field_id!("speech_url").once(design::Once::Settings);
const SPEECH_MODEL: design::FieldId = crate::field_id!("speech_model").once(design::Once::Settings);
const LLM_SERVICE: design::FieldId = crate::field_id!("llm_service").once(design::Once::Settings);
const LLM_URL: design::FieldId = crate::field_id!("llm_url").once(design::Once::Settings);
const LLM_MODEL: design::FieldId = crate::field_id!("llm_model").once(design::Once::Settings);

#[derive(Default)]
struct Store {
    path: Option<PathBuf>,
}

impl Store {
    #[cfg(not(test))]
    fn user() -> Self {
        Self {
            path: directories::ProjectDirs::from("", "", "splat")
                .map(|dirs| dirs.config_dir().join("session-processing.json")),
        }
    }

    #[cfg(test)]
    fn user() -> Self {
        Self::default()
    }

    /// A bad file is reported, and left in place until the DM changes something.
    fn load(&self) -> Result<Config, String> {
        let Some(path) = &self.path else {
            return Ok(Config::default());
        };
        match fs::read(path) {
            Ok(bytes) => serde_json::from_slice(&bytes).map_err(|_| {
                format!(
                    "{} could not be read, so these are the defaults.",
                    path.display()
                )
            }),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn save(&self, config: &Config) -> Result<(), String> {
        let Some(path) = &self.path else {
            return Ok(());
        };
        write(path, config)
    }
}

fn write(path: &Path, config: &Config) -> Result<(), String> {
    let parent = path.parent().ok_or("No parent folder")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let bytes = serde_json::to_vec_pretty(config).map_err(|e| e.to_string())?;
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    temp.write_all(&bytes).map_err(|e| e.to_string())?;
    temp.persist(path).map_err(|e| e.to_string())?;
    Ok(())
}

/// A key the DM is typing, and whether one is saved.
#[derive(Default)]
struct KeyField {
    typing: String,
    saved: bool,
    error: Option<String>,
}

impl Drop for KeyField {
    fn drop(&mut self) {
        self.typing.zeroize();
    }
}

enum Reply {
    Saved(KeySlot, Result<(), String>),
    Forgot(KeySlot, Result<(), String>),
    Found(KeySlot, bool),
}

pub struct ProcessingSettings {
    store: Store,
    pub config: Config,
    /// A recording is live, so a new input cannot apply to it.
    pub recording_now: bool,
    pub focus_input: bool,
    pub focus_speech_key: bool,
    started: bool,
    load_error: Option<String>,
    save_error: Option<String>,
    probe: Option<Probe>,
    probing: Option<mpsc::Receiver<Probe>>,
    probed: Option<std::time::Instant>,
    codex_found: bool,
    speech_key: KeyField,
    llm_key: KeyField,
    replies: Option<(mpsc::Sender<Reply>, mpsc::Receiver<Reply>)>,
    /// A failed save to the keyring keeps the key for this run of the app only.
    held_speech_key: Option<Zeroizing<String>>,
    held_llm_key: Option<Zeroizing<String>>,
    /// The DM chose a service last frame: bring the notice and an empty field that blocks
    /// processing into view once. It ends on the next frame, whether or not it scrolls.
    reveal_problem: bool,
    /// Where this frame drew the first field reason that blocks processing.
    reason_rect: Option<egui::Rect>,
}

impl Default for ProcessingSettings {
    fn default() -> Self {
        Self {
            store: Store::user(),
            config: Config::default(),
            recording_now: false,
            focus_input: false,
            focus_speech_key: false,
            started: false,
            load_error: None,
            save_error: None,
            probe: None,
            probing: None,
            probed: None,
            codex_found: false,
            speech_key: KeyField::default(),
            llm_key: KeyField::default(),
            replies: None,
            held_speech_key: None,
            held_llm_key: None,
            reveal_problem: false,
            reason_rect: None,
        }
    }
}

impl ProcessingSettings {
    fn start(&mut self, ctx: &egui::Context) {
        if self.started {
            return;
        }
        self.started = true;
        match self.store.load() {
            Ok(config) => self.config = config,
            Err(error) => self.load_error = Some(error),
        }
        self.probing = Some(recording::probe());
        // Test builds behave the same on every computer.
        self.codex_found = cfg!(test) || recording::on_path("codex");
        let (tx, rx) = mpsc::channel();
        for slot in [KeySlot::Speech, KeySlot::Llm] {
            let (tx, ctx) = (tx.clone(), ctx.clone());
            credentials::in_background(move || {
                let found = credentials::load_key(slot).is_ok_and(|key| key.is_some());
                let _ = tx.send(Reply::Found(slot, found));
                ctx.request_repaint();
            });
        }
        self.replies = Some((tx, rx));
    }

    /// Look again for recorders and inputs. A microphone can be plugged in at any time.
    pub fn look_again(&mut self) {
        if self.probing.is_none() {
            self.probing = Some(recording::probe());
            self.probed = Some(std::time::Instant::now());
        }
    }

    pub fn poll(&mut self, ctx: &egui::Context) {
        self.start(ctx);
        // The table is waiting while a chosen input is missing, so keep looking.
        if self.input_missing()
            && self
                .probed
                .is_none_or(|at| at.elapsed() >= std::time::Duration::from_secs(5))
        {
            self.look_again();
        }
        if let Some(found) = self.probing.as_ref().and_then(|rx| rx.try_recv().ok()) {
            self.probe = Some(found);
            self.probing = None;
        } else if self.probing.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
        let Some(replies) = self.replies.take() else {
            return;
        };
        while let Ok(reply) = replies.1.try_recv() {
            match reply {
                Reply::Found(slot, found) => self.field(slot).saved = found,
                Reply::Saved(slot, result) => {
                    let field = self.field(slot);
                    field.saved = result.is_ok();
                    field.error = result.err();
                }
                Reply::Forgot(slot, result) => {
                    let field = self.field(slot);
                    field.saved = result.is_err();
                    field.error = result.err();
                }
            }
        }
        self.replies = Some(replies);
    }

    fn field(&mut self, slot: KeySlot) -> &mut KeyField {
        match slot {
            KeySlot::Speech => &mut self.speech_key,
            KeySlot::Llm => &mut self.llm_key,
        }
    }

    fn held(&mut self, slot: KeySlot) -> &mut Option<Zeroizing<String>> {
        match slot {
            KeySlot::Speech => &mut self.held_speech_key,
            KeySlot::Llm => &mut self.held_llm_key,
        }
    }

    pub fn tool(&self) -> Option<recording::Tool> {
        self.probe.as_ref().and_then(|probe| probe.tool)
    }

    /// Still looking for a recorder, so the app cannot yet say there is none.
    pub fn probing(&self) -> bool {
        self.probing.is_some()
    }

    /// The input to record from: the DM's choice, if the recorder can use one.
    pub fn input(&self) -> Option<&str> {
        self.config
            .input
            .as_deref()
            .filter(|_| self.tool().is_some_and(recording::Tool::can_choose_input))
    }

    /// The chosen input is gone. Recording stops here, not on another microphone.
    pub fn input_missing(&self) -> bool {
        let chosen = self.input();
        let inputs = self.probe.as_ref().map_or(&[][..], |probe| &probe.inputs);
        chosen.is_some_and(|name| !inputs.iter().any(|input| input.name == name))
    }

    #[cfg(test)]
    pub fn simulate_input(&mut self, name: &str) {
        self.config.input = Some(name.to_owned());
    }

    /// A key for the service chosen now. A key saved for another service does not count,
    /// because it must never be sent to a different company.
    pub fn has_key(&self, slot: KeySlot) -> bool {
        match slot {
            KeySlot::Speech => {
                (self.speech_key.saved || self.held_speech_key.is_some())
                    && self.config.speech_key_fits()
            }
            KeySlot::Llm => {
                (self.llm_key.saved || self.held_llm_key.is_some()) && self.config.llm_key_fits()
            }
        }
    }

    /// The first thing that stops Process. A service on this computer needs no key.
    pub fn problem(&self) -> Option<String> {
        self.config.problem().or_else(|| {
            let codex = self.config.provider == Provider::CodexCli;
            if !self.has_key(KeySlot::Speech) && !super::is_local(&self.config.speech_url) {
                Some("Save an API key for this speech service in Settings.".into())
            } else if codex && !self.codex_found {
                Some("The codex command was not found. Install Codex, or choose another transcript and summary service in Settings.".into())
            } else if !codex && !self.has_key(KeySlot::Llm) && !super::is_local(&self.config.llm_url) {
                Some("Save an API key for the transcript and summary service in Settings.".into())
            } else {
                None
            }
        })
    }

    /// A copy of any key held only for this run. Keys in the keyring load on the worker.
    pub fn held_keys(&self) -> (Option<Zeroizing<String>>, Option<Zeroizing<String>>) {
        (
            self.held_speech_key
                .clone()
                .filter(|_| self.config.speech_key_fits()),
            self.held_llm_key
                .clone()
                .filter(|_| self.config.llm_key_fits()),
        )
    }

    #[cfg(test)]
    pub fn simulate_keys(&mut self) {
        self.speech_key.saved = true;
        self.llm_key.saved = true;
        self.config.speech_key_for = self.config.speech_url.clone();
        self.config.llm_key_for = self.config.llm_url.clone();
    }

    fn persist(&mut self) {
        self.save_error = self
            .store
            .save(&self.config)
            .err()
            .map(|error| error.to_string());
    }

    fn key_row(&mut self, ui: &mut egui::Ui, slot: KeySlot, label: &str) {
        let (tx, ctx) = (
            self.replies.as_ref().map(|(tx, _)| tx.clone()),
            ui.ctx().clone(),
        );
        let saved = self.field(slot).saved;
        let fits = match slot {
            KeySlot::Speech => self.config.speech_key_fits(),
            KeySlot::Llm => self.config.llm_key_fits(),
        };
        let saved_for = match slot {
            KeySlot::Speech => &self.config.speech_key_for,
            KeySlot::Llm => &self.config.llm_key_for,
        };
        let owner = key_service_name(saved_for).to_owned();
        let target = key_service_name(match slot {
            KeySlot::Speech => &self.config.speech_url,
            KeySlot::Llm => &self.config.llm_url,
        })
        .to_owned();
        let slot_word = if slot == KeySlot::Speech {
            "speech"
        } else {
            "transcript and summary"
        };
        let remove = format!("Remove {slot_word} key ({owner})");
        let remove_name = format!("{remove}: {label}");
        let held = self.held(slot).is_some();
        let focus = slot == KeySlot::Speech && std::mem::take(&mut self.focus_speech_key);
        let key_field = match slot {
            KeySlot::Speech => SPEECH_KEY,
            KeySlot::Llm => LLM_KEY,
        };
        let mut save = false;
        // The field and `Save key` share the form's width, so `Save key` ends at the edge the
        // other fields end at. Its slot stays when it is hidden, so typing moves nothing. The
        // line below holds the reason, else the saved status, so neither moves the form.
        design::small_field(ui, "API key", None, None, |ui| {
            let mut reason = None;
            ui.horizontal(|ui| {
                let row = ui.available_width().min(design::FORM_WIDTH);
                let field = self.field(slot);
                let empty = field.typing.trim().is_empty();
                let show_save = !empty || (!saved && !held) || !fits;
                reason = (empty && show_save).then_some("Paste a key first.");
                let save_width = Action::secondary("Save key").natural_width(ui);
                let width = (row - save_width - ui.spacing().item_spacing.x).max(design::L * 4.0);
                let hint = design::placeholder(
                    ui,
                    if saved && fits {
                        "Paste a new key to replace it"
                    } else {
                        "Paste the key"
                    },
                );
                let response = key_field.single_line(ui, &mut field.typing, |edit| {
                    edit.password(true)
                        .desired_width(width - 2.0 * design::S)
                        .min_size(egui::vec2(width, 0.0))
                        .margin(egui::vec2(design::S, 5.0))
                        .hint_text(hint)
                });
                design::named(ui, &response, label);
                if focus {
                    response.request_focus();
                    response.scroll_to_me(None);
                }
                if show_save {
                    save = Action::secondary("Save key")
                        .name(&format!("Save key: {label}"))
                        .blocked_if(empty, "Paste a key first.")
                        .show(ui)
                        .clicked();
                }
            });
            // Where a field's help goes. A key error replaces the line with its message.
            if self.field(slot).error.is_none() {
                if let Some(reason) = reason {
                    design::blocked_reason(ui, Some(reason));
                } else if saved && fits {
                    ui.label(
                        Type::Small
                            .rich(ui, format!("{} Saved on this computer.", icons::OK))
                            .color(colors().document.saved),
                    );
                } else {
                    // The line stays, so the reason coming and going moves nothing.
                    let font = Type::Small.font(ui);
                    let height = ui.fonts_mut(|fonts| fonts.row_height(&font));
                    ui.allocate_exact_size(egui::vec2(0.0, height), egui::Sense::hover());
                }
            }
        });
        if (saved || held) && !fits {
            key_fault(
                ui,
                &SettingsFault::KeyForAnother {
                    owner: &owner,
                    target: &target,
                },
            );
        }
        if save {
            match slot {
                KeySlot::Speech => self.config.speech_key_for = self.config.speech_url.clone(),
                KeySlot::Llm => self.config.llm_key_for = self.config.llm_url.clone(),
            }
            self.persist();
            let key = Zeroizing::new(self.field(slot).typing.trim().to_owned());
            self.field(slot).typing.zeroize();
            self.field(slot).typing.clear();
            // Usable at once, even if the keyring is locked.
            *self.held(slot) = Some(key.clone());
            if let Some(tx) = tx.clone() {
                credentials::in_background(move || {
                    let result = credentials::save_key(slot, &key);
                    let _ = tx.send(Reply::Saved(slot, result));
                    ctx.request_repaint();
                });
            }
        }
        if let Some(error) = self.field(slot).error.clone() {
            key_fault(ui, &SettingsFault::KeyNotKept(&error));
        }
        if (saved || held)
            && Action::quiet(&remove)
                .line_start()
                .name(&remove_name)
                .show(ui)
                .clicked()
        {
            *self.held(slot) = None;
            if let Some(tx) = self.replies.as_ref().map(|(tx, _)| tx.clone()) {
                let ctx = ui.ctx().clone();
                credentials::in_background(move || {
                    let result = credentials::forget_key(slot);
                    let _ = tx.send(Reply::Forgot(slot, result));
                    ctx.request_repaint();
                });
            }
        }
    }

    fn input_section(&mut self, ui: &mut egui::Ui) {
        let heading = design::heading(
            ui,
            design::Heading::Group {
                title: "Recording",
                count: 0,
            },
        );
        if self.focus_input {
            heading.scroll_to_me(Some(egui::Align::TOP));
        }
        match &self.probe {
            None => {
                design::help(ui, "Looking for a recorder…");
            }
            // Round 28: why it cannot record is in the tab's footer.
            Some(Probe { tool: None, .. }) => {}
            Some(Probe {
                tool: Some(tool),
                inputs,
            }) => {
                design::help(
                    ui,
                    &format!(
                        "Records with {}, as 16 kHz mono WAV in the campaign folder.",
                        tool.name()
                    ),
                );
                if tool.can_choose_input() {
                    let inputs = inputs.clone();
                    // A chosen input that is not plugged in now stays the choice, and says so.
                    let missing = self
                        .config
                        .input
                        .clone()
                        .filter(|name| !inputs.iter().any(|input| input.name == *name));
                    let mut changed = false;
                    design::small_field(ui, "Input", None, None, |ui| {
                        let shown =
                            design::Dropdown::new(INPUT, "Recording input")
                                .choice(None, "System default")
                                .choices(inputs.iter().map(|input| {
                                    (Some(input.name.clone()), input.description.clone())
                                }))
                                .choices(missing.map(|name| {
                                    let words = format!("{name} (not found now)");
                                    (Some(name), words)
                                }))
                                .show(ui, &mut self.config.input);
                        changed = shown.changed;
                        if std::mem::take(&mut self.focus_input) {
                            shown.response.request_focus();
                        }
                    });
                    if changed {
                        self.persist();
                    }
                    // Round 28 FM11: the notice that a new input waits for the next recording is
                    // in the tab's footer, so it never pushes the form down.
                }
            }
        }
    }

    /// A one-line setting with its label above it. A value that blocks processing says so
    /// directly below its own field in the field's help form (round 26 FM6), before the field's
    /// own help; the field part then keeps `L` before the next label (N3), so the reason reads
    /// as its own field's (round 27 FI7). The first reason's place is kept for the reveal.
    fn setting_field(
        &mut self,
        ui: &mut egui::Ui,
        field: (design::FieldId, &str, &str),
        text: Text,
        reason: &str,
        help: Option<&str>,
    ) -> bool {
        let (id, label, name) = field;
        let value = match text {
            Text::SpeechUrl => &mut self.config.speech_url,
            Text::SpeechModel => &mut self.config.speech_model,
            Text::LlmUrl => &mut self.config.llm_url,
            Text::LlmModel => &mut self.config.llm_model,
        };
        let blocks = value.trim().is_empty();
        // With no help of its own, the reason is the field's help; else it comes first.
        let (help, first) = match help {
            None => (blocks.then_some(reason), None),
            Some(help) => (Some(help), blocks.then_some(reason)),
        };
        let response = design::small_field(ui, label, help, None, |ui| {
            let width = ui.available_width().min(design::FORM_WIDTH) - 2.0 * design::S;
            let response = id.single_line(ui, value, |edit| {
                edit.desired_width(width)
                    .min_size(egui::vec2(0.0, Density::Read.control()))
                    .margin(egui::vec2(design::S, 5.0))
            });
            if let Some(reason) = first {
                design::help(ui, reason);
            }
            response
        });
        design::named(ui, &response, name);
        if blocks {
            // The reason is the first line below the field.
            let font = Type::Small.font(ui);
            let line = ui.fonts_mut(|fonts| fonts.row_height(&font));
            let below = egui::Rect::from_min_size(
                response.rect.left_bottom(),
                egui::vec2(response.rect.width(), design::XS + line),
            );
            self.reason_rect.get_or_insert(below);
        }
        response.changed()
    }

    fn speech_section(&mut self, ui: &mut egui::Ui) {
        design::help(
            ui,
            "Turns your recording into text. Choose a speech service and enter its API key.",
        );
        let current = SPEECH_CHOICES
            .iter()
            .find(|(_, url, _)| self.config.speech_url == *url)
            .map_or(OTHER_SERVICE, |(name, ..)| name);
        let mut chosen = current;
        // Round 28 FI7: the custom service's help is the Service field's own, so `L` follows it
        // and it never reads as help for Address.
        let custom = (current == OTHER_SERVICE)
            .then_some("The custom service must support the OpenAI audio/transcriptions API.");
        design::small_field(ui, "Service", custom, None, |ui| {
            design::Dropdown::new(SPEECH_SERVICE, "Speech service")
                .choice(SPEECH_CHOICES[0].0, SPEECH_CHOICES[0].0)
                .choices(SPEECH_CHOICES[1..].iter().map(|(name, ..)| (*name, *name)))
                .choice(OTHER_SERVICE, OTHER_SERVICE)
                .show(ui, &mut chosen)
        });
        if chosen != current {
            let (url, model) = SPEECH_CHOICES
                .iter()
                .find(|(name, ..)| *name == chosen)
                .map_or(("", ""), |(_, url, model)| (*url, *model));
            self.config.speech_url = url.to_owned();
            self.config.speech_model = model.to_owned();
            self.persist();
            self.reveal_problem = true;
            ui.ctx().request_repaint();
        }
        let mut changed = self.setting_field(
            ui,
            (SPEECH_URL, "Address", "Speech address"),
            Text::SpeechUrl,
            "Enter the address.",
            None,
        );
        changed |= self.setting_field(
            ui,
            (SPEECH_MODEL, "Model", "Speech model"),
            Text::SpeechModel,
            "Enter the model’s name.",
            Some("The name your service gives it. For OpenAI, gpt-transcribe."),
        );
        if changed {
            self.persist();
        }
        self.key_row(ui, KeySlot::Speech, "Speech API key");
    }

    fn llm_section(&mut self, ui: &mut egui::Ui) {
        design::help(
            ui,
            "Goes over the transcript twice: once to clean it and mark events, and once to check that work against the raw words.",
        );
        let current = LLM_CHOICES
            .iter()
            .find(|(_, provider, url, _)| {
                self.config.provider == *provider && self.config.llm_url == *url
            })
            .map_or(OTHER_SERVICE, |(name, ..)| name);
        let mut chosen = current;
        design::small_field(ui, "Service", None, None, |ui| {
            design::Dropdown::new(LLM_SERVICE, "Transcript and summary service")
                .choice(LLM_CHOICES[0].0, LLM_CHOICES[0].0)
                .choices(LLM_CHOICES[1..].iter().map(|(name, ..)| (*name, *name)))
                .choice(OTHER_SERVICE, OTHER_SERVICE)
                .show(ui, &mut chosen)
        });
        if chosen != current {
            let (provider, url, model) = LLM_CHOICES
                .iter()
                .find(|(name, ..)| *name == chosen)
                .map_or(
                    (Provider::OpenAiCompatible, "", ""),
                    |(_, provider, url, model)| (*provider, *url, *model),
                );
            self.config.provider = provider;
            self.config.llm_url = url.to_owned();
            self.config.llm_model = model.to_owned();
            self.persist();
            self.reveal_problem = true;
            ui.ctx().request_repaint();
        }
        let codex = self.config.provider == Provider::CodexCli;
        let mut changed = false;
        if !codex {
            changed |= self.setting_field(
                ui,
                (LLM_URL, "Address", "Transcript and summary address"),
                Text::LlmUrl,
                "Enter the address.",
                None,
            );
        }
        changed |= self.setting_field(
            ui,
            (LLM_MODEL, "Model", "Transcript and summary model"),
            Text::LlmModel,
            "Enter the model’s name.",
            Some("The name your service gives it."),
        );
        if changed {
            self.persist();
        }
        if codex {
            let words = if self.codex_found {
                "Uses your Codex sign-in. It cannot change your campaign files. No API key is needed here. Codex cannot take audio; the speech service turns it into text first."
            } else {
                "The codex command was not found on this computer."
            };
            design::help(ui, words);
        } else {
            self.key_row(ui, KeySlot::Llm, "Transcript and summary API key");
        }
        let mut changed = false;
        // Technical, and a stray key here changes every later job.
        design::quiet_section(ui, "Transcript and summary instructions", |ui| {
            for (label, field_id, help, prompt, default) in [
                (
                    "First pass: clean and mark",
                    CLEAN_PROMPT,
                    "Cleans the raw transcript and marks events in it.",
                    &mut self.config.clean_prompt,
                    super::default_clean_prompt(),
                ),
                (
                    "Second pass: check and summarize",
                    REVIEW_PROMPT,
                    "Checks the first pass against the raw transcript, corrects it, and writes the summary. Splat divides its reply at the headings # Review and # Marked transcript.",
                    &mut self.config.review_prompt,
                    super::default_review_prompt(),
                ),
            ] {
                design::small_field(ui, label, None, None, |ui| {
                    let width = ui.available_width().min(design::FORM_WIDTH) - 2.0 * design::S;
                    let response = field_id.multi_line(ui, prompt, |edit| {
                        edit.desired_width(width)
                            .desired_rows(6)
                            .margin(egui::vec2(design::S, design::S))
                    });
                    design::named(ui, &response, label);
                    changed |= response.changed();
                    // The field's own help, at the reader measure like the rest of this tab.
                    design::help(ui, help);
                });
                if *prompt != default
                    && Action::quiet("Reset")
                        .name(&format!("Reset: {label}"))
                        .show(ui)
                        .clicked()
                {
                    *prompt = default;
                    changed = true;
                }
            }
        });
        if changed {
            self.persist();
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        design::help(
            ui,
            "Recording is your choice in each session. Splat sends nothing until you select Process on a recording. Changes here save as you type. A key needs Save key.",
        );
        // The notice that stood here is in the tab's footer (round 27 FI7): `L` before the
        // first group heading, as after any help.
        let reveal = std::mem::take(&mut self.reveal_problem);
        self.reason_rect = None;
        self.input_section(ui);
        // The headings stand outside the `push_id` scopes: a heading in a new `Ui` is first in
        // it and adds no space above itself.
        design::heading(
            ui,
            design::Heading::Group {
                title: "Speech service",
                count: 0,
            },
        );
        ui.push_id("speech_settings", |ui| self.speech_section(ui));
        design::heading(
            ui,
            design::Heading::Group {
                title: "Transcript and summary",
                count: 0,
            },
        );
        ui.push_id("llm_settings", |ui| self.llm_section(ui));
        // After the DM chooses a service that leaves a field empty, that field's reason comes
        // into view; the notice is in the tab's footer. A key problem has its reason at the key
        // row beside the choice, so the view stays where the DM is.
        if reveal && let Some(reason) = self.reason_rect {
            ui.scroll_to_rect(reason, None);
        }
    }

    /// The Recording tab as one pane: its fields, then its message footer, `S` below them, pinned
    /// only when they fill the pane (round 28, `Body::Content`). The messages are made before the
    /// fields draw, from owned words.
    pub fn pane(&mut self, ui: &mut egui::Ui) {
        let width = ui.available_width().min(design::LIST_MEASURE);
        let frame = PaneFrame::new(ui);
        let messages = self.messages(&frame, width);
        let _ = messages.pane(ui, Body::Content, |ui| {
            let mut fields = ui.available_rect_before_wrap();
            fields.min.x += design::S;
            ui.scope_builder(egui::UiBuilder::new().max_rect(fields), |ui| {
                ui.set_max_width(design::FORM_WIDTH + design::XXL);
                self.show(ui);
            });
        });
    }

    /// Round 27 FI7: the Recording tab's notices sit in its message footer, so the form never
    /// moves when one comes or goes (the DM fills the last missing value, a save fails). Round
    /// 28 FM11: so does the notice that a new input waits for the next recording.
    fn messages<'f>(&self, frame: &'f PaneFrame, width: f32) -> Messages<'f, NoPress> {
        let mut messages = Messages::new(frame, "recording_settings_footer", width);
        if let Some(error) = &self.load_error {
            messages.outcome(FaultMessage::new(&SettingsFault::Unreadable(error)));
        }
        if let Some(error) = &self.save_error {
            messages.outcome(FaultMessage::new(&SettingsFault::NotSaved(error)));
        }
        match &self.probe {
            Some(Probe { tool: None, .. }) => {
                messages.outcome(FaultMessage::new(&SettingsFault::NoRecorder));
            }
            Some(Probe {
                tool: Some(tool), ..
            }) if tool.can_choose_input() && self.recording_now => {
                messages.outcome(Notice::new(crate::sentence!(
                    "A new input applies to the next recording. Stop and record again to use it."
                )));
            }
            _ => {}
        }
        if let Some(problem) = self.problem() {
            let problem = if problem.starts_with("Save an API key for this speech service") {
                let host = super::host(&self.config.speech_url);
                format!(
                    "Save an {} key under Speech service.",
                    if host == "api.openai.com" {
                        "OpenAI"
                    } else {
                        &host
                    }
                )
            } else {
                problem.replace(" in Settings", "")
            };
            messages.outcome(Notice::new(crate::sentence!(
                "You cannot process yet. {}.",
                problem.trim_end_matches('.')
            )));
        }
        messages
    }
}

/// A key's fault, where its key row is: the row's own place, so the DM reads it beside the key.
#[allow(
    clippy::disallowed_methods,
    reason = "a key’s fault belongs to its key row (`Ban::Body`)"
)]
fn key_fault(ui: &mut egui::Ui, fault: &SettingsFault<'_>) {
    crate::design::outcome::row_fault(ui, fault);
}

/// The service a key was saved for, by its address.
fn key_service_name(saved_for: &str) -> &str {
    if saved_for.contains("api.openai.com") {
        "OpenAI"
    } else if saved_for.contains("anthropic.com") {
        "Anthropic"
    } else if saved_for.contains("groq.com") {
        "Groq"
    } else if saved_for.contains("googleapis.com") {
        "Gemini"
    } else if saved_for.contains("openrouter.ai") {
        "OpenRouter"
    } else {
        saved_for
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .split('/')
            .next()
            .filter(|s| !s.is_empty())
            .unwrap_or("this service")
    }
}
