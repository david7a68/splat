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
//!   order lives in the entry, and it leaves the store only inside the [`Shown`] view, which
//!   [`Footer::kept`] takes, or a [`FooterResult`] bound to that result's own words, which
//!   [`Footer::kept_line`] takes. A caller never holds a bare order, so it
//!   cannot keep orders by kind and show one result with another's order. A draw, which has
//!   `&Results`, cannot issue or renew an order.
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
//! use splat::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
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
//! // The twin of each case below: a draw reads, and the footer line binds words and order.
//! fn draw(results: &Results<Entry, Done>, a: &Entry) -> Option<splat::ownership::FooterResult> {
//!     results.get(a).map(|shown| shown.footer(|done| done.words.clone()))
//! }
//! assert!(draw(&results, &a).is_some());
//! let undone = results.undo(&a, |_| Ok::<(), String>(()));
//! assert!(matches!(undone, Some(Ok(_))));
//! ```
//!
//! The footer takes the view, and maps the stored value to its words and its drawing:
//!
//! ```no_run
//! # use splat::ownership::{DiskChanged, Ends, Footer, Kept, Owner, OwnerChanged, Press, Results};
//! # #[derive(Clone, PartialEq, Eq)] struct Entry { id: u32, root: String }
//! # impl Owner for Entry { type Scope = str; fn scope(&self) -> &str { &self.root } }
//! # struct Done { entry: Entry, words: String }
//! # impl Kept for Done { type Owner = Entry; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Checked, press: Press::Undo }; fn owner(&self) -> Entry { self.entry.clone() } }
//! fn add<'a>(footer: &mut Footer<'a>, results: &'a Results<Entry, Done>, a: &Entry) {
//!     if let Some(shown) = results.get(a) {
//!         footer.kept(shown, true, |done| done.words.as_str().into(), |done, ui| {
//!             ui.label(done.words.as_str());
//!         });
//!     }
//! }
//! ```
//!
//! A draw gets `&Results`, so it cannot record a result or issue its order:
//!
//! ```compile_fail
//! # use splat::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
//! # #[derive(Clone, PartialEq, Eq)] struct Entry { id: u32, root: String }
//! # impl Owner for Entry { type Scope = str; fn scope(&self) -> &str { &self.root } }
//! # struct Done { entry: Entry, words: String }
//! # impl Kept for Done { type Owner = Entry; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Checked, press: Press::Undo }; fn owner(&self) -> Entry { self.entry.clone() } }
//! fn draw(results: &Results<Entry, Done>, a: &Entry) {
//!     results.record(Done { entry: a.clone(), words: String::new() }); // <- issues an order while drawing
//! }
//! ```
//!
//! Orders kept by kind (the fault of PR 298): a caller cannot take a bare order out of the
//! store, so it cannot keep one per kind and give A's result B's order:
//!
//! ```compile_fail
//! # use splat::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
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
//! let order = splat::ownership::ResultOrder(7); // <- an order from a number
//! ```
//!
//! A footer line's words and order come from one stored result:
//!
//! ```compile_fail
//! # use splat::ownership::{FooterResult, ResultOrder};
//! let line = FooterResult { order: ResultOrder::default(), words: String::from("B.") }; // <-
//! ```
//!
//! There is no way to take an entry out except its end events and its press, so a failed Undo
//! cannot drop the change (review of PR 299):
//!
//! ```compile_fail
//! # use splat::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
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
//! # use splat::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
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
//! # use splat::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
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
//! # use splat::ownership::{DiskChanged, Ends, OwnerChanged};
//! const ENDS: Ends = Ends {
//!     owner_changed: OwnerChanged::Ends,
//!     disk_changed: DiskChanged::Checked,
//!     // <- `press` left out
//! };
//! ```
use super::components::{Footer, ResultOrder};
use eframe::egui::Ui;

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
/// store makes one, and its order leaves it only inside a [`FooterResult`], with its own words.
pub struct Shown<'a, T> {
    order: ResultOrder,
    value: &'a T,
}

impl<'a, T> Shown<'a, T> {
    /// The order its action issued, for tests only. A caller gives the footer a [`FooterResult`],
    /// so it cannot keep an order apart from its result.
    #[cfg(test)]
    pub(crate) fn order(&self) -> ResultOrder {
        self.order
    }
    pub fn value(&self) -> &'a T {
        self.value
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
        super::outcome::KeptLine::new(frame, Ticket { order: self.order }, make(self.value))
    }

    /// This result's footer line: its words, made from its own value, with its own order.
    pub fn footer(&self, words: impl FnOnce(&T) -> String) -> FooterResult {
        FooterResult {
            order: self.order,
            words: words(self.value),
        }
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
}

/// What a store did with a ticket.
#[derive(Debug, PartialEq, Eq)]
pub enum ByTicket<E> {
    /// The entry it named was replaced or ended: nothing changed.
    Gone,
    /// The entry is there, and its action failed: it stays.
    Failed(E),
}

