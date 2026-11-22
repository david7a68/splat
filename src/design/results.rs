//! Kept results and their footer order: one store per kind of kept result, keyed by the object
//! that owns each result.
//!
//! A kept result is a message that stays after its action (`Links now say “Ash Wolf”.` with
//! `Undo link update`, an offer to change links after a rename). [`Results`] holds every result
//! of one kind with three things that must never come apart:
//!
//! - **Its owner.** [`Kept::owner`] names the object (an entry in its root, a note, a run step).
//!   The store takes the owner from the value, and it keys every lookup and change by the
//!   owner's scope and the owner together, so one campaign never reads, replaces, undoes or
//!   clears another campaign's result, whatever the owner's `PartialEq` compares.
//! - **Its order.** [`Results::record`] issues the [`ResultOrder`] when the action is done. The
//!   order lives in the entry, and it leaves the store only inside the [`Shown`] view, as the
//!   ticket of the typed footer line it makes ([`Shown::outcome`], `outcome::Messages::kept`). A
//!   caller never holds a bare order, so it cannot keep orders by kind and show one result with
//!   another's order. A draw, which has `&Results`, cannot issue or renew an order.
//! - **Its end events.** [`Kept::ENDS`] says, for every end event, what it does to this kind.
//!   An Undo leaves the store only when it works: [`Results::undo`] keeps the entry on `Err`.
//!
//! **The one escape** is interior mutability: a store inside a `RefCell`, `Cell`, `Mutex` or
//! `RwLock` can be changed through `&`, so a draw could record. The source scan
//! `results_are_never_behind_interior_mutability` fails on such a store.
//!
//! Caller duties left: call the end events where they happen ([`Results::owner_changed`] after
//! a saved edit of the owner, [`Results::disk_changed`] after a reload or a file event).
//!
//! # What does not compile
//!
//! Every case has a twin that compiles and differs only in the marked line (rustc 1.95 does not
//! check a `compile_fail` test's error code, so the twin shows that the marked line is the
//! cause). The setup is shared:
//!
//! ```
//! use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
//! #[derive(Clone, PartialEq, Eq)]
//! struct Entry { id: u32, root: String }
//! impl Owner for Entry {
//!     type Scope = str;
//!     fn scope(&self) -> &str { &self.root }
//! }
//! struct Done { entry: Entry, words: String }
//! impl Kept for Done {
//!     type Owner = Entry;
//!     const ENDS: Ends = Ends {
//!         owner_changed: OwnerChanged::Ends,
//!         disk_changed: DiskChanged::Checked,
//!         press: Press::Undo,
//!     };
//!     fn owner(&self) -> Entry { self.entry.clone() }
//! }
//! let a = Entry { id: 1, root: "campaign".into() };
//! let mut results: Results<Entry, Done> = Results::new();
//! results.record(Done { entry: a.clone(), words: "Links now say “Ash Wolf”.".into() });
//! // The twin of each case below: a draw reads.
//! fn draw(results: &Results<Entry, Done>, a: &Entry) -> Option<String> {
//!     results.get(a).map(|shown| shown.value().words.clone())
//! }
//! assert!(draw(&results, &a).is_some());
//! let undone = results.undo(&a, |_| Ok::<(), String>(()));
//! assert!(matches!(undone, Some(Ok(_))));
//! ```
//!
//! A draw gets `&Results`, so it cannot record a result or issue its order:
//!
//! ```compile_fail
//! # use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
//! # #[derive(Clone, PartialEq, Eq)] struct Entry { id: u32, root: String }
//! # impl Owner for Entry { type Scope = str; fn scope(&self) -> &str { &self.root } }
//! # struct Done { entry: Entry, words: String }
//! # impl Kept for Done { type Owner = Entry; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Checked, press: Press::Undo }; fn owner(&self) -> Entry { self.entry.clone() } }
//! fn draw(results: &Results<Entry, Done>, a: &Entry) {
//!     results.record(Done { entry: a.clone(), words: String::new() }); // <- issues an order while drawing
//! }
//! ```
//!
//! Nor change a kept value in place (round 28, M3); the action does, with `&mut`:
//!
//! ```compile_fail
//! # use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
//! # #[derive(Clone, PartialEq, Eq)] struct Entry { id: u32, root: String }
//! # impl Owner for Entry { type Scope = str; fn scope(&self) -> &str { &self.root } }
//! # struct Done { entry: Entry, words: String }
//! # impl Kept for Done { type Owner = Entry; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Checked, press: Press::Undo }; fn owner(&self) -> Entry { self.entry.clone() } }
//! fn draw(results: &Results<Entry, Done>, a: &Entry) {
//!     let _ = results.revise(a, |done| done.words.clear()); // <- changes a result while drawing
//! }
//! ```
//!
//! Orders kept by kind (the fault of PR 298): a caller cannot take a bare order out of the
//! store, so it cannot keep one per kind and give A's result B's order:
//!
//! ```compile_fail
//! # use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
//! # #[derive(Clone, PartialEq, Eq)] struct Entry { id: u32, root: String }
//! # impl Owner for Entry { type Scope = str; fn scope(&self) -> &str { &self.root } }
//! # struct Done { entry: Entry, words: String }
//! # impl Kept for Done { type Owner = Entry; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Checked, press: Press::Undo }; fn owner(&self) -> Entry { self.entry.clone() } }
//! # let a = Entry { id: 1, root: "campaign".into() };
//! # let mut results: Results<Entry, Done> = Results::new();
//! # results.record(Done { entry: a.clone(), words: "A.".into() });
//! let mut orders = std::collections::HashMap::new();
//! orders.insert("links", results.get(&a).unwrap().order()); // <- an order kept by kind
//! ```
//!
//! An order is never made by hand:
//!
//! ```compile_fail
//! let order = splat_design::ownership::ResultOrder(7); // <- an order from a number
//! ```
//!
//! There is no way to take an entry out except its end events and its press, so a failed Undo
//! cannot drop the change (review of PR 299):
//!
//! ```compile_fail
//! # use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
//! # #[derive(Clone, PartialEq, Eq)] struct Entry { id: u32, root: String }
//! # impl Owner for Entry { type Scope = str; fn scope(&self) -> &str { &self.root } }
//! # struct Done { entry: Entry, words: String }
//! # impl Kept for Done { type Owner = Entry; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Checked, press: Press::Undo }; fn owner(&self) -> Entry { self.entry.clone() } }
//! # let a = Entry { id: 1, root: "campaign".into() };
//! # let mut results: Results<Entry, Done> = Results::new();
//! let taken = results.remove(&a); // <- takes the entry before the Undo is known to work
//! ```
//!
//! An Undo result is not used up by its press, and an offer has no Undo:
//!
//! ```compile_fail
//! # use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
//! # #[derive(Clone, PartialEq, Eq)] struct Entry { id: u32, root: String }
//! # impl Owner for Entry { type Scope = str; fn scope(&self) -> &str { &self.root } }
//! # struct Done { entry: Entry, words: String }
//! # impl Kept for Done { type Owner = Entry; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Checked, press: Press::Undo }; fn owner(&self) -> Entry { self.entry.clone() } }
//! # let a = Entry { id: 1, root: "campaign".into() };
//! # let mut results: Results<Entry, Done> = Results::new();
//! let used = results.use_press(&a); // <- an Undo result taken without its Undo
//! ```
//!
//! ```compile_fail
//! # use splat_design::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
//! # #[derive(Clone, PartialEq, Eq)] struct Entry { id: u32, root: String }
//! # impl Owner for Entry { type Scope = str; fn scope(&self) -> &str { &self.root } }
//! struct Offer { entry: Entry }
//! impl Kept for Offer { type Owner = Entry; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Checked, press: Press::Use }; fn owner(&self) -> Entry { self.entry.clone() } }
//! # let a = Entry { id: 1, root: "campaign".into() };
//! let mut offers: Results<Entry, Offer> = Results::new();
//! let undone = offers.undo(&a, |_| Ok::<(), String>(())); // <- an offer has no Undo
//! ```
//!
//! Each kind says what every end event does to it; a kind that leaves one out does not compile,
//! so a new end event makes every kind decide:
//!
//! ```compile_fail
//! # use splat_design::ownership::{DiskChanged, Ends, OwnerChanged};
//! const ENDS: Ends = Ends {
//!     owner_changed: OwnerChanged::Ends,
//!     disk_changed: DiskChanged::Checked,
//!     // <- `press` left out
//! };
//! ```
use super::components::ResultOrder;
use super::memo::Revision;
use super::outcome::Sentence;
use egui;
use std::time::Duration;

mod projection;
mod refusal;
#[cfg(any(test, feature = "test-support"))]
pub use projection::work as projection_work;
pub use projection::{Mismatch, Projection, Retained};
pub use refusal::{
    AtLimit, Cause, Change, FailedAct, Fallback, File, Folder, Kind, Limit, Name, Part, Recovery,
    Refusal, Rows, RunRule, Step, Thing, Way,
};

/// A new order for an act done now: a result recorded, an Undo attempt refused.
#[allow(
    clippy::disallowed_methods,
    reason = "the store is where an order is issued"
)]
fn act_order() -> ResultOrder {
    ResultOrder::issue()
}

/// A refused attempt kept on its entry: the attempt's own order (the entry keeps its ticket) and
/// the refusal.
struct Refused {
    order: ResultOrder,
    refusal: Refusal,
}

/// The object a kept result belongs to: an entry in its root, a note, a run step. The store keys
/// every lookup and every change by the owner's scope and the owner together, so an owner whose
/// `PartialEq` compares its id only still never meets another scope's result.
pub trait Owner: Clone + Eq {
    /// The campaign or content root the owner lives in. A result shows only in its own scope, so
    /// a campaign switch hides it and switching back shows it again.
    type Scope: Eq + ?Sized;
    fn scope(&self) -> &Self::Scope;
}

/// One kind of kept result.
pub trait Kept {
    type Owner: Owner;
    /// What each end event does to this kind. Every field is required: a new end event fails to
    /// compile in every kind until the kind says what the event does.
    const ENDS: Ends;
    /// The object this result belongs to. The store reads it once, when the result is recorded.
    fn owner(&self) -> Self::Owner;
    /// How long the result lives on the frame clock (`crate::frame_clock`) after its act, for a
    /// result that ends by itself (an Add's Undo). Such a kind is recorded with
    /// [`Results::record_timed`], which arms its end, and its owner ends it with
    /// [`Results::expire`] in its update, before any projection of it is made or shown. `None`:
    /// it has no timer, and it is recorded with [`Results::record`].
    const LIFE: Option<Duration> = None;
}

/// A kind of kept result that checks, at [`Results::reconcile`], the state its words and its
/// Undo rest on, and revises its own value (a clause that no longer holds ends; an Undo whose
/// files changed waits). `With` is what the check reads besides the value itself.
pub trait Reconcile {
    type With: ?Sized;
    fn reconcile(&mut self, with: &Self::With);
}

/// What each end event does to one kind of kept result. A new action of the same owner always
/// ends its earlier result ([`Results::record`]); a campaign switch always hides it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ends {
    /// The owner's own later action of another kind: a saved edit, a rename, a removal.
    pub owner_changed: OwnerChanged,
    /// A reload from disk or a file event.
    pub disk_changed: DiskChanged,
    /// Its own action is pressed.
    pub press: Press,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnerChanged {
    Ends,
    Stays,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiskChanged {
    Ends,
    /// It stays while the check that the event passes says it still holds (its files still
    /// hold what its action wrote).
    Checked,
    Stays,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Press {
    /// Its action is an Undo: the result leaves only when the Undo works ([`Results::undo`]).
    Undo,
    /// Its action uses it up, whatever the action's outcome: an offer ([`Results::use_press`]).
    Use,
    /// It has no action.
    Nothing,
}

/// A kept result as the store holds it: its value and the order its action issued. Only the
/// store makes one, and its order leaves it only as the ticket of its typed footer line
/// ([`Shown::outcome`]).
pub struct Shown<'a, T> {
    order: ResultOrder,
    value: &'a T,
    refused: Option<&'a Refused>,
    lock: Option<&'a Sentence>,
}

impl<'a, T> Shown<'a, T> {
    /// The order its action issued, for tests only. A caller gives the footer a `KeptLine`
    /// ([`Shown::outcome`]), so it cannot keep an order apart from its result.
    #[cfg(test)]
    pub(crate) fn order(&self) -> ResultOrder {
        self.order
    }

    /// The order as seen by a caller's tests in another crate.
    #[cfg(feature = "test-support")]
    pub fn order_for_caller_test(&self) -> ResultOrder {
        self.order
    }
    pub fn value(&self) -> &'a T {
        self.value
    }

    /// The refusal of this entry's last attempt, while it stands
    /// ([`Results::undo_or_refuse`]).
    pub fn refusal(&self) -> Option<&'a Refusal> {
        self.refused.map(|refused| &refused.refusal)
    }

    /// Why its Undo waits now, while it waits ([`Results::set_locks`]).
    pub fn lock(&self) -> Option<&'a Sentence> {
        self.lock
    }

    /// This result as a typed footer line for this frame's pane (package M), made from its own
    /// value and carrying this entry's [`Ticket`]. The call takes the view, so the store's
    /// borrow ends here: the pane's body may change the store or its owner. The line borrows the
    /// pane's local `frame`, so it cannot be kept past this frame. Give it to `Messages::kept`.
    pub fn outcome<'f, A>(
        self,
        frame: &'f super::outcome::PaneFrame,
        make: impl FnOnce(&T) -> super::outcome::KeptOutcome<A>,
    ) -> super::outcome::KeptLine<'f, A> {
        self.outcome_told_apart(frame, None, make)
    }

    /// `outcome`, with the place of a refused attempt's answer told apart by `apart`
    /// (`Refusal::words_told_apart`), for a reader that shows two answers that would read alike.
    pub fn outcome_told_apart<'f, A>(
        self,
        frame: &'f super::outcome::PaneFrame,
        apart: Option<&Name>,
        make: impl FnOnce(&T) -> super::outcome::KeptOutcome<A>,
    ) -> super::outcome::KeptLine<'f, A> {
        let refused = self.refused.map(|refused| {
            (
                refused.order,
                refused.refusal.words_told_apart(apart),
                refused.refusal.tone(),
            )
        });
        super::outcome::KeptLine::new(
            frame,
            Ticket { order: self.order },
            make(self.value),
            refused,
        )
    }
}

