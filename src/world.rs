//! The campaign's world: plain notes Splat reads and does not own, and the NPCs and locations
//! made from them. Obsidian stays the editor. A plain note becomes a Lookup item, not an entry,
//! so nothing depends on it. See `docs/design/world-and-house-rules.md`.
mod fields;
pub mod page;
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
    if has("splat_") || has("timeline_version:") || has("trackers_version:") {
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
        inventory: Vec::new(),
        inventory_containers: Vec::new(),
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
fn each_link(text: &str) -> Vec<(std::ops::Range<usize>, bool, String, Option<String>)> {
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

pub fn place(entry: &Entry) -> Option<Reference> {
    reference(world_field(entry, "in")?)
}

fn link_name(value: &Value, fallback: &str) -> String {
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
fn entry_fields(id: Uuid, name: &str, kind: &str) -> [(&'static str, Value); 5] {
    let mut world = Mapping::new();
    world.insert("type".into(), kind.into());
    if kind == "npc" {
        world.insert("status".into(), "alive".into());
    }
    [
        ("splat_schema", 1.into()),
        ("splat_id", id.to_string().into()),
        ("splat_kind", "world".into()),
        ("splat_name", name.into()),
        ("splat_world", Value::Mapping(world)),
    ]
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
    if world_field(&item.entry, "in").is_some_and(|value| !value.is_null())
        && index.place_of(item.entry.id).is_none()
        && index.place_choices(item.entry.id).is_empty()
    {
        problems.push("Place not found");
    }
    if let Some(reference) = creature(&item.entry)
        && index.position_of(&reference).is_none()
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

fn create_with_details(
    campaign: &Campaign,
    name: &str,
    kind: &str,
    details: &str,
) -> Result<Uuid, String> {
    let name = name.trim();
    let stem = sessions::file_safe(name);
    if stem.is_empty() {
        return Err("Type a name first.".to_owned());
    }
    let folder = match kind {
        "location" => "Locations",
        "npc" => "NPCs",
        _ => "Notes",
    };
    let relative = format!("{folder}/{stem}.md");
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
    fs::create_dir_all(campaign.path.join(folder)).map_err(|e| e.to_string())?;
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
    Ok(if kind.is_empty() {
        note_id(&relative)
    } else {
        id
    })
}

/// Give a plain note an identity. Its body does not change.
pub fn make_entry(campaign: &Campaign, item: &Item, kind: &str) -> Result<Uuid, String> {
    let relative = &item.entry.path;
    let path = note_path(&campaign.path, relative)?;
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let id = crate::hermetic::new_uuid();
    let name = &item.entry.name;
    let updated = if sessions::split_note(&text).is_some() {
        let changes = entry_fields(id, name, kind).map(|(key, value)| (key, Some(value)));
        patch_fields(&text, &changes)?
    } else {
        format!("{}\n{text}", frontmatter(id, name, kind)?)
    };
    write_note(&campaign.path, relative, Some(&text), &updated)?;
    Ok(id)
}

/// Change one field of a world entry's `splat_world`, keeping the rest of the note.
fn set_world_field(
    campaign: &Campaign,
    entry: &Entry,
    name: &str,
    value: Value,
) -> Result<(), String> {
    let path = note_path(&campaign.path, &entry.path)?;
    let text = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    Entry::parse(&entry.path, &text)?;
    let updated = fields::patch(&text, name, value)?;
    write_note(&campaign.path, &entry.path, Some(&text), &updated).map(|_| ())
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
    let id = create_entry(campaign, &entry.name, "npc")?;
    let path = format!("NPCs/{}.md", sessions::file_safe(&entry.name));
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

struct Change {
    path: String,
    before: String,
    after: String,
    word: String,
}

impl Change {
    fn undo(&self, campaign: &Campaign) -> Result<(), String> {
        write_note(&campaign.path, &self.path, Some(&self.after), &self.before).map(|_| ())
    }
}

fn new_identity(campaign: &Campaign, relative: &str) -> Result<(Change, Uuid), String> {
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
            word: "New identity saved.".into(),
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
    pub in_run: std::collections::BTreeMap<Uuid, (usize, Option<i64>)>,
    pub select_run: Option<usize>,
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
            if let Some(previous) = self.showing {
                self.status_drafts.insert(previous, self.status.clone());
            }
            self.showing = Some(item.entry.id);
            self.status = self
                .status_drafts
                .remove(&item.entry.id)
                .unwrap_or_else(|| status(&item.entry).to_owned());
            self.status_editing = self.status != status(&item.entry);
            self.other_type.clear();
            self.adding = None;
            self.picker = None;
            self.error = None;
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
        view::title_with_art(
            ui,
            &title,
            &format!("{} · {}", item.subtitle, item.owner),
            Some(&item.entry),
        );
        let undo_name = format!(
            "Undo {} change for {}",
            if self
                .undo
                .as_ref()
                .is_some_and(|change| change.word.starts_with("In:"))
            {
                "place"
            } else {
                "stat block"
            },
            item.entry.name
        );
        ui.data_mut(|data| data.insert_temp(egui::Id::new("world_undo_name"), undo_name));
        if let Some(error) = &self.error {
            design::message(ui, Tone::Error, error);
        }
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
                        &format!("Use {stem}"),
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
                    && design::message_with_action(ui, Tone::Info, &change.word, "Undo name change")
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
                && Action::quiet("Undo new identity")
                    .icon(design::icons::UNDO)
                    .show(ui)
                    .clicked()
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
                design::message(
                    ui,
                    Tone::Warning,
                    "These files share an identity. Choose which file gets a new one.",
                );
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
                    let label = format!("New identity for {filename}");
                    let clicked = if Action::quiet(&label).natural_width(ui) > ui.available_width()
                    {
                        design::wrapped_link(ui, &label).clicked()
                    } else {
                        Action::quiet(&label).show(ui).clicked()
                    };
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
        let label = if key == "in" { "In" } else { "Stat block" };
        let name = |value: &Value| {
            serde_yaml_ng::from_value::<EntryRef>(value.clone())
                .ok()
                .and_then(|reference| reference.reference().ok())
                .and_then(|reference| index.position_of(&reference))
                .map(|at| index.items[at].entry.name.clone())
                .unwrap_or_else(|| {
                    if value.is_null() {
                        "not set".into()
                    } else {
                        link_name(value, label)
                    }
                })
        };
        let before_name = world_field(&item.entry, key).map_or_else(|| "not set".into(), name);
        let after_name = name(&value);
        let result = (|| {
            let before = fs::read_to_string(note_path(&campaign.path, &item.entry.path)?)
                .map_err(|e| e.to_string())?;
            let after = fields::patch(&before, key, value)?;
            write_note(&campaign.path, &item.entry.path, Some(&before), &after)?;
            Ok::<_, String>(Change {
                path: item.entry.path.clone(),
                before,
                after,
                word: format!("{label}: {before_name} → {after_name}."),
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

    fn relationship_row(
        ui: &mut egui::Ui,
        label: &str,
        width: f32,
        value: impl FnOnce(&mut egui::Ui),
        notice: Option<&str>,
    ) -> (bool, bool) {
        let name = format!("Change {label}");
        let change = Action::quiet("Change")
            .density(design::Density::Scan)
            .name(&name);
        let stacked = ui.available_width() < design::XXL * 12.0;
        let width = if stacked {
            ui.available_width() - change.natural_width(ui) - design::S
        } else {
            width
        };
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
                            ui.set_min_width(width);
                            ui.set_max_width(width);
                            value(ui);
                        },
                    );
                    let clicked = change.show(ui).clicked();
                    if let Some(word) = notice {
                        ui.allocate_ui_with_layout(
                            egui::vec2(ui.available_width(), Density::Scan.control()),
                            egui::Layout::left_to_right(egui::Align::Center),
                            |ui| {
                                undo = Self::change_notice(ui, word);
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

    fn notice_fits(ui: &egui::Ui, value_width: f32, word: &str) -> bool {
        ui.available_width()
            >= design::XXL * 3.0
                + design::S
                + value_width
                + Action::quiet("Change")
                    .density(Density::Scan)
                    .natural_width(ui)
                + design::number_width(ui, word, &Type::Small.font(ui))
                + Type::Small.line()
                + design::XS
                + Action::quiet("Undo").natural_width(ui)
                + ui.spacing().item_spacing.x * 4.0
    }

    fn change_notice(ui: &mut egui::Ui, word: &str) -> bool {
        design::status(
            ui,
            design::Status {
                icon: design::icons::INFO,
                word,
                color: colors().muted,
            },
        );
        Action::quiet("Undo")
            .name(
                &ui.data(|data| data.get_temp::<String>(egui::Id::new("world_undo_name")))
                    .unwrap_or_else(|| "Undo World change".into()),
            )
            .show(ui)
            .clicked()
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
        let value_width = Self::relationship_width(ui);
        for (key, label) in [("creature", "Stat block"), ("in", "In")] {
            if key == "creature" && world_type(&item.entry) != "npc" {
                continue;
            }
            let target = if key == "in" {
                index.place_of(item.entry.id)
            } else {
                creature(&item.entry).and_then(|r| index.position_of(&r))
            };
            let notice = self
                .undo
                .as_ref()
                .filter(|change| {
                    change.path == item.entry.path
                        && change
                            .word
                            .starts_with(if key == "in" { "In:" } else { "Stat block:" })
                        && Self::notice_fits(ui, value_width, &change.word)
                })
                .map(|change| change.word.clone());
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
                            let name = world_field(&item.entry, key)
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
                        } else if let Some(value) = world_field(&item.entry, key) {
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
                    notice.as_deref(),
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
                ui.label(
                    Type::Small
                        .rich(ui, "This link matches more than one note. Choose one.")
                        .color(colors().muted),
                );
                for at in index.place_choices(item.entry.id) {
                    let path = &index.items[at].entry.path;
                    if design::link(ui, path).clicked() {
                        go = Some(Go::Item(at, None));
                    }
                    if Action::quiet("Always use this one")
                        .name(&format!("Always use {path}"))
                        .show(ui)
                        .clicked()
                    {
                        let path = path.strip_suffix(".md").unwrap_or(path);
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
        }
        let Some(picker) = &mut self.picker else {
            return go;
        };
        let key = picker.key;
        let label = if key == "in" {
            "Find a location"
        } else {
            "Find a stat block"
        };
        let width = ui.available_width();
        let response = design::search_field(ui, label, &mut picker.search, label, width);
        if std::mem::take(&mut picker.focus) {
            response.request_focus();
        }
        let matches = if key == "creature" {
            index.creatures_matching(&picker.search, 5)
        } else {
            index
                .items
                .iter()
                .enumerate()
                .filter(|(_, i)| {
                    i.is_private()
                        && i.entry.kind == "world"
                        && world_type(&i.entry) == "location"
                        && i.entry
                            .name
                            .to_lowercase()
                            .contains(&picker.search.to_lowercase())
                })
                .map(|(at, _)| at)
                .take(5)
                .collect()
        };
        let mut chosen = None;
        let mut cancel = false;
        for at in matches {
            let entry = &index.items[at].entry;
            if design::link(ui, &entry.name).clicked() {
                chosen = serde_yaml_ng::to_value(EntryRef::id(entry.id)).ok();
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
                            Ok(id) => chosen = serde_yaml_ng::to_value(EntryRef::id(id)).ok(),
                            Err(error) => self.error = Some(error),
                        }
                    }
                    design::blocked_reason(ui, reason);
                });
            });
        }
        ui.horizontal_wrapped(|ui| {
            if Action::quiet("Clear value").show(ui).clicked() {
                chosen = Some(Value::Null);
            }
            cancel = Action::quiet("Cancel").show(ui).clicked();
        });
        if let Some(value) = chosen {
            return self.change(campaign, item, index, key, value);
        }
        if cancel || ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            self.picker = None;
        }
        go
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
                let targets = [
                    (
                        "Fights as",
                        creature(&item.entry).and_then(|r| index.position_of(&r)),
                    ),
                    ("In", index.place_of(item.entry.id)),
                ];
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
                            "Fights as {} · Not found",
                            world_field(&item.entry, "creature")
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
                    ui.label(
                        Type::Small
                            .rich(ui, "This link matches more than one note. Choose one.")
                            .color(colors().muted),
                    );
                    for at in choices {
                        let path = &index.items[at].entry.path;
                        if design::link(ui, path).clicked() {
                            go = Some(Go::Item(at, None));
                        }
                        if Action::quiet("Always use this one")
                            .name(&format!("Always use {path}"))
                            .show(ui)
                            .clicked()
                        {
                            let path = path.strip_suffix(".md").unwrap_or(path);
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
            }
            if let Some(change) = &self.undo
                && change.path == item.entry.path
                && Self::change_notice(ui, &change.word)
            {
                return self.undo_change(campaign, item.entry.id);
            }
            if index.place_of(item.entry.id).is_none()
                && index.place_choices(item.entry.id).is_empty()
                && let Some(place) = world_field(&item.entry, "in").and_then(Value::as_str)
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
            let value_width = Self::relationship_width(ui);
            if !self.status_editing {
                let value = status(&item.entry);
                if Self::relationship_row(
                    ui,
                    "Status",
                    value_width,
                    |ui| {
                        ui.label(
                            Type::Small
                                .rich(ui, if value.is_empty() { "not set" } else { value })
                                .color(colors().muted),
                        );
                    },
                    None,
                )
                .0
                {
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
                        match set_world_field(campaign, &item.entry, "status", wanted.into()) {
                            Ok(()) => {
                                self.status_editing = false;
                                self.status_drafts.remove(&item.entry.id);
                                return Some(Go::Reload(item.entry.id));
                            }
                            Err(error) => self.error = Some(error),
                        }
                    }
                }
            }
        }
        if !self.compact
            && let Some(change) = &self.undo
            && change.path == item.entry.path
            && !Self::notice_fits(ui, Self::relationship_width(ui), &change.word)
            && ui
                .horizontal(|ui| Self::change_notice(ui, &change.word))
                .inner
        {
            return self.undo_change(campaign, item.entry.id);
        }
        if world_type(&item.entry) != "npc" {
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
            if Action::secondary("Add to run")
                .icon(design::icons::ADD)
                .density(Density::Read)
                .show(ui)
                .clicked()
            {
                self.adding = Some(Adding {
                    stat_block: creature(&item.entry).and_then(|r| index.position_of(&r)),
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
                let saved = adding.save && creature(&item.entry).is_none();
                if saved {
                    let stored = serde_yaml_ng::to_value(EntryRef::id(stat_block.id))
                        .map_err(|e| e.to_string())
                        .and_then(|stored| {
                            set_world_field(campaign, &item.entry, "creature", stored)
                        });
                    if let Err(error) = stored {
                        self.error = Some(error);
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

/// What is here, for a place or a group, and what mentions this.
fn related(ui: &mut egui::Ui, item: &Item, index: &ContentIndex) -> Option<Go> {
    let mut go = None;
    let here = index.inside(&item.entry);
    if !here.is_empty() {
        ui.add_space(design::S);
        style::section_heading(ui, "Here");
        for position in here {
            let entry = &index.items[position].entry;
            ui.horizontal(|ui| {
                if design::link(ui, &entry.name).clicked() {
                    go = Some(Go::Item(position, None));
                }
                ui.label(
                    Type::Small
                        .rich(ui, type_label(world_type(entry)))
                        .color(colors().muted),
                );
            });
        }
    }
    let mentions = index.mentions_of(item.entry.id);
    if !mentions.is_empty() {
        ui.add_space(design::S);
        style::section_heading(ui, "Mentioned in");
        for &position in mentions {
            if design::link(ui, &index.items[position].entry.name).clicked() {
                go = Some(Go::Item(position, None));
            }
        }
    }
    go
}

/// Choose the stat block, side and initiative. Returns how the form ended, if it did.
fn add_form(
    ui: &mut egui::Ui,
    item: &Item,
    index: &ContentIndex,
    sides: &[String],
    adding: &mut Adding,
) -> Option<FormEnd> {
    ui.label(Type::Strong.rich(ui, format!("Add {} to the run", item.entry.name)));
    match adding.stat_block.map(|at| &index.items[at].entry) {
        Some(stat_block) => {
            ui.horizontal_wrapped(|ui| {
                ui.label(Type::Small.rich(ui, "Stat block").color(colors().muted));
                ui.label(Type::Body.rich(ui, &stat_block.name).color(colors().text));
                if design::link(ui, "Change").clicked() {
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
        ui.label(Type::Label.rich(ui, "SIDE").color(colors().muted));
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
    }
    design::text_field(
        ui,
        "Initiative",
        &mut adding.initiative,
        Some("Leave empty to roll it."),
        None,
    );
    let mut end = None;
    ui.horizontal(|ui| {
        if Action::secondary("Add")
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
        let before = patch_fields(
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
