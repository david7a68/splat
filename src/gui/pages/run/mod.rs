//! The live run: the Run page and the run strip on other pages (state: `src/encounter_run.rs`,
//! `src/encounter_run/`, `src/combat_sync.rs`, `src/rolls.rs`, `src/roll_table.rs`).
//!
//! The run's state is `encounter_run::Run` in `root.app.encounters`; this view reads it and calls
//! its methods. Each phase has its own body: `setup` (Setup), `live`
//! (Running) and `ended` (Ended), with the run's own panel (`panel`) beside them. The keys are in
//! `keys`; the footer's second steps and the close dialog are in `footer`.
//!
//! Each frame: `poll` takes the requests the last frame's acts left (`Splat::run_page_end`),
//! saves the run (`Run::autosave`) and gives the page the keyboard when nothing has it; `render`
//! tells the run the app's facts (`Splat::run_page_begin`) and draws the phase.
#[cfg(test)]
mod captures;
mod dock;
mod ended;
mod footer;
mod header;
mod keys;
mod live;
mod measure;
mod panel;
mod roster;
mod setup;
mod stat_block;
mod strip;
#[cfg(test)]
mod tests;

use crate::encounter_run::{Command, Phase, Run, view};
use crate::gui::{SplatRoot, kit, theme};
use crate::{Page, design};
use gpui::{
    AnyElement, App, ClickEvent, Context, FocusHandle, InteractiveElement, IntoElement,
    ParentElement, Pixels, Styled, Window, div, prelude::*, px,
};
use theme::{Type, colors};

pub(crate) use strip::render_strip;

/// This area's GPUI-only state, kept in `root.pages.run`.
#[derive(Default)]
pub struct State {
    /// The page's keyboard focus: the run's letters and digits reach it while no field has it.
    focus: Option<FocusHandle>,
    /// The page's width in the last frame, so the next frame's facts use the room it had.
    width: Option<Pixels>,
    /// The live roster's scroll, kept so the selected row can be brought into view and the
    /// overflow line can say what is out of it.
    roster_scroll: gpui::ScrollHandle,
    /// The window height the roster was last laid out for: a new one scrolls to the selection.
    roster_room: Option<Pixels>,
    /// The height the live roster's box had when it was last laid out (its rows and the
    /// overflow line), which the next frame fits its rows to.
    roster_box: std::rc::Rc<std::cell::Cell<f32>>,
    /// Lookup or Notes was open beside the run last frame.
    side_open: bool,
}

/// Once, when the root is built.
pub(crate) fn init(root: &mut SplatRoot, _window: &mut Window, cx: &mut Context<SplatRoot>) {
    root.pages.run.focus = Some(cx.focus_handle());
}

/// Once per frame, before anything draws: the requests the last frame's acts left, the save,
/// and the keyboard for the page when nothing else has it.
pub(crate) fn poll(root: &mut SplatRoot, window: &mut Window, cx: &mut Context<SplatRoot>) {
    if root.app.page != Page::Run {
        return;
    }
    stat_block::poll(root, cx);
    root.app.run_page_end(false);
    if let Some(run) = root.app.encounters.run_mut() {
        run.refresh_portraits();
        run.autosave();
    }
    // When Lookup or Notes closes, the keyboard comes back to the run, so its letters work
    // again (the closed panel's field no longer holds it); a hidden editor keeps no key.
    let side_open = root.app.lookup.open || root.app.sessions.notes_open;
    let closed = std::mem::replace(&mut root.pages.run.side_open, side_open) && !side_open;
    if let Some(focus) = &root.pages.run.focus
        && (closed
            || window
                .focused(cx)
                .is_none_or(|focused| focused == root.focus))
    {
        window.focus(focus, cx);
    }
}

