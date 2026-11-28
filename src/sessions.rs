//! Session notes and templates, as plain Markdown in the campaign's own folder, so that
//! Obsidian can read and edit them too. A note's frontmatter avoids the `splat_` prefix on
//! purpose: the catalogue treats any such note as a Library entry.
#[path = "sessions/page.rs"]
pub mod page;
#[path = "sessions/slash.rs"]
mod slash;
/// The `/` menu's rules, for the GPUI note.
pub use slash::plain as slash_plain;

use crate::{
    content_workspace::{CampaignDocument, patch_fields},
    library::write_note,
    persistence::Campaign,
};
use serde_yaml_ng::Value;
use splat::content::Entry;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::Instant,
};
use uuid::Uuid;

#[cfg(any(test, not(feature = "test-support")))]
const SAVE_AFTER: std::time::Duration = std::time::Duration::from_secs(2);

/// `open_folder_typed` with the fault's words, for the fixtures. The page keeps the fault typed.
#[cfg(any(test, feature = "test-support"))]
pub fn open_folder(campaign: &Campaign) -> Result<PathBuf, String> {
    open_folder_typed(campaign).map_err(String::from)
}

/// `open_folder` with its fault typed: the page shows it in its footer and on its page.
pub(crate) fn open_folder_typed(campaign: &Campaign) -> Result<PathBuf, page::FolderFault> {
    let document = CampaignDocument::load(campaign)?;
    let entry = Entry::parse(&document.path, &document.text)?;
    if !entry.fields.contains_key(FOLDER_FIELD) {
        write_folder(campaign, &folder_setting(campaign)?).map_err(page::FolderFault::NotSaved)
    } else {
        folder_typed(campaign)
    }
}

pub fn save_folder(campaign: &Campaign, name: &str) -> Result<PathBuf, String> {
    write_folder(campaign, name)
}
fn write_folder(campaign: &Campaign, name: &str) -> Result<PathBuf, String> {
    let path = resolve(campaign, name)?;
    fs::create_dir_all(&path).map_err(|error| not_created(&error))?;
    let document = CampaignDocument::load(campaign)?;
    let mut map = serde_yaml_ng::Mapping::new();
    map.insert("directory".into(), Value::String(name.trim().to_owned()));
    let text = splat::content::formats::patch_model(
        &document.text,
        &[(FOLDER_FIELD, Some(Value::Mapping(map)))],
    )?;
    write_note(
        &campaign.path,
        &document.path,
        document.baseline.as_deref(),
        &text,
    )?;
    Ok(path)
}

use splat_campaign::note::first_number;
pub use splat_content_ui::session_notes::*;

pub struct IdentityUndo {
    path: PathBuf,
    before: String,
    after: String,
}

pub fn new_identity(path: &Path) -> Result<IdentityUndo, String> {
    let before = fs::read_to_string(path).map_err(|error| not_read(&error))?;
    let after = patch_fields(
        &before,
        &[(
            "session_id",
            Some(crate::hermetic::new_uuid().to_string().into()),
        )],
    )?;
    if fs::read_to_string(path).ok().as_deref() != Some(before.as_str()) {
        return Err("This session changed on disk. Reload before trying again.".into());
    }
    replace_file_io(path, &after).map_err(|error| not_saved(&error))?;
    Ok(IdentityUndo {
        path: path.into(),
        before,
        after,
    })
}

impl IdentityUndo {
    /// The copy's file name, which its result names.
    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    }

    pub fn undo(&self) -> Result<(), String> {
        if fs::read_to_string(&self.path).ok().as_deref() != Some(self.after.as_str()) {
            return Err("This session changed on disk. Undo did not replace it.".into());
        }
        replace_file_io(&self.path, &self.before).map_err(|error| not_saved(&error))
    }
}

pub(crate) fn remember_notes(folder: &Path, notes: &[Note]) -> Result<(), String> {
    let path = folder.join(NOTE_PATHS);
    let mut known = known_notes(folder);
    for note in notes {
        known.insert(
            note.id,
            note.path
                .strip_prefix(folder)
                .map_err(|e| e.to_string())?
                .into(),
        );
    }
    let text = serde_json::to_string_pretty(&known).map_err(|e| e.to_string())?;
    // A scan that found nothing new writes nothing: an idle campaign's files keep their times.
    if fs::read(&path).is_ok_and(|bytes| bytes == text.as_bytes()) {
        return Ok(());
    }
    replace_file_io(&path, &text).map_err(|error| not_saved(&error))
}

/// Notes that claim a session but cannot be read, by file name. Their files are safe.
pub fn unreadable_notes(folder: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(folder)
        .into_iter()
        .flatten()
        .flatten()
        .map(|file| file.path())
        .filter(|path| path.extension().is_some_and(|e| e == "md"))
        .filter(|path| fs::read_to_string(path).is_ok_and(|text| unreadable_note(path, &text)))
        .filter_map(|path| Some(path.file_name()?.to_string_lossy().into_owned()))
        .collect();
    names.sort();
    names
}

/// Whether a file's words claim a session whose properties cannot be read.
fn unreadable_note(path: &Path, text: &str) -> bool {
    text.lines().any(|line| line.starts_with("session_id:")) && parse_note(path, text).is_none()
}

fn free_path(folder: &Path, stem: &str, extension: &str) -> PathBuf {
    let first = folder.join(format!("{stem}.{extension}"));
    if !first.exists() {
        return first;
    }
    (2..)
        .map(|n| folder.join(format!("{stem} ({n}).{extension}")))
        .find(|path| !path.exists())
        .expect("a free name")
}

/// What a new note needs, before its template is filled.
pub struct NewNote<'a> {
    pub number: u32,
    pub title: &'a str,
    pub date: &'a str,
    pub campaign: &'a str,
    pub template: &'a str,
}

/// Fill a template's placeholders. An unknown one stays, in case it belongs to Obsidian.
pub fn fill_template(new: &NewNote<'_>) -> String {
    let filled = new
        .template
        .replace("{{number}}", &new.number.to_string())
        .replace("{{title}}", new.title.trim())
        .replace("{{date}}", new.date)
        .replace("{{campaign}}", new.campaign);
    // An empty title must not leave `# Session 12: ` behind.
    filled
        .split_inclusive('\n')
        .map(|line| {
            let bare = line.trim_end();
            match bare.strip_suffix(':') {
                Some(kept) if bare.starts_with('#') => {
                    format!(
                        "{kept}{}",
                        &line[line.trim_end_matches(['\n', '\r']).len()..]
                    )
                }
                _ => line.to_owned(),
            }
        })
        .collect()
}

/// The heading a template makes for a session.
fn session_heading(number: u32, title: &str) -> String {
    match title.trim() {
        "" => format!("# Session {number}"),
        title => format!("# Session {number}: {title}"),
    }
}

pub fn note_path(folder: &Path, number: u32, title: &str) -> PathBuf {
    let mut stem = format!("Session {number}");
    let title = file_safe(title);
    if !title.is_empty() {
        stem.push_str(" - ");
        stem.push_str(&title);
    }
    free_path(folder, &stem, "md")
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Scans of a session folder by `storage_files`, counted inside the scan.
    static STORAGE_SCANS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// A scan before a storage move that the system refused, in plain words from the error's kind
/// (round 30 B7e): the file or folder by its name, and the recovery. Never the system's words.
fn scan_fault(path: &Path, error: &std::io::Error) -> String {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    format!(
        "Not moved. {}",
        crate::world::file_fault::FileFault::read(&name, error).message()
    )
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// A test's refused scan: reading the session folder fails with this system error code.
    pub(crate) static FAILING_SCAN: std::cell::Cell<Option<i32>> = const { std::cell::Cell::new(None) };
}

/// Files owned by session storage. Other campaign notes stay where they are.
pub fn storage_files(folder: &Path) -> Result<Vec<PathBuf>, String> {
    #[cfg(any(test, feature = "test-support"))]
    STORAGE_SCANS.with(|scans| scans.set(scans.get() + 1));
    #[cfg(any(test, feature = "test-support"))]
    if let Some(code) = FAILING_SCAN.with(std::cell::Cell::get) {
        return Err(scan_fault(folder, &std::io::Error::from_raw_os_error(code)));
    }
    fn files(root: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
        if !root.exists() {
            return Ok(());
        }
        for item in fs::read_dir(root).map_err(|error| scan_fault(root, &error))? {
            let item = item.map_err(|error| scan_fault(root, &error))?;
            let path = item.path();
            if item
                .file_type()
                .map_err(|error| scan_fault(&path, &error))?
                .is_symlink()
            {
                return Err(format!(
                    "{} is a link to another file or folder. Move it by hand first.",
                    path.display()
                ));
            }
            if path.is_dir() {
                files(&path, out)?;
            } else {
                out.push(path);
            }
        }
        Ok(())
    }
    let mut out: Vec<_> = list_notes(folder)
        .into_iter()
        .flat_map(|note| std::iter::once(note.path).chain(note.also))
        .collect();
    if folder.join("Timeline.md").exists() {
        out.push(folder.join("Timeline.md"));
    }
    if folder.join(NOTE_PATHS).exists() {
        out.push(folder.join(NOTE_PATHS));
    }
    for file in fs::read_dir(folder).map_err(|error| scan_fault(folder, &error))? {
        let path = file.map_err(|error| scan_fault(folder, &error))?.path();
        if path.extension().is_some_and(|ext| ext == "md") && !out.contains(&path) {
            let text = fs::read_to_string(&path).map_err(|error| scan_fault(&path, &error))?;
            if text.lines().any(|line| {
                line.starts_with("session_id:") || line.starts_with("timeline_version:")
            }) {
                out.push(path);
            }
        }
    }
    files(&folder.join(TEMPLATES), &mut out)?;
    files(&folder.join(RECORDINGS), &mut out)?;
    Ok(out)
}

/// Copy before changing the manifest; retain the old files if any step fails.
/// The files of a session folder's storage by kind: what a Move files and save would move, or
/// moved. Counted from one listing (`storage_files`), never from the page's own lists.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct StorageCounts {
    pub files: usize,
    pub notes: usize,
    pub templates: usize,
    pub recordings: usize,
}

impl StorageCounts {
    pub fn of(folder: &Path, files: &[PathBuf]) -> Self {
        let templates = folder.join(TEMPLATES);
        let recordings = folder.join(RECORDINGS);
        let kept = [folder.join("Timeline.md"), folder.join(NOTE_PATHS)];
        let count = |holds: &dyn Fn(&PathBuf) -> bool| files.iter().filter(|p| holds(p)).count();
        Self {
            files: files.len(),
            notes: count(&|path| {
                path.parent() == Some(folder)
                    && path.extension().is_some_and(|ext| ext == "md")
                    && !kept.contains(path)
            }),
            templates: count(&|path| path.starts_with(&templates)),
            recordings: count(&|path| {
                path.starts_with(&recordings) && path.extension().is_some_and(|ext| ext == "wav")
            }),
        }
    }
}

/// Moves the session storage to the folder `name`, and says what it moved: the counts of the
/// files it listed and moved at this act.
pub(crate) fn move_storage(
    campaign: &Campaign,
    name: &str,
) -> Result<(PathBuf, StorageCounts), String> {
    let old = folder(campaign)?;
    let new = resolve(campaign, name)?;
    if old == new {
        return save_folder(campaign, name).map(|folder| (folder, StorageCounts::default()));
    }
    let files = storage_files(&old)?;
    let moved = StorageCounts::of(&old, &files);
    let pairs: Vec<_> = files
        .iter()
        .map(|path| {
            (
                path,
                new.join(path.strip_prefix(&old).expect("storage path")),
            )
        })
        .collect();
    if let Some((_, target)) = pairs.iter().find(|(_, target)| target.exists()) {
        return Err(format!(
            "Not moved. {} already exists. Choose another folder.",
            target.display()
        ));
    }
    let mut copied = Vec::new();
    let result = (|| {
        for (source, target) in &pairs {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).map_err(|error| {
                    TransferFault::Refused {
                        source: (*source).clone(),
                        read: false,
                        error,
                    }
                    .words(false)
                })?;
            }
            transfer_file(source, target, false, || {}).map_err(|fault| fault.words(false))?;
            copied.push(target.clone());
        }
        save_folder(campaign, name)
    })();
    if let Err(error) = result {
        // The files that moved go back; one that cannot says where it stays, and the setting
        // keeps the old folder.
        let mut errors = vec![error];
        let mut stranded = false;
        for target in copied.into_iter().rev() {
            let source = old.join(target.strip_prefix(&new).expect("storage target"));
            if let Err(fault) = transfer_file(&target, &source, false, || {}) {
                stranded = true;
                errors.push(fault.words(true));
            }
        }
        if stranded {
            let folder_name = |path: &Path| {
                path.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned()
            };
            errors.push(format!(
                "The session folder is still {}. Move the files named above back into it, or \
                 choose {} again.",
                folder_name(&old),
                folder_name(&new)
            ));
        }
        return Err(errors.join(" "));
    }
    Ok((new, moved))
}

/// Why a file did not move, and where it is now (round 30 B7e): the step that failed, both
/// paths, and the system's error, whose words never reach the DM. [`TransferFault::words`]
/// says where the file is and what the DM does next, in plain words from the error's kind
/// (through the World page's file faults, `world::file_fault`).
#[derive(Debug)]
pub(crate) enum TransferFault {
    /// Refused before anything moved: the file is still at `source`. `read` when reading it
    /// for a copy failed.
    Refused {
        source: PathBuf,
        read: bool,
        error: std::io::Error,
    },
    /// The copy is at `target` and the original could not be removed: it is in both folders.
    BothKept {
        source: PathBuf,
        target: PathBuf,
        error: std::io::Error,
    },
    /// The file moved to `target`, but writing its folder through to the disk failed.
    Moved {
        target: PathBuf,
        error: std::io::Error,
    },
    /// The original changed while it was copied: both files were kept.
    Changed { source: PathBuf, target: PathBuf },
    /// The original is gone (moved or deleted outside Splat after the scan): it is no longer in
    /// its folder, and its place is not known. `copy` is a copy this transfer wrote first.
    Gone {
        source: PathBuf,
        copy: Option<PathBuf>,
    },
}

impl TransferFault {
    /// What happened and where the file is now. `back` for the return of a file that a
    /// refused storage move had already moved.
    pub fn words(&self, back: bool) -> String {
        use crate::world::file_fault::FileFault;
        let name = |path: &Path| {
            path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        };
        let folder = |path: &Path| path.parent().map_or_else(String::new, &name);
        let fault = |read: bool, error: &std::io::Error| {
            let fault = if read {
                FileFault::read("", error)
            } else {
                FileFault::write("", error)
            };
            format!("{}. {}", fault.cause(), fault.recovery())
        };
        let moved = if back { "moved back" } else { "moved" };
        match self {
            Self::Refused {
                source,
                read,
                error,
            } => format!(
                "{} was not {moved}. It is still in {}. {}",
                name(source),
                folder(source),
                fault(*read, error)
            ),
            Self::BothKept {
                source,
                target,
                error,
            } => format!(
                "{} was copied to {} but not removed from {}, so it is in both. {} Then delete \
                 the copy you do not need.",
                name(source),
                folder(target),
                folder(source),
                fault(false, error)
            ),
            Self::Moved { target, error } => format!(
                "{} is now in {}, but Splat could not finish writing that folder. {}",
                name(target),
                folder(target),
                fault(false, error)
            ),
            Self::Changed { source, target } => format!(
                "{} changed during the move. Both files were kept, in {} and {}.",
                source.display(),
                folder(source),
                folder(target)
            ),
            // Only what is known to exist: the original is not where it was, and the copy this
            // transfer wrote, if any.
            Self::Gone { source, copy } => {
                let mut words = format!(
                    "{} was not {moved}: it is no longer in {}. It was moved or deleted outside \
                     Splat.",
                    name(source),
                    folder(source)
                );
                if let Some(copy) = copy {
                    words.push_str(&format!(" A copy is in {}.", folder(copy)));
                }
                words
            }
        }
    }
}

/// Where a test's failing transfer fails.
#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum FailAt {
    /// Before anything moves.
    Start,
    /// After the file moved, at the folder's sync.
    Sync,
    /// After the copy is written, at the removal of the original.
    Remove,
}

#[cfg(test)]
thread_local! {
    /// A test's failing transfers: the first `n` succeed, then each fails at this step with
    /// this system error code.
    pub(crate) static FAILING_TRANSFERS: std::cell::Cell<Option<(u32, i32, FailAt)>> =
        const { std::cell::Cell::new(None) };
}

/// The test's failure for this transfer at `at`, if it is set there.
#[cfg(test)]
fn failing_transfer(at: &str) -> Option<std::io::Error> {
    let (left, code, step) = FAILING_TRANSFERS.with(std::cell::Cell::get)?;
    let here = matches!(
        (step, at),
        (FailAt::Start, "start") | (FailAt::Sync, "sync") | (FailAt::Remove, "remove")
    );
    if !here {
        return None;
    }
    if left == 0 {
        Some(std::io::Error::from_raw_os_error(code))
    } else {
        FAILING_TRANSFERS.with(|failing| failing.set(Some((left - 1, code, step))));
        None
    }
}

