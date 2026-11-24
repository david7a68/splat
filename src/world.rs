//! The campaign's world: plain notes Splat reads and does not own, and the NPCs and locations
//! made from them. Obsidian stays a plain note's editor; an entry also has Edit here. A plain
//! note becomes a Lookup item, not an entry, so nothing depends on it.
//! See `docs/design/world-and-house-rules.md`.
#[path = "world/checks.rs"]
pub mod checks;
#[path = "world/edit.rs"]
pub mod edit;
#[path = "world/fields.rs"]
mod fields;
#[path = "world/file_fault.rs"]
pub mod file_fault;
#[path = "world/lines.rs"]
pub mod lines;
#[path = "world/model.rs"]
pub mod model;
#[path = "world/related.rs"]
pub mod related;
#[path = "world/repair.rs"]
pub mod repair;

use crate::{
    content_workspace::{EntryRef, patch_fields},
    design::{
        self,
        outcome::{
            Choice, Fault, FaultMessage, KeptOutcome, MessageChoice, Messages, Notice, Outcome,
            PaneFrame, Said, Sentence, Severity, UndoStep, Verb,
        },
        reader as style,
    },
    library::{note_path, write_note},
    lookup::{ContentIndex, Item},
    markdown_view::{FILE_LINK, NOTE_LINK},
    persistence::Campaign,
    roster::{GroupMember, RunGroup},
    session_notes as sessions,
};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use serde_yaml_ng::{Mapping, Value};
use splat::character::stable_id;
use splat::content::{
    Entry, Reference,
    art::FORMATS,
    layout::{Field, FlagKind},
    overrides::Overrides,
};
use splat_campaign::timeline;
use std::{
    collections::HashMap,
    fs,
    hash::{Hash, Hasher},
    path::{Component, Path, PathBuf},
    sync::Arc,
};
use uuid::Uuid;

/// The kind of a plain note's Lookup item. No file holds this kind.
pub const NOTE_KIND: &str = "world_note";

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
        // The tracker ledger is Splat's own file, not a world note. A note of the DM's that has
        // that name and is no ledger stays a world note.
        if path.parent() == Some(root)
            && splat_campaign::trackers::ledger::is_ledger(&file_name, front)
        {
            continue;
        }
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
pub fn each_link(text: &str) -> Vec<(std::ops::Range<usize>, bool, String, Option<String>)> {
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
pub fn redirect_links(text: &str, old: &Entry, new_id: Uuid) -> String {
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
pub fn rename_note_links(text: &str, old: &str, new: &str) -> (String, usize) {
    let mut count = 0;
    let (front, body) = crate::session_notes::split_note(text).unwrap_or(("", text));
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
    with_named_links(body, images, &HashMap::new())
}

/// The words a link with no words of its own shows: a session note's name as the Sessions page
/// gives it (`Session 1: The Crossing`, from `names`, `ContentIndex::session_names`), else its
/// heading or its target. A link to a heading of a session shows that heading.
pub fn link_words(target: &str, names: &HashMap<String, String>) -> String {
    // By the whole link, folder and all: `[[Notes/Session 1]]` is not the session's
    // `Sessions/Session 1`, and a bare stem is in `names` only when one note has it.
    if !target.contains('#') {
        let link = target.trim_start_matches('/').to_lowercase();
        if let Some(name) = names.get(link.trim_end_matches(".md")) {
            return name.clone();
        }
    }
    target
        .rsplit('#')
        .next()
        .unwrap_or(target)
        .trim_start_matches('^')
        .to_owned()
}

/// `text` with each `[[link]]` as its words (`link_words`), for a line that shows a link as
/// plain text, such as the stamp beside `Add to Log`'s field.
pub(crate) fn plain_named_links(text: &str, names: &HashMap<String, String>) -> String {
    let mut out = String::with_capacity(text.len());
    let mut copied = 0;
    for (range, _, target, words) in each_link(text) {
        out.push_str(&text[copied..range.start]);
        copied = range.end;
        out.push_str(&words.unwrap_or_else(|| link_words(&target, names)));
    }
    out.push_str(&text[copied..]);
    out
}

/// `with_links`, where a link to a session note shows the session's name (`link_words`).
pub fn with_named_links(
    body: &str,
    images: &HashMap<String, PathBuf>,
    names: &HashMap<String, String>,
) -> String {
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
            let shown = words.unwrap_or_else(|| link_words(&target, names));
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
    world_field(entry, key).filter(|value| !empty_optional(value))
}

/// Which group a Part of value names. Two spellings of one link (an alias, a folder path, a
/// different case) are one group.
#[derive(PartialEq)]
enum GroupKey {
    /// The entry the value resolves to.
    Found(usize),
    /// A link that resolves to no single entry: its target as Obsidian reads it.
    Link(String),
    /// Any other value, compared as written.
    Text(Value),
}

fn group_key(index: &ContentIndex, value: &Value) -> GroupKey {
    if let Some(at) = index.world_value_target(value) {
        return GroupKey::Found(at);
    }
    value
        .as_str()
        .or_else(|| value.get("link").and_then(Value::as_str))
        .and_then(|raw| raw.trim().strip_prefix("[[")?.strip_suffix("]]"))
        .and_then(|link| link.split(['|', '#']).next())
        .map_or_else(
            || GroupKey::Text(value.clone()),
            |target| GroupKey::Link(target.trim().trim_end_matches(".md").to_lowercase()),
        )
}

/// Each group's key, found once per list: a link to no note can scan the whole catalogue.
fn group_keys(index: &ContentIndex, groups: &[Value]) -> Vec<GroupKey> {
    groups.iter().map(|group| group_key(index, group)).collect()
}

/// True when `groups` has a value that names the same group as `group`.
#[cfg(test)]
fn has_group(index: &ContentIndex, groups: &[Value], group: &Value) -> bool {
    group_keys(index, groups).contains(&group_key(index, group))
}

/// The disk list with only the groups that changed from `before` to `after` added or removed.
/// Groups added outside Splat stay. Groups compare by target, so a link Obsidian respelled is
/// still the same group, and an unchanged group keeps its disk spelling. The Read picker and
/// Edit's Overwrite both use this.
fn merge_groups(
    index: &ContentIndex,
    before: &[Value],
    after: &[Value],
    disk: &[Value],
) -> Vec<Value> {
    let before = group_keys(index, before);
    let after_keys = group_keys(index, after);
    let (mut groups, mut keys): (Vec<_>, Vec<_>) = disk
        .iter()
        .zip(group_keys(index, disk))
        .filter(|(_, key)| !before.contains(key) || after_keys.contains(key))
        .map(|(group, key)| (group.clone(), key))
        .unzip();
    for (group, key) in after.iter().zip(after_keys) {
        if !before.contains(&key) && !keys.contains(&key) {
            groups.push(group.clone());
            keys.push(key);
        }
    }
    groups
}

/// The name part of a missing link: the warning icon and the name, in `Body` as a set link is.
fn missing_name(name: &str) -> String {
    format!("{} {name}", design::icons::WARN)
}

/// The state part of a missing link, in `Small` after the name.
const NOT_FOUND: &str = " · Not found";

/// The words of a place link that matches more than one note.
fn place_choices_words(name: &str, count: usize) -> String {
    format!("{name} · {count} matches")
}

fn empty_optional(value: &Value) -> bool {
    value.is_null() || value.as_str().is_some_and(|text| text.trim().is_empty())
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

/// A World note's aliases, or another entry's top-level `aliases` (round 28 row 144: a renamed
/// built-in keeps its old name). One reader for both: `formats::aliases`.
pub fn aliases(entry: &Entry) -> Vec<&str> {
    splat::content::formats::aliases(entry)
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

/// A fight that nobody planned: what a reader's or a scene's `Start fight` asks the app for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StartFight {
    /// The NPC's or the creature's name, for the encounter `Fight with <name>`.
    pub name: String,
    /// The stat block that joins: the creature itself, or the NPC's. An NPC with none joins
    /// through the run's form for a creature made up at the table.
    pub creature: Option<Uuid>,
    /// The name is an NPC's, which its stat block's row takes.
    pub npc: bool,
}

/// Whether a Lookup item is a World entry: a world note, or a note of kind `world`.
pub fn is_world(item: &Item) -> bool {
    item.entry.kind == "world" || item.is_world_note()
}

/// Whether `Add to Log` can add a line to this entry's Log: a World entry of the campaign,
/// as Edit can change it. A package's entry has no action: its Log would belong to every
/// campaign that uses the package.
pub fn takes_log(item: &Item, index: &ContentIndex) -> bool {
    edit::editable(item, index)
}

/// A creature or an NPC that a fight can start with. A party member's copy is neither.
pub fn start_fight(item: &Item, index: &ContentIndex) -> Option<StartFight> {
    let entry = &item.entry;
    if is_world(item) {
        if !fights(entry) {
            return None;
        }
        return Some(StartFight {
            name: entry.name.clone(),
            creature: index
                .world_target(entry, "creature")
                .filter(|at| index.items[*at].entry.kind == "creature")
                .map(|at| index.items[at].entry.id),
            npc: true,
        });
    }
    (entry.kind == "creature"
        && crate::lookup::entry_origin(Some(entry), item.is_private())
            != crate::lookup::Origin::Sheet)
        .then(|| StartFight {
            name: entry.name.clone(),
            creature: Some(entry.id),
            npc: false,
        })
}

pub fn link_name(value: &Value, fallback: &str) -> String {
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
        "" => "World note".to_owned(),
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

/// A link on a world note that the DM must fix: its note is missing, or it waits for a choice.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LinkProblem {
    PlaceNotFound,
    /// The place link matches this many notes, and waits for the DM to choose one.
    PlaceMatches(usize),
    StatBlockNotFound,
    GroupNotFound,
    /// The place's `In` chain leads back to this note.
    PlaceLoop,
}

impl std::fmt::Display for LinkProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlaceNotFound => f.write_str("Place not found"),
            Self::PlaceMatches(count) => write!(f, "Place has {count} matches"),
            Self::StatBlockNotFound => f.write_str("Stat block not found"),
            Self::GroupNotFound => f.write_str("Group not found"),
            Self::PlaceLoop => f.write_str("Place loop"),
        }
    }
}

/// A list row's words for the links that need fixing. One problem names itself. Several are
/// `broken links` only when every target is missing; a place that waits for a choice, or one in a
/// loop, is not broken.
pub fn link_problem_words(problems: &[LinkProblem]) -> String {
    match problems {
        [one] => one.to_string(),
        many if many.iter().any(|problem| {
            matches!(
                problem,
                LinkProblem::PlaceMatches(_) | LinkProblem::PlaceLoop
            )
        }) =>
        {
            design::count(many.len(), "link needs fixing", "links need fixing")
        }
        many => design::count(many.len(), "broken link", "broken links"),
    }
}

/// The file's own name, when a World note's name is not its file's name (a rename in Obsidian
/// or in the note's heading). The reader shows the warning with its two repairs; the list row
/// and the header count show it too (FI2). A duplicate has its own repair first.
pub(crate) fn renamed_file(item: &Item, index: &ContentIndex) -> Option<String> {
    if item.entry.kind != "world" || index.duplicate_world.contains_key(&item.entry.path) {
        return None;
    }
    let stem = Path::new(&item.entry.path)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    (stem != item.entry.name).then(|| stem.into_owned())
}

/// A World note's fault, as its list row says it and the header counts it: the first that
/// holds, in this order. Every kind is a warning (one fault, one tone); a note that cannot be
/// read at all is the header's error count, not one of these.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NoteFault {
    /// Two files claim the note's identity.
    Duplicate,
    /// Values that could not be read, with their count.
    Unread(usize),
    /// Roll tables with faults, with their count.
    Tables(usize),
    /// Links that need fixing.
    Links(Vec<LinkProblem>),
    /// The note's name is not its file's name (FI2). The reader's warning names both. The
    /// file's name only while another note has the same name: then it tells the rows apart,
    /// as a same-name row's file name does.
    Renamed(Option<String>),
}

impl NoteFault {
    fn of(at: usize, index: &ContentIndex) -> Option<Self> {
        let item = &index.items[at];
        let entry = &item.entry;
        let problems = broken_links(item, index);
        let tables = crate::roll_table::fault_count(&entry.body);
        if index.duplicate_world.contains_key(&entry.path) {
            Some(Self::Duplicate)
        } else if !entry.flags.is_empty() {
            Some(Self::Unread(entry.flags.len()))
        } else if tables > 0 {
            Some(Self::Tables(tables))
        } else if !problems.is_empty() {
            Some(Self::Links(problems))
        } else {
            renamed_file(item, index)
                .map(|stem| Self::Renamed(index.shares_name(at).then(|| format!("{stem}.md"))))
        }
    }

    /// The list row's words after its warning icon.
    pub fn words(&self) -> String {
        match self {
            Self::Duplicate => "Same note in two files".into(),
            Self::Unread(count) => design::count(*count, "value not read", "values not read"),
            Self::Tables(count) => design::count(*count, "table fault", "table faults"),
            Self::Links(problems) => link_problem_words(problems),
            // The row names the fault, not a value the DM must compare with the note's name
            // (SF6); the reader's warning names both names and offers the two renames. The
            // file's name follows only when it tells two same-name rows apart, so the words
            // fit a narrow list whole otherwise.
            Self::Renamed(None) => "File name differs".into(),
            Self::Renamed(Some(file)) => format!("File name differs · {file}"),
        }
    }

    /// The row's words where `words` does not fit whole. A same-name note keeps its file's
    /// name, which tells its row from the other same-name rows (each may have this fault too);
    /// its warning mark and the header count still say there is a fault, and its reader names
    /// both names.
    pub fn narrow_words(&self) -> Option<String> {
        match self {
            Self::Renamed(Some(file)) => Some(file.clone()),
            _ => None,
        }
    }
}

/// The faults of one index's World notes, worked out once per index
/// (`ContentIndex::note_faults`): the list rows, the header's count and the count's menu read
/// these records, so they never disagree and nothing that grows with the notes runs each frame.
#[derive(Debug, Default)]
pub struct NoteFaults {
    by_position: HashMap<usize, NoteFault>,
    /// The notes with a fault, in the World list's order: the count's menu.
    listed: Vec<usize>,
    /// Every World note in the list, for the header's note count.
    notes: usize,
}

impl NoteFaults {
    pub fn of(index: &ContentIndex) -> Self {
        let mut faults = Self::default();
        for &(at, _) in index.world_groups("").values().flatten() {
            faults.notes += 1;
            if let Some(fault) = NoteFault::of(at, index) {
                faults.by_position.insert(at, fault);
                faults.listed.push(at);
            }
        }
        faults
    }

    /// The fault of the note at `at` in the index.
    pub fn of_note(&self, at: usize) -> Option<&NoteFault> {
        self.by_position.get(&at)
    }

    /// The notes with a fault, in list order.
    pub fn listed(&self) -> &[usize] {
        &self.listed
    }

    /// How many World notes the list holds.
    pub fn notes(&self) -> usize {
        self.notes
    }
}

/// The save row's one reason while a Log field of the note waits for a changed form: the
/// campaign switch and the Log line have one cause (ruling Q-t).
pub const LOG_WAITS_FOR_SAVE: &str = "Save or discard to add a Log line or switch campaign.";
/// The same, for a form with no changes: Cancel closes it and opens the field.
pub(crate) const LOG_WAITS_FOR_CANCEL: &str = "Cancel to add a Log line.";

