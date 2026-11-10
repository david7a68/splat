//! The campaign's world: plain notes Splat reads and does not own, and the NPCs and locations
//! made from them. Obsidian stays a plain note's editor; an entry also has Edit here. A plain
//! note becomes a Lookup item, not an entry, so nothing depends on it.
//! See `docs/design/world-and-house-rules.md`.
mod checks;
mod edit;
mod fields;
pub mod lines;
pub mod page;
mod related;
pub(crate) mod repair;

use crate::{
    content_workspace::{EntryRef, patch_fields},
    design::{self, Action, Density, Tone, Type, colors, reader as style},
    encounter_run::{GroupMember, RunGroup},
    library::{note_path, view, write_note},
    lookup::{ContentIndex, Item},
    markdown_view::{FILE_LINK, NOTE_LINK},
    persistence::Campaign,
    sessions, timeline,
};
use eframe::egui;
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use related::related;
use serde_yaml_ng::{Mapping, Value};
use splat::character::stable_id;
use splat::content::{Entry, Reference, art::FORMATS, overrides::Overrides};
use std::{
    collections::HashMap,
    fs,
    hash::{Hash, Hasher},
    path::{Component, Path, PathBuf},
    sync::Arc,
};
use uuid::Uuid;

/// The kind of a plain note's Lookup item. No file holds this kind.
const NOTE_KIND: &str = "world_note";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    World,
    Session,
    Transcript,
}

#[derive(Debug)]
pub struct PlainNote {
    /// Relative to the campaign root, with forward slashes.
    pub path: String,
    pub name: String,
    pub role: Role,
    pub body: String,
}

#[derive(Default)]
pub struct Scan {
    pub notes: Vec<PlainNote>,
    /// By lower-case file name, as Obsidian finds an embedded image.
    pub images: HashMap<String, PathBuf>,
}

fn walk(current: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(current) else {
        return;
    };
    let mut paths: Vec<_> = entries.flatten().map(|entry| entry.path()).collect();
    paths.sort();
    for path in paths {
        // A vault's `.obsidian` and `.trash` hold Markdown that is nobody's world.
        if path
            .file_name()
            .is_some_and(|name| name.to_string_lossy().starts_with('.'))
        {
            continue;
        }
        let Ok(meta) = fs::symlink_metadata(&path) else {
            continue;
        };
        if meta.file_type().is_symlink() {
            continue;
        }
        if meta.is_dir() {
            walk(&path, files);
        } else {
            files.push(path);
        }
    }
}

/// What a note is for. The test is its role and not its folder, because a session folder can be
/// the campaign root, and a DM can keep world notes beside sessions.
fn role_of(front: Option<&str>, inside_sessions: Option<&Path>) -> Option<Role> {
    let has = |name: &str| front.is_some_and(|front| front.lines().any(|l| l.starts_with(name)));
    if has("splat_")
        || has("timeline_version:")
        || has("trackers_version:")
        || has(crate::sheet_writer::KEY)
    {
        return None;
    }
    if has("session_id:") {
        return Some(Role::Session);
    }
    if let Some(inside) = inside_sessions
        && let Some(Component::Normal(first)) = inside.components().next()
        && (first == sessions::TEMPLATES || first == sessions::RECORDINGS)
    {
        return None;
    }
    Some(Role::World)
}

/// The best words of a recording, and the name to show them under.
fn transcript_name(path: &Path) -> Option<String> {
    let name = path.file_name()?.to_string_lossy();
    if let Some(stem) = name.strip_suffix(".marked.md") {
        return Some(format!("{stem}, transcript"));
    }
    let stem = name.strip_suffix(".transcript.md")?;
    (!path.with_file_name(format!("{stem}.marked.md")).exists())
        .then(|| format!("{stem}, transcript"))
}

pub fn scan(root: &Path, sessions_folder: &Path) -> Scan {
    let mut scan = Scan::default();
    let mut files = Vec::new();
    walk(root, &mut files);
    // A session folder can be an absolute path outside the campaign.
    if !sessions_folder.starts_with(root) {
        walk(sessions_folder, &mut files);
    }
    let recordings = sessions_folder.join(sessions::RECORDINGS);
    for path in files {
        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_lowercase())
            .unwrap_or_default();
        if FORMATS.contains(&extension.as_str()) {
            scan.images.entry(file_name).or_insert(path);
            continue;
        }
        if extension != "md"
            || (file_name.eq_ignore_ascii_case(timeline::FILE)
                && path.parent() == Some(sessions_folder))
        {
            continue;
        }
        // A recording leaves several files. Only one is worth reading.
        let transcript = if path.starts_with(&recordings) {
            match transcript_name(&path) {
                Some(name) => Some(name),
                None => continue,
            }
        } else {
            None
        };
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let text = text.trim_start_matches('\u{feff}').replace("\r\n", "\n");
        let (front, body) =
            sessions::split_note(&text).map_or((None, text.as_str()), |(f, b)| (Some(f), b));
        let (name, role) = match transcript {
            Some(name) => (name, Role::Transcript),
            None => {
                let Some(role) = role_of(front, path.strip_prefix(sessions_folder).ok()) else {
                    continue;
                };
                let stem = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                (stem, role)
            }
        };
        scan.notes.push(PlainNote {
            path: path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/"),
            name,
            role,
            body: body.to_owned(),
        });
    }
    scan
}

/// Changes when a note is edited, so returning from Obsidian can refresh what Splat shows.
pub fn fingerprint(root: &Path) -> u64 {
    let mut files = Vec::new();
    walk(root, &mut files);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for path in files {
        if path.extension().is_some_and(|e| e == "md" || e == "json")
            && let Ok(meta) = fs::metadata(&path)
        {
            path.hash(&mut hasher);
            meta.len().hash(&mut hasher);
            if let Ok(modified) = meta.modified() {
                modified.hash(&mut hasher);
            }
        }
    }
    hasher.finish()
}

/// A note has no UUID. This one is stable for its path, and it never reaches a file.
pub fn note_id(path: &str) -> Uuid {
    stable_id(&format!("splat:world_note:{path}"))
}

pub fn note_entry(path: &str, name: &str, body: String) -> Entry {
    Entry {
        path: path.to_owned(),
        id: note_id(path),
        key: None,
        name: name.to_owned(),
        kind: NOTE_KIND.to_owned(),
        fields: Mapping::new().into(),
        body,
        body_line: 1,
        inventory: Vec::new(),
        inventory_containers: Vec::new(),
        flags: Vec::new(),
    }
}

fn outside_code(body: &str, mut plain: impl FnMut(&str, &mut String)) -> String {
    let mut output = String::with_capacity(body.len());
    let mut fence = false;
    for line in body.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            fence = !fence;
            output.push_str(line);
            continue;
        }
        if fence {
            output.push_str(line);
            continue;
        }
        for (index, part) in line.split('`').enumerate() {
            if index > 0 {
                output.push('`');
            }
            if index % 2 == 0 {
                plain(part, &mut output);
            } else {
                output.push_str(part);
            }
        }
    }
    output
}

/// Every `[[link]]` in a text, as (is an embed, target, words).
pub(crate) fn each_link(text: &str) -> Vec<(std::ops::Range<usize>, bool, String, Option<String>)> {
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(start) = text[from..].find("[[").map(|at| at + from) {
        let inner = start + 2;
        let Some(end) = text[inner..].find("]]").map(|at| at + inner) else {
            break;
        };
        let embed = text[..start].ends_with('!');
        let mut parts = text[inner..end].splitn(2, '|');
        let target = parts.next().unwrap_or("").trim_end_matches('\\').trim();
        let words = parts.next().map(|words| words.trim().to_owned());
        if !target.is_empty() && !target.contains('\n') {
            let at = if embed { start - 1 } else { start };
            found.push((at..end + 2, embed, target.to_owned(), words));
        }
        from = end + 2;
    }
    found
}

/// The file or note each link in a body names, in lower case, without its heading.
pub fn links_in(body: &str) -> Vec<String> {
    let mut links = Vec::new();
    outside_code(body, |part, _| {
        for (_, _, target, _) in each_link(part) {
            let name = target.split('#').next().unwrap_or("").trim().to_lowercase();
            if !name.is_empty() {
                links.push(name);
            }
        }
    });
    links
}

/// Redirect explicit links when their named dependency is copied. Keep their display words.
pub(crate) fn redirect_links(text: &str, old: &Entry, new_id: Uuid) -> String {
    outside_code(text, |part, output| {
        let mut copied = 0;
        for (range, embed, target, words) in each_link(part) {
            output.push_str(&part[copied..range.start]);
            let (name, heading) = target
                .split_once('#')
                .map_or((target.as_str(), String::new()), |(name, heading)| {
                    (name, format!("#{heading}"))
                });
            let matches = name.eq_ignore_ascii_case(&old.name)
                || name == old.id.to_string()
                || old.key.as_deref() == Some(name)
                || name.trim_end_matches(".md") == old.path.trim_end_matches(".md");
            if matches && !embed {
                output.push_str(&format!(
                    "[[{new_id}{heading}|{}]]",
                    words.as_deref().unwrap_or(&target)
                ));
            } else {
                output.push_str(&part[range.clone()]);
            }
            copied = range.end;
        }
        output.push_str(&part[copied..]);
    })
}

/// Rename note links outside code. Keep headings, aliases, and unrelated words.
pub(crate) fn rename_note_links(text: &str, old: &str, new: &str) -> (String, usize) {
    let mut count = 0;
    let (front, body) = crate::sessions::split_note(text).unwrap_or(("", text));
    let after = outside_code(body, |part, output| {
        let mut copied = 0;
        for (range, embed, target, words) in each_link(part) {
            output.push_str(&part[copied..range.start]);
            let (name, heading) = target
                .split_once('#')
                .map_or((target.as_str(), String::new()), |(name, heading)| {
                    (name, format!("#{heading}"))
                });
            if !embed && name.eq_ignore_ascii_case(old) {
                output.push_str(&format!(
                    "[[{new}{heading}{}]]",
                    words.map_or(String::new(), |words| format!("|{words}"))
                ));
                count += 1;
            } else {
                output.push_str(&part[range.clone()]);
            }
            copied = range.end;
        }
        output.push_str(&part[copied..]);
    });
    (format!("{front}{after}"), count)
}

/// Obsidian's links, written as links the Markdown reader understands.
pub fn with_links(body: &str, images: &HashMap<String, PathBuf>) -> String {
    outside_code(body, |part, output| {
        let mut copied = 0;
        for (range, embed, target, words) in each_link(part) {
            output.push_str(&part[copied..range.start]);
            copied = range.end;
            let file = target.rsplit('/').next().unwrap_or(&target).to_lowercase();
            let image = FORMATS
                .iter()
                .any(|extension| file.ends_with(&format!(".{extension}")));
            if embed && image {
                match images.get(&file) {
                    Some(path) => {
                        output.push_str(&format!("[image](<{FILE_LINK}{}>)", path.display()));
                    }
                    None => output.push_str(&format!("(Image not found: {target})")),
                }
                continue;
            }
            let shown = words.unwrap_or_else(|| {
                target
                    .rsplit('#')
                    .next()
                    .unwrap_or(&target)
                    .trim_start_matches('^')
                    .to_owned()
            });
            let shown = shown.replace('[', "\\[").replace(']', "\\]");
            let target = target.replace(['<', '>'], "");
            output.push_str(&format!("[{shown}](<{NOTE_LINK}{target}>)"));
        }
        output.push_str(&part[copied..]);
    })
}

/// Each heading's words, as the reader shows them.
pub fn headings(body: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut heading: Option<String> = None;
    for event in Parser::new_ext(body, Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH) {
        match event {
            Event::Start(Tag::Heading { .. }) => heading = Some(String::new()),
            Event::End(TagEnd::Heading(_)) => found.extend(heading.take()),
            Event::Text(text) | Event::Code(text) => {
                if let Some(heading) = &mut heading {
                    heading.push_str(&text);
                }
            }
            _ => {}
        }
    }
    found.retain(|text| !text.trim().is_empty());
    found
}

// --- World entries.

fn world_field<'a>(entry: &'a Entry, name: &str) -> Option<&'a Value> {
    entry.fields.get("splat_world")?.get(name)
}

