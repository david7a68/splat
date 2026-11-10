//! Edit, for the DM's own world entries: one form for the status, the stat block, the place and
//! the text. Save writes them together, with one Undo. Lookup and World show the same form.
use super::{
    Change, ChangeKind, Go, Picker, Reader, link_name, reference, status, stored_link, world_field,
};
use crate::{
    design::{self, Action, Density, Tone, Type, colors, reader as style},
    library::{note_path, write_note},
    lookup::{ContentIndex, Item},
    persistence::Campaign,
    sessions,
};
use eframe::egui;
use serde_yaml_ng::Value;
use splat::content::{Entry, Fields as EntryFields};
use std::fs;
use uuid::Uuid;

const SAVED: &str = "Changes saved.";

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
    reference(value)
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
        .map(|at| Value::String(stored_link(&index.items[at].entry)))
        .unwrap_or(value)
}

#[derive(Clone, PartialEq)]
struct Fields {
    status: String,
    creature: Option<Value>,
    place: Option<Value>,
    text: String,
}

/// Which values differ from where the form began. The status ignores outer spaces.
struct Changed {
    status: bool,
    creature: bool,
    place: bool,
    text: bool,
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

    /// Parse a file read from disk. Nothing, when it cannot be read as an entry.
    fn read(path: &str, file: &str) -> Option<Self> {
        Entry::parse(path, file)
            .ok()
            .map(|entry| Self::of(&entry, file))
    }

    fn changed(&self, start: &Self) -> Changed {
        Changed {
            status: self.status.trim() != start.status,
            creature: self.creature != start.creature,
            place: self.place != start.place,
            text: self.text != start.text,
        }
    }
}

/// One entry's form. A draft with changes stays while the DM reads other entries.
pub(super) struct EditDraft {
    name: String,
    path: String,
    /// The entry as the index read it, so a draft without changes can follow the file.
    seen: (EntryFields, String),
    /// The whole file when editing began, or when the DM last chose between versions.
    baseline: String,
    start: Fields,
    now: Fields,
    /// The file on disk, with its values, when it changed under the draft.
    disk: Option<(String, Option<Fields>)>,
    /// The draft before Reload, until the DM edits again.
    reloaded: Option<Fields>,
}

