//! Discover, migrate, and update the campaign manifest without replacing other settings.
use crate::{
    content_workspace::{CampaignDocument, patch_fields},
    library::write_note,
    persistence::Campaign,
};
use splat::{
    campaign_storage::{self, Identity},
    content::Entry,
};
use std::path::Path;
use uuid::Uuid;

fn identity(c: &Campaign) -> Identity {
    Identity {
        version: 1,
        dndbeyond_campaign_id: c.dndbeyond_campaign_id.clone(),
        dndbeyond_campaign_name: c.dndbeyond_campaign_name.clone(),
    }
}
fn apply(c: &mut Campaign, entry: &Entry) -> Result<(), String> {
    c.name = entry.name.clone();
    if let Some(value) = Identity::read(entry)? {
        c.dndbeyond_campaign_id = value.dndbeyond_campaign_id;
        c.dndbeyond_campaign_name = value.dndbeyond_campaign_name;
    }
    Ok(())
}

pub fn open(path: &Path, remote: Option<(&str, &str)>) -> Result<Campaign, String> {
    let path = path
        .canonicalize()
        .map_err(|e| format!("Choose an existing campaign folder: {e}"))?;
    let entry = campaign_storage::discover(&path)?;
    if entry.is_none() && remote.is_none() {
        return Err("No campaign manifest found in this folder.".into());
    }
    let mut c = Campaign {
        id: entry.as_ref().map_or_else(Uuid::new_v4, |e| e.id),
        name: remote.map_or("", |(_, name)| name).into(),
        path,
        dndbeyond_campaign_id: None,
        dndbeyond_campaign_name: None,
    };
    if let Some(entry) = &entry {
        apply(&mut c, entry)?;
    }
    if let Some((id, name)) = remote {
        let id = id
            .parse::<u64>()
            .ok()
            .filter(|id| *id > 0)
            .ok_or("Invalid D&D Beyond campaign ID.")?
            .to_string();
        if c.dndbeyond_campaign_id
            .as_ref()
            .is_some_and(|current| current != &id)
        {
            return Err("This folder is linked to another D&D Beyond campaign. Choose its campaign or a different folder.".into());
        }
        c.dndbeyond_campaign_id = Some(id);
        c.dndbeyond_campaign_name = Some(name.into());
    }
    Ok(c)
}

/// Existing manifests are authoritative. Missing folders stay registered.
pub fn reload(c: &mut Campaign) -> Result<(), String> {
    if !c.path.exists() {
        return Ok(());
    }
    let entry = campaign_storage::discover(&c.path)?;
    if let Some(entry) = &entry {
        if entry.id != c.id {
            return Err("Campaign manifest identity differs from its registration.".into());
        }
        apply(c, entry)?;
    }
    if let Some(id) = &mut c.dndbeyond_campaign_id {
        *id = id
            .parse::<u64>()
            .ok()
            .filter(|id| *id > 0)
            .ok_or("Invalid D&D Beyond campaign ID.")?
            .to_string();
    }
    if entry
        .as_ref()
        .is_none_or(|e| !e.fields.contains_key("splat_campaign"))
    {
        save(c, None)?;
    }
    Ok(())
}

