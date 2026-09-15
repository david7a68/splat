//! Campaign run location and the selected initiative rule.
use crate::{
    content_workspace::{CampaignDocument, Catalogue, EntryRef, patch_fields},
    library::{note_path, write_note},
    persistence::Campaign,
};
use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;
use splat::content::Entry;
use std::{
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

const MECHANIC: &str = "dnd2024:rules/combat_initiative";
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Grouping {
    #[default]
    Identical,
    Individual,
    BySide,
    Explicit,
}
impl Grouping {
    pub fn label(self) -> &'static str {
        match self {
            Self::Identical => "Identical NPCs (D&D default)",
            Self::Individual => "Each NPC separately",
            Self::BySide => "One roll per side",
            Self::Explicit => "Encounter initiative groups",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Initiative {
    pub grouping: Grouping,
    pub roster_ties: bool,
}
impl Default for Initiative {
    fn default() -> Self {
        Self {
            grouping: Grouping::Identical,
            roster_ties: false,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    pub folder: String,
    pub initiative: Initiative,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            folder: "Encounter Runs".into(),
            initiative: Initiative::default(),
        }
    }
}
impl Options {
    pub fn load(campaign: &Campaign, catalogue: &Catalogue) -> Result<Self, String> {
        let document = CampaignDocument::load(campaign)?;
        let entry = Entry::parse(&document.path, &document.text)?;
        let folder = match entry
            .fields
            .get("splat_run_storage")
            .and_then(|v| v.get("directory"))
        {
            Some(v) => v.as_str().ok_or("Run directory must be text.")?.into(),
            None => "Encounter Runs".into(),
        };
        let mut result = Self {
            folder,
            ..Default::default()
        };
        result.directory(campaign)?;
        if let Some(reference) = entry
            .fields
            .get("splat_rule_choices")
            .and_then(|v| v.get(MECHANIC))
        {
            let reference: EntryRef =
                serde_yaml_ng::from_value(reference.clone()).map_err(|e| e.to_string())?;
            let rule = &catalogue.entries[&catalogue.resolve_ref(&reference)?].entry;
            let params = rule
                .fields
                .get("splat_rule")
                .and_then(|v| v.get("parameters"))
                .ok_or("Selected initiative rule has no parameters.")?;
            result.initiative.grouping = match params.get("grouping").and_then(Value::as_str) {
                Some("identical") => Grouping::Identical,
                Some("individual") => Grouping::Individual,
                Some("by_side") => Grouping::BySide,
                Some("explicit") => Grouping::Explicit,
                None if params
                    .get("identical_monster_group")
                    .and_then(Value::as_str)
                    == Some("one_shared_roll") =>
                {
                    Grouping::Identical
                }
                _ => {
                    return Err(
                        "The selected initiative grouping is not supported by the DM runner."
                            .into(),
                    );
                }
            };
            result.initiative.roster_ties = match params.get("tie").and_then(Value::as_str) {
                None | Some("core_rule") => false,
                Some("roster") => true,
                _ => return Err("The selected initiative tie rule is not supported.".into()),
            };
            if let Some(value) = params.get("within_group").and_then(Value::as_str)
                && !["roster", "controller"].contains(&value)
            {
                return Err("Unsupported initiative group order.".into());
            }
        }
        Ok(result)
    }
    pub fn directory(&self, campaign: &Campaign) -> Result<PathBuf, String> {
        let name = self.folder.trim();
        if name.is_empty() {
            return Err("Enter a run folder.".into());
        }
        let path = Path::new(name);
        if path
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err("Run folders cannot contain '..'.".into());
        }
        Ok(if path.is_absolute() {
            path.into()
        } else {
            campaign.path.join(path)
        })
    }
    pub fn save(&self, campaign: &Campaign, baseline: &str) -> Result<(), String> {
        self.directory(campaign)?;
        let document = CampaignDocument::load(campaign)?;
        if document.text != baseline {
            return Err("Campaign settings changed on disk. Reload before saving.".into());
        }
        let entry = Entry::parse(&document.path, &document.text)?;
        let relative = "Rules/Encounter Initiative.md";
        let rule_path = note_path(&campaign.path, relative)?;
        let previous = match fs::read_to_string(&rule_path) {
            Ok(t) => Some(t),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
            Err(e) => return Err(e.to_string()),
        };
        let rule_id = if let Some(t) = &previous {
            let rule = Entry::parse(relative, t)?;
            if rule
                .fields
                .get("splat_run_settings")
                .and_then(Value::as_bool)
                != Some(true)
            {
                return Err("Rules/Encounter Initiative.md already contains another note. Move it before saving initiative settings.".into());
            }
            rule.id
        } else {
            Uuid::new_v4()
        };
        let grouping = match self.initiative.grouping {
            Grouping::Identical => "identical",
            Grouping::Individual => "individual",
            Grouping::BySide => "by_side",
            Grouping::Explicit => "explicit",
        };
        let value: Value = serde_yaml_ng::from_str(&format!("mechanic: {MECHANIC}\nmode: replace\nimplementation: splat:manual_initiative\nparameters:\n  grouping: {grouping}\n  within_group: {}\n  tie: {}\n  group_bonus: first_npc_in_roster\n", if self.initiative.roster_ties {"roster"}else{"controller"}, if self.initiative.roster_ties {"roster"}else{"core_rule"})).map_err(|e|e.to_string())?;
        let source = previous.clone().unwrap_or_else(|| format!("---\nsplat_schema: 1\nsplat_id: {rule_id}\nsplat_kind: rule\nsplat_name: Campaign initiative\nsplat_run_settings: true\nsplat_rule: {{}}\n---\n\nNPC group rolls use the first NPC in roster order as the group's initiative bonus and roll mode. Player initiative is entered separately.\n"));
        let rule = patch_fields(&source, &[("splat_rule", Some(value))])?;
        let mut choices = entry
            .fields
            .get("splat_rule_choices")
            .and_then(Value::as_mapping)
            .cloned()
            .unwrap_or_default();
        choices.insert(
            MECHANIC.into(),
            serde_yaml_ng::to_value(EntryRef::id(rule_id)).map_err(|e| e.to_string())?,
        );
        let text = patch_fields(
            &document.text,
            &[
                ("splat_rule_choices", Some(choices.into())),
                (
                    "splat_run_storage",
                    Some(
                        serde_yaml_ng::to_value(
                            serde_json::json!({"directory":self.folder.trim()}),
                        )
                        .map_err(|e| e.to_string())?,
                    ),
                ),
            ],
        )?;
        fs::create_dir_all(rule_path.parent().ok_or("Invalid rules folder")?)
            .map_err(|e| e.to_string())?;
        write_note(&campaign.path, relative, previous.as_deref(), &rule)?;
        write_note(
            &campaign.path,
            &document.path,
            document.baseline.as_deref(),
            &text,
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overrides_roundtrip_and_external_manifest_edits_are_preserved() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        let document = CampaignDocument::load(&campaign).unwrap();
        let options = Options {
            folder: "Sessions/Combat".into(),
            initiative: Initiative {
                grouping: Grouping::BySide,
                roster_ties: true,
            },
        };
        options.save(&campaign, &document.text).unwrap();
        let catalogue = Catalogue::load(&[], Some(&campaign)).unwrap();
        assert_eq!(Options::load(&campaign, &catalogue).unwrap(), options);
        assert_eq!(
            options.directory(&campaign).unwrap(),
            dir.path().join("Sessions/Combat")
        );
        let text = fs::read_to_string(dir.path().join("Campaign.md")).unwrap();
        assert!(options.save(&campaign, &document.text).is_err());
        assert_eq!(
            fs::read_to_string(dir.path().join("Campaign.md")).unwrap(),
            text
        );
        let invalid = Options {
            folder: "../escape".into(),
            ..options
        };
        assert!(invalid.directory(&campaign).is_err());
    }
}
