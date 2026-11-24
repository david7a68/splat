//! The Library page and the copy dialog (egui: `src/library.rs`, `src/library/`,
//! `src/copy_dialog.rs`, `src/library_page/`).
//!
//! The page reads `crate::library::Library` through its model (`library::model`): the header, the
//! sections and entries columns, the reader (Read, Edit Markdown, a changed file), the New entry and
//! Manage packages forms and the unregister confirmation. Every act calls a model method; the egui
//! page shares the list's logic with it. The footer of each pane draws the same typed lines as
//! egui's (`design::outcome::plain`), and a press goes back to `Library::press_footer`.
mod copy;
mod footer;
mod forms;
mod header;
mod list;
mod reader;
#[cfg(test)]
#[cfg(test)]
mod tests;

use crate::Page;
use crate::gui::{SplatRoot, kit, theme};
use gpui::{
    AnyElement, App, Context, FocusHandle, InteractiveElement, IntoElement, KeyBinding,
    ParentElement, Styled, UniformListScrollHandle, Window, actions, div, px,
};
use std::sync::Arc;

actions!(
    library,
    [
        /// Alt+Left: back to the entry before the last link.
        Back,
        /// Alt+Right: forward to the entry Back left.
        Forward,
        /// Up in the entry list: the keyboard goes to the row above.
        RowUp,
        /// Down in the entry list: the keyboard goes to the row below.
        RowDown,
        /// Enter or Space on a row that has the keyboard: it opens.
        RowOpen,
    ]
);

/// This area's GPUI-only state, kept in `root.pages.library`.
#[derive(Default)]
pub struct State {
    /// The page's keyboard focus, so its keys (Ctrl+S, Escape on a form) reach it while nothing
    /// else holds the keyboard.
    focus: Option<FocusHandle>,
    /// The entry list's scroll, which a link's reveal moves.
    list_scroll: UniformListScrollHandle,
    /// The reader's scroll, which a typed jump to a heading moves.
    reader_scroll: gpui::ScrollHandle,
    /// The row of the entry list that Up and Down gave the keyboard to (egui's region: the
    /// keyboard moves row to row and the row shows whole), and its focus.
    pub(super) row: Option<usize>,
    pub(super) row_focus: Option<FocusHandle>,
    /// The entry list's row count and the open entry's row, as the list drew them.
    pub(super) list_rows: usize,
    pub(super) list_selected: Option<usize>,
    /// The New entry folder field's suggestions, read when its menu opens and kept while it
    /// stays open, for one root and kind (`catalogue::folder_suggestions`).
    pub(super) folders: Option<(std::path::PathBuf, String, Arc<Vec<String>>)>,
    /// The Markdown editor's state, by the file it edits, so the page can place its caret.
    pub(super) editor: Option<(String, kit::EditorHandle)>,
    /// The comparison's aligned rows, by the two texts (`design::comparison_rows`), and the
    /// change group `Next difference` went to.
    pub(super) comparison: Option<(u64, Arc<crate::design::ComparedLines>)>,
    pub(super) difference: Option<usize>,
}

/// Once, when the root is built.
pub(crate) fn init(root: &mut SplatRoot, _window: &mut Window, cx: &mut Context<SplatRoot>) {
    cx.bind_keys([
        KeyBinding::new("alt-left", Back, Some("Library")),
        KeyBinding::new("alt-right", Forward, Some("Library")),
        // A text field or the editor binds these keys in its own, deeper context first.
        KeyBinding::new("up", RowUp, Some("Library")),
        KeyBinding::new("down", RowDown, Some("Library")),
        KeyBinding::new("enter", RowOpen, Some("Library")),
        KeyBinding::new("space", RowOpen, Some("Library")),
    ]);
    root.pages.library.focus = Some(cx.focus_handle());
}

/// Once per frame, before anything draws: what the Library asked of the app (`Splat::show`'s
/// Library arm after `library.show`), handled as egui does after the page draws.
pub(crate) fn poll(root: &mut SplatRoot, _window: &mut Window, cx: &mut Context<SplatRoot>) {
    if root.app.page != Page::Library {
        return;
    }
    let app = &mut root.app;
    if let Some((id, name)) = app.library.renamed.take()
        && let Err(error) = app.campaigns.rename_module(id, name)
    {
        app.library.report_error(error);
    }
    if std::mem::take(&mut app.library.wants_settings) {
        if app.campaigns.all().is_empty() {
            app.campaigns.focus_add();
        }
        app.settings_tab = crate::SettingsTab::Account;
        app.page = Page::Settings;
        cx.notify();
    }
    if let Some(folder) = app.library.content_changed.take() {
        app.campaign_content.refresh_destination(&folder);
        app.encounters
            .refresh_content(&root.ctx, &app.campaigns, &folder);
        app.lookup.refresh();
    }
}

