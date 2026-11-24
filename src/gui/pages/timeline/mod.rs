//! The Timeline page (egui: `src/timeline/page.rs`, `src/timeline/checks.rs`): the whole
//! campaign in story order, apart from any one session. Its header with the count and the clock,
//! the file's problems, the Add strip, the filters, the list of events in a virtual list, and the
//! open event's form beside it (or in its place at a narrow width), with the list's results in
//! its message footer.
//!
//! The state is the egui page's (`root.app.timeline_page`, `TimelinePage`) and the book's
//! (`root.app.timeline`); this view calls the same methods (`row_words`, `list_messages`,
//! `footer_act`, `apply_detail`, `checks::block`, …).
pub(crate) mod checks;
pub(crate) mod detail;
pub(crate) mod list;
#[cfg(test)]
pub(crate) mod tests;

use crate::design::outcome::{Choice, FaultMessage, MessageChoice, Notice, PaneFrame};
use crate::gui::theme::{
    Density, HAIRLINE_WIDTH, L, LIST_MEASURE, M, PANEL_MAX_WIDTH, PANEL_MIN_WIDTH, READER_MEASURE,
    S, Type, TypeExt, XS, colors,
};
use crate::gui::{SplatRoot, icons, kit};
use crate::timeline::page::{ADD_KINDS, FILTERS, ListAct};
use crate::timeline::{Kind, kind_icon};
use crate::{Page, timeline};
use gpui::{
    AnyElement, Context, InteractiveElement, IntoElement, ParentElement, Pixels, ScrollHandle,
    SharedString, Styled, UniformListScrollHandle, Window, div, prelude::*, px,
};

/// Two columns from this width of the page; below it the open event takes the list's place.
const TWO_COLUMNS_FROM: Pixels = px(640.0);

