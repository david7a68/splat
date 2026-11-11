//! The same disk comparison for encounter, group and creature drafts.
use crate::{
    design::{self, Action, Density, Tone, Type, colors},
    library::note_path,
};
use eframe::egui;
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

/// The one sentence for a draft whose file changed on disk. It names what changed, never the file.
/// The result after `Remove <noun> and lose draft`, as Library says it (FF7).
pub(super) fn removed_with_draft(name: &str) -> String {
    format!("{name} removed. Its draft was discarded.")
}

pub(super) fn changed_on_disk(what: &str) -> String {
    format!("This {what} changed on disk. Your draft is safe until you choose.")
}

impl Recovery {
    /// `what` names the draft's kind in the DM's words: `encounter`, `group` or `creature`.
    pub fn check(
        &mut self,
        root: &Path,
        path: &mut String,
        baseline: Option<&str>,
        what: &str,
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
            return Err(changed_on_disk(what));
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

    pub fn show(&self, ui: &mut egui::Ui, draft: &str, what: &str) -> Option<bool> {
        let change = self.change.as_ref()?;
        let mut messages = vec![changed_on_disk(what)];
        messages.extend(
            change
                .copies
                .iter()
                .map(|path| format!("Another file uses this identity: {path}. Both files stay.")),
        );
        let last = messages.pop().unwrap_or_default();
        for message in &messages {
            design::message(ui, Tone::Warning, message);
        }
        // The choices follow the last fact the DM reads, at the message text: safe first, then
        // an `L` gap before the red one.
        let mut choice = None;
        design::message_with_actions(ui, Tone::Warning, &last, |ui| {
            ui.horizontal_wrapped(|ui| {
                // The same Read density as every other disk-conflict choice row.
                if Action::secondary("Overwrite with my draft")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    choice = Some(true);
                }
                // egui adds its item spacing too, so the whole gap is exactly `L`.
                ui.add_space(design::L - ui.spacing().item_spacing.x);
                // FF7: the file is gone, so the choice says what it removes and loses.
                let lose = if change.text.is_some() {
                    "Reload and lose my draft".to_owned()
                } else {
                    format!("Remove {what} and lose draft")
                };
                if Action::destructive(&lose)
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    choice = Some(false);
                }
            });
        });
        if let Some(text) = &change.text {
            // One line for each part that differs, in the DM's words, above the raw lines.
            let key = egui::Id::new(("disk_differences", draft, text.as_str()));
            let lines = ui
                .data(|data| data.get_temp::<Vec<(String, String)>>(key))
                .unwrap_or_else(|| {
                    let lines = differences(draft, text);
                    ui.data_mut(|data| data.insert_temp(key, lines.clone()));
                    lines
                });
            if !lines.is_empty() {
                ui.add_space(design::S);
            }
            for (label, words) in &lines {
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
            design::draft_comparison_styled(
                ui,
                draft,
                text,
                (&change.path, text),
                design::Type::Mono.font(ui),
                design::Type::Mono.line(),
            );
        }
        choice
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
    pub(super) fn encounter_disk_view(
        &mut self,
        ui: &mut egui::Ui,
        campaign: &crate::persistence::Campaign,
        modules: &[crate::persistence::Module],
    ) -> bool {
        let Some(snapshot) = self.snapshot.as_ref() else {
            return false;
        };
        let Some(draft) = self.draft.as_mut() else {
            return false;
        };
        if draft.recovery.change.is_none() {
            if let Some(group) = draft
                .groups
                .iter_mut()
                .find(|group| group.has_disk_change())
            {
                match group.disk_choice(ui, &campaign.path, snapshot) {
                    Ok(Some(true)) => self.save_draft(ui.ctx(), campaign, modules),
                    Ok(Some(false)) => self.error = None,
                    Err(error) => self.error = Some(error),
                    _ => {}
                }
                return true;
            }
            return false;
        }
        let choice = draft.recovery.show(
            ui,
            &draft.comparison_text(snapshot).unwrap_or_default(),
            "encounter",
        );
        if let Some(keep) = choice {
            if keep {
                if let Some(change) = draft.recovery.accept() {
                    draft.path = change.path;
                    draft.baseline = change.text.clone();
                    if let Some(text) = change.text {
                        draft.source = text;
                    }
                    self.save_draft(ui.ctx(), campaign, modules);
                }
            } else {
                let path = draft.path.clone();
                let old = self.draft.take();
                self.draft = super::Draft::open(&campaign.path, &path).ok();
                self.discarded = old;
                self.error = None;
            }
        }
        true
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
