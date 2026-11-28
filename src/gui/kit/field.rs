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
    Density, FOCUS_WIDTH, HAIRLINE_WIDTH, L, RADIUS_CONTROL, READER_MEASURE, S, Type, TypeExt, XS,
    colors,
};
use gpui::Focusable as _;
use gpui::{
    AnyElement, App, Context, Div, ElementId, Entity, IntoElement, ParentElement, RenderOnce,
    SharedString, Styled, Subscription, Window, div, prelude::*,
};
use gpui_component::input::{
    InlineToken, InlineTokenError, Input, InputContent, InputEvent, InputState, TextDecoration,
    Textarea, TextareaState,
};
use std::ops::Range;
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
    /// The `splat:` marks of the source that fold off the caret's line (`kit::markdown_source`).
    folds: Vec<Range<usize>>,
    /// The marks folded now, once the input took them; `None` after a change of the words.
    shut: Option<Vec<Range<usize>>>,
    /// The field has taken the caller's words once (`follow`).
    shown: bool,
    _changes: Subscription,
    /// The caret and the focus move: a mark on the caret's line opens, the others fold.
    _moves: Subscription,
}

/// A field whose words can carry decorations and folded marks (a text editor's Markdown
/// source); a one-line field carries none.
trait Decorate {
    fn decorate(&mut self, decorations: Vec<TextDecoration>, cx: &mut Context<Self>)
    where
        Self: Sized;
    /// Fold the tokens of `content`: its text is the field's, and the history stays.
    fn fold(
        &mut self,
        content: InputContent,
        cx: &mut Context<Self>,
    ) -> Result<(), InlineTokenError>
    where
        Self: Sized;
}

impl Decorate for InputState {
    fn decorate(&mut self, _: Vec<TextDecoration>, _: &mut Context<Self>) {}
    fn fold(&mut self, _: InputContent, _: &mut Context<Self>) -> Result<(), InlineTokenError> {
        Ok(())
    }
}