fn transfer_file(
    source: &Path,
    target: &Path,
    force_copy: bool,
    before_delete: impl FnOnce(),
) -> Result<(), TransferFault> {
    let refused = |read: bool| {
        let source = source.to_owned();
        move |error| TransferFault::Refused {
            source,
            read,
            error,
        }
    };
    // A step on the original itself: one that finds it gone does not say where it is.
    let refused_source = |read: bool| {
        let source = source.to_owned();
        move |error: std::io::Error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                TransferFault::Gone { source, copy: None }
            } else {
                TransferFault::Refused {
                    source,
                    read,
                    error,
                }
            }
        }
    };
    let gone_after_copy = || TransferFault::Gone {
        source: source.to_owned(),
        copy: Some(target.to_owned()),
    };
    let moved = || {
        let target = target.to_owned();
        move |error| TransferFault::Moved { target, error }
    };
    #[cfg(test)]
    if let Some(error) = failing_transfer("start") {
        return Err(refused_source(false)(error));
    }
    let gone = || std::io::Error::from(std::io::ErrorKind::NotFound);
    let source_folder = source.parent().ok_or_else(gone).map_err(refused(false))?;
    let target_folder = target.parent().ok_or_else(gone).map_err(refused(false))?;
    let sync = |folder: &Path| {
        #[cfg(test)]
        if let Some(error) = failing_transfer("sync") {
            return Err(error);
        }
        sync_folder_io(folder)
    };
    #[cfg(target_os = "linux")]
    if !force_copy {
        // RENAME_NOREPLACE protects an arriving sync file.
        match rename_no_replace(source, target) {
            Ok(()) => {
                sync(source_folder).map_err(moved())?;
                sync(target_folder).map_err(moved())?;
                return Ok(());
            }
            Err(error) if error.raw_os_error() != Some(libc::EXDEV) => {
                return Err(refused_source(false)(error));
            }
            Err(_) => {}
        }
    }
    let bytes = fs::read(source).map_err(refused_source(true))?;
    let mut output = tempfile::NamedTempFile::new_in(target_folder).map_err(refused(false))?;
    output
        .write_all(&bytes)
        .and_then(|()| output.as_file().sync_all())
        .map_err(refused(false))?;
    output
        .persist_noclobber(target)
        .map_err(|error| refused(false)(error.error))?;
    let both = |error| TransferFault::BothKept {
        source: source.to_owned(),
        target: target.to_owned(),
        error,
    };
    sync(target_folder).map_err(both)?;
    before_delete();
    match fs::read(source) {
        Ok(now) if now == bytes => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(gone_after_copy());
        }
        _ => {
            return Err(TransferFault::Changed {
                source: source.to_owned(),
                target: target.to_owned(),
            });
        }
    }
    let removed = {
        #[cfg(test)]
        if let Some(error) = failing_transfer("remove") {
            Err(error)
        } else {
            fs::remove_file(source)
        }
        #[cfg(not(test))]
        fs::remove_file(source)
    };
    match removed {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(gone_after_copy());
        }
        Err(error) => return Err(both(error)),
    }
    sync(source_folder).map_err(moved())?;
    sync(target_folder).map_err(moved())
}

fn require_folder(folder: &Path) -> Result<(), String> {
    if folder.is_dir() {
        Ok(())
    } else {
        Err(format!(
            "Session folder is gone: {}. Choose folder before saving.",
            folder.display()
        ))
    }
}

pub fn create_note(folder: &Path, new: &NewNote<'_>) -> Result<Note, String> {
    require_folder(folder)?;
    let id = crate::hermetic::new_uuid();
    let path = note_path(folder, new.number, new.title);
    let mut front = serde_yaml_ng::Mapping::new();
    front.insert("session_id".into(), id.to_string().into());
    front.insert("session_number".into(), u64::from(new.number).into());
    if !new.title.trim().is_empty() {
        front.insert("session_title".into(), new.title.trim().to_owned().into());
    }
    front.insert("session_date".into(), new.date.to_owned().into());
    let front = serde_yaml_ng::to_string(&front).map_err(|e| e.to_string())?;
    let text = format!("---\n{front}---\n{}", fill_template(new));
    write_new_io(&path, &text).map_err(|error| not_created(&error))?;
    let note = parse_note(&path, &text).ok_or("The new note could not be read back.")?;
    remember_notes(folder, std::slice::from_ref(&note))?;
    Ok(note)
}

/// A new file, never replacing one, with the system error kept, so the caller can say what
/// happened in plain words (`not_created`).
fn write_new_io(path: &Path, text: &str) -> std::io::Result<()> {
    #[cfg(any(test, feature = "test-support"))]
    if let Some(code) = FAILING_WRITES.with(std::cell::Cell::get) {
        return Err(std::io::Error::from_raw_os_error(code));
    }
    let parent = path
        .parent()
        .ok_or_else(|| std::io::Error::other("Invalid location."))?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    temp.write_all(text.as_bytes())?;
    temp.as_file().sync_all()?;
    temp.persist_noclobber(path).map_err(|e| e.error)?;
    sync_folder_io(parent)
}

/// Change a note's title or date, and leave any other property the DM added.
pub fn update_details(note: &Note, title: &str, date: &str, number: u32) -> Result<Note, String> {
    let text = fs::read_to_string(&note.path).map_err(|error| not_read(&error))?;
    let patched = patch_fields(
        &text,
        &[
            (
                "session_title",
                (!title.trim().is_empty()).then(|| Value::String(title.trim().to_owned())),
            ),
            ("session_date", Some(Value::String(date.trim().to_owned()))),
            (
                "session_number",
                Some(Value::Number(u64::from(number).into())),
            ),
        ],
    )?;
    // A heading the DM has not changed follows the new name.
    let old = session_heading(note.number, &note.title);
    let new = session_heading(number, title);
    let patched = match split_note(&patched) {
        Some((front, body))
            if body.lines().find(|line| !line.trim().is_empty()) == Some(old.as_str()) =>
        {
            format!("{front}{}", body.replacen(&old, &new, 1))
        }
        _ => patched,
    };
    // So does a date line near the top, as a template writes it.
    let patched = match split_note(&patched) {
        Some((front, body)) if !note.date.is_empty() => {
            let head: usize = body.split_inclusive('\n').take(4).map(str::len).sum();
            let (top, rest) = body.split_at(head);
            let moved: String = top
                .split_inclusive('\n')
                .map(|line| {
                    if line.starts_with(&note.date) {
                        line.replacen(&note.date, date.trim(), 1)
                    } else {
                        line.to_owned()
                    }
                })
                .collect();
            format!("{front}{moved}{rest}")
        }
        _ => patched,
    };
    replace_file_io(&note.path, &patched).map_err(|error| not_saved(&error))?;
    parse_note(&note.path, &patched).ok_or_else(|| "The note could not be read back.".to_owned())
}

/// The session id a note's text holds, read the way the folder scan reads it.
pub fn note_identity(path: &Path, text: &str) -> Option<Uuid> {
    parse_note(path, text).map(|note| note.id)
}

pub(crate) fn replace_file(path: &Path, text: &str) -> Result<(), String> {
    replace_file_io(path, text).map_err(|e| e.to_string())
}

/// `replace_file` with the system error kept, so a note's save can say what happened in plain
/// words (`not_saved`).
fn replace_file_io(path: &Path, text: &str) -> std::io::Result<()> {
    #[cfg(any(test, feature = "test-support"))]
    if let Some(code) = FAILING_WRITES.with(std::cell::Cell::get) {
        return Err(std::io::Error::from_raw_os_error(code));
    }
    let parent = path
        .parent()
        .ok_or_else(|| std::io::Error::other("Invalid location."))?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    temp.write_all(text.as_bytes())
        .and_then(|()| temp.as_file().sync_all())?;
    temp.persist(path).map_err(|e| e.error)?;
    sync_folder_io(parent)
}

use splat_campaign::file_batch::sync_folder as sync_folder_io;

/// Round 30 B7d: a note's failed save in plain words from the error's KIND, never the system's
/// words: `Not saved.`, the cause and the recovery of the World page's file faults
/// (`world::file_fault`, the one mapping). Every painter of a note's failed save reads this
/// value: the Sessions banner on every page, the Notes status, an act's `NotSaved` result.
fn not_saved(error: &std::io::Error) -> String {
    let fault = crate::world::file_fault::FileFault::write("", error);
    format!("Not saved. {}. {}", fault.cause(), fault.recovery())
}

/// A new session note that could not be written (Create session), in the same plain words.
fn not_created(error: &std::io::Error) -> String {
    let fault = crate::world::file_fault::FileFault::write("", error);
    format!("Not created. {}. {}", fault.cause(), fault.recovery())
}

/// A note that could not be read before a change to it (Save details), in the same plain words.
fn not_read(error: &std::io::Error) -> String {
    let fault = crate::world::file_fault::FileFault::read("", error);
    format!("Not saved. {}. {}", fault.cause(), fault.recovery())
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// A test's failing disk for the write helpers (`replace_file_io`, `write_new_io`): each
    /// fails with this system error code before anything is written.
    pub(crate) static FAILING_WRITES: std::cell::Cell<Option<i32>> = const { std::cell::Cell::new(None) };
}

#[derive(Debug, PartialEq)]
pub struct Template {
    pub name: String,
    /// None for a built-in template, which is read-only until customized.
    pub path: Option<PathBuf>,
    pub body: String,
}

/// A copy of a template copies its words: counted in tests (layout PR 9b-C), so a frame that
/// copies one shows in `pane::text_work`.
impl Clone for Template {
    fn clone(&self) -> Self {
        crate::design::text_draft::count_copied(self.body.len());
        Self {
            name: self.name.clone(),
            path: self.path.clone(),
            body: self.body.clone(),
        }
    }
}

pub const BUILT_IN_TEMPLATES: [(&str, &str); 2] = [
    (
        "Standard session",
        "# Session {{number}}: {{title}}\n\n{{date}}, {{campaign}}\n\n## Recap of last time\n\n## Plan for tonight\n\n## What happened\n\n## People and places\n\n## Loot and rewards\n\n## Open threads\n\n",
    ),
    (
        "Short notes",
        "# Session {{number}}: {{title}}\n\n{{date}}\n\n## What happened\n\n## Next time\n\n",
    ),
];

/// The campaign's own templates, by name, then the built-in ones it has not replaced.
pub fn list_templates(folder: &Path) -> Vec<Template> {
    let mut own: Vec<Template> = fs::read_dir(folder.join(TEMPLATES))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|file| file.path().extension().is_some_and(|e| e == "md"))
        .filter_map(|file| {
            let path = file.path();
            Some(Template {
                name: path.file_stem()?.to_string_lossy().into_owned(),
                body: fs::read_to_string(&path).ok()?,
                path: Some(path),
            })
        })
        .collect();
    own.sort_by_key(|template| template.name.to_lowercase());
    for (name, body) in BUILT_IN_TEMPLATES {
        if !own.iter().any(|template| template.name == name) {
            own.push(Template {
                name: name.to_owned(),
                path: None,
                body: body.to_owned(),
            });
        }
    }
    own
}

pub fn create_template(folder: &Path, name: &str, body: &str) -> Result<Template, String> {
    require_folder(folder)?;
    let name = file_safe(name);
    if name.is_empty() {
        return Err("Enter a name for the template.".into());
    }
    let templates = folder.join(TEMPLATES);
    match fs::create_dir(&templates) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(error) => return Err(not_created(&error)),
    }
    let path = templates.join(format!("{name}.md"));
    if path.exists() {
        return Err(format!("A template named “{name}” already exists."));
    }
    write_new_io(&path, body).map_err(|error| not_created(&error))?;
    Ok(Template {
        name,
        path: Some(path),
        body: body.to_owned(),
    })
}

/// A recording's file name as the DM reads it, with the time written as a time.
pub fn recording_title(path: &Path) -> String {
    let stem = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default();
    match stem.split_once(' ') {
        Some((date, time)) => format!("{date} {}", time.replace('-', ":")),
        None => stem,
    }
}

/// One recording of a session, and what later processing has made from it.
#[derive(Clone, Debug, PartialEq)]
pub struct Recording {
    pub path: PathBuf,
    pub name: String,
    /// The name as the DM reads it, with the time written as a time.
    pub title: String,
    pub seconds: Option<u64>,
    pub bytes: u64,
    pub transcript: bool,
    pub summary: bool,
    pub marked: bool,
    pub review: bool,
    /// What the DM must know about the result, if anything.
    pub warning: Option<String>,
    /// A retained processing failure, cancellation, or recorder stop.
    pub status: Option<String>,
    /// The events an LLM marked. The DM chooses which go on the timeline.
    pub markers: Vec<crate::timeline::Marker>,
}

impl Recording {
    pub fn check_missing(&self) -> bool {
        self.warning
            .as_deref()
            .is_some_and(|warning| warning.contains(crate::processing::NO_CHECK_WARNING))
    }
}

pub fn recordings_folder(folder: &Path, session: Uuid) -> PathBuf {
    folder.join(RECORDINGS).join(session.to_string())
}

/// The newest first. The file name begins with the time it started.
pub fn list_recordings(folder: &Path, session: Uuid) -> Vec<Recording> {
    let paths: std::collections::BTreeSet<PathBuf> =
        fs::read_dir(recordings_folder(folder, session))
            .into_iter()
            .flatten()
            .flatten()
            .filter_map(|file| {
                let path = file.path();
                let name = path.file_name()?.to_str()?;
                [
                    ".wav",
                    ".transcript.md",
                    ".summary.md",
                    ".marked.md",
                    ".cleaned.md",
                    ".review.md",
                    ".warning.md",
                    ".state.txt",
                ]
                .iter()
                .find_map(|suffix| name.strip_suffix(suffix))
                .map(|stem| path.with_file_name(format!("{stem}.wav")))
            })
            .collect();
    let mut recordings: Vec<Recording> = paths
        .into_iter()
        .map(|path| Recording {
            name: path
                .file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
                .unwrap_or_default(),
            title: recording_title(&path),
            seconds: crate::recording::Layout::read(&path)
                .ok()
                .map(|l| l.seconds()),
            bytes: fs::metadata(&path).map_or(0, |metadata| metadata.len()),
            transcript: crate::processing::transcript_path(&path).exists(),
            summary: crate::processing::summary_path(&path).exists(),
            marked: crate::processing::marked_path(&path).exists(),
            review: crate::processing::review_path(&path).exists(),
            warning: fs::read_to_string(crate::processing::warning_path(&path)).ok(),
            status: fs::read_to_string(path.with_extension("state.txt")).ok(),
            markers: [
                crate::processing::marked_path(&path),
                crate::processing::summary_path(&path),
            ]
            .iter()
            .filter_map(|made| fs::read_to_string(made).ok())
            .flat_map(|text| crate::timeline::parse_markers(&text))
            .collect(),
            path,
        })
        .collect();
    recordings.sort_by(|a, b| b.name.cmp(&a.name));
    recordings
}

/// A new recording's path. The DM reads this name, so it is the local date and time.
pub fn new_recording_path(folder: &Path, session: Uuid) -> PathBuf {
    let directory = recordings_folder(folder, session);
    free_path(&directory, &crate::hermetic::local_stamp(), "wav")
}

/// Remove a recording and what was made from it.
/// A removal the system refused, in plain words from the error's kind (round 30 B7e): what was
/// not deleted, and the recovery of the World page's file faults. Never the system's words.
pub(crate) fn not_deleted(path: &Path, error: &std::io::Error) -> String {
    use crate::world::file_fault::{FileFault, Kind};
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let fault = FileFault::write(&name, error);
    let what = match fault.kind {
        Kind::WriteDenied => format!("Splat has no permission to delete {name}."),
        Kind::ReadOnly => format!("The disk is read-only, so Splat cannot delete {name}."),
        _ => format!("Splat cannot delete {name}."),
    };
    format!("Not deleted. {what} {}", fault.recovery())
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// A test's refused removals: removing a file that exists fails with this system error code.
    pub(crate) static FAILING_REMOVES: std::cell::Cell<Option<i32>> = const { std::cell::Cell::new(None) };
}

/// Remove a file, with the system's error kept for `not_deleted`.
pub fn remove_file(path: &Path) -> std::io::Result<()> {
    #[cfg(any(test, feature = "test-support"))]
    if let Some(code) = FAILING_REMOVES.with(std::cell::Cell::get)
        && path.exists()
    {
        return Err(std::io::Error::from_raw_os_error(code));
    }
    fs::remove_file(path)
}

pub fn delete_recording(path: &Path) -> Result<(), String> {
    for made in [
        crate::processing::transcript_path(path),
        crate::processing::summary_path(path),
        crate::processing::marked_path(path),
        crate::processing::cleaned_path(path),
        crate::processing::review_path(path),
        crate::processing::warning_path(path),
        path.with_extension("state.txt"),
        path.to_path_buf(),
    ] {
        match remove_file(&made) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(not_deleted(&made, &e)),
        }
    }
    Ok(())
}

/// The heading a note's events and summaries belong under, if the note has it.
const WHAT_HAPPENED: &str = "## What happened";

/// Where a note's `What happened` section ends, if it has one.
fn what_happened_end(body: &str) -> Option<usize> {
    body.match_indices(WHAT_HAPPENED)
        .find(|(at, _)| {
            (*at == 0 || body[..*at].ends_with('\n'))
                && body[at + WHAT_HAPPENED.len()..]
                    .chars()
                    .next()
                    .is_none_or(|c| c == '\n' || c == '\r')
        })
        .map(|(at, _)| at + WHAT_HAPPENED.len())
        .map(|start| {
            body[start..]
                .find("\n## ")
                .map_or(body.len(), |next| start + next)
        })
}

/// Add words to a note where a DM would put them: at the end of its `What happened`
/// section, or at the end of the note.
pub fn insert_in_note(body: &mut String, words: &str) {
    let end = what_happened_end(body).unwrap_or(body.len());
    let kept = body[..end].trim_end().len();
    let rest = body[end..].to_owned();
    body.truncate(kept);
    if !body.is_empty() {
        body.push_str("\n\n");
    }
    body.push_str(words.trim());
    body.push('\n');
    body.push_str(&rest);
}

/// Open a fresh line at the same place, for typing during play. Returns the caret, in
/// characters, and whether the text had to change to make the line.
pub fn fresh_line_in_note(body: &mut String) -> (usize, bool) {
    let end = what_happened_end(body).unwrap_or(body.len());
    let kept = body[..end].trim_end().len();
    let blank = body[kept..].chars().take_while(|c| *c == '\n').count();
    let before = if kept == 0 { 0 } else { 2 };
    // What follows stays a line of its own, or the DM's words would join a heading.
    let after = if body[kept..].trim().is_empty() { 0 } else { 2 };
    let changed = blank < before + after;
    if changed {
        body.insert_str(kept, &"\n".repeat(before + after - blank));
    }
    (body[..kept + before].chars().count(), changed)
}

/// A text's headings, one level down, so that it sits inside a section of a note.
pub fn nested_in_note(text: &str) -> String {
    text.split_inclusive('\n')
        .map(|line| {
            if line.starts_with('#') {
                format!("#{line}")
            } else {
                line.to_owned()
            }
        })
        .collect()
}

/// A note for the Reader, without the heading and date the page already shows above it.
pub fn without_own_heading(body: &str, note: &Note) -> String {
    let heading = format!("# {}", note.label());
    let mut lines = body.split_inclusive('\n').peekable();
    while lines.peek().is_some_and(|line| line.trim().is_empty()) {
        lines.next();
    }
    if lines.peek().map(|line| line.trim_end()) != Some(heading.as_str()) {
        return body.to_owned();
    }
    lines.next();
    while lines.peek().is_some_and(|line| line.trim().is_empty()) {
        lines.next();
    }
    if !note.date.is_empty()
        && lines
            .peek()
            .is_some_and(|line| line.starts_with(&note.date))
    {
        lines.next();
    }
    lines.collect::<String>().trim_start().to_owned()
}

