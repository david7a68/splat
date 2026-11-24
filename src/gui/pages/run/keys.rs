//! The run's keys (egui `view::live_keys`, `back_out_on_escape`, the Ctrl+G of `Run::show`).
//!
//! Single letters work while no field has the keyboard, and in Amount, which takes only
//! digits; beside Lookup or Notes a letter is the next word typed, never a command. Enter and
//! Shift+Enter in Amount are the dock's (`dock::amount_keys`). A key the run takes goes no
//! further, so a letter never lands in Amount.
use crate::encounter_run::{Command, Panel, Phase, Run};
use crate::gui::SplatRoot;
use gpui::{Context, KeyDownEvent, Modifiers, Window};

/// The run page's keys.
pub(super) fn key_down(
    root: &mut SplatRoot,
    event: &KeyDownEvent,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) {
    if root.modal_open(cx) {
        return;
    }
    let stack = window.context_stack();
    let in_field = stack.iter().any(|context| context.contains("RunField"));
    let in_amount = stack.iter().any(|context| context.contains("RunAmount"));
    let Some(run) = root.app.encounters.run_mut() else {
        return;
    };
    let keys = &event.keystroke;
    let ctrl = keys.modifiers.secondary(cx) && !keys.modifiers.shift && !keys.modifiers.alt;
    if take(run, &keys.key, keys.modifiers, ctrl, in_field, in_amount) {
        cx.stop_propagation();
        cx.notify();
    }
}

/// What the run does with a key; true when it took it.
fn take(
    run: &mut Run,
    key: &str,
    modifiers: Modifiers,
    ctrl: bool,
    in_field: bool,
    in_amount: bool,
) -> bool {
    let plain = !modifiers.modified();
    let ended = matches!(run.state.phase, Phase::Ended { .. });
    // Ctrl+G: Add to fight, in every phase but the end.
    if ctrl && key == "g" && !ended {
        run.panel = Panel::Groups;
        run.close_lookup = true;
        return true;
    }
    // Another field's keys are its own; beside a panel only Amount's are the run's.
    if in_field || (run.lookup_open && !in_amount) {
        return false;
    }
    // Escape backs out of a footer's second step, in every phase.
    if plain && key == "escape" && (run.closing.is_open() || run.ending) {
        run.closing.shut();
        run.ending = false;
        run.return_focus = run.lookup_open;
        return true;
    }
    if run.state.phase != Phase::Running {
        return false;
    }
    if plain && key == "escape" {
        // A row's initiative field closes, and nothing else.
        if run.live_initiative.is_some() {
            run.live_initiative = None;
            return true;
        }
        // A question waiting on a target closes first, then the armed action stops.
        if run.asking.is_some() {
            run.asking = None;
            return true;
        }
        if run.armed.is_some() {
            run.disarm();
            return true;
        }
        // Back to whose turn it is, after a look at someone else.
        if let Some(current) = run.current()
            && (run.selected != current || !run.also.is_empty())
        {
            run.select(current);
            run.panel = Panel::Reference;
            return true;
        }
        return false;
    }
    // Ctrl+1 to Ctrl+9 arm the panel's actions, as a game's action bar does.
    if ctrl
        && let Some(number) = key.parse::<usize>().ok().filter(|n| (1..=9).contains(n))
    {
        run.arm_by_number(number);
        return true;
    }
    if ctrl && key == "z" {
        run.undo();
        run.follow_with_keyboard();
        return true;
    }
    if !plain {
        return false;
    }
    let letters = !run.lookup_open;
    match key {
        "n" if letters => run.run_command(Command::Next),
        "u" if letters => {
            run.undo();
            run.follow_with_keyboard();
        }
        "up" => run.step_selection(-1),
        "down" => run.step_selection(1),
        "a" if letters => run.focus_amount = true,
        "c" if letters => run.open_conditions_tab(),
        "e" if letters => run.open_effect_field(),
        digit if digit.len() == 1 && digit.chars().all(|c| c.is_ascii_digit()) && !in_amount => {
            run.amount.push_str(digit);
            run.focus_amount = true;
        }
        _ => return false,
    }
    true
}