/// Names one kept entry of a store: the order its action issued, which no other entry of any
/// store has. Only a store's view makes one (`Shown::outcome`); it is not `Clone`, and its order
/// cannot be read. A press on a kept line comes back with it, and the store acts by it
/// ([`Results::undo_by`], [`Results::use_press_by`]): a replaced or ended entry is [`ByTicket::Gone`]
/// and nothing changes.
#[derive(Debug, PartialEq, Eq)]
pub struct Ticket {
    order: ResultOrder,
}

/// The owner's hold on the entry it just recorded ([`Results::record_held`], M0543): it revises
/// that entry, and only that one, on an event that is no press (a run's update, a file read),
/// with [`Results::revise_held`]. It is not a [`Ticket`]: no press, Undo or use goes by it, and
/// a replaced or ended entry is [`Gone`], so late work never changes a newer result of the same
/// owner, as a revision by owner (`revise`) can.
#[derive(Debug, PartialEq, Eq)]
pub struct Hold {
    order: ResultOrder,
}

impl Ticket {
    /// For the staged path (`Messages::outcome_at`) and the footer's control ids.
    pub(in crate::design) fn of(order: ResultOrder) -> Self {
        Self { order }
    }

    pub(in crate::design) fn order(&self) -> ResultOrder {
        self.order
    }
}

impl<T> Shown<'_, T> {
    /// This entry's ticket, for the store's own tests.
    #[cfg(test)]
    pub(in crate::design) fn ticket_for_test(&self, _frame: &super::outcome::PaneFrame) -> Ticket {
        Ticket { order: self.order }
    }

    /// This entry's ticket, for a caller's tests of its press.
    #[cfg(any(test, feature = "test-support"))]
    pub fn ticket_for_caller_test(&self) -> Ticket {
        Ticket { order: self.order }
    }
}

/// What a store did with a ticket.
#[derive(Debug, PartialEq, Eq)]
pub enum ByTicket<E> {
    /// The entry it named was replaced or ended: nothing changed.
    Gone,
    /// The entry is there, and its action failed: it stays.
    Failed(E),
}

/// A revision in place found no entry: the one it named was replaced or ended, and nothing
/// changed.
#[derive(Debug, PartialEq, Eq)]
pub struct Gone;

struct Entry<O, T> {
    owner: O,
    order: ResultOrder,
    value: T,
    /// The refusal of its last attempt ([`Results::undo_or_refuse`]): it ends with the entry, at
    /// the entry's next attempt, and at [`Results::end_refusals`].
    refused: Option<Refused>,
    /// When its act was done, on the frame clock, for a kind with a [`Kept::LIFE`].
    born: f64,
    /// Why its Undo waits now ([`Results::set_locks`]); `None` while it is open.
    lock: Option<Sentence>,
}

impl<O: Owner, T: Kept> Entry<O, T> {
    /// The same owner in the same scope. The scope is compared by the store, not left to the
    /// owner's `PartialEq`.
    fn is(&self, owner: &O) -> bool {
        self.owner.scope() == owner.scope() && self.owner == *owner
    }

    /// Its life ([`Kept::LIFE`]) has passed on the frame clock.
    fn ended(&self) -> bool {
        T::LIFE.is_some_and(|life| crate::frame_clock::passed(self.born, life))
    }
}

/// Every kept result of one kind, each with its owner and its order. See the module comment.
///
/// **Its revision** ([`Results::revision`]) names one version of the whole collection, for a
/// retained [`Projection`]: it advances at every change that a projection reads (an entry came
/// or left, a refusal was set or ended, a lock changed, a value was changed in place, a life
/// ended at [`Results::expire`]) and never at a read. A ticket's order is not this revision: a
/// refusal or a lock changes the words of an entry that keeps its ticket.
pub struct Results<O, T> {
    entries: Vec<Entry<O, T>>,
    revision: Revision,
    /// For a kind with a life: when its oldest entry's act was done (`None`: no entry), as
    /// worked out at a revision. Valid while that revision stands, so `expire` visits no entry
    /// on a frame before the next end.
    oldest: Option<(Revision, Option<f64>)>,
}

impl<O, T> Default for Results<O, T> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            revision: Revision::next(),
            oldest: None,
        }
    }
}

impl<O: Owner, T: Kept<Owner = O>> Results<O, T> {
    pub fn new() -> Self {
        Self::default()
    }

    /// The version of the whole collection. See the type's comment.
    pub fn revision(&self) -> Revision {
        self.revision
    }

    /// Something a projection reads changed.
    fn changed(&mut self) {
        self.revision = Revision::next();
    }

    /// `changed`, when the entries are fewer than `before`.
    fn changed_if_fewer(&mut self, before: usize) {
        if self.entries.len() != before {
            self.changed();
        }
    }

    /// Keep the result of an action done now. It gets a new order, newer than every result
    /// before it, and it ends its owner's earlier result of this kind. Call it where the action
    /// is done; Undo and then the same action again is a new action with a new order. A kind
    /// with a [`Kept::LIFE`] does not compile here: [`Results::record_timed`] arms its end.
    pub fn record(&mut self, value: T) {
        const {
            assert!(
                T::LIFE.is_none(),
                "this kind has a life: record it with `record_timed`, which arms its end"
            )
        };
        self.keep(value);
    }

    /// `record`, and the owner's [`Hold`] on the entry, for a later revision on an event that is
    /// no press ([`Results::revise_held`]): the Timeline's removal follows the run's updates.
    pub fn record_held(&mut self, value: T) -> Hold {
        const {
            assert!(
                T::LIFE.is_none(),
                "this kind has a life: record it with `record_timed`, which arms its end"
            )
        };
        self.keep(value);
        let order = self
            .entries
            .last()
            .map(|entry| entry.order)
            .expect("the entry just kept");
        Hold { order }
    }

    /// Change the value of the entry `hold` names, in place, as `revise_by` does for a ticket:
    /// the same entry, place and owner; only the value changes, and no order is issued. An
    /// entry replaced or ended since is [`Gone`], and `change` does not run.
    pub fn revise_held<R>(
        &mut self,
        hold: &Hold,
        change: impl FnOnce(&mut T) -> R,
    ) -> Result<R, Gone> {
        let entry = self
            .entries
            .iter_mut()
            .find(|entry| entry.order == hold.order)
            .ok_or(Gone)?;
        let done = change(&mut entry.value);
        self.changed();
        Ok(done)
    }

    /// `record` for a kind with a [`Kept::LIFE`]: it also asks `ctx` for a repaint at its end
    /// ([`Results::expire`]), so a result recorded after its owner's update this frame still
    /// ends with no input.
    pub fn record_timed(&mut self, value: T, ctx: &egui::Context) {
        const {
            assert!(
                T::LIFE.is_some(),
                "this kind has no life: record it with `record`"
            )
        };
        self.keep(value);
        self.expire(ctx);
    }

    fn keep(&mut self, value: T) {
        let owner = value.owner();
        self.entries.retain(|entry| !entry.is(&owner));
        let order = act_order();
        self.entries.push(Entry {
            owner,
            order,
            value,
            refused: None,
            born: crate::frame_clock::now(),
            lock: None,
        });
        self.changed();
    }

    /// The owner's update for a kind with a [`Kept::LIFE`]: each result whose life passed on the
    /// frame clock ends, and the revision advances only when one ended. The soonest end left is
    /// asked of `ctx` as a repaint, so the result ends with no input. Call it before any
    /// projection of the store is made or shown ([`Retained::refresh`], [`Results::present`]).
    ///
    /// Its cost on a frame before the next end is constant: the oldest act is kept for the
    /// store's revision, and no entry is visited. A frame at or after an end visits each entry
    /// once.
    pub fn expire(&mut self, ctx: &egui::Context) {
        let Some(life) = T::LIFE else {
            return;
        };
        let mut oldest = self.oldest_born();
        if oldest.is_some_and(|born| crate::frame_clock::passed(born, life)) {
            // One visit of each entry: the ended leave, and the next oldest is found among those
            // that stay.
            let before = self.entries.len();
            let mut next: Option<f64> = None;
            self.entries.retain(|entry| {
                let stays = !entry.ended();
                if stays {
                    next = Some(next.map_or(entry.born, |next| next.min(entry.born)));
                }
                stays
            });
            projection::count(|work| work.visits += before);
            self.changed_if_fewer(before);
            self.oldest = Some((self.revision, next));
            oldest = next;
        }
        if let Some(born) = oldest {
            let left = life.as_secs_f64() - (crate::frame_clock::now() - born);
            ctx.request_repaint_after(Duration::from_secs_f64(left.max(0.0)));
        }
    }

    /// When the oldest entry's act was done: kept for the store's revision, worked out again
    /// (one visit of each entry) only after a change.
    fn oldest_born(&mut self) -> Option<f64> {
        match self.oldest {
            Some((at, born)) if at == self.revision => born,
            _ => {
                projection::count(|work| work.visits += self.entries.len());
                let born = self
                    .entries
                    .iter()
                    .map(|entry| entry.born)
                    .min_by(f64::total_cmp);
                self.oldest = Some((self.revision, born));
                born
            }
        }
    }

    /// The owner's locks: `lock` says, for each result, why its Undo waits now (`None`: it is
    /// open). The revision advances only when one differs from what the store holds. A
    /// projection reads it through [`Shown::lock`]. It visits every entry: call it in the
    /// owner's update when the lock's source changed (its own revision), not on every frame.
    pub fn set_locks(&mut self, mut lock: impl FnMut(&T) -> Option<Sentence>) {
        let mut changed = false;
        for entry in &mut self.entries {
            let now = lock(&entry.value);
            if entry.lock != now {
                entry.lock = now;
                changed = true;
            }
        }
        if changed {
            self.changed();
        }
    }

