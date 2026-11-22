//! Encounter groups: their notes, their editor, and the overrides form they share with the
//! roster. A group is prepared here and added to a run from the run's own panel.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{
    ADD_PANE_MAX_WIDTH, Creature, Draft, EditorPane, Encounters, ROSTER_MAX_WIDTH, Snapshot,
};
use crate::design::memo::Revised;
use crate::design::text_draft::{KeptCompare, TextDraft};
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue, EntryRef},
    design::{self, Action, Density, Tone, Type, colors, icons},
    difficulty::creature_xp,
    encounter_run::{FaultKind, GroupMember, MemberFault, RunGroup, change_rows},
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

const FOLDER: &str = "Encounter Groups";
const DEFAULT_SIDE: &str = "enemies";
const SIDE_FIELD: &str = "Default side";
const BELONGS_TO: &str = "Belongs to";
const ROSTER: &str = "Roster";
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

/// Layout step 6: the group head's two reason slots (`save_row::head_plan`): Save's own reason,
/// then the campaign-switch reason in both its forms, whose place G122's short words for a fault
/// said elsewhere can take. Each slot's room and line are measured by all of them, so neither a
/// state nor a refusal moves a field.
const HEAD_REASONS: [design::save_row::ReasonSlot<'static>; 2] = [
    design::save_row::ReasonSlot::of(&[EMPTY_REASON]),
    design::save_row::ReasonSlot {
        forms: &[
            design::save_row::SAVE_OR_DISCARD_TO_SWITCH,
            design::save_row::DISCARD_TO_SWITCH,
        ],
        substitutes: &[GONE_BESIDE_SAVE, MEMBER_BESIDE_SAVE],
        short: &[
            design::save_row::SAVE_OR_DISCARD_SHORT,
            design::save_row::DISCARD_SHORT,
        ],
    },
];
/// The Groups list's heading for groups whose encounter is gone; Arrivals says the same words
/// beside such a group's name (FB10).
pub(crate) const ENCOUNTER_DELETED: &str = "Encounter deleted";
const NOTES_FIELD: &str = "Group notes";
const LABEL_WIDTH: f32 = 120.0;
const OVERRIDE_NAME: design::FieldName = crate::field_id!("override_name");
/// The change rows' drop-downs, each keyed by its form (`overrides_form`).
const CHANGE_MAX_HP: design::FieldName = crate::field_id!("override_change_max_hp");
const CHANGE_AC: design::FieldName = crate::field_id!("override_change_ac");
const CHANGE_INITIATIVE: design::FieldName = crate::field_id!("override_change_initiative");
const CHANGE_ATTACK: design::FieldName = crate::field_id!("override_change_attack");
const CHANGE_DAMAGE: design::FieldName = crate::field_id!("override_change_damage");
const CHANGE_SPEED: design::FieldName = crate::field_id!("override_change_speed");
const OVERRIDE_NOTE: design::FieldName = crate::field_id!("override_note");
const GROUP_NAME: design::FieldName = crate::field_id!("group_name");
const ENCOUNTER_GROUP_NAME: design::FieldName = crate::field_id!("encounter_group_name");
const BELONGS: design::FieldName = crate::field_id!("group_encounter");
/// The name of a roster slot's button, for the form it opens.
pub(super) const SLOT_OPTIONS: &str =
    "Options: starting HP, temporary HP, initiative group and changes";

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
/// The standalone group's name field. Its id comes from the group's own id, which the first
/// save does not change, so the field keeps its focus and its text history through a save.
fn name_field(ui: &mut egui::Ui, draft: &mut GroupDraft, width: f32) -> egui::Response {
    let font = Type::Heading.font(ui);
    GROUP_NAME
        .of(draft.id)
        .single_line(ui, &mut draft.name, |edit| {
            edit.desired_width(width)
                .min_size(egui::vec2(0.0, Density::Scan.control()))
                .margin(egui::vec2(design::S, design::XS))
                .font(font)
        })
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
    repair: Option<(usize, Mapping)>,
    /// The words typed in a change form's field while it has focus ([`TypedWords`]).
    typed: TypedWords,
    /// M0237, M0251, M0254: the reveal of a form the DM opens here, or of the member row that
    /// `Done` closes, in the editor's column (`design::reveal`): owned by this draft.
    reveal: DraftReveal,
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
    edited_last_frame: bool,
    repair_error: Option<String>,
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

/// A fault the group editor says in its body (lane E1, ruling M20), as a typed fault of the
/// tone its check gives it (R41): an error (an invalid change, unreadable data) or a warning (a
/// member left out, a link to fix). Its words are the check's or the loader's.
fn editor_fault(tone: Tone, words: &str) -> MemberFault {
    let kind = if tone == Tone::Error {
        FaultKind::InvalidChange
    } else {
        FaultKind::CannotJoin
    };
    MemberFault::new(kind, words.to_owned())
}

/// `fault` in the body's full form (`BodyFault`, no repair): the rect it took. Every use of it
/// replaces an untyped `design::message` (ruling M20).
#[allow(
    clippy::disallowed_methods,
    reason = "ruling M20: the group editor’s faults in its body, typed (`Ban::Body`)"
)]
fn body_fault(ui: &mut egui::Ui, fault: &MemberFault) -> egui::Rect {
    let shown = ui.scope(|ui| {
        let _: Option<()> = design::outcome::BodyFault::new(fault).show(ui);
    });
    shown.response.rect
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
    /// A removed unread row: its line and its text.
    Unread {
        line: usize,
        row: String,
        name: String,
    },
}

impl MemberUndo {
    /// The result's words.
    pub(super) fn words(&self) -> String {
        match self {
            Self::Replaced(result) => result.words.clone(),
            Self::Removed { name, .. } | Self::Unread { name, .. } => format!("Removed {name}."),
        }
    }
    /// Its Undo's words.
    pub(super) fn step(&self) -> design::outcome::UndoStep {
        use design::outcome::{UndoStep, Verb};
        match self {
            Self::Replaced(_) => UndoStep::of(Verb::Replace, "member"),
            Self::Removed { .. } | Self::Unread { .. } => UndoStep::of(Verb::Remove, "member"),
        }
    }
    /// Whether its Undo still applies: a replacement's member (by its identity, which a move
    /// does not change) is in the draft and still holds the chosen creature. A removal always
    /// holds.
    pub(super) fn holds(&self, draft: &GroupDraft) -> bool {
        match self {
            Self::Replaced(result) => draft.replaced_member(result).is_some(),
            Self::Removed { .. } | Self::Unread { .. } => true,
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
            typed: TypedWords::default(),
            reveal: DraftReveal::default(),
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
            repair: None,
            member_new: None,
            results_key: super::results_key::ResultsKey::new(),
            revision: next_revision(),
            members_saved: None,
            edited_last_frame: false,
            repair_error: None,
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
            typed: TypedWords::default(),
            reveal: DraftReveal::default(),
            recovery: Default::default(),
            id: entry.id,
            path: path.into(),
            origin: design::save_row::DraftOrigin::Saved,
            baseline: Some(text.clone()),
            member_lines: splat::content::list_formats::group_member_lines(&text, &entry.flags),
            source: text,
            flags: entry.flags.clone(),
            repair: None,
            member_new: None,
            results_key: super::results_key::ResultsKey::new(),
            revision: next_revision(),
            members_saved: None,
            edited_last_frame: false,
            repair_error: None,
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
            MemberUndo::Unread { line, row, .. } => {
                let at = self
                    .source
                    .split_inclusive('\n')
                    .take(line - 1)
                    .map(str::len)
                    .sum();
                self.source.insert_str(at, row);
                if let Ok(entry) = Entry::parse(&self.path, &self.source) {
                    self.flags = entry.flags;
                }
                self.reread_lines();
            }
        }
        Ok(())
    }
    /// Whether no member has moved, been added, changed or removed since the note was read.
    /// The comparison runs only after a change (`members_saved`); each frame reaches it.
    fn members_are_saved(&mut self, ctx: &egui::Context) -> bool {
        let edited = super::edit_input(ctx);
        let edited_before = std::mem::replace(&mut self.edited_last_frame, edited);
        if edited || edited_before {
            self.members_saved = None;
        }
        #[cfg(any(test, feature = "test-support"))]
        MEMBER_COMPARISONS.with(|counts| counts.set((counts.get().0 + 1, counts.get().1)));
        *self.members_saved.get_or_insert_with(|| {
            #[cfg(any(test, feature = "test-support"))]
            MEMBER_COMPARISONS.with(|counts| counts.set((counts.get().0, counts.get().1 + 1)));
            self.setup.members == self.saved_setup.members
        })
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

    fn repair_member(
        &mut self,
        line: usize,
        changes: Option<&Mapping>,
        snapshot: &Snapshot,
    ) -> Result<(), String> {
        use splat::content::{layout::Document, list_formats};
        self.touch();
        let member = list_formats::unread_group_members(&self.source, &self.flags)
            .into_iter()
            .find(|member| member.flag.line == line)
            .ok_or("This row is no longer flagged.")?;
        let table = Document::new(&self.source)
            .table(list_formats::GROUP)
            .ok_or("Member table not found")?;
        let row = table
            .rows
            .iter()
            .find(|row| row[0].line == line)
            .ok_or("Member row not found")?;
        let mut source = self.source.clone();
        if let Some(changes) = changes {
            Overrides::parse(changes)?;
            for (cell, value) in [
                (
                    4,
                    changes
                        .get("note")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_owned(),
                ),
                (
                    3,
                    list_formats::show_changes(&Value::Mapping(changes.clone())),
                ),
                (
                    2,
                    changes
                        .get("name")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_owned(),
                ),
            ] {
                source.replace_range(row[cell].range.clone(), &value.replace('|', "\\|"));
            }
        } else {
            let start = source[..row[0].range.start]
                .rfind('\n')
                .map_or(0, |at| at + 1);
            let end = source[row[0].range.start..]
                .find('\n')
                .map_or(source.len(), |at| row[0].range.start + at + 1);
            // One result line: the newest member result takes it (the store replaces).
            self.member_new = Some(MemberUndo::Unread {
                line,
                row: source[start..end].to_owned(),
                name: if member.name.is_empty() {
                    member.label.clone()
                } else {
                    member.name.clone()
                },
            });
            source.replace_range(start..end, "");
        }
        let mut entry = Entry::parse(&self.path, &source)?;
        let entries: Vec<_> = snapshot
            .catalogue
            .entries
            .values()
            .map(|entry| entry.entry.clone())
            .collect();
        list_formats::resolve(&mut entry, &entries, &source);
        if changes.is_some() {
            if let Some(flag) = entry.flags.iter().find(|flag| flag.line == line) {
                return Err(flag.words().unwrap_or_else(|| flag.message.clone()));
            }
            let setup: GroupSetup =
                serde_yaml_ng::from_value(entry.fields["splat_encounter_group"].clone())
                    .map_err(|error| error.to_string())?;
            self.setup
                .members
                .insert(member.before, setup.members[member.before].clone());
        }
        self.source = source;
        self.flags = entry.flags;
        self.reread_lines();
        self.repair = None;
        self.repair_error = None;
        Ok(())
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
    pub(super) fn disk_choice(
        &mut self,
        ui: &mut egui::Ui,
        root: &std::path::Path,
        snapshot: &Snapshot,
    ) -> Result<Option<bool>, String> {
        let choice =
            self.recovery
                .show(ui, &self.comparison_text(snapshot)?, ("group", &self.name));
        if let Some(keep) = choice {
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
        }
        Ok(choice)
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

/// A member's fault in its one form (`member_fault`), as a typed fault: its words whole, in the
/// tone its kind gives (R41): an error is `CannotRead`, a warning `NeedsFixing`.
struct MemberFaultWords {
    words: String,
    severity: design::outcome::Severity,
}

impl MemberFaultWords {
    fn new(words: String, tone: Tone) -> Self {
        let severity = if tone == Tone::Error {
            design::outcome::Severity::CannotRead
        } else {
            design::outcome::Severity::NeedsFixing
        };
        Self { words, severity }
    }
}

impl design::outcome::Fault for MemberFaultWords {
    fn severity(&self) -> design::outcome::Severity {
        self.severity
    }
    fn place(&self) -> Option<design::outcome::FaultPlace<'_>> {
        None
    }
    fn cause(&self) -> design::outcome::Sentence {
        super::outcome_words(&self.words)
    }
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

/// Group actions use the encounter ledger's columns (layout PR 5a).
fn list_row(
    ui: &mut egui::Ui,
    group: &RunGroup,
    blocked: Option<&'static str>,
    column: super::list::Column,
    action: &mut Option<super::ListAction>,
) {
    super::list::list_row(
        ui,
        group.id,
        &group.name,
        &row_detail(group),
        false,
        None,
        column,
        super::list::group_actions(ui),
        |ui| {
            super::row_menu(ui, &group.name, |ui| {
                if super::menu_action(ui, "Remove", blocked) {
                    *action = Some(super::ListAction::RemoveGroup(group.path.clone()));
                }
            });
            if Action::quiet("Edit")
                .density(Density::Read)
                .name(&format!("Edit group {}", group.name))
                .blocked(blocked)
                .show(ui)
                .clicked()
            {
                *action = Some(super::ListAction::OpenGroup(group.path.clone()));
            }
        },
    );
    if !group.warnings.is_empty() {
        ui.label(Type::Small.rich(ui, "Cannot join").color(colors().muted));
    }
    // The row's own Edit keeps the editor one click away when a fault points to Packages.
    for warning in &group.warnings {
        match member_fault_row(ui, warning, MemberFaultView::EncounterList) {
            Some(MemberRepair::Packages) => {
                ui.data_mut(|data| data.insert_temp(egui::Id::new("open_campaign_packages"), true));
            }
            Some(MemberRepair::EditGroup) => {
                *action = Some(super::ListAction::OpenGroup(group.path.clone()));
            }
            None => {}
        }
    }
    // A group with no member that can join says so after its member faults, as a message in
    // the same form as the faults above it.
    for reason in group.own_problems() {
        body_fault(ui, &editor_fault(Tone::Warning, reason));
    }
}

/// One flat set of group headings inside the ledger's Groups section.
pub(super) fn list(
    ui: &mut egui::Ui,
    snapshot: &Snapshot,
    blocked: Option<&'static str>,
    column: super::list::Column,
    action: &mut Option<super::ListAction>,
) {
    // FB8: a sub-heading after a group has its heading's space above it, so it reads as the
    // start of its own groups, not the end of the ones above. The first follows the Groups
    // heading as it is.
    let sub_heading = |ui: &mut egui::Ui, title: &str, count: usize| {
        design::heading(ui, design::Heading::Group { title, count });
    };
    let any: Vec<_> = snapshot
        .groups
        .iter()
        .filter(|group| group.encounter.is_none())
        .collect();
    if !any.is_empty() {
        sub_heading(ui, "For any encounter", any.len());
        for group in any {
            list_row(ui, group, blocked, column, action);
        }
    }
    for (id, name) in snapshot.encounter_choices() {
        let own: Vec<_> = snapshot
            .groups
            .iter()
            .filter(|group| group.encounter == Some(id))
            .collect();
        if !own.is_empty() {
            sub_heading(ui, &format!("For {name}"), own.len());
            for group in own {
                list_row(ui, group, blocked, column, action);
            }
        }
    }
    let orphans: Vec<_> = snapshot
        .groups
        .iter()
        .filter(|group| group.encounter.is_some() && group.encounter_name.is_none())
        .collect();
    if !orphans.is_empty() {
        sub_heading(ui, ENCOUNTER_DELETED, orphans.len());
        for group in orphans {
            list_row(ui, group, blocked, column, action);
        }
    }
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
    fn group_disk_view(
        &mut self,
        ui: &mut egui::Ui,
        campaign: &Campaign,
        modules: &[Module],
    ) -> bool {
        let Some(snapshot) = self.snapshot.as_ref() else {
            return false;
        };
        let Some(draft) = self.group_draft.as_mut() else {
            return false;
        };
        if draft.recovery.change.is_none() {
            return false;
        }
        if let Some(keep) = draft.recovery.show(
            ui,
            &draft.comparison_text(snapshot).unwrap_or_default(),
            ("group", &draft.name),
        ) {
            if keep {
                if let Some(change) = draft.recovery.accept() {
                    draft.path = change.path;
                    draft.baseline = change.text.clone();
                    if let Some(text) = change.text {
                        draft.source = text;
                        draft.reread_lines();
                    }
                    draft.touch();
                    self.save_group_draft(ui.ctx(), campaign, modules);
                }
            } else {
                let path = draft.path.clone();
                self.discarded_group = self.group_draft.take();
                self.stamp_kept(super::KeptResult::GroupDiscarded);
                self.group_draft = GroupDraft::open(&campaign.path, &path).ok();
                self.error = None;
            }
        }
        true
    }
}

/// Which the search adds to: the roster, or one of this encounter's groups.
pub(super) fn add_target(ui: &mut egui::Ui, draft: &mut Draft, _snapshot: &Snapshot) {
    if draft.groups.is_empty() {
        return;
    }
    // Beside the search, because the group itself can be out of view or on another tab.
    let shown = draft.target_group().map_or_else(
        || format!("the {}", ROSTER.to_lowercase()),
        |group| group.name.clone(),
    );
    ui.label(
        Type::Small
            .rich(ui, format!("Adding to {shown}"))
            .color(colors().muted),
    );
}

/// The groups that belong to an encounter, each in a frame of its own, built here and saved
/// with it. Returns true when the DM chooses to add creatures to one.
pub(super) fn of_encounter(
    ui: &mut egui::Ui,
    draft: &mut Draft,
    snapshot: &Snapshot,
    focus_search: &mut bool,
    expanded: &mut Option<String>,
) -> bool {
    if !draft.groups_read {
        return false;
    }
    let mut go_to_search = false;
    let mut discard = None;
    let this = (draft.id, draft.name.clone());
    design::heading(
        ui,
        design::Heading::Group {
            title: "Groups for this encounter",
            count: draft
                .groups
                .iter()
                .filter(|group| group.belongs_to(draft.id))
                .count(),
        },
    );
    design::help(
        ui,
        "They arrive when you add them to a run of this encounter. Save encounter saves them too.",
    );
    // `XS` sets the first group's card apart from the help. With no group, New group follows the
    // help as any action under its text does (B4 review F2: at 1920 the space let the 34 px back
    // line cut the roster's last control).
    if draft.groups.iter().any(|group| group.belongs_to(draft.id)) {
        design::gap(ui, design::Gap::XS);
    }
    let reveal = std::mem::take(&mut draft.reveal_group);
    let Draft {
        groups,
        add_to,
        focus_group,
        new_results,
        deleted_groups,
        ..
    } = draft;
    for (index, group) in groups.iter_mut().enumerate() {
        if group.setup.encounter.as_ref().and_then(|owner| owner.id) != Some(this.0) {
            continue;
        }
        let path = group.path.clone();
        let adding = add_to.as_deref() == Some(path.as_str());
        // The editor's usable area: `Move to` opens inside it, clear of the scroll bar and the
        // splitter beside it.
        let inside = ui.max_rect().intersect(ui.clip_rect());
        ui.push_id(&path, |ui| {
            let stroke = if adding {
                egui::Stroke::new(design::FOCUS_WIDTH, colors().accent)
            } else {
                egui::Stroke::new(1.0, colors().rule)
            };
            // The stroke is inside the frame, so a thinner one takes a wider margin: every
            // frame's content keeps one inset whether or not it is the Add target.
            let frame = egui::Frame::new()
                .stroke(stroke)
                .corner_radius(design::RADIUS_CONTROL)
                .inner_margin(design::S + design::FOCUS_WIDTH - stroke.width)
                .show(ui, |ui| {
                    let mut name_right = f32::INFINITY;
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            super::row_menu(ui, &group.name.clone(), |ui| {
                                design::menu_submenu(ui, "Move to", inside, |ui| {
                                    for (id, name) in
                                        snapshot.encounter_choices().filter(|(id, _)| *id != this.0)
                                    {
                                        if super::menu_action(ui, name, None) {
                                            group.setup.encounter = Some(EntryRef::id(id));
                                            // N1: who owns the group is checked by Save.
                                            group.touch();
                                        }
                                    }
                                });
                                if super::menu_action(ui, "Make available to any encounter", None) {
                                    discard = Some((index, false));
                                }
                                if super::menu_action(ui, "Delete group", None) {
                                    discard = Some((index, true));
                                }
                            });
                            // The slot keeps the wider word's width, so every frame's name
                            // field ends at one edge and does not move with the Add target.
                            let slot = Action::quiet("Adding here")
                                .icon(icons::OK)
                                .density(Density::Read)
                                .natural_width(ui);
                            if Action::quiet(if adding { "Adding here" } else { "Add here" })
                                .icon(if adding { icons::OK } else { icons::ADD })
                                .density(Density::Read)
                                .min_width(slot)
                                .name(&format!("Add creatures to {}", group.name))
                                .selected(adding)
                                .show(ui)
                                .clicked()
                            {
                                if adding {
                                    *add_to = None;
                                } else {
                                    *add_to = Some(path.clone());
                                    *focus_search = true;
                                    go_to_search = true;
                                }
                            }
                            // The name starts at the frame's left edge, as the fields below do,
                            // and ends at FORM_WIDTH or before the actions.
                            ui.with_layout(
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    let width = (ui.available_width() - 2.0 * design::S)
                                        .min(design::FORM_WIDTH);
                                    let font = Type::Heading.font(ui);
                                    let response = ENCOUNTER_GROUP_NAME.of(group.id).single_line(
                                        ui,
                                        &mut group.name,
                                        |edit| {
                                            edit.desired_width(width)
                                                .margin(egui::vec2(design::S, 4.0))
                                                .font(font)
                                        },
                                    );
                                    design::named(
                                        ui,
                                        &response,
                                        &format!("Group name: {}", group.name),
                                    );
                                    // F1: a name edit is an edit of the group: its revision
                                    // keys Save's checks, as in the group's own editor.
                                    if response.changed() {
                                        group.touch();
                                    }
                                    name_right = response.rect.right();
                                    if focus_group.as_deref() == Some(path.as_str()) {
                                        *focus_group = None;
                                        design::focus_and_select(ui, response.id, &group.name);
                                        response.scroll_to_me(Some(egui::Align::Center));
                                    }
                                },
                            );
                        });
                    });
                    group_fields(
                        ui,
                        group,
                        snapshot,
                        expanded,
                        Some((this.0, &this.1)),
                        name_right,
                        None,
                    );
                });
            // M0226 (ruling M27): a creature added from the Add pane went to this card, so the
            // card comes whole into view, once, by its column's least scroll.
            if adding && reveal {
                design::reveal::bring_into_view(
                    ui,
                    frame.response.rect,
                    design::reveal::JumpTo::AddedTarget,
                );
            }
            design::gap(ui, design::Gap::S);
        });
    }
    if let Some((index, delete)) = discard {
        let gone = groups[index].clone();
        if add_to.as_deref() == Some(gone.path()) {
            *add_to = None;
        }
        if delete {
            deleted_groups.push(groups.remove(index));
        } else {
            groups[index].setup.encounter = None;
            // N1: a group for any encounter is checked against the others' names.
            groups[index].touch();
        }
        new_results.push(super::DraftUndo::RemovedGroup {
            index,
            group: Box::new(gone),
            delete,
        });
    }
    if Action::secondary("New group for this encounter")
        .icon(icons::ADD)
        .density(Density::Read)
        .show(ui)
        .clicked()
    {
        let group = GroupDraft::for_encounter(this.0);
        *add_to = Some(group.path.clone());
        *focus_group = Some(group.path.clone());
        groups.push(group);
    }
    go_to_search
}

