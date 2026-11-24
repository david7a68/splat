//! The reader as plain data and acts, for a view that is not egui (the GPUI port,
//! `docs/gpui-port.md`). The egui reader calls the acts here too (`Reader::rename_note`,
//! `rename_file`, `separate`, `check_disk`, `title_facts`), so both views are one reader: each
//! act writes, records its result and says what the page does next in one place.
//!
//! Nothing here draws. A view reads the facts (`TitleFacts`, `BodyFacts`, `ValueRow`, …), draws
//! them its own way, and calls the act a press asks for. An act returns the `Nav` the page takes,
//! as the egui reader's `read_in_pane` does.
use super::*;
use crate::design::outcome::{Messages, plain};

/// The title line: the title, its kind line, and which actions follow it.
pub struct TitleFacts {
    pub title: String,
    pub kind_line: String,
    /// `Edit`, and `Add to Log`: the note is editable here and no form is open.
    pub edit: bool,
    /// `Start fight`, with its fight; `start_blocked` says why it waits.
    pub start: Option<StartFight>,
    /// `Add to fight` while a run is open.
    pub add: Option<StartFight>,
}

/// A press on the title line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TitleAct {
    Edit,
    StartFight,
    AddToFight,
    AddToLog,
}

/// The note's file line: its path and whether `Edit in Obsidian` may open it.
pub struct FileLineFacts {
    pub path: String,
    pub editor: bool,
    /// Another note has its name: the line names the file whole.
    pub shared_name: bool,
}

/// What the read view draws between the title and the values, and how the body reads.
pub struct BodyFacts {
    pub file_line: Option<FileLineFacts>,
    /// The Part of list that could not be read: the reason cut and whole, and whether the file
    /// line above has the editor link (else the warning has its own).
    pub unread: Option<(String, String)>,
    /// Every other value not read: the words cut and whole.
    pub notices: Vec<(String, String)>,
    /// The file's name is not the note's: the file's stem, and the file that takes the note's
    /// name when it can.
    pub rename: Option<(String, Option<String>)>,
    /// Two files claim this note: each file with its repair's words and help, first the one to
    /// keep.
    pub duplicates: Vec<(String, String, Option<String>)>,
    /// The index's warnings on this note.
    pub warnings: Vec<String>,
    /// The note is a World entry with editable values (Stat block, Place, Part of, Status).
    pub values: bool,
    /// The note is a duplicate: its values read only.
    pub duplicate_values: bool,
    /// The note has prose to read.
    pub has_prose: bool,
    /// A plain World note: `Give this note a type`.
    pub make_type: bool,
    /// The reader shows the body as a plain note (Markdown and related), else as an entry.
    pub plain_note: bool,
}

/// What a value row shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueShown {
    /// A link to the note at this place in the index.
    Link(usize, String),
    /// A link that matches several notes: its warning words, which open the chooser.
    PlaceChoices(String),
    /// A link to no note: its words, with ` · Not found`.
    Missing(String),
    NotSet,
    NotRead,
    /// A status's words.
    Words(String),
    /// Part of: each group, as a link or a missing name.
    Groups(Vec<(String, Option<usize>)>),
}

/// One labelled value of an entry, with its Change.
pub struct ValueRow {
    pub key: &'static str,
    pub label: &'static str,
    pub value: ValueShown,
    /// Why Change waits, while the value is not read.
    pub blocked: Option<String>,
}

/// The open picker of a value.
pub struct PickerView {
    pub key: &'static str,
    /// The search field's name and placeholder (`Find a location`).
    pub label: &'static str,
    pub search: String,
    pub new_location: String,
    /// The field asks for the keyboard once.
    pub focus: bool,
}

/// A press in an open picker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PickAct {
    /// A match, by its place in the index.
    Match(usize),
    /// `Remove <group>` of Part of, by its place in the list.
    Remove(usize),
    /// `Clear value`.
    Clear,
    Cancel,
    /// `Create location here`.
    CreateLocation,
}

/// An open `Add to Log` field.
pub struct LogFieldView {
    /// `[[Session 1]], Day 2:` in the session's name, or none.
    pub stamp: Option<String>,
    pub words: String,
    /// The field's accessible name: `Log line for <name>`.
    pub name: String,
    /// The words came from a done scene: the field selects them once.
    pub select: bool,
}

/// The join form of an NPC with no stat block, while a run is open.
pub struct JoinView {
    pub title: String,
    /// The stat block chosen, by its place and name.
    pub stat_block: Option<(usize, String)>,
    pub search: String,
    pub matches: Vec<(usize, String)>,
    /// `Save this as their stat block`, when the form may save it.
    pub save: Option<bool>,
    pub sides: Vec<String>,
    pub side: String,
    pub initiative: String,
}

