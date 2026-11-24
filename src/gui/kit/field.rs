//! Text fields and the field block (`design::field`, `labelled_field`, `TextDraft` editors).
//!
//! - `TextField`: one line of text. `TextEditor`: several lines that wrap (a note, a description).
//!   Both wrap gpui-component's input and keep a two-way sync with the caller's `String`: give the
//!   current value each frame with `value`, and take the DM's edits in `on_change`. When the
//!   program changes the value (a load, a reset), the field shows the new words; while the DM
//!   types, the field keeps the caret and the words.
//! - `field`: a sentence-case muted `Small` caption, the control, help and an error, then `L`
//!   before the next thing, so help never reads as the next field's.
//! - `help`: `Small` muted words that wrap at `READER_MEASURE`.
//!
//! The field's GPUI state (its input entity, caret, Undo) is kept under its id, so the id says
//! whose text the field edits: give each object's field its own id (`("note", note_id)`), never a
//! place in the layout.
use super::{button::element_name, target};
use crate::gui::theme::{
    Density, FOCUS_WIDTH, HAIRLINE_WIDTH, L, RADIUS_CONTROL, READER_MEASURE, Type, TypeExt, XS,
    colors,
};
use gpui::Focusable as _;
use gpui::{
    AnyElement, App, Context, Div, ElementId, Entity, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Subscription, Window, div, prelude::*,
};
use gpui_component::input::{Input, InputEvent, InputState, Textarea, TextareaState};
use std::rc::Rc;

type OnChange = Rc<dyn Fn(&str, &mut Window, &mut App)>;

/// The kept state of one field: its input, the words it last agreed with the caller on, and the
/// caller's latest change handler.
pub struct FieldState<S: 'static> {
    pub input: Entity<S>,
    last: SharedString,
    on_change: Option<OnChange>,
    /// The caller's handler of focus leaving the field (`TextField::on_blur`).
    on_blur: Option<super::OnAct>,
    _changes: Subscription,
}

macro_rules! field_state {
    ($state:ty) => {
        impl FieldState<$state> {
            fn create(input: Entity<$state>, window: &mut Window, cx: &mut Context<Self>) -> Self {
                let changes = cx.subscribe_in(
                    &input,
                    window,
                    |this: &mut Self, input, event: &InputEvent, window, cx| {
                        if matches!(event, InputEvent::Blur) {
                            if let Some(on_blur) = this.on_blur.clone() {
                                on_blur(window, cx);
                            }
                            return;
                        }
                        if !matches!(event, InputEvent::Change) {
                            return;
                        }
                        let now = input.read(cx).value();
                        if now == this.last {
                            return;
                        }
                        this.last = now.clone();
                        if let Some(on_change) = this.on_change.clone() {
                            on_change(&now, window, cx);
                        }
                    },
                );
                Self {
                    input,
                    last: SharedString::default(),
                    on_change: None,
                    on_blur: None,
                    _changes: changes,
                }
            }

            /// Take the caller's value: words the program changed replace the field's.
            fn follow(
                &mut self,
                value: &SharedString,
                on_change: Option<OnChange>,
                window: &mut Window,
                cx: &mut Context<Self>,
            ) {
                self.on_change = on_change;
                if *value != self.last {
                    self.last = value.clone();
                    let value = value.clone();
                    self.input
                        .update(cx, |input, cx| input.set_value(value, window, cx));
                }
            }
        }
    };
}

field_state!(InputState);
field_state!(TextareaState);

/// One line of text.
///
/// ```ignore
/// kit::TextField::new(("campaign-name", campaign.id), &self.draft.name)
///     .placeholder("Campaign name")
///     .on_change(cx.listener(|root, words: &str, _, cx| root.draft.name = words.to_owned()))
/// ```
#[derive(IntoElement)]
pub struct TextField {
    id: ElementId,
    value: SharedString,
    placeholder: SharedString,
    name: Option<SharedString>,
    masked: bool,
    disabled: bool,
    width: Option<gpui::Pixels>,
    on_change: Option<OnChange>,
    on_enter: Option<super::OnAct>,
    on_escape: Option<super::OnAct>,
    on_blur: Option<super::OnAct>,
    focus: Option<FocusAt>,
}

