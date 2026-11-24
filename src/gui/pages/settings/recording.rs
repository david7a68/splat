//! Settings → Recording and processing (egui: `ProcessingSettings::pane`, `show`): the recorder
//! and its input, the speech service, the transcript and summary service with their keys, and
//! the two prompts; the tab's notices in its footer.
use crate::credentials::KeySlot;
use crate::gui::kit::{
    self, Button, Dropdown, Message, Pane, Section, TextEditor, TextField, Tone,
};
use crate::gui::theme::{FORM_WIDTH, L, S, Type, XS, XXL, colors};
use crate::gui::{SplatRoot, icons};
use crate::processing::settings::view::{Prompt, Recorder, Setting};
use gpui::{
    AnyElement, Context, IntoElement, ParentElement, Styled, Window, div, prelude::*,
};

/// The tab's own GPUI state.
#[derive(Default)]
pub struct State {
    /// `Transcript and summary instructions` is open.
    pub(crate) prompts_open: bool,
}

pub(super) fn render(root: &mut SplatRoot, _: &mut Window, cx: &mut Context<SplatRoot>) -> AnyElement {
    let lines = root.app.processing.footer_lines();
    let mut column = div()
        .flex()
        .flex_col()
        .w_full()
        .max_w(FORM_WIDTH + XXL)
        .pl(S)
        .child(kit::help(
            "Recording is your choice in each session. Splat sends nothing until you select Process on a recording. Changes here save as you type. A key needs Save key.",
        ));
    column = column.child(input_section(root, cx));
    column = column.child(kit::heading(
        kit::Heading::Group {
            title: "Speech service",
            count: 0,
        },
        false,
    ));
    column = column.child(speech_section(root, cx));
    column = column.child(kit::heading(
        kit::Heading::Group {
            title: "Transcript and summary",
            count: 0,
        },
        false,
    ));
    column = column.child(llm_section(root, cx));
    let mut pane = Pane::new("recording-settings")
        .flush()
        .child(div().pt(L).child(column));
    for line in kit::footer::lines("recording-settings-footer", lines, |_, _, _| {}) {
        pane = pane.footer(line);
    }
    pane.into_any_element()
}

