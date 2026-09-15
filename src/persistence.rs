use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, io::Write, path::PathBuf};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Campaign {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
    #[serde(default)]
    pub dndbeyond_campaign_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UserData {
    pub version: u32,
    pub active_campaign_id: Option<Uuid>,
    pub campaigns: Vec<Campaign>,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            version: 3,
            active_campaign_id: None,
            campaigns: Vec::new(),
        }
    }
}

impl UserData {
    fn validate(&self) -> Result<(), String> {
        if self.version != 3 {
            return Err(
                "This settings version is not supported. The file has not been changed.".into(),
            );
        }
        let mut ids = HashSet::new();
        let mut remote_ids = HashSet::new();
        for campaign in &self.campaigns {
            if campaign.id.is_nil()
                || !ids.insert(campaign.id)
                || campaign.name.trim().is_empty()
                || !campaign.path.is_absolute()
            {
                return Err(
                    "Each campaign needs a unique UUID, a name, and an absolute folder path."
                        .into(),
                );
            }
            if let Some(id) = &campaign.dndbeyond_campaign_id {
                let numeric = id
                    .parse::<u64>()
                    .ok()
                    .filter(|n| *n > 0)
                    .ok_or("D&D Beyond campaign IDs must be positive numbers.")?;
                if !remote_ids.insert(numeric) {
                    return Err("A D&D Beyond campaign can only be linked to one local campaign. Remove the duplicate link before saving.".into());
                }
            }
        }
        if self.active_campaign_id.is_some_and(|id| !ids.contains(&id)) {
            return Err("The selected campaign is not in the campaign list.".into());
        }
        Ok(())
    }
}

fn decode(bytes: &[u8]) -> Result<UserData, String> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct LegacyProject {
        id: Uuid,
        name: String,
        path: PathBuf,
        #[serde(default)]
        dndbeyond_campaign_ids: Vec<String>,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Legacy {
        version: u32,
        #[serde(default)]
        active_project_id: Option<Uuid>,
        #[serde(default)]
        active_project_ids: Vec<Uuid>,
        projects: Vec<LegacyProject>,
    }
    let value: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|_| "The settings file is invalid. It has not been changed.".to_owned())?;
    if matches!(value.get("version").and_then(|v| v.as_u64()), Some(1 | 2)) {
        let legacy: Legacy = serde_json::from_value(value)
            .map_err(|_| "The old settings file is invalid. It has not been changed.".to_owned())?;
        let mut campaigns = Vec::new();
        for project in legacy.projects {
            if project.dndbeyond_campaign_ids.len() > 1 {
                return Err("Old settings contain multiple D&D Beyond links for one campaign. Keep one ID per entry in settings.json, then reload. The file has not been changed.".into());
            }
            campaigns.push(Campaign {
                id: project.id,
                name: project.name,
                path: project.path,
                dndbeyond_campaign_id: project.dndbeyond_campaign_ids.into_iter().next(),
            });
        }
        Ok(UserData {
            version: 3,
            active_campaign_id: if legacy.version == 1 {
                legacy.active_project_id
            } else {
                legacy.active_project_ids.first().copied()
            },
            campaigns,
        })
    } else {
        serde_json::from_value(value)
            .map_err(|_| "The settings file is invalid. It has not been changed.".to_owned())
    }
}

pub struct Store {
    pub path: PathBuf,
    // A load error blocks writes. Saving also detects changes made by another instance.
    baseline: Option<Vec<u8>>,
    loaded: bool,
}

impl Store {
    pub fn user() -> Result<Self, String> {
        let dirs = ProjectDirs::from("", "", "splat")
            .ok_or("Could not find your user settings directory.")?;
        Ok(Self::at(dirs.config_dir().join("settings.json")))
    }

    pub(crate) fn at(path: PathBuf) -> Self {
        Self {
            path,
            baseline: None,
            loaded: false,
        }
    }

