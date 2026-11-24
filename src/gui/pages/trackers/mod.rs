//! Trackers, a view of Lookup (egui: `src/trackers/view.rs`, `src/trackers/history.rs`). Lookup's
//! panel draws it while `root.app.lookup.on_trackers()`: the Due rows, the pinned trackers in the
//! note's order, then `More trackers`, in one scroll area, with the results in the panel's
//! message footer. Every press is one act, which the shell applies after the frame's draw
//! (`Splat::apply_tracker_act`), as egui does.
//!
//! The words come from the trackers crate (`view::due_parts`, `view::line_words`,
//! `view::messages`); this view lays them out and keeps what it worked out by the trackers'
//! revision, so a quiet frame measures nothing.
pub(crate) mod history;
pub(crate) mod measure;
#[cfg(test)]
pub(crate) mod tests;

use crate::design::memo::Revision;
use crate::design::outcome::{PaneFrame, fault_words, plain::fault_tone};
use crate::gui::theme::{
    Density, HAIRLINE_WIDTH, L, M, PANEL_MIN_WIDTH, S, SCROLL_BAR_WIDTH, Type, TypeExt, XS, colors,
};
use crate::gui::{SplatRoot, icons, kit};
use crate::trackers::view::{Act, DueParts, LineWords, RULES_ROUTE, TrackerPress};
use crate::trackers::{Notice, TrackerFault, Trackers, mark_of};
use gpui::{
    AnyElement, Context, ElementId, InteractiveElement, IntoElement, ParentElement, Pixels,
    ScrollHandle, SharedString, StatefulInteractiveElement, Styled, Window, div, prelude::*, px,
};
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// This area's GPUI-only state, kept in `root.pages.trackers`.
#[derive(Default)]
pub struct State {
    /// What the view worked out for the trackers' revision: the Due rows' words, each tracker's
    /// lines and the panel's columns.
    laid: Option<Laid>,
    /// The widest value slot since the definitions were read: an Undo that takes `1000` back to
    /// `999` keeps the room. It ends when the trackers' keys change.
    widest_value: Pixels,
    widest_keys: Vec<String>,
    /// `More trackers` is open.
    more_open: bool,
    /// The view's scroll, which opens at its top when Trackers comes into view.
    scroll: ScrollHandle,
    /// The quick action pressed last, held under a still pointer while a Due row comes or goes
    /// above it (egui `ContentScroll::holds_presses`).
    hold: Rc<Hold>,
    /// The open sections of tracker histories in a reader, by note and member key.
    #[allow(
        dead_code,
        reason = "World’s reader draws the histories (`history::render`)"
    )]
    pub(crate) history_open: HashSet<(uuid::Uuid, String)>,
}

/// What the hold keeps between frames: each quick action's top in the last frame, the pressed
/// action and its top at the press (with the frames it still holds), and the blank room after
/// the content that lets a short view scroll to keep the action where it was.
#[derive(Default)]
struct Hold {
    tops: RefCell<HashMap<String, Pixels>>,
    held: RefCell<Option<Held>>,
    room: Cell<Pixels>,
}

struct Held {
    action: String,
    top: Pixels,
    /// Frames left in which a change is still the press's own (egui `CHANGE_FRAMES`).
    frames: u8,
}

/// How many frames after its press a change may come and still be the press's own: the shell
/// applies an act after the view drew.
const CHANGE_FRAMES: u8 = 4;

impl Hold {
    /// The DM pressed `action`: hold it where the last frame drew it.
    fn press(&self, action: &str) {
        let top = self.tops.borrow().get(action).copied();
        *self.held.borrow_mut() = top.map(|top| Held {
            action: action.to_owned(),
            top,
            frames: CHANGE_FRAMES,
        });
    }

    /// A frame begins: an old hold ends.
    fn frame(&self) {
        let mut held = self.held.borrow_mut();
        if let Some(hold) = held.as_mut() {
            hold.frames = hold.frames.saturating_sub(1);
            if hold.frames == 0 {
                *held = None;
            }
        }
    }

