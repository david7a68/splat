//! Imported creature records. Provider totals are supplied stats, never recalculated.
//! Source records project into shared campaign content; simulation support is separate.
mod cache;
mod campaign;
pub mod campaign_package;
mod content;
mod presentation;
mod rules;
use crate::{
    content::Entry,
    execution::model::{Creature, Health, Pool, Stat},
};
pub use cache::Cache;
pub use campaign::CampaignCopyError;
pub use content::ContentNote;
pub use presentation::description_text;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

#[cfg(test)]
mod tests;

pub const VERSION: u32 = 1;
pub const ABILITIES: [&str; 6] = [
    "strength",
    "dexterity",
    "constitution",
    "intelligence",
    "wisdom",
    "charisma",
];

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreatureDefinition {
    pub splat_schema: u32,
    pub splat_id: Uuid,
    pub splat_kind: String,
    pub splat_name: String,
    pub splat_creature: Creature,
    pub splat_stats: BTreeMap<String, Stat>,
    pub splat_health: Health,
    pub splat_resources: BTreeMap<String, Pool>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DeathSaves {
    pub successes: u64,
    pub failures: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InitialState {
    pub hp: i64,
    pub temporary_hp: i64,
    pub death_saves: DeathSaves,
    pub conditions: Vec<Value>,
    pub resources: BTreeMap<String, i64>,
    pub inspiration: bool,
    pub hit_dice_spent: BTreeMap<String, u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryItem {
    pub id: Uuid,
    pub definition_id: Uuid,
    pub source_id: String,
    pub name: String,
    pub quantity: u64,
    pub equipped: bool,
    pub attuned_to: Option<Uuid>,
    pub holder_source_id: Option<String>,
    pub definition: Value,
    pub source: Value,
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
pub struct CharacterDetails {
    pub level: u64,
    pub species: String,
    pub classes: Vec<Value>,
    pub skills: Vec<Check>,
    pub attacks: Vec<Value>,
    pub casting: Value,
    pub senses: Vec<Value>,
    pub proficiencies: Vec<Value>,
    pub resistances: Vec<Value>,
    pub immunities: Vec<Value>,
    pub vulnerabilities: Vec<Value>,
    pub features: Vec<Value>,
    pub spells: Vec<Value>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub provider: String,
    pub campaign_id: String,
    pub character_id: String,
    pub fetched_at: u64,
    pub raw_fetched_at: u64,
    pub source_modified: Option<String>,
    pub content_hash: String,
    /// Complete source records preserve fields not yet understood by the shared model.
    /// Never authoritative for stats already supplied by the computed service.
    pub raw: Value,
    pub computed: Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntitySnapshot {
    pub version: u32,
    pub definition: CreatureDefinition,
    pub initial: InitialState,
    pub inventory: Vec<InventoryItem>,
    pub character: CharacterDetails,
    pub source: Source,
    pub coverage: Vec<String>,
}

impl EntitySnapshot {
    /// The portrait the player chose on D&D Beyond, if any.
    pub fn portrait_url(&self) -> Option<&str> {
        self.source.raw["decorations"]["avatarUrl"]
            .as_str()
            .filter(|url| !url.is_empty())
    }
}

pub fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
pub fn stable_id(value: &str) -> Uuid {
    let hash = Sha256::digest(value.as_bytes());
    let mut bytes = [0; 16];
    bytes.copy_from_slice(&hash[..16]);
    bytes[6] = (bytes[6] & 15) | 0x80;
    bytes[8] = (bytes[8] & 63) | 0x80;
    Uuid::from_bytes(bytes)
}
pub fn list(value: &Value) -> &[Value] {
    value.as_array().map(Vec::as_slice).unwrap_or(&[])
}
pub fn string(value: &Value) -> &str {
    value.as_str().unwrap_or("")
}
pub fn key(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}
fn number(value: &Value, label: &str) -> Result<i64, String> {
    value
        .as_i64()
        .ok_or_else(|| format!("D&D Beyond omitted {label}; keeping the previous local copy."))
}
fn id(value: &Value) -> Option<String> {
    value.as_u64().filter(|n| *n > 0).map(|n| n.to_string())
}

impl EntitySnapshot {
    pub fn from_beyond(
        raw: Value,
        computed: Value,
        campaign_id: &str,
        character_id: &str,
        raw_fetched_at: u64,
        fetched_at: u64,
    ) -> Result<Self, String> {
        if id(&raw["id"]).as_deref() != Some(character_id)
            || id(&computed["characterId"]).as_deref() != Some(character_id)
            || id(&raw["campaign"]["id"]).as_deref() != Some(campaign_id)
            || id(&computed["campaign"]["id"]).as_deref() != Some(campaign_id)
        {
            return Err(
                "Character identity or campaign membership changed. Refresh the party.".into(),
            );
        }
        let name = string(&computed["name"]);
        if name.is_empty() {
            return Err("D&D Beyond omitted the character name.".into());
        }
        // Different revisions can arrive from these two independent services. Do not hide it.
        let mut coverage = vec!["Computed totals, checks, attacks, defenses and casting values are supplied by D&D Beyond; their internal modifier contributions are not exposed.".into(),
            "Descriptions, choices, inventory and custom fields are retained from character v5. Campaign copies use the shared content schema. Simulation rule conversion is reported separately.".into()];
        if string(&raw["name"]) != name {
            coverage.push("The two D&D Beyond services returned different names. A later refresh may reconcile them.".into());
        }
        let mut stats = BTreeMap::new();
        let mut put = |key: String, value: i64, unit: Option<&str>| {
            stats.insert(
                key,
                Stat {
                    value,
                    unit: unit.map(str::to_owned),
                    includes: vec!["dndbeyond:supplied_total".into()],
                },
            );
        };
        for (key, field) in [
            ("armor_class", "armorClass"),
            ("proficiency_bonus", "proficiencyBonus"),
            ("initiative", "initiativeBonus"),
            ("passive_perception", "passivePerception"),
            ("passive_insight", "passiveInsight"),
            ("passive_investigation", "passiveInvestigation"),
        ] {
            put(key.into(), number(&computed[field], field)?, None);
        }
        put(
            "max_hp".into(),
            number(&computed["hitPointInfo"]["maximum"], "maximum HP")?,
            None,
        );
        for (index, ability) in ABILITIES.iter().enumerate() {
            let abbreviated = ["str", "dex", "con", "int", "wis", "cha"][index];
            let values = list(&computed["abilities"])
                .iter()
                .filter(|a| a["name"] == abbreviated)
                .collect::<Vec<_>>();
            if values.len() != 1 {
                return Err(format!("D&D Beyond omitted or duplicated {ability}."));
            }
            let a = values[0];
            for (suffix, field) in [("", "score"), ("_modifier", "modifier"), ("_save", "save")] {
                put(
                    format!("{ability}{suffix}"),
                    number(&a[field], ability)?,
                    None,
                );
            }
        }
        let mut skills = Vec::new();
        let mut skill_names = BTreeSet::new();
        for skill in list(&computed["skills"]) {
            let name = string(&skill["name"]);
            let stat = format!("skill_{}", key(name));
            if name.is_empty() || !skill_names.insert(stat.clone()) {
                return Err("Invalid computed skill list.".into());
            }
            put(stat.clone(), number(&skill["modifier"], name)?, None);
            skills.push(Check {
                name: name.into(),
                stat,
                proficient: skill["isProficient"] == true,
                expertise: skill["isExpert"] == true,
                half_proficient: skill["isHalfProficient"] == true,
                advantage: skill["isAdvantage"] == true,
                disadvantage: skill["isDisadvantage"] == true,
            });
        }
        if skills.len() < 18 {
            return Err("D&D Beyond returned an incomplete skill list.".into());
        }
        for speed in list(&computed["speeds"]) {
            put(
                format!("speed_{}", key(string(&speed["name"]))),
                number(&speed["distance"], "movement speed")?,
                Some("ft"),
            );
        }
        let size = raw["modifiers"]["race"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|m| m["type"] == "size")
            .and_then(|m| m["subType"].as_str())
            .map(str::to_owned)
            .unwrap_or_else(|| {
                match raw["race"]["sizeId"].as_u64() {
                    Some(2) => "tiny",
                    Some(3) => "small",
                    Some(4) => "medium",
                    Some(5) => "large",
                    Some(6) => "huge",
                    Some(7) => "gargantuan",
                    _ => "unknown",
                }
                .into()
            });
        let creature_type = match raw["race"]["creatureTypeId"].as_u64() {
            Some(11) => "humanoid",
            Some(8) => "fey",
            Some(4) => "construct",
            Some(16) => "undead",
            _ => "unknown",
        };
        if size == "unknown" || creature_type == "unknown" {
            coverage.push(
                "Creature size or type is not mapped; the original choice remains in source data."
                    .into(),
            );
        }
        let entity_id = stable_id(&format!("dndbeyond:character:{character_id}"));
        let mut inventory = Vec::new();
        for item in list(&raw["inventory"]) {
            let source_id = id(&item["id"]).ok_or("An inventory item has no stable ID.")?;
            inventory.push(InventoryItem {
                id: stable_id(&format!(
                    "dndbeyond:character:{character_id}:item:{source_id}"
                )),
                definition_id: stable_id(&format!(
                    "dndbeyond:item:{}:{}",
                    item["definition"]["entityTypeId"], item["definition"]["id"]
                )),
                source_id,
                name: string(&item["definition"]["name"]).into(),
                quantity: item["quantity"]
                    .as_u64()
                    .ok_or("An item has no quantity.")?,
                equipped: item["equipped"] == true,
                attuned_to: (item["isAttuned"] == true).then_some(entity_id),
                holder_source_id: id(&item["containerEntityId"]),
                definition: item["definition"].clone(),
                source: item.clone(),
            });
        }
        let mut pools = BTreeMap::new();
        let mut resource_state = BTreeMap::new();
        let classes = list(&raw["classes"]);
        if classes.len() == 1 {
            let class = &classes[0];
            let level = class["level"].as_u64().unwrap_or(0) as usize;
            let pact = class["definition"]["name"] == "Warlock";
            let source = if pact { "pactMagic" } else { "spellSlots" };
            for (i, count) in list(&class["definition"]["spellRules"]["levelSpellSlots"][level])
                .iter()
                .enumerate()
            {
                if let Some(max) = count.as_i64().filter(|n| *n > 0) {
                    let used = list(&raw[source])
                        .iter()
                        .find(|v| v["level"] == (i + 1) as u64)
                        .and_then(|v| v["used"].as_i64())
                        .ok_or("D&D Beyond omitted slot usage.")?;
                    let key = format!(
                        "{}_{}",
                        if pact { "pact_slots" } else { "spell_slots" },
                        i + 1
                    );
                    pools.insert(
                        key.clone(),
                        Pool {
                            name: format!(
                                "Level {} {}",
                                i + 1,
                                if pact { "pact slots" } else { "spell slots" }
                            ),
                            max,
                            initial: None,
                            kind: Some("spell_slot".into()),
                            level: Some((i + 1) as i64),
                        },
                    );
                    resource_state.insert(key, (max - used).max(0));
                }
            }
        } else {
            coverage.push(
                "Multiclass resource normalization is pending; source slot records are preserved."
                    .into(),
            );
        }
        for action in raw["actions"]
            .as_object()
            .into_iter()
            .flat_map(|v| v.values())
            .flat_map(list)
        {
            let limited = &action["limitedUse"];
            if !limited.is_object() {
                continue;
            }
            let mut max = limited["maxUses"].as_i64().filter(|n| *n >= 0).unwrap_or(0);
            if let Some(id) = limited["statModifierUsesId"]
                .as_u64()
                .filter(|id| (1..=6).contains(id))
            {
                let modifier = stats
                    .get(&format!("{}_modifier", ABILITIES[id as usize - 1]))
                    .ok_or("Missing resource ability.")?
                    .value;
                if limited["operator"] == 2 {
                    max *= modifier;
                } else {
                    max += modifier;
                }
            }
            if limited["useProficiencyBonus"] == true {
                let bonus = stats["proficiency_bonus"].value;
                if limited["proficiencyBonusOperator"] == 2 {
                    max *= bonus;
                } else {
                    max += bonus;
                }
            }
            if max <= 0 {
                coverage.push(format!(
                    "Resource '{}' has no resolved maximum; source usage is preserved.",
                    string(&action["name"])
                ));
                continue;
            }
            let used = limited["numberUsed"]
                .as_i64()
                .ok_or("Missing resource usage.")?;
            let key = format!(
                "feature_{}",
                id(&action["id"]).unwrap_or_else(|| key(string(&action["name"])))
            );
            if pools.contains_key(&key) {
                coverage.push(format!(
                    "Repeated resource '{}' needs shared-pool mapping.",
                    string(&action["name"])
                ));
                continue;
            }
            pools.insert(
                key.clone(),
                Pool {
                    name: string(&action["name"]).into(),
                    max,
                    initial: None,
                    kind: Some("feature".into()),
                    level: None,
                },
            );
            resource_state.insert(key, (max - used).max(0));
        }
        let mut features = Vec::new();
        for trait_ in list(&raw["race"]["racialTraits"]) {
            features.push(trait_.clone());
        }
        for class in classes {
            for feature in list(&class["classFeatures"]) {
                if feature["definition"]["requiredLevel"].as_u64().unwrap_or(0)
                    <= class["level"].as_u64().unwrap_or(0)
                {
                    features.push(feature.clone());
                }
            }
        }
        features.extend(list(&raw["feats"]).iter().cloned());
        let mut spells = raw["spells"]
            .as_object()
            .into_iter()
            .flat_map(|v| v.values())
            .flat_map(list)
            .cloned()
            .collect::<Vec<_>>();
        for class in list(&raw["classSpells"]) {
            spells.extend(list(&class["spells"]).iter().cloned());
        }
        let death = DeathSaves {
            successes: computed["deathSaveInfo"]["successCount"]
                .as_u64()
                .ok_or("Missing death saves.")?,
            failures: computed["deathSaveInfo"]["failCount"]
                .as_u64()
                .ok_or("Missing death saves.")?,
        };
        if death.successes > 3 || death.failures > 3 {
            return Err("Invalid death saves.".into());
        }
        let hash = format!(
            "{:x}",
            Sha256::digest(
                serde_json::to_vec(&(&raw, &computed))
                    .map_err(|_| "Could not fingerprint source data.")?
            )
        );
        let record = Self {
            version: VERSION,
            definition: CreatureDefinition {
                splat_schema: 1,
                splat_id: entity_id,
                splat_kind: "creature".into(),
                splat_name: name.into(),
                splat_creature: Creature {
                    size,
                    creature_type: creature_type.into(),
                    tags: vec!["player_character".into()],
                },
                splat_stats: stats,
                splat_health: Health {
                    zero_hp: "dnd2024:character".into(),
                },
                splat_resources: pools,
            },
            initial: InitialState {
                hp: number(&computed["hitPointInfo"]["current"], "current HP")?,
                temporary_hp: number(&computed["hitPointInfo"]["temp"], "temporary HP")?,
                death_saves: death,
                conditions: list(&computed["conditions"]).to_vec(),
                resources: resource_state,
                inspiration: computed["inspiration"] == true,
                hit_dice_spent: classes
                    .iter()
                    .filter_map(|c| Some((id(&c["id"])?, c["hitDiceUsed"].as_u64().unwrap_or(0))))
                    .collect(),
            },
            inventory,
            character: CharacterDetails {
                level: computed["level"]
                    .as_u64()
                    .ok_or("Missing character level.")?,
                species: string(&computed["race"]["name"]).into(),
                classes: list(&computed["classes"]).to_vec(),
                skills,
                attacks: list(&computed["attacks"]).to_vec(),
                casting: computed["castingInfo"].clone(),
                senses: list(&computed["senses"]).to_vec(),
                proficiencies: list(&computed["proficiencyGroups"]).to_vec(),
                resistances: list(&computed["resistances"]).to_vec(),
                immunities: list(&computed["immunities"]).to_vec(),
                vulnerabilities: list(&computed["vulnerabilities"]).to_vec(),
                features,
                spells,
            },
            source: Source {
                provider: "dndbeyond".into(),
                campaign_id: campaign_id.into(),
                character_id: character_id.into(),
                fetched_at,
                raw_fetched_at,
                source_modified: raw["dateModified"].as_str().map(str::to_owned),
                content_hash: hash,
                raw,
                computed,
            },
            coverage,
        };
        record.validate()?;
        Ok(record)
    }
    pub fn stat(&self, key: &str) -> Option<i64> {
        self.definition.splat_stats.get(key).map(|s| s.value)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.version != VERSION
            || self.definition.splat_schema != 1
            || self.definition.splat_kind != "creature"
            || self.source.provider != "dndbeyond"
        {
            return Err("Unsupported imported creature version.".into());
        }
        if self.initial.hp < 0
            || self.initial.temporary_hp < 0
            || self
                .stat("max_hp")
                .is_none_or(|hp| hp < 0 || self.initial.hp > hp)
        {
            return Err("Invalid character health state.".into());
        }
        if self.definition.splat_id
            != stable_id(&format!("dndbeyond:character:{}", self.source.character_id))
            || id(&self.source.raw["id"]).as_deref() != Some(&self.source.character_id)
            || id(&self.source.computed["characterId"]).as_deref()
                != Some(&self.source.character_id)
            || id(&self.source.raw["campaign"]["id"]).as_deref() != Some(&self.source.campaign_id)
            || id(&self.source.computed["campaign"]["id"]).as_deref()
                != Some(&self.source.campaign_id)
        {
            return Err("Cached character identity is inconsistent.".into());
        }
        let hash = format!(
            "{:x}",
            Sha256::digest(
                serde_json::to_vec(&(&self.source.raw, &self.source.computed))
                    .map_err(|_| "Could not fingerprint cache.")?
            )
        );
        if hash != self.source.content_hash {
            return Err("Cached source fingerprint is invalid.".into());
        }
        self.entry()?;
        Ok(())
    }
    /// The same Entry reader used for authored creature definitions accepts this definition.
    /// Source inventory remains instance state, not a second authored inventory table.
    pub fn entry(&self) -> Result<Entry, String> {
        Entry::parse("imported-character.md", &self.entry_markdown()?)
    }
    pub fn entry_markdown(&self) -> Result<String, String> {
        self.content_notes()?
            .pop()
            .map(|n| n.markdown)
            .ok_or_else(|| "No creature content generated".into())
    }
}
