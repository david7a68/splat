//! Encounter groups: their notes, their editor, and the overrides form they share with the
//! roster. A group is prepared here and added to a run from the run's own panel.
use super::{
    ADD_PANE_MAX_WIDTH, Creature, EditorPane, Encounters, NAME_FIELD, ROSTER_MAX_WIDTH, Snapshot,
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
    fn validate(&self, snapshot: &Snapshot) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Enter a group name.".into());
        }
        if self.setup.side.trim().is_empty() {
            return Err("Enter a default side for this group.".into());
        }
        if self.setup.members.is_empty() {
            return Err("This group is empty. Add at least one creature.".into());
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
    fn save(&mut self, campaign: &Campaign, modules: &[Module]) -> Result<(), String> {
        let snapshot = Snapshot::load(campaign, modules)?;
        self.validate(&snapshot)?;
        let text = self.text()?;
        let entry = Entry::parse(&self.path, &text)?;
        if snapshot.catalogue.entries.values().any(|e| {
            e.entry.id == entry.id && (e.owner != campaign.id || e.entry.path != self.path)
        }) {
            return Err("Another note uses this group's identity.".into());
        }
        // Create the manifest first so a new campaign keeps its package selections.
        let mut document = CampaignDocument::load(campaign)?;
        if document.baseline.is_none() {
            document.save()?;
        }
        let path = note_path(&campaign.path, &self.path)?;
        fs::create_dir_all(path.parent().ok_or("Missing group folder")?)
            .map_err(|e| e.to_string())?;
        write_note(&campaign.path, &self.path, self.baseline.as_deref(), &text)?;
        self.source = text.clone();
        self.baseline = Some(text);
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

/// The Encounters page's list. Returns the group the DM chose to edit.
pub(super) fn list(
    ui: &mut egui::Ui,
    snapshot: &Snapshot,
    blocked: Option<&'static str>,
) -> Option<String> {
    let mut open = None;
    ui.label(
        Type::Heading
            .rich(ui, format!("Encounter groups ({})", snapshot.groups.len()))
            .color(colors().text),
    );
    design::rule(ui);
    if snapshot.groups.is_empty() {
        ui.add(
            egui::Label::new(
                Type::Body
                    .rich(
                        ui,
                        "No groups yet. Prepare a team or a wave with New group, then add it to a running encounter when it arrives.",
                    )
                    .color(colors().muted),
            )
            .wrap(),
        );
    }
    for group in snapshot.groups.iter() {
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
                        egui::Label::new(
                            Type::Small.rich(ui, group.summary()).color(colors().muted),
                        )
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
                    if Action::quiet("Edit")
                        .density(Density::Read)
                        .name(&format!("Edit group {}", group.name))
                        .blocked(blocked)
                        .show(ui)
                        .clicked()
                    {
                        open = Some(group.path.clone());
                    }
                });
            });
        });
    }
    open
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
        match draft.save(active, modules) {
            Ok(()) => {
                let name = draft.name.trim().to_owned();
                self.saved = true;
                self.error = None;
                self.message = Some("Group saved.".into());
                // The list and an open run read groups from the snapshot.
                self.refreshing = true;
                self.start_load(ctx, active.clone(), modules.to_vec());
                if self.group_in_plan {
                    match old_name {
                        None => self.saved_for_plan = Some(name),
                        Some(old) if old.trim() != name => {
                            self.renamed_in_plan = Some((old.trim().to_owned(), name));
                        }
                        Some(_) => {}
                    }
                    self.close_group_in_plan();
                }
            }
            Err(e) => self.error = Some(e),
        }
    }

    /// The plan shows again. The group stays where the DM left it.
    fn close_group_in_plan(&mut self) {
        self.group_draft = None;
        self.group_in_plan = false;
        self.saved = false;
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
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(Type::Small.rich(ui, SIDE_FIELD).color(colors().muted));
                design::text_input(ui, SIDE_FIELD, &mut draft.setup.side, DEFAULT_SIDE, 160.0);
            });
            ui.label(
                Type::Small
                    .rich(ui, "Party, enemies, or any other name, such as rivals.")
                    .color(colors().muted),
            );
            ui.add_space(design::S);
            ui.label(
                Type::Heading
                    .rich(ui, members_heading(draft, snapshot))
                    .color(colors().text),
            );
            if draft.setup.members.is_empty() {
                ui.label(
                    Type::Body
                        .rich(ui, "Add creatures from the list.")
                        .color(colors().muted),
                );
            }
            members(ui, draft, snapshot, expanded);
            ui.add_space(design::M);
            design::group_label(ui, NOTES_FIELD);
            let response = ui.add(
                egui::TextEdit::multiline(&mut draft.notes)
                    .desired_rows(4)
                    .desired_width(ui.available_width() - design::M)
                    .hint_text("When they arrive, what they want. Markdown is fine."),
            );
            design::named(ui, &response, NOTES_FIELD);
        });
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
            let key = format!("member_{index}");
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
            encounters.renamed_in_plan.take(),
            Some(("Ash Pack".to_owned(), "Cinder Pack".to_owned()))
        );
        assert!(!encounters.group_open_in_plan());
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
