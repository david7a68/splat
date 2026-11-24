use crate::content_workspace::CampaignDocument;
use crate::persistence::Campaign;
use serde_yaml_ng::Value;
use splat::content::Entry;
pub use splat_campaign::note::split_note;
use std::{
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub const FOLDER_FIELD: &str = "splat_session_storage";
pub const DEFAULT_FOLDER: &str = "Sessions";
pub const TEMPLATES: &str = "Templates";
pub const RECORDINGS: &str = "Recordings";

/// The folder's name as the manifest gives it.
pub fn folder_setting(campaign: &Campaign) -> Result<String, String> {
    let document = CampaignDocument::load(campaign)?;
    let entry = Entry::parse(&document.path, &document.text)?;
    match entry
        .fields
        .get(FOLDER_FIELD)
        .and_then(|v| v.get("directory"))
    {
        Some(value) => Ok(value
            .as_str()
            .ok_or("The session folder must be text.")?
            .to_owned()),
        None => Ok(DEFAULT_FOLDER.to_owned()),
    }
}

pub fn resolve(campaign: &Campaign, name: &str) -> Result<PathBuf, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("Enter a session folder.".into());
    }
    let path = Path::new(name);
    if path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err("Session folders cannot contain “..”.".into());
    }
    Ok(if path.is_absolute() {
        path.into()
    } else {
        campaign.path.join(path)
    })
}

pub fn folder(campaign: &Campaign) -> Result<PathBuf, String> {
    folder_typed(campaign).map_err(String::from)
}

/// The campaign's session folder, or why it cannot be used, as a typed fault.
pub fn folder_typed(campaign: &Campaign) -> Result<PathBuf, FolderFault> {
    let path = resolve(campaign, &folder_setting(campaign)?)?;
    let document = CampaignDocument::load(campaign)?;
    let entry = Entry::parse(&document.path, &document.text)?;
    if entry.fields.contains_key(FOLDER_FIELD) && !path.is_dir() {
        return Err(FolderFault::Missing(path));
    }
    Ok(path)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub id: Uuid,
    pub number: u32,
    pub title: String,
    pub date: String,
    pub path: PathBuf,
    /// Other files that claim this session, such as a sync conflict's copy.
    pub also: Vec<PathBuf>,
}

impl Note {
    /// `Session 12`, then the title if it has one.
    pub fn label(&self) -> String {
        let mut label = format!("Session {}", self.number);
        if !self.title.trim().is_empty() {
            label.push_str(": ");
            label.push_str(self.title.trim());
        }
        label
    }

    /// The name a link in Obsidian uses.
    pub fn link(&self) -> String {
        self.path
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
            .unwrap_or_default()
    }
}

/// The YAML inside a note's properties block, without its fences.
pub fn properties_yaml(text: &str) -> Option<&str> {
    let (front, _) = split_note(text)?;
    Some(
        front
            .trim_start_matches('\u{feff}')
            .trim_end()
            .strip_suffix("---")?
            .trim_start_matches("---"),
    )
}

pub fn parse_note(path: &Path, text: &str) -> Option<Note> {
    let value: Value = serde_yaml_ng::from_str(properties_yaml(text)?).ok()?;
    let id = Uuid::parse_str(value.get("session_id")?.as_str()?).ok()?;
    let text_of = |name: &str| match value.get(name) {
        Some(Value::String(text)) => text.clone(),
        Some(Value::Number(number)) => number.to_string(),
        _ => String::new(),
    };
    Some(Note {
        id,
        number: value
            .get("session_number")
            .and_then(Value::as_u64)
            .and_then(|n| u32::try_from(n).ok())
            .unwrap_or(0),
        title: text_of("session_title"),
        date: text_of("session_date"),
        path: path.to_owned(),
        also: Vec::new(),
    })
}

