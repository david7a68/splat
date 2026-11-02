//! The campaign's world: plain notes Splat reads and does not own, and the NPCs and locations
//! made from them. Obsidian stays the editor. A plain note becomes a Lookup item, not an entry,
//! so nothing depends on it. See `docs/design/world-and-house-rules.md`.
pub mod page;

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
        if path.extension().is_some_and(|e| e == "md")
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
        fields: Mapping::new(),
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

fn creature(entry: &Entry) -> Option<Reference> {
    reference(world_field(entry, "creature")?)
}

/// A world entry's type, as a heading and a list say it.
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
    let name = name.trim();
    let stem = sessions::file_safe(name);
    if stem.is_empty() {
        return Err("Type a name first.".to_owned());
    }
    let folder = if kind == "location" {
        "Locations"
    } else {
        "NPCs"
    };
    let relative = format!("{folder}/{stem}.md");
    let path = note_path(&campaign.path, &relative)?;
    if path.exists() {
        return Err(format!("{relative} already exists."));
    }
    let id = crate::hermetic::new_uuid();
    let text = format!("{}\n# {name}\n", frontmatter(id, name, kind)?);
    fs::create_dir_all(campaign.path.join(folder)).map_err(|e| e.to_string())?;
    write_note(&campaign.path, &relative, None, &text)?;
    Ok(id)
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
    let current = Entry::parse(&entry.path, &text)?;
    let mut world = current
        .fields
        .get("splat_world")
        .and_then(Value::as_mapping)
        .cloned()
        .unwrap_or_default();
    world.insert(name.into(), value);
    let updated = patch_fields(&text, &[("splat_world", Some(Value::Mapping(world)))])?;
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
    Arc::new(RunGroup {
        id: npc.id,
        path: npc.path.clone(),
        name: npc.name.clone(),
        side: side.to_owned(),
        encounter: None,
        encounter_name: None,
        members: vec![GroupMember {
            entry: Arc::new(stat_block.clone()),
            count: 1,
            overrides,
        }],
        notes: String::new(),
        xp: 0,
        xp_unknown: Vec::new(),
        unavailable: 0,
        problem,
    })
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
    other_type: String,
    adding: Option<Adding>,
    error: Option<String>,
    /// A link that matches several files, waiting for the DM to choose.
    many: Vec<usize>,
    heading: Option<String>,
    /// What only the app can do, for it to take after this reader draws.
    pub join: Option<Join>,
    pub open_timeline: Option<String>,
    pub sides: RunSides,
}

impl Reader {
    /// Show this heading the next time the item is drawn.
    pub fn show_heading(&mut self, heading: Option<String>) {
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
        chosen.and_then(|position| self.settle(Go::Item(position, None)))
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
            self.showing = Some(item.entry.id);
            self.status = status(&item.entry).to_owned();
            self.other_type.clear();
            self.adding = None;
            self.error = None;
        }
        crate::markdown_view::want_heading(ui.ctx(), self.heading.take());
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
        use crate::library::presentation;
        let (title, line) = match item.entry.reading().ok().flatten() {
            Some(reading) => (
                reading.title,
                presentation::reading_subtitle(&item.entry, &item.subtitle),
            ),
            None => (
                presentation::display_name(&item.entry.name).to_owned(),
                item.subtitle.clone(),
            ),
        };
        style::title(ui, &title);
        style::subtitle(ui, &format!("{line} · {}", item.owner));
        if let Some(error) = &self.error {
            design::message(ui, Tone::Error, error);
        }
        if item.entry.kind == "world" {
            go = self.entry_controls(ui, item, index, campaign);
        }
        if item.is_note() {
            style::measure(ui, |ui| style::markdown(ui, &item.entry.body));
            if item.is_world_note() {
                go = go.or(self.make_entry_control(ui, item, campaign));
            }
        } else {
            let followed = view::show(
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
            );
            go = go.or(followed.map(|position| Go::Item(position, None)));
        }
        go = go.or(related(ui, item, index));
        if let Some(target) = crate::markdown_view::take_note_link(ui.ctx()) {
            go = go.or(Some(index.follow(&target, item)));
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
        design::quiet_section(ui, "Make world entry", |ui| {
            ui.label(
                Type::Small
                    .rich(ui, "Gives this note an identity. Its text does not change.")
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

    fn entry_controls(
        &mut self,
        ui: &mut egui::Ui,
        item: &Item,
        index: &ContentIndex,
        campaign: Option<&Campaign>,
    ) -> Option<Go> {
        let campaign = campaign.filter(|_| item.is_private())?;
        ui.add_space(design::XS);
        design::text_field(ui, "Status", &mut self.status, None, None);
        let wanted = self.status.trim().to_owned();
        if wanted != status(&item.entry)
            && Action::secondary("Save status")
                .density(Density::Read)
                .show(ui)
                .clicked()
        {
            match set_world_field(campaign, &item.entry, "status", wanted.into()) {
                Ok(()) => return Some(Go::Reload(item.entry.id)),
                Err(error) => self.error = Some(error),
            }
        }
        if world_type(&item.entry) != "npc" {
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
            let label = format!("{} ({})", entry.name, type_label(world_type(entry)));
            if design::link(ui, &label).clicked() {
                go = Some(Go::Item(position, None));
            }
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
                ui.label(
                    Type::Stat
                        .rich(ui, format!("Stat block: {}", stat_block.name))
                        .color(colors().text),
                );
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
        egui::ComboBox::from_id_salt("world_add_side")
            .selected_text(adding.side.clone())
            .icon(design::combo_icon)
            .show_ui(ui, |ui| {
                for side in sides {
                    ui.selectable_value(&mut adding.side, side.clone(), side);
                }
            });
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
        if Action::primary("Add")
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

    fn note(dir: &Path, path: &str, text: &str) {
        let path = dir.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
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
}
