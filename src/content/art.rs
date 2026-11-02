//! Images that packages offer for entries and for links in notes.
//!
//! A package can hold an `Art Index.md` next to its manifest. A JSON block in the note gives
//! the path of an image for an entry key or for a link address, relative to the package folder.
//! An entry can belong to any package, so one art package can serve the built-in SRD and every
//! book. It is a note because a vault sync can leave out every file that is not.
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Component, Path, PathBuf},
};

pub const ART_FILE: &str = "Art Index.md";
const MAX_FILE: u64 = 64 * 1024 * 1024;
const FORMATS: &[&str] = &["webp", "png", "jpg", "jpeg"];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtFile {
    splat_art: u32,
    #[serde(default)]
    entries: HashMap<String, String>,
    #[serde(default)]
    links: HashMap<String, String>,
}

/// Where the art is. With two offers for one target, the package registered first wins.
#[derive(Debug, Default)]
pub struct ArtIndex {
    entries: HashMap<String, PathBuf>,
    links: HashMap<String, PathBuf>,
}

impl ArtIndex {
    /// Read the art of each package folder. A bad file costs only that package its art.
    pub fn load<'a>(roots: impl IntoIterator<Item = &'a Path>) -> Self {
        let mut index = Self::default();
        for root in roots {
            // The Library reports these errors, with the other package diagnostics.
            let _ = index.add_package(root);
        }
        index
    }

    /// Add one package folder. A package with no art file adds nothing.
    pub fn add_package(&mut self, root: &Path) -> Result<(), String> {
        let path = root.join(ART_FILE);
        let Ok(meta) = fs::symlink_metadata(&path) else {
            return Ok(());
        };
        if !meta.is_file() || meta.len() > MAX_FILE {
            return Err(format!("{ART_FILE} is not a file of a usable size."));
        }
        let text = fs::read_to_string(&path).map_err(|e| format!("{ART_FILE}: {e}"))?;
        let block = json_block(&text).ok_or_else(|| format!("{ART_FILE}: no JSON block."))?;
        let file: ArtFile = serde_json::from_str(block).map_err(|e| format!("{ART_FILE}: {e}"))?;
        if file.splat_art != 1 {
            return Err(format!("{ART_FILE}: unsupported version."));
        }
        // Many targets share an image, and a slow sync can leave images still to arrive.
        let package = fs::canonicalize(root).map_err(|e| format!("{ART_FILE}: {e}"))?;
        let mut present = HashMap::new();
        let mut rejected = HashSet::new();
        for (targets, offers) in [
            (&mut self.entries, file.entries),
            (&mut self.links, file.links),
        ] {
            for (target, relative) in offers {
                if !usable(&relative) {
                    rejected.insert(relative);
                    continue;
                }
                let image = root.join(&relative);
                // Resolved, because a linked folder could lead out of the package.
                if *present.entry(relative).or_insert_with(|| {
                    fs::canonicalize(&image)
                        .is_ok_and(|real| real.starts_with(&package) && real.is_file())
                }) {
                    targets.entry(target).or_insert(image);
                }
            }
        }
        match rejected.iter().next() {
            Some(relative) => Err(format!("{ART_FILE}: unusable image path: {relative}")),
            None => Ok(()),
        }
    }

    pub fn entry(&self, key: &str) -> Option<&Path> {
        self.entries.get(key).map(PathBuf::as_path)
    }

    pub fn link(&self, url: &str) -> Option<&Path> {
        self.links.get(url).map(PathBuf::as_path)
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.links.is_empty()
    }
}

fn json_block(note: &str) -> Option<&str> {
    let (_, rest) = note.split_once("```json\n")?;
    Some(rest.split_once("\n```")?.0)
}