    /// `action` was laid out at `top`: when it is the held one and it moved, the view scrolls
    /// by as much, adding blank room after the content where it is too short to scroll.
    fn seen(&self, action: &str, top: Pixels, scroll: &ScrollHandle, window: &mut Window) {
        self.tops.borrow_mut().insert(action.to_owned(), top);
        let held = self.held.borrow();
        let Some(hold) = held.as_ref().filter(|hold| hold.action == action) else {
            return;
        };
        let moved = top - hold.top;
        if moved.abs() < px(0.5) {
            return;
        }
        let offset = scroll.offset();
        let wanted = (offset.y - moved).min(px(0.0));
        let short = -wanted - scroll.max_offset().y;
        if short > px(0.0) {
            self.room.set(self.room.get() + short);
        }
        scroll.set_offset(gpui::point(offset.x, wanted));
        window.refresh();
    }
}

/// One Due row as laid: its notice as drawn, its words and its rolls' table keys.
struct DueRow {
    notice: Notice,
    parts: DueParts,
    tables: Vec<String>,
}

/// One tracker as laid: its key, its name over its lines (a tracker with `per`), its lines with
/// their names as the column shows them, and whether its actions stand beside its values.
struct TrackerView {
    key: String,
    title: Option<String>,
    pin: bool,
    lines: Vec<(LineWords, String)>,
    beside: bool,
    /// Each quick action's label, by its place in `quick`.
    labels: Vec<String>,
}

struct Laid {
    revision: Revision,
    room: Pixels,
    due: Vec<DueRow>,
    trackers: Vec<TrackerView>,
    name: Pixels,
    value: Pixels,
}

/// The most of a line that its name column takes, so the value and an action keep room.
const NAME_SHARE: f32 = 0.4;

/// The room inside the panel: its width less the view's sides and the scroll bar's lane.
fn panel_room(panel: Option<Pixels>) -> Pixels {
    panel.unwrap_or(PANEL_MIN_WIDTH) - M * 2.0 - SCROLL_BAR_WIDTH
}

/// Lay the view out for the trackers' revision and the panel's room: once per change, never on
/// a quiet frame.
fn lay(state: &mut State, trackers: &Trackers, room: Pixels, window: &Window) {
    let revision = trackers.revision();
    if state
        .laid
        .as_ref()
        .is_some_and(|laid| laid.revision == revision && laid.room == room)
    {
        return;
    }
    let (rules, values) = (&trackers.rules, &trackers.state);
    let due = values
        .due
        .iter()
        .filter_map(|notice| {
            let mark = mark_of(rules, notice)?;
            Some(DueRow {
                notice: notice.clone(),
                parts: crate::trackers::view::due_parts(rules, notice, mark),
                tables: mark.tables.iter().map(|t| t.key().to_owned()).collect(),
            })
        })
        .collect();
    let keys: Vec<String> = rules.trackers.iter().map(|t| t.key.clone()).collect();
    if keys != state.widest_keys {
        state.widest_keys = keys;
        state.widest_value = px(0.0);
    }
    let (mut name, mut value) = (px(0.0), state.widest_value);
    let mut pairs = Vec::new();
    let mut words = Vec::new();
    for tracker in &rules.trackers {
        let Ok(def) = &tracker.def else { continue };
        let indent = if def.per.is_empty() { px(0.0) } else { L };
        let lead: Pixels = def
            .quick
            .iter()
            .take(2)
            .enumerate()
            .map(|(at, quick)| {
                let gap = if at == 0 { px(0.0) } else { S };
                gap + measure::button_width(window, &quick.label, false)
            })
            .fold(px(0.0), |a, b| a + b);
        pairs.push(lead);
        for instance in def.instances() {
            let line = instance.unwrap_or(&def.name);
            name = name.max(indent + measure::width(window, Type::Body, line));
            let slot = crate::trackers::view::value_slot(
                values.value(rules, &tracker.key, instance),
                def.goal,
            );
            value = value.max(measure::width(window, Type::Numeral, &slot));
        }
        words.push((tracker, def, indent));
    }
    let name = name.min(room * NAME_SHARE);
    state.widest_value = value;
    // The line's parts and the gaps between them: name, S, value, S, then the actions.
    let used = name + value + S * 3.0;
    let actions_room = room - used;
    let trackers_laid = words
        .into_iter()
        .zip(pairs)
        .map(|((tracker, def, indent), lead)| TrackerView {
            key: tracker.key.clone(),
            title: (!def.per.is_empty()).then(|| def.name.clone()),
            pin: def.pin,
            lines: crate::trackers::view::line_words(rules, values, &tracker.key, def)
                .into_iter()
                .map(|line| {
                    let fitted = measure::fit_words(window, Type::Body, &line.name, name - indent);
                    (line, fitted)
                })
                .collect(),
            beside: lead <= actions_room,
            labels: def.quick.iter().map(|quick| quick.label.clone()).collect(),
        })
        .collect();
    state.laid = Some(Laid {
        revision,
        room,
        due,
        trackers: trackers_laid,
        name,
        value,
    });
}

