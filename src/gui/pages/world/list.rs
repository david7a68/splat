//! The list column (egui: `WorldPage::list`, `check_rows`, `broken_row`, `marked_row`): the search
//! field, the two check rows, the notes by group, the notes that cannot be read and the repair
//! drafts.
//!
//! The list can hold thousands of notes, so it is a virtual list (`kit::scroll::list_column`):
//! its rows are data (`Flat`), made again only when what they say changes (a new load of the
//! notes, new search words, a check or repair row), and only the rows in view are laid out. A
//! row reads whether it is chosen from the page when it draws, so a press moves nothing.
use super::{SplatRoot, on_words};
use crate::gui::{icons, kit, theme};
use crate::lookup::ContentIndex;
use crate::world::page::{ListRow, View};
use gpui::{
    AnyElement, App, Context, InteractiveElement, IntoElement, ListAlignment, ListState,
    ParentElement, SharedString, StatefulInteractiveElement, Styled, WeakEntity, Window, div,
    prelude::*, px,
};
use std::rc::Rc;

/// One row of the list, as data.
#[derive(Clone, PartialEq)]
enum Flat {
    Nothing,
    Check(View, &'static str, String),
    Heading {
        title: String,
        count: usize,
        folder: bool,
        first: bool,
    },
    Note(Rc<ListRowData>),
    BrokenHeading(usize),
    Broken {
        path: String,
        cause: String,
        hover: String,
    },
    DraftHeading(usize),
    Draft(String),
}

/// A note's row words.
#[derive(Clone, PartialEq)]
struct ListRowData {
    id: uuid::Uuid,
    name: String,
    detail: String,
    depth: usize,
    path: String,
}

impl From<ListRow> for ListRowData {
    fn from(row: ListRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            detail: row.detail,
            depth: row.depth,
            path: row.path,
        }
    }
}

/// What the rows were made from: when it is the same, the rows are too.
#[derive(Clone, PartialEq, Default)]
struct Key {
    generation: u64,
    search: String,
    small: Vec<Flat>,
}

/// The list's rows and its virtual state.
pub(super) struct ListCache {
    key: Option<Key>,
    rows: Rc<Vec<Flat>>,
    state: ListState,
}

impl Default for ListCache {
    fn default() -> Self {
        Self {
            key: None,
            rows: Rc::new(Vec::new()),
            state: ListState::new(0, ListAlignment::Top, px(400.0)),
        }
    }
}

/// The rows of checks, unreadable notes and repair drafts: few, so they are made each frame.
fn small_rows(root: &mut SplatRoot, index: &ContentIndex) -> (Vec<Flat>, Vec<Flat>) {
    let page = &mut root.app.world_page;
    let mut before = Vec::new();
    if page.search().trim().is_empty() {
        for (view, title, detail, _) in page.check_rows_shown() {
            before.push(Flat::Check(view, title, detail));
        }
    }
    let mut after = Vec::new();
    let broken = page.broken_rows(index);
    if !broken.is_empty() {
        after.push(Flat::BrokenHeading(broken.len()));
        for (path, cause, hover, _) in broken {
            after.push(Flat::Broken { path, cause, hover });
        }
    }
    let drafts = page.draft_repairs(index);
    if !drafts.is_empty() {
        after.push(Flat::DraftHeading(drafts.len()));
        after.extend(drafts.into_iter().map(Flat::Draft));
    }
    (before, after)
}

