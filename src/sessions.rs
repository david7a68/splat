//! Session notes and templates, as plain Markdown in the campaign's own folder, so that
//! Obsidian can read and edit them too. A note's frontmatter avoids the `splat_` prefix on
//! purpose: the catalogue treats any such note as a Library entry.
pub mod page;

use crate::{
    content_workspace::{CampaignDocument, patch_fields},
    library::write_note,
    persistence::Campaign,
};
use serde_yaml_ng::Value;
use splat::content::Entry;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use uuid::Uuid;

const FOLDER_FIELD: &str = "splat_session_storage";
pub const DEFAULT_FOLDER: &str = "Sessions";
const TEMPLATES: &str = "Templates";
const RECORDINGS: &str = "Recordings";
const SAVE_AFTER: Duration = Duration::from_secs(2);

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

fn resolve(campaign: &Campaign, name: &str) -> Result<PathBuf, String> {
    let name = name.trim();
    if name.is_empty() {
        return Err("Enter a session folder.".into());
    }
    let path = Path::new(name);
    if path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err("Session folders cannot contain '..'.".into());
    }
    Ok(if path.is_absolute() {
        path.into()
    } else {
        campaign.path.join(path)
    })
}

pub fn folder(campaign: &Campaign) -> Result<PathBuf, String> {
    resolve(campaign, &folder_setting(campaign)?)
}

pub fn save_folder(campaign: &Campaign, name: &str) -> Result<PathBuf, String> {
    let path = resolve(campaign, name)?;
    let document = CampaignDocument::load(campaign)?;
    let mut map = serde_yaml_ng::Mapping::new();
    map.insert("directory".into(), Value::String(name.trim().to_owned()));
    let text = patch_fields(&document.text, &[(FOLDER_FIELD, Some(Value::Mapping(map)))])?;
    write_note(
        &campaign.path,
        &document.path,
        document.baseline.as_deref(),
        &text,
    )?;
    Ok(path)
}

/// The frontmatter block with its fences, and the body after it.
fn split_note(text: &str) -> Option<(&str, &str)> {
    let start = text.strip_prefix('\u{feff}').unwrap_or(text);
    let newline = if start.starts_with("---\r\n") {
        "\r\n"
    } else if start.starts_with("---\n") {
        "\n"
    } else {
        return None;
    };
    let offset = text.len() - start.len();
    let close = format!("{newline}---");
    let mut from = offset + 3 + newline.len();
    loop {
        let found = text[from..].find(&close)? + from;
        let after = found + close.len();
        let rest = &text[after..];
        if rest.is_empty() {
            return Some((&text[..after], ""));
        }
        if let Some(body) = rest.strip_prefix(newline) {
            return Some((&text[..after + newline.len()], body));
        }
        from = after;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Note {
    pub id: Uuid,
    pub number: u32,
    pub title: String,
    pub date: String,
    pub path: PathBuf,
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

fn parse_note(path: &Path, text: &str) -> Option<Note> {
    let (front, _) = split_note(text)?;
    let inner = front
        .trim_start_matches('\u{feff}')
        .trim_end()
        .strip_suffix("---")?
        .trim_start_matches("---");
    let value: Value = serde_yaml_ng::from_str(inner).ok()?;
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
    })
}

/// Every note in the folder, newest first. Other Markdown files there are not sessions.
pub fn list_notes(folder: &Path) -> Vec<Note> {
    let mut notes: Vec<Note> = fs::read_dir(folder)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|file| file.path().extension().is_some_and(|e| e == "md"))
        .filter_map(|file| {
            let path = file.path();
            parse_note(&path, &fs::read_to_string(&path).ok()?)
        })
        .collect();
    notes.sort_by(|a, b| {
        b.number
            .cmp(&a.number)
            .then_with(|| b.date.cmp(&a.date))
            .then_with(|| a.path.cmp(&b.path))
    });
    notes
}