/// The links on this note that need fixing, in the order the reader shows them.
pub fn broken_links(item: &Item, index: &ContentIndex) -> Vec<LinkProblem> {
    let mut problems = Vec::new();
    if reader_field(&item.entry, "in").is_some() && index.place_of(item.entry.id).is_none() {
        problems.push(match index.place_choices(item.entry.id).len() {
            0 => LinkProblem::PlaceNotFound,
            count => LinkProblem::PlaceMatches(count),
        });
    }
    if index.in_place_loop(item.entry.id) {
        problems.push(LinkProblem::PlaceLoop);
    }
    if reader_field(&item.entry, "creature").is_some()
        && index.world_target(&item.entry, "creature").is_none()
    {
        problems.push(LinkProblem::StatBlockNotFound);
    }
    // The index found each group when it loaded. Its list follows the file's order; a value it
    // did not see (two files with one id) is looked up again.
    let found = index.groups_of(&item.entry);
    for (at, value) in part_of(&item.entry).into_iter().enumerate() {
        if empty_optional(value) {
            continue;
        }
        let target = match found.get(at) {
            Some((name, target)) if *name == link_name(value, "Group") => *target,
            _ => index.world_value_target(value),
        };
        if target.is_none() {
            problems.push(LinkProblem::GroupNotFound);
        }
    }
    problems
}

/// A new NPC from the generator, with its attributes as the note's text.
pub fn create_generated(
    campaign: &Campaign,
    npc: &crate::npc_generator::Generated,
) -> Result<Uuid, String> {
    create_with_details(campaign, &npc.name, "npc", &npc.details())
}

/// The folder that holds most entries of this type, from each entry's path and type. Folders
/// belong to the DM, so a new entry goes where its kind already is.
pub fn common_folder<'a>(
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

/// The relative path of a new entry with this name in this folder. An empty folder is the
/// campaign's root.
fn path_in(folder: &str, name: &str) -> String {
    let stem = sessions::file_safe(name.trim());
    if folder.is_empty() {
        format!("{stem}.md")
    } else {
        format!("{folder}/{stem}.md")
    }
}