/// Once, when the root is built.
pub(crate) fn init(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Once per frame, before anything draws.
pub(crate) fn poll(_root: &mut SplatRoot, _window: &mut Window, _cx: &mut Context<SplatRoot>) {}

/// Trackers in Lookup's panel: its title line (Back while Lookup holds something, Close), the
/// search field that leaves Trackers for a search, and the view.
pub(crate) fn render_view(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let back = root.app.lookup.trackers_hold_something().then(|| {
        kit::IconButton::new("trackers-back", icons::HISTORY_BACK, "Back").on_click(cx.listener(
            |root, _, _, cx| {
                root.app.lookup.back_from_trackers();
                cx.notify();
            },
        ))
    });
    let title = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .min_h(Density::Read.control())
        .child(kit::text(Type::Heading, colors().text, "Trackers"))
        .children(back)
        .child(div().flex_1())
        .child(
            kit::IconButton::new("trackers-close", icons::CLOSE, "Close Lookup").on_click(
                cx.listener(|root, _, _, cx| {
                    root.app.lookup.close();
                    cx.notify();
                }),
            ),
        );
    // Empty, with its placeholder, so it does not read as a filter of the trackers.
    let search = kit::TextField::new("trackers-search", "")
        .placeholder("Search entries and notes…")
        .name(crate::lookup::SEARCH_NAME)
        .on_change(cx.listener(|root, words: &str, _, cx| {
            root.app.lookup.search_from_trackers(words);
            cx.notify();
        }));
    div()
        .flex()
        .flex_col()
        .size_full()
        .pt(M)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(S)
                .px(M)
                .pb(S)
                .child(title)
                .child(search),
        )
        .child(kit::rule())
        .child(
            div()
                .flex_1()
                .min_h_0()
                .child(render_body(root, window, cx)),
        )
        .into_any_element()
}

/// The view itself, without Lookup's title line and search: the empty state, a file that cannot
/// be read, or the trackers, then the message footer.
pub(crate) fn render_body(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let footer = footer(root, cx);
    let trackers = &root.app.trackers;
    let body = if !trackers.in_play() {
        div()
            .pt(S)
            .child(kit::empty_state(
                icons::BELL,
                "No house rules in play",
                "Turn one on to see its trackers here.",
                Some(
                    kit::Button::secondary("trackers-rules", RULES_ROUTE).on_click(cx.listener(
                        |root, _, _, cx| {
                            root.app.trackers.wants_rules = true;
                            cx.notify();
                        },
                    )),
                ),
            ))
            .into_any_element()
    } else {
        if root.app.lookup.take_trackers_from_top() {
            root.pages
                .trackers
                .scroll
                .set_offset(gpui::point(px(0.0), px(0.0)));
            root.pages.trackers.hold.room.set(px(0.0));
        }
        root.pages.trackers.hold.frame();
        body(root, window, cx)
    };
    let scroll = root.pages.trackers.scroll.clone();
    let room = root.pages.trackers.hold.room.get();
    div()
        .flex()
        .flex_col()
        .size_full()
        .child(
            div()
                .flex_1()
                .min_h_0()
                .child(kit::scroll::scroll_column_with(
                    "trackers-scroll",
                    &scroll,
                    div().px(M).pb(S).child(body).child(kit::gap(room)),
                )),
        )
        .when(!footer.is_empty(), |pane| {
            pane.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(S)
                    .px(M)
                    .py(S)
                    .border_t(HAIRLINE_WIDTH)
                    .border_color(colors().hairline)
                    .children(footer),
            )
        })
        .into_any_element()
}

