//! Encounter groups: their notes, their editor, and the overrides form they share with the
//! roster. A group is prepared here and added to a run from the run's own panel.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{Creature, EditorPane, Encounters, Snapshot};
use crate::design::memo::Revised;
use crate::design::text_draft::{KeptCompare, TextDraft};
use crate::{
    content_workspace::{CampaignDocument, Catalogue, EntryRef},
    design::{self, Tone},
    difficulty::creature_xp,
    encounter_run::{FaultKind, GroupMember, MemberFault, RunGroup},
    library::{note_path, write_note},
    persistence::{Campaign, Module},
};
use eframe::egui;
use serde::{Deserialize, Serialize};
use serde_yaml_ng::{Mapping, Value};
use splat::content::{
    Entry,
    encounter_group::{EncounterGroup, MAX_COUNT},
    overrides::{Base, Change, MAX_NAME, Overrides, Resolved, same_condition},
};
use std::{fs, sync::Arc};
use uuid::Uuid;

#[path = "groups/view.rs"]
pub mod view;
pub use view::{GroupFields, MemberRow};

const FOLDER: &str = "Encounter Groups";
const DEFAULT_SIDE: &str = "enemies";
const ANY_ENCOUNTER: &str = "Any encounter";
const MISSING_ENCOUNTER: &str = "Deleted encounter";
/// G122: the one message for a group whose encounter is gone, under Belongs to: the cause and
/// the repair together. Save's check refuses with the same words, in place, so the footer does
/// not say them again.
const GONE_ENCOUNTER: &str = "Its encounter was deleted, so no fight offers this group. Choose another encounter, or Any encounter.";
/// The same fault beside Save, while the line under Belongs to is not on screen: the cause and
/// where to repair it.
const GONE_BESIDE_SAVE: &str = "Its encounter was deleted. See Belongs to.";
/// A member's fault beside Save, while its row is not on screen: fixed words with no name, so
/// the place's width is known; the row keeps the full cause with the name.
const MEMBER_BESIDE_SAVE: &str = "A creature here cannot be saved. See its row.";
/// The Groups list's heading for groups whose encounter is gone; Arrivals says the same words
/// beside such a group's name (FB10).
pub(crate) const ENCOUNTER_DELETED: &str = "Encounter deleted";

fn one() -> u64 {
    1
}

/// A member's identity in the open draft. Its row, its fields and its open changes form key by
/// it, so a move, a removal or an Undo never gives one member's form, cursor or typed words to
/// another (K3: a key is an identity, not a place). It is not saved: a member read again gets a
/// new one, and two members compare by their saved words alone.
#[derive(Clone, Copy, Debug)]
struct MemberId(Uuid);

impl Default for MemberId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl PartialEq for MemberId {
    /// Not part of what a member is: a draft and its saved form compare by their words.
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct MemberDraft {
    #[serde(skip)]
    id: MemberId,
    creature: EntryRef,
    #[serde(default = "one")]
    count: u64,
    #[serde(default, skip_serializing_if = "Mapping::is_empty")]
    overrides: Mapping,
    #[serde(flatten)]
    extra: Mapping,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct GroupSetup {
    side: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    encounter: Option<EntryRef>,
    members: Vec<MemberDraft>,
    #[serde(flatten)]
    extra: Mapping,
}

/// Why Save group waits on an empty group.
const EMPTY_REASON: &str = "Add a creature first.";

#[derive(Clone)]
pub(super) struct GroupDraft {
    recovery: super::disk::Recovery,
    /// The group's own id (`splat_id`), the same before and after its first save and after a
    /// recovery. Its fields take it as their key; the path changes at the first save.
    id: Uuid,
    path: String,
    /// Whether this group was ever saved (R37). Its own fact, owned by the draft: `New` from
    /// [`GroupDraft::new`], `Saved` from [`GroupDraft::open`] and from the first successful write
    /// (`saved_text`). A failed write, a lost file, an accepted recovery or an outside edit leave
    /// it as it is; Discard and Undo carry it with the draft; it ends with the draft. It is not
    /// `baseline`, which is only the text to compare with the disk.
    origin: design::save_row::DraftOrigin,
    baseline: Option<String>,
    source: String,
    /// The line of each readable member in `source`, read when `source` or `flags` change (never
    /// per frame). A fault names its line only while the members match the saved ones (FJ6).
    member_lines: Vec<usize>,
    name: String,
    setup: GroupSetup,
    /// The group's notes, a document of its own (layout PR 9b).
    notes: TextDraft,
    saved_name: String,
    saved_setup: GroupSetup,
    /// The saved notes, with their revision (layout PR 9b-C): `dirty` compares two revisions on
    /// a quiet frame.
    saved_notes: Revised<String>,
    /// The last answer of the notes' compare with the saved ones.
    same_notes: KeptCompare,
    flags: Vec<splat::content::layout::Flag>,
    /// A member result just made, with what its Undo gives back. The draft keeps none: the page
    /// moves it into its store (`Encounters::record_member_results`), which issues its order and
    /// holds it with its ticket, so an Undo applies only its own data.
    member_new: Option<MemberUndo>,
    /// This draft's content, as the owner of its member result: a fresh one at each open, kept
    /// by a clone, so a discarded draft and the clean draft that replaces it (same note, same
    /// `id`) never share a result.
    results_key: super::results_key::ResultsKey,
    /// Changes at each edit of the draft (`touch`), and only then; a new draft has its own. Save's
    /// cached check is keyed by it (FN5, review of PR 299).
    revision: u64,
    /// Whether the members are the saved ones, worked out once after each change (FJ6); `None`
    /// until then. Editing input (this frame or the one before, as a click's change lands late)
    /// and the events that change the members or the saved ones clear it.
    members_saved: Option<bool>,
}

/// A refusal lives in its own module, so that even this file reads its words and place only
/// through accessors, and cannot change them after its kind is set.
///
/// F4: a group's check never meets unreadable data, so a refusal is only a refused change or a
/// missing part. `GroupDraft::open` builds a draft only from a note that reads and parses (else
/// it gives an error and no editor opens), and the check reads the draft's typed fields, not the
/// file; `an_empty_group_or_a_missing_creature_cannot_be_saved` tests the first part. Open debt,
/// "post-open read failure of a group file": when the file is replaced by bytes that do not read
/// AFTER it was opened, the read fails before `disk::Recovery` records a change, so the disk view
/// does not take over. Save's write is then refused safely, the draft is kept, and the message is
/// a generic failed save.
mod refusal {
    use super::{FaultKind, RefusalPlace, Tone, Uuid, design};

    /// Why Save would refuse a group draft (FN5, AI2). Its fields are private and read-only: a
    /// refusal is made only by [`Refusal::refused`] or [`Refusal::incomplete`], which also make its
    /// owned typed diagnosis, so the words, the place and the kind cannot drift apart.
    #[derive(Clone, Debug, PartialEq)]
    pub(in crate::encounters) struct Refusal {
        text: String,
        /// The one place that says this fault. Save, by its button or by the keyboard, keeps it
        /// there: no other place repeats it.
        place: RefusalPlace,
        /// The save row's typed diagnosis (`design::save_row::Diagnosis`), owned, so the cached
        /// check carries it whole to `DraftState`. Boxed, so a `Result` that holds a refusal
        /// stays small.
        diagnosis: Box<design::save_row::Diagnosis<'static>>,
        /// The member's row that says this fault, by the name it shows (N2): the next step
        /// that a pane with the row out of view names.
        row: Option<Box<str>>,
    }

    impl Refusal {
        /// The words the footer shows for this refusal: none when another place says it.
        pub(in crate::encounters) fn footer(&self) -> Option<&str> {
            (self.place == RefusalPlace::Footer).then_some(self.text.as_str())
        }

        /// A change that cannot stand (a member's change, a missing creature, a taken name).
        pub(in crate::encounters) fn refused(text: impl Into<String>, place: RefusalPlace) -> Self {
            let text = text.into();
            Self {
                diagnosis: Box::new(design::save_row::Diagnosis::refused(text.clone())),
                text,
                place,
                row: None,
            }
        }

        /// A change that names a thing that is gone (a deleted encounter): a warning (R41).
        pub(in crate::encounters) fn missing(text: impl Into<String>, place: RefusalPlace) -> Self {
            let text = text.into();
            Self {
                diagnosis: Box::new(design::save_row::Diagnosis::missing(text.clone())),
                text,
                place,
                row: None,
            }
        }

        /// A member's fault of `kind`, which the member's row `row` shows. The kind that gives
        /// the row its tone gives the refusal its diagnosis, so the save row's word, the words
        /// beside Save and the row say the fault in one tone (R41).
        pub(in crate::encounters) fn in_row(
            text: impl Into<String>,
            kind: FaultKind,
            member: Uuid,
            row: &str,
        ) -> Self {
            let place = RefusalPlace::Member(member);
            let refusal = match kind.tone() {
                Tone::Error => Self::refused(text, place),
                _ => Self::missing(text, place),
            };
            Self {
                row: Some(row.into()),
                ..refusal
            }
        }

        /// The name of the member's row that says this fault, if it is a row's.
        pub(in crate::encounters) fn row(&self) -> Option<&str> {
            self.row.as_deref()
        }

        /// A required part is missing (no name, no side, no creature). With the draft's origin it
        /// gives the save row's word (R37).
        pub(in crate::encounters) fn incomplete(
            text: impl Into<String>,
            place: RefusalPlace,
        ) -> Self {
            let text = text.into();
            Self {
                diagnosis: Box::new(design::save_row::Diagnosis::incomplete(text.clone())),
                text,
                place,
                row: None,
            }
        }

        pub(in crate::encounters) fn text(&self) -> &str {
            &self.text
        }

        pub(in crate::encounters) fn place(&self) -> RefusalPlace {
            self.place
        }

        /// The save row's typed diagnosis of this refusal.
        pub(in crate::encounters) fn diagnosis(&self) -> &design::save_row::Diagnosis<'static> {
            &self.diagnosis
        }

        /// The same fault in other words (Save's words for it in the encounter editor): the
        /// kind, and so the tone, stays.
        pub(in crate::encounters) fn named(
            &self,
            words: String,
        ) -> design::save_row::Diagnosis<'static> {
            use design::save_row::{Diagnosis, InvalidCause};
            match self.diagnosis.cause() {
                Some(InvalidCause::Incomplete) => Diagnosis::incomplete(words),
                Some(InvalidCause::Missing) => Diagnosis::missing(words),
                _ => Diagnosis::refused(words),
            }
        }

        /// The one tone of this fault (R41), from its diagnosis: the save row's word takes it,
        /// and so does every message that says the fault (under Belongs to, beside Save, the
        /// footer).
        pub(in crate::encounters) fn tone(&self) -> Tone {
            // A refusal is made only with a fault, so its diagnosis always has a tone.
            self.diagnosis.tone().unwrap_or(Tone::Error)
        }
    }
}
pub(super) use refusal::Refusal;

/// Why a member's creature does not resolve, with the words its fault says: a file that is there
/// and cannot be read is unreadable, an error; else the creature is not in this campaign's
/// packages, a warning (R41). The roster's classification (`super::creature_unreadable`), read by
/// Save's check, the editor's member row, the list and Arrivals.
fn unresolved_member(catalogue: &Catalogue, reference: &EntryRef) -> (FaultKind, &'static str) {
    if super::creature_unreadable(catalogue, reference) {
        (FaultKind::Unreadable, "its file cannot be read")
    } else {
        (FaultKind::NotInPackages, "not in this campaign’s packages")
    }
}

/// The `EntryRef` of a member's stored reference, for the catalogue's questions about it.
fn member_ref(reference: &splat::content::Reference) -> EntryRef {
    match reference {
        splat::content::Reference::Id(id) => EntryRef::id(*id),
        splat::content::Reference::Key(key) => EntryRef {
            id: None,
            key: Some(key.clone()),
            link: None,
        },
    }
}

#[derive(Clone, Copy)]
enum MemberIssue<'a> {
    Reference(FaultKind, &'static str),
    InvalidChange(&'a str),
    CannotJoin(&'static str),
}

/// Shared facts for the editor, Save and Arrivals. The callers decide where to put the
/// fault and whether a fault on one member blocks the whole group.
struct MemberAssessment<'a> {
    entry: Option<&'a Entry>,
    creature: Option<&'a Creature>,
    missing: Option<(FaultKind, &'static str)>,
    changed: Result<Option<Resolved>, String>,
}

impl MemberAssessment<'_> {
    fn eligible(&self) -> Result<&Entry, MemberIssue<'_>> {
        if let Some((kind, cause)) = self.missing {
            return Err(MemberIssue::Reference(kind, cause));
        }
        let entry = self.entry.expect("a member with a creature has an entry");
        if self.creature.is_some_and(|creature| creature.linked) {
            return Err(MemberIssue::CannotJoin(LINKED_CAUSE));
        }
        let changed = self
            .changed
            .as_ref()
            .map_err(|error| MemberIssue::InvalidChange(error))?;
        if super::max_hp(entry, changed.as_ref()).is_none_or(|hp| hp <= 0) {
            return Err(MemberIssue::CannotJoin(NO_MAX_HP_CAUSE));
        }
        Ok(entry)
    }
}

fn assess_member<'a>(
    catalogue: &'a Catalogue,
    creatures: &'a [Creature],
    reference: &EntryRef,
    side: &str,
    overrides: Result<&Overrides, &str>,
) -> MemberAssessment<'a> {
    let entry = catalogue
        .resolve_ref(reference)
        .ok()
        .and_then(|id| catalogue.entries.get(&id))
        .map(|item| &item.entry);
    let creature =
        entry.and_then(|entry| creatures.iter().find(|creature| creature.id == entry.id));
    let missing = creature
        .is_none()
        .then(|| unresolved_member(catalogue, reference));
    let changed = match (entry, overrides) {
        (Some(entry), Ok(overrides)) if !overrides.is_empty() => {
            overrides.resolve_for(entry, side).map(Some)
        }
        (Some(_), Err(error)) => Err(error.to_owned()),
        _ => Ok(None),
    };
    MemberAssessment {
        entry,
        creature,
        missing,
        changed,
    }
}

/// Save's refusal of a group whose encounter is gone: a missing thing, so a warning (R41). The
/// one value that both the check and the message under Belongs to read.
fn gone_encounter() -> Refusal {
    Refusal::missing(GONE_ENCOUNTER, RefusalPlace::BelongsTo)
}

/// Where the group editor says a refusal of Save.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum RefusalPlace {
    /// The editor's footer: the fault has no place of its own.
    Footer,
    /// Beside Save, on the save row (an empty group's `Add a creature first.`).
    SaveRow,
    /// Under Belongs to (G122: an encounter that is gone).
    BelongsTo,
    /// A member's row: the member's identity (`MemberId`'s own id, since `MemberId` itself
    /// compares equal to every other). Its tone is the refusal's (`Refusal::tone`).
    Member(Uuid),
}

/// What a member result's Undo gives back, held in the page's store with the result's ticket.
#[derive(Clone)]
pub(super) enum MemberUndo {
    /// `Use` on a member this campaign could not find: its creature before and after.
    Replaced(Replaced),
    /// A removed member, at its place.
    Removed {
        index: usize,
        member: MemberDraft,
        name: String,
    },
}

impl MemberUndo {
    /// The result's words.
    pub(super) fn words(&self) -> String {
        match self {
            Self::Replaced(result) => result.words.clone(),
            Self::Removed { name, .. } => format!("Removed {name}."),
        }
    }
    /// Its Undo's words.
    pub(super) fn step(&self) -> design::outcome::UndoStep {
        use design::outcome::{UndoStep, Verb};
        match self {
            Self::Replaced(_) => UndoStep::of(Verb::Replace, "member"),
            Self::Removed { .. } => UndoStep::of(Verb::Remove, "member"),
        }
    }
    /// Whether its Undo still applies: a replacement's member (by its identity, which a move
    /// does not change) is in the draft and still holds the chosen creature. A removal always
    /// holds.
    pub(super) fn holds(&self, draft: &GroupDraft) -> bool {
        match self {
            Self::Replaced(result) => draft.replaced_member(result).is_some(),
            Self::Removed { .. } => true,
        }
    }
}

/// A member's creature before and after `Use`, with the result's words. It names its member
/// by identity, not by place: a move between the act and its Undo cannot point it at another
/// member, even one that holds the same creature.
#[derive(Clone)]
pub(super) struct Replaced {
    member: MemberId,
    before: EntryRef,
    after: EntryRef,
    words: String,
}

