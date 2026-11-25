//! `/` in the note (`sessions/slash.rs`): a short menu of commands at the note. `Search` links an
//! entry or a note; `Generate NPC` rolls one from the campaign's tables, saves it to the World,
//! and links it. The link goes where the `/` was typed, so the DM keeps writing. Up, Down and
//! Enter choose; Escape closes and leaves the words as they are. With only the `/` typed, Enter
//! still ends the line. The menu has these keys only while the note or the menu has the
//! keyboard, and a press outside the menu closes it (egui's popup closed on a click outside).
use super::note;
use crate::gui::{SplatRoot, kit, theme};
use crate::sessions::page::gpui_facade::slash as rules;
use gpui::{
    AnyElement, Context, ElementId, InteractiveElement, IntoElement, ParentElement, SharedString,
    Styled, Window, div, px,
};
use theme::{L, M, RADIUS_LAYER, S, Type, XS, colors};
use uuid::Uuid;

/// The menu's width (`MENU_WIDTH`).
const MENU_WIDTH: gpui::Pixels = px(320.0);

/// An open `/` menu: whose note, where the `/` is (a character count), and its stage.
pub(super) struct Open {
    pub(super) session: Option<Uuid>,
    pub(super) in_panel: bool,
    at: usize,
    stage: Stage,
}

enum Stage {
    Menu {
        highlight: usize,
        /// Up or Down moved the highlight: Enter chooses, also with no letters typed.
        moved: bool,
    },
    Search {
        query: String,
        highlight: usize,
        focus: bool,
    },
    Npc {
        npc: Option<splat_content_ui::npc_generator::Generated>,
        error: Option<String>,
    },
}

fn chars_before(words: &str, byte: usize) -> usize {
    words[..byte.min(words.len())].chars().count()
}

fn byte_of(words: &str, at: usize) -> usize {
    words
        .char_indices()
        .nth(at)
        .map_or(words.len(), |(byte, _)| byte)
}

/// The DM's edit of the note: open the menu on a new `/` at a word's start; follow the letters
/// typed after it while the menu lists commands.
pub(super) fn follow(
    root: &mut SplatRoot,
    in_panel: bool,
    before: &str,
    words: &str,
    caret: usize,
) {
    let session = root.app.sessions.gv_selected();
    let caret = chars_before(words, caret);
    let state = &mut root.pages.sessions.note;
    match &mut state.slash {
        Some(open) if open.session == session && open.in_panel == in_panel => {
            if let Stage::Menu { highlight, .. } = &mut open.stage {
                let typed: Option<String> =
                    (caret > open.at && words.chars().nth(open.at) == Some('/')).then(|| {
                        words
                            .chars()
                            .skip(open.at + 1)
                            .take(caret - open.at - 1)
                            .collect()
                    });
                match typed.as_deref().and_then(rules::commands) {
                    Some(found) => *highlight = (*highlight).min(found.len() - 1),
                    None => state.slash = None,
                }
            }
        }
        _ => {
            state.slash = None;
            let typed_one = words.chars().count() == before.chars().count() + 1;
            if typed_one && rules::opens_at(words, caret) {
                state.slash = Some(Open {
                    session,
                    in_panel,
                    at: caret - 1,
                    stage: Stage::Menu {
                        highlight: 0,
                        moved: false,
                    },
                });
            }
        }
    }
}

/// Up, Down, Enter and Escape reach the open menu before any key binding (the editor's own,
/// the root's Escape): a keystroke interceptor, kept for the root's life. A key the menu does not
/// take goes to the note's other keys (`note::keys`).
pub(super) fn init(window: &mut Window, cx: &mut Context<SplatRoot>) -> gpui::Subscription {
    let _ = window;
    let root = cx.entity().downgrade();
    cx.intercept_keystrokes(move |event, window, cx| {
        let _ = root.update(cx, |root, cx| {
            if !keys(root, &event.keystroke, window, cx) {
                note::keys(root, &event.keystroke, window, cx);
            }
        });
    })
}