/// The list column.
pub(super) fn render(
    root: &mut SplatRoot,
    index: &ContentIndex,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let (before, after) = small_rows(root, index);
    let key = Key {
        generation: index.generation(),
        search: root.app.world_page.search().to_owned(),
        small: before.iter().chain(&after).cloned().collect(),
    };
    let cache = &mut root.pages.world.list;
    if cache.key.as_ref() != Some(&key) {
        let groups = root.app.world_page.list_groups(index);
        let mut rows = before;
        if groups.is_empty() && index.broken_world.is_empty() {
            rows.push(Flat::Nothing);
        }
        for (at, group) in groups.into_iter().enumerate() {
            rows.push(Flat::Heading {
                title: group.title,
                count: group.rows.len(),
                folder: group.folder,
                first: at == 0,
            });
            rows.extend(
                group
                    .rows
                    .into_iter()
                    .map(|row| Flat::Note(Rc::new(row.into()))),
            );
        }
        rows.extend(after);
        let old = cache.state.item_count();
        cache.state.splice(0..old, rows.len());
        cache.rows = Rc::new(rows);
        cache.key = Some(key);
    }
    let rows = cache.rows.clone();
    let state = cache.state.clone();
    let entity = cx.entity().downgrade();
    let list = gpui::list(state.clone(), move |at, _, cx| {
        row(&rows[at], at, &entity, cx)
    });
    let search = root.app.world_page.search().to_owned();
    div()
        .flex()
        .flex_col()
        .size_full()
        .child(
            div().pb(theme::XS).child(
                kit::TextField::new("world-search", search)
                    .placeholder("Find a note")
                    .name("Search the world")
                    .on_change(on_words(cx, |root, words| {
                        root.app.world_page.set_search(words);
                    })),
            ),
        )
        .child(
            div()
                .flex_1()
                .min_h_0()
                .child(kit::scroll::list_column("world-list", &state, list)),
        )
        .into_any_element()
}

/// A press handler on a row: it acts on the root with the open campaign and notes.
fn press(
    entity: &WeakEntity<SplatRoot>,
    f: impl Fn(&mut SplatRoot, &crate::persistence::Campaign, &ContentIndex) + 'static,
) -> impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static {
    let entity = entity.clone();
    move |_, _, cx| {
        let _ = entity.update(cx, |root, cx| {
            if let Some((campaign, index)) = super::world(root) {
                f(root, &campaign, &index);
                cx.notify();
            }
        });
    }
}

/// One row, laid out when it is in view.
fn row(flat: &Flat, at: usize, entity: &WeakEntity<SplatRoot>, cx: &mut App) -> AnyElement {
    let c = theme::colors();
    let page = entity
        .upgrade()
        .map(|root| root.read(cx).app.world_page.view());
    match flat {
        Flat::Nothing => kit::text(theme::Type::Body, c.muted, "Nothing here matches.")
            .py(theme::XS)
            .into_any_element(),
        Flat::Check(view, title, detail) => {
            let view = *view;
            let open = page == Some(view);
            kit::two_line_row(("world-check", at), *title, detail.clone(), open)
                .height(px(40.0))
                .on_click(press(entity, move |root, _, _| {
                    root.app.world_page.press_check_row(view);
                }))
                .into_any_element()
        }
        Flat::Heading {
            title,
            count,
            folder,
            first,
        } => {
            if *folder {
                // A folder's heading: its icon, the folder's words and its count, muted `Small`.
                let words = crate::design::FolderGroup::of_path(title).words();
                div()
                    .pt(if *first { theme::XS } else { theme::L })
                    .pb(theme::XS)
                    .flex()
                    .flex_row()
                    .gap(theme::XS)
                    .items_center()
                    .child(kit::icon(icons::phosphor::FOLDER, theme::Type::Small, c.muted))
                    .child(kit::text(
                        theme::Type::Small,
                        c.muted,
                        format!("{words}  {count}"),
                    ))
                    .into_any_element()
            } else {
                kit::heading(
                    kit::Heading::Group {
                        title,
                        count: *count,
                    },
                    false,
                )
                .when(*first, |heading| heading.pt(theme::XS))
                .into_any_element()
            }
        }
        Flat::Note(note) => {
            let selected = entity
                .upgrade()
                .is_some_and(|root| root.read(cx).app.world_page.row_selected(note.id));
            note_row(note, selected, entity).into_any_element()
        }
        Flat::BrokenHeading(count) => kit::heading(
            kit::Heading::Group {
                title: "Cannot be read",
                count: *count,
            },
            false,
        )
        .into_any_element(),
        Flat::Broken { path, cause, hover } => {
            let open = entity
                .upgrade()
                .is_some_and(|root| root.read(cx).app.world_page.repair_open(path));
            broken_row(at, path, cause, hover, open, entity)
        }
        Flat::DraftHeading(count) => kit::heading(
            kit::Heading::Group {
                title: "Repair drafts",
                count: *count,
            },
            false,
        )
        .into_any_element(),
        Flat::Draft(path) => {
            let path = path.clone();
            kit::two_line_row(("world-draft", at), path.clone(), "Unsaved repair", false)
                .height(px(40.0))
                .on_click(press(entity, move |root, campaign, _| {
                    root.app.world_page.press_repair(campaign, &path);
                }))
                .into_any_element()
        }
    }
}

