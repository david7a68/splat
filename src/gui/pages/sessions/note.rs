//! The session's note (`Sessions::note_pane`, `note_editor`, `read_tab`): words to write in an
//! editor that saves a moment after the DM stops (and at Ctrl+S), or the same words to read;
//! and, while the file changed on disk, the draft beside the file (`grouped_draft_comparison`).
//! The editor keeps one GPUI state per session and place (page or panel), so its caret and its
//! typing history are that note's.
use super::with_plan;
use crate::gui::{SplatRoot, kit, theme};
use gpui::{
    AnyElement, App, Context, ElementId, InteractiveElement, IntoElement, KeyBinding,
    ParentElement, SharedString, Styled, Window, actions, div, prelude::*,
};
use std::collections::HashMap;
use theme::{HAIRLINE_WIDTH, L, READER_MEASURE, S, Type, XS, colors};
use uuid::Uuid;

actions!(session_note, [SaveNote]);

/// The note editors' GPUI state, by session and place (`true`: the Notes panel).
#[derive(Default)]
pub(crate) struct State {
    editors: HashMap<(Option<Uuid>, bool), kit::EditorHandle>,
    /// The `/` menu, while it is open (`slash`).
    pub(super) slash: Option<super::slash::Open>,
    /// The `/` menu's keys (`slash::init`).
    slash_keys: Option<gpui::Subscription>,
}

/// The editor of the open note at this place, if it was drawn.
fn handle(root: &SplatRoot, in_panel: bool) -> Option<kit::EditorHandle> {
    let session = root.app.sessions.gv_selected();
    root.pages
        .sessions
        .note
        .editors
        .get(&(session, in_panel))
        .cloned()
}

/// The note editor's caret, as a byte offset of its words.
pub(super) fn caret(root: &SplatRoot, in_panel: bool, cx: &App) -> Option<usize> {
    Some(handle(root, in_panel)?.caret(cx))
}

/// Show the note's new words in its editor now, with the caret at character `at`, and give it
/// the keyboard (a command's link, the menu closed).
pub(super) fn show(
    root: &mut SplatRoot,
    in_panel: bool,
    words: &str,
    at: usize,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) {
    if let Some(handle) = handle(root, in_panel) {
        handle.show_words(words.to_owned(), window, cx);
        let byte = words
            .char_indices()
            .nth(at)
            .map_or(words.len(), |(byte, _)| byte);
        handle.set_caret(byte, cx);
        handle.focus(window, cx);
    }
}

/// Back to the note with the caret at character `at`.
pub(super) fn place_caret(
    root: &mut SplatRoot,
    in_panel: bool,
    at: usize,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) {
    let words = root
        .app
        .sessions
        .gv_note_words(in_panel)
        .unwrap_or_default();
    show(root, in_panel, &words, at, window, cx);
}

pub(super) fn init(root: &mut SplatRoot, window: &mut Window, cx: &mut Context<SplatRoot>) {
    root.pages.sessions.note.slash_keys = Some(super::slash::init(window, cx));
    cx.bind_keys([KeyBinding::new("ctrl-s", SaveNote, Some("SessionNote"))]);
    super::form::init(cx);
    super::templates::init(cx);
}

/// The note frame's width: the reader's measure and the frame's edges.
pub(super) fn frame_width() -> gpui::Pixels {
    READER_MEASURE + (S + HAIRLINE_WIDTH) * 2.0
}

/// The note: its editor, or its Read view.
pub(super) fn render(
    root: &mut SplatRoot,
    beside: bool,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    if root.app.sessions.gv_reading_note() {
        read(root, beside, cx)
    } else {
        editor(root, false, window, cx)
    }
}

