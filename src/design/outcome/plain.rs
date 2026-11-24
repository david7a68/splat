//! A pane footer's messages as plain data, for the GPUI footer (`gui::kit::messages`). The
//! `Messages` a page builds gives its lines here, in the footer's one order (`footer_order`),
//! with each line's tone, words, Undo, choices and file line, and the press each control makes.
//! A kept line's press carries its store's ticket, so it reaches only the entry the line showed.
//!
//! ```ignore
//! let frame = PaneFrame::new();
//! let mut messages = Messages::new();
//! messages.outcome(Notice::new(sentence!("Sync is off.")));
//! for line in messages.into_plain() {
//!     // Draw line.words in line.tone; a press of its Undo gives `line.press_undo()`.
//! }
//! ```
use super::{
    FaultMessage, FooterRank, KeptLine, Lost, MessageChoice, Messages, Outcome, Press, Tone, Typed,
    footer_order, holding, refusal_rank,
};
use crate::design::results::Ticket;

/// What a choice of a message is: its look and what it does.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChoiceForm {
    /// A safe choice: bordered (secondary).
    Choice,
    /// The message's one commit.
    Primary,
    /// It loses something: red, `L` after the others.
    Lose,
    /// A link (opens a file, a page).
    Link,
    /// Dismiss the message.
    Dismiss,
}

/// One choice of a plain line.
#[derive(Clone, Debug, PartialEq)]
pub struct PlainChoice {
    pub form: ChoiceForm,
    pub words: String,
    /// Words on hover (a link's path).
    pub hover: Option<String>,
    /// Why it cannot be pressed now.
    pub locked: Option<String>,
    /// What pressing it does, in a help line under it.
    pub help: Option<String>,
}

/// A plain line's Undo.
#[derive(Clone, Debug, PartialEq)]
pub struct PlainUndo {
    /// `Undo remove Ash`: the whole label.
    pub label: String,
    /// Why it waits now.
    pub locked: Option<String>,
}

/// The file a line is about, on a line under its words.
#[derive(Clone, Debug, PartialEq)]
pub struct PlainFile {
    /// The path as the DM reads it (relative to its folder).
    pub shown: String,
    /// The whole path, when its links may open it.
    pub path: Option<std::path::PathBuf>,
    /// With `Edit in Obsidian`.
    pub edit: bool,
    /// With `Show file`.
    pub show: bool,
}

/// One footer line as data. Its presses are made with `press_undo` and `press_choice`.
pub struct PlainLine<A> {
    pub tone: Tone,
    pub words: String,
    /// What the system itself said, on hover over the words.
    pub hover: Option<String>,
    /// Help lines under the words.
    pub help: Vec<String>,
    pub undo: Option<PlainUndo>,
    pub choices: Vec<PlainChoice>,
    pub file: Option<PlainFile>,
    undo_id: Option<A>,
    choice_ids: Vec<A>,
    /// A kept line's ticket, by its order.
    ticket: Option<crate::design::ResultOrder>,
}

impl<A: Clone> PlainLine<A> {
    /// The press of its Undo, unless it waits.
    pub fn press_undo(&self) -> Option<Press<A>> {
        if self.undo.as_ref().is_some_and(|undo| undo.locked.is_some()) {
            return None;
        }
        Some(Press {
            id: self.undo_id.clone()?,
            ticket: self.ticket.map(Ticket::of),
        })
    }

    /// The press of its choice at `at`, unless it waits.
    pub fn press_choice(&self, at: usize) -> Option<Press<A>> {
        if self.choices.get(at)?.locked.is_some() {
            return None;
        }
        Some(Press {
            id: self.choice_ids.get(at)?.clone(),
            ticket: self.ticket.map(Ticket::of),
        })
    }
}

