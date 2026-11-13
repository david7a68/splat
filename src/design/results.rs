//! Kept results and their footer order: one store per kind of kept result, keyed by the object
//! that owns each result.
//!
//! A kept result is a message that stays after its action (`Links now say “Ash Wolf”.` with
//! `Undo link update`, an offer to change links after a rename). [`Results`] holds every result
//! of one kind with three things that must never come apart:
//!
//! - **Its owner.** [`Kept::owner`] names the object (an entry in its root, a note, a run step).
//!   The store takes the owner from the value, so a result cannot be filed under another
//!   object, and a second object's result never shares the first one's slot.
//! - **Its order.** [`Results::record`] issues the [`ResultOrder`] when the action is done. The
//!   order lives in the entry: nothing else holds it, so it cannot be keyed by kind, and a draw,
//!   which gets `&Results`, cannot issue or renew one.
//! - **Its end events.** [`Kept::ENDS`] says, for every end event, what it does to this kind.
//!   An Undo leaves the store only when it works: [`Results::undo`] keeps the entry on `Err`.
//!
//! Caller duties left: call the end events where they happen ([`Results::owner_changed`] after
//! a saved edit of the owner, [`Results::disk_changed`] after a reload or a file event), and give
//! the footer the order from [`Shown::order`]. Order, owner, scope and the failed Undo need no
//! proof in the caller: the tests below hold them.
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
//! struct Done { entry: Entry }
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
//! results.record(Done { entry: a.clone() });
//! // The twin of each case below:
//! fn draw(results: &Results<Entry, Done>, a: &Entry) -> Option<splat::ownership::ResultOrder> {
//!     results.get(a).map(|shown| shown.order())
//! }
//! assert!(draw(&results, &a).is_some());
//! let undone = results.undo(&a, |_| Ok::<(), String>(()));
//! assert!(matches!(undone, Some(Ok(_))));
//! ```
//!
//! A draw gets `&Results`, so it cannot record a result or issue its order:
//!
//! ```compile_fail
//! # use splat::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
//! # #[derive(Clone, PartialEq, Eq)] struct Entry { id: u32, root: String }
//! # impl Owner for Entry { type Scope = str; fn scope(&self) -> &str { &self.root } }
//! # struct Done { entry: Entry }
//! # impl Kept for Done { type Owner = Entry; const ENDS: Ends = Ends { owner_changed: OwnerChanged::Ends, disk_changed: DiskChanged::Checked, press: Press::Undo }; fn owner(&self) -> Entry { self.entry.clone() } }
//! fn draw(results: &Results<Entry, Done>, a: &Entry) {
//!     results.record(Done { entry: a.clone() }); // <- issues an order while drawing
//! }
//! ```
//!
//! An order is never made by hand; it comes from the store's entry:
//!
//! ```compile_fail
//! let order = splat::ownership::ResultOrder(7); // <- an order from a number
//! ```
//!
//! A footer item's order and value come from the store only:
//!
//! ```compile_fail
//! # use splat::ownership::{ResultOrder, Shown};
//! let shown = Shown { order: ResultOrder::default(), value: &"Links now say “Ash Wolf”." };
//! ```
//!
//! There is no way to take an entry out except its end events and its press, so a failed Undo
//! cannot drop the change (review of PR 299):
//!
//! ```compile_fail
//! # use splat::ownership::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Results};
//! # #[derive(Clone, PartialEq, Eq)] struct Entry { id: u32, root: String }
//! # impl Owner for Entry { type Scope = str; fn scope(&self) -> &str { &self.root } }
//! # struct Done { entry: Entry }
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
//! # struct Done { entry: Entry }
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
use super::components::ResultOrder;

/// The object a kept result belongs to: an entry in its root, a note, a run step. Two results of
/// one kind with equal owners are one result: the newer one ends the older.
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

/// A kept result as the footer shows it: its value and the order its action issued. Only the
/// store makes one.
pub struct Shown<'a, T> {
    order: ResultOrder,
    value: &'a T,
}

impl<'a, T> Shown<'a, T> {
    /// The order its action issued, for `Footer::result`.
    pub fn order(&self) -> ResultOrder {
        self.order
    }
    pub fn value(&self) -> &'a T {
        self.value
    }
}

struct Entry<O, T> {
    owner: O,
    order: ResultOrder,
    value: T,
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
    pub fn record(&mut self, value: T) {
        let owner = value.owner();
        self.entries.retain(|entry| entry.owner != owner);
        self.entries.push(Entry {
            owner,
            // The one place outside `components` that issues an order (see `staged`).
            order: ResultOrder::issue(),
            value,
        });
    }

    /// The owner's result, if it has one.
    pub fn get(&self, owner: &O) -> Option<Shown<'_, T>> {
        self.entries
            .iter()
            .find(|entry| entry.owner == *owner)
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
        let at = self
            .entries
            .iter()
            .position(|entry| entry.owner == *owner)?;
        Some(run(&self.entries[at].value).map(|done| (self.entries.remove(at).value, done)))
    }

    /// The owner's offer, used up by its press whatever the action's outcome.
    pub fn use_press(&mut self, owner: &O) -> Option<T> {
        const {
            assert!(
                matches!(T::ENDS.press, Press::Use),
                "this kind is not used up by its press: its press is not `Press::Use`"
            )
        };
        let at = self
            .entries
            .iter()
            .position(|entry| entry.owner == *owner)?;
        Some(self.entries.remove(at).value)
    }

    /// The owner changed by its own later action (a saved edit, a rename, a removal).
    pub fn owner_changed(&mut self, owner: &O) {
        match T::ENDS.owner_changed {
            OwnerChanged::Ends => self.entries.retain(|entry| entry.owner != *owner),
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

/// Calls of parts that a shared part replaces, by file. A count can only go down: a new call
/// fails, and a file with fewer calls than its count asks to lower the count. When a pattern's
/// counts are all 0, its old part becomes private and the compiler has the last word.
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
            ("src/world.rs", 2),
            ("src/world/page.rs", 2),
            ("src/world/repair.rs", 2),
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
            ("src/world/repair.rs", 1),
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
    }

    #[test]
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