/// The footer's messages (`view::messages`): why the last act recorded nothing, then the last
/// act with `Undo: <label>` and `Add to Timeline`, or `Added to the Timeline.` with `Undo add`.
fn footer(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> Vec<AnyElement> {
    let frame = PaneFrame::plain();
    let lines = crate::trackers::view::messages(&frame, &root.app.trackers, 0.0).into_plain();
    kit::messages::lines(
        "trackers-footer",
        lines,
        cx.listener(
            |root, pressed: &kit::messages::Pressed<TrackerPress>, _, cx| {
                if let Some(press) = pressed.take() {
                    let mut act = None;
                    crate::trackers::view::footer_press(
                        &root.app.trackers,
                        Some(press.id),
                        &mut act,
                    );
                    if let Some(act) = act {
                        root.app.trackers.press_in_view(act);
                    }
                    cx.notify();
                }
            },
        ),
    )
}

/// A fault as a message in its place (a page's placeholder, a banner, a row's fault).
fn fault_message(fault: &TrackerFault, small: bool) -> kit::Message {
    let message = kit::Message::new(
        kit::messages::tone(fault_tone(fault)),
        fault_words(fault).to_string(),
    );
    if small { message.small() } else { message }
}

fn body(root: &mut SplatRoot, window: &mut Window, cx: &mut Context<SplatRoot>) -> AnyElement {
    let trackers = &root.app.trackers;
    if let Some(error) = &trackers.unreadable {
        let fault = TrackerFault::LedgerUnreadable(error.clone());
        let mut message = fault_message(&fault, false).action(
            kit::Button::quiet("trackers-read-again", "Try again")
                .density(Density::Read)
                .on_click(cx.listener(|root, _, _, cx| {
                    root.app.trackers.read_again();
                    cx.notify();
                })),
        );
        if let Some(path) = trackers.ledger_path()
            && let Some(url) = crate::design::outcome::file_url(&path)
        {
            message = message.action(
                kit::Link::new("trackers-show-file", "Show file")
                    .hint(path.display().to_string())
                    .on_click(move |_, _, cx| cx.open_url(&url)),
            );
        }
        return div().pt(S).child(message).into_any_element();
    }
    // A definition that cannot be read shows no value, never an old one.
    if let Some(error) = &trackers.rules.unreadable {
        let fault = TrackerFault::NotesUnreadable(error.clone());
        return div()
            .pt(S)
            .child(fault_message(&fault, false))
            .into_any_element();
    }
    let room = panel_room(root.side_width());
    lay(&mut root.pages.trackers, &root.app.trackers, room, window);
    let focus = root.app.trackers.take_focus_asking();
    let trackers = &root.app.trackers;
    let state = &root.pages.trackers;
    let Some(laid) = &state.laid else {
        return div().into_any_element();
    };
    let mut column = div().flex().flex_col().w_full();
    // A day count that is not known shows no day notice, never an old one, and says why.
    let banner = trackers.state.clock_not_read.clone();
    if let Some(why) = &banner {
        column = column.child(div().pt(S).child(fault_message(
            &TrackerFault::ClockNotRead(why.clone()),
            false,
        )));
    }
    if !laid.due.is_empty() {
        column = column.child(kit::heading(
            kit::Heading::Group {
                title: "Due",
                count: laid.due.len(),
            },
            banner.is_none(),
        ));
        for (at, row) in laid.due.iter().enumerate() {
            column = column.child(due_row(trackers, row, at, cx));
        }
    }
    let asking = trackers.asking().map(|(key, instance, label, typed)| {
        (
            key.to_owned(),
            instance.map(str::to_owned),
            label.to_owned(),
            typed.to_owned(),
        )
    });
    let pinned: Vec<&TrackerView> = laid.trackers.iter().filter(|t| t.pin).collect();
    if !pinned.is_empty() {
        column = column.child(kit::heading(
            kit::Heading::Group {
                title: "Pinned",
                count: 0,
            },
            laid.due.is_empty() && banner.is_none(),
        ));
        for tracker in pinned {
            column = column.child(tracker_lines(
                laid,
                tracker,
                &asking,
                focus,
                (&state.hold, &state.scroll),
                window,
                cx,
            ));
        }
    }
    let more: Vec<&TrackerView> = laid.trackers.iter().filter(|t| !t.pin).collect();
    if !more.is_empty() {
        let open = state.more_open;
        column = column.child(section_heading(
            "trackers-more",
            "More trackers",
            more.len(),
            open,
            cx.listener(|root, _, _, cx| {
                root.pages.trackers.more_open = !root.pages.trackers.more_open;
                cx.notify();
            }),
        ));
        if open {
            for tracker in more {
                column = column.child(tracker_lines(
                    laid,
                    tracker,
                    &asking,
                    focus,
                    (&state.hold, &state.scroll),
                    window,
                    cx,
                ));
            }
        }
    }
    // A member that cannot be used is reported, and the others still work. So are a row of the
    // ledger that counts for nothing and a value below its `min`.
    for (_, problem) in trackers.problems() {
        column = column.child(
            div()
                .pt(XS)
                .child(fault_message(&TrackerFault::Problem(problem.clone()), true)),
        );
    }
    column.into_any_element()
}

/// A folded section's heading (`design::section_with_id`): its caret, its title in capitals and
/// its count; a press opens or closes it.
pub(crate) fn section_heading(
    id: impl Into<ElementId>,
    title: &str,
    count: usize,
    open: bool,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    let c = colors();
    let id: ElementId = id.into();
    let heading = div()
        .id(id.clone())
        .flex()
        .flex_row()
        .items_center()
        .gap(XS)
        .mt(L)
        .min_h(Density::Read.control())
        .rounded(crate::gui::theme::RADIUS_CONTROL)
        .cursor_pointer()
        .hover(|s| s.bg(c.hover))
        .aria_label(SharedString::from(title.to_owned()))
        .aria_expanded(open)
        .child(kit::icon(
            if open { icons::DOWN } else { icons::NEXT },
            Type::Small,
            c.muted,
        ))
        .child(kit::label(title.to_owned(), count))
        .on_click(on_click);
    let heading = kit::target(heading, title.to_owned());
    kit::target(heading, kit::button::element_name(&id))
}

/// One Due row: its words, its rolls with why each is blocked, the rolls' results, a check for
/// each proposed change, then `Done` and the row's menu (`Dismiss`, `Dismiss all`) at its end.
fn due_row(
    trackers: &Trackers,
    row: &DueRow,
    at: usize,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    let c = colors();
    let parts = &row.parts;
    let id = |part: &str| ElementId::Name(format!("trackers-due-{at}-{part}").into());
    let finish = {
        let (done_notice, ticks) = (row.notice.clone(), parts.ticks.clone());
        let done = kit::Button::quiet(id("done"), parts.finish.clone())
            .name(parts.done_name.clone())
            .density(Density::Read)
            .on_click(cx.listener(move |root, _, _, cx| {
                let ticked = ticks
                    .iter()
                    .map(|tick| root.app.trackers.ticked(tick))
                    .collect();
                root.app
                    .trackers
                    .press_in_view(Act::Done(done_notice.clone(), ticked));
                cx.notify();
            }));
        let dismiss_notice = row.notice.clone();
        let mut menu = kit::MenuButton::new(
            id("menu"),
            kit::Button::quiet(id("more"), "")
                .icon(icons::EXPAND)
                .name(format!("More actions for {}", parts.short))
                .density(Density::Read),
        )
        .item(kit::MenuItem::new("Dismiss").on_click(cx.listener(
            move |root, _, _, cx| {
                root.app
                    .trackers
                    .press_in_view(Act::Dismiss(dismiss_notice.clone()));
                cx.notify();
            },
        )));
        if row.notice.days.len() > 1 {
            let all = row.notice.clone();
            menu = menu.item(kit::MenuItem::new("Dismiss all").on_click(cx.listener(
                move |root, _, _, cx| {
                    root.app
                        .trackers
                        .press_in_view(Act::DismissAll(all.clone()));
                    cx.notify();
                },
            )));
        }
        div()
            .flex()
            .flex_row()
            .flex_none()
            .items_center()
            .gap(XS)
            .child(done)
            .child(menu)
    };
    let words = kit::text(Type::Body, c.text, parts.words.clone());
    let plain = parts.rolls.is_empty() && parts.checks.is_empty() && parts.results.is_none();
    let mut block = div().flex().flex_col().gap(XS).pb(XS);
    if plain {
        return block.child(
            div()
                .flex()
                .flex_row()
                .items_end()
                .gap(S)
                .child(words.flex_1().min_w_0())
                .child(finish),
        );
    }
    block = block.child(words);
    for (slot, ((label, reason), table)) in parts.rolls.iter().zip(&row.tables).enumerate() {
        let (notice, table) = (row.notice.clone(), table.clone());
        block = block.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(S)
                .child(
                    kit::Button::secondary(id(&format!("roll-{slot}")), label.clone())
                        .icon(icons::DICE)
                        .density(Density::Read)
                        .blocked(*reason)
                        .on_click(cx.listener(move |root, _, _, cx| {
                            root.app
                                .trackers
                                .press_in_view(Act::Roll(notice.clone(), table.clone()));
                            cx.notify();
                        })),
                )
                .children(reason.map(|reason| kit::text(Type::Small, c.muted, reason))),
        );
    }
    if let Some(results) = &parts.results {
        block = block.child(
            div().pl(L).flex().flex_col().children(
                results
                    .lines()
                    .map(|line| kit::text(Type::Body, c.text, line.to_owned())),
            ),
        );
    }
    for (slot, (words, tick)) in parts.checks.iter().zip(&parts.ticks).enumerate() {
        let on = trackers.ticked(tick);
        let tick = tick.clone();
        block = block.child(
            kit::Checkbox::new(id(&format!("check-{slot}")), words.clone(), on).on_click(
                cx.listener(move |root, _, _, cx| {
                    let on = root.app.trackers.ticked(&tick);
                    root.app.trackers.set_ticked(&tick, !on);
                    cx.notify();
                }),
            ),
        );
    }
    block.child(div().flex().flex_row().justify_end().child(finish))
}

