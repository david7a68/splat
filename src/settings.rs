use crate::{
    auth::Session,
    credentials::{Action, Worker},
    design::{self, Density, Status, Type, colors, icons},
};
use eframe::egui;
use std::{
    sync::mpsc,
    time::{Duration, Instant},
};
use zeroize::Zeroize;

/// The account's warning, a typed notice in the account column: a sign-in the desktop keyring
/// did not save asks to connect again (true when the DM chooses it); any other is the keyring's
/// own words.
#[allow(
    clippy::disallowed_methods,
    reason = "a body place: the account’s warning stands in the account column, in the keyring’s words (Ban::Body, Ban::Words)"
)]
fn account_warning(ui: &mut egui::Ui, keyring: bool, warning: &str) -> bool {
    use design::outcome::{BodyMessage, Choice, MessageChoice, Notice, Sentence};
    let notice = if keyring {
        Notice::warning(crate::sentence!(
            "This computer did not save your sign-in. Unlock your password manager, then connect again."
        ))
        .choice(Choice::new(MessageChoice::Choice("Connect again"), ()))
    } else if let Some(words) = Sentence::outside(warning) {
        Notice::warning(words)
    } else {
        return false;
    };
    BodyMessage::from(notice).show(ui).is_some()
}

pub struct Settings {
    cookie_input: String,
    focus_cookie: bool,
    /// The request in progress is the DM's Connect.
    connecting: bool,
    /// `error` is a failed Connect's cause (round 27 FA1): it sits in the reason slot beside
    /// Connect. Every other error (a failed removal, restore or worker) keeps its message in the
    /// Settings footer, with its recovery. Set with `error`, from the request that failed.
    connect_failed: bool,
    /// Connect's press, kept until its reply (issue #698, M0279; `Reveal::ask_on_result`): a
    /// failed Connect brings its kept cookie, its cause and Connect whole into view together;
    /// any other reply, or a new request, ends it.
    reveal: design::reveal::Reveal,
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

impl Account {
    const ALL: [Self; 6] = [
        Self::Busy,
        Self::Connected,
        Self::NotConnected,
        Self::Failed,
        Self::KeyringWarning,
        Self::RefreshFailing,
    ];
}

/// The cookie field keeps its id when Settings changes between one and two columns, so a DM who
/// is pasting keeps the focus and the cursor.
const SESSION_COOKIE: design::FieldId =
    crate::field_id!("session_cookie").once(design::Once::Settings);

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
            reveal: design::reveal::Reveal::default(),
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

