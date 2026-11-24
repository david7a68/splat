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
//! use splat_design::ownership::{Changed, Memo, Revised, Revision};
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
//! # use splat_design::ownership::Memo;
//! let frame: u64 = 41;
//! let mut lines: Memo<u64, usize> = Memo::new(); // <- keyed by a frame count
//! lines.get_or(frame, || 1);
//! ```
//!
//! ```compile_fail
//! # use splat_design::ownership::{Memo, Revised, Revision};
//! let draft = Revised::new(String::from("# Cinder Wolf"));
//! let mut lines: Memo<(Revision, usize), usize> = Memo::new(); // <- an address in the key
//! lines.get_or((draft.revision(), draft.as_ptr() as usize), || 1);
//! ```
//!
//! No new key kind can be added outside this module:
//!
//! ```compile_fail
//! # use splat_design::ownership::{Key, Memo};
//! #[derive(Clone, Copy, PartialEq, Eq)]
//! struct Frame(u64);
//! impl Key for Frame {} // <- not sealed for outside kinds
//! ```
//!
//! A fit's key holds the owner's revision and the [`Measure`] it was fitted in, never a hash of
//! the content (ruling M6):
//!
//! ```compile_fail
//! # use splat_design::ownership::{Measure, Memo};
//! let fits: Memo<(u64, Measure), String> = Memo::new(); // <- a content hash beside the measure
//! ```
//!
//! ```
//! # use splat_design::ownership::{Measure, Memo, Revision};
//! let fits: Memo<(Revision, Measure), String> = Memo::new();
//! ```
//!
//! A revision is never made by hand:
//!
//! ```compile_fail
//! # use splat_design::ownership::Revision;
//! let revision = Revision::default(); // <- no change made it
//! ```
//!
//! A `Cell` or `RefCell` inside a `Revised`, at any depth, changes it through `&`:
//!
//! ```compile_fail
//! # use splat_design::ownership::Revised;
//! let draft = Revised::new(std::cell::RefCell::new(String::new())); // <- not `Sync`
//! draft.borrow_mut().push_str("# Cinder Wolf");
//! ```
//!
//! ```compile_fail
//! # use splat_design::ownership::Revised;
//! struct Draft { text: String, cursor: std::cell::Cell<usize> }
//! let draft = Revised::new(Draft { text: String::new(), cursor: std::cell::Cell::new(0) }); // <-
//! ```
//!
//! The twin of both: the same draft with plain fields.
//!
//! ```
//! # use splat_design::ownership::Revised;
//! struct Draft { text: String, cursor: usize }
//! let draft = Revised::new(Draft { text: String::new(), cursor: 0 });
//! assert_eq!((draft.text.len(), draft.cursor), (0, 0));
//! ```
//!
//! `set` compares, so it needs a value that equals itself. `NaN` does not, so a `set` of it would
//! advance the revision on every call:
//!
//! ```compile_fail
//! # use splat_design::ownership::Revised;
//! let mut speed = Revised::new(f64::NAN);
//! speed.set(f64::NAN); // <- `f64` is not `Eq`
//! ```
//!
//! Its twin: a value that is `Eq` uses `set`; one that is not says its change with `revise`.
//!
//! ```
//! # use splat_design::ownership::{Changed, Revised};
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
//! # use splat_design::ownership::Revised;
//! let mut draft = Revised::new(String::from("# Cinder Wolf"));
//! draft.push_str("\nSize: Medium"); // <- a change with no new revision
//! ```
//!
//! ```compile_fail
//! # use splat_design::ownership::Revised;
//! let mut draft = Revised::new(String::from("# Cinder Wolf"));
//! draft.edit().push_str("\nSize: Medium"); // <- a new revision whether or not it changed
//! ```
//!
//! A change says whether it changed:
//!
//! ```compile_fail
//! # use splat_design::ownership::Revised;
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
    /// A new revision, for a store in `design` that publishes its own (`Results::revision`).
    pub(in crate::design) fn next() -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self(NonZeroU64::new(n).expect("a revision count never wraps"))
    }

    pub(in crate::design) fn value(self) -> u64 {
        self.0.get()
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

/// A new value made with its default, as `Revised::new(T::default())`: a new version.
impl<T: Sync + Default> Default for Revised<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Sync> std::ops::Deref for Revised<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

/// What a text was measured in (ruling M6): its room, its font, the theme's font generation and
/// the display scale. A fit kept with one is worked out again when any of them changes, so a
/// font installed again under the same `FontId` or a new scale never shows an old fit. It is
/// made only from the `Ui` that measures ([`Measure::of`]), so no other number can stand in.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Measure {
    room: u32,
    size: u32,
    family: egui::FontFamily,
    fonts: u64,
    scale: u32,
}

