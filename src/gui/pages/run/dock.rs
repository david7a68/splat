//! The dock under the live roster (egui `dock::dock`): the one home of the entry controls,
//! labelled with the row they act on, so no row moves when the selection changes. Line one:
//! Amount, Damage, Heal, Temp HP, the dice and Roll. Line two: the selected name, Conditions,
//! Conc, Absent and Effect; the effect field for that row while it is open; or how to give half
//! while several rows are targets.
use super::{number_text, on_run};
use crate::encounter_run::{Command, Phase, Run, view};
use crate::gui::{SplatRoot, icons, kit, theme};
use kit::WithTooltip as _;
use gpui::{
    Context, ElementId, InteractiveElement, IntoElement, KeyDownEvent, ParentElement, Styled,
    Window, div, prelude::*, px,
};
use theme::{Density, Type, TypeExt, colors};

/// Under this width the dock gives up the hint and an icon (egui `DOCK_ALL_WORDS_FROM`).
const ALL_WORDS_FROM: f32 = view::DOCK_ALL_WORDS_FROM;

/// The dock.
pub(super) fn render(run: &Run, width: gpui::Pixels, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let narrow = f32::from(width) < ALL_WORDS_FROM;
    let value = crate::design::parse_signed(&run.amount);
    let empty = value.is_none().then_some("Type an amount first.");
    let amount = div()
        .key_context("RunAmount")
        .w(px(if narrow { 68.0 } else { 84.0 }))
        .flex_none()
        .on_key_down(cx.listener(amount_keys))
        .child(
            kit::TextField::new(("run-amount", run.id().as_u128() as u64), run.amount.clone())
                .placeholder("Amount")
                .name("Amount")
                .focus(run.focus_amount)
                .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
                    if let Some(run) = root.app.encounters.run_mut() {
                        let digits = number_text(words);
                        if run.amount != digits {
                            run.amount = digits;
                            run.amount_typed = true;
                        }
                    }
                    cx.notify();
                })),
        );
    let damage = kit::Button::secondary("run-damage", "Damage")
        .density(Density::Glance)
        .hint("Apply damage. Temporary HP absorbs it first.")
        .blocked(empty)
        .on_click(on_run(cx, move |run| apply(run, Command::Damage)));
    let damage = if narrow { damage } else { damage.key("Enter") };
    let heal = kit::Button::quiet("run-heal", "Heal")
        .density(Density::Glance)
        .hint("Heal (Shift+Enter)")
        .blocked(empty)
        .on_click(on_run(cx, move |run| apply(run, Command::Heal)));
    let temp = kit::Button::quiet("run-temp-hp", "Temp HP")
        .icon(icons::TEMP_HP)
        .density(Density::Glance)
        .name("Replace temporary HP")
        .hint("Replace temporary HP with this amount")
        .blocked(empty)
        .on_click(on_run(cx, move |run| apply(run, Command::TempHp)));
    let first = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::XS)
        .child(amount)
        .child(damage)
        .child(div().w(if narrow { theme::XS } else { theme::M }).flex_none())
        .child(heal)
        .child(temp)
        .child(div().flex_1())
        .child(dice(run, narrow, cx));
    let second = if !run.also.is_empty() {
        div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(28.0))
            .child(kit::text::one_line(Type::Small, c.muted, crate::encounter_run::dock::target_hint()))
            .into_any_element()
    } else if run
        .effect_open
        .as_ref()
        .is_some_and(|key| *key == run.state.participants[run.selected].key)
    {
        effect_line(run, cx).into_any_element()
    } else {
        selected_line(run, width, cx).into_any_element()
    };
    div()
        .flex()
        .flex_col()
        .flex_none()
        .gap(theme::XS)
        .w_full()
        .px(theme::S)
        .py(theme::XS)
        .bg(c.selected)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(theme::XS)
                .max_w(theme::LIST_MEASURE)
                .child(first)
                .child(second),
        )
}

/// Damage, Heal or Temp HP by the amount, on the selected row and every other target, as one
/// step; then the amount clears and the keyboard goes back to it.
pub(super) fn apply(run: &mut Run, make: fn(usize, i64) -> Command) {
    let Some(amount) = crate::design::parse_signed(&run.amount) else {
        return;
    };
    let command = run.amount_command(amount, make);
    run.run_command(command);
    run.amount.clear();
    run.clear_targets();
    run.follow_with_keyboard();
}