/// The open menu's keys; true when it took the key.
fn keys(
    root: &mut SplatRoot,
    keystroke: &gpui::Keystroke,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> bool {
    let session = root.app.sessions.gv_selected();
    let Some(open) = root
        .pages
        .sessions
        .note
        .slash
        .as_ref()
        .filter(|open| open.session == session)
    else {
        return false;
    };
    let in_panel = open.in_panel;
    if !has_keys(root, window, cx) || keystroke.modifiers.modified() {
        return false;
    }
    let key = keystroke.key.as_str();
    // With only the `/`, Enter ends the line, and the menu goes with the line's end.
    if key == "enter"
        && let Some(Open {
            at,
            stage: Stage::Menu { moved: false, .. },
            ..
        }) = root.pages.sessions.note.slash
        && typed(root, in_panel, at, cx).0.is_empty()
    {
        root.pages.sessions.note.slash = None;
        cx.notify();
        return false;
    }
    let Some(open) = root.pages.sessions.note.slash.as_mut() else {
        return false;
    };
    match key {
        "escape" => {
            let in_menu = matches!(open.stage, Stage::Menu { .. });
            let at = open.at;
            root.pages.sessions.note.slash = None;
            if !in_menu {
                note::place_caret(root, in_panel, at, window, cx);
            }
        }
        "up" | "down" => match &mut open.stage {
            Stage::Menu { highlight, moved } => {
                *moved = true;
                *highlight = step(*highlight, key);
            }
            Stage::Search { highlight, .. } => *highlight = step(*highlight, key),
            Stage::Npc { .. } => {}
        },
        "enter" => match &open.stage {
            Stage::Menu { highlight, .. } => {
                let highlight = *highlight;
                choose_command(root, in_panel, highlight, window, cx);
            }
            Stage::Search {
                highlight, query, ..
            } => {
                let (highlight, query) = (*highlight, query.clone());
                choose_result(root, in_panel, &query, highlight, window, cx);
            }
            Stage::Npc { .. } => save_npc(root, in_panel, window, cx),
        },
        _ => return false,
    }
    cx.stop_propagation();
    cx.notify();
    true
}

/// Whether the open menu has the keys now. Its keys are the note's or the menu's own: a key
/// pressed in another control, or while the note is not drawn, goes on as usual (egui: only
/// while the editor had focus).
pub(super) fn has_keys(root: &SplatRoot, window: &Window, cx: &gpui::App) -> bool {
    let session = root.app.sessions.gv_selected();
    let Some(open) = root
        .pages
        .sessions
        .note
        .slash
        .as_ref()
        .filter(|open| open.session == session)
    else {
        return false;
    };
    let in_menu = note::menu_has_keyboard(root, window, cx);
    match open.stage {
        Stage::Menu { .. } | Stage::Npc { .. } => {
            in_menu || note::has_keyboard(root, open.in_panel, window, cx)
        }
        Stage::Search { .. } => in_menu,
    }
}

/// The highlight after Up or Down.
fn step(highlight: usize, key: &str) -> usize {
    if key == "up" {
        highlight.saturating_sub(1)
    } else {
        highlight + 1
    }
}

/// The words typed after the `/` (the command's letters), as the note holds them now.
fn typed(root: &mut SplatRoot, in_panel: bool, at: usize, cx: &gpui::App) -> (String, usize) {
    let words = root
        .app
        .sessions
        .gv_note_words(in_panel)
        .unwrap_or_default();
    let caret = note::caret(root, in_panel, cx)
        .map(|byte| chars_before(&words, byte))
        .unwrap_or(at + 1)
        .max(at + 1);
    let typed = words.chars().skip(at + 1).take(caret - at - 1).collect();
    (typed, caret)
}

/// A command chosen: its letters leave the note, and its stage opens.
fn choose_command(
    root: &mut SplatRoot,
    in_panel: bool,
    highlight: usize,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) {
    let Some(at) = root.pages.sessions.note.slash.as_ref().map(|open| open.at) else {
        return;
    };
    let (letters, caret) = typed(root, in_panel, at, cx);
    let Some(found) = rules::commands(&letters) else {
        root.pages.sessions.note.slash = None;
        return;
    };
    let Some(command) = found.get(highlight.min(found.len() - 1)) else {
        return;
    };
    let search = command.search;
    // The command's words go; its link takes their place later.
    let words = root
        .app
        .sessions
        .gv_note_words(in_panel)
        .unwrap_or_default();
    let mut now = words.clone();
    now.replace_range(byte_of(&words, at)..byte_of(&words, caret), "");
    root.app.sessions.gv_note_edited(in_panel, &now);
    note::show(root, in_panel, &now, at, window, cx);
    let stage = if search {
        Stage::Search {
            query: String::new(),
            highlight: 0,
            focus: true,
        }
    } else {
        match root.app.lookup.index() {
            Some(index) => match rules::roll(&index) {
                Ok(npc) => Stage::Npc {
                    npc: Some(npc),
                    error: None,
                },
                Err(error) => Stage::Npc {
                    npc: None,
                    error: Some(error),
                },
            },
            None => Stage::Npc {
                npc: None,
                error: None,
            },
        }
    };
    if let Some(open) = &mut root.pages.sessions.note.slash {
        open.stage = stage;
    }
}

/// A search result chosen: its link goes where the `/` was.
fn choose_result(
    root: &mut SplatRoot,
    in_panel: bool,
    query: &str,
    highlight: usize,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) {
    let Some(index) = root.app.lookup.index() else {
        return;
    };
    let (found, _) = rules::search(&index, query);
    if let Some((_, _, link)) = found.get(highlight.min(found.len().saturating_sub(1))) {
        insert_link(root, in_panel, link, window, cx);
    }
}

/// `Save`: the rolled NPC goes to the World, and its link to the note.
fn save_npc(
    root: &mut SplatRoot,
    in_panel: bool,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) {
    let index = root.app.lookup.index();
    let Some(Open {
        stage: Stage::Npc { npc: Some(npc), .. },
        ..
    }) = &root.pages.sessions.note.slash
    else {
        return;
    };
    let npc = npc.clone();
    match root.app.sessions.gv_save_npc(&npc, index.as_deref()) {
        Ok(link) => insert_link(root, in_panel, &link, window, cx),
        Err(problem) => {
            if let Some(Open {
                stage: Stage::Npc { error, .. },
                ..
            }) = &mut root.pages.sessions.note.slash
            {
                *error = Some(problem);
            }
        }
    }
}

fn insert_link(
    root: &mut SplatRoot,
    in_panel: bool,
    link: &str,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) {
    let Some(open) = root.pages.sessions.note.slash.take() else {
        return;
    };
    let words = root
        .app
        .sessions
        .gv_note_words(in_panel)
        .unwrap_or_default();
    let at = open.at.min(words.chars().count());
    let mut now = words.clone();
    now.insert_str(byte_of(&words, at), link);
    root.app.sessions.gv_note_edited(in_panel, &now);
    note::show(root, in_panel, &now, at + link.chars().count(), window, cx);
}

fn id(words: impl std::fmt::Display) -> ElementId {
    ElementId::Name(SharedString::from(words.to_string()))
}

fn row(
    element: ElementId,
    icon: Option<&'static str>,
    word: String,
    hint: String,
    selected: bool,
) -> kit::Row {
    let c = colors();
    kit::Row::new(element, word.clone())
        .height(theme::XXL)
        .selected(selected)
        .children(icon.map(|icon| kit::icon(icon, Type::Body, c.muted)))
        .child(
            div()
                .flex()
                .flex_row()
                .min_w_0()
                .items_baseline()
                .child(kit::text(Type::Body, c.text, word).flex_none())
                .child(kit::text::one_line(Type::Small, c.muted, hint).min_w_0()),
        )
}

/// The open menu, over the note, while it belongs to this note and place.
pub(super) fn render(
    root: &mut SplatRoot,
    in_panel: bool,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let session = root.app.sessions.gv_selected();
    let open = root
        .pages
        .sessions
        .note
        .slash
        .as_ref()
        .filter(|open| open.session == session && open.in_panel == in_panel)?;
    let c = colors();
    let at = open.at;
    let content: AnyElement = match &open.stage {
        Stage::Menu { highlight, .. } => {
            let highlight = *highlight;
            let (letters, _) = typed(root, in_panel, at, cx);
            let found = rules::commands(&letters)?;
            let highlight = highlight.min(found.len() - 1);
            div()
                .flex()
                .flex_col()
                .children(found.into_iter().enumerate().map(|(row_at, command)| {
                    row(
                        id(format!("slash-command-{}", command.word)),
                        Some(command.icon),
                        command.word.to_owned(),
                        format!(" · {}", command.hint),
                        row_at == highlight,
                    )
                    .on_click(cx.listener(move |root, _, window, cx| {
                        choose_command(root, in_panel, row_at, window, cx);
                        cx.notify();
                    }))
                }))
                .into_any_element()
        }
        Stage::Search {
            query,
            highlight,
            focus,
        } => {
            let (query, highlight, focus) = (query.clone(), *highlight, *focus);
            let field = kit::TextField::new("slash-search", query.clone())
                .name("Search to link")
                .placeholder("Search to link")
                .focus(focus)
                .on_change(cx.listener(|root, words: &str, _, cx| {
                    if let Some(Open {
                        stage:
                            Stage::Search {
                                query,
                                highlight,
                                focus,
                            },
                        ..
                    }) = &mut root.pages.sessions.note.slash
                    {
                        words.clone_into(query);
                        *highlight = 0;
                        *focus = false;
                    }
                    cx.notify();
                }));
            if focus
                && let Some(Open {
                    stage: Stage::Search { focus, .. },
                    ..
                }) = &mut root.pages.sessions.note.slash
            {
                *focus = false;
            }
            let mut column = div().flex().flex_col().gap(XS).child(field);
            match root.app.lookup.index() {
                None => column = column.child(kit::text(Type::Body, c.muted, "Reading content…")),
                Some(_) if query.trim().is_empty() => {
                    column = column.child(kit::text(
                        Type::Small,
                        c.muted,
                        "Type a name, a heading or words from a note.",
                    ));
                }
                Some(index) => {
                    let (found, all) = rules::search(&index, &query);
                    if found.is_empty() {
                        column = column.child(kit::text(
                            Type::Body,
                            c.muted,
                            "Nothing found. Check the spelling, or try fewer words.",
                        ));
                    } else {
                        let highlight = highlight.min(found.len() - 1);
                        let shown = found.len();
                        for (row_at, (name, detail, _)) in found.into_iter().enumerate() {
                            let query = query.clone();
                            column = column.child(
                                row(
                                    id(format!("slash-result-{row_at}")),
                                    None,
                                    name,
                                    format!(" · {detail}"),
                                    row_at == highlight,
                                )
                                .on_click(cx.listener(
                                    move |root, _, window, cx| {
                                        choose_result(root, in_panel, &query, row_at, window, cx);
                                        cx.notify();
                                    },
                                )),
                            );
                        }
                        if all > shown {
                            column = column.child(kit::text(
                                Type::Small,
                                c.muted,
                                format!(
                                    "The best {shown} of {all}. Type more words to narrow them."
                                ),
                            ));
                        }
                    }
                }
            }
            column.into_any_element()
        }
        Stage::Npc { npc, error } => {
            let mut column = div().flex().flex_col().gap(S);
            match npc {
                Some(npc) => {
                    column = column
                        .child(kit::text(Type::Section, c.text, npc.name.clone()))
                        .child(kit::markdown(id("slash-npc-details"), npc.details()).compact());
                    column = column.child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(S)
                            .child(
                                kit::Button::primary("slash-npc-save", "Save to World")
                                    .density(theme::Density::Read)
                                    .on_click(cx.listener(move |root, _, window, cx| {
                                        save_npc(root, in_panel, window, cx);
                                        cx.notify();
                                    })),
                            )
                            .child(
                                kit::Button::secondary("slash-npc-again", "Roll again")
                                    .density(theme::Density::Read)
                                    .on_click(cx.listener(move |root, _, window, cx| {
                                        reroll(root);
                                        let _ = window;
                                        cx.notify();
                                    })),
                            )
                            .child(
                                kit::Button::quiet("slash-npc-discard", "Discard")
                                    .density(theme::Density::Read)
                                    .on_click(cx.listener(move |root, _, window, cx| {
                                        root.pages.sessions.note.slash = None;
                                        note::place_caret(root, in_panel, at, window, cx);
                                        cx.notify();
                                    })),
                            ),
                    );
                }
                None if error.is_none() => {
                    column = column.child(kit::text(Type::Body, c.muted, "Reading content…"));
                }
                None => {}
            }
            if let Some(error) = error {
                column = column.child(kit::Message::new(kit::Tone::Error, error.clone()));
            }
            column.into_any_element()
        }
    };
    let focus = note::menu_focus(root, cx);
    let layer = div()
        .track_focus(&focus)
        // A press outside closes the menu; what was typed stays as it is.
        .on_mouse_down_out(cx.listener(|root, _, _, cx| {
            root.pages.sessions.note.slash = None;
            cx.notify();
        }))
        .w(MENU_WIDTH)
        .p(S)
        .rounded(RADIUS_LAYER)
        .bg(c.raised)
        .border(theme::HAIRLINE_WIDTH)
        .border_color(c.hairline)
        .shadow_md()
        .child(content);
    // Under the `/`, or above it when the window has no room below: the
    // place where the `/` was last drawn. Before the note has drawn it, at the note's top.
    let words = root
        .app
        .sessions
        .gv_note_words(in_panel)
        .unwrap_or_default();
    let place = note::bounds_at(root, in_panel, byte_of(&words, at), cx);
    Some(match place {
        Some(place) => gpui::deferred(
            gpui::anchored()
                .attach(place, gpui::Anchor::BottomLeft)
                .anchor(gpui::Anchor::TopLeft)
                .offset(gpui::point(px(0.0), XS))
                .child(layer),
        )
        .with_priority(1)
        .into_any_element(),
        None => layer.absolute().top(L * 2.0).left(M).into_any_element(),
    })
}

fn reroll(root: &mut SplatRoot) {
    let Some(index) = root.app.lookup.index() else {
        return;
    };
    if let Some(Open {
        stage: Stage::Npc { npc, error },
        ..
    }) = &mut root.pages.sessions.note.slash
    {
        match rules::roll(&index) {
            Ok(rolled) => {
                *npc = Some(rolled);
                *error = None;
            }
            Err(problem) => *error = Some(problem),
        }
    }
}
