//! The session's plan (`Sessions::plan_pane`): its heading and count, the Add strip (Fight,
//! Group, Long rest and the rarer kinds under `⋯`), its rows (the session's events, the planned
//! fights with the scenes before them, the planned groups), the Scene field, and the plan's
//! message footer. The rows' words and acts are the egui plan's (`Sessions::gv_plan`,
//! `gv_plan_act`); the open row shows the event's fields, a journey's checks or how a played
//! fight ended.
//!
//! What egui measures as it draws, this view measures in each frame for the next (`Probes`):
//! the rows' heights for the scenes' fold (`Sessions::note_fold_measure`), and the rows' width
//! for a planned fight's line, which keeps the party's size while the row has room for it.
use super::{footer, with_plan};
use crate::gui::{SplatRoot, bridge, icons, kit, theme};
use crate::sessions::page::gpui_facade::{
    EventRow, FightRow, GroupRow, PlanAct, PlanRow, PlanView, SceneRow,
};
use crate::timeline::Kind;
use gpui::{
    AnyElement, Context, Div, ElementId, IntoElement, ParentElement, Pixels, ScrollHandle,
    SharedString, Styled, Window, canvas, div, prelude::*, px,
};
use std::cell::RefCell;
use std::rc::Rc;
use theme::{Density, L, M, S, Type, XS, colors};

/// The plan's GPUI state: its scroll column and what its last frame measured.
#[derive(Default)]
pub(crate) struct State {
    scroll: ScrollHandle,
    probes: Rc<RefCell<Probes>>,
}

/// What this frame measures (`now`) and what the last frame did (`then`).
#[derive(Default)]
struct Probes {
    now: Probe,
    then: Probe,
}

/// Where an open row's details write their height: the probes and the row's place.
type DetailsProbe<'a> = (&'a Rc<RefCell<Probes>>, usize);

