//! Edit, for the DM's own world entries: one form for the status, the stat block, the place and
//! the text. Save writes them together, with one Undo. Lookup and World show the same form.
use super::{Change, Go, Picker, Reader, link_name, reference, status, world_field, world_type};
use crate::{
    content_workspace::EntryRef,
    design::{self, Action, Density, Tone, Type, colors, reader as style},
    library::{note_path, write_note},
    lookup::{ContentIndex, Item},
    persistence::Campaign,
    sessions,
};
use eframe::egui;
use serde_yaml_ng::{Mapping, Value};
use splat::content::Entry;
use std::fs;
use uuid::Uuid;

pub(super) const SAVED: &str = "Changes saved.";

/// Only the DM's own world entries change here. A shared identity must be fixed first.
pub(super) fn editable(item: &Item, index: &ContentIndex) -> bool {
    item.entry.kind == "world"
        && item.is_private()
        && !index.duplicate_world.contains_key(&item.entry.path)
}

/// The front matter, and the title line that the reader shows as the entry's name, with the
/// blank lines around them. The rest is the text the DM edits.
fn split_text<'a>(file: &'a str, name: &str) -> Option<(&'a str, &'a str)> {
    let (_, body) = sessions::split_note(file)?;
    let mut text = body.trim_start_matches(['\r', '\n']);
    let end = text.find('\n').map_or(text.len(), |at| at + 1);
    if text[..end]
        .trim_end()
        .strip_prefix("# ")
        .is_some_and(|title| title.trim().eq_ignore_ascii_case(name.trim()))
    {
        text = text[end..].trim_start_matches(['\r', '\n']);
    }
    Some((&file[..file.len() - text.len()], text))
}

/// The name of an entry a value points at, as the DM knows it.
pub(super) fn value_name(index: &ContentIndex, value: &Value, label: &str) -> String {
    serde_yaml_ng::from_value::<EntryRef>(value.clone())
        .ok()
        .and_then(|reference| reference.reference().ok())
        .and_then(|reference| index.position_of(&reference))
        .map(|at| index.items[at].entry.name.clone())
        .unwrap_or_else(|| {
            if value.is_null() {
                "not set".into()
            } else {
                link_name(value, label)
            }
        })
}

/// What the file keeps for a chosen entry: its key, or a link Obsidian can follow too.
pub(super) fn stored_value(index: &ContentIndex, mut value: Value) -> Value {
    if let Value::Mapping(fields) = &mut value {
        fields.remove("link");
    }
    reference(&value)
        .and_then(|r| index.position_of(&r))
        .map(|at| {
            let target = &index.items[at].entry;
            if let Some(key) = &target.key {
                Value::String(key.clone())
            } else {
                Value::String(format!(
                    "[[{}|{}]]",
                    target.path.trim_end_matches(".md"),
                    target.name
                ))
            }
        })
        .unwrap_or(value)
}

#[derive(Clone, PartialEq)]
struct Fields {
    status: String,
    creature: Option<Value>,
    place: Option<Value>,
    text: String,
}

impl Fields {
    fn of(entry: &Entry, file: &str) -> Self {
        let set = |key| {
            world_field(entry, key)
                .filter(|value| !value.is_null())
                .cloned()
        };
        Self {
            status: status(entry).to_owned(),
            creature: set("creature"),
            place: set("in"),
            text: split_text(file, &entry.name)
                .map_or(file, |(_, text)| text)
                .to_owned(),
        }
    }

    fn get(&mut self, key: &str) -> &mut Option<Value> {
        if key == "in" {
            &mut self.place
        } else {
            &mut self.creature
        }
    }
}

/// One entry's form. A draft with changes stays while the DM reads other entries.
pub(super) struct EditDraft {
    pub name: String,
    path: String,
    /// The entry as the index read it, so a draft without changes can follow the file.
    seen: (Mapping, String),
    /// The whole file when editing began, or when the DM last chose between versions.
    baseline: String,
    start: Fields,
    now: Fields,
    /// The file on disk, when it changed under the draft.
    disk: Option<String>,
}

impl EditDraft {
    fn open(campaign: &Campaign, entry: &Entry) -> Result<Self, String> {
        let baseline = fs::read_to_string(note_path(&campaign.path, &entry.path)?)
            .map_err(|e| e.to_string())?;
        let on_disk = Entry::parse(&entry.path, &baseline)?;
        let start = Fields::of(&on_disk, &baseline);
        Ok(Self {
            name: entry.name.clone(),
            path: entry.path.clone(),
            seen: ((*entry.fields).clone(), entry.body.clone()),
            baseline,
            now: start.clone(),
            start,
            disk: None,
        })
    }