impl Decorate for TextareaState {
    fn decorate(&mut self, decorations: Vec<TextDecoration>, cx: &mut Context<Self>) {
        self.set_decorations(decorations, cx);
    }
    fn fold(
        &mut self,
        content: InputContent,
        cx: &mut Context<Self>,
    ) -> Result<(), InlineTokenError> {
        self.set_tokens(content, cx)
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
                        if matches!(event, InputEvent::Focus | InputEvent::Blur) {
                            this.refold(window, cx);
                        }
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
                        this.restyle(&now, window, cx);
                        if let Some(on_change) = this.on_change.clone() {
                            on_change(&now, window, cx);
                        }
                    },
                );
                let moves = cx.observe_in(&input, window, |this: &mut Self, _, window, cx| {
                    this.refold(window, cx)
                });
                Self {
                    input,
                    last: SharedString::default(),
                    on_change: None,
                    on_blur: None,
                    markdown: false,
                    rules: Rc::default(),
                    folds: Vec::new(),
                    shut: None,
                    shown: false,
                    _changes: changes,
                    _moves: moves,
                }
            }

            /// Style the words again, when they are Markdown source: once per change of them.
            fn restyle(&mut self, words: &str, window: &Window, cx: &mut Context<Self>) {
                if !self.markdown {
                    return;
                }
                let style = super::markdown_source::style(words);
                self.rules = Rc::new(style.rules);
                self.folds = style.folds;
                self.shut = None;
                self.input
                    .update(cx, |input, cx| input.decorate(style.decorations, cx));
                self.refold(window, cx);
            }

            /// Fold each `splat:` mark off the lines that hold the caret or the selection, and
            /// open the marks on them; while the field does not have the keyboard, all fold
            /// (egui's layouter). Only a move to another line, a change of focus or of the words
            /// changes what folds; any other notice of the input costs a compare.
            fn refold(&mut self, window: &Window, cx: &mut Context<Self>) {
                if !self.markdown
                    || (self.folds.is_empty() && self.shut.as_ref().is_some_and(Vec::is_empty))
                {
                    return;
                }
                let input = self.input.read(cx);
                // The input changed words the field has not heard of yet: their change event
                // folds them.
                if input.text().len() != self.last.len() {
                    return;
                }
                let open = input.focus_handle(cx).is_focused(window).then(|| {
                    super::markdown_source::caret_lines(&self.last, input.selected_range())
                });
                let shut = super::markdown_source::shut(&self.folds, open.as_ref());
                if self.shut.as_ref() == Some(&shut) {
                    return;
                }
                let mut content = InputContent::new(self.last.clone());
                for range in &shut {
                    let token = InlineToken::new(
                        super::markdown_source::FOLD_TOKEN,
                        self.last[range.clone()].to_owned(),
                    )
                    .with_label(super::markdown_source::FOLD_MARK);
                    content = match content.clone().with_token(range.clone(), token) {
                        Ok(content) => content,
                        Err(_) => content,
                    };
                }
                let folded = self.input.update(cx, |input, cx| input.fold(content, cx));
                self.shut = folded.is_ok().then_some(shut);
            }

            /// Take the caller's value: words the program changed replace the field's.
            ///
            /// `own`: the change is the field's own edit (a command typed in it took its words
            /// out, words went in at its caret, the page added words to the open note), so the
            /// field's Undo history stays and Ctrl+Z goes back through the change. Any other
            /// change (a load, a reload, another object's words) ends the history, so another
            /// object's words never come back through Ctrl+Z. A field that has shown no words yet
            /// takes its first words with no history either way.
            fn follow(
                &mut self,
                value: &SharedString,
                on_change: Option<OnChange>,
                own: bool,
                window: &mut Window,
                cx: &mut Context<Self>,
            ) {
                self.on_change = on_change;
                if *value != self.last {
                    let keep = own && self.shown;
                    self.last = value.clone();
                    self.shown = true;
                    let value = value.clone();
                    self.input.update(cx, |input, cx| {
                        if keep {
                            input.replace_all(value.clone(), window, cx);
                        } else {
                            input.set_value(value.clone(), window, cx);
                        }
                    });
                    self.restyle(&value, window, cx);
                }
                self.shown = true;
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
    search: bool,
    tight: bool,
    trailing: Option<gpui::AnyElement>,
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
            search: false,
            tight: false,
            trailing: None,
        }
    }

    /// A search field (egui's `search_box`): the magnifier in muted at its left, then the words.
    pub fn search(mut self) -> Self {
        self.search = true;
        self
    }

    /// Only the frame's own padding round the words, as egui's number fields had: a narrow
    /// field (an amount, dice, an initiative total) shows its whole placeholder and number.
    pub fn tight(mut self) -> Self {
        self.tight = true;
        self
    }

    /// A control inside the field's frame, after the words: an editable combo's chevron
    /// (egui's `Dropdown::editable_value` with its `combo_icon`).
    pub fn trailing(mut self, trailing: impl IntoElement) -> Self {
        self.trailing = Some(trailing.into_any_element());
        self
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
            state.follow(&value, on_change, false, window, cx)
        });
        let input = state.read(cx).input.clone();
        input.update(cx, |input, cx| input.set_disabled(self.disabled, cx));
        match self.focus {
            Some(FocusAt::End) => input.update(cx, |input, cx| input.focus(window, cx)),
            Some(FocusAt::All) => {
                input.update(cx, |input, cx| input.focus(window, cx));
                window.dispatch_action(Box::new(gpui_component::input::SelectAll), cx);
            }
            None => {}
        }
        let focused = input.read(cx).focus_handle(cx).is_focused(window);
        let name = self
            .name
            .clone()
            .unwrap_or_else(|| self.placeholder.clone());
        let (ty, height) = match self.text_type {
            Some(ty) if ty != Type::Body => (ty, Density::Scan.control()),
            _ => (Type::Body, Density::Read.control()),
        };
        // gpui-component sets a smaller size of its own; the field's words and placeholder are
        // the field's type, as egui's were.
        let mut element = Input::new(&input)
            .appearance(false)
            .aria_label(name.clone())
            .disabled(self.disabled)
            .text_size(ty.size());
        if self.tight {
            element = element.px_0();
        }
        if self.masked {
            element = element.mask_toggle();
        }
        let enter = self.on_enter.clone();
        let mut frame = frame(focused, height)
            .ty(ty)
            .text_color(colors().text)
            .when(self.search, |frame| {
                frame.child(
                    div()
                        .flex_none()
                        .pl(S - XS)
                        .icon_font(ty)
                        .text_color(colors().muted)
                        .child(SharedString::new_static(crate::gui::icons::SEARCH)),
                )
            });
        frame = match self.trailing {
            Some(trailing) => frame
                .child(div().flex_1().min_w_0().child(element))
                .child(trailing),
            None => frame.child(element),
        };
        // For tests: the words' own room, inside the frame's padding and the input's (its default
        // `Medium` size), under `<name> words` (a narrow field's placeholder must fit it whole).
        #[cfg(test)]
        {
            let inset = XS
                + if self.tight {
                    gpui::px(0.0)
                } else {
                    gpui_component::Size::Medium.input_px()
                };
            frame = frame.relative().child(target(
                div().absolute().top_0().bottom_0().left(inset).right(inset),
                format!("{name} words"),
            ));
        }
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
    read_only: bool,
    own_write: bool,
    links: Option<Links>,
}

