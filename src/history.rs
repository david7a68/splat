//! Back and Forward through what a reader opened: Alt+Left and Alt+Right, or the mouse's
//! side buttons. Library and Lookup each keep their own trail, so each request goes to
//! one of them and is then taken, so the other does not act on it too.
//!
//! The side buttons never reach egui: `set_aside` takes them out of the raw input, or a
//! press would also move a text caret, close a popup or take focus where the pointer is.
use eframe::egui;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Back,
    Forward,
}

/// Where a request came from. A key goes to the reader with focus; a button press goes
/// to the reader under the pointer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Source {
    Key,
    Button(egui::Pos2),
}

impl Step {
    fn of_key(key: egui::Key) -> Option<Self> {
        match key {
            egui::Key::ArrowLeft => Some(Step::Back),
            egui::Key::ArrowRight => Some(Step::Forward),
            _ => None,
        }
    }

    fn key(self) -> egui::Key {
        match self {
            Step::Back => egui::Key::ArrowLeft,
            Step::Forward => egui::Key::ArrowRight,
        }
    }

    /// Egui names the mouse's Back button Extra1 and its Forward button Extra2.
    fn of_button(button: egui::PointerButton) -> Option<Self> {
        match button {
            egui::PointerButton::Extra1 => Some(Step::Back),
            egui::PointerButton::Extra2 => Some(Step::Forward),
            _ => None,
        }
    }
}

/// One reader's history: the places Back and Forward go to, oldest first. The open entry
/// is not in it. It sits between `steps[at - 1]`, where Back goes, and `steps[at]`, where
/// Forward goes. A step swaps the open entry with the one it goes to.
#[derive(Debug)]
pub struct Trail<T> {
    steps: Vec<T>,
    at: usize,
}

impl<T> Default for Trail<T> {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            at: 0,
        }
    }
}

impl<T> Trail<T> {
    /// Leave the open entry for a new one: Back returns to it, and nothing is ahead.
    pub fn leave(&mut self, here: Option<T>) {
        self.steps.truncate(self.at);
        self.steps.extend(here);
        self.at = self.steps.len();
    }

    /// Where a step would go.
    pub fn peek(&self, step: Step) -> Option<&T> {
        match step {
            Step::Back => self.steps.get(self.at.checked_sub(1)?),
            Step::Forward => self.steps.get(self.at),
        }
    }

    /// Take a step, if `find` finds where it goes. The open entry, `here`, takes its place.
    /// A place that is gone, or a step with no open entry to leave, drops that place.
    pub fn go<R>(
        &mut self,
        step: Step,
        here: Option<T>,
        find: impl FnOnce(&T) -> Option<R>,
    ) -> Option<(T, R)> {
        let slot = match step {
            Step::Back => self.at.checked_sub(1)?,
            Step::Forward => self.at,
        };
        let found = find(self.steps.get(slot)?);
        let (target, kept) = match here.filter(|_| found.is_some()) {
            Some(here) => (std::mem::replace(&mut self.steps[slot], here), true),
            None => (self.steps.remove(slot), false),
        };
        self.at = slot + usize::from(kept && step == Step::Forward);
        Some((target, found?))
    }
}

/// The side-button presses set aside for one pass, and that pass's number.
type Presses = (u64, Vec<(Step, egui::Pos2)>);

fn presses_id() -> egui::Id {
    egui::Id::new("history_button_presses")
}

/// Take the side buttons out of this frame's raw input, and keep their presses for
/// `take_if` in the coming pass only.
pub fn set_aside(ctx: &egui::Context, events: &mut Vec<egui::Event>) {
    let mut presses = Vec::new();
    events.retain(|event| match event {
        egui::Event::PointerButton {
            pos,
            button,
            pressed,
            ..
        } => match Step::of_button(*button) {
            Some(step) => {
                if *pressed {
                    presses.push((step, *pos));
                }
                false
            }
            None => true,
        },
        _ => true,
    });
    let pass = ctx.cumulative_pass_nr();
    ctx.data_mut(|data| {
        if presses.is_empty() {
            data.remove::<Presses>(presses_id());
        } else {
            data.insert_temp::<Presses>(presses_id(), (pass, presses));
        }
    });
}

