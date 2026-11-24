//! The Library's kept results (M-L0.4a, ruling M14): one mark store and the data it marks.
//!
//! Each kept result (an Undo of a write, a discarded draft, a notice that stays) has its data
//! here and its mark in one [`design::Results`] store. The store gives the result its footer
//! order when its act is done, and its ticket in a typed footer (PR 4b). The data and the mark
//! change together: the fields are private to this module, so a writer can only `set` a result
//! (data and mark in one call) or `take` it (data out, mark ended). No writer can store data
//! without its mark, and no mark outlives its data.
//!
//! Every kind but a copy holds one result at a time: a new act of that kind replaces the older
//! result, whose order (and ticket) ends with it. Copies are several per file, each owned by the
//! file it linked and the path it copied. Scope is `()`: a campaign switch hides none of them;
//! each kind's own end events decide. See `KeptMark::ENDS` and `forget_changed_undos`.
#![warn(deprecated)] // Its parent allows old message functions; this file does not.
use super::{DependencyUndo, FileId, IdentityUndo, LetGo, OwnerKey, Scope, SelectionUndo};
use crate::content_workspace::{ContentEdits, ManifestWrite};
use crate::design::{
    self,
    outcome::{KeptLine, KeptOutcome, PaneFrame, Sentence},
    results::{DiskChanged, Ends, Kept, Owner, OwnerChanged, Press, Ticket},
};
use crate::persistence::Module;
use std::path::PathBuf;
use uuid::Uuid;

/// Which kept result a mark is.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum KeptKind {
    ScopeNotice,
    Unregistered,
    PackageUse,
    Manifest,
    Identities,
    Identity,
    Discarded,
    Removed,
    Heading,
    Vital,
    Copy,
}

/// Whose a mark is: its kind, and for a copy the file it linked (its package root and path) and
/// the path it copied, which are absolute and complete.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct KeptOwner {
    kind: KeptKind,
    copy: Option<(PathBuf, String, String)>,
}

impl KeptOwner {
    fn of(kind: KeptKind) -> Self {
        Self { kind, copy: None }
    }

    fn copy(undo: &DependencyUndo) -> Self {
        Self {
            kind: KeptKind::Copy,
            copy: Some((
                undo.root.clone(),
                undo.path.clone(),
                undo.copied_path.clone(),
            )),
        }
    }
}

impl Owner for KeptOwner {
    type Scope = ();
    fn scope(&self) -> &() {
        &()
    }
}

/// A kept result's mark: its owner, for its footer order (and its ticket in a typed footer).
pub(super) struct KeptMark(KeptOwner);

impl Kept for KeptMark {
    type Owner = KeptOwner;
    const ENDS: Ends = Ends {
        // Its data was taken or replaced (`take_*`, a newer `set_*`): the mark goes with it.
        owner_changed: OwnerChanged::Ends,
        // A reload keeps the mark; the kind's own check takes the data (`forget_changed_undos`).
        disk_changed: DiskChanged::Stays,
        // Its press is its Undo (a scope notice has none and is never pressed).
        press: Press::Undo,
    };
    fn owner(&self) -> KeptOwner {
        self.0.clone()
    }
}

/// A heading fix: its file, the draft before and the draft the fix wrote.
pub(super) type HeadingFix = (FileId, String, crate::design::memo::Revised<String>);
/// A stat fix: its file, the draft before, the draft the fix wrote and its result's words.
pub(super) type StatFix = (
    FileId,
    String,
    crate::design::memo::Revised<String>,
    Sentence,
);
/// A discarded draft: its file (owner and path) and the words Discard dropped.
pub(super) type DiscardedDraft = (OwnerKey, String, String);
/// Why the scope widened: the entry it shows, by name, and the scope it widened to. The notice
/// stands while the list keeps that scope and has no query (M-L0.4b).
pub(super) type ScopeWidened = (String, Scope);