impl GroupDraft {
    pub(super) fn new() -> Self {
        let id = crate::hermetic::new_uuid();
        let name = "New group".to_owned();
        let setup = GroupSetup {
            extra: Mapping::new(),
            side: DEFAULT_SIDE.into(),
            encounter: None,
            members: Vec::new(),
        };
        Self {
            recovery: Default::default(),
            id,
            path: format!("{FOLDER}/{id}.md"),
            origin: design::save_row::DraftOrigin::New,
            baseline: None,
            source: format!(
                "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: encounter_group\nsplat_name: New group\nsplat_side: enemies\n---\n\n## Members\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n"
            ),
            member_lines: Vec::new(),
            name: name.clone(),
            saved_name: name,
            setup: setup.clone(),
            saved_setup: setup,
            notes: TextDraft::default(),
            saved_notes: Revised::default(),
            same_notes: KeptCompare::default(),
            flags: Vec::new(),
            member_new: None,
            results_key: super::results_key::ResultsKey::new(),
            revision: next_revision(),
            members_saved: None,
        }
    }
    /// A new group that belongs to this encounter. Saving it is what makes it so.
    pub(super) fn for_encounter(encounter: Uuid) -> Self {
        let mut draft = Self::new();
        draft.setup.encounter = Some(EntryRef::id(encounter));
        draft
    }
    pub(super) fn open(root: &std::path::Path, path: &str) -> Result<Self, String> {
        let text = fs::read_to_string(note_path(root, path)?).map_err(|e| e.to_string())?;
        let mut entry = Entry::parse(path, &text)?;
        let (entries, _) = splat::content::read_directory(root)?;
        splat::content::list_formats::resolve(&mut entry, &entries, "");
        if entry.kind != "encounter_group" {
            return Err("Select an encounter group note.".into());
        }
        let setup: GroupSetup = serde_yaml_ng::from_value(
            entry.fields["splat_encounter_group"].clone(),
        )
        .map_err(|e| format!("This group needs repair in Campaign → Campaign entries: {e}"))?;
        let notes = splat::content::list_formats::prose(&entry);
        Ok(Self {
            recovery: Default::default(),
            id: entry.id,
            path: path.into(),
            origin: design::save_row::DraftOrigin::Saved,
            baseline: Some(text.clone()),
            member_lines: splat::content::list_formats::group_member_lines(&text, &entry.flags),
            source: text,
            flags: entry.flags.clone(),
            member_new: None,
            results_key: super::results_key::ResultsKey::new(),
            revision: next_revision(),
            members_saved: None,
            saved_name: entry.name.clone(),
            name: entry.name,
            saved_setup: setup.clone(),
            setup,
            saved_notes: Revised::new(notes.clone()),
            same_notes: KeptCompare::default(),
            notes: TextDraft::new(notes),
        })
    }
    pub(super) fn name(&self) -> String {
        self.name.clone()
    }
    /// The draft's own identity, the owner of its member result in the page's store.
    pub(super) fn id(&self) -> Uuid {
        self.id
    }
    /// The member result made since the page last looked, handed to the page's store.
    pub(super) fn take_member_new(&mut self) -> Option<MemberUndo> {
        self.member_new.take()
    }
    /// The owner of this draft's member result in the page's store.
    pub(super) fn results_key(&self) -> Uuid {
        self.results_key.id()
    }
    /// A watch on that owner for a store entry: the entry ends when no copy of this draft is
    /// left.
    pub(super) fn results_watch(&self) -> super::results_key::KeyWatch {
        self.results_key.watch()
    }
    /// Give this group the owner `earlier` had, and change nothing else: Undo of a move.
    pub(super) fn take_owner_from(&mut self, earlier: &GroupDraft) {
        self.setup.encounter = earlier.setup.encounter.clone();
        self.touch();
    }
    /// The draft changed: Save's cached check runs again.
    pub(super) fn touch(&mut self) {
        self.revision = next_revision();
    }
    /// No member that Save would write: Save refuses the group.
    pub(super) fn is_empty(&self) -> bool {
        self.setup.members.is_empty()
    }
    /// The member a replacement names, while it still holds the chosen creature.
    fn replaced_member(&self, result: &Replaced) -> Option<usize> {
        self.setup
            .members
            .iter()
            .position(|member| member.id.0 == result.member.0 && member.creature == result.after)
    }
    /// Undo one member result, from the data its store holds with its ticket: the creature,
    /// the member or the unread row comes back, and other draft edits stay. A replacement whose
    /// member was removed or changed since (in this pass too) is refused, and nothing changes.
    pub(super) fn undo_member_result(&mut self, undo: &MemberUndo) -> Result<(), ()> {
        if !undo.holds(self) {
            return Err(());
        }
        self.members_saved = None;
        self.touch();
        match undo {
            MemberUndo::Replaced(result) => {
                if let Some(at) = self.replaced_member(result) {
                    self.setup.members[at].creature = result.before.clone();
                }
            }
            MemberUndo::Removed { index, member, .. } => {
                self.setup
                    .members
                    .insert((*index).min(self.setup.members.len()), member.clone());
            }
        }
        Ok(())
    }
    fn reread_lines(&mut self) {
        self.members_saved = None;
        self.member_lines =
            splat::content::list_formats::group_member_lines(&self.source, &self.flags);
    }

    pub(super) fn set_new_path(&mut self, path: String) {
        self.path = path;
        self.touch();
    }
    pub(super) fn path(&self) -> &str {
        &self.path
    }
    /// Its name, when a plan could link it: it is for any encounter.
    pub(super) fn linkable_name(&self) -> Option<&str> {
        self.setup.encounter.is_none().then(|| self.name.trim())
    }
    pub(super) fn belongs_to(&self, encounter: Uuid) -> bool {
        self.setup
            .encounter
            .as_ref()
            .is_some_and(|reference| reference.id == Some(encounter))
    }
    /// Whether this group was ever saved: its stored origin (see the field).
    pub(super) fn origin(&self) -> design::save_row::DraftOrigin {
        self.origin
    }
    /// Whether the notes are the saved ones: compared once per pair of revisions.
    fn notes_saved(&self) -> bool {
        self.notes.same_as(
            self.saved_notes.revision(),
            &self.saved_notes,
            &self.same_notes,
        )
    }

    pub(super) fn dirty(&self) -> bool {
        self.baseline.is_none()
            || self.name != self.saved_name
            || self.setup != self.saved_setup
            || !self.notes_saved()
            || self.baseline.as_deref() != Some(self.source.as_str())
    }
    fn text(&self) -> Result<String, String> {
        let disk_source = self
            .recovery
            .change
            .as_ref()
            .and_then(|change| change.text.as_deref())
            .unwrap_or(&self.source);
        let source = if self.notes_saved() {
            disk_source.to_owned()
        } else {
            splat::content::list_formats::with_prose(disk_source, &self.notes)?
        };
        let text = splat::content::formats::patch_model(
            &source,
            &[
                ("splat_name", Some(self.name.clone().into())),
                (
                    "splat_encounter_group",
                    Some(serde_yaml_ng::to_value(&self.setup).map_err(|e| e.to_string())?),
                ),
            ],
        )?;
        Ok(text)
    }
    fn comparison_text(&self, snapshot: &Snapshot) -> Result<String, String> {
        super::disk::comparison_text(&self.path, self.text()?, snapshot)
    }
    fn write_text(&self, snapshot: &Snapshot) -> Result<String, String> {
        splat::content::list_formats::write_references(
            &self.text()?,
            snapshot
                .catalogue
                .entries
                .values()
                .map(|entry| &entry.entry),
        )
    }

    pub(super) fn add(&mut self, id: Uuid) {
        self.touch();
        self.members_saved = None;
        // The same creature again is one more of it, unless that member has its own changes.
        if let Some(member) = self
            .setup
            .members
            .last_mut()
            .filter(|member| member.creature.id == Some(id) && member.overrides.is_empty())
            .filter(|member| member.count < MAX_COUNT)
        {
            member.count += 1;
            return;
        }
        self.setup.members.push(MemberDraft {
            id: MemberId::default(),
            extra: Mapping::new(),
            creature: EntryRef::id(id),
            count: 1,
            overrides: Mapping::new(),
        });
    }
    pub(super) fn check_delete(&self, campaign: &Campaign) -> Result<(), String> {
        let Some(baseline) = &self.baseline else {
            return Ok(());
        };
        let path = note_path(&campaign.path, &self.path)?;
        if fs::read_to_string(&path).map_err(|e| e.to_string())? != *baseline {
            return Err(format!(
                "{} changed on disk. Reload before deleting it.",
                self.name
            ));
        }
        Ok(())
    }
    pub(super) fn has_file(&self) -> bool {
        self.baseline.is_some()
    }
    pub(super) fn add_named(&mut self, id: Uuid, name: String) {
        self.touch();
        self.members_saved = None;
        self.setup.members.push(MemberDraft {
            id: MemberId::default(),
            extra: Mapping::new(),
            creature: EntryRef::id(id),
            count: 1,
            overrides: Mapping::from_iter([("name".into(), name.into())]),
        });
    }
    /// An encounter being saved with this group counts as there, though it is not on disk yet.
    pub(super) fn validate(&self, snapshot: &Snapshot, saving: Option<Uuid>) -> Result<(), String> {
        self.check(snapshot, saving)
            .map_err(|refusal| refusal.text().to_owned())
    }
    /// Save's check, without writing. A refusal says whether the editor already shows the fault
    /// in its place (a member row's missing creature or change that cannot stand, or the empty
    /// group's reason beside Save), so the footer does not repeat it.
    ///
    /// Lead ruling (s30 B11, R41): of all the faults, Save names the most severe by the diagnosis
    /// rank (`Diagnosis::outranks`: a change that cannot stand before a thing that is gone, before
    /// a missing part), then the first in order among equals: the group's own fields, then the
    /// members in their order. So the save row's word, the words beside Save, the footer's next
    /// step and the Session plan's editor all read the one chosen fault.
    pub(super) fn check(&self, snapshot: &Snapshot, saving: Option<Uuid>) -> Result<(), Refusal> {
        #[cfg(any(test, feature = "test-support"))]
        GROUP_CHECKS.with(|counts| counts.set((counts.get().0, counts.get().1 + 1)));
        // A refusal in the error tone is a change that cannot stand: nothing a group's check
        // finds outranks it (F4: it never meets unreadable data), so the first one ends the check.
        let ends = |refusal: &Refusal| refusal.tone() == Tone::Error;
        let mut chosen = match self.check_group(snapshot, saving) {
            Err(refusal) if ends(&refusal) => return Err(refusal),
            Err(refusal) => Some(refusal),
            Ok(missing) => missing,
        };
        for member in &self.setup.members {
            let Err(refusal) = self.check_member(member, snapshot) else {
                continue;
            };
            if ends(&refusal) {
                return Err(refusal);
            }
            if chosen
                .as_ref()
                .is_none_or(|kept| refusal.diagnosis().outranks(kept.diagnosis()))
            {
                chosen = Some(refusal);
            }
        }
        chosen.map_or(Ok(()), Err)
    }

    /// One member's fault, if it has one: the first that its checks find.
    fn check_member(&self, member: &MemberDraft, snapshot: &Snapshot) -> Result<(), Refusal> {
        // The row says a missing creature as a warning, a change that cannot stand as an
        // error (`FaultKind::InvalidChange`).
        // FA1: Save's check names a member as every view does. A draft's lines may have
        // moved since the note was read, so it names no line.
        let changed = member.overrides.get("name").and_then(Value::as_str);
        let overrides = Overrides::parse(&member.overrides);
        let assessed = assess_member(
            &snapshot.catalogue,
            &snapshot.creatures,
            &member.creature,
            &self.setup.side,
            overrides.as_ref().map_err(String::as_str),
        );
        let base = assessed.entry.map(|entry| entry.name.as_str());
        let shown = match (changed, base) {
            (Some(changed), _) if !changed.trim().is_empty() => changed.trim().to_owned(),
            (_, Some(base)) => base.to_owned(),
            _ => missing_member_name(None, member.creature.link.as_deref()),
        };
        let in_place =
            |text: String, kind: FaultKind| Refusal::in_row(text, kind, member.id.0, &shown);
        let fault = |cause: &str| member_fault(&shown, base, None, cause);
        let not_in_place = |cause: &str| Refusal::refused(fault(cause), RefusalPlace::Footer);
        if !(1..=MAX_COUNT).contains(&member.count) {
            return Err(not_in_place(&format!(
                "count must be from 1 to {MAX_COUNT}"
            )));
        }
        assessed
            .eligible()
            .map(|_| ())
            .map_err(|issue| match issue {
                MemberIssue::Reference(_, _) if base.is_some() => {
                    not_in_place("this creature is unavailable in campaign content")
                }
                MemberIssue::Reference(kind, cause) => in_place(fault(cause), kind),
                MemberIssue::InvalidChange(cause) => {
                    in_place(fault(cause), FaultKind::InvalidChange)
                }
                MemberIssue::CannotJoin(cause) => not_in_place(cause),
            })
    }
    /// The group's own fields, before its members.
    /// `Err` for a change that cannot stand; `Ok(Some(..))` for the first missing part, which the
    /// caller says only when no member's fault outranks it.
    fn check_group(
        &self,
        snapshot: &Snapshot,
        saving: Option<Uuid>,
    ) -> Result<Option<Refusal>, Refusal> {
        let refuse = |text: String| Err(Refusal::refused(text, RefusalPlace::Footer));
        // E2: a missing part does not end the check. A change that cannot stand outranks it,
        // so the first missing part is kept and said only when nothing is refused.
        let mut missing = None;
        let name = self.name.trim();
        if name.is_empty() {
            missing = Some(Refusal::incomplete(
                "Enter a group name.",
                RefusalPlace::Footer,
            ));
        }
        // A plan links a group for any encounter by its name, so that name must mean one thing.
        // A group that belongs to an encounter is not linked, and each can have "Reinforcements".
        // A blank name has nothing to compare: it is a missing part, above.
        let linkable = self.setup.encounter.is_none() && !name.is_empty();
        if linkable
            && snapshot.groups.iter().any(|group| {
                group.encounter.is_none()
                    && group.path != self.path
                    && group.name.eq_ignore_ascii_case(name)
            })
        {
            return refuse(format!(
                "Another group is named {name}. Choose another name."
            ));
        }
        if linkable
            && snapshot
                .templates
                .iter()
                .any(|template| template.name.eq_ignore_ascii_case(name))
        {
            return refuse(format!(
                "An encounter is named {name}. Choose another name."
            ));
        }
        if self.setup.side.trim().is_empty() {
            missing.get_or_insert(Refusal::incomplete(
                "Enter a default side for this group.",
                RefusalPlace::Footer,
            ));
        }
        if self.setup.members.is_empty() {
            // Its reason (`Add a creature first.`) is beside Save.
            missing.get_or_insert(Refusal::incomplete(
                "This group is empty. Add at least one creature.",
                RefusalPlace::SaveRow,
            ));
        }
        if self
            .setup
            .encounter
            .as_ref()
            .and_then(|reference| reference.id)
            .is_some_and(|id| {
                Some(id) != saving && !snapshot.encounter_choices().any(|(known, _)| known == id)
            })
        {
            // Its words are under Belongs to (`belongs_to`).
            return Err(gone_encounter());
        }
        Ok(missing)
    }
    /// The note's new text, if this group can be saved as it is.
    pub(super) fn checked(
        &self,
        snapshot: &Snapshot,
        campaign: &Campaign,
        saving: Option<Uuid>,
    ) -> Result<String, String> {
        self.validate(snapshot, saving)?;
        let text = self.write_text(snapshot)?;
        let entry = Entry::parse(&self.path, &text)?;
        if snapshot.catalogue.entries.values().any(|e| {
            e.entry.id == entry.id
                && (e.owner != campaign.id
                    || (self.baseline.is_none() && e.entry.path != self.path))
        }) {
            return Err("Another note uses this group’s identity.".into());
        }
        Ok(text)
    }
    fn save(&mut self, campaign: &Campaign, modules: &[Module]) -> Result<(), String> {
        self.check_disk(campaign)?;
        let snapshot = Snapshot::load(campaign, modules)?;
        let text = self.checked(&snapshot, campaign, None)?;
        // Create the manifest first so a new campaign keeps its package selections.
        let document = CampaignDocument::load(campaign)?;
        if document.baseline.is_none() {
            crate::file_batch::apply(&[
                crate::file_batch::Change::text(
                    note_path(&campaign.path, &document.path)?,
                    None,
                    Some(&document.text),
                ),
                self.change(campaign, Some(&text))?,
            ])?;
            self.saved_text(&text);
            return Ok(());
        }
        self.write(campaign, &text)
    }
    pub(super) fn check_disk(&mut self, campaign: &Campaign) -> Result<(), String> {
        self.touch();
        if self.baseline.is_none() {
            self.path = super::named_note_path(&campaign.path, FOLDER, &self.name)?;
        }
        self.recovery.check(
            &campaign.path,
            &mut self.path,
            self.baseline.as_deref(),
            ("group", &self.name),
        )
    }
    pub(super) fn file_gone(&self) -> bool {
        self.recovery.file_gone()
    }
    pub(super) fn has_disk_change(&self) -> bool {
        self.recovery.change.is_some()
    }
    /// The group's disk conflict as data.
    pub(super) fn disk_view(&self, snapshot: &Snapshot) -> Option<super::disk::DiskView> {
        self.recovery
            .view(&self.comparison_text(snapshot).ok()?, ("group", &self.name))
    }
    /// The DM's choice in the group's disk conflict: `keep` takes the file's place and text as
    /// the baseline, else the group's file is read again.
    pub(super) fn disk_choice(&mut self, root: &std::path::Path, keep: bool) -> Result<(), String> {
        if keep {
            if let Some(change) = self.recovery.accept() {
                self.path = change.path;
                self.baseline = change.text.clone();
                if let Some(text) = change.text {
                    self.source = text;
                    self.reread_lines();
                }
                // N1: the comparison text decides `dirty`, which decides what Save checks.
                self.touch();
            }
        } else if !self.recovery.file_gone() {
            *self = Self::open(root, &self.path)?;
        }
        // A group whose file is gone has nothing to reopen: the encounter removes it and
        // keeps it for Undo remove.
        Ok(())
    }
    pub(super) fn write(&mut self, campaign: &Campaign, text: &str) -> Result<(), String> {
        let path = note_path(&campaign.path, &self.path)?;
        fs::create_dir_all(path.parent().ok_or("Missing group folder")?)
            .map_err(|e| e.to_string())?;
        write_note(&campaign.path, &self.path, self.baseline.as_deref(), text)?;
        self.saved_text(text);
        Ok(())
    }
    pub(super) fn change(
        &self,
        campaign: &Campaign,
        text: Option<&str>,
    ) -> Result<crate::file_batch::Change, String> {
        Ok(crate::file_batch::Change::text(
            note_path(&campaign.path, &self.path)?,
            self.baseline.as_deref(),
            text,
        ))
    }
    pub(super) fn saved_text(&mut self, text: &str) {
        self.touch();
        self.recovery = Default::default();
        self.source = text.to_owned();
        if let Ok(entry) = Entry::parse(&self.path, text) {
            self.flags = entry.flags;
        }
        self.reread_lines();
        self.baseline = Some(text.to_owned());
        self.origin = design::save_row::DraftOrigin::Saved;
        self.saved_name = self.name.clone();
        self.saved_setup = self.setup.clone();
        let trimmed = self.notes.trim().to_owned();
        if !self.notes.same_words(&trimmed) {
            let _ = self.notes.replace(trimmed);
        }
        self.saved_notes = Revised::new(self.notes.as_str().to_owned());
    }
}

