//! The session pane (`Sessions::session_pane`): the title line with the note's state, Details and
//! the record control; the record line; Last time; the tabs; and the shown tab's body: the plan
//! and the note side by side, the plan, the note, the recordings, or a conflict's choice. The
//! page's own results and notices sit in the pane's footer (`session_messages`).
use super::{
    PLAN_BESIDE_NOTE_FROM, PLAN_MAX_WIDTH, PLAN_WIDTH, TWO_COLUMNS_FROM, footer, with_plan,
};
use crate::gui::{SplatRoot, icons, kit, theme};
use crate::sessions::page::gpui_facade::{RecordControl, RecordLine};
use gpui::{
    AnyElement, Context, IntoElement, ParentElement, Pixels, Styled, Window, div, prelude::*, px,
};
use std::rc::Rc;
use theme::{Density, L, LIST_MEASURE, READER_MEASURE, S, Type, XS, colors};

/// What the pane shows, worked out before anything draws.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Shown {
    Plan,
    Note,
    Recordings,
}

/// The session pane's footer press: the page's `session_press`.
pub(super) fn on_session_press() -> footer::OnPress {
    Rc::new(|root, press, _, _| {
        with_plan(&mut root.app, |sessions, _, book, plan| {
            sessions.gv_session_press(press, book, plan);
        });
    })
}

pub(super) fn render(
    root: &mut SplatRoot,
    width: Pixels,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    if root.app.sessions.gv_selected_note().is_none() {
        return gone(root, window, cx);
    }
    let note = root
        .app
        .sessions
        .gv_selected_note()
        .cloned()
        .expect("a selected note");
    let index = root.app.lookup.index();
    let conflict = root.app.sessions.gv_conflict();
    let editing_group_here = with_plan(&mut root.app, |sessions, _, _, plan| {
        sessions.gv_editing_group_here(plan)
    })
    .unwrap_or(false);
    let beside_from = if editing_group_here {
        LIST_MEASURE + L + READER_MEASURE
    } else {
        PLAN_BESIDE_NOTE_FROM
    };
    let beside = width >= beside_from;
    let tab = match root.app.sessions.gv_tab() {
        0 => Shown::Plan,
        1 => Shown::Note,
        _ => Shown::Recordings,
    };
    let shown = if beside && tab == Shown::Note {
        Shown::Plan
    } else {
        tab
    };
    let title_on_own_line = width < TWO_COLUMNS_FROM;
    let c = colors();

    // The title line: the title and its date, then the note's state, Details and the record
    // control at the right.
    let settings_blocked;
    let record = {
        let control = root.app.sessions.gv_record_control(&root.app.processing);
        settings_blocked = match &control {
            RecordControl::Record { blocked } => blocked.clone(),
            _ => None,
        };
        record_control(control, cx)
    };
    let title_details = (!title_on_own_line || conflict).then(|| details(root, cx));
    let title = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_baseline()
        .gap_x(S)
        .flex_1()
        .min_w_0()
        .child(kit::text(Type::Title, c.text, note.label()))
        .child(kit::text(Type::Small, c.muted, note.date.clone()));
    let title_line = div()
        .flex()
        .flex_row()
        .items_center()
        .min_h(Density::Scan.control())
        .gap(L)
        .child(title)
        .children(title_details)
        .children(record);
    let blocked_line = settings_blocked.map(|reason| {
        div()
            .flex()
            .justify_end()
            .pb(L)
            .child(kit::text(Type::Small, c.muted, reason))
    });
    let record_line = record_line(root, cx);
    let last_time = last_time(root, cx);

    // The tabs, the Read toggle and (narrow) Details.
    let count = root.app.sessions.gv_recording_count();
    let recordings = if count == 0 {
        "Recordings".to_owned()
    } else {
        format!("Recordings {count}")
    };
    let (options, chosen): (Vec<String>, usize) = if beside {
        (
            vec!["Plan and notes".to_owned(), recordings],
            usize::from(shown == Shown::Recordings),
        )
    } else {
        (
            vec!["Plan".to_owned(), "Note".to_owned(), recordings],
            match shown {
                Shown::Plan => 0,
                Shown::Note => 1,
                Shown::Recordings => 2,
            },
        )
    };
    let tabs = kit::Segmented::new("session-tabs", options, chosen).on_select(cx.listener(
        move |root, at: &usize, _, cx| {
            let tab = match (beside, *at) {
                (true, 0) | (false, 0) => 0,
                (false, 1) => 1,
                _ => 2,
            };
            root.app.sessions.gv_set_tab(tab);
            cx.notify();
        },
    ));
    let offers_read = (shown == Shown::Note || (shown == Shown::Plan && beside))
        && with_plan(&mut root.app, |sessions, _, _, plan| {
            sessions.gv_offers_read(plan, index.as_deref(), beside)
        })
        .unwrap_or(false);
    let read = offers_read.then(|| {
        let (word, icon) = if root.app.sessions.gv_reading_note() {
            ("Write", icons::EDIT)
        } else {
            ("Read", icons::DOCUMENT)
        };
        kit::Button::quiet("session-read-toggle", word)
            .icon(icon)
            .density(Density::Read)
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.sessions.gv_toggle_reading();
                cx.notify();
            }))
    });
    let narrow_details = title_on_own_line.then(|| details(root, cx));
    let tab_row = (!conflict).then(|| {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .child(div().flex_1().child(tabs))
            .children(read)
            .children(narrow_details)
    });

    // The body, and the footer that holds the page's messages.
    let plan_footer = shown == Shown::Plan && !beside && !conflict;
    let body = if conflict {
        super::note::conflict(root, window, cx)
    } else {
        match shown {
            Shown::Plan if beside => {
                let plan_width = if editing_group_here {
                    LIST_MEASURE
                } else {
                    (width * 0.4).clamp(PLAN_WIDTH, PLAN_MAX_WIDTH)
                };
                let plan_focus = super::note::plan_focus(root, cx);
                let plan = div()
                    .size_full()
                    .track_focus(&plan_focus)
                    .child(super::plan::render(root, false, window, cx));
                let note = super::note::render(root, beside, window, cx);
                kit::Columns::new("session-plan-note")
                    .column(plan_width, PLAN_WIDTH..px(4000.0), plan)
                    .rest(div().size_full().pl(L).child(note))
                    .into_any_element()
            }
            Shown::Plan => {
                let plan_focus = super::note::plan_focus(root, cx);
                div()
                    .size_full()
                    .track_focus(&plan_focus)
                    .child(super::plan::render(root, true, window, cx))
                    .into_any_element()
            }
            Shown::Note => super::note::render(root, beside, window, cx),
            Shown::Recordings => super::recordings::render(root, window, cx),
        }
    };
    let footer_lines = if plan_footer {
        Vec::new()
    } else {
        with_plan(&mut root.app, |sessions, settings, book, plan| {
            sessions.gv_session_footer(settings, book, plan)
        })
        .unwrap_or_default()
    };
    let footer = footer::lines("session-footer", footer_lines, on_session_press(), cx);
    div()
        .flex()
        .flex_col()
        .size_full()
        .child(title_line)
        .children(blocked_line)
        .children(record_line)
        .children(last_time)
        .children(tab_row)
        .child(kit::gap(XS))
        // Rows of recordings end where their content does, and the footer follows them `S`
        // below (pinned only when the rows fill the pane); the plan and note fill the pane.
        .child(if shown == Shown::Recordings && !conflict {
            div().flex_shrink(1.0).min_h_0().child(body)
        } else {
            div().flex_1().min_h_0().child(body)
        })
        .when(!footer.is_empty(), |pane| {
            pane.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(S)
                    .pt(S)
                    .border_t(theme::HAIRLINE_WIDTH)
                    .border_color(c.hairline)
                    .children(footer),
            )
        })
        .into_any_element()
}

