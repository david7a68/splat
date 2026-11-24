//! The run's footers and second steps:
//! the live footer's Close run, End encounter…, Sync party, Keys and the save state; the end
//! step's outcome field; the close dialog; and the message footer under each body.
use super::{on_run, save_status};
use crate::encounter_run::{Command, NO_OUTCOME, Panel, PartySync, Run, view};
use crate::gui::{SplatRoot, icons, kit, theme};
use gpui::{
    AnyElement, ClickEvent, Context, IntoElement, ParentElement, Styled, Window, div, prelude::*,
};
use splat_party::combat_sync::SyncState;
use theme::{Density, Type, colors};

/// The words of Close run: while sync must settle first, `Close after sync`.
fn close_word(run: &Run) -> &'static str {
    if run.close_block().is_some() {
        "Close after sync"
    } else {
        "Close run"
    }
}

/// `Close run`: closes now, or opens the close dialog when something would be lost.
pub(super) fn close_button(
    run: &Run,
    id: &'static str,
    primary: bool,
    cx: &mut Context<SplatRoot>,
) -> kit::Button {
    let button = if primary {
        kit::Button::primary(id, close_word(run)).density(Density::Glance)
    } else {
        kit::Button::quiet(id, close_word(run)).density(Density::Read)
    };
    button
        .name("Close run")
        .blocked(run.close_block())
        .on_click(cx.listener(|root: &mut SplatRoot, _: &ClickEvent, _, cx| {
            let closed = root
                .app
                .encounters
                .run_mut()
                .is_some_and(|run| run.request_close());
            if closed {
                super::closed(root, cx);
            }
            cx.notify();
        }))
}

/// The live footer: Close run and End encounter… on the left; Sync party, Keys and the save
/// state on the right. While the DM ends the fight, the outcome field, End encounter and Keep
/// playing take the line.
pub(super) fn live(run: &Run, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let line = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::XS)
        .flex_none()
        .h(gpui::px(super::live::FOOTER_HEIGHT))
        .pt(theme::XS);
    if run.ending {
        return line.child(end_step(run, cx));
    }
    let end = kit::Button::quiet("run-end-encounter", "End encounter…")
        .density(Density::Read)
        .hint("End encounter… Set an outcome. You can undo this.")
        .on_click(on_run(cx, |run| {
            if run.outcome.is_empty()
                && let Some(outcome) = run.outcome_before_continue()
            {
                run.outcome = outcome.to_owned();
            }
            run.ending = true;
            run.focus_outcome = true;
        }));
    let keys = kit::Button::quiet("run-keys", "Keys")
        .icon(icons::KEYS)
        .density(Density::Read)
        .name("Keyboard commands")
        .selected(run.panel == Panel::Keys)
        .on_click(on_run(cx, |run| {
            run.panel = if run.panel == Panel::Keys && !run.hide_panel {
                Panel::Reference
            } else {
                Panel::Keys
            };
            run.close_lookup = true;
        }));
    line.child(close_button(run, "run-close", false, cx))
        .child(end)
        .child(div().flex_1())
        .children(sync_party(run, cx))
        .child(keys)
        .child(div().w(theme::S).flex_none())
        .child(save_status(run))
}

/// `Sync party` or `Stop sync`, while the run has linked characters (`party_sync_action`).
pub(super) fn sync_party(run: &Run, cx: &mut Context<SplatRoot>) -> Option<kit::Button> {
    if run.linked_participants().is_empty() {
        return None;
    }
    let stopping = matches!(run.party_sync_plan(), PartySync::Stop(_));
    Some(
        kit::Button::quiet(
            "run-sync-party",
            if stopping { "Stop sync" } else { "Sync party" },
        )
        .icon(icons::SYNC_ON)
        .density(Density::Read)
        .name(if stopping {
            "Stop party sync"
        } else {
            "Sync party"
        })
        .hint("Read every linked character’s sheet. A value you changed stays, and you choose.")
        .blocked(
            (!run.connected && !run.simulate_connection)
                .then_some("Connect to D&D Beyond in Settings to sync."),
        )
        .on_click(on_run(cx, |run| {
            run.toggle_party_sync();
            run.return_focus = run.lookup_open;
        })),
    )
}