impl<A> PlainLine<A> {
    fn of(typed: Typed<A>, ticket: Option<crate::design::ResultOrder>) -> Self {
        let Typed {
            body,
            undo,
            choices,
        } = typed;
        let plain_choices = body
            .choices
            .iter()
            .enumerate()
            .map(|(at, what)| {
                let (form, words, hover) = match what {
                    MessageChoice::Choice(words) => (ChoiceForm::Choice, (*words).to_owned(), None),
                    MessageChoice::Primary(words) => {
                        (ChoiceForm::Primary, (*words).to_owned(), None)
                    }
                    MessageChoice::Lose(words) => (ChoiceForm::Lose, (*words).to_owned(), None),
                    MessageChoice::Link { word, hover } => (
                        ChoiceForm::Link,
                        (*word).to_owned(),
                        hover.map(str::to_owned),
                    ),
                    MessageChoice::Dismiss(lost) => (
                        ChoiceForm::Dismiss,
                        super::DISMISS_NAME.to_owned(),
                        (*lost == Lost::TypedWords).then(|| super::DISMISS_NAME.to_owned()),
                    ),
                };
                // A choice named from data says its whole words.
                let words = body
                    .named
                    .get(at)
                    .and_then(Option::as_ref)
                    .map_or(words, |(_, named)| named.label());
                PlainChoice {
                    form,
                    words,
                    hover: body.hovers.get(at).cloned().flatten().or(hover),
                    locked: body
                        .locks
                        .get(at)
                        .and_then(|lock| holding(lock).map(str::to_owned)),
                    help: body
                        .helps
                        .get(at)
                        .cloned()
                        .flatten()
                        .map(|help| help.to_string()),
                }
            })
            .collect();
        Self {
            tone: body.tone,
            words: body.words.to_string(),
            hover: body.words_hover.clone(),
            help: body.help.iter().map(ToString::to_string).collect(),
            undo: body.undo.as_ref().map(|step| PlainUndo {
                label: step.label(),
                locked: holding(&body.undo_lock).map(str::to_owned),
            }),
            choices: plain_choices,
            file: body.file.as_ref().map(|file| PlainFile {
                shown: file.shown(),
                path: file.path(),
                edit: file.edit && file.openable,
                show: file.show && file.openable,
            }),
            undo_id: undo,
            choice_ids: choices,
            ticket,
        }
    }
}

impl<'f, A: 'static> Messages<'f, A> {
    /// Every line, in the footer's one order (`footer_order`): an answer first, results with a
    /// live Undo newest first, reasons, then notices and results with no Undo.
    pub fn into_plain(self) -> Vec<PlainLine<A>> {
        let newest = self.newest;
        let mut lines: Vec<_> = self
            .lines
            .into_iter()
            .map(|mut line| {
                if line.refused {
                    line.rank = refusal_rank(line.order.unwrap_or_default(), newest);
                }
                Ranked(line.rank, line.order, line.typed, line.ticket)
            })
            .collect();
        footer_order(&mut lines);
        lines
            .into_iter()
            .map(|Ranked(_, _, typed, ticket)| PlainLine::of(typed, ticket.map(|t| t.order())))
            .collect()
    }
}

/// Where an owner's lines go: a pane's footer ([`Messages`]) or a save row's own messages
/// ([`RowMessages`]). An owner that feeds both writes its lines once, against this.
pub trait Sink<'f, A: 'static> {
    fn outcome(&mut self, outcome: Outcome<A>);
    fn kept(&mut self, line: KeptLine<'f, A>);
}

impl<'f, A: 'static> Sink<'f, A> for Messages<'f, A> {
    fn outcome(&mut self, outcome: Outcome<A>) {
        Messages::outcome(self, outcome);
    }
    fn kept(&mut self, line: KeptLine<'f, A>) {
        Messages::kept(self, line);
    }
}

impl<'f, A> KeptLine<'f, A> {
    /// The line as plain data, for a place that is no footer (a result under a field).
    pub fn into_plain(self) -> PlainLine<A> {
        PlainLine::of(self.typed, Some(self.ticket.order()))
    }
}

impl<A> FaultMessage<A> {
    /// The message as plain data, for a place that is no footer (a page's placeholder, the
    /// body's own message).
    pub fn into_plain(self) -> PlainLine<A> {
        let (_, typed) = super::Outcome::from(self).parts();
        PlainLine::of(typed, None)
    }
}

impl<A> super::Notice<A> {
    /// The notice as plain data, for a place that is no footer (a notice over a list).
    pub fn into_plain(self) -> PlainLine<A> {
        let (_, typed) = super::Outcome::from(self).parts();
        PlainLine::of(typed, None)
    }
}

/// The tone of a fault: its severity's (`Severity`), so every UI gives one fault one tone.
pub fn fault_tone(fault: &dyn super::Fault) -> Tone {
    fault.severity().tone()
}

/// One line with its place in the footer's order.
struct Ranked<A>(
    FooterRank,
    Option<crate::design::ResultOrder>,
    Typed<A>,
    Option<Ticket>,
);

impl<A> super::Ranked for Ranked<A> {
    fn key(&self) -> (FooterRank, Option<crate::design::ResultOrder>) {
        (self.0, self.1)
    }
}
