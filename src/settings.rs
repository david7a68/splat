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
    focus_cookie: bool,
    pub campaign_form_open: bool,
    sign_in_again: bool,
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

const SESSION_ENDED: &str = "Your sign-in ended. Select Connect to sign in again.";

/// 5, 10, 20, 40, then 60 seconds. Renewal starts a minute before the token ends at most,
/// so a short drop gets several tries.
fn refresh_backoff(failures: u32) -> Duration {
    Duration::from_secs((5_u64 << failures.saturating_sub(1).min(4)).min(60))
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            cookie_input: String::new(),
            focus_cookie: false,
            campaign_form_open: false,
            sign_in_again: false,
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
    pub(crate) fn focus_sign_in(&mut self) {
        self.focus_cookie = true;
        self.sign_in_again = true;
    }
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

    pub fn signed_out(&self) -> bool {
        self.error.as_deref() == Some(SESSION_ENDED)
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
                                    self.sign_in_again = false;
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

    pub fn connection_status(&self) -> Status<'static> {
        let c = colors();
        let (icon, word, color) = match self.account() {
            Account::NotConnected => (icons::OFFLINE, "Not connected", c.connection.offline),
            Account::Failed if self.error.as_deref() == Some(SESSION_ENDED) => (
                icons::OFFLINE,
                "Signed out of D&D Beyond",
                c.connection.offline,
            ),
            Account::Failed => (icons::ERROR, "Not connected", c.result.error),
            Account::Busy => (icons::BUSY, "Connecting", c.connection.busy),
            Account::RefreshFailing => (icons::WARN, "Sign-in failing", c.result.warning),
            Account::KeyringWarning => {
                (icons::WARN, "Connected · not remembered", c.result.warning)
            }
            Account::Connected => (icons::ONLINE, "Connected", c.connection.connected),
        };
        Status { icon, word, color }
    }

    pub fn show_compact(&mut self, ui: &mut egui::Ui) {
        let state = self.connection_status();
        let id = egui::Id::new("compact_account_open");
        let mut open = ui
            .ctx()
            .data(|data| data.get_temp::<bool>(id).unwrap_or(false));
        open |= self.focus_cookie
            || self.sign_in_again
            || ui.ctx().data(|data| {
                data.get_temp::<bool>(egui::Id::new("focus_cookie"))
                    .unwrap_or(false)
            });
        let heading_top = ui.cursor().top();
        ui.ctx().data_mut(|data| {
            data.insert_temp(egui::Id::new("account_heading_content_y"), heading_top)
        });
        design::group_heading(ui, "D&D Beyond account", 0);
        if open {
            self.show_account(ui, false);
        } else {
            ui.horizontal(|ui| {
                design::status(ui, state);
                if self.account() == Account::RefreshFailing {
                    ui.label(
                        Type::Small
                            .rich(ui, "Sync works for about one more minute.")
                            .color(colors().muted),
                    );
                }
                if design::Action::quiet(
                    if matches!(self.account(), Account::NotConnected | Account::Failed) {
                        "Connect…"
                    } else if self.account() == Account::RefreshFailing {
                        "Connect again…"
                    } else {
                        "Account…"
                    },
                )
                .show(ui)
                .clicked()
                {
                    open = true;
                }
            });
        }
        ui.ctx().data_mut(|data| data.insert_temp(id, open));
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.show_account(ui, true);
    }

    fn show_account(&mut self, ui: &mut egui::Ui, heading: bool) {
        let c = colors();
        if heading {
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), Density::Read.control()),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    design::group_heading(ui, "D&D Beyond account", 0);
                },
            );
        }
        let small = |ui: &mut egui::Ui, text: &str| {
            ui.scope(|ui| {
                ui.set_max_width(ui.available_width().min(design::READER_MEASURE));
                ui.add(egui::Label::new(Type::Small.rich(ui, text).color(c.muted)).wrap());
            });
        };
        self.sign_in_again |= ui.ctx().data(|data| {
            data.get_temp::<bool>(egui::Id::new("focus_cookie"))
                .unwrap_or(false)
        });
        let account = self.account();
        let state = self.connection_status();
        {
            ui.horizontal(|ui| {
                design::status(ui, state);
            });
        }
        match account {
            Account::Connected => {
                small(
                    ui,
                    "The access token refreshes automatically while Splat is open.",
                );
            }
            Account::RefreshFailing => {
                small(
                    ui,
                    "Splat keeps trying. Sync works for about one more minute.",
                );
            }
            _ => {}
        }
        if let Some(error) = &self.error {
            if error == SESSION_ENDED {
                small(ui, error);
            } else {
                design::message(ui, Tone::Error, error);
            }
        }
        if let Some(warning) = &self.warning {
            if account == Account::KeyringWarning {
                if design::message_with_action(
                    ui,
                    Tone::Warning,
                    "Your desktop keyring did not save the session. Unlock the keyring, then connect again.",
                    "Connect again",
                ) {
                    self.focus_sign_in();
                }
            } else {
                design::message(ui, Tone::Warning, warning);
            }
        }
        if account == Account::RefreshFailing
            && design::Action::secondary("Connect again")
                .show(ui)
                .clicked()
        {
            self.focus_sign_in();
        }
        if self.sign_in_again || matches!(account, Account::NotConnected | Account::Failed) {
            if self.error.as_deref() != Some(SESSION_ENDED) {
                small(
                    ui,
                    "Connect your account to import campaigns and read characters.",
                );
            }
            if design::link(ui, "Open D&D Beyond sign-in").clicked() {
                ui.ctx()
                    .open_url(egui::OpenUrl::new_tab("https://www.dndbeyond.com/sign-in"));
            }
            ui.scope_builder(egui::UiBuilder::new().id("account_cookie_help"), |ui| {
                design::section(ui, "How to get your session cookie", |ui| {
                    ui.label("1. Sign in to D&D Beyond in your browser.");
                    ui.label(
                        "2. Open developer tools (F12), then Application or Storage → Cookies.",
                    );
                    ui.label("3. Select dndbeyond.com and copy the value of CobaltSession.");
                    ui.label("4. Paste only the value below and select Connect.");
                });
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
                self.focus_cookie |= ui
                    .ctx()
                    .data_mut(|data| data.remove_temp::<bool>(egui::Id::new("focus_cookie")))
                    .unwrap_or(false);
                if std::mem::take(&mut self.focus_cookie) {
                    response.request_focus();
                    response.scroll_to_me(Some(egui::Align::Center));
                }
            });
            ui.checkbox(
                &mut self.remember,
                "Remember my D&D Beyond sign-in on this computer",
            );
            ui.horizontal(|ui| {
                let reason = self
                    .cookie_input
                    .trim()
                    .is_empty()
                    .then_some("Paste your session cookie first.");
                design::blocked_reason(ui, reason);
                let action = if reason.is_none()
                    && account != Account::Connected
                    && !self.campaign_form_open
                {
                    design::Action::primary("Connect")
                } else {
                    design::Action::secondary("Connect")
                };
                if action.blocked(reason).show(ui).clicked() {
                    self.sign_in_again = false;
                    let cookie = Zeroizing::new(self.cookie_input.trim().to_owned());
                    self.send(
                        crate::credentials::Action::Connect {
                            cookie,
                            remember: self.remember,
                        },
                        ui.ctx(),
                    );
                }
            });
            small(
                ui,
                "Splat sends your session cookie only to D&D Beyond. In the keyring, it can reconnect at startup. The access token stays in memory.",
            );
        }
        if matches!(
            account,
            Account::Connected | Account::KeyringWarning | Account::RefreshFailing
        ) {
            small(
                ui,
                "Disconnect removes Splat's saved D&D Beyond sign-in and clears local credentials. It does not sign your browser out of D&D Beyond.",
            );
        }
        // Only when there is something to forget. It follows the trust text.
        if (!matches!(account, Account::NotConnected | Account::Failed)
            || self.warning.is_some()
            || self.saved_session)
            && design::Action::quiet(if account == Account::Busy {
                "Cancel"
            } else {
                "Disconnect"
            })
            .line_start()
            .density(Density::Read)
            .show(ui)
            .clicked()
        {
            self.disconnect(ui.ctx());
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
