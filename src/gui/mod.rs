//! The GPUI app: the window, the root view (`SplatRoot`) and the shell around the pages (the rail,
//! the page switch, the run strip, the side panels, the close prompt, the global keys and the
//! background poll). `docs/development.md` (How the app runs) and `docs/design-system.md` (Kit)
//! describe it.
//!
//! The root owns the app's state (`crate::Splat`), its state context
//! (`crate::state_context::StateContext`) and the GPUI state of each page (`pages::States`).
//! Pages read the state and call its methods from event handlers (`cx.listener`); the logic stays
//! in the state types.
//!
//! Each frame (`SplatRoot::render`) first runs the frame logic that draws nothing: background
//! polls, the pages' `poll` hooks, cross-page requests (`Splat::route`), the party sync, the run's
//! place in the Timeline, the page change. Between frames one timer task reads the context's
//! waker and runs the frame logic only when background work asked for a frame.

mod close;
#[cfg(test)]
mod gallery;
#[cfg(test)]
pub(crate) mod harness;
#[allow(
    dead_code,
    reason = "the foundation of the kit: parts that no page calls yet"
)]
pub mod icons;
#[allow(
    dead_code,
    unused_imports,
    reason = "the foundation of the kit: parts that no page calls yet"
)]
pub mod kit;
pub mod pages;
mod rail;
#[cfg(test)]
mod tests;
#[allow(
    dead_code,
    reason = "the foundation of the kit: parts that no page calls yet"
)]
pub mod theme;
mod touchpad;

use crate::state_context::StateContext;
use crate::{CampaignTab, Page, SettingsTab, Splat};
use gpui::{
    AnyElement, App, AppContext, Bounds, Context, FocusHandle, InteractiveElement, IntoElement,
    KeyBinding, ParentElement, Render, Styled, Task, TitlebarOptions, Window, WindowBounds,
    WindowOptions, actions, div, prelude::*, px, size,
};
use std::time::Duration;
use theme::{M, S, colors};

actions!(
    splat,
    [
        /// Ctrl+K: Lookup beside the page.
        ToggleLookup,
        /// Ctrl+J: this session's notes beside the page.
        ToggleNotes,
        /// Ctrl+T: Trackers, a view of Lookup.
        ToggleTrackers,
        /// Ctrl+\: the Session page's list.
        ToggleSessionList,
        /// Escape: close the open layer, then the notes panel or Lookup.
        Dismiss,
        /// Ctrl+S: save the draft the DM is working on (`SplatRoot::save`).
        Save,
    ]
);

/// The window's default and smallest sizes.
pub const WINDOW_SIZE: (f32, f32) = (1200.0, 800.0);
pub const WINDOW_MIN_SIZE: (f32, f32) = (960.0, 640.0);

/// How often the background poll looks for reported work between frames.
const POLL_EVERY: Duration = Duration::from_millis(50);

