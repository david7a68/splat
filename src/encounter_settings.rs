//! Campaign run location and the selected initiative rule.
use crate::{
    content_workspace::{CampaignDocument, Catalogue, EntryRef},
    library::note_path,
    persistence::Campaign,
};
use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;
use splat::content::Entry;
use splat::content::inspiration::{self, Inspiration};
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
    pub const ALL: [Self; 4] = [
        Self::Identical,
        Self::Individual,
        Self::BySide,
        Self::Explicit,
    ];
    /// Reads on from "Roll NPC initiative".
    pub fn label(self) -> &'static str {
        match self {
            Self::Identical => "In groups of identical NPCs (D&D default)",
            Self::Individual => "Individually",
            Self::BySide => "In one group per side",
            Self::Explicit => "In the encounter's initiative groups",
        }
    }
    /// How an encounter group takes its initiative under this rule. A typed total is the
    /// DM's ruling, and it applies to every member under any rule.
    pub fn arrival_rule(self) -> &'static str {
        match self {
            Self::Identical => {
                "A group rolls once, with the first member’s bonus, even with different creatures."
            }
            Self::Individual => "Each member of a group rolls alone, by the campaign's rule.",
            Self::BySide => {
                "A group takes the initiative its side already has. A side with none rolls once."
            }
            Self::Explicit => {
                "A group rolls once, with the first member’s bonus, as its own initiative group."
            }
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
/// An inspiration rule the campaign can use, from its packages or its own notes.
#[derive(Clone, Debug, PartialEq)]
pub struct InspirationRule {
    pub id: Uuid,
    pub name: String,
    pub effect: Inspiration,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    pub folder: String,
    pub initiative: Initiative,
    /// The inspiration rules in the campaign's packages and notes. No ruleset comes first.
    pub inspiration_rules: Vec<InspirationRule>,
    /// The chosen rule, when there are several. With one, it is that one.
    pub inspiration_choice: Option<Uuid>,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            folder: "Encounter Runs".into(),
            initiative: Initiative::default(),
            inspiration_rules: Vec::new(),
            inspiration_choice: None,
        }
    }
}
impl Options {
    pub fn load(campaign: &Campaign, catalogue: &Catalogue) -> Result<Self, String> {
        let document = CampaignDocument::load(campaign)?;
        let mut entry = Entry::parse(&document.path, &document.text)?;
        splat::content::list_formats::resolve(
            &mut entry,
            &catalogue
                .entries
                .values()
                .map(|e| e.entry.clone())
                .collect::<Vec<_>>(),
            &document.text,
        );
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
        result.inspiration_rules = inspiration_rules(campaign, catalogue, &document);
        result.inspiration_choice = chosen_inspiration(
            entry
                .fields
                .get("splat_rule_choices")
                .and_then(|v| v.get(inspiration::CHOICE)),
            catalogue,
            &result.inspiration_rules,
        );
        Ok(result)
    }
    /// What inspiration does in this campaign's runs: its only rule, or the chosen one of
    /// several. `None` when its packages have no inspiration rule or none is chosen.
    pub fn inspiration(&self) -> Option<Inspiration> {
        let rule = match self.inspiration_rules.as_slice() {
            [only] => Some(only),
            rules => rules
                .iter()
                .find(|rule| Some(rule.id) == self.inspiration_choice),
        };
        rule.map(|rule| rule.effect)
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
    /// Existing run files that a folder change would hide, including unreadable saves.
    pub fn runs_left_in(&self, old: &Self, campaign: &Campaign) -> Result<Vec<PathBuf>, String> {
        let from = old.directory(campaign)?;
        let to = self.directory(campaign)?;
        if from == to || !from.try_exists().map_err(|e| e.to_string())? {
            return Ok(Vec::new());
        }
        if to.exists() && fs::canonicalize(&from).ok() == fs::canonicalize(&to).ok() {
            return Ok(Vec::new());
        }
        fs::read_dir(from)
            .map_err(|e| e.to_string())?
            .map(|file| {
                let file = file.map_err(|e| e.to_string())?;
                Ok((file.file_type().map_err(|e| e.to_string())?.is_file()
                    && file
                        .path()
                        .extension()
                        .is_some_and(|extension| extension == "json"))
                .then(|| file.path()))
            })
            .collect::<Result<Vec<_>, String>>()
            .map(|files| files.into_iter().flatten().collect())
    }

    /// Stage every save without replacing a file. Change settings only after all copies exist.
    pub fn move_runs_and_save(
        &self,
        old: &Self,
        campaign: &Campaign,
        baseline: &str,
    ) -> Result<Option<String>, String> {
        let files = self.runs_left_in(old, campaign)?;
        let target = self.directory(campaign)?;
        fs::create_dir_all(&target).map_err(|e| format!("Could not create run folder: {e}"))?;
        let mut copied = Vec::new();
        let result = (|| {
            for source in &files {
                let destination = target.join(source.file_name().ok_or("Invalid run file name")?);
                let bytes = fs::read(source).map_err(|e| e.to_string())?;
                let mut staged =
                    tempfile::NamedTempFile::new_in(&target).map_err(|e| e.to_string())?;
                use std::io::Write;
                staged
                    .write_all(&bytes)
                    .and_then(|_| staged.as_file().sync_all())
                    .map_err(|e| e.to_string())?;
                staged.persist_noclobber(&destination).map_err(|e| {
                    format!("Could not move runs. No existing file was replaced: {e}")
                })?;
                copied.push((source.clone(), destination, bytes));
            }
            self.save(campaign, baseline)
        })();
        if let Err(error) = result {
            for (_, destination, _) in &copied {
                let _ = fs::remove_file(destination);
            }
            return Err(error);
        }
        let mut warnings = Vec::new();
        for (source, _, bytes) in copied {
            if fs::read(&source).ok().as_deref() == Some(bytes.as_slice()) {
                if let Err(e) = fs::remove_file(&source) {
                    warnings.push(format!("Could not remove {}: {e}", source.display()));
                }
            } else {
                warnings.push(format!(
                    "{} changed during the move and stays in the old folder.",
                    source.display()
                ));
            }
        }
        Ok((!warnings.is_empty())
            .then(|| format!("Settings saved and runs copied. {}", warnings.join(" "))))
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
        let source = previous.clone().unwrap_or_else(|| format!("---\nsplat_schema: 2\nsplat_id: {rule_id}\nsplat_kind: rule\nsplat_name: Campaign initiative\nsplat_run_settings: true\nsplat_rule: {{}}\n---\n\nNPC group rolls use the first NPC in roster order as the group's initiative bonus and roll mode. Player initiative is entered separately.\n"));
        let rule = splat::content::formats::patch_model(
            &source,
            &[
                ("splat_rule", Some(value)),
                (
                    "splat_requires_mechanics",
                    Some(vec![Value::from("splat:manual_initiative")].into()),
                ),
            ],
        )?;
        let mut choices = entry
            .fields
            .get("splat_rule_choices")
            .and_then(Value::as_mapping)
            .cloned()
            .unwrap_or_default();
        choices.insert(
            MECHANIC.into(),
            serde_yaml_ng::to_value(EntryRef {
                id: Some(rule_id),
                key: None,
                link: Some("Campaign initiative".into()),
            })
            .map_err(|e| e.to_string())?,
        );
        // A choice is kept only while there is something to choose between.
        match self
            .inspiration_choice
            .filter(|_| self.inspiration_rules.len() > 1)
            .and_then(|id| self.inspiration_rules.iter().find(|rule| rule.id == id))
        {
            Some(rule) => {
                choices.insert(
                    inspiration::CHOICE.into(),
                    serde_yaml_ng::to_value(EntryRef {
                        id: Some(rule.id),
                        key: None,
                        link: Some(rule.name.clone()),
                    })
                    .map_err(|e| e.to_string())?,
                );
            }
            None => {
                choices.remove(inspiration::CHOICE);
            }
        }
        let text = splat::content::formats::patch_model(
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
        crate::file_batch::apply(&[
            crate::file_batch::Change::text(rule_path, previous.as_deref(), Some(&rule)),
            crate::file_batch::Change::text(
                note_path(&campaign.path, &document.path)?,
                document.baseline.as_deref(),
                Some(&text),
            ),
        ])?;
        Ok(())
    }
}

/// The campaign's saved choice of inspiration rule, if it still names one of its rules. A
/// choice that is gone, such as a deleted note, is no choice: the rest of the settings still
/// load, and the DM chooses again where there are several.
fn chosen_inspiration(
    saved: Option<&Value>,
    catalogue: &Catalogue,
    rules: &[InspirationRule],
) -> Option<Uuid> {
    let reference = serde_yaml_ng::from_value::<EntryRef>(saved?.clone()).ok()?;
    let id = catalogue.resolve_ref(&reference).ok()?;
    rules.iter().any(|rule| rule.id == id).then_some(id)
}

/// The inspiration rules a campaign can use: its own notes, and its packages' entries.
fn inspiration_rules(
    campaign: &Campaign,
    catalogue: &Catalogue,
    document: &CampaignDocument,
) -> Vec<InspirationRule> {
    let closure = catalogue.closure(&document.selections, campaign.id);
    let mut rules: Vec<_> = catalogue
        .entries
        .iter()
        .filter(|(id, e)| {
            (e.private && e.owner == campaign.id)
                || closure.direct.contains(*id)
                || closure.required.contains_key(*id)
        })
        .filter_map(|(id, e)| {
            let effect = Inspiration::from_rule(e.entry.fields.get("splat_rule")?)?.ok()?;
            Some(InspirationRule {
                id: *id,
                name: e.entry.name.clone(),
                effect,
            })
        })
        .collect();
    rules.sort_by_key(|rule| rule.name.to_lowercase());
    rules
}

#[cfg(test)]
mod tests {
    use super::*;
    fn inspiration_note(root: &Path, name: &str, rule: &str) {
        fs::create_dir_all(root.join("Rules")).unwrap();
        let old = format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: rule\nsplat_name: {name}\nsplat_rule: {rule}\n---\n\n# {name}\n",
            Uuid::new_v4()
        );
        let text = splat::content::formats::convert(&format!("{name}.md"), &old).unwrap();
        fs::write(root.join(format!("Rules/{name}.md")), text).unwrap();
    }

    #[test]
    fn no_inspiration_rule_comes_first() {
        // Without a rule in the campaign's packages or notes, there is no inspiration.
        assert_eq!(Options::default().inspiration(), None);
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let load = || {
            let catalogue = Catalogue::load(&[], Some(&campaign)).unwrap();
            Options::load(&campaign, &catalogue).unwrap()
        };
        // A new campaign uses the SRD 5.2.1 package, and so its rule.
        assert_eq!(load().inspiration(), Some(Inspiration::Reroll));
        inspiration_note(
            dir.path(),
            "Inspiration",
            "{implementation: dnd2014:inspiration, parameters: {eligible: player_character, expend: {timing: when_making_d20_test, grants: advantage, applies_to: [attack_roll, saving_throw, ability_check]}}}",
        );
        let both = load();
        assert_eq!(both.inspiration_rules.len(), 2);
        assert_eq!(both.inspiration(), None, "two rules and no choice");
        let pick = |effect| {
            both.inspiration_rules
                .iter()
                .find(|rule| rule.effect == effect)
                .unwrap()
                .id
        };
        for effect in [Inspiration::Advantage, Inspiration::Reroll] {
            let chosen = Options {
                inspiration_choice: Some(pick(effect)),
                ..both.clone()
            };
            let document = CampaignDocument::load(&campaign).unwrap();
            chosen.save(&campaign, &document.text).unwrap();
            assert_eq!(load().inspiration(), Some(effect));
        }
        // The chosen note is deleted: the settings still load, and the one rule left applies.
        let document = CampaignDocument::load(&campaign).unwrap();
        Options {
            inspiration_choice: Some(pick(Inspiration::Advantage)),
            ..both.clone()
        }
        .save(&campaign, &document.text)
        .unwrap();
        fs::remove_file(dir.path().join("Rules/Inspiration.md")).unwrap();
        let left = load();
        assert_eq!(left.inspiration_choice, None);
        assert_eq!(left.inspiration(), Some(Inspiration::Reroll));
        // The same at the loader's own boundary, where a choice names an entry by its ID: one
        // that is gone, and one that is not an inspiration rule, are each no choice.
        let catalogue = Catalogue::load(&[], Some(&campaign)).unwrap();
        let saved = |id: Uuid| {
            serde_yaml_ng::to_value(EntryRef {
                id: Some(id),
                key: None,
                link: None,
            })
            .unwrap()
        };
        let gone = saved(Uuid::new_v4());
        assert!(
            catalogue
                .resolve_ref(&serde_yaml_ng::from_value(gone.clone()).unwrap())
                .is_err(),
            "this choice does not resolve, which once stopped the load"
        );
        assert_eq!(
            chosen_inspiration(Some(&gone), &catalogue, &left.inspiration_rules),
            None
        );
        let other = *catalogue
            .entries
            .keys()
            .find(|id| left.inspiration_rules.iter().all(|rule| rule.id != **id))
            .unwrap();
        assert_eq!(
            chosen_inspiration(Some(&saved(other)), &catalogue, &left.inspiration_rules),
            None
        );
        let kept = left.inspiration_rules[0].id;
        assert_eq!(
            chosen_inspiration(Some(&saved(kept)), &catalogue, &left.inspiration_rules),
            Some(kept)
        );
    }

    fn campaign(root: &Path) -> Campaign {
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Test".into(),
            path: root.into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        if campaign.path.is_dir() && !campaign.path.join("Campaign.md").exists() {
            crate::content_workspace::CampaignDocument::new(&campaign)
                .unwrap()
                .save()
                .unwrap();
        }
        campaign
    }

    #[test]
    fn folder_move_keeps_every_run_and_does_not_replace_a_destination() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let old = Options::default();
        let next = Options {
            folder: "Moved".into(),
            ..old.clone()
        };
        let source = old.directory(&campaign).unwrap();
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("run.json"), b"saved run").unwrap();
        fs::write(source.join("broken.json"), b"unreadable save").unwrap();
        let document = CampaignDocument::load(&campaign).unwrap();
        assert_eq!(next.runs_left_in(&old, &campaign).unwrap().len(), 2);
        let destination = next.directory(&campaign).unwrap();
        fs::create_dir_all(&destination).unwrap();
        fs::write(destination.join("run.json"), b"other run").unwrap();
        assert!(
            next.move_runs_and_save(&old, &campaign, &document.text)
                .is_err()
        );
        assert_eq!(
            fs::read(destination.join("run.json")).unwrap(),
            b"other run"
        );
        assert_eq!(fs::read(source.join("run.json")).unwrap(), b"saved run");
        assert!(!destination.join("broken.json").exists());
        fs::remove_file(destination.join("run.json")).unwrap();
        next.move_runs_and_save(&old, &campaign, &document.text)
            .unwrap();
        assert_eq!(
            fs::read(destination.join("run.json")).unwrap(),
            b"saved run"
        );
        assert_eq!(
            fs::read(destination.join("broken.json")).unwrap(),
            b"unreadable save"
        );
        assert!(!source.join("run.json").exists());
        let catalogue = Catalogue::load(&[], Some(&campaign)).unwrap();
        let loaded = Options::load(&campaign, &catalogue).unwrap();
        assert_eq!(
            (&loaded.folder, &loaded.initiative),
            (&next.folder, &next.initiative)
        );
    }

    #[test]
    fn a_settings_conflict_leaves_the_old_runs_and_removes_staged_copies() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = campaign(dir.path());
        let old = Options::default();
        let next = Options {
            folder: "Moved".into(),
            ..old.clone()
        };
        let source = old.directory(&campaign).unwrap();
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("run.json"), b"saved run").unwrap();
        assert!(
            next.move_runs_and_save(&old, &campaign, "stale manifest")
                .is_err()
        );
        assert_eq!(fs::read(source.join("run.json")).unwrap(), b"saved run");
        assert!(!next.directory(&campaign).unwrap().join("run.json").exists());
    }

    #[test]
    fn settings_roundtrip_and_external_manifest_edits_are_preserved() {
        let dir = tempfile::tempdir().unwrap();
        let campaign = Campaign {
            id: Uuid::new_v4(),
            name: "Sample".into(),
            path: dir.path().into(),
            dndbeyond_campaign_id: None,
            dndbeyond_campaign_name: None,
        };
        CampaignDocument::new(&campaign).unwrap().save().unwrap();
        let document = CampaignDocument::load(&campaign).unwrap();
        let options = Options {
            folder: "Sessions/Combat".into(),
            initiative: Initiative {
                grouping: Grouping::BySide,
                roster_ties: true,
            },
            ..Options::default()
        };
        options.save(&campaign, &document.text).unwrap();
        let catalogue = Catalogue::load(&[], Some(&campaign)).unwrap();
        let loaded = Options::load(&campaign, &catalogue).unwrap();
        assert_eq!(
            (&loaded.folder, &loaded.initiative),
            (&options.folder, &options.initiative)
        );
        assert_eq!(
            options.directory(&campaign).unwrap(),
            dir.path().join("Sessions/Combat")
        );
        let rule = "Rules/Encounter Initiative.md";
        let source = fs::read_to_string(dir.path().join(rule)).unwrap();
        let rule = Entry::parse(rule, &source).unwrap();
        assert_eq!(splat::content::validate(&[rule]).warnings.len(), 0);
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
