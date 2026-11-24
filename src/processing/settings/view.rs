//! The recording settings' reads and acts, free of any toolkit, so the egui tab
//! (`ProcessingSettings::show`) and the GPUI tab (`src/gui/pages/settings/recording.rs`) share
//! them.
use super::{
    KeySlot, LLM_CHOICES, OTHER_SERVICE, ProcessingSettings, Provider, Reply, SPEECH_CHOICES,
    SettingsFault, Text, key_service_name,
};
use crate::credentials;
use crate::design::outcome::{Fault as _, NoPress, PaneFrame, plain::PlainLine};
use crate::recording::Probe;
use eframe::egui;
use zeroize::{Zeroize as _, Zeroizing};

/// Which text setting a page edits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Setting {
    SpeechUrl,
    SpeechModel,
    LlmUrl,
    LlmModel,
}

/// Which of the two prompts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prompt {
    Clean,
    Review,
}

/// What the recorder search found, as the Recording group says it.
pub enum Recorder {
    /// Still looking.
    Looking,
    /// None installed (the footer says so).
    None,
    /// Found: its words, and the inputs when it can choose one.
    Found {
        help: String,
        inputs: Option<Vec<(Option<String>, String)>>,
    },
}

/// One API key's row, as a page shows it.
pub struct KeyView {
    pub typing: String,
    pub saved: bool,
    pub held: bool,
    pub fits: bool,
    /// `Remove speech key (OpenAI)`.
    pub remove: String,
    /// The key saved for another service, in words.
    pub for_another: Option<String>,
    /// The keyring's words when it did not keep or remove the key.
    pub not_kept: Option<String>,
}

impl From<Setting> for Text {
    fn from(setting: Setting) -> Self {
        match setting {
            Setting::SpeechUrl => Self::SpeechUrl,
            Setting::SpeechModel => Self::SpeechModel,
            Setting::LlmUrl => Self::LlmUrl,
            Setting::LlmModel => Self::LlmModel,
        }
    }
}

impl ProcessingSettings {
    /// The recorder, and the inputs it can choose from (a chosen input not plugged in now stays,
    /// and says so).
    pub fn recorder(&self) -> Recorder {
        match &self.probe {
            None => Recorder::Looking,
            Some(Probe { tool: None, .. }) => Recorder::None,
            Some(Probe {
                tool: Some(tool),
                inputs,
            }) => Recorder::Found {
                help: format!(
                    "Records with {}, as 16 kHz mono WAV in the campaign folder.",
                    tool.name()
                ),
                inputs: tool.can_choose_input().then(|| {
                    let mut choices = vec![(None, "System default".to_owned())];
                    choices.extend(
                        inputs
                            .iter()
                            .map(|input| (Some(input.name.clone()), input.description.clone())),
                    );
                    if let Some(name) = self
                        .config
                        .input
                        .clone()
                        .filter(|name| !inputs.iter().any(|input| input.name == *name))
                    {
                        let words = format!("{name} (not found now)");
                        choices.push((Some(name), words));
                    }
                    choices
                }),
            },
        }
    }

    /// The DM chose an input (`None`: the system default).
    pub fn set_input(&mut self, input: Option<String>) {
        if self.config.input != input {
            self.config.input = input;
            self.persist();
        }
    }

    /// The service names a choice lists, and the one chosen now.
    pub fn speech_services(&self) -> (Vec<&'static str>, &'static str) {
        let current = SPEECH_CHOICES
            .iter()
            .find(|(_, url, _)| self.config.speech_url == *url)
            .map_or(OTHER_SERVICE, |(name, ..)| name);
        let mut names: Vec<_> = SPEECH_CHOICES.iter().map(|(name, ..)| *name).collect();
        names.push(OTHER_SERVICE);
        (names, current)
    }

    pub fn llm_services(&self) -> (Vec<&'static str>, &'static str) {
        let current = LLM_CHOICES
            .iter()
            .find(|(_, provider, url, _)| {
                self.config.provider == *provider && self.config.llm_url == *url
            })
            .map_or(OTHER_SERVICE, |(name, ..)| name);
        let mut names: Vec<_> = LLM_CHOICES.iter().map(|(name, ..)| *name).collect();
        names.push(OTHER_SERVICE);
        (names, current)
    }

    /// The DM chose a speech service: its address and model fill in.
    pub fn choose_speech_service(&mut self, chosen: &str) {
        if chosen == self.speech_services().1 {
            return;
        }
        let (url, model) = SPEECH_CHOICES
            .iter()
            .find(|(name, ..)| *name == chosen)
            .map_or(("", ""), |(_, url, model)| (*url, *model));
        self.config.speech_url = url.to_owned();
        self.config.speech_model = model.to_owned();
        self.persist();
        self.reveal_problem = true;
    }

