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
    #[serde(default)]
    pub rules: BTreeMap<String, EntryRef>,
    #[serde(default)]
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub contributions: Vec<Contribution>,
    #[serde(default)]
    pub class_progression: BTreeMap<String, ClassProgression>,
    #[serde(default)]
    pub resource_bindings: BTreeMap<String, ResourceBinding>,
    #[serde(default)]
    pub campaign_options: BTreeMap<String, CampaignOptionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<Background>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub details: Details,
    #[serde(default, skip_serializing_if = "is_default")]
    pub characteristics: Characteristics,
    #[serde(default, skip_serializing_if = "is_default")]
    pub story: Story,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coins: Option<Coins>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub companions: BTreeMap<String, Companion>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub custom_items: BTreeMap<String, CustomItem>,
}
fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    *value == T::default()
}
/// The background as the sheet names it. Its proficiencies, choices, feats and spells are
/// imported with the rest of the sheet, so the description is the printed text only.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Background {
    pub name: String,
    /// The player wrote this background on the sheet instead of choosing a published one.
    pub custom: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_key: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// A key in `features`: the background's own feature, when it is not a feat.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    /// Feats the background grants. They are imported as features.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub feats: Vec<String>,
}
/// Facts about the character that no rule reads.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Details {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub player: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faith: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifestyle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eyes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hair: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experience: Option<u64>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Characteristics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personality_traits: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ideals: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bonds: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flaws: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appearance: Option<String>,
}
/// The player's notes on the sheet.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Story {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backstory: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allies: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enemies: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_possessions: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_holdings: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_notes: Option<String>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Coins {
    pub cp: u64,
    pub sp: u64,
    pub ep: u64,
    pub gp: u64,
    pub pp: u64,
}
/// A creature on the sheet's Extras: a familiar, mount, Wild Shape form and so on. Its stat
/// block is not converted, so it is not an encounter participant.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Companion {
    pub name: String,
    pub creature: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub active: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    /// Printed in the stat block, before anything the sheet adds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub armor_class: Option<i64>,
    /// Printed in the stat block, before anything the sheet adds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit_points: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub damage_taken: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporary_hp: Option<i64>,
    pub source_id: String,
}
/// An item the player wrote on the sheet. It has no item definition, so it has no
/// inventory placement.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CustomItem {
    pub name: String,
    pub quantity: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight_lb: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub description: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub notes: String,
    pub source_id: String,
}
/// Inactive means no selected grant, not that the campaign has disabled the system.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum CampaignOptionState {
    Inactive,
    Unselected,
    Active {
        selections: Vec<CampaignOptionSelection>,
    },
    Unknown,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CampaignOptionSelection {
    pub choice_id: String,
    pub option_id: String,
    pub name: String,
    pub rule: Option<EntryRef>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResourceBinding {
    pub rule: EntryRef,
    pub pool: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClassProgression {
    pub cantrips: Vec<u64>,
    pub spells_known: Vec<u64>,
    pub spells_prepared: Vec<u64>,
    pub spell_slots: Vec<Vec<u64>>,
    pub casting_ability: Option<String>,
    pub ritual_caster: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Choice {
    pub source_id: String,
    pub component_id: String,
    pub category: String,
    pub label: String,
    pub selected_option_id: Option<String>,
    pub selected: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Contribution {
    pub source_id: String,
    pub component_id: String,
    pub category: String,
    pub kind: String,
    pub stat: String,
    pub amount: Option<i64>,
    pub ability: Option<String>,
    pub restriction: String,
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
    #[serde(default)]
    pub casting: Option<CastingBinding>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CastingBinding {
    pub attack_bonus: Option<i64>,
    pub save_dc: Option<i64>,
    pub slot_pools: Vec<String>,
    pub at_will: bool,
    pub ritual_only: bool,
    pub free_cast_pool: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Import {
    pub version: u32,
    pub gaps: Vec<Gap>,
    #[serde(default)]
    pub engine_gaps: Vec<String>,
    #[serde(default)]
    pub required_inputs: Vec<Gap>,
    #[serde(default)]
    pub rule_fingerprint: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gap {
    pub path: String,
    pub reason: String,
}

/// Bind conversion coverage to the exact typed mechanics, independently of source prose.
pub fn rule_fingerprint(fields: &serde_yaml_ng::Mapping) -> Result<String, String> {
    let fields: BTreeMap<_, _> = [
        "splat_rule",
        "splat_actions",
        "splat_traits",
        "splat_effects",
        "splat_parameters",
        "splat_contracts",
        "splat_character",
        "splat_stats",
        "splat_resources",
        "splat_spell",
        "splat_item",
        "splat_attack_bindings",
        "splat_stat_policy",
        "splat_requires_mechanics",
    ]
    .into_iter()
    .filter_map(|key| fields.get(key).map(|value| (key, value)))
    .collect();
    super::contracts::fingerprint(&serde_json::to_value(fields).map_err(|e| e.to_string())?)
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
        for (system, state) in &sheet.campaign_options {
            if !["dark_bargain", "character_threads", "runestones"].contains(&system.as_str()) {
                return Err(format!("Unknown campaign option system: {system}"));
            }
            if let CampaignOptionState::Active { selections } = state {
                let mut ids = std::collections::BTreeSet::new();
                if selections.is_empty()
                    || selections.iter().any(|s| {
                        s.choice_id.is_empty()
                            || s.option_id.is_empty()
                            || s.name.is_empty()
                            || !ids.insert(&s.choice_id)
                    })
                {
                    return Err("Active campaign options require distinct, named selections".into());
                }
            }
        }
        if let Some(background) = &sheet.background {
            if background.name.is_empty() {
                return Err("A character background needs a name".into());
            }
            if let Some(key) = &background.feature
                && !sheet.features.contains_key(key)
            {
                return Err(format!("Background refers to missing feature {key}"));
            }
        }
        for (key, companion) in &sheet.companions {
            if !super::local_key(key) || companion.name.is_empty() {
                return Err("Invalid character companion".into());
            }
        }
        for (key, item) in &sheet.custom_items {
            if !super::local_key(key) || item.name.is_empty() {
                return Err("Invalid custom item".into());
            }
        }
        for (key, spell) in &sheet.spells {
            if !super::local_key(key) || spell.name.is_empty() {
                return Err("Invalid character spell".into());
            }
            if let Some(casting) = &spell.casting {
                for pool in casting
                    .slot_pools
                    .iter()
                    .chain(casting.free_cast_pool.iter())
                {
                    if entry
                        .fields
                        .get("splat_resources")
                        .and_then(|v| v.get(pool))
                        .is_none()
                    {
                        return Err(format!("Spell {key} refers to missing resource {pool}"));
                    }
                }
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
        for binding in sheet.resource_bindings.values() {
            if entry
                .fields
                .get("splat_resources")
                .and_then(|p| p.get(&binding.pool))
                .is_none()
            {
                return Err(format!("Rule refers to missing resource {}", binding.pool));
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
        if ![1, 2].contains(&import.version)
            || (import.version == 2 && import.rule_fingerprint.is_none())
            || import
                .gaps
                .iter()
                .any(|g| g.path.is_empty() || g.reason.is_empty())
        {
            return Err("Invalid import coverage".into());
        }
        if import.engine_gaps.iter().any(|k| !super::resource_key(k))
            || import
                .required_inputs
                .iter()
                .any(|g| g.path.is_empty() || g.reason.is_empty())
        {
            return Err("Invalid engine gap or required input".into());
        }
        if let Some(expected) = &import.rule_fingerprint
            && *expected != rule_fingerprint(&entry.fields)?
        {
            return Err("Imported mechanics changed without updating conversion coverage".into());
        }
    }
    Ok(())
}
