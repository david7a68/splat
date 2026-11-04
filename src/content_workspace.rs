//! Content ownership, campaign selections, and independent copies.
use crate::{
    library::{note_path, note_text, write_note},
    persistence::{Campaign, Module},
};
use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;
use splat::character::campaign_package;
pub use splat::content::patch_fields;
use splat::content::{self, Entry, Reference};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

/// Recover an identity even when another front-matter line cannot be parsed.
pub(crate) fn note_id(text: &str) -> Option<Uuid> {
    let mut lines = text.trim_start_matches('\u{feff}').lines();
    if lines.next()?.trim() != "---" {
        return None;
    }
    lines
        .take_while(|line| line.trim() != "---")
        .find_map(|line| {
            let value = line.strip_prefix("splat_id:")?;
            let value: String = serde_yaml_ng::from_str(value).ok()?;
            Uuid::parse_str(&value).ok().filter(|id| !id.is_nil())
        })
}

pub(crate) const DELETED_MANIFEST: &str = "Package.md was deleted. Entries are still available. Write it again, or restore it from a backup.";

pub(crate) fn package_manifest_path(module: &Module) -> String {
    if let Ok((entries, errors)) = content::read_directory(&module.path) {
        if let Some(entry) = entries.iter().find(|entry| entry.kind == "package") {
            return entry.path.clone();
        }
        for error in errors {
            if matches!(error.path.as_str(), "Package.md" | "Module.md")
                || fs::read_to_string(module.path.join(&error.path)).is_ok_and(|text| {
                    text.lines()
                        .any(|line| line.trim() == "splat_kind: package")
                })
            {
                return error.path;
            }
        }
    }
    "Package.md".into()
}

/// A copied note gets its file's name in the same write as its new identity.
pub(crate) fn new_note_identity(text: &str, path: &str, id: Uuid) -> Result<String, String> {
    let entry = Entry::parse(path, text)?;
    let name = Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid file name.")?;
    let redirected = redirect_reference(text, &entry, id)?;
    let after = patch_fields(
        &redirected,
        &[
            ("splat_id", Some(id.to_string().into())),
            ("splat_key", None),
            ("splat_name", Some(name.into())),
        ],
    )?;
    Ok(rename_first_heading(after, &entry.name, name))
}

pub(crate) fn rename_first_heading(mut after: String, old: &str, name: &str) -> String {
    let start = after.find("\n---").map_or(0, |at| at + 4);
    let mut offset = start;
    for line in after[start..].split_inclusive('\n') {
        if let Some(heading) = line.trim_end().strip_prefix("# ") {
            if heading.trim() == old {
                after.replace_range(offset..offset + line.trim_end().len(), &format!("# {name}"));
            }
            break;
        }
        offset += line.len();
    }
    after
}

/// All writes for a copied package, with one guarded Undo.
pub(crate) struct PackageCopy {
    pub module: Module,
    files: Vec<(String, String, String)>,
}
impl PackageCopy {
    pub fn write(root: &Path, name: &str) -> Result<Self, String> {
        Self::write_owner(root, name, "package")
    }

    pub fn write_owner(root: &Path, name: &str, kind: &str) -> Result<Self, String> {
        if name.trim().is_empty() || name.contains(['\n', '\r']) {
            return Err("Type a package name first.".into());
        }
        let root = root.canonicalize().map_err(|e| e.to_string())?;
        let (entries, errors) = content::read_directory(&root)?;
        if !errors.is_empty() || entries.iter().filter(|e| e.kind == kind).count() != 1 {
            return Err("Repair the copied files before registering this package.".into());
        }
        let ids: BTreeMap<_, _> = entries
            .iter()
            .map(|e| (e.id, crate::hermetic::new_uuid()))
            .collect();
        if ids.len() != entries.len() {
            return Err("Repair duplicate identities inside this folder first.".into());
        }
        let manifest = entries.iter().find(|e| e.kind == kind).unwrap();
        let id = ids[&manifest.id];
        let namespace = format!("package_{}", id.simple());
        let mut files = Vec::new();
        for entry in &entries {
            let before =
                fs::read_to_string(note_path(&root, &entry.path)?).map_err(|e| e.to_string())?;
            let mut after = before.clone();
            for old in &entries {
                after = redirect_reference(&after, old, ids[&old.id])?;
            }
            let mut fields = vec![
                ("splat_id", Some(ids[&entry.id].to_string().into())),
                (
                    "splat_key",
                    Some(format!("{namespace}:{}", ids[&entry.id].simple()).into()),
                ),
            ];
            if entry.kind == kind {
                fields.push(("splat_name", Some(name.trim().into())));
                fields.push(("splat_namespace", Some(namespace.clone().into())));
                if kind == "campaign" {
                    fields.push(("splat_dndbeyond_campaign_id", Some(Value::Null)));
                    fields.push(("splat_dndbeyond_campaign_name", Some(Value::Null)));
                }
            }
            after = patch_fields(&after, &fields)?;
            if entry.kind == kind {
                after = rename_first_heading(after, &entry.name, name.trim());
            }
            files.push((entry.path.clone(), before, after));
        }
        let copy = Self {
            module: Module {
                id,
                name: name.trim().into(),
                path: root,
            },
            files,
        };
        copy.apply(false)?;
        Ok(copy)
    }

    fn apply(&self, undo: bool) -> Result<(), String> {
        for (path, before, after) in &self.files {
            let expected = if undo { after } else { before };
            if fs::read_to_string(note_path(&self.module.path, path)?).map_err(|e| e.to_string())?
                != *expected
            {
                return Err(format!(
                    "{path} changed on disk. Reload before trying again."
                ));
            }
        }
        let changes = self
            .files
            .iter()
            .map(|(path, before, after)| {
                let (from, to) = if undo {
                    (after, before)
                } else {
                    (before, after)
                };
                Ok(crate::file_batch::Change::text(
                    note_path(&self.module.path, path)?,
                    Some(from),
                    Some(to),
                ))
            })
            .collect::<Result<Vec<_>, String>>()?;
        crate::file_batch::apply(&changes)
    }

    pub fn undo(&self) -> Result<(), String> {
        self.apply(true)
    }

    pub fn undo_with(
        &self,
        mut extra: Vec<(PathBuf, String, String, String)>,
    ) -> Result<ContentEdits, String> {
        extra.extend(self.files.iter().map(|(path, before, after)| {
            (
                self.module.path.clone(),
                path.clone(),
                after.clone(),
                before.clone(),
            )
        }));
        ContentEdits::write(extra)
    }

