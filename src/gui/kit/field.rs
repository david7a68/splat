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
use gpui_component::input::{
    Input, InputEvent, InputState, TextDecoration, Textarea, TextareaState,
};
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
    /// The words are Markdown source, styled where they are typed (`TextEditor::markdown_source`).
    markdown: bool,
    /// Where each section heading of the source ends (its rule goes under that line).
    rules: Rc<Vec<usize>>,
    _changes: Subscription,
}

/// A field whose words can carry decorations (a text editor's Markdown source); a one-line
/// field carries none.
trait Decorate {
    fn decorate(&mut self, decorations: Vec<TextDecoration>, cx: &mut Context<Self>)
    where
        Self: Sized;
}

impl Decorate for InputState {
    fn decorate(&mut self, _: Vec<TextDecoration>, _: &mut Context<Self>) {}
}

impl Decorate for TextareaState {
    fn decorate(&mut self, decorations: Vec<TextDecoration>, cx: &mut Context<Self>) {
        self.set_decorations(decorations, cx);
    }
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
                        this.restyle(&now, cx);
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
                    markdown: false,
                    rules: Rc::default(),
                    _changes: changes,
                }
            }

            /// Style the words again, when they are Markdown source: once per change of them.
            fn restyle(&mut self, words: &str, cx: &mut Context<Self>) {
                if !self.markdown {
                    return;
                }
                let style = super::markdown_source::style(words);
                self.rules = Rc::new(style.rules);
                self.input
                    .update(cx, |input, cx| input.decorate(style.decorations, cx));
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
                        .update(cx, |input, cx| input.set_value(value.clone(), window, cx));
                    self.restyle(&value, cx);
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
    text_type: Option<Type>,
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
            text_type: None,
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

    /// The words' type (`Body` by default): `Heading` for a name that heads its editor, with the
    /// field one `Scan` control high.
    pub fn text_type(mut self, ty: Type) -> Self {
        self.text_type = Some(ty);
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
        let (ty, height) = match self.text_type {
            Some(ty) if ty != Type::Body => (ty, Density::Scan.control()),
            _ => (Type::Body, Density::Read.control()),
        };
        let mut frame = frame(focused, height)
            .ty(ty)
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
    focus: bool,
    handle: Option<EditorHandle>,
    ty: Option<Type>,
    markdown: bool,
}

/// A text editor's state that the caller keeps (`TextEditor::handle`), so the caller can read and
/// place the caret and insert words: a note the program writes into at the caret. Keep one per
/// thing it edits, and drop it when that thing goes away.
#[derive(Clone)]
pub struct EditorHandle(Entity<FieldState<TextareaState>>, Entity<TextareaState>);

impl EditorHandle {
    pub fn new(placeholder: impl Into<SharedString>, window: &mut Window, cx: &mut App) -> Self {
        let placeholder = placeholder.into();
        let input = cx.new(|cx| TextareaState::new(window, cx).placeholder(placeholder));
        let state = cx.new(|cx| FieldState::<TextareaState>::create(input.clone(), window, cx));
        Self(state, input)
    }

    /// The caret, as a byte offset into the words. It can be read from the change handler.
    pub fn caret(&self, cx: &App) -> usize {
        self.1.read(cx).cursor()
    }

    /// Put the caret at byte offset `at` (clipped to a character boundary).
    pub fn set_caret(&self, at: usize, cx: &mut App) {
        self.1
            .update(cx, |input, cx| input.set_selected_range(at..at, cx));
    }

    /// Select the bytes `range` (clipped to character boundaries): a value the DM types over.
    pub fn select(&self, range: std::ops::Range<usize>, cx: &mut App) {
        self.1
            .update(cx, |input, cx| input.set_selected_range(range, cx));
    }

    /// The selected bytes (empty at the caret), for a test of where a page put the caret.
    #[cfg(test)]
    pub fn selection(&self, cx: &App) -> std::ops::Range<usize> {
        self.1.read(cx).selected_range()
    }

    /// Where the character at byte `at` was last drawn, in window coordinates (its line's
    /// height, no width): a place to open a menu at (the note's `/` menu). `None` while that
    /// line is not drawn.
    pub fn bounds_at(&self, at: usize, cx: &App) -> Option<gpui::Bounds<gpui::Pixels>> {
        self.1.read(cx).range_to_bounds(&(at..at))
    }

    /// Where the caret was last drawn, in window coordinates.
    pub fn caret_bounds(&self, cx: &App) -> Option<gpui::Bounds<gpui::Pixels>> {
        self.bounds_at(self.caret(cx), cx)
    }

    /// Whether it has the keyboard.
    pub fn focused(&self, window: &Window, cx: &App) -> bool {
        self.1.read(cx).focus_handle(cx).is_focused(window)
    }

    /// Give it the keyboard.
    pub fn focus(&self, window: &mut Window, cx: &mut App) {
        let handle = self.1.read(cx).focus_handle(cx);
        window.focus(&handle, cx);
    }

    /// The words it shows now.
    pub fn words(&self, cx: &App) -> SharedString {
        self.1.read(cx).value()
    }

    /// Show `words` now (the program changed them), before the editor draws again, so a caret
    /// placed next lands in the new words.
    pub fn show_words(&self, words: impl Into<SharedString>, window: &mut Window, cx: &mut App) {
        let words = words.into();
        self.0.update(cx, |state, cx| {
            let on_change = state.on_change.clone();
            state.follow(&words, on_change, window, cx);
        });
    }
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
            focus: false,
            handle: None,
            ty: None,
            markdown: false,
        }
    }

    /// The words are Markdown source (a note): styled where they are typed, as the Reader sets
    /// them, with a rule under each section heading (`kit::markdown_source`). Every character
    /// stays, markup included.
    pub fn markdown_source(mut self) -> Self {
        self.markdown = true;
        self
    }

    /// Use the caller's state (`EditorHandle`) in place of one kept under the id.
    pub fn handle(mut self, handle: &EditorHandle) -> Self {
        self.handle = Some(handle.clone());
        self
    }

    /// The words' type (default `Body`, or `Mono` with `mono`): `Prose` for a note's prose.
    pub fn text_type(mut self, ty: Type) -> Self {
        self.ty = Some(ty);
        self
    }

    /// Take the keyboard in this frame. Give it once, not every frame.
    pub fn focus(mut self, focus: bool) -> Self {
        self.focus = focus;
        self
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
        let state = match &self.handle {
            Some(handle) => handle.0.clone(),
            None => window.use_keyed_state(self.id.clone(), cx, |window, cx| {
                let input = cx.new(|cx| {
                    TextareaState::new(window, cx)
                        .placeholder(placeholder)
                        .rows(rows)
                });
                FieldState::<TextareaState>::create(input, window, cx)
            }),
        };
        let value = self.value.clone();
        let on_change = self.on_change.clone();
        let markdown = self.markdown;
        state.update(cx, |state, cx| {
            if markdown && !state.markdown {
                state.markdown = true;
                let words = state.input.read(cx).value();
                state.restyle(&words, cx);
            }
            state.follow(&value, on_change, window, cx)
        });
        let input = state.read(cx).input.clone();
        if self.focus {
            let handle = input.read(cx).focus_handle(cx);
            window.focus(&handle, cx);
        }
        let focused = input.read(cx).focus_handle(cx).is_focused(window);
        let ty = self
            .ty
            .unwrap_or(if self.mono { Type::Mono } else { Type::Body });
        let name = self
            .name
            .clone()
            .unwrap_or_else(|| self.placeholder.clone());
        let mut frame = frame(focused, ty.line() * self.rows as f32)
            .ty(ty)
            .text_color(colors().text)
            .items_start()
            .w_full()
            .child({
                let area = Textarea::new(&input)
                    .appearance(false)
                    .aria_label(name.clone())
                    .h_full();
                // gpui-component sets its own small size; a chosen type replaces it.
                match self.ty {
                    Some(ty) => area
                        .text_size(ty.size())
                        .line_height(ty.line())
                        .font_family(ty.family()),
                    None => area,
                }
            });
        if self.fill {
            frame = frame.h_full();
        }
        if self.markdown {
            frame = frame
                .relative()
                .child(section_rules(input, state.read(cx).rules.clone()));
        }
        let frame = target(frame, name);
        target(frame, element_name(&self.id))
    }
}

