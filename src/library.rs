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
use splat::content::{
    self, Entry,
    creature_filter::{Facets, Filter, Query},
};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    sync::mpsc,
};
use uuid::Uuid;

const BUILTIN_NAME: &str = "D&D 2024 · SRD 5.2.1";

fn copy_destination_reason(campaigns: &Campaigns) -> Option<&'static str> {
    (campaigns.modules().is_empty() && campaigns.all().is_empty())
        .then_some("Add a campaign or a package to create entries.")
}

/// Who owns an entry, and so whether and where it can be saved.
#[derive(Clone, Default, PartialEq, Debug)]
enum Owner {
    #[default]
    Builtin,
    Package(Module),
    Campaign(Campaign),
}

impl Owner {
    fn folder(&self) -> String {
        self.root()
            .and_then(Path::file_name)
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    }
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
            Self::Campaign(_) => "Campaign".into(),
        }
    }
    /// Where an entry came from.
    fn source(&self) -> &str {
        match self {
            Self::Builtin => BUILTIN_NAME,
            Self::Package(module) => &module.name,
            Self::Campaign(_) => "Campaign",
        }
    }
}

fn name_list(names: &[String]) -> String {
    let shown = &names[..names.len().min(3)];
    if names.len() > 3 {
        return format!("{} and {} more", shown.join(", "), names.len() - 3);
    }
    match shown {
        [] => String::new(),
        [name] => name.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

fn draft_words(campaign: bool) -> &'static str {
    if campaign {
        "Save or discard to copy, add an entry, change packages or switch campaign."
    } else {
        "Save or discard to copy, add an entry or change packages."
    }
}

/// The reader's kind line keeps its kind, class, level and source.
fn reader_subtitle(owner: &Owner, listed: &str, class_level: Option<String>) -> String {
    match class_level {
        Some(level) => format!("Class feature · {level} · {}", owner.source()),
        None if *owner == Owner::Builtin => format!("{listed} · {BUILTIN_NAME}"),
        None => listed.to_owned(),
    }
}

fn scope_label(ui: &egui::Ui, selected: &str, budget: f32) -> String {
    let font = Type::Strong.font(ui);
    let fitted = design::words_that_fit(ui, selected, &font, budget);
    if design::number_width(ui, &fitted, &font) <= budget {
        return fitted;
    }
    let mut cut = fitted.trim_end_matches('…').to_owned();
    while !cut.is_empty() && design::number_width(ui, &format!("{cut}…"), &font) > budget {
        cut.pop();
    }
    format!("{cut}…")
}

fn section_row(ui: &mut egui::Ui, label: &str, count: usize, selected: bool) -> egui::Response {
    design::row(
        ui,
        Density::Read.row(),
        selected,
        &format!("{label}   {}", design::count(count, "", "").trim()),
        |ui, rect| {
            let c = colors();
            let font = if selected { Type::Strong } else { Type::Body };
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
                design::count(count, "", "").trim(),
                &Type::Small.font(ui),
                c.muted,
            );
        },
    )
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
    fn is_manifest(&self) -> bool {
        matches!(self.owner, Owner::Package(_))
            && (self
                .entry
                .as_ref()
                .is_some_and(|entry| entry.kind == "package")
                || matches!(self.path.as_str(), "Package.md" | "Module.md")
                || self.text.lines().any(|line| line == "splat_kind: package"))
    }

    fn about_label(&self) -> String {
        let name = match &self.owner {
            Owner::Package(module) => &module.name,
            _ => self.name(),
        };
        format!(
            "{}About {name}",
            if self.error.is_some() {
                format!("{} ", icons::WARN)
            } else {
                String::new()
            }
        )
    }

    fn unreadable_kind(&self) -> Section {
        if self.is_manifest() {
            return Section::PackageDetails;
        }
        let declared = self
            .text
            .lines()
            .find_map(|line| line.strip_prefix("splat_kind: "))
            .map(|kind| Section::for_kind(kind.trim().trim_matches(['\'', '"'])))
            .unwrap_or(Section::Unreadable);
        if ![Section::Other, Section::Unreadable].contains(&declared) {
            return declared;
        }
        self.text
            .lines()
            .find_map(|line| {
                let kind = line.strip_prefix("splat_")?.split_once(':')?.0;
                let section = Section::for_kind(kind);
                (![Section::Other, Section::Unreadable].contains(&section)).then_some(section)
            })
            .unwrap_or(Section::Unreadable)
    }
    fn name(&self) -> &str {
        self.entry.as_ref().map_or(&self.path, |e| &e.name)
    }
    fn section(&self) -> Section {
        self.entry
            .as_ref()
            .map_or(Section::Unreadable, Section::for_entry)
    }
    /// Plain words match the name, path or section, or a creature's facts. `habitat:forest`
    /// and the other fact terms match creatures only.
    fn matches(&self, section: Section, query: &Query, facets: Option<&Facets>) -> bool {
        let words = query.words.as_str();
        if words.is_empty() && !query.names_facts() {
            return section == Section::All || self.section() == section;
        }
        let text = words.is_empty()
            || self.name().to_lowercase().contains(words)
            || self.path.to_lowercase().contains(words)
            || self.section().label().to_lowercase().contains(words)
            || facets.is_some_and(|facets| query.words_in(self.name(), facets));
        text && (!query.names_facts() || facets.is_some_and(|facets| query.matches_facts(facets)))
    }
}
#[derive(Default)]
struct DuplicateIndex {
    ids: std::collections::HashMap<Uuid, Vec<usize>>,
    keys: std::collections::HashMap<String, Vec<usize>>,
}

#[derive(Default)]
struct Loaded {
    notes: Vec<Note>,
    owners: Vec<Owner>,
    /// Problems that kept content from loading. The notes cannot show these again.
    load_errors: Vec<String>,
    diagnostics: Vec<String>,
    /// What the active campaign can use: its selections and what they require.
    uses: Option<std::collections::HashSet<Uuid>>,
    subtitles: std::sync::OnceLock<Vec<String>>,
    counts: std::sync::OnceLock<Vec<(Section, usize)>>,
    duplicate_ids: std::sync::OnceLock<std::collections::HashSet<Uuid>>,
    duplicates: std::sync::OnceLock<DuplicateIndex>,
    /// Each note's creature facts, for search and the creature filter.
    facets: std::sync::OnceLock<Vec<Option<Facets>>>,
}
impl Loaded {
    fn facets(&self) -> &[Option<Facets>] {
        self.facets.get_or_init(|| {
            self.notes
                .iter()
                .map(|note| note.entry.as_ref().and_then(Facets::of))
                .collect()
        })
    }
    fn old_layout_count(&self) -> usize {
        self.notes
            .iter()
            .filter(|note| {
                note.owner != Owner::Builtin
                    && note
                        .error
                        .as_deref()
                        .is_some_and(|e| e.contains(content::OLD_LAYOUT))
            })
            .count()
    }

    fn can_rename_links(&self, selected: usize, old: &str, new: &str) -> bool {
        !self.notes.iter().enumerate().any(|(at, note)| {
            at != selected
                && note.entry.is_some()
                && [old, new].iter().any(|name| {
                    note.name().eq_ignore_ascii_case(name)
                        || Path::new(&note.path)
                            .file_stem()
                            .and_then(|stem| stem.to_str())
                            .is_some_and(|stem| stem.eq_ignore_ascii_case(name))
                })
        })
    }
    fn duplicate_folder_paths(&self, owner: &Owner) -> Option<Vec<String>> {
        let root = owner.root()?;
        let mut other_root = None;
        let mut paths = Vec::new();
        for (at, note) in self.notes.iter().enumerate() {
            if note
                .entry
                .as_ref()
                .is_none_or(|entry| entry.kind == "package")
            {
                continue;
            }
            let duplicates = self.duplicates(at);
            if duplicates.is_empty() {
                continue;
            }
            if duplicates.len() != 1 {
                return None;
            }
            let pair = &self.notes[duplicates[0]];
            let (here, there) = (note.owner.root()?, pair.owner.root()?);
            let counterpart = if here == root {
                there
            } else if there == root {
                here
            } else {
                return None;
            };
            if counterpart == root || other_root.is_some_and(|known| known != counterpart) {
                return None;
            }
            other_root = Some(counterpart);
            if here == root {
                paths.push(note.path.clone());
            }
        }
        (paths.len() > 1).then_some(paths)
    }
    fn duplicate_ids(&self) -> &std::collections::HashSet<Uuid> {
        self.duplicate_ids.get_or_init(|| {
            self.notes
                .iter()
                .enumerate()
                .filter(|(at, _)| !self.duplicates(*at).is_empty())
                .filter_map(|(_, note)| note.entry.as_ref().map(|entry| entry.id))
                .collect()
        })
    }
    fn dependents(&self, note: &Note) -> Vec<String> {
        let Some(id) = crate::content_workspace::note_id(&note.text) else {
            return Vec::new();
        };
        self.notes
            .iter()
            .filter_map(|other| {
                let entry = other.entry.as_ref()?;
                (entry.id != id
                    && content::required_references(entry).is_ok_and(|refs| {
                        refs.contains(&content::Reference::Id(id))
                            || note
                                .entry
                                .as_ref()
                                .and_then(|entry| entry.key.as_ref())
                                .is_some_and(|key| {
                                    refs.contains(&content::Reference::Key(key.clone()))
                                })
                    }))
                .then(|| entry.name.clone())
            })
            .collect()
    }
    fn duplicate_index(&self) -> &DuplicateIndex {
        self.duplicates.get_or_init(|| {
            let mut index = DuplicateIndex::default();
            for (at, note) in self.notes.iter().enumerate() {
                if let Some(entry) = &note.entry {
                    index.ids.entry(entry.id).or_default().push(at);
                    if let Some(key) = &entry.key {
                        index.keys.entry(key.clone()).or_default().push(at);
                    }
                }
            }
            index
        })
    }
    fn duplicates(&self, index: usize) -> Vec<usize> {
        let Some(target) = self.notes[index].entry.as_ref() else {
            return Vec::new();
        };
        let lookup = self.duplicate_index();
        let mut matches: Vec<_> = lookup
            .ids
            .get(&target.id)
            .into_iter()
            .flatten()
            .chain(
                target
                    .key
                    .as_ref()
                    .and_then(|key| lookup.keys.get(key))
                    .into_iter()
                    .flatten(),
            )
            .copied()
            .filter(|&at| at != index)
            .collect();
        // The original scan returns each matching note once, in Library order.
        matches.sort_unstable();
        matches.dedup();
        matches
    }
    /// Checks all the loaded content as one, so that a reference can resolve to built-in
    /// content or to another package. The check of one package alone cannot see those.
    fn refresh_diagnostics(&mut self) {
        let entries: Vec<_> = self
            .notes
            .iter()
            .filter_map(|n| n.entry.as_ref())
            // The SRD pages have no notes in the Library, but entries can refer to them.
            .chain(content::builtin::entries())
            .collect();
        let report = content::validate_all(&entries);
        let checks = report.errors.into_iter().chain(report.warnings);
        self.diagnostics = self
            .load_errors
            .iter()
            .cloned()
            .chain(checks.map(|d| format!("{}: {}", d.path, d.message)))
            .chain(
                self.notes
                    .iter()
                    .filter_map(|n| n.error.as_ref().map(|e| format!("{}: {e}", n.path))),
            )
            .collect();
        for note in self
            .notes
            .iter()
            .filter(|note| note.owner != Owner::Builtin)
        {
            if let Some(entry) = &note.entry {
                for flag in &entry.flags {
                    self.diagnostics
                        .push(format!("{}: {}", note.path, flag.message));
                }
                let faults = crate::roll_table::fault_count(&entry.body);
                if faults > 0 {
                    self.diagnostics.push(format!(
                        "{}: {}",
                        note.path,
                        design::count(faults, "table fault", "table faults")
                    ));
                }
            }
            if let Some(entry) = &note.entry
                && let Some((old, _)) = presentation::heading_fix(&note.text, entry)
            {
                self.diagnostics.push(format!(
                    "{}: The first heading says {old}, but the name is {}.",
                    note.path, entry.name
                ));
            }
        }
    }