pub fn save(c: &Campaign, previous: Option<&Campaign>) -> Result<(), String> {
    let document = CampaignDocument::load(c)?;
    let entry = Entry::parse(&document.path, &document.text)?;
    let actual = Identity::read(&entry)?;
    if let Some(previous) = previous
        && let Some(actual) = actual
        && (actual != identity(previous) || entry.name != previous.name)
    {
        return Err("Campaign settings changed on disk. Reload before saving.".into());
    }
    let text = patch_fields(
        &document.text,
        &[
            ("splat_name", Some(c.name.clone().into())),
            (
                "splat_campaign",
                Some(serde_yaml_ng::to_value(identity(c)).map_err(|e| e.to_string())?),
            ),
        ],
    )?;
    if document.baseline.as_deref() != Some(&text) {
        write_note(&c.path, &document.path, document.baseline.as_deref(), &text)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn import_reopen_and_move_preserve_identity_paths_and_notes() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("original");
        fs::create_dir(&root).unwrap();
        let campaign = open(&root, Some(("012", "Remote name"))).unwrap();
        assert!(!root.join("Campaign.md").exists()); // Selection alone has no effects.
        save(&campaign, None).unwrap();
        let file = root.join("Campaign.md");
        let text = fs::read_to_string(&file).unwrap();
        let text = patch_fields(
            &text,
            &[
                (
                    "splat_storage",
                    Some(
                        serde_yaml_ng::from_str(
                            "characters: Party/Sheets\ncharacter_cache: Data/Cache",
                        )
                        .unwrap(),
                    ),
                ),
                (
                    "splat_run_storage",
                    Some(serde_yaml_ng::from_str("directory: Sessions/Combat").unwrap()),
                ),
            ],
        )
        .unwrap()
            + "\nKeep my notes.\n";
        fs::write(&file, &text).unwrap();
        let moved = temp.path().join("moved");
        fs::rename(&root, &moved).unwrap();
        let reopened = open(&moved, None).unwrap();
        assert_eq!(reopened.id, campaign.id);
        assert_eq!(reopened.name, "Remote name");
        assert_eq!(reopened.dndbeyond_campaign_id.as_deref(), Some("12"));
        let storage = campaign_storage::Storage::load(&moved).unwrap();
        assert_eq!(storage.characters, "Party/Sheets");
        assert_eq!(storage.character_cache, "Data/Cache");
        assert_eq!(fs::read_to_string(moved.join("Campaign.md")).unwrap(), text);
        assert!(open(&moved, Some(("34", "Other"))).is_err());
        assert!(campaign_storage::Storage::for_link(&moved, Some(campaign.id), "34").is_err());
        assert!(campaign_storage::Storage::for_link(&moved, Some(Uuid::new_v4()), "12").is_err());
    }

    #[test]
    fn migration_retains_manifest_settings_and_stops_on_external_identity_edits() {
        let root = tempfile::tempdir().unwrap();
        let mut campaign = open(root.path(), Some(("12", "Old name"))).unwrap();
        let document = CampaignDocument::load(&campaign).unwrap();
        let text = document.text + "\nPersonal notes.\n";
        fs::write(root.path().join("Notes.md"), &text).unwrap();
        reload(&mut campaign).unwrap();
        assert!(!root.path().join("Campaign.md").exists());
        let file = root.path().join("Notes.md");
        let migrated = fs::read_to_string(&file).unwrap();
        assert!(migrated.contains("Personal notes."));
        assert!(migrated.contains("splat_packages:"));
        let external = patch_fields(
            &migrated,
            &[("splat_name", Some("Edited elsewhere".into()))],
        )
        .unwrap();
        fs::write(&file, &external).unwrap();
        let mut changed = campaign.clone();
        changed.name = "App edit".into();
        assert!(save(&changed, Some(&campaign)).is_err());
        assert_eq!(fs::read_to_string(&file).unwrap(), external);
        reload(&mut campaign).unwrap();
        assert_eq!(campaign.name, "Edited elsewhere");
    }

    #[test]
    fn invalid_duplicate_and_future_manifests_are_not_replaced() {
        let root = tempfile::tempdir().unwrap();
        let c = open(root.path(), Some(("12", "Sample"))).unwrap();
        save(&c, None).unwrap();
        let file = root.path().join("Campaign.md");
        let text = fs::read_to_string(&file).unwrap();
        fs::write(root.path().join("Duplicate.md"), &text).unwrap();
        assert!(open(root.path(), None).is_err());
        fs::remove_file(root.path().join("Duplicate.md")).unwrap();
        let future = text.replace("version: 1", "version: 99");
        fs::write(&file, &future).unwrap();
        assert!(open(root.path(), None).is_err());
        assert_eq!(fs::read_to_string(&file).unwrap(), future);
        fs::write(&file, "---\nsplat_id: [\n---\n").unwrap();
        assert!(open(root.path(), Some(("12", "Sample"))).is_err());
    }
}