/// One frame's measures: the pane's height, the heading and Add strip's height above the rows,
/// the rows' width, each row's height and kind, and each open row's details.
#[derive(Clone, Default, PartialEq)]
struct Probe {
    pane: Option<Pixels>,
    header: Option<Pixels>,
    width: Option<Pixels>,
    rows: Vec<Option<(RowKind, Pixels)>>,
    details: Vec<Pixels>,
    folded: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RowKind {
    Scene,
    /// The next fight still to come: the plan's first planned fight.
    NextFight,
    Other,
}

impl Probe {
    /// The fold's measure (`fold::Measure`): the rows' room, the closed rows without the scenes,
    /// the scenes, the opened rows' details, and the next fight's bottom with the details and
    /// scenes above it. `None` until a whole frame was measured.
    #[allow(clippy::type_complexity)]
    fn fold(&self) -> Option<(f32, (f32, f32, f32), Option<(f32, f32, f32)>)> {
        let viewport = f32::from(self.pane? - self.header?);
        let (mut closed, mut scenes, mut details) = (0.0, 0.0, 0.0);
        let mut next = None;
        for (at, row) in self.rows.iter().enumerate() {
            let (kind, height) = (*row)?;
            let height = f32::from(height);
            let opened = f32::from(self.details.get(at).copied().unwrap_or_default());
            if kind == RowKind::NextFight && next.is_none() {
                next = Some((closed + height, details, scenes));
            }
            if kind == RowKind::Scene {
                scenes += height;
            } else {
                closed += height - opened;
                details += opened;
            }
        }
        Some((viewport, (closed, scenes, details), next))
    }
}

/// An invisible box over its parent that writes the parent's bounds into this frame's probe.
fn probe(
    probes: &Rc<RefCell<Probes>>,
    write: impl Fn(&mut Probe, gpui::Bounds<Pixels>) + 'static,
) -> impl IntoElement {
    let probes = probes.clone();
    canvas(
        move |bounds, _, _| write(&mut probes.borrow_mut().now, bounds),
        |_, _, _, _| {},
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

/// `element` with a probe that writes its bounds.
fn probed(
    element: impl IntoElement,
    probes: &Rc<RefCell<Probes>>,
    write: impl Fn(&mut Probe, gpui::Bounds<Pixels>) + 'static,
) -> AnyElement {
    div()
        .relative()
        .w_full()
        .child(element)
        .child(probe(probes, write))
        .into_any_element()
}

fn id(words: impl std::fmt::Display) -> ElementId {
    ElementId::Name(SharedString::from(words.to_string()))
}

/// Do a plan act through the page's state.
pub(super) fn act(root: &mut SplatRoot, act: PlanAct) {
    let ctx = root.bridge.ctx().clone();
    let index = root.app.lookup.index();
    with_plan(&mut root.app, |sessions, _, book, plan| {
        sessions.gv_plan_act(act, book, plan, index.as_deref(), &ctx);
    });
}

/// A click handler that does `plan_act`.
fn on(
    cx: &mut Context<SplatRoot>,
    plan_act: PlanAct,
) -> impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static {
    cx.listener(move |root, _, _, cx| {
        act(root, plan_act.clone());
        cx.notify();
    })
}

/// The plan pane. `page_footer`: the plan is all the pane shows (the narrow Plan tab), so its
/// footer holds the page's messages too.
pub(super) fn render(
    root: &mut SplatRoot,
    page_footer: bool,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let index = root.app.lookup.index();
    let editing_group = with_plan(&mut root.app, |sessions, _, _, plan| {
        sessions.gv_editing_group_here(plan)
    })
    .unwrap_or(false);
    // The last frame's measures feed this frame's fold, as egui's do.
    let probes = root.pages.sessions.plan.probes.clone();
    let last = {
        let mut probes = probes.borrow_mut();
        probes.then = std::mem::take(&mut probes.now);
        probes.then.clone()
    };
    if let Some((viewport, rows, next)) = last.fold() {
        let changed = root
            .app
            .sessions
            .gv_fold_measure(last.folded, viewport, rows, next);
        if changed {
            cx.notify();
        }
    }
    let Some(view) = with_plan(&mut root.app, |sessions, _, book, plan| {
        sessions.gv_plan(book, plan, index.as_deref())
    }) else {
        return div().into_any_element();
    };
    {
        let mut probes = probes.borrow_mut();
        probes.now.rows = vec![None; view.rows.len()];
        probes.now.details = vec![px(0.0); view.rows.len()];
        probes.now.folded = view.folded;
    }
    let lines = with_plan(&mut root.app, |sessions, settings, book, plan| {
        sessions.gv_plan_footer(settings, book, plan, index.as_deref(), page_footer)
    })
    .unwrap_or_default();
    let on_press: footer::OnPress = Rc::new(|root, press, _, _| {
        with_plan(&mut root.app, |sessions, _, book, plan| {
            sessions.gv_plan_press(press, book, plan);
        });
    });
    let footer = footer::lines("plan-footer", lines, on_press, cx);
    let travel = travel_block(root, window, cx);
    let fight = fight_block(root, last.width, window, cx);
    let scroll = root.pages.sessions.plan.scroll.clone();
    let body: AnyElement = if editing_group {
        // The group being assembled takes the plan's place: the Encounters page's group editor.
        kit::Message::new(
            kit::Tone::Info,
            "A group is being assembled here. Finish it on the Encounters page.",
        )
        .into_any_element()
    } else {
        body(
            view,
            Opened { travel, fight },
            Measures {
                probes: &probes,
                scroll: &scroll,
                width: last.width,
            },
            window,
            cx,
        )
    };
    let c = colors();
    // The pane's probe draws last: after every row wrote its measure, a frame that measured
    // other heights or another width than the last asks for one more (as egui's fold settles).
    let pane_probe = {
        let probes = probes.clone();
        canvas(
            move |bounds, window, _| {
                let mut probes = probes.borrow_mut();
                probes.now.pane = Some(bounds.size.height);
                if probes.now != probes.then {
                    window.refresh();
                }
            },
            |_, _, _, _| {},
        )
        .absolute()
        .top_0()
        .left_0()
        .size_full()
    };
    div()
        .relative()
        .flex()
        .flex_col()
        .size_full()
        .child(div().flex_1().min_h_0().child(body))
        .when(!footer.is_empty(), |pane| {
            pane.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(S)
                    .pt(S)
                    .border_t(theme::HAIRLINE_WIDTH)
                    .border_color(c.hairline)
                    .children(footer),
            )
        })
        .child(pane_probe)
        .into_any_element()
}

/// What the open rows show besides their fields: a journey's checks, a played fight's block.
struct Opened {
    travel: Option<(String, AnyElement)>,
    fight: Option<(String, AnyElement)>,
}

/// Where the rows write their measures, the plan's scroll, and the rows' width as last measured.
struct Measures<'a> {
    probes: &'a Rc<RefCell<Probes>>,
    scroll: &'a ScrollHandle,
    width: Option<Pixels>,
}

fn body(
    view: PlanView,
    mut opened: Opened,
    measures: Measures<'_>,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let probes = measures.probes;
    let heading = plan_heading(&view, cx);
    let strip = add_strip(&view, cx);
    let primary_free = std::cell::Cell::new(view.primary_free);
    let mut rows: Vec<AnyElement> = Vec::new();
    let any = !view.rows.is_empty();
    // The reveal after an add or an act: the row after its target, or the target when it is
    // the last (`Reveal`); and an event row just opened, at the plan's top.
    let count = view.rows.len();
    let target = view.reveal.as_ref().and_then(|target| {
        view.rows
            .iter()
            .position(|row| matches!(row, PlanRow::Event(event) if event.id == *target))
            .map(|at| (at + 1).min(count - 1))
    });
    for (at, row) in view.rows.into_iter().enumerate() {
        let (kind, top) = match &row {
            PlanRow::Scene(_) => (RowKind::Scene, false),
            PlanRow::Fight(fight) if fight.position == 0 => (RowKind::NextFight, false),
            PlanRow::Event(event) => (
                RowKind::Other,
                view.reveal_open.as_deref() == Some(event.id.as_str()),
            ),
            _ => (RowKind::Other, false),
        };
        let element = match row {
            PlanRow::Event(row) => {
                let block = opened
                    .travel
                    .take_if(|(journey, _)| *journey == row.id)
                    .or_else(|| opened.fight.take_if(|(event, _)| *event == row.id))
                    .map(|(_, block)| block);
                event_row(at, row, block, (probes, at), &primary_free, window, cx)
            }
            PlanRow::Fight(row) => fight_row(
                row,
                &primary_free,
                &view.fight_blocked,
                (probes, at, measures.width),
                window,
                cx,
            ),
            PlanRow::Group(row) => group_row(row, &view.group_blocked, cx),
            PlanRow::Scene(row) => scene_row(row, &view.fight_blocked, cx),
        };
        let element = probed(element, probes, move |probe, bounds| {
            if let Some(row) = probe.rows.get_mut(at) {
                *row = Some((kind, bounds.size.height));
            }
        });
        let align = if top {
            Some(kit::reveal::Align::Top)
        } else {
            (target == Some(at)).then_some(kit::reveal::Align::Least)
        };
        rows.push(kit::reveal::revealed(element, measures.scroll, align));
    }
    let empty = (!any).then(|| {
        kit::empty_state(
            icons::SESSIONS,
            "Nothing planned yet",
            "Use Add to plan a fight, a group, or an event.",
            None::<Div>,
        )
    });
    let scene = view.scene_field.map(|words| scene_field(words, cx));
    let header = probed(
        div().flex().flex_col().child(heading).child(strip),
        probes,
        |probe, bounds| probe.header = Some(bounds.size.height),
    );
    let rows = probed(
        div().flex().flex_col().w_full().children(rows),
        probes,
        |probe, bounds| probe.width = Some(bounds.size.width),
    );
    kit::scroll::scroll_column_with(
        "session-plan",
        measures.scroll,
        div()
            .flex()
            .flex_col()
            .w_full()
            .pr(M)
            .child(header)
            .child(rows)
            .children(empty)
            .children(scene),
    )
    .into_any_element()
}

/// `Tonight 3`, and `N scenes` in the room after it while the scenes fold or the DM opened them:
/// the section caret and a link that opens the folded scenes in place and closes them again.
fn plan_heading(view: &PlanView, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let heading = kit::label(view.heading, view.count);
    let Some((count, folded)) = view.fold else {
        return heading.into_any_element();
    };
    let words = crate::design::count(count, "scene", "scenes");
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(L)
        .child(heading)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(XS)
                .child(kit::icon(
                    if folded { icons::NEXT } else { icons::DOWN },
                    Type::Body,
                    c.muted,
                ))
                .child(
                    kit::Link::new("plan-scene-fold", words)
                        .on_click(on(cx, PlanAct::SceneFold(folded))),
                ),
        )
        .into_any_element()
}

