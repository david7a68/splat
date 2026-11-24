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

#[derive(Default)]
struct Store {
    path: Option<PathBuf>,
}

impl Store {
    #[cfg(not(any(test, feature = "test-support")))]
    fn user() -> Self {
        Self {
            path: directories::ProjectDirs::from("", "", "splat")
                .map(|dirs| dirs.config_dir().join("session-processing.json")),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
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
        self.codex_found = cfg!(any(test, feature = "test-support")) || recording::on_path("codex");
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

    #[cfg(any(test, feature = "test-support"))]
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

    /// Faults to show, for a test of their places: the keyring's words for each key, and the
    /// settings file's reader's words. `None` clears one.
    #[cfg(any(test, feature = "test-support"))]
    pub fn simulate_faults(&mut self, speech: Option<&str>, llm: Option<&str>, file: Option<&str>) {
        self.speech_key.error = speech.map(str::to_owned);
        self.llm_key.error = llm.map(str::to_owned);
        self.load_error = file.map(str::to_owned);
    }

    #[cfg(any(test, feature = "test-support"))]
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

#[path = "settings/view.rs"]
pub mod view;

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
