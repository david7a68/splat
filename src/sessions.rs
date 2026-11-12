//! Session notes and templates, as plain Markdown in the campaign's own folder, so that
//! Obsidian can read and edit them too. A note's frontmatter avoids the `splat_` prefix on
//! purpose: the catalogue treats any such note as a Library entry.
pub mod page;
mod slash;

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
pub(crate) const TEMPLATES: &str = "Templates";
pub(crate) const RECORDINGS: &str = "Recordings";
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
        return Err("Session folders cannot contain “..”.".into());
    }
    Ok(if path.is_absolute() {
        path.into()
    } else {
        campaign.path.join(path)
    })
}

/// One wording for a folder that is gone, whatever it holds (`Session`, `Run`).
pub(crate) fn missing_folder(what: &str, folder: &Path) -> String {
    format!(
        "{what} folder not found: {}. It may have been moved or renamed.",
        folder.display()
    )
}

pub fn folder(campaign: &Campaign) -> Result<PathBuf, String> {
    let path = resolve(campaign, &folder_setting(campaign)?)?;
    let document = CampaignDocument::load(campaign)?;
    let entry = Entry::parse(&document.path, &document.text)?;
    if entry.fields.contains_key(FOLDER_FIELD) && !path.is_dir() {
        return Err(missing_folder("Session", &path));
    }
    Ok(path)
}

pub(crate) fn open_folder(campaign: &Campaign) -> Result<PathBuf, String> {
    let document = CampaignDocument::load(campaign)?;
    let entry = Entry::parse(&document.path, &document.text)?;
    if !entry.fields.contains_key(FOLDER_FIELD) {
        write_folder(campaign, &folder_setting(campaign)?)
    } else {
        folder(campaign)
    }
}

pub fn save_folder(campaign: &Campaign, name: &str) -> Result<PathBuf, String> {
    write_folder(campaign, name)
}
fn write_folder(campaign: &Campaign, name: &str) -> Result<PathBuf, String> {
    let path = resolve(campaign, name)?;
    fs::create_dir_all(&path).map_err(|e| format!("Could not create session folder: {e}"))?;
    let document = CampaignDocument::load(campaign)?;
    let mut map = serde_yaml_ng::Mapping::new();
    map.insert("directory".into(), Value::String(name.trim().to_owned()));
    let text = splat::content::formats::patch_model(
        &document.text,
        &[(FOLDER_FIELD, Some(Value::Mapping(map)))],
    )?;
    write_note(
        &campaign.path,
        &document.path,
        document.baseline.as_deref(),
        &text,
    )?;
    Ok(path)
}

/// The frontmatter block with its fences, and the body after it.
pub(crate) fn split_note(text: &str) -> Option<(&str, &str)> {
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

fn note_paths(folder: &Path) -> Vec<PathBuf> {
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

const NOTE_PATHS: &str = ".splat-session-paths.json";

pub(crate) struct IdentityUndo {
    path: PathBuf,
    before: String,
    after: String,
}

pub(crate) fn new_identity(path: &Path) -> Result<IdentityUndo, String> {
    let before = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let after = patch_fields(
        &before,
        &[(
            "session_id",
            Some(crate::hermetic::new_uuid().to_string().into()),
        )],
    )?;
    if fs::read_to_string(path).ok().as_deref() != Some(before.as_str()) {
        return Err("This session changed on disk. Reload before trying again.".into());
    }
    replace_file(path, &after)?;
    Ok(IdentityUndo {
        path: path.into(),
        before,
        after,
    })
}

impl IdentityUndo {
    pub fn undo(&self) -> Result<(), String> {
        if fs::read_to_string(&self.path).ok().as_deref() != Some(self.after.as_str()) {
            return Err("This session changed on disk. Undo did not replace it.".into());
        }
        replace_file(&self.path, &self.before)
    }
}

fn known_notes(folder: &Path) -> std::collections::BTreeMap<Uuid, PathBuf> {
    fs::read(folder.join(NOTE_PATHS))
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}

pub(crate) fn remember_notes(folder: &Path, notes: &[Note]) -> Result<(), String> {
    let path = folder.join(NOTE_PATHS);
    let mut known = known_notes(folder);
    for note in notes {
        known.insert(
            note.id,
            note.path
                .strip_prefix(folder)
                .map_err(|e| e.to_string())?
                .into(),
        );
    }
    let text = serde_json::to_string_pretty(&known).map_err(|e| e.to_string())?;
    replace_file(&path, &text)
}

/// Notes that claim a session but cannot be read, by file name. Their files are safe.
pub fn unreadable_notes(folder: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(folder)
        .into_iter()
        .flatten()
        .flatten()
        .map(|file| file.path())
        .filter(|path| path.extension().is_some_and(|e| e == "md"))
        .filter(|path| {
            fs::read_to_string(path).is_ok_and(|text| {
                text.lines().any(|line| line.starts_with("session_id:"))
                    && parse_note(path, &text).is_none()
            })
        })
        .filter_map(|path| Some(path.file_name()?.to_string_lossy().into_owned()))
        .collect();
    names.sort();
    names
}

/// What a file name cannot hold, in any common file system.
pub(crate) fn file_safe(text: &str) -> String {
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
    let filled = new
        .template
        .replace("{{number}}", &new.number.to_string())
        .replace("{{title}}", new.title.trim())
        .replace("{{date}}", new.date)
        .replace("{{campaign}}", new.campaign);
    // An empty title must not leave `# Session 12: ` behind.
    filled
        .split_inclusive('\n')
        .map(|line| {
            let bare = line.trim_end();
            match bare.strip_suffix(':') {
                Some(kept) if bare.starts_with('#') => {
                    format!(
                        "{kept}{}",
                        &line[line.trim_end_matches(['\n', '\r']).len()..]
                    )
                }
                _ => line.to_owned(),
            }
        })
        .collect()
}

/// The heading a template makes for a session.
fn session_heading(number: u32, title: &str) -> String {
    match title.trim() {
        "" => format!("# Session {number}"),
        title => format!("# Session {number}: {title}"),
    }
}

pub(crate) fn note_path(folder: &Path, number: u32, title: &str) -> PathBuf {
    let mut stem = format!("Session {number}");
    let title = file_safe(title);
    if !title.is_empty() {
        stem.push_str(" - ");
        stem.push_str(&title);
    }
    free_path(folder, &stem, "md")
}

/// Files owned by session storage. Other campaign notes stay where they are.
pub(crate) fn storage_files(folder: &Path) -> Result<Vec<PathBuf>, String> {
    fn files(root: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
        if !root.exists() {
            return Ok(());
        }
        for item in fs::read_dir(root).map_err(|error| error.to_string())? {
            let item = item.map_err(|error| error.to_string())?;
            let path = item.path();
            if item
                .file_type()
                .map_err(|error| error.to_string())?
                .is_symlink()
            {
                return Err(format!(
                    "{} is a link to another file or folder. Move it by hand first.",
                    path.display()
                ));
            }
            if path.is_dir() {
                files(&path, out)?;
            } else {
                out.push(path);
            }
        }
        Ok(())
    }
    let mut out: Vec<_> = list_notes(folder)
        .into_iter()
        .flat_map(|note| std::iter::once(note.path).chain(note.also))
        .collect();
    if folder.join("Timeline.md").exists() {
        out.push(folder.join("Timeline.md"));
    }
    if folder.join(NOTE_PATHS).exists() {
        out.push(folder.join(NOTE_PATHS));
    }
    for file in fs::read_dir(folder).map_err(|e| e.to_string())? {
        let path = file.map_err(|e| e.to_string())?.path();
        if path.extension().is_some_and(|ext| ext == "md") && !out.contains(&path) {
            let text = fs::read_to_string(&path)
                .map_err(|e| format!("{}: {e}. Move this file by hand first.", path.display()))?;
            if text.lines().any(|line| {
                line.starts_with("session_id:") || line.starts_with("timeline_version:")
            }) {
                out.push(path);
            }
        }
    }
    files(&folder.join(TEMPLATES), &mut out)?;
    files(&folder.join(RECORDINGS), &mut out)?;
    Ok(out)
}

/// Copy before changing the manifest; retain the old files if any step fails.
pub(crate) fn move_storage(campaign: &Campaign, name: &str) -> Result<PathBuf, String> {
    let old = folder(campaign)?;
    let new = resolve(campaign, name)?;
    if old == new {
        return save_folder(campaign, name);
    }
    let files = storage_files(&old)?;
    let pairs: Vec<_> = files
        .iter()
        .map(|path| {
            (
                path,
                new.join(path.strip_prefix(&old).expect("storage path")),
            )
        })
        .collect();
    if let Some((_, target)) = pairs.iter().find(|(_, target)| target.exists()) {
        return Err(format!(
            "Not moved. {} already exists. Choose another folder.",
            target.display()
        ));
    }
    let mut copied = Vec::new();
    let result = (|| {
        for (source, target) in &pairs {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).map_err(|error| error.to_string())?;
            }
            transfer_file(source, target, false, || {})?;
            copied.push(target.clone());
        }
        save_folder(campaign, name)
    })();
    if let Err(error) = result {
        let mut errors = vec![error];
        for target in copied.into_iter().rev() {
            let source = old.join(target.strip_prefix(&new).expect("storage target"));
            if let Err(error) = transfer_file(&target, &source, false, || {}) {
                errors.push(error);
            }
        }
        return Err(errors.join(" "));
    }
    Ok(new)
}

/// Moves `source` to `target` in one step, and never replaces a file at `target`: Linux's
/// `RENAME_NOREPLACE` refuses with `AlreadyExists`, even for a file that arrives at the last
/// moment. Moves across file systems fail with `EXDEV`.
#[cfg(target_os = "linux")]
pub(crate) fn rename_no_replace(source: &Path, target: &Path) -> std::io::Result<()> {
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

fn transfer_file(
    source: &Path,
    target: &Path,
    force_copy: bool,
    before_delete: impl FnOnce(),
) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    if !force_copy {
        // RENAME_NOREPLACE protects an arriving sync file.
        match rename_no_replace(source, target) {
            Ok(()) => {
                sync_folder(source.parent().ok_or("Missing source folder")?)?;
                sync_folder(target.parent().ok_or("Missing target folder")?)?;
                return Ok(());
            }
            Err(error) if error.raw_os_error() != Some(libc::EXDEV) => {
                return Err(format!("Could not move {}: {error}", source.display()));
            }
            Err(_) => {}
        }
    }
    let bytes = fs::read(source).map_err(|e| format!("{}: {e}", source.display()))?;
    let mut output =
        tempfile::NamedTempFile::new_in(target.parent().ok_or("Missing target folder")?)
            .map_err(|e| e.to_string())?;
    output
        .write_all(&bytes)
        .and_then(|()| output.as_file().sync_all())
        .map_err(|e| e.to_string())?;
    output
        .persist_noclobber(target)
        .map_err(|e| e.to_string())?;
    sync_folder(target.parent().ok_or("Missing target folder")?)?;
    before_delete();
    if fs::read(source).ok().as_deref() != Some(bytes.as_slice()) {
        return Err(format!(
            "{} changed during the move. Both files were kept.",
            source.display()
        ));
    }
    fs::remove_file(source).map_err(|e| e.to_string())?;
    sync_folder(source.parent().ok_or("Missing source folder")?)?;
    sync_folder(target.parent().ok_or("Missing target folder")?)
}

