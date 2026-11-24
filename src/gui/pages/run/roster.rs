//! The roster's rows (egui `view::roster`, `paint_row`, `RowLayout`): the initiative and the turn
//! mark, the side's shape, the picture, the name over its conditions, AC, temporary HP, the HP
//! readout and bar, and the HP word and sync state. A live row selects on click, adds a target
//! on Ctrl+click, opens its initiative on a click of the number, and offers its menu on a
//! right-click; the ended view's rows only show.
use super::on_run;
use splat_party::combat_sync::SyncState;
use crate::design::{hp_state, hp_word};
use crate::encounter_run::rolling::RowChoice;
use crate::encounter_run::{Command, Participant, Phase, Run};
use crate::gui::{SplatRoot, bridge, icons, kit, theme};
use crate::rolls::Mode;
use gpui::{
    AnyElement, ClickEvent, Context, ElementId, Hsla, InteractiveElement, IntoElement,
    MouseButton, ParentElement, SharedString, Styled, div, prelude::*, px,
};
use theme::{Density, RunScale, Type, TypeExt, colors};

/// A row's place in the turn order.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum TurnMark {
    Current,
    Next,
    None,
}

impl TurnMark {
    pub(super) fn of(index: usize, current: Option<usize>, next_up: Option<usize>) -> Self {
        if current == Some(index) {
            Self::Current
        } else if next_up == Some(index) {
            Self::Next
        } else {
            Self::None
        }
    }
}

/// How a row is used: live (it acts), or shown in the ended view's final state.
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Use {
    Live,
    Final,
}

/// The sync state's status (`view::sync_status`).
pub(super) fn sync_status(state: SyncState) -> kit::Status {
    super::status(&crate::encounter_run::view::sync_status(state))
}

/// The side's shape: a square for the party, a diamond for enemies, a circle for others. The
/// dock and tooltip give the word.
fn side_mark(side: &str, size: f32) -> AnyElement {
    let c = colors().side;
    let size = px(size);
    match side {
        "party" => div().size(size).flex_none().bg(c.party).into_any_element(),
        "enemies" => {
            let color = c.enemies;
            gpui::canvas(
                |_, _, _| {},
                move |bounds, _, window, _| {
                    let center = bounds.center();
                    let reach = bounds.size.width / 2.0 * 1.25;
                    let mut path = gpui::PathBuilder::fill();
                    path.move_to(gpui::point(center.x, center.y - reach));
                    path.line_to(gpui::point(center.x + reach, center.y));
                    path.line_to(gpui::point(center.x, center.y + reach));
                    path.line_to(gpui::point(center.x - reach, center.y));
                    path.close();
                    if let Ok(path) = path.build() {
                        window.paint_path(path, color);
                    }
                },
            )
            .size(size)
            .flex_none()
            .into_any_element()
        }
        _ => div().size(size).flex_none().rounded_full().bg(c.other).into_any_element(),
    }
}

/// The HP readout: current in the state's colour, `/max` muted, a bar under it.
pub(super) fn hp_block(p: &Participant, scale: RunScale) -> impl IntoElement {
    let c = colors();
    let state = hp_state(p.hp, p.max_hp);
    let current_color = kit::hp::current_color(state);
    let fill_color = kit::hp::hp_color(state);
    let current = p.hp.map_or_else(|| "?".to_owned(), |hp| hp.to_string());
    let max = p.max_hp.map_or_else(|| "?".to_owned(), |max| max.to_string());
    let fraction = match (p.hp, p.max_hp) {
        (Some(hp), Some(max)) if max > 0 => (hp as f32 / max as f32).clamp(0.0, 1.0),
        _ => 0.0,
    };
    let temp = match p.max_hp {
        Some(max) if max > 0 && p.temporary_hp > 0 => {
            Some((p.temporary_hp as f32 / max as f32).clamp(0.05, 1.0))
        }
        _ => None,
    };
    let bar_height = px((scale.row / 14.0).clamp(3.0, 5.0));
    let width = px(scale.hp * 2.6);
    let readout = div()
        .flex()
        .flex_row()
        .items_baseline()
        .justify_end()
        .child(
            div()
                .ty(Type::NumeralLarge)
                .text_size(px(scale.hp))
                .line_height(px(scale.hp * 1.1))
                .text_color(current_color)
                .child(current),
        )
        .child(
            div()
                .ty(Type::Numeral)
                .text_size(px(scale.hp_max))
                .text_color(c.muted)
                .child(format!("/{max}")),
        );
    let bar = div()
        .relative()
        .w_full()
        .h(bar_height)
        .rounded(px(1.0))
        .bg(c.hp.track)
        .child(
            div()
                .absolute()
                .top_0()
                .left_0()
                .h_full()
                .w(gpui::relative(fraction))
                .rounded(px(1.0))
                .bg(fill_color),
        )
        .children(temp.map(|share| {
            div()
                .absolute()
                .top(px(-2.0))
                .left_0()
                .h(px(2.0))
                .w(gpui::relative(share))
                .bg(c.hp.temp)
        }));
    div()
        .flex()
        .flex_col()
        .flex_none()
        .w(width)
        .gap(px(2.0))
        .child(readout)
        .child(bar)
}

