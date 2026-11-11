//! Generated campaign content. Preflight every note before replacing any of the bundle.
use super::EntitySnapshot;
use super::campaign_package::{self, Destinations};
use crate::content::{Entry, Reference, note_id};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::{
    fmt, fs,
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};
const MARKER: &str = "\n<!-- splat-generated-sha256:";
pub(super) const PERSONAL_NOTES: &str =
    "\n## Personal notes\n\nThis section is kept when the sheet refreshes.\n";
/// A file that changes during a save sends the save back to read it again. Another Splat writer
/// is the usual cause, and it finishes in moments.
const ATTEMPTS: usize = 8;

/// Why a campaign copy was not saved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CampaignCopyError {
    /// A note fails its checksum, so it has local edits. `overwrite_campaign_copy` can replace
    /// them. A file that only changed during the save is not a conflict.
    Conflict(PathBuf),
    Failed(String),
}
impl fmt::Display for CampaignCopyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conflict(path) => write!(
                f,
                "Campaign character copy has local edits; kept unchanged: {}. Move it aside to resume generated updates.",
                path.display()
            ),
            Self::Failed(message) => f.write_str(message),
        }
    }
}
impl std::error::Error for CampaignCopyError {}
impl From<String> for CampaignCopyError {
    fn from(message: String) -> Self {
        Self::Failed(message)
    }
}
impl From<&str> for CampaignCopyError {
    fn from(message: &str) -> Self {
        Self::Failed(message.to_owned())
    }
}
impl From<CampaignCopyError> for String {
    fn from(error: CampaignCopyError) -> Self {
        error.to_string()
    }
}

/// What a save does with a note that has local edits.
#[derive(Clone, Copy, PartialEq)]
enum OnEdits {
    Stop,
    Archive,
}
/// A note to publish. `edited` means its current bytes go to an archive first.
struct Note {
    path: PathBuf,
    previous: Option<String>,
    text: String,
    edited: bool,
}
/// A former copy, which has no new version.
struct Former {
    path: PathBuf,
    text: String,
    edited: bool,
    /// Other entries need it, so it stays, and its edits become its generated content.
    keep: bool,
}

impl EntitySnapshot {
    pub fn campaign_path(&self, campaign: &Path) -> PathBuf {
        campaign
            .join("Characters")
            .join("D&D Beyond")
            .join(super::content::character_filename(
                &self.definition.splat_name,
            ))
    }
    pub fn save_campaign_copy(&self, campaign: &Path) -> Result<PathBuf, CampaignCopyError> {
        self.save_campaign_copy_to(campaign, &Destinations::default())
    }
    /// Saves this copy, with its definitions routed to these destinations.
    pub fn save_campaign_copy_to(
        &self,
        campaign: &Path,
        destinations: &Destinations,
    ) -> Result<PathBuf, CampaignCopyError> {
        self.publish(campaign, destinations, OnEdits::Stop)?;
        self.current_campaign_path(campaign)
    }

