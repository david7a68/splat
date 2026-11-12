//! `Trackers.md`, the ledger: one ordered list of acts in the campaign's root. This module is
//! its one reader and its one writer. An act is applied to the file as it is on disk at the
//! click, and saved with the shared atomic write and its outside-change check
//! (`file_batch::apply`), so one act writes one file and never half of it.
use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

/// The ledger's file name, in the campaign's root folder.
pub const FILE: &str = "Trackers.md";
const VERSION: u32 = 1;

fn version() -> u32 {
    VERSION
}

fn is_false(value: &bool) -> bool {
    !value
}

/// The whole file: a version, the acts in order, and one line of free text for a value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ledger {
    #[serde(default = "version")]
    pub trackers_version: u32,
    #[serde(default)]
    pub changes: Vec<Entry>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub notes: BTreeMap<String, String>,
    /// Keys this version does not know stay in the file.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl Default for Ledger {
    fn default() -> Self {
        Self {
            trackers_version: VERSION,
            changes: Vec::new(),
            notes: BTreeMap::new(),
            extra: BTreeMap::new(),
        }
    }
}

/// One act: a tracker change, a roll on a notice, or the end of a notice.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracker: Option<String>,
    /// The instance of a tracker with `per`, or the name a notice's roll is for.
    #[serde(default, rename = "for", skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<i64>,
    /// What a `set` replaced: the one value that cannot be worked out from the entries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub was: Option<i64>,
    /// The quick action's label, so the result line and Undo name the act after a restart.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notice: Option<NoticeRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roll: Option<Roll>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row: Option<String>,
    /// The changes of one press share a `use`, and undo together.
    #[serde(default, rename = "use", skip_serializing_if = "Option::is_none")]
    pub press: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub done: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub dismissed: bool,
    /// The session being played when the act was made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// A notice's identity: the change that crossed the mark, and the mark's value.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NoticeRef {
    pub change: String,
    pub at: i64,
}

/// A quick action's dice (`"1d6: [4]"`), or a notice's roll on a table.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Roll {
    Dice(String),
    Table(TableRoll),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableRoll {
    pub table: String,
    #[serde(default, rename = "for", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub value: i64,
    pub row: String,
}

impl Entry {
    /// The entries of one act: those that share its `use`, or itself.
    pub fn same_press(&self, other: &Entry) -> bool {
        match (&self.press, &other.press) {
            (Some(a), Some(b)) => a == b,
            _ => self.id == other.id,
        }
    }
}

/// A new entry ID: `c-` and eight hex digits.
pub fn new_id(prefix: &str) -> String {
    let id = crate::hermetic::new_uuid().simple().to_string();
    format!("{prefix}-{}", &id[id.len() - 8..])
}

/// One act, as a change of the list on disk.
#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    /// New entries at the end. An entry whose ID is already in the file is not added again,
    /// so a retry never makes a second entry for one press.
    Append(Vec<Entry>),
    /// These entries, exactly as they were read, leave the list.
    Remove(Vec<Entry>),
}

