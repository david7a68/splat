//! A journey's encounter checks in the open event's form (egui: `timeline::checks::header`,
//! `lines`): `Encounter checks` over the table menu and its dice words, the result of the last
//! act with its Undo, one line for each check (`Roll`, a typed roll, or its result with its plan
//! offers and `Clear`), `Roll all`, and the results that are no longer on a line. The rows come
//! from `checks::block`.
//!
//! Two places draw the block, as egui's two callers of `header` and `lines` do (`Host`): the
//! Timeline's open event, whose presses go through `TimelinePage::apply_detail`, and an open
//! Travel row of the session plan, whose presses go through the plan's `travel_act`. Each keeps
//! its own typed rolls, fault and last result.
use super::detail::apply;
use crate::gui::theme::{Density, L, S, Type, XS, XXL, colors};
use crate::gui::{SplatRoot, icons, kit};
use crate::timeline::checks::{self as logic, Act, Planned, View};
use crate::timeline::page::DetailActs;
use crate::timeline::{Check, Event};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, Pixels, Styled, Window, div,
    prelude::*, px,
};

/// Where the block is drawn: whose typed rolls, fault and last result it shows, and where its
/// presses go.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Host {
    /// The Timeline's open event (`TimelinePage::apply_detail`).
    Timeline,
    /// An open Travel row of the session plan (`Sessions::travel_act`).
    Plan,
}

fn part(host: Host, id: &str, name: &str) -> ElementId {
    let place = match host {
        Host::Timeline => "timeline",
        Host::Plan => "plan",
    };
    ElementId::Name(format!("{place}-checks-{id}-{name}").into())
}

/// The block's typed rolls and its last fault.
fn state(root: &mut SplatRoot, host: Host) -> &mut logic::State {
    match host {
        Host::Timeline => &mut root.app.timeline_page.checks,
        Host::Plan => root.app.sessions.gv_travel_block(),
    }
}

/// Do a checks act on the journey `id`.
fn act(root: &mut SplatRoot, host: Host, id: &str, act: Act, cx: &mut Context<SplatRoot>) {
    match host {
        Host::Timeline => {
            let acts = DetailActs {
                checks: Some(act),
                ..DetailActs::default()
            };
            apply(root, id, acts, cx);
        }
        Host::Plan => {
            crate::gui::pages::sessions::travel_act(
                root,
                crate::sessions::page::gpui_facade::PlanAct::Travel(id.to_owned(), act),
            );
            cx.notify();
        }
    }
}

/// `Undo` of the journey's last result.
fn undo(root: &mut SplatRoot, host: Host, id: &str, cx: &mut Context<SplatRoot>) {
    match host {
        Host::Timeline => {
            let acts = DetailActs {
                undo_checks: true,
                ..DetailActs::default()
            };
            apply(root, id, acts, cx);
        }
        Host::Plan => {
            crate::gui::pages::sessions::travel_act(
                root,
                crate::sessions::page::gpui_facade::PlanAct::TravelUndo(id.to_owned()),
            );
            cx.notify();
        }
    }
}

/// The block of the journey `id` on the Timeline, at `index` in the book, with `length` as the
/// DM typed it.
pub(crate) fn render(
    root: &mut SplatRoot,
    id: &str,
    index: usize,
    length: &str,
    start: Option<u64>,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let app = &mut root.app;
    let plan = app
        .sessions
        .journey_plan(&app.timeline, id, &app.timeline_page.plan_names);
    let tables = app
        .timeline_page
        .tables
        .clone()
        .map(|sources| sources.tables.clone())
        .unwrap_or_default();
    render_in(
        root,
        Host::Timeline,
        id,
        index,
        (length, start),
        (&tables, &|name: &str| plan.get(name)),
        window,
        cx,
    )
}

