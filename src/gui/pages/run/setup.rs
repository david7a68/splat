//! The Setup phase: the title, Roll NPCs
//! and Begin combat with its reason, Add to fight and Undo; each participant's row with its
//! initiative, Roll, tie moves, place, Absent and menu; the message footer; Close run.
use super::{Side, on_command, on_run, panel, plain, run_title, save_status};
use crate::encounter_run::{Command, Panel, Participant, Run, view};
use crate::gui::{SplatRoot, icons, kit, theme};
use gpui::{
    AnyElement, Context, ElementId, InteractiveElement, IntoElement, ParentElement, SharedString,
    Styled, Window, div, prelude::*, px,
};
use splat_encounters_ui::encounter_settings::GroupingWords as _;
use theme::{Density, Type, colors};

/// The Setup phase.
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
    if let Some(run) = root.app.encounters.run_mut() {
        run.refresh_roster_faults();
        if std::mem::take(&mut run.focus_first_missing) && !run.lookup_open {
            run.focus_field = run
                .state
                .participants
                .iter()
                .position(|p| p.initiative.is_none() && p.present);
        }
    }
    let side = super::side_width(Side::Setup, room, window_width, cx);
    let arrivals = super::arrivals_prepare(root);
    let Some(run) = root.app.encounters.run() else {
        return div().into_any_element();
    };
    let width = if run.hide_panel {
        (room - theme::M).min(px(theme::RUN_LARGE.max_width))
    } else {
        room - side - theme::M
    };
    let main = div()
        .flex()
        .flex_col()
        .h_full()
        .w(width)
        .flex_none()
        .child(header(run, cx))
        .child(kit::rule())
        .children(roster_faults(run))
        .child(
            div()
                .flex_1()
                .min_h_0()
                .flex()
                .flex_col()
                .child(
                    panel::group_drop_zone(
                        div().flex().flex_col().flex_1().min_h_0(),
                        "run-setup-drop",
                        cx,
                    )
                    .child(kit::scroll_column("run-setup-rows", rows(run, cx))),
                )
                .children(message_footer(run, cx)),
        )
        .child(kit::rule())
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .pt(theme::XS)
                .h(px(super::live::FOOTER_HEIGHT))
                .flex_none()
                .child(super::footer::close_button(run, "run-close", false, cx))
                .child(div().flex_1())
                .child(save_status(run)),
        );
    let mut page = div().flex().flex_row().size_full().child(main);
    if !run.hide_panel {
        let panel = panel::render(root, run, side, arrivals, window, cx);
        page = page.child(div().w(theme::M).flex_none()).child(
            super::side_column(Side::Setup, room, window_width).render(room, panel, window, cx),
        );
    }
    page.into_any_element()
}

fn message_footer(run: &Run, cx: &mut Context<SplatRoot>) -> Option<AnyElement> {
    let messages = super::footer::messages(run, true, cx);
    (!messages.is_empty()).then(|| {
        div()
            .flex()
            .flex_col()
            .flex_none()
            .gap(theme::S)
            .py(theme::S)
            .children(messages)
            .into_any_element()
    })
}

