//! The session's recordings (`Sessions::recordings_tab`): one row per recording with its size,
//! state, live lines, results and actions, its marked events, and the text being read
//! (`reading_view`); and the dialog that deletes a recording (`delete_dialog`).
use crate::design::outcome::plain::ChoiceForm as Kind;
use crate::gui::{SplatRoot, icons, kit, theme};
use crate::sessions::page::gpui_facade::{ReadingView, RecordingAct, RecordingRow};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, SharedString, Styled, Window, div,
    prelude::*,
};
use std::path::PathBuf;
use theme::{Density, L, LIST_MEASURE, READER_MEASURE, S, Type, XS, colors};

fn id(words: impl std::fmt::Display) -> ElementId {
    ElementId::Name(SharedString::from(words.to_string()))
}

fn on(
    cx: &mut Context<SplatRoot>,
    path: PathBuf,
    act: RecordingAct,
) -> impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static {
    cx.listener(move |root, _, _, cx| {
        let ctx = root.ctx.clone();
        let app = &mut root.app;
        app.sessions
            .gv_recording_act(&path, act.clone(), &app.processing, &mut app.timeline, &ctx);
        cx.notify();
    })
}

pub(super) fn render(
    root: &mut SplatRoot,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    if root.app.sessions.gv_reading() {
        return reading(root, cx);
    }
    if let Some((words, settings)) = root.app.sessions.gv_recordings_empty(&root.app.processing) {
        let action = settings.then(|| {
            kit::Button::quiet("recordings-open-settings", "Open Settings").on_click(cx.listener(
                |root, _, _, cx| {
                    root.app.sessions.gv_open_settings();
                    cx.notify();
                },
            ))
        });
        return div()
            .pt(theme::M)
            .child(kit::empty_state(
                icons::RECORD,
                "Session recordings",
                words,
                action,
            ))
            .into_any_element();
    }
    let app = &mut root.app;
    let rows = app
        .sessions
        .gv_recording_rows(&app.processing, &app.timeline);
    let rows: Vec<AnyElement> = rows.into_iter().map(|row| self::row(row, cx)).collect();
    kit::scroll_column(
        "session-recordings",
        div()
            .flex()
            .flex_col()
            .max_w(LIST_MEASURE)
            .pr(theme::M)
            .children(rows),
    )
    .into_any_element()
}

