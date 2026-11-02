//! Encounter groups: their notes, their editor, and the overrides form they share with the
//! roster. A group is prepared here and added to a run from the run's own panel.
use super::{
    ADD_PANE_MAX_WIDTH, Creature, Draft, EditorPane, Encounters, NAME_FIELD, ROSTER_MAX_WIDTH,
    Snapshot,
};
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue, EntryRef, patch_fields},
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
    overrides::{BY_HAND, Base, Change, MAX_NAME, Overrides, condition_names, same_condition},
};
use std::{fs, sync::Arc};
use uuid::Uuid;

const FOLDER: &str = "Encounter Groups";
const DEFAULT_SIDE: &str = "enemies";
const SIDE_FIELD: &str = "Default side";
const BELONGS_TO: &str = "Belongs to";
const ADD_TO: &str = "Add to";
const ROSTER: &str = "Roster";
const FOR_ENCOUNTERS: &str = "Groups for encounters";
const ANY_ENCOUNTER: &str = "Any encounter";
const MISSING_ENCOUNTER: &str = "An encounter this campaign no longer has";
const NOTES_FIELD: &str = "Group notes";
const LABEL_WIDTH: f32 = 120.0;
/// The name of a roster slot's button, for the form it opens.
pub(super) const SLOT_OPTIONS: &str =
    "Options: starting HP, temporary HP, initiative group and overrides";

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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct GroupSetup {
    side: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    encounter: Option<EntryRef>,
    members: Vec<MemberDraft>,
}
pub(super) struct GroupDraft {
    path: String,
    baseline: Option<String>,
    source: String,
    name: String,
    setup: GroupSetup,
    notes: String,
    saved_name: String,
    saved_setup: GroupSetup,
    saved_notes: String,
}