/// One kept result's words and its order, bound together. It is not `Clone`, and its order is
/// private: [`Footer::kept_line`] is the only way to give it to the footer.
pub struct FooterResult {
    order: ResultOrder,
    words: String,
}

impl FooterResult {
    pub fn words(&self) -> &str {
        &self.words
    }
}

/// The footer's entries for kept results. Each takes a view that only a [`Results`] store makes
/// and maps the stored value to what is drawn; none hands the order to the caller.
///
/// - [`Footer::kept`] takes the [`Shown`] view while the store is borrowed: `words` and `draw`
///   get the value.
/// - [`Footer::kept_line`] takes an owned [`FooterResult`] (made by [`Shown::footer`]), for a
///   caller that draws after the store's borrow ends.
///
/// A typed outcome (package M) adds its entry beside these in `design`, as
/// `footer.kept_outcome(shown, |value| -> Outcome)`, through [`Footer::add_kept`].
impl<'a> Footer<'a> {
    /// A kept result from a [`Results`] store, with the order its action issued. `words` gives
    /// its words (the footer's order and accessible name), and `draw` draws it; both get the
    /// stored value. `has_undo` is true while its Undo is live.
    #[deprecated(
        note = "A footer holds only owned, typed data: make a `KeptLine` with `Shown::outcome` and give it to `Messages::kept` (src/design/outcome.rs)."
    )]
    pub fn kept<T>(
        &mut self,
        shown: Shown<'a, T>,
        has_undo: bool,
        words: impl FnOnce(&'a T) -> std::borrow::Cow<'a, str>,
        draw: impl FnOnce(&'a T, &mut Ui) + 'a,
    ) -> &mut Self {
        let value = shown.value;
        self.add_kept(shown.order, words(value), has_undo, move |ui| {
            draw(value, ui)
        })
    }

    /// A kept result's owned footer line ([`Shown::footer`]): its words, with the order its
    /// action issued. `draw` gets the line's own words.
    #[deprecated(
        note = "A footer holds only owned, typed data: make a `KeptLine` with `Shown::outcome` and give it to `Messages::kept` (src/design/outcome.rs)."
    )]
    pub fn kept_line(
        &mut self,
        line: &'a FooterResult,
        has_undo: bool,
        draw: impl FnOnce(&'a str, &mut Ui) + 'a,
    ) -> &mut Self {
        self.add_kept(line.order, line.words.as_str(), has_undo, move |ui| {
            draw(&line.words, ui)
        })
    }

    /// The one way from a stored order to the footer, for the kept entries in `design` (these,
    /// and a typed outcome's).
    pub(in crate::design) fn add_kept(
        &mut self,
        order: ResultOrder,
        words: impl Into<std::borrow::Cow<'a, str>>,
        has_undo: bool,
        draw: impl FnOnce(&mut Ui) + 'a,
    ) -> &mut Self {
        let rank = if has_undo {
            super::components::FooterRank::Undo
        } else {
            super::components::FooterRank::Notice
        };
        self.push(rank, Some(order), words.into(), draw)
    }
}

struct Entry<O, T> {
    owner: O,
    order: ResultOrder,
    value: T,
}

impl<O: Owner, T> Entry<O, T> {
    /// The same owner in the same scope. The scope is compared by the store, not left to the
    /// owner's `PartialEq`.
    fn is(&self, owner: &O) -> bool {
        self.owner.scope() == owner.scope() && self.owner == *owner
    }
}

/// Every kept result of one kind, each with its owner and its order. See the module comment.
pub struct Results<O, T> {
    entries: Vec<Entry<O, T>>,
}

