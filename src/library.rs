//! One catalogue: built-in, package and campaign-owned entries, with their authoring.
//! Browsing it never changes the active campaign.
mod class;
mod creature;
pub(crate) mod presentation;
mod reader;
pub(crate) mod view;
use crate::{
    campaigns::Campaigns,
    content_workspace::{CampaignDocument, Catalogue},
    design::{self, Action, Density, Status, Tone, Type, colors, icons, reader as reader_style},
    persistence::{Campaign, Module},
};
use eframe::egui;
use presentation::Section;
use splat::content::{self, Entry};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::mpsc,
};
use uuid::Uuid;

const BUILTIN_NAME: &str = "D&D 2024 · SRD 5.2.1";

/// Who owns an entry, and so whether and where it can be saved.
#[derive(Clone, Default, PartialEq, Debug)]
enum Owner {
    #[default]
    Builtin,
    Package(Module),
    Campaign(Campaign),
}

impl Owner {
    fn id(&self) -> Option<Uuid> {
        match self {
            Self::Builtin => None,
            Self::Package(module) => Some(module.id),
            Self::Campaign(campaign) => Some(campaign.id),
        }
    }
    fn root(&self) -> Option<&Path> {
        match self {
            Self::Builtin => None,
            Self::Package(module) => Some(&module.path),
            Self::Campaign(campaign) => Some(&campaign.path),
        }
    }
    fn editable(&self) -> bool {
        !matches!(self, Self::Builtin)
    }
    fn label(&self) -> String {
        match self {
            Self::Builtin => "Built-in package · Read only".into(),
            Self::Package(module) => format!("Package · {}", module.name),
            Self::Campaign(campaign) => format!("Campaign · {}", campaign.name),
        }
    }
}

/// What the catalogue shows. The default is what the active campaign can use.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
enum Scope {
    #[default]
    Uses,
    All,
    Builtin,
    Package(Uuid),
    Campaign,
}

struct Note {
    path: String,
    text: String,
    entry: Option<Entry>,
    error: Option<String>,
    owner: Owner,
}
impl Note {
    fn name(&self) -> &str {
        self.entry.as_ref().map_or(&self.path, |e| &e.name)
    }
    fn section(&self) -> Section {
        self.entry
            .as_ref()
            .map_or(Section::Other, Section::for_entry)
    }
    fn matches(&self, section: Section, query: &str) -> bool {
        if !query.is_empty() {
            self.name().to_lowercase().contains(query)
                || self.path.to_lowercase().contains(query)
                || self.section().label().to_lowercase().contains(query)
        } else {
            section == Section::All || self.section() == section
        }
    }
}
#[derive(Default)]
struct Loaded {
    notes: Vec<Note>,
    diagnostics: Vec<String>,
    /// What the active campaign can use: its selections and what they require.
    uses: Option<std::collections::HashSet<Uuid>>,
    subtitles: std::sync::OnceLock<Vec<String>>,
    counts: std::sync::OnceLock<Vec<(Section, usize)>>,
}
impl Loaded {
    fn in_scope(&self, note: &Note, scope: Scope) -> bool {
        match scope {
            Scope::All => true,
            Scope::Uses => match &self.uses {
                None => true,
                Some(uses) => {
                    matches!(note.owner, Owner::Campaign(_))
                        || note.entry.as_ref().is_some_and(|e| uses.contains(&e.id))
                }
            },
            Scope::Builtin => note.owner == Owner::Builtin,
            Scope::Package(id) => {
                matches!(note.owner, Owner::Package(_)) && note.owner.id() == Some(id)
            }
            Scope::Campaign => matches!(note.owner, Owner::Campaign(_)),
        }
    }

    fn resolve_reference(&self, reference: &content::Reference) -> Option<usize> {
        let mut matches = self.notes.iter().enumerate().filter_map(|(i, note)| {
            let entry = note.entry.as_ref()?;
            let matches = match reference {
                content::Reference::Id(id) => entry.id == *id,
                content::Reference::Key(key) => entry.key.as_ref() == Some(key),
            };
            matches.then_some(i)
        });
        let first = matches.next()?;
        matches.next().is_none().then_some(first)
    }

    fn resolve_spell(&self, reference: &content::Reference) -> Option<usize> {
        self.resolve_reference(reference).filter(|&index| {
            self.notes[index]
                .entry
                .as_ref()
                .is_some_and(|entry| entry.kind == "spell")
        })
    }

    fn subtitles(&self) -> &Vec<String> {
        self.subtitles.get_or_init(|| self.build_subtitles())
    }
    #[cfg(test)]
    fn count(&self, section: Section) -> usize {
        self.counts
            .get_or_init(|| {
                Section::GROUPS
                    .iter()
                    .flat_map(|(_, sections)| sections.iter())
                    .map(|&section| {
                        (
                            section,
                            self.notes
                                .iter()
                                .filter(|n| section == Section::All || n.section() == section)
                                .count(),
                        )
                    })
                    .collect()
            })
            .iter()
            .find(|(s, _)| *s == section)
            .map_or(0, |(_, n)| *n)
    }
    fn invalidate(&mut self) {
        self.subtitles.take();
        self.counts.take();
    }
    fn build_subtitles(&self) -> Vec<String> {
        let mut subtitles: Vec<_> = self
            .notes
            .iter()
            .map(|note| {
                let line = note
                    .entry
                    .as_ref()
                    .map_or_else(|| "Invalid note".to_owned(), presentation::subtitle);
                match &note.owner {
                    Owner::Builtin => line,
                    Owner::Package(module) => format!("{line} · {}", module.name),
                    Owner::Campaign(_) => format!("{line} · Campaign"),
                }
            })
            .collect();
        let mut labels = std::collections::HashMap::new();
        for (i, note) in self.notes.iter().enumerate() {
            labels
                .entry((note.name(), subtitles[i].clone()))
                .or_insert_with(Vec::new)
                .push(i);
        }
        for indices in labels.values().filter(|indices| indices.len() > 1) {
            for &i in indices {
                // The printed page tells them apart, in the DM's own terms.
                if let Some(page) = self.notes[i].entry.as_ref().and_then(source_page) {
                    subtitles[i].push_str(&format!(" · {page}"));
                }
            }
        }
        // Two on one page: a plain count is all that is left to tell them apart.
        let mut seen = std::collections::HashMap::new();
        for (i, note) in self.notes.iter().enumerate() {
            let count = seen
                .entry((note.name(), subtitles[i].clone()))
                .or_insert(0_usize);
            *count += 1;
            if *count > 1 {
                subtitles[i].push_str(&format!(" · {count}"));
            }
        }
        subtitles
    }
}

impl view::Links for Loaded {
    fn find(&self, reference: &content::Reference) -> Option<(usize, String)> {
        let index = self.resolve_reference(reference)?;
        Some((index, self.notes[index].name().to_owned()))
    }
    fn find_spell(&self, reference: &content::Reference) -> Option<(usize, String)> {
        let index = self.resolve_spell(reference)?;
        Some((index, self.notes[index].name().to_owned()))
    }
    fn spells_named_in(&self, text: &str) -> Vec<(usize, String)> {
        self.notes
            .iter()
            .enumerate()
            .filter(|(_, note)| {
                note.entry.as_ref().is_some_and(|e| e.kind == "spell")
                    && crate::lookup::names_word(text, note.name())
            })
            .map(|(index, note)| (index, note.name().to_owned()))
            .collect()
    }
}

const ENTRY_ROW_HEIGHT: f32 = 44.0;
const SEARCH_NAME: &str = "Search the Library";

// Fixed height keeps the virtual scroll position independent of label length.
fn entry_row(ui: &mut egui::Ui, name: &str, subtitle: &str, selected: bool) -> egui::Response {
    design::two_line_row(ui, ENTRY_ROW_HEIGHT, selected, name, subtitle)
}

fn local_module(path: &Path) -> Result<Module, String> {
    let path = path.canonicalize().map_err(|e| e.to_string())?;
    let (entries, _) = content::read_directory(&path)?;
    module_from_entries(path, &entries)
}

fn module_from_entries(path: PathBuf, entries: &[Entry]) -> Result<Module, String> {
    let manifests: Vec<_> = entries.iter().filter(|e| e.kind == "package").collect();
    if manifests.len() != 1 || entries.iter().any(|e| e.kind == "campaign") {
        return Err(
            "Choose a package folder with exactly one package manifest and no campaign manifest."
                .into(),
        );
    }
    let manifest = manifests[0];
    let ns = manifest
        .fields
        .get("splat_namespace")
        .and_then(|v| v.as_str())
        .ok_or("The package needs a namespace.")?;
    if !content::resource_key(&format!("{ns}:library")) || ns == "dnd2024" {
        return Err(
            "Use a valid package namespace other than the built-in dnd2024 namespace.".into(),
        );
    }
    Ok(Module {
        id: manifest.id,
        name: manifest.name.clone(),
        path,
    })
}

pub(crate) fn note_text(name: &str, kind: &str, namespace: Option<&str>) -> String {
    let mut fields = serde_yaml_ng::Mapping::new();
    fields.insert("splat_schema".into(), 1.into());
    fields.insert(
        "splat_id".into(),
        crate::hermetic::new_uuid().to_string().into(),
    );
    fields.insert("splat_kind".into(), kind.into());
    fields.insert("splat_name".into(), name.into());
    if namespace.is_none() {
        fields.insert(
            format!("splat_{kind}").into(),
            serde_yaml_ng::Value::Mapping(serde_yaml_ng::Mapping::new()),
        );
    }
    if let Some(ns) = namespace {
        fields.insert("splat_namespace".into(), ns.into());
    }
    format!(
        "---\n{}---\n\n# {}\n\n",
        serde_yaml_ng::to_string(&fields).expect("string fields"),
        name
    )
}

fn create_module(path: &Path, name: &str, namespace: &str) -> Result<Module, String> {
    if !path.is_absolute()
        || name.trim().is_empty()
        || namespace == "dnd2024"
        || !content::resource_key(&format!("{namespace}:library"))
    {
        return Err(
            "Enter a name, a new absolute folder path, and a namespace such as my_homebrew.".into(),
        );
    }
    fs::create_dir(path).map_err(|e| format!("Could not create a new package folder: {e}"))?;
    let text = note_text(name.trim(), "package", Some(namespace));
    if let Err(error) = write_note(path, "Module.md", None, &text) {
        let _ = fs::remove_dir(path);
        return Err(error);
    }
    local_module(path)
}

