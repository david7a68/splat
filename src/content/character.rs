//! Provider-independent sheet data. Descriptive features are not executable actions.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sheet {
    pub level: u64,
    pub species: String,
    pub classes: Vec<Class>,
    pub checks: Vec<Check>,
    pub senses: Vec<Sense>,
    pub defenses: BTreeMap<String, Vec<String>>,
    pub casting: BTreeMap<String, Vec<CastingTotal>>,
    pub attacks: BTreeMap<String, Attack>,
    pub features: BTreeMap<String, Feature>,
    pub spells: BTreeMap<String, KnownSpell>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Attack {
    pub name: String,
    pub attack_bonus: Option<i64>,
    pub damage_type: String,
    pub damage_dice: Option<String>,
    pub damage_modifier: Option<i64>,
    pub damage_fixed: Option<i64>,
    pub damage_multiplier: Option<i64>,
    pub range_feet: Option<u64>,
    pub long_range_feet: Option<u64>,
    pub range_origin: String,
    pub requires_save_conversion: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Class {
    pub name: String,
    pub level: u64,
    pub subclass: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Check {
    pub name: String,
    pub stat: String,
    pub proficient: bool,
    pub expertise: bool,
    pub half_proficient: bool,
    pub advantage: bool,
    pub disadvantage: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sense {
    pub name: String,
    pub distance: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CastingTotal {
    pub value: i64,
    pub sources: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub activation: Option<String>,
    pub resource: Option<String>,
    pub source_key: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntryRef {
    pub id: Uuid,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnownSpell {
    pub entry: EntryRef,
    pub name: String,
    pub prepared: bool,
    pub always_prepared: bool,
    pub uses_spell_slot: bool,
    pub ability: Option<String>,
    pub save_dc: Option<i64>,
    pub cast_at_level: Option<u64>,
    pub restriction: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Import {
    pub version: u32,
    pub gaps: Vec<Gap>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    pub path: String,
    pub reason: String,
}

pub fn validate(entry: &super::Entry) -> Result<(), String> {
    if let Some(value) = entry.fields.get("splat_character") {
        if entry.kind != "creature" {
            return Err("splat_character requires a creature".into());
        }
        let sheet: Sheet = serde_yaml_ng::from_value(value.clone()).map_err(|e| e.to_string())?;
        if sheet.level == 0
            || sheet
                .classes
                .iter()
                .any(|c| c.level == 0 || c.name.is_empty())
        {
            return Err("Character and class levels must be positive".into());
        }
        for (key, feature) in &sheet.features {
            if !super::local_key(key) || feature.name.is_empty() {
                return Err("Invalid character feature".into());
            }
            if let Some(pool) = &feature.resource
                && entry
                    .fields
                    .get("splat_resources")
                    .and_then(|v| v.get(pool))
                    .is_none()
            {
                return Err(format!("Feature {key} refers to missing resource {pool}"));
            }
        }
        for (key, spell) in &sheet.spells {
            if !super::local_key(key) || spell.name.is_empty() {
                return Err("Invalid character spell".into());
            }
        }
        for (key, attack) in &sheet.attacks {
            if !super::local_key(key) || attack.name.is_empty() {
                return Err("Invalid character attack profile".into());
            }
            if attack.damage_fixed.is_some() && attack.damage_dice.is_some() {
                return Err("Attack profile cannot have both fixed damage and damage dice".into());
            }
        }
        for check in &sheet.checks {
            if entry
                .fields
                .get("splat_stats")
                .and_then(|v| v.get(&check.stat))
                .is_none()
            {
                return Err(format!("Check {} refers to missing stat", check.name));
            }
        }
    }
    if let Some(value) = entry.fields.get("splat_import") {
        let import: Import = serde_yaml_ng::from_value(value.clone()).map_err(|e| e.to_string())?;
        if import.version != 1
            || import
                .gaps
                .iter()
                .any(|g| g.path.is_empty() || g.reason.is_empty())
        {
            return Err("Invalid import coverage".into());
        }
    }
    Ok(())
}
