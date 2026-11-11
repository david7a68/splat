use super::{EntitySnapshot, VERSION};
use image::{DynamicImage, ImageFormat, ImageReader, Limits, imageops::FilterType};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};
use uuid::Uuid;
const MAX_CACHE: u64 = 32 * 1024 * 1024;
const PORTRAIT_SIZE: u32 = 512;
const MAX_PORTRAIT_SIDE: u32 = 8192;
#[derive(Clone)]
pub struct Cache {
    root: PathBuf,
}
impl Cache {
    pub fn user() -> Result<Self, String> {
        let dirs = directories::ProjectDirs::from("", "", "splat")
            .ok_or("Could not locate local application data.")?;
        Ok(Self::at(dirs.data_local_dir().join("characters")))
    }
    pub fn at(root: PathBuf) -> Self {
        Self { root }
    }
    pub fn directory(&self, campaign: Uuid, remote: &str) -> Result<PathBuf, String> {
        valid_id(remote)?;
        Ok(self.root.join(campaign.to_string()).join(remote))
    }
    pub fn path(&self, campaign: Uuid, remote: &str, character: &str) -> Result<PathBuf, String> {
        valid_id(character)?;
        Ok(self
            .directory(campaign, remote)?
            .join(format!("{character}.json")))
    }
    pub fn save(&self, campaign: Uuid, record: &EntitySnapshot) -> Result<PathBuf, String> {
        record.validate()?;
        let path = self.path(
            campaign,
            &record.source.campaign_id,
            &record.source.character_id,
        )?;
        let parent = path.parent().ok_or("Invalid character cache path.")?;
        fs::create_dir_all(parent)
            .map_err(|_| "Could not create the character cache directory.")?;
        let bytes = serde_json::to_vec_pretty(record)
            .map_err(|_| "Could not encode the character cache.")?;
        if bytes.len() as u64 > MAX_CACHE {
            return Err("The character cache record is too large.".into());
        }
        // A record the app cannot read must not be silently replaced. A record from an
        // earlier Splat is the exception: this one holds newer data from the same source.
        if path.exists() {
            read(&path)?;
        }
        let mut temp = tempfile::NamedTempFile::new_in(parent)
            .map_err(|_| "Could not stage the character cache.")?;
        temp.write_all(&bytes)
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|_| "Could not write the character cache.")?;
        temp.persist(&path)
            .map_err(|_| "Could not replace the character cache.")?;
        Ok(path)
    }
    pub fn save_roster(&self, campaign: Uuid, remote: &str, ids: &[String]) -> Result<(), String> {
        for id in ids {
            valid_id(id)?;
        }
        let dir = self.directory(campaign, remote)?;
        fs::create_dir_all(&dir).map_err(|_| "Could not create roster cache.")?;
        let mut temp =
            tempfile::NamedTempFile::new_in(&dir).map_err(|_| "Could not stage roster cache.")?;
        serde_json::to_writer(temp.as_file_mut(), ids)
            .map_err(|_| "Could not encode roster cache.")?;
        temp.as_file()
            .sync_all()
            .map_err(|_| "Could not write roster cache.")?;
        temp.persist(dir.join("roster-index"))
            .map_err(|_| "Could not replace roster cache.")?;
        Ok(())
    }
    /// Where a character's portrait is kept. The name changes with the portrait's address.
    pub fn portrait_path(
        &self,
        campaign: Uuid,
        remote: &str,
        character: &str,
        url: &str,
    ) -> Result<PathBuf, String> {
        valid_id(character)?;
        let address = url.split('?').next().unwrap_or(url);
        let hash = Sha256::digest(address.as_bytes());
        let name: String = hash[..8].iter().map(|b| format!("{b:02x}")).collect();
        Ok(self
            .directory(campaign, remote)?
            .join(format!("{character}-{name}.png")))
    }
    /// The portrait kept for a character, for a caller that does not have the sheet at hand.
    pub fn find_portrait(&self, campaign: Uuid, remote: &str, character: &str) -> Option<PathBuf> {
        valid_id(character).ok()?;
        let prefix = format!("{character}-");
        fs::read_dir(self.directory(campaign, remote).ok()?)
            .ok()?
            .flatten()
            .map(|item| item.path())
            .find(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(&prefix) && name.ends_with(".png"))
            })
    }
    /// Keep a downloaded portrait as a square PNG, in place of the character's older portrait.
    pub fn save_portrait(
        &self,
        campaign: Uuid,
        remote: &str,
        character: &str,
        url: &str,
        bytes: &[u8],
    ) -> Result<PathBuf, String> {
        let path = self.portrait_path(campaign, remote, character, url)?;
        let parent = path.parent().ok_or("Invalid portrait cache path.")?;
        let mut encoded = Vec::new();
        decode_portrait(bytes)?
            .resize_to_fill(PORTRAIT_SIZE, PORTRAIT_SIZE, FilterType::Lanczos3)
            .write_to(&mut Cursor::new(&mut encoded), ImageFormat::Png)
            .map_err(|_| "Could not encode the portrait.")?;
        fs::create_dir_all(parent).map_err(|_| "Could not create the portrait cache.")?;
        let mut temp =
            tempfile::NamedTempFile::new_in(parent).map_err(|_| "Could not stage the portrait.")?;
        temp.write_all(&encoded)
            .and_then(|_| temp.as_file().sync_all())
            .map_err(|_| "Could not write the portrait.")?;
        temp.persist(&path)
            .map_err(|_| "Could not replace the portrait.")?;
        let older = format!("{character}-");
        for item in fs::read_dir(parent).map_err(|_| "Could not read the portrait cache.")? {
            let other = item
                .map_err(|_| "Could not list the portrait cache.")?
                .path();
            let same_character = other
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(&older) && name.ends_with(".png"));
            if same_character && other != path {
                fs::remove_file(other).map_err(|_| "Could not remove an old portrait.")?;
            }
        }
        Ok(path)
    }
    pub fn load_all(&self, campaign: Uuid, remote: &str) -> Result<Vec<EntitySnapshot>, String> {
        let dir = self.directory(campaign, remote)?;
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let index = dir.join("roster-index");
        let ids: Option<Vec<String>> = if index.exists() {
            let bytes = fs::read(index).map_err(|_| "Could not read roster cache.")?;
            if bytes.len() > 65536 {
                return Err("Roster cache is too large.".into());
            }
            Some(serde_json::from_slice(&bytes).map_err(|_| "Invalid roster cache.")?)
        } else {
            None
        };
        let mut records = Vec::new();
        for item in
            fs::read_dir(dir).map_err(|_| "Could not read the character cache directory.")?
        {
            let path = item
                .map_err(|_| "Could not list character cache files.")?
                .path();
            if path.extension().is_some_and(|s| s == "json") {
                if ids.as_ref().is_some_and(|ids| {
                    !ids.iter()
                        .any(|id| path.file_stem().and_then(|s| s.to_str()) == Some(id))
                }) {
                    continue;
                }
                let Some(record) = read(&path)? else {
                    continue;
                };
                if record.source.campaign_id != remote
                    || path.file_stem().and_then(|s| s.to_str())
                        != Some(&record.source.character_id)
                {
                    return Err("Cached character identity does not match its path.".into());
                }
                records.push(record);
            }
        }
        records.sort_by(|a, b| a.definition.splat_name.cmp(&b.definition.splat_name));
        Ok(records)
    }
}
fn valid_id(id: &str) -> Result<(), String> {
    if id.parse::<u64>().ok().is_none_or(|v| v == 0) || !id.bytes().all(|b| b.is_ascii_digit()) {
        Err("Invalid D&D Beyond ID.".into())
    } else {
        Ok(())
    }
}
/// Bounded, because the bytes come from the network.
fn decode_portrait(bytes: &[u8]) -> Result<DynamicImage, String> {
    let mut limits = Limits::default();
    limits.max_image_width = Some(MAX_PORTRAIT_SIDE);
    limits.max_image_height = Some(MAX_PORTRAIT_SIDE);
    limits.max_alloc = Some(MAX_CACHE * 8);
    let mut reader = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|_| "Could not read the portrait.")?;
    reader.limits(limits);
    reader
        .decode()
        .map_err(|_| "The portrait is not a usable image.".into())
}
/// The version of any record, read before the rest of it.
#[derive(serde::Deserialize)]
struct Header {
    version: u32,
}

