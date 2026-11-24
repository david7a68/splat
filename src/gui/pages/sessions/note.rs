//! The session's note: words to write in an
//! editor that saves a moment after the DM stops (and at Ctrl+S), or the same words to read;
//! and, while the file changed on disk, the draft beside the file (`grouped_draft_comparison`).
//! The editor keeps one GPUI state per session and place (page or panel), so its caret and its
//! typing history are that note's.
use super::with_plan;
use crate::gui::{SplatRoot, kit, theme};
use gpui::{
    AnyElement, App, Context, ElementId, InteractiveElement, IntoElement, ParentElement,
    SharedString, Styled, Window, div,
};
use std::collections::HashMap;
use theme::{HAIRLINE_WIDTH, READER_MEASURE, S, Type, XS, colors};
use uuid::Uuid;

/// The note editors' GPUI state, by session and place (`true`: the Notes panel).
#[derive(Default)]
pub(crate) struct State {
    editors: HashMap<(Option<Uuid>, bool), kit::EditorHandle>,
    /// The `/` menu, while it is open (`slash`).
    pub(super) slash: Option<super::slash::Open>,
    /// The `/` menu's keys (`slash::init`).
    slash_keys: Option<gpui::Subscription>,
    /// The `/` menu's place in the focus tree: its search field has the keyboard inside it.
    slash_focus: Option<gpui::FocusHandle>,
    /// The plan column's place in the focus tree: a plan control has the keyboard inside it.
    plan_focus: Option<gpui::FocusHandle>,
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

/// The `/` menu's focus handle, made once.
pub(super) fn menu_focus(root: &mut SplatRoot, cx: &mut App) -> gpui::FocusHandle {
    root.pages
        .sessions
        .note
        .slash_focus
        .get_or_insert_with(|| cx.focus_handle())
        .clone()
}

/// The plan column's focus handle, made once (`session::render` tracks it on the plan).
pub(super) fn plan_focus(root: &mut SplatRoot, cx: &mut App) -> gpui::FocusHandle {
    root.pages
        .sessions
        .note
        .plan_focus
        .get_or_insert_with(|| cx.focus_handle())
        .clone()
}

/// Whether the keyboard is in the `/` menu (its search field), as last drawn.
pub(super) fn menu_has_keyboard(root: &SplatRoot, window: &Window, cx: &App) -> bool {
    root.pages
        .sessions
        .note
        .slash_focus
        .as_ref()
        .is_some_and(|focus| focus.contains_focused(window, cx))
}

/// Whether the note's editor at this place has the keyboard and was drawn in the last frame (a
/// focus kept by an editor that is no longer drawn is not the note's).
pub(super) fn has_keyboard(root: &SplatRoot, in_panel: bool, window: &Window, cx: &App) -> bool {
    handle(root, in_panel).is_some_and(|handle| handle.focused(window, cx))
        && root.focus.contains_focused(window, cx)
}

/// The note's keys that are not the `/` menu's (`slash::init`'s interceptor):
/// - a correction key in the note means the DM goes on with the same line, so no new paragraph
///   waits (`gv_new_thought`);
/// - beside the Notes panel, words typed or pasted while no control has the keyboard belong in
///   the note: a stray click leaves the keyboard on nothing while the DM's eyes are on the table.
pub(super) fn keys(
    root: &mut SplatRoot,
    keystroke: &gpui::Keystroke,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) {
    const CORRECTING: [&str; 8] = [
        "backspace",
        "delete",
        "left",
        "right",
        "up",
        "down",
        "home",
        "end",
    ];
    if CORRECTING.contains(&keystroke.key.as_str())
        && (has_keyboard(root, false, window, cx) || has_keyboard(root, true, window, cx))
    {
        root.app.sessions.gv_correcting();
        return;
    }
    let panel_drawn = root.app.sessions.notes_open && root.app.page != crate::Page::Sessions;
    let on_nothing = window
        .focused(cx)
        .is_none_or(|focused| page_keyboard(root).contains(&focused));
    if !panel_drawn || !on_nothing || root.modal_open(cx) {
        return;
    }
    let modifiers = keystroke.modifiers;
    let words = if modifiers.secondary(cx) && !modifiers.alt && keystroke.key == "v" {
        cx.read_from_clipboard().and_then(|item| item.text())
    } else if !modifiers.control && !modifiers.alt && !modifiers.platform && !modifiers.function {
        keystroke
            .key_char
            .clone()
            .filter(|words| !words.is_empty() && !words.chars().any(char::is_control))
    } else {
        None
    };
    let Some(words) = words.filter(|words| !words.is_empty()) else {
        return;
    };
    root.app.sessions.gv_stray_words(&words);
    cx.stop_propagation();
    cx.notify();
}

/// The focus handles that mean no control has the keyboard: the root's, and a page's own, which
/// a page takes from the root so its keys reach it.
fn page_keyboard(root: &SplatRoot) -> Vec<gpui::FocusHandle> {
    let pages = &root.pages;
    std::iter::once(Some(&root.focus))
        .chain([
            pages.timeline.focus.as_ref(),
            pages.run.focus.as_ref(),
            pages.encounters.focus.as_ref(),
            pages.library.focus.as_ref(),
            pages.campaigns.focus.as_ref(),
        ])
        .flatten()
        .cloned()
        .collect()
}

/// Where the note editor last drew the character at byte `at`, in window coordinates.
pub(super) fn bounds_at(
    root: &SplatRoot,
    in_panel: bool,
    at: usize,
    cx: &App,
) -> Option<gpui::Bounds<gpui::Pixels>> {
    handle(root, in_panel)?.bounds_at(at, cx)
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
    // Elsewhere is a plan control: not the note's own `/` search field, the tabs or a control
    // outside the plan, which in the tab layout would swap the note (and its menu) for the plan.
    let elsewhere = root
        .pages
        .sessions
        .note
        .plan_focus
        .as_ref()
        .is_some_and(|plan| plan.contains_focused(window, cx));
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
        .markdown_source()
        .fill()
        .on_change(cx.listener(move |root, words: &str, window, cx| {
            let before = root
                .app
                .sessions
                .gv_note_words(in_panel)
                .unwrap_or_default();
            let caret = caret(root, in_panel, cx).unwrap_or(words.len());
            // After a step of the run, the first words typed at the end of a line start a new
            // paragraph.
            if let Some((now, caret)) = root.app.sessions.gv_new_thought(&before, words, caret) {
                root.app.sessions.gv_note_edited(in_panel, &now);
                // The editor is inside its own change event: show the words after it.
                cx.defer_in(window, move |root, window, cx| {
                    if let Some(handle) = self::handle(root, in_panel) {
                        handle.show_words(now, window, cx);
                        handle.set_caret(caret, cx);
                    }
                    cx.notify();
                });
                cx.notify();
                return;
            }
            root.app.sessions.gv_note_edited(in_panel, words);
            super::slash::follow(root, in_panel, &before, words, caret);
            cx.notify();
        }));
    let menu = super::slash::render(root, in_panel, cx);
    div()
        .key_context("SessionNote")
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
/// the banner (`Overwrite with my draft`, `Reload and lose my draft`). The note's grouped
/// comparison (`design::grouped_draft_comparison`, in the note's prose face).
pub(super) fn conflict(
    root: &mut SplatRoot,
    _window: &mut Window,
    _cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let session = root.app.sessions.gv_selected();
    let Some((draft, disk)) = root.app.sessions.gv_conflict_texts() else {
        return div().into_any_element();
    };
    let id = ElementId::Name(SharedString::from(format!(
        "session-conflict-{}",
        session.map_or_else(String::new, |id| id.to_string())
    )));
    div()
        .size_full()
        .child(kit::DraftComparison::new(id, draft, disk))
        .into_any_element()
}