/// The note with this text after its frontmatter. The frontmatter stays as it is.
fn with_notes(text: &str, notes: &str) -> Result<String, String> {
    // The frontmatter's own line ending. A body can have another one.
    let start = text.find("---").ok_or("Missing frontmatter")? + 3;
    let newline = if text[start..].starts_with("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let end = format!("{newline}---");
    let close = text[start..].find(&end).ok_or("Missing frontmatter end")? + start + end.len();
    let notes = notes.trim().replace('\n', newline);
    Ok(format!(
        "{}{newline}{newline}{notes}{newline}",
        &text[..close]
    ))
}

impl GroupDraft {
    pub(super) fn new() -> Self {
        let id = crate::hermetic::new_uuid();
        let name = "New group".to_owned();
        let setup = GroupSetup {
            side: DEFAULT_SIDE.into(),
            encounter: None,
            members: Vec::new(),
        };
        Self {
            path: format!("{FOLDER}/{id}.md"),
            baseline: None,
            source: format!(
                "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: encounter_group\nsplat_name: New group\nsplat_encounter_group: {{}}\n---\n\n"
            ),
            name: name.clone(),
            saved_name: name,
            setup: setup.clone(),
            saved_setup: setup,
            notes: String::new(),
            saved_notes: String::new(),
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
        let entry = Entry::parse(path, &text)?;
        if entry.kind != "encounter_group" {
            return Err("Select an encounter group note.".into());
        }
        let setup: GroupSetup = serde_yaml_ng::from_value(
            entry.fields["splat_encounter_group"].clone(),
        )
        .map_err(|e| format!("This group needs repair in Campaign → Campaign entries: {e}"))?;
        let notes = entry.body.trim().to_owned();
        Ok(Self {
            path: path.into(),
            baseline: Some(text.clone()),
            source: text,
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
    }
    fn text(&self) -> Result<String, String> {
        let text = patch_fields(
            &self.source,
            &[
                ("splat_name", Some(self.name.clone().into())),
                (
                    "splat_encounter_group",
                    Some(serde_yaml_ng::to_value(&self.setup).map_err(|e| e.to_string())?),
                ),
            ],
        )?;
        // Text the DM did not change keeps its exact form.
        if self.notes == self.saved_notes {
            Ok(text)
        } else {
            with_notes(&text, &self.notes)
        }
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
            creature: EntryRef::id(id),
            count: 1,
            overrides: Mapping::new(),
        });
    }
    /// An encounter being saved with this group counts as there, though it is not on disk yet.
    fn validate(&self, snapshot: &Snapshot, saving: Option<Uuid>) -> Result<(), String> {
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
                    "this creature has no maximum HP, so a run cannot add it. Set it in this member's overrides."
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
        let text = self.text()?;
        let entry = Entry::parse(&self.path, &text)?;
        if snapshot.catalogue.entries.values().any(|e| {
            e.entry.id == entry.id && (e.owner != campaign.id || e.entry.path != self.path)
        }) {
            return Err("Another note uses this group's identity.".into());
        }
        Ok(text)
    }
    fn save(&mut self, campaign: &Campaign, modules: &[Module]) -> Result<(), String> {
        let snapshot = Snapshot::load(campaign, modules)?;
        let text = self.checked(&snapshot, campaign, None)?;
        // Create the manifest first so a new campaign keeps its package selections.
        let mut document = CampaignDocument::load(campaign)?;
        if document.baseline.is_none() {
            document.save()?;
        }
        self.write(campaign, &text)
    }
    pub(super) fn write(&mut self, campaign: &Campaign, text: &str) -> Result<(), String> {
        let path = note_path(&campaign.path, &self.path)?;
        fs::create_dir_all(path.parent().ok_or("Missing group folder")?)
            .map_err(|e| e.to_string())?;
        write_note(&campaign.path, &self.path, self.baseline.as_deref(), text)?;
        self.source = text.to_owned();
        self.baseline = Some(text.to_owned());
        self.saved_name = self.name.clone();
        self.saved_setup = self.setup.clone();
        self.notes = self.notes.trim().to_owned();
        self.saved_notes = self.notes.clone();
        Ok(())
    }
}

/// What an entry's text says it is worth. Nothing is invented when it says nothing.
fn creature_xp(entry: &Entry) -> Option<u64> {
    entry
        .fields
        .get("splat_creature")
        .and_then(|creature| creature.get("cr_text"))
        .and_then(Value::as_str)
        .and_then(xp_in_text)
        .or_else(|| {
            entry
                .body
                .lines()
                .find(|line| line.trim_start().starts_with("CR "))
                .and_then(xp_in_text)
        })
}

/// Both forms the sources use: `XP 450`, and `450 XP`.
fn xp_in_text(text: &str) -> Option<u64> {
    let number = |digits: Vec<char>| -> Option<u64> {
        let digits: String = digits.into_iter().filter(char::is_ascii_digit).collect();
        digits.parse().ok()
    };
    let in_number = |c: &char| c.is_ascii_digit() || *c == ',';
    if let Some((_, after)) = text.split_once("XP ")
        && let Some(xp) = number(after.chars().take_while(in_number).collect())
    {
        return Some(xp);
    }
    let (before, _) = text.split_once(" XP")?;
    let mut digits: Vec<char> = before.chars().rev().take_while(in_number).collect();
    digits.reverse();
    number(digits)
}

/// A creature's name in an editor: the one it will have. Its changes say which entry it is.
pub(super) fn shown_name(block: &Mapping, name: String) -> String {
    Overrides::parse(block)
        .ok()
        .and_then(|overrides| overrides.name)
        .map_or(name, |changed| changed.trim().to_owned())
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
            let group = EncounterGroup::of(&e.entry).ok()?;
            let mut members = Vec::new();
            let mut problems = Vec::new();
            let mut unavailable = 0_u64;
            let (mut xp, mut xp_unknown) = (0_u64, Vec::new());
            for (index, member) in group.members.iter().enumerate() {
                let what = format!("Member {}", index + 1);
                let Some((id, creature)) = catalogue
                    .resolve(&member.creature)
                    .ok()
                    .and_then(|id| Some((id, creatures.iter().find(|c| c.id == id)?)))
                else {
                    problems.push(
                        "Unavailable creature: check the packages this campaign uses.".to_owned(),
                    );
                    unavailable = unavailable.saturating_add(member.count);
                    continue;
                };
                let entry = &catalogue.entries[&id].entry;
                let resolved = member.overrides.resolve_for(entry, &group.side);
                let issue = if creature.linked {
                    Some(format!("{what}: {} has a D&D Beyond sheet.", entry.name))
                } else {
                    match &resolved {
                        Err(error) => Some(format!("{what} ({}): {error}", entry.name)),
                        Ok(resolved) if resolved.max_hp.is_none() => Some(format!(
                            "{what}: {} has no maximum HP. Set it in this member's overrides.",
                            entry.name
                        )),
                        Ok(_) => None,
                    }
                };
                problems.extend(issue);
                match creature_xp(entry) {
                    Some(each) => xp = u64::saturating_add(xp, each.saturating_mul(member.count)),
                    None => xp_unknown.push(entry.name.clone()),
                }
                members.push(GroupMember {
                    entry: Arc::new(entry.clone()),
                    count: member.count,
                    overrides: member.overrides.clone(),
                });
            }
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
                notes: e.entry.body.trim().to_owned(),
                xp,
                xp_unknown,
                unavailable,
                problem: (!problems.is_empty()).then(|| problems.join(" ")),
            }))
        })
        .collect();
    groups.sort_by_key(|group| group.name.to_lowercase());
    groups
}

