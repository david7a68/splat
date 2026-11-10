//! Bound machine-wide visual-suite memory before entering a private namespace.
use std::{
    env,
    fs::{self, File, OpenOptions},
    io,
    path::Path,
    time::{Duration, Instant},
};

// Shared parsed fields reduced a full suite to 6.25 GiB on the 31 GiB host.
// Two suites leave ample memory for the app, builds and filesystem cache.
const DEFAULT_SLOTS: usize = 2;

pub(super) struct Permit {
    index: usize,
    lock: Option<File>,
}

impl Permit {
    /// Slot zero is also the legacy shared-world lock. Transfer it to the
    /// non-isolated world instead of trying to lock that file a second time.
    pub(super) fn take_world_lock(&mut self) -> Option<File> {
        (self.index == 0).then(|| self.lock.take()).flatten()
    }
}

pub(super) fn acquire() -> Option<Permit> {
    // The parent keeps its permit until this child has finished. Acquiring
    // inside the namespace would see private lock files and defeat the limit.
    #[cfg(target_os = "linux")]
    if super::isolation::is_child() {
        return None;
    }
    let value = env::var_os("SPLAT_VISUAL_SLOTS");
    let value = value
        .as_deref()
        .map(|value| value.to_str().expect("SPLAT_VISUAL_SLOTS must be UTF-8"));
    let count = slot_count(value).expect("SPLAT_VISUAL_SLOTS must be a positive integer");
    let started = Instant::now();
    let permit = acquire_in(&env::temp_dir(), count).expect("lock a visual suite slot");
    if env::var_os("SPLAT_VISUAL_TIMINGS").is_some() {
        println!(
            "Timing visual slot wait = {:?} (slot {}/{count})",
            started.elapsed(),
            permit.index + 1,
        );
    }
    Some(permit)
}

fn slot_count(value: Option<&str>) -> Result<usize, &'static str> {
    let count = match value {
        Some(value) => value.parse().map_err(|_| "invalid slot count")?,
        None => DEFAULT_SLOTS,
    };
    (count > 0).then_some(count).ok_or("zero slots")
}

pub(super) fn acquire_in(temp: &Path, count: usize) -> io::Result<Permit> {
    loop {
        if let Some(permit) = try_acquire(temp, count)? {
            return Ok(permit);
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

pub(super) fn try_acquire(temp: &Path, count: usize) -> io::Result<Option<Permit>> {
    for index in 0..count {
        // Reuse the old lock for slot zero, so main/older worktrees count
        // against the default limit too. Never unlink any slot lock file.
        let name = if index == 0 {
            "splat-visual-world.lock".to_owned()
        } else {
            format!("splat-visual-slot-{index}.lock")
        };
        let lock = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(temp.join(name))?;
        match lock.try_lock() {
            Ok(()) => {
                return Ok(Some(Permit {
                    index,
                    lock: Some(lock),
                }));
            }
            Err(fs::TryLockError::WouldBlock) => {}
            Err(fs::TryLockError::Error(error)) => return Err(error),
        }
    }
    Ok(None)
}

#[test]
fn slot_count_requires_a_positive_integer() {
    assert_eq!(slot_count(None), Ok(2));
    assert_eq!(slot_count(Some("2")), Ok(2));
    for invalid in ["", "0", "-1", "two", "1.5"] {
        assert!(slot_count(Some(invalid)).is_err(), "{invalid}");
    }
}

#[test]
fn occupied_slots_are_skipped_and_released_slots_are_reused() {
    let temp = tempfile::tempdir().unwrap();
    let mut first = try_acquire(temp.path(), 2).unwrap().unwrap();
    assert_eq!(first.index, 0);
    // The old shared-world owner participates in slot zero as well.
    let old_world_lock = first.take_world_lock().unwrap();
    let second = try_acquire(temp.path(), 2).unwrap().unwrap();
    assert_eq!(second.index, 1);
    assert!(try_acquire(temp.path(), 2).unwrap().is_none());
    drop(second);
    let released = acquire_in(temp.path(), 2).unwrap();
    assert_eq!(released.index, 1, "do not wait behind occupied slot zero");
    drop(old_world_lock);
    assert_eq!(try_acquire(temp.path(), 1).unwrap().unwrap().index, 0);
}

#[test]
fn a_waiter_proceeds_after_a_slot_owner_exits() {
    let temp = tempfile::tempdir().unwrap();
    let owner = acquire_in(temp.path(), 1).unwrap();
    let (send, receive) = std::sync::mpsc::channel();
    std::thread::scope(|scope| {
        scope.spawn(|| {
            let permit = acquire_in(temp.path(), 1).unwrap();
            send.send(permit.index).unwrap();
        });
        assert!(receive.try_recv().is_err());
        drop(owner);
        assert_eq!(receive.recv_timeout(Duration::from_secs(5)).unwrap(), 0);
    });
}