/// The link index the reader and the list read (`requests::LINK_INDEX`), as the frame logic
/// leaves it.
pub(crate) fn link_index(root: &SplatRoot) -> Option<Arc<crate::lookup::ContentIndex>> {
    root.ctx.get(crate::requests::LINK_INDEX)
}

/// The page's width: the window less the rail, the page's padding and a side panel.
pub(crate) fn page_width(root: &SplatRoot, window: &Window) -> f32 {
    f32::from(root.page_width(window))
}

/// The page.
pub(crate) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let ctx = root.ctx.clone();
    {
        let app = &mut root.app;
        app.library.begin(&ctx, &mut app.campaigns);
    }
    // The page takes the keyboard while nothing else holds it, so its keys reach it.
    let focus = root
        .pages
        .library
        .focus
        .get_or_insert_with(|| cx.focus_handle())
        .clone();
    let unheld = window.focused(cx).is_none() || root.focus.is_focused(window);
    if unheld && !root.modal_open(cx) {
        window.focus(&focus, cx);
    }
    let width = page_width(root, window);
    let index = link_index(root);
    let header = header::render(root, &index, width, window, cx);
    let body = body(root, &index, width, window, cx);
    let unregister = forms::unregister_dialog(root, cx);
    div()
        .id("library-page")
        .key_context("Library")
        .track_focus(&focus)
        .on_action(cx.listener(|root, _: &Back, _, cx| {
            if root.modal_open(cx) {
                return;
            }
            root.app.library.back();
            cx.notify();
        }))
        .on_action(cx.listener(|root, _: &RowUp, window, cx| {
            if list_keys(root, window, cx) {
                list::move_row(root, -1, window, cx);
            }
        }))
        .on_action(cx.listener(|root, _: &RowDown, window, cx| {
            if list_keys(root, window, cx) {
                list::move_row(root, 1, window, cx);
            }
        }))
        .on_action(cx.listener(|root, _: &RowOpen, window, cx| {
            list::open_row(root, window, cx);
        }))
        .on_action(cx.listener(|root, _: &Forward, _, cx| {
            if root.modal_open(cx) {
                return;
            }
            root.app.library.forward();
            cx.notify();
        }))
        .on_mouse_down(
            gpui::MouseButton::Navigate(gpui::NavigationDirection::Back),
            cx.listener(|root, _, _, cx| {
                if !root.modal_open(cx) {
                    root.app.library.back();
                    cx.notify();
                }
            }),
        )
        .on_mouse_down(
            gpui::MouseButton::Navigate(gpui::NavigationDirection::Forward),
            cx.listener(|root, _, _, cx| {
                if !root.modal_open(cx) {
                    root.app.library.forward();
                    cx.notify();
                }
            }),
        )
        .on_action(cx.listener(dismiss))
        .flex()
        .flex_col()
        .size_full()
        .child(header)
        .child(div().flex_1().min_h_0().child(body))
        .children(unregister)
        .into_any_element()
}

/// Whether Up and Down move in the entry list now: the page or a list row holds the keyboard
/// and nothing modal is open. Else the key goes on to the control that holds it.
fn list_keys(root: &SplatRoot, window: &Window, cx: &mut Context<SplatRoot>) -> bool {
    let library = &root.pages.library;
    let held = |focus: &Option<FocusHandle>| focus.as_ref().is_some_and(|f| f.is_focused(window));
    let ours = window.focused(cx).is_none() || held(&library.focus) || held(&library.row_focus);
    if ours && !root.modal_open(cx) {
        true
    } else {
        cx.propagate();
        false
    }
}

/// Ctrl+S on this page (the root's one owner calls it, `pages::save_key`): the open draft.
pub(crate) fn save_key(root: &mut SplatRoot, _: &mut Context<SplatRoot>) {
    let app = &mut root.app;
    app.library.save(&app.campaigns);
}

