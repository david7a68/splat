//! Convert provider snapshots into ordinary campaign content, independently of execution.
use super::*;
use crate::content::character::{
    Attack, CastingTotal, Check, Class, EntryRef, Feature, Gap, Import, KnownSpell, Sense, Sheet,
};

#[derive(Clone)]
pub struct ContentNote {
    pub relative_path: String,
    pub markdown: String,
    /// The D&D Beyond definitions it holds, as `feature:ID`, `action:ID`, `item:KEY` or `spell:ID`.
    pub sources: Vec<String>,
}
fn prose(value: &Value) -> String {
    super::description_text(string(value))
}
fn activation(value: &Value) -> Option<String> {
    match value["activationType"].as_u64()? {
        1 => Some("action"),
        3 => Some("bonus_action"),
        4 => Some("reaction"),
        6 => Some("minute"),
        7 => Some("hour"),
        8 => Some("special"),
        _ => None,
    }
    .map(str::to_owned)
}
fn duration(value: &Value) -> String {
    match value["durationInterval"].as_u64() {
        Some(amount) => format!("{amount} {}", string(&value["durationUnit"]).to_lowercase()),
        None => string(&value["durationType"]).to_lowercase(),
    }
}
pub(super) fn reference_id(record: &EntitySnapshot, kind: &str, source: &str) -> Uuid {
    stable_id(&format!(
        "dndbeyond:character:{}:{kind}:{source}",
        record.source.character_id
    ))
}
/// The D&D Beyond ID of an item. An item with no key has none: it cannot be told from another.
pub(super) fn item_identity(definition: &Value) -> Option<String> {
    let key = string(&definition["definitionKey"]);
    (!key.is_empty()).then(|| format!("item:{key}"))
}
/// The D&D Beyond ID of a spell, with the fallbacks that give the spell its local identity.
pub(super) fn spell_identity(definition: &Value) -> String {
    let source = id(&definition["id"])
        .or_else(|| definition["definitionKey"].as_str().map(str::to_owned))
        .unwrap_or_else(|| string(&definition["name"]).to_owned());
    format!("spell:{source}")
}
fn note(fields: Value, body: &str) -> Result<String, String> {
    Ok(format!(
        "---\n{}---\n\n{body}\n",
        serde_yaml_ng::to_string(&fields).map_err(|e| e.to_string())?
    ))
}
fn gap(gaps: &mut Vec<Gap>, path: impl Into<String>, reason: impl Into<String>) {
    gaps.push(Gap {
        path: path.into(),
        reason: reason.into(),
    });
}
pub(super) fn named_key(prefix: &str, value: &Value) -> String {
    // A change to totals or remaining uses must not change identity.
    format!(
        "{prefix}_{}",
        id(&value["id"]).unwrap_or_else(|| stable_id(&format!(
            "{}:{}:{}",
            value["name"], value["entityTypeId"], value["componentId"]
        ))
        .simple()
        .to_string())
    )
}
fn import(gaps: Vec<Gap>) -> Value {
    serde_json::to_value(Import {
        version: 1,
        gaps,
        engine_gaps: Vec::new(),
        required_inputs: Vec::new(),
        rule_fingerprint: None,
    })
    .expect("serializable import")
}