/// Empty optional values have no row in Read mode. The editor keeps the source value.
fn reader_field<'a>(entry: &'a Entry, key: &str) -> Option<&'a Value> {
    world_field(entry, key).filter(|value| {
        !value.is_null() && value.as_str().is_none_or(|text| !text.trim().is_empty())
    })
}

fn reference(value: &Value) -> Option<Reference> {
    serde_yaml_ng::from_value::<EntryRef>(value.clone())
        .ok()?
        .reference()
        .ok()
}

pub fn world_type(entry: &Entry) -> &str {
    world_field(entry, "type")
        .and_then(Value::as_str)
        .unwrap_or("")
}

pub fn status(entry: &Entry) -> &str {
    world_field(entry, "status")
        .and_then(Value::as_str)
        .unwrap_or("")
}

pub fn aliases(entry: &Entry) -> Vec<&str> {
    world_field(entry, "aliases")
        .and_then(Value::as_sequence)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect()
}

/// What this entry belongs to, as its file wrote each group.
pub fn part_of(entry: &Entry) -> Vec<&Value> {
    world_field(entry, "part_of")
        .and_then(Value::as_sequence)
        .into_iter()
        .flatten()
        .collect()
}

/// An event's date, as its file wrote it: `120 years ago`.
pub fn when(entry: &Entry) -> Option<&str> {
    world_field(entry, "when").and_then(Value::as_str)
}

pub fn until(entry: &Entry) -> Option<&str> {
    world_field(entry, "until").and_then(Value::as_str)
}

/// How long before Day 1 a date is, in minutes of the campaign clock. A date counts from
/// Day 1, so it does not move as the campaign runs.
pub fn ago(date: &str) -> Option<u64> {
    let date = date.trim().to_lowercase();
    let date = date.strip_prefix("about ").unwrap_or(&date);
    timeline::read_duration(date.strip_suffix(" ago")?.trim())
}

/// An event's date and its end, for a list: `120 to 90 years ago`.
pub fn date_words(entry: &Entry) -> Option<String> {
    let when = when(entry)?.trim();
    let Some(until) = until(entry)
        .map(str::trim)
        .filter(|until| !until.is_empty())
    else {
        return Some(when.to_owned());
    };
    // The same unit is said once.
    let short = when
        .strip_suffix(" ago")
        .and_then(|start| start.rsplit_once(' '))
        .filter(|(_, unit)| until.ends_with(&format!(" {unit} ago")))
        .map(|(number, _)| number);
    Some(match short {
        Some(number) => format!("{number} to {until}"),
        None => format!("{when} to {until}"),
    })
}

/// Any entry with a stat block can join a fight. An NPC can choose one when it does.
pub fn fights(entry: &Entry) -> bool {
    world_type(entry) == "npc" || world_field(entry, "creature").is_some_and(|v| !v.is_null())
}

pub(crate) fn link_name(value: &Value, fallback: &str) -> String {
    value
        .as_str()
        .or_else(|| value.get("link").and_then(Value::as_str))
        .map(|link| {
            link.trim_matches(['[', ']'])
                .split('|')
                .next_back()
                .unwrap_or(link)
                .to_owned()
        })
        .unwrap_or_else(|| fallback.to_owned())
}

fn creature(entry: &Entry) -> Option<Reference> {
    reference(world_field(entry, "creature")?)
}

/// A world entry's type, as a heading and a list say it.
pub fn type_group(kind: &str) -> String {
    match kind {
        "npc" => "NPCs".into(),
        "location" => "Locations".into(),
        // These have no plural.
        "lore" | "history" | "news" | "equipment" | "gear" => type_label(kind),
        other => {
            let label = type_label(other);
            if label.ends_with('y')
                && !label
                    .chars()
                    .rev()
                    .nth(1)
                    .is_some_and(|c| "aeiou".contains(c))
            {
                format!("{}ies", label.trim_end_matches('y'))
            } else if label.ends_with('s') {
                format!("{label}es")
            } else {
                format!("{label}s")
            }
        }
    }
}

pub fn type_label(kind: &str) -> String {
    match kind {
        "npc" => "NPC".to_owned(),
        "" => "World entry".to_owned(),
        other => style::human_label(other),
    }
}

/// The fields every new world entry starts with.
fn entry_fields(id: Uuid, name: &str, kind: &str) -> Vec<(&'static str, Value)> {
    let mut fields = vec![
        ("splat_schema", 2.into()),
        ("splat_id", id.to_string().into()),
        ("splat_kind", "world".into()),
        ("splat_name", name.into()),
        ("splat_type", kind.into()),
    ];
    if kind == "npc" {
        fields.push(("splat_status", "alive".into()));
    }
    fields
}

fn frontmatter(id: Uuid, name: &str, kind: &str) -> Result<String, String> {
    let fields: Mapping = entry_fields(id, name, kind)
        .into_iter()
        .map(|(key, value)| (key.into(), value))
        .collect();
    let yaml = serde_yaml_ng::to_string(&fields).map_err(|e| e.to_string())?;
    Ok(format!("---\n{yaml}---\n"))
}

/// A new NPC or location from a name typed at the table. Nothing else is required.
pub fn create_entry(campaign: &Campaign, name: &str, kind: &str) -> Result<Uuid, String> {
    create_with_details(campaign, name, kind, "")
}

/// A new NPC from the generator, with its attributes as the note's text.
pub(crate) fn broken_links(item: &Item, index: &ContentIndex) -> Vec<&'static str> {
    let mut problems = Vec::new();
    if reader_field(&item.entry, "in").is_some()
        && index.place_of(item.entry.id).is_none()
        && index.place_choices(item.entry.id).is_empty()
    {
        problems.push("Place not found");
    }
    if reader_field(&item.entry, "creature").is_some()
        && index.world_target(&item.entry, "creature").is_none()
    {
        problems.push("Stat block not found");
    }
    problems
}

pub fn create_generated(
    campaign: &Campaign,
    npc: &crate::npc_generator::Generated,
) -> Result<Uuid, String> {
    create_with_details(campaign, &npc.name, "npc", &npc.details())
}

/// The folder that holds most entries of this type, from each entry's path and type. Folders
/// belong to the DM, so a new entry goes where its kind already is.
pub(crate) fn common_folder<'a>(
    entries: impl Iterator<Item = (&'a str, &'a str)>,
    kind: &str,
) -> Option<String> {
    let mut folders: std::collections::BTreeMap<&str, usize> = Default::default();
    for (path, _) in entries.filter(|(_, other)| !kind.is_empty() && *other == kind) {
        *folders
            .entry(path.rsplit_once('/').map_or("", |(folder, _)| folder))
            .or_default() += 1;
    }
    // Of two equal folders, the first by name, so the answer does not change between loads.
    folders
        .into_iter()
        .max_by_key(|&(folder, count)| (count, std::cmp::Reverse(folder)))
        .map(|(folder, _)| folder.to_owned())
}

/// Where a new entry of this kind is written when the campaign has none yet.
fn default_folder(kind: &str) -> String {
    match kind {
        "" => "Notes".into(),
        kind => sessions::file_safe(&type_group(kind)),
    }
}

/// Where a new entry of this kind is written, from the files as they are now.
fn folder_for(campaign: &Campaign, kind: &str) -> String {
    let entries = splat::content::read_directory(&campaign.path)
        .map(|(entries, _)| entries)
        .unwrap_or_default();
    common_folder(
        entries
            .iter()
            .filter(|entry| entry.kind == "world")
            .map(|entry| (entry.path.as_str(), world_type(entry))),
        kind,
    )
    .unwrap_or_else(|| default_folder(kind))
}

/// The link to an entry just made by `create_entry` or `create_generated`, before the index
/// reads it. Its bare name serves when that is the file's name and nothing else has it.
pub(crate) fn new_entry_link(index: Option<&ContentIndex>, name: &str, kind: &str) -> String {
    let name = name.trim();
    let stem = sessions::file_safe(name);
    let taken = index.is_some_and(|index| {
        index.has_file_named(&stem) || !matches!(index.resolve_link(name, None), Go::Missing(_))
    });
    if stem == name && !taken {
        format!("[[{name}]]")
    } else {
        let shown: String = name
            .chars()
            .filter(|c| !matches!(c, '|' | '[' | ']'))
            .collect();
        let folder = index
            .and_then(|index| index.type_folder(kind))
            .unwrap_or_else(|| default_folder(kind));
        if folder.is_empty() {
            format!("[[{stem}|{shown}]]")
        } else {
            format!("[[{folder}/{stem}|{shown}]]")
        }
    }
}

fn create_with_details(
    campaign: &Campaign,
    name: &str,
    kind: &str,
    details: &str,
) -> Result<Uuid, String> {
    create_at(campaign, name, kind, details).map(|(id, _)| id)
}

/// The new entry's identity and its path in the campaign.
fn create_at(
    campaign: &Campaign,
    name: &str,
    kind: &str,
    details: &str,
) -> Result<(Uuid, String), String> {
    let name = name.trim();
    let stem = sessions::file_safe(name);
    if stem.is_empty() {
        return Err("Type a name first.".to_owned());
    }
    let folder = folder_for(campaign, kind);
    let relative = if folder.is_empty() {
        format!("{stem}.md")
    } else {
        format!("{folder}/{stem}.md")
    };
    let path = note_path(&campaign.path, &relative)?;
    if path.exists() {
        return Err(format!("{relative} already exists."));
    }
    let id = crate::hermetic::new_uuid();
    let mut text = if kind.is_empty() {
        format!("# {name}\n")
    } else {
        format!("{}\n# {name}\n", frontmatter(id, name, kind)?)
    };
    if !details.is_empty() {
        text.push('\n');
        text.push_str(details);
    }
    fs::create_dir_all(campaign.path.join(&folder)).map_err(|e| e.to_string())?;
    if kind.is_empty() {
        use std::io::Write;
        let mut temp =
            tempfile::NamedTempFile::new_in(path.parent().ok_or("Missing parent folder")?)
                .map_err(|e| e.to_string())?;
        temp.write_all(text.as_bytes())
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|e| e.to_string())?;
        temp.persist_noclobber(&path).map_err(|e| e.to_string())?;
    } else {
        write_note(&campaign.path, &relative, None, &text)?;
    }
    let id = if kind.is_empty() {
        note_id(&relative)
    } else {
        id
    };
    Ok((id, relative))
}

/// Give a plain note an identity. Its body does not change.
pub fn make_entry(campaign: &Campaign, item: &Item, kind: &str) -> Result<Uuid, String> {
    let relative = &item.entry.path;
    let path = note_path(&campaign.path, relative)?;
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let id = crate::hermetic::new_uuid();
    let name = &item.entry.name;
    let updated = if sessions::split_note(&text).is_some() {
        let changes = entry_fields(id, name, kind)
            .into_iter()
            .map(|(key, value)| (key, Some(value)))
            .collect::<Vec<_>>();
        patch_fields(&text, &changes)?
    } else {
        format!("{}\n{text}", frontmatter(id, name, kind)?)
    };
    write_note(&campaign.path, relative, Some(&text), &updated)?;
    Ok(id)
}

/// What a note keeps to point at an entry: its key, or a link Obsidian can follow too.
fn stored_link(entry: &Entry) -> String {
    entry
        .key
        .clone()
        .unwrap_or_else(|| format!("[[{}|{}]]", entry.path.trim_end_matches(".md"), entry.name))
}

/// Change one field of a world entry's `splat_world`, keeping the rest of the note.
fn set_world_field(
    campaign: &Campaign,
    entry: &Entry,
    name: &str,
    value: Value,
) -> Result<Change, String> {
    let path = note_path(&campaign.path, &entry.path)?;
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    Entry::parse(&entry.path, &text)?;
    let updated = fields::patch(&text, name, value)?;
    write_note(&campaign.path, &entry.path, Some(&text), &updated)?;
    Ok(Change {
        path: entry.path.clone(),
        before: text,
        after: updated,
        word: "World value saved.".into(),
        kind: ChangeKind::of_key(name),
    })
}