    pub fn dirty(&self) -> bool {
        let (now, start) = (&self.now, &self.start);
        now.status.trim() != start.status
            || now.creature != start.creature
            || now.place != start.place
            || now.text != start.text
    }

    /// Write every changed value and the text in one write. A file changed on disk is kept,
    /// and the draft waits for the DM to choose.
    fn save(
        &mut self,
        campaign: &Campaign,
        index: &ContentIndex,
    ) -> Result<Option<Change>, String> {
        let disk = fs::read_to_string(note_path(&campaign.path, &self.path)?)
            .map_err(|e| e.to_string())?;
        if disk != self.baseline {
            self.disk = Some(disk);
            return Ok(None);
        }
        let mut after = disk.clone();
        let status = self.now.status.trim();
        if status != self.start.status {
            after = super::fields::patch(&after, "status", status.into())?;
        }
        for (key, now, start) in [
            ("creature", &self.now.creature, &self.start.creature),
            ("in", &self.now.place, &self.start.place),
        ] {
            if now != start {
                let value = now
                    .clone()
                    .map_or(Value::Null, |value| stored_value(index, value));
                after = super::fields::patch(&after, key, value)?;
            }
        }
        if self.now.text != self.start.text {
            let (head, _) =
                split_text(&after, &self.name).ok_or("This note has no front matter.")?;
            // A file can end at its closing fence or its title, with no line break yet.
            let newline = if head.ends_with('\n') {
                ""
            } else if head.contains("\r\n") {
                "\r\n"
            } else {
                "\n"
            };
            after = format!("{head}{newline}{}", self.now.text);
        }
        write_note(&campaign.path, &self.path, Some(&disk), &after)?;
        Ok(Some(Change {
            path: self.path.clone(),
            before: disk,
            after,
            word: SAVED.into(),
        }))
    }
}

impl Reader {
    /// Open the form for the entry on show.
    pub(super) fn begin_edit(&mut self, campaign: &Campaign, item: &Item) {
        match EditDraft::open(campaign, &item.entry) {
            Ok(draft) => {
                self.edits.insert(item.entry.id, draft);
                self.discarded = None;
                self.undo = None;
                self.picker = None;
                self.error = None;
            }
            Err(error) => self.error = Some(error),
        }
    }

    pub(super) fn editing(&self, id: Uuid) -> bool {
        self.edits.contains_key(&id)
    }

    /// Entries with changes that are not saved, by name.
    pub fn unsaved_edits(&self) -> Vec<(Uuid, String)> {
        let mut names: Vec<_> = self
            .edits
            .iter()
            .filter(|(_, draft)| draft.dirty())
            .map(|(id, draft)| (*id, draft.name.clone()))
            .collect();
        names.sort_by(|a, b| a.1.cmp(&b.1));
        names
    }

    /// Escape cancels a picker, and leaves a field of the form, before it closes a panel.
    pub fn takes_escape(&self) -> bool {
        self.picker.is_some() || self.form_focused
    }

    /// Start again with nothing on show. Unsaved changes stay.
    pub fn clear(&mut self) {
        *self = Self {
            edits: std::mem::take(&mut self.edits),
            ..Self::default()
        };
    }

    /// Drafts whose note was deleted or given a new identity outside Splat. They cannot open
    /// again, so the DM copies the text or lets the draft go here.
    pub(super) fn lost_edits(&mut self, ui: &mut egui::Ui, index: &ContentIndex) {
        let lost = self.unsaved_edits().into_iter();
        for (id, name) in lost.filter(|(id, _)| index.position(*id).is_none()) {
            design::message(
                ui,
                Tone::Warning,
                &format!(
                    "The note for your unsaved changes to {name} is gone. It was deleted, or given a new identity."
                ),
            );
            ui.push_id(id, |ui| {
                ui.horizontal_wrapped(|ui| {
                    if Action::quiet("Copy text")
                        .icon(design::icons::COPY)
                        .density(Density::Read)
                        .name(&format!("Copy the text of {name}"))
                        .show(ui)
                        .clicked()
                        && let Some(draft) = self.edits.get(&id)
                    {
                        ui.ctx().copy_text(draft.now.text.clone());
                    }
                    if Action::quiet("Discard draft")
                        .density(Density::Read)
                        .name(&format!("Discard the draft of {name}"))
                        .show(ui)
                        .clicked()
                    {
                        self.edits.remove(&id);
                    }
                });
            });
        }
    }

