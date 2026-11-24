//! A pane's message footer drawn from the egui page's own footer lines (`Messages::into_plain`):
//! the same words, order, choices, Undo and presses (`design::outcome::Messages`).
use crate::design::outcome::plain::Kind;
use crate::gui::{SplatRoot, kit, theme};
use crate::sessions::page::gpui_facade::{FooterChoice, FooterLine, FooterPress};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, SharedString, Styled, Window,
    div,
};
use std::{cell::RefCell, rc::Rc};
use theme::{S, Type, XS, colors};

/// What a press in this footer does: the page's own `session_press`, `draft_press`, …
pub(super) type OnPress = Rc<dyn Fn(&mut SplatRoot, FooterPress, &mut Window, &mut Context<SplatRoot>)>;

pub(super) fn tone(tone: crate::design::Tone) -> kit::Tone {
    match tone {
        crate::design::Tone::Info => kit::Tone::Info,
        crate::design::Tone::Success => kit::Tone::Success,
        crate::design::Tone::Warning => kit::Tone::Warning,
        crate::design::Tone::Error => kit::Tone::Error,
    }
}

/// Each line as a kit message, in the footer's order. `id` keeps the controls' ids apart from
/// another footer's.
pub(super) fn lines(
    id: &str,
    lines: Vec<FooterLine>,
    on_press: OnPress,
    cx: &mut Context<SplatRoot>,
) -> Vec<AnyElement> {
    lines
        .into_iter()
        .enumerate()
        .map(|(at, line)| self::line(&format!("{id}-{at}"), line, on_press.clone(), cx))
        .collect()
}

fn button(
    id: &str,
    choice: FooterChoice,
    on_press: OnPress,
    cx: &mut Context<SplatRoot>,
) -> (Kind, AnyElement) {
    let FooterChoice {
        words,
        kind,
        blocked,
        hover,
        press,
    } = choice;
    let slot = Rc::new(RefCell::new(Some(press)));
    let click = cx.listener(move |root, _, window, cx| {
        if let Some(press) = slot.borrow_mut().take() {
            on_press(root, press, window, cx);
            cx.notify();
        }
    });
    let element_id = ElementId::Name(SharedString::from(format!("{id}-{words}")));
    let element = match kind {
        Kind::Dismiss => kit::IconButton::new(element_id, crate::gui::icons::CLOSE, words)
            .on_click(click)
            .into_any_element(),
        _ => {
            let mut button = match kind {
                Kind::Primary => kit::Button::primary(element_id, words),
                Kind::Lose => kit::Button::destructive(element_id, words),
                Kind::Link => kit::Button::quiet(element_id, words).line_start(),
                _ => kit::Button::secondary(element_id, words),
            }
            .density(theme::Density::Read)
            .blocked(blocked);
            if let Some(hover) = hover {
                button = button.hint(hover);
            }
            button.on_click(click).into_any_element()
        }
    };
    (kind, element)
}

fn line(
    id: &str,
    line: FooterLine,
    on_press: OnPress,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let FooterLine {
        tone,
        words,
        file,
        help,
        undo,
        choices,
        ..
    } = line;
    let mut message = kit::Message::new(self::tone(tone), words);
    let mut dismiss = None;
    if let Some(undo) = undo {
        let (_, element) = button(&format!("{id}-undo"), undo, on_press.clone(), cx);
        message = message.action(element);
    }
    for choice in choices {
        let (kind, element) = button(id, choice, on_press.clone(), cx);
        match kind {
            Kind::Dismiss => dismiss = Some(element),
            Kind::Lose => message = message.destructive(element),
            _ => message = message.action(element),
        }
    }
    let mut column = div().flex().flex_col().flex_1().min_w_0().gap(XS).child(message);
    if let Some(file) = file {
        column = column.child(kit::text(Type::Small, c.muted, format!("File: {file}")).pl(S));
    }
    for words in help {
        column = column.child(kit::help(words).pl(S));
    }
    div()
        .flex()
        .flex_row()
        .items_start()
        .w_full()
        .gap(S)
        .child(column)
        .children(dismiss)
        .into_any_element()
}