/// A creature's name in an editor: the one it will have. Its changes say which entry it is.
pub(super) fn shown_name(block: &Mapping, name: String) -> String {
    Overrides::parse(block)
        .ok()
        .and_then(|overrides| overrides.name)
        .map_or(name, |changed| changed.trim().to_owned())
}

/// A group's own reason when it has no member fault to show.
const NONE_CAN_JOIN: &str = "No members can join.";
/// Why a member with a D&D Beyond sheet cannot join from a group.
const LINKED_CAUSE: &str = "it has a D&D Beyond sheet, so it joins from the roster";
/// Why a member with no maximum HP cannot join. Its Edit button sets one.
pub const NO_MAX_HP_CAUSE: &str = "it has no maximum HP. Set it with this member’s Edit button";

/// One member's fault, in the one form every view uses (FA1, FC5): `<shown name> (as <base>) ·
/// line N: <cause>.` The Encounters list, the group editor, the session plan, Save's check and
/// Arrivals all take their words from here. `(as <base>)` is left out when the member keeps its
/// base name or the base is not known; `line N` when the note gives the member no line.
pub fn member_fault(shown: &str, base: Option<&str>, line: Option<usize>, cause: &str) -> String {
    let shown = shown.trim();
    let mut text = shown.to_owned();
    if let Some(base) = base
        .map(str::trim)
        .filter(|base| !base.is_empty() && *base != shown)
    {
        text.push_str(&format!(" (as {base})"));
    }
    if let Some(line) = line {
        // `line 15` wraps whole.
        text.push_str(&format!(" · line\u{a0}{line}"));
    }
    // R36: the cause starts with a capital (`layout::cause_words`) and ends with one full stop.
    format!(
        "{text}: {}.",
        splat::content::layout::cause_words(cause.trim().trim_end_matches('.'))
    )
}

fn missing_member_name(changed: Option<&str>, link: Option<&str>) -> String {
    let link = link.map(crate::sessions::plain_links).filter(|name| {
        Uuid::parse_str(name.trim()).is_err() && !splat::content::resource_key(name.trim())
    });
    changed
        .or(link.as_deref())
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or("Unknown creature")
        .to_owned()
}

/// The campaign's groups, each with its members resolved and any reason it cannot be added.
pub(super) fn load(
    catalogue: &Catalogue,
    creatures: &[Creature],
    campaign: Uuid,
) -> Vec<Arc<RunGroup>> {
    let mut groups: Vec<_> = catalogue
        .entries
        .values()
        .filter(|e| e.private && e.owner == campaign && e.entry.kind == "encounter_group")
        .filter_map(|e| {
            let source = catalogue.text_of(e.entry.id);
            resolve_group(&e.entry, source.as_deref(), catalogue, creatures, campaign)
        })
        .collect();
    groups.sort_by_key(|group| group.name.to_lowercase());
    groups
}

/// `source` is the note as it was read, for each member's line; without it, faults name no line.
fn resolve_group(
    entry: &Entry,
    source: Option<&str>,
    catalogue: &Catalogue,
    creatures: &[Creature],
    campaign: Uuid,
) -> Option<Arc<RunGroup>> {
    let unread = splat::content::list_formats::unread_group_members(
        &format!("{}{}", "\n".repeat(entry.body_line - 1), entry.body),
        &entry.flags,
    );
    let group = match EncounterGroup::of(entry) {
        Ok(group) => group,
        Err(error)
            if !unread.is_empty() && error == "This group is empty. Add at least one creature." =>
        {
            // Keep an all-unread table visible without relaxing save validation.
            let setup: GroupSetup =
                serde_yaml_ng::from_value(entry.fields.get("splat_encounter_group")?.clone())
                    .ok()?;
            if !setup.members.is_empty() || !setup.extra.is_empty() {
                return None;
            }
            EncounterGroup {
                side: setup.side.trim().to_owned(),
                encounter: setup.encounter.and_then(|reference| reference.id),
                members: Vec::new(),
            }
        }
        Err(_) => return None,
    };
    let mut members = Vec::new();
    let mut warnings = Vec::new();
    let mut unavailable = 0_u64;
    let mut unavailable_members = Vec::new();
    let (mut xp, mut xp_unknown) = (0_u64, Vec::new());
    let lines = source.map_or_else(Vec::new, |text| {
        splat::content::list_formats::group_member_lines(text, &entry.flags)
    });
    for (index, member) in group.members.iter().enumerate() {
        let line = lines.get(index).copied();
        let assessed = assess_member(
            catalogue,
            creatures,
            &member_ref(&member.creature),
            &group.side,
            Ok(&member.overrides),
        );
        match assessed.eligible() {
            Ok(creature) => {
                match creature_xp(creature) {
                    Some(each) => xp = xp.saturating_add(each.saturating_mul(member.count)),
                    None => xp_unknown.push(creature.name.clone()),
                }
                members.push(GroupMember {
                    entry: Arc::new(catalogue.run_entry(creature.id)),
                    count: member.count,
                    overrides: member.overrides.clone(),
                });
            }
            Err(issue) => {
                let (kind, cause) = match issue {
                    MemberIssue::Reference(kind, cause) => (kind, cause.to_owned()),
                    MemberIssue::InvalidChange(error) => {
                        (FaultKind::InvalidChange, override_error(error))
                    }
                    MemberIssue::CannotJoin(cause) => (FaultKind::CannotJoin, cause.to_owned()),
                };
                let changed = member.overrides.name.as_deref();
                let (shown, base, name) = if matches!(issue, MemberIssue::Reference(..)) {
                    let link = entry.fields["splat_encounter_group"]["members"]
                        .get(index)
                        .and_then(|member| member["creature"]["link"].as_str());
                    let shown = missing_member_name(changed, link);
                    let base = link.map(|link| missing_member_name(None, Some(link)));
                    (shown.clone(), base, shown)
                } else {
                    let base = &assessed.entry.expect("the member has a creature").name;
                    let shown = changed.unwrap_or(base);
                    let name = if shown == base {
                        shown.to_owned()
                    } else {
                        format!("{shown} (as {base})")
                    };
                    (shown.to_owned(), Some(base.clone()), name)
                };
                warnings.push(MemberFault::new(
                    kind,
                    member_fault(&shown, base.as_deref(), line, &cause),
                ));
                unavailable_members.push((index, member.count, name));
                unavailable = unavailable.saturating_add(member.count);
            }
        }
    }
    for member in unread {
        let name = member
            .creature
            .clone()
            .and_then(|value| serde_yaml_ng::from_value::<EntryRef>(value).ok())
            .and_then(|reference| catalogue.resolve_ref(&reference).ok())
            .and_then(|id| catalogue.entries.get(&id))
            .map_or_else(
                || missing_member_name(None, Some(&member.label)),
                |entry| entry.entry.name.clone(),
            );
        let shown = if member.name.is_empty() {
            &name
        } else {
            &member.name
        };
        warnings.push(MemberFault::new(
            // R41 (lead ruling, s30 B11): a member row that cannot be read is unreadable data,
            // an error in the list, Arrivals and the editor alike.
            FaultKind::Unreadable,
            // The row, its line and the cause, as a roster row says them; the help stays in the
            // editor. A flag without its parts (read back from a saved run) says its own
            // sentence, which already starts with its line.
            member.flag.cause().map_or_else(
                || member_fault(shown, Some(&name), None, &member.flag.message),
                |cause| member_fault(shown, Some(&name), Some(member.flag.line), cause),
            ),
        ));
        let count = member.count.unwrap_or(1);
        unavailable = unavailable.saturating_add(count);
        unavailable_members.push((member.before, count, format!("{shown} (not read)")));
    }
    // FK5: the member faults say why none can join, and the count line says `0 of N can join`,
    // so the group adds its own reason only when no member fault shows.
    let problem = members.is_empty().then(|| {
        let mut problems: Vec<String> = warnings.iter().map(|fault| fault.text.clone()).collect();
        if warnings.is_empty() {
            problems.push(NONE_CAN_JOIN.into());
        }
        problems.join("\n")
    });
    Some(Arc::new(RunGroup {
        id: entry.id,
        path: entry.path.clone(),
        name: entry.name.clone(),
        side: group.side,
        encounter: group.encounter,
        encounter_name: group
            .encounter
            .and_then(|id| catalogue.entries.get(&id))
            .filter(|owner| {
                owner.private && owner.owner == campaign && owner.entry.kind == "encounter"
            })
            .map(|owner| owner.entry.name.clone()),
        members,
        notes: splat::content::list_formats::prose(entry),
        xp,
        xp_unknown,
        unavailable,
        unavailable_members,
        warnings,
        problem,
    }))
}

/// A group row's second line: its summary, after a warning when no fight offers it.
pub(super) fn row_detail(group: &RunGroup) -> String {
    format!(
        "{}{}",
        if group.encounter.is_some() && group.encounter_name.is_none() {
            "Offered in no fight · "
        } else {
            ""
        },
        group.summary()
    )
}

/// A removed note stays in memory until the DM leaves this campaign or removes another. A saved
/// group and an encounter are removed alike.
pub(super) struct RemovedNote {
    path: std::path::PathBuf,
    text: String,
    pub(super) name: String,
}
impl RemovedNote {
    /// Remove the note at `path`, when it is of `kind`: `encounter_group` or `encounter`.
    pub(super) fn remove(campaign: &Campaign, path: &str, kind: &str) -> Result<Self, String> {
        let path = note_path(&campaign.path, path)?;
        let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        let entry = Entry::parse(path.to_str().ok_or("Invalid note path")?, &text)?;
        if entry.kind != kind {
            return Err(if kind == "encounter" {
                "This note is not an encounter.".to_owned()
            } else {
                "This note is not a group.".to_owned()
            });
        }
        fs::remove_file(&path).map_err(|error| error.to_string())?;
        Ok(Self {
            path,
            text,
            name: entry.name,
        })
    }
    /// Write the note back, only where no file has its path now.
    pub(super) fn restore(&self) -> Result<(), String> {
        use std::io::Write;
        let mut copy =
            tempfile::NamedTempFile::new_in(self.path.parent().ok_or("Invalid note folder")?)
                .map_err(|error| error.to_string())?;
        copy.write_all(self.text.as_bytes())
            .and_then(|_| copy.as_file().sync_all())
            .map_err(|error| error.to_string())?;
        copy.persist_noclobber(&self.path)
            .map_err(|error| error.to_string())?;
        Ok(())
    }
}

impl Encounters {
    /// The group editor's disk conflict, while it has one.
    pub fn group_disk(&self) -> Option<super::disk::DiskView> {
        let snapshot = self.snapshot.as_ref()?;
        let draft = self.group_draft.as_ref()?;
        draft.recovery.view(
            &draft.comparison_text(snapshot).unwrap_or_default(),
            ("group", &draft.name),
        )
    }

    /// The DM's choice in the group editor's disk conflict: `keep` overwrites with the draft,
    /// else the draft goes (with Undo) and the file's text comes back.
    pub fn group_disk_choice(
        &mut self,
        ctx: &egui::Context,
        campaign: &Campaign,
        modules: &[Module],
        keep: bool,
    ) {
        let Some(draft) = self.group_draft.as_mut() else {
            return;
        };
        if keep {
            if let Some(change) = draft.recovery.accept() {
                draft.path = change.path;
                draft.baseline = change.text.clone();
                if let Some(text) = change.text {
                    draft.source = text;
                    draft.reread_lines();
                }
                draft.touch();
                self.save_group_draft(ctx, campaign, modules);
            }
        } else {
            let path = draft.path.clone();
            self.discarded_group = self.group_draft.take();
            self.stamp_kept(super::KeptResult::GroupDiscarded);
            self.group_draft = GroupDraft::open(&campaign.path, &path).ok();
            self.error = None;
        }
    }
}

/// The line under a creature's name in an editor, with the values this block gives it.
fn shown_changes(
    block: &Mapping,
    entry: &Entry,
    side: &str,
) -> Option<splat::content::overrides::Resolved> {
    Overrides::parse(block)
        .ok()
        .filter(|overrides| !overrides.is_empty())
        .and_then(|overrides| overrides.resolve_for(entry, side).ok())
}

pub(super) fn shown_detail(
    block: &Mapping,
    entry: &Entry,
    side: &str,
    creature: &Creature,
) -> String {
    shown_changes(block, entry, side).map_or_else(
        || creature.detail.clone(),
        |resolved| super::creature_detail(entry, creature.party, Some(&resolved)),
    )
}

/// The member has no hit points after its changes, so `shown_detail` shows the HP fault.
pub(super) fn shown_hp_missing(
    block: &Mapping,
    entry: &Entry,
    side: &str,
    creature: &Creature,
) -> bool {
    shown_changes(block, entry, side).map_or(creature.hp_missing, |resolved| {
        super::max_hp(entry, Some(&resolved)).is_none()
    })
}

