//! Edit, for the DM's own world entries: one form for the status, the stat block, the place and
//! the text. Save writes them together, with one Undo. Lookup and World show the same form.
use super::{
    Change, ChangeKind, Go, Picker, Reader, link_name, reference, status, stored_link, world_field,
};
use crate::{
    design::{self, Action, Density, Tone, Type, colors, reader as style},
    library::write_note,
    lookup::{ContentIndex, Item},
    persistence::Campaign,
    sessions,
};
use eframe::egui;
use serde_yaml_ng::Value;
use splat::content::{Entry, Fields as EntryFields, layout::Flag};
use uuid::Uuid;

const SAVED: &str = "Changes saved.";
/// What the form says when it closes with nothing to write.
pub(crate) const ALREADY_SAVED: &str = "Your changes are already in the note.";
/// The faint example in an empty Status field: the rule the value must follow.
pub(crate) const STATUS_HINT: &str = "A short word, such as alive.";

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

/// The note a value's link opens, as the reader finds it, else the note its key or id names.
pub(super) fn value_target(index: &ContentIndex, value: &Value) -> Option<usize> {
    index
        .world_value_target(value)
        .or_else(|| reference(value).and_then(|reference| index.position_of(&reference)))
}

/// A value's name as the reader shows it: the name of the note its link opens, else the words
/// written in the file.
pub(super) fn value_name(index: &ContentIndex, value: &Value, label: &str) -> String {
    value_target(index, value)
        .map(|at| index.items[at].entry.name.clone())
        .unwrap_or_else(|| {
            if super::empty_optional(value) {
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
    groups: Vec<Value>,
    groups_unread: Option<super::fields::Unread>,
    text: String,
}

/// Which values differ from where the form began. The status ignores outer spaces.
struct Changed {
    status: bool,
    creature: bool,
    place: bool,
    groups: bool,
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
            groups: super::part_of(entry).into_iter().cloned().collect(),
            groups_unread: super::fields::unread_groups(entry, file),
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

    /// The groups Overwrite saves, from the form's start: the draft's additions and removals
    /// applied to the disk list. An unread disk list stays as it is.
    fn overwritten_groups(&self, draft: &Self, disk: &Self, index: &ContentIndex) -> Vec<Value> {
        if disk.groups_unread.is_some() {
            return disk.groups.clone();
        }
        super::merge_groups(index, &self.groups, &draft.groups, &disk.groups)
    }

    fn changed(&self, start: &Self) -> Changed {
        Changed {
            status: self.status.trim() != start.status,
            creature: self.creature != start.creature,
            place: self.place != start.place,
            groups: self.groups != start.groups,
            text: self.text != start.text,
        }
    }
}

/// One entry's form. A draft with changes stays while the DM reads other entries.
pub(super) struct EditDraft {
    /// The campaign whose note this is. Its Undo discard and its lost-note warning show only
    /// there: Lookup keeps its reader across a campaign change.
    campaign: Uuid,
    name: String,
    path: String,
    /// The entry as the index read it, so a draft without changes can follow the file.
    seen: (EntryFields, String, Vec<Flag>),
    /// The whole file when editing began, or when the DM last chose between versions.
    baseline: String,
    start: Fields,
    now: Fields,
    /// The file on disk, with its values, when it changed under the draft.
    disk: Option<(String, Option<Fields>)>,
    /// The draft before Reload, with the values it started from (`start`, then `now`), until the
    /// DM edits again. The start lets a later Overwrite count only the DM's own changes.
    reloaded: Option<(Fields, Fields)>,
    /// Overwrite kept the note's unread groups in place of the draft's group change.
    groups_refused: bool,
}

/// What Save did.
enum Saved {
    /// The note changed on disk under the draft. The draft waits for the DM to choose.
    Waiting,
    /// The note on disk already reads as the draft would write it, so nothing was written. It
    /// holds that text.
    Unchanged(String),
    Written(Change),
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
        let baseline = super::file_fault::read_note(&campaign.path, &entry.path)?;
        let on_disk = Entry::parse(&entry.path, &baseline)?;
        let start = Fields::of(&on_disk, &baseline);
        Ok(Self {
            campaign: campaign.id,
            name: entry.name.clone(),
            path: entry.path.clone(),
            seen: (
                entry.fields.clone(),
                entry.body.clone(),
                entry.flags.clone(),
            ),
            baseline,
            now: start.clone(),
            start,
            disk: None,
            reloaded: None,
            groups_refused: false,
        })
    }

    pub(super) fn dirty(&self) -> bool {
        let changed = self.now.changed(&self.start);
        changed.status || changed.creature || changed.place || changed.groups || changed.text
    }

    /// The index read the entry again since the form saw it. Shared fields avoid a front
    /// matter comparison. Flags also change when repair removes an unread property.
    fn index_moved(&mut self, entry: &Entry) -> bool {
        if self.seen.2 != entry.flags {
            return true;
        }
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
    ) -> Result<Saved, String> {
        // A draft writes only into its own campaign's note.
        if campaign.id != self.campaign {
            return Err("This draft belongs to another campaign. Nothing was saved.".into());
        }
        let disk = super::file_fault::read_note(&campaign.path, &self.path)?;
        if disk != self.baseline {
            let theirs = Fields::read(&self.path, &disk);
            self.disk = Some((disk, theirs));
            return Ok(Saved::Waiting);
        }
        let changed = self.now.changed(&self.start);
        if changed.groups
            && let Some(unread) = &self.start.groups_unread
        {
            return Err(unread.reason.clone());
        }
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
        if changed.groups {
            let groups = self
                .now
                .groups
                .iter()
                .cloned()
                .map(|value| stored_value(index, value))
                .collect();
            after = super::fields::patch(&after, "part_of", Value::Sequence(groups))?;
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
            let groups = (
                Value::Sequence(self.start.groups.clone()),
                Value::Sequence(self.now.groups.clone()),
            );
            for (key, changed, before, now) in [
                ("status", changed.status, &status.0, &status.1),
                ("in", changed.place, place.0, place.1),
                ("part_of", changed.groups, &groups.0, &groups.1),
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
        // For example, Overwrite adds a group that Obsidian added too. A write and an Undo
        // would change nothing.
        if after == disk {
            return Ok(Saved::Unchanged(disk));
        }
        write_note(&campaign.path, &self.path, Some(&disk), &after)?;
        Ok(Saved::Written(Change {
            campaign: campaign.id,
            path: self.path.clone(),
            before: disk,
            after,
            word: SAVED.into(),
            kind: ChangeKind::Edit,
        }))
    }

    /// What Overwrite does to each field that differs between the draft and the note on disk.
    fn overwrite_effects(
        &self,
        theirs: &Fields,
        index: &ContentIndex,
    ) -> Vec<(&'static str, String)> {
        let changed = self.now.changed(&self.start);
        let named = |value: &Option<Value>, label: &str| {
            value
                .as_ref()
                .map_or_else(|| "not set".into(), |value| value_name(index, value, label))
        };
        // A link names the note it opens and `not set` is no value, so neither has marks. Status
        // words are the DM's own text, so they have curly marks (`status_effect` below).
        let effect = |draft_changed: bool, draft: String, disk: String| {
            if draft_changed {
                format!("Overwrite saves {draft} in place of {disk}.")
            } else {
                format!("Overwrite keeps {disk} from the note.")
            }
        };
        let mut effects = Vec::new();
        let status = |value: &str| {
            if value.trim().is_empty() {
                "not set".to_owned()
            } else {
                value.trim().to_owned()
            }
        };
        // The Read order: Stat block, In, Part of, then Status.
        for (label, draft_changed, now, disk) in [
            (
                "Stat block",
                changed.creature,
                &self.now.creature,
                &theirs.creature,
            ),
            ("In", changed.place, &self.now.place, &theirs.place),
        ] {
            if now != disk {
                effects.push((
                    label,
                    effect(draft_changed, named(now, label), named(disk, label)),
                ));
            }
        }
        if let Some(super::fields::Unread { line, .. }) = &theirs.groups_unread {
            effects.push((
                "Part of",
                format!("Overwrite keeps the note’s groups. They could not be read (line {line})."),
            ));
        } else if self.now.groups != theirs.groups {
            let saved = self.start.overwritten_groups(&self.now, theirs, index);
            let names = |groups: Vec<&Value>| {
                let names: Vec<_> = groups
                    .into_iter()
                    .map(|group| value_name(index, group, "group"))
                    .collect();
                match names.split_last() {
                    Some((last, [])) => last.clone(),
                    Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
                    None => String::new(),
                }
            };
            // Groups compare by target. Each list is resolved once per frame.
            let saved_keys = super::group_keys(index, &saved);
            let disk_keys = super::group_keys(index, &theirs.groups);
            let start_keys = super::group_keys(index, &self.start.groups);
            let added: Vec<_> = saved
                .iter()
                .zip(&saved_keys)
                .filter(|(_, key)| !disk_keys.contains(key))
                .map(|(group, _)| group)
                .collect();
            let removed: Vec<_> = theirs
                .groups
                .iter()
                .zip(&disk_keys)
                .filter(|(_, key)| !saved_keys.contains(key))
                .map(|(group, _)| group)
                .collect();
            let outside: Vec<_> = theirs
                .groups
                .iter()
                .zip(&disk_keys)
                .filter(|(_, key)| !start_keys.contains(key) && saved_keys.contains(key))
                .map(|(group, _)| group)
                .collect();
            let mut parts = Vec::new();
            if !added.is_empty() {
                parts.push(format!("adds {}", names(added)));
            }
            if !removed.is_empty() {
                parts.push(format!("removes {}", names(removed)));
            }
            if !outside.is_empty() {
                parts.push(format!("keeps {} from the note", names(outside)));
            }
            if parts.is_empty() {
                parts.push("keeps the note’s groups".into());
            }
            let last = parts.pop().unwrap_or_default();
            let words = if parts.is_empty() {
                last
            } else {
                format!("{} and {last}", parts.join(", "))
            };
            effects.push(("Part of", format!("Overwrite {words}.")));
        }
        if self.now.status.trim() != theirs.status.trim() {
            let quoted = |value: &str| match status(value) {
                words if value.trim().is_empty() => words,
                words => format!("“{words}”"),
            };
            effects.push((
                "Status",
                effect(
                    changed.status,
                    quoted(&self.now.status),
                    quoted(&theirs.status),
                ),
            ));
        }
        if self.now.text != theirs.text {
            effects.push((
                "Text",
                if changed.text {
                    "Overwrite saves your text. The lines that differ are below.".into()
                } else {
                    "Overwrite keeps the note’s text. The lines that differ are below.".into()
                },
            ));
        }
        effects
    }

    /// The note changed on disk under the draft. The DM keeps it, or reloads with an Undo.
    fn conflict(&mut self, ui: &mut egui::Ui, index: &ContentIndex) -> Conflict {
        let Some((disk, theirs)) = self.disk.take() else {
            return Conflict::None;
        };
        let choice = design::disk_conflict_choices(ui, "note");
        if let Some(theirs) = &theirs {
            let effects = self.overwrite_effects(theirs, index);
            if !effects.is_empty() {
                ui.add_space(design::S);
            }
            for (label, words) in effects {
                ui.horizontal_top(|ui| {
                    ui.allocate_ui_with_layout(
                        egui::vec2(design::XXL * 3.0 + design::S, 0.0),
                        egui::Layout::left_to_right(egui::Align::Min),
                        |ui| {
                            ui.set_min_width(design::XXL * 3.0 + design::S);
                            ui.label(Type::Small.rich(ui, label).color(colors().muted));
                        },
                    );
                    ui.add(egui::Label::new(Type::Small.rich(ui, words)).wrap());
                });
            }
        }
        let their_text = theirs.as_ref().map_or("", |theirs| theirs.text.as_str());
        if their_text != self.now.text {
            design::grouped_draft_comparison(ui, &self.now.text, their_text, &self.path, false);
        }
        let Some(choice) = choice else {
            self.disk = Some((disk, theirs));
            return Conflict::Waiting;
        };
        self.resolve(disk, theirs, choice, index)
    }

    /// Apply the DM's choice between the draft and the file on disk.
    fn resolve(
        &mut self,
        disk: String,
        theirs: Option<Fields>,
        choice: design::DiskConflictChoice,
        index: &ContentIndex,
    ) -> Conflict {
        self.groups_refused = false;
        if let Some(theirs) = theirs {
            match choice {
                design::DiskConflictChoice::Reload => {
                    let previous = std::mem::replace(&mut self.now, theirs.clone());
                    self.reloaded = Some((self.start.clone(), previous));
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
                    self.groups_refused = changed.groups && theirs.groups_unread.is_some();
                    now.groups = self.start.overwritten_groups(now, &theirs, index);
                    now.groups_unread.clone_from(&theirs.groups_unread);
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

/// The campaign this index was read for; nil for an index with none.
fn index_campaign(index: &ContentIndex) -> Uuid {
    index
        .campaign
        .as_ref()
        .map_or(Uuid::nil(), |campaign| campaign.id)
}

/// The choices of a message about words that have no place now: quiet `Copy text` with its
/// icon, then, an `L` gap after it, `let_go` with the secondary border, which lets the words go.
/// A lost draft and a gone entry's Log line both use it. Returns whether each was pressed.
pub(super) fn copy_or_let_go(
    ui: &mut egui::Ui,
    copy_name: &str,
    let_go: &str,
    let_go_name: &str,
) -> (bool, bool) {
    ui.horizontal_wrapped(|ui| {
        let copy = Action::quiet("Copy text")
            .icon(design::icons::COPY)
            .density(Density::Read)
            .name(copy_name);
        let copied = super::starts_line(ui, copy, true).show(ui);
        // The second choice lets the DM's words go, so an `L` gap keeps it apart. The gap
        // starts at Copy text's target, which `line_start` draws `S` left of its place in the row.
        ui.add_space((copied.rect.right() + design::L - ui.cursor().left()).max(0.0));
        // A message action without an icon has the secondary border.
        let let_go = Action::secondary(let_go)
            .density(Density::Read)
            .name(let_go_name)
            .show(ui)
            .clicked();
        (copied.clicked(), let_go)
    })
    .inner
}

impl super::GoneLog {
    /// The notice with its choices. `Copy text` puts the typed words on the clipboard; either
    /// choice ends the notice.
    pub(super) fn draw(
        &self,
        ui: &mut egui::Ui,
        words: &str,
        tone: Tone,
    ) -> Option<super::FooterAct> {
        let (copied, dismissed) = design::message_with_actions(ui, tone, words, |ui| {
            copy_or_let_go(
                ui,
                &format!("Copy the Log line for {}", self.name),
                "Dismiss",
                &format!("Dismiss the Log line for {}", self.name),
            )
        });
        if copied {
            ui.ctx().copy_text(self.words.clone());
        }
        (copied || dismissed).then_some(super::FooterAct::GoneLog(self.id))
    }
}

/// A lost draft's warning with its choices: `Copy text`, then, an `L` gap on, `Discard draft`.
pub(super) fn lost_draft(
    ui: &mut egui::Ui,
    words: &str,
    tone: Tone,
    id: Uuid,
    name: &str,
) -> Option<super::FooterAct> {
    // The choices start at the message text, as every message's choices do.
    let (copied, discard) = ui
        .push_id(id, |ui| {
            design::message_with_actions(ui, tone, words, |ui| {
                copy_or_let_go(
                    ui,
                    &format!("Copy the text of {name}"),
                    "Discard draft",
                    &format!("Discard the draft of {name}"),
                )
            })
        })
        .inner;
    if copied {
        Some(super::FooterAct::LostCopy(id))
    } else {
        discard.then_some(super::FooterAct::LostDiscard(id))
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
                self.kept = None;
                self.picker = None;
                self.error = None;
            }
            Err(error) => self.error = Some(error),
        }
    }

    /// A form is open for this campaign's note. A draft from another campaign with the same
    /// note id (a copied note) is never selected: Lookup keeps its reader across a change.
    pub(super) fn editing(&self, id: Uuid, campaign: &Campaign) -> bool {
        self.edits
            .get(&id)
            .is_some_and(|draft| draft.campaign == campaign.id)
    }

    /// An edit form with changes, or a Log line with typed words, that is not saved.
    pub fn has_unsaved_edits(&self) -> bool {
        self.edits.values().any(EditDraft::dirty) || self.typed_log_lines().next().is_some()
    }

    /// Entries with changes that are not saved, by name: edit forms, and typed Log lines.
    pub fn unsaved_edits(&self) -> Vec<(Uuid, String)> {
        let mut names: Vec<_> = self
            .edits
            .iter()
            .filter(|(_, draft)| draft.dirty())
            .map(|(id, draft)| (*id, draft.name.clone()))
            .chain(self.typed_log_lines())
            .collect();
        names.sort_by(|a, b| a.1.cmp(&b.1));
        names
    }

    /// Escape cancels a picker, and leaves a field of the form, before it closes a panel.
    pub fn takes_escape(&self) -> bool {
        self.picker.is_some() || self.form_focused
    }

    /// A new reader that keeps only this one's typed Log lines, and the words of those whose
    /// entry is gone.
    pub(crate) fn keep_log_lines(mut old: Self) -> Self {
        Self {
            log_fields: std::mem::take(&mut old.log_fields),
            gone_log_lines: std::mem::take(&mut old.gone_log_lines),
            ..Self::default()
        }
    }

    /// Start again with nothing on show. Unsaved changes stay: edit forms, Log lines, and the
    /// words of Log lines whose entry is gone.
    pub fn clear(&mut self) {
        *self = Self {
            edits: std::mem::take(&mut self.edits),
            log_fields: std::mem::take(&mut self.log_fields),
            gone_log_lines: std::mem::take(&mut self.gone_log_lines),
            ..Self::default()
        };
    }

    /// The Log lines with typed words, by entry and name.
    fn typed_log_lines(&self) -> impl Iterator<Item = (Uuid, String)> + '_ {
        self.log_fields
            .iter()
            .filter(|(_, open)| !open.field.words.trim().is_empty())
            .map(|((_, id), open)| (*id, open.name.clone()))
    }

    /// Drop the Log fields of this index's campaign whose entry is gone (deleted, or another
    /// note took its ID), so no guard waits on words that no field can show. Typed words are
    /// not lost without a word: the pane's footer says them, with `Copy text`, until the DM
    /// copies or dismisses them.
    pub fn forget_gone_log_lines(&mut self, index: &ContentIndex) {
        if self.log_fields.is_empty() {
            return;
        }
        let campaign = index_campaign(index);
        let gone: Vec<_> = self
            .log_fields
            .keys()
            .filter(|(owner, id)| *owner == campaign && index.position(*id).is_none())
            .copied()
            .collect();
        for key in gone {
            if let Some(open) = self.log_fields.remove(&key)
                && !open.field.words.trim().is_empty()
            {
                // Each lost draft keeps its own notice, also a second one from the same entry.
                self.gone_log_lines.push(super::GoneLog {
                    id: super::next_order(),
                    campaign: key.0,
                    name: open.name,
                    words: open.field.words,
                });
            }
        }
    }

    /// The words of Log lines whose entry is gone, as footer notices, for the campaign that is
    /// open. Each shows, whatever the reader shows, until the DM copies or dismisses it
    /// (`FooterAct::GoneLog`); another campaign's notice waits, unseen, for its campaign.
    pub(crate) fn gone_log_notices(&self, campaign: Option<Uuid>) -> Vec<super::FooterLine> {
        self.gone_log_lines
            .iter()
            .filter(|gone| Some(gone.campaign) == campaign)
            .map(|gone| {
                super::FooterLine::new(
                    super::FooterRank::Notice,
                    Tone::Warning,
                    &format!(
                        "\u{201c}{}\u{201d} was deleted. The Log line was not added: \u{201c}{}\u{201d}",
                        gone.name,
                        gone.words.trim()
                    ),
                    super::FooterKind::GoneLog(gone.clone()),
                )
            })
            .collect()
    }

    /// The DM copied or dismissed the words of this notice.
    pub(crate) fn end_gone_log(&mut self, id: u64) {
        self.gone_log_lines.retain(|gone| gone.id != id);
    }

    /// Put the entries whose edit form is open in this reader in `out`, with no new
    /// allocation once `out` has room.
    pub fn edit_forms_into(&self, out: &mut Vec<Uuid>) {
        out.clear();
        out.extend(self.edits.keys().copied());
    }

    /// Drafts whose note was deleted or given a new identity outside Splat, as footer lines of
    /// the reader's pane (round 27 FE6, FC1): they are no note's state, so they never sit in the
    /// body of the note on show. They cannot open again, so the DM copies the text or lets the
    /// draft go (`FooterAct::LostCopy`, `FooterAct::LostDiscard`). Each shows for its own
    /// campaign only, whatever note is open, in the order of the names. A draft let go stays in
    /// `discarded` for one `Undo discard` until the next Edit, the next Discard or the Undo.
    pub(super) fn lost_lines(&self, index: &ContentIndex) -> Vec<super::FooterLine> {
        let here = index_campaign(index);
        let mut lost: Vec<_> = self
            .edits
            .iter()
            .filter(|(id, draft)| {
                draft.campaign == here && index.position(**id).is_none() && draft.dirty()
            })
            .map(|(id, draft)| (*id, draft.name.as_str()))
            .collect();
        lost.sort_by(|a, b| a.1.cmp(b.1));
        let mut lines: Vec<_> = lost
            .into_iter()
            .map(|(id, name)| {
                super::FooterLine::new(
                    super::FooterRank::Notice,
                    Tone::Warning,
                    &format!(
                        "The note for your unsaved changes to {name} is gone. It was deleted, or another note took its place."
                    ),
                    super::FooterKind::LostDraft {
                        id,
                        name: name.to_owned(),
                    },
                )
            })
            .collect();
        if let Some((at, draft)) = &self.discarded
            && draft.campaign == here
            && index.position(*at).is_none()
        {
            lines.push(super::FooterLine::discarded(
                &draft.name,
                super::FooterAct::Discarded,
            ));
        }
        lines
    }

    /// `Copy text` of a lost draft: the draft's text on the clipboard; the draft stays.
    pub(super) fn copy_lost(&self, ctx: &egui::Context, id: Uuid) {
        if let Some(draft) = self.edits.get(&id) {
            ctx.copy_text(draft.now.text.clone());
        }
    }

    /// `Discard draft` of a lost draft: it waits in `discarded` for `Undo discard`.
    pub(super) fn discard_lost(&mut self, id: Uuid) {
        if let Some(draft) = self.edits.remove(&id) {
            self.discarded = Some((id, draft));
            self.stamp(super::FooterAct::Discarded);
        }
    }

    /// Undo for a Discard that dropped changes, while the DM reads that entry again. It sits in
    /// the pane's footer; `FooterAct::Discarded` brings the draft back.
    pub(super) fn discarded_line(
        &self,
        campaign: &Campaign,
        id: Uuid,
    ) -> Option<super::FooterLine> {
        let (at, draft) = self.discarded.as_ref()?;
        (*at == id && draft.campaign == campaign.id)
            .then(|| super::FooterLine::discarded(&draft.name, super::FooterAct::Discarded))
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
        if draft.campaign != campaign.id {
            self.edits.insert(id, draft);
            return None;
        }
        // A rename in Obsidian keeps the identity, so the draft follows the file.
        draft.path.clone_from(&item.entry.path);
        draft.name.clone_from(&item.entry.name);
        if !draft.dirty() && draft.disk.is_none() && draft.index_moved(&item.entry) {
            match EditDraft::open(campaign, &item.entry) {
                // The draft already holds this file, as after a save conflict's reload. Keep it,
                // so `Undo reload` stays until the file changes again.
                Ok(fresh) if fresh.baseline == draft.baseline => draft.seen = fresh.seen,
                // The file changed again after Reload. The DM's held draft comes back and the
                // choice shows again, so it is not lost to a version the DM has not seen.
                Ok(fresh) if draft.reloaded.is_some() => {
                    if let Some((start, previous)) = draft.reloaded.take() {
                        // Its own start too, so changes on disk do not count as the DM's.
                        draft.start = start;
                        draft.now = previous;
                    }
                    let theirs = Fields::read(&draft.path, &fresh.baseline);
                    draft.disk = Some((fresh.baseline, theirs));
                    draft.seen = fresh.seen;
                }
                Ok(fresh) => draft = fresh,
                Err(error) => {
                    // Try again when the index next changes, not on every frame.
                    draft.seen = (
                        item.entry.fields.clone(),
                        item.entry.body.clone(),
                        item.entry.flags.clone(),
                    );
                    self.error = Some(error);
                }
            }
        }
        ui.add_space(design::M);
        // The frame the DM chooses Reload: its result, below the save row, scrolls into view.
        let had_conflict = draft.disk.is_some();
        let overwrite = match draft.conflict(ui, index) {
            Conflict::Waiting => {
                self.edits.insert(id, draft);
                return None;
            }
            Conflict::Overwrite => true,
            Conflict::None => false,
        };
        let mut focused = false;
        // The Read order: Stat block, In, Part of, then Status.
        let npc = super::fights(&item.entry);
        let groups_error = draft
            .start
            .groups_unread
            .as_ref()
            .map(|unread| unread.reason.clone());
        if groups_error.is_some()
            && self
                .picker
                .as_ref()
                .is_some_and(|picker| picker.key == "part_of")
        {
            self.picker = None;
        }
        // Each value as the form draws it, with its colour and size, as Read draws it: a set value
        // is `Body` in the text colour, also a group (round 27 FF2); `not set` is muted `Small`
        // and `not read` a `Small` warning, not names (FF3).
        let slot_words = |slot: &Option<Value>, label: &str| match slot {
            Some(value) if !super::empty_optional(value) => {
                (value_name(index, value, label), colors().text, Type::Body)
            }
            _ => ("not set".to_owned(), colors().muted, Type::Small),
        };
        let group_words = if groups_error.is_some() {
            ("not read".to_owned(), colors().result.warning, Type::Small)
        } else if draft.now.groups.is_empty() {
            ("not set".to_owned(), colors().muted, Type::Small)
        } else {
            (
                draft
                    .now
                    .groups
                    .iter()
                    .map(|value| value_name(index, value, "group"))
                    .collect::<Vec<_>>()
                    .join(", "),
                colors().text,
                Type::Body,
            )
        };
        let words = [
            npc.then(|| slot_words(&draft.now.creature, "Stat block")),
            Some(slot_words(&draft.now.place, "In")),
            Some(group_words.clone()),
        ];
        // One Change column for Stat block, In and Part of (AI1): an `L` gap after the widest
        // value, which wraps within the column when the form is narrow.
        let change_width = Action::quiet("Change")
            .density(Density::Read)
            .natural_width(ui);
        let value_width = words
            .iter()
            .flatten()
            .map(|(words, _, role)| design::number_width(ui, words, &role.font(ui)))
            .fold(0.0_f32, f32::max)
            .min(
                (ui.available_width().min(design::FORM_WIDTH) - change_width - design::L)
                    .max(design::XXL),
            );
        let value_and_change = |ui: &mut egui::Ui,
                                (words, color, role): (String, egui::Color32, Type),
                                change: Action<'_>| {
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(value_width, Density::Read.control()),
                    egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true),
                    |ui| {
                        ui.set_width(value_width);
                        ui.add(egui::Label::new(role.rich(ui, words).color(color)).wrap());
                    },
                );
                ui.add_space((design::L - ui.spacing().item_spacing.x).max(0.0));
                change.show(ui).clicked()
            })
            .inner
        };
        for (key, label, slot, words) in [
            (
                "creature",
                "Stat block",
                npc.then_some(&mut draft.now.creature),
                words[0].clone(),
            ),
            ("in", "In", Some(&mut draft.now.place), words[1].clone()),
        ] {
            let (Some(slot), Some(words)) = (slot, words) else {
                continue;
            };
            design::field(ui, label, None, None, |ui| {
                if value_and_change(
                    ui,
                    words,
                    Action::quiet("Change")
                        .density(Density::Read)
                        .name(&format!("Change {label}")),
                ) {
                    self.picker = Some(Picker {
                        key,
                        focus: true,
                        ..Default::default()
                    });
                }
            });
            if self.picker.as_ref().is_some_and(|picker| picker.key == key)
                && let Some(chosen) = self.pick(ui, item, index, campaign, None)
            {
                *slot = Some(chosen).filter(|value| !value.is_null());
            }
        }
        design::field(ui, "Part of", None, None, |ui| {
            if value_and_change(
                ui,
                group_words,
                Action::quiet("Change")
                    .density(Density::Read)
                    .name("Change Part of")
                    .blocked(groups_error.as_deref()),
            ) {
                self.picker = Some(Picker {
                    key: "part_of",
                    focus: true,
                    ..Default::default()
                });
            }
        });
        if let Some(reason) = &groups_error {
            // The warning keeps the form's right edge.
            ui.scope(|ui| {
                ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
                let whole = draft
                    .start
                    .groups_unread
                    .as_ref()
                    .map_or(reason.as_str(), |unread| unread.whole.as_str());
                super::unread_groups_message(ui, campaign, &draft.path, reason, whole);
            });
        }
        if self
            .picker
            .as_ref()
            .is_some_and(|picker| picker.key == "part_of")
            && let Some(Value::Sequence(groups)) =
                self.pick(ui, item, index, campaign, Some(&draft.now.groups))
        {
            draft.now.groups = groups;
        }
        // The shared text field, with a faint example of the one rule the value has.
        focused |= design::field(ui, "Status", None, None, |ui| {
            let width = ui.available_width().min(design::FORM_WIDTH) - 2.0 * design::S;
            let response = ui.add(
                egui::TextEdit::singleline(&mut draft.now.status)
                    .desired_width(width)
                    .min_size(egui::vec2(0.0, Density::Read.control()))
                    .margin(egui::vec2(design::S, 5.0))
                    .hint_text(design::placeholder(ui, STATUS_HINT)),
            );
            design::named(ui, &response, "Status");
            response
        })
        .has_focus();
        let text = design::field(ui, "Text", None, None, |ui| {
            let font = style::prose_type(ui).font(ui);
            let height = (ui.available_height() - Density::Read.control() - design::L - design::S)
                .max(style::prose_type(ui).line() * 2.0);
            ui.scope(|ui| {
                ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
                // One field frame includes the editor and its standard scrollbar.
                let frame = egui::Frame::new()
                    .fill(colors().well)
                    .stroke(egui::Stroke::new(
                        design::HAIRLINE_WIDTH,
                        colors().field_border,
                    ))
                    .corner_radius(design::RADIUS_CONTROL)
                    .inner_margin(egui::Margin::same(design::S as i8))
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .id_salt(("world_edit_text_scroll", id))
                            .max_height(height - 2.0 * (design::S + design::HAIRLINE_WIDTH))
                            .auto_shrink([false, true])
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut draft.now.text)
                                        .id(egui::Id::new(("world_edit_text", id)))
                                        .font(font)
                                        .desired_width(ui.available_width())
                                        .desired_rows(8)
                                        .frame(false)
                                        .margin(egui::Vec2::ZERO),
                                )
                            })
                            .inner
                    });
                frame.response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Other,
                        ui.is_enabled(),
                        "Text field",
                    )
                });
                if frame.inner.has_focus() {
                    design::focus_ring(ui, frame.response.rect);
                }
                frame.inner
            })
            .inner
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
        // The campaign-switch reason sits between Discard and Save where it fits. In a narrow
        // form (Lookup beside a run) it takes its own line below the row, reserved while the
        // form has no changes, so the first edit moves nothing.
        let switch = crate::encounters::draft_switch_reason();
        let spacing = ui.spacing().item_spacing.x;
        let narrow = ui.available_width().min(design::FORM_WIDTH)
            < Action::primary("Save changes")
                .density(Density::Read)
                .key("Ctrl+S")
                .natural_width(ui)
                + Action::quiet("Discard").natural_width(ui)
                + design::number_width(
                    ui,
                    &format!("{} Unsaved", design::icons::DRAFT),
                    &Type::Body.font(ui),
                )
                + design::number_width(ui, switch, &Type::Small.font(ui))
                + spacing * 4.0;
        ui.horizontal(|ui| {
            ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Save is primary unless a live run owns Next turn on this screen.
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
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let state = if dirty {
                        design::Status {
                            icon: design::icons::DRAFT,
                            word: "Unsaved",
                            color: colors().document.unsaved,
                        }
                    } else {
                        design::Status {
                            icon: design::icons::OK,
                            word: "No changes",
                            color: colors().muted,
                        }
                    };
                    // A dirty draft says Discard. A clean form loses nothing, so it says Cancel.
                    // A changed form locks the campaign switcher; the save row states why, as
                    // every other draft's does.
                    cancel = if dirty {
                        let discard = design::save_row_state(ui, state, true, None);
                        if !narrow {
                            design::blocked_reason(ui, Some(switch));
                        }
                        discard
                    } else {
                        design::status(ui, state);
                        Action::quiet("Cancel").show(ui).clicked()
                    };
                });
            });
        });
        if narrow {
            if dirty {
                design::blocked_reason(ui, Some(switch));
            } else {
                ui.add_space(Type::Small.line() + ui.spacing().item_spacing.y);
            }
        }
        // The reload result sits below the save row, where nothing the DM works on follows it:
        // the first edit ends it without moving a field (round 25 results family).
        if draft.reloaded.is_some() {
            let just_reloaded = had_conflict && draft.disk.is_none() && !overwrite;
            let shown = ui.scope(|ui| {
                ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
                if design::reloaded_draft(ui)
                    && let Some((_, previous)) = draft.reloaded.take()
                {
                    draft.now = previous;
                }
            });
            if just_reloaded {
                ui.scroll_to_rect(shown.response.rect, None);
            }
        }
        if cancel {
            self.error = None;
            self.picker = None;
            self.form_focused = false;
            if draft.dirty() {
                self.discarded = Some((id, draft));
                self.stamp(super::FooterAct::Discarded);
            }
            return None;
        }
        if save {
            match draft.save(campaign, index, self.log_as().as_deref()) {
                Ok(Saved::Written(change)) => {
                    self.picker = None;
                    self.form_focused = false;
                    self.undo = Some(change);
                    self.stamp(super::FooterAct::Change);
                    self.kept = None;
                    return Some(Go::Reload(id));
                }
                Ok(Saved::Unchanged(disk)) => {
                    self.picker = None;
                    self.form_focused = false;
                    self.undo = None;
                    // A refused group change is not in the note. The note's own warning says why.
                    self.kept = (!draft.groups_refused).then(|| (draft.path.clone(), disk));
                    return Some(Go::Reload(id));
                }
                Ok(Saved::Waiting) => {
                    // The note changed on disk. Read it again for its list row and header count,
                    // as a refused Part of write does. The draft waits for the DM's choice.
                    self.edits.insert(id, draft);
                    return Some(Go::Reload(id));
                }
                Err(error) => self.error = Some(error),
            }
        }
        self.edits.insert(id, draft);
        None
    }
}