/// One group in a list, with its button. Returns true when the DM chooses to edit it.
fn list_row(ui: &mut egui::Ui, group: &RunGroup, blocked: Option<&'static str>) -> bool {
    let mut open = false;
    ui.push_id(group.id, |ui| {
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Scan.row() + design::S);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.set_max_width((ui.available_width() - 80.0).max(100.0));
                ui.add(
                    egui::Label::new(Type::Strong.rich(ui, &group.name).color(colors().text))
                        .truncate(),
                );
                ui.add(
                    egui::Label::new(Type::Small.rich(ui, group.summary()).color(colors().muted))
                        .truncate(),
                );
                if let Some(problem) = &group.problem {
                    ui.add(
                        egui::Label::new(
                            Type::Small.rich(ui, problem).color(colors().result.warning),
                        )
                        .wrap(),
                    );
                }
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                open = Action::quiet("Edit")
                    .density(Density::Read)
                    .name(&format!("Edit group {}", group.name))
                    .blocked(blocked)
                    .show(ui)
                    .clicked();
            });
        });
    });
    open
}

/// The Encounters page's list. The groups for any encounter come first. Each encounter's own
/// stay in its editor and behind one closed section, because this list is for what can arrive
/// anywhere. A group whose encounter is gone stays in view, because it needs a decision. Returns
/// the group the DM chose to edit.
pub(super) fn list(
    ui: &mut egui::Ui,
    snapshot: &Snapshot,
    blocked: Option<&'static str>,
) -> Option<String> {
    let mut open = None;
    let any: Vec<_> = snapshot
        .groups
        .iter()
        .filter(|group| group.encounter.is_none())
        .collect();
    let orphans: Vec<_> = snapshot
        .groups
        .iter()
        .filter(|group| group.encounter.is_some() && group.encounter_name.is_none())
        .collect();
    let owned = snapshot.groups.len() - any.len() - orphans.len();
    ui.label(
        Type::Heading
            .rich(
                ui,
                format!("Encounter groups ({})", any.len() + orphans.len()),
            )
            .color(colors().text),
    );
    design::rule(ui);
    if snapshot.groups.is_empty() {
        ui.add(
            egui::Label::new(
                Type::Body
                    .rich(
                        ui,
                        "No groups yet. Prepare a set of creatures with New group, then add it to a running encounter when it arrives.",
                    )
                    .color(colors().muted),
            )
            .wrap(),
        );
    }
    for group in any {
        if list_row(ui, group, blocked) {
            open = Some(group.path.clone());
        }
    }
    if !orphans.is_empty() {
        ui.add_space(design::XS);
        design::group_label(ui, MISSING_ENCOUNTER);
        for group in orphans {
            if list_row(ui, group, blocked) {
                open = Some(group.path.clone());
            }
        }
    }
    if owned > 0 {
        ui.add_space(design::XS);
        design::section_with_id(
            ui,
            "groups_for_encounters",
            format!("{FOR_ENCOUNTERS} ({owned})"),
            |ui| {
                for (id, name) in snapshot.encounter_choices() {
                    let mut own = snapshot
                        .groups
                        .iter()
                        .filter(|group| group.encounter == Some(id))
                        .peekable();
                    if own.peek().is_some() {
                        design::group_label(ui, &format!("For {name}"));
                    }
                    for group in own {
                        if list_row(ui, group, blocked) {
                            open = Some(group.path.clone());
                        }
                    }
                }
            },
        );
    }
    open
}