/// What a file name cannot hold, in any common file system.
fn file_safe(text: &str) -> String {
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

fn free_path(folder: &Path, stem: &str, extension: &str) -> PathBuf {
    let first = folder.join(format!("{stem}.{extension}"));
    if !first.exists() {
        return first;
    }
    (2..)
        .map(|n| folder.join(format!("{stem} ({n}).{extension}")))
        .find(|path| !path.exists())
        .expect("a free name")
}

/// What a new note needs, before its template is filled.
pub struct NewNote<'a> {
    pub number: u32,
    pub title: &'a str,
    pub date: &'a str,
    pub campaign: &'a str,
    pub template: &'a str,
}

/// Fill a template's placeholders. An unknown one stays, in case it belongs to Obsidian.
pub fn fill_template(new: &NewNote<'_>) -> String {
    new.template
        .replace("{{number}}", &new.number.to_string())
        .replace("{{title}}", new.title.trim())
        .replace("{{date}}", new.date)
        .replace("{{campaign}}", new.campaign)
}

pub fn create_note(folder: &Path, new: &NewNote<'_>) -> Result<Note, String> {
    fs::create_dir_all(folder).map_err(|e| format!("Could not create the folder: {e}"))?;
    let id = crate::hermetic::new_uuid();
    let mut stem = format!("Session {}", new.number);
    let title = file_safe(new.title);
    if !title.is_empty() {
        stem.push_str(" - ");
        stem.push_str(&title);
    }
    let path = free_path(folder, &stem, "md");
    let mut front = serde_yaml_ng::Mapping::new();
    front.insert("session_id".into(), id.to_string().into());
    front.insert("session_number".into(), u64::from(new.number).into());
    front.insert("session_title".into(), new.title.trim().to_owned().into());
    front.insert("session_date".into(), new.date.to_owned().into());
    let front = serde_yaml_ng::to_string(&front).map_err(|e| e.to_string())?;
    let text = format!("---\n{front}---\n{}", fill_template(new));
    write_new(&path, &text)?;
    parse_note(&path, &text).ok_or_else(|| "The new note could not be read back.".to_owned())
}

fn write_new(path: &Path, text: &str) -> Result<(), String> {
    let parent = path.parent().ok_or("Invalid location.")?;
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    temp.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
    temp.persist_noclobber(path).map_err(|e| e.to_string())?;
    Ok(())
}

/// Change a note's title or date, and leave any other property the DM added.
pub fn update_details(note: &Note, title: &str, date: &str, number: u32) -> Result<Note, String> {
    let text = fs::read_to_string(&note.path).map_err(|e| e.to_string())?;
    let patched = patch_fields(
        &text,
        &[
            (
                "session_title",
                Some(Value::String(title.trim().to_owned())),
            ),
            ("session_date", Some(Value::String(date.trim().to_owned()))),
            (
                "session_number",
                Some(Value::Number(u64::from(number).into())),
            ),
        ],
    )?;
    replace_file(&note.path, &patched)?;
    parse_note(&note.path, &patched).ok_or_else(|| "The note could not be read back.".to_owned())
}

fn replace_file(path: &Path, text: &str) -> Result<(), String> {
    let parent = path.parent().ok_or("Invalid location.")?;
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    temp.write_all(text.as_bytes())
        .and_then(|()| temp.as_file().sync_all())
        .map_err(|e| e.to_string())?;
    temp.persist(path).map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
pub struct Template {
    pub name: String,
    /// None for a built-in template, which is read-only until customized.
    pub path: Option<PathBuf>,
    pub body: String,
}

pub const BUILT_IN_TEMPLATES: [(&str, &str); 2] = [
    (
        "Standard session",
        "# Session {{number}}: {{title}}\n\n{{date}}, {{campaign}}\n\n## Recap of last time\n\n\n\n## Plan for tonight\n\n\n\n## What happened\n\n\n\n## People and places\n\n\n\n## Loot and rewards\n\n\n\n## Open threads\n\n",
    ),
    (
        "Short notes",
        "# Session {{number}}: {{title}}\n\n{{date}}\n\n## What happened\n\n\n\n## Next time\n\n",
    ),
];

/// The campaign's own templates, by name, then the built-in ones it has not replaced.
pub fn list_templates(folder: &Path) -> Vec<Template> {
    let mut own: Vec<Template> = fs::read_dir(folder.join(TEMPLATES))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|file| file.path().extension().is_some_and(|e| e == "md"))
        .filter_map(|file| {
            let path = file.path();
            Some(Template {
                name: path.file_stem()?.to_string_lossy().into_owned(),
                body: fs::read_to_string(&path).ok()?,
                path: Some(path),
            })
        })
        .collect();
    own.sort_by_key(|template| template.name.to_lowercase());
    for (name, body) in BUILT_IN_TEMPLATES {
        if !own.iter().any(|template| template.name == name) {
            own.push(Template {
                name: name.to_owned(),
                path: None,
                body: body.to_owned(),
            });
        }
    }
    own
}