    pub fn use_in_rosters(&self, campaign: &Campaign) -> Result<ContentEdits, String> {
        let replacements = self
            .files
            .iter()
            .map(|(path, before, after)| {
                Ok((Entry::parse(path, before)?, Entry::parse(path, after)?.id))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let (entries, errors) = content::read_directory(&campaign.path)?;
        if !errors.is_empty() {
            return Err("Repair unreadable campaign files before changing their sources.".into());
        }
        let mut files = Vec::new();
        for entry in entries
            .iter()
            .filter(|entry| matches!(entry.kind.as_str(), "encounter" | "encounter_group"))
        {
            let before = fs::read_to_string(note_path(&campaign.path, &entry.path)?)
                .map_err(|e| e.to_string())?;
            let mut after = before.clone();
            for (old, id) in &replacements {
                after = redirect_reference(&after, old, *id)?;
            }
            if before != after {
                files.push((campaign.path.clone(), entry.path.clone(), before, after));
            }
        }
        ContentEdits::write(files)
    }
}

/// A repair keeps the registered identity and never overwrites a later disk edit.
pub(crate) struct ContentEdits {
    files: Vec<(PathBuf, String, String, String)>,
}
impl ContentEdits {
    pub fn inverse_files(&self) -> Vec<(PathBuf, String, String, String)> {
        self.files
            .iter()
            .map(|(root, path, before, after)| {
                (root.clone(), path.clone(), after.clone(), before.clone())
            })
            .collect()
    }
    pub fn new_identities(root: &Path, paths: &[String]) -> Result<Self, String> {
        let entries = paths
            .iter()
            .map(|path| {
                let before =
                    fs::read_to_string(note_path(root, path)?).map_err(|e| e.to_string())?;
                let entry = Entry::parse(path, &before)?;
                Ok((path.clone(), before, entry, crate::hermetic::new_uuid()))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut files = Vec::new();
        for (path, before, _, id) in &entries {
            let mut after = new_note_identity(before, path, *id)?;
            for (_, _, old, id) in &entries {
                after = redirect_reference(&after, old, *id)?;
            }
            files.push((root.to_path_buf(), path.clone(), before.clone(), after));
        }
        Self::write(files)
    }
    pub fn write(files: Vec<(PathBuf, String, String, String)>) -> Result<Self, String> {
        let edits = Self { files };
        edits.apply(false)?;
        Ok(edits)
    }
    fn apply(&self, undo: bool) -> Result<(), String> {
        for (root, path, before, after) in &self.files {
            let expected = if undo { after } else { before };
            if fs::read_to_string(note_path(root, path)?).map_err(|e| e.to_string())? != *expected {
                return Err(format!(
                    "{path} changed on disk. Reload before trying again."
                ));
            }
        }
        for (written, (root, path, before, after)) in self.files.iter().enumerate() {
            let (from, to) = if undo {
                (after, before)
            } else {
                (before, after)
            };
            if let Err(error) = write_note(root, path, Some(from), to) {
                let mut errors = vec![error];
                for (root, path, before, after) in self.files[..written].iter().rev() {
                    let (from, to) = if undo {
                        (before, after)
                    } else {
                        (after, before)
                    };
                    if let Err(error) = write_note(root, path, Some(from), to) {
                        errors.push(format!("Could not restore {path}: {error}"));
                    }
                }
                return Err(errors.join("\n"));
            }
        }
        Ok(())
    }
    pub fn undo(&self) -> Result<(), String> {
        self.apply(true)
    }
}

/// A repair keeps the registered identity and never overwrites a later disk edit.
pub(crate) struct ManifestWrite {
    root: PathBuf,
    path: String,
    before: Option<String>,
    after: String,
}
impl ManifestWrite {
    pub fn restore_campaign(campaign: &Campaign, path: &str, text: &str) -> Result<Self, String> {
        write_note(&campaign.path, path, None, text)?;
        Ok(Self {
            root: campaign.path.clone(),
            path: path.into(),
            before: None,
            after: text.into(),
        })
    }

    pub fn write(module: &Module) -> Result<Self, String> {
        let catalogue = Catalogue::load(std::slice::from_ref(module), None)?;
        if !catalogue.manifest_problems.contains_key(&module.id) {
            return Err("Package.md is valid. No repair is needed.".into());
        }
        let manifest = &catalogue
            .entries
            .get(&module.id)
            .ok_or("Package details are unavailable.")?
            .entry;
        let namespace = manifest
            .fields
            .get("splat_namespace")
            .and_then(Value::as_str)
            .ok_or("Package namespace is unavailable.")?;
        let after = patch_fields(
            &note_text(&module.name, "package", Some(namespace)),
            &[("splat_id", Some(module.id.to_string().into()))],
        )?;
        let relative = package_manifest_path(module);
        let path = module.path.join(&relative);
        let before = match fs::read_to_string(path) {
            Ok(text) => Some(text),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => return Err(error.to_string()),
        };
        write_note(&module.path, &relative, before.as_deref(), &after)?;
        Ok(Self {
            root: module.path.clone(),
            path: relative,
            before,
            after,
        })
    }
    pub fn replaced(&self) -> bool {
        self.before.is_some()
    }
    pub fn undo(&self) -> Result<PathBuf, String> {
        // A broken manifest cannot pass write_note's parse check.
        let path = note_path(&self.root, &self.path)?;
        if fs::read_to_string(&path).map_err(|e| e.to_string())? != self.after {
            return Err(format!(
                "{} changed on disk. Reload before undoing.",
                self.path
            ));
        }
        let Some(before) = self.before.as_deref() else {
            fs::remove_file(path).map_err(|e| e.to_string())?;
            return Ok(self.root.clone());
        };
        use std::io::Write;
        let mut temp = tempfile::NamedTempFile::new_in(&self.root).map_err(|e| e.to_string())?;
        temp.write_all(before.as_bytes())
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|e| e.to_string())?;
        temp.persist(path).map_err(|e| e.to_string())?;
        Ok(self.root.clone())
    }
}

/// Add new work to the active campaign without replacing its other choices.
/// `None` selects the whole package, including future entries.
pub fn use_in_campaign(
    campaign: &Campaign,
    package: Uuid,
    package_name: &str,
    entry: Option<Uuid>,
) -> Result<(), String> {
    let mut document = CampaignDocument::load(campaign)?;
    if let Some(selection) = document
        .selections
        .iter_mut()
        .find(|s| s.package.id == Some(package))
    {
        selection.name = Some(package_name.into());
        match (&mut selection.selection, entry) {
            (Selection::All, _) => {}
            (Selection::Selected { entries }, Some(id)) => {
                if !entries.iter().any(|e| e.id == Some(id)) {
                    entries.push(EntryRef::id(id));
                }
            }
            (selection, None) => *selection = Selection::All,
        }
    } else {
        document.selections.push(PackageSelection {
            name: Some(package_name.into()),
            package: EntryRef::id(package),
            selection: entry.map_or(Selection::All, |id| Selection::Selected {
                entries: vec![EntryRef::id(id)],
            }),
        });
    }
    document.save()
}

pub fn builtin_manifest() -> Entry {
    let bytes = content::builtin::source_file("Package.md").expect("bundled manifest");
    Entry::parse(
        "Package.md",
        std::str::from_utf8(bytes).expect("UTF-8 manifest"),
    )
    .expect("valid manifest")
}

/// Where an import can route definitions for the campaign in this folder. A campaign that cannot
/// be read has no enabled packages, so its definitions go to the campaign package, and the check
/// offers their move once it can read the campaign again.
pub fn destinations(root: &Path, modules: &[Module]) -> campaign_package::Destinations {
    let document = CampaignDocument::discover(root)
        .ok()
        .flatten()
        .and_then(|campaign| CampaignDocument::load(&campaign).ok());
    let Some(document) = document else {
        return Default::default();
    };
    // A partial selection also enables what its entries require. A sheet's own references do not
    // count, or a note the DM deselected would stay selected by the sheet that uses it.
    let partial = document
        .selections
        .iter()
        .any(|selection| matches!(selection.selection, Selection::Selected { .. }));
    let enabled: Option<Vec<Reference>> = partial
        .then(|| Catalogue::load(modules, None).ok())
        .flatten()
        .map(|catalogue| {
            let closure = catalogue.closure(&document.selections, document.campaign.id);
            closure
                .direct
                .iter()
                .chain(closure.required.keys())
                .map(|id| Reference::Id(*id))
                .collect()
        });
    let packages = document
        .selections
        .iter()
        .filter_map(|selection| {
            let module = modules
                .iter()
                .find(|m| Some(m.id) == selection.package.id)?;
            let only = match &selection.selection {
                Selection::All => None,
                Selection::Selected { entries } => Some(enabled.clone().unwrap_or_else(|| {
                    entries
                        .iter()
                        .filter_map(|entry| entry.reference().ok())
                        .collect()
                })),
            };
            Some(campaign_package::EnabledPackage {
                id: module.id,
                name: module.name.clone(),
                path: module.path.clone(),
                only,
            })
        })
        .collect();
    campaign_package::Destinations::new(document.campaign.id, packages)
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
    /// Last known display name, retained if the package is removed or moved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub package: EntryRef,
    pub selection: Selection,
}

impl PackageSelection {
    pub fn display_name(&self) -> &str {
        self.name
            .as_deref()
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .unwrap_or("A package that is no longer on this computer")
    }

    pub fn campaign_users(&self, catalogue: &Catalogue) -> Vec<(Uuid, String)> {
        let references: Vec<_> = match &self.selection {
            Selection::Selected { entries } => entries
                .iter()
                .filter_map(|entry| entry.reference().ok())
                .collect(),
            Selection::All => catalogue
                .modules
                .iter()
                .find(|module| Some(module.id) == self.package.id)
                .map(|module| module.entries.iter().map(|id| Reference::Id(*id)).collect())
                .unwrap_or_default(),
        };
        catalogue
            .entries
            .values()
            .filter(|entry| entry.private)
            .filter(|entry| {
                content::required_references(&entry.entry).is_ok_and(|required| {
                    required.iter().any(|reference| {
                        references.contains(reference)
                            || catalogue.resolve(reference).is_ok_and(|id| {
                                references.iter().any(|r| catalogue.resolve(r) == Ok(id))
                            })
                    })
                })
            })
            .map(|entry| (entry.entry.id, entry.entry.name.clone()))
            .collect()
    }

    pub fn unavailable_message(&self) -> String {
        if self
            .name
            .as_deref()
            .is_some_and(|name| !name.trim().is_empty())
        {
            format!("Splat cannot find {}.", self.display_name())
        } else {
            format!("{}.", self.display_name())
        }
    }
}

pub struct ModuleIndex {
    pub id: Uuid,
    pub name: String,
    pub entries: Vec<Uuid>,
    /// The campaign package. It serves one campaign, so nothing can select it.
    pub private: bool,
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
    duplicate_keys: BTreeSet<String>,
    pub errors: Vec<String>,
    pub manifest_problems: BTreeMap<Uuid, String>,
    pub unreadable: BTreeMap<Uuid, UnreadableNote>,
    pub duplicate_world: Vec<IndexedEntry>,
    duplicate_ids: BTreeSet<Uuid>,
    files: BTreeMap<Uuid, (PathBuf, String)>,
    namespaces: BTreeSet<String>,
    neighbours: BTreeMap<String, String>,
}

#[derive(Clone)]
pub struct UnreadableNote {
    pub path: PathBuf,
    pub word: String,
}

/// A file that differs from its shared key's filename is the copy. Otherwise
/// prefer the newer file. Atomic saves can replace a file's birth time.
pub(crate) fn duplicate_repair_path(paths: impl IntoIterator<Item = PathBuf>) -> Option<PathBuf> {
    paths.into_iter().max_by_key(|path| {
        let copied = fs::read_to_string(path)
            .ok()
            .and_then(|text| Entry::parse("note.md", &text).ok())
            .and_then(|entry| entry.key)
            .is_some_and(|key| {
                let key = key.rsplit(['/', ':']).next().unwrap_or(&key);
                let stem = path.file_stem().unwrap_or_default().to_string_lossy();
                let slug: String = stem
                    .to_lowercase()
                    .chars()
                    .map(|c| if c.is_alphanumeric() { c } else { '_' })
                    .collect();
                slug != key
            });
        let created = fs::metadata(path)
            .ok()
            .and_then(|meta| meta.created().or_else(|_| meta.modified()).ok());
        (copied, created, path.as_os_str().len())
    })
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
    fn insert_file(
        &mut self,
        entry: Entry,
        owner: Uuid,
        private: bool,
        root: &Path,
    ) -> Result<(), String> {
        let path = note_path(root, &entry.path)?;
        if self.entries.contains_key(&entry.id) || self.duplicate_ids.contains(&entry.id) {
            // A manifest-only hand repair must not remove the registered original.
            // Library loads both files and offers a named repair for each one.
            if let Some(original) = self.entries.get(&entry.id)
                && original.owner != owner
                && let Some((first, _)) = self.files.get(&entry.id)
            {
                self.errors.push(format!(
                    "{} and {} share an identity. Choose one in Library.",
                    first.display(),
                    path.display()
                ));
                return Ok(());
            }
            let first = self
                .files
                .get(&entry.id)
                .map(|(path, _)| path)
                .or_else(|| self.unreadable.get(&entry.id).map(|note| &note.path));
            let mut names: Vec<_> = first
                .into_iter()
                .chain([&path])
                .filter_map(|path| path.file_name())
                .map(|name| name.to_string_lossy().into_owned())
                .collect();
            names.sort();
            let path = duplicate_repair_path(first.cloned().into_iter().chain([path])).unwrap();
            if private && entry.kind == "world" {
                if let Some(original) = self.entries.remove(&entry.id) {
                    self.duplicate_world.push(original);
                }
                self.duplicate_world.push(IndexedEntry {
                    entry: entry.clone(),
                    owner,
                    private,
                });
            }
            self.duplicate_ids.insert(entry.id);
            self.entries.remove(&entry.id);
            self.files.remove(&entry.id);
            self.keys.retain(|_, id| *id != entry.id);
            let word = format!(
                "{} share an identity. Choose one in {}.",
                names.join(" and "),
                if private && entry.kind == "world" {
                    "World"
                } else {
                    "Library"
                }
            );
            self.errors.push(word.clone());
            self.unreadable
                .insert(entry.id, UnreadableNote { path, word });
            return Ok(());
        }
        self.files.insert(
            entry.id,
            (
                path.clone(),
                fs::read_to_string(&path).map_err(|e| e.to_string())?,
            ),
        );
        self.insert(entry, owner, private)
    }

    fn remember_unreadable(&mut self, root: &Path, errors: &[content::Diagnostic]) {
        for error in errors {
            let Ok(path) = note_path(root, &error.path) else {
                continue;
            };
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            let Some(id) = note_id(&text) else {
                continue;
            };
            let (_, line) = crate::campaign_content::repair_problem(&error.path, &text);
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            let line = line.map_or(String::new(), |line| format!(" (line {line})"));
            self.unreadable.insert(
                id,
                UnreadableNote {
                    word: format!("{name} cannot be read{line}"),
                    path,
                },
            );
        }
    }
    fn insert(&mut self, entry: Entry, owner: Uuid, private: bool) -> Result<(), String> {
        if self.entries.contains_key(&entry.id) {
            return Err(format!("Duplicate identity: {}", entry.name));
        }
        if let Some(key) = &entry.key {
            if self.keys.contains_key(key) || self.duplicate_keys.contains(key) {
                let mut paths: Vec<_> = self
                    .entries
                    .values()
                    .filter(|other| other.entry.key.as_ref() == Some(key))
                    .map(|other| {
                        self.files.get(&other.entry.id).map_or_else(
                            || other.entry.path.clone(),
                            |(path, _)| path.display().to_string(),
                        )
                    })
                    .collect();
                paths.push(self.files.get(&entry.id).map_or_else(
                    || entry.path.clone(),
                    |(path, _)| path.display().to_string(),
                ));
                self.errors.push(format!(
                    "{} share the key {key}. Choose a new identity in Library.",
                    paths.join(" and ")
                ));
                self.keys.remove(key);
                self.duplicate_keys.insert(key.clone());
            } else {
                self.keys.insert(key.clone(), entry.id);
            }
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
    /// The name of the package with this ID.
    pub fn package_name(&self, id: Uuid) -> String {
        self.modules
            .iter()
            .find(|module| module.id == id)
            .map_or_else(|| "Package".to_owned(), |module| module.name.clone())
    }
    pub fn resolve(&self, reference: &Reference) -> Result<Uuid, String> {
        let id = match reference {
            Reference::Id(id) => Some(*id),
            Reference::Key(key) => self.keys.get(key).copied(),
        };
        id.filter(|id| self.entries.contains_key(id))
            .ok_or_else(|| format!("Unavailable content: {reference}"))
    }
    /// An entry's file as it was read, to copy from. A built-in entry has no file on disk.
    pub fn text_of(&self, id: Uuid) -> Option<String> {
        self.files
            .get(&id)
            .map(|(_, text)| text.clone())
            .or_else(|| {
                content::definitions::source_file(&self.entries.get(&id)?.entry.path)
                    .map(str::to_owned)
            })
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
            private: false,
        });
        for module in modules {
            let read = content::read_directory(&module.path);
            let (mut entries, errors) = match read {
                Ok(v) => v,
                Err(e) => {
                    result.errors.push(format!("{}: {e}", module.name));
                    continue;
                }
            };
            let manifest_path = package_manifest_path(module);
            if !entries
                .iter()
                .any(|entry| entry.kind == "package" && entry.id == module.id)
                && module.path.is_dir()
            {
                let problem = if let Ok(text) = fs::read_to_string(module.path.join(&manifest_path))
                {
                    let (_, line) = crate::campaign_content::repair_problem(&manifest_path, &text);
                    format!(
                        "{manifest_path} {}{}",
                        if Entry::parse(&manifest_path, &text).is_ok() {
                            "has another identity"
                        } else {
                            "cannot be read"
                        },
                        line.map_or(String::new(), |line| format!(" (line {line})"))
                    )
                } else {
                    DELETED_MANIFEST.into()
                };
                result.manifest_problems.insert(module.id, problem.clone());
                result.errors.push(format!(
                    "{}: {problem}{}",
                    module.name,
                    if problem == DELETED_MANIFEST {
                        ""
                    } else {
                        " Entries are still available."
                    }
                ));
                // The registration keeps ownership while the on-disk manifest is repaired.
                // The repair action can write these registered details again.
                let namespace = entries
                    .iter()
                    .filter_map(|entry| entry.key.as_deref())
                    .find_map(|key| {
                        key.split_once(':')
                            .map(|(namespace, _)| namespace.to_owned())
                    })
                    .unwrap_or_else(|| format!("recovery_{}", module.id.simple()));
                let mut manifest = builtin_manifest();
                entries.retain(|entry| entry.kind != "package");
                manifest.path = manifest_path;
                manifest.id = module.id;
                manifest.key = None;
                manifest.name = module.name.clone();
                manifest
                    .fields
                    .insert("splat_namespace".into(), namespace.into());
                manifest.body.clear();
                entries.push(manifest);
            }
            result.remember_unreadable(&module.path, &errors);
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
                result.errors.push(format!(
                    "Duplicate package namespace: {ns}. Choose a new identity in Library."
                ));
            }
            let ids = entries.iter().map(|e| e.id).collect();
            let name = manifests[0].name.clone();
            for entry in entries {
                if entry.kind == "package" && result.manifest_problems.contains_key(&module.id) {
                    result.insert(entry, module.id, false)?;
                } else {
                    result.insert_file(entry, module.id, false, &module.path)?;
                }
            }
            result.modules.push(ModuleIndex {
                id: module.id,
                name,
                entries: ids,
                private: false,
            });
        }
        if let Some(campaign) = campaign {
            let (entries, errors) = content::read_directory(&campaign.path)?;
            result.remember_unreadable(&campaign.path, &errors);
            result.errors.extend(
                errors
                    .into_iter()
                    .map(|e| format!("{}: {}", e.path, e.message)),
            );
            let manifest = entries
                .iter()
                .find(|entry| campaign_package::is_manifest(entry))
                .map(|entry| (entry.id, entry.name.clone()));
            let mut package_entries = Vec::new();
            for entry in entries {
                if entry.kind == "package" && !campaign_package::is_manifest(&entry) {
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
                // The campaign package is private to this campaign, like its other notes.
                let package = manifest
                    .as_ref()
                    .filter(|_| campaign_package::in_folder(&entry.path))
                    .map(|(id, _)| *id);
                package_entries.extend(package.map(|_| entry.id));
                result.insert_file(entry, package.unwrap_or(campaign.id), true, &campaign.path)?;
            }
            if let Some((id, name)) = manifest {
                result.modules.push(ModuleIndex {
                    id,
                    name,
                    entries: package_entries,
                    private: true,
                });
            }
        }
        let mut resolved = Vec::new();
        for (id, indexed) in &result.entries {
            if let Some(values) = indexed
                .entry
                .fields
                .get("splat_requires")
                .and_then(Value::as_sequence)
            {
                for (at, value) in values.iter().enumerate() {
                    if let Some(raw) = value.as_str() {
                        let reference = content::formats::resolve_readable(
                            raw,
                            result.entries.values().map(|item| &item.entry),
                        );
                        resolved.push((*id, at, reference));
                    }
                }
            }
        }
        for (id, at, reference) in resolved {
            let indexed = result.entries.get_mut(&id).unwrap();
            match reference {
                Ok(id) => {
                    indexed
                        .entry
                        .fields
                        .get_mut("splat_requires")
                        .and_then(Value::as_sequence_mut)
                        .unwrap()[at] =
                        serde_yaml_ng::to_value(EntryRef::id(id)).map_err(|e| e.to_string())?
                }
                Err(message) => {
                    let line = result
                        .files
                        .get(&id)
                        .and_then(|(_, text)| {
                            text.lines()
                                .position(|line| line.contains("splat_requires:"))
                        })
                        .map_or(1, |at| at + 1);
                    indexed
                        .entry
                        .flags
                        .push(content::layout::Flag { line, message });
                }
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
                        selection.display_name()
                    ));
                    continue;
                }
            };
            match &selection.selection {
                Selection::All => {
                    for &id in &module.entries {
                        if self.entries.contains_key(&id) {
                            result.direct.insert(id);
                        } else if let Some(note) = self.unreadable.get(&id)
                            && !result.errors.contains(&note.word)
                        {
                            result.errors.push(note.word.clone());
                        }
                    }
                }
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

fn campaign_manifest(root: &Path, expected: Option<Uuid>) -> Result<Option<Entry>, String> {
    let (entries, errors) = content::read_directory(root)?;
    if !errors.is_empty() && !entries.iter().any(|e| e.kind == "campaign") {
        return Err(errors
            .iter()
            .map(|error| format!("{}: {}", error.path, error.message))
            .collect::<Vec<_>>()
            .join("\n"));
    }
    if entries
        .iter()
        .any(|e| e.kind == "package" && !campaign_package::is_manifest(e))
    {
        return Err("A campaign folder cannot contain a package manifest.".into());
    }
    let mut manifests: Vec<_> = entries
        .into_iter()
        .filter(|e| e.kind == "campaign")
        .collect();
    if manifests.len() > 1 {
        let matching: Vec<_> = manifests
            .iter()
            .filter(|entry| Some(entry.id) == expected)
            .collect();
        if matching.len() == 1 {
            return Ok(Some(matching[0].clone()));
        }
        return Err(format!(
            "A campaign needs exactly one manifest: {}.",
            manifests
                .iter()
                .map(|entry| entry.path.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    Ok(manifests.pop())
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
        campaign_manifest(&root, None)?
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
        Ok(())
    }

    #[cfg(test)]
    pub fn save_metadata(campaign: &Campaign, previous: Option<&Campaign>) -> Result<(), String> {
        crate::file_batch::apply(&[Self::metadata_change(campaign, previous)?])
    }

    pub(crate) fn metadata_change(
        campaign: &Campaign,
        previous: Option<&Campaign>,
    ) -> Result<crate::file_batch::Change, String> {
        let document = if previous.is_none()
            && campaign_manifest(&campaign.path, Some(campaign.id))?.is_none()
        {
            Self::new(campaign)?
        } else {
            Self::load(campaign)?
        };
        let entry = Entry::parse(&document.path, &document.text)?;
        if document.baseline.is_some()
            && entry.fields.contains_key("splat_dndbeyond_campaign_id")
            && entry.fields.contains_key("splat_dndbeyond_campaign_name")
            && campaign_metadata(&entry, campaign.clone())? == *campaign
        {
            return Ok(crate::file_batch::Change::text(
                note_path(&campaign.path, &document.path)?,
                document.baseline.as_deref(),
                Some(&document.text),
            ));
        }
        if let Some(previous) = previous
            && entry.fields.contains_key("splat_dndbeyond_campaign_id")
            && campaign_metadata(&entry, previous.clone())? != *previous
        {
            return Err("Campaign settings changed on disk. Reload before saving.".into());
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
        Ok(crate::file_batch::Change::text(
            note_path(&campaign.path, &document.path)?,
            document.baseline.as_deref(),
            Some(&text),
        ))
    }

    /// Only a new campaign or an explicit repair may start with defaults.
    pub fn new(campaign: &Campaign) -> Result<Self, String> {
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
                            name: Some(builtin_manifest().name),
                            package: EntryRef::id(builtin_manifest().id),
                            selection: Selection::All,
                        }])
                        .map_err(|e| e.to_string())?,
                    ),
                ),
            ],
        )?;
        let selections = serde_yaml_ng::from_value(
            Entry::parse("Campaign.md", &text)?.fields["splat_packages"].clone(),
        )
        .map_err(|e| e.to_string())?;
        Ok(Self {
            campaign: campaign.clone(),
            path: "Campaign.md".into(),
            baseline: None,
            text,
            selections,
        })
    }

    pub fn load(campaign: &Campaign) -> Result<Self, String> {
        let (path, baseline, text) = if let Some(manifest) =
            campaign_manifest(&campaign.path, Some(campaign.id))?
        {
            if manifest.id != campaign.id {
                return Err(format!(
                    "Campaign “{}” does not match its folder. Settings has ID {}, but {} in {} has ID {}. Correct the ID or folder in settings.json, then reload.",
                    campaign.name,
                    campaign.id,
                    manifest.path,
                    campaign.path.display(),
                    manifest.id,
                ));
            }
            let text = fs::read_to_string(note_path(&campaign.path, &manifest.path)?)
                .map_err(|e| e.to_string())?;
            (manifest.path.clone(), Some(text.clone()), text)
        } else {
            return Err(
                "Campaign.md is missing. Write Campaign.md again, or restore it from a backup."
                    .into(),
            );
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
        .filter(|e| !campaign_package::is_manifest(e))
        .collect();
    if private && manifests.is_empty() {
        return Ok(());
    }
    if manifests.len() != 1
        || manifests[0].id != id
        || manifests[0].kind != if private { "campaign" } else { "package" }
    {
        return Err(
            "The destination owner changed. Reload its registration before copying.".into(),
        );
    }
    Ok(())
}

/// A note's text, with each line that refers to the note itself now referring to another id.
fn own_references_to(text: &str, entry: &Entry, new_id: Uuid) -> String {
    let own_id = entry.id.to_string();
    let quoted = |value: &str| {
        [
            value.to_owned(),
            format!("\"{value}\""),
            format!("'{value}'"),
        ]
    };
    text.split_inclusive('\n')
        .map(|line| {
            let trimmed = line.trim_end();
            let body = trimmed.trim_start();
            let (marker, field) = body
                .strip_prefix("- ")
                .map_or(("", body), |rest| ("- ", rest));
            let own = field.strip_prefix("key: ").is_some_and(|value| {
                entry
                    .key
                    .as_ref()
                    .is_some_and(|key| quoted(key).iter().any(|form| form == value))
            }) || field
                .strip_prefix("id: ")
                .is_some_and(|value| quoted(&own_id).iter().any(|form| form == value));
            if !own {
                return line.to_owned();
            }
            let indent = &trimmed[..trimmed.len() - body.len()];
            format!("{indent}{marker}id: {new_id}{}", &line[trimmed.len()..])
        })
        .collect()
}

/// Reconnect structured references without changing the note's prose or other fields.
pub fn redirect_reference(text: &str, old: &Entry, new_id: Uuid) -> Result<String, String> {
    fn replace(value: &mut Value, old: &Entry, new_id: Uuid) {
        match value {
            Value::Mapping(map) => {
                let matches = map.get("id").and_then(Value::as_str)
                    == Some(old.id.to_string().as_str())
                    || old
                        .key
                        .as_deref()
                        .is_some_and(|key| map.get("key").and_then(Value::as_str) == Some(key));
                if matches {
                    map.remove("key");
                    map.insert("id".into(), new_id.to_string().into());
                }
                if let Some(Value::Mapping(related)) = map.get_mut("related") {
                    for target in [Some(old.id.to_string()), old.key.clone()]
                        .into_iter()
                        .flatten()
                    {
                        if let Some(label) = related.remove(Value::String(target)) {
                            related.insert(new_id.to_string().into(), label);
                        }
                    }
                }
                for value in map.values_mut() {
                    replace(value, old, new_id);
                }
            }
            Value::Sequence(values) => {
                for value in values {
                    replace(value, old, new_id);
                }
            }
            _ => {}
        }
    }
    let entry = Entry::parse("reference.md", text)?;
    let mut changes = Vec::new();
    for (key, value) in entry.fields.iter() {
        if ["splat_id", "splat_origin"].contains(&key.as_str().unwrap_or_default()) {
            continue;
        }
        let mut updated = value.clone();
        // Readable references are model leaves, never prose substitutions.
        let redirect = |value: &mut Value| {
            if value
                .as_str()
                .is_some_and(|raw| content::formats::resolve_readable(raw, [old]) == Ok(old.id))
            {
                *value = serde_yaml_ng::to_value(EntryRef::id(new_id)).expect("entry reference");
            }
        };
        if key.as_str() == Some("splat_requires") {
            if let Some(values) = updated.as_sequence_mut() {
                for value in values {
                    redirect(value);
                }
            }
        } else if key.as_str() == Some("splat_world") {
            for field in ["creature", "in"] {
                if let Some(value) = updated.get_mut(field) {
                    redirect(value);
                }
            }
        }
        replace(&mut updated, old, new_id);
        if updated != *value {
            changes.push((key.as_str().ok_or("Invalid field name")?, Some(updated)));
        }
    }
    let updated = content::formats::patch_model(text, &changes)?;
    let Some((front, body)) = crate::sessions::split_note(&updated) else {
        return Ok(updated);
    };
    Ok(format!(
        "{front}{}",
        crate::world::redirect_links(body, old, new_id)
    ))
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
    pub shared: Vec<CopyPlan>,
}

/// Copies use the same creature fields as a new note. Keep every prose block.
pub(crate) fn creature_copy_fields(text: &str, entry: &Entry) -> Result<String, String> {
    use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
    if entry.kind != "creature"
        || entry.fields.get("splat_schema").and_then(Value::as_u64) == Some(2)
    {
        return Ok(text.to_owned());
    }
    let mut heading = None;
    let mut pending = None;
    let mut ranges = Vec::new();
    for (event, range) in Parser::new_ext(text, Options::ENABLE_TABLES).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { .. }) => heading = Some(range.clone()),
            Event::Html(ref html)
                if html.starts_with("<!-- splat:")
                    && ["splat_creature", "splat_stats"]
                        .iter()
                        .any(|key| html.contains(&format!("path={key} -->"))) =>
            {
                let start = heading
                    .as_ref()
                    .filter(|heading| text[heading.end..range.start].trim().is_empty())
                    .map_or(range.start, |heading| heading.start);
                pending = Some(start);
            }
            Event::End(TagEnd::Table) => {
                if let Some(start) = pending.take() {
                    ranges.push(start..range.end);
                }
            }
            _ => {}
        }
    }
    let mut output = text.to_owned();
    for range in ranges.into_iter().rev() {
        output.replace_range(range, "");
    }
    let changes: Vec<_> = ["splat_creature", "splat_stats"]
        .into_iter()
        .filter_map(|key| {
            entry
                .fields
                .get(key)
                .map(|value| (key, Some(value.clone())))
        })
        .collect();
    let mut output = patch_fields(&output, &changes)?;
    for (key, order) in [
        (
            "splat_creature",
            &["size", "type", "alignment", "cr_text"][..],
        ),
        (
            "splat_stats",
            &[
                "max_hp",
                "armor_class",
                "initiative",
                "speed",
                "swim_speed",
                "strength",
                "dexterity",
                "constitution",
                "intelligence",
                "wisdom",
                "charisma",
                "save_strength",
                "save_dexterity",
                "save_constitution",
                "save_intelligence",
                "save_wisdom",
                "save_charisma",
            ][..],
        ),
    ] {
        let Some(map) = entry.fields.get(key).and_then(Value::as_mapping) else {
            continue;
        };
        if map.is_empty() {
            continue;
        }
        let start = output
            .find(&format!("\n{key}:\n"))
            .ok_or("Missing creature fields")?
            + 1;
        let end = output[start..].find('\n').unwrap() + start + 1;
        let length = output[end..]
            .split_inclusive('\n')
            .take_while(|line| line.starts_with(' '))
            .map(str::len)
            .sum::<usize>();
        let mut keys: Vec<_> = map.keys().filter_map(Value::as_str).collect();
        keys.sort_by_key(|name| {
            (
                order
                    .iter()
                    .position(|key| key == name || (*key == "speed" && name.ends_with("_speed")))
                    .unwrap_or(order.len()),
                *name,
            )
        });
        let mut block = format!("{key}:\n");
        for name in keys {
            block.push_str(&format!("  {name}: {}\n", flow_yaml(&map[name])?));
        }
        output.replace_range(start..end + length, &block);
    }
    Ok(output)
}

