//! The Session page, the notes panel beside other pages, and the session banner (egui:
//! `src/sessions/`, `src/sessions.rs`, `src/sessions_ui/`, `src/session_records/`).
//!
//! The page's state is `root.app.sessions` (`crate::sessions::page::Sessions`), the same the
//! egui page draws. The view reads it through `Sessions::gv_*` (`src/sessions/gpui_facade.rs`)
//! and acts through the same methods the egui page calls; this folder keeps only GPUI state.
mod banner;
mod footer;
mod form;
mod list;
mod note;
mod panel;
mod plan;
mod recordings;
mod session;
mod slash;
mod templates;
#[cfg(test)]
mod tests;

use crate::Splat;
use crate::gui::{SplatRoot, icons, kit, theme};
use crate::processing::settings::ProcessingSettings;
use crate::sessions::page::{Plan, Sessions};
use crate::timeline::Book;
use gpui::{AnyElement, Context, IntoElement, ParentElement, Pixels, Styled, Window, div, px};
use theme::{L, M, S, Type, colors};

/// The list's default width and the widths the DM can drag it to (`LIST_WIDTH`, `LIST_WIDTHS`).
const LIST_WIDTH: Pixels = px(216.0);
const LIST_MIN: Pixels = px(170.0);
const LIST_MAX: Pixels = px(420.0);
/// The list stands beside the session from this page width (`TWO_COLUMNS_FROM`).
const TWO_COLUMNS_FROM: Pixels = px(640.0);
/// The plan and the note stand side by side from this pane width (`PLAN_BESIDE_NOTE_FROM`).
const PLAN_BESIDE_NOTE_FROM: Pixels = px(720.0);
const PLAN_WIDTH: Pixels = px(330.0);
const PLAN_MAX_WIDTH: Pixels = px(480.0);

/// This area's GPUI-only state, kept in `root.pages.sessions`.
#[derive(Default)]
pub struct State {
    /// The note editor's last words as the editor sent them, by session, so a change the
    /// program made (a load, an act) replaces the editor's words and the DM's typing never does.
    pub(super) note: note::State,
    /// The New template form just opened: its name field takes the keyboard once.
    pub(super) focus_template_name: bool,
    /// The plan's scroll and what its last frame measured.
    pub(super) plan: plan::State,
    /// The plan's Add menus (Fight, Group) were open last frame: a search takes the keyboard
    /// as its menu opens.
    pub(super) add_menu_open: [bool; 2],
}

/// Once, when the root is built.
pub(crate) fn init(root: &mut SplatRoot, window: &mut Window, cx: &mut Context<SplatRoot>) {
    note::init(root, window, cx);
}

/// Once per frame, before anything draws.
pub(crate) fn poll(root: &mut SplatRoot, window: &mut Window, _cx: &mut Context<SplatRoot>) {
    // The width test of the page as it will draw, for `Ctrl+\` (`list_toggle_key`).
    let width = page_width(root, window);
    root.app
        .sessions
        .gv_set_list_beside(width >= TWO_COLUMNS_FROM);
}

/// The page's room: the window, less the rail, the side panel and the page's padding.
pub(super) fn page_width(root: &SplatRoot, window: &Window) -> Pixels {
    root.page_width(window)
}

/// Run `act` with the page's state and what the rest of the app tells it about fights
/// (`sessions::page::Plan`). `None` without a campaign.
pub(super) fn with_plan<R>(
    app: &mut Splat,
    act: impl FnOnce(&mut Sessions, &ProcessingSettings, &mut Book, &Plan<'_>) -> R,
) -> Option<R> {
    let campaign = app.campaigns.active_campaign()?.clone();
    let kept = app.encounters.plan_kept();
    let (fights, groups) = (&kept.fights, &kept.groups);
    let fights_revision = app.encounters.plan_fights_revision();
    let saved_runs = app.encounters.saved_runs();
    let saved_runs_revision = app.encounters.saved_runs_revision();
    let encounters = &app.encounters;
    let blocked = encounters.plan_blocked();
    let group_blocked = encounters.plan_group_blocked();
    let group_open = encounters.group_open_in_plan();
    let arrivals_open = encounters.run_takes_arrivals();
    let run_directory = encounters.run_directory(&campaign);
    let open_state = encounters
        .run()
        .and_then(crate::encounter_run::Run::live_info)
        .map(|run| crate::encounters::run_state_word(&run, false))
        .unwrap_or_default();
    // The group editor in the plan is the Encounters page area's view; the GPUI plan draws its
    // own place for it (`plan::group_editor`), so this one draws nothing.
    let group_discard: &crate::sessions::page::GroupDiscardLine<'_> =
        &|frame| encounters.group_discard_line(frame);
    let party = app.party.order();
    let sources = app.lookup.index().map(|index| index.loot_sources());
    let plan = Plan {
        party: &party,
        tables: sources.as_ref().map_or(&[], |sources| &sources.tables),
        fights,
        fights_revision,
        saved_runs: &saved_runs,
        saved_runs_revision,
        groups,
        group_blocked,
        group_open,
        group_discard,
        arrivals_open,
        blocked,
        run_directory,
        open_run: app.run_fact.as_ref().map(|fact| fact.id),
        open_state,
        open_encounter: app.run_fact.as_ref().map(|fact| fact.name.as_str()),
    };
    Some(act(
        &mut app.sessions,
        &app.processing,
        &mut app.timeline,
        &plan,
    ))
}

/// The page.
pub(crate) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    root.app.sessions.gv_follow_context();
    let width = page_width(root, window);
    let sessions = &root.app.sessions;
    let folder_missing = sessions.folder_missing();
    let templates = sessions.gv_templates_shown();
    let header = header(root, width, window, cx);
    let body = if folder_missing {
        folder_fault(root, cx)
    } else if templates {
        templates::render(root, window, cx)
    } else {
        notes_view(root, width, window, cx)
    };
    let dialog = recordings::delete_dialog(root, cx);
    div()
        .flex()
        .flex_col()
        .size_full()
        .child(header)
        .child(div().flex_1().min_h_0().child(body))
        .children(dialog)
        .into_any_element()
}

