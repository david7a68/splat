//! The app's routes to per-user state and to values that differ between runs.
//!
//! Test builds compile a closed variant of each route, so a test can never reach
//! the user's character cache, a native file dialog, or real time and randomness.
//! The settings store, keyring and sign-in are closed the same way where they live
//! (`persistence::Store::user`, `credentials::Vault`, `auth::connect`). Every D&D
//! Beyond request needs a `Session`, and only `auth::connect` creates one.
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

#[cfg(test)]
pub use closed::{character_cache, new_uuid, now, pick_folder};

#[cfg(test)]
pub mod closed {
    use super::*;
    use std::{cell::Cell, sync::OnceLock};

    /// A fixed clock, so "Fetched 3 h ago" renders the same on every run.
    pub const FIXED_NOW: u64 = 1_767_236_400;

    static CACHE_ROOT: OnceLock<PathBuf> = OnceLock::new();

    thread_local! {
        static NEXT_UUID: Cell<Option<u128>> = const { Cell::new(None) };
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
}
