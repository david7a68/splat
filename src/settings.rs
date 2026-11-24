use crate::state_context::StateContext;
use crate::{
    auth::Session,
    credentials::{Action, Worker},
    design::{self, Status, colors, icons},
};
use std::{
    sync::mpsc,
    time::{Duration, Instant},
};
use zeroize::Zeroize;

pub struct Settings {
    cookie_input: String,
    focus_cookie: bool,
    /// The request in progress is the DM's Connect.
    connecting: bool,
    /// `error` is a failed Connect's cause (round 27 FA1): it sits in the reason slot beside
    /// Connect. Every other error (a failed removal, restore or worker) keeps its message in the
    /// Settings footer, with its recovery. Set with `error`, from the request that failed.
    connect_failed: bool,
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
    /// Round 28 FH6: the session in use is saved on this computer (a restored one, or a Connect
    /// with the box ticked whose save worked). It is set by the reply to a Restore or Connect
    /// and ends with Disconnect, a failed sign-in or a failed save; a refresh keeps it.
    remembered: bool,
    /// The request in progress saves the session it gets (a Restore, or a Connect with the box
    /// ticked).
    saves: bool,
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
            connecting: false,
            connect_failed: false,
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
            remembered: false,
            saves: false,
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
        self.remembered = false;
        match account {
            Account::Busy => self.simulated = Some(Account::Busy),
            Account::Connected => {
                self.session = session();
                self.remembered = true;
            }
            Account::NotConnected => self.session = None,
            Account::Failed => {
                self.session = None;
                self.connect_failed = true;
                self.error = Some("Could not reach D&D Beyond. Check your connection.".into());
            }
            Account::KeyringWarning => {
                self.session = session();
                self.warning = Some(
                    "Connected, but your sign-in could not be saved in the desktop keyring.".into(),
                );
            }
            Account::RefreshFailing => {
                self.session = session();
                self.refresh_error =
                    Some("Could not reach D&D Beyond. Check your connection.".into());
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

    fn send(&mut self, action: Action, ctx: &StateContext) {
        self.restoring = matches!(&action, Action::Restore);
        self.connecting = matches!(&action, Action::Connect { .. });
        self.forgetting = matches!(&action, Action::Forget);
        self.refreshing = matches!(&action, Action::Refresh(_));
        self.saves = matches!(
            &action,
            Action::Restore | Action::Connect { remember: true, .. }
        );
        if !matches!(&action, Action::Refresh(_)) {
            self.warning = None;
            self.account_epoch += 1;
        }
        self.generation += 1;
        self.error = None;
        self.connect_failed = false;
        self.busy = true;
        let worker = self.worker.get_or_insert_with(|| Worker::new(ctx.waker()));
        if let Err(error) = worker.send(self.generation, action) {
            self.error = Some(error);
            self.busy = false;
        }
    }

    pub fn poll(&mut self, ctx: &StateContext) {
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
                                if !self.refreshing {
                                    self.remembered = self.saves
                                        && outcome.session.is_some()
                                        && outcome.warning.is_none();
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
                                self.remembered = false;
                                self.error = Some(error);
                                self.connect_failed = self.connecting;
                                // A failed removal says to select Disconnect again.
                                self.saved_session = self.restoring || self.forgetting;
                            }
                        }
                    }
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => {
                        self.busy = false;
                        self.connect_failed = false;
                        self.error = Some("The sign-in worker stopped. Restart Splat.".into());
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
                self.connect_failed = false;
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

    fn disconnect(&mut self, ctx: &StateContext) {
        self.session = None;
        self.cookie_input.zeroize();
        self.warning = None;
        self.send(Action::Forget, ctx);
    }

    pub fn connection_status(&self) -> Status<'static> {
        Self::status_of((self.account(), self.signed_out()))
    }

    /// The account's status word for each state: the one function that names them, so the
    /// compact line's slot is as wide as the widest of them.
    fn status_of((account, signed_out): (Account, bool)) -> Status<'static> {
        let c = colors();
        let (icon, word, color) = match account {
            Account::NotConnected => (icons::OFFLINE, "Not connected", c.connection.offline),
            Account::Failed if signed_out => (
                icons::OFFLINE,
                "Signed out of D&D Beyond",
                c.connection.offline,
            ),
            // The words name the failure, as the error icon and colour do (round 24 FA6).
            Account::Failed => (icons::ERROR, "Connect failed", c.result.error),
            Account::Busy => (icons::BUSY, "Connecting", c.connection.busy),
            Account::RefreshFailing => (icons::WARN, "Sign-in failing", c.result.warning),
            Account::KeyringWarning => {
                (icons::WARN, "Connected · not remembered", c.result.warning)
            }
            Account::Connected => (icons::ONLINE, "Connected", c.connection.connected),
        };
        Status { icon, word, color }
    }