/// What a Log line says about one changed value. A place and a group are links, so the line
/// also shows on their pages.
fn log_words(index: &ContentIndex, key: &str, before: Option<&Value>, after: &Value) -> String {
    let label = match key {
        "in" => "In",
        "part_of" => "Part of",
        _ => "Status",
    };
    let one = |value: &Value| -> String {
        if key == "status" {
            return match value.as_str().map(str::trim) {
                Some(word) if !word.is_empty() => word.to_owned(),
                _ => "not set".into(),
            };
        }
        if value.is_null() {
            return "not set".into();
        }
        index.world_value_target(value).map_or_else(
            || format!("[[{}]]", link_name(value, label)),
            |at| index.link_to(at),
        )
    };
    let all = |value: &Value| match value.as_sequence() {
        Some(groups) if groups.is_empty() => "not set".into(),
        Some(groups) => groups.iter().map(&one).collect::<Vec<_>>().join(", "),
        None => one(value),
    };
    format!(
        "{label} {} → {}.",
        before.map_or_else(|| "not set".into(), all),
        all(after)
    )
}

/// One NPC, as an encounter group of one, so a run adds them the way it adds any group.
pub fn npc_group(npc: &Entry, stat_block: &Entry, side: &str) -> Arc<RunGroup> {
    let overrides = Overrides {
        name: Some(npc.name.clone()),
        ..Default::default()
    };
    let problem = match overrides.resolve_for(stat_block, side) {
        Err(error) => Some(error),
        Ok(resolved) if resolved.max_hp.is_none() => Some(format!(
            "{} has no maximum HP, so choose another stat block.",
            stat_block.name
        )),
        Ok(_) => None,
    };
    let mut frozen = stat_block.clone();
    frozen
        .fields
        .insert("splat_run_npc_name".into(), npc.name.clone().into());
    Arc::new(RunGroup {
        id: npc.id,
        path: npc.path.clone(),
        name: npc.name.clone(),
        side: side.to_owned(),
        encounter: None,
        encounter_name: None,
        members: vec![GroupMember {
            entry: Arc::new(frozen),
            count: 1,
            overrides,
        }],
        notes: String::new(),
        xp: 0,
        xp_unknown: Vec::new(),
        unavailable: 0,
        unavailable_members: Vec::new(),
        warnings: Vec::new(),
        problem,
    })
}

/// Always offer both routes: Obsidian for editing and the system file handler.
pub(crate) fn file_actions(ui: &mut egui::Ui, campaign: &Campaign, relative: &str) {
    entry_file_actions(ui, &campaign.path, relative, true);
}

pub(crate) fn entry_file_actions(ui: &mut egui::Ui, root: &Path, relative: &str, obsidian: bool) {
    ui.horizontal_wrapped(|ui| entry_file_parts(ui, root, relative, obsidian));
}

pub(crate) fn entry_file_parts(ui: &mut egui::Ui, root: &Path, relative: &str, obsidian: bool) {
    let path = root.join(relative);
    let action_width = design::number_width(ui, "Show file", &Type::Body.font(ui))
        + if obsidian {
            design::number_width(ui, "Edit in Obsidian", &Type::Body.font(ui)) + design::S
        } else {
            0.0
        };
    let budget = (ui.available_width() - action_width - design::L).max(0.0);
    let font = Type::Small.font(ui);
    let mut shown = relative.to_owned();
    if design::number_width(ui, &shown, &font) > budget {
        let filename = relative.rsplit('/').next().unwrap_or(relative);
        shown = format!("…/{filename}");
        if design::number_width(ui, &shown, &font) > budget {
            shown = design::words_that_fit(ui, filename, &font, budget);
            shown = shown.trim_end_matches('…').to_owned();
            while !shown.is_empty()
                && design::number_width(ui, &format!("{shown}…"), &font) > budget
            {
                shown.pop();
            }
            shown.push('…');
        }
    }
    let response = ui.label(Type::Small.rich(ui, shown).color(colors().muted));
    design::named(ui, &response, &path.display().to_string());
    response.on_hover_text(path.display().to_string());
    if obsidian && design::link(ui, "Edit in Obsidian").clicked() {
        ui.ctx()
            .open_url(egui::OpenUrl::new_tab(obsidian_url(&path)));
    }
    if design::link(ui, "Show file").clicked() {
        ui.ctx().open_url(egui::OpenUrl::new_tab(format!(
            "file://{}",
            encode_path(&path)
        )));
    }
}

fn encode_path(path: &Path) -> String {
    path.to_string_lossy()
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-._~/".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

fn obsidian_url(path: &Path) -> String {
    format!("obsidian://open?path={}", encode_path(path))
}

pub(crate) fn make_npc_note(campaign: &Campaign, entry: &Entry) -> Result<Uuid, String> {
    let (id, path) = create_at(campaign, &entry.name, "npc", "")?;
    let text = fs::read_to_string(campaign.path.join(&path)).map_err(|e| e.to_string())?;
    let npc = Entry::parse(&path, &text)?;
    set_world_field(
        campaign,
        &npc,
        "creature",
        serde_yaml_ng::to_value(EntryRef::id(entry.id)).map_err(|e| e.to_string())?,
    )?;
    Ok(id)
}

#[derive(Default)]
struct Picker {
    key: &'static str,
    search: String,
    new_location: String,
    focus: bool,
}

#[derive(Clone)]
pub(crate) struct Change {
    path: String,
    before: String,
    after: String,
    word: String,
    kind: ChangeKind,
}

/// What a change touched, which decides its Undo words and the row that shows it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum ChangeKind {
    Place,
    PartOf,
    StatBlock,
    Status,
    Name,
    Identity,
    /// Everything the edit form saved at once.
    Edit,
}

impl ChangeKind {
    /// The kind for one `splat_world` key.
    fn of_key(key: &str) -> Self {
        match key {
            "in" => Self::Place,
            "part_of" => Self::PartOf,
            "status" => Self::Status,
            _ => Self::StatBlock,
        }
    }

    fn undo_label(self) -> &'static str {
        match self {
            Self::Place => "Undo place change",
            Self::PartOf => "Undo group change",
            Self::Status => "Undo status change",
            Self::StatBlock => "Undo stat block change",
            Self::Name => "Undo name change",
            Self::Identity => "Undo new identity",
            Self::Edit => "Undo changes",
        }
    }

    /// A relationship row shows its own change when there is room beside it.
    fn has_row(self) -> bool {
        matches!(
            self,
            Self::Place | Self::PartOf | Self::StatBlock | Self::Status
        )
    }
}

impl Change {
    pub(crate) fn message(&self) -> &str {
        &self.word
    }

    pub(crate) fn undo(&self, campaign: &Campaign) -> Result<(), String> {
        write_note(&campaign.path, &self.path, Some(&self.after), &self.before).map(|_| ())
    }
}

pub(crate) const DUPLICATE_ENTRY: &str = "These two files claim to be the same entry.";
pub(crate) const SEPARATE_ENTRY_UNDO: &str = "Undo separate entry";

pub(crate) fn separate_entry_action(filename: &str) -> String {
    format!("Make {filename} a separate entry")
}

/// Long filenames keep the complete repair action inside its secondary border.
pub(crate) fn separate_entry_button(ui: &mut egui::Ui, filename: &str) -> bool {
    let label = separate_entry_action(filename);
    if Action::secondary(&label).natural_width(ui) <= ui.available_width() {
        return Action::secondary(&label).show(ui).clicked();
    }
    let response = ui.add(
        egui::Button::new(Type::Strong.rich(ui, &label))
            .wrap()
            .min_size(egui::vec2(0.0, Density::Scan.control()))
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::new(
                design::HAIRLINE_WIDTH,
                colors().field_border,
            )),
    );
    design::named(ui, &response, &label);
    response.clicked()
}

pub(crate) fn separate_entry_result(filename: &str) -> String {
    format!("{filename} is now a separate entry.")
}

pub(crate) fn new_identity(campaign: &Campaign, relative: &str) -> Result<(Change, Uuid), String> {
    let path = note_path(&campaign.path, relative)?;
    let before = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let id = crate::hermetic::new_uuid();
    let after = crate::content_workspace::new_note_identity(&before, relative, id)?;
    write_note(&campaign.path, relative, Some(&before), &after)?;
    Ok((
        Change {
            path: relative.into(),
            before,
            after,
            word: separate_entry_result(
                Path::new(relative)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .as_ref(),
            ),
            kind: ChangeKind::Identity,
        },
        id,
    ))
}

pub(crate) fn repair_in_world(path: &Path) -> bool {
    fs::read_to_string(path)
        .ok()
        .and_then(|text| Entry::parse("note.md", &text).ok())
        .is_some_and(|entry| entry.kind == "world")
}

// --- Reading.

/// What a run can tell a reader. `None` means that no run is open.
pub type RunSides = Option<Vec<String>>;

pub struct Join {
    pub group: Arc<RunGroup>,
    pub total: Option<i64>,
    /// The NPC just saved a stat block, so the index is out of date.
    pub saved: bool,
}

/// What pressing something in a reader asks for.
pub enum Go {
    Item(usize, Option<String>),
    Many(Vec<usize>),
    Missing(String),
    Timeline(String),
    Reload(Uuid),
    Join(Join),
}

/// What only the page holding a reader can do.
pub enum Nav {
    Item(usize),
    /// A link to a note that does not exist. The DM may want to create it.
    Missing(String),
    Reload(Uuid),
}

#[derive(Default)]
struct Adding {
    stat_block: Option<usize>,
    search: String,
    save: bool,
    side: String,
    initiative: String,
}

enum FormEnd {
    Cancel,
    Add,
}

/// Lookup's and the World page's reader, and its controls for the item on show.
#[derive(Default)]
pub struct Reader {
    showing: Option<Uuid>,
    status: String,
    status_editing: bool,
    status_drafts: HashMap<Uuid, String>,
    /// Edit forms by entry. A form with changes stays while the DM reads other entries.
    edits: HashMap<Uuid, edit::EditDraft>,
    /// What Discard dropped, until the DM undoes it or edits again.
    discarded: Option<(Uuid, edit::EditDraft)>,
    /// A field of the form had the keyboard when it last drew.
    form_focused: bool,
    pub compact: bool,
    other_type: String,
    adding: Option<Adding>,
    picker: Option<Picker>,
    undo: Option<Change>,
    identity_undo: Option<(Uuid, Change)>,
    name_undo: Option<Change>,
    error: Option<String>,
    /// A link that matches several files, waiting for the DM to choose.
    many: Vec<usize>,
    many_heading: Option<String>,
    place_choices_open: bool,
    place_choices_document: Option<Uuid>,
    heading_document: Option<Uuid>,
    heading: Option<String>,
    /// What only the app can do, for it to take after this reader draws.
    pub join: Option<Join>,
    pub open_timeline: Option<String>,
    pub sides: RunSides,
    /// A run is in live play and owns `Next turn`, the screen's primary action.
    pub live: bool,
    pub in_run: std::collections::BTreeMap<Uuid, (usize, Option<i64>)>,
    pub select_run: Option<usize>,
    /// The session being played and the day, as a Log line begins: `[[Session 1]], Day 1`.
    /// With no session, a change writes no Log line.
    pub log_as: Option<String>,
}

impl Reader {
    /// Offer the same short chooser to links opened outside the World page.
    pub fn choose_link(&mut self, positions: Vec<usize>, target: &str) {
        self.many = positions;
        self.many_heading = target
            .split('|')
            .next()
            .and_then(|target| target.split_once('#'))
            .map(|(_, heading)| {
                heading
                    .rsplit('#')
                    .next()
                    .unwrap_or(heading)
                    .trim()
                    .to_owned()
            });
    }

    /// Show this heading the next time the item is drawn.
    pub fn show_heading(&mut self, heading: Option<String>) {
        self.heading_document = None;
        self.heading = heading;
    }

    fn settle(&mut self, go: Go) -> Option<Nav> {
        match go {
            Go::Item(position, heading) => {
                self.many.clear();
                self.heading = heading;
                return Some(Nav::Item(position));
            }
            Go::Many(positions) => self.many = positions,
            Go::Missing(words) => return Some(Nav::Missing(words)),
            Go::Timeline(id) => self.open_timeline = Some(id),
            Go::Reload(id) => return Some(Nav::Reload(id)),
            Go::Join(join) => self.join = Some(join),
        }
        None
    }

    /// The choice a link to several files is waiting for. It stays above the scrolling text.
    pub fn choices(&mut self, ui: &mut egui::Ui, index: &ContentIndex) -> Option<Nav> {
        if self.many.is_empty() {
            return None;
        }
        ui.label(
            Type::Small
                .rich(ui, "This link matches more than one note. Choose one.")
                .color(colors().muted),
        );
        let mut chosen = None;
        for &position in &self.many {
            if design::link(ui, &index.items[position].entry.path).clicked() {
                chosen = Some(position);
            }
        }
        chosen.and_then(|position| {
            let heading = self.many_heading.take();
            self.heading_document = Some(index.items[position].entry.id);
            self.settle(Go::Item(position, heading))
        })
    }

