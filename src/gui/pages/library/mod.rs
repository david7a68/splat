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
        /// Ctrl+S: save the open draft.
        Save,
        /// Alt+Left: back to the entry before the last link.
        Back,
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
}

/// Once, when the root is built.
pub(crate) fn init(root: &mut SplatRoot, _window: &mut Window, cx: &mut Context<SplatRoot>) {
    cx.bind_keys([
        KeyBinding::new("ctrl-s", Save, Some("Library")),
        KeyBinding::new("alt-left", Back, Some("Library")),
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
            .refresh_content(root.bridge.ctx(), &app.campaigns, &folder);
        app.lookup.refresh();
    }
}

/// The link index the reader and the list read (`reader_link_index`), as the Encounters page's
/// content refresh leaves it.
pub(crate) fn link_index(root: &SplatRoot) -> Option<Arc<crate::lookup::ContentIndex>> {
    root.bridge.ctx().data(|data| {
        data.get_temp::<Arc<crate::lookup::ContentIndex>>(eframe::egui::Id::new(
            "reader_link_index",
        ))
    })
}

/// The page's width: the window less the rail, the page's padding and a side panel.
pub(crate) fn page_width(root: &SplatRoot, window: &Window) -> f32 {
    let window = window.viewport_size().width;
    let rail = if window >= theme::RAIL_WIDE_FROM {
        theme::RAIL_WIDE_WIDTH
    } else {
        theme::RAIL_WIDTH
    };
    let side =
        root.app.lookup.open || (root.app.sessions.notes_open && root.app.page != Page::Sessions);
    let panel = if side {
        theme::PANEL_MIN_WIDTH
    } else {
        px(0.0)
    };
    f32::from(window - rail - panel - theme::M * 2.0)
}

/// The page.
pub(crate) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let ctx = root.bridge.ctx().clone();
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
        .on_action(cx.listener(|root, _: &Save, _, cx| {
            if root.modal_open(cx) {
                return;
            }
            let app = &mut root.app;
            app.library.save(&app.campaigns);
            cx.notify();
        }))
        .on_action(cx.listener(|root, _: &Back, _, cx| {
            if root.modal_open(cx) {
                return;
            }
            root.app.library.back();
            cx.notify();
        }))
        .on_action(cx.listener(dismiss))
        .flex()
        .flex_col()
        .size_full()
        .child(header)
        .child(div().flex_1().min_h_0().child(body))
        .children(unregister)
        .into_any_element()
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
        columns = columns.column(
            px(166.0) + lane + theme::S,
            px(150.0) + lane + theme::S..px(280.0) + lane + theme::S,
            div().size_full().pr(theme::S).child(sections),
        );
    }
    if let Some(entries) = entries {
        columns = columns.column(
            px(list_width),
            px(220.0)..px(560.0),
            div().size_full().px(theme::S).child(entries),
        );
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
