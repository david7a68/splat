//! A message line (`design::outcome`'s look): a bar and an icon in the tone's colour, the words in
//! the text colour, and the message's actions under the words, at the words' edge.
//!
//! Where a message goes is the page's rule: a pane's results, reasons and notices sit
//! in its footer (`kit::Pane::footer`); a notice that came by itself sits at the top of the pane's
//! body; a row's fault sits on its row. The tone comes from what happened (`Tone`), never from the
//! caller's taste: Error for a failure, Warning for a risk, Success for a done act, Info otherwise.
use crate::gui::icons;
use crate::gui::theme::{L, LIST_MEASURE, MARK_WIDTH, S, Type, TypeExt, XS, colors};
use gpui::{
    AnyElement, App, Div, Hsla, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, prelude::*,
};

/// A message's tone (`design::Tone`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tone {
    Info,
    Success,
    Warning,
    Error,
}

impl Tone {
    /// Its icon and colour.
    pub fn parts(self) -> (&'static str, Hsla) {
        let result = colors().result;
        match self {
            Self::Info => (icons::INFO, result.info),
            Self::Success => (icons::SUCCESS, result.success),
            Self::Warning => (icons::WARN, result.warning),
            Self::Error => (icons::ERROR, result.error),
        }
    }
}

/// One message: its tone, words, and actions (usually `kit::Button`s; Undo first, quiet).
///
/// ```ignore
/// kit::Message::new(Tone::Success, "Saved “Bridge Ambush”.")
///     .action(kit::Button::quiet("undo-save", "Undo").on_click(...))
/// ```
#[derive(IntoElement)]
pub struct Message {
    tone: Tone,
    words: SharedString,
    small: bool,
    details: Vec<AnyElement>,
    actions: Vec<AnyElement>,
    destructive: Option<AnyElement>,
    dismiss: Option<AnyElement>,
}

impl Message {
    pub fn new(tone: Tone, words: impl Into<SharedString>) -> Self {
        Self {
            tone,
            words: words.into(),
            small: false,
            details: Vec::new(),
            actions: Vec::new(),
            destructive: None,
            dismiss: None,
        }
    }

    /// `Small` words (a row's own fault).
    pub fn small(mut self) -> Self {
        self.small = true;
        self
    }

    /// A line under the words, before the actions (a help line, the file a message is about).
    pub fn detail(mut self, detail: impl IntoElement) -> Self {
        self.details.push(detail.into_any_element());
        self
    }

    /// An action under the words. The first starts at the words' edge; the safe choice comes
    /// first.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }

    /// The action that loses data, after the others with exactly `L` before it.
    pub fn destructive(mut self, action: impl IntoElement) -> Self {
        self.destructive = Some(action.into_any_element());
        self
    }

    /// The message's Dismiss (a `kit::IconButton` `×`), at the right end of its first line, as
    /// egui drew it: the message then takes its parent's width, its words still at most
    /// `LIST_MEASURE` wide.
    pub fn dismiss(mut self, control: impl IntoElement) -> Self {
        self.dismiss = Some(control.into_any_element());
        self
    }
}

impl RenderOnce for Message {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let (icon, color) = self.tone.parts();
        let ty = if self.small { Type::Small } else { Type::Body };
        let words: Div = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .gap(XS)
            .child(div().ty(ty).text_color(colors().text).child(self.words))
            .children(self.details)
            .when_actions(self.actions, self.destructive);
        let dismissed = self.dismiss.is_some();
        let words = if dismissed {
            words.max_w(LIST_MEASURE)
        } else {
            words
        };
        div()
            .flex()
            .flex_row()
            .items_start()
            // In a row it gives way to the row's other parts, and its words wrap there.
            .min_w_0()
            .when(!dismissed, |message| message.max_w(LIST_MEASURE))
            .when(dismissed, |message| message.w_full())
            .border_l(MARK_WIDTH)
            .border_color(color)
            .pl(S - MARK_WIDTH + gpui::px(1.0))
            .gap(XS)
            .child(
                div()
                    .icon_font(ty)
                    .text_color(color)
                    .flex_none()
                    .child(SharedString::new_static(icon)),
            )
            .child(words)
            .when_some(self.dismiss, |message, dismiss| {
                // Centred on the words' first line; the control is taller than the line, so it
                // reaches out above and below it evenly.
                message.child(
                    div()
                        .flex()
                        .flex_none()
                        .ml_auto()
                        .items_center()
                        .h(ty.line())
                        .child(dismiss),
                )
            })
    }
}

trait WhenActions {
    fn when_actions(self, actions: Vec<AnyElement>, destructive: Option<AnyElement>) -> Self;
}

impl WhenActions for Div {
    fn when_actions(self, actions: Vec<AnyElement>, destructive: Option<AnyElement>) -> Self {
        if actions.is_empty() && destructive.is_none() {
            return self;
        }
        let gap_before = if actions.is_empty() {
            gpui::px(0.0)
        } else {
            L - S
        };
        self.child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(S)
                .children(actions)
                .children(destructive.map(|action| div().ml(gap_before).child(action))),
        )
    }
}