/// `None` is a record that an earlier Splat wrote. Splat does not read it; the next sync
/// replaces it.
fn read(path: &Path) -> Result<Option<EntitySnapshot>, String> {
    let file = fs::File::open(path).map_err(|_| "Could not read a cached character.")?;
    let mut bytes = Vec::new();
    file.take(MAX_CACHE + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Could not read a cached character.")?;
    if bytes.len() as u64 > MAX_CACHE {
        return Err("The cached character is too large.".into());
    }
    const UNREADABLE: &str = "Invalid or unsupported character cache. The file was kept.";
    let header: Header = serde_json::from_slice(&bytes).map_err(|_| UNREADABLE)?;
    if header.version < VERSION {
        return Ok(None);
    }
    let record: EntitySnapshot = serde_json::from_slice(&bytes).map_err(|_| UNREADABLE)?;
    record.validate()?;
    Ok(Some(record))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHARACTER: &str = "171105273";
    const REMOTE: &str = "8112390";

    fn png(width: u32, height: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        DynamicImage::new_rgb8(width, height)
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .unwrap();
        bytes
    }

    #[test]
    fn a_portrait_is_cached_as_a_square_and_a_new_one_replaces_it() {
        let root = tempfile::tempdir().unwrap();
        let cache = Cache::at(root.path().to_path_buf());
        let campaign = Uuid::new_v4();
        let first = "https://www.dndbeyond.com/avatars/1/1/first.jpeg?width=150";
        let saved = cache
            .save_portrait(campaign, REMOTE, CHARACTER, first, &png(300, 150))
            .unwrap();
        assert_eq!(image::image_dimensions(&saved).unwrap(), (512, 512));
        // The size options in an address do not make it a different portrait.
        assert_eq!(
            cache
                .portrait_path(
                    campaign,
                    REMOTE,
                    CHARACTER,
                    "https://www.dndbeyond.com/avatars/1/1/first.jpeg?width=999"
                )
                .unwrap(),
            saved
        );
        let second = "https://www.dndbeyond.com/avatars/1/1/second.jpeg";
        let replaced = cache
            .save_portrait(campaign, REMOTE, CHARACTER, second, &png(64, 64))
            .unwrap();
        assert_ne!(saved, replaced);
        assert_eq!(
            cache.find_portrait(campaign, REMOTE, CHARACTER).as_ref(),
            Some(&replaced)
        );
        assert_eq!(cache.find_portrait(campaign, REMOTE, "171105274"), None);
        assert!(!saved.exists());
        assert!(replaced.exists());
    }

    #[test]
    fn another_characters_portrait_stays() {
        let root = tempfile::tempdir().unwrap();
        let cache = Cache::at(root.path().to_path_buf());
        let campaign = Uuid::new_v4();
        let other = cache
            .save_portrait(campaign, REMOTE, "171105274", "https://a/b.png", &png(8, 8))
            .unwrap();
        cache
            .save_portrait(campaign, REMOTE, CHARACTER, "https://a/c.png", &png(8, 8))
            .unwrap();
        assert!(other.exists());
    }

    #[test]
    fn a_bad_portrait_leaves_the_old_one_and_the_roster_alone() {
        let root = tempfile::tempdir().unwrap();
        let cache = Cache::at(root.path().to_path_buf());
        let campaign = Uuid::new_v4();
        let saved = cache
            .save_portrait(campaign, REMOTE, CHARACTER, "https://a/b.png", &png(8, 8))
            .unwrap();
        assert!(
            cache
                .save_portrait(
                    campaign,
                    REMOTE,
                    CHARACTER,
                    "https://a/c.png",
                    b"not an image"
                )
                .is_err()
        );
        assert!(saved.exists());
        assert!(cache.load_all(campaign, REMOTE).unwrap().is_empty());
    }

    #[test]
    fn a_character_id_cannot_leave_the_cache() {
        let cache = Cache::at(std::env::temp_dir());
        assert!(
            cache
                .portrait_path(Uuid::new_v4(), REMOTE, "../../x", "https://a/b.png")
                .is_err()
        );
    }
}