/// A row's second line: Concentration, conditions and effects, the stat block an NPC uses,
/// changed stats and loot, in the condition colour.
fn second_line(run: &Run, index: usize, p: &Participant, scale: RunScale, live: bool, cx: &mut Context<SplatRoot>) -> Option<AnyElement> {
    let c = colors();
    let size = px(scale.condition);
    let mut parts: Vec<AnyElement> = Vec::new();
    let dot = || {
        div()
            .ty(Type::Body)
            .text_size(size)
            .text_color(c.muted)
            .child("·")
            .into_any_element()
    };
    if p.concentration {
        parts.push(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(2.0))
                .text_color(c.participant.concentrating)
                .child(kit::icon(icons::CONCENTRATING, Type::Body, c.participant.concentrating).text_size(size))
                .child(div().ty(Type::Strong).text_size(size).child("Conc"))
                .into_any_element(),
        );
        if let Some(dc) = run.conc_save_dc(index) {
            parts.push(dot());
            parts.push(
                div()
                    .ty(Type::Body)
                    .text_size(size)
                    .text_color(c.result.warning)
                    .child(format!("Conc save DC {dc}"))
                    .into_any_element(),
            );
        }
    }
    for (at, condition) in p.conditions.iter().enumerate() {
        if !parts.is_empty() {
            parts.push(dot());
        }
        let words = condition.words();
        let id = ElementId::Name(format!("run-row-{}-condition-{at}", p.key).into());
        let mut link = div()
            .id(id)
            .ty(Type::Body)
            .text_size(size)
            .text_color(c.participant.condition)
            .underline()
            .whitespace_nowrap()
            .child(words.clone());
        if live {
            link = link.cursor_pointer().on_click(on_run(cx, move |run| {
                run.select(index);
                run.open_conditions_tab();
            }));
        }
        parts.push(link.into_any_element());
    }
    if p.npc_identity() {
        if !parts.is_empty() {
            parts.push(dot());
        }
        parts.push(
            div()
                .ty(Type::Body)
                .text_size(size)
                .text_color(c.muted)
                .whitespace_nowrap()
                .child(format!("as {}", p.definition.name))
                .into_any_element(),
        );
    }
    if p.stat_changes() {
        if !parts.is_empty() {
            parts.push(dot());
        }
        parts.push(
            div()
                .flex()
                .flex_row()
                .gap(px(2.0))
                .text_color(c.participant.changed)
                .child(kit::icon(icons::CHANGED, Type::Body, c.participant.changed).text_size(size))
                .child(div().ty(Type::Body).text_size(size).child("Changed"))
                .into_any_element(),
        );
    }
    if p.loot_ready() {
        if !parts.is_empty() {
            parts.push(dot());
        }
        parts.push(
            div()
                .flex()
                .flex_row()
                .gap(px(2.0))
                .text_color(c.accent)
                .child(kit::icon(icons::LOOT, Type::Body, c.accent).text_size(size))
                .child(div().ty(Type::Body).text_size(size).child("Loot"))
                .into_any_element(),
        );
    }
    if let Some(badge) = run.row_badge(index) {
        parts.push(
            div()
                .ty(Type::Small)
                .text_color(c.accent)
                .whitespace_nowrap()
                .pl(theme::S)
                .child(badge)
                .into_any_element(),
        );
    }
    (!parts.is_empty()).then(|| {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_x(theme::XS)
            .min_w_0()
            .overflow_hidden()
            .whitespace_nowrap()
            .children(parts)
            .into_any_element()
    })
}