/// Where a new entry of this kind and name will be written, from the campaign's notes as the
/// index read them. It uses the folder rule of the write, so a card can promise the real path.
pub fn new_entry_path(index: Option<&ContentIndex>, name: &str, kind: &str) -> String {
    let folder = index
        .and_then(|index| index.type_folder(kind))
        .unwrap_or_else(|| default_folder(kind));
    path_in(&folder, name)
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
pub fn new_entry_link(index: Option<&ContentIndex>, name: &str, kind: &str) -> String {
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
    let relative = path_in(&folder, name);
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
    let failed = file_fault::write_error(&relative);
    fs::create_dir_all(campaign.path.join(&folder)).map_err(&failed)?;
    if kind.is_empty() {
        use std::io::Write;
        let mut temp =
            tempfile::NamedTempFile::new_in(path.parent().ok_or("Missing parent folder")?)
                .map_err(&failed)?;
        temp.write_all(text.as_bytes())
            .and_then(|_| temp.as_file().sync_all())
            .map_err(&failed)?;
        temp.persist_noclobber(&path)
            .map_err(|error| failed(error.error))?;
    } else {
        write_note(&campaign.path, &relative, None, &text)?;
    }
    drop(failed);
    let id = if kind.is_empty() {
        note_id(&relative)
    } else {
        id
    };
    Ok((id, relative))
}

/// What giving a note a type gives the DM, in the DM's words. It does not repeat the section
/// heading.
pub const TYPE_HELP: &str = "A type adds the fields for that kind of note, such as an NPC’s place and stat block. Its text stays the same.";
/// How Splat stores the type, for Technical details only.
pub const TYPE_STORAGE: &str = "Splat adds fields that start with splat_ (the type, name and an ID) to the front matter \
     at the top of the file.";

/// Give a plain note an identity. Its body does not change.
pub fn make_entry(campaign: &Campaign, item: &Item, kind: &str) -> Result<Uuid, String> {
    let relative = &item.entry.path;
    let text = file_fault::read_note(&campaign.path, relative)?;
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
    let text = file_fault::read_note(&campaign.path, &entry.path)?;
    Entry::parse(&entry.path, &text)?;
    let updated = fields::patch(&text, name, value)?;
    write_note(&campaign.path, &entry.path, Some(&text), &updated)?;
    Ok(Change {
        campaign: campaign.id,
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
        "in" => "Place",
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

/// A value change's result as a sentence (round 26 FE3): `Stat block set to Guard.`, `Place set
/// to North Gate. It was Watch House.`, `Place cleared. It was Ashfall Keep.` The arrow form stays in
/// the note's Log lines only (`log_words`). `None` is a value that is not set.
fn change_words(label: &str, before: Option<&str>, after: Option<&str>) -> String {
    match (before, after) {
        (None, Some(after)) => format!("{label} set to {after}."),
        (Some(before), Some(after)) => format!("{label} set to {after}. It was {before}."),
        (Some(before), None) => format!("{label} cleared. It was {before}."),
        (None, None) => format!("{label} cleared."),
    }
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

pub fn make_npc_note(campaign: &Campaign, entry: &Entry) -> Result<Uuid, String> {
    let (id, path) = create_at(campaign, &entry.name, "npc", "")?;
    let text = file_fault::read_note(&campaign.path, &path)?;
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
pub struct Change {
    /// The campaign whose folder holds the note. The result shows, and Undo acts, only there:
    /// Lookup keeps its reader across a campaign change.
    campaign: Uuid,
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
    Identity(IdentityNoun),
    /// Everything the edit form saved at once.
    Edit,
    /// One line that `Add to Log` added, in the DM's words.
    Log,
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

    /// Its Undo (ruling R16): when one field changed, its object is that field's name on
    /// screen, the name the result's words use (`Part of set to …` has `Undo change to Part
    /// of`); when the edit form saved several fields, the note's name (`Undo change to Ashfall
    /// Faction`). A separate note names the copy's file, as the encounter editor's does
    /// (`Undo separate Rook copy.md`, FG5). `note` is the note's name, `file` its file's.
    fn undo_step(self, note: &str, file: &str) -> UndoStep {
        match self {
            Self::Place => UndoStep::of(Verb::Change, "Place"),
            Self::PartOf => UndoStep::of(Verb::Change, "Part of"),
            Self::Status => UndoStep::of(Verb::Change, "Status"),
            Self::StatBlock => UndoStep::of(Verb::Change, "Stat block"),
            Self::Name => UndoStep::of(Verb::Change, "Name"),
            Self::Identity(_) => UndoStep::of(Verb::Separate, file),
            Self::Edit => UndoStep::of(Verb::Change, note),
            Self::Log => UndoStep::of(Verb::Add, "Log line"),
        }
    }
}

impl Change {
    /// Its Undo step: the kind's, with the name of the note it changed (`ChangeKind::undo_step`).
    pub fn undo_step(&self, note: &str) -> UndoStep {
        self.kind
            .undo_step(note, self.path.rsplit('/').next().unwrap_or(&self.path))
    }

    pub fn message(&self) -> &str {
        &self.word
    }

    /// Whether this change's result belongs on this campaign's note at `path`.
    pub(crate) fn shows_on(&self, campaign: &Campaign, path: &str) -> bool {
        self.campaign == campaign.id && self.path == path
    }

    pub fn undo(&self, campaign: &Campaign) -> Result<(), String> {
        if campaign.id != self.campaign {
            return Err("This change belongs to another campaign. Nothing was changed.".into());
        }
        write_note(&campaign.path, &self.path, Some(&self.after), &self.before).map(|_| ())
    }
}

/// A note's file renamed to match its name (FD5). Undo moves it back. Neither move replaces a
/// file, and each checks that the file still holds the text the DM saw.
pub(crate) struct FileRename {
    /// The campaign whose folder holds the file. Undo acts only there.
    campaign: Uuid,
    from: String,
    to: String,
    text: String,
}

impl FileRename {
    /// `seen` is the file's text that produced the reader: the move refuses unless the file
    /// still holds it.
    fn apply(campaign: &Campaign, from: &str, to: &str, seen: &str) -> Result<Self, String> {
        move_note(&campaign.path, from, to, seen, || {})?;
        Ok(Self {
            campaign: campaign.id,
            from: from.into(),
            to: to.into(),
            text: seen.into(),
        })
    }

    /// Whether this rename's result belongs on this campaign's note at `path`.
    fn shows_on(&self, campaign: &Campaign, path: &str) -> bool {
        self.campaign == campaign.id && self.to == path
    }

    fn undo(&self, campaign: &Campaign) -> Result<(), String> {
        if campaign.id != self.campaign {
            return Err("This rename belongs to another campaign. Nothing was renamed.".into());
        }
        move_note(&campaign.path, &self.to, &self.from, &self.text, || {})
    }

    fn message(&self) -> String {
        let file = self.to.rsplit('/').next().unwrap_or(&self.to);
        format!("The file is now {file}.")
    }
}

/// The file beside `path` for a note named `name`, when the name can be a file name.
fn renamed_path(path: &str, name: &str) -> Option<String> {
    let name = name.trim();
    if name.is_empty()
        || name.starts_with('.')
        || name.contains([
            '/', '\\', ':', '*', '?', '"', '<', '>', '|', '#', '^', '[', ']',
        ])
    {
        return None;
    }
    let to = match path.rsplit_once('/') {
        Some((folder, _)) => format!("{folder}/{name}.md"),
        None => format!("{name}.md"),
    };
    (to != path && crate::library::note_path_rule(&to).is_none()).then_some(to)
}

/// How a note file moves. `rename` never replaces a file; `link` makes a second name and fails
/// when the target exists. Tests give other ones to stand in for a file system.
struct Moves {
    rename: fn(&Path, &Path) -> std::io::Result<()>,
    link: fn(&Path, &Path) -> std::io::Result<()>,
    remove: fn(&Path) -> std::io::Result<()>,
}

const SYSTEM_MOVES: Moves = Moves {
    #[cfg(target_os = "linux")]
    rename: |source, target| crate::session_notes::rename_no_replace(source, target),
    #[cfg(not(target_os = "linux"))]
    rename: |source, target| std::fs::rename(source, target),
    link: |source, target| std::fs::hard_link(source, target),
    remove: |path| std::fs::remove_file(path),
};

/// The words when this folder's file system has no move that is safe for an existing file.
const NO_SAFE_MOVE: &str = "Splat cannot rename files safely in this folder, so nothing was moved.";

/// The words when a move left one file with two names and Splat could remove neither.
fn two_names(from: &str, to: &str) -> String {
    format!(
        "{from} and {to} are now two names for one note, and Splat could remove neither. Delete {to} in your file manager to keep the note as it was."
    )
}

/// The file system does not offer this kind of move (not a fault of these files).
fn move_unsupported(error: &std::io::Error) -> bool {
    error.kind() == std::io::ErrorKind::Unsupported
        || matches!(
            error.raw_os_error(),
            Some(libc::EINVAL | libc::ENOSYS | libc::EOPNOTSUPP)
        )
}

/// Moves a note file within the campaign. It refuses when `from` no longer holds `expected` or
/// a file is at `to`, so it never replaces or loses a file. The move itself never replaces a
/// file either (`RENAME_NOREPLACE`), so a file that arrives at `to` after the check keeps its
/// bytes. Where the file system does not offer that move, a hard link (which also refuses an
/// existing target) and then the removal of the old name move it; where it offers neither, it
/// refuses and nothing moves. `before_move` runs between the checks and the move, for tests.
fn move_note(
    root: &Path,
    from: &str,
    to: &str,
    expected: &str,
    before_move: impl FnOnce(),
) -> Result<(), String> {
    move_note_with(&SYSTEM_MOVES, root, from, to, expected, before_move)
}

fn move_note_with(
    moves: &Moves,
    root: &Path,
    from: &str,
    to: &str,
    expected: &str,
    before_move: impl FnOnce(),
) -> Result<(), String> {
    let source = crate::library::note_path(root, from)?;
    let target = crate::library::note_path(root, to)?;
    if file_fault::read_note(root, from)? != expected {
        return Err(format!("{from} changed on disk. Nothing was renamed."));
    }
    let exists = || format!("{to} already exists. Nothing was renamed.");
    if std::fs::symlink_metadata(&target).is_ok() {
        return Err(exists());
    }
    before_move();
    let failed = |error: std::io::Error| {
        if error.kind() == std::io::ErrorKind::AlreadyExists {
            exists()
        } else {
            file_fault::FileFault::write(from, &error).message()
        }
    };
    match (moves.rename)(&source, &target) {
        Ok(()) => Ok(()),
        Err(error) if move_unsupported(&error) => match (moves.link)(&source, &target) {
            // Both names now hold one file. The old name goes; if it cannot, the new one goes,
            // so nothing moved; if neither can, the DM is told both names and what to do.
            Ok(()) => match (moves.remove)(&source) {
                Ok(()) => Ok(()),
                Err(error) => match (moves.remove)(&target) {
                    Ok(()) => Err(format!(
                        "{} Nothing was renamed.",
                        file_fault::FileFault::write(from, &error).headline()
                    )),
                    Err(_) => Err(two_names(from, to)),
                },
            },
            Err(error) if move_unsupported(&error) || error.raw_os_error() == Some(libc::EPERM) => {
                Err(NO_SAFE_MOVE.into())
            }
            Err(error) => Err(failed(error)),
        },
        Err(error) => Err(failed(error)),
    }
}

/// World notes and Library entries share identity repair, but keep their own nouns.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IdentityNoun {
    Note,
    Entry,
}

impl IdentityNoun {
    pub fn of(entry: &Entry) -> Self {
        if entry.kind == "world" {
            Self::Note
        } else {
            Self::Entry
        }
    }

    pub fn word(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Entry => "entry",
        }
    }

    fn result(self, filename: &str) -> String {
        format!("{filename} is now a separate {}.", self.word())
    }
}

pub const DUPLICATE_ENTRY: &str = "These two files claim to be the same entry.";

/// The duplicate-identity repair's one form (M0178), on Library, World and Encounters: the
/// fault is two files that claim one identity, in the warning tone; each file has the repair
/// `Make <file> a separate <noun>` ([`separate_choice`]), with what it leaves as its help line
/// ([`duplicate_consequence`]). Library says it in its footer (a `Notice`), World's reader and
/// Encounters' search in their body (`BodyFault`), with the same words.
pub struct DuplicateIdentity(pub IdentityNoun);

impl design::outcome::Fault for DuplicateIdentity {
    fn severity(&self) -> design::outcome::Severity {
        design::outcome::Severity::NeedsFixing
    }
    fn cause(&self) -> design::outcome::Sentence {
        match self.0 {
            IdentityNoun::Note => crate::sentence!("These two files claim to be the same note."),
            IdentityNoun::Entry => crate::sentence!("These two files claim to be the same entry."),
        }
    }
}

/// The repair of one of the files: `Make <file> a separate note` (or `entry`). The file shows
/// whole: in a narrow reader the words wrap in the action (M0583), never cut.
pub fn separate_choice(noun: IdentityNoun, file: &str) -> design::outcome::ChoiceWords {
    match noun {
        IdentityNoun::Note => crate::choice!("Make {} a separate note", file),
        IdentityNoun::Entry => crate::choice!("Make {} a separate entry", file),
    }
    .whole_objects()
}

/// The help line under a World note's repair: what its existing links keep. A duplicate of
/// another kind can leave a World note with only its own path, so the first repair may have no
/// other file to name.
fn links_consequence(paths: &[String], at: usize) -> Option<design::outcome::Sentence> {
    let other = if at == 0 { paths.get(1) } else { paths.first() }?;
    Some(duplicate_consequence(KeptBy::Links, other, at == 0))
}

/// What still points at the identity, which a repair's help line names.
pub enum KeptBy<'a> {
    /// The encounters that use the entry, by name.
    Encounters(&'a [String]),
    /// The links to the note.
    Links,
}

/// What a file's repair leaves the identity's users with (ruling M8, its help line): the first
/// file's repair keeps them on the other file (`keeps`), the others move them to `remaining`.
pub fn duplicate_consequence(
    kept: KeptBy<'_>,
    remaining: &str,
    keeps: bool,
) -> design::outcome::Sentence {
    match kept {
        KeptBy::Encounters(users) => {
            let names = crate::library::name_list(users);
            match (keeps, users.len()) {
                (true, 1) => crate::sentence!("{} keeps {}.", names, remaining),
                (true, _) => crate::sentence!("{} keep {}.", names, remaining),
                (false, _) => crate::sentence!("{} will then use {}.", names, remaining),
            }
        }
        KeptBy::Links if keeps => crate::sentence!("Existing links keep {}.", remaining),
        KeptBy::Links => crate::sentence!("Existing links will then use {}.", remaining),
    }
}

pub fn new_identity(campaign: &Campaign, relative: &str) -> Result<(Change, Uuid), String> {
    let before = file_fault::read_note(&campaign.path, relative)?;
    let noun = IdentityNoun::of(&Entry::parse(relative, &before)?);
    let id = crate::hermetic::new_uuid();
    let after = crate::content_workspace::new_note_identity(&before, relative, id)?;
    write_note(&campaign.path, relative, Some(&before), &after)?;
    Ok((
        Change {
            campaign: campaign.id,
            path: relative.into(),
            before,
            after,
            word: noun.result(
                Path::new(relative)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .as_ref(),
            ),
            kind: ChangeKind::Identity(noun),
        },
        id,
    ))
}

pub fn repair_in_world(path: &Path) -> bool {
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

impl Adding {
    /// The form for an NPC with no stat block to join the run: the enemies' side when the run
    /// has one or none, else the run's first side.
    fn new(item: &Item, index: &ContentIndex, sides: Option<&[String]>) -> Self {
        let sides = sides.unwrap_or_default();
        Self {
            stat_block: index.world_target(&item.entry, "creature"),
            save: true,
            side: if sides.is_empty() || sides.iter().any(|side| side == "enemies") {
                "enemies".to_owned()
            } else {
                sides[0].clone()
            },
            ..Default::default()
        }
    }
}

/// One index generation's source-line lookup, and the entry without the flags drawn below the
/// file line.
struct GroupFault {
    id: Uuid,
    generation: u64,
    /// The Part of list on disk that could not be read.
    unread: Option<fields::Unread>,
    /// Every other value not read, as the reader words it, with a long value cut, then whole.
    /// They sit with the Part of reason.
    notices: Vec<(String, String)>,
    /// The entry for the shared reader, without its flags. None when it has none.
    entry: Option<Entry>,
}

/// The kind of the Part of flag, which World shows with its Change action.
const PART_OF: FlagKind = FlagKind::Unread(Field::PartOf);

/// Where a reader shows an entry's status: once (ruling R9, N9). A campaign entry whose reader
/// draws the editable Status row shows its status there, and its kind line has none
/// (`NPC · Campaign`); every other entry keeps its status on the kind line. The kind line's words
/// and the Status row both come from this one value, so the two can never both show it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReaderStatus {
    /// The Status row shows it, with its Change.
    Row,
    /// The kind line shows it (`NPC · alive`), or the entry has none.
    KindLine,
}

impl ReaderStatus {
    /// The place for this entry's status. `campaign` is the campaign whose entries the reader
    /// may change; `editing` is true while the Status field is open.
    pub fn of(item: &Item, index: &ContentIndex, campaign: bool, editing: bool) -> Self {
        let entry = &item.entry;
        let set = !status(entry).trim().is_empty();
        let row = if entry.kind != "world" || !campaign || !item.is_private() {
            false
        } else if index.duplicate_world.contains_key(&entry.path) {
            // A duplicate note's read-only values have their own Status line.
            set
        } else {
            set || status_unread(entry) || editing
        };
        if row { Self::Row } else { Self::KindLine }
    }

    /// The kind line's first words: the entry's kind, with its status only when no Status row
    /// shows it. A duplicate note's line names its file, not the fault its row names, so it
    /// takes the plain kind and status (`subtitle`), not its row's words.
    pub(crate) fn kind_words(self, item: &Item, duplicate: bool) -> String {
        match self {
            Self::Row => type_label(world_type(&item.entry)),
            Self::KindLine if duplicate => crate::library::presentation::subtitle(&item.entry),
            Self::KindLine => item.subtitle.clone(),
        }
    }
}

/// True when the note's Status value was not read.
fn status_unread(entry: &Entry) -> bool {
    entry
        .flags
        .iter()
        .any(|flag| flag.kind == FlagKind::Unread(Field::Status))
}

/// A value the reader could not read, as the shared reader says it (`src/library/view.rs`):
/// the sentence with a long value cut, then whole.
fn flag_words(flag: &splat::content::layout::Flag) -> (String, String) {
    (copy_words(flag.short_message()), copy_words(&flag.message))
}

/// The one copy of an unread warning's words, made when its fault is cached. The reader draws
/// from a borrow, so an unchanged frame makes none.
pub(super) fn copy_words(words: &str) -> String {
    words.to_owned()
}

impl GroupFault {
    /// The flags of `entry` other than the Part of one, and the entry without any flags.
    fn flags(
        entry: &Entry,
        unread: Option<&fields::Unread>,
    ) -> (Vec<(String, String)>, Option<Entry>) {
        if entry.flags.is_empty() {
            return (Vec::new(), None);
        }
        let notices = entry
            .flags
            .iter()
            // The Part of flag is the one with its line and kind; another flag can share a line.
            .filter(|flag| {
                unread.is_none_or(|unread| unread.line != flag.line || flag.kind != PART_OF)
            })
            .map(flag_words)
            .collect();
        let mut entry = entry.clone();
        entry.flags.clear();
        (notices, Some(entry))
    }
}

/// A new id for a lost Log draft's notice, so two notices from one entry stay apart. A
/// result's place in a footer is its `design::ResultOrder`.
pub fn next_order() -> u64 {
    static ORDER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    ORDER.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1
}

/// A reader's open `Add to Log` fields, by campaign and note. A change of its keys (a field
/// opens or closes) takes a new stamp, so the other reader rebuilds its copy of the keys only
/// then (`Reader::share_with`), not on each pass. Typing in a field keeps the stamp.
#[derive(Default)]
struct LogFields {
    fields: HashMap<LogKey, LogField>,
    /// 0 for none ever opened; else a stamp from `next_order`, new at each change of the keys.
    keys: u64,
}

impl LogFields {
    /// The field of this note, opened empty when it is not open.
    fn open(&mut self, key: LogKey) -> &mut LogField {
        if !self.fields.contains_key(&key) {
            self.keys = next_order();
        }
        self.fields.entry(key).or_default()
    }

    fn remove(&mut self, key: &LogKey) -> Option<LogField> {
        let removed = self.fields.remove(key);
        if removed.is_some() {
            self.keys = next_order();
        }
        removed
    }

    /// The words and state of an open field; its keys stay.
    fn get_mut(&mut self, key: &LogKey) -> Option<&mut LogField> {
        self.fields.get_mut(key)
    }

    #[cfg(test)]
    fn clear(&mut self) {
        if !self.fields.is_empty() {
            self.keys = next_order();
        }
        self.fields.clear();
    }
}

impl std::ops::Deref for LogFields {
    type Target = HashMap<LogKey, LogField>;
    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Rebuilds of a reader's copy of the other reader's Log keys, on this thread.
    pub(crate) static LOG_KEY_SHARES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// How many times a reader read a file on disk to check a result or a kept form (layout PR
    /// 9b-R1c's proof: none on a quiet frame).
    pub static READER_FILE_READS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Lookup's and the World page's reader, and its controls for the item on show.
/// What a reader holds for the DM that no reset of the reader may drop: the DM's typed words,
/// and the results whose end events are their own (`Kept::ENDS`). Lookup's Clear and World's
/// campaign reset replace the reader, and both carry this whole (`Reader::clear`,
/// `Reader::keep_log_lines`), so a new field here is kept by both at once (review of PR 333).
/// Each result shows only in its own campaign, so it can outlive a campaign change.
#[derive(Default)]
pub struct Held {
    /// Each open `Add to Log` field, by its entry and the session it was opened in. Its words
    /// wait while the reader shows another entry; a restart loses them, as every held form.
    log_fields: LogFields,
    /// The Log lines whose entry is gone, for the footer, until the DM copies or dismisses each.
    gone_log_lines: Vec<GoneLog>,
    /// Log lines the DM dismissed with their typed words, each with its Undo (ruling R3): the
    /// only copy of those words.
    dismissed: design::Results<GoneOwner, DismissedLog>,
    /// The kept results with an Undo, one of each kind (`Slot`), each with its order. A
    /// discarded draft's Undo holds the only copy of its words.
    results: design::Results<Slot, ReaderResult>,
}

#[derive(Default)]
pub struct Reader {
    /// What the reader holds for the DM across every reset (`Held`).
    held: Held,
    showing: Option<Uuid>,
    status: String,
    status_editing: bool,
    /// `Change` on Status was pressed: the field takes the keyboard, its words selected, once
    /// the frame of the press is over.
    status_select: bool,
    status_drafts: HashMap<Uuid, String>,
    /// Edit forms by entry. A form with changes stays while the DM reads other entries.
    edits: HashMap<Uuid, edit::EditDraft>,
    /// A field of the form had the keyboard when it last drew.
    form_focused: bool,
    pub compact: bool,
    other_type: String,
    adding: Option<Adding>,
    picker: Option<Picker>,
    /// The form closed with nothing to write: the note's path and its text, while it reads so.
    kept: Option<(String, String)>,
    /// The index's load at which the reader last compared its files on disk (a change's result,
    /// the kept form): the disk is read again only when a new load may hold an outside edit
    /// (layout PR 9b-R1c), never on a quiet frame.
    disk_checked: Option<u64>,
    error: Option<String>,
    /// Source lookup is cached by note and index generation, including repairs and moves.
    group_fault: Option<GroupFault>,
    group_error: bool,
    /// The refused Part of write could not read the note at all (no permission, not text), so
    /// no editor can open it either: its error has no `Edit in Obsidian` link.
    group_error_unread_file: bool,
    /// A link that matches several files, waiting for the DM to choose.
    many: Vec<usize>,
    many_heading: Option<String>,
    place_choices_open: bool,
    heading_document: Option<Uuid>,
    heading: Option<String>,
    /// What only the app can do, for it to take after this reader draws.
    pub join: Option<Join>,
    /// `Start fight` in the header, for the app to take after this reader draws.
    pub start_fight: Option<StartFight>,
    /// `Add to fight` on the title line of a creature's or an NPC's reader while a run is open
    /// (round 27 FJ5, round 28 FJ7), for the app to take after this reader draws, through the
    /// run's own add.
    pub add_to_fight: Option<AddToFight>,
    /// The campaign and the run that are open now, which the app sets each frame. A refusal
    /// of `Add to fight` belongs to them.
    pub run_owner: Option<(Uuid, Uuid)>,
    /// Why the run refused `Add to fight` for one entry, with its owner. It shows in the footer
    /// while that entry is open; a successful retry, the run's close and a campaign change end
    /// it (review of PR 300).
    add_refusal: Option<Refusal>,
    /// Why no fight can start now, as the encounter editor's `Start fight` says it.
    pub start_blocked: Option<&'static str>,
    /// The encounter editor is the page beside this reader: its own primary `Start fight` is the
    /// one there, so the header offers none.
    pub beside_editor: bool,
    pub open_timeline: Option<String>,
    pub sides: RunSides,
    /// A run is in live play and owns `Next turn`, the screen's primary action.
    pub live: bool,
    pub in_run: std::collections::BTreeMap<Uuid, (usize, Option<i64>)>,
    pub select_run: Option<usize>,
    /// The session being played, as a Log line names it: `[[Session 1]]`. None when no
    /// session is being played (`played_session`).
    pub played: Option<String>,
    /// The campaign's day on the Timeline's clock, when the clock has one.
    pub day: Option<u64>,
    /// The entries whose edit form is open in the other reader (Lookup's, or the World
    /// page's). Their Log field waits too, so Splat never makes its own disk conflict.
    pub edited_elsewhere: Vec<Uuid>,
    /// The other reader's open Log fields, by campaign and note. An edit form here holds them
    /// back too, so its save row says so (ruling Q-t): the waiting state is the two readers'.
    pub logs_elsewhere: std::collections::HashSet<LogKey>,
    /// The other reader's `LogFields` stamp that `logs_elsewhere` was copied from.
    logs_elsewhere_from: u64,
    /// The note whose edit form the other reader drew in its last pass. Its save row explains
    /// a waiting Log field, so this reader's footer does not say it again.
    pub form_shown_elsewhere: Option<Uuid>,
    /// The note whose edit form this reader drew, and the pass it drew it in.
    form_drawn: Option<(Uuid, u64)>,
    /// Whether the entry on show has prose, for the entry and the index's load: its body is
    /// parsed for it once, not in every frame (layout PR 9b-R1a).
    prose: Option<(Uuid, u64, bool)>,
}

/// A reader's `Add to fight`: what joins the open run, by the run's own add
/// (`Run::add_entry_to_fight`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddToFight {
    /// The entry on show, a creature or an NPC: a refusal is its own.
    pub entry: Uuid,
    /// The stat block that joins.
    pub stat_block: Uuid,
    /// The entry is an NPC, who joins under their own name with that stat block.
    pub npc: bool,
}

/// Why the run refused a reader's `Add to fight`, and whose refusal it is.
#[derive(Clone, Debug, PartialEq)]
struct Refusal {
    /// The campaign and the run that refused.
    owner: (Uuid, Uuid),
    /// The entry whose `Add to fight` was refused.
    entry: Uuid,
    words: String,
}

/// A result that the World page owns, drawn by the reader in the one place for results.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PageResult {
    Repaired,
    /// A generated NPC that Discard let go, by name, with `Undo discard`.
    Discarded(String),
}

impl PageResult {
    /// The page's result as a footer outcome. Its Undo is the page's (`FooterAct::Page`).
    pub fn outcome(&self) -> KeptOutcome<FooterAct> {
        match self {
            Self::Repaired => Said::new(crate::sentence!("Note repaired.")).into(),
            Self::Discarded(name) => discarded_outcome(name, FooterAct::Page),
        }
    }
}

/// `<name> draft discarded.` with `Undo discard of <name>`, the name every discarded draft's
/// Undo has in the app; its visible label gives way by whole words where the footer is narrow.
pub(crate) fn discarded_outcome(name: &str, act: FooterAct) -> KeptOutcome<FooterAct> {
    KeptOutcome::Notice(
        Notice::new(crate::sentence!("{} draft discarded.", name)),
        UndoStep::of(Verb::Discard, name).pressed_as(act),
    )
}

/// Whose an `Add to Log` field is: the campaign and the entry's ID, nothing else. A copied note
/// in another campaign has the same entry ID, so the campaign is part of the key, and the words
/// never reach another campaign's note. The session is not: the stamp is the session being
/// played at Enter, so a renamed, closed or changed session cannot hide typed words.
pub(crate) type LogKey = (Uuid, Uuid);

/// The typed words of a Log line whose entry is gone: the footer says them, with `Copy text`,
/// until the DM copies or dismisses them.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GoneLog {
    /// This notice's own number, so two drafts lost from one entry stay two notices.
    id: u64,
    /// The campaign of the entry: the notice shows only there.
    campaign: Uuid,
    name: String,
    /// The words as the DM typed them, byte for byte, for `Copy text`.
    words: String,
}

/// One open `Add to Log` field: its words, and whether its next draw selects them.
#[derive(Default)]
struct LogField {
    field: design::AddField,
    /// The entry's name, for the warnings that list unsaved work.
    name: String,
    /// The words came from a done scene, so typing replaces them and Enter keeps them.
    select: bool,
}

/// Where a reader's title-line actions go (round 27, one rule for World, Lookup and the
/// encounter editor's header): `None` when the measured title, the `L` gap and every action
/// with its word fit in `room`, so they follow the title on its line, `XS` apart. Else the row,
/// under the title, of each action: all of them move together, in order, on one row when that
/// row fits in `line`, the width under the title. Only when it does not fit does it wrap, at
/// whole actions, each row filled from the left with as many as fit. An action never goes
/// away and never loses its word, whatever the length of the name. `title` is the title's measured width, `widths`
/// each action's own.
pub fn title_line_rows(room: f32, line: f32, title: f32, widths: &[f32]) -> Option<Vec<usize>> {
    // The one decision lives in the design system (`design::title_placement`); the reader's
    // first action is quiet, so its icon sits `L` after the title.
    match design::title_placement(title, design::title_lead_in(true), widths, room, line) {
        design::TitlePlacement::OnTitle => None,
        design::TitlePlacement::Under(rows) => Some(
            rows.iter()
                .enumerate()
                .flat_map(|(row, range)| range.clone().map(move |_| row))
                .collect(),
        ),
    }
}

/// What a reader pane's footer can press. A kept result's press comes with its ticket, and the
/// store it came from acts by it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FooterAct {
    /// The Undo of a result of the reader's store (`Reader::results`).
    Undo,
    /// The page result's Undo: the World page's store.
    Page,
    /// `Undo remove` of a row of a tracker's history: the trackers own it.
    Trackers,
    /// `Copy text` of a Log line whose entry is gone, by its notice.
    GoneCopy(u64),
    /// The close icon of that notice: its typed words go, with an Undo (ruling R3).
    GoneDismiss(u64),
    /// The Undo of a dismissed Log line (`Reader::dismissed`).
    UndoDismiss,
    /// `Copy text` of a draft whose note is gone, by the note's ID.
    LostCopy(Uuid),
    /// `Discard draft` of a draft whose note is gone, by the note's ID.
    LostDiscard(Uuid),
    /// `Undo tick` of an open question: the World page's ticks.
    Untick,
    /// `Edit in Obsidian` of the file the error names.
    OpenEditor(std::path::PathBuf),
}

/// Trackers can draw in Lookup and in a note's reader without giving those screens its state.
pub trait TrackerUi {
    fn history_footer<'f>(
        &self,
        note: Uuid,
        frame: &'f design::outcome::PaneFrame,
        messages: &mut design::outcome::Messages<'f, FooterAct>,
    );
    /// The last Remove's Undo, by the press's ticket (M-S0.4).
    fn undo_remove(&mut self, ticket: Option<design::results::Ticket>);
}

