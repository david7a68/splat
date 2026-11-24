//! The close prompt (`Splat::close_prompt`): what closing would lose, by name, and the ways on.
//! Its words and choices: the title and actions depend on whether only a
//! recording or processing runs, only the run could not be saved, or only sync has not finished.
use super::{SplatRoot, kit, theme};
use crate::{Page, SettingsTab};
use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, Window, div};
use theme::{S, Type, colors};

/// Draw the prompt, or end it when nothing is left to lose.
pub fn prompt(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let app = &mut root.app;
    let items = app.unsaved_items();
    // Sync finished, or a draft was saved, while the prompt was open.
    if items.is_empty() {
        app.confirm_close = false;
        cx.notify();
        return div().into_any_element();
    }
    let only_sync = items.len() == 1 && !app.encounters.pending_sync_names().is_empty();
    let conflict = app.encounters.run().and_then(|run| run.strip().conflict);
    let only_run_unsaved = items.len() == 1 && app.encounters.run_not_saved().is_some();
    let sync_loss = app
        .encounters
        .run()
        .is_some_and(|run| !run.unsent_only_here_names().is_empty());
    let background = usize::from(app.sessions.recording()) + usize::from(app.sessions.processing());
    let signed_out = app.settings.signed_out();
    let (title, keep_label, discard_label, advice) = if items.len() == background {
        (
            if app.sessions.recording() {
                "Recording has not finished"
            } else {
                "Processing has not finished"
            },
            if app.sessions.recording() {
                "Keep recording"
            } else {
                "Keep processing"
            },
            "Stop and close",
            "",
        )
    } else if only_run_unsaved {
        (
            "This run could not be saved",
            "Keep running",
            "Close and lose changes",
            "Closing loses changes since the last save.",
        )
    } else if only_sync {
        (
            if conflict.is_some() {
                "Sync conflict"
            } else if signed_out {
                "Changes not sent to D&D Beyond"
            } else {
                "Sync has not finished"
            },
            "Keep running",
            if sync_loss {
                "Close and lose changes"
            } else {
                "Close now"
            },
            if conflict.is_some() {
                "Choose which HP to keep, or close and choose next time."
            } else if signed_out {
                "Connect to send these changes. Closing keeps the saved run."
            } else {
                "Keep running to finish sync. Closing keeps the saved run."
            },
        )
    } else {
        (
            "Unsaved changes",
            "Keep editing",
            "Discard and close",
            "Save and close, keep editing, or close and lose these changes.",
        )
    };
    let title = if items.len() == background {
        app.sessions
            .recording_label()
            .map(|label| format!("“{label}” is recording"))
            .or_else(|| {
                app.sessions
                    .processing_label()
                    .map(|label| format!("“{label}” is processing"))
            })
            .unwrap_or_else(|| title.to_owned())
    } else if only_sync && let Some(name) = &conflict {
        format!("{name} has a sync conflict")
    } else {
        title.to_owned()
    };
    let loses = items.len() != background && (!only_sync || sync_loss);
    let c = colors();
    let mut dialog = kit::Dialog::new("unsaved-on-close", title);
    if !only_sync || conflict.is_none() {
        for (item, _) in &items {
            dialog = dialog.child(kit::text(Type::Body, c.text, item.clone()));
        }
    }
    if !advice.is_empty() {
        dialog = dialog.child(kit::text(Type::Small, c.muted, advice));
    }
    // The recovery actions above the line (`close_recovery_actions`): Connect to sync before
    // Review conflict.
    let (review, connect) = (only_sync && conflict.is_some(), only_sync && signed_out);
    if review || connect {
        let mut recovery = div().flex().flex_row().gap(S).pt(S);
        if connect {
            recovery = recovery.child(
                kit::Button::secondary("close-connect", "Connect to sync").on_click(cx.listener(
                    |root, _, _, cx| {
                        root.app.page = Page::Settings;
                        root.app.settings_tab = SettingsTab::default();
                        root.app.settings.focus_sign_in();
                        root.app.confirm_close = false;
                        cx.notify();
                    },
                )),
            );
        }
        if review {
            recovery = recovery.child(
                kit::Button::secondary("close-review", "Review conflict").on_click(cx.listener(
                    |root, _, _, cx| {
                        if let Some(run) = root.app.encounters.run_mut() {
                            run.review_conflict();
                        }
                        root.app.lookup.open = false;
                        root.app.sessions.notes_open = false;
                        root.app.page = Page::Run;
                        root.app.confirm_close = false;
                        cx.notify();
                    },
                )),
            );
        }
        dialog = dialog.child(recovery);
    }
    let keep = cx.listener(move |root, _, _, cx| keep_editing(root, background, cx));
    let keep_on_escape = cx.listener(move |root, _: &(), _, cx| keep_editing(root, background, cx));
    dialog = dialog.on_escape(move |window, cx| keep_on_escape(&(), window, cx));
    if loses {
        dialog = dialog.destructive(
            kit::Button::destructive("close-discard", discard_label)
                .on_click(cx.listener(|root, _, window, cx| discard_and_close(root, window, cx))),
        );
    }
    dialog = dialog.action(kit::Button::secondary("close-keep", keep_label).on_click(keep));
    if only_run_unsaved {
        dialog = dialog.action(
            kit::Button::primary("close-save-new", "Save as a new run and close").on_click(
                cx.listener(|root, _, window, cx| {
                    let saved = root
                        .app
                        .encounters
                        .run_mut()
                        .map(|run| run.save_as_new_run());
                    match saved {
                        Some(Ok(())) => discard_and_close(root, window, cx),
                        _ => {
                            root.app.confirm_close = false;
                            root.app.page = Page::Run;
                            cx.notify();
                        }
                    }
                }),
            ),
        );
    } else if items.len() == 1 && app.library.can_save_and_close() {
        dialog = dialog.action(
            kit::Button::primary("close-save", "Save and close").on_click(cx.listener(
                |root, _, window, cx| {
                    if root.app.library.save_for_close() {
                        root.app.confirm_close = false;
                        window.remove_window();
                    }
                    cx.notify();
                },
            )),
        );
    } else if !loses {
        dialog = dialog.action(
            kit::Button::secondary("close-discard", discard_label)
                .on_click(cx.listener(|root, _, window, cx| discard_and_close(root, window, cx))),
        );
    }
    let _ = window;
    dialog.into_any_element()
}

/// Keep editing: the prompt closes, and the first thing named is where the DM goes.
fn keep_editing(root: &mut SplatRoot, background: usize, cx: &mut Context<SplatRoot>) {
    let app = &mut root.app;
    let items = app.unsaved_items();
    app.confirm_close = false;
    if items.len() != background
        && let Some((_, page)) = items.first()
    {
        app.page = *page;
    }
    // Lookup is a panel, not a page, so it opens beside the page on its draft.
    if let Some((id, _)) = app.lookup.reader.unsaved_edits().first() {
        app.lookup.open_id(*id);
    }
    match app.page {
        Page::Library => app.library.refocus_draft(),
        Page::Sessions => app.sessions.return_focus_to_note(),
        Page::Encounters => app.encounters.refocus_draft(),
        _ => {}
    }
    cx.notify();
}

/// Close and lose what is unsaved.
fn discard_and_close(root: &mut SplatRoot, window: &mut Window, cx: &mut Context<SplatRoot>) {
    root.app.discard_on_close = true;
    cx.notify();
    window.remove_window();
}