/// The accessible name of a row: every fact it shows, as `row_name` gives it.
fn row_name(p: &Participant, turn: TurnMark, sync: Option<SyncState>) -> String {
    let display = |value: Option<i64>| value.map_or_else(|| "—".to_owned(), |n| n.to_string());
    let mut parts = vec![
        p.name.clone(),
        p.side.clone(),
        format!("HP {}/{}", display(p.hp), display(p.max_hp)),
        format!("Temp {}", p.temporary_hp),
        format!("AC {}", display(p.armor_class)),
        format!("Initiative {}", display(p.initiative)),
    ];
    match turn {
        TurnMark::Current => parts.push("Current turn".into()),
        TurnMark::Next => parts.push("Next up".into()),
        TurnMark::None => {}
    }
    if !p.present {
        parts.push("Absent".into());
    }
    if let Some((_, word, _)) = hp_word(p.hp, p.max_hp) {
        parts.push(word.into());
    }
    if p.concentration {
        parts.push("Concentrating".into());
    }
    parts.extend(p.conditions.iter().map(crate::encounter_run::Condition::words));
    if p.loot_ready() {
        parts.push("Loot".into());
    }
    if let Some(sync) = sync {
        parts.push(sync_status(sync).word.to_string());
    }
    parts.join(", ")
}