/// The rule under each section heading of a Markdown source, as the Reader draws it: a hairline
/// in the rule colour `RULE_GAP` under the heading's last line, across the text. Only headings in
/// view have a place (`range_to_bounds`), so a frame paints only those.
fn section_rules(input: Entity<TextareaState>, rules: Rc<Vec<usize>>) -> impl IntoElement {
    /// The gap between a heading's line and its rule in Markdown source.
    const RULE_GAP: gpui::Pixels = gpui::px(3.0);
    gpui::canvas(
        |_, _, _| {},
        move |bounds, _, window, cx| {
            let input = input.read(cx);
            let Some(text) = input.text_bounds() else {
                return;
            };
            let color = colors().rule;
            window.with_content_mask(Some(gpui::ContentMask { bounds }), |window| {
                for end in rules.iter() {
                    let Some(line) = input.range_to_bounds(&(*end..*end)) else {
                        continue;
                    };
                    let y = line.bottom() + RULE_GAP;
                    if y < bounds.top() || y > bounds.bottom() {
                        continue;
                    }
                    window.paint_quad(gpui::fill(
                        gpui::Bounds::new(
                            gpui::point(text.left(), y),
                            gpui::size(text.size.width, HAIRLINE_WIDTH),
                        ),
                        color,
                    ));
                }
            });
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
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