    /// The owner's result, if it has one.
    pub fn get(&self, owner: &O) -> Option<Shown<'_, T>> {
        self.entries
            .iter()
            .find(|entry| entry.is(owner))
            .map(Self::shown)
    }

    /// The entry `ticket` names, if it still stands.
    pub fn get_by(&self, ticket: &Ticket) -> Option<Shown<'_, T>> {
        self.entries
            .iter()
            .find(|entry| entry.order == ticket.order)
            .map(Self::shown)
    }

    /// The results of one scope, newest first.
    pub fn newest_first<'a>(&'a self, scope: &'a O::Scope) -> impl Iterator<Item = Shown<'a, T>> {
        // `record` appends, so the entries are oldest first.
        self.entries
            .iter()
            .rev()
            .filter(move |entry| entry.owner.scope() == scope)
            .map(Self::shown)
    }

    fn shown(entry: &Entry<O, T>) -> Shown<'_, T> {
        Shown {
            order: entry.order,
            value: &entry.value,
            refused: entry.refused.as_ref(),
            lock: entry.lock.as_ref(),
        }
    }

    fn position(&self, owner: &O) -> Option<usize> {
        self.entries.iter().position(|entry| entry.is(owner))
    }

    /// Undo the owner's result with `run`. The result leaves the store only when `run` returns
    /// `Ok`; then it is returned with `run`'s value. On `Err` it stays, with its order, so its
    /// Undo can be pressed again. `None`: the owner has no result.
    pub fn undo<R, E>(
        &mut self,
        owner: &O,
        run: impl FnOnce(&T) -> Result<R, E>,
    ) -> Option<Result<(T, R), E>> {
        const {
            assert!(
                matches!(T::ENDS.press, Press::Undo),
                "this kind has no Undo: its press is not `Press::Undo`"
            )
        };
        let at = self.position(owner)?;
        Some(run(&self.entries[at].value).map(|done| {
            self.changed();
            (self.entries.remove(at).value, done)
        }))
    }

    /// The Undo of the entry `ticket` names, as `undo` does by owner: it leaves only when `run`
    /// works. An entry replaced or ended since the ticket was made is `Gone`, and `run` does not
    /// run.
    pub fn undo_by<R, E>(
        &mut self,
        ticket: Ticket,
        run: impl FnOnce(&T) -> Result<R, E>,
    ) -> Result<(T, R), ByTicket<E>> {
        const {
            assert!(
                matches!(T::ENDS.press, Press::Undo),
                "this kind has no Undo: its press is not `Press::Undo`"
            )
        };
        let at = self
            .entries
            .iter()
            .position(|entry| entry.order == ticket.order)
            .ok_or(ByTicket::Gone)?;
        match run(&self.entries[at].value) {
            Ok(done) => {
                self.changed();
                Ok((self.entries.remove(at).value, done))
            }
            Err(error) => Err(ByTicket::Failed(error)),
        }
    }

    /// The opt-in recorded Undo: as [`Results::undo_by`], and a refused attempt is kept. On
    /// `Err` the entry stays with its ticket (its controls keep their ids, a press made on its
    /// line still reaches it) and holds the refusal with a new order of its own, the attempt's,
    /// which ranks its answer (ruling R33). Its line then opens with the refusal's words. The
    /// refusal ends with the entry (a successful Undo, a new result of the same owner, an end
    /// event), at the entry's next attempt, and at [`Results::end_refusals`].
    pub fn undo_or_refuse<R>(
        &mut self,
        ticket: Ticket,
        run: impl FnOnce(&T) -> Result<R, Refusal>,
    ) -> Result<(T, R), ByTicket<()>> {
        const {
            assert!(
                matches!(T::ENDS.press, Press::Undo),
                "this kind has no Undo: its press is not `Press::Undo`"
            )
        };
        let at = self
            .entries
            .iter()
            .position(|entry| entry.order == ticket.order)
            .ok_or(ByTicket::Gone)?;
        let result = run(&self.entries[at].value);
        let order = act_order();
        // Either way its line changes: it goes, or it opens with the refusal's words.
        self.changed();
        match result {
            Ok(done) => Ok((self.entries.remove(at).value, done)),
            Err(refusal) => {
                self.entries[at].refused = Some(Refused { order, refusal });
                Err(ByTicket::Failed(()))
            }
        }
    }

    /// The state a result rests on changed (its file's words, its draft). `holds` revises each
    /// value in place (its Undo may wait) and says whether its result still holds. One that does
    /// not hold ends, unless it holds a refused attempt: that answer to the DM's act is not ended
    /// by the change that caused it; its own events end it. Call it at the change, never each
    /// frame.
    pub fn source_changed(&mut self, mut holds: impl FnMut(&mut T) -> bool) {
        if self.entries.is_empty() {
            return;
        }
        self.entries
            .retain_mut(|entry| holds(&mut entry.value) || entry.refused.is_some());
        // `holds` may revise a value in place, which the store cannot see.
        self.changed();
    }

    /// The refused attempt of each entry for which `ends` is true ends (its object went away;
    /// another act on its owner). The entry itself then stays only while `holds` says its result
    /// still holds. How many refusals ended.
    pub fn end_refusals(
        &mut self,
        mut ends: impl FnMut(&T) -> bool,
        mut holds: impl FnMut(&T) -> bool,
    ) -> usize {
        let mut ended = 0;
        self.entries.retain_mut(|entry| {
            if entry.refused.is_some() && ends(&entry.value) {
                entry.refused = None;
                ended += 1;
                return holds(&entry.value);
            }
            true
        });
        if ended > 0 {
            self.changed();
        }
        ended
    }

    /// The offer `ticket` names, used up by its press; `None` when it was replaced or ended.
    pub fn use_press_by(&mut self, ticket: Ticket) -> Option<T> {
        const {
            assert!(
                matches!(T::ENDS.press, Press::Use),
                "this kind is not used up by its press: its press is not `Press::Use`"
            )
        };
        let at = self
            .entries
            .iter()
            .position(|entry| entry.order == ticket.order)?;
        self.changed();
        Some(self.entries.remove(at).value)
    }

    /// The owner's offer, used up by its press whatever the action's outcome.
    pub fn use_press(&mut self, owner: &O) -> Option<T> {
        const {
            assert!(
                matches!(T::ENDS.press, Press::Use),
                "this kind is not used up by its press: its press is not `Press::Use`"
            )
        };
        let at = self.position(owner)?;
        self.changed();
        Some(self.entries.remove(at).value)
    }

    /// Change the value of the entry `ticket` names, in place: the same entry, so the same
    /// ticket, the same order and place among the others, the same owner (the store keys by the
    /// owner it read at `record`, which `change` cannot reach), and every end event of its kind.
    /// Only the value changes. A press made on a line before the revision still reaches it, and
    /// the press's action gets the revised value. An entry replaced or ended since is [`Gone`],
    /// and `change` does not run. It issues no order: a revision is not a new action.
    pub fn revise_by<R>(
        &mut self,
        ticket: &Ticket,
        change: impl FnOnce(&mut T) -> R,
    ) -> Result<R, Gone> {
        let entry = self
            .entries
            .iter_mut()
            .find(|entry| entry.order == ticket.order)
            .ok_or(Gone)?;
        let done = change(&mut entry.value);
        // A change in place, which the store cannot see: its words may differ.
        self.changed();
        Ok(done)
    }

    /// `revise_by` for the owner's entry, whichever it is now. [`Gone`] when the owner has none.
    /// Banned outside `src/design/` and counted (`Ban::ByOwner(n)`), as `undo` is: late work
    /// that revises by owner changes a newer result of that owner. Use it only for a check made
    /// in the same frame as the read it follows (Sessions' `end_what_changed` holds only the
    /// session, not a ticket).
    pub fn revise<R>(&mut self, owner: &O, change: impl FnOnce(&mut T) -> R) -> Result<R, Gone> {
        let entry = self
            .entries
            .iter_mut()
            .find(|entry| entry.is(owner))
            .ok_or(Gone)?;
        let done = change(&mut entry.value);
        self.changed();
        Ok(done)
    }

    /// `revise_by` for every entry whose value `holds` (a note file moved, and each Undo that
    /// names it follows it), in every scope; how many were revised. Banned and counted as
    /// `revise` is (Sessions' `Carries::moved` holds paths, not tickets).
    pub fn revise_where(
        &mut self,
        mut holds: impl FnMut(&T) -> bool,
        mut change: impl FnMut(&mut T),
    ) -> usize {
        let mut revised = 0;
        for entry in &mut self.entries {
            if holds(&entry.value) {
                change(&mut entry.value);
                revised += 1;
            }
        }
        if revised > 0 {
            self.changed();
        }
        revised
    }

    /// The one event for a kind whose words or Undo rest on state that a later write or a
    /// reload can change (R11: a copy's registration says what the campaign chose, until the
    /// copy's use changes). Each entry checks that state itself ([`Reconcile`]) and is revised
    /// in place through [`Results::revise_by`]: the same entry, ticket, order and Undo. The
    /// caller picks no entry, so no predicate reaches a newer result. Call it when the state's
    /// generation changes (a reload), never each frame: a check may read files.
    pub fn reconcile(&mut self, with: &T::With)
    where
        T: Reconcile,
    {
        let tickets: Vec<Ticket> = self
            .entries
            .iter()
            .map(|entry| Ticket { order: entry.order })
            .collect();
        for ticket in &tickets {
            let _ = self.revise_by(ticket, |value| value.reconcile(with));
        }
    }

    /// The owner changed by its own later action (a saved edit, a rename, a removal).
    pub fn owner_changed(&mut self, owner: &O) {
        let before = self.entries.len();
        match T::ENDS.owner_changed {
            OwnerChanged::Ends => self.entries.retain(|entry| !entry.is(owner)),
            OwnerChanged::Stays => {}
        }
        self.changed_if_fewer(before);
    }

    /// Each result whose owner can never come back (a dropped draft that no Undo can restore)
    /// ends, in every scope. `reachable` is asked once per result, with its value.
    pub fn end_unreachable(&mut self, mut reachable: impl FnMut(&T) -> bool) {
        let before = self.entries.len();
        self.entries.retain(|entry| reachable(&entry.value));
        self.changed_if_fewer(before);
    }

    /// The files changed on disk (a reload, a file event). `holds` says whether a result still
    /// holds; it is asked only for a kind whose `disk_changed` is `Checked`.
    pub fn disk_changed(&mut self, mut holds: impl FnMut(&T) -> bool) {
        let before = self.entries.len();
        match T::ENDS.disk_changed {
            DiskChanged::Ends => self.entries.clear(),
            DiskChanged::Checked => self.entries.retain(|entry| holds(&entry.value)),
            DiskChanged::Stays => {}
        }
        self.changed_if_fewer(before);
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Every suppression of `clippy::disallowed_methods` that may stay, one row per suppression:
/// the file, the item it is on (a function's name, or `<function or macro!> statement`), and the
/// bans it is for. Clippy's `disallowed-methods` (`clippy.toml`) fails `check` on any call of
/// `ResultOrder::issue`, `Footer::result` and `save_row_state`, and of package M's escapes and
/// helpers (`Ban::Words`, `Ban::Sentence`, `Ban::Order`, `Ban::Body`), by any name (an alias, a
/// function item, a qualified path); a suppression frees only what it sits on. `Ban::Words` and
/// `Ban::Body` are named escapes: their rows need not reach 0, but each new one is a visible row.
///
/// `staged_allows_only_shrink` finds every suppression (`allow` or `expect`, any reason or none,
/// also in `cfg_attr`, also of a group that holds the lint) and fails on one that is not listed,
/// on a file, module, `impl`, trait or macro, or on a function that holds a nested function. A
/// listed one whose item no longer calls, or a row with no suppression, asks to remove the row.
/// The rows can only go; at none with `Ban::Results`, those parts become private.
///
/// **Package K.** K bans raw egui calls through the same lint (`Ban::Layout(n)`, `Ban::RowKey(n)`, each with the number of calls its item holds). Its staged rows are
/// here; where one function holds both kinds of call, that function keeps one attribute and its
/// row lists both bans. The owners of the raw calls, `src/design/` and the capture harness
/// `src/visual_tests/`, list theirs in [`OWNED_ALLOWS`], which never shrinks to none.
#[cfg(test)]
pub(crate) const STAGED_ALLOWS: &[(&str, &str, &[suppressions::Ban])] = {
    use suppressions::Ban;
    &[
        // Campaign content (Wave 2 d, C): body faults with their repairs, and outside words.
        ("src/campaign_content.rs", "cause", &[Ban::Words]),
        ("src/campaign_content.rs", "draw_reason", &[Ban::Body]),
        ("src/campaign_content.rs", "outcome_of", &[Ban::Words]),
        ("src/campaign_content.rs", "packages_content", &[Ban::Body]),
        ("src/campaign_content.rs", "page", &[Ban::Body]),
        // Ruling Q-x: the answer to the DM's last act leads the footer.
        ("src/campaign_content.rs", "page_footer", &[Ban::Answer]),
        (
            "src/campaign_content.rs",
            "repair_section_content",
            &[Ban::Extras(1), Ban::Body],
        ),
        (
            "src/campaign_content.rs",
            "selection_bar_content",
            &[Ban::Results],
        ),
        (
            "src/campaigns.rs",
            "footer_messages",
            // Layout PR 2b: typed lines; the page's lock words are app-owned `&str`.
            &[Ban::Words],
        ),
        ("src/campaigns.rs", "form", &[Ban::Results]),
        // M20: the first campaign's form has no footer; its error is a typed body fault in
        // place of `design::message` (`draw_error`, gone).
        ("src/campaigns.rs", "form_error_inline", &[Ban::Body]),
        // The design system's pane tests with closure lines (the `#[cfg(test)]` `Footer::result`,
        // `legacy_reason`) and a footer placed by hand (migration-32 L2c: no app code can).
        (
            "src/design/components.rs",
            "iter26_a_the_footer_keeps_the_body_top_and_reaches_its_last_message",
            &[Ban::Results, Ban::Layout(1), Ban::Pane(1), Ban::Footer],
        ),
        (
            "src/design/components.rs",
            "iter27_a_footer_results_go_newest_first_by_their_action",
            &[Ban::Results, Ban::Footer],
        ),
        (
            "src/design/components.rs",
            "iter27_a_the_footer_follows_a_short_body",
            &[Ban::Results, Ban::Layout(1), Ban::Pane(1), Ban::Footer],
        ),
        (
            "src/design/gallery.rs",
            "components",
            &[Ban::Results, Ban::Layout(4)],
        ),
        ("src/design/results.rs", "act_order", &[Ban::Results]),
        // Layout PR 2b: the one escape of the Settings and Link and folder footers' words.
        ("src/campaigns.rs", "footer_words", &[Ban::Words]),
        // Arrivals' list keeps its own scroll under a `Fills` body (migration-32 L2c), a design
        // `ScrollColumn` since layout step 11a (no raw scroll area is left).
        ("src/encounter_run/view.rs", "groups_tab", &[Ban::Pane(1)]),
        (
            "src/encounters/groups.rs",
            "group_editor_view",
            &[Ban::Results, Ban::Pane(1)],
        ),
        (
            "src/encounters.rs",
            "creature_pane",
            &[Ban::Results, Ban::Pane(1), Ban::RowKey(1)],
        ),
        ("src/encounters.rs", "run_settings", &[Ban::Results]),
        (
            "src/design/results.rs",
            "a_failed_undo_keeps_its_entry_and_its_order",
            &[Ban::ByOwner(3)],
        ),
        (
            "src/design/results.rs",
            "end_events_do_what_the_kind_says",
            &[Ban::ByOwner(1)],
        ),
        // The part's own tests of the by-owner revisions (M3), and the pilot fixture's.
        (
            "src/design/results.rs",
            "a_ticket_reaches_only_its_own_entry",
            &[Ban::ByOwner(4)],
        ),
        ("src/design/outcome.rs", "show_active", &[Ban::ByOwner(1)]),
        (
            "src/design/results.rs",
            "a_result_shows_only_in_its_own_scope statement",
            &[Ban::ByOwner(1)],
        ),
        (
            "src/design/results.rs",
            "a_result_shows_only_in_its_own_scope statement",
            &[Ban::ByOwner(1)],
        ),
        // M20 (M-L0.6): the disk-change warnings above the comparison, typed placeholders.
        ("src/library.rs", "editor_pane", &[Ban::Body]),
        // M20 (M-L0.6): a read fault in the reader's body, with `Try again`.
        ("src/library/catalogue.rs", "fault_notice", &[Ban::Body]),
        // M20 (M-L0.6): a copied package folder's warnings in the form, with its repair.
        ("src/library.rs", "package_form", &[Ban::Body]),
        // M20 (M-L0.6): a failed load where the page has no pane.
        ("src/library.rs", "show statement", &[Ban::Body]),
        ("src/library/catalogue.rs", "field_key", &[Ban::RowKey(1)]),
        // M17 (M-L0.4b): the page's error, the system's or the workspace's words.
        ("src/library/catalogue.rs", "cause", &[Ban::Words]),
        // M17 (M-L0.4b): a manifest write's words, which start with the file's path from disk.
        ("src/library.rs", "outcome_of", &[Ban::Words]),
        ("src/party.rs", "sheet_view", &[Ban::Body]),
        // R24: a value block's save row shows its own messages (the Session folder's).
        ("src/sessions/page.rs", "folder_block", &[Ban::Body]),
        // M-N0.6 (M20): typed body places, each in place of an untyped body message.
        ("src/sessions/page.rs", "show_form", &[Ban::Body]),
        ("src/sessions/page.rs", "notes_panel", &[Ban::Body]),
        ("src/sessions/page.rs", "show", &[Ban::Body]),
        // M-N0.10: the record line's warning (M0453).
        ("src/sessions/page.rs", "record_line_body", &[Ban::Body]),
        (
            "src/sessions/page.rs",
            "banner_notices",
            &[Ban::Results, Ban::Body],
        ),
        ("src/sessions/page.rs", "markers", &[Ban::Results]),
        (
            "src/sessions/page.rs",
            "reading_view",
            &[Ban::Results, Ban::Body],
        ),
        ("src/sessions/page.rs", "session_messages", &[Ban::Order]),
        // M-N0.4: the template's text field is keyed by its file (`TextKey::from_data`): a
        // template has no typed owner key yet.
        (
            "src/sessions/page.rs",
            "template_editor",
            &[Ban::Results, Ban::RowKey(1)],
        ),
        (
            "src/sessions/plan.rs",
            "plan_pane",
            // `ByOwner`: `checks::Results::undo(id, book)`, not the footer store's; counted by name.
            // `Body` (M-N1.5, M20): a planned group's member faults in its row, typed.
            &[Ban::Body, Ban::ByOwner(1)],
        ),
        // M-N1.3 (ruling M20): Read's embed whose source is gone, typed (`BodyMessage`), in
        // place of a `small_message`.
        ("src/sessions/carry.rs", "show_prepared", &[Ban::Body]),
        // M-N1.4 (ruling M20): a scene act's typed fault where the DM acted, in place of a
        // `design::message`.
        ("src/sessions/plan.rs", "row_fault", &[Ban::Body]),
        // M-N1.5 (M20): the open played fight's fault in its details, typed, in place of two
        // `design::message` calls; `/npc`'s fault in its menu, in place of two more.
        ("src/sessions/plan.rs", "fight_block", &[Ban::Body]),
        ("src/sessions/slash.rs", "show", &[Ban::Body]),
        // M-N1.5 (M17): `/npc`'s fault carries the generator's and the system's words.
        ("src/sessions/slash.rs", "cause", &[Ban::Words]),
        // M-N1.1 (ruling M20): a result with the note's Undo at its own place
        // (`PlacedResult`), replacing three `undo_message` calls.
        (
            "src/sessions/plan.rs",
            "removed_scene_message",
            &[Ban::Body],
        ),
        ("src/sessions/plan.rs", "field_result", &[Ban::Body]),
        (
            "src/sessions/plan.rs",
            "removed_fight_message",
            &[Ban::Body],
        ),
        ("src/world/edit.rs", "edit_form", &[Ban::Results]),
        // M20 (M-W0.4): the creation error in place of the page's body (was
        // `message_with_actions`).
        ("src/world/page.rs", "creation_body", &[Ban::Body]),
        // Package K: raw egui calls that a layout part replaces.
        (
            "src/campaigns.rs",
            "selector",
            &[Ban::Layout(1), Ban::Extras(1)],
        ),
        // M20: the copy dialog's error is a typed body message (`BodyMessage` of a
        // `FaultMessage`) in place of an untyped `design::message`.
        ("src/copy_dialog.rs", "show", &[Ban::Body]),
        (
            "src/encounter_run/ad_hoc.rs",
            "add_creature_form",
            &[Ban::Layout(1)],
        ),
        (
            "src/encounter_run/dock.rs",
            "effect_line",
            &[Ban::Extras(1)],
        ),
        (
            "src/encounter_run/view.rs",
            "conditions_tab",
            &[Ban::RowKey(1)],
        ),
        ("src/encounter_run/view.rs", "record_tab", &[Ban::RowKey(1)]),
        ("src/encounter_run/view.rs", "roster", &[Ban::Layout(1)]),
        (
            "src/encounter_run/view.rs",
            "setup_body",
            &[Ban::Layout(1), Ban::Pane(1), Ban::Body],
        ),
        ("src/encounter_run/view.rs", "setup_row", &[Ban::Extras(1)]),
        // The editor's columns (`Fills::because`). Its unread rows' warning is a typed fault
        // with its file line since migration-32 L2c.
        ("src/encounters.rs", "editor_view", &[Ban::Pane(1)]),
        ("src/encounters.rs", "roster", &[Ban::RowKey(1)]),
        ("src/encounters/groups.rs", "members", &[Ban::Body]),
        (
            "src/encounters/groups.rs",
            "members_pane",
            &[Ban::Layout(1)],
        ),
        ("src/loot_editor.rs", "add_row", &[Ban::Extras(1)]),
        (
            "src/markdown_view.rs",
            "wide_tables_keep_whole_words_and_can_scroll_to_the_last_column",
            &[Ban::Layout(1)],
        ),
        ("src/sessions/page.rs", "banner", &[Ban::Body]),
        ("src/sessions/page.rs", "note_editor", &[Ban::Body]),
        (
            "src/timeline/page.rs",
            "detail",
            // `ByOwner`: `checks::Results::undo(id, book)`, not the footer store's; counted by name.
            &[Ban::ByOwner(1)],
        ),
        ("src/trackers/view.rs", "amount_field", &[Ban::RowKey(3)]),
        ("src/world/repair.rs", "source_editor", &[Ban::RowKey(1)]),
        // Package M (round 28): the escapes of `design::outcome`, and its own helpers.
        ("src/design/outcome.rs", "__literal", &[Ban::Sentence]),
        ("src/design/outcome.rs", "check_literal", &[Ban::Sentence]),
        (
            "src/design/outcome.rs",
            "sentence! statement",
            &[Ban::Sentence],
        ),
        (
            "src/design/outcome.rs",
            "sentence! statement",
            &[Ban::Sentence],
        ),
        (
            "src/design/outcome.rs",
            "sentence! statement",
            &[Ban::Sentence],
        ),
        (
            "src/design/outcome.rs",
            "choice! statement",
            &[Ban::Sentence],
        ),
        (
            "src/design/outcome.rs",
            "choice! statement",
            &[Ban::Sentence],
        ),
        // M4: the macro's own check of its lead words.
        ("src/design/outcome.rs", "lead! statement", &[Ban::Sentence]),
        ("src/design/outcome.rs", "rejected", &[Ban::Sentence]),
        (
            "src/design/outcome.rs",
            "a_template_is_a_sentence_in_the_dms_words",
            &[Ban::Sentence],
        ),
        (
            "src/design/outcome.rs",
            "outside_words_get_a_capital_and_an_end_mark",
            &[Ban::Words],
        ),
        (
            "src/design/outcome.rs",
            "one_fault_has_one_tone_in_every_place",
            &[Ban::Body],
        ),
        ("src/encounter_run/view.rs", "outside_words", &[Ban::Words]),
        ("src/encounters.rs", "cause", &[Ban::Words]),
        ("src/encounters.rs", "kept_outcome", &[Ban::Words]),
        ("src/encounters.rs", "outcome_words", &[Ban::Words]),
        // Ruling M20 (M0178): the duplicate repair in the search results; it replaces an
        // untyped `design::message_with_actions`.
        (
            "src/encounters.rs",
            "duplicate_creature_results",
            &[Ban::Body],
        ),
        // Ruling M20 (M0192): the page's placeholder while another campaign's encounter is open;
        // it replaces an untyped `design::message`.
        ("src/encounters.rs", "show_page", &[Ban::Body]),
        // M0126 (M-R0.9): a group row's member faults are typed messages in the row they are
        // about, with their repairs (`BodyMessage`); their words are the loader's, with the
        // DM's names in them (`Sentence::outside`).
        ("src/content_ui/src/roster.rs", "cause", &[Ban::Words]),
        ("src/encounters/groups.rs", "member_fault_row", &[Ban::Body]),
        // Ruling M20 (lane E1): the group editor's faults, typed; each replaces an untyped
        // `design::message` or `small_message`.
        ("src/encounters/groups.rs", "belongs_to", &[Ban::Body]),
        ("src/encounters/groups.rs", "body_fault", &[Ban::Body]),
        // Layout 11c (L11c-E.2): a change's fault under its own field (was under the row's
        // `Extras` allow with its raw space).
        ("src/encounters/groups.rs", "change_row", &[Ban::Body]),
        // Ruling M20 (lane E2): a draft's disk conflict takes its editor's place (the
        // comparison follows), typed; it replaces `design::message` and `message_with_actions`.
        ("src/encounters/disk.rs", "show", &[Ban::Body]),
        (
            "src/design/outcome.rs",
            "a_body_fault_gives_its_repairs_back_by_id",
            &[Ban::Body],
        ),
        (
            "src/design/outcome.rs",
            "a_placed_result_shows_its_undo_whole_where_it_fits",
            &[Ban::Body],
        ),
        // N0 (ruling M21): a message holds at most one primary choice.
        (
            "src/design/outcome.rs",
            "a_message_has_at_most_one_primary_choice",
            &[Ban::Body],
        ),
        ("src/processing/settings.rs", "cause", &[Ban::Words]),
        ("src/processing/settings.rs", "key_fault", &[Ban::Body]),
        // M-U0.1 (rulings M17, M20): the account's warning, in the keyring's words when it is not
        // the unsaved sign-in; it replaces `message_with_action` and `message` in `show_account`.
        (
            "src/settings.rs",
            "account_warning",
            &[Ban::Body, Ban::Words],
        ),
        // M-U0.1 (rulings M17, M20): a table marker's warning, in its parser's words, under the
        // table in the reader's body; it replaces two `design::message` calls in `roll_control`.
        (
            "src/roll_table.rs",
            "table_warning",
            &[Ban::Body, Ban::Words],
        ),
        ("src/timeline/checks.rs", "cause", &[Ban::Words]),
        ("src/timeline/page.rs", "as_sentence", &[Ban::Words]),
        ("src/sessions/plan.rs", "as_sentence", &[Ban::Words]),
        ("src/timeline/checks.rs", "check_fault", &[Ban::Body]),
        ("src/timeline/checks.rs", "check_notice", &[Ban::Body]),
        // Ruling M20 (lane T0): Timeline.md's problems in the page's place and the session date
        // notice, typed; each replaces `design::message`, `message_with_actions` or
        // `message_with_action`.
        ("src/timeline/page.rs", "problems", &[Ban::Body]),
        ("src/timeline/page.rs", "dated_notice", &[Ban::Body]),
        ("src/trackers.rs", "act_words", &[Ban::Words]),
        ("src/world/repair.rs", "cause", &[Ban::Words]),
        ("src/world.rs", "said", &[Ban::Words]),
        ("src/combat_sync.rs", "cause", &[Ban::Words]),
        ("src/party.rs", "cause", &[Ban::Words]),
        ("src/trackers.rs", "cause", &[Ban::Words]),
        ("src/trackers/history.rs", "show", &[Ban::Body]),
        ("src/trackers/view.rs", "body", &[Ban::Body]),
        // Package T (PR 328 review): a repeated amount field's problem stands at its line.
        ("src/trackers/view.rs", "tracker_lines", &[Ban::Body]),
        (
            "src/content_ui/src/session_notes.rs",
            "notice_sentence",
            &[Ban::Words],
        ),
        (
            "src/encounter_run/notice.rs",
            "live_header",
            &[Ban::Extras(2)],
        ),
        ("src/encounter_run/view.rs", "live_view", &[Ban::Pane(1)]),
        // M-P0 item 7: the run's notices in the compact typed form, where they stand.
        ("src/encounter_run/view.rs", "live_notice", &[Ban::Body]),
        ("src/encounter_run/notice.rs", "header_notice", &[Ban::Body]),
        ("src/encounter_run/view.rs", "toggle", &[Ban::Extras(1)]),
        ("src/sessions/plan.rs", "plan_row_with", &[Ban::Extras(1)]),
        ("src/encounters/list.rs", "heading", &[Ban::Extras(1)]),
        (
            "src/encounters/groups.rs",
            "conditions_row",
            &[Ban::RowKey(1)],
        ),
        (
            "src/encounters/groups.rs",
            "overrides_form",
            &[Ban::RowKey(2)],
        ),
        ("src/encounters/groups.rs", "unread_member", &[Ban::Body]),
        // M20 (M-L0.6): a repair draft's fault in the reader's body, with its repair.
        ("src/library/catalogue.rs", "repair_notice", &[Ban::Body]),
        (
            "src/library/catalogue.rs",
            "use_checkbox",
            &[Ban::Extras(1)],
        ),
        ("src/sessions/page.rs", "draft_warning", &[Ban::Body]),
        ("src/sessions/page.rs", "recording_row", &[Ban::Body]),
        (
            "src/sessions/plan.rs",
            "gone_scene_words",
            &[Ban::Extras(1)],
        ),
        ("src/sessions/plan.rs", "plan_details", &[Ban::Extras(1)]),
        ("src/sessions/plan.rs", "scene_words", &[Ban::Extras(1)]),
        // M20 (M-W0.4, M-W0.5, M0178): the index's warnings on this note, the renamed-file
        // warning with its two repairs and the duplicate repair, in the reader's body (were
        // `message`, `message_with_actions`).
        ("src/world.rs", "draw", &[Ban::Body]),
        // M20 (M-W0.4): an unread value's warning (was `message`).
        ("src/world.rs", "unread_warning", &[Ban::Body]),
        // M20 (M-W0.5): an unread value's warning with its repair, `Edit in Obsidian` (was
        // `message` and a link).
        ("src/world.rs", "unread_groups_message", &[Ban::Body]),
        ("src/lookup.rs", "written_reader", &[Ban::Extras(1)]),
        // M20 (M-W0.5): a note that cannot be read, in place of its page (was
        // `message_with_actions`).
        ("src/world/page.rs", "unread_pane", &[Ban::Body]),
        ("src/sessions/page.rs", "fills_body", &[Ban::Pane(1)]),
        ("src/party.rs", "show", &[Ban::Pane(1)]),
    ]
};

/// The suppressions of `clippy::disallowed_methods` in the owners of the raw egui calls: the
/// design system (`src/design/`) and its capture harness (`src/visual_tests/`). One row per
/// suppression, as in [`STAGED_ALLOWS`], so each one sits on one function and frees nothing
/// else; only rows in those two folders may stand here, and only for `Ban::Layout`, `Ban::RowKey`
/// and `Ban::Extras`.
#[cfg(test)]
pub(crate) const OWNED_ALLOWS: &[(&str, &str, &[suppressions::Ban])] = {
    use suppressions::Ban;
    &[
        ("src/design/column.rs", "finish_handle", &[Ban::Layout(1)]),
        (
            "src/design/reader.rs",
            "the_scroll_bar_does_not_choose_the_prose_type",
            &[Ban::Layout(1)],
        ),
        // Migration-32 L2c: the design system's own pane tests draw closure lines (the
        // `#[cfg(test)]` `legacy_reason`, `legacy_notice`) to place a footer of a known height;
        // every app footer is typed. Its other three such tests, which also call
        // `Footer::result` and `show_by_hand`, stay in `STAGED_ALLOWS`.
        (
            "src/design/components.rs",
            "iter28_p_the_pane_places_its_footer_for_every_body",
            &[Ban::Layout(1), Ban::Footer],
        ),
        // The test of the staged keys-only add field itself (`AddFieldForm::keys_only`).
        (
            "src/design/components.rs",
            "iter29_p1_an_opened_add_field_shows_add_and_cancel",
            &[Ban::Extras(1)],
        ),
        (
            "src/design/components.rs",
            "suggestion_rows",
            &[Ban::Extras(1)],
        ),
        (
            "src/design/components.rs",
            "dialog_width",
            &[Ban::Layout(3)],
        ),
        (
            "src/design/components.rs",
            "draft_comparison_labeled",
            &[Ban::Layout(1), Ban::Extras(2)],
        ),
        (
            "src/design/components.rs",
            "enclosing_lock_describes_each_control_and_stays_inside_its_scope",
            &[Ban::Layout(1)],
        ),
        (
            "src/design/components.rs",
            "hanging_add_strip",
            &[Ban::Layout(1)],
        ),
        (
            "src/design/components.rs",
            "iter3_text_input_placeholder_uses_body_and_faint",
            &[Ban::Layout(1)],
        ),
        (
            "src/design/components.rs",
            "iter_two_form_labels_share_sentence_case_small_muted_type",
            &[Ban::Layout(1)],
        ),
        (
            "src/design/components.rs",
            "menu_submenu",
            &[Ban::Layout(1)],
        ),
        // M0412: the one collapsing section (`Section::show`); `CollapsingHeader::new` is
        // banned outside it.
        ("src/design/components.rs", "show", &[Ban::Layout(1)]),
        // Review of PR 730: the section whose header is the caller's row (a sheet's spells).
        ("src/design/components.rs", "section_row", &[Ban::Layout(1)]),
        (
            "src/design/components.rs",
            "number_field_with_id",
            &[Ban::Layout(1)],
        ),
        (
            "src/design/components.rs",
            "overflow_tabs",
            &[Ban::Layout(1)],
        ),
        ("src/design/components.rs", "row_menu", &[Ban::Layout(1)]),
        ("src/design/components.rs", "scroll_part", &[Ban::Layout(1)]),
        ("src/design/components.rs", "search_box", &[Ban::Layout(1)]),
        (
            "src/design/components.rs",
            "split_action",
            &[Ban::Layout(1)],
        ),
        ("src/design/components.rs", "stepper_as", &[Ban::Layout(1)]),
        (
            "src/design/components.rs",
            "text_field_label",
            &[Ban::Layout(1)],
        ),
        ("src/design/components.rs", "text_input", &[Ban::Layout(1)]),
        ("src/design/components.rs", "paint", &[Ban::Extras(1)]),
        ("src/design/extras.rs", "space", &[Ban::Extras(1)]),
        // Layout step 11c (M0478): an empty line as tall as a label, allocated as a label is.
        ("src/design/extras.rs", "empty_line", &[Ban::Extras(1)]),
        ("src/design/gallery.rs", "markdown", &[Ban::Layout(1)]),
        ("src/design/gallery.rs", "show", &[Ban::Layout(1)]),
        ("src/design/layout.rs", "area", &[Ban::Layout(2)]),
        ("src/design/pane/laid.rs", "stray_mark", &[Ban::Layout(1)]),
        ("src/design/layout.rs", "editor", &[Ban::Layout(1)]),
        (
            "src/design/layout.rs",
            "field_id! statement",
            &[Ban::Layout(1)],
        ),
        // Both anchors of a menu (a control, or a place such as the text cursor): M-P0.S1 moved
        // Sessions' `/` menu here, whose own `popup_at` row (1) is gone.
        ("src/design/layout.rs", "menu_popup_raw", &[Ban::Layout(2)]),
        ("src/design/layout.rs", "multi_line", &[Ban::Layout(1)]),
        ("src/design/layout.rs", "sideways_scroll", &[Ban::Layout(1)]),
        ("src/design/layout.rs", "single_line", &[Ban::Layout(1)]),
        ("src/design/layout.rs", "word! statement", &[Ban::Layout(1)]),
        ("src/design/layout.rs", "key! statement", &[Ban::Layout(1)]),
        (
            "src/design/layout/tests.rs",
            "a_field_id_keeps_focus_across_a_layout_branch",
            &[Ban::Layout(2)],
        ),
        (
            "src/visual_tests/ergonomics.rs",
            "a_control_in_a_popup_is_not_its_background",
            &[Ban::Layout(1)],
        ),
        (
            "src/visual_tests/ergonomics.rs",
            "a_menu_over_a_column_splitter_is_an_app_fault",
            &[Ban::Layout(3)],
        ),
        (
            "src/visual_tests/ergonomics.rs",
            "a_press_on_a_menu_item_reaches_the_item_and_the_pair_stays_chrome",
            &[Ban::Layout(1)],
        ),
        (
            "src/visual_tests/ergonomics.rs",
            "a_press_where_a_scroll_bar_lies_on_a_row_reaches_the_bar_and_is_an_app_fault",
            &[Ban::Layout(2)],
        ),
        (
            "src/visual_tests/ergonomics.rs",
            "a_small_control_without_a_name_is_not_chrome",
            &[Ban::Layout(1)],
        ),
        (
            "src/visual_tests/ergonomics.rs",
            "a_thin_unnamed_control_inside_horizontal_content_is_not_chrome",
            &[Ban::Layout(1)],
        ),
        (
            "src/visual_tests/ergonomics.rs",
            "help_measure_fails_on_a_wide_muted_small_label",
            &[Ban::Extras(1)],
        ),
        (
            "src/visual_tests/ergonomics.rs",
            "horizontal_clipping_is_allowed_only_where_a_real_scroll_bar_can_reveal_it",
            &[Ban::Layout(2), Ban::Extras(1)],
        ),
        (
            "src/visual_tests/ergonomics.rs",
            "only_the_real_scroll_bar_is_chrome",
            &[Ban::Layout(1), Ban::Extras(1)],
        ),
        (
            "src/visual_tests/ergonomics.rs",
            "scrolled_content_is_clipped_by_its_own_area_and_not_one_it_passes_behind",
            &[Ban::Layout(2)],
        ),
        // Package K2: the popup parts.
        ("src/design/outcome.rs", "measuring", &[Ban::Layout(1)]),
        ("src/design/pane.rs", "stop_body", &[Ban::Layout(1)]),
        // Layout PR 9b: the quiet-frame harnesses' bare screen and their stock reference, egui's
        // own text edit, whose cost a document editor is measured against (shared by
        // `quiet_text` and `splat-content-ui`'s `quiet_markdown`).
        (
            "src/design/tests/quiet/counting.rs",
            "root",
            &[Ban::Layout(1)],
        ),
        (
            "src/design/tests/quiet/counting.rs",
            "stock_text_edit",
            &[Ban::Layout(1)],
        ),
        // Layout PR 9b-R1a: the Reader's harness (`quiet_reader`): a bare screen of its size.
        (
            "src/design/tests/quiet/counting.rs",
            "root_sized",
            &[Ban::Layout(1)],
        ),
        ("src/design/popups.rs", "list", &[Ban::Layout(1)]),
        ("src/design/popups.rs", "rich_tip", &[Ban::Layout(2)]),
        ("src/design/popups.rs", "drag_source", &[Ban::Layout(3)]),
        ("src/design/popups.rs", "image_viewer", &[Ban::Layout(1)]),
        (
            "src/design/popups/tests.rs",
            "a_submenu_in_a_narrow_pane_keeps_its_parent_reachable",
            &[Ban::Layout(1)],
        ),
    ]
};

#[cfg(test)]
mod suppressions;

/// Calls of parts that a shared part replaces, by file, counted in the source text. They are a
/// second net inside the functions of [`STAGED_ALLOWS`], whose allow would let a new call
/// in: a count can only go down. A file with fewer calls than its count asks to lower the count.
///
/// - `ResultOrder::issue(`: only [`Results::record`] issues an order. Elsewhere, an order
///   issued by hand can be keyed by kind, or issued while drawing (reviews of PR 298).
/// - `save_row_state(`: a save row whose word, Save weight and reasons the caller decides
///   (`design::save_row` decides them from a `DraftState`).
/// - `consume_key(… Key::S)`: a page's own take of `Ctrl+S`, with its own order and its own modal
///   test. `design::save_row::save_key` is the one rule (issues #513, #514).
/// - `ptr_eq(`: a cache keyed by a pointer address. Key it by a `Revision` in a
///   `design::memo::Memo`, which takes no other key.
/// - `TitleLine::back(`: the old back link part, which a caller can place inside a row and block
///   with its reason on hover only. Its five page sites (Sessions 4, Timeline 1) keep it; an
///   editor uses `design::EditorHead` (R38).
/// - `BackEdge::Segments`: the Recordings row's named escape (its back link in one row with
///   `Add to notes`); its one use is counted, so a second row back link fails.
/// - `UndoStep::verb(`: an Undo with no object (`Undo remove`), whose words cannot name what it
///   takes back even where the line has room (ruling M9). The 23 production calls move to
///   `UndoStep::of` in their caller lanes, or stay as named exceptions; `outcome.rs` keeps its
///   part's own examples and tests. The text count sees a call written by this path, not one
///   through an alias of the type or a bare function reference: that gap closes when the path
///   becomes a Clippy ban after the callers move (the inventory's closing rule).
/// - `Refusal::at_limit(`: a refusal with no next step (rulings M13, M15), only at a limit the DM
///   cannot change (`design::results::Limit`: a tracker's `min`, every name rolled, the run rule
///   after the fight's encounter was deleted). Each production use is a reviewed raise of its
///   file's row; the part's own doc test and test hold the rest.
#[cfg(test)]
pub(crate) type Staged = (&'static str, &'static str, &'static [(&'static str, usize)]);

