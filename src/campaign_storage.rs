//! Portable campaign settings and paths. Reading never creates files.
use crate::content::{self, Entry};
use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Identity {
    pub version: u32,
    pub dndbeyond_campaign_id: Option<String>,
    pub dndbeyond_campaign_name: Option<String>,
}
impl Identity {
    pub fn read(entry: &Entry) -> Result<Option<Self>, String> {
        let Some(value) = entry.fields.get("splat_campaign") else {
            return Ok(None);
        };
        let mut identity: Self = serde_yaml_ng::from_value(value.clone())
            .map_err(|e| format!("Invalid campaign settings: {e}"))?;
        if identity.version != 1 {
            return Err("Unsupported campaign settings version. The file was kept.".into());
        }
        if let Some(id) = &mut identity.dndbeyond_campaign_id {
            *id = id
                .parse::<u64>()
                .ok()
                .filter(|id| *id > 0)
                .ok_or("Invalid D&D Beyond campaign ID.")?
                .to_string();
        }
        Ok(Some(identity))
    }
}

pub fn discover(root: &Path) -> Result<Option<Entry>, String> {
    let (entries, errors) = content::read_directory(root)?;
    if !errors.is_empty() {
        return Err(format!(
            "Repair invalid campaign notes before opening this folder: {}",
            errors
                .iter()
                .map(|e| format!("{}: {}", e.path, e.message))
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }
    if entries.iter().any(|e| e.kind == "package") {
        return Err("A campaign folder cannot contain a package manifest.".into());
    }
    let mut manifests = entries.into_iter().filter(|e| e.kind == "campaign");
    let manifest = manifests.next();
    if manifests.next().is_some() {
        return Err("The folder contains more than one campaign manifest.".into());
    }
    if let Some(entry) = &manifest {
        let namespace = entry
            .fields
            .get("splat_namespace")
            .and_then(|v| v.as_str())
            .ok_or("The campaign manifest needs a namespace.")?;
        if namespace == "dnd2024" || !content::resource_key(&format!("{namespace}:campaign")) {
            return Err("Use a valid campaign namespace other than dnd2024.".into());
        }
        Identity::read(entry)?;
        Storage::from_entry(entry)?.validate(root)?;
    }
    Ok(manifest)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Storage {
    pub characters: String,
    pub character_cache: String,
}
impl Default for Storage {
    fn default() -> Self {
        Self {
            characters: "Characters/D&D Beyond".into(),
            character_cache: ".splat/characters".into(),
        }
    }
}
impl Storage {
    pub fn from_entry(entry: &Entry) -> Result<Self, String> {
        entry
            .fields
            .get("splat_storage")
            .map(|v| {
                serde_yaml_ng::from_value(v.clone())
                    .map_err(|e| format!("Invalid campaign paths: {e}"))
            })
            .unwrap_or_else(|| Ok(Self::default()))
    }
    pub fn load(root: &Path) -> Result<Self, String> {
        discover(root)?
            .as_ref()
            .map(Self::from_entry)
            .unwrap_or_else(|| Ok(Self::default()))
    }
    pub fn for_link(root: &Path, local: Option<uuid::Uuid>, remote: &str) -> Result<Self, String> {
        let Some(entry) = discover(root)? else {
            return Ok(Self::default());
        };
        if local.is_some_and(|id| id != entry.id) {
            return Err(
                "The campaign folder now has a different identity. Reload campaign settings."
                    .into(),
            );
        }
        if let Some(identity) = Identity::read(&entry)?
            && identity.dndbeyond_campaign_id.as_deref() != Some(remote)
        {
            return Err("The campaign link changed on disk. Reload campaign settings.".into());
        }
        Self::from_entry(&entry)
    }
    pub fn validate(&self, root: &Path) -> Result<(), String> {
        relative_directory(root, &self.characters)?;
        relative_directory(root, &self.character_cache)?;
        Ok(())
    }
}
pub fn relative_directory(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let path = Path::new(relative);
    if relative.trim().is_empty()
        || path
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err("Use a relative data folder inside the campaign, without '..'.".into());
    }
    if !root.is_dir() {
        return Err("The campaign folder is unavailable.".into());
    }
    let mut result = root.to_path_buf();
    for component in path.components() {
        result.push(component);
        if std::fs::symlink_metadata(&result).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err("Campaign data folders cannot use symbolic links.".into());
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paths_are_portable_and_cannot_escape() {
        let root = tempfile::tempdir().unwrap();
        assert_eq!(
            relative_directory(root.path(), "Party/Sheets").unwrap(),
            root.path().join("Party/Sheets")
        );
        for path in ["", "../outside", "/tmp/outside", "Party/../../outside"] {
            assert!(relative_directory(root.path(), path).is_err());
        }
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("/tmp", root.path().join("link")).unwrap();
            assert!(relative_directory(root.path(), "link/Sheets").is_err());
        }
        assert!(!root.path().join("Party").exists());
    }
}