/// Open the GPUI app's window and run until it closes.
pub fn run() {
    gpui_platform::application().run(|cx: &mut App| {
        init(cx);
        let bounds = Bounds::centered(None, size(px(WINDOW_SIZE.0), px(WINDOW_SIZE.1)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            window_min_size: Some(size(px(WINDOW_MIN_SIZE.0), px(WINDOW_MIN_SIZE.1))),
            titlebar: Some(TitlebarOptions {
                title: Some("Splat".into()),
                ..Default::default()
            }),
            app_id: Some("splat".to_owned()),
            ..Default::default()
        };
        let opened = cx.open_window(options, |window, cx| {
            let root = cx.new(|cx| SplatRoot::new(Splat::default(), window, cx));
            cx.new(|cx| gpui_component::Root::new(root, window, cx))
        });
        if let Err(error) = opened {
            eprintln!("Splat could not open its window: {error}");
            cx.quit();
        }
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
    });
}

/// Install the GPUI-wide parts: gpui-component, Splat's theme and fonts, and the global keys.
pub fn init(cx: &mut App) {
    gpui_component::init(cx);
    theme::install(cx);
    cx.bind_keys([
        KeyBinding::new("ctrl-k", ToggleLookup, None),
        KeyBinding::new("ctrl-j", ToggleNotes, None),
        KeyBinding::new("ctrl-t", ToggleTrackers, None),
        KeyBinding::new("ctrl-\\", ToggleSessionList, None),
        KeyBinding::new("escape", Dismiss, Some("SplatRoot")),
        KeyBinding::new("ctrl-s", Save, Some("SplatRoot")),
    ]);
}

/// What the rail says about the D&D Beyond account (`crate::Connection`).
fn connection(app: &Splat, session: bool) -> crate::Connection {
    if app.settings.connecting() {
        crate::Connection::Connecting
    } else if app.settings.refresh_failing() {
        crate::Connection::Failing
    } else if session {
        crate::Connection::Connected
    } else {
        crate::Connection::Offline
    }
}

/// The root view: the app's state, its state context, and every page's GPUI state.
pub struct SplatRoot {
    /// The app's state (`crate::Splat`).
    pub app: Splat,
    /// The context that the state's methods take: typed slots, the pass and the waker.
    pub ctx: StateContext,
    /// Each page's GPUI-only state (`pages::States`).
    #[allow(
        dead_code,
        reason = "each page keeps its GPUI state here; some fields wait for their page"
    )]
    pub pages: pages::States,
    /// The root's keyboard focus, so the global keys reach it with nothing else focused.
    pub focus: FocusHandle,
    /// The rail's and the side panel's widths in this frame (`measure_shell`), for `page_width`.
    shell_widths: (gpui::Pixels, Option<gpui::Pixels>),
    /// What `remember` last wrote, so an unchanged frame writes nothing.
    remembered: Option<crate::ui_state::UiState>,
    _poll: Option<Task<()>>,
    /// The keyboard comes back to the root when the focused element leaves the window
    /// (`SplatRoot::new`), and the root's Escape runs before any page's own Escape binding.
    _keys: Vec<gpui::Subscription>,
    /// Tests only: the kit gallery in place of the page.
    #[cfg(test)]
    pub(crate) gallery: Option<gallery::Gallery>,
}

impl SplatRoot {
    /// Build the root over `app` (a fresh `Splat::default()`, or a test fixture's), restore what
    /// this computer remembers, set up each page, and start the background poll.
    pub fn new(mut app: Splat, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let saved = app.restore();
        let mut root = Self {
            app,
            ctx: StateContext::new(),
            pages: pages::States::default(),
            focus: cx.focus_handle(),
            remembered: None,
            shell_widths: (theme::RAIL_WIDTH, None),
            _poll: None,
            _keys: Vec::new(),
            #[cfg(test)]
            gallery: None,
        };
        kit::edge::restore_column_widths(saved.column_widths, cx);
        window.focus(&root.focus, cx);
        // Before the pages' own interceptors, so the root sees a key before they act on it.
        root._keys = root.watch_keys(window, cx);
        pages::init(&mut root, window, cx);
        root.watch_close(window, cx);
        root._poll = Some(root.start_poll(window, cx));
        root
    }

    /// The window's keys reach the root wherever the keyboard was:
    /// - When the focused element leaves the window (a closed panel, a field a keyboard act
    ///   took away), GPUI sends keys to no view of Splat's. The root takes the keyboard back in
    ///   that draw, so Escape, Ctrl+K and the page's keys still work (egui read the global keys
    ///   whatever had focus).
    /// - Escape: the root's own order (an open menu, a dialog, the notes panel, Lookup) runs
    ///   before any binding of a page or a part, as egui's root consumed Escape before the page
    ///   drew. What the root does not take goes on to the page's binding, then to the focused
    ///   field's `on_escape`.
    fn watch_keys(&self, window: &mut Window, cx: &mut Context<Self>) -> Vec<gpui::Subscription> {
        let lost = cx.on_focus_lost(window, |root, window, cx| {
            window.focus(&root.focus, cx);
        });
        let root = cx.entity().downgrade();
        let this_window = window.window_handle();
        let escape = cx.intercept_keystrokes(move |event, window, cx| {
            let keystroke = &event.keystroke;
            if keystroke.key != "escape"
                || keystroke.modifiers.modified()
                || window.window_handle() != this_window
            {
                return;
            }
            // Every interceptor runs, even after one stopped the key: the note's `/` menu takes
            // its Escape itself (it runs after this one), so the root leaves it alone then.
            let took = root
                .update(cx, |root, cx| {
                    !pages::sessions::takes_escape(root, window, cx) && root.take_escape(window, cx)
                })
                .unwrap_or(false);
            if took {
                cx.stop_propagation();
            }
        });
        // Coming back from Obsidian or a file manager: the files are checked again, so a file
        // deleted or written there shows at once (`requests::WINDOW_ACTIVATED`).
        let activation = cx.observe_window_activation(window, |root, window, cx| {
            if window.is_window_active() {
                root.ctx
                    .set(splat_content_ui::requests::WINDOW_ACTIVATED, true);
                root.ctx.request_repaint();
                cx.notify();
            }
        });
        vec![lost, escape, activation]
    }