/// `Details`, with the note's state before it.
fn details(root: &SplatRoot, cx: &mut Context<SplatRoot>) -> AnyElement {
    let status = root
        .app
        .sessions
        .gv_note_status()
        .map(|status| kit::status(&kit::Status::of(&status)));
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(L)
        .children(status)
        .child(
            kit::Button::quiet("session-details", "Details")
                .name("Edit session details")
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.sessions.gv_edit_details();
                    cx.notify();
                })),
        )
        .into_any_element()
}

/// The record control: `Record` or `Stop recording`.
pub(super) fn record_control(
    control: RecordControl,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    match control {
        RecordControl::None => None,
        RecordControl::Stop { name } => Some(
            kit::Button::secondary("session-stop-recording", "Stop recording")
                .icon(icons::STOP_RECORDING)
                .name(name.clone())
                .hint(name)
                .on_click(cx.listener(|root, _, _, cx| {
                    let ctx = root.ctx.clone();
                    root.app.sessions.gv_stop_recording(&ctx);
                    cx.notify();
                }))
                .into_any_element(),
        ),
        RecordControl::Record { blocked } => Some(
            kit::Button::secondary("session-record", "Record")
                .icon(icons::RECORD)
                .name("Record this session")
                .blocked(blocked)
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.sessions.gv_ask_to_record();
                    cx.notify();
                }))
                .into_any_element(),
        ),
    }
}