/// This area's GPUI-only state, kept in `root.pages.timeline`.
#[derive(Default)]
pub struct State {
    /// The page was shown in the last poll: coming back reopens it (`TimelinePage::reopen`).
    shown: bool,
    /// The list's virtual scroll.
    pub(crate) list: UniformListScrollHandle,
    /// The open event's form's scroll.
    pub(crate) form: ScrollHandle,
    /// The open event's form opens at its top in the next draw.
    pub(crate) form_top: bool,
    /// The form's `Link text` section is open.
    pub(crate) link_open: bool,
    /// The open event's column and its title line, as the last frame laid them out: their
    /// heights, for whether the title line scrolls with the form (`detail::render`).
    pub(crate) detail_room: std::rc::Rc<std::cell::Cell<(Pixels, Pixels)>>,
    /// The page's keyboard focus, so Ctrl+S reaches it while no field has the keyboard.
    focus: Option<gpui::FocusHandle>,
}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Ctrl+S on this page (the root's one owner calls it, `pages::save_key`): save the Timeline now.
pub(crate) fn save_key(root: &mut SplatRoot, _: &mut Context<SplatRoot>) {
    crate::timeline::checks::settle_length(&mut root.app.timeline, None);
    root.app.timeline.save();
}

/// Once per frame, before anything draws: what the egui app gives the page before it draws
/// (`Splat::update`'s `Page::Timeline` arm).
pub(crate) fn poll(root: &mut SplatRoot, _window: &mut Window, cx: &mut Context<SplatRoot>) {
    let on_page = root.app.page == Page::Timeline;
    let came_back = on_page && !root.pages.timeline.shown;
    root.pages.timeline.shown = on_page;
    if !on_page || root.app.campaigns.active_campaign().is_none() {
        return;
    }
    let modal = root.modal_open(cx);
    let app = &mut root.app;
    if came_back {
        app.timeline_page.reopen();
    }
    app.timeline_page.open_run = app.run_fact.as_ref().map(|fact| fact.id);
    app.timeline.run_open = app.encounters.has_run();
    app.timeline_page.set_saved_runs(
        app.encounters.saved_runs(),
        app.encounters.saved_runs_revision(),
    );
    app.timeline_page.dated = app.sessions.current_dated();
    app.timeline_page.tables = app.lookup.index().map(|index| index.loot_sources());
    // Row 123: what a check result can offer to its session's plan, copied once for each read
    // of the campaign's encounters.
    let revision = app.encounters.plan_revision();
    if app.timeline_page.plan_names.revision != revision {
        let (fights, groups) = app.encounters.plan_names();
        app.timeline_page.plan_names = timeline::page::PlanNames::new(revision, fights, groups);
    }
    app.timeline_page.places = app
        .lookup
        .index()
        .map(|index| index.places())
        .unwrap_or_default();
    app.timeline_page.modal = modal;
}

/// The page's width: the window less the rail, the side panel and the page's sides.
fn page_width(root: &SplatRoot, window: &Window) -> Pixels {
    root.page_width(window)
}

/// The open event's column beside the list (`design::side_panel_width`).
fn detail_width(page: Pixels) -> Pixels {
    (page * 0.36).clamp(PANEL_MIN_WIDTH, PANEL_MAX_WIDTH)
}

/// The page.
pub(crate) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let Some(campaign) = root.app.campaigns.active_campaign().map(|c| c.id) else {
        return div().into_any_element();
    };
    let page_width = page_width(root, window);
    {
        let SplatRoot { app, bridge, .. } = root;
        let page = &mut app.timeline_page;
        // Only a real change: the first show can already have an event to reveal.
        if page.campaign.is_some_and(|old| old != campaign) {
            *page = timeline::page::TimelinePage {
                open_run: page.open_run,
                saved_runs: std::mem::take(&mut page.saved_runs),
                saved_runs_revision: page.saved_runs_revision,
                modal: page.modal,
                ..Default::default()
            };
        }
        page.campaign = Some(campaign);
        page.settle_added(bridge.ctx(), &app.timeline);
        // A pending reveal is its journey's: a changed selection drops it before any draw.
        if page
            .reveal_checks
            .as_ref()
            .is_some_and(|(journey, _)| page.selected.as_deref() != Some(journey.as_str()))
        {
            page.reveal_checks = None;
        }
    }
    let context = root.app.timeline_page.header_context(&root.app.timeline);
    let header = kit::page_header(
        "Timeline",
        (!context.is_empty()).then(|| context.into()),
        Vec::new(),
    );
    let focus = root
        .pages
        .timeline
        .focus
        .get_or_insert_with(|| cx.focus_handle())
        .clone();
    // With the keyboard at the root, the page takes it, so its keys reach it.
    if root.focus.is_focused(window) {
        window.focus(&focus, cx);
    }
    let mut page = div()
        .key_context("Timeline")
        .track_focus(&focus)
        .flex()
        .flex_col()
        .size_full()
        .child(header);
    if let Some(problem) = problems(root, cx) {
        page = page.child(problem);
        if root.app.timeline.conflict && root.app.timeline.unreadable.is_none() {
            return page
                .child(div().flex_1().min_h_0().child(comparison(root)))
                .into_any_element();
        }
    }
    page = page.child(add_strip(root, page_width, window, cx));
    if let Some(words) = root.app.timeline_page.dated.clone() {
        let notice = Notice::warning(timeline::page::as_sentence(&words))
            .choice(Choice::new(MessageChoice::Choice("Play this one"), ()));
        page = page.child(div().pb(S).children(kit::messages::lines(
            "timeline-dated",
            vec![notice.into_plain()],
            cx.listener(|root, pressed: &kit::messages::Pressed<()>, _, cx| {
                if pressed.take().is_some() {
                    root.app.timeline_page.play_current = true;
                    cx.notify();
                }
            }),
        )));
    }
    page = page.child(kit::gap(XS));
    let book = &root.app.timeline;
    if book.events.is_empty() && book.unreadable.is_none() {
        let open_run = root.app.timeline_page.open_run.is_some();
        let label = "Start with day 1";
        let start = if open_run {
            kit::Button::secondary("timeline-start", label)
        } else {
            kit::Button::primary("timeline-start", label)
        }
        .on_click(cx.listener(|root, _, _, cx| {
            let current = root.app.sessions.current();
            let id = root.app.timeline.add(Kind::Day, current);
            let SplatRoot { app, bridge, .. } = root;
            app.timeline_page
                .record_added(id, &app.timeline, bridge.ctx());
            cx.notify();
        }));
        let body = kit::empty_state(
            icons::TIMELINE,
            "Nothing on the timeline yet",
            "Add the first day above. Each encounter run appears here by itself, in the session it belongs to.",
            Some(start),
        );
        let footer = list_footer(root, None, cx);
        return page
            .child(pane("timeline-empty", body.into_any_element(), footer))
            .into_any_element();
    }
    {
        let page_state = &mut root.app.timeline_page;
        if !std::mem::replace(&mut page_state.opened, true) && page_state.scroll_to.is_none() {
            page_state.scroll_to = root.app.timeline.events.len().checked_sub(1);
        }
    }
    let narrow = page_width < TWO_COLUMNS_FROM;
    root.app.timeline_page.narrow = narrow;
    let selected = root.app.timeline_page.selected.clone();
    let open = selected
        .clone()
        .filter(|id| root.app.timeline.events.iter().any(|event| event.id == *id));
    if !(narrow && open.is_some()) {
        page = page.child(filters(root, page_width, cx));
    }
    page = page.child(kit::gap(XS));
    root.app
        .sessions
        .expire_event_link_undo(true, &root.app.timeline);
    if narrow {
        if let Some(id) = open {
            let back = kit::Button::quiet("timeline-back", "All events")
                .icon(icons::HISTORY_BACK)
                .line_start()
                .density(Density::Read)
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.timeline_page.select(None);
                    cx.notify();
                }));
            let form = detail::render(root, &id, page_width, window, cx);
            let footer = list_footer(root, Some(&id), cx);
            return page
                .child(back)
                .child(pane("timeline-detail-pane", form, footer))
                .into_any_element();
        }
        let list = list::render(root, window, cx);
        let footer = list_footer(root, None, cx);
        return page
            .child(pane("timeline-list-pane", list, footer))
            .into_any_element();
    }
    let Some(id) = open else {
        let list = list::render(root, window, cx);
        let footer = list_footer(root, None, cx);
        return page
            .child(div().flex_1().min_h_0().max_w(LIST_MEASURE).child(pane(
                "timeline-list-pane",
                list,
                footer,
            )))
            .into_any_element();
    };
    let width = detail_width(page_width);
    let content = page_width.min(LIST_MEASURE + width);
    let list = list::render(root, window, cx);
    let footer = list_footer(root, Some(&id), cx);
    let form = detail::render(root, &id, width - S, window, cx);
    page.child(
        div()
            .flex()
            .flex_row()
            .flex_1()
            .min_h_0()
            .w(content)
            .child(div().flex_1().min_w_0().h_full().pr(S).child(pane(
                "timeline-list-pane",
                list,
                footer,
            )))
            .child(
                div()
                    .w(width)
                    .flex_none()
                    .h_full()
                    .pl(S)
                    .border_l(HAIRLINE_WIDTH)
                    .border_color(colors().hairline)
                    .child(form),
            ),
    )
    .into_any_element()
}