fn require_folder(folder: &Path) -> Result<(), String> {
    if folder.is_dir() {
        Ok(())
    } else {
        Err(format!(
            "Session folder is gone: {}. Choose folder before saving.",
            folder.display()
        ))
    }
}

pub fn create_note(folder: &Path, new: &NewNote<'_>) -> Result<Note, String> {
    require_folder(folder)?;
    let id = crate::hermetic::new_uuid();
    let path = note_path(folder, new.number, new.title);
    let mut front = serde_yaml_ng::Mapping::new();
    front.insert("session_id".into(), id.to_string().into());
    front.insert("session_number".into(), u64::from(new.number).into());
    if !new.title.trim().is_empty() {
        front.insert("session_title".into(), new.title.trim().to_owned().into());
    }
    front.insert("session_date".into(), new.date.to_owned().into());
    let front = serde_yaml_ng::to_string(&front).map_err(|e| e.to_string())?;
    let text = format!("---\n{front}---\n{}", fill_template(new));
    write_new(&path, &text)?;
    let note = parse_note(&path, &text).ok_or("The new note could not be read back.")?;
    remember_notes(folder, std::slice::from_ref(&note))?;
    Ok(note)
}

fn write_new(path: &Path, text: &str) -> Result<(), String> {
    let parent = path.parent().ok_or("Invalid location.")?;
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    temp.write_all(text.as_bytes()).map_err(|e| e.to_string())?;
    temp.as_file().sync_all().map_err(|e| e.to_string())?;
    temp.persist_noclobber(path).map_err(|e| e.to_string())?;
    sync_folder(parent)?;
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
                (!title.trim().is_empty()).then(|| Value::String(title.trim().to_owned())),
            ),
            ("session_date", Some(Value::String(date.trim().to_owned()))),
            (
                "session_number",
                Some(Value::Number(u64::from(number).into())),
            ),
        ],
    )?;
    // A heading the DM has not changed follows the new name.
    let old = session_heading(note.number, &note.title);
    let new = session_heading(number, title);
    let patched = match split_note(&patched) {
        Some((front, body))
            if body.lines().find(|line| !line.trim().is_empty()) == Some(old.as_str()) =>
        {
            format!("{front}{}", body.replacen(&old, &new, 1))
        }
        _ => patched,
    };
    // So does a date line near the top, as a template writes it.
    let patched = match split_note(&patched) {
        Some((front, body)) if !note.date.is_empty() => {
            let head: usize = body.split_inclusive('\n').take(4).map(str::len).sum();
            let (top, rest) = body.split_at(head);
            let moved: String = top
                .split_inclusive('\n')
                .map(|line| {
                    if line.starts_with(&note.date) {
                        line.replacen(&note.date, date.trim(), 1)
                    } else {
                        line.to_owned()
                    }
                })
                .collect();
            format!("{front}{moved}{rest}")
        }
        _ => patched,
    };
    replace_file(&note.path, &patched)?;
    parse_note(&note.path, &patched).ok_or_else(|| "The note could not be read back.".to_owned())
}

/// The session id a note's text holds, read the way the folder scan reads it.
pub fn note_identity(path: &Path, text: &str) -> Option<Uuid> {
    parse_note(path, text).map(|note| note.id)
}

pub(crate) fn replace_file(path: &Path, text: &str) -> Result<(), String> {
    let parent = path.parent().ok_or("Invalid location.")?;
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    temp.write_all(text.as_bytes())
        .and_then(|()| temp.as_file().sync_all())
        .map_err(|e| e.to_string())?;
    temp.persist(path).map_err(|e| e.to_string())?;
    sync_folder(parent)?;
    Ok(())
}

pub(crate) fn sync_folder(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    fs::File::open(path)
        .and_then(|file| file.sync_all())
        .map_err(|e| e.to_string())?;
    #[cfg(not(unix))]
    let _ = path;
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
        "# Session {{number}}: {{title}}\n\n{{date}}, {{campaign}}\n\n## Recap of last time\n\n## Plan for tonight\n\n## What happened\n\n## People and places\n\n## Loot and rewards\n\n## Open threads\n\n",
    ),
    (
        "Short notes",
        "# Session {{number}}: {{title}}\n\n{{date}}\n\n## What happened\n\n## Next time\n\n",
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
    require_folder(folder)?;
    let name = file_safe(name);
    if name.is_empty() {
        return Err("Enter a name for the template.".into());
    }
    let templates = folder.join(TEMPLATES);
    match fs::create_dir(&templates) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(error) => return Err(format!("Could not create the folder: {error}")),
    }
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

/// A recording's file name as the DM reads it, with the time written as a time.
pub fn recording_title(path: &Path) -> String {
    let stem = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default();
    match stem.split_once(' ') {
        Some((date, time)) => format!("{date} {}", time.replace('-', ":")),
        None => stem,
    }
}

/// One recording of a session, and what later processing has made from it.
#[derive(Clone, Debug, PartialEq)]
pub struct Recording {
    pub path: PathBuf,
    pub name: String,
    /// The name as the DM reads it, with the time written as a time.
    pub title: String,
    pub seconds: Option<u64>,
    pub bytes: u64,
    pub transcript: bool,
    pub summary: bool,
    pub marked: bool,
    pub review: bool,
    /// What the DM must know about the result, if anything.
    pub warning: Option<String>,
    /// A retained processing failure, cancellation, or recorder stop.
    pub status: Option<String>,
    /// The events an LLM marked. The DM chooses which go on the timeline.
    pub markers: Vec<crate::timeline::Marker>,
}

impl Recording {
    pub fn check_missing(&self) -> bool {
        self.warning
            .as_deref()
            .is_some_and(|warning| warning.contains(crate::processing::NO_CHECK_WARNING))
    }
}

pub fn recordings_folder(folder: &Path, session: Uuid) -> PathBuf {
    folder.join(RECORDINGS).join(session.to_string())
}

