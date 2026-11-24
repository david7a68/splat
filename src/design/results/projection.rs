//! A retained projection of a kept result, and the capability to act on it in one frame
//! (layout PR 2a, ruling L9; `docs/design/retained-results.md`).
//!
//! A [`Projection`] is what a pane may keep of its result across frames: the words, the tone,
//! the Undo and its lock, the choices, a refused attempt's words and the line's place in the
//! footer's order, as data. It holds no [`Ticket`], no closure and no borrow, and it has no draw
//! or act method. Only [`Retained::refresh`] makes one, from a store, and only when the store's
//! revision, the key or the owner's inputs changed.
//!
//! The capability is [`Results::present`]: with this frame's [`PaneFrame`] and a borrow of the
//! live store it checks that the projection was made from this store's current revision, for this
//! key, from these inputs (its [`Bound`]), and only then mints the entry's [`Ticket`] into this
//! frame's [`KeptLine`]. A mismatch is rejected ([`Mismatch`]): no line, no ticket.
//!
//! # What does not compile
//!
//! The setup, and a twin that compiles: the owner keeps the projection, and a frame presents it.
//!
//! ```
//! use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results, Retained};
//! use splat_design::outcome::{KeptLine, NoPress, PaneFrame, Said, UndoStep, Verb};
//! #[derive(Clone, PartialEq, Eq)] struct Link;
//! impl Owner for Link { type Scope = str; fn scope(&self) -> &str { "c" } }
//! struct Saved;
//! impl Kept for Saved { type Owner = Link; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Stays, press: Press::Nothing }; fn owner(&self) -> Link { Link } }
//! struct App { store: Results<Link, Saved>, view: Retained<Link, NoPress, splat_design::ownership::Revision> }
//! fn frame(app: &mut App, book: splat_design::ownership::Revision) {
//!     let projection = app.view.refresh(&app.store, &Link, book, |_| {
//!         Said::new(splat_design::sentence!("Campaign link saved.")).into()
//!     });
//!     let frame = PaneFrame::plain();
//!     let line: Option<KeptLine<NoPress>> = app.store.present(&frame, projection, &Link, book).ok().flatten();
//!     let _ = line;
//! }
//! ```
//!
//! A projection is made only by `Retained::refresh`, from a store:
//!
//! ```compile_fail
//! use splat_design::ownership::{Projection, Revision};
//! let _ = Projection::<u8, u8, Revision> { bound: todo!(), line: None }; // <- made by hand
//! ```
//!
//! It carries no ticket that a caller can read or act with:
//!
//! ```compile_fail
//! # use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results, Retained, Revision};
//! # use splat_design::outcome::{NoPress, Said};
//! # #[derive(Clone, PartialEq, Eq)] struct Link;
//! # impl Owner for Link { type Scope = str; fn scope(&self) -> &str { "c" } }
//! # struct Saved;
//! # impl Kept for Saved { type Owner = Link; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Stays, press: Press::Nothing }; fn owner(&self) -> Link { Link } }
//! fn owner(store: &Results<Link, Saved>, view: &mut Retained<Link, NoPress, Revision>, book: Revision) {
//!     let projection = view.refresh(store, &Link, book, |_| Said::new(splat_design::sentence!("Saved.")).into());
//!     let _ticket = projection.line.unwrap().ticket(); // <- a ticket without a frame
//! }
//! ```
//!
//! A line that `present` made is this frame's, as every `KeptLine` is: kept in app state, it
//! does not compile (the twin above uses it in its frame):
//!
//! ```compile_fail
//! # use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results, Retained, Revision};
//! # use splat_design::outcome::{KeptLine, NoPress, PaneFrame, Said};
//! # #[derive(Clone, PartialEq, Eq)] struct Link;
//! # impl Owner for Link { type Scope = str; fn scope(&self) -> &str { "c" } }
//! # struct Saved;
//! # impl Kept for Saved { type Owner = Link; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Stays, press: Press::Nothing }; fn owner(&self) -> Link { Link } }
//! struct App {
//!     store: Results<Link, Saved>,
//!     view: Retained<Link, NoPress, Revision>,
//!     kept: Option<KeptLine<'static, NoPress>>,
//! }
//! fn frame(app: &mut App, book: Revision) {
//!     let projection = app.view.refresh(&app.store, &Link, book, |_| Said::new(splat_design::sentence!("Saved.")).into());
//!     let frame = PaneFrame::plain();
//!     app.kept = app.store.present(&frame, projection, &Link, book).ok().flatten(); // <- kept past its frame
//! }
//! ```
use super::super::components::ResultOrder;
use super::super::memo::{Key, Revision};
use super::super::outcome::{KeptLine, KeptOutcome, PaneFrame, Typed, prepare};
use super::{Kept, Owner, Results, Shown, Ticket};

