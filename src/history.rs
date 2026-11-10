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

/// This frame's side-button presses, set aside from the raw input.
fn presses_id() -> egui::Id {
    egui::Id::new("history_button_presses")
}

/// Take the side buttons out of this frame's raw input, and keep their presses for
/// `pending`. Call it on every frame, so presses from an earlier frame are gone.
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
    ctx.data_mut(|data| data.insert_temp(presses_id(), presses));
}

/// The first Back or Forward request still waiting this frame.
pub fn pending(ctx: &egui::Context) -> Option<(Step, Source)> {
    let key = ctx.input(|input| {
        input.events.iter().find_map(|event| match event {
            egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } if modifiers.matches_logically(egui::Modifiers::ALT) => [Step::Back, Step::Forward]
                .into_iter()
                .find(|step| step.key() == *key),
            _ => None,
        })
    });
    key.map(|step| (step, Source::Key)).or_else(|| {
        ctx.data(|data| data.get_temp::<Vec<(Step, egui::Pos2)>>(presses_id()))?
            .first()
            .map(|&(step, pos)| (step, Source::Button(pos)))
    })
}

/// Take a request, so no other reader acts on it too. False if it was already taken.
pub fn take(ctx: &egui::Context, (step, source): (Step, Source)) -> bool {
    match source {
        Source::Key => ctx.input_mut(|input| input.consume_key(egui::Modifiers::ALT, step.key())),
        Source::Button(pos) => ctx.data_mut(|data| {
            let presses = data.get_temp_mut_or_default::<Vec<(Step, egui::Pos2)>>(presses_id());
            let at = presses.iter().position(|&press| press == (step, pos));
            at.map(|at| presses.remove(at)).is_some()
        }),
    }
}

/// Take the first request waiting, wherever it came from.
pub fn take_any(ctx: &egui::Context) -> Option<Step> {
    let request = pending(ctx)?;
    take(ctx, request).then_some(request.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run one frame with these events, the way eframe passes them on, and report what
    /// was still waiting before `act` and what `act` returned.
    fn frame<T>(
        events: Vec<egui::Event>,
        act: impl Fn(&egui::Context) -> T,
    ) -> (Option<(Step, Source)>, T) {
        let ctx = egui::Context::default();
        let mut raw = egui::RawInput {
            events,
            ..Default::default()
        };
        set_aside(&ctx, &mut raw.events);
        let mut out = None;
        let _ = ctx.run(raw, |ctx| {
            let waiting = pending(ctx);
            out = Some((waiting, act(ctx)));
        });
        out.unwrap()
    }

    fn button(button: egui::PointerButton, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: egui::pos2(10.0, 20.0),
            button,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn key(key: egui::Key, modifiers: egui::Modifiers) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        }
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

    fn twice(ctx: &egui::Context) -> (Option<Step>, Option<Step>) {
        (take_any(ctx), take_any(ctx))
    }

    #[test]
    fn side_buttons_go_back_and_forward_once_and_never_reach_egui() {
        let mut events = vec![
            button(egui::PointerButton::Extra1, true),
            button(egui::PointerButton::Extra1, false),
            button(egui::PointerButton::Primary, true),
        ];
        set_aside(&egui::Context::default(), &mut events);
        assert_eq!(events, [button(egui::PointerButton::Primary, true)]);
        let (waiting, taken) = frame(vec![button(egui::PointerButton::Extra1, true)], twice);
        assert_eq!(
            waiting,
            Some((Step::Back, Source::Button(egui::pos2(10.0, 20.0))))
        );
        assert_eq!(taken, (Some(Step::Back), None));
        let (_, taken) = frame(vec![button(egui::PointerButton::Extra2, true)], twice);
        assert_eq!(taken, (Some(Step::Forward), None));
    }

    #[test]
    fn a_release_or_another_button_is_not_a_step() {
        let (waiting, _) = frame(vec![button(egui::PointerButton::Extra1, false)], twice);
        assert_eq!(waiting, None);
        let (waiting, _) = frame(vec![button(egui::PointerButton::Middle, true)], twice);
        assert_eq!(waiting, None);
    }

    #[test]
    fn a_press_does_not_outlast_its_frame() {
        let ctx = egui::Context::default();
        set_aside(&ctx, &mut vec![button(egui::PointerButton::Extra1, true)]);
        set_aside(&ctx, &mut Vec::new());
        assert_eq!(pending(&ctx), None);
    }

    #[test]
    fn alt_arrows_go_back_and_forward_once() {
        let alt = egui::Modifiers::ALT;
        let (_, taken) = frame(vec![key(egui::Key::ArrowLeft, alt)], twice);
        assert_eq!(taken, (Some(Step::Back), None));
        let (_, taken) = frame(vec![key(egui::Key::ArrowRight, alt)], twice);
        assert_eq!(taken, (Some(Step::Forward), None));
        let (waiting, _) = frame(
            vec![key(egui::Key::ArrowRight, egui::Modifiers::NONE)],
            twice,
        );
        assert_eq!(waiting, None);
    }

    /// Ctrl+Alt+Left is not Alt+Left, so no reader may act on it.
    #[test]
    fn another_modifier_with_alt_is_not_a_step() {
        let ctrl_alt = egui::Modifiers::ALT | egui::Modifiers::CTRL;
        let (waiting, taken) = frame(vec![key(egui::Key::ArrowLeft, ctrl_alt)], twice);
        assert_eq!(waiting, None);
        assert_eq!(taken, (None, None));
    }
}
