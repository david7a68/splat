//! A pane footer's messages as the egui pages build them (`design::outcome::Messages`), drawn as
//! `kit::Message`s. A page builds its `Messages` with `PaneFrame::plain()` as egui does, takes
//! the lines with `into_plain()`, and gives them here with one press handler. A press carries
//! the line's id and, for a kept result, its store's ticket (`Press`), so it reaches only the
//! entry the line showed.
//!
//! ```ignore
//! let frame = PaneFrame::plain();
//! let messages = page.list_messages(&frame, …);
//! let lines = kit::messages::lines("timeline-footer", messages.into_plain(),
//!     cx.listener(|root, pressed: &kit::messages::Pressed<ListAct>, _, cx| {
//!         if let Some(press) = pressed.take() { …; cx.notify() }
//!     }));
//! kit::Pane::new("timeline-list").footer_all(lines)
//! ```
use super::{Button, IconButton, Link, Message, Tone};
use crate::design::outcome::Press;
use crate::design::outcome::plain::{ChoiceForm, PlainLine};
use crate::gui::icons;
use crate::gui::theme::{Density, Type, colors};
use gpui::{AnyElement, App, ElementId, IntoElement, ParentElement, SharedString, Styled, Window, div};
use std::cell::RefCell;
use std::rc::Rc;

/// The press a message's control made. `take` it once in the handler.
pub struct Pressed<A>(RefCell<Option<Press<A>>>);

impl<A> Pressed<A> {
    /// The press, the first time it is asked for.
    pub fn take(&self) -> Option<Press<A>> {
        self.0.borrow_mut().take()
    }
}

/// A handler of a message's press. Fits `cx.listener(|root, pressed: &Pressed<A>, window, cx| …)`.
pub type OnPress<A> = Rc<dyn Fn(&Pressed<A>, &mut Window, &mut App)>;

/// The kit's tone of a design tone.
pub fn tone(tone: crate::design::Tone) -> Tone {
    match tone {
        crate::design::Tone::Info => Tone::Info,
        crate::design::Tone::Success => Tone::Success,
        crate::design::Tone::Warning => Tone::Warning,
        crate::design::Tone::Error => Tone::Error,
    }
}

/// Every line, in the order given, each with its own id under `id`.
pub fn lines<A: Clone + 'static>(
    id: impl Into<ElementId>,
    lines: Vec<PlainLine<A>>,
    on_press: impl Fn(&Pressed<A>, &mut Window, &mut App) + 'static,
) -> Vec<AnyElement> {
    let id: ElementId = id.into();
    let on_press: OnPress<A> = Rc::new(on_press);
    lines
        .into_iter()
        .enumerate()
        .map(|(at, line)| {
            message(
                ElementId::NamedChild(
                    std::sync::Arc::new(id.clone()),
                    SharedString::from(format!("line-{at}")),
                ),
                line,
                on_press.clone(),
                false,
            )
            .into_any_element()
        })
        .collect()
}

/// One line as a message: its words, its file line and help, its Undo first, then its choices
/// (a choice that loses something last, `L` after the others). `small`: a row's own message.
pub fn message<A: Clone + 'static>(
    id: ElementId,
    line: PlainLine<A>,
    on_press: OnPress<A>,
    small: bool,
) -> Message {
    let c = colors();
    let child = |name: &str| {
        ElementId::NamedChild(std::sync::Arc::new(id.clone()), SharedString::from(name.to_owned()))
    };
    let line = Rc::new(line);
    let mut message = Message::new(tone(line.tone), line.words.clone());
    if small {
        message = message.small();
    }
    if let Some(file) = &line.file {
        let mut row = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(crate::gui::theme::S)
            .child(super::text::text(Type::Small, c.muted, file.shown.clone()));
        if let Some(path) = file.path.clone() {
            if file.edit {
                let url = crate::design::outcome::obsidian_url(&path);
                row = row.child(
                    Link::new(child("edit"), crate::design::outcome::EDIT_IN_OBSIDIAN)
                        .small()
                        .name(format!(
                            "{}: {}",
                            crate::design::outcome::EDIT_IN_OBSIDIAN,
                            file.shown
                        ))
                        .on_click(move |_, _, cx| cx.open_url(&url)),
                );
            }
            if file.show
                && let Some(url) = crate::design::outcome::file_url(&path)
            {
                row = row.child(
                    Link::new(child("show"), crate::design::outcome::SHOW_FILE)
                        .small()
                        .name(format!("{}: {}", crate::design::outcome::SHOW_FILE, file.shown))
                        .hint(path.display().to_string())
                        .on_click(move |_, _, cx| cx.open_url(&url)),
                );
            }
        }
        message = message.detail(row);
    }
    for help in &line.help {
        message = message.detail(super::field::help(help.clone()));
    }
    if let Some(undo) = &line.undo {
        let (line, on_press) = (line.clone(), on_press.clone());
        message = message.action(
            Button::quiet(child("undo"), undo.label.clone())
                .icon(icons::UNDO)
                .density(Density::Read)
                .blocked(undo.locked.clone())
                .on_click(move |_, window, cx| {
                    if let Some(press) = line.press_undo() {
                        on_press(&Pressed(RefCell::new(Some(press))), window, cx);
                    }
                }),
        );
    }
    for (at, choice) in line.choices.iter().enumerate() {
        let press = {
            let (line, on_press) = (line.clone(), on_press.clone());
            move |_: &gpui::ClickEvent, window: &mut Window, cx: &mut App| {
                if let Some(press) = line.press_choice(at) {
                    on_press(&Pressed(RefCell::new(Some(press))), window, cx);
                }
            }
        };
        let id = child(&format!("choice-{at}"));
        match choice.form {
            ChoiceForm::Link => {
                let mut link = Link::new(id, choice.words.clone()).on_click(press);
                if let Some(hover) = &choice.hover {
                    link = link.hint(hover.clone());
                }
                message = message.action(link);
            }
            ChoiceForm::Dismiss => {
                message = message.action(
                    IconButton::new(id, icons::CLOSE, choice.words.clone()).on_click(press),
                );
            }
            form => {
                let button = match form {
                    ChoiceForm::Primary => Button::primary(id, choice.words.clone()),
                    ChoiceForm::Lose => Button::destructive(id, choice.words.clone()),
                    _ => Button::quiet(id, choice.words.clone()),
                }
                .density(Density::Read)
                .blocked(choice.locked.clone())
                .on_click(press);
                let button = match &choice.hover {
                    Some(hover) if choice.locked.is_none() => button.hint(hover.clone()),
                    _ => button,
                };
                message = if form == ChoiceForm::Lose {
                    message.destructive(button)
                } else {
                    message.action(button)
                };
            }
        }
    }
    message
}
