//! The account's reads and acts, free of any toolkit, so the egui account column
//! (`Settings::show_account`) and the GPUI Settings page (`src/gui/pages/settings/`) share them.
use super::{Account, SESSION_ENDED, Settings};
use crate::state_context::StateContext;
use zeroize::Zeroizing;

impl Settings {
    /// The account's state now.
    pub fn account_now(&self) -> Account {
        self.account()
    }

    /// The session in use is saved on this computer.
    pub fn remembered(&self) -> bool {
        self.remembered
    }

    /// The sign-in ended, and its words say so (the account's state, in the account column).
    pub fn session_ended(&self) -> Option<&'static str> {
        (self.error.as_deref() == Some(SESSION_ENDED)).then_some(SESSION_ENDED)
    }

    /// A failed Connect's cause, beside Connect.
    pub fn connect_failure(&self) -> Option<&str> {
        self.error.as_deref().filter(|_| self.connect_failed)
    }

    /// The keyring's warning, while it stands.
    pub fn warning(&self) -> Option<&str> {
        self.warning.as_deref()
    }

    /// The DM asked to sign in again (the sign-in form stays open while connected).
    pub fn signing_in_again(&self) -> bool {
        self.sign_in_again
    }

    /// The sign-in form shows: asked for again, or no working account.
    pub fn sign_in_open(&self) -> bool {
        self.sign_in_again || matches!(self.account(), Account::NotConnected | Account::Failed)
    }

    /// `Disconnect` (or `Cancel` while busy) shows: there is something to forget.
    pub fn can_forget(&self) -> bool {
        !matches!(self.account(), Account::NotConnected | Account::Failed)
            || self.warning.is_some()
            || self.saved_session
    }

    /// The pasted cookie, as typed.
    pub fn cookie(&self) -> &str {
        &self.cookie_input
    }

    pub fn set_cookie(&mut self, words: &str) {
        use zeroize::Zeroize as _;
        self.cookie_input.zeroize();
        self.cookie_input = words.to_owned();
    }

    /// The cookie field asked for the keyboard (once).
    pub fn take_focus_cookie(&mut self) -> bool {
        std::mem::take(&mut self.focus_cookie)
    }

    /// `Remember my D&D Beyond sign-in on this computer`.
    pub fn remember(&self) -> bool {
        self.remember
    }

    pub fn set_remember(&mut self, remember: bool) {
        self.remember = remember;
    }

    /// Why Connect waits: the cookie field is empty.
    pub fn connect_reason(&self) -> Option<&'static str> {
        self.cookie_input
            .trim()
            .is_empty()
            .then_some("Paste your sign-in cookie first.")
    }

    /// `Connect` with the pasted cookie.
    pub fn connect(&mut self, ctx: &StateContext) {
        self.sign_in_again = false;
        let cookie = Zeroizing::new(self.cookie_input.trim().to_owned());
        self.send(
            crate::credentials::Action::Connect {
                cookie,
                remember: self.remember,
            },
            ctx,
        );
    }

    /// `Disconnect`, or `Cancel` while busy.
    pub fn forget(&mut self, ctx: &StateContext) {
        self.disconnect(ctx);
    }
}