/// What a projection was made from: the store's revision (which also names the store: a
/// revision is new in the run, and a store is not `Clone`), the owner key it shows, and the
/// owner's other revisions that its words read. [`Results::present`] checks all three.
struct Bound<O, I> {
    revision: Revision,
    key: O,
    inputs: I,
}

/// One kept result as its footer line shows it, as data: the prepared message, its entry's
/// place in the store and order (private: they name the entry for `present`, and never leave as
/// a ticket; the place holds while the store's revision does), and the order of its refused
/// attempt while the refusal stands.
struct Line<A> {
    at: usize,
    order: ResultOrder,
    typed: Typed<A>,
    refused: Option<ResultOrder>,
}

/// The retained presentation of one owner key's kept result. See the module comment.
pub struct Projection<O, A, I> {
    bound: Bound<O, I>,
    line: Option<Line<A>>,
}

impl<O, A, I> Projection<O, A, I> {
    /// Its words, for a test.
    #[cfg(test)]
    pub(crate) fn words(&self) -> Option<&str> {
        self.line.as_ref().map(|line| line.typed.words())
    }

    /// Whether its line's Undo is locked, for a test.
    #[cfg(test)]
    pub(crate) fn undo_locked(&self) -> bool {
        self.line
            .as_ref()
            .is_some_and(|line| line.typed.undo_locked())
    }
}

/// Why [`Results::present`] refused a projection. Nothing is drawn, and no ticket is made.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mismatch {
    /// It was made from another revision of this store, or from another store.
    Revision,
    /// It shows another owner key.
    Key,
    /// It was made from other inputs of its owner.
    Inputs,
    /// Its entry's life passed and the owner did not `expire` first (an owner fault: a debug
    /// assertion fails too).
    Expired,
    /// Its entry is gone while the revision says nothing changed (a store fault: a debug
    /// assertion fails too).
    Missing,
}

/// The owner's retained projection of one key's kept result: kept across frames beside its
/// store, made again only when the store's revision, the key or the inputs changed.
pub struct Retained<O, A, I> {
    kept: Option<Projection<O, A, I>>,
}

impl<O, A, I> Default for Retained<O, A, I> {
    fn default() -> Self {
        Self { kept: None }
    }
}

/// The same owner in the same scope: the store's own comparison (`Entry::is`).
fn same<O: Owner>(a: &O, b: &O) -> bool {
    a.scope() == b.scope() && a == b
}

impl<O: Owner, A, I: Key> Retained<O, A, I> {
    pub const fn new() -> Self {
        Self { kept: None }
    }