    /// The timer task between frames: when background work reported a change through the
    /// context's waker, run the frame logic and ask for a frame.
    fn start_poll(&self, window: &Window, cx: &mut Context<Self>) -> Task<()> {
        let waker = self.ctx.waker();
        cx.spawn_in(window, async move |this, cx| {
            loop {
                cx.background_executor().timer(POLL_EVERY).await;
                if !waker.take_due() {
                    continue;
                }
                let alive = this.update_in(cx, |root, window, cx| {
                    root.frame_logic(window, cx);
                    cx.notify();
                });
                if alive.is_err() {
                    break;
                }
            }
        })
    }

    /// The window's close button: an edit in its pause is saved now; anything that would be
    /// lost opens the close prompt instead (`close::prompt`).
    fn watch_close(&self, window: &mut Window, cx: &mut Context<Self>) {
        let root = cx.entity().downgrade();
        window.on_window_should_close(cx, move |window, cx| {
            root.update(cx, |root, cx| root.should_close(window, cx))
                .unwrap_or(true)
        });
    }

    /// Whether the window may close now.
    pub fn should_close(&mut self, _: &mut Window, cx: &mut Context<Self>) -> bool {
        let app = &mut self.app;
        app.sessions.flush();
        crate::timeline::checks::settle_length(&mut app.timeline, None);
        app.timeline.run_open = app.encounters.has_run();
        app.timeline.save();
        let loses = app.library.dirty()
            || app.world_page.dirty()
            || app.world_page.has_generated()
            || app.lookup.generated_name().is_some()
            || app.any_unsaved_edits()
            || app.campaign_content.dirty()
            || app.encounters.unsaved()
            || app.encounters.pending_sync()
            || app.party.unconfirmed()
            || app.sessions.recording()
            || app.sessions.processing()
            || app.sessions.busy_with_edit()
            || app.timeline.not_saved();
        if loses && !app.discard_on_close {
            app.confirm_close = true;
            cx.notify();
            return false;
        }
        true
    }

    /// Whether a modal is open: the close prompt or the copy dialog (`Splat::modal_open`).
    pub fn modal_open(&self, cx: &App) -> bool {
        self.app.copy_dialog.is_some()
            || self.app.confirm_close
            || kit::menu::any_open(cx)
            || kit::dialog::any_open(cx)
    }

    /// Go to a page (the rail, a link). The page change's follow-up runs in the next frame.
    pub fn show_page(&mut self, page: Page, cx: &mut Context<Self>) {
        self.app.page = page;
        cx.notify();
    }

    /// The frame logic that draws nothing. Runs at the start of every frame and from the
    /// background poll. Its two passes (`StateContext::begin_pass`) are before and after the
    /// pages' polls.
    pub fn frame_logic(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Self { app, ctx, .. } = self;
        ctx.begin_pass();
        app.campaign_content.tick(ctx, &app.campaigns);
        let (epoch, session) = app.poll_background(ctx);
        let modal = app.copy_dialog.is_some() || app.confirm_close;
        app.sync_party(ctx, epoch, session.clone());
        app.sessions.on_page = app.page == Page::Sessions;
        app.sessions.modal = modal;
        if !app.sessions.on_page {
            app.sessions.left_page();
        }
        // The pages' own polls (their GPUI state follows the app's).
        pages::poll(self, window, cx);
        let Self { app, ctx, .. } = self;
        ctx.begin_pass();
        if app.notes_gave_way && !app.lookup.open {
            app.notes_gave_way = false;
            app.sessions.notes_come_back();
        }
        app.apply_tracker_act();
        app.add_tracker_act_to_timeline();
        // A pressed dice number: the run rolls it, or it is a preview.
        app.route_rolls(ctx);
        app.route(ctx);
        app.follow_page_change();
        self.remember(cx);
    }