/// Take the first Back or Forward request waiting in this pass, if `accept` wants it,
/// so no other reader acts on it too.
pub fn take_if(ctx: &egui::Context, accept: impl FnOnce(Source) -> bool) -> Option<Step> {
    let key = ctx.input(|input| {
        input.events.iter().find_map(|event| match event {
            egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } if modifiers.matches_logically(egui::Modifiers::ALT) => Step::of_key(*key),
            _ => None,
        })
    });
    if let Some(step) = key {
        let taken = accept(Source::Key)
            && ctx.input_mut(|input| input.consume_key(egui::Modifiers::ALT, step.key()));
        return taken.then_some(step);
    }
    let pass = ctx.cumulative_pass_nr();
    let (step, pos) = ctx
        .data(|data| data.get_temp::<Presses>(presses_id()))
        .filter(|(set_in, _)| *set_in == pass)?
        .1
        .first()
        .copied()?;
    accept(Source::Button(pos)).then(|| {
        ctx.data_mut(|data| {
            data.get_temp_mut_or_default::<Presses>(presses_id())
                .1
                .remove(0)
        });
        step
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Run one pass with these events, as eframe passes them on.
    pub(crate) fn frame<T>(
        events: Vec<egui::Event>,
        mut act: impl FnMut(&egui::Context) -> T,
    ) -> T {
        let ctx = egui::Context::default();
        let mut raw = egui::RawInput {
            events,
            ..Default::default()
        };
        set_aside(&ctx, &mut raw.events);
        let mut out = None;
        let _ = ctx.run(raw, |ctx| out = Some(act(ctx)));
        out.unwrap()
    }

    pub(crate) fn press(button: egui::PointerButton, pos: egui::Pos2) -> egui::Event {
        egui::Event::PointerButton {
            pos,
            button,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        }
    }

    const AT: egui::Pos2 = egui::pos2(10.0, 20.0);

    fn key(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        }
    }

    fn twice(ctx: &egui::Context) -> (Option<Step>, Option<Step>) {
        (take_if(ctx, |_| true), take_if(ctx, |_| true))
    }

    /// Go with `here` open, where every place but "gone" is found.
    fn go(trail: &mut Trail<&'static str>, step: Step, here: &'static str) -> Option<&'static str> {
        trail
            .go(step, Some(here), |place| (*place != "gone").then_some(()))
            .map(|(place, ())| place)
    }

    #[test]
    fn a_step_swaps_the_open_entry_with_where_it_goes() {
        let mut trail = Trail::default();
        trail.leave(Some("a"));
        trail.leave(Some("b"));
        assert_eq!(go(&mut trail, Step::Back, "c"), Some("b"));
        assert_eq!(go(&mut trail, Step::Back, "b"), Some("a"));
        assert_eq!(go(&mut trail, Step::Back, "a"), None);
        assert_eq!(go(&mut trail, Step::Forward, "a"), Some("b"));
        assert_eq!(go(&mut trail, Step::Forward, "b"), Some("c"));
        assert_eq!(go(&mut trail, Step::Forward, "c"), None);
        assert_eq!(trail.peek(Step::Back), Some(&"b"));
    }

    #[test]
    fn leaving_for_a_new_entry_drops_what_was_ahead() {
        let mut trail = Trail::default();
        trail.leave(Some("a"));
        trail.leave(Some("b"));
        go(&mut trail, Step::Back, "c");
        trail.leave(Some("b"));
        assert_eq!(trail.peek(Step::Forward), None);
        assert_eq!(go(&mut trail, Step::Back, "d"), Some("b"));
        assert_eq!(go(&mut trail, Step::Back, "b"), Some("a"));
    }

    #[test]
    fn a_place_that_is_gone_is_dropped() {
        let mut trail = Trail::default();
        trail.leave(Some("a"));
        trail.leave(Some("gone"));
        assert_eq!(go(&mut trail, Step::Back, "b"), None);
        assert_eq!(go(&mut trail, Step::Back, "b"), Some("a"));
        assert_eq!(trail.peek(Step::Forward), Some(&"b"));
        go(&mut trail, Step::Forward, "a");
        assert_eq!(trail.peek(Step::Back), Some(&"a"));
        assert_eq!(trail.peek(Step::Forward), None);
        // With nothing open, a step drops the place it went to; nothing is left to return to.
        let found = trail.go(Step::Back, None, |_| Some(()));
        assert_eq!(found.map(|(place, ())| place), Some("a"));
        assert_eq!(
            (trail.peek(Step::Back), trail.peek(Step::Forward)),
            (None, None)
        );
    }

    #[test]
    fn side_buttons_go_back_and_forward_once_and_never_reach_egui() {
        let mut events = vec![
            press(egui::PointerButton::Extra1, AT),
            egui::Event::PointerButton {
                pos: AT,
                button: egui::PointerButton::Extra1,
                pressed: false,
                modifiers: egui::Modifiers::NONE,
            },
            press(egui::PointerButton::Primary, AT),
        ];
        set_aside(&egui::Context::default(), &mut events);
        assert_eq!(events, [press(egui::PointerButton::Primary, AT)]);
        let taken = frame(vec![press(egui::PointerButton::Extra1, AT)], twice);
        assert_eq!(taken, (Some(Step::Back), None));
        let taken = frame(vec![press(egui::PointerButton::Extra2, AT)], twice);
        assert_eq!(taken, (Some(Step::Forward), None));
        let taken = frame(vec![press(egui::PointerButton::Middle, AT)], twice);
        assert_eq!(taken, (None, None));
    }

    #[test]
    fn a_refused_request_waits_for_another_reader() {
        let taken = frame(vec![press(egui::PointerButton::Extra1, AT)], |ctx| {
            let refused = take_if(ctx, |source| {
                assert_eq!(source, Source::Button(AT));
                false
            });
            (refused, take_if(ctx, |_| true))
        });
        assert_eq!(taken, (None, Some(Step::Back)));
    }

    #[test]
    fn a_press_does_not_outlast_its_pass() {
        let ctx = egui::Context::default();
        let mut events = vec![press(egui::PointerButton::Extra1, AT)];
        set_aside(&ctx, &mut events);
        let _ = ctx.run(egui::RawInput::default(), |_| {});
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            assert_eq!(take_if(ctx, |_| true), None);
        });
    }

    #[test]
    fn alt_arrows_go_back_and_forward_once() {
        let alt = egui::Modifiers::ALT;
        let taken = frame(vec![key(egui::Key::ArrowLeft, alt)], twice);
        assert_eq!(taken, (Some(Step::Back), None));
        let taken = frame(vec![key(egui::Key::ArrowRight, alt)], twice);
        assert_eq!(taken, (Some(Step::Forward), None));
        let taken = frame(
            vec![key(egui::Key::ArrowRight, egui::Modifiers::NONE)],
            twice,
        );
        assert_eq!(taken, (None, None));
    }

    /// Ctrl+Alt+Left is not Alt+Left, so no reader may act on it.
    #[test]
    fn another_modifier_with_alt_is_not_a_step() {
        let ctrl_alt = egui::Modifiers::ALT | egui::Modifiers::CTRL;
        let taken = frame(vec![key(egui::Key::ArrowLeft, ctrl_alt)], twice);
        assert_eq!(taken, (None, None));
    }
}