    /// Undo for a Cancel that dropped changes, while the DM reads that entry again.
    pub(super) fn discarded_notice(&mut self, ui: &mut egui::Ui, id: Uuid) {
        if let Some((at, draft)) = &self.discarded
            && *at == id
            && design::draft_discarded(ui, &draft.name)
            && let Some((id, draft)) = self.discarded.take()
        {
            self.edits.insert(id, draft);
        }
    }

    /// The form, in place of the entry's rows and text.
    pub(super) fn edit_form(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: &Campaign,
    ) -> Option<Go> {
        let id = item.entry.id;
        // The draft leaves the map while the form draws, and goes back unless it closes.
        let mut draft = self.edits.remove(&id)?;
        // A rename in Obsidian keeps the identity, so the draft follows the file.
        draft.path.clone_from(&item.entry.path);
        draft.name.clone_from(&item.entry.name);
        if !draft.dirty()
            && draft.disk.is_none()
            && (draft.seen.0 != *item.entry.fields || draft.seen.1 != item.entry.body)
        {
            match EditDraft::open(campaign, &item.entry) {
                Ok(fresh) => draft = fresh,
                Err(error) => self.error = Some(error),
            }
        }
        ui.add_space(design::M);
        if draft.conflict(ui) {
            self.edits.insert(id, draft);
            return None;
        }
        let mut focused = false;
        focused |= design::text_field(ui, "Status", &mut draft.now.status, None, None).has_focus();
        for (key, label) in [("creature", "Stat block"), ("in", "In")] {
            if key == "creature" && world_type(&item.entry) != "npc" {
                continue;
            }
            ui.label(
                Type::Label
                    .rich(ui, label.to_uppercase())
                    .color(colors().muted),
            );
            let value = &*draft.now.get(key);
            ui.horizontal_wrapped(|ui| {
                ui.label(match value {
                    Some(value) => Type::Body.rich(ui, value_name(index, value, label)),
                    None => Type::Body.rich(ui, "not set").color(colors().muted),
                });
                if Action::quiet("Change")
                    .density(Density::Read)
                    .name(&format!("Change {label}"))
                    .show(ui)
                    .clicked()
                {
                    self.picker = Some(Picker {
                        key,
                        focus: true,
                        ..Default::default()
                    });
                }
            });
            if self.picker.as_ref().is_some_and(|picker| picker.key == key)
                && let Some(chosen) = self.pick(ui, index, campaign)
            {
                *draft.now.get(key) = Some(chosen).filter(|value| !value.is_null());
            }
            ui.add_space(design::XS);
        }
        ui.label(Type::Label.rich(ui, "TEXT").color(colors().muted));
        let font = style::prose_type(ui).font(ui);
        let text = ui.add(
            egui::TextEdit::multiline(&mut draft.now.text)
                .font(font)
                .desired_width(ui.available_width() - 2.0 * design::S)
                .desired_rows(8)
                .margin(egui::vec2(design::S, design::S)),
        );
        design::named(ui, &text, "Text");
        focused |= text.has_focus();
        self.form_focused = focused;
        let dirty = draft.dirty();
        let reason = (!dirty).then_some("No changes to save.");
        let mut save = dirty
            && focused
            && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S));
        let mut cancel = false;
        ui.add_space(design::S);
        ui.horizontal_wrapped(|ui| {
            save |= Action::primary("Save changes")
                .density(Density::Read)
                .blocked(reason)
                .show(ui)
                .clicked();
            if dirty {
                design::status(
                    ui,
                    design::Status {
                        icon: design::icons::DRAFT,
                        word: "Unsaved",
                        color: colors().document.unsaved,
                    },
                );
            }
            ui.add_space(design::L);
            cancel = Action::quiet("Cancel")
                .density(Density::Read)
                .show(ui)
                .clicked();
        });
        if cancel {
            self.picker = None;
            self.form_focused = false;
            if draft.dirty() {
                self.discarded = Some((id, draft));
            }
            return None;
        }
        if save {
            match draft.save(campaign, index) {
                Ok(Some(change)) => {
                    self.picker = None;
                    self.form_focused = false;
                    self.undo = Some(change);
                    return Some(Go::Reload(id));
                }
                Ok(None) => {}
                Err(error) => self.error = Some(error),
            }
        }
        self.edits.insert(id, draft);
        None
    }
}