    /// Save what this computer remembers, when it changed. A drag (a splitter's) writes the file
    /// once, when it ends, and not on each frame of it.
    fn remember(&mut self, cx: &App) {
        if cx.has_active_drag() {
            return;
        }
        let now = self.app.ui_state_now(kit::edge::column_widths(cx));
        if self.remembered.as_ref() != Some(&now) {
            self.app.ui_store.save(&now);
            self.remembered = Some(now);
        }
    }

    /// The side panel beside the page, when one is open: Lookup, or the session's notes (not on
    /// the Session page, which has the note itself). Its natural width is the window's share
    /// (`design::side_panel_width`); the DM can drag it (`Column::panel`).
    fn side_column(&self, window: &Window) -> Option<kit::EdgeColumn> {
        let (key, name) = if self.app.lookup.open {
            ("lookup", "Lookup")
        } else if self.app.sessions.notes_open && self.app.page != Page::Sessions {
            ("session_notes", "session notes")
        } else {
            return None;
        };
        let natural = px(crate::design::side_panel_width(f32::from(
            window.viewport_size().width,
        )));
        Some(
            kit::EdgeColumn::new(
                key,
                kit::edge::Side::Right,
                natural,
                theme::PANEL_MIN_WIDTH..=theme::PANEL_DRAG_MAX_WIDTH,
            )
            .page_keeps(theme::PANEL_LEAVES_PAGE)
            .name(name),
        )
    }

    /// Measure the rail and the side panel for this frame, before anything lays out by them.
    fn measure_shell(&mut self, window: &Window, cx: &App) {
        let rail = rail::width(self, window, cx);
        let room = window.viewport_size().width - rail;
        let side = self
            .side_column(window)
            .map(|column| column.width(room, cx));
        self.shell_widths = (rail, side);
    }

    /// The rail's width in this frame.
    pub fn rail_width(&self) -> gpui::Pixels {
        self.shell_widths.0
    }

    /// The side panel's width in this frame (its border included), or `None` with no panel open.
    pub fn side_width(&self) -> Option<gpui::Pixels> {
        self.shell_widths.1
    }

    /// The page's own width: the window less the rail, the side panel and the page's sides. One
    /// rule for every page that lays out by its width.
    pub fn page_width(&self, window: &Window) -> gpui::Pixels {
        window.viewport_size().width
            - self.rail_width()
            - self.side_width().unwrap_or_default()
            - M * 2.0
    }

    /// The account's connection now.
    fn connection(&self) -> crate::Connection {
        let (_, session) = self.app.settings.campaign_session();
        connection(&self.app, session.is_some())
    }

    // The global keys (`update`'s shortcuts), each refused while a modal is open.

    fn toggle_lookup(&mut self, _: &ToggleLookup, _: &mut Window, cx: &mut Context<Self>) {
        if self.modal_open(cx) {
            return;
        }
        self.app.toggle_lookup();
        cx.notify();
    }

    fn toggle_notes(&mut self, _: &ToggleNotes, _: &mut Window, cx: &mut Context<Self>) {
        if self.modal_open(cx) {
            return;
        }
        self.app.toggle_notes();
        cx.notify();
    }

    fn toggle_trackers(&mut self, _: &ToggleTrackers, _: &mut Window, cx: &mut Context<Self>) {
        if self.modal_open(cx) {
            return;
        }
        self.app.notes_give_way();
        self.app.lookup.toggle_trackers();
        cx.notify();
    }

    fn toggle_session_list(
        &mut self,
        _: &ToggleSessionList,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.modal_open(cx) {
            return;
        }
        if self.app.page == Page::Sessions && self.app.sessions.list_toggle_key() {
            self.app.sessions.toggle_list();
            cx.notify();
        }
    }

    /// Ctrl+S, with one owner for the whole window, side panels before the page (each takes the
    /// key with `design::save_row::save_key`): nothing
    /// under a modal (the close prompt, the copy dialog, a kit dialog or menu); the Notes panel's
    /// note when it had the keyboard; else the page shown (`pages::save_key`). No page binds
    /// Ctrl+S itself, so the key saves the same thing wherever the keyboard is.
    fn save(&mut self, _: &Save, _: &mut Window, cx: &mut Context<Self>) {
        if self.modal_open(cx) {
            return;
        }
        let app = &mut self.app;
        let notes_panel = app.sessions.notes_open && app.page != Page::Sessions && !app.lookup.open;
        if notes_panel && app.sessions.gv_note_had_focus() {
            app.sessions.gv_save_note();
        } else {
            pages::save_key(self, cx);
        }
        cx.notify();
    }