    fn current_campaign_path(&self, campaign: &Path) -> Result<PathBuf, CampaignCopyError> {
        let (entries, errors) = crate::content::read_directory(campaign)?;
        let mut paths: Vec<_> = entries
            .iter()
            .filter(|entry| entry.id == self.definition.splat_id)
            .map(|entry| campaign.join(&entry.path))
            .collect();
        for error in errors {
            let path = campaign.join(&error.path);
            let Ok(text) = fs::read_to_string(&path) else {
                continue;
            };
            if !text.contains(&self.definition.splat_id.to_string()) {
                continue;
            }
            // Generated words that still match their checksum hold nothing of the DM's. When
            // the properties name this sheet, the refresh writes the copy again, whatever
            // Splat can read of it, and keeps the notes below it.
            if parts(&text).intact && note_id(&text) == Some(self.definition.splat_id) {
                paths.push(path);
                continue;
            }
            return Err(format!("{}: {}", path.display(), error.message).into());
        }
        match paths.len() {
            0 => {
                // The last path also protects a renamed sheet whose identity was
                // removed by an outside edit. Never infer that the file is gone.
                let remembered = self.path_record(campaign);
                if let Ok(relative) = fs::read_to_string(&remembered) {
                    let relative = Path::new(relative.trim());
                    if relative
                        .components()
                        .all(|c| matches!(c, std::path::Component::Normal(_)))
                    {
                        let path = campaign.join(relative);
                        if path.exists() {
                            return Ok(path);
                        }
                    } else {
                        return Err("Character path record needs repair.".into());
                    }
                }
                let legacy = campaign
                    .join("Characters/D&D Beyond")
                    .join(format!("{}.md", self.definition.splat_id));
                if legacy.exists() {
                    return Ok(legacy);
                }
                let named = self.campaign_path(campaign);
                if named.exists()
                    && fs::read_to_string(&named)
                        .ok()
                        .and_then(|text| note_id(&text))
                        .is_some_and(|id| id != self.definition.splat_id)
                {
                    return Ok(named.with_file_name(format!(
                        "{} {}.md",
                        named.file_stem().unwrap_or_default().to_string_lossy(),
                        &self.definition.splat_id.simple().to_string()[..8]
                    )));
                }
                Ok(named)
            }
            1 => Ok(paths.pop().unwrap()),
            _ => Err(format!(
                "Several character files share this identity: {}. Keep one before refreshing.",
                paths
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .into()),
        }
    }
    fn path_record(&self, campaign: &Path) -> PathBuf {
        campaign
            .join("Characters/D&D Beyond")
            .join(format!(".splat-path-{}", self.definition.splat_id))
    }
    /// Saves this copy over local edits, with the same checks and order as any save. The bytes
    /// it replaces go to an archive, and readable personal notes carry over. Returns the
    /// archives that hold edits.
    pub fn overwrite_campaign_copy(
        &self,
        campaign: &Path,
    ) -> Result<Vec<PathBuf>, CampaignCopyError> {
        self.overwrite_campaign_copy_to(campaign, &Destinations::default())
    }
    pub fn overwrite_campaign_copy_to(
        &self,
        campaign: &Path,
        destinations: &Destinations,
    ) -> Result<Vec<PathBuf>, CampaignCopyError> {
        self.publish(campaign, destinations, OnEdits::Archive)
    }
    fn publish(
        &self,
        campaign: &Path,
        destinations: &Destinations,
        on_edits: OnEdits,
    ) -> Result<Vec<PathBuf>, CampaignCopyError> {
        self.validate()?;
        if !campaign.is_absolute() || !campaign.is_dir() {
            return Err("Campaign folder is unavailable; character copy was not saved.".into());
        }
        let mut archived = Vec::new();
        for _ in 0..ATTEMPTS {
            if self.publish_once(campaign, destinations, on_edits, &mut archived)? {
                return Ok(archived);
            }
        }
        Err("A campaign character copy kept changing during the save. The next refresh tries again.".into())
    }
    /// False means that a file changed during this attempt, so nothing it read still holds.
    fn publish_once(
        &self,
        campaign: &Path,
        destinations: &Destinations,
        on_edits: OnEdits,
        archived: &mut Vec<PathBuf>,
    ) -> Result<bool, CampaignCopyError> {
        let _saving = campaign_package::saving();
        let root = campaign.join("Characters").join("D&D Beyond");
        let mut staged = Vec::new();
        let routed = campaign_package::route(campaign, destinations, self.content_notes()?)?;
        let character_path = self.current_campaign_path(campaign)?;
        for (mut path, markdown) in routed.notes {
            if path == self.campaign_path(campaign) {
                path = character_path.clone();
            }
            let previous = match fs::read_to_string(&path) {
                Ok(text) => Some(text),
                Err(e) if e.kind() == ErrorKind::NotFound => None,
                Err(_) => {
                    return Err(format!("Could not read campaign copy: {}", path.display()).into());
                }
            };
            let copy = previous.as_deref().map(parts);
            let occupant = previous.as_deref().and_then(note_id);
            if path == character_path && occupant.is_some_and(|id| id != self.definition.splat_id) {
                return Err(
                    "A different character already uses this file name. No file was replaced."
                        .into(),
                );
            }
            // A file that Splat cannot read is this note's generated copy only when its
            // properties name this note. Any other one is the DM's, whatever its checksum.
            let foreign = occupant != note_id(&markdown)
                && previous
                    .as_deref()
                    .is_some_and(|text| Entry::parse("note.md", text).is_err());
            let edited = foreign || copy.as_ref().is_some_and(|copy| !copy.intact);
            if edited && on_edits == OnEdits::Stop {
                return Err(conflict(&path));
            }
            let notes = copy.map_or(PERSONAL_NOTES, |copy| copy.notes);
            let text = stamped(&markdown, notes);
            staged.push(Note {
                path,
                previous,
                text,
                edited,
            });
        }
        let planned = staged
            .iter()
            .map(|note| Entry::parse(&note.path.to_string_lossy(), &note.text))
            .collect::<Result<Vec<_>, _>>()?;
        let formers = retire_plan(self, campaign, &root, &staged, &planned, on_edits)?;
        // A file that moved since it was read sends the save back to read it again.
        for write in &routed.shared {
            if fs::read_to_string(&write.path).ok().as_ref() != Some(&write.previous) {
                return Ok(false);
            }
            stage(&write.path, write.text.as_bytes())?
                .persist(&write.path)
                .map_err(|_| "Could not update a package note.")?;
        }
        if let Some((previous, text)) = &routed.manifest {
            let path = campaign.join(campaign_package::FOLDER).join("Package.md");
            if fs::read_to_string(&path).ok() != *previous {
                return Ok(false);
            }
            stage(&path, text.as_bytes())?
                .persist(&path)
                .map_err(|_| "Could not save the campaign package.")?;
        }
        // Dependencies precede the creature. A filesystem failure may leave updated dependencies,
        // but never a new creature whose dependency files have not yet been published.
        for note in staged {
            // Rewriting the same words costs an fsync per note, and changes the file's time.
            // A later read of the same sheet differs only in when it was read.
            if note
                .previous
                .as_deref()
                .is_some_and(|previous| same_but_read_time(previous, &note.text))
            {
                if fs::read_to_string(&note.path).ok() != note.previous {
                    return Ok(false);
                }
                continue;
            }
            let temp = stage(&note.path, note.text.as_bytes())?;
            if let Some(previous) = &note.previous {
                if fs::read_to_string(&note.path).ok().as_ref() != Some(previous) {
                    return Ok(false);
                }
                if note.edited {
                    report(archived, archive_copy(&note.path, previous)?);
                }
                temp.persist(&note.path)
                    .map_err(|_| "Could not replace campaign copy.")?;
            } else {
                match temp.persist_noclobber(&note.path) {
                    Ok(_) => {}
                    Err(e) if e.error.kind() == ErrorKind::AlreadyExists => return Ok(false),
                    Err(_) => return Err("Could not save campaign copy.".into()),
                }
            }
        }
        // Only retire after the new creature has been published. Each archive preserves all bytes.
        for former in formers {
            if fs::read_to_string(&former.path).ok().as_ref() != Some(&former.text) {
                return Ok(false);
            }
            if former.keep {
                let copy = parts(&former.text);
                stage(&former.path, stamped(copy.generated, copy.notes).as_bytes())?
                    .persist(&former.path)
                    .map_err(|_| "Could not replace campaign copy.")?;
                continue;
            }
            let archive = archive_copy(&former.path, &former.text)?;
            if former.edited {
                report(archived, archive);
            }
            if fs::read_to_string(&former.path).ok().as_ref() != Some(&former.text) {
                return Ok(false);
            }
            fs::remove_file(&former.path).map_err(|e| e.to_string())?;
        }
        let relative = character_path
            .strip_prefix(campaign)
            .map_err(|e| e.to_string())?
            .to_string_lossy();
        let record = self.path_record(campaign);
        stage(&record, relative.as_bytes())?
            .persist(&record)
            .map_err(|e| e.to_string())?;
        Ok(true)
    }
}

fn retire_plan(
    record: &EntitySnapshot,
    campaign: &Path,
    root: &Path,
    staged: &[Note],
    planned: &[Entry],
    on_edits: OnEdits,
) -> Result<Vec<Former>, CampaignCopyError> {
    let mut formers = Vec::new();
    let mut candidates = BTreeMap::new();
    let mut retired: BTreeSet<_> = super::rules::retired_shared_paths(record, root)?
        .into_iter()
        .collect();
    retired.extend(campaign_package::former_paths(root, record));
    for relative in retired {
        let path = root.join(relative);
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(e) if e.kind() == ErrorKind::NotFound => continue,
            Err(e) => return Err(e.to_string().into()),
        };
        let copy = parts(&text);
        let edited = !copy.intact;
        if edited && on_edits == OnEdits::Stop {
            return Err(conflict(&path));
        }
        // Publishing replaces a note at this path, and archives its edits.
        if edited && staged.iter().any(|note| note.path == path) {
            continue;
        }
        let annotated = copy.notes != PERSONAL_NOTES;
        // Keep annotated notes available at their original Markdown paths.
        if annotated && !edited {
            continue;
        }
        // A refresh does not add a second archive beside one with other words.
        let other_archive = || {
            let archive = path.with_extension("md.splat-archive");
            on_edits == OnEdits::Stop
                && archive.exists()
                && fs::read_to_string(&archive).ok().as_ref() != Some(&text)
        };
        let entry = match Entry::parse(&path.to_string_lossy(), &text) {
            Ok(entry) => entry,
            // No entry can need words that Splat cannot read. The archive keeps them.
            Err(_) => {
                if !edited && other_archive() {
                    return Err(conflict(&path.with_extension("md.splat-archive")));
                }
                formers.push(Former {
                    path,
                    text,
                    edited,
                    keep: false,
                });
                continue;
            }
        };
        if planned.iter().any(|e| e.id == entry.id) {
            if edited {
                formers.push(Former {
                    path,
                    text,
                    edited,
                    keep: false,
                });
            }
            continue;
        }
        if annotated {
            formers.push(Former {
                path,
                text,
                edited,
                keep: true,
            });
            continue;
        }
        if other_archive() {
            return Err(conflict(&path.with_extension("md.splat-archive")));
        }
        candidates.insert(entry.id, (path, text, entry));
    }
    if candidates.is_empty() {
        return Ok(formers);
    }
    // These files leave during this save, so what they say does not stop it or keep anything.
    let leaving: BTreeSet<PathBuf> = staged
        .iter()
        .filter(|note| note.edited)
        .map(|note| note.path.clone())
        .chain(formers.iter().map(|former| former.path.clone()))
        .collect();
    let (existing, errors) = crate::content::read_directory(campaign)?;
    // A staged note gets its new words in this save, so what it holds now cannot stop it:
    // one that Splat cannot read is this note's own copy, or it is archived as an edit.
    let errors: Vec<_> = errors
        .into_iter()
        .filter(|error| {
            let path = campaign.join(&error.path);
            !leaving.contains(&path) && !staged.iter().any(|note| note.path == path)
        })
        .collect();
    if !errors.is_empty() {
        return Err(format!(
            "Could not check references before retiring imported SRD copies: {errors:?}"
        )
        .into());
    }
    let overwritten: BTreeSet<_> = planned.iter().map(|e| e.id).collect();
    let mut queue: Vec<_> = existing
        .iter()
        .filter(|e| {
            !candidates.contains_key(&e.id)
                && !overwritten.contains(&e.id)
                && !leaving.contains(&campaign.join(&e.path))
        })
        .chain(planned)
        .collect();
    let mut retained = BTreeSet::new();
    while let Some(entry) = queue.pop() {
        for reference in crate::content::required_references(entry)? {
            if let Reference::Id(id) = reference
                && let Some((_, _, dependency)) = candidates.get(&id)
                && retained.insert(id)
            {
                queue.push(dependency);
            }
        }
    }
    for (id, (path, text, _)) in candidates {
        let edited = !parts(&text).intact;
        let needed = retained.contains(&id);
        // A needed copy without edits stays as it is.
        if !needed || edited {
            formers.push(Former {
                path,
                text,
                edited,
                keep: needed,
            });
        }
    }
    Ok(formers)
}
fn conflict(path: &Path) -> CampaignCopyError {
    CampaignCopyError::Conflict(path.to_path_buf())
}
/// A note's generated words and personal notes, and whether its checksum holds. A note with no
/// readable checksum is all generated words, with the first personal notes.
pub(super) struct Parts<'a> {
    pub generated: &'a str,
    pub notes: &'a str,
    pub intact: bool,
}
pub(super) fn parts(text: &str) -> Parts<'_> {
    if let Some((generated, tail)) = text.rsplit_once(MARKER)
        && let Some((hash, notes)) = tail.split_once(" -->\n")
    {
        return Parts {
            generated,
            notes,
            intact: hash == checksum(generated),
        };
    }
    Parts {
        generated: text,
        notes: PERSONAL_NOTES,
        intact: false,
    }
}
/// Whether two copies hold the same sheet and notes, whenever each was read.
fn same_but_read_time(previous: &str, next: &str) -> bool {
    let facts = |text: &'_ str| {
        let copy = parts(text);
        let lines: Vec<String> = copy
            .generated
            .lines()
            .filter(|line| {
                let line = line.trim_start().trim_start_matches('>').trim_start();
                !line.starts_with("fetched_at:") && !line.starts_with("raw_fetched_at:")
            })
            .map(str::to_owned)
            .collect();
        (copy.intact, lines, copy.notes.to_owned())
    };
    previous == next || facts(previous) == facts(next)
}
fn checksum(generated: &str) -> String {
    format!("{:x}", Sha256::digest(generated.as_bytes()))
}
pub(super) fn stamped(generated: &str, notes: &str) -> String {
    format!("{generated}{MARKER}{} -->\n{notes}", checksum(generated))
}
/// A temporary file beside `path`, holding `bytes` on disk, ready to persist.
pub(super) fn stage(
    path: &Path,
    bytes: &[u8],
) -> Result<tempfile::NamedTempFile, CampaignCopyError> {
    let parent = path.parent().ok_or("Invalid campaign copy path")?;
    fs::create_dir_all(parent).map_err(|_| "Could not create campaign character folder.")?;
    let mut temp =
        tempfile::NamedTempFile::new_in(parent).map_err(|_| "Could not stage campaign copy.")?;
    temp.write_all(bytes)
        .and_then(|_| temp.as_file().sync_all())
        .map_err(|_| "Could not write campaign copy.")?;
    Ok(temp)
}
/// Keeps `text` in the first archive name that holds it or is unused, and never overwrites an
/// archive. These names do not end in `.md`, so the content reader skips them.
pub(super) fn archive_copy(path: &Path, text: &str) -> Result<PathBuf, CampaignCopyError> {
    let base = path.with_extension("md.splat-archive");
    let mut temp = None;
    for archive in std::iter::once(base.clone())
        .chain((1u32..).map(|n| PathBuf::from(format!("{}.{n}", base.display()))))
    {
        match fs::read(&archive) {
            Ok(bytes) if bytes == text.as_bytes() => return Ok(archive),
            Ok(_) => continue,
            Err(e) if e.kind() == ErrorKind::NotFound => {}
            Err(e) => return Err(e.to_string().into()),
        }
        let staged = match temp.take() {
            Some(staged) => staged,
            None => stage(path, text.as_bytes())?,
        };
        match staged.persist_noclobber(&archive) {
            Ok(_) => return Ok(archive),
            Err(e) if e.error.kind() == ErrorKind::AlreadyExists => temp = Some(e.file),
            Err(e) => return Err(e.to_string().into()),
        }
    }
    Err("Could not choose an archive name.".into())
}
fn report(archived: &mut Vec<PathBuf>, archive: PathBuf) {
    if !archived.contains(&archive) {
        archived.push(archive);
    }
}