/// The reader's kinds of kept result, one each: a later one of a kind replaces the earlier
/// (`Results::record`). A reader is one pane (World's or Lookup's), so its results are its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Slot {
    /// A value, edit-form, row or Log change.
    Change,
    FileRename,
    Name,
    Identity,
    /// An edit draft that Discard let go.
    Discarded,
    /// An edit draft that a disk conflict's Reload replaced.
    Reloaded,
}

impl design::results::Owner for Slot {
    type Scope = ();
    fn scope(&self) -> &() {
        &()
    }
}

/// A kept result of the reader, with what its Undo needs. Each shows only on its own campaign's
/// note (`shows_on`). End events: its own Undo; a later result of its kind; the owner's ends
/// that `Reader::end` names (another note opens, an Edit, a repair).
enum ReaderResult {
    Change(Change),
    FileRename(FileRename),
    Name(Change),
    /// The campaign it was made in, and the change.
    Identity(Uuid, Change),
    /// The entry and the draft let go.
    Discarded(Uuid, Box<edit::EditDraft>),
    /// The entry whose open draft Reload replaced; the draft keeps what Undo brings back.
    Reloaded(Uuid),
}

impl design::results::Kept for ReaderResult {
    type Owner = Slot;
    const ENDS: design::results::Ends = design::results::Ends {
        owner_changed: design::results::OwnerChanged::Ends,
        disk_changed: design::results::DiskChanged::Stays,
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> Slot {
        match self {
            Self::Change(_) => Slot::Change,
            Self::FileRename(_) => Slot::FileRename,
            Self::Name(_) => Slot::Name,
            Self::Identity(..) => Slot::Identity,
            Self::Discarded(..) => Slot::Discarded,
            Self::Reloaded(_) => Slot::Reloaded,
        }
    }
}

/// A Log line the DM dismissed with its typed words: its Undo puts the notice back.
pub(crate) struct DismissedLog(pub GoneLog);

/// One dismissed Log line, by its notice's number, in its campaign.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct GoneOwner {
    campaign: Uuid,
    id: u64,
}

impl design::results::Owner for GoneOwner {
    type Scope = Uuid;
    fn scope(&self) -> &Uuid {
        &self.campaign
    }
}

impl design::results::Kept for DismissedLog {
    type Owner = GoneOwner;
    const ENDS: design::results::Ends = design::results::Ends {
        owner_changed: design::results::OwnerChanged::Stays,
        disk_changed: design::results::DiskChanged::Stays,
        press: design::results::Press::Undo,
    };
    fn owner(&self) -> GoneOwner {
        GoneOwner {
            campaign: self.0.campaign,
            id: self.0.id,
        }
    }
}

/// Words the app made from the DM's own (a name, a value, a file's error) as one sentence: the
/// counted escape for words from outside.
#[allow(
    clippy::disallowed_methods,
    reason = "the words hold the DM’s names and values, or the system’s error"
)]
pub fn said(words: &str) -> Sentence {
    Sentence::outside(words).unwrap_or(crate::sentence!("Done."))
}

/// A reader's fault for its footer, by kind: it gives the tone.
pub enum ReaderFault {
    /// The last action did not write (a refused or failed write, a file that cannot be read).
    Action(String),
    /// Typed words that wait: a Log line whose entry is gone, a draft whose note is gone.
    Waiting(String),
    /// What the DM fixes before it counts, in the reader's body (M20): an unread value, a name
    /// that is not its file's, the index's warning on this note.
    Fix(Sentence),
    /// A note that cannot be read, in place of its page (its fault and the recovery steps).
    Unreadable(String),
}

impl Fault for ReaderFault {
    fn severity(&self) -> Severity {
        match self {
            Self::Action(_) => Severity::NotSaved,
            Self::Waiting(_) | Self::Fix(_) => Severity::NeedsFixing,
            Self::Unreadable(_) => Severity::CannotRead,
        }
    }
    fn cause(&self) -> Sentence {
        match self {
            Self::Action(words) | Self::Waiting(words) | Self::Unreadable(words) => said(words),
            Self::Fix(words) => words.clone(),
        }
    }
}

impl Reader {
    /// The run refused `Add to fight` for `entry`: its reason waits in the footer, owned by the
    /// campaign and run open now (`run_owner`). No other error changes.
    pub fn refuse(&mut self, entry: Uuid, reason: String) {
        self.add_refusal = self.run_owner.map(|owner| Refusal {
            owner,
            entry,
            words: reason,
        });
    }

    /// `Add to fight` for `entry` succeeded: its refusal, if any, is over.
    pub fn added_to_fight(&mut self, entry: Uuid) {
        if self
            .add_refusal
            .as_ref()
            .is_some_and(|refusal| refusal.entry == entry)
        {
            self.add_refusal = None;
        }
    }

    /// The refusal of `Add to fight` for `entry`, while its campaign and run are still the open
    /// ones. A closed run or another campaign ends it here.
    fn refusal_for(&mut self, entry: Uuid) -> Option<&str> {
        if self
            .add_refusal
            .as_ref()
            .is_some_and(|refusal| Some(refusal.owner) != self.run_owner)
        {
            self.add_refusal = None;
        }
        self.add_refusal
            .as_ref()
            .filter(|refusal| refusal.entry == entry)
            .map(|refusal| refusal.words.as_str())
    }

    /// The stamp that a change's Log line begins with: the session being played and the day,
    /// `[[Session 1]], Day 1`. With no session, a change writes no Log line.
    fn log_as(&self) -> Option<String> {
        let session = self.played.as_ref()?;
        Some(match self.day {
            Some(day) => format!("{session}, Day {day}"),
            None => session.clone(),
        })
    }

    /// The stamp of an `Add to Log` line: a change's stamp, or the day alone when no session
    /// is being played, because the DM asked for a line.
    fn line_stamp(&self) -> Option<String> {
        self.log_as()
            .or_else(|| self.day.map(|day| format!("Day {day}")))
    }

    /// Open `Add to Log` for this entry of this campaign, as a done scene's `Add to Log` and the
    /// header's do. The field starts with `words`, selected, unless the DM already typed words
    /// there.
    pub fn open_log(&mut self, campaign: Uuid, id: Uuid, name: &str, words: &str) {
        let open = self.held.log_fields.open((campaign, id));
        open.field.focus = true;
        name.clone_into(&mut open.name);
        if open.field.words.trim().is_empty() {
            words.clone_into(&mut open.field.words);
            open.select = !words.is_empty();
        }
    }

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

    /// Draw any item: a plain note, a world entry, or an entry of another kind.
    /// A newly opened note starts with its own state: the last note's change, kept form,
    /// error and pickers end here. Both the footer and the body call it first.
    fn prepare(&mut self, item: &Item) {
        if self.showing != Some(item.entry.id) {
            self.end(Slot::Change);
            self.kept = None;
            if let Some(previous) = self.showing {
                self.status_drafts.insert(previous, self.status.clone());
            }
            self.showing = Some(item.entry.id);
            self.group_fault = None;
            self.group_error = false;
            self.group_error_unread_file = false;
            self.status = self
                .status_drafts
                .remove(&item.entry.id)
                .unwrap_or_else(|| status(&item.entry).to_owned());
            self.status_editing = self.status != status(&item.entry);
            self.status_select = false;
            // Only a form with changes is worth keeping. It opens again where it was left.
            self.edits.retain(|_, draft| draft.dirty());
            self.other_type.clear();
            self.adding = None;
            self.picker = None;
            self.error = None;
        }
    }

    /// No file read, source scan or entry copy occurs before the cache lookup.
    fn refresh_group_fault(
        &mut self,
        item: &Item,
        generation: u64,
        read: impl FnOnce() -> Option<String>,
    ) {
        if self
            .group_fault
            .as_ref()
            .is_some_and(|fault| fault.id == item.entry.id && fault.generation == generation)
        {
            return;
        }
        let source = if item.entry.flags.is_empty() && !self.group_error {
            None
        } else {
            read()
        };
        let unread = source
            .as_ref()
            .and_then(|file| fields::unread_groups(&item.entry, file));
        if self.group_error
            && unread.is_none()
            && source.as_ref().is_some_and(|file| {
                Entry::parse(&item.entry.path, file)
                    .is_ok_and(|entry| fields::unread_groups(&entry, file).is_none())
            })
        {
            self.error = None;
            self.group_error = false;
            self.group_error_unread_file = false;
        }
        // Draw each unread value once, below the file line.
        let (notices, entry) = GroupFault::flags(&item.entry, unread.as_ref());
        self.group_fault = Some(GroupFault {
            id: item.entry.id,
            generation,
            unread,
            notices,
            entry,
        });
    }

