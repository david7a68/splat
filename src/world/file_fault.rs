//! Plain words for a note file that the system will not read or write.
//!
//! The DM reads what happened to which file and how to recover it. The system's own words
//! (`stream did not contain valid UTF-8`, `Permission denied (os error 13)`) go only to
//! Technical details. Nothing here changes the file.
use std::{io, path::Path};

/// What went wrong, from the system error's kind. Never from its words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    /// The bytes are not UTF-8 text.
    NotText,
    ReadDenied,
    WriteDenied,
    Gone,
    Exists,
    DiskFull,
    CannotRead,
    CannotWrite,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FileFault {
    /// The file as the DM knows it: a path relative to the campaign, or a folder.
    pub path: String,
    pub kind: Kind,
    /// The system's words, for Technical details only.
    pub system: String,
}

impl FileFault {
    /// A failed read of `path`.
    pub fn read(path: &str, error: &io::Error) -> Self {
        let kind = match error.kind() {
            io::ErrorKind::InvalidData => Kind::NotText,
            io::ErrorKind::PermissionDenied => Kind::ReadDenied,
            io::ErrorKind::NotFound => Kind::Gone,
            _ => Kind::CannotRead,
        };
        Self::new(path, kind, error)
    }

    /// A failed write of `path`, or of the folder that holds it.
    pub fn write(path: &str, error: &io::Error) -> Self {
        let kind = match error.kind() {
            io::ErrorKind::PermissionDenied | io::ErrorKind::ReadOnlyFilesystem => {
                Kind::WriteDenied
            }
            io::ErrorKind::StorageFull | io::ErrorKind::QuotaExceeded => Kind::DiskFull,
            io::ErrorKind::NotFound => Kind::Gone,
            io::ErrorKind::AlreadyExists => Kind::Exists,
            _ => Kind::CannotWrite,
        };
        Self::new(path, kind, error)
    }

    fn new(path: &str, kind: Kind, error: &io::Error) -> Self {
        Self {
            path: path.to_owned(),
            kind,
            system: error.to_string(),
        }
    }

    /// The first sentence: what happened to which file.
    pub fn headline(&self) -> String {
        let path = &self.path;
        match self.kind {
            Kind::NotText => format!("Splat cannot read {path} as text."),
            Kind::ReadDenied => format!("Splat has no permission to read {path}."),
            Kind::WriteDenied => format!("Splat has no permission to save {path}."),
            Kind::Gone => format!("Splat cannot find {path}."),
            Kind::Exists => format!("{path} already exists."),
            Kind::DiskFull => format!("The disk is full, so Splat cannot save {path}."),
            Kind::CannotRead => format!("Splat cannot open {path}."),
            Kind::CannotWrite => format!("Splat cannot save {path}."),
        }
    }

    /// How the DM recovers. Splat has not changed the file.
    pub fn recovery(&self) -> &'static str {
        match self.kind {
            Kind::NotText => {
                "Splat reads notes as UTF-8 text. Open the file in a text editor and save it as \
                 UTF-8 text, or restore it from a backup."
            }
            Kind::ReadDenied => "Give your user account permission to read the file.",
            Kind::WriteDenied => {
                "Give your user account permission to change the file and its folder."
            }
            Kind::Gone => "It was moved or deleted outside Splat.",
            Kind::Exists => "Choose another name.",
            Kind::DiskFull => "Free some space on the disk, then try again.",
            Kind::CannotRead | Kind::CannotWrite => {
                "Check that the disk that holds it is connected, then try again."
            }
        }
    }

    /// The short cause on a list row.
    pub fn cause(&self) -> &'static str {
        match self.kind {
            Kind::NotText => "Text format cannot be read",
            Kind::ReadDenied => "No permission to read",
            Kind::WriteDenied => "No permission to save",
            Kind::Gone => "File not found",
            Kind::Exists => "File already exists",
            Kind::DiskFull => "Disk is full",
            Kind::CannotRead => "File cannot be opened",
            Kind::CannotWrite => "File cannot be saved",
        }
    }

    /// The whole fault in one message, where there is no room for Technical details.
    pub fn message(&self) -> String {
        format!("{} {}", self.headline(), self.recovery())
    }
}

/// A note's text, or a message that names the file in plain words.
pub(crate) fn read_note(root: &Path, relative: &str) -> Result<String, String> {
    let path = crate::library::note_path(root, relative)?;
    std::fs::read_to_string(path).map_err(|error| FileFault::read(relative, &error).message())
}