/// The newest first. The file name begins with the time it started.
pub fn list_recordings(folder: &Path, session: Uuid) -> Vec<Recording> {
    let paths: std::collections::BTreeSet<PathBuf> =
        fs::read_dir(recordings_folder(folder, session))
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|file| {
                let path = file.path();
                let name = path.file_name()?.to_str()?;
                [
                    ".wav",
                    ".transcript.md",
                    ".summary.md",
                    ".marked.md",
                    ".cleaned.md",
                    ".review.md",
                    ".warning.md",
                    ".state.txt",
                ]
                .iter()
                .find_map(|suffix| name.strip_suffix(suffix))
                .map(|stem| path.with_file_name(format!("{stem}.wav")))
            })
            .collect();
    let mut recordings: Vec<Recording> = paths
        .into_iter()
        .map(|path| Recording {
            name: path
                .file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
                .unwrap_or_default(),
            title: recording_title(&path),
            seconds: crate::recording::Layout::read(&path)
                .ok()
                .map(|l| l.seconds()),
            bytes: fs::metadata(&path).map_or(0, |metadata| metadata.len()),
            transcript: crate::processing::transcript_path(&path).exists(),
            summary: crate::processing::summary_path(&path).exists(),
            marked: crate::processing::marked_path(&path).exists(),
            review: crate::processing::review_path(&path).exists(),
            warning: fs::read_to_string(crate::processing::warning_path(&path)).ok(),
            status: fs::read_to_string(path.with_extension("state.txt")).ok(),
            markers: [
                crate::processing::marked_path(&path),
                crate::processing::summary_path(&path),
            ]
            .iter()
            .filter_map(|made| fs::read_to_string(made).ok())
            .flat_map(|text| crate::timeline::parse_markers(&text))
            .collect(),
            path,
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
    for made in [
        crate::processing::transcript_path(path),
        crate::processing::summary_path(path),
        crate::processing::marked_path(path),
        crate::processing::cleaned_path(path),
        crate::processing::review_path(path),
        crate::processing::warning_path(path),
        path.with_extension("state.txt"),
        path.to_path_buf(),
    ] {
        match fs::remove_file(made) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(())
}

/// The heading a note's events and summaries belong under, if the note has it.
const WHAT_HAPPENED: &str = "## What happened";

/// Where a note's `What happened` section ends, if it has one.
fn what_happened_end(body: &str) -> Option<usize> {
    body.match_indices(WHAT_HAPPENED)
        .find(|(at, _)| {
            (*at == 0 || body[..*at].ends_with('\n'))
                && body[at + WHAT_HAPPENED.len()..]
                    .chars()
                    .next()
                    .is_none_or(|c| c == '\n' || c == '\r')
        })
        .map(|(at, _)| at + WHAT_HAPPENED.len())
        .map(|start| {
            body[start..]
                .find("\n## ")
                .map_or(body.len(), |next| start + next)
        })
}

/// Add words to a note where a DM would put them: at the end of its `What happened`
/// section, or at the end of the note.
pub fn insert_in_note(body: &mut String, words: &str) {
    let end = what_happened_end(body).unwrap_or(body.len());
    let kept = body[..end].trim_end().len();
    let rest = body[end..].to_owned();
    body.truncate(kept);
    if !body.is_empty() {
        body.push_str("\n\n");
    }
    body.push_str(words.trim());
    body.push('\n');
    body.push_str(&rest);
}

/// Open a fresh line at the same place, for typing during play. Returns the caret, in
/// characters, and whether the text had to change to make the line.
pub fn fresh_line_in_note(body: &mut String) -> (usize, bool) {
    let end = what_happened_end(body).unwrap_or(body.len());
    let kept = body[..end].trim_end().len();
    let blank = body[kept..].chars().take_while(|c| *c == '\n').count();
    let before = if kept == 0 { 0 } else { 2 };
    // What follows stays a line of its own, or the DM's words would join a heading.
    let after = if body[kept..].trim().is_empty() { 0 } else { 2 };
    let changed = blank < before + after;
    if changed {
        body.insert_str(kept, &"\n".repeat(before + after - blank));
    }
    (body[..kept + before].chars().count(), changed)
}

/// A text's headings, one level down, so that it sits inside a section of a note.
pub fn nested_in_note(text: &str) -> String {
    text.split_inclusive('\n')
        .map(|line| {
            if line.starts_with('#') {
                format!("#{line}")
            } else {
                line.to_owned()
            }
        })
        .collect()
}

/// A note for the Reader, without the heading and date the page already shows above it.
pub fn without_own_heading(body: &str, note: &Note) -> String {
    let heading = format!("# {}", note.label());
    let mut lines = body.split_inclusive('\n').peekable();
    while lines.peek().is_some_and(|line| line.trim().is_empty()) {
        lines.next();
    }
    if lines.peek().map(|line| line.trim_end()) != Some(heading.as_str()) {
        return body.to_owned();
    }
    lines.next();
    while lines.peek().is_some_and(|line| line.trim().is_empty()) {
        lines.next();
    }
    if !note.date.is_empty()
        && lines
            .peek()
            .is_some_and(|line| line.starts_with(&note.date))
    {
        lines.next();
    }
    lines.collect::<String>().trim_start().to_owned()
}

/// A note's headings with something under them, for reading. A template leaves headings the
/// DM never filled, and a page of empty headings hides the few words there are.
pub fn without_empty_sections(text: &str) -> String {
    let level = |line: &str| {
        let hashes = line.chars().take_while(|c| *c == '#').count();
        (hashes > 0 && line[hashes..].starts_with(' ')).then_some(hashes)
    };
    let mut lines: Vec<&str> = text.lines().collect();
    // A heading whose only subheading was empty is empty too, so this goes until nothing changes.
    loop {
        let empty = (0..lines.len()).find(|&at| {
            level(lines[at]).is_some_and(|own| {
                lines[at + 1..]
                    .iter()
                    .find(|next| !next.trim().is_empty())
                    .is_none_or(|next| level(next).is_some_and(|other| other <= own))
            })
        });
        let Some(at) = empty else {
            break;
        };
        lines.remove(at);
    }
    let mut kept = String::new();
    let mut blank = true;
    for line in lines {
        let is_blank = line.trim().is_empty();
        if !(is_blank && blank) {
            kept.push_str(line);
            kept.push('\n');
        }
        blank = is_blank;
    }
    format!("{}\n", kept.trim_end())
}

/// The words under one `## heading`, up to the next heading of that level.
pub fn section_text(body: &str, heading: &str) -> Option<String> {
    let mut lines = body.lines();
    lines.find(|line| line.trim_end().strip_prefix("## ") == Some(heading))?;
    let words = lines
        .take_while(|line| !line.starts_with("## "))
        .collect::<Vec<_>>()
        .join("\n");
    let words = words.trim();
    (!words.is_empty()).then(|| words.to_owned())
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

/// Add a line to a note's plan: under `Plan for tonight`, or where other words go.
pub fn insert_in_plan(body: &mut String, words: &str) {
    const PLAN: &str = "## Plan for tonight";
    let Some(at) = body
        .match_indices(PLAN)
        .find(|(at, _)| *at == 0 || body[..*at].ends_with('\n'))
        .map(|(at, _)| at + PLAN.len())
    else {
        insert_in_note(body, words);
        return;
    };
    let end = body[at..]
        .find("\n## ")
        .map_or(body.len(), |next| at + next);
    let kept = body[..end].trim_end().len();
    let rest = body[end..].to_owned();
    body.truncate(kept);
    body.push_str(if kept == at { "\n\n" } else { "\n" });
    body.push_str(words.trim());
    body.push('\n');
    body.push_str(&rest);
}

/// One line of a note's plan, in the note's order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlanLine {
    /// A task line that starts at the line's start: `- [ ] words` or `- [x] words`.
    Task(TaskLine),
    /// Any other line, by the names it links.
    Links(Vec<String>),
}

/// A task line of the plan. It is a scene when it links no fight and no group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskLine {
    /// The line's number in the note's words, so that an act finds it nearest this place.
    pub at: usize,
    /// The whole line, as the note holds it.
    pub line: String,
    pub done: bool,
    /// The words after the box.
    pub words: String,
}

/// A line's box and words, when it is a task line that starts at the line's start.
fn task_parts(line: &str) -> Option<(bool, &str)> {
    let rest = line
        .strip_prefix("- ")
        .or_else(|| line.strip_prefix("* "))
        .or_else(|| line.strip_prefix("+ "))?;
    let (done, words) = if let Some(words) = rest.strip_prefix("[ ]") {
        (false, words)
    } else {
        (true, rest.strip_prefix("[x]").or(rest.strip_prefix("[X]"))?)
    };
    let words = words.strip_prefix(' ')?.trim();
    (!words.is_empty()).then_some((done, words))
}

/// The lines under `## Plan for tonight`, as `section_text` finds that section.
pub fn plan_lines(body: &str) -> Vec<PlanLine> {
    let mut lines = body.lines().enumerate();
    if lines
        .find(|(_, line)| line.trim_end().strip_prefix("## ") == Some("Plan for tonight"))
        .is_none()
    {
        return Vec::new();
    }
    lines
        .take_while(|(_, line)| !line.starts_with("## "))
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(at, line)| match task_parts(line.trim_end()) {
            Some((done, words)) => PlanLine::Task(TaskLine {
                at,
                line: line.trim_end_matches('\r').to_owned(),
                done,
                words: words.to_owned(),
            }),
            None => PlanLine::Links(each_linked_name(line)),
        })
        .collect()
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

/// A scene's line at the end of the plan. A note with no plan gets the heading at its end.
pub fn add_scene(body: &str, words: &str) -> String {
    let line = format!(
        "- [ ] {}",
        words.split_whitespace().collect::<Vec<_>>().join(" ")
    );
    let has_plan = body
        .lines()
        .any(|text| text.trim_end().strip_prefix("## ") == Some("Plan for tonight"));
    if has_plan {
        let mut out = body.to_owned();
        insert_in_plan(&mut out, &line);
        return out;
    }
    let kept = body.trim_end();
    format!(
        "{kept}{}## Plan for tonight\n\n{line}\n",
        if kept.is_empty() { "" } else { "\n\n" }
    )
}

/// The links to a thing that has a new name, whatever their case. Returns whether any changed.
pub fn rename_links(body: &mut String, old: &str, new: &str) -> bool {
    let mut excluded = Vec::new();
    let mut block = None;
    for (event, range) in pulldown_cmark::Parser::new(body).into_offset_iter() {
        match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(_)) => {
                block = Some(range.start)
            }
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::CodeBlock) => {
                if let Some(start) = block.take() {
                    excluded.push(start..range.end);
                }
            }
            pulldown_cmark::Event::Code(_) => excluded.push(range),
            _ => {}
        }
    }
    let mut renamed = String::with_capacity(body.len());
    let mut rest = body.as_str();
    let mut changed = false;
    while let Some(start) = rest.find("[[") {
        let offset = body.len() - rest.len() + start;
        let (before, after) = rest.split_at(start + 2);
        renamed.push_str(before);
        rest = after;
        let Some(end) = after.find(['|', ']']) else {
            break;
        };
        let (name, tail) = after.split_at(end);
        let (target, anchor) = name
            .split_once('#')
            .map_or((name, ""), |(target, _)| (target, &name[target.len()..]));
        if !excluded.iter().any(|range| range.contains(&offset))
            && target.trim().eq_ignore_ascii_case(old)
            && (tail.starts_with('|') || tail.starts_with("]]"))
        {
            renamed.push_str(new);
            renamed.push_str(anchor);
            rest = tail;
            changed = true;
        }
    }
    renamed.push_str(rest);
    *body = renamed;
    changed
}

/// The same for a note that is not open. Its frontmatter and its other words stay as they are.
pub fn rename_links_in_file(path: &Path, old: &str, new: &str) -> Result<bool, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let (front, body) = split_note(&text).unwrap_or(("", &text));
    let mut renamed = body.to_owned();
    if !rename_links(&mut renamed, old, new) {
        return Ok(false);
    }
    replace_file(path, &format!("{front}{renamed}"))?;
    Ok(true)
}

/// A long text, divided at its time headings, so that each part can be read alone.
pub fn parts_of(text: &str) -> Vec<(String, String)> {
    let mut parts: Vec<(String, String)> = Vec::new();
    for line in text.split_inclusive('\n') {
        let heading = line.trim_end().strip_prefix("## ").filter(|time| {
            time.len() == 8 && time.as_bytes()[2] == b':' && time.as_bytes()[5] == b':'
        });
        match (heading, parts.last_mut()) {
            (Some(time), _) => parts.push((time.to_owned(), line.to_owned())),
            (None, Some(last)) => last.1.push_str(line),
            (None, None) => parts.push((String::new(), line.to_owned())),
        }
    }
    // Words before the first heading are read with it.
    if parts.len() > 1 && parts[0].0.is_empty() {
        let first = parts.remove(0);
        parts[0].1.insert_str(0, &first.1);
    }
    if parts.is_empty() {
        parts.push((String::new(), String::new()));
    }
    parts
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
    /// The file is gone from the folder. It may have been moved or renamed.
    Missing,
    Failed(String),
}

/// One file being edited. The prefix (frontmatter) stays as the file has it. Edits save
/// shortly after they stop, and a change made elsewhere is never overwritten silently.
#[derive(Clone)]
pub struct Draft {
    pub path: PathBuf,
    search_folder: PathBuf,
    prefix: String,
    pub body: String,
    baseline: Option<String>,
    pub state: DraftState,
    /// What the file holds now, once the DM asks to see it during a conflict.
    pub on_disk: Option<String>,
    pub moved_to: Option<PathBuf>,
    /// This note's new ID, once it was saved again after its file went.
    pub restored: Option<Uuid>,
    /// The words the DM chose to lose, and how, until the next edit.
    pub dropped: Option<Dropped>,
    /// The file's text when the words were dropped. If the draft has followed the file since,
    /// Undo meets the conflict choice instead of saving over the newer text.
    dropped_baseline: Option<String>,
    /// Why `Save as a new file` could not make the file. The draft stays `Missing`, so no save
    /// writes anywhere until the DM chooses again.
    pub save_again_error: Option<String>,
    last_edit: Option<Instant>,
    /// Changes when the words or the file change. No two drafts share one.
    revision: u64,
}

/// A revision no draft has had.
fn next_revision() -> u64 {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// How a note's draft was lost. Its Undo names the same action.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Dropped {
    /// `Reload and lose my draft` after the file changed on disk.
    Reloaded(String),
    /// `Discard` after the file was moved or deleted.
    Discarded(String),
}