fn flow_yaml(value: &Value) -> Result<String, String> {
    Ok(match value {
        Value::String(_) => serde_yaml_ng::to_string(value)
            .map_err(|e| e.to_string())?
            .trim()
            .to_owned(),
        Value::Mapping(map) => format!(
            "{{{}}}",
            {
                let mut fields: Vec<_> = map.iter().collect();
                fields.sort_by_key(|(key, _)| match key.as_str() {
                    Some("value") => (0, ""),
                    Some("unit") => (1, ""),
                    _ => (2, key.as_str().unwrap_or("")),
                });
                fields
            }
            .into_iter()
            .map(|(key, value)| {
                let key = key
                    .as_str()
                    .filter(|key| {
                        !key.is_empty()
                            && key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    })
                    .map_or_else(|| flow_yaml(key), |key| Ok(key.to_owned()))?;
                Ok(format!("{key}: {}", flow_yaml(value)?))
            })
            .collect::<Result<Vec<_>, String>>()?
            .join(", ")
        ),
        Value::Sequence(items) => format!(
            "[{}]",
            items
                .iter()
                .map(flow_yaml)
                .collect::<Result<Vec<_>, _>>()?
                .join(", ")
        ),
        _ => serde_yaml_ng::to_string(value)
            .map_err(|e| e.to_string())?
            .trim()
            .to_owned(),
    })
}