#[cfg(test)]
mod tests {
    use super::super::content::reference_id;
    use super::*;
    fn old_copy(
        record: &EntitySnapshot,
        campaign: &Path,
        key: &str,
        extra: &str,
    ) -> (PathBuf, String, uuid::Uuid) {
        let id = reference_id(record, "rule", key);
        let path = campaign.join(format!(
            "Characters/D&D Beyond/Definitions/{}/Rules/{id}.md",
            record.definition.splat_id
        ));
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let generated = crate::content::formats::fixture("Fixture.md", &format!("splat_id: {id}\nsplat_name: Old copied rule\nsplat_kind: rule\nsplat_rule: {{}}\nsplat_origin:\n  key: {key}\n"), "\nOld generated content.\n").expect("schema-2 fixture");
        let hash = format!("{:x}", Sha256::digest(generated.as_bytes()));
        let text = format!("{generated}{MARKER}{hash} -->\n{PERSONAL_NOTES}{extra}");
        fs::write(&path, &text).unwrap();
        (path, text, id)
    }
    fn routed_notes(record: &EntitySnapshot, campaign: &Path) -> Vec<(PathBuf, String)> {
        let notes = record.content_notes().unwrap();
        campaign_package::route(campaign, &Destinations::default(), notes)
            .unwrap()
            .notes
    }
    fn dependency_path(record: &EntitySnapshot, campaign: &Path) -> PathBuf {
        routed_notes(record, campaign).remove(0).0
    }
    fn custom_entry_requiring(campaign: &Path, id: uuid::Uuid) {
        fs::write(campaign.join("Custom.md"), crate::content::formats::fixture("Fixture.md", &format!("splat_id: {}\nsplat_name: Custom rule\nsplat_kind: rule\nsplat_rule: {{}}\nsplat_requires:\n- id: {id}\n", uuid::Uuid::new_v4()), "").expect("schema-2 fixture")).unwrap();
    }
    #[test]
    fn retires_only_unreferenced_unannotated_copies_and_preserves_all_bytes() {
        let root = tempfile::tempdir().unwrap();
        let record = super::super::tests::sample();
        let (retire, old, _) = old_copy(&record, root.path(), "dnd2024:spells/fire_bolt", "");
        let (annotated, notes, _) = old_copy(
            &record,
            root.path(),
            "dnd2024:spells/acid_splash",
            "My annotations.\n",
        );
        let (referenced, original, id) =
            old_copy(&record, root.path(), "dnd2024:spells/shield", "");
        custom_entry_requiring(root.path(), id);
        record.save_campaign_copy(root.path()).unwrap();
        assert!(!retire.exists());
        assert_eq!(
            fs::read_to_string(retire.with_extension("md.splat-archive")).unwrap(),
            old
        );
        assert_eq!(fs::read_to_string(&annotated).unwrap(), notes);
        assert_eq!(fs::read_to_string(&referenced).unwrap(), original);
        record.save_campaign_copy(root.path()).unwrap();
        assert_eq!(
            fs::read_to_string(retire.with_extension("md.splat-archive")).unwrap(),
            old
        );
    }
    #[cfg(unix)]
    #[test]
    fn unchanged_copy_keeps_its_file() {
        use std::os::unix::fs::MetadataExt;
        let root = tempfile::tempdir().unwrap();
        let record = super::super::tests::sample();
        let creature = record.save_campaign_copy(root.path()).unwrap();
        let inode = fs::metadata(&creature).unwrap().ino();
        record.save_campaign_copy(root.path()).unwrap();
        assert_eq!(fs::metadata(&creature).unwrap().ino(), inode);
    }
    #[test]
    fn a_later_read_of_the_same_sheet_keeps_its_file_and_a_change_replaces_it() {
        use std::os::unix::fs::MetadataExt;
        let root = tempfile::tempdir().unwrap();
        let mut record = super::super::tests::sample();
        let creature = record.save_campaign_copy(root.path()).unwrap();
        let inode = fs::metadata(&creature).unwrap().ino();
        record.source.fetched_at += 100;
        record.source.raw_fetched_at += 100;
        record.save_campaign_copy(root.path()).unwrap();
        assert_eq!(fs::metadata(&creature).unwrap().ino(), inode);
        record.initial.hp -= 1;
        record.save_campaign_copy(root.path()).unwrap();
        assert_ne!(fs::metadata(&creature).unwrap().ino(), inode);
    }
    #[test]
    fn overwrite_replaces_edits_archives_them_and_leaves_other_notes() {
        let root = tempfile::tempdir().unwrap();
        let mut record = super::super::tests::sample();
        let creature = record.save_campaign_copy(root.path()).unwrap();
        let dependency = dependency_path(&record, root.path());
        let annotated = fs::read_to_string(&dependency).unwrap() + "My own notes.\n";
        fs::write(&dependency, &annotated).unwrap();
        fs::write(&creature, "An unmanaged local note").unwrap();
        record.definition.splat_name = "New name".into();

        assert_eq!(
            record.save_campaign_copy(root.path()),
            Err(CampaignCopyError::Conflict(creature.clone()))
        );
        let archive = creature.with_extension("md.splat-archive");
        assert_eq!(
            record.overwrite_campaign_copy(root.path()).unwrap(),
            std::slice::from_ref(&archive)
        );

        let replaced = fs::read_to_string(&creature).unwrap();
        assert!(replaced.contains("New name"));
        assert!(replaced.ends_with(PERSONAL_NOTES));
        assert_eq!(
            fs::read_to_string(&archive).unwrap(),
            "An unmanaged local note"
        );
        assert!(
            fs::read_to_string(&dependency)
                .unwrap()
                .ends_with("My own notes.\n")
        );
        assert!(
            !dependency.with_extension("md.splat-archive").exists(),
            "A note without edits has nothing to archive"
        );
        // The next ordinary refresh has nothing left to object to.
        record.save_campaign_copy(root.path()).unwrap();
    }