/// The page's title line (`Sessions::show`): the title and count, its actions after them, and
/// the session list's toggle at the line's right end.
fn header(
    root: &mut SplatRoot,
    width: Pixels,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let sessions = &root.app.sessions;
    let templates = sessions.gv_templates_shown();
    let folder_missing = sessions.folder_missing();
    let (title, count, one, many) = if templates {
        (
            "Templates",
            sessions.gv_templates().len(),
            "template",
            "templates",
        )
    } else {
        ("Session", sessions.gv_notes().len(), "session", "sessions")
    };
    let context = (count > 0 && !folder_missing).then(|| crate::design::count(count, one, many));
    let in_form = sessions.gv_form_open();
    let empty = sessions.gv_notes().is_empty();
    let locked = sessions.gv_locked();
    let list_alone = sessions.gv_list_only() && width + rail_and_padding() < px(1500.0);
    let mut actions: Vec<AnyElement> = Vec::new();
    if !templates && !in_form && !empty && !folder_missing {
        actions.push(
            kit::Button::secondary("session-new", "New session")
                .icon(icons::ADD)
                .blocked(locked.then_some("Other sessions wait until you choose what to keep."))
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.sessions.gv_open_form();
                    cx.notify();
                }))
                .into_any_element(),
        );
    }
    if templates && !sessions.gv_new_template_open() {
        actions.push(
            kit::Button::secondary("template-new", "New template")
                .icon(icons::ADD)
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.sessions.gv_new_template();
                    root.pages.sessions.focus_template_name = true;
                    cx.notify();
                }))
                .into_any_element(),
        );
    }
    if sessions.recording() && (templates || in_form || list_alone || empty) {
        actions.push(
            kit::Button::quiet("header-stop-recording", "Stop recording")
                .icon(icons::STOP_RECORDING)
                .on_click(cx.listener(|root, _, _, cx| {
                    let ctx = root.ctx.clone();
                    root.app.sessions.gv_stop_recording(&ctx);
                    cx.notify();
                }))
                .into_any_element(),
        );
    }
    if !templates {
        actions.push(
            kit::Button::quiet("session-templates", "Templates")
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.sessions.gv_show_templates();
                    cx.notify();
                }))
                .into_any_element(),
        );
    }
    if sessions.party_hidden {
        actions.push(
            kit::Button::quiet("session-show-party", "Show party")
                .icon(icons::PARTY)
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.sessions.party_bar = Some(true);
                    cx.notify();
                }))
                .into_any_element(),
        );
    }
    let toggle = sessions
        .gv_offers_list_toggle(width >= TWO_COLUMNS_FROM)
        .then(|| {
            let hidden = sessions.list_hidden;
            kit::IconButton::new(
                "session-list-toggle",
                icons::SESSION_LIST,
                crate::sessions::page::LIST_TOGGLE,
            )
            .hint(if hidden {
                "Show the session list (Ctrl+\\)"
            } else {
                "Hide the session list (Ctrl+\\)"
            })
            .toggled(!hidden)
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.sessions.toggle_list();
                cx.notify();
            }))
        });
    let mut heading = div()
        .flex()
        .flex_row()
        .items_baseline()
        .gap(S + theme::XS)
        .flex_none()
        .child(kit::text(Type::Title, c.text, title).whitespace_nowrap());
    if let Some(context) = context {
        heading = heading.child(kit::text(Type::Small, c.muted, context).whitespace_nowrap());
    }
    div()
        .flex()
        .flex_col()
        .w_full()
        .flex_none()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .min_h(theme::Density::Scan.control())
                .gap(L)
                .child(heading)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .flex_1()
                        .items_center()
                        .gap(S)
                        .children(actions),
                )
                .children(toggle),
        )
        .child(kit::rule().mt(theme::LINE_GAP))
        .child(kit::gap(L))
        .into_any_element()
}