    fn change(
        &mut self,
        campaign: &Campaign,
        item: &Item,
        index: &ContentIndex,
        key: &str,
        value: Value,
    ) -> Option<Go> {
        // A picker can outlive an outside edit. Re-read before deriving a replacement or Log.
        let fresh = if key == "part_of" {
            // The last value says whether the file itself could not be read.
            let result = (|| {
                let file = file_fault::read_note(&campaign.path, &item.entry.path)
                    .map_err(|error| (error, None, true))?;
                let entry =
                    Entry::parse(&item.entry.path, &file).map_err(|error| (error, None, false))?;
                if let Some(unread) = fields::unread_groups(&entry, &file) {
                    return Err((
                        unread.reason.clone(),
                        Some((unread, Box::new(entry))),
                        false,
                    ));
                }
                Ok((file, entry))
            })();
            match result {
                Ok(fresh) => Some(fresh),
                Err((error, unread, unread_file)) => {
                    self.picker = None;
                    self.error = Some(error);
                    self.group_error = true;
                    self.group_error_unread_file = unread_file;
                    // The file on disk has an unread list now. Show that state at once, and
                    // read the note again so its list row and the header count it too.
                    let (unread, entry) = unread?;
                    if let Some(fault) = &mut self.group_fault
                        && fault.id == item.entry.id
                    {
                        (fault.notices, fault.entry) = GroupFault::flags(&entry, Some(&unread));
                        fault.unread = Some(unread);
                    }
                    return Some(Go::Reload(item.entry.id));
                }
            }
        } else {
            None
        };
        let current_entry = fresh.as_ref().map_or(&item.entry, |(_, entry)| entry);
        let value = if key == "part_of" {
            // Apply only the picker's add/remove to the fresh disk list.
            let previous: Vec<_> = part_of(&item.entry).into_iter().cloned().collect();
            let disk: Vec<_> = part_of(current_entry).into_iter().cloned().collect();
            Value::Sequence(merge_groups(index, &previous, value.as_sequence()?, &disk))
        } else {
            value
        };
        let label = match key {
            "in" => "Place",
            "part_of" => "Part of",
            "status" => "Status",
            _ => "Stat block",
        };
        // Two notes with one name: the one a value opens is named with its folder, so the
        // result tells the DM which file the change chose (AD2, FD3, SD2).
        let one_name = |value: &Value| match edit::value_target(index, value) {
            Some(at) => index.shown_name(at),
            None => edit::value_name(index, value, label),
        };
        // `None` is a value that is not set: no words, an empty list or blank text.
        let name = |value: &Value| match value.as_sequence() {
            Some(groups) if groups.is_empty() => None,
            // Each value by the name the reader shows: the note its link opens, else the words
            // written in the file.
            Some(groups) => Some(groups.iter().map(one_name).collect::<Vec<_>>().join(", ")),
            None if empty_optional(value) => None,
            // A status is a word, not a link: the words in the file, as its row shows them.
            None if key == "status" => value.as_str().map(|word| word.trim().to_owned()),
            None => Some(one_name(value)),
        };
        // Choosing what is already there changes nothing, and writes no Log line.
        let current = world_field(current_entry, key).filter(|value| !value.is_null());
        let same = match (key, current) {
            ("part_of", _) => false,
            ("status", _) => value.as_str().map(str::trim) == Some(status(current_entry)),
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
        // The row, the result and the Undo say one name, `Place` (ruling Q-p, R16).
        let words = change_words(
            label,
            world_field(current_entry, key).and_then(name).as_deref(),
            name(&value).as_deref(),
        );
        // What play changes goes in the entry's Log, in the same write, so one Undo takes
        // both back.
        let log = self
            .log_as()
            .filter(|_| matches!(key, "in" | "part_of" | "status"))
            .map(|stamp| {
                let words = log_words(index, key, world_field(current_entry, key), &value);
                format!("{stamp}: {words}")
            });
        let value = edit::stored_value(index, value);
        let result = (|| {
            let before = match &fresh {
                Some((file, _)) => file.clone(),
                None => file_fault::read_note(&campaign.path, &item.entry.path)?,
            };
            let patched = fields::patch(&before, key, value)?;
            let after = match &log {
                Some(line) => lines::add_log(&patched, line),
                None => patched,
            };
            write_note(&campaign.path, &item.entry.path, Some(&before), &after)?;
            Ok::<_, String>(Change {
                campaign: campaign.id,
                path: item.entry.path.clone(),
                before: before.clone(),
                after: after.clone(),
                word: words,
                kind: ChangeKind::of_key(key),
            })
        })();
        let go = self.written(result, item.entry.id);
        if go.is_some() {
            self.picker = None;
            self.place_choices_open = false;
        }
        go
    }

    /// `Add to Log`: one line in the entry's own note, at the end of its Log, stamped as a
    /// change is at this moment. It reads the file now, so an edit made a moment before stays,
    /// and writes only when the file still holds what it read. A failed write keeps the field
    /// and its words, and names the file.
    fn add_log_line(&mut self, campaign: &Campaign, item: &Item, words: &str) -> Option<Go> {
        let line = match self.line_stamp() {
            Some(stamp) => format!("{stamp}: {words}"),
            None => words.to_owned(),
        };
        let result = (|| {
            let before = file_fault::read_note(&campaign.path, &item.entry.path)?;
            let after = lines::add_log(&before, &line);
            write_note(&campaign.path, &item.entry.path, Some(&before), &after)?;
            Ok::<_, String>(Change {
                campaign: campaign.id,
                path: item.entry.path.clone(),
                before,
                after,
                word: format!("Added to {}’s Log.", item.entry.name),
                kind: ChangeKind::Log,
            })
        })();
        let go = self.written(result, item.entry.id);
        if go.is_some() {
            self.held.log_fields.remove(&(campaign.id, item.entry.id));
        }
        go
    }

    /// A change to one World note, written or not: its result and Undo, or its error, in the
    /// reader's one place for each.
    fn written(&mut self, result: Result<Change, String>, id: Uuid) -> Option<Go> {
        match result {
            Ok(change) => {
                self.error = None;
                self.group_error = false;
                self.group_error_unread_file = false;
                self.held.results.record(ReaderResult::Change(change));
                self.kept = None;
                Some(Go::Reload(id))
            }
            Err(error) => {
                self.error = Some(error);
                None
            }
        }
    }

    /// The Part of reason while this note's list could not be read.
    fn unread_groups(&self, id: Uuid) -> Option<String> {
        self.group_fault
            .as_ref()
            .filter(|fault| fault.id == id)
            .and_then(|fault| fault.unread.as_ref())
            .map(|unread| unread.reason.clone())
    }

    pub fn clear_notice(&mut self) {
        self.end(Slot::Change);
        self.kept = None;
    }

    /// The change whose result the reader keeps, for the checks.
    #[cfg(test)]
    pub fn last_change(&self) -> Option<Change> {
        match self.result(Slot::Change)? {
            ReaderResult::Change(change) => Some(change.clone()),
            _ => None,
        }
    }

    /// The reader's kept result of this kind.
    fn result(&self, slot: Slot) -> Option<&ReaderResult> {
        self.held.results.get(&slot).map(|shown| shown.value())
    }

    /// End the reader's result of this kind (its owner changed: another note, an Edit).
    fn end(&mut self, slot: Slot) {
        self.held.results.owner_changed(&slot);
    }

    /// A repair of the note at `path` ends a name change's result on it: it has nothing left
    /// to undo.
    pub fn end_name_change_on(&mut self, campaign: &Campaign, path: &str) {
        if matches!(self.result(Slot::Name), Some(ReaderResult::Name(change)) if change.shows_on(campaign, path))
        {
            self.end(Slot::Name);
        }
    }

    /// This reader's messages for the open note, for its pane's footer (round 26): every result
    /// with its Undo from the reader's stores, then the reasons, then the notices. Each keeps
    /// its owner and its end events: a change, a kept form and an error belong to the open note
    /// and end when another note opens (`prepare`), a later change replaces them, and Undo ends
    /// them; a rename, a name change and a separate note end at their Undo or the next of their
    /// kind. The page adds its own result. Take the press with `footer_act`.
    pub fn footer<'f>(
        &mut self,
        frame: &'f PaneFrame,
        messages: &mut Messages<'f, FooterAct>,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
    ) {
        self.prepare(item);
        let id = item.entry.id;
        let editing = campaign.is_some_and(|campaign| self.editing(id, campaign));
        self.gone_log_messages(frame, messages, campaign.map(|campaign| campaign.id));
        // A draft whose note is gone, and its Undo, whatever note is open, also in an edit form.
        self.lost_messages(messages, index);
        // Why the open run refused this entry's `Add to fight`.
        if let Some(words) = self.refusal_for(id) {
            messages.outcome(Outcome::Refused(said(words)));
        }
        // An open `Add to Log` field waits while an edit form of its entry is open, here or in
        // the other reader. It ends when the form closes (Save, Discard) or the reader shows
        // another entry.
        // One save row says it (ruling Q-t): the form's, in this reader or the other. This
        // footer says it only while the form is open in the other reader and its save row is
        // not on screen there.
        if let Some(campaign) = campaign
            && takes_log(item, index)
            && self.footer_names_log((campaign.id, id))
        {
            messages.outcome(Notice::new(crate::sentence!(
                "Save or discard your edit to add a Log line."
            )));
        }
        let has_unread = self
            .group_fault
            .as_ref()
            .filter(|fault| !editing && fault.id == id)
            .is_some_and(|fault| fault.unread.is_some());
        // A refused Part of write shows as the unread warning below the file line instead.
        if let Some(error) = &self.error
            && !(self.group_error && has_unread)
        {
            let mut fault = FaultMessage::new(&ReaderFault::Action(error.clone()));
            // A file that Splat cannot read, no editor can open either.
            if let Some(campaign) =
                campaign.filter(|_| self.group_error && !self.group_error_unread_file)
            {
                let path = campaign.path.join(&item.entry.path);
                fault = fault.choice(
                    Choice::new(
                        MessageChoice::Link {
                            word: "Edit in Obsidian",
                            hover: None,
                        },
                        FooterAct::OpenEditor(path.clone()),
                    )
                    .hover(path.display().to_string()),
                );
            }
            messages.outcome(fault);
        }
        let Some(campaign) = campaign else {
            return;
        };
        if matches!(self.result(Slot::Identity), Some(ReaderResult::Identity(owner, _)) if *owner != campaign.id)
        {
            self.end(Slot::Identity);
        }
        let duplicate = index.duplicate_world.contains_key(&item.entry.path);
        let path = item.entry.path.as_str();
        for slot in [
            Slot::Change,
            Slot::Reloaded,
            Slot::Discarded,
            Slot::FileRename,
            Slot::Name,
            Slot::Identity,
        ] {
            let Some(shown) = self.held.results.get(&slot) else {
                continue;
            };
            let shows = match shown.value() {
                // A draft let go whose note is gone keeps its Undo while another entry is edited
                // (the next Edit ends the Undo of its own entry).
                ReaderResult::Discarded(..) => self.discarded_shows(campaign, id, index).is_some(),
                // Its own entry's open form, while the draft still holds what Undo restores.
                ReaderResult::Reloaded(at) => {
                    *at == id
                        && editing
                        && self
                            .edits
                            .get(at)
                            .is_some_and(edit::EditDraft::was_reloaded)
                }
                _ if editing => false,
                ReaderResult::Change(change) => change.shows_on(campaign, path),
                ReaderResult::FileRename(rename) => {
                    item.entry.kind == "world" && !duplicate && rename.shows_on(campaign, path)
                }
                ReaderResult::Name(change) => {
                    item.entry.kind == "world" && !duplicate && change.shows_on(campaign, path)
                }
                ReaderResult::Identity(..) => true,
            };
            if !shows {
                continue;
            }
            messages.kept(shown.outcome(frame, |result| {
                match result {
                    ReaderResult::Change(change) | ReaderResult::Name(change) => {
                        Said::new(said(&change.word))
                            .undo(change.undo_step(&item.entry.name).pressed_as(FooterAct::Undo))
                            .into()
                    }
                    ReaderResult::Identity(_, change) => Said::new(said(change.message()))
                        .undo(change.undo_step(&item.entry.name).pressed_as(FooterAct::Undo))
                        .into(),
                    ReaderResult::FileRename(rename) => Said::new(said(&rename.message()))
                        .undo(UndoStep::of(Verb::Rename, "file").pressed_as(FooterAct::Undo))
                        .into(),
                    ReaderResult::Discarded(_, draft) => {
                        discarded_outcome(&draft.name, FooterAct::Undo)
                    }
                    ReaderResult::Reloaded(_) => KeptOutcome::Notice(
                        Notice::new(crate::sentence!(
                            "Reloaded from disk. Your previous draft is kept until you edit or the file changes again."
                        )),
                        UndoStep::of(Verb::Reload, &item.entry.name).pressed_as(FooterAct::Undo),
                    ),
                }
            }));
        }
        // The form closed because the note already held every change. Nothing to undo.
        if !editing
            && edit::editable(item, index)
            && self
                .kept
                .as_ref()
                .is_some_and(|(path, _)| *path == item.entry.path)
        {
            // `edit::ALREADY_SAVED`.
            messages.outcome(Notice::new(crate::sentence!(
                "Your changes are already in the note."
            )));
        }
    }

    /// `Undo` of a kept result of the reader's store, by its ticket: a replaced or ended result
    /// (`Gone`) does nothing; a failed Undo keeps the result and says why.
    pub(crate) fn undo_result(
        &mut self,
        ticket: design::results::Ticket,
        open: Option<Uuid>,
        campaign: &Campaign,
    ) -> Option<Go> {
        match self.held.results.undo_by(ticket, |result| match result {
            ReaderResult::Change(change) | ReaderResult::Name(change) => change.undo(campaign),
            ReaderResult::Identity(_, change) => change.undo(campaign),
            ReaderResult::FileRename(rename) => rename.undo(campaign),
            ReaderResult::Discarded(..) | ReaderResult::Reloaded(_) => Ok(()),
        }) {
            Ok((ReaderResult::Discarded(id, draft), ())) => {
                self.edits.insert(id, *draft);
                None
            }
            Ok((ReaderResult::Reloaded(id), ())) => {
                if let Some(draft) = self.edits.get_mut(&id) {
                    draft.undo_reload();
                }
                None
            }
            Ok((ReaderResult::Identity(_, change), ())) => {
                // A finished Undo ends the reason an earlier action left.
                self.error = None;
                Some(Go::Reload(note_id(&change.path)))
            }
            Ok(_) => {
                self.error = None;
                open.map(Go::Reload)
            }
            Err(design::results::ByTicket::Failed(error)) => {
                self.error = Some(error);
                None
            }
            Err(design::results::ByTicket::Gone) => None,
        }
    }
}

/// The words for a held NPC, in the rail and in the row above another view. The rail's link
/// is found by this constant.
pub const UNSAVED_NPC: &str = "Unsaved NPC";

#[cfg(test)]
mod tests {
    #[test]
    fn iter9_lead_duplicate_with_one_path_names_no_other_file() {
        // Rook.md (world), Rook item.md (same id, an item) and Rook copy.md (world) leave
        // Rook copy.md with one duplicate path; its first action must not read a second one.
        let one = vec!["Rook copy.md".to_string()];
        assert!(links_consequence(&one, 0).is_none());
        let two = vec!["Rook.md".to_string(), "Rook copy.md".to_string()];
        let words = |at| links_consequence(&two, at).map(|help| help.as_str().to_owned());
        assert_eq!(
            words(0).as_deref(),
            Some("Existing links keep Rook copy.md.")
        );
        assert_eq!(
            words(1).as_deref(),
            Some("Existing links will then use Rook.md.")
        );
    }

    use super::*;

