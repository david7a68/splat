use splat_trackers::trackers::{Trackers, history};
use uuid::Uuid;

pub(crate) struct Reader<'a>(pub &'a mut Trackers);

impl crate::world::TrackerUi for Reader<'_> {
    // The egui panel and history are gone (the GPUI pages draw them); World's strip removes
    // these two trait methods with their egui callers.
    fn show(&mut self, _: &mut eframe::egui::Ui, _: bool) {}

    fn history(&mut self, _: &mut eframe::egui::Ui, _: &splat_content::content::Entry) {}

    fn history_footer<'f>(
        &self,
        note: Uuid,
        frame: &'f crate::design::outcome::PaneFrame,
        messages: &mut crate::design::outcome::Messages<'f, crate::world::FooterAct>,
    ) {
        history::footer(
            self.0,
            note,
            frame,
            messages,
            crate::world::said,
            crate::world::FooterAct::Trackers,
        );
    }

    fn undo_remove(&mut self, ticket: Option<crate::design::results::Ticket>) {
        self.0.undo_remove(ticket);
    }
}
