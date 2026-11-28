//! A message line (`design::outcome`'s look): a bar and an icon in the tone's colour, the words in
//! the text colour, and the message's actions `L` after the words when they fit on the words'
//! line, else under the words, at their edge (egui's FK3).
//!
//! Where a message goes is the page's rule: a pane's results, reasons and notices sit
//! in its footer (`kit::Pane::footer`); a notice that came by itself sits at the top of the pane's
//! body; a row's fault sits on its row. The tone comes from what happened (`Tone`), never from the
//! caller's taste: Error for a failure, Warning for a risk, Success for a done act, Info otherwise.
use crate::gui::icons;
use crate::gui::theme::{Density, L, LIST_MEASURE, MARK_WIDTH, S, Type, TypeExt, XS, colors};
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
    actions_under: bool,
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
            actions_under: false,
        }
    }

    /// The actions always on lines under the words (choices that each have a help line under
    /// them, ruling M8), never beside them.
    pub fn actions_under(mut self) -> Self {
        self.actions_under = true;
        self
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

    /// An action: `L` after the words when all the actions fit on the words' line, else under
    /// the words, the first at their edge. The safe choice comes first.
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
        let text = super::target(
            div()
                .ty(ty)
                .text_color(colors().text)
                .min_w_0()
                .child(self.words.clone()),
            self.words,
        );
        let band = actions_band(self.actions, self.destructive);
        let words: Div = div().flex().flex_col().flex_1().min_w_0();
        let words = if self.details.is_empty() && !self.actions_under {
            // egui's rule (FK3): the actions start `L` after the words' last letter when they
            // fit on the words' line, else on a line under the words, at their edge. A column
            // gap lies only between parts on one line, so a wrapped band starts at the edge.
            // The band reaches as far above the words' line as its controls are taller than it.
            let overhang = (Density::Read.control() - ty.line()) / 2.0;
            words.child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_start()
                    .gap_x(L)
                    .gap_y(XS + overhang)
                    .child(text)
                    .when_some(band, |head, band| head.child(band.min_w_0().mt(-overhang))),
            )
        } else {
            words
                .gap(XS)
                .child(text)
                .children(self.details)
                .when_some(band, |words, band| words.child(band))
        };
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

/// The actions in one band: the safe ones `S` apart, then the one that loses data `L` after them.
fn actions_band(actions: Vec<AnyElement>, destructive: Option<AnyElement>) -> Option<Div> {
    if actions.is_empty() && destructive.is_none() {
        return None;
    }
    let gap_before = if actions.is_empty() {
        gpui::px(0.0)
    } else {
        L - S
    };
    Some(
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