impl CopyPlan {
    fn requirements(entry: &Entry, catalogue: &Catalogue) -> Result<Vec<Reference>, String> {
        let mut entry = entry.clone();
        if let Some(values) = entry
            .fields
            .get_mut("splat_requires")
            .and_then(Value::as_sequence_mut)
        {
            for value in values {
                if let Some(raw) = value.as_str() {
                    let id = content::formats::resolve_readable(
                        raw,
                        catalogue.entries.values().map(|item| &item.entry),
                    )?;
                    *value =
                        serde_yaml_ng::to_value(EntryRef::id(id)).map_err(|e| e.to_string())?;
                }
            }
        }
        content::required_references(&entry)
    }

    pub fn prepare(
        source: &CopySource,
        destination: &Path,
        destination_campaign: Option<Uuid>,
        name: &str,
        folder: &str,
        catalogue: &Catalogue,
    ) -> Result<Self, String> {
        Self::prepare_inner(
            source,
            destination,
            destination_campaign,
            name,
            folder,
            catalogue,
            false,
        )
    }

    /// Review all copies before writing any of them. Cycles use the assigned new identities.
    pub fn prepare_sharing(
        source: &CopySource,
        destination: &Path,
        name: &str,
        folder: &str,
        catalogue: &Catalogue,
    ) -> Result<Self, String> {
        let mut entries = BTreeMap::from([(source.entry.id, source.entry.clone())]);
        let mut queue = VecDeque::from([source.entry.clone()]);
        while let Some(entry) = queue.pop_front() {
            for reference in Self::requirements(&entry, catalogue)? {
                let id = catalogue.resolve(&reference)?;
                if let std::collections::btree_map::Entry::Vacant(slot) = entries.entry(id) {
                    let dependency = catalogue.entries[&id].entry.clone();
                    slot.insert(dependency.clone());
                    queue.push_back(dependency);
                }
            }
        }
        let mut copying: BTreeSet<Uuid> = entries
            .keys()
            .filter(|id| catalogue.entries.get(id).is_some_and(|e| e.private))
            .copied()
            .collect();
        // Copy shared intermediates too when they need a campaign dependency.
        loop {
            let before = copying.len();
            for (id, entry) in &entries {
                if Self::requirements(entry, catalogue)?
                    .iter()
                    .any(|r| catalogue.resolve(r).is_ok_and(|id| copying.contains(&id)))
                {
                    copying.insert(*id);
                }
            }
            if before == copying.len() {
                break;
            }
        }
        copying.remove(&source.entry.id);
        let mut plan =
            Self::prepare_inner(source, destination, None, name, folder, catalogue, true)?;
        for id in copying {
            let entry = &entries[&id];
            let (path, text) = catalogue
                .files
                .get(&id)
                .ok_or("The required note has no readable file. Reload and try again.")?;
            let root = path
                .ancestors()
                .find(|root| note_path(root, &entry.path).is_ok_and(|candidate| candidate == *path))
                .ok_or("Required note folder is missing")?;
            let dependency = CopySource {
                entry: entry.clone(),
                text: text.clone(),
                root: Some(root.into()),
                campaign: None,
            };
            plan.shared.push(Self::prepare_inner(
                &dependency,
                destination,
                None,
                &entry.name,
                folder,
                catalogue,
                true,
            )?);
        }
        let mapping: Vec<_> = std::iter::once((&source.entry, &plan))
            .chain(plan.shared.iter().map(|p| {
                let original = entries
                    .values()
                    .find(|e| {
                        p.source_path.as_ref().is_some_and(|path| {
                            catalogue
                                .files
                                .get(&e.id)
                                .is_some_and(|(file, _)| file == path)
                        })
                    })
                    .expect("reviewed source");
                (original, p)
            }))
            .map(|(entry, p)| Ok((entry.clone(), Entry::parse(&p.relative, &p.text)?.id)))
            .collect::<Result<_, String>>()?;
        for (old, new_id) in mapping {
            plan.text = redirect_reference(&plan.text, &old, new_id)?;
            for dependency in &mut plan.shared {
                dependency.text = redirect_reference(&dependency.text, &old, new_id)?;
            }
        }
        let mut paths = BTreeSet::new();
        for p in std::iter::once(&plan).chain(plan.shared.iter()) {
            if !paths.insert(&p.relative) {
                return Err(
                    "Two copies need the same file name. Rename a source note before sharing."
                        .into(),
                );
            }
        }
        Ok(plan)
    }

