//! Content ownership, campaign selections, and independent copies.
use crate::{
    library::{note_path, note_text, write_note},
    persistence::{Campaign, Module},
};
use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;
use splat::content::{self, Entry, Reference};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub fn builtin_manifest() -> Entry {
    let bytes = content::builtin::source_file("Package.md").expect("bundled manifest");
    Entry::parse(
        "Package.md",
        std::str::from_utf8(bytes).expect("UTF-8 manifest"),
    )
    .expect("valid manifest")
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EntryRef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
impl EntryRef {
    pub fn id(id: Uuid) -> Self {
        Self {
            id: Some(id),
            key: None,
            link: None,
        }
    }
    pub fn reference(&self) -> Result<Reference, String> {
        match (&self.id, &self.key) {
            (Some(id), None) if !id.is_nil() => Ok(Reference::Id(*id)),
            (None, Some(key)) if content::resource_key(key) => Ok(Reference::Key(key.clone())),
            _ => Err("A content reference needs exactly one valid UUID or key.".into()),
        }
    }
    pub fn label(&self) -> String {
        self.id
            .map(|id| id.to_string())
            .or(self.key.clone())
            .unwrap_or_default()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum Selection {
    All,
    Selected { entries: Vec<EntryRef> },
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PackageSelection {
    pub package: EntryRef,
    pub selection: Selection,
}

pub struct ModuleIndex {
    pub id: Uuid,
    pub name: String,
    pub entries: Vec<Uuid>,
}
pub struct IndexedEntry {
    pub entry: Entry,
    pub owner: Uuid,
    pub private: bool,
}
#[derive(Default)]
pub struct Catalogue {
    pub modules: Vec<ModuleIndex>,
    pub entries: BTreeMap<Uuid, IndexedEntry>,
    keys: BTreeMap<String, Uuid>,
    pub errors: Vec<String>,
    files: BTreeMap<Uuid, (PathBuf, String)>,
    namespaces: BTreeSet<String>,
    neighbours: BTreeMap<String, String>,
}
/// Names, by namespace, of the packages in folders beside the registered ones. A reference
/// holds only a namespace, so this is what lets a missing package appear under its own name.
fn neighbouring_packages(modules: &[Module]) -> BTreeMap<String, String> {
    let parents: BTreeSet<_> = modules.iter().filter_map(|m| m.path.parent()).collect();
    parents
        .into_iter()
        .filter_map(|parent| fs::read_dir(parent).ok())
        .flatten()
        .flatten()
        .filter_map(|folder| {
            let text = fs::read_to_string(folder.path().join("Package.md")).ok()?;
            let manifest = Entry::parse("Package.md", &text).ok()?;
            let namespace = manifest.fields.get("splat_namespace")?.as_str()?.to_owned();
            (manifest.kind == "package").then_some((namespace, manifest.name))
        })
        .collect()
}
/// A short list of the entries that need something, for an issue that names many of them.
fn needed_by(names: &BTreeSet<String>) -> String {
    let shown: Vec<_> = names.iter().take(3).map(String::as_str).collect();
    match names.len().saturating_sub(shown.len()) {
        0 => shown.join(", "),
        more => format!("{} and {more} more", shown.join(", ")),
    }
}
impl Catalogue {
    fn insert(&mut self, entry: Entry, owner: Uuid, private: bool) -> Result<(), String> {
        if self.entries.contains_key(&entry.id) {
            return Err(format!("Duplicate identity: {}", entry.name));
        }
        if let Some(key) = &entry.key {
            if self.keys.contains_key(key) {
                return Err(format!("Duplicate key: {key}"));
            }
            self.keys.insert(key.clone(), entry.id);
        }
        self.entries.insert(
            entry.id,
            IndexedEntry {
                entry,
                owner,
                private,
            },
        );
        Ok(())
    }
    pub fn resolve(&self, reference: &Reference) -> Result<Uuid, String> {
        let id = match reference {
            Reference::Id(id) => Some(*id),
            Reference::Key(key) => self.keys.get(key).copied(),
        };
        id.filter(|id| self.entries.contains_key(id))
            .ok_or_else(|| format!("Unavailable content: {reference}"))
    }
    pub fn resolve_ref(&self, reference: &EntryRef) -> Result<Uuid, String> {
        self.resolve(&reference.reference()?)
    }
    /// The package an unresolved reference points into, when no such package is loaded.
    fn missing_package(&self, reference: &Reference) -> Option<String> {
        let Reference::Key(key) = reference else {
            return None;
        };
        let (namespace, _) = key.split_once(':')?;
        (!self.namespaces.contains(namespace)).then(|| {
            self.neighbours
                .get(namespace)
                .cloned()
                .unwrap_or_else(|| namespace.to_owned())
        })
    }
    pub fn load(modules: &[Module], campaign: Option<&Campaign>) -> Result<Self, String> {
        let mut result = Self {
            namespaces: BTreeSet::from(["dnd2024".to_owned()]),
            neighbours: neighbouring_packages(modules),
            ..Self::default()
        };
        let builtin = builtin_manifest();
        let builtin_id = builtin.id;
        let definitions = content::definitions::entries()?;
        let entries = || content::builtin::entries().iter().chain(definitions);
        let ids = entries().map(|e| e.id).collect();
        for entry in entries() {
            result.insert(entry.clone(), builtin_id, false)?;
        }
        result.modules.push(ModuleIndex {
            id: builtin_id,
            name: "D&D 2024 · SRD 5.2.1".into(),
            entries: ids,
        });
        for module in modules {
            let read = content::read_directory(&module.path);
            let (entries, errors) = match read {
                Ok(v) => v,
                Err(e) => {
                    result.errors.push(format!("{}: {e}", module.name));
                    continue;
                }
            };
            result.errors.extend(
                errors
                    .into_iter()
                    .map(|e| format!("{} / {}: {}", module.name, e.path, e.message)),
            );
            let manifests: Vec<_> = entries.iter().filter(|e| e.kind == "package").collect();
            if manifests.len() != 1
                || manifests[0].id != module.id
                || entries.iter().any(|e| e.kind == "campaign")
            {
                result.errors.push(format!(
                    "{}: package identity or ownership does not match its registration.",
                    module.name
                ));
                continue;
            }
            let ns = manifests[0]
                .fields
                .get("splat_namespace")
                .and_then(Value::as_str)
                .ok_or("Missing package namespace")?;
            if !result.namespaces.insert(ns.to_owned()) {
                return Err(format!("Duplicate package namespace: {ns}"));
            }
            let ids = entries.iter().map(|e| e.id).collect();
            let name = manifests[0].name.clone();
            for entry in entries {
                let path = note_path(&module.path, &entry.path)?;
                result.files.insert(
                    entry.id,
                    (
                        path.clone(),
                        fs::read_to_string(&path).map_err(|e| e.to_string())?,
                    ),
                );
                result.insert(entry, module.id, false)?;
            }
            result.modules.push(ModuleIndex {
                id: module.id,
                name,
                entries: ids,
            });
        }
        if let Some(campaign) = campaign {
            let (entries, errors) = content::read_directory(&campaign.path)?;
            result.errors.extend(
                errors
                    .into_iter()
                    .map(|e| format!("{}: {}", e.path, e.message)),
            );
            for entry in entries {
                if entry.kind == "package" {
                    return Err("A campaign cannot contain a shared package manifest.".into());
                }
                if entry.kind == "campaign" && entry.id != campaign.id {
                    return Err("Campaign manifest identity differs from settings.".into());
                }
                if entry.kind == "campaign" {
                    let ns = entry
                        .fields
                        .get("splat_namespace")
                        .and_then(Value::as_str)
                        .ok_or("Missing campaign namespace")?;
                    if !result.namespaces.insert(ns.into()) {
                        return Err(format!("Duplicate owner namespace: {ns}"));
                    }
                }
                let path = note_path(&campaign.path, &entry.path)?;
                result.files.insert(
                    entry.id,
                    (
                        path.clone(),
                        fs::read_to_string(&path).map_err(|e| e.to_string())?,
                    ),
                );
                result.insert(entry, campaign.id, true)?;
            }
        }
        Ok(result)
    }
    pub fn selectable(&self, id: Uuid) -> bool {
        self.entries
            .get(&id)
            .is_some_and(|e| !["package", "reference", "campaign"].contains(&e.entry.kind.as_str()))
    }
    pub fn closure(&self, selections: &[PackageSelection], campaign: Uuid) -> Closure {
        let mut result = Closure::default();
        for selection in selections {
            let module = match self
                .resolve_ref(&selection.package)
                .ok()
                .and_then(|id| self.modules.iter().find(|m| m.id == id))
            {
                Some(module) => module,
                None => {
                    result.errors.push(format!(
                        "Selected package unavailable: {}",
                        selection.package.label()
                    ));
                    continue;
                }
            };
            match &selection.selection {
                Selection::All => result.direct.extend(module.entries.iter().copied()),
                Selection::Selected { entries } => {
                    for reference in entries {
                        match self.resolve_ref(reference) {
                            Ok(id) if self.entries[&id].owner == module.id => {
                                result.direct.insert(id);
                            }
                            _ => result.errors.push(format!(
                                "Selected entry unavailable in {}: {}",
                                module.name,
                                reference.label()
                            )),
                        }
                    }
                }
            }
        }
        let mut queue: VecDeque<_> = result.direct.iter().copied().collect();
        queue.extend(
            self.entries
                .iter()
                .filter(|(_, e)| e.private && e.owner == campaign)
                .map(|(id, _)| *id),
        );
        let mut visited = BTreeSet::new();
        // One issue for each thing that is missing, however many entries need it.
        let mut missing_packages = BTreeSet::new();
        let mut missing_entries = BTreeMap::<String, BTreeSet<String>>::new();
        while let Some(id) = queue.pop_front() {
            if !visited.insert(id) {
                continue;
            }
            let entry = &self.entries[&id];
            match content::required_references(&entry.entry) {
                Err(e) => result.errors.push(format!("{}: {e}", entry.entry.name)),
                Ok(refs) => {
                    for reference in refs {
                        match self.resolve(&reference) {
                            Ok(dep) => {
                                if !entry.private && self.entries[&dep].private {
                                    result.errors.push(format!(
                                        "{} requires private campaign content: {}",
                                        entry.entry.name, self.entries[&dep].entry.name
                                    ));
                                    continue;
                                }
                                if !result.direct.contains(&dep) {
                                    result
                                        .required
                                        .entry(dep)
                                        .or_insert_with(BTreeSet::new)
                                        .insert(entry.entry.name.clone());
                                }
                                queue.push_back(dep);
                            }
                            Err(_) => match self.missing_package(&reference) {
                                Some(package) => {
                                    missing_packages.insert(package);
                                }
                                None => {
                                    missing_entries
                                        .entry(reference.to_string())
                                        .or_default()
                                        .insert(entry.entry.name.clone());
                                }
                            },
                        }
                    }
                }
            }
        }
        result.errors.extend(
            missing_packages
                .iter()
                .map(|package| format!("Package not found: {package}")),
        );
        result
            .errors
            .extend(missing_entries.iter().map(|(reference, names)| {
                format!(
                    "Entry not found: {reference} · needed by {}",
                    needed_by(names)
                )
            }));
        // Cycles between named rule groups are invalid selection units.
        fn visit(
            cat: &Catalogue,
            id: Uuid,
            active: &mut BTreeSet<Uuid>,
            done: &mut BTreeSet<Uuid>,
        ) -> bool {
            if done.contains(&id) {
                return false;
            }
            if !active.insert(id) {
                return true;
            }
            if let Ok(refs) = content::required_references(&cat.entries[&id].entry) {
                for r in refs {
                    if let Ok(dep) = cat.resolve(&r)
                        && cat.entries[&dep].entry.kind == "rule_group"
                        && visit(cat, dep, active, done)
                    {
                        return true;
                    }
                }
            }
            active.remove(&id);
            done.insert(id);
            false
        }
        let mut done = BTreeSet::new();
        for id in visited
            .iter()
            .filter(|id| self.entries[id].entry.kind == "rule_group")
        {
            if visit(self, *id, &mut BTreeSet::new(), &mut done) {
                result
                    .errors
                    .push("Rule-group dependency cycle. Repair the group references.".into());
                break;
            }
        }
        result
    }
}
#[derive(Default)]
pub struct Closure {
    pub direct: BTreeSet<Uuid>,
    pub required: BTreeMap<Uuid, BTreeSet<String>>,
    pub errors: Vec<String>,
}

/// Replace only named top-level fields; all other bytes, comments and body stay intact.
pub fn patch_fields(text: &str, changes: &[(&str, Option<Value>)]) -> Result<String, String> {
    let prefix = if text.starts_with('\u{feff}') {
        "\u{feff}"
    } else {
        ""
    };
    let raw = text.trim_start_matches('\u{feff}');
    let newline = if raw.starts_with("---\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let start = format!("---{newline}");
    let raw = raw.strip_prefix(&start).ok_or("Missing frontmatter")?;
    let end = format!("{newline}---");
    let index = raw.find(&end).ok_or("Missing frontmatter end")?;
    let front = &raw[..index];
    let value: Value = serde_yaml_ng::from_str(front).map_err(|e| e.to_string())?;
    let map = value.as_mapping().ok_or("Frontmatter must be a mapping")?;
    let lines: Vec<_> = front.split(newline).collect();
    let mut remove = BTreeSet::new();
    for (key, _) in changes {
        if map.contains_key(*key) {
            let matches: Vec<_> = lines
                .iter()
                .enumerate()
                .filter(|(_, line)| line.starts_with(&format!("{key}:")))
                .map(|(i, _)| i)
                .collect();
            if matches.len() != 1 {
                return Err(format!(
                    "Cannot safely edit {key}. Use a plain, unquoted top-level field in Markdown first."
                ));
            }
            let start = matches[0];
            let mut stop = start + 1;
            while stop < lines.len() {
                let line = lines[stop];
                if !line.is_empty() && !line.starts_with([' ', '\t', '-', '#']) {
                    break;
                }
                // Keep comments and blank lines; removing a changed field's data is intentional.
                if !line.trim().is_empty() && !line.trim_start().starts_with('#') {
                    remove.insert(stop);
                }
                stop += 1;
            }
            remove.insert(start);
        }
    }
    let mut output = format!("{prefix}{start}");
    for (i, line) in lines.iter().enumerate() {
        if !remove.contains(&i) {
            output.push_str(line);
            output.push_str(newline);
        }
    }
    for (key, value) in changes {
        if let Some(value) = value {
            let mut field = serde_yaml_ng::Mapping::new();
            field.insert((*key).into(), value.clone());
            let yaml = serde_yaml_ng::to_string(&field).map_err(|e| e.to_string())?;
            output.push_str(&yaml.replace('\n', newline));
        }
    }
    output.push_str(raw[index + newline.len()..].as_ref());
    Ok(output)
}

fn campaign_manifest(root: &Path) -> Result<Option<Entry>, String> {
    let (entries, errors) = content::read_directory(root)?;
    if !errors.is_empty() && !entries.iter().any(|e| e.kind == "campaign") {
        return Err("Repair invalid notes before creating a campaign manifest.".into());
    }
    if entries.iter().any(|e| e.kind == "package") {
        return Err("A campaign folder cannot contain a package manifest.".into());
    }
    let mut manifests = entries.into_iter().filter(|e| e.kind == "campaign");
    let manifest = manifests.next();
    if manifests.next().is_some() {
        return Err("A campaign needs exactly one manifest.".into());
    }
    Ok(manifest)
}

fn campaign_metadata(entry: &Entry, mut campaign: Campaign) -> Result<Campaign, String> {
    campaign.name = entry.name.clone();
    for (field, target) in [
        (
            "splat_dndbeyond_campaign_id",
            &mut campaign.dndbeyond_campaign_id,
        ),
        (
            "splat_dndbeyond_campaign_name",
            &mut campaign.dndbeyond_campaign_name,
        ),
    ] {
        if let Some(value) = entry.fields.get(field) {
            *target = serde_yaml_ng::from_value(value.clone())
                .map_err(|_| format!("Invalid {field}."))?;
        }
    }
    if let Some(id) = &mut campaign.dndbeyond_campaign_id {
        *id = id
            .parse::<u64>()
            .ok()
            .filter(|id| *id > 0)
            .ok_or("Invalid D&D Beyond campaign ID.")?
            .to_string();
    }
    Ok(campaign)
}

pub struct CampaignDocument {
    pub campaign: Campaign,
    pub path: String,
    pub baseline: Option<String>,
    pub text: String,
    pub selections: Vec<PackageSelection>,
}
impl CampaignDocument {
    pub fn discover(root: &Path) -> Result<Option<Campaign>, String> {
        let root = root.canonicalize().map_err(|e| e.to_string())?;
        campaign_manifest(&root)?
            .map(|entry| {
                campaign_metadata(
                    &entry,
                    Campaign {
                        id: entry.id,
                        name: entry.name.clone(),
                        path: root,
                        dndbeyond_campaign_id: None,
                        dndbeyond_campaign_name: None,
                    },
                )
            })
            .transpose()
    }

    pub fn reload_metadata(campaign: &mut Campaign) -> Result<(), String> {
        if !campaign.path.exists() {
            return Ok(());
        }
        let document = Self::load(campaign)?;
        let entry = Entry::parse(&document.path, &document.text)?;
        *campaign = campaign_metadata(&entry, campaign.clone())?;
        if !entry.fields.contains_key("splat_dndbeyond_campaign_id") {
            Self::save_metadata(campaign, None)?;
        }
        Ok(())
    }

    pub fn save_metadata(campaign: &Campaign, previous: Option<&Campaign>) -> Result<(), String> {
        let document = Self::load(campaign)?;
        let entry = Entry::parse(&document.path, &document.text)?;
        if let Some(previous) = previous
            && entry.fields.contains_key("splat_dndbeyond_campaign_id")
            && campaign_metadata(&entry, previous.clone())? != *previous
        {
            return Err("Campaign settings changed on disk. Refresh before saving.".into());
        }
        let text = patch_fields(
            &document.text,
            &[
                ("splat_name", Some(campaign.name.clone().into())),
                (
                    "splat_dndbeyond_campaign_id",
                    Some(
                        serde_yaml_ng::to_value(&campaign.dndbeyond_campaign_id)
                            .map_err(|e| e.to_string())?,
                    ),
                ),
                (
                    "splat_dndbeyond_campaign_name",
                    Some(
                        serde_yaml_ng::to_value(&campaign.dndbeyond_campaign_name)
                            .map_err(|e| e.to_string())?,
                    ),
                ),
            ],
        )?;
        write_note(
            &campaign.path,
            &document.path,
            document.baseline.as_deref(),
            &text,
        )?;
        Ok(())
    }

    pub fn load(campaign: &Campaign) -> Result<Self, String> {
        let (path, baseline, text) = if let Some(manifest) = campaign_manifest(&campaign.path)? {
            if manifest.id != campaign.id {
                return Err("Campaign manifest identity differs from its registration. Repair the mismatch before saving.".into());
            }
            let text = fs::read_to_string(note_path(&campaign.path, &manifest.path)?)
                .map_err(|e| e.to_string())?;
            (manifest.path.clone(), Some(text.clone()), text)
        } else {
            let text = note_text(
                &campaign.name,
                "campaign",
                Some(&format!("campaign_{}", campaign.id.simple())),
            );
            let text = patch_fields(
                &text,
                &[
                    ("splat_id", Some(campaign.id.to_string().into())),
                    (
                        "splat_packages",
                        Some(
                            serde_yaml_ng::to_value(vec![PackageSelection {
                                package: EntryRef::id(builtin_manifest().id),
                                selection: Selection::All,
                            }])
                            .map_err(|e| e.to_string())?,
                        ),
                    ),
                ],
            )?;
            ("Campaign.md".into(), None, text)
        };
        let entry = Entry::parse(&path, &text)?;
        let selections: Vec<PackageSelection> = match entry.fields.get("splat_packages") {
            Some(value) => serde_yaml_ng::from_value(value.clone())
                .map_err(|e| format!("Invalid package selection: {e}"))?,
            None => Vec::new(),
        };
        let mut seen = BTreeSet::new();
        for selection in &selections {
            selection.package.reference()?;
            if !seen.insert(selection.package.label()) {
                return Err("Duplicate package selection.".into());
            }
            if let Selection::Selected { entries } = &selection.selection {
                for entry in entries {
                    entry.reference()?;
                }
            }
        }
        Ok(Self {
            campaign: campaign.clone(),
            path,
            baseline,
            text,
            selections,
        })
    }
    pub fn save(&mut self) -> Result<(), String> {
        let text = patch_fields(
            &self.text,
            &[(
                "splat_packages",
                Some(serde_yaml_ng::to_value(&self.selections).map_err(|e| e.to_string())?),
            )],
        )?;
        write_note(
            &self.campaign.path,
            &self.path,
            self.baseline.as_deref(),
            &text,
        )?;
        self.baseline = Some(text.clone());
        self.text = text;
        Ok(())
    }
}

/// Recheck destination ownership before a reviewed copy is written.
pub fn check_destination(root: &Path, id: Uuid, private: bool) -> Result<(), String> {
    let (entries, errors) = content::read_directory(root)?;
    if !errors.is_empty() {
        return Err(format!(
            "Repair invalid destination notes first: {}",
            errors[0].message
        ));
    }
    let manifests: Vec<_> = entries
        .iter()
        .filter(|e| ["package", "campaign"].contains(&e.kind.as_str()))
        .collect();
    if private && manifests.is_empty() {
        return Ok(());
    }
    if manifests.len() != 1
        || manifests[0].id != id
        || manifests[0].kind != if private { "campaign" } else { "package" }
    {
        return Err(
            "The destination owner changed. Refresh its registration before copying.".into(),
        );
    }
    Ok(())
}

#[derive(Clone)]
pub struct CopySource {
    pub entry: Entry,
    pub text: String,
    pub root: Option<PathBuf>,
    pub campaign: Option<Campaign>,
}
pub struct CopyPlan {
    pub root: PathBuf,
    pub relative: String,
    pub text: String,
    source_path: Option<PathBuf>,
    source_text: String,
    dependency_files: Vec<(PathBuf, String)>,
    pub dependencies: Vec<String>,
}
impl CopyPlan {
    pub fn prepare(
        source: &CopySource,
        destination: &Path,
        destination_campaign: Option<Uuid>,
        name: &str,
        folder: &str,
        catalogue: &Catalogue,
    ) -> Result<Self, String> {
        if ["package", "campaign", "reference"].contains(&source.entry.kind.as_str()) {
            return Err("Copy a content entry, not an owner manifest or source archive.".into());
        }
        if name.trim().is_empty()
            || name.contains(['/', '\\', '\n', '\r'])
            || [".", ".."].contains(&name)
        {
            return Err("Enter a note name without path separators or line breaks.".into());
        }
        let mut dependencies = BTreeSet::new();
        let mut queue = VecDeque::from([source.entry.clone()]);
        let mut seen = BTreeSet::from([source.entry.id]);
        while let Some(entry) = queue.pop_front() {
            for reference in content::required_references(&entry)? {
                let id = catalogue.resolve(&reference)?;
                // A single-entry copy cannot safely rewrite arbitrary self-references in Markdown tables.
                if id == source.entry.id {
                    return Err("This entry refers to itself. Update the self-reference explicitly before making an independent copy.".into());
                }
                let dependency = &catalogue.entries[&id];
                if dependency.private && destination_campaign != Some(dependency.owner) {
                    return Err(format!(
                        "Cannot share {}: it requires private campaign entry ‘{}’. Share or replace that dependency first.",
                        source.entry.name, dependency.entry.name
                    ));
                }
                dependencies.insert(dependency.entry.name.clone());
                if seen.insert(id) {
                    queue.push_back(dependency.entry.clone());
                }
            }
        }
        let mut text = patch_fields(
            &source.text,
            &[
                ("splat_id", Some(Uuid::new_v4().to_string().into())),
                ("splat_key", None),
                ("splat_name", Some(name.trim().into())),
                (
                    "splat_origin",
                    Some(
                        serde_yaml_ng::to_value(EntryRef::id(source.entry.id))
                            .map_err(|e| e.to_string())?,
                    ),
                ),
            ],
        )?;
        // Preserve the full original body and source citations; append attribution for bundled material.
        if source
            .entry
            .key
            .as_ref()
            .is_some_and(|k| k.starts_with("dnd2024:"))
            && !text.contains(content::builtin::ATTRIBUTION)
        {
            text.push_str("\n\n");
            text.push_str(content::builtin::ATTRIBUTION);
            text.push('\n');
        }
        let relative = if folder.trim().is_empty() || folder == "." {
            format!("{}.md", name.trim())
        } else {
            format!("{}/{}.md", folder.trim().trim_end_matches('/'), name.trim())
        };
        let path = note_path(destination, &relative)?;
        if path.exists() {
            return Err(
                "A note already exists at this destination. Choose another name or folder.".into(),
            );
        }
        if !path.parent().is_some_and(Path::is_dir) {
            return Err("The destination subfolder must already exist.".into());
        }
        Entry::parse(&relative, &text)?;
        Ok(Self {
            root: destination.into(),
            relative,
            text,
            dependency_files: seen
                .iter()
                .filter_map(|id| catalogue.files.get(id).cloned())
                .collect(),
            dependencies: dependencies.into_iter().collect(),
            source_path: source
                .root
                .as_ref()
                .map(|r| note_path(r, &source.entry.path))
                .transpose()?,
            source_text: source.text.clone(),
        })
    }
    pub fn save(&self) -> Result<Entry, String> {
        for (path, original) in &self.dependency_files {
            if fs::read_to_string(path).map_err(|e| e.to_string())? != *original {
                return Err("A required entry changed after review. Close the dialog and review a new copy.".into());
            }
        }
        if let Some(path) = &self.source_path
            && fs::read_to_string(path).map_err(|e| e.to_string())? != self.source_text
        {
            return Err("The source changed on disk. Refresh it and review a new copy.".into());
        }
        write_note(&self.root, &self.relative, None, &self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn entry(name: &str, kind: &str) -> Entry {
        Entry::parse(&format!("{name}.md"), &note_text(name, kind, None)).unwrap()
    }
    fn campaign(path: PathBuf) -> Campaign {
        Campaign {
            id: Uuid::new_v4(),
            name: "Test campaign".into(),
            path,
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        }
    }
    fn require(entry: &mut Entry, id: Uuid) {
        entry.fields.insert(
            "splat_requires".into(),
            serde_yaml_ng::to_value(vec![EntryRef::id(id)]).unwrap(),
        );
    }
    #[test]
    fn selections_keep_dependency_reasons_and_optional_entries_separate() {
        let owner = Uuid::new_v4();
        let mut cat = Catalogue::default();
        let mut manifest = entry("Module", "rule");
        manifest.kind = "package".into();
        manifest.id = owner;
        cat.insert(manifest, owner, false).unwrap();
        let dep = entry("Required spell", "spell");
        let dep_id = dep.id;
        let optional = entry("Optional item", "item");
        let optional_id = optional.id;
        let mut creature = entry("Creature", "creature");
        let creature_id = creature.id;
        require(&mut creature, dep_id);
        creature.fields.insert(
            "splat_optional".into(),
            serde_yaml_ng::to_value(vec![EntryRef::id(optional_id)]).unwrap(),
        );
        cat.insert(dep, owner, false).unwrap();
        cat.insert(optional, owner, false).unwrap();
        cat.insert(creature, owner, false).unwrap();
        cat.modules.push(ModuleIndex {
            id: owner,
            name: "Module".into(),
            entries: vec![creature_id, dep_id, optional_id],
        });
        let selection = vec![PackageSelection {
            package: EntryRef::id(owner),
            selection: Selection::Selected {
                entries: vec![EntryRef::id(creature_id)],
            },
        }];
        let closure = cat.closure(&selection, Uuid::new_v4());
        assert_eq!(closure.direct, BTreeSet::from([creature_id]));
        assert_eq!(
            closure.required[&dep_id],
            BTreeSet::from(["Creature".into()])
        );
        assert!(!closure.required.contains_key(&optional_id));
        assert!(closure.errors.is_empty());
        let later = entry("Later addition", "item");
        let later_id = later.id;
        cat.insert(later, owner, false).unwrap();
        cat.modules[0].entries.push(later_id);
        assert!(
            !cat.closure(&selection, Uuid::new_v4())
                .direct
                .contains(&later_id)
        );
        let all = cat.closure(
            &[PackageSelection {
                package: EntryRef::id(owner),
                selection: Selection::All,
            }],
            Uuid::new_v4(),
        );
        assert!(all.direct.contains(&optional_id));
        assert!(all.direct.contains(&later_id));
        let unavailable = cat.closure(
            &[PackageSelection {
                package: EntryRef::id(Uuid::new_v4()),
                selection: Selection::All,
            }],
            Uuid::new_v4(),
        );
        assert_eq!(unavailable.errors.len(), 1);
    }
    #[test]
    fn rule_groups_are_indivisible_and_cycles_are_errors() {
        let owner = Uuid::new_v4();
        let mut cat = Catalogue::default();
        let mut a = entry("Group A", "rule_group");
        let a_id = a.id;
        let mut b = entry("Group B", "rule_group");
        let b_id = b.id;
        require(&mut a, b_id);
        require(&mut b, a_id);
        cat.insert(a, owner, true).unwrap();
        cat.insert(b, owner, true).unwrap();
        let closure = cat.closure(&[], owner);
        assert!(closure.errors.iter().any(|e| e.contains("cycle")));
    }
    #[test]
    fn patch_preserves_unrelated_frontmatter_comments_and_body() {
        let text = "---\r\n# owner comment\r\nsplat_id: old\r\ntags: [one, two] # keep\r\nsplat_packages:\r\n  - package: {key: 'old:module'}\r\n    selection: {mode: all}\r\ncustom: |\r\n  Keep this text\r\n---\r\n# Body\r\nUnchanged.\r\n";
        let patched = patch_fields(
            text,
            &[
                ("splat_id", Some("new".into())),
                ("splat_packages", Some(Value::Sequence(vec![]))),
            ],
        )
        .unwrap();
        assert!(patched.contains("# owner comment\r\n"));
        assert!(patched.contains("tags: [one, two] # keep\r\n"));
        assert!(patched.contains("custom: |\r\n  Keep this text\r\n"));
        assert!(patched.ends_with("---\r\n# Body\r\nUnchanged.\r\n"));
        assert!(!patched.contains("old:module"));
    }
    #[test]
    fn invalid_campaign_note_can_be_repaired_without_losing_manifest() {
        let temp = tempfile::tempdir().unwrap();
        let campaign = campaign(temp.path().into());
        let mut doc = CampaignDocument::load(&campaign).unwrap();
        doc.save().unwrap();
        fs::write(temp.path().join("Broken.md"), "---\nsplat_id: [\n---\n").unwrap();
        let loaded = CampaignDocument::load(&campaign).unwrap();
        assert_eq!(loaded.selections, doc.selections);
        let bad = fs::read_to_string(temp.path().join("Broken.md")).unwrap();
        let fixed = note_text("Repaired", "item", None);
        write_note(temp.path(), "Broken.md", Some(&bad), &fixed).unwrap();
        assert!(content::read_directory(temp.path()).unwrap().1.is_empty());
    }
    #[test]
    fn invalid_notes_prevent_creation_of_a_second_manifest() {
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join("Campaign.md"), "---\nsplat_id: [\n---\n").unwrap();
        assert!(CampaignDocument::load(&campaign(temp.path().into())).is_err());
    }
    #[test]
    fn unrelated_parse_errors_do_not_block_independent_copy() {
        let temp = tempfile::tempdir().unwrap();
        let text = note_text("Item", "item", None);
        let source = CopySource {
            entry: Entry::parse("Item.md", &text).unwrap(),
            text,
            root: None,
            campaign: None,
        };
        let mut catalogue = Catalogue::default();
        catalogue.errors.push("Unrelated invalid note".into());
        assert!(CopyPlan::prepare(&source, temp.path(), None, "Copy", "", &catalogue).is_ok());
    }
    #[test]
    fn campaign_selection_save_preserves_notes_and_rejects_external_edits() {
        let temp = tempfile::tempdir().unwrap();
        let campaign = campaign(temp.path().into());
        let mut doc = CampaignDocument::load(&campaign).unwrap();
        assert!(doc.baseline.is_none());
        assert_eq!(doc.selections.len(), 1);
        doc.save().unwrap();
        let saved = fs::read_to_string(temp.path().join("Campaign.md")).unwrap();
        assert_eq!(Entry::parse("Campaign.md", &saved).unwrap().id, campaign.id);
        let mut doc = CampaignDocument::load(&campaign).unwrap();
        doc.selections.clear();
        fs::write(
            temp.path().join("Campaign.md"),
            format!("{saved}\nExternal note"),
        )
        .unwrap();
        assert!(doc.save().is_err());
        assert!(
            fs::read_to_string(temp.path().join("Campaign.md"))
                .unwrap()
                .ends_with("External note")
        );
    }
    #[test]
    fn independent_copy_keeps_original_and_blocks_private_transitive_dependencies() {
        let temp = tempfile::tempdir().unwrap();
        let dest = temp.path().join("module");
        fs::create_dir(&dest).unwrap();
        let private_owner = Uuid::new_v4();
        let module = Uuid::new_v4();
        let mut cat = Catalogue::default();
        let private = entry("Secret rule", "rule");
        let private_id = private.id;
        cat.insert(private, private_owner, true).unwrap();
        let mut shared = entry("Shared rule", "rule");
        let shared_id = shared.id;
        require(&mut shared, private_id);
        cat.insert(shared, module, false).unwrap();
        let mut original = entry("Original", "creature");
        require(&mut original, shared_id);
        let text = note_text("Original", "creature", None);
        let source = CopySource {
            entry: original.clone(),
            text,
            root: None,
            campaign: None,
        };
        let error = CopyPlan::prepare(&source, &dest, None, "Copy", "", &cat)
            .err()
            .unwrap();
        assert!(error.contains("Secret rule"));
        cat.entries
            .get_mut(&shared_id)
            .unwrap()
            .entry
            .fields
            .remove("splat_requires");
        let text = note_text("Original", "creature", None);
        let original = Entry::parse("Original.md", &text).unwrap();
        let source = CopySource {
            entry: original.clone(),
            text: text.clone(),
            root: None,
            campaign: None,
        };
        let plan = CopyPlan::prepare(&source, &dest, None, "Copy", "", &cat).unwrap();
        let copy = plan.save().unwrap();
        assert_ne!(copy.id, original.id);
        assert!(copy.key.is_none());
        assert_eq!(
            copy.fields["splat_origin"]["id"].as_str(),
            Some(original.id.to_string().as_str())
        );
        assert_eq!(source.text, text);
        assert!(
            content::validate(&[copy]).errors.is_empty(),
            "Copy provenance is optional"
        );
        assert!(plan.save().is_err());
    }
    #[test]
    fn builtin_catalogue_and_guard_copy_use_one_owner_and_preserve_attribution() {
        let cat = Catalogue::load(&[], None).unwrap();
        assert_eq!(cat.modules.len(), 1);
        let guard = cat
            .entries
            .values()
            .find(|e| e.entry.key.as_deref() == Some("dnd2024:creatures/guard"))
            .unwrap();
        let source = CopySource {
            entry: guard.entry.clone(),
            text: content::definitions::source_file(&guard.entry.path)
                .unwrap()
                .into(),
            root: None,
            campaign: None,
        };
        let temp = tempfile::tempdir().unwrap();
        let plan = CopyPlan::prepare(
            &source,
            temp.path(),
            Some(Uuid::new_v4()),
            "Gate Guard",
            "",
            &cat,
        )
        .unwrap();
        let copy = plan.save().unwrap();
        assert_ne!(copy.id, guard.entry.id);
        assert!(plan.text.contains(content::builtin::ATTRIBUTION));
        let all = cat.closure(
            &[PackageSelection {
                package: EntryRef::id(builtin_manifest().id),
                selection: Selection::All,
            }],
            Uuid::new_v4(),
        );
        assert!(all.errors.is_empty(), "{:?}", all.errors);
    }
    #[test]
    fn private_character_pulls_shared_rules_without_explicit_package_selection() {
        let mut cat = Catalogue::load(&[], None).unwrap();
        let campaign = Uuid::new_v4();
        let spell = cat
            .resolve(&Reference::Key("dnd2024:spells/fire_bolt".into()))
            .unwrap();
        let mut character = entry("Imported character", "creature");
        require(&mut character, spell);
        cat.insert(character, campaign, true).unwrap();
        let closure = cat.closure(&[], campaign);
        assert!(closure.errors.is_empty(), "{:?}", closure.errors);
        assert!(closure.required.contains_key(&spell));
        assert!(!cat.entries[&spell].private);
        assert_eq!(cat.entries[&spell].owner, builtin_manifest().id);
    }
    fn write_package(folder: &Path, namespace: &str, name: &str, requires: &str) -> Uuid {
        let id = Uuid::new_v4();
        fs::create_dir(folder).unwrap();
        fs::write(
            folder.join("Package.md"),
            format!(
                "---\nsplat_schema: 1\nsplat_id: \"{id}\"\nsplat_kind: \"package\"\nsplat_key: \"{namespace}:package\"\nsplat_name: \"{name}\"\nsplat_namespace: \"{namespace}\"\nsplat_requires:\n  - key: \"{requires}\"\n---\n"
            ),
        )
        .unwrap();
        id
    }
    fn write_creature(folder: &Path, name: &str, requires: &str) {
        let text = patch_fields(
            &note_text(name, "creature", None),
            &[(
                "splat_requires",
                Some(
                    serde_yaml_ng::to_value(vec![EntryRef {
                        id: None,
                        key: Some(requires.into()),
                        link: None,
                    }])
                    .unwrap(),
                ),
            )],
        )
        .unwrap();
        fs::write(folder.join(format!("{name}.md")), text).unwrap();
    }
    #[test]
    fn each_missing_package_and_entry_is_one_issue() {
        let temp = tempfile::tempdir().unwrap();
        let book = temp.path().join("Monster Manual");
        let id = write_package(&book, "book", "Monster Manual", "phb:package");
        write_creature(&book, "Arch-hag", "phb:spells/plane_shift");
        write_creature(&book, "Blue Slaad", "phb:spells/wish");
        write_creature(&book, "Cambion", "phb:spells/plane_shift");
        for name in ["Devil", "Elf", "Fiend", "Ghoul"] {
            write_creature(&book, name, "book:spells/lost");
        }
        let neighbour = temp.path().join("Players Handbook");
        write_package(
            &neighbour,
            "phb",
            "Player's Handbook (2024)",
            "dnd2024:package/srd",
        );
        let modules = [Module {
            id,
            name: "Monster Manual".into(),
            path: book,
        }];
        let issues = || {
            Catalogue::load(&modules, None)
                .unwrap()
                .closure(
                    &[PackageSelection {
                        package: EntryRef::id(id),
                        selection: Selection::All,
                    }],
                    Uuid::new_v4(),
                )
                .errors
        };
        let lost = "Entry not found: book:spells/lost · needed by Devil, Elf, Fiend and 1 more";
        assert_eq!(
            issues(),
            ["Package not found: Player's Handbook (2024)", lost]
        );
        // With nothing beside the package to name it, the namespace still identifies it.
        fs::remove_dir_all(neighbour).unwrap();
        assert_eq!(issues(), ["Package not found: phb", lost]);
    }
}
