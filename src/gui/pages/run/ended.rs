//! The Ended phase (egui `view::ended_view`, `ended_panel`, `loot::loot_section`): the title
//! with the outcome, the save state, Undo of the ending or Continue fight (or why not), Close
//! run; the final state's rows and the loot; beside them sync, Prep and the combat log.
use super::{on_run, panel, roster, run_title, save_status};
use crate::encounter_run::loot::{Holder, place};
use crate::encounter_run::{Phase, Run, Unend};
use crate::gui::{SplatRoot, icons, kit, theme};
use gpui::{
    AnyElement, ClickEvent, Context, ElementId, IntoElement, ParentElement, Styled, Window, div,
    prelude::*, px,
};
use splat::content::loot::Loot;
use theme::{Density, Type, colors};

/// The Ended phase.
pub(super) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let room = root
        .pages
        .run
        .width
        .unwrap_or_else(|| super::page_width(root, window));
    let window_width = window.viewport_size().width;
    let height = window.viewport_size().height;
    let side = super::side_width(super::Side::Ended, room, window_width, cx);
    let Some(run) = root.app.encounters.run() else {
        return div().into_any_element();
    };
    let c = colors();
    let outcome = match &run.state.phase {
        Phase::Ended { outcome } => outcome.clone(),
        _ => String::new(),
    };
    let rows_width = if run.hide_panel {
        room
    } else {
        room - side - theme::M
    };
    let order = run.state.order.clone();
    let plan = roster::plan(
        run,
        &order,
        f32::from(rows_width - theme::S - theme::SCROLL_BAR_WIDTH),
        f32::from(height) - 160.0,
        window,
    );
    let scale = plan.scale;
    let title = div()
        .flex()
        .flex_col()
        .flex_1()
        .min_w_0()
        .child(run_title(run, run.state_words(false)))
        .child(kit::text(
            Type::Small,
            c.muted,
            format!("Outcome: {outcome}"),
        ));
    let close_reason = run.close_reason();
    let close = div()
        .flex()
        .flex_col()
        .items_end()
        .child(super::footer::close_button(run, "run-close", true, cx))
        .children(close_reason.map(|reason| kit::text(Type::Small, c.muted, reason)));
    let header = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::M)
        .min_h(theme::L * 4.0)
        .flex_none()
        .child(title)
        .child(save_status(run))
        .children(undo_or_continue(run, cx))
        .child(close);
    let rows: Vec<AnyElement> = order
        .iter()
        .enumerate()
        .map(|(at, &index)| {
            roster::row(
                root,
                run,
                index,
                roster::TurnMark::None,
                &plan,
                at,
                roster::Use::Final,
                cx,
            )
        })
        .collect();
    let left = div()
        .flex()
        .flex_col()
        .w(rows_width)
        .flex_none()
        .h_full()
        .pr(theme::S)
        .child(kit::scroll_column(
            "run-final-state",
            div()
                .flex()
                .flex_col()
                .max_w(px(scale.max_width))
                .child(kit::label("Final state", 0))
                .child(kit::gap(theme::XS))
                .children(rows)
                .child(kit::gap(theme::L))
                .children(loot_section(run, cx)),
        ));
    let mut body = div().flex().flex_row().flex_1().min_h_0().child(left);
    if !run.hide_panel {
        let panel = ended_panel(run, side, cx).into_any_element();
        body = body.child(div().w(theme::M).flex_none()).child(
            super::side_column(super::Side::Ended, room, window_width)
                .render(room, panel, window, cx),
        );
    }
    let messages = super::footer::messages(run, false, cx);
    div()
        .flex()
        .flex_col()
        .size_full()
        .child(header)
        .child(kit::rule())
        .child(kit::gap(theme::S))
        .child(body)
        .when(!messages.is_empty(), |page| {
            page.child(
                div()
                    .flex()
                    .flex_col()
                    .flex_none()
                    .gap(theme::S)
                    .pt(theme::S)
                    .children(messages),
            )
        })
        .into_any_element()
}

/// Undo of the ending (the way the fight goes on), or Continue fight, or why it cannot go on.
fn undo_or_continue(run: &Run, cx: &mut Context<SplatRoot>) -> Vec<AnyElement> {
    let c = colors();
    let mut parts = Vec::new();
    let continues = run.undo_continues() && run.undo_blocked().is_none();
    if !continues {
        if let Some(reason) = run.continue_words() {
            parts.push(
                div()
                    .max_w(theme::L * 15.0)
                    .child(kit::text(Type::Small, c.muted, reason))
                    .into_any_element(),
            );
        } else {
            parts.push(
                kit::Button::secondary("run-continue-fight", "Continue fight")
                    .icon(icons::RUN)
                    .density(Density::Glance)
                    .on_click(on_run(cx, |run| run.wants_unend = Some(Unend::Continue)))
                    .into_any_element(),
            );
        }
    }
    if !run.history.is_empty() && run.undo_blocked().is_none() {
        let label = run.undo_label();
        parts.push(
            super::undo_button(run, "run-undo", label, Density::Glance)
                .on_click(on_run(cx, |run| {
                    run.undo();
                    run.return_focus = run.lookup_open;
                }))
                .into_any_element(),
        );
    }
    parts
}

