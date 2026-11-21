use splat_campaign::encounter_run::read_saved;
pub use splat_campaign::encounter_run::{FightRow, FightSummary, fight_summary};
pub use splat_content_ui::roster::{RunGroup, grouped_number};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub fn find_run(directory: &Path, campaign: Uuid, id: Uuid) -> Result<PathBuf, String> {
    find_run_typed(directory, campaign, id).map_err(|fault| match fault {
        FindRunFault::Folder(folder) => folder.message(),
        FindRunFault::File(words) => words,
    })
}

/// Why a run's file was not found: its folder (R41: missing or unreadable), or the folder
/// holds no single file for it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FindRunFault {
    Folder(RunFolderFault),
    File(String),
}

/// `find_run`, keeping the folder's cause typed, so a page says it as Encounters does.
pub fn find_run_typed(directory: &Path, campaign: Uuid, id: Uuid) -> Result<PathBuf, FindRunFault> {
    let folder = |fault| FindRunFault::Folder(fault);
    let unreadable = |error: std::io::Error| folder(RunFolderFault::unreadable(directory, &error));
    let mut found = Vec::new();
    for file in RunFolderFault::read(directory).map_err(folder)? {
        let path = file.map_err(unreadable)?.path();
        if path.extension().is_some_and(|ext| ext == "json")
            && read_saved::<()>(&path, campaign).is_ok_and(|run| run.id == id)
        {
            found.push(path);
        }
    }
    match found.as_slice() {
        [path] => Ok(path.clone()),
        [] => Err(FindRunFault::File(format!(
            "Run {id} was not found in {}.",
            directory.display()
        ))),
        _ => Err(FindRunFault::File(format!(
            "Several files use run {id}: {}. Keep one identity before carrying HP.",
            found
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ))),
    }
}

/// Where a saved run stands, as the Encounters page sorts it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Standing {
    Unfinished,
    Ended,
    /// The file is not a run this campaign can open. It can belong to another campaign that
    /// shares the folder, so the page offers nothing that would change it.
    Unreadable,
}

/// R41: why the campaign's run folder cannot be listed. This one value gives the footer
/// sentence, its tone, and what In progress and History say in place of their rows. The
/// Encounters snapshot holds it; the next list read that succeeds ends it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunFolderFault {
    /// The folder is not there: it may have been moved or renamed. A warning.
    Missing(PathBuf),
    /// The folder is there, and Splat cannot list it. An error. `cause` is plain words.
    Unreadable {
        folder: PathBuf,
        cause: &'static str,
    },
    /// The run folder setting names no folder Splat can use: its own complete sentence.
    Setting(String),
}

impl RunFolderFault {
    /// The one read of a run folder's files: a folder that is not there is `Missing`, any
    /// other failure `Unreadable`. `Run::list` and `find_run` both read through it.
    pub fn read(folder: &Path) -> Result<fs::ReadDir, Self> {
        match fs::metadata(folder) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                Err(Self::Missing(folder.to_path_buf()))
            }
            Err(error) => Err(Self::unreadable(folder, &error)),
            Ok(_) => fs::read_dir(folder).map_err(|error| Self::unreadable(folder, &error)),
        }
    }

    pub fn unreadable(folder: &Path, error: &std::io::Error) -> Self {
        let cause = match error.kind() {
            std::io::ErrorKind::PermissionDenied => "Splat has no permission to open it",
            std::io::ErrorKind::NotADirectory => "it, or a folder above it, is a file",
            _ => "the system did not let Splat list its files",
        };
        Self::Unreadable {
            folder: folder.to_path_buf(),
            cause,
        }
    }

    /// The footer's sentence: what is wrong, with the folder, and what the DM can do. The
    /// footer offers Choose folder beside it.
    pub fn message(&self) -> String {
        match self {
            Self::Missing(folder) => format!(
                "Splat cannot find the run folder {}. It may have been moved or renamed. Choose its new place, or put it back.",
                folder.display()
            ),
            Self::Unreadable { folder, cause } => format!(
                "Splat cannot read the run folder {}: {cause}. Fix it outside Splat, or choose another folder.",
                folder.display()
            ),
            Self::Setting(words) => words.clone(),
        }
    }

    /// The tone of `message`: a folder that is not there, or a setting, is a warning; a folder
    /// that cannot be read is an error.
    pub fn severity(&self) -> crate::design::outcome::Severity {
        match self {
            Self::Missing(_) | Self::Setting(_) => crate::design::outcome::Severity::NeedsFixing,
            Self::Unreadable { .. } => crate::design::outcome::Severity::CannotRead,
        }
    }

    /// The tone of `message` where a page paints it as a message: the tone of `severity`.
    pub fn tone(&self) -> crate::design::Tone {
        match self.severity() {
            crate::design::outcome::Severity::NeedsFixing => crate::design::Tone::Warning,
            crate::design::outcome::Severity::CannotRead
            | crate::design::outcome::Severity::NotSaved => crate::design::Tone::Error,
        }
    }

    /// What In progress (`ended` false) or History (`ended` true) says in place of its rows.
    pub fn list_words(&self, ended: bool) -> &'static str {
        match (self, ended) {
            (Self::Missing(_), false) => {
                "Splat cannot find the run folder, so it cannot show unfinished runs."
            }
            (Self::Missing(_), true) => {
                "Splat cannot find the run folder, so it cannot show ended fights."
            }
            (Self::Unreadable { .. }, false) => {
                "Splat cannot read the run folder, so it cannot show unfinished runs."
            }
            (Self::Unreadable { .. }, true) => {
                "Splat cannot read the run folder, so it cannot show ended fights."
            }
            (Self::Setting(_), false) => {
                "Splat has no run folder to read, so it cannot show unfinished runs."
            }
            (Self::Setting(_), true) => {
                "Splat has no run folder to read, so it cannot show ended fights."
            }
        }
    }
}

