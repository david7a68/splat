//! Where imported definitions live. A definition goes to the enabled package note that lists its
//! D&D Beyond ID. Otherwise it goes to the campaign package. Matching words only suggest an ID for
//! the DM to approve. A route stays until the DM approves a move, so a change of packages never
//! moves anything by itself. Rule data stays in the conversion list, keyed by the same ID.
use super::{
    EntitySnapshot,
    content::{ContentNote, item_identity, spell_identity},
    rules, stable_id,
};
use crate::content::{Entry, Reference};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError};
use std::{fs, time::SystemTime};
use uuid::Uuid;

pub const FOLDER: &str = "Campaign Package";
const MANIFEST: &str = "Campaign Package/Package.md";
const ROUTES: &str = "splat_import_routes";
/// Starts the key of a proposal to remove a campaign package note that no route knows.
const UNROUTED: &str = "unrouted:";
/// A package note's field that lists the D&D Beyond definitions it holds.
pub const DNDBEYOND_IDS: &str = "splat_dndbeyond_ids";
/// Shorter text is too common for matching words to mean the same definition.
const MIN_WORDS: usize = 20;
/// In the character's own species note, short text such as "You are Small." can still match.
const MIN_TRAIT_WORDS: usize = 5;
/// A package note's frontmatter fields that an imported definition can fill in.
const RULE_FIELDS: [&str; 3] = ["splat_rule", "splat_requires_mechanics", "splat_contracts"];
const FACT_FIELDS: [&str; 2] = ["splat_item", "splat_spell"];

/// One save at a time reads and replaces the manifest and shared notes. Characters refresh on
/// separate threads, and two saves that each add a route must not drop the other's.
pub(super) fn saving() -> MutexGuard<'static, ()> {
    static SAVING: Mutex<()> = Mutex::new(());
    SAVING.lock().unwrap_or_else(PoisonError::into_inner)
}

pub fn is_manifest(entry: &Entry) -> bool {
    entry.kind == "package" && entry.path == MANIFEST
}
pub fn in_folder(path: &str) -> bool {
    Path::new(path).starts_with(FOLDER)
}

/// A package that the campaign enables, and any limit on the entries it selects.
pub struct EnabledPackage {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
    pub only: Option<Vec<Reference>>,
}

#[derive(Clone)]
struct SharedEntry {
    id: Uuid,
    key: Option<String>,
    kind: String,
    name: String,
    path: PathBuf,
    words: String,
    dndbeyond_ids: Vec<String>,
}

struct Shared {
    package: EnabledPackage,
    entries: Arc<Vec<SharedEntry>>,
}

/// What an import can route to: the campaign, and the packages it enables.
#[derive(Default)]
pub struct Destinations {
    /// None means that the campaign's own manifest says.
    campaign: Option<Uuid>,
    packages: Vec<Shared>,
}

enum Found<'a> {
    None,
    One(&'a Shared, &'a SharedEntry),
    Several(Vec<String>),
}
impl<'a> Found<'a> {
    fn of(matches: Vec<(&'a Shared, &'a SharedEntry)>) -> Self {
        match matches.as_slice() {
            [] => Self::None,
            [(shared, entry)] => Self::One(shared, entry),
            several => Self::Several(several.iter().map(|(s, e)| label(s, e)).collect()),
        }
    }
}
fn label(shared: &Shared, entry: &SharedEntry) -> String {
    format!("{} ({})", entry.name, shared.package.name)
}

impl Destinations {
    pub fn new(campaign: Uuid, packages: Vec<EnabledPackage>) -> Self {
        let packages = packages
            .into_iter()
            .filter_map(|package| {
                // A package that cannot be read, or is not the one registered, takes no routes
                // and no writes. The catalogue reports it.
                let (manifest, entries) = indexed(&package.path).ok()?;
                (manifest == Some(package.id)).then_some(Shared { package, entries })
            })
            .collect();
        Self {
            campaign: Some(campaign),
            packages,
        }
    }
    /// The campaign that owns the campaign package's identities.
    fn owner(&self, campaign: &Path) -> Result<Uuid, String> {
        if let Some(id) = self.campaign {
            return Ok(id);
        }
        let (entries, _) = crate::content::read_directory(campaign)?;
        Ok(entries
            .iter()
            .find(|entry| entry.kind == "campaign")
            .map_or_else(
                || stable_id(&format!("splat:campaign_folder:{}", campaign.display())),
                |entry| entry.id,
            ))
    }
    /// Whether exactly one note that the campaign enables lists this D&D Beyond ID.
    pub fn lists(&self, identity: &str) -> bool {
        matches!(self.mapped(identity), Found::One(..))
    }
    fn package(&self, id: Uuid) -> Option<&Shared> {
        self.packages.iter().find(|shared| shared.package.id == id)
    }
    fn holds(&self, route: &Route) -> bool {
        route
            .package
            .and_then(|id| self.package(id))
            .is_some_and(|shared| {
                shared
                    .entries
                    .iter()
                    .any(|entry| entry.id == route.id && selected(&shared.package, entry))
            })
    }
    fn entries(&self) -> impl Iterator<Item = (&Shared, &SharedEntry)> {
        self.packages.iter().flat_map(|shared| {
            shared
                .entries
                .iter()
                .filter(|entry| selected(&shared.package, entry))
                .map(move |entry| (shared, entry))
        })
    }
    /// The notes that list this D&D Beyond ID.
    fn mapped(&self, identity: &str) -> Found<'_> {
        Found::of(
            self.entries()
                .filter(|(_, entry)| entry.dndbeyond_ids.iter().any(|id| id == identity))
                .collect(),
        )
    }
    /// Notes with the same words. This only suggests an ID: the DM approves it. A species trait
    /// looks first in the character's species note, because many species share a trait's words.
    fn suggested(&self, kind: &str, source: &SourceWords) -> Found<'_> {
        if kind != "rule" {
            return Found::of(
                self.entries()
                    .filter(|(_, e)| e.kind == kind && same_words(&e.words, &source.words))
                    .collect(),
            );
        }
        let traits = || {
            self.entries()
                .filter(|(_, e)| ["rule", "reference"].contains(&e.kind.as_str()))
        };
        let species: Vec<_> = traits()
            .filter(|(_, e)| e.name == source.species && holds_words(&e.words, &source.words))
            .collect();
        if !species.is_empty() {
            return Found::of(species);
        }
        Found::of(
            traits()
                .filter(|(_, e)| {
                    source.words.len() >= MIN_WORDS && holds_words(&e.words, &source.words)
                })
                .collect(),
        )
    }
}

fn selected(package: &EnabledPackage, entry: &SharedEntry) -> bool {
    package.only.as_ref().is_none_or(|only| {
        only.iter().any(|reference| match reference {
            Reference::Id(id) => *id == entry.id,
            Reference::Key(key) => entry.key.as_ref() == Some(key),
        })
    })
}