/// One presentation for validation errors in the editor and Arrivals.
pub(super) fn override_error(error: &str) -> String {
    let mut out = String::new();
    let mut number = String::new();
    for c in error.chars().chain(std::iter::once('\0')) {
        if c.is_ascii_digit() || (c == '-' && number.is_empty()) {
            number.push(c);
        } else {
            if !number.is_empty() {
                out.push_str(&number.parse::<i64>().map_or_else(
                    |_| number.clone(),
                    |value| {
                        let digits = if value == 0 {
                            "0".to_owned()
                        } else {
                            design::count(value.unsigned_abs() as usize, "", "")
                        };
                        if value < 0 {
                            format!("−{digits}")
                        } else {
                            digits
                        }
                    },
                ));
                number.clear();
            }
            if c != '\0' {
                out.push(c);
            }
        }
    }
    out
}

/// `missing` are the loot's table keys that cannot roll, which the row names so that a typo
/// shows before play.
pub(super) fn change_words(
    overrides: &Overrides,
    entry: &Entry,
    side: &str,
    missing: &[String],
) -> String {
    let count = overrides
        .rows(&Base::of(entry, side))
        .iter()
        .filter(|row| row.by_hand)
        .count();
    let mut words = Vec::new();
    if overrides.changes_creature() {
        words.push("Changed".to_owned());
    }
    if count > 0 {
        words.push(format!("{count} to add by hand"));
    }
    if overrides.loot.is_some() {
        words.push(match missing {
            [] => "Loot".to_owned(),
            [key] => format!("Loot · no table {key}"),
            keys => format!("Loot · no tables {}", keys.join(", ")),
        });
    }
    if words.is_empty() {
        "Changed".to_owned()
    } else {
        words.join(" · ")
    }
}

/// How a change row's value changes, as its drop-down offers it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChangeKind {
    Same,
    Set,
    Add,
}

impl ChangeKind {
    fn of(value: Option<Change>) -> Self {
        match value {
            None => Self::Same,
            Some(Change::Set(_)) => Self::Set,
            Some(Change::Add(_)) => Self::Add,
        }
    }
}

/// One change on its own, for its row's preview and its own fault.
fn single_change(label: &str, value: Option<Change>) -> Overrides {
    let mut preview = Overrides::default();
    match label {
        "Maximum HP" => preview.max_hp = value,
        "AC" => preview.armor_class = value,
        "Initiative bonus" => preview.initiative_bonus = value,
        "Speed" => preview.speed = value,
        "Attack bonus" => {
            preview.attack_bonus = value.and_then(|c| {
                if let Change::Add(n) = c {
                    Some(n)
                } else {
                    None
                }
            })
        }
        "Damage bonus" => {
            preview.damage_bonus = value.and_then(|c| {
                if let Change::Add(n) = c {
                    Some(n)
                } else {
                    None
                }
            })
        }
        _ => {}
    }
    preview
}

impl Encounters {
    /// Why Save would refuse the open group now, if it would. Checked again only when the
    /// campaign's read (its revision) or the draft (its own revision) changes: never for a held
    /// pointer or a scroll (review of PR 299).
    fn group_refusal(&mut self) -> Option<Refusal> {
        let (snapshot, draft) = (self.snapshot.as_ref()?, self.group_draft.as_ref()?);
        #[cfg(any(test, feature = "test-support"))]
        GROUP_CHECKS.with(|counts| counts.set((counts.get().0 + 1, counts.get().1)));
        let fresh = self
            .group_check
            .as_ref()
            .is_some_and(|((revision, draft_revision), _)| {
                *revision == snapshot.revision && *draft_revision == draft.revision
            });
        if !fresh {
            let refusal = draft.check(snapshot, None).err();
            self.group_check = Some(((snapshot.revision, draft.revision), refusal));
        }
        self.group_check
            .as_ref()
            .and_then(|(_, refusal)| refusal.clone())
    }

    pub(super) fn save_group_draft(
        &mut self,
        ctx: &egui::Context,
        active: &Campaign,
        modules: &[Module],
    ) {
        let Some(draft) = self.group_draft.as_mut() else {
            return;
        };
        let path = draft.path.clone();
        self.watch_refusals(path);
        let Some(draft) = self.group_draft.as_mut() else {
            return;
        };
        // G122: one refusal path for the button and the keyboard. The refusal keeps its place
        // and its tone: a fault the body says in its place is not said again in the footer.
        if let Some(refusal) = self
            .snapshot
            .as_ref()
            .and_then(|snapshot| draft.check(snapshot, None).err())
        {
            self.refuse_group_save(&refusal);
            return;
        }
        let old_name = draft.baseline.as_ref().map(|_| draft.saved_name.clone());
        // Only a group for any encounter can be linked in a plan.
        let was_linkable = draft.saved_setup.encounter.is_none();
        let path = draft.path.clone();
        match draft.save(active, modules) {
            Ok(()) => {
                let name = draft.name.trim().to_owned();
                let linkable = draft.setup.encounter.is_none();
                self.saved = true;
                self.error = None;
                self.message = Some("Group saved.".into());
                self.stamp_kept(super::KeptResult::Saved);

                // The list and an open run read groups from the snapshot.
                self.refreshing = true;
                self.start_load(ctx, active.clone(), modules.to_vec());
                match old_name {
                    None if self.group_in_plan && linkable => self.saved_for_plan = Some(name),
                    Some(old)
                        if was_linkable
                            && old.trim() != name
                            && !self.name_stays_taken(&path, old.trim()) =>
                    {
                        self.renamed_group = Some((old.trim().to_owned(), name));
                    }
                    _ => {}
                }
                if self.group_in_plan {
                    self.close_group_in_plan();
                }
            }
            Err(e) => {
                match self
                    .snapshot
                    .as_ref()
                    .and_then(|snapshot| draft.check(snapshot, None).err())
                    .filter(|refusal| refusal.text() == e)
                {
                    Some(refusal) => self.refuse_group_save(&refusal),
                    None => {
                        self.refused(None);
                        self.error = Some(e);
                    }
                }
            }
        }
    }

    /// Save's check refused the group draft: the watch starts, and the footer says the fault
    /// only when no other place does.
    fn refuse_group_save(&mut self, refusal: &Refusal) {
        self.refused(Some(refusal.into()));
        self.error = refusal.footer().map(str::to_owned);
    }

    /// Whether something else still has this group's old name. A link with that name may mean
    /// the other thing, so a rename leaves it alone.
    fn name_stays_taken(&self, path: &str, old: &str) -> bool {
        self.snapshot.as_ref().is_some_and(|snapshot| {
            snapshot
                .templates
                .iter()
                .any(|template| template.name.eq_ignore_ascii_case(old))
                || snapshot.groups.iter().any(|group| {
                    group.encounter.is_none()
                        && group.path != path
                        && group.name.eq_ignore_ascii_case(old)
                })
        })
    }

    /// The plan shows again. The group stays where the DM left it.
    fn close_group_in_plan(&mut self) {
        self.group_draft = None;
        self.group_in_plan = false;
        self.message = None;
    }

    /// `Discard` in the group editor: the saved group opens clean, and the discarded draft
    /// waits for `Undo discard` with its results.
    pub(super) fn discard_group_draft(&mut self, active: &Campaign) {
        self.clear_notice();
        let draft = self.group_draft.take().expect("the editor has a draft");
        if draft.baseline.is_some() {
            match GroupDraft::open(&active.path, &draft.path) {
                Ok(reopened) => self.group_draft = Some(reopened),
                Err(e) => self.error = Some(e),
            }
        }
        self.discarded_group = Some(draft);
        // The Session plan's footer shows this store's line (`group_discard_line`).
        self.stamp_kept(super::KeptResult::GroupDiscarded);
        self.discarded = None;
    }
}

fn draft_member_summary(draft: &GroupDraft, snapshot: &Snapshot) -> String {
    draft
        .setup
        .members
        .iter()
        .map(|member| {
            let name = snapshot
                .catalogue
                .resolve_ref(&member.creature)
                .ok()
                .and_then(|id| snapshot.catalogue.entries.get(&id))
                .map(|entry| entry.entry.name.as_str())
                .unwrap_or("Unknown creature");
            format!(
                "{} × {}",
                member.count,
                shown_name(&member.overrides, name.to_owned())
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// The key of a member's open changes form and of its fields: its group and its identity in the
/// draft, never its place among the members.
fn member_key(path: &str, member: &MemberDraft) -> String {
    format!("{path}:member_{}", member.id.0)
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// (frames that reached `members_are_saved`, comparisons it made)
    static MEMBER_COMPARISONS: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((0, 0)) };
}
#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// (frames that asked for the open group's check, checks that ran in `GroupDraft::check`)
    static GROUP_CHECKS: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((0, 0)) };
}
/// The counts of Save's check of a group, for `CachedWork`.
#[cfg(any(test, feature = "test-support"))]
pub fn group_checks() -> (usize, usize) {
    GROUP_CHECKS.with(std::cell::Cell::get)
}
#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How often the member loop copied a member's change map (review of PR 299).
    static CHANGE_COPIES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}
/// How often the member loop copied a member's change map.
#[cfg(any(test, feature = "test-support"))]
pub fn change_copies() -> usize {
    CHANGE_COPIES.with(std::cell::Cell::get)
}
/// The counts of `members_are_saved`, for `CachedWork`.
#[cfg(any(test, feature = "test-support"))]
pub fn member_comparisons() -> (usize, usize) {
    MEMBER_COMPARISONS.with(std::cell::Cell::get)
}

/// A new revision for a group draft (`GroupDraft::touch`): larger than every one before it.
static NEXT_REVISION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

thread_local! {
    /// F5: the group revisions issued on this thread. The page and its drafts live on the
    /// thread that draws them, so another thread's drafts (another app in the same process, as
    /// the visual suite runs them) never change it.
    static ISSUED_HERE: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

fn next_revision() -> u64 {
    ISSUED_HERE.with(|issued| issued.set(issued.get() + 1));
    NEXT_REVISION.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// F5: the count of group revisions issued on this thread so far. Every edit of a group draft
/// (`touch`), a new draft and an opened one issue a revision on the thread that draws it, so an
/// unchanged count means that no group of this page changed: a stored edit generation that a
/// cache reads in constant time. Revisions themselves stay unique across threads.
pub(super) fn revisions_issued() -> u64 {
    ISSUED_HERE.with(std::cell::Cell::get)
}

/// Remove the member at `index`: its data goes with the result to the page's store.
fn remove_member(draft: &mut GroupDraft, index: usize, snapshot: &Snapshot) {
    let member = draft.setup.members.remove(index);
    let base = snapshot
        .catalogue
        .resolve_ref(&member.creature)
        .ok()
        .and_then(|id| snapshot.catalogue.entries.get(&id))
        .map(|entry| entry.entry.name.clone())
        .unwrap_or_else(|| missing_member_name(None, member.creature.link.as_deref()));
    let name = shown_name(&member.overrides, base);
    draft.member_new = Some(MemberUndo::Removed {
        index,
        member,
        name,
    });
}

/// How many creatures will arrive, and what they are worth, as the list and the panel say it.
fn members_heading(draft: &GroupDraft, snapshot: &Snapshot) -> String {
    let Some(mut entry) = draft
        .text()
        .ok()
        .and_then(|text| Entry::parse(&draft.path, &text).ok())
    else {
        return "XP not known".into();
    };
    let entries: Vec<_> = snapshot
        .catalogue
        .entries
        .values()
        .map(|e| e.entry.clone())
        .collect();
    splat::content::list_formats::resolve(&mut entry, &entries, "");
    resolve_group(
        &entry,
        None,
        &snapshot.catalogue,
        &snapshot.creatures,
        Uuid::nil(),
    )
    .map_or_else(
        || "XP not known".into(),
        |group| {
            // When not all can join, the XP is for those who can, and the heading says so.
            if group.unavailable_members.is_empty() {
                group.xp_label()
            } else if group.joinable_count() == 0 {
                "None can join".into()
            } else {
                format!("{} can join · {}", group.joinable_count(), group.xp_label())
            }
        },
    )
}

/// The repair a member fault other than a missing creature offers.
pub(crate) const EDIT_GROUP: &str = "Edit group";

/// Where a missing creature's package is chosen.
pub(crate) const PACKAGES_LINK: &str = "Campaign → Packages";

/// The group editor's save-row diagnosis: the cached check's one ranked result (E2), owned and
/// passed through whole (E4). An empty group says its reason beside Save (`Add a creature
/// first.`) only while nothing outranks the missing part.
fn save_row_diagnosis(refusal: Option<&Refusal>, empty: bool) -> design::save_row::Diagnosis<'_> {
    use design::save_row::Diagnosis;
    match refusal {
        Some(refusal) if empty && refusal.diagnosis().only_incomplete() => {
            Diagnosis::incomplete(EMPTY_REASON)
        }
        Some(refusal) => refusal.diagnosis().clone(),
        None if empty => Diagnosis::incomplete(EMPTY_REASON),
        None => Diagnosis::valid(),
    }
}

// ------------------------------------------------------------------------------------------------
// The group editor as plain data, for the GPUI page (`encounters::model`).

/// The group editor, as data.
pub struct GroupView {
    pub id: Uuid,
    pub name: String,
    pub back: super::model::BackLine,
    pub save_primary: bool,
    pub save_blocked: Option<String>,
    pub state: (String, String, egui::Color32),
    pub discard: bool,
    /// `Add a creature first.` beside Save, and the campaign-switch reason.
    pub empty_reason: Option<&'static str>,
    pub lock_reason: Option<&'static str>,
    /// The words beside Save while its refusal is said in a row or under Belongs to.
    pub elsewhere: Option<&'static str>,
    /// Default side, Belongs to, the members and the notes.
    pub fields: GroupFields,
    /// The members' summary under the search, in the narrow layout (`draft_member_summary`).
    pub summary: String,
    pub reading: bool,
}

/// A change to a group's member from its row.
#[derive(Clone, Copy, Debug)]
pub enum MemberAct {
    Up(usize),
    Down(usize),
    Remove(usize),
    Count(usize, u64),
}

/// The edits a view makes to the open group draft.
pub struct GroupEdit<'a>(&'a mut GroupDraft);

impl GroupEdit<'_> {
    pub fn name(&mut self, words: &str) {
        self.0.name = words.to_owned();
        self.0.touch();
    }
    pub fn side(&mut self, side: &str) {
        if !side.trim().is_empty() {
            self.0.setup.side = side.trim().to_lowercase();
            self.0.touch();
        }
    }
    pub fn belongs(&mut self, encounter: Option<Uuid>) {
        self.0.setup.encounter = encounter.map(EntryRef::id);
        self.0.touch();
    }
    pub fn notes(&mut self, words: &str) {
        self.0.notes = words.to_owned().into();
        self.0.touch();
    }
}

impl Encounters {
    /// The group editor's head, save row and body (`group_editor_view`), as data.
    pub fn group_frame(
        &mut self,
        sources: Option<&crate::loot_editor::Sources>,
    ) -> Option<GroupView> {
        let dirty = self.draft_dirty();
        let has_run = self.has_run();
        let refusal = self.group_refusal();
        let snapshot = self.snapshot.as_ref();
        let draft = self.group_draft.as_ref()?;
        let empty = draft.setup.members.is_empty();
        let changed = if dirty {
            design::memo::Changed::Yes
        } else {
            design::memo::Changed::No
        };
        let primary = if has_run {
            design::save_row::Primary::Elsewhere
        } else {
            design::save_row::Primary::Draft
        };
        let mut state = design::DraftState::new(draft.origin(), changed, primary).locks_campaign();
        state = state.check(save_row_diagnosis(refusal.as_ref(), empty));
        if self.message.is_some() {
            state = state.just_saved();
        }
        // Drawn in a session's plan, Back goes to the plan (`group_in_plan_view`).
        let in_plan = self.group_in_plan;
        let head = design::EditorHead::new(design::EditorKind::Group { in_plan }, &*self);
        let (word, back_name) = head.back_words();
        let status = state.status();
        let elsewhere = refusal.as_ref().and_then(|refusal| match refusal.place() {
            RefusalPlace::BelongsTo => Some(GONE_BESIDE_SAVE),
            RefusalPlace::Member(_) => Some(MEMBER_BESIDE_SAVE),
            _ => None,
        });
        let summary = snapshot.map_or_else(String::new, |snapshot| {
            draft_member_summary(draft, snapshot)
        });
        let reading = snapshot.is_none();
        let id = draft.id();
        let name = draft.name.clone();
        let fields = self.group_fields_at(view::GroupAt::Open, sources)?;
        Some(GroupView {
            id,
            name,
            back: super::model::BackLine {
                word,
                name: back_name,
                held: head
                    .held_by()
                    .map(|draft| (draft.reason(), draft.short_reason())),
            },
            save_primary: matches!(state.save_weight(), design::save_row::SaveWeight::Primary),
            save_blocked: state.save_blocked().map(str::to_owned),
            state: (status.icon.to_owned(), status.word.to_owned(), status.color),
            discard: state.discard_shown(),
            empty_reason: empty.then_some(EMPTY_REASON),
            lock_reason: state.lock_reason(),
            elsewhere,
            fields,
            summary,
            reading,
        })
    }