impl EntitySnapshot {
    /// The final note is the creature; preceding notes are its campaign-owned dependencies.
    /// Used by both fresh reads and version-1 caches, so migration requires no network request.
    pub fn content_notes(&self) -> Result<Vec<ContentNote>, String> {
        let mut notes = Vec::new();
        let mut gaps = Vec::new();
        let mut fields = serde_json::to_value(&self.definition).map_err(|e| e.to_string())?;
        fields["splat_initial"] = serde_json::to_value(&self.initial).map_err(|e| e.to_string())?;
        fields["splat_initial"]["conditions"] = json!(self.initial.conditions.iter().map(|condition| {
            let name = condition.as_str().or_else(|| condition["name"].as_str());
            if name.is_none() { gap(&mut gaps,"splat_initial/conditions","A source condition has no name; condition rule mapping requires conversion."); }
            json!({"name":name.unwrap_or("Unresolved source condition"),"source_id":id(&condition["id"]),"level":condition["level"].as_u64()})
        }).collect::<Vec<_>>());
        fields["splat_external"] = json!({"provider":"dndbeyond","character_id":self.source.character_id,"campaign_id":self.source.campaign_id,"fetched_at":self.source.fetched_at,"raw_fetched_at":self.source.raw_fetched_at,"source_hash":self.source.content_hash,"read_only":true});
        let mut sheet = Sheet {
            level: self.character.level,
            species: self.character.species.clone(),
            classes: self
                .character
                .classes
                .iter()
                .map(|c| Class {
                    name: string(&c["name"]).into(),
                    level: c["level"].as_u64().unwrap_or(0),
                    subclass: c["subclassName"].as_str().map(str::to_owned),
                })
                .collect(),
            checks: self
                .character
                .skills
                .iter()
                .map(|s| Check {
                    name: s.name.clone(),
                    stat: s.stat.clone(),
                    proficient: s.proficient,
                    expertise: s.expertise,
                    half_proficient: s.half_proficient,
                    advantage: s.advantage,
                    disadvantage: s.disadvantage,
                })
                .collect(),
            senses: self
                .character
                .senses
                .iter()
                .map(|s| Sense {
                    name: string(&s["name"]).into(),
                    distance: string(&s["distance"]).into(),
                })
                .collect(),
            ..Sheet::default()
        };
        for (name, values) in [
            ("resistance", &self.character.resistances),
            ("immunity", &self.character.immunities),
            ("vulnerability", &self.character.vulnerabilities),
        ] {
            sheet.defenses.insert(
                name.into(),
                values
                    .iter()
                    .map(|v| {
                        v.as_str()
                            .or_else(|| v["name"].as_str())
                            .unwrap_or("Unknown defense")
                            .to_owned()
                    })
                    .collect(),
            );
        }
        for (name, source) in [
            ("ability_modifier", "modifiers"),
            ("save_dc", "saveDcs"),
            ("attack_bonus", "spellAttacks"),
        ] {
            sheet.casting.insert(
                name.into(),
                list(&self.character.casting[source])
                    .iter()
                    .filter_map(|v| {
                        Some(CastingTotal {
                            value: v["value"].as_i64()?,
                            sources: list(&v["sources"])
                                .iter()
                                .filter_map(Value::as_str)
                                .map(str::to_owned)
                                .collect(),
                        })
                    })
                    .collect(),
            );
        }
        let mut proficiencies = json!({});
        for group in &self.character.proficiencies {
            let values = string(&group["values"])
                .split(',')
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .collect::<Vec<_>>();
            if group["group"] == "Languages" {
                fields["splat_languages"] = json!(values);
            } else {
                proficiencies[key(string(&group["group"]))] = json!(values);
            }
        }
        proficiencies["skills"] = json!(
            self.character
                .skills
                .iter()
                .filter(|s| s.proficient || s.expertise || s.half_proficient)
                .map(|s| (
                    key(&s.name),
                    if s.expertise {
                        "expertise"
                    } else if s.half_proficient {
                        "half_proficient"
                    } else {
                        "proficient"
                    }
                ))
                .collect::<BTreeMap<_, _>>()
        );
        fields["splat_proficiencies"] = proficiencies;
        let dice = list(&self.source.raw["classes"]).iter().filter_map(|c| Some((format!("class_{}",id(&c["id"])?),json!({"count":c["level"],"die":format!("d{}",c["definition"]["hitDice"].as_u64()?)})))).collect::<BTreeMap<_,_>>();
        for class in list(&self.source.raw["classes"]) {
            if class["definition"]["hitDice"].as_u64().is_none() {
                gap(
                    &mut gaps,
                    "splat_hit_dice",
                    "A class hit-die size was not supplied; spent hit dice remain recorded.",
                );
            }
        }
        fields["splat_hit_dice"] = json!({"classes":dice});
        fields["splat_initial"]["hit_dice_spent"] = json!(
            self.initial
                .hit_dice_spent
                .iter()
                .map(|(k, v)| (format!("class_{k}"), v))
                .collect::<BTreeMap<_, _>>()
        );
        // Keep source descriptions without pretending that prose is a rule implementation.
        for feature in &self.character.features {
            let d = feature.get("definition").unwrap_or(feature);
            let name = string(&d["name"]);
            if name.is_empty() {
                continue;
            }
            let k = format!(
                "feature_{}",
                stable_id(&format!(
                    "{}:{}",
                    d["entityTypeId"],
                    id(&d["id"]).unwrap_or_else(|| name.into())
                ))
                .simple()
            );
            gap(
                &mut gaps,
                format!("splat_character/features/{k}"),
                format!("{name}: descriptive feature; rule conversion is pending."),
            );
            sheet.features.insert(
                k,
                Feature {
                    name: name.into(),
                    description: prose(&d["description"]),
                    activation: activation(&d["activation"]),
                    resource: None,
                    source_key: string(&d["definitionKey"]).into(),
                },
            );
        }
        for (category, actions) in self.source.raw["actions"]
            .as_object()
            .into_iter()
            .flat_map(|o| o.iter())
        {
            for action in list(actions) {
                let name = string(&action["name"]);
                if name.is_empty() {
                    continue;
                }
                let k = named_key(&format!("action_{}", key(category)), action);
                let pool = format!("feature_{}", id(&action["id"]).unwrap_or_else(|| key(name)));
                gap(
                    &mut gaps,
                    format!("splat_character/features/{k}"),
                    format!(
                        "{name}: activation and description imported; resolution rules are pending."
                    ),
                );
                sheet.features.insert(
                    k,
                    Feature {
                        name: name.into(),
                        description: prose(
                            action
                                .get("description")
                                .filter(|v| !v.is_null())
                                .unwrap_or(&action["snippet"]),
                        ),
                        activation: activation(&action["activation"]),
                        resource: self
                            .definition
                            .splat_resources
                            .contains_key(&pool)
                            .then_some(pool.clone()),
                        source_key: id(&action["id"]).unwrap_or_default(),
                    },
                );
                if self.definition.splat_resources.contains_key(&pool) {
                    add_recovery(
                        &mut fields["splat_resources"][&pool],
                        &action["limitedUse"],
                        &mut gaps,
                        &pool,
                    );
                }
            }
        }
        for (pool, definition) in &self.definition.splat_resources {
            if definition.kind.as_deref() == Some("spell_slot") {
                fields["splat_resources"][pool]["recovery"] =
                    json!([{"key":"long_rest","mechanic":"dnd2024:long_rest","restore":"all"}]);
                if pool.starts_with("pact_") {
                    fields["splat_resources"][pool]["recovery"].as_array_mut().unwrap().push(json!({"key":"short_rest","mechanic":"dnd2024:short_rest","restore":"all"}));
                }
            }
        }
        let mut actions = json!({});
        for attack in &self.character.attacks {
            let base = named_key("attack", attack);
            let mut k = base.clone();
            let mut occurrence = 1;
            while sheet.attacks.contains_key(&k) {
                occurrence += 1;
                k = format!("{base}_{occurrence}");
            }
            let damage = &attack["damage"]["value"];
            sheet.attacks.insert(
                k.clone(),
                Attack {
                    name: string(&attack["name"]).into(),
                    attack_bonus: attack["toHit"].as_i64(),
                    damage_type: string(&attack["damage"]["type"]).to_lowercase(),
                    damage_dice: damage["diceString"].as_str().map(str::to_owned),
                    damage_modifier: damage["fixedValue"].as_i64(),
                    damage_fixed: damage.as_i64(),
                    damage_multiplier: damage["diceMultiplier"].as_i64(),
                    range_feet: attack["range"]["value"].as_u64(),
                    long_range_feet: attack["range"]["longRangeValue"].as_u64(),
                    range_origin: attack["range"]["origin"]
                        .as_str()
                        .filter(|v| !v.is_empty())
                        .unwrap_or(string(&attack["range"]["label"]))
                        .into(),
                    requires_save_conversion: !attack["saveInfo"].is_null(),
                },
            );
            let weapon = attack["name"] == "Unarmed Strike"
                || self.inventory.iter().any(|i| {
                    i.name == string(&attack["name"]) && i.definition["filterType"] == "Weapon"
                });
            // A spell's attack total alone does not give its casting costs or effects.
            if let Some(action) = weapon.then(|| attack_action(attack)).flatten() {
                actions[&k] = action;
                gap(
                    &mut gaps,
                    format!("splat_actions/{k}"),
                    "Supplied weapon attack imported; equipment requirements, attack substitutions, and feature interactions still require rule conversion.",
                );
            } else {
                gap(
                    &mut gaps,
                    format!("splat_character/attacks/{k}"),
                    format!(
                        "{}: supplied attack profile imported; complete action/casting rules require conversion.",
                        string(&attack["name"])
                    ),
                );
            }
        }
        fields["splat_actions"] = actions;
        let folder = format!("Definitions/{}", self.definition.splat_id);
        let mut definitions = BTreeMap::new();
        let mut definition_sources = BTreeMap::new();
        let mut inventory = BTreeMap::<Option<String>, Vec<String>>::new();
        let mut details = json!({});
        let mut source_ids = BTreeSet::new();
        for item in &self.inventory {
            if !source_ids.insert(&item.source_id) {
                return Err("Duplicate imported inventory identity".into());
            }
        }
        for item in &self.inventory {
            if item.quantity == 0 {
                gap(
                    &mut gaps,
                    format!("inventory/item_{}", item.source_id),
                    "Zero-quantity source item retained in private cache; no starting placement.",
                );
                continue;
            }
            let d = &item.definition;
            let shared = super::rules::shared_reference(
                &format!("item:{}", string(&d["definitionKey"])),
                d,
            )?;
            let def_id = shared
                .unwrap_or_else(|| reference_id(self, "item", &item.definition_id.to_string()));
            let row = format!("item_{}", item.source_id);
            let parent = item
                .holder_source_id
                .as_ref()
                .filter(|s| *s != &self.source.character_id);
            if parent.is_some_and(|s| !source_ids.contains(s)) {
                return Err(format!("{} has an unknown source container", item.name));
            }
            let parent = parent.map(|p| format!("item_{p}"));
            // Beyond's equipped flag does not distinguish held, worn or currently wielded.
            let equipment = if parent.is_some() {
                "stored"
            } else if item.equipped && d["filterType"] == "Armor" && d["armorTypeId"] != 4 {
                "worn"
            } else {
                "carried"
            };
            if item.equipped && equipment != "worn" {
                gap(
                    &mut gaps,
                    format!("splat_inventory_details/{row}"),
                    "Source marks this item equipped; hand/wielded state is unknown.",
                );
            }
            inventory.entry(parent).or_default().push(format!(
                "| {row} | `{def_id}` | {} | {equipment} | {} | {} |",
                item.quantity,
                item.attuned_to.is_some(),
                item.name.replace('|', " ").replace(['\n', '\r'], " ")
            ));
            details[&row] = json!({"instance_id":item.id,"source_id":item.source_id,"source_equipped":item.equipped,"charges_used":item.source["chargesUsed"].as_u64()});
            if shared.is_some() {
                continue;
            }
            let mut item_fields = json!({"splat_schema":1,"splat_id":def_id,"splat_kind":"item","splat_name":item.name,"splat_item":{"category":if d["isConsumable"]==true {"consumable".into()}else{key(string(&d["filterType"]))},"stackable":d["stackable"]==true,"equipment":{"allowed":["carried","stored","worn","wielded"],"attunement":if d["canAttune"]==true {"required"}else{"none"}}}});
            if let Some(weight) = d["weight"].as_f64() {
                item_fields["splat_item"]["mass"] = json!({"value":weight,"unit":"lb"});
            }
            if d["isContainer"] == true
                || self
                    .inventory
                    .iter()
                    .filter(|i| i.definition_id == item.definition_id)
                    .any(|container| {
                        self.inventory
                            .iter()
                            .any(|i| i.holder_source_id.as_ref() == Some(&container.source_id))
                    })
            {
                item_fields["splat_item"]["containment"] = json!({"capacity_weight_lb":d["capacityWeight"].as_f64().filter(|n| *n > 0.0),"description":string(&d["capacity"])});
            }
            if let Some(ac) = d["armorClass"].as_i64() {
                item_fields["splat_item"]["armor"] = json!({"value":ac,"contribution":if d["armorTypeId"]==4 {"bonus"}else{"base"},"strength_requirement":d["strengthRequirement"].as_u64()});
            }
            if d["damage"].is_object() || d["fixedDamage"].is_number() {
                item_fields["splat_item"]["weapon"] = json!({"damage_dice":d["damage"]["diceString"].as_str(),"damage_modifier":d["damage"]["fixedValue"].as_i64(),"damage_fixed":d["fixedDamage"].as_i64(),"damage_type":string(&d["damageType"]).to_lowercase(),"range_feet":d["range"].as_u64(),"long_range_feet":d["longRange"].as_u64()});
            }
            item_fields["splat_item"]["properties"]=json!(list(&d["properties"]).iter().map(|p| json!({"name":string(&p["name"]),"description":prose(&p["description"])})).collect::<Vec<_>>());
            item_fields["splat_import"]=import(vec![Gap{path:"splat_item".into(),reason:"Item description and equipment facts imported; granted mechanics and charge recovery require conversion.".into()}]);
            let markdown = note(
                item_fields,
                &format!("# {}\n\n{}", item.name, prose(&d["description"])),
            )?;
            if let Some(previous) = definitions.insert(def_id, markdown.clone())
                && previous != markdown
            {
                return Err(format!("Conflicting source definitions for {}", item.name));
            }
            if let Some(identity) = item_identity(d) {
                definition_sources.insert(def_id, identity);
            }
        }
        fields["splat_inventory_details"] = details;
        for spell in &self.character.spells {
            let d = &spell["definition"];
            let name = string(&d["name"]);
            if name.is_empty() {
                gap(
                    &mut gaps,
                    "splat_character/spells",
                    "Unnamed spell retained in private cache.",
                );
                continue;
            }
            let source = id(&d["id"])
                .or_else(|| d["definitionKey"].as_str().map(str::to_owned))
                .unwrap_or_else(|| name.to_owned());
            let shared = super::rules::shared_reference(&format!("spell:{source}"), d)?;
            let def_id = shared.unwrap_or_else(|| reference_id(self, "spell", &source));
            let k = named_key("spell", spell);
            let known = KnownSpell {
                entry: EntryRef { id: def_id },
                name: name.into(),
                prepared: spell["prepared"] == true,
                always_prepared: spell["alwaysPrepared"] == true,
                uses_spell_slot: spell["usesSpellSlot"] == true,
                ability: spell["spellCastingAbilityId"]
                    .as_u64()
                    .and_then(|i| i.checked_sub(1))
                    .and_then(|i| ABILITIES.get(i as usize))
                    .map(|s| s.to_string()),
                save_dc: spell["overrideSaveDc"].as_i64(),
                cast_at_level: spell["castAtLevel"].as_u64(),
                restriction: string(&spell["restriction"]).into(),
                casting: None,
            };
            // Same spell can be granted by several sources with distinct casting permissions.
            let k = if sheet.spells.contains_key(&k) {
                format!("{k}_{}", stable_id(&spell.to_string()).simple())
            } else {
                k
            };
            sheet.spells.insert(k, known);
            if shared.is_some() {
                continue;
            }
            let components = list(&d["components"])
                .iter()
                .map(|v| match v.as_u64() {
                    Some(1) => "verbal",
                    Some(2) => "somatic",
                    Some(3) => "material",
                    _ => "unknown",
                })
                .collect::<Vec<_>>();
            let spell_fields = json!({"splat_schema":1,"splat_id":def_id,"splat_kind":"spell","splat_name":name,"splat_spell":{"level":d["level"],"school":key(string(&d["school"])),"components":components,"material":string(&d["componentsDescription"]),"concentration":d["concentration"]==true,"ritual":d["ritual"]==true,"duration":duration(&d["duration"]),"casting_time":{"amount":d["activation"]["activationTime"].as_u64(),"unit":activation(&d["activation"])},"range":{"feet":d["range"]["rangeValue"],"origin":string(&d["range"]["origin"])}},"splat_import":import(vec![Gap{path:"splat_spell".into(),reason:format!("{name}: casting facts and description imported; spell resolution requires conversion.")}])});
            let markdown = note(
                spell_fields,
                &format!("# {name}\n\n{}", prose(&d["description"])),
            )?;
            if let Some(previous) = definitions.insert(def_id, markdown.clone())
                && previous != markdown
            {
                return Err(format!("Conflicting spell definitions for {name}"));
            }
            definition_sources.insert(def_id, spell_identity(d));
        }
        for (uuid, markdown) in definitions {
            notes.push(ContentNote {
                relative_path: format!("{folder}/{uuid}.md"),
                markdown,
                sources: definition_sources.remove(&uuid).into_iter().collect(),
            });
        }
        fields["splat_character"] = serde_json::to_value(&sheet).map_err(|e| e.to_string())?;
        gap(
            &mut gaps,
            "splat_stats",
            "Supplied totals do not expose their contributions; rebuilding totals after equipment or effect changes requires conversion.",
        );
        for message in &self.coverage {
            if !message.starts_with("Computed totals,")
                && !message.starts_with("Descriptions, choices,")
            {
                gap(&mut gaps, "source", message);
            }
        }
        fields["splat_import"] = import(gaps);
        let mut body = format!(
            "# {}\n\nLevel {} {} · {}. HP: {}; temporary HP: {}.\n\n[Open D&D Beyond sheet](https://www.dndbeyond.com/characters/{})\n\n",
            self.definition.splat_name,
            sheet.level,
            sheet.species,
            sheet
                .classes
                .iter()
                .map(|c| c.name.as_str())
                .collect::<Vec<_>>()
                .join(" / "),
            self.initial.hp,
            self.initial.temporary_hp,
            self.source.character_id
        );
        body.push_str(&format!(
            "AC: {} · Initiative: {:+} · Maximum HP: {} · Inspiration: {}.\n\n",
            self.stat("armor_class").unwrap_or(0),
            self.stat("initiative").unwrap_or(0),
            self.stat("max_hp").unwrap_or(0),
            if self.initial.inspiration {
                "Yes"
            } else {
                "No"
            }
        ));
        body.push_str("| Ability | Score | Modifier | Save |\n| --- | --- | --- | --- |\n");
        for ability in ABILITIES {
            body.push_str(&format!(
                "| {} | {} | {:+} | {:+} |\n",
                ability[..1].to_uppercase() + &ability[1..],
                self.stat(ability).unwrap_or(0),
                self.stat(&format!("{ability}_modifier")).unwrap_or(0),
                self.stat(&format!("{ability}_save")).unwrap_or(0)
            ));
        }
        body.push_str("\n## Skills\n\n| Skill | Total | Proficiency | Roll flags |\n| --- | --- | --- | --- |\n");
        for check in &sheet.checks {
            body.push_str(&format!(
                "| {} | {:+} | {} | {}{} |\n",
                check.name,
                self.stat(&check.stat).unwrap_or(0),
                if check.expertise {
                    "expertise"
                } else if check.proficient {
                    "proficient"
                } else if check.half_proficient {
                    "half proficient"
                } else {
                    "—"
                },
                if check.advantage { "advantage " } else { "" },
                if check.disadvantage {
                    "disadvantage"
                } else {
                    ""
                }
            ));
        }
        body.push_str(
            "\n## Resources\n\n| Resource | Remaining | Maximum |\n| --- | --- | --- |\n",
        );
        for (key, pool) in &self.definition.splat_resources {
            body.push_str(&format!(
                "| {} | {} | {} |\n",
                pool.name,
                self.initial.resources.get(key).unwrap_or(&pool.max),
                pool.max
            ));
        }
        body.push_str("\n## Training, movement, and senses\n\n");
        for group in &self.character.proficiencies {
            let values = string(&group["values"]);
            if !values.is_empty() {
                body.push_str(&format!("- {}: {values}\n", string(&group["group"])));
            }
        }
        for (key, stat) in &self.definition.splat_stats {
            if key.starts_with("speed_") || key.starts_with("passive_") {
                body.push_str(&format!(
                    "- {}: {} {}\n",
                    key.replace('_', " "),
                    stat.value,
                    stat.unit.as_deref().unwrap_or("")
                ));
            }
        }
        for sense in &sheet.senses {
            body.push_str(&format!("- {}: {}\n", sense.name, sense.distance));
        }
        for (kind, names) in &sheet.defenses {
            if !names.is_empty() {
                body.push_str(&format!("- {kind}: {}\n", names.join(", ")));
            }
        }
        for (kind, totals) in &sheet.casting {
            for total in totals {
                body.push_str(&format!(
                    "- {} ({}): {}\n",
                    kind.replace('_', " "),
                    total.sources.join(", "),
                    total.value
                ));
            }
        }
        body.push_str("\n## Attacks\n\n");
        for attack in sheet.attacks.values() {
            body.push_str(&format!(
                "- **{}**: attack {}; damage {} {}; range {}.\n",
                attack.name,
                attack
                    .attack_bonus
                    .map(|n| format!("{n:+}"))
                    .unwrap_or_else(|| "not supplied".into()),
                attack
                    .damage_fixed
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| format!(
                        "{}{:+}",
                        attack.damage_dice.as_deref().unwrap_or("unknown"),
                        attack.damage_modifier.unwrap_or(0)
                    )),
                attack.damage_type,
                if attack.range_origin == "Touch" {
                    "touch".into()
                } else {
                    attack
                        .range_feet
                        .map(|n| format!("{n} ft"))
                        .unwrap_or_else(|| "not supplied".into())
                }
            ));
        }
        body.push_str("## Spells\n\n");
        for spell in sheet.spells.values() {
            let link = super::rules::shared_source_link(spell.entry.id)?
                .unwrap_or_else(|| format!("{folder}/{}.md", spell.entry.id));
            body.push_str(&format!(
                "- [{}]({link}){}\n",
                spell.name,
                if spell.always_prepared {
                    " (always prepared)"
                } else if spell.prepared {
                    " (prepared)"
                } else {
                    ""
                }
            ));
        }
        body.push_str("\n## Equipment\n\n");
        for (container, rows) in inventory {
            body.push_str(&match container {
                Some(c) => format!("<!-- splat:inventory container={c} -->\n\n"),
                None => "<!-- splat:inventory -->\n\n".into(),
            });
            body.push_str("| Key | Item | Quantity | Equipment | Attuned | Link |\n| --- | --- | --- | --- | --- | --- |\n");
            for row in rows {
                body.push_str(&row);
                body.push('\n');
            }
            body.push('\n');
        }
        body.push_str("\n## Features\n\n");
        for feature in sheet.features.values() {
            body.push_str(&format!(
                "### {}\n\n{}\n\n",
                feature.name, feature.description
            ));
        }
        body.push_str("## Import gaps\n\n");
        let coverage: Import =
            serde_json::from_value(fields["splat_import"].clone()).map_err(|e| e.to_string())?;
        for g in coverage.gaps {
            body.push_str(&format!("- {}\n", g.reason));
        }
        notes.push(ContentNote {
            sources: Vec::new(),
            relative_path: format!("{}.md", self.definition.splat_id),
            markdown: note(fields, &body)?,
        });
        super::rules::complete(self, &mut notes)?;
        let entries = notes
            .iter()
            .map(|n| Entry::parse(&n.relative_path, &n.markdown))
            .collect::<Result<Vec<_>, _>>()?;
        let report = crate::content::validate_with_builtin(&entries)?;
        if !report.errors.is_empty() {
            return Err(format!("Invalid imported content: {:?}", report.errors));
        }
        Ok(notes)
    }
}
fn add_recovery(pool: &mut Value, limited: &Value, gaps: &mut Vec<Gap>, key: &str) {
    let mechanics = match limited["resetType"].as_u64() {
        Some(1) => vec!["dnd2024:short_rest", "dnd2024:long_rest"],
        Some(2) => vec!["dnd2024:long_rest"],
        _ => Vec::new(),
    };
    if mechanics.is_empty() {
        gap(
            gaps,
            format!("splat_resources/{key}/recovery"),
            "Recovery rule is not mapped.",
        );
    } else {
        pool["recovery"] = json!(
            mechanics
                .iter()
                .enumerate()
                .map(|(i, m)| json!({"key":format!("rest_{i}"),"mechanic":m,"restore":"all"}))
                .collect::<Vec<_>>()
        );
    }
}
fn attack_action(a: &Value) -> Option<Value> {
    let bonus = a["toHit"].as_i64()?;
    let range = a["range"]["value"].as_u64()?;
    if !a["saveInfo"].is_null() {
        return None;
    }
    let damage = &a["damage"]["value"];
    let kind = string(&a["damage"]["type"]).to_lowercase();
    if kind.is_empty() {
        return None;
    }
    let mut rolls = json!({"attack":{"expression":"1d20","modifier":bonus,"scope":"attack"}});
    let amount = if let Some(n) = damage.as_i64() {
        json!(n)
    } else {
        if damage["diceMultiplier"].as_i64().is_some_and(|n| n != 1) {
            return None;
        }
        let dice = damage["diceString"].as_str()?;
        let modifier = damage["fixedValue"].as_i64().unwrap_or(0);
        let expression = format!("{dice}{modifier:+}");
        // Parsing is a syntax check only; it does not establish rule coverage.
        if crate::execution::dice::Dice::parse(&expression).is_err() {
            return None;
        }
        rolls["damage"] = json!({"expression":expression,"scope":"attack"});
        json!({"roll":"damage"})
    };
    let mut range_field = json!({"feet":range});
    if let Some(long) = a["range"]["longRangeValue"].as_u64() {
        range_field["long"] = json!(long);
    }
    Some(
        json!({"name":string(&a["name"]),"activation":{"economy":"action","category":"attack","timing":"on_turn","cost_at":"commit"},"targets":{"chosen":{"kind":"creature","count":{"min":1,"max":1},"range":range_field}},"rolls":rolls,"steps":[{"key":"attack","operation":"dnd2024:attack","target":"chosen","kind":if a["range"]["label"]=="Reach"{"melee_weapon"}else{"ranged_weapon"},"roll":"attack","on_hit":{"damage":[{"type":kind,"amount":amount}]}}]}),
    )
}