/// The header: the title with the ties, Roll NPCs and Begin combat with its reason; the
/// ready line; then Add to fight and Undo.
fn header(run: &Run, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let blocked = run.setup_blocked();
    let default_roll = run.initiative.grouping;
    let tied = run.tied_count();
    let mut title = div()
        .flex()
        .flex_col()
        .flex_1()
        .min_w_0()
        .child(run_title(run, run.state_words(false)));
    if tied > 0 {
        title = title.child(kit::status(&kit::Status::new(
            icons::WARN,
            crate::design::count(tied, "tied", "tied"),
            c.result.warning,
        )));
    }
    let mut modes = kit::MenuButton::new(
        "run-roll-npcs-menu",
        kit::Button::secondary("run-roll-npcs-more", "")
            .icon(icons::DOWN)
            .name("Roll NPC initiative another way")
            .density(Density::Glance),
    )
    .heading("This roll only");
    for mode in run.roll_choices() {
        modes = modes.item(
            kit::MenuItem::new(mode.label())
                .checked(mode == default_roll)
                .on_click(on_command(cx, Command::RollNpcs(mode))),
        );
    }
    let roll = div()
        .flex()
        .flex_row()
        .gap(px(2.0))
        .child(
            kit::Button::secondary("run-roll-npcs", "Roll NPCs")
                .name("Roll NPC initiative")
                .icon(icons::DICE)
                .density(Density::Glance)
                .hint(format!("Campaign default: {}", default_roll.label()))
                .on_click(roll_npcs(cx, default_roll)),
        )
        .child(modes);
    let begin = div()
        .flex()
        .flex_col()
        .items_end()
        .child(
            kit::Button::primary("run-begin-combat", "Begin combat")
                .density(Density::Glance)
                .blocked(blocked.clone())
                .on_click(on_run(cx, |run| {
                    run.panel = Panel::Reference;
                    run.run_command(Command::Begin);
                })),
        )
        .children(
            blocked
                .clone()
                .map(|reason| kit::text(Type::Small, c.muted, reason)),
        );
    let first = div()
        .flex()
        .flex_row()
        .items_start()
        .gap(theme::S)
        .child(title)
        .child(roll)
        .child(begin);
    let ready = blocked.is_none().then(|| {
        let first = run
            .state
            .order
            .iter()
            .copied()
            .find(|i| run.state.participants[*i].present);
        let words = first.map_or_else(
            || "Everyone is ready".to_owned(),
            |i| {
                format!(
                    "Everyone is ready · {} goes first",
                    run.state.participants[i].name
                )
            },
        );
        div()
            .flex()
            .flex_row()
            .justify_end()
            .child(kit::status(&kit::Status::new(icons::OK, words, c.muted)))
    });
    let undo = if run.history.is_empty() {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(theme::S)
            .child(kit::text(
                Type::Small,
                c.muted,
                run.empty_undo_words().to_owned(),
            ))
            .child(super::undo_button(
                run,
                "run-undo",
                "Undo".into(),
                Density::Read,
            ))
    } else {
        div().child(
            super::undo_button(run, "run-undo", run.undo_label(), Density::Read).on_click(on_run(
                cx,
                |run| {
                    run.undo();
                    run.return_focus = run.lookup_open;
                },
            )),
        )
    };
    let add = kit::Button::secondary("run-add-to-fight", "Add to fight")
        .icon(icons::ADD)
        .name("Add to fight")
        .hint("Add an encounter group or a creature (Ctrl+G)")
        .density(Density::Read)
        .selected(run.panel == Panel::Groups && !run.hide_panel)
        .on_click(on_run(cx, |run| run.show_arrivals()));
    div()
        .flex()
        .flex_col()
        .flex_none()
        .gap(theme::XS)
        .pb(theme::XS)
        .child(first)
        .children(ready)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::S)
                .h(Density::Read.control())
                .child(add)
                .child(div().flex_1())
                .child(undo),
        )
}

/// Roll NPCs in a way: the step, then the keyboard goes to the first missing initiative.
fn roll_npcs(
    cx: &mut Context<SplatRoot>,
    mode: splat_encounters_ui::encounter_settings::Grouping,
) -> super::OnClick {
    on_run(cx, move |run| {
        run.run_command(Command::RollNpcs(mode));
        run.focus_field = run
            .state
            .participants
            .iter()
            .position(|p| p.initiative.is_none() && p.present);
        if run.focus_field.is_none() {
            run.return_focus = run.lookup_open;
        }
    })
}

/// Each unread roster row's fault, with the file's line (`roster_faults`).
fn roster_faults(run: &Run) -> Vec<AnyElement> {
    run.roster_faults
        .iter()
        .map(|fault| {
            kit::Message::new(kit::Tone::Error, fault.words())
                .small()
                .into_any_element()
        })
        .collect()
}

/// The rows, under their `INITIATIVE` label.
fn rows(run: &Run, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let mut column = div().flex().flex_col().w_full().child(
        div()
            .flex()
            .flex_row()
            .h(px(28.0))
            .items_end()
            .child(div().flex_1())
            .child(div().w(px(SLOTS_RIGHT)).flex_none().child(kit::text(
                Type::Label,
                c.muted,
                "INITIATIVE",
            ))),
    );
    for index in 0..run.state.participants.len() {
        column = column.child(row(run, index, cx));
    }
    column
}

/// The room from the initiative field's left: the field (`roster::INITIATIVE_FIELD`), tie,
/// place, Absent and the menu.
const SLOTS_RIGHT: f32 = 60.0 + 56.0 + 36.0 + 100.0 + 28.0 + 16.0;