    /// A press in the group editor's head: Back, Discard or Save. True when it closed.
    pub fn group_head_act(
        &mut self,
        ctx: &egui::Context,
        active: &Campaign,
        modules: &[Module],
        act: super::model::HeadAct,
    ) -> bool {
        match act {
            super::model::HeadAct::Back => {
                self.clear_notice();
                self.close_group_in_plan();
                self.error = None;
                true
            }
            super::model::HeadAct::Discard => {
                self.discard_group_draft(active);
                true
            }
            _ => {
                if self.group_draft.as_ref().is_some_and(GroupDraft::dirty) {
                    self.save_group_draft(ctx, active, modules);
                }
                false
            }
        }
    }

    /// The open group draft's members, each by its creature's link words and its count.
    pub fn group_members(&self) -> Option<Vec<(String, u64)>> {
        let draft = self.group_draft.as_ref()?;
        Some(
            draft
                .setup
                .members
                .iter()
                .map(|member| (member.creature.label(), member.count))
                .collect(),
        )
    }

    /// Whether a group editor is open, and its draft has changes.
    pub fn group_open(&self) -> Option<bool> {
        self.group_draft.as_ref().map(GroupDraft::dirty)
    }

    /// Change the open group draft.
    pub fn edit_group(&mut self, edit: impl FnOnce(&mut GroupEdit<'_>)) {
        if let Some(draft) = self.group_draft.as_mut() {
            edit(&mut GroupEdit(draft));
        }
    }

    /// A member's change from its row: its place, its count, or its removal (with its Undo).
    pub fn member_act(&mut self, act: MemberAct) {
        let Some(snapshot) = self.snapshot.as_ref() else {
            return;
        };
        let Some(draft) = self.group_draft.as_mut() else {
            return;
        };
        let total = draft.setup.members.len();
        match act {
            MemberAct::Up(index) if index > 0 && index < total => {
                draft.setup.members.swap(index, index - 1);
            }
            MemberAct::Down(index) if index + 1 < total => {
                draft.setup.members.swap(index, index + 1);
            }
            MemberAct::Remove(index) if index < total => remove_member(draft, index, snapshot),
            MemberAct::Count(index, count) if index < total => {
                draft.setup.members[index].count = count.clamp(1, 99);
            }
            _ => return,
        }
        draft.touch();
        self.expanded = None;
    }

    /// The group editor's footer lines (`group_editor_view`'s `then`), for both views: the
    /// member result, the discard, the copies' separations, newest first by their stores, each
    /// with its ticket; then Save's reason, a failed separation and the file's warnings.
    pub(super) fn group_lines<'f>(
        &mut self,
        messages: &mut design::outcome::Messages<'f, super::FooterPress>,
        frame: &'f design::outcome::PaneFrame,
        root: &std::path::Path,
        refusal: Option<&Refusal>,
        file_warnings: Vec<String>,
    ) {
        self.record_member_results();
        let kinds = self.kept_results(false);
        if let Some(draft) = self.group_draft.as_ref()
            && let Some(shown) = self.member_results.get(&super::MemberOwner {
                root: root.to_owned(),
                draft: draft.results_key(),
            })
        {
            messages.kept(shown.outcome(frame, super::MemberMark::outcome));
        }
        for line in self.kept_lines(frame, &kinds) {
            messages.kept(line);
        }
        for line in self.separation_lines(frame, root) {
            messages.kept(line);
        }
        // Save's reason: the last refusal, or the check's fault when the editor shows it at no
        // other place (a member row or the reason beside Save already says theirs). R41: a
        // refusal's words take the refusal's tone; a failed write is an error.
        let live =
            refusal.and_then(|refusal| refusal.footer().map(|words| (words, refusal.diagnosis())));
        let reason = match (self.error.clone(), live) {
            (Some(error), Some((words, found))) if error == words => {
                Some(super::EncounterFault::refusal(error, found))
            }
            (Some(error), _) => Some(super::EncounterFault::Failed(error)),
            (None, Some((words, found))) => {
                Some(super::EncounterFault::refusal(words.to_owned(), found))
            }
            (None, None) => None,
        };
        if let Some(fault) = reason {
            messages.outcome(design::outcome::FaultMessage::new(&fault));
        }
        if let Some(error) = self.separation_failure() {
            messages.outcome(design::outcome::FaultMessage::new(
                &super::EncounterFault::SeparationFailed(error),
            ));
        }
        for warning in file_warnings {
            messages.outcome(design::outcome::FaultMessage::new(
                &super::EncounterFault::FileWarning(warning),
            ));
        }
    }

    /// The group editor's footer as plain lines, for the GPUI page (`group_lines`).
    pub fn group_footer(
        &mut self,
        root: &std::path::Path,
    ) -> Vec<design::outcome::plain::PlainLine<super::FooterPress>> {
        let refusal = self.group_refusal();
        let file_warnings: Vec<String> = self
            .group_draft
            .as_ref()
            .map(|draft| {
                let unread =
                    splat::content::list_formats::unread_group_members(&draft.source, &draft.flags);
                draft
                    .flags
                    .iter()
                    .filter(|flag| !unread.iter().any(|member| member.flag.line == flag.line))
                    .map(|flag| flag.message.clone())
                    .collect()
            })
            .unwrap_or_default();
        let frame = design::outcome::PaneFrame::plain();
        let footer_id = self.group_draft.as_ref().map(|draft| draft.id());
        let mut messages =
            design::outcome::Messages::new(&frame, ("group_editor_footer", footer_id), 0.0);
        self.group_lines(&mut messages, &frame, root, refusal.as_ref(), file_warnings);
        messages.into_plain()
    }
}

#[cfg(test)]
mod tests {
    use super::super::Draft;
    use super::*;