/// The live recording's state, or the step before it.
pub(super) fn record_line(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> Option<AnyElement> {
    let c = colors();
    match root.app.sessions.gv_record_line() {
        RecordLine::None => None,
        RecordLine::Live {
            status,
            words,
            detail,
            silent,
        } => {
            let check = silent.then(|| {
                kit::Button::quiet("session-check-input", "Check the input in Settings")
                    .density(Density::Read)
                    .on_click(cx.listener(|root, _, _, cx| {
                        root.app.sessions.wants_processing_settings = true;
                        cx.notify();
                    }))
            });
            Some(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(S)
                    .min_h(Density::Read.control() + XS)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(XS)
                            .child(kit::icon(
                                status.icon,
                                Type::Strong,
                                theme::color(status.icon_color()),
                            ))
                            .child(kit::text(
                                Type::Strong,
                                theme::color(status.word_color()),
                                words,
                            )),
                    )
                    .child(kit::text(Type::Small, c.muted, detail))
                    .children(check)
                    .into_any_element(),
            )
        }
        RecordLine::Confirm {
            words,
            tone,
            play_this_one,
            start_primary,
        } => {
            let mut message = kit::Message::new(footer::tone(tone), words);
            if play_this_one {
                message = message.action(
                    kit::Button::secondary("record-play-this-one", "Play this one")
                        .density(Density::Read)
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.sessions.gv_play_this_one();
                            cx.notify();
                        })),
                );
            }
            let start = if start_primary {
                kit::Button::primary("record-start", "Start recording")
            } else {
                kit::Button::secondary("record-start", "Start recording")
            };
            message = message
                .action(
                    start
                        .density(Density::Read)
                        .on_click(cx.listener(|root, _, _, cx| {
                            let app = &mut root.app;
                            app.sessions.gv_start_recording(&app.processing);
                            cx.notify();
                        })),
                )
                .action(
                    kit::Button::secondary("record-not-now", "Not now")
                        .density(Density::Read)
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.sessions.gv_not_now();
                            cx.notify();
                        })),
                );
            Some(div().pb(S).child(message).into_any_element())
        }
    }
}

/// `Last time: <link> — <first line>`, and `Carry over` while this session is prepared.
fn last_time(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> Option<AnyElement> {
    let c = colors();
    let last = with_plan(&mut root.app, |sessions, _, book, plan| {
        sessions.gv_last_time(book, plan)
    })??;
    let id = last.id;
    let link = kit::Link::new("last-time-link", last.label.clone())
        .wrap()
        .hint(last.full.clone())
        .on_click(cx.listener(move |root, _, _, cx| {
            root.app.sessions.gv_open_session(id);
            cx.notify();
        }));
    let carry = last.carry.map(|carry| {
        let mut button = kit::Button::quiet("carry-over", "Carry over").density(Density::Read);
        match carry {
            Ok(hover) => button = button.hint(hover),
            Err(reason) => button = button.blocked(Some(reason)),
        }
        button.on_click(cx.listener(move |root, _, _, cx| {
            with_plan(&mut root.app, |sessions, _, book, plan| {
                sessions.gv_carry_over(id, plan, book);
            });
            cx.notify();
        }))
    });
    // The link keeps its words; the preview gives way, cut at the line's end.
    // With nothing written last time there is no preview, so no dash (egui: the link alone).
    let preview = (!last.preview.is_empty()).then(|| {
        kit::text::one_line(Type::Small, c.muted, format!("— {}", last.preview))
            .max_w(READER_MEASURE)
            .flex_1()
            .min_w_0()
            .overflow_hidden()
    });
    Some(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(XS)
            .min_w_0()
            .child(kit::text(Type::Small, c.muted, "Last time:").flex_none())
            .child(div().flex_none().child(link))
            .children(preview)
            .children(carry.map(|carry| div().pl(L - XS).child(carry)))
            .into_any_element(),
    )
}

/// The open note's file went from the folder while its words are still here: the same header
/// as a note (the file's name and the draft's state), the record line, and the editor with the
/// draft's choices in its footer.
fn gone(root: &mut SplatRoot, window: &mut Window, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let control = root.app.sessions.gv_record_control(&root.app.processing);
    let record = record_control(control, cx);
    let name = root.app.sessions.gv_draft_name();
    let status = root
        .app
        .sessions
        .gv_note_status()
        .map(|status| kit::status(&kit::Status::of(&status)));
    let title = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(L)
        .flex_1()
        .children(name.map(|name| kit::text(Type::Title, c.text, name)))
        .children(status);
    let record_line = record_line(root, cx);
    let has_draft = root.app.sessions.gv_has_draft();
    let editor = has_draft.then(|| super::note::render(root, false, window, cx));
    let lines = if has_draft {
        root.app.sessions.gv_draft_footer(false)
    } else {
        Vec::new()
    };
    let footer = footer::lines(
        "session-gone-footer",
        lines,
        Rc::new(|root, press, _, _| root.app.sessions.gv_draft_press(press, false)),
        cx,
    );
    div()
        .flex()
        .flex_col()
        .size_full()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .min_h(Density::Scan.control())
                .child(title)
                .children(record),
        )
        .children(record_line)
        .child(div().flex_1().min_h_0().children(editor))
        .when(!footer.is_empty(), |pane| {
            pane.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(S)
                    .pt(S)
                    .border_t(theme::HAIRLINE_WIDTH)
                    .border_color(c.hairline)
                    .children(footer),
            )
        })
        .into_any_element()
}