// Refuse traversal and symlinks, and preserve external edits before atomic replacement.
pub(crate) fn note_path(root: &Path, relative: &str) -> Result<PathBuf, String> {
    if fs::symlink_metadata(root)
        .map_err(|e| e.to_string())?
        .file_type()
        .is_symlink()
    {
        return Err(
            "The package folder was replaced with a symbolic link. Register the actual folder."
                .into(),
        );
    }
    let relative = Path::new(relative);
    if relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|c| !matches!(c, std::path::Component::Normal(_)))
    {
        return Err("Use a relative note path within the package.".into());
    }
    let mut path = root.to_path_buf();
    for component in relative.components() {
        path.push(component);
        if fs::symlink_metadata(&path).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err("Editing through symbolic links is not supported.".into());
        }
    }
    Ok(path)
}

pub(crate) fn write_note(
    root: &Path,
    relative: &str,
    baseline: Option<&str>,
    text: &str,
) -> Result<Entry, String> {
    let entry = Entry::parse(relative, text)?;
    let path = note_path(root, relative)?;
    let current = match fs::read_to_string(&path) {
        Ok(text) => Some(text),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => return Err(e.to_string()),
    };
    if baseline.is_none() && current.is_some() {
        return Err("A note with this name already exists. Choose another name.".into());
    }
    if current.as_deref() != baseline {
        return Err(NOTE_CHANGED.into());
    }
    let mut temp = tempfile::NamedTempFile::new_in(path.parent().ok_or("Missing parent folder")?)
        .map_err(|e| e.to_string())?;
    temp.write_all(text.as_bytes())
        .and_then(|_| temp.as_file().sync_all())
        .map_err(|e| e.to_string())?;
    if baseline.is_some() {
        temp.persist(&path).map_err(|e| e.to_string())?;
    } else {
        temp.persist_noclobber(&path).map_err(|e| e.to_string())?;
    }
    Ok(entry)
}

fn load(module: Option<&Module>) -> Result<Loaded, String> {
    let mut notes = Vec::new();
    let mut diagnostics = Vec::new();
    if let Some(module) = module {
        let path = module.path.canonicalize().map_err(|e| e.to_string())?;
        let (entries, errors) = content::read_directory(&path)?;
        let actual = module_from_entries(path, &entries)?;
        if actual.id != module.id {
            return Err(
                "The package at this folder has a different identity. Register it separately."
                    .into(),
            );
        }
        let report = content::validate(&entries);
        diagnostics.extend(
            report
                .errors
                .into_iter()
                .chain(report.warnings)
                .map(|d| format!("{}: {}", d.path, d.message)),
        );
        for entry in entries {
            let text = fs::read_to_string(note_path(&module.path, &entry.path)?)
                .map_err(|e| e.to_string())?;
            notes.push(Note {
                path: entry.path.clone(),
                text,
                entry: Some(entry),
                error: None,
                owner: Owner::Package(module.clone()),
            });
        }
        for error in errors {
            let text = fs::read_to_string(note_path(&module.path, &error.path)?)
                .map_err(|e| e.to_string())?;
            diagnostics.push(format!("{}: {}", error.path, error.message));
            notes.push(Note {
                path: error.path,
                text,
                entry: None,
                error: Some(error.message),
                owner: Owner::Package(module.clone()),
            });
        }
    } else {
        for entry in content::definitions::entries()?.iter().cloned() {
            let text = content::definitions::source_file(&entry.path)
                .ok_or("Missing built-in note")?
                .to_owned();
            notes.push(Note {
                path: entry.path.clone(),
                text,
                entry: Some(entry),
                error: None,
                owner: Owner::Builtin,
            });
        }
    }
    notes.sort_by_cached_key(|n| natural_key(n.name()));
    Ok(Loaded {
        notes,
        diagnostics,
        ..Default::default()
    })
}

/// The whole catalogue: built-in, every package, and the active campaign's own entries.
fn load_all(modules: &[Module], campaign: Option<&Campaign>) -> Result<Loaded, String> {
    let mut all = load(None)?;
    for module in modules {
        match load(Some(module)) {
            Ok(package) => {
                all.notes.extend(package.notes);
                all.diagnostics.extend(package.diagnostics);
            }
            Err(error) => all.diagnostics.push(format!("{}: {error}", module.name)),
        }
    }
    if let Some(campaign) = campaign {
        let owner = Owner::Campaign(campaign.clone());
        match content::read_directory(&campaign.path) {
            Ok((entries, errors)) => {
                for entry in entries.into_iter().filter(|e| e.kind != "campaign") {
                    let text = fs::read_to_string(note_path(&campaign.path, &entry.path)?)
                        .map_err(|e| e.to_string())?;
                    all.notes.push(Note {
                        path: entry.path.clone(),
                        text,
                        entry: Some(entry),
                        error: None,
                        owner: owner.clone(),
                    });
                }
                for error in errors {
                    let text = fs::read_to_string(note_path(&campaign.path, &error.path)?)
                        .map_err(|e| e.to_string())?;
                    all.notes.push(Note {
                        path: error.path,
                        text,
                        entry: None,
                        error: Some(error.message),
                        owner: owner.clone(),
                    });
                }
            }
            Err(error) => all.diagnostics.push(format!("{}: {error}", campaign.name)),
        }
        // If the selections cannot be read, say so and show everything rather than nothing.
        match CampaignDocument::load(campaign)
            .and_then(|document| Ok((document, Catalogue::load(modules, Some(campaign))?)))
        {
            Ok((document, catalogue)) => {
                let closure = catalogue.closure(&document.selections, campaign.id);
                all.uses = Some(
                    closure
                        .direct
                        .iter()
                        .copied()
                        .chain(closure.required.keys().copied())
                        .collect(),
                );
            }
            Err(error) => all.diagnostics.push(format!("{}: {error}", campaign.name)),
        }
    }
    all.notes.sort_by_cached_key(|n| natural_key(n.name()));
    Ok(all)
}

/// Undo discard offers words only to the note they came from. Two owners can each have a
/// note at the same path.
fn discard_is_for(discarded: Option<&(Option<Uuid>, String, String)>, note: &Note) -> bool {
    discarded.is_some_and(|(owner, path, _)| *owner == note.owner.id() && *path == note.path)
}

/// The catalogue needs reloading when either of these changes.
type LoadKey = (Option<Uuid>, Vec<Uuid>);

fn load_key(campaigns: &Campaigns) -> LoadKey {
    (
        campaigns.active_campaign().map(|c| c.id),
        campaigns.modules().iter().map(|m| m.id).collect(),
    )
}

#[derive(Default)]
pub struct Library {
    list_is_empty: bool,
    /// The list should scroll to the selected entry, which arrived by a link.
    reveal_row: bool,
    /// The draft `Discard changes` last dropped, with its note's path, for one step of undo.
    /// The note's owner and path, and the discarded words.
    discarded: Option<(Option<Uuid>, String, String)>,
    /// A save was refused because the file changed. The draft waits for the DM's choice.
    disk_change: Option<DiskChange>,
    /// The note on screen was saved, and has not changed since.
    just_saved: bool,
    overwrite_disk: bool,
    scope: Scope,
    load_key: Option<LoadKey>,
    stale: bool,
    matches_scope: Option<Scope>,
    scope_counts: Option<(Scope, Vec<(Section, usize)>)>,
    confirm_unregister: Option<Module>,
    new_place: Option<usize>,
    /// A saved note's owner folder, so other pages can read it again.
    pub content_changed: Option<PathBuf>,
    /// A package's new name, for the registration.
    pub renamed: Option<(Uuid, String)>,
    /// The DM asked to go and add a campaign.
    pub wants_settings: bool,
    loaded: Option<Loaded>,
    pending: Option<mpsc::Receiver<Result<Loaded, String>>>,
    selected: Option<usize>,
    draft: String,
    search: String,
    matches: Option<(Section, String, Vec<usize>)>,
    section: Section,
    error: Option<String>,
    form: bool,
    path: String,
    name: String,
    namespace: String,
    new_name: String,
    new_kind: String,
    source: bool,
    message: Option<String>,
    reselect: Option<(Option<Uuid>, String)>,
    pub copy: Option<crate::content_workspace::CopySource>,
    /// An entry to read beside the open draft, in Lookup.
    pub peek: Option<Uuid>,
    history: Vec<BackStep>,
}
/// `pp. 35`, where a built-in entry is printed.
fn source_page(entry: &Entry) -> Option<String> {
    let sources = entry.fields.get("splat_sources")?.as_sequence()?;
    Some(sources.first()?.get("locator")?.as_str()?.to_owned())
}

/// Sort so that "Level 2" comes before "Level 18".
fn natural_key(name: &str) -> Vec<(String, u64)> {
    let lower = name.to_lowercase();
    let mut parts = Vec::new();
    let mut text = String::new();
    let mut chars = lower.chars().peekable();
    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            let mut number = u64::from(c.to_digit(10).unwrap_or(0));
            while let Some(digit) = chars.peek().and_then(|c| c.to_digit(10)) {
                number = number.saturating_mul(10).saturating_add(u64::from(digit));
                chars.next();
            }
            parts.push((std::mem::take(&mut text), number));
        } else {
            text.push(c);
        }
    }
    parts.push((text, 0));
    parts
}

