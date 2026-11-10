//! Bound machine-wide visual-suite load before entering a private namespace.
use std::{
    env,
    fs::{self, File, OpenOptions},
    io,
    path::Path,
    time::{Duration, Instant},
};

// At the default four workers, a full capture run keeps about six cores busy
// and needs about 1.1 GiB. Each 4 GiB slot leaves room for builds and the app.
const CORES_PER_SLOT: usize = 6;
const GIB_PER_SLOT: u64 = 4;

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
    let count =
        slot_count(value, default_slots()).expect("SPLAT_VISUAL_SLOTS must be a positive integer");
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

/// Every worktree on a computer computes the same count, so they share the limit.
fn default_slots() -> usize {
    let cores = std::thread::available_parallelism().map_or(1, usize::from);
    slots_for(cores, total_memory_gib())
}

fn slots_for(cores: usize, memory_gib: Option<u64>) -> usize {
    let by_memory = memory_gib.map_or(usize::MAX, |gib| {
        usize::try_from(gib / GIB_PER_SLOT).unwrap_or(usize::MAX)
    });
    (cores / CORES_PER_SLOT).min(by_memory).max(1)
}

/// Whole GiB of RAM, where the system reports it.
fn total_memory_gib() -> Option<u64> {
    let info = fs::read_to_string("/proc/meminfo").ok()?;
    let kib: u64 = info
        .lines()
        .find_map(|line| line.strip_prefix("MemTotal:"))?
        .trim()
        .strip_suffix("kB")?
        .trim()
        .parse()
        .ok()?;
    Some(kib >> 20)
}

fn slot_count(value: Option<&str>, default: usize) -> Result<usize, &'static str> {
    let count = match value {
        Some(value) => value.parse().map_err(|_| "invalid slot count")?,
        None => default,
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
    assert_eq!(slot_count(None, 3), Ok(3));
    assert_eq!(slot_count(Some("2"), 3), Ok(2));
    for invalid in ["", "0", "-1", "two", "1.5"] {
        assert!(slot_count(Some(invalid), 3).is_err(), "{invalid}");
    }
}

#[test]
fn default_slots_follow_cores_and_memory() {
    // A 20-core laptop with 15 GiB, and a 16-core desktop with 31 GiB.
    assert_eq!(slots_for(20, Some(15)), 3);
    assert_eq!(slots_for(16, Some(31)), 2);
    // Memory limits a computer with many cores.
    assert_eq!(slots_for(64, Some(8)), 2);
    // Small or unknown computers still run one suite.
    assert_eq!(slots_for(4, Some(2)), 1);
    assert_eq!(slots_for(12, None), 2);
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