/// One participant's row in setup.
fn row(run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let p = &run.state.participants[index];
    let selected = run.panel == Panel::Record && run.selected == index;
    let detail = detail_line(run, p);
    let name = p.name.clone();
    let row_name = format!(
        "{name} · {}",
        detail
            .iter()
            .map(|(words, _)| words.as_str())
            .collect::<Vec<_>>()
            .join(" · ")
    );
    let label = div()
        .id(ElementId::Name(format!("run-setup-row-{}", p.key).into()))
        .flex()
        .flex_col()
        .justify_center()
        .flex_1()
        .min_w_0()
        .px(theme::S)
        .py(theme::XS)
        .cursor_pointer()
        .child(kit::text::one_line(Type::Body, c.text, name.clone()))
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap_x(theme::XS)
                .children(
                    detail
                        .into_iter()
                        .map(|(words, color)| kit::text(Type::Small, color, words)),
                ),
        )
        .on_click(on_run(cx, move |run| {
            run.selected = index;
            run.return_focus = run.lookup_open;
        }));
    let label = kit::target(kit::target(label, row_name), name.clone());
    let roll = div().w(px(64.0)).flex_none().when(p.npc, |slot| {
        slot.child(
            kit::Button::quiet(
                ElementId::Name(format!("run-setup-roll-{}", p.key).into()),
                "Roll",
            )
            .plain()
            .icon(icons::DICE)
            .density(Density::Scan)
            .name(format!("Roll for {name}"))
            .blocked(
                p.initiative_bonus
                    .is_none()
                    .then_some("Set an initiative bonus, or type a total."),
            )
            .on_click(on_command(cx, Command::RollInitiative(index))),
        )
    });
    let initiative = initiative_field(run, index, cx);
    let tie = div()
        .w(px(56.0))
        .flex_none()
        .flex()
        .flex_row()
        .children(tie_controls(run, index, cx));
    let place = run
        .state
        .participants
        .iter()
        .all(|p| !p.present || p.initiative.is_some())
        && p.present;
    let place = place
        .then(|| {
            p.initiative
                .and_then(|_| run.state.order.iter().position(|i| *i == index))
        })
        .flatten()
        .map(|place| view::ordinal(place + 1))
        .unwrap_or_default();
    let (word, hint) = view::presence_action(p.present);
    let present = p.present;
    let absent = kit::Button::quiet(
        ElementId::Name(format!("run-setup-absent-{}", p.key).into()),
        word,
    )
    .icon(icons::ABSENT)
    .plain()
    .density(Density::Scan)
    .name(format!("{word}: {name}"))
    .hint(hint)
    .selected(!present)
    .on_click(on_command(cx, Command::Presence(index, !present)));
    let expanded = run.setup_expanded == Some(index);
    let menu = kit::MenuButton::new(
        ElementId::Name(format!("run-setup-menu-{}", p.key).into()),
        kit::Button::quiet(
            ElementId::Name(format!("run-setup-more-{}", p.key).into()),
            "",
        )
        .icon(icons::EXPAND)
        .name(format!("More actions for {name}"))
        .density(Density::Read),
    )
    .item(
        kit::MenuItem::new("Correct…").on_click(on_run(cx, move |run| {
            run.selected = index;
            run.panel = Panel::Record;
            run.close_lookup = true;
        })),
    )
    .item(
        kit::MenuItem::new(if expanded {
            "Hide NPC and roll mode"
        } else {
            "Change NPC and roll mode"
        })
        .on_click(on_run(cx, move |run| {
            run.setup_expanded = if expanded { None } else { Some(index) };
            run.return_focus = run.lookup_open;
        })),
    );
    let line = div()
        .flex()
        .flex_row()
        .items_center()
        .w_full()
        .when(selected, |line| line.bg(c.selected))
        .child(label)
        .child(roll)
        .child(initiative)
        .child(tie)
        .child(
            div()
                .w(px(36.0))
                .flex_none()
                .child(kit::text(Type::Small, c.muted, place)),
        )
        .child(div().w(px(100.0)).flex_none().child(absent))
        .child(menu);
    let mut block = div().flex().flex_col().w_full().child(line);
    if expanded {
        let npc = p.npc;
        let mode = p.initiative_mode;
        let modes = [-1_i8, 0, 1];
        block = block.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(theme::M)
                .pl(theme::S)
                .pb(theme::XS)
                .child(
                    kit::Checkbox::new(
                        ElementId::Name(format!("run-setup-npc-{}", p.key).into()),
                        "NPC",
                        npc,
                    )
                    .on_click(on_command(cx, Command::Npc(index, !npc))),
                )
                .child(
                    kit::Dropdown::new(
                        ElementId::Name(format!("run-setup-mode-{}", p.key).into()),
                        format!("Initiative roll for {name}"),
                        modes.map(|m| SharedString::from(crate::encounter_run::mode_label(m))),
                        modes.iter().position(|m| *m == mode),
                    )
                    .width(px(170.0))
                    .density(Density::Read)
                    .on_select(cx.listener(
                        move |root: &mut SplatRoot, at: &usize, _, cx| {
                            if let Some(run) = root.app.encounters.run_mut()
                                && let Some(picked) = modes.get(*at).copied()
                                && picked != mode
                            {
                                run.run_command(Command::RollMode(index, picked));
                            }
                            cx.notify();
                        },
                    )),
                ),
        );
    }
    block.child(kit::rule()).into_any_element()
}