    /// A generated copy in a layout that Splat does not read.
    fn unreadable_copy(id: uuid::Uuid, intact: bool, notes: &str) -> String {
        let generated = format!(
            "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: creature\nsplat_name: Copy\n---\n\nSheet.\n"
        );
        let hash = if intact {
            checksum(&generated)
        } else {
            "0".repeat(64)
        };
        format!("{generated}{MARKER}{hash} -->\n{PERSONAL_NOTES}{notes}")
    }

    #[test]
    fn an_unreadable_copy_is_written_again_only_while_its_generated_words_are_unchanged() {
        // Splat reads only schema 2. A copy it cannot read is still generated text while its
        // checksum holds, so a refresh writes it again. Former copies do not stop that: a
        // readable one retires as usual, and one that Splat cannot read goes to its archive.
        let root = tempfile::tempdir().unwrap();
        let mut record = super::super::tests::sample();
        let id = record.definition.splat_id;
        let creature = record.save_campaign_copy(root.path()).unwrap();
        fs::write(&creature, unreadable_copy(id, true, "Kept note.\n")).unwrap();
        let (readable, readable_text, _) =
            old_copy(&record, root.path(), "dnd2024:spells/fire_bolt", "");
        let former = readable.with_file_name(format!("{}.md", uuid::Uuid::new_v4()));
        let former_text = unreadable_copy(uuid::Uuid::new_v4(), true, "");
        fs::write(&former, &former_text).unwrap();
        record.definition.splat_name = "Refreshed".into();
        assert_eq!(record.save_campaign_copy(root.path()).unwrap(), creature);
        let refreshed = fs::read_to_string(&creature).unwrap();
        assert_eq!(
            Entry::parse("sheet.md", &refreshed).unwrap().name,
            "Refreshed"
        );
        assert!(refreshed.ends_with("Kept note.\n"));
        assert!(!creature.with_extension("md.splat-archive").exists());
        for (path, text) in [(readable, readable_text), (former, former_text)] {
            assert!(!path.exists(), "{}", path.display());
            assert_eq!(
                fs::read_to_string(path.with_extension("md.splat-archive")).unwrap(),
                text
            );
        }

        // A copy of another note where this character's rule goes is the DM's file too.
        let rule = dependency_path(&record, root.path());
        let foreign = unreadable_copy(uuid::Uuid::new_v4(), true, "");
        fs::write(&rule, &foreign).unwrap();
        assert_eq!(
            record.save_campaign_copy(root.path()),
            Err(CampaignCopyError::Conflict(rule.clone()))
        );
        assert_eq!(fs::read_to_string(&rule).unwrap(), foreign);

        // With a change in it, the copy is the DM's file. So is a copy whose properties name
        // another character, whatever its notes say. A refresh or Replace names the file and
        // changes nothing.
        let another = unreadable_copy(uuid::Uuid::new_v4(), true, &format!("splat_id: {id}\n"));
        for kept in [unreadable_copy(id, false, "Kept note.\n"), another] {
            fs::write(&creature, &kept).unwrap();
            let expected = format!(
                "{}: Unsupported content schema; expected 2",
                creature.display()
            );
            assert_eq!(
                record.save_campaign_copy(root.path()),
                Err(expected.clone().into())
            );
            assert_eq!(
                record.overwrite_campaign_copy(root.path()),
                Err(expected.into())
            );
            assert_eq!(fs::read_to_string(&creature).unwrap(), kept);
            assert!(!creature.with_extension("md.splat-archive").exists());
        }
    }

