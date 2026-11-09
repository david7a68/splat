//! Encounter groups: their notes, their editor, and the overrides form they share with the
//! roster. A group is prepared here and added to a run from the run's own panel.
use super::{
    ADD_PANE_MAX_WIDTH, Creature, Draft, EditorPane, Encounters, NAME_FIELD, ROSTER_MAX_WIDTH,
    Snapshot,
};
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue, EntryRef},
    design::{self, Action, Density, Status, Tone, Type, colors, icons},
    encounter_run::{GroupMember, RunGroup, change_rows},
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
#[derive(Clone)]
pub(super) struct GroupDraft {
    recovery: super::disk::Recovery,
    path: String,
    baseline: Option<String>,
    source: String,
    name: String,
    setup: GroupSetup,
    notes: String,
    saved_name: String,
    saved_setup: GroupSetup,
    saved_notes: String,
    flags: Vec<splat::content::layout::Flag>,
    repair: Option<(usize, Mapping)>,
    removed_unread: Option<(usize, String)>,
    repair_error: Option<String>,
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
            name: name.clone(),
            saved_name: name,
            setup: setup.clone(),
            saved_setup: setup,
            notes: String::new(),
            saved_notes: String::new(),
            flags: Vec::new(),
            repair: None,
            removed_unread: None,
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
            source: text,
            flags: entry.flags.clone(),
            repair: None,
            removed_unread: None,
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
            self.removed_unread = Some((line, source[start..end].to_owned()));
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
                return Err(flag.message.clone());
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
        self.repair = None;
        self.repair_error = None;
        Ok(())
    }
    pub(super) fn add(&mut self, id: Uuid) {
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
        for (index, member) in self.setup.members.iter().enumerate() {
            let what = format!("Member {}", index + 1);
            if !(1..=MAX_COUNT).contains(&member.count) {
                return Err(format!("{what}: count must be from 1 to {MAX_COUNT}."));
            }
            let id = snapshot
                .catalogue
                .resolve_ref(&member.creature)
                .map_err(|e| format!("{what}: {e}"))?;
            let creature = snapshot
                .creatures
                .iter()
                .find(|c| c.id == id)
                .ok_or_else(|| {
                    format!("{what}: this creature is unavailable in campaign content.")
                })?;
            if creature.linked {
                return Err(format!(
                    "{what}: {} has a D&D Beyond sheet, and a group can add it twice. Use the roster.",
                    creature.name
                ));
            }
            let named = |error: String| format!("{what} ({}): {error}", creature.name);
            let overrides = Overrides::parse(&member.overrides).map_err(&named)?;
            let resolved = overrides
                .resolve_for(&snapshot.catalogue.entries[&id].entry, &self.setup.side)
                .map_err(&named)?;
            if resolved.max_hp.is_none() {
                return Err(named(
                    "this creature has no maximum HP, so a run cannot add it. Set it with this member’s Edit button."
                        .into(),
                ));
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
            return Err("Another note uses this group's identity.".into());
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
        self.recovery
            .check(&campaign.path, &mut self.path, self.baseline.as_deref())
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
        let choice = self.recovery.show(ui, &self.comparison_text(snapshot)?);
        if let Some(keep) = choice {
            if keep {
                if let Some(change) = self.recovery.accept() {
                    self.path = change.path;
                    self.baseline = change.text.clone();
                    if let Some(text) = change.text {
                        self.source = text;
                    }
                }
            } else {
                *self = Self::open(root, &self.path)?;
            }
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
        self.baseline = Some(text.to_owned());
        self.saved_name = self.name.clone();
        self.saved_setup = self.setup.clone();
        self.notes = self.notes.trim().to_owned();
        self.saved_notes = self.notes.clone();
    }
}

/// Explicit XP wins; a bare CR supplies its standard reward.
fn creature_xp(entry: &Entry) -> Option<u64> {
    crate::library::presentation::challenge_text(entry)
        .and_then(splat::content::stat_blocks::challenge_xp)
}

/// A creature's name in an editor: the one it will have. Its changes say which entry it is.
pub(super) fn shown_name(block: &Mapping, name: String) -> String {
    Overrides::parse(block)
        .ok()
        .and_then(|overrides| overrides.name)
        .map_or(name, |changed| changed.trim().to_owned())
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
            let unread = splat::content::list_formats::unread_group_members(
                &format!("{}{}", "\n".repeat(e.entry.body_line - 1), e.entry.body),
                &e.entry.flags,
            );
            let group = match EncounterGroup::of(&e.entry) {
                Ok(group) => group,
                Err(error) if !unread.is_empty() && error == "This group is empty. Add at least one creature." => {
                    // Keep an all-unread table visible without relaxing save validation.
                    let setup: GroupSetup = serde_yaml_ng::from_value(
                        e.entry.fields.get("splat_encounter_group")?.clone(),
                    ).ok()?;
                    if !setup.members.is_empty() || !setup.extra.is_empty() { return None; }
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
            for (index, member) in group.members.iter().enumerate() {
                let what = format!("Member {}", index + 1);
                let Some((id, creature)) = catalogue
                    .resolve(&member.creature)
                    .ok()
                    .and_then(|id| Some((id, creatures.iter().find(|c| c.id == id)?)))
                else {
                    let link = e.entry.fields.get("splat_encounter_group")
                        .and_then(|group| group.get("members"))
                        .and_then(|members| members.get(index))
                        .and_then(|member| member.get("creature"))
                        .and_then(|creature| creature.get("link"))
                        .and_then(Value::as_str);
                    let name = missing_member_name(member.overrides.name.as_deref(), link);
                    let fault_name = &name;
                    warnings.push(if let Some(note) = catalogue.old_layout_ref(&member.creature).and_then(|id| catalogue.unreadable.get(&id)) {
                        note.word.clone()
                    } else { format!(
                        "{what}, {fault_name}: not in this campaign's packages. Campaign → Packages."
                    ) });
                    unavailable_members.push((index, member.count, name.to_owned()));
                    unavailable = unavailable.saturating_add(member.count);
                    continue;
                };
                let entry = &catalogue.entries[&id].entry;
                let resolved = member.overrides.resolve_for(entry, &group.side);
                let shown = member.overrides.name.as_deref().unwrap_or(&entry.name);
                let member_name = if shown == entry.name { shown.to_owned() } else { format!("{shown} (as {})", entry.name) };
                let issue = if creature.linked {
                    Some(format!("{what}: {member_name} has a D&D Beyond sheet."))
                } else {
                    match &resolved {
                        Err(error) => Some(format!(
                            "{what}, {}: {}",
                            member_name,
                            override_error(error)
                        )),
                        Ok(resolved) if resolved.max_hp.is_none() => Some(format!(
                            "{what}: {} has no maximum HP. Set it with this member’s Edit button.",
                            member_name
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
                let reference = member.creature.clone().and_then(|value| serde_yaml_ng::from_value::<EntryRef>(value).ok());
                if let Some(note) = reference.as_ref().and_then(|reference| catalogue.unreadable_ref(reference)).filter(|note| note.old_layout) {
                    warnings.push(note.word.clone());
                    let count = member.count.unwrap_or(1);
                    unavailable = unavailable.saturating_add(count);
                    unavailable_members.push((member.before, count, member.label.clone()));
                    continue;
                }
                let name = member.creature.clone()
                    .and_then(|value| serde_yaml_ng::from_value::<EntryRef>(value).ok())
                    .and_then(|reference| catalogue.resolve_ref(&reference).ok())
                    .and_then(|id| catalogue.entries.get(&id))
                    .map_or_else(|| missing_member_name(None, Some(&member.label)), |entry| entry.entry.name.clone());
                let shown = if member.name.is_empty() { &name } else { &member.name };
                let fault_name = if shown == &name { shown.to_owned() } else { format!("{shown} (as {name})") };
                warnings.push(format!("{fault_name} needs fixing (line {})", member.flag.line));
                let count = member.count.unwrap_or(1);
                unavailable = unavailable.saturating_add(count);
                unavailable_members.push((member.before, count, format!("{shown} (not read)")));
            }
            let problem = members.is_empty().then(|| {
                let mut problems = warnings.clone();
                // Edit cannot fix a group whose only fault is the old layout; Convert files can.
                problems.push(
                    if warnings.iter().any(|word| !word.contains("uses the old layout.")) {
                        "No members can join. Edit group to fix them."
                    } else {
                        "No members can join."
                    }
                    .into(),
                );
                problems.join("\n")
            });
            Some(Arc::new(RunGroup {
                id: e.entry.id,
                path: e.entry.path.clone(),
                name: e.entry.name.clone(),
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
                notes: splat::content::list_formats::prose(&e.entry),
                xp,
                xp_unknown,
                unavailable,
                unavailable_members,
                warnings,
                problem,
            }))
        })
        .collect();
    groups.sort_by_key(|group| group.name.to_lowercase());
    groups
}

/// Group actions use the encounter ledger's right-hand columns.
fn list_row(
    ui: &mut egui::Ui,
    group: &RunGroup,
    blocked: Option<&'static str>,
    action: &mut Option<super::ListAction>,
) {
    let summary = if group.warnings.is_empty() {
        group.summary()
    } else {
        format!(
            "{} · {} · {}",
            design::reader::human_label(&group.side),
            group.count_label(),
            group.xp_label()
        )
    };
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
        |ui| {
            super::row_menu(ui, &group.name, |ui| {
                if super::menu_action(ui, "Remove…", blocked) {
                    *action = Some(super::ListAction::RemoveGroup(group.path.clone()));
                }
            });
            if Action::quiet(if group.warnings.is_empty() {
                "Edit"
            } else {
                "Edit group"
            })
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
    for warning in &group.warnings {
        ui.add(
            egui::Label::new(
                Type::Small
                    .rich(ui, format!("{} {warning}", icons::WARN))
                    .color(colors().result.warning),
            )
            .wrap(),
        );
    }
    if let Some(problem) = &group.problem {
        ui.add_space(-design::S);
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y = design::XS;
            let faults = problem
                .lines()
                .map(|fault| {
                    format!(
                        "{} {}",
                        icons::WARN,
                        fault.trim_end_matches(" Campaign → Packages.")
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            ui.vertical(|ui| {
                ui.add(
                    egui::Label::new(
                        Type::Small
                            .rich(ui, faults)
                            .line_height(Some(Type::Small.line()))
                            .color(colors().result.warning),
                    )
                    .wrap(),
                );
                if problem.contains("Campaign → Packages") && packages_link(ui) {
                    ui.data_mut(|data| {
                        data.insert_temp(egui::Id::new("open_campaign_packages"), true)
                    });
                }
            });
        });
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
        design::group_heading(ui, "Encounter deleted", orphans.len());
        for group in orphans {
            list_row(ui, group, blocked, action);
        }
    }
}

/// A removed note stays in memory until the DM leaves this campaign or removes another.
pub(super) struct RemovedGroup {
    path: std::path::PathBuf,
    text: String,
    name: String,
}
impl RemovedGroup {
    pub(super) fn remove(campaign: &Campaign, path: &str) -> Result<Self, String> {
        let path = note_path(&campaign.path, path)?;
        let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        let entry = Entry::parse(path.to_str().ok_or("Invalid group path")?, &text)?;
        if entry.kind != "encounter_group" {
            return Err("This note is not a group.".to_owned());
        }
        fs::remove_file(&path).map_err(|error| error.to_string())?;
        Ok(Self {
            path,
            text,
            name: entry.name,
        })
    }
    fn restore(&self) -> Result<(), String> {
        use std::io::Write;
        let mut copy =
            tempfile::NamedTempFile::new_in(self.path.parent().ok_or("Invalid group folder")?)
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
        if let Some(keep) = draft
            .recovery
            .show(ui, &draft.comparison_text(snapshot).unwrap_or_default())
        {
            if keep {
                if let Some(change) = draft.recovery.accept() {
                    draft.path = change.path;
                    draft.baseline = change.text.clone();
                    if let Some(text) = change.text {
                        draft.source = text;
                    }
                    self.save_group_draft(ui.ctx(), campaign, modules);
                }
            } else {
                let path = draft.path.clone();
                self.discarded_group = self.group_draft.take();
                self.group_draft = GroupDraft::open(&campaign.path, &path).ok();
                self.error = None;
            }
        }
        true
    }
    pub(super) fn group_remove_undo(
        &mut self,
        ui: &mut egui::Ui,
        active: &Campaign,
        modules: &[Module],
    ) {
        let Some(removed) = &self.removed_saved_group else {
            return;
        };
        if design::undo_message(
            ui,
            Tone::Info,
            &format!("Removed group {}.", removed.name),
            "Undo remove group",
        ) {
            match removed.restore() {
                Ok(()) => {
                    self.removed_saved_group = None;
                    self.reload_snapshot(active, modules);
                    self.saved = true;
                }
                Err(error) => self.error = Some(format!("The group did not come back: {error}")),
            }
        }
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
    ui.add(
        egui::Label::new(
            Type::Small
                .rich(
                    ui,
                    "They arrive when you add them to a run of this encounter. Save encounter saves them too.",
                )
                .color(colors().muted),
        )
        .wrap(),
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
        ui.push_id(&path, |ui| {
            let stroke = if adding {
                egui::Stroke::new(design::FOCUS_WIDTH, colors().accent)
            } else {
                egui::Stroke::new(1.0, colors().rule)
            };
            let frame = egui::Frame::new()
                .stroke(stroke)
                .corner_radius(design::RADIUS_CONTROL)
                .inner_margin(design::S)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            super::row_menu(ui, &group.name.clone(), |ui| {
                                ui.menu_button("Move to…", |ui| {
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
                            if Action::quiet(if adding { "Adding here" } else { "Add here" })
                                .icon(if adding { icons::OK } else { icons::ADD })
                                .density(Density::Read)
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
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut group.name)
                                    .desired_width(ui.available_width() - 2.0 * design::S)
                                    .margin(egui::vec2(design::S, 4.0))
                                    .font(Type::Heading.font(ui)),
                            );
                            design::named(ui, &response, &format!("Group name: {}", group.name));
                            if focus_group.as_deref() == Some(path.as_str()) {
                                *focus_group = None;
                                design::focus_and_select(ui, response.id, &group.name);
                                response.scroll_to_me(Some(egui::Align::Center));
                            }
                        });
                    });
                    group_fields(
                        ui,
                        group,
                        snapshot,
                        expanded,
                        Some((this.0, &this.1)),
                        false,
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
        *removed_group = Some((index, gone, delete));
    }
    if let Some((_, removed, delete)) = removed_group.as_ref() {
        let words = if *delete {
            "Group deleted. Save deletes its file.".to_owned()
        } else {
            format!("{} moves to any encounter when you save.", removed.name)
        };
        let undo = if *delete {
            "Undo remove group"
        } else {
            "Undo move"
        };
        if design::undo_message(ui, Tone::Info, &words, undo) {
            let (index, removed, delete) = removed_group.take().expect("a removed group");
            if delete {
                deleted_groups.retain(|group| group.path != removed.path);
            }
            if let Some(group) = groups.iter_mut().find(|group| group.path == removed.path) {
                *group = removed;
            } else {
                groups.insert(index.min(groups.len()), removed);
            }
        }
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
        ui.label(Type::Small.rich(ui, BELONGS_TO).color(colors().muted));
        let combo = egui::ComboBox::from_id_salt("group_encounter")
            .icon(design::combo_icon)
            .width(240.0)
            .truncate()
            .selected_text(shown)
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(current.is_none(), ANY_ENCOUNTER)
                    .clicked()
                {
                    draft.setup.encounter = None;
                }
                for (id, name) in &choices {
                    // The same choice again keeps a link that the note gives.
                    if ui.selectable_label(current == Some(*id), *name).clicked()
                        && current != Some(*id)
                    {
                        draft.setup.encounter = Some(EntryRef::id(*id));
                    }
                }
            });
        design::named(ui, &combo.response, &format!("{BELONGS_TO}: {shown}"));
    });
    // In an encounter's editor, belonging to it goes without saying.
    if current.is_some() && current == open.map(|(id, _)| id) {
        return;
    }
    ui.add(
        egui::Label::new(
            Type::Small
                .rich(
                    ui,
                    match current {
                        None => "A run of any encounter offers this group.",
                        Some(_) if shown == MISSING_ENCOUNTER => {
                            "No fight offers this group now. Choose an encounter, or Any encounter."
                        }
                        Some(_) => "Only runs of this encounter offer this group.",
                    },
                )
                .color(colors().muted),
        )
        .wrap(),
    );
}

/// The line under a creature's name in an editor, with the values this block gives it.
pub(super) fn shown_detail(
    block: &Mapping,
    entry: &Entry,
    side: &str,
    creature: &Creature,
) -> String {
    Overrides::parse(block)
        .ok()
        .filter(|overrides| !overrides.is_empty())
        .and_then(|overrides| overrides.resolve_for(entry, side).ok())
        .map_or_else(
            || creature.detail.clone(),
            |resolved| super::creature_detail(entry, creature.party, Some(&resolved)),
        )
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

pub(super) fn change_words(overrides: &Overrides, entry: &Entry, side: &str) -> String {
    let count = overrides
        .rows(&Base::of(entry, side))
        .iter()
        .filter(|row| row.by_hand)
        .count();
    if count == 0 {
        "Changed".to_owned()
    } else {
        format!("Changed · {count} to add by hand")
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
            let word = change_words(&overrides, entry, side);
            let clicked = design::link(ui, &word).clicked();
            if let Err(error) = overrides.resolve_for(entry, side) {
                design::message(ui, Tone::Error, &override_error(&error));
                // The failed change stays visible beside its fault.
                change_rows(ui, &rows);
            }
            clicked
        }
        Err(error) => {
            design::message(ui, Tone::Error, &override_error(&error));
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
        let mut preview = Overrides::default();
        match label {
            "Maximum HP" => preview.max_hp = *value,
            "AC" => preview.armor_class = *value,
            "Initiative bonus" => preview.initiative_bonus = *value,
            "Speed" => preview.speed = *value,
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
        if let Some(row) = preview.rows(base).first() {
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(ui, crate::encounter_run::change_preview(row))
                        .color(colors().muted),
                )
                .wrap(),
            );
        }
    });
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
        let width = (ui.available_width() - design::S).clamp(120.0, 320.0);
        if design::text_input(ui, label, &mut text, hint, width).changed() {
            *value = (!text.trim().is_empty()).then_some(text);
        }
    });
}

/// The whole list of changes this slot or member can build in. `sides` is for a roster slot,
/// whose row already sets its side, so this offers a side only when a note has given one, to
/// change or clear it. A group can arrive anywhere, so its members can name any side.
pub(super) fn overrides_form(
    ui: &mut egui::Ui,
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
    if let Err(error) = overrides.resolve_for(entry, side) {
        design::message(ui, Tone::Error, &override_error(&error));
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
        conditions_row(ui, &mut overrides.conditions);
    }
    text_row(ui, "Note", &mut overrides.note, "Why this creature differs");
    if overrides != before {
        *block = overrides.to_mapping();
    }
    done
}

/// The same official names and local Other effect as the run.
fn conditions_row(ui: &mut egui::Ui, conditions: &mut Vec<String>) {
    design::group_label(ui, "Starting conditions");
    let columns = if ui.available_width() >= design::CONDITION_THREE_COLUMNS_FROM {
        3
    } else {
        2
    };
    egui::Grid::new("starting_conditions")
        .num_columns(columns)
        .min_col_width(
            (ui.available_width() / columns as f32 - design::XS).max(design::CONDITION_COLUMN_MIN),
        )
        .spacing([design::XS, design::XS])
        .show(ui, |ui| {
            for (position, (_, name)) in crate::combat_sync::CONDITIONS.iter().enumerate() {
                let found = conditions.iter().position(|c| same_condition(c, name));
                if crate::encounter_run::view::toggle(
                    ui,
                    name,
                    if found.is_some() {
                        icons::OK
                    } else {
                        icons::ADD
                    },
                    found.is_some(),
                    name,
                    Density::Read,
                )
                .clicked()
                {
                    if let Some(at) = found {
                        conditions.remove(at);
                    } else {
                        conditions.push((*name).to_owned());
                    }
                }
                if position % columns == columns - 1 {
                    ui.end_row();
                }
            }
        });
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
    for effect in custom {
        ui.horizontal(|ui| {
            ui.label(Type::Small.rich(ui, &effect).color(colors().muted));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                design::row_menu(ui, &effect, |ui| {
                    if ui.button("Remove").clicked() {
                        conditions.retain(|c| c != &effect);
                        ui.close();
                    }
                });
            });
        });
    }
    let id = ui.id().with("starting_other_effect");
    let mut text = ui
        .data(|data| data.get_temp::<String>(id))
        .unwrap_or_default();
    ui.horizontal(|ui| {
        let field = design::text_input(
            ui,
            "Other effect",
            &mut text,
            "Other effect",
            design::FORM_WIDTH.min(ui.available_width() - design::L * 8.0),
        );
        let enter = field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
        if (Action::secondary("Add")
            .density(Density::Read)
            .show(ui)
            .clicked()
            || enter)
            && !text.trim().is_empty()
        {
            if !conditions.iter().any(|c| same_condition(c, text.trim())) {
                conditions.push(text.trim().to_owned());
            }
            text.clear();
        }
    });
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
        self.validation_path = Some(draft.path.clone());
        if let Some(error) = self
            .snapshot
            .as_ref()
            .and_then(|snapshot| draft.validate(snapshot, None).err())
        {
            self.validation_error = Some(error.clone());
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
                self.validation_error = self
                    .snapshot
                    .as_ref()
                    .and_then(|snapshot| draft.validate(snapshot, None).err())
                    .filter(|validation| *validation == e);
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
        let focus_name = std::mem::take(&mut self.focus_name);
        let draft = self.group_draft.as_mut().expect("the editor has a draft");
        let (mut back, mut save, mut discard) = (false, false, false);
        ui.horizontal(|ui| {
            back = Action::quiet(icons::BACK)
                .name(back_name)
                .hint(back_word)
                .blocked_if(dirty, "Save or discard this group first.")
                .show(ui)
                .clicked();
            let width = (ui.available_width()
                - design::S * 17.0
                - if draft.setup.members.is_empty() {
                    design::number_width(ui, "Add a creature first", &Type::Small.font(ui))
                        + design::L
                } else {
                    0.0
                })
            .max(Density::Scan.control());
            let response = ui.add(
                egui::TextEdit::singleline(&mut draft.name)
                    .id(design::focus_id(NAME_FIELD))
                    .desired_width(width)
                    .min_size(egui::vec2(0.0, Density::Scan.control()))
                    .margin(egui::vec2(design::S, design::XS))
                    .font(Type::Heading.font(ui)),
            );
            design::named(ui, &response, &format!("Group name: {}", draft.name));
            if focus_name {
                design::focus_and_select(ui, response.id, &draft.name);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                save = if dirty && !has_run {
                    Action::primary("Save group")
                } else {
                    Action::secondary("Save group")
                }
                .blocked(if draft.setup.members.is_empty() {
                    Some("Add a creature first")
                } else if !dirty {
                    Some("No changes to save.")
                } else {
                    None
                })
                .show(ui)
                .clicked();
                super::blocked_reason(
                    ui,
                    if draft.setup.members.is_empty() {
                        Some("Add a creature first")
                    } else {
                        None
                    },
                );
            });
        });
        ui.horizontal_wrapped(|ui| {
            ui.set_min_height(design::Density::Read.control());
            let (icon, word, color) = if dirty {
                (icons::DRAFT, "Unsaved", colors().document.unsaved)
            } else if let Some(note) = saved_note.as_deref() {
                (icons::OK, note, colors().document.saved)
            } else {
                (icons::OK, "No changes", colors().muted)
            };
            {
                discard = super::editor_save_state(ui, Status { icon, word, color }, dirty, None);
            }
            if dirty && in_plan {
                super::blocked_reason(ui, Some(super::draft_switch_reason()));
            }
        });
        design::rule(ui);
        let unread =
            splat::content::list_formats::unread_group_members(&draft.source, &draft.flags);
        for flag in &draft.flags {
            if !unread.iter().any(|member| member.flag.line == flag.line) {
                design::message(
                    ui,
                    Tone::Warning,
                    &format!("Line {}: {}", flag.line, flag.message),
                );
            }
        }

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
            self.discarded = None;
            return;
        }
        if save || save_key {
            self.save_group_draft(ui.ctx(), active, campaigns.modules());
        }
        self.discard_undo(ui);
        self.refresh_validation_error(true);
        if let Some(e) = &self.error {
            design::message(ui, Tone::Error, e);
        }
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
                EditorPane::Roster => members_pane(ui, draft, snapshot, expanded, true),
                EditorPane::Add => add_pane(ui, draft, snapshot, search, focus_search, peek, true),
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
                members_pane(ui, draft, snapshot, expanded, false);
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
    ui.add(
        egui::Label::new(
            Type::Small
                .rich(
                    ui,
                    "From this campaign and the packages it uses. Party members go on an encounter’s roster.",
                )
                .color(colors().muted),
        )
        .wrap(),
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

fn members_pane(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    expanded: &mut Option<String>,
    tabbed: bool,
) {
    let top = ui.cursor().top();
    egui::ScrollArea::vertical()
        .id_salt("group_editor")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let mut clip = ui.clip_rect();
            clip.min.y = clip.min.y.max(top);
            ui.set_clip_rect(clip);
            group_fields(ui, draft, snapshot, expanded, None, tabbed);
        });
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
) {
    ui.horizontal(|ui| {
        ui.label(Type::Small.rich(ui, SIDE_FIELD).color(colors().muted));
        let name = format!("{SIDE_FIELD} for {}", draft.name);
        super::side_control(ui, &name, &mut draft.setup.side, &[]);
    });
    if open.is_none() {
        ui.label(
            Type::Small
                .rich(ui, "Party, enemies, or any other name, such as rivals.")
                .color(colors().muted),
        );
        ui.add_space(design::S);
    }
    if open.is_none_or(|(id, _)| !draft.belongs_to(id)) {
        belongs_to(ui, draft, snapshot, open);
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
                    .rich(ui, format!("{}{}", " · ", members_heading(draft, snapshot)))
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
    design::group_label(ui, NOTES_FIELD);
    let response = ui.add(
        egui::TextEdit::multiline(&mut draft.notes)
            .desired_rows(if open.is_some() { 2 } else { 4 })
            .desired_width(ui.available_width() - design::M)
            .hint_text("When they arrive, what they want. Markdown is fine."),
    );
    design::named(ui, &response, &format!("{NOTES_FIELD} for {}", draft.name));
}

/// How many creatures will arrive, and what they are worth, as the list and the panel say it.
fn members_heading(draft: &GroupDraft, snapshot: &Snapshot) -> String {
    let (mut xp, mut unknown) = (0_u64, !draft.flags.is_empty());
    for member in &draft.setup.members {
        let each = snapshot
            .catalogue
            .resolve_ref(&member.creature)
            .ok()
            .and_then(|id| snapshot.catalogue.entries.get(&id))
            .and_then(|e| creature_xp(&e.entry));
        match each {
            Some(each) => xp = xp.saturating_add(each.saturating_mul(member.count)),
            None => unknown = true,
        }
    }
    format!(
        "XP {}{}",
        crate::encounter_run::grouped_number(xp),
        if unknown { " + unknown" } else { "" }
    )
}

fn members(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    expanded: &mut Option<String>,
) {
    let (mut remove, mut raise, mut lower) = (None, None, None);
    let total = draft.setup.members.len();
    let side = draft.setup.side.clone();
    let path = draft.path.clone();
    let unread = splat::content::list_formats::unread_group_members(&draft.source, &draft.flags);
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
            let detail = entry
                .and_then(|entry| {
                    let creature = snapshot.creatures.iter().find(|c| c.id == entry.id)?;
                    let mut detail = shown_detail(&member.overrides, entry, &side, creature);
                    if member.overrides.contains_key("name") { detail.push_str(&format!(" · as {}", entry.name)); }
                    Some(detail)
                })
                .unwrap_or_default();
            let key = format!("{path}:member_{index}");
            let is_open = expanded.as_deref() == Some(key.as_str());
            let member_row = ui.label(Type::Strong.rich(ui, format!("{number}. {name}")).color(colors().text)).on_hover_text(&name).rect;
            if !detail.is_empty() {
                let response = ui.add(egui::Label::new(Type::Small.rich(ui, super::stat_line(&detail)).color(colors().muted)).wrap());
                design::named(ui, &response, &detail);
            }
            if let Some(note) = snapshot.catalogue.unreadable_ref(&member.creature).filter(|note| note.old_layout) {
                crate::formats_ui::old_notice(ui, &note.word, vec![note.path.clone()]);
            }
            ui.horizontal(|ui| {
                ui.label(Type::Small.rich(ui, "Count").color(colors().muted));
                let count = ui.add(egui::DragValue::new(&mut member.count).range(1..=MAX_COUNT)
                    .custom_formatter(|value, _| super::grouped(value as u64))
                    .custom_parser(|text| text.replace(',', "").parse().ok()));
                design::named(ui, &count, &format!("How many of {name}"));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    super::row_menu(ui, &name, |ui| {
                        if super::menu_action(ui, "Move up", (index == 0).then_some("Already first.")) { raise = Some(index); }
                        if super::menu_action(ui, "Move down", (index + 1 == total).then_some("Already last.")) { lower = Some(index); }
                        if super::menu_action(ui, "Remove", None) { remove = Some(index); }
                    });
                    if Action::quiet(if entry.is_none() { "Replace…" } else { "Edit" }).density(Density::Read).name(&button).selected(is_open).show(ui).clicked() {
                        *expanded = if is_open { None } else { show_form_when_it_opens(ui); Some(key.clone()) };
                    }
                });
            });
            if entry.is_none() {
                // The reference is on hover: it is what a repair needs, and not a word to read.
                design::message(
                    ui,
                    Tone::Warning,
                    "This campaign cannot find this creature. Choose a replacement, check the packages it uses, or remove this member.",
                )
                .on_hover_text(member.creature.label());
                if packages_link(ui) {
                    ui.data_mut(|data| data.insert_temp(egui::Id::new("open_campaign_packages"), true));
                }
                if is_open && let Some(creature) = super::choose_replacement_creature(ui, snapshot) {
                    member.creature = creature;
                    *expanded = None;
                }
            }
            if let Some(entry) = entry {
                if changes_link(ui, &member.overrides, entry, &side, is_open) {
                    show_form_when_it_opens(ui); *expanded = Some(key.clone());
                }
                if is_open && overrides_form(ui, &mut member.overrides, entry, &side, None) { *expanded = None; ui.scroll_to_rect(member_row, Some(egui::Align::Min)); }
            }
            design::rule(ui);
        });
    }
    if draft.removed_unread.is_some()
        && Action::quiet("Undo remove member").show(ui).clicked()
        && let Some((line, row)) = draft.removed_unread.take()
    {
        let at = draft
            .source
            .split_inclusive('\n')
            .take(line - 1)
            .map(str::len)
            .sum();
        draft.source.insert_str(at, &row);
        if let Ok(entry) = Entry::parse(&draft.path, &draft.source) {
            draft.flags = entry.flags;
        }
    }
    // An open form is keyed by place. Left open, it would now edit another member.
    if remove.or(raise).or(lower).is_some() {
        *expanded = None;
    }
    if let Some(index) = remove {
        draft.setup.members.remove(index);
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
    if let Some(note) = reference
        .as_ref()
        .and_then(|reference| snapshot.catalogue.unreadable_ref(reference))
        .filter(|note| note.old_layout)
    {
        crate::formats_ui::old_notice(ui, &note.word, vec![note.path.clone()]);
        return;
    }
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
    let mut detail = entry
        .and_then(|entry| snapshot.creatures.iter().find(|c| c.id == entry.entry.id))
        .map_or(count.clone(), |creature| {
            format!("{count} · {}", creature.detail)
        });
    if name != base_name {
        detail.push_str(&format!(" · as {base_name}"));
    }
    let summary = ui.add(
        egui::Label::new(
            Type::Small
                .rich(ui, super::stat_line(&detail))
                .color(colors().muted),
        )
        .wrap(),
    );
    design::named(ui, &summary, &detail);
    let problem = member
        .flag
        .message
        .strip_prefix("Changes: ")
        .and_then(|text| text.strip_suffix(" not read"))
        .map_or_else(
            || member.flag.message.clone(),
            |term| format!("Changes \"{term}\" not read"),
        );
    design::message(
        ui,
        Tone::Warning,
        &format!("Line {}: {problem}.", member.flag.line),
    );
    let blocked = if member.flag.message.starts_with("Changes:") {
        "This member cannot join a fight until its Changes cell is fixed. Other members can join."
    } else {
        "This member cannot join a fight until this row is fixed. Other members can join."
    };
    ui.add(egui::Label::new(Type::Small.rich(ui, blocked).color(colors().muted)).wrap());
    crate::world::entry_file_actions(ui, &snapshot.campaign_root, &draft.path, true);
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
                    for term in raw.split(',') {
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
    if let Some((repair_line, block)) = &mut draft.repair
        && *repair_line == line
    {
        let unread = ui.add(
            egui::Label::new(
                Type::Small
                    .rich(
                        ui,
                        format!(
                            "Changes \"{raw}\" not read. Set the changes below, then save the group."
                        ),
                    )
                    .color(colors().result.warning),
            )
            .wrap(),
        );
        if ui
            .data_mut(|data| data.remove_temp::<bool>(egui::Id::new(("unread_form_opened", line))))
            .is_some()
        {
            let mut rect = unread.rect;
            rect.min.y -= design::L;
            ui.scroll_to_rect(rect, Some(egui::Align::Min));
        }
        ui.label(
            Type::Small
                .rich(ui, "For example: HP +5, AC +2, Speed 40 ft.")
                .color(colors().muted),
        );
        if let Some(entry) = entry {
            if overrides_form(ui, block, &entry.entry, &draft.setup.side, None) {
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

fn packages_link(ui: &mut egui::Ui) -> bool {
    design::status_link(
        ui,
        Status {
            icon: icons::CAMPAIGN,
            word: "Campaign → Packages",
            color: colors().accent,
        },
    )
    .clicked()
}

#[cfg(test)]
mod tests {
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
            &splat::content::formats::convert("Fixture.md", &format!(
                "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: creature\nsplat_name: {name}\nsplat_creature: {{cr_text: '1/4 (XP 50; PB +2)'}}\nsplat_stats:\n  max_hp: {{value: 10}}\n  armor_class: {{value: 15}}\n  initiative: {{value: 2}}\n  speed: {{value: 30}}\n---\n"
            )).expect("schema-2 fixture"),
        )
        .unwrap();
        id
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
                        .show(ctx, |ui| members(ui, &mut state.0, &snapshot, &mut state.1));
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
            validation_error: Some(error.clone()),
            error: Some(error),
            snapshot: Some(snapshot),
            group_draft: Some(group),
            ..Default::default()
        };
        encounters.group_draft.as_mut().unwrap().name = "Scouts".into();
        encounters.refresh_validation_error(true);
        assert!(encounters.error.is_none());
        encounters
            .group_draft
            .as_mut()
            .unwrap()
            .setup
            .members
            .clear();
        encounters.refresh_validation_error(true);
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
        let removed = RemovedGroup::remove(&active, &group.path).unwrap();
        assert!(!path.exists());
        removed.restore().unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), text);
        fs::write(&path, "A new note").unwrap();
        assert!(removed.restore().is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "A new note");
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
            &splat::content::formats::convert("Fixture.md", &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter\nsplat_name: Patrol\nsplat_encounter: {{roster: []}}\n---\n",
                Uuid::new_v4()
            )).expect("schema-2 fixture"),
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
            &splat::content::formats::convert("Fixture.md", &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter\nsplat_name: Ford\nsplat_encounter: {{roster: []}}\n---\n",
                Uuid::new_v4()
            )).expect("schema-2 fixture"),
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
                &splat::content::formats::convert("Fixture.md", &format!(
                    "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: encounter\nsplat_name: {name}\nsplat_encounter: {{roster: []}}\n---\n"
                )).expect("schema-2 fixture"),
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
            &splat::content::formats::convert("Fixture.md", &format!(
                "---\nsplat_schema: 1\nsplat_id: {owner}\nsplat_kind: encounter\nsplat_name: Bridge\nsplat_encounter: {{roster: []}}\n---\n"
            )).expect("schema-2 fixture"),
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
            &splat::content::formats::convert("Fixture.md", &format!("---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: creature\nsplat_name: Hero\nsplat_creature: {{}}\nsplat_external: {{provider: dndbeyond, character_id: '1'}}\n---\n")).expect("schema-2 fixture"),
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
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Broken row\nsplat_side: enemies\n---\n\n## Members\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| 7 | [[Scout]] | Lookout | HP lots | Keep this row |\n| 2 | [[Scout]] | | | |\n",
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
            9,
            "all named members remain visible"
        );
        assert!(group.summary().contains("2 of 9 members can join"));
        assert!(
            group
                .summary()
                .contains("Lookout (as Scout) needs fixing (line 13)")
        );
        assert_eq!(group.xp_label(), "XP 100 + unknown");
        let draft = GroupDraft::open(dir.path(), "Broken row.md").unwrap();
        let unread =
            splat::content::list_formats::unread_group_members(&draft.source, &draft.flags);
        assert_eq!(unread.len(), 1);
        assert_eq!(
            unread[0].before, 0,
            "the rejected row precedes the valid member"
        );
        assert_eq!(unread[0].count, Some(7));
        assert_eq!(
            fs::read_to_string(dir.path().join("Broken row.md")).unwrap(),
            source
        );
        fs::write(
            dir.path().join("Broken row.md"),
            source.replace("| 2 | [[Scout]] | | | |\n", ""),
        )
        .unwrap();
        let snapshot = Snapshot::load(&active, &[]).unwrap();
        let group = snapshot
            .groups
            .iter()
            .find(|group| group.name == "Broken row")
            .expect("a group with only unread members stays visible");
        assert!(group.summary().contains("0 of 7 members can join"));
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
        assert!(error.contains("Member 1"), "{error}");
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
        assert!(error.contains("Member 1 (Scout): Maximum HP"), "{error}");
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
        assert_eq!(group.count_label(), "1 of 3 members can join");
        assert_eq!(group.unavailable, 2);
        assert_eq!(group.warnings.len(), 2);
        assert!(group.summary().contains("Lookout"));
        assert!(group.summary().contains("Weak scout"));
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
            &splat::content::formats::convert("Fixture.md", &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Stray\nsplat_encounter_group:\n  side: enemies\n  members:\n  - creature: {{id: {id}}}\n---\n",
                Uuid::new_v4()
            )).expect("schema-2 fixture"),
        )
        .unwrap();
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let problem = snapshot.groups[0].problem.clone().unwrap();
        assert!(!problem.contains(&id.to_string()), "{problem}");
        assert!(problem.contains("Member 1, Unknown creature"), "{problem}");
        assert_eq!(snapshot.groups[0].member_summary(), "1 × Unknown creature");
        assert!(
            problem.contains("not in this campaign's packages"),
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
            write_note(dir.path(), "Stray.md", None, &splat::content::formats::convert("Fixture.md", &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Stray\nsplat_encounter_group:\n  side: enemies\n  members:\n  - creature: {{key: 'missing:goblin_boss'{extra}}}\n    count: 2\n---\n", Uuid::new_v4()
            )).expect("schema-2 fixture")).unwrap();
            let snapshot = Snapshot::load(&campaign, &[]).unwrap();
            let group = &snapshot.groups[0];
            let problem = group.problem.as_ref().unwrap();
            assert!(problem.contains(name), "{problem}");
            assert!(!problem.contains("missing:goblin_boss"), "{problem}");
            assert!(
                !problem.contains("00000000-0000-4000-8000-000000000001"),
                "{problem}"
            );
            assert_eq!(group.member_summary(), format!("2 × {name}"));
            assert_eq!(group.creature_count(), 2);
        }
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = Uuid::new_v4();
        write_note(dir.path(), "Stray.md", None, &splat::content::formats::convert("Fixture.md", &format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Stray\nsplat_encounter_group:\n  side: enemies\n  members:\n  - creature: {{id: {id}}}\n    overrides: {{name: Captain Varra}}\n---\n", Uuid::new_v4()
        )).expect("schema-2 fixture")).unwrap();
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let group = &snapshot.groups[0];
        assert!(
            group
                .problem
                .as_ref()
                .unwrap()
                .contains("Member 1, Captain Varra:")
        );
        assert!(!group.problem.as_ref().unwrap().contains(&id.to_string()));
        assert_eq!(group.member_summary(), "1 × Captain Varra");
    }

    fn encounter(root: &std::path::Path, name: &str) -> Uuid {
        let id = Uuid::new_v4();
        write_note(
            root,
            &format!("{name}.md"),
            None,
            &splat::content::formats::convert("Fixture.md", &format!(
                "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: encounter\nsplat_name: {name}\nsplat_encounter:\n  roster: []\n  sides: {{}}\n  assumptions: []\n  completion: {{mode: manual}}\n  limits: {{rounds: 1, resolution_events: 1}}\n---\n"
            )).expect("schema-2 fixture"),
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
            &splat::content::formats::convert("Fixture.md", &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Linked\nsplat_encounter_group:\n  side: enemies\n  encounter: {{id: {arena}, link: '[[Arena]]'}}\n  members:\n  - creature: {{id: {scout}}}\n---\n",
                Uuid::new_v4()
            )).expect("schema-2 fixture"),
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
        assert!(error.contains("Choose which version to keep"), "{error}");
        assert!(draft.groups[0].recovery.change.is_some());
        assert_eq!(fs::read_to_string(&note).unwrap(), external);
        assert!(draft.dirty(), "the group's change is still here to save");
        assert_eq!(
            Draft::open(dir.path(), &draft.path).unwrap().name,
            saved_name
        );
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
        let overwrite = splat::content::formats::convert_resolved(
            &harness.state().group_draft.as_ref().unwrap().path,
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
                &splat::content::formats::convert("Fixture.md", &format!(
                    "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: C\nsplat_creature: {creature}\n---\n\n{body}\n",
                    Uuid::new_v4()
                )).expect("schema-2 fixture"),
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
            &splat::content::formats::convert("Fixture.md", &format!("---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: creature\nsplat_name: Blank\nsplat_creature: {{}}\n---\n")).expect("schema-2 fixture"),
        )
        .unwrap();
        let mut draft = GroupDraft::new();
        draft.add(id);
        let error = draft.save(&campaign, &[]).unwrap_err();
        assert!(
            error.contains("Member 1 (Blank)") && error.contains("no maximum HP"),
            "{error}"
        );
        draft.setup.members[0].overrides = serde_yaml_ng::from_str("{max_hp: {set: 9}}").unwrap();
        draft.save(&campaign, &[]).unwrap();
    }
}