impl Ledger {
    /// The file's text. A missing file is an empty ledger; a file that cannot be read is an
    /// error, and nothing is ever written over it.
    pub fn parse(text: &str) -> Result<Self, String> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        let (front, _) = crate::sessions::split_note(text)
            .ok_or("Trackers.md has no properties block at its top.")?;
        let yaml = front_yaml(front);
        let value: Value = serde_yaml_ng::from_str(yaml).map_err(|error| error.to_string())?;
        // A note of the DM's with this name is not a ledger, and is never written over.
        if value.get("trackers_version").is_none() {
            return Err(
                "Trackers.md is not a tracker ledger: its properties have no trackers_version."
                    .into(),
            );
        }
        let ledger: Self = serde_yaml_ng::from_value(value).map_err(|error| error.to_string())?;
        let mut seen = std::collections::HashSet::new();
        for entry in &ledger.changes {
            if !seen.insert(entry.id.as_str()) {
                return Err(format!("The ID {} is on two entries.", entry.id));
            }
        }
        Ok(ledger)
    }

    /// The text to write, with the body of `old` kept below the properties.
    pub fn render(&self, old: &str) -> Result<String, String> {
        let body = crate::sessions::split_note(old).map_or("", |(_, body)| body);
        let yaml = serde_yaml_ng::to_string(self).map_err(|error| error.to_string())?;
        Ok(format!("---\n{yaml}---\n{body}"))
    }

    /// The list after `op`, or why it cannot be applied to this list.
    pub fn apply(&self, op: &Op) -> Result<Self, String> {
        let mut next = self.clone();
        match op {
            Op::Append(entries) => {
                for entry in entries {
                    if !next.changes.iter().any(|old| old.id == entry.id) {
                        next.changes.push(entry.clone());
                    }
                }
            }
            Op::Remove(entries) => {
                for entry in entries {
                    match next.changes.iter().position(|old| old.id == entry.id) {
                        Some(at) if next.changes[at] == *entry => {
                            next.changes.remove(at);
                        }
                        // Gone already: an earlier try or an edit took it out.
                        None => {}
                        Some(_) => {
                            return Err("That entry changed on disk. Nothing was undone.".into());
                        }
                    }
                }
            }
        }
        Ok(next)
    }
}

/// The YAML between the fences of a properties block.
fn front_yaml(front: &str) -> &str {
    let inner = front
        .trim_start_matches('\u{feff}')
        .strip_prefix("---")
        .unwrap_or(front);
    let inner = inner.trim_start_matches(['\r', '\n']);
    inner.rsplit_once("---").map_or(inner, |(yaml, _)| yaml)
}

/// Is this file in a campaign's root the ledger? Its name, and `trackers_version` in its
/// properties.
pub fn is_ledger(file_name: &str, front: Option<&str>) -> bool {
    file_name.eq_ignore_ascii_case(FILE)
        && front.is_some_and(|front| {
            front
                .lines()
                .any(|line| line.starts_with("trackers_version:"))
        })
}

/// Where the ledger of a campaign lives.
pub fn path(campaign_root: &Path) -> PathBuf {
    campaign_root.join(FILE)
}

/// What a save could not do.
#[derive(Clone, Debug, PartialEq)]
pub enum Fault {
    /// The file on disk cannot be read. Nothing is written.
    NotRead(String),
    /// The act does not fit the list on disk now. Nothing is written.
    Refused(String),
    /// The write failed. The act waits, and a retry adds it once.
    NotSaved(String, Op),
}

/// The file as it is on disk now: its bytes, or none when it does not exist.
pub fn read_bytes(path: &Path) -> Result<Option<Vec<u8>>, String> {
    match fs::read(path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.to_string()),
    }
}

/// The ledger in these bytes.
pub fn parse_bytes(bytes: Option<&[u8]>) -> Result<Ledger, String> {
    match bytes {
        None => Ok(Ledger::default()),
        Some(bytes) => Ledger::parse(&String::from_utf8_lossy(bytes)),
    }
}