pub fn create_template(folder: &Path, name: &str, body: &str) -> Result<Template, String> {
    let name = file_safe(name);
    if name.is_empty() {
        return Err("Enter a name for the template.".into());
    }
    let templates = folder.join(TEMPLATES);
    fs::create_dir_all(&templates).map_err(|e| format!("Could not create the folder: {e}"))?;
    let path = templates.join(format!("{name}.md"));
    if path.exists() {
        return Err(format!("A template named “{name}” already exists."));
    }
    write_new(&path, body)?;
    Ok(Template {
        name,
        path: Some(path),
        body: body.to_owned(),
    })
}

/// One recording of a session, and what later processing has made from it.
#[derive(Clone, Debug, PartialEq)]
pub struct Recording {
    pub path: PathBuf,
    pub name: String,
    pub seconds: Option<u64>,
    pub bytes: u64,
    pub transcript: bool,
    pub summary: bool,
    pub marked: bool,
    pub review: bool,
    /// The events an LLM marked. The DM chooses which go on the timeline.
    pub markers: Vec<crate::timeline::Marker>,
}

pub fn recordings_folder(folder: &Path, session: Uuid) -> PathBuf {
    folder.join(RECORDINGS).join(session.to_string())
}

/// The newest first. The file name begins with the time it started.
pub fn list_recordings(folder: &Path, session: Uuid) -> Vec<Recording> {
    let mut recordings: Vec<Recording> = fs::read_dir(recordings_folder(folder, session))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|file| file.path().extension().is_some_and(|e| e == "wav"))
        .map(|file| {
            let path = file.path();
            Recording {
                name: path
                    .file_stem()
                    .map(|stem| stem.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                seconds: crate::recording::Layout::read(&path)
                    .ok()
                    .map(|l| l.seconds()),
                bytes: file.metadata().map_or(0, |metadata| metadata.len()),
                transcript: crate::processing::transcript_path(&path).exists(),
                summary: crate::processing::summary_path(&path).exists(),
                marked: crate::processing::marked_path(&path).exists(),
                review: crate::processing::review_path(&path).exists(),
                markers: [
                    crate::processing::marked_path(&path),
                    crate::processing::summary_path(&path),
                ]
                .iter()
                .filter_map(|made| fs::read_to_string(made).ok())
                .flat_map(|text| crate::timeline::parse_markers(&text))
                .collect(),
                path,
            }
        })
        .collect();
    recordings.sort_by(|a, b| b.name.cmp(&a.name));
    recordings
}

/// A new recording's path. The DM reads this name, so it is the local date and time.
pub fn new_recording_path(folder: &Path, session: Uuid) -> PathBuf {
    let directory = recordings_folder(folder, session);
    free_path(&directory, &crate::hermetic::local_stamp(), "wav")
}