/// A note's headings with something under them, for reading. A template leaves headings the
/// DM never filled, and a page of empty headings hides the few words there are.
pub fn without_empty_sections(text: &str) -> String {
    let level = |line: &str| {
        let hashes = line.chars().take_while(|c| *c == '#').count();
        (hashes > 0 && line[hashes..].starts_with(' ')).then_some(hashes)
    };
    let mut lines: Vec<&str> = text.lines().collect();
    // A heading whose only subheading was empty is empty too, so this goes until nothing changes.
    loop {
        let empty = (0..lines.len()).find(|&at| {
            level(lines[at]).is_some_and(|own| {
                lines[at + 1..]
                    .iter()
                    .find(|next| !next.trim().is_empty())
                    .is_none_or(|next| level(next).is_some_and(|other| other <= own))
            })
        });
        let Some(at) = empty else {
            break;
        };
        lines.remove(at);
    }
    let mut kept = String::new();
    let mut blank = true;
    for line in lines {
        let is_blank = line.trim().is_empty();
        if !(is_blank && blank) {
            kept.push_str(line);
            kept.push('\n');
        }
        blank = is_blank;
    }
    format!("{}\n", kept.trim_end())
}

/// The words under one `## heading`, up to the next heading of that level.
pub fn section_text(body: &str, heading: &str) -> Option<String> {
    let mut lines = body.lines();
    lines.find(|line| line.trim_end().strip_prefix("## ") == Some(heading))?;
    let words = lines
        .take_while(|line| !line.starts_with("## "))
        .collect::<Vec<_>>()
        .join("\n");
    let words = words.trim();
    (!words.is_empty()).then(|| words.to_owned())
}

/// Add a line to a note's plan: under `Plan for tonight`, or where other words go.
pub fn insert_in_plan(body: &mut String, words: &str) {
    const PLAN: &str = "## Plan for tonight";
    let Some(at) = body
        .match_indices(PLAN)
        .find(|(at, _)| *at == 0 || body[..*at].ends_with('\n'))
        .map(|(at, _)| at + PLAN.len())
    else {
        insert_in_note(body, words);
        return;
    };
    let end = body[at..]
        .find("\n## ")
        .map_or(body.len(), |next| at + next);
    let kept = body[..end].trim_end().len();
    let rest = body[end..].to_owned();
    body.truncate(kept);
    body.push_str(if kept == at { "\n\n" } else { "\n" });
    body.push_str(words.trim());
    body.push('\n');
    body.push_str(&rest);
}

/// One line of a note's plan, in the note's order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlanLine {
    /// A task line that starts at the line's start: `- [ ] words` or `- [x] words`.
    Task(TaskLine),
    /// Any other line, by the names it links.
    Links(Vec<String>),
}

/// A task line of the plan. It is a scene when it links no fight and no group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskLine {
    /// The line's number in the note's words, so that an act finds it nearest this place.
    pub at: usize,
    /// The whole line, as the note holds it.
    pub line: String,
    pub done: bool,
    /// The words after the box, as readers show them: a travel check's mark is not among them.
    /// Acts that change the line use `line`.
    pub words: String,
}

/// A line's box and words, when it is a task line that starts at the line's start.
fn task_parts(line: &str) -> Option<(bool, &str)> {
    let rest = line
        .strip_prefix("- ")
        .or_else(|| line.strip_prefix("* "))
        .or_else(|| line.strip_prefix("+ "))?;
    let (done, words) = if let Some(words) = rest.strip_prefix("[ ]") {
        (false, words)
    } else {
        (true, rest.strip_prefix("[x]").or(rest.strip_prefix("[X]"))?)
    };
    let words = words.strip_prefix(' ')?.trim();
    (!words.is_empty()).then_some((done, words))
}

/// The lines under `## Plan for tonight`, as `section_text` finds that section.
pub fn plan_lines(body: &str) -> Vec<PlanLine> {
    let mut lines = body.lines().enumerate();
    if lines
        .find(|(_, line)| line.trim_end().strip_prefix("## ") == Some("Plan for tonight"))
        .is_none()
    {
        return Vec::new();
    }
    lines
        .take_while(|(_, line)| !line.starts_with("## "))
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(at, line)| match task_parts(line.trim_end()) {
            Some((done, words)) => PlanLine::Task(TaskLine {
                at,
                line: line.trim_end_matches('\r').to_owned(),
                done,
                // What a reader shows and copies: the mark of a travel check is no word.
                words: crate::timeline::checks::marked(words)
                    .words
                    .trim_end()
                    .to_owned(),
            }),
            None => PlanLine::Links(each_linked_name(line)),
        })
        .collect()
}

/// A scene's line at the end of the plan. A note with no plan gets the heading at its end.
pub fn add_scene(body: &str, words: &str) -> String {
    let line = format!(
        "- [ ] {}",
        words.split_whitespace().collect::<Vec<_>>().join(" ")
    );
    let has_plan = body
        .lines()
        .any(|text| text.trim_end().strip_prefix("## ") == Some("Plan for tonight"));
    if has_plan {
        let mut out = body.to_owned();
        insert_in_plan(&mut out, &line);
        return out;
    }
    let kept = body.trim_end();
    format!(
        "{kept}{}## Plan for tonight\n\n{line}\n",
        if kept.is_empty() { "" } else { "\n\n" }
    )
}

/// The links to a thing that has a new name, whatever their case. Returns whether any changed.
pub fn rename_links(body: &mut String, old: &str, new: &str) -> bool {
    let mut excluded = Vec::new();
    let mut block = None;
    for (event, range) in pulldown_cmark::Parser::new(body).into_offset_iter() {
        match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::CodeBlock(_)) => {
                block = Some(range.start)
            }
            pulldown_cmark::Event::End(pulldown_cmark::TagEnd::CodeBlock) => {
                if let Some(start) = block.take() {
                    excluded.push(start..range.end);
                }
            }
            pulldown_cmark::Event::Code(_) => excluded.push(range),
            _ => {}
        }
    }
    let mut renamed = String::with_capacity(body.len());
    let mut rest = body.as_str();
    let mut changed = false;
    while let Some(start) = rest.find("[[") {
        let offset = body.len() - rest.len() + start;
        let (before, after) = rest.split_at(start + 2);
        renamed.push_str(before);
        rest = after;
        let Some(end) = after.find(['|', ']']) else {
            break;
        };
        let (name, tail) = after.split_at(end);
        let (target, anchor) = name
            .split_once('#')
            .map_or((name, ""), |(target, _)| (target, &name[target.len()..]));
        if !excluded.iter().any(|range| range.contains(&offset))
            && target.trim().eq_ignore_ascii_case(old)
            && (tail.starts_with('|') || tail.starts_with("]]"))
        {
            renamed.push_str(new);
            renamed.push_str(anchor);
            rest = tail;
            changed = true;
        }
    }
    renamed.push_str(rest);
    *body = renamed;
    changed
}

/// The same for a note that is not open. Its frontmatter and its other words stay as they are.
pub fn rename_links_in_file(path: &Path, old: &str, new: &str) -> Result<bool, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let (front, body) = split_note(&text).unwrap_or(("", &text));
    let mut renamed = body.to_owned();
    if !rename_links(&mut renamed, old, new) {
        return Ok(false);
    }
    replace_file(path, &format!("{front}{renamed}"))?;
    Ok(true)
}

/// A long text, divided at its time headings, so that each part can be read alone.
pub fn parts_of(text: &str) -> Vec<(String, String)> {
    let mut parts: Vec<(String, String)> = Vec::new();
    for line in text.split_inclusive('\n') {
        let heading = line.trim_end().strip_prefix("## ").filter(|time| {
            time.len() == 8 && time.as_bytes()[2] == b':' && time.as_bytes()[5] == b':'
        });
        match (heading, parts.last_mut()) {
            (Some(time), _) => parts.push((time.to_owned(), line.to_owned())),
            (None, Some(last)) => last.1.push_str(line),
            (None, None) => parts.push((String::new(), line.to_owned())),
        }
    }
    // Words before the first heading are read with it.
    if parts.len() > 1 && parts[0].0.is_empty() {
        let first = parts.remove(0);
        parts[0].1.insert_str(0, &first.1);
    }
    if parts.is_empty() {
        parts.push((String::new(), String::new()));
    }
    parts
}

/// A file's frontmatter removed, for reading. Text with none is returned as it is.
pub fn body_of(text: &str) -> &str {
    split_note(text).map_or(text, |(_, body)| body)
}

#[derive(Clone, Debug, PartialEq)]
pub enum DraftState {
    Clean,
    Unsaved,
    Saved,
    /// The file changed outside Splat during an edit. Saving stops until the DM chooses.
    Conflict,
    /// The file is gone from the folder. It may have been moved or renamed.
    Missing,
    Failed(String),
}

/// One file being edited. The prefix (frontmatter) stays as the file has it. Edits save
/// shortly after they stop, and a change made elsewhere is never overwritten silently.
#[derive(Clone)]
pub struct Draft {
    pub path: PathBuf,
    search_folder: PathBuf,
    prefix: String,
    /// The note's words, a document of their own (layout PR 9b): its editor keeps its layout
    /// and its parse by the draft's key.
    pub body: crate::design::text_draft::TextDraft,
    baseline: Option<String>,
    pub state: DraftState,
    /// What the file holds now, once the DM asks to see it during a conflict.
    pub on_disk: Option<String>,
    pub moved_to: Option<PathBuf>,
    /// This note's new ID, once it was saved again after its file went.
    pub restored: Option<Uuid>,
    /// The words the DM chose to lose, and how, until the next edit.
    pub dropped: Option<Dropped>,
    /// The file's text when the words were dropped. If the draft has followed the file since,
    /// Undo meets the conflict choice instead of saving over the newer text.
    dropped_baseline: Option<String>,
    /// Why `Save as a new file` could not make the file. The draft stays `Missing`, so no save
    /// writes anywhere until the DM chooses again.
    pub save_again_error: Option<String>,
    last_edit: Option<Instant>,
    /// Changes when the words or the file change. No two drafts share one.
    revision: u64,
    /// The Notes panel's mark and the note's own writes since it (review of PR 825): the body's
    /// revision when the panel last held it ([`Draft::mark_followed`]), and its revision after
    /// the note's own writes that came after it, each from the revision the one before left
    /// ([`Draft::own_write`]). Any other change breaks the row (a reload makes a new draft; a
    /// discard writes no own write), and then the panel's typing history ends.
    own_since: Option<(
        crate::design::text_draft::DraftKey,
        crate::design::text_draft::DraftKey,
    )>,
    /// A test's failing disk: each write fails with this system error code (28 is a full disk),
    /// before anything is written or the baseline moves.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) failing_write: Option<i32>,
}

/// A revision no draft has had.
fn next_revision() -> u64 {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// How a note's draft was lost. Its Undo names the same action.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Dropped {
    /// `Reload and lose my draft` after the file changed on disk.
    Reloaded(String),
    /// `Discard` after the file was moved or deleted.
    Discarded(String),
}

impl Draft {
    pub fn open(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let (prefix, body) = split_note(&text).unwrap_or(("", &text));
        Ok(Self {
            path: path.to_owned(),
            search_folder: path
                .ancestors()
                .skip(1)
                .find(|folder| folder.join(NOTE_PATHS).is_file())
                .or_else(|| path.parent())
                .unwrap_or(path)
                .to_owned(),
            prefix: prefix.to_owned(),
            body: crate::design::text_draft::TextDraft::new(body),
            baseline: Some(text.clone()),
            state: DraftState::Clean,
            on_disk: None,
            moved_to: None,
            restored: None,
            dropped: None,
            dropped_baseline: None,
            save_again_error: None,
            last_edit: None,
            revision: next_revision(),
            own_since: None,
            #[cfg(any(test, feature = "test-support"))]
            failing_write: None,
        })
    }

    /// The Notes panel holds the body as it is now: the row of the note's own writes starts here.
    pub fn mark_followed(&mut self) {
        let now = self.body.key();
        self.own_since = Some((now, now));
    }

    /// Every change since the Notes panel last held the body was the note's own write (or there
    /// was none): the panel's typing history stays under them.
    pub fn own_since_followed(&self) -> bool {
        self.own_since
            .is_some_and(|(_, head)| head == self.body.key())
    }

    /// The note's own write changed the body from revision `before` (an act, the page's insert,
    /// a renamed link, the DM's typing on the page). It goes on the row only when the row ends
    /// at `before`: a change made another way between them broke it.
    pub fn own_write(&mut self, before: crate::design::text_draft::DraftKey) {
        self.own_since = self
            .own_since
            .filter(|(_, head)| *head == before)
            .map(|(from, _)| (from, self.body.key()));
    }

    /// This draft, a copy of `open` that an act wrote and saved (an event's link update, its
    /// Undo), takes `open`'s place. The copy is a new document, so the row of own writes goes
    /// on through it only when the copy started from the words `open` holds at the row's end.
    pub fn take_over_from(&mut self, open: &Draft, started: &str) {
        self.own_since = open
            .own_since
            .filter(|(_, head)| *head == open.body.key() && open.body.same_words(started))
            .map(|(from, _)| (from, self.body.key()));
    }

    /// The file's words as this draft last read or wrote them.
    #[cfg(any(test, feature = "test-support"))]
    pub fn baseline(&self) -> Option<&str> {
        self.baseline.as_deref()
    }

    /// Changes whenever the words or the file change, so a reader can skip an unchanged draft.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// A conflict or a failed save stays in view through more typing, until it is resolved.
    pub fn edited(&mut self) {
        self.dropped = None;
        if !self.needs_attention() {
            self.state = DraftState::Unsaved;
        }
        self.last_edit = Some(Instant::now());
        self.revision = next_revision();
    }

    /// The note's whole text in this draft: its properties, then its words.
    pub fn text(&self) -> String {
        format!("{}{}", self.prefix, self.body.as_str())
    }

    /// The text this draft was read from or last wrote.
    pub fn file_text(&self) -> Option<&str> {
        self.baseline.as_deref()
    }

    /// The file still holds the text this draft was read from.
    pub fn file_unchanged(&self) -> bool {
        fs::read_to_string(&self.path).ok() == self.baseline
    }

    /// Does this draft hold something the file does not?
    pub fn unsaved(&self) -> bool {
        matches!(
            self.state,
            DraftState::Unsaved
                | DraftState::Conflict
                | DraftState::Missing
                | DraftState::Failed(_)
        )
    }

    pub fn needs_attention(&self) -> bool {
        matches!(
            self.state,
            DraftState::Conflict | DraftState::Missing | DraftState::Failed(_)
        )
    }

    #[cfg(any(test, not(feature = "test-support")))]
    fn due_at(&self, now: Instant) -> bool {
        self.state == DraftState::Unsaved
            && self
                .last_edit
                .is_none_or(|at| now.duration_since(at) >= SAVE_AFTER)
    }

    pub fn save(&mut self) {
        if !matches!(self.state, DraftState::Unsaved | DraftState::Failed(_)) {
            return;
        }
        let current = fs::read_to_string(&self.path).ok();
        if current != self.baseline {
            if current.is_none()
                && let Some(note) = self
                    .baseline
                    .as_deref()
                    .and_then(|text| parse_note(&self.path, text))
            {
                let matches: Vec<_> = list_notes(&self.search_folder)
                    .into_iter()
                    .filter(|found| found.id == note.id)
                    .collect();
                if matches.len() == 1 && matches[0].also.is_empty() {
                    self.moved_to = Some(matches[0].path.clone());
                }
            }
            self.state = if current.is_none() {
                DraftState::Missing
            } else {
                DraftState::Conflict
            };
            return;
        }
        self.write();
    }

    fn write(&mut self) {
        let text = format!("{}{}", self.prefix, self.body);
        #[cfg(any(test, feature = "test-support"))]
        let written = match self.failing_write {
            Some(code) => Err(std::io::Error::from_raw_os_error(code)),
            None => replace_file_io(&self.path, &text),
        };
        #[cfg(not(any(test, feature = "test-support")))]
        let written = replace_file_io(&self.path, &text);
        match written {
            Ok(()) => {
                self.baseline = Some(text);
                self.state = DraftState::Saved;
            }
            Err(error) => {
                self.state = DraftState::Failed(not_saved(&error));
            }
        }
    }

    /// Save the words where the file was. A renamed file can still hold this note's ID, so
    /// this one takes a new ID, and two rows can never answer for one session.
    pub fn save_again(&mut self) {
        // The new identity is prepared aside and becomes the draft's only once the file exists:
        // a failed creation leaves the draft as it was, still `Missing`.
        self.save_again_error = None;
        let mut prefix = self.prefix.clone();
        let mut restored = None;
        if self.prefix.contains("session_id:") {
            let id = crate::hermetic::new_uuid();
            let patched = patch_fields(
                &format!("{}{}", self.prefix, self.body),
                &[("session_id", Some(Value::String(id.to_string())))],
            );
            match patched.as_deref().map(split_note) {
                Ok(Some((new_prefix, _))) => {
                    new_prefix.clone_into(&mut prefix);
                    restored = Some(id);
                }
                _ => {
                    self.refuse_save_again("Not saved. Its properties cannot be read.".into());
                    return;
                }
            }
        }
        if self.path.parent().is_none_or(|parent| !parent.is_dir()) {
            self.refuse_save_again("Session folder is gone. Choose folder before saving.".into());
            return;
        }
        let text = format!("{prefix}{}", self.body);
        match write_new_io(&self.path, &text) {
            Ok(()) => {
                self.prefix = prefix;
                if restored.is_some() {
                    self.restored = restored;
                }
                self.baseline = Some(text);
                self.state = DraftState::Saved;
            }
            Err(error) => self.refuse_save_again(not_saved(&error)),
        }
    }

    /// A new file could not be made: say why, and keep the missing-file write block.
    fn refuse_save_again(&mut self, error: String) {
        self.save_again_error = Some(error);
        if self.state != DraftState::Missing {
            self.state = DraftState::Missing;
        }
    }

    pub fn save_moved(&mut self) {
        if let Some(path) = self.moved_to.take() {
            self.path = path;
            self.revision = next_revision();
            self.state = DraftState::Unsaved;
            self.save();
        }
    }

