//! Timeline Undo fixtures for its book and page tests.
use super::Book;

/// Which of the book's kept results a test presses Undo on.
#[derive(Clone, Copy)]
pub enum TestUndo {
    Remove,
    Reload,
    Ruling,
}

/// Press that result's Undo in a footer made from the book's store, as the pages make it: the
/// line comes from the store's view and the press acts by its ticket. Returns what `undo_remove`
/// returns (the event put back), else `None`. With no such result, no Undo shows: `None`.
pub fn press_footer_undo(book: &mut Book, which: TestUndo) -> Option<String> {
    use crate::design::outcome::{Messages, PaneFrame, Said, UndoStep, Verb};
    let frame = PaneFrame::plain();
    let mut messages = Messages::new(&frame, "test_footer", 0.0);
    let file = book.file();
    let words = crate::sentence!("A result.");
    let step = match which {
        TestUndo::Remove => UndoStep::verb(Verb::Remove),
        TestUndo::Reload => UndoStep::verb(Verb::Reload),
        TestUndo::Ruling => UndoStep::of(Verb::Change, "ruling"),
    };
    // Each store keeps its own kind of result: the same line from each.
    macro_rules! line {
        ($store:ident) => {
            book.results.$store.get(&file).map(|shown| {
                shown.outcome(&frame, |_| {
                    Said::new(words).undo(step.pressed_as(())).into()
                })
            })
        };
    }
    let kept = match which {
        TestUndo::Remove => line!(removal),
        TestUndo::Reload => line!(reloaded),
        TestUndo::Ruling => line!(ruling),
    };
    if let Some(kept) = kept {
        messages.kept(kept);
    }
    let ticket = messages
        .into_plain()
        .iter()
        .find_map(|line| line.press_undo())
        .and_then(|press| press.ticket)?;
    match which {
        TestUndo::Remove => book.undo_remove(ticket),
        TestUndo::Reload => {
            book.undo_reload(ticket);
            None
        }
        TestUndo::Ruling => {
            book.undo_restores(ticket);
            None
        }
    }
}