/// Which the search adds to: the roster, or one of this encounter's groups.
pub(super) fn add_target(ui: &mut egui::Ui, draft: &mut Draft, snapshot: &Snapshot) {
    if draft.groups.is_empty() {
        return;
    }
    // Beside the search, because the group itself can be out of view or on another tab.
    let result = draft
        .target_group()
        .map(|group| members_heading(group, snapshot));
    let shown = draft
        .target_group()
        .map_or_else(|| ROSTER.to_owned(), |group| group.name.clone());
    let Draft { groups, add_to, .. } = draft;
    ui.horizontal(|ui| {
        ui.label(Type::Small.rich(ui, ADD_TO).color(colors().muted));
        let combo = egui::ComboBox::from_id_salt("add_to")
            .icon(design::combo_icon)
            .width(ui.available_width().min(240.0))
            .truncate()
            .selected_text(shown.as_str())
            .show_ui(ui, |ui| {
                if ui.selectable_label(add_to.is_none(), ROSTER).clicked() {
                    *add_to = None;
                }
                for group in groups.iter() {
                    let chosen = add_to.as_deref() == Some(group.path.as_str());
                    if ui.selectable_label(chosen, &group.name).clicked() {
                        *add_to = Some(group.path.clone());
                    }
                }
            });
        design::named(ui, &combo.response, &format!("{ADD_TO}: {shown}"));
    });
    if let Some(result) = result {
        ui.label(Type::Small.rich(ui, result).color(colors().muted));
    }
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
    ui.label(
        Type::Heading
            .rich(
                ui,
                format!("Groups for this encounter ({})", draft.groups.len()),
            )
            .color(colors().text),
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
        ..
    } = draft;
    for (index, group) in groups.iter_mut().enumerate() {
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
                            // A saved group leaves by Belongs to. This one has no note yet.
                            if group.baseline.is_none()
                                && Action::quiet("Remove")
                                    .density(Density::Read)
                                    .name(&format!("Remove group {}", group.name))
                                    .show(ui)
                                    .clicked()
                            {
                                discard = Some(index);
                            }
                            if Action::secondary("Add creatures")
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
                    group_fields(ui, group, snapshot, expanded, Some((this.0, &this.1)));
                });
            if adding && reveal {
                frame.response.scroll_to_me(Some(egui::Align::Max));
            }
            ui.add_space(design::S);
        });
    }
    if let Some(index) = discard {
        let gone = groups.remove(index);
        if add_to.as_deref() == Some(gone.path()) {
            *add_to = None;
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
                            "Choose another before you save. Only a run of the missing encounter offers this group."
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

const FORM_OPENED: &str = "override_form_opened";

/// The form can open below the fold, so the next frame scrolls to it.
pub(super) fn show_form_when_it_opens(ui: &egui::Ui) {
    ui.data_mut(|data| data.insert_temp(egui::Id::new(FORM_OPENED), true));
}

/// A slot's or member's changes, out in the open, with the values they replace.
pub(super) fn show_changes(
    ui: &mut egui::Ui,
    block: &Mapping,
    entry: &Entry,
    side: &str,
    starting: Option<&Mapping>,
    form_open: bool,
) {
    if block.is_empty() {
        return;
    }
    match Overrides::parse(block) {
        Ok(overrides) => {
            // First, because a number below may be one that cannot stand. An open form says it
            // under its own heading, which is where opening it scrolls to.
            if !form_open && let Err(error) = overrides.resolve_for(entry, side) {
                design::message(ui, Tone::Error, &error);
            }
            let mut base = Base::of(entry, side);
            if let Some(conditions) = starting.and_then(|starting| starting.get("conditions")) {
                base.conditions = condition_names(conditions);
            }
            design::status(
                ui,
                Status {
                    icon: icons::CHANGED,
                    word: "Overrides, here only",
                    color: colors().participant.changed,
                },
            );
            let rows = overrides.rows(&base);
            change_rows(ui, &rows);
            if rows.iter().any(|row| row.by_hand) {
                ui.add(
                    egui::Label::new(Type::Small.rich(ui, BY_HAND).color(colors().muted)).wrap(),
                );
            }
        }
        Err(error) => {
            design::message(ui, Tone::Error, &error);
        }
    }
}

/// A number that can stay the same, be set, or change by an amount. Each control keeps the
/// override valid on its own, so that the form never holds a block that it cannot read.
fn change_row(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut Option<Change>,
    set: Option<std::ops::RangeInclusive<i64>>,
    add: std::ops::RangeInclusive<i64>,
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
                if ui
                    .selectable_label(matches!(value, Some(Change::Add(_))), "Add")
                    .clicked()
                {
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
) {
    let mut overrides = match Overrides::parse(block) {
        Ok(overrides) => overrides,
        Err(error) => {
            design::message(
                ui,
                Tone::Error,
                &format!("{error} Repair it in Campaign → Campaign entries."),
            );
            return;
        }
    };
    let before = overrides.clone();
    let linked = entry.fields.contains_key("splat_external");
    ui.add_space(design::XS);
    let heading = design::group_label(
        ui,
        if sides.is_some() {
            "Overrides, for this encounter only"
        } else {
            "Overrides, for this group only"
        },
    );
    if ui
        .data_mut(|data| data.remove_temp::<bool>(egui::Id::new(FORM_OPENED)))
        .is_some()
    {
        heading.scroll_to_me(Some(egui::Align::Min));
    }
    if let Err(error) = overrides.resolve_for(entry, side) {
        design::message(ui, Tone::Error, &error);
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
        );
    }
    change_row(
        ui,
        "AC",
        &mut overrides.armor_class,
        Some(0..=1000),
        -1000..=1000,
    );
    change_row(
        ui,
        "Initiative bonus",
        &mut overrides.initiative_bonus,
        Some(-1000..=1000),
        -1000..=1000,
    );
    for (label, bonus) in [
        ("Attack bonus", &mut overrides.attack_bonus),
        ("Damage bonus", &mut overrides.damage_bonus),
    ] {
        let mut value = bonus.map(Change::Add);
        change_row(ui, label, &mut value, None, -1000..=1000);
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
                    "Side, over the row's",
                    Type::Small.font(ui),
                    colors().muted,
                );
                let shown = overrides.side.as_deref().unwrap_or("Same");
                let combo = egui::ComboBox::from_id_salt("override_side")
                    .icon(design::combo_icon)
                    .width(120.0)
                    .selected_text(design::reader::human_label(shown))
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(overrides.side.is_none(), "Same")
                            .clicked()
                        {
                            overrides.side = None;
                        }
                        for option in sides {
                            let picked = overrides.side.as_deref() == Some(option.as_str());
                            if ui
                                .selectable_label(picked, design::reader::human_label(option))
                                .clicked()
                            {
                                overrides.side = Some(option.clone());
                            }
                        }
                    });
                design::named(ui, &combo.response, "Changed side");
            });
        }
        None => text_row(ui, "Side", &mut overrides.side, side),
    }
    if !linked || !overrides.conditions.is_empty() {
        conditions_row(ui, &mut overrides.conditions);
    }
    text_row(ui, "Note", &mut overrides.note, "Why this creature differs");
    if overrides != before {
        *block = overrides.to_mapping();
    }
}

