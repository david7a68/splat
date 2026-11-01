use crate::{
    auth::Session,
    credentials::{Action, Worker},
    design::{self, Density, Status, Tone, Type, colors, icons},
};
use eframe::egui;
use std::{sync::mpsc, time::Duration};
use zeroize::{Zeroize, Zeroizing};

pub struct Settings {
    cookie_input: String,
    session: Option<Session>,
    worker: Option<Worker>,
    generation: u64,
    account_epoch: u64,
    busy: bool,
    initialized: bool,
    restoring: bool,
    remember: bool,
    error: Option<String>,
    warning: Option<String>,
    /// A saved session failed to restore, so one is still in the keyring.
    saved_session: bool,
    /// Test builds cannot reach these states. Only the words are checked.
    #[cfg(test)]
    simulated: Option<Account>,
}

/// Every state of the account, each with words.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Account {
    Busy,
    Connected,
    NotConnected,
    Failed,
    KeyringWarning,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            cookie_input: String::new(),
            session: None,
            worker: None,
            generation: 0,
            account_epoch: 0,
            busy: false,
            initialized: false,
            restoring: false,
            remember: true,
            error: None,
            warning: None,
            saved_session: false,
            #[cfg(test)]
            simulated: None,
        }
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        self.cookie_input.zeroize();
    }
}

impl Settings {
    #[cfg(test)]
    pub fn busy(&self) -> bool {
        self.busy
    }

    #[cfg(test)]
    pub fn simulate(&mut self, account: Account) {
        self.simulated = Some(account);
    }

    fn account(&self) -> Account {
        #[cfg(test)]
        if let Some(account) = self.simulated {
            return account;
        }
        if self.busy {
            Account::Busy
        } else if self.session.is_some() && self.warning.is_some() {
            Account::KeyringWarning
        } else if self.session.is_some() {
            Account::Connected
        } else if self.error.is_some() {
            Account::Failed
        } else {
            Account::NotConnected
        }
    }

    pub fn connecting(&self) -> bool {
        self.busy
    }

    pub fn campaign_session(&self) -> (u64, Option<Session>) {
        (self.account_epoch, self.session.clone())
    }

    fn send(&mut self, action: Action, ctx: &egui::Context) {
        self.restoring = matches!(&action, Action::Restore);
        if !matches!(&action, Action::Refresh(_)) {
            self.warning = None;
            self.account_epoch += 1;
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
                                if outcome.session.is_some() {
                                    // Kept only for a retry after a failed attempt.
                                    self.cookie_input.zeroize();
                                }
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
                                self.saved_session = self.restoring;
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
        let c = colors();
        ui.label(Type::Section.rich(ui, "D&D Beyond account").color(c.text));
        let small = |ui: &mut egui::Ui, text: &str| {
            ui.add(egui::Label::new(Type::Small.rich(ui, text).color(c.muted)).wrap());
        };
        let account = self.account();
        let state = match account {
            Account::Busy => Status {
                icon: icons::BUSY,
                word: "Connecting: updating your D&D Beyond session",
                color: c.connection.busy,
            },
            Account::Connected => Status {
                icon: icons::ONLINE,
                word: "Connected to D&D Beyond",
                color: c.connection.connected,
            },
            Account::NotConnected => Status {
                icon: icons::OFFLINE,
                word: "Not connected",
                color: c.connection.offline,
            },
            Account::Failed => Status {
                icon: icons::ERROR,
                word: "Not connected: the last attempt failed",
                color: c.result.error,
            },
            Account::KeyringWarning => Status {
                icon: icons::WARN,
                word: "Connected, but the session was not saved",
                color: c.result.warning,
            },
        };
        ui.horizontal(|ui| {
            if account == Account::Busy {
                ui.spinner();
            }
            design::status(ui, state);
        });
        match account {
            Account::Connected => {
                small(
                    ui,
                    "The access token refreshes automatically while Splat is open.",
                );
            }
            Account::KeyringWarning => {
                small(ui, "You will need to connect again next time Splat opens.");
            }
            _ => {}
        }
        // A simulated connection has no attempt behind it, so an older error does not apply.
        #[cfg(test)]
        let old_error = matches!(
            self.simulated,
            Some(Account::Busy | Account::Connected | Account::KeyringWarning)
        );
        #[cfg(not(test))]
        let old_error = false;
        if let Some(error) = self.error.as_ref().filter(|_| !old_error) {
            design::message(ui, Tone::Error, error);
        }
        if let Some(warning) = &self.warning {
            design::message(ui, Tone::Warning, warning);
        }
        if matches!(account, Account::NotConnected | Account::Failed) {
            small(
                ui,
                "Connect your account to import campaigns and read characters.",
            );
            if design::link(ui, "Open D&D Beyond sign-in").clicked() {
                ui.ctx()
                    .open_url(egui::OpenUrl::new_tab("https://www.dndbeyond.com/sign-in"));
            }
            design::section(ui, "How to get your session cookie", |ui| {
                ui.label("1. Sign in to D&D Beyond in your browser.");
                ui.label("2. Open developer tools (F12), then Application or Storage → Cookies.");
                ui.label("3. Select dndbeyond.com and copy the value of CobaltSession.");
                ui.label("4. Paste only the value below and select Connect.");
            });
            design::field(ui, "CobaltSession", None, None, |ui| {
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.cookie_input)
                        .password(true)
                        .hint_text("Paste session cookie")
                        .min_size(egui::vec2(0.0, Density::Read.control()))
                        .margin(egui::vec2(design::S, 5.0))
                        .desired_width(420.0_f32.min(ui.available_width() - 2.0 * design::S)),
                );
                design::named(ui, &response, "Paste session cookie");
            });
            ui.checkbox(&mut self.remember, "Remember session in desktop keyring");
            if design::Action::primary("Connect")
                .blocked_if(
                    self.cookie_input.trim().is_empty(),
                    "Paste your session cookie first.",
                )
                .show(ui)
                .clicked()
            {
                let cookie = Zeroizing::new(self.cookie_input.trim().to_owned());
                self.send(
                    crate::credentials::Action::Connect {
                        cookie,
                        remember: self.remember,
                    },
                    ui.ctx(),
                );
            }
            small(
                ui,
                "Splat sends your session cookie only to D&D Beyond. In the keyring, it can reconnect at startup. The access token stays in memory.",
            );
        }
        // Only when there is something to forget.
        if (!matches!(account, Account::NotConnected | Account::Failed)
            || self.warning.is_some()
            || self.saved_session)
            && design::Action::secondary("Disconnect")
                .density(Density::Read)
                .show(ui)
                .clicked()
        {
            self.disconnect(ui.ctx());
        }
        if matches!(account, Account::Connected | Account::KeyringWarning) {
            small(
                ui,
                "Disconnect removes Splat's saved session and clears local credentials. It does not sign your browser out of D&D Beyond.",
            );
        }
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