/// A column with its body and its message footer under it.
fn pane(id: &'static str, body: AnyElement, footer: Vec<AnyElement>) -> impl IntoElement {
    div()
        .id(id)
        .flex()
        .flex_col()
        .flex_1()
        .min_h_0()
        .h_full()
        .child(div().flex_1().min_h_0().child(body))
        .when(!footer.is_empty(), |pane| {
            pane.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(S)
                    .pt(S)
                    .pb(S)
                    .border_t(HAIRLINE_WIDTH)
                    .border_color(colors().hairline)
                    .children(footer),
            )
        })
}

/// The list's footer (`TimelinePage::list_messages`): the link update after a rename and its
/// Undo, the open event's rest ruling, the file's notices, a removal and an add.
fn list_footer(
    root: &mut SplatRoot,
    open: Option<&str>,
    cx: &mut Context<SplatRoot>,
) -> Vec<AnyElement> {
    let app = &mut root.app;
    let offer = open.and_then(|id| app.sessions.event_link_offer(&app.timeline, id));
    let frame = PaneFrame::plain();
    let lines = app
        .timeline_page
        .list_messages(
            &frame,
            0.0,
            &app.timeline,
            &mut app.sessions,
            open,
            offer.as_ref(),
        )
        .into_plain();
    let open = open.map(str::to_owned);
    kit::messages::lines(
        "timeline-footer",
        lines,
        cx.listener(
            move |root, pressed: &kit::messages::Pressed<ListAct>, _, cx| {
                let Some(press) = pressed.take() else {
                    return;
                };
                let app = &mut root.app;
                let offer = open
                    .as_deref()
                    .and_then(|id| app.sessions.event_link_offer(&app.timeline, id));
                app.timeline_page
                    .footer_act(press, &mut app.timeline, &mut app.sessions, offer);
                cx.notify();
            },
        ),
    )
}