impl EditDraft {
    /// The note changed on disk under the draft: the DM chooses which to keep.
    fn conflict(&mut self, ui: &mut egui::Ui) -> bool {
        let Some(disk) = self.disk.take() else {
            return false;
        };
        design::message(
            ui,
            Tone::Warning,
            "This note changed on disk. Choose which to keep.",
        );
        let (mut use_disk, mut keep) = (false, false);
        ui.horizontal_wrapped(|ui| {
            use_disk = Action::secondary("Use disk version")
                .density(Density::Read)
                .show(ui)
                .clicked();
            keep = Action::secondary("Keep my changes")
                .density(Density::Read)
                .show(ui)
                .clicked();
        });
        let theirs = Entry::parse(&self.path, &disk)
            .ok()
            .map(|entry| Fields::of(&entry, &disk));
        let their_text = theirs.as_ref().map_or("", |theirs| theirs.text.as_str());
        if their_text != self.now.text {
            design::grouped_draft_comparison(ui, &self.now.text, their_text, &self.path, false);
        }
        if !(use_disk || keep) {
            self.disk = Some(disk);
            return true;
        }
        if let Some(theirs) = theirs {
            let (now, start) = (&mut self.now, &self.start);
            if use_disk {
                now.clone_from(&theirs);
            } else {
                // Keep only what the DM changed. The disk version supplies the rest.
                if now.status.trim() == start.status {
                    now.status.clone_from(&theirs.status);
                }
                if now.creature == start.creature {
                    now.creature.clone_from(&theirs.creature);
                }
                if now.place == start.place {
                    now.place.clone_from(&theirs.place);
                }
                if now.text == start.text {
                    now.text.clone_from(&theirs.text);
                }
            }
            self.start = theirs;
        }
        self.baseline = disk;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_title_line_and_front_matter_stay_out_of_the_text() {
        let file = "---\nsplat_name: Rook\n---\n\n# Rook\n\nKeeps the gate.\n";
        assert_eq!(
            split_text(file, "Rook"),
            Some((
                "---\nsplat_name: Rook\n---\n\n# Rook\n\n",
                "Keeps the gate.\n"
            ))
        );
        // Another first heading is part of the text.
        let file = "---\na: 1\n---\n# Patrol\nAt midnight.\n";
        assert_eq!(
            split_text(file, "Rook").unwrap().1,
            "# Patrol\nAt midnight.\n"
        );
        assert_eq!(split_text("---\na: 1\n---", "Rook").unwrap().1, "");
    }

    #[test]
    fn save_writes_every_change_once_keeps_the_identity_and_undoes() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let place = super::super::create_entry(&campaign, "Watch House", "location").unwrap();
        let id = super::super::create_generated(
            &campaign,
            &crate::npc_generator::Generated {
                name: "Rook".into(),
                attributes: ["A scar".into(), "Quiet".into(), "The gate key".into()],
            },
        )
        .unwrap();
        let path = dir.path().join("NPCs/Rook.md");
        let before = fs::read_to_string(&path).unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let entry = &index.items[index.position(id).unwrap()].entry;
        let mut draft = EditDraft::open(&campaign, entry).unwrap();
        assert!(!draft.dirty());
        assert!(
            draft.now.text.starts_with("- **Look:** A scar"),
            "{}",
            draft.now.text
        );
        draft.now.status = " missing ".into();
        draft.now.place = serde_yaml_ng::to_value(EntryRef::id(place)).ok();
        draft.now.text.push_str("\nOwes Nell a favour.\n");
        assert!(draft.dirty());
        let change = draft.save(&campaign, &index).unwrap().unwrap();
        let after = fs::read_to_string(&path).unwrap();
        let saved = Entry::parse("NPCs/Rook.md", &after).unwrap();
        assert_eq!(saved.id, id);
        assert_eq!(status(&saved), "missing");
        assert_eq!(
            world_field(&saved, "in").and_then(Value::as_str),
            Some("[[Locations/Watch House|Watch House]]")
        );
        assert!(after.contains("\n# Rook\n"));
        assert!(after.ends_with("Owes Nell a favour.\n"));
        assert_eq!(
            (change.before.as_str(), change.word.as_str()),
            (before.as_str(), SAVED)
        );

        // A change on disk under a draft writes nothing.
        let entry = Entry::parse("NPCs/Rook.md", &after).unwrap();
        let mut draft = EditDraft::open(&campaign, &entry).unwrap();
        draft.now.status = "dead".into();
        fs::write(&path, format!("{after}From Obsidian.\n")).unwrap();
        assert!(draft.save(&campaign, &index).unwrap().is_none());
        assert!(draft.disk.is_some());
        fs::write(&path, &after).unwrap();
        change.undo(&campaign).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), before);
    }
}