    /// Escape as a binding (a part that dispatches `Dismiss`): the root's order
    /// (`take_escape`), else the focused view's own key listener gets it.
    fn dismiss(&mut self, _: &Dismiss, window: &mut Window, cx: &mut Context<Self>) {
        if !self.take_escape(window, cx) {
            // Nothing of the root's took the Escape: the focused view's own key listener gets it
            // (a field's `on_escape`, a page's keys). Bindings run before key listeners, so
            // without this no listener under the root would ever see Escape.
            cx.propagate();
        }
    }

    /// The root's Escape, before any page's (`watch_keys`): an open menu first; then the notes
    /// panel (not on Sessions, not over the run footer's second step); then Lookup, unless its
    /// view takes Escape itself or a Library form is open. A dialog takes Escape before all of
    /// these but the menu (`kit::Dialog`); under the close prompt or the copy dialog nothing
    /// else gets it. True when the root took it.
    fn take_escape(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if kit::menu::close_open_popup(window, cx) || kit::dialog::escape_open_dialog(window, cx) {
            cx.notify();
            return true;
        }
        if self.app.copy_dialog.is_some() || self.app.confirm_close {
            return true;
        }
        let app = &mut self.app;
        if app.sessions.notes_open
            && app.page != Page::Sessions
            && !(app.page == Page::Run
                && app
                    .encounters
                    .run()
                    .is_some_and(|run| run.footer_step_open()))
        {
            app.sessions.notes_open = false;
            cx.notify();
            return true;
        }
        let view_takes_escape = if app.lookup.on_trackers() {
            // Escape ends a typed amount, not Lookup (the field loses focus).
            if app.trackers.takes_escape() {
                app.trackers.end_asking(false, 0);
                cx.notify();
                return true;
            }
            false
        } else {
            app.lookup.reader.takes_escape()
        };
        if app.lookup.open
            && !view_takes_escape
            && !(app.page == Page::Library && app.library.form_open())
        {
            app.lookup.close();
            cx.notify();
            return true;
        }
        false
    }

    /// The page's own place: the no-campaign state where a page needs a campaign, else the page
    /// area's render hook (`pages::render_page`).
    fn page(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        #[cfg(test)]
        if self.gallery.is_some() {
            return gallery::render(self, window, cx);
        }
        #[cfg(test)]
        if let Some(preview) = pages::reader::render_preview(self, window, cx) {
            return preview;
        }
        let needs_campaign = matches!(
            self.app.page,
            Page::Sessions
                | Page::Timeline
                | Page::Encounters
                | Page::World
                | Page::Party
                | Page::Campaign
        );
        if needs_campaign && self.app.campaigns.active_campaign().is_none() {
            return self.no_campaign(cx);
        }
        match self.app.page {
            Page::Settings => self.settings_page(window, cx),
            Page::Campaign => self.campaign_page(window, cx),
            _ => pages::render_page(self, window, cx),
        }
    }

    /// A page that needs a campaign, with none open (`Splat::no_campaign`).
    fn no_campaign(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let no_campaigns = self.app.campaigns.all().is_empty();
        let action = if self.app.page == Page::Campaign && no_campaigns {
            // The Campaign page's own add form is the Settings page area's part.
            pages::settings::render_add_campaign_form(self, cx)
        } else {
            kit::Button::primary(
                "no-campaign-action",
                if no_campaigns {
                    "Add campaign"
                } else {
                    "Choose a campaign"
                },
            )
            .on_click(cx.listener(move |root, _, _, cx| {
                if no_campaigns {
                    root.app.campaigns.focus_add();
                }
                root.app.settings_tab = SettingsTab::Account;
                root.show_page(Page::Settings, cx);
            }))
            .into_any_element()
        };
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(kit::page_header(self.app.page.name(), None, Vec::new()))
            .child(kit::empty_state(
                icons::CAMPAIGN,
                if no_campaigns {
                    "No campaign yet"
                } else {
                    "No campaign open"
                },
                "A campaign keeps your notes, party and encounters together. The Library works without one.",
                Some(action),
            ))
            .into_any_element()
    }

