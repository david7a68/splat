//! The same disk comparison for encounter, group and creature drafts.
#![warn(deprecated)] // Its parent allows old message functions; this file uses none.
use crate::state_context::StateContext;
use crate::{
    design::{self, outcome::Sentence},
    library::note_path,
};
use std::{fs, path::Path};

#[derive(Clone, Default)]
pub(super) struct Recovery {
    pub change: Option<Change>,
    copies_accepted: bool,
}

#[derive(Clone)]
pub(super) struct Change {
    pub path: String,
    pub text: Option<String>,
    pub copies: Vec<String>,
}

/// The result after `Remove <noun> and lose draft`, as Library says it (FF7).
pub(super) fn removed_with_draft(name: &str) -> String {
    format!("{name} removed. Its draft was discarded.")
}

/// The one sentence for a draft whose file changed on disk. It names the thing by its name, never
/// the file (R10): `Bridge Ambush changed on disk.` Only a draft with no name yet says
/// `This <what>`.
pub(super) fn changed_on_disk(what: &str, name: &str) -> Sentence {
    let name = name.trim();
    if name.is_empty() {
        crate::sentence!(
            "This {} changed on disk. Your draft is safe until you choose.",
            what
        )
    } else {
        crate::sentence!(
            "{} changed on disk. Your draft is safe until you choose.",
            name
        )
    }
}

/// A draft's disk conflict (`Recovery::show`), as data for a view that does not draw with egui.
#[derive(Clone, Debug, PartialEq)]
pub struct DiskView {
    /// The facts before the last one (another file claims this identity).
    pub facts: Vec<String>,
    /// The last fact: the choices follow its words.
    pub last: String,
    /// The safe choice: `Overwrite with my draft`.
    pub keep: String,
    /// The red choice: `Reload and lose my draft`, or `Remove <what> and lose draft`.
    pub lose: String,
    /// What differs, part by part, in the DM's words.
    pub differences: Vec<(String, String)>,
    /// The draft's text, as Overwrite would save it.
    pub draft: String,
    /// The file's text now, and its path; none when the file is gone.
    pub disk: Option<(String, String)>,
}

impl Recovery {
    /// `what` names the draft's kind in the DM's words: `encounter`, `group` or `creature`;
    /// `name` is the draft's name.
    pub fn check(
        &mut self,
        root: &Path,
        path: &mut String,
        baseline: Option<&str>,
        (what, name): (&str, &str),
    ) -> Result<(), String> {
        let Some(baseline) = baseline else {
            return Ok(());
        };
        let id = crate::content_workspace::note_id(baseline);
        let (entries, errors) = splat::content::read_directory(root)?;
        let mut peers = Vec::new();
        for other in entries
            .into_iter()
            .map(|entry| entry.path)
            .chain(errors.into_iter().map(|error| error.path))
        {
            if other != *path
                && let Ok(text) = fs::read_to_string(note_path(root, &other)?)
                && id.is_some()
                && crate::content_workspace::note_id(&text) == id
            {
                peers.push((other, text));
            }
        }
        let mut current = read(root, path)?;
        if current.is_none() && peers.len() == 1 {
            let (moved, text) = peers.remove(0);
            *path = moved;
            current = Some(text);
        }
        if current.as_deref() != Some(baseline) || (!peers.is_empty() && !self.copies_accepted) {
            self.change = Some(Change {
                path: path.clone(),
                text: current,
                copies: peers.into_iter().map(|(path, _)| path).collect(),
            });
            return Err(changed_on_disk(what, name).to_string());
        }
        self.change = None;
        Ok(())
    }

    /// The draft's file is gone from the disk: the destructive choice removes the draft's
    /// object and loses the draft (FF7's words), and its result offers `Undo remove`.
    pub fn file_gone(&self) -> bool {
        self.change
            .as_ref()
            .is_some_and(|change| change.text.is_none())
    }

    pub fn accept(&mut self) -> Option<Change> {
        self.copies_accepted = true;
        self.change.take()
    }