/// Remove a recording and what was made from it.
pub fn delete_recording(path: &Path) -> Result<(), String> {
    fs::remove_file(path).map_err(|e| e.to_string())?;
    for made in [
        crate::processing::transcript_path(path),
        crate::processing::summary_path(path),
        crate::processing::marked_path(path),
        crate::processing::cleaned_path(path),
        crate::processing::review_path(path),
    ] {
        match fs::remove_file(made) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(())
}

/// A file's frontmatter removed, for reading. Text with none is returned as it is.
pub fn body_of(text: &str) -> &str {
    split_note(text).map_or(text, |(_, body)| body)
}

#[derive(Clone, Debug, PartialEq)]
pub enum DraftState {
    Clean,
    Unsaved,
    Saved,
    /// The file changed outside Splat during an edit. Saving stops until the DM chooses.
    Conflict,
    Failed(String),
}

/// One file being edited. The prefix (frontmatter) stays as the file has it. Edits save
/// shortly after they stop, and a change made elsewhere is never overwritten silently.
pub struct Draft {
    pub path: PathBuf,
    prefix: String,
    pub body: String,
    baseline: Option<String>,
    pub state: DraftState,
    last_edit: Option<Instant>,
}

impl Draft {
    pub fn open(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let (prefix, body) = split_note(&text).unwrap_or(("", &text));
        Ok(Self {
            path: path.to_owned(),
            prefix: prefix.to_owned(),
            body: body.to_owned(),
            baseline: Some(text.clone()),
            state: DraftState::Clean,
            last_edit: None,
        })
    }

    pub fn edited(&mut self) {
        self.state = DraftState::Unsaved;
        self.last_edit = Some(Instant::now());
    }

    /// Does this draft hold something the file does not?
    pub fn unsaved(&self) -> bool {
        matches!(
            self.state,
            DraftState::Unsaved | DraftState::Conflict | DraftState::Failed(_)
        )
    }

    pub fn needs_attention(&self) -> bool {
        matches!(self.state, DraftState::Conflict | DraftState::Failed(_))
    }

    fn due_at(&self, now: Instant) -> bool {
        self.state == DraftState::Unsaved
            && self
                .last_edit
                .is_none_or(|at| now.duration_since(at) >= SAVE_AFTER)
    }

    pub fn save(&mut self) {
        if !matches!(self.state, DraftState::Unsaved | DraftState::Failed(_)) {
            return;
        }
        let current = fs::read_to_string(&self.path).ok();
        if current != self.baseline {
            self.state = DraftState::Conflict;
            return;
        }
        self.write();
    }

    fn write(&mut self) {
        let text = format!("{}{}", self.prefix, self.body);
        match replace_file(&self.path, &text) {
            Ok(()) => {
                self.baseline = Some(text);
                self.state = DraftState::Saved;
            }
            Err(error) => {
                self.state = DraftState::Failed(format!("Not saved. {error}"));
            }
        }
    }

    /// The DM keeps this version over the one on disk.
    pub fn overwrite(&mut self) {
        // The frontmatter on disk can be newer, and the DM chose only about the words.
        if let Ok(current) = fs::read_to_string(&self.path)
            && let Some((prefix, _)) = split_note(&current)
        {
            self.prefix = prefix.to_owned();
        }
        self.write();
    }

    /// Back to the file, and the draft's words are gone.
    pub fn reload(&mut self) -> Result<(), String> {
        *self = Self::open(&self.path)?;
        Ok(())
    }

    /// Follow a change made elsewhere when there is no edit to lose.
    pub fn follow_disk(&mut self) {
        if self.unsaved() {
            return;
        }
        let current = fs::read_to_string(&self.path).ok();
        if current != self.baseline && current.is_some() {
            let state = self.state.clone();
            if self.reload().is_ok() {
                self.state = if state == DraftState::Saved {
                    DraftState::Clean
                } else {
                    state
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_note<'a>(number: u32, title: &'a str, template: &'a str) -> NewNote<'a> {
        NewNote {
            number,
            title,
            date: "2026-11-01",
            campaign: "The Marches",
            template,
        }
    }

    #[test]
    fn a_note_is_created_from_a_template_and_reads_back() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(
            dir.path(),
            &new_note(
                12,
                "The Bridge: at dusk?",
                "# {{number}} {{title}}\n{{date}} {{campaign}} {{unknown}}\n",
            ),
        )
        .unwrap();
        assert_eq!(note.number, 12);
        assert_eq!(note.title, "The Bridge: at dusk?");
        assert_eq!(note.link(), "Session 12 - The Bridge at dusk");
        let text = fs::read_to_string(&note.path).unwrap();
        assert!(
            text.ends_with("# 12 The Bridge: at dusk?\n2026-11-01 The Marches {{unknown}}\n"),
            "{text}"
        );
        assert_eq!(list_notes(dir.path()), [note]);
    }

    #[test]
    fn a_note_never_looks_like_a_library_entry_to_the_catalogue() {
        let dir = tempfile::tempdir().unwrap();
        create_note(dir.path(), &new_note(1, "", "Body")).unwrap();
        let (entries, errors) = splat::content::read_directory(dir.path()).unwrap();
        assert!(entries.is_empty() && errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn notes_list_newest_first_and_leave_other_markdown_out() {
        let dir = tempfile::tempdir().unwrap();
        create_note(dir.path(), &new_note(2, "", "")).unwrap();
        create_note(dir.path(), &new_note(10, "", "")).unwrap();
        fs::write(
            dir.path().join("Ideas.md"),
            "---\ntag: x\n---\nNot a session",
        )
        .unwrap();
        fs::write(dir.path().join("Plain.md"), "No frontmatter").unwrap();
        let numbers: Vec<u32> = list_notes(dir.path()).iter().map(|n| n.number).collect();
        assert_eq!(numbers, [10, 2]);
    }

    #[test]
    fn two_notes_with_one_name_both_survive() {
        let dir = tempfile::tempdir().unwrap();
        let first = create_note(dir.path(), &new_note(3, "", "A")).unwrap();
        let second = create_note(dir.path(), &new_note(3, "", "B")).unwrap();
        assert_ne!(first.path, second.path);
        assert_eq!(list_notes(dir.path()).len(), 2);
    }

    #[test]
    fn details_change_and_the_dms_own_properties_and_words_stay() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Session 1.md");
        let id = Uuid::from_u128(5);
        fs::write(
            &path,
            format!("---\nsession_id: {id}\nsession_number: 1\nsession_title: Old\nsession_date: 2026-01-01\ntags: [dnd]\n---\n\nMy words\n"),
        )
        .unwrap();
        let note = parse_note(&path, &fs::read_to_string(&path).unwrap()).unwrap();
        let note = update_details(&note, " New ", "2026-02-02", 4).unwrap();
        assert_eq!(
            (note.number, note.title.as_str(), note.date.as_str()),
            (4, "New", "2026-02-02")
        );
        let text = fs::read_to_string(&path).unwrap();
        assert!(
            text.contains("tags: [dnd]") && text.ends_with("My words\n"),
            "{text}"
        );
    }

    #[test]
    fn a_draft_saves_only_the_body_after_its_pause() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(1, "", "Start\n")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        assert_eq!(draft.body, "Start\n");
        assert_eq!(draft.state, DraftState::Clean);
        draft.body.push_str("More\n");
        draft.edited();
        assert!(draft.unsaved());
        let edit = draft.last_edit.unwrap();
        assert!(!draft.due_at(edit));
        assert!(draft.due_at(edit + SAVE_AFTER));
        draft.save();
        assert_eq!(draft.state, DraftState::Saved);
        let text = fs::read_to_string(&note.path).unwrap();
        assert!(
            text.starts_with("---\nsession_id:") && text.ends_with("\nStart\nMore\n"),
            "{text}"
        );
    }

    #[test]
    fn a_draft_stops_at_a_change_made_elsewhere_and_the_dm_chooses() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(1, "", "Start\n")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        let outside = fs::read_to_string(&note.path)
            .unwrap()
            .replace("Start", "From Obsidian");
        fs::write(&note.path, &outside).unwrap();

        draft.body = "Mine\n".into();
        draft.edited();
        draft.save();
        assert_eq!(draft.state, DraftState::Conflict);
        assert_eq!(fs::read_to_string(&note.path).unwrap(), outside);
        // Saving again changes nothing until the DM chooses.
        draft.save();
        assert_eq!(fs::read_to_string(&note.path).unwrap(), outside);

        draft.overwrite();
        assert_eq!(draft.state, DraftState::Saved);
        assert!(fs::read_to_string(&note.path).unwrap().ends_with("Mine\n"));

        fs::write(&note.path, &outside).unwrap();
        draft.body = "Lost\n".into();
        draft.edited();
        draft.save();
        draft.reload().unwrap();
        assert_eq!(draft.state, DraftState::Clean);
        assert!(draft.body.contains("From Obsidian"));
    }

    #[test]
    fn a_clean_draft_follows_the_disk_and_an_edit_does_not() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(1, "", "Start\n")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        let outside = fs::read_to_string(&note.path)
            .unwrap()
            .replace("Start", "Elsewhere");
        fs::write(&note.path, &outside).unwrap();
        draft.follow_disk();
        assert!(draft.body.contains("Elsewhere"));
        draft.body = "Mine".into();
        draft.edited();
        fs::write(&note.path, "---\nsession_id: x\n---\nAgain").unwrap();
        draft.follow_disk();
        assert_eq!(draft.body, "Mine");
    }

    #[test]
    fn crlf_notes_keep_their_frontmatter() {
        let text = "---\r\nsession_id: 00000000-0000-0000-0000-000000000005\r\n---\r\nBody\r\n";
        let (front, body) = split_note(text).unwrap();
        assert!(front.ends_with("---\r\n"));
        assert_eq!(body, "Body\r\n");
        assert_eq!(
            parse_note(Path::new("a.md"), text).unwrap().id,
            Uuid::from_u128(5)
        );
    }

    #[test]
    fn a_rule_inside_the_frontmatter_value_does_not_end_it() {
        let (front, body) = split_note("---\na: 1\n---not the end\n---\nBody").unwrap();
        assert_eq!(front, "---\na: 1\n---not the end\n---\n");
        assert_eq!(body, "Body");
        assert_eq!(body_of("no frontmatter"), "no frontmatter");
    }

    #[test]
    fn templates_list_the_campaigns_own_first_and_replace_a_built_in_of_the_same_name() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(list_templates(dir.path()).len(), BUILT_IN_TEMPLATES.len());
        create_template(dir.path(), "Downtime", "Between adventures").unwrap();
        create_template(dir.path(), "Short notes", "My version").unwrap();
        let templates = list_templates(dir.path());
        let names: Vec<&str> = templates.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(names, ["Downtime", "Short notes", "Standard session"]);
        assert_eq!(templates[1].body, "My version");
        assert!(templates[2].path.is_none());
        let error = create_template(dir.path(), "Downtime", "Again").unwrap_err();
        assert!(error.contains("already exists"));
        assert!(create_template(dir.path(), "///", "x").is_err());
    }

    #[test]
    fn the_folder_comes_from_the_manifest_and_stays_inside_the_rules() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::from_u128(9),
            name: "Marches".into(),
            path: dir.path().to_owned(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        assert_eq!(folder(&campaign).unwrap(), dir.path().join(DEFAULT_FOLDER));
        assert_eq!(
            save_folder(&campaign, "D20.42.22 Sessions").unwrap(),
            dir.path().join("D20.42.22 Sessions")
        );
        assert_eq!(
            folder(&campaign).unwrap(),
            dir.path().join("D20.42.22 Sessions")
        );
        assert!(save_folder(&campaign, "../outside").is_err());
        assert!(save_folder(&campaign, "  ").is_err());
        assert_eq!(folder_setting(&campaign).unwrap(), "D20.42.22 Sessions");
    }

    #[test]
    fn recordings_list_newest_first_with_what_was_made_from_them() {
        let dir = tempfile::tempdir().unwrap();
        let session = Uuid::from_u128(3);
        let folder = recordings_folder(dir.path(), session);
        fs::create_dir_all(&folder).unwrap();
        let wav = crate::recording::wav_bytes(&[0; 32_000]);
        fs::write(folder.join("2026-11-01 19-00.wav"), &wav).unwrap();
        fs::write(folder.join("2026-11-01 21-00.wav"), &wav).unwrap();
        fs::write(folder.join("2026-11-01 21-00.transcript.md"), "x").unwrap();
        let list = list_recordings(dir.path(), session);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].name, "2026-11-01 21-00");
        assert_eq!(list[0].seconds, Some(2));
        assert!(list[0].transcript && !list[0].summary);
        assert!(!list[1].transcript);

        delete_recording(&list[0].path).unwrap();
        assert!(!folder.join("2026-11-01 21-00.transcript.md").exists());
        assert_eq!(list_recordings(dir.path(), session).len(), 1);
        assert!(list_recordings(dir.path(), Uuid::from_u128(4)).is_empty());
    }
}