/// Escape on the page: a form takes it (it closes), as in egui; else the root's Escape.
fn dismiss(
    root: &mut SplatRoot,
    _: &crate::gui::Dismiss,
    _: &mut Window,
    cx: &mut Context<SplatRoot>,
) {
    let library = &mut root.app.library;
    let (package_form, new_form) = library.forms();
    if kit::menu::any_open(cx) || kit::dialog::any_open(cx) || !(package_form || new_form) {
        cx.propagate();
        return;
    }
    if package_form {
        library.close_package_form();
    } else {
        library.close_new_entry();
    }
    cx.notify();
}

/// Everything under the header: the load, its fault, the empty campaign scope, or the columns.
fn body(
    root: &mut SplatRoot,
    index: &Option<Arc<crate::lookup::ContentIndex>>,
    width: f32,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = theme::colors();
    let library = &root.app.library;
    if library.loading() {
        return div()
            .flex()
            .flex_row()
            .items_center()
            .gap(theme::S)
            .child(kit::icon(
                crate::gui::icons::BUSY,
                theme::Type::Body,
                c.muted,
            ))
            .child(kit::text(theme::Type::Body, c.text, "Loading the Library…"))
            .into_any_element();
    }
    if !library.loaded() {
        // Nothing loaded: there is no pane, so a failed load says so here.
        return match library.load_fault() {
            Some((tone, words)) => kit::Message::new(footer::tone(tone), words).into_any_element(),
            None => div().into_any_element(),
        };
    }
    if library.empty_scope() {
        return list::empty_scope(root, width, window, cx);
    }
    let compact = width < 900.0 || root.app.library.disk_changed();
    let lane = theme::SCROLL_BAR_WIDTH;
    let sections_width = f32::from(px(166.0) + lane + theme::S);
    // The list takes a share of the room the sections column leaves (`entries_column`).
    let rest = if compact {
        width
    } else {
        width - sections_width
    };
    let list_width = (rest * 0.38).clamp(220.0, 420.0);
    let list = {
        let app = &mut root.app;
        app.library
            .list_view(&app.campaigns, index.as_deref(), compact, rest)
    };
    let Some(list) = list else {
        return div().into_any_element();
    };
    // The reader's room: what the columns before it leave.
    let reader_width = rest - if list.list_hidden { 0.0 } else { list_width } - f32::from(theme::L);
    let reader = reader::pane(root, index, reader_width, window, cx);
    let sections = (!compact).then(|| list::sections_column(root, &list, cx));
    let entries =
        (!list.list_hidden).then(|| list::entries_column(root, &list, index, compact, window, cx));
    let reader = div()
        .size_full()
        .pl(if root.app.library.disk_changed() {
            px(0.0)
        } else {
            theme::L
        })
        .child(reader);
    let layout = match (&sections, &entries) {
        (Some(_), Some(_)) => "library-columns-3",
        (None, Some(_)) => "library-columns-list",
        (Some(_), None) => "library-columns-sections",
        (None, None) => "library-columns-1",
    };
    let mut columns = kit::Columns::new(layout);
    if let Some(sections) = sections {
        columns = columns
            .column(
                px(166.0) + lane + theme::S,
                px(150.0) + lane + theme::S..px(280.0) + lane + theme::S,
                div().size_full().pr(theme::S).child(sections),
            )
            .saved("library_sections");
    }
    if let Some(entries) = entries {
        columns = columns
            .column(
                px(list_width),
                px(220.0)..px(560.0),
                div().size_full().px(theme::S).child(entries),
            )
            .saved("library_entries");
    }
    columns.rest(reader).into_any_element()
}

/// A handler with no event (a field's Enter, a dialog's Escape) that acts on the root.
pub(super) fn act(
    cx: &mut Context<SplatRoot>,
    f: impl Fn(&mut SplatRoot, &mut Window, &mut Context<SplatRoot>) + 'static,
) -> impl Fn(&mut Window, &mut App) + 'static {
    let root = cx.entity().downgrade();
    move |window, cx| {
        let _ = root.update(cx, |root, cx| f(root, window, cx));
    }
}

/// The copy dialog (`copy_dialog::CopyDialog`), while `root.app.copy_dialog` is open.
pub(crate) fn render_copy_dialog(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    copy::render(root, window, cx)
}
