//! Per-change caches: a value worked out once per change of its inputs, never per frame.
//!
//! A [`Revision`] names one version of a value. A new [`Revised`] value gets one, and after
//! that a revision advances only when a change says it changed: [`Revised::revise`] advances it
//! only when its closure returns [`Changed::Yes`], and [`Revised::set`] only when the new value
//! differs. A call made on every frame with no change keeps the revision. Every revision is new
//! in this run of the app, so the revisions of two values never meet. A [`Memo`] takes only
//! revisions as its key (one, or a tuple of them), so a pointer address, a frame count or an
//! input epoch cannot be a key: the cache type does not accept them.
//!
//! A `Revised` value and a `Memo` value must be `Sync`, so a `Cell` or `RefCell` at any depth
//! (a draft that holds one too) does not compile: it would change the value through `&` with no
//! new revision.
//!
//! **Caller limits that the types cannot hold** (stable Rust has no way to forbid them):
//!
//! - A `Mutex`, `RwLock` or atomic inside a `Revised` is `Sync` and still changes the value
//!   through `&`. That is a deliberate bypass: the source scan
//!   `results_are_never_behind_interior_mutability` (`design::results`) fails on the spellings it
//!   can see (`Revised<Mutex<…>>`, `Revised::new(Mutex::new(…))`, and their `std::sync` paths);
//!   one hidden in a field of another type is for a reviewer.
//! - `Revised::new` made again on every frame (for example around a frame counter) gives a new
//!   revision every frame. Make a `Revised` where its value is made, and keep it; a reviewer
//!   checks this.
//! - `revise` trusts its closure's [`Changed`]. For an egui field, use
//!   `Changed::from(response.changed())`.
//!
//! Caller duties left: keep each input that the cached work reads in a [`Revised`] (or take the
//! revision of the [`Revised`] that holds it).
//!
//! # What does not compile
//!
//! Each case has a twin that compiles and differs only in the marked line (rustc 1.95 does not
//! check a `compile_fail` test's error code).
//!
//! ```
//! use splat::ownership::{Changed, Memo, Revised, Revision};
//! let mut draft = Revised::new(String::from("# Cinder Wolf"));
//! let mut lines: Memo<Revision, usize> = Memo::new();
//! assert_eq!(*lines.get_or(draft.revision(), || draft.lines().count()), 1);
//! draft.revise(|text| {
//!     text.push_str("\nSize: Medium");
//!     Changed::Yes
//! });
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
//! A `Cell` or `RefCell` inside a `Revised`, at any depth, changes it through `&`:
//!
//! ```compile_fail
//! # use splat::ownership::Revised;
//! let draft = Revised::new(std::cell::RefCell::new(String::new())); // <- not `Sync`
//! draft.borrow_mut().push_str("# Cinder Wolf");
//! ```
//!
//! ```compile_fail
//! # use splat::ownership::Revised;
//! struct Draft { text: String, cursor: std::cell::Cell<usize> }
//! let draft = Revised::new(Draft { text: String::new(), cursor: std::cell::Cell::new(0) }); // <-
//! ```
//!
//! The twin of both: the same draft with plain fields.
//!
//! ```
//! # use splat::ownership::Revised;
//! struct Draft { text: String, cursor: usize }
//! let draft = Revised::new(Draft { text: String::new(), cursor: 0 });
//! assert_eq!((draft.text.len(), draft.cursor), (0, 0));
//! ```
//!
//! `set` compares, so it needs a value that equals itself. `NaN` does not, so a `set` of it would
//! advance the revision on every call:
//!
//! ```compile_fail
//! # use splat::ownership::Revised;
//! let mut speed = Revised::new(f64::NAN);
//! speed.set(f64::NAN); // <- `f64` is not `Eq`
//! ```
//!
//! Its twin: a value that is `Eq` uses `set`; one that is not says its change with `revise`.
//!
//! ```
//! # use splat::ownership::{Changed, Revised};
//! let mut count = Revised::new(3_u32);
//! let before = count.revision();
//! count.set(3);
//! assert_eq!(count.revision(), before);
//! let mut speed = Revised::new(30.0_f64);
//! speed.revise(|speed| {
//!     let new = 40.0;
//!     let changed = Changed::from(*speed != new);
//!     *speed = new;
//!     changed
//! });
//! assert_ne!(speed.revision(), before);
//! ```
//!
//! A `Revised` value changes only through `revise` or `set`. There is no bare `&mut` to it:
//!
//! ```compile_fail
//! # use splat::ownership::Revised;
//! let mut draft = Revised::new(String::from("# Cinder Wolf"));
//! draft.push_str("\nSize: Medium"); // <- a change with no new revision
//! ```
//!
//! ```compile_fail
//! # use splat::ownership::Revised;
//! let mut draft = Revised::new(String::from("# Cinder Wolf"));
//! draft.edit().push_str("\nSize: Medium"); // <- a new revision whether or not it changed
//! ```
//!
//! A change says whether it changed:
//!
//! ```compile_fail
//! # use splat::ownership::Revised;
//! let mut draft = Revised::new(String::from("# Cinder Wolf"));
//! draft.revise(|text| text.push_str("\nSize: Medium")); // <- no `Changed`
//! ```
use std::num::NonZeroU64;

