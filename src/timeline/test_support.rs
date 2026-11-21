//! Timeline Undo fixtures for its book and page tests.
use super::Book;

/// Which of the book's kept results a test presses Undo on.
#[derive(Clone, Copy)]
pub enum TestUndo {
    Remove,
    Reload,
    Ruling,
}

/// Press that result's Undo in a footer drawn from the book's store, as the pages do: the line
/// comes from the store's view and the press acts by its ticket. Returns what `undo_remove`
/// returns (the event put back), else `None`. With no such result, no Undo shows: `None`.
pub fn press_footer_undo(book: &mut Book, which: TestUndo) -> Option<String> {
    use crate::design::outcome::{Messages, PaneFrame, Said, UndoStep, Verb};
    use egui_kittest::kittest::Queryable;
    let label = match which {
        TestUndo::Remove => "Undo remove",
        TestUndo::Reload => "Undo reload",
        TestUndo::Ruling => "Undo change to ruling",
    };
    let mut harness = egui_kittest::Harness::builder()
        .with_size(eframe::egui::vec2(800.0, 300.0))
        .build_state(
            move |ctx, (book, back): &mut (Book, Option<String>)| {
                eframe::egui::CentralPanel::default().show(ctx, |ui| {
                    let frame = PaneFrame::new(ui);
                    let mut messages = Messages::new(&frame, "test_footer", ui.available_width());
                    let file = book.file();
                    let words = crate::sentence!("A result.");
                    match which {
                        TestUndo::Remove => {
                            if let Some(shown) = book.results.removal.get(&file) {
                                messages.kept(shown.outcome(&frame, |_| {
                                    Said::new(words)
                                        .undo(UndoStep::verb(Verb::Remove).pressed_as(()))
                                        .into()
                                }));
                            }
                        }
                        TestUndo::Reload => {
                            if let Some(shown) = book.results.reloaded.get(&file) {
                                messages.kept(shown.outcome(&frame, |_| {
                                    Said::new(words)
                                        .undo(UndoStep::verb(Verb::Reload).pressed_as(()))
                                        .into()
                                }));
                            }
                        }
                        TestUndo::Ruling => {
                            if let Some(shown) = book.results.ruling.get(&file) {
                                messages.kept(shown.outcome(&frame, |_| {
                                    Said::new(words)
                                        .undo(UndoStep::of(Verb::Change, "ruling").pressed_as(()))
                                        .into()
                                }));
                            }
                        }
                    }
                    let paned = messages.pane(ui, crate::design::Body::Content, |_| ());
                    if let Some(ticket) = paned.pressed.and_then(|press| press.ticket) {
                        match which {
                            TestUndo::Remove => *back = book.undo_remove(ticket),
                            TestUndo::Reload => book.undo_reload(ticket),
                            TestUndo::Ruling => book.undo_restores(ticket),
                        }
                    }
                });
            },
            (std::mem::take(book), None),
        );
    crate::design::theme::install(&harness.ctx);
    harness.run_steps(2);
    if harness.query_by_label(label).is_some() {
        harness.get_by_label(label).click();
        harness.run_steps(2);
    }
    let (after, back) = std::mem::take(harness.state_mut());
    *book = after;
    back
}