/// What stops the timeline from saving (`timeline::page::problems`): a file that cannot be read
/// (`Read the file again`, and for a gone file `Save this timeline there again`), or a file that
/// changed under the draft (`Overwrite with my timeline`, then `Reload and lose my timeline`).
fn problems(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> Option<AnyElement> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Repair {
        ReadAgain,
        SaveAgain,
        Overwrite,
        Reload,
    }
    let book = &root.app.timeline;
    let message = if let Some(error) = &book.unreadable {
        let mut message = FaultMessage::new(&Unread(error.clone())).choice(Choice::new(
            MessageChoice::Choice("Read the file again"),
            Repair::ReadAgain,
        ));
        if book.gone {
            message = message.choice(Choice::new(
                MessageChoice::Choice("Save this timeline there again"),
                Repair::SaveAgain,
            ));
        }
        message
    } else if book.conflict {
        FaultMessage::new(&Changed)
            .choice(Choice::new(
                MessageChoice::Choice("Overwrite with my timeline"),
                Repair::Overwrite,
            ))
            .choice(Choice::new(
                MessageChoice::Lose("Reload and lose my timeline"),
                Repair::Reload,
            ))
    } else {
        return None;
    };
    Some(
        div()
            .pb(L)
            .children(kit::messages::lines(
                "timeline-problem",
                vec![message.into_plain()],
                cx.listener(|root, pressed: &kit::messages::Pressed<Repair>, _, cx| {
                    let Some(press) = pressed.take() else {
                        return;
                    };
                    let book = &mut root.app.timeline;
                    match press.id {
                        Repair::ReadAgain => book.reload(),
                        Repair::SaveAgain => book.save_again(),
                        Repair::Overwrite => book.overwrite(),
                        Repair::Reload => book.reload_keeping_undo(),
                    }
                    cx.notify();
                }),
            ))
            .into_any_element(),
    )
}

/// `Timeline.md` cannot be read: the book's words.
struct Unread(String);

impl crate::design::outcome::Fault for Unread {
    fn severity(&self) -> crate::design::outcome::Severity {
        crate::design::outcome::Severity::CannotRead
    }
    fn cause(&self) -> crate::design::outcome::Sentence {
        timeline::page::as_sentence(&self.0)
    }
}

/// `Timeline.md` changed on disk under the DM's unsaved edit.
struct Changed;

impl crate::design::outcome::Fault for Changed {
    fn severity(&self) -> crate::design::outcome::Severity {
        crate::design::outcome::Severity::NeedsFixing
    }
    fn cause(&self) -> crate::design::outcome::Sentence {
        timeline::page::as_sentence(crate::timeline::CHANGED_UNDER_DRAFT)
    }
}