/// One roster row.
#[allow(clippy::too_many_arguments)]
pub(super) fn row(
    root: &SplatRoot,
    run: &Run,
    index: usize,
    turn: TurnMark,
    scale: RunScale,
    use_: Use,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let p = &run.state.participants[index];
    let live = use_ == Use::Live;
    let current = turn == TurnMark::Current;
    let selected = live && (run.selected == index || run.also.contains(&index));
    let sync = run.row_sync_word(index);
    let text = if p.present { c.text } else { c.participant.absent };
    let height = px(scale.row);
    // The initiative, with the turn mark under it, or a field while the DM changes it.
    let initiative_words = p.initiative.map_or_else(|| "—".to_owned(), |n| n.to_string());
    let mark = match turn {
        TurnMark::Current => Some(("TURN", c.turn.current)),
        TurnMark::Next => Some(("NEXT", c.turn.next)),
        TurnMark::None => None,
    };
    let editing = live && run.live_initiative == Some(index);
    let initiative: AnyElement = if editing {
        let text = run.setup_inputs.get(&index).cloned().unwrap_or_default();
        let name = format!("Initiative for {}", p.name);
        let focus = run.focus_field == Some(index);
        div()
            .key_context("RunField")
            .w(px(52.0))
            .flex_none()
            .child(
                kit::TextField::new(ElementId::Name(format!("run-live-initiative-{}", p.key).into()), text)
                    .name(name)
                    .focus(focus)
                    .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                        if let Some(run) = root.app.encounters.run_mut() {
                            run.setup_inputs.insert(index, super::number_text(words));
                        }
                        cx.notify();
                    }))
                    .on_enter(super::plain(cx, move |root, _, cx| {
                        if let Some(run) = root.app.encounters.run_mut() {
                            let total = run.setup_inputs.get(&index).and_then(|text| crate::design::parse_signed(text));
                            if let Some(total) = total
                                && run.state.participants[index].initiative != Some(total)
                            {
                                run.run_command(Command::Initiative(index, Some(total)));
                            }
                            run.live_initiative = None;
                        }
                        cx.notify();
                    })),
            )
            .into_any_element()
    } else {
        let number = div()
            .id(ElementId::Name(format!("run-row-{}-initiative", p.key).into()))
            .flex()
            .flex_col()
            .items_end()
            .w(px(40.0))
            .flex_none()
            .child(
                div()
                    .ty(Type::Numeral)
                    .text_size(px(scale.stat))
                    .text_color(if current { c.turn.current } else { c.muted })
                    .child(initiative_words),
            )
            .children(mark.map(|(word, color)| {
                div()
                    .ty(Type::Label)
                    .text_size(px(scale.mark))
                    .text_color(color)
                    .child(word)
            }));
        if live && run.state.phase == Phase::Running && (!selected || run.also.is_empty()) {
            let key = p.key.clone();
            let number = number
                .cursor_pointer()
                .on_click(on_run(cx, move |run| {
                    let Some(index) = run.state.participants.iter().position(|p| p.key == key) else {
                        return;
                    };
                    run.live_initiative = Some(index);
                    run.focus_field = Some(index);
                    let total = run.state.participants[index].initiative;
                    run.setup_inputs
                        .insert(index, total.map_or_else(String::new, |n| n.to_string()));
                }));
            kit::target(
                kit::WithTooltip::with_tooltip(number, "Change initiative"),
                format!("Change initiative for {}", p.name),
            )
            .into_any_element()
        } else {
            number.into_any_element()
        }
    };
    let picture = px((scale.row * 0.5).clamp(22.0, 34.0));
    let art = run.row_art(root.bridge.ctx(), p);
    let name = div()
        .flex()
        .flex_col()
        .justify_center()
        .flex_1()
        .min_w_0()
        .child(
            div()
                .ty(Type::Name)
                .text_size(px(scale.name))
                .text_color(text)
                .truncate()
                .child(p.name.clone()),
        )
        .children(second_line(run, index, p, scale, live, cx));
    let display = |value: Option<i64>| value.map_or_else(|| "—".to_owned(), |n| n.to_string());
    let save = live.then(|| run.save_on_row(index)).flatten();
    let (value, label, strong) = match &save {
        Some((ability, total)) => (total.to_string(), ability.clone(), true),
        None => (display(p.armor_class), "AC".to_owned(), false),
    };
    let ac = div()
        .flex()
        .flex_row()
        .items_baseline()
        .justify_end()
        .gap(px(4.0))
        .w(px(scale.stat * 3.2))
        .flex_none()
        .child(
            div()
                .ty(Type::Small)
                .text_color(if current || strong { c.muted } else { c.faint })
                .child(label),
        )
        .child(
            div()
                .ty(Type::Numeral)
                .text_size(px(scale.stat))
                .text_color(if strong || !p.present { text } else { c.muted })
                .child(value),
        );
    let temp = div()
        .flex()
        .flex_row()
        .items_center()
        .justify_end()
        .gap(px(2.0))
        .w(px(scale.stat * 2.6))
        .flex_none()
        .when(p.temporary_hp > 0, |temp| {
            temp.child(kit::icon(icons::TEMP_HP, Type::Small, c.hp.temp)).child(
                div()
                    .ty(Type::Numeral)
                    .text_size(px(scale.stat))
                    .text_color(c.hp.temp)
                    .child(format!("+{}", p.temporary_hp)),
            )
        });
    // The HP word and sync state take only their own room, after the HP, which keeps its place.
    let mut words = div()
        .flex()
        .flex_col()
        .justify_center()
        .gap(px(2.0))
        .w(px(scale.mark.max(12.0) * 7.0))
        .flex_none();
    if let Some((icon, word, color)) = hp_word(p.hp, p.max_hp) {
        let color = if current && word == "HP unknown" {
            c.muted
        } else {
            bridge::color(color)
        };
        words = words.child(state_word(icon, word, color, scale));
    }
    if let Some(sync) = sync {
        let status = sync_status(sync);
        words = words.child(state_word_owned(status.icon.clone(), status.word.clone(), status.color, scale));
    }
    let mut row = div()
        .id(ElementId::Name(format!("run-row-{}", p.key).into()))
        .relative()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::S)
        .w_full()
        .h(height)
        .flex_none()
        .pr(theme::S)
        .border_b(theme::HAIRLINE_WIDTH)
        .border_color(c.hairline)
        .when(selected && !current, |row| row.bg(c.selected))
        .when(current, |row| {
            row.bg(c.current_row)
                .border(theme::FOCUS_WIDTH)
                .border_color(c.turn.current)
                .rounded(px(2.0))
        })
        .when(live && !selected && !current, |row| row.hover(|s| s.bg(c.hover)))
        .child(
            // The current turn's bar at the left edge.
            div()
                .w(px(6.0))
                .h_full()
                .flex_none()
                .when(current, |bar| bar.bg(c.turn.current)),
        )
        .child(initiative)
        .child(side_mark(&p.side, scale.name * 0.55))
        .child(super::portrait(art, &p.name, picture, !p.present))
        .child(name)
        .child(ac)
        .child(temp)
        .child(hp_block(p, scale))
        .child(words);
    if selected && !run.also.is_empty() {
        let half = run.halved.contains(&index);
        let key = p.key.clone();
        row = row.child(
            div()
                .id(ElementId::Name(format!("run-row-{}-half", p.key).into()))
                .absolute()
                .left(px(10.0))
                .top(height / 2.0 - px(12.0))
                .size(px(24.0))
                .flex()
                .items_center()
                .justify_center()
                .cursor_pointer()
                .ty(Type::Heading)
                .text_color(c.accent)
                .child(if half {
                    div().child("½")
                } else {
                    kit::icon(icons::OK, Type::Heading, c.accent)
                })
                .on_click(on_run(cx, move |run| {
                    let Some(index) = run.state.participants.iter().position(|p| p.key == key) else {
                        return;
                    };
                    if !run.halved.remove(&index) {
                        run.halved.insert(index);
                    }
                    run.focus_amount = true;
                }))
                .with_tooltip_words("This target saved, so it takes half, rounded down."),
        );
    }
    let name_words = row_name(p, turn, sync);
    if live {
        let key = p.key.clone();
        row = row
            .cursor_pointer()
            .on_click(cx.listener(move |root: &mut SplatRoot, event: &ClickEvent, _, cx| {
                let Some(run) = root.app.encounters.run_mut() else {
                    return;
                };
                let Some(index) = run.state.participants.iter().position(|p| p.key == key) else {
                    return;
                };
                let modifiers = event.modifiers();
                let ctrl = modifiers.secondary(cx);
                let mode = if ctrl {
                    Some(Mode::Disadvantage)
                } else if modifiers.shift {
                    Some(Mode::Advantage)
                } else {
                    None
                };
                if !run.target(index, mode) {
                    if ctrl {
                        run.toggle_target(index);
                    } else {
                        run.select(index);
                    }
                }
                cx.notify();
            }));
        let menu_id = ElementId::Name(format!("run-row-menu-{}", p.key).into());
        let sections = run.row_menu_sections(index);
        let asking = run.asking.as_ref().filter(|asking| asking.row == index).cloned();
        if !sections.is_empty() || asking.is_some() {
            let opener = menu_id.clone();
            row = row.on_mouse_down(MouseButton::Right, move |_, window, cx| {
                kit::menu::open_popup(opener.clone(), window, cx);
            });
        }
        let menu = row_menu(&menu_id, index, sections, asking, cx);
        let row = kit::target(kit::target(row, name_words), p.name.clone());
        return div()
            .flex()
            .flex_col()
            .w_full()
            .flex_none()
            .child(row)
            .children(menu)
            .into_any_element();
    }
    kit::target(row, name_words).into_any_element()
}

