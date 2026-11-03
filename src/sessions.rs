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
    // The plainest name first, so that a copy such as `Session 12 (conflict).md` never wins.
    notes.sort_by_key(|note| (note.path.as_os_str().len(), note.path.clone()));
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
                title: recording_title(&path),
                seconds: crate::recording::Layout::read(&path)
                    .ok()
                    .map(|l| l.seconds()),
                bytes: file.metadata().map_or(0, |metadata| metadata.len()),
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
        crate::processing::warning_path(path),
        path.with_extension("state.txt"),
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

/// The links to a thing that has a new name, whatever their case. Returns whether any changed.
pub fn rename_links(body: &mut String, old: &str, new: &str) -> bool {
    let mut renamed = String::with_capacity(body.len());
    let mut rest = body.as_str();
    let mut changed = false;
    while let Some(start) = rest.find("[[") {
        let (before, after) = rest.split_at(start + 2);
        renamed.push_str(before);
        rest = after;
        let Some(end) = after.find(['|', ']']) else {
            break;
        };
        let (name, tail) = after.split_at(end);
        if name.trim().eq_ignore_ascii_case(old)
            && (tail.starts_with('|') || tail.starts_with("]]"))
        {
            renamed.push_str(new);
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
pub struct Draft {
    pub path: PathBuf,
    prefix: String,
    pub body: String,
    baseline: Option<String>,
    pub state: DraftState,
    /// What the file holds now, once the DM asks to see it during a conflict.
    pub on_disk: Option<String>,
    /// This note's new ID, once it was saved again after its file went.
    pub restored: Option<Uuid>,
    /// The words the DM chose to lose, until the next edit.
    pub dropped: Option<String>,
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
            on_disk: None,
            restored: None,
            dropped: None,
            last_edit: None,
        })
    }

    /// A conflict or a failed save stays in view through more typing, until it is resolved.
    pub fn edited(&mut self) {
        self.dropped = None;
        if !self.needs_attention() {
            self.state = DraftState::Unsaved;
        }
        self.last_edit = Some(Instant::now());
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
        if self.prefix.contains("session_id:") {
            let id = crate::hermetic::new_uuid();
            let patched = patch_fields(
                &format!("{}{}", self.prefix, self.body),
                &[("session_id", Some(Value::String(id.to_string())))],
            );
            match patched.as_deref().map(split_note) {
                Ok(Some((prefix, _))) => {
                    self.prefix = prefix.to_owned();
                    self.restored = Some(id);
                }
                _ => {
                    self.state =
                        DraftState::Failed("Not saved. Its properties cannot be read.".into());
                    return;
                }
            }
        }
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        self.write();
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
        self.dropped = Some(words);
        Ok(())
    }

    /// Bring back what `reload_keeping_undo` dropped.
    pub fn undo_reload(&mut self) {
        if let Some(words) = self.dropped.take() {
            self.body = words;
            self.edited();
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
        draft.save();
        assert_eq!(draft.state, DraftState::Conflict);
        assert_eq!(fs::read_to_string(&note.path).unwrap(), outside);
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
        draft.undo_reload();
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

    #[test]
    fn a_note_whose_file_went_is_saved_again_under_a_new_id() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(4, "", "Start\n")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        // Renamed in Obsidian.
        let renamed = dir.path().join("Renamed.md");
        fs::rename(&note.path, &renamed).unwrap();
        draft.body = "My words\n".into();
        draft.edited();
        draft.save();
        assert_eq!(draft.state, DraftState::Missing);
        assert!(draft.needs_attention() && draft.unsaved());
        assert!(!note.path.exists());

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