/// Names split by commas. The words stay as typed while the field has the keyboard, or a
/// comma just typed would vanish.
fn conditions_row(ui: &mut egui::Ui, conditions: &mut Vec<String>) {
    const NAME: &str = "Starting conditions";
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(
            egui::vec2(LABEL_WIDTH, Density::Read.control()),
            egui::Sense::hover(),
        );
        ui.painter().text(
            rect.left_center(),
            egui::Align2::LEFT_CENTER,
            NAME,
            Type::Small.font(ui),
            colors().muted,
        );
        let id = ui.id().with(NAME);
        let mut text = ui
            .data(|data| data.get_temp::<String>(id))
            .unwrap_or_else(|| conditions.join(", "));
        let width = (ui.available_width() - design::S).clamp(120.0, 320.0);
        let response = design::text_input(ui, NAME, &mut text, "Poisoned, Prone", width);
        if response.has_focus() {
            ui.data_mut(|data| data.insert_temp(id, text.clone()));
        } else {
            ui.data_mut(|data| data.remove_temp::<String>(id));
        }
        if response.changed() {
            let mut seen = Vec::<String>::new();
            for name in text.split(',').map(str::trim).filter(|n| !n.is_empty()) {
                if !seen.iter().any(|other| same_condition(other, name)) {
                    seen.push(name.to_owned());
                }
            }
            *conditions = seen;
        }
    });
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
            Err(e) => self.error = Some(e),
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
        let dirty = self.draft_dirty();
        // The Session page has no save of its own to give this key to.
        let save_key = in_plan
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
        // The title and its actions need more room than the panes do.
        let two_rows = ui.available_width() < 1040.0;
        let title = |ui: &mut egui::Ui, back: &mut bool, draft: &mut GroupDraft| {
            *back = Action::quiet(back_word)
                .icon(icons::BACK)
                .name(back_name)
                .blocked_if(dirty, "Save or discard this group first.")
                .show(ui)
                .clicked();
            let room = if two_rows { 150.0 } else { 450.0 };
            let width = (ui.available_width() - room).clamp(120.0, 360.0);
            let response = ui.add(
                egui::TextEdit::singleline(&mut draft.name)
                    .id(design::focus_id(NAME_FIELD))
                    .desired_width(width)
                    .min_size(egui::vec2(0.0, Density::Scan.control()))
                    .margin(egui::vec2(design::S, 5.0))
                    .font(Type::Heading.font(ui)),
            );
            design::named(ui, &response, &format!("Group name: {}", draft.name));
            if focus_name {
                design::focus_and_select(ui, response.id, &draft.name);
            }
            let (icon, word, color) = if dirty {
                (icons::DRAFT, "Unsaved", colors().document.unsaved)
            } else if let Some(note) = &saved_note {
                (icons::OK, note.as_str(), colors().document.saved)
            } else {
                (icons::OK, "No changes", colors().muted)
            };
            design::status(ui, Status { icon, word, color });
        };
        let actions = |ui: &mut egui::Ui, save: &mut bool, discard: &mut bool| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                *save = if dirty {
                    Action::primary("Save group")
                } else {
                    Action::secondary("Save group")
                }
                .blocked_if(!dirty, "No changes to save.")
                .key("Ctrl+S")
                .show(ui)
                .clicked();
                ui.add_space(design::XL);
                if dirty {
                    *discard = Action::destructive("Discard changes").show(ui).clicked();
                }
            });
        };
        if two_rows {
            ui.horizontal(|ui| title(ui, &mut back, draft));
            ui.horizontal(|ui| actions(ui, &mut save, &mut discard));
        } else {
            ui.horizontal(|ui| {
                title(ui, &mut back, draft);
                actions(ui, &mut save, &mut discard);
            });
        }
        design::rule(ui);
        if back {
            self.close_group_in_plan();
            self.error = None;
            return;
        }
        if discard {
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
            let count: u64 = draft.setup.members.iter().map(|member| member.count).sum();
            let members = format!("Members ({count})");
            design::segmented(
                ui,
                &mut pane,
                &[
                    (EditorPane::Roster, members.as_str()),
                    (EditorPane::Add, "Add creatures"),
                ],
            );
            self.editor_pane = pane;
        }
        let search = &mut self.search;
        let focus_search = &mut self.focus_search;
        let peek = &mut self.peek;
        let expanded = &mut self.expanded;
        if compact {
            match pane {
                EditorPane::Roster => members_pane(ui, draft, snapshot, expanded),
                EditorPane::Add => add_pane(ui, draft, snapshot, search, focus_search, peek),
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
                add_pane(ui, draft, snapshot, search, focus_search, peek);
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
                members_pane(ui, draft, snapshot, expanded);
            });
    }
}

