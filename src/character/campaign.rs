//! User-visible campaign copies. Generated text is checksummed; personal notes survive refresh.
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
        let path = self.campaign_path(campaign);
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
        let mut text = self.entry_markdown()?;
        text.push_str(&format!("\n# {}\n\nLevel {} {}. HP: {}; temporary HP: {}.\n\n[Open D&D Beyond sheet](https://www.dndbeyond.com/characters/{})\n\nFetched at Unix time {}. Generated content above Personal notes is managed by Splat.\n\n## Equipment\n\n", clean(&self.definition.splat_name), self.character.level, clean(&self.character.species), self.initial.hp, self.initial.temporary_hp, self.source.character_id, self.source.fetched_at));
        for item in &self.inventory {
            text.push_str(&format!(
                "- {} × {}{}{}\n",
                item.quantity,
                clean(&item.name),
                if item.equipped { " (equipped)" } else { "" },
                if item.attuned_to.is_some() {
                    " (attuned)"
                } else {
                    ""
                }
            ));
        }
        // This is a source-detail block, not an authoritative authored inventory table.
        // It retains all normalized fields and both complete API responses without inventing rules.
        let json =
            serde_json::to_string_pretty(self).map_err(|_| "Could not encode campaign copy.")?;
        let fence = "`".repeat(
            json.split(|c| c != '`')
                .map(str::len)
                .max()
                .unwrap_or(0)
                .max(3)
                + 1,
        );
        text.push_str(&format!("\n## Complete character data\n\nThe data below includes stats, resources, attacks, spells, features, inventory instances, and both source responses.\n\n<details>\n<summary>Character data (JSON)</summary>\n\n{fence}json\n{json}\n{fence}\n\n</details>\n"));
        let hash = format!("{:x}", Sha256::digest(text.as_bytes()));
        text.push_str(&format!("{MARKER}{hash} -->\n{notes}"));
        let parent = path.parent().ok_or("Invalid campaign copy path.")?;
        fs::create_dir_all(parent).map_err(|_| "Could not create campaign character folder.")?;
        let mut temp = tempfile::NamedTempFile::new_in(parent)
            .map_err(|_| "Could not stage campaign copy.")?;
        temp.write_all(text.as_bytes())
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|_| "Could not write campaign copy.")?;
        // Detect edits made while rendering. Each replacement is atomic, though external editors
        // must still avoid saving at the exact same instant as the final rename.
        if previous.is_some() {
            if fs::read_to_string(&path).ok() != previous {
                return Err(conflict(&path));
            }
            temp.persist(&path)
                .map_err(|_| "Could not replace campaign copy.")?;
        } else {
            temp.persist_noclobber(&path).map_err(|_| conflict(&path))?;
        }
        Ok(path)
    }
}
fn conflict(path: &Path) -> String {
    format!(
        "Campaign character copy has local edits; kept unchanged: {}. Move it aside to resume generated updates.",
        path.display()
    )
}
fn clean(text: &str) -> String {
    text.replace(['\r', '\n'], " ")
        .replace(['<', '>'], "")
        .replace(['[', ']', '*', '_', '`', '#'], "")
}