impl TextField {
    pub fn new(id: impl Into<ElementId>, value: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            placeholder: SharedString::default(),
            name: None,
            masked: false,
            disabled: false,
            width: None,
            on_change: None,
            on_enter: None,
            on_escape: None,
            on_blur: None,
            focus: None,
        }
    }

    /// Faint words while the field is empty. Set it once: the field keeps its first placeholder.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// The accessible name, when no `field` caption names it.
    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Dots in place of the words (a password, a sign-in cookie).
    pub fn masked(mut self) -> Self {
        self.masked = true;
        self
    }

    /// Shown, not editable.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// A fixed width (default: its parent's).
    pub fn width(mut self, width: gpui::Pixels) -> Self {
        self.width = Some(width);
        self
    }

    /// The DM's edit: the field's whole words after it. Fits `cx.listener(|root, words: &str, ..|)`.
    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Enter in the field (a form's Create).
    pub fn on_enter(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_enter = Some(Rc::new(handler));
        self
    }

    /// Escape in the field (an amount that ends unused, a picker's Cancel). The press stops here:
    /// the window's Escape (close a panel) does not also run.
    pub fn on_escape(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_escape = Some(Rc::new(handler));
        self
    }

    /// Focus left the field (a length that the event takes when the DM moves on).
    pub fn on_blur(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_blur = Some(Rc::new(handler));
        self
    }

    /// Take the keyboard in this frame, the caret after the words (a field that opens for
    /// typing). Give `true` for one frame; the caller keeps the request.
    pub fn focus(mut self, focus: bool) -> Self {
        if focus {
            self.focus = Some(FocusAt::End);
        }
        self
    }

    /// Take the keyboard in this frame with all the words selected, so typing replaces them (a
    /// rename that a press opened, `design::focus_and_select`). Give `true` for one frame.
    pub fn focus_selected(mut self, focus: bool) -> Self {
        if focus {
            self.focus = Some(FocusAt::All);
        }
        self
    }
}

/// Where a field that takes the keyboard puts its caret.
#[derive(Clone, Copy)]
enum FocusAt {
    /// After the words.
    End,
    /// Around all the words.
    All,
}

impl RenderOnce for TextField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (placeholder, masked) = (self.placeholder.clone(), self.masked);
        let state = window.use_keyed_state(self.id.clone(), cx, |window, cx| {
            let input = cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder(placeholder)
                    .masked(masked)
            });
            FieldState::<InputState>::create(input, window, cx)
        });
        let value = self.value.clone();
        let on_change = self.on_change.clone();
        let on_blur = self.on_blur.clone();
        state.update(cx, |state, cx| {
            state.on_blur = on_blur;
            state.follow(&value, on_change, window, cx)
        });
        let input = state.read(cx).input.clone();
        input.update(cx, |input, cx| input.set_disabled(self.disabled, cx));
        match self.focus {
            Some(FocusAt::End) => input.update(cx, |input, cx| input.focus(window, cx)),
            Some(FocusAt::All) => {
                let handle = input.read(cx).focus_handle(cx);
                window.focus(&handle, cx);
                window.dispatch_action(Box::new(gpui_component::input::SelectAll), cx);
            }
            None => {}
        }
        let focused = input.read(cx).focus_handle(cx).is_focused(window);
        let name = self
            .name
            .clone()
            .unwrap_or_else(|| self.placeholder.clone());
        let mut element = Input::new(&input)
            .appearance(false)
            .aria_label(name.clone())
            .disabled(self.disabled);
        if self.masked {
            element = element.mask_toggle();
        }
        let enter = self.on_enter.clone();
        let mut frame = frame(focused, Density::Read.control())
            .ty(Type::Body)
            .text_color(colors().text)
            .child(element);
        frame = match self.width {
            Some(width) => frame.w(width),
            None => frame.w_full(),
        };
        if let Some(enter) = enter {
            frame = frame.on_key_down(move |event, window, cx| {
                if event.keystroke.key == "enter" && !event.keystroke.modifiers.modified() {
                    enter(window, cx);
                }
            });
        }
        if let Some(escape) = self.on_escape.clone() {
            frame = frame.on_key_down(move |event, window, cx| {
                if event.keystroke.key == "escape" && !event.keystroke.modifiers.modified() {
                    escape(window, cx);
                    cx.stop_propagation();
                }
            });
        }
        let frame = target(frame, name);
        target(frame, element_name(&self.id))
    }
}