impl Draft {
    pub fn open(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let (prefix, body) = split_note(&text).unwrap_or(("", &text));
        Ok(Self {
            path: path.to_owned(),
            search_folder: path
                .ancestors()
                .skip(1)
                .find(|folder| folder.join(NOTE_PATHS).is_file())
                .or_else(|| path.parent())
                .unwrap_or(path)
                .to_owned(),
            prefix: prefix.to_owned(),
            body: body.to_owned(),
            baseline: Some(text.clone()),
            state: DraftState::Clean,
            on_disk: None,
            moved_to: None,
            restored: None,
            dropped: None,
            dropped_baseline: None,
            save_again_error: None,
            last_edit: None,
            revision: next_revision(),
        })
    }

    /// Changes whenever the words or the file change, so a reader can skip an unchanged draft.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// A conflict or a failed save stays in view through more typing, until it is resolved.
    pub fn edited(&mut self) {
        self.dropped = None;
        if !self.needs_attention() {
            self.state = DraftState::Unsaved;
        }
        self.last_edit = Some(Instant::now());
        self.revision = next_revision();
    }

    /// The text this draft was read from or last wrote.
    pub fn file_text(&self) -> Option<&str> {
        self.baseline.as_deref()
    }

    /// The file still holds the text this draft was read from.
    pub fn file_unchanged(&self) -> bool {
        fs::read_to_string(&self.path).ok() == self.baseline
    }

    /// Does this draft hold something the file does not?
    pub fn unsaved(&self) -> bool {
        matches!(
            self.state,
            DraftState::Unsaved
                | DraftState::Conflict
                | DraftState::Missing
                | DraftState::Failed(_)
        )
    }

    pub fn needs_attention(&self) -> bool {
        matches!(
            self.state,
            DraftState::Conflict | DraftState::Missing | DraftState::Failed(_)
        )
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
            if current.is_none()
                && let Some(note) = self
                    .baseline
                    .as_deref()
                    .and_then(|text| parse_note(&self.path, text))
            {
                let matches: Vec<_> = list_notes(&self.search_folder)
                    .into_iter()
                    .filter(|found| found.id == note.id)
                    .collect();
                if matches.len() == 1 && matches[0].also.is_empty() {
                    self.moved_to = Some(matches[0].path.clone());
                }
            }
            self.state = if current.is_none() {
                DraftState::Missing
            } else {
                DraftState::Conflict
            };
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

    /// Save the words where the file was. A renamed file can still hold this note's ID, so
    /// this one takes a new ID, and two rows can never answer for one session.
    pub fn save_again(&mut self) {
        // The new identity is prepared aside and becomes the draft's only once the file exists:
        // a failed creation leaves the draft as it was, still `Missing`.
        self.save_again_error = None;
        let mut prefix = self.prefix.clone();
        let mut restored = None;
        if self.prefix.contains("session_id:") {
            let id = crate::hermetic::new_uuid();
            let patched = patch_fields(
                &format!("{}{}", self.prefix, self.body),
                &[("session_id", Some(Value::String(id.to_string())))],
            );
            match patched.as_deref().map(split_note) {
                Ok(Some((new_prefix, _))) => {
                    new_prefix.clone_into(&mut prefix);
                    restored = Some(id);
                }
                _ => {
                    self.refuse_save_again("Not saved. Its properties cannot be read.".into());
                    return;
                }
            }
        }
        if self.path.parent().is_none_or(|parent| !parent.is_dir()) {
            self.refuse_save_again("Session folder is gone. Choose folder before saving.".into());
            return;
        }
        let text = format!("{prefix}{}", self.body);
        match write_new(&self.path, &text) {
            Ok(()) => {
                self.prefix = prefix;
                if restored.is_some() {
                    self.restored = restored;
                }
                self.baseline = Some(text);
                self.state = DraftState::Saved;
            }
            Err(error) => self.refuse_save_again(format!("Not saved. {error}")),
        }
    }

    /// A new file could not be made: say why, and keep the missing-file write block.
    fn refuse_save_again(&mut self, error: String) {
        self.save_again_error = Some(error);
        if self.state != DraftState::Missing {
            self.state = DraftState::Missing;
        }
    }

    pub fn save_moved(&mut self) {
        if let Some(path) = self.moved_to.take() {
            self.path = path;
            self.revision = next_revision();
            self.state = DraftState::Unsaved;
            self.save();
        }
    }

    /// The file went, and the DM chose to lose the draft. One step of undo brings it back.
    ///
    /// When the note is known at a new path and that file still holds this note, the editor
    /// follows it: the words, the baseline and the save target become the moved file's. With
    /// no such file, the draft stays `Missing` with the last saved words, so no edit or Save
    /// can write the old path; only the DM's explicit `Save as a new file` does.
    pub fn discard(&mut self) {
        let words = self.body.clone();
        let note = self
            .baseline
            .as_deref()
            .and_then(|text| parse_note(&self.path, text))
            .map(|note| note.id);
        let followed = self.moved_to.take().and_then(|path| {
            let moved = Self::open(&path).ok()?;
            let owner = moved
                .baseline
                .as_deref()
                .and_then(|text| parse_note(&path, text))
                .map(|found| found.id);
            (note.is_some() && owner == note).then_some(moved)
        });
        match followed {
            Some(moved) => *self = moved,
            None => {
                self.body = self
                    .baseline
                    .as_deref()
                    .map(body_of)
                    .unwrap_or_default()
                    .to_owned();
                self.revision = next_revision();
                self.state = DraftState::Missing;
            }
        }
        self.dropped = Some(Dropped::Discarded(words));
        self.dropped_baseline.clone_from(&self.baseline);
    }

    /// Read the file's own words, so that the DM can choose knowing both.
    pub fn show_disk(&mut self) {
        self.on_disk = fs::read_to_string(&self.path)
            .ok()
            .map(|text| body_of(&text).to_owned());
    }

    /// The DM keeps this version over the one on disk.
    pub fn overwrite(&mut self) {
        self.on_disk = None;
        // The frontmatter on disk can be newer, and the DM chose only about the words.
        if let Ok(current) = fs::read_to_string(&self.path)
            && let Some((prefix, _)) = split_note(&current)
        {
            self.prefix = prefix.to_owned();
        }
        self.write();
    }

    /// Back to the file, keeping the draft's words for one step of undo.
    pub fn reload_keeping_undo(&mut self) -> Result<(), String> {
        let words = self.body.clone();
        self.reload()?;
        self.dropped = Some(Dropped::Reloaded(words));
        self.dropped_baseline.clone_from(&self.baseline);
        Ok(())
    }

    /// Bring back what `reload_keeping_undo` or `discard` dropped.
    pub fn undo_dropped(&mut self) {
        if let Some(Dropped::Reloaded(words) | Dropped::Discarded(words)) = self.dropped.take() {
            self.body = words;
            self.edited();
            // The file changed since the words were dropped, and the draft followed it: the DM
            // chooses between the two, as in any conflict.
            if self.baseline != self.dropped_baseline.take() && self.state == DraftState::Unsaved {
                self.state = DraftState::Conflict;
            }
        }
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
            // Words the DM dropped stay recoverable through a reload the timer makes.
            let dropped = (self.dropped.clone(), self.dropped_baseline.clone());
            if self.reload().is_ok() {
                (self.dropped, self.dropped_baseline) = dropped;
                self.state = if state == DraftState::Saved {
                    DraftState::Clean
                } else {
                    state
                };
            }
        }
    }

    /// The one way an act changes this note's words (section 4.1 of the session-cycle design).
    /// `note` is the session the act is for, which its place knows; the file is checked against
    /// it, never against what a reload last read. `patch` gets the words and returns them with
    /// one fragment changed, or `None` when the fragment's place is gone. The Undo that comes
    /// back is the act's, and the act's place keeps it. `Apply` says how the act meets the DM's
    /// typing; see each of its values.
    pub fn apply(
        &mut self,
        note: Uuid,
        write: Apply,
        patch: impl FnOnce(&str) -> Option<String>,
    ) -> Result<NoteUndo, ActFault> {
        if write == Apply::Now {
            // An edit made elsewhere a moment ago is in the words that the act changes.
            self.follow_disk();
            match fs::read_to_string(&self.path) {
                Err(_) => return Err(ActFault::Missing),
                // Unsaved words and a changed file: the conflict view decides first.
                Ok(current)
                    if Some(&current) != self.baseline.as_ref()
                        || self.state == DraftState::Conflict =>
                {
                    return Err(ActFault::Changed);
                }
                Ok(_) if self.state == DraftState::Missing => return Err(ActFault::Missing),
                // Also on a retry: the file holds another note for as long as it does.
                Ok(_) if self.note_id() != Some(note) => return Err(ActFault::OtherNote),
                Ok(_) => {}
            }
        }
        let before = self.body.clone();
        let after = patch(&before).ok_or(ActFault::LineChanged)?;
        self.body.clone_from(&after);
        self.edited();
        let mut undo = NoteUndo {
            path: self.path.clone(),
            note,
            before,
            after,
            written: false,
        };
        if write == Apply::WithTyping {
            return Ok(undo);
        }
        self.save();
        // Otherwise the fragment waits in the draft, and the note's own state says why: a
        // failed save, or a file that changed or went between the check and the save.
        match &self.state {
            DraftState::Saved => {
                undo.written = true;
                Ok(undo)
            }
            DraftState::Failed(error) => Err(ActFault::NotSaved(error.clone(), undo)),
            _ => Err(ActFault::NotSaved(String::new(), undo)),
        }
    }

    /// The note's ID in the words Splat last read from its file.
    fn note_id(&self) -> Option<Uuid> {
        parse_note(&self.path, self.baseline.as_deref()?).map(|note| note.id)
    }
}

/// How a note act meets the DM's typing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Apply {
    /// An act of the session-cycle design: follow the disk, refuse while unsaved words meet a
    /// changed file, and save at once.
    Now,
    /// Today's plan acts, as they were: the change joins the draft as typed words do, and saves
    /// with them. A changed file then shows the note's conflict.
    WithTyping,
}

/// Why a note act did not change the note, or did not reach the file.
#[derive(Clone, Debug, PartialEq)]
pub enum ActFault {
    /// Splat holds unsaved words and the file changed on disk. Nothing was applied.
    Changed,
    /// The file is gone from the folder. Nothing was applied.
    Missing,
    /// The file now holds another note. Nothing was applied.
    OtherNote,
    /// The fragment's place is gone or changed. Nothing was applied.
    LineChanged,
    /// The act is in the draft and not in the file: the save failed with these words, or the
    /// file changed or went before the save. The note's own state says which. The Undo works.
    NotSaved(String, NoteUndo),
}

impl ActFault {
    /// The words at the act's place.
    pub fn words(&self) -> &str {
        match self {
            Self::Changed => "The note changed on disk. Choose in the note first.",
            Self::Missing => "The note is gone from its folder.",
            Self::OtherNote => "The file now holds another note.",
            Self::LineChanged => "That line changed on disk.",
            Self::NotSaved(..) => "Notes not saved",
        }
    }
}