impl Measure {}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Revision {}
    impl Sealed for (super::Revision, super::Revision) {}
    impl Sealed for (super::Revision, super::Revision, super::Revision) {}
    impl<K: super::Key> Sealed for (K, super::Measure) {}
}

/// What a [`Memo`] can be keyed by: a revision, a tuple of revisions, or either with the
/// [`Measure`] a text was fitted in. Sealed.
pub trait Key: sealed::Sealed + Clone + Eq {}
impl Key for Revision {}
impl Key for (Revision, Revision) {}
impl Key for (Revision, Revision, Revision) {}
impl<K: Key> Key for (K, Measure) {}

/// Several entries of one owner, each a [`Memo`] under its own id (a list's rows): each one is
/// worked out once per change of its own key. The owner holds it, so the entries end with that
/// owner, and it drops the rows it no longer shows with [`Memos::retain`].
pub struct Memos<I, K: Key, T: Sync> {
    entries: std::collections::HashMap<I, Memo<K, T>>,
}

impl<I, K: Key, T: Sync> Default for Memos<I, K, T> {
    fn default() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
}

impl<I: std::hash::Hash + Eq, K: Key, T: Sync> Memos<I, K, T> {
    /// The value of entry `id` for `key`: the kept one, or a new one from `build` when the key
    /// changed. The id is copied only for an entry that is new (a name is looked up by `&str`).
    pub fn get_or<Q>(&mut self, id: &Q, key: K, build: impl FnOnce() -> T) -> &T
    where
        I: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ToOwned<Owned = I> + ?Sized,
    {
        if !self.entries.contains_key(id) {
            self.entries.insert(id.to_owned(), Memo::new());
        }
        self.entries
            .get_mut(id)
            .expect("inserted just now")
            .get_or(key, build)
    }

    /// Whether an entry is kept for `id`, whatever its key.
    pub fn holds<Q>(&self, id: &Q) -> bool
    where
        I: std::borrow::Borrow<Q>,
        Q: std::hash::Hash + Eq + ?Sized,
    {
        self.entries.contains_key(id)
    }

    /// Keep only the entries whose id `keep` accepts.
    pub fn retain(&mut self, mut keep: impl FnMut(&I) -> bool) {
        self.entries.retain(|id, _| keep(id));
    }

    /// Test builds: an entry for `id` with no value yet, as a row shown once leaves.
    #[cfg(any(test, feature = "test-support"))]
    pub fn seed(&mut self, id: I) {
        self.entries.insert(id, Memo::new());
    }
}

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
        // Several entries of one owner: each is worked out once per change of its own key, a
        // change of one leaves the other, and a dropped entry is worked out again.
        let mut rows: Memos<u32, Revision, usize> = Memos::default();
        let other = Revised::new(String::from("x\ny\nz"));
        for _ in 0..3 {
            assert_eq!(*rows.get_or(&1, draft.revision(), || count(&draft)), 1);
            assert_eq!(*rows.get_or(&2, other.revision(), || count(&other)), 3);
        }
        assert_eq!(builds.get(), 4, "one build for each entry");
        draft.set(String::from("a\nb"));
        assert_eq!(*rows.get_or(&1, draft.revision(), || count(&draft)), 2);
        assert_eq!(*rows.get_or(&2, other.revision(), || count(&other)), 3);
        assert_eq!(builds.get(), 5, "a change of entry 1 leaves entry 2");
        rows.retain(|id| *id != 2);
        assert!(rows.holds(&1) && !rows.holds(&2));
        assert_eq!(*rows.get_or(&2, other.revision(), || count(&other)), 3);
        assert_eq!(builds.get(), 6, "a dropped entry is worked out again");
    }
}
