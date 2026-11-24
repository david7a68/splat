//! A pane's footer lines as plain data, for a UI that draws them itself (the GPUI app,
//! `src/gui/`): each line's tone, words, help, its Undo and its choices, each with the `Press`
//! the footer would give back for it (a kept line's carries its store's ticket). The order is the
//! footer's one order (`footer_order`, `refusal_rank`).
//!
//! ```ignore
//! let frame = PaneFrame::headless();
//! let mut messages = Messages::<CampaignAct>::new(&frame, "settings_footer", 0.0);
//! campaigns.footer_messages(&frame, &mut messages, true, blocked);
//! for line in messages.into_plain() { /* draw line.words, line.undo, line.choices */ }
//! ```
use super::{
    ChoiceKind, DISMISS_NAME, Messages, MessageChoice, PaneFrame, Press, ResultOrder, RowMessages,
    Ticket, Tone, Typed, footer_order, holding, refusal_rank,
};

/// One footer line.
pub struct PlainLine<A> {
    pub tone: Tone,
    pub words: String,
    /// What the system itself said, on hover over the words.
    pub hover: Option<String>,
    /// What the DM can do, under the words.
    pub help: Vec<String>,
    /// The file it is about, as the footer names it.
    pub file: Option<String>,
    pub undo: Option<PlainChoice<A>>,
    /// Safe choices first; a choice that loses something last.
    pub choices: Vec<PlainChoice<A>>,
}

/// One control of a footer line.
pub struct PlainChoice<A> {
    /// Its whole words (its accessible name).
    pub label: String,
    pub kind: PlainKind,
    /// Why it waits now, while it waits.
    pub locked: Option<String>,
    /// Its help line, under the line.
    pub help: Option<String>,
    /// What the footer gives back when it is pressed.
    pub press: Press<A>,
}

/// How a choice draws.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlainKind {
    /// An Undo.
    Undo,
    /// A secondary choice.
    Choice,
    /// The message's one commit.
    Primary,
    /// A destructive choice: `L` after the safe ones.
    Lose,
    /// A link in the words.
    Link,
    /// The close icon.
    Dismiss,
}

impl PaneFrame {
    /// The token for a pane drawn by a UI with no egui `Ui` (the GPUI app), for this frame.
    pub fn headless() -> Self {
        Self {
            _not_static: std::marker::PhantomData,
        }
    }
}

impl<A: 'static> Messages<'_, A> {
    /// Every line, in the footer's order, as plain data.
    pub fn into_plain(self) -> Vec<PlainLine<A>> {
        let newest = self.newest;
        let mut lines = self.lines;
        for line in &mut lines {
            if line.refused {
                line.rank = refusal_rank(line.order.unwrap_or_default(), newest);
            }
        }
        lines.sort_by_key(|line| (line.rank, line.order.map(std::cmp::Reverse)));
        lines
            .into_iter()
            .map(|line| plain(line.typed, line.ticket.as_ref().map(Ticket::order)))
            .collect()
    }
}

impl<A: 'static> RowMessages<'_, A> {
    /// Every line, in the row's order, as plain data.
    pub fn into_plain(self) -> Vec<PlainLine<A>> {
        let mut items = self.items;
        let newest = items.iter().filter_map(|item| item.1).max();
        for (rank, order, _, _, refused) in &mut items {
            if let (true, Some(attempt)) = (*refused, *order) {
                *rank = refusal_rank(attempt, newest);
            }
        }
        footer_order(&mut items);
        items
            .into_iter()
            .map(|(_, _, typed, ticket, _)| plain(typed, ticket.as_ref().map(Ticket::order)))
            .collect()
    }
}

/// One typed line as plain data; `order` is its kept result's, for its presses' tickets.
fn plain<A>(typed: Typed<A>, order: Option<ResultOrder>) -> PlainLine<A> {

                let ticket = || order.map(Ticket::of);
                let body = typed.body;
                let undo = match (body.undo.as_ref(), typed.undo) {
                    (Some(step), Some(id)) => Some(PlainChoice {
                        label: step.label(),
                        kind: PlainKind::Undo,
                        locked: holding(&body.undo_lock).map(str::to_owned),
                        help: None,
                        press: Press {
                            id,
                            ticket: ticket(),
                        },
                    }),
                    _ => None,
                };
                let choices = typed
                    .choices
                    .into_iter()
                    .enumerate()
                    .map(|(at, id)| {
                        let what = body.choices.get(at);
                        let named = body.named.get(at).and_then(Option::as_ref);
                        let (label, kind) = match (named, what) {
                            (Some((kind, words)), _) => (
                                words.label(),
                                match kind {
                                    ChoiceKind::Safe => PlainKind::Choice,
                                    ChoiceKind::Lose => PlainKind::Lose,
                                },
                            ),
                            (None, Some(MessageChoice::Choice(words))) => {
                                ((*words).to_owned(), PlainKind::Choice)
                            }
                            (None, Some(MessageChoice::Primary(words))) => {
                                ((*words).to_owned(), PlainKind::Primary)
                            }
                            (None, Some(MessageChoice::Lose(words))) => {
                                ((*words).to_owned(), PlainKind::Lose)
                            }
                            (None, Some(MessageChoice::Link { word, .. })) => {
                                ((*word).to_owned(), PlainKind::Link)
                            }
                            (None, Some(MessageChoice::Dismiss(_)) | None) => {
                                (DISMISS_NAME.to_owned(), PlainKind::Dismiss)
                            }
                        };
                        PlainChoice {
                            label,
                            kind,
                            locked: body
                                .locks
                                .get(at)
                                .and_then(holding)
                                .map(str::to_owned),
                            help: body
                                .helps
                                .get(at)
                                .and_then(Option::as_ref)
                                .map(|help| help.as_str().to_owned()),
                            press: Press {
                                id,
                                ticket: ticket(),
                            },
                        }
                    })
                    .collect();
                PlainLine {
                    tone: body.tone,
                    words: body.words.as_str().to_owned(),
                    hover: body.words_hover.clone(),
                    help: body
                        .help
                        .iter()
                        .map(|help| help.as_str().to_owned())
                        .collect(),
                    file: body.file.as_ref().map(super::FileLine::shown),
                    undo,
                    choices,
                }
}
