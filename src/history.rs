//! Back and Forward through what a reader opened: Alt+Left and Alt+Right, or the mouse's
//! side buttons. Library and Lookup each keep their own trail, so each request goes to
//! one of them and is then taken out of the frame's input.
use eframe::egui;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Back,
    Forward,
}

/// Where a request came from. A key goes to the reader with focus; a button press goes
/// to the reader under the pointer.
#[derive(Clone, Copy, Debug)]
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
    fn button(self) -> egui::PointerButton {
        match self {
            Step::Back => egui::PointerButton::Extra1,
            Step::Forward => egui::PointerButton::Extra2,
        }
    }
}

/// The first Back or Forward request still in this frame's input.
pub fn pending(input: &egui::InputState) -> Option<(Step, Source)> {
    [Step::Back, Step::Forward].into_iter().find_map(|step| {
        if input.modifiers.alt && input.key_pressed(step.key()) {
            return Some((step, Source::Key));
        }
        input.events.iter().find_map(|event| match event {
            egui::Event::PointerButton {
                pos,
                button,
                pressed: true,
                ..
            } if *button == step.button() => Some((step, Source::Button(*pos))),
            _ => None,
        })
    })
}

/// Take a request out of the input, so no other reader acts on it too.
pub fn take(input: &mut egui::InputState, (step, source): (Step, Source)) {
    match source {
        Source::Key => {
            input.consume_key(egui::Modifiers::ALT, step.key());
        }
        Source::Button(_) => input.events.retain(|event| {
            !matches!(event, egui::Event::PointerButton { button, pressed: true, .. }
                if *button == step.button())
        }),
    }
}

/// Take the first request in the input, wherever it came from.
pub fn take_any(input: &mut egui::InputState) -> Option<Step> {
    let request = pending(input)?;
    take(input, request);
    Some(request.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(events: Vec<egui::Event>, alt: bool) -> egui::InputState {
        let modifiers = egui::Modifiers {
            alt,
            ..Default::default()
        };
        let raw = egui::RawInput {
            events,
            modifiers,
            ..Default::default()
        };
        egui::InputState::default().begin_pass(raw, false, 1.0, egui::InputOptions::default())
    }

    fn button(button: egui::PointerButton, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: egui::pos2(10.0, 20.0),
            button,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    fn key(key: egui::Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::ALT,
        }
    }

    #[test]
    fn side_buttons_go_back_and_forward_once() {
        let mut back = input(vec![button(egui::PointerButton::Extra1, true)], false);
        assert_eq!(take_any(&mut back), Some(Step::Back));
        assert_eq!(take_any(&mut back), None);
        let mut forward = input(vec![button(egui::PointerButton::Extra2, true)], false);
        assert!(matches!(
            pending(&forward),
            Some((Step::Forward, Source::Button(pos))) if pos == egui::pos2(10.0, 20.0)
        ));
        assert_eq!(take_any(&mut forward), Some(Step::Forward));
        assert_eq!(take_any(&mut forward), None);
    }

    #[test]
    fn a_release_or_another_button_is_not_a_step() {
        let mut released = input(vec![button(egui::PointerButton::Extra1, false)], false);
        assert_eq!(take_any(&mut released), None);
        let mut middle = input(vec![button(egui::PointerButton::Middle, true)], false);
        assert_eq!(take_any(&mut middle), None);
    }

    #[test]
    fn alt_arrows_go_back_and_forward_once() {
        let mut back = input(vec![key(egui::Key::ArrowLeft)], true);
        assert_eq!(take_any(&mut back), Some(Step::Back));
        assert_eq!(take_any(&mut back), None);
        let mut forward = input(vec![key(egui::Key::ArrowRight)], true);
        assert_eq!(take_any(&mut forward), Some(Step::Forward));
        assert_eq!(take_any(&mut forward), None);
        let mut plain = input(vec![key(egui::Key::ArrowRight)], false);
        assert_eq!(take_any(&mut plain), None);
    }
}