/// The note's editor, on the page or in the Notes panel.
pub(super) fn editor(
    root: &mut SplatRoot,
    in_panel: bool,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let session = root.app.sessions.gv_selected();
    let Some(words) = root.app.sessions.gv_note_words(in_panel) else {
        return kit::Message::new(kit::Tone::Error, "This note could not be opened.")
            .into_any_element();
    };
    // Forget the editors of notes that are gone from the folder.
    let notes: Vec<Uuid> = root.app.sessions.gv_notes().iter().map(|n| n.id).collect();
    let state = &mut root.pages.sessions.note;
    state
        .editors
        .retain(|(id, _), _| id.is_none_or(|id| notes.contains(&id)) || *id == session);
    let handle = state
        .editors
        .entry((session, in_panel))
        .or_insert_with(|| kit::EditorHandle::new("", window, cx))
        .clone();
    // A request for the keyboard: a fresh line under What happened, or words typed elsewhere
    // put in at the caret.
    let mut words = words;
    if root.app.sessions.gv_wants_focus() {
        handle.show_words(words.clone(), window, cx);
        handle.focus(window, cx);
        if let Some(caret) = root.app.sessions.gv_take_focus(in_panel) {
            if let Some(now) = root.app.sessions.gv_note_words(in_panel) {
                words = now;
            }
            handle.show_words(words.clone(), window, cx);
            handle.set_caret(caret, cx);
        }
        let caret = handle.caret(cx);
        if let Some((now, caret)) = root.app.sessions.gv_take_words(&words, caret) {
            if now != words {
                root.app.sessions.gv_note_edited(in_panel, &now);
                words = now;
            }
            handle.show_words(words.clone(), window, cx);
            handle.set_caret(caret, cx);
        }
    }
    let focused = handle.focused(window, cx);
    let elsewhere = window
        .focused(cx)
        .is_some_and(|other| other != root.focus);
    root.app
        .sessions
        .gv_note_has_focus(in_panel, focused, elsewhere);
    let id = ElementId::Name(SharedString::from(format!(
        "session-note-{}-{in_panel}",
        session.map_or_else(String::new, |id| id.to_string())
    )));
    let editor = kit::TextEditor::new(id, words)
        .handle(&handle)
        .name("Note text")
        .text_type(Type::Prose)
        .fill()
        .on_change(cx.listener(move |root, words: &str, _, cx| {
            let before = root
                .app
                .sessions
                .gv_note_words(in_panel)
                .unwrap_or_default();
            root.app.sessions.gv_note_edited(in_panel, words);
            let caret = caret(root, in_panel, cx).unwrap_or(words.len());
            super::slash::follow(root, in_panel, &before, words, caret);
            cx.notify();
        }));
    let menu = super::slash::render(root, in_panel, cx);
    div()
        .key_context("SessionNote")
        .on_action(cx.listener(|root, _: &SaveNote, _, cx| {
            root.app.sessions.gv_save_note();
            cx.notify();
        }))
        .relative()
        .size_full()
        .max_w(frame_width())
        .child(editor)
        .children(menu)
        .into_any_element()
}

/// The note's Read view: its words as prose, `Nothing written yet.` with `Write` when empty.
fn read(root: &mut SplatRoot, beside: bool, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let index = root.app.lookup.index();
    let Some(view) = with_plan(&mut root.app, |sessions, _, book, plan| {
        sessions.gv_read(index.as_deref(), plan, beside, book)
    })
    .flatten() else {
        return div().into_any_element();
    };
    let mut column = div().flex().flex_col().gap(S).max_w(READER_MEASURE);
    if !view.written {
        column = column.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .child(kit::text(Type::Small, c.muted, "Nothing written yet."))
                .child(
                    kit::Link::new("note-read-write", "Write").on_click(cx.listener(
                        |root, _, _, cx| {
                            root.app.sessions.gv_write();
                            cx.notify();
                        },
                    )),
                ),
        );
    } else {
        column = column.children(parts(view.before, "before", cx));
        if let Some(summary) = view.summary {
            column = column
                .child(kit::text(Type::Small, c.muted, summary))
                .children(parts(view.after, "after", cx));
        }
    }
    kit::scroll_column(
        ElementId::Name(SharedString::from(format!(
            "session-reader-{}",
            root.app
                .sessions
                .gv_selected()
                .map_or_else(String::new, |id| id.to_string())
        ))),
        column,
    )
    .into_any_element()
}

