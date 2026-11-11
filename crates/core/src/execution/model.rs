//! Typed executable subset of the note model. Unknown fields are errors, never no-ops.
use super::dice::Dice;
use crate::content::Entry;
use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;
use std::collections::BTreeMap;

pub type Map<T> = BTreeMap<String, T>;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stat {
    pub value: i64,
    pub unit: Option<String>,
    #[serde(default)]
    pub includes: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Creature {
    pub size: String,
    #[serde(rename = "type")]
    pub creature_type: String,
    #[serde(default)]
    pub tags: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Health {
    pub zero_hp: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trait {
    pub rule: String,
    pub parameters: Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pool {
    pub name: String,
    pub max: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Parameter {
    #[serde(rename = "type")]
    pub kind: String,
    pub minimum: Option<i64>,
    pub maximum: Option<i64>,
    #[serde(default)]
    pub required: bool,
    pub default: Option<Value>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spell {
    pub level: i64,
    pub school: String,
    pub components: Vec<String>,
    pub concentration: bool,
    pub duration: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub category: String,
    pub mass: Quantity,
    pub stackable: bool,
    pub equipment: Equipment,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Equipment {
    pub allowed: Vec<String>,
    pub attunement: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub name: String,
    pub activation: Activation,
    #[serde(default)]
    pub requirements: Vec<Requirement>,
    pub targets: Map<Target>,
    #[serde(default)]
    pub costs: Vec<Cost>,
    #[serde(default)]
    pub rolls: Map<RollDef>,
    pub steps: Vec<Step>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Activation {
    pub economy: String,
    pub category: String,
    pub timing: String,
    pub cost_at: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Requirement {
    pub predicate: String,
    pub subject: Option<String>,
    pub minimum: Option<i64>,
    pub spell: Option<String>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
    pub kind: String,
    pub count: Count,
    #[serde(default)]
    pub allow_self: bool,
    pub range: Range,
    pub visibility: Option<String>,
    pub assumption: Option<String>,
    pub area: Option<Area>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Area {
    pub shape: String,
    pub radius_feet: i64,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Count {
    pub min: usize,
    pub max: usize,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Range {
    pub feet: Option<i64>,
    #[serde(default)]
    pub touch: bool,
    pub long: Option<i64>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cost {
    pub from: String,
    pub resource: Option<Value>,
    pub amount: Option<i64>,
    pub quantity: Option<i64>,
    pub at: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollDef {
    pub expression: Option<String>,
    pub dice: Option<DynamicDice>,
    pub modifier: Option<Value>,
    pub scope: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DynamicDice {
    pub count: Value,
    pub sides: u32,
    pub add: Value,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub key: String,
    pub operation: String,
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub roll: Option<String>,
    pub on_hit: Option<Hit>,
    pub ability: Option<String>,
    pub dc: Option<Value>,
    pub damage: Option<Vec<Damage>>,
    pub success: Option<Success>,
    pub amount: Option<Value>,
    pub count: Option<Value>,
    pub steps: Option<Vec<Step>>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hit {
    pub damage: Vec<Damage>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Damage {
    #[serde(rename = "type")]
    pub kind: String,
    pub amount: Value,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Success {
    pub damage_scale: Scale,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scale {
    pub numerator: i64,
    pub denominator: i64,
}

#[derive(Clone, Debug)]
pub struct Definition {
    pub entry: Entry,
    pub stats: Map<Stat>,
    pub creature: Option<Creature>,
    pub health: Option<Health>,
    pub traits: Map<Trait>,
    pub pools: Map<Pool>,
    pub spell: Option<Spell>,
    pub item: Option<Item>,
    pub parameters: Map<Parameter>,
    pub actions: Map<Action>,
}

fn read<T: serde::de::DeserializeOwned>(entry: &Entry, name: &str) -> Result<Option<T>, String> {
    entry
        .fields
        .get(Value::String(name.into()))
        .map(|v| {
            let value = if name == "splat_creature"
                && let Some(fields) = v.as_mapping()
            {
                Value::Mapping(crate::content::stat_blocks::creature_mechanics(fields))
            } else {
                v.clone()
            };
            serde_yaml_ng::from_value(value).map_err(|e| format!("{} {name}: {e}", entry.name))
        })
        .transpose()
}

pub fn compile(entry: &Entry) -> Result<Definition, String> {
    if let Some(meta) = crate::content::definitions::conversion(entry)? {
        return Err(format!(
            "Definition is not executable: conversion gaps {:?}; engine gaps {:?}",
            meta.conversion_gaps, meta.engine_gaps
        ));
    }
    if let Some(rule) = read::<serde_yaml_ng::Mapping>(entry, "splat_rule")?
        && let Some(implementation) = rule.get(Value::String("implementation".into()))
    {
        return Err(format!(
            "Definition is not executable: unavailable rule adapter {implementation:?}"
        ));
    }
    let allowed = [
        "splat_schema",
        "splat_id",
        "splat_key",
        "splat_kind",
        "splat_name",
        "splat_sources",
        "splat_origin",
        "splat_requires_mechanics",
        "splat_creature",
        "splat_stats",
        "splat_health",
        "splat_traits",
        "splat_resources",
        "splat_spell",
        "splat_item",
        "splat_parameters",
        "splat_actions",
    ];
    for key in entry.fields.keys().filter_map(Value::as_str) {
        if key.starts_with("splat_") && !allowed.contains(&key) {
            return Err(format!(
                "{}: unsupported executable field {key}",
                entry.name
            ));
        }
    }
    if !["creature", "spell", "item"].contains(&entry.kind.as_str()) {
        return Err(format!("{} is not an executable definition", entry.name));
    }
    let d = Definition {
        entry: entry.clone(),
        stats: read(entry, "splat_stats")?.unwrap_or_default(),
        creature: read(entry, "splat_creature")?,
        health: read(entry, "splat_health")?,
        traits: read(entry, "splat_traits")?.unwrap_or_default(),
        pools: read(entry, "splat_resources")?.unwrap_or_default(),
        spell: read(entry, "splat_spell")?,
        item: read(entry, "splat_item")?,
        parameters: read(entry, "splat_parameters")?.unwrap_or_default(),
        actions: read(entry, "splat_actions")?.unwrap_or_default(),
    };
    if !entry.inventory.is_empty() {
        return Err("Authored inventories are not instantiated by this proof executor".into());
    }
    if entry.kind == "creature" {
        if d.creature.is_none()
            || d.health
                .as_ref()
                .is_none_or(|h| h.zero_hp != "dnd2024:monster")
        {
            return Err("Proof creatures require explicit monster zero-HP policy".into());
        }
        for name in [
            "armor_class",
            "max_hp",
            "initiative",
            "strength",
            "dexterity",
            "constitution",
            "intelligence",
            "wisdom",
            "charisma",
        ] {
            if !d.stats.contains_key(name) {
                return Err(format!("Missing stat {name}"));
            }
        }
        if d.stats["max_hp"].value <= 0 {
            return Err("max_hp must be positive".into());
        }
    }
    if let Some(spell) = &d.spell
        && (!(0..=9).contains(&spell.level)
            || spell.concentration
            || spell.duration != "instantaneous"
            || spell
                .components
                .iter()
                .any(|c| !["verbal", "somatic"].contains(&c.as_str())))
    {
        return Err("Unsupported spell level, duration, Concentration or components".into());
    }
    if let Some(item) = &d.item
        && (item.category != "consumable" || item.equipment.attunement != "none")
    {
        return Err("Only unattuned consumables are executable in this proof".into());
    }
    for p in d.parameters.values() {
        if p.minimum.zip(p.maximum).is_some_and(|(min, max)| min > max) {
            return Err("Invalid parameter bounds".into());
        }
        if !["integer", "actor_resource"].contains(&p.kind.as_str()) {
            return Err("Unsupported parameter type".into());
        }
    }
    for pool in d.pools.values() {
        if pool.max < 0 || pool.initial.is_some_and(|v| v < 0 || v > pool.max) {
            return Err("Invalid pool bounds".into());
        }
    }
    for t in d.traits.values() {
        if t.rule == "dnd2024:condition_immunities" {
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct Immunities {
                conditions: Vec<String>,
            }
            let data: Immunities =
                serde_yaml_ng::from_value(t.parameters.clone()).map_err(|e| e.to_string())?;
            if data.conditions.iter().any(|s| {
                ![
                    "blinded",
                    "charmed",
                    "deafened",
                    "exhaustion",
                    "frightened",
                    "grappled",
                    "incapacitated",
                    "invisible",
                    "paralyzed",
                    "petrified",
                    "poisoned",
                    "prone",
                    "restrained",
                    "stunned",
                    "unconscious",
                ]
                .contains(&s.as_str())
            }) {
                return Err("Unknown condition immunity".into());
            }
            continue;
        }
        if t.rule != "dnd2024:damage_affinities" {
            return Err(format!("Unsupported trait {}", t.rule));
        }
        let affinities: Affinities =
            serde_yaml_ng::from_value(t.parameters.clone()).map_err(|e| e.to_string())?;
        for damage in affinities
            .immunities
            .iter()
            .chain(&affinities.resistances)
            .chain(&affinities.vulnerabilities)
        {
            damage_type(damage)?;
        }
    }
    if let Some(required) = read::<Vec<String>>(entry, "splat_requires_mechanics")? {
        for name in required {
            if ![
                "dnd2024:attack",
                "dnd2024:saving_throw",
                "dnd2024:damage",
                "dnd2024:heal",
                "dnd2024:spellcasting_instantaneous",
                "dnd2024:monster",
                "dnd2024:damage_affinities",
                "dnd2024:condition_immunities",
            ]
            .contains(&name.as_str())
            {
                return Err(format!("Unsupported required mechanic {name}"));
            }
        }
    }
    for (key, stat) in &d.stats {
        if stat.value.unsigned_abs() > 1_000_000_000 {
            return Err(format!("Stat {key} exceeds proof numeric limit"));
        }
    }
    if d.actions.is_empty() {
        return Err("No supported executable actions".into());
    }
    for (key, a) in &d.actions {
        validate_action(key, a, &d.parameters)?;
    }
    Ok(d)
}
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Affinities {
    #[serde(default)]
    pub immunities: Vec<String>,
    #[serde(default)]
    pub resistances: Vec<String>,
    #[serde(default)]
    pub vulnerabilities: Vec<String>,
}
fn damage_type(s: &str) -> Result<(), String> {
    if [
        "acid",
        "bludgeoning",
        "cold",
        "fire",
        "force",
        "lightning",
        "necrotic",
        "piercing",
        "poison",
        "psychic",
        "radiant",
        "slashing",
        "thunder",
    ]
    .contains(&s)
    {
        Ok(())
    } else {
        Err(format!("Unknown damage type {s}"))
    }
}

// Expressions are deliberately data-only. This subset needs integer arithmetic
// and bound parameters; wider expression/condition families fail compilation.
pub fn expression(
    value: &Value,
    parameters: &Map<Parameter>,
    allow_roll: bool,
    rolls: &Map<RollDef>,
) -> Result<(), String> {
    if value.as_i64().is_some() {
        return Ok(());
    }
    let m = value.as_mapping().ok_or("Expected integer expression")?;
    if m.len() == 1 {
        if let Some(name) = m.get("binding").and_then(Value::as_str) {
            return if parameters.get(name).is_some_and(|p| p.kind == "integer") {
                Ok(())
            } else {
                Err(format!("Unknown binding {name}"))
            };
        }
        if let Some(name) = m.get("roll").and_then(Value::as_str) {
            return if allow_roll && rolls.contains_key(name) {
                Ok(())
            } else {
                Err(format!("Unknown or disallowed roll {name}"))
            };
        }
    }
    if m.len() == 2
        && let Some(op) = m.get("op").and_then(Value::as_str)
    {
        let args = m
            .get("args")
            .and_then(Value::as_sequence)
            .ok_or("Expression needs args")?;
        if !["add", "subtract", "multiply"].contains(&op) || args.len() != 2 {
            return Err("Unsupported expression operator or arity".into());
        }
        for arg in args {
            expression(arg, parameters, allow_roll, rolls)?;
        }
        return Ok(());
    }
    Err("Unsupported expression shape".into())
}
fn validate_action(key: &str, a: &Action, parameters: &Map<Parameter>) -> Result<(), String> {
    if !["action", "bonus_action"].contains(&a.activation.economy.as_str())
        || a.activation.timing != "on_turn"
        || a.activation.cost_at != "commit"
        || !["attack", "magic"].contains(&a.activation.category.as_str())
    {
        return Err(format!("{key}: unsupported activation"));
    }
    for requirement in &a.requirements {
        match requirement.predicate.as_str() {
            "dnd2024:spell_casting"
                if requirement.spell.as_deref() == Some("self")
                    && requirement.subject.is_none()
                    && requirement.minimum.is_none() => {}
            "accessible"
                if requirement.subject.as_deref() == Some("source_item")
                    && requirement.minimum.is_none()
                    && requirement.spell.is_none() => {}
            "free_hands"
                if requirement.subject.as_deref() == Some("actor")
                    && requirement.minimum.is_some_and(|n| n > 0)
                    && requirement.spell.is_none() => {}
            _ => return Err(format!("Unsupported requirement {}", requirement.predicate)),
        }
    }
    for t in a.targets.values() {
        if t.assumption.is_some() != t.area.is_some()
            || t.area
                .as_ref()
                .is_some_and(|a| a.shape != "sphere" || a.radius_feet <= 0)
        {
            return Err("Unsupported area selector".into());
        }
        if t.kind != "creature"
            || t.count.min > t.count.max
            || t.count.max > 1000
            || t.range.feet.is_some() == t.range.touch
            || t.range.feet.is_some_and(|n| n < 0)
            || t.visibility.as_deref().is_some_and(|v| v != "seen")
            || t.range.long.is_some_and(|n| n < t.range.feet.unwrap_or(5))
        {
            return Err("Unsupported target selector".into());
        }
    }
    if a.costs.len() > 1 {
        return Err("Multiple costs require a wider cost contract than this proof supports".into());
    }
    for cost in &a.costs {
        if cost.from == "actor"
            && let Some(resource_value) = &cost.resource
            && cost.amount == Some(1)
            && cost.quantity.is_none()
            && cost.at == "dnd2024:spell_slot_expenditure"
        {
            let resource = resource_value
                .as_mapping()
                .ok_or("Resource needs a binding")?;
            let name = resource
                .get("binding")
                .and_then(Value::as_str)
                .ok_or("Resource needs a binding")?;
            if resource.len() != 1
                || parameters
                    .get(name)
                    .is_none_or(|p| p.kind != "actor_resource")
            {
                return Err("Invalid resource binding type".into());
            }
        } else if !(cost.from == "source_item"
            && cost.quantity == Some(1)
            && cost.resource.is_none()
            && cost.amount.is_none()
            && cost.at == "resolution")
        {
            return Err("Unsupported cost".into());
        }
    }
    for roll in a.rolls.values() {
        if !["use", "target", "attack"].contains(&roll.scope.as_str())
            || roll.expression.is_some() == roll.dice.is_some()
        {
            return Err("Invalid roll definition".into());
        }
        if let Some(text) = &roll.expression {
            Dice::parse(text)?;
        }
        if let Some(dice) = &roll.dice {
            if dice.sides == 0 || dice.sides > 1_000_000 {
                return Err("Invalid die size".into());
            }
            expression(&dice.count, parameters, false, &a.rolls)?;
            expression(&dice.add, parameters, false, &a.rolls)?;
        }
        if let Some(modifier) = &roll.modifier {
            expression(modifier, parameters, false, &a.rolls)?;
        }
    }
    let mut keys = std::collections::BTreeSet::new();
    fn check_steps(
        steps: &[Step],
        a: &Action,
        params: &Map<Parameter>,
        keys: &mut std::collections::BTreeSet<String>,
    ) -> Result<(), String> {
        for s in steps {
            if !keys.insert(s.key.clone()) {
                return Err(format!("Duplicate step {}", s.key));
            }
            if s.operation == "splat:repeat" {
                if s.target.is_some()
                    || s.kind.is_some()
                    || s.roll.is_some()
                    || s.on_hit.is_some()
                    || s.ability.is_some()
                    || s.dc.is_some()
                    || s.damage.is_some()
                    || s.success.is_some()
                    || s.amount.is_some()
                {
                    return Err("Unexpected repeat fields".into());
                }
                expression(
                    s.count.as_ref().ok_or("Repeat count missing")?,
                    params,
                    false,
                    &a.rolls,
                )?;
                check_steps(
                    s.steps.as_deref().ok_or("Repeat steps missing")?,
                    a,
                    params,
                    keys,
                )?;
                continue;
            }
            if s.count.is_some() || s.steps.is_some() {
                return Err("Unexpected nested steps".into());
            }
            if !s.target.as_ref().is_some_and(|t| a.targets.contains_key(t)) {
                return Err("Unknown target binding".into());
            }
            let damage = match s.operation.as_str() {
                "dnd2024:attack" => {
                    if !matches!(
                        s.kind.as_deref(),
                        Some("melee_weapon" | "ranged_weapon" | "ranged_spell")
                    ) || !s.roll.as_ref().is_some_and(|r| a.rolls.contains_key(r))
                        || s.ability.is_some()
                        || s.dc.is_some()
                        || s.damage.is_some()
                        || s.success.is_some()
                        || s.amount.is_some()
                    {
                        return Err("Invalid attack step".into());
                    }
                    let r = &a.rolls[s.roll.as_ref().unwrap()];
                    if r.expression.as_deref() != Some("1d20") || r.scope != "attack" {
                        return Err("Attack needs attack-scoped 1d20".into());
                    }
                    &s.on_hit.as_ref().ok_or("Missing on_hit")?.damage
                }
                "dnd2024:save_damage" => {
                    if !matches!(
                        s.ability.as_deref(),
                        Some(
                            "strength"
                                | "dexterity"
                                | "constitution"
                                | "intelligence"
                                | "wisdom"
                                | "charisma"
                        )
                    ) || s.kind.is_some()
                        || s.roll.is_some()
                        || s.on_hit.is_some()
                        || s.amount.is_some()
                    {
                        return Err("Invalid saving throw step".into());
                    }
                    expression(
                        s.dc.as_ref().ok_or("Missing save DC")?,
                        params,
                        false,
                        &a.rolls,
                    )?;
                    let scale = &s
                        .success
                        .as_ref()
                        .ok_or("Missing save success rule")?
                        .damage_scale;
                    if ![(0, 1), (1, 2)].contains(&(scale.numerator, scale.denominator)) {
                        return Err("Unsupported save damage scale".into());
                    }
                    s.damage.as_ref().ok_or("Missing save damage")?
                }
                "dnd2024:heal" => {
                    if s.kind.is_some()
                        || s.roll.is_some()
                        || s.on_hit.is_some()
                        || s.ability.is_some()
                        || s.dc.is_some()
                        || s.damage.is_some()
                        || s.success.is_some()
                    {
                        return Err("Unexpected healing fields".into());
                    }
                    expression(
                        s.amount.as_ref().ok_or("Missing healing amount")?,
                        params,
                        true,
                        &a.rolls,
                    )?;
                    continue;
                }
                _ => return Err(format!("Unsupported operation {}", s.operation)),
            };
            if damage.is_empty() {
                return Err("Empty damage components".into());
            }
            for d in damage {
                damage_type(&d.kind)?;
                expression(&d.amount, params, true, &a.rolls)?;
            }
        }
        Ok(())
    }
    // Bound recursion also gets a runtime operation limit.
    check_steps(&a.steps, a, parameters, &mut keys)
}
#[derive(Debug, Serialize)]
pub struct Capability {
    pub key: String,
    pub executable: bool,
    pub reason: Option<String>,
}