impl<O, T> Default for Results<O, T> {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

impl<O: Owner, T: Kept<Owner = O>> Results<O, T> {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Keep the result of an action done now. It gets a new order, newer than every result
    /// before it, and it ends its owner's earlier result of this kind. Call it where the action
    /// is done; Undo and then the same action again is a new action with a new order.
    #[allow(
        clippy::disallowed_methods,
        reason = "the store is where an order is issued"
    )]
    pub fn record(&mut self, value: T) {
        let owner = value.owner();
        self.entries.retain(|entry| !entry.is(&owner));
        self.entries.push(Entry {
            owner,
            order: ResultOrder::issue(),
            value,
        });
    }

    /// The owner's result, if it has one.
    pub fn get(&self, owner: &O) -> Option<Shown<'_, T>> {
        self.entries
            .iter()
            .find(|entry| entry.is(owner))
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
        Some(run(&self.entries[at].value).map(|done| (self.entries.remove(at).value, done)))
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
            Ok(done) => Ok((self.entries.remove(at).value, done)),
            Err(error) => Err(ByTicket::Failed(error)),
        }
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
        Some(self.entries.remove(at).value)
    }

    /// The owner changed by its own later action (a saved edit, a rename, a removal).
    pub fn owner_changed(&mut self, owner: &O) {
        match T::ENDS.owner_changed {
            OwnerChanged::Ends => self.entries.retain(|entry| !entry.is(owner)),
            OwnerChanged::Stays => {}
        }
    }

    /// The files changed on disk (a reload, a file event). `holds` says whether a result still
    /// holds; it is asked only for a kind whose `disk_changed` is `Checked`.
    pub fn disk_changed(&mut self, mut holds: impl FnMut(&T) -> bool) {
        match T::ENDS.disk_changed {
            DiskChanged::Ends => self.entries.clear(),
            DiskChanged::Checked => self.entries.retain(|entry| holds(&entry.value)),
            DiskChanged::Stays => {}
        }
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
        (
            "src/campaign_content.rs",
            "new_result! statement",
            &[Ban::Results],
        ),
        ("src/campaign_content.rs", "page_footer", &[Ban::Results]),
        (
            "src/campaign_content.rs",
            "packages_content",
            &[Ban::Extras(2)],
        ),
        (
            "src/campaign_content.rs",
            "repair_section_content",
            &[Ban::Results, Ban::Extras(1)],
        ),
        (
            "src/campaign_content.rs",
            "selection_bar_content",
            &[Ban::Results],
        ),
        ("src/campaigns.rs", "at", &[Ban::Results]),
        ("src/campaigns.rs", "default", &[Ban::Results]),
        ("src/campaigns.rs", "footer_messages", &[Ban::Results]),
        ("src/campaigns.rs", "form", &[Ban::Results]),
        (
            "src/campaigns.rs",
            "one_to_one_links_replace_and_unlink_without_changing_other_campaigns",
            &[Ban::Results],
        ),
        (
            "src/campaigns.rs",
            "register_campaign_copy",
            &[Ban::Results],
        ),
        ("src/campaigns.rs", "save_form", &[Ban::Results]),
        ("src/combat_sync.rs", "footer_messages", &[Ban::Results]),
        ("src/combat_sync.rs", "noticed", &[Ban::Results]),
        (
            "src/design/components.rs",
            "iter26_a_the_footer_keeps_the_body_top_and_reaches_its_last_message",
            &[Ban::Results, Ban::Layout(1), Ban::Pane(1)],
        ),
        (
            "src/design/components.rs",
            "iter27_a_footer_results_go_newest_first_by_their_action",
            &[Ban::Results],
        ),
        (
            "src/design/components.rs",
            "iter27_a_the_footer_follows_a_short_body",
            &[Ban::Results, Ban::Layout(1), Ban::Pane(1)],
        ),
        (
            "src/design/gallery.rs",
            "components",
            &[Ban::Results, Ban::Layout(4)],
        ),
        ("src/design/results.rs", "record", &[Ban::Results]),
        ("src/dndbeyond.rs", "footer_messages", &[Ban::Results]),
        ("src/dndbeyond.rs", "show", &[Ban::Results]),
        ("src/encounter_run.rs", "keep_step", &[Ban::Results]),
        ("src/encounter_run.rs", "open", &[Ban::Results]),
        (
            "src/encounter_run/view.rs",
            "groups_tab",
            &[Ban::Results, Ban::Layout(1), Ban::Pane(1)],
        ),
        (
            "src/encounter_run/view.rs",
            "message_footer",
            &[Ban::Results, Ban::Extras(1), Ban::Pane(1)],
        ),
        (
            "src/encounters/groups.rs",
            "group_editor_view",
            &[Ban::Results, Ban::Extras(1), Ban::Pane(1)],
        ),
        (
            "src/encounters/groups.rs",
            "member_result_into",
            &[Ban::Results],
        ),
        (
            "src/encounters/groups.rs",
            "save_group_draft",
            &[Ban::Results],
        ),
        (
            "src/encounters.rs",
            "creature_pane",
            &[Ban::Results, Ban::Layout(2), Ban::Pane(1), Ban::RowKey(1)],
        ),
        (
            "src/encounters.rs",
            "i20_w1_search_finds_a_hidden_duplicate_and_repairs_it_with_undo",
            &[Ban::Results, Ban::Pane(1)],
        ),
        ("src/encounters.rs", "kept_into_footer", &[Ban::Results]),
        (
            "src/encounters.rs",
            "ledger_view",
            &[Ban::Results, Ban::Layout(1), Ban::Pane(1)],
        ),
        ("src/encounters.rs", "results_into", &[Ban::Results]),
        ("src/encounters.rs", "result_stamp", &[Ban::Results]),
        (
            "src/encounters.rs",
            "run_settings",
            &[Ban::Results, Ban::Layout(2), Ban::Pane(1)],
        ),
        ("src/encounters.rs", "save_draft", &[Ban::Results]),
        ("src/encounters.rs", "stamp_kept", &[Ban::Results]),
        ("src/library.rs", "add_reader_result", &[Ban::Results]),
        // A by-owner Undo and `use_press` of Library's page lines (Wave 2 moves them to tickets).
        (
            "src/library.rs",
            "apply_page_press",
            &[Ban::Results, Ban::ByOwner(2)],
        ),
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
        (
            "src/library.rs",
            "editor_pane",
            &[
                Ban::Results,
                Ban::Layout(4),
                Ban::Extras(3),
                Ban::Pane(3),
                Ban::RowKey(1),
            ],
        ),
        ("src/library.rs", "give_new_identity", &[Ban::Results]),
        (
            "src/library.rs",
            "iter14_e_changed_files_end_every_undo_result",
            &[Ban::Results],
        ),
        ("src/library.rs", "open_entry", &[Ban::Results]),
        ("src/library.rs", "reader_messages", &[Ban::Results]),
        ("src/library.rs", "register_copy", &[Ban::Results]),
        ("src/library.rs", "reload_note_from_disk", &[Ban::Results]),
        ("src/library.rs", "unregister_dialog", &[Ban::Results]),
        ("src/party.rs", "page_footer", &[Ban::Results]),
        ("src/party.rs", "sheet_view", &[Ban::Extras(1)]),
        ("src/party_sync.rs", "campaign_copy_saved", &[Ban::Results]),
        ("src/sessions/carry.rs", "carry_in_footer", &[Ban::Results]),
        ("src/sessions/carry.rs", "record", &[Ban::Results]),
        (
            "src/sessions/page.rs",
            "accept_event_link_offer",
            &[Ban::Results],
        ),
        ("src/sessions/page.rs", "banner_notices", &[Ban::Results]),
        ("src/sessions/page.rs", "folder_messages", &[Ban::Results]),
        ("src/sessions/page.rs", "folder_save_row", &[Ban::Results]),
        (
            "src/sessions/page.rs",
            "markers",
            &[Ban::Results, Ban::Extras(1)],
        ),
        (
            "src/sessions/page.rs",
            "move_run_to_session",
            &[Ban::Results],
        ),
        ("src/sessions/page.rs", "new_notice", &[Ban::Results]),
        (
            "src/sessions/page.rs",
            "reading_view",
            &[Ban::Results, Ban::Layout(1)],
        ),
        (
            "src/sessions/page.rs",
            "session_footer",
            &[Ban::Results, Ban::Pane(1)],
        ),
        (
            "src/sessions/page.rs",
            "template_editor",
            &[Ban::Results, Ban::Layout(3)],
        ),
        (
            "src/sessions/plan.rs",
            "plan_pane",
            // `ByOwner`: `checks::Results::undo(id, book)`, not the footer store's; counted by name.
            &[Ban::Results, Ban::Layout(1), Ban::Pane(1), Ban::ByOwner(1)],
        ),
        (
            "src/sessions/plan.rs",
            "record_deleted_run",
            &[Ban::Results],
        ),
        ("src/sessions/plan.rs", "scene_act", &[Ban::Results]),
        (
            "src/sessions/plan.rs",
            "scenes_are_note_acts_with_their_own_undo statement",
            &[Ban::Results],
        ),
        ("src/sessions/plan.rs", "travel_act", &[Ban::Results]),
        ("src/timeline/page.rs", "into_footer", &[Ban::Results]),
        (
            "src/timeline/page.rs",
            "list_footer",
            &[Ban::Results, Ban::Pane(1)],
        ),
        ("src/timeline/page.rs", "show", &[Ban::Results]),
        ("src/timeline.rs", "next_result_order", &[Ban::Results]),
        ("src/timeline.rs", "remove", &[Ban::Results]),
        ("src/timeline.rs", "touch", &[Ban::Results]),
        ("src/trackers/history.rs", "remove", &[Ban::Results]),
        ("src/trackers/history.rs", "footer", &[Ban::Order]),
        (
            "src/world/edit.rs",
            "edit_form",
            &[Ban::Results, Ban::Layout(1), Ban::Extras(2)],
        ),
        ("src/world/page.rs", "creation_body", &[Ban::Extras(1)]),
        // Package K: raw egui calls that a layout part replaces.
        (
            "src/campaigns.rs",
            "selector",
            &[Ban::Layout(1), Ban::Extras(2)],
        ),
        ("src/copy_dialog.rs", "show", &[Ban::Layout(3)]),
        ("src/creature_filter.rs", "filter_bar", &[Ban::Layout(1)]),
        ("src/creature_filter.rs", "menu", &[Ban::Layout(2)]),
        (
            "src/encounter_run/ad_hoc.rs",
            "add_creature_form",
            &[Ban::Layout(2), Ban::Extras(1)],
        ),
        (
            "src/encounter_run/rolling.rs",
            "row_menu",
            &[Ban::Layout(2)],
        ),
        (
            "src/encounter_run/view.rs",
            "conditions_tab",
            &[Ban::Layout(1), Ban::RowKey(1)],
        ),
        (
            "src/encounter_run/view.rs",
            "ended_view",
            &[Ban::Layout(2), Ban::Extras(1)],
        ),
        (
            "src/encounter_run/view.rs",
            "group_row",
            &[Ban::Layout(3), Ban::Extras(2)],
        ),
        (
            "src/encounter_run/view.rs",
            "live_footer",
            &[Ban::Layout(1)],
        ),
        ("src/encounter_run/view.rs", "log_tab", &[Ban::Layout(1)]),
        ("src/encounter_run/view.rs", "notes_tab", &[Ban::Layout(1)]),
        (
            "src/encounter_run/view.rs",
            "optional_number",
            &[Ban::Layout(1)],
        ),
        ("src/encounter_run/view.rs", "panel_tabs", &[Ban::Layout(1)]),
        (
            "src/encounter_run/view.rs",
            "record_tab",
            &[Ban::Layout(1), Ban::Extras(1), Ban::RowKey(1)],
        ),
        (
            "src/encounter_run/view.rs",
            "reference_tab",
            &[Ban::Layout(2), Ban::Pane(1)],
        ),
        ("src/encounter_run/view.rs", "roster", &[Ban::Layout(1)]),
        (
            "src/encounter_run/view.rs",
            "setup_body",
            &[Ban::Layout(1), Ban::Extras(5)],
        ),
        (
            "src/encounter_run/view.rs",
            "setup_row",
            &[Ban::Layout(1), Ban::Extras(7)],
        ),
        (
            "src/encounters.rs",
            "creature_search_with_summary",
            &[Ban::Layout(1), Ban::Extras(1)],
        ),
        (
            "src/encounters.rs",
            "editor_view",
            &[Ban::Pane(1), Ban::Extras(1)],
        ),
        ("src/encounters.rs", "notes_field", &[Ban::Layout(1)]),
        (
            "src/encounters.rs",
            "roster",
            &[Ban::Layout(1), Ban::RowKey(1)],
        ),
        ("src/encounters.rs", "roster_pane", &[Ban::Layout(1)]),
        ("src/encounters/groups.rs", "belongs_to", &[Ban::Layout(1)]),
        (
            "src/encounters/groups.rs",
            "change_row",
            &[Ban::Layout(3), Ban::Extras(1)],
        ),
        ("src/encounters/groups.rs", "members", &[Ban::Layout(1)]),
        (
            "src/encounters/groups.rs",
            "members_pane",
            &[Ban::Layout(1)],
        ),
        ("src/encounters/list.rs", "row_menu", &[Ban::Layout(1)]),
        ("src/encounters/list.rs", "side_control", &[Ban::Layout(2)]),
        ("src/library.rs", "above_footer", &[Ban::Layout(1)]),
        ("src/library.rs", "folder_field", &[Ban::Layout(1)]),
        (
            "src/library.rs",
            "new_entry",
            &[Ban::Layout(3), Ban::Extras(2)],
        ),
        (
            "src/library.rs",
            "package_form",
            &[Ban::Layout(5), Ban::Extras(5)],
        ),
        ("src/library.rs", "scope_menu", &[Ban::Layout(1)]),
        ("src/library.rs", "show", &[Ban::Layout(6)]),
        ("src/lookup.rs", "written_reader", &[Ban::Extras(1)]),
        (
            "src/loot_editor.rs",
            "line_row",
            &[Ban::Layout(1), Ban::Extras(1)],
        ),
        ("src/loot_editor.rs", "table_menu", &[Ban::Layout(1)]),
        (
            "src/markdown_source.rs",
            "press_in_editor",
            &[Ban::Layout(1)],
        ),
        (
            "src/markdown_source.rs",
            "the_first_letter_of_a_wrapped_row_is_part_of_its_link",
            &[Ban::Layout(1)],
        ),
        (
            "src/markdown_view.rs",
            "reader_table",
            &[Ban::Layout(1), Ban::Extras(1)],
        ),
        (
            "src/markdown_view.rs",
            "wide_tables_keep_whole_words_and_can_scroll_to_the_last_column",
            &[Ban::Layout(1)],
        ),
        (
            "src/processing/settings.rs",
            "input_section",
            &[Ban::Layout(1)],
        ),
        ("src/processing/settings.rs", "key_row", &[Ban::Extras(1)]),
        (
            "src/processing/settings.rs",
            "llm_section",
            &[Ban::Layout(3), Ban::Extras(1)],
        ),
        (
            "src/processing/settings.rs",
            "speech_section",
            &[Ban::Layout(3), Ban::Extras(1)],
        ),
        ("src/rolls.rs", "mode_menu", &[Ban::Layout(1)]),
        ("src/rolls.rs", "more_actions", &[Ban::Layout(1)]),
        ("src/rolls.rs", "outcome_line", &[Ban::Layout(1)]),
        (
            "src/rolls.rs",
            "rules_on_hover_and_focus",
            &[Ban::Layout(2)],
        ),
        (
            "src/sessions/page.rs",
            "banner",
            &[Ban::Layout(1), Ban::Extras(2)],
        ),
        ("src/sessions/page.rs", "list_pane", &[Ban::Layout(1)]),
        (
            "src/sessions/page.rs",
            "new_template_form",
            &[Ban::Extras(2)],
        ),
        ("src/sessions/page.rs", "note_editor", &[Ban::Layout(2)]),
        ("src/sessions/page.rs", "notes_view", &[Ban::Layout(3)]),
        ("src/sessions/page.rs", "read_tab", &[Ban::Layout(1)]),
        ("src/sessions/page.rs", "recordings_tab", &[Ban::Layout(1)]),
        ("src/sessions/page.rs", "template_list", &[Ban::Layout(1)]),
        ("src/sessions/plan.rs", "fight_block", &[Ban::Layout(1)]),
        ("src/sessions/plan.rs", "plan_menu", &[Ban::Layout(1)]),
        ("src/sessions/slash.rs", "note", &[Ban::Layout(1)]),
        ("src/sessions/slash.rs", "popup_at", &[Ban::Layout(1)]),
        ("src/timeline/checks.rs", "table_line", &[Ban::Layout(1)]),
        (
            "src/timeline/page.rs",
            "detail",
            // `ByOwner`: `checks::Results::undo(id, book)`, not the footer store's; counted by name.
            &[Ban::Layout(7), Ban::ByOwner(1)],
        ),
        ("src/timeline/page.rs", "filters", &[Ban::Layout(1)]),
        ("src/timeline/page.rs", "list", &[Ban::Layout(1)]),
        ("src/timeline/page.rs", "new_events_line", &[Ban::Layout(1)]),
        (
            "src/trackers.rs",
            "an_open_history_prepares_its_rows_once_and_draws_only_those_in_view",
            &[Ban::Layout(1)],
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
        ("src/trackers.rs", "act_words", &[Ban::Words]),
        ("src/world/repair.rs", "cause", &[Ban::Words]),
        ("src/world.rs", "said", &[Ban::Words]),
        ("src/trackers.rs", "cause", &[Ban::Words]),
        ("src/trackers/history.rs", "show", &[Ban::Body]),
        ("src/trackers/view.rs", "body", &[Ban::Body]),
        ("src/trackers/view.rs", "messages", &[Ban::Words]),
        (
            "src/encounter_run/loot.rs",
            "loot_section",
            &[Ban::Extras(1)],
        ),
        (
            "src/encounter_run/notice.rs",
            "live_header",
            &[Ban::Extras(2)],
        ),
        (
            "src/encounter_run/view.rs",
            "correction_reason",
            &[Ban::Extras(1)],
        ),
        (
            "src/encounter_run/view.rs",
            "dismissable",
            &[Ban::Extras(1)],
        ),
        ("src/encounter_run/view.rs", "live_view", &[Ban::Extras(1)]),
        ("src/encounter_run/view.rs", "toggle", &[Ban::Extras(1)]),
        (
            "src/encounter_run/view.rs",
            "setup_tie_controls",
            &[Ban::Extras(1)],
        ),
        ("src/encounter_run/view.rs", "log_link", &[Ban::Extras(1)]),
        ("src/sessions/plan.rs", "word_link", &[Ban::Extras(1)]),
        ("src/sessions/plan.rs", "plan_row_with", &[Ban::Extras(1)]),
        (
            "src/encounters.rs",
            "replace_missing_creature",
            &[Ban::Extras(1)],
        ),
        ("src/encounters.rs", "add_pane", &[Ban::Extras(1)]),
        ("src/encounters.rs", "loot_field", &[Ban::Extras(1)]),
        ("src/encounters/list.rs", "heading", &[Ban::Extras(1)]),
        (
            "src/encounters/groups.rs",
            "choice_label",
            &[Ban::Extras(1)],
        ),
        (
            "src/encounters/groups.rs",
            "conditions_row",
            &[Ban::RowKey(1)],
        ),
        (
            "src/encounters/groups.rs",
            "group_fields",
            &[Ban::Extras(2)],
        ),
        (
            "src/encounters/groups.rs",
            "overrides_form",
            &[Ban::Extras(1), Ban::RowKey(2)],
        ),
        (
            "src/encounters/groups.rs",
            "unread_member",
            &[Ban::Extras(1)],
        ),
        ("src/encounters/list.rs", "choice_note", &[Ban::Extras(2)]),
        ("src/encounters/list.rs", "history_list", &[Ban::Extras(2)]),
        (
            "src/encounters/list.rs",
            "history_summary",
            &[Ban::Extras(6)],
        ),
        ("src/library.rs", "duplicate_choices", &[Ban::Extras(2)]),
        ("src/library.rs", "form_heading", &[Ban::Extras(1)]),
        ("src/library.rs", "kept_undo_message", &[Ban::Extras(2)]),
        ("src/library.rs", "recovery_action", &[Ban::Extras(2)]),
        ("src/library.rs", "repair_notice", &[Ban::Extras(1)]),
        ("src/library.rs", "use_checkbox", &[Ban::Extras(1)]),
        ("src/library/creature.rs", "show", &[Ban::Extras(1)]),
        ("src/library/reader.rs", "show", &[Ban::Extras(1)]),
        ("src/markdown_view.rs", "flush", &[Ban::Extras(1)]),
        (
            "src/processing/settings.rs",
            "field_reason",
            &[Ban::Extras(2)],
        ),
        ("src/rolls.rs", "armed_hint", &[Ban::Extras(2)]),
        ("src/sessions/carry.rs", "last_time_line", &[Ban::Extras(1)]),
        ("src/sessions/page.rs", "draft_warning", &[Ban::Extras(2)]),
        ("src/sessions/page.rs", "record_line", &[Ban::Extras(1)]),
        ("src/sessions/page.rs", "recording_row", &[Ban::Extras(2)]),
        (
            "src/sessions/plan.rs",
            "gone_scene_words",
            &[Ban::Extras(1)],
        ),
        ("src/sessions/plan.rs", "plan_details", &[Ban::Extras(1)]),
        ("src/sessions/plan.rs", "scene_words", &[Ban::Extras(1)]),
        ("src/timeline/checks.rs", "result", &[Ban::Extras(1)]),
        ("src/timeline/page.rs", "problems", &[Ban::Extras(1)]),
        (
            "src/timeline/page.rs",
            "rest_ruling_line",
            &[Ban::Extras(1)],
        ),
        ("src/world.rs", "draw", &[Ban::Extras(4)]),
        ("src/world.rs", "missing_link", &[Ban::Extras(1)]),
        ("src/world.rs", "unread_groups_message", &[Ban::Extras(1)]),
        ("src/world.rs", "value_row", &[Ban::Extras(1)]),
        ("src/world/page.rs", "list", &[Ban::Extras(1)]),
        ("src/world/page.rs", "unread_pane", &[Ban::Extras(1)]),
        ("src/world/related.rs", "history_rows", &[Ban::Extras(1)]),
        (
            "src/encounters/groups.rs",
            "editor_members",
            &[Ban::Pane(1)],
        ),
        (
            "src/encounters/groups.rs",
            "a_group_changed_on_disk_blocks_the_whole_save_and_keeps_the_draft",
            &[Ban::Pane(1)],
        ),
        ("src/sessions/carry.rs", "harness", &[Ban::Pane(1)]),
        ("src/sessions/page.rs", "draft_footer", &[Ban::Pane(1)]),
        ("src/sessions/page.rs", "notes_panel", &[Ban::Pane(1)]),
        ("src/sessions/page.rs", "templates_view", &[Ban::Pane(1)]),
        (
            "src/encounters.rs",
            "integrity_d8_a_changed_encounter_compares_and_keeps_unknown_fields",
            &[Ban::Pane(1)],
        ),
        ("src/library.rs", "form_footer", &[Ban::Pane(1)]),
        ("src/campaign_content.rs", "show", &[Ban::Pane(1)]),
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
            "src/design/components.rs",
            "iter28_p_the_pane_places_its_footer_for_every_body",
            &[Ban::Layout(1)],
        ),
        (
            "src/design/components.rs",
            "add_field",
            &[Ban::Layout(1), Ban::Extras(1)],
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
        (
            "src/design/components.rs",
            "stepper_with_density",
            &[Ban::Layout(1)],
        ),
        (
            "src/design/components.rs",
            "text_field_label",
            &[Ban::Layout(1)],
        ),
        ("src/design/components.rs", "text_input", &[Ban::Layout(1)]),
        ("src/design/components.rs", "paint", &[Ban::Extras(1)]),
        ("src/design/extras.rs", "space", &[Ban::Extras(1)]),
        ("src/design/gallery.rs", "markdown", &[Ban::Layout(1)]),
        ("src/design/gallery.rs", "show", &[Ban::Layout(1)]),
        ("src/design/layout.rs", "area", &[Ban::Layout(2)]),
        ("src/design/layout.rs", "editor", &[Ban::Layout(1)]),
        (
            "src/design/layout.rs",
            "field_id! statement",
            &[Ban::Layout(1)],
        ),
        ("src/design/layout.rs", "menu_popup", &[Ban::Layout(1)]),
        ("src/design/layout.rs", "multi_line", &[Ban::Layout(1)]),
        ("src/design/layout.rs", "sideways_scroll", &[Ban::Layout(1)]),
        ("src/design/layout.rs", "single_line", &[Ban::Layout(1)]),
        ("src/design/layout.rs", "word! statement", &[Ban::Layout(1)]),
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
            &[Ban::Layout(2)],
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
/// - `ptr_eq(`: a cache keyed by a pointer address. Key it by a `Revision` in a
///   `design::memo::Memo`, which takes no other key.
#[cfg(test)]
pub(crate) type Staged = (&'static str, &'static str, &'static [(&'static str, usize)]);