/// End encounter's second step: the outcome, End encounter and Keep playing.
fn end_step(run: &Run, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let end = move |root: &mut SplatRoot, cx: &mut Context<SplatRoot>| {
        if let Some(run) = root.app.encounters.run_mut() {
            let outcome = if run.outcome.trim().is_empty() {
                NO_OUTCOME.to_owned()
            } else {
                run.outcome.trim().to_owned()
            };
            run.ending = false;
            run.run_command(Command::End(outcome));
        }
        cx.notify();
    };
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::XS)
        .child(
            div()
                .key_context("RunField")
                .w(gpui::px(160.0))
                .flex_none()
                .child(
                    kit::TextField::new(
                        ("run-outcome", run.id().as_u128() as u64),
                        run.outcome.clone(),
                    )
                    .placeholder("Outcome, if any")
                    .name("Outcome")
                    .focus(run.focus_outcome)
                    .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
                        if let Some(run) = root.app.encounters.run_mut() {
                            run.outcome = words.to_owned();
                            run.focus_outcome = false;
                        }
                        cx.notify();
                    }))
                    .on_enter(super::plain(cx, move |root, _, cx| end(root, cx))),
                ),
        )
        .child(
            kit::Button::destructive("run-end-confirm", "End encounter")
                .density(Density::Read)
                .on_click(
                    cx.listener(move |root: &mut SplatRoot, _: &ClickEvent, _, cx| end(root, cx)),
                ),
        )
        .child(
            kit::Button::quiet("run-keep-playing", "Keep playing")
                .density(Density::Read)
                .on_click(on_run(cx, |run| {
                    run.ending = false;
                    run.return_focus = run.lookup_open;
                })),
        )
}

/// The footer messages under a body (`run_messages`, the footer's typed lines), drawn by
/// `kit::messages`; a press goes through `run_press` with its ticket.
pub(super) fn messages(run: &Run, setup: bool, cx: &mut Context<SplatRoot>) -> Vec<AnyElement> {
    let frame = crate::design::outcome::PaneFrame::new();
    let lines = run.run_messages(&frame, setup).into_plain();
    kit::messages::lines(
        "run-messages",
        lines,
        cx.listener(
            |root: &mut SplatRoot, pressed: &kit::messages::Pressed<view::RunPress>, _, cx| {
                if let Some(press) = pressed.take()
                    && let Some(run) = root.app.encounters.run_mut()
                {
                    let mut command = None;
                    run.run_press(Some(press), &mut command);
                    if let Some(command) = command {
                        run.run_command(command);
                    }
                }
                cx.notify();
            },
        ),
    )
}

