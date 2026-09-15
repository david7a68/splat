use crate::auth::{self, Session};
use eframe::egui;
use std::{
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};
use zeroize::{Zeroize, Zeroizing};

#[derive(Default)]
pub struct Settings {
    cookie_input: String,
    session: Option<Session>,
    pending: Option<Receiver<Result<Session, String>>>,
    error: Option<String>,
}

impl Drop for Settings {
    fn drop(&mut self) {
        self.cookie_input.zeroize();
    }
}

impl Settings {
    fn start(&mut self, cookie: Zeroizing<String>, ctx: &egui::Context) {
        let (tx, rx) = mpsc::channel();
        let ctx = ctx.clone();
        self.pending = Some(rx);
        self.error = None;
        thread::spawn(move || {
            let _ = tx.send(auth::connect(cookie));
            ctx.request_repaint();
        });
    }

    pub fn poll(&mut self, ctx: &egui::Context) {
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    match result {
                        Ok(session) => {
                            self.session = Some(session);
                            self.error = None;
                        }
                        Err(error) => {
                            self.session = None;
                            self.error = Some(error);
                        }
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.session = None;
                    self.error = Some("The connection stopped. Please try again.".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        if self.pending.is_none()
            && let Some(session) = &self.session
            && session.needs_refresh()
        {
            self.start(session.cookie(), ctx);
        }
        if self.session.is_some() || self.pending.is_some() {
            ctx.request_repaint_after(Duration::from_secs(1));
        }
    }

    fn disconnect(&mut self) {
        // Dropping the receiver also discards late results from an in-flight request.
        self.pending = None;
        self.session = None;
        self.cookie_input.zeroize();
        self.error = None;
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.separator();
        ui.heading("D&D Beyond");
        ui.label("Connect your account to prepare for character and campaign imports.");
        ui.hyperlink_to(
            "Open D&D Beyond sign-in",
            "https://www.dndbeyond.com/sign-in",
        );
        ui.collapsing("How to get your session cookie", |ui| {
            ui.label("1. Sign in to D&D Beyond in your browser.");
            ui.label("2. Open developer tools (F12), then Application or Storage → Cookies.");
            ui.label("3. Select dndbeyond.com and copy the value of CobaltSession.");
            ui.label("4. Paste only the value below and select Connect.");
        });
        ui.label("Your session cookie grants account access. Splat sends it only to D&D Beyond and keeps credentials in memory until you disconnect or close the app.");
        ui.add_space(12.0);
        if self.pending.is_some() {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Connecting to D&D Beyond…");
            });
        } else if self.session.is_some() {
            ui.colored_label(egui::Color32::LIGHT_GREEN, "Connected to D&D Beyond");
            ui.label("The access token refreshes automatically while Splat is open.");
        } else {
            ui.label("Not connected");
        }
        if let Some(error) = &self.error {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
        if self.session.is_none() && self.pending.is_none() {
            ui.label("CobaltSession");
            ui.add(
                egui::TextEdit::singleline(&mut self.cookie_input)
                    .password(true)
                    .hint_text("Paste session cookie")
                    .desired_width(420.0),
            );
            if ui
                .add_enabled(
                    !self.cookie_input.trim().is_empty(),
                    egui::Button::new("Connect"),
                )
                .clicked()
            {
                let cookie = Zeroizing::new(self.cookie_input.trim().to_owned());
                self.cookie_input.zeroize();
                self.start(cookie, ui.ctx());
            }
        }
        if (self.session.is_some() || self.pending.is_some()) && ui.button("Disconnect").clicked() {
            self.disconnect();
        }
        ui.add_space(12.0);
        ui.weak("Disconnect clears Splat's credentials. It does not sign your browser out of D&D Beyond.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disconnect_discards_pending_results_and_input() {
        let (tx, rx) = mpsc::channel();
        let mut settings = Settings {
            cookie_input: "secret".into(),
            pending: Some(rx),
            session: None,
            error: None,
        };
        settings.disconnect();
        assert!(settings.cookie_input.is_empty());
        assert!(settings.pending.is_none());
        assert!(tx.send(Err("late error".into())).is_err());
    }
}
