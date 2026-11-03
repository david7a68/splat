//! The app's routes to per-user state and to values that differ between runs.
//!
//! Test builds compile a closed variant of each route, so a test can never reach
//! the user's character cache, a native file dialog, or real time and randomness.
//! The settings store, keyring and sign-in are closed the same way where they live
//! (`persistence::Store::user`, `credentials::Vault`, `auth::connect`). Test builds can
//! make a `Session`, to show the signed-in state, but each route that would use one
//! returns first (`dndbeyond::fetch`, `party_sync::refresh`, `party_stream::Stream::start`,
//! `combat_sync::CombatSync::tick`). So no D&D Beyond request is ever sent.
use splat::character::Cache;
use std::path::PathBuf;
use uuid::Uuid;

#[cfg(not(test))]
pub fn character_cache() -> Result<Cache, String> {
    Cache::user()
}

#[cfg(not(test))]
pub fn pick_folder() -> Option<PathBuf> {
    rfd::FileDialog::new().pick_folder()
}

#[cfg(not(test))]
pub fn new_uuid() -> Uuid {
    Uuid::new_v4()
}

#[cfg(not(test))]
pub fn now() -> u64 {
    splat::character::now()
}

/// A point on the clock that times a word on screen, or sync's next step.
#[cfg(not(test))]
pub type Moment = std::time::Instant;

#[cfg(not(test))]
pub fn moment() -> Moment {
    Moment::now()
}

/// How long ago this moment was.
#[cfg(not(test))]
pub fn since(at: Moment) -> std::time::Duration {
    at.elapsed()
}

/// When a file last changed, in seconds since the epoch.
#[cfg(not(test))]
pub fn file_time(path: &std::path::Path) -> Option<u64> {
    std::fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|elapsed| elapsed.as_secs())
}

/// Local time, by the system's zone. A session's date is the DM's date, not UTC's.
#[cfg(not(test))]
fn local_time() -> libc::tm {
    let seconds = now() as libc::time_t;
    // SAFETY: both pointers are valid for this call, and `tm` is plain data.
    unsafe {
        let mut time: libc::tm = std::mem::zeroed();
        libc::localtime_r(&seconds, &mut time);
        time
    }
}

/// Today's local date, as `2026-11-01`.
#[cfg(not(test))]
pub fn today() -> String {
    let time = local_time();
    format!(
        "{:04}-{:02}-{:02}",
        time.tm_year + 1900,
        time.tm_mon + 1,
        time.tm_mday
    )
}

/// The local date and time, for a file's name.
#[cfg(not(test))]
pub fn local_stamp() -> String {
    let time = local_time();
    format!("{} {:02}-{:02}", today(), time.tm_hour, time.tm_min)
}

#[cfg(test)]
pub use closed::{
    Moment, character_cache, file_time, local_stamp, moment, new_uuid, now, pick_folder, since,
    today,
};

#[cfg(test)]
pub mod closed {
    use super::*;
    use std::{cell::Cell, sync::OnceLock, time::Duration};

    /// A fixed clock, so "Fetched 3 h ago" renders the same on every run.
    pub const FIXED_NOW: u64 = 1_767_236_400;

    static CACHE_ROOT: OnceLock<PathBuf> = OnceLock::new();

    thread_local! {
        static NEXT_UUID: Cell<Option<u128>> = const { Cell::new(None) };
        static CLOCK: Cell<Duration> = const { Cell::new(Duration::ZERO) };
    }

    /// Time on a clock that moves only when a test says so, so that a word on a timer is on
    /// screen in the same captures of every run.
    pub type Moment = Duration;

    pub fn moment() -> Moment {
        CLOCK.get()
    }

    pub fn since(at: Moment) -> Duration {
        CLOCK.get().saturating_sub(at)
    }

    pub fn pass_time(time: Duration) {
        CLOCK.set(CLOCK.get() + time);
    }

    /// Open the character cache at a temporary folder for this test process.
    /// Without this call, every character cache request fails.
    pub fn use_cache_root(root: PathBuf) {
        assert!(root.starts_with(std::env::temp_dir()));
        assert_eq!(CACHE_ROOT.get_or_init(|| root.clone()), &root);
    }

    /// Make `new_uuid` repeatable on this thread. Other tests keep random IDs.
    pub fn repeatable_uuids(start: u128) {
        NEXT_UUID.with(|next| next.set(Some(start)));
    }

    pub fn character_cache() -> Result<Cache, String> {
        CACHE_ROOT
            .get()
            .cloned()
            .map(Cache::at)
            .ok_or_else(|| "Test builds have no character cache.".to_owned())
    }

    pub fn pick_folder() -> Option<PathBuf> {
        None
    }

    pub fn new_uuid() -> Uuid {
        NEXT_UUID.with(|next| match next.get() {
            Some(value) => {
                next.set(Some(value + 1));
                Uuid::from_u128(value)
            }
            None => Uuid::new_v4(),
        })
    }

    pub fn now() -> u64 {
        FIXED_NOW
    }

    pub fn today() -> String {
        crate::encounter_run::calendar_date(FIXED_NOW)
    }

    pub fn local_stamp() -> String {
        format!("{} 19-00", today())
    }

    /// The fixed clock, for a file too: a real one would change every capture of its date.
    pub fn file_time(_: &std::path::Path) -> Option<u64> {
        Some(FIXED_NOW)
    }
}