    #[allow(clippy::too_many_arguments)]
    fn prepare_inner(
        source: &CopySource,
        destination: &Path,
        destination_campaign: Option<Uuid>,
        name: &str,
        folder: &str,
        catalogue: &Catalogue,
        sharing: bool,
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
            for reference in Self::requirements(&entry, catalogue)? {
                let id = catalogue.resolve(&reference)?;
                // Its own structured references follow the copy, below.
                if id == source.entry.id {
                    continue;
                }
                let dependency = &catalogue.entries[&id];
                if !sharing && dependency.private && destination_campaign != Some(dependency.owner)
                {
                    return Err(format!(
                        "{} needs {}, which is in this campaign only.",
                        source.entry.name, dependency.entry.name
                    ));
                }
                dependencies.insert(dependency.entry.name.clone());
                if seen.insert(id) {
                    queue.push_back(dependency.entry.clone());
                }
            }
        }
        let new_id = Uuid::new_v4();
        // A were-creature's curse makes more of this creature, and the copy's makes more of the
        // copy. Prose that only names the original keeps its words.
        let normalized = creature_copy_fields(&source.text, &source.entry)?;
        let normalized = if let Some((generated, tail)) =
            normalized.rsplit_once("\n<!-- splat-generated-sha256:")
            && let Some((_, notes)) = tail.split_once(" -->\n")
        {
            format!("{generated}{notes}")
        } else {
            normalized
        };
        let source_text = own_references_to(&normalized, &source.entry, new_id);
        let mut text = patch_fields(
            &source_text,
            &[
                ("splat_id", Some(new_id.to_string().into())),
                ("splat_key", None),
                ("splat_name", Some(name.trim().into())),
                ("splat_origin", Some(source.entry.id.to_string().into())),
            ],
        )?;
        // Rename only the first matching H1. Keep all other prose and citations.
        let body_start = text.find("\n---").map(|at| at + "\n---".len()).unwrap_or(0);
        let mut offset = body_start;
        for line in text[body_start..].split_inclusive('\n') {
            if let Some(heading) = line.trim_end().strip_prefix("# ") {
                if heading.trim() == source.entry.name {
                    let end = offset + line.trim_end().len();
                    text.replace_range(offset..end, &format!("# {}", name.trim()));
                }
                break;
            }
            offset += line.len();
        }
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
        let text = content::formats::convert(&relative, &text)?;
        Entry::parse(&relative, &text)?;
        Ok(Self {
            root: destination.into(),
            shared: Vec::new(),
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
    fn validate(&self) -> Result<(), String> {
        if note_path(&self.root, &self.relative)?.exists() {
            return Err(
                "A note already exists at this destination. Review the copies again.".into(),
            );
        }
        for dependency in &self.shared {
            dependency.validate()?;
        }
        for (path, original) in &self.dependency_files {
            if fs::read_to_string(path).map_err(|e| e.to_string())? != *original {
                return Err("A required entry changed after review. Close the dialog and review a new copy.".into());
            }
        }
        if let Some(path) = &self.source_path
            && fs::read_to_string(path).map_err(|e| e.to_string())? != self.source_text
        {
            return Err("The source changed on disk. Reload it and review a new copy.".into());
        }
        Ok(())
    }
    pub fn save(&self) -> Result<Entry, String> {
        self.validate()?;
        let mut written = Vec::new();
        for dependency in &self.shared {
            match dependency.save() {
                Ok(_) => written.push(note_path(&dependency.root, &dependency.relative)?),
                Err(error) => {
                    for path in written {
                        let _ = fs::remove_file(path);
                    }
                    return Err(error);
                }
            }
        }
        let result = self.save_one();
        if result.is_err() {
            for path in written {
                let _ = fs::remove_file(path);
            }
        }
        result
    }
    fn save_one(&self) -> Result<Entry, String> {
        let path = note_path(&self.root, &self.relative)?;
        fs::create_dir_all(path.parent().ok_or("Missing destination folder")?)
            .map_err(|e| e.to_string())?;
        write_note(&self.root, &self.relative, None, &self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changed_package_id_keeps_entries_and_repairs_the_actual_manifest() {
        let temp = tempfile::tempdir().unwrap();
        let manifest = note_text("Marches", "package", Some("marches"));
        fs::write(temp.path().join("Module.md"), &manifest).unwrap();
        let module = crate::library::local_module(temp.path()).unwrap();
        let wolf = note_text("Wolf", "creature", None);
        let wolf_id = note_id(&wolf).unwrap();
        fs::write(temp.path().join("Wolf.md"), &wolf).unwrap();
        let changed = patch_fields(
            &manifest,
            &[("splat_id", Some(Uuid::new_v4().to_string().into()))],
        )
        .unwrap();
        fs::write(temp.path().join("Module.md"), &changed).unwrap();
        let catalogue = Catalogue::load(std::slice::from_ref(&module), None).unwrap();
        assert!(catalogue.entries.contains_key(&wolf_id));
        assert!(catalogue.manifest_problems[&module.id].contains("another identity"));
        let repair = ManifestWrite::write(&module).unwrap();
        assert_eq!(
            crate::library::local_module(temp.path()).unwrap().id,
            module.id
        );
        assert!(!temp.path().join("Package.md").exists());
        repair.undo().unwrap();
        assert_eq!(
            fs::read_to_string(temp.path().join("Module.md")).unwrap(),
            changed
        );
        assert_eq!(
            fs::read_to_string(temp.path().join("Wolf.md")).unwrap(),
            wolf
        );
    }

    #[test]
    fn folder_identity_repair_redirects_its_links_and_undo_checks_every_file() {
        let temp = tempfile::tempdir().unwrap();
        let wolf = note_text("Wolf", "creature", None);
        let old = Entry::parse("Wolf.md", &wolf).unwrap();
        let rule = patch_fields(
            &note_text("Rule", "rule", None),
            &[(
                "splat_requires",
                Some(serde_yaml_ng::to_value(vec![EntryRef::id(old.id)]).unwrap()),
            )],
        )
        .unwrap();
        fs::write(temp.path().join("Wolf.md"), &wolf).unwrap();
        fs::write(temp.path().join("Rule.md"), &rule).unwrap();
        let edits =
            ContentEdits::new_identities(temp.path(), &["Wolf.md".into(), "Rule.md".into()])
                .unwrap();
        let (entries, errors) = content::read_directory(temp.path()).unwrap();
        assert!(errors.is_empty());
        let new = entries.iter().find(|entry| entry.name == "Wolf").unwrap();
        let rule_entry = entries.iter().find(|entry| entry.name == "Rule").unwrap();
        assert_ne!(new.id, old.id);
        assert_eq!(
            content::required_references(rule_entry).unwrap(),
            vec![Reference::Id(new.id)]
        );
        let repaired_wolf = fs::read_to_string(temp.path().join("Wolf.md")).unwrap();
        fs::write(temp.path().join("Rule.md"), "Later edit").unwrap();
        assert!(edits.undo().is_err());
        assert_eq!(
            fs::read_to_string(temp.path().join("Wolf.md")).unwrap(),
            repaired_wolf
        );
        fs::write(temp.path().join("Rule.md"), &edits.files[1].3).unwrap();
        edits.undo().unwrap();
        assert_eq!(
            fs::read_to_string(temp.path().join("Wolf.md")).unwrap(),
            wolf
        );
        assert_eq!(
            fs::read_to_string(temp.path().join("Rule.md")).unwrap(),
            rule
        );
    }

    #[test]
    fn identity_repair_keeps_a_different_heading_and_other_fields() {
        let before =
            note_text("Rook", "creature", None).replace("# Rook", "# Watch captain") + "\n# Rook\n";
        let id = Uuid::new_v4();
        let after = new_note_identity(&before, "Rook copy.md", id).unwrap();
        let parsed = Entry::parse("Rook copy.md", &after).unwrap();
        assert_eq!(parsed.id, id);
        assert_eq!(parsed.name, "Rook copy");
        assert_eq!(
            parsed.fields["splat_creature"],
            Entry::parse("Rook.md", &before).unwrap().fields["splat_creature"]
        );
        assert!(parsed.body.contains("# Watch captain\n"));
        assert!(parsed.body.contains("# Rook\n"));
    }

    #[test]
    fn duplicate_keys_keep_both_files_and_other_content_available() {
        let temp = tempfile::tempdir().unwrap();
        fs::write(
            temp.path().join("Package.md"),
            note_text("Test", "package", Some("test")),
        )
        .unwrap();
        let module = crate::library::local_module(temp.path()).unwrap();
        let mut ids = Vec::new();
        for name in ["First", "Second", "Third"] {
            let text = patch_fields(
                &note_text(name, "creature", None),
                &[("splat_key", Some("test:wolf".into()))],
            )
            .unwrap();
            ids.push(Entry::parse(name, &text).unwrap().id);
            fs::write(temp.path().join(format!("{name}.md")), text).unwrap();
        }
        let catalogue = Catalogue::load(&[module], None).unwrap();
        assert!(ids.iter().all(|id| catalogue.entries.contains_key(id)));
        assert!(
            catalogue
                .resolve(&Reference::Key("test:wolf".into()))
                .is_err()
        );
        let errors = catalogue.errors.join("\n");
        assert!(
            errors.contains("First.md")
                && errors.contains("Second.md")
                && errors.contains("Third.md")
        );
    }

    #[test]
    fn package_copy_refuses_broken_files_without_changing_the_manifest() {
        let temp = tempfile::tempdir().unwrap();
        let manifest = note_text("Original", "package", Some("original"));
        fs::write(temp.path().join("Package.md"), &manifest).unwrap();
        fs::write(temp.path().join("Broken.md"), "---\nsplat_id: [\n---\n").unwrap();
        assert!(PackageCopy::write(temp.path(), "Copy").is_err());
        assert_eq!(
            fs::read_to_string(temp.path().join("Package.md")).unwrap(),
            manifest
        );
    }

    #[test]
    fn package_copy_keeps_original_entries_encounters_and_byte_exact_undo() {
        use crate::visual_tests::fixture::{Seed, World};
        let temp = tempfile::tempdir().unwrap();
        let world = World::build(
            temp.path(),
            &temp.path().join("cache"),
            "copy-test",
            Seed::Campaign,
        )
        .unwrap();
        let app = world.app();
        let original = &app.campaigns.modules()[0];
        let campaign = app.campaigns.active_campaign().unwrap();
        let copied = temp.path().join("copy");
        fs::create_dir(&copied).unwrap();
        let (entries, errors) = content::read_directory(&original.path).unwrap();
        assert!(errors.is_empty());
        let before: Vec<_> = entries
            .iter()
            .map(|e| {
                let text = fs::read_to_string(original.path.join(&e.path)).unwrap();
                let to = copied.join(&e.path);
                fs::create_dir_all(to.parent().unwrap()).unwrap();
                fs::write(to, &text).unwrap();
                (e.path.clone(), text)
            })
            .collect();
        let copy = PackageCopy::write(&copied, "New Marches").unwrap();
        let modules = [original.clone(), copy.module.clone()];
        let catalogue = Catalogue::load(&modules, Some(&campaign)).unwrap();
        assert!(catalogue.errors.is_empty(), "{:?}", catalogue.errors);
        for entry in &entries {
            assert_eq!(catalogue.entries[&entry.id].owner, original.id);
            assert_eq!(
                fs::read_to_string(original.path.join(&entry.path)).unwrap(),
                before.iter().find(|(p, _)| p == &entry.path).unwrap().1
            );
        }
        let (copies, _) = content::read_directory(&copied).unwrap();
        assert_eq!(copies.len(), entries.len());
        for entry in &copies {
            assert!(
                !entries
                    .iter()
                    .any(|old| old.id == entry.id || old.key == entry.key)
            );
            assert_eq!(catalogue.entries[&entry.id].owner, copy.module.id);
            for reference in content::required_references(entry).unwrap() {
                assert!(catalogue.resolve(&reference).is_ok());
            }
        }
        for name in ["Bridge Ambush", "Shrine of Cinders"] {
            let encounter = catalogue
                .entries
                .values()
                .find(|e| e.entry.name == name)
                .unwrap();
            let refs = content::required_references(&encounter.entry).unwrap();
            assert!(!refs.is_empty());
            for reference in refs {
                assert!(catalogue.resolve(&reference).is_ok());
            }
        }
        let (rosters, _) = content::read_directory(&campaign.path).unwrap();
        let rosters: Vec<_> = rosters
            .into_iter()
            .filter(|entry| matches!(entry.kind.as_str(), "encounter" | "encounter_group"))
            .collect();
        let originals: Vec<_> = rosters
            .iter()
            .map(|entry| fs::read_to_string(campaign.path.join(&entry.path)).unwrap())
            .collect();
        let edits = copy.use_in_rosters(&campaign).unwrap();
        assert!(!edits.files.is_empty());
        for (entry, before) in rosters.iter().zip(&originals) {
            let mut expected = before.clone();
            for old in &entries {
                let replacement = copies.iter().find(|new| new.path == old.path).unwrap();
                expected = redirect_reference(&expected, old, replacement.id).unwrap();
            }
            let after = fs::read_to_string(campaign.path.join(&entry.path)).unwrap();
            assert_eq!(
                after, expected,
                "Only references change; counts, names, changes and prose stay: {}",
                entry.path
            );
        }
        edits.undo().unwrap();
        for (entry, before) in rosters.iter().zip(&originals) {
            assert_eq!(
                fs::read_to_string(campaign.path.join(&entry.path)).unwrap(),
                *before
            );
        }
        copy.undo().unwrap();
        for (path, text) in &before {
            assert_eq!(fs::read(copied.join(path)).unwrap(), text.as_bytes());
        }
        let copy = PackageCopy::write(&copied, "New Marches").unwrap();
        fs::write(copied.join("Module.md"), "A later edit").unwrap();
        assert!(copy.undo().is_err());
        for (path, _, after) in &copy.files {
            if path != "Module.md" {
                assert_eq!(fs::read_to_string(copied.join(path)).unwrap(), *after);
            }
        }
    }

    #[test]
    fn manifest_write_preserves_identity_entries_and_supports_safe_undo() {
        let temp = tempfile::tempdir().unwrap();
        let folder = temp.path().join("package");
        fs::create_dir(&folder).unwrap();
        fs::write(
            folder.join("Module.md"),
            note_text("Test", "package", Some("test")),
        )
        .unwrap();
        let module = crate::library::local_module(&folder).unwrap();
        let wolf = patch_fields(
            &note_text("Wolf", "creature", None),
            &[("splat_key", Some("test:wolf".into()))],
        )
        .unwrap();
        fs::write(module.path.join("Wolf.md"), wolf).unwrap();
        let valid = fs::read_to_string(module.path.join("Module.md")).unwrap();
        fs::rename(
            module.path.join("Module.md"),
            module.path.join("Package.md"),
        )
        .unwrap();
        let count = Catalogue::load(std::slice::from_ref(&module), None)
            .unwrap()
            .entries
            .len();
        assert!(ManifestWrite::write(&module).is_err());
        assert_eq!(
            fs::read_to_string(module.path.join("Package.md")).unwrap(),
            valid
        );
        for broken in [true, false] {
            let path = module.path.join("Package.md");
            let bad = "---\nsplat_kind: [\n---\n";
            if broken {
                fs::write(&path, bad).unwrap();
            } else {
                fs::remove_file(&path).unwrap();
            }
            let repair = ManifestWrite::write(&module).unwrap();
            assert_eq!(repair.replaced(), broken);
            let catalogue = Catalogue::load(std::slice::from_ref(&module), None).unwrap();
            assert!(catalogue.manifest_problems.is_empty());
            assert_eq!(catalogue.entries.len(), count);
            assert_eq!(catalogue.entries[&module.id].entry.name, "Test");
            assert_eq!(
                catalogue.entries[&module.id].entry.fields["splat_namespace"].as_str(),
                Some("test")
            );
            assert_eq!(
                crate::library::local_module(&module.path).unwrap().id,
                module.id
            );
            if broken {
                repair.undo().unwrap();
                assert_eq!(fs::read_to_string(&path).unwrap(), bad);
                let repair = ManifestWrite::write(&module).unwrap();
                fs::write(&path, format!("{}\nExternal change.\n", repair.after)).unwrap();
                assert!(repair.undo().is_err());
            }
        }
    }

    #[test]
    fn broken_or_deleted_package_manifest_keeps_entries_and_reports_repair() {
        let temp = tempfile::tempdir().unwrap();
        let module = Module {
            id: Uuid::new_v4(),
            name: "Test".into(),
            path: temp.path().into(),
        };
        let text = note_text("Wolf", "creature", None);
        let id = Entry::parse("Wolf.md", &text).unwrap().id;
        fs::write(temp.path().join("Wolf.md"), text).unwrap();
        let manifest = temp.path().join("Package.md");
        let before = format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: package\nsplat_name: Test\nsplat_namespace: test\n---\n",
            module.id
        );
        fs::write(&manifest, "---\nsplat_id: [\n---\n").unwrap();
        for deleted in [false, true] {
            if deleted {
                fs::remove_file(&manifest).unwrap();
            }
            let catalogue = Catalogue::load(std::slice::from_ref(&module), None).unwrap();
            assert!(catalogue.entries.contains_key(&id));
            assert_eq!(catalogue.entries[&id].owner, module.id);
            assert!(
                catalogue.manifest_problems[&module.id].contains(if deleted {
                    "deleted"
                } else {
                    "cannot be read"
                })
            );
            assert!(
                catalogue
                    .closure(
                        &[PackageSelection {
                            name: None,
                            package: EntryRef::id(module.id),
                            selection: Selection::All
                        }],
                        Uuid::new_v4()
                    )
                    .direct
                    .contains(&id)
            );
        }
        fs::write(&manifest, before).unwrap();
        assert!(
            Catalogue::load(&[module], None)
                .unwrap()
                .manifest_problems
                .is_empty()
        );
    }

    #[test]
    fn unreadable_package_note_keeps_its_id_path_and_file_line() {
        let temp = tempfile::tempdir().unwrap();
        let module = Module {
            id: Uuid::new_v4(),
            name: "Test".into(),
            path: temp.path().into(),
        };
        fs::write(module.path.join("Package.md"), format!("---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: package\nsplat_name: Test\nsplat_namespace: test\n---\n", module.id)).unwrap();
        let id = Uuid::new_v4();
        let text =
            format!("---\nsplat_schema: 1\nsplat_id: '{id}' # keep identity\nsplat_kind: [\n---\n");
        fs::write(module.path.join("Wolf.md"), &text).unwrap();
        let catalogue = Catalogue::load(std::slice::from_ref(&module), None).unwrap();
        let note = &catalogue.unreadable[&id];
        assert_eq!(note.path, module.path.join("Wolf.md"));
        assert_eq!(note.word, "Wolf.md cannot be read (line 4)");
        assert!(!catalogue.entries.contains_key(&id));
        assert_eq!(note_id(&format!("No front matter\nsplat_id: {id}")), None);
        assert_eq!(
            note_id(&format!("---\nsplat_id: [\n---\nsplat_id: {id}")),
            None
        );
    }

    #[test]
    fn copied_aboleth_keeps_book_stats_as_front_matter() {
        let catalogue = Catalogue::load(&[], None).unwrap();
        let entry = catalogue
            .entries
            .values()
            .find(|e| e.entry.name == "Aboleth" && e.entry.kind == "creature")
            .unwrap()
            .entry
            .clone();
        let source = CopySource {
            text: catalogue.text_of(entry.id).unwrap(),
            entry,
            root: None,
            campaign: None,
        };
        let temp = tempfile::tempdir().unwrap();
        let plan =
            CopyPlan::prepare(&source, temp.path(), None, "Watch Aboleth", "", &catalogue).unwrap();
        let copied = Entry::parse(&plan.relative, &plan.text).unwrap();
        let stats = plan.text.split("splat_stats:\n").nth(1).unwrap();
        assert!(stats.starts_with("  max_hp: {value: 150}\n"));
        let names: Vec<_> = stats
            .lines()
            .take_while(|line| line.starts_with(' '))
            .map(|line| {
                assert!(
                    line.starts_with("  ") && !line.starts_with("    "),
                    "one line per stat: {line}"
                );
                line.trim().split(':').next().unwrap()
            })
            .collect();
        assert_eq!(
            &names[..11],
            &[
                "max_hp",
                "armor_class",
                "initiative",
                "speed",
                "swim_speed",
                "strength",
                "dexterity",
                "constitution",
                "intelligence",
                "wisdom",
                "charisma"
            ]
        );
        assert!(stats.contains("speed: {value: 10, unit: ft}"));
        assert!(plan.text.contains("size: large"));
        for key in ["splat_creature", "splat_stats"] {
            assert_eq!(copied.fields.get(key), source.entry.fields.get(key));
            assert!(
                plan.text
                    .split("\n---")
                    .next()
                    .unwrap()
                    .contains(&format!("{key}:"))
            );
            assert!(!copied.body.contains(&format!("path={key}")));
        }
        assert!(
            copied
                .body
                .contains("The aboleth can breathe air and water.")
        );
    }
    fn entry(name: &str, kind: &str) -> Entry {
        Entry::parse(&format!("{name}.md"), &note_text(name, kind, None)).unwrap()
    }
    fn campaign(path: PathBuf) -> Campaign {
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Test campaign".into(),
            path,
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        if campaign.path.is_dir() && !campaign.path.join("Campaign.md").exists() {
            crate::content_workspace::CampaignDocument::new(&campaign)
                .unwrap()
                .save()
                .unwrap();
        }
        campaign
    }

    #[test]
    fn fmt_f1_readable_requirements_resolve_or_stay_flagged() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path().into());
        fs::create_dir(dir.path().join("Rules")).unwrap();
        let target = note_text("Beacon", "rule", None);
        let target_id = note_id(&target).unwrap();
        fs::write(dir.path().join("Rules/Beacon.md"), target).unwrap();
        let source = patch_fields(
            &note_text("Rook", "rule", None),
            &[(
                "splat_requires",
                Some(Value::Sequence(vec!["[[Rules/Beacon|Beacon]]".into()])),
            )],
        )
        .unwrap();
        let source = content::formats::convert("Rook.md", &source).unwrap();
        let source_id = note_id(&source).unwrap();
        fs::write(dir.path().join("Rook.md"), &source).unwrap();
        let cat = Catalogue::load(&[], Some(&campaign)).unwrap();
        assert_eq!(
            content::required_references(&cat.entries[&source_id].entry).unwrap(),
            [Reference::Id(target_id)]
        );
        assert!(cat.entries[&source_id].entry.flags.is_empty());
        let copy_source = CopySource {
            entry: Entry::parse("Rook.md", &source).unwrap(),
            text: source.clone(),
            root: Some(dir.path().into()),
            campaign: Some(campaign.clone()),
        };
        let destination = tempfile::tempdir().unwrap();
        let plan =
            CopyPlan::prepare_sharing(&copy_source, destination.path(), "Rook copy", "", &cat)
                .unwrap();
        assert_eq!(plan.shared.len(), 1);
        let copied = plan.save().unwrap();
        let dependency = Entry::parse(
            "Beacon.md",
            &fs::read_to_string(destination.path().join("Beacon.md")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            content::required_references(&copied).unwrap(),
            [Reference::Id(dependency.id)]
        );
        fs::remove_file(dir.path().join("Rules/Beacon.md")).unwrap();
        let cat = Catalogue::load(&[], Some(&campaign)).unwrap();
        assert!(content::required_references(&cat.entries[&source_id].entry).is_err());
        assert_eq!(cat.entries[&source_id].entry.flags.len(), 1);
        assert_eq!(
            fs::read_to_string(dir.path().join("Rook.md")).unwrap(),
            source
        );
    }
    fn require(entry: &mut Entry, id: Uuid) {
        entry.fields.insert(
            "splat_requires".into(),
            serde_yaml_ng::to_value(vec![EntryRef::id(id)]).unwrap(),
        );
    }
    #[test]
    fn sharing_reconnects_dependencies_and_checks_every_source_before_writing() {
        let temp = tempfile::tempdir().unwrap();
        let source_root = temp.path().join("campaign");
        let destination = temp.path().join("package");
        fs::create_dir_all(&source_root).unwrap();
        fs::create_dir_all(&destination).unwrap();
        let owner = Uuid::new_v4();
        let dependency_text = note_text("Captain Varra", "creature", None);
        let dependency = Entry::parse("Captain Varra.md", &dependency_text).unwrap();
        let source_text = patch_fields(
            &note_text("Watch Orders", "rule", None),
            &[(
                "splat_requires",
                Some(serde_yaml_ng::to_value(vec![EntryRef::id(dependency.id)]).unwrap()),
            )],
        )
        .unwrap();
        let source_entry = Entry::parse("Watch Orders.md", &source_text).unwrap();
        let mut catalogue = Catalogue::default();
        for (entry, text) in [
            (&source_entry, &source_text),
            (&dependency, &dependency_text),
        ] {
            let path = source_root.join(&entry.path);
            fs::write(&path, text).unwrap();
            catalogue.files.insert(entry.id, (path, text.clone()));
            catalogue.insert(entry.clone(), owner, true).unwrap();
        }
        let source = CopySource {
            entry: source_entry.clone(),
            text: source_text.clone(),
            root: Some(source_root.clone()),
            campaign: None,
        };
        assert!(
            CopyPlan::prepare(
                &source,
                &destination,
                None,
                "Watch Orders",
                "Watch",
                &catalogue
            )
            .is_err()
        );
        let plan =
            CopyPlan::prepare_sharing(&source, &destination, "Watch Orders", "Watch", &catalogue)
                .unwrap();
        fs::write(
            source_root.join(&dependency.path),
            format!("{dependency_text}\nExternal change"),
        )
        .unwrap();
        assert!(plan.save().is_err());
        assert!(!destination.join("Watch/Watch Orders.md").exists());
        assert!(!destination.join("Watch/Captain Varra.md").exists());
        fs::write(source_root.join(&dependency.path), &dependency_text).unwrap();
        let copied = plan.save().unwrap();
        let copied_dependency = Entry::parse(
            "Watch/Captain Varra.md",
            &fs::read_to_string(destination.join("Watch/Captain Varra.md")).unwrap(),
        )
        .unwrap();
        assert_ne!(copied_dependency.id, dependency.id);
        assert_ne!(copied.id, source_entry.id);
        assert_eq!(
            content::required_references(&copied).unwrap(),
            vec![Reference::Id(copied_dependency.id)]
        );
        assert_eq!(
            fs::read_to_string(source_root.join(&source_entry.path)).unwrap(),
            source_text
        );
        assert_eq!(
            fs::read_to_string(source_root.join(&dependency.path)).unwrap(),
            dependency_text
        );
    }

    #[test]
    fn reconnecting_a_related_entry_preserves_prose_and_unrelated_fields() {
        let old = entry("Captain Varra", "creature");
        let original = format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: rule\nsplat_name: Ash Exposure\nsplat_rule: {{}}\nsplat_reading:\n  title: Ash Exposure\n  summary: Keep these words.\n  related:\n    {}: Read Captain Varra\ncustom: 'keep the quotes'\n---\n\nCaptain Varra keeps her name here.\n",
            Uuid::new_v4(),
            old.id
        );
        let new_id = Uuid::new_v4();
        let changed = redirect_reference(&original, &old, new_id).unwrap();
        assert!(changed.contains(&new_id.to_string()));
        assert!(!changed.contains(&old.id.to_string()));
        assert!(changed.contains("custom: 'keep the quotes'"));
        assert!(changed.ends_with("Captain Varra keeps her name here.\n"));
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
            private: false,
        });
        let selection = vec![PackageSelection {
            name: None,
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
                name: None,
                package: EntryRef::id(owner),
                selection: Selection::All,
            }],
            Uuid::new_v4(),
        );
        assert!(all.direct.contains(&optional_id));
        assert!(all.direct.contains(&later_id));
        let unavailable = cat.closure(
            &[PackageSelection {
                name: None,
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
    fn manifest_mismatch_names_what_to_repair() {
        let temp = tempfile::tempdir().unwrap();
        let registered = campaign(temp.path().into());
        CampaignDocument::load(&registered).unwrap().save().unwrap();
        let stale = campaign(temp.path().into());
        let error = CampaignDocument::load(&stale).err().unwrap();
        for detail in [
            stale.name,
            stale.id.to_string(),
            registered.id.to_string(),
            "Campaign.md".into(),
            temp.path().display().to_string(),
        ] {
            assert!(error.contains(&detail), "{detail} missing from: {error}");
        }
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
        assert!(
            doc.baseline.is_some(),
            "The fixture explicitly created this campaign."
        );
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
    fn a_built_in_that_names_itself_copies_and_the_copy_names_the_copy() {
        let catalogue = Catalogue::load(&[], None).unwrap();
        let temp = tempfile::tempdir().unwrap();
        // The entries a were-creature's curse, or a spell's own effect, makes refer to themselves.
        for key in [
            "creatures/werewolf",
            "creatures/wererat",
            "creatures/weretiger",
            "creatures/wereboar",
            "creatures/werebear",
            "creatures/ochre_jelly",
            "creatures/black_pudding",
            "creatures/shadow",
            "spells/foresight",
        ] {
            let key = format!("dnd2024:{key}");
            let indexed = catalogue
                .entries
                .values()
                .find(|e| e.entry.key.as_deref() == Some(key.as_str()))
                .unwrap_or_else(|| panic!("{key} is built in"));
            let name = &indexed.entry.name;
            let source = CopySource {
                entry: indexed.entry.clone(),
                text: catalogue.text_of(indexed.entry.id).unwrap(),
                root: None,
                campaign: None,
            };
            let plan = CopyPlan::prepare(
                &source,
                temp.path(),
                None,
                &format!("{name} copy"),
                "",
                &catalogue,
            )
            .unwrap_or_else(|e| panic!("{name}: {e}"));
            let copy = Entry::parse(&plan.relative, &plan.text).unwrap();
            assert_ne!(copy.id, indexed.entry.id);
            let own_key = indexed.entry.key.clone().unwrap();
            let references = content::required_references(&copy).unwrap();
            assert!(
                !references.iter().any(|reference| match reference {
                    Reference::Key(key) => *key == own_key,
                    Reference::Id(id) => *id == indexed.entry.id,
                }),
                "{name} still refers to the original"
            );
        }
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
    fn a_copy_keeps_an_independent_heading_and_the_rest_of_the_prose() {
        let text = note_text("Wolf", "rule", None).replace("# Wolf", "# Road notes")
            + "\nWolf tracks.\n\n# Wolf\n";
        let source = CopySource {
            entry: Entry::parse("Wolf.md", &text).unwrap(),
            text,
            root: None,
            campaign: None,
        };
        let folder = tempfile::tempdir().unwrap();
        let plan = CopyPlan::prepare(
            &source,
            folder.path(),
            None,
            "Watch Wolf",
            "",
            &Catalogue::default(),
        )
        .unwrap();
        assert!(plan.text.contains("# Road notes"));
        assert!(plan.text.contains("Wolf tracks.\n\n# Wolf\n"));
        assert!(!plan.text.contains("# Watch Wolf"));
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
        assert!(plan.text.contains("\n# Gate Guard\n"));
        assert!(!plan.text.contains("\n# Guard\n"));
        assert!(plan.text.contains("HP 11 (2d8 + 2)"));
        assert!(plan.text.contains(content::builtin::ATTRIBUTION));
        let all = cat.closure(
            &[PackageSelection {
                name: None,
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
                        name: None,
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

    #[test]
    fn the_campaign_package_is_private_and_listed_with_the_packages() {
        let root = tempfile::tempdir().unwrap();
        let id = Uuid::new_v4();
        let package = campaign_package::package_id(id);
        let folder = root.path().join("Campaign Package/Rules");
        fs::create_dir_all(&folder).unwrap();
        fs::write(
            root.path().join("Campaign.md"),
            format!("---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: campaign\nsplat_name: Test\nsplat_namespace: test_campaign\n---\n"),
        )
        .unwrap();
        fs::write(
            root.path().join("Campaign Package/Package.md"),
            format!("---\nsplat_schema: 1\nsplat_id: {package}\nsplat_kind: package\nsplat_name: Campaign Package\nsplat_namespace: test_campaign_package\n---\n"),
        )
        .unwrap();
        let rule = Uuid::new_v4();
        fs::write(
            folder.join("Rule.md"),
            format!("---\nsplat_schema: 1\nsplat_id: {rule}\nsplat_kind: rule\nsplat_name: Rule\nsplat_rule: {{}}\n---\n"),
        )
        .unwrap();
        let discovered = CampaignDocument::discover(root.path()).unwrap().unwrap();
        assert_eq!(discovered.id, id);
        let catalogue = Catalogue::load(&[], Some(&discovered)).unwrap();
        assert!(catalogue.errors.is_empty(), "{:?}", catalogue.errors);
        let entry = &catalogue.entries[&rule];
        assert!(entry.private);
        assert_eq!(entry.owner, package);
        assert_eq!(catalogue.package_name(package), "Campaign Package");
    }

    #[test]
    fn a_partial_selection_can_route_to_what_its_entries_require() {
        let root = tempfile::tempdir().unwrap();
        let (campaign_root, folder) = (root.path().join("campaign"), root.path().join("package"));
        fs::create_dir_all(&campaign_root).unwrap();
        fs::create_dir_all(&folder).unwrap();
        let (package, chosen, needed, other) = (
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        );
        fs::write(
            folder.join("Package.md"),
            format!("---\nsplat_schema: 1\nsplat_id: {package}\nsplat_kind: package\nsplat_name: P\nsplat_namespace: p\n---\n"),
        )
        .unwrap();
        let item = |id: Uuid, name: &str, extra: &str| {
            fs::write(
                folder.join(format!("{name}.md")),
                format!("---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: item\nsplat_name: {name}\nsplat_item:\n  category: gear\n{extra}---\n"),
            )
            .unwrap();
        };
        item(
            chosen,
            "Chosen",
            &format!("splat_requires:\n- id: {needed}\n"),
        );
        item(needed, "Needed", "splat_dndbeyond_ids:\n- item:needed\n");
        item(other, "Other", "splat_dndbeyond_ids:\n- item:other\n");
        fs::write(
            campaign_root.join("Campaign.md"),
            format!("---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: campaign\nsplat_name: C\nsplat_namespace: c\nsplat_packages:\n- package:\n    id: {package}\n  selection:\n    mode: selected\n    entries:\n    - id: {chosen}\n---\n", Uuid::new_v4()),
        )
        .unwrap();
        let modules = [Module {
            id: package,
            name: "P".into(),
            path: folder,
        }];
        let destinations = destinations(&campaign_root, &modules);
        assert!(destinations.lists("item:needed"));
        assert!(!destinations.lists("item:other"));
    }
    #[test]
    fn new_work_extends_partial_selections_and_keeps_whole_packages() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path().into());
        let package = Uuid::new_v4();
        let first = Uuid::new_v4();
        let second = Uuid::new_v4();
        use_in_campaign(&campaign, package, "Test package", Some(first)).unwrap();
        use_in_campaign(&campaign, package, "Test package", Some(second)).unwrap();
        use_in_campaign(&campaign, package, "Test package", Some(first)).unwrap();
        let document = CampaignDocument::load(&campaign).unwrap();
        let selection = document
            .selections
            .iter()
            .find(|s| s.package.id == Some(package))
            .unwrap();
        assert_eq!(selection.name.as_deref(), Some("Test package"));
        assert_eq!(
            selection.selection,
            Selection::Selected {
                entries: vec![EntryRef::id(first), EntryRef::id(second)]
            }
        );
        assert!(
            document
                .selections
                .iter()
                .any(|s| s.package.id == Some(builtin_manifest().id))
        );
        use_in_campaign(&campaign, package, "Test package", None).unwrap();
        use_in_campaign(&campaign, package, "Test package", Some(Uuid::new_v4())).unwrap();
        let document = CampaignDocument::load(&campaign).unwrap();
        assert_eq!(
            document
                .selections
                .iter()
                .find(|s| s.package.id == Some(package))
                .unwrap()
                .selection,
            Selection::All
        );
    }
    #[test]
    fn old_package_selections_load_and_missing_names_stay_out_of_identifiers() {
        let id = Uuid::new_v4();
        let old = format!("package: {{id: {id}}}\nselection: {{mode: all}}\n");
        let mut selection: PackageSelection = serde_yaml_ng::from_str(&old).unwrap();
        assert_eq!(selection.name, None);
        let message = selection.unavailable_message();
        assert!(message.starts_with("A package that is no longer on this computer."));
        assert!(!message.contains(&id.to_string()));
        selection.name = Some("Ashen Marches".into());
        let text = serde_yaml_ng::to_string(&selection).unwrap();
        assert_eq!(
            serde_yaml_ng::from_str::<PackageSelection>(&text).unwrap(),
            selection
        );
    }

    #[test]
    fn missing_package_counts_campaign_entries_once_not_individual_references() {
        let package = Uuid::new_v4();
        let needed = Uuid::new_v4();
        let selection = PackageSelection {
            name: Some("Ashen Marches".into()),
            package: EntryRef::id(package),
            selection: Selection::Selected {
                entries: vec![EntryRef::id(needed)],
            },
        };
        let mut catalogue = Catalogue::default();
        for number in 0..3 {
            let text = note_text(&format!("Entry {number}"), "rule", None);
            let text = patch_fields(
                &text,
                &[(
                    "splat_requires",
                    Some(
                        serde_yaml_ng::to_value(vec![EntryRef::id(needed), EntryRef::id(needed)])
                            .unwrap(),
                    ),
                )],
            )
            .unwrap();
            catalogue
                .insert(
                    Entry::parse(&format!("Entry {number}.md"), &text).unwrap(),
                    Uuid::nil(),
                    true,
                )
                .unwrap();
        }
        assert_eq!(
            selection.unavailable_message(),
            "Splat cannot find Ashen Marches."
        );
        assert_eq!(selection.campaign_users(&catalogue).len(), 3);
        assert!(
            catalogue
                .closure(&[selection], Uuid::nil())
                .errors
                .iter()
                .all(|error| !error.contains(&package.to_string()))
        );
    }
}