    /// Draw any item: a plain note, a world entry, or an entry of another kind.
    pub fn read(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
    ) -> Option<Nav> {
        if self.showing != Some(item.entry.id) {
            self.undo = None;
            if let Some(previous) = self.showing {
                self.status_drafts.insert(previous, self.status.clone());
            }
            self.showing = Some(item.entry.id);
            self.status = self
                .status_drafts
                .remove(&item.entry.id)
                .unwrap_or_else(|| status(&item.entry).to_owned());
            self.status_editing = self.status != status(&item.entry);
            // Only a form with changes is worth keeping. It opens again where it was left.
            self.edits.retain(|_, draft| draft.dirty());
            self.other_type.clear();
            self.adding = None;
            self.picker = None;
            self.error = None;
        }
        if self.undo.as_ref().is_some_and(|change| {
            campaign.is_none_or(|campaign| {
                fs::read_to_string(campaign.path.join(&change.path))
                    .ok()
                    .as_deref()
                    != Some(&change.after)
            })
        }) {
            self.undo = None;
        }
        let heading = if self.heading_document.is_none_or(|id| id == item.entry.id) {
            self.heading_document = None;
            self.heading.take()
        } else {
            None
        };
        crate::markdown_view::want_heading(ui.ctx(), heading);
        let go = self.draw(ui, item, index, campaign);
        // A request lasts one draw, so it cannot reach a heading on another page.
        crate::markdown_view::want_heading(ui.ctx(), None);
        go.and_then(|go| self.settle(go))
    }