/// The page.
pub(crate) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    if root.app.campaigns.active_campaign().is_none() {
        root.app.page = Page::Encounters;
        cx.notify();
        return div().into_any_element();
    }
    let window_width = window.viewport_size().width;
    let room = root
        .pages
        .run
        .width
        .unwrap_or_else(|| page_width(root, window));
    root.pages.run.width = Some(page_width(root, window));
    if !root
        .app
        .run_page_begin(f32::from(room), f32::from(window_width))
    {
        root.app.page = Page::Encounters;
        cx.notify();
        return div().into_any_element();
    }
    let Some(phase) = root.app.encounters.run().map(|run| run.state.phase.clone()) else {
        return div().into_any_element();
    };
    let body = match phase {
        Phase::Setup => setup::render(root, window, cx),
        Phase::Running => live::render(root, window, cx),
        Phase::Ended { .. } => ended::render(root, window, cx),
    };
    let dialog = footer::close_dialog(root, window, cx);
    // Each focus request is for the frame that drew its field.
    if let Some(run) = root.app.encounters.run_mut() {
        run.focus_amount = false;
        run.focus_outcome = false;
        run.focus_field = None;
        run.focus_other_effect = false;
        run.ad_hoc.focus_hp = false;
        for field in run.effect_fields.values_mut() {
            field.focus = false;
        }
    }
    let mut page = div()
        .id("run-page")
        .key_context("RunPage")
        .on_key_down(cx.listener(keys::key_down))
        .flex()
        .flex_col()
        .size_full()
        .child(body)
        .children(dialog);
    if let Some(focus) = &root.pages.run.focus {
        page = page.track_focus(focus);
    }
    page.into_any_element()
}

/// The page's width: the window less the rail, the page's padding and a side panel.
fn page_width(root: &SplatRoot, window: &Window) -> Pixels {
    root.page_width(window)
}

/// The run panel's natural width beside the roster: the stat block's share
/// of the window, or more when the roster would pass its largest width.
fn panel_width(room: Pixels, window: Pixels) -> Pixels {
    let share = px(design::side_panel_width(f32::from(window)));
    share.max(room - px(theme::RUN_LARGE.max_width) - theme::M)
}

/// The run panel's columns, by the ids their widths are saved under (`ui_state`).
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Side {
    /// The live view's panel.
    Live,
    /// Setup's panel.
    Setup,
    /// The ended view's Log.
    Ended,
}

impl Side {
    fn id(self) -> &'static str {
        match self {
            Self::Live => "run_panel",
            Self::Setup => "run_setup_panel",
            Self::Ended => "run_ended_log",
        }
    }
}

/// The run panel's column (`page_keeps(RUN_LIST_KEEPS)`): at least
/// `PANEL_MIN_WIDTH`, at most `PANEL_DRAG_MAX_WIDTH`, and the run's list keeps 454 px. The DM's
/// width is kept with the other columns' (`kit::edge`), here and after a restart; its splitter
/// takes a drag, Left and Right (`L` a press) and Home (the natural width again).
pub(super) fn side_column(side: Side, room: Pixels, window: Pixels) -> kit::EdgeColumn {
    kit::EdgeColumn::new(
        side.id(),
        kit::edge::Side::Right,
        panel_width(room, window),
        theme::PANEL_MIN_WIDTH..=theme::PANEL_DRAG_MAX_WIDTH,
    )
    .page_keeps(theme::RUN_LIST_KEEPS)
    .name("fight panel")
}

/// The panel's width in `room`: the DM's dragged width, else the natural width, inside its
/// range and the page.
pub(super) fn side_width(side: Side, room: Pixels, window: Pixels, cx: &App) -> Pixels {
    side_column(side, room, window).width(room, cx)
}

/// Before a phase draws its panel: Arrivals' kept fight words (`arrival_words`) and its
/// footer's lines (`arrival_lines`), which the run works out and keeps as it reads them.
fn arrivals_prepare(
    root: &mut SplatRoot,
) -> Vec<crate::design::outcome::plain::PlainLine<view::ArrivalPress>> {
    let Some(run) = root
        .app
        .encounters
        .run_mut()
        .filter(|run| run.panel == crate::encounter_run::Panel::Groups && !run.hide_panel)
    else {
        return Vec::new();
    };
    run.keep_offered_lines();
    let _ = run.arrival_words();
    let frame = design::outcome::PaneFrame::new();
    let mut messages = design::outcome::Messages::new();
    run.arrival_lines(&mut messages, &frame);
    messages.into_plain()
}