/// Whether a draft or a revised value changed. Use it in place of a `bool`.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Changed {
    No,
    Yes,
}

impl From<bool> for Changed {
    /// For a test that already answers the question, such as egui's `response.changed()`.
    fn from(changed: bool) -> Self {
        if changed { Self::Yes } else { Self::No }
    }
}

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
pub struct Revised<T: Sync> {
    value: T,
    revision: Revision,
}

impl<T: Sync> Revised<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            revision: Revision::next(),
        }
    }

    pub fn revision(&self) -> Revision {
        self.revision
    }

    /// Change the value with `change`, which says whether it changed. The revision advances
    /// only on [`Changed::Yes`].
    pub fn revise(&mut self, change: impl FnOnce(&mut T) -> Changed) {
        if change(&mut self.value) == Changed::Yes {
            self.revision = Revision::next();
        }
    }

    /// Set the value. The revision advances only when it differs; the comparison is as large
    /// as the value. A value that is not `Eq` (a float: `NaN` never equals itself) says its
    /// change with [`Self::revise`].
    pub fn set(&mut self, value: T)
    where
        T: Eq,
    {
        if self.value != value {
            self.value = value;
            self.revision = Revision::next();
        }
    }
}

impl<T: Sync> std::ops::Deref for Revised<T> {
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
/// Its value must be `Sync`, so a kept value cannot change through `&` with a `Cell`.
pub struct Memo<K: Key, T: Sync> {
    kept: Option<(K, T)>,
}

impl<K: Key, T: Sync> Default for Memo<K, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Key, T: Sync> Memo<K, T> {
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

    /// The work runs once per change, and not for an unchanged read, a clone, a change that
    /// says it changed nothing, a `set` to the same value, or another value's change. The
    /// counter is inside the work.
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
        // A revise on every frame that changes nothing (an egui field with no edit) keeps the
        // revision (review of PR 301: a bare `edit` advanced it every frame).
        let before = draft.revision();
        for _ in 0..3 {
            draft.revise(|_| Changed::from(false));
            assert_eq!(*memo.get_or(draft.revision(), || count(&draft)), 1);
        }
        assert_eq!((draft.revision(), builds.get()), (before, 1));
        draft.revise(|text| {
            text.push_str("\nb");
            Changed::Yes
        });
        assert_ne!(draft.revision(), before);
        assert_eq!(*memo.get_or(draft.revision(), || count(&draft)), 2);
        assert_eq!(builds.get(), 2);
        // A set to the same value is no change; a different one is.
        let before = draft.revision();
        draft.set(String::from("a\nb"));
        assert_eq!(draft.revision(), before);
        draft.set(String::from("a"));
        assert_ne!(draft.revision(), before);
        assert_eq!(memo.get(before), Some(&2));
        assert_eq!(memo.get(draft.revision()), None);
        memo.forget();
        assert_eq!(memo.get(before), None);
    }
}