    /// The file went, and the DM chose to lose the draft. One step of undo brings it back.
    ///
    /// When the note is known at a new path and that file still holds this note, the editor
    /// follows it: the words, the baseline and the save target become the moved file's. With
    /// no such file, the draft stays `Missing` with the last saved words, so no edit or Save
    /// can write the old path; only the DM's explicit `Save as a new file` does.
    pub fn discard(&mut self) {
        let words = self.body.as_str().to_owned();
        let note = self
            .baseline
            .as_deref()
            .and_then(|text| parse_note(&self.path, text))
            .map(|note| note.id);
        let followed = self.moved_to.take().and_then(|path| {
            let moved = Self::open(&path).ok()?;
            let owner = moved
                .baseline
                .as_deref()
                .and_then(|text| parse_note(&path, text))
                .map(|found| found.id);
            (note.is_some() && owner == note).then_some(moved)
        });
        match followed {
            Some(moved) => *self = moved,
            None => {
                let words = self
                    .baseline
                    .as_deref()
                    .map(body_of)
                    .unwrap_or_default()
                    .to_owned();
                let _ = self.body.replace(words);
                self.revision = next_revision();
                self.state = DraftState::Missing;
            }
        }
        self.dropped = Some(Dropped::Discarded(words));
        self.dropped_baseline.clone_from(&self.baseline);
    }

    /// Read the file's own words, so that the DM can choose knowing both.
    pub fn show_disk(&mut self) {
        self.on_disk = fs::read_to_string(&self.path)
            .ok()
            .map(|text| body_of(&text).to_owned());
    }

    /// The DM keeps this version over the one on disk.
    pub fn overwrite(&mut self) {
        self.on_disk = None;
        // The frontmatter on disk can be newer, and the DM chose only about the words.
        if let Ok(current) = fs::read_to_string(&self.path)
            && let Some((prefix, _)) = split_note(&current)
        {
            self.prefix = prefix.to_owned();
        }
        self.write();
    }

    /// Back to the file, keeping the draft's words for one step of undo.
    pub fn reload_keeping_undo(&mut self) -> Result<(), String> {
        let words = self.body.as_str().to_owned();
        self.reload()?;
        self.dropped = Some(Dropped::Reloaded(words));
        self.dropped_baseline.clone_from(&self.baseline);
        Ok(())
    }

    /// Bring back what `reload_keeping_undo` or `discard` dropped.
    pub fn undo_dropped(&mut self) {
        if let Some(Dropped::Reloaded(words) | Dropped::Discarded(words)) = self.dropped.take() {
            let _ = self.body.replace(words);
            self.edited();
            // The file changed since the words were dropped, and the draft followed it: the DM
            // chooses between the two, as in any conflict.
            if self.baseline != self.dropped_baseline.take() && self.state == DraftState::Unsaved {
                self.state = DraftState::Conflict;
            }
        }
    }

    /// Back to the file, and the draft's words are gone.
    pub fn reload(&mut self) -> Result<(), String> {
        *self = Self::open(&self.path)?;
        Ok(())
    }

    /// Follow a change made elsewhere when there is no edit to lose.
    pub fn follow_disk(&mut self) {
        if self.unsaved() {
            return;
        }
        let current = fs::read_to_string(&self.path).ok();
        if current != self.baseline && current.is_some() {
            let state = self.state.clone();
            // Words the DM dropped stay recoverable through a reload the timer makes.
            let dropped = (self.dropped.clone(), self.dropped_baseline.clone());
            if self.reload().is_ok() {
                (self.dropped, self.dropped_baseline) = dropped;
                self.state = if state == DraftState::Saved {
                    DraftState::Clean
                } else {
                    state
                };
            }
        }
    }

    /// The one way an act changes this note's words (section 4.1 of the session-cycle design).
    /// `note` is the session the act is for, which its place knows; the file is checked against
    /// it, never against what a reload last read. `patch` gets the words and returns them with
    /// one fragment changed, or `None` when the fragment's place is gone. The Undo that comes
    /// back is the act's, and the act's place keeps it. `Apply` says how the act meets the DM's
    /// typing; see each of its values.
    pub fn apply(
        &mut self,
        note: Uuid,
        write: Apply,
        patch: impl FnOnce(&str) -> Option<String>,
    ) -> Result<NoteUndo, ActFault> {
        self.apply_parts(note, write, |prefix, body| {
            patch(body).map(|body| (prefix.to_owned(), body))
        })
    }

    /// `apply` for an act that also changes the note's properties: `patch` gets the note's
    /// whole text, properties first, and returns it whole. Its Undo puts both back.
    pub fn apply_with_properties(
        &mut self,
        note: Uuid,
        patch: impl FnOnce(&str) -> Option<String>,
    ) -> Result<NoteUndo, ActFault> {
        self.apply_parts(note, Apply::Now, |prefix, body| {
            let text = patch(&format!("{prefix}{body}"))?;
            let (prefix, body) = split_note(&text).unwrap_or(("", &text));
            Some((prefix.to_owned(), body.to_owned()))
        })
    }

    /// The one path of `apply` and `apply_with_properties`: `patch` gets the properties and the
    /// words, and returns both.
    fn apply_parts(
        &mut self,
        note: Uuid,
        write: Apply,
        patch: impl FnOnce(&str, &str) -> Option<(String, String)>,
    ) -> Result<NoteUndo, ActFault> {
        if write == Apply::Now {
            // An edit made elsewhere a moment ago is in the words that the act changes.
            self.follow_disk();
            match fs::read_to_string(&self.path) {
                Err(_) => return Err(ActFault::Missing),
                // Unsaved words and a changed file: the conflict view decides first.
                Ok(current)
                    if Some(&current) != self.baseline.as_ref()
                        || self.state == DraftState::Conflict =>
                {
                    return Err(ActFault::Changed);
                }
                Ok(_) if self.state == DraftState::Missing => return Err(ActFault::Missing),
                // Also on a retry: the file holds another note for as long as it does.
                Ok(_) if self.note_id() != Some(note) => return Err(ActFault::OtherNote),
                Ok(_) => {}
            }
        }
        let before = self.body.as_str().to_owned();
        let (prefix, after) = patch(&self.prefix, &before).ok_or(ActFault::LineChanged)?;
        // Only an act that changed the properties puts them back.
        let properties = (prefix != self.prefix).then(|| {
            let before = std::mem::replace(&mut self.prefix, prefix);
            Box::new((before, self.prefix.clone()))
        });
        let key = self.body.key();
        let _ = self.body.replace(after.as_str());
        self.own_write(key);
        self.edited();
        let mut undo = NoteUndo {
            path: self.path.as_path().into(),
            note,
            before,
            after,
            properties,
            written: false,
        };
        if write == Apply::WithTyping {
            return Ok(undo);
        }
        self.save();
        // Otherwise the fragment waits in the draft, and the note's own state says why: a
        // failed save, or a file that changed or went between the check and the save.
        match &self.state {
            DraftState::Saved => {
                undo.written = true;
                Ok(undo)
            }
            DraftState::Failed(error) => Err(ActFault::NotSaved(error.clone(), undo)),
            _ => Err(ActFault::NotSaved(String::new(), undo)),
        }
    }

    /// The note's ID in the words Splat last read from its file.
    fn note_id(&self) -> Option<Uuid> {
        parse_note(&self.path, self.baseline.as_deref()?).map(|note| note.id)
    }
}

/// What a note act says when its save failed and the fragment waits in the draft.
pub const NOT_SAVED: &str = "Notes not saved";

/// How a note act meets the DM's typing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Apply {
    /// An act of the session-cycle design: follow the disk, refuse while unsaved words meet a
    /// changed file, and save at once.
    Now,
    /// Today's plan acts, as they were: the change joins the draft as typed words do, and saves
    /// with them. A changed file then shows the note's conflict.
    WithTyping,
}

/// Why a note act did not change the note, or did not reach the file.
#[derive(Clone, Debug, PartialEq)]
pub enum ActFault {
    /// Splat holds unsaved words and the file changed on disk. Nothing was applied.
    Changed,
    /// The file is gone from the folder. Nothing was applied.
    Missing,
    /// The file now holds another note. Nothing was applied.
    OtherNote,
    /// The fragment's place is gone or changed. Nothing was applied.
    LineChanged,
    /// The act is in the draft and not in the file: the save failed with these words, or the
    /// file changed or went before the save. The note's own state says which. The Undo works.
    NotSaved(String, NoteUndo),
}

impl ActFault {
    /// The words at the act's place.
    pub fn words(&self) -> &'static str {
        match self {
            Self::Changed => "The note changed on disk. Choose in the note first.",
            Self::Missing => "The note is gone from its folder.",
            Self::OtherNote => "The file now holds another note.",
            Self::LineChanged => "That line changed on disk.",
            Self::NotSaved(..) => NOT_SAVED,
        }
    }
}

/// The Undo of one note act. It works while the note's words are the words that the act left,
/// with no time limit. Any other change of the words, in Splat or on disk, ends it, and so does
/// another note. The act's place owns it; nothing keeps it across a restart.
#[derive(Clone, Debug, PartialEq)]
pub struct NoteUndo {
    /// The note's file. It follows a move of that note, never a copy.
    path: Box<Path>,
    /// The session the act was for.
    note: Uuid,
    before: String,
    after: String,
    /// The properties before and after the act, when the act changed them. Boxed, as most
    /// acts change only words.
    properties: Option<Box<(String, String)>>,
    /// Whether the act's save wrote the fragment. One that waits in the draft is taken back
    /// from the draft alone.
    written: bool,
}

impl NoteUndo {
    /// The words that the act left.
    pub fn after(&self) -> &str {
        &self.after
    }

    /// Whether the act's fragment is in the file. One whose save failed waits in the draft.
    pub fn written(&self) -> bool {
        self.written
    }

    /// Whether this Undo still applies to the note in this draft.
    pub fn live(&self, draft: &Draft) -> bool {
        self.owns(draft)
            && draft.body.same_words(&self.after)
            && self.left_properties(&draft.prefix)
    }

    /// The properties are the ones the act left, or the act did not change them.
    fn left_properties(&self, prefix: &str) -> bool {
        self.properties
            .as_ref()
            .is_none_or(|properties| properties.1 == prefix)
    }

    /// Whether the draft is the note that the act changed: its file and its ID.
    fn owns(&self, draft: &Draft) -> bool {
        *draft.path == *self.path && draft.note_id() == Some(self.note)
    }

    /// The folder scan followed this note, the one note with its ID, to a new file.
    pub fn moved(&mut self, from: &Path, to: &Path) {
        if *self.path == *from {
            self.path = to.into();
        }
    }

    /// An act of `Apply::Now` whose save failed: once a later save wrote the draft with the
    /// fragment, the fragment is written, and its Undo goes back through the checked path.
    pub fn follow_save(&mut self, draft: &Draft) {
        if !self.written && !draft.unsaved() && self.live(draft) {
            self.written = true;
        }
    }

    /// Put back the words from before the act. A fragment that waits in the draft leaves the
    /// draft, with no look at the disk, as typed words would. A written one goes back through
    /// the act's own checked path, and is saved at once.
    pub fn undo(&self, draft: &mut Draft) -> Result<(), ActFault> {
        if !self.owns(draft) {
            return Err(ActFault::OtherNote);
        }
        let back = |prefix: &str, words: &str| {
            (words == self.after && self.left_properties(prefix)).then(|| {
                let prefix = self
                    .properties
                    .as_ref()
                    .map_or(prefix, |properties| &properties.0);
                (prefix.to_owned(), self.before.clone())
            })
        };
        if self.written {
            return draft.apply_parts(self.note, Apply::Now, back).map(drop);
        }
        let (prefix, before) = back(&draft.prefix, &draft.body).ok_or(ActFault::LineChanged)?;
        draft.prefix = prefix;
        let key = draft.body.key();
        let _ = draft.body.replace(before);
        draft.own_write(key);
        draft.edited();
        Ok(())
    }
}

/// Where `Add rewards` puts a new block (F7). A block found under another heading counts too.
pub const REWARDS_HEADING: &str = "## Loot and rewards";

/// A note as the rewards record reads it: its real Markdown blocks, read once by [`outline`].
/// Text in a code fence is never a heading and never a block. A block inside a callout or a
/// quote is a block, as the DM sees it.
struct Outline<'a> {
    /// Each heading outside a list and a quote: its level, and its range with its line end.
    headings: Vec<(pulldown_cmark::HeadingLevel, std::ops::Range<usize>)>,
    /// Each rewards block, in the note's order.
    blocks: Vec<Block<'a>>,
    /// Each code block, with its fences.
    code: Vec<std::ops::Range<usize>>,
}

impl Outline<'_> {
    /// Where the section that holds `at` ends: at the next heading of level 1 or 2.
    fn section_end(&self, at: usize, text: &str) -> usize {
        self.headings
            .iter()
            .find(|(level, range)| range.start >= at && *level <= pulldown_cmark::HeadingLevel::H2)
            .map_or(text.len(), |(_, range)| range.start)
    }
}

/// One rewards block: a list item whose words start with a Timeline reference,
/// `- [[Timeline#^t-…|Name]]`, with the items under it.
struct Block<'a> {
    id: &'a str,
    /// Byte range of the whole item, to the line end of its last line. A blank line after it
    /// is not its own.
    range: std::ops::Range<usize>,
    /// The words of its first item under it that start with `XP`.
    xp: Option<&'a str>,
}

/// The Timeline ID that words start with: `[[Timeline#^t-…|Name]]`.
fn reference(words: &str) -> Option<&str> {
    let rest = words.strip_prefix("[[Timeline#^")?;
    let end = rest.find(['|', ']'])?;
    Some(rest[..end].trim())
}

/// The one reading of a note's structure for the rewards record (F7): insertion, `In notes`,
/// the XP choice and Undo all ask it.
fn outline(text: &str) -> Outline<'_> {
    use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
    /// A list item being read: its range, where its words start, and its `XP` item.
    struct Open<'a> {
        range: std::ops::Range<usize>,
        words: Option<usize>,
        xp: Option<&'a str>,
    }
    let line_at = |at: usize| text[at..].lines().next().unwrap_or("").trim();
    let mut out = Outline {
        headings: Vec::new(),
        blocks: Vec::new(),
        code: Vec::new(),
    };
    let mut items: Vec<Open> = Vec::new();
    let mut quotes = 0;
    let options = Options::ENABLE_TABLES | Options::ENABLE_WIKILINKS;
    for (event, range) in Parser::new_ext(text, options).into_offset_iter() {
        match event {
            Event::Start(Tag::Item) => items.push(Open {
                range,
                words: None,
                xp: None,
            }),
            Event::Start(Tag::Paragraph) => {}
            Event::Start(Tag::CodeBlock(_)) => out.code.push(range),
            Event::Start(Tag::BlockQuote(_)) => quotes += 1,
            Event::End(TagEnd::BlockQuote(_)) => quotes -= 1,
            Event::Start(Tag::Heading { level, .. }) if items.is_empty() && quotes == 0 => {
                out.headings.push((level, range));
            }
            Event::End(TagEnd::Item) => {
                let Some(item) = items.pop() else {
                    continue;
                };
                let words = item.words.map_or("", line_at);
                if let Some(parent) = items.last_mut()
                    && parent.xp.is_none()
                    && (words == "XP" || words.starts_with("XP "))
                {
                    parent.xp = Some(words);
                }
                if let Some(id) = reference(words) {
                    let end = item.range.start + text[item.range.clone()].trim_end().len();
                    let end = text[end..].find('\n').map_or(text.len(), |n| end + n + 1);
                    out.blocks.push(Block {
                        id,
                        range: item.range.start..end,
                        xp: item.xp,
                    });
                }
            }
            _ => {
                if let Some(item) = items.last_mut() {
                    item.words.get_or_insert(range.start);
                }
            }
        }
    }
    // An item ends after the items inside it.
    out.blocks.sort_by_key(|block| block.range.start);
    out
}

/// Each Timeline ID that a rewards block of the note starts with: the fights whose rewards
/// are in the note, wherever the block is.
pub fn listed_references(text: &str) -> std::collections::BTreeSet<String> {
    outline(text)
        .blocks
        .iter()
        .map(|block| block.id.to_owned())
        .collect()
}

/// Whether the next rewards block gets an XP line, from the newest block that Splat can read
/// (F7: XP has no switch). `notes` are the session notes in order: this one first, then each
/// earlier one, newest first; `None` is a note that cannot be read. A note with no block is
/// passed. An unreadable note stops the search in its place: what it holds is not known, and an
/// older block is never used in its stead. `None` when nothing is known. `fight` says which
/// Timeline IDs are fights; a block for another event is not a rewards block.
pub fn rewards_with_xp<'a>(
    notes: impl IntoIterator<Item = Option<&'a str>>,
    fight: &dyn Fn(&str) -> bool,
) -> Option<bool> {
    for note in notes {
        if let Some(block) = outline(note?).blocks.iter().rfind(|block| fight(block.id)) {
            return Some(block.xp.is_some());
        }
    }
    None
}

/// The words of the session notes before `session`, newest first, read now from `folder`, for
/// [`rewards_with_xp`]. A note that cannot be read keeps its place as `None`. A file of the
/// folder's one scan, subfolders too, that claims a session but whose properties cannot be
/// read, or that cannot be read at all, has no known place: it could be the newest, so it
/// stands first. `None` alone when `session` is not in the folder.
pub fn earlier_notes(folder: &Path, session: Uuid) -> Vec<Option<String>> {
    let notes = list_notes(folder);
    let Some(at) = notes.iter().position(|note| note.id == session) else {
        return vec![None];
    };
    let unplaced = note_paths(folder)
        .iter()
        .any(|path| fs::read_to_string(path).map_or(true, |text| unreadable_note(path, &text)))
        .then_some(None);
    unplaced
        .into_iter()
        .chain(notes[at + 1..].iter().map(|note| {
            fs::read_to_string(&note.path)
                .ok()
                .map(|text| body_of(&text).to_owned())
        }))
        .collect()
}

/// The session's XP as the note stands: the first number of each block's XP line, and how many
/// blocks it sums. Splat stores no total.
pub fn xp_in_note(text: &str, fight: &dyn Fn(&str) -> bool) -> (u64, usize) {
    outline(text)
        .blocks
        .iter()
        .filter(|block| fight(block.id))
        .filter_map(|block| first_number(block.xp?))
        .fold((0, 0), |(sum, count), xp| {
            (sum.saturating_add(xp), count + 1)
        })
}

/// A rewards block that `Add rewards` wrote, as it left it, so that its Undo takes back exactly
/// it. `lead` is what the act put before the block: a line end, the heading, or nothing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rewarded {
    pub id: String,
    lead: String,
    /// Whether `lead` holds the heading that the act added.
    heading: bool,
    block: String,
}