/// The close dialog, while it is open (`close_dialog`): what closing would lose, the ways to
/// recover, and the choices, safe first.
pub(super) fn close_dialog(
    root: &mut SplatRoot,
    _: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let c = colors();
    let run = root.app.encounters.run()?;
    if !run.closing.is_open() {
        return None;
    }
    let cannot_save = run.strip().not_saved;
    let stranded = run.stranded_names();
    let lost = run.unsent_only_here_names();
    let block = run.close_reason();
    let conflict = run.first_sync_in(SyncState::Conflict);
    let signed_out = run.sync_needs_sign_in() && !stranded.is_empty();
    let title = if cannot_save {
        "This run is not saved".to_owned()
    } else if let Some(index) = conflict {
        format!("{} has a sync conflict", run.state.participants[index].name)
    } else if signed_out {
        "Changes not sent to D&D Beyond".to_owned()
    } else if !stranded.is_empty() || block.is_some() {
        "Sync has not finished".to_owned()
    } else {
        "Close this run?".to_owned()
    };
    let mut words: Vec<String> = Vec::new();
    if cannot_save {
        let changes = run.unsaved_summary();
        words.push(format!(
            "{}Closing loses {}:",
            if run.has_disk_conflict() {
                "The file changed on disk, so saving stopped. "
            } else {
                ""
            },
            if changes.len() == 1 {
                "this change"
            } else {
                "these changes"
            }
        ));
        words.extend(changes.iter().take(3).cloned());
        if changes.len() > 3 {
            words.push(format!("and {} more", changes.len() - 3));
        }
    } else if conflict.is_some() {
        words.extend(view::conflict_close_words(block.is_some()).map(str::to_owned));
    } else if stranded.is_empty() {
        words.push(run.close_note());
    } else {
        words.push(format!(
            "Changes for {} were not sent to D&D Beyond. The saved run keeps HP, death saves and conditions for the next sync.",
            stranded.join(", ")
        ));
    }
    if !lost.is_empty() {
        words.push(format!(
            "Spell slots and limited uses not sent for {} are lost.",
            lost.join(", ")
        ));
    }
    let keep = |root: &mut SplatRoot| {
        if let Some(run) = root.app.encounters.run_mut() {
            run.closing.shut();
        }
    };
    let mut recovery = div().flex().flex_row().gap(theme::S);
    let mut any_recovery = false;
    if signed_out {
        any_recovery = true;
        recovery = recovery.child(
            kit::Button::secondary("run-close-connect", "Connect to sync").on_click(cx.listener(
                move |root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                    if let Some(run) = root.app.encounters.run_mut() {
                        run.wants_sign_in = true;
                    }
                    keep(root);
                    cx.notify();
                },
            )),
        );
    }
    if conflict.is_some() {
        any_recovery = true;
        recovery = recovery.child(
            kit::Button::secondary("run-close-review", "Review conflict").on_click(cx.listener(
                move |root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                    if let Some(run) = root.app.encounters.run_mut() {
                        run.review_conflict();
                        run.panel = Panel::Reference;
                    }
                    keep(root);
                    cx.notify();
                },
            )),
        );
    }
    let mut dialog = kit::Dialog::new("run-close-dialog", title)
        .on_escape(super::plain(cx, move |root, _, cx| {
            keep(root);
            cx.notify();
        }))
        .children(
            words
                .into_iter()
                .map(|words| kit::text(Type::Body, c.text, words)),
        )
        .when(any_recovery, |dialog| {
            dialog.child(kit::gap(theme::S)).child(recovery)
        })
        .children(
            block
                .clone()
                .map(|reason| kit::text(Type::Small, c.muted, reason)),
        );
    if cannot_save || !lost.is_empty() {
        dialog = dialog.destructive(
            kit::Button::destructive("run-close-lose", "Close and lose changes")
                .blocked(block.clone())
                .on_click(
                    cx.listener(move |root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                        let closed = root
                            .app
                            .encounters
                            .run_mut()
                            .is_some_and(|run| cannot_save || run.close_run());
                        if closed {
                            super::closed(root, cx);
                        }
                        cx.notify();
                    }),
                ),
        );
    }
    let keep_running = || {
        kit::Button::secondary("run-close-keep", "Keep running").on_click(cx.listener(
            move |root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                keep(root);
                cx.notify();
            },
        ))
    };
    if cannot_save {
        // The safe choice before the commit.
        dialog = dialog.action(keep_running());
        dialog = dialog.action(
            kit::Button::primary("run-close-save-new", "Save as a new run and close")
                .blocked(block.clone())
                .on_click(cx.listener(|root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                    let closed = root.app.encounters.run_mut().is_some_and(|run| {
                        match run.save_as_new_run() {
                            Ok(()) => run.close_run(),
                            Err(error) => {
                                run.error = Some(error);
                                false
                            }
                        }
                    });
                    if closed {
                        super::closed(root, cx);
                    }
                    cx.notify();
                })),
        );
    } else if lost.is_empty() {
        dialog = dialog.action(
            kit::Button::secondary("run-close-now", "Close now")
                .blocked(block.clone())
                .on_click(cx.listener(|root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                    let closed = root
                        .app
                        .encounters
                        .run_mut()
                        .is_some_and(|run| run.close_run());
                    if closed {
                        super::closed(root, cx);
                    }
                    cx.notify();
                })),
        );
    }
    if !cannot_save {
        dialog = dialog.action(keep_running());
    }
    Some(dialog.into_any_element())
}
