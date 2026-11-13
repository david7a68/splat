//! Per-change caches: a value worked out once per change of its inputs, never per frame.
//!
//! A [`Revision`] names one version of a value. Only a real change makes one: a new
//! [`Revised`] value, or its [`Revised::edit`], [`Revised::set`] or a [`Revised::set_if_changed`]
//! that finds a change. Every revision is new in this run of the app, so the revisions of two
//! values never meet. A [`Memo`] takes only revisions as its key (one, or a tuple of them), so a
//! pointer address, a frame count or an input epoch cannot be a key: the cache type does not
//! accept them.
//!
//! Caller duties left: keep each input that the cached work reads in a [`Revised`] (or take the
//! revision of the [`Revised`] that holds it), and change it only through `Revised`.
//!
//! # What does not compile
//!
//! Each case has a twin that compiles and differs only in the marked line (rustc 1.95 does not
//! check a `compile_fail` test's error code).
//!
//! ```
//! use splat::ownership::{Memo, Revised, Revision};
//! let mut draft = Revised::new(String::from("# Cinder Wolf"));
//! let mut lines: Memo<Revision, usize> = Memo::new();
//! assert_eq!(*lines.get_or(draft.revision(), || draft.lines().count()), 1);
//! draft.edit().push_str("\nSize: Medium");
//! assert_eq!(*lines.get_or(draft.revision(), || draft.lines().count()), 2);
//! let mut both: Memo<(Revision, Revision), bool> = Memo::new();
//! let index = Revised::new(vec!["Cinder Wolf"]);
//! both.get_or((draft.revision(), index.revision()), || true);
//! ```
//!
//! A frame count or an address is not a key:
//!
//! ```compile_fail
//! # use splat::ownership::Memo;
//! let frame: u64 = 41;
//! let mut lines: Memo<u64, usize> = Memo::new(); // <- keyed by a frame count
//! lines.get_or(frame, || 1);
//! ```
//!
//! ```compile_fail
//! # use splat::ownership::{Memo, Revised, Revision};
//! let draft = Revised::new(String::from("# Cinder Wolf"));
//! let mut lines: Memo<(Revision, usize), usize> = Memo::new(); // <- an address in the key
//! lines.get_or((draft.revision(), draft.as_ptr() as usize), || 1);
//! ```
//!
//! No new key kind can be added outside this module:
//!
//! ```compile_fail
//! # use splat::ownership::{Key, Memo};
//! #[derive(Clone, Copy, PartialEq, Eq)]
//! struct Frame(u64);
//! impl Key for Frame {} // <- not sealed for outside kinds
//! ```
//!
//! A revision is never made by hand:
//!
//! ```compile_fail
//! # use splat::ownership::Revision;
//! let revision = Revision::default(); // <- no change made it
//! ```
//!
//! A `Revised` value changes only through `edit`, `set` or `set_if_changed`, each of which
//! gives a new revision:
//!
//! ```compile_fail
//! # use splat::ownership::Revised;
//! let mut draft = Revised::new(String::from("# Cinder Wolf"));
//! draft.push_str("\nSize: Medium"); // <- a change with no new revision
//! ```
use std::num::NonZeroU64;

/// One version of a value. See the module comment.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Revision(NonZeroU64);

impl Revision {
    fn next() -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self(NonZeroU64::new(n).expect("a revision count never wraps"))
    }
}

/// A value and its revision. A clone is the same version, with the same revision.
#[derive(Clone, Debug)]
pub struct Revised<T> {
    value: T,
    revision: Revision,
}

impl<T> Revised<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            revision: Revision::next(),
        }
    }

    pub fn revision(&self) -> Revision {
        self.revision
    }

    /// Change the value: it gets a new revision now, whatever the change.
    pub fn edit(&mut self) -> &mut T {
        self.revision = Revision::next();
        &mut self.value
    }

    pub fn set(&mut self, value: T) {
        *self.edit() = value;
    }

    /// Set the value, and give a new revision only when it differs. For small values (a file
    /// stamp, a count): the comparison is as large as the value.
    pub fn set_if_changed(&mut self, value: T) -> bool
    where
        T: PartialEq,
    {
        let changed = self.value != value;
        if changed {
            self.set(value);
        }
        changed
    }
}

impl<T> std::ops::Deref for Revised<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Revision {}
    impl Sealed for (super::Revision, super::Revision) {}
    impl Sealed for (super::Revision, super::Revision, super::Revision) {}
}

/// What a [`Memo`] can be keyed by: a revision, or a tuple of revisions. Sealed.
pub trait Key: sealed::Sealed + Copy + Eq {}
impl Key for Revision {}
impl Key for (Revision, Revision) {}
impl Key for (Revision, Revision, Revision) {}

/// The last value worked out, and the revisions it was worked out from.
pub struct Memo<K: Key, T> {
    kept: Option<(K, T)>,
}

impl<K: Key, T> Default for Memo<K, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Key, T> Memo<K, T> {
    pub const fn new() -> Self {
        Self { kept: None }
    }

    /// The value for `key`: the kept one, or a new one from `build` when the key changed.
    pub fn get_or(&mut self, key: K, build: impl FnOnce() -> T) -> &T {
        if self.kept.as_ref().is_none_or(|(seen, _)| *seen != key) {
            self.kept = Some((key, build()));
        }
        &self.kept.as_ref().expect("kept just now").1
    }

    /// The kept value, only when it was worked out for `key`.
    pub fn get(&self, key: K) -> Option<&T> {
        self.kept
            .as_ref()
            .filter(|(seen, _)| *seen == key)
            .map(|(_, value)| value)
    }

    pub fn forget(&mut self) {
        self.kept = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    /// The work runs once per change, and not for an unchanged read, a clone, or another value's
    /// change. The counter is inside the work.
    #[test]
    fn a_memo_works_once_per_change() {
        let builds = Cell::new(0);
        let count = |text: &str| {
            builds.set(builds.get() + 1);
            text.lines().count()
        };
        let mut draft = Revised::new(String::from("a"));
        let other = Revised::new(String::from("a"));
        assert_ne!(
            draft.revision(),
            other.revision(),
            "equal values, two versions"
        );
        let mut memo: Memo<Revision, usize> = Memo::new();
        for _ in 0..3 {
            assert_eq!(*memo.get_or(draft.revision(), || count(&draft)), 1);
        }
        let copy = draft.clone();
        assert_eq!(*memo.get_or(copy.revision(), || count(&copy)), 1);
        assert_eq!(builds.get(), 1, "unchanged reads and a clone do no work");
        draft.edit().push_str("\nb");
        assert_eq!(*memo.get_or(draft.revision(), || count(&draft)), 2);
        assert_eq!(builds.get(), 2);
        // A set to the same value is no change.
        let before = draft.revision();
        assert!(!draft.set_if_changed(String::from("a\nb")));
        assert_eq!(draft.revision(), before);
        assert!(draft.set_if_changed(String::from("a")));
        assert_ne!(draft.revision(), before);
        assert_eq!(memo.get(before), Some(&2));
        assert_eq!(memo.get(draft.revision()), None);
        memo.forget();
        assert_eq!(memo.get(before), None);
    }
}