    fn in_scope(&self, note: &Note, scope: Scope) -> bool {
        if note
            .entry
            .as_ref()
            .is_some_and(|entry| entry.kind == "package")
        {
            return false;
        }
        match scope {
            Scope::All => true,
            Scope::Uses => match &self.uses {
                None => true,
                Some(uses) => {
                    matches!(note.owner, Owner::Campaign(_))
                        || note
                            .entry
                            .as_ref()
                            .is_some_and(|entry| self.duplicate_ids().contains(&entry.id))
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
        self.duplicate_ids.take();
        self.duplicates.take();
        self.facets.take();
    }
    fn build_subtitles(&self) -> Vec<String> {
        let mut subtitles: Vec<_> = self
            .notes
            .iter()
            .map(|note| {
                let source = match &note.owner {
                    Owner::Builtin => None,
                    Owner::Package(module) => Some(module.name.as_str()),
                    Owner::Campaign(_) => Some("Campaign"),
                };
                let line = note.entry.as_ref().map_or_else(
                    || {
                        let (_, line) =
                            crate::campaign_content::repair_problem(&note.path, &note.text);
                        if note.error.as_deref().is_some_and(crate::formats_ui::is_old) {
                            return "Old layout".to_owned();
                        }
                        line.map_or_else(
                            || "Cannot be read".to_owned(),
                            |line| format!("Line {line}"),
                        )
                    },
                    presentation::subtitle,
                );
                let line = if note
                    .entry
                    .as_ref()
                    .is_some_and(|entry| self.duplicate_ids().contains(&entry.id))
                {
                    format!("{} Duplicate identity · {line}", icons::WARN)
                } else {
                    line
                };
                let mut subtitle = presentation::list_subtitle(&line, source);
                if let Owner::Package(module) = &note.owner
                    && self.owners.iter().any(|owner| matches!(owner, Owner::Package(other) if other.id != module.id && other.name == module.name))
                { subtitle.push_str(&format!(" · {}/", note.owner.folder())); }
                subtitle
            })
            .collect();
        // Imported sections share a broad parent. Use the closest named subsection
        // before a repeated heading, from the source spans, rather than a page number.
        let parent_of = |note: &Note| {
            note.entry
                .as_ref()
                .and_then(|entry| entry.fields.get("splat_conversion"))
                .and_then(|meta| meta.get("parent_name"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_owned()
        };
        let mut name_counts = std::collections::HashMap::new();
        for note in &self.notes {
            *name_counts
                .entry((parent_of(note), note.name()))
                .or_insert(0_usize) += 1;
        }
        for (i, note) in self.notes.iter().enumerate() {
            let duplicates = name_counts[&(parent_of(note), note.name())];
            if duplicates < 2 {
                continue;
            }
            let Some(entry) = &note.entry else {
                continue;
            };
            let Some(meta) = entry.fields.get("splat_conversion") else {
                continue;
            };
            let parent = meta.get("parent_name").and_then(|v| v.as_str());
            let first = meta
                .get("line_span")
                .and_then(|span| span.get("first"))
                .and_then(|v| v.as_u64());
            let Some(first) = first else {
                continue;
            };
            let context = self
                .notes
                .iter()
                .filter(|other| other.owner == note.owner)
                .filter_map(|other| {
                    let meta = other.entry.as_ref()?.fields.get("splat_conversion")?;
                    if meta.get("parent_name").and_then(|v| v.as_str()) != parent
                        || name_counts[&(parent_of(other), other.name())] > 1
                    {
                        return None;
                    }
                    let start = meta.get("line_span")?.get("first")?.as_u64()?;
                    (if parent.is_some_and(|parent| parent.eq_ignore_ascii_case(note.name())) {
                        start > first
                    } else {
                        start < first
                    })
                    .then_some((start, other.name()))
                })
                .min_by_key(|(start, _)| {
                    if parent.is_some_and(|parent| parent.eq_ignore_ascii_case(note.name())) {
                        *start
                    } else {
                        u64::MAX - *start
                    }
                });
            if let Some((_, context)) = context
                && let Some(parent) = parent
            {
                if parent.eq_ignore_ascii_case(note.name()) {
                    subtitles[i].push_str(&format!(" · {context}"));
                } else {
                    subtitles[i] = subtitles[i]
                        .replace(&format!(" · {parent}"), &format!(" · {parent} › {context}"));
                }
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
    let width = ui.available_width() - design::S * 2.0;
    let title = design::words_that_fit(
        ui,
        name,
        &if selected { Type::Strong } else { Type::Body }.font(ui),
        width,
    );
    let visible_subtitle = subtitle
        .rsplit_once(" · ")
        .filter(|(_, folder)| folder.ends_with('/'))
        .map(|(detail, folder)| format!("{folder} · {detail}"));
    let detail = design::words_that_fit(
        ui,
        visible_subtitle.as_deref().unwrap_or(subtitle),
        &Type::Small.font(ui),
        width,
    );
    let response = design::two_line_row(ui, ENTRY_ROW_HEIGHT, selected, &title, &detail);
    design::named(ui, &response, &format!("{name} · {subtitle}"));
    response
}

pub(crate) fn local_module(path: &Path) -> Result<Module, String> {
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
    fields.insert("splat_schema".into(), 2.into());
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
    if kind == "world" {
        fields.remove("splat_world");
        fields.insert("splat_type".into(), "other".into());
    }
    if kind == "creature" {
        fields.insert(
            "splat_creature".into(),
            serde_yaml_ng::from_str::<serde_yaml_ng::Value>(
                "size: medium\ntype: humanoid\nalignment: Neutral\nmovement_text: 30 ft.\ncr_text: '1 (XP 200; PB +2)'\n",
            )
            .expect("creature identity"),
        );
        fields.insert(
            "splat_stats".into(),
            serde_yaml_ng::from_str::<serde_yaml_ng::Value>(
                "initiative: {value: 0}\nspeed: {value: 30, unit: ft}\nstrength: {value: 10}\ndexterity: {value: 10}\nconstitution: {value: 10}\nintelligence: {value: 10}\nwisdom: {value: 10}\ncharisma: {value: 10}\n",
            )
            .expect("creature fields"),
        );
    }
    if kind == "spell" {
        fields.insert("splat_spell".into(), serde_yaml_ng::from_str::<serde_yaml_ng::Value>("level: 1\nschool: evocation\ncasting_time: Action\nrange_text: 60 feet\ncomponents: [verbal, somatic]\nduration: Instantaneous\n").expect("spell fields"));
    }
    if kind == "item" {
        fields.insert(
            "splat_item".into(),
            serde_yaml_ng::from_str::<serde_yaml_ng::Value>(
                "category: wondrous_item\nrarity: common\nequipment: {attunement: none}\n",
            )
            .expect("item fields"),
        );
    }
    if ["creature", "spell", "item"].contains(&kind) {
        fields.insert("splat_schema".into(), 2.into());
        let table = content::stat_blocks::render(&fields, kind);
        for key in content::stat_blocks::data_roots(kind) {
            fields.remove(*key);
        }
        let yaml = serde_yaml_ng::to_string(&fields).expect("string fields");
        return format!(
            "---\n{yaml}---\n\n# {name}\n\n{table}{}",
            if kind == "creature" {
                "## Traits\n\n**Trait.** Describe the trait.\n\n## Actions\n\n**Attack.** Describe the attack.\n\n"
            } else if kind == "spell" {
                "Describe the spell.\n"
            } else {
                "Describe the item and its use.\n"
            }
        );
    }
    if kind == "campaign" {
        fields.remove("splat_campaign");
    }
    let yaml = serde_yaml_ng::to_string(&fields).expect("string fields");
    format!(
        "---\n{yaml}---\n\n# {name}\n\n{}",
        if kind == "campaign" {
            "## Packages\n\n## Rule choices\n\n| Rule | Choice |\n| --- | --- |\n"
        } else if kind == "effect" {
            "## Effect\n\nDescribe what changes and when it ends.\n"
        } else if kind == "rule" {
            "## Rule\n\nDescribe when and how this rule applies.\n"
        } else if kind == "rule_group" {
            "## Rules\n\nDescribe the rules this group enables.\n"
        } else {
            ""
        }
    )
}

/// Consistent type and inset for entry form controls, including the editable Folder.
pub(crate) fn form_control<R>(ui: &mut egui::Ui, add: impl FnOnce(&mut egui::Ui) -> R) -> R {
    ui.scope(|ui| {
        ui.style_mut().override_font_id = Some(Type::Strong.font(ui));
        ui.spacing_mut().button_padding.x = design::S;
        add(ui)
    })
    .inner
}

pub(crate) fn creation_path(ui: &mut egui::Ui, root: &Path, relative: &str, destination: &str) {
    let full = format!("Will create {}", root.join(relative).display());
    let short = format!("Will create {destination}/{relative}");
    let text = design::words_that_fit(ui, &short, &Type::Small.font(ui), ui.available_width());
    let response = ui.label(Type::Small.rich(ui, text).color(colors().muted));
    design::named(ui, &response, &full);
    response.on_hover_text(full);
}

/// Existing folders, with an optional kind folder as the first suggestion.
fn kind_folder(kind: &str) -> &'static str {
    match kind {
        "creature" => "Creatures",
        "spell" => "Spells",
        "item" => "Items",
        "effect" => "Effects",
        _ => "Rules",
    }
}

pub(crate) fn folder_field(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut String,
    root: &Path,
    kind: Option<&str>,
) {
    let mut folders = Vec::new();
    fn visit(root: &Path, path: &Path, folders: &mut Vec<String>) {
        let Ok(entries) = fs::read_dir(path) else {
            return;
        };
        for entry in entries.flatten() {
            if entry.file_type().is_ok_and(|kind| kind.is_dir())
                && !entry.file_name().to_string_lossy().starts_with('.')
            {
                if let Ok(path) = entry.path().strip_prefix(root) {
                    folders.push(path.to_string_lossy().to_string());
                }
                visit(root, &entry.path(), folders);
            }
        }
    }
    visit(root, root, &mut folders);
    if let Some(kind) = kind {
        folders.push(
            match kind {
                "creature" => "Creatures",
                "spell" => "Spells",
                "item" => "Items",
                "effect" => "Effects",
                _ => "Rules",
            }
            .into(),
        );
    }
    folders.sort();
    folders.dedup();
    let query = value.to_lowercase();
    design::field(
        ui,
        label,
        Some("Type a new folder, or choose one. A new folder is made with the file."),
        None,
        |ui| {
            form_control(ui, |ui| {
                egui::Frame::new()
                    .fill(colors().well)
                    .stroke(egui::Stroke::new(
                        design::HAIRLINE_WIDTH,
                        colors().field_border,
                    ))
                    .corner_radius(design::RADIUS_CONTROL as u8)
                    .inner_margin(egui::Margin::symmetric(design::S as i8, 0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let width =
                                (ui.available_width() - design::XL - ui.spacing().item_spacing.x)
                                    .max(design::XXL);
                            let response = ui.add(
                                egui::TextEdit::singleline(value)
                                    .frame(false)
                                    .font(Type::Strong.font(ui))
                                    .margin(egui::Vec2::ZERO)
                                    .desired_width(width)
                                    .min_size(egui::vec2(width, design::XL)),
                            );
                            design::named(ui, &response, label);
                            if value.is_empty() {
                                ui.painter().text(
                                    response.rect.left_center(),
                                    egui::Align2::LEFT_CENTER,
                                    "Main folder",
                                    Type::Strong.font(ui),
                                    colors().text,
                                );
                            }
                            let (rect, response) = ui.allocate_exact_size(
                                egui::vec2(design::XL, design::XL),
                                egui::Sense::click(),
                            );
                            let icon = egui::Align2::RIGHT_CENTER.align_size_within_rect(
                                egui::Vec2::splat(ui.spacing().icon_width),
                                rect,
                            );
                            design::combo_icon(
                                ui,
                                icon,
                                ui.style().interact(&response),
                                egui::Popup::is_id_open(
                                    ui.ctx(),
                                    egui::Popup::default_response_id(&response),
                                ),
                            );
                            design::named(ui, &response, &format!("Suggestions for {label}"));
                            egui::Popup::menu(&response).show(|ui| {
                                if ui
                                    .selectable_label(value.is_empty(), "Main folder")
                                    .clicked()
                                {
                                    value.clear();
                                    ui.close();
                                }
                                for folder in folders
                                    .iter()
                                    .filter(|folder| folder.to_lowercase().contains(&query))
                                {
                                    if ui.selectable_label(value == folder, folder).clicked() {
                                        *value = folder.clone();
                                        ui.close();
                                    }
                                }
                            });
                        });
                    });
            });
        },
    );
}

pub(crate) fn use_checkbox(ui: &mut egui::Ui, not_used: &mut bool, help: &str) {
    let mut enabled = !*not_used;
    ui.checkbox(&mut enabled, "Use in this campaign");
    *not_used = !enabled;
    ui.label(Type::Small.rich(ui, help).color(colors().muted));
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
    if fs::metadata(&path).is_ok_and(|m| m.permissions().readonly())
        || fs::metadata(root).is_ok_and(|m| m.permissions().readonly())
    {
        return Err("This file is read only.".into());
    }
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
    if baseline.is_none() {
        fs::create_dir_all(path.parent().ok_or("Missing parent folder")?)
            .map_err(|e| e.to_string())?;
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
    if let Some(module) = module {
        let path = module.path.canonicalize().map_err(|e| e.to_string())?;
        let (entries, errors) = content::read_directory(&path)?;
        let manifest_path = crate::content_workspace::package_manifest_path(module);
        let missing_manifest = !entries.iter().any(|entry| entry.kind == "package")
            && !path.join(&manifest_path).exists();
        for entry in entries {
            let text = fs::read_to_string(note_path(&module.path, &entry.path)?)
                .map_err(|e| e.to_string())?;
            let mismatched = entry.kind == "package" && entry.id != module.id;
            notes.push(Note {
                path: entry.path.clone(),
                text,
                entry: (!mismatched).then_some(entry),
                error: mismatched.then(|| "Package details have another identity. Replace with the registered details, or restore the original id.".into()),
                owner: Owner::Package(module.clone()),
            });
        }
        if missing_manifest {
            notes.push(Note {
                path: "Package.md".into(),
                text: String::new(),
                entry: None,
                error: Some(crate::content_workspace::DELETED_MANIFEST.into()),
                owner: Owner::Package(module.clone()),
            });
        }
        for error in errors {
            let text =
                fs::read_to_string(note_path(&module.path, &error.path)?).unwrap_or_default();
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
        ..Default::default()
    })
}

/// The whole catalogue: built-in, every package, and the active campaign's own entries.
fn load_all(modules: &[Module], campaign: Option<&Campaign>) -> Result<Loaded, String> {
    let mut all = load(None)?;
    all.owners = modules
        .iter()
        .cloned()
        .map(Owner::Package)
        .chain(campaign.cloned().map(Owner::Campaign))
        .collect();
    for module in modules {
        if let Err(error) = splat::content::art::ArtIndex::default().add_package(&module.path) {
            all.load_errors.push(format!("{}: {error}", module.name));
        }
        match load(Some(module)) {
            Ok(package) => {
                all.notes.extend(package.notes);
            }
            Err(error) => all.load_errors.push(format!("{}: {error}", module.name)),
        }
    }
    if let Some(campaign) = campaign {
        let owner = Owner::Campaign(campaign.clone());
        match content::read_directory(&campaign.path) {
            Ok((entries, errors)) => {
                for entry in entries.into_iter().filter(|e| {
                    e.kind != "campaign" && !splat::character::campaign_package::is_manifest(e)
                }) {
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
                        .unwrap_or_default();
                    all.notes.push(Note {
                        path: error.path,
                        text,
                        entry: None,
                        error: Some(error.message),
                        owner: owner.clone(),
                    });
                }
            }
            Err(error) => all.load_errors.push(format!("{}: {error}", campaign.name)),
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
            Err(error) => all.load_errors.push(format!("{}: {error}", campaign.name)),
        }
    }
    all.notes.sort_by_cached_key(|n| natural_key(n.name()));
    all.refresh_diagnostics();
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

pub(crate) fn catalogue_subtitles(
    catalogue: &crate::content_workspace::Catalogue,
) -> std::collections::HashMap<Uuid, String> {
    let loaded = Loaded {
        notes: catalogue
            .entries
            .values()
            .map(|e| Note {
                path: e.entry.path.clone(),
                text: String::new(),
                entry: Some(e.entry.clone()),
                error: None,
                owner: Owner::Builtin,
            })
            .collect(),
        ..Default::default()
    };
    loaded
        .notes
        .iter()
        .zip(loaded.subtitles())
        .filter_map(|(note, suffix)| Some((note.entry.as_ref()?.id, suffix.clone())))
        .collect()
}

struct SelectionUndo {
    campaign: Campaign,
    before: Vec<crate::content_workspace::PackageSelection>,
    after: Vec<crate::content_workspace::PackageSelection>,
}

struct DependencyUndo {
    root: PathBuf,
    path: String,
    before: String,
    after: String,
    copied_path: String,
    copied_text: String,
}

struct IdentityUndo {
    root: PathBuf,
    path: String,
    before: String,
    after: String,
}

#[derive(Default)]
pub struct Library {
    layouts: crate::formats_ui::State,
    layout_scope: Option<Scope>,
    /// The list should scroll to the selected entry, which arrived by a link.
    reveal_row: bool,
    list_hidden: bool,
    /// The draft `Discard changes` last dropped, with its note's path, for one step of undo.
    /// The note's owner and path, and the discarded words.
    discarded: Option<(Option<Uuid>, String, String)>,
    discarded_note: Option<Note>,
    identity_undo: Option<IdentityUndo>,
    identities_undo: Option<(crate::content_workspace::ContentEdits, String)>,
    manifest_write: Option<crate::content_workspace::ManifestWrite>,
    package_copy: Option<crate::content_workspace::PackageCopy>,
    copy_not_used: bool,
    copy_campaign_before: Option<(Campaign, String, String, String)>,
    unregistered: Option<Module>,
    copied_rosters: Option<crate::content_workspace::ContentEdits>,
    renamed_links: Option<(String, String)>,
    link_edits: Option<crate::content_workspace::ContentEdits>,
    copied_package: Option<Module>,
    copy_name: String,
    copy_name_open: bool,
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
    /// Chosen creature facts. They apply in Creatures and to search results.
    filter: Filter,
    /// The filter that `matches` used.
    matched_filter: Filter,
    section: Section,
    unreadable_kind: Option<Section>,
    error: Option<String>,
    form: bool,
    path: String,
    name: String,
    pub run_open: bool,
    pub open_world_npc: Option<Uuid>,
    pending_edit: Option<Entry>,
    pending_repair: Option<PathBuf>,
    scope_notice: Option<String>,
    new_form: bool,
    focus_new_name: bool,
    focus_hp: bool,
    focus_repair: bool,
    focus_markdown: bool,
    new_name: String,
    new_kind: String,
    new_folder: String,
    new_not_used: bool,
    package_not_used: bool,
    focus_package_name: bool,
    package_path: Option<PathBuf>,
    package_folder_changed: bool,
    pub use_entry: Option<(Uuid, Uuid)>,
    source: bool,
    message: Option<String>,
    reselect: Option<(Option<Uuid>, String)>,
    pub copy: Option<crate::content_workspace::CopySource>,
    pub copy_to_package: Option<Uuid>,
    /// An entry to read beside the open draft, in Lookup.
    pub peek: Option<Uuid>,
    history: Vec<BackStep>,
    dependency_undo: Option<DependencyUndo>,
    selection_undo: Option<SelectionUndo>,
    link_choices: Vec<Uuid>,
    vital_undo: Option<(String, String, String)>,
    heading_undo: Option<(String, String, String)>,
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

fn form_heading(ui: &mut egui::Ui, title: &str) {
    ui.add_space(design::L);
    ui.label(
        Type::Label
            .rich(ui, title.to_uppercase())
            .color(colors().text),
    );
}

fn obsidian_for(root: &Path, owner: &Owner) -> bool {
    matches!(owner, Owner::Campaign(_))
        || root
            .ancestors()
            .any(|parent| parent.join(".obsidian").is_dir())
}
const NOTE_CHANGED: &str = "This entry changed on disk.";
/// What is on disk now, in place of what the draft started from.
struct DiskChange {
    /// None when the file has gone.
    text: Option<String>,
    moved_to: Option<String>,
}

/// Check readable and broken notes, so a move cannot create a second identity.
fn moved_note(root: &Path, id: Uuid, old: &str) -> Result<Option<(String, String)>, String> {
    let (entries, errors) = content::read_directory(root)?;
    let mut found = None;
    for path in entries
        .into_iter()
        .map(|e| e.path)
        .chain(errors.into_iter().map(|e| e.path))
    {
        if path == old {
            continue;
        }
        let text = fs::read_to_string(note_path(root, &path)?).map_err(|e| e.to_string())?;
        if crate::content_workspace::note_id(&text) == Some(id) {
            if found.is_some() {
                return Err(
                    "Several files have this entry's ID. Keep one file, then try again.".into(),
                );
            }
            found = Some((path, text));
        }
    }
    Ok(found)
}
/// Where Back returns to after following a link.
struct BackStep {
    owner: Option<Uuid>,
    path: String,
    scope: Scope,
    section: Section,
    search: String,
}
impl Library {
    pub fn refocus_draft(&mut self) {
        self.focus_markdown = true;
    }
    fn register_copy(&mut self, campaigns: &mut Campaigns) -> Result<(), String> {
        let source = self
            .copied_package
            .clone()
            .ok_or("Choose the copied folder first.")?;
        if campaigns
            .modules()
            .iter()
            .any(|m| m.path.starts_with(&source.path) || source.path.starts_with(&m.path))
        {
            return Err("Choose a separate copied folder first.".into());
        }
        let campaign_document = if !self.copy_not_used {
            campaigns
                .active_campaign()
                .map(|campaign| CampaignDocument::load(&campaign))
                .transpose()?
        } else {
            None
        };
        let copy = crate::content_workspace::PackageCopy::write(&source.path, &self.copy_name)?;
        if let Err(error) = campaigns.register_module(copy.module.clone()) {
            copy.undo()?;
            return Err(error);
        }
        if let Some(mut document) = campaign_document {
            let before = document.text.clone();
            document
                .selections
                .push(crate::content_workspace::PackageSelection {
                    name: Some(copy.module.name.clone()),
                    package: crate::content_workspace::EntryRef::id(copy.module.id),
                    selection: crate::content_workspace::Selection::All,
                });
            if let Err(error) = document.save() {
                let unregister = campaigns.unregister_module(copy.module.id);
                let restore = copy.undo();
                return Err(format!(
                    "Could not use the copy: {error}. {} {}",
                    unregister.err().unwrap_or_default(),
                    restore.err().unwrap_or_default()
                ));
            }
            self.copy_campaign_before =
                Some((document.campaign, document.path, before, document.text));
        }
        self.package_copy = Some(copy);
        self.copied_package = None;
        self.copy_name_open = false;
        self.form = false;
        self.stale = true;
        self.scope = Scope::All;
        self.content_changed = Some(source.path.clone());
        Ok(())
    }

    fn undo_package_registration(&mut self, campaigns: &mut Campaigns) -> Result<(), String> {
        let copy = self
            .package_copy
            .as_ref()
            .ok_or("No package registration to undo.")?;
        let mut extra: Vec<_> = self
            .copy_campaign_before
            .as_ref()
            .map(|(campaign, path, before, after)| {
                (
                    campaign.path.clone(),
                    path.clone(),
                    after.clone(),
                    before.clone(),
                )
            })
            .into_iter()
            .collect();
        if let Some(edits) = &self.copied_rosters {
            extra.extend(edits.inverse_files());
        }
        let edits = copy.undo_with(extra)?;
        if let Err(error) = campaigns.unregister_module(copy.module.id) {
            return Err(match edits.undo() {
                Ok(()) => error,
                Err(restore) => format!("{error}. Could not restore the copy: {restore}"),
            });
        }
        self.content_changed = Some(copy.module.path.clone());
        self.package_copy = None;
        self.copy_campaign_before = None;
        self.copied_rosters = None;
        self.stale = true;
        Ok(())
    }

    fn give_new_identity(&mut self) -> Result<(), String> {
        let index = self.selected.ok_or("Open the copy first.")?;
        let note = &self.loaded.as_ref().unwrap().notes[index];
        note.entry.as_ref().ok_or("Repair this entry first.")?;
        let root = note
            .owner
            .root()
            .ok_or("Built-in entries are read only.")?
            .to_path_buf();
        let before = note.text.clone();
        let id = Uuid::new_v4();
        let after = crate::content_workspace::new_note_identity(&before, &note.path, id)?;
        let updated = write_note(&root, &note.path, Some(&before), &after)?;
        self.identity_undo = Some(IdentityUndo {
            root: root.clone(),
            path: note.path.clone(),
            before,
            after: after.clone(),
        });
        let loaded = self.loaded.as_mut().unwrap();
        loaded.notes[index].entry = Some(updated);
        loaded.notes[index].text = after.clone();
        loaded.invalidate();
        loaded.refresh_diagnostics();
        self.draft = after;
        self.stale = true;
        self.content_changed = Some(root);
        Ok(())
    }

    fn undo_new_identity(&mut self) -> Result<(), String> {
        let undo = self.identity_undo.as_ref().ok_or("Nothing to undo.")?;
        write_note(&undo.root, &undo.path, Some(&undo.after), &undo.before)?;
        self.stale = true;
        self.content_changed = Some(undo.root.clone());
        if let Some(index) = self.selected {
            let note = &mut self.loaded.as_mut().unwrap().notes[index];
            if note.path == undo.path && note.owner.root() == Some(undo.root.as_path()) {
                note.text = undo.before.clone();
                note.entry = Entry::parse(&note.path, &note.text).ok();
                self.draft = note.text.clone();
            }
        }
        self.identity_undo = None;
        self.loaded.as_mut().unwrap().invalidate();
        self.loaded.as_mut().unwrap().refresh_diagnostics();
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn selected_name(&self) -> Option<&str> {
        self.selected
            .and_then(|i| self.loaded.as_ref()?.notes.get(i))
            .map(Note::name)
    }

    pub fn report_error(&mut self, error: String) {
        self.error = Some(error);
    }
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
        if let Some(path) = change.moved_to {
            note.path = path;
        }
        let draft = std::mem::take(&mut self.draft);
        self.discarded = Some((note.owner.id(), note.path.clone(), draft));
        match change.text {
            Some(text) => {
                self.stale = true;
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
            None => {
                self.discarded_note = Some(Note {
                    path: note.path.clone(),
                    text: note.text.clone(),
                    entry: note.entry.clone(),
                    error: note.error.clone(),
                    owner: note.owner.clone(),
                });
                self.draft = note.text.clone();
                self.stale = true;
                self.selected = None;
            }
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
        self.discarded_note = None;
        let Some(note) = self.loaded.as_ref().and_then(|l| l.notes.get(index)) else {
            return;
        };
        if let Some(current) = self
            .selected
            .and_then(|i| self.loaded.as_ref().and_then(|l| l.notes.get(i)))
            .filter(|_| self.selected != Some(index))
        {
            self.history.push(BackStep {
                owner: current.owner.id(),
                path: current.path.clone(),
                scope: self.scope,
                section: self.section,
                search: self.search.clone(),
            });
        }
        if !self.loaded.as_ref().unwrap().in_scope(note, self.scope) {
            self.scope = Scope::All;
            self.scope_notice = Some(format!("Scope is now Everything, to show {}.", note.name()));
        }
        self.layouts.clear();
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
        Some((
            note.name().to_owned(),
            match &note.owner {
                Owner::Package(module) => format!("the {} package", module.name),
                Owner::Campaign(campaign) => format!("the {} campaign", campaign.name),
                Owner::Builtin => "the built-in package".to_owned(),
            },
        ))
    }
    pub fn dirty(&self) -> bool {
        self.loaded
            .as_ref()
            .and_then(|l| self.selected.and_then(|i| l.notes.get(i)))
            .is_some_and(|n| n.text != self.draft)
    }
    pub fn clear_notice(&mut self) {
        self.layouts.leave();
    }

    pub fn form_open(&self) -> bool {
        self.form || self.new_form || self.confirm_unregister.is_some()
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
    pub fn edit_copy(&mut self, entry: Entry) {
        self.pending_edit = Some(entry);
    }
    pub fn repair_file(&mut self, path: PathBuf) {
        self.pending_repair = Some(path);
        self.stale = true;
    }
    pub fn can_save_and_close(&self) -> bool {
        self.dirty()
            && self.disk_change.is_none()
            && self
                .selected
                .and_then(|i| self.loaded.as_ref()?.notes.get(i))
                .is_some_and(|note| Entry::parse(&note.path, &self.draft).is_ok())
    }
    pub fn save_for_close(&mut self) -> bool {
        if !self.can_save_and_close() {
            return false;
        }
        self.save_selected(false);
        !self.dirty()
    }

    pub fn refresh_destination(&mut self, root: &Path, _ctx: &egui::Context) {
        if self.dirty()
            && let Some(note) = self
                .selected
                .and_then(|i| self.loaded.as_ref()?.notes.get(i))
            && note.owner.root() == Some(root)
        {
            let text = note_path(root, &note.path)
                .ok()
                .and_then(|path| fs::read_to_string(path).ok());
            if text.as_deref() != Some(&note.text) {
                let moved = (text.is_none()
                    && self
                        .loaded
                        .as_ref()
                        .unwrap()
                        .duplicates(self.selected.unwrap())
                        .is_empty())
                .then(|| {
                    crate::content_workspace::note_id(&note.text)
                        .and_then(|id| moved_note(root, id, &note.path).ok().flatten())
                })
                .flatten();
                self.disk_change = Some(DiskChange {
                    text: moved.as_ref().map(|(_, text)| text.clone()).or(text),
                    moved_to: moved.map(|(path, _)| path),
                });
            }
        }
        self.discarded = None;
        self.discarded_note = None;
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
    fn draft_reason(&self) -> Option<&'static str> {
        if self.disk_change.is_some() {
            Some("Choose what to do with the changed file first.")
        } else if self.dirty() {
            Some(draft_words(
                self.selected
                    .and_then(|i| self.loaded.as_ref().map(|l| &l.notes[i].owner))
                    .is_some_and(|owner| matches!(owner, Owner::Campaign(_))),
            ))
        } else {
            None
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui, campaigns: &mut Campaigns) {
        ui.ctx().data_mut(|data| {
            data.insert_temp(egui::Id::new("reader_surface"), egui::Id::new("library"))
        });
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
                            for module in campaigns.modules().to_vec() {
                                if let Some(name) = l
                                    .notes
                                    .iter()
                                    .filter(|n| {
                                        n.owner.id() == Some(module.id) && n.error.is_none()
                                    })
                                    .find_map(|n| {
                                        n.entry
                                            .as_ref()
                                            .filter(|entry| entry.kind == "package")
                                            .map(|entry| entry.name.clone())
                                    })
                                    && name != module.name
                                {
                                    let _ = campaigns.rename_module(module.id, name);
                                }
                            }
                            self.selected = self.reselect.as_ref().and_then(|(owner, path)| {
                                l.notes
                                    .iter()
                                    .position(|n| n.owner.id() == *owner && &n.path == path)
                            });
                            if let Some(i) = self.selected {
                                self.draft = l.notes[i].text.clone();
                            }
                            self.reveal_row |= self.selected.is_some();
                            self.focus_repair =
                                self.selected.is_some_and(|i| l.notes[i].error.is_some());
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
        if self.pending.is_none()
            && !self.stale
            && let Some(entry) = self.pending_edit.take()
        {
            if self.reveal(&entry) {
                self.source = true;
            } else {
                self.pending_edit = Some(entry);
            }
        }
        if self.pending.is_none()
            && !self.stale
            && !self.dirty()
            && let Some(path) = self.pending_repair.take()
            && let Some(index) = self.loaded.as_ref().and_then(|loaded| {
                loaded.notes.iter().position(|note| {
                    note.owner
                        .root()
                        .is_some_and(|root| root.join(&note.path) == path)
                })
            })
        {
            self.scope = Scope::All;
            self.search.clear();
            self.section = Section::Unreadable;
            self.unreadable_kind =
                Some(self.loaded.as_ref().unwrap().notes[index].unreadable_kind());
            self.open_entry(index);
            self.source = true;
            self.focus_repair = true;
        }
        let dirty = self.dirty();
        let locked = self.draft_reason().or(self
            .pending
            .is_some()
            .then_some("Wait for the Library to load."));
        // Beside Lookup at the minimum window, everything here gets smaller.
        let compact = ui.available_width() < 900.0 || self.disk_change.is_some();
        let mut open_form = false;
        let mut reload = false;
        let total = self.loaded.as_ref().map_or(0, |loaded| {
            loaded
                .notes
                .iter()
                .filter(|n| loaded.in_scope(n, self.scope))
                .count()
        });
        let mut new_entry = false;
        design::page_header(
            ui,
            "Library",
            (total > 0 && ui.available_width() >= design::READER_MEASURE)
                .then(|| design::count(total, "entry", "entries"))
                .as_deref(),
            |ui| {
                let no_place =
                    campaigns.active_campaign().is_none() && campaigns.modules().is_empty();
                if !(self.scope == Scope::Campaign && total == 0) {
                    new_entry = Action::secondary("New entry")
                        .icon(icons::ADD)
                        .blocked(
                            locked.or(no_place.then_some("Add a campaign or a package first.")),
                        )
                        .show(ui)
                        .clicked();
                }
                design::locked(ui, locked, |ui| {
                    let response = design::menu_button(ui, "Packages", false);
                    egui::Popup::menu(&response).show(|ui| {
                        if Action::quiet("Reload from disk")
                            .icon(icons::REFRESH)
                            .show(ui)
                            .clicked()
                        {
                            reload = true;
                            ui.close();
                        }
                        if Action::quiet("Convert files…")
                            .blocked(
                                locked.or(self
                                    .run_open
                                    .then_some("Close the run before converting files.")),
                            )
                            .show(ui)
                            .clicked()
                        {
                            self.layouts.scan(campaigns);
                            ui.close();
                        }
                        if Action::quiet("Manage packages…")
                            .icon(icons::SETTINGS)
                            .show(ui)
                            .clicked()
                        {
                            open_form = true;
                            ui.close();
                        }
                    });
                });
                if let Some(loaded) = &self.loaded {
                    let old = loaded.old_layout_count();
                    if old > 0
                        && design::locked(
                            ui,
                            locked.or(self
                                .run_open
                                .then_some("Close the run before converting files.")),
                            |ui| {
                                design::status_link(
                                    ui,
                                    Status {
                                        icon: icons::INFO,
                                        word: &design::count(
                                            old,
                                            "file uses the old layout",
                                            "files use the old layout",
                                        ),
                                        color: colors().muted,
                                    },
                                )
                            },
                        )
                        .clicked()
                    {
                        self.layouts.scan(campaigns);
                    }
                }
                if let Some(loaded) = &self.loaded
                    && !loaded.diagnostics.is_empty()
                {
                    let word = design::count(
                        loaded
                            .diagnostics
                            .iter()
                            .map(|message| message.split(": ").next().unwrap_or(message))
                            .collect::<std::collections::HashSet<_>>()
                            .len(),
                        "file needs fixing",
                        "files need fixing",
                    );
                    let response = design::status_link(
                        ui,
                        Status {
                            icon: icons::WARN,
                            word: &word,
                            color: colors().result.warning,
                        },
                    );
                    egui::Popup::menu(&response).show(|ui| {
                        ui.set_max_width(design::READER_MEASURE);
                        egui::ScrollArea::vertical()
                            .max_height(design::XXL * 6.0)
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
                    });
                }
            },
        );
        if new_entry {
            self.new_form = true;
            self.new_folder = kind_folder(&self.new_kind).into();
            self.focus_new_name = true;
            self.form = false;
        }
        let convert_blocked = self.draft_reason().map(str::to_owned).or_else(|| {
            self.run_open
                .then(|| "Close the run before converting files.".into())
        });
        if self.form || self.new_form || self.layout_scope.is_some_and(|scope| scope != self.scope)
        {
            self.layouts.clear();
        }
        if self.pending.is_none()
            && self
                .layouts
                .show_conversion(ui, convert_blocked.as_deref(), campaigns)
        {
            self.stale = true;
            self.content_changed = campaigns.active_campaign().map(|c| c.path.clone());
        }
        if let Some(words) = &self.scope_notice {
            design::message(ui, Tone::Info, words);
        }
        if reload {
            self.start_load(ui.ctx(), campaigns);
        }
        if open_form {
            self.new_form = false;
            self.form = !self.form;
            self.focus_package_name = self.form;
        }
        if campaigns.active_campaign().is_none() {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    Type::Small
                        .rich(
                            ui,
                            if campaigns.all().is_empty() {
                                "No campaign yet. Add a campaign or a package to create entries."
                            } else {
                                "No campaign open. The Library works without one."
                            },
                        )
                        .color(colors().muted),
                );
                if Action::primary(if campaigns.all().is_empty() {
                    "Add campaign"
                } else {
                    "Choose a campaign"
                })
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
            design::message_with_code(ui, Tone::Error, &format!("Error: {e}"));
        }
        if let Some(message) = &self.message {
            design::message(ui, Tone::Success, message);
        }
        if let Some((old, new)) = self.renamed_links.clone()
            && let Some(loaded) = &self.loaded
        {
            let mut files = Vec::new();
            let mut count = 0;
            for note in &loaded.notes {
                if let Some(root) = note.owner.root() {
                    let (after, changed) = crate::world::rename_note_links(&note.text, &old, &new);
                    if changed > 0 {
                        count += changed;
                        files.push((
                            root.to_path_buf(),
                            note.path.clone(),
                            note.text.clone(),
                            after,
                        ));
                    }
                }
            }
            if count > 0
                && design::message_with_action(
                    ui,
                    Tone::Info,
                    &format!("Links still say {old}."),
                    &format!("Update {}", design::count(count, "link", "links")),
                )
            {
                match crate::content_workspace::ContentEdits::write(files) {
                    Ok(edits) => {
                        self.link_edits = Some(edits);
                        self.renamed_links = None;
                        self.stale = true;
                        self.content_changed =
                            campaigns.active_campaign().map(|campaign| campaign.path);
                    }
                    Err(error) => self.error = Some(error),
                }
            }
        }
        if let Some(edits) = &self.link_edits
            && Action::quiet("Undo link update")
                .icon(icons::UNDO)
                .show(ui)
                .clicked()
        {
            match edits.undo() {
                Ok(()) => {
                    self.link_edits = None;
                    self.stale = true;
                    self.content_changed =
                        campaigns.active_campaign().map(|campaign| campaign.path);
                }
                Err(error) => self.error = Some(error),
            }
        }
        if let Some(module) = self.unregistered.clone() {
            let message = format!(
                "{} is unregistered. Its files stay in {}.",
                module.name,
                module.path.display()
            );
            if design::message_with_action(ui, Tone::Info, &message, "Undo unregister") {
                let result = local_module(&module.path).and_then(|found| {
                    if found.id != module.id {
                        return Err("That folder now holds a different package.".into());
                    }
                    campaigns.restore_module(module)
                });
                match result {
                    Ok(()) => {
                        self.unregistered = None;
                        self.stale = true;
                    }
                    Err(error) => self.error = Some(error),
                }
            }
        }
        if self.package_copy.is_some()
            && design::message_with_action(
                ui,
                Tone::Success,
                &format!(
                    "{} is registered · {}",
                    self.package_copy.as_ref().unwrap().module.name,
                    if self.copy_campaign_before.is_some() {
                        "Used here"
                    } else {
                        "Not used here"
                    }
                ),
                "Undo package registration",
            )
            && let Err(error) = self.undo_package_registration(campaigns)
        {
            self.error = Some(error);
        }

        if let Some(copy) = &self.package_copy
            && let Some(campaign) = campaigns.active_campaign()
            && self.copied_rosters.is_none()
            && Action::quiet("Use the copy's entries")
                .blocked(self.draft_reason())
                .show(ui)
                .clicked()
        {
            match copy.use_in_rosters(&campaign) {
                Ok(edits) => {
                    self.copied_rosters = Some(edits);
                    self.content_changed = Some(campaign.path);
                    self.stale = true;
                }
                Err(error) => self.error = Some(error),
            }
        }
        if let Some(edits) = &self.copied_rosters
            && Action::quiet("Undo source change")
                .icon(icons::UNDO)
                .show(ui)
                .clicked()
        {
            match edits.undo() {
                Ok(()) => {
                    self.copied_rosters = None;
                    self.content_changed =
                        campaigns.active_campaign().map(|campaign| campaign.path);
                    self.stale = true;
                }
                Err(error) => self.error = Some(error),
            }
        }
        if self.selection_undo.as_ref().is_some_and(|undo| {
            campaigns
                .active_campaign()
                .is_some_and(|c| c.id == undo.campaign.id)
        }) && design::message_with_action(
            ui,
            Tone::Success,
            "Every entry in this package is now used here.",
            "Undo package use",
        ) {
            let undo = self.selection_undo.take().unwrap();
            let result = crate::content_workspace::CampaignDocument::load(&undo.campaign).and_then(|mut document| {
                if document.selections != undo.after { return Err("Campaign choices changed. Keep them and edit the package choices in Campaign.".into()); }
                document.selections = undo.before;
                document.save()
            });
            match result {
                Ok(()) => {
                    self.stale = true;
                    self.content_changed = Some(undo.campaign.path);
                }
                Err(error) => self.error = Some(error),
            }
        }
        if self.pending.is_some() {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
                ui.label("Loading the Library…");
            });
            return;
        }

        let Some(loaded) = &self.loaded else {
            return;
        };
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
        let empty_scope = scope == Scope::Campaign && count_of(Section::All, &counts) == 0;
        if empty_scope {
            ui.scope(|ui| {
                ui.set_max_width(166.0);
                Self::scope_menu(
                    ui,
                    compact,
                    campaigns,
                    &mut self.scope,
                    &mut self.section,
                    &mut self.search,
                );
            });
            if self.new_form {
                self.new_entry(ui, dirty, campaigns);
            } else {
                let mut can_use = false;
                design::empty_state_with_body(
                    ui,
                    icons::LIBRARY,
                    "No entries of your own yet",
                    |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.label(Type::Body.rich(ui, "Copy an ").color(colors().muted));
                            can_use = design::link(ui, "entry you can use").clicked();
                            ui.label(
                                Type::Body
                                    .rich(ui, ", or make a new one here.")
                                    .color(colors().muted),
                            );
                        });
                    },
                    |ui| {
                        if Action::primary("New entry")
                            .icon(icons::ADD)
                            .show(ui)
                            .clicked()
                        {
                            self.new_form = true;
                            self.new_folder = kind_folder(&self.new_kind).into();
                        }
                    },
                );
                if can_use {
                    self.scope = Scope::Uses;
                }
            }
            return;
        }
        if !compact && self.disk_change.is_none() {
            design::Column::left("library_sections", 166.0, 150.0..=280.0)
                // The drag handle is on this edge, so the rows stop short of it.
                .frame(egui::Frame::new().inner_margin(egui::Margin {
                    right: design::S as i8,
                    ..Default::default()
                }))
                .show_inside(ui, |ui| {
                    Self::scope_menu(
                        ui,
                        compact,
                        campaigns,
                        &mut self.scope,
                        &mut self.section,
                        &mut self.search,
                    );
                    if let Scope::Package(package) = scope
                        && let Some((index, note)) = loaded
                            .notes
                            .iter()
                            .enumerate()
                            .find(|(_, n)| n.owner.id() == Some(package) && n.is_manifest())
                        && design::link(ui, &note.about_label()).clicked()
                        && !dirty
                    {
                        self.selected = Some(index);
                        self.draft = note.text.clone();
                        self.source = false;
                    }
                    egui::ScrollArea::vertical()
                        .id_salt("section_list")
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            for &(group, sections) in Section::GROUPS {
                                // Keep the empty encounter section discoverable, but hide an unused fallback.
                                if sections
                                    .iter()
                                    .all(|section| count_of(*section, &counts) == 0)
                                {
                                    continue;
                                }
                                if !group.is_empty() {
                                    ui.add_space(design::M);
                                    design::group_heading(
                                        ui,
                                        group,
                                        sections
                                            .iter()
                                            .map(|section| count_of(*section, &counts))
                                            .sum(),
                                    );
                                    ui.add_space(design::XS);
                                }
                                for &section in sections {
                                    let count = count_of(section, &counts);
                                    if count == 0 {
                                        continue;
                                    }
                                    if section == Section::Unreadable {
                                        for kind in Section::GROUPS
                                            .iter()
                                            .flat_map(|(_, sections)| sections.iter())
                                            .copied()
                                        {
                                            let count = loaded
                                                .notes
                                                .iter()
                                                .filter(|note| {
                                                    note.error.is_some()
                                                        && loaded.in_scope(note, scope)
                                                        && note.unreadable_kind() == kind
                                                })
                                                .count();
                                            if count == 0 {
                                                continue;
                                            }
                                            let label = if kind == Section::Unreadable {
                                                "Entries"
                                            } else {
                                                kind.label()
                                            };
                                            if section_row(
                                                ui,
                                                label,
                                                count,
                                                self.section == section
                                                    && self.unreadable_kind == Some(kind)
                                                    && !searching,
                                            )
                                            .clicked()
                                            {
                                                self.section = section;
                                                self.unreadable_kind = Some(kind);
                                                self.matches = None;
                                                self.search.clear();
                                            }
                                        }
                                        continue;
                                    }
                                    let selected = self.section == section && !searching;
                                    let label = section.label();
                                    let response = section_row(ui, label, count, selected);
                                    if response.clicked() {
                                        self.section = section;
                                        self.unreadable_kind = None;
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
                .is_some_and(|(_, prior, _)| *prior != query)
        {
            self.scope_notice = None;
        }
        if self.matches_scope != Some(scope)
            || self
                .matches
                .as_ref()
                .is_none_or(|(section, cached_query, _)| {
                    *section != self.section || *cached_query != query
                })
            || self.matched_filter != self.filter
        {
            self.matched_filter = self.filter.clone();
            self.reveal_row |= self.matches_scope != Some(scope);
            self.matches_scope = Some(scope);
            let parsed = Query::parse(&query);
            let facets = loaded.facets();
            let filtering = (self.section == Section::Creatures || !query.is_empty())
                && !self.filter.is_empty();
            let mut found: Vec<usize> = loaded
                .notes
                .iter()
                .enumerate()
                .filter(|(i, n)| {
                    loaded.in_scope(n, scope)
                        && n.matches(self.section, &parsed, facets[*i].as_ref())
                        && (!filtering
                            || facets[*i].as_ref().is_some_and(|f| self.filter.matches(f)))
                        && (!query.is_empty()
                            || self.section != Section::Unreadable
                            || self
                                .unreadable_kind
                                .is_none_or(|kind| n.unreadable_kind() == kind))
                })
                .map(|(i, _)| i)
                .collect();
            if !parsed.words.is_empty() {
                // Best first: the exact name, then a prefix, then a whole word.
                found.sort_by_key(|i| {
                    let name = loaded.notes[*i].name().to_lowercase();
                    let rank = crate::lookup::name_rank(&name, &parsed.words).unwrap_or(4);
                    (
                        rank,
                        crate::lookup::entry_origin(
                            loaded.notes[*i].entry.as_ref(),
                            matches!(loaded.notes[*i].owner, Owner::Campaign(_)),
                        ),
                    )
                });
            }
            self.matches = Some((self.section, query.clone(), found));
        }
        let matches = &self.matches.as_ref().unwrap().2;
        // Keep the reader in sync with the visible list without dropping an unsaved draft.
        let mut select = None;
        // The whole catalogue has no first entry worth opening for the DM.
        let whole_catalogue = self.section == Section::All && query.is_empty();
        if !dirty
            && self.selected.is_none_or(|i| {
                !matches.contains(&i)
                    && loaded.notes[i]
                        .entry
                        .as_ref()
                        .is_none_or(|e| e.kind != "package")
            })
        {
            select = matches.first().copied().filter(|_| !whole_catalogue);
            if select.is_none() {
                self.selected = None;
                self.draft.clear();
            }
        }
        let selected = select.or(self.selected);
        let subtitles = loaded.subtitles();
        let link_index = ui.ctx().data(|data| {
            data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                "reader_link_index",
            ))
        });
        // A wide window's list gets room too, so that long names keep their second line.
        let min_list_width =
            (design::number_width(ui, "Can use", &Type::Strong.font(ui)) + design::XXL + design::L)
                * 2.0
                + design::S * 2.0
                + ui.spacing().item_spacing.x;
        let list_width = (ui.available_width() * 0.38).clamp(min_list_width, 420.0);
        let mut peek = None;
        // Beside Lookup at the minimum window, a draft needs the room more than the list.
        let hide_list =
            self.disk_change.is_some() || (ui.available_width() < 480.0 && (dirty || self.source));
        self.list_hidden = hide_list;
        if !hide_list {
            design::Column::left("library_entries", list_width, min_list_width..=560.0)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: design::S as i8,
                right: design::S as i8,
                top: 0,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                if compact {
                    let half = (ui.available_width() - ui.spacing().item_spacing.x) / 2.0;
                    ui.horizontal_top(|ui| {
                        ui.allocate_ui_with_layout(egui::vec2(half, 0.0), egui::Layout::top_down(egui::Align::Min), |ui| {
                            Self::scope_menu(ui, compact, campaigns, &mut self.scope, &mut self.section, &mut self.search);
                        });
                        let combo = egui::ComboBox::from_id_salt("library_section")
                            .icon(design::combo_icon).width(half).truncate()
                            .selected_text(match self.section { Section::All => "All", Section::Characters => "Characters", _ => self.section.label() })
                            .show_ui(ui, |ui| {
                                for &(_, sections) in Section::GROUPS {
                                    for &section in sections {
                                        if count_of(section, &counts) == 0 { continue; }
                                        let response = ui.selectable_label(self.section == section, section.label());
                                        design::named(ui, &response, &format!("Library section: {}", section.label()));
                                        if response.clicked() { self.section = section; self.search.clear(); }
                                    }
                                }
                            });
                        design::named(ui, &combo.response, "Section");
                    });
                    if let Scope::Package(package) = scope
                        && let Some((index, note)) = loaded.notes.iter().enumerate().find(|(_, n)| n.owner.id() == Some(package) && n.is_manifest())
                        && design::link(ui, &note.about_label()).clicked() && !dirty
                    { self.selected = Some(index); self.draft = note.text.clone(); self.source = false; }
                }
                ui.horizontal(|ui| {
                    let width =
                        ui.available_width() - if self.search.is_empty() { 0.0 } else { Action::quiet("Clear").density(Density::Scan).natural_width(ui) + ui.spacing().item_spacing.x };
                    design::search_field(
                        ui,
                        SEARCH_NAME,
                        &mut self.search,
                        "Search by name or detail…",
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
                if self.section == Section::Creatures || searching {
                    let notes = &loaded.notes;
                    let facets = loaded.facets();
                    let changed = crate::creature_filter::filter_bar(ui, &mut self.filter, || {
                        notes
                            .iter()
                            .zip(facets)
                            .filter(|(note, _)| loaded.in_scope(note, scope))
                            .filter_map(|(_, facets)| facets.as_ref())
                            .collect()
                    });
                    if changed {
                        ui.ctx().request_repaint();
                    }
                }
                ui.add_space(design::XS);
                if searching || (compact && self.section != Section::All) {
                    design::group_heading(ui, if searching { "Search results" } else { self.section.label() }, matches.len());
                }
                if matches.is_empty() {
                    ui.add_space(design::S);
                    if let Scope::Package(id) = scope && !searching
                        && !loaded.notes.iter().any(|note| note.owner.id() == Some(id) && note.entry.as_ref().is_some_and(|entry| entry.kind != "package")) {
                        let name = campaigns.modules().iter().find(|m| m.id == id).map_or("this package", |m| m.name.as_str());
                        let mut create = false;
                        if self.new_form {
                            ui.label(Type::Title.rich(ui, icons::LIBRARY).color(colors().muted));
                            ui.label(Type::Title.rich(ui, format!("No entries in {name} yet.")).color(colors().text));
                        } else {
                            design::empty_state(ui, icons::LIBRARY, &format!("No entries in {name} yet."), "Create the first entry.", |ui| {
                                create = Action::secondary("New entry").icon(icons::ADD).show(ui).clicked();
                            });
                        }
                        if create { self.new_form = true; self.focus_new_name = true; self.new_kind = "creature".into(); self.new_folder = "Creatures".into(); }
                    } else {
                    let filtered = !self.filter.is_empty() && (searching || self.section == Section::Creatures);
                    let text = if filtered {
                        "No creatures match these filters. Remove a filter, or try Everything."
                    } else if scope == Scope::Campaign && !searching {
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
                            let line = &subtitles[i];
                            let subtitle = match (&note.owner, scope) {
                                (Owner::Campaign(_), Scope::Campaign) => line.strip_suffix(" · Campaign").unwrap_or(line),
                                (Owner::Package(module), Scope::Package(id)) if module.id == id => line.strip_suffix(&format!(" · {}", module.name)).unwrap_or(line),
                                _ => line,
                            };
                            let broken = note.entry.as_ref().zip(link_index.as_ref()).filter(|_| note.owner != Owner::Builtin).map_or(0, |(entry, index)| index.broken_link_count(&entry.body, entry.id));
                            let faults = note.entry.as_ref().map_or(0, |entry| crate::roll_table::fault_count(&entry.body));
                            let subtitle = if faults > 0 {
                                format!("{} {} · {subtitle}", icons::WARN, design::count(faults, "table fault", "table faults"))
                            } else if broken > 0 {
                                format!("{} {} · {subtitle}", icons::WARN, design::count(broken, "broken link", "broken links"))
                            } else { subtitle.to_owned() };
                            let response = ui
                                .push_id(i, |ui| {
                                    entry_row(
                                        ui,
                                        note.name(),
                                        &subtitle,
                                        selected == Some(i),
                                    )
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
                left: if self.disk_change.is_some() {
                    0
                } else {
                    design::L as i8
                },
                right: 0,
                top: 0,
                bottom: 0,
            }))
            .show_inside(ui, |ui| {
                if self.form {
                    self.package_form(ui, campaigns);
                } else if self.new_form {
                    self.new_entry(ui, dirty, campaigns);
                } else {
                    self.editor(ui, &subtitle, campaigns);
                }
            });
    }

    /// What the catalogue shows. Choosing here never changes the active campaign.
    fn scope_menu(
        ui: &mut egui::Ui,
        compact: bool,
        campaigns: &Campaigns,
        current: &mut Scope,
        section: &mut Section,
        _search: &mut String,
    ) {
        let active = campaigns.active_campaign();
        let label = |scope: Scope| -> String {
            match scope {
                Scope::Uses => "Can use".to_owned(),
                Scope::All => "Everything".to_owned(),
                Scope::Builtin => BUILTIN_NAME.to_owned(),
                Scope::Package(id) => campaigns
                    .modules()
                    .iter()
                    .find(|m| m.id == id)
                    .map_or_else(|| "A package".to_owned(), |m| m.name.clone()),
                Scope::Campaign => "Campaign entries".to_owned(),
            }
        };
        let mut scope = if active.is_none() && matches!(*current, Scope::Uses | Scope::Campaign) {
            Scope::All
        } else {
            *current
        };
        let before = scope;
        // This control moves between columns as the reader narrows. Keep an open
        // menu attached to it when the window changes size.
        ui.scope_builder(egui::UiBuilder::new().id("library_scope_host"), |ui| {
            let width =
                ui.available_width()
                    .min(if compact { ui.available_width() } else { 166.0 });
            let selected = match scope {
                Scope::Uses => "Can use".to_owned(),
                Scope::Builtin => "D&D 2024".to_owned(),
                Scope::Campaign => "Own entries".to_owned(),
                _ => label(scope),
            };
            let budget = width - design::XXL - design::L;
            let fitted = scope_label(ui, &selected, budget);
            let combo = egui::ComboBox::from_id_salt("library_scope")
                .icon(design::combo_icon)
                .width(width)
                .truncate()
                .selected_text(Type::Strong.rich(ui, fitted))
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
            combo.response.clone().on_hover_text(label(scope));
            design::named(
                ui,
                &combo.response,
                &format!("What the Library shows: {}", label(scope)),
            );
        });
        *current = scope;
        if scope != before {
            // The selected entry can stay if it is in this scope. A draft always stays.
            *section = Section::All;
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
                .filter(|n| {
                    n.owner.id() == Some(module.id)
                        && n.entry
                            .as_ref()
                            .is_none_or(|e| !matches!(e.kind.as_str(), "package" | "reference"))
                })
                .count()
        });
        let mut keep = ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        let mut remove = false;
        let title = format!("Unregister {}?", module.name);
        design::dialog(ctx, "unregister_package", &title, |ui| {
            ui.add(
                egui::Label::new(format!(
                    "{} leave Splat's lists, and any campaign that selects them can no longer use them.", design::count(count, "entry", "entries")
                ))
                .wrap(),
            );
            if let Some(campaign) = campaigns.active_campaign()
                && let Ok(catalogue) =
                    crate::content_workspace::Catalogue::load(campaigns.modules(), Some(&campaign))
            {
                let entry_ids: Vec<_> = catalogue
                    .entries
                    .values()
                    .filter(|entry| {
                        entry.owner == module.id && catalogue.selectable(entry.entry.id)
                    })
                    .map(|entry| crate::content_workspace::EntryRef::id(entry.entry.id))
                    .collect();
                let selection = crate::content_workspace::PackageSelection {
                    name: Some(module.name.clone()),
                    package: crate::content_workspace::EntryRef::id(module.id),
                    selection: crate::content_workspace::Selection::Selected { entries: entry_ids },
                };
                let users = selection.campaign_users(&catalogue);
                if !users.is_empty() {
                    let mut names = Vec::new();
                    for (id, _) in &users {
                        if let Some(user) = catalogue.entries.get(id) {
                            for reference in
                                content::required_references(&user.entry).unwrap_or_default()
                            {
                                if let Ok(id) = catalogue.resolve(&reference)
                                    && let Some(entry) = catalogue.entries.get(&id)
                                    && entry.owner == module.id
                                {
                                    names.push(entry.entry.name.clone());
                                }
                            }
                        }
                    }
                    names.sort();
                    names.dedup();
                    let users: Vec<_> = users.into_iter().map(|(_, name)| name).collect();
                    ui.label(format!(
                        "{} uses {}, in {}.",
                        campaign.name,
                        name_list(&names),
                        name_list(&users)
                    ));
                }
            }
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
                    self.unregistered = Some(module);
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
        let locked = self.draft_reason().or(self
            .pending
            .is_some()
            .then_some("Wait for the Library to load."));
        if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            self.form = false;
        }
        egui::ScrollArea::vertical()
            .id_salt("package_form")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
                ui.add_space(design::L);
                ui.horizontal(|ui| {
                    ui.label(Type::Label.rich(ui, "MANAGE PACKAGES").color(colors().text));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if Action::quiet("Close")
                            .name("Close the package form")
                            .show(ui)
                            .clicked()
                        {
                            self.form = false;
                        }
                    });
                });
                form_heading(ui, "Add a package folder");
                ui.horizontal(|ui| {
                    design::text_input(
                        ui,
                        "Package folder",
                        &mut self.path,
                        "Choose a folder",
                        (ui.available_width()
                            - Action::secondary("Browse…").natural_width(ui)
                            - ui.spacing().item_spacing.x)
                            .max(design::XXL),
                    );
                    if Action::secondary("Browse…")
                        .blocked(locked)
                        .show(ui)
                        .clicked()
                        && let Some(path) = crate::hermetic::pick_folder()
                    {
                        self.path = path.display().to_string();
                    }
                });
                ui.horizontal(|ui| {
                    if Action::secondary("Add folder")
                        .blocked(
                            locked.or(self
                                .path
                                .trim()
                                .is_empty()
                                .then_some("Choose a folder first.")),
                        )
                        .show(ui)
                        .clicked()
                    {
                        match local_module(Path::new(self.path.trim())).and_then(|module| {
                            if let Some(original) = campaigns
                                .modules()
                                .iter()
                                .find(|m| m.id == module.id && m.path != module.path)
                            {
                                self.copied_package = Some(module);
                                self.copy_name = format!("{} copy", original.name);
                                self.copy_name_open = false;
                                self.error = None;
                                return Ok(None);
                            }
                            campaigns.register_module(module.clone())?;
                            Ok(Some(module))
                        }) {
                            Ok(Some(module)) => {
                                self.scope = Scope::Package(module.id);
                                self.form = false;
                                self.stale = true;
                            }
                            Ok(None) => {}
                            Err(error) => self.error = Some(error),
                        }
                    }
                    design::blocked_reason(
                        ui,
                        self.path
                            .trim()
                            .is_empty()
                            .then_some("Choose a folder first."),
                    );
                    ui.add_space(design::L);
                });

                if let Some(copy) = self.copied_package.clone() {
                    if Path::new(self.path.trim()).canonicalize().ok().as_ref() != Some(&copy.path)
                    {
                        self.copied_package = None;
                    } else {
                        let name = campaigns
                            .modules()
                            .iter()
                            .find(|m| m.id == copy.id)
                            .map_or(copy.name.as_str(), |m| m.name.as_str());
                        design::message(
                            ui,
                            Tone::Warning,
                            &format!("This folder is a copy of {name}."),
                        );
                        design::quiet_section(ui, "Technical details", |ui| {
                            ui.label(copy.id.to_string());
                        });
                        if self.copy_name_open {
                            design::text_field(
                                ui,
                                "New package name",
                                &mut self.copy_name,
                                None,
                                None,
                            );
                            if campaigns.active_campaign().is_some() {
                                use_checkbox(
                                    ui,
                                    &mut self.copy_not_used,
                                    "Uses every entry in this package, including future entries.",
                                );
                            }
                        }
                        let label = if self.copy_name_open && self.copy_name.trim().is_empty() {
                            "Register package".into()
                        } else if self.copy_name_open {
                            format!("Register {}", self.copy_name.trim())
                        } else {
                            "Register as a new package".into()
                        };
                        let reason = locked
                            .or((self.copy_name_open && self.copy_name.trim().is_empty())
                                .then_some("Type a name first."));
                        ui.horizontal_wrapped(|ui| {
                            if Action::quiet(&label).blocked(reason).show(ui).clicked() {
                                if self.copy_name_open {
                                    if let Err(error) = self.register_copy(campaigns) {
                                        self.error = Some(error);
                                    }
                                } else {
                                    self.copy_not_used = campaigns
                                        .active_campaign()
                                        .and_then(|campaign| CampaignDocument::load(&campaign).ok())
                                        .is_some_and(|document| {
                                            document.selections.iter().any(|selection| {
                                                selection.package.id == Some(copy.id)
                                            })
                                        });
                                    self.copy_name_open = true;
                                }
                            }
                            design::blocked_reason(ui, reason);
                        });
                    }
                }

                if !self.copy_name_open {
                    form_heading(ui, "New package");
                    let response = design::text_field(ui, "Name", &mut self.name, None, None);
                    if std::mem::take(&mut self.focus_package_name) {
                        response.request_focus();
                    }
                    let parent = self.package_path.get_or_insert_with(|| {
                        campaigns
                            .modules()
                            .last()
                            .and_then(|module| module.path.parent())
                            .map(Path::to_path_buf)
                            .or_else(|| {
                                campaigns
                                    .package_folder("new_package")
                                    .ok()
                                    .and_then(|p| p.parent().map(Path::to_path_buf))
                            })
                            .unwrap_or_default()
                    });
                    ui.horizontal_top(|ui| {
                        let width = ui.available_width()
                            - Action::secondary("Browse…").natural_width(ui)
                            - ui.spacing().item_spacing.x;
                        ui.allocate_ui_with_layout(
                            egui::vec2(width, 0.0),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                let mut path = parent.display().to_string();
                                if design::text_field(ui, "Folder", &mut path, None, None).changed()
                                {
                                    *parent = PathBuf::from(path);
                                    self.package_folder_changed = true;
                                }
                            },
                        );
                        ui.vertical(|ui| {
                            ui.add_space(design::L + design::XS);
                            if Action::secondary("Browse…")
                                .name("Choose new package folder")
                                .show(ui)
                                .clicked()
                                && let Some(chosen) = crate::hermetic::pick_folder()
                            {
                                *parent = chosen;
                                self.package_folder_changed = true;
                            }
                        });
                    });
                    let namespace: String = self
                        .name
                        .trim()
                        .to_lowercase()
                        .chars()
                        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                        .collect();
                    let root = self
                        .package_path
                        .as_ref()
                        .map(|parent| Ok(parent.join(&namespace)))
                        .unwrap_or_else(|| {
                            campaigns
                                .modules()
                                .last()
                                .and_then(|module| module.path.parent())
                                .map(|parent| Ok(parent.join(&namespace)))
                                .unwrap_or_else(|| campaigns.package_folder(&namespace))
                        });
                    if let Ok(root) = &root
                        && !self.name.trim().is_empty()
                    {
                        ui.add(
                            egui::Label::new(
                                Type::Small
                                    .rich(ui, format!("Will create {}", root.display()))
                                    .color(c.muted),
                            )
                            .wrap(),
                        );
                    }
                    if campaigns.active_campaign().is_some() {
                        use_checkbox(
                            ui,
                            &mut self.package_not_used,
                            "Uses every entry in this package, including future entries.",
                        );
                    }
                    let reason =
                        locked.or(self.name.trim().is_empty().then_some("Type a name first."));
                    ui.horizontal(|ui| {
                        if (if self.run_open {
                            Action::secondary("Create package")
                        } else {
                            Action::primary("Create package")
                        })
                        .blocked(reason)
                        .show(ui)
                        .clicked()
                        {
                            let namespace: String = self
                                .name
                                .trim()
                                .to_lowercase()
                                .chars()
                                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                                .collect();
                            match root
                                .and_then(|root| {
                                    fs::create_dir_all(
                                        root.parent().ok_or("No package folder is available.")?,
                                    )
                                    .map_err(|e| e.to_string())?;
                                    create_module(&root, &self.name, &namespace)
                                })
                                .and_then(|module| {
                                    campaigns.register_module(module.clone())?;
                                    if !self.package_not_used
                                        && let Some(campaign) = campaigns.active_campaign()
                                    {
                                        crate::content_workspace::use_in_campaign(
                                            &campaign,
                                            module.id,
                                            &module.name,
                                            None,
                                        )?;
                                    }
                                    Ok(module)
                                }) {
                                Ok(module) => {
                                    self.content_changed =
                                        campaigns.active_campaign().map(|c| c.path);
                                    self.scope = Scope::Package(module.id);
                                    self.form = false;
                                    self.new_form = true;
                                    self.new_place = None;
                                    self.new_name.clear();
                                    self.new_folder = kind_folder(&self.new_kind).into();
                                    self.new_not_used = false;
                                    self.focus_new_name = true;
                                    self.stale = true;
                                }
                                Err(error) => self.error = Some(error),
                            }
                        }
                        design::blocked_reason(ui, reason);
                        ui.add_space(design::L);
                        if (!self.name.is_empty() || self.package_folder_changed)
                            && Action::quiet("Clear")
                                .name("Clear the new package fields")
                                .show(ui)
                                .clicked()
                        {
                            self.name.clear();
                            self.package_path = None;
                            self.package_folder_changed = false;
                        }
                    });
                }
                form_heading(ui, "Registered packages");
                design::rule(ui);
                if campaigns.modules().is_empty() {
                    ui.label(Type::Body.rich(ui, "None yet.").color(c.muted));
                }
                for module in campaigns.modules().to_vec() {
                    ui.push_id(module.id, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing.y = 0.0;
                                ui.set_max_width((ui.available_width() - design::XXL).max(120.0));
                                ui.label(Type::Strong.rich(ui, &module.name).color(c.text));
                                ui.add(
                                    egui::Label::new(
                                        Type::Small
                                            .rich(
                                                ui,
                                                module
                                                    .path
                                                    .display()
                                                    .to_string()
                                                    .replace('/', "/\u{200b}"),
                                            )
                                            .color(c.muted),
                                    )
                                    .wrap(),
                                );
                            });
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    design::row_menu(ui, &module.name, |ui| {
                                        if Action::destructive("Unregister…")
                                            .density(Density::Read)
                                            .name(&format!("Unregister {}", module.name))
                                            .blocked(locked)
                                            .show(ui)
                                            .clicked()
                                        {
                                            self.confirm_unregister = Some(module.clone());
                                            ui.close();
                                        }
                                    });
                                },
                            );
                        });
                    });
                    design::rule(ui);
                }
            });
    }

    fn new_entry(&mut self, ui: &mut egui::Ui, dirty: bool, campaigns: &Campaigns) {
        if self.new_kind.is_empty() {
            self.new_kind = "creature".into();
            self.new_folder = kind_folder(&self.new_kind).into();
        }
        if self.loaded.is_none() {
            return;
        }
        if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
            self.new_form = false;
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
        ui.vertical(|ui| {
            ui.set_max_width(ui.available_width().min(design::FORM_WIDTH));
            form_heading(ui, "New entry");
            ui.label(
                Type::Small
                    .rich(ui, "Creates the entry, then opens it in the editor.")
                    .color(colors().muted),
            );
            ui.vertical(|ui| {
                let response = design::text_field(ui, "Entry name", &mut self.new_name, None, None);
                if std::mem::take(&mut self.focus_new_name) {
                    response.request_focus();
                }
                if self.new_kind.is_empty() {
                    self.new_kind = "creature".into();
                }
                design::group_label(ui, "Kind");
                let old_kind = self.new_kind.clone();
                let combo = form_control(ui, |ui| {
                    egui::ComboBox::from_id_salt("new_kind")
                        .width(ui.available_width())
                        .icon(design::combo_icon)
                        .selected_text(
                            self.new_kind
                                .replace('_', " ")
                                .chars()
                                .enumerate()
                                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                                .collect::<String>(),
                        )
                        .show_ui(ui, |ui| {
                            for kind in
                                ["creature", "spell", "item", "effect", "rule", "rule_group"]
                            {
                                let word = kind.replace('_', " ");
                                let word = format!("{}{}", word[..1].to_uppercase(), &word[1..]);
                                ui.selectable_value(&mut self.new_kind, kind.into(), word);
                            }
                        })
                });
                if self.new_kind != old_kind {
                    self.new_folder = kind_folder(&self.new_kind).into();
                }
                design::named(ui, &combo.response, "Kind of entry");
                design::group_label(ui, "Destination");
                let combo = form_control(ui, |ui| {
                    egui::ComboBox::from_id_salt("new_place")
                        .icon(design::combo_icon)
                        .width(ui.available_width())
                        .truncate()
                        .selected_text(places[place].label())
                        .show_ui(ui, |ui| {
                            for (i, owner) in places.iter().enumerate() {
                                ui.selectable_value(&mut place, i, owner.label());
                            }
                        })
                });
                design::named(ui, &combo.response, "Destination");
                folder_field(
                    ui,
                    "Folder",
                    &mut self.new_folder,
                    places[place].root().unwrap(),
                    Some(&self.new_kind),
                );
                let relative = if self.new_folder.trim().is_empty() {
                    format!("{}.md", self.new_name.trim())
                } else {
                    format!(
                        "{}/{}.md",
                        self.new_folder.trim().trim_end_matches('/'),
                        self.new_name.trim()
                    )
                };
                if !self.new_name.trim().is_empty() {
                    let destination = match &places[place] {
                        Owner::Package(package) => &package.name,
                        Owner::Campaign(campaign) => &campaign.name,
                        Owner::Builtin => BUILTIN_NAME,
                    };
                    creation_path(ui, places[place].root().unwrap(), &relative, destination);
                }
                if matches!(places[place], Owner::Package(_))
                    && campaigns.active_campaign().is_some()
                {
                    use_checkbox(
                        ui,
                        &mut self.new_not_used,
                        "Adds this entry to the campaign's package choices.",
                    );
                }
                let blocked = if dirty {
                    Some("Save or discard this entry first.")
                } else if self.new_name.trim().is_empty() {
                    Some("Type a name first.")
                } else {
                    None
                };
                ui.horizontal(|ui| {
                    let create = Action::primary("Create entry")
                        .blocked(blocked)
                        .show(ui)
                        .clicked();
                    design::blocked_reason(ui, blocked);
                    ui.add_space(design::L);
                    if Action::quiet("Cancel").show(ui).clicked() {
                        self.new_form = false;
                    }
                    if create {
                        let name = self.new_name.trim();
                        if name.contains(['/', '\\', '\n', '\r']) || name == "." || name == ".." {
                            self.error =
                                Some("Use a name without path separators or line breaks.".into());
                            return;
                        }
                        let path = relative.clone();
                        let text = note_text(name, &self.new_kind, None);
                        let owner = places[place].clone();
                        let root = owner.root().expect("a writable owner").to_path_buf();
                        match write_note(&root, &path, None, &text) {
                            Ok(entry) => {
                                let used =
                                    if !self.new_not_used && matches!(owner, Owner::Package(_)) {
                                        campaigns
                                            .active_campaign()
                                            .map(|campaign| {
                                                crate::content_workspace::use_in_campaign(
                                                    &campaign,
                                                    owner.id().unwrap(),
                                                    owner.source(),
                                                    Some(entry.id),
                                                )
                                            })
                                            .transpose()
                                    } else {
                                        Ok(None)
                                    };
                                let l = self.loaded.as_mut().unwrap();
                                if !self.new_not_used
                                    && used.is_ok()
                                    && let Some(uses) = &mut l.uses
                                {
                                    uses.insert(entry.id);
                                }
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
                                self.reveal_row = true;
                                self.search.clear();
                                self.section = Section::All;
                                self.source = true;
                                self.draft = text;
                                self.focus_hp = self.new_kind == "creature";
                                self.new_name.clear();
                                self.new_folder.clear();
                                self.new_form = false;
                                self.error = used.err().map(|error| {
                                    format!(
                                        "Entry created. Could not use it in this campaign: {error}"
                                    )
                                });
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
        });
        self.new_place = (place != default_place).then_some(place);
    }

    fn go_back(&mut self) {
        if self.dirty() {
            return;
        }
        let Some(step) = self.history.pop() else {
            return;
        };
        let Some(index) = self.loaded.as_ref().and_then(|l| {
            l.notes
                .iter()
                .position(|n| n.owner.id() == step.owner && n.path == step.path)
        }) else {
            return;
        };
        self.selected = Some(index);
        self.reveal_row = true;
        self.draft = self.loaded.as_ref().unwrap().notes[index].text.clone();
        self.disk_change = None;
        self.scope_notice = None;
        self.scope = step.scope;
        self.section = step.section;
        self.search = step.search;
        self.matches = None;
        self.source = false;
        self.error = None;
        self.message = None;
    }

    /// In a wide window, the reader and its margin sit together in the pane, with the scroll
    /// bar beside them rather than across an empty band.
    fn editor(&mut self, ui: &mut egui::Ui, subtitle: &str, campaigns: &Campaigns) {
        let spare = ui.available_width() - view::WIDE_READER;
        if spare <= 0.0 || self.disk_change.is_some() {
            self.editor_pane(ui, subtitle, campaigns);
            return;
        }
        ui.horizontal_top(|ui| {
            ui.add_space(spare / 2.0);
            let size = egui::vec2(view::WIDE_READER, ui.available_height());
            ui.allocate_ui_with_layout(size, egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_max_width(view::WIDE_READER);
                self.editor_pane(ui, subtitle, campaigns);
            });
        });
    }

    fn save_selected(&mut self, file_gone: bool) {
        self.layouts.clear();
        self.layout_scope = Some(self.scope);
        let Some(i) = self.selected else {
            return;
        };
        let note = &self.loaded.as_ref().unwrap().notes[i];
        let mut file_gone = file_gone;
        if self.loaded.as_ref().unwrap().duplicates(i).is_empty()
            && (file_gone
                || note_path(note.owner.root().unwrap(), &note.path)
                    .is_ok_and(|path| !path.exists()))
        {
            let moved = crate::content_workspace::note_id(&note.text)
                .map(|id| {
                    let mut owners = vec![note.owner.clone()];
                    for owner in &self.loaded.as_ref().unwrap().owners {
                        if !owners.contains(owner) {
                            owners.push(owner.clone());
                        }
                    }
                    for other in &self.loaded.as_ref().unwrap().notes {
                        if other.owner.root().is_some() && !owners.contains(&other.owner) {
                            owners.push(other.owner.clone());
                        }
                    }
                    let mut found = None;
                    for owner in owners {
                        let old = if owner == note.owner {
                            note.path.as_str()
                        } else {
                            ""
                        };
                        if let Some((path, text)) = moved_note(owner.root().unwrap(), id, old)? {
                            if found.is_some() {
                                return Err(
                                    "Several files have this entry's ID. Choose one in Library."
                                        .to_owned(),
                                );
                            }
                            found = Some((owner, path, text));
                        }
                    }
                    Ok(found)
                })
                .transpose();
            match moved {
                Ok(Some(Some((owner, path, text)))) => {
                    let note = &mut self.loaded.as_mut().unwrap().notes[i];
                    note.path = path;
                    note.owner = owner;
                    if note.text != text {
                        self.disk_change = Some(DiskChange {
                            text: Some(text),
                            moved_to: Some(note.path.clone()),
                        });
                        self.error = Some(NOTE_CHANGED.into());
                        return;
                    }
                    file_gone = false;
                    self.stale = true;
                }
                Err(error) => {
                    self.error = Some(error);
                    return;
                }
                _ => {}
            }
        }
        let note = &self.loaded.as_ref().unwrap().notes[i];
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
                            (e.id == entry.id
                                && note.entry.as_ref().is_none_or(|old| old.id != entry.id))
                                || (entry.key.is_some()
                                    && e.key == entry.key
                                    && note.entry.as_ref().is_none_or(|old| old.key != entry.key))
                        })
                })
            {
                return Err("Another entry already uses this UUID or key.".into());
            }
            let baseline = (!file_gone).then_some(note.text.as_str());
            let entry = write_note(&root, &note.path, baseline, &self.draft)?;
            Ok(entry)
        });
        match result {
            Ok(entry) => {
                let l = self.loaded.as_mut().unwrap();
                if let Some(old) = &l.notes[i].entry
                    && old.name != entry.name
                    && entry.kind != "package"
                    && l.can_rename_links(i, &old.name, &entry.name)
                {
                    self.renamed_links = Some((old.name.clone(), entry.name.clone()));
                }
                if entry.kind == "package"
                    && let Owner::Package(module) = &l.notes[i].owner
                {
                    self.renamed = Some((module.id, entry.name.clone()));
                    let id = module.id;
                    for note in &mut l.notes {
                        if let Owner::Package(owner) = &mut note.owner
                            && owner.id == id
                        {
                            owner.name = entry.name.clone();
                        }
                    }
                }
                self.stale |= l.notes[i]
                    .entry
                    .as_ref()
                    .is_none_or(|old| old.id != entry.id);
                l.notes[i].entry = Some(entry);
                l.notes[i].text = self.draft.clone();
                l.notes[i].error = None;
                l.invalidate();
                self.matches = None;
                self.scope_counts = None;
                self.just_saved = true;
                self.content_changed = Some(root);
                l.refresh_diagnostics();
                self.error = None;
            }
            Err(e) if e == NOTE_CHANGED => {
                self.disk_change = Some(DiskChange {
                    moved_to: None,
                    text: note_path(&root, &note.path)
                        .ok()
                        .and_then(|path| fs::read_to_string(path).ok()),
                });
            }
            Err(e) => self.error = Some(e),
        }
    }

    fn editor_pane(&mut self, ui: &mut egui::Ui, subtitle: &str, campaigns: &Campaigns) {
        if let Some(write) = &self.manifest_write
            && design::locked(ui, self.draft_reason(), |ui| {
                design::message_with_action(
                    ui,
                    Tone::Success,
                    if write.replaced() {
                        "Package.md was replaced."
                    } else {
                        "Package.md was written again."
                    },
                    "Undo write Package.md",
                )
            })
        {
            match write.undo() {
                Ok(root) => {
                    self.content_changed = Some(root);
                    self.manifest_write = None;
                    self.stale = true;
                }
                Err(error) => self.error = Some(error),
            }
        }
        let recovery = self
            .selected
            .and_then(|at| self.loaded.as_ref()?.notes.get(at))
            .filter(|note| note.is_manifest() && note.error.is_some())
            .and_then(|note| match &note.owner {
                Owner::Package(module) => Some(module.clone()),
                _ => None,
            });
        let gone_notice = self.discarded_note.as_ref().map(|note| {
            let names = self
                .loaded
                .as_ref()
                .map(|loaded| loaded.dependents(note))
                .unwrap_or_default();
            if names.is_empty() {
                format!("{} let go.", note.name())
            } else {
                format!(
                    "{} let go. {} now needs a missing entry.",
                    note.name(),
                    name_list(&names)
                )
            }
        });
        if self.discarded_note.is_some()
            && !self.dirty()
            && design::undo_message(
                ui,
                Tone::Info,
                gone_notice.as_deref().unwrap(),
                "Undo let go",
            )
            && let Some((_, _, draft)) = self.discarded.take()
            && let Some(note) = self.discarded_note.take()
            && let Some(loaded) = self.loaded.as_mut()
        {
            self.selected = Some(loaded.notes.len());
            loaded.notes.push(note);
            loaded.invalidate();
            self.draft = draft;
            self.source = true;
            self.disk_change = Some(DiskChange {
                text: None,
                moved_to: None,
            });
            self.invalidate();
        }
        let Some(i) = self.selected else {
            design::empty_state(
                ui,
                icons::LIBRARY,
                "No entry open",
                if campaigns.active_campaign().is_some() || !campaigns.modules().is_empty() {
                    "Choose one from the list, or make a new one."
                } else {
                    "Choose one from the list."
                },
                |_| {},
            );
            return;
        };
        // Overwrite was chosen: what is on disk now becomes what this save replaces.
        let overwrite = std::mem::take(&mut self.overwrite_disk);
        let mut file_gone = false;
        if overwrite && let Some(change) = self.disk_change.take() {
            if let Some(path) = change.moved_to {
                self.loaded.as_mut().unwrap().notes[i].path = path;
                self.stale = true;
            }
            match change.text {
                Some(text) => self.loaded.as_mut().unwrap().notes[i].text = text,
                None => file_gone = true,
            }
        }
        let draft_reason = self.draft_reason();
        let note = &self.loaded.as_ref().unwrap().notes[i];
        let parsed = Entry::parse(&note.path, &self.draft);
        let reading = parsed
            .as_ref()
            .ok()
            .and_then(|entry| entry.reading().ok().flatten());
        let title = reading
            .as_ref()
            .map_or(note.name(), |reading| reading.title.as_str());
        let reading_subtitle = reader_subtitle(
            &note.owner,
            &note
                .entry
                .as_ref()
                .filter(|entry| {
                    self.loaded
                        .as_ref()
                        .unwrap()
                        .duplicate_ids()
                        .contains(&entry.id)
                })
                .map_or_else(|| subtitle.to_owned(), presentation::subtitle),
            reading
                .as_ref()
                .and_then(|_| presentation::class_level(parsed.as_ref().ok()?)),
        );
        let dirty = self.draft != note.text;
        if dirty {
            self.message = None;
        }
        let editable = note.owner.editable();
        let conflict = self.disk_change.is_some();
        let mut duplicate_indices = self.loaded.as_ref().unwrap().duplicates(i);
        if !duplicate_indices.is_empty() {
            duplicate_indices.push(i);
            let notes = &self.loaded.as_ref().unwrap().notes;
            let first = crate::content_workspace::duplicate_repair_path(
                duplicate_indices.iter().filter_map(|&at| {
                    notes[at]
                        .owner
                        .root()
                        .map(|root| root.join(&notes[at].path))
                }),
            );
            duplicate_indices.sort_by_key(|&at| {
                notes[at]
                    .owner
                    .root()
                    .map(|root| root.join(&notes[at].path))
                    != first
            });
        }
        let mut new_identity = None;
        let mut new_identities = None;
        let mut undo_identities = false;
        let mut undo_identity = false;
        let editor_width = if conflict {
            ui.available_width()
        } else {
            ui.available_width().min(EDITOR_MAX_WIDTH)
        };
        self.just_saved &= !dirty && self.source;
        let mut save = overwrite
            || (dirty
                && editable
                && !conflict
                && ui.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)));
        let d = colors().document;
        let about = note.entry.as_ref().is_some_and(|e| e.kind == "package");
        let mut use_every = None;
        let package_word = match &note.owner {
            Owner::Package(_) if about && campaigns.active_campaign().is_none() => {
                "No campaign open".into()
            }
            Owner::Package(module) if about => {
                let loaded = self.loaded.as_ref().unwrap();
                let entries: Vec<_> = loaded
                    .notes
                    .iter()
                    .filter(|n| {
                        n.owner.id() == Some(module.id)
                            && n.entry.as_ref().is_some_and(|e| {
                                !["package", "reference"].contains(&e.kind.as_str())
                            })
                    })
                    .collect();
                let used = entries
                    .iter()
                    .filter(|n| loaded.in_scope(n, Scope::Uses))
                    .count();
                if used == entries.len() {
                    "All entries used here".into()
                } else {
                    format!("Used here · {used} of {} entries", entries.len())
                }
            }
            Owner::Package(module) if note.error.is_some() => {
                format!("{} · Cannot be read", module.name)
            }
            Owner::Package(module) => format!(
                "{} · {}",
                module.name,
                if self.loaded.as_ref().unwrap().in_scope(note, Scope::Uses) {
                    "Used here"
                } else {
                    "Not used here"
                }
            ),
            _ => String::new(),
        };
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
                    word: &package_word,
                    color: d.clean,
                },
                Owner::Campaign(_) => Status {
                    icon: icons::EDIT,
                    word: "Campaign · Editable",
                    color: d.clean,
                },
            }
        };
        let save_row = editable && (self.source || dirty);
        let mut go_back =
            ui.input_mut(|input| input.consume_key(egui::Modifiers::ALT, egui::Key::ArrowLeft));
        let can_copy = note
            .entry
            .as_ref()
            .is_some_and(|e| !["package", "campaign", "reference"].contains(&e.kind.as_str()));
        let copy_reason = draft_reason.or_else(|| copy_destination_reason(campaigns));
        let can_make = matches!(note.owner, Owner::Campaign(_))
            && note.entry.as_ref().is_some_and(|e| e.kind == "creature");
        let mut copy_clicked = false;
        let mut make_clicked = false;
        let back = Action::quiet("Back")
            .icon(icons::BACK)
            .density(Density::Read);
        let edit_label = if editable {
            "Edit Markdown"
        } else {
            "Markdown source"
        };
        let base_width = Action::quiet("Read").natural_width(ui)
            + Action::quiet(edit_label).natural_width(ui)
            + if self.history.is_empty() {
                0.0
            } else {
                back.natural_width(ui) + design::S
            };
        let extras = if can_copy {
            Action::quiet("Copy to…")
                .icon(icons::COPY)
                .density(Density::Read)
                .natural_width(ui)
                + design::S
        } else {
            0.0
        } + if can_make {
            Action::quiet("Make NPC note").natural_width(ui) + design::S
        } else {
            0.0
        };
        let with_key = base_width
            + extras
            + design::number_width(ui, "Alt+Left", &Type::Small.font(ui))
            + design::L;
        let show_key = with_key <= ui.available_width();
        let overflow = base_width + extras > ui.available_width();
        ui.horizontal(|ui| {
            let edit_label = if editable {
                "Edit Markdown"
            } else {
                "Markdown source"
            };
            let was_source = self.source;
            design::segmented(ui, &mut self.source, &[(false, "Read"), (true, edit_label)]);
            if self.source != was_source {
                self.scope_notice = None;
            }
            if !overflow {
                if can_copy {
                    copy_clicked = Action::quiet("Copy to…")
                        .icon(icons::COPY)
                        .density(Density::Read)
                        .blocked(copy_reason)
                        .show(ui)
                        .clicked();
                }
                if can_make {
                    make_clicked = Action::quiet("Make NPC note")
                        .blocked(draft_reason)
                        .show(ui)
                        .clicked();
                }
            }
            if !self.history.is_empty()
                && (if show_key { back.key("Alt+Left") } else { back })
                    .hint(
                        "Return to the entry before this link. During a draft, read it in Lookup.",
                    )
                    .show(ui)
                    .clicked()
            {
                go_back = true;
            }
            if overflow {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::containers::menu::MenuButton::from_button(
                        egui::Button::new(Type::Strong.rich(ui, "More"))
                            .frame(false)
                            .min_size(egui::vec2(0.0, Density::Read.control())),
                    )
                    .ui(ui, |ui| {
                        if can_copy {
                            copy_clicked = Action::quiet("Copy to…")
                                .icon(icons::COPY)
                                .blocked(copy_reason)
                                .show(ui)
                                .clicked();
                        }
                        if can_make {
                            make_clicked = Action::quiet("Make NPC note")
                                .blocked(draft_reason)
                                .show(ui)
                                .clicked();
                        }
                        if copy_clicked || make_clicked {
                            ui.close();
                        }
                    });
                });
            }
        });
        if copy_clicked {
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
        if make_clicked && let (Owner::Campaign(campaign), Some(entry)) = (&note.owner, &note.entry)
        {
            match crate::world::make_npc_note(campaign, entry) {
                Ok(id) => {
                    self.stale = true;
                    self.content_changed = Some(campaign.path.clone());
                    self.open_world_npc = Some(id);
                }
                Err(error) => self.error = Some(error),
            }
        }
        if let Some(root) = note.owner.root() {
            let moved_path = self
                .disk_change
                .as_ref()
                .and_then(|change| change.moved_to.as_deref());
            let shown_path = moved_path.unwrap_or(&note.path);
            let moved_from = moved_path.map(|_| {
                format!(
                    "moved from {}/",
                    Path::new(&note.path)
                        .parent()
                        .unwrap_or(Path::new(""))
                        .display()
                )
            });
            let file_line = |ui: &mut egui::Ui| {
                if let Some(from) = &moved_from {
                    let width = (ui.available_width()
                        - design::number_width(ui, from, &Type::Small.font(ui))
                        - design::S)
                        .max(design::XXL);
                    ui.allocate_ui_with_layout(
                        egui::vec2(width, Density::Read.control()),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            crate::world::entry_file_parts(
                                ui,
                                root,
                                shown_path,
                                obsidian_for(root, &note.owner),
                            );
                        },
                    );
                    ui.label(Type::Small.rich(ui, from).color(colors().muted));
                } else {
                    crate::world::entry_file_parts(
                        ui,
                        root,
                        shown_path,
                        obsidian_for(root, &note.owner),
                    );
                }
            };
            let obsidian = obsidian_for(root, &note.owner);
            let owner_word = match &note.owner {
                Owner::Package(_) => package_word.as_str(),
                _ => "Campaign · Editable",
            };
            let file_below = ui.available_width()
                < design::number_width(ui, owner_word, &Type::Small.font(ui))
                    + design::number_width(ui, "Show file", &Type::Body.font(ui))
                    + design::XXL * 2.0
                    + if obsidian {
                        design::number_width(ui, "Edit in Obsidian", &Type::Body.font(ui))
                    } else {
                        0.0
                    };
            let owner_line = |ui: &mut egui::Ui| {
                design::status(
                    ui,
                    Status {
                        icon: icons::EDIT,
                        word: match &note.owner {
                            Owner::Package(_) => &package_word,
                            Owner::Campaign(_) => "Campaign · Editable",
                            Owner::Builtin => "Built-in · Read only",
                        },
                        color: d.clean,
                    },
                );
                if let Owner::Package(module) = &note.owner
                    && about
                    && campaigns.active_campaign().is_some()
                    && package_word != "All entries used here"
                    && Action::quiet("Use every entry")
                        .blocked(draft_reason)
                        .show(ui)
                        .clicked()
                {
                    use_every = Some(module.clone());
                }
                if let Owner::Package(module) = &note.owner
                    && !about
                    && !self.loaded.as_ref().unwrap().in_scope(note, Scope::Uses)
                    && let Some(entry) = &note.entry
                    && Action::quiet("Use in this campaign")
                        .blocked(draft_reason)
                        .show(ui)
                        .clicked()
                {
                    self.use_entry = Some((module.id, entry.id));
                }
                if !file_below {
                    file_line(ui);
                }
            };
            if file_below {
                ui.horizontal_wrapped(owner_line);
            } else {
                ui.horizontal(owner_line);
            }
            if file_below {
                ui.horizontal(file_line);
            }
        } else {
            design::status(
                ui,
                Status {
                    icon: icons::READ_ONLY,
                    word: "Built-in · Read only",
                    color: d.read_only,
                },
            );
        }
        if !duplicate_indices.is_empty() {
            design::message(
                ui,
                Tone::Warning,
                "These two files claim to be the same entry. Choose which one becomes a separate entry.",
            );
            let loaded = self.loaded.as_ref().unwrap();
            let users: Vec<_> = loaded
                .notes
                .iter()
                .filter_map(|other| {
                    let entry = other.entry.as_ref()?;
                    (entry.kind == "encounter"
                        && content::required_references(entry).is_ok_and(|refs| {
                            refs.contains(&content::Reference::Id(note.entry.as_ref().unwrap().id))
                        }))
                    .then(|| entry.name.clone())
                })
                .collect();
            for (position, &at) in duplicate_indices.iter().enumerate() {
                let target = &loaded.notes[at];
                let filename = Path::new(&target.path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                let same_filename = duplicate_indices.iter().any(|&other| {
                    other != at
                        && Path::new(&loaded.notes[other].path).file_name()
                            == Path::new(&target.path).file_name()
                });
                let filename = if same_filename {
                    format!("{filename} in {}/", target.owner.folder())
                } else {
                    filename.into_owned()
                };
                if Action::quiet(&format!("Make {filename} a separate entry"))
                    .blocked(draft_reason)
                    .show(ui)
                    .clicked()
                {
                    new_identity = Some(at);
                }
                if position > 0 && !users.is_empty() {
                    let remaining = &loaded.notes[duplicate_indices[0]].path;
                    let remaining = Path::new(remaining)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy();
                    let remaining = if same_filename {
                        format!(
                            "{remaining} in {}/",
                            loaded.notes[duplicate_indices[0]].owner.folder()
                        )
                    } else {
                        remaining.into_owned()
                    };
                    ui.label(
                        Type::Small
                            .rich(
                                ui,
                                format!("{} will then use {remaining}.", name_list(&users)),
                            )
                            .color(colors().muted),
                    );
                }
            }
            let mut shown = std::collections::HashSet::new();
            for &at in &duplicate_indices {
                let owner = &loaded.notes[at].owner;
                if shown.insert(owner.id())
                    && let Some(paths) = loaded.duplicate_folder_paths(owner)
                    && Action::quiet(&format!(
                        "Make all {} files in {}/ separate entries",
                        paths.len(),
                        owner.folder()
                    ))
                    .blocked(draft_reason)
                    .show(ui)
                    .clicked()
                {
                    new_identities = Some((owner.root().unwrap().to_path_buf(), paths));
                }
            }
        }
        if let Some((_, words)) = &self.identities_undo {
            design::locked(ui, draft_reason, |ui| {
                undo_identities =
                    design::undo_message(ui, Tone::Success, words, "Undo separate entries");
            });
        }
        if let Some(undo) = &self.identity_undo {
            let filename = Path::new(&undo.path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            design::locked(ui, draft_reason, |ui| {
                undo_identity = design::undo_message(
                    ui,
                    Tone::Success,
                    &format!("{filename} is now a separate entry."),
                    "Undo separate entry",
                );
            });
        }
        // Save and Discard are one row, as far apart as the pane allows.
        if save_row && !conflict {
            ui.horizontal(|ui| {
                ui.set_max_width(editor_width);
                ui.set_min_height(Density::Scan.control());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let blocked = if conflict {
                        Some("Choose what to do with the changed file first.")
                    } else {
                        (!dirty).then_some("No changes to save.")
                    };
                    save |= (if self.run_open {
                        Action::secondary("Save")
                    } else {
                        Action::primary("Save")
                    })
                    .density(Density::Read)
                    .blocked(blocked)
                    .key("Ctrl+S")
                    .show(ui)
                    .clicked();
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        if design::save_row_state(ui, state, dirty && !conflict, None) {
                            self.layouts.clear();
                            self.discarded =
                                Some((note.owner.id(), note.path.clone(), self.draft.clone()));
                            self.draft = note.text.clone();
                            self.error = None;
                        }
                    });
                });
            });
            // Measure the same words in both states, so the first edit keeps the caret in place.
            let words = draft_words(matches!(note.owner, Owner::Campaign(_)));
            let height = egui::WidgetText::from(Type::Small.rich(ui, words))
                .into_galley(
                    ui,
                    Some(egui::TextWrapMode::Wrap),
                    editor_width,
                    Type::Small.font(ui),
                )
                .size()
                .y
                .max(Density::Read.control() + design::XS);
            let (slot, _) =
                ui.allocate_exact_size(egui::vec2(editor_width, height), egui::Sense::hover());
            if draft_reason.is_some() || discard_is_for(self.discarded.as_ref(), note) {
                let mut slot_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(slot)
                        .layout(egui::Layout::left_to_right(egui::Align::Center)),
                );
                let ui = &mut slot_ui;
                if discard_is_for(self.discarded.as_ref(), note) && self.draft == note.text {
                    ui.label(Type::Small.rich(ui, format!("{} draft discarded.", note.name())));
                    if Action::quiet("Undo discard")
                        .name(&format!("Undo discard of {}", note.name()))
                        .icon(icons::UNDO)
                        .density(Density::Read)
                        .show(ui)
                        .clicked()
                        && let Some((_, _, text)) = self.discarded.take()
                    {
                        self.draft = text;
                    }
                } else if let Some(reason) = draft_reason {
                    ui.add(
                        egui::Label::new(Type::Small.rich(ui, reason).color(colors().muted)).wrap(),
                    );
                }
            }
        }
        // Live, because Discard in this same frame changed the draft.
        if self.draft != note.text {
            self.discarded = None;
        }
        if !save_row
            && discard_is_for(self.discarded.as_ref(), note)
            && design::undo_message(
                ui,
                Tone::Info,
                &format!("{} draft discarded.", note.name()),
                &format!("Undo discard of {}", note.name()),
            )
            && let Some((_, _, text)) = self.discarded.take()
        {
            self.draft = text;
        }
        let mut reload_from_disk = false;
        if let Some(change) = &mut self.disk_change {
            design::message(
                ui,
                Tone::Warning,
                if change.moved_to.is_some() {
                    "This entry moved on disk. Save your draft to its new path, or keep the disk version."
                } else if change.text.is_some() {
                    "This entry changed on disk. Your draft is safe until you choose."
                } else {
                    "This entry's file has gone from disk. Your draft is safe here until you choose."
                },
            );
            if change.text.is_none() {
                let names = self.loaded.as_ref().unwrap().dependents(note);
                if !names.is_empty() {
                    ui.label(
                        Type::Small
                            .rich(ui, format!("{} needs this entry.", name_list(&names)))
                            .color(colors().muted),
                    );
                }
            }
            ui.horizontal(|ui| {
                let save_word = change.moved_to.as_ref().map_or_else(
                    || {
                        if change.text.is_none() {
                            "Save my draft again".to_owned()
                        } else {
                            "Overwrite with my draft".to_owned()
                        }
                    },
                    |path| format!("Save my draft to {path}"),
                );
                if Action::secondary(&save_word)
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    self.overwrite_disk = true;
                }
                ui.add_space(design::L);
                reload_from_disk = Action::destructive(if change.text.is_none() {
                    "Let it go"
                } else {
                    "Reload and lose my draft"
                })
                .density(Density::Read)
                .show(ui)
                .clicked();
            });
        }
        let (repair_problem, repair_line) =
            crate::campaign_content::repair_problem(&note.path, &self.draft);
        if note.error.is_some()
            && let Some(problem) = &repair_problem
        {
            if crate::formats_ui::is_old(problem) {
                crate::formats_ui::old_notice(
                    ui,
                    &crate::formats_ui::old_message(&note.path),
                    note.owner
                        .root()
                        .map(|root| root.join(&note.path))
                        .into_iter()
                        .collect(),
                );
            } else {
                design::message(ui, Tone::Error, problem);
            }
        }
        if let Some(module) = recovery {
            let path = crate::content_workspace::package_manifest_path(&module);
            let exists = module.path.join(&path).exists();
            if !exists {
                design::message(
                    ui,
                    Tone::Warning,
                    crate::content_workspace::DELETED_MANIFEST,
                );
            }
            let label = if exists {
                format!("Replace with a basic {path}")
            } else {
                format!("Write {path} again")
            };
            if exists {
                ui.label(
                    Type::Small
                        .rich(ui, "Keeps the id, name and namespace. Drops the rest.")
                        .color(colors().muted),
                );
            }
            if Action::quiet(&label)
                .blocked(draft_reason)
                .show(ui)
                .clicked()
            {
                match crate::content_workspace::ManifestWrite::write(&module) {
                    Ok(write) => {
                        self.manifest_write = Some(write);
                        self.stale = true;
                        self.content_changed = Some(module.path);
                    }
                    Err(error) => self.error = Some(error),
                }
                return;
            }
        }
        if editable
            && !conflict
            && let Ok(entry) = &parsed
        {
            if let Some((old, after)) = presentation::heading_fix(&self.draft, entry)
                && design::message_with_action(
                    ui,
                    Tone::Warning,
                    &format!(
                        "The first heading says {old}, but the name is {}.",
                        entry.name
                    ),
                    &format!("Use {}", entry.name),
                )
            {
                self.heading_undo = Some((note.path.clone(), self.draft.clone(), after.clone()));
                self.draft = after;
                save = true;
            }
            if let Some((path, before, after)) = &self.heading_undo
                && path == &note.path
                && after == &self.draft
                && Action::quiet("Undo heading change")
                    .icon(icons::UNDO)
                    .show(ui)
                    .clicked()
            {
                self.draft = before.clone();
                self.heading_undo = None;
                save = true;
            }
        }
        if self.source && !conflict {
            if editable && let Ok(entry) = &parsed {
                for (key, value, message) in creature::vital_checks(entry) {
                    if design::message_with_action(
                        ui,
                        Tone::Warning,
                        &message,
                        &format!("Use {value}"),
                    ) {
                        match creature::use_vital(&self.draft, entry, key, value) {
                            Ok(after) => {
                                self.vital_undo =
                                    Some((note.path.clone(), self.draft.clone(), after.clone()));
                                self.draft = after;
                            }
                            Err(error) => self.error = Some(error),
                        }
                    }
                }
            }
            if let Some((path, before, after)) = &self.vital_undo
                && path == &note.path
                && after == &self.draft
                && Action::quiet("Undo stat change")
                    .icon(icons::UNDO)
                    .show(ui)
                    .clicked()
            {
                self.draft = before.clone();
                self.vital_undo = None;
            }
        }
        let mut open_link = None;
        let mut copy_dependency = None;
        let mut undo_dependency = false;
        if self.source && self.disk_change.is_none() {
            egui::Frame::new()
                .fill(colors().well)
                .stroke(egui::Stroke::new(
                    design::HAIRLINE_WIDTH,
                    colors().field_border,
                ))
                .corner_radius(design::RADIUS_CONTROL)
                .inner_margin(design::S)
                .show(ui, |ui| {
                    ui.set_max_width(editor_width - (design::S + design::HAIRLINE_WIDTH) * 2.0);
                    let visible_height = ui.available_height().max(design::XL);
                    egui::ScrollArea::vertical()
                        .id_salt(("library_source", note.owner.id(), &note.path))
                        .max_height(visible_height)
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let response = ui.add(
                                egui::TextEdit::multiline(&mut self.draft)
                                    .id_salt(("library_markdown", note.owner.id(), &note.path))
                                    .font(egui::TextStyle::Monospace)
                                    .frame(false)
                                    .margin(egui::Vec2::ZERO)
                                    .desired_width(ui.available_width())
                                    .min_size(egui::vec2(0.0, visible_height))
                                    .interactive(editable),
                            );
                            design::named(ui, &response, "Markdown editor");
                            if std::mem::take(&mut self.focus_markdown) {
                                response.request_focus();
                            }
                            if std::mem::take(&mut self.focus_repair)
                                && let Some(line) = repair_line
                            {
                                let at = self
                                    .draft
                                    .lines()
                                    .take(line.saturating_sub(1))
                                    .map(|line| line.chars().count() + 1)
                                    .sum();
                                response.request_focus();
                                let mut state = egui::TextEdit::load_state(ui.ctx(), response.id)
                                    .unwrap_or_default();
                                state
                                    .cursor
                                    .set_char_range(Some(egui::text::CCursorRange::one(
                                        egui::text::CCursor::new(at),
                                    )));
                                state.store(ui.ctx(), response.id);
                            }
                            if std::mem::take(&mut self.focus_hp)
                                && let Some(at) = self
                                    .draft
                                    .find("initiative: ")
                                    .map(|at| at + "initiative: ".len())
                            {
                                response.request_focus();
                                let at = self.draft[..at].chars().count();
                                let mut state = egui::TextEdit::load_state(ui.ctx(), response.id)
                                    .unwrap_or_default();
                                state
                                    .cursor
                                    .set_char_range(Some(egui::text::CCursorRange::two(
                                        egui::text::CCursor::new(at),
                                        egui::text::CCursor::new(at + 1),
                                    )));
                                state.store(ui.ctx(), response.id);
                            }
                        });
                });
        } else {
            egui::ScrollArea::vertical()
            .id_salt(("entry_body", note.owner.id(), &note.path))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if let Some(change) = &mut self.disk_change {
                    if let Some(text) = &change.text {
                        design::grouped_draft_comparison(ui, &self.draft, text, (&note.path, text), true);
                    } else {
                        design::group_heading(ui, "Your draft", 0);
                        let mut draft = self.draft.as_str();
                        let response = ui.add(egui::TextEdit::multiline(&mut draft).font(egui::TextStyle::Monospace).margin(egui::vec2(design::S, design::S)).desired_width(editor_width - 2.0 * design::S).interactive(false));
                        design::named(ui, &response, "Your draft");
                    }
                    return;
                }
                    // A failed parse must never render YAML properties as book prose.
                    let body = parsed.as_ref().map_or("", |e| e.body.as_str());
                    ui.push_id(&note.path, |ui| {
                        reader_style::measure(ui, |ui| {
                            ui.add_space(design::XS);
                            if !about {
                                ui.label(Type::Small.rich(ui, format!("Library / {}", note.section().label())).color(colors().muted));
                            }
                            let reading_subtitle = match &note.owner {
                                Owner::Package(module) => reading_subtitle.strip_suffix(&format!(" · {}", module.name)).unwrap_or(&reading_subtitle),
                                _ => &reading_subtitle,
                            };
                            let subtitle = reading_subtitle;
                            let subtitle = subtitle.strip_suffix(" · Campaign").unwrap_or(subtitle);
                            let subtitle = if note.error.is_some() { "" } else if subtitle == "Campaign" { note.section().label().trim_end_matches('s') } else { subtitle };
                            view::title_with_art(ui, title, subtitle, parsed.as_ref().ok());
                        });
                        open_link = view::show_with_related(
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
                            |ui| {
                                if let Owner::Package(package) = &note.owner
                                    && let Some(entry) = &note.entry
                                    && let Some(index) = ui.ctx().data(|data| data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new("reader_link_index"))) {
                                    for at in index.campaign_dependencies(entry) {
                                        let dependency = &index.items[at].entry;
                                        if design::locked(ui, draft_reason, |ui| design::message_with_action(ui, Tone::Warning, &format!("{} is in this campaign only.", dependency.name), &format!("Copy {} to {}", dependency.name, package.name))) {
                                            copy_dependency = Some(dependency.id);
                                        }
                                    }
                                }
                                if self.dependency_undo.as_ref().is_some_and(|change| change.path == note.path) {
                                    undo_dependency = design::message_with_action(ui, Tone::Success, "Dependency copied and linked.", "Undo dependency copy");
                                }
                                None
                            },
                        );
                    });
            });
        }
        let reconnect = (|| -> Result<(), String> {
            if undo_dependency
                && let Some(DependencyUndo {
                    root,
                    path,
                    before,
                    after,
                    copied_path,
                    copied_text,
                }) = &self.dependency_undo
            {
                let copied = note_path(root, copied_path)?;
                if fs::read_to_string(&copied).map_err(|e| e.to_string())? != *copied_text {
                    return Err("The dependency copy changed on disk. Keep it and reconnect the reference in Markdown.".into());
                }
                write_note(root, path, Some(after), before)?;
                if let Err(error) = fs::remove_file(copied) {
                    let _ = write_note(root, path, Some(before), after);
                    return Err(error.to_string());
                }
                self.content_changed = Some(root.clone());
                self.dependency_undo = None;
                self.stale = true;
            }
            if let Some(id) = copy_dependency
                && let Owner::Package(package) = &note.owner
                && let Some(source) = self
                    .loaded
                    .as_ref()
                    .unwrap()
                    .notes
                    .iter()
                    .find(|n| n.entry.as_ref().is_some_and(|e| e.id == id))
            {
                let source = crate::content_workspace::CopySource {
                    entry: source.entry.clone().unwrap(),
                    text: source.text.clone(),
                    root: source.owner.root().map(Path::to_path_buf),
                    campaign: None,
                };
                let catalogue = crate::content_workspace::Catalogue::load(
                    campaigns.modules(),
                    campaigns.active_campaign().as_ref(),
                )?;
                let plan = crate::content_workspace::CopyPlan::prepare(
                    &source,
                    &package.path,
                    None,
                    &source.entry.name,
                    "",
                    &catalogue,
                )?;
                let copied = plan.save()?;
                let after = crate::content_workspace::redirect_reference(
                    &note.text,
                    &source.entry,
                    copied.id,
                )?;
                if let Err(error) = write_note(&package.path, &note.path, Some(&note.text), &after)
                {
                    let _ = fs::remove_file(package.path.join(&plan.relative));
                    return Err(error);
                }
                self.dependency_undo = Some(DependencyUndo {
                    root: package.path.clone(),
                    path: note.path.clone(),
                    before: note.text.clone(),
                    after,
                    copied_path: plan.relative.clone(),
                    copied_text: plan.text.clone(),
                });
                self.content_changed = Some(package.path.clone());
                self.stale = true;
            }
            Ok(())
        })();
        if let Err(error) = reconnect {
            self.error = Some(error);
        }
        if reload_from_disk {
            self.reload_note_from_disk(i);
            return;
        }
        if let Some(target) = crate::markdown_view::take_note_link(ui.ctx())
            && let Some(index) = ui.ctx().data(|data| {
                data.get_temp::<std::sync::Arc<crate::lookup::ContentIndex>>(egui::Id::new(
                    "reader_link_index",
                ))
            })
        {
            let from = note.entry.as_ref().map(|entry| entry.id);
            let choices = match index.resolve_link(&target, from) {
                crate::world::Go::Item(at, heading) => {
                    crate::markdown_view::want_heading(ui.ctx(), heading);
                    vec![index.items[at].entry.id]
                }
                crate::world::Go::Many(positions) => positions
                    .iter()
                    .map(|&at| index.items[at].entry.id)
                    .collect(),
                _ => Vec::new(),
            };
            self.link_choices = choices;
        }
        if self.link_choices.len() == 1 {
            let id = self.link_choices.remove(0);
            open_link = self
                .loaded
                .as_ref()
                .unwrap()
                .notes
                .iter()
                .position(|n| n.entry.as_ref().is_some_and(|e| e.id == id));
            if open_link.is_none() {
                self.peek = Some(id);
            }
        } else if !self.link_choices.is_empty() {
            ui.horizontal_wrapped(|ui| {
                for id in &self.link_choices {
                    if let Some((at, note)) = self
                        .loaded
                        .as_ref()
                        .unwrap()
                        .notes
                        .iter()
                        .enumerate()
                        .find(|(_, n)| n.entry.as_ref().is_some_and(|e| e.id == *id))
                        && design::link(ui, &format!("{} · {}", note.name(), note.owner.label()))
                            .clicked()
                    {
                        open_link = Some(at);
                    }
                }
            });
            if open_link.is_some() {
                self.link_choices.clear();
            }
        }
        if let Some(module) = use_every
            && let Some(campaign) = campaigns.active_campaign()
        {
            let result =
                crate::content_workspace::CampaignDocument::load(&campaign).and_then(|before| {
                    crate::content_workspace::use_in_campaign(
                        &campaign,
                        module.id,
                        &module.name,
                        None,
                    )?;
                    let after = crate::content_workspace::CampaignDocument::load(&campaign)?;
                    Ok(SelectionUndo {
                        campaign: campaign.clone(),
                        before: before.selections,
                        after: after.selections,
                    })
                });
            match result {
                Ok(undo) => {
                    self.selection_undo = Some(undo);
                    self.stale = true;
                    self.content_changed = Some(campaign.path);
                }
                Err(error) => self.error = Some(error),
            }
        }

        if save {
            self.save_selected(file_gone);
        }
        let identity_result = if let Some((root, paths)) = new_identities {
            crate::content_workspace::ContentEdits::new_identities(&root, &paths).map(|edits| {
                let folder = root.file_name().unwrap_or_default().to_string_lossy();
                self.identities_undo = Some((
                    edits,
                    format!(
                        "{} in {folder}/ are now separate entries.",
                        design::count(paths.len(), "file", "files")
                    ),
                ));
                self.stale = true;
                self.content_changed = Some(root);
            })
        } else if undo_identities {
            self.identities_undo.as_ref().unwrap().0.undo().map(|()| {
                self.identities_undo = None;
                self.stale = true;
                self.content_changed = campaigns.active_campaign().map(|campaign| campaign.path);
            })
        } else if let Some(at) = new_identity {
            self.open_entry(at);
            self.give_new_identity()
        } else if undo_identity {
            self.undo_new_identity()
        } else {
            Ok(())
        };
        if let Err(error) = identity_result {
            self.error = Some(error);
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
            if dirty {
                self.peek = self.history.last().and_then(|step| {
                    self.loaded
                        .as_ref()?
                        .notes
                        .iter()
                        .find(|note| note.owner.id() == step.owner && note.path == step.path)?
                        .entry
                        .as_ref()
                        .map(|e| e.id)
                });
            } else {
                self.go_back();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_checks_keep_id_key_matches_and_refresh_after_edits() {
        let base = Entry::parse("Rule.md", &note_text("Rule", "rule", None)).unwrap();
        let mut loaded = Loaded::default();
        // Same ID, same key, both, no key, and unreadable notes all occur in a
        // loaded Library. A key collision must mark both distinct IDs.
        for (id, key) in [
            (1, Some("test:a")),
            (2, Some("test:a")),
            (1, Some("test:b")),
            (3, Some("test:b")),
            (1, Some("test:a")),
            (4, None),
            (5, None),
        ] {
            let mut entry = base.clone();
            entry.id = Uuid::from_u128(id);
            entry.key = key.map(str::to_owned);
            loaded.notes.push(Note {
                path: format!("{}.md", loaded.notes.len()),
                text: String::new(),
                entry: Some(entry),
                error: None,
                owner: Owner::Builtin,
            });
        }
        loaded.notes.push(Note {
            path: "Broken.md".into(),
            text: String::new(),
            entry: None,
            error: Some("Invalid YAML".into()),
            owner: Owner::Builtin,
        });
        let check = |loaded: &Loaded| {
            let mut ids = std::collections::HashSet::new();
            for (at, note) in loaded.notes.iter().enumerate() {
                let expected: Vec<_> = loaded
                    .notes
                    .iter()
                    .enumerate()
                    .filter_map(|(other, candidate)| {
                        let target = note.entry.as_ref()?;
                        let candidate = candidate.entry.as_ref()?;
                        (at != other
                            && (target.id == candidate.id
                                || (target.key.is_some() && target.key == candidate.key)))
                            .then_some(other)
                    })
                    .collect();
                assert_eq!(loaded.duplicates(at), expected);
                if !expected.is_empty() {
                    ids.insert(note.entry.as_ref().unwrap().id);
                }
            }
            assert_eq!(loaded.duplicate_ids(), &ids);
        };
        check(&loaded);
        for (at, note) in loaded.notes.iter_mut().enumerate() {
            if let Some(entry) = &mut note.entry {
                entry.id = Uuid::from_u128(at as u128 + 10);
                entry.key = None;
            }
        }
        loaded.invalidate();
        check(&loaded);
        assert!(loaded.duplicate_ids().is_empty());
    }

    #[test]
    fn i20_c1_stat_flags_reach_list_subtitles_and_header_count() {
        let dir = tempfile::tempdir().unwrap();
        let module = create_module(&dir.path().join("book"), "Book", "book").unwrap();
        for (name, kind, table) in [
            (
                "Wolf",
                "creature",
                "## Stat block\n\n| Stat | Value |\n| --- | --- |\n| Hit Points | about twenty |\n",
            ),
            (
                "Spark",
                "spell",
                "## Spell\n\n| Field | Value |\n| --- | --- |\n| Level | many |\n",
            ),
        ] {
            fs::write(module.path.join(format!("{name}.md")), format!(
                "---\nsplat_schema: 2\nsplat_id: {}\nsplat_kind: {kind}\nsplat_name: {name}\n---\n\n{table}", Uuid::new_v4()
            )).unwrap();
        }
        let mut loaded = load(Some(&module)).unwrap();
        loaded.refresh_diagnostics();
        for name in ["Wolf", "Spark"] {
            let at = loaded
                .notes
                .iter()
                .position(|note| note.name() == name)
                .unwrap();
            assert!(loaded.subtitles()[at].contains("1 flagged value"));
        }
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 800.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .show(ctx, |ui| library.show(ui, &mut Campaigns::default()));
            },
        );
        assert!(output.shapes.iter().any(|shape| matches!(&shape.shape, egui::Shape::Text(text) if text.galley.job.text.contains("2 files need fixing"))));
        let diagnostics = &library.loaded.as_ref().unwrap().diagnostics;
        assert!(
            diagnostics
                .iter()
                .any(|s| s.contains("Wolf.md: Hit Points \"about twenty\" not read"))
        );
        assert!(
            diagnostics
                .iter()
                .any(|s| s.contains("Spark.md: Level \"many\" not read"))
        );
    }

    #[test]
    fn generated_old_files_stay_in_the_layout_count() {
        let dir = tempfile::tempdir().unwrap();
        let module = create_module(&dir.path().join("book"), "Book", "book").unwrap();
        fs::write(
            module.path.join("Wolf.md"),
            format!("---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: Wolf\nsplat_creature: {{size: medium, type: beast}}\n---\n", Uuid::new_v4()),
        )
        .unwrap();
        let manifest = module.path.join("Package.md");
        fs::rename(module.path.join("Module.md"), &manifest).unwrap();
        let text = fs::read_to_string(&manifest).unwrap();
        fs::write(
            &manifest,
            content::patch_fields(&text, &[("splat_generated", Some(true.into()))]).unwrap(),
        )
        .unwrap();
        let loaded = load_all(std::slice::from_ref(&module), None).unwrap();
        assert_eq!(loaded.old_layout_count(), 1);
        fs::write(manifest, text).unwrap();
        assert!(loaded.old_layout_count() > 0);
    }

    #[test]
    fn copy_registration_and_undo_keep_campaign_choices_and_later_edits() {
        let temp = tempfile::tempdir().unwrap();
        let world = crate::visual_tests::fixture::World::build(
            temp.path(),
            &temp.path().join("cache"),
            "copy-use",
            crate::visual_tests::fixture::Seed::Campaign,
        )
        .unwrap();
        let mut app = world.app();
        let original = app.campaigns.modules()[0].clone();
        let campaign = app.campaigns.active_campaign().unwrap();
        let campaign_before = CampaignDocument::load(&campaign).unwrap().text;
        let copied = temp.path().join("copied");
        fs::create_dir(&copied).unwrap();
        for entry in content::read_directory(&original.path).unwrap().0 {
            let path = copied.join(&entry.path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::copy(original.path.join(&entry.path), path).unwrap();
        }
        for used in [false, true] {
            let mut library = Library {
                copied_package: Some(local_module(&copied).unwrap()),
                copy_name: "Reaches".into(),
                copy_not_used: !used,
                ..Default::default()
            };
            library.register_copy(&mut app.campaigns).unwrap();
            let id = library.package_copy.as_ref().unwrap().module.id;
            let document = CampaignDocument::load(&campaign).unwrap();
            assert_eq!(
                document
                    .selections
                    .iter()
                    .any(|selection| selection.package.id == Some(id)),
                used
            );
            assert!(
                document
                    .selections
                    .iter()
                    .any(|selection| selection.package.id == Some(original.id))
            );
            let path = copied.join("Module.md");
            let manifest = fs::read_to_string(&path).unwrap();
            fs::write(&path, format!("{manifest}\nLater edit.\n")).unwrap();
            assert!(
                library
                    .undo_package_registration(&mut app.campaigns)
                    .is_err()
            );
            assert_eq!(
                CampaignDocument::load(&campaign).unwrap().text,
                document.text
            );
            fs::write(path, manifest).unwrap();
            library
                .undo_package_registration(&mut app.campaigns)
                .unwrap();
            assert_eq!(
                CampaignDocument::load(&campaign).unwrap().text,
                campaign_before
            );
            assert_eq!(app.campaigns.modules(), std::slice::from_ref(&original));
        }
    }

    fn navigation_library() -> Library {
        let notes = ["First", "Second", "Destination"]
            .into_iter()
            .map(|name| {
                let text = note_text(name, "rule", None);
                Note {
                    path: if name == "Destination" {
                        "Destination.md".into()
                    } else {
                        "Rule.md".into()
                    },
                    entry: Some(Entry::parse("Rule.md", &text).unwrap()),
                    text,
                    error: None,
                    owner: Owner::Package(Module {
                        id: Uuid::new_v4(),
                        name: name.into(),
                        path: name.into(),
                    }),
                }
            })
            .collect();
        Library {
            loaded: Some(Loaded {
                notes,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn i19_w9_header_counts_only_editable_old_files_and_hides_at_zero() {
        for size in [egui::vec2(960.0, 640.0), egui::vec2(1200.0, 800.0)] {
            let mut library = navigation_library();
            let notes = &mut library.loaded.as_mut().unwrap().notes;
            notes[0].owner = Owner::Builtin;
            for index in [0, 2] {
                notes[index].entry = None;
                notes[index].error = Some(content::OLD_LAYOUT.into());
            }

            notes[1]
                .entry
                .as_mut()
                .unwrap()
                .fields
                .insert("splat_schema".into(), 2.into());
            let ctx = egui::Context::default();
            let mut campaigns = Campaigns::default();
            let mut render = |library: &mut Library| {
                let mut frame = || {
                    ctx.run(
                        egui::RawInput {
                            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                            ..Default::default()
                        },
                        |ctx| {
                            egui::CentralPanel::default()
                                .show(ctx, |ui| library.show(ui, &mut campaigns));
                        },
                    )
                };
                frame();
                frame()
                    .shapes
                    .into_iter()
                    .filter_map(|shape| match shape.shape {
                        egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            };
            let text = render(&mut library);
            assert!(
                text.iter()
                    .any(|text| text.ends_with("1 file uses the old layout")),
                "{text:?}"
            );
            library.loaded.as_mut().unwrap().notes[2].owner = Owner::Builtin;
            assert!(
                render(&mut library)
                    .iter()
                    .all(|text| !text.contains("old layout"))
            );
            let notes = &mut library.loaded.as_mut().unwrap().notes;
            notes[0].owner = notes[1].owner.clone();
            notes[2].owner = notes[1].owner.clone();
            assert!(
                render(&mut library)
                    .iter()
                    .any(|text| text.ends_with("2 files use the old layout"))
            );
        }
    }

    #[test]
    fn link_updates_stop_when_a_name_or_filename_is_ambiguous() {
        let mut library = navigation_library();
        let loaded = library.loaded.as_mut().unwrap();
        assert!(loaded.can_rename_links(0, "First", "New name"));
        assert!(!loaded.can_rename_links(0, "First", "Second"));
        loaded.notes[1].path = "Elsewhere/FIRST.md".into();
        assert!(!loaded.can_rename_links(0, "First", "New name"));
        loaded.notes[1].path = "Second.md".into();
        loaded.notes[1].entry.as_mut().unwrap().name = "First".into();
        assert!(!loaded.can_rename_links(0, "First", "New name"));
    }

    #[test]
    fn saving_a_note_moved_to_another_owner_does_not_create_a_second_file() {
        let temp = tempfile::tempdir().unwrap();
        let first = create_module(&temp.path().join("first"), "First", "first").unwrap();
        let second = create_module(&temp.path().join("second"), "Second", "second").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(first.path.join("Wolf.md"), &text).unwrap();
        let loaded = load_all(&[first.clone(), second.clone()], None).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|note| note.path == "Wolf.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: format!("{text}\nMy draft.\n"),
            ..Default::default()
        };
        fs::rename(first.path.join("Wolf.md"), second.path.join("Moved.md")).unwrap();
        library.save_selected(false);
        assert!(library.error.is_none(), "{:?}", library.error);
        assert!(!first.path.join("Wolf.md").exists());
        assert_eq!(
            fs::read_to_string(second.path.join("Moved.md")).unwrap(),
            library.draft
        );
    }

    #[test]
    fn saving_a_moved_and_edited_note_opens_comparison_before_writing() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &text).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|note| note.path == "Wolf.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: format!("{text}\nMy draft.\n"),
            ..Default::default()
        };
        fs::rename(module.path.join("Wolf.md"), module.path.join("Moved.md")).unwrap();
        let outside = format!("{text}\nOutside edit.\n");
        fs::write(module.path.join("Moved.md"), &outside).unwrap();
        library.save_selected(false);
        assert!(library.disk_change.is_some());
        assert_eq!(
            fs::read_to_string(module.path.join("Moved.md")).unwrap(),
            outside
        );
        assert!(library.draft.contains("My draft."));
    }

    #[test]
    fn repairing_a_created_package_replaces_its_actual_manifest() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let (entries, _) = content::read_directory(&module.path).unwrap();
        let path = module.path.join(&entries[0].path);
        let valid = fs::read_to_string(&path).unwrap();
        fs::write(
            &path,
            valid.replace("splat_namespace: test", "splat_namespace: ["),
        )
        .unwrap();
        let repair = crate::content_workspace::ManifestWrite::write(&module).unwrap();
        assert!(repair.replaced());
        assert_eq!(content::read_directory(&module.path).unwrap().0.len(), 1);
        repair.undo().unwrap();
        fs::write(&path, valid).unwrap();
        assert_eq!(local_module(&module.path).unwrap().id, module.id);
    }

    #[test]
    fn copying_without_a_destination_is_blocked() {
        let campaigns = Campaigns::default();
        assert_eq!(
            copy_destination_reason(&campaigns),
            Some("Add a campaign or a package to create entries.")
        );
    }

    #[test]
    fn duplicate_package_id_save_keeps_both_files() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &text).unwrap();
        let copy = format!("{text}\nVariant on disk.\n");
        fs::write(module.path.join("Wolf copy.md"), &copy).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|n| n.path == "Wolf.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: format!("{text}\nEdited original.\n"),
            ..Default::default()
        };
        library.save_selected(false);
        assert!(library.error.is_none());
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf.md")).unwrap(),
            library.draft
        );
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            copy
        );
        let id = crate::content_workspace::note_id(&text).unwrap();
        let catalogue = Catalogue::load(std::slice::from_ref(&module), None).unwrap();
        assert!(!catalogue.entries.contains_key(&id));
        let closure = catalogue.closure(
            &[crate::content_workspace::PackageSelection {
                name: Some(module.name.clone()),
                package: crate::content_workspace::EntryRef::id(module.id),
                selection: crate::content_workspace::Selection::All,
            }],
            Uuid::new_v4(),
        );
        assert!(!closure.direct.contains(&id));
        assert!(closure.errors.iter().any(|error| {
            error.contains("Wolf copy.md and Wolf.md share an identity. Choose one in Library.")
        }));
        assert!(
            catalogue.unreadable[&id]
                .word
                .contains("Wolf copy.md and Wolf.md share an identity. Choose one in Library.")
        );
        let copy_index = library
            .loaded
            .as_ref()
            .unwrap()
            .notes
            .iter()
            .position(|n| n.path == "Wolf copy.md")
            .unwrap();
        library.selected = Some(copy_index);
        library.draft = format!("{copy}\nEdited copy.\n");
        library.save_selected(false);
        let original = fs::read_to_string(module.path.join("Wolf.md")).unwrap();
        let saved_copy = library.draft.clone();
        library.give_new_identity().unwrap();
        assert_ne!(crate::content_workspace::note_id(&library.draft), Some(id));
        let repaired = Entry::parse("Wolf copy.md", &library.draft).unwrap();
        assert_eq!(repaired.name, "Wolf copy");
        assert!(repaired.body.contains("# Wolf copy\n"));
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf.md")).unwrap(),
            original
        );
        assert!(
            Catalogue::load(std::slice::from_ref(&module), None)
                .unwrap()
                .entries
                .contains_key(&id)
        );
        library.undo_new_identity().unwrap();
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            saved_copy
        );
        assert_eq!(
            library
                .loaded
                .as_ref()
                .unwrap()
                .duplicates(copy_index)
                .len(),
            1
        );
        library.give_new_identity().unwrap();
        fs::write(module.path.join("Wolf copy.md"), "A later disk edit.").unwrap();
        assert!(library.undo_new_identity().is_err());
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            "A later disk edit."
        );
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf.md")).unwrap(),
            original
        );
    }

    #[test]
    fn manifest_only_hand_repair_keeps_original_and_both_library_files() {
        let temp = tempfile::tempdir().unwrap();
        let original =
            create_module(&temp.path().join("original"), "Original", "original").unwrap();
        let copy = create_module(&temp.path().join("copy"), "Copy", "copy").unwrap();
        let manifest = fs::read_to_string(original.path.join("Module.md")).unwrap();
        let hand_edit = crate::content_workspace::patch_fields(
            &manifest,
            &[("splat_id", Some(copy.id.to_string().into()))],
        )
        .unwrap();
        fs::write(copy.path.join("Module.md"), hand_edit).unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(original.path.join("Wolf.md"), &text).unwrap();
        fs::write(copy.path.join("Wolf.md"), &text).unwrap();
        let id = crate::content_workspace::note_id(&text).unwrap();
        let modules = [original.clone(), copy];
        let catalogue = Catalogue::load(&modules, None).unwrap();
        assert_eq!(catalogue.entries[&id].owner, original.id);
        assert!(
            catalogue
                .errors
                .iter()
                .any(|e| e.contains("share an identity"))
        );
        let loaded = load_all(&modules, None).unwrap();
        let wolves: Vec<_> = loaded
            .notes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.name() == "Wolf" && n.owner.root().is_some())
            .collect();
        assert_eq!(wolves.len(), 2);
        for (at, _) in wolves {
            assert_eq!(loaded.duplicates(at).len(), 1);
            assert!(loaded.build_subtitles()[at].contains("Duplicate identity"));
        }
    }

    #[test]
    fn duplicate_repair_keeps_key_named_original_after_atomic_save() {
        let temp = tempfile::tempdir().unwrap();
        let original = temp.path().join("Wolf.md");
        let copy = temp.path().join("Wolf copy.md");
        let text = note_text("Wolf", "creature", None).replace(
            "splat_kind: creature",
            "splat_key: test:creatures/wolf\nsplat_kind: creature",
        );
        fs::write(&original, &text).unwrap();
        fs::write(&copy, &text).unwrap();
        write_note(
            temp.path(),
            "Wolf.md",
            Some(&text),
            &format!("{text}\nEdit original.\n"),
        )
        .unwrap();
        assert_eq!(
            crate::content_workspace::duplicate_repair_path([original, copy.clone()]),
            Some(copy)
        );
    }

    #[test]
    fn deleted_duplicate_is_not_treated_as_a_move_to_the_other_file() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Wolf", "creature", None);
        fs::write(module.path.join("Wolf.md"), &text).unwrap();
        fs::write(module.path.join("Wolf copy.md"), &text).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|note| note.path == "Wolf.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: format!("{text}\nMy draft.\n"),
            ..Default::default()
        };
        fs::remove_file(module.path.join("Wolf.md")).unwrap();
        library.refresh_destination(&module.path, &egui::Context::default());
        assert!(library.disk_change.as_ref().unwrap().moved_to.is_none());
        library.save_selected(false);
        assert!(!module.path.join("Wolf.md").exists());
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            text
        );
        library.save_selected(true);
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf.md")).unwrap(),
            library.draft
        );
        assert_eq!(
            fs::read_to_string(module.path.join("Wolf copy.md")).unwrap(),
            text
        );
    }

    #[test]
    fn moved_draft_save_does_not_duplicate_its_id() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        let text = note_text("Moved rule", "rule", None);
        fs::write(module.path.join("Rule.md"), &text).unwrap();
        let loaded = load(Some(&module)).unwrap();
        let selected = loaded
            .notes
            .iter()
            .position(|n| n.path == "Rule.md")
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: format!("{text}\nMy draft.\n"),
            ..Default::default()
        };
        fs::create_dir(module.path.join("Moved")).unwrap();
        fs::rename(
            module.path.join("Rule.md"),
            module.path.join("Moved/Rule.md"),
        )
        .unwrap();
        library.refresh_destination(&module.path, &egui::Context::default());
        library.save_selected(true);
        assert!(
            !module.path.join("Rule.md").exists(),
            "must not recreate the old path"
        );
        assert!(
            fs::read_to_string(module.path.join("Moved/Rule.md"))
                .unwrap()
                .contains("My draft.")
        );
    }

    #[test]
    fn repaired_package_note_reloads_campaign_uses() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test package", "test").unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Test campaign".into(),
            path: temp.path().join("campaign"),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        fs::create_dir_all(&campaign.path).unwrap();
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
        let text = note_text("Test rule", "rule", None);
        let id = Entry::parse("Rule.md", &text).unwrap().id;
        fs::write(module.path.join("Rule.md"), &text).unwrap();
        crate::content_workspace::use_in_campaign(&campaign, module.id, &module.name, Some(id))
            .unwrap();
        assert!(
            load_all(std::slice::from_ref(&module), Some(&campaign))
                .unwrap()
                .uses
                .unwrap()
                .contains(&id)
        );
        fs::write(
            module.path.join("Rule.md"),
            text.replace("splat_kind: rule", "splat_kind: ["),
        )
        .unwrap();
        let loaded = load_all(std::slice::from_ref(&module), Some(&campaign)).unwrap();
        assert!(!loaded.uses.as_ref().unwrap().contains(&id));
        let selected = loaded
            .notes
            .iter()
            .position(|n| n.path == "Rule.md" && n.owner.id() == Some(module.id))
            .unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            selected: Some(selected),
            draft: text.clone(),
            ..Default::default()
        };
        library.save_selected(false);
        assert!(library.error.is_none(), "{:?}", library.error);
        assert!(library.stale, "the next frame must rebuild campaign uses");
        let loaded = load_all(&[module], Some(&campaign)).unwrap();
        assert!(
            loaded.uses.unwrap().contains(&id),
            "the repaired note is Used here"
        );
    }

    #[test]
    fn scope_picker_cuts_two_words_with_one_ellipsis() {
        let ctx = egui::Context::default();
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let budget = design::number_width(ui, "Ashen…", &Type::Strong.font(ui));
                assert_eq!(scope_label(ui, "Ashen Marches", budget), "Ashen…");
                assert_eq!(
                    scope_label(ui, "Ashen Marches", budget / 2.0)
                        .matches('…')
                        .count(),
                    1
                );
            });
        });
    }

    #[test]
    fn creature_template_uses_fields_on_every_read_surface() {
        let text = note_text("Copper Drake", "creature", None);
        assert!(
            !text
                .lines()
                .any(|line| line.trim_start().starts_with("value:"))
        );
        assert!(text.contains("| Speed | 30 ft. |"));
        assert!(text.contains("splat_schema: 2"));
        assert!(!text.contains("splat_stats:"));
        let entry = Entry::parse("Copper Drake.md", &text).unwrap();
        assert!(entry.fields["splat_stats"]["max_hp"].get("value").is_none());
        assert!(
            entry.fields["splat_stats"]["armor_class"]
                .get("value")
                .is_none()
        );
        assert_eq!(
            entry.fields["splat_creature"]["type"].as_str(),
            Some("humanoid")
        );
        for key in [
            "strength",
            "dexterity",
            "constitution",
            "intelligence",
            "wisdom",
            "charisma",
        ] {
            assert_eq!(entry.fields["splat_stats"][key]["value"].as_i64(), Some(10));
        }
        assert_eq!(
            presentation::challenge_text(&entry),
            Some("1 (XP 200; PB +2)")
        );
        assert_eq!(presentation::subtitle(&entry), "Creature · CR 1");
        assert!(!entry.body.contains("Speed 30"));
        assert!(!entry.body.contains("Str 10"));
        assert!(!entry.body.contains("CR 1"));
        let mut conflicting = entry.clone();
        conflicting.body.push_str("\nCR 9 (XP 5,000; PB +4)\n");
        assert_eq!(
            presentation::challenge_text(&conflicting),
            Some("1 (XP 200; PB +2)")
        );
        assert_eq!(presentation::subtitle(&conflicting), "Creature · CR 1");
    }

    #[test]
    fn heading_repair_changes_only_the_first_h1() {
        let text = note_text("Watch Ashfall", "spell", None)
            .replace("# Watch Ashfall", "# Ashfall")
            + "\n# Another heading\n";
        let entry = Entry::parse("Ashfall.md", &text).unwrap();
        let (old, after) = presentation::heading_fix(&text, &entry).unwrap();
        assert_eq!(old, "Ashfall");
        assert_eq!(after, text.replacen("# Ashfall", "# Watch Ashfall", 1));
    }

    #[test]
    fn back_keeps_an_unsaved_homebrew_draft() {
        let mut library = navigation_library();
        library.open_entry(0);
        library.open_entry(1);
        library.draft.push_str("Unsaved ruling.");
        let draft = library.draft.clone();
        library.go_back();
        assert_eq!(library.selected, Some(1));
        assert_eq!(library.draft, draft);
        assert_eq!(library.history.len(), 1);
    }

    #[test]
    fn back_returns_to_the_right_owner_and_scope() {
        let mut library = navigation_library();
        let owner = library.loaded.as_ref().unwrap().notes[1]
            .owner
            .id()
            .unwrap();
        library.scope = Scope::Package(owner);
        library.open_entry(1);
        library.search = "Second".into();
        library.open_entry(2);
        library.scope = Scope::All;
        library.go_back();
        assert_eq!(library.selected, Some(1));
        assert_eq!(library.scope, Scope::Package(owner));
        assert_eq!(library.search, "Second");
    }

    #[test]
    fn following_a_link_reveals_an_entry_outside_the_package_scope() {
        let mut library = navigation_library();
        let owner = library.loaded.as_ref().unwrap().notes[1]
            .owner
            .id()
            .unwrap();
        library.scope = Scope::Package(owner);
        library.open_entry(1);
        library.open_entry(2);
        let loaded = library.loaded.as_ref().unwrap();
        assert!(loaded.in_scope(&loaded.notes[2], library.scope));
        library.go_back();
        assert_eq!(library.scope, Scope::Package(owner));
    }
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
    fn duplicate_package_rows_show_the_folder_before_clipped_details() {
        let ctx = egui::Context::default();
        let output = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(280.0);
                let response = entry_row(
                    ui,
                    "Cinder Wolf",
                    "Duplicate identity · Creature · CR 1 · Ashen Marches · ashen-marches-copy/",
                    false,
                );
                assert_eq!(response.rect.height(), ENTRY_ROW_HEIGHT);
            });
        });
        assert!(output.shapes.iter().any(|shape| matches!(
            &shape.shape,
            egui::Shape::Text(text) if text.galley.job.text.starts_with("ashen-marches-copy/")
        )));
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

    #[test]
    fn reader_names_the_package_once() {
        let package = Owner::Package(Module {
            id: Uuid::new_v4(),
            name: "Ashen Marches".into(),
            path: "ashen".into(),
        });
        // The list leaves the built-in package unsaid, and already names any other.
        assert_eq!(
            reader_subtitle(&Owner::Builtin, "Spell · Level 3", None),
            format!("Spell · Level 3 · {BUILTIN_NAME}")
        );
        assert_eq!(
            reader_subtitle(&package, "Spell · Level 3 · Ashen Marches", None),
            "Spell · Level 3 · Ashen Marches"
        );
        // A class page's own line replaces the list's, so it needs the name too.
        assert_eq!(
            reader_subtitle(&package, "Rule", Some("Bard · Level 3".into())),
            "Class feature · Bard · Level 3 · Ashen Marches"
        );
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
    fn repeated_rule_headings_name_their_subsection() {
        let loaded = load(None).unwrap();
        let subtitles = loaded.subtitles();
        for (suffix, context) in [
            ("ability_modifier", "Ability Checks"),
            ("ability_modifier_2", "Saving Throws"),
            ("ability_modifier_3", "Attack Rolls"),
        ] {
            let row = loaded
                .notes
                .iter()
                .position(|note| {
                    note.entry.as_ref().and_then(|entry| entry.key.as_deref())
                        == Some(format!("dnd2024:rules/d20_tests_{suffix}").as_str())
                })
                .unwrap();
            assert_eq!(subtitles[row], format!("Rule · D20 Tests › {context}"));
        }
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
                "Physically resist direct force",
            ),
            (
                "definitions/rules/d20_tests_ability_modifier.md",
                "Notice things in the environment or in creatures’ behavior",
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
                        "| Weal | Good |\n| Woe | Bad |\n| Weal and woe | Good and bad |"
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
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, &subtitles[0], &Campaigns::default())
                    });
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
                            egui::CentralPanel::default().show(ctx, |ui| {
                                library.editor(ui, "Definition · creature", &Campaigns::default())
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
                egui::CentralPanel::default().show(ctx, |ui| {
                    library.editor(ui, "Definition · creature", &Campaigns::default())
                });
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
                .contains("of Lore subclass is detailed after this class’s description")
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
                            library.editor(
                                ui,
                                "Definition · class feature · Bard",
                                &Campaigns::default(),
                            )
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
                "Bard · Level 3 · D&D 2024 · SRD 5.2.1",
                "Choose a Bard subclass.",
                "Subclasses",
                "Source text",
            ] {
                assert!(rendered.contains(expected), "{rendered}");
            }
            assert!(!rendered.contains("You gain a Bard subclass of your choice."));
            // The published wording is still readable, and the related link navigates.
            for label in ["Source text", "College of Lore"] {
                let current = frame(vec![]);
                let text = current
                    .shapes
                    .iter()
                    .filter_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) => Some(text),
                        _ => None,
                    })
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

    #[test]
    fn library_toolbar_keeps_back_visible_with_more_menu() {
        let loaded = load(None).unwrap();
        let mut library = Library {
            loaded: Some(loaded),
            ..Default::default()
        };
        library.open_entry(0);
        library.open_entry(1);
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let mut rendered = Vec::new();
        for _ in 0..3 {
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(360.0, 640.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Entry", &Campaigns::default());
                    });
                },
            );
            rendered = output
                .shapes
                .into_iter()
                .filter_map(|shape| match shape.shape {
                    egui::Shape::Text(text)
                        if shape.clip_rect.contains_rect(text.visual_bounding_rect()) =>
                    {
                        Some(text.galley.job.text.clone())
                    }
                    _ => None,
                })
                .collect();
        }
        for word in ["Read", "Back", "More"] {
            assert!(
                rendered.iter().any(|text| text == word),
                "Missing {word}: {rendered:?}"
            );
        }
    }

    #[test]
    fn source_editor_keeps_its_frame_inside_the_remaining_height() {
        for size in [egui::vec2(480.0, 480.0), egui::vec2(960.0, 920.0)] {
            let source = format!(
                "{}\n{}",
                note_text("Long rule", "rule", None),
                "Long source line.\n".repeat(200)
            );
            let mut library = Library {
                loaded: Some(Loaded {
                    notes: vec![Note {
                        path: "Long rule.md".into(),
                        text: source.clone(),
                        entry: Some(Entry::parse("Long rule.md", &source).unwrap()),
                        error: None,
                        owner: Owner::Builtin,
                    }],
                    ..Default::default()
                }),
                selected: Some(0),
                source: true,
                draft: source,
                ..Default::default()
            };
            let ctx = egui::Context::default();
            crate::design::theme::install(&ctx);
            for _ in 0..3 {
                let output = ctx.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            library.editor(ui, "Rule", &Campaigns::default());
                        });
                    },
                );
                let frames: Vec<_> = output
                    .shapes
                    .iter()
                    .filter_map(|shape| {
                        if let egui::Shape::Rect(rect) = &shape.shape
                            && rect.stroke.color == colors().field_border
                            && rect.rect.height() > design::XXL
                        {
                            Some((rect.rect, shape.clip_rect))
                        } else {
                            None
                        }
                    })
                    .collect();
                assert!(!frames.is_empty(), "The source needs a visible frame");
                for (frame, clip) in frames {
                    assert!(
                        clip.expand(design::HAIRLINE_WIDTH).contains_rect(frame),
                        "Frame {frame:?} leaves viewport {clip:?}"
                    );
                    assert!(
                        frame.bottom() <= size.y,
                        "Frame leaves the window: {frame:?}"
                    );
                }
            }
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
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Definition", &Campaigns::default())
                    });
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
                    egui::CentralPanel::default().show(ctx, |ui| {
                        library.editor(ui, "Definition · class", &Campaigns::default())
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
        // Stat-sized table cells can extend sideways inside their scroll area.
        // Prose and feature links must still fit without horizontal scrolling.
        assert!(output.shapes.iter().all(|shape| {
            let egui::Shape::Text(text) = &shape.shape else {
                return true;
            };
            text.pos.x + text.galley.size().x <= 390.0
                || (shape.clip_rect.right() <= 390.0
                    && text
                        .galley
                        .job
                        .sections
                        .iter()
                        .all(|section| section.format.font_id.size == Type::Stat.size()))
        }));
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
        // The class table names the feature too. The link is in the list below it.
        assert!(rendered.contains("Class table"));
        let link = texts
            .iter()
            .rfind(|text| text.galley.job.text == "Relentless Rage")
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
                        egui::CentralPanel::default().show(ctx, |ui| {
                            library.editor(ui, "Definition · creature", &Campaigns::default())
                        });
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
        let loaded = load_all(std::slice::from_ref(&m), None).unwrap();
        assert!(
            loaded
                .diagnostics
                .iter()
                .any(|d| d.starts_with("Broken.md"))
        );
    }

    #[test]
    fn package_manifest_failure_keeps_library_entries_and_a_named_warning() {
        let temp = tempfile::tempdir().unwrap();
        let module = create_module(&temp.path().join("package"), "Test", "test").unwrap();
        fs::rename(
            module.path.join("Module.md"),
            module.path.join("Package.md"),
        )
        .unwrap();
        fs::write(
            module.path.join("Wolf.md"),
            note_text("Wolf", "creature", None),
        )
        .unwrap();
        fs::write(module.path.join("Package.md"), "---\nsplat_id: [\n---\n").unwrap();
        for deleted in [false, true] {
            if deleted {
                fs::remove_file(module.path.join("Package.md")).unwrap();
            }
            let loaded = load_all(std::slice::from_ref(&module), None).unwrap();
            assert!(loaded.notes.iter().any(|note| note.name() == "Wolf"));
            assert!(
                loaded
                    .diagnostics
                    .iter()
                    .any(|message| message.contains("Package.md"))
            );
        }
    }

    /// Writes a package whose one item needs `needs`.
    fn package_needing(root: &Path, name: &str, namespace: &str, needs: &str) -> Module {
        let module = create_module(&root.join(name), name, namespace).unwrap();
        let note = splat::content::formats::convert("Fixture.md", &format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: item\nsplat_key: {namespace}:items/own\n\
             splat_name: Own\nsplat_item: {{category: gear}}\nsplat_requires:\n  - key: {needs}\n---\n",
            Uuid::new_v4()
        )).expect("schema-2 fixture");
        fs::write(module.path.join("Own.md"), note).unwrap();
        module
    }

    #[test]
    fn references_resolve_across_the_whole_library() {
        let temp = tempfile::tempdir().unwrap();
        let modules = [
            // One reference to built-in content, and one to the other package.
            package_needing(temp.path(), "First", "first", "dnd2024:equipment/battleaxe"),
            package_needing(temp.path(), "Second", "second", "first:items/own"),
        ];
        let mut loaded = load_all(&modules, None).unwrap();
        assert_eq!(loaded.diagnostics, Vec::<String>::new());
        // A save checks again, and that check must agree with the first.
        loaded.refresh_diagnostics();
        assert_eq!(loaded.diagnostics, Vec::<String>::new());
    }

    #[test]
    fn a_reference_to_nothing_is_still_reported() {
        let temp = tempfile::tempdir().unwrap();
        let module = package_needing(temp.path(), "First", "first", "missing:items/gone");
        let loaded = load_all(&[module], None).unwrap();
        assert_eq!(loaded.diagnostics.len(), 1, "{:?}", loaded.diagnostics);
        assert!(loaded.diagnostics[0].contains("missing:items/gone"));
    }

    #[test]
    fn a_save_keeps_the_problems_that_kept_content_from_loading() {
        let temp = tempfile::tempdir().unwrap();
        let mut module = package_needing(temp.path(), "First", "first", "first:items/own");
        module.path = temp.path().join("deleted");
        let mut loaded = load_all(&[module], None).unwrap();
        assert_eq!(loaded.diagnostics.len(), 1, "{:?}", loaded.diagnostics);
        loaded.refresh_diagnostics();
        assert_eq!(loaded.diagnostics.len(), 1, "{:?}", loaded.diagnostics);
    }
}
