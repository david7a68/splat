//! Results of slow pure functions, remembered by a hash of their inputs.
use sha2::{Digest, Sha256};
use std::{collections::VecDeque, sync::Mutex};

/// The last few results of one pure function. The key is a SHA-256 of everything the result
/// depends on, so a hit returns exactly what a new build would return.
pub struct Memo<T> {
    capacity: usize,
    entries: Mutex<VecDeque<([u8; 32], T)>>,
}

impl<T: Clone> Memo<T> {
    pub const fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Mutex::new(VecDeque::new()),
        }
    }

    /// The remembered result for these inputs, or a new one from `build`.
    pub fn get_or(&self, inputs: &[&[u8]], build: impl FnOnce() -> T) -> T {
        let mut hasher = Sha256::new();
        for input in inputs {
            // The length keeps ("ab", "c") and ("a", "bc") apart.
            hasher.update((input.len() as u64).to_le_bytes());
            hasher.update(input);
        }
        let key: [u8; 32] = hasher.finalize().into();
        let entries = || self.entries.lock().unwrap_or_else(|e| e.into_inner());
        if let Some((_, value)) = entries().iter().find(|(seen, _)| *seen == key) {
            return value.clone();
        }
        // Build outside the lock, so other threads' lookups do not wait for this build.
        let value = build();
        let mut entries = entries();
        if !entries.iter().any(|(seen, _)| *seen == key) {
            if entries.len() == self.capacity {
                entries.pop_front();
            }
            entries.push_back((key, value.clone()));
        }
        value
    }
}

#[test]
fn a_memo_builds_once_per_input_and_forgets_the_oldest() {
    let memo = Memo::new(2);
    let builds = std::cell::Cell::new(0);
    let get = |input: &str| {
        memo.get_or(&[input.as_bytes()], || {
            builds.set(builds.get() + 1);
            input.len()
        })
    };
    assert_eq!((get("a"), get("a"), get("bb")), (1, 1, 2));
    assert_eq!(get("ccc"), 3);
    // "a" was the oldest of three, so it is built again.
    assert_eq!(get("a"), 1);
    assert_eq!(builds.get(), 4);
    let split = Memo::new(4);
    assert_ne!(
        split.get_or(&[b"ab", b"c"], || 1),
        split.get_or(&[b"a", b"bc"], || 2)
    );
}