    /// The Settings page: its header and tabs are the shell's; Touchpad is the shell's own tab
    /// (the shell owns the touchpad speed); the other tabs are the Settings page area's.
    fn settings_page(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let tab = match self.app.settings_tab {
            SettingsTab::Account => 0,
            SettingsTab::Recording => 1,
            SettingsTab::Touchpad => 2,
        };
        let tabs = kit::Segmented::new(
            "settings-tabs",
            [
                "Account and campaigns",
                "Recording and processing",
                "Touchpad",
            ],
            tab,
        )
        .on_select(cx.listener(|root, index: &usize, _, cx| {
            root.app.settings_tab = match index {
                0 => SettingsTab::Account,
                1 => SettingsTab::Recording,
                _ => SettingsTab::Touchpad,
            };
            root.app.processing.look_again();
            cx.notify();
        }));
        let body = match self.app.settings_tab {
            SettingsTab::Touchpad => self.touchpad_tab(cx),
            SettingsTab::Recording => pages::settings::render_recording(self, window, cx),
            SettingsTab::Account => pages::settings::render_account(self, window, cx),
        };
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(kit::page_header("Settings", None, Vec::new()))
            .child(tabs)
            .child(kit::rule())
            .child(div().flex_1().min_h_0().child(body))
            .into_any_element()
    }

    /// Settings → Touchpad: the scrolling speed of this computer's touchpad.
    fn touchpad_tab(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let speeds = crate::touchpad::TouchpadSpeed::ALL;
        let chosen = speeds
            .iter()
            .position(|(speed, _)| *speed == self.app.touchpad_speed)
            .unwrap_or(0);
        let choice = kit::Segmented::new("touchpad-speed", speeds.map(|(_, words)| words), chosen)
            .on_select(cx.listener(|root, index: &usize, _, cx| {
                if let Some((speed, _)) = crate::touchpad::TouchpadSpeed::ALL.get(*index) {
                    root.app.touchpad_speed = *speed;
                    cx.notify();
                }
            }));
        div()
            .flex()
            .flex_col()
            .child(kit::heading(
                kit::Heading::Group {
                    title: "Touchpad",
                    count: 0,
                },
                false,
            ))
            .child(kit::field(
                "Scrolling speed",
                Some("Mouse wheels keep their speed.".into()),
                None,
                choice,
            ))
            .into_any_element()
    }

    /// The Campaign page: its header (`Campaign entries` opens the Library) and tabs are the
    /// shell's; each tab's body is the Campaigns page area's.
    fn campaign_page(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let context = (self.app.campaign_tab == CampaignTab::Packages)
            .then(|| self.app.campaign_content.context().into());
        let entries = kit::Button::quiet("campaign-entries", "Campaign entries")
            .icon(icons::DOCUMENT)
            .hint("This campaign’s own entries, in the Library")
            .on_click(cx.listener(|root, _, _, cx| {
                root.app.library.show_campaign_entries();
                root.show_page(Page::Library, cx);
            }));
        let tab = match self.app.campaign_tab {
            CampaignTab::Packages => 0,
            CampaignTab::RunSettings => 1,
            CampaignTab::Link => 2,
        };
        let tabs = kit::Segmented::new(
            "campaign-tabs",
            ["Packages", "Rules and runs", "Link and folder"],
            tab,
        )
        .on_select(cx.listener(|root, index: &usize, _, cx| {
            root.app.campaign_tab = match index {
                0 => CampaignTab::Packages,
                1 => CampaignTab::RunSettings,
                _ => CampaignTab::Link,
            };
            cx.notify();
        }));
        let body = match self.app.campaign_tab {
            CampaignTab::Packages => pages::campaigns::render_packages(self, window, cx),
            CampaignTab::RunSettings => pages::campaigns::render_run_settings(self, window, cx),
            CampaignTab::Link => pages::campaigns::render_link(self, window, cx),
        };
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(kit::page_header(
                "Campaign",
                context,
                vec![entries.into_any_element()],
            ))
            .child(tabs)
            .child(kit::gap(theme::XS))
            .child(div().flex_1().min_h_0().child(body))
            .into_any_element()
    }
}