/// The detail under a setup row's name: side, HP, temp HP, AC, HP state, conditions; a line
/// that could not be read and Absent first.
fn detail_line(run: &Run, p: &Participant) -> Vec<(String, gpui::Hsla)> {
    let c = colors();
    let display = |value: Option<i64>| value.map_or_else(|| "—".to_owned(), |n| n.to_string());
    let mut parts = Vec::new();
    if let Some(line) = run.roster_fault_line(&p.key) {
        parts.push((format!("Line {line} not read"), c.result.error));
    }
    if !p.present {
        parts.push(("Absent".to_owned(), c.muted));
    }
    let mut numbers = format!(
        "{} · HP {}",
        crate::design::reader::human_label(&p.side),
        splat_party::combat_sync::fraction(display(p.hp), display(p.max_hp))
    );
    if p.temporary_hp > 0 {
        numbers.push_str(&format!(" +{} temp", p.temporary_hp));
    }
    numbers.push_str(&format!(" · AC {}", display(p.armor_class)));
    parts.push((numbers, c.muted));
    if let Some((_, word, color)) = crate::design::hp_word(p.hp, p.max_hp) {
        parts.push((word.to_owned(), crate::gui::theme::color(color)));
    }
    if p.stat_changes() {
        parts.push(("Changed".to_owned(), c.participant.changed));
    }
    if p.npc_identity() {
        parts.push((format!("as {}", p.definition.name), c.muted));
    }
    for condition in &p.conditions {
        parts.push((condition.words(), c.muted));
    }
    parts
}

/// The initiative field: a typed total is a step; Enter goes on to the next missing one.
fn initiative_field(run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> AnyElement {
    let p = &run.state.participants[index];
    let typed = run.setup_inputs.get(&index).cloned();
    let value = match typed {
        Some(text) if crate::design::parse_signed(&text) == p.initiative => text,
        _ => p.initiative.map_or_else(String::new, |n| n.to_string()),
    };
    let focus = run.focus_field == Some(index);
    div()
        .key_context("RunField")
        .w(super::roster::INITIATIVE_FIELD)
        .flex_none()
        .child(
            kit::TextField::new(
                ElementId::Name(format!("run-setup-initiative-{}", p.key).into()),
                value,
            )
            .name(format!("Initiative for {}", p.name))
            .focus(focus)
            .on_change(
                cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                    if let Some(run) = root.app.encounters.run_mut() {
                        let text = super::number_text(words);
                        let parsed = crate::design::parse_signed(&text);
                        run.setup_inputs.insert(index, text);
                        run.focus_field = None;
                        if parsed != run.state.participants[index].initiative {
                            run.run_command(Command::Initiative(index, parsed));
                        }
                    }
                    cx.notify();
                }),
            )
            .on_enter(plain(cx, move |root, _, cx| {
                if let Some(run) = root.app.encounters.run_mut() {
                    let count = run.state.participants.len();
                    run.focus_field = (1..count).map(|step| (index + step) % count).find(|i| {
                        let p = &run.state.participants[*i];
                        p.initiative.is_none() && p.present
                    });
                    if run.focus_field.is_none() {
                        run.return_focus = run.lookup_open;
                    }
                }
                cx.notify();
            })),
        )
        .into_any_element()
}

/// Move a tied row before or after the one it ties with (`setup_tie_controls`).
fn tie_controls(run: &Run, index: usize, cx: &mut Context<SplatRoot>) -> Vec<AnyElement> {
    let Some((position, up, down)) = run.setup_tie_directions(index) else {
        return Vec::new();
    };
    let p = &run.state.participants[index];
    let mut controls = Vec::new();
    for (allowed, icon, destination, relation) in [
        (up, icons::MOVE_UP, position.saturating_sub(1), "before"),
        (down, icons::MOVE_DOWN, position + 1, "after"),
    ] {
        if allowed {
            let name = format!(
                "Move {} {relation} {}",
                p.name, run.state.participants[run.state.order[destination]].name
            );
            controls.push(
                kit::IconButton::new(
                    ElementId::Name(format!("run-tie-{relation}-{}", p.key).into()),
                    icon,
                    name,
                )
                .on_click(on_command(cx, Command::Move(position, destination)))
                .into_any_element(),
            );
        } else {
            controls.push(
                div()
                    .size(Density::Read.control())
                    .flex_none()
                    .into_any_element(),
            );
        }
    }
    controls
}