    /// The conflict as data (`show`'s facts, choices and comparison), while there is one.
    pub fn view(&self, draft: &str, (what, name): (&str, &str)) -> Option<DiskView> {
        let change = self.change.as_ref()?;
        let mut facts = vec![changed_on_disk(what, name).to_string()];
        facts.extend(change.copies.iter().map(|path| {
            crate::sentence!(
                "Another file claims to be this {}: {}. Both files stay.",
                what,
                path
            )
            .to_string()
        }));
        let last = facts.pop()?;
        Some(DiskView {
            facts,
            last,
            keep: "Overwrite with my draft".to_owned(),
            lose: if change.text.is_some() {
                "Reload and lose my draft".to_owned()
            } else {
                format!("Remove {what} and lose draft")
            },
            differences: change
                .text
                .as_deref()
                .map(|text| differences(draft, text))
                .unwrap_or_default(),
            draft: draft.to_owned(),
            disk: change.text.clone().map(|text| (change.path.clone(), text)),
        })
    }
}

/// What differs between the draft and the file, one part at a time, as a label and the DM's
/// words: a property, the notes (every line outside a table), and each table by its heading. A
/// table whose rows name the same things in another form says so.
pub(super) fn differences(draft: &str, disk: &str) -> Vec<(String, String)> {
    let (mine, theirs) = (Note::of(draft), Note::of(disk));
    let mut lines = Vec::new();
    let keys: std::collections::BTreeSet<_> = mine
        .properties
        .keys()
        .chain(theirs.properties.keys())
        .collect();
    for key in keys {
        let (a, b) = (mine.properties.get(key), theirs.properties.get(key));
        if a == b {
            continue;
        }
        let shown = |value: Option<&String>| {
            value.map_or_else(|| "nothing".to_owned(), |value| format!("“{value}”"))
        };
        lines.push((
            property_label(key),
            format!("your draft says {}; the file says {}.", shown(a), shown(b)),
        ));
    }
    let notes = note_changes(&mine.notes, &theirs.notes);
    if !notes.is_empty() {
        lines.push(("Notes".into(), notes.join("; ")));
    }
    let headings: Vec<_> = mine
        .tables
        .iter()
        .chain(&theirs.tables)
        .map(|(heading, _)| heading.clone())
        .fold(Vec::new(), |mut all, heading| {
            if !all.contains(&heading) {
                all.push(heading);
            }
            all
        });
    for heading in headings {
        let rows = |note: &Note| {
            note.tables
                .iter()
                .find(|(name, _)| *name == heading)
                .map(|(_, rows)| rows.clone())
                .unwrap_or_default()
        };
        let (a, b) = (rows(&mine), rows(&theirs));
        if a == b {
            continue;
        }
        let words = if a
            .iter()
            .map(|row| row_key(row))
            .eq(b.iter().map(|row| row_key(row)))
        {
            if b.len() == 1 {
                "the file names the same row in another form.".to_owned()
            } else {
                format!("the file names the same {} rows in another form.", b.len())
            }
        } else if a.len() != b.len() {
            format!(
                "your draft has {}; the file has {}.",
                design::count(a.len(), "row", "rows"),
                b.len()
            )
        } else {
            let differ = a
                .iter()
                .zip(&b)
                .filter(|(a, b)| row_key(a) != row_key(b))
                .count();
            if differ == 1 {
                "1 row differs.".to_owned()
            } else {
                format!("{differ} rows differ.")
            }
        };
        lines.push((heading, words));
    }
    lines
}

/// A note in parts: its properties, the lines outside tables, and each table's rows.
struct Note {
    properties: std::collections::BTreeMap<String, String>,
    notes: Vec<String>,
    tables: Vec<(String, Vec<String>)>,
}