/// Where a disk change under the draft stands.
enum Conflict {
    None,
    Waiting,
    /// The DM chose their draft: save it now.
    Overwrite,
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
            seen: (entry.fields.clone(), entry.body.clone()),
            baseline,
            now: start.clone(),
            start,
            disk: None,
            reloaded: None,
        })
    }

    pub(super) fn dirty(&self) -> bool {
        let changed = self.now.changed(&self.start);
        changed.status || changed.creature || changed.place || changed.text
    }

    /// The index read the entry again since the form saw it. The same front matter shares
    /// one allocation, so the usual answer costs no comparison.
    fn index_moved(&mut self, entry: &Entry) -> bool {
        if std::ptr::eq::<serde_yaml_ng::Mapping>(&*self.seen.0, &*entry.fields)
            && self.seen.1 == entry.body
        {
            return false;
        }
        if *self.seen.0 == *entry.fields && self.seen.1 == entry.body {
            self.seen.0 = entry.fields.clone();
            return false;
        }
        true
    }

    /// Write every changed value and the text in one write. A file changed on disk is kept,
    /// and the draft waits for the DM to choose.
    fn save(
        &mut self,
        campaign: &Campaign,
        index: &ContentIndex,
        log_as: Option<&str>,
    ) -> Result<Option<Change>, String> {
        let disk = fs::read_to_string(note_path(&campaign.path, &self.path)?)
            .map_err(|e| e.to_string())?;
        if disk != self.baseline {
            let theirs = Fields::read(&self.path, &disk);
            self.disk = Some((disk, theirs));
            return Ok(None);
        }
        let changed = self.now.changed(&self.start);
        let mut after = disk.clone();
        if changed.status {
            after = super::fields::patch(&after, "status", self.now.status.trim().into())?;
        }
        for (key, changed, now) in [
            ("creature", changed.creature, &self.now.creature),
            ("in", changed.place, &self.now.place),
        ] {
            if changed {
                let value = now
                    .clone()
                    .map_or(Value::Null, |value| stored_value(index, value));
                after = super::fields::patch(&after, key, value)?;
            }
        }
        if changed.text {
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
        // A status or a place that play changed goes in the Log, as it does outside the form.
        if let Some(stamp) = log_as {
            let null = Value::Null;
            let status = (
                Value::from(self.start.status.as_str()),
                Value::from(self.now.status.trim()),
            );
            let place = (
                self.start.place.as_ref().unwrap_or(&null),
                self.now.place.as_ref().unwrap_or(&null),
            );
            for (key, changed, before, now) in [
                ("status", changed.status, &status.0, &status.1),
                ("in", changed.place, place.0, place.1),
            ] {
                // The same place under another spelling is not a change in play.
                let same = key == "in"
                    && index.world_value_target(before).is_some()
                    && index.world_value_target(before) == index.world_value_target(now);
                if changed && !same {
                    let words = super::log_words(index, key, Some(before), now);
                    after = super::lines::add_log(&after, &format!("{stamp}: {words}"));
                }
            }
        }
        write_note(&campaign.path, &self.path, Some(&disk), &after)?;
        Ok(Some(Change {
            path: self.path.clone(),
            before: disk,
            after,
            word: SAVED.into(),
            kind: ChangeKind::Edit,
        }))
    }

    /// The note changed on disk under the draft. The DM keeps it, or reloads with an Undo.
    fn conflict(&mut self, ui: &mut egui::Ui) -> Conflict {
        let Some((disk, theirs)) = self.disk.take() else {
            return Conflict::None;
        };
        let choice = design::disk_conflict_choices(ui);
        let their_text = theirs.as_ref().map_or("", |theirs| theirs.text.as_str());
        if their_text != self.now.text {
            design::grouped_draft_comparison(ui, &self.now.text, their_text, &self.path, false);
        }
        let Some(choice) = choice else {
            self.disk = Some((disk, theirs));
            return Conflict::Waiting;
        };
        if let Some(theirs) = theirs {
            match choice {
                design::DiskConflictChoice::Reload => {
                    self.reloaded = Some(std::mem::replace(&mut self.now, theirs.clone()));
                }
                design::DiskConflictChoice::Overwrite => {
                    // Only what the DM changed overwrites. The disk version supplies the rest.
                    let changed = self.now.changed(&self.start);
                    let now = &mut self.now;
                    if !changed.status {
                        now.status.clone_from(&theirs.status);
                    }
                    if !changed.creature {
                        now.creature.clone_from(&theirs.creature);
                    }
                    if !changed.place {
                        now.place.clone_from(&theirs.place);
                    }
                    if !changed.text {
                        now.text.clone_from(&theirs.text);
                    }
                }
            }
            self.start = theirs;
        }
        self.baseline = disk;
        match choice {
            design::DiskConflictChoice::Overwrite => Conflict::Overwrite,
            design::DiskConflictChoice::Reload => Conflict::None,
        }
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

    pub fn has_unsaved_edits(&self) -> bool {
        self.edits.values().any(EditDraft::dirty)
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
        let mut lost: Vec<_> = self
            .edits
            .iter()
            .filter(|(id, draft)| index.position(**id).is_none() && draft.dirty())
            .map(|(id, draft)| (*id, draft.name.clone()))
            .collect();
        lost.sort_by(|a, b| a.1.cmp(&b.1));
        for (id, name) in lost {
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
        if !draft.dirty() && draft.disk.is_none() && draft.index_moved(&item.entry) {
            match EditDraft::open(campaign, &item.entry) {
                Ok(fresh) => draft = fresh,
                Err(error) => {
                    // Try again when the index next changes, not on every frame.
                    draft.seen = (item.entry.fields.clone(), item.entry.body.clone());
                    self.error = Some(error);
                }
            }
        }
        ui.add_space(design::M);
        let overwrite = match draft.conflict(ui) {
            Conflict::Waiting => {
                self.edits.insert(id, draft);
                return None;
            }
            Conflict::Overwrite => true,
            Conflict::None => false,
        };
        if draft.reloaded.is_some()
            && design::reloaded_draft(ui)
            && let Some(previous) = draft.reloaded.take()
        {
            draft.now = previous;
        }
        let mut focused = false;
        focused |= design::text_field(ui, "Status", &mut draft.now.status, None, None).has_focus();
        let npc = super::fights(&item.entry);
        for (key, label, slot) in [
            (
                "creature",
                "Stat block",
                npc.then_some(&mut draft.now.creature),
            ),
            ("in", "In", Some(&mut draft.now.place)),
        ] {
            let Some(slot) = slot else {
                continue;
            };
            design::field(ui, label, None, None, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(match &*slot {
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
            });
            if self.picker.as_ref().is_some_and(|picker| picker.key == key)
                && let Some(chosen) = self.pick(ui, item, index, campaign)
            {
                *slot = Some(chosen).filter(|value| !value.is_null());
            }
        }
        let text = design::field(ui, "Text", None, None, |ui| {
            let font = style::prose_type(ui).font(ui);
            ui.add(
                egui::TextEdit::multiline(&mut draft.now.text)
                    .font(font)
                    .desired_width(ui.available_width() - 2.0 * design::S)
                    .desired_rows(8)
                    .margin(egui::vec2(design::S, design::S)),
            )
        });
        design::named(ui, &text, "Text");
        focused |= text.has_focus();
        self.form_focused = focused;
        // Right after Reload the form matches the disk. Any edit ends Undo reload.
        if draft.reloaded.is_some() && draft.now != draft.start {
            draft.reloaded = None;
        }
        let dirty = draft.dirty();
        let reason = (!dirty).then_some("No changes to save.");
        let mut save = overwrite
            || (dirty
                && focused
                && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)));
        let mut cancel = false;
        ui.add_space(design::S);
        ui.horizontal_wrapped(|ui| {
            // Save is primary unless a run in live play owns Next turn on this screen.
            save |= (if self.live {
                Action::secondary("Save changes")
            } else {
                Action::primary("Save changes")
            })
            .density(Density::Read)
            .key("Ctrl+S")
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
            match draft.save(campaign, index, self.log_as.as_deref()) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content_workspace::EntryRef;

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
        let change = draft.save(&campaign, &index, None).unwrap().unwrap();
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
        assert!(draft.save(&campaign, &index, None).unwrap().is_none());
        assert!(draft.disk.is_some());
        fs::write(&path, &after).unwrap();
        change.undo(&campaign).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), before);
    }
}