/// Every note in the folder, newest first. Other Markdown files there are not sessions.
pub fn list_notes(folder: &Path) -> Vec<Note> {
    let mut notes: Vec<Note> = note_paths(folder)
        .into_iter()
        .filter_map(|path| parse_note(&path, &fs::read_to_string(&path).ok()?))
        .collect();
    // The plainest name first, so that a copy such as `Session 12 (conflict).md` never wins.
    let known = known_notes(folder);
    notes.sort_by_key(|note| {
        (
            known
                .get(&note.id)
                .is_none_or(|path| folder.join(path) != note.path),
            note.path.as_os_str().len(),
            note.path.clone(),
        )
    });
    let mut unique: Vec<Note> = Vec::new();
    for note in notes {
        match unique.iter_mut().find(|kept| kept.id == note.id) {
            Some(kept) => kept.also.push(note.path),
            None => unique.push(note),
        }
    }
    unique.sort_by(|a, b| {
        b.number
            .cmp(&a.number)
            .then_with(|| b.date.cmp(&a.date))
            .then_with(|| a.path.cmp(&b.path))
    });
    unique
}

pub fn note_paths(folder: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for file in fs::read_dir(folder).into_iter().flatten().flatten() {
        let path = file.path();
        let name = file.file_name();
        if name.to_string_lossy().starts_with('.') || name == TEMPLATES || name == RECORDINGS {
            continue;
        }
        let Ok(kind) = file.file_type() else {
            continue;
        };
        if kind.is_dir() {
            paths.extend(note_paths(&path));
        } else if kind.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            paths.push(path);
        }
    }
    paths
}

pub const NOTE_PATHS: &str = ".splat-session-paths.json";

pub fn known_notes(folder: &Path) -> std::collections::BTreeMap<Uuid, PathBuf> {
    fs::read(folder.join(NOTE_PATHS))
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}