/// What Ctrl+click on a link in a text editor does (`TextEditor::follow_links`).
#[derive(Clone)]
struct Links {
    /// The note being edited, for a link relative to it.
    from: Option<uuid::Uuid>,
    /// The caller's own way to open a note link; else the Reader's (Lookup opens it).
    on_note: Option<OnChange>,
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

    /// The byte ranges folded to a mark now (`kit::markdown_source`), for a test.
    #[cfg(test)]
    pub fn folded(&self, cx: &App) -> Vec<Range<usize>> {
        self.1
            .read(cx)
            .tokens()
            .iter()
            .map(|span| span.range())
            .collect()
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

    /// Give it the keyboard, its caret drawn at once. (`window.focus` alone does not start
    /// gpui-base's caret, so a field the program focused showed none until a key or a press.)
    pub fn focus(&self, window: &mut Window, cx: &mut App) {
        self.1.update(cx, |input, cx| input.focus(window, cx));
    }

    /// The words it shows now.
    pub fn words(&self, cx: &App) -> SharedString {
        self.1.read(cx).value()
    }

    /// Show `words` now, before the editor draws again, so a caret placed next lands in the new
    /// words. They are the field's own edit (a command's result, words put in at the caret, the
    /// page's words added to the open note): the DM's Undo history stays under them, and Ctrl+Z
    /// takes the change back. For words from elsewhere (a reload, another note), use
    /// `show_other_words`.
    pub fn show_words(&self, words: impl Into<SharedString>, window: &mut Window, cx: &mut App) {
        self.show(words.into(), true, window, cx);
    }

    /// Show words that did not come from the field's own edit (a reload of an outside edit, a
    /// discard): its Undo history ends there.
    pub fn show_other_words(
        &self,
        words: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.show(words.into(), false, window, cx);
    }

    fn show(&self, words: SharedString, own: bool, window: &mut Window, cx: &mut App) {
        self.0.update(cx, |state, cx| {
            let on_change = state.on_change.clone();
            state.follow(&words, on_change, own, window, cx);
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
            read_only: false,
            own_write: false,
            links: None,
        }
    }

    /// With Ctrl held (Cmd on a Mac), a press on a link in the words follows it, as in the
    /// Reader; a plain press only places the caret (egui's `pressed_note_link`). A web link opens
    /// the browser, a dice link asks for its roll, a note link opens in Lookup, and a link that
    /// names nothing in the campaign does nothing. `from` is the note being edited.
    pub fn follow_links(mut self, from: Option<uuid::Uuid>) -> Self {
        let on_note = self.links.take().and_then(|links| links.on_note);
        self.links = Some(Links { from, on_note });
        self
    }

    /// As `follow_links`, with the caller's own way to open a note link (a session note opens an
    /// event in the Timeline). It gets the note the link names.
    pub fn on_note_link(mut self, handler: impl Fn(&str, &mut Window, &mut App) + 'static) -> Self {
        let from = self.links.take().and_then(|links| links.from);
        self.links = Some(Links {
            from,
            on_note: Some(Rc::new(handler)),
        });
        self
    }

    /// The words can be read, selected and copied, but not changed (a built-in entry's source).
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// The value's change since the last frame, if any, is the field's own edit (the owner's
    /// own write into the words the DM may be typing: a recording's summary, a plan's link), so
    /// the DM's Undo history stays under it (`docs/design-system.md`, text fields). Without it a
    /// changed value ends the history.
    pub fn own_write(mut self, own: bool) -> Self {
        self.own_write = own;
        self
    }

    /// The words are Markdown source (a note): styled where they are typed, as the Reader sets
    /// them, with a rule under each section heading (`kit::markdown_source`). Every character
    /// stays, markup included. Its words are `Prose` (the body serif at reading size, with
    /// italics) unless `text_type` says otherwise.
    pub fn markdown_source(mut self) -> Self {
        self.markdown = true;
        self
    }

    /// Use the caller's state (`EditorHandle`) in place of one kept under the id.
    pub fn handle(mut self, handle: &EditorHandle) -> Self {
        self.handle = Some(handle.clone());
        self
    }

    /// The words' type (default `Body`; `Mono` with `mono`; `Prose` with `markdown_source`).
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
        let own_write = self.own_write;
        state.update(cx, |state, cx| {
            if markdown && !state.markdown {
                state.markdown = true;
                let words = state.input.read(cx).value();
                state.restyle(&words, window, cx);
            }
            state.follow(&value, on_change, own_write, window, cx)
        });
        let input = state.read(cx).input.clone();
        if self.focus {
            input.update(cx, |input, cx| input.focus(window, cx));
        }
        let focused = input.read(cx).focus_handle(cx).is_focused(window);
        let ty = self.ty.unwrap_or(if self.mono {
            Type::Mono
        } else if self.markdown {
            Type::Prose
        } else {
            Type::Body
        });
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
                // The text area fills the frame's rows, so a press anywhere in the frame puts
                // the caret in it (a frame of no fixed height gives `h_full` nothing to fill).
                let area = Textarea::new(&input)
                    .when(self.markdown, |area| {
                        area.token(|token, _, _| fold_mark(token.line_height()))
                    })
                    .appearance(false)
                    .readonly(self.read_only)
                    .aria_label(name.clone())
                    .h_full()
                    .when(!self.fill, |area| {
                        area.min_h(ty.line() * self.rows as f32 - FOCUS_WIDTH * 2.0)
                    });
                // gpui-component sets its own small size and line; the editor's type replaces
                // them always, as egui's editors drew in their type.
                area.text_size(ty.size())
                    .line_height(ty.line())
                    .font_family(ty.family())
            });
        if self.fill {
            frame = frame.h_full();
        }
        if let Some(links) = self.links.clone() {
            let input = input.clone();
            frame = frame.on_mouse_up(gpui::MouseButton::Left, move |event, window, cx| {
                if event.modifiers.secondary(cx) {
                    follow_link_at(&input, &links, event.position, window, cx);
                }
            });
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

/// A folded `splat:` mark: a faint `⋯` in the icon face, `XS` of room round it, on its line.
fn fold_mark(line: gpui::Pixels) -> impl IntoElement {
    div()
        .h(line)
        .flex()
        .items_center()
        .px(XS / 2.0)
        .child(super::text::icon(
            super::markdown_source::FOLD_MARK,
            Type::Body,
            colors().faint,
        ))
}

/// Follow the link under the pointer at `position`, if the pointer is on its characters.
fn follow_link_at(
    input: &Entity<TextareaState>,
    links: &Links,
    position: gpui::Point<gpui::Pixels>,
    window: &mut Window,
    cx: &mut App,
) {
    use gpui::EntityInputHandler as _;
    let Some(at) = input.update(cx, |input, cx| {
        input.character_index_for_point(position, window, cx)
    }) else {
        return;
    };
    let words = input.read(cx).value();
    // The index is in UTF-16 units; the links are in bytes.
    let mut units = 0;
    let Some(byte) = words.char_indices().find_map(|(byte, letter)| {
        let here = units >= at;
        units += letter.len_utf16();
        here.then_some(byte)
    }) else {
        return;
    };
    let Some(mut target) = super::markdown_source::link_at(&words, byte) else {
        return;
    };
    let ctx = cx
        .try_global::<super::markdown::Host>()
        .map(|host| host.0.clone());
    if let super::markdown::LinkTarget::Note(note) = &target {
        let index = ctx
            .as_ref()
            .and_then(|ctx| ctx.get(crate::requests::LINK_INDEX));
        if index.is_some_and(|index| {
            matches!(
                index.resolve_link(note, links.from),
                crate::world::Go::Missing(_)
            )
        }) {
            target = super::markdown::LinkTarget::Missing;
        } else if let Some(on_note) = &links.on_note {
            on_note(note, window, cx);
            window.refresh();
            return;
        }
    }
    super::markdown::follow(
        &target,
        super::markdown::From::source(links.from),
        (None, None, ctx.as_ref()),
        window,
        cx,
    );
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

#[cfg(test)]
mod tests {
    use super::{EditorHandle, TextEditor};
    use crate::fixtures::Seed;
    use crate::gui::harness::Shell;
    use gpui::{IntoElement, Modifiers, point, px};
    use std::cell::RefCell;
    use std::rc::Rc;

    const MARK: &str = "<!-- splat:check journey=j-1 at=12 sum=9f3a -->";

    /// Show a Markdown editor of `words` in place of the page, with `links` on; its handle. With
    /// `focus`, the program gives it the keyboard in the frame that makes it (a page that opens a
    /// note for typing).
    fn show(shell: &mut Shell, words: &str, links: bool, focus: bool) -> EditorHandle {
        let kept: Rc<RefCell<Option<EditorHandle>>> = Rc::default();
        let draw_kept = kept.clone();
        let words = Rc::new(RefCell::new(words.to_owned()));
        shell.update(move |root, _, cx| {
            let (kept, words) = (draw_kept.clone(), words.clone());
            root.pages.reader.preview = Some(Rc::new(move |_, window, cx| {
                let fresh = kept.borrow().is_none();
                let handle = kept
                    .borrow_mut()
                    .get_or_insert_with(|| EditorHandle::new("", window, cx))
                    .clone();
                if focus && fresh {
                    handle.focus(window, cx);
                }
                let typed = words.clone();
                let editor = TextEditor::new("source-editor", words.borrow().clone())
                    .on_change(move |now: &str, _, _| *typed.borrow_mut() = now.to_owned())
                    .handle(&handle)
                    .name("Source editor")
                    .markdown_source()
                    .rows(8);
                if links {
                    editor.follow_links(None)
                } else {
                    editor
                }
                .into_any_element()
            }));
            cx.notify();
        });
        kept.borrow().clone().expect("the editor's handle")
    }

    /// A `splat:` mark folds while the caret is off its line and opens on it, as egui's did; the
    /// words keep every byte. The focused editor draws its caret in a capture.
    #[test]
    fn a_splat_mark_folds_off_the_carets_line_and_the_caret_shows() {
        let mut shell = Shell::new("gpui-source-fold", Seed::Campaign);
        let text = format!("\nThe road east.\n{MARK}\nWolves at dusk.\n");
        let handle = show(&mut shell, &text, false, true);
        let mark = text.find(MARK).unwrap()..text.find(MARK).unwrap() + MARK.len();
        let read = |shell: &mut Shell, handle: &EditorHandle| {
            let handle = handle.clone();
            shell.update(move |_, _, cx| (handle.folded(cx), handle.words(cx).to_string()))
        };
        // The caret is drawn in an editor the program gave the keyboard as it made it: the window
        // has the keyboard, and the caret does not blink (`docs/design-system.md`, Motion). Half
        // a second on, a blinking caret would be in its hidden half. On the empty first line only
        // the caret differs from the well beside it.
        shell.wait(std::time::Duration::from_millis(500));
        let caret = {
            let handle = handle.clone();
            shell.update(move |_, _, cx| handle.caret_bounds(cx))
        }
        .expect("the caret's place");
        let image = shell.screenshot();
        let at = |x: f32, y: f32| *image.get_pixel(x as u32, y as u32);
        let (x, y) = (f32::from(caret.left()), f32::from(caret.center().y));
        let well = at(x + 20.0, y);
        assert!(
            (-1..=1).any(|dx| at(x + dx as f32, y) != well),
            "a caret is drawn at {caret:?}"
        );
        assert_eq!(
            read(&mut shell, &handle).0,
            vec![mark.clone()],
            "off its line"
        );
        // Down onto the mark's line: it opens, letter by letter.
        shell.press("down down");
        assert!(
            read(&mut shell, &handle).0.is_empty(),
            "on its line it opens"
        );
        shell.press("down");
        assert_eq!(
            read(&mut shell, &handle).0,
            vec![mark.clone()],
            "and folds after"
        );
        // Words typed elsewhere keep the mark whole, and it stays folded.
        shell.type_text("Grey ");
        let (folded, words) = read(&mut shell, &handle);
        assert_eq!(
            words,
            format!("\nThe road east.\n{MARK}\nGrey Wolves at dusk.\n")
        );
        assert_eq!(folded, vec![mark.clone()]);
        shell.capture("gpui-source-fold__01-caret-off-the-mark__1200x800");
        // An editor without the keyboard folds every mark.
        shell.update(|_, window, cx| window.blur(cx));
        assert_eq!(read(&mut shell, &handle).0, vec![mark]);
    }

    /// Ctrl+click on a link in the words follows it, as the Reader does; a plain click only
    /// places the caret; a link that names nothing does nothing.
    #[test]
    fn ctrl_click_on_a_link_in_the_editor_follows_it() {
        let mut shell = Shell::new("gpui-source-links", Seed::World);
        for _ in 0..300 {
            if shell.read(|root| root.app.lookup.index().is_some()) {
                break;
            }
            shell.wait(std::time::Duration::from_millis(100));
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        let text = "Ride to [[Verori|The home country]] or [[Nowhere at all]].\n";
        let handle = show(&mut shell, text, true, false);
        shell.click("Source editor");
        let place = |shell: &mut Shell, words: &str| {
            let (handle, at) = (handle.clone(), text.find(words).unwrap() + 2);
            let bounds = shell
                .update(move |_, _, cx| handle.bounds_at(at, cx))
                .expect("the words are drawn");
            point(bounds.left() + px(1.0), bounds.center().y)
        };
        let ctrl = Modifiers {
            control: true,
            ..Modifiers::default()
        };
        let home = place(&mut shell, "home");
        shell.click_at(home);
        assert!(
            !shell.read(|root| root.app.lookup.open),
            "a plain click only places the caret"
        );
        let nowhere = place(&mut shell, "Nowhere");
        shell.click_at_with(nowhere, ctrl);
        assert!(
            !shell.read(|root| root.app.lookup.open),
            "a link to nothing does nothing"
        );
        shell.click_at_with(home, ctrl);
        assert!(
            shell.read(|root| root.app.lookup.open),
            "Ctrl+click opened the note in Lookup"
        );
    }
}