fn add_pane(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    search: &mut String,
    focus_search: &mut bool,
    peek: &mut Option<Uuid>,
) {
    ui.label(Type::Heading.rich(ui, "Add creatures").color(colors().text));
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
    let chosen = Encounters::creature_search(
        ui,
        snapshot.creatures.iter().filter(|c| !c.linked),
        search,
        focus_search,
        peek,
        &|_| false,
    );
    if let Some((id, _)) = chosen {
        draft.add(id);
    }
}

fn members_pane(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    expanded: &mut Option<String>,
) {
    egui::ScrollArea::vertical()
        .id_salt("group_editor")
        .auto_shrink([false, false])
        .show(ui, |ui| group_fields(ui, draft, snapshot, expanded, None));
}

/// What a group is, in its own editor and in the frame an encounter's editor gives it. There,
/// `open` is that encounter, and the frame needs less weight and room.
fn group_fields(
    ui: &mut egui::Ui,
    draft: &mut GroupDraft,
    snapshot: &Snapshot,
    expanded: &mut Option<String>,
    open: Option<(Uuid, &str)>,
) {
    ui.horizontal(|ui| {
        ui.label(Type::Small.rich(ui, SIDE_FIELD).color(colors().muted));
        let name = format!("{SIDE_FIELD} for {}", draft.name);
        design::text_input(ui, &name, &mut draft.setup.side, DEFAULT_SIDE, 160.0);
    });
    if open.is_none() {
        ui.label(
            Type::Small
                .rich(ui, "Party, enemies, or any other name, such as rivals.")
                .color(colors().muted),
        );
        ui.add_space(design::S);
    }
    belongs_to(ui, draft, snapshot, open);
    ui.add_space(design::S);
    // In a frame, the group's name is the heading.
    let (heading, color) = if open.is_some() {
        (Type::Small, colors().muted)
    } else {
        (Type::Heading, colors().text)
    };
    ui.label(
        heading
            .rich(ui, members_heading(draft, snapshot))
            .color(color),
    );
    if draft.setup.members.is_empty() {
        ui.label(
            Type::Body
                .rich(ui, "Add creatures from the list.")
                .color(colors().muted),
        );
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
    let (mut count, mut xp, mut unknown) = (0_u64, 0_u64, false);
    for member in &draft.setup.members {
        count = count.saturating_add(member.count);
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
        "Members: {} · Total XP {xp}{}",
        design::count(count as usize, "creature", "creatures"),
        if unknown { " or more" } else { "" }
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
    for (index, member) in draft.setup.members.iter_mut().enumerate() {
        ui.push_id(index, |ui| {
            let entry = snapshot
                .catalogue
                .resolve_ref(&member.creature)
                .ok()
                .and_then(|id| snapshot.catalogue.entries.get(&id))
                .map(|e| &e.entry);
            let name = entry.map_or_else(
                || "Unavailable creature".to_owned(),
                |entry| entry.name.clone(),
            );
            let button = format!("Overrides for {name}");
            let name = if entry.is_some() {
                shown_name(&member.overrides, name)
            } else {
                name
            };
            let detail = entry
                .and_then(|entry| {
                    let creature = snapshot.creatures.iter().find(|c| c.id == entry.id)?;
                    Some(shown_detail(&member.overrides, entry, &side, creature))
                })
                .unwrap_or_default();
            let key = format!("{path}:member_{index}");
            let is_open = expanded.as_deref() == Some(key.as_str());
            ui.horizontal(|ui| {
                ui.set_min_height(Density::Glance.row());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;
                    if design::icon_button(ui, icons::REMOVE, "Remove").clicked() {
                        remove = Some(index);
                    }
                    if index + 1 < total {
                        if design::icon_button(ui, icons::MOVE_DOWN, "Move down").clicked() {
                            lower = Some(index);
                        }
                    } else {
                        ui.add_space(Density::Read.control());
                    }
                    if index > 0 {
                        if design::icon_button(ui, icons::MOVE_UP, "Move up").clicked() {
                            raise = Some(index);
                        }
                    } else {
                        ui.add_space(Density::Read.control());
                    }
                    // With no creature, there is nothing for a change to change.
                    if entry.is_none() {
                        ui.add_space(0.0);
                    } else if Action::quiet("Overrides")
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
                    ui.add_space(design::XS);
                    let count =
                        ui.add(egui::DragValue::new(&mut member.count).range(1..=MAX_COUNT));
                    design::named(ui, &count, &format!("How many of {name}"));
                    ui.add_space(design::XS);
                    ui.label(Type::Small.rich(ui, "Count").color(colors().muted));
                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.add(
                            egui::Label::new(
                                Type::Strong
                                    .rich(ui, format!("{}. {name}", index + 1))
                                    .color(colors().text),
                            )
                            .truncate(),
                        );
                        if !detail.is_empty() {
                            ui.add(
                                egui::Label::new(
                                    Type::Small.rich(ui, detail).color(colors().muted),
                                )
                                .truncate(),
                            );
                        }
                    });
                });
            });
            if entry.is_none() {
                // The reference is on hover: it is what a repair needs, and not a word to read.
                design::message(
                    ui,
                    Tone::Warning,
                    "This campaign cannot find this creature. Check the packages it uses, or remove this member.",
                )
                .on_hover_text(member.creature.label());
            }
            if let Some(entry) = entry {
                show_changes(ui, &member.overrides, entry, &side, None, is_open);
                if is_open {
                    overrides_form(ui, &mut member.overrides, entry, &side, None);
                }
            }
            design::rule(ui);
        });
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