    /// The DM chose a transcript and summary service.
    pub fn choose_llm_service(&mut self, chosen: &str) {
        if chosen == self.llm_services().1 {
            return;
        }
        let (provider, url, model) = LLM_CHOICES
            .iter()
            .find(|(name, ..)| *name == chosen)
            .map_or((Provider::OpenAiCompatible, "", ""), |(_, provider, url, model)| {
                (*provider, *url, *model)
            });
        self.config.provider = provider;
        self.config.llm_url = url.to_owned();
        self.config.llm_model = model.to_owned();
        self.persist();
        self.reveal_problem = true;
    }

    /// The transcript service is the Codex command (no address, no key).
    pub fn uses_codex(&self) -> bool {
        self.config.provider == Provider::CodexCli
    }

    /// The Codex command is on this computer.
    pub fn codex_found(&self) -> bool {
        self.codex_found
    }

    pub fn setting(&self, setting: Setting) -> &str {
        match Text::from(setting) {
            Text::SpeechUrl => &self.config.speech_url,
            Text::SpeechModel => &self.config.speech_model,
            Text::LlmUrl => &self.config.llm_url,
            Text::LlmModel => &self.config.llm_model,
        }
    }

    /// An edit of a text setting saves as the DM types.
    pub fn set_setting(&mut self, setting: Setting, words: &str) {
        let value = match Text::from(setting) {
            Text::SpeechUrl => &mut self.config.speech_url,
            Text::SpeechModel => &mut self.config.speech_model,
            Text::LlmUrl => &mut self.config.llm_url,
            Text::LlmModel => &mut self.config.llm_model,
        };
        if value != words {
            *value = words.to_owned();
            self.persist();
        }
    }

    pub fn prompt(&self, prompt: Prompt) -> &str {
        match prompt {
            Prompt::Clean => &self.config.clean_prompt,
            Prompt::Review => &self.config.review_prompt,
        }
    }

    /// The prompt differs from Splat's own (`Reset` shows).
    pub fn prompt_changed(&self, prompt: Prompt) -> bool {
        self.prompt(prompt) != Self::default_prompt(prompt)
    }

    fn default_prompt(prompt: Prompt) -> String {
        match prompt {
            Prompt::Clean => super::super::default_clean_prompt(),
            Prompt::Review => super::super::default_review_prompt(),
        }
    }

    pub fn set_prompt(&mut self, prompt: Prompt, words: &str) {
        let value = match prompt {
            Prompt::Clean => &mut self.config.clean_prompt,
            Prompt::Review => &mut self.config.review_prompt,
        };
        if value != words {
            *value = words.to_owned();
            self.persist();
        }
    }

    /// `Reset`: Splat's own prompt comes back.
    pub fn reset_prompt(&mut self, prompt: Prompt) {
        self.set_prompt(prompt, &Self::default_prompt(prompt));
    }

    /// One key's row.
    pub fn key_view(&mut self, slot: KeySlot) -> KeyView {
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
        let held = self.held(slot).is_some();
        let field = self.field(slot);
        let saved = field.saved;
        let for_another = ((saved || held) && !fits).then(|| {
            SettingsFault::KeyForAnother {
                owner: &owner,
                target: &target,
            }
            .cause()
            .as_str()
            .to_owned()
        });
        let not_kept = field.error.as_deref().map(|error| {
            SettingsFault::KeyNotKept(error)
                .cause()
                .as_str()
                .to_owned()
        });
        KeyView {
            typing: field.typing.clone(),
            saved,
            held,
            fits,
            remove: format!("Remove {slot_word} key ({owner})"),
            for_another,
            not_kept,
        }
    }

    pub fn set_key_typing(&mut self, slot: KeySlot, words: &str) {
        let field = self.field(slot);
        field.typing.zeroize();
        field.typing = words.to_owned();
    }

    /// The speech key field asked for the keyboard (once, from a session's notice).
    pub fn take_focus_speech_key(&mut self) -> bool {
        std::mem::take(&mut self.focus_speech_key)
    }

    /// `Save key`: the key is usable at once and saves to the keyring in the background.
    pub fn save_key(&mut self, slot: KeySlot, ctx: &egui::Context) {
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
        if let Some(tx) = self.replies.as_ref().map(|(tx, _)| tx.clone()) {
            let ctx = ctx.clone();
            credentials::in_background(move || {
                let result = credentials::save_key(slot, &key);
                let _ = tx.send(Reply::Saved(slot, result));
                ctx.request_repaint();
            });
        }
    }

    /// `Remove … key`.
    pub fn remove_key(&mut self, slot: KeySlot, ctx: &egui::Context) {
        *self.held(slot) = None;
        if let Some(tx) = self.replies.as_ref().map(|(tx, _)| tx.clone()) {
            let ctx = ctx.clone();
            credentials::in_background(move || {
                let result = credentials::forget_key(slot);
                let _ = tx.send(Reply::Forgot(slot, result));
                ctx.request_repaint();
            });
        }
    }

    /// The tab's footer (`messages`), as plain lines.
    pub fn footer_lines(&self) -> Vec<PlainLine<NoPress>> {
        let frame = PaneFrame::plain();
        self.messages(&frame, 0.0).into_plain()
    }
}