impl Render for SplatRoot {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        #[cfg(test)]
        kit::probe::clear();
        kit::dialog::begin_frame(cx);
        kit::menu::begin_frame(window, cx);
        self.frame_logic(window, cx);
        self.app.begin_draw();
        // After the frame logic, which can open or close a panel or change page.
        self.measure_shell(window, cx);
        let c = colors();
        let rail = rail::render(self, self.connection(), window, cx);
        let panels_inert = self.app.copy_dialog.is_some() || self.app.confirm_close;
        // Bands above the page: the run strip on any other page while a run is open; the party
        // bar on the Session page.
        let strip = (self.app.encounters.has_run() && self.app.page != Page::Run)
            .then(|| pages::run::render_strip(self, window, cx));
        let party_bar = pages::party::render_bar(self, window, cx);
        let banner = pages::sessions::render_banner(self, window, cx);
        // One panel beside the page at a time: Lookup, or the session's notes.
        let side_column = self.side_column(window);
        let side = if self.app.lookup.open {
            Some(pages::lookup::render_panel(self, window, cx))
        } else if self.app.sessions.notes_open && self.app.page != Page::Sessions {
            Some(pages::sessions::render_notes_panel(self, window, cx))
        } else {
            None
        };
        let side_room = window.viewport_size().width - self.rail_width();
        let page = self.page(window, cx);
        let main = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .h_full()
            .bg(c.page)
            .children(strip)
            .children(party_bar)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .min_h_0()
                    .child(div().flex_1().min_w_0().h_full().px(M).py(S).child(page))
                    .children(side.zip(side_column).map(|(side, column)| {
                        column.render(
                            side_room,
                            div()
                                .size_full()
                                .border_l(theme::HAIRLINE_WIDTH)
                                .border_color(c.hairline)
                                .child(side),
                            window,
                            cx,
                        )
                    })),
            )
            // The session banner sits at the bottom, so it never moves `Next turn` under a
            // pointer parked on it (`Sessions::banner`).
            .children(banner);
        let dialogs = [
            self.app
                .copy_dialog
                .is_some()
                .then(|| pages::library::render_copy_dialog(self, window, cx)),
            self.app
                .confirm_close
                .then(|| close::prompt(self, window, cx)),
        ];
        div()
            .id("splat-root")
            .key_context("SplatRoot")
            .track_focus(&self.focus)
            // The first key listener on every focus path, and an empty one: when an interceptor
            // stops a key (the root's Escape, `watch_keys`), GPUI still calls the first key
            // listener of the path once before it looks at the stop (`dispatch_key_down_up_event`).
            // Without this one, that call went to a part's own key listener: a dialog's frame
            // closed the dialog on the Escape that had closed its drop-down.
            // It moves the frame clock to the key's time too (`clock_on_press` for presses).
            .capture_key_down({
                let ctx = self.ctx.clone();
                move |_, _, _| ctx.tick_clock()
            })
            .on_action(cx.listener(Self::toggle_lookup))
            .on_action(cx.listener(Self::toggle_notes))
            .on_action(cx.listener(Self::toggle_trackers))
            .on_action(cx.listener(Self::toggle_session_list))
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::save))
            .on_action(cx.listener(pages::lookup::back))
            .on_action(cx.listener(pages::lookup::forward))
            .relative()
            .flex()
            .flex_row()
            .size_full()
            .bg(c.page)
            .text_color(c.text)
            .font_family(theme::family::UI)
            .text_size(theme::Type::Body.size())
            // First, so a touchpad's scroll is at the chosen speed before anything sees it.
            .child(touchpad::listener(self.app.touchpad_speed))
            .child(clock_on_press(self.ctx.clone()))
            .child(rail::column(self, window).render(
                window.viewport_size().width,
                rail.when(panels_inert, |rail| rail.opacity(0.6)),
                window,
                cx,
            ))
            .child(main)
            .children(dialogs.into_iter().flatten())
    }
}

/// The frame clock moves to the time of each press, before any handler reads it: the clock moves
/// only as a frame starts, and a press and its release can come in one frame long after the last
/// one (the pointer still on Add for half a minute), so a timed result the press starts (an Add
/// with its Undo) would be born old and end at once.
fn clock_on_press(ctx: StateContext) -> impl IntoElement {
    gpui::canvas(
        |_, _, _| {},
        move |_, _, window, _| {
            let down = ctx.clone();
            window.on_mouse_event(move |_: &gpui::MouseDownEvent, phase, _, _| {
                if phase == gpui::DispatchPhase::Capture {
                    down.tick_clock();
                }
            });
            let up = ctx.clone();
            window.on_mouse_event(move |_: &gpui::MouseUpEvent, phase, _, _| {
                if phase == gpui::DispatchPhase::Capture {
                    up.tick_clock();
                }
            });
        },
    )
    .absolute()
    .size_0()
}