/// The block of the journey `id` in `host`: at `index` in the book, with its length as the DM
/// typed it and its start on the clock, the tables it can roll on and what a result's names
/// are to the journey's plan.
#[allow(clippy::too_many_arguments)]
pub(crate) fn render_in(
    root: &mut SplatRoot,
    host: Host,
    id: &str,
    index: usize,
    (length, start): (&str, Option<u64>),
    (tables, planned): (&[crate::roll_table::Table], &dyn Fn(&str) -> Option<bool>),
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    state(root, host).follow(id);
    // Round 29 G142: the check just rolled on the Timeline scrolls least into view with its
    // result and offers, once (`reveal_checks`); another journey's reveal waits for its own.
    let reveal = match host {
        Host::Timeline => {
            let page = &mut root.app.timeline_page;
            page.reveal_checks
                .take_if(|(journey, _)| journey == id)
                .map(|(_, at)| at)
        }
        Host::Plan => None,
    };
    let event: Event = root.app.timeline.events[index].clone();
    let tables = tables.to_vec();
    let view = View {
        event: &event,
        start,
        length,
        tables: &tables,
        planned,
        reveal: None,
    };
    let (word, dice) = logic::table_words(&view);
    let block = logic::block(&view);
    let mut column = div().flex().flex_col().pb(L);
    // The label, then the table menu and the dice words.
    let mut menu = kit::MenuButton::new(
        part(host, id, "table"),
        kit::Button::quiet(part(host, id, "table-button"), word.clone())
            .icon(icons::DOWN)
            .line_start()
            .density(Density::Read),
    );
    let mut names: Vec<_> = tables.iter().collect();
    names.sort_by(|a, b| a.name().cmp(b.name()).then(a.key.cmp(&b.key)));
    for table in names {
        let (key, owner) = (table.key.clone(), id.to_owned());
        menu = menu.item(
            kit::MenuItem::new(table.name().to_owned())
                .checked(block.table.is_some_and(|chosen| chosen.key == table.key))
                .on_click(cx.listener(move |root, _, _, cx| {
                    act(root, host, &owner, Act::Choose(Some(key.clone())), cx);
                })),
        );
    }
    let owner = id.to_owned();
    menu = menu.item(
        kit::MenuItem::new("No checks")
            .blocked((!event.checks.is_empty()).then_some("Clear the results first."))
            .on_click(
                cx.listener(move |root, _, _, cx| act(root, host, &owner, Act::Choose(None), cx)),
            ),
    );
    let mut line = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap(S)
        .child(menu);
    if let Some(dice) = dice {
        line = line.child(kit::text(Type::Small, c.muted, format!("· {dice}")));
    }
    column = column.child(kit::field("Encounter checks", None, None, line).pb(XS));
    if let Some(table) = block.table {
        if let Some(fault) = table.fault() {
            column = column.child(
                div()
                    .id(part(host, id, "cannot-roll"))
                    .child(kit::status(&kit::Status::new(
                        icons::WARN,
                        "Cannot roll",
                        c.document.external,
                    )))
                    .tooltip(kit::tooltip::tip(fault.to_string().into())),
            );
        } else if let Some(warning) = table.per.warning() {
            column = column.child(kit::Message::new(kit::Tone::Warning, warning).small());
        }
    }
    // The last act's result, with its Undo, under the table.
    let done = match host {
        Host::Timeline => root.app.timeline_page.checks_done.get(id),
        Host::Plan => root.app.sessions.gv_travel_done(id),
    };
    if let Some(done) = done {
        let owner = id.to_owned();
        column = column.child(
            div().py(XS).child(
                kit::Message::new(kit::Tone::Success, done.words.to_string()).action(
                    kit::Button::quiet(part(host, id, "undo"), done.undo.label())
                        .icon(icons::UNDO)
                        .density(Density::Read)
                        .on_click(cx.listener(move |root, _, _, cx| {
                            undo(root, host, &owner, cx);
                        })),
                ),
            ),
        );
    }
    // The label column: as wide as the widest name, and `S`.
    let names = block
        .rows
        .iter()
        .map(|(label, _, _)| label.as_str())
        .chain(block.outside.iter().map(|(label, _)| label.as_str()));
    let label_width = names
        .map(|label| crate::gui::pages::trackers::measure::width(window, Type::Small, label))
        .fold(px(0.0), Pixels::max)
        + S;
    let problem = state(root, host)
        .problem()
        .map(|(at, words)| (at, words.to_owned()));
    for (label, check, at) in &block.rows {
        let line = check_line(
            root,
            host,
            id,
            label,
            *check,
            Some(*at),
            &view,
            block.blocked,
            label_width,
            cx,
        )
        .into_any_element();
        let align = (reveal == Some(*at)).then_some(kit::reveal::Align::Least);
        let form = root.pages.timeline.form.clone();
        column = column.child(kit::reveal::revealed(line, &form, align));
        if check.is_none()
            && let Some((Some(place), words)) = &problem
            && place == at
        {
            column = column.child(kit::Message::new(kit::Tone::Warning, words.clone()).small());
        }
    }
    if block.table.is_none()
        && let Some(reason) = block.blocked
    {
        column = column.child(kit::text(Type::Small, c.muted, reason));
    }
    if let Some(count) = block.too_many {
        column = column.child(kit::Message::new(
            kit::Tone::Info,
            format!(
                "This journey has {count} checks. Split it into shorter journeys to roll them here."
            ),
        ));
    }
    if block.waiting.len() > 1 {
        let (owner, waiting) = (id.to_owned(), block.waiting.clone());
        column = column.child(
            div().pt(XS).child(
                kit::Button::secondary(part(host, id, "roll-all"), format!("Roll all {}", waiting.len()))
                    .icon(icons::DICE)
                    .density(Density::Read)
                    .blocked(block.blocked)
                    .on_click(cx.listener(move |root, _, _, cx| {
                        act(root, host, &owner, Act::Roll(waiting.clone()), cx);
                    })),
            ),
        );
    }
    if !block.outside.is_empty() {
        column = column.child(kit::gap(XS)).child(kit::text(
            Type::Small,
            c.muted,
            "Outside this journey now",
        ));
        for (label, check) in &block.outside {
            column = column.child(check_line(
                root,
                host,
                id,
                label,
                Some(check),
                None,
                &view,
                block.blocked,
                label_width,
                cx,
            ));
        }
    }
    if let Some((None, words)) = &problem {
        column = column.child(kit::Message::new(kit::Tone::Warning, words.clone()).small());
    }
    column.into_any_element()
}