#[cfg(test)]
mod tests {
    use super::*;

    fn campaign(root: &std::path::Path) -> Campaign {
        Campaign {
            id: Uuid::new_v4(),
            name: "Arena".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        }
    }
    fn creature(root: &std::path::Path, name: &str) -> Uuid {
        let id = Uuid::new_v4();
        write_note(
            root,
            &format!("{name}.md"),
            None,
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: creature\nsplat_name: {name}\nsplat_creature: {{cr_text: '1/4 (XP 50; PB +2)'}}\nsplat_stats:\n  max_hp: {{value: 10}}\n  armor_class: {{value: 15}}\n  initiative: {{value: 2}}\n  speed: {{value: 30}}\n---\n"
            ),
        )
        .unwrap();
        id
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
        let path = draft.path.clone();
        encounters.save_group_draft(&ctx, &campaign, &[]);
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
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter\nsplat_name: Patrol\nsplat_encounter: {{roster: []}}\n---\n",
                Uuid::new_v4()
            ),
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
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter\nsplat_name: Ford\nsplat_encounter: {{roster: []}}\n---\n",
                Uuid::new_v4()
            ),
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
                &format!(
                    "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: encounter\nsplat_name: {name}\nsplat_encounter: {{roster: []}}\n---\n"
                ),
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
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {owner}\nsplat_kind: encounter\nsplat_name: Bridge\nsplat_encounter: {{roster: []}}\n---\n"
            ),
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
            &format!("---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: creature\nsplat_name: Hero\nsplat_creature: {{}}\nsplat_external: {{provider: dndbeyond, character_id: '1'}}\n---\n"),
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
    fn a_group_read_from_disk_says_why_it_cannot_be_added() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let id = Uuid::new_v4();
        write_note(
            dir.path(),
            "Stray.md",
            None,
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Stray\nsplat_encounter_group:\n  side: enemies\n  members:\n  - creature: {{id: {id}}}\n---\n",
                Uuid::new_v4()
            ),
        )
        .unwrap();
        let snapshot = Snapshot::load(&campaign, &[]).unwrap();
        let problem = snapshot.groups[0].problem.clone().unwrap();
        assert!(problem.contains("Unavailable creature"), "{problem}");
        assert!(snapshot.issues.iter().any(|issue| issue.contains("Stray")));
    }

    fn encounter(root: &std::path::Path, name: &str) -> Uuid {
        let id = Uuid::new_v4();
        write_note(
            root,
            &format!("{name}.md"),
            None,
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: encounter\nsplat_name: {name}\nsplat_encounter:\n  roster: []\n  sides: {{}}\n  assumptions: []\n  completion: {{mode: manual}}\n  limits: {{rounds: 1, resolution_events: 1}}\n---\n"
            ),
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
        assert_eq!(group.encounter, Some(arena), "a run of it still offers it");
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
            &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: encounter_group\nsplat_name: Linked\nsplat_encounter_group:\n  side: enemies\n  encounter: {{id: {arena}, link: '[[Arena]]'}}\n  members:\n  - creature: {{id: {scout}}}\n---\n",
                Uuid::new_v4()
            ),
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
    fn a_group_changed_on_disk_is_not_overwritten_and_the_message_says_what_saved() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let scout = creature(dir.path(), "Scout");
        let mut draft = encounter_with_a_group(scout);
        draft.save(&campaign, &[]).unwrap();
        let note = dir.path().join(draft.groups[0].path());
        let external = format!("{}\nFrom outside.\n", fs::read_to_string(&note).unwrap());
        fs::write(&note, &external).unwrap();

        draft.name = "Renamed".into();
        draft.groups[0].name = "Changed".into();
        let error = draft.save(&campaign, &[]).unwrap_err();
        assert!(error.contains("group Changed did not"), "{error}");
        assert_eq!(fs::read_to_string(&note).unwrap(), external);
        assert!(draft.dirty(), "the group's change is still here to save");
        assert_eq!(
            Draft::open(dir.path(), &draft.path).unwrap().name,
            "Renamed"
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
    fn xp_comes_from_the_text_and_is_never_invented() {
        let entry = |creature: &str, body: &str| {
            Entry::parse(
                "c.md",
                &format!(
                    "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: C\nsplat_creature: {creature}\n---\n\n{body}\n",
                    Uuid::new_v4()
                ),
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
        assert_eq!(creature_xp(&entry("{}", "CR 3 (XP 700; PB +2)")), Some(700));
        assert_eq!(creature_xp(&entry("{}", "No such line.")), None);
        assert_eq!(
            creature_xp(&entry("{cr_text: '2 (450 XP; PB +2)'}", "")),
            Some(450),
            "the form four built-in creatures use"
        );
        assert_eq!(
            creature_xp(&entry("{cr_text: '2'}", "CR 2 (XP 450; PB +2)")),
            Some(450),
            "a field that has none does not hide the body's"
        );
        assert_eq!(
            creature_xp(&entry("{cr_text: '0 (XP 0; PB +2)'}", "")),
            Some(0)
        );
    }

    #[test]
    fn notes_go_after_a_frontmatter_whose_line_endings_differ_from_its_body() {
        let text = with_notes("---\nsplat_name: x\n---\n\nold\r\n---\r\nolder\r\n", "new").unwrap();
        assert_eq!(text, "---\nsplat_name: x\n---\n\nnew\n");
        let text = with_notes("---\r\nsplat_name: x\r\n---\r\n\r\nold\r\n", "a\nb").unwrap();
        assert_eq!(text, "---\r\nsplat_name: x\r\n---\r\n\r\na\r\nb\r\n");
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
            &format!("---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: creature\nsplat_name: Blank\nsplat_creature: {{}}\n---\n"),
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