/// One saved run, as the Encounters page lists it.
#[derive(Clone)]
pub struct SavedRunInfo {
    pub path: PathBuf,
    pub id: Option<Uuid>,
    /// The encounter it came from, if the save says.
    pub source: Option<Uuid>,
    pub encounter: String,
    pub label: String,
    pub standing: Standing,
    /// The run is still in Setup: combat has not begun. A typed fact, so no decision reads
    /// the label's words (`Copy 2 · ` and `Changed on disk · ` come before the state).
    pub setup: bool,
    pub saved_at: Option<u64>,
    /// Who holds values not sent to D&D Beyond.
    pub unsent: Vec<String>,
}

/// A run file the DM deleted, kept in memory for one step of undo.
pub struct DeletedRun {
    path: PathBuf,
    bytes: Vec<u8>,
    saved_at: Option<std::time::SystemTime>,
    pub encounter: String,
    pub unsent: Vec<String>,
}

impl DeletedRun {
    /// One deletion warning on every page, including the way to keep unsent changes.
    pub fn message(&self) -> String {
        let name = &self.encounter;
        if self.unsent.is_empty() {
            format!("Deleted the saved run of “{name}”.")
        } else {
            format!(
                "Deleted the saved run of “{name}”. It held changes for {} that D&D Beyond does not have. Undo to keep them.",
                self.unsent.join(", ")
            )
        }
    }

    /// Remove the run's file.
    pub fn delete(info: &SavedRunInfo) -> Result<Self, String> {
        let bytes = fs::read(&info.path).map_err(|e| e.to_string())?;
        let saved_at = fs::metadata(&info.path)
            .and_then(|metadata| metadata.modified())
            .ok();
        fs::remove_file(&info.path).map_err(|e| e.to_string())?;
        Ok(Self {
            path: info.path.clone(),
            bytes,
            saved_at,
            encounter: info.encounter.clone(),
            unsent: info.unsent.clone(),
        })
    }
    /// Put the file back as it was. A file that has since appeared there stays.
    pub fn restore(&self) -> Result<(), String> {
        let parent = self.path.parent().ok_or("Invalid run location")?;
        let mut temp = tempfile::NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
        temp.write_all(&self.bytes)
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|e| e.to_string())?;
        if let Some(at) = self.saved_at {
            temp.as_file().set_modified(at).map_err(|e| e.to_string())?;
        }
        temp.persist_noclobber(&self.path)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// A run's read-aloud text and notes, and its setup's technical details, if it has them.
pub fn prep_parts(setup_details: &str) -> (&str, Option<&str>) {
    match setup_details.split_once(SETUP_HEADING) {
        Some((prep, technical)) => (prep.trim(), Some(technical.trim())),
        None => (setup_details.trim(), None),
    }
}

/// One vocabulary for every place a run's state is shown. Round zero means setup.
pub fn state_words(round: u64, paused: bool, ended: bool) -> String {
    if ended {
        format!("Ended · Round {}", round.max(1))
    } else {
        let state = if round == 0 {
            "Setup".to_owned()
        } else {
            format!("Round {round}")
        };
        if paused {
            format!("{state} · Paused")
        } else {
            state
        }
    }
}

/// Where a run's own text ends. Saved runs hold both parts in one string.
pub const SETUP_HEADING: &str = "## Setup (DM reference)";
