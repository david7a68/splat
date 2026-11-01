use crate::auth::{self, Session};
use eframe::egui;
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};
use zeroize::Zeroizing;

trait Vault: Send {
    fn load(&self) -> Result<Option<Zeroizing<String>>, String>;
    fn save(&self, cookie: &str) -> Result<(), String>;
    fn forget(&self) -> Result<(), String>;
}

#[cfg(not(test))]
struct DesktopVault;

#[cfg(not(test))]
fn entry() -> Result<keyring::Entry, String> {
    keyring::Entry::new("splat", "dndbeyond-session")
        .map_err(|_| "The desktop keyring is unavailable.".into())
}

#[cfg(not(test))]
impl Vault for DesktopVault {
    fn load(&self) -> Result<Option<Zeroizing<String>>, String> {
        match entry()?.get_password() {
            Ok(cookie) => Ok(Some(Zeroizing::new(cookie))),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(_) => Err(
                "Could not read the desktop keyring. You can connect without saving your session."
                    .into(),
            ),
        }
    }

    fn save(&self, cookie: &str) -> Result<(), String> {
        entry()?.set_password(cookie).map_err(|_| {
            "Connected, but the session could not be saved in the desktop keyring.".into()
        })
    }

    fn forget(&self) -> Result<(), String> {
        match entry()?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(_) => Err("Local session cleared, but the saved cookie could not be removed. Unlock your keyring and select Disconnect again.".into()),
        }
    }
}

// Test builds have no route to the desktop keyring: nothing to load, nothing saved.
#[cfg(test)]
struct DesktopVault;

#[cfg(test)]
impl Vault for DesktopVault {
    fn load(&self) -> Result<Option<Zeroizing<String>>, String> {
        Ok(None)
    }

    fn save(&self, _: &str) -> Result<(), String> {
        Err("Connected, but the session could not be saved in the desktop keyring.".into())
    }

    fn forget(&self) -> Result<(), String> {
        Ok(())
    }
}

pub enum Action {
    Restore,
    Connect {
        cookie: Zeroizing<String>,
        remember: bool,
    },
    Refresh(Zeroizing<String>),
    Forget,
}

pub struct Outcome {
    pub session: Option<Session>,
    pub warning: Option<String>,
}

pub struct Reply {
    pub id: u64,
    pub result: Result<Outcome, String>,
}

pub struct Worker {
    tx: Sender<(u64, Action)>,
    pub rx: Receiver<Reply>,
}

fn perform(vault: &dyn Vault, action: Action) -> Result<Outcome, String> {
    match action {
        Action::Restore => match vault.load()? {
            Some(cookie) => Ok(Outcome {
                session: Some(auth::connect(cookie)?),
                warning: None,
            }),
            None => Ok(Outcome {
                session: None,
                warning: None,
            }),
        },
        Action::Connect { cookie, remember } => {
            let session = auth::connect(cookie)?;
            let warning = if remember {
                vault.save(&session.cookie()).err()
            } else {
                // Remove any older saved account when choosing a temporary session.
                vault.forget().err()
            };
            Ok(Outcome {
                session: Some(session),
                warning,
            })
        }
        Action::Refresh(cookie) => Ok(Outcome {
            session: Some(auth::connect(cookie)?),
            warning: None,
        }),
        Action::Forget => Ok(Outcome {
            session: None,
            warning: vault.forget().err(),
        }),
    }
}

impl Worker {
    #[cfg(test)]
    pub fn test_channels() -> (Self, Receiver<(u64, Action)>, Sender<Reply>) {
        let (tx, commands) = mpsc::channel();
        let (results, rx) = mpsc::channel();
        (Self { tx, rx }, commands, results)
    }

    pub fn new(ctx: egui::Context) -> Self {
        let (tx, commands) = mpsc::channel();
        let (results, rx) = mpsc::channel();
        // One worker serializes keyring writes and deletes. A late connect can
        // never save credentials after a queued disconnect has removed them.
        thread::spawn(move || {
            while let Ok((id, action)) = commands.recv() {
                let result = perform(&DesktopVault, action);
                // Drain queued operations even if the window closed, so a queued
                // Forget still runs after an in-flight Connect.
                let _ = results.send(Reply { id, result });
                ctx.request_repaint();
            }
        });
        Self { tx, rx }
    }

    pub fn send(&self, id: u64, action: Action) -> Result<(), String> {
        self.tx
            .send((id, action))
            .map_err(|_| "The session worker stopped. Restart Splat to reconnect.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct FakeVault(Mutex<Option<String>>);
    impl Vault for FakeVault {
        fn load(&self) -> Result<Option<Zeroizing<String>>, String> {
            Ok(self.0.lock().unwrap().clone().map(Zeroizing::new))
        }
        fn save(&self, cookie: &str) -> Result<(), String> {
            *self.0.lock().unwrap() = Some(cookie.into());
            Ok(())
        }
        fn forget(&self) -> Result<(), String> {
            *self.0.lock().unwrap() = None;
            Ok(())
        }
    }

    #[test]
    fn missing_session_restores_without_network_and_disconnect_removes_saved_cookie() {
        let vault = FakeVault(Mutex::new(None));
        assert!(perform(&vault, Action::Restore).unwrap().session.is_none());
        vault.save("test-cookie").unwrap();
        assert!(perform(&vault, Action::Forget).unwrap().warning.is_none());
        assert!(vault.load().unwrap().is_none());
    }

    #[test]
    fn delete_failure_is_reported() {
        struct LockedVault;
        impl Vault for LockedVault {
            fn load(&self) -> Result<Option<Zeroizing<String>>, String> {
                Err("locked".into())
            }
            fn save(&self, _: &str) -> Result<(), String> {
                Err("locked".into())
            }
            fn forget(&self) -> Result<(), String> {
                Err("locked".into())
            }
        }
        assert!(perform(&LockedVault, Action::Restore).is_err());
        assert_eq!(
            perform(&LockedVault, Action::Forget)
                .unwrap()
                .warning
                .as_deref(),
            Some("locked")
        );
    }
}
