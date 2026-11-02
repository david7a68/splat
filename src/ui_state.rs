//! Where the app was when it closed: the page, the open run, and the participants
//! with sync on. This is convenience state, so a bad file is ignored, never fatal.
//! It lives beside the settings file. Test builds name a temporary file or none.
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OpenRun {
    pub campaign: Uuid,
    pub path: PathBuf,
    #[serde(default)]
    pub synced: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UiState {
    #[serde(default)]
    pub page: Option<String>,
    #[serde(default)]
    pub open_run: Option<OpenRun>,
    /// The session the DM chose to play, by campaign, when it is not the newest.
    #[serde(default)]
    pub playing: Option<(Uuid, Option<Uuid>)>,
    /// The widths the DM dragged columns to, by column.
    #[serde(default)]
    pub column_widths: BTreeMap<String, f32>,
}

#[derive(Default)]
pub struct Store {
    path: Option<PathBuf>,
    saved: UiState,
}

impl Store {
    #[cfg(not(test))]
    pub fn user() -> Self {
        let path = directories::ProjectDirs::from("", "", "splat")
            .map(|dirs| dirs.config_dir().join("ui-state.json"));
        Self {
            path,
            saved: UiState::default(),
        }
    }

    #[cfg(test)]
    pub fn user() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub fn at(path: PathBuf) -> Self {
        Self {
            path: Some(path),
            saved: UiState::default(),
        }
    }

    pub fn load(&mut self) -> UiState {
        self.saved = self
            .path
            .as_deref()
            .and_then(|path| fs::read(path).ok())
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default();
        self.saved.clone()
    }

    /// Write only when something changed. A failed write costs only the next restart.
    pub fn save(&mut self, state: &UiState) {
        if &self.saved == state {
            return;
        }
        self.saved = state.clone();
        if let Some(path) = &self.path {
            let _ = write(path, state);
        }
    }
}

fn write(path: &Path, state: &UiState) -> Result<(), String> {
    let parent = path.parent().ok_or("No parent folder")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    let bytes = serde_json::to_vec_pretty(state).map_err(|e| e.to_string())?;
    let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
    temp.write_all(&bytes).map_err(|e| e.to_string())?;
    temp.persist(path).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_round_trips_and_a_bad_file_means_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("ui-state.json");
        let mut store = Store::at(path.clone());
        assert_eq!(store.load(), UiState::default());
        let state = UiState {
            playing: None,
            page: Some("Run".into()),
            open_run: Some(OpenRun {
                campaign: Uuid::new_v4(),
                path: "/tmp/run.json".into(),
                synced: vec!["actor_hero_1".into()],
            }),
            column_widths: BTreeMap::from([("navigation".to_owned(), 240.0)]),
        };
        store.save(&state);
        assert_eq!(Store::at(path.clone()).load(), state);
        // A file from before columns could be dragged still opens.
        fs::write(&path, r#"{"page":"Party"}"#).unwrap();
        assert_eq!(
            Store::at(path.clone()).load().page.as_deref(),
            Some("Party")
        );
        fs::write(&path, "not json").unwrap();
        assert_eq!(Store::at(path).load(), UiState::default());
    }

    #[test]
    fn test_builds_have_no_user_state_file() {
        assert!(Store::user().path.is_none());
    }
}
