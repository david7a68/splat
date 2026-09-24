use super::{EntitySnapshot, VERSION};
use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};
use uuid::Uuid;
const MAX_CACHE: u64 = 32 * 1024 * 1024;
#[derive(Clone)]
pub struct Cache {
    root: PathBuf,
}
impl Cache {
    pub fn user() -> Result<Self, String> {
        let dirs = directories::ProjectDirs::from("", "", "splat")
            .ok_or("Could not locate local application data.")?;
        Ok(Self::at(dirs.data_local_dir().join("characters")))
    }
    /// Use portable storage, copying the old application cache only on first use.
    pub fn campaign(root: &Path, local: Uuid, remote: &str) -> Result<Self, String> {
        let storage = crate::campaign_storage::Storage::for_link(root, Some(local), remote)?;
        let cache = Self::at(crate::campaign_storage::relative_directory(
            root,
            &storage.character_cache,
        )?);
        let legacy = Self::user()?;
        cache.migrate_from(&legacy, local, remote)?;
        Ok(cache)
    }
    pub fn for_campaign(root: &Path) -> Result<Self, String> {
        let storage = crate::campaign_storage::Storage::load(root)?;
        Ok(Self::at(crate::campaign_storage::relative_directory(
            root,
            &storage.character_cache,
        )?))
    }
    pub fn migrate_from(&self, legacy: &Self, local: Uuid, remote: &str) -> Result<(), String> {
        let destination = self.directory(local, remote)?;
        let source = legacy.directory(local, remote)?;
        if destination.exists() || !source.is_dir() {
            return Ok(());
        }
        // Validate before copying; retain the original cache for recovery.
        legacy.load_all(local, remote)?;
        let parent = destination.parent().ok_or("Invalid cache folder.")?;
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        let staging = tempfile::tempdir_in(parent).map_err(|e| e.to_string())?;
        for item in fs::read_dir(source).map_err(|e| e.to_string())? {
            let item = item.map_err(|e| e.to_string())?;
            if item.file_type().map_err(|e| e.to_string())?.is_file() {
                fs::copy(item.path(), staging.path().join(item.file_name()))
                    .map_err(|e| e.to_string())?;
            }
        }
        if !destination.exists() {
            fs::rename(staging.path(), &destination).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
    pub fn at(root: PathBuf) -> Self {
        Self { root }
    }
    pub fn directory(&self, campaign: Uuid, remote: &str) -> Result<PathBuf, String> {
        valid_id(remote)?;
        Ok(self.root.join(campaign.to_string()).join(remote))
    }
    pub fn path(&self, campaign: Uuid, remote: &str, character: &str) -> Result<PathBuf, String> {
        valid_id(character)?;
        Ok(self
            .directory(campaign, remote)?
            .join(format!("{character}.json")))
    }
    pub fn save(&self, campaign: Uuid, record: &EntitySnapshot) -> Result<PathBuf, String> {
        record.validate()?;
        let path = self.path(
            campaign,
            &record.source.campaign_id,
            &record.source.character_id,
        )?;
        let parent = path.parent().ok_or("Invalid character cache path.")?;
        fs::create_dir_all(parent)
            .map_err(|_| "Could not create the character cache directory.")?;
        let bytes = serde_json::to_vec_pretty(record)
            .map_err(|_| "Could not encode the character cache.")?;
        if bytes.len() as u64 > MAX_CACHE {
            return Err("The character cache record is too large.".into());
        }
        // A version the app cannot read must not be silently replaced.
        if path.exists() {
            let existing = read(&path)?;
            if existing.version != VERSION {
                return Err("Unsupported existing character cache; it was not replaced.".into());
            }
        }
        let mut temp = tempfile::NamedTempFile::new_in(parent)
            .map_err(|_| "Could not stage the character cache.")?;
        temp.write_all(&bytes)
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|_| "Could not write the character cache.")?;
        temp.persist(&path)
            .map_err(|_| "Could not replace the character cache.")?;
        Ok(path)
    }
    pub fn save_roster(&self, campaign: Uuid, remote: &str, ids: &[String]) -> Result<(), String> {
        for id in ids {
            valid_id(id)?;
        }
        let dir = self.directory(campaign, remote)?;
        fs::create_dir_all(&dir).map_err(|_| "Could not create roster cache.")?;
        let mut temp =
            tempfile::NamedTempFile::new_in(&dir).map_err(|_| "Could not stage roster cache.")?;
        serde_json::to_writer(temp.as_file_mut(), ids)
            .map_err(|_| "Could not encode roster cache.")?;
        temp.as_file()
            .sync_all()
            .map_err(|_| "Could not write roster cache.")?;
        temp.persist(dir.join("roster-index"))
            .map_err(|_| "Could not replace roster cache.")?;
        Ok(())
    }
    pub fn load_all(&self, campaign: Uuid, remote: &str) -> Result<Vec<EntitySnapshot>, String> {
        let dir = self.directory(campaign, remote)?;
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let index = dir.join("roster-index");
        let ids: Option<Vec<String>> = if index.exists() {
            let bytes = fs::read(index).map_err(|_| "Could not read roster cache.")?;
            if bytes.len() > 65536 {
                return Err("Roster cache is too large.".into());
            }
            Some(serde_json::from_slice(&bytes).map_err(|_| "Invalid roster cache.")?)
        } else {
            None
        };
        let mut records = Vec::new();
        for item in
            fs::read_dir(dir).map_err(|_| "Could not read the character cache directory.")?
        {
            let path = item
                .map_err(|_| "Could not list character cache files.")?
                .path();
            if path.extension().is_some_and(|s| s == "json") {
                if ids.as_ref().is_some_and(|ids| {
                    !ids.iter()
                        .any(|id| path.file_stem().and_then(|s| s.to_str()) == Some(id))
                }) {
                    continue;
                }
                let record = read(&path)?;
                if record.source.campaign_id != remote
                    || path.file_stem().and_then(|s| s.to_str())
                        != Some(&record.source.character_id)
                {
                    return Err("Cached character identity does not match its path.".into());
                }
                records.push(record);
            }
        }
        records.sort_by(|a, b| a.definition.splat_name.cmp(&b.definition.splat_name));
        Ok(records)
    }
}
fn valid_id(id: &str) -> Result<(), String> {
    if id.parse::<u64>().ok().is_none_or(|v| v == 0) || !id.bytes().all(|b| b.is_ascii_digit()) {
        Err("Invalid D&D Beyond ID.".into())
    } else {
        Ok(())
    }
}
fn read(path: &Path) -> Result<EntitySnapshot, String> {
    let file = fs::File::open(path).map_err(|_| "Could not read a cached character.")?;
    let mut bytes = Vec::new();
    file.take(MAX_CACHE + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Could not read a cached character.")?;
    if bytes.len() as u64 > MAX_CACHE {
        return Err("The cached character is too large.".into());
    }
    let record: EntitySnapshot = serde_json::from_slice(&bytes)
        .map_err(|_| "Invalid or unsupported character cache. The file was kept.")?;
    record.validate()?;
    Ok(record)
}

#[cfg(test)]
mod portable_tests {
    use super::*;
    #[test]
    fn migration_preserves_roster_and_never_replaces_local_data() {
        let old = tempfile::tempdir().unwrap();
        let new = tempfile::tempdir().unwrap();
        let legacy = Cache::at(old.path().into());
        let portable = Cache::at(new.path().into());
        let local = Uuid::new_v4();
        legacy.save_roster(local, "12", &[]).unwrap();
        portable.migrate_from(&legacy, local, "12").unwrap();
        let target = portable
            .directory(local, "12")
            .unwrap()
            .join("roster-index");
        assert_eq!(fs::read_to_string(&target).unwrap(), "[]");
        assert!(
            legacy
                .directory(local, "12")
                .unwrap()
                .join("roster-index")
                .exists()
        );
        fs::write(&target, "[\"42\"]").unwrap();
        portable.migrate_from(&legacy, local, "12").unwrap();
        assert_eq!(fs::read_to_string(target).unwrap(), "[\"42\"]");
    }
}