/// One line: its name, then its result, or `Roll` and a field for a player's own roll.
#[allow(clippy::too_many_arguments)]
fn check_line(
    root: &mut SplatRoot,
    host: Host,
    id: &str,
    label: &str,
    check: Option<&Check>,
    at: Option<u64>,
    view: &View<'_>,
    blocked: Option<&'static str>,
    label_width: Pixels,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    let c = colors();
    let name = div()
        .w(label_width)
        .flex_none()
        .min_h(Density::Read.control())
        .flex()
        .items_center()
        .child(kit::text(Type::Small, c.muted, label.to_owned()));
    let line = div().flex().flex_row().items_start().gap(S).child(name);
    match check {
        Some(check) => line.child(result(host, id, label, check, view, cx)),
        None => {
            let at = at.unwrap_or_default();
            let owner = id.to_owned();
            let roll = kit::Button::secondary(part(host, id, &format!("roll-{at}")), "Roll")
                .icon(icons::DICE)
                .name(format!("Roll for {label}"))
                .density(Density::Read)
                .blocked(blocked)
                .on_click(cx.listener(move |root, _, _, cx| {
                    act(root, host, &owner, Act::Roll(vec![at]), cx);
                }));
            let typed = state(root, host).typed(at).to_owned();
            let owner = id.to_owned();
            let field = kit::TextField::new(part(host, id, &format!("enter-{at}")), typed)
                .placeholder("Enter a roll")
                .name(format!("Enter a roll for {label}"))
                .width(XXL * 3.0)
                .disabled(blocked.is_some())
                .on_change(cx.listener(move |root, words: &str, _, cx| {
                    state(root, host).set_typed(at, words);
                    cx.notify();
                }))
                .on_enter({
                    let root = cx.entity().downgrade();
                    move |_, cx| {
                        let _ = root.update(cx, |root, cx| {
                            if let Some(found) = state(root, host).enter(at) {
                                act(root, host, &owner, found, cx);
                            }
                            cx.notify();
                        });
                    }
                });
            line.items_center().child(roll).child(field)
        }
    }
}

/// A result: its words, an offer for each encounter or group it links (or `In the plan`), and
/// `Clear` at the line's end.
fn result(
    host: Host,
    id: &str,
    label: &str,
    check: &Check,
    view: &View<'_>,
    cx: &mut Context<SplatRoot>,
) -> impl IntoElement {
    let c = colors();
    let mut words = div()
        .flex()
        .flex_col()
        .flex_1()
        .min_w_0()
        .pt(px(4.0))
        .child(kit::text(
            Type::Body,
            c.text,
            crate::sessions::plain_links(&check.words),
        ));
    for (name, in_plan) in logic::offers(check, view) {
        if in_plan {
            words = words.child(
                div()
                    .min_h(Density::Read.control())
                    .flex()
                    .items_center()
                    .child(kit::status(&kit::Status::new(
                        icons::OK,
                        "In the plan",
                        c.document.saved,
                    ))),
            );
        } else {
            let owner = id.to_owned();
            let planned = Planned {
                name: name.clone(),
                check: (check.label.clone(), check.words.clone()),
            };
            words = words.child(
                kit::Button::quiet(
                    part(host, id, &format!("plan-{name}-{}", check.label)),
                    format!("Plan “{name}”"),
                )
                .line_start()
                .density(Density::Read)
                .on_click(cx.listener(move |root, _, _, cx| {
                    act(root, host, &owner, Act::Plan(planned.clone()), cx);
                })),
            );
        }
    }
    let owner = id.to_owned();
    let (check_label, check_words) = (check.label.clone(), check.words.clone());
    div()
        .flex()
        .flex_row()
        .items_start()
        .gap(S)
        .flex_1()
        .min_w_0()
        .child(words)
        .child(
            kit::Button::quiet(part(host, id, &format!("clear-{}", check.label)), "Clear")
                .name(format!("Clear the check for {label}"))
                .density(Density::Read)
                .on_click(cx.listener(move |root, _, _, cx| {
                    act(
                        root,
                        host,
                        &owner,
                        Act::Clear(check_label.clone(), check_words.clone()),
                        cx,
                    );
                })),
        )
}