/// A press in the join form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JoinAct {
    Search(String),
    Choose(usize),
    ChangeStatBlock,
    Save(bool),
    Side(String),
    Initiative(String),
    Add,
    Cancel,
}

/// What a footer press asks of the view beside the reader's own state: words to copy, a link to
/// open.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Effect {
    Copy(String),
    Open(String),
}

/// The file line's words for a note at `path`: the folder, then the file.
pub fn obsidian_link(path: &Path) -> String {
    obsidian_url(path)
}

/// The words of a place link that matches `count` notes.
pub fn place_choices_text(name: &str, count: usize) -> String {
    place_choices_words(name, count)
}

/// A missing link's words: the name and ` · Not found`.
pub fn missing_text(name: &str) -> String {
    missing_name(name)
}

impl Reader {
    /// The entry on show.
    pub fn showing(&self) -> Option<Uuid> {
        self.showing
    }

    /// A newly opened note starts with its own state (`prepare`).
    pub fn open_item(&mut self, item: &Item) {
        self.prepare(item);
    }

    /// The disk is read again only at a new load of the index: a change's result ends when its
    /// file no longer holds what the change wrote, and so does the kept form's notice.
    pub fn check_disk(&mut self, index: &ContentIndex, campaign: Option<&Campaign>) {
        if self.disk_checked == Some(index.generation()) {
            return;
        }
        self.disk_checked = Some(index.generation());
        #[cfg(any(test, feature = "test-support"))]
        let read = |path: std::path::PathBuf| {
            READER_FILE_READS.with(|reads| reads.set(reads.get() + 1));
            fs::read_to_string(path)
        };
        #[cfg(not(any(test, feature = "test-support")))]
        let read = fs::read_to_string::<std::path::PathBuf>;
        if let Some(ReaderResult::Change(change)) = self.result(Slot::Change)
            && campaign.is_none_or(|campaign| {
                read(campaign.path.join(&change.path)).ok().as_deref() != Some(&change.after)
            })
        {
            self.end(Slot::Change);
        }
        if self.kept.as_ref().is_some_and(|(path, text)| {
            campaign
                .is_none_or(|campaign| read(campaign.path.join(path)).ok().as_deref() != Some(text))
        }) {
            self.kept = None;
        }
    }

    /// The heading a link or a check asked to show, once, for the entry `id`.
    pub fn take_heading_for(&mut self, id: Uuid) -> Option<String> {
        if self.heading_document.is_none_or(|document| document == id) {
            self.heading_document = None;
            self.heading.take()
        } else {
            None
        }
    }

    /// The title line's words and actions.
    pub fn title_facts(
        &self,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
    ) -> TitleFacts {
        let id = item.entry.id;
        let title = item
            .entry
            .reading()
            .ok()
            .flatten()
            .map_or_else(|| index.name_of(item).to_owned(), |reading| reading.title);
        let editing = campaign.is_some_and(|campaign| self.editing(id, campaign));
        let edit = campaign.is_some() && edit::editable(item, index) && !editing;
        let duplicate = index.duplicate_world.contains_key(&item.entry.path);
        let kind = ReaderStatus::of(item, index, campaign.is_some(), self.status_editing)
            .kind_words(item, duplicate);
        let kind_line = if self.compact && campaign.is_some() && duplicate {
            format!("{kind} · {} · {}", item.owner, item.entry.path)
        } else {
            format!("{kind} · {}", item.owner)
        };
        // While a run is open, `Add to fight` takes the place of `Start fight`.
        let start = (self.sides.is_none() && !self.beside_editor && !editing)
            .then(|| start_fight(item, index))
            .flatten();
        let add = self
            .sides
            .is_some()
            .then(|| start_fight(item, index))
            .flatten()
            .filter(|fight| {
                let form_open = fight.creature.is_none() && self.adding.is_some();
                let elsewhere = self.in_run.contains_key(&id) || editing || form_open;
                !fight.npc || !elsewhere
            });
        TitleFacts {
            title,
            kind_line,
            edit,
            start,
            add,
        }
    }

    /// A press on the title line.
    pub fn press_title(
        &mut self,
        act: TitleAct,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
    ) {
        let facts = self.title_facts(item, index, campaign);
        let id = item.entry.id;
        match act {
            TitleAct::Edit => {
                if let Some(campaign) = campaign.filter(|_| facts.edit) {
                    self.begin_edit(campaign, item);
                }
            }
            TitleAct::StartFight => {
                if self.start_blocked.is_none() {
                    self.start_fight = facts.start;
                }
            }
            TitleAct::AddToFight => {
                if let Some(add) = facts.add {
                    match add.creature {
                        Some(stat_block) => {
                            self.add_to_fight = Some(AddToFight {
                                entry: id,
                                stat_block,
                                npc: add.npc,
                            });
                        }
                        None => self.adding = Some(Adding::new(item, index, self.sides.as_deref())),
                    }
                }
            }
            TitleAct::AddToLog => {
                if let Some(campaign) = campaign.filter(|_| facts.edit) {
                    self.open_log(campaign.id, id, &item.entry.name, "");
                }
            }
        }
    }