/// The draft and the file side by side while the file changed under the draft.
fn comparison(root: &SplatRoot) -> AnyElement {
    let c = colors();
    match root.app.timeline.conflict_comparison() {
        Ok((draft, disk)) => {
            let side = |id: &'static str, title: &'static str, words: String| {
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .gap(XS)
                    .child(kit::label(title, 0))
                    .child(
                        div().flex_1().min_h_0().child(kit::scroll_column(
                            id,
                            div()
                                .ty(Type::Mono)
                                .text_color(c.text)
                                .whitespace_normal()
                                .child(words),
                        )),
                    )
            };
            div()
                .flex()
                .flex_row()
                .gap(L)
                .size_full()
                .child(side("timeline-draft", "Your version", draft))
                .child(side("timeline-disk", "Timeline.md now", disk))
                .into_any_element()
        }
        Err(error) => kit::Message::new(kit::Tone::Error, error).into_any_element(),
    }
}

/// Add an event of `kind` to the session new events go to, and open it.
fn add_kind(root: &mut SplatRoot, kind: Kind) {
    let current = root.app.sessions.current();
    let book = &mut root.app.timeline;
    let id = if kind == Kind::LongRest {
        book.add_long_rest(current)
    } else {
        book.add(kind, current)
    };
    let SplatRoot { app, bridge, .. } = root;
    app.timeline_page
        .record_added(id, &app.timeline, bridge.ctx());
}

/// Whether every kind of the Add strip fits on its line in `room` (`hanging_add_strip`'s fit):
/// the buttons' natural widths and the gaps between them.
fn strip_fits(window: &Window, room: Pixels) -> bool {
    use crate::gui::pages::trackers::measure;
    let buttons: Pixels = ADD_KINDS
        .iter()
        .map(|kind| measure::button_width(window, kind.word(), true))
        .fold(px(0.0), |a, b| a + b);
    buttons + XS * (ADD_KINDS.len() - 1) as f32 <= room
}

/// `Add` and one quiet button for each kind; blocked while the file must be repaired. Fight and
/// Long rest stay on the line; when the line has no room for every kind, the rarer kinds share
/// `More Add actions` (egui `quick_add_with`, `hanging_add_strip`).
fn add_strip(
    root: &SplatRoot,
    page_width: Pixels,
    window: &Window,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    let blocked = root
        .app
        .timeline
        .unreadable
        .is_some()
        .then_some("The timeline file must be repaired first.");
    let label = div().ty(Type::Label).text_color(colors().text).child("Add");
    let room =
        page_width - crate::gui::pages::trackers::measure::width(window, Type::Label, "ADD") - XS;
    let fits = strip_fits(window, room);
    let shown = |kind: &Kind| fits || matches!(kind, Kind::Encounter | Kind::LongRest);
    let mut strip = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(XS)
        .pb(S)
        .child(label)
        .children(ADD_KINDS.iter().filter(|kind| shown(kind)).map(|&kind| {
            kit::Button::quiet(
                SharedString::from(format!("timeline-add-{}", kind.word())),
                kind.word(),
            )
            .icon(kind_icon(kind))
            .name(format!("Add {}", kind.word()))
            .density(Density::Read)
            .blocked(blocked)
            .on_click(cx.listener(move |root, _, _, cx| {
                add_kind(root, kind);
                cx.notify();
            }))
        }));
    if !fits {
        let rest: Vec<Kind> = ADD_KINDS
            .iter()
            .copied()
            .filter(|kind| !shown(kind))
            .collect();
        let hint = rest
            .iter()
            .map(|kind| kind.word())
            .collect::<Vec<_>>()
            .join(", ");
        let mut more = kit::MenuButton::new(
            "timeline-add-more",
            kit::Button::quiet("timeline-add-more-button", "")
                .icon(icons::EXPAND)
                .name("More Add actions")
                .hint(hint)
                .density(Density::Read),
        );
        for kind in rest {
            more = more.item(
                kit::MenuItem::new(kind.word())
                    .icon(kind_icon(kind))
                    .blocked(blocked)
                    .on_click(cx.listener(move |root, _, _, cx| {
                        add_kind(root, kind);
                        cx.notify();
                    })),
            );
        }
        strip = strip.child(more);
    }
    strip
}

