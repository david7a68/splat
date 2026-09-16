//! Generated campaign content. Preflight every note before replacing any of the bundle.
use super::EntitySnapshot;
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
const MARKER: &str = "\n<!-- splat-generated-sha256:";

impl EntitySnapshot {
    pub fn campaign_path(&self, campaign: &Path) -> PathBuf {
        campaign
            .join("Characters")
            .join("D&D Beyond")
            .join(format!("{}.md", self.definition.splat_id))
    }
    pub fn save_campaign_copy(&self, campaign: &Path) -> Result<PathBuf, String> {
        self.validate()?;
        if !campaign.is_absolute() || !campaign.is_dir() {
            return Err("Campaign folder is unavailable; character copy was not saved.".into());
        }
        let root = campaign.join("Characters").join("D&D Beyond");
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
                "\n## Personal notes\n\nThis section is kept when the sheet refreshes.\n"
            };
            let hash = format!("{:x}", Sha256::digest(note.markdown.as_bytes()));
            let text = format!("{}{MARKER}{hash} -->\n{notes}", note.markdown);
            staged.push((path, previous, text));
        }
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
        Ok(self.campaign_path(campaign))
    }
}
fn conflict(path: &Path) -> String {
    format!(
        "Campaign character copy has local edits; kept unchanged: {}. Move it aside to resume generated updates.",
        path.display()
    )
}