/// Layout PR 2b: every use of the legacy footer's closure lines (`suppressions::FOOTER_LINES`),
/// by file, in every call form (a method, an associated call, a reference: ruling L13), read as
/// tokens (`suppressions::footer_line_uses`). Clippy bans them, each calling function lists
/// `Ban::Footer`, and these counts can only shrink: give the footer typed data instead
/// (`outcome::Messages`).
#[cfg(test)]
pub(crate) const LEGACY_FOOTER_LINES: &[(&str, usize)] = &[("src/design/components.rs", 11)];

/// See [`Staged`]: the pattern, what to use instead, and the count of each file.
#[cfg(test)]
pub(crate) const STAGED: &[Staged] = &[
    (
        "Outcome::Refused(",
        "Answer a refused act with a typed `Refusal` on its entry (`Results::undo_or_refuse`): it names the thing, the cause and a next step, and ranks as an act (M1).",
        &[
            ("src/design/outcome.rs", 1),
            ("src/design/pane.rs", 5),
            ("src/sessions/plan.rs", 1),
            ("src/timeline/page.rs", 1),
            ("src/world.rs", 1),
        ],
    ),
    (
        "ResultOrder::issue(",
        "Record the result in a `design::Results` store: it issues the order at the action.",
        &[("src/design/components.rs", 15)],
    ),
    (
        "save_row_state(",
        "Use `design::save_row` with a `DraftState`: it decides the word, Save's weight and its reason.",
        &[
            ("src/campaign_content.rs", 1),
            ("src/campaigns.rs", 1),
            ("src/design/components.rs", 1),
            ("src/design/gallery.rs", 2),
        ],
    ),
    (
        "consume_key(egui::Modifiers::COMMAND, egui::Key::S)",
        "A page's Ctrl+S is `design::save_row::save_key`: one rule for its order and for modals (issues #513, #514).",
        &[("src/design/save_row.rs", 1)],
    ),
    (
        "ptr_eq(",
        "Key the cache by a `Revision` in a `design::memo::Memo`; a pointer address is not a change.",
        &[
            ("src/design/comparison.rs", 1),
            ("crates/campaign/src/encounter_run/undo.rs", 2),
            ("src/markdown_view.rs", 3),
            ("src/rolls.rs", 1),
        ],
    ),
    (
        "TitleLine::back(",
        "An editor's back is `design::EditorHead` (R38); a new page's back link that starts a line asks the lead for a row here.",
        &[
            ("src/design/layout.rs", 2),
            ("src/design/layout/tests.rs", 2),
            ("src/sessions/page.rs", 4),
            ("src/timeline/page.rs", 1),
        ],
    ),
    (
        "BackEdge::Segments",
        "The Recordings row's escape: its back link and `Add to notes` share one row. No other back link is drawn inside a row.",
        &[
            ("src/design/layout.rs", 1),
            ("src/design/layout/tests.rs", 1),
            ("src/sessions/page.rs", 1),
        ],
    ),
    (
        "UndoStep::verb(",
        "Give the Undo the object its result names, `UndoStep::of(verb, object)`: a lone Undo shows it whenever it fits (M9). An Undo with no object is a named exception the lead rules on.",
        &[
            ("src/design/outcome.rs", 7),
            ("src/encounters.rs", 4),
            ("src/sessions/plan.rs", 1),
            ("src/timeline/page.rs", 2),
            ("src/timeline/test_support.rs", 2),
        ],
    ),
    // M-S0.3: a tracker's `min` and every name rolled (M13), in `trackers::refusal`.
    (
        "Refusal::at_limit(",
        "A refusal ends at its cause only at a limit the DM cannot change: a tracker's `min`, every name rolled (M13), or the run rule after the fight's encounter was deleted (M15). A fault in the tracker rules, and every other cause, gets its next step: `Refusal::new`.",
        &[("src/design/results/refusal.rs", 3), ("src/trackers.rs", 2)],
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Note {
        id: u32,
        campaign: &'static str,
    }

    impl Owner for Note {
        type Scope = str;
        fn scope(&self) -> &str {
            self.campaign
        }
    }

    #[derive(Debug, PartialEq)]
    struct Done {
        note: Note,
        words: &'static str,
    }

    impl Kept for Done {
        type Owner = Note;
        const ENDS: Ends = Ends {
            owner_changed: OwnerChanged::Ends,
            disk_changed: DiskChanged::Checked,
            press: Press::Undo,
        };
        fn owner(&self) -> Note {
            self.note.clone()
        }
    }

    /// The test kind's state check: the note `with` names had its use changed.
    impl Reconcile for Done {
        type With = Note;
        fn reconcile(&mut self, with: &Note) {
            if self.note == *with {
                self.words = "A, its use changed.";
            }
        }
    }

    struct Offer(Note);

    impl Kept for Offer {
        type Owner = Note;
        const ENDS: Ends = Ends {
            owner_changed: OwnerChanged::Stays,
            disk_changed: DiskChanged::Ends,
            press: Press::Use,
        };
        fn owner(&self) -> Note {
            self.0.clone()
        }
    }

    const A: Note = Note {
        id: 1,
        campaign: "Tidewater",
    };
    const B: Note = Note {
        id: 2,
        campaign: "Tidewater",
    };
    const OTHER: Note = Note {
        id: 1,
        campaign: "Ashfall",
    };

    /// A footer line for the test of the one footer order (`outcome::footer_order`).
    struct Line(
        super::super::components::FooterRank,
        Option<ResultOrder>,
        &'static str,
    );

    impl super::super::outcome::Ranked for Line {
        fn key(&self) -> (super::super::components::FooterRank, Option<ResultOrder>) {
            (self.0, self.1)
        }
    }

    fn done(note: Note, words: &'static str) -> Done {
        Done { note, words }
    }

    fn words<'a>(results: &'a Results<Note, Done>, scope: &'a str) -> Vec<&'a str> {
        results
            .newest_first(scope)
            .map(|shown| shown.value().words)
            .collect()
    }

    /// A retained footer line of a `Done` (ruling L9): its words, and `Undo add` locked while
    /// the store's lock for it holds.
    fn line_of(shown: Shown<'_, Done>) -> super::super::outcome::KeptOutcome<u8> {
        use super::super::outcome::{Said, UndoStep, Verb};
        let undo = UndoStep::verb(Verb::Add).pressed_as(1_u8);
        let undo = match shown.lock() {
            Some(reason) => undo.blocked(reason.clone()),
            None => undo,
        };
        Said::new(crate::sentence!(
            "{}.",
            shown.value().words.trim_end_matches('.')
        ))
        .undo(undo)
        .into()
    }

    /// The revision of inputs that a test's lines read besides the store: never changed.
    fn no_inputs() -> Revision {
        super::super::memo::Revised::new(()).revision()
    }

    /// Review of PR 298, item 2: A's and B's link results each keep their own order. A slot
    /// keyed by kind gave B's order to A.
    #[test]
    fn each_object_keeps_the_order_its_own_action_issued() {
        let mut results = Results::new();
        results.record(done(A, "A."));
        let a = results.get(&A).unwrap().order();
        results.record(done(B, "B."));
        let b = results.get(&B).unwrap().order();
        assert!(b > a, "B was done later");
        assert_eq!(
            results.get(&A).unwrap().order(),
            a,
            "B's action leaves A's order"
        );
        assert_eq!(words(&results, "Tidewater"), ["B.", "A."]);
        // Reading (a draw) many times issues nothing.
        for _ in 0..3 {
            let _ = words(&results, "Tidewater");
        }
        assert_eq!(results.get(&A).unwrap().order(), a);
        // A again is a new action: a new order, newest, and A's earlier result has ended.
        results.record(done(A, "A again."));
        let again = results.get(&A).unwrap().order();
        assert!(again > b);
        assert_eq!(words(&results, "Tidewater"), ["A again.", "B."]);
    }

    /// Review of PR 299: a failed Undo dropped its change. Here A's Undo fails after an
    /// unrelated B; A stays with its order, and B is untouched. Then A's Undo works.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "the test of the by-owner Undo, staged until its callers use `undo_by`"
    )]
    fn a_failed_undo_keeps_its_entry_and_its_order() {
        let mut results = Results::new();
        results.record(done(A, "A."));
        results.record(done(B, "B."));
        let (a, b) = (
            results.get(&A).unwrap().order(),
            results.get(&B).unwrap().order(),
        );
        // Ruling L9: a failed plain Undo changes nothing a projection reads.
        let revision = results.revision();
        let failed = results.undo(&A, |_| Err::<(), _>("A's file changed."));
        assert_eq!(failed, Some(Err("A's file changed.")));
        assert_eq!(results.revision(), revision, "a failed Undo is no change");
        assert_eq!(
            results.get(&A).unwrap().order(),
            a,
            "A stays with its order"
        );
        assert_eq!(results.get(&B).unwrap().order(), b);
        assert_eq!(words(&results, "Tidewater"), ["B.", "A."]);
        let undone = results.undo(&A, |value| Ok::<_, ()>(value.words.len()));
        assert_eq!(undone, Some(Ok((done(A, "A."), 2))));
        assert_eq!(words(&results, "Tidewater"), ["B."]);
        assert_eq!(results.undo(&A, |_| Ok::<(), ()>(())), None, "A has gone");

        // M1: the recorded Undo. A refused attempt keeps the entry and its ticket, holds the
        // refusal with an order of its own, and outlives the change that caused it.
        use super::super::components::FooterRank;
        use super::super::outcome::{footer_order, refusal_rank};
        results.record(done(A, "A."));
        let a = results.get(&A).unwrap().order();
        let refusal = |cause| {
            Refusal::new(
                FailedAct::NotRemoved {
                    what: Thing::new(Kind::PlanLine, Name::new("Bell").unwrap()),
                    from: Name::new("Session 1").unwrap(),
                },
                cause,
                Recovery::ByHand {
                    file: File::SessionNote,
                    change: Change::RemovePlanLine,
                },
            )
        };
        let changed = || refusal(Cause::Changed(File::SessionNote));
        // Ruling L9: A's line, kept as data, before the refused attempt.
        let (frame, inputs) = (super::super::outcome::PaneFrame::for_test(), no_inputs());
        let mut view = Retained::new();
        view.refresh(&results, &A, inputs, line_of);
        let before = view.get().unwrap();
        assert!(matches!(
            results.present(&frame, before, &A, inputs),
            Ok(Some(_))
        ));
        let refused = results.undo_or_refuse(Ticket { order: a }, |_| Err::<(), _>(changed()));
        assert_eq!(refused, Err(ByTicket::Failed(())));
        // The refusal changes A's words with the same ticket: its kept line is refused, not
        // drawn; made again, it opens with the refusal's words.
        assert_eq!(
            results
                .present(&frame, view.get().unwrap(), &A, inputs)
                .err(),
            Some(Mismatch::Revision),
            "a line kept from before the refusal is not drawn"
        );
        let after = view.refresh(&results, &A, inputs, line_of);
        assert!(
            after
                .words()
                .is_some_and(|words| words.starts_with(changed().words_told_apart(None).as_str())),
            "made again, it says the refusal: {:?}",
            after.words()
        );
        // A lock is a change too, and only a change of it: the Undo's lock comes with it.
        let revision = results.revision();
        results.set_locks(|_| Some(crate::sentence!("Save the note first.")));
        assert_ne!(results.revision(), revision, "a new lock");
        let locked = results.revision();
        results.set_locks(|_| Some(crate::sentence!("Save the note first.")));
        assert_eq!(results.revision(), locked, "the same lock is no change");
        assert!(view.refresh(&results, &A, inputs, line_of).undo_locked());
        results.set_locks(|_| None);
        assert!(!view.refresh(&results, &A, inputs, line_of).undo_locked());
        let shown = results.get_by(&Ticket { order: a }).unwrap();
        assert_eq!(shown.order(), a, "the ticket stays");
        assert_eq!(shown.refusal(), Some(&changed()));
        // Review N7: an answer told apart from one that reads alike joins its place with the
        // middle dot, as its result and its Undo do (R45: no brackets).
        assert_eq!(
            changed()
                .words_told_apart(Name::new("Travel · 2").as_ref())
                .as_str(),
            "The plan line for Bell was not removed from Session 1 · Travel · 2. The session \
             note changed on disk. Open that session’s notes and remove the plan line there."
        );
        let attempt = shown.refused.unwrap().order;
        assert!(
            attempt > a && attempt > b,
            "the attempt has an order of its own"
        );
        // R41: the tone comes from the diagnosis.
        assert_eq!(changed().tone(), crate::design::Tone::Warning);
        assert_eq!(
            refusal(Cause::Gone(File::SessionNote)).tone(),
            crate::design::Tone::Warning
        );
        assert_eq!(
            refusal(Cause::Unreadable(File::SessionNote)).tone(),
            crate::design::Tone::Error
        );
        // The change that caused the refusal does not end it; a result with none ends.
        results.source_changed(|_| false);
        assert_eq!(
            words(&results, "Tidewater"),
            ["A."],
            "B ended, A's answer stays"
        );
        // R42 as amended, in one feed: standing lines keep their places; answers stand newest
        // first in their group; a refusal leads only while it is the footer's newest answer, then
        // it stands with the results with no Undo, by its attempt's order, and stays.
        let order = |lines: &mut Vec<Line>| {
            footer_order(lines);
            lines.iter().map(|line| line.2).collect::<Vec<_>>()
        };
        let older = Some(a);
        let feed = |newest: Option<ResultOrder>, more: Vec<Line>| {
            let mut lines = vec![
                Line(FooterRank::Undo, older, "older success with Undo"),
                Line(FooterRank::Reason, None, "standing fault"),
                Line(FooterRank::Notice, None, "standing notice"),
                Line(refusal_rank(attempt, newest), Some(attempt), "A refused"),
            ];
            lines.extend(more);
            lines
        };
        assert_eq!(
            order(&mut feed(Some(attempt), Vec::new())),
            [
                "A refused",
                "older success with Undo",
                "standing fault",
                "standing notice"
            ],
            "A's refusal is the newest answer: it leads"
        );
        // A later act that leaves no line here moves nothing.
        let _ = act_order();
        assert_eq!(order(&mut feed(Some(attempt), Vec::new()))[0], "A refused");
        // A later success with no Undo is the newest answer: A goes below it, and stays; the
        // standing lines keep their places above both.
        results.record(done(B, "B."));
        let b = results.get(&B).unwrap().order();
        assert_eq!(
            order(&mut feed(
                Some(b),
                vec![Line(FooterRank::Notice, Some(b), "B, no Undo")]
            )),
            [
                "older success with Undo",
                "standing fault",
                "standing notice",
                "B, no Undo",
                "A refused"
            ]
        );
        assert!(
            results.get(&A).unwrap().refusal().is_some(),
            "pushed down, not ended"
        );
        // A's next attempt that works ends its words and its data together; B stays.
        assert!(
            results
                .undo_or_refuse(Ticket { order: a }, |_| Ok(()))
                .is_ok()
        );
        assert!(results.get(&A).is_none());
        assert_eq!(words(&results, "Tidewater"), ["B."], "B stays");
        // `end_refusals` ends only the refusals it names; the entry stays only while it holds.
        results.record(done(A, "A."));
        let a = results.get(&A).unwrap().order();
        let _ = results.undo_or_refuse(Ticket { order: a }, |_| Err::<(), _>(changed()));
        assert_eq!(results.end_refusals(|value| value.note == B, |_| false), 0);
        assert!(
            results.get(&A).unwrap().refusal().is_some(),
            "another owner's end: A stays"
        );
        assert_eq!(
            results.end_refusals(|value| value.note == A, |value| value.words == "B."),
            1
        );
        assert_eq!(words(&results, "Tidewater"), ["B."]);
    }

    /// A campaign switch hides a result and does not end it; one campaign never shows another's
    /// result, even for the same id.
    #[test]
    fn a_result_shows_only_in_its_own_scope() {
        let mut results = Results::new();
        results.record(done(A, "A."));
        results.record(done(OTHER, "Other."));
        assert_eq!(words(&results, "Tidewater"), ["A."]);
        assert_eq!(words(&results, "Ashfall"), ["Other."]);
        assert_eq!(results.get(&A).unwrap().value().words, "A.");
        assert_eq!(results.get(&OTHER).unwrap().value().words, "Other.");
        assert!(words(&results, "Riverhold").is_empty());

        // Review of PR 301: an owner whose `PartialEq` compares its id only. Campaign B's
        // owner has A's id; B cannot read, replace, undo or clear A.
        #[derive(Clone, Debug)]
        struct ById {
            id: u32,
            campaign: &'static str,
        }
        impl PartialEq for ById {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
        impl Eq for ById {}
        impl Owner for ById {
            type Scope = str;
            fn scope(&self) -> &str {
                self.campaign
            }
        }
        struct Linked(ById, &'static str);
        impl Kept for Linked {
            type Owner = ById;
            const ENDS: Ends = Ends {
                owner_changed: OwnerChanged::Ends,
                disk_changed: DiskChanged::Checked,
                press: Press::Undo,
            };
            fn owner(&self) -> ById {
                self.0.clone()
            }
        }
        let (a, b) = (
            ById {
                id: 7,
                campaign: "Tidewater",
            },
            ById {
                id: 7,
                campaign: "Ashfall",
            },
        );
        // Ruling L9, early review F3: a kept line's binding compares the owner's scope, as the
        // store does. Both entries are in before A's line is made, so the revision matches;
        // B's key, equal to A's by `PartialEq`, is still refused: no line, no ticket.
        {
            use super::super::outcome::{Said, UndoStep, Verb};
            let line = |shown: Shown<'_, Linked>| -> super::super::outcome::KeptOutcome<u8> {
                Said::new(crate::sentence!(
                    "{}.",
                    shown.value().1.trim_end_matches('.')
                ))
                .undo(UndoStep::verb(Verb::Add).pressed_as(1_u8))
                .into()
            };
            let (frame, inputs) = (super::super::outcome::PaneFrame::for_test(), no_inputs());
            let mut both = Results::new();
            both.record(Linked(a.clone(), "A."));
            both.record(Linked(b.clone(), "B."));
            let mut view = Retained::new();
            let a_line = view.refresh(&both, &a, inputs, line);
            assert_eq!(a_line.words(), Some("A."));
            assert!(matches!(
                both.present(&frame, a_line, &a, inputs),
                Ok(Some(_))
            ));
            assert_eq!(
                both.present(&frame, a_line, &b, inputs).err(),
                Some(Mismatch::Key),
                "A's line is not B's, in B's campaign"
            );
            // A store made by `Default`, with the same entries, is another store.
            let mut other: Results<ById, Linked> = Results::default();
            other.record(Linked(a.clone(), "A."));
            let mut fresh = Retained::new();
            let fresh_line = fresh.refresh(&other, &a, inputs, line);
            assert_ne!(other.revision(), both.revision());
            assert_eq!(
                both.present(&frame, fresh_line, &a, inputs).err(),
                Some(Mismatch::Revision),
                "a line of another store is not this store's"
            );
            assert_eq!(
                both.present(&frame, view.get().unwrap(), &a, no_inputs())
                    .err(),
                Some(Mismatch::Inputs),
                "a line made from other inputs is not drawn"
            );
        }
        let mut links = Results::new();
        links.record(Linked(a.clone(), "A."));
        let a_order = links.get(&a).unwrap().order();
        assert!(links.get(&b).is_none(), "B reads A");
        #[allow(
            clippy::disallowed_methods,
            reason = "the test of the by-owner Undo, staged until its callers use `undo_by`"
        )]
        let b_undoes_a = links.undo(&b, |_| Ok::<(), ()>(()));
        assert!(b_undoes_a.is_none(), "B undoes A");
        links.owner_changed(&b);
        assert!(links.get(&a).is_some(), "B's saved edit ends A");
        links.record(Linked(b.clone(), "B."));
        let a_shown = links.get(&a).unwrap();
        assert_eq!(
            (a_shown.value().1, a_shown.order()),
            ("A.", a_order),
            "B's result replaces A's"
        );
        assert_eq!(links.get(&b).unwrap().value().1, "B.");
        #[allow(
            clippy::disallowed_methods,
            reason = "the test of the by-owner Undo, staged until its callers use `undo_by`"
        )]
        let undone = links.undo(&b, |_| Ok::<(), ()>(()));
        assert!(matches!(undone, Some(Ok(_))));
        assert_eq!(links.get(&a).unwrap().value().1, "A.", "B's Undo takes A");
    }

    /// A ticket names one entry: after its owner's later action replaced it, or an end event
    /// ended it, the ticket's Undo is `Gone` and changes nothing; the other entries stay.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "the test of the by-owner revisions against the ticket's"
    )]
    fn a_ticket_reaches_only_its_own_entry() {
        let mut results = Results::<Note, Done>::new();
        let (a, b) = (A, B);
        results.record(done(A, "A1"));
        results.record(done(B, "B"));
        let frame = super::super::outcome::PaneFrame::for_test();
        let ticket = |results: &Results<Note, Done>, owner: &Note| {
            results.get(owner).unwrap().ticket_for_test(&frame)
        };
        let old = ticket(&results, &a);
        results.record(done(A, "A2"));
        assert_eq!(
            results.undo_by(old, |_| Ok::<_, ()>(())).err(),
            Some(ByTicket::Gone)
        );
        assert_eq!(results.get(&a).map(|shown| shown.value().words), Some("A2"));
        assert_eq!(results.get(&b).map(|shown| shown.value().words), Some("B"));
        let old = ticket(&results, &a);
        results.owner_changed(&a);
        assert_eq!(
            results.undo_by(old, |_| Ok::<_, ()>(())).err(),
            Some(ByTicket::Gone)
        );
        let b_ticket = ticket(&results, &b);
        // A failed Undo keeps its entry.
        assert_eq!(
            results.undo_by(b_ticket, |_| Err::<(), _>("no")).err(),
            Some(ByTicket::Failed("no"))
        );
        let b_ticket = ticket(&results, &b);
        assert_eq!(
            results
                .undo_by(b_ticket, |_| Ok::<_, ()>(()))
                .map(|(done, ())| done.words),
            Ok("B")
        );

        // Ruling L9, early review F5: kept lines of A and B share one store revision. Undo A:
        // both kept lines are refused until their owner makes them again; then A has none, and
        // B shows its unchanged words with a ticket that still reaches B. A recorded again: A's
        // first line and ticket stay unusable.
        let mut results = Results::<Note, Done>::new();
        results.record(done(A, "A"));
        results.record(done(B, "B"));
        let inputs = no_inputs();
        let (mut view_a, mut view_b, mut first_a) =
            (Retained::new(), Retained::new(), Retained::new());
        first_a.refresh(&results, &a, inputs, line_of);
        view_a.refresh(&results, &a, inputs, line_of);
        view_b.refresh(&results, &b, inputs, line_of);
        let b_before = ticket(&results, &b);
        let a_first = ticket(&results, &a);
        assert!(matches!(
            results.present(&frame, view_b.get().unwrap(), &b, inputs),
            Ok(Some(line)) if *line.ticket() == b_before
        ));
        assert!(
            results
                .undo_by(ticket(&results, &a), |_| Ok::<_, ()>(()))
                .is_ok()
        );
        for (view, owner) in [(&view_a, &a), (&view_b, &b)] {
            assert_eq!(
                results
                    .present(&frame, view.get().unwrap(), owner, inputs)
                    .err(),
                Some(Mismatch::Revision),
                "before its owner makes it again, a kept line is not drawn"
            );
        }
        let a_now = view_a.refresh(&results, &a, inputs, line_of);
        assert!(matches!(
            results.present(&frame, a_now, &a, inputs),
            Ok(None)
        ));
        let b_now = view_b.refresh(&results, &b, inputs, line_of);
        assert_eq!(b_now.words(), Some("B."), "B's words are unchanged");
        assert!(matches!(
            results.present(&frame, b_now, &b, inputs),
            Ok(Some(line)) if *line.ticket() == b_before
        ));
        assert_eq!(results.get(&b).map(|shown| shown.value().words), Some("B"));
        results.record(done(A, "A again"));
        assert_eq!(
            results
                .present(&frame, first_a.get().unwrap(), &a, inputs)
                .err(),
            Some(Mismatch::Revision),
            "A's first line does not come back with a new A"
        );
        assert_eq!(
            results.undo_by(a_first, |_| Ok::<_, ()>(())).err(),
            Some(ByTicket::Gone)
        );

        // M3 gap 3: a kept entry changes in place. Revise A (by its ticket, then by its owner):
        // the unrelated B stays, A keeps its order and its place, its old ticket still acts and
        // the action gets the revised value; a revised entry keeps its kind's end events; an
        // ended entry is `Gone`.
        let mut results = Results::<Note, Done>::new();
        results.record(done(A, "A"));
        results.record(done(B, "B"));
        let order = results.get(&a).unwrap().order();
        let before = ticket(&results, &a);
        assert_eq!(results.revise_by(&before, |v| v.words = "A moved"), Ok(()));
        assert_eq!(results.revise(&a, |v| v.words), Ok("A moved"));
        assert_eq!(words(&results, "Tidewater"), ["B", "A moved"]);
        assert_eq!(results.get(&a).unwrap().order(), order);
        assert_eq!(results.get(&b).map(|shown| shown.value().words), Some("B"));
        assert_eq!(results.revise(&OTHER, |_| ()), Err(Gone), "another scope");
        assert_eq!(
            results.revise_where(|v| v.note == b, |v| v.words = "B moved"),
            1
        );
        assert_eq!(words(&results, "Tidewater"), ["B moved", "A moved"]);
        assert_eq!(
            results
                .undo_by(before, |v| Ok::<_, ()>(v.words))
                .map(|(_, seen)| seen),
            Ok("A moved"),
            "the ticket made before the revision acts, on the revised value"
        );
        let b_ticket = ticket(&results, &b);
        results.owner_changed(&b);
        assert_eq!(results.revise_by(&b_ticket, |_| ()), Err(Gone));
        assert_eq!(results.revise(&b, |_| ()), Err(Gone));
        assert!(
            results.is_empty(),
            "the revised B ended at its owner's change"
        );
        // Review of PR 332, 1: A's late work ends after B replaced A for the same owner. Its
        // revision by A's ticket is `Gone`, and B is unchanged.
        let mut results = Results::<Note, Done>::new();
        results.record(done(A, "A"));
        let stale = ticket(&results, &a);
        results.record(done(A, "B for the same note"));
        assert_eq!(results.revise_by(&stale, |v| v.words = "late"), Err(Gone));
        assert_eq!(
            results.get(&a).map(|shown| shown.value().words),
            Some("B for the same note")
        );
        // M0543: the owner's hold revises the entry it recorded, on an event that is no press
        // (the Timeline's removal follows the run's updates). After a newer record for the same
        // owner, the hold is `Gone` and the newer entry stays as it is.
        let mut results = Results::<Note, Done>::new();
        let hold = results.record_held(done(A, "A"));
        results.record(done(B, "B"));
        assert_eq!(
            results.revise_held(&hold, |v| v.words = "A followed"),
            Ok(()),
            "m-p0-hold: the hold revises its entry"
        );
        assert_eq!(words(&results, "Tidewater"), ["B", "A followed"]);
        results.record(done(A, "newer A"));
        assert_eq!(
            results.revise_held(&hold, |v| v.words = "late"),
            Err(Gone),
            "m-p0-hold: a newer entry of the owner is not the held one"
        );
        assert_eq!(
            results.get(&a).map(|shown| shown.value().words),
            Some("newer A")
        );
    }

    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "the test of the by-owner offer press, staged until its callers use `use_press_by`"
    )]
    fn end_events_do_what_the_kind_says() {
        let mut results = Results::new();
        results.record(done(A, "A."));
        results.record(done(B, "B."));
        // `owner_changed: Ends` ends the owner's result only.
        results.owner_changed(&A);
        assert_eq!(words(&results, "Tidewater"), ["B."]);
        // `disk_changed: Checked` asks each result.
        results.record(done(A, "A."));
        let asked = Cell::new(0);
        results.disk_changed(|value| {
            asked.set(asked.get() + 1);
            value.note == A
        });
        assert_eq!((asked.get(), words(&results, "Tidewater")), (2, vec!["A."]));

        let mut offers = Results::new();
        offers.record(Offer(A));
        offers.record(Offer(B));
        // `owner_changed: Stays`.
        offers.owner_changed(&A);
        assert!(offers.get(&A).is_some());
        // `press: Use` takes it whatever comes next.
        assert!(offers.use_press(&A).is_some());
        assert!(offers.get(&A).is_none() && offers.get(&B).is_some());
        // `disk_changed: Ends` never asks.
        offers.disk_changed(|_| unreachable!("an `Ends` kind is not checked"));
        assert!(offers.is_empty());
    }

    /// R11: `reconcile` revises each entry in place by itself: A's value changes, B's does not;
    /// both keep their order, and A's ticket from before still reaches A (its Undo stays).
    /// Ruling L9, early review F4: a timed store's owner update visits no entry on a frame
    /// before the next end, and `present` finds its entry by the place kept with its line, so
    /// an unchanged frame costs the same with one entry or two hundred. What a frame hands the
    /// legacy footer is the shown line's words, all of them, also the clauses the footer may
    /// not show: the counted exception that PR 2b removes. At the end, one frame visits each
    /// entry once, ends them, and advances the revision.
    #[test]
    fn a_timed_store_does_no_entry_work_before_its_end() {
        use super::super::outcome::{Said, UndoStep, Verb};
        struct Timed(Note, String);
        impl Kept for Timed {
            type Owner = Note;
            const ENDS: Ends = Ends {
                owner_changed: OwnerChanged::Ends,
                disk_changed: DiskChanged::Stays,
                press: Press::Undo,
            };
            const LIFE: Option<Duration> = Some(Duration::from_secs(30));
            fn owner(&self) -> Note {
                self.0.clone()
            }
        }
        fn line(shown: Shown<'_, Timed>) -> super::super::outcome::KeptOutcome<u8> {
            Said::new(crate::sentence!(
                "{}.",
                shown.value().1.trim_end_matches('.')
            ))
            .undo(UndoStep::verb(Verb::Add).pressed_as(1_u8))
            .into()
        }
        let ctx = egui::Context::default();
        let (frame, inputs) = (super::super::outcome::PaneFrame::for_test(), no_inputs());
        crate::frame_clock::set(10.0);
        let mut results = Results::<Note, Timed>::new();
        for id in 0..200 {
            let hidden = Note {
                id: 100 + id,
                campaign: "Ashfall",
            };
            results.record_timed(Timed(hidden, "Hidden.".into()), &ctx);
        }
        let long = "Bridge keeps its saved HP. ".repeat(40);
        let long = long.trim_end();
        results.record_timed(Timed(A, long.to_owned()), &ctx);
        let mut view = Retained::new();
        let mut frame_of = |results: &mut Results<Note, Timed>| {
            results.expire(&ctx);
            let projection = view.refresh(results, &A, inputs, line);
            results
                .present(&frame, projection, &A, inputs)
                .unwrap()
                .is_some()
        };
        assert!(frame_of(&mut results), "the first frame makes the line");
        let before = projection_work();
        for at in [12.0, 20.0, 39.5] {
            crate::frame_clock::set(at);
            assert!(frame_of(&mut results), "{at}: it stands");
        }
        let after = projection_work();
        assert_eq!(
            (after.built - before.built, after.visits - before.visits),
            (0, 0),
            "frames before the end make no line and visit no entry"
        );
        assert_eq!(
            after.handed - before.handed,
            3 * long.len(),
            "each frame hands the shown line's words to the legacy footer, no more"
        );
        let revision = results.revision();
        crate::frame_clock::set(40.0);
        assert!(!frame_of(&mut results), "at the end the line is gone");
        assert!(results.is_empty());
        assert_ne!(results.revision(), revision, "the end is a change");
        assert_eq!(
            projection_work().visits - after.visits,
            201,
            "the end visits each entry once"
        );
        // Review F3 of PR 444: staggered births. C at 50 s and D at 60 s: at 80 s C ends and D
        // stays; that frame visits each entry once, finding D's end on the way, and the next
        // frame before D's end visits none.
        crate::frame_clock::set(50.0);
        results.record_timed(Timed(B, "C.".into()), &ctx);
        crate::frame_clock::set(60.0);
        results.record_timed(Timed(OTHER, "D.".into()), &ctx);
        let before = projection_work().visits;
        crate::frame_clock::set(80.0);
        results.expire(&ctx);
        assert!(
            results.get(&B).is_none() && results.get(&OTHER).is_some(),
            "C ended, D stays"
        );
        assert_eq!(
            projection_work().visits - before,
            2,
            "an end visits each entry once, also the one that stays"
        );
        let before = projection_work().visits;
        crate::frame_clock::set(85.0);
        results.expire(&ctx);
        assert_eq!(
            projection_work().visits - before,
            0,
            "the next end was found on the way"
        );
    }

    #[test]
    fn reconcile_revises_each_entry_in_place() {
        let mut results = Results::<Note, Done>::new();
        results.record(done(A, "A."));
        results.record(done(B, "B."));
        results.record(done(OTHER, "Other."));
        let frame = super::super::outcome::PaneFrame::for_test();
        let a_ticket = results.get(&A).unwrap().ticket_for_test(&frame);
        let orders = |results: &Results<Note, Done>| {
            [A, B, OTHER].map(|note| results.get(&note).map(|shown| shown.order()))
        };
        let before = orders(&results);
        results.reconcile(&A);
        assert_eq!(words(&results, "Tidewater"), ["B.", "A, its use changed."]);
        assert_eq!(words(&results, "Ashfall"), ["Other."]);
        assert_eq!(orders(&results), before, "no order is issued or lost");
        let undone = results.undo_by(a_ticket, |value| Ok::<_, ()>(value.words));
        assert_eq!(
            undone.ok().map(|(_, words)| words),
            Some("A, its use changed.")
        );
        assert_eq!(words(&results, "Tidewater"), ["B."]);
    }

    /// The staged counts of `STAGED`: a new call fails, and a lower count asks to lower it.
    #[test]
    fn staged_calls_only_shrink() {
        let mut files = Vec::new();
        collect(&crate::workspace().join("src"), &mut files);
        collect(&crate::workspace().join("crates/campaign/src"), &mut files);
        let mut faults = Vec::new();
        for (pattern, instead, allowed) in STAGED {
            for (path, text) in &files {
                // The store itself, this table and the suppressions' checker name the patterns.
                if path == "src/design/results.rs" || path == "src/design/results/suppressions.rs" {
                    continue;
                }
                let count = text.matches(pattern).count();
                let allowed = allowed
                    .iter()
                    .find(|(file, _)| file == path)
                    .map_or(0, |(_, count)| *count);
                if count > allowed {
                    faults.push(format!(
                        "{path}: `{pattern}` occurs {count} times, {allowed} allowed. {instead}"
                    ));
                } else if count < allowed {
                    faults.push(format!(
                        "{path}: `{pattern}` occurs only {count} times. Lower its count in `design::results::STAGED` to {count}."
                    ));
                }
            }
        }
        assert!(faults.is_empty(), "{}", faults.join("\n"));
    }

    /// Layout PR 2b, ruling L13: the legacy footer's closure lines only shrink, counted in every
    /// call form.
    #[test]
    fn legacy_footer_lines_only_shrink() {
        let mut files = Vec::new();
        collect(&crate::workspace().join("src"), &mut files);
        let mut found: Vec<(String, usize)> = files
            .iter()
            .map(|(path, text)| (path.clone(), suppressions::footer_line_uses(text)))
            .filter(|(_, uses)| *uses > 0)
            .collect();
        found.sort();
        let listed: Vec<(String, usize)> = LEGACY_FOOTER_LINES
            .iter()
            .map(|(path, uses)| ((*path).to_owned(), *uses))
            .collect();
        assert_eq!(
            found, listed,
            "a footer line lowered to a closure is the legacy footer's: give the footer typed \
             data (`outcome::Messages`); a removed use lowers its row"
        );
    }

    /// Every suppression of `clippy::disallowed_methods` is a row of `STAGED_ALLOWS` (see
    /// there); the checker's own cases are in `suppressions::tests`.
    #[test]
    fn staged_allows_only_shrink() {
        let mut files = Vec::new();
        collect(&crate::workspace().join("src"), &mut files);
        for (file, item, bans) in OWNED_ALLOWS {
            assert!(
                (file.starts_with("src/design/") || file.starts_with("src/visual_tests/"))
                    && bans.iter().all(|ban| matches!(
                        ban,
                        suppressions::Ban::Layout(_)
                            | suppressions::Ban::RowKey(_)
                            | suppressions::Ban::Extras(_)
                            // Layout PR 2b: the design system owns the legacy footer's closure
                            // lines, and its own tests of them.
                            | suppressions::Ban::Footer
                    )),
                "{file} `{item}`: only the design system and its harness own raw egui calls and \
                 the legacy footer's closure lines"
            );
        }
        let rows: Vec<_> = STAGED_ALLOWS.iter().chain(OWNED_ALLOWS).copied().collect();
        let faults = suppressions::check(&files, &rows);
        assert!(faults.is_empty(), "{}", faults.join("\n"));
    }

    /// The escapes from `&Results` and `Revised` that the compiler cannot see. A store behind
    /// a `Cell`, `RefCell`, `OnceCell`, `Mutex` or `RwLock` changes through `&` (rung 3: only
    /// these spellings are caught, `Wrapper<Results<…>>` and `Wrapper::new(Results::…)`; a store
    /// whose wrapper type is inferred from elsewhere is for a reviewer). A `Mutex`, `RwLock` or
    /// atomic inside a `Revised` is `Sync`, so the compiler takes it; this scan does not. A `Cell`
    /// or `RefCell` inside one is not `Sync` and does not compile.
    #[test]
    fn results_are_never_behind_interior_mutability() {
        let mut files = Vec::new();
        collect(&crate::workspace().join("src"), &mut files);
        let wrappers: Vec<String> = ["Cell", "RefCell", "OnceCell", "Mutex", "RwLock"]
            .into_iter()
            .flat_map(|w| {
                ["", "cell::", "sync::", "std::cell::", "std::sync::"].map(|p| format!("{p}{w}"))
            })
            .collect();
        let held = |name: &str| {
            [
                "",
                "design::",
                "results::",
                "memo::",
                "design::results::",
                "design::memo::",
                "crate::design::",
                "crate::design::results::",
                "crate::design::memo::",
            ]
            .map(|p| format!("{p}{name}"))
        };
        let mut patterns = Vec::new();
        for wrapper in &wrappers {
            for store in held("Results").into_iter().chain(held("Revised")) {
                patterns.push(format!("{wrapper}<{store}<"));
                patterns.push(format!("{wrapper}::new({store}::"));
            }
        }
        for revised in held("Revised") {
            for wrapper in &wrappers {
                patterns.push(format!("{revised}<{wrapper}<"));
                patterns.push(format!("{revised}::new({wrapper}::new("));
            }
            for atomic in ["Atomic", "atomic::Atomic", "std::sync::atomic::Atomic"] {
                patterns.push(format!("{revised}<{atomic}"));
                patterns.push(format!("{revised}::new({atomic}"));
            }
        }
        let mut faults = Vec::new();
        for (path, text) in &files {
            let compact: String = suppressions::code_only(text).split_whitespace().collect();
            for pattern in &patterns {
                if compact.contains(pattern.as_str()) {
                    faults.push(format!(
                        "{path}: `{pattern}` changes a kept result or a revised value through `&`. Keep it in a plain field."
                    ));
                }
            }
        }
        assert!(faults.is_empty(), "{}", faults.join("\n"));
    }

    fn collect(folder: &std::path::Path, files: &mut Vec<(String, String)>) {
        for item in std::fs::read_dir(folder).expect("read source folder") {
            let path = item.expect("source entry").path();
            if path.is_dir() {
                collect(&path, files);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                let text = std::fs::read_to_string(&path).expect("read source file");
                files.push((
                    path.strip_prefix(crate::workspace())
                        .expect("path in workspace")
                        .to_string_lossy()
                        .replace('\\', "/"),
                    text,
                ));
            }
        }
    }
}