/// A failed write, as one message that names the file.
pub(crate) fn write_error(relative: &str) -> impl Fn(io::Error) -> String + '_ {
    move |error| FileFault::write(relative, &error).message()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter18_d_kinds_come_from_the_error_kind_and_hide_system_words() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Broken.md");
        std::fs::write(&path, [0xff, 0xfe, b'\n']).unwrap();
        let error = std::fs::read_to_string(&path).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        let fault = FileFault::read("Notes/Broken.md", &error);
        assert_eq!(fault.kind, Kind::NotText, "AD2-kind");
        assert_eq!(
            fault.headline(),
            "Splat cannot read Notes/Broken.md as text.",
            "AD2-headline"
        );
        assert_eq!(fault.cause(), "Text format cannot be read", "AD2-cause");
        assert!(fault.system.contains("UTF-8"), "AD2-system-kept");
        for words in [fault.message(), fault.cause().to_owned()] {
            assert!(
                !words.contains(&fault.system),
                "AD2-no-system-words: {words}"
            );
            assert!(!words.contains("stream"), "AD2-no-stream: {words}");
        }
        assert_eq!(std::fs::read(&path).unwrap(), [0xff, 0xfe, b'\n']);
    }

    /// Whether this user can be refused by file permissions (root is not).
    #[cfg(unix)]
    fn permissions_hold(dir: &Path) -> bool {
        use std::os::unix::fs::PermissionsExt;
        let probe = dir.join("probe");
        std::fs::create_dir(&probe).unwrap();
        std::fs::set_permissions(&probe, std::fs::Permissions::from_mode(0o555)).unwrap();
        let held = std::fs::write(probe.join("x"), "x").is_err();
        std::fs::set_permissions(&probe, std::fs::Permissions::from_mode(0o755)).unwrap();
        held
    }

    #[cfg(unix)]
    #[test]
    fn iter18_d_world_reads_and_saves_name_the_file_not_the_system() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        if !permissions_hold(dir.path()) {
            return;
        }
        let folder = dir.path().join("NPCs");
        std::fs::create_dir(&folder).unwrap();
        let file = folder.join("Rook.md");
        let before = "---\nsplat_schema: 2\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: world\nsplat_name: Rook\n---\n";
        std::fs::write(&file, before).unwrap();
        assert_eq!(read_note(dir.path(), "NPCs/Rook.md").unwrap(), before);

        // A save the folder refuses: the note's own path and the recovery, never `os error`.
        std::fs::set_permissions(&folder, std::fs::Permissions::from_mode(0o555)).unwrap();
        let saved = crate::library::write_note(
            dir.path(),
            "NPCs/Rook.md",
            Some(before),
            &before.replace("Rook", "Rook Ambry"),
        );
        std::fs::set_permissions(&folder, std::fs::Permissions::from_mode(0o755)).unwrap();
        let error = saved.err().expect("the folder refuses the save");
        assert!(
            error.starts_with("Splat has no permission to save NPCs/Rook.md."),
            "write-denied: {error}"
        );
        assert!(!error.contains("os error"), "write-raw: {error}");
        assert_eq!(std::fs::read_to_string(&file).unwrap(), before);

        // A read the file refuses.
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o000)).unwrap();
        let read = read_note(dir.path(), "NPCs/Rook.md");
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o644)).unwrap();
        let error = read.unwrap_err();
        assert!(
            error.starts_with("Splat has no permission to read NPCs/Rook.md."),
            "read-denied: {error}"
        );
        assert!(!error.contains("os error"), "read-raw: {error}");

        // Text that is not UTF-8.
        std::fs::write(&file, [0xff, 0xfe]).unwrap();
        let error = read_note(dir.path(), "NPCs/Rook.md").unwrap_err();
        assert!(
            error.starts_with("Splat cannot read NPCs/Rook.md as text."),
            "{error}"
        );
        assert!(!error.contains("stream"), "{error}");
        assert_eq!(std::fs::read(&file).unwrap(), [0xff, 0xfe]);

        // A campaign folder that is gone.
        let gone = dir.path().join("Gone");
        let error = read_note(&gone, "NPCs/Rook.md").unwrap_err();
        assert!(error.starts_with("Splat cannot find "), "{error}");
        assert!(!error.contains("os error"), "{error}");
    }

    #[test]
    fn iter18_d_every_kind_names_its_file_and_a_recovery() {
        let cases = [
            (io::ErrorKind::InvalidData, true, Kind::NotText),
            (io::ErrorKind::PermissionDenied, true, Kind::ReadDenied),
            (io::ErrorKind::NotFound, true, Kind::Gone),
            (io::ErrorKind::Interrupted, true, Kind::CannotRead),
            (io::ErrorKind::PermissionDenied, false, Kind::WriteDenied),
            (io::ErrorKind::ReadOnlyFilesystem, false, Kind::WriteDenied),
            (io::ErrorKind::StorageFull, false, Kind::DiskFull),
            (io::ErrorKind::AlreadyExists, false, Kind::Exists),
            (io::ErrorKind::NotFound, false, Kind::Gone),
            (io::ErrorKind::Interrupted, false, Kind::CannotWrite),
        ];
        for (kind, read, expected) in cases {
            let error = io::Error::new(kind, "raw system words (os error 99)");
            let fault = if read {
                FileFault::read("NPCs/Rook.md", &error)
            } else {
                FileFault::write("NPCs/Rook.md", &error)
            };
            assert_eq!(fault.kind, expected, "{kind:?} read={read}");
            let message = fault.message();
            assert!(message.contains("NPCs/Rook.md"), "{message}");
            assert!(!message.contains("os error"), "{message}");
            assert!(!fault.recovery().is_empty());
            assert_eq!(fault.system, "raw system words (os error 99)");
        }
    }
}