/// The filters: the kinds, the search, the session shown, and where new events go. At a width
/// under `LIST_MEASURE` the kinds take a row of their own.
fn filters(root: &SplatRoot, page_width: Pixels, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let page = &root.app.timeline_page;
    let sessions = root.app.sessions.links();
    let current = root.app.sessions.current();
    let chosen = FILTERS
        .iter()
        .position(|(filter, _)| *filter == page.filter)
        .unwrap_or(0);
    let tabs = kit::Segmented::new("timeline-filter", FILTERS.map(|(_, words)| words), chosen)
        .on_select(cx.listener(|root, index: &usize, _, cx| {
            if let Some((filter, _)) = FILTERS.get(*index) {
                root.app.timeline_page.filter = *filter;
                cx.notify();
            }
        }));
    let search = kit::TextField::new("timeline-search", page.search.clone())
        .placeholder("Search")
        .name("Search the timeline")
        .width(px(160.0))
        .on_change(cx.listener(|root, words: &str, _, cx| {
            // A filter or a search changes the list, not the open event.
            root.app.timeline_page.search = words.to_owned();
            cx.notify();
        }));
    let mut shown: Vec<String> = vec!["All sessions".to_owned()];
    shown.extend(sessions.iter().map(|session| session.label.clone()));
    let ids: Vec<uuid::Uuid> = sessions.iter().map(|session| session.id).collect();
    let session_at = page
        .session
        .and_then(|id| ids.iter().position(|known| *known == id))
        .map_or(0, |at| at + 1);
    let session_ids = ids.clone();
    let session = kit::Dropdown::new(
        "timeline-session-filter",
        "Show one session",
        shown,
        Some(session_at),
    )
    .width(px(150.0))
    .on_select(cx.listener(move |root, index: &usize, _, cx| {
        root.app.timeline_page.session = index
            .checked_sub(1)
            .and_then(|at| session_ids.get(at).copied());
        cx.notify();
    }));
    let mut new_shown: Vec<String> = vec!["No session".to_owned()];
    new_shown.extend(
        sessions
            .iter()
            .map(|session| timeline::page::short_session(&session.label).to_owned()),
    );
    let new_at = current
        .and_then(|id| ids.iter().position(|known| *known == id))
        .map_or(0, |at| at + 1);
    let new_ids = ids;
    let adds_to = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .child(kit::text(Type::Small, c.muted, "Adds to"))
        .child(
            kit::Dropdown::new(
                "timeline-new-events-session",
                "Session for new events",
                new_shown,
                Some(new_at),
            )
            .width(READER_MEASURE / 4.0)
            .on_select(cx.listener(move |root, index: &usize, _, cx| {
                let chosen = index.checked_sub(1).and_then(|at| new_ids.get(at).copied());
                if chosen != root.app.sessions.current() {
                    root.app.timeline_page.new_events_to = Some(chosen);
                }
                cx.notify();
            })),
        );
    let wide = page_width >= LIST_MEASURE;
    let row = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(S)
        .when(wide, |row| row.child(tabs_box(tabs.into_any_element())))
        .child(search)
        .child(session)
        .child(adds_to);
    if wide {
        row.into_any_element()
    } else {
        let tabs = kit::Segmented::new("timeline-filter", FILTERS.map(|(_, words)| words), chosen)
            .on_select(cx.listener(|root, index: &usize, _, cx| {
                if let Some((filter, _)) = FILTERS.get(*index) {
                    root.app.timeline_page.filter = *filter;
                    cx.notify();
                }
            }));
        div()
            .flex()
            .flex_col()
            .gap(S)
            .child(tabs)
            .child(row)
            .into_any_element()
    }
}

fn tabs_box(tabs: AnyElement) -> impl IntoElement {
    div().flex_none().mr(M).child(tabs)
}