    /// An account error that is not a failed Connect (a failed removal, restore or worker), so
    /// that it stands in the account's footer, as `request` sets it; `None` ends it.
    #[cfg(test)]
    pub fn simulate_error(&mut self, error: Option<&str>) {
        self.error = error.map(str::to_owned);
        self.connect_failed = false;
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
        self.connecting = matches!(&action, Action::Connect { .. });
        // A new request ends a Connect's awaited reveal; Connect's press asks again after this.
        self.reveal.end();
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
                                self.reveal.end();
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
                                if self.connecting {
                                    self.reveal.result_came(ctx);
                                } else {
                                    self.reveal.end();
                                }
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

    fn disconnect(&mut self, ctx: &egui::Context) {
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

    pub fn show_compact(&mut self, ui: &mut egui::Ui) {
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
        design::heading(
            ui,
            design::Heading::Group {
                title: "D&D Beyond account",
                count: 0,
            },
        );
        // Round 28 FO3: a connected account opens its sign-in and Disconnect as a caret section,
        // as the other inline forms on this page open (`Add campaign`).
        let details = !open
            && !matches!(
                self.account(),
                Account::NotConnected | Account::Failed | Account::RefreshFailing
            );
        if open {
            self.show_account(ui, false, true);
        } else {
            ui.horizontal(|ui| {
                // A fixed slot: the action after the status stays in place when the state
                // changes (family F8).
                let every: Vec<_> = Account::ALL
                    .into_iter()
                    .flat_map(|account| [(account, false), (account, true)])
                    .collect();
                design::status_slot(
                    ui,
                    &every,
                    (self.account(), self.signed_out()),
                    Self::status_of,
                );
                if self.account() == Account::RefreshFailing {
                    ui.label(
                        Type::Small
                            .rich(ui, "Sync works for about one more minute.")
                            .color(colors().muted),
                    );
                }
                if !details
                    && design::Action::quiet(
                        if matches!(self.account(), Account::NotConnected | Account::Failed) {
                            "Connect…"
                        } else {
                            "Connect again…"
                        },
                    )
                    .show(ui)
                    .clicked()
                {
                    open = true;
                }
            });
            if details {
                // The section says whether it is open (expanded) to a screen reader.
                design::section_with_id(ui, "compact_sign_in_details", "Sign-in details", |ui| {
                    self.show_account(ui, false, false)
                });
            }
        }
        ui.ctx().data_mut(|data| data.insert_temp(id, open));
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.show_account(ui, true, true);
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

    /// `status`: draw the account's status line (a caret section under the compact status line
    /// does not say it again).
    fn show_account(&mut self, ui: &mut egui::Ui, heading: bool, status: bool) {
        if heading {
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), Density::Read.control()),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    design::heading(
                        ui,
                        design::Heading::Group {
                            title: "D&D Beyond account",
                            count: 0,
                        },
                    );
                },
            );
        }
        let small = |ui: &mut egui::Ui, text: &str| design::help(ui, text);
        self.sign_in_again |= ui.ctx().data(|data| {
            data.get_temp::<bool>(egui::Id::new("focus_cookie"))
                .unwrap_or(false)
        });
        let account = self.account();
        let state = self.connection_status();
        if status {
            ui.horizontal(|ui| {
                design::status(ui, state);
            });
        }
        match account {
            // Round 28 FH6: the help says whether the sign-in was saved.
            Account::Connected if self.remembered => {
                small(ui, "Splat signs in again when it starts.");
            }
            Account::Connected => {
                small(ui, "Splat stays signed in while it is open.");
            }
            Account::RefreshFailing => {
                small(
                    ui,
                    "Splat keeps trying. Sync works for about one more minute.",
                );
            }
            _ => {}
        }
        // Round 27 FA1: a failed Connect's cause sits in the reason slot left of Connect, where
        // the DM pressed (it was in the page's footer, far from Connect); other errors stay in
        // the footer (`footer_messages`). An ended session is the account's state, so its help
        // stays here.
        if let Some(error) = self
            .error
            .as_deref()
            .filter(|error| *error == SESSION_ENDED)
        {
            small(ui, error);
        }
        if let Some(warning) = &self.warning
            && account_warning(ui, account == Account::KeyringWarning, warning)
        {
            self.focus_sign_in();
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
                design::section(ui, "How to get your sign-in cookie", |ui| {
                    ui.label("1. Sign in to D&D Beyond in your browser.");
                    ui.label(
                        "2. Open developer tools (F12), then Application or Storage → Cookies.",
                    );
                    let mut instruction = egui::text::LayoutJob::default();
                    for (words, style) in [
                        ("3. Select dndbeyond.com and copy the value of ", Type::Body),
                        ("CobaltSession", Type::Mono),
                        (".", Type::Body),
                    ] {
                        instruction.append(
                            words,
                            0.0,
                            egui::TextFormat {
                                font_id: style.font(ui),
                                color: colors().text,
                                ..Default::default()
                            },
                        );
                    }
                    ui.label(instruction);
                    ui.label("4. Paste only the value below and select Connect.");
                });
            });
            let mut cookie_rect = None;
            let cookie_width = design::small_field(ui, "Sign-in cookie", None, None, |ui| {
                let hint = design::placeholder(ui, "Paste sign-in cookie");
                let width = 420.0_f32.min(ui.available_width() - 2.0 * design::S);
                let response = SESSION_COOKIE.single_line(ui, &mut self.cookie_input, |edit| {
                    edit.password(true)
                        .hint_text(hint)
                        .min_size(egui::vec2(0.0, Density::Read.control()))
                        .margin(egui::vec2(design::S, 5.0))
                        .desired_width(width)
                });
                design::named(ui, &response, "Paste sign-in cookie");
                self.focus_cookie |= ui
                    .ctx()
                    .data_mut(|data| data.remove_temp::<bool>(egui::Id::new("focus_cookie")))
                    .unwrap_or(false);
                // M0541 (M-P0.31): the field takes the keyboard; the pane's focus arbiter
                // brings it whole into view above the footer, by the least scroll, also when the
                // request came from another page (`focus_sign_in`).
                if std::mem::take(&mut self.focus_cookie) {
                    response.request_focus();
                }
                cookie_rect = Some(response.rect);
                response.rect.width()
            });
            ui.checkbox(
                &mut self.remember,
                "Remember my D&D Beyond sign-in on this computer",
            );
            let row = ui
                .horizontal(|ui| {
                    ui.set_max_width(cookie_width);
                    let reason = self.connect_reason();
                    // Round 27 FA1: the reason slot left of Connect holds a failed Connect's cause
                    // while Connect is open. It lives until the next Connect, Disconnect or a working
                    // sign-in, as before; while the cookie field is empty, the slot names that step.
                    // Only a Connect's failure goes here (`connect_failed`); the others keep their
                    // footer message (`footer_messages`).
                    let failure = self
                        .error
                        .as_deref()
                        .filter(|_| self.connect_failed && reason.is_none());
                    let action = if reason.is_none()
                        && account != Account::Connected
                        && !self.campaign_form_open
                    {
                        design::Action::primary("Connect")
                    } else {
                        design::Action::secondary("Connect")
                    };
                    let (clicked, act) = ui
                        .with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let connect = action.blocked(reason).show(ui);
                            let act = design::reveal::Act::clicked(ui, &connect);
                            ui.spacing_mut().item_spacing.x = design::S;
                            // Round 28 FA11: a reason that fits on one line sits `S` left of
                            // Connect; a longer one starts at the field's left edge and wraps from
                            // there, so no line is left with one word. Measured, not a fixed width.
                            let shown = reason.or(failure);
                            let fits = shown.is_none_or(|words| {
                                design::number_width(ui, words, &Type::Small.font(ui))
                                    <= ui.available_width()
                            });
                            if fits {
                                design::blocked_reason(ui, shown);
                            } else {
                                ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::Center)
                                        .with_main_wrap(true),
                                    |ui| design::blocked_reason(ui, shown),
                                );
                            }
                            (connect.clicked(), act)
                        })
                        .inner;
                    if clicked {
                        self.connect(ui.ctx());
                        // M0279 (M-P0.32): the reply comes later. A failure shows the kept cookie,
                        // the cause beside Connect and Connect together, by the least scroll; the
                        // cause can wrap, so the target is the field to the row's end.
                        if let Some(act) = act {
                            self.reveal.ask_on_result(act, self.generation, "connect");
                        }
                    }
                })
                .response
                .rect;
            if let Some(cookie) = cookie_rect {
                self.reveal
                    .mark(ui, self.generation, "connect", cookie.union(row));
            }
            small(
                ui,
                "Splat sends your sign-in cookie only to D&D Beyond. With the box ticked, Splat signs in again when it starts.",
            );
        }
        if matches!(
            account,
            Account::Connected | Account::KeyringWarning | Account::RefreshFailing
        ) {
            small(
                ui,
                if self.remembered {
                    "Disconnect removes Splat’s saved D&D Beyond sign-in. It does not sign your browser out of D&D Beyond."
                } else {
                    "Disconnect signs Splat out. It does not sign your browser out of D&D Beyond."
                },
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

mod view;

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

    /// Round 28 FO3: in the compact layout a connected account opens `Sign-in details` as a
    /// caret section, by keyboard too; its expanded state is in the accessible tree, and it
    /// holds Disconnect.
    #[test]
    fn sign_in_details_is_a_caret_section_that_opens_by_keyboard() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::{NodeT, Queryable};
        let mut settings = Settings::default();
        settings.simulate(Account::Connected);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(600.0, 600.0))
            .themed_ui_state(
                |ui, settings: &mut Settings| settings.show_compact(ui),
                settings,
            );
        harness.run_steps(3);
        let expanded = |harness: &egui_kittest::Harness<'_, Settings>| {
            harness
                .get_by_label("Sign-in details")
                .accesskit_node()
                .data()
                .is_expanded()
        };
        assert_eq!(expanded(&harness), Some(false));
        assert!(harness.query_all_by_label("Disconnect").next().is_none());
        harness.get_by_label("Sign-in details").focus();
        harness.run_steps(2);
        harness.key_press(egui::Key::Enter);
        harness.run_steps(4);
        assert_eq!(expanded(&harness), Some(true), "Enter opens the section");
        assert!(
            harness.query_all_by_label("Disconnect").next().is_some(),
            "the open section holds Disconnect"
        );
        assert_eq!(
            harness.query_all_by_label_contains("Connected").count(),
            1,
            "the status shows once"
        );
    }

    /// The keyring warning names the sign-in and offers Connect again. M0541 (M-P0.31,
    /// `m-p0-sign-in-focus`): drawn in a pane under 120 px of other content in a 300 px window,
    /// Connect again gives the cookie field the keyboard, and the pane's focus arbiter brings it
    /// whole into view by the least scroll: its bottom near the view's bottom, not at its centre.
    #[test]
    fn a_keyring_warning_names_the_sign_in() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        let mut settings = Settings::default();
        settings.simulate(Account::KeyringWarning);
        const SIZE: egui::Vec2 = egui::vec2(600.0, 300.0);
        let mut harness = egui_kittest::Harness::builder()
            .with_size(SIZE)
            .themed_ui_state(
                |ui, settings: &mut Settings| {
                    let footer = design::Footer::new("keyring_test", SIZE.x);
                    footer.pane_content(ui, design::pane::ContentScroll::default(), |ui| {
                        ui.allocate_exact_size(egui::vec2(1.0, 120.0), egui::Sense::hover());
                        settings.show_account(ui, true, true);
                    });
                },
                settings,
            );
        harness.run();
        harness.get_by_label(
            "This computer did not save your sign-in. Unlock your password manager, then connect again.",
        );
        let link = harness.get_by_label("Connect again").rect();
        assert!(
            link.bottom() <= SIZE.y,
            "Connect again is in view: {link:?}"
        );
        harness.get_by_label("Connect again").click();
        harness.run_steps(6);
        let field = harness.get_by_label("Paste sign-in cookie").rect();
        assert!(
            harness.ctx.memory(|memory| memory.focused()).is_some(),
            "m-p0-sign-in-focus: the field has the keyboard"
        );
        assert!(
            field.top() >= 0.0 && field.bottom() <= SIZE.y && field.bottom() > 0.6 * SIZE.y,
            "m-p0-sign-in-focus: the field is whole, at the view's lower part: {field:?} in {SIZE:?}"
        );
    }

    #[test]
    fn a_failed_removal_keeps_disconnect_until_it_works() {
        use crate::design::test_harness::Themed as _;
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
        // Round 23 SF1: only a failed Connect scrolls its retry into view.
        assert!(
            !settings.reveal.running(),
            "a failed removal is not a retry"
        );
        // Round 27 review: drawn as the Settings tab draws it, with the cookie field empty (a
        // Disconnect clears it), the removal's error stays in view with Disconnect to try
        // again. Only a failed Connect's cause moves to the slot beside Connect.
        assert!(settings.cookie_input.is_empty());
        use egui_kittest::kittest::Queryable;
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(1200.0, 800.0))
            .themed_ui_state(
                |ui, settings: &mut Settings| {
                    let frame = design::outcome::PaneFrame::new(ui);
                    let mut messages = design::outcome::Messages::<design::outcome::NoPress>::new(
                        &frame,
                        "settings_footer",
                        600.0,
                    );
                    settings.footer_messages(&mut messages);
                    let _ = messages.pane(ui, design::Body::Content, |ui| settings.show(ui));
                },
                std::mem::take(&mut settings),
            );
        harness.run_steps(3);
        assert!(
            harness
                .query_all_by_label("The saved cookie could not be removed.")
                .next()
                .is_some(),
            "the removal's error is drawn"
        );
        assert!(harness.query_all_by_label("Disconnect").next().is_some());
        settings = std::mem::take(harness.state_mut());
        drop(harness);
        // Issue #698 (M0279, `m-p0-connect-reveal`): drawn as the Settings tab draws it, in a
        // pane whose column hosts reveals, under 200 px of other content, in a window too short
        // for the whole form. Connect's
        // press waits for its reply while the form gives way to the busy state; the failure then
        // brings the kept cookie and Connect whole into view together.
        settings.cookie_input = "test-cookie".into();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(600.0, 600.0))
            .themed_ui_state(
                |ui, settings: &mut Settings| {
                    settings.poll(ui.ctx());
                    let footer = design::Footer::new("connect_test", 600.0);
                    footer.pane_content(
                        ui,
                        design::pane::ContentScroll::default().hosts_reveals(),
                        |ui| {
                            // Other content above the account, as the campaigns are.
                            ui.allocate_exact_size(egui::vec2(1.0, 200.0), egui::Sense::hover());
                            settings.show_account(ui, true, true);
                        },
                    );
                },
                std::mem::take(&mut settings),
            );
        harness.run_steps(3);
        let connect = |harness: &egui_kittest::Harness<'_, Settings>| {
            harness
                .get_by_role_and_label(egui::accesskit::Role::Button, "Connect")
                .rect()
        };
        harness
            .get_by_role_and_label(egui::accesskit::Role::Button, "Connect")
            .click();
        harness.run_steps(2);
        assert!(
            harness.state().reveal.waits(),
            "Connect's press waits for its reply"
        );
        const SHORT: f32 = 300.0;
        harness.set_size(egui::vec2(600.0, SHORT));
        harness.run_steps(4);
        let (id, _) = commands.recv().unwrap();
        results
            .send(Reply {
                id,
                result: Err("Could not reach D&D Beyond.".into()),
            })
            .unwrap();
        harness.run_steps(8);
        let whole = |rect: egui::Rect| rect.top() >= 0.0 && rect.bottom() <= SHORT;
        let cookie = harness.get_by_label("Paste sign-in cookie").rect();
        assert!(
            harness.state().reveal.ended() == Some(design::reveal::End::Whole)
                && whole(cookie)
                && whole(connect(&harness)),
            "m-p0-connect-reveal: a failed Connect shows its retry: cookie {cookie:?}, Connect {:?}, {:?}",
            connect(&harness),
            harness.state().reveal.ended()
        );
        // A new request ends a waiting reveal, before its reply.
        harness
            .get_by_role_and_label(egui::accesskit::Role::Button, "Connect")
            .click();
        harness.run_steps(2);
        assert!(harness.state().reveal.waits());
        let _ = commands.recv().unwrap();
        settings = std::mem::take(harness.state_mut());
        drop(harness);
        settings.disconnect(&ctx);
        assert!(!settings.reveal.running());
        let (id, _) = commands.recv().unwrap();
        results
            .send(Reply {
                id,
                result: Err("The saved cookie could not be removed.".into()),
            })
            .unwrap();
        settings.poll(&ctx);
        assert!(
            !settings.reveal.running(),
            "a failed removal is not a retry"
        );
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