/// Enter and Shift+Enter in Amount: damage and heal (Shift first, so a heal is never damage).
/// Beside Notes, a letter in Amount goes on to the note with the digits before it.
fn amount_keys(root: &mut SplatRoot, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<SplatRoot>) {
    let Some(run) = root.app.encounters.run_mut() else {
        return;
    };
    let keys = &event.keystroke;
    if keys.key == "enter" && !keys.modifiers.control && !keys.modifiers.alt {
        apply(run, if keys.modifiers.shift { Command::Heal } else { Command::Damage });
        cx.stop_propagation();
        cx.notify();
        return;
    }
    if run.notes_open
        && let Some(typed) = keys.key_char.as_ref().filter(|typed| {
            typed.chars().any(|c| c.is_alphabetic()) && !keys.modifiers.control
        })
    {
        run.for_note = Some(format!("{}{typed}", std::mem::take(&mut run.amount)));
        run.return_focus = true;
        cx.stop_propagation();
        cx.notify();
    }
}

/// The dice field, Roll and the last roll's total (`dock::dice`).
fn dice(run: &Run, narrow: bool, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let roll = move |root: &mut SplatRoot, cx: &mut Context<SplatRoot>| {
        if let Some(run) = root.app.encounters.run_mut() {
            let expression = if run.dice.trim().is_empty() {
                "1d20".to_owned()
            } else {
                run.dice.trim().to_owned()
            };
            match run.roll_dice(&expression) {
                Err(error) => run.error = Some(error),
                Ok(()) => run.note.clear(),
            }
            run.follow_with_keyboard();
        }
        cx.notify();
    };
    let result = run.dock_roll().map(|(total, line, _)| {
        div()
            .id("run-dice-result")
            .flex_none()
            .min_w(px(32.0))
            .ty(if narrow { Type::Strong } else { Type::Heading })
            .text_color(c.text)
            .whitespace_nowrap()
            .child(if narrow {
                format!("{total}")
            } else {
                format!("= {total}")
            })
            .with_tooltip(line)
    });
    let field = div()
        .key_context("RunField")
        .w(px(if narrow { 60.0 } else { 72.0 }))
        .flex_none()
        .child(
            kit::TextField::new(("run-dice", run.id().as_u128() as u64), run.dice.clone())
                .placeholder("1d20")
                .name("Dice")
                .on_change(cx.listener(|root: &mut SplatRoot, words: &str, _, cx| {
                    if let Some(run) = root.app.encounters.run_mut() {
                        run.dice = words.to_owned();
                    }
                    cx.notify();
                }))
                .on_enter(super::plain(cx, move |root, _, cx| roll(root, cx))),
        );
    let button = kit::Button::secondary("run-roll-dice", "Roll")
        .name("Roll dice")
        .density(Density::Read)
        .hint("Roll this expression. The result goes in the log.")
        .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| roll(root, cx)));
    let button = if narrow { button } else { button.icon(icons::DICE) };
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::XS)
        .flex_none()
        .child(field)
        .child(button)
        .children(result)
}