fn parts(
    parts: Vec<crate::sessions::page::gpui_facade::ReadPart>,
    place: &str,
    cx: &mut Context<SplatRoot>,
) -> Vec<AnyElement> {
    use crate::sessions::page::gpui_facade::ReadPart;
    let c = colors();
    parts
        .into_iter()
        .enumerate()
        .map(|(at, part)| {
            let id = format!("session-read-{place}-{at}");
            let link = cx.listener(|root, target: &str, _, cx| {
                root.app.sessions.gv_open_link(target);
                cx.notify();
            });
            match part {
                ReadPart::Words(words) => {
                    kit::markdown::Markdown::new(ElementId::Name(id.into()), words)
                        .on_link(link)
                        .into_any_element()
                }
                ReadPart::Embed { from, label, words } => div()
                    .flex()
                    .flex_col()
                    .gap(XS)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(S)
                            .child(kit::text(Type::Small, c.muted, "From"))
                            .child(
                                kit::Link::new(ElementId::Name(format!("{id}-from").into()), label)
                                    .small()
                                    .wrap()
                                    .on_click(cx.listener(move |root, _, _, cx| {
                                        root.app.sessions.gv_open_session(from);
                                        cx.notify();
                                    })),
                            ),
                    )
                    .child(
                        kit::markdown::Markdown::new(ElementId::Name(id.into()), words)
                            .on_link(link),
                    )
                    .into_any_element(),
                ReadPart::Gone(words) => kit::Message::new(kit::Tone::Warning, words)
                    .small()
                    .into_any_element(),
            }
        })
        .collect()
}

/// The file changed on disk during an edit: the draft beside the file, until the DM chooses in
/// the banner (`Overwrite with my draft`, `Reload and lose my draft`).
pub(super) fn conflict(
    root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let Some((draft, disk)) = root.app.sessions.gv_conflict_texts() else {
        return div().into_any_element();
    };
    // Paragraphs, not blank lines: the two texts side by side, each line once.
    let (draft_lines, disk_lines): (Vec<&str>, Vec<&str>) = (
        draft
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect(),
        disk.lines()
            .filter(|line| !line.trim().is_empty())
            .collect(),
    );
    let summary = changed_lines(&draft_lines, &disk_lines);
    let column = |title: &'static str, lines: &[&str], other: &[&str]| {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .gap(XS)
            .child(kit::label(title, 0))
            .children(lines.iter().map(|line| {
                let differs = !other.contains(line);
                div()
                    .px(S)
                    .when(differs, |row| {
                        row.bg(c.raised)
                            .border_l(theme::MARK_WIDTH)
                            .border_color(c.focus)
                    })
                    .child(kit::text(Type::Prose, c.text, (*line).to_owned()))
            }))
    };
    kit::scroll_column(
        "session-conflict",
        div()
            .flex()
            .flex_col()
            .gap(S)
            .child(kit::text(Type::Small, c.muted, summary))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap(L)
                    .child(column("Your draft", &draft_lines, &disk_lines))
                    .child(column("In the file now", &disk_lines, &draft_lines)),
            ),
    )
    .into_any_element()
}

/// `1 line differs`, `3 lines differ`.
fn changed_lines(draft: &[&str], disk: &[&str]) -> String {
    let count = draft.iter().filter(|line| !disk.contains(line)).count()
        + disk.iter().filter(|line| !draft.contains(line)).count();
    let count = count.div_ceil(2).max(1);
    if count == 1 {
        "1 line differs".to_owned()
    } else {
        format!("{count} lines differ")
    }
}