    fn read(&self) -> Result<Option<Vec<u8>>, String> {
        match fs::read(&self.path) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(_) => Err("Could not read settings. Check the file and its permissions.".into()),
        }
    }

    pub fn load(&mut self) -> Result<UserData, String> {
        self.loaded = false;
        let bytes = self.read()?;
        let data: UserData = match &bytes {
            Some(bytes) => decode(bytes)?,
            None => UserData::default(),
        };
        data.validate()?;
        self.baseline = bytes;
        self.loaded = true;
        Ok(data)
    }

    pub fn save(&mut self, data: &UserData) -> Result<(), String> {
        if !self.loaded {
            return Err("Reload a valid settings file before saving changes.".into());
        }
        data.validate()?;
        if self.read()? != self.baseline {
            return Err("Settings changed outside this window. Reload them before saving.".into());
        }
        let parent = self.path.parent().ok_or("Invalid settings directory.")?;
        fs::create_dir_all(parent).map_err(|_| "Could not create the settings directory.")?;
        let mut bytes =
            serde_json::to_vec_pretty(data).map_err(|_| "Could not encode settings.")?;
        bytes.push(b'\n');
        let mut temp = tempfile::NamedTempFile::new_in(parent)
            .map_err(|_| "Could not create a temporary settings file.")?;
        temp.write_all(&bytes)
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|_| "Could not write settings. Your previous file is unchanged.")?;
        temp.persist(&self.path)
            .map_err(|_| "Could not replace settings. Your previous file is unchanged.")?;
        self.baseline = Some(bytes);
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn campaign(id: Uuid, path: PathBuf, remote: Option<&str>) -> Campaign {
        Campaign {
            id,
            name: "Campaign".into(),
            path,
            dndbeyond_campaign_id: remote.map(str::to_owned),
        }
    }

    #[test]
    fn legacy_versions_preserve_ids_paths_and_unique_links_without_rewriting() {
        let dir = tempfile::tempdir().unwrap();
        for version in [1, 2] {
            let id = Uuid::new_v4();
            let mut old = serde_json::json!({"version":version,"projects":[{"id":id,"name":"Old","path":dir.path(),"dndbeyond_campaign_ids":["12"]}]});
            if version == 1 {
                old["active_project_id"] = serde_json::json!(id);
            } else {
                old["active_project_ids"] = serde_json::json!([id]);
            }
            let path = dir.path().join("settings.json");
            let bytes = serde_json::to_vec(&old).unwrap();
            fs::write(&path, &bytes).unwrap();
            let mut store = Store::at(path.clone());
            let data = store.load().unwrap();
            assert_eq!(data.version, 3);
            assert_eq!(data.active_campaign_id, Some(id));
            assert_eq!(data.campaigns[0].id, id);
            assert_eq!(data.campaigns[0].path, dir.path());
            assert_eq!(
                data.campaigns[0].dndbeyond_campaign_id.as_deref(),
                Some("12")
            );
            assert_eq!(fs::read(&path).unwrap(), bytes);
            store.save(&data).unwrap();
            let saved: serde_json::Value =
                serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            assert!(saved.get("projects").is_none());
            assert_eq!(Store::at(path).load().unwrap(), data);
        }
    }

    #[test]
    fn ambiguous_legacy_links_block_writes_and_preserve_file() {
        let dir = tempfile::tempdir().unwrap();
        for duplicate in [false, true] {
            let mut entries = vec![
                serde_json::json!({"id":Uuid::new_v4(),"name":"One","path":dir.path(),"dndbeyond_campaign_ids":if duplicate { vec!["12"] } else { vec!["12","34"] }}),
            ];
            if duplicate {
                entries.push(serde_json::json!({"id":Uuid::new_v4(),"name":"Two","path":dir.path(),"dndbeyond_campaign_ids":["12"]}));
            }
            let bytes = serde_json::to_vec(
                &serde_json::json!({"version":2,"active_project_ids":[],"projects":entries}),
            )
            .unwrap();
            let path = dir.path().join("settings.json");
            fs::write(&path, &bytes).unwrap();
            let mut store = Store::at(path.clone());
            assert!(store.load().is_err());
            assert!(store.save(&UserData::default()).is_err());
            assert_eq!(fs::read(path).unwrap(), bytes);
        }
    }

    #[test]
    fn invalid_and_future_settings_are_preserved() {
        let dir = tempfile::tempdir().unwrap();
        for content in [
            "broken",
            r#"{"version":4,"active_campaign_id":null,"campaigns":[]}"#,
            r#"{"version":3,"active_campaign_id":null,"campaigns":[],"future":true}"#,
        ] {
            let path = dir.path().join("settings.json");
            fs::write(&path, content).unwrap();
            let mut store = Store::at(path.clone());
            assert!(store.load().is_err());
            assert!(store.save(&UserData::default()).is_err());
            assert_eq!(fs::read_to_string(path).unwrap(), content);
        }
    }

    #[test]
    fn rejects_duplicate_remote_links_including_numeric_aliases() {
        let dir = tempfile::tempdir().unwrap();
        let mut data = UserData {
            campaigns: vec![
                campaign(Uuid::new_v4(), dir.path().into(), Some("12")),
                campaign(Uuid::new_v4(), dir.path().into(), Some("012")),
            ],
            ..Default::default()
        };
        assert!(data.validate().is_err());
        data.campaigns[1].dndbeyond_campaign_id = Some("34".into());
        assert!(data.validate().is_ok());
        data.active_campaign_id = Some(Uuid::new_v4());
        assert!(data.validate().is_err());
    }

    #[test]
    fn external_edits_are_preserved() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let mut store = Store::at(path.clone());
        assert!(store.load().unwrap().campaigns.is_empty());
        fs::write(&path, "external edit").unwrap();
        assert!(store.save(&UserData::default()).is_err());
        assert_eq!(fs::read_to_string(path).unwrap(), "external edit");
    }
}