/// A click handler that acts on the run, then draws again.
fn on_run(cx: &mut Context<SplatRoot>, act: impl Fn(&mut Run) + 'static) -> OnClick {
    Box::new(
        cx.listener(move |root: &mut SplatRoot, _: &ClickEvent, _, cx| {
            if let Some(run) = root.app.encounters.run_mut() {
                act(run);
            }
            cx.notify();
        }),
    )
}

/// A boxed handler with no event (a field's Enter, a dialog's Escape).
type OnPlain = Box<dyn Fn(&mut Window, &mut App)>;

/// A boxed click handler: it borrows nothing, so a builder can make several in one chain.
type OnClick = Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// A click handler that runs one command (`run_command`).
fn on_command(cx: &mut Context<SplatRoot>, command: Command) -> OnClick {
    on_run(cx, move |run| run.run_command(command.clone()))
}

/// The DM closed the run: it goes, and the page goes back to where the run was opened.
fn closed(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) {
    if let Some(campaign) = root.app.campaigns.active_campaign() {
        root.app.encounters.close_shown_run(&campaign);
    }
    root.app.run_page_end(true);
    cx.notify();
}

/// The two-line title of setup and the ended view: `Run · <name>` over the state.
fn run_title(run: &Run, what: impl Into<gpui::SharedString>) -> gpui::Div {
    let c = colors();
    div()
        .flex()
        .flex_col()
        .min_w_0()
        .child(kit::text::one_line(
            Type::Small,
            c.muted,
            format!("Run · {}", run.name),
        ))
        .child(kit::text::one_line(Type::Title, c.text, what))
}

/// A participant's picture (`kit::portrait`); absent participants are dimmed.
fn portrait(path: Option<std::path::PathBuf>, name: &str, size: Pixels, dim: bool) -> gpui::Div {
    kit::portrait(path.as_deref(), name, size).when(dim, |circle| circle.opacity(0.5))
}

/// The design status as the kit's.
fn status(status: &design::Status<'_>) -> kit::Status {
    kit::Status::of(status)
}

/// The save state beside Keys and at the end of each footer: `Saved`, `Not saved`, the copy's
/// label, and the file's path on hover (`save_status`).
fn save_status(run: &Run) -> impl IntoElement {
    let value = run.save_status_value();
    let word = run.copy_label.as_ref().map_or_else(
        || value.word.to_owned(),
        |copy| format!("{} · {copy}", value.word),
    );
    let hover = run.path().map(|path| path.display().to_string());
    let shown = kit::Status::new(value.icon, word, theme::color(value.color));
    let mut element = div()
        .id("run-save-status")
        .flex_none()
        .child(kit::status(&shown));
    if let Some(hover) = hover {
        element = kit::WithTooltip::with_tooltip(element, hover);
    }
    element
}

/// An Undo action with the run's rules: its key while the keys work, blocked with the reason
/// when there is nothing to undo (`undo_action`).
fn undo_button(run: &Run, id: &'static str, words: String, density: theme::Density) -> kit::Button {
    undo_button_keyed(run, id, words, density, true)
}

/// `undo_button`, whose key letter may give up its room (`key` false) before its words shorten.
fn undo_button_keyed(
    run: &Run,
    id: &'static str,
    words: String,
    density: theme::Density,
    key: bool,
) -> kit::Button {
    let label = run.undo_label();
    let button = kit::Button::secondary(id, words)
        .icon(crate::gui::icons::UNDO)
        .density(density)
        .name(label.clone())
        .hint(label);
    let button = if key && matches!(run.state.phase, Phase::Running) && !run.lookup_open {
        button.key("U")
    } else {
        button
    };
    button.blocked(
        run.history
            .is_empty()
            .then(|| run.empty_undo_reason().to_owned()),
    )
}

/// A handler with no event (a field's Enter, a dialog's Escape) that acts on the root.
fn plain(
    cx: &mut Context<SplatRoot>,
    act: impl Fn(&mut SplatRoot, &mut Window, &mut Context<SplatRoot>) + 'static,
) -> OnPlain {
    let listener =
        cx.listener(move |root: &mut SplatRoot, _: &(), window, cx| act(root, window, cx));
    Box::new(move |window, cx| listener(&(), window, cx))
}

/// A number field's words: only the characters of a whole number (`design::number_char`).
fn number_text(words: &str) -> String {
    words.chars().filter(|c| design::number_char(*c)).collect()
}