/// Beside the final state: sync that has not settled, Prep and the combat log, each a section
/// (`ended_panel`).
fn ended_panel(run: &Run, width: gpui::Pixels, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let mut column = div().flex().flex_col().gap(theme::XS);
    if !run.setup_details.is_empty() {
        column = column
            .child(kit::Section::new("run-ended-prep", "Prep", false).child(panel::prep(run, cx)));
    }
    column = column
        .child(kit::Section::new("run-ended-log", "Combat log", false).child(panel::log(run)));
    div()
        .flex()
        .flex_col()
        .w(width)
        .flex_none()
        .h_full()
        .pl(theme::L)
        .border_l(theme::HAIRLINE_WIDTH)
        .border_color(c.hairline)
        .child(kit::scroll_column("run-ended-panel", column))
}

/// One loot line: its mark, the words (a note's name as a link), and where a roll came from.
fn loot_line(mark: &'static str, line: &str, source: Option<String>) -> impl IntoElement {
    let c = colors();
    let words = splat_encounters_ui::loot_editor::shown(line)
        .map(str::to_owned)
        .unwrap_or_else(|| line.to_owned());
    let link = splat_encounters_ui::loot_editor::link_target(line).is_some();
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .items_baseline()
        .gap(theme::XS)
        .child(if mark == "•" {
            kit::text(Type::Body, c.muted, "•")
        } else {
            kit::icon(mark, Type::Body, c.muted)
        })
        .child(kit::text(
            Type::Body,
            if link { c.accent } else { c.text },
            words,
        ))
        .children(source.map(|source| kit::text(Type::Small, c.muted, source)))
}

/// A holder's loot lines: plain lines, rolled lines with their roll, and a Roll for each line
/// still to roll (`loot_list`).
fn loot_list(run: &Run, holder: &Holder, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let sources = run.content.as_ref().map(|index| index.loot_sources());
    let mut column = div().flex().flex_col().gap(theme::XS);
    for (at, line) in holder.lines.iter().enumerate() {
        let Loot::Roll { times, key } = Loot::parse(line) else {
            column = column.child(loot_line("•", line, None));
            continue;
        };
        let plain = crate::encounter_run::loot::plain_roll_words(times, &key);
        match run.state.loot.get(&place(&holder.key, at)) {
            Some(found) if found.len() == 1 => {
                let source = format!(
                    "{key} · {} on {}",
                    crate::encounter_run::loot::found_values(found),
                    found[0].dice
                );
                column = column.child(loot_line(icons::DICE, &found[0].text, Some(source)));
            }
            Some(found) => {
                let dice = found.first().map_or("", |one| one.dice.as_str());
                column = column.child(kit::text(
                    Type::Small,
                    c.muted,
                    format!(
                        "{plain} · {} on {dice}",
                        crate::encounter_run::loot::found_values(found)
                    ),
                ));
                for one in found {
                    column = column.child(loot_line(icons::DICE, &one.text, None));
                }
            }
            None => {
                let problem = sources.as_ref().and_then(|sources| sources.problem(&key));
                let table = sources
                    .as_ref()
                    .and_then(|sources| sources.table(&key))
                    .filter(|_| problem.is_none())
                    .map(|table| table.dice.clone());
                let pick = (holder.clone(), at);
                column = column.child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(theme::XS)
                        .child(kit::icon(icons::DICE, Type::Body, c.muted))
                        .child(kit::text(Type::Body, c.text, plain.clone()))
                        .children(
                            table.map(|dice| kit::text(Type::Small, c.muted, format!("· {dice}"))),
                        )
                        .when(problem.is_some(), |line| {
                            line.child(kit::status(&kit::Status::new(
                                icons::WARN,
                                "Cannot roll",
                                c.result.warning,
                            )))
                        })
                        .child(div().w(theme::S).flex_none())
                        .child(
                            kit::Button::secondary(
                                ElementId::Name(
                                    format!("run-loot-roll-{}-{at}", holder.key).into(),
                                ),
                                "Roll",
                            )
                            .icon(icons::DICE)
                            .name(format!("Roll {plain} for {}", holder.name))
                            .density(Density::Read)
                            .on_click(on_run(cx, move |run| {
                                run.roll_loot(std::slice::from_ref(&pick))
                            })),
                        ),
                );
                if let Some(problem) = problem {
                    column = column.child(kit::text(Type::Small, c.muted, problem));
                }
            }
        }
    }
    column
}