fn row(row: RecordingRow, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let RecordingRow {
        path,
        title,
        name,
        size,
        status,
        processing,
        live_line,
        sentences,
        views,
        process_again,
        delete_blocked,
        actions,
        reason_link,
        destination,
        confirm_again,
        markers,
    } = row;
    let key = path.display().to_string();
    let mut menu = kit::MenuButton::new(
        id(format!("recording-menu-{key}")),
        kit::Button::quiet(id(format!("recording-menu-button-{key}")), "")
            .icon(icons::EXPAND)
            .name(format!("More actions for {title}"))
            .density(Density::Read),
    );
    if let Some(blocked) = process_again {
        menu = menu.item(
            kit::MenuItem::new("Process again…")
                .blocked(blocked)
                .on_click(on(cx, path.clone(), RecordingAct::ProcessAgain)),
        );
    }
    menu = menu.item(
        kit::MenuItem::new("Delete…")
            .blocked(delete_blocked)
            .on_click(on(cx, path.clone(), RecordingAct::Delete)),
    );
    let head = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(L)
        .min_h(Density::Read.control())
        .child(kit::text(Type::Strong, c.text, title.clone()))
        .child(menu);
    let mut column = div()
        .flex()
        .flex_col()
        .gap(XS)
        .pt(S)
        .child(head)
        .child(kit::text(Type::Small, c.muted, size));
    if let Some(processing) = processing {
        column = column.child(kit::text(Type::Small, c.muted, processing.words));
        column = column.child(if processing.cancelling {
            kit::text(
                Type::Small,
                c.muted,
                "Cancelling. This ends after the current step.",
            )
            .into_any_element()
        } else {
            kit::Button::quiet(id(format!("recording-cancel-{key}")), "Cancel")
                .name("Cancel processing")
                .density(Density::Read)
                .line_start()
                .on_click(on(cx, path.clone(), RecordingAct::Cancel))
                .into_any_element()
        });
    } else if let Some(status) = status {
        column = column.child(
            kit::Status::new(status.icon, status.word, theme::color(status.word_color()))
                .icon_color(theme::color(status.icon_color()))
                .into_element(),
        );
    }
    if let (Some((time, sound)), Some(status)) = (live_line, status) {
        column = column.child(
            div()
                .flex()
                .flex_row()
                .child(kit::text(
                    Type::Small,
                    theme::color(status.word_color()),
                    time,
                ))
                .child(kit::text(Type::Small, c.muted, sound)),
        );
    }
    for (at, (words, hover)) in sentences.into_iter().enumerate() {
        let sentence = kit::text(Type::Small, c.muted, words).max_w(READER_MEASURE);
        column = column.child(match hover {
            Some(hover) => div()
                .id(id(format!("recording-sentence-{key}-{at}")))
                .child(sentence)
                .with_tooltip(hover)
                .into_any_element(),
            None => sentence.into_any_element(),
        });
    }
    if !views.is_empty() {
        let mut links = div().flex().flex_row().flex_wrap().gap(S);
        for (made, words) in views {
            links = links.child(
                kit::Link::new(id(format!("recording-read-{key}-{made}")), words)
                    .hint(format!("Read this for {name}"))
                    .on_click(on(cx, path.clone(), RecordingAct::Read(made))),
            );
        }
        column = column.child(links);
    }
    if !actions.is_empty() || reason_link.is_some() {
        let mut line = div().flex().flex_row().flex_wrap().items_center().gap(S);
        for choice in actions {
            let element_id = id(format!("recording-{}-{key}", choice.words));
            let click = on(cx, path.clone(), choice.act.clone());
            let mut button = match choice.kind {
                Kind::Primary => kit::Button::primary(element_id, choice.words),
                Kind::Link => kit::Button::quiet(element_id, choice.words),
                _ => kit::Button::secondary(element_id, choice.words),
            }
            .name(format!("{}: {name}", choice.words))
            .density(Density::Read)
            .blocked(choice.blocked);
            if let Some(hover) = choice.hover {
                button = button.hint(hover);
            }
            line = line.child(button.on_click(click));
        }
        if let Some((words, key_needed)) = reason_link {
            line = line.child(
                kit::Link::new(id(format!("recording-reason-{key}")), words)
                    .small()
                    .on_click(on(cx, path.clone(), RecordingAct::OpenSettings(key_needed))),
            );
        }
        column = column.child(line);
    }
    if let Some(destination) = destination {
        column = column.child(kit::help(destination));
    }
    if let Some(words) = confirm_again {
        column = column.child(
            div().max_w(READER_MEASURE).child(
                kit::Message::new(kit::Tone::Info, words)
                    .action(
                        kit::Button::secondary(id(format!("recording-not-now-{key}")), "Not now")
                            .density(Density::Read)
                            .on_click(on(cx, path.clone(), RecordingAct::AgainNotNow)),
                    )
                    .destructive(
                        kit::Button::destructive(
                            id(format!("recording-replace-{key}")),
                            "Replace the results",
                        )
                        .density(Density::Read)
                        .on_click(on(
                            cx,
                            path.clone(),
                            RecordingAct::AgainReplace,
                        )),
                    ),
            ),
        );
    }
    let marked = !markers.is_empty();
    if marked {
        column = column
            .child(kit::gap(S))
            .child(kit::heading(
                kit::Heading::Group {
                    title: "Events marked in this recording",
                    count: markers.len(),
                },
                true,
            ))
            .child(kit::text(
                Type::Small,
                c.muted,
                "These events were suggested from the transcript. Only events you add go on the timeline.",
            ));
        for marker in markers {
            let index = marker.index;
            let mut facts = div().flex().flex_row().flex_wrap().items_center().gap(S);
            if let Some(at) = marker.at.clone() {
                facts = facts.child(
                    kit::Link::new(id(format!("marker-at-{key}-{index}")), at)
                        .hint("Read the transcript here")
                        .on_click(on(cx, path.clone(), RecordingAct::ReadMarker(index))),
                );
            }
            facts = facts.child(kit::text(Type::Small, c.muted, marker.facts.clone()));
            let action = match &marker.added {
                Some(event) => kit::status_link(
                    id(format!("marker-on-timeline-{key}-{index}")),
                    kit::Status::new(icons::OK, "On the timeline", c.result.success),
                )
                .on_click(on(cx, path.clone(), RecordingAct::OpenEvent(event.clone())))
                .into_any_element(),
                None => kit::Button::secondary(id(format!("marker-add-{key}-{index}")), "Add")
                    .name(format!("Add to timeline: {} ({index})", marker.title))
                    .density(Density::Read)
                    .on_click(on(cx, path.clone(), RecordingAct::AddMarker(index)))
                    .into_any_element(),
            };
            column = column.child(
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap(L)
                    .max_w(READER_MEASURE + L * 6.0)
                    .py(S)
                    .border_b(theme::HAIRLINE_WIDTH)
                    .border_color(c.hairline)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_start()
                            .gap(S)
                            .flex_1()
                            .min_w_0()
                            .child(kit::icon(marker.icon, Type::Body, c.muted))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .min_w_0()
                                    .child(kit::text(Type::Body, c.text, marker.title.clone()))
                                    .child(facts)
                                    .when(!marker.placement.is_empty(), |column| {
                                        column.child(kit::text(
                                            Type::Small,
                                            c.muted,
                                            marker.placement.clone(),
                                        ))
                                    }),
                            ),
                    )
                    .child(action),
            );
        }
    }
    column
        .pb(S)
        .when(!marked, |column| {
            column
                .border_b(theme::HAIRLINE_WIDTH)
                .border_color(c.hairline)
        })
        .into_any_element()
}