/// Who a group is for, in its editor. An encounter being built is a choice before it is saved,
/// and under the name it has now. True when it says a gone encounter (G122), whole on screen.
#[allow(
    clippy::disallowed_methods,
    reason = "ruling M20: the gone encounter is a typed fault at its field (`Ban::Body`)"
)]
fn belongs_to(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    open: Option<(Uuid, &str)>,
    name_right: f32,
) -> bool {
    let mut choices: Vec<(Uuid, &str)> = snapshot
        .encounter_choices()
        .filter(|(id, _)| open.is_none_or(|(open, _)| open != *id))
        .collect();
    choices.extend(open);
    let current = draft
        .setup
        .encounter
        .as_ref()
        .and_then(|reference| reference.id);
    let shown = current.map_or(ANY_ENCOUNTER, |id| {
        choices
            .iter()
            .find(|(known, _)| *known == id)
            .map_or(MISSING_ENCOUNTER, |(_, name)| name)
    });
    whole_row(ui, |ui| {
        ui.horizontal(|ui| {
            choice_label(ui, BELONGS_TO, |ui| {
                // The choice ends where the name field ends, inside the column. The part's list is as
                // wide as its control at every size (FB8), and a long name wraps in it.
                let right = name_right.min(ui.max_rect().right());
                let width = (right - ui.cursor().left()).max(Density::Scan.control());
                let mut chosen = current;
                let mut dropdown = design::Dropdown::new(BELONGS.of(draft.id), BELONGS_TO)
                    .width(width)
                    .choice(None, ANY_ENCOUNTER)
                    .choices(choices.iter().map(|(id, name)| (Some(*id), *name)));
                // An encounter that is gone stays the value, as words the list cannot choose again.
                if current.is_some_and(|id| !choices.iter().any(|(known, _)| *known == id)) {
                    dropdown = dropdown.blocked_choice(
                        current,
                        MISSING_ENCOUNTER,
                        "Its encounter was deleted. Choose another.",
                    );
                }
                if dropdown.show(ui, &mut chosen).changed {
                    // The same choice again keeps a link that the note gives: only a change writes.
                    draft.setup.encounter = chosen.map(EntryRef::id);
                    draft.touch();
                }
            });
        })
    });
    // In an encounter's editor, belonging to it goes without saying.
    if current.is_some() && current == open.map(|(id, _)| id) {
        return false;
    }
    let help = match current {
        None => "A run of any encounter offers this group.",
        // A group no fight can offer is a fault, not field help.
        Some(_) if shown == MISSING_ENCOUNTER => {
            // R41: the tone of the refusal that Save gives for it, so the save row's word and
            // this message agree.
            let said = design::outcome::row_fault(
                ui,
                &editor_fault(gone_encounter().tone(), GONE_ENCOUNTER),
            );
            // Whole on screen: the save row does not say it again (`GONE_BESIDE_SAVE`).
            return ui.is_visible() && ui.clip_rect().contains_rect(said.rect);
        }
        Some(_) => "Only runs of this encounter offer this group.",
    };
    design::help(ui, help);
    false
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

/// A member's change words (`Changed`, `Loot`…) for the end of its detail line.
fn change_word(ui: &egui::Ui, overrides: &Overrides, entry: &Entry, side: &str) -> String {
    let missing = crate::loot_editor::missing_in(ui.ctx(), overrides.loot.as_deref());
    change_words(overrides, entry, side, &missing)
}

/// The muted pencil and the change words before it opens the form (FC9): `✎ Changed`, as the
/// Encounters list, the run's setup and Arrivals show a change. The words are the link.
pub(super) fn change_mark() -> String {
    format!("{}\u{a0}", icons::CHANGED)
}

/// What separates a detail line from the change link after it.
const CHANGE_SEPARATOR: &str = " · ";

/// Whether the whole change link (the pencil and `word`) fits after `detail` and its separator
/// in `width` (G135). When it does not, the link starts its own row.
fn link_fits_after(ui: &egui::Ui, detail: &str, word: &str, width: f32) -> bool {
    let font = Type::Small.font(ui);
    let link = design::number_width(ui, &format!("{}{word}", change_mark()), &font).max(design::XL);
    design::number_width(ui, detail, &font)
        + design::number_width(ui, CHANGE_SEPARATOR, &font)
        + link
        <= width
}

/// The change link on a row of its own (G135): only its words, at the text edge. A quiet action
/// that starts a line outside a message has no icon (N4, `Action::line_start`), so nothing hangs
/// into the margin.
fn change_link_line_start(ui: &mut egui::Ui, word: &str) -> egui::Response {
    design::small_link(ui, word)
}

/// The change link at the end of a detail line: the muted pencil, then the words as a `Small`
/// link. Returns the link's response: the DM opens the form when it is clicked.
pub(super) fn change_link(ui: &mut egui::Ui, word: &str) -> egui::Response {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label(Type::Small.rich(ui, change_mark()).color(colors().muted));
        design::small_link(ui, word)
    })
    .inner
}