/// Why `Add rewards` wrote nothing: the block would be in a code block that is not closed.
pub const REWARDS_IN_CODE: &str =
    "A code block in the note is not closed, so nothing was written. Close it, then add rewards.";
/// Why `Add rewards` wrote nothing, when no other reason is known.
pub const REWARDS_NOT_READ: &str =
    "The note would not show these rewards as a list, so nothing was written.";

/// Put a block for the fight `id` at the end of `## Loot and rewards`, after its last line,
/// or add that heading at the end of the note. `block` is whole lines, each ending in `\n`.
/// The new words are read again first: unless they hold exactly this block, as the act's
/// one block for the fight, under that heading, nothing is written and the reason is given.
pub fn add_rewards(
    text: &str,
    id: &str,
    block: &str,
) -> Result<(String, Rewarded, usize), &'static str> {
    let found = outline(text);
    let heading = found.headings.iter().find(|(_, range)| {
        text[range.clone()]
            .trim()
            .eq_ignore_ascii_case(REWARDS_HEADING)
    });
    let (at, lead) = match heading {
        Some((_, heading)) => {
            // The section ends at the next heading of its level or higher.
            let end = found.section_end(heading.end, text);
            let words = text[heading.end..end].trim_end().len();
            let under_heading = words == 0;
            // After the line end of its last line, which can be the file's last, with none.
            let at = heading.end + words;
            let at = if under_heading {
                at
            } else {
                text[at..].find('\n').map_or(text.len(), |n| at + n + 1)
            };
            let end = if text[..at].ends_with('\n') { "" } else { "\n" };
            // A blank line between the heading and its first block.
            let blank = if under_heading { "\n" } else { "" };
            (at, format!("{end}{blank}"))
        }
        None => {
            let gap = if text.is_empty() || text.ends_with("\n\n") {
                ""
            } else if text.ends_with('\n') {
                "\n"
            } else {
                "\n\n"
            };
            (text.len(), format!("{gap}{REWARDS_HEADING}\n\n"))
        }
    };
    let after = format!("{}{lead}{block}{}", &text[..at], &text[at..]);
    let written = Rewarded {
        id: id.to_owned(),
        heading: heading.is_none(),
        lead,
        block: block.to_owned(),
    };
    let block_at = at + written.lead.len();
    let read = outline(&after);
    let mut matching = read.blocks.iter().filter(|found| found.id == id);
    let section = read.headings.iter().find(|(_, range)| {
        after[range.clone()]
            .trim()
            .eq_ignore_ascii_case(REWARDS_HEADING)
    });
    let ours = matching.next().is_some_and(|found| {
        found.range == (block_at..block_at + block.len())
            && section.is_some_and(|(_, heading)| {
                heading.end <= block_at && block_at < read.section_end(heading.end, &after)
            })
    }) && matching.next().is_none();
    if !ours {
        let in_code = read.code.iter().any(|code| code.contains(&block_at));
        return Err(if in_code {
            REWARDS_IN_CODE
        } else {
            REWARDS_NOT_READ
        });
    }
    Ok((after, written, block_at))
}

/// The note with the block taken back, while it is still the complete, unchanged list item
/// that `Add rewards` wrote: the one block with its fight's ID holds exactly those lines, and
/// no item, also after a blank line, joined it. The bytes from the start of its first line to
/// the end of its last line must be the act's bytes: a quote mark or an indent before it
/// makes them differ. The heading that the act added goes too while
/// nothing else is under it. Another lead goes while no line follows the block before a blank
/// line or the end, or the blank line of a block that it followed and that is gone. `None`
/// when the block changed: then nothing is undone.
pub fn remove_rewards(text: &str, written: &Rewarded) -> Option<String> {
    let found = outline(text);
    let mut matching = found.blocks.iter().filter(|block| block.id == written.id);
    let block = matching.next()?;
    let line = text[..block.range.start].rfind('\n').map_or(0, |n| n + 1);
    if matching.next().is_some() || text[line..block.range.end] != written.block {
        return None;
    }
    let rest = &text[block.range.end..];
    let before = &text[..block.range.start];
    let alone = if written.heading {
        text[block.range.end..found.section_end(block.range.end, text)]
            .trim()
            .is_empty()
    } else {
        rest.lines()
            .next()
            .is_none_or(|line| line.trim().is_empty())
    };
    let start = if !alone {
        block.range.start
    } else if !written.lead.is_empty() && before.ends_with(&written.lead) {
        block.range.start - written.lead.len()
    } else if written.lead.is_empty() && before.ends_with("\n\n") {
        // It followed a block that is gone: the blank line above it was that block's.
        block.range.start - 1
    } else {
        block.range.start
    };
    Some(format!("{}{rest}", &text[..start]))
}

#[cfg(test)]
mod tests {

    /// Round 30 B7d: a note's failed save says plain words from the error's kind, one sentence
    /// each, ending with a full stop, never the system's words; the draft's own write uses it.
    #[test]
    fn a_failed_note_save_says_plain_words_from_the_error_kind() {
        use std::io::ErrorKind;
        let permission = "Not saved. No permission to save. Give your user account permission to change the file and its folder.";
        for (kind, words) in [
            (
                ErrorKind::StorageFull,
                "Not saved. Disk is full. Free some space on the disk, then try again.",
            ),
            (ErrorKind::PermissionDenied, permission),
            (
                ErrorKind::ReadOnlyFilesystem,
                "Not saved. Disk is read-only. Make the disk writable, or move the campaign to a disk you can write to.",
            ),
            (
                ErrorKind::Interrupted,
                "Not saved. File cannot be saved. Check that the disk that holds it is connected, then try again.",
            ),
        ] {
            let said = not_saved(&std::io::Error::new(kind, "raw words (os error 99)"));
            assert_eq!(said, words, "{kind:?}");
            assert!(
                !said.contains("os error") && !said.contains("raw"),
                "{said}"
            );
        }
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("Session 1.md");
        fs::write(&path, "---\nsession_id: 1\n---\n# One\n").unwrap();
        let mut draft = Draft::open(&path).unwrap();
        draft.failing_write = Some(28);
        draft.body.push_str("More words.\n");
        draft.edited();
        draft.save();
        assert_eq!(
            draft.state,
            DraftState::Failed(
                "Not saved. Disk is full. Free some space on the disk, then try again.".into()
            )
        );
    }
    use super::*;

    /// F7: a block goes at the end of `## Loot and rewards`, or under that heading added at the
    /// end; its Undo takes back exactly what the act wrote, also beside another act's change.
    #[test]
    fn a_rewards_block_and_its_undo_are_exact_fragments() {
        let a = "- [[Timeline#^t-a|Bridge]]\n  - XP 200 for 1 at 0 HP\n  - 30 gp\n";
        let b = "- [[Timeline#^t-b|Shrine]]\n  - Iron key\n";
        let template =
            "# Session 2\n\n## What happened\n\n## Loot and rewards\n\n## Open threads\n\n";
        let (one, written, at) = add_rewards(template, "t-a", a).unwrap();
        assert_eq!(
            one,
            "# Session 2\n\n## What happened\n\n## Loot and rewards\n\n- [[Timeline#^t-a|Bridge]]\n  - XP 200 for 1 at 0 HP\n  - 30 gp\n\n## Open threads\n\n"
        );
        assert_eq!(&one[at..at + a.len()], a);
        // The next block follows the first.
        let (two, second, _) = add_rewards(&one, "t-b", b).unwrap();
        assert!(two.contains(&format!("\n\n{a}{b}\n## Open threads")));
        // A: add, B: another act on another line, then Undo of A. B's change stays.
        let other = two.replace("## What happened\n", "## What happened\n\nThe ford held.\n");
        let undone = remove_rewards(&other, &written).expect("the block stands");
        assert_eq!(
            undone,
            template
                .replace("## What happened\n", "## What happened\n\nThe ford held.\n")
                .replace(
                    "## Loot and rewards\n\n",
                    &format!("## Loot and rewards\n\n{b}\n")
                )
        );
        assert_eq!(
            remove_rewards(&undone, &second).unwrap(),
            template.replace("## What happened\n", "## What happened\n\nThe ford held.\n")
        );
        // A block that the DM changed, or typed under, is no longer the act's: no Undo.
        assert_eq!(
            remove_rewards(&one.replace("30 gp", "40 gp"), &written),
            None
        );
        assert_eq!(
            remove_rewards(
                &one.replace("  - 30 gp\n", "  - 30 gp\n  - A signet ring\n"),
                &written
            ),
            None
        );
        // With no heading, the heading is added at the end, and Undo takes it back too, also
        // from a note with no line end at its end.
        for note in [
            "# Session 3\n\nWords.\n",
            "# Session 3\n\nWords.",
            "",
            "# S\n\n",
        ] {
            let (with, written, _) = add_rewards(note, "t-a", a).unwrap();
            assert!(
                with.ends_with(&format!("{REWARDS_HEADING}\n\n{a}")),
                "{with:?}"
            );
            assert_eq!(remove_rewards(&with, &written).as_deref(), Some(note));
        }
        // A heading that is the note's last line, and a section whose last line has no end.
        for note in [
            "# S\n\n## Loot and rewards",
            "# S\n\n## Loot and rewards\n\nTyped",
        ] {
            let (with, written, _) = add_rewards(note, "t-a", a).unwrap();
            assert!(with.ends_with(&format!("\n{a}")), "{with:?}");
            assert_eq!(remove_rewards(&with, &written).as_deref(), Some(note));
        }
        // The fights listed in a note, wherever the line is; a link in prose is not a list line.
        let listed = listed_references(
            "Words [[Timeline#^t-c|C]]\n## X\n  * [[Timeline#^t-b]]\n- [[Timeline#^t-a|A]] and more\n",
        );
        assert_eq!(listed.into_iter().collect::<Vec<_>>(), ["t-a", "t-b"]);
        // Text in a code fence is never a heading and never a block.
        let fenced =
            "# Session\n\n```markdown\n## Loot and rewards\n```\n\n## Open threads\n\nKeep this.\n";
        let (with, _, _) = add_rewards(fenced, "t-a", a).unwrap();
        assert!(
            with.ends_with(&format!("\n{REWARDS_HEADING}\n\n{a}")),
            "a fenced heading is not the section: {with:?}"
        );
        assert!(
            listed_references(&format!("```markdown\n{a}```\n")).is_empty(),
            "a code example is not a block"
        );
        // A block that the DM moved into a callout is still the fight's block.
        let callout = format!(
            "> [!note] Rewards\n>{}",
            a.lines().map(|l| format!(" {l}\n>")).collect::<String>()
        );
        assert!(listed_references(&callout).contains("t-a"), "{callout:?}");
        // An item that the DM typed under the block after a blank line is the block's, as
        // CommonMark reads it: Undo takes back nothing.
        let (alone, written, _) = add_rewards("# Session\n", "t-a", a).unwrap();
        let child = format!("{alone}\n  - Varra's signet ring\n");
        assert_eq!(remove_rewards(&child, &written), None, "{child:?}");
        // Undo compares the bytes from the start of the block's first line to the end of its
        // last line: a quote mark or an indent before a one-line block makes them differ.
        let single = "- [[Timeline#^t-a|Bridge]]\n";
        let (_, one_line, _) = add_rewards("# Session\n", "t-a", single).unwrap();
        for moved in [
            format!("> {single}## Keep this heading\nKeep this.\n"),
            format!("- Parent\n  {single}\nKeep this.\n"),
        ] {
            assert_eq!(remove_rewards(&moved, &one_line), None, "{moved:?}");
        }
        // Rewards for A, then B, Undo A, Undo B: the heading that A's act added stays. Only A's
        // Undo knew that it was the act's; its bytes are those of a template's heading.
        let (two, second, _) = add_rewards(&alone, "t-b", &a.replace("t-a", "t-b")).unwrap();
        let without_a = remove_rewards(&two, &written).unwrap();
        assert_eq!(
            remove_rewards(&without_a, &second).as_deref(),
            Some("# Session\n\n## Loot and rewards\n")
        );
        // The act reads its new words again: a block that would not be the fight's block under
        // the heading is refused, with the reason when it is known.
        for (note, why) in [
            ("# Session\n\n```\nunfinished example\n", REWARDS_IN_CODE),
            ("## Loot and rewards\n\n~~~\nopen\n", REWARDS_IN_CODE),
            ("# Session\n\n<!-- a draft\n", REWARDS_NOT_READ),
        ] {
            assert_eq!(add_rewards(note, "t-a", a), Err(why), "{note:?}");
        }
        // Words typed under the act's heading keep it: Undo takes back the block alone.
        let words = format!("{alone}\nVarra's signet ring\n");
        assert_eq!(
            remove_rewards(&words, &written).as_deref(),
            Some("# Session\n\n## Loot and rewards\n\n\nVarra's signet ring\n")
        );
    }

    /// F7: XP has no switch. The newest block that Splat can read says whether the next one
    /// has XP; an unreadable note stops the search in its place.
    #[test]
    fn the_newest_rewards_block_says_whether_the_next_has_xp() {
        let fight = |id: &str| id != "t-rest";
        let with = "## Loot and rewards\n\n- [[Timeline#^t-1|One]]\n  - XP 1,250 for 3 at 0 HP · 250 each for 5\n";
        let without = "## Loot and rewards\n\n- [[Timeline#^t-2|Two]]\n  - 30 gp\n";
        let rest = "- [[Timeline#^t-rest|Short rest]]\n";
        let read = |notes: &[Option<&str>]| rewards_with_xp(notes.iter().copied(), &fight);
        assert_eq!(read(&[Some(with)]), Some(true));
        assert_eq!(read(&[Some(""), Some(without), Some(with)]), Some(false));
        // The last block of a note is its newest; an event's line is not a block.
        assert_eq!(read(&[Some(&format!("{without}{with}{rest}"))]), Some(true));
        assert_eq!(read(&[Some(rest), Some(without)]), Some(false));
        // Nothing found, or an unreadable note before any block: not known.
        assert_eq!(read(&[Some(""), Some(rest)]), None);
        assert_eq!(read(&[Some(""), None, Some(with)]), None);
        // The sum reads the first number of each block's XP line, as the note stands.
        let two = format!(
            "{with}- [[Timeline#^t-3|Three]]\n  - XP 400 for 2 at 0 HP · 1 still stood (XP 200)\n{rest}"
        );
        assert_eq!(xp_in_note(&two, &fight), (1650, 2));
        assert_eq!(xp_in_note(without, &fight), (0, 0));

        // The notes in their order, read now: this one is not among them; one that cannot be
        // read keeps its place, and one whose place is not known stands first.
        let dir = tempfile::tempdir().unwrap();
        let note = |number: u32, body: &str| {
            create_note(
                dir.path(),
                &NewNote {
                    number,
                    title: "",
                    date: "2026-11-01",
                    campaign: "Marches",
                    template: body,
                },
            )
            .unwrap()
        };
        note(1, with);
        let second = note(2, "");
        note(3, without);
        let fourth = note(4, "");
        let found = earlier_notes(dir.path(), fourth.id);
        assert_eq!(found.len(), 3);
        assert_eq!(
            rewards_with_xp(found.iter().map(Option::as_deref), &fight),
            Some(false)
        );
        assert!(found.iter().all(Option::is_some));
        assert_eq!(
            rewards_with_xp(
                earlier_notes(dir.path(), second.id)
                    .iter()
                    .map(Option::as_deref),
                &fight
            ),
            Some(true)
        );
        fs::write(dir.path().join("Broken.md"), "---\nsession_id: [\n---\n").unwrap();
        let found = earlier_notes(dir.path(), fourth.id);
        assert_eq!(found[0], None);
        assert_eq!(
            rewards_with_xp(found.iter().map(Option::as_deref), &fight),
            None
        );
        // Also in a subfolder of the folder's one scan.
        fs::remove_file(dir.path().join("Broken.md")).unwrap();
        fs::create_dir(dir.path().join("Archive")).unwrap();
        fs::write(
            dir.path().join("Archive/Session 2.md"),
            "---\nsession_id: [\n---\n",
        )
        .unwrap();
        let found = earlier_notes(dir.path(), fourth.id);
        assert_eq!(found[0], None);
        assert_eq!(
            rewards_with_xp(found.iter().map(Option::as_deref), &fight),
            None,
            "a session with unreadable properties stops the search in a subfolder too"
        );
        assert_eq!(earlier_notes(dir.path(), Uuid::nil()), [None]);
    }

    #[test]
    fn integrity_d10_new_notes_and_templates_refuse_a_gone_folder() {
        let dir = tempfile::tempdir().unwrap();
        let gone = dir.path().join("Gone");
        assert!(create_note(&gone, &new_note(1, "", "Words")).is_err());
        assert!(create_template(&gone, "Notes", "Words").is_err());
        assert!(!gone.exists());
        // Round 30 B7e: a new template the disk refuses says plain words, never the system's,
        // and leaves no file.
        FAILING_WRITES.with(|code| code.set(Some(28)));
        let refused = create_template(dir.path(), "Notes", "Words").unwrap_err();
        FAILING_WRITES.with(|code| code.set(None));
        assert_eq!(
            refused,
            "Not created. Disk is full. Free some space on the disk, then try again."
        );
        assert!(!dir.path().join(TEMPLATES).join("Notes.md").exists());
    }

    #[test]
    fn integrity_d14_save_again_keeps_an_arriving_file() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(1, "", "Before")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        fs::remove_file(&note.path).unwrap();
        let _ = draft.body.replace("My draft");
        draft.edited();
        draft.save();
        fs::write(&note.path, "DM's restored file").unwrap();
        draft.save_again();
        assert!(draft.unsaved());
        assert_eq!(
            fs::read_to_string(&note.path).unwrap(),
            "DM's restored file"
        );
        // Review of PR 255, item 1: through Discard, a failed creation, Undo and a retry, the
        // arriving file keeps its bytes and the draft keeps its identity until a file is made.
        let note = create_note(dir.path(), &new_note(2, "", "Before")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        fs::remove_file(&note.path).unwrap();
        let _ = draft.body.replace("My draft");
        draft.edited();
        draft.save();
        assert_eq!(draft.state, DraftState::Missing);
        let identity = draft.prefix.clone();
        draft.discard();
        fs::write(&note.path, "Arrived outside").unwrap();
        draft.save_again();
        assert_eq!(fs::read_to_string(&note.path).unwrap(), "Arrived outside");
        assert_eq!(draft.prefix, identity);
        assert_eq!(draft.restored, None);
        assert_eq!(draft.state, DraftState::Missing);
        draft.undo_dropped();
        assert_eq!(draft.body.as_str(), "My draft");
        draft.save();
        assert_eq!(fs::read_to_string(&note.path).unwrap(), "Arrived outside");
        draft.save_again();
        assert_eq!(fs::read_to_string(&note.path).unwrap(), "Arrived outside");
        assert_eq!(draft.prefix, identity);
        assert_eq!(draft.restored, None);
        assert!(draft.unsaved() && draft.needs_attention());
        // When the place is free again, the retry makes the file, with a new identity.
        fs::remove_file(&note.path).unwrap();
        draft.save_again();
        assert_eq!(draft.state, DraftState::Saved);
        let id = draft.restored.expect("a new identity");
        assert_ne!(id, note.id);
        let saved = fs::read_to_string(&note.path).unwrap();
        assert!(saved.contains("My draft") && saved.contains(&id.to_string()));
    }