    #[test]
    fn fmt_f1_character_names_do_not_replace_another_character() {
        let record = super::super::tests::sample();
        let readable = crate::content::formats::fixture("Other.md", "splat_id: 00000000-0000-0000-0000-000000000099\nsplat_kind: creature\nsplat_name: Example\nsplat_creature: {}\n", "\nA different character's note.\n").unwrap();
        // The properties name the other character: also in a copy that Splat cannot read, and
        // with a property that looks like their end.
        let odd = readable.replacen("splat_id:", "---custom: kept\nsplat_id:", 1);
        assert!(Entry::parse("Other.md", &odd).is_ok());
        let unreadable = unreadable_copy(uuid::Uuid::new_v4(), true, "");
        for other in [readable, odd, unreadable.clone()] {
            let root = tempfile::tempdir().unwrap();
            let path = record.campaign_path(root.path());
            assert_eq!(path.file_name().unwrap(), "Example.md");
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, &other).unwrap();
            let saved = record.save_campaign_copy(root.path()).unwrap();
            assert_ne!(saved, path);
            assert_eq!(fs::read_to_string(path).unwrap(), other);
        }
        // A file with this character's ID as its name is checked the same way.
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join(format!(
            "Characters/D&D Beyond/{}.md",
            record.definition.splat_id
        ));
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, &unreadable).unwrap();
        assert!(record.save_campaign_copy(root.path()).is_err());
        assert_eq!(fs::read_to_string(path).unwrap(), unreadable);
    }
    #[test]
    fn overwrite_carries_readable_personal_notes_over() {
        let root = tempfile::tempdir().unwrap();
        let record = super::super::tests::sample();
        let creature = record.save_campaign_copy(root.path()).unwrap();
        let original = fs::read_to_string(&creature).unwrap();
        let edited = format!("---\nEdited: yes\n{}My own notes.\n", &original[4..]);
        fs::write(&creature, &edited).unwrap();

        record.overwrite_campaign_copy(root.path()).unwrap();

        let replaced = fs::read_to_string(&creature).unwrap();
        assert_eq!(replaced, original.clone() + "My own notes.\n");
        assert_eq!(
            fs::read_to_string(creature.with_extension("md.splat-archive")).unwrap(),
            edited
        );
    }
    #[test]
    fn overwrite_clears_every_conflict_in_the_bundle_and_reports_each_archive() {
        let root = tempfile::tempdir().unwrap();
        let record = super::super::tests::sample();
        record.save_campaign_copy(root.path()).unwrap();
        let notes = routed_notes(&record, root.path());
        for (path, _) in &notes {
            fs::write(path, "Edited note").unwrap();
        }
        let archived = record.overwrite_campaign_copy(root.path()).unwrap();
        assert_eq!(archived.len(), notes.len());
        for (path, markdown) in &notes {
            assert!(fs::read_to_string(path).unwrap().starts_with(markdown));
        }
        record.save_campaign_copy(root.path()).unwrap();
    }
    #[test]
    fn overwrite_that_fails_its_checks_changes_nothing() {
        let root = tempfile::tempdir().unwrap();
        let mut record = super::super::tests::sample();
        let creature = record.save_campaign_copy(root.path()).unwrap();
        let dependency = dependency_path(&record, root.path());
        fs::remove_file(&dependency).unwrap();
        fs::write(&creature, "My edited creature").unwrap();
        // A retired copy needs a reference check, and this other note stops that check.
        old_copy(&record, root.path(), "dnd2024:spells/fire_bolt", "");
        fs::write(
            root.path().join("Broken.md"),
            "---\nsplat_schema: 1\nsplat_id: nope\n---\n",
        )
        .unwrap();
        record.definition.splat_name = "New name".into();

        assert!(matches!(
            record.overwrite_campaign_copy(root.path()),
            Err(CampaignCopyError::Failed(_))
        ));

        assert_eq!(fs::read_to_string(&creature).unwrap(), "My edited creature");
        assert!(!creature.with_extension("md.splat-archive").exists());
        assert!(!dependency.exists());
    }
    #[test]
    fn overwrite_gets_past_an_edited_note_that_no_longer_parses() {
        let root = tempfile::tempdir().unwrap();
        let record = super::super::tests::sample();
        let creature = record.save_campaign_copy(root.path()).unwrap();
        // Frontmatter that the reader tries to parse, and cannot.
        fs::write(&creature, "---\nsplat_schema: 2\nsplat_id: nope\n---\n").unwrap();
        // A retired copy makes the save read the whole campaign.
        let (retired, _, _) = old_copy(&record, root.path(), "dnd2024:spells/fire_bolt", "");
        record.overwrite_campaign_copy(root.path()).unwrap();
        assert!(!retired.exists());
        record.save_campaign_copy(root.path()).unwrap();
    }
    #[test]
    fn overwrite_archives_edited_former_copies_and_keeps_older_archives() {
        let root = tempfile::tempdir().unwrap();
        let record = super::super::tests::sample();
        let (edited, old, _) = old_copy(&record, root.path(), "dnd2024:spells/fire_bolt", "");
        let changed = old.replace("Old generated content.", "User rule changes.");
        fs::write(&edited, &changed).unwrap();
        let (blocked, blocked_text, _) =
            old_copy(&record, root.path(), "dnd2024:spells/acid_splash", "");
        let older = blocked.with_extension("md.splat-archive");
        fs::write(&older, "An older archive").unwrap();

        let archived = record.overwrite_campaign_copy(root.path()).unwrap();

        assert_eq!(archived, [edited.with_extension("md.splat-archive")]);
        assert!(!edited.exists());
        assert_eq!(fs::read_to_string(&archived[0]).unwrap(), changed);
        assert!(!blocked.exists());
        assert_eq!(fs::read_to_string(&older).unwrap(), "An older archive");
        let next = format!("{}.1", older.display());
        assert_eq!(fs::read_to_string(next).unwrap(), blocked_text);
        let (_, errors) = crate::content::read_directory(root.path()).unwrap();
        assert!(errors.is_empty(), "{errors:?}");
    }
    #[test]
    fn overwrite_keeps_an_edited_former_copy_that_another_entry_needs() {
        let root = tempfile::tempdir().unwrap();
        let record = super::super::tests::sample();
        let (needed, old, id) = old_copy(&record, root.path(), "dnd2024:spells/shield", "");
        fs::write(
            &needed,
            old.replace("Old generated content.", "User rule changes."),
        )
        .unwrap();
        custom_entry_requiring(root.path(), id);

        assert!(
            record
                .overwrite_campaign_copy(root.path())
                .unwrap()
                .is_empty()
        );

        assert!(
            fs::read_to_string(&needed)
                .unwrap()
                .contains("User rule changes.")
        );
        // Its edits are its content now, so they no longer stop a save.
        record.save_campaign_copy(root.path()).unwrap();
    }
    #[test]
    fn a_second_overwrite_of_the_same_edits_adds_no_archive() {
        let root = tempfile::tempdir().unwrap();
        let record = super::super::tests::sample();
        let creature = record.save_campaign_copy(root.path()).unwrap();
        fs::write(&creature, "Edited").unwrap();
        record.overwrite_campaign_copy(root.path()).unwrap();
        fs::write(&creature, "Edited").unwrap();
        let archived = record.overwrite_campaign_copy(root.path()).unwrap();
        assert_eq!(archived, [creature.with_extension("md.splat-archive")]);
        assert!(!PathBuf::from(format!("{}.1", archived[0].display())).exists());
    }
    /// Sync and a refresh can save one character at the same time. Neither is a local edit.
    #[test]
    fn splats_own_concurrent_saves_are_not_conflicts_and_keep_personal_notes() {
        let root = tempfile::tempdir().unwrap();
        let mut one = super::super::tests::sample();
        let creature = one.save_campaign_copy(root.path()).unwrap();
        let annotated = fs::read_to_string(&creature).unwrap() + "My personal notes.\n";
        fs::write(&creature, annotated).unwrap();
        let mut two = one.clone();
        one.initial.temporary_hp = 1;
        two.initial.temporary_hp = 2;
        let start = |record: EntitySnapshot, force: bool, campaign: PathBuf| {
            std::thread::spawn(move || {
                (0..40)
                    .filter(|_| {
                        let saved = if force {
                            record.overwrite_campaign_copy(&campaign).map(|_| ())
                        } else {
                            record.save_campaign_copy(&campaign).map(|_| ())
                        };
                        matches!(saved, Err(CampaignCopyError::Conflict(_)))
                    })
                    .count()
            })
        };
        for force in [false, true] {
            let first = start(one.clone(), force, root.path().to_path_buf());
            let second = start(two.clone(), false, root.path().to_path_buf());
            assert_eq!(first.join().unwrap() + second.join().unwrap(), 0);
        }
        assert!(
            fs::read_to_string(&creature)
                .unwrap()
                .ends_with("My personal notes.\n")
        );
        assert!(!creature.with_extension("md.splat-archive").exists());
    }
    #[test]
    fn edited_retired_copy_blocks_publication_before_any_file_changes() {
        let root = tempfile::tempdir().unwrap();
        let mut record = super::super::tests::sample();
        let creature = record.save_campaign_copy(root.path()).unwrap();
        let original = fs::read_to_string(&creature).unwrap();
        let (path, old, _) = old_copy(&record, root.path(), "dnd2024:spells/fire_bolt", "");
        let edited = old.replace("Old generated content.", "User rule changes.");
        fs::write(&path, &edited).unwrap();
        record.definition.splat_name = "New name".into();
        assert!(record.save_campaign_copy(root.path()).is_err());
        assert_eq!(fs::read_to_string(&creature).unwrap(), original);
        assert_eq!(fs::read_to_string(&path).unwrap(), edited);
        assert!(!path.with_extension("md.splat-archive").exists());
    }
}