/// The Undo of one note act. It works while the note's words are the words that the act left,
/// with no time limit. Any other change of the words, in Splat or on disk, ends it, and so does
/// another note. The act's place owns it; nothing keeps it across a restart.
#[derive(Clone, Debug, PartialEq)]
pub struct NoteUndo {
    /// The note's file. It follows a move of that note, never a copy.
    path: PathBuf,
    /// The session the act was for.
    note: Uuid,
    before: String,
    after: String,
    /// Whether the act's save wrote the fragment. One that waits in the draft is taken back
    /// from the draft alone.
    written: bool,
}

impl NoteUndo {
    /// The words that the act left.
    pub fn after(&self) -> &str {
        &self.after
    }

    /// Whether this Undo still applies to the note in this draft.
    pub fn live(&self, draft: &Draft) -> bool {
        self.owns(draft) && draft.body == self.after
    }

    /// Whether the draft is the note that the act changed: its file and its ID.
    fn owns(&self, draft: &Draft) -> bool {
        draft.path == self.path && draft.note_id() == Some(self.note)
    }

    /// The folder scan followed this note, the one note with its ID, to a new file.
    pub fn moved(&mut self, from: &Path, to: &Path) {
        if self.path == from {
            to.clone_into(&mut self.path);
        }
    }

    /// An act of `Apply::Now` whose save failed: once a later save wrote the draft with the
    /// fragment, the fragment is written, and its Undo goes back through the checked path.
    pub fn follow_save(&mut self, draft: &Draft) {
        if !self.written && !draft.unsaved() && self.live(draft) {
            self.written = true;
        }
    }

