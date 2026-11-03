//! Settings for recording and processing: which input, which speech service, which LLM.
//! The choices live in a small file beside the settings file. Keys live in the keyring.
use super::{
    ANTHROPIC_MODEL, ANTHROPIC_URL, CODEX_MODEL, Config, OPENAI_SPEECH_MODEL, OPENAI_URL, Provider,
};
use crate::{
    credentials::{self, KeySlot},
    design::{self, Action, Tone, Type, colors, icons},
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
                Some("The codex command was not found. Install Codex, or choose another LLM in Settings.".into())
            } else if !codex && !self.has_key(KeySlot::Llm) && !super::is_local(&self.config.llm_url) {
                Some("Save an API key for this LLM in Settings.".into())
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
            .map(|error| format!("These settings were not saved. {error}"));
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
        fn service_name(saved_for: &str) -> &str {
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
                    .unwrap_or("service")
            }
        }
        let owner = service_name(saved_for);
        let target = service_name(match slot {
            KeySlot::Speech => &self.config.speech_url,
            KeySlot::Llm => &self.config.llm_url,
        });
        let warning = format!(
            "The saved key is for {owner}, so Splat will not send it to {target}. Save a key for this service."
        );
        let slot_word = if slot == KeySlot::Speech {
            "speech"
        } else {
            "LLM"
        };
        let remove = format!("Remove {slot_word} key ({owner})");
        let remove_name = format!("{remove}: {label}");
        let held = self.held(slot).is_some();
        let focus = slot == KeySlot::Speech && std::mem::take(&mut self.focus_speech_key);
        let mut save = false;
        design::small_field(ui, "API key", None, None, |ui| {
            ui.horizontal(|ui| {
                let field = self.field(slot);
                let empty = field.typing.trim().is_empty();
                let show_save = !empty || (!saved && !held) || !fits;
                let reason = (empty && show_save).then_some("Paste a key first.");
                let reason_width = reason.map_or(0.0, |text| {
                    design::number_width(ui, text, &Type::Small.font(ui)) + design::S
                });
                let room = if show_save {
                    Action::secondary("Save key").natural_width(ui)
                } else {
                    0.0
                } + reason_width
                    + design::S * 3.0;
                let width = (ui.available_width() - room - design::S * 2.0)
                    .clamp(design::L * 4.0, design::FORM_WIDTH / 2.0);
                let response = ui.add(
                    egui::TextEdit::singleline(&mut field.typing)
                        .password(true)
                        .desired_width(width)
                        .margin(egui::vec2(design::S, 5.0))
                        .hint_text(if saved && fits {
                            "Paste a new key to replace it"
                        } else {
                            "Paste the key"
                        }),
                );
                design::named(ui, &response, label);
                if focus {
                    response.request_focus();
                    response.scroll_to_me(Some(egui::Align::Center));
                }
                if show_save {
                    save = Action::secondary("Save key")
                        .name(&format!("Save key: {label}"))
                        .blocked_if(empty, "Paste a key first.")
                        .show(ui)
                        .clicked();
                }
                design::blocked_reason(ui, reason);
            });
        });
        if (saved || held) && !fits {
            design::message(ui, Tone::Warning, &warning);
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
        let field = self.field(slot);
        if let Some(error) = &field.error {
            design::message(
                ui,
                Tone::Warning,
                &format!("{error} Splat keeps the key until it closes."),
            );
        } else if saved && fits {
            ui.label(
                Type::Small
                    .rich(ui, format!("{} Saved in the desktop keyring.", icons::OK))
                    .color(colors().document.saved),
            );
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
        let c = colors();
        let heading = ui
            .scope(|ui| design::group_heading(ui, "Recording", 0))
            .response;
        if self.focus_input {
            heading.scroll_to_me(Some(egui::Align::TOP));
        }
        match &self.probe {
            None => {
                ui.label(
                    Type::Small
                        .rich(ui, "Looking for a recorder…")
                        .color(c.muted),
                );
            }
            Some(Probe { tool: None, .. }) => {
                design::message(
                    ui,
                    Tone::Warning,
                    "No recorder found. Install pw-record (PipeWire), parecord (PulseAudio) or arecord (ALSA). Notes and the timeline work without one.",
                );
            }
            Some(Probe {
                tool: Some(tool),
                inputs,
            }) => {
                ui.label(
                    Type::Small
                        .rich(
                            ui,
                            format!(
                                "Records with {}, as 16 kHz mono WAV in the campaign folder.",
                                tool.name()
                            ),
                        )
                        .color(c.muted),
                );
                if tool.can_choose_input() {
                    let inputs = inputs.clone();
                    let current = self.config.input.clone();
                    let label = current.as_deref().map_or_else(
                        || "System default".to_owned(),
                        |name| {
                            inputs.iter().find(|input| input.name == name).map_or_else(
                                || format!("{name} (not found now)"),
                                |i| i.description.clone(),
                            )
                        },
                    );
                    let mut changed = false;
                    design::small_field(ui, "Input", None, None, |ui| {
                        let inner = egui::ComboBox::from_id_salt("recording_input")
                            .selected_text(&label)
                            .width(ui.available_width().min(design::FORM_WIDTH))
                            .icon(design::combo_icon)
                            .show_ui(ui, |ui| {
                                changed |= ui
                                    .selectable_value(
                                        &mut self.config.input,
                                        None,
                                        "System default",
                                    )
                                    .changed();
                                for input in &inputs {
                                    changed |= ui
                                        .selectable_value(
                                            &mut self.config.input,
                                            Some(input.name.clone()),
                                            &input.description,
                                        )
                                        .changed();
                                }
                            });
                        design::named(ui, &inner.response, "Recording input");
                        if std::mem::take(&mut self.focus_input) {
                            inner.response.request_focus();
                        }
                    });
                    if changed {
                        self.persist();
                    }
                    if self.recording_now {
                        design::message(
                            ui,
                            Tone::Info,
                            "A new input applies to the next recording. Stop and record again to use it.",
                        );
                    }
                }
            }
        }
    }

    fn speech_section(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        design::group_heading(ui, "Speech service", 0);
        ui.label(
            Type::Small
                .rich(ui, "Writes a transcript from the audio, before the LLM. The Codex CLI cannot take audio, so this is a separate service, with its own key. It must offer the OpenAI audio/transcriptions API.")
                .color(c.muted),
        );
        let current = SPEECH_CHOICES
            .iter()
            .find(|(_, url, _)| self.config.speech_url == *url)
            .map_or(OTHER_SERVICE, |(name, ..)| name);
        let mut chosen = None;
        design::small_field(ui, "Service", None, None, |ui| {
            let inner = egui::ComboBox::from_id_salt("speech_service")
                .selected_text(current)
                .width(ui.available_width().min(design::FORM_WIDTH))
                .icon(design::combo_icon)
                .show_ui(ui, |ui| {
                    for (name, url, model) in SPEECH_CHOICES {
                        if ui.selectable_label(current == name, name).clicked() {
                            chosen = Some((url, model));
                        }
                    }
                    if ui
                        .selectable_label(current == OTHER_SERVICE, OTHER_SERVICE)
                        .clicked()
                    {
                        chosen = Some(("", ""));
                    }
                });
            design::named(ui, &inner.response, "Speech service");
        });
        if let Some((url, model)) = chosen {
            self.config.speech_url = url.to_owned();
            self.config.speech_model = model.to_owned();
            self.persist();
        }
        let mut changed = false;
        let response =
            design::small_text_field(ui, "Address", &mut self.config.speech_url, None, None);
        design::named(ui, &response, "Speech address");
        changed |= response.changed();
        let response = design::small_text_field(
            ui,
            "Model",
            &mut self.config.speech_model,
            Some("The name your service gives it. For OpenAI, gpt-transcribe."),
            None,
        );
        design::named(ui, &response, "Speech model");
        changed |= response.changed();
        if changed {
            self.persist();
        }
        self.key_row(ui, KeySlot::Speech, "Speech API key");
    }

    fn llm_section(&mut self, ui: &mut egui::Ui) {
        let c = colors();
        design::group_heading(ui, "LLM", 0);
        ui.label(
            Type::Small
                .rich(
                    ui,
                    "Goes over the transcript twice: once to clean it and mark events, and once to check that work against the raw words.",
                )
                .color(c.muted),
        );
        let current = LLM_CHOICES
            .iter()
            .find(|(_, provider, url, _)| {
                self.config.provider == *provider && self.config.llm_url == *url
            })
            .map_or(OTHER_SERVICE, |(name, ..)| name);
        let mut chosen = None;
        design::small_field(ui, "Service", None, None, |ui| {
            let inner = egui::ComboBox::from_id_salt("llm_service")
                .selected_text(current)
                .width(ui.available_width().min(design::FORM_WIDTH))
                .icon(design::combo_icon)
                .show_ui(ui, |ui| {
                    for (name, provider, url, model) in LLM_CHOICES {
                        if ui.selectable_label(current == name, name).clicked() {
                            chosen = Some((provider, url, model));
                        }
                    }
                    if ui
                        .selectable_label(current == OTHER_SERVICE, OTHER_SERVICE)
                        .clicked()
                    {
                        chosen = Some((Provider::OpenAiCompatible, "", ""));
                    }
                });
            design::named(ui, &inner.response, "LLM service");
        });
        if let Some((provider, url, model)) = chosen {
            self.config.provider = provider;
            self.config.llm_url = url.to_owned();
            self.config.llm_model = model.to_owned();
            self.persist();
        }
        let mut changed = false;
        let codex = self.config.provider == Provider::CodexCli;
        if !codex {
            let response =
                design::small_text_field(ui, "Address", &mut self.config.llm_url, None, None);
            design::named(ui, &response, "LLM address");
            changed |= response.changed();
        }
        let response = design::small_text_field(
            ui,
            "Model",
            &mut self.config.llm_model,
            Some("The name your service gives it."),
            None,
        );
        design::named(ui, &response, "LLM model");
        changed |= response.changed();
        if changed {
            self.persist();
        }
        if codex {
            let words = if self.codex_found {
                "Uses your Codex sign-in, in a read-only sandbox. It needs no key here."
            } else {
                "The codex command was not found on this computer."
            };
            ui.label(Type::Small.rich(ui, words).color(c.muted));
        } else {
            self.key_row(ui, KeySlot::Llm, "LLM API key");
        }
        let mut changed = false;
        // Technical, and a stray key here changes every later job.
        design::quiet_section(ui, "Instructions to the LLM", |ui| {
            for (label, help, prompt, default) in [
                (
                    "First pass: clean and mark",
                    "Cleans the raw transcript and marks events in it.",
                    &mut self.config.clean_prompt,
                    super::default_clean_prompt(),
                ),
                (
                    "Second pass: check and summarize",
                    "Checks the first pass against the raw transcript, corrects it, and writes the summary. Splat divides its reply at the headings # Review and # Marked transcript.",
                    &mut self.config.review_prompt,
                    super::default_review_prompt(),
                ),
            ] {
                design::small_field(ui, label, Some(help), None, |ui| {
                    let response = ui.add(
                        egui::TextEdit::multiline(prompt)
                            .desired_width(
                                ui.available_width().min(design::FORM_WIDTH) - 2.0 * design::S,
                            )
                            .desired_rows(6)
                            .margin(egui::vec2(design::S, design::S)),
                    );
                    design::named(ui, &response, label);
                    changed |= response.changed();
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
        let c = colors();
        ui.label(
            Type::Small
                .rich(ui, "Recording is your choice in each session. Splat sends nothing until you select Process on a recording. Changes here save as you type. A key needs Save key.")
                .color(c.muted),
        );
        ui.add_space(design::S);
        if let Some(error) = &self.load_error {
            design::message(ui, Tone::Warning, error);
        }
        if let Some(error) = &self.save_error {
            design::message(ui, Tone::Error, error);
        }
        if let Some(problem) = self.problem() {
            let problem = if problem.starts_with("Save an API key for this speech service") {
                let host = super::host(&self.config.speech_url);
                format!(
                    "save an {} key under Speech service.",
                    if host == "api.openai.com" {
                        "OpenAI"
                    } else {
                        &host
                    }
                )
            } else {
                problem.replace(" in Settings", "")
            };
            design::message(
                ui,
                Tone::Info,
                &format!("Before you can process: {problem}"),
            );
        }
        ui.add_space(design::L);
        self.input_section(ui);
        ui.add_space(design::L);
        ui.push_id("speech_settings", |ui| self.speech_section(ui));
        ui.add_space(design::L);
        ui.push_id("llm_settings", |ui| self.llm_section(ui));
    }
}