/// The rail and the page padding, to turn the page's width back into the window's.
fn rail_and_padding() -> Pixels {
    theme::RAIL_WIDTH + M * 2.0
}

/// The session folder cannot be used: the page says why, with `Choose folder`.
fn folder_fault(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> AnyElement {
    let words = root.app.sessions.gv_folder_fault();
    div()
        .flex()
        .flex_col()
        .items_start()
        .max_w(theme::READER_MEASURE)
        .child(kit::Message::new(kit::Tone::Warning, words).action(
            kit::Button::secondary("session-choose-folder", "Choose folder").on_click(cx.listener(
                |root, _, _, cx| {
                    root.app.sessions.wants_campaign_link = true;
                    cx.notify();
                },
            )),
        ))
        .into_any_element()
}

/// The notes view (`notes_view`): the empty state, the first form, or the list beside the
/// session (or, in a narrow page, one of them).
fn notes_view(
    root: &mut SplatRoot,
    width: Pixels,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let sessions = &root.app.sessions;
    let holding = sessions.gv_locked();
    if sessions.gv_notes().is_empty() && !sessions.gv_form_open() && !holding {
        let open_run = sessions.gv_open_run();
        let button = if open_run {
            kit::Button::secondary("session-empty-new", "New session")
        } else {
            kit::Button::primary("session-empty-new", "New session")
        }
        .on_click(cx.listener(|root, _, _, cx| {
            root.app.sessions.gv_open_form();
            cx.notify();
        }));
        return kit::empty_state(
            icons::SESSIONS,
            "No session notes yet",
            "Start a session from a template. It holds tonight’s plan and your notes.",
            Some(button),
        )
        .into_any_element();
    }
    if sessions.gv_notes().is_empty() && !holding {
        return form::render(root, window, cx);
    }
    let beside = width >= TWO_COLUMNS_FROM;
    if !beside {
        let sessions = &root.app.sessions;
        if sessions.gv_list_only() && !sessions.gv_form_open() {
            return list::render(root, cx);
        }
        let form_open = sessions.gv_form_open();
        let back = (!form_open).then(|| {
            kit::Button::quiet("session-all", "All sessions")
                .icon(icons::HISTORY_BACK)
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.sessions.gv_set_list_only(true);
                    cx.notify();
                }))
        });
        let body = if form_open {
            form::render(root, window, cx)
        } else {
            session::render(root, width, window, cx)
        };
        return div()
            .flex()
            .flex_col()
            .size_full()
            .children(back)
            .child(div().flex_1().min_h_0().child(body))
            .into_any_element();
    }
    let list_hides = root.app.sessions.gv_list_hides();
    let session_width = if list_hides {
        width
    } else {
        width - LIST_WIDTH - S - theme::SPLITTER_WIDTH
    };
    let body = if root.app.sessions.gv_form_open() {
        form::render(root, window, cx)
    } else {
        session::render(root, session_width, window, cx)
    };
    if list_hides {
        return body;
    }
    let list = div().size_full().pr(S).child(list::render(root, cx));
    kit::Columns::new("session-columns")
        .column(LIST_WIDTH, LIST_MIN..LIST_MAX, list)
        .saved("session_list_panel")
        .rest(div().size_full().pl(S).child(body))
        .into_any_element()
}

/// Ctrl+S on the Session page (`pages::save_key`): the shown template's draft; a group open in
/// the plan while the note does not have the keyboard (`Encounters::group_open_in_plan`); else
/// the note, which saves at once.
pub(crate) fn save_key(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) {
    let sessions = &mut root.app.sessions;
    if sessions.gv_templates_shown() {
        sessions.gv_save_template();
    } else if root.app.encounters.group_open_in_plan() && !root.app.sessions.gv_note_had_focus() {
        super::encounters::save_key(root, cx);
    } else {
        root.app.sessions.gv_save_note();
    }
}

/// An act of the session plan from outside its rows: an open journey's checks block, which the
/// Timeline's view draws for the plan (`timeline::checks::Host::Plan`).
pub(crate) fn travel_act(root: &mut SplatRoot, act: crate::sessions::page::gpui_facade::PlanAct) {
    plan::act(root, act);
}

/// The note in the side panel beside another page (`Sessions::notes_panel`).
pub(crate) fn render_notes_panel(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    panel::render(root, window, cx)
}

/// The session banner above every page (`Sessions::banner`: recording and processing notices),
/// or `None` when it has nothing to say.
pub(crate) fn render_banner(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    banner::render(root, window, cx)
}