/// Reading a package takes a noticeable time, and most saves find it unchanged.
fn indexed(root: &Path) -> Result<(Option<Uuid>, Arc<Vec<SharedEntry>>), String> {
    type Indexed = (Option<Uuid>, Arc<Vec<SharedEntry>>);
    type Memo = Mutex<HashMap<PathBuf, ((usize, SystemTime), Indexed)>>;
    static MEMO: OnceLock<Memo> = OnceLock::new();
    let signature = signature(root)?;
    let memo = MEMO.get_or_init(Default::default);
    if let Some((seen, indexed)) = memo.lock().map_err(|e| e.to_string())?.get(root)
        && *seen == signature
    {
        return Ok(indexed.clone());
    }
    let (entries, _) = crate::content::read_directory(root)?;
    let manifests: Vec<_> = entries.iter().filter(|e| e.kind == "package").collect();
    let manifest = match manifests.as_slice() {
        [manifest] if !entries.iter().any(|e| e.kind == "campaign") => Some(manifest.id),
        _ => None,
    };
    let entries: Vec<_> = entries
        .into_iter()
        .filter(|entry| ["item", "spell", "rule", "reference"].contains(&entry.kind.as_str()))
        .map(|entry| SharedEntry {
            id: entry.id,
            words: normalized(source_section(&entry.body)),
            dndbeyond_ids: listed_ids(&entry),
            key: entry.key,
            kind: entry.kind,
            name: entry.name,
            path: root.join(entry.path),
        })
        .collect();
    let indexed = (manifest, Arc::new(entries));
    memo.lock()
        .map_err(|e| e.to_string())?
        .insert(root.to_owned(), (signature, indexed.clone()));
    Ok(indexed)
}
fn signature(root: &Path) -> Result<(usize, SystemTime), String> {
    fn walk(path: &Path, count: &mut usize, newest: &mut SystemTime) -> std::io::Result<()> {
        for item in fs::read_dir(path)? {
            let item = item?;
            let meta = item.metadata()?;
            if meta.is_dir() {
                walk(&item.path(), count, newest)?;
            } else {
                *count += 1;
                *newest = (*newest).max(meta.modified()?);
            }
        }
        Ok(())
    }
    let (mut count, mut newest) = (0, SystemTime::UNIX_EPOCH);
    walk(root, &mut count, &mut newest).map_err(|e| e.to_string())?;
    Ok((count, newest))
}
fn json_field(entry: &Entry, field: &str) -> Option<Value> {
    entry
        .fields
        .get(field)
        .and_then(|value| serde_json::to_value(value).ok())
}
fn source_section(body: &str) -> &str {
    let Some((_, source)) = body.split_once("## Source text") else {
        return body;
    };
    source.split("\n## ").next().unwrap_or(source)
}
fn listed_ids(entry: &Entry) -> Vec<String> {
    entry
        .fields
        .get(DNDBEYOND_IDS)
        .and_then(serde_yaml_ng::Value::as_sequence)
        .into_iter()
        .flatten()
        .filter_map(serde_yaml_ng::Value::as_str)
        .map(str::to_owned)
        .collect()
}
/// Package notes put a source line before the text that a provider definition holds.
fn same_words(package: &str, definition: &str) -> bool {
    definition.len() >= MIN_WORDS
        && package
            .strip_suffix(definition)
            .is_some_and(|before| before.is_empty() || before.ends_with(' '))
}
/// A package note can hold a definition among other text, such as a trait in a species note.
fn holds_words(package: &str, definition: &str) -> bool {
    definition.len() >= MIN_TRAIT_WORDS
        && format!(" {package} ").contains(&format!(" {definition} "))
}
fn normalized(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some((before, after)) = rest.split_once("<!--") {
        out.push_str(before);
        rest = after.split_once("-->").map_or("", |(_, tail)| tail);
    }
    out.push_str(rest);
    let out = out
        .replace('\u{2019}', "'")
        .replace(['\u{201c}', '\u{201d}'], "\"")
        .replace("\\\n", " ")
        .replace(['*', '_', '#', '`', '|'], "");
    out.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Where one source definition lives. No package means the campaign package.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Route {
    name: String,
    kind: String,
    id: Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    package: Option<Uuid>,
    /// The D&D Beyond IDs that a campaign package note holds. One note can serve several IDs,
    /// and each ID moves on its own.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    sources: Vec<String>,
    /// The sheet's link to a package note, kept for when the package is not available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    link: Option<String>,
}
struct Manifest {
    previous: Option<String>,
    routes: BTreeMap<String, Route>,
}
impl Manifest {
    fn read(campaign: &Path) -> Result<Self, String> {
        let path = campaign.join(MANIFEST);
        let previous = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self {
                    previous: None,
                    routes: BTreeMap::new(),
                });
            }
            Err(e) => return Err(e.to_string()),
        };
        let entry = Entry::parse(MANIFEST, &previous)?;
        let routes = match entry.fields.get(ROUTES) {
            Some(value) => serde_yaml_ng::from_value(value.clone())
                .map_err(|e| format!("Invalid campaign package routes: {e}"))?,
            None => BTreeMap::new(),
        };
        Ok(Self {
            previous: Some(previous),
            routes,
        })
    }
    fn text(&self, campaign: Uuid) -> Result<String, String> {
        let base = self.previous.clone().unwrap_or_else(|| {
            format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: package\nsplat_name: Campaign Package\nsplat_namespace: campaign_package_{}\n---\n\n# Campaign Package\n\nDefinitions from D&D Beyond that no enabled package note lists by ID. Splat writes this folder. Campaign > Packages lists the entries that can move to or from a package.\n",
                package_id(campaign),
                campaign.simple(),
            )
        });
        let routes = serde_yaml_ng::to_value(&self.routes).map_err(|e| e.to_string())?;
        let text = crate::content::formats::patch_model(&base, &[(ROUTES, Some(routes))])?;
        if self.previous.is_none() {
            crate::content::formats::convert(MANIFEST, &text)
        } else {
            Ok(text)
        }
    }
}
pub fn package_id(campaign: Uuid) -> Uuid {
    stable_id(&format!("splat:campaign_package:{campaign}"))
}
fn campaign_entry_id(campaign: Uuid, key: &str) -> Uuid {
    stable_id(&format!("splat:campaign_package:{campaign}:{key}"))
}
/// The same source gives the same key on every character, so characters share one definition.
fn source_key(entry: &Entry, local: &[String]) -> Result<String, String> {
    let mut fields = serde_json::to_value(&entry.fields).map_err(|e| e.to_string())?;
    fields["splat_id"] = Value::Null;
    let mut text = format!(
        "{}\n{}",
        serde_json::to_string(&fields).map_err(|e| e.to_string())?,
        entry.body
    );
    for id in local {
        text = text.replace(id.as_str(), "");
    }
    let hash = crate::content::contracts::fingerprint(&Value::String(text))?;
    Ok(hash[..32].to_owned())
}
fn folder_for(kind: &str) -> &'static str {
    match kind {
        "item" => "Items",
        "spell" => "Spells",
        _ => "Rules",
    }
}
fn note_path_for(kind: &str, name: &str, id: Uuid) -> String {
    let safe: String = name
        .chars()
        .filter(|c| !"/\\:*?\"<>|#^[]".contains(*c))
        .collect();
    let short = id.simple().to_string();
    format!(
        "{FOLDER}/{}/{} {}.md",
        folder_for(kind),
        safe.trim(),
        &short[..8]
    )
}
fn link_to(sheet_folder: &Path, target: &Path) -> String {
    let from: Vec<_> = sheet_folder.components().collect();
    let to: Vec<_> = target.components().collect();
    let shared = from.iter().zip(&to).take_while(|(a, b)| a == b).count();
    let mut parts = vec!["..".to_owned(); from.len() - shared];
    parts.extend(to[shared..].iter().map(|part| {
        part.as_os_str()
            .to_string_lossy()
            .replace('%', "%25")
            .replace(' ', "%20")
            .replace('(', "%28")
            .replace(')', "%29")
    }));
    parts.join("/")
}