/// A state word beside the HP: `♡ Bloodied`, `0 HP`, a sync state.
fn state_word(icon: &'static str, word: &'static str, color: Hsla, scale: RunScale) -> impl IntoElement {
    state_word_owned(SharedString::new_static(icon), SharedString::new_static(word), color, scale)
}

fn state_word_owned(icon: SharedString, word: SharedString, color: Hsla, scale: RunScale) -> impl IntoElement {
    let size = px(scale.mark.max(12.0));
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(3.0))
        .text_color(color)
        .child(div().icon_font(Type::Small).text_size(size).child(icon))
        .child(div().ty(Type::Heading).text_size(size).whitespace_nowrap().child(word))
}

/// The row's menu: the armed action's rolls against it, a reroll with inspiration, or the
/// question that waits on it (`row_menu`). Open from a right-click, or by itself while it asks.
fn row_menu(
    id: &ElementId,
    index: usize,
    sections: Vec<(String, Vec<(String, RowChoice)>)>,
    asking: Option<crate::encounter_run::rolling::Asking>,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let c = colors();
    let open = kit::menu::is_open(id, cx) || asking.is_some();
    if !open {
        return None;
    }
    let mut content = div().flex().flex_col().min_w(px(240.0));
    let choice = |content: gpui::Div, at: String, words: String, choice: RowChoice, cx: &mut Context<SplatRoot>| {
        content.child(
            kit::Button::quiet(ElementId::Name(at.into()), words)
                .plain()
                .density(Density::Read)
                .full_width()
                .on_click(on_run(cx, move |run| run.row_choice(index, choice.clone()))),
        )
    };
    if let Some(asking) = asking {
        content = content.child(
            div()
                .px(theme::S)
                .py(theme::XS)
                .max_w(px(320.0))
                .child(kit::text(Type::Body, c.text, asking.question.clone())),
        );
        for (at, (words, _)) in asking.choices.iter().enumerate() {
            content = choice(content, format!("run-ask-{index}-{at}"), words.clone(), RowChoice::Damage(at), cx);
        }
        content = choice(content, format!("run-ask-{index}-cancel"), "Cancel".into(), RowChoice::Cancel, cx);
    } else {
        for (section, (heading, items)) in sections.into_iter().enumerate() {
            content = content.child(
                div()
                    .px(theme::S)
                    .py(theme::XS)
                    .child(kit::text(Type::Small, c.muted, heading)),
            );
            for (at, (words, row_choice)) in items.into_iter().enumerate() {
                content = choice(content, format!("run-menu-{index}-{section}-{at}"), words, row_choice, cx);
            }
        }
    }
    Some(kit::menu::layer_for(id, content).into_any_element())
}

/// Hover words on an element with an id.
trait TooltipWords: kit::WithTooltip {
    fn with_tooltip_words(self, words: impl Into<SharedString>) -> Self {
        self.with_tooltip(words)
    }
}
impl<T: kit::WithTooltip> TooltipWords for T {}