/// See [`Staged`]: the pattern, what to use instead, and the count of each file.
#[cfg(test)]
pub(crate) const STAGED: &[Staged] = &[
    (
        "ResultOrder::issue(",
        "Record the result in a `design::Results` store: it issues the order at the action.",
        &[
            ("src/campaign_content.rs", 1),
            ("src/campaigns.rs", 9),
            ("src/combat_sync.rs", 1),
            ("src/design/components.rs", 16),
            ("src/dndbeyond.rs", 1),
            ("src/encounter_run.rs", 2),
            ("src/encounters.rs", 4),
            ("src/encounters/groups.rs", 2),
            ("src/library.rs", 16),
            ("src/party_sync.rs", 1),
            ("src/sessions/carry.rs", 1),
            ("src/sessions/page.rs", 8),
            ("src/sessions/plan.rs", 8),
            ("src/timeline.rs", 3),
            ("src/timeline/page.rs", 2),
            ("src/trackers/history.rs", 1),
        ],
    ),
    (
        "save_row_state(",
        "Use `design::save_row` with a `DraftState`: it decides the word, Save's weight and its reason.",
        &[
            ("src/campaign_content.rs", 2),
            ("src/campaigns.rs", 1),
            ("src/design/components.rs", 1),
            ("src/design/gallery.rs", 2),
            ("src/encounters.rs", 3),
            ("src/world/edit.rs", 1),
        ],
    ),
    (
        "ptr_eq(",
        "Key the cache by a `Revision` in a `design::memo::Memo`; a pointer address is not a change.",
        &[
            ("src/content_workspace.rs", 1),
            ("src/design/comparison.rs", 1),
            ("src/encounter_run/undo.rs", 2),
            ("src/encounter_run/view.rs", 2),
            ("src/library.rs", 2),
            ("src/markdown_view.rs", 3),
            ("src/rolls.rs", 1),
            ("src/trackers.rs", 1),
        ],
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

    fn done(note: Note, words: &'static str) -> Done {
        Done { note, words }
    }

    fn words<'a>(results: &'a Results<Note, Done>, scope: &'a str) -> Vec<&'a str> {
        results
            .newest_first(scope)
            .map(|shown| shown.value().words)
            .collect()
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
        let failed = results.undo(&A, |_| Err::<(), _>("A's file changed."));
        assert_eq!(failed, Some(Err("A's file changed.")));
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

    /// The footer takes each kept result with the order its own action issued: added oldest
    /// first, the newer one still leads; then the older one again, and it leads. `words` and
    /// `draw` get the stored value.
    #[allow(
        deprecated,
        reason = "the test of `Footer::kept`, deprecated until Wave 3 removes it"
    )]
    #[test]
    fn the_footer_takes_each_kept_result_with_its_own_order() {
        use egui_kittest::kittest::Queryable;
        let mut results = Results::new();
        results.record(done(A, "Links now say “Ash Wolf”."));
        results.record(done(B, "Links now say “Veilglass Lantern”."));
        let tops = |results: &Results<Note, Done>| {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(eframe::egui::vec2(600.0, 300.0))
                .build_ui(|ui| {
                    let mut footer = Footer::new("kept_test", 500.0);
                    for owner in [&A, &B] {
                        let shown = results.get(owner).unwrap();
                        footer.kept(
                            shown,
                            true,
                            |done| done.words.into(),
                            |done, ui| {
                                ui.label(done.words);
                            },
                        );
                    }
                    footer.pane(ui, crate::design::Body::Content, |_| ());
                });
            crate::design::theme::install(&harness.ctx);
            harness.run();
            let top = |words: &str| harness.get_by_label(words).rect().top();
            (
                top("Links now say “Ash Wolf”."),
                top("Links now say “Veilglass Lantern”."),
            )
        };
        let (a, b) = tops(&results);
        assert!(b < a, "B is newer, so it leads: A {a}, B {b}");
        results.record(done(A, "Links now say “Ash Wolf”."));
        let (a, b) = tops(&results);
        assert!(a < b, "A again is newest: A {a}, B {b}");
    }

    /// A ticket names one entry: after its owner's later action replaced it, or an end event
    /// ended it, the ticket's Undo is `Gone` and changes nothing; the other entries stay.
    #[test]
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

    /// The staged counts of `STAGED`: a new call fails, and a lower count asks to lower it.
    #[test]
    fn staged_calls_only_shrink() {
        let mut files = Vec::new();
        collect(std::path::Path::new("src"), &mut files);
        let mut faults = Vec::new();
        for (pattern, instead, allowed) in STAGED {
            for (path, text) in &files {
                // The store itself, and this table, name the patterns.
                if path == "src/design/results.rs" {
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

    /// Every suppression of `clippy::disallowed_methods` is a row of `STAGED_ALLOWS` (see
    /// there); the checker's own cases are in `suppressions::tests`.
    #[test]
    fn staged_allows_only_shrink() {
        let mut files = Vec::new();
        collect(std::path::Path::new("src"), &mut files);
        for (file, item, bans) in OWNED_ALLOWS {
            assert!(
                (file.starts_with("src/design/") || file.starts_with("src/visual_tests/"))
                    && bans.iter().all(|ban| matches!(
                        ban,
                        suppressions::Ban::Layout(_)
                            | suppressions::Ban::RowKey(_)
                            | suppressions::Ban::Extras(_)
                    )),
                "{file} `{item}`: only the design system and its harness own raw egui calls"
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
        collect(std::path::Path::new("src"), &mut files);
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
                files.push((path.to_string_lossy().replace('\\', "/"), text));
            }
        }
    }
}