/// A note's row: its name, and under it a muted detail whose warning mark has the warning's
/// colour. A row with no detail is one `Read` row tall. Each level of depth sets it in by `S`.
fn note_row(note: &ListRowData, selected: bool, entity: &WeakEntity<SplatRoot>) -> impl IntoElement {
    let c = theme::colors();
    let id = note.id;
    let name = if note.detail.is_empty() {
        format!("{} · ", note.name)
    } else {
        format!("{} · {}", note.name, note.detail)
    };
    let title = kit::text(
        if selected && !note.detail.is_empty() {
            theme::Type::Strong
        } else {
            theme::Type::Body
        },
        c.text,
        note.name.clone(),
    )
    .truncate();
    let row = kit::Row::new(("world-note", id.as_u128() as u64), name)
        .selected(selected)
        .on_click(press(entity, move |root, _, _| {
            root.app.world_page.press_note(id);
        }));
    let row = if note.detail.is_empty() {
        row.height(theme::Density::Read.row()).child(title)
    } else {
        row.height(px(40.0)).child(
            div()
                .flex()
                .flex_col()
                .min_w_0()
                .flex_1()
                .child(title)
                .child(marked_line(&note.detail)),
        )
    };
    div()
        .id(("world-note-place", id.as_u128() as u64))
        .pl(theme::S * note.depth as f32)
        .tooltip(kit::tooltip::tip(SharedString::from(note.path.clone())))
        .child(row)
}

/// A detail line: a leading warning mark in the warning colour, the words muted, cut to one line.
fn marked_line(detail: &str) -> impl IntoElement {
    let c = theme::colors();
    let (mark, words) = match detail.strip_prefix(icons::WARN) {
        Some(rest) => (Some(icons::WARN), rest.trim_start().to_owned()),
        None => (None, detail.to_owned()),
    };
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::XS)
        .min_w_0()
        .when_some(mark, |line, mark| {
            line.child(kit::icon(mark, theme::Type::Small, c.result.warning))
        })
        .child(kit::text(theme::Type::Small, c.muted, words).truncate())
}

/// A note that cannot be read: its path whole (it wraps), and under it the error mark and its
/// cause, each wrapped line at the cause's first word. The whole cause on hover.
fn broken_row(
    at: usize,
    path: &str,
    cause: &str,
    hover: &str,
    open: bool,
    entity: &WeakEntity<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let owned = path.to_owned();
    div()
        .id(("world-broken-place", at))
        .tooltip(kit::tooltip::tip(SharedString::from(hover.to_owned())))
        .child(
            kit::Row::new(("world-broken", at), format!("{path} · {cause}"))
                .height(px(40.0))
                .grows()
                .selected(open)
                .on_click(press(entity, move |root, campaign, _| {
                    root.app.world_page.press_repair(campaign, &owned);
                }))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .min_w_0()
                        .flex_1()
                        .py(theme::XS)
                        .child(kit::text(
                            if open {
                                theme::Type::Strong
                            } else {
                                theme::Type::Body
                            },
                            c.text,
                            path.to_owned(),
                        ))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_start()
                                .gap(theme::XS)
                                .child(kit::icon(icons::ERROR, theme::Type::Small, c.result.error))
                                .child(
                                    kit::text(theme::Type::Small, c.muted, cause.to_owned())
                                        .flex_1()
                                        .min_w_0(),
                                ),
                        ),
                ),
        )
        .into_any_element()
}