#[cfg(test)]
impl Saved {
    fn change(self) -> Change {
        match self {
            Self::Written(change) => change,
            _ => panic!("Save wrote nothing"),
        }
    }

    fn waiting(&self) -> bool {
        matches!(self, Self::Waiting)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content_workspace::EntryRef;
    use splat::content::layout::{Field, FlagKind};
    use std::fs;

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
    fn iter8_fd5_object_groups_are_rejected_by_the_note_reader() {
        let file = format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: Rook\nsplat_type: npc\nsplat_part_of:\n- '[[Zarel|the Archdevil]]'\n- {{id: {}, link: '[[Lost Guild|the lost]]'}}\n---\n",
            Uuid::new_v4(),
            Uuid::new_v4()
        );
        let entry = Entry::parse("NPCs/Rook.md", &file).unwrap();
        assert!(
            entry
                .flags
                .iter()
                .any(|flag| flag.kind == FlagKind::Unread(Field::PartOf)
                    && flag
                        .message
                        .contains("Write each group as a link, such as [[Arena Guild]].")),
            "FD-5-object-input: an object group is explicitly flagged"
        );
        assert!(
            super::super::part_of(&entry).is_empty(),
            "FD-5-object-input: rejected values never reach stored_value"
        );
    }

    #[test]
    fn iter11_d_fd2_unread_values_name_their_text_and_line() {
        let help = "Write each group as a link, such as [[Arena Guild]].";
        for (front, expected) in [
            (
                "splat_part_of: [Guild, {id: 1}] # A note.\n".to_owned(),
                Some(format!("Line 7: Part of “[Guild, {{id: 1}}]” was not read. {help}")),
            ),
            (
                "splat_part_of: 17\n".to_owned(),
                Some(format!("Line 7: Part of “17” was not read. {help}")),
            ),
            (
                "splat_part_of: # Kept.\n- '[[Zarel]]'\n- [[Verori]]\n\n- {link: '[[Lost]]'} # Old.\n"
                    .to_owned(),
                Some(format!("Line 11: Part of “{{link: '[[Lost]]'}}” was not read. {help}")),
            ),
            (
                "splat_part_of:\n- '[[Zarel]]'\n- [[Verori]]\n".to_owned(),
                None,
            ),
            // A key with no value is an empty list, not an unread value.
            ("splat_part_of:\n".to_owned(), None),
        ] {
            let file = format!(
                "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: Rook\nsplat_type: npc\n{front}splat_status: alive\n---\n",
                Uuid::new_v4()
            );
            let entry = Entry::parse("NPCs/Rook.md", &file).unwrap();
            let unread = super::super::fields::unread_groups(&entry, &file);
            assert_eq!(
                unread.as_ref().map(|unread| unread.reason.clone()),
                expected,
                "FD2-{front}"
            );
            // Neighbour: the flag and the message name the same line.
            if let Some(unread) = unread {
                assert!(unread.reason.starts_with(&format!("Line {}: ", unread.line)));
            }
        }
        // The last key's scan stops at the closing `---`: the note's text is never a value.
        for (front, expected) in [
            (
                "splat_part_of:\n  x: 1\n",
                Some(format!("Line 7: Part of “” was not read. {help}")),
            ),
            ("splat_part_of:\n", None),
            (
                "splat_in:\n  x: 1\n",
                Some("Line 7: In “” was not read. Use one place name or link.".to_owned()),
            ),
        ] {
            let file = format!(
                "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: Rook\nsplat_type: npc\n{front}---\n# Notes\n- HP: 30\n- Lair: cave\n",
                Uuid::new_v4()
            );
            let entry = Entry::parse("NPCs/Rook.md", &file).unwrap();
            let messages: Vec<_> = entry
                .flags
                .iter()
                .map(|flag| flag.message.clone())
                .collect();
            assert_eq!(
                messages,
                expected.into_iter().collect::<Vec<_>>(),
                "FD2-last-key-{front}"
            );
        }
        // Neighbour: an unread Status keeps its own words and is not a Part of fault.
        let file = format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: Rook\nsplat_type: npc\nsplat_status: [alive] # Old.\naliases:\n- Rook\n- 17\n---\n",
            Uuid::new_v4()
        );
        let entry = Entry::parse("NPCs/Rook.md", &file).unwrap();
        assert!(super::super::fields::unread_groups(&entry, &file).is_none());
        assert!(entry.flags.iter().any(|flag| flag.line == 7
            && flag.kind == FlagKind::Unread(Field::Status)
            && flag.message
                == "Line 7: Status “[alive]” was not read. Use a short word, such as alive."));
        assert!(super::super::status_unread(&entry));
        // Contract F: World finds its flags by kind. Other words on every flag change nothing; a
        // plain flag that has the old Status words is not a Status fault.
        let file = format!(
            "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: Rook\nsplat_type: npc\nsplat_part_of: 17\nsplat_status: [alive]\n---\n",
            Uuid::new_v4()
        );
        let mut entry = Entry::parse("NPCs/Rook.md", &file).unwrap();
        let unread = super::super::fields::unread_groups(&entry, &file).unwrap();
        assert_eq!(entry.flags.len(), 2);
        for flag in &mut entry.flags {
            flag.message = "Other words".into();
        }
        assert!(super::super::status_unread(&entry));
        let (notices, _) = super::super::GroupFault::flags(&entry, Some(&unread));
        assert_eq!(
            notices
                .iter()
                .map(|(shown, _)| shown.as_str())
                .collect::<Vec<_>>(),
            ["Other words"],
            "only Status; Part of is the unread list"
        );
        entry.flags = vec![splat::content::layout::Flag::new(
            8,
            "Status “[alive]” was not read.".into(),
        )];
        assert!(!super::super::status_unread(&entry));
    }

    #[test]
    fn iter8_fd5_adding_a_group_keeps_untouched_stored_values() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        super::super::create_entry(&campaign, "Zarel", "npc").unwrap();
        super::super::create_entry(&campaign, "Arena Guild", "faction").unwrap();
        let id = super::super::create_entry(&campaign, "Rook", "npc").unwrap();
        let path = dir.path().join("NPCs/Rook.md");
        let missing = Uuid::new_v4();
        let file = format!(
            "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: world\nsplat_name: Rook\nsplat_type: npc\nsplat_part_of:\n- '[[Zarel|the Archdevil]]'\n- '[[Lost Guild|the lost]] %% splat id={missing} %%'\n---\n\n# Rook\n\nKeeps the gate.\n"
        );
        fs::write(&path, &file).unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let entry = &index.items[index.position(id).unwrap()].entry;
        let before: Vec<_> = super::super::part_of(entry).into_iter().cloned().collect();
        assert_eq!(
            before.len(),
            2,
            "FD-5-fixture: both original groups must be read"
        );
        assert_eq!(before[0].as_str(), Some("[[Zarel|the Archdevil]]"));
        assert_eq!(
            before[1].as_str(),
            Some(format!("[[Lost Guild|the lost]] %% splat id={missing} %%").as_str())
        );
        let mut draft = EditDraft::open(&campaign, entry).unwrap();
        draft
            .now
            .groups
            .push(Value::String("[[Arena Guild]]".into()));
        let change = draft.save(&campaign, &index, None).unwrap().change();
        let after = fs::read_to_string(&path).unwrap();
        let saved = Entry::parse("NPCs/Rook.md", &after).unwrap();
        for (position, value) in before.iter().enumerate() {
            assert_eq!(
                serde_yaml_ng::to_string(super::super::part_of(&saved)[position])
                    .unwrap()
                    .as_bytes(),
                serde_yaml_ng::to_string(value).unwrap().as_bytes(),
                "FD-5-untouched-{position}: stored group values must be identical"
            );
        }
        assert_eq!(super::super::part_of(&saved).len(), 3);
        change.undo(&campaign).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), file);
    }

    fn unread_group_note() -> (tempfile::TempDir, Campaign, ContentIndex, Uuid, String) {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let id = super::super::create_entry(&campaign, "Rook", "npc").unwrap();
        super::super::create_entry(&campaign, "Arena Guild", "faction").unwrap();
        let file = format!(
            "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: world\nsplat_name: Rook\nsplat_type: npc\nsplat_part_of: # Keep these exact bytes.\n- '[[Zarel|the Archdevil]]'\n- {{id: {}, link: '[[Lost Guild|the lost]]'}}\nsplat_status: alive\n---\n\n# Rook\n\nKeeps the gate.\n",
            Uuid::new_v4()
        );
        fs::write(dir.path().join("NPCs/Rook.md"), &file).unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        (dir, campaign, index, id, file)
    }

    /// The Part of reason for `unread_group_note`: the object on line 9, and how to write a group.
    /// The object is longer than 40 characters, so the reason quotes its first and last 12
    /// characters around `…` (round 25 FD4).
    fn unread_reason(file: &str) -> String {
        let value: Vec<char> = format!(
            "{{id:{}",
            file.lines()
                .find_map(|line| line.strip_prefix("- {id:"))
                .unwrap()
        )
        .chars()
        .collect();
        assert!(value.len() > 40, "the fixture's value is long");
        let start: String = value[..12].iter().collect();
        let end: String = value[value.len() - 12..].iter().collect();
        format!(
            "Line 9: Part of “{start}…{end}” was not read. Write each group as a link, such as [[Arena Guild]]."
        )
    }

    #[test]
    fn iter9_d_read_and_live_changes_keep_reported_unread_disk_bytes() {
        for played in [None, Some("[[Session 1]]".to_owned())] {
            let (dir, campaign, _, id, file) = unread_group_note();
            // The picker opened on a readable file. The index is deliberately stale.
            let path = dir.path().join("NPCs/Rook.md");
            let start = file.find("- {id:").unwrap();
            let end = file[start..].find('\n').unwrap() + start;
            fs::write(&path, format!("{}{}", &file[..start], &file[end + 1..])).unwrap();
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let stale = &index.items[index.position(id).unwrap()];
            fs::write(&path, &file).unwrap();
            let mut reader = Reader {
                played,
                picker: Some(Picker {
                    key: "part_of",
                    ..Default::default()
                }),
                ..Default::default()
            };
            let result = reader.change(
                &campaign,
                stale,
                &index,
                "part_of",
                Value::Sequence(vec!["[[Arena Guild]]".into()]),
            );
            assert_eq!(
                fs::read(dir.path().join("NPCs/Rook.md")).unwrap(),
                file.as_bytes(),
                "SD1/AD1/AT1: unread disk groups must keep every raw byte"
            );
            assert!(
                matches!(result, Some(Go::Reload(reloaded)) if reloaded == id),
                "SD2/FD1: a refused write reads the note again"
            );
            assert!(reader.picker.is_none());
            assert_eq!(reader.error.as_deref(), Some(unread_reason(&file).as_str()));
        }
    }

    #[test]
    fn iter9_d_readable_group_add_keeps_raw_group_text() {
        let (dir, campaign, _, id, file) = unread_group_note();
        let start = file.find("- {id:").unwrap();
        let end = file[start..].find('\n').unwrap() + start;
        let file = format!("{}{}", &file[..start], &file[end + 1..]);
        let file = file.replace(
            "- '[[Zarel|the Archdevil]]'",
            "- \"[[Zarel|the Archdevil]]\" # Keep the alias and comment.",
        );
        fs::write(dir.path().join("NPCs/Rook.md"), &file).unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(id).unwrap()];
        let mut groups: Vec<_> = super::super::part_of(&item.entry)
            .into_iter()
            .cloned()
            .collect();
        groups.push("[[Arena Guild]]".into());
        let mut reader = Reader::default();
        assert!(
            reader
                .change(&campaign, item, &index, "part_of", Value::Sequence(groups))
                .is_some()
        );
        let saved = fs::read_to_string(dir.path().join("NPCs/Rook.md")).unwrap();
        assert!(
            saved.contains("- \"[[Zarel|the Archdevil]]\" # Keep the alias and comment.\n"),
            "SD1-raw-readable: adding a group keeps the other group's raw text"
        );
        reader.undo.as_ref().unwrap().undo(&campaign).unwrap();
        assert_eq!(
            fs::read(dir.path().join("NPCs/Rook.md")).unwrap(),
            file.as_bytes()
        );
    }

    #[test]
    fn iter9_d_readable_list_layouts_keep_raw_groups() {
        for field in [
            "splat_part_of: [\"[[Zarel|the Archdevil]]\"] # Keep the flow list.\n",
            "splat_part_of: # Keep the header.\n  [\"[[Zarel|the Archdevil]]\"] # Keep the flow list.\n",
            "splat_part_of:\n  - '[[Zarel|the Archdevil]]' # Keep the indent.\n",
            "splat_part_of:\n- Zarel # Keep the plain name.\n",
            "splat_part_of:\n- >-\n  [[Zarel|the Archdevil]]\n",
        ] {
            let (dir, campaign, _, id, file) = unread_group_note();
            let start = file.find("splat_part_of:").unwrap();
            let end = file.find("splat_status:").unwrap();
            let before = format!("{}{field}{}", &file[..start], &file[end..]);
            let path = dir.path().join("NPCs/Rook.md");
            fs::write(&path, &before).unwrap();
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(id).unwrap()];
            let mut groups: Vec<_> = super::super::part_of(&item.entry)
                .into_iter()
                .cloned()
                .collect();
            assert_eq!(groups.len(), 1, "fixture: {field}");
            groups.push("[[Arena Guild]]".into());
            let mut reader = Reader::default();
            assert!(
                reader
                    .change(&campaign, item, &index, "part_of", Value::Sequence(groups))
                    .is_some(),
                "{field}: {:?}",
                reader.error
            );
            let saved = fs::read_to_string(&path).unwrap();
            let original = if field.contains("[\"") {
                "\"[[Zarel|the Archdevil]]\""
            } else {
                field.lines().find(|line| line.contains("Zarel")).unwrap()
            };
            assert!(saved.contains(original), "SD1-raw-layout: {field}");
            if field.contains("Keep the flow list") {
                assert!(saved.contains("] # Keep the flow list."));
            }
            reader.undo.as_ref().unwrap().undo(&campaign).unwrap();
            assert_eq!(fs::read(&path).unwrap(), before.as_bytes());
        }
    }

    #[test]
    fn iter9_d_source_preparation_stays_behind_the_generation_cache() {
        let (_dir, _campaign, index, id, file) = unread_group_note();
        let item = &index.items[index.position(id).unwrap()];
        let mut reader = Reader::default();
        let mut reads = 0;
        reader.refresh_group_fault(item, 1, || {
            reads += 1;
            Some(file.clone())
        });
        assert_eq!(reads, 1);
        reader.refresh_group_fault(item, 1, || {
            panic!("unchanged render must not prepare or read the source")
        });
        assert_eq!(reads, 1);
        reader.refresh_group_fault(item, 2, || {
            reads += 1;
            Some(file.clone())
        });
        assert_eq!(
            reads, 2,
            "a new index generation invalidates the source lookup"
        );
        assert_eq!(
            reader
                .group_fault
                .as_ref()
                .unwrap()
                .unread
                .as_ref()
                .unwrap()
                .line,
            9
        );
        // Round 25 review: the reader draws the unread warning from a borrow of the cached
        // fault. The first render copies the warning's words once, when it caches the fault; an
        // unchanged second render copies none.
        let (_dir, campaign, index, id, _) = unread_group_note();
        let item = &index.items[index.position(id).unwrap()];
        let mut reader = Reader::default();
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let render = |reader: &mut Reader| {
            let mut words = String::new();
            let output = ctx.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    reader.read(ui, item, &index, Some(&campaign));
                });
            });
            for clipped in &output.shapes {
                if let egui::Shape::Text(text) = &clipped.shape {
                    words.push_str(&text.galley.job.text);
                }
            }
            words
        };
        let before = super::super::word_copies();
        let first = render(&mut reader);
        let cached = super::super::word_copies();
        assert!(
            cached > before && first.contains("was not read"),
            "the first render caches the fault and shows its warning\n{first}"
        );
        let second = render(&mut reader);
        assert!(
            second.contains("was not read"),
            "the warning still shows\n{second}"
        );
        assert_eq!(
            super::super::word_copies(),
            cached,
            "review-25-2: an unchanged render copies no warning words"
        );
    }

    #[test]
    fn iter9_d_failed_reads_and_two_faults_keep_disk_bytes() {
        for fault in ["missing", "invalid yaml", "two fields"] {
            let (dir, campaign, index, id, file) = unread_group_note();
            let item = &index.items[index.position(id).unwrap()];
            let path = dir.path().join("NPCs/Rook.md");
            match fault {
                "missing" => fs::remove_file(&path).unwrap(),
                "invalid yaml" => fs::write(&path, "---\nsplat_part_of: [\n---\n").unwrap(),
                _ => fs::write(
                    &path,
                    file.replace("splat_part_of:", "aliases: [Rook, 17]\nsplat_part_of:"),
                )
                .unwrap(),
            }
            let before = fs::read(&path).ok();
            let mut reader = Reader {
                picker: Some(Picker {
                    key: "part_of",
                    ..Default::default()
                }),
                ..Default::default()
            };
            let result = reader.change(
                &campaign,
                item,
                &index,
                "part_of",
                Value::Sequence(vec!["[[Arena Guild]]".into()]),
            );
            // Only an unread list reads the note again; a file that cannot be read stays put.
            assert_eq!(
                result.is_some(),
                fault == "two fields",
                "SD2-read-fault: {fault}"
            );
            assert!(reader.picker.is_none());
            assert!(reader.error.is_some());
            assert_eq!(fs::read(&path).ok(), before, "SD1-read-fault: {fault}");
        }
    }

    #[test]
    fn iter8_d2_unread_object_groups_cannot_be_replaced_by_edit() {
        let (dir, campaign, index, id, file) = unread_group_note();
        let item = &index.items[index.position(id).unwrap()];
        assert!(editable(item, &index));
        assert!(super::super::part_of(&item.entry).is_empty());
        let mut draft = EditDraft::open(&campaign, &item.entry).unwrap();
        draft.now.groups.push("[[Arena Guild]]".into());
        draft.now.status = "missing".into();
        let result = draft.save(&campaign, &index, None);
        let saved = fs::read_to_string(dir.path().join("NPCs/Rook.md")).unwrap();
        assert_eq!(
            saved, file,
            "D2-object-save: refused save must preserve the whole file"
        );
        assert_eq!(result.err().as_deref(), Some(unread_reason(&file).as_str()));
    }

    #[test]
    fn iter12_d_ad2_overwrite_that_only_changes_unread_groups_writes_nothing() {
        let (dir, campaign, _, id, file) = unread_group_note();
        // The form opens on a readable note. Obsidian then writes groups the app cannot read.
        let path = dir.path().join("NPCs/Rook.md");
        let start = file.find("- {id:").unwrap();
        let end = file[start..].find('\n').unwrap() + start;
        fs::write(&path, format!("{}{}", &file[..start], &file[end + 1..])).unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let entry = &index.items[index.position(id).unwrap()].entry;
        let mut draft = EditDraft::open(&campaign, entry).unwrap();
        draft.now.groups.push("[[Arena Guild]]".into());
        fs::write(&path, &file).unwrap();
        assert!(draft.save(&campaign, &index, None).unwrap().waiting());
        let (disk, theirs) = draft.disk.take().unwrap();
        draft.resolve(disk, theirs, design::DiskConflictChoice::Overwrite, &index);
        assert!(
            draft.groups_refused,
            "AD2-refused: the form knows the group change did not go in"
        );
        assert!(matches!(
            draft.save(&campaign, &index, None).unwrap(),
            Saved::Unchanged(text) if text == file
        ));
        assert_eq!(fs::read(&path).unwrap(), file.as_bytes(), "AD2-refused-raw");
    }

    #[test]
    fn iter8_d2_other_edits_keep_unread_group_bytes() {
        let (dir, campaign, index, id, file) = unread_group_note();
        let item = &index.items[index.position(id).unwrap()];
        let mut draft = EditDraft::open(&campaign, &item.entry).unwrap();
        draft.now.status = "missing".into();
        draft.now.text = "Left the gate.\n".into();
        let change = draft.save(&campaign, &index, None).unwrap().change();
        let saved = fs::read_to_string(dir.path().join("NPCs/Rook.md")).unwrap();
        let group_bytes = |text: &str| {
            text.split_once("splat_part_of:")
                .unwrap()
                .1
                .split_once("splat_status:")
                .unwrap()
                .0
                .to_owned()
        };
        assert_eq!(
            group_bytes(&saved),
            group_bytes(&file),
            "D2-other-save: unread groups must stay byte-identical"
        );
        let parsed = Entry::parse("NPCs/Rook.md", &saved).unwrap();
        assert_eq!(status(&parsed), "missing");
        assert!(saved.ends_with("Left the gate.\n"));
        change.undo(&campaign).unwrap();
        assert_eq!(
            fs::read_to_string(dir.path().join("NPCs/Rook.md")).unwrap(),
            file
        );
    }

    #[test]
    fn iter8_d2_clean_edit_notices_repair_when_another_field_is_still_unread() {
        let (dir, campaign, _, _, file) = unread_group_note();
        // Two rejected lists: repairing Part of must work while Aliases is still unread.
        let file = file.replace("splat_part_of:", "aliases: [Rook, 17]\nsplat_part_of:");
        let path = dir.path().join("NPCs/Rook.md");
        fs::write(&path, &file).unwrap();
        let entry = Entry::parse("NPCs/Rook.md", &file).unwrap();
        assert_eq!(entry.flags.len(), 2);
        let mut draft = EditDraft::open(&campaign, &entry).unwrap();
        let start = file.find("splat_part_of:").unwrap();
        let end = file.find("splat_status:").unwrap();
        let repaired_file = format!("{}{}", &file[..start], &file[end..]);
        fs::write(&path, &repaired_file).unwrap();
        let repaired = Entry::parse("NPCs/Rook.md", &repaired_file).unwrap();
        assert_eq!(repaired.flags.len(), 1);
        assert_eq!(*entry.fields, *repaired.fields);
        assert!(!draft.dirty());
        assert!(
            draft.index_moved(&repaired),
            "D2-repair-refresh: changed read flags must refresh a clean form even when parsed fields are unchanged"
        );
    }

    /// The readable note that `world-part-of-safety` reports, byte for byte.
    fn safe_rook() -> (
        tempfile::TempDir,
        Campaign,
        Uuid,
        std::path::PathBuf,
        String,
    ) {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        super::super::create_entry(&campaign, "Added Guild", "faction").unwrap();
        super::super::create_entry(&campaign, "Outside Guild", "faction").unwrap();
        let id = Uuid::new_v4();
        let readable = format!(
            "---\nsplat_schema: 2\nsplat_id: {id}\nsplat_kind: world\nsplat_name: Safe Rook\nsplat_type: npc\nsplat_part_of: # Keep the stored groups.\n- \"[[Zarel|the Archdevil]]\" # Keep the alias.\nsplat_status: alive\n---\n\n# Safe Rook\n\nKeeps the gate.\n"
        );
        let path = dir.path().join("NPCs/Safe Rook.md");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, &readable).unwrap();
        (dir, campaign, id, path, readable)
    }

    const ZAREL: &str = "- \"[[Zarel|the Archdevil]]\" # Keep the alias.\n";
    const OUTSIDE: &str = "- '[[Outside Guild]]' # Added in Obsidian.\n";

    /// The draft after Overwrite, its index and campaign, the note's path, the disk text before
    /// Overwrite, the folder that holds them, and what the comparison said Overwrite would do.
    type Overwritten = (
        EditDraft,
        ContentIndex,
        Campaign,
        std::path::PathBuf,
        String,
        tempfile::TempDir,
        Vec<(&'static str, String)>,
    );

    /// Open the form, change the draft, let Obsidian add a group, then choose Overwrite.
    fn overwrite_after_outside_group(
        change: impl FnOnce(&mut EditDraft, &ContentIndex),
    ) -> Overwritten {
        overwrite_after(
            |readable| readable.replace(ZAREL, &format!("{ZAREL}{OUTSIDE}")),
            change,
        )
    }

    /// Open the form, change the draft, let Obsidian change the file, then choose Overwrite.
    fn overwrite_after(
        outside: impl FnOnce(&str) -> String,
        change: impl FnOnce(&mut EditDraft, &ContentIndex),
    ) -> Overwritten {
        let (dir, campaign, id, path, readable) = safe_rook();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let entry = &index.items[index.position(id).unwrap()].entry;
        let mut draft = EditDraft::open(&campaign, entry).unwrap();
        change(&mut draft, &index);
        let disk = outside(&readable);
        assert_ne!(disk, readable, "fixture: Obsidian changes the note");
        fs::write(&path, &disk).unwrap();
        assert!(draft.save(&campaign, &index, None).unwrap().waiting());
        let (disk_text, theirs) = draft.disk.take().unwrap();
        assert_eq!(disk_text, disk, "fixture: the outside change is on disk");
        let effects = draft.overwrite_effects(theirs.as_ref().unwrap(), &index);
        assert!(matches!(
            draft.resolve(
                disk_text,
                theirs,
                design::DiskConflictChoice::Overwrite,
                &index
            ),
            Conflict::Overwrite
        ));
        (draft, index, campaign, path, disk, dir, effects)
    }

    #[test]
    fn iter10_d_sd1_overwrite_adds_the_draft_group_and_keeps_the_outside_group() {
        let added = std::cell::RefCell::new(Value::Null);
        let (mut draft, index, campaign, path, disk, _dir, effects) =
            overwrite_after_outside_group(|draft, index| {
                let at = index
                    .items
                    .iter()
                    .position(|item| item.entry.name == "Added Guild")
                    .unwrap();
                let link: Value = index.link_to(at).into();
                *added.borrow_mut() = link.clone();
                draft.now.groups.push(link);
            });
        assert_eq!(
            effects,
            [(
                "Part of",
                "Overwrite adds Added Guild and keeps Outside Guild from the note.".into()
            )],
            "SD1-preview-add: the choice names the group it adds and the one it keeps"
        );
        let change = draft.save(&campaign, &index, None).unwrap().change();
        let stored = stored_value(&index, added.into_inner());
        let added_line = format!(
            "- {}\n",
            serde_yaml_ng::to_string(&stored).unwrap().trim_end()
        );
        let expected = disk.replace(OUTSIDE, &format!("{OUTSIDE}{added_line}"));
        assert_eq!(
            fs::read(&path).unwrap(),
            expected.as_bytes(),
            "SD1-add: Overwrite saves A, B and C, with A and C as raw text"
        );
        change.undo(&campaign).unwrap();
        assert_eq!(fs::read(&path).unwrap(), disk.as_bytes());
    }

    #[test]
    fn iter10_d_sd1_overwrite_removes_only_the_draft_group() {
        let (mut draft, index, campaign, path, disk, _dir, effects) =
            overwrite_after_outside_group(|draft, _| {
                draft.now.groups.clear();
            });
        assert_eq!(
            effects,
            [(
                "Part of",
                "Overwrite removes the Archdevil and keeps Outside Guild from the note.".into()
            )],
            "SD1-preview-remove: the choice names the group it removes and the one it keeps"
        );
        // Round 24 FD8: Status words are the DM's own text, so each has curly marks; `not set`
        // and link names have none.
        let mut theirs = draft.now.clone();
        theirs.status = "alive".into();
        let mut probe = draft.now.clone();
        probe.status = "ready".into();
        let now = std::mem::replace(&mut draft.now, probe);
        let start_status = std::mem::take(&mut draft.start.status);
        let status = |effects: Vec<(&str, String)>| {
            effects
                .into_iter()
                .find(|(label, _)| *label == "Status")
                .map(|(_, words)| words)
        };
        assert_eq!(
            status(draft.overwrite_effects(&theirs, &index)).as_deref(),
            Some("Overwrite saves “ready” in place of “alive”."),
            "iter-24-FD8-changed"
        );
        draft.start.status = "ready".into();
        draft.now.status = "ready".into();
        theirs.status = " ".into();
        assert_eq!(
            status(draft.overwrite_effects(&theirs, &index)).as_deref(),
            Some("Overwrite keeps not set from the note."),
            "iter-24-FD8-not-set"
        );
        draft.now = now;
        draft.start.status = start_status;
        draft.save(&campaign, &index, None).unwrap().change();
        assert_eq!(
            fs::read(&path).unwrap(),
            disk.replace(ZAREL, "").as_bytes(),
            "SD1-remove: Overwrite saves C alone, as raw text"
        );
    }

    const RESPELLED: &str = "- \"[[Zarel]]\" # Respelled in Obsidian.\n";
    const OTHER_SPELLING: &str = "- \"[[Factions/Added Guild|Guild]]\" # Added in Obsidian.\n";

    #[test]
    fn iter11_d_sd1_overwrite_removes_a_group_obsidian_respelled() {
        let (mut draft, index, campaign, path, disk, _dir, effects) = overwrite_after(
            |readable| readable.replace(ZAREL, RESPELLED),
            |draft, _| draft.now.groups.clear(),
        );
        assert_eq!(
            effects,
            [("Part of", "Overwrite removes Zarel.".into())],
            "SD1-preview-respelled: the choice names the respelled group it removes"
        );
        let change = draft.save(&campaign, &index, None).unwrap().change();
        let saved = fs::read_to_string(&path).unwrap();
        let entry = Entry::parse("NPCs/Safe Rook.md", &saved).unwrap();
        assert!(
            super::super::part_of(&entry).is_empty() && !saved.contains("Zarel"),
            "SD1-respelled: Overwrite removes the group whatever its spelling: {saved}"
        );
        // Neighbour: every other byte of the note stays as Obsidian left it.
        let part_of = "splat_part_of: # Keep the stored groups.\n";
        assert_eq!(
            saved,
            disk.replace(&format!("{part_of}{RESPELLED}"), ""),
            "SD1-respelled-raw"
        );
        change.undo(&campaign).unwrap();
        assert_eq!(fs::read(&path).unwrap(), disk.as_bytes());
    }

    #[test]
    fn iter11_d_sd1_overwrite_adds_a_group_once_when_obsidian_added_it_too() {
        let (mut draft, index, campaign, path, disk, _dir, effects) = overwrite_after(
            |readable| readable.replace(ZAREL, &format!("{ZAREL}{OTHER_SPELLING}")),
            |draft, _| draft.now.groups.push("[[Added Guild]]".into()),
        );
        assert!(
            effects.iter().all(|(_, words)| !words.contains("adds")),
            "SD1-preview-same: the choice does not add a group the note has: {effects:?}"
        );
        // An old time on the file shows whether Save wrote it, even with the same bytes.
        let old = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_000_000);
        fs::File::options()
            .write(true)
            .open(&path)
            .unwrap()
            .set_modified(old)
            .unwrap();
        let saved = draft.save(&campaign, &index, None).unwrap();
        assert!(
            matches!(&saved, Saved::Unchanged(text) if *text == disk),
            "AD2-no-undo: nothing written, no Change"
        );
        assert_eq!(
            fs::metadata(&path).unwrap().modified().unwrap(),
            old,
            "AD2-no-write: the note is not written again"
        );
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            disk,
            "SD1-same-raw: one line for the group, in the note's own spelling"
        );
    }

    #[test]
    fn iter11_d_sd1_groups_compare_by_target() {
        let (_dir, campaign, _, _, _) = safe_rook();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let value = |text: &str| Value::String(text.into());
        let same = |a: &str, b: &str| super::super::has_group(&index, &[value(a)], &value(b));
        // Found notes: the target decides, whatever the folder, alias or case.
        assert!(same("[[Added Guild]]", "[[Factions/Added Guild|Guild]]"));
        assert!(same("[[added guild]]", "[[Factions/Added Guild.md]]"));
        assert!(!same("[[Added Guild]]", "[[Outside Guild]]"));
        // Missing notes: the link target, without alias or heading.
        assert!(same("[[Zarel|the Archdevil]]", "[[Zarel]]"));
        assert!(same("[[Zarel#Court]]", "[[zarel]]"));
        assert!(!same("[[Zarel]]", "[[Devils/Zarel]]"));
        // Plain text compares as written.
        assert!(same("Night Market", "Night Market"));
        assert!(!same("Night Market", "night market"));
        assert!(!same("Zarel", "[[Zarel]]"));
    }

    #[test]
    fn lead_review_a_discarded_draft_belongs_to_its_campaign() {
        // Lookup keeps its reader across a campaign change. A draft let go in one campaign
        // offers its Undo discard (and its lost-note warning) only there.
        let campaign = |dir: &std::path::Path| Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let (home_dir, other_dir) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let (home, other) = (campaign(home_dir.path()), campaign(other_dir.path()));
        let id = super::super::create_entry(&home, "Rook", "npc").unwrap();
        let index = ContentIndex::load(&[], Some(&home)).unwrap();
        let entry = &index.items[index.position(id).unwrap()].entry;
        let mut draft = EditDraft::open(&home, entry).unwrap();
        draft.now.status = "gone".into();
        let mut reader = Reader {
            discarded: Some((id, draft)),
            ..Default::default()
        };
        // The note goes in its own campaign: the Undo is offered there.
        fs::remove_file(home_dir.path().join(&entry.path)).unwrap();
        let home_index = ContentIndex::load(&[], Some(&home)).unwrap();
        // Its `Undo discard` is a footer line of the pane, by the draft's own name (round 27 FC1).
        let lines = reader.lost_lines(&home_index);
        assert!(
            lines.len() == 1
                && lines[0].words == "Rook draft discarded."
                && lines[0].rank == super::super::FooterRank::Undo,
            "owner-home"
        );
        // Another campaign has no such note either, but the draft is not its own.
        let other_index = ContentIndex::load(&[], Some(&other)).unwrap();
        assert!(reader.lost_lines(&other_index).is_empty(), "owner-other");
        reader.discarded = None;
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
        draft
            .now
            .groups
            .push(Value::String("[[Watch House]]".into()));
        assert!(
            draft.dirty(),
            "FD6-group-dirty: a group-only edit is unsaved"
        );
        draft.now.groups.clear();
        assert!(!draft.dirty());
        assert!(
            draft.now.text.starts_with("- **Look:** A scar"),
            "{}",
            draft.now.text
        );
        draft.now.status = " missing ".into();
        draft.now.place = serde_yaml_ng::to_value(EntryRef::id(place)).ok();
        draft.now.groups = vec![Value::String("[[Watch House]]".into())];
        draft.now.text.push_str("\nOwes Nell a favour.\n");
        assert!(draft.dirty());
        let change = draft.save(&campaign, &index, None).unwrap().change();
        let after = fs::read_to_string(&path).unwrap();
        let saved = Entry::parse("NPCs/Rook.md", &after).unwrap();
        assert_eq!(saved.id, id);
        assert_eq!(status(&saved), "missing");
        assert_eq!(
            world_field(&saved, "in").and_then(Value::as_str),
            Some("[[Locations/Watch House|Watch House]]")
        );
        assert_eq!(
            super::super::part_of(&saved),
            vec![&Value::String("[[Watch House]]".into())]
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
        assert!(draft.save(&campaign, &index, None).unwrap().waiting());
        assert!(draft.disk.is_some());
        let invalid = "---\nsplat_schema: 2\nsplat_id: [\n---\n";
        fs::write(&path, invalid).unwrap();
        draft.now.groups.clear();
        assert!(draft.save(&campaign, &index, None).unwrap().waiting());
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            invalid,
            "FD6-two-faults: an unreadable disk file and group draft do not write"
        );
        fs::write(&path, &after).unwrap();
        change.undo(&campaign).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), before);
    }
}