/// Read the file now, build the act from what it holds, and write the result as one atomic
/// save. Returns the bytes now on disk and the list they hold.
pub fn commit(
    path: &Path,
    build: impl FnOnce(&Ledger) -> Result<Op, String>,
) -> Result<(Vec<u8>, Ledger, Op), Fault> {
    let before = read_bytes(path).map_err(Fault::NotRead)?;
    let ledger = parse_bytes(before.as_deref()).map_err(Fault::NotRead)?;
    let op = build(&ledger).map_err(Fault::Refused)?;
    let next = ledger.apply(&op).map_err(Fault::Refused)?;
    let old = before
        .as_deref()
        .map(String::from_utf8_lossy)
        .unwrap_or_default();
    let text = match next.render(&old) {
        Ok(text) => text,
        Err(error) => return Err(Fault::NotSaved(error, op)),
    };
    if let Err(error) = crate::file_batch::apply(&[crate::file_batch::Change {
        path: path.to_path_buf(),
        before: before.clone(),
        after: Some(text.clone().into_bytes()),
    }]) {
        return Err(Fault::NotSaved(error, op));
    }
    Ok((text.into_bytes(), next, op))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DESIGN: &str = "---\ntrackers_version: 1\nchanges:\n- id: c-1a2b3c4d\n  tracker: seals\n  for: Old Guard\n  add: 1\n  session: 070eca36-331c-46cc-b5a6-32a6e35265da\n- id: c-5e6f7a8b\n  tracker: favor\n  add: -1\n  use: u-77aa01\n  roll: \"1d6: [4]\"\n  row: A healing potion\n  event: t-08a07eb0ec\n- id: c-6f7a8b9c\n  notice: {change: c-1a2b3c4d, at: 1}\n  roll: {table: rival_track, for: Old Guard, value: 4, row: Takes a seal.}\n  session: 070eca36-331c-46cc-b5a6-32a6e35265da\n- id: c-7a8b9c0d\n  notice: {change: c-1a2b3c4d, at: 1}\n  done: true\n  session: 070eca36-331c-46cc-b5a6-32a6e35265da\nnotes:\n  seals/Old Guard: Coming for the party\n---\n\nThe DM's own words.\n";

    #[test]
    fn the_designs_ledger_reads_and_writes_back_every_field() {
        let ledger = Ledger::parse(DESIGN).unwrap();
        assert_eq!(ledger.changes.len(), 4);
        assert_eq!(ledger.changes[0].instance.as_deref(), Some("Old Guard"));
        assert_eq!(ledger.changes[1].roll, Some(Roll::Dice("1d6: [4]".into())));
        assert!(matches!(
            &ledger.changes[2].roll,
            Some(Roll::Table(TableRoll { value: 4, name: Some(name), .. })) if name == "Old Guard"
        ));
        assert!(ledger.changes[3].done);
        let text = ledger.render(DESIGN).unwrap();
        assert!(text.ends_with("---\n\nThe DM's own words.\n"), "{text}");
        assert_eq!(Ledger::parse(&text).unwrap(), ledger);
    }

    #[test]
    fn a_retry_adds_an_entry_once_and_undo_refuses_an_entry_changed_on_disk() {
        let ledger = Ledger::parse(DESIGN).unwrap();
        let entry = Entry {
            id: "c-00000001".into(),
            tracker: Some("favor".into()),
            add: Some(1),
            ..Entry::default()
        };
        let once = ledger.apply(&Op::Append(vec![entry.clone()])).unwrap();
        let twice = once.apply(&Op::Append(vec![entry.clone()])).unwrap();
        assert_eq!(once, twice);
        assert_eq!(twice.changes.len(), 5);
        let mut edited = once.clone();
        edited.changes[4].add = Some(2);
        assert!(edited.apply(&Op::Remove(vec![entry.clone()])).is_err());
        let undone = once.apply(&Op::Remove(vec![entry.clone()])).unwrap();
        assert_eq!(undone, ledger);
        assert_eq!(undone.apply(&Op::Remove(vec![entry])).unwrap(), ledger);
    }

    #[test]
    fn a_file_that_cannot_be_read_is_never_written_over() {
        let dir = tempfile::tempdir().unwrap();
        let path = path(dir.path());
        fs::write(&path, "---\nchanges: [\n---\n").unwrap();
        let fault = commit(&path, |_| Ok(Op::Append(Vec::new()))).unwrap_err();
        assert!(matches!(fault, Fault::NotRead(_)), "{fault:?}");
        assert_eq!(fs::read_to_string(&path).unwrap(), "---\nchanges: [\n---\n");
        // A note of the DM's with the ledger's name.
        let note = "---\ntags: [race]\n---\n\n# Trackers\n";
        fs::write(&path, note).unwrap();
        assert!(matches!(
            commit(&path, |_| Ok(Op::Append(Vec::new()))),
            Err(Fault::NotRead(_))
        ));
        assert_eq!(fs::read_to_string(&path).unwrap(), note);
        fs::write(&path, DESIGN.replace("c-5e6f7a8b", "c-1a2b3c4d")).unwrap();
        assert!(matches!(
            commit(&path, |_| Ok(Op::Append(Vec::new()))),
            Err(Fault::NotRead(_))
        ));
    }
}