/// The Library's kept results, behind the one boundary that changes data and mark together.
#[derive(Default)]
pub(super) struct LibraryKept {
    marks: design::Results<KeptOwner, KeptMark>,
    scope_notice: Option<ScopeWidened>,
    unregistered: Option<Module>,
    package_use: Option<SelectionUndo>,
    manifest: Option<ManifestWrite>,
    identities: Option<(ContentEdits, Sentence)>,
    identity: Option<IdentityUndo>,
    discarded: Option<DiscardedDraft>,
    removed: Option<LetGo>,
    heading: Option<HeadingFix>,
    vital: Option<StatFix>,
    copies: Vec<DependencyUndo>,
}

/// For each one-result kind: its read, its `set` (data and mark in one call: the newest order)
/// and its `take` (data out, mark ended).
macro_rules! slot {
    ($($kind:ident: $field:ident: $ty:ty => $get:ident, $set:ident, $take:ident;)*) => {$(
        pub(super) fn $get(&self) -> Option<&$ty> {
            self.$field.as_ref()
        }

        pub(super) fn $set(&mut self, value: $ty) {
            self.$field = Some(value);
            self.marks.record(KeptMark(KeptOwner::of(KeptKind::$kind)));
        }

        pub(super) fn $take(&mut self) -> Option<$ty> {
            let value = self.$field.take();
            if value.is_some() {
                self.marks.owner_changed(&KeptOwner::of(KeptKind::$kind));
            }
            value
        }
    )*};
}

impl LibraryKept {
    slot! {
        ScopeNotice: scope_notice: ScopeWidened => scope_notice, set_scope_notice, take_scope_notice;
        Unregistered: unregistered: Module => unregistered, set_unregistered, take_unregistered;
        PackageUse: package_use: SelectionUndo => package_use, set_package_use, take_package_use;
        Manifest: manifest: ManifestWrite => manifest, set_manifest, take_manifest;
        Identities: identities: (ContentEdits, Sentence) => identities, set_identities, take_identities;
        Identity: identity: IdentityUndo => identity, set_identity, take_identity;
        Discarded: discarded: DiscardedDraft => discarded, set_discarded, take_discarded;
        Removed: removed: LetGo => removed, set_removed, take_removed;
        Heading: heading: HeadingFix => heading, set_heading, take_heading;
        Vital: vital: StatFix => vital, set_vital, take_vital;
    }

    /// The open campaign, as the app gives it each frame: Remove's record keeps the first one it
    /// sees (its `used` flag and catalogue are that campaign's). At a switch to another, Remove's
    /// Undo ends with the draft it keeps. The one change of a kept payload in place: no caller
    /// gets a mutable payload, so none can swap one result's data under another's mark.
    pub(super) fn note_removed_campaign(&mut self, campaign: Option<Uuid>) {
        let Some(gone) = self.removed.as_mut() else {
            return;
        };
        if *gone.campaign.get_or_insert(campaign) != campaign {
            let _ = self.take_removed();
            let _ = self.take_discarded();
        }
    }

    /// A reload's check of the manifest write: it goes, with its mark, when its file changed.
    pub(super) fn forget_manifest_if_changed(&mut self) {
        let had = self.manifest.is_some();
        ManifestWrite::forget_if_changed(&mut self.manifest);
        if had && self.manifest.is_none() {
            self.marks.owner_changed(&KeptOwner::of(KeptKind::Manifest));
        }
    }

    /// The copies, oldest first.
    pub(super) fn copies(&self) -> &[DependencyUndo] {
        &self.copies
    }

    /// A new copy, the newest result: its own mark.
    pub(super) fn push_copy(&mut self, undo: DependencyUndo) {
        self.marks.record(KeptMark(KeptOwner::copy(&undo)));
        self.copies.push(undo);
    }