/// `Add`: Fight and Group (menus of what the campaign has), Long rest, and the rarer kinds and
/// Scene under `⋯`.
fn add_strip(view: &PlanView, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let blocked = view.add_blocked;
    let mut fight = kit::MenuButton::new(
        "plan-add-fight",
        kit::Button::quiet("plan-add-fight-button", "Fight")
            .icon(icons::ENCOUNTERS)
            .name("Add a fight")
            .density(Density::Read)
            .blocked(view.fight_blocked.clone()),
    );
    for name in &view.fights {
        fight = fight.item(
            kit::MenuItem::new(name.clone()).on_click(on(cx, PlanAct::PlanName(name.clone()))),
        );
    }
    fight = fight.item(
        kit::MenuItem::new("New encounter")
            .icon(icons::ADD)
            .on_click(on(cx, PlanAct::NewFight)),
    );
    let mut group = kit::MenuButton::new(
        "plan-add-group",
        kit::Button::quiet("plan-add-group-button", "Group")
            .icon(icons::GROUP)
            .name("Add a group")
            .density(Density::Read)
            .blocked(view.group_blocked.clone()),
    );
    for name in &view.groups {
        group = group.item(
            kit::MenuItem::new(name.clone()).on_click(on(cx, PlanAct::PlanName(name.clone()))),
        );
    }
    group = group.item(
        kit::MenuItem::new("New group")
            .icon(icons::ADD)
            .on_click(on(cx, PlanAct::NewGroup)),
    );
    let rest = kit::Button::quiet("plan-add-long-rest", Kind::LongRest.word())
        .icon(bridge_icon(Kind::LongRest))
        .name("Add Long rest")
        .density(Density::Read)
        .blocked(blocked)
        .on_click(on(cx, PlanAct::Add(Kind::LongRest)));
    let mut more = kit::MenuButton::new(
        "plan-add-more",
        kit::Button::quiet("plan-add-more-button", "")
            .icon(icons::EXPAND)
            .name("More to add")
            .density(Density::Read),
    );
    for kind in [Kind::Day, Kind::ShortRest, Kind::Travel, Kind::Event] {
        more = more.item(
            kit::MenuItem::new(kind.word())
                .icon(bridge_icon(kind))
                .blocked(blocked)
                .on_click(on(cx, PlanAct::Add(kind))),
        );
    }
    more = more.item(
        kit::MenuItem::new("Scene")
            .icon(icons::CHECK_SQUARE)
            .on_click(on(cx, PlanAct::OpenScene)),
    );
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .py(XS)
        .child(kit::text(Type::Small, c.muted, "Add").pr(XS))
        .child(fight)
        .child(group)
        .child(rest)
        .child(more)
        .into_any_element()
}

/// A Timeline kind's icon, as the egui plan names it.
fn bridge_icon(kind: Kind) -> &'static str {
    crate::timeline::kind_icon(kind)
}