/// A tracker's lines: its name over them (a tracker with `per`), then one line for each, each in
/// the panel's columns: the name, the value, and its actions beside the value when its first two
/// fit there, else on the rows under it from the name's edge. An action that asks for an amount
/// has the amount field in its place while it asks.
fn tracker_lines(
    laid: &Laid,
    tracker: &TrackerView,
    asking: &Option<(String, Option<String>, String, String)>,
    focus: bool,
    held: (&Rc<Hold>, &ScrollHandle),
    window: &Window,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    let c = colors();
    let (hold, scroll) = held;
    let indent = if tracker.title.is_some() { L } else { px(0.0) };
    let mut block = div().flex().flex_col().pb(XS);
    if let Some(title) = &tracker.title {
        block = block.child(
            div()
                .min_h(Density::Read.control())
                .flex()
                .items_center()
                .child(kit::text::one_line(Type::Body, c.text, title.clone())),
        );
    }
    for (line, (words, fitted)) in tracker.lines.iter().enumerate() {
        let key = tracker.key.clone();
        let name = div()
            .id(ElementId::Name(
                format!("trackers-name-{}-{line}", tracker.key).into(),
            ))
            .w(laid.name - indent)
            .flex_none()
            .ty(Type::Body)
            .text_color(c.text)
            .whitespace_nowrap()
            .overflow_hidden()
            .child(fitted.clone())
            .when(*fitted != words.name, |name| {
                name.tooltip(kit::tooltip::tip(words.name.clone().into()))
            });
        let value = div()
            .w(laid.value)
            .flex_none()
            .ty(Type::Numeral)
            .text_color(c.text)
            .whitespace_nowrap()
            .flex()
            .justify_end()
            .child(words.value.clone());
        let actions = words.actions.iter().map(|(at, named)| {
            let label = tracker.labels.get(*at).cloned().unwrap_or_default();
            let here = asking
                .as_ref()
                .is_some_and(|(k, i, l, _)| *k == key && *i == words.instance && *l == label);
            let id = ElementId::Name(
                format!(
                    "trackers-action-{key}-{}-{}-{at}",
                    words.instance.as_deref().unwrap_or(""),
                    words.twin
                )
                .into(),
            );
            let held_name = match &id {
                ElementId::Name(name) => name.to_string(),
                _ => String::new(),
            };
            if here {
                let typed = asking.as_ref().map_or(String::new(), |(.., t)| t.clone());
                let at = *at;
                return kit::TextField::new(id, typed)
                    .placeholder("Amount")
                    .name(label.clone())
                    .width(measure::button_width(window, &label, false).max(px(64.0)))
                    .focus(focus)
                    .on_change(cx.listener(|root, words: &str, _, cx| {
                        root.app.trackers.set_typed(words);
                        cx.notify();
                    }))
                    .on_enter(act(cx, move |root| root.app.trackers.end_asking(true, at)))
                    .on_escape(act(cx, move |root| root.app.trackers.end_asking(false, at)))
                    .on_blur(act(cx, move |root| root.app.trackers.end_asking(false, at)))
                    .into_any_element();
            }
            let (key, instance, at) = (key.clone(), words.instance.clone(), *at);
            let pressed = held_name.clone();
            let button = kit::Button::secondary(id, label)
                .name(named.clone())
                .density(Density::Read)
                .on_click(cx.listener(move |root, _, _, cx| {
                    root.pages.trackers.hold.press(&pressed);
                    let trackers = &mut root.app.trackers;
                    let Some(quick) = trackers.rules.def(&key).and_then(|d| d.quick.get(at)) else {
                        return;
                    };
                    let label = quick.label.clone();
                    // Read on the click only: a dice amount is parsed to tell it from `ask`.
                    if quick.asks() {
                        trackers.ask(&key, instance.as_deref(), &label);
                    } else {
                        trackers.press_in_view(Act::Press {
                            key: key.clone(),
                            instance: instance.clone(),
                            label,
                            at,
                            asked: None,
                        });
                    }
                    cx.notify();
                }));
            // Where the action is in each frame, for the hold.
            let (hold, scroll) = (hold.clone(), scroll.clone());
            div()
                .relative()
                .child(button)
                .child(
                    gpui::canvas(
                        move |bounds, window, _| {
                            hold.seen(&held_name, bounds.top(), &scroll, window)
                        },
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full(),
                )
                .into_any_element()
        });
        let actions: Vec<AnyElement> = actions.collect();
        let has_actions = !actions.is_empty();
        let row = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(S)
            .pl(indent)
            .min_h(Density::Read.control())
            .child(name)
            .child(value);
        if tracker.beside {
            block = block.child(
                row.items_start().child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .flex_1()
                        .min_w_0()
                        .gap(S)
                        .children(actions),
                ),
            );
        } else {
            block = block.child(row).when(has_actions, |block| {
                block.child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .pl(indent)
                        .pb(XS)
                        .gap(S)
                        .children(actions),
                )
            });
        }
        if let Some(fault) = &words.fault {
            block = block.child(div().pl(indent).child(fault_message(fault, true)));
        }
    }
    block
}

/// A handler with no event (a field's Enter, Escape or focus lost) that changes the root, then
/// draws again.
pub(crate) fn act(
    cx: &Context<SplatRoot>,
    change: impl Fn(&mut SplatRoot) + 'static,
) -> impl Fn(&mut Window, &mut gpui::App) + 'static {
    let root = cx.entity().downgrade();
    move |_, cx| {
        let _ = root.update(cx, |root, cx| {
            change(root);
            cx.notify();
        });
    }
}

/// The history of a house rule note in its reader (World's hook): see `history::render`.
#[allow(
    unused_imports,
    reason = "World’s reader calls these (docs/gpui-port.md)"
)]
pub(crate) use history::{footer as history_footer, render as render_history};