/// The selected name and its controls (line two).
fn selected_line(run: &Run, width: gpui::Pixels, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let c = colors();
    let index = run.selected;
    let p = &run.state.participants[index];
    let conditions_key = if run.lookup_open { "" } else { "C" };
    let (presence, hint) = view::presence_action(p.present);
    let name = div()
        .id("run-dock-name")
        .w(px(view::DOCK_NAME_WIDTH))
        .flex_none()
        .min_w_0()
        .ty(Type::Strong)
        .text_color(c.text)
        .line_clamp(2)
        .child(p.name.clone())
        .with_tooltip(p.name.clone());
    let name = kit::target(name, format!("Selected: {}", p.name));
    let conditions = kit::Button::quiet("run-dock-conditions", "Conditions")
        .density(Density::Read)
        .name("Conditions for this participant")
        .key(conditions_key)
        .on_click(on_run(cx, |run| run.open_conditions_tab()));
    let concentration = p.concentration;
    let conc = toggle("run-dock-conc", "Conc", icons::CONCENTRATING, concentration, "Concentrating")
        .hint("Mark concentration. It records your ruling only.")
        .on_click(super::on_command(cx, Command::Concentration(index, !concentration)));
    let present = p.present;
    let absent = toggle("run-dock-absent", presence, icons::ABSENT, !present, presence)
        .hint(hint)
        .on_click(super::on_command(cx, Command::Presence(index, !present)));
    let running = run.state.phase == Phase::Running;
    let effect_key = if run.lookup_open || run.notes_open {
        ""
    } else {
        "E"
    };
    // Effect takes the room left after Absent: its word, else its icon, else nothing.
    let wide = f32::from(width) >= ALL_WORDS_FROM;
    let effect = running.then(|| {
        if wide {
            kit::Button::quiet("run-dock-effect", "Effect")
                .icon(icons::PIN)
                .density(Density::Read)
                .name("Effect: words for this row")
                .hint("Pin words to this row, such as Bless 10 rounds")
                .key(effect_key)
                .on_click(on_run(cx, |run| {
                    run.effect_in_dock = true;
                    run.open_effect_field();
                }))
                .into_any_element()
        } else {
            kit::IconButton::new("run-dock-effect", icons::PIN, "Effect: words for this row")
                .on_click(on_run(cx, |run| {
                    run.effect_in_dock = true;
                    run.open_effect_field();
                }))
                .into_any_element()
        }
    });
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(theme::XS)
        .min_h(Density::Read.control())
        .child(name)
        .child(div().w(theme::M).flex_none())
        .child(conditions)
        .child(conc)
        .child(div().w(theme::S).flex_none())
        .child(absent)
        .when_some(effect, |line, effect| {
            line.child(div().w(theme::M).flex_none()).child(effect)
        })
}

/// A quiet two-state action: its icon and word, filled while on (`view::toggle`).
fn toggle(
    id: &'static str,
    words: &'static str,
    icon: &'static str,
    on: bool,
    name: &'static str,
) -> kit::Button {
    kit::Button::quiet(id, words)
        .icon(icon)
        .density(Density::Read)
        .name(name)
        .selected(on)
}

/// Line two as one field while the DM adds an effect to the selected row (`dock::effect_line`):
/// Enter adds the words, Escape clears them; either puts the keyboard back in Amount.
fn effect_line(run: &Run, cx: &mut Context<SplatRoot>) -> impl IntoElement {
    let index = run.selected;
    let p = &run.state.participants[index];
    let key = p.key.clone();
    let words = run
        .effect_fields
        .get(&key)
        .map(|field| field.words.clone())
        .unwrap_or_default();
    let focus = run.effect_fields.get(&key).is_some_and(|field| field.focus);
    let hint = format!("Effect on {}, such as Bless 10 rounds", p.name);
    let typing = key.clone();
    let entering = key.clone();
    let escaping = key.clone();
    div()
        .key_context("RunField")
        .flex()
        .flex_row()
        .items_center()
        .min_h(Density::Read.control())
        .on_key_down(cx.listener(move |root: &mut SplatRoot, event: &KeyDownEvent, _, cx| {
            if event.keystroke.key != "escape" {
                return;
            }
            if let Some(run) = root.app.encounters.run_mut() {
                if let Some(field) = run.effect_fields.get_mut(&escaping) {
                    field.clear();
                }
                run.effect_fields.retain(|_, field| !field.words.is_empty());
                run.effect_open = None;
                run.focus_amount = true;
            }
            cx.stop_propagation();
            cx.notify();
        }))
        .child(
            kit::TextField::new(ElementId::Name(format!("run-effect-{key}").into()), words)
                .placeholder(hint.clone())
                .name("Effect")
                .focus(focus)
                .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                    if let Some(run) = root.app.encounters.run_mut() {
                        let field = run.effect_fields.entry(typing.clone()).or_default();
                        field.words = words.to_owned();
                        field.focus = false;
                    }
                    cx.notify();
                }))
                .on_enter(super::plain(cx, move |root, _, cx| {
                    if let Some(run) = root.app.encounters.run_mut()
                        && let Some(index) =
                            run.state.participants.iter().position(|p| p.key == entering)
                    {
                        let words = run
                            .effect_fields
                            .get(&entering)
                            .map(|field| field.words.trim().to_owned())
                            .unwrap_or_default();
                        if words.is_empty() {
                            run.effect_fields.remove(&entering);
                        } else {
                            run.run_command(Command::Condition(index, words));
                            if run.error.is_none() {
                                run.effect_fields.remove(&entering);
                            }
                        }
                        run.effect_open = None;
                        run.focus_amount = true;
                    }
                    cx.notify();
                })),
        )
}