    /// Put back the words from before the act. A fragment that waits in the draft leaves the
    /// draft, with no look at the disk, as typed words would. A written one goes back through
    /// the act's own checked path, and is saved at once.
    pub fn undo(&self, draft: &mut Draft) -> Result<(), ActFault> {
        if !self.owns(draft) {
            return Err(ActFault::OtherNote);
        }
        if self.written {
            return draft
                .apply(self.note, Apply::Now, |words| {
                    (words == self.after).then(|| self.before.clone())
                })
                .map(drop);
        }
        if draft.body != self.after {
            return Err(ActFault::LineChanged);
        }
        draft.body.clone_from(&self.before);
        draft.edited();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integrity_d10_new_notes_and_templates_refuse_a_gone_folder() {
        let dir = tempfile::tempdir().unwrap();
        let gone = dir.path().join("Gone");
        assert!(create_note(&gone, &new_note(1, "", "Words")).is_err());
        assert!(create_template(&gone, "Notes", "Words").is_err());
        assert!(!gone.exists());
    }

    #[test]
    fn integrity_d14_save_again_keeps_an_arriving_file() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(1, "", "Before")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        fs::remove_file(&note.path).unwrap();
        draft.body = "My draft".into();
        draft.edited();
        draft.save();
        fs::write(&note.path, "DM's restored file").unwrap();
        draft.save_again();
        assert!(draft.unsaved());
        assert_eq!(
            fs::read_to_string(&note.path).unwrap(),
            "DM's restored file"
        );
        // Review of PR 255, item 1: through Discard, a failed creation, Undo and a retry, the
        // arriving file keeps its bytes and the draft keeps its identity until a file is made.
        let note = create_note(dir.path(), &new_note(2, "", "Before")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        fs::remove_file(&note.path).unwrap();
        draft.body = "My draft".into();
        draft.edited();
        draft.save();
        assert_eq!(draft.state, DraftState::Missing);
        let identity = draft.prefix.clone();
        draft.discard();
        fs::write(&note.path, "Arrived outside").unwrap();
        draft.save_again();
        assert_eq!(fs::read_to_string(&note.path).unwrap(), "Arrived outside");
        assert_eq!(draft.prefix, identity);
        assert_eq!(draft.restored, None);
        assert_eq!(draft.state, DraftState::Missing);
        draft.undo_dropped();
        assert_eq!(draft.body, "My draft");
        draft.save();
        assert_eq!(fs::read_to_string(&note.path).unwrap(), "Arrived outside");
        draft.save_again();
        assert_eq!(fs::read_to_string(&note.path).unwrap(), "Arrived outside");
        assert_eq!(draft.prefix, identity);
        assert_eq!(draft.restored, None);
        assert!(draft.unsaved() && draft.needs_attention());
        // When the place is free again, the retry makes the file, with a new identity.
        fs::remove_file(&note.path).unwrap();
        draft.save_again();
        assert_eq!(draft.state, DraftState::Saved);
        let id = draft.restored.expect("a new identity");
        assert_ne!(id, note.id);
        let saved = fs::read_to_string(&note.path).unwrap();
        assert!(saved.contains("My draft") && saved.contains(&id.to_string()));
    }

    #[test]
    fn integrity_d14_moves_are_found_across_session_subfolders() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(1, "", "Before")).unwrap();
        let past = dir.path().join("Past");
        fs::create_dir(&past).unwrap();
        let nested = past.join("Meeting.md");
        fs::rename(&note.path, &nested).unwrap();
        let mut draft = Draft::open(&nested).unwrap();
        let moved = dir.path().join("Moved.md");
        fs::rename(&nested, &moved).unwrap();
        draft.body = "My draft".into();
        draft.edited();
        draft.save();
        assert_eq!(draft.moved_to.as_deref(), Some(moved.as_path()));
        draft.save_moved();
        assert_eq!(draft.state, DraftState::Saved);
        assert!(fs::read_to_string(moved).unwrap().contains("My draft"));
    }

    #[test]
    fn integrity_d15_an_edit_during_copy_is_never_deleted() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.md");
        let target = dir.path().join("target.md");
        fs::write(&source, "Before").unwrap();
        let error = transfer_file(&source, &target, true, || {
            fs::write(&source, "DM's edit").unwrap()
        })
        .unwrap_err();
        assert!(error.contains("Both files were kept"));
        assert_eq!(fs::read_to_string(source).unwrap(), "DM's edit");
        assert_eq!(fs::read_to_string(target).unwrap(), "Before");
    }

    #[test]
    fn integrity_d16_links_keep_code_and_other_paths_and_follow_headings() {
        let mut text = "[[Ash Pack#Members|the pack]] `[[Ash Pack]]`\n```md\n[[Ash Pack]]\n```\n[[Places/Ash Pack]]\n".to_owned();
        rename_links(&mut text, "Ash Pack", "Cinder Pack");
        assert_eq!(
            text,
            "[[Cinder Pack#Members|the pack]] `[[Ash Pack]]`\n```md\n[[Ash Pack]]\n```\n[[Places/Ash Pack]]\n"
        );
    }

    #[test]
    fn integrity_d9_new_identity_and_undo_keep_both_files() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(3, "", "DM's words")).unwrap();
        let copy = dir.path().join("Session 4.md");
        fs::copy(&note.path, &copy).unwrap();
        let before = fs::read(&copy).unwrap();
        let undo = new_identity(&copy).unwrap();
        assert_eq!(list_notes(dir.path()).len(), 2);
        assert!(fs::read_to_string(&copy).unwrap().contains("DM's words"));
        undo.undo().unwrap();
        assert_eq!(fs::read(copy).unwrap(), before);
        assert_eq!(list_notes(dir.path()).len(), 1);
    }

    #[test]
    fn integrity_d9_a_shorter_copy_cannot_take_over_the_known_session() {
        let dir = tempfile::tempdir().unwrap();
        let note =
            create_note(dir.path(), &new_note(3, "A long meeting name", "My notes")).unwrap();
        fs::copy(&note.path, dir.path().join("Session 4.md")).unwrap();
        let notes = list_notes(dir.path());
        assert_eq!(notes[0].path, note.path);
        assert_eq!(notes[0].also.len(), 1);
    }

    #[test]
    fn integrity_d10_a_missing_configured_session_folder_blocks_access() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::from_u128(9),
            name: "Marches".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
        let path = save_folder(&campaign, "Meetings").unwrap();
        fs::create_dir_all(&path).unwrap();
        fs::rename(&path, dir.path().join("Moved meetings")).unwrap();
        let error = folder(&campaign).unwrap_err();
        assert!(error.contains("Session folder not found"));
        assert!(!path.exists());
    }

    #[test]
    fn integrity_d12_recordings_keep_results_without_audio() {
        let dir = tempfile::tempdir().unwrap();
        let id = Uuid::from_u128(3);
        let folder = recordings_folder(dir.path(), id);
        fs::create_dir_all(&folder).unwrap();
        fs::write(folder.join("evening.transcript.md"), "Words").unwrap();
        fs::write(folder.join("evening.summary.md"), "Summary").unwrap();
        let rows = list_recordings(dir.path(), id);
        assert_eq!(rows.len(), 1);
        assert!(rows[0].transcript && rows[0].summary);
        assert!(!rows[0].path.exists());
        delete_recording(&rows[0].path).unwrap();
        assert!(list_recordings(dir.path(), id).is_empty());
    }

    #[test]
    fn a_new_name_follows_every_link_and_leaves_other_words() {
        let mut body =
            "- [[Ash Pack]]\n- [[Ash Pack|the pack]]\nAsh Pack is not a link.\n- [[Ash Packs]]\n"
                .to_owned();
        assert!(rename_links(&mut body, "Ash Pack", "Cinder Pack"));
        assert_eq!(
            body,
            "- [[Cinder Pack]]\n- [[Cinder Pack|the pack]]\nAsh Pack is not a link.\n- [[Ash Packs]]\n"
        );
        assert!(!rename_links(&mut body, "Ash Pack", "Other"));
    }

    #[test]
    fn a_new_name_follows_a_link_of_any_case_and_a_link_left_open_stays() {
        let mut body = "- [[ash pack]]\n- [[ASH PACK|them]]\n[[Ash Pack".to_owned();
        assert!(rename_links(&mut body, "Ash Pack", "Cinder Pack"));
        assert_eq!(
            body,
            "- [[Cinder Pack]]\n- [[Cinder Pack|them]]\n[[Ash Pack"
        );
    }

    #[test]
    fn a_note_not_open_keeps_its_frontmatter_and_its_words_through_a_rename() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Session 1.md");
        let text = "---\nsession_title: '[[Ash Pack]]'\n---\n\nWords.\n- [[Ash Pack]]\n";
        fs::write(&path, text).unwrap();
        assert!(rename_links_in_file(&path, "Ash Pack", "Cinder Pack").unwrap());
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            "---\nsession_title: '[[Ash Pack]]'\n---\n\nWords.\n- [[Cinder Pack]]\n"
        );
        assert!(!rename_links_in_file(&path, "Ash Pack", "Other").unwrap());
    }

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
    fn a_copy_of_a_note_shows_one_row_and_the_plain_file_wins() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(12, "", "Words")).unwrap();
        let copy = dir.path().join("Session 12 (conflict).md");
        fs::copy(&note.path, &copy).unwrap();
        let notes = list_notes(dir.path());
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].path, note.path);
        assert_eq!(notes[0].also, [copy]);
    }

    #[test]
    fn a_note_that_cannot_be_read_is_named_and_a_stranger_is_not() {
        let dir = tempfile::tempdir().unwrap();
        create_note(dir.path(), &new_note(1, "", "Fine")).unwrap();
        fs::write(
            dir.path().join("Broken.md"),
            "---\nsession_id: [\n---\nWords",
        )
        .unwrap();
        fs::write(
            dir.path().join("Plain.md"),
            "Mentions session_id in passing",
        )
        .unwrap();
        assert_eq!(unreadable_notes(dir.path()), ["Broken.md"]);
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
    fn a_blank_title_leaves_no_colon_and_a_rename_moves_the_heading() {
        let dir = tempfile::tempdir().unwrap();
        let template = "# Session {{number}}: {{title}}\n\nNote: stays.\n";
        let note = create_note(dir.path(), &new_note(3, "", template)).unwrap();
        let text = fs::read_to_string(&note.path).unwrap();
        assert!(text.ends_with("# Session 3\n\nNote: stays.\n"), "{text}");
        let note = update_details(&note, "The Bridge", "2026-11-01", 3).unwrap();
        let text = fs::read_to_string(&note.path).unwrap();
        assert!(
            text.ends_with("# Session 3: The Bridge\n\nNote: stays.\n"),
            "{text}"
        );
        // A heading the DM wrote stays.
        let body = text.replace("# Session 3: The Bridge", "# Our own heading");
        fs::write(&note.path, body).unwrap();
        let note = update_details(&note, "Another", "2026-11-01", 3).unwrap();
        assert!(
            fs::read_to_string(&note.path)
                .unwrap()
                .contains("# Our own heading")
        );
        assert_eq!(note.title, "Another");
    }

    #[test]
    fn a_new_date_moves_the_date_line_and_only_that() {
        let dir = tempfile::tempdir().unwrap();
        let template = "# Session {{number}}\n\n{{date}}, The Marches\n\nOn 2026-11-01 we met.\n";
        let note = create_note(dir.path(), &new_note(1, "", template)).unwrap();
        update_details(&note, "", "2026-11-07", 1).unwrap();
        let text = fs::read_to_string(&note.path).unwrap();
        assert!(text.contains("\n2026-11-07, The Marches\n"), "{text}");
        assert!(text.contains("On 2026-11-01 we met."), "{text}");
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
        // A note act beside unsaved words and a changed file is not applied, and says so.
        // The words and the file stay as they were, and the conflict view decides.
        let act = draft.apply(note.id, Apply::Now, |words| {
            Some(format!("{words}- [[Act]]\n"))
        });
        assert_eq!(act, Err(ActFault::Changed));
        assert_eq!(
            act.unwrap_err().words(),
            "The note changed on disk. Choose in the note first."
        );
        assert_eq!(draft.body, "Mine\n");
        assert_eq!(fs::read_to_string(&note.path).unwrap(), outside);
        // Today's plan acts are as they were: the line joins the draft, as typed words do.
        let undo = draft
            .apply(note.id, Apply::WithTyping, |words| {
                Some(format!("{words}- [[Plan]]\n"))
            })
            .unwrap();
        assert_eq!(draft.body, "Mine\n- [[Plan]]\n");
        assert_eq!(draft.state, DraftState::Unsaved);
        undo.undo(&mut draft).unwrap();
        assert_eq!(draft.body, "Mine\n");
        draft.save();
        assert_eq!(draft.state, DraftState::Conflict);
        assert_eq!(fs::read_to_string(&note.path).unwrap(), outside);
        assert_eq!(
            draft.apply(note.id, Apply::Now, |words| Some(format!(
                "{words}- [[Act]]\n"
            ))),
            Err(ActFault::Changed),
            "a conflict waits for the DM's choice"
        );
        // More typing does not hide the choice.
        draft.body.push_str("And more\n");
        draft.edited();
        assert_eq!(draft.state, DraftState::Conflict);
        // Saving again changes nothing until the DM chooses.
        draft.save();
        assert_eq!(fs::read_to_string(&note.path).unwrap(), outside);

        draft.overwrite();
        assert_eq!(draft.state, DraftState::Saved);
        assert!(
            fs::read_to_string(&note.path)
                .unwrap()
                .ends_with("And more\n")
        );

        fs::write(&note.path, &outside).unwrap();
        draft.body = "Kept for one step\n".into();
        draft.edited();
        draft.save();
        draft.reload_keeping_undo().unwrap();
        assert!(matches!(draft.dropped, Some(Dropped::Reloaded(_))));
        draft.undo_dropped();
        assert_eq!(draft.body, "Kept for one step\n");
        assert_eq!(draft.state, DraftState::Unsaved);
        draft.overwrite();
        fs::write(&note.path, &outside).unwrap();
        draft.body = "Lost\n".into();
        draft.edited();
        draft.save();
        draft.reload().unwrap();
        assert_eq!(draft.state, DraftState::Clean);
        assert!(draft.body.contains("From Obsidian"));
    }

    /// Section 4.1's path for one note act: an edit on disk a moment before, a failed save, a
    /// restart, a missing file, another note, a line that is gone, then act A, an unrelated act
    /// B and Undo of A. Unsaved words with a changed file are in the test above.
    #[cfg(unix)]
    #[test]
    fn a_note_act_follows_the_disk_saves_at_once_and_keeps_its_undo() {
        let dir = tempfile::tempdir().unwrap();
        let folder = dir.path().join("Sessions");
        fs::create_dir(&folder).unwrap();
        let plan = "## Plan for tonight\n\n- [[Bridge]]\n\n## What happened\n";
        let note = create_note(&folder, &new_note(1, "", plan)).unwrap();
        let other = create_note(&folder, &new_note(2, "", plan)).unwrap();
        let add = |line: &'static str| {
            move |words: &str| {
                let mut words = words.to_owned();
                insert_in_plan(&mut words, line);
                Some(words)
            }
        };
        let mut draft = Draft::open(&note.path).unwrap();

        // An edit made in Obsidian a moment before the act is kept, under the act's line.
        let outside = fs::read_to_string(&note.path)
            .unwrap()
            .replace("## What happened\n", "## What happened\n\nFrom Obsidian.\n");
        fs::write(&note.path, &outside).unwrap();
        let a = draft
            .apply(note.id, Apply::Now, add("- [[Wolf Pack]]"))
            .unwrap();
        let saved = fs::read_to_string(&note.path).unwrap();
        assert_eq!(
            draft.state,
            DraftState::Saved,
            "saved at once, with no pause"
        );
        assert!(
            saved.contains("- [[Bridge]]\n- [[Wolf Pack]]\n") && saved.contains("From Obsidian.")
        );
        assert!(a.live(&draft));
        // An Undo belongs to its own file: a copy with the same ID and the same words at
        // another path does not take it. A move that the folder scan follows does (the plan's
        // test of Undo remove).
        let copy = folder.join("Session 1 (copy).md");
        fs::copy(&note.path, &copy).unwrap();
        let mut copied = Draft::open(&copy).unwrap();
        assert_eq!(
            (copied.note_id(), copied.body.as_str()),
            (Some(note.id), a.after())
        );
        assert!(!a.live(&copied));
        assert_eq!(a.undo(&mut copied), Err(ActFault::OtherNote));
        assert_eq!(fs::read(&copy).unwrap(), fs::read(&note.path).unwrap());
        fs::remove_file(&copy).unwrap();

        // Act B is unrelated: another note. Undo of A puts back only A's line, at once, and B
        // stays.
        let mut second = Draft::open(&other.path).unwrap();
        let b = second
            .apply(other.id, Apply::Now, add("- [[Toll Gang]]"))
            .unwrap();
        a.undo(&mut draft).unwrap();
        assert_eq!(fs::read_to_string(&note.path).unwrap(), outside);
        assert!(!a.live(&draft), "an Undo acts once");
        assert!(
            fs::read_to_string(&other.path)
                .unwrap()
                .contains("- [[Toll Gang]]")
        );
        assert!(b.live(&second));
        // Another change of the words ends an Undo, also with no time limit before it.
        second.body.push_str("Typed.\n");
        second.edited();
        assert!(!b.live(&second));
        assert_eq!(b.undo(&mut second), Err(ActFault::LineChanged));

        // A failed save is reported and writes nothing; the line waits in the draft.
        let listed = |folder: &Path| {
            let mut names: Vec<_> = fs::read_dir(folder)
                .unwrap()
                .map(|file| file.unwrap().file_name())
                .collect();
            names.sort();
            names
        };
        let files = listed(&folder);
        let before = fs::read_to_string(&note.path).unwrap();
        let locked = |mode| {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&folder, fs::Permissions::from_mode(mode)).unwrap();
        };
        locked(0o555);
        assert!(
            fs::write(folder.join("probe"), "").is_err(),
            "the folder takes no new file"
        );
        let failed = draft.apply(note.id, Apply::Now, add("- [[Ash Pack]]"));
        locked(0o755);
        let Err(fault @ ActFault::NotSaved(..)) = failed else {
            panic!("{failed:?}");
        };
        assert_eq!(fault.words(), "Notes not saved");
        assert!(matches!(draft.state, DraftState::Failed(_)));
        assert!(draft.body.contains("- [[Ash Pack]]"));
        assert_eq!(fs::read_to_string(&note.path).unwrap(), before);
        assert_eq!(listed(&folder), files, "nothing half written");
        // The next act saves both lines.
        draft.apply(note.id, Apply::Now, add("- [[Rook]]")).unwrap();

        // After a restart the file holds the lines, and nothing else remembers the acts.
        let reopened = Draft::open(&note.path).unwrap();
        assert!(reopened.body.contains("- [[Ash Pack]]\n- [[Rook]]\n"));
        assert_eq!(listed(&folder), files);
        assert_eq!(listed(dir.path()), [std::ffi::OsString::from("Sessions")]);

        // A line that is gone writes nothing and says so.
        let gone = draft.apply(note.id, Apply::Now, |_| None);
        assert_eq!(
            gone.as_ref().unwrap_err().words(),
            "That line changed on disk."
        );
        assert_eq!(gone, Err(ActFault::LineChanged));

        // A missing file is not an outside edit.
        let moved = folder.join("Moved.md");
        fs::rename(&note.path, &moved).unwrap();
        assert_eq!(
            draft.apply(note.id, Apply::Now, add("- [[Lost]]")),
            Err(ActFault::Missing)
        );
        assert!(!draft.body.contains("Lost"));
        // A file that now holds another note is not this one, also after the one-second poll
        // took that note's words, and on every retry: the act is for note 1.
        fs::copy(&other.path, &note.path).unwrap();
        draft.follow_disk();
        assert_eq!(draft.note_id(), Some(other.id), "the poll follows the file");
        for _ in 0..2 {
            assert_eq!(
                draft.apply(note.id, Apply::Now, add("- [[Lost]]")),
                Err(ActFault::OtherNote)
            );
        }
        assert_eq!(
            fs::read(&note.path).unwrap(),
            fs::read(&other.path).unwrap()
        );

        // An Undo belongs to its note's ID too: the same path and words in another note are not
        // that note, and its Undo writes nothing there.
        let third = create_note(&folder, &new_note(3, "", plan)).unwrap();
        let mut replaced = Draft::open(&third.path).unwrap();
        let c = replaced
            .apply(third.id, Apply::Now, add("- [[Ash]]"))
            .unwrap();
        let text = fs::read_to_string(&other.path).unwrap();
        let (prefix, _) = split_note(&text).unwrap();
        let swapped = format!("{prefix}{}", c.after());
        fs::write(&third.path, &swapped).unwrap();
        replaced.follow_disk();
        assert_eq!(replaced.body, c.after());
        assert!(!c.live(&replaced));
        assert_eq!(c.undo(&mut replaced), Err(ActFault::OtherNote));
        assert_eq!(fs::read_to_string(&third.path).unwrap(), swapped);

        // An outside write after the act's check and before its save: the fragment waits in the
        // draft with the conflict, and its Undo takes it back from the draft alone.
        let fourth = create_note(&folder, &new_note(4, "", plan)).unwrap();
        let mut pending = Draft::open(&fourth.path).unwrap();
        let outside = fs::read_to_string(&fourth.path)
            .unwrap()
            .replace("## What happened\n", "## What happened\n\nOutside.\n");
        let result = pending.apply(fourth.id, Apply::Now, |words| {
            fs::write(&fourth.path, &outside).unwrap();
            add("- [[Pending]]")(words)
        });
        let Err(ActFault::NotSaved(_, d)) = result else {
            panic!("{result:?}");
        };
        assert_eq!(pending.state, DraftState::Conflict);
        assert!(d.live(&pending));
        d.undo(&mut pending).unwrap();
        assert!(!pending.body.contains("Pending"));
        assert_eq!(pending.state, DraftState::Conflict, "the DM still chooses");
        assert_eq!(fs::read_to_string(&fourth.path).unwrap(), outside);
    }

    #[test]
    fn discarding_a_deleted_notes_draft_writes_no_file() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(5, "", "Start\n")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        fs::remove_file(&note.path).unwrap();
        draft.body = "My words\n".into();
        draft.edited();
        draft.save();
        assert_eq!(draft.state, DraftState::Missing);
        assert_eq!(draft.moved_to, None);
        assert!(!note.path.exists());

        draft.discard();
        assert!(!note.path.exists());
        assert!(list_notes(dir.path()).is_empty());
        // iter-25-S SM1: with no file left, the editor says so.
        assert_eq!(draft.state, DraftState::Missing);
        assert_eq!(draft.body, "Start\n");
        draft.undo_dropped();
        assert_eq!(draft.body, "My words\n");
        assert!(!note.path.exists());
        // Discard, edit, Save: nothing is written at the old path.
        draft.discard();
        draft.body.push_str("Typed after discard\n");
        draft.edited();
        draft.save();
        assert_eq!(draft.state, DraftState::Missing);
        assert!(!note.path.exists());
    }

    #[test]
    fn a_note_whose_file_went_is_saved_again_under_a_new_id() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(4, "", "Start\n")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        // Renamed in Obsidian.
        fs::create_dir_all(dir.path().join("Past")).unwrap();
        let renamed = dir.path().join("Past/Renamed.md");
        fs::rename(&note.path, &renamed).unwrap();
        draft.body = "My words\n".into();
        draft.edited();
        draft.save();
        assert_eq!(draft.state, DraftState::Missing);
        assert!(draft.needs_attention() && draft.unsaved());
        assert!(!note.path.exists());
        assert_eq!(draft.moved_to.as_deref(), Some(renamed.as_path()));

        let mut moved = draft.clone();
        moved.save_moved();
        assert_eq!(moved.state, DraftState::Saved);
        assert_eq!(
            parse_note(&renamed, &fs::read_to_string(&renamed).unwrap())
                .unwrap()
                .id,
            note.id
        );
        assert!(!note.path.exists());

        // Discard names itself, so its Undo says Undo discard, not Undo reload. It writes
        // nothing: the moved file keeps its text, and no file comes back at the old path.
        // iter-25-S SM1: Discard follows the moved file, with the words it holds now (here an
        // outside edit), and later saves go there.
        let moved_text = fs::read_to_string(&renamed)
            .unwrap()
            .replace("My words", "Moved words");
        fs::write(&renamed, &moved_text).unwrap();
        let mut discarded = draft.clone();
        discarded.discard();
        assert!(!note.path.exists());
        assert_eq!(fs::read_to_string(&renamed).unwrap(), moved_text);
        assert_eq!(
            discarded.dropped,
            Some(Dropped::Discarded("My words\n".into()))
        );
        assert_eq!(discarded.path, renamed);
        assert_eq!(discarded.state, DraftState::Clean);
        assert_eq!(discarded.body, "Moved words\n");
        // Review of PR 255, item 2: the moved file is edited outside; the disk timer follows it
        // (twice, the second time with nothing new). The outside words show, and the discarded
        // words stay recoverable; their Undo meets the conflict choice, not a silent write.
        let mut followed = discarded.clone();
        let edited_text = moved_text.replace("Moved words", "Edited in Obsidian");
        fs::write(&renamed, &edited_text).unwrap();
        followed.follow_disk();
        followed.follow_disk();
        assert_eq!(followed.body, "Edited in Obsidian\n");
        assert_eq!(
            followed.dropped,
            Some(Dropped::Discarded("My words\n".into()))
        );
        followed.undo_dropped();
        assert_eq!(followed.body, "My words\n");
        assert_eq!(followed.state, DraftState::Conflict);
        followed.save();
        assert_eq!(fs::read_to_string(&renamed).unwrap(), edited_text);
        fs::write(&renamed, &moved_text).unwrap();
        let mut typed = discarded.clone();
        typed.body.push_str("Typed after discard\n");
        typed.edited();
        typed.save();
        assert_eq!(typed.state, DraftState::Saved);
        assert!(!note.path.exists());
        assert!(
            fs::read_to_string(&renamed)
                .unwrap()
                .contains("Moved words\nTyped after discard")
        );
        fs::write(&renamed, &moved_text).unwrap();
        discarded.undo_dropped();
        assert_eq!(discarded.body, "My words\n");
        assert_eq!(discarded.dropped, None);
        assert!(!note.path.exists());
        assert_eq!(fs::read_to_string(&renamed).unwrap(), moved_text);
        // Undo discard, then Save: the words go to the moved file, never the old path.
        discarded.save();
        assert_eq!(discarded.state, DraftState::Saved);
        assert!(!note.path.exists());
        assert!(fs::read_to_string(&renamed).unwrap().contains("My words"));
        fs::write(&renamed, &moved_text).unwrap();
        // A moved file that no longer holds this note is not followed.
        let mut other = draft.clone();
        let other_text = moved_text.replace(&note.id.to_string(), &Uuid::new_v4().to_string());
        fs::write(&renamed, &other_text).unwrap();
        other.discard();
        assert_eq!(other.path, note.path);
        assert_eq!(other.state, DraftState::Missing);
        assert_eq!(fs::read_to_string(&renamed).unwrap(), other_text);
        fs::write(&renamed, &moved_text).unwrap();

        draft.save_again();
        assert_eq!(draft.state, DraftState::Saved);
        let id = draft.restored.unwrap();
        assert_ne!(id, note.id);
        let notes = list_notes(dir.path());
        assert_eq!(notes.len(), 2);
        assert!(notes.iter().any(|n| n.id == id && n.path == note.path));
        assert!(notes.iter().any(|n| n.id == note.id && n.path == renamed));
        assert!(
            fs::read_to_string(&note.path)
                .unwrap()
                .ends_with("My words\n")
        );
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
    fn words_go_under_what_happened_or_at_the_end() {
        let mut note = "# Title\n\n## What happened\n\nWe met.\n\n\n## Next\n\nLater.\n".to_owned();
        insert_in_note(&mut note, "[[Timeline#^a|Rest]]");
        assert_eq!(
            note,
            "# Title\n\n## What happened\n\nWe met.\n\n[[Timeline#^a|Rest]]\n\n## Next\n\nLater.\n"
        );
        let mut last = "## What happened\n".to_owned();
        insert_in_note(&mut last, "A");
        assert_eq!(last, "## What happened\n\nA\n");
        let mut none = "Free text\n\n".to_owned();
        insert_in_note(&mut none, "A");
        assert_eq!(none, "Free text\n\nA\n");
        let mut empty = String::new();
        insert_in_note(&mut empty, "A");
        assert_eq!(empty, "A\n");
        // A longer heading that starts the same way is not this one.
        let mut other = "## What happened later\n\nX\n".to_owned();
        insert_in_note(&mut other, "A");
        assert!(other.ends_with("X\n\nA\n"), "{other}");
    }

    #[test]
    fn a_table_note_starts_on_a_fresh_line_under_what_happened() {
        let template = "# Title\n\n## What happened\n\n\n\n## Open threads\n\n";
        let mut body = template.to_owned();
        let (caret, changed) = fresh_line_in_note(&mut body);
        assert!(!changed);
        assert_eq!(body, template);
        assert_eq!(
            body.chars().take(caret).collect::<String>(),
            "# Title\n\n## What happened\n\n"
        );

        let mut tight = "## What happened\n## Next\n".to_owned();
        let (caret, changed) = fresh_line_in_note(&mut tight);
        assert!(changed);
        assert_eq!(tight, "## What happened\n\n\n\n## Next\n");
        assert_eq!(
            tight.chars().take(caret).collect::<String>(),
            "## What happened\n\n"
        );

        // Accented text counts in characters, which is what the caret counts.
        let mut plain = "Été".to_owned();
        let (caret, _) = fresh_line_in_note(&mut plain);
        assert_eq!(caret, 5);
        assert_eq!(plain, "Été\n\n");

        let mut empty = String::new();
        assert_eq!(fresh_line_in_note(&mut empty), (0, false));

        // A second note, and a third, never join the heading that follows.
        let mut note = template.to_owned();
        for words in ["First note.", "Second note.", "Third note."] {
            let (caret, _) = fresh_line_in_note(&mut note);
            let at = note
                .char_indices()
                .nth(caret)
                .map_or(note.len(), |(at, _)| at);
            note.insert_str(at, words);
        }
        assert_eq!(
            note,
            "# Title\n\n## What happened\n\nFirst note.\n\nSecond note.\n\nThird note.\n\n## Open threads\n\n"
        );
    }

    #[test]
    fn a_summary_nested_in_a_note_keeps_the_notes_sections() {
        let mut note = "## What happened\n\nWe met.\n\n## Open threads\n\n".to_owned();
        insert_in_note(
            &mut note,
            &nested_in_note("> A draft.\n\n## Recap\n\nA rest.\n"),
        );
        assert_eq!(
            note,
            "## What happened\n\nWe met.\n\n> A draft.\n\n### Recap\n\nA rest.\n\n## Open threads\n\n"
        );
        // A later note still finds the end of the section, after the nested heading.
        let (caret, _) = fresh_line_in_note(&mut note);
        let before: String = note.chars().take(caret).collect();
        assert!(before.ends_with("A rest.\n\n"), "{before}");
    }

    #[test]
    fn the_reader_leaves_out_only_what_the_page_already_shows() {
        let note = Note {
            id: Uuid::from_u128(1),
            number: 2,
            title: "Bridge".into(),
            date: "2026-11-01".into(),
            path: "a.md".into(),
            also: Vec::new(),
        };
        let body = "# Session 2: Bridge\n\n2026-11-01, The Marches\n\n## What happened\n";
        assert_eq!(without_own_heading(body, &note), "## What happened\n");
        // A heading the DM wrote stays, and so does a date line under it.
        let own = "# Our night\n\n2026-11-01\n";
        assert_eq!(without_own_heading(own, &note), own);
    }

    #[test]
    fn reading_leaves_out_empty_headings_and_keeps_filled_ones() {
        let text = "## Recap\n\n\n## Plan\n\nGo north.\n\n## People\n\n### Friends\n\n## Loot\n\n";
        assert_eq!(without_empty_sections(text), "## Plan\n\nGo north.\n");
        // A heading whose subheading has words is not empty.
        assert_eq!(
            without_empty_sections("## People\n\n### Friends\n\nHobb.\n"),
            "## People\n\n### Friends\n\nHobb.\n"
        );
    }

    #[test]
    fn a_section_is_found_by_its_heading() {
        let body = "## What happened\n\nA fight.\n\n## Open threads\n\nWho burned the ropes?\n";
        assert_eq!(section_text(body, "What happened").unwrap(), "A fight.");
        assert_eq!(
            section_text(body, "Open threads").unwrap(),
            "Who burned the ropes?"
        );
        assert_eq!(
            section_text("## What happened\n\n\n", "What happened"),
            None
        );
        assert_eq!(section_text(body, "Loot"), None);
    }

    #[test]
    fn links_are_read_as_names_and_as_words() {
        let text =
            "See [[Ashfall Crossing]], [[Hobb|the miller]] and [[Timeline#^t-1|Day 1]]. [[Hobb]]";
        assert_eq!(linked_names(text), vec!["Ashfall Crossing", "Hobb"]);
        assert_eq!(
            each_linked_name(text),
            vec!["Ashfall Crossing", "Hobb", "Hobb"]
        );
        assert_eq!(
            plain_links("See [[Ashfall Crossing]] and [[Hobb|the miller]], [[open"),
            "See Ashfall Crossing and the miller, [[open"
        );
    }

    #[test]
    fn a_plan_line_goes_under_the_plan_heading() {
        let mut body = "## Plan for tonight\n\n\n\n## What happened\n\n".to_owned();
        insert_in_plan(&mut body, "[[A]]");
        insert_in_plan(&mut body, "[[B]]");
        assert_eq!(
            body,
            "## Plan for tonight\n\n[[A]]\n[[B]]\n\n## What happened\n\n"
        );
        let mut other = "## Next time\n\n".to_owned();
        insert_in_plan(&mut other, "[[A]]");
        assert!(other.contains("[[A]]"));
    }

    /// T12.10: a task line is a scene line, a plain or nested line is not, and a tick, a removal
    /// and a new scene change only their own line.
    #[test]
    fn a_task_line_of_the_plan_is_a_scene_line_and_changes_alone() {
        let body = "## Plan for tonight\n\n- [[Bridge]]\n- [ ] [[Captain Varra]]\n  - [ ] nested\n* [x] The miller is the informer\n- [] not a box\n1. [ ] numbered\n\n## What happened\n\n- [ ] Not the plan\n";
        let lines = plan_lines(body);
        let tasks: Vec<_> = lines
            .iter()
            .filter_map(|line| match line {
                PlanLine::Task(task) => Some((task.at, task.done, task.words.as_str())),
                PlanLine::Links(_) => None,
            })
            .collect();
        assert_eq!(
            tasks,
            [
                (3, false, "[[Captain Varra]]"),
                (5, true, "The miller is the informer")
            ]
        );
        assert_eq!(
            lines.len(),
            6,
            "every other line of the plan keeps its links"
        );
        assert!(lines.contains(&PlanLine::Links(vec!["Bridge".into()])));

        let tick = change_line(
            body,
            3,
            "- [ ] [[Captain Varra]]",
            Some(&ticked("- [ ] [[Captain Varra]]", true)),
        )
        .unwrap();
        assert_eq!(tick.len(), body.len());
        assert_eq!(
            tick.chars()
                .zip(body.chars())
                .filter(|(a, b)| a != b)
                .count(),
            1,
            "a tick changes one character"
        );
        assert!(tick.contains("- [x] [[Captain Varra]]\n"));
        assert_eq!(
            ticked("* [x] The miller is the informer", false),
            "* [ ] The miller is the informer"
        );

        // Two lines with the same words: the act changes the one at its own place, and a place
        // that holds other words now changes nothing, even when another line reads the same.
        let twice = "## Plan for tonight\n\n- [ ] A\n- [[Bridge]]\n- [ ] A\n";
        assert_eq!(
            change_line(twice, 4, "- [ ] A", None).unwrap(),
            "## Plan for tonight\n\n- [ ] A\n- [[Bridge]]\n"
        );
        assert_eq!(change_line(twice, 2, "- [ ] B", None), None);
        assert_eq!(change_line(twice, 3, "- [ ] A", None), None);
        assert_eq!(change_line(twice, 9, "- [ ] A", None), None);

        assert_eq!(
            add_scene(
                "## Plan for tonight\n\n- [[Bridge]]\n\n## What happened\n",
                "  Meet\nHobb "
            ),
            "## Plan for tonight\n\n- [[Bridge]]\n- [ ] Meet Hobb\n\n## What happened\n"
        );
        assert_eq!(
            add_scene("# Session 1\n\nWords.\n", "Meet Hobb"),
            "# Session 1\n\nWords.\n\n## Plan for tonight\n\n- [ ] Meet Hobb\n"
        );
    }

    #[test]
    fn fmt_f1_empty_session_title_is_absent() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(1, "", "Body")).unwrap();
        assert!(
            !fs::read_to_string(&note.path)
                .unwrap()
                .contains("session_title:")
        );
        let titled = update_details(&note, "Title", "2026-11-03", 1).unwrap();
        let cleared = update_details(&titled, "", "2026-11-03", 1).unwrap();
        assert!(
            !fs::read_to_string(&cleared.path)
                .unwrap()
                .contains("session_title:")
        );
    }

    #[test]
    fn a_text_divides_at_time_headings_and_loses_nothing() {
        let text =
            "> A note.\n\n## 00:00:00\n\nOne.\n\n## People\n\nTwo.\n\n## 00:10:00\n\nThree.\n";
        let parts = parts_of(text);
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].0, "00:00:00");
        assert_eq!(parts[1].0, "00:10:00");
        assert_eq!(parts[0].1.clone() + &parts[1].1, text);
        assert_eq!(parts_of("No times.\n").len(), 1);
        assert_eq!(parts_of("").len(), 1);
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
    fn moving_storage_keeps_every_file_and_refuses_collisions() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::from_u128(9),
            name: "Marches".into(),
            path: dir.path().to_owned(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
        let old = open_folder(&campaign).unwrap();
        let note = create_note(&old, &new_note(1, "The Bridge", "My words.")).unwrap();
        let relative = [
            note.path.strip_prefix(&old).unwrap().to_path_buf(),
            PathBuf::from("Timeline.md"),
            PathBuf::from("Templates/Own.md"),
            PathBuf::from("Recordings/session/audio.wav"),
            PathBuf::from("Recordings/session/audio.transcript.md"),
        ];
        for path in &relative[1..] {
            fs::create_dir_all(old.join(path).parent().unwrap()).unwrap();
            fs::write(old.join(path), format!("kept {}", path.display())).unwrap();
        }
        fs::write(old.join("Other.md"), "Not a session note.").unwrap();
        let originals: Vec<_> = relative
            .iter()
            .map(|path| fs::read(old.join(path)).unwrap())
            .collect();
        let new = dir.path().join("Evenings");
        fs::create_dir_all(new.join("Templates")).unwrap();
        fs::write(new.join("Templates/Own.md"), "Existing template").unwrap();
        assert!(
            move_storage(&campaign, "Evenings")
                .unwrap_err()
                .contains("already exists")
        );
        assert_eq!(folder(&campaign).unwrap(), old);
        assert!(note.path.exists());
        assert_eq!(
            fs::read_to_string(new.join("Templates/Own.md")).unwrap(),
            "Existing template"
        );
        fs::remove_file(new.join("Templates/Own.md")).unwrap();
        assert_eq!(move_storage(&campaign, "Evenings").unwrap(), new);
        assert_eq!(folder(&campaign).unwrap(), new);
        for (path, bytes) in relative.iter().zip(originals) {
            assert_eq!(fs::read(new.join(path)).unwrap(), bytes);
            assert!(!old.join(path).exists());
        }
        assert!(old.join("Other.md").exists());
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
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
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
        fs::write(
            folder.join("2026-11-01 21-00.state.txt"),
            "Failed at part 3 of 24: busy",
        )
        .unwrap();
        let list = list_recordings(dir.path(), session);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].name, "2026-11-01 21-00");
        assert_eq!(list[0].seconds, Some(2));
        assert!(list[0].transcript && !list[0].summary);
        assert_eq!(
            list[0].status.as_deref(),
            Some("Failed at part 3 of 24: busy")
        );
        assert!(!list[1].transcript);

        delete_recording(&list[0].path).unwrap();
        assert!(!folder.join("2026-11-01 21-00.transcript.md").exists());
        assert!(!folder.join("2026-11-01 21-00.state.txt").exists());
        assert_eq!(list_recordings(dir.path(), session).len(), 1);
        assert!(list_recordings(dir.path(), Uuid::from_u128(4)).is_empty());
    }
}
