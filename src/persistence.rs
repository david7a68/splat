use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fs, io::Write, path::PathBuf};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub path: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UserData {
    pub version: u32,
    pub active_project_id: Option<Uuid>,
    pub projects: Vec<Project>,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            version: 1,
            active_project_id: None,
            projects: Vec::new(),
        }
    }
}

impl UserData {
    fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err(
                "This settings version is not supported. The file has not been changed.".into(),
            );
        }
        let mut ids = HashSet::new();
        for project in &self.projects {
            if project.id.is_nil()
                || !ids.insert(project.id)
                || project.name.trim().is_empty()
                || !project.path.is_absolute()
            {
                return Err(
                    "Each project needs a unique UUID, a name, and an absolute directory path."
                        .into(),
                );
            }
        }
        if self.active_project_id.is_some_and(|id| !ids.contains(&id)) {
            return Err("The selected project is not in the project list.".into());
        }
        Ok(())
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

    fn at(path: PathBuf) -> Self {
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
            Some(bytes) => serde_json::from_slice(bytes)
                .map_err(|_| "The settings file is invalid. It has not been changed.".to_owned())?,
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

    #[test]
    fn round_trip_keeps_uuid_when_name_or_path_changes() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = Store::at(dir.path().join("settings.json"));
        let mut data = store.load().unwrap();
        let id = Uuid::new_v4();
        data.projects.push(Project {
            id,
            name: "Campaign".into(),
            path: dir.path().join("missing"),
        });
        data.active_project_id = Some(id);
        store.save(&data).unwrap();
        data.projects[0].name = "Renamed".into();
        data.projects[0].path = dir.path().join("moved");
        store.save(&data).unwrap();
        assert_eq!(Store::at(store.path.clone()).load().unwrap(), data);
        assert_eq!(data.projects[0].id, id);
    }

    #[test]
    fn invalid_and_future_files_are_never_overwritten() {
        let dir = tempfile::tempdir().unwrap();
        for content in [
            "broken",
            r#"{"version":2,"projects":[],"active_project_id":null}"#,
            r#"{"version":1,"projects":[],"active_project_id":null,"future":true}"#,
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
    fn external_edit_is_preserved() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("settings.json");
        let mut first = Store::at(path.clone());
        first.load().unwrap();
        fs::write(&path, "external edit").unwrap();
        assert!(first.save(&UserData::default()).is_err());
        assert_eq!(fs::read_to_string(path).unwrap(), "external edit");
    }

    #[test]
    fn rejects_duplicate_ids_and_dangling_selection() {
        let dir = tempfile::tempdir().unwrap();
        let project = Project {
            id: Uuid::new_v4(),
            name: "One".into(),
            path: dir.path().into(),
        };
        let mut data = UserData {
            projects: vec![project.clone(), project],
            ..Default::default()
        };
        assert!(data.validate().is_err());
        data.projects.pop();
        data.active_project_id = Some(Uuid::new_v4());
        assert!(data.validate().is_err());
    }
}