/// A generated note with its identities replaced, and its coverage fingerprint brought up to date.
fn rewritten(
    relative_path: &str,
    markdown: &str,
    ids: &BTreeMap<String, String>,
    links: &BTreeMap<String, String>,
) -> Result<String, String> {
    let entry = Entry::parse(relative_path, markdown)?;
    let replace = |mut text: String| {
        for (from, to) in links {
            text = text.replace(from.as_str(), to);
        }
        for (from, to) in ids {
            text = text.replace(from.as_str(), to);
        }
        text
    };
    let fields =
        serde_json::to_string(&serde_json::to_value(&entry.fields).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let mut fields: Value = serde_json::from_str(&replace(fields)).map_err(|e| e.to_string())?;
    refresh_coverage(&mut fields)?;
    if entry
        .fields
        .get("splat_schema")
        .and_then(serde_yaml_ng::Value::as_u64)
        == Some(2)
        && crate::content::formats::eligible(&entry)
    {
        // Routing edits the model, then emits its one payload. Reusing the
        // previous fence here would create a second source for the same facts.
        fields["splat_schema"] = 1.into();
        let body = replace(
            entry
                .body
                .split("\n## Splat data")
                .next()
                .unwrap_or(&entry.body)
                .to_owned(),
        );
        let legacy = rules::rendered(fields, &body)?;
        return crate::content::formats::convert(relative_path, &legacy);
    }
    rules::rendered(fields, &replace(entry.body))
}
fn refresh_coverage(fields: &mut Value) -> Result<(), String> {
    if fields["splat_import"]["rule_fingerprint"].is_string() {
        let report = rules::coverage(fields)?;
        rules::put_coverage(fields, report)?;
    }
    Ok(())
}

/// A change to a shared note. The save checks `previous` again before it replaces the file.
pub(super) struct SharedWrite {
    pub path: PathBuf,
    pub previous: String,
    pub text: String,
}
pub(super) struct Routed {
    /// Dependencies, then the creature.
    pub notes: Vec<(PathBuf, String)>,
    pub shared: Vec<SharedWrite>,
    pub manifest: Option<(Option<String>, String)>,
}

/// Decides each definition's place. An existing route stands. A definition that is already in
/// this character's old folder goes to the campaign package, and the check offers its move.
pub(super) fn route(
    campaign: &Path,
    destinations: &Destinations,
    mut notes: Vec<ContentNote>,
) -> Result<Routed, String> {
    let creature = notes.pop().ok_or("Missing creature")?;
    let root = campaign.join("Characters").join("D&D Beyond");
    let owner = destinations.owner(campaign)?;
    let mut manifest = Manifest::read(campaign)?;
    let before = manifest.routes.clone();
    let parsed = notes
        .iter()
        .map(|note| Entry::parse(&note.relative_path, &note.markdown))
        .collect::<Result<Vec<_>, _>>()?;
    let local: Vec<String> = parsed.iter().map(|entry| entry.id.to_string()).collect();
    let (mut ids, mut links) = (BTreeMap::new(), BTreeMap::new());
    let mut placed = Vec::new();
    let mut shared = Vec::new();
    for (note, entry) in notes.iter().zip(&parsed) {
        let hash = source_key(entry, &local)?;
        let moved = note
            .sources
            .iter()
            .find(|source| manifest.routes.contains_key(*source))
            .cloned();
        let key = moved.unwrap_or_else(|| {
            if manifest.routes.contains_key(&hash) {
                return hash.clone();
            }
            let mapped = (!root.join(&note.relative_path).exists())
                .then(|| {
                    note.sources
                        .iter()
                        .find_map(|source| match destinations.mapped(source) {
                            Found::One(shared, found) => Some((source, shared, found)),
                            _ => None,
                        })
                })
                .flatten();
            let (key, route) = match mapped {
                Some((source, shared, found)) => (
                    source.clone(),
                    Route {
                        name: entry.name.clone(),
                        kind: entry.kind.clone(),
                        id: found.id,
                        package: Some(shared.package.id),
                        sources: Vec::new(),
                        link: None,
                    },
                ),
                None => (
                    hash.clone(),
                    Route {
                        name: entry.name.clone(),
                        kind: entry.kind.clone(),
                        id: campaign_entry_id(owner, &hash),
                        package: None,
                        sources: Vec::new(),
                        link: None,
                    },
                ),
            };
            manifest.routes.insert(key.clone(), route);
            key
        });
        let route = manifest.routes.get_mut(&key).ok_or("Missing route")?;
        if route.package.is_none() {
            for source in &note.sources {
                if !route.sources.contains(source) {
                    route.sources.push(source.clone());
                    route.sources.sort();
                }
            }
        }
        let route = route.clone();
        ids.insert(entry.id.to_string(), route.id.to_string());
        match route.package {
            None => {
                let relative = note_path_for(&entry.kind, &entry.name, route.id);
                let path = campaign.join(&relative);
                links.insert(note.relative_path.clone(), link_to(&root, &path));
                placed.push((note, path, relative));
            }
            Some(package) => {
                let found = destinations.package(package).and_then(|shared| {
                    let found = shared.entries.iter().find(|e| e.id == route.id)?;
                    Some((shared, found))
                });
                let link = if let Some((holder, found)) = found {
                    // A regenerated package loses what Splat added, so each save puts it back.
                    // A note the campaign no longer selects keeps its route, and takes no writes.
                    if selected(&holder.package, found)
                        && let Ok(Some(write)) =
                            package_write(&found.path, Some(entry), std::slice::from_ref(&key))
                    {
                        shared.push(write);
                    }
                    let link = link_to(&root, &found.path);
                    if let Some(route) = manifest.routes.get_mut(&key) {
                        route.link = Some(link.clone());
                    }
                    Some(link)
                } else {
                    route.link.clone()
                };
                if let Some(link) = link {
                    links.insert(note.relative_path.clone(), link);
                }
            }
        }
    }
    let mut output = Vec::new();
    for (note, path, relative) in placed {
        output.push((path, rewritten(&relative, &note.markdown, &ids, &links)?));
    }
    let sheet = rewritten(&creature.relative_path, &creature.markdown, &ids, &links)?;
    output.push((root.join(&creature.relative_path), sheet));
    let manifest = (manifest.routes != before || manifest.previous.is_none()).then(|| {
        manifest
            .text(owner)
            .map(|text| (manifest.previous.clone(), text))
    });
    Ok(Routed {
        notes: output,
        shared,
        manifest: manifest.transpose()?,
    })
}
/// Adds D&D Beyond IDs to a shared note. A note that holds one item or spell also takes what it
/// lacks from the definition. A note with other kinds of content, such as a species note with
/// many traits, takes only the IDs: rule data stays in the conversion list. Its own values and
/// words stay. None means nothing to add.
fn package_write(
    path: &Path,
    note: Option<&Entry>,
    ids: &[String],
) -> Result<Option<SharedWrite>, String> {
    if fs::metadata(path).is_ok_and(|meta| meta.permissions().readonly()) {
        return Err("This package file is read only.".into());
    }
    let previous = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let target = Entry::parse(&path.to_string_lossy(), &previous)?;
    let mut merged = serde_json::to_value(&target.fields).map_err(|e| e.to_string())?;
    let mut changed = Vec::new();
    let mut listed = listed_ids(&target);
    if ids.iter().any(|id| !listed.contains(id)) {
        listed.extend(ids.iter().cloned());
        listed.sort();
        listed.dedup();
        merged[DNDBEYOND_IDS] = json!(listed);
        changed.push(DNDBEYOND_IDS);
    }
    if let Some(note) = note.filter(|note| note.kind == target.kind) {
        let mut data = false;
        for field in RULE_FIELDS {
            match (json_field(note, field), json_field(&target, field)) {
                (Some(value), None) => {
                    merged[field] = value;
                    changed.push(field);
                    data = true;
                }
                (Some(value), Some(existing)) if value != existing => {
                    return Err(format!("{} already has different rule data.", target.name));
                }
                _ => {}
            }
        }
        for field in FACT_FIELDS {
            if let Some(value) = json_field(note, field) {
                let mut filled = merged[field].take();
                let added = fill(&mut filled, &value);
                merged[field] = filled;
                if added {
                    changed.push(field);
                    data = true;
                }
            }
        }
        if data && let Some(mut report) = json_field(note, "splat_import") {
            report["rule_fingerprint"] = json!("");
            merged["splat_import"] = report;
            refresh_coverage(&mut merged)?;
            changed.push("splat_import");
        }
    }
    if changed.is_empty() {
        return Ok(None);
    }
    let changes = changed
        .iter()
        .map(|field| {
            serde_yaml_ng::to_value(&merged[*field])
                .map(|value| (*field, Some(value)))
                .map_err(|e| e.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let text = crate::content::formats::patch_model(&previous, &changes)?;
    Entry::parse(&path.to_string_lossy(), &text).map_err(|e| format!("{}: {e}", target.name))?;
    Ok(Some(SharedWrite {
        path: path.to_owned(),
        previous,
        text,
    }))
}
fn fill(target: &mut Value, source: &Value) -> bool {
    match (&mut *target, source) {
        (Value::Null, _) => {
            *target = source.clone();
            !source.is_null()
        }
        (Value::Object(have), Value::Object(new)) => {
            let mut added = false;
            for (key, value) in new {
                added |= fill(have.entry(key.clone()).or_insert(Value::Null), value);
            }
            added
        }
        _ => false,
    }
}

/// Paths in the old per-character folder. Nothing is published there now.
pub(super) fn former_paths(root: &Path, record: &EntitySnapshot) -> Vec<String> {
    let folder = format!("Definitions/{}", record.definition.splat_id);
    let mut found = Vec::new();
    for sub in [folder.clone(), format!("{folder}/Rules")] {
        for item in fs::read_dir(root.join(&sub))
            .into_iter()
            .flatten()
            .flatten()
        {
            let name = item.file_name().to_string_lossy().into_owned();
            if name.ends_with(".md") {
                found.push(format!("{sub}/{name}"));
            }
        }
    }
    found
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Move {
    /// The package note named here lists this D&D Beyond ID, or has the same words.
    ToPackage(String),
    /// The package that held it is no longer enabled.
    ToCampaign,
    /// No current entry refers to it.
    Forget,
}
/// One change that the check found. A blocked one explains what the DM must decide first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proposal {
    pub key: String,
    pub name: String,
    pub kind: String,
    pub change: Move,
    pub blocked: Option<String>,
}

/// A D&D Beyond definition's words, and the species of a character who has it.
struct SourceWords {
    words: String,
    species: String,
}
fn source_words(records: &[EntitySnapshot]) -> BTreeMap<String, SourceWords> {
    let mut found = BTreeMap::new();
    for record in records {
        let mut add = |identity: String, definition: &Value| {
            let text = definition["description"]
                .as_str()
                .filter(|text| !text.is_empty())
                .or(definition["snippet"].as_str())
                .unwrap_or_default();
            found.entry(identity).or_insert_with(|| SourceWords {
                words: normalized(&super::description_text(text)),
                species: record.character.species.clone(),
            });
        };
        for feature in &record.character.features {
            let definition = feature.get("definition").unwrap_or(feature);
            let id = rules::source_id(&definition["id"]);
            if !id.is_empty() {
                add(format!("feature:{id}"), definition);
            }
        }
        for actions in record.source.raw["actions"]
            .as_object()
            .into_iter()
            .flat_map(|groups| groups.values())
        {
            for action in super::list(actions) {
                let id = rules::source_id(&action["id"]);
                if !id.is_empty() {
                    add(format!("action:{id}"), action);
                }
            }
        }
        for item in &record.inventory {
            if let Some(identity) = item_identity(&item.definition) {
                add(identity, &item.definition);
            }
        }
        for spell in &record.character.spells {
            let definition = &spell["definition"];
            add(spell_identity(definition), definition);
        }
    }
    found
}
/// The note a definition can move to: the one that lists its ID, or else a suggestion by words.
fn destination<'a>(
    destinations: &'a Destinations,
    identity: &str,
    kind: &str,
    words: &BTreeMap<String, SourceWords>,
) -> Found<'a> {
    match destinations.mapped(identity) {
        Found::None => words
            .get(identity)
            .map_or(Found::None, |source| destinations.suggested(kind, source)),
        found => found,
    }
}

/// Compares the routes with the enabled packages. It changes nothing.
pub fn check(
    campaign: &Path,
    destinations: &Destinations,
    records: &[EntitySnapshot],
) -> Result<Vec<Proposal>, String> {
    let manifest = Manifest::read(campaign)?;
    if manifest.previous.is_none() {
        return Ok(Vec::new());
    }
    let (entries, errors) = crate::content::read_directory(campaign)?;
    // A note that cannot be read may use anything, so nothing can count as unused.
    let unreadable = errors.first().map(|error| {
        format!(
            "{} cannot be read, so Splat cannot tell whether it uses this.",
            error.path
        )
    });
    let mut users: BTreeMap<Uuid, Vec<&Entry>> = BTreeMap::new();
    for entry in &entries {
        for reference in crate::content::required_references(entry)? {
            if let Reference::Id(id) = reference {
                users.entry(id).or_default().push(entry);
            }
        }
    }
    let cached: BTreeSet<Uuid> = records
        .iter()
        .map(|record| record.definition.splat_id)
        .collect();
    let words = source_words(records);
    let mut proposals = Vec::new();
    for (key, route) in &manifest.routes {
        let using = users.get(&route.id).map_or(&[][..], Vec::as_slice);
        let proposal = |key: &str, change: Move, reason: Option<String>| Proposal {
            key: key.to_owned(),
            name: route.name.clone(),
            kind: route.kind.clone(),
            change,
            blocked: reason.or_else(|| blocker(using, &cached)),
        };
        let local = (route.package.is_none())
            .then(|| entries.iter().find(|entry| entry.id == route.id))
            .flatten();
        if using.is_empty() {
            proposals.push(proposal(
                key,
                Move::Forget,
                local
                    .and_then(|entry| kept_note(campaign, entry))
                    .or_else(|| unreadable.clone()),
            ));
        } else if let Some(note) = local {
            let generated = generated_entry(campaign, note);
            for source in &route.sources {
                // The route under this key is the one being checked, not a move already made.
                if source != key && manifest.routes.contains_key(source) {
                    continue;
                }
                match destination(destinations, source, &route.kind, &words) {
                    Found::None => {}
                    Found::One(shared, found) => {
                        let reason = kept_note(campaign, note).or_else(|| {
                            package_write(&found.path, generated.as_ref(), std::slice::from_ref(source)).err()
                        });
                        proposals.push(proposal(
                            source,
                            Move::ToPackage(label(shared, found)),
                            reason,
                        ));
                    }
                    Found::Several(names) => proposals.push(proposal(
                        source,
                        Move::ToPackage(names.join(", ")),
                        Some(format!(
                            "More than one note fits. List {source} in {DNDBEYOND_IDS} on the right note only."
                        )),
                    )),
                }
            }
        } else if route.package.is_some() && !destinations.holds(route) {
            proposals.push(proposal(key, Move::ToCampaign, None));
        }
    }
    // A failed apply or an outside change can leave a note that no route knows.
    for note in entries.iter().filter(|entry| {
        in_folder(&entry.path)
            && !is_manifest(entry)
            && !users.contains_key(&entry.id)
            && !manifest.routes.values().any(|route| route.id == entry.id)
    }) {
        proposals.push(Proposal {
            key: format!("{UNROUTED}{}", note.id),
            name: note.name.clone(),
            kind: note.kind.clone(),
            change: Move::Forget,
            blocked: kept_note(campaign, note).or_else(|| unreadable.clone()),
        });
    }
    Ok(proposals)
}
fn blocker(using: &[&Entry], cached: &BTreeSet<Uuid>) -> Option<String> {
    using.iter().find_map(|entry| {
        if entry.kind != "creature" || !entry.fields.contains_key("splat_external") {
            Some(format!(
                "{} refers to it, and Splat cannot change that note.",
                entry.name
            ))
        } else if !cached.contains(&entry.id) {
            Some(format!(
                "{} has no saved D&D Beyond data. Read the sheet first.",
                entry.name
            ))
        } else {
            None
        }
    })
}
fn kept_note(campaign: &Path, note: &Entry) -> Option<String> {
    let text = fs::read_to_string(campaign.join(&note.path)).ok()?;
    let copy = super::campaign::parts(&text);
    if !copy.intact {
        Some("It has local edits.".into())
    } else if copy.notes != super::campaign::PERSONAL_NOTES {
        Some("It has personal notes.".into())
    } else {
        None
    }
}
fn generated_entry(campaign: &Path, note: &Entry) -> Option<Entry> {
    let text = fs::read_to_string(campaign.join(&note.path)).ok()?;
    Entry::parse(&note.path, super::campaign::parts(&text).generated).ok()
}

/// Carries out the chosen proposals, then saves each sheet again so that it follows the new
/// routes. Blocked proposals stay. A note that nothing uses any more goes to an archive. Nothing
/// is forgotten until the saves succeed, so a failed apply can run again.
/// Returns how many changes it made.
pub fn apply(
    campaign: &Path,
    destinations: &Destinations,
    records: &[EntitySnapshot],
    keys: &BTreeSet<String>,
) -> Result<usize, String> {
    let chosen: Vec<_> = check(campaign, destinations, records)?
        .into_iter()
        .filter(|proposal| proposal.blocked.is_none() && keys.contains(&proposal.key))
        .collect();
    if chosen.is_empty() {
        return Ok(0);
    }
    let owner = destinations.owner(campaign)?;
    let words = source_words(records);
    let mut replaced = Vec::new();
    {
        let _saving = saving();
        let mut manifest = Manifest::read(campaign)?;
        let (entries, _) = crate::content::read_directory(campaign)?;
        for proposal in &chosen {
            match &proposal.change {
                Move::ToPackage(_) => {
                    let holder = manifest
                        .routes
                        .values()
                        .find(|route| route.sources.contains(&proposal.key))
                        .cloned()
                        .ok_or_else(|| format!("Could not find {}.", proposal.name))?;
                    let note = entries
                        .iter()
                        .find(|entry| entry.id == holder.id)
                        .and_then(|entry| generated_entry(campaign, entry));
                    let Found::One(shared, found) =
                        destination(destinations, &proposal.key, &holder.kind, &words)
                    else {
                        return Err(format!(
                            "{} no longer has one matching note.",
                            proposal.name
                        ));
                    };
                    let ids = [proposal.key.clone()];
                    if let Some(write) = package_write(&found.path, note.as_ref(), &ids)? {
                        replace(&write.path, Some(&write.previous), &write.text)?;
                    }
                    if manifest.routes.contains_key(&proposal.key) {
                        replaced.push(holder.id);
                    }
                    manifest.routes.insert(
                        proposal.key.clone(),
                        Route {
                            package: Some(shared.package.id),
                            id: found.id,
                            sources: Vec::new(),
                            link: None,
                            ..holder
                        },
                    );
                }
                // The route says so, so that a save cannot send it to another package instead.
                Move::ToCampaign => {
                    if let Some(route) = manifest.routes.get_mut(&proposal.key) {
                        route.id = campaign_entry_id(owner, &proposal.key);
                        route.package = None;
                        route.link = None;
                        route.sources = vec![proposal.key.clone()];
                    }
                }
                Move::Forget => {}
            }
        }
        let text = manifest.text(owner)?;
        replace(
            &campaign.join(MANIFEST),
            manifest.previous.as_deref(),
            &text,
        )?;
    }
    for record in records {
        record
            .save_campaign_copy_to(campaign, destinations)
            .map_err(|e| e.to_string())?;
    }
    let _saving = saving();
    let mut manifest = Manifest::read(campaign)?;
    let (entries, errors) = crate::content::read_directory(campaign)?;
    let mut used = BTreeSet::new();
    for entry in &entries {
        for reference in crate::content::required_references(entry)? {
            if let Reference::Id(id) = reference {
                used.insert(id);
            }
        }
    }
    // A note goes once a move leaves it unused. A note that cannot be read may use it.
    let mut leaving = Vec::new();
    for proposal in chosen.iter().filter(|p| p.change == Move::Forget) {
        let unrouted = proposal.key.strip_prefix(UNROUTED);
        leaving.extend(
            unrouted
                .and_then(|id| Uuid::parse_str(id).ok())
                .map(|id| (None, id, true)),
        );
        leaving.extend(manifest.routes.get(&proposal.key).map(|route| {
            (
                Some(proposal.key.clone()),
                route.id,
                route.package.is_none(),
            )
        }));
    }
    for (key, route) in &manifest.routes {
        if route.package.is_none()
            && route
                .sources
                .iter()
                .any(|source| source != key && manifest.routes.contains_key(source))
        {
            leaving.push((Some(key.clone()), route.id, true));
        }
    }
    leaving.extend(replaced.into_iter().map(|id| (None, id, true)));
    let mut changed = false;
    for (key, id, has_note) in leaving {
        if used.contains(&id) || !errors.is_empty() {
            continue;
        }
        let note = has_note
            .then(|| entries.iter().find(|entry| entry.id == id))
            .flatten();
        if let Some(note) = note
            && !retire(&campaign.join(&note.path))?
        {
            continue;
        }
        if let Some(key) = key {
            changed |= manifest.routes.remove(&key).is_some();
        }
    }
    if changed {
        let text = manifest.text(owner)?;
        replace(
            &campaign.join(MANIFEST),
            manifest.previous.as_deref(),
            &text,
        )?;
    }
    Ok(chosen.len())
}

/// Moves a generated note with no edits or personal notes to its archive. False means that it has
/// them, or changed during the move, so it stays. Renaming first means that a later save by an
/// editor makes a new file, and no bytes are lost.
fn retire(path: &Path) -> Result<bool, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let copy = super::campaign::parts(&text);
    if !copy.intact || copy.notes != super::campaign::PERSONAL_NOTES {
        return Ok(false);
    }
    let holding = path.with_extension("md.splat-retiring");
    // An interrupted retire can leave bytes here. They go to an archive, not under this rename.
    match fs::read_to_string(&holding) {
        Ok(left) => {
            super::campaign::archive_copy(path, &left).map_err(|e| e.to_string())?;
            fs::remove_file(&holding).map_err(|e| e.to_string())?;
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e.to_string()),
    }
    fs::rename(path, &holding).map_err(|e| e.to_string())?;
    let restore = |error: String| {
        if !path.exists() {
            let _ = fs::rename(&holding, path);
        }
        error
    };
    let held = fs::read_to_string(&holding).map_err(|e| restore(e.to_string()))?;
    if held != text {
        restore(String::new());
        return Ok(false);
    }
    super::campaign::archive_copy(path, &text).map_err(|e| restore(e.to_string()))?;
    fs::remove_file(&holding).map_err(|e| e.to_string())?;
    Ok(true)
}

/// Replaces a file only if it still holds `previous`.
pub(super) fn replace(path: &Path, previous: Option<&str>, text: &str) -> Result<(), String> {
    let current = match fs::read_to_string(path) {
        Ok(text) => Some(text),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => return Err(e.to_string()),
    };
    if current.as_deref() != previous {
        return Err(format!("{} changed during the save.", path.display()));
    }
    let temp = super::campaign::stage(path, text.as_bytes()).map_err(|e| e.to_string())?;
    temp.persist(path).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn fmt_f1_import_routes_stay_in_the_end_fence_when_refreshed() {
        let root = tempfile::tempdir().unwrap();
        let mut manifest = super::Manifest::read(root.path()).unwrap();
        let owner = uuid::Uuid::new_v4();
        let text = manifest.text(owner).unwrap();
        assert!(text.contains("splat_schema: 2"));
        assert!(text.contains("<!-- splat:yaml path=splat_import_routes -->"));
        let path = root.path().join(super::MANIFEST);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, format!("{text}\n%% Keep %%\n")).unwrap();
        manifest = super::Manifest::read(root.path()).unwrap();
        manifest.routes.insert(
            "test".into(),
            super::Route {
                name: "Test".into(),
                kind: "rule".into(),
                id: uuid::Uuid::new_v4(),
                package: None,
                sources: vec![],
                link: None,
            },
        );
        let after = manifest.text(owner).unwrap();
        assert_eq!(
            after
                .matches("<!-- splat:yaml path=splat_import_routes -->")
                .count(),
            1
        );
        assert!(after.ends_with("%% Keep %%\n"));
        assert!(
            !after
                .split("\n---\n")
                .next()
                .unwrap()
                .contains("splat_import_routes:")
        );
        std::fs::write(path, after).unwrap();
        assert_eq!(
            super::Manifest::read(root.path()).unwrap().routes,
            manifest.routes
        );
    }

    use super::super::tests::sample;
    use super::*;

    const WORDS: &str = "A slim wand of dark wood that hums when held.";
    const CAMPAIGN: Uuid = Uuid::from_u128(1);
    const PACKAGE: Uuid = Uuid::from_u128(2);
    const WAND: Uuid = Uuid::from_u128(3);
    const IDENTITY: &str = "item:1:8";
    const LISTED: &str = "splat_dndbeyond_ids:\n- item:1:8\n";

    fn with_words() -> EntitySnapshot {
        let mut record = sample();
        for item in &mut record.inventory {
            item.definition["description"] = json!(format!("<p>{WORDS}</p>"));
            item.definition["definitionKey"] = json!("1:8");
        }
        record
    }
    fn package_note(extra: &str, words: &str) -> String {
        format!(
            "---\nsplat_schema: 1\nsplat_id: {WAND}\nsplat_kind: item\nsplat_key: test:items/wand\nsplat_name: Rod\nsplat_item:\n  category: gear\n{extra}---\n\n# Rod\n\n## Source text\n\n*Gear*\n\n{words}\n"
        )
    }
    fn package_with(root: &Path, extra: &str, words: &str) -> PathBuf {
        let folder = root.join("package");
        fs::create_dir_all(folder.join("Items")).unwrap();
        fs::write(
            folder.join("Package.md"),
            format!("---\nsplat_schema: 1\nsplat_id: {PACKAGE}\nsplat_kind: package\nsplat_name: Test\nsplat_namespace: test\n---\n"),
        )
        .unwrap();
        fs::write(folder.join("Items/Rod.md"), package_note(extra, words)).unwrap();
        folder
    }
    fn package(root: &Path) -> PathBuf {
        package_with(root, LISTED, "Other words entirely, about a different rod.")
    }
    fn enabled(folder: &Path) -> Destinations {
        Destinations::new(
            CAMPAIGN,
            vec![EnabledPackage {
                id: PACKAGE,
                name: "Test".into(),
                path: folder.to_owned(),
                only: None,
            }],
        )
    }
    fn none() -> Destinations {
        Destinations::new(CAMPAIGN, Vec::new())
    }
    fn sheet_uses(record: &EntitySnapshot, campaign: &Path, id: Uuid) -> bool {
        fs::read_to_string(record.campaign_path(campaign))
            .unwrap()
            .contains(&id.to_string())
    }
    /// Places an imported definition in the character's old folder, as older versions did.
    fn old_note(record: &EntitySnapshot, campaign: &Path) -> PathBuf {
        let note = record.content_notes().unwrap().remove(0);
        let path = campaign
            .join("Characters/D&D Beyond")
            .join(&note.relative_path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(
            &path,
            super::super::campaign::stamped(&note.markdown, super::super::campaign::PERSONAL_NOTES),
        )
        .unwrap();
        path
    }
    fn campaign_notes(campaign: &Path, kind: &str) -> Vec<PathBuf> {
        let (entries, _) = crate::content::read_directory(campaign).unwrap();
        entries
            .iter()
            .filter(|entry| entry.kind == kind && in_folder(&entry.path))
            .map(|entry| campaign.join(&entry.path))
            .collect()
    }
    fn all_keys(proposals: &[Proposal]) -> BTreeSet<String> {
        proposals.iter().map(|p| p.key.clone()).collect()
    }
    fn to_rod() -> Move {
        Move::ToPackage("Rod (Test)".into())
    }

    #[test]
    fn a_new_definition_goes_to_the_note_that_lists_its_id_whatever_its_name_or_words() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        let record = with_words();
        record
            .save_campaign_copy_to(campaign.path(), &enabled(&folder))
            .unwrap();
        assert!(sheet_uses(&record, campaign.path(), WAND));
        assert!(campaign_notes(campaign.path(), "item").is_empty());
        let text = fs::read_to_string(folder.join("Items/Rod.md")).unwrap();
        let entry = Entry::parse("Rod.md", &text).unwrap();
        // It fills in what the note lacks, and keeps what the note says.
        assert!(entry.fields["splat_item"].get("equipment").is_some());
        assert_eq!(
            entry.fields["splat_item"]["category"].as_str(),
            Some("gear")
        );
        assert!(entry.body.contains("Other words entirely"));
        assert!(
            check(campaign.path(), &enabled(&folder), &[record])
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn matching_words_alone_only_suggest_a_move_and_applying_it_writes_the_id() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package_with(shared.path(), "", WORDS);
        let destinations = enabled(&folder);
        let records = [with_words()];
        records[0]
            .save_campaign_copy_to(campaign.path(), &destinations)
            .unwrap();
        assert!(!sheet_uses(&records[0], campaign.path(), WAND));
        assert_eq!(
            fs::read_to_string(folder.join("Items/Rod.md")).unwrap(),
            package_note("", WORDS)
        );
        let proposals = check(campaign.path(), &destinations, &records).unwrap();
        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].key, IDENTITY);
        assert_eq!(proposals[0].change, to_rod());

        let keys = all_keys(&proposals);
        apply(campaign.path(), &destinations, &records, &keys).unwrap();
        assert!(sheet_uses(&records[0], campaign.path(), WAND));
        let text = fs::read_to_string(folder.join("Items/Rod.md")).unwrap();
        let entry = Entry::parse("Rod.md", &text).unwrap();
        assert_eq!(listed_ids(&entry), [IDENTITY]);
    }

    #[test]
    fn a_definition_no_package_lists_goes_to_the_campaign_package_for_every_character() {
        let campaign = tempfile::tempdir().unwrap();
        let record = with_words();
        let mut other = record.clone();
        other.source.character_id = "999".into();
        other.definition.splat_id = Uuid::from_u128(9);
        let path_of = |record: &EntitySnapshot| {
            route(campaign.path(), &none(), record.content_notes().unwrap())
                .unwrap()
                .notes
                .remove(0)
                .0
        };
        assert_eq!(path_of(&record), path_of(&other));
        assert!(path_of(&record).starts_with(campaign.path().join(FOLDER)));
    }

    #[test]
    fn an_existing_note_waits_for_review_then_moves_when_applied() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        let before = fs::read_to_string(folder.join("Items/Rod.md")).unwrap();
        let record = with_words();
        let old = old_note(&record, campaign.path());
        let destinations = enabled(&folder);
        record
            .save_campaign_copy_to(campaign.path(), &destinations)
            .unwrap();
        // The old folder is empty and archived, the campaign package has it, and no package changed.
        assert!(!old.exists());
        assert!(old.with_extension("md.splat-archive").exists());
        let note = campaign_notes(campaign.path(), "item").remove(0);
        assert_eq!(
            fs::read_to_string(folder.join("Items/Rod.md")).unwrap(),
            before
        );
        let records = [record];
        let proposals = check(campaign.path(), &destinations, &records).unwrap();
        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].change, to_rod());
        assert_eq!(proposals[0].blocked, None);
        assert!(note.exists());

        let keys = all_keys(&proposals);
        assert_eq!(
            apply(campaign.path(), &destinations, &records, &keys).unwrap(),
            1
        );
        assert!(!note.exists());
        assert!(note.with_extension("md.splat-archive").exists());
        assert!(sheet_uses(&records[0], campaign.path(), WAND));
        assert!(
            fs::read_to_string(folder.join("Items/Rod.md"))
                .unwrap()
                .contains("equipment:")
        );
        assert!(
            check(campaign.path(), &destinations, &records)
                .unwrap()
                .is_empty()
        );
    }

    fn second_package(root: &Path, id: u128, listed: &str) -> (Uuid, PathBuf) {
        let folder = root.join(format!("package-{id}"));
        let package = Uuid::from_u128(id);
        fs::create_dir_all(&folder).unwrap();
        fs::write(
            folder.join("Package.md"),
            format!("---\nsplat_schema: 1\nsplat_id: {package}\nsplat_kind: package\nsplat_name: Other\nsplat_namespace: other{id}\n---\n"),
        )
        .unwrap();
        fs::write(
            folder.join("Staff.md"),
            package_note(listed, "Words of another book, about a staff.")
                .replace(&WAND.to_string(), &Uuid::from_u128(id + 1).to_string())
                .replace("test:items/wand", &format!("other{id}:items/staff")),
        )
        .unwrap();
        (package, folder)
    }

    #[test]
    fn returning_to_the_campaign_cannot_send_a_definition_to_another_package() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        let (other, other_folder) = second_package(shared.path(), 20, LISTED);
        let records = [with_words()];
        records[0]
            .save_campaign_copy_to(campaign.path(), &enabled(&folder))
            .unwrap();
        let only_other = || {
            Destinations::new(
                CAMPAIGN,
                vec![EnabledPackage {
                    id: other,
                    name: "Other".into(),
                    path: other_folder.clone(),
                    only: None,
                }],
            )
        };
        let proposals = check(campaign.path(), &only_other(), &records).unwrap();
        assert_eq!(proposals[0].change, Move::ToCampaign);
        apply(
            campaign.path(),
            &only_other(),
            &records,
            &all_keys(&proposals),
        )
        .unwrap();
        assert!(!sheet_uses(
            &records[0],
            campaign.path(),
            Uuid::from_u128(21)
        ));
        let note = campaign_notes(campaign.path(), "item").remove(0);

        // The other package is still only a choice, and taking it clears the campaign note.
        let proposals = check(campaign.path(), &only_other(), &records).unwrap();
        assert_eq!(proposals.len(), 1);
        assert!(matches!(&proposals[0].change, Move::ToPackage(name) if name.contains("Other")));
        apply(
            campaign.path(),
            &only_other(),
            &records,
            &all_keys(&proposals),
        )
        .unwrap();
        assert!(sheet_uses(
            &records[0],
            campaign.path(),
            Uuid::from_u128(21)
        ));
        assert!(!note.exists());
        assert!(note.with_extension("md.splat-archive").exists());
        assert!(
            check(campaign.path(), &only_other(), &records)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn a_note_that_cannot_be_read_blocks_removal() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        let record = with_words();
        old_note(&record, campaign.path());
        let destinations = enabled(&folder);
        record
            .save_campaign_copy_to(campaign.path(), &destinations)
            .unwrap();
        let note = campaign_notes(campaign.path(), "item").remove(0);
        fs::write(
            record.campaign_path(campaign.path()),
            "---\nsplat_schema: [\n---\n",
        )
        .unwrap();
        let proposals = check(campaign.path(), &destinations, &[]).unwrap();
        assert_eq!(proposals[0].change, Move::Forget);
        assert!(
            proposals[0]
                .blocked
                .as_deref()
                .unwrap()
                .contains("cannot be read")
        );
        apply(campaign.path(), &destinations, &[], &all_keys(&proposals)).unwrap();
        assert!(note.exists());
    }

    #[test]
    fn a_note_the_campaign_no_longer_selects_takes_no_writes() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        let path = folder.join("Items/Rod.md");
        let original = fs::read_to_string(&path).unwrap();
        let records = [with_words()];
        records[0]
            .save_campaign_copy_to(campaign.path(), &enabled(&folder))
            .unwrap();
        fs::write(&path, &original).unwrap();
        let limited = Destinations::new(
            CAMPAIGN,
            vec![EnabledPackage {
                id: PACKAGE,
                name: "Test".into(),
                path: folder.clone(),
                only: Some(Vec::new()),
            }],
        );
        records[0]
            .save_campaign_copy_to(campaign.path(), &limited)
            .unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), original);
        assert_eq!(
            check(campaign.path(), &limited, &records).unwrap()[0].change,
            Move::ToCampaign
        );
    }

    #[test]
    fn a_failed_apply_can_run_again() {
        let campaign = tempfile::tempdir().unwrap();
        let destinations = none();
        let first = with_words();
        first
            .save_campaign_copy_to(campaign.path(), &destinations)
            .unwrap();
        let note = campaign_notes(campaign.path(), "item").remove(0);
        // The character no longer carries the item, so the note has no use.
        let mut record = first.clone();
        record.inventory.clear();
        record
            .save_campaign_copy_to(campaign.path(), &destinations)
            .unwrap();
        let records = [record];
        let sheet = records[0].campaign_path(campaign.path());
        let saved = fs::read_to_string(&sheet).unwrap();
        let proposals = check(campaign.path(), &destinations, &records).unwrap();
        let keys: BTreeSet<_> = proposals
            .iter()
            .filter(|p| p.change == Move::Forget && p.kind == "item")
            .map(|p| p.key.clone())
            .collect();
        assert_eq!(keys.len(), 1);

        fs::write(&sheet, saved.replace("# Example", "# Edited")).unwrap();
        assert!(apply(campaign.path(), &destinations, &records, &keys).is_err());
        assert!(note.exists());
        assert!(
            check(campaign.path(), &destinations, &records)
                .unwrap()
                .iter()
                .any(|p| keys.contains(&p.key))
        );

        fs::write(&sheet, saved).unwrap();
        assert_eq!(
            apply(campaign.path(), &destinations, &records, &keys).unwrap(),
            1
        );
        assert!(!note.exists());
        assert!(note.with_extension("md.splat-archive").exists());
    }

    #[test]
    fn a_route_keeps_its_link_while_its_package_is_away() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        let record = with_words();
        record
            .save_campaign_copy_to(campaign.path(), &enabled(&folder))
            .unwrap();
        let link = |campaign: &Path| {
            Manifest::read(campaign).unwrap().routes[IDENTITY]
                .link
                .clone()
        };
        let before = link(campaign.path());
        assert!(before.as_deref().unwrap().ends_with("package/Items/Rod.md"));
        record
            .save_campaign_copy_to(campaign.path(), &none())
            .unwrap();
        assert_eq!(link(campaign.path()), before);
    }

    #[test]
    fn a_package_that_is_not_the_one_registered_takes_nothing() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        let original = fs::read_to_string(folder.join("Items/Rod.md")).unwrap();
        let wrong = Destinations::new(
            CAMPAIGN,
            vec![EnabledPackage {
                id: Uuid::from_u128(99),
                name: "Test".into(),
                path: folder.clone(),
                only: None,
            }],
        );
        assert!(!wrong.lists(IDENTITY));
        with_words()
            .save_campaign_copy_to(campaign.path(), &wrong)
            .unwrap();
        assert_eq!(
            fs::read_to_string(folder.join("Items/Rod.md")).unwrap(),
            original
        );
    }

    #[test]
    fn a_default_save_uses_the_campaigns_own_identity() {
        let id_of = |campaign_id: u128| {
            let campaign = tempfile::tempdir().unwrap();
            let id = Uuid::from_u128(campaign_id);
            fs::write(
                campaign.path().join("Campaign.md"),
                format!("---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: campaign\nsplat_name: C\nsplat_namespace: c{campaign_id}\n---\n"),
            )
            .unwrap();
            with_words().save_campaign_copy(campaign.path()).unwrap();
            let text = fs::read_to_string(campaign.path().join(MANIFEST)).unwrap();
            let entry = Entry::parse(MANIFEST, &text).unwrap();
            assert_eq!(entry.id, package_id(id));
            entry.id
        };
        assert_ne!(id_of(31), id_of(32));
    }

    #[test]
    fn empty_identities_are_not_identities() {
        let mut record = with_words();
        for item in &mut record.inventory {
            item.definition["definitionKey"] = json!("");
        }
        assert!(
            source_words(std::slice::from_ref(&record))
                .keys()
                .all(|k| k != "item:")
        );
        assert!(record.content_notes().unwrap()[0].sources.is_empty());
    }

    #[test]
    fn a_spell_has_one_identity_everywhere() {
        let mut record = with_words();
        record.character.spells.push(json!({"definition": {
            "name": "Spark", "definitionKey": "spell-key", "level": 0,
            "description": "<p>A spark leaps from your finger to the target.</p>",
        }}));
        let notes = record.content_notes().unwrap();
        let spell = notes
            .iter()
            .find(|note| note.sources.iter().any(|s| s.starts_with("spell:")))
            .unwrap();
        assert_eq!(spell.sources, ["spell:spell-key"]);
        assert!(source_words(&[record]).contains_key("spell:spell-key"));
    }

    #[test]
    fn saves_at_the_same_time_keep_each_others_routes() {
        for _ in 0..12 {
            let campaign = tempfile::tempdir().unwrap();
            let saved: Vec<_> = (0..4)
                .map(|n| {
                    let mut record = with_words();
                    for item in &mut record.inventory {
                        item.definition["definitionKey"] = json!(format!("1:{n}"));
                        item.definition["description"] =
                            json!(format!("<p>Words of the wand number {n}, which hums.</p>"));
                    }
                    record
                })
                .collect();
            std::thread::scope(|scope| {
                for record in &saved {
                    let path = campaign.path();
                    scope.spawn(move || record.save_campaign_copy_to(path, &none()).unwrap());
                }
            });
            let routes = Manifest::read(campaign.path()).unwrap().routes;
            let items = routes.values().filter(|route| route.kind == "item").count();
            assert_eq!(items, 4);
        }
    }

    #[test]
    fn a_note_that_changes_while_it_retires_stays() {
        let campaign = tempfile::tempdir().unwrap();
        let record = with_words();
        record.save_campaign_copy(campaign.path()).unwrap();
        let note = campaign_notes(campaign.path(), "item").remove(0);
        let text = fs::read_to_string(&note).unwrap();
        fs::write(&note, format!("{text}Mine.\n")).unwrap();
        assert!(!retire(&note).unwrap());
        assert!(note.exists());
        fs::write(&note, &text).unwrap();
        assert!(retire(&note).unwrap());
        assert!(!note.exists());
        assert!(!note.with_extension("md.splat-retiring").exists());
        assert_eq!(
            fs::read_to_string(note.with_extension("md.splat-archive")).unwrap(),
            text
        );
    }

    #[test]
    fn a_leftover_from_an_interrupted_retire_is_archived_and_not_overwritten() {
        let campaign = tempfile::tempdir().unwrap();
        let record = with_words();
        record.save_campaign_copy(campaign.path()).unwrap();
        let note = campaign_notes(campaign.path(), "item").remove(0);
        let left = "Bytes that an interrupted retire left behind.\n";
        fs::write(note.with_extension("md.splat-retiring"), left).unwrap();
        let text = fs::read_to_string(&note).unwrap();
        assert!(retire(&note).unwrap());
        assert!(!note.with_extension("md.splat-retiring").exists());
        let base = note.with_extension("md.splat-archive");
        assert_eq!(fs::read_to_string(&base).unwrap(), left);
        let next = PathBuf::from(format!("{}.1", base.display()));
        assert_eq!(fs::read_to_string(next).unwrap(), text);
    }

    #[test]
    fn a_note_that_no_route_knows_is_offered_for_removal() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        let destinations = enabled(&folder);
        let records = [with_words()];
        records[0]
            .save_campaign_copy_to(campaign.path(), &none())
            .unwrap();
        let note = campaign_notes(campaign.path(), "item").remove(0);
        // A failed apply leaves this state: the route has moved, and the sheet follows it.
        let mut manifest = Manifest::read(campaign.path()).unwrap();
        manifest.routes.retain(|_, route| route.kind != "item");
        let text = manifest.text(CAMPAIGN).unwrap();
        fs::write(campaign.path().join(MANIFEST), text).unwrap();
        records[0]
            .save_campaign_copy_to(campaign.path(), &destinations)
            .unwrap();
        assert!(sheet_uses(&records[0], campaign.path(), WAND));
        assert!(note.exists());

        let proposals = check(campaign.path(), &destinations, &records).unwrap();
        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].change, Move::Forget);
        assert!(proposals[0].key.starts_with(UNROUTED));
        let keys = all_keys(&proposals);
        assert_eq!(
            apply(campaign.path(), &destinations, &records, &keys).unwrap(),
            1
        );
        assert!(!note.exists());
        assert!(note.with_extension("md.splat-archive").exists());
        assert!(
            check(campaign.path(), &destinations, &records)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn a_note_goes_when_a_move_leaves_nothing_using_it() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        let destinations = enabled(&folder);
        let first = with_words();
        old_note(&first, campaign.path());
        first
            .save_campaign_copy_to(campaign.path(), &destinations)
            .unwrap();
        // This definition has a second ID, as one rule on two source records can.
        let mut manifest = Manifest::read(campaign.path()).unwrap();
        let route = manifest
            .routes
            .values_mut()
            .find(|r| r.kind == "item")
            .unwrap();
        route.sources.push("item:other".into());
        let text = manifest.text(CAMPAIGN).unwrap();
        fs::write(campaign.path().join(MANIFEST), text).unwrap();
        let note = campaign_notes(campaign.path(), "item").remove(0);

        let records = [first];
        let proposals = check(campaign.path(), &destinations, &records).unwrap();
        let keys = all_keys(&proposals);
        apply(campaign.path(), &destinations, &records, &keys).unwrap();
        assert!(sheet_uses(&records[0], campaign.path(), WAND));
        assert!(!note.exists());
        assert!(note.with_extension("md.splat-archive").exists());
        assert!(
            check(campaign.path(), &destinations, &records)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn a_change_of_packages_moves_nothing_until_it_is_applied() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        let records = [with_words()];
        records[0]
            .save_campaign_copy_to(campaign.path(), &enabled(&folder))
            .unwrap();
        let proposals = check(campaign.path(), &none(), &records).unwrap();
        assert_eq!(proposals.len(), 1);
        assert_eq!(proposals[0].change, Move::ToCampaign);
        // A refresh follows the route that stands.
        records[0]
            .save_campaign_copy_to(campaign.path(), &none())
            .unwrap();
        assert!(sheet_uses(&records[0], campaign.path(), WAND));
        assert!(campaign_notes(campaign.path(), "item").is_empty());

        apply(campaign.path(), &none(), &records, &all_keys(&proposals)).unwrap();
        assert!(!sheet_uses(&records[0], campaign.path(), WAND));
        assert_eq!(campaign_notes(campaign.path(), "item").len(), 1);
        assert!(
            check(campaign.path(), &none(), &records)
                .unwrap()
                .is_empty()
        );
        // The package note keeps the ID, so it can take the entry again.
        assert_eq!(
            check(campaign.path(), &enabled(&folder), &records).unwrap()[0].change,
            to_rod()
        );
    }

    #[test]
    fn personal_notes_and_edits_block_a_move() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        let records = [with_words()];
        old_note(&records[0], campaign.path());
        let destinations = enabled(&folder);
        records[0]
            .save_campaign_copy_to(campaign.path(), &destinations)
            .unwrap();
        let note = campaign_notes(campaign.path(), "item").remove(0);
        let text = fs::read_to_string(&note).unwrap();

        fs::write(&note, format!("{text}Mine.\n")).unwrap();
        let proposals = check(campaign.path(), &destinations, &records).unwrap();
        assert_eq!(
            proposals[0].blocked.as_deref(),
            Some("It has personal notes.")
        );
        let keys = all_keys(&proposals);
        assert_eq!(
            apply(campaign.path(), &destinations, &records, &keys).unwrap(),
            0
        );
        assert!(note.exists());

        fs::write(
            &note,
            text.replace("splat_name: Wand", "splat_name: My wand"),
        )
        .unwrap();
        let proposals = check(campaign.path(), &destinations, &records).unwrap();
        assert_eq!(proposals[0].blocked.as_deref(), Some("It has local edits."));
    }

    #[test]
    fn sheets_without_saved_data_block_a_move() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        let record = with_words();
        old_note(&record, campaign.path());
        let destinations = enabled(&folder);
        record
            .save_campaign_copy_to(campaign.path(), &destinations)
            .unwrap();
        let proposals = check(campaign.path(), &destinations, &[]).unwrap();
        assert!(
            proposals[0]
                .blocked
                .as_deref()
                .unwrap()
                .contains("no saved")
        );
    }

    #[test]
    fn an_id_listed_on_two_notes_is_blocked_until_one_note_has_it() {
        let (campaign, shared) = (tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap());
        let folder = package(shared.path());
        fs::write(
            folder.join("Items/Copy.md"),
            package_note(LISTED, "Other words.")
                .replace(&WAND.to_string(), &Uuid::from_u128(4).to_string())
                .replace("test:items/wand", "test:items/copy"),
        )
        .unwrap();
        let destinations = enabled(&folder);
        let records = [with_words()];
        records[0]
            .save_campaign_copy_to(campaign.path(), &destinations)
            .unwrap();
        // With two candidates it does not guess: the definition goes to the campaign package.
        assert!(!sheet_uses(&records[0], campaign.path(), WAND));
        let proposals = check(campaign.path(), &destinations, &records).unwrap();
        assert!(
            proposals[0]
                .blocked
                .as_deref()
                .unwrap()
                .contains("More than one")
        );
    }

    #[test]
    fn a_package_note_with_other_rule_data_cannot_take_a_definition() {
        let shared = tempfile::tempdir().unwrap();
        let extra = format!(
            "{LISTED}splat_rule:\n  mechanic: test:rules/other\n  implementation: test:rules/other\n"
        );
        let folder = package_with(shared.path(), &extra, "Other words entirely here.");
        let note = Entry::parse(
            "note.md",
            &format!("---\nsplat_schema: 1\nsplat_id: {CAMPAIGN}\nsplat_kind: item\nsplat_name: Wand\nsplat_item:\n  category: gear\nsplat_rule:\n  mechanic: test:rules/mine\n  implementation: test:rules/mine\n---\n\n# Wand\n"),
        )
        .unwrap();
        assert!(
            package_write(
                &folder.join("Items/Rod.md"),
                Some(&note),
                &[IDENTITY.into()]
            )
            .is_err()
        );
    }

    #[test]
    fn a_note_with_several_traits_takes_ids_and_no_rule_data() {
        let shared = tempfile::tempdir().unwrap();
        let path = shared.path().join("Goblin.md");
        let body = "\n# Goblin\n\n## Source text\n\n***Fury.*** You hit hard.\n";
        fs::write(
            &path,
            format!("---\nsplat_schema: 1\nsplat_id: {WAND}\nsplat_kind: reference\nsplat_name: Goblin\nsplat_reference:\n  source: Test\n  category: species\n  executable: false\n---\n{body}"),
        )
        .unwrap();
        let note = Entry::parse(
            "note.md",
            &format!("---\nsplat_schema: 1\nsplat_id: {CAMPAIGN}\nsplat_kind: rule\nsplat_name: Fury\nsplat_rule:\n  mechanic: test:rules/fury\n  implementation: test:rules/fury\n---\n"),
        )
        .unwrap();
        let write = package_write(&path, Some(&note), &["feature:1".into()])
            .unwrap()
            .unwrap();
        assert!(write.text.contains("- feature:1"));
        assert!(!write.text.contains("splat_rule"));
        assert!(write.text.ends_with(body));
    }

    #[test]
    fn a_trait_looks_first_in_the_characters_species_note() {
        let shared = tempfile::tempdir().unwrap();
        let folder = package_with(shared.path(), "", "Unrelated words about a rod.");
        for (name, n) in [("Goblin", 5u128), ("Tabaxi", 6)] {
            fs::write(
                folder.join(format!("{name}.md")),
                format!(
                    "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: reference\nsplat_name: {name}\nsplat_reference:\n  source: Test\n  category: species\n  executable: false\n---\n\n# {name}\n\n***Darkvision.*** You see in the dark for 60 feet.\n\n***Small.*** You are Small.\n",
                    Uuid::from_u128(n)
                ),
            )
            .unwrap();
        }
        let destinations = enabled(&folder);
        let source = |text: &str, species: &str| SourceWords {
            words: normalized(text),
            species: species.into(),
        };
        let dark = "You see in the dark for 60 feet.";
        assert!(matches!(
            destinations.suggested("rule", &source(dark, "Goblin")),
            Found::One(_, entry) if entry.name == "Goblin"
        ));
        assert!(matches!(
            destinations.suggested("rule", &source(dark, "Dwarf")),
            Found::Several(_)
        ));
        // Short text can match only in the character's own species note.
        assert!(matches!(
            destinations.suggested("rule", &source("You are Small.", "Tabaxi")),
            Found::One(_, entry) if entry.name == "Tabaxi"
        ));
        assert!(matches!(
            destinations.suggested("rule", &source("You are Small.", "Dwarf")),
            Found::None
        ));
    }

    #[test]
    fn a_selection_of_other_entries_is_not_a_destination() {
        let shared = tempfile::tempdir().unwrap();
        let folder = package(shared.path());
        let limited = |only: Vec<Reference>| {
            Destinations::new(
                CAMPAIGN,
                vec![EnabledPackage {
                    id: PACKAGE,
                    name: "Test".into(),
                    path: folder.clone(),
                    only: Some(only),
                }],
            )
        };
        assert!(matches!(limited(Vec::new()).mapped(IDENTITY), Found::None));
        assert!(matches!(
            limited(vec![Reference::Key("test:items/wand".into())]).mapped(IDENTITY),
            Found::One(..)
        ));
    }

    #[test]
    fn words_match_only_as_whole_words() {
        let package = normalized("*Gear*\n\nA \u{2019}slim\u{2019} wand of dark wood hums.");
        assert!(same_words(
            &package,
            &normalized("A 'slim' wand of dark wood hums.")
        ));
        assert!(!same_words(
            &package,
            &normalized("slim' wand of dark wood hums.")
        ));
        assert!(!same_words(&package, "hums."));
        assert!(!same_words(&package, ""));
        assert!(holds_words(&package, &normalized("A 'slim' wand")));
        assert!(!holds_words(&package, &normalized("A 'slim' wan")));
    }
}