const EDITOR_MAX_WIDTH: f32 = 720.0;
const NOTE_CHANGED: &str = "This note changed on disk.";
/// What is on disk now, in place of what the draft started from.
struct DiskChange {
    /// None when the file has gone.
    text: Option<String>,
    show: bool,
}
/// Where Back returns to after following a link.
struct BackStep {
    path: String,
    section: Section,
    search: String,
}
impl Library {
    #[cfg(test)]
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }

    /// The DM chose the file over the draft. The draft stays one step of undo away.
    fn reload_note_from_disk(&mut self, index: usize) {
        let Some(change) = self.disk_change.take() else {
            return;
        };
        let Some(note) = self.loaded.as_mut().and_then(|l| l.notes.get_mut(index)) else {
            return;
        };
        let draft = std::mem::take(&mut self.draft);
        self.discarded = Some((note.owner.id(), note.path.clone(), draft));
        match change.text {
            Some(text) => {
                match Entry::parse(&note.path, &text) {
                    Ok(entry) => {
                        note.entry = Some(entry);
                        note.error = None;
                    }
                    Err(error) => {
                        note.entry = None;
                        note.error = Some(error);
                    }
                }
                note.text = text.clone();
                self.draft = text;
            }
            None => self.draft = note.text.clone(),
        }
        if let Some(loaded) = &mut self.loaded {
            loaded.invalidate();
        }
        self.matches = None;
        self.scope_counts = None;
    }

    fn open_entry(&mut self, index: usize) {
        if self.dirty() {
            return;
        }
        let Some(note) = self.loaded.as_ref().and_then(|l| l.notes.get(index)) else {
            return;
        };
        if let Some(current) = self
            .selected
            .and_then(|i| self.loaded.as_ref().and_then(|l| l.notes.get(i)))
            .filter(|_| self.selected != Some(index))
        {
            self.history.push(BackStep {
                path: current.path.clone(),
                section: self.section,
                search: self.search.clone(),
            });
        }
        self.selected = Some(index);
        self.draft = note.text.clone();
        self.disk_change = None;
        self.section = note.section();
        self.search.clear();
        self.matches = None;
        self.source = false;
        self.error = None;
        self.message = None;
    }

    /// The open draft's entry name and owner, if it has changes.
    pub fn draft_place(&self) -> Option<(String, String)> {
        let note = self
            .loaded
            .as_ref()
            .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
            .filter(|n| n.text != self.draft)?;
        Some((note.name().to_owned(), note.owner.label()))
    }
    pub fn dirty(&self) -> bool {
        self.loaded
            .as_ref()
            .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
            .is_some_and(|n| n.text != self.draft)
    }
    pub fn form_open(&self) -> bool {
        self.form || self.confirm_unregister.is_some()
    }
    /// A draft in the campaign's own folder must block a campaign switch.
    pub fn campaign_draft(&self) -> bool {
        self.dirty()
            && self
                .loaded
                .as_ref()
                .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
                .is_some_and(|n| matches!(n.owner, Owner::Campaign(_)))
    }
    /// Show an entry where it lives. False if a draft is open: it stays in Lookup.
    pub fn reveal(&mut self, entry: &Entry) -> bool {
        if self.dirty() {
            return false;
        }
        self.reveal_row = true;
        let Some((index, in_scope)) = self.loaded.as_ref().and_then(|l| {
            let index = l
                .notes
                .iter()
                .position(|n| n.entry.as_ref().is_some_and(|e| e.id == entry.id))?;
            Some((index, l.in_scope(&l.notes[index], self.scope)))
        }) else {
            return false;
        };
        if !in_scope {
            self.scope = Scope::All;
        }
        self.search.clear();
        self.section = Section::for_entry(entry);
        self.source = false;
        self.matches = None;
        self.selected = Some(index);
        self.draft = self.loaded.as_ref().unwrap().notes[index].text.clone();
        true
    }
    pub fn refresh_destination(&mut self, _root: &Path, _ctx: &egui::Context) {
        self.stale = true;
    }
    fn invalidate(&mut self) {
        self.matches = None;
        self.scope_counts = None;
    }
    fn start_load(&mut self, ctx: &egui::Context, campaigns: &Campaigns) {
        self.reselect = self
            .loaded
            .as_ref()
            .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
            .map(|n| (n.owner.id(), n.path.clone()));
        self.message = None;
        self.load_key = Some(load_key(campaigns));
        let modules = campaigns.modules().to_vec();
        let campaign = campaigns.active_campaign();
        let ctx = ctx.clone();
        let (tx, rx) = mpsc::channel();
        self.pending = Some(rx);
        self.loaded = None;
        self.invalidate();
        self.selected = None;
        self.error = None;
        std::thread::spawn(move || {
            let _ = tx.send(load_all(&modules, campaign.as_ref()));
            ctx.request_repaint();
        });
    }
    pub fn show(&mut self, ui: &mut egui::Ui, campaigns: &mut Campaigns) {
        let key = load_key(campaigns);
        if self.load_key.is_none() && self.loaded.is_some() {
            self.load_key = Some(key.clone());
        }
        let changed = self.load_key.as_ref().is_some_and(|known| *known != key);
        if self.loaded.is_none() && self.pending.is_none() && self.error.is_none() {
            self.start_load(ui.ctx(), campaigns);
        } else if (changed || self.stale) && !self.dirty() && self.pending.is_none() {
            self.stale = false;
            self.start_load(ui.ctx(), campaigns);
        }
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(result) => {
                    self.pending = None;
                    match result {
                        Ok(l) => {
                            self.selected = self.reselect.as_ref().and_then(|(owner, path)| {
                                l.notes
                                    .iter()
                                    .position(|n| n.owner.id() == *owner && &n.path == path)
                            });
                            if let Some(i) = self.selected {
                                self.draft = l.notes[i].text.clone();
                            }
                            self.loaded = Some(l);
                            self.invalidate();
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    self.pending = None;
                    self.error = Some("Library loading stopped. Reload to retry.".into());
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        let dirty = self.dirty();
        let locked =
            (dirty || self.pending.is_some()).then_some("Save or discard this entry first.");
        // Beside Lookup at the minimum window, everything here gets smaller.
        let compact = ui.available_width() < 640.0;
        let mut open_form = false;
        let mut reload = false;
        ui.horizontal(|ui| {
            ui.set_min_height(Density::Scan.control());
            ui.label(Type::Title.rich(ui, "Library").color(colors().text));
            ui.add_space(design::M);
            // Beside what it changes, not across the page from it.
            self.scope_menu(ui, compact, campaigns);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                design::locked(ui, locked, |ui| {
                    ui.menu_button("Packages", |ui| {
                        if ui.button("Reload the Library").clicked() {
                            reload = true;
                            ui.close();
                        }
                        if ui.button("Manage packages…").clicked() {
                            open_form = true;
                            ui.close();
                        }
                    })
                    .response
                    .on_disabled_hover_text(locked.unwrap_or_default());
                });
            });
        });
        design::rule(ui);
        if reload {
            self.start_load(ui.ctx(), campaigns);
        }
        if open_form {
            self.form = !self.form;
        }
        if campaigns.active_campaign().is_none() {
            ui.horizontal(|ui| {
                // Level with the action beside it. A message aligns to the top of its row.
                ui.vertical(|ui| {
                    ui.add_space((Density::Read.control() - Type::Body.line()) / 2.0);
                    design::message(
                        ui,
                        Tone::Info,
                        "No campaign yet. Next step: add one in Settings.",
                    );
                });
                if Action::primary("Open Settings")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.wants_settings = true;
                }
            });
        }
        ui.add_space(design::XS);
        self.unregister_dialog(ui.ctx(), campaigns);
        if let Some(e) = &self.error {
            design::message(ui, Tone::Error, &format!("Error: {e}"));
        }
        if let Some(message) = &self.message {
            design::message(ui, Tone::Success, message);
        }
        if self.pending.is_some() {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
                ui.label("Loading the Library…");
            });
            return;
        }

        self.new_entry(ui, dirty, campaigns);
        let Some(loaded) = &self.loaded else {
            return;
        };
        if !loaded.diagnostics.is_empty() {
            design::section(
                ui,
                format!("{} validation messages", loaded.diagnostics.len()),
                |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(140.0)
                        .show(ui, |ui| {
                            for message in &loaded.diagnostics {
                                ui.add(
                                    egui::Label::new(
                                        Type::Small.rich(ui, message).color(colors().muted),
                                    )
                                    .wrap(),
                                );
                            }
                        });
                    ui.label(
                        Type::Small
                            .rich(ui, "A reference to another package can stay unresolved in a check of one package.")
                            .color(colors().muted),
                    );
                },
            );
        }
        // Each scope has its own counts. Search and sections stay within the scope.
        let scope = self.scope;
        if self
            .scope_counts
            .as_ref()
            .is_none_or(|(known, _)| *known != scope)
        {
            let notes = &loaded.notes;
            let counts = Section::GROUPS
                .iter()
                .flat_map(|(_, sections)| sections.iter())
                .map(|&section| {
                    (
                        section,
                        notes
                            .iter()
                            .filter(|n| {
                                loaded.in_scope(n, scope)
                                    && (section == Section::All || n.section() == section)
                            })
                            .count(),
                    )
                })
                .collect();
            self.scope_counts = Some((scope, counts));
        }
        let count_of = |section: Section, counts: &Vec<(Section, usize)>| {
            counts
                .iter()
                .find(|(s, _)| *s == section)
                .map_or(0, |(_, n)| *n)
        };
        let counts = self
            .scope_counts
            .as_ref()
            .map(|(_, counts)| counts.clone())
            .unwrap_or_default();
        let searching = !self.search.trim().is_empty();
        if !compact {
            design::Column::left("library_sections", 166.0, 150.0..=280.0)
                // The drag handle is on this edge, so the rows stop short of it.
                .frame(egui::Frame::new().inner_margin(egui::Margin {
                    right: design::S as i8,
                    ..Default::default()
                }))
                .show_inside(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("section_list")
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            for &(group, sections) in Section::GROUPS {
                                // Keep the empty encounter section discoverable, but hide an unused fallback.
                                if sections == [Section::Other]
                                    && count_of(Section::Other, &counts) == 0
                                {
                                    continue;
                                }
                                if !group.is_empty() {
                                    ui.add_space(design::M);
                                    design::group_label(ui, group);
                                    ui.add_space(design::XS);
                                }
                                for &section in sections {
                                    let count = count_of(section, &counts);
                                    let selected = self.section == section && !searching;
                                    let label = section.label();
                                    let response = design::row(
                                        ui,
                                        Density::Read.row(),
                                        selected,
                                        &format!("{label}   {count}"),
                                        |ui, rect| {
                                            let c = colors();
                                            let font =
                                                if selected { Type::Strong } else { Type::Body };
                                            let galley = design::one_line(
                                                ui,
                                                label,
                                                font.font(ui),
                                                if selected { c.text } else { c.muted },
                                                rect.width() - 44.0,
                                            );
                                            ui.painter().galley(
                                                egui::pos2(
                                                    rect.left() + design::S,
                                                    rect.center().y - galley.size().y / 2.0,
                                                ),
                                                galley,
                                                c.text,
                                            );
                                            design::number(
                                                ui.painter(),
                                                rect.right() - design::S,
                                                rect.center().y,
                                                &count.to_string(),
                                                &Type::Small.font(ui),
                                                c.muted,
                                            );
                                        },
                                    );
                                    if response.clicked() {
                                        self.section = section;
                                        self.search.clear();
                                    }
                                }
                            }
                        });
                });
        }
        // A query always searches the whole scope. Clearing it returns to the selected section.
        let query = self.search.trim().to_lowercase();
        let searching = !query.is_empty();
        if self.matches_scope != Some(scope)
            || self
                .matches
                .as_ref()
                .is_none_or(|(section, cached_query, _)| {
                    *section != self.section || *cached_query != query
                })
        {
            self.matches_scope = Some(scope);
            let mut found: Vec<usize> = loaded
                .notes
                .iter()
                .enumerate()
                .filter(|(_, n)| loaded.in_scope(n, scope) && n.matches(self.section, &query))
                .map(|(i, _)| i)
                .collect();
            if !query.is_empty() {
                // Best first: the exact name, then a prefix, then a whole word.
                found.sort_by_key(|i| {
                    let name = loaded.notes[*i].name().to_lowercase();
                    if name == query {
                        0
                    } else if name.starts_with(&query) {
                        1
                    } else if crate::lookup::names_word(&name, &query) {
                        2
                    } else {
                        3
                    }
                });
            }
            self.matches = Some((self.section, query.clone(), found));
        }
        let matches = &self.matches.as_ref().unwrap().2;
        self.list_is_empty = matches.is_empty();
        // Keep the reader in sync with the visible list without dropping an unsaved draft.
        let mut select = None;
        // The whole catalogue has no first entry worth opening for the DM.
        let whole_catalogue = self.section == Section::All && query.is_empty();
        if !dirty && self.selected.is_none_or(|i| !matches.contains(&i)) {
            select = matches.first().copied().filter(|_| !whole_catalogue);
            if select.is_none() {
                self.selected = None;
                self.draft.clear();
            }
        }
        let selected = select.or(self.selected);
        let subtitles = loaded.subtitles();
        // A wide window's list gets room too, so that long names keep their second line.
        let list_width = (ui.available_width() * 0.38).clamp(190.0, 420.0);
        let mut peek = None;
        // Beside Lookup at the minimum window, a draft needs the room more than the list.
        let hide_list = ui.available_width() < 480.0 && (dirty || self.source);
        if !hide_list {
            design::Column::left("library_entries", list_width, 190.0..=560.0)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::S as i8,
                right: design::S as i8,
                top: 0,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    let width =
                        ui.available_width() - if self.search.is_empty() { 0.0 } else { 72.0 };
                    design::search_field(
                        ui,
                        SEARCH_NAME,
                        &mut self.search,
                        "Search by name…",
                        width,
                    );
                    if !self.search.is_empty()
                        && Action::quiet("Clear")
                            .density(Density::Scan)
                            .show(ui)
                            .clicked()
                    {
                        self.search.clear();
                    }
                });
                if compact {
                    ui.add_space(design::XS);
                    let combo = egui::ComboBox::from_id_salt("library_section")
                        .icon(design::combo_icon)
                        .width(ui.available_width().min(200.0))
                        .selected_text(self.section.label())
                        .show_ui(ui, |ui| {
                            for &(_, sections) in Section::GROUPS {
                                for &section in sections {
                                    if ui
                                        .selectable_label(self.section == section, section.label())
                                        .clicked()
                                    {
                                        self.section = section;
                                        self.search.clear();
                                    }
                                }
                            }
                        });
                    design::named(ui, &combo.response, "Section");
                }
                ui.add_space(design::XS);
                ui.horizontal(|ui| {
                    ui.label(
                        Type::Heading
                            .rich(
                                ui,
                                if searching {
                                    "Search results"
                                } else {
                                    self.section.label()
                                },
                            )
                            .color(colors().text),
                    );
                    ui.label(
                        Type::Small
                            .rich(
                                ui,
                                design::count(matches.len(), "entry", "entries"),
                            )
                            .color(colors().muted),
                    );
                });
                design::rule(ui);
                if matches.is_empty() {
                    ui.add_space(design::S);
                    let text = if scope == Scope::Campaign && !searching {
                        "No campaign entries yet. Copy an entry here with Copy to…, or create one with New entry."
                    } else if searching {
                        "No entries found in this scope. Try fewer words, or Everything."
                    } else {
                        "No entries in this section."
                    };
                    ui.add(
                        egui::Label::new(Type::Body.rich(ui, text).color(colors().muted)).wrap(),
                    );
                }
                ui.spacing_mut().item_spacing.y = 0.0;
                let mut list = egui::ScrollArea::vertical()
                    .id_salt(("entry_list", self.section.label(), &query, scope))
                    .auto_shrink([false, false]);
                if std::mem::take(&mut self.reveal_row)
                    && let Some(row) =
                        selected.and_then(|selected| matches.iter().position(|i| *i == selected))
                {
                    list = list.vertical_scroll_offset(
                        (row as f32 - 2.0).max(0.0) * ENTRY_ROW_HEIGHT,
                    );
                }
                list.show_rows(ui, ENTRY_ROW_HEIGHT, matches.len(), |ui, range| {
                        for row in range {
                            let i = matches[row];
                            let note = &loaded.notes[i];
                            let subtitle = &subtitles[i];
                            let response = ui
                                .push_id(i, |ui| {
                                    entry_row(ui, note.name(), subtitle, selected == Some(i))
                                })
                                .inner;
                            if response
                                .on_hover_text(format!(
                                    "{}\n{}\n{}",
                                    note.name(),
                                    subtitle,
                                    note.path
                                ))
                                .clicked()
                            {
                                if dirty {
                                    peek = note.entry.as_ref().map(|e| e.id);
                                } else {
                                    select = Some(i);
                                }
                            }
                        }
                    });
            });
        }
        if peek.is_some() {
            self.peek = peek;
        }
        if let Some(i) = select {
            self.selected = Some(i);
            self.draft = self.loaded.as_ref().unwrap().notes[i].text.clone();
            self.error = None;
            self.source = false;
        }
        let subtitle = self
            .selected
            .map(|i| subtitles[i].clone())
            .unwrap_or_default();
        egui::CentralPanel::default()
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::L as i8,
                right: 0,
                top: 0,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                if self.form {
                    self.package_form(ui, campaigns);
                } else {
                    self.editor(ui, &subtitle);
                }
            });
    }

    /// What the catalogue shows. Choosing here never changes the active campaign.
    fn scope_menu(&mut self, ui: &mut egui::Ui, compact: bool, campaigns: &Campaigns) {
        let active = campaigns.active_campaign();
        let label = |scope: Scope| -> String {
            match scope {
                Scope::Uses => "What this campaign can use".to_owned(),
                Scope::All => "Everything".to_owned(),
                Scope::Builtin => BUILTIN_NAME.to_owned(),
                Scope::Package(id) => campaigns
                    .modules()
                    .iter()
                    .find(|m| m.id == id)
                    .map_or_else(|| "A package".to_owned(), |m| m.name.clone()),
                Scope::Campaign => active.as_ref().map_or_else(
                    || "This campaign".to_owned(),
                    |c| format!("Own entries · {}", c.name),
                ),
            }
        };
        let mut scope = if active.is_none() && matches!(self.scope, Scope::Uses | Scope::Campaign) {
            Scope::All
        } else {
            self.scope
        };
        let before = scope;
        let combo = egui::ComboBox::from_id_salt("library_scope")
            .icon(design::combo_icon)
            .width(if compact { 150.0 } else { 240.0 })
            .truncate()
            .selected_text(label(scope))
            .show_ui(ui, |ui| {
                ui.set_min_width(260.0);
                if active.is_some() {
                    ui.selectable_value(&mut scope, Scope::Uses, label(Scope::Uses));
                }
                ui.selectable_value(&mut scope, Scope::All, label(Scope::All));
                ui.separator();
                ui.selectable_value(&mut scope, Scope::Builtin, label(Scope::Builtin));
                for m in campaigns.modules() {
                    ui.selectable_value(
                        &mut scope,
                        Scope::Package(m.id),
                        label(Scope::Package(m.id)),
                    );
                }
                if active.is_some() {
                    ui.selectable_value(&mut scope, Scope::Campaign, label(Scope::Campaign));
                }
            });
        design::named(ui, &combo.response, "What the Library shows");
        self.scope = scope;
        if scope != before {
            // The selected entry can stay if it is in this scope. A draft always stays.
            self.search.clear();
            self.section = Section::All;
        }
    }

    /// Show the catalogue limited to this campaign's own entries.
    pub fn show_campaign_entries(&mut self) {
        self.scope = Scope::Campaign;
        self.search.clear();
        self.section = Section::All;
    }

    fn unregister_dialog(&mut self, ctx: &egui::Context, campaigns: &mut Campaigns) {
        let Some(module) = self.confirm_unregister.clone() else {
            return;
        };
        let count = self.loaded.as_ref().map_or(0, |l| {
            l.notes
                .iter()
                .filter(|n| n.owner.id() == Some(module.id))
                .count()
        });
        let mut keep = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let mut remove = false;
        let title = format!("Unregister {}?", module.name);
        design::dialog(ctx, "unregister_package", &title, |ui| {
            ui.add(
                egui::Label::new(format!(
                    "{count} entries leave Splat's lists, and any campaign that selects them can no longer use them."
                ))
                .wrap(),
            );
            ui.add(
                egui::Label::new(
                    Type::Small
                        .rich(ui, format!("The files stay in {}.", module.path.display()))
                        .color(colors().muted),
                )
                .wrap(),
            );
            ui.add_space(design::S);
            ui.horizontal(|ui| {
                keep |= Action::secondary("Keep the package").show(ui).clicked();
                ui.add_space(design::L);
                remove = Action::destructive("Unregister").show(ui).clicked();
            });
        });
        if keep {
            self.confirm_unregister = None;
        }
        if remove {
            self.confirm_unregister = None;
            match campaigns.unregister_module(module.id) {
                Ok(()) => {
                    self.scope = Scope::default();
                    self.stale = true;
                }
                Err(e) => self.error = Some(e),
            }
        }
    }

    /// Registering a package is a form in the page, not a window over it.
    fn package_form(&mut self, ui: &mut egui::Ui, campaigns: &mut Campaigns) {
        let c = colors();
        let dirty = self.dirty();
        let locked =
            (dirty || self.pending.is_some()).then_some("Save or discard this entry first.");
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            self.form = false;
        }
        egui::ScrollArea::vertical()
            .id_salt("package_form")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
                ui.horizontal(|ui| {
                    ui.label(Type::Section.rich(ui, "Manage packages").color(c.text));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if Action::quiet("Close")
                            .density(Density::Read)
                            .name("Close the package form")
                            .show(ui)
                            .clicked()
                        {
                            self.form = false;
                        }
                    });
                });
                ui.label("Register an existing package folder, or create a new package.");
                design::text_field(ui, "Absolute folder path", &mut self.path, None, None);
                design::group_label(ui, "For a new package only");
                design::text_field(ui, "Name", &mut self.name, None, None);
                design::text_field(ui, "Namespace", &mut self.namespace, None, None);
                ui.horizontal_wrapped(|ui| {
                    let no_path = self.path.trim().is_empty();
                    let register = Action::secondary("Register folder")
                        .blocked(locked.or(no_path.then_some("Enter the folder's path first.")))
                        .show(ui)
                        .clicked();
                    let create = Action::primary("Create package")
                        .blocked(
                            locked.or((no_path
                                || self.name.trim().is_empty()
                                || self.namespace.trim().is_empty())
                            .then_some("Enter a path, a name and a namespace first.")),
                        )
                        .show(ui)
                        .clicked();
                    if register || create {
                        let result = if create {
                            create_module(
                                Path::new(self.path.trim()),
                                &self.name,
                                self.namespace.trim(),
                            )
                        } else {
                            local_module(Path::new(self.path.trim()))
                        };
                        match result.and_then(|m| {
                            campaigns.register_module(m.clone())?;
                            Ok(m)
                        }) {
                            Ok(m) => {
                                self.scope = Scope::Package(m.id);
                                self.form = false;
                                self.search.clear();
                                self.section = Section::All;
                                self.stale = true;
                            }
                            Err(e) => self.error = Some(e),
                        }
                    }
                });
                ui.add_space(design::M);
                design::group_label(ui, "Registered packages");
                design::rule(ui);
                if campaigns.modules().is_empty() {
                    ui.label(Type::Body.rich(ui, "None yet.").color(c.muted));
                }
                for module in campaigns.modules().to_vec() {
                    ui.push_id(module.id, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                ui.set_max_width((ui.available_width() - 180.0).max(120.0));
                                ui.label(Type::Strong.rich(ui, &module.name).color(c.text));
                                ui.add(
                                    egui::Label::new(
                                        Type::Small
                                            .rich(ui, module.path.display().to_string())
                                            .color(c.muted),
                                    )
                                    .wrap(),
                                );
                            });
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if Action::destructive("Unregister…")
                                        .density(Density::Read)
                                        .name(&format!("Unregister {}", module.name))
                                        .blocked(locked)
                                        .show(ui)
                                        .clicked()
                                    {
                                        self.confirm_unregister = Some(module.clone());
                                    }
                                },
                            );
                        });
                    });
                    design::rule(ui);
                }
            });
    }

    fn new_entry(&mut self, ui: &mut egui::Ui, dirty: bool, campaigns: &Campaigns) {
        if self.loaded.is_none() {
            return;
        }
        // Where a new entry can go: the campaign, or a package. Never the built-in rules.
        let mut places: Vec<Owner> = campaigns
            .modules()
            .iter()
            .cloned()
            .map(Owner::Package)
            .collect();
        if let Some(campaign) = campaigns.active_campaign() {
            places.insert(0, Owner::Campaign(campaign));
        }
        if places.is_empty() {
            return;
        }
        let in_scope = places.iter().position(|owner| match self.scope {
            Scope::Package(id) => owner.id() == Some(id),
            Scope::Campaign => matches!(owner, Owner::Campaign(_)),
            _ => false,
        });
        let default_place = in_scope.unwrap_or(0);
        let mut place = self
            .new_place
            .filter(|place| *place < places.len())
            .unwrap_or(default_place);
        design::section(ui, "New entry", |ui| {
            ui.label(
                Type::Small
                    .rich(
                        ui,
                        "Create a draft note, then add its rules in the Markdown editor.",
                    )
                    .color(colors().muted),
            );
            ui.horizontal_wrapped(|ui| {
                design::text_input(ui, "Entry name", &mut self.new_name, "Entry name", 180.0);
                if self.new_kind.is_empty() {
                    self.new_kind = "creature".into();
                }
                let combo = egui::ComboBox::from_id_salt("new_kind")
                    .icon(design::combo_icon)
                    .selected_text(&self.new_kind)
                    .show_ui(ui, |ui| {
                        for kind in ["creature", "spell", "item", "effect", "rule", "rule_group"] {
                            ui.selectable_value(&mut self.new_kind, kind.into(), kind);
                        }
                    });
                design::named(ui, &combo.response, "Kind of entry");
                let combo = egui::ComboBox::from_id_salt("new_place")
                    .icon(design::combo_icon)
                    .width(150.0)
                    .truncate()
                    .selected_text(places[place].label())
                    .show_ui(ui, |ui| {
                        for (i, owner) in places.iter().enumerate() {
                            ui.selectable_value(&mut place, i, owner.label());
                        }
                    });
                design::named(ui, &combo.response, "Where the entry goes");
                let blocked = if dirty {
                    Some("Save or discard this entry first.")
                } else if self.new_name.trim().is_empty() {
                    Some("Enter a name first.")
                } else {
                    None
                };
                if Action::secondary("Create entry")
                    .blocked(blocked)
                    .show(ui)
                    .clicked()
                {
                    let name = self.new_name.trim();
                    if name.contains(['/', '\\', '\n', '\r']) || name == "." || name == ".." {
                        self.error =
                            Some("Use a name without path separators or line breaks.".into());
                        return;
                    }
                    let path = format!("{name}.md");
                    let text = note_text(name, &self.new_kind, None);
                    let owner = places[place].clone();
                    let root = owner.root().expect("a writable owner").to_path_buf();
                    match write_note(&root, &path, None, &text) {
                        Ok(entry) => {
                            let l = self.loaded.as_mut().unwrap();
                            let created_path = path.clone();
                            let created_owner = owner.id();
                            l.notes.push(Note {
                                path,
                                text: text.clone(),
                                entry: Some(entry),
                                error: None,
                                owner,
                            });
                            l.notes.sort_by_cached_key(|n| natural_key(n.name()));
                            l.invalidate();
                            self.matches = None;
                            self.scope_counts = None;
                            self.selected = l.notes.iter().position(|n| {
                                n.path == created_path && n.owner.id() == created_owner
                            });
                            self.search.clear();
                            self.section = Section::All;
                            self.source = true;
                            self.draft = text;
                            self.new_name.clear();
                            self.error = None;
                            // Show the new entry, wherever the DM chose to put it.
                            if self
                                .selected
                                .is_some_and(|i| !l.in_scope(&l.notes[i], self.scope))
                            {
                                self.scope = Scope::All;
                            }
                            self.content_changed = Some(root);
                        }
                        Err(e) => self.error = Some(e),
                    }
                }
            });
        });
        self.new_place = (place != default_place).then_some(place);
    }

    fn go_back(&mut self) {
        let Some(step) = self.history.pop() else {
            return;
        };
        let Some(index) = self
            .loaded
            .as_ref()
            .and_then(|l| l.notes.iter().position(|n| n.path == step.path))
        else {
            return;
        };
        self.selected = Some(index);
        self.reveal_row = true;
        self.draft = self.loaded.as_ref().unwrap().notes[index].text.clone();
        self.section = step.section;
        self.search = step.search;
        self.matches = None;
        self.source = false;
        self.error = None;
        self.message = None;
    }

    /// In a wide window, the reader and its margin sit together in the pane, with the scroll
    /// bar beside them rather than across an empty band.
    fn editor(&mut self, ui: &mut egui::Ui, subtitle: &str) {
        let spare = ui.available_width() - view::WIDE_READER;
        if spare <= 0.0 {
            self.editor_pane(ui, subtitle);
            return;
        }
        ui.horizontal_top(|ui| {
            ui.add_space(spare / 2.0);
            let size = egui::vec2(view::WIDE_READER, ui.available_height());
            ui.allocate_ui_with_layout(size, egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_max_width(view::WIDE_READER);
                self.editor_pane(ui, subtitle);
            });
        });
    }

    fn editor_pane(&mut self, ui: &mut egui::Ui, subtitle: &str) {
        let Some(i) = self.selected else {
            ui.label(
                Type::Body
                    .rich(
                        ui,
                        if self.list_is_empty {
                            "Nothing to read here yet."
                        } else {
                            "Choose an entry from the list to read it."
                        },
                    )
                    .color(colors().muted),
            );
            return;
        };
        // Overwrite was chosen: what is on disk now becomes what this save replaces.
        let overwrite = std::mem::take(&mut self.overwrite_disk);
        let mut file_gone = false;
        if overwrite && let Some(change) = self.disk_change.take() {
            match change.text {
                Some(text) => self.loaded.as_mut().unwrap().notes[i].text = text,
                None => file_gone = true,
            }
        }
        let note = &self.loaded.as_ref().unwrap().notes[i];
        let parsed = Entry::parse(&note.path, &self.draft);
        let reading = parsed
            .as_ref()
            .ok()
            .and_then(|entry| entry.reading().ok().flatten());
        let title = reading
            .as_ref()
            .map_or(note.name(), |reading| reading.title.as_str());
        let reading_subtitle = reading
            .as_ref()
            .and_then(|_| presentation::class_level(parsed.as_ref().ok()?));
        let dirty = self.draft != note.text;
        if dirty {
            self.message = None;
        }
        let editable = note.owner.editable();
        let conflict = self.disk_change.is_some();
        self.just_saved &= !dirty && self.source;
        let mut save = overwrite
            || (dirty
                && editable
                && !conflict
                && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)));
        let d = colors().document;
        let state = if note.error.is_some() {
            Status {
                icon: icons::ERROR,
                word: "Invalid",
                color: d.invalid,
            }
        } else if conflict {
            Status {
                icon: icons::WARN,
                word: "Changed on disk",
                color: d.external,
            }
        } else if dirty {
            Status {
                icon: icons::DRAFT,
                word: "Unsaved",
                color: d.unsaved,
            }
        } else if self.just_saved {
            // In the status slot beside Save, so that saving moves nothing.
            Status {
                icon: icons::OK,
                word: "Saved",
                color: d.saved,
            }
        } else if self.source && !matches!(note.owner, Owner::Builtin) {
            Status {
                icon: icons::OK,
                word: "No changes",
                color: d.clean,
            }
        } else {
            match &note.owner {
                Owner::Builtin => Status {
                    icon: icons::READ_ONLY,
                    word: "Built-in · Read only",
                    color: d.read_only,
                },
                Owner::Package(_) => Status {
                    icon: icons::EDIT,
                    word: "Local package",
                    color: d.clean,
                },
                Owner::Campaign(_) => Status {
                    icon: icons::EDIT,
                    word: "Campaign-owned",
                    color: d.clean,
                },
            }
        };
        let save_row = editable && (self.source || dirty);
        let mut go_back = false;
        ui.horizontal_wrapped(|ui| {
            // Back keeps its place, so the tabs beside it do not move.
            if Action::quiet("Back")
                .icon(icons::BACK)
                .density(Density::Read)
                .hint("Return to the entry before this link")
                .blocked_if(self.history.is_empty(), "No earlier entry to return to.")
                .show(ui)
                .clicked()
            {
                go_back = true;
            }
            let edit_label = if editable {
                "Edit Markdown"
            } else {
                "Markdown source"
            };
            design::segmented(ui, &mut self.source, &[(false, "Read"), (true, edit_label)]);
            if note
                .entry
                .as_ref()
                .is_some_and(|e| !["package", "campaign", "reference"].contains(&e.kind.as_str()))
                && Action::quiet("Copy to…")
                    .icon(icons::COPY)
                    .density(Density::Read)
                    .blocked_if(dirty, "Save or discard this entry before you copy it.")
                    .show(ui)
                    .clicked()
            {
                self.copy = Some(crate::content_workspace::CopySource {
                    entry: note.entry.as_ref().unwrap().clone(),
                    text: note.text.clone(),
                    root: note.owner.root().map(Path::to_path_buf),
                    campaign: match &note.owner {
                        Owner::Campaign(campaign) => Some(campaign.clone()),
                        _ => None,
                    },
                });
            }
            if !save_row {
                design::status(ui, state);
            }
        });
        // Save and Discard are one row, as far apart as the pane allows.
        if save_row {
            ui.horizontal(|ui| {
                // Its right edge is the editor's, so Discard lines up with something.
                ui.set_max_width(ui.available_width().min(EDITOR_MAX_WIDTH));
                let blocked = if conflict {
                    Some("Choose what to do with the changed file first.")
                } else {
                    (!dirty).then_some("No changes to save.")
                };
                save |= Action::primary("Save")
                    .density(Density::Read)
                    .blocked(blocked)
                    .key("Ctrl+S")
                    .show(ui)
                    .clicked();
                design::status(ui, state);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if dirty
                        && !conflict
                        && Action::destructive("Discard changes")
                            .density(Density::Read)
                            .show(ui)
                            .clicked()
                    {
                        self.discarded =
                            Some((note.owner.id(), note.path.clone(), self.draft.clone()));
                        self.draft = note.text.clone();
                        self.error = None;
                    }
                });
            });
        }
        // Live, because Discard in this same frame changed the draft.
        if self.draft != note.text {
            self.discarded = None;
        }
        if discard_is_for(self.discarded.as_ref(), note)
            && design::draft_discarded(ui)
            && let Some((_, _, text)) = self.discarded.take()
        {
            self.draft = text;
        }
        let mut reload_from_disk = false;
        if let Some(change) = &mut self.disk_change {
            design::message(
                ui,
                Tone::Warning,
                if change.text.is_some() {
                    "This note changed on disk. Your draft is safe here until you choose."
                } else {
                    "This note's file has gone from disk. Your draft is safe here until you choose."
                },
            );
            ui.horizontal_wrapped(|ui| {
                if change.text.is_some() {
                    let word = if change.show {
                        "Hide disk version"
                    } else {
                        "Show disk version"
                    };
                    // The safe look comes first, and overwriting says what it is.
                    if Action::primary(word)
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                    {
                        change.show = !change.show;
                    }
                }
                ui.add_space(design::XL);
                if Action::destructive("Overwrite with my draft")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.overwrite_disk = true;
                }
                reload_from_disk = Action::destructive("Reload and lose my draft")
                    .density(Density::Read)
                    .show(ui)
                    .clicked();
            });
            if change.show
                && let Some(text) = &change.text
            {
                egui::ScrollArea::vertical()
                    .id_salt("disk_version")
                    .max_height(180.0)
                    .show(ui, |ui| {
                        ui.label(Type::Mono.rich(ui, text).color(colors().muted));
                    });
                design::rule(ui);
            }
        }
        if let Some(error) = &note.error {
            design::message(ui, Tone::Error, error);
        }
        let mut open_link = None;
        egui::ScrollArea::vertical()
            .id_salt(("entry_body", note.owner.id(), &note.path))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if self.source {
                    let response = ui.add(
                        egui::TextEdit::multiline(&mut self.draft)
                            .font(egui::TextStyle::Monospace)
                            .margin(egui::vec2(design::S, design::S))
                            .desired_width(ui.available_width().min(EDITOR_MAX_WIDTH))
                            .min_size(egui::vec2(0.0, ui.available_height()))
                            .interactive(editable),
                    );
                    design::named(ui, &response, "Markdown editor");
                } else {
                    let body = parsed.as_ref().map_or(self.draft.as_str(), |e| &e.body);
                    ui.push_id(&note.path, |ui| {
                        reader_style::measure(ui, |ui| {
                            ui.add_space(design::XS);
                            ui.label(
                                Type::Small
                                    .rich(ui, format!("Library / {}", note.section().label()))
                                    .color(colors().muted),
                            );
                            reader_style::title(ui, title);
                            if note.entry.is_some() {
                                reader_style::subtitle(
                                    ui,
                                    reading_subtitle.as_deref().unwrap_or(subtitle),
                                );
                            }
                        });
                        open_link = view::show(
                            ui,
                            note.name(),
                            parsed.as_ref().ok(),
                            body,
                            &view::Origin {
                                path: &note.path,
                                owner: &note.owner.label(),
                                builtin: note.owner == Owner::Builtin,
                            },
                            self.loaded.as_ref().unwrap(),
                        );
                    });
                }
            });
        if reload_from_disk {
            self.reload_note_from_disk(i);
            return;
        }
        if save {
            let root = note.owner.root().expect("an editable note").to_path_buf();
            let owner_is_campaign = matches!(note.owner, Owner::Campaign(_));
            let result = Entry::parse(&note.path, &self.draft).and_then(|entry| {
                if let Some(old) = &note.entry {
                    if entry.id != old.id || entry.kind != old.kind {
                        return Err("Keep this entry's UUID and kind unchanged.".into());
                    }
                    if owner_is_campaign && entry.key != old.key {
                        return Err("Keep the entry's UUID, kind and key unchanged.".into());
                    }
                    if old.kind == "package"
                        && entry.fields.get("splat_namespace") != old.fields.get("splat_namespace")
                    {
                        return Err("Keep the package namespace unchanged.".into());
                    }
                }
                if self
                    .loaded
                    .as_ref()
                    .unwrap()
                    .notes
                    .iter()
                    .enumerate()
                    .any(|(other, n)| {
                        other != i
                            && n.entry.as_ref().is_some_and(|e| {
                                e.id == entry.id || (entry.key.is_some() && e.key == entry.key)
                            })
                    })
                {
                    return Err("Another entry already uses this UUID or key.".into());
                }
                let baseline = (!file_gone).then_some(note.text.as_str());
                write_note(&root, &note.path, baseline, &self.draft)
            });
            match result {
                Ok(entry) => {
                    let l = self.loaded.as_mut().unwrap();
                    if entry.kind == "package"
                        && let Owner::Package(module) = &mut l.notes[i].owner
                    {
                        self.renamed = Some((module.id, entry.name.clone()));
                        module.name = entry.name.clone();
                    }
                    l.notes[i].entry = Some(entry);
                    l.notes[i].text = self.draft.clone();
                    l.notes[i].error = None;
                    l.invalidate();
                    self.matches = None;
                    self.scope_counts = None;
                    self.just_saved = true;
                    self.content_changed = Some(root);
                    let entries: Vec<_> = l.notes.iter().filter_map(|n| n.entry.clone()).collect();
                    let r = content::validate(&entries);
                    l.diagnostics = r
                        .errors
                        .into_iter()
                        .chain(r.warnings)
                        .map(|d| format!("{}: {}", d.path, d.message))
                        .collect();
                    l.diagnostics.extend(
                        l.notes
                            .iter()
                            .filter_map(|n| n.error.as_ref().map(|e| format!("{}: {e}", n.path))),
                    );
                    self.error = None;
                }
                Err(e) if e == NOTE_CHANGED => {
                    self.disk_change = Some(DiskChange {
                        text: note_path(&root, &note.path)
                            .ok()
                            .and_then(|path| fs::read_to_string(path).ok()),
                        show: false,
                    });
                }
                Err(e) => self.error = Some(e),
            }
        }
        if let Some(index) = open_link {
            if dirty {
                // Keep the draft. The linked entry opens beside it, in Lookup.
                self.peek = self
                    .loaded
                    .as_ref()
                    .and_then(|l| l.notes.get(index))
                    .and_then(|n| n.entry.as_ref())
                    .map(|e| e.id);
            } else {
                self.open_entry(index);
                // Followed from the text, so the list may not show it yet.
                self.reveal_row = true;
            }
        }
        if go_back {
            self.go_back();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Run alone so the first load starts without a warmed process cache.
    #[test]
    #[ignore]
    fn srd_library_load_time() {
        for run in 0..3 {
            let start = std::time::Instant::now();
            let loaded = load(None).unwrap();
            assert_eq!(loaded.notes.len(), 2065);
            let load_time = start.elapsed();
            let _ = loaded.subtitles();
            let _ = loaded.count(Section::All);
            let elapsed = start.elapsed();
            eprintln!("SRD run {run}: load={load_time:?}, ready={elapsed:?}");
            assert!(elapsed.as_secs_f64() < 1.0, "SRD library must load in <1s");
        }
    }

    /// Exercise the editable package path with the same notes, read from disk.
    #[test]
    #[ignore]
    fn user_library_load_time() {
        fn copy_notes(source: &Path, destination: &Path) {
            fs::create_dir_all(destination).unwrap();
            for item in fs::read_dir(source).unwrap() {
                let path = item.unwrap().path();
                let target = destination.join(path.file_name().unwrap());
                if path.is_dir() {
                    copy_notes(&path, &target);
                } else if path.extension().is_some_and(|ext| ext == "md") {
                    fs::copy(path, target).unwrap();
                }
            }
        }
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "User package", "user").unwrap();
        copy_notes(
            &Path::new(env!("CARGO_MANIFEST_DIR")).join("builtin/dnd2024-definitions"),
            &module.path,
        );
        for run in 0..3 {
            let start = std::time::Instant::now();
            let loaded = load(Some(&module)).unwrap();
            assert_eq!(loaded.notes.len(), 2066);
            assert!(loaded.notes.iter().all(|note| note.entry.is_some()));
            let _ = loaded.subtitles();
            let _ = loaded.count(Section::All);
            let elapsed = start.elapsed();
            eprintln!("User package run {run}: ready={elapsed:?}");
            assert!(elapsed.as_secs_f64() < 1.0, "User library must load in <1s");
        }
    }

    #[test]
    fn entry_rows_keep_both_lines_at_a_fixed_height() {
        let ctx = egui::Context::default();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(186.0, 100.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.set_width(170.0);
                    let response = entry_row(
                        ui,
                        &"Long entry name ".repeat(30),
                        &"Long subtitle ".repeat(30),
                        false,
                    );
                    assert_eq!(response.rect.height(), 44.0);
                });
            },
        );
        let rows: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if !text.galley.job.text.is_empty() => Some(&text.galley),
                _ => None,
            })
            .collect();
        assert_eq!(rows.len(), 2);
        assert!(
            rows.iter()
                .all(|galley| galley.rows.len() == 1 && galley.rect.width() <= 162.0)
        );
    }

    #[test]
    fn list_metadata_is_refreshed_after_content_changes() {
        let mut loaded = Loaded {
            notes: vec![Note {
                path: "first.md".into(),
                text: String::new(),
                entry: None,
                error: None,
                owner: Owner::Builtin,
            }],
            ..Default::default()
        };
        assert_eq!(loaded.count(Section::All), 1);
        assert_eq!(loaded.subtitles().len(), 1);
        loaded.notes.push(Note {
            path: "second.md".into(),
            text: String::new(),
            entry: None,
            error: None,
            owner: Owner::Builtin,
        });
        loaded.invalidate();
        assert_eq!(loaded.count(Section::All), 2);
        assert_eq!(loaded.subtitles().len(), 2);
    }

    /// Run explicitly to measure the complete library UI while scrolling the built-in package.
    #[test]
    #[ignore]
    fn library_scroll_frame_times() {
        let mut library = Library {
            loaded: Some(load(None).unwrap()),
            ..Default::default()
        };
        let mut campaigns = Campaigns::default();
        let ctx = egui::Context::default();
        let mut times = Vec::new();
        for frame in 0..150 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                time: Some(frame as f64 / 60.0),
                events: vec![
                    egui::Event::PointerMoved(egui::pos2(300.0, 450.0)),
                    egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: egui::vec2(0.0, -30.0),
                        modifiers: Default::default(),
                    },
                ],
                ..Default::default()
            };
            let start = std::time::Instant::now();
            let output = ctx.run(input, |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| library.show(ui, &mut campaigns));
            });
            let _ = ctx.tessellate(output.shapes, output.pixels_per_point);
            if frame >= 30 {
                times.push(start.elapsed().as_secs_f64() * 1000.0);
            }
        }
        times.sort_by(f64::total_cmp);
        println!(
            "Library entries: {}; frame CPU ms: median={:.3}, p95={:.3}, max={:.3}",
            library.loaded.as_ref().unwrap().notes.len(),
            times[times.len() / 2],
            times[times.len() * 95 / 100],
            times.last().unwrap()
        );
    }

    #[test]
    fn same_name_entries_have_distinct_visible_labels() {
        let loaded = load(None).unwrap();
        let subtitles = loaded.subtitles();
        let labels: std::collections::HashSet<_> = loaded
            .notes
            .iter()
            .zip(subtitles)
            .map(|(note, subtitle)| (note.name(), subtitle))
            .collect();
        assert_eq!(labels.len(), loaded.notes.len());
    }

    #[test]
    fn imported_overview_renders_as_prose_without_source_markup() {
        for (path, expected) in [
            (
                "definitions/glossary/ability_check.md",
                "represents using one of the six abilities",
            ),
            (
                "definitions/spells/bless.md",
                "Whenever a target makes an attack roll",
            ),
            (
                "definitions/spells/command.md",
                "Approach. The target moves toward you by the shortest",
            ),
            (
                "definitions/rules/d20_tests_ability_modifier_2.md",
                "Strength Physically resist direct force\nDexterity Dodge",
            ),
            (
                "definitions/rules/d20_tests_ability_modifier.md",
                "Wisdom Notice things in the environment or in creatures’ behavior\nCharisma",
            ),
            (
                "definitions/spells/divine_smite.md",
                "if the target is a Fiend or an Undead.",
            ),
            (
                "definitions/spells/augury.md",
                "circumstances, such as other spells, that might change the results.",
            ),
        ] {
            let text = content::definitions::source_file(path).unwrap().to_owned();
            let entry = Entry::parse(path, &text).unwrap();
            let overview = presentation::reading_sections(&entry.body, &entry.name)
                .into_iter()
                .find(|section| section.title == "Source text")
                .unwrap();
            let readable = presentation::overview_markdown(overview.body);
            if path.ends_with("command.md") {
                for option in ["Approach.", "Drop.", "Flee.", "Grovel.", "Halt."] {
                    assert!(readable.contains(&format!("\n\n{option}")), "{readable}");
                }
                assert!(
                    readable.contains("holding and then ends its turn."),
                    "{readable}"
                );
            }
            if path.ends_with("augury.md") {
                assert!(
                    readable.contains(
                        "Weal Good  \nWoe Bad  \nWeal and woe Good and bad  \nIndifference"
                    ),
                    "{readable}"
                );
            }
            let mut library = Library {
                loaded: Some(Loaded {
                    notes: vec![Note {
                        path: entry.path.clone(),
                        text: text.clone(),
                        entry: Some(entry),
                        error: None,
                        owner: Owner::Builtin,
                    }],
                    diagnostics: vec![],
                    ..Default::default()
                }),
                selected: Some(0),
                draft: text,
                ..Default::default()
            };
            let ctx = egui::Context::default();
            let subtitles = library.loaded.as_ref().unwrap().subtitles().clone();
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1000.0, 700.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| library.editor(ui, &subtitles[0]));
                },
            );
            let text = output
                .shapes
                .iter()
                .filter_map(|s| match &s.shape {
                    egui::Shape::Text(t) => Some(t.galley.job.text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");
            assert!(text.contains(expected), "{text}");
            assert!(!text.contains('\\'), "{text}");
            assert!(!text.contains("[["), "{text}");
            assert!(!text.contains("Original layout:"), "{text}");
        }
    }
    #[test]
    fn creature_spell_links_open_named_spells_and_protect_drafts() {
        let notes = [
            "creatures/adult_bronze_dragon",
            "spells/detect_magic",
            "spells/guiding_bolt",
            "spells/shapechange",
            "spells/speak_with_animals",
            "spells/thaumaturgy",
            "spells/detect_thoughts",
            "spells/water_breathing",
        ]
        .map(|name| {
            let path = format!("definitions/{name}.md");
            let source = content::definitions::source_file(&path).unwrap();
            Note {
                entry: Some(Entry::parse(&path, source).unwrap()),
                path,
                text: source.into(),
                error: None,
                owner: Owner::Builtin,
            }
        })
        .into_iter()
        .collect();
        let loaded = Loaded {
            notes,
            ..Default::default()
        };
        let references = creature::spell_references(loaded.notes[0].entry.as_ref().unwrap());
        assert_eq!(references.len(), 7);
        for (reference, index) in references.iter().zip(1..8) {
            assert_eq!(loaded.resolve_spell(reference), Some(index));
        }
        let by_id = content::Reference::Id(loaded.notes[2].entry.as_ref().unwrap().id);
        assert_eq!(loaded.resolve_spell(&by_id), Some(2));
        assert_eq!(
            loaded.resolve_spell(&content::Reference::Key("missing:spell".into())),
            None
        );

        let mut library = Library {
            draft: loaded.notes[0].text.clone(),
            loaded: Some(loaded),
            selected: Some(0),
            section: Section::Creatures,
            search: "bronze".into(),
            ..Default::default()
        };
        for width in [390.0, 900.0] {
            for dirty in [true, false] {
                library.open_entry(0);
                library.search = "bronze".into();
                library.matches = Some((Section::Creatures, "bronze".into(), vec![0]));
                if dirty {
                    library.draft.push_str("\nUnsaved note.\n");
                }
                let ctx = egui::Context::default();
                let mut frame = |events| {
                    ctx.run(
                        egui::RawInput {
                            screen_rect: Some(egui::Rect::from_min_size(
                                egui::Pos2::ZERO,
                                egui::vec2(width, 6000.0),
                            )),
                            events,
                            ..Default::default()
                        },
                        |ctx| {
                            egui::CentralPanel::default()
                                .show(ctx, |ui| library.editor(ui, "Definition · creature"));
                        },
                    )
                };
                frame(vec![]);
                let output = frame(vec![]);
                let texts: Vec<_> = output
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => Some(text),
                        _ => None,
                    })
                    .collect();
                let link = texts
                    .iter()
                    .find(|t| t.galley.job.text == "Guiding Bolt")
                    .unwrap();
                let spellcasting = texts
                    .iter()
                    .find(|t| t.galley.job.text.starts_with("Spellcasting."))
                    .unwrap();
                let legendary = texts
                    .iter()
                    .find(|t| t.galley.job.text == "Legendary Actions")
                    .unwrap();
                assert!(link.pos.y > spellcasting.pos.y && link.pos.y < legendary.pos.y);
                assert!(!texts.iter().any(|t| t.galley.job.text.contains("^splat-")));
                let position = link.pos
                    + link.galley.rows[0]
                        .rect_without_leading_space()
                        .center()
                        .to_vec2();
                frame(vec![egui::Event::PointerMoved(position)]);
                for pressed in [true, false] {
                    frame(vec![egui::Event::PointerButton {
                        pos: position,
                        button: egui::PointerButton::Primary,
                        pressed,
                        modifiers: egui::Modifiers::NONE,
                    }]);
                }
                if dirty {
                    assert_eq!(library.selected, Some(0));
                    assert!(library.draft.ends_with("Unsaved note.\n"));
                    library.draft = library.loaded.as_ref().unwrap().notes[0].text.clone();
                } else {
                    assert_eq!(library.selected, Some(2));
                    assert_eq!(library.section, Section::Spells);
                    assert!(library.search.is_empty());
                    assert!(library.matches.is_none());
                    assert!(!library.source);
                    assert!(library.draft.contains("# Guiding Bolt"));
                }
            }
        }
        // Conflicting identities must not choose an arbitrary destination.
        let loaded = library.loaded.as_mut().unwrap();
        let duplicate = &loaded.notes[2];
        loaded.notes.push(Note {
            path: "duplicate.md".into(),
            text: duplicate.text.clone(),
            entry: duplicate.entry.clone(),
            error: None,
            owner: Owner::Builtin,
        });
        assert_eq!(loaded.resolve_spell(&by_id), None);
    }

    #[test]
    fn creature_read_view_hides_rule_anchors() {
        let path = "definitions/creatures/adult_bronze_dragon.md";
        let source = content::definitions::source_file(path).unwrap();
        let mut library = Library {
            loaded: Some(Loaded {
                notes: vec![Note {
                    path: path.into(),
                    text: source.into(),
                    entry: Some(Entry::parse(path, source).unwrap()),
                    error: None,
                    owner: Owner::Builtin,
                }],
                ..Default::default()
            }),
            selected: Some(0),
            draft: source.into(),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 6000.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| library.editor(ui, "Definition · creature"));
            },
        );
        let text = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(text.iter().any(|text| text.contains("Spellcasting.")));
        assert!(
            !text.iter().any(|text| text.contains("^splat-")),
            "{text:?}"
        );
        assert!(
            !text.contains(&"Rule"),
            "Empty mechanics section should be hidden"
        );
    }

    #[test]
    fn reviewed_summary_keeps_rules_and_source_and_opens_related_entry() {
        let loaded = load(None).unwrap();
        let index = loaded
            .notes
            .iter()
            .position(|note| note.name() == "Level 3: Bard Subclass")
            .unwrap();
        let entry = loaded.notes[index].entry.as_ref().unwrap();
        let mut original = entry.clone();
        original.fields.remove("splat_reading");
        assert_eq!(
            content::contracts::specification(entry).unwrap(),
            content::contracts::specification(&original).unwrap()
        );
        assert_eq!(
            serde_json::to_value(content::required_references(entry).unwrap()).unwrap(),
            serde_json::to_value(content::required_references(&original).unwrap()).unwrap()
        );
        assert!(
            entry
                .body
                .contains("of Lore subclass is detailed after this class’s descrip-")
        );
        for width in [390.0, 900.0] {
            let mut library = Library {
                loaded: Some(load(None).unwrap()),
                ..Default::default()
            };
            library.open_entry(index);
            let ctx = egui::Context::default();
            let mut frame = |events| {
                ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 1400.0),
                        )),
                        events,
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            library.editor(ui, "Definition · class feature · Bard")
                        });
                    },
                )
            };
            frame(vec![]);
            let output = frame(vec![]);
            let texts: Vec<_> = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some(text),
                    _ => None,
                })
                .collect();
            assert!(
                texts
                    .iter()
                    .all(|text| text.pos.x + text.galley.size().x <= width)
            );
            let rendered = texts
                .iter()
                .map(|text| text.galley.job.text.as_str())
                .collect::<Vec<_>>()
                .join("\n");
            for expected in [
                "Bard Subclass",
                "Bard · Level 3",
                "Choose a Bard subclass.",
                "Available in this package",
                "Source text",
            ] {
                assert!(rendered.contains(expected), "{rendered}");
            }
            assert!(!rendered.contains("You gain a Bard subclass of your choice."));
            // The published wording is still readable, and the related link navigates.
            for label in ["Source text", "College of Lore"] {
                let text = texts
                    .iter()
                    .find(|text| text.galley.job.text == label)
                    .unwrap();
                let pos = text.pos + text.galley.size() * 0.5;
                for pressed in [true, false] {
                    frame(vec![
                        egui::Event::PointerMoved(pos),
                        egui::Event::PointerButton {
                            pos,
                            button: egui::PointerButton::Primary,
                            pressed,
                            modifiers: Default::default(),
                        },
                    ]);
                }
                if label == "Source text" {
                    let output = frame(vec![]);
                    assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.contains("You gain a Bard subclass of your choice."))));
                }
            }
            assert!(library.draft.contains("# College of Lore"));
        }
    }

    /// The words a built-in entry paints in the reader, in order.
    fn reader_text(name: &str) -> String {
        let loaded = load(None).unwrap();
        let index = loaded
            .notes
            .iter()
            .position(|note| note.name() == name)
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        library.open_entry(index);
        let ctx = egui::Context::default();
        let mut frame = || {
            ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(390.0, 2400.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| library.editor(ui, "Definition"));
                },
            )
        };
        frame();
        frame()
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn an_item_has_one_identity_line_with_its_attunement_before_the_rules() {
        let text = reader_text("Dragon Orb");
        assert!(!text.contains("Source classification"), "{text}");
        assert!(!text.contains("wondrous item · artifact"), "{text}");
        let identity = text.find("(Requires Attunement)").expect("the book's line");
        assert!(
            identity < text.find("An orb is an etched crystal").unwrap(),
            "{text}"
        );
    }

    #[test]
    fn a_class_that_grants_its_features_links_to_them_and_its_spell_list() {
        let text = reader_text("Wizard");
        assert!(text.contains("Class features"), "{text}");
        assert!(text.find("Arcane Recovery").unwrap() < text.find("Signature Spells").unwrap());
        assert!(text.contains("Wizard Spell List"), "{text}");
        assert!(text.contains("Wizard Progression"), "{text}");
        // Tight rows in one case, as a spell's are, and each option on its own row.
        assert!(text.contains("Primary ability\nIntelligence"), "{text}");
        assert!(text.contains("Option A\n"), "{text}");
        assert!(text.contains("Option B\n55 GP"), "{text}");
    }

    #[test]
    fn a_subclass_lists_its_features() {
        let text = reader_text("College of Lore");
        assert!(text.contains("Subclass features"), "{text}");
        assert!(text.contains("Cutting Words"), "{text}");
    }

    #[test]
    fn class_reader_is_readable_and_opens_features() {
        let loaded = load(None).unwrap();
        let index = loaded
            .notes
            .iter()
            .position(|note| note.name() == "Barbarian")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        library.open_entry(index);
        let ctx = egui::Context::default();
        let mut frame = |events| {
            ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(390.0, 2400.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default()
                        .show(ctx, |ui| library.editor(ui, "Definition · class"));
                },
            )
        };
        frame(vec![]);
        let output = frame(vec![]);
        let texts: Vec<_> = output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::Shape::Text(text) => Some(text),
                _ => None,
            })
            .collect();
        assert!(
            texts
                .iter()
                .all(|text| text.pos.x + text.galley.size().x <= 390.0)
        );
        let rendered = texts
            .iter()
            .map(|text| text.galley.job.text.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        // A label and its value, as a spell's facts are: on one row, in one case.
        assert!(rendered.contains("Primary ability\nStrength"), "{rendered}");
        assert!(rendered.contains("Starting equipment"));
        assert!(!rendered.contains("dnd2024:class_features/"));
        assert!(rendered.find("Rage").unwrap() < rendered.find("Relentless Rage").unwrap());
        let link = texts
            .iter()
            .find(|text| text.galley.job.text == "Relentless Rage")
            .unwrap();
        let pos = link.pos + link.galley.size() * 0.5;
        for pressed in [true, false] {
            frame(vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: Default::default(),
                },
            ]);
        }
        assert!(library.draft.contains("# Level 11: Relentless Rage"));
    }

    #[test]
    fn creature_read_view_shows_one_styled_stat_block() {
        let path = "definitions/creatures/vampire_familiar.md";
        let source = content::definitions::source_file(path).unwrap();
        let entry = Entry::parse(path, source).unwrap();
        for width in [390.0, 800.0] {
            let mut library = Library {
                loaded: Some(Loaded {
                    notes: vec![Note {
                        path: path.into(),
                        text: source.into(),
                        entry: Some(entry.clone()),
                        error: None,
                        owner: Owner::Builtin,
                    }],
                    diagnostics: vec![],
                    ..Default::default()
                }),
                selected: Some(0),
                draft: source.into(),
                ..Default::default()
            };
            let ctx = egui::Context::default();
            let mut rendered = String::new();
            for _ in 0..2 {
                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(width, 2400.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default()
                            .show(ctx, |ui| library.editor(ui, "Definition · creature"));
                    },
                );
                rendered = output
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
            }
            assert_eq!(rendered.matches("65 (10d8 + 20)").count(), 1, "{rendered}");
            for expected in [
                "ARMOR CLASS",
                "STR",
                "Save +5",
                "Vampiric Connection.",
                "Umbral Dagger.",
                "Deathless Agility.",
                "except from its vampire master",
                "Paralyzed",
            ] {
                assert!(
                    rendered.contains(expected),
                    "Missing {expected}: {rendered}"
                );
            }
            for duplicate in [
                "Creature data",
                "Abilities data",
                "Stats data",
                "MOD SAVE",
                "AC 15",
            ] {
                assert!(!rendered.contains(duplicate), "{rendered}");
            }
            assert_eq!(library.draft, source);
        }
    }

    #[test]
    fn create_register_and_edit_preserves_identity_and_external_changes() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("module");
        let m = create_module(&root, "My Homebrew", "my_homebrew").unwrap();
        assert_eq!(local_module(&root).unwrap().id, m.id);
        assert!(create_module(&root, "Overwrite", "other").is_err());
        let text = note_text("Goblin", "creature", None);
        let entry = write_note(&root, "Goblin.md", None, &text).unwrap();
        assert!(write_note(&root, "Goblin.md", None, &text).is_err());
        let edited = format!("{text}New description\n");
        assert_eq!(
            write_note(&root, "Goblin.md", Some(&text), &edited)
                .unwrap()
                .id,
            entry.id
        );
        fs::write(root.join("Goblin.md"), "external edit").unwrap();
        assert!(write_note(&root, "Goblin.md", Some(&edited), &text).is_err());
        assert_eq!(
            fs::read_to_string(root.join("Goblin.md")).unwrap(),
            "external edit"
        );
        assert!(write_note(&root, "../escape.md", None, &text).is_err());
    }
    #[test]
    fn undo_discard_is_offered_only_to_the_note_it_came_from() {
        let temp = tempfile::tempdir().unwrap();
        let first = create_module(&temp.path().join("first"), "First", "first").unwrap();
        let second = create_module(&temp.path().join("second"), "Second", "second").unwrap();
        let note = |module: &Module, path: &str| Note {
            path: path.into(),
            text: String::new(),
            entry: None,
            error: None,
            owner: Owner::Package(module.clone()),
        };
        let discarded = (Some(first.id), "a.md".to_string(), "words".to_string());
        assert!(discard_is_for(Some(&discarded), &note(&first, "a.md")));
        assert!(!discard_is_for(Some(&discarded), &note(&first, "b.md")));
        assert!(!discard_is_for(Some(&discarded), &note(&second, "a.md")));
    }

    #[test]
    fn library_frame_keeps_campaign_selection_and_draft_independent() {
        let temp = tempfile::tempdir().unwrap();
        let m = create_module(&temp.path().join("module"), "Module", "test").unwrap();
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        campaigns.register_module(m.clone()).unwrap();
        let mut library = Library {
            loaded: Some(load(Some(&m)).unwrap()),
            selected: Some(0),
            ..Default::default()
        };
        library.draft = library.loaded.as_ref().unwrap().notes[0].text.clone();
        library.draft.push_str("Unsaved description");
        let expected = library.draft.clone();
        library.search = "no matching entry".into();
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| library.show(ui, &mut campaigns));
        });
        assert!(campaigns.active_campaign().is_none());
        assert_eq!(library.draft, expected);
        campaigns.unregister_module(m.id).unwrap();
        assert!(m.path.join("Module.md").exists());
    }

    #[test]
    fn loading_keeps_invalid_notes_available_for_repair() {
        let temp = tempfile::tempdir().unwrap();
        let m = create_module(&temp.path().join("module"), "Module", "test").unwrap();
        fs::write(
            m.path.join("Broken.md"),
            "---\nsplat_schema: 999\n---\nBroken",
        )
        .unwrap();
        let loaded = load(Some(&m)).unwrap();
        assert_eq!(loaded.notes.len(), 2);
        assert!(loaded.notes.iter().any(|n| n.error.is_some()));
        assert!(!loaded.diagnostics.is_empty());
    }
}