/// Draw the member fault (FJ6) and the changes that caused it.
fn show_change_fault(
    ui: &mut egui::Ui,
    overrides: Option<&Overrides>,
    entry: &Entry,
    side: &str,
    error: &str,
    fault: &dyn Fn(&str) -> String,
) -> egui::Rect {
    let said = body_fault(
        ui,
        &MemberFault::new(FaultKind::InvalidChange, fault(&override_error(error))),
    );
    if let Some(overrides) = overrides {
        change_rows(ui, &overrides.rows(&Base::of(entry, side)));
    }
    // G122 asks whether the fault itself is on screen.
    said
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

/// A number that can stay the same, be set, or change by an amount. Each control keeps the
/// override valid on its own, so that the form never holds a block that it cannot read.
#[allow(
    clippy::disallowed_methods,
    reason = "ruling M20: a change’s fault under its own field, typed (`Ban::Body`)"
)]
fn change_row(
    ui: &mut egui::Ui,
    field: design::FieldId,
    label: &str,
    value: &mut Option<Change>,
    set: Option<std::ops::RangeInclusive<i64>>,
    add: std::ops::RangeInclusive<i64>,
    base: &Base,
) -> Option<egui::Rect> {
    let mut fault = None;
    let mut field_left = ui.cursor().left();
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(
            egui::vec2(LABEL_WIDTH, Density::Read.control()),
            egui::Sense::hover(),
        );
        ui.painter().text(
            rect.left_center(),
            egui::Align2::LEFT_CENTER,
            label,
            Type::Small.font(ui),
            colors().muted,
        );
        let low = set.as_ref().map_or(1, |range| *range.start().max(&1));
        // The pressure caller of lane E1 (round 31 SD11, chosen-value-weight): the `Dropdown`
        // part, whose closed value is regular `Body` weight, as every closed drop-down's.
        let mut kind = ChangeKind::of(*value);
        let name = format!("{label}: how it changes");
        // Where the field starts: its reason starts there too.
        field_left = ui.cursor().left();
        let mut dropdown = design::Dropdown::new(field, &name)
            .width(86.0)
            .choice(ChangeKind::Same, "Same");
        if set.is_some() {
            dropdown = dropdown.choices([(ChangeKind::Set, "Set to")]);
        }
        if dropdown
            .choices([(ChangeKind::Add, "Add")])
            .show(ui, &mut kind)
            .changed
        {
            *value = match kind {
                ChangeKind::Same => None,
                ChangeKind::Set => Some(Change::Set(low)),
                ChangeKind::Add => Some(Change::Add(1)),
            };
        }
        match value {
            Some(Change::Set(number)) => {
                let range = set.clone().unwrap_or(0..=0);
                design::stepper(ui, &format!("{label}: value"), number, range);
            }
            Some(Change::Add(number)) => {
                let mut amount = *number;
                design::signed_stepper(ui, &format!("{label}: amount"), &mut amount, add);
                // Zero changes nothing, so the choice for that is Same: a step from ±1 goes on
                // to ∓1 (issue #586), and a typed 0 keeps the amount.
                if amount == 0 && number.abs() == 1 {
                    amount = -*number;
                }
                if amount != 0 {
                    *number = amount;
                }
            }
            None => {}
        }
        let preview = single_change(label, *value);
        fault = preview.resolve(base).err();
        if let Some(row) = preview.rows(base).first() {
            // An invalid value marks the place where the DM typed it.
            let (words, color) = if fault.is_some() {
                (
                    format!(
                        "{} {}",
                        icons::ERROR,
                        crate::encounter_run::change_preview(row)
                    ),
                    colors().result.error,
                )
            } else {
                (crate::encounter_run::change_preview(row), colors().muted)
            };
            ui.add(egui::Label::new(Type::Small.rich(ui, words).color(color)).wrap());
        }
    });
    // The reason sits under its own field, at the field's left edge, not at the form's top.
    // Its rect is where the form says this fault (G122).
    // Layout 11c (M0233): it is placed by its rect at the field's measured left.
    fault.map(|error| {
        let mut column = ui.available_rect_before_wrap();
        column.min.x = column.min.x.max(field_left);
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(column)
                .layout(egui::Layout::top_down(egui::Align::Min)),
            |ui| {
                design::outcome::row_fault(
                    ui,
                    &MemberFault::new(FaultKind::InvalidChange, override_error(&error)),
                )
                .rect
            },
        )
        .inner
    })
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

/// One line of words, or nothing. Blank means no change.
/// The exact words the DM types in a draft's field that stores a cleaned value (a lone space
/// stores no name). While the field has focus it keeps them, so the field part never sees its
/// bound text rewritten and keeps the DM's text Undo. It is a field of the draft, so the words
/// end with the draft (Save, Discard, a replacement or a reload). One field has focus at a time.
#[derive(Clone, Debug, Default)]
pub(super) struct TypedWords(Option<(egui::Id, String)>);

impl TypedWords {
    /// The text to bind to the field `id`: the kept words while it has them, else `stored`.
    pub(super) fn bound(&self, id: egui::Id, stored: impl FnOnce() -> String) -> String {
        match &self.0 {
            Some((kept, words)) if *kept == id => words.clone(),
            _ => stored(),
        }
    }

    /// After the field `id` drew: keep its words while it has focus, and forget them when not.
    pub(super) fn keep(&mut self, id: egui::Id, focused: bool, words: String) {
        if focused {
            self.0 = Some((id, words));
        } else if self.0.as_ref().is_some_and(|(kept, _)| *kept == id) {
            self.0 = None;
        }
    }

    #[cfg(test)]
    pub(super) fn holds_words(&self) -> bool {
        self.0.is_some()
    }
}

fn text_row(
    ui: &mut egui::Ui,
    field: design::FieldId,
    label: &str,
    value: &mut Option<String>,
    hint: &str,
    typed: &mut TypedWords,
) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(
            egui::vec2(LABEL_WIDTH, Density::Read.control()),
            egui::Sense::hover(),
        );
        ui.painter().text(
            rect.left_center(),
            egui::Align2::LEFT_CENTER,
            label,
            Type::Small.font(ui),
            colors().muted,
        );
        // While the field has focus it keeps the exact words typed (a lone space too): the field
        // part drops its text Undo when the bound text is rewritten under it.
        let id = field.named_id(label);
        let mut text = typed.bound(id, || value.clone().unwrap_or_default());
        let width = text_field_width(ui.available_width());
        let response = design::line_field(ui, field, label, &mut text, hint, width);
        if response.changed() {
            *value = (!text.trim().is_empty()).then(|| text.clone());
        }
        typed.keep(id, response.has_focus(), text);
    });
}

/// A text row's field, from the room after its label.
fn text_field_width(room: f32) -> f32 {
    (room - design::S).clamp(120.0, 320.0)
}

/// FB6: where the change form's fields end, from its left: a text row's label and field. The
/// Other effect field with its Add, and the Loot field, end here too.
fn form_edge(ui: &egui::Ui) -> f32 {
    let label = LABEL_WIDTH + ui.spacing().item_spacing.x;
    label + text_field_width(ui.available_width() - label)
}

/// The Other effect field's width, so that its Add ends at the form's edge.
fn other_effect_width(ui: &egui::Ui) -> f32 {
    let add = Action::secondary("Add")
        .density(Density::Read)
        .natural_width(ui);
    (form_edge(ui) - ui.spacing().item_spacing.x - add).max(design::XXL)
}

/// The whole list of changes this slot or member can build in. `sides` is for a roster slot,
/// whose row already sets its side, so this offers a side only when a note has given one, to
/// change or clear it. A group can arrive anywhere, so its members can name any side.
/// What a change form's frame did: `done` when the DM closed it, `changed` when it wrote a new
/// change into the block (its own comparison says so, so a caller copies nothing to find out).
#[derive(Default)]
pub(super) struct FormEnd {
    pub(super) done: bool,
    /// The press of `Done`, for its owner's reveal of the row (`design::reveal`).
    pub(super) done_act: Option<design::reveal::Act>,
    pub(super) changed: bool,
    /// Where the form says its change faults, if any (G122: the text the DM reads).
    pub(super) fault: Option<egui::Rect>,
    /// The whole form, from its heading to its end: the target of the reveal its opener asked
    /// for (M0237).
    pub(super) form: Option<egui::Rect>,
}

#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
pub(super) fn overrides_form(
    ui: &mut egui::Ui,
    id: &str,
    block: &mut Mapping,
    entry: &Entry,
    side: &str,
    sides: Option<&[String]>,
    typed: &mut TypedWords,
) -> FormEnd {
    // The form's fields, keyed by its row (`id`).
    let key = design::TextKey::from_data(id);
    let mut overrides = match Overrides::parse(block) {
        Ok(overrides) => overrides,
        Err(error) => {
            let said = body_fault(
                ui,
                &MemberFault::new(
                    FaultKind::Unreadable,
                    format!("{error} Repair it in Campaign → Campaign entries."),
                ),
            );
            return FormEnd {
                fault: Some(said),
                ..FormEnd::default()
            };
        }
    };
    let base = Base::of(entry, side);
    let before = overrides.clone();
    let linked = entry.fields.contains_key("splat_external");
    design::gap(ui, design::Gap::XS);
    let mut done = false;
    let mut done_act = None;
    let heading = ui
        .horizontal(|ui| {
            // M0236: the form's label is the shared heading part (`Heading::Label`), which adds
            // no space in a row.
            let title = format!("Changes for {}", shown_name(block, entry.name.clone()));
            let heading = design::heading(ui, design::Heading::Label(&title));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let response = Action::quiet("Done").name("Done with changes").show(ui);
                done = response.clicked();
                done_act = design::reveal::Act::clicked(ui, &response);
            });
            heading
        })
        .inner;
    // A value that cannot stand says so under its own field; only a fault no field explains
    // (a change that a linked sheet decides) stays at the top.
    let on_a_field = [
        ("Maximum HP", overrides.max_hp),
        ("AC", overrides.armor_class),
        ("Initiative bonus", overrides.initiative_bonus),
        ("Speed", overrides.speed),
        ("Attack bonus", overrides.attack_bonus.map(Change::Add)),
        ("Damage bonus", overrides.damage_bonus.map(Change::Add)),
    ]
    .into_iter()
    .any(|(label, value)| single_change(label, value).resolve(&base).is_err());
    let mut fault: Option<egui::Rect> = None;
    let mut said = |rect: Option<egui::Rect>| {
        if let Some(rect) = rect {
            fault = Some(fault.map_or(rect, |fault| fault.union(rect)));
        }
    };
    if let Err(error) = overrides.resolve_for(entry, side)
        && !on_a_field
    {
        said(Some(body_fault(
            ui,
            &MemberFault::new(FaultKind::InvalidChange, override_error(&error)),
        )));
    }
    text_row(
        ui,
        OVERRIDE_NAME.of(&key),
        "Display name",
        &mut overrides.name,
        &entry.name,
        typed,
    );
    // Only a name just typed: opening the form must not edit what a note holds.
    if overrides.name != before.name {
        overrides.name = overrides
            .name
            .take()
            .map(|name| name.trim_start().chars().take(MAX_NAME).collect());
    }
    // A linked character's sheet decides these. One already here can still be cleared.
    if !linked || overrides.max_hp.is_some() {
        said(change_row(
            ui,
            CHANGE_MAX_HP.of(&key),
            "Maximum HP",
            &mut overrides.max_hp,
            Some(1..=1_000_000),
            -1_000_000..=1_000_000,
            &base,
        ));
    }
    said(change_row(
        ui,
        CHANGE_AC.of(&key),
        "AC",
        &mut overrides.armor_class,
        Some(0..=1000),
        -1000..=1000,
        &base,
    ));
    said(change_row(
        ui,
        CHANGE_INITIATIVE.of(&key),
        "Initiative bonus",
        &mut overrides.initiative_bonus,
        Some(-1000..=1000),
        -1000..=1000,
        &base,
    ));
    for (field, label, bonus) in [
        (CHANGE_ATTACK, "Attack bonus", &mut overrides.attack_bonus),
        (CHANGE_DAMAGE, "Damage bonus", &mut overrides.damage_bonus),
    ] {
        let mut value = bonus.map(Change::Add);
        said(change_row(
            ui,
            field.of(&key),
            label,
            &mut value,
            None,
            -1000..=1000,
            &base,
        ));
        *bonus = match value {
            Some(Change::Add(amount)) => Some(amount),
            _ => None,
        };
    }
    said(change_row(
        ui,
        CHANGE_SPEED.of(&key),
        "Speed",
        &mut overrides.speed,
        Some(0..=1000),
        -1000..=1000,
        &base,
    ));
    match sides {
        Some(_) if overrides.side.is_none() => {}
        Some(sides) => {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(LABEL_WIDTH, Density::Read.control()),
                    egui::Sense::hover(),
                );
                ui.painter().text(
                    rect.left_center(),
                    egui::Align2::LEFT_CENTER,
                    "Side",
                    Type::Small.font(ui),
                    colors().muted,
                );
                let mut changed = overrides.side.clone().unwrap_or_else(|| side.to_owned());
                super::side_control(
                    ui,
                    (
                        super::list::SIDE_CHOICE.of(&key),
                        super::list::SIDE_OTHER.of(&key),
                    ),
                    "Changed side",
                    &mut changed,
                    sides,
                );
                overrides.side = Some(changed);
                if Action::quiet("Same as row").show(ui).clicked() {
                    overrides.side = None;
                }
            });
        }
        None => {
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(LABEL_WIDTH, Density::Read.control()),
                    egui::Sense::hover(),
                );
                ui.painter().text(
                    rect.left_center(),
                    egui::Align2::LEFT_CENTER,
                    "Side",
                    Type::Small.font(ui),
                    colors().muted,
                );
                let mut changed = overrides.side.clone().unwrap_or_else(|| side.to_owned());
                super::side_control(
                    ui,
                    (
                        super::list::SIDE_CHOICE.of(&key),
                        super::list::SIDE_OTHER.of(&key),
                    ),
                    "Changed side",
                    &mut changed,
                    &[],
                );
                if changed != overrides.side.as_deref().unwrap_or(side) {
                    overrides.side = Some(changed);
                }
                if overrides.side.is_some() && Action::quiet("Same as group").show(ui).clicked() {
                    overrides.side = None;
                }
            });
        }
    }
    if !linked || !overrides.conditions.is_empty() {
        // FI8: the heading is drawn in the form's column, so the part puts its `L` above it (as
        // the first thing in the width scope below, it had none).
        design::heading(ui, design::Heading::Label("Starting conditions"));
        // The run's condition form width: the grid, the effects and Add stay within it.
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
            conditions_row(ui, id, &mut overrides.conditions);
        });
    }
    text_row(
        ui,
        OVERRIDE_NOTE.of(design::TextKey::from_data(id)),
        "Note",
        &mut overrides.note,
        "Why this creature differs",
        typed,
    );
    // A group of rows, as Starting conditions is.
    design::heading(ui, design::Heading::Label("Loot"));
    // The loot field ends at the form's edge; `Roll on a table` follows it.
    let loot_width = form_edge(ui)
        + ui.spacing().item_spacing.x
        + design::menu_button_width(ui, crate::loot_editor::ROLL_MENU);
    ui.scope(|ui| {
        ui.set_max_width(loot_width.min(ui.available_width()));
        crate::loot_editor::show_cell(
            ui,
            &key,
            &shown_name(block, entry.name.clone()),
            &mut overrides.loot,
        );
    });
    let changed = overrides != before;
    if changed {
        *block = overrides.to_mapping();
    }
    // The form's whole rect, `S` above its heading as the old scroll kept it.
    let mut form = heading.rect;
    form.min.y -= design::S;
    form.max.y = form.max.y.max(ui.cursor().top());
    FormEnd {
        done,
        done_act,
        changed,
        fault,
        form: Some(form),
    }
}