    /// The copy at `at` leaves (its Undo worked), with its mark.
    pub(super) fn remove_copy(&mut self, at: usize) -> DependencyUndo {
        let undo = self.copies.remove(at);
        self.marks.owner_changed(&KeptOwner::copy(&undo));
        undo
    }

    /// Keeps the copies that `keep` holds; the others leave with their marks.
    pub(super) fn retain_copies(&mut self, mut keep: impl FnMut(&DependencyUndo) -> bool) {
        let marks = &mut self.marks;
        self.copies.retain(|undo| {
            let kept = keep(undo);
            if !kept {
                marks.owner_changed(&KeptOwner::copy(undo));
            }
            kept
        });
    }

    /// The kind of the result `ticket` names, while that result stands: a press on its line acts
    /// on it, in the frame of the press. A result replaced (a newer act of its kind) or ended
    /// since its line was drawn has another order, or none: `None`, and nothing is done.
    pub(super) fn kind_by(&self, ticket: &Ticket) -> Option<KeptKind> {
        self.marks.get_by(ticket).map(|shown| shown.value().0.kind)
    }

    /// The typed footer line of the one-result kind `kind`, made by `make` from its data (with
    /// the order its act issued, and its ticket), or `None` when it has no mark or `make` none.
    /// `make` runs only when it has one.
    pub(super) fn outcome<'f, A>(
        &self,
        frame: &'f PaneFrame,
        kind: KeptKind,
        make: impl FnOnce() -> Option<KeptOutcome<A>>,
    ) -> Option<KeptLine<'f, A>> {
        let shown = self.marks.get(&KeptOwner::of(kind))?;
        let outcome = make()?;
        Some(shown.outcome(frame, |_| outcome))
    }

    /// The typed footer line of one copy, made by `make`, or `None` when it has no mark.
    pub(super) fn copy_outcome<'f, A>(
        &self,
        frame: &'f PaneFrame,
        undo: &DependencyUndo,
        make: impl FnOnce() -> KeptOutcome<A>,
    ) -> Option<KeptLine<'f, A>> {
        let shown = self.marks.get(&KeptOwner::copy(undo))?;
        Some(shown.outcome(frame, |_| make()))
    }

    /// The place in `copies` of the copy `ticket` names, while it stands.
    pub(super) fn copy_at(&self, ticket: &Ticket) -> Option<usize> {
        let owner = self.marks.get_by(ticket)?.value().0.clone();
        self.copies
            .iter()
            .position(|undo| KeptOwner::copy(undo) == owner)
    }

    /// The ticket of the result of `kind`, for the tests of a press by ticket.
    #[cfg(test)]
    pub(super) fn ticket_of(&self, kind: KeptKind) -> Option<Ticket> {
        self.marks
            .get(&KeptOwner::of(kind))
            .map(|shown| shown.ticket_for_caller_test())
    }

    /// The ticket of one copy's result, for the tests of a press by ticket.
    #[cfg(test)]
    pub(super) fn copy_ticket_for_test(&self, undo: &DependencyUndo) -> Option<Ticket> {
        self.marks
            .get(&KeptOwner::copy(undo))
            .map(|shown| shown.ticket_for_caller_test())
    }

    /// The order each mark's act issued, newest first, for the tests of the footer order.
    #[cfg(test)]
    pub(super) fn kinds_newest_first(&self) -> Vec<KeptKind> {
        self.marks
            .newest_first(&())
            .map(|shown| shown.value().0.kind)
            .collect()
    }

    /// The order the mark of `kind` holds, for the tests of `set` and replacement.
    #[cfg(test)]
    pub(super) fn order_of(&self, kind: KeptKind) -> Option<design::ResultOrder> {
        self.marks
            .get(&KeptOwner::of(kind))
            .map(|shown| shown.order_for_caller_test())
    }

    /// How many marks the store holds, for the tests that no mark outlives its data.
    #[cfg(feature = "test-support")]
    pub(super) fn marks(&self) -> usize {
        self.marks.newest_first(&()).count()
    }
}