fn input_section(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> AnyElement {
    let mut column = div().flex().flex_col().child(kit::heading(
        kit::Heading::Group {
            title: "Recording",
            count: 0,
        },
        false,
    ));
    match root.app.processing.recorder() {
        Recorder::Looking => column = column.child(kit::help("Looking for a recorder…")),
        Recorder::None => {}
        Recorder::Found { help, inputs } => {
            column = column.child(div().pb(L).child(kit::help(help)));
            if let Some(inputs) = inputs {
                let chosen = root.app.processing.config.input.clone();
                let at = inputs.iter().position(|(name, _)| *name == chosen);
                let names: Vec<String> = inputs.iter().map(|(_, words)| words.clone()).collect();
                let values: Vec<Option<String>> =
                    inputs.into_iter().map(|(name, _)| name).collect();
                root.app.processing.focus_input = false;
                column = column.child(kit::field(
                    "Input",
                    None,
                    None,
                    Dropdown::new("recording_input", "Recording input", names, at)
                        .width(FORM_WIDTH)
                        .on_select(cx.listener(move |root, index: &usize, _, cx| {
                            if let Some(value) = values.get(*index) {
                                root.app.processing.set_input(value.clone());
                                cx.notify();
                            }
                        })),
                ));
            }
        }
    }
    column.into_any_element()
}

/// A one-line setting: its caption, its field, the reason it blocks processing (directly below
/// the field, as a warning), then its help.
fn setting_field(
    root: &SplatRoot,
    setting: Setting,
    (label, name): (&'static str, &'static str),
    reason: &'static str,
    help: Option<&'static str>,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let value = root.app.processing.setting(setting).to_owned();
    let blocks = value.trim().is_empty();
    let field = TextField::new(name, value)
        .name(name)
        .width(FORM_WIDTH)
        .on_change(cx.listener(move |root, words: &str, _, cx| {
            root.app.processing.set_setting(setting, words);
            cx.notify();
        }));
    let c = colors();
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .pb(L)
        .child(kit::text(Type::Small, c.muted, label))
        .child(field)
        .when(blocks, |block| {
            block.child(Message::new(Tone::Warning, reason).small())
        })
        .when_some(help, |block, help| block.child(kit::help(help)))
        .into_any_element()
}

fn speech_section(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> AnyElement {
    let (names, current) = root.app.processing.speech_services();
    let custom = (current == names[names.len() - 1])
        .then_some("The custom service must support the OpenAI audio/transcriptions API.");
    let at = names.iter().position(|name| *name == current);
    let choices = names.clone();
    let service = Dropdown::new("speech_service", "Speech service", names, at)
        .width(FORM_WIDTH)
        .on_select(cx.listener(move |root, index: &usize, _, cx| {
            if let Some(name) = choices.get(*index) {
                root.app.processing.choose_speech_service(name);
                cx.notify();
            }
        }));
    div()
        .flex()
        .flex_col()
        .child(div().pb(L).child(kit::help(
            "Turns your recording into text. Choose a speech service and enter its API key.",
        )))
        .child(kit::field("Service", custom.map(Into::into), None, service))
        .child(setting_field(
            root,
            Setting::SpeechUrl,
            ("Address", "Speech address"),
            "Enter the address.",
            None,
            cx,
        ))
        .child(setting_field(
            root,
            Setting::SpeechModel,
            ("Model", "Speech model"),
            "Enter the model’s name.",
            Some("The name your service gives it. For OpenAI, gpt-transcribe."),
            cx,
        ))
        .child(key_row(root, KeySlot::Speech, "Speech API key", cx))
        .into_any_element()
}

fn llm_section(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let (names, current) = root.app.processing.llm_services();
    let at = names.iter().position(|name| *name == current);
    let choices = names.clone();
    let service = Dropdown::new("llm_service", "Transcript and summary service", names, at)
        .width(FORM_WIDTH)
        .on_select(cx.listener(move |root, index: &usize, _, cx| {
            if let Some(name) = choices.get(*index) {
                root.app.processing.choose_llm_service(name);
                cx.notify();
            }
        }));
    let codex = root.app.processing.uses_codex();
    let mut column = div()
        .flex()
        .flex_col()
        .child(div().pb(L).child(kit::help(
            "Goes over the transcript twice: once to clean it and mark events, and once to check that work against the raw words.",
        )))
        .child(kit::field("Service", None, None, service));
    if !codex {
        column = column.child(setting_field(
            root,
            Setting::LlmUrl,
            ("Address", "Transcript and summary address"),
            "Enter the address.",
            None,
            cx,
        ));
    }
    column = column.child(setting_field(
        root,
        Setting::LlmModel,
        ("Model", "Transcript and summary model"),
        "Enter the model’s name.",
        Some("The name your service gives it."),
        cx,
    ));
    if codex {
        column = column.child(kit::help(if root.app.processing.codex_found() {
            "Uses your Codex sign-in. It cannot change your campaign files. No API key is needed here. Codex cannot take audio; the speech service turns it into text first."
        } else {
            "The codex command was not found on this computer."
        }));
    } else {
        column = column.child(key_row(root, KeySlot::Llm, "Transcript and summary API key", cx));
    }
    let open = root.pages.settings.recording.prompts_open;
    let mut prompts = Section::new(
        "recording-prompts",
        "Transcript and summary instructions",
        open,
    )
    .quiet()
    .on_toggle(cx.listener(|root, open: &bool, _, cx| {
        root.pages.settings.recording.prompts_open = *open;
        cx.notify();
    }));
    if open {
        for (prompt, label, help) in [
            (
                Prompt::Clean,
                "First pass: clean and mark",
                "Cleans the raw transcript and marks events in it.",
            ),
            (
                Prompt::Review,
                "Second pass: check and summarize",
                "Checks the first pass against the raw transcript, corrects it, and writes the summary. Splat divides its reply at the headings # Review and # Marked transcript.",
            ),
        ] {
            let editor = TextEditor::new(
                match prompt {
                    Prompt::Clean => "clean_prompt",
                    Prompt::Review => "review_prompt",
                },
                root.app.processing.prompt(prompt).to_owned(),
            )
            .name(label)
            .rows(6)
            .on_change(cx.listener(move |root, words: &str, _, cx| {
                root.app.processing.set_prompt(prompt, words);
                cx.notify();
            }));
            prompts = prompts.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(XS)
                    .w(FORM_WIDTH)
                    .child(kit::text(Type::Small, c.muted, label))
                    .child(editor)
                    .child(kit::help(help)),
            );
            if root.app.processing.prompt_changed(prompt) {
                prompts = prompts.child(
                    Button::quiet(
                        match prompt {
                            Prompt::Clean => "reset-clean-prompt",
                            Prompt::Review => "reset-review-prompt",
                        },
                        "Reset",
                    )
                    .name(format!("Reset: {label}"))
                    .on_click(cx.listener(move |root, _, _, cx| {
                        root.app.processing.reset_prompt(prompt);
                        cx.notify();
                    })),
                );
            }
            prompts = prompts.child(kit::gap(S));
        }
    }
    column.child(prompts).into_any_element()
}

/// An API key: its field and `Save key` on one line, the reason or the saved status under it,
/// its faults, and `Remove … key` while one is saved or held.
fn key_row(
    root: &mut SplatRoot,
    slot: KeySlot,
    label: &'static str,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let focus = slot == KeySlot::Speech && root.app.processing.take_focus_speech_key();
    let key = root.app.processing.key_view(slot);
    let empty = key.typing.trim().is_empty();
    let show_save = !empty || (!key.saved && !key.held) || !key.fits;
    let reason = (empty && show_save).then_some("Paste a key first.");
    let id = match slot {
        KeySlot::Speech => "speech_api_key",
        KeySlot::Llm => "llm_api_key",
    };
    let field = TextField::new(id, key.typing.clone())
        .placeholder(if key.saved && key.fits {
            "Paste a new key to replace it"
        } else {
            "Paste the key"
        })
        .name(label)
        .masked()
        .focus(focus)
        .on_change(cx.listener(move |root, words: &str, _, cx| {
            root.app.processing.set_key_typing(slot, words);
            cx.notify();
        }));
    let mut line = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .w(FORM_WIDTH)
        .child(div().flex_1().min_w_0().child(field));
    if show_save {
        line = line.child(
            Button::secondary(format!("{id}-save"), "Save key")
                .name(format!("Save key: {label}"))
                .blocked(empty.then_some("Paste a key first."))
                .on_click(cx.listener(move |root, _, _, cx| {
                    let SplatRoot { app, bridge, .. } = root;
                    app.processing.save_key(slot, bridge.ctx());
                    cx.notify();
                })),
        );
    }
    let mut block = div()
        .flex()
        .flex_col()
        .gap(XS)
        .pb(L)
        .child(kit::text(Type::Small, c.muted, "API key"))
        .child(line);
    if key.not_kept.is_none() {
        if let Some(reason) = reason {
            block = block.child(kit::text(Type::Small, c.muted, reason));
        } else if key.saved && key.fits {
            block = block.child(kit::status(&kit::Status::new(
                icons::OK,
                "Saved on this computer.",
                c.document.saved,
            )));
        } else {
            block = block.child(div().h(Type::Small.line()));
        }
    }
    let mut column = div().flex().flex_col().child(block);
    if let Some(words) = key.for_another {
        column = column.child(Message::new(Tone::Warning, words).small());
    }
    if let Some(words) = key.not_kept {
        column = column.child(Message::new(Tone::Error, words).small());
    }
    if key.saved || key.held {
        column = column.child(
            Button::quiet(format!("{id}-remove"), key.remove.clone())
                .line_start()
                .name(format!("{}: {label}", key.remove))
                .on_click(cx.listener(move |root, _, _, cx| {
                    let SplatRoot { app, bridge, .. } = root;
                    app.processing.remove_key(slot, bridge.ctx());
                    cx.notify();
                })),
        );
    }
    column.into_any_element()
}
