//! Generated campaign content. Preflight every note before replacing any of the bundle.
use super::EntitySnapshot;
use crate::content::{Entry, Reference};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
const MARKER: &str = "\n<!-- splat-generated-sha256:";
const PERSONAL_NOTES: &str =
    "\n## Personal notes\n\nThis section is kept when the sheet refreshes.\n";

impl EntitySnapshot {
    pub fn campaign_path(&self, campaign: &Path) -> Result<PathBuf, String> {
        let storage =
            crate::campaign_storage::Storage::for_link(campaign, None, &self.source.campaign_id)?;
        Ok(
            crate::campaign_storage::relative_directory(campaign, &storage.characters)?
                .join(format!("{}.md", self.definition.splat_id)),
        )
    }
    pub fn save_campaign_copy(&self, campaign: &Path) -> Result<PathBuf, String> {
        self.validate()?;
        if !campaign.is_absolute() || !campaign.is_dir() {
            return Err("Campaign folder is unavailable; character copy was not saved.".into());
        }
        let destination = self.campaign_path(campaign)?;
        let root = destination
            .parent()
            .ok_or("Invalid character folder.")?
            .to_path_buf();
        let mut staged = Vec::new();
        for note in self.content_notes()? {
            let path = root.join(note.relative_path);
            let previous = match fs::read_to_string(&path) {
                Ok(text) => Some(text),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
                Err(_) => return Err(format!("Could not read campaign copy: {}", path.display())),
            };
            let notes = if let Some(text) = &previous {
                let (generated, tail) = text.rsplit_once(MARKER).ok_or_else(|| conflict(&path))?;
                let (hash, notes) = tail.split_once(" -->\n").ok_or_else(|| conflict(&path))?;
                if hash != format!("{:x}", Sha256::digest(generated.as_bytes())) {
                    return Err(conflict(&path));
                }
                notes
            } else {
                PERSONAL_NOTES
            };
            let hash = format!("{:x}", Sha256::digest(note.markdown.as_bytes()));
            let text = format!("{}{MARKER}{hash} -->\n{notes}", note.markdown);
            staged.push((path, previous, text));
        }
        let planned = staged
            .iter()
            .map(|(path, _, text)| Entry::parse(&path.to_string_lossy(), text))
            .collect::<Result<Vec<_>, _>>()?;
        let retired = retire_plan(self, campaign, &root, &planned)?;
        // Dependencies precede the creature. A filesystem failure may leave updated dependencies,
        // but never a new creature whose dependency files have not yet been published.
        for (path, previous, text) in staged {
            let parent = path.parent().ok_or("Invalid campaign copy path")?;
            fs::create_dir_all(parent)
                .map_err(|_| "Could not create campaign character folder.")?;
            let mut temp = tempfile::NamedTempFile::new_in(parent)
                .map_err(|_| "Could not stage campaign copy.")?;
            temp.write_all(text.as_bytes())
                .and_then(|_| temp.as_file().sync_all())
                .map_err(|_| "Could not write campaign copy.")?;
            if previous.is_some() {
                if fs::read_to_string(&path).ok() != previous {
                    return Err(conflict(&path));
                }
                temp.persist(&path)
                    .map_err(|_| "Could not replace campaign copy.")?;
            } else {
                temp.persist_noclobber(&path).map_err(|_| conflict(&path))?;
            }
        }
        // Only retire after the new creature has been published. Each archive preserves all bytes.
        for (path, previous) in retired {
            if fs::read_to_string(&path).ok().as_ref() != Some(&previous) {
                return Err(conflict(&path));
            }
            let archive = path.with_extension("md.splat-archive");
            // Do not overwrite an earlier archive if the source has drifted or been recreated.
            if archive.exists() {
                if fs::read_to_string(&archive).ok().as_ref() != Some(&previous) {
                    return Err(conflict(&archive));
                }
            } else {
                let mut temp =
                    tempfile::NamedTempFile::new_in(path.parent().ok_or("Invalid archive path")?)
                        .map_err(|e| e.to_string())?;
                temp.write_all(previous.as_bytes())
                    .and_then(|_| temp.as_file().sync_all())
                    .map_err(|e| e.to_string())?;
                temp.persist_noclobber(&archive)
                    .map_err(|e| e.to_string())?;
            }
            if fs::read_to_string(&path).ok().as_ref() != Some(&previous) {
                return Err(conflict(&path));
            }
            fs::remove_file(&path).map_err(|e| e.to_string())?;
        }
        Ok(destination)
    }
}

fn retire_plan(
    record: &EntitySnapshot,
    campaign: &Path,
    root: &Path,
    planned: &[Entry],
) -> Result<Vec<(PathBuf, String)>, String> {
    let mut candidates = BTreeMap::new();
    for relative in super::rules::retired_shared_paths(record)? {
        let path = root.join(relative);
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(e.to_string()),
        };
        let (generated, tail) = text.rsplit_once(MARKER).ok_or_else(|| conflict(&path))?;
        let (hash, personal) = tail.split_once(" -->\n").ok_or_else(|| conflict(&path))?;
        if hash != format!("{:x}", Sha256::digest(generated.as_bytes())) {
            return Err(conflict(&path));
        }
        // Keep annotated notes available at their original Markdown paths.
        if personal != PERSONAL_NOTES {
            continue;
        }
        let entry = Entry::parse(&path.to_string_lossy(), &text)?;
        if planned.iter().any(|e| e.id == entry.id) {
            continue;
        }
        let archive = path.with_extension("md.splat-archive");
        if archive.exists() && fs::read_to_string(&archive).ok().as_ref() != Some(&text) {
            return Err(conflict(&archive));
        }
        candidates.insert(entry.id, (path, text, entry));
    }
    if candidates.is_empty() {
        return Ok(Vec::new());
    }
    let (existing, errors) = crate::content::read_directory(campaign)?;
    if !errors.is_empty() {
        return Err(format!(
            "Could not check references before retiring imported SRD copies: {errors:?}"
        ));
    }
    let overwritten: BTreeSet<_> = planned.iter().map(|e| e.id).collect();
    let mut queue: Vec<_> = existing
        .iter()
        .filter(|e| !candidates.contains_key(&e.id) && !overwritten.contains(&e.id))
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
    Ok(candidates
        .into_iter()
        .filter(|(id, _)| !retained.contains(id))
        .map(|(_, (path, text, _))| (path, text))
        .collect())
}
fn conflict(path: &Path) -> String {
    format!(
        "Campaign character copy has local edits; kept unchanged: {}. Move it aside to resume generated updates.",
        path.display()
    )
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
        let generated = format!(
            "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_name: Old copied rule\nsplat_kind: rule\nsplat_rule: {{}}\nsplat_origin:\n  key: {key}\n---\n\nOld generated content.\n"
        );
        let hash = format!("{:x}", Sha256::digest(generated.as_bytes()));
        let text = format!("{generated}{MARKER}{hash} -->\n{PERSONAL_NOTES}{extra}");
        fs::write(&path, &text).unwrap();
        (path, text, id)
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
        fs::write(root.path().join("Custom.md"), format!("---\nsplat_schema: 1\nsplat_id: {}\nsplat_name: Custom rule\nsplat_kind: rule\nsplat_rule: {{}}\nsplat_requires:\n- id: {id}\n---\n", uuid::Uuid::new_v4())).unwrap();
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
