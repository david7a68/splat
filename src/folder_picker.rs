use eframe::egui;
use std::{path::PathBuf, sync::mpsc};

#[derive(Default)]
pub struct FolderPicker {
    pending: Option<mpsc::Receiver<Option<PathBuf>>>,
}
impl FolderPicker {
    pub fn show(&mut self, ui: &mut egui::Ui, path: &mut String) {
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(selected) => {
                    if let Some(selected) = selected {
                        *path = selected.to_string_lossy().into_owned();
                    }
                    self.pending = None;
                }
                Err(mpsc::TryRecvError::Disconnected) => self.pending = None,
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        if ui
            .add_enabled(self.pending.is_none(), egui::Button::new("Browse…"))
            .clicked()
        {
            let (tx, rx) = mpsc::channel();
            self.pending = Some(rx);
            let ctx = ui.ctx().clone();
            std::thread::spawn(move || {
                let _ = tx.send(
                    rfd::FileDialog::new()
                        .set_title("Choose campaign data folder")
                        .pick_folder(),
                );
                ctx.request_repaint();
            });
        }
    }
}