impl Note {
    fn of(text: &str) -> Self {
        let text = text.replace("\r\n", "\n");
        let (front, body) = text
            .strip_prefix("---\n")
            .and_then(|rest| rest.split_once("\n---\n"))
            .unwrap_or(("", text.as_str()));
        let properties = serde_yaml_ng::from_str::<serde_yaml_ng::Mapping>(front)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|(key, value)| {
                let shown = match value {
                    serde_yaml_ng::Value::String(text) => text,
                    serde_yaml_ng::Value::Number(number) => number.to_string(),
                    serde_yaml_ng::Value::Bool(flag) => flag.to_string(),
                    other => serde_yaml_ng::to_string(&other).ok()?.trim().to_owned(),
                };
                Some((key.as_str()?.to_owned(), shown))
            })
            .collect();
        let mut notes = Vec::new();
        let mut tables: Vec<(String, Vec<String>)> = Vec::new();
        let mut heading = String::new();
        let mut in_table = false;
        for line in body.lines().map(str::trim) {
            if let Some(name) = line.strip_prefix("## ") {
                heading = name.trim().to_owned();
                in_table = false;
            } else if line.starts_with('|') {
                // The first two lines of a table are its header and separator.
                if !in_table {
                    in_table = true;
                    tables.push((heading.clone(), vec![line.to_owned()]));
                } else if let Some((_, rows)) = tables.last_mut() {
                    rows.push(line.to_owned());
                }
            } else {
                in_table = false;
                if !line.is_empty() && !line.starts_with('#') {
                    notes.push(line.to_owned());
                }
            }
        }
        for (_, rows) in &mut tables {
            rows.drain(..rows.len().min(2));
        }
        Self {
            properties,
            notes,
            tables,
        }
    }
}

