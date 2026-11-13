//! Encounter groups: their notes, their editor, and the overrides form they share with the
//! roster. A group is prepared here and added to a run from the run's own panel.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use super::{
    ADD_PANE_MAX_WIDTH, Creature, Draft, EditorPane, Encounters, NAME_FIELD, ROSTER_MAX_WIDTH,
    Snapshot,
};
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue, EntryRef},
    design::{self, Action, Density, Status, Tone, Type, colors, icons},
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
    overrides::{Base, Change, MAX_NAME, Overrides, same_condition},
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
/// The Groups list's heading for groups whose encounter is gone; Arrivals says the same words
/// beside such a group's name (FB10).
pub(crate) const ENCOUNTER_DELETED: &str = "Encounter deleted";
const NOTES_FIELD: &str = "Group notes";
const LABEL_WIDTH: f32 = 120.0;
/// The name of a roster slot's button, for the form it opens.
pub(super) const SLOT_OPTIONS: &str =
    "Options: starting HP, temporary HP, initiative group and changes";

fn one() -> u64 {
    1
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct MemberDraft {
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
const EMPTY_REASON: &str = "Add a creature first";

#[derive(Clone)]
pub(super) struct GroupDraft {
    recovery: super::disk::Recovery,
    path: String,
    baseline: Option<String>,
    source: String,
    /// The line of each readable member in `source`, read when `source` or `flags` change (never
    /// per frame). A fault names its line only while the members match the saved ones (FJ6).
    member_lines: Vec<usize>,
    name: String,
    setup: GroupSetup,
    notes: String,
    saved_name: String,
    saved_setup: GroupSetup,
    saved_notes: String,
    flags: Vec<splat::content::layout::Flag>,
    repair: Option<(usize, Mapping)>,
    removed_unread: Option<(usize, String, String)>,
    removed_member: Option<(usize, MemberDraft, String)>,
    /// The last `Use` on a member this campaign could not find. It owns the result line until
    /// its Undo, a newer member result, or a move or removal that changes the members' places.
    replaced: Option<Replaced>,
    /// When the newest member result was made (`result_stamp`); `None` before the first.
    member_stamp: Option<design::ResultOrder>,
    /// Whether the members are the saved ones, worked out once after each change (FJ6); `None`
    /// until then. Editing input (this frame or the one before, as a click's change lands late)
    /// and the events that change the members or the saved ones clear it.
    members_saved: Option<bool>,
    edited_last_frame: bool,
    repair_error: Option<String>,
}

/// A member's creature before and after `Use`, with the result's words.
#[derive(Clone)]
struct Replaced {
    index: usize,
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
            path: format!("{FOLDER}/{id}.md"),
            baseline: None,
            source: format!(
                "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: encounter_group\nsplat_name: New group\nsplat_side: enemies\n---\n\n## Members\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n"
            ),
            member_lines: Vec::new(),
            name: name.clone(),
            saved_name: name,
            setup: setup.clone(),
            saved_setup: setup,
            notes: String::new(),
            saved_notes: String::new(),
            flags: Vec::new(),
            repair: None,
            removed_unread: None,
            removed_member: None,
            replaced: None,
            member_stamp: None,
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
            recovery: Default::default(),
            path: path.into(),
            baseline: Some(text.clone()),
            member_lines: splat::content::list_formats::group_member_lines(&text, &entry.flags),
            source: text,
            flags: entry.flags.clone(),
            repair: None,
            removed_unread: None,
            removed_member: None,
            replaced: None,
            member_stamp: None,
            members_saved: None,
            edited_last_frame: false,
            repair_error: None,
            saved_name: entry.name.clone(),
            name: entry.name,
            saved_setup: setup.clone(),
            setup,
            saved_notes: notes.clone(),
            notes,
        })
    }
    pub(super) fn name(&self) -> String {
        self.name.clone()
    }
    /// Give this group the owner `earlier` had, and change nothing else: Undo of a move.
    pub(super) fn take_owner_from(&mut self, earlier: &GroupDraft) {
        self.setup.encounter = earlier.setup.encounter.clone();
    }
    /// No member that Save would write: Save refuses the group.
    pub(super) fn is_empty(&self) -> bool {
        self.setup.members.is_empty()
    }
    /// The member result that shows now, with its Undo: the newest of a replacement, a removed
    /// member and a removed unread row owns it. A replacement's ends when its member no longer
    /// holds the chosen creature.
    pub(super) fn member_result(&mut self) -> Option<(String, &'static str, design::ResultOrder)> {
        if let Some(result) = &self.replaced {
            if self
                .setup
                .members
                .get(result.index)
                .is_some_and(|member| member.creature == result.after)
            {
                // Each member result is made with its stamp, so a shown one has it.
                return self
                    .member_stamp
                    .map(|stamp| (result.words.clone(), "Undo replacement", stamp));
            }
            self.replaced = None;
        }
        self.removed_member
            .as_ref()
            .map(|(_, _, name)| name)
            .or(self.removed_unread.as_ref().map(|(_, _, name)| name))
            .zip(self.member_stamp)
            .map(|(name, stamp)| (format!("Removed {name}."), "Undo remove member", stamp))
    }
    /// Undo the member result that shows: the creature, the member or the unread row comes
    /// back at its place, and other draft edits stay.
    pub(super) fn undo_member_result(&mut self) {
        self.members_saved = None;
        if let Some(result) = self.replaced.take() {
            if let Some(member) = self.setup.members.get_mut(result.index) {
                member.creature = result.before;
            }
        } else if let Some((index, member, _)) = self.removed_member.take() {
            self.setup
                .members
                .insert(index.min(self.setup.members.len()), member);
        } else if let Some((line, row, _)) = self.removed_unread.take() {
            let at = self
                .source
                .split_inclusive('\n')
                .take(line - 1)
                .map(str::len)
                .sum();
            self.source.insert_str(at, &row);
            if let Ok(entry) = Entry::parse(&self.path, &self.source) {
                self.flags = entry.flags;
            }
            self.reread_lines();
        }
    }
    /// Whether no member has moved, been added, changed or removed since the note was read.
    /// The comparison runs only after a change (`members_saved`); each frame reaches it.
    fn members_are_saved(&mut self, ctx: &egui::Context) -> bool {
        let edited = super::edit_input(ctx);
        let edited_before = std::mem::replace(&mut self.edited_last_frame, edited);
        if edited || edited_before {
            self.members_saved = None;
        }
        #[cfg(test)]
        MEMBER_COMPARISONS.with(|counts| counts.set((counts.get().0 + 1, counts.get().1)));
        *self.members_saved.get_or_insert_with(|| {
            #[cfg(test)]
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
    pub(super) fn dirty(&self) -> bool {
        self.baseline.is_none()
            || self.name != self.saved_name
            || self.setup != self.saved_setup
            || self.notes != self.saved_notes
            || self.baseline.as_deref() != Some(self.source.as_str())
    }
    fn text(&self) -> Result<String, String> {
        let disk_source = self
            .recovery
            .change
            .as_ref()
            .and_then(|change| change.text.as_deref())
            .unwrap_or(&self.source);
        let source = if self.notes == self.saved_notes {
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
            self.removed_member = None;
            self.replaced = None;
            self.member_stamp = Some(super::result_stamp());
            self.removed_unread = Some((
                line,
                source[start..end].to_owned(),
                if member.name.is_empty() {
                    member.label.clone()
                } else {
                    member.name.clone()
                },
            ));
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
        self.members_saved = None;
        self.setup.members.push(MemberDraft {
            extra: Mapping::new(),
            creature: EntryRef::id(id),
            count: 1,
            overrides: Mapping::from_iter([("name".into(), name.into())]),
        });
    }
    /// An encounter being saved with this group counts as there, though it is not on disk yet.
    pub(super) fn validate(&self, snapshot: &Snapshot, saving: Option<Uuid>) -> Result<(), String> {
        let name = self.name.trim();
        if name.is_empty() {
            return Err("Enter a group name.".into());
        }
        // A plan links a group for any encounter by its name, so that name must mean one thing.
        // A group that belongs to an encounter is not linked, and each can have "Reinforcements".
        let linkable = self.setup.encounter.is_none();
        if linkable
            && snapshot.groups.iter().any(|group| {
                group.encounter.is_none()
                    && group.path != self.path
                    && group.name.eq_ignore_ascii_case(name)
            })
        {
            return Err(format!(
                "Another group is named {name}. Choose another name."
            ));
        }
        if linkable
            && snapshot
                .templates
                .iter()
                .any(|template| template.name.eq_ignore_ascii_case(name))
        {
            return Err(format!(
                "An encounter is named {name}. Choose another name."
            ));
        }
        if self.setup.side.trim().is_empty() {
            return Err("Enter a default side for this group.".into());
        }
        if self.setup.members.is_empty() {
            return Err("This group is empty. Add at least one creature.".into());
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
            return Err(format!(
                "This campaign no longer has the encounter this group belongs to. Under {BELONGS_TO}, choose another, or {ANY_ENCOUNTER}."
            ));
        }
        for member in &self.setup.members {
            // FA1: Save's check names a member as every view does. A draft's lines may have
            // moved since the note was read, so it names no line.
            let changed = member
                .overrides
                .get("name")
                .and_then(Value::as_str)
                .map(str::to_owned);
            let resolved_id = snapshot.catalogue.resolve_ref(&member.creature);
            let base = resolved_id
                .as_ref()
                .ok()
                .and_then(|id| snapshot.catalogue.entries.get(id))
                .map(|entry| entry.entry.name.clone());
            let shown = match (&changed, &base) {
                (Some(changed), _) if !changed.trim().is_empty() => changed.trim().to_owned(),
                (_, Some(base)) => base.clone(),
                _ => missing_member_name(None, member.creature.link.as_deref()),
            };
            let fault = |cause: &str| member_fault(&shown, base.as_deref(), None, cause);
            if !(1..=MAX_COUNT).contains(&member.count) {
                return Err(fault(&format!("count must be from 1 to {MAX_COUNT}")));
            }
            let id = resolved_id.map_err(|_| fault("not in this campaign’s packages"))?;
            let creature = snapshot
                .creatures
                .iter()
                .find(|c| c.id == id)
                .ok_or_else(|| fault("this creature is unavailable in campaign content"))?;
            if creature.linked {
                return Err(fault(LINKED_CAUSE));
            }
            let overrides = Overrides::parse(&member.overrides).map_err(|e| fault(&e))?;
            let resolved = overrides
                .resolve_for(&snapshot.catalogue.entries[&id].entry, &self.setup.side)
                .map_err(|e| fault(&e))?;
            if resolved.max_hp.is_none() {
                return Err(fault(NO_MAX_HP_CAUSE));
            }
        }
        Ok(())
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
        if self.baseline.is_none() {
            self.path = super::named_note_path(&campaign.path, FOLDER, &self.name)?;
        }
        self.recovery.check(
            &campaign.path,
            &mut self.path,
            self.baseline.as_deref(),
            "group",
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
        let choice = self
            .recovery
            .show(ui, &self.comparison_text(snapshot)?, "group");
        if let Some(keep) = choice {
            if keep {
                if let Some(change) = self.recovery.accept() {
                    self.path = change.path;
                    self.baseline = change.text.clone();
                    if let Some(text) = change.text {
                        self.source = text;
                        self.reread_lines();
                    }
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
        self.recovery = Default::default();
        self.source = text.to_owned();
        if let Ok(entry) = Entry::parse(&self.path, text) {
            self.flags = entry.flags;
        }
        self.reread_lines();
        self.baseline = Some(text.to_owned());
        self.saved_name = self.name.clone();
        self.saved_setup = self.setup.clone();
        self.notes = self.notes.trim().to_owned();
        self.saved_notes = self.notes.clone();
    }
}

/// A creature's name in an editor: the one it will have. Its changes say which entry it is.
pub(super) fn shown_name(block: &Mapping, name: String) -> String {
    Overrides::parse(block)
        .ok()
        .and_then(|overrides| overrides.name)
        .map_or(name, |changed| changed.trim().to_owned())
}

/// What the DM can do about a member whose creature this campaign cannot find.
const MISSING_MEMBER_HELP: &str =
    "Choose a replacement, check the packages it uses, or remove this member.";
/// A group's own reason when it has no member fault to show.
const NONE_CAN_JOIN: &str = "No members can join.";
/// Why a member with a D&D Beyond sheet cannot join from a group.
const LINKED_CAUSE: &str = "it has a D&D Beyond sheet, so it joins from the roster";
/// Why a member with no maximum HP cannot join. Its Edit button sets one.
pub(crate) const NO_MAX_HP_CAUSE: &str =
    "it has no maximum HP. Set it with this member’s Edit button";

/// One member's fault, in the one form every view uses (FA1, FC5): `<shown name> (as <base>) ·
/// line N: <cause>.` The Encounters list, the group editor, the session plan, Save's check and
/// Arrivals all take their words from here. `(as <base>)` is left out when the member keeps its
/// base name or the base is not known; `line N` when the note gives the member no line.
pub(crate) fn member_fault(
    shown: &str,
    base: Option<&str>,
    line: Option<usize>,
    cause: &str,
) -> String {
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
    format!("{text}: {}.", cause.trim().trim_end_matches('.'))
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
        let Some((id, creature)) = catalogue
            .resolve(&member.creature)
            .ok()
            .and_then(|id| Some((id, creatures.iter().find(|c| c.id == id)?)))
        else {
            let link = entry
                .fields
                .get("splat_encounter_group")
                .and_then(|group| group.get("members"))
                .and_then(|members| members.get(index))
                .and_then(|member| member.get("creature"))
                .and_then(|creature| creature.get("link"))
                .and_then(Value::as_str);
            let name = missing_member_name(member.overrides.name.as_deref(), link);
            // The base is the name the note links, when it gives one.
            let base = missing_member_name(None, link);
            warnings.push(MemberFault::new(
                FaultKind::NotInPackages,
                member_fault(
                    &name,
                    link.is_some().then_some(base.as_str()),
                    line,
                    "not in this campaign’s packages",
                ),
            ));
            unavailable_members.push((index, member.count, name.to_owned()));
            unavailable = unavailable.saturating_add(member.count);
            continue;
        };
        let entry = &catalogue.entries[&id].entry;
        let resolved = member.overrides.resolve_for(entry, &group.side);
        let shown = member.overrides.name.as_deref().unwrap_or(&entry.name);
        let member_name = if shown == entry.name {
            shown.to_owned()
        } else {
            format!("{shown} (as {})", entry.name)
        };
        let fault = |cause: &str| member_fault(shown, Some(&entry.name), line, cause);
        // The kind is decided here, where the fault is found; every view takes its tone.
        let issue = if creature.linked {
            Some(MemberFault::new(FaultKind::CannotJoin, fault(LINKED_CAUSE)))
        } else {
            match &resolved {
                Err(error) => Some(MemberFault::new(
                    FaultKind::InvalidChange,
                    fault(&override_error(error)),
                )),
                Ok(resolved) if resolved.max_hp.is_none() => Some(MemberFault::new(
                    FaultKind::CannotJoin,
                    fault(NO_MAX_HP_CAUSE),
                )),
                Ok(_) => None,
            }
        };
        if let Some(issue) = issue {
            warnings.push(issue);
            unavailable = unavailable.saturating_add(member.count);
            unavailable_members.push((index, member.count, member_name));
            continue;
        }
        match creature_xp(entry) {
            Some(each) => xp = u64::saturating_add(xp, each.saturating_mul(member.count)),
            None => xp_unknown.push(entry.name.clone()),
        }
        members.push(GroupMember {
            entry: Arc::new(catalogue.run_entry(id)),
            count: member.count,
            overrides: member.overrides.clone(),
        });
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
            FaultKind::CannotJoin,
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

/// Group actions use the encounter ledger's right-hand columns.
fn list_row(
    ui: &mut egui::Ui,
    group: &RunGroup,
    blocked: Option<&'static str>,
    action: &mut Option<super::ListAction>,
) {
    let summary = group.summary();
    super::list::list_row(
        ui,
        group.id,
        &group.name,
        &format!(
            "{}{}",
            if group.encounter.is_some() && group.encounter_name.is_none() {
                "Offered in no fight · "
            } else {
                ""
            },
            summary
        ),
        false,
        None,
        |ui| {
            super::row_menu(ui, &group.name, |ui| {
                if super::menu_action(ui, "Remove…", blocked) {
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
    for warning in &group.warnings {
        if member_fault_row(ui, warning) {
            if warning.kind == FaultKind::NotInPackages {
                ui.data_mut(|data| data.insert_temp(egui::Id::new("open_campaign_packages"), true));
            } else {
                *action = Some(super::ListAction::OpenGroup(group.path.clone()));
            }
        }
    }
    // A group with no member that can join says so after its member faults, as a message in
    // the same form as the faults above it.
    for reason in group.own_problems() {
        design::message(ui, Tone::Warning, reason);
    }
}

/// One flat set of group headings inside the ledger's Groups section.
pub(super) fn list(
    ui: &mut egui::Ui,
    snapshot: &Snapshot,
    blocked: Option<&'static str>,
    action: &mut Option<super::ListAction>,
) {
    let any: Vec<_> = snapshot
        .groups
        .iter()
        .filter(|group| group.encounter.is_none())
        .collect();
    if !any.is_empty() {
        design::group_heading(ui, "For any encounter", any.len());
        for group in any {
            list_row(ui, group, blocked, action);
        }
    }
    for (id, name) in snapshot.encounter_choices() {
        let own: Vec<_> = snapshot
            .groups
            .iter()
            .filter(|group| group.encounter == Some(id))
            .collect();
        if !own.is_empty() {
            design::group_heading(ui, &format!("For {name}"), own.len());
            for group in own {
                list_row(ui, group, blocked, action);
            }
        }
    }
    let orphans: Vec<_> = snapshot
        .groups
        .iter()
        .filter(|group| group.encounter.is_some() && group.encounter_name.is_none())
        .collect();
    if !orphans.is_empty() {
        design::group_heading(ui, ENCOUNTER_DELETED, orphans.len());
        for group in orphans {
            list_row(ui, group, blocked, action);
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
            "group",
        ) {
            if keep {
                if let Some(change) = draft.recovery.accept() {
                    draft.path = change.path;
                    draft.baseline = change.text.clone();
                    if let Some(text) = change.text {
                        draft.source = text;
                        draft.reread_lines();
                    }
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
    ui.add_space(design::M);
    design::group_heading(
        ui,
        "Groups for this encounter",
        draft
            .groups
            .iter()
            .filter(|group| group.belongs_to(draft.id))
            .count(),
    );
    design::help(
        ui,
        "They arrive when you add them to a run of this encounter. Save encounter saves them too.",
    );
    ui.add_space(design::XS);
    let reveal = std::mem::take(&mut draft.reveal_group);
    let Draft {
        groups,
        add_to,
        focus_group,
        removed_group,
        deleted_groups,
        ..
    } = draft;
    for (index, group) in groups.iter_mut().enumerate() {
        if group.setup.encounter.as_ref().and_then(|owner| owner.id) != Some(this.0) {
            continue;
        }
        let path = group.path.clone();
        let adding = add_to.as_deref() == Some(path.as_str());
        // The editor's usable area: `Move to…` opens inside it, clear of the scroll bar and the
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
                                design::menu_submenu(ui, "Move to…", inside, |ui| {
                                    for (id, name) in
                                        snapshot.encounter_choices().filter(|(id, _)| *id != this.0)
                                    {
                                        if super::menu_action(ui, name, None) {
                                            group.setup.encounter = Some(EntryRef::id(id));
                                        }
                                    }
                                });
                                if super::menu_action(ui, "Make available to any encounter", None) {
                                    discard = Some((index, false));
                                }
                                if super::menu_action(ui, "Delete group…", None) {
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
                                    let response = ui.add(
                                        egui::TextEdit::singleline(&mut group.name)
                                            .desired_width(
                                                (ui.available_width() - 2.0 * design::S)
                                                    .min(design::FORM_WIDTH),
                                            )
                                            .margin(egui::vec2(design::S, 4.0))
                                            .font(Type::Heading.font(ui)),
                                    );
                                    design::named(
                                        ui,
                                        &response,
                                        &format!("Group name: {}", group.name),
                                    );
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
                        false,
                        name_right,
                    );
                });
            if adding && reveal {
                frame.response.scroll_to_me(Some(egui::Align::Max));
            }
            ui.add_space(design::S);
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
        }
        *removed_group = Some((index, gone, delete, super::result_stamp()));
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
/// and under the name it has now.
fn belongs_to(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    open: Option<(Uuid, &str)>,
    name_right: f32,
) {
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
    ui.horizontal(|ui| {
        choice_label(ui, BELONGS_TO);
        // The choice ends where the name field ends, inside the column.
        let right = name_right.min(ui.max_rect().right());
        let width = (right - ui.cursor().left()).max(Density::Scan.control());
        // FB8: egui keeps a popup's first size, so a menu opened at one width would stay that
        // wide after a resize. When the control's width changes while its menu is open, the
        // remembered sizes go, and the menu takes the control's width again. Only one popup is
        // open at a time, and Splat has no window that keeps a place.
        let key = width.round() as i32;
        let last_id = ui.make_persistent_id("group_encounter_width");
        // The combo's own popup id: its button is `make_persistent_id(Id::new(salt))`.
        let popup = ui
            .make_persistent_id(egui::Id::new("group_encounter"))
            .with("popup");
        if ui
            .data(|data| data.get_temp::<i32>(last_id))
            .is_some_and(|last| last != key)
            && egui::Popup::is_id_open(ui.ctx(), popup)
        {
            ui.ctx().memory_mut(egui::Memory::reset_areas);
        }
        ui.data_mut(|data| data.insert_temp(last_id, key));
        let was_open = egui::Popup::is_id_open(ui.ctx(), popup);
        let combo = egui::ComboBox::from_id_salt("group_encounter")
            .icon(design::combo_icon)
            .width(width)
            .truncate()
            .selected_text(shown)
            .show_ui(ui, |ui| {
                // FB8: the menu is as wide as its control, as Library's scope menu is. Its
                // content is the control's width less the menu's frame; a long name wraps.
                let frame = ui.style().spacing.menu_margin.sum().x
                    + 2.0 * ui.style().visuals.window_stroke.width;
                let inner = (width - frame).max(0.0);
                ui.set_min_width(inner);
                ui.set_max_width(inner);
                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                let any = ui.selectable_label(current.is_none(), ANY_ENCOUNTER);
                crate::rolls::menu_choice_focus(ui, &any);
                if any.clicked() {
                    draft.setup.encounter = None;
                }
                for (id, name) in &choices {
                    // The same choice again keeps a link that the note gives.
                    let choice = ui.selectable_label(current == Some(*id), *name);
                    crate::rolls::menu_choice_focus(ui, &choice);
                    if choice.clicked() && current != Some(*id) {
                        draft.setup.encounter = Some(EntryRef::id(*id));
                    }
                }
            });
        // The menu's focus model (the run's menus use it too): the control owns its menu while it
        // is open, and when the menu closes by the keys or a choice the focus comes back to it;
        // a click on another control keeps the focus there.
        crate::rolls::keep_menu_owner(&combo.response);
        if !was_open && egui::Popup::is_id_open(ui.ctx(), popup) {
            let by_mouse = ui.ctx().input(|input| input.pointer.any_click());
            crate::rolls::own_menu(&combo.response, !by_mouse);
        }
        design::named(ui, &combo.response, &format!("{BELONGS_TO}: {shown}"));
    });
    // In an encounter's editor, belonging to it goes without saying.
    if current.is_some() && current == open.map(|(id, _)| id) {
        return;
    }
    let help = match current {
        None => "A run of any encounter offers this group.",
        // A group no fight can offer is a fault, not field help.
        Some(_) if shown == MISSING_ENCOUNTER => {
            design::small_message(
                ui,
                Tone::Warning,
                "No fight offers this group now. Choose an encounter, or Any encounter.",
            );
            return;
        }
        Some(_) => "Only runs of this encounter offer this group.",
    };
    design::help(ui, help);
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

pub(super) fn changes_link(
    ui: &mut egui::Ui,
    block: &Mapping,
    entry: &Entry,
    side: &str,
    form_open: bool,
) -> bool {
    if block.is_empty() || form_open {
        return false;
    }
    match Overrides::parse(block) {
        Ok(overrides) => {
            let rows = overrides.rows(&Base::of(entry, side));
            let missing = crate::loot_editor::missing_in(ui.ctx(), overrides.loot.as_deref());
            let word = change_words(&overrides, entry, side, &missing);
            let clicked = design::link(ui, &word).clicked();
            if let Err(error) = overrides.resolve_for(entry, side) {
                design::message(ui, FaultKind::InvalidChange.tone(), &override_error(&error));
                // The failed change stays visible beside its fault.
                change_rows(ui, &rows);
            }
            clicked
        }
        Err(error) => {
            design::message(ui, FaultKind::InvalidChange.tone(), &override_error(&error));
            false
        }
    }
}

const FORM_OPENED: &str = "override_form_opened";

/// The form can open below the fold, so the next frame scrolls to it.
pub(super) fn show_form_when_it_opens(ui: &egui::Ui) {
    ui.data_mut(|data| data.insert_temp(egui::Id::new(FORM_OPENED), true));
}

/// A number that can stay the same, be set, or change by an amount. Each control keeps the
/// override valid on its own, so that the form never holds a block that it cannot read.
fn change_row(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<Change>,
    set: Option<std::ops::RangeInclusive<i64>>,
    add: std::ops::RangeInclusive<i64>,
    base: &Base,
) {
    let mut fault = None;
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
        let word = match value {
            None => "Same",
            Some(Change::Set(_)) => "Set to",
            Some(Change::Add(_)) => "Add",
        };
        let low = set.as_ref().map_or(1, |range| *range.start().max(&1));
        let combo = egui::ComboBox::from_id_salt(label)
            .icon(design::combo_icon)
            .width(86.0)
            .selected_text(word)
            .show_ui(ui, |ui| {
                if ui.selectable_label(value.is_none(), "Same").clicked() {
                    *value = None;
                }
                if set.is_some()
                    && ui
                        .selectable_label(matches!(value, Some(Change::Set(_))), "Set to")
                        .clicked()
                {
                    *value = Some(Change::Set(low));
                }
                let add_option = ui.selectable_label(matches!(value, Some(Change::Add(_))), "Add");
                design::named(ui, &add_option, &format!("{label}: Add"));
                if add_option.clicked() {
                    *value = Some(Change::Add(1));
                }
            });
        design::named(ui, &combo.response, &format!("{label}: how it changes"));
        match value {
            Some(Change::Set(number)) => {
                let range = set.clone().unwrap_or(0..=0);
                let response = ui.add(egui::DragValue::new(number).range(range));
                design::named(ui, &response, &format!("{label}: value"));
            }
            Some(Change::Add(number)) => {
                let mut amount = *number;
                let response = ui.add(egui::DragValue::new(&mut amount).range(add));
                design::named(ui, &response, &format!("{label}: amount"));
                // Zero changes nothing, so the choice for that is Same.
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
    if let Some(error) = fault {
        ui.horizontal(|ui| {
            ui.add_space(LABEL_WIDTH + ui.spacing().item_spacing.x);
            ui.vertical(|ui| {
                design::small_message(ui, FaultKind::InvalidChange.tone(), &override_error(&error));
            });
        });
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

/// One line of words, or nothing. Blank means no change.
fn text_row(ui: &mut egui::Ui, label: &str, value: &mut Option<String>, hint: &str) {
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
        let mut text = value.clone().unwrap_or_default();
        let width = text_field_width(ui.available_width());
        if design::text_input(ui, label, &mut text, hint, width).changed() {
            *value = (!text.trim().is_empty()).then_some(text);
        }
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
pub(super) fn overrides_form(
    ui: &mut egui::Ui,
    id: &str,
    block: &mut Mapping,
    entry: &Entry,
    side: &str,
    sides: Option<&[String]>,
) -> bool {
    let mut overrides = match Overrides::parse(block) {
        Ok(overrides) => overrides,
        Err(error) => {
            design::message(
                ui,
                Tone::Error,
                &format!("{error} Repair it in Campaign → Campaign entries."),
            );
            return false;
        }
    };
    let base = Base::of(entry, side);
    let before = overrides.clone();
    let linked = entry.fields.contains_key("splat_external");
    ui.add_space(design::XS);
    let mut done = false;
    let heading = ui
        .horizontal(|ui| {
            let heading = design::group_label(
                ui,
                &format!("Changes for {}", shown_name(block, entry.name.clone())),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                done = Action::quiet("Done")
                    .name("Done with changes")
                    .show(ui)
                    .clicked();
            });
            heading
        })
        .inner;
    if ui
        .data_mut(|data| data.remove_temp::<bool>(egui::Id::new(FORM_OPENED)))
        .is_some()
    {
        let mut rect = heading.rect;
        rect.min.y -= design::S;
        ui.scroll_to_rect(rect, Some(egui::Align::Min));
    }
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
    if let Err(error) = overrides.resolve_for(entry, side)
        && !on_a_field
    {
        design::message(ui, FaultKind::InvalidChange.tone(), &override_error(&error));
    }
    text_row(ui, "Display name", &mut overrides.name, &entry.name);
    // Only a name just typed: opening the form must not edit what a note holds.
    if overrides.name != before.name {
        overrides.name = overrides
            .name
            .take()
            .map(|name| name.trim_start().chars().take(MAX_NAME).collect());
    }
    // A linked character's sheet decides these. One already here can still be cleared.
    if !linked || overrides.max_hp.is_some() {
        change_row(
            ui,
            "Maximum HP",
            &mut overrides.max_hp,
            Some(1..=1_000_000),
            -1_000_000..=1_000_000,
            &base,
        );
    }
    change_row(
        ui,
        "AC",
        &mut overrides.armor_class,
        Some(0..=1000),
        -1000..=1000,
        &base,
    );
    change_row(
        ui,
        "Initiative bonus",
        &mut overrides.initiative_bonus,
        Some(-1000..=1000),
        -1000..=1000,
        &base,
    );
    for (label, bonus) in [
        ("Attack bonus", &mut overrides.attack_bonus),
        ("Damage bonus", &mut overrides.damage_bonus),
    ] {
        let mut value = bonus.map(Change::Add);
        change_row(ui, label, &mut value, None, -1000..=1000, &base);
        *bonus = match value {
            Some(Change::Add(amount)) => Some(amount),
            _ => None,
        };
    }
    change_row(
        ui,
        "Speed",
        &mut overrides.speed,
        Some(0..=1000),
        -1000..=1000,
        &base,
    );
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
                super::side_control(ui, "Changed side", &mut changed, sides);
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
                super::side_control(ui, "Changed side", &mut changed, &[]);
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
        // The run's condition form width: the grid, the effects and Add stay within it.
        ui.scope(|ui| {
            ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
            conditions_row(ui, &mut overrides.conditions);
        });
    }
    text_row(ui, "Note", &mut overrides.note, "Why this creature differs");
    // A group of rows, as Starting conditions is.
    design::group_label(ui, "Loot");
    // The loot field ends at the form's edge; `Roll on a table` follows it.
    let loot_width = form_edge(ui)
        + ui.spacing().item_spacing.x
        + design::menu_button_width(ui, crate::loot_editor::ROLL_MENU);
    ui.scope(|ui| {
        ui.set_max_width(loot_width.min(ui.available_width()));
        crate::loot_editor::show_cell(
            ui,
            id,
            &shown_name(block, entry.name.clone()),
            &mut overrides.loot,
        );
    });
    if overrides != before {
        *block = overrides.to_mapping();
    }
    done
}

/// The same official names and local Other effect as the run.
fn conditions_row(ui: &mut egui::Ui, conditions: &mut Vec<String>) {
    design::group_label(ui, "Starting conditions");
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
    let id = ui.id().with("starting_other_effect");
    let mut text = ui
        .data(|data| data.get_temp::<String>(id))
        .unwrap_or_default();
    if crate::encounter_run::view::other_effect_field(
        ui,
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
    ui.data_mut(|data| data.insert_temp(id, text));
}

impl Encounters {
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
        if let Some(error) = self
            .snapshot
            .as_ref()
            .and_then(|snapshot| draft.validate(snapshot, None).err())
        {
            self.refused(Some(error.clone().into()));
            self.error = Some(error);
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
                self.message_order = Some(design::ResultOrder::issue());

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
                let invalid = self
                    .snapshot
                    .as_ref()
                    .and_then(|snapshot| draft.validate(snapshot, None).err())
                    .filter(|validation| *validation == e)
                    .map(Into::into);
                self.refused(invalid);
                self.error = Some(e);
            }
        }
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
        // The Session page has no save of its own to give this key to.
        let save_key = in_plan
            && self
                .group_draft
                .as_ref()
                .is_some_and(|draft| !draft.setup.members.is_empty())
            && dirty
            && ui
                .ctx()
                .input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S));
        let (back_word, back_name) = if in_plan {
            ("Plan", "Back to the plan")
        } else {
            ("Encounters", "Back to encounters")
        };
        let compact = ui.available_width() < 760.0;

        let saved_note = self.message.clone();
        // AH1: a draft that Save's check refused says Invalid, as the encounter editor does.
        let refused = self.shows_validation_error();
        let focus_name = std::mem::take(&mut self.focus_name);
        let draft = self.group_draft.as_mut().expect("the editor has a draft");
        let column_right = ui.data(|data| data.get_temp::<f32>(column_right_id(&draft.path)));
        let (mut back, mut save, mut discard) = (false, false, false);
        let mut name_right = f32::INFINITY;
        // AK1, AB2: the header is one block that ends where the members column ends. Save group
        // sits at that edge on the save row, above the member actions and beside the save state
        // and Discard, never across the Add creatures column. The name keeps its measure, so it
        // still ends with `Belongs to` and, in a narrow column, at the column's edge. Before the
        // column is first drawn (and in the compact Add pane), the page's edge stands in for it.
        let block_right = column_right.map_or(ui.max_rect().right(), |right| {
            right.min(ui.max_rect().right())
        });
        ui.horizontal(|ui| {
            back = Action::quiet(icons::BACK)
                .name(back_name)
                .hint(back_word)
                .blocked_if(dirty, "Save or discard this group first.")
                .show(ui)
                .clicked();
            let room = (block_right - ui.cursor().left()).min(ui.available_width());
            let width = (room - 2.0 * design::S).max(Density::Scan.control());
            let column = column_right.map_or(f32::INFINITY, |right| {
                right - ui.cursor().left() - 2.0 * design::S
            });
            let response = ui.add(
                egui::TextEdit::singleline(&mut draft.name)
                    .id(design::focus_id(NAME_FIELD))
                    .desired_width(width.min(name_width(column)))
                    .min_size(egui::vec2(0.0, Density::Scan.control()))
                    .margin(egui::vec2(design::S, design::XS))
                    .font(Type::Heading.font(ui)),
            );
            design::named(ui, &response, &format!("Group name: {}", draft.name));
            name_right = response.rect.right();
            if focus_name {
                design::focus_and_select(ui, response.id, &draft.name);
            }
        });
        // The save row: the state and Discard at its start, Save group at the block's edge. A
        // reason that has no room on the row takes its own line below it. AK1, FK1: each reason
        // has one place, chosen as if both were shown, and the lines below are reserved, so a
        // reason that comes or goes moves no field.
        let empty = draft.setup.members.is_empty();
        // Each reason: the words its place is measured by (the longest form), and the words it
        // shows now, if any.
        let mut below: Vec<(&'static str, Option<&'static str>)> = Vec::new();
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Scan.control());
            let (icon, word, color) = if dirty && (empty || refused) {
                (icons::ERROR, "Invalid", colors().document.invalid)
            } else if dirty {
                (icons::DRAFT, "Unsaved", colors().document.unsaved)
            } else if saved_note.is_some() {
                // FC1: the save row says the shared `Saved`, as the encounter editor does. The
                // list keeps the full sentence for after Back.
                (icons::OK, "Saved", colors().document.saved)
            } else {
                (icons::OK, "No changes", colors().muted)
            };
            discard = super::editor_save_state(ui, Status { icon, word, color }, dirty, None);
            let button = if dirty && !has_run && !empty && !refused {
                Action::primary("Save group")
            } else {
                Action::secondary("Save group")
            }
            .blocked(if empty {
                Some(EMPTY_REASON)
            } else if !dirty {
                Some("No changes to save.")
            } else {
                None
            });
            let gap = ui.spacing().item_spacing.x;
            let font = Type::Small.font(ui);
            let mut room = block_right - ui.cursor().left() - button.natural_width(ui) - gap;
            // Save's own reason first: it stays beside Save while there is room. A dirty group
            // blocks the campaign switch wherever it is open, so it says so, as the encounter
            // editor does.
            let mut beside = |measure: &'static str, shown: Option<&'static str>| {
                let width = design::number_width(ui, measure, &font) + gap;
                if width <= room {
                    room -= width;
                    shown
                } else {
                    below.push((measure, shown));
                    None
                }
            };
            let empty_reason = beside(EMPTY_REASON, empty.then_some(EMPTY_REASON));
            // FB6: while Save is blocked, Discard is the one way out, so the lock says only that.
            let switch = beside(
                super::draft_switch_reason(),
                dirty.then(|| {
                    if empty {
                        super::blocked_draft_switch_reason()
                    } else {
                        super::draft_switch_reason()
                    }
                }),
            );
            super::blocked_reason(ui, switch);
            ui.allocate_ui_with_layout(
                egui::vec2(
                    (block_right - ui.cursor().left()).max(0.0),
                    Density::Scan.control(),
                ),
                egui::Layout::right_to_left(egui::Align::Center),
                |ui| {
                    save = button.show(ui).clicked();
                    super::blocked_reason(ui, empty_reason);
                },
            );
        });
        for (reason, shown) in below {
            let top = ui.cursor().top();
            let height = ui
                .painter()
                .layout(
                    reason.to_owned(),
                    Type::Small.font(ui),
                    colors().muted,
                    ui.available_width(),
                )
                .size()
                .y;
            super::blocked_reason(ui, shown);
            let used = ui.cursor().top() - top;
            ui.add_space((height + ui.spacing().item_spacing.y - used).max(0.0));
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
            self.clear_notice();
            let draft = self.group_draft.take().expect("the editor has a draft");
            if draft.baseline.is_some() {
                match GroupDraft::open(&active.path, &draft.path) {
                    Ok(reopened) => self.group_draft = Some(reopened),
                    Err(e) => self.error = Some(e),
                }
            }
            self.discarded_group = Some(draft);
            self.discarded_group_order = Some(design::ResultOrder::issue());
            self.stamp_kept(super::KeptResult::GroupDiscarded);
            self.discarded = None;
            return;
        }
        if save || save_key {
            self.save_group_draft(ui.ctx(), active, campaigns.modules());
        }
        self.refresh_validation_error(true, super::edit_input(ui.ctx()));
        // FI1's family: the editor's results with a live Undo (the discard, the newest member
        // result), then Save's reason, then the file's warnings sit in one footer pinned below
        // the members and the search, so none of them moves a member or a field.
        let kinds = self.kept_results(false);
        let pressed = std::cell::Cell::new(None);
        let member_undo = std::cell::Cell::new(false);
        let (pressed_ref, member_ref) = (&pressed, &member_undo);
        let footer_id = self.group_draft.as_ref().map(|draft| draft.path.clone());
        let mut footer =
            design::Footer::new(("group_editor_footer", footer_id), ui.available_width());
        // The member result is newer than the discard: it was made on the draft that the
        // discard left open.
        if let Some(draft) = self.group_draft.as_mut() {
            member_result_into(draft, &mut footer, member_ref);
        }
        self.kept_into_footer(&mut footer, &kinds, pressed_ref);
        if let Some(e) = self.error.clone() {
            footer.reason(e.clone(), move |ui| {
                design::message(ui, Tone::Error, &e);
            });
        }
        for warning in file_warnings {
            footer.notice(warning.clone(), move |ui| {
                design::message(ui, Tone::Warning, &warning);
            });
        }
        let bottom = footer.show(ui);
        if let Some(kind) = pressed.get() {
            self.undo_kept(kind, Some((active, campaigns.modules())));
        }
        if member_undo.get()
            && let Some(draft) = self.group_draft.as_mut()
        {
            draft.undo_member_result();
            self.expanded = None;
        }
        let mut body = super::body_above(ui, bottom);
        let ui = &mut body;
        let Some(snapshot) = &self.snapshot else {
            design::spinner(ui);
            return;
        };
        let Some(draft) = self.group_draft.as_mut() else {
            return;
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
                EditorPane::Roster => members_pane(ui, draft, snapshot, expanded, true, name_right),
                EditorPane::Add => {
                    // No members column is drawn, so the name keeps no edge from an older one.
                    let id = column_right_id(&draft.path);
                    if ui.data_mut(|data| data.remove_temp::<f32>(id)).is_some() {
                        ui.ctx().request_discard("The members column is not shown");
                    }
                    add_pane(ui, draft, snapshot, search, focus_search, peek, true)
                }
            }
            return;
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
                add_pane(ui, draft, snapshot, search, focus_search, peek, false);
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
                members_pane(ui, draft, snapshot, expanded, false, name_right);
            });
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
    peek: &mut Option<Uuid>,
    compact: bool,
) {
    if !compact {
        design::group_heading(ui, "Add creatures", 0);
    }
    design::help(
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
        snapshot.creatures.iter().filter(|c| !c.linked),
        search,
        focus_search,
        peek,
        super::SearchRoster {
            already: &|_| false,
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

/// How wide a group's name may be, given the room to the members column's right edge. It ends at
/// FORM_WIDTH or at the column, whichever comes first. Within `XL` of the column it ends at the
/// column, so the two edges are the same or clearly apart.
fn name_width(column: f32) -> f32 {
    if column - design::FORM_WIDTH < design::XL {
        column
    } else {
        design::FORM_WIDTH
    }
}

/// Where a group editor's members column ended in the last pass, for the name field above it.
fn column_right_id(path: &str) -> egui::Id {
    egui::Id::new(("group_column_right", path))
}

/// The newest member result, with its Undo, in the editor's footer. A pressed Undo is recorded
/// in `pressed` and applied with `undo_member_result` after the footer is drawn.
fn member_result_into<'a>(
    draft: &mut GroupDraft,
    footer: &mut design::Footer<'a>,
    pressed: &'a std::cell::Cell<bool>,
) {
    if let Some((words, undo, stamp)) = draft.member_result() {
        footer.result(stamp, words.clone(), true, move |ui| {
            if design::undo_message(ui, Tone::Success, &words, undo) {
                pressed.set(true);
            }
        });
    }
}

fn members_pane(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    expanded: &mut Option<String>,
    tabbed: bool,
    name_right: f32,
) {
    let top = ui.cursor().top();
    // FH3: the column leaves the scroll bar's lane free whether or not the bar shows, so the
    // name field, `Edit` and `⋯` keep their x when a long list (a replacement search) scrolls.
    let width = super::scroll_lane(ui).min(design::LIST_MEASURE);
    egui::ScrollArea::vertical()
        .id_salt("group_editor")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let mut clip = ui.clip_rect();
            clip.min.y = clip.min.y.max(top);
            ui.set_clip_rect(clip);
            let column = ui.scope(|ui| {
                ui.set_width(width.min(ui.available_width()));
                group_fields(ui, draft, snapshot, expanded, None, tabbed, name_right);
            });
            // The name field is drawn before this column; when the column moves (a resize, a
            // scroll bar), draw the pass again so that both end at the same edge.
            let right = column.response.rect.right();
            let id = column_right_id(&draft.path);
            if ui.data(|data| data.get_temp::<f32>(id)) != Some(right) {
                ui.data_mut(|data| data.insert_temp(id, right));
                ui.ctx()
                    .request_discard("The group name field ends at the members column");
            }
        });
}

/// FB4: `Default side` and `Belongs to` share one label column, the wider label plus `S`, so
/// their controls start at one x.
fn choice_label(ui: &mut egui::Ui, text: &str) {
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
    let gap = column_end - ui.cursor().left();
    if gap > 0.0 {
        ui.add_space(gap);
    }
}

#[cfg(test)]
thread_local! {
    /// (frames that reached `members_are_saved`, comparisons it made)
    static MEMBER_COMPARISONS: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((0, 0)) };
}
/// The counts of `members_are_saved`, for `CachedWork`.
#[cfg(test)]
pub(crate) fn member_comparisons() -> (usize, usize) {
    MEMBER_COMPARISONS.with(std::cell::Cell::get)
}

/// What a group is, in its own editor and in the frame an encounter's editor gives it. There,
/// `open` is that encounter, and the frame needs less weight and room.
fn group_fields(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    expanded: &mut Option<String>,
    open: Option<(Uuid, &str)>,
    _tabbed: bool,
    name_right: f32,
) {
    ui.set_max_width(ui.available_width().min(design::LIST_MEASURE));
    ui.horizontal(|ui| {
        choice_label(ui, SIDE_FIELD);
        let name = format!("{SIDE_FIELD} for {}", draft.name);
        super::side_control(ui, &name, &mut draft.setup.side, &[]);
    });
    if open.is_none() {
        design::help(ui, "Party, enemies, or any other name, such as rivals.");
        ui.add_space(design::S);
    }
    if open.is_none_or(|(id, _)| !draft.belongs_to(id)) {
        belongs_to(ui, draft, snapshot, open, name_right);
    }
    ui.add_space(design::S);
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
        design::group_heading(ui, "Members", count as usize);
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
    members(ui, draft, snapshot, expanded);
    ui.add_space(if open.is_some() { design::S } else { design::M });
    design::field(ui, NOTES_FIELD, None, None, |ui| {
        let response = ui.add(
            super::notes_field(&mut draft.notes, if open.is_some() { 2 } else { 4 }).hint_text(
                design::placeholder(ui, "When they arrive, what they want. Markdown is fine."),
            ),
        );
        design::named(ui, &response, &format!("{NOTES_FIELD} for {}", draft.name));
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

fn members(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    expanded: &mut Option<String>,
) {
    let (mut remove, mut raise, mut lower, mut replaced) = (None, None, None, None);
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
        ui.push_id(index, |ui| {
            let entry = snapshot
                .catalogue
                .resolve_ref(&member.creature)
                .ok()
                .and_then(|id| snapshot.catalogue.entries.get(&id))
                .map(|e| &e.entry);
            let name = entry.map_or_else(
                || missing_member_name(None, member.creature.link.as_deref()),
                |entry| entry.name.clone(),
            );
            let button = if entry.is_none() {
                format!("Replace {name}")
            } else {
                format!("Changes for {name}")
            };
            let name = if entry.is_some() {
                shown_name(&member.overrides, name)
            } else {
                name
            };
            let (detail, hp_missing) = entry
                .and_then(|entry| {
                    let creature = snapshot.creatures.iter().find(|c| c.id == entry.id)?;
                    let mut detail = shown_detail(&member.overrides, entry, &side, creature);
                    if member.overrides.contains_key("name") {
                        detail.push_str(&format!(" · as {}", entry.name));
                    }
                    Some((
                        detail,
                        shown_hp_missing(&member.overrides, entry, &side, creature),
                    ))
                })
                .unwrap_or_default();
            let key = format!("{path}:member_{index}");
            let is_open = expanded.as_deref() == Some(key.as_str());
            let member_row = ui
                .label(
                    Type::Strong
                        .rich(ui, format!("{number}. {name}"))
                        .color(colors().text),
                )
                .on_hover_text(&name)
                .rect;
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
            ui.horizontal(|ui| {
                ui.label(Type::Small.rich(ui, "Count").color(colors().muted));
                let count = ui.add(
                    egui::DragValue::new(&mut member.count)
                        .range(1..=MAX_COUNT)
                        .custom_formatter(|value, _| super::grouped(value as u64))
                        .custom_parser(|text| text.replace(',', "").parse().ok()),
                );
                design::named(ui, &count, &format!("How many of {name}"));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
                    if Action::quiet(if entry.is_none() {
                        "Replace…"
                    } else {
                        "Edit"
                    })
                    .density(Density::Read)
                    .name(&button)
                    .selected(is_open)
                    .show(ui)
                    .clicked()
                    {
                        *expanded = if is_open {
                            None
                        } else {
                            show_form_when_it_opens(ui);
                            Some(key.clone())
                        };
                    }
                });
            });
            if entry.is_none() {
                // The reference is on hover: it is what a repair needs, and not a word to read.
                // The repair link is a notice action, so its words start at the warning text.
                // FA1, FJ6: the fault in the one member-fault form, with its line while the
                // members are the saved ones (a draft's lines can move), then its help, then the
                // repair at the words.
                let line = saved_members
                    .then(|| draft.member_lines.get(index).copied())
                    .flatten();
                let changed = member
                    .overrides
                    .get("name")
                    .and_then(Value::as_str)
                    .map(str::trim)
                    .filter(|changed| !changed.is_empty());
                let words = member_fault(
                    changed.unwrap_or(&name),
                    member.creature.link.is_some().then_some(name.as_str()),
                    line,
                    "not in this campaign’s packages",
                );
                let repair = ui.scope(|ui| {
                    design::message_with_actions(ui, Tone::Warning, &words, |ui| {
                        design::help(ui, MISSING_MEMBER_HELP);
                        packages_repair(ui)
                    })
                });
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
                        index,
                        before: std::mem::replace(&mut member.creature, creature.clone()),
                        after: creature,
                        words: format!("Replaced {name} with {chosen}."),
                    });
                    *expanded = None;
                }
            }
            if let Some(entry) = entry {
                if changes_link(ui, &member.overrides, entry, &side, is_open) {
                    show_form_when_it_opens(ui);
                    *expanded = Some(key.clone());
                }
                if is_open && overrides_form(ui, &key, &mut member.overrides, entry, &side, None) {
                    *expanded = None;
                    ui.scroll_to_rect(member_row, Some(egui::Align::Min));
                }
            }
            design::rule(ui);
        });
    }
    // A move, a removal or a replacement changes the members: the comparison runs again.
    if remove.or(raise).or(lower).is_some() || replaced.is_some() {
        draft.members_saved = None;
    }
    // An open form is keyed by place. Left open, it would now edit another member.
    if remove.or(raise).or(lower).is_some() {
        *expanded = None;
        draft.replaced = None;
    }
    // One result line: the newest member result takes it.
    if let Some(result) = replaced {
        draft.removed_member = None;
        draft.removed_unread = None;
        draft.replaced = Some(result);
        draft.member_stamp = Some(super::result_stamp());
    }
    if let Some(index) = remove {
        let member = draft.setup.members.remove(index);
        let base = snapshot
            .catalogue
            .resolve_ref(&member.creature)
            .ok()
            .and_then(|id| snapshot.catalogue.entries.get(&id))
            .map(|entry| entry.entry.name.clone())
            .unwrap_or_else(|| missing_member_name(None, member.creature.link.as_deref()));
        let name = shown_name(&member.overrides, base);
        draft.removed_unread = None;
        draft.removed_member = Some((index, member, name));
        draft.member_stamp = Some(super::result_stamp());
    } else if let Some(index) = raise {
        draft.setup.members.swap(index, index - 1);
    } else if let Some(index) = lower {
        draft.setup.members.swap(index, index + 1);
    }
}

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
    design::message_with_actions(ui, Tone::Warning, &words, |ui| {
        if !help.is_empty() {
            design::help(ui, help);
        }
        ui.add(egui::Label::new(Type::Small.rich(ui, blocked).color(colors().muted)).wrap());
        super::file_line(ui, &snapshot.campaign_root, &draft.path);
    });
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
            if Action::quiet("Edit")
                .density(Density::Read)
                .name(&format!("Changes for {name}"))
                .show(ui)
                .clicked()
            {
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
                ui.data_mut(|data| {
                    data.insert_temp(egui::Id::new(("unread_form_opened", line)), true)
                });
            }
        });
    });
    let mut apply = None;
    // A loot list's typing belongs to this group and this row.
    let repair_id = format!("{}:repair", draft.path());
    if let Some((repair_line, block)) = &mut draft.repair
        && *repair_line == line
    {
        // An unread value is a message, as every other unread value is: a warning mark and bar,
        // and words in text colour.
        let unread = design::small_message(ui, Tone::Warning, &unread_changes(&raw));
        if ui
            .data_mut(|data| data.remove_temp::<bool>(egui::Id::new(("unread_form_opened", line))))
            .is_some()
        {
            let mut rect = unread.rect;
            rect.min.y -= design::L;
            ui.scroll_to_rect(rect, Some(egui::Align::Min));
        }
        // The example belongs to the message, so it starts at the message words.
        let inset = unread.rect.left() - ui.cursor().left();
        ui.horizontal(|ui| {
            ui.add_space(inset);
            ui.label(
                Type::Small
                    .rich(ui, "For example: HP +5, AC +2, Speed 40 ft.")
                    .color(colors().muted),
            );
        });
        if let Some(entry) = entry {
            let id = format!("{repair_id}:{line}");
            if overrides_form(ui, &id, block, &entry.entry, &draft.setup.side, None) {
                apply = Some(block.clone());
            }
        } else {
            design::message(
                ui,
                Tone::Warning,
                "Fix the creature link in Obsidian first.",
            );
        }
    }
    if (remove || apply.is_some())
        && let Err(error) = draft.repair_member(line, apply.as_ref(), snapshot)
    {
        draft.repair_error = Some(error);
    }
    if let Some(error) = &draft.repair_error {
        design::message(ui, Tone::Warning, error);
    }
    design::rule(ui);
}

/// The repair under a member's warning: a bordered secondary button that starts at the warning
/// text (FB5), as every message action without an icon is.
fn packages_repair(ui: &mut egui::Ui) -> bool {
    Action::secondary(PACKAGES_LINK)
        .density(Density::Read)
        .show(ui)
        .clicked()
}

/// One member's fault with its repair (FB5): the fault in its own tone, then a bordered
/// secondary button, `Campaign → Packages` for a missing creature and `Edit group` for any other.
/// The button sits beside the words after `L` when it fits, else below them at the words. The
/// Encounters list draws member faults with this, and Arrivals calls it too.
pub(crate) fn member_fault_row(ui: &mut egui::Ui, fault: &MemberFault) -> bool {
    design::message_with_action(
        ui,
        fault.tone(),
        fault,
        if fault.kind == FaultKind::NotInPackages {
            PACKAGES_LINK
        } else {
            EDIT_GROUP
        },
    )
}

/// The repair a member fault other than a missing creature offers.
pub(crate) const EDIT_GROUP: &str = "Edit group";

/// Where a missing creature's package is chosen.
pub(super) const PACKAGES_LINK: &str = "Campaign → Packages";

/// The repair form's reason. It names the words it could not read in curly quotation marks.
fn unread_changes(raw: &str) -> String {
    format!("Changes “{raw}” not read. Set the changes below, then save the group.")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// FT4: UI text names a value in curly quotation marks, never straight ones.
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

    /// The members with the editor's footer, as the group editor draws them.
    fn editor_members(
        ui: &mut egui::Ui,
        state: &mut (GroupDraft, Option<String>),
        snapshot: &Snapshot,
    ) {
        let pressed = std::cell::Cell::new(false);
        let mut footer = design::Footer::new("members_test_footer", ui.available_width());
        member_result_into(&mut state.0, &mut footer, &pressed);
        let bottom = footer.show(ui);
        if pressed.get() {
            state.0.undo_member_result();
            state.1 = None;
        }
        let mut body = super::super::body_above(ui, bottom);
        members(&mut body, &mut state.0, snapshot, &mut state.1);
    }

    /// iter-26-B, review 2: a control's change that lands after the comparison, in a frame with
    /// editing input, is seen in the next frame even when that frame has no input; after it, an
    /// unchanged frame compares nothing.
    #[test]
    fn members_are_saved_sees_a_change_that_lands_late_in_an_edited_frame() {
        let ctx = egui::Context::default();
        let mut draft = GroupDraft::new();
        let member = MemberDraft {
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
            .build_state(
                move |ctx, state: &mut (GroupDraft, Option<String>)| {
                    egui::CentralPanel::default()
                        .show(ctx, |ui| editor_members(ui, state, &snapshot));
                },
                (draft, None),
            );
        design::theme::install(&harness.ctx);
        harness.run_steps(2);
        harness.get_by_label("More actions for Scout").click();
        harness.run_steps(2);
        harness.get_by_label("Remove").click();
        harness.run_steps(2);
        assert_eq!(harness.state().0.setup.members.len(), 1);
        harness.state_mut().0.notes = "A new note after removal".into();
        harness.state_mut().0.setup.members[0].count = 7;
        harness.get_by_label("Undo remove member").click();
        harness.run_steps(2);
        assert_eq!(harness.state().0.setup.members[0], member);
        assert_eq!(harness.state().0.setup.members[1].count, 7);
        assert_eq!(harness.state().0.notes, "A new note after removal");
    }

    #[test]
    fn i20_p2_flagged_member_uses_the_normal_form_and_keeps_name_and_note() {
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
            .build_state(
                move |ctx, state: &mut (GroupDraft, Option<String>)| {
                    egui::CentralPanel::default()
                        .show(ctx, |ui| editor_members(ui, state, &snapshot));
                },
                (draft, None),
            );
        design::theme::install(&harness.ctx);
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
        harness.get_by_label("Maximum HP: how it changes");
        harness.get_by_label("AC: amount");
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
        assert_eq!(
            encounters.error.as_deref(),
            Some("This group is empty. Add at least one creature.")
        );
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
        // A group is not an encounter, so an encounter's Remove… leaves it.
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

        assert!(encounters.open_group_in_plan(&path, &campaign));
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
        assert!(encounters.open_group_in_plan(&paths[0], &campaign));
        encounters.search = "wolf".into();
        assert!(encounters.open_group_in_plan(&paths[0], &campaign));
        assert_eq!(encounters.search, "wolf");
        assert!(encounters.open_group_in_plan(&paths[1], &campaign));
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
        first.notes = "New words.".into();
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
        let encounters = Encounters {
            snapshot: Some(Snapshot::load(&campaign, &[]).unwrap()),
            ..Encounters::default()
        };
        let offered = encounters.plan_groups();
        assert_eq!(offered.len(), 1);
        assert_eq!(offered[0].encounter, None);
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
        let mut encounters = Encounters {
            draft: Some(Draft::new()),
            ..Encounters::default()
        };
        encounters.new_group_in_plan();
        encounters.discarded_group = encounters.group_draft.take();
        assert!(encounters.group_discarded_in_plan());
        encounters.undo_group_discard_in_plan();
        assert!(encounters.group_open_in_plan());
    }

    #[test]
    fn a_group_discarded_in_the_plan_can_come_back_there() {
        let mut encounters = Encounters::default();
        encounters.new_group_in_plan();
        encounters.group_draft.as_mut().unwrap().name = "Ash Pack".into();
        encounters.discarded_group = encounters.group_draft.take();
        assert!(encounters.group_discarded_in_plan());
        assert!(!encounters.group_open_in_plan());
        encounters.undo_group_discard_in_plan();
        assert!(encounters.group_open_in_plan());
        assert_eq!(encounters.group_draft.as_ref().unwrap().name, "Ash Pack");
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
                "Picket (as Lost scout) · line\u{a0}15: not in this campaign’s packages.",
                "Lookout (as Scout) · line\u{a0}13: the change “HP lots” was not read.",
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
            "Line 13: the change “HP lots” was not read. Use a number, such as HP +5.",
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
        draft.notes = "They hold the east gate.".into();
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
        assert_eq!(reopened.notes, "They hold the east gate.");
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
        draft.notes = "First words.".into();
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
        draft.notes = "Other words.".into();
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
        assert_eq!(error, "Unknown creature: not in this campaign’s packages.");
        assert!(!dir.path().join(&draft.path).exists());
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
        assert!(error.contains("unknown override level"), "{error}");
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
        fs::write(dir.path().join("Patrol.md"), source).unwrap();
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
    }

    #[test]
    fn iter_2_b_partial_group_names_only_joining_members_and_uses_one_xp_total() {
        let dir = tempfile::tempdir().unwrap();
        let active = campaign(dir.path());
        creature(dir.path(), "Scout");
        let source = format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Patrol\nsplat_side: enemies\n---\n\n## Members\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| 1 | [[Scout]] | | | |\n| 1 | [[Missing]] | Lookout | | |\n| 1 | [[Scout]] | Weak scout | HP -100 | |\n",
            Uuid::new_v4()
        );
        fs::write(dir.path().join("Patrol.md"), source).unwrap();
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let group = &snapshot.groups[0];
        assert_eq!(group.member_summary(), "Will add: 1 × Scout");
        assert_eq!(group.xp_label(), "XP 50");
        let draft = GroupDraft::open(dir.path(), "Patrol.md").unwrap();
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
                .starts_with("Unknown creature · line\u{a0}13: not in this campaign’s packages."),
            "{problem}"
        );
        assert_eq!(snapshot.groups[0].member_summary(), "Will add: none");
        assert!(
            problem.contains("not in this campaign’s packages"),
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
                .starts_with("Captain Varra · line\u{a0}13: not in this campaign’s packages.")
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
        // iter-19-B-FD3: a column just past FORM_WIDTH would give a near miss.
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
        assert!(error.contains(BELONGS_TO), "{error}");
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
        assert_eq!(error, super::super::disk::changed_on_disk("group"));
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
            draft: Some(draft),
            snapshot: Some(Snapshot::load(&campaign, &[]).unwrap()),
            ..Default::default()
        };
        let mut harness = egui_kittest::Harness::builder().build_state(
            |ctx, editor: &mut Encounters| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    if !editor.encounter_disk_view(ui, &campaign, &[]) {
                        let Encounters {
                            draft, snapshot, ..
                        } = editor;
                        if let (Some(draft), Some(snapshot)) = (draft.as_mut(), snapshot.as_ref()) {
                            // The draft's results in a footer, as the editor draws them.
                            let pressed = super::super::DraftUndo::default();
                            let mut footer =
                                design::Footer::new("group_test_footer", ui.available_width());
                            draft.results_into(&mut footer, &pressed);
                            let bottom = footer.show(ui);
                            draft.apply_undo(&pressed);
                            let mut body = super::super::body_above(ui, bottom);
                            of_encounter(&mut body, draft, snapshot, &mut false, &mut None);
                        }
                    }
                });
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
        harness.get_by_label("Undo remove").click();
        harness.run_steps(2);
        let draft = harness.state().draft.as_ref().unwrap();
        assert_eq!(draft.name, "Renamed again", "the unrelated edit stays");
        assert_eq!(draft.groups.len(), 1);
        assert_eq!(
            draft.groups[0].name, "Changed",
            "the group comes back with its draft"
        );
        assert!(!note.exists(), "nothing was written");
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
        group.notes = "My plan".into();
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
            error == "Blank: it has no maximum HP. Set it with this member’s Edit button.",
            "{error}"
        );
        draft.setup.members[0].overrides = serde_yaml_ng::from_str("{max_hp: {set: 9}}").unwrap();
        draft.save(&campaign, &[]).unwrap();
    }
}
