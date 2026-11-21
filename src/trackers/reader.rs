use splat_trackers::trackers::{Trackers, history, view};
use uuid::Uuid;

pub(crate) struct Reader<'a>(pub &'a mut Trackers);

impl crate::world::TrackerUi for Reader<'_> {
    fn show(&mut self, ui: &mut eframe::egui::Ui, from_top: bool) {
        view::show(ui, self.0, from_top);
    }

    fn history(&mut self, ui: &mut eframe::egui::Ui, note: &splat_content::content::Entry) {
        history::show(ui, self.0, note);
    }

    fn history_footer(
        &self,
        note: Uuid,
        messages: &mut crate::design::outcome::Messages<'_, crate::world::FooterAct>,
    ) {
        history::footer(
            self.0,
            note,
            messages,
            crate::world::said,
            crate::world::FooterAct::Trackers,
        );
    }

    fn undo_remove(&mut self) {
        self.0.undo_remove();
    }
}
