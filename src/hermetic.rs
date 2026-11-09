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

pub fn pick_folder_from(parent: Option<&std::path::Path>) -> Option<PathBuf> {
    #[cfg(not(test))]
    {
        let dialog = rfd::FileDialog::new();
        parent
            .map_or(dialog.clone(), |parent| dialog.set_directory(parent))
            .pick_folder()
    }
    #[cfg(test)]
    {
        let _ = parent;
        pick_folder()
    }
}

#[cfg(not(test))]
pub fn new_uuid() -> Uuid {
    Uuid::new_v4()
}

#[cfg(not(test))]
pub fn now() -> u64 {
    splat::character::now()
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

/// This computer's lasting identity and its name, for `sheet_writer`. The identity lives in the
/// user's config folder, outside the vault, so each computer keeps its own.
#[cfg(not(test))]
pub fn device() -> (Uuid, String) {
    static DEVICE: std::sync::OnceLock<(Uuid, String)> = std::sync::OnceLock::new();
    DEVICE
        .get_or_init(|| {
            let file = directories::ProjectDirs::from("", "", "splat")
                .map(|dirs| dirs.config_dir().join("device-id"));
            let saved = file
                .as_ref()
                .and_then(|file| std::fs::read_to_string(file).ok())
                .and_then(|text| text.trim().parse().ok());
            let id = saved.unwrap_or_else(|| {
                let id = Uuid::new_v4();
                if let Some(file) = &file {
                    let _ = file
                        .parent()
                        .map(std::fs::create_dir_all)
                        .transpose()
                        .and_then(|_| std::fs::write(file, id.to_string()));
                }
                id
            });
            let mut buffer = [0u8; 256];
            // SAFETY: the buffer is valid for its length, and the call writes inside it.
            let ok = unsafe { libc::gethostname(buffer.as_mut_ptr().cast(), buffer.len()) } == 0;
            let name = ok
                .then(|| {
                    let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
                    String::from_utf8_lossy(&buffer[..end]).trim().to_owned()
                })
                .filter(|name| !name.is_empty())
                .unwrap_or_else(|| "another computer".to_owned());
            (id, name)
        })
        .clone()
}

#[cfg(test)]
pub use closed::{
    character_cache, device, file_time, local_stamp, new_uuid, now, pick_folder, today,
};

#[cfg(test)]
pub mod closed {
    use super::*;
    use std::{cell::Cell, sync::OnceLock};

    /// A fixed clock, so "Fetched 3 h ago" renders the same on every run.
    pub const FIXED_NOW: u64 = 1_767_236_400;

    static CACHE_ROOT: OnceLock<PathBuf> = OnceLock::new();

    thread_local! {
        static NEXT_UUID: Cell<Option<u128>> = const { Cell::new(None) };
        static NEXT_FOLDER: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
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
        NEXT_FOLDER.with(|folder| folder.borrow_mut().take())
    }

    pub fn next_folder(path: PathBuf) {
        NEXT_FOLDER.with(|folder| *folder.borrow_mut() = Some(path));
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

    /// One fixed computer, so a test never reads the user's config folder.
    pub fn device() -> (Uuid, String) {
        (Uuid::from_u128(0x5e17), "This computer".to_owned())
    }

    /// The fixed clock, for a file too: a real one would change every capture of its date.
    pub fn file_time(_: &std::path::Path) -> Option<u64> {
        Some(FIXED_NOW)
    }
}