    /// What the read view draws between the title and the values. It reads the note's file
    /// only at a new load of the index (`refresh_group_fault`).
    pub fn body_facts(
        &mut self,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
    ) -> BodyFacts {
        let id = item.entry.id;
        let editing = campaign.is_some_and(|campaign| self.editing(id, campaign));
        if !editing && let Some(campaign) = campaign {
            self.refresh_group_fault(item, index.generation(), || {
                note_path(&campaign.path, &item.entry.path)
                    .ok()
                    .and_then(|path| fs::read_to_string(path).ok())
            });
        }
        let file_line = (!self.compact
            && (item.is_world_note() || item.entry.kind == "world")
            && item.is_private()
            && campaign.is_some())
        .then(|| FileLineFacts {
            path: item.entry.path.clone(),
            editor: !(self.group_error && self.group_error_unread_file),
            shared_name: index
                .position(item.entry.id)
                .is_some_and(|at| index.shares_name(at)),
        });
        let fault = self.group_fault.as_ref().filter(|fault| fault.id == id);
        let unread = fault
            .filter(|_| !editing)
            .and_then(|fault| fault.unread.as_ref())
            .filter(|_| campaign.is_some() && (self.error.is_none() || self.group_error))
            .map(|unread| (unread.reason.clone(), unread.whole.clone()));
        let notices = fault.map(|fault| fault.notices.clone()).unwrap_or_default();
        let rename = campaign.and_then(|campaign| {
            let stem = renamed_file(item, index)?;
            let file = renamed_path(&item.entry.path, &item.entry.name)
                .filter(|_| !self.editing(id, campaign));
            Some((stem, file))
        });
        let duplicates = match (campaign, index.duplicate_world.get(&item.entry.path)) {
            (Some(campaign), Some(paths)) => {
                let mut paths = paths.clone();
                let first = crate::content_workspace::duplicate_repair_path(
                    paths.iter().map(|path| campaign.path.join(path)),
                );
                paths.sort_by_key(|path| Some(campaign.path.join(path)) != first);
                paths
                    .iter()
                    .enumerate()
                    .map(|(at, path)| {
                        let filename = Path::new(path)
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned();
                        (
                            path.clone(),
                            separate_choice(IdentityNoun::Note, &filename).label(),
                            links_consequence(&paths, at).map(|help| help.as_str().to_owned()),
                        )
                    })
                    .collect()
            }
            _ => Vec::new(),
        };
        let warnings = index
            .errors
            .iter()
            .filter(|error| {
                error.starts_with(&format!("{}:", item.entry.path))
                    || error.starts_with(&format!("{} is in ", item.entry.name))
            })
            .cloned()
            .collect();
        let duplicate = index.duplicate_world.contains_key(&item.entry.path);
        let has_prose = self.has_prose(item, index);
        BodyFacts {
            file_line,
            unread,
            notices,
            rename,
            duplicates,
            warnings,
            values: item.entry.kind == "world" && !duplicate,
            duplicate_values: item.entry.kind == "world" && duplicate,
            has_prose,
            make_type: !self.compact && item.is_world_note() && campaign.is_some(),
            plain_note: item.is_note(),
        }
    }

    /// Whether the entry has prose, parsed once per entry and load of the index.
    pub fn has_prose(&mut self, item: &Item, index: &ContentIndex) -> bool {
        match self.prose {
            Some((id, load, prose)) if id == item.entry.id && load == index.generation() => prose,
            _ => {
                let prose = !crate::library::presentation::reading_sections(
                    &item.entry.body,
                    &item.entry.name,
                )
                .is_empty();
                self.prose = Some((item.entry.id, index.generation(), prose));
                prose
            }
        }
    }

    /// `Rename the note to <stem>`: the note takes its file's name, in one write with an Undo.
    pub fn rename_note(&mut self, campaign: &Campaign, item: &Item, stem: &str) -> Option<Nav> {
        let go = self.note_renamed(campaign, item, stem)?;
        self.settle(go)
    }

