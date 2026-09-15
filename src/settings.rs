use crate::{
    auth::Session,
    credentials::{Action, Worker},
};
use eframe::egui;
use std::{sync::mpsc, time::Duration};
use zeroize::{Zeroize, Zeroizing};

pub struct Settings {
    cookie_input: String,
    session: Option<Session>,
    worker: Option<Worker>,
    generation: u64,
    busy: bool,
    initialized: bool,
    remember: bool,
    error: Option<String>,
    warning: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            cookie_input: String::new(),
            session: None,
            worker: None,
            generation: 0,
            busy: false,
            initialized: false,
            remember: true,
            error: None,
            warning: None,
        }
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        self.cookie_input.zeroize();
    }
}

impl Settings {
    fn send(&mut self, action: Action, ctx: &egui::Context) {
        if !matches!(&action, Action::Refresh(_)) {
            self.warning = None;
        }
        self.generation += 1;
        self.error = None;
        self.busy = true;
        let worker = self.worker.get_or_insert_with(|| Worker::new(ctx.clone()));
        if let Err(error) = worker.send(self.generation, action) {
            self.error = Some(error);
            self.busy = false;
        }
    }

    pub fn poll(&mut self, ctx: &egui::Context) {
        if !self.initialized {
            self.initialized = true;
            self.send(Action::Restore, ctx);
        }
        if let Some(worker) = &self.worker {
            loop {
                match worker.rx.try_recv() {
                    Ok(reply) => {
                        if reply.id != self.generation {
                            continue;
                        }
                        self.busy = false;
                        match reply.result {
                            Ok(outcome) => {
                                self.session = outcome.session;
                                // Refresh must not erase a failed-save warning.
                                if outcome.warning.is_some() {
                                    self.warning = outcome.warning;
                                }
                                self.error = None;
                            }
                            Err(error) => {
                                self.session = None;
                                self.error = Some(error);
                            }
                        }
                    }
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => {
                        self.busy = false;
                        self.error = Some("The session worker stopped. Restart Splat.".into());
                        break;
                    }
                }
            }
        }
        if !self.busy
            && let Some(session) = &self.session
            && session.needs_refresh()
        {
            self.send(Action::Refresh(session.cookie()), ctx);
        }
        if self.session.is_some() || self.busy {
            ctx.request_repaint_after(Duration::from_secs(1));
        }
    }

    fn disconnect(&mut self, ctx: &egui::Context) {
        self.session = None;
        self.cookie_input.zeroize();
        self.warning = None;
        self.send(Action::Forget, ctx);
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
        ui.label("Splat sends your session cookie only to D&D Beyond. Save it in your desktop keyring to reconnect automatically at startup. The access token stays in memory.");
        ui.add_space(12.0);
        if self.busy {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label("Updating D&D Beyond session…");
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
        if let Some(warning) = &self.warning {
            ui.colored_label(egui::Color32::YELLOW, warning);
        }
        if self.session.is_none() && !self.busy {
            ui.checkbox(&mut self.remember, "Remember session in desktop keyring");
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
                self.send(
                    crate::credentials::Action::Connect {
                        cookie,
                        remember: self.remember,
                    },
                    ui.ctx(),
                );
            }
        }
        if (self.session.is_some() || self.busy || self.warning.is_some() || self.error.is_some())
            && ui.button("Disconnect").clicked()
        {
            self.disconnect(ui.ctx());
        }
        ui.add_space(12.0);
        ui.weak("Disconnect removes Splat's saved session and clears local credentials. It does not sign your browser out of D&D Beyond.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credentials::{Outcome, Reply};

    #[test]
    fn disconnect_clears_input_and_ignores_late_auth_results() {
        let ctx = egui::Context::default();
        let (worker, commands, results) = Worker::test_channels();
        let mut settings = Settings::default();
        settings.initialized = true;
        settings.worker = Some(worker);
        settings.cookie_input = "test-cookie".into();
        settings.generation = 1;
        settings.busy = true;
        settings.disconnect(&ctx);
        let (id, action) = commands.recv().unwrap();
        assert!(matches!(action, Action::Forget));
        assert!(settings.cookie_input.is_empty());
        results
            .send(Reply {
                id: 1,
                result: Err("stale error".into()),
            })
            .unwrap();
        settings.poll(&ctx);
        assert!(settings.error.is_none());
        assert!(settings.busy);
        results
            .send(Reply {
                id,
                result: Ok(Outcome {
                    session: None,
                    warning: None,
                }),
            })
            .unwrap();
        settings.poll(&ctx);
        assert!(!settings.busy);
        assert!(settings.session.is_none());
    }
}