    /// Round 24 FD5: `Rename the file to <name>.md` and its Undo never replace or lose a file.
    /// Bad inputs: a name that cannot be a file name, a file already at the target, a note
    /// changed on disk since the DM saw it, the note moved away, and Undo after either side
    /// changed.
    #[test]
    fn iter24_d_fd5_file_rename_is_guarded_and_undone() {
        assert_eq!(
            renamed_path("Notes/Broken.md", "Repaired watchkeeper").as_deref(),
            Some("Notes/Repaired watchkeeper.md")
        );
        assert_eq!(
            renamed_path("Broken.md", "Rook").as_deref(),
            Some("Rook.md")
        );
        for name in ["", "  ", "A/B", "Who?", ".hidden", "[[Rook]]", "Broken"] {
            assert_eq!(renamed_path("Notes/Broken.md", name), None, "{name:?}");
        }
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Rename checks".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        std::fs::create_dir_all(root.join("Notes")).unwrap();
        let text = "---\nsplat_schema: 2\nsplat_kind: world\nsplat_name: Rook\n---\n";
        let from = root.join("Notes/Broken.md");
        let to = root.join("Notes/Rook.md");
        std::fs::write(&from, text).unwrap();
        // A file already at the target keeps its bytes, and the note stays where it was.
        std::fs::write(&to, "Another note.").unwrap();
        let refused = FileRename::apply(&campaign, "Notes/Broken.md", "Notes/Rook.md", text);
        assert!(refused.is_err(), "fd5-target-exists");
        assert_eq!(std::fs::read_to_string(&to).unwrap(), "Another note.");
        assert_eq!(std::fs::read_to_string(&from).unwrap(), text);
        std::fs::remove_file(&to).unwrap();
        // Review repair 2: a file that arrives at the target after the check, just before the
        // move, keeps its bytes, and so does the note.
        let raced = move_note(root, "Notes/Broken.md", "Notes/Rook.md", text, || {
            std::fs::write(&to, "Arrived during the move.").unwrap()
        });
        assert!(raced.is_err(), "review-2-race-refused");
        assert_eq!(
            std::fs::read_to_string(&to).unwrap(),
            "Arrived during the move.",
            "review-2-race-target-bytes"
        );
        assert_eq!(
            std::fs::read_to_string(&from).unwrap(),
            text,
            "review-2-race-source-bytes"
        );
        std::fs::remove_file(&to).unwrap();
        // Review re-check 3: a file system that does not offer RENAME_NOREPLACE (EINVAL). A hard
        // link and the removal of the old name move the note instead.
        let einval = |_: &Path, _: &Path| Err(std::io::Error::from_raw_os_error(libc::EINVAL));
        let link_moves = Moves {
            rename: einval,
            link: |source, target| std::fs::hard_link(source, target),
            remove: |path| std::fs::remove_file(path),
        };
        move_note_with(
            &link_moves,
            root,
            "Notes/Broken.md",
            "Notes/Rook.md",
            text,
            || {},
        )
        .expect("re-check-3-fallback");
        assert!(!from.exists(), "re-check-3-fallback-moved");
        assert_eq!(
            std::fs::read_to_string(&to).unwrap(),
            text,
            "re-check-3-fallback-bytes"
        );
        move_note_with(
            &link_moves,
            root,
            "Notes/Rook.md",
            "Notes/Broken.md",
            text,
            || {},
        )
        .expect("re-check-3-fallback-back");
        // The fallback never replaces a file that arrives during the move.
        let raced = move_note_with(
            &link_moves,
            root,
            "Notes/Broken.md",
            "Notes/Rook.md",
            text,
            || std::fs::write(&to, "Arrived during the move.").unwrap(),
        );
        assert!(raced.is_err(), "re-check-3-fallback-race");
        assert_eq!(
            std::fs::read_to_string(&to).unwrap(),
            "Arrived during the move."
        );
        assert_eq!(std::fs::read_to_string(&from).unwrap(), text);
        std::fs::remove_file(&to).unwrap();
        // Neither move is offered (hard links refused with EPERM, as on FAT): a plain refusal,
        // and both names keep what they held.
        let no_moves = Moves {
            rename: einval,
            link: |_, _| Err(std::io::Error::from_raw_os_error(libc::EPERM)),
            remove: |path| std::fs::remove_file(path),
        };
        assert_eq!(
            move_note_with(
                &no_moves,
                root,
                "Notes/Broken.md",
                "Notes/Rook.md",
                text,
                || {}
            ),
            Err(NO_SAFE_MOVE.to_owned()),
            "re-check-3-refused"
        );
        assert_eq!(
            std::fs::read_to_string(&from).unwrap(),
            text,
            "re-check-3-source"
        );
        assert!(!to.exists(), "re-check-3-target");
        // Re-check 2, repair 1: the link works, then the old name cannot be removed (a folder
        // made read only). The new name goes again, so nothing moved, and the words say so.
        let keep_old = Moves {
            rename: einval,
            link: |source, target| std::fs::hard_link(source, target),
            remove: |path| {
                if path.ends_with("Broken.md") {
                    Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied))
                } else {
                    std::fs::remove_file(path)
                }
            },
        };
        let refused = move_note_with(
            &keep_old,
            root,
            "Notes/Broken.md",
            "Notes/Rook.md",
            text,
            || {},
        );
        assert_eq!(
            refused,
            Err("Splat has no permission to save Notes/Broken.md. Nothing was renamed.".to_owned()),
            "re-check-2-1-new-name-removed"
        );
        assert!(
            std::fs::read_to_string(&from).unwrap() == text && !to.exists(),
            "re-check-2-1-one-name"
        );
        // Neither name can be removed: both stay, and the words name both and the way out.
        let keep_both = Moves {
            rename: einval,
            link: |source, target| std::fs::hard_link(source, target),
            remove: |_| Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied)),
        };
        let refused = move_note_with(
            &keep_both,
            root,
            "Notes/Broken.md",
            "Notes/Rook.md",
            text,
            || {},
        );
        assert_eq!(
            refused,
            Err(two_names("Notes/Broken.md", "Notes/Rook.md")),
            "re-check-2-1-two-names"
        );
        assert!(
            refused
                .unwrap_err()
                .contains("Delete Notes/Rook.md in your file manager"),
            "re-check-2-1-way-out"
        );
        assert!(
            std::fs::read_to_string(&from).unwrap() == text
                && std::fs::read_to_string(&to).unwrap() == text,
            "re-check-2-1-both-names"
        );
        std::fs::remove_file(&to).unwrap();
        // The note changed on disk after the DM saw it.
        let refused = FileRename::apply(&campaign, "Notes/Broken.md", "Notes/Rook.md", "Old text");
        assert!(refused.is_err() && !to.exists(), "fd5-changed");
        // The note moved away: nothing to rename.
        assert!(
            FileRename::apply(&campaign, "Notes/Gone.md", "Notes/Rook.md", text).is_err(),
            "fd5-gone"
        );
        let rename =
            FileRename::apply(&campaign, "Notes/Broken.md", "Notes/Rook.md", text).unwrap();
        assert!(!from.exists(), "fd5-moved");
        assert_eq!(std::fs::read_to_string(&to).unwrap(), text, "fd5-bytes");
        assert_eq!(rename.message(), "The file is now Rook.md.");
        // Review repair 3: the Undo belongs to its campaign. Another campaign with a note at the
        // same relative path neither shows it nor moves anything.
        let other_dir = tempfile::tempdir().unwrap();
        let other = Campaign {
            id: Uuid::new_v4(),
            path: other_dir.path().into(),
            ..campaign.clone()
        };
        std::fs::create_dir_all(other_dir.path().join("Notes")).unwrap();
        std::fs::write(other_dir.path().join("Notes/Rook.md"), text).unwrap();
        assert!(
            !rename.shows_on(&other, "Notes/Rook.md")
                && rename.shows_on(&campaign, "Notes/Rook.md"),
            "review-3-owner"
        );
        assert!(rename.undo(&other).is_err(), "review-3-undo-refused");
        assert!(
            other_dir.path().join("Notes/Rook.md").exists()
                && !other_dir.path().join("Notes/Broken.md").exists(),
            "review-3-other-bytes"
        );
        // Undo refuses while a new file holds the old path, and keeps both.
        std::fs::write(&from, "A new note.").unwrap();
        assert!(rename.undo(&campaign).is_err(), "fd5-undo-target-exists");
        assert_eq!(std::fs::read_to_string(&from).unwrap(), "A new note.");
        assert_eq!(std::fs::read_to_string(&to).unwrap(), text);
        std::fs::remove_file(&from).unwrap();
        // Undo refuses when the renamed file changed since.
        std::fs::write(&to, format!("{text}\nAn outside edit.\n")).unwrap();
        assert!(rename.undo(&campaign).is_err(), "fd5-undo-changed");
        assert!(!from.exists());
        std::fs::write(&to, text).unwrap();
        rename.undo(&campaign).unwrap();
        assert_eq!(std::fs::read_to_string(&from).unwrap(), text, "fd5-undo");
        assert!(!to.exists(), "fd5-undo-moved");
    }

    #[test]
    fn iter8_optional_links_warn_only_when_set_and_unresolved() {
        for (value, expected) in [
            ("null", false),
            ("''", false),
            ("'  '", false),
            ("'[[Nowhere]]'", true),
        ] {
            for (key, warning) in [
                ("in", LinkProblem::PlaceNotFound),
                ("creature", LinkProblem::StatBlockNotFound),
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
    fn iter12_d_fd1_a_place_with_two_matches_needs_fixing() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let id = create_entry(&campaign, "Rook", "npc").unwrap();
        for folder in ["Locations", "Other"] {
            fs::create_dir_all(dir.path().join(folder)).unwrap();
            fs::write(
                dir.path().join(format!("{folder}/Watch House.md")),
                "# Watch House\n",
            )
            .unwrap();
        }
        let path = dir.path().join("NPCs/Rook.md");
        let file = fs::read_to_string(&path).unwrap();
        for (place, expected) in [
            ("[[Watch House]]", vec![LinkProblem::PlaceMatches(2)]),
            // `Always use this one` writes the chosen file's path. The warning goes.
            ("[[Locations/Watch House]]", vec![]),
            // Neighbour: a missing place still says not found, not matches.
            ("[[Nowhere]]", vec![LinkProblem::PlaceNotFound]),
        ] {
            fs::write(
                &path,
                file.replacen("---\n", &format!("---\nsplat_in: '{place}'\n"), 1),
            )
            .unwrap();
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(id).unwrap()];
            assert!(item.entry.flags.is_empty(), "fixture {place} reads");
            assert_eq!(broken_links(item, &index), expected, "FD1-{place}");
        }
        assert_eq!(
            LinkProblem::PlaceMatches(2).to_string(),
            "Place has 2 matches",
            "FD1-words"
        );

        // Round 29 FI2: one fault record per note, found once per index. The row's words, the
        // header's count and the count's menu read the same records. Rook keeps `[[Nowhere]]`;
        // a renamed note and a duplicate join it, and a plain note has none.
        let mara = create_entry(&campaign, "Mara", "npc").unwrap();
        fs::rename(
            dir.path().join("NPCs/Mara.md"),
            dir.path().join("NPCs/Vell.md"),
        )
        .unwrap();
        let gate = create_entry(&campaign, "Gate", "location").unwrap();
        let gate_path = {
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            dir.path()
                .join(&index.items[index.position(gate).unwrap()].entry.path)
        };
        fs::copy(&gate_path, gate_path.with_file_name("Gate copy.md")).unwrap();
        let plain = create_entry(&campaign, "Hob", "npc").unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let scans = crate::lookup::FAULT_SCANS.with(std::cell::Cell::get);
        let faults = index.note_faults();
        // By path: two files that claim one identity have no one position by id.
        let at_path = |path: &std::path::Path| {
            let relative = path.strip_prefix(dir.path()).unwrap().to_string_lossy();
            index
                .items
                .iter()
                .position(|item| item.entry.path == relative)
                .unwrap()
        };
        let fault_of = |id| faults.of_note(index.position(id).unwrap()).cloned();
        assert_eq!(
            fault_of(id),
            Some(NoteFault::Links(vec![LinkProblem::PlaceNotFound]))
        );
        assert_eq!(fault_of(mara), Some(NoteFault::Renamed(None)));
        assert_eq!(NoteFault::Renamed(None).words(), "File name differs");
        assert_eq!(
            NoteFault::Renamed(Some("Vell.md".into())).words(),
            "File name differs · Vell.md"
        );
        assert_eq!(
            NoteFault::Renamed(Some("Vell.md".into()))
                .narrow_words()
                .as_deref(),
            Some("Vell.md")
        );
        assert_eq!(
            faults.of_note(at_path(&gate_path)).cloned(),
            Some(NoteFault::Duplicate)
        );
        assert_eq!(fault_of(plain), None);
        // The menu lists exactly the notes with a record, so the count is their number.
        let listed: Vec<_> = faults.listed().to_vec();
        assert!(listed.iter().all(|&at| faults.of_note(at).is_some()));
        for at in index.world_groups("").values().flatten().map(|(at, _)| *at) {
            assert_eq!(faults.of_note(at).is_some(), listed.contains(&at), "{at}");
        }
        // Asked again, as every frame asks: no second scan of the notes' bodies.
        let _ = index.note_faults();
        let _ = index.note_faults().listed().len();
        assert_eq!(
            crate::lookup::FAULT_SCANS.with(std::cell::Cell::get),
            scans + 1
        );
    }

    #[test]
    fn iter13_d_sd2_mixed_link_problems_need_fixing_and_missing_ones_are_broken() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let id = create_entry(&campaign, "Rook", "npc").unwrap();
        for folder in ["Locations", "Other"] {
            fs::create_dir_all(dir.path().join(folder)).unwrap();
            fs::write(
                dir.path().join(format!("{folder}/Watch House.md")),
                "# Watch House\n",
            )
            .unwrap();
        }
        // Round 23 FG2: a place whose `In` leads back to Rook makes a loop.
        let loop_two = create_entry(&campaign, "Loop Two", "location").unwrap();
        let loop_path = {
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            dir.path()
                .join(&index.items[index.position(loop_two).unwrap()].entry.path)
        };
        let loop_file = fs::read_to_string(&loop_path).unwrap();
        fs::write(
            &loop_path,
            loop_file.replacen("---\n", "---\nsplat_in: '[[Rook]]'\n", 1),
        )
        .unwrap();
        let path = dir.path().join("NPCs/Rook.md");
        let file = fs::read_to_string(&path).unwrap();
        for (fields, expected, words) in [
            // Round 23 FG2: a note in a place loop names the loop on its row.
            (
                "splat_in: '[[Loop Two]]'\n",
                vec![LinkProblem::PlaceLoop],
                "Place loop",
            ),
            // A loop and a missing group: the loop is not a broken link.
            (
                "splat_in: '[[Loop Two]]'\nsplat_part_of: ['[[Nowhere]]']\n",
                vec![LinkProblem::PlaceLoop, LinkProblem::GroupNotFound],
                "2 links need fixing",
            ),
            // Lookalike: a place that does not lead back is no loop.
            ("splat_in: '[[Locations/Watch House]]'\n", vec![], ""),
            // A place with two matches and a missing group: one waits for a choice.
            (
                "splat_in: '[[Watch House]]'\nsplat_part_of: ['[[Nowhere]]']\n",
                vec![LinkProblem::PlaceMatches(2), LinkProblem::GroupNotFound],
                "2 links need fixing",
            ),
            // Two missing targets are broken links.
            (
                "splat_in: '[[Nowhere]]'\nsplat_part_of: ['[[Elsewhere]]']\n",
                vec![LinkProblem::PlaceNotFound, LinkProblem::GroupNotFound],
                "2 broken links",
            ),
            // Neighbour: a single place with two matches names itself.
            (
                "splat_in: '[[Watch House]]'\n",
                vec![LinkProblem::PlaceMatches(2)],
                "Place has 2 matches",
            ),
            // Neighbour: a single missing group names itself.
            (
                "splat_part_of: ['[[Nowhere]]']\n",
                vec![LinkProblem::GroupNotFound],
                "Group not found",
            ),
        ] {
            fs::write(&path, file.replacen("---\n", &format!("---\n{fields}"), 1)).unwrap();
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(id).unwrap()];
            assert!(item.entry.flags.is_empty(), "fixture {fields} reads");
            let problems = broken_links(item, &index);
            assert_eq!(problems, expected, "SD2-problems-{fields}");
            if !words.is_empty() {
                assert_eq!(link_problem_words(&problems), words, "SD2-words-{fields}");
            }
            // The other end of the loop names the loop too, and only while Rook closes it.
            let other = &index.items[index.position(loop_two).unwrap()];
            assert_eq!(
                broken_links(other, &index).contains(&LinkProblem::PlaceLoop),
                fields.contains("Loop Two"),
                "FG2-other-end-{fields}"
            );
        }
    }

    #[test]
    fn iter11_d_fd1_each_missing_group_is_a_broken_link() {
        for (groups, expected) in [
            ("['[[Nowhere]]']", 1),
            ("['[[Nowhere]]', '[[Elsewhere|the else]]']", 2),
            ("['[[Arena Guild]]', '[[Nowhere]]']", 1),
            ("['[[Factions/Arena Guild|the arena]]']", 0),
            ("['[[Arena Guild]]']", 0),
            ("[]", 0),
            ("['']", 0),
            ("['  ']", 0),
            // A key with no value reads as an empty list.
            ("", 0),
            ("# No groups yet.", 0),
        ] {
            let dir = tempfile::tempdir().unwrap();
            let campaign = sample_campaign(dir.path());
            let id = create_entry(&campaign, "Rook", "npc").unwrap();
            create_entry(&campaign, "Arena Guild", "faction").unwrap();
            let path = dir.path().join("NPCs/Rook.md");
            let file = fs::read_to_string(&path).unwrap().replacen(
                "---\n",
                &format!("---\nsplat_part_of: {groups}\n"),
                1,
            );
            fs::write(path, file).unwrap();
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(id).unwrap()];
            assert!(item.entry.flags.is_empty(), "fixture {groups} reads");
            let problems = broken_links(item, &index);
            assert_eq!(
                problems
                    .iter()
                    .filter(|problem| **problem == LinkProblem::GroupNotFound)
                    .count(),
                expected,
                "FD1-{groups}"
            );
            // Neighbour: Part of adds no In or Stat block warning.
            assert_eq!(problems.len(), expected, "FD1-only-groups-{groups}");
        }
    }

    #[test]
    fn r9_status_shows_in_the_status_row_or_on_the_kind_line_never_both() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let id = create_entry(&campaign, "Rook", "npc").unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(id).unwrap()];
        assert_eq!(
            status(&item.entry),
            "alive",
            "R9-fixture: a new NPC is alive"
        );
        // A campaign NPC in a reader that may change it: the Status row shows the status.
        let place = ReaderStatus::of(item, &index, true, false);
        assert_eq!(place, ReaderStatus::Row, "R9-row");
        assert_eq!(
            place.kind_words(item, false),
            "NPC",
            "R9-row: no status on the kind line"
        );
        // The same entry where no Status row is drawn: the kind line keeps its status.
        let place = ReaderStatus::of(item, &index, false, false);
        assert_eq!(place, ReaderStatus::KindLine, "R9-kind-line");
        assert_eq!(
            place.kind_words(item, false),
            "NPC · alive",
            "R9-kind-line: the status stays"
        );
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
                // World's reader and Lookup's compact one show the same rows (`value_rows`).
                let reader = Reader {
                    compact: true,
                    sides: Some(Vec::new()),
                    ..Default::default()
                };
                let rows = reader.value_rows(item, &index);
                // The Status row, the block's last row: `Add to fight` is on the title line
                // (round 28 FJ7), so it no longer stands for the block.
                assert!(
                    rows.last().is_some_and(|row| row.label == "Status"),
                    "D2-compact-fixture: compact reader rows must show"
                );
                actual.push((
                    key,
                    value,
                    rows.iter()
                        .any(|row| matches!(row.value, model::ValueShown::Missing(_))),
                ));
                expected.push((key, value, warning));
            }
        }
        assert_eq!(
            actual, expected,
            "D2-compact: empty optional links have no warning; unresolved links keep it"
        );
    }

    /// Round 30 FH3, SH2, AH3, FB14, FE15: World's reader and Lookup's show the same value rows
    /// with the same Change for one entry, whatever the value's state: set, not found, unset,
    /// or a Part of list that was not read.
    #[test]
    fn s30_both_readers_show_the_same_value_rows_and_change() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        create_entry(&campaign, "Gate", "location").unwrap();
        create_entry(&campaign, "Ash Guild", "faction").unwrap();
        let rows = |head: &str, unread: bool| {
            let id = create_entry(&campaign, "Rook", "npc").unwrap();
            let path = dir.path().join("NPCs/Rook.md");
            let file =
                fs::read_to_string(&path)
                    .unwrap()
                    .replacen("---\n", &format!("---\n{head}"), 1);
            fs::write(&path, file).unwrap();
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(id).unwrap()];
            let drawn = [false, true].map(|compact| {
                let reader = Reader {
                    compact,
                    sides: Some(Vec::new()),
                    group_fault: unread.then(|| GroupFault {
                        id,
                        generation: 0,
                        unread: Some(fields::Unread {
                            line: 9,
                            reason: "Line 9: Part of was not read.".into(),
                            whole: "Line 9: Part of was not read.".into(),
                        }),
                        notices: Vec::new(),
                        entry: None,
                    }),
                    ..Default::default()
                };
                let mut changes: Vec<String> = reader
                    .value_rows(item, &index)
                    .iter()
                    .map(|row| format!("Change {}", row.label))
                    .collect();
                changes.sort();
                changes
            });
            fs::remove_file(&path).unwrap();
            drawn
        };
        for (head, unread, wanted) in [
            // A set place and group, and an NPC's unset stat block.
            (
                "splat_in: '[[Gate]]'\nsplat_part_of: ['[[Ash Guild]]']\n",
                false,
                &[
                    "Change Part of",
                    "Change Place",
                    "Change Stat block",
                    "Change Status",
                ][..],
            ),
            // A stat block and a place that are not found.
            (
                "splat_in: '[[Nowhere]]'\nsplat_stat_block: '[[Nobody]]'\n",
                false,
                &["Change Place", "Change Stat block", "Change Status"][..],
            ),
            // A Part of list that was not read keeps its row and its blocked Change.
            (
                "",
                true,
                &["Change Part of", "Change Stat block", "Change Status"][..],
            ),
        ] {
            let [page, lookup] = rows(head, unread);
            assert_eq!(page, wanted, "S30-rows-fixture: World's rows for {head:?}");
            assert_eq!(
                lookup, page,
                "S30-rows: Lookup's rows for {head:?} are World's"
            );
        }
    }

    #[test]
    fn i19_h1_world_notice_names_the_change_and_undo() {
        // A place change's Undo names its verb and its object, as the result's words do.
        assert_eq!(
            ChangeKind::Place.undo_step("Rook", "Rook.md").label(),
            "Undo change to Place"
        );
        assert_eq!(
            ChangeKind::PartOf.undo_step("Rook", "Rook.md").label(),
            "Undo change to Part of"
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
        assert_eq!(undo.message(), "Rook copy.md is now a separate note.");
        // Round 29 FG5: the Undo names the copy, as the encounter editor's does.
        assert_eq!(
            undo.undo_step("Rook copy").label(),
            "Undo separate Rook copy.md"
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
            "---\ntimeline_version: 2\n---\n",
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
        note(root, "Entry.md", "---\nsplat_schema: 2\n---\n");
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
        note(dir.path(), "Timeline.md", "---\ntimeline_version: 2\n---\n");
        note(
            dir.path(),
            "Trackers.md",
            "---\ntrackers_version: 1\nchanges: []\n---\n",
        );
        let scan = scan(dir.path(), dir.path());
        assert_eq!(scan.notes.len(), 1);
        assert_eq!(scan.notes[0].path, "Setting.md");
        // The scan and the ledger's reader use one rule: a ledger with a quoted key is one.
        let quoted = "---\n\"trackers_version\": 1\nchanges: []\n---\n";
        assert!(splat_campaign::trackers::ledger::Ledger::parse(quoted).is_ok());
        note(dir.path(), "Trackers.md", quoted);
        assert_eq!(super::scan(dir.path(), dir.path()).notes.len(), 1);
        // A note of the DM's named Trackers.md that is no ledger stays a world note.
        note(
            dir.path(),
            "Trackers.md",
            "---\ntags: [race]\n---\n\n# Trackers\n",
        );
        let scan = super::scan(dir.path(), dir.path());
        assert_eq!(scan.notes.len(), 2);
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

    /// AD2: the card's `Will save` path is the path Save NPC writes, for a campaign with no
    /// NPCs, NPCs in another folder, and NPCs at the campaign's root.
    #[test]
    fn iter15_d_ad2_the_card_promises_the_path_the_save_writes() {
        let npc_at = |path: &str, name: &str| {
            format!(
                "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: world\nsplat_name: {name}\nsplat_type: npc\n---\n\n# {name}\n",
                splat::character::stable_id(path)
            )
        };
        for (existing, expected) in [
            (vec![], "NPCs/Wren Hallam.md"),
            (
                vec!["People/Rook.md", "People/Tally.md", "NPCs/Blot.md"],
                "People/Wren Hallam.md",
            ),
            (vec!["Rook.md"], "Wren Hallam.md"),
        ] {
            let dir = tempfile::tempdir().unwrap();
            let campaign = sample_campaign(dir.path());
            for path in &existing {
                let file = dir.path().join(path);
                fs::create_dir_all(file.parent().unwrap()).unwrap();
                let name = Path::new(path).file_stem().unwrap().to_string_lossy();
                fs::write(file, npc_at(path, &name)).unwrap();
            }
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let promised = new_entry_path(Some(&index), "Wren Hallam", "npc");
            assert_eq!(promised, expected, "AD2-promise: {existing:?}");
            let npc = crate::npc_generator::Generated {
                name: "Wren Hallam".into(),
                attributes: ["Ink".into(), "Bored".into(), "A writ".into()],
            };
            create_generated(&campaign, &npc).unwrap();
            assert!(
                dir.path().join(&promised).is_file(),
                "AD2-write: the save wrote the promised {promised} for {existing:?}"
            );
        }
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
            campaign: campaign.id,
            path: path.into(),
            before: before.clone(),
            after: after.clone(),
            word: "Place changed.".into(),
            kind: ChangeKind::Place,
        };
        // Lead ruling, round 24 review: a change belongs to its campaign. Another campaign with a
        // note at the same path neither shows it nor has it undone, and keeps its bytes.
        let other_dir = tempfile::tempdir().unwrap();
        let other = sample_campaign(other_dir.path());
        fs::create_dir_all(other_dir.path().join("NPCs")).unwrap();
        fs::write(other_dir.path().join(path), &after).unwrap();
        assert!(
            !change.shows_on(&other, path) && change.shows_on(&campaign, path),
            "owner-shows-on"
        );
        assert!(change.undo(&other).is_err(), "owner-undo-refused");
        assert_eq!(
            fs::read_to_string(other_dir.path().join(path)).unwrap(),
            after,
            "owner-other-bytes"
        );
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
            played: Some("[[Session 1]]".into()),
            day: Some(2),
            ..Default::default()
        };
        let mut change = |key: &str, value: Value| {
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(hob).unwrap()];
            assert!(reader.change(&campaign, item, &index, key, value).is_some());
            reader.last_change().unwrap()
        };
        let status = change("status", "dead".into());
        let place = change("in", serde_yaml_ng::to_value(EntryRef::id(arena)).unwrap());
        let group = change("part_of", Value::Sequence(vec!["[[Harvest Cart]]".into()]));
        let text = fs::read_to_string(&path).unwrap();
        assert!(
            text.ends_with(
                "## Log\n\n- [[Session 1]], Day 2: Status alive → dead.\n- [[Session 1]], Day 2: Place not set → [[The Arena]].\n- [[Session 1]], Day 2: Part of not set → [[Harvest Cart]].\n"
            ),
            "{text}"
        );
        // Round 26 FE3: a result is a sentence; only the Log keeps the arrow form.
        assert_eq!(status.word, "Status set to dead. It was alive.");
        assert_eq!(place.word, "Place set to The Arena.");
        assert_eq!(group.word, "Part of set to Harvest Cart.");
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
            (arena, "Place not set → The Arena."),
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

        // F6: `Add to Log` writes the DM's words as one line with the stamp a change has at
        // that moment, and the session lists it. Its Undo takes back that line alone.
        let before_log = fs::read_to_string(&path).unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        assert!(
            reader
                .add_log_line(&campaign, item, "Gave the gate key to Iselle.")
                .is_some()
        );
        let logged = fs::read_to_string(&path).unwrap();
        assert_eq!(
            logged,
            format!("{before_log}- [[Session 1]], Day 2: Gave the gate key to Iselle.\n")
        );
        let line = reader.last_change().unwrap();
        assert_eq!(line.word, "Added to Hob’s Log.");
        assert_eq!(line.undo_step("Hob").label(), "Undo add Log line");
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        assert_eq!(
            index
                .world_changes(&dir.path().join("Sessions/Session 1.md"))
                .len(),
            4
        );
        line.undo(&campaign).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), before_log);

        // Undo takes back the field and its Log line together.
        group.undo(&campaign).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(!text.contains("splat_part_of") && !text.contains("Harvest Cart"));
        assert!(text.ends_with("Place not set → [[The Arena]].\n"));

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
        reader.played = None;
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        reader.change(&campaign, item, &index, "status", "dead".into());
        assert!(!fs::read_to_string(&path).unwrap().contains("## Log"));
        // With no session, `Add to Log` still writes the line the DM asked for, with the day
        // alone; with no day either, the words alone.
        for (day, line) in [(Some(2), "- Day 2: Rode north."), (None, "- Rode north.")] {
            reader.day = day;
            let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
            let item = &index.items[index.position(hob).unwrap()];
            let before = fs::read_to_string(&path).unwrap();
            reader.add_log_line(&campaign, item, "Rode north.");
            let after = fs::read_to_string(&path).unwrap();
            assert!(after.ends_with(&format!("## Log\n\n{line}\n")), "{after}");
            fs::write(&path, &before).unwrap();
        }
    }

    /// Layout PR 9b-R1c: a change's result reads its file on disk only when the index's load
    /// is new (the app loads the index again when a file changes), never on a quiet frame; an
    /// outside edit then ends the result at that load.
    #[test]
    fn a_change_result_reads_its_file_only_at_a_new_index_load() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let hob = create_entry(&campaign, "Hob", "npc").unwrap();
        let path = dir.path().join("NPCs/Hob.md");
        let load = |campaign: &Campaign| {
            std::sync::Arc::new(ContentIndex::load(&[], Some(campaign)).unwrap())
        };
        let stands =
            |reader: &Reader| matches!(reader.result(Slot::Change), Some(ReaderResult::Change(_)));
        // Each frame of the reader checks the disk (`check_disk`, the GPUI pane's call).
        let mut reader = Reader::default();
        let mut index = load(&campaign);
        let frames = |reader: &mut Reader, index: &ContentIndex, count: usize| {
            for _ in 0..count {
                reader.check_disk(index, Some(&campaign));
            }
        };
        frames(&mut reader, &index, 3);
        // The DM changes the status while the reader shows the entry; the app then loads the
        // index again for the written file.
        {
            let at = index.position(hob).unwrap();
            let _ = reader.change(&campaign, &index.items[at], &index, "status", "away".into());
            assert!(stands(&reader), "9b-R1c-setup: the change's result stands");
        }
        index = load(&campaign);
        frames(&mut reader, &index, 3);
        let reads = || READER_FILE_READS.with(std::cell::Cell::get);
        let before = reads();
        frames(&mut reader, &index, 5);
        assert!(
            reads() == before && stands(&reader),
            "9b-R1c-quiet: quiet frames read no file ({} reads), and the result stands",
            reads() - before
        );
        // An outside edit: nothing reads it until the app loads the index again.
        let words = fs::read_to_string(&path).unwrap();
        fs::write(&path, format!("{words}\nOutside words.\n")).unwrap();
        frames(&mut reader, &index, 2);
        assert!(
            reads() == before && stands(&reader),
            "9b-R1c-no-load: before the index's load, no read and the result stands"
        );
        index = load(&sample_campaign(dir.path()));
        frames(&mut reader, &index, 2);
        assert!(
            reads() == before + 1 && !stands(&reader),
            "9b-R1c-load: at the new load, one read ({}), and the result ends",
            reads() - before
        );
    }

    #[test]
    fn a_log_line_follows_the_disk_and_a_failed_write_keeps_the_words() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let campaign = sample_campaign(dir.path());
        let hob = create_entry(&campaign, "Hob", "npc").unwrap();
        let oren = create_entry(&campaign, "Oren", "npc").unwrap();
        let path = dir.path().join("NPCs/Hob.md");
        let reader = || Reader {
            played: Some("[[Session 1]]".into()),
            ..Default::default()
        };
        let mut lookup = reader();
        let key = (campaign.id, hob);
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        lookup.open_log(campaign.id, hob, "Hob", "the ropes");
        lookup.held.log_fields.get_mut(&key).unwrap().field.words =
            "Told who burned the ropes.".into();

        // An edit in Obsidian a moment before, while the field holds typed words: the line
        // goes after it, in the entry's new Log heading.
        let edited = format!("{}Seen at the ford.\n", fs::read_to_string(&path).unwrap());
        fs::write(&path, &edited).unwrap();
        assert!(
            lookup
                .add_log_line(&campaign, item, "Told who burned the ropes.")
                .is_some()
        );
        let a = fs::read_to_string(&path).unwrap();
        assert_eq!(
            a,
            format!("{edited}\n## Log\n\n- [[Session 1]]: Told who burned the ropes.\n")
        );
        assert!(
            !lookup.held.log_fields.contains_key(&key),
            "Enter closes the field"
        );

        // Act A in Lookup, an unrelated act B in the World page's reader, then Undo of A: A's
        // line goes, B's stays.
        let mut world = reader();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let other = &index.items[index.position(oren).unwrap()];
        let oren_path = dir.path().join("NPCs/Oren.md");
        let oren_before = fs::read_to_string(&oren_path).unwrap();
        world
            .add_log_line(&campaign, other, "Paid the toll.")
            .unwrap();
        let oren_after = fs::read_to_string(&oren_path).unwrap();
        assert_ne!(oren_before, oren_after);
        assert!(lookup.last_change().unwrap().undo(&campaign).is_ok());
        assert_eq!(fs::read_to_string(&path).unwrap(), edited);
        assert_eq!(fs::read_to_string(&oren_path).unwrap(), oren_after);

        // A failed write names the file, writes nothing, and keeps the field with its words.
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        lookup.open_log(campaign.id, hob, "Hob", "");
        lookup.held.log_fields.get_mut(&key).unwrap().field.words = "Kept.".into();
        let folder = dir.path().join("NPCs");
        fs::set_permissions(&folder, fs::Permissions::from_mode(0o555)).unwrap();
        let failed = lookup.add_log_line(&campaign, item, "Kept.");
        fs::set_permissions(&folder, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(failed.is_none());
        assert!(
            lookup
                .error
                .as_deref()
                .is_some_and(|error| error.contains("NPCs/Hob.md"))
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), edited);
        assert_eq!(lookup.held.log_fields[&key].field.words, "Kept.");

        // Round 29 FC10, ruling Q-t: a Log field of Hob open in BOTH readers, and an edit form
        // of Hob in one of them, with its save row on screen: each pass the shell shares the
        // readers' state. The one save row says the cause; the other reader's footer does not
        // repeat it. Then the same with the readers exchanged.
        world.open_log(campaign.id, hob, "Hob", "");
        lookup.compact = true;
        let at = index.position(hob).unwrap();
        let drawn =
            |mut world: Reader, mut lookup: Reader, form_in_world: bool| -> (Reader, Reader) {
                {
                    let form = if form_in_world {
                        &mut world
                    } else {
                        &mut lookup
                    };
                    form.begin_edit(&campaign, &index.items[at]);
                }
                let mut shares = 0;
                for pass in 1..=6 {
                    // The form's save row is on screen this pass (the view's record of it).
                    let form = if form_in_world {
                        &mut world
                    } else {
                        &mut lookup
                    };
                    form.form_drawn = Some((hob, pass));
                    lookup.share_with(&mut world, pass);
                    world.share_with(&mut lookup, pass);
                    // Review of f29ef2c0: unchanged passes copy no Log keys again; a field that
                    // opens or closes in one reader is the only thing that makes the other copy
                    // them.
                    if pass == 3 {
                        shares = LOG_KEY_SHARES.with(std::cell::Cell::get);
                    }
                }
                assert_eq!(
                    LOG_KEY_SHARES.with(std::cell::Cell::get),
                    shares,
                    "no copy of the Log keys on an unchanged pass"
                );
                let (form, other) = if form_in_world {
                    (&mut world, &mut lookup)
                } else {
                    (&mut lookup, &mut world)
                };
                assert!(
                    form.save_row_names_log(key) && !form.footer_names_log(key),
                    "one save row says the cause (form in World: {form_in_world})"
                );
                assert!(
                    !other.footer_names_log(key),
                    "the other reader's footer is quiet while the form's save row is on screen"
                );
                // The form's save row is no longer drawn (its page is hidden): the other reader's
                // footer says it, once.
                form.form_drawn = None;
                form.share_with(other, u64::MAX);
                assert!(other.footer_names_log(key));
                form.edits.clear();
                (world, lookup)
            };
        let (world, lookup) = drawn(world, lookup, true);
        let (mut world, mut lookup) = drawn(world, lookup, false);
        // A Log field in Lookup only, the form in World: World's save row says it too.
        world.held.log_fields.clear();
        world.begin_edit(&campaign, item);
        lookup.share_with(&mut world, 0);
        assert!(world.save_row_names_log(key));
        world.edits.clear();

        // A note that cannot be read, and a note renamed or deleted under the open field: the
        // act names the file and writes nothing.
        fs::write(&path, [0xff, 0xfe, b'\n']).unwrap();
        assert!(lookup.add_log_line(&campaign, item, "Kept.").is_none());
        assert!(
            lookup
                .error
                .as_deref()
                .is_some_and(|error| error.contains("NPCs/Hob.md"))
        );
        assert_eq!(fs::read(&path).unwrap(), [0xff, 0xfe, b'\n']);
        fs::remove_file(&path).unwrap();
        assert!(lookup.add_log_line(&campaign, item, "Kept.").is_none());
        assert!(
            lookup
                .error
                .as_deref()
                .is_some_and(|error| error.contains("NPCs/Hob.md"))
        );
        assert!(!path.exists());
        assert_eq!(lookup.held.log_fields[&key].field.words, "Kept.");

        // A done scene's words fill only an empty field: words the DM typed stay, and nothing
        // selects them. Clearing Lookup's search keeps them too.
        lookup.open_log(campaign.id, hob, "Hob", "the ropes");
        assert_eq!(lookup.held.log_fields[&key].field.words, "Kept.");
        lookup.clear();
        assert_eq!(lookup.held.log_fields[&key].field.words, "Kept.");
        // The app gives the reader the session being played again on each frame.
        lookup.played = Some("[[Session 1]]".into());
        // Each field keeps its own request to select its words.
        lookup.open_log(campaign.id, oren, "Oren", "paid the toll");
        assert!(!lookup.held.log_fields[&key].select);
        assert!(lookup.held.log_fields[&(campaign.id, oren)].select);

        // Typed Log words are unsaved work: the campaign switch's guard and the close warning
        // count them, by the entry's name. An empty field holds nothing to lose.
        assert!(lookup.has_unsaved_edits());
        assert!(
            lookup
                .unsaved_edits()
                .iter()
                .any(|(id, name)| *id == hob && name == "Hob")
        );
        let mut empty = reader();
        empty.open_log(campaign.id, oren, "Oren", "");
        assert!(!empty.has_unsaved_edits() && empty.unsaved_edits().is_empty());

        // A copied note in another campaign has the same ID. Its reader shows no field with
        // this campaign's words, so Enter there cannot write them into the copy.
        let other_dir = tempfile::tempdir().unwrap();
        let other = sample_campaign(other_dir.path());
        fs::create_dir_all(other_dir.path().join("NPCs")).unwrap();
        fs::write(other_dir.path().join("NPCs/Hob.md"), &edited).unwrap();
        let index = ContentIndex::load(&[], Some(&other)).unwrap();
        let copy = &index.items[index.position(hob).unwrap()];
        assert!(
            lookup.log_view(copy, &index, &other).is_none(),
            "another campaign's reader shows this campaign's typed words"
        );
        assert!(!lookup.held.log_fields.contains_key(&(other.id, hob)));

        // Every draft that the guards count has a field in view. A renamed played session
        // does not hide typed words: the stamp is the session at Enter, not the field's key.
        fs::write(&path, &edited).unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        let item = &index.items[index.position(hob).unwrap()];
        lookup.played = Some("[[Renamed Session]]".into());
        assert!(lookup.has_unsaved_edits());
        assert!(
            lookup
                .log_view(item, &index, &campaign)
                .is_some_and(|field| field.name == "Log line for Hob" && field.words == "Kept."),
            "the guarded words have a field after the session is renamed"
        );

        // Round 29 P-1 review: the same entry open in World's page and in Lookup, each with its
        // own Log draft. Typing, Add and Cancel change only the chosen draft.
        {
            let mut page = Reader {
                played: Some("[[Session 1]]".into()),
                ..Default::default()
            };
            let mut compact = Reader {
                played: Some("[[Session 1]]".into()),
                compact: true,
                ..Default::default()
            };
            page.open_log(campaign.id, hob, "Hob", "");
            compact.open_log(campaign.id, hob, "Hob", "");
            let words = |page: &Reader, compact: &Reader| {
                [page, compact].map(|reader| {
                    reader
                        .held
                        .log_fields
                        .get(&key)
                        .map(|open| open.field.words.clone())
                })
            };
            compact.set_log_words(campaign.id, hob, "Paid the ferryman.");
            assert_eq!(
                words(&page, &compact),
                [Some(String::new()), Some("Paid the ferryman.".into())],
                "typing changes Lookup's draft only"
            );
            page.set_log_words(campaign.id, hob, "Lost the map.");
            let _ = compact.log_enter(item, &campaign);
            let text = fs::read_to_string(&path).unwrap();
            assert!(
                text.contains("Paid the ferryman.") && !text.contains("Lost the map."),
                "Lookup's Add writes Lookup's words only: {text}"
            );
            assert_eq!(
                words(&page, &compact),
                [Some("Lost the map.".into()), None],
                "Add closes Lookup's draft only"
            );
            page.log_cancel(campaign.id, hob);
            assert_eq!(
                words(&page, &compact),
                [None, None],
                "Cancel closes the page's draft"
            );
            assert!(!fs::read_to_string(&path).unwrap().contains("Lost the map."));
        }

        // An entry that is gone: its draft waits for no guard, and the pane's footer says the
        // words, so nothing is lost without a word.
        let hob_text = fs::read_to_string(&path).unwrap();
        fs::remove_file(&path).unwrap();
        let index = ContentIndex::load(&[], Some(&campaign)).unwrap();
        assert!(index.position(hob).is_none());
        lookup.forget_gone_log_lines(&index);
        assert!(lookup.unsaved_edits().iter().all(|(id, _)| *id != hob));
        let item = &index.items[index.position(oren).unwrap()];
        let _ = item;
        let notices: Vec<_> = lookup
            .gone_logs(campaign.id)
            .map(edit::gone_words)
            .collect();
        assert!(
            notices.contains(
                &"\u{201c}Hob\u{201d} was deleted. The Log line was not added: \u{201c}Kept.\u{201d}"
                    .to_owned()
            ),
            "{notices:?}"
        );
        // X28 of app round 2: the notice stays until the DM copies or dismisses it. Lookup's
        // Clear, and another entry shown after it, keep it; its choice ends it.
        lookup.clear();
        let notices = |lookup: &mut Reader| lookup.gone_logs(campaign.id).count();
        assert_eq!(notices(&mut lookup), 1);
        assert_eq!(notices(&mut lookup), 1);
        // The entry comes back with its ID, the DM types again, and it goes again: both lost
        // drafts keep their notice (Astra's review of PR 287).
        fs::write(&path, &hob_text).unwrap();
        let again = ContentIndex::load(&[], Some(&campaign)).unwrap();
        lookup.forget_gone_log_lines(&again);
        lookup.open_log(campaign.id, hob, "Hob", "");
        // The words keep every byte the DM typed, for Copy text (review round 1 of PR 287).
        let again = "  \"Again.\"\nsecond line\n  ";
        lookup.held.log_fields.get_mut(&key).unwrap().field.words = again.into();
        fs::remove_file(&path).unwrap();
        lookup.forget_gone_log_lines(&ContentIndex::load(&[], Some(&campaign)).unwrap());
        assert_eq!(notices(&mut lookup), 2);
        let gone: Vec<_> = lookup.gone_logs(campaign.id).cloned().collect();
        assert!(gone.iter().any(|gone| gone.words == again), "{gone:?}");
        // A notice belongs to its campaign: another campaign's footer does not show it.
        let other = Campaign {
            id: Uuid::new_v4(),
            ..campaign.clone()
        };
        assert!(lookup.gone_logs(other.id).next().is_none());
        let first = gone.iter().find(|gone| gone.words == "Kept.").unwrap().id;
        // Its close icon ends it, and keeps its words for `Undo dismiss` (ruling R3).
        let _ = lookup.footer_press(
            design::outcome::Press {
                id: FooterAct::GoneDismiss(first),
                ticket: None,
            },
            None,
            &campaign,
        );
        assert_eq!(notices(&mut lookup), 1);
        assert!(
            !lookup.held.dismissed.is_empty(),
            "R3: the dismissed words keep an Undo"
        );
    }

    #[test]
    fn an_add_to_fight_refusal_belongs_to_its_campaign_and_run() {
        let (campaign, run, entry) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        let mut reader = Reader {
            run_owner: Some((campaign, run)),
            error: Some("An older action's error.".into()),
            ..Default::default()
        };
        reader.refuse(entry, "This encounter has ended.".into());
        assert_eq!(reader.refusal_for(entry), Some("This encounter has ended."));
        // Another entry has none; the refusal waits for its own.
        assert_eq!(reader.refusal_for(Uuid::new_v4()), None);
        // A successful retry ends it, and no other error.
        reader.added_to_fight(entry);
        assert_eq!(reader.refusal_for(entry), None, "retry");
        assert_eq!(reader.error.as_deref(), Some("An older action's error."));
        // Another campaign, or no run, ends it for good: coming back does not bring it back.
        for owner in [
            Some((Uuid::new_v4(), run)),
            Some((campaign, Uuid::new_v4())),
            None,
        ] {
            reader.run_owner = Some((campaign, run));
            reader.refuse(entry, "This encounter has ended.".into());
            reader.run_owner = owner;
            assert_eq!(reader.refusal_for(entry), None, "{owner:?}");
            reader.run_owner = Some((campaign, run));
            assert_eq!(reader.refusal_for(entry), None, "back {owner:?}");
        }
        assert_eq!(reader.error.as_deref(), Some("An older action's error."));
    }

    #[test]
    fn title_line_actions_keep_their_words_beside_or_under_the_title() {
        // Edit, Start fight and Add to Log with their words (round 27 FC4, FJ3, FM5, FF5).
        let widths = [60.0, 100.0, 108.0];
        let title = 150.0;
        let all = 60.0 + 100.0 + 108.0 + 2.0 * design::XS;
        let fits = title + design::L - design::S + all;
        // All of them fit after the title: they share its line.
        assert_eq!(title_line_rows(fits, fits, title, &widths), None);
        // One pixel short: all of them move together under the title, none goes away, in one
        // row while the line holds them.
        assert_eq!(
            title_line_rows(fits - 1.0, fits - 1.0, title, &widths),
            Some(vec![0, 0, 0])
        );
        // A line too narrow for all three takes as many as fit, in order, on more rows.
        let two = 60.0 + design::XS + 100.0;
        assert_eq!(
            title_line_rows(two, two, title, &widths),
            Some(vec![0, 0, 1])
        );
        assert_eq!(
            title_line_rows(two - 1.0, two - 1.0, title, &widths),
            Some(vec![0, 1, 2])
        );
        // A long title alone sends them under it, with room to spare under the title.
        assert_eq!(
            title_line_rows(600.0, 600.0, 600.0, &widths),
            Some(vec![0, 0, 0])
        );
        // No actions: nothing to place.
        assert_eq!(title_line_rows(10.0, 10.0, 600.0, &[]), None);
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