/// One row: a lead (chevron, box or icon), its title and second line, and its actions at the
/// right; a rule under it.
fn row_frame(
    lead: Option<AnyElement>,
    title: AnyElement,
    second: Option<AnyElement>,
    actions: Vec<AnyElement>,
    details: Option<(AnyElement, DetailsProbe<'_>)>,
) -> AnyElement {
    let c = colors();
    // The title's line holds the lead, the title and the actions; the second line and the
    // open row's details run under the title to the row's right edge.
    div()
        .flex()
        .flex_col()
        .w_full()
        .py(S)
        .border_b(theme::HAIRLINE_WIDTH)
        .border_color(c.hairline)
        .child(
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap(S)
                .child(
                    div()
                        .w(theme::ICON_SLOT)
                        .flex_none()
                        .flex()
                        .justify_center()
                        .pt(XS)
                        .children(lead),
                )
                .child(div().flex_1().min_w_0().child(title))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(S)
                        .flex_none()
                        .children(actions),
                ),
        )
        .children(second.map(|second| div().pl(theme::ICON_SLOT + S).child(second)))
        .children(details.map(|(details, (probes, at))| {
            // The open row's details, which the fold counts apart from the closed row.
            probed(
                div().pl(theme::ICON_SLOT + S).pt(S).child(details),
                probes,
                move |probe, bounds| {
                    if let Some(details) = probe.details.get_mut(at) {
                        *details = bounds.size.height;
                    }
                },
            )
        }))
        .into_any_element()
}