    /// The account's errors that are not a failed Connect (a failed removal, restore or worker),
    /// in the Settings footer, so their recovery stays in view whatever the cookie field holds.
    /// A failed Connect's cause sits beside Connect instead (round 27 FA1); an ended session is
    /// the account's state, said in the account column. Each lives until the next request.
    /// The message is typed data, so the pane's body can change the account while the footer
    /// waits for it (`Messages::pane`).
    pub fn footer_messages<A: 'static>(&self, messages: &mut design::outcome::Messages<'_, A>) {
        if let Some(error) = self
            .error
            .as_deref()
            .filter(|error| !self.connect_failed && *error != SESSION_ENDED)
        {
            messages.outcome(design::outcome::FaultMessage::new(
                &crate::campaigns::NotDone::words(error),
            ));
        }
    }
}

mod view;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credentials::{Outcome, Reply};

    fn refreshing_settings(
        expires_in: Duration,
    ) -> (
        Settings,
        StateContext,
        mpsc::Receiver<(u64, Action)>,
        mpsc::Sender<Reply>,
    ) {
        let ctx = StateContext::new();
        let (worker, commands, results) = Worker::test_channels();
        let mut settings = Settings::default();
        settings.initialized = true;
        settings.worker = Some(worker);
        settings.session = Some(Session::for_test(Duration::ZERO, expires_in));
        (settings, ctx, commands, results)
    }

    #[test]
    fn a_failed_removal_keeps_disconnect_until_it_works() {
        let ctx = StateContext::new();
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
        // Round 27 review: with the cookie field empty (a Disconnect clears it), the removal's
        // error stays in the footer, with Disconnect to try again. Only a failed Connect's cause
        // moves to the slot beside Connect.
        assert!(settings.cookie_input.is_empty());
        let mut messages = design::outcome::Messages::<design::outcome::NoPress>::new();
        settings.footer_messages(&mut messages);
        assert!(
            messages
                .into_plain()
                .iter()
                .any(|line| line.words == "The saved cookie could not be removed."),
            "the removal's error is in the footer"
        );
        assert!(settings.can_forget() && settings.connect_failure().is_none());
        // A failed Connect's cause is beside Connect, not in the footer.
        settings.cookie_input = "test-cookie".into();
        settings.connect(&ctx);
        let (id, _) = commands.recv().unwrap();
        results
            .send(Reply {
                id,
                result: Err("Could not reach D&D Beyond.".into()),
            })
            .unwrap();
        settings.poll(&ctx);
        assert_eq!(
            settings.connect_failure(),
            Some("Could not reach D&D Beyond.")
        );
        assert_eq!(
            settings.cookie(),
            "test-cookie",
            "the cookie stays for a retry"
        );
        let mut messages = design::outcome::Messages::<design::outcome::NoPress>::new();
        settings.footer_messages(&mut messages);
        assert!(messages.into_plain().is_empty());
        settings.disconnect(&ctx);
        let (id, _) = commands.recv().unwrap();
        results
            .send(Reply {
                id,
                result: Err("The saved cookie could not be removed.".into()),
            })
            .unwrap();
        settings.poll(&ctx);
        assert!(settings.saved_session && settings.connect_failure().is_none());
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
        let ctx = StateContext::new();
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