    fn draw(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
    ) -> Option<Go> {
        let mut go = None;
        if self.place_choices_document != Some(item.entry.id) {
            self.place_choices_open = false;
            self.place_choices_document = Some(item.entry.id);
        }
        ui.ctx()
            .data_mut(|data| data.insert_temp(egui::Id::new("reader_document"), item.entry.id));
        let title = item
            .entry
            .reading()
            .ok()
            .flatten()
            .map_or_else(|| item.entry.name.clone(), |reading| reading.title);
        let id = item.entry.id;
        let edit = campaign.filter(|_| edit::editable(item, index) && !self.editing(id));
        ui.horizontal_top(|ui| {
            let edit_action = Action::quiet("Edit")
                .icon(design::icons::EDIT)
                .density(Density::Read);
            let room = edit.map_or(0.0, |_| edit_action.natural_width(ui) + design::S);
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width() - room, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    view::title_with_art(
                        ui,
                        &title,
                        &format!(
                            "{} · {}",
                            if !self.compact && world_type(&item.entry) == "npc" {
                                "NPC"
                            } else {
                                &item.subtitle
                            },
                            item.owner
                        ),
                        Some(&item.entry),
                    );
                },
            );
            if let Some(campaign) = edit
                && edit_action
                    .name(&format!("Edit {}", item.entry.name))
                    .show(ui)
                    .clicked()
            {
                self.begin_edit(campaign, item);
            }
        });
        self.form_focused = false;
        self.lost_edits(ui, index);
        if let Some(error) = &self.error {
            design::message(ui, Tone::Error, error);
        }
        if let Some(campaign) = campaign
            && self.editing(id)
        {
            return self.edit_form(ui, item, index, campaign);
        }
        self.discarded_notice(ui, id);
        if !self.compact
            && (item.is_world_note() || item.entry.kind == "world")
            && item.is_private()
            && let Some(campaign) = campaign
        {
            file_actions(ui, campaign, &item.entry.path);
        }
        if let Some(campaign) = campaign {
            if item.entry.kind == "world" && !index.duplicate_world.contains_key(&item.entry.path) {
                let stem = Path::new(&item.entry.path)
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy();
                if stem != item.entry.name {
                    let message = format!(
                        "The file is {stem}.md, but the name is {}.",
                        item.entry.name
                    );
                    if design::message_with_action(
                        ui,
                        Tone::Warning,
                        &message,
                        &format!("Rename the note to {stem}"),
                    ) {
                        let result = (|| {
                            let before =
                                fs::read_to_string(note_path(&campaign.path, &item.entry.path)?)
                                    .map_err(|error| error.to_string())?;
                            let after = crate::content_workspace::patch_fields(
                                &before,
                                &[("splat_name", Some(stem.to_string().into()))],
                            )?;
                            let after = crate::content_workspace::rename_first_heading(
                                after,
                                &item.entry.name,
                                &stem,
                            );
                            write_note(&campaign.path, &item.entry.path, Some(&before), &after)?;
                            Ok::<_, String>(Change {
                                path: item.entry.path.clone(),
                                before,
                                after,
                                word: format!("Name changed to {stem}."),
                                kind: ChangeKind::Name,
                            })
                        })();
                        match result {
                            Ok(change) => {
                                self.name_undo = Some(change);
                                return Some(Go::Reload(item.entry.id));
                            }
                            Err(error) => self.error = Some(error),
                        }
                    }
                }
                if let Some(change) = &self.name_undo
                    && change.path == item.entry.path
                    && design::undo_message(ui, Tone::Info, &change.word, "Undo name change")
                {
                    match change.undo(campaign) {
                        Ok(()) => {
                            self.name_undo = None;
                            return Some(Go::Reload(item.entry.id));
                        }
                        Err(error) => self.error = Some(error),
                    }
                }
            }
            if self
                .identity_undo
                .as_ref()
                .is_some_and(|(id, _)| *id != campaign.id)
            {
                self.identity_undo = None;
            }
            if let Some((_, undo)) = &self.identity_undo
                && design::undo_message(ui, Tone::Success, undo.message(), SEPARATE_ENTRY_UNDO)
            {
                match undo.undo(campaign) {
                    Ok(()) => {
                        let id = note_id(&undo.path);
                        self.identity_undo = None;
                        return Some(Go::Reload(id));
                    }
                    Err(error) => self.error = Some(error),
                }
            }
            if let Some(paths) = index.duplicate_world.get(&item.entry.path) {
                design::message(ui, Tone::Warning, DUPLICATE_ENTRY);
                let mut paths = paths.clone();
                let first = crate::content_workspace::duplicate_repair_path(
                    paths.iter().map(|path| campaign.path.join(path)),
                );
                paths.sort_by_key(|path| Some(campaign.path.join(path)) != first);
                for (at, path) in paths.iter().enumerate() {
                    let filename = Path::new(path)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy();
                    let clicked = separate_entry_button(ui, &filename);
                    if clicked {
                        match new_identity(campaign, path) {
                            Ok((undo, id)) => {
                                self.identity_undo = Some((campaign.id, undo));
                                return Some(Go::Reload(id));
                            }
                            Err(error) => self.error = Some(error),
                        }
                    }
                    if at > 0 {
                        ui.label(
                            Type::Small
                                .rich(ui, format!("Existing links will then use {}.", paths[0]))
                                .color(colors().result.warning),
                        );
                    }
                }
            }
        }
        for warning in index.errors.iter().filter(|error| {
            error.starts_with(&format!("{}:", item.entry.path))
                || error.starts_with(&format!("{} is in ", item.entry.name))
        }) {
            design::message(ui, Tone::Warning, warning);
        }
        if item.entry.kind == "world" && !index.duplicate_world.contains_key(&item.entry.path) {
            go = ui
                .scope(|ui| {
                    ui.spacing_mut().item_spacing.y =
                        design::Density::Scan.row() - design::Density::Scan.control();
                    self.entry_controls(ui, item, index, campaign)
                })
                .inner;
        }
        if item.entry.kind == "world" && index.duplicate_world.contains_key(&item.entry.path) {
            for (key, label) in [
                ("creature", "Stat block"),
                ("in", "In"),
                ("status", "Status"),
            ] {
                if let Some(value) = world_field(&item.entry, key) {
                    let name = serde_yaml_ng::from_value::<EntryRef>(value.clone())
                        .ok()
                        .and_then(|reference| reference.reference().ok())
                        .and_then(|reference| index.position_of(&reference))
                        .map(|at| index.items[at].entry.name.clone())
                        .unwrap_or_else(|| link_name(value, label));
                    ui.label(
                        Type::Small
                            .rich(ui, format!("{label}: {name}"))
                            .color(colors().muted),
                    );
                }
            }
        }
        if (item.entry.kind == "world" || item.is_world_note())
            && crate::library::presentation::reading_sections(&item.entry.body, &item.entry.name)
                .is_empty()
        {
            ui.label(
                Type::Small
                    .rich(ui, "Nothing written yet.")
                    .color(colors().muted),
            );
        }
        if item.is_note() {
            let followed = style::measure(ui, |ui| {
                style::markdown(ui, &item.entry.body);
                related(ui, item, index)
            });
            go = go.or(followed);
            if !self.compact && item.is_world_note() {
                go = go.or(self.make_entry_control(ui, item, campaign));
            }
        } else {
            let followed = view::show_with_related(
                ui,
                &item.entry.name,
                Some(&item.entry),
                &item.entry.body,
                &view::Origin {
                    path: &item.entry.path,
                    owner: &item.owner,
                    builtin: item.is_builtin(),
                },
                index,
                |ui| {
                    related(ui, item, index).and_then(|go| {
                        if let Go::Item(at, _) = go {
                            Some(at)
                        } else {
                            None
                        }
                    })
                },
            );
            go = go.or(followed.map(|position| Go::Item(position, None)));
        }

        if let Some(target) = crate::markdown_view::take_note_link(ui.ctx()) {
            let followed = index.follow(&target, item);
            if let Go::Many(positions) = &followed {
                self.choose_link(positions.clone(), &target);
            }
            go = go.or(Some(followed));
        }
        go
    }

    fn make_entry_control(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        campaign: Option<&Campaign>,
    ) -> Option<Go> {
        let campaign = campaign?;
        let mut chosen = None;
        ui.add_space(design::M);
        design::quiet_section(ui, "Give this note a type", |ui| {
            ui.label(
                Type::Small
                    .rich(
                        ui,
                        "Adds a type to the file's front matter. The text does not change.",
                    )
                    .color(colors().muted),
            );
            ui.horizontal_wrapped(|ui| {
                for (label, kind) in [("NPC", "npc"), ("Location", "location")] {
                    if Action::quiet(label)
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                    {
                        chosen = Some(kind.to_owned());
                    }
                }
            });
            design::text_field(
                ui,
                "Another type",
                &mut self.other_type,
                Some("A word of your own, such as faction or quest."),
                None,
            );
            let other = self.other_type.trim().to_lowercase();
            if !other.is_empty()
                && Action::quiet(&format!("Make {other}"))
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
            {
                chosen = Some(other);
            }
        });
        match make_entry(campaign, item, &chosen?) {
            Ok(id) => Some(Go::Reload(id)),
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    fn change(
        &mut self,
        campaign: &Campaign,
        item: &Item,
        index: &ContentIndex,
        key: &str,
        value: Value,
    ) -> Option<Go> {
        let label = match key {
            "in" => "In",
            "part_of" => "Part of",
            "status" => "Status",
            _ => "Stat block",
        };
        let name = |value: &Value| match value.as_sequence() {
            Some(groups) if groups.is_empty() => "not set".into(),
            Some(groups) => groups
                .iter()
                .map(|group| link_name(group, label))
                .collect::<Vec<_>>()
                .join(", "),
            None => edit::value_name(index, value, label),
        };
        // Choosing what is already there changes nothing, and writes no Log line.
        let current = world_field(&item.entry, key).filter(|value| !value.is_null());
        let same = match (key, current) {
            ("part_of", _) => false,
            ("status", _) => value.as_str().map(str::trim) == Some(status(&item.entry)),
            (_, None) => value.is_null(),
            (_, Some(now)) => {
                let target = index.world_value_target(now);
                target.is_some() && target == index.world_value_target(&value)
            }
        };
        if same {
            self.picker = None;
            self.place_choices_open = false;
            return None;
        }
        let before_name = world_field(&item.entry, key).map_or_else(|| "not set".into(), name);
        let after_name = name(&value);
        // What play changes goes in the entry's Log, in the same write, so one Undo takes
        // both back.
        let log = self
            .log_as
            .as_ref()
            .filter(|_| matches!(key, "in" | "part_of" | "status"))
            .map(|stamp| {
                let words = log_words(index, key, world_field(&item.entry, key), &value);
                format!("{stamp}: {words}")
            });
        let value = edit::stored_value(index, value);
        let result = (|| {
            let before = fs::read_to_string(note_path(&campaign.path, &item.entry.path)?)
                .map_err(|e| e.to_string())?;
            let patched = fields::patch(&before, key, value)?;
            let after = match &log {
                Some(line) => lines::add_log(&patched, line),
                None => patched,
            };
            write_note(&campaign.path, &item.entry.path, Some(&before), &after)?;
            Ok::<_, String>(Change {
                path: item.entry.path.clone(),
                before: before.clone(),
                after: after.clone(),
                word: format!("{label}: {before_name} → {after_name}."),
                kind: ChangeKind::of_key(key),
            })
        })();
        match result {
            Ok(change) => {
                self.undo = Some(change);
                self.picker = None;
                self.place_choices_open = false;
                Some(Go::Reload(item.entry.id))
            }
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    fn relationship_width(ui: &egui::Ui) -> f32 {
        let width = design::XXL * 7.0 + design::L;
        width.min(
            (ui.available_width()
                - (design::XXL * 3.0 + design::S)
                - Action::quiet("Change")
                    .density(Density::Scan)
                    .natural_width(ui)
                - ui.spacing().item_spacing.x * 2.0)
                .max(design::XXL),
        )
    }

    fn relationship_value_width(ui: &egui::Ui, item: &Item, index: &ContentIndex) -> f32 {
        let mut values = Vec::new();
        for (key, label) in [("creature", "Stat block"), ("in", "In")] {
            if key == "creature" && !fights(&item.entry) {
                continue;
            }
            let target = if key == "in" {
                index.place_of(item.entry.id)
            } else {
                index.world_target(&item.entry, key)
            };
            let value = if let Some(at) = target {
                index.items[at].entry.name.clone()
            } else if key == "in" && !index.place_choices(item.entry.id).is_empty() {
                let choices = index.place_choices(item.entry.id);
                let name = reader_field(&item.entry, key)
                    .map(|value| link_name(value, label))
                    .unwrap_or_default();
                format!("{} {name} · {} matches", design::icons::WARN, choices.len())
            } else if let Some(value) = reader_field(&item.entry, key) {
                format!(
                    "{} {} · Not found",
                    design::icons::WARN,
                    link_name(value, label)
                )
            } else if key == "creature" {
                "not set".into()
            } else {
                continue;
            };
            let linked =
                target.is_some() || (key == "in" && !index.place_choices(item.entry.id).is_empty());
            values.push((value, if linked { Type::Body } else { Type::Small }));
        }
        let groups = index.groups_of(&item.entry);
        if !groups.is_empty() {
            let names: Vec<&str> = groups.iter().map(|(name, _)| name.as_str()).collect();
            values.push((names.join("  "), Type::Body));
        }
        let status = if item
            .entry
            .flags
            .iter()
            .any(|flag| flag.message.starts_with("Status "))
        {
            "not read"
        } else if status(&item.entry).is_empty() {
            "not set"
        } else {
            status(&item.entry)
        };
        values.push((status.into(), Type::Small));
        values
            .iter()
            .map(|(value, role)| design::number_width(ui, value, &role.font(ui)))
            .fold(0.0_f32, f32::max)
            .min(Self::relationship_width(ui))
    }

    fn place_chooser(
        &self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
    ) -> (Option<Go>, Option<usize>) {
        let mut go = None;
        let mut chosen = None;
        let indent = if self.compact {
            design::number_width(ui, "In", &Type::Small.font(ui)) + ui.spacing().item_spacing.x
        } else if ui.available_width() >= design::XXL * 12.0 {
            design::XXL * 3.0 + design::S + ui.spacing().item_spacing.x
        } else {
            0.0
        };
        egui::Frame::new()
            .inner_margin(egui::Margin {
                left: indent as i8,
                ..Default::default()
            })
            .show(ui, |ui| {
                ui.label(
                    Type::Small
                        .rich(ui, "This link matches more than one note. Choose one.")
                        .color(colors().muted),
                );
                let choices = index.place_choices(item.entry.id);
                let file_width = choices
                    .iter()
                    .map(|&at| {
                        design::number_width(ui, &index.items[at].entry.path, &Type::Body.font(ui))
                    })
                    .fold(0.0_f32, f32::max)
                    .min(
                        (ui.available_width()
                            - Action::secondary("Always use this one").natural_width(ui)
                            - design::L)
                            .max(design::XXL),
                    );
                for at in choices {
                    let path = &index.items[at].entry.path;
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = design::L;
                        let name = format!("Always use {path}");
                        let action = Action::secondary("Always use this one").name(&name);
                        ui.allocate_ui_with_layout(
                            egui::vec2(file_width, Density::Read.control()),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                ui.set_width(file_width);
                                if design::wrapped_link(ui, path).clicked() {
                                    go = Some(Go::Item(at, None));
                                }
                            },
                        );
                        if action.show(ui).clicked() {
                            chosen = Some(at);
                        }
                    });
                }
            });
        (go, chosen)
    }

    fn relationship_row(
        ui: &mut egui::Ui,
        label: &str,
        width: f32,
        value: impl FnOnce(&mut egui::Ui),
        notice: Option<(&str, ChangeKind)>,
    ) -> (bool, bool) {
        let name = format!("Change {label}");
        let change = Action::quiet("Change")
            .density(design::Density::Scan)
            .name(&name);
        let stacked = ui.available_width() < design::XXL * 12.0;
        let width = width
            .min((ui.available_width() - change.natural_width(ui) - design::L).max(design::XXL));
        if stacked {
            ui.label(Type::Small.rich(ui, label).color(colors().muted));
        }
        let mut undo = false;
        let clicked = ui
            .horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    if !stacked {
                        ui.allocate_ui_with_layout(
                            egui::vec2(
                                design::XXL * 3.0 + design::S,
                                design::Density::Scan.control(),
                            ),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                ui.set_min_width(design::XXL * 3.0 + design::S);
                                ui.label(Type::Small.rich(ui, label).color(colors().muted));
                            },
                        );
                    }
                    ui.allocate_ui_with_layout(
                        egui::vec2(width, design::Density::Scan.control()),
                        egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true),
                        |ui| {
                            ui.set_width(width);
                            value(ui);
                        },
                    );
                    ui.add_space((design::L - ui.spacing().item_spacing.x).max(0.0));
                    let clicked = change.show(ui).clicked();
                    if let Some((word, kind)) = notice {
                        ui.allocate_ui_with_layout(
                            egui::vec2(ui.available_width(), Density::Scan.control()),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                undo = Self::change_notice(ui, word, kind);
                            },
                        );
                    }
                    clicked
                })
                .inner
            })
            .inner;
        (clicked, undo)
    }

    fn notice_fits(ui: &egui::Ui, value_width: f32, word: &str, kind: ChangeKind) -> bool {
        ui.available_width()
            >= design::XXL * 3.0
                + design::S
                + value_width
                + Action::quiet("Change")
                    .density(Density::Scan)
                    .natural_width(ui)
                + design::number_width(ui, word, &Type::Body.font(ui))
                + design::S
                + design::HAIRLINE_WIDTH
                + design::number_width(ui, design::icons::INFO, &Type::Body.font(ui))
                - design::XS
                + Action::quiet(kind.undo_label())
                    .icon(design::icons::UNDO)
                    .density(Density::Read)
                    .natural_width(ui)
                + ui.spacing().item_spacing.x * 6.0
    }

    fn change_notice(ui: &mut egui::Ui, word: &str, kind: ChangeKind) -> bool {
        design::undo_message(ui, Tone::Info, word, kind.undo_label())
    }

    /// The change to show beside this row, when it is this row's and there is room.
    fn row_notice(
        &self,
        ui: &egui::Ui,
        item: &Item,
        kind: ChangeKind,
        value_width: f32,
    ) -> Option<(String, ChangeKind)> {
        self.undo
            .as_ref()
            .filter(|change| {
                change.path == item.entry.path
                    && change.kind == kind
                    && Self::notice_fits(ui, value_width, &change.word, kind)
            })
            .map(|change| (change.word.clone(), kind))
    }

    pub fn clear_notice(&mut self) {
        self.undo = None;
    }

    fn undo_change(&mut self, campaign: &Campaign, id: Uuid) -> Option<Go> {
        match self.undo.as_ref()?.undo(campaign) {
            Ok(_) => {
                self.undo = None;
                Some(Go::Reload(id))
            }
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    fn relationships(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: &Campaign,
    ) -> Option<Go> {
        let mut go = None;
        let value_width = Self::relationship_value_width(ui, item, index);
        for (key, label) in [("creature", "Stat block"), ("in", "In")] {
            if key == "creature" && !fights(&item.entry) {
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
            let notice = self.row_notice(ui, item, ChangeKind::of_key(key), value_width);
            ui.push_id(key, |ui| {
                let (changed, undo) = Self::relationship_row(
                    ui,
                    label,
                    value_width,
                    |ui| {
                        if let Some(at) = target {
                            if design::wrapped_link(ui, &index.items[at].entry.name).clicked() {
                                go = Some(Go::Item(at, None));
                            }
                        } else if key == "in" && !index.place_choices(item.entry.id).is_empty() {
                            let choices = index.place_choices(item.entry.id);
                            let name = reader_field(&item.entry, key)
                                .map(|value| link_name(value, label))
                                .unwrap_or_default();
                            if design::wrapped_link(
                                ui,
                                &format!(
                                    "{} {name} · {} matches",
                                    design::icons::WARN,
                                    choices.len()
                                ),
                            )
                            .clicked()
                            {
                                self.place_choices_open = !self.place_choices_open;
                            }
                        } else if let Some(value) = reader_field(&item.entry, key) {
                            let words = format!(
                                "{} {} · Not found",
                                design::icons::WARN,
                                link_name(value, label)
                            );
                            let fits = design::number_width(ui, &words, &Type::Small.font(ui))
                                <= ui.available_width();
                            if fits {
                                ui.label(
                                    Type::Small.rich(ui, &words).color(colors().result.warning),
                                );
                            } else {
                                ui.vertical(|ui| {
                                    let (name, state) = words.split_once(" · ").unwrap();
                                    ui.add(
                                        egui::Label::new(
                                            Type::Small
                                                .rich(ui, name)
                                                .color(colors().result.warning),
                                        )
                                        .wrap(),
                                    );
                                    ui.label(
                                        Type::Small.rich(ui, state).color(colors().result.warning),
                                    );
                                });
                            }
                        } else {
                            ui.label(Type::Small.rich(ui, "not set").color(colors().muted));
                        }
                    },
                    notice.as_ref().map(|(word, kind)| (word.as_str(), *kind)),
                );
                if changed {
                    self.picker = Some(Picker {
                        key,
                        focus: true,
                        ..Default::default()
                    });
                }
                if undo {
                    go = self.undo_change(campaign, item.entry.id);
                }
            });
            if key == "in" && self.place_choices_open {
                let (followed, chosen) = self.place_chooser(ui, item, index);
                go = go.or(followed);
                if let Some(at) = chosen {
                    let path = index.items[at].entry.path.trim_end_matches(".md");
                    return self.change(campaign, item, index, "in", format!("[[{path}]]").into());
                }
            }
        }
        let groups = index.groups_of(&item.entry);
        let notice = self.row_notice(ui, item, ChangeKind::PartOf, value_width);
        if !groups.is_empty() {
            ui.push_id("part_of", |ui| {
                let (changed, undo) = Self::relationship_row(
                    ui,
                    "Part of",
                    value_width,
                    |ui| {
                        for (name, target) in groups {
                            match target {
                                Some(at) => {
                                    if design::wrapped_link(ui, &index.items[*at].entry.name)
                                        .clicked()
                                    {
                                        go = Some(Go::Item(*at, None));
                                    }
                                }
                                None => {
                                    ui.label(
                                        Type::Small
                                            .rich(
                                                ui,
                                                format!(
                                                    "{} {name} · Not found",
                                                    design::icons::WARN
                                                ),
                                            )
                                            .color(colors().result.warning),
                                    );
                                }
                            }
                        }
                    },
                    notice.as_ref().map(|(word, kind)| (word.as_str(), *kind)),
                );
                if changed {
                    self.picker = Some(Picker {
                        key: "part_of",
                        focus: true,
                        ..Default::default()
                    });
                }
                if undo {
                    go = self.undo_change(campaign, item.entry.id);
                }
            });
        }
        if let Some(key) = self.picker.as_ref().map(|picker| picker.key)
            && let Some(value) = self.pick(ui, item, index, campaign, None)
        {
            return self.change(campaign, item, index, key, value);
        }
        go
    }

    /// The open picker. Returns the value the DM chose, or `Value::Null` to clear it.
    fn pick(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: &Campaign,
        draft_groups: Option<&[Value]>,
    ) -> Option<Value> {
        let picker = self.picker.as_mut()?;
        let key = picker.key;
        let label = match key {
            "in" => "Find a location",
            "part_of" => "Find a group",
            _ => "Find a stat block",
        };
        let groups: Vec<Value> = draft_groups.map_or_else(
            || part_of(&item.entry).into_iter().cloned().collect(),
            <[Value]>::to_vec,
        );
        let joined: Vec<usize> = groups
            .iter()
            .filter_map(|value| index.world_value_target(value))
            .collect();
        let width = ui.available_width();
        let response = design::search_field(ui, label, &mut picker.search, label, width);
        if std::mem::take(&mut picker.focus) {
            response.request_focus();
        }
        let matches = if key == "creature" {
            index.creatures_matching(&picker.search, 5)
        } else {
            // Anything can hold a place or a member. Locations come first for a place.
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
        let mut chosen = None;
        let mut cancel = false;
        for at in matches {
            let entry = &index.items[at].entry;
            if design::link(ui, &entry.name).clicked() {
                chosen = if key == "part_of" {
                    let mut groups = groups.clone();
                    groups.push(index.link_to(at).into());
                    Some(Value::Sequence(groups))
                } else {
                    serde_yaml_ng::to_value(EntryRef::id(entry.id)).ok()
                };
            }
        }
        if key == "part_of" {
            for (at, group) in groups.iter().enumerate() {
                let name = format!("Remove {}", link_name(group, "group"));
                if Action::quiet(&name).show(ui).clicked() {
                    let mut groups = groups.clone();
                    groups.remove(at);
                    chosen = Some(Value::Sequence(groups));
                }
            }
        }
        if key == "in" {
            design::quiet_section(ui, "New location…", |ui| {
                design::text_field(ui, "Location name", &mut picker.new_location, None, None);
                let reason = picker
                    .new_location
                    .trim()
                    .is_empty()
                    .then_some("Type a name first.");
                ui.horizontal_wrapped(|ui| {
                    if Action::secondary("Create location here")
                        .blocked(reason)
                        .show(ui)
                        .clicked()
                    {
                        match create_entry(campaign, &picker.new_location, "location") {
                            Ok(id) => {
                                let mut target = EntryRef::id(id);
                                target.link = Some(picker.new_location.trim().to_owned());
                                chosen = serde_yaml_ng::to_value(target).ok();
                            }
                            Err(error) => self.error = Some(error),
                        }
                    }
                    design::blocked_reason(ui, reason);
                });
            });
        }
        ui.horizontal_wrapped(|ui| {
            if key != "part_of" && Action::quiet("Clear value").show(ui).clicked() {
                chosen = Some(Value::Null);
            }
            cancel = Action::quiet("Cancel").show(ui).clicked();
        });
        if chosen.is_some()
            || cancel
            || ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        {
            self.picker = None;
        }
        chosen
    }

    fn entry_controls(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
    ) -> Option<Go> {
        let campaign = campaign.filter(|_| item.is_private())?;
        let mut go = None;
        if self.compact {
            ui.horizontal_wrapped(|ui| {
                let mut targets = vec![
                    ("Stat block", index.world_target(&item.entry, "creature")),
                    ("In", index.place_of(item.entry.id)),
                ];
                targets.extend(
                    index
                        .groups_of(&item.entry)
                        .iter()
                        .map(|(_, group)| ("Part of", *group)),
                );
                let mut first = true;
                for (label, target) in targets {
                    if let Some(at) = target {
                        if !first {
                            ui.label(Type::Small.rich(ui, "·").color(colors().muted));
                        }
                        first = false;
                        ui.label(Type::Small.rich(ui, label).color(colors().muted));
                        if design::link(ui, &index.items[at].entry.name).clicked() {
                            go = Some(Go::Item(at, None));
                        }
                    }
                }
            });
            if broken_links(item, index).contains(&"Stat block not found") {
                design::status(
                    ui,
                    design::Status {
                        icon: design::icons::WARN,
                        word: &format!(
                            "Stat block: {} · Not found",
                            reader_field(&item.entry, "creature")
                                .map(|v| link_name(v, "Stat block"))
                                .unwrap_or_else(|| "Stat block".into())
                        ),
                        color: colors().result.warning,
                    },
                );
            }
            let choices = index.place_choices(item.entry.id);
            if !choices.is_empty() {
                ui.horizontal(|ui| {
                    ui.label(Type::Small.rich(ui, "In").color(colors().muted));
                    let name = world_field(&item.entry, "in")
                        .map(|value| link_name(value, "Place"))
                        .unwrap_or_default();
                    if design::wrapped_link(
                        ui,
                        &format!("{} {name} · {} matches", design::icons::WARN, choices.len()),
                    )
                    .clicked()
                    {
                        self.place_choices_open = !self.place_choices_open;
                    }
                });
                if self.place_choices_open {
                    let (followed, chosen) = self.place_chooser(ui, item, index);
                    go = go.or(followed);
                    if let Some(at) = chosen {
                        let path = index.items[at].entry.path.trim_end_matches(".md");
                        return self.change(
                            campaign,
                            item,
                            index,
                            "in",
                            format!("[[{path}]]").into(),
                        );
                    }
                }
            }
            if let Some(change) = &self.undo
                && change.path == item.entry.path
                && Self::change_notice(ui, &change.word, change.kind)
            {
                return self.undo_change(campaign, item.entry.id);
            }
            if index.place_of(item.entry.id).is_none()
                && index.place_choices(item.entry.id).is_empty()
                && let Some(place) = reader_field(&item.entry, "in").and_then(Value::as_str)
            {
                ui.label(
                    Type::Small
                        .rich(
                            ui,
                            format!(
                                "{} In {} · Not found",
                                design::icons::WARN,
                                place.trim_matches(['[', ']'])
                            ),
                        )
                        .color(colors().result.warning),
                );
            }
            if go.is_some() {
                return go;
            }
        }
        if !self.compact
            && let Some(go) = self.relationships(ui, item, index, campaign)
        {
            return Some(go);
        }
        if !self.compact {
            let value_width = Self::relationship_value_width(ui, item, index);
            if !self.status_editing
                && (!status(&item.entry).trim().is_empty()
                    || item
                        .entry
                        .flags
                        .iter()
                        .any(|flag| flag.message.starts_with("Status ")))
            {
                let value = status(&item.entry);
                let unread = item
                    .entry
                    .flags
                    .iter()
                    .any(|flag| flag.message.starts_with("Status "));
                let notice = self.row_notice(ui, item, ChangeKind::Status, value_width);
                let (changed, undo) = Self::relationship_row(
                    ui,
                    "Status",
                    value_width,
                    |ui| {
                        ui.label(
                            Type::Small
                                .rich(
                                    ui,
                                    if unread {
                                        "not read"
                                    } else if value.is_empty() {
                                        "not set"
                                    } else {
                                        value
                                    },
                                )
                                .color(if unread {
                                    colors().result.warning
                                } else {
                                    colors().muted
                                }),
                        );
                    },
                    notice.as_ref().map(|(word, kind)| (word.as_str(), *kind)),
                );
                if undo {
                    return self.undo_change(campaign, item.entry.id);
                }
                if changed {
                    self.status_editing = true;
                }
            }
            if self.status_editing {
                let response = design::text_field(ui, "Status", &mut self.status, None, None);
                let wanted = self.status.trim().to_owned();
                if wanted != status(&item.entry) {
                    let mut save = response.has_focus()
                        && ui.input(|input| input.key_pressed(egui::Key::Enter));
                    ui.horizontal(|ui| {
                        design::status(
                            ui,
                            design::Status {
                                icon: design::icons::DRAFT,
                                word: "Unsaved",
                                color: colors().document.unsaved,
                            },
                        );
                        save |= Action::secondary("Save status")
                            .density(Density::Read)
                            .show(ui)
                            .clicked();
                    });
                    if save {
                        let result = self.change(campaign, item, index, "status", wanted.into());
                        if result.is_some() {
                            self.status_editing = false;
                            self.status_drafts.remove(&item.entry.id);
                        }
                        return result;
                    }
                }
            }
        }
        if !self.compact
            && let Some(change) = &self.undo
            && change.path == item.entry.path
            // A row shows its own change when it fits. Any other change shows here.
            && (!Self::notice_fits(
                ui,
                Self::relationship_value_width(ui, item, index),
                &change.word,
                change.kind,
            ) || !change.kind.has_row()
                || (change.kind == ChangeKind::Status && (self.status_editing || status(&item.entry).is_empty()))
                || (change.kind == ChangeKind::Place && reader_field(&item.entry, "in").is_none())
                || (change.kind == ChangeKind::PartOf && index.groups_of(&item.entry).is_empty()))
            && ui
                .horizontal_wrapped(|ui| Self::change_notice(ui, &change.word, change.kind))
                .inner
        {
            return self.undo_change(campaign, item.entry.id);
        }
        if !fights(&item.entry) {
            return None;
        }
        if let Some((index, initiative)) = self.in_run.get(&item.entry.id) {
            let word = format!(
                "In the run · initiative {}",
                initiative.map_or_else(|| "unknown".to_owned(), |n| n.to_string())
            );
            if design::status_link(
                ui,
                design::Status {
                    icon: design::icons::RUN,
                    word: &word,
                    color: colors().accent,
                },
            )
            .clicked()
            {
                self.select_run = Some(*index);
            }
            return None;
        }
        let sides = self.sides.as_ref()?;
        let Some(adding) = &mut self.adding else {
            if Action::secondary("Add to fight")
                .icon(design::icons::ADD)
                .density(Density::Read)
                .show(ui)
                .clicked()
            {
                self.adding = Some(Adding {
                    stat_block: index.world_target(&item.entry, "creature"),
                    save: true,
                    side: if sides.is_empty() || sides.iter().any(|side| side == "enemies") {
                        "enemies".to_owned()
                    } else {
                        sides[0].clone()
                    },
                    ..Default::default()
                });
            }
            return None;
        };
        match add_form(ui, item, index, sides, adding)? {
            FormEnd::Cancel => self.adding = None,
            FormEnd::Add => {
                let stat_block = &index.items[adding.stat_block?].entry;
                let total = match adding.initiative.trim() {
                    "" => None,
                    text => match text.parse::<i64>() {
                        Ok(total) => Some(total),
                        Err(_) => {
                            self.error = Some("Initiative must be a whole number.".to_owned());
                            return None;
                        }
                    },
                };
                let saved = adding.save && index.world_target(&item.entry, "creature").is_none();
                if saved {
                    match set_world_field(
                        campaign,
                        &item.entry,
                        "creature",
                        stored_link(stat_block).into(),
                    ) {
                        Ok(mut change) => {
                            change.word = format!("Stat block: not set → {}.", stat_block.name);
                            self.undo = Some(change);
                        }
                        Err(error) => self.error = Some(error),
                    }
                }
                let group = npc_group(&item.entry, stat_block, &adding.side);
                self.adding = None;
                return Some(Go::Join(Join {
                    group,
                    total,
                    saved,
                }));
            }
        }
        None
    }
}

/// Choose the stat block, side and initiative. Returns how the form ended, if it did.
fn add_form(
    ui: &mut egui::Ui,
    item: &Item,
    index: &ContentIndex,
    sides: &[String],
    adding: &mut Adding,
) -> Option<FormEnd> {
    ui.label(Type::Strong.rich(ui, format!("Add {} to the fight", item.entry.name)));
    match adding.stat_block.map(|at| &index.items[at].entry) {
        Some(stat_block) => {
            ui.horizontal_wrapped(|ui| {
                ui.label(Type::Small.rich(ui, "Stat block").color(colors().muted));
                ui.label(Type::Body.rich(ui, &stat_block.name).color(colors().text));
                if Action::quiet("Change")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    adding.search = stat_block.name.clone();
                    adding.stat_block = None;
                }
            });
        }
        None => {
            let width = ui.available_width();
            design::search_field(
                ui,
                "Stat block",
                &mut adding.search,
                "Stat block: Commoner, Guard…",
                width,
            );
            for position in index.creatures_matching(&adding.search, 5) {
                if design::link(ui, &index.items[position].entry.name).clicked() {
                    adding.stat_block = Some(position);
                }
            }
        }
    }
    if creature(&item.entry).is_none() {
        ui.checkbox(&mut adding.save, "Save this as their stat block");
    }
    if sides.len() > 1 {
        design::small_field(ui, "Side", None, None, |ui| {
            let combo = egui::ComboBox::from_id_salt("world_add_side")
                .selected_text(design::reader::human_label(&adding.side))
                .icon(design::combo_icon)
                .show_ui(ui, |ui| {
                    for side in sides {
                        ui.selectable_value(
                            &mut adding.side,
                            side.clone(),
                            design::reader::human_label(side),
                        );
                    }
                });
            design::named(ui, &combo.response, "Side");
        });
    }
    design::small_field(
        ui,
        "Initiative",
        Some("Leave empty to roll it."),
        None,
        |ui| {
            design::text_input(
                ui,
                "Initiative",
                &mut adding.initiative,
                "",
                design::L * 5.0,
            );
        },
    );
    let mut end = None;
    ui.horizontal(|ui| {
        if Action::secondary("Add to fight")
            .density(Density::Read)
            .blocked_if(adding.stat_block.is_none(), "Choose a stat block first.")
            .show(ui)
            .clicked()
        {
            end = Some(FormEnd::Add);
        }
        if Action::quiet("Cancel")
            .density(Density::Read)
            .show(ui)
            .clicked()
        {
            end = Some(FormEnd::Cancel);
        }
    });
    end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter8_optional_links_warn_only_when_set_and_unresolved() {
        for (value, expected) in [
            ("null", false),
            ("''", false),
            ("'  '", false),
            ("'[[Nowhere]]'", true),
        ] {
            for (key, warning) in [
                ("in", "Place not found"),
                ("creature", "Stat block not found"),
            ] {
                let dir = tempfile::tempdir().unwrap();
                let campaign = sample_campaign(dir.path());
                let id = create_entry(&campaign, "Rook", "npc").unwrap();
                let path = dir.path().join("NPCs/Rook.md");
                let field = if key == "creature" {
                    "splat_stat_block"
                } else {
                    "splat_in"
                };
                let file = fs::read_to_string(&path).unwrap().replacen(
                    "---\n",
                    &format!("---\n{field}: {value}\n"),
                    1,
                );
                fs::write(path, file).unwrap();
                let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
                let item = &index.items[index.position(id).unwrap()];
                assert_eq!(
                    broken_links(item, &index).contains(&warning),
                    expected,
                    "FD-1-{key}-{value}"
                );
            }
        }
    }

    #[test]
    fn iter8_d2_compact_reader_warns_only_for_nonempty_unresolved_links() {
        let mut actual = Vec::new();
        let mut expected = Vec::new();
        for key in ["splat_in", "splat_stat_block"] {
            for (value, warning) in [("''", false), ("'  '", false), ("'[[Nowhere]]'", true)] {
                let dir = tempfile::tempdir().unwrap();
                let campaign = sample_campaign(dir.path());
                let id = create_entry(&campaign, "Rook", "npc").unwrap();
                let path = dir.path().join("NPCs/Rook.md");
                let file = fs::read_to_string(&path).unwrap().replacen(
                    "---\n",
                    &format!("---\n{key}: {value}\n"),
                    1,
                );
                fs::write(path, file).unwrap();
                let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
                let item = &index.items[index.position(id).unwrap()];
                let mut reader = Reader {
                    compact: true,
                    sides: Some(Vec::new()),
                    ..Default::default()
                };
                let ctx = egui::Context::default();
                crate::design::theme::install(&ctx);
                ctx.enable_accesskit();
                let output = ctx.run(Default::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        reader.entry_controls(ui, item, &index, Some(&campaign));
                    });
                });
                let nodes = output.platform_output.accesskit_update.unwrap().nodes;
                let labels: Vec<_> = nodes
                    .iter()
                    .filter_map(|(_, node)| node.label().or_else(|| node.value()))
                    .collect();
                assert!(
                    labels.contains(&"Add to fight"),
                    "D2-compact-fixture: compact reader controls must render"
                );
                actual.push((
                    key,
                    value,
                    labels.iter().any(|label| label.contains("Not found")),
                ));
                expected.push((key, value, warning));
            }
        }
        assert_eq!(
            actual, expected,
            "D2-compact: empty optional links have no warning; unresolved links keep it"
        );
    }

    #[test]
    fn i19_h1_world_notice_names_the_change_and_undo() {
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                Reader::change_notice(ui, "In: Watch House → North Gate.", ChangeKind::Place);
            });
        });
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        let labels: Vec<_> = nodes.iter().filter_map(|(_, node)| node.label()).collect();
        assert_eq!(
            labels
                .iter()
                .filter(|s| s.starts_with("Undo"))
                .copied()
                .collect::<Vec<_>>(),
            ["Undo place change"]
        );
    }

    #[test]
    fn integrity_d27_focus_checks_include_run_json_files() {
        let dir = tempfile::tempdir().unwrap();
        let runs = dir.path().join("Encounter Runs");
        fs::create_dir_all(&runs).unwrap();
        let before = fingerprint(dir.path());
        let path = runs.join("fight.json");
        fs::write(&path, "run").unwrap();
        let saved = fingerprint(dir.path());
        assert_ne!(saved, before);
        fs::write(&path, "DM changed the run").unwrap();
        assert_ne!(fingerprint(dir.path()), saved);
    }

    fn note(dir: &Path, path: &str, text: &str) {
        let path = dir.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    #[test]
    fn renaming_links_keeps_aliases_headings_code_and_other_targets() {
        let text = "[[Cinder Wolf]] [[Cinder Wolf#Bite|fang]] [[Other/Cinder Wolf]] `[[Cinder Wolf]]`\n```md\n[[Cinder Wolf]]\n```\n";
        let (after, count) = rename_note_links(text, "Cinder Wolf", "Ash Wolf");
        assert_eq!(count, 2);
        assert_eq!(
            after,
            text.replacen("[[Cinder Wolf]]", "[[Ash Wolf]]", 1)
                .replace("[[Cinder Wolf#Bite|fang]]", "[[Ash Wolf#Bite|fang]]")
        );
    }

    #[test]
    fn world_identity_repair_writes_only_its_file_and_undo_keeps_later_edits() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Test".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let before = splat::content::formats::patch_model(
            &crate::library::note_text("Rook", "world", None),
            &[(
                "splat_world",
                Some(serde_yaml_ng::from_str("{type: npc}").unwrap()),
            )],
        )
        .unwrap();
        fs::write(dir.path().join("Rook.md"), &before).unwrap();
        fs::write(dir.path().join("Rook copy.md"), &before).unwrap();
        let (undo, id) = super::new_identity(&campaign, "Rook copy.md").unwrap();
        assert_eq!(
            fs::read_to_string(dir.path().join("Rook.md")).unwrap(),
            before
        );
        assert_eq!(Entry::parse("copy.md", &undo.after).unwrap().id, id);
        let repaired = Entry::parse("copy.md", &undo.after).unwrap();
        assert_eq!(repaired.name, "Rook copy");
        assert!(repaired.body.contains("# Rook copy\n"));
        undo.undo(&campaign).unwrap();
        assert_eq!(
            fs::read_to_string(dir.path().join("Rook copy.md")).unwrap(),
            before
        );
        let (undo, _) = super::new_identity(&campaign, "Rook copy.md").unwrap();
        fs::write(
            dir.path().join("Rook copy.md"),
            format!("{}\nExternal edit.", undo.after),
        )
        .unwrap();
        assert!(undo.undo(&campaign).is_err());
    }

    #[test]
    fn a_note_is_found_by_its_role_and_not_its_folder() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let sessions = root.join("Sessions");
        note(root, "Setting.md", "# Setting\n");
        note(root, "Powers/Verori.md", "# Verori\n");
        note(root, "Sessions/Beside the sessions.md", "World\n");
        note(
            root,
            "Sessions/Session 1.md",
            "---\nsession_id: x\n---\nWe met.\n",
        );
        note(
            root,
            "Sessions/Timeline.md",
            "---\ntimeline_version: 1\n---\n",
        );
        note(root, "Sessions/Templates/Lazy.md", "Template\n");
        note(root, "Sessions/Recordings/a/a.transcript.md", "raw\n");
        note(
            root,
            "Sessions/Recordings/a/a.marked.md",
            "---\nmarked_model: x\n---\nbest\n",
        );
        note(root, "Sessions/Recordings/a/a.review.md", "no\n");
        note(root, "Sessions/Recordings/b/b.transcript.md", "only\n");
        note(root, "Trackers.md", "---\ntrackers_version: 1\n---\n");
        note(root, "Entry.md", "---\nsplat_schema: 1\n---\n");
        note(root, ".obsidian/plugins/a/README.md", "no\n");
        note(root, ".trash/Old.md", "no\n");
        fs::write(root.join("Map.PNG"), b"x").unwrap();
        let scan = scan(root, &sessions);
        let found: Vec<_> = scan
            .notes
            .iter()
            .map(|n| (n.path.as_str(), n.role))
            .collect();
        assert_eq!(
            found,
            [
                ("Powers/Verori.md", Role::World),
                ("Sessions/Beside the sessions.md", Role::World),
                ("Sessions/Recordings/a/a.marked.md", Role::Transcript),
                ("Sessions/Recordings/b/b.transcript.md", Role::Transcript),
                ("Sessions/Session 1.md", Role::Session),
                ("Setting.md", Role::World),
            ]
        );
        assert_eq!(scan.notes[2].body, "best\n");
        assert!(scan.images.contains_key("map.png"));
    }

    #[test]
    fn a_session_folder_at_the_root_hides_no_world_note() {
        let dir = tempfile::tempdir().unwrap();
        note(dir.path(), "Setting.md", "# Setting\n");
        note(dir.path(), "Timeline.md", "---\ntimeline_version: 1\n---\n");
        let scan = scan(dir.path(), dir.path());
        assert_eq!(scan.notes.len(), 1);
        assert_eq!(scan.notes[0].path, "Setting.md");
    }

    #[test]
    fn links_become_reader_links_outside_code() {
        let images = HashMap::from([("map.png".to_owned(), PathBuf::from("/c/Map.png"))]);
        let body = "See [[Verori]], [[Gate Mechanics#Reliability]] and [[Kaun|the East]].\n\
                    ![[Map.png]]\n![[Gone.png]]\n`[[code]]`\n```\n[[fence]]\n```\n";
        let text = with_links(body, &images);
        assert!(text.contains("[Verori](<splat-note:Verori>)"));
        assert!(text.contains("[Reliability](<splat-note:Gate Mechanics#Reliability>)"));
        assert!(text.contains("[the East](<splat-note:Kaun>)"));
        assert!(text.contains("[image](<splat-file:/c/Map.png>)"));
        assert!(text.contains("(Image not found: Gone.png)"));
        assert!(text.contains("`[[code]]`"));
        assert!(text.contains("\n[[fence]]\n"));
        assert_eq!(
            links_in(body),
            ["verori", "gate mechanics", "kaun", "map.png", "gone.png"]
        );
    }

    #[test]
    fn headings_skip_code_and_keep_their_words() {
        assert_eq!(
            headings("# One\ntext\n## Two ##\n```\n# Not\n```\n#nope\n### Three\n"),
            ["One", "Two", "Three"]
        );
    }

    #[test]
    fn a_new_entry_parses_and_never_replaces_a_note() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let id = create_entry(&campaign, "Ashori: Deepcut", "npc").unwrap();
        let path = "NPCs/Ashori Deepcut.md";
        let text = fs::read_to_string(dir.path().join(path)).unwrap();
        let entry = Entry::parse(path, &text).unwrap();
        assert_eq!(entry.id, id);
        assert_eq!(entry.name, "Ashori: Deepcut");
        assert_eq!(world_type(&entry), "npc");
        assert_eq!(status(&entry), "alive");
        assert!(create_entry(&campaign, "Ashori: Deepcut", "npc").is_err());
        assert!(create_entry(&campaign, " :: ", "npc").is_err());
        set_world_field(&campaign, &entry, "status", "dead".into()).unwrap();
        let text = fs::read_to_string(dir.path().join(path)).unwrap();
        let entry = Entry::parse(path, &text).unwrap();
        assert_eq!(status(&entry), "dead");
        assert_eq!(world_type(&entry), "npc", "the rest of the entry stays");
        assert!(text.ends_with("# Ashori: Deepcut\n"), "the body stays");
    }

    #[test]
    fn a_generated_npc_saves_its_attributes_and_never_replaces_a_note() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let npc = crate::npc_generator::Generated {
            name: "Tally".into(),
            attributes: ["Ink".into(), "Bored".into(), "A clean writ".into()],
        };
        let id = create_generated(&campaign, &npc).unwrap();
        let path = "NPCs/Tally.md";
        let text = fs::read_to_string(dir.path().join(path)).unwrap();
        let entry = Entry::parse(path, &text).unwrap();
        assert_eq!(entry.id, id);
        assert_eq!(world_type(&entry), "npc");
        assert_eq!(status(&entry), "alive");
        assert!(text.ends_with(
            "# Tally\n\n- **Look:** Ink\n- **Manner:** Bored\n- **Want:** A clean writ\n"
        ));
        let again = create_generated(&campaign, &npc).unwrap_err();
        assert!(again.contains("already exists"), "{again}");
        assert_eq!(fs::read_to_string(dir.path().join(path)).unwrap(), text);
    }
    fn sample_campaign(root: &Path) -> Campaign {
        Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        }
    }

    #[test]
    fn plain_note_creation_uses_a_stable_path_identity_and_never_replaces_a_file() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let id = create_entry(&campaign, "Watch notes", "").unwrap();
        assert_eq!(id, note_id("Notes/Watch notes.md"));
        let path = dir.path().join("Notes/Watch notes.md");
        assert_eq!(fs::read_to_string(&path).unwrap(), "# Watch notes\n");
        assert!(create_entry(&campaign, "Watch notes", "").is_err());
        assert_eq!(
            scan(dir.path(), &dir.path().join("Sessions")).notes.len(),
            1
        );
    }

    #[test]
    fn iter3_d_relationship_changes_share_one_column() {
        for width in [300.0, 600.0, 1000.0] {
            let ctx = egui::Context::default();
            design::theme::install(&ctx);
            let mut output = None;
            for _ in 0..2 {
                output = Some(ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 640.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            // The value column is the widest value, as in the reader.
                            let width = ["Goblin Warrior", "North Gate", "not read"]
                                .iter()
                                .map(|value| design::number_width(ui, value, &Type::Small.font(ui)))
                                .fold(0.0_f32, f32::max)
                                .min(Reader::relationship_width(ui));
                            for (label, value) in [
                                ("Stat block", "Goblin Warrior"),
                                ("In", "North Gate"),
                                ("Status", "not read"),
                            ] {
                                Reader::relationship_row(
                                    ui,
                                    label,
                                    width,
                                    |ui| {
                                        ui.label(Type::Small.rich(ui, value));
                                    },
                                    None,
                                );
                            }
                        });
                    },
                ));
            }
            let output = output.unwrap();
            let widest_value = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text)
                        if ["Goblin Warrior", "North Gate", "not read"]
                            .contains(&text.galley.job.text.as_str()) =>
                    {
                        Some(text.pos.x + text.galley.size().x)
                    }
                    _ => None,
                })
                .fold(0.0_f32, f32::max);
            let changes: Vec<_> = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.job.text == "Change" => Some(text.pos.x),
                    _ => None,
                })
                .collect();
            assert_eq!(changes.len(), 3);
            assert!(
                changes.iter().all(|x| (x - changes[0]).abs() < 1.0),
                "{width}: {changes:?}"
            );
            // Change follows the widest value after an L gap (plus item spacing), not the cap.
            assert!(
                changes[0] <= widest_value + design::L + design::S + 1.0,
                "{width}: Change at {} after a widest value ending at {widest_value}",
                changes[0]
            );
        }
    }

    #[test]
    fn undo_restores_exact_bytes_and_refuses_a_later_disk_edit() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        create_entry(&campaign, "Rook", "npc").unwrap();
        let path = "NPCs/Rook.md";
        let before = fs::read_to_string(dir.path().join(path)).unwrap();
        let after = fields::patch(&before, "in", "[[Ashfall Keep]]".into()).unwrap();
        write_note(dir.path(), path, Some(&before), &after).unwrap();
        let change = Change {
            path: path.into(),
            before: before.clone(),
            after: after.clone(),
            word: "Place changed.".into(),
            kind: ChangeKind::Place,
        };
        change.undo(&campaign).unwrap();
        assert_eq!(fs::read_to_string(dir.path().join(path)).unwrap(), before);
        let external = format!("{after}\nWords from Obsidian.\n");
        fs::write(dir.path().join(path), &external).unwrap();
        assert!(
            change
                .undo(&campaign)
                .unwrap_err()
                .contains("changed on disk")
        );
        assert_eq!(fs::read_to_string(dir.path().join(path)).unwrap(), external);
    }

    #[test]
    fn a_change_in_play_writes_the_field_and_a_log_line_and_one_undo_removes_both() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let hob = create_entry(&campaign, "Hob", "npc").unwrap();
        let arena = create_entry(&campaign, "The Arena", "location").unwrap();
        let cart = create_entry(&campaign, "Harvest Cart", "team").unwrap();
        let session = format!(
            "---\nsession_id: {}\nsession_title: ''\nsession_date: 2026-01-01\nsession_number: 1\n---\n# Session 1\n",
            Uuid::new_v4()
        );
        fs::create_dir_all(dir.path().join("Sessions")).unwrap();
        fs::write(dir.path().join("Sessions/Session 1.md"), session).unwrap();
        let path = dir.path().join("NPCs/Hob.md");
        let original = fs::read_to_string(&path).unwrap();
        let mut reader = Reader {
            log_as: Some("[[Session 1]], Day 2".into()),
            ..Default::default()
        };
        let mut change = |key: &str, value: Value| {
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(hob).unwrap()];
            assert!(reader.change(&campaign, item, &index, key, value).is_some());
            reader.undo.clone().unwrap()
        };
        change("status", "dead".into());
        let place = change("in", serde_yaml_ng::to_value(EntryRef::id(arena)).unwrap());
        let group = change("part_of", Value::Sequence(vec!["[[Harvest Cart]]".into()]));
        let text = fs::read_to_string(&path).unwrap();
        assert!(
            text.ends_with(
                "## Log\n\n- [[Session 1]], Day 2: Status alive → dead.\n- [[Session 1]], Day 2: In not set → [[The Arena]].\n- [[Session 1]], Day 2: Part of not set → [[Harvest Cart]].\n"
            ),
            "{text}"
        );
        assert_eq!(place.word, "In: not set → The Arena.");
        assert_eq!(group.kind, ChangeKind::PartOf);

        // The session, the place and the group each show the line. The entry holds it.
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let changes = index.world_changes(&dir.path().join("Sessions/Session 1.md"));
        assert_eq!(changes.len(), 3);
        assert!(
            changes
                .iter()
                .all(|(source, _)| index.items[*source].entry.id == hob)
        );
        for (id, words) in [
            (arena, "In not set → The Arena."),
            (cart, "Part of not set → Harvest Cart."),
        ] {
            let about = index.lines_about(id);
            assert!(
                about.iter().any(|mention| {
                    index.items[mention.source].lines[mention.line.unwrap()]
                        .words()
                        .ends_with(words)
                }),
                "{words}"
            );
        }
        assert_eq!(index.members(cart).len(), 1);

        // Undo takes back the field and its Log line together.
        group.undo(&campaign).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(!text.contains("splat_part_of") && !text.contains("Harvest Cart"));
        assert!(text.ends_with("In not set → [[The Arena]].\n"));

        // Choosing the value that is already there writes nothing.
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        let same_place = serde_yaml_ng::to_value(EntryRef::id(arena)).unwrap();
        assert!(
            reader
                .change(&campaign, item, &index, "in", same_place)
                .is_none()
        );
        assert!(
            reader
                .change(&campaign, item, &index, "status", "dead".into())
                .is_none()
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), text);

        // With no session, a change writes no Log line.
        fs::write(&path, &original).unwrap();
        reader.log_as = None;
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        reader.change(&campaign, item, &index, "status", "dead".into());
        assert!(!fs::read_to_string(&path).unwrap().contains("## Log"));
    }

    #[test]
    fn groups_save_as_a_list_and_an_empty_list_leaves_no_property() {
        let before = "---\nsplat_schema: 2\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: world\nsplat_name: Zarel\nsplat_type: npc\n---\n\n# Zarel\n\nKeep this.\n";
        let groups = Value::Sequence(vec!["[[Arena Guild]]".into(), "[[Verori]]".into()]);
        let joined = fields::patch(before, "part_of", groups).unwrap();
        let entry = Entry::parse("Zarel.md", &joined).unwrap();
        let names: Vec<_> = part_of(&entry)
            .into_iter()
            .map(|group| link_name(group, "Group"))
            .collect();
        assert_eq!(names, ["Arena Guild", "Verori"]);
        assert!(joined.ends_with("# Zarel\n\nKeep this.\n"));
        let left = fields::patch(&joined, "part_of", Value::Sequence(Vec::new())).unwrap();
        assert_eq!(left, before);
    }

    #[test]
    fn dates_count_back_from_day_one_and_say_a_span_once() {
        assert_eq!(ago("About 2 years ago"), Some(2 * 12 * 30 * 24 * 60));
        assert_eq!(ago("a year ago"), ago("1 year ago"));
        assert_eq!(ago("long ago"), None);
        assert_eq!(ago("2 years"), None);
        let event = |dates: &str| {
            Entry::parse(
                "Event.md",
                &format!("---\nsplat_schema: 2\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: world\nsplat_name: Event\nsplat_type: event\n{dates}\n---\n"),
            )
            .unwrap()
        };
        assert_eq!(
            date_words(&event(
                "splat_when: 70 years ago\nsplat_until: 45 years ago"
            ))
            .as_deref(),
            Some("70 to 45 years ago")
        );
        assert_eq!(
            date_words(&event("splat_when: 2 years ago\nsplat_until: 6 months ago")).as_deref(),
            Some("2 years ago to 6 months ago")
        );
        assert_eq!(date_words(&event("splat_status: done")), None);
        // Most entries of a type decide its folder. Of two equal folders, the first by name.
        let entries = [
            ("Places/A.md", "location"),
            ("Places/B.md", "location"),
            ("Old/C.md", "location"),
            ("Zed/D.md", "team"),
            ("Alpha/E.md", "team"),
            ("F.md", "lore"),
        ];
        let folder = |kind| common_folder(entries.iter().copied(), kind);
        assert_eq!(folder("location").as_deref(), Some("Places"));
        assert_eq!(folder("team").as_deref(), Some("Alpha"));
        assert_eq!(folder("lore").as_deref(), Some(""));
        assert_eq!(folder("npc"), None);
        assert_eq!(type_group("lore"), "Lore");
        assert_eq!(type_group("company"), "Companies");
    }

    #[test]
    fn obsidian_paths_encode_query_characters_spaces_and_unicode() {
        assert_eq!(
            obsidian_url(Path::new("/vault/A & B/#é?.md")),
            "obsidian://open?path=/vault/A%20%26%20B/%23%C3%A9%3F.md"
        );
    }

    #[test]
    fn making_an_npc_links_its_creature_and_keeps_that_creatures_file() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let mut entry = note_entry("Creatures/Varra.md", "Varra", "A captain.".into());
        entry.kind = "creature".into();
        let id = make_npc_note(&campaign, &entry).unwrap();
        let text = fs::read_to_string(dir.path().join("NPCs/Varra.md")).unwrap();
        let npc = Entry::parse("NPCs/Varra.md", &text).unwrap();
        assert_eq!(npc.id, id);
        assert_eq!(creature(&npc), Some(Reference::Id(entry.id)));
        assert!(make_npc_note(&campaign, &entry).is_err());
        assert_eq!(
            fs::read_to_string(dir.path().join("NPCs/Varra.md")).unwrap(),
            text
        );
    }
}