/// What a file name cannot hold, in any common file system.
pub fn file_safe(text: &str) -> String {
    text.chars()
        .filter(|c| {
            !matches!(
                c,
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '#' | '^' | '[' | ']'
            )
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Moves `source` to `target` in one step, and never replaces a file at `target`: Linux's
/// `RENAME_NOREPLACE` refuses with `AlreadyExists`, even for a file that arrives at the last
/// moment. Moves across file systems fail with `EXDEV`.
#[cfg(target_os = "linux")]
pub fn rename_no_replace(source: &Path, target: &Path) -> std::io::Result<()> {
    use std::os::unix::ffi::OsStrExt;
    let from =
        std::ffi::CString::new(source.as_os_str().as_bytes()).map_err(std::io::Error::other)?;
    let to =
        std::ffi::CString::new(target.as_os_str().as_bytes()).map_err(std::io::Error::other)?;
    // Both paths are valid C strings.
    if unsafe {
        libc::renameat2(
            libc::AT_FDCWD,
            from.as_ptr(),
            libc::AT_FDCWD,
            to.as_ptr(),
            libc::RENAME_NOREPLACE,
        )
    } == 0
    {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

/// A note's `[[links]]` as the words a reader expects, not as markup.
pub fn plain_links(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("[[") {
        out.push_str(&rest[..start]);
        rest = &rest[start + 2..];
        let Some(end) = rest.find("]]") else {
            out.push_str("[[");
            break;
        };
        let inner = &rest[..end];
        out.push_str(inner.rsplit('|').next().unwrap_or(inner).trim());
        rest = &rest[end + 2..];
    }
    out.push_str(rest);
    out
}

/// The note's words with one line replaced, or taken out with `None`. The line is the one at
/// `at` that the act was drawn from, and it must still hold `line` exactly. `None` comes back
/// when that place holds other words: the act never looks for another line that reads the same.
pub fn change_line(body: &str, at: usize, line: &str, new: Option<&str>) -> Option<String> {
    let lines: Vec<&str> = body.split_inclusive('\n').collect();
    if lines.get(at)?.trim_end_matches(['\n', '\r']) != line {
        return None;
    }
    let mut out = String::with_capacity(body.len());
    for (index, text) in lines.iter().enumerate() {
        if index != at {
            out.push_str(text);
        } else if let Some(new) = new {
            out.push_str(new);
            out.push_str(&text[line.len()..]);
        }
    }
    Some(out)
}

/// The same task line, ticked or not: one character changes.
pub fn ticked(line: &str, done: bool) -> String {
    let at = line.find('[').map_or(0, |at| at + 1);
    let mut out = line.to_owned();
    if at > 0 && at < line.len() {
        out.replace_range(at..at + 1, if done { "x" } else { " " });
    }
    out
}

use crate::design::{
    self,
    outcome::{Fault, Sentence, Severity},
};
/// Why the session folder cannot be used, or why a write to it failed. The kind gives the tone,
/// in the page's own place and in the footer of the Session folder form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FolderFault {
    /// The folder the campaign names is gone: the DM chooses it again (routing row 136).
    Missing(PathBuf),
    /// The campaign's note or the folder cannot be read, in the system's own words.
    CannotRead(String),
    /// A write failed: the folder setting, the move, or the notes' list.
    NotSaved(String),
    /// A folder save waited, because the open note could not be saved first.
    NoteNotSaved,
    /// Save folder found files in the current folder that the form did not show yet.
    FilesCame,
    /// Save without moving found other files in the current folder than its warning named.
    FilesChanged,
    /// Save without moving found no file left in the current folder: Save folder is the way on.
    FilesGone,
}

impl From<String> for FolderFault {
    fn from(error: String) -> Self {
        Self::CannotRead(error)
    }
}

impl From<FolderFault> for String {
    fn from(fault: FolderFault) -> Self {
        design::outcome::fault_words(&fault).as_str().to_owned()
    }
}

impl Fault for FolderFault {
    fn severity(&self) -> Severity {
        match self {
            Self::Missing(_) => Severity::NeedsFixing,
            Self::CannotRead(_) => Severity::CannotRead,
            Self::NotSaved(_) | Self::NoteNotSaved => Severity::NotSaved,
            Self::FilesCame | Self::FilesChanged | Self::FilesGone => Severity::NeedsFixing,
        }
    }
    fn cause(&self) -> Sentence {
        match self {
            Self::Missing(folder) => crate::sentence!(
                "Splat cannot find the session folder {}. It may have been moved or renamed.",
                folder.display()
            ),
            Self::CannotRead(words) | Self::NotSaved(words) => notice_sentence(words),
            Self::NoteNotSaved => crate::sentence!(
                "Not changed. Your note could not be saved. Choose what to do with it first."
            ),
            Self::FilesCame => crate::sentence!(
                "Not changed. The current folder now keeps session files. Choose to move them or not."
            ),
            Self::FilesChanged => crate::sentence!(
                "Not changed. The files in the current folder changed. Check the new counts, then choose again."
            ),
            Self::FilesGone => crate::sentence!(
                "Not changed. The current folder has no files now. Use Save folder."
            ),
        }
    }
}

/// The page's notice slot holds its words as a sentence; an error of the system comes as its
/// own text and is made a sentence here, the one place the page takes words from outside.
#[allow(
    clippy::disallowed_methods,
    reason = "an error of the system or of a file keeps its own words"
)]
pub fn notice_sentence(words: &str) -> Sentence {
    Sentence::outside(words).unwrap_or(crate::sentence!("Something went wrong."))
}

/// Every name in a note's `[[links]]`, in order, without timeline references. A name linked
/// twice comes twice: a plan can hold the same fight again.
pub fn each_linked_name(text: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("[[") {
        rest = &rest[start + 2..];
        let Some(end) = rest.find("]]") else {
            break;
        };
        let inner = &rest[..end];
        let name = inner.split('|').next().unwrap_or(inner).trim();
        if !name.is_empty() && !name.starts_with("Timeline#") {
            names.push(name.to_owned());
        }
        rest = &rest[end + 2..];
    }
    names
}

/// The names in a note's `[[links]]`, once each.
pub fn linked_names(text: &str) -> Vec<String> {
    let mut names = each_linked_name(text);
    let mut seen = std::collections::BTreeSet::new();
    names.retain(|name| seen.insert(name.clone()));
    names
}