    /// `rename_note`, as the egui reader's body takes it.
    pub(super) fn note_renamed(
        &mut self,
        campaign: &Campaign,
        item: &Item,
        stem: &str,
    ) -> Option<Go> {
        let result = (|| {
            let before = file_fault::read_note(&campaign.path, &item.entry.path)?;
            let after = crate::content_workspace::patch_fields(
                &before,
                &[("splat_name", Some(stem.to_owned().into()))],
            )?;
            let after =
                crate::content_workspace::rename_first_heading(after, &item.entry.name, stem);
            write_note(&campaign.path, &item.entry.path, Some(&before), &after)?;
            Ok::<_, String>(Change {
                campaign: campaign.id,
                path: item.entry.path.clone(),
                before,
                after,
                word: format!("Name changed to {stem}."),
                kind: ChangeKind::Name,
            })
        })();
        match result {
            Ok(change) => {
                self.held.results.record(ReaderResult::Name(change));
                Some(Go::Reload(item.entry.id))
            }
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    /// `Rename the file to <name>.md`: the file takes the note's name, when the whole file is
    /// still the one the index read.
    pub fn rename_file(&mut self, campaign: &Campaign, item: &Item, to: &str) -> Option<Nav> {
        let go = self.file_renamed(campaign, item, to)?;
        self.settle(go)
    }

    /// `rename_file`, as the egui reader's body takes it.
    pub(super) fn file_renamed(
        &mut self,
        campaign: &Campaign,
        item: &Item,
        to: &str,
    ) -> Option<Go> {
        let result = file_fault::read_note(&campaign.path, &item.entry.path).and_then(|text| {
            if item.source_text() == Some(crate::lookup::text_hash(&text)) {
                FileRename::apply(campaign, &item.entry.path, to, &text)
            } else {
                Err(format!(
                    "{} changed on disk. Nothing was renamed.",
                    item.entry.path
                ))
            }
        });
        match result {
            Ok(rename) => {
                self.held.results.record(ReaderResult::FileRename(rename));
                Some(Go::Reload(item.entry.id))
            }
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    /// A duplicate's repair: the file at `path` gets a new identity of its own.
    pub fn separate(&mut self, campaign: &Campaign, path: &str) -> Option<Nav> {
        let go = self.separated(campaign, path)?;
        self.settle(go)
    }

    /// `separate`, as the egui reader's body takes it.
    pub(super) fn separated(&mut self, campaign: &Campaign, path: &str) -> Option<Go> {
        match new_identity(campaign, path) {
            Ok((undo, id)) => {
                self.held
                    .results
                    .record(ReaderResult::Identity(campaign.id, undo));
                Some(Go::Reload(id))
            }
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    /// A press on a note link in the body: the note it names, a choice of several, or a note
    /// that does not exist.
    pub fn follow(&mut self, item: &Item, index: &ContentIndex, target: &str) -> Option<Nav> {
        let followed = index.follow(target, item);
        if let Go::Many(positions) = &followed {
            self.choose_link(positions.clone(), target);
        }
        self.settle(followed)
    }

    /// A press on a link to the note at `at`.
    pub fn go_to(&mut self, at: usize) -> Option<Nav> {
        self.settle(Go::Item(at, None))
    }

    /// The notes a link to several files waits on, with the words above them.
    pub fn link_choices(&self) -> &[usize] {
        &self.many
    }

    /// The DM chose one of a link's several notes.
    pub fn choose_many(&mut self, index: &ContentIndex, position: usize) -> Option<Nav> {
        let heading = self.many_heading.take();
        self.heading_document = Some(index.items[position].entry.id);
        self.settle(Go::Item(position, heading))
    }

    /// The value rows of a campaign entry (Stat block, Place, Part of), in the read order.
    /// World's reader and Lookup's show the same rows.
    pub fn value_rows(&self, item: &Item, index: &ContentIndex) -> Vec<ValueRow> {
        let mut rows = Vec::new();
        for (key, label) in [("creature", "Stat block"), ("in", "Place")] {
            if key == "creature" && !fights(&item.entry) {
                continue;
            }
            if key == "creature"
                && reader_field(&item.entry, key).is_none()
                && self.adding.is_some()
            {
                continue;
            }
            let target = if key == "in" {
                index.place_of(item.entry.id)
            } else {
                index.world_target(&item.entry, "creature")
            };
            if key != "creature" && reader_field(&item.entry, key).is_none_or(Value::is_null) {
                continue;
            }
            let value = if let Some(at) = target {
                ValueShown::Link(at, index.items[at].entry.name.clone())
            } else if key == "in" && !index.place_choices(item.entry.id).is_empty() {
                let name = reader_field(&item.entry, key)
                    .map(|value| link_name(value, label))
                    .unwrap_or_default();
                ValueShown::PlaceChoices(place_choices_words(
                    &name,
                    index.place_choices(item.entry.id).len(),
                ))
            } else if let Some(value) = reader_field(&item.entry, key) {
                ValueShown::Missing(link_name(value, label))
            } else {
                ValueShown::NotSet
            };
            rows.push(ValueRow {
                key,
                label,
                value,
                blocked: None,
            });
        }
        let groups = index.groups_of(&item.entry);
        let unread = self.unread_groups(item.entry.id);
        if !groups.is_empty() || unread.is_some() {
            let value = if unread.is_some() {
                ValueShown::NotRead
            } else {
                ValueShown::Groups(
                    groups
                        .iter()
                        .map(|(name, target)| match target {
                            Some(at) => (index.items[*at].entry.name.clone(), Some(*at)),
                            None => (name.clone(), None),
                        })
                        .collect(),
                )
            };
            rows.push(ValueRow {
                key: "part_of",
                label: "Part of",
                value,
                blocked: unread,
            });
        }
        if !self.status_editing && ReaderStatus::of(item, index, true, false) == ReaderStatus::Row {
            let value = status(&item.entry);
            rows.push(ValueRow {
                key: "status",
                label: "Status",
                value: if status_unread(&item.entry) {
                    ValueShown::NotRead
                } else if value.is_empty() {
                    ValueShown::NotSet
                } else {
                    ValueShown::Words(value.to_owned())
                },
                blocked: None,
            });
        }
        rows
    }

    /// A duplicate note's values, read only: label, name and the note it opens.
    pub fn duplicate_rows(
        item: &Item,
        index: &ContentIndex,
    ) -> Vec<(&'static str, String, Option<usize>)> {
        [
            ("creature", "Stat block"),
            ("in", "Place"),
            ("status", "Status"),
        ]
        .into_iter()
        .filter_map(|(key, label)| {
            let value = world_field(&item.entry, key).filter(|value| !empty_optional(value))?;
            if key == "status" {
                return Some((label, value.as_str()?.trim().to_owned(), None));
            }
            let target = edit::value_target(index, value);
            let name = target.map_or_else(
                || link_name(value, label),
                |at| index.items[at].entry.name.clone(),
            );
            Some((label, name, target))
        })
        .collect()
    }

    /// `Change` on a value row: its picker opens, or the Status field with its words.
    pub fn press_change(&mut self, key: &'static str, item: &Item) {
        if key == "status" {
            status(&item.entry).clone_into(&mut self.status);
            self.status_editing = true;
            self.status_select = true;
        } else {
            self.picker = Some(Picker {
                key,
                focus: true,
                ..Default::default()
            });
        }
    }

    /// Whether the place link's chooser is open.
    pub fn place_choices_open(&self) -> bool {
        self.place_choices_open
    }

    /// The place link's warning: the chooser opens or closes.
    pub fn toggle_place_choices(&mut self) {
        self.place_choices_open = !self.place_choices_open;
    }

    /// `Always use this one` in the place chooser.
    pub fn choose_place(
        &mut self,
        campaign: &Campaign,
        item: &Item,
        index: &ContentIndex,
        at: usize,
    ) -> Option<Nav> {
        let path = index.items[at].entry.path.trim_end_matches(".md");
        let go = self.change(campaign, item, index, "in", format!("[[{path}]]").into())?;
        self.settle(go)
    }

    /// The open picker.
    pub fn picker_view(&self) -> Option<PickerView> {
        let picker = self.picker.as_ref()?;
        Some(PickerView {
            key: picker.key,
            label: match picker.key {
                "in" => "Find a location",
                "part_of" => "Find a group",
                _ => "Find a stat block",
            },
            search: picker.search.clone(),
            new_location: picker.new_location.clone(),
            focus: picker.focus,
        })
    }

    /// The picker's field took the keyboard.
    pub fn picker_focused(&mut self) {
        if let Some(picker) = &mut self.picker {
            picker.focus = false;
        }
    }

    pub fn set_picker_search(&mut self, words: &str) {
        if let Some(picker) = &mut self.picker {
            words.clone_into(&mut picker.search);
        }
    }

    pub fn set_new_location(&mut self, words: &str) {
        if let Some(picker) = &mut self.picker {
            words.clone_into(&mut picker.new_location);
        }
    }

    /// The groups the picker starts from: the draft's, or the note's.
    fn picker_groups(item: &Item, draft_groups: Option<&[Value]>) -> Vec<Value> {
        draft_groups.map_or_else(
            || part_of(&item.entry).into_iter().cloned().collect(),
            <[Value]>::to_vec,
        )
    }

    /// The picker's matches (at most five), by place and the name they show, and Part of's
    /// `Remove <group>` names.
    pub fn picker_matches(
        &self,
        item: &Item,
        index: &ContentIndex,
        draft_groups: Option<&[Value]>,
    ) -> (Vec<(usize, String)>, Vec<String>) {
        let Some(picker) = &self.picker else {
            return (Vec::new(), Vec::new());
        };
        let key = picker.key;
        let groups = Self::picker_groups(item, draft_groups);
        let joined: Vec<usize> = groups
            .iter()
            .filter_map(|value| index.world_value_target(value))
            .collect();
        let matches = if key == "creature" {
            index.creatures_matching(&picker.search, 5)
        } else {
            let wanted = picker.search.to_lowercase();
            let mut found: Vec<(bool, usize)> = index
                .items
                .iter()
                .enumerate()
                .filter(|(at, i)| {
                    i.is_private()
                        && i.entry.kind == "world"
                        && i.entry.id != item.entry.id
                        && !(key == "part_of" && joined.contains(at))
                        && i.entry.name.to_lowercase().contains(&wanted)
                })
                .map(|(at, i)| (key == "in" && world_type(&i.entry) != "location", at))
                .collect();
            found.sort_by_key(|&(later, _)| later);
            found.into_iter().map(|(_, at)| at).take(5).collect()
        };
        let removes = if key == "part_of" {
            groups
                .iter()
                .map(|group| format!("Remove {}", link_name(group, "group")))
                .collect()
        } else {
            Vec::new()
        };
        (
            matches
                .into_iter()
                .map(|at| (at, index.shown_name(at)))
                .collect(),
            removes,
        )
    }

    /// The value a picker press chooses (`Value::Null` clears it), or none; the picker closes
    /// on a choice or Cancel.
    pub fn pick_value(
        &mut self,
        act: PickAct,
        item: &Item,
        index: &ContentIndex,
        campaign: &Campaign,
        draft_groups: Option<&[Value]>,
    ) -> Option<Value> {
        let picker = self.picker.as_ref()?;
        let key = picker.key;
        let groups = Self::picker_groups(item, draft_groups);
        let chosen = match act {
            PickAct::Match(at) => {
                if key == "part_of" {
                    let mut groups = groups;
                    groups.push(index.link_to(at).into());
                    Some(Value::Sequence(groups))
                } else {
                    serde_yaml_ng::to_value(EntryRef::id(index.items[at].entry.id)).ok()
                }
            }
            PickAct::Remove(at) if at < groups.len() => {
                let mut groups = groups;
                groups.remove(at);
                Some(Value::Sequence(groups))
            }
            PickAct::Remove(_) => None,
            PickAct::Clear => Some(Value::Null),
            PickAct::Cancel => None,
            PickAct::CreateLocation => {
                let name = picker.new_location.trim().to_owned();
                if name.is_empty() {
                    return None;
                }
                match create_entry(campaign, &name, "location") {
                    Ok(id) => {
                        let mut target = EntryRef::id(id);
                        target.link = Some(name);
                        serde_yaml_ng::to_value(target).ok()
                    }
                    Err(error) => {
                        self.error = Some(error);
                        None
                    }
                }
            }
        };
        if chosen.is_some() || act == PickAct::Cancel {
            self.picker = None;
        }
        chosen
    }

    /// A picker press in the read view: the value it chooses is written at once, with its Log
    /// line and one Undo.
    pub fn pick_and_change(
        &mut self,
        act: PickAct,
        item: &Item,
        index: &ContentIndex,
        campaign: &Campaign,
    ) -> Option<Nav> {
        let key = self.picker.as_ref()?.key;
        let value = self.pick_value(act, item, index, campaign, None)?;
        let go = self.change(campaign, item, index, key, value)?;
        self.settle(go)
    }

    /// The open Status field's words, while it is open.
    pub fn status_field(&self) -> Option<&str> {
        self.status_editing.then_some(self.status.as_str())
    }

    /// The Status field asks for the keyboard with its words selected, once.
    pub fn take_status_select(&mut self) -> bool {
        std::mem::take(&mut self.status_select)
    }

    pub fn set_status_text(&mut self, words: &str) {
        words.clone_into(&mut self.status);
    }

    /// The typed status differs from the note's: `Unsaved` and `Save status` show.
    pub fn status_unsaved(&self, item: &Item) -> bool {
        self.status_editing && self.status.trim() != status(&item.entry)
    }

    /// `Save status`, or Enter in its field.
    pub fn save_status(
        &mut self,
        item: &Item,
        index: &ContentIndex,
        campaign: &Campaign,
    ) -> Option<Nav> {
        if !self.status_unsaved(item) {
            return None;
        }
        let wanted = self.status.trim().to_owned();
        let go = self.change(campaign, item, index, "status", wanted.into())?;
        self.status_editing = false;
        self.status_drafts.remove(&item.entry.id);
        self.settle(go)
    }

    /// Escape in the reader: an open picker closes first.
    pub fn escape(&mut self) -> bool {
        if self.picker.take().is_some() {
            return true;
        }
        false
    }

    /// The run's place of this NPC, with its initiative, when it is in the fight.
    pub fn in_run_of(&self, id: Uuid) -> Option<(usize, Option<i64>)> {
        self.in_run.get(&id).copied()
    }

    /// The `In the run` link: the run opens at this arrival.
    pub fn press_in_run(&mut self, at: usize) {
        self.select_run = Some(at);
    }

    /// The open `Add to Log` field of this note, unless an edit form holds it back.
    pub fn log_view(
        &self,
        item: &Item,
        index: &ContentIndex,
        campaign: &Campaign,
    ) -> Option<LogFieldView> {
        let key = (campaign.id, item.entry.id);
        if !takes_log(item, index)
            || self.editing(item.entry.id, campaign)
            || self.edited_elsewhere.contains(&item.entry.id)
        {
            return None;
        }
        let open = self.held.log_fields.get(&key)?;
        Some(LogFieldView {
            stamp: self
                .line_stamp()
                .map(|stamp| format!("{}:", plain_named_links(&stamp, index.session_names()))),
            words: open.field.words.clone(),
            name: format!("Log line for {}", item.entry.name),
            select: open.select,
        })
    }

    /// The Log field selected its words.
    pub fn log_selected(&mut self, campaign: Uuid, id: Uuid) {
        if let Some(open) = self.held.log_fields.get_mut(&(campaign, id)) {
            open.select = false;
        }
    }

    pub fn set_log_words(&mut self, campaign: Uuid, id: Uuid, words: &str) {
        if let Some(open) = self.held.log_fields.get_mut(&(campaign, id)) {
            words.clone_into(&mut open.field.words);
        }
    }

    /// Enter (or `Add`) in the Log field: the line is added, or an empty field closes.
    pub fn log_enter(&mut self, item: &Item, campaign: &Campaign) -> Option<Nav> {
        let key = (campaign.id, item.entry.id);
        let words = self
            .held
            .log_fields
            .get(&key)?
            .field
            .words
            .trim()
            .to_owned();
        if words.is_empty() {
            self.held.log_fields.remove(&key);
            return None;
        }
        let go = self.add_log_line(campaign, item, &words)?;
        self.settle(go)
    }

    /// Escape (or `Cancel`) in the Log field: it closes and its words go.
    pub fn log_cancel(&mut self, campaign: Uuid, id: Uuid) {
        self.held.log_fields.remove(&(campaign, id));
    }

    /// `Another type`'s words under `Give this note a type`.
    pub fn other_type(&self) -> &str {
        &self.other_type
    }

    pub fn set_other_type(&mut self, words: &str) {
        words.clone_into(&mut self.other_type);
    }

    /// `Make NPC`, `Make location` or `Make <type>`: the plain note becomes an entry.
    pub fn make_type(&mut self, campaign: &Campaign, item: &Item, kind: &str) -> Option<Nav> {
        match make_entry(campaign, item, kind) {
            Ok(id) => self.settle(Go::Reload(id)),
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    /// The join form, while it is open for an NPC and a run is open.
    pub fn join_view(
        &self,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
    ) -> Option<JoinView> {
        let sides = self.sides.as_ref()?;
        let adding = self.adding.as_ref()?;
        let can_save = campaign.is_some() && item.is_private();
        Some(JoinView {
            title: format!("Add {} to the fight", item.entry.name),
            stat_block: adding
                .stat_block
                .map(|at| (at, index.items[at].entry.name.clone())),
            search: adding.search.clone(),
            matches: if adding.stat_block.is_none() {
                index
                    .creatures_matching(&adding.search, 5)
                    .into_iter()
                    .map(|at| (at, index.items[at].entry.name.clone()))
                    .collect()
            } else {
                Vec::new()
            },
            save: (can_save && creature(&item.entry).is_none()).then_some(adding.save),
            sides: sides.clone(),
            side: adding.side.clone(),
            initiative: adding.initiative.clone(),
        })
    }

    /// A press in the join form.
    pub fn join_act(
        &mut self,
        act: JoinAct,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
    ) -> Option<Nav> {
        let campaign = campaign.filter(|_| item.is_private());
        let adding = self.adding.as_mut()?;
        match act {
            JoinAct::Search(words) => adding.search = words,
            JoinAct::Choose(at) => adding.stat_block = Some(at),
            JoinAct::ChangeStatBlock => {
                if let Some(at) = adding.stat_block.take() {
                    adding.search = index.items[at].entry.name.clone();
                }
            }
            JoinAct::Save(save) => adding.save = save,
            JoinAct::Side(side) => adding.side = side,
            JoinAct::Initiative(words) => adding.initiative = words,
            JoinAct::Cancel => self.adding = None,
            JoinAct::Add => {
                if campaign.is_none() {
                    adding.save = false;
                }
                let stat_block = &index.items[adding.stat_block?].entry;
                let total = match adding.initiative.trim() {
                    "" => None,
                    text => match design::parse_signed(text) {
                        Some(total) => Some(total),
                        None => {
                            self.error = Some("Initiative must be a whole number.".to_owned());
                            return None;
                        }
                    },
                };
                let saved = adding.save && index.world_target(&item.entry, "creature").is_none();
                let side = adding.side.clone();
                if saved && let Some(campaign) = campaign {
                    match set_world_field(
                        campaign,
                        &item.entry,
                        "creature",
                        stored_link(stat_block).into(),
                    ) {
                        Ok(mut change) => {
                            change.word = change_words("Stat block", None, Some(&stat_block.name));
                            self.held.results.record(ReaderResult::Change(change));
                            self.kept = None;
                        }
                        Err(error) => self.error = Some(error),
                    }
                }
                let group = npc_group(&item.entry, stat_block, &side);
                self.adding = None;
                return self.settle(Go::Join(Join {
                    group,
                    total,
                    saved,
                }));
            }
        }
        None
    }

    /// This reader's footer for the open note (`footer`), as plain lines, with the lines a
    /// caller adds after the reader's (`more`).
    pub fn footer_plain(
        &mut self,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
        more: impl for<'f> FnOnce(&'f design::outcome::PaneFrame, &mut Messages<'f, FooterAct>),
    ) -> Vec<plain::Line<FooterAct>> {
        let frame = plain::frame();
        let mut messages = Messages::new(&frame, "plain", 0.0);
        self.footer(&frame, &mut messages, item, index, campaign);
        more(&frame, &mut messages);
        messages.into_plain()
    }

    /// A footer press of the reader's own (every `FooterAct` but `Page`, `Trackers` and
    /// `Untick`): what the page does next, and what the view does beside it.
    pub fn footer_press(
        &mut self,
        press: design::outcome::Press<FooterAct>,
        open: Option<Uuid>,
        campaign: &Campaign,
    ) -> (Option<Nav>, Option<Effect>) {
        let effect = match &press.id {
            FooterAct::GoneCopy(id) => self
                .held
                .gone_log_lines
                .iter()
                .find(|gone| gone.id == *id)
                .map(|gone| Effect::Copy(gone.words.clone())),
            FooterAct::LostCopy(id) => self
                .edits
                .get(id)
                .map(|draft| Effect::Copy(draft.now_text().to_owned())),
            FooterAct::OpenEditor(path) => Some(Effect::Open(obsidian_url(path))),
            _ => None,
        };
        let nav = match press.id {
            // The view copies or opens; the reader's state follows as in `footer_act`.
            FooterAct::GoneCopy(id) => {
                self.end_gone_log(id);
                None
            }
            FooterAct::LostCopy(_) | FooterAct::OpenEditor(_) => None,
            _ => self.footer_act_plain(press, open, campaign),
        };
        (nav, effect)
    }

    /// `footer_act` without the egui context: every press but a copy and an open, which the
    /// view does (`footer_press`).
    fn footer_act_plain(
        &mut self,
        press: design::outcome::Press<FooterAct>,
        open: Option<Uuid>,
        campaign: &Campaign,
    ) -> Option<Nav> {
        let go = match press.id {
            FooterAct::Undo => self.undo_result(press.ticket?, open, campaign),
            FooterAct::GoneDismiss(id) => {
                self.dismiss_gone_log(id);
                None
            }
            FooterAct::UndoDismiss => {
                self.undo_dismiss(press.ticket?);
                None
            }
            FooterAct::LostDiscard(id) => {
                self.discard_lost(id);
                None
            }
            _ => None,
        };
        go.and_then(|go| self.settle(go))
    }

    /// Why the last act did not write, for a check.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

/// The line a generated NPC's card copies (`Copy the marker`).
pub const NAMES_MARKER: &str = crate::lookup::NAMES_MARKER;

/// `Show file`'s link to the file at `path`.
pub fn file_link(path: &Path) -> String {
    format!("file://{}", encode_path(path))
}

/// The words of a duplicate note's fault (`DuplicateIdentity`).
pub fn duplicate_words() -> String {
    use design::outcome::Fault as _;
    DuplicateIdentity(IdentityNoun::Note)
        .cause()
        .as_str()
        .to_owned()
}

/// What a missing value adds after its name.
pub const NOT_FOUND_WORDS: &str = NOT_FOUND;

/// Cancel's reason while a clean edit form holds a Log field back.
pub const LOG_WAITS_FOR_CANCEL_WORDS: &str = LOG_WAITS_FOR_CANCEL;

/// The notes a place link of the note `id` matches, when it matches several.
pub fn place_choices(index: &ContentIndex, id: Uuid) -> Vec<usize> {
    index.place_choices(id)
}

/// The session notes' names, as the Sessions page gives them.
pub fn session_names(index: &ContentIndex) -> &HashMap<String, String> {
    index.session_names()
}