    /// The projection of `key`'s result in `results`, made from `inputs`: the kept one while
    /// the store's revision, the key and the inputs are as it was made from; else made again with
    /// `make`, from the store's view of the entry. Call it after the owner's update (its
    /// `expire`, its end events), so a projection is made only from a settled store.
    pub fn refresh<T: Kept<Owner = O>>(
        &mut self,
        results: &Results<O, T>,
        key: &O,
        inputs: I,
        make: impl FnOnce(Shown<'_, T>) -> KeptOutcome<A>,
    ) -> &Projection<O, A, I> {
        count(|work| work.lookups += 1);
        let fresh = self.kept.as_ref().is_some_and(|kept| {
            kept.bound.revision == results.revision
                && same(&kept.bound.key, key)
                && kept.bound.inputs == inputs
        });
        if !fresh {
            count(|work| work.visits += results.entries.len());
            let at = results.entries.iter().position(|entry| entry.is(key));
            let line = at.map(|at| {
                let shown = Results::shown(&results.entries[at]);
                let order = shown.order;
                let refused = shown.refused.map(|refused| {
                    (
                        refused.order,
                        refused.refusal.words_told_apart(None),
                        refused.refusal.tone(),
                    )
                });
                let (typed, refused) = prepare(make(shown), refused);
                count(|work| {
                    work.built += 1;
                    work.bytes += typed.word_bytes();
                });
                Line {
                    at,
                    order,
                    typed,
                    refused,
                }
            });
            self.kept = Some(Projection {
                bound: Bound {
                    revision: results.revision,
                    key: key.clone(),
                    inputs,
                },
                line,
            });
        }
        self.kept.as_ref().expect("made just now")
    }

    /// The kept projection, if one was made.
    pub fn get(&self) -> Option<&Projection<O, A, I>> {
        self.kept.as_ref()
    }

    pub fn forget(&mut self) {
        self.kept = None;
    }
}

impl<O: Owner, T: Kept<Owner = O>> Results<O, T> {
    /// The capability to show and act on `projection` in this frame: its line as this frame's
    /// [`KeptLine`], with its entry's [`Ticket`] (`Ok(None)`: the key has no result). Refused,
    /// with nothing drawn and no ticket, when the projection was not made from this store's
    /// current revision, for `key`, from `inputs` ([`Mismatch`]). The line copies the projected
    /// message for the frame's footer (counted: `handed`; PR 2b's footer paints the projection).
    pub fn present<'f, A: Clone, I: Key>(
        &self,
        frame: &'f PaneFrame,
        projection: &Projection<O, A, I>,
        key: &O,
        inputs: I,
    ) -> Result<Option<KeptLine<'f, A>>, Mismatch> {
        let bound = &projection.bound;
        if bound.revision != self.revision {
            return Err(Mismatch::Revision);
        }
        if !same(&bound.key, key) {
            return Err(Mismatch::Key);
        }
        if bound.inputs != inputs {
            return Err(Mismatch::Inputs);
        }
        let Some(line) = &projection.line else {
            return Ok(None);
        };
        // The entry's place, kept with the projection: the revision says the entries are as
        // they were, so no entry is searched for.
        let Some(entry) = self
            .entries
            .get(line.at)
            .filter(|entry| entry.order == line.order)
        else {
            debug_assert!(
                false,
                "a projection's entry is gone while its store's revision is unchanged"
            );
            return Err(Mismatch::Missing);
        };
        if entry.ended() {
            debug_assert!(
                false,
                "a kept result's life passed: its owner calls `Results::expire` before it presents"
            );
            return Err(Mismatch::Expired);
        }
        count(|work| work.handed += line.typed.word_bytes());
        Ok(Some(KeptLine::prepared(
            frame,
            Ticket { order: line.order },
            line.typed.clone(),
            line.refused,
        )))
    }
}

/// The work of retained projections and timed stores on this thread, for tests (the visual
/// suite runs scenarios on parallel threads, so the count is the thread's own). Counted inside
/// the work itself.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Work {
    /// Calls of `Retained::refresh`: frames that reached the seam.
    pub lookups: usize,
    /// Lines made from the store and the owner's data.
    pub built: usize,
    /// The bytes of the words of those lines.
    pub bytes: usize,
    /// Store entries visited by `refresh`, `present` and `expire`.
    pub visits: usize,
    /// The bytes of words that `present` copied into this frame's lines (the legacy footer's
    /// copy, PR 2b's to remove).
    pub handed: usize,
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    pub(crate) static WORK: std::cell::Cell<Work> = const {
        std::cell::Cell::new(Work { lookups: 0, built: 0, bytes: 0, visits: 0, handed: 0 })
    };
}

/// Count work, in a test build.
#[cfg(any(test, feature = "test-support"))]
pub(super) fn count(add: impl FnOnce(&mut Work)) {
    WORK.with(|work| {
        let mut now = work.get();
        add(&mut now);
        work.set(now);
    });
}

#[cfg(not(any(test, feature = "test-support")))]
pub(super) fn count(_: impl FnOnce(&mut Work)) {}

/// This thread's projection and timer work so far.
#[cfg(any(test, feature = "test-support"))]
pub fn work() -> Work {
    WORK.with(std::cell::Cell::get)
}