/// Inside the package, and of a format that the app can draw.
fn usable(relative: &str) -> bool {
    let path = Path::new(relative);
    !relative.is_empty()
        && path
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
        && path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| FORMATS.contains(&extension.to_ascii_lowercase().as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package(art: &str, images: &[&str]) -> tempfile::TempDir {
        let root = tempfile::tempdir().unwrap();
        fs::write(
            root.path().join(ART_FILE),
            format!("# Art\n\n```json\n{art}\n```\n"),
        )
        .unwrap();
        for image in images {
            let path = root.path().join(image);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, b"image").unwrap();
        }
        root
    }

    #[test]
    fn finds_art_by_entry_key_and_by_link() {
        let root = package(
            r#"{"splat_art":1,"entries":{"dnd2024:creatures/goblin":"Creatures/goblin.webp"},"links":{"https://example.com/map.png":"Figures/map.webp"}}"#,
            &["Creatures/goblin.webp", "Figures/map.webp"],
        );
        let index = ArtIndex::load([root.path()]);
        assert_eq!(
            index.entry("dnd2024:creatures/goblin"),
            Some(root.path().join("Creatures/goblin.webp").as_path())
        );
        assert_eq!(
            index.link("https://example.com/map.png"),
            Some(root.path().join("Figures/map.webp").as_path())
        );
        assert_eq!(index.entry("dnd2024:creatures/orc"), None);
    }

    #[test]
    fn a_package_without_art_adds_nothing() {
        let root = tempfile::tempdir().unwrap();
        let mut index = ArtIndex::default();
        index.add_package(root.path()).unwrap();
        assert!(index.is_empty());
    }

    #[test]
    fn the_package_registered_first_wins() {
        let art = r#"{"splat_art":1,"entries":{"a:b":"Art/b.webp"}}"#;
        let first = package(art, &["Art/b.webp"]);
        let second = package(art, &["Art/b.webp"]);
        let index = ArtIndex::load([first.path(), second.path()]);
        assert!(index.entry("a:b").unwrap().starts_with(first.path()));
    }

    #[test]
    fn an_image_still_to_arrive_is_left_out() {
        let root = package(r#"{"splat_art":1,"entries":{"a:b":"Art/b.webp"}}"#, &[]);
        let mut index = ArtIndex::default();
        index.add_package(root.path()).unwrap();
        assert_eq!(index.entry("a:b"), None);
    }

    #[test]
    fn refuses_paths_outside_the_package_and_formats_it_cannot_draw() {
        for path in [
            "../secret.webp",
            "/etc/secret.webp",
            "Art/../../x.png",
            "Art/x.svg",
            "",
        ] {
            let root = package(
                &format!(r#"{{"splat_art":1,"entries":{{"a:b":{path:?}}}}}"#),
                &[],
            );
            let mut index = ArtIndex::default();
            assert!(index.add_package(root.path()).is_err(), "{path}");
            assert_eq!(index.entry("a:b"), None, "{path}");
        }
    }

    #[cfg(unix)]
    #[test]
    fn a_linked_folder_cannot_lead_out_of_the_package() {
        let outside = tempfile::tempdir().unwrap();
        fs::write(outside.path().join("private.webp"), b"image").unwrap();
        let root = package(
            r#"{"splat_art":1,"entries":{"a:b":"Art/private.webp"}}"#,
            &[],
        );
        std::os::unix::fs::symlink(outside.path(), root.path().join("Art")).unwrap();
        let mut index = ArtIndex::default();
        index.add_package(root.path()).unwrap();
        assert_eq!(index.entry("a:b"), None);
    }

    #[test]
    fn a_note_with_no_json_block_is_an_error() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join(ART_FILE), "# Art\n").unwrap();
        assert!(ArtIndex::default().add_package(root.path()).is_err());
    }

    #[test]
    fn refuses_bad_files_and_other_versions() {
        for art in ["not json", r#"{"splat_art":2}"#, r#"{"entries":{}}"#] {
            let root = package(art, &[]);
            assert!(
                ArtIndex::default().add_package(root.path()).is_err(),
                "{art}"
            );
        }
    }
}