    fn campaign(root: &std::path::Path) -> Campaign {
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Arena".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        if campaign.path.is_dir() && !campaign.path.join("Campaign.md").exists() {
            crate::content_workspace::CampaignDocument::new(&campaign)
                .unwrap()
                .save()
                .unwrap();
        }
        campaign
    }
    fn creature(root: &std::path::Path, name: &str) -> Uuid {
        let id = Uuid::new_v4();
        write_note(
            root,
            &format!("{name}.md"),
            None,
            &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {id}\nsplat_kind: creature\nsplat_name: {name}\nsplat_creature: {{cr_text: '1/4 (XP 50; PB +2)'}}\nsplat_stats:\n  max_hp: {{value: 10}}\n  armor_class: {{value: 15}}\n  initiative: {{value: 2}}\n  speed: {{value: 30}}\n"), "").expect("schema-2 fixture"),
        )
        .unwrap();
        id
    }

    /// Review of PR 339 (1): a member result belongs to its draft's content. Remove a member,
    /// Discard (the saved group opens clean, with the same identity), then Undo discard: the
    /// member is still removed and its result comes back with its Undo.
    #[test]
    fn undo_discard_brings_back_the_group_member_result() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let (goblin, wolf) = (creature(dir.path(), "Goblin"), creature(dir.path(), "Wolf"));
        let mut saved = GroupDraft::new();
        saved.name = "Pack".into();
        saved.add(goblin);
        saved.add(wolf);
        saved.save(&active, &[]).unwrap();
        let mut page = Encounters {
            active: Some(active.clone()),
            group_draft: Some(GroupDraft::open(dir.path(), &saved.path).unwrap()),
            ..Default::default()
        };
        // Remove as the member list does: the member's data, then the mark.
        let draft = page.group_draft.as_mut().unwrap();
        let member = draft.setup.members.remove(0);
        draft.member_new = Some(MemberUndo::Removed {
            index: 0,
            member,
            name: "Goblin".into(),
        });
        draft.touch();
        let owner = super::super::MemberOwner {
            root: active.path.clone(),
            draft: draft.results_key(),
        };
        page.record_member_results();
        assert!(
            page.member_results.get(&owner).is_some(),
            "recorded at the act"
        );
        page.discard_group_draft(&active);
        page.record_member_results();
        assert!(
            page.member_results.get(&owner).is_some(),
            "the discarded draft keeps its result while it waits for Undo discard"
        );
        assert!(page.undo_kept(super::super::KeptResult::GroupDiscarded, None));
        page.record_member_results();
        let draft = page.group_draft.as_mut().unwrap();
        assert_eq!(draft.setup.members.len(), 1, "the member is still removed");
        assert!(
            page.member_results.get(&owner).is_some(),
            "Undo discard shows the member result again, with its Undo"
        );
        // Review 2 of PR 347: the result lives while its draft can come back, and ends when it
        // cannot, through `poll` as the app calls it. Discard again: it is kept. Edit the clean
        // draft: Undo discard expires, and the result ends. Repeated, no entry builds up.
        let ctx = egui::Context::default();
        let campaigns = crate::campaigns::Campaigns::default();
        for _ in 0..2 {
            let draft = page.group_draft.as_mut().unwrap();
            let owner = super::super::MemberOwner {
                root: active.path.clone(),
                draft: draft.results_key(),
            };
            let member = draft.setup.members.remove(0);
            draft.member_new = Some(MemberUndo::Removed {
                index: 0,
                member,
                name: "Wolf".into(),
            });
            draft.touch();
            page.record_member_results();
            page.discard_group_draft(&active);
            page.poll(&ctx, &campaigns);
            assert!(
                page.member_results.get(&owner).is_some(),
                "kept while Undo discard can bring its draft back"
            );
            page.group_draft.as_mut().unwrap().name = "Edited".into();
            let _ = page.kept_results(false);
            assert!(page.discarded_group.is_none(), "Undo discard expired");
            page.poll(&ctx, &campaigns);
            assert!(
                page.member_results.get(&owner).is_none(),
                "its draft cannot come back: the result ends"
            );
            assert!(page.member_results.is_empty(), "no entry is left behind");
            let reopened = GroupDraft::open(dir.path(), &saved.path).unwrap();
            page.group_draft = Some(reopened);
        }
    }

    /// Round 28 (K3 review): a member's key is its identity in the draft. A move keeps each key
    /// with its member, two members never share one, and the identity is not part of what a
    /// member is: the draft still equals its saved form, and a member read again equals it.
    #[test]
    fn a_member_keys_its_form_by_its_identity_not_its_place() {
        let mut draft = GroupDraft::new();
        let (a, b) = (Uuid::new_v4(), Uuid::new_v4());
        draft.add(a);
        draft.add(b);
        let keys: Vec<String> = draft
            .setup
            .members
            .iter()
            .map(|member| member_key("g.md", member))
            .collect();
        assert_ne!(keys[0], keys[1], "two members, two keys");
        draft.setup.members.swap(0, 1);
        assert_eq!(
            member_key("g.md", &draft.setup.members[1]),
            keys[0],
            "A keeps its key"
        );
        assert_eq!(
            member_key("g.md", &draft.setup.members[0]),
            keys[1],
            "B keeps its key"
        );
        // Not a saved part of a member: the same words compare equal.
        let read: GroupSetup =
            serde_yaml_ng::from_value(serde_yaml_ng::to_value(&draft.setup).unwrap()).unwrap();
        assert_eq!(read, draft.setup);
        assert_ne!(
            member_key("g.md", &read.members[0]),
            keys[1],
            "a member read again is new"
        );
    }

    #[test]
    fn a_group_edit_rechecks_the_last_validation_failure() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let id = creature(dir.path(), "Scout");
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let mut group = GroupDraft::new();
        group.add(id);
        group.name.clear();
        let error = group.validate(&snapshot, None).unwrap_err();
        let mut encounters = Encounters {
            validation_path: Some(group.path.clone()),
            validation_error: Some(error.clone().into()),
            refusal_watch: true,
            error: Some(error),
            snapshot: Some(snapshot),
            group_draft: Some(group),
            ..Default::default()
        };
        encounters.group_draft.as_mut().unwrap().name = "Scouts".into();
        encounters.refresh_validation_error(true, true);
        assert!(encounters.error.is_none());
        encounters
            .group_draft
            .as_mut()
            .unwrap()
            .setup
            .members
            .clear();
        encounters.refresh_validation_error(true, true);
        // The recheck finds the new fault. Its reason is beside Save (`Add a creature first.`),
        // so the footer does not say it again (G122: one place for each refusal).
        assert_eq!(
            encounters
                .validation_error
                .as_ref()
                .map(|invalid| invalid.text.as_str()),
            Some("This group is empty. Add at least one creature.")
        );
        assert!(encounters.error.is_none(), "{:?}", encounters.error);
    }

    #[test]
    fn removing_a_saved_group_can_restore_it_without_overwriting_a_new_note() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let id = creature(dir.path(), "Scout");
        let mut group = GroupDraft::new();
        group.add(id);
        group.save(&active, &[]).unwrap();
        let path = note_path(dir.path(), &group.path).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        // A group is not an encounter, so an encounter's Remove leaves it.
        assert!(RemovedNote::remove(&active, &group.path, "encounter").is_err());
        assert!(path.exists());
        let removed = RemovedNote::remove(&active, &group.path, "encounter_group").unwrap();
        assert!(!path.exists());
        removed.restore().unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), text);
        fs::write(&path, "A new note").unwrap();
        assert!(removed.restore().is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "A new note");

        // An encounter is removed and comes back the same way (T12.12).
        let mut fight = Draft::new();
        fight.name = "Fight with Scout".into();
        fight.add(id, false);
        fight.save(&active, &[]).unwrap();
        let fight_path = note_path(dir.path(), &fight.path).unwrap();
        let fight_text = fs::read_to_string(&fight_path).unwrap();
        assert!(RemovedNote::remove(&active, &fight.path, "encounter_group").is_err());
        let removed = RemovedNote::remove(&active, &fight.path, "encounter").unwrap();
        assert_eq!(removed.name, "Fight with Scout");
        assert!(!fight_path.exists());
        removed.restore().unwrap();
        assert_eq!(fs::read_to_string(&fight_path).unwrap(), fight_text);
    }

    /// The draft closes on save, so that the plan shows again with its new link.
    #[test]
    fn a_group_saved_from_the_plan_goes_back_to_it_and_a_rename_follows() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let ctx = egui::Context::default();
        let mut encounters = Encounters::default();
        encounters.new_group_in_plan();
        assert!(encounters.group_open_in_plan());
        let draft = encounters.group_draft.as_mut().unwrap();
        draft.name = "Ash Pack".into();
        draft.add(scout);
        encounters.save_group_draft(&ctx, &campaign, &[]);
        let path = "Encounter Groups/Ash Pack.md".to_owned();
        assert_eq!(
            encounters.saved_for_plan.take().as_deref(),
            Some("Ash Pack")
        );
        assert!(!encounters.group_open_in_plan());
        assert!(encounters.group_draft.is_none());

        assert_eq!(
            encounters.open_group_in_plan(&path, &campaign),
            crate::encounters::PlanOpen::Opened
        );
        encounters.group_draft.as_mut().unwrap().name = "Cinder Pack".into();
        encounters.save_group_draft(&ctx, &campaign, &[]);
        assert_eq!(encounters.saved_for_plan, None, "it is linked already");
        assert_eq!(
            encounters.renamed_group.take(),
            Some(("Ash Pack".to_owned(), "Cinder Pack".to_owned()))
        );
        assert!(!encounters.group_open_in_plan());
    }

    #[test]
    fn a_different_group_clears_search_but_the_same_group_keeps_it() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let ctx = egui::Context::default();
        let mut encounters = Encounters::default();
        let mut paths = Vec::new();
        for name in ["Ash Pack", "Rival Team"] {
            encounters.new_group_in_plan();
            let draft = encounters.group_draft.as_mut().unwrap();
            draft.name = name.into();
            draft.add(scout);
            encounters.save_group_draft(&ctx, &campaign, &[]);
            paths.push(format!("Encounter Groups/{name}.md"));
        }
        assert_eq!(
            encounters.open_group_in_plan(&paths[0], &campaign),
            crate::encounters::PlanOpen::Opened
        );
        encounters.search = "wolf".into();
        assert_eq!(
            encounters.open_group_in_plan(&paths[0], &campaign),
            crate::encounters::PlanOpen::Opened
        );
        assert_eq!(encounters.search, "wolf");
        assert_eq!(
            encounters.open_group_in_plan(&paths[1], &campaign),
            crate::encounters::PlanOpen::Opened
        );
        assert!(encounters.search.is_empty());
        encounters.search = "scout".into();
        encounters.start_new_draft();
        assert!(encounters.search.is_empty());
    }

    #[test]
    fn a_group_taken_up_on_the_encounters_page_links_nothing_but_a_rename_still_follows() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let ctx = egui::Context::default();
        let mut encounters = Encounters {
            group_draft: Some(GroupDraft::new()),
            ..Encounters::default()
        };
        let draft = encounters.group_draft.as_mut().unwrap();
        draft.name = "Ash Pack".into();
        draft.add(scout);
        encounters.save_group_draft(&ctx, &campaign, &[]);
        assert_eq!(encounters.saved_for_plan, None);
        assert!(encounters.group_draft.is_some(), "this page keeps it open");
        encounters.group_draft.as_mut().unwrap().name = "Cinder Pack".into();
        encounters.save_group_draft(&ctx, &campaign, &[]);
        assert_eq!(
            encounters.renamed_group.take(),
            Some(("Ash Pack".to_owned(), "Cinder Pack".to_owned()))
        );
    }

    #[test]
    fn a_rename_leaves_links_that_may_mean_something_else_with_the_old_name() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let mut draft = GroupDraft::new();
        draft.name = "Patrol".into();
        draft.add(scout);
        draft.save(&campaign, &[]).unwrap();
        // Older files can share a name. Nothing saved through the app can.
        write_note(
            dir.path(),
            "Patrol encounter.md",
            None,
            &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: encounter\nsplat_name: Patrol\nsplat_encounter: {{roster: []}}\n", Uuid::new_v4()), "").expect("schema-2 fixture"),
        )
        .unwrap();
        let ctx = egui::Context::default();
        let mut encounters = Encounters {
            snapshot: Some(Snapshot::load(&campaign, &[]).unwrap()),
            group_draft: Some(GroupDraft::open(dir.path(), &draft.path).unwrap()),
            ..Encounters::default()
        };
        encounters.group_draft.as_mut().unwrap().name = "Scouts".into();
        encounters.save_group_draft(&ctx, &campaign, &[]);
        assert_eq!(encounters.error, None);
        assert_eq!(encounters.renamed_group, None, "the fight keeps its links");
    }

    #[test]
    fn one_name_means_one_thing_to_a_plan() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let mut first = GroupDraft::new();
        first.name = "Patrol".into();
        first.add(scout);
        first.save(&campaign, &[]).unwrap();
        let mut second = GroupDraft::new();
        second.name = " PATROL ".into();
        second.add(scout);
        let error = second.save(&campaign, &[]).unwrap_err();
        assert!(error.contains("Another group"), "{error}");
        assert!(!dir.path().join(&second.path).exists());
        // A group keeps its own name through an edit.
        first.notes.replace("New words.");
        first.save(&campaign, &[]).unwrap();
        write_note(
            dir.path(),
            "Ford.md",
            None,
            &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: encounter\nsplat_name: Ford\nsplat_encounter: {{roster: []}}\n", Uuid::new_v4()), "").expect("schema-2 fixture"),
        )
        .unwrap();
        second.name = "ford".into();
        let error = second.save(&campaign, &[]).unwrap_err();
        assert!(error.contains("An encounter"), "{error}");
    }

    #[test]
    fn groups_of_different_encounters_can_share_a_name_and_a_plan_offers_neither() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let encounter = |name: &str| {
            let id = Uuid::new_v4();
            write_note(
                dir.path(),
                &format!("{name}.md"),
                None,
                &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {id}\nsplat_kind: encounter\nsplat_name: {name}\nsplat_encounter: {{roster: []}}\n"), "").expect("schema-2 fixture"),
            )
            .unwrap();
            id
        };
        let (bridge, shrine) = (encounter("Bridge"), encounter("Shrine"));
        for owner in [bridge, shrine] {
            let mut draft = GroupDraft::for_encounter(owner);
            draft.name = "Reinforcements".into();
            draft.add(scout);
            draft.save(&campaign, &[]).unwrap();
        }
        // A group for any encounter is linked by name, so it can take a name only they have.
        let mut free = GroupDraft::new();
        free.name = "Reinforcements".into();
        free.add(scout);
        free.save(&campaign, &[]).unwrap();
        let mut encounters = Encounters {
            snapshot: Some(Snapshot::load(&campaign, &[]).unwrap()),
            ..Encounters::default()
        };
        let offered = encounters.plan_groups();
        assert_eq!(offered.len(), 1);
        assert_eq!(offered[0].encounter, None);
        // Layout 11c A2a: the plan's lists are kept by the snapshot's and the saved runs'
        // revisions: two frames with no change share one list, copied once.
        let kept = encounters.plan_kept();
        let _ = encounters.plan_kept();
        assert_eq!(
            encounters.plan_kept_builds, 1,
            "a quiet frame copies no plan list"
        );
        assert_eq!(kept.groups.len(), 1, "the kept groups are the offered ones");
        // Review of PR 804 (a cache proof changes content): each input of the kept lists, alone.
        // The snapshot: the free group renamed on disk, with the same count, read again.
        free.name = "Wolf Pack".into();
        free.save(&campaign, &[]).unwrap();
        encounters.snapshot = Some(Snapshot::load(&campaign, &[]).unwrap());
        let kept = encounters.plan_kept();
        assert_eq!(
            kept.groups
                .iter()
                .map(|group| group.name.as_str())
                .collect::<Vec<_>>(),
            ["Wolf Pack"],
            "a group renamed on disk is listed by its new name"
        );
        // The saved runs: Bridge's unfinished run, listed with the snapshot's revision kept.
        let run = crate::encounter_run::SavedRunInfo {
            path: std::path::PathBuf::from("bridge.json"),
            id: Some(Uuid::new_v4()),
            source: Some(bridge),
            encounter: "Bridge".into(),
            label: "Round 2".into(),
            standing: crate::encounter_run::Standing::Unfinished,
            setup: false,
            saved_at: None,
            unsent: vec![],
        };
        encounters
            .snapshot
            .as_mut()
            .unwrap()
            .take_run_list(Ok(vec![run]), false);
        encounters.note_run_facts();
        let kept = encounters.plan_kept();
        assert!(
            kept.fights
                .iter()
                .any(|fight| fight.name == "Bridge" && fight.saved.is_some()),
            "a run saved for a fight is listed with it"
        );
    }

    #[test]
    fn a_group_saved_for_an_encounter_from_the_plan_is_not_linked_there() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let owner = Uuid::new_v4();
        write_note(
            dir.path(),
            "Bridge.md",
            None,
            &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {owner}\nsplat_kind: encounter\nsplat_name: Bridge\nsplat_encounter: {{roster: []}}\n"), "").expect("schema-2 fixture"),
        )
        .unwrap();
        let ctx = egui::Context::default();
        let mut encounters = Encounters::default();
        encounters.new_group_in_plan();
        let mut draft = GroupDraft::for_encounter(owner);
        draft.name = "Wave".into();
        draft.add(scout);
        encounters.group_draft = Some(draft);
        encounters.save_group_draft(&ctx, &campaign, &[]);
        assert_eq!(encounters.error, None);
        assert_eq!(encounters.saved_for_plan, None);
        assert!(!encounters.group_open_in_plan());
    }

    #[test]
    fn an_encounter_draft_left_open_does_not_hide_a_plan_groups_undo() {
        let mut encounters = Encounters::default();
        encounters.new_group_in_plan();
        // B4, E1: a new group is refused over an unsaved encounter, so the encounter draft is
        // opened after the group; what is tested is the Undo with both open.
        encounters.draft = Some(Draft::new());
        encounters.active = Some(campaign(std::path::Path::new("arena")));
        let ticket = discard_in_plan(&mut encounters);
        assert!(encounters.group_discarded_in_plan());
        encounters.undo_group_discard_in_plan(ticket);
        assert!(encounters.group_open_in_plan());
    }

    /// A plan group's discard as the page's kept store holds it, and the ticket of its line
    /// (`group_discard_line`), as the Session plan's footer gets it.
    fn discard_in_plan(encounters: &mut Encounters) -> design::results::Ticket {
        encounters.discarded_group = encounters.group_draft.take();
        encounters.stamp_kept(super::super::KeptResult::GroupDiscarded);
        discard_ticket(encounters)
    }

    /// The ticket of the line of the discard the page's store holds now.
    fn discard_ticket(encounters: &Encounters) -> design::results::Ticket {
        let root = encounters.active.as_ref().unwrap().path.clone();
        encounters
            .kept
            .get(&super::super::KeptOwner {
                root,
                kind: super::super::KeptResult::GroupDiscarded,
            })
            .expect("the discard is in the page's store")
            .ticket_for_caller_test()
    }

    #[test]
    fn a_group_discarded_in_the_plan_can_come_back_there() {
        let mut encounters = Encounters {
            active: Some(campaign(std::path::Path::new("arena"))),
            ..Encounters::default()
        };
        encounters.new_group_in_plan();
        encounters.group_draft.as_mut().unwrap().name = "Ash Pack".into();
        let ticket = discard_in_plan(&mut encounters);
        assert!(encounters.group_discarded_in_plan());
        assert!(!encounters.group_open_in_plan());
        encounters.undo_group_discard_in_plan(ticket);
        assert!(encounters.group_open_in_plan());
        assert_eq!(encounters.group_draft.as_ref().unwrap().name, "Ash Pack");
        // M0244: the plan's Undo acts by its line's ticket. A line of the first discard
        // pressed after its Undo and a second discard reaches nothing; the second discard's
        // line brings the draft back.
        let first = discard_in_plan(&mut encounters);
        let first_again = discard_ticket(&encounters);
        encounters.undo_group_discard_in_plan(first);
        let second = discard_in_plan(&mut encounters);
        encounters.undo_group_discard_in_plan(first_again);
        assert!(
            encounters.group_discarded_in_plan(),
            "a press on the first discard's line leaves the second discard alone"
        );
        encounters.undo_group_discard_in_plan(second);
        assert!(encounters.group_open_in_plan());
    }

    #[test]
    fn a_group_that_cannot_save_stays_open_in_the_plan() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let ctx = egui::Context::default();
        let mut encounters = Encounters::default();
        encounters.new_group_in_plan();
        encounters.save_group_draft(&ctx, &campaign, &[]);
        assert!(encounters.group_open_in_plan());
        assert!(encounters.error.as_deref().unwrap().contains("empty"));
        assert_eq!(encounters.saved_for_plan, None);
    }

    #[test]
    fn a_run_is_offered_every_creature_but_a_linked_one() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        creature(dir.path(), "Scout");
        let id = Uuid::new_v4();
        write_note(
            dir.path(),
            "Hero.md",
            None,
            &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {id}\nsplat_kind: creature\nsplat_name: Hero\nsplat_creature: {{}}\nsplat_external: {{provider: dndbeyond, character_id: '1'}}\n"), "").expect("schema-2 fixture"),
        )
        .unwrap();
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let names: Vec<_> = snapshot
            .run_creatures
            .iter()
            .map(|creature| creature.entry.name.as_str())
            .collect();
        assert!(names.contains(&"Scout"), "its own");
        assert!(
            !names.contains(&"Hero"),
            "a sheet can be added once, on the roster"
        );
    }

    #[test]
    fn an_unread_changes_row_cannot_join_but_other_members_stay_usable() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let source = format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Broken row\nsplat_side: enemies\n---\n\n## Members\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| 7 | [[Scout]] | Lookout | HP lots | Keep this row |\n| 2 | [[Scout]] | | | |\n| 1 | [[Lost scout]] | Picket | | |\n",
            Uuid::new_v4()
        );
        fs::write(dir.path().join("Broken row.md"), &source).unwrap();
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let group = snapshot
            .groups
            .iter()
            .find(|group| group.name == "Broken row")
            .unwrap();
        assert!(
            group.problem.is_none(),
            "the valid members can join: {:?}",
            group.problem
        );
        assert_eq!(group.members.len(), 1);
        assert_eq!(group.members[0].entry.id, scout);
        assert_eq!(
            group.creature_count(),
            10,
            "all named members remain visible"
        );
        assert!(group.summary().contains("2 of 10 can join"));
        // FB2: the summary names the row, its line and the cause; the example is the editor's.
        // FA1, FC5: an unread row and a missing creature in one group take the one form, which
        // the list, the plan and Arrivals all draw from these warnings.
        assert_eq!(
            group
                .warnings
                .iter()
                .map(|warning| warning.text.as_str())
                .collect::<Vec<_>>(),
            [
                "Picket (as Lost scout) · line\u{a0}15: Not in this campaign’s packages.",
                "Lookout (as Scout) · line\u{a0}13: The change “HP lots” was not read.",
            ]
        );
        assert!(
            group
                .warnings
                .iter()
                .all(|warning| !warning.text.contains("Use a number")),
            "{:?}",
            group
                .warnings
                .iter()
                .map(|warning| &warning.text)
                .collect::<Vec<_>>()
        );
        assert_eq!(group.xp_label(), "XP 100");
        let draft = GroupDraft::open(dir.path(), "Broken row.md").unwrap();
        let unread =
            splat::content::list_formats::unread_group_members(&draft.source, &draft.flags);
        assert_eq!(unread.len(), 1);
        assert_eq!(
            unread[0].flag.message,
            "Line 13: The change “HP lots” was not read. Use a number, such as HP +5.",
            "the editor's words keep the example"
        );
        assert_eq!(
            unread[0].before, 0,
            "the rejected row precedes the valid member"
        );
        assert_eq!(unread[0].count, Some(7));
        // Contract F: a member link that names no creature is found by its kind, not its words.
        // A plain fault with the old words on that row is a row fault like any other.
        use splat::content::layout::{Flag, FlagKind};
        let mut flags = draft.flags.clone();
        for flag in &mut flags {
            flag.message = "Other words".into();
        }
        flags.push(Flag::of(
            FlagKind::CreatureNotFound,
            14,
            "Other words".into(),
        ));
        let unread = splat::content::list_formats::unread_group_members(&draft.source, &flags);
        assert_eq!(
            unread.iter().map(|m| m.count).collect::<Vec<_>>(),
            [Some(7)]
        );
        flags.pop();
        flags.push(Flag::new(
            14,
            "Creature [[Scout]] not found or ambiguous".into(),
        ));
        let unread = splat::content::list_formats::unread_group_members(&draft.source, &flags);
        assert_eq!(
            unread.iter().map(|m| m.count).collect::<Vec<_>>(),
            [Some(7), Some(2)]
        );
        assert_eq!(
            fs::read_to_string(dir.path().join("Broken row.md")).unwrap(),
            source
        );
        fs::write(
            dir.path().join("Broken row.md"),
            source
                .replace("| 2 | [[Scout]] | | | |\n", "")
                .replace("| 1 | [[Lost scout]] | Picket | | |\n", ""),
        )
        .unwrap();
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let group = snapshot
            .groups
            .iter()
            .find(|group| group.name == "Broken row")
            .expect("a group with only unread members stays visible");
        // iter-23 FH3: nobody can join, so the summary ends at the count.
        assert!(
            group.summary().ends_with("0 of 7 can join"),
            "{}",
            group.summary()
        );
        assert!(!group.summary().contains("XP") && !group.summary().contains("Will add"));
        assert!(group.problem.is_some());
    }

    #[test]
    fn a_group_saves_reopens_and_keeps_its_notes_and_order() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let guard = creature(dir.path(), "Guard");
        let mut draft = GroupDraft::new();
        draft.name = "Rival team".into();
        draft.setup.side = "rivals".into();
        draft.notes.replace("They hold the east gate.");
        draft.add(scout);
        draft.add(scout);
        draft.add(guard);
        assert_eq!(draft.setup.members[0].count, 2, "one more of the same");
        draft.setup.members[1].overrides.insert(
            "max_hp".into(),
            serde_yaml_ng::from_str("{add: 10}").unwrap(),
        );
        draft.save(&campaign, &[]).unwrap();
        assert!(!draft.dirty());
        let reopened = GroupDraft::open(dir.path(), &draft.path).unwrap();
        assert_eq!(reopened.setup, draft.setup);
        assert_eq!(reopened.notes.as_str(), "They hold the east gate.");
        assert_eq!(reopened.name, "Rival team");
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let group = &snapshot.groups[0];
        assert!(group.problem.is_none(), "{:?}", group.problem);
        assert_eq!(group.xp, 150, "XP counts each creature, once for each");
        assert!(group.xp_unknown.is_empty());
        assert_eq!(group.side, "rivals");
    }

    #[test]
    fn an_edit_keeps_what_the_editor_does_not_change() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let mut draft = GroupDraft::new();
        draft.notes.replace("First words.");
        draft.add(scout);
        draft.save(&campaign, &[]).unwrap();
        let path = dir.path().join(&draft.path);
        let saved = fs::read_to_string(&path).unwrap();
        let external = saved.replace("First words.", "First words.\n\n## Kept\nBy hand.");
        fs::write(&path, &external).unwrap();
        let mut draft = GroupDraft::open(dir.path(), &draft.path).unwrap();
        draft.name = "Renamed".into();
        draft.save(&campaign, &[]).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(text.contains("## Kept\nBy hand."), "{text}");
        draft.notes.replace("Other words.");
        draft.save(&campaign, &[]).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(
            text.contains("Other words.") && !text.contains("Kept"),
            "{text}"
        );
        assert_eq!(
            GroupDraft::open(dir.path(), &draft.path).unwrap().name,
            "Renamed"
        );
    }

    #[test]
    fn an_empty_group_or_a_missing_creature_cannot_be_saved() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let mut draft = GroupDraft::new();
        let error = draft.save(&campaign, &[]).unwrap_err();
        assert!(error.contains("empty"), "{error}");
        draft.add(Uuid::new_v4());
        let error = draft.save(&campaign, &[]).unwrap_err();
        assert_eq!(error, "Unknown creature: Not in this campaign’s packages.");
        assert!(!dir.path().join(&draft.path).exists());

        // E2, R37: a missing part never hides a change that cannot stand. A blank name does
        // not end the check before the member's fault is read, and the save row says the
        // refusal, not a new draft's `New`.
        use design::save_row::{Changed, DraftState, Primary, SaveWord};
        let word = |draft: &GroupDraft, refusal: Option<&Refusal>| {
            DraftState::new(draft.origin(), Changed::Yes, Primary::Draft)
                .check(save_row_diagnosis(refusal, draft.is_empty()))
                .word()
        };
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        draft.name.clear();
        let refusal = draft.check(&snapshot, None).unwrap_err();
        assert_eq!(
            refusal.text(),
            "Unknown creature: Not in this campaign’s packages.",
            "a blank name and a member's fault: the member's fault"
        );
        assert!(matches!(word(&draft, Some(&refusal)), SaveWord::Invalid(_)));
        // An empty new group whose name another group has: the taken name, not `New`.
        let scout = creature(dir.path(), "Scout");
        let mut other = GroupDraft::new();
        other.name = "Ash Pack".into();
        other.add(scout);
        other.save(&campaign, &[]).unwrap();
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let mut empty = GroupDraft::new();
        empty.name = "Ash Pack".into();
        let refusal = empty.check(&snapshot, None).unwrap_err();
        assert!(
            refusal.text().starts_with("Another group is named"),
            "{}",
            refusal.text()
        );
        assert!(matches!(word(&empty, Some(&refusal)), SaveWord::Invalid(_)));
        // With a name of its own it is a new group with only its name: `New`.
        empty.name = "Ember Pack".into();
        let refusal = empty.check(&snapshot, None).unwrap_err();
        assert_eq!(word(&empty, Some(&refusal)), SaveWord::New);

        // N1: in an encounter, a changed group's refusal is not hidden by another, new and
        // empty group: the encounter says Invalid, not New. The new group alone is New.
        let encounter_word = |groups: &[GroupDraft]| {
            let super::super::OwnedGroupsFound { found, origin, .. } =
                super::super::owned_groups_found(groups, &snapshot, Uuid::new_v4());
            DraftState::new(
                origin.unwrap_or(design::save_row::DraftOrigin::Saved),
                Changed::Yes,
                Primary::Draft,
            )
            .check(found)
            .word()
        };
        let mut faulty = GroupDraft::new();
        faulty.name = "Picket".into();
        faulty.add(Uuid::new_v4());
        let mut fresh = GroupDraft::new();
        fresh.name = "Ash Hounds".into();
        assert_eq!(encounter_word(std::slice::from_ref(&fresh)), SaveWord::New);
        // R41 (s30 B11): its member's creature is not in the packages, a thing that is gone, so
        // `Invalid` takes the warning tone that the member's row says it in.
        assert!(matches!(
            encounter_word(&[faulty, fresh]),
            SaveWord::Invalid(design::save_row::InvalidCause::Missing)
        ));
        // Lead ruling 1 (s30 B11 F1): the most severe fault leads, in either order: inside a
        // group (a count that cannot stand, an error, over a creature not in the packages, a
        // warning) and across the encounter's groups, whose footer and cause come with it.
        let missing_member = |name: &str| {
            let mut group = GroupDraft::new();
            group.name = name.into();
            group.add(Uuid::new_v4());
            group
        };
        let refused_member = |name: &str| {
            let mut group = missing_member(name);
            group.setup.members[0].count = 0;
            group
        };
        let mut pair = missing_member("Pair");
        pair.add(Uuid::new_v4());
        pair.setup.members[1].count = 0;
        for _ in 0..2 {
            let refusal = pair.check(&snapshot, None).unwrap_err();
            assert_eq!(refusal.tone(), Tone::Error, "{refusal:?}");
            assert!(refusal.text().contains("Count must be"), "{refusal:?}");
            pair.setup.members.reverse();
        }
        for groups in [
            [missing_member("Picket"), refused_member("Vanguard")],
            [refused_member("Vanguard"), missing_member("Picket")],
        ] {
            let found = super::super::owned_groups_found(&groups, &snapshot, Uuid::new_v4());
            assert_eq!(
                found.found.cause(),
                Some(design::save_row::InvalidCause::Refused)
            );
            assert!(
                found
                    .cause
                    .as_deref()
                    .is_some_and(|cause| cause.starts_with("Group Vanguard:")),
                "{:?}",
                found.cause
            );
            assert!(
                matches!(&found.footer, Some(super::super::EncounterFault::Failed(words)) if words.starts_with("Group Vanguard:")),
                "{:?}",
                found.footer
            );
        }

        // F4: no editor opens a group whose note does not read: open gives an error.
        let unreadable = dir.path().join(FOLDER).join("Broken.md");
        fs::create_dir_all(unreadable.parent().unwrap()).unwrap();
        fs::write(&unreadable, "---\nsplat_kind: [encounter_group\n---\n").unwrap();
        assert!(GroupDraft::open(dir.path(), &format!("{FOLDER}/Broken.md")).is_err());
        fs::write(&unreadable, [0xff_u8, 0xfe, 0x00]).unwrap();
        assert!(GroupDraft::open(dir.path(), &format!("{FOLDER}/Broken.md")).is_err());
        fs::remove_file(&unreadable).unwrap();

        // F5 (B3b): the edit generation that keys the owned-group check counts this thread's
        // group revisions only; a group draft made and edited on another thread (another app
        // in the same process) leaves it as it was.
        let before = revisions_issued();
        std::thread::spawn(|| {
            let mut other = GroupDraft::new();
            other.touch();
        })
        .join()
        .unwrap();
        assert_eq!(
            revisions_issued(),
            before,
            "another thread's drafts do not change it"
        );
        GroupDraft::new().touch();
        assert!(revisions_issued() > before, "this thread's do");

        // E3: the origin is the group's own fact. A saved group whose file is lost, whose
        // lost file the DM accepted (the comparison text goes, as `disk_choice` does), and
        // whose write then fails, is still a saved group: emptied, it says `Invalid`.
        let mut saved = GroupDraft::open(&campaign.path, &other.path).unwrap();
        fs::remove_file(dir.path().join(&saved.path)).unwrap();
        saved.baseline = None;
        let folder = dir.path().join(FOLDER);
        fs::remove_dir_all(&folder).unwrap();
        fs::write(&folder, "not a folder").unwrap();
        assert!(saved.write(&campaign, "text").is_err(), "the write fails");
        saved.setup.members.clear();
        saved.touch();
        let refusal = saved.check(&snapshot, None).unwrap_err();
        assert_eq!(
            word(&saved, Some(&refusal)),
            SaveWord::Invalid(design::save_row::InvalidCause::Incomplete)
        );
    }

    #[test]
    fn a_bad_override_names_its_member_and_saves_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let mut draft = GroupDraft::new();
        draft.add(scout);
        draft.setup.members[0].overrides.insert(
            "max_hp".into(),
            serde_yaml_ng::from_str("{add: -10}").unwrap(),
        );
        let error = draft.save(&campaign, &[]).unwrap_err();
        assert!(error.starts_with("Scout: Maximum HP"), "{error}");
        draft.setup.members[0].overrides.clear();
        draft.setup.members[0]
            .overrides
            .insert("level".into(), 3.into());
        let error = draft.save(&campaign, &[]).unwrap_err();
        assert!(error.contains("Unknown override level"), "{error}");
        assert!(!dir.path().join(&draft.path).exists());
    }

    #[test]
    fn iter_1_b_available_members_join_a_partly_unavailable_group() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let source = format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Patrol\nsplat_side: enemies\n---\n\n## Members\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| 1 | [[Scout]] | | | |\n| 1 | [[Missing]] | Lookout | | |\n| 1 | [[Scout]] | Weak scout | HP -100 | |\n",
            Uuid::new_v4()
        );
        fs::write(dir.path().join("Patrol.md"), &source).unwrap();
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let group = &snapshot.groups[0];
        assert!(group.problem.is_none(), "{:?}", group.problem);
        assert_eq!(group.members.len(), 1);
        assert_eq!(group.members[0].entry.id, scout);
        assert_eq!(group.count_label(), "1 of 3 can join");
        assert_eq!(group.unavailable, 2);
        assert_eq!(group.warnings.len(), 2);
        assert!(
            group
                .warnings
                .iter()
                .any(|warning| warning.contains("Lookout"))
        );
        assert!(
            group
                .warnings
                .iter()
                .any(|warning| warning.contains("Weak scout"))
        );
        // iter-22-B FT1: the check that finds each fault decides its kind, and so its tone.
        let kinds: Vec<_> = group
            .warnings
            .iter()
            .map(|warning| (warning.kind, warning.tone() == design::Tone::Error))
            .collect();
        assert_eq!(
            kinds,
            [
                (FaultKind::NotInPackages, false),
                (FaultKind::InvalidChange, true)
            ],
            "{:?}",
            group.warnings
        );

        // The same invalid member that Arrivals leaves behind stops the editor's Save.
        let mut draft = GroupDraft::open(dir.path(), "Patrol.md").unwrap();
        let refusal = draft.check(&snapshot, Some(draft.id)).unwrap_err();
        let invalid = draft.setup.members[2].id.0;
        assert_eq!(refusal.place(), RefusalPlace::Member(invalid));
        assert!(
            refusal
                .text()
                .starts_with("Weak scout (as Scout): Maximum HP")
        );
        assert!(draft.save(&campaign, &[]).is_err());
        assert_eq!(
            fs::read_to_string(dir.path().join("Patrol.md")).unwrap(),
            source
        );

        assert_eq!(group.member_summary(), "Will add: 1 × Scout");
        assert_eq!(group.xp_label(), "XP 50");
        // iter-23 FC3: the heading's XP is for the one who can join, and it says so.
        assert_eq!(
            members_heading(&draft, &snapshot),
            format!("1 can join · {}", group.xp_label())
        );
    }

    #[test]
    fn a_group_read_from_disk_says_why_it_cannot_be_added() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = Uuid::new_v4();
        write_note(
            dir.path(),
            "Stray.md",
            None,
            &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: encounter_group\nsplat_name: Stray\nsplat_encounter_group:\n  side: enemies\n  members:\n  - creature: {{id: {id}}}\n", Uuid::new_v4()), "").expect("schema-2 fixture"),
        )
        .unwrap();
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let problem = snapshot.groups[0].problem.clone().unwrap();
        assert!(!problem.contains(&id.to_string()), "{problem}");
        // FA1: the one member-fault form, with the member's line in the note.
        assert!(
            problem
                .starts_with("Unknown creature · line\u{a0}13: Not in this campaign’s packages."),
            "{problem}"
        );
        assert_eq!(snapshot.groups[0].member_summary(), "Will add: none");
        assert!(
            problem.contains("Not in this campaign’s packages"),
            "{problem}"
        );
        assert!(snapshot.issues.iter().any(|issue| issue.contains("Stray")));
    }

    #[test]
    fn missing_group_members_keep_display_names_without_exposing_keys() {
        for (extra, name) in [
            ("", "Unknown creature"),
            (", link: '[[missing:goblin_boss]]'", "Unknown creature"),
            (
                ", link: '[[00000000-0000-4000-8000-000000000001]]'",
                "Unknown creature",
            ),
            (
                ", link: '[[Creatures/Goblin Boss|Goblin Boss]]'",
                "Goblin Boss",
            ),
        ] {
            let dir = tempfile::tempdir().unwrap();
            let campaign = campaign(dir.path());
            write_note(dir.path(), "Stray.md", None, &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: encounter_group\nsplat_name: Stray\nsplat_encounter_group:\n  side: enemies\n  members:\n  - creature: {{key: 'missing:goblin_boss'{extra}}}\n    count: 2\n", Uuid::new_v4()), "").expect("schema-2 fixture")).unwrap();
            let snapshot = Snapshot::load(&campaign, &[]).unwrap();
            let group = &snapshot.groups[0];
            let problem = group.problem.as_ref().unwrap();
            assert!(problem.contains(name), "{problem}");
            assert!(!problem.contains("missing:goblin_boss"), "{problem}");
            assert!(
                !problem.contains("00000000-0000-4000-8000-000000000001"),
                "{problem}"
            );
            assert_eq!(group.member_summary(), "Will add: none");
            assert_eq!(group.unavailable_members, vec![(0, 2, name.to_owned())]);
            assert_eq!(group.creature_count(), 2);
        }
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = Uuid::new_v4();
        write_note(dir.path(), "Stray.md", None, &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: encounter_group\nsplat_name: Stray\nsplat_encounter_group:\n  side: enemies\n  members:\n  - creature: {{id: {id}}}\n    overrides: {{name: Captain Varra}}\n", Uuid::new_v4()), "").expect("schema-2 fixture")).unwrap();
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let group = &snapshot.groups[0];
        assert!(
            group
                .problem
                .as_ref()
                .unwrap()
                .starts_with("Captain Varra · line\u{a0}13: Not in this campaign’s packages.")
        );
        assert!(!group.problem.as_ref().unwrap().contains(&id.to_string()));
        assert_eq!(group.member_summary(), "Will add: none");
        assert_eq!(
            group.unavailable_members,
            vec![(0, 1, "Captain Varra".into())]
        );
    }

    fn encounter(root: &std::path::Path, name: &str) -> Uuid {
        let id = Uuid::new_v4();
        write_note(
            root,
            &format!("{name}.md"),
            None,
            &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {id}\nsplat_kind: encounter\nsplat_name: {name}\nsplat_encounter:\n  roster: []\n  sides: {{}}\n  assumptions: []\n  completion: {{mode: manual}}\n  limits: {{rounds: 1, resolution_events: 1}}\n"), "").expect("schema-2 fixture"),
        )
        .unwrap();
        id
    }

    #[test]
    fn a_group_can_belong_to_an_encounter_and_go_back_to_any() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let arena = encounter(dir.path(), "Arena");
        let mut draft = GroupDraft::for_encounter(arena);
        draft.add(scout);
        draft.save(&campaign, &[]).unwrap();
        let text = fs::read_to_string(dir.path().join(&draft.path)).unwrap();
        assert!(text.contains(&arena.to_string()), "{text}");
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        assert_eq!(snapshot.groups[0].encounter, Some(arena));
        assert_eq!(snapshot.groups[0].encounter_name.as_deref(), Some("Arena"));
        assert!(snapshot.issues.is_empty(), "{:?}", snapshot.issues);
        assert_eq!(snapshot.encounter_choices().next(), Some((arena, "Arena")));

        let mut reopened = GroupDraft::open(dir.path(), &draft.path).unwrap();
        assert_eq!(reopened.setup.encounter, Some(EntryRef::id(arena)));
        assert!(!reopened.dirty());
        reopened.setup.encounter = None;
        assert!(reopened.dirty(), "a change of owner needs saving");
        reopened.save(&campaign, &[]).unwrap();
        let text = fs::read_to_string(dir.path().join(&draft.path)).unwrap();
        assert!(
            !text.contains("encounter:"),
            "no field for any encounter: {text}"
        );
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        assert_eq!(snapshot.groups[0].encounter, None);
        assert_eq!(snapshot.groups[0].encounter_name, None);
    }

    #[test]
    fn a_group_name_ends_at_its_column_or_clearly_before_it() {
        // iter-19-B-FD3: a column just past FORM_WIDTH would give a near miss. Layout step 6: the
        // rule is the shared draft head's (`save_row::name_width`), for both editors.
        use design::save_row::name_width;
        let near = design::FORM_WIDTH + design::XL - 1.0;
        assert_eq!(
            name_width(near),
            near,
            "within XL, the name ends at the column"
        );
        let far = design::FORM_WIDTH + design::XL;
        assert_eq!(
            name_width(far),
            design::FORM_WIDTH,
            "past XL, it ends at FORM_WIDTH"
        );
        assert_eq!(name_width(300.0), 300.0, "a narrow column bounds the name");
        assert_eq!(
            name_width(f32::INFINITY),
            design::FORM_WIDTH,
            "no column yet"
        );
    }

    #[test]
    fn a_group_whose_encounter_is_gone_stays_usable_and_says_so() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let arena = encounter(dir.path(), "Arena");
        let mut draft = GroupDraft::for_encounter(arena);
        draft.add(scout);
        draft.save(&campaign, &[]).unwrap();
        fs::remove_file(dir.path().join("Arena.md")).unwrap();
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let group = &snapshot.groups[0];
        assert_eq!(
            group.encounter,
            Some(arena),
            "a run of it still offers it: {}",
            fs::read_to_string(dir.path().join(&draft.path)).unwrap()
        );
        assert_eq!(group.encounter_name, None);
        assert!(group.problem.is_none(), "{:?}", group.problem);
        assert!(
            snapshot
                .issues
                .iter()
                .any(|issue| issue.contains(&draft.name)),
            "{:?}",
            snapshot.issues
        );

        // The editor will not write the same broken reference again, and says what to do.
        let mut reopened = GroupDraft::open(dir.path(), &draft.path).unwrap();
        reopened.name = "Renamed".into();
        let error = reopened.save(&campaign, &[]).unwrap_err();
        assert_eq!(error, GONE_ENCOUNTER);
        // G122: one message, under Belongs to, says the cause and the repair; the footer does
        // not say the refusal again.
        assert!(
            error.contains("deleted") && error.contains(ANY_ENCOUNTER),
            "{error}"
        );
        assert_eq!(
            reopened.check(&snapshot, None),
            Err(Refusal::missing(GONE_ENCOUNTER, RefusalPlace::BelongsTo))
        );
        // The keyboard's Save goes the same way as the blocked button: the refusal starts the
        // watch and the footer stays empty. A check after the next edit keeps it so.
        let ctx = egui::Context::default();
        let mut encounters = Encounters {
            snapshot: Some(snapshot),
            group_draft: Some(reopened.clone()),
            ..Default::default()
        };
        encounters.save_group_draft(&ctx, &campaign, &[]);
        assert!(encounters.error.is_none(), "{:?}", encounters.error);
        assert_eq!(
            encounters
                .validation_error
                .as_ref()
                .map(|invalid| invalid.text.as_str()),
            Some(GONE_ENCOUNTER)
        );
        encounters.group_draft.as_mut().unwrap().name = "Renamed again".into();
        encounters.refresh_validation_error(true, true);
        assert!(encounters.error.is_none(), "{:?}", encounters.error);
        reopened.setup.encounter = None;
        reopened.save(&campaign, &[]).unwrap();
    }

    #[test]
    fn a_link_in_the_note_stays_when_the_owner_does_not_change() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let arena = encounter(dir.path(), "Arena");
        write_note(
            dir.path(),
            "Linked.md",
            None,
            &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: encounter_group\nsplat_name: Linked\nsplat_encounter_group:\n  side: enemies\n  encounter: {{id: {arena}, link: '[[Arena]]'}}\n  members:\n  - creature: {{id: {scout}}}\n", Uuid::new_v4()), "").expect("schema-2 fixture"),
        )
        .unwrap();
        let mut draft = GroupDraft::open(dir.path(), "Linked.md").unwrap();
        draft.name = "Renamed".into();
        draft.save(&campaign, &[]).unwrap();
        let text = fs::read_to_string(dir.path().join("Linked.md")).unwrap();
        assert!(text.contains("[[Arena]]"), "{text}");
    }

    /// A new encounter with one new group of one scout.
    fn encounter_with_a_group(scout: Uuid) -> Draft {
        let mut draft = Draft::new();
        draft.name = "Arena".into();
        let mut group = GroupDraft::for_encounter(draft.id);
        group.name = "Rivals".into();
        group.add(scout);
        draft.groups.push(group);
        draft
    }

    #[test]
    fn a_group_built_in_a_new_encounter_saves_with_it() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let mut draft = encounter_with_a_group(scout);
        assert!(draft.dirty());
        draft.save(&campaign, &[]).unwrap();
        assert!(!draft.dirty(), "the encounter and its group are both saved");

        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        assert!(snapshot.issues.is_empty(), "{:?}", snapshot.issues);
        assert_eq!(snapshot.groups[0].name, "Rivals");
        assert_eq!(snapshot.groups[0].encounter, Some(draft.id));
        assert_eq!(snapshot.groups[0].encounter_name.as_deref(), Some("Arena"));

        let mut reopened = Draft::open(dir.path(), &draft.path).unwrap();
        assert!(!reopened.groups_read);
        reopened.read_groups(dir.path(), &snapshot).unwrap();
        assert_eq!(reopened.groups.len(), 1);
        assert!(!reopened.dirty());
    }

    #[test]
    fn groups_saved_together_cannot_both_take_one_name_for_any_encounter() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let mut draft = Draft::new();
        let group = |name: &str| {
            let mut group = GroupDraft::new();
            group.name = name.into();
            group.add(scout);
            group
        };
        draft.groups = vec![group("Rivals"), group(" RIVALS ")];
        let error = draft.save(&campaign, &[]).unwrap_err();
        assert!(error.contains("needs a name of its own"), "{error}");
        assert!(!dir.path().join(&draft.path).exists(), "nothing is written");
        // Nor can one take the name of the encounter saved with it.
        draft.name = "Ford".into();
        draft.groups = vec![group("ford")];
        let error = draft.save(&campaign, &[]).unwrap_err();
        assert!(error.contains("needs a name of its own"), "{error}");
        // Two that belong to the encounter can share one.
        let own = |name: &str| {
            let mut group = GroupDraft::for_encounter(draft.id);
            group.name = name.into();
            group.add(scout);
            group
        };
        let both = vec![own("Rivals"), own("Rivals")];
        draft.groups = both;
        draft.save(&campaign, &[]).unwrap();
    }

    #[test]
    fn a_group_that_cannot_be_saved_stops_the_whole_save_and_names_itself() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let mut draft = Draft::new();
        let mut group = GroupDraft::for_encounter(draft.id);
        group.name = "Rivals".into();
        draft.groups.push(group);
        let error = draft.save(&campaign, &[]).unwrap_err();
        assert!(
            error.contains("Group Rivals: This group is empty"),
            "{error}"
        );
        assert!(!dir.path().join(&draft.path).exists(), "no encounter note");
        assert!(draft.baseline.is_none());
        assert!(draft.dirty(), "the DM's work is still there");
    }

    #[test]
    fn a_change_to_a_group_alone_leaves_the_encounter_note_as_it_is() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let mut draft = encounter_with_a_group(scout);
        draft.save(&campaign, &[]).unwrap();
        let note = dir.path().join(&draft.path);
        // A form the app would not write, so a rewrite of the note would show.
        let by_hand = format!("{}\nKept by hand.\n", fs::read_to_string(&note).unwrap());
        fs::write(&note, &by_hand).unwrap();
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let mut draft = Draft::open(dir.path(), &draft.path).unwrap();
        draft.read_groups(dir.path(), &snapshot).unwrap();

        draft.groups[0].add(scout);
        assert!(draft.dirty(), "a group's change is the encounter draft's");
        draft.save(&campaign, &[]).unwrap();
        assert!(!draft.dirty());
        assert_eq!(fs::read_to_string(&note).unwrap(), by_hand);
        let reopened = GroupDraft::open(dir.path(), draft.groups[0].path()).unwrap();
        assert_eq!(reopened.setup.members[0].count, 2);
    }

    #[test]
    fn a_group_given_to_another_owner_leaves_the_encounter_when_it_saves() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let mut draft = encounter_with_a_group(scout);
        draft.save(&campaign, &[]).unwrap();
        let path = draft.groups[0].path().to_owned();
        draft.groups[0].setup.encounter = None;
        assert_eq!(draft.groups.len(), 1, "it stays in view until it is saved");
        draft.save(&campaign, &[]).unwrap();
        assert!(draft.groups.is_empty());
        assert_eq!(
            GroupDraft::open(dir.path(), &path).unwrap().setup.encounter,
            None
        );
    }

    #[test]
    fn a_group_changed_on_disk_blocks_the_whole_save_and_keeps_the_draft() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let mut draft = encounter_with_a_group(scout);
        draft.save(&campaign, &[]).unwrap();
        let saved_name = draft.name.clone();
        let note = dir.path().join(draft.groups[0].path());
        let external = format!("{}\nFrom outside.\n", fs::read_to_string(&note).unwrap());
        fs::write(&note, &external).unwrap();

        draft.name = "Renamed".into();
        draft.groups[0].name = "Changed".into();
        let error = draft.save(&campaign, &[]).unwrap_err();
        assert_eq!(
            error,
            super::super::disk::changed_on_disk("group", "Changed").to_string()
        );
        assert!(
            error.starts_with("Changed changed on disk."),
            "R10: {error}"
        );
        assert!(draft.groups[0].recovery.change.is_some());
        assert_eq!(fs::read_to_string(&note).unwrap(), external);
        assert!(draft.dirty(), "the group's change is still here to save");
        assert_eq!(
            Draft::open(dir.path(), &draft.path).unwrap().name,
            saved_name
        );

        // FF7 (Astra review of PR 259): the group's file goes. `Remove group and lose draft`
        // takes it out of this encounter's draft; an unrelated edit stays through Undo remove,
        // which puts the group back with its draft. Nothing is written.
        fs::remove_file(&note).unwrap();
        draft.groups[0].recovery = Default::default();
        assert!(draft.save(&campaign, &[]).is_err());
        assert!(draft.groups[0].file_gone());
        draft.groups_read = true;
        let mut editor = Encounters {
            // The draft's results are the campaign's (`EditorOwner`).
            active: Some(campaign.clone()),
            draft: Some(draft),
            snapshot: Some(Snapshot::load(&campaign, &[]).unwrap()),
            ..Default::default()
        };
        // Through the model the GPUI page draws: the conflict, the choice, the editor's footer.
        let ctx = egui::Context::default();
        let view = editor.encounter_disk().expect("the gone group's conflict");
        assert_eq!(view.lose, "Remove group and lose draft");
        editor.encounter_disk_choice(&ctx, &campaign, &[], false);
        let draft = editor.draft.as_ref().unwrap();
        assert!(draft.groups.is_empty(), "the gone group left the draft");
        assert!(editor.error.is_none());
        editor.draft.as_mut().unwrap().name = "Renamed again".into();
        let lines = editor.editor_footer(&ctx, &campaign.path);
        let removed = lines
            .iter()
            .find(|line| line.words == "Changed removed. Its draft was discarded.")
            .expect("the footer names the group and what was lost");
        assert_eq!(
            removed.undo.as_ref().map(|undo| undo.label.as_str()),
            Some("Undo remove Changed")
        );
        let undo = removed.press_undo().expect("its Undo");
        editor.footer_press(
            &ctx,
            undo,
            &campaign,
            &crate::campaigns::Campaigns::default(),
        );
        let draft = editor.draft.as_ref().unwrap();
        assert_eq!(draft.name, "Renamed again", "the unrelated edit stays");
        assert_eq!(draft.groups.len(), 1);
        assert_eq!(
            draft.groups[0].name, "Changed",
            "the group comes back with its draft"
        );
        assert!(!note.exists(), "nothing was written");
    }

    #[test]
    fn integrity_d8_a_copied_group_keeps_both_files_and_offers_comparison() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let mut group = GroupDraft::new();
        group.name = "Scouts".into();
        group.add(scout);
        group.save(&campaign, &[]).unwrap();
        let copy = dir.path().join("Copied group.md");
        fs::copy(dir.path().join(group.path()), &copy).unwrap();
        let bytes = fs::read(&copy).unwrap();
        group.notes.replace("My plan");
        assert!(group.save(&campaign, &[]).is_err());
        assert!(
            group
                .recovery
                .change
                .as_ref()
                .unwrap()
                .copies
                .contains(&"Copied group.md".into())
        );
        group.recovery.accept();
        group.save(&campaign, &[]).unwrap();
        assert_eq!(fs::read(copy).unwrap(), bytes);
        assert!(
            fs::read_to_string(dir.path().join(group.path()))
                .unwrap()
                .contains("My plan")
        );
    }

    #[test]
    fn the_search_adds_to_the_group_the_dm_chose_or_else_to_the_roster() {
        let scout = Uuid::new_v4();
        let mut draft = encounter_with_a_group(scout);
        let path = draft.groups[0].path().to_owned();
        draft.add_chosen(scout, false);
        assert_eq!(draft.setup.roster.len(), 1, "the roster, by default");
        assert_eq!(draft.groups[0].setup.members[0].count, 1);

        draft.add_to = Some(path);
        draft.add_chosen(scout, false);
        assert_eq!(draft.setup.roster.len(), 1);
        assert_eq!(draft.groups[0].setup.members[0].count, 2);

        // A group that is gone cannot take a creature, and the creature is not lost.
        draft.groups.clear();
        draft.add_chosen(scout, false);
        assert_eq!(draft.setup.roster.len(), 2);
    }

    #[test]
    fn xp_uses_printed_values_or_standard_cr_rewards() {
        let entry = |creature: &str, body: &str| {
            Entry::parse(
                "c.md",
                &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_kind: creature\nsplat_name: C\nsplat_creature: {creature}\n", Uuid::new_v4()), &format!("\n{body}\n")).expect("schema-2 fixture"),
            )
            .unwrap()
        };
        assert_eq!(
            creature_xp(&entry("{cr_text: '1/4 (XP 50; PB +2)'}", "")),
            Some(50)
        );
        assert_eq!(
            creature_xp(&entry(
                "{cr_text: '12 (XP 8,400, or 10,000 in lair; PB +4)'}",
                ""
            )),
            Some(8400)
        );
        assert_eq!(
            creature_xp(&entry("{}", "CR 3 (XP 700; PB +2)")),
            None,
            "Only the schema-2 Challenge row supplies CR"
        );
        assert_eq!(creature_xp(&entry("{cr_text: '3'}", "")), Some(700));
        assert_eq!(creature_xp(&entry("{}", "No such line.")), None);
        assert_eq!(
            creature_xp(&entry("{cr_text: '2 (450 XP; PB +2)'}", "")),
            Some(450),
            "the form four built-in creatures use"
        );
        assert_eq!(
            creature_xp(&entry("{cr_text: '0 (XP 0; PB +2)'}", "")),
            Some(0)
        );
    }

    #[test]
    fn a_creature_no_run_could_add_is_refused_when_the_group_is_saved() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = Uuid::new_v4();
        write_note(
            dir.path(),
            "Blank.md",
            None,
            &splat::content::formats::fixture("Fixture.md", &format!("splat_id: {id}\nsplat_kind: creature\nsplat_name: Blank\nsplat_creature: {{}}\n"), "").expect("schema-2 fixture"),
        )
        .unwrap();
        let mut draft = GroupDraft::new();
        draft.add(id);
        let error = draft.save(&campaign, &[]).unwrap_err();
        assert!(
            error == "Blank: It has no maximum HP. Set it with this member’s Edit button.",
            "{error}"
        );
        draft.setup.members[0].overrides = serde_yaml_ng::from_str("{max_hp: {set: 9}}").unwrap();
        draft.save(&campaign, &[]).unwrap();
    }
}