/// A row's title as a link that opens or reads it.
fn title_link(
    id_words: String,
    words: String,
    click: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> AnyElement {
    kit::Link::new(id(id_words), words)
        .wrap()
        .on_click(click)
        .into_any_element()
}

/// A scene's words: its links underlined (each opens its entry in Lookup), the rest plain.
fn scene_words(line: usize, words: &str, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let mut row = div().flex().flex_row().flex_wrap().items_center().gap_x(XS);
    let mut rest = words;
    let mut at = 0;
    while let Some(start) = rest.find("[[") {
        let Some(end) = rest[start..].find("]]") else {
            break;
        };
        for word in rest[..start].split_whitespace() {
            row = row.child(kit::text(Type::Body, c.text, word.to_owned()));
        }
        let inner = &rest[start + 2..start + end];
        let (target, shown) = inner.split_once('|').unwrap_or((inner, inner));
        let target = target.split('#').next().unwrap_or(target).to_owned();
        row = row.child(
            kit::Link::new(id(format!("plan-scene-link-{line}-{at}")), shown.to_owned())
                .wrap()
                .on_click(on(cx, PlanAct::ReadEncounter(target))),
        );
        at += 1;
        rest = &rest[start + end + 2..];
    }
    for word in rest.split_whitespace() {
        row = row.child(kit::text(Type::Body, c.text, word.to_owned()));
    }
    row.into_any_element()
}

fn small(words: impl Into<SharedString>) -> AnyElement {
    kit::text(Type::Small, colors().muted, words).into_any_element()
}

fn in_notes() -> AnyElement {
    let c = colors();
    kit::status(&kit::Status::new(icons::OK, "In notes", c.document.saved)).into_any_element()
}

fn row_menu(name: &str, items: Vec<kit::MenuItem>) -> AnyElement {
    let mut menu = kit::MenuButton::new(
        id(format!("plan-menu-{name}")),
        kit::Button::quiet(id(format!("plan-menu-button-{name}")), "")
            .icon(icons::EXPAND)
            .name(format!("More actions for {name}"))
            .density(Density::Read),
    );
    for item in items {
        menu = menu.item(item);
    }
    menu.into_any_element()
}

fn primary_or_secondary(
    free: &std::cell::Cell<bool>,
    words_id: String,
    words: &'static str,
) -> kit::Button {
    if free.replace(false) {
        kit::Button::primary(id(words_id), words)
    } else {
        kit::Button::secondary(id(words_id), words)
    }
}

fn event_row(
    at: usize,
    row: EventRow,
    checks: Option<AnyElement>,
    probe: (&Rc<RefCell<Probes>>, usize),
    primary_free: &std::cell::Cell<bool>,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let EventRow {
        id: event,
        icon: _,
        label,
        summary,
        warning,
        restores,
        open,
        running,
        saved,
        saved_fight,
        ended,
        listed,
        linked,
        reference,
        rolls,
        details,
    } = row;
    let lead = kit::IconButton::new(
        id(format!("plan-event-open-{event}")),
        if open { icons::DOWN } else { icons::NEXT },
        if open { "Close" } else { "Open" },
    )
    .on_click(on(cx, PlanAct::ToggleEvent(event.clone())))
    .into_any_element();
    let title = title_link(
        format!("plan-event-{event}"),
        label.clone(),
        on(cx, PlanAct::ToggleEvent(event.clone())),
    );
    let second = {
        let mut line = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_baseline()
            .gap_x(XS);
        if running {
            line = line.child(
                kit::Button::quiet(id(format!("plan-back-{event}")), summary.clone())
                    .plain()
                    .on_click(on(cx, PlanAct::BackToRun)),
            );
        } else {
            match warning
                .as_deref()
                .and_then(|w| summary.find(w).map(|i| (i, w.len())))
            {
                Some((start, len)) => {
                    line = line
                        .child(small(summary[..start].to_owned()))
                        .child(kit::text(
                            Type::Small,
                            c.result.warning,
                            summary[start..start + len].to_owned(),
                        ))
                        .child(small(summary[start + len..].to_owned()));
                }
                None if !summary.is_empty() => line = line.child(small(summary.clone())),
                None => {}
            }
        }
        if let Some(status) = restores {
            if !summary.is_empty() {
                line = line.child(small("·"));
            }
            line = line.child(kit::status(&bridge::status(&status)));
        }
        line.into_any_element()
    };
    let mut actions = Vec::new();
    if let Some(saved) = saved {
        let mut items = Vec::new();
        if let Some(path) = saved_fight {
            items.push(
                kit::MenuItem::new("Start a new run").on_click(on(cx, PlanAct::StartFight(path))),
            );
        }
        if let Some(run) = saved.id {
            items.push(
                kit::MenuItem::new("Delete the saved run")
                    .on_click(on(cx, PlanAct::DeleteRun(run))),
            );
        }
        items.push(
            kit::MenuItem::new("Remove").on_click(on(cx, PlanAct::RemoveEvent(event.clone()))),
        );
        actions.push(
            primary_or_secondary(primary_free, format!("plan-resume-{event}"), "Resume")
                .icon(icons::RUN)
                .name(format!("Resume: {label}"))
                .density(Density::Read)
                .on_click(on(cx, PlanAct::Resume(saved.path.clone())))
                .into_any_element(),
        );
        actions.push(row_menu(&label, items));
    } else {
        if let Some(run) = ended {
            actions.push(if listed {
                in_notes()
            } else {
                kit::Button::quiet(id(format!("plan-rewards-{event}")), "Add rewards")
                    .name(format!("Add rewards: {label}"))
                    .density(Density::Read)
                    .on_click(on(cx, PlanAct::AddRewards(event.clone(), run)))
                    .into_any_element()
            });
        }
        if let Some(link) = reference {
            actions.push(if linked {
                in_notes()
            } else {
                kit::Button::quiet(id(format!("plan-add-notes-{event}")), "Add to notes")
                    .name(format!("Add to notes: {label}"))
                    .density(Density::Read)
                    .on_click(on(cx, PlanAct::AddToNotes(link)))
                    .into_any_element()
            });
        }
        let mut items = Vec::new();
        if rolls {
            items.push(
                kit::MenuItem::new("Roll for encounters")
                    .on_click(on(cx, PlanAct::RollForEncounters(event.clone()))),
            );
        }
        items.push(
            kit::MenuItem::new("Remove")
                .blocked(running.then_some("This run is open. Close it first."))
                .on_click(on(cx, PlanAct::RemoveEvent(event.clone()))),
        );
        actions.push(row_menu(&label, items));
    }
    let details = details.map(|details| {
        (
            event_details(at, &event, details, checks, window, cx),
            probe,
        )
    });
    row_frame(Some(lead), title, Some(second), actions, details)
}

/// The open journey's checks block (`travel_block`): **How long** with its range or help, then
/// the Timeline's checks view in the plan's hands (`timeline::checks::Host::Plan`). `None`
/// when the open row is not a journey.
fn travel_block(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> Option<(String, AnyElement)> {
    use crate::gui::pages::timeline::checks::{Host, render_in};
    let index = root.app.lookup.index();
    let view = with_plan(&mut root.app, |sessions, _, book, plan| {
        sessions.gv_travel(book, plan, index.as_deref())
    })
    .flatten()?;
    let tables = index
        .map(|index| index.loot_sources().tables.clone())
        .unwrap_or_default();
    let (typed, commit, blur) = (view.id.clone(), view.id.clone(), view.id.clone());
    let length = kit::TextField::new(
        id(format!("plan-travel-length-{}", view.id)),
        view.length.clone(),
    )
    .name("How long")
    .focus(view.focus_length)
    .on_change(cx.listener(move |root, words: &str, _, cx| {
        let book = &mut root.app.timeline;
        if let Some(event) = book.events.iter().find(|e| e.id == typed).cloned() {
            book.length.type_words(&event, words);
        }
        cx.notify();
    }))
    .on_enter(crate::gui::pages::trackers::act(cx, move |root| {
        crate::timeline::checks::commit_length(&mut root.app.timeline, &commit);
    }))
    .on_blur(crate::gui::pages::trackers::act(cx, move |root| {
        crate::timeline::checks::commit_length(&mut root.app.timeline, &blur);
    }));
    let block = render_in(
        root,
        Host::Plan,
        &view.id,
        view.index,
        (&view.length, view.start),
        (&tables, &|name: &str| view.plan.get(name)),
        window,
        cx,
    );
    let element = div()
        .flex()
        .flex_col()
        .pt(XS)
        .child(kit::field(
            "How long",
            view.help.map(Into::into),
            None,
            length,
        ))
        .child(block)
        .into_any_element();
    Some((view.id, element))
}

/// The open event's fields: Title and Notes (or a played fight's lines and Where), a long
/// rest's ruling, and `Timeline`. A journey's checks come first, under its row.
fn event_details(
    _at: usize,
    event: &str,
    details: crate::sessions::page::gpui_facade::EventDetails,
    checks: Option<AnyElement>,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let mut column = div().flex().flex_col().gap(XS);
    if details.fight.is_some() {
        // How the fight ended (`fight_block`), or, while it runs, where its HP will show.
        column = if details.running {
            column.child(kit::text(
                Type::Small,
                c.muted,
                "This fight is running. Its final HP shows here when it ends.",
            ))
        } else {
            column.children(checks)
        };
        let place = {
            let event = event.to_owned();
            kit::TextField::new(id(format!("plan-event-place-{event}")), details.place)
                .name("Where")
                .on_change(cx.listener(move |root, words: &str, _, cx| {
                    act(root, PlanAct::EventPlace(event.clone(), words.to_owned()));
                    cx.notify();
                }))
        };
        column = column.child(kit::field("Where", None, None, place));
    } else {
        column = column.children(checks);
        let title = {
            let event = event.to_owned();
            kit::TextField::new(id(format!("plan-event-title-{event}")), details.title)
                .name("Title")
                .on_change(cx.listener(move |root, words: &str, _, cx| {
                    act(root, PlanAct::EventTitle(event.clone(), words.to_owned()));
                    cx.notify();
                }))
        };
        let notes = {
            let event = event.to_owned();
            kit::TextEditor::new(id(format!("plan-event-notes-{event}")), details.notes)
                .name("Notes on this event")
                .rows(2)
                .on_change(cx.listener(move |root, words: &str, _, cx| {
                    act(root, PlanAct::EventNotes(event.clone(), words.to_owned()));
                    cx.notify();
                }))
        };
        column = column
            .child(kit::field("Title", None, None, title))
            .child(kit::field("Notes", None, None, notes));
        if let Some(restores) = details.rest {
            let status = crate::timeline::page::long_rest_status(restores);
            column = column.child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(L)
                    .child(kit::status(&bridge::status(&status)))
                    .child(
                        kit::Button::quiet(
                            id(format!("plan-rest-ruling-{event}")),
                            if restores {
                                "Mark not at full"
                            } else {
                                "Mark everyone at full"
                            },
                        )
                        .density(Density::Read)
                        .on_click(on(cx, PlanAct::RestRuling(event.to_owned(), !restores))),
                    ),
            );
        }
    }
    // A played fight's block has its own `Timeline` among its links.
    let timeline = (details.fight.is_none()).then(|| {
        kit::Button::quiet(id(format!("plan-event-timeline-{event}")), "Timeline")
            .plain()
            .line_start()
            .on_click(on(cx, PlanAct::OpenEventOnTimeline(event.to_owned())))
    });
    column.children(timeline).into_any_element()
}

/// The open played fight's block (`fight_block`): its players' final HP in the party's order,
/// two or three to a line, each with its whole line on hover; `Log · Whole run · Timeline`; and
/// the Log, newest first, one line an entry cut at a whole word, with what Whole run has more.
fn fight_block(
    root: &mut SplatRoot,
    width: Option<Pixels>,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> Option<(String, AnyElement)> {
    use crate::gui::pages::trackers::measure;
    use crate::sessions::page::gpui_facade::FightBlockView;
    let (event, view) = with_plan(&mut root.app, |sessions, _, book, plan| {
        sessions.gv_fight_block(book, plan)
    })
    .flatten()?;
    let c = colors();
    let (cells, path, log) = match view {
        FightBlockView::Fault(line) => {
            let lines = kit::messages::lines(
                id(format!("plan-fight-fault-{event}")),
                vec![line],
                |_: &kit::messages::Pressed<()>, _: &mut Window, _: &mut gpui::App| {},
            );
            return Some((
                event,
                div().flex().flex_col().children(lines).into_any_element(),
            ));
        }
        FightBlockView::Ended { cells, path, log } => (cells, path, log),
    };
    // The details' room: the rows' width, less the row's lead and its gap.
    let room = width.map_or(theme::FORM_WIDTH, |width| width - theme::ICON_SLOT - S);
    let words = |cell: &crate::sessions::page::gpui_facade::FightCell| {
        let known = |n: Option<i64>| n.map_or_else(|| "?".to_owned(), |n| n.to_string());
        (
            cell.first.clone(),
            format!(
                " {}",
                crate::design::fraction(known(cell.hp), known(cell.max))
            ),
            (cell.temp > 0).then(|| format!(" {}", crate::design::temp_hp_text(cell.temp))),
        )
    };
    let widest = cells
        .iter()
        .map(|cell| {
            let (first, hp, temp) = words(cell);
            measure::width(
                window,
                Type::Small,
                &format!("{first}{hp}{}", temp.unwrap_or_default()),
            )
        })
        .fold(px(0.0), Pixels::max);
    let across = if widest * 3.0 + S * 2.0 <= room { 3 } else { 2 };
    let cell_width = (room - S * (across - 1) as f32) / across as f32;
    let grid = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .gap_x(S)
        .gap_y(XS)
        .children(cells.iter().enumerate().map(|(at, cell)| {
            let (first, hp, temp) = words(cell);
            let state = crate::design::hp_state(cell.hp, cell.max);
            let cell_div = div()
                .id(id(format!("plan-fight-cell-{event}-{at}")))
                .w(cell_width)
                .min_h(Type::Small.line())
                .flex()
                .flex_row()
                .flex_wrap()
                .child(kit::text(Type::Small, c.muted, first))
                .child(kit::text(Type::Small, kit::hp::hp_color(state), hp))
                .children(temp.map(|temp| kit::text(Type::Small, c.hp.temp, temp)))
                .tooltip(kit::tooltip::tip(cell.detail.clone().into()));
            kit::target(cell_div, cell.detail.clone())
        }));
    let dot = || kit::text(Type::Small, c.muted, "·");
    let mut links = div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_center()
        .gap_x(XS)
        .child(
            kit::Link::new(id(format!("plan-fight-log-{event}")), "Log")
                .on_click(on(cx, PlanAct::FightLog)),
        )
        .child(dot());
    let whole = kit::Link::new(id(format!("plan-fight-run-{event}")), "Whole run");
    links = links.child(match path {
        Some(path) => whole.on_click(on(cx, PlanAct::WholeRun(path))),
        None => whole,
    });
    links = links.child(dot()).child(
        kit::Link::new(id(format!("plan-fight-timeline-{event}")), "Timeline")
            .on_click(on(cx, PlanAct::OpenEventOnTimeline(event.clone()))),
    );
    let mut block = div().flex().flex_col().gap(XS).child(grid).child(links);
    if let Some(log) = log {
        // A capped block, not a scroll column of its own: the plan is the scroll column.
        let fits = crate::sessions::page::gpui_facade::log_room(f32::from(Type::Small.line()), 0.0);
        let mut rows: Vec<(String, String)> = log
            .iter()
            .take(fits)
            .map(|entry| {
                let whole = entry.replace(['\n', '\r'], " ");
                (measure::fit_words(window, Type::Small, &whole, room), whole)
            })
            .collect();
        let cut = rows.iter().any(|(shown, whole)| shown != whole);
        let notice = (log.len() > rows.len() || cut).then(|| {
            rows.truncate(fits - 1);
            crate::sessions::page::gpui_facade::log_notice(log.len() - rows.len())
        });
        for (at, (shown, whole)) in rows.into_iter().enumerate() {
            let line = div()
                .id(id(format!("plan-fight-log-{event}-{at}")))
                .child(kit::text(Type::Small, c.muted, shown.clone()).whitespace_nowrap());
            block = block.child(if shown == whole {
                line.into_any_element()
            } else {
                line.tooltip(kit::tooltip::tip(whole.into()))
                    .into_any_element()
            });
        }
        if let Some(notice) = notice {
            block = block.child(kit::text(Type::Small, c.muted, notice));
        }
    }
    Some((event, block.into_any_element()))
}

fn fight_row(
    row: FightRow,
    primary_free: &std::cell::Cell<bool>,
    blocked: &Option<String>,
    (probes, at, width): (&Rc<RefCell<Probes>>, usize, Option<Pixels>),
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let FightRow {
        position,
        name,
        path,
        id: encounter,
        roster,
        roster_with_party,
        difficulty,
        running,
        saved,
        open,
    } = row;
    let lead = kit::IconButton::new(
        id(format!("plan-fight-open-{path}-{position}")),
        if open { icons::DOWN } else { icons::NEXT },
        if open { "Close" } else { "Open" },
    )
    .on_click(on(cx, PlanAct::TogglePlanned(path.clone())))
    .into_any_element();
    let title = title_link(
        format!("plan-fight-{path}-{position}"),
        name.clone(),
        on(cx, PlanAct::TogglePlanned(path.clone())),
    );
    // The roster and the difficulty as one line that wraps between words, its word strong
    // (`prepare_plan_line`). The form never adds a line to the roster's own: where it would,
    // the party's size gives way first (`5 party`, the same on every row).
    let roster = match (roster_with_party, width) {
        (Some(with_party), Some(width)) => {
            let room = width - theme::ICON_SLOT - S;
            let form: String = difficulty.iter().map(|(part, _)| part.as_str()).collect();
            let lines = |words: &str| {
                crate::gui::pages::trackers::measure::lines(window, Type::Small, words, room)
            };
            let whole = format!("{with_party} · {}", form.trim_start_matches(" · "));
            if lines(&whole) <= lines(&with_party) {
                with_party
            } else {
                roster
            }
        }
        _ => roster,
    };
    let mut words = roster;
    let mut strong = Vec::new();
    for (at, (part, is_word)) in difficulty.into_iter().enumerate() {
        let part = if words.is_empty() {
            part.trim_start_matches(" · ").to_owned()
        } else if at == 0 && !part.starts_with(" · ") {
            format!(" · {part}")
        } else {
            part
        };
        if is_word {
            strong.push(words.len()..words.len() + part.len());
        }
        words.push_str(&part);
    }
    let highlights = strong.into_iter().map(|range| {
        (
            range,
            gpui::HighlightStyle {
                color: Some(c.text),
                font_weight: Some(gpui::FontWeight::SEMIBOLD),
                ..Default::default()
            },
        )
    });
    let second = div()
        .ty_small()
        .text_color(c.muted)
        .child(gpui::StyledText::new(words).with_highlights(highlights));
    let mut items = vec![
        kit::MenuItem::new("Remove from tonight")
            .on_click(on(cx, PlanAct::RemoveFight(position, name.clone()))),
    ];
    if let Some(saved) = &saved {
        items.push(
            kit::MenuItem::new("Start a new run")
                .blocked(blocked.clone())
                .on_click(on(cx, PlanAct::StartFight(path.clone()))),
        );
        if let Some(run) = saved.id {
            items.push(
                kit::MenuItem::new("Delete the saved run")
                    .blocked(blocked.clone())
                    .on_click(on(cx, PlanAct::DeleteRun(run))),
            );
        }
    }
    items.push(
        kit::MenuItem::new("Edit encounter").on_click(on(cx, PlanAct::EditEncounter(encounter))),
    );
    let mut actions = Vec::new();
    if !running {
        let (words, act_now) = match &saved {
            Some(saved) => ("Resume", PlanAct::Resume(saved.path.clone())),
            None => ("Start fight", PlanAct::StartFight(path.clone())),
        };
        actions.push(
            primary_or_secondary(primary_free, format!("plan-start-{path}-{position}"), words)
                .icon(icons::RUN)
                .name(format!("{words}: {name}"))
                .density(Density::Read)
                .blocked(blocked.clone())
                .on_click(on(cx, act_now))
                .into_any_element(),
        );
    }
    actions.push(row_menu(&name, items));
    let details = open.then(|| {
        (
            kit::Button::quiet(id(format!("plan-read-encounter-{path}")), "Read encounter")
                .plain()
                .line_start()
                .on_click(on(cx, PlanAct::ReadEncounter(name.clone())))
                .into_any_element(),
            (probes, at),
        )
    });
    row_frame(
        Some(lead),
        title,
        Some(second.into_any_element()),
        actions,
        details,
    )
}

trait SmallText {
    fn ty_small(self) -> Self;
}

impl SmallText for Div {
    fn ty_small(self) -> Self {
        use theme::TypeExt;
        self.ty(Type::Small)
    }
}

fn group_row(row: GroupRow, blocked: &Option<String>, cx: &mut Context<SplatRoot>) -> AnyElement {
    let GroupRow {
        position,
        name,
        path,
        summary,
        edit_word,
        joins_fight,
        warnings,
        problems,
        refused,
    } = row;
    let lead = kit::icon(icons::GROUP, Type::Body, colors().muted).into_any_element();
    let title = title_link(
        format!("plan-group-{path}"),
        name.clone(),
        on(cx, PlanAct::ReadGroup(path.clone())),
    );
    let mut second = div().flex().flex_col().gap(XS).child(small(summary));
    for words in warnings.into_iter().chain(problems) {
        second = second.child(kit::Message::new(kit::Tone::Warning, words).small());
    }
    if let Some(reason) = refused {
        second = second.child(kit::Message::new(kit::Tone::Warning, reason).small());
    }
    let mut items = vec![
        kit::MenuItem::new("Remove from tonight")
            .blocked(blocked.clone())
            .on_click(on(cx, PlanAct::RemoveGroup(position, name.clone()))),
    ];
    if joins_fight && edit_word == "Edit group" {
        items.push(
            kit::MenuItem::new("Edit group")
                .blocked(blocked.clone())
                .on_click(on(cx, PlanAct::EditGroup(path.clone()))),
        );
    }
    let action = if joins_fight {
        kit::Button::quiet(id(format!("plan-group-join-{path}")), "Add to fight…")
            .icon(icons::ADD)
            .name(format!("Add {name} to fight"))
            .density(Density::Read)
            .on_click(on(cx, PlanAct::AddToFight))
    } else {
        kit::Button::quiet(id(format!("plan-group-edit-{path}")), edit_word)
            .name(format!("Edit group {name}"))
            .density(Density::Read)
            .blocked(blocked.clone())
            .on_click(on(cx, PlanAct::EditGroup(path.clone())))
    };
    row_frame(
        Some(lead),
        title,
        Some(second.into_any_element()),
        vec![action.into_any_element(), row_menu(&name, items)],
        None,
    )
}

fn scene_row(row: SceneRow, blocked: &Option<String>, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let SceneRow {
        line,
        done,
        words,
        detail,
        not_found,
        start,
        log,
        ..
    } = row;
    let plain = crate::sessions::plain_links(&words);
    let lead = kit::Checkbox::new(id(format!("plan-scene-box-{line}-{plain}")), "", done)
        .on_click(on(cx, PlanAct::SceneTick(line)))
        .into_any_element();
    let title = scene_words(line, &words, cx);
    let second = if let Some(detail) = detail {
        Some(small(detail))
    } else if not_found {
        Some(kit::text(Type::Small, c.result.warning, "Not found").into_any_element())
    } else {
        None
    };
    let mut items = Vec::new();
    if start.is_some() {
        items.push(
            kit::MenuItem::new("Start fight")
                .icon(icons::RUN)
                .blocked(blocked.clone())
                .on_click(on(cx, PlanAct::SceneStart(line))),
        );
    }
    let mut actions = Vec::new();
    if log {
        actions.push(
            kit::Button::quiet(id(format!("plan-scene-log-{line}")), "Add to Log")
                .name(format!("Add to Log: {plain}"))
                .density(Density::Read)
                .on_click(on(cx, PlanAct::SceneLog(line)))
                .into_any_element(),
        );
    }
    items.push(
        kit::MenuItem::new("Remove from tonight")
            .icon(icons::REMOVE)
            .on_click(on(cx, PlanAct::SceneRemove(line))),
    );
    actions.push(row_menu(&plain, items));
    row_frame(Some(lead), title, second, actions, None)
}

/// The Scene field: `Plan a scene`, with Add and Cancel.
fn scene_field(words: String, cx: &mut Context<SplatRoot>) -> AnyElement {
    let entity = cx.entity().downgrade();
    let field = kit::TextField::new("plan-scene-field", words)
        .name("Plan a scene: words, a person or a place")
        .placeholder("Plan a scene")
        .on_change(cx.listener(|root, words: &str, _, cx| {
            act(root, PlanAct::SceneWords(words.to_owned()));
            cx.notify();
        }))
        .on_enter(move |_, cx| {
            let _ = entity.update(cx, |root, cx| {
                act(root, PlanAct::SceneAdd);
                cx.notify();
            });
        });
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .pt(S)
        .child(field)
        .child(
            div()
                .flex()
                .flex_row()
                .gap(S)
                .child(
                    kit::Button::secondary("plan-scene-add", "Add")
                        .density(Density::Read)
                        .on_click(on(cx, PlanAct::SceneAdd)),
                )
                .child(
                    kit::Button::quiet("plan-scene-cancel", "Cancel")
                        .density(Density::Read)
                        .on_click(on(cx, PlanAct::SceneCancel)),
                ),
        )
        .into_any_element()
}
