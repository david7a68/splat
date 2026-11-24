//! A pane's footer lines from the egui app's typed footer (`design::outcome::Messages`, read
//! out as `design::outcome::plain::PlainLine`s): each line is a `kit::Message` with its Undo
//! (quiet, first), its choices, and a destructive choice `L` after them. A press gives back the
//! line's `Press`, with its store's ticket, exactly as the egui footer does; the page hands it to
//! the same state method (`Campaigns::footer_act`, …).
//!
//! ```ignore
//! let frame = PaneFrame::headless();
//! let mut messages = Messages::new(&frame, "settings_footer", 0.0);
//! root.app.campaigns.footer_messages(&frame, &mut messages, true, blocked);
//! let lines = kit::footer::lines("settings-footer", messages.into_plain(),
//!     cx.listener(|root, press, _, cx| { root.app.campaigns.footer_act(press); cx.notify() }));
//! ```
use super::{Button, IconButton, Message, Tone};
use crate::design::outcome::Press;
use crate::design::outcome::plain::{PlainKind, PlainLine};
use crate::gui::icons;
use gpui::{AnyElement, App, ElementId, IntoElement, SharedString, Window, div, prelude::*};
use std::cell::RefCell;
use std::rc::Rc;

/// What a footer gives back: the press of one of its controls.
pub type OnPress<A> = Rc<dyn Fn(Press<A>, &mut Window, &mut App)>;

/// The kit's tone of a design tone.
pub fn tone(tone: crate::design::Tone) -> Tone {
    match tone {
        crate::design::Tone::Info => Tone::Info,
        crate::design::Tone::Success => Tone::Success,
        crate::design::Tone::Warning => Tone::Warning,
        crate::design::Tone::Error => Tone::Error,
    }
}

/// Each line as a message, in the footer's order. `id` keeps the controls' ids apart from other
/// footers'. A footer with no lines is empty.
pub fn lines<A: 'static>(
    id: &str,
    lines: Vec<PlainLine<A>>,
    on_press: impl Fn(Press<A>, &mut Window, &mut App) + 'static,
) -> Vec<AnyElement> {
    let on_press: OnPress<A> = Rc::new(on_press);
    lines
        .into_iter()
        .enumerate()
        .map(|(at, line)| {
            let mut message = Message::new(tone(line.tone), line.words);
            let mut dismiss = None;
            let controls = line.undo.into_iter().chain(line.choices);
            for (index, choice) in controls.enumerate() {
                let element_id =
                    ElementId::Name(SharedString::from(format!("{id}-{at}-{index}")));
                let press = Rc::new(RefCell::new(Some(choice.press)));
                let handler = {
                    let on_press = on_press.clone();
                    move |_: &gpui::ClickEvent, window: &mut Window, cx: &mut App| {
                        if let Some(press) = press.borrow_mut().take() {
                            on_press(press, window, cx);
                        }
                    }
                };
                match choice.kind {
                    PlainKind::Dismiss => {
                        dismiss = Some(
                            IconButton::new(element_id, icons::CLOSE, choice.label)
                                .on_click(handler)
                                .into_any_element(),
                        );
                    }
                    kind => {
                        let button = match kind {
                            PlainKind::Undo => {
                                Button::quiet(element_id, choice.label).icon(icons::UNDO)
                            }
                            PlainKind::Primary => Button::primary(element_id, choice.label),
                            PlainKind::Lose => Button::destructive(element_id, choice.label),
                            PlainKind::Link => Button::quiet(element_id, choice.label).plain(),
                            PlainKind::Choice | PlainKind::Dismiss => {
                                Button::secondary(element_id, choice.label)
                            }
                        }
                        .blocked(choice.locked)
                        .on_click(handler);
                        message = if kind == PlainKind::Lose {
                            message.destructive(button)
                        } else {
                            message.action(button)
                        };
                    }
                }
            }
            let message = message.into_any_element();
            let help = line.help;
            div()
                .flex()
                .flex_col()
                .w_full()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_start()
                        .child(div().flex_1().min_w_0().child(message))
                        .children(dismiss),
                )
                .children(help.into_iter().map(super::help))
                .into_any_element()
        })
        .collect()
}