/// The same official names and local Other effect as the run.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn conditions_row(ui: &mut egui::Ui, id: &str, conditions: &mut Vec<String>) {
    // The run's grid: fixed columns, `S` apart.
    if let Some(name) =
        crate::encounter_run::view::conditions_grid(ui, "starting_conditions", |name| {
            conditions.iter().any(|c| same_condition(c, name))
        })
    {
        if let Some(at) = conditions.iter().position(|c| same_condition(c, name)) {
            conditions.remove(at);
        } else {
            conditions.push(name.to_owned());
        }
    }
    let official = |c: &str| {
        crate::combat_sync::CONDITIONS
            .iter()
            .any(|(_, name)| same_condition(c, name))
    };
    let custom: Vec<_> = conditions
        .iter()
        .filter(|c| !official(c))
        .cloned()
        .collect();
    let width = other_effect_width(ui);
    if let Some(at) =
        crate::encounter_run::view::other_effects_list(ui, custom.iter().map(String::as_str), width)
    {
        conditions.retain(|c| c != &custom[at]);
    }
    let draft_id = ui.id().with("starting_other_effect");
    let mut text = ui
        .data(|data| data.get_temp::<String>(draft_id))
        .unwrap_or_default();
    // The member's draft: its field keeps its own cursor and Undo (`id` names this member).
    if crate::encounter_run::view::other_effect_field(
        ui,
        design::TextKey::from_data(id),
        &mut text,
        (width, "Other effect…"),
        false,
    )
    .0
    {
        if !conditions.iter().any(|c| same_condition(c, text.trim())) {
            conditions.push(text.trim().to_owned());
        }
        text.clear();
    }
    ui.data_mut(|data| data.insert_temp(draft_id, text));
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

    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state`, and raw egui calls (design::results::STAGED_ALLOWS)"
    )]
    pub(super) fn group_editor_view(
        &mut self,
        ui: &mut egui::Ui,
        active: &Campaign,
        campaigns: &Campaigns,
        in_plan: bool,
    ) {
        if self.group_disk_view(ui, active, campaigns.modules()) {
            return;
        }
        let dirty = self.draft_dirty();
        let has_run = self.has_run();
        // FN5, AI2: a draft that Save's check refuses says Invalid, changed or not, before any
        // Save, and Save is blocked and not primary.
        let refusal = self.group_refusal();
        // The Session page has no save of its own to give this key to.
        // One `DraftState` decides the save row's word, Save's weight and reason, Discard and the
        // campaign lock; keyboard Save runs exactly when Save can (migrate:O, rows 139, FB9).
        let empty = self
            .group_draft
            .as_ref()
            .is_some_and(|draft| draft.setup.members.is_empty());
        let state = {
            let changed = if dirty {
                design::memo::Changed::Yes
            } else {
                design::memo::Changed::No
            };
            // An open run's `Next turn` owns the one primary action.
            let primary = if has_run {
                design::save_row::Primary::Elsewhere
            } else {
                design::save_row::Primary::Draft
            };
            // R37: the origin is the draft's own stored fact (`GroupDraft::origin`), so a new
            // group with only its name says `New`, and a saved group an edit emptied stays a
            // refused change. With no draft open the row shows nothing of a draft.
            let origin = self
                .group_draft
                .as_ref()
                .map_or(design::save_row::DraftOrigin::Saved, GroupDraft::origin);
            let mut state = design::DraftState::new(origin, changed, primary).locks_campaign();
            // E2: the check's one ranked result; an empty group says its reason beside Save
            // (`Add a creature first.`) only while nothing outranks the missing part.
            state = state.check(save_row_diagnosis(refusal.as_ref(), empty));
            if self.message.is_some() {
                // FC1: the shared `Saved`; the list keeps the full sentence for after Back.
                state = state.just_saved();
            }
            state
        };
        // The plan's Ctrl+S is the group's, taken before its fields and done after its whole
        // body took this frame's input, when the group then has changes (the first edit too).
        // On Encounters the page takes it (`save_shortcut`).
        let save_key = in_plan && super::take_save_key(ui.ctx());
        // R38: the back line on a line of its own above the name row. The group's changes hold
        // it first, then an unsaved creature.
        let head = design::EditorHead::new(design::EditorKind::Group { in_plan }, &*self);
        let compact = ui.available_width() < 760.0;

        let focus_name = std::mem::take(&mut self.focus_name);
        let draft = self.group_draft.as_mut().expect("the editor has a draft");
        let (mut save, mut discard) = (false, false);
        let mut name_right = f32::INFINITY;
        // Layout step 6 (`DraftHead`, I7, I8): one plan for the head, made before it paints from
        // the widths only. The members column's edge comes from the room and the widths the body
        // draws its columns with (`members_edge`), not from an older frame: the name field ends
        // at that column (with `Belongs to`) or at `FORM_WIDTH`; Save group follows it `L` after,
        // or takes its own line; the save row's reasons are measured to the same edge, beside the
        // widest state word and Discard in every state, so a state change moves no field (SL4).
        let members_right = members_edge(ui, compact);
        let left = ui.max_rect().left();
        // Save keeps one width in both weights, so its left edge does not move when a press
        // changes its weight (G1b).
        let save_width = Action::primary("Save group")
            .natural_width(ui)
            .max(Action::secondary("Save group").natural_width(ui))
            .ceil();
        let plan = design::save_row::head_plan(
            ui,
            &design::save_row::HeadSpec {
                // The group's state never says changed on disk, not saved or repair.
                words: &design::save_row::EDIT_WORDS,
                column: members_right - left,
                line: ui.max_rect().right() - left,
                actions: save_width,
                row: members_right - left,
                reasons: &HEAD_REASONS,
            },
        );
        let button = || {
            match state.save_weight() {
                design::save_row::SaveWeight::Primary => Action::primary("Save group"),
                design::save_row::SaveWeight::Secondary => Action::secondary("Save group"),
            }
            .min_width(save_width)
            .blocked(state.save_blocked())
        };
        let (back, ()) = head.show(ui, |ui| {
            ui.horizontal(|ui| {
                let response = name_field(ui, draft, plan.name_width);
                design::named(ui, &response, &format!("Group name: {}", draft.name));
                if response.changed() {
                    draft.touch();
                }
                // Belongs to ends where the name field ends.
                name_right = response.rect.right();
                if focus_name {
                    design::focus_and_select(ui, response.id, &draft.name);
                }
                if plan.actions_beside {
                    // Save starts `L` after the field.
                    let gap = ui.spacing().item_spacing.x;
                    ui.allocate_ui_with_layout(
                        egui::vec2(design::L - gap + save_width, Density::Scan.control()),
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| save = button().show(ui).clicked(),
                    );
                }
            });
            if !plan.actions_beside {
                ui.horizontal(|ui| save = button().show(ui).clicked());
            }
        });
        // The save row: the state and Discard at its start. A reason the plan does not put on the
        // row takes its own line below it, kept in every state (AK1, FK1), so a reason that comes
        // or goes moves no field.
        // Each line below the row: the words it is measured by, the words it shows now, and
        // whether it is the campaign-switch reason's line (G122, below).
        let mut below: Vec<(&'static str, Option<&'static str>, bool)> = Vec::new();
        // G122, lead ruling (round 29, the band at 960): while Save refuses a fault that the
        // body says in its own place (under Belongs to, or in a member's row), short words for
        // it take the place of the campaign-switch reason, the lowest of the row's reasons: from
        // that reason's start to the row's end, where Save's own reason would be (an empty group
        // cannot be refused for these). No place is added, so the row and the rule stay where
        // they are on main. The body draws first; then that place shows these words when the
        // fault's own place is not whole on screen (another tab, scrolled away), else the switch
        // reason as before. So the fault is said once, by the mouse and the keyboard alike.
        // R41: in the refusal's own tone, the one the save row's word takes.
        let elsewhere: Option<(Tone, &'static str)> =
            refusal.as_ref().and_then(|refusal| match refusal.place() {
                RefusalPlace::BelongsTo => Some((refusal.tone(), GONE_BESIDE_SAVE)),
                RefusalPlace::Member(_) => Some((refusal.tone(), MEMBER_BESIDE_SAVE)),
                _ => None,
            });
        let gone = elsewhere.is_some();
        // The switch reason's place while `gone`, and its words, drawn after the body.
        let mut gone_place: Option<egui::Rect> = None;
        let mut deferred_switch: Option<&'static str> = None;
        let reasons_right = members_right;
        // Ruling M24: where the switch reason does not fit beside the widest word and Discard,
        // the row says its short form in every state, the full reason in hover and the accessible
        // description; no line is kept for it below.
        let switch_short = plan.short[1];
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Scan.control());
            discard =
                super::editor_save_state(ui, state.word().status(), state.discard_shown(), None);
            // Save's own reason first, at the row's end, then the campaign-switch reason: each in
            // its planned place. FB6: while Save is blocked, Discard is the one way out, so the
            // lock says only that (`DraftState::lock_reason`); its place is measured by the
            // longer form.
            let mut place =
                |at: usize, measure: &'static str, shown: Option<&'static str>, switch| {
                    if plan.on_row[at] {
                        (shown, true)
                    } else {
                        below.push((measure, shown, switch));
                        (None, false)
                    }
                };
            let (empty_reason, _) = place(0, EMPTY_REASON, empty.then_some(EMPTY_REASON), false);
            let (switch, on_row) = place(
                1,
                design::save_row::SAVE_OR_DISCARD_TO_SWITCH,
                state.lock_reason(),
                true,
            );
            if gone && on_row {
                let row = ui.max_rect();
                gone_place = Some(egui::Rect::from_min_max(
                    egui::pos2(ui.cursor().left(), row.top()),
                    egui::pos2(reasons_right, row.top() + Density::Scan.control()),
                ));
                deferred_switch = switch;
            } else {
                switch_reason(ui, switch, switch_short);
            }
            ui.allocate_ui_with_layout(
                egui::vec2(
                    (reasons_right - ui.cursor().left()).max(0.0),
                    Density::Scan.control(),
                ),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| super::blocked_reason(ui, empty_reason),
            );
        });
        for (_, shown, switch) in below {
            let top = ui.cursor().top();
            // A layout too narrow for the reason on the row (narrower than 960, beside Lookup):
            // its own line, as tall as the tallest of its forms and of the words that can take
            // its place, whether or not any shows or a refusal stands (layout step 6, Astra's P2).
            let height = design::save_row::slot_height(
                ui,
                &HEAD_REASONS[usize::from(switch)],
                ui.available_width(),
            );
            let shown = if gone && switch {
                gone_place = Some(egui::Rect::from_min_size(
                    egui::pos2(ui.cursor().left(), top),
                    egui::vec2(ui.available_width(), height),
                ));
                deferred_switch = shown;
                None
            } else {
                shown
            };
            // Layout 11c (M0241): the slot is allocated at its height, so it keeps that height
            // whatever its reason takes, and grows only for a taller one.
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_min_height(height);
                    super::blocked_reason(ui, shown);
                },
            );
        }
        design::rule(ui);
        // The file's warnings that no member row states; they go in the footer below.
        let unread =
            splat::content::list_formats::unread_group_members(&draft.source, &draft.flags);
        let file_warnings: Vec<String> = draft
            .flags
            .iter()
            .filter(|flag| !unread.iter().any(|member| member.flag.line == flag.line))
            .map(|flag| flag.message.clone())
            .collect();

        if back {
            self.clear_notice();
            self.close_group_in_plan();
            self.error = None;
            return;
        }
        if discard {
            self.discard_group_draft(active);
            return;
        }
        // The group's Save (its button above, the plan's or the page's Ctrl+S) is done after the
        // WHOLE body (the members, the notes) took this frame's input, in the pane's `then`, and
        // the footer's lines are read after it (L1c, review of PR 499). A key saves only a group
        // that has changes then; the button is blocked without them.
        let key = save_key || std::mem::take(&mut self.save_key);
        let frame = design::outcome::PaneFrame::new(ui);
        let footer_id = self.group_draft.as_ref().map(|draft| draft.id());
        let messages = design::outcome::Messages::new(
            &frame,
            ("group_editor_footer", footer_id),
            ui.available_width(),
        );
        // FF8: a press of a copied note's repair in the search, for the page's store.
        let separated = std::cell::Cell::new(None);
        // G122: whether the body drew the refusal's own place whole on screen.
        let gone_shown = std::cell::Cell::new(false);
        let (separated_ref, gone_ref) = (&separated, &gone_shown);
        let place = refusal.as_ref().map(Refusal::place);
        let ctx = ui.ctx().clone();
        let this = &mut *self;
        // The members and Add creatures are columns, each with its own scroll (in the narrow
        // layout, one of them under its tab): the body fills its room above the footer.
        let paned = messages.pane_then(
            ui,
            design::pane::Body::Fills(design::pane::Fills::because(
                design::pane::FillsReason::Columns,
            )),
            move |ui| {
                gone_ref.set(this.group_editor_body(ui, compact, name_right, separated_ref, place));
                this
            },
            |this, messages| {
                let keyed = key && this.group_draft.as_ref().is_some_and(GroupDraft::dirty);
                if save || keyed {
                    this.save_group_draft(&ctx, active, campaigns.modules());
                }
                this.refresh_validation_error(true, super::edit_input(&ctx));
                // FI1's family: the editor's results with a live Undo (the discard, the newest
                // member result), then Save's reason, then the file's warnings sit in one footer
                // pinned below the members and the search, so none of them moves a member or a
                // field: the member result, the discard, the copies' separations, newest first by
                // their stores, then the reasons and warnings.
                this.record_member_results();
                let kinds = this.kept_results(false);
                if let Some(root) = this.active.as_ref().map(|active| active.path.clone())
                    && let Some(draft) = this.group_draft.as_ref()
                    && let Some(shown) = this.member_results.get(&super::MemberOwner {
                        root,
                        draft: draft.results_key(),
                    })
                {
                    messages.kept(shown.outcome(&frame, super::MemberMark::outcome));
                }
                for line in this.kept_lines(&frame, &kinds) {
                    messages.kept(line);
                }
                for line in this.separation_lines(&frame, &active.path) {
                    messages.kept(line);
                }
                // Save's reason: the last refusal, or the check's fault when the editor shows it
                // at no other place (a member row or the reason beside Save already says
                // theirs). R41: a refusal's words take the refusal's tone; a failed write is an
                // error.
                let live = refusal
                    .as_ref()
                    .and_then(|refusal| refusal.footer().map(|words| (words, refusal.diagnosis())));
                let reason = match (this.error.clone(), live) {
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
                if let Some(error) = this.separation_failure() {
                    messages.outcome(design::outcome::FaultMessage::new(
                        &super::EncounterFault::SeparationFailed(error),
                    ));
                }
                for warning in file_warnings {
                    messages.outcome(design::outcome::FaultMessage::new(
                        &super::EncounterFault::FileWarning(warning),
                    ));
                }
                this
            },
        );
        let pressed = paned.pressed;
        let this = paned.body;
        if let (Some(place), Some((tone, words))) = (gone_place, elsewhere.as_ref()) {
            let mut beside = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(place)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            // Nothing is painted outside the place.
            beside.set_clip_rect(place.intersect(ui.clip_rect()));
            if gone_shown.get() {
                switch_reason(&mut beside, deferred_switch, switch_short);
            } else {
                design::outcome::row_fault(&mut beside, &editor_fault(*tone, words));
            }
        }
        this.record_separation(separated.into_inner());
        if let Some(press) = pressed {
            match (press.id, press.ticket) {
                (super::FooterPress::Undo, Some(ticket)) => {
                    this.undo_by_ticket(ticket, Some((active, campaigns.modules())));
                }
                (super::FooterPress::UndoSeparation, Some(ticket)) => {
                    this.undo_separation(ui.ctx(), ticket, active);
                }
                (super::FooterPress::UndoMember, Some(ticket)) => this.undo_member(ticket),
                _ => {}
            }
        }
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

    /// The group editor's body, in the pane: the members and Add creatures, as columns or tabs.
    fn group_editor_body(
        &mut self,
        ui: &mut egui::Ui,
        compact: bool,
        name_right: f32,
        separated: &std::cell::Cell<Option<super::Separated>>,
        refused: Option<RefusalPlace>,
    ) -> bool {
        let Some(snapshot) = &self.snapshot else {
            design::spinner(ui);
            return false;
        };
        let Some(draft) = self.group_draft.as_mut() else {
            return false;
        };
        let mut pane = self.editor_pane;
        if compact {
            let count = draft
                .setup
                .members
                .iter()
                .map(|member| member.count as usize)
                .sum();
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                design::counted_tab(
                    ui,
                    &mut pane,
                    EditorPane::Roster,
                    "Members",
                    count,
                    ("member", "members"),
                );
                design::segmented(ui, &mut pane, &[(EditorPane::Add, "Add creatures")]);
            });
            self.editor_pane = pane;
        }
        let search = &mut self.search;
        let focus_search = &mut self.focus_search;
        let peek = &mut self.peek;
        let expanded = &mut self.expanded;
        if compact {
            match pane {
                EditorPane::Roster => {
                    return members_pane(ui, draft, snapshot, expanded, name_right, refused);
                }
                EditorPane::Add => add_pane(
                    ui,
                    draft,
                    snapshot,
                    search,
                    focus_search,
                    (peek, separated),
                    true,
                ),
            }
            return false;
        }
        let width = design::side_panel_width(ui.ctx().content_rect().width())
            .max(ui.available_width() - ROSTER_MAX_WIDTH)
            .min(ADD_PANE_MAX_WIDTH);
        egui::SidePanel::right("group_add")
            .resizable(false)
            .exact_width(width)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::M as i8,
                right: 0,
                top: design::S as i8,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                add_pane(
                    ui,
                    draft,
                    snapshot,
                    search,
                    focus_search,
                    (peek, separated),
                    false,
                );
            });
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: 0,
                right: 0,
                top: design::S as i8,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                ui.set_max_width(ui.available_width().min(ROSTER_MAX_WIDTH));
                members_pane(ui, draft, snapshot, expanded, name_right, refused)
            })
            .inner
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

fn add_pane(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    search: &mut String,
    focus_search: &mut bool,
    // Lookup's creature, and where a press of a copied note's repair says what it did (FF8).
    (peek, separated): (
        &mut Option<Uuid>,
        &std::cell::Cell<Option<super::Separated>>,
    ),
    compact: bool,
) {
    if !compact {
        design::heading(
            ui,
            design::Heading::Group {
                title: "Add creatures",
                count: 0,
            },
        );
    }
    super::lane_help(
        ui,
        "From this campaign and the packages it uses. Party members go on an encounter’s roster.",
    );
    let summary = if compact {
        draft_member_summary(draft, snapshot)
    } else {
        String::new()
    };
    let chosen = Encounters::creature_search_with_summary(
        ui,
        snapshot.search_list(true),
        search,
        focus_search,
        peek,
        super::SearchRoster {
            already: &|_| false,
            on_roster: super::OnRoster::Ids(&[]),
            separated,
        },
        &summary,
    );
    if let Some((id, _)) = chosen {
        if let Some((stat, name)) = snapshot.npc_stat_block(id) {
            draft.add_named(stat, name);
        } else {
            draft.add(id);
        }
    }
}

/// The key of a member's open changes form and of its fields: its group and its identity in the
/// draft, never its place among the members.
fn member_key(path: &str, member: &MemberDraft) -> String {
    format!("{path}:member_{}", member.id.0)
}

/// Where a group editor's members column ended in the last pass, for the name field above it.
/// The campaign-switch reason on the group's save row: its full words, or its short form with
/// the full words in hover and the accessible description (ruling M24).
fn switch_reason(ui: &mut egui::Ui, words: Option<&str>, short: bool) {
    match words {
        Some(full) if short => {
            design::save_row::short_reason(ui, design::save_row::switch_short(full), full);
        }
        words => super::blocked_reason(ui, words),
    }
}

/// Layout step 6: where the members column ends, from the room the editor has and the widths its
/// body draws the columns with (`group_editor_body`: the Add creatures panel, then the members at
/// most `ROSTER_MAX_WIDTH`; `members_pane`: the scroll bar's lane kept, at most `LIST_MEASURE`).
/// The head asks before it paints, so its name field and reasons end at the column's edge in the
/// same pass, with no edge kept from an older one.
fn members_edge(ui: &egui::Ui, compact: bool) -> f32 {
    let room = ui.available_width();
    let members = if compact {
        room
    } else {
        let add = design::side_panel_width(ui.ctx().content_rect().width())
            .max(room - ROSTER_MAX_WIDTH)
            .min(ADD_PANE_MAX_WIDTH);
        (room - add).min(ROSTER_MAX_WIDTH)
    };
    ui.max_rect().left()
        + (members - ui.spacing().scroll.allocated_width()).clamp(0.0, design::LIST_MEASURE)
}

fn members_pane(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    expanded: &mut Option<String>,
    name_right: f32,
    refused: Option<RefusalPlace>,
) -> bool {
    // FH3: the column leaves the scroll bar's lane free whether or not the bar shows, so the
    // name field, `Edit` and `⋯` keep their x when a long list (a replacement search) scrolls.
    let width = super::scroll_lane(ui).min(design::LIST_MEASURE);
    // M0245 (ruling M27): the editor's own column, in its pane's `Fills(Columns)` body on the
    // Encounters page and in the Session plan; it hosts the reveals of its forms.
    design::ScrollColumn::new("group_editor")
        .hosts_reveals()
        .show(ui, |ui| {
            // Its width is the one `members_edge` gives the head before it paints.
            ui.scope(|ui| {
                ui.set_width(width.min(ui.available_width()));
                group_fields(ui, draft, snapshot, expanded, None, name_right, refused)
            })
            .inner
        })
        .inner
}

/// FB4: `Default side` and `Belongs to` share one label column, the wider label plus `S`, so
/// Layout 11c (SB1, AB1): a whole block of a group's form, registered, so a `Fills` pane's
/// footer ends the column above it, with a hairline, and never cuts through it.
fn register_whole(ui: &egui::Ui, block: egui::Rect) {
    design::register_row(ui, block);
}

/// `add` as one whole block of a group's form (`register_whole`).
fn whole_row<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let block = ui.scope(add);
    register_whole(ui, block.response.rect);
    block.inner
}

/// their controls start at one x.
fn choice_label<R>(ui: &mut egui::Ui, text: &str, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let font = Type::Small.font(ui);
    let widest = [SIDE_FIELD, BELONGS_TO]
        .map(|label| {
            ui.fonts_mut(|fonts| {
                fonts
                    .layout_no_wrap(label.to_owned(), font.clone(), colors().muted)
                    .size()
                    .x
            })
        })
        .into_iter()
        .fold(0.0, f32::max);
    let column_end = ui.cursor().left() + widest + design::S.max(ui.spacing().item_spacing.x);
    ui.label(Type::Small.rich(ui, text).color(colors().muted));
    // Layout 11c (M0246): the control after the label is placed by its rect at the labels' one
    // column, so every control of these rows starts at one x.
    let mut room = ui.available_rect_before_wrap();
    room.min.x = room.min.x.max(column_end);
    ui.scope_builder(
        egui::UiBuilder::new().max_rect(room).layout(*ui.layout()),
        add,
    )
    .inner
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
/// A member's change map, copied and counted. The editor makes no such copy on a frame; a test
/// that puts one back (its mutation proof) goes through here, so the scenario counts it.
#[cfg(any(test, feature = "test-support"))]
#[allow(dead_code)]
fn counted_changes_copy(changes: &Mapping) -> Mapping {
    CHANGE_COPIES.with(|copies| copies.set(copies.get() + 1));
    changes.clone()
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

/// What a group is, in its own editor and in the frame an encounter's editor gives it. There,
/// `open` is that encounter, and the frame needs less weight and room.
fn group_fields(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    expanded: &mut Option<String>,
    open: Option<(Uuid, &str)>,
    name_right: f32,
    refused: Option<RefusalPlace>,
) -> bool {
    ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
    whole_row(ui, |ui| {
        ui.horizontal(|ui| {
            choice_label(ui, SIDE_FIELD, |ui| {
                let name = format!("{SIDE_FIELD} for {}", draft.name);
                let side = draft.setup.side.clone();
                super::side_control(
                    ui,
                    (
                        super::list::SIDE_CHOICE.of(draft.id),
                        super::list::SIDE_OTHER.of(draft.id),
                    ),
                    &name,
                    &mut draft.setup.side,
                    &[],
                );
                if draft.setup.side != side {
                    draft.touch();
                }
            });
        })
    });
    if open.is_none() {
        design::help(ui, "Party, enemies, or any other name, such as rivals.");
        design::gap(ui, design::Gap::S);
    }
    // G122: whether the refusal's own place is whole on screen, for the save row: the gone
    // encounter's line, or the refused member's row.
    let gone_shown = open.is_none_or(|(id, _)| !draft.belongs_to(id))
        && belongs_to(ui, draft, snapshot, open, name_right);
    let refused_member = match refused {
        Some(RefusalPlace::Member(id)) => Some(id),
        _ => None,
    };
    design::gap(ui, design::Gap::S);
    let unread = splat::content::list_formats::unread_group_members(&draft.source, &draft.flags);
    let count = draft
        .setup
        .members
        .iter()
        .map(|member| member.count)
        .sum::<u64>()
        + unread.iter().filter_map(|member| member.count).sum::<u64>();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        // M0247: the shared heading part; in this row it adds no space.
        design::heading(
            ui,
            design::Heading::Group {
                title: "Members",
                count: count as usize,
            },
        );
        if !draft.setup.members.is_empty() {
            ui.label(
                Type::Small
                    .rich(
                        ui,
                        format!("{}{}", " · ", cached_members_heading(ui, draft, snapshot)),
                    )
                    .color(colors().muted),
            );
        }
    });
    if draft.setup.members.is_empty() && unread.is_empty() {
        let empty = "No members yet. Add creatures from the search.";
        ui.add(egui::Label::new(Type::Small.rich(ui, empty).color(colors().muted)).wrap());
    }
    let member_shown = members(ui, draft, snapshot, expanded, refused_member);
    design::gap(
        ui,
        if open.is_some() {
            design::Gap::S
        } else {
            design::Gap::M
        },
    );
    whole_row(ui, |ui| {
        design::field(ui, NOTES_FIELD, None, None, |ui| {
            let hint =
                design::placeholder(ui, "When they arrive, what they want. Markdown is fine.");
            let rows = if open.is_some() { 2 } else { 4 };
            // Keyed by the group draft, so two groups never share its text or its Undo.
            let output = GROUP_NOTES.of(draft.id()).editor(
                ui,
                &mut draft.notes,
                super::plain_face(ui),
                |edit| super::notes_style(edit, rows).hint_text(hint),
            );
            design::named(
                ui,
                &output.response,
                &format!("{NOTES_FIELD} for {}", draft.name),
            );
            // F1: notes make a group changed, so Save checks it: an edit of them is an edit.
            if output.response.changed() {
                draft.touch();
            }
        })
    });
    gone_shown || member_shown
}

/// A group's notes field (`group_fields`).
const GROUP_NOTES: design::layout::FieldName = crate::field_id!("group_notes");

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

/// `members_heading` resolves the group against the whole catalogue; do it once per draft text.
fn cached_members_heading(ui: &egui::Ui, draft: &GroupDraft, snapshot: &Snapshot) -> String {
    let revision = egui::Id::new((
        draft.text().ok(),
        std::ptr::from_ref(snapshot) as usize,
        snapshot.catalogue.entries.len(),
    ))
    .value();
    let slot = egui::Id::new(("group_members_heading", &draft.path));
    if let Some((seen, label)) = ui.data(|data| data.get_temp::<(u64, String)>(slot))
        && seen == revision
    {
        return label;
    }
    let label = members_heading(draft, snapshot);
    ui.data_mut(|data| data.insert_temp(slot, (revision, label.clone())));
    label
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

#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
)]
/// `refused` is the member whose row says Save's refusal (G122): true when that row is drawn
/// whole on screen.
fn members(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    expanded: &mut Option<String>,
    refused: Option<Uuid>,
) -> bool {
    let mut refused_shown = false;
    let (mut remove, mut raise, mut lower, mut replaced) = (None, None, None, None);
    // A change made by a widget that edits a member in place (its count, its open changes).
    let mut edited = false;
    let total = draft.setup.members.len();
    let side = draft.setup.side.clone();
    let path = draft.path.clone();
    let unread = splat::content::list_formats::unread_group_members(&draft.source, &draft.flags);
    // FJ6: a fault names its line only while no member has moved, been added, changed or removed
    // since the note was read, so the line is still that member's.
    let saved_members = draft.members_are_saved(ui.ctx());
    for index in 0..=total {
        let preceding = unread.iter().filter(|member| member.before < index).count();
        for (offset, member) in unread
            .iter()
            .filter(|member| member.before.min(total) == index)
            .enumerate()
        {
            unread_member(ui, member, index + preceding + offset + 1, snapshot, draft);
        }
        let Some(member) = draft.setup.members.get_mut(index) else {
            break;
        };
        let number = index
            + unread
                .iter()
                .filter(|member| member.before <= index)
                .count()
            + 1;
        let identity = member.id.0;
        let row_top = ui.cursor().top();
        // Where the row says its fault (G122): the DM reads it there when it is whole on screen.
        let mut fault_said: Option<egui::Rect> = None;
        // The row's widgets key by the member's identity, not its place.
        ui.push_id(member.id.0, |ui| {
            let parsed = Overrides::parse(&member.overrides);
            let assessed = assess_member(
                &snapshot.catalogue,
                &snapshot.creatures,
                &member.creature,
                &side,
                parsed.as_ref().map_err(String::as_str),
            );
            let entry = assessed.entry;
            let overrides = entry.and(parsed.as_ref().ok());
            let name = entry.map_or_else(
                || missing_member_name(None, member.creature.link.as_deref()),
                |entry| entry.name.clone(),
            );
            let button = if entry.is_none() {
                format!("Replace {name}")
            } else {
                format!("Changes for {name}")
            };
            let name = overrides
                .and_then(|overrides| overrides.name.as_deref())
                .map_or(name, |changed| changed.trim().to_owned());
            let (detail, hp_missing) = entry
                .zip(assessed.creature)
                .map(|(entry, creature)| {
                    let changed = assessed.changed.as_ref().ok().and_then(Option::as_ref);
                    let (mut detail, hp_missing) = changed.map_or_else(
                        || (creature.detail.clone(), creature.hp_missing),
                        |changed| {
                            (
                                super::creature_detail(entry, creature.party, Some(changed)),
                                changed.max_hp.is_none(),
                            )
                        },
                    );
                    if member.overrides.contains_key("name") {
                        detail.push_str(&format!(" · as {}", entry.name));
                    }
                    (detail, hp_missing)
                })
                .unwrap_or_default();
            let key = member_key(&path, member);
            let is_open = expanded.as_deref() == Some(key.as_str());
            // Layout PR 5b (I6): the member's facts, measured for the members' action column.
            let facts = ui.scope(|ui| {
                let member_row = ui
                    .label(
                        Type::Strong
                            .rich(ui, format!("{number}. {name}"))
                            .color(colors().text),
                    )
                    .on_hover_text(&name)
                    .rect;
                // FC9: the change words end the detail line, after the muted pencil, as in the
                // encounter editor; a closed form only.
                let word = entry
                    .zip(overrides)
                    .filter(|_| !is_open && !member.overrides.is_empty())
                    .map(|(entry, overrides)| change_word(ui, overrides, entry, &side));
                if word.is_none() && !detail.is_empty() {
                    let response = ui.add(
                        egui::Label::new(super::detail_text(
                            ui,
                            &super::stat_line(&detail),
                            hp_missing,
                        ))
                        .wrap(),
                    );
                    design::named(ui, &response, &detail);
                } else if let Some(word) = &word {
                    // G135: the whole link fits after the detail, or it starts its own row under
                    // the whole detail. It is never cut, and no stat is left out for it.
                    let own_row = !detail.is_empty()
                        && !link_fits_after(
                            ui,
                            &super::stat_line(&detail),
                            word,
                            ui.available_width(),
                        );
                    if own_row {
                        let response = ui.add(
                            egui::Label::new(super::detail_text(
                                ui,
                                &super::stat_line(&detail),
                                hp_missing,
                            ))
                            .wrap(),
                        );
                        design::named(ui, &response, &detail);
                        let link = change_link_line_start(ui, word);
                        if link.clicked() {
                            // M0237: the form comes whole into view, by its column's reveal.
                            if let Some(act) = design::reveal::Act::clicked(ui, &link) {
                                draft.reveal.0.ask(act, draft.id, ("member_form", key.as_str()));
                            }
                            *expanded = Some(key.clone());
                        }
                    } else {
                        ui.horizontal_wrapped(|ui| {
                            // One row as tall as the link's target, so the detail and the link share
                            // their centre line.
                            ui.set_row_height(design::XL);
                            ui.spacing_mut().item_spacing.x = 0.0;
                            if !detail.is_empty() {
                                let response = ui.add(
                                    egui::Label::new(super::detail_text(
                                        ui,
                                        &super::stat_line(&detail),
                                        hp_missing,
                                    ))
                                    .wrap(),
                                );
                                design::named(ui, &response, &detail);
                            }
                            if !detail.is_empty() {
                                ui.label(
                                    Type::Small.rich(ui, CHANGE_SEPARATOR).color(colors().muted),
                                );
                            }
                            let link = change_link(ui, word);
                            if link.clicked() {
                                if let Some(act) = design::reveal::Act::clicked(ui, &link) {
                                    draft
                                        .reveal
                                        .0
                                        .ask(act, draft.id, ("member_form", key.as_str()));
                                }
                                *expanded = Some(key.clone());
                            }
                        });
                    }
                }
                member_row
            });
            let (member_row, facts) = (facts.inner, facts.response.rect);
            // M0251: the row, for the reveal its form's `Done` asks for.
            draft
                .reveal
                .0
                .mark(ui, draft.id, ("member_row", key.as_str()), member_row);
            let count_line = ui.horizontal(|ui| {
                ui.label(Type::Small.rich(ui, "Count").color(colors().muted));
                // M0249: the shared stepper for a count, with − and + and typing, up to
                // `MAX_COUNT`.
                let mut value = i64::try_from(member.count).unwrap_or(i64::MAX);
                let count = ui.scope(|ui| {
                    design::grouped_stepper(
                        ui,
                        &format!("How many of {name}"),
                        &mut value,
                        1..=i64::try_from(MAX_COUNT).unwrap_or(i64::MAX),
                    )
                });
                if count.inner {
                    member.count = u64::try_from(value).unwrap_or(1);
                    edited = true;
                }
                let count = count.response;
                // Layout PR 5b (I6), L5c: Edit and the menu end at the members' column, `L` after
                // their widest facts (the name, its detail, the count); where it does not fit,
                // the line's far edge, as before (`MeasuredColumn`). The line keeps its height.
                let left = facts.left();
                let own_facts = facts.width().max(count.rect.right() - left);
                let slot = design::action_rows::MeasuredColumn::end("member_row_columns").slot(
                    ui,
                    egui::Rect::from_min_max(
                        egui::pos2(ui.cursor().left(), ui.max_rect().top()),
                        ui.max_rect().max,
                    ),
                    left,
                );
                let actions = slot
                    .scope(ui, |ui| {
                            super::row_menu(ui, &name, |ui| {
                                if super::menu_action(
                                    ui,
                                    "Move up",
                                    (index == 0).then_some("Already first."),
                                ) {
                                    raise = Some(index);
                                }
                                if super::menu_action(
                                    ui,
                                    "Move down",
                                    (index + 1 == total).then_some("Already last."),
                                ) {
                                    lower = Some(index);
                                }
                                if super::menu_action(ui, "Remove", None) {
                                    remove = Some(index);
                                }
                            });
                            let edit = Action::quiet(if entry.is_none() {
                                "Replace…"
                            } else {
                                "Edit"
                            })
                            .density(Density::Read)
                            .name(&button)
                            .selected(is_open)
                            .show(ui);
                            if edit.clicked() {
                                *expanded = if is_open {
                                    None
                                } else {
                                    if let Some(act) = design::reveal::Act::clicked(ui, &edit) {
                                        draft
                                            .reveal
                                            .0
                                            .ask(act, draft.id, ("member_form", key.as_str()));
                                    }
                                    Some(key.clone())
                                };
                            }
                    })
                    .response
                    .rect;
                slot.done(
                    ui,
                    "members",
                    index,
                    egui::Rect::from_min_size(facts.min, egui::vec2(own_facts, facts.height())),
                    // The actions' line holds the count, after the name and detail lines.
                    egui::Rect::from_min_max(egui::pos2(left, count.rect.top()), count.rect.max),
                    actions,
                );
            })
            .response
            .rect;
            // The member's facts and its count line are one whole block.
            register_whole(ui, facts.union(count_line));
            // FA1, FJ6: every member fault is in the one member-fault form, with its line while
            // the members are the saved ones (a draft's lines can move).
            let line = saved_members
                .then(|| draft.member_lines.get(index).copied())
                .flatten();
            if let (None, Some((kind, cause))) = (entry, assessed.missing) {
                // The reference is on hover: it is what a repair needs, and not a word to read.
                // The repair link is a notice action, so its words start at the warning text.
                // The fault, then its help, then the repair at the words.
                let changed = member
                    .overrides
                    .get("name")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|changed| !changed.is_empty());
                // R41: the kind Save's check and the list give this member, its tone and words.
                let words = member_fault(
                    changed.unwrap_or(&name),
                    member.creature.link.is_some().then_some(name.as_str()),
                    line,
                    cause,
                );
                // M-P0 item 9: one typed body fault, its help under its words, then its repair
                // at them.
                let repair = ui.scope(|ui| {
                    design::outcome::BodyFault::new(&MemberFaultWords::new(words, kind.tone()))
                        .help(crate::sentence!(
                            "Choose a replacement, check the packages it uses, or remove this member."
                        ))
                        .repair(design::outcome::Repair::new(PACKAGES_LINK, ()))
                        .show(ui)
                        .is_some()
                });
                fault_said = Some(repair.response.rect);
                repair.response.on_hover_text(member.creature.label());
                if repair.inner {
                    ui.data_mut(|data| {
                        data.insert_temp(egui::Id::new("open_campaign_packages"), true)
                    });
                }
                if is_open && let Some(creature) = super::choose_replacement_creature(ui, snapshot)
                {
                    let chosen = snapshot
                        .catalogue
                        .resolve_ref(&creature)
                        .ok()
                        .and_then(|id| snapshot.catalogue.entries.get(&id))
                        .map_or_else(|| creature.label(), |entry| entry.entry.name.clone());
                    replaced = Some(Replaced {
                        member: member.id,
                        before: std::mem::replace(&mut member.creature, creature.clone()),
                        after: creature,
                        words: format!("Replaced {name} with {chosen}."),
                    });
                    *expanded = None;
                }
            }
            if let Some(entry) = entry {
                let fault = |cause: &str| member_fault(&name, Some(&entry.name), line, cause);
                if !is_open && let Err(error) = &assessed.changed {
                    fault_said = Some(show_change_fault(
                        ui, overrides, entry, &side, error, &fault,
                    ));
                }
                // The open form edits one member's changes: a change it writes is an edit. The form
                // says so itself; nothing here copies the changes (review of PR 299).
                if is_open {
                    let end = overrides_form(
                        ui,
                        &key,
                        &mut member.overrides,
                        entry,
                        &side,
                        None,
                        &mut draft.typed,
                    );
                    edited |= end.changed;
                    // The open form says the change's fault on its field or at its top.
                    if end.fault.is_some() {
                        fault_said = end.fault;
                    }
                    if let Some(form) = end.form {
                        draft
                            .reveal
                            .0
                            .mark(ui, draft.id, ("member_form", key.as_str()), form);
                    }
                    if end.done {
                        *expanded = None;
                        // M0251: the row comes whole into view, by its column's reveal.
                        if let Some(act) = end.done_act {
                            draft
                                .reveal
                                .0
                                .ask(act, draft.id, ("member_row", key.as_str()));
                        }
                    }
                }
            }
            design::rule(ui);
        });
        if refused == Some(identity) {
            // The fault's own words where the row says them, else the whole row.
            let said = fault_said.unwrap_or_else(|| {
                egui::Rect::from_x_y_ranges(ui.min_rect().x_range(), row_top..=ui.cursor().top())
            });
            refused_shown = ui.is_visible() && ui.clip_rect().contains_rect(said);
        }
    }
    // A move, a removal or a replacement changes the members: the comparison runs again.
    if remove.or(raise).or(lower).is_some() || replaced.is_some() {
        draft.members_saved = None;
    }
    if edited || remove.or(raise).or(lower).is_some() || replaced.is_some() {
        draft.touch();
    }
    // An open form is keyed by its member's identity: a move keeps it on that member. Only the
    // removed member's own form closes.
    if let Some(index) = remove
        && let Some(member) = draft.setup.members.get(index)
        && expanded.as_deref() == Some(member_key(&path, member).as_str())
    {
        *expanded = None;
    }
    // One result line: the newest member result takes it (the store replaces).
    if let Some(result) = replaced {
        draft.member_new = Some(MemberUndo::Replaced(result));
    }
    if let Some(index) = remove {
        remove_member(draft, index, snapshot);
    } else if let Some(index) = raise {
        draft.setup.members.swap(index, index - 1);
    } else if let Some(index) = lower {
        draft.setup.members.swap(index, index + 1);
    }
    refused_shown
}

#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
fn unread_member(
    ui: &mut egui::Ui,
    member: &splat::content::list_formats::UnreadGroupMember,
    number: usize,
    snapshot: &Snapshot,
    draft: &mut GroupDraft,
) {
    let reference: Option<EntryRef> = member
        .creature
        .clone()
        .and_then(|value| serde_yaml_ng::from_value(value).ok());
    let entry = reference
        .as_ref()
        .and_then(|reference| snapshot.catalogue.resolve_ref(reference).ok())
        .and_then(|id| snapshot.catalogue.entries.get(&id));
    let base_name = entry.map_or_else(
        || missing_member_name(None, Some(&member.label)),
        |entry| entry.entry.name.clone(),
    );
    let row = splat::content::layout::Document::new(&draft.source)
        .table(splat::content::list_formats::GROUP)
        .and_then(|table| {
            table
                .rows
                .into_iter()
                .find(|row| row[0].line == member.flag.line)
        });
    let name = row
        .as_ref()
        .and_then(|row| row.get(2))
        .map(|cell| cell.value.trim())
        .filter(|name| !name.is_empty())
        .unwrap_or(&base_name)
        .to_owned();
    let raw = row
        .as_ref()
        .and_then(|row| row.get(3))
        .map(|cell| cell.value.clone())
        .unwrap_or_default();
    ui.label(
        Type::Strong
            .rich(ui, format!("{number}. {name}"))
            .color(colors().text),
    );
    let count = member
        .count
        .map_or_else(|| "Count not read".into(), |count| format!("Count {count}"));
    let creature =
        entry.and_then(|entry| snapshot.creatures.iter().find(|c| c.id == entry.entry.id));
    let hp_missing = creature.is_some_and(|creature| creature.hp_missing);
    let mut detail = creature.map_or_else(String::new, |creature| creature.detail.clone());
    if member.count.is_none() {
        detail = if detail.is_empty() {
            count.clone()
        } else {
            format!("{count} · {detail}")
        };
    }
    if name != base_name {
        detail.push_str(&format!(" · as {base_name}"));
    }
    let summary = ui.add(
        egui::Label::new(super::detail_text(
            ui,
            &super::stat_line(&detail),
            hp_missing,
        ))
        .wrap(),
    );
    design::named(ui, &summary, &detail);
    let blocked = if splat::content::list_formats::changes(&raw, "", "").is_err() {
        "This member cannot join a fight until its Changes cell is fixed. Other members can join."
    } else {
        "This member cannot join a fight until this row is fixed. Other members can join."
    };
    // FA1: the fault in the one member-fault form, as the list, the plan and Arrivals say it;
    // the help, what the fault blocks, the file's path and its links start at its words. A flag
    // read back without its parts says its own sentence.
    let words = member.flag.cause().map_or_else(
        || member.flag.message.trim_end_matches('.').to_owned() + ".",
        |cause| member_fault(&name, Some(&base_name), Some(member.flag.line), cause),
    );
    let help = member.flag.help();
    // R41: the tone of the fault the list and Arrivals give this row (`FaultKind::Unreadable`).
    // M-P0 item 9: one typed fault, its help and what it blocks under its words, a line each,
    // then its file.
    let mut message = design::outcome::FaultMessage::<()>::new(&MemberFaultWords::new(
        words,
        FaultKind::Unreadable.tone(),
    ));
    if !help.is_empty() {
        message = message.help(super::outcome_words(help));
    }
    let message = message.help(super::outcome_words(blocked)).file(
        design::outcome::FileLine::new(&snapshot.campaign_root, &draft.path)
            .edit()
            .show(),
    );
    let _ = design::outcome::BodyMessage::from(message).show(ui);
    let line = member.flag.line;
    let mut remove = false;
    ui.horizontal(|ui| {
        ui.label(Type::Small.rich(ui, &count).color(colors().muted));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            super::row_menu(ui, &name, |ui| {
                if super::menu_action(ui, "Remove member", None) {
                    remove = true;
                }
            });
            let edit = Action::quiet("Edit")
                .density(Density::Read)
                .name(&format!("Changes for {name}"))
                .show(ui);
            if edit.clicked() {
                // M0254: the repair form comes whole into view, by its column's reveal.
                if let Some(act) = design::reveal::Act::clicked(ui, &edit) {
                    draft.reveal.0.ask(act, draft.id, ("unread_form", line));
                }
                let mut block = Mapping::new();
                if let Some(row) = &row {
                    block = splat::content::list_formats::changes("", &row[2].value, &row[4].value)
                        .unwrap_or_default();
                    // Loot is the last term, and its words can hold commas.
                    let (stats, loot) = splat::content::loot::split_changes(&raw);
                    let loot = loot.map(|loot| format!("Loot {loot}"));
                    for term in stats.split(',').chain(loot.as_deref()) {
                        if let Ok(read) = splat::content::list_formats::changes(term.trim(), "", "")
                        {
                            for (key, value) in read {
                                if key.as_str() == Some("conditions") {
                                    block
                                        .entry(key)
                                        .or_insert_with(|| Value::Sequence(Vec::new()))
                                        .as_sequence_mut()
                                        .unwrap()
                                        .extend(value.as_sequence().unwrap().iter().cloned());
                                } else {
                                    block.insert(key, value);
                                }
                            }
                        }
                    }
                }
                draft.repair = Some((line, block));
            }
        });
    });
    let mut apply = None;
    // A loot list's typing belongs to this group and this row.
    let repair_id = format!("{}:repair", draft.path());
    if let Some((repair_line, block)) = &mut draft.repair
        && *repair_line == line
    {
        // An unread value is a message, as every other unread value is: a mark and bar in the
        // unread row's tone (R41: an error), and words in text colour.
        let unread = design::outcome::row_fault(
            ui,
            &MemberFault::new(FaultKind::Unreadable, unread_changes(&raw)),
        );
        // M0254: the repair form's top, `L` above its message as the old scroll kept it.
        let mut form_top = unread.rect;
        form_top.min.y -= design::L;
        // The example belongs to the message, so it starts at the message words.
        // Layout 11c (M0255): placed by its rect at the message's measured left.
        let mut column = ui.available_rect_before_wrap();
        column.min.x = column.min.x.max(unread.rect.left());
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(column)
                .layout(egui::Layout::left_to_right(egui::Align::Min)),
            |ui| {
                ui.label(
                    Type::Small
                        .rich(ui, "For example: HP +5, AC +2, Speed 40 ft.")
                        .color(colors().muted),
                );
            },
        );
        if let Some(entry) = entry {
            let id = format!("{repair_id}:{line}");
            let end = overrides_form(
                ui,
                &id,
                block,
                &entry.entry,
                &draft.setup.side,
                None,
                &mut draft.typed,
            );
            draft.reveal.0.mark(
                ui,
                draft.id,
                ("unread_form", line),
                end.form.map_or(form_top, |form| form.union(form_top)),
            );
            if end.done {
                apply = Some(block.clone());
            }
        } else {
            body_fault(
                ui,
                &editor_fault(Tone::Warning, "Fix the creature link in Obsidian first."),
            );
        }
    }
    if (remove || apply.is_some())
        && let Err(error) = draft.repair_member(line, apply.as_ref(), snapshot)
    {
        draft.repair_error = Some(error);
    }
    if let Some(error) = &draft.repair_error {
        body_fault(ui, &editor_fault(Tone::Warning, error));
    }
    design::rule(ui);
}

/// A reveal that a draft owns (`design::reveal::Reveal`). A copy of the draft (an Undo's, a
/// discard's) starts with none: a running reveal is this draft's, never a copy's.
#[derive(Debug, Default)]
pub(super) struct DraftReveal(design::reveal::Reveal);

impl Clone for DraftReveal {
    fn clone(&self) -> Self {
        Self::default()
    }
}

/// The repair a member fault's choice asks for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MemberRepair {
    /// `Campaign → Packages`: the missing creature's package is chosen there.
    Packages,
    /// `Edit group`: the group editor.
    EditGroup,
}

#[derive(Clone, Copy)]
pub(crate) enum MemberFaultView {
    EncounterList,
    Arrivals { edit_too: bool },
}

/// Both screens use the same repair for a fault. The list fits its single repair beside
/// the words. Arrivals keeps its choices underneath and can also offer the group editor.
#[allow(
    clippy::disallowed_methods,
    reason = "a group row’s member faults stand in the row they are about, with their repairs"
)]
pub fn member_fault_row(
    ui: &mut egui::Ui,
    fault: &MemberFault,
    view: MemberFaultView,
) -> Option<MemberRepair> {
    use design::outcome::{BodyMessage, Choice, FaultMessage, MessageChoice};
    let (label, repair) = if fault.kind == FaultKind::NotInPackages {
        (PACKAGES_LINK, MemberRepair::Packages)
    } else {
        (EDIT_GROUP, MemberRepair::EditGroup)
    };
    let message = match view {
        MemberFaultView::EncounterList => BodyMessage::row_repair(fault, label, repair),
        MemberFaultView::Arrivals { edit_too } => {
            let mut message =
                FaultMessage::new(fault).choice(Choice::new(MessageChoice::Choice(label), repair));
            if edit_too && repair == MemberRepair::Packages {
                message = message.choice(Choice::new(
                    MessageChoice::Choice(EDIT_GROUP),
                    MemberRepair::EditGroup,
                ));
            }
            BodyMessage::from(message)
        }
    };
    message.show(ui)
}

/// The repair a member fault other than a missing creature offers.
pub(crate) const EDIT_GROUP: &str = "Edit group";

/// Where a missing creature's package is chosen.
pub(super) const PACKAGES_LINK: &str = "Campaign → Packages";

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

/// The repair form's reason. It names the words it could not read in curly quotation marks.
fn unread_changes(raw: &str) -> String {
    format!("Changes “{raw}” not read. Set the changes below, then save the group.")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// K3 review 1, finding 4: a change form's text row keeps the DM's exact words while it has
    /// focus, so the field's text Undo survives a word the row stores as `None` (a lone space).
    #[test]
    fn a_text_row_keeps_its_undo_through_a_blank_word() {
        const ROW: design::FieldName = crate::field_id!("test_override_row");
        let ctx = egui::Context::default();
        let field = ROW.of(Uuid::from_u128(7));
        let id = field.named_id("Display name");
        let mut value = Some(String::from("Wolf"));
        // The words typed belong to the draft (review 2, finding A): a field of the draft.
        let mut draft = GroupDraft::new();
        let mut frame = |value: &mut Option<String>, events: Vec<egui::Event>| {
            let input = egui::RawInput {
                events,
                ..Default::default()
            };
            let _ = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    text_row(ui, field, "Display name", value, "", &mut draft.typed);
                });
            });
        };
        ctx.memory_mut(|memory| memory.request_focus(id));
        frame(&mut value, Vec::new());
        // Select all and type a space: the row stores no name.
        let key = |key, modifiers| egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        };
        frame(
            &mut value,
            vec![key(egui::Key::A, egui::Modifiers::COMMAND)],
        );
        frame(&mut value, vec![egui::Event::Text(" ".into())]);
        assert_eq!(value, None, "a blank word stores no name");
        frame(&mut value, Vec::new());
        frame(
            &mut value,
            vec![key(egui::Key::Z, egui::Modifiers::COMMAND)],
        );
        assert_eq!(
            value.as_deref(),
            Some("Wolf"),
            "k3-blank: Ctrl+Z brings the name back"
        );
        // Type a space again and close the draft (Save closes it before its fields draw again)
        // while the field still has focus: the words end with the draft, and egui's map keeps
        // no copy of them.
        frame(
            &mut value,
            vec![key(egui::Key::A, egui::Modifiers::COMMAND)],
        );
        frame(&mut value, vec![egui::Event::Text(" ".into())]);
        assert!(ctx.memory(|memory| memory.has_focus(id)));
        assert!(draft.typed.holds_words(), "the draft holds the typed words");
        drop(draft);
        assert_eq!(
            ctx.data(|data| data.get_temp::<String>(id.with("typed"))),
            None,
            "k3-draft-owns-words: no typed words outlive the draft in egui's map"
        );
    }

    /// FT4: UI text names a value in curly quotation marks, never straight ones.
    #[test]
    fn a_group_name_keeps_focus_and_undo_through_its_first_save() {
        use crate::design::test_harness::ThemedPage as _;
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let mut fresh = GroupDraft::new();
        fresh.name = String::new();
        let first_path = fresh.path.clone();
        let draft = std::rc::Rc::new(std::cell::RefCell::new(fresh));
        let shown = draft.clone();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(600.0, 200.0))
            .themed(move |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    name_field(ui, &mut shown.borrow_mut(), 300.0);
                });
            });
        harness.run_steps(2);
        harness
            .get_by_role(egui::accesskit::Role::TextInput)
            .click();
        harness.run_steps(2);
        harness.event(egui::Event::Text("Rival".into()));
        harness.run_steps(2);
        assert_eq!(draft.borrow().name, "Rival");
        harness.run_steps(40);
        // The first save names the file, so the draft's path changes.
        draft.borrow_mut().check_disk(&campaign).unwrap();
        assert_ne!(draft.borrow().path, first_path, "the path changed");
        harness.run_steps(2);
        harness.event(egui::Event::Text(" Team".into()));
        harness.run_steps(2);
        assert_eq!(
            draft.borrow().name,
            "Rival Team",
            "The field keeps its focus through the save"
        );
        harness.event(egui::Event::Key {
            key: egui::Key::Z,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        });
        harness.run_steps(2);
        assert_eq!(
            draft.borrow().name,
            "Rival",
            "Undo takes back the last edit"
        );
    }

    #[test]
    fn iter_17_b_unread_changes_use_curly_quotes() {
        let words = unread_changes("+2 to hit");
        assert!(
            words.starts_with("Changes “+2 to hit” not read."),
            "{words}"
        );
        assert!(!words.contains('"'), "{words}");
    }

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

    /// A member act the body does in the next frame, before a held press is applied.
    #[derive(Clone, Copy)]
    enum BodyAct {
        Remove(usize),
        /// The member at this place moves up, as `Move up` does.
        Raise(usize),
    }

    /// The draft, its open form, the member result store, as the page holds them; a member act
    /// the body does in the next frame; and a footer press held one frame (`Some(None)` holds the
    /// next press), so that it is applied after a later body, with the ticket it was made with.
    type MembersState = (
        GroupDraft,
        Option<String>,
        design::Results<super::super::MemberOwner, super::super::MemberMark>,
        Option<BodyAct>,
        Option<Option<design::outcome::Press<super::super::FooterPress>>>,
    );

    /// The members with the editor's typed footer, as the group editor draws them: the member
    /// result is recorded in its store in the frame of the act, and its Undo is pressed through
    /// the footer and used by the press's ticket.
    fn editor_members(ui: &mut egui::Ui, state: &mut MembersState, snapshot: &Snapshot) {
        let owner = super::super::MemberOwner {
            root: std::path::PathBuf::from("campaign"),
            draft: state.0.results_key(),
        };
        let mut page = super::super::Encounters {
            // Not a folder: nothing is written.
            active: Some(campaign(&owner.root)),
            group_draft: Some(std::mem::replace(&mut state.0, GroupDraft::new())),
            member_results: std::mem::take(&mut state.2),
            ..Default::default()
        };
        // The page's own recording: the draft hands its new result, with its data, to the store.
        page.record_member_results();
        let frame = design::outcome::PaneFrame::new(ui);
        let mut messages =
            design::outcome::Messages::new(&frame, "members_test_footer", ui.available_width());
        if let Some(shown) = page.member_results.get(&owner) {
            messages.kept(shown.outcome(&frame, super::super::MemberMark::outcome));
        }
        let expanded = &mut state.1;
        let body_acts = &mut state.3;
        let draft = page.group_draft.as_mut().expect("the draft");
        let paned = messages.pane(ui, design::pane::Body::Content, |ui| {
            members(ui, draft, snapshot, expanded, None);
            // Review of PR 347: the body removes another member before a press is applied.
            match body_acts.take() {
                Some(BodyAct::Remove(index)) => remove_member(draft, index, snapshot),
                // As `members` does for `Move up`, the change flag included.
                Some(BodyAct::Raise(index)) => {
                    draft.members_saved = None;
                    draft.touch();
                    draft.setup.members.swap(index, index - 1);
                }
                None => {}
            }
        });
        // A press held from the frame before is applied after this frame's body.
        let held = match &mut state.4 {
            Some(slot @ Some(_)) => slot.take(),
            _ => None,
        };
        if let Some(press) = paned.pressed {
            if matches!(state.4, Some(None)) {
                state.4 = Some(Some(press));
            } else if let Some(ticket) = press.ticket {
                page.undo_member(ticket);
                *expanded = None;
            }
        }
        if let Some(ticket) = held.and_then(|press| press.ticket) {
            page.undo_member(ticket);
            state.4 = None;
            *expanded = None;
        }
        state.0 = page.group_draft.take().expect("the draft");
        state.2 = std::mem::take(&mut page.member_results);
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

    /// iter-26-B, review 2: a control's change that lands after the comparison, in a frame with
    /// editing input, is seen in the next frame even when that frame has no input; after it, an
    /// unchanged frame compares nothing.
    #[test]
    fn members_are_saved_sees_a_change_that_lands_late_in_an_edited_frame() {
        let ctx = egui::Context::default();
        let mut draft = GroupDraft::new();
        let member = MemberDraft {
            id: MemberId::default(),
            creature: EntryRef::id(Uuid::new_v4()),
            count: 1,
            overrides: Mapping::new(),
            extra: Mapping::new(),
        };
        draft.setup.members.push(member.clone());
        draft.saved_setup.members.push(member);
        let key = egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Num2,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        };
        let mut seen = Vec::new();
        // The edited frame compares first; the count field then changes late in it.
        let _ = ctx.run(key, |ctx| {
            seen.push(draft.members_are_saved(ctx));
            draft.setup.members[0].count = 2;
        });
        // The next frame has no input, and still compares again.
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            seen.push(draft.members_are_saved(ctx));
        });
        let before = member_comparisons();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            seen.push(draft.members_are_saved(ctx));
        });
        let idle = member_comparisons();
        assert_eq!(seen, [true, false, false], "the late change is seen");
        assert_eq!(
            (idle.0 - before.0, idle.1 - before.1),
            (1, 0),
            "an unchanged frame reaches the cache and compares nothing"
        );
    }

    #[test]
    fn iter4_b_remove_member_undo_keeps_full_member_and_other_draft_edits() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        let goblin = creature(dir.path(), "Goblin");
        let wolf = creature(dir.path(), "Wolf");
        let mut draft = GroupDraft::new();
        draft.add(goblin);
        draft.add(wolf);
        draft.setup.members[0].count = 3;
        draft.setup.members[0].overrides =
            super::super::yaml("{name: Scout, max_hp: {add: 5}, note: Keep this}")
                .as_mapping()
                .unwrap()
                .clone();
        draft.setup.members[0]
            .extra
            .insert("arrival_cue".into(), "North door".into());
        let member = draft.setup.members[0].clone();
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(880.0, 1000.0))
            .themed_state(
                move |ctx, state: &mut MembersState| {
                    egui::CentralPanel::default()
                        .show(ctx, |ui| editor_members(ui, state, &snapshot));
                },
                (draft, None, design::Results::default(), None, None),
            );
        harness.run_steps(2);
        harness.get_by_label("More actions for Scout").click();
        harness.run_steps(2);
        harness.get_by_label("Remove").click();
        harness.run_steps(2);
        assert_eq!(harness.state().0.setup.members.len(), 1);
        harness
            .state_mut()
            .0
            .notes
            .replace("A new note after removal");
        harness.state_mut().0.setup.members[0].count = 7;
        // Review of PR 347: A (Scout) is in the footer, and its Undo is pressed; the body then
        // removes B (Wolf) before that press is applied. A comes back from its own data, kept
        // with its ticket; B stays removed, with its own line.
        harness.state_mut().4 = Some(None);
        harness.get_by_label("Undo remove member").click();
        harness.run_steps(1);
        assert!(
            matches!(harness.state().4, Some(Some(_))),
            "the press is held for the next frame"
        );
        harness.state_mut().3 = Some(BodyAct::Remove(0));
        harness.run_steps(2);
        let names: Vec<_> = harness
            .state()
            .0
            .setup
            .members
            .iter()
            .map(|member| member.creature.label())
            .collect();
        assert_eq!(
            harness.state().0.setup.members,
            vec![member.clone()],
            "A comes back and B stays removed: {names:?}"
        );
        assert!(
            harness.query_all_by_label("Removed Wolf.").next().is_some()
                && harness
                    .query_all_by_label("Removed Scout.")
                    .next()
                    .is_none(),
            "B's own line shows, A's left with its data"
        );
        // B's Undo gives B back too; each kept the edits made since.
        harness.get_by_label("Undo remove member").click();
        harness.run_steps(2);
        let members = &harness.state().0.setup.members;
        assert!(
            members.len() == 2
                && members.contains(&member)
                && members.iter().any(|other| other.count == 7),
            "both are back, Wolf with its count of 7"
        );
        assert_eq!(harness.state().0.notes.as_str(), "A new note after removal");

        // A member's Other effect draft is its own, with its own cursor and text Undo: type in A
        // (twice, with time between, so A has two undo states), open B, Ctrl+Z in B's field.
        harness.get_by_label("Changes for Goblin").click();
        harness.run_steps(2);
        harness.get_by_label("Other effect").click();
        harness.run_steps(1);
        harness.get_by_label("Other effect").type_text("Haste");
        harness.run_steps(16);
        harness.get_by_label("Other effect").type_text("!");
        harness.run_steps(1);
        assert_eq!(
            harness.get_by_label("Other effect").value().as_deref(),
            Some("Haste!"),
            "A's draft was typed"
        );
        harness.get_by_label("Done with changes").click();
        // Layout PR 5b: closing A's form widens its facts; the members' column grows in the same
        // frame (the pass is discarded and drawn again), so B's Changes is where it is drawn.
        harness.run_steps(2);
        harness.get_by_label("Changes for Wolf").click();
        harness.run_steps(2);
        harness.get_by_label("Other effect").click();
        harness.run_steps(1);
        harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::Z);
        harness.run_steps(2);
        assert_eq!(
            harness
                .get_by_label("Other effect")
                .value()
                .as_deref()
                .unwrap_or(""),
            "",
            "Ctrl+Z in B's Other effect gave B the history of A"
        );

        // Review 2 of PR 347: a replacement's Undo names its member, not its place. Scout (A)
        // is replaced so that both members hold Wolf, and A's Undo arrives in the pass of a
        // body act, after it: (1) A moves up: A gets its Goblin back at its new place, Wolf (B)
        // and the new order stay; (2) A is removed: the Undo is refused and B stays.
        harness.get_by_label("Done with changes").click();
        harness.run_steps(2);
        let replace_scout = |draft: &mut GroupDraft| {
            let at = draft
                .setup
                .members
                .iter()
                .position(|member| member.count == 3)
                .expect("Scout");
            let before =
                std::mem::replace(&mut draft.setup.members[at].creature, EntryRef::id(wolf));
            draft.member_new = Some(MemberUndo::Replaced(Replaced {
                member: draft.setup.members[at].id,
                before,
                after: EntryRef::id(wolf),
                words: "Replaced Scout with Wolf.".into(),
            }));
            draft.touch();
            at
        };
        let wolf_b = |draft: &GroupDraft| {
            draft
                .setup
                .members
                .iter()
                .find(|member| member.count == 7)
                .cloned()
                .expect("Wolf (B)")
        };
        assert_eq!(replace_scout(&mut harness.state_mut().0), 1, "A is second");
        let b = wolf_b(&harness.state().0);
        harness.run_steps(2);
        let held_with = |harness: &mut egui_kittest::Harness<'_, MembersState>, act: BodyAct| {
            // The press is held, and applied after the next pass's body, which does `act`.
            harness.state_mut().4 = Some(None);
            harness.get_by_label("Undo replace member").click();
            harness.run_steps(1);
            assert!(
                matches!(harness.state().4, Some(Some(_))),
                "the press is held for the next pass"
            );
            harness.state_mut().3 = Some(act);
            harness.run_steps(1);
            assert!(harness.state().4.is_none(), "the held press was applied");
        };
        held_with(&mut harness, BodyAct::Raise(1));
        let members = harness.state().0.setup.members.clone();
        assert!(
            members.len() == 2
                && members[0].count == 3
                && members[0].creature == EntryRef::id(goblin)
                && members[1] == b
                && members[1].creature == EntryRef::id(wolf),
            "after a move, Undo replace gives A its Goblin and leaves B: {:?}",
            members
                .iter()
                .map(|member| (member.count, member.creature.label()))
                .collect::<Vec<_>>()
        );
        harness.run_steps(2);
        assert_eq!(replace_scout(&mut harness.state_mut().0), 0, "A is first");
        harness.run_steps(2);
        held_with(&mut harness, BodyAct::Remove(0));
        assert_eq!(
            harness.state().0.setup.members,
            vec![b.clone()],
            "an Undo whose member left in its pass is refused; B stays"
        );
        // M0249: the count is the shared stepper (`grouped_stepper`, as every count that a DM
        // types; a member's count stops at `MAX_COUNT`, 100): it shows the member's count, and a
        // typed count reaches the member.
        harness.state_mut().0.setup.members[0].count = 42;
        harness.run_steps(2);
        let shown = |harness: &egui_kittest::Harness<'_, _>| {
            harness
                .query_all_by_label("How many of Wolf")
                .find_map(|node| node.value())
        };
        assert_eq!(shown(&harness).as_deref(), Some("42"));
        // A pointer click on the count opens it for typing, as on the shared stepper's own test.
        let at = harness
            .query_all_by_label("How many of Wolf")
            .find(|node| node.value().is_some())
            .expect("the count's field")
            .rect()
            .center();
        harness.hover_at(at);
        harness.run_steps(1);
        for pressed in [true, false] {
            harness.event(egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
            harness.run_steps(1);
        }
        harness.run_steps(1);
        harness.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::A);
        harness.run_steps(1);
        for letter in "57".chars() {
            harness.event(egui::Event::Text(letter.to_string()));
            harness.run_steps(1);
        }
        harness.key_press(egui::Key::Enter);
        harness.run_steps(2);
        assert_eq!(harness.state().0.setup.members[0].count, 57);
        // Its + is the shared part's: one more of the member.
        harness.get_by_label("Increase How many of Wolf").click();
        harness.run_steps(2);
        assert_eq!(harness.state().0.setup.members[0].count, 58);
    }

    #[test]
    fn i20_p2_flagged_member_uses_the_normal_form_and_keeps_name_and_note() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        creature(dir.path(), "Goblin");
        let source = format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Guards\nsplat_side: enemies\n---\n\n## Members\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| 1 | [[Goblin]] | Scout | HP lots, AC +2, Blessed, Poisoned | Keep this row |\n",
            Uuid::new_v4()
        );
        fs::write(dir.path().join("Guards.md"), &source).unwrap();
        let draft = GroupDraft::open(dir.path(), "Guards.md").unwrap();
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(880.0, 1000.0))
            .themed_state(
                move |ctx, state: &mut MembersState| {
                    egui::CentralPanel::default()
                        .show(ctx, |ui| editor_members(ui, state, &snapshot));
                },
                (draft, None, design::Results::default(), None, None),
            );
        harness.run_steps(2);
        harness.get_by_label("More actions for Scout").click();
        harness.run_steps(2);
        harness.get_by_label("Remove member").click();
        harness.run_steps(2);
        harness.get_by_label("Undo remove member").click();
        harness.run_steps(2);
        harness.get_by_label("Changes for Scout").click();
        harness.run_steps(2);
        assert_eq!(
            harness.get_by_label("Display name").value().as_deref(),
            Some("Scout")
        );
        assert_eq!(
            harness.get_by_label("Note").value().as_deref(),
            Some("Keep this row")
        );
        harness.get_by_label_contains("Maximum HP: how it changes");
        harness.get_by_label("AC: amount");
        // Issue #586: the amount's steps cross zero, which changes nothing: +2, +1, −1, and back.
        let ac = |harness: &egui_kittest::Harness<'_, MembersState>| {
            // The flagged row's form edits its repair block.
            let (_, block) = harness.state().0.repair.as_ref().expect("the open repair");
            Overrides::parse(block).unwrap().armor_class
        };
        for (press, after) in [
            ("Decrease AC: amount", 1),
            ("Decrease AC: amount", -1),
            ("Increase AC: amount", 1),
            ("Decrease AC: amount", -1),
            ("Increase AC: amount", 1),
            ("Increase AC: amount", 2),
        ] {
            harness.get_by_label(press).click();
            harness.run_steps(2);
            assert_eq!(
                ac(&harness),
                Some(Change::Add(after)),
                "{press} gives {after}"
            );
        }
        // Lane E1 (ruling M20): the unread Changes cell is a typed fault at its row, in the
        // error tone (R41), not an untyped small message.
        assert!(
            design::outcome::message_rects(&harness.ctx)
                .into_iter()
                .any(
                    |(_, place, tone)| place == design::outcome::MessagePlace::Row
                        && tone == Tone::Error
                ),
            "the unread Changes cell is a typed row fault"
        );
        harness.get_by_label("Done with changes").click();
        harness.run_steps(2);
        harness.state_mut().0.save(&active, &[]).unwrap();
        let fixed = GroupDraft::open(dir.path(), "Guards.md").unwrap();
        assert!(fixed.flags.is_empty());
        let changes = Overrides::parse(&fixed.setup.members[0].overrides).unwrap();
        assert_eq!(changes.name.as_deref(), Some("Scout"));
        assert_eq!(changes.note.as_deref(), Some("Keep this row"));
        assert_eq!(changes.armor_class, Some(Change::Add(2)));
        assert_eq!(changes.conditions, vec!["Blessed", "Poisoned"]);
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
        use egui_kittest::kittest::Queryable;
        fs::remove_file(&note).unwrap();
        draft.groups[0].recovery = Default::default();
        assert!(draft.save(&campaign, &[]).is_err());
        assert!(draft.groups[0].file_gone());
        draft.groups_read = true;
        let editor = Encounters {
            // The draft's results are the campaign's (`EditorOwner`).
            active: Some(campaign.clone()),
            draft: Some(draft),
            snapshot: Some(Snapshot::load(&campaign, &[]).unwrap()),
            ..Default::default()
        };
        let mut harness = egui_kittest::Harness::builder().build_state(
            |ctx, editor: &mut Encounters| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if !editor.encounter_disk_view(ui, &campaign, &[]) {
                        // The draft's results in the editor's own footer, from the page's
                        // store; the Undo goes to the store by its ticket.
                        let frame = design::outcome::PaneFrame::new(ui);
                        let messages = editor.editor_messages(
                            &frame,
                            ui.available_width(),
                            &campaign.path,
                            None,
                            None,
                        );
                        let paned = messages.pane(ui, design::pane::Body::Content, |ui| {
                            let Encounters {
                                draft, snapshot, ..
                            } = &mut *editor;
                            if let (Some(draft), Some(snapshot)) =
                                (draft.as_mut(), snapshot.as_ref())
                            {
                                of_encounter(ui, draft, snapshot, &mut false, &mut None);
                            }
                        });
                        if let Some(press) = paned.pressed {
                            editor.editor_press(ui.ctx(), press, &campaign, &[]);
                        }
                    }
                });
                // Each frame, as the app does (review 2 of PR 347).
                editor.poll(ctx, &crate::campaigns::Campaigns::default());
            },
            editor,
        );
        harness.run_steps(2);
        harness.get_by_label("Remove group and lose draft").click();
        harness.run_steps(2);
        let draft = harness.state().draft.as_ref().unwrap();
        assert!(draft.groups.is_empty(), "the gone group left the draft");
        assert!(harness.state().error.is_none());
        // The footer's group is named with its words too.
        assert!(
            harness
                .query_all_by_label("Changed removed. Its draft was discarded.")
                .next()
                .is_some()
        );
        harness.state_mut().draft.as_mut().unwrap().name = "Renamed again".into();
        harness.run_steps(2);
        harness.get_by_label("Undo remove Changed").click();
        harness.run_steps(2);
        let draft = harness.state().draft.as_ref().unwrap();
        assert_eq!(draft.name, "Renamed again", "the unrelated edit stays");
        assert_eq!(draft.groups.len(), 1);
        assert_eq!(
            draft.groups[0].name, "Changed",
            "the group comes back with its draft"
        );
        assert!(!note.exists(), "nothing was written");

        // Review 2 of PR 347 (P2-2): the group's file changes outside Splat while the group has
        // a member result, and another group has its own. `Reload and lose my draft` reads the
        // group again (`*self = GroupDraft::open(…)`): its old result ends at the next `poll`;
        // the other group's result stays.
        let mut other = GroupDraft::for_encounter(harness.state().draft.as_ref().unwrap().id);
        other.name = "Others".into();
        other.add(scout);
        harness
            .state_mut()
            .draft
            .as_mut()
            .unwrap()
            .groups
            .push(other);
        fs::write(&note, &external).unwrap();
        let (gone, kept) = {
            let draft = harness.state_mut().draft.as_mut().unwrap();
            for group in &mut draft.groups {
                group.member_new = Some(MemberUndo::Unread {
                    line: 1,
                    row: "| 1 | x |\n".into(),
                    name: "x".into(),
                });
            }
            draft.groups[0].recovery = Default::default();
            (draft.groups[0].results_key(), draft.groups[1].results_key())
        };
        harness.run_steps(2);
        let owner = |draft| super::super::MemberOwner {
            root: campaign.path.clone(),
            draft,
        };
        let has = |harness: &egui_kittest::Harness<'_, Encounters>, draft| {
            harness.state().member_results.get(&owner(draft)).is_some()
        };
        assert!(has(&harness, gone) && has(&harness, kept), "both recorded");
        let draft = harness.state_mut().draft.as_mut().unwrap();
        assert!(
            draft.save(&campaign, &[]).is_err(),
            "the outside change blocks Save"
        );
        harness.run_steps(2);
        harness.get_by_label("Reload and lose my draft").click();
        harness.run_steps(2);
        assert_ne!(
            harness.state().draft.as_ref().unwrap().groups[0].results_key(),
            gone,
            "the group was read again"
        );
        assert!(
            !has(&harness, gone),
            "the group read again from disk ends its old result"
        );
        assert!(has(&harness, kept), "another group's result stays");
    }

    #[test]
    fn i19_e_group_comparison_matches_disk_layout_and_overwrite() {
        use egui_kittest::kittest::Queryable;
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let mut group = GroupDraft::new();
        group.name = "Scouts".into();
        group.add(creature(dir.path(), "Scout"));
        group.save(&campaign, &[]).unwrap();
        let path = dir.path().join(group.path());
        let external = format!(
            "{}\nDM's outside account.\n",
            fs::read_to_string(&path)
                .unwrap()
                .replace("splat_schema:", "tags: [dawn]\nsplat_schema:")
        );
        fs::write(&path, &external).unwrap();
        group.name = "Dawn scouts".into();
        assert!(group.save(&campaign, &[]).is_err());
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let mut harness = egui_kittest::Harness::new(|ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                assert_eq!(group.disk_choice(ui, dir.path(), &snapshot).unwrap(), None);
            });
        });
        harness.run_steps(2);
        harness.get_by_label("1 line differs");
        drop(harness);
        let editor = Encounters {
            group_draft: Some(group),
            snapshot: Some(Snapshot::load(&campaign, &[]).unwrap()),
            ..Default::default()
        };
        let mut harness = egui_kittest::Harness::builder().build_state(
            |ctx, editor: &mut Encounters| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| if !editor.group_disk_view(ui, &campaign, &[]) {});
            },
            editor,
        );
        harness.run_steps(2);
        harness.get_by_label("1 line differs");
        let comparison = harness
            .state()
            .group_draft
            .as_ref()
            .unwrap()
            .comparison_text(&snapshot)
            .unwrap();
        assert_eq!(
            comparison,
            external.replace("splat_name: Scouts", "splat_name: Dawn scouts")
        );
        let overwrite = splat::content::list_formats::write_references(
            &comparison,
            snapshot
                .catalogue
                .entries
                .values()
                .map(|entry| &entry.entry),
        )
        .unwrap();
        assert!(comparison.contains("tags: [dawn]"));
        assert!(comparison.contains("DM's outside account."));
        harness.get_by_label("Overwrite with my draft").click();
        harness.run_steps(2);
        assert_eq!(harness.state().error, None);
        assert_eq!(fs::read_to_string(&path).unwrap(), overwrite);
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