/// `splat_name` → `Name`.
fn property_label(key: &str) -> String {
    let words = key.strip_prefix("splat_").unwrap_or(key).replace('_', " ");
    let mut chars = words.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

/// A table row by what it names: a cell that holds an id stands for that id, whatever the form
/// of its link; other cells by their words.
fn row_key(row: &str) -> Vec<String> {
    row.trim_matches('|')
        .split(" | ")
        .map(|cell| {
            cell.split(|c: char| !(c.is_ascii_hexdigit() || c == '-'))
                .find(|word| uuid::Uuid::parse_str(word).is_ok())
                .map_or_else(|| cell.trim().to_owned(), str::to_owned)
        })
        .collect()
}

/// What each side adds to the notes. A line that the other side has the start of adds only
/// its end.
fn note_changes(mine: &[String], theirs: &[String]) -> Vec<String> {
    let mut only_theirs: Vec<&String> = theirs.iter().filter(|line| !mine.contains(line)).collect();
    let mut changes = Vec::new();
    for line in mine.iter().filter(|line| !theirs.contains(line)) {
        if let Some(at) = only_theirs
            .iter()
            .position(|other| line.starts_with(other.as_str()) || other.starts_with(line.as_str()))
        {
            let other = only_theirs.remove(at);
            if line.len() >= other.len() {
                changes.push(format!("your draft adds “{}”", line[other.len()..].trim()));
            } else {
                changes.push(format!("the file adds “{}”", other[line.len()..].trim()));
            }
        } else {
            changes.push(format!("your draft adds “{line}”"));
        }
    }
    for line in only_theirs {
        changes.push(format!("the file adds “{line}”"));
    }
    changes
}

fn read(root: &Path, path: &str) -> Result<Option<String>, String> {
    match fs::read_to_string(note_path(root, path)?) {
        Ok(text) => Ok(Some(text)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("{path}: {error}")),
    }
}

/// Compare the exact schema-2 text that Overwrite will save.
pub(super) fn comparison_text(
    _path: &str,
    text: String,
    snapshot: &super::Snapshot,
) -> Result<String, String> {
    splat::content::list_formats::write_references(
        &text,
        snapshot
            .catalogue
            .entries
            .values()
            .map(|entry| &entry.entry),
    )
}

impl super::Encounters {
    /// The encounter editor's disk conflict: its own, else the first of its groups' (the
    /// editor shows it in its place, `encounter_disk_view`).
    pub fn encounter_disk(&self) -> Option<DiskView> {
        let snapshot = self.snapshot.as_ref()?;
        let draft = self.draft.as_ref()?;
        if draft.recovery.change.is_none() {
            let group = draft.groups.iter().find(|group| group.has_disk_change())?;
            return group.disk_view(snapshot);
        }
        draft.recovery.view(
            &draft.comparison_text(snapshot).unwrap_or_default(),
            ("encounter", &draft.name),
        )
    }

    /// The DM's choice in the encounter editor's disk conflict: `keep` overwrites with the
    /// draft, else the draft goes (with Undo) and the file's text comes back.
    pub fn encounter_disk_choice(
        &mut self,
        ctx: &StateContext,
        campaign: &crate::persistence::Campaign,
        modules: &[crate::persistence::Module],
        keep: bool,
    ) {
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        if draft.recovery.change.is_none() {
            let Some(index) = draft
                .groups
                .iter()
                .position(|group| group.has_disk_change())
            else {
                return;
            };
            let gone = draft.groups[index].file_gone();
            match draft.groups[index].disk_choice(&campaign.path, keep) {
                Ok(()) if keep => self.save_draft(ctx, campaign, modules),
                Ok(()) => {
                    // FF7: the group's file is gone. It leaves this encounter's draft, and
                    // Undo remove puts it back at its place with its draft.
                    if gone {
                        let group = draft.groups.remove(index);
                        if draft.add_to.as_deref() == Some(group.path()) {
                            draft.add_to = None;
                        }
                        draft.new_results.push(super::DraftUndo::RemovedGroup {
                            index,
                            group: Box::new(group),
                            delete: false,
                        });
                    }
                    self.error = None;
                }
                Err(error) => self.error = Some(error),
            }
            return;
        }
        if keep {
            if let Some(change) = draft.recovery.accept() {
                draft.path = change.path;
                draft.baseline = change.text.clone();
                if let Some(text) = change.text {
                    draft.source = text;
                }
                self.save_draft(ctx, campaign, modules);
            }
        } else {
            let path = draft.path.clone();
            let old = self.draft.take();
            self.draft = super::Draft::open(&campaign.path, &path).ok();
            self.discarded = old;
            self.stamp_kept(super::KeptResult::Discarded);
            self.error = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_23_b_fb1_differences_name_each_part_in_the_dms_words() {
        let id = "1fd24d90-202f-80d7-94e3-2efade3bde0b";
        let draft = format!(
            "---\nsplat_schema: 2\nsplat_name: Bridge Ambush\n---\n\nWolves hold the bank. Ash on the water.\n\n## Roster\n\n| Side | Creature | Note |\n| --- | --- | --- |\n| party | [[Characters/Brannoch\\|Brannoch]] %% splat id={id} %% | %% splat slot=a %% |\n"
        );
        let disk = format!(
            "---\nsplat_schema: 2\nsplat_name: Bridge Ambush\n---\n\nWolves hold the bank.\n\n## Roster\n\n| Side | Creature | Note |\n| --- | --- | --- |\n| party | [[{id}]] | %% splat slot=a %% |\n\nWritten outside Splat.\n"
        );
        assert_eq!(
            differences(&draft, &disk),
            [
                (
                    "Notes".to_owned(),
                    "your draft adds “Ash on the water.”; the file adds “Written outside Splat.”"
                        .to_owned()
                ),
                (
                    "Roster".to_owned(),
                    "the file names the same row in another form.".to_owned()
                ),
            ]
        );
        // Another creature in the row, another name: each is its own line.
        let other = disk
            .replace(id, "c976a55c-1ca2-8dfc-888f-9d7a28d8a5d0")
            .replace("splat_name: Bridge Ambush", "splat_name: Ford Ambush");
        let lines = differences(&draft, &other);
        assert!(lines.contains(&(
            "Name".to_owned(),
            "your draft says “Bridge Ambush”; the file says “Ford Ambush”.".to_owned()
        )));
        assert!(lines.contains(&("Roster".to_owned(), "1 row differs.".to_owned())));
        // The same text: nothing to say.
        assert!(differences(&draft, &draft).is_empty());
    }
}
