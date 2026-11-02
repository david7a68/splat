use crate::{
    auth::Session,
    credentials::{Action, Worker},
    design::{self, Density, Status, Tone, Type, colors, icons},
};
use eframe::egui;
use std::{
    sync::mpsc,
    time::{Duration, Instant},
};
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
    /// The worker is removing the saved session.
    forgetting: bool,
    /// The request in progress renews a working session. The DM need not hear of it.
    refreshing: bool,
    /// Refreshes that failed in a row, and when the next one may start.
    refresh_failures: u32,
    retry_at: Option<Instant>,
    refresh_error: Option<String>,
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
    /// Still connected, because the token has not expired, but renewing it failed.
    RefreshFailing,
}

const SESSION_ENDED: &str = "The D&D Beyond session ended because it could not be renewed. Check your connection, then select Connect.";

/// 5, 10, 20, 40, then 60 seconds. Renewal starts a minute before the token ends at most,
/// so a short drop gets several tries.
fn refresh_backoff(failures: u32) -> Duration {
    Duration::from_secs((5_u64 << failures.saturating_sub(1).min(4)).min(60))
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
            forgetting: false,
            refreshing: false,
            refresh_failures: 0,
            retry_at: None,
            refresh_error: None,
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

    /// A session past its end, so that the next frame ends it by the real path.
    #[cfg(test)]
    pub fn simulate_session_expiry(&mut self) {
        self.simulated = None;
        self.refresh_error = None;
        self.session = Some(Session::for_test(Duration::ZERO, Duration::ZERO));
    }

    #[cfg(test)]
    pub fn simulate(&mut self, account: Account) {
        let session = || {
            Some(Session::for_test(
                Duration::from_secs(3600),
                Duration::from_secs(7200),
            ))
        };
        self.simulated = None;
        self.error = None;
        self.warning = None;
        self.refresh_error = None;
        match account {
            Account::Busy => self.simulated = Some(Account::Busy),
            Account::Connected => self.session = session(),
            Account::NotConnected => self.session = None,
            Account::Failed => {
                self.session = None;
                self.error =
                    Some("Could not reach D&D Beyond. Check your connection and try again.".into());
            }
            Account::KeyringWarning => {
                self.session = session();
                self.warning = Some(
                    "Connected, but the session could not be saved in the desktop keyring.".into(),
                );
            }
            Account::RefreshFailing => {
                self.session = session();
                self.refresh_error =
                    Some("Could not reach D&D Beyond. Check your connection and try again.".into());
            }
        }
        self.account_epoch += 1;
    }

    fn account(&self) -> Account {
        #[cfg(test)]
        if let Some(account) = self.simulated {
            return account;
        }
        if self.busy && !self.refreshing {
            Account::Busy
        } else if self.session.is_some() && self.refresh_error.is_some() {
            Account::RefreshFailing
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

    pub fn refresh_failing(&self) -> bool {
        self.account() == Account::RefreshFailing
    }

    pub fn connecting(&self) -> bool {
        #[cfg(test)]
        if self.simulated == Some(Account::Busy) {
            return true;
        }
        self.busy && !self.refreshing
    }

    pub fn campaign_session(&self) -> (u64, Option<Session>) {
        (self.account_epoch, self.session.clone())
    }

    fn send(&mut self, action: Action, ctx: &egui::Context) {
        self.restoring = matches!(&action, Action::Restore);
        self.forgetting = matches!(&action, Action::Forget);
        self.refreshing = matches!(&action, Action::Refresh(_));
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
                                self.saved_session = false;
                                self.refresh_failures = 0;
                                self.retry_at = None;
                                self.refresh_error = None;
                                // Refresh must not erase a failed-save warning.
                                if outcome.warning.is_some() {
                                    self.warning = outcome.warning;
                                }
                                self.error = None;
                            }
                            // A short network drop must not end sync at the table. The
                            // token still works, so try again while it lasts.
                            Err(error)
                                if self.refreshing
                                    && self.session.as_ref().is_some_and(|s| !s.expired()) =>
                            {
                                self.refresh_failures += 1;
                                self.retry_at =
                                    Some(Instant::now() + refresh_backoff(self.refresh_failures));
                                self.refresh_error = Some(error);
                            }
                            Err(error) => {
                                self.session = None;
                                self.error = Some(error);
                                // A failed removal says to select Disconnect again.
                                self.saved_session = self.restoring || self.forgetting;
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
        {
            if session.expired() {
                // Every retry failed. The cookie stays for one click on Connect.
                self.cookie_input = session.cookie().to_string();
                self.session = None;
                self.refresh_failures = 0;
                self.retry_at = None;
                self.refresh_error = None;
                self.error = Some(SESSION_ENDED.into());
            } else if session.needs_refresh() && self.retry_at.is_none_or(|at| Instant::now() >= at)
            {
                self.send(Action::Refresh(session.cookie()), ctx);
            }
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
        // The height of the Campaigns heading's row beside it, so the two sit on one line.
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Read.control());
            ui.label(Type::Heading.rich(ui, "D&D Beyond account").color(c.text));
        });
        let small = |ui: &mut egui::Ui, text: &str| {
            ui.add(egui::Label::new(Type::Small.rich(ui, text).color(c.muted)).wrap());
        };
        let account = self.account();
        let state = match account {
            Account::Busy => Status {
                icon: "",
                word: "Connecting to D&D Beyond",
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
            // The warning below says what is wrong, and this says only the connection.
            Account::KeyringWarning => Status {
                icon: icons::ONLINE,
                word: "Connected to D&D Beyond",
                color: c.connection.connected,
            },
            Account::RefreshFailing => Status {
                icon: icons::WARN,
                word: "Connected, but renewing the session failed. Retrying.",
                color: c.result.warning,
            },
        };
        ui.horizontal(|ui| {
            if account == Account::Busy {
                crate::design::spinner(ui);
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
            Account::RefreshFailing => {
                small(
                    ui,
                    "Splat keeps trying. Sync works until the session expires, within a minute.",
                );
            }
            _ => {}
        }
        if let Some(error) = &self.error {
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
            design::field(ui, "Session cookie", None, None, |ui| {
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
            if self.cookie_input.trim().is_empty() {
                small(ui, "Paste your session cookie to connect.");
            }
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
            && design::Action::secondary(if account == Account::Busy {
                "Cancel"
            } else {
                "Disconnect"
            })
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

    fn refreshing_settings(
        expires_in: Duration,
    ) -> (
        Settings,
        egui::Context,
        mpsc::Receiver<(u64, Action)>,
        mpsc::Sender<Reply>,
    ) {
        let ctx = egui::Context::default();
        let (worker, commands, results) = Worker::test_channels();
        let mut settings = Settings::default();
        settings.initialized = true;
        settings.worker = Some(worker);
        settings.session = Some(Session::for_test(Duration::ZERO, expires_in));
        (settings, ctx, commands, results)
    }

    #[test]
    fn a_failed_removal_keeps_disconnect_until_it_works() {
        let ctx = egui::Context::default();
        let (worker, commands, results) = Worker::test_channels();
        let mut settings = Settings::default();
        settings.initialized = true;
        settings.worker = Some(worker);
        settings.disconnect(&ctx);
        let (id, _) = commands.recv().unwrap();
        results
            .send(Reply {
                id,
                result: Err("The saved cookie could not be removed.".into()),
            })
            .unwrap();
        settings.poll(&ctx);
        assert!(settings.saved_session, "the message says to select it");
        settings.disconnect(&ctx);
        let (id, _) = commands.recv().unwrap();
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
        assert!(!settings.saved_session);
    }

    #[test]
    fn a_failed_refresh_keeps_the_session_and_retries_later() {
        let (mut settings, ctx, commands, results) = refreshing_settings(Duration::from_secs(600));
        settings.poll(&ctx);
        let (id, action) = commands.try_recv().expect("a refresh is due");
        assert!(matches!(action, Action::Refresh(_)));
        assert!(!settings.connecting(), "a refresh is not a new connection");
        assert_ne!(settings.account(), Account::Busy);
        results
            .send(Reply {
                id,
                result: Err("Could not reach D&D Beyond.".into()),
            })
            .unwrap();
        settings.poll(&ctx);
        assert!(
            settings.session.is_some(),
            "the session outlives one failure"
        );
        assert_eq!(settings.account(), Account::RefreshFailing);
        assert!(settings.error.is_none());
        assert!(commands.try_recv().is_err(), "it waits before trying again");
        settings.retry_at = Some(Instant::now());
        settings.poll(&ctx);
        let (id, action) = commands.try_recv().expect("the retry");
        assert!(matches!(action, Action::Refresh(_)));
        results
            .send(Reply {
                id,
                result: Ok(Outcome {
                    session: Some(Session::for_test(
                        Duration::from_secs(60),
                        Duration::from_secs(600),
                    )),
                    warning: None,
                }),
            })
            .unwrap();
        settings.poll(&ctx);
        assert_eq!(settings.account(), Account::Connected);
        assert_eq!(settings.refresh_failures, 0);
    }

    #[test]
    fn an_expired_session_ends_with_words_and_the_cookie_ready_to_retry() {
        let (mut settings, ctx, commands, _results) = refreshing_settings(Duration::ZERO);
        settings.poll(&ctx);
        assert!(commands.try_recv().is_err());
        assert!(settings.session.is_none());
        assert_eq!(settings.error.as_deref(), Some(SESSION_ENDED));
        assert_eq!(settings.cookie_input, "test-cookie");
    }

    #[test]
    fn the_refresh_wait_grows_and_stays_within_a_minute() {
        let waits: Vec<u64> = (1..=6).map(|n| refresh_backoff(n).as_secs()).collect();
        assert_eq!(waits, [5, 10, 20, 40, 60, 60]);
    }

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