/// A recording's text, one part at a time (`reading_view`).
fn reading(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let Some(view) = root.app.sessions.gv_reading_view() else {
        return div().into_any_element();
    };
    let ReadingView {
        name,
        views,
        made,
        add_to_notes,
        short_of,
        draft_note,
        part,
        times,
        body,
    } = view;
    let back = kit::Button::quiet("reading-back", "Back to recordings")
        .icon(icons::HISTORY_BACK)
        .on_click(cx.listener(|root, _, _, cx| {
            root.app.sessions.gv_close_reading();
            cx.notify();
        }));
    let notes = add_to_notes.map(|listed| {
        if listed {
            kit::status(&kit::Status::new(icons::OK, "In notes", c.document.saved))
                .into_any_element()
        } else {
            kit::Button::secondary("reading-add-to-notes", "Add to notes")
                .density(Density::Read)
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.sessions.gv_summary_to_notes();
                    cx.notify();
                }))
                .into_any_element()
        }
    });
    let head = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .max_w(READER_MEASURE)
        .child(back)
        .child(kit::text(Type::Small, c.muted, name).flex_1())
        .children(notes);
    let chosen = views
        .iter()
        .position(|(index, _)| *index == made)
        .unwrap_or(0);
    let indices: Vec<u8> = views.iter().map(|(index, _)| *index).collect();
    let tabs = kit::Segmented::new(
        "reading-views",
        views.iter().map(|(_, words)| *words).collect::<Vec<_>>(),
        chosen,
    )
    .on_select(cx.listener(move |root, at: &usize, _, cx| {
        if let Some(made) = indices.get(*at) {
            root.app.sessions.gv_read_made(*made);
            cx.notify();
        }
    }));
    let short = short_of.map(|(done, parts)| {
        kit::Message::new(
            kit::Tone::Warning,
            format!(
                "Transcribed so far: {done} of {parts} parts. Continue processing adds the rest."
            ),
        )
    });
    let draft = draft_note.then(|| {
        kit::text(
            Type::Small,
            c.muted,
            "This draft was written automatically. Check it against your memory.",
        )
    });
    let count = times.len();
    let parts = (count > 1).then(|| {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .child(
                kit::Button::quiet("reading-earlier", "Earlier")
                    .icon(icons::UP)
                    .name("Earlier part")
                    .density(Density::Read)
                    .line_start()
                    .blocked((part == 0).then_some("This is the first part."))
                    .on_click(cx.listener(move |root, _, _, cx| {
                        root.app.sessions.gv_read_part(part.saturating_sub(1));
                        cx.notify();
                    })),
            )
            .child(kit::text(
                Type::Small,
                c.muted,
                format!("Part {} of {count}", part + 1),
            ))
            .child(
                kit::Dropdown::new("reading-part", "Go to a time", times.clone(), Some(part))
                    .width(theme::L * 7.5)
                    .on_select(cx.listener(|root, at: &usize, _, cx| {
                        root.app.sessions.gv_read_part(*at);
                        cx.notify();
                    })),
            )
            .child(
                kit::Button::quiet("reading-later", "Later")
                    .icon(icons::DOWN)
                    .name("Later part")
                    .density(Density::Read)
                    .blocked((part + 1 == count).then_some("This is the last part."))
                    .on_click(cx.listener(move |root, _, _, cx| {
                        root.app.sessions.gv_read_part(part + 1);
                        cx.notify();
                    })),
            )
    });
    div()
        .flex()
        .flex_col()
        .size_full()
        .gap(XS)
        .child(head)
        .child(tabs)
        .children(short)
        .children(draft)
        .children(parts)
        .child(div().flex_1().min_h_0().child(kit::scroll_column(
            "reading-body",
            div().max_w(READER_MEASURE).child(kit::markdown(
                id(format!("reading-text-{made}-{part}")),
                body,
            )),
        )))
        .into_any_element()
}

/// `Delete this recording?`, while one waits for the DM's choice.
pub(super) fn delete_dialog(
    root: &mut SplatRoot,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let title = root.app.sessions.gv_confirm_delete()?;
    let entity = cx.entity().downgrade();
    Some(
        kit::Dialog::new("delete-recording", "Delete this recording?")
            .child(kit::text(
                Type::Body,
                colors().text,
                format!(
                    "This deletes the recording of {title}, its transcripts and its summary. You cannot undo this. The timeline stays as it is."
                ),
            ))
            .destructive(
                kit::Button::destructive("delete-recording-confirm", "Delete recording").on_click(
                    cx.listener(|root, _, _, cx| {
                        root.app.sessions.gv_delete_recording();
                        cx.notify();
                    }),
                ),
            )
            .action(
                kit::Button::secondary("delete-recording-keep", "Keep the recording").on_click(
                    cx.listener(|root, _, _, cx| {
                        root.app.sessions.gv_keep_recording();
                        cx.notify();
                    }),
                ),
            )
            .on_escape(move |_, cx| {
                let _ = entity.update(cx, |root, cx| {
                    root.app.sessions.gv_keep_recording();
                    cx.notify();
                });
            })
            .into_any_element(),
    )
}

use kit::WithTooltip as _;

trait IntoStatusElement {
    fn into_element(self) -> AnyElement;
}

impl IntoStatusElement for kit::Status {
    fn into_element(self) -> AnyElement {
        kit::status(&self).into_any_element()
    }
}