    #[test]
    fn integrity_d14_moves_are_found_across_session_subfolders() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(1, "", "Before")).unwrap();
        let past = dir.path().join("Past");
        fs::create_dir(&past).unwrap();
        let nested = past.join("Meeting.md");
        fs::rename(&note.path, &nested).unwrap();
        let mut draft = Draft::open(&nested).unwrap();
        let moved = dir.path().join("Moved.md");
        fs::rename(&nested, &moved).unwrap();
        let _ = draft.body.replace("My draft");
        draft.edited();
        draft.save();
        assert_eq!(draft.moved_to.as_deref(), Some(moved.as_path()));
        draft.save_moved();
        assert_eq!(draft.state, DraftState::Saved);
        assert!(fs::read_to_string(moved).unwrap().contains("My draft"));
    }

    #[test]
    fn integrity_d15_an_edit_during_copy_is_never_deleted() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.md");
        let target = dir.path().join("target.md");
        fs::write(&source, "Before").unwrap();
        let error = transfer_file(&source, &target, true, || {
            fs::write(&source, "DM's edit").unwrap()
        })
        .unwrap_err();
        assert!(error.words(false).contains("Both files were kept"));
        assert_eq!(fs::read_to_string(&source).unwrap(), "DM's edit");
        assert_eq!(fs::read_to_string(&target).unwrap(), "Before");
        // Round 30 B7e: a transfer the system refuses says plain words from the error's kind,
        // never the system's, and moves nothing.
        let other = dir.path().join("other.md");
        let fail = |at: Option<(u32, i32, FailAt)>| FAILING_TRANSFERS.with(|f| f.set(at));
        fail(Some((0, 28, FailAt::Start)));
        let refused = transfer_file(&source, &other, true, || {}).unwrap_err();
        fail(None);
        let words = refused.words(false);
        assert!(
            words.starts_with("source.md was not moved. It is still in ")
                && words.ends_with("Disk is full. Free some space on the disk, then try again."),
            "{words}"
        );
        assert_eq!(fs::read_to_string(&source).unwrap(), "DM's edit");
        assert!(!other.exists());
        // Round 30 B7e (review F2 of PR 431): a late failure says where the file is now.
        let (from, to) = (dir.path().join("From"), dir.path().join("To"));
        fs::create_dir_all(&from).unwrap();
        fs::create_dir_all(&to).unwrap();
        // The move is done, and writing the folder through fails: the file is in To.
        fs::write(from.join("a.md"), "A").unwrap();
        fail(Some((0, 28, FailAt::Sync)));
        let late = transfer_file(&from.join("a.md"), &to.join("a.md"), false, || {}).unwrap_err();
        fail(None);
        assert_eq!(
            late.words(false),
            "a.md is now in To, but Splat could not finish writing that folder. Disk is full. \
             Free some space on the disk, then try again."
        );
        assert!(to.join("a.md").exists() && !from.join("a.md").exists());
        // The copy is written, and the original cannot be removed: it is in both folders.
        fs::write(from.join("b.md"), "B").unwrap();
        fail(Some((0, 13, FailAt::Remove)));
        let both = transfer_file(&from.join("b.md"), &to.join("b.md"), true, || {}).unwrap_err();
        fail(None);
        assert_eq!(
            both.words(false),
            "b.md was copied to To but not removed from From, so it is in both. No permission to \
             save. Give your user account permission to change the file and its folder. Then \
             delete the copy you do not need."
        );
        assert_eq!(fs::read_to_string(from.join("b.md")).unwrap(), "B");
        assert_eq!(fs::read_to_string(to.join("b.md")).unwrap(), "B");
        // Review F2 of PR 431 (re-check): an original that is gone says only what is known.
        // Gone before the move:
        fail(Some((0, 2, FailAt::Start)));
        let gone = transfer_file(&from.join("c.md"), &to.join("c.md"), false, || {}).unwrap_err();
        fail(None);
        assert_eq!(
            gone.words(false),
            "c.md was not moved: it is no longer in From. It was moved or deleted outside Splat."
        );
        // Gone while it was copied: the copy is in To.
        fs::write(from.join("d.md"), "D").unwrap();
        let gone = transfer_file(&from.join("d.md"), &to.join("d.md"), true, || {
            fs::remove_file(from.join("d.md")).unwrap()
        })
        .unwrap_err();
        assert_eq!(
            gone.words(false),
            "d.md was not moved: it is no longer in From. It was moved or deleted outside Splat. \
             A copy is in To."
        );
        assert_eq!(fs::read_to_string(to.join("d.md")).unwrap(), "D");
        // Gone when the original is removed after the copy:
        fs::write(from.join("e.md"), "E").unwrap();
        fail(Some((0, 2, FailAt::Remove)));
        let gone = transfer_file(&from.join("e.md"), &to.join("e.md"), true, || {}).unwrap_err();
        fail(None);
        assert_eq!(
            gone.words(true),
            "e.md was not moved back: it is no longer in From. It was moved or deleted outside \
             Splat. A copy is in To."
        );
    }

    #[test]
    fn integrity_d16_links_keep_code_and_other_paths_and_follow_headings() {
        let mut text = "[[Ash Pack#Members|the pack]] `[[Ash Pack]]`\n```md\n[[Ash Pack]]\n```\n[[Places/Ash Pack]]\n".to_owned();
        rename_links(&mut text, "Ash Pack", "Cinder Pack");
        assert_eq!(
            text,
            "[[Cinder Pack#Members|the pack]] `[[Ash Pack]]`\n```md\n[[Ash Pack]]\n```\n[[Places/Ash Pack]]\n"
        );
    }

    #[test]
    fn integrity_d9_new_identity_and_undo_keep_both_files() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(3, "", "DM's words")).unwrap();
        let copy = dir.path().join("Session 4.md");
        fs::copy(&note.path, &copy).unwrap();
        let before = fs::read(&copy).unwrap();
        let undo = new_identity(&copy).unwrap();
        assert_eq!(list_notes(dir.path()).len(), 2);
        assert!(fs::read_to_string(&copy).unwrap().contains("DM's words"));
        undo.undo().unwrap();
        assert_eq!(fs::read(copy).unwrap(), before);
        assert_eq!(list_notes(dir.path()).len(), 1);
    }

    #[test]
    fn integrity_d9_a_shorter_copy_cannot_take_over_the_known_session() {
        let dir = tempfile::tempdir().unwrap();
        let note =
            create_note(dir.path(), &new_note(3, "A long meeting name", "My notes")).unwrap();
        fs::copy(&note.path, dir.path().join("Session 4.md")).unwrap();
        let notes = list_notes(dir.path());
        assert_eq!(notes[0].path, note.path);
        assert_eq!(notes[0].also.len(), 1);
    }

    #[test]
    fn integrity_d10_a_missing_configured_session_folder_blocks_access() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::from_u128(9),
            name: "Marches".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
        let path = save_folder(&campaign, "Meetings").unwrap();
        fs::create_dir_all(&path).unwrap();
        fs::rename(&path, dir.path().join("Moved meetings")).unwrap();
        // Routing row 136: the fault is typed, and its words are a sentence that names the path.
        let fault = folder_typed(&campaign).unwrap_err();
        assert_eq!(fault, page::FolderFault::Missing(path.clone()));
        let error = folder(&campaign).unwrap_err();
        assert!(
            error.starts_with("Splat cannot find the session folder ")
                && error.contains(&path.display().to_string())
                && error.ends_with("It may have been moved or renamed."),
            "{error}"
        );
        assert!(!path.exists());
    }

    #[test]
    fn integrity_d12_recordings_keep_results_without_audio() {
        let dir = tempfile::tempdir().unwrap();
        let id = Uuid::from_u128(3);
        let folder = recordings_folder(dir.path(), id);
        fs::create_dir_all(&folder).unwrap();
        fs::write(folder.join("evening.transcript.md"), "Words").unwrap();
        fs::write(folder.join("evening.summary.md"), "Summary").unwrap();
        let rows = list_recordings(dir.path(), id);
        assert_eq!(rows.len(), 1);
        assert!(rows[0].transcript && rows[0].summary);
        assert!(!rows[0].path.exists());
        // Round 30 B7e (review F3 of PR 431): a removal the system refuses says plain words,
        // never the system's, and the file it could not remove stays.
        FAILING_REMOVES.with(|code| code.set(Some(13)));
        let refused = delete_recording(&rows[0].path).unwrap_err();
        FAILING_REMOVES.with(|code| code.set(None));
        assert_eq!(
            refused,
            "Not deleted. Splat has no permission to delete evening.transcript.md. Give your user \
             account permission to change the file and its folder."
        );
        assert_eq!(
            fs::read_to_string(folder.join("evening.transcript.md")).unwrap(),
            "Words"
        );
        delete_recording(&rows[0].path).unwrap();
        assert!(list_recordings(dir.path(), id).is_empty());
    }

    #[test]
    fn a_new_name_follows_every_link_and_leaves_other_words() {
        let mut body =
            "- [[Ash Pack]]\n- [[Ash Pack|the pack]]\nAsh Pack is not a link.\n- [[Ash Packs]]\n"
                .to_owned();
        assert!(rename_links(&mut body, "Ash Pack", "Cinder Pack"));
        assert_eq!(
            body,
            "- [[Cinder Pack]]\n- [[Cinder Pack|the pack]]\nAsh Pack is not a link.\n- [[Ash Packs]]\n"
        );
        assert!(!rename_links(&mut body, "Ash Pack", "Other"));
    }

    #[test]
    fn a_new_name_follows_a_link_of_any_case_and_a_link_left_open_stays() {
        let mut body = "- [[ash pack]]\n- [[ASH PACK|them]]\n[[Ash Pack".to_owned();
        assert!(rename_links(&mut body, "Ash Pack", "Cinder Pack"));
        assert_eq!(
            body,
            "- [[Cinder Pack]]\n- [[Cinder Pack|them]]\n[[Ash Pack"
        );
    }

    #[test]
    fn a_note_not_open_keeps_its_frontmatter_and_its_words_through_a_rename() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Session 1.md");
        let text = "---\nsession_title: '[[Ash Pack]]'\n---\n\nWords.\n- [[Ash Pack]]\n";
        fs::write(&path, text).unwrap();
        assert!(rename_links_in_file(&path, "Ash Pack", "Cinder Pack").unwrap());
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            "---\nsession_title: '[[Ash Pack]]'\n---\n\nWords.\n- [[Cinder Pack]]\n"
        );
        assert!(!rename_links_in_file(&path, "Ash Pack", "Other").unwrap());
    }

    fn new_note<'a>(number: u32, title: &'a str, template: &'a str) -> NewNote<'a> {
        NewNote {
            number,
            title,
            date: "2026-11-01",
            campaign: "The Marches",
            template,
        }
    }

    #[test]
    fn a_note_is_created_from_a_template_and_reads_back() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(
            dir.path(),
            &new_note(
                12,
                "The Bridge: at dusk?",
                "# {{number}} {{title}}\n{{date}} {{campaign}} {{unknown}}\n",
            ),
        )
        .unwrap();
        assert_eq!(note.number, 12);
        assert_eq!(note.title, "The Bridge: at dusk?");
        assert_eq!(note.link(), "Session 12 - The Bridge at dusk");
        let text = fs::read_to_string(&note.path).unwrap();
        assert!(
            text.ends_with("# 12 The Bridge: at dusk?\n2026-11-01 The Marches {{unknown}}\n"),
            "{text}"
        );
        assert_eq!(list_notes(dir.path()), [note]);
        // An idle scan writes nothing: the index of note paths keeps its file (W9-D2).
        #[cfg(unix)]
        {
            let index = dir.path().join(NOTE_PATHS);
            let file =
                |path: &Path| std::os::unix::fs::MetadataExt::ino(&fs::metadata(path).unwrap());
            let written = file(&index);
            remember_notes(dir.path(), &list_notes(dir.path())).unwrap();
            assert_eq!(file(&index), written, "an idle scan replaced the index");
        }
        // Round 30 B7d (review F2 of PR 416): a failed Create session says plain words, never
        // the system's, and leaves no file.
        FAILING_WRITES.with(|code| code.set(Some(28)));
        let refused = create_note(dir.path(), &new_note(13, "Full", "Words")).unwrap_err();
        FAILING_WRITES.with(|code| code.set(None));
        assert_eq!(
            refused,
            "Not created. Disk is full. Free some space on the disk, then try again."
        );
        assert_eq!(list_notes(dir.path()).len(), 1);
    }

    #[test]
    fn a_note_never_looks_like_a_library_entry_to_the_catalogue() {
        let dir = tempfile::tempdir().unwrap();
        create_note(dir.path(), &new_note(1, "", "Body")).unwrap();
        let (entries, errors) = splat::content::read_directory(dir.path()).unwrap();
        assert!(entries.is_empty() && errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn notes_list_newest_first_and_leave_other_markdown_out() {
        let dir = tempfile::tempdir().unwrap();
        create_note(dir.path(), &new_note(2, "", "")).unwrap();
        create_note(dir.path(), &new_note(10, "", "")).unwrap();
        fs::write(
            dir.path().join("Ideas.md"),
            "---\ntag: x\n---\nNot a session",
        )
        .unwrap();
        fs::write(dir.path().join("Plain.md"), "No frontmatter").unwrap();
        let numbers: Vec<u32> = list_notes(dir.path()).iter().map(|n| n.number).collect();
        assert_eq!(numbers, [10, 2]);
    }

    #[test]
    fn a_copy_of_a_note_shows_one_row_and_the_plain_file_wins() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(12, "", "Words")).unwrap();
        let copy = dir.path().join("Session 12 (conflict).md");
        fs::copy(&note.path, &copy).unwrap();
        let notes = list_notes(dir.path());
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].path, note.path);
        assert_eq!(notes[0].also, [copy]);
    }

    #[test]
    fn a_note_that_cannot_be_read_is_named_and_a_stranger_is_not() {
        let dir = tempfile::tempdir().unwrap();
        create_note(dir.path(), &new_note(1, "", "Fine")).unwrap();
        fs::write(
            dir.path().join("Broken.md"),
            "---\nsession_id: [\n---\nWords",
        )
        .unwrap();
        fs::write(
            dir.path().join("Plain.md"),
            "Mentions session_id in passing",
        )
        .unwrap();
        assert_eq!(unreadable_notes(dir.path()), ["Broken.md"]);
    }

    #[test]
    fn two_notes_with_one_name_both_survive() {
        let dir = tempfile::tempdir().unwrap();
        let first = create_note(dir.path(), &new_note(3, "", "A")).unwrap();
        let second = create_note(dir.path(), &new_note(3, "", "B")).unwrap();
        assert_ne!(first.path, second.path);
        assert_eq!(list_notes(dir.path()).len(), 2);
    }

    #[test]
    fn details_change_and_the_dms_own_properties_and_words_stay() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Session 1.md");
        let id = Uuid::from_u128(5);
        fs::write(
            &path,
            format!("---\nsession_id: {id}\nsession_number: 1\nsession_title: Old\nsession_date: 2026-01-01\ntags: [dnd]\n---\n\nMy words\n"),
        )
        .unwrap();
        let note = parse_note(&path, &fs::read_to_string(&path).unwrap()).unwrap();
        let note = update_details(&note, " New ", "2026-02-02", 4).unwrap();
        assert_eq!(
            (note.number, note.title.as_str(), note.date.as_str()),
            (4, "New", "2026-02-02")
        );
        let text = fs::read_to_string(&path).unwrap();
        assert!(
            text.contains("tags: [dnd]") && text.ends_with("My words\n"),
            "{text}"
        );
        // Round 30 B7d (review F2 of PR 416): a failed Save details says plain words, never the
        // system's, and leaves the file as it was.
        let before = fs::read_to_string(&path).unwrap();
        FAILING_WRITES.with(|code| code.set(Some(13)));
        let refused = update_details(&note, "Other", "2026-02-03", 4).unwrap_err();
        FAILING_WRITES.with(|code| code.set(None));
        assert_eq!(
            refused,
            "Not saved. No permission to save. Give your user account permission to change the file and its folder."
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), before);
    }

    #[test]
    fn a_blank_title_leaves_no_colon_and_a_rename_moves_the_heading() {
        let dir = tempfile::tempdir().unwrap();
        let template = "# Session {{number}}: {{title}}\n\nNote: stays.\n";
        let note = create_note(dir.path(), &new_note(3, "", template)).unwrap();
        let text = fs::read_to_string(&note.path).unwrap();
        assert!(text.ends_with("# Session 3\n\nNote: stays.\n"), "{text}");
        let note = update_details(&note, "The Bridge", "2026-11-01", 3).unwrap();
        let text = fs::read_to_string(&note.path).unwrap();
        assert!(
            text.ends_with("# Session 3: The Bridge\n\nNote: stays.\n"),
            "{text}"
        );
        // A heading the DM wrote stays.
        let body = text.replace("# Session 3: The Bridge", "# Our own heading");
        fs::write(&note.path, body).unwrap();
        let note = update_details(&note, "Another", "2026-11-01", 3).unwrap();
        assert!(
            fs::read_to_string(&note.path)
                .unwrap()
                .contains("# Our own heading")
        );
        assert_eq!(note.title, "Another");
    }

    #[test]
    fn a_new_date_moves_the_date_line_and_only_that() {
        let dir = tempfile::tempdir().unwrap();
        let template = "# Session {{number}}\n\n{{date}}, The Marches\n\nOn 2026-11-01 we met.\n";
        let note = create_note(dir.path(), &new_note(1, "", template)).unwrap();
        update_details(&note, "", "2026-11-07", 1).unwrap();
        let text = fs::read_to_string(&note.path).unwrap();
        assert!(text.contains("\n2026-11-07, The Marches\n"), "{text}");
        assert!(text.contains("On 2026-11-01 we met."), "{text}");
    }

    #[test]
    fn a_draft_saves_only_the_body_after_its_pause() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(1, "", "Start\n")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        assert_eq!(draft.body.as_str(), "Start\n");
        assert_eq!(draft.state, DraftState::Clean);
        draft.body.push_str("More\n");
        draft.edited();
        assert!(draft.unsaved());
        let edit = draft.last_edit.unwrap();
        assert!(!draft.due_at(edit));
        assert!(draft.due_at(edit + SAVE_AFTER));
        draft.save();
        assert_eq!(draft.state, DraftState::Saved);
        let text = fs::read_to_string(&note.path).unwrap();
        assert!(
            text.starts_with("---\nsession_id:") && text.ends_with("\nStart\nMore\n"),
            "{text}"
        );
    }

    #[test]
    fn a_draft_stops_at_a_change_made_elsewhere_and_the_dm_chooses() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(1, "", "Start\n")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        let outside = fs::read_to_string(&note.path)
            .unwrap()
            .replace("Start", "From Obsidian");
        fs::write(&note.path, &outside).unwrap();

        let _ = draft.body.replace("Mine\n");
        draft.edited();
        // A note act beside unsaved words and a changed file is not applied, and says so.
        // The words and the file stay as they were, and the conflict view decides.
        let act = draft.apply(note.id, Apply::Now, |words| {
            Some(format!("{words}- [[Act]]\n"))
        });
        assert_eq!(act, Err(ActFault::Changed));
        assert_eq!(
            act.unwrap_err().words(),
            "The note changed on disk. Choose in the note first."
        );
        assert_eq!(draft.body.as_str(), "Mine\n");
        assert_eq!(fs::read_to_string(&note.path).unwrap(), outside);
        // Today's plan acts are as they were: the line joins the draft, as typed words do.
        let undo = draft
            .apply(note.id, Apply::WithTyping, |words| {
                Some(format!("{words}- [[Plan]]\n"))
            })
            .unwrap();
        assert_eq!(draft.body.as_str(), "Mine\n- [[Plan]]\n");
        assert_eq!(draft.state, DraftState::Unsaved);
        undo.undo(&mut draft).unwrap();
        assert_eq!(draft.body.as_str(), "Mine\n");
        draft.save();
        assert_eq!(draft.state, DraftState::Conflict);
        assert_eq!(fs::read_to_string(&note.path).unwrap(), outside);
        assert_eq!(
            draft.apply(note.id, Apply::Now, |words| Some(format!(
                "{words}- [[Act]]\n"
            ))),
            Err(ActFault::Changed),
            "a conflict waits for the DM's choice"
        );
        // More typing does not hide the choice.
        draft.body.push_str("And more\n");
        draft.edited();
        assert_eq!(draft.state, DraftState::Conflict);
        // Saving again changes nothing until the DM chooses.
        draft.save();
        assert_eq!(fs::read_to_string(&note.path).unwrap(), outside);

        draft.overwrite();
        assert_eq!(draft.state, DraftState::Saved);
        assert!(
            fs::read_to_string(&note.path)
                .unwrap()
                .ends_with("And more\n")
        );

        fs::write(&note.path, &outside).unwrap();
        let _ = draft.body.replace("Kept for one step\n");
        draft.edited();
        draft.save();
        draft.reload_keeping_undo().unwrap();
        assert!(matches!(draft.dropped, Some(Dropped::Reloaded(_))));
        draft.undo_dropped();
        assert_eq!(draft.body.as_str(), "Kept for one step\n");
        assert_eq!(draft.state, DraftState::Unsaved);
        draft.overwrite();
        fs::write(&note.path, &outside).unwrap();
        let _ = draft.body.replace("Lost\n");
        draft.edited();
        draft.save();
        draft.reload().unwrap();
        assert_eq!(draft.state, DraftState::Clean);
        assert!(draft.body.contains("From Obsidian"));
    }

    /// Section 4.1's path for one note act: an edit on disk a moment before, a failed save, a
    /// restart, a missing file, another note, a line that is gone, then act A, an unrelated act
    /// B and Undo of A. Unsaved words with a changed file are in the test above.
    #[cfg(unix)]
    #[test]
    fn a_note_act_follows_the_disk_saves_at_once_and_keeps_its_undo() {
        let dir = tempfile::tempdir().unwrap();
        let folder = dir.path().join("Sessions");
        fs::create_dir(&folder).unwrap();
        let plan = "## Plan for tonight\n\n- [[Bridge]]\n\n## What happened\n";
        let note = create_note(&folder, &new_note(1, "", plan)).unwrap();
        let other = create_note(&folder, &new_note(2, "", plan)).unwrap();
        let add = |line: &'static str| {
            move |words: &str| {
                let mut words = words.to_owned();
                insert_in_plan(&mut words, line);
                Some(words)
            }
        };
        let mut draft = Draft::open(&note.path).unwrap();

        // An edit made in Obsidian a moment before the act is kept, under the act's line.
        let outside = fs::read_to_string(&note.path)
            .unwrap()
            .replace("## What happened\n", "## What happened\n\nFrom Obsidian.\n");
        fs::write(&note.path, &outside).unwrap();
        let a = draft
            .apply(note.id, Apply::Now, add("- [[Wolf Pack]]"))
            .unwrap();
        let saved = fs::read_to_string(&note.path).unwrap();
        assert_eq!(
            draft.state,
            DraftState::Saved,
            "saved at once, with no pause"
        );
        assert!(
            saved.contains("- [[Bridge]]\n- [[Wolf Pack]]\n") && saved.contains("From Obsidian.")
        );
        assert!(a.live(&draft));
        // An Undo belongs to its own file: a copy with the same ID and the same words at
        // another path does not take it. A move that the folder scan follows does (the plan's
        // test of Undo remove).
        let copy = folder.join("Session 1 (copy).md");
        fs::copy(&note.path, &copy).unwrap();
        let mut copied = Draft::open(&copy).unwrap();
        assert_eq!(
            (copied.note_id(), copied.body.as_str()),
            (Some(note.id), a.after())
        );
        assert!(!a.live(&copied));
        assert_eq!(a.undo(&mut copied), Err(ActFault::OtherNote));
        assert_eq!(fs::read(&copy).unwrap(), fs::read(&note.path).unwrap());
        fs::remove_file(&copy).unwrap();

        // Act B is unrelated: another note. Undo of A puts back only A's line, at once, and B
        // stays.
        let mut second = Draft::open(&other.path).unwrap();
        let b = second
            .apply(other.id, Apply::Now, add("- [[Toll Gang]]"))
            .unwrap();
        a.undo(&mut draft).unwrap();
        assert_eq!(fs::read_to_string(&note.path).unwrap(), outside);
        assert!(!a.live(&draft), "an Undo acts once");
        assert!(
            fs::read_to_string(&other.path)
                .unwrap()
                .contains("- [[Toll Gang]]")
        );
        assert!(b.live(&second));
        // Another change of the words ends an Undo, also with no time limit before it.
        second.body.push_str("Typed.\n");
        second.edited();
        assert!(!b.live(&second));
        assert_eq!(b.undo(&mut second), Err(ActFault::LineChanged));

        // A failed save is reported and writes nothing; the line waits in the draft.
        let listed = |folder: &Path| {
            let mut names: Vec<_> = fs::read_dir(folder)
                .unwrap()
                .map(|file| file.unwrap().file_name())
                .collect();
            names.sort();
            names
        };
        let files = listed(&folder);
        let before = fs::read_to_string(&note.path).unwrap();
        let locked = |mode| {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&folder, fs::Permissions::from_mode(mode)).unwrap();
        };
        locked(0o555);
        assert!(
            fs::write(folder.join("probe"), "").is_err(),
            "the folder takes no new file"
        );
        let failed = draft.apply(note.id, Apply::Now, add("- [[Ash Pack]]"));
        locked(0o755);
        let Err(fault @ ActFault::NotSaved(..)) = failed else {
            panic!("{failed:?}");
        };
        assert_eq!(fault.words(), "Notes not saved");
        assert!(matches!(draft.state, DraftState::Failed(_)));
        assert!(draft.body.contains("- [[Ash Pack]]"));
        assert_eq!(fs::read_to_string(&note.path).unwrap(), before);
        assert_eq!(listed(&folder), files, "nothing half written");
        // The next act saves both lines.
        draft.apply(note.id, Apply::Now, add("- [[Rook]]")).unwrap();

        // After a restart the file holds the lines, and nothing else remembers the acts.
        let reopened = Draft::open(&note.path).unwrap();
        assert!(reopened.body.contains("- [[Ash Pack]]\n- [[Rook]]\n"));
        assert_eq!(listed(&folder), files);
        assert_eq!(listed(dir.path()), [std::ffi::OsString::from("Sessions")]);

        // A line that is gone writes nothing and says so.
        let gone = draft.apply(note.id, Apply::Now, |_| None);
        assert_eq!(
            gone.as_ref().unwrap_err().words(),
            "That line changed on disk."
        );
        assert_eq!(gone, Err(ActFault::LineChanged));

        // A missing file is not an outside edit.
        let moved = folder.join("Moved.md");
        fs::rename(&note.path, &moved).unwrap();
        assert_eq!(
            draft.apply(note.id, Apply::Now, add("- [[Lost]]")),
            Err(ActFault::Missing)
        );
        assert!(!draft.body.contains("Lost"));
        // A file that now holds another note is not this one, also after the one-second poll
        // took that note's words, and on every retry: the act is for note 1.
        fs::copy(&other.path, &note.path).unwrap();
        draft.follow_disk();
        assert_eq!(draft.note_id(), Some(other.id), "the poll follows the file");
        for _ in 0..2 {
            assert_eq!(
                draft.apply(note.id, Apply::Now, add("- [[Lost]]")),
                Err(ActFault::OtherNote)
            );
        }
        assert_eq!(
            fs::read(&note.path).unwrap(),
            fs::read(&other.path).unwrap()
        );

        // An Undo belongs to its note's ID too: the same path and words in another note are not
        // that note, and its Undo writes nothing there.
        let third = create_note(&folder, &new_note(3, "", plan)).unwrap();
        let mut replaced = Draft::open(&third.path).unwrap();
        let c = replaced
            .apply(third.id, Apply::Now, add("- [[Ash]]"))
            .unwrap();
        let text = fs::read_to_string(&other.path).unwrap();
        let (prefix, _) = split_note(&text).unwrap();
        let swapped = format!("{prefix}{}", c.after());
        fs::write(&third.path, &swapped).unwrap();
        replaced.follow_disk();
        assert_eq!(replaced.body.as_str(), c.after());
        assert!(!c.live(&replaced));
        assert_eq!(c.undo(&mut replaced), Err(ActFault::OtherNote));
        assert_eq!(fs::read_to_string(&third.path).unwrap(), swapped);

        // An outside write after the act's check and before its save: the fragment waits in the
        // draft with the conflict, and its Undo takes it back from the draft alone.
        let fourth = create_note(&folder, &new_note(4, "", plan)).unwrap();
        let mut pending = Draft::open(&fourth.path).unwrap();
        let outside = fs::read_to_string(&fourth.path)
            .unwrap()
            .replace("## What happened\n", "## What happened\n\nOutside.\n");
        let result = pending.apply(fourth.id, Apply::Now, |words| {
            fs::write(&fourth.path, &outside).unwrap();
            add("- [[Pending]]")(words)
        });
        let Err(ActFault::NotSaved(_, d)) = result else {
            panic!("{result:?}");
        };
        assert_eq!(pending.state, DraftState::Conflict);
        assert!(d.live(&pending));
        d.undo(&mut pending).unwrap();
        assert!(!pending.body.contains("Pending"));
        assert_eq!(pending.state, DraftState::Conflict, "the DM still chooses");
        assert_eq!(fs::read_to_string(&fourth.path).unwrap(), outside);
    }

    #[test]
    fn discarding_a_deleted_notes_draft_writes_no_file() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(5, "", "Start\n")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        fs::remove_file(&note.path).unwrap();
        let _ = draft.body.replace("My words\n");
        draft.edited();
        draft.save();
        assert_eq!(draft.state, DraftState::Missing);
        assert_eq!(draft.moved_to, None);
        assert!(!note.path.exists());

        draft.discard();
        assert!(!note.path.exists());
        assert!(list_notes(dir.path()).is_empty());
        // iter-25-S SM1: with no file left, the editor says so.
        assert_eq!(draft.state, DraftState::Missing);
        assert_eq!(draft.body.as_str(), "Start\n");
        draft.undo_dropped();
        assert_eq!(draft.body.as_str(), "My words\n");
        assert!(!note.path.exists());
        // Discard, edit, Save: nothing is written at the old path.
        draft.discard();
        draft.body.push_str("Typed after discard\n");
        draft.edited();
        draft.save();
        assert_eq!(draft.state, DraftState::Missing);
        assert!(!note.path.exists());
    }

    #[test]
    fn a_note_whose_file_went_is_saved_again_under_a_new_id() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(4, "", "Start\n")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        // Renamed in Obsidian.
        fs::create_dir_all(dir.path().join("Past")).unwrap();
        let renamed = dir.path().join("Past/Renamed.md");
        fs::rename(&note.path, &renamed).unwrap();
        let _ = draft.body.replace("My words\n");
        draft.edited();
        draft.save();
        assert_eq!(draft.state, DraftState::Missing);
        assert!(draft.needs_attention() && draft.unsaved());
        assert!(!note.path.exists());
        assert_eq!(draft.moved_to.as_deref(), Some(renamed.as_path()));

        let mut moved = draft.clone();
        moved.save_moved();
        assert_eq!(moved.state, DraftState::Saved);
        assert_eq!(
            parse_note(&renamed, &fs::read_to_string(&renamed).unwrap())
                .unwrap()
                .id,
            note.id
        );
        assert!(!note.path.exists());

        // Discard names itself, so its Undo says Undo discard, not Undo reload. It writes
        // nothing: the moved file keeps its text, and no file comes back at the old path.
        // iter-25-S SM1: Discard follows the moved file, with the words it holds now (here an
        // outside edit), and later saves go there.
        let moved_text = fs::read_to_string(&renamed)
            .unwrap()
            .replace("My words", "Moved words");
        fs::write(&renamed, &moved_text).unwrap();
        let mut discarded = draft.clone();
        discarded.discard();
        assert!(!note.path.exists());
        assert_eq!(fs::read_to_string(&renamed).unwrap(), moved_text);
        assert_eq!(
            discarded.dropped,
            Some(Dropped::Discarded("My words\n".into()))
        );
        assert_eq!(discarded.path, renamed);
        assert_eq!(discarded.state, DraftState::Clean);
        assert_eq!(discarded.body.as_str(), "Moved words\n");
        // Review of PR 255, item 2: the moved file is edited outside; the disk timer follows it
        // (twice, the second time with nothing new). The outside words show, and the discarded
        // words stay recoverable; their Undo meets the conflict choice, not a silent write.
        let mut followed = discarded.clone();
        let edited_text = moved_text.replace("Moved words", "Edited in Obsidian");
        fs::write(&renamed, &edited_text).unwrap();
        followed.follow_disk();
        followed.follow_disk();
        assert_eq!(followed.body.as_str(), "Edited in Obsidian\n");
        assert_eq!(
            followed.dropped,
            Some(Dropped::Discarded("My words\n".into()))
        );
        followed.undo_dropped();
        assert_eq!(followed.body.as_str(), "My words\n");
        assert_eq!(followed.state, DraftState::Conflict);
        followed.save();
        assert_eq!(fs::read_to_string(&renamed).unwrap(), edited_text);
        fs::write(&renamed, &moved_text).unwrap();
        let mut typed = discarded.clone();
        typed.body.push_str("Typed after discard\n");
        typed.edited();
        typed.save();
        assert_eq!(typed.state, DraftState::Saved);
        assert!(!note.path.exists());
        assert!(
            fs::read_to_string(&renamed)
                .unwrap()
                .contains("Moved words\nTyped after discard")
        );
        fs::write(&renamed, &moved_text).unwrap();
        discarded.undo_dropped();
        assert_eq!(discarded.body.as_str(), "My words\n");
        assert_eq!(discarded.dropped, None);
        assert!(!note.path.exists());
        assert_eq!(fs::read_to_string(&renamed).unwrap(), moved_text);
        // Undo discard, then Save: the words go to the moved file, never the old path.
        discarded.save();
        assert_eq!(discarded.state, DraftState::Saved);
        assert!(!note.path.exists());
        assert!(fs::read_to_string(&renamed).unwrap().contains("My words"));
        fs::write(&renamed, &moved_text).unwrap();
        // A moved file that no longer holds this note is not followed.
        let mut other = draft.clone();
        let other_text = moved_text.replace(&note.id.to_string(), &Uuid::new_v4().to_string());
        fs::write(&renamed, &other_text).unwrap();
        other.discard();
        assert_eq!(other.path, note.path);
        assert_eq!(other.state, DraftState::Missing);
        assert_eq!(fs::read_to_string(&renamed).unwrap(), other_text);
        fs::write(&renamed, &moved_text).unwrap();

        draft.save_again();
        assert_eq!(draft.state, DraftState::Saved);
        let id = draft.restored.unwrap();
        assert_ne!(id, note.id);
        let notes = list_notes(dir.path());
        assert_eq!(notes.len(), 2);
        assert!(notes.iter().any(|n| n.id == id && n.path == note.path));
        assert!(notes.iter().any(|n| n.id == note.id && n.path == renamed));
        assert!(
            fs::read_to_string(&note.path)
                .unwrap()
                .ends_with("My words\n")
        );
    }

    #[test]
    fn a_clean_draft_follows_the_disk_and_an_edit_does_not() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(1, "", "Start\n")).unwrap();
        let mut draft = Draft::open(&note.path).unwrap();
        let outside = fs::read_to_string(&note.path)
            .unwrap()
            .replace("Start", "Elsewhere");
        fs::write(&note.path, &outside).unwrap();
        draft.follow_disk();
        assert!(draft.body.contains("Elsewhere"));
        let _ = draft.body.replace("Mine");
        draft.edited();
        fs::write(&note.path, "---\nsession_id: x\n---\nAgain").unwrap();
        draft.follow_disk();
        assert_eq!(draft.body.as_str(), "Mine");
    }

    #[test]
    fn words_go_under_what_happened_or_at_the_end() {
        let mut note = "# Title\n\n## What happened\n\nWe met.\n\n\n## Next\n\nLater.\n".to_owned();
        insert_in_note(&mut note, "[[Timeline#^a|Rest]]");
        assert_eq!(
            note,
            "# Title\n\n## What happened\n\nWe met.\n\n[[Timeline#^a|Rest]]\n\n## Next\n\nLater.\n"
        );
        let mut last = "## What happened\n".to_owned();
        insert_in_note(&mut last, "A");
        assert_eq!(last, "## What happened\n\nA\n");
        let mut none = "Free text\n\n".to_owned();
        insert_in_note(&mut none, "A");
        assert_eq!(none, "Free text\n\nA\n");
        let mut empty = String::new();
        insert_in_note(&mut empty, "A");
        assert_eq!(empty, "A\n");
        // A longer heading that starts the same way is not this one.
        let mut other = "## What happened later\n\nX\n".to_owned();
        insert_in_note(&mut other, "A");
        assert!(other.ends_with("X\n\nA\n"), "{other}");
    }

    #[test]
    fn a_table_note_starts_on_a_fresh_line_under_what_happened() {
        let template = "# Title\n\n## What happened\n\n\n\n## Open threads\n\n";
        let mut body = template.to_owned();
        let (caret, changed) = fresh_line_in_note(&mut body);
        assert!(!changed);
        assert_eq!(body, template);
        assert_eq!(
            body.chars().take(caret).collect::<String>(),
            "# Title\n\n## What happened\n\n"
        );

        let mut tight = "## What happened\n## Next\n".to_owned();
        let (caret, changed) = fresh_line_in_note(&mut tight);
        assert!(changed);
        assert_eq!(tight, "## What happened\n\n\n\n## Next\n");
        assert_eq!(
            tight.chars().take(caret).collect::<String>(),
            "## What happened\n\n"
        );

        // Accented text counts in characters, which is what the caret counts.
        let mut plain = "Été".to_owned();
        let (caret, _) = fresh_line_in_note(&mut plain);
        assert_eq!(caret, 5);
        assert_eq!(plain, "Été\n\n");

        let mut empty = String::new();
        assert_eq!(fresh_line_in_note(&mut empty), (0, false));

        // A second note, and a third, never join the heading that follows.
        let mut note = template.to_owned();
        for words in ["First note.", "Second note.", "Third note."] {
            let (caret, _) = fresh_line_in_note(&mut note);
            let at = note
                .char_indices()
                .nth(caret)
                .map_or(note.len(), |(at, _)| at);
            note.insert_str(at, words);
        }
        assert_eq!(
            note,
            "# Title\n\n## What happened\n\nFirst note.\n\nSecond note.\n\nThird note.\n\n## Open threads\n\n"
        );
    }

    #[test]
    fn a_summary_nested_in_a_note_keeps_the_notes_sections() {
        let mut note = "## What happened\n\nWe met.\n\n## Open threads\n\n".to_owned();
        insert_in_note(
            &mut note,
            &nested_in_note("> A draft.\n\n## Recap\n\nA rest.\n"),
        );
        assert_eq!(
            note,
            "## What happened\n\nWe met.\n\n> A draft.\n\n### Recap\n\nA rest.\n\n## Open threads\n\n"
        );
        // A later note still finds the end of the section, after the nested heading.
        let (caret, _) = fresh_line_in_note(&mut note);
        let before: String = note.chars().take(caret).collect();
        assert!(before.ends_with("A rest.\n\n"), "{before}");
    }

    #[test]
    fn the_reader_leaves_out_only_what_the_page_already_shows() {
        let note = Note {
            id: Uuid::from_u128(1),
            number: 2,
            title: "Bridge".into(),
            date: "2026-11-01".into(),
            path: "a.md".into(),
            also: Vec::new(),
        };
        let body = "# Session 2: Bridge\n\n2026-11-01, The Marches\n\n## What happened\n";
        assert_eq!(without_own_heading(body, &note), "## What happened\n");
        // A heading the DM wrote stays, and so does a date line under it.
        let own = "# Our night\n\n2026-11-01\n";
        assert_eq!(without_own_heading(own, &note), own);
    }

    #[test]
    fn reading_leaves_out_empty_headings_and_keeps_filled_ones() {
        let text = "## Recap\n\n\n## Plan\n\nGo north.\n\n## People\n\n### Friends\n\n## Loot\n\n";
        assert_eq!(without_empty_sections(text), "## Plan\n\nGo north.\n");
        // A heading whose subheading has words is not empty.
        assert_eq!(
            without_empty_sections("## People\n\n### Friends\n\nHobb.\n"),
            "## People\n\n### Friends\n\nHobb.\n"
        );
    }

    #[test]
    fn a_section_is_found_by_its_heading() {
        let body = "## What happened\n\nA fight.\n\n## Open threads\n\nWho burned the ropes?\n";
        assert_eq!(section_text(body, "What happened").unwrap(), "A fight.");
        assert_eq!(
            section_text(body, "Open threads").unwrap(),
            "Who burned the ropes?"
        );
        assert_eq!(
            section_text("## What happened\n\n\n", "What happened"),
            None
        );
        assert_eq!(section_text(body, "Loot"), None);
    }

    #[test]
    fn links_are_read_as_names_and_as_words() {
        let text =
            "See [[Ashfall Crossing]], [[Hobb|the miller]] and [[Timeline#^t-1|Day 1]]. [[Hobb]]";
        assert_eq!(linked_names(text), vec!["Ashfall Crossing", "Hobb"]);
        assert_eq!(
            each_linked_name(text),
            vec!["Ashfall Crossing", "Hobb", "Hobb"]
        );
        assert_eq!(
            plain_links("See [[Ashfall Crossing]] and [[Hobb|the miller]], [[open"),
            "See Ashfall Crossing and the miller, [[open"
        );
    }

    #[test]
    fn a_plan_line_goes_under_the_plan_heading() {
        let mut body = "## Plan for tonight\n\n\n\n## What happened\n\n".to_owned();
        insert_in_plan(&mut body, "[[A]]");
        insert_in_plan(&mut body, "[[B]]");
        assert_eq!(
            body,
            "## Plan for tonight\n\n[[A]]\n[[B]]\n\n## What happened\n\n"
        );
        let mut other = "## Next time\n\n".to_owned();
        insert_in_plan(&mut other, "[[A]]");
        assert!(other.contains("[[A]]"));
    }

    /// T12.10: a task line is a scene line, a plain or nested line is not, and a tick, a removal
    /// and a new scene change only their own line.
    #[test]
    fn a_task_line_of_the_plan_is_a_scene_line_and_changes_alone() {
        let body = "## Plan for tonight\n\n- [[Bridge]]\n- [ ] [[Captain Varra]]\n  - [ ] nested\n* [x] The miller is the informer\n- [] not a box\n1. [ ] numbered\n\n## What happened\n\n- [ ] Not the plan\n";
        let lines = plan_lines(body);
        let tasks: Vec<_> = lines
            .iter()
            .filter_map(|line| match line {
                PlanLine::Task(task) => Some((task.at, task.done, task.words.as_str())),
                PlanLine::Links(_) => None,
            })
            .collect();
        assert_eq!(
            tasks,
            [
                (3, false, "[[Captain Varra]]"),
                (5, true, "The miller is the informer")
            ]
        );
        assert_eq!(
            lines.len(),
            6,
            "every other line of the plan keeps its links"
        );
        assert!(lines.contains(&PlanLine::Links(vec!["Bridge".into()])));

        let tick = change_line(
            body,
            3,
            "- [ ] [[Captain Varra]]",
            Some(&ticked("- [ ] [[Captain Varra]]", true)),
        )
        .unwrap();
        assert_eq!(tick.len(), body.len());
        assert_eq!(
            tick.chars()
                .zip(body.chars())
                .filter(|(a, b)| a != b)
                .count(),
            1,
            "a tick changes one character"
        );
        assert!(tick.contains("- [x] [[Captain Varra]]\n"));
        assert_eq!(
            ticked("* [x] The miller is the informer", false),
            "* [ ] The miller is the informer"
        );

        // Two lines with the same words: the act changes the one at its own place, and a place
        // that holds other words now changes nothing, even when another line reads the same.
        let twice = "## Plan for tonight\n\n- [ ] A\n- [[Bridge]]\n- [ ] A\n";
        assert_eq!(
            change_line(twice, 4, "- [ ] A", None).unwrap(),
            "## Plan for tonight\n\n- [ ] A\n- [[Bridge]]\n"
        );
        assert_eq!(change_line(twice, 2, "- [ ] B", None), None);
        assert_eq!(change_line(twice, 3, "- [ ] A", None), None);
        assert_eq!(change_line(twice, 9, "- [ ] A", None), None);

        assert_eq!(
            add_scene(
                "## Plan for tonight\n\n- [[Bridge]]\n\n## What happened\n",
                "  Meet\nHobb "
            ),
            "## Plan for tonight\n\n- [[Bridge]]\n- [ ] Meet Hobb\n\n## What happened\n"
        );
        assert_eq!(
            add_scene("# Session 1\n\nWords.\n", "Meet Hobb"),
            "# Session 1\n\nWords.\n\n## Plan for tonight\n\n- [ ] Meet Hobb\n"
        );
    }

    #[test]
    fn fmt_f1_empty_session_title_is_absent() {
        let dir = tempfile::tempdir().unwrap();
        let note = create_note(dir.path(), &new_note(1, "", "Body")).unwrap();
        assert!(
            !fs::read_to_string(&note.path)
                .unwrap()
                .contains("session_title:")
        );
        let titled = update_details(&note, "Title", "2026-11-03", 1).unwrap();
        let cleared = update_details(&titled, "", "2026-11-03", 1).unwrap();
        assert!(
            !fs::read_to_string(&cleared.path)
                .unwrap()
                .contains("session_title:")
        );
    }

    #[test]
    fn a_text_divides_at_time_headings_and_loses_nothing() {
        let text =
            "> A note.\n\n## 00:00:00\n\nOne.\n\n## People\n\nTwo.\n\n## 00:10:00\n\nThree.\n";
        let parts = parts_of(text);
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].0, "00:00:00");
        assert_eq!(parts[1].0, "00:10:00");
        assert_eq!(parts[0].1.clone() + &parts[1].1, text);
        assert_eq!(parts_of("No times.\n").len(), 1);
        assert_eq!(parts_of("").len(), 1);
    }

    #[test]
    fn crlf_notes_keep_their_frontmatter() {
        let text = "---\r\nsession_id: 00000000-0000-0000-0000-000000000005\r\n---\r\nBody\r\n";
        let (front, body) = split_note(text).unwrap();
        assert!(front.ends_with("---\r\n"));
        assert_eq!(body, "Body\r\n");
        assert_eq!(
            parse_note(Path::new("a.md"), text).unwrap().id,
            Uuid::from_u128(5)
        );
    }

    #[test]
    fn a_rule_inside_the_frontmatter_value_does_not_end_it() {
        let (front, body) = split_note("---\na: 1\n---not the end\n---\nBody").unwrap();
        assert_eq!(front, "---\na: 1\n---not the end\n---\n");
        assert_eq!(body, "Body");
        assert_eq!(body_of("no frontmatter"), "no frontmatter");
    }

    #[test]
    fn templates_list_the_campaigns_own_first_and_replace_a_built_in_of_the_same_name() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(list_templates(dir.path()).len(), BUILT_IN_TEMPLATES.len());
        create_template(dir.path(), "Downtime", "Between adventures").unwrap();
        create_template(dir.path(), "Short notes", "My version").unwrap();
        let templates = list_templates(dir.path());
        let names: Vec<&str> = templates.iter().map(|t| t.name.as_str()).collect();
        assert_eq!(names, ["Downtime", "Short notes", "Standard session"]);
        assert_eq!(templates[1].body, "My version");
        assert!(templates[2].path.is_none());
        let error = create_template(dir.path(), "Downtime", "Again").unwrap_err();
        assert!(error.contains("already exists"));
        assert!(create_template(dir.path(), "///", "x").is_err());
    }

    #[test]
    fn moving_storage_keeps_every_file_and_refuses_collisions() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::from_u128(9),
            name: "Marches".into(),
            path: dir.path().to_owned(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
        let old = open_folder(&campaign).unwrap();
        let note = create_note(&old, &new_note(1, "The Bridge", "My words.")).unwrap();
        let relative = [
            note.path.strip_prefix(&old).unwrap().to_path_buf(),
            PathBuf::from("Timeline.md"),
            PathBuf::from("Templates/Own.md"),
            PathBuf::from("Recordings/session/audio.wav"),
            PathBuf::from("Recordings/session/audio.transcript.md"),
        ];
        for path in &relative[1..] {
            fs::create_dir_all(old.join(path).parent().unwrap()).unwrap();
            fs::write(old.join(path), format!("kept {}", path.display())).unwrap();
        }
        fs::write(old.join("Other.md"), "Not a session note.").unwrap();
        let originals: Vec<_> = relative
            .iter()
            .map(|path| fs::read(old.join(path)).unwrap())
            .collect();
        let new = dir.path().join("Evenings");
        fs::create_dir_all(new.join("Templates")).unwrap();
        fs::write(new.join("Templates/Own.md"), "Existing template").unwrap();
        assert!(
            move_storage(&campaign, "Evenings")
                .unwrap_err()
                .contains("already exists")
        );
        assert_eq!(folder(&campaign).unwrap(), old);
        assert!(note.path.exists());
        assert_eq!(
            fs::read_to_string(new.join("Templates/Own.md")).unwrap(),
            "Existing template"
        );
        fs::remove_file(new.join("Templates/Own.md")).unwrap();
        assert_eq!(move_storage(&campaign, "Evenings").unwrap().0, new);
        assert_eq!(folder(&campaign).unwrap(), new);
        for (path, bytes) in relative.iter().zip(originals) {
            assert_eq!(fs::read(new.join(path)).unwrap(), bytes.clone());
            assert!(!old.join(path).exists());
        }
        assert!(old.join("Other.md").exists());
        // Round 30 B7e: a move the system refuses after one file moved: the move and its
        // rollback say plain words, never the system's; the folder setting stays; every file
        // keeps its bytes in one of the two folders.
        let kept: Vec<_> = relative
            .iter()
            .map(|path| fs::read(new.join(path)).unwrap())
            .collect();
        FAILING_TRANSFERS.with(|failing| failing.set(Some((1, 13, FailAt::Start))));
        let refused = move_storage(&campaign, "Mornings").unwrap_err();
        FAILING_TRANSFERS.with(|failing| failing.set(None));
        // Review F2 of PR 431: the refused move and the refused return each say where their
        // file is now, and the setting's folder is named.
        assert!(
            refused.contains("was not moved. It is still in Evenings. No permission to save.")
                && refused.contains("was not moved back. It is still in Mornings. No permission")
                && refused.contains(
                    "The session folder is still Evenings. Move the files named above back into \
                     it, or choose Mornings again."
                )
                && !refused.contains("os error")
                && !refused.contains("Permission denied"),
            "{refused}"
        );
        assert_eq!(folder(&campaign).unwrap(), new);
        let mornings = dir.path().join("Mornings");
        for (path, bytes) in relative.iter().zip(kept) {
            let here = fs::read(new.join(path)).ok();
            let there = fs::read(mornings.join(path)).ok();
            assert!(
                here.as_ref() == Some(&bytes) || there.as_ref() == Some(&bytes),
                "{} kept its bytes",
                path.display()
            );
        }
        // Review F1 of PR 431: the scan before a move says plain words: a folder it may not
        // read, and a note that is not text. Nothing moves, and the bytes stay.
        FAILING_SCAN.with(|code| code.set(Some(13)));
        let refused = move_storage(&campaign, "Nights").unwrap_err();
        FAILING_SCAN.with(|code| code.set(None));
        assert_eq!(
            refused,
            "Not moved. Splat has no permission to read Evenings. Give your user account \
             permission to read the file."
        );
        fs::write(new.join("Bad.md"), [0xff, 0xfe, b'\n']).unwrap();
        let refused = move_storage(&campaign, "Nights").unwrap_err();
        assert_eq!(
            refused,
            "Not moved. Splat cannot read Bad.md as text. Splat reads notes as UTF-8 text. Open \
             the file in a text editor and save it as UTF-8 text, or restore it from a backup."
        );
        assert_eq!(fs::read(new.join("Bad.md")).unwrap(), [0xff, 0xfe, b'\n']);
        assert!(!dir.path().join("Nights").exists());
        assert_eq!(folder(&campaign).unwrap(), new);
    }

    #[test]
    fn the_folder_comes_from_the_manifest_and_stays_inside_the_rules() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::from_u128(9),
            name: "Marches".into(),
            path: dir.path().to_owned(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
        assert_eq!(folder(&campaign).unwrap(), dir.path().join(DEFAULT_FOLDER));
        assert_eq!(
            save_folder(&campaign, "D20.42.22 Sessions").unwrap(),
            dir.path().join("D20.42.22 Sessions")
        );
        assert_eq!(
            folder(&campaign).unwrap(),
            dir.path().join("D20.42.22 Sessions")
        );
        assert!(save_folder(&campaign, "../outside").is_err());
        assert!(save_folder(&campaign, "  ").is_err());
        assert_eq!(folder_setting(&campaign).unwrap(), "D20.42.22 Sessions");
    }

    #[test]
    fn recordings_list_newest_first_with_what_was_made_from_them() {
        let dir = tempfile::tempdir().unwrap();
        let session = Uuid::from_u128(3);
        let folder = recordings_folder(dir.path(), session);
        fs::create_dir_all(&folder).unwrap();
        let wav = crate::recording::wav_bytes(&[0; 32_000]);
        fs::write(folder.join("2026-11-01 19-00.wav"), &wav).unwrap();
        fs::write(folder.join("2026-11-01 21-00.wav"), &wav).unwrap();
        fs::write(folder.join("2026-11-01 21-00.transcript.md"), "x").unwrap();
        fs::write(
            folder.join("2026-11-01 21-00.state.txt"),
            "Failed at part 3 of 24: busy",
        )
        .unwrap();
        let list = list_recordings(dir.path(), session);
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].name, "2026-11-01 21-00");
        assert_eq!(list[0].seconds, Some(2));
        assert!(list[0].transcript && !list[0].summary);
        assert_eq!(
            list[0].status.as_deref(),
            Some("Failed at part 3 of 24: busy")
        );
        assert!(!list[1].transcript);

        delete_recording(&list[0].path).unwrap();
        assert!(!folder.join("2026-11-01 21-00.transcript.md").exists());
        assert!(!folder.join("2026-11-01 21-00.state.txt").exists());
        assert_eq!(list_recordings(dir.path(), session).len(), 1);
        assert!(list_recordings(dir.path(), Uuid::from_u128(4)).is_empty());
    }
}