/// The loot on a creature's card in the stat block tab (`card_loot`): a summary while it
/// stands, the list with its rolls once it is down.
pub(super) fn card_loot(
    run: &Run,
    index: usize,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let c = colors();
    let p = &run.state.participants[index];
    let lines = p.loot_lines();
    if lines.is_empty() {
        return None;
    }
    let holder = Holder {
        key: p.key.clone(),
        name: p.name.clone(),
        lines,
        open: p.hp == Some(0),
    };
    if !holder.open {
        let summary: Vec<String> = holder
            .lines
            .iter()
            .map(|line| splat_encounters_ui::loot_editor::plain(line))
            .collect();
        return Some(
            div()
                .flex()
                .flex_row()
                .gap(theme::XS)
                .pb(theme::S)
                .child(kit::icon(icons::LOOT, Type::Small, c.muted))
                .child(kit::text(
                    Type::Small,
                    c.muted,
                    format!("Loot · {}", summary.join(", ")),
                ))
                .into_any_element(),
        );
    }
    Some(
        div()
            .flex()
            .flex_col()
            .gap(theme::XS)
            .pb(theme::S)
            .child(kit::label("Loot", 0))
            .child(loot_list(run, &holder, cx))
            .into_any_element(),
    )
}

/// The ended view's loot (`loot_section`): each fallen creature's and the encounter's, `Roll
/// all`, `Copy loot`, and the creatures still standing in a section.
fn loot_section(run: &Run, cx: &mut Context<SplatRoot>) -> Option<AnyElement> {
    let c = colors();
    let holders = run.holders();
    if holders.is_empty() {
        return None;
    }
    let (open, standing): (Vec<Holder>, Vec<Holder>) =
        holders.into_iter().partition(|holder| holder.open);
    let unrolled: Vec<(Holder, usize)> = run
        .unrolled(&open)
        .into_iter()
        .map(|(holder, at)| (holder.clone(), at))
        .collect();
    let left = unrolled.len();
    let copied = run.loot_copied == Some(*run.revision);
    let copy_holders: Vec<Holder> = open
        .iter()
        .chain(standing.iter().filter(|holder| {
            (0..holder.lines.len()).any(|at| run.state.loot.contains_key(&place(&holder.key, at)))
        }))
        .cloned()
        .collect();
    let markdown = run.loot_markdown(&copy_holders);
    let mut heading = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::S)
        .child(kit::label("Loot", 0));
    if left > 0 {
        heading = heading.child(kit::text(Type::Small, c.muted, format!("{left} to roll")));
    }
    heading = heading.child(div().flex_1());
    if copied {
        heading = heading.child(kit::text(Type::Small, c.muted, "Loot copied."));
    }
    if left > 1 {
        heading = heading.child(
            kit::Button::secondary("run-loot-roll-all", format!("Roll all {left}"))
                .icon(icons::DICE)
                .density(Density::Read)
                .on_click(on_run(cx, move |run| run.roll_loot(&unrolled))),
        );
    }
    heading = heading.child(
        kit::Button::quiet("run-loot-copy", "Copy loot")
            .icon(icons::COPY)
            .density(Density::Read)
            .on_click(
                cx.listener(move |root: &mut SplatRoot, _: &ClickEvent, _, cx| {
                    cx.write_to_clipboard(gpui::ClipboardItem::new_string(markdown.clone()));
                    if let Some(run) = root.app.encounters.run_mut() {
                        run.loot_copied = Some(*run.revision);
                    }
                    cx.notify();
                }),
            ),
    );
    let mut column = div().flex().flex_col().gap(theme::XS).child(heading);
    for holder in &open {
        column = column
            .child(kit::text(Type::Strong, c.text, holder.name.clone()))
            .child(loot_list(run, holder, cx))
            .child(kit::gap(theme::S));
    }
    if !standing.is_empty() {
        let title = if standing.len() == 1 {
            "1 more creature carries loot".to_owned()
        } else {
            format!("{} more creatures carry loot", standing.len())
        };
        let mut inner = div().flex().flex_col().gap(theme::XS);
        for holder in &standing {
            inner = inner
                .child(kit::text(Type::Strong, c.text, holder.name.clone()))
                .child(loot_list(run, holder, cx))
                .child(kit::gap(theme::S));
        }
        column = column.child(kit::Section::new("run-loot-standing", title, false).child(inner));
    }
    Some(column.child(kit::gap(theme::M)).into_any_element())
}