/// Several lines that wrap: a note, a description, a template.
#[derive(IntoElement)]
pub struct TextEditor {
    id: ElementId,
    value: SharedString,
    placeholder: SharedString,
    name: Option<SharedString>,
    rows: usize,
    mono: bool,
    fill: bool,
    on_change: Option<OnChange>,
}

impl TextEditor {
    pub fn new(id: impl Into<ElementId>, value: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            placeholder: SharedString::default(),
            name: None,
            rows: 4,
            mono: false,
            fill: false,
            on_change: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Its height in rows when it does not fill its parent (default 4).
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows;
        self
    }

    /// Source text in `Type::Mono` (a Markdown source, a template).
    pub fn mono(mut self) -> Self {
        self.mono = true;
        self
    }

    /// As tall as its parent (a document editor).
    pub fn fill(mut self) -> Self {
        self.fill = true;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for TextEditor {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (placeholder, rows) = (self.placeholder.clone(), self.rows);
        let state = window.use_keyed_state(self.id.clone(), cx, |window, cx| {
            let input = cx.new(|cx| {
                TextareaState::new(window, cx)
                    .placeholder(placeholder)
                    .rows(rows)
            });
            FieldState::<TextareaState>::create(input, window, cx)
        });
        let value = self.value.clone();
        let on_change = self.on_change.clone();
        state.update(cx, |state, cx| state.follow(&value, on_change, window, cx));
        let input = state.read(cx).input.clone();
        let focused = input.read(cx).focus_handle(cx).is_focused(window);
        let ty = if self.mono { Type::Mono } else { Type::Body };
        let name = self
            .name
            .clone()
            .unwrap_or_else(|| self.placeholder.clone());
        let mut frame = frame(focused, ty.line() * self.rows as f32)
            .ty(ty)
            .text_color(colors().text)
            .items_start()
            .w_full()
            .child(
                Textarea::new(&input)
                    .appearance(false)
                    .aria_label(name.clone())
                    .h_full(),
            );
        if self.fill {
            frame = frame.h_full();
        }
        let frame = target(frame, name);
        target(frame, element_name(&self.id))
    }
}

/// The well and edge every input shows: the well colour, a 3:1 field edge, and the focus colour
/// at 2 px while it has the keyboard.
fn frame(focused: bool, height: gpui::Pixels) -> Div {
    let c = colors();
    div()
        .flex()
        .items_center()
        .min_h(height)
        .px(XS)
        .rounded(RADIUS_CONTROL)
        .bg(c.well)
        .border(if focused { FOCUS_WIDTH } else { HAIRLINE_WIDTH })
        .border_color(if focused { c.focus } else { c.field_border })
}

/// A form field: its caption, its control, its help and its error, then `L`.
pub fn field(
    caption: impl Into<SharedString>,
    help: Option<SharedString>,
    error: Option<SharedString>,
    control: impl IntoElement,
) -> Div {
    let c = colors();
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .pb(L)
        .child(super::text::text(Type::Small, c.muted, caption))
        .child(control)
        .when_some(help, |block, words| block.child(self::help(words)))
        .when_some(error, |block, words| {
            block.child(super::outcome::Message::new(super::outcome::Tone::Error, words).small())
        })
}

/// Help under a field or a section: `Small` muted words that wrap at `READER_MEASURE`.
pub fn help(words: impl Into<SharedString>) -> Div {
    super::text::text(Type::Small, colors().muted, words).max_w(READER_MEASURE)
}
