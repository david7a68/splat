//! Conflict-aware combat sheet synchronization with D&D Beyond.
//! Writes use source identities and absolute values, followed by a raw read-back.
use crate::auth::Session;
use eframe::egui;
use reqwest::{Method, blocking::Client, redirect::Policy};
use serde_json::{Value, json};
use splat::character::EntitySnapshot;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::{Duration, Instant},
};

const BASE: &str = "https://character-service.dndbeyond.com/character/v5";
const INTERVAL: Duration = Duration::from_secs(15);
pub const CONDITIONS: [(u64, &str); 15] = [
    (1, "Blinded"),
    (2, "Charmed"),
    (3, "Deafened"),
    (4, "Exhaustion"),
    (5, "Frightened"),
    (6, "Grappled"),
    (7, "Incapacitated"),
    (8, "Invisible"),
    (9, "Paralyzed"),
    (10, "Petrified"),
    (11, "Poisoned"),
    (12, "Prone"),
    (13, "Restrained"),
    (14, "Stunned"),
    (15, "Unconscious"),
];
/// The character's own purse, as the official sheet names it in a currency write.
const CHARACTER_ENTITY_TYPE: u64 = 1_581_111_423;
/// Each coin: its field on the sheet, its route, and its name.
pub const COINS: [(&str, &str, &str); 5] = [
    ("pp", "platinum", "Platinum"),
    ("gp", "gold", "Gold"),
    ("ep", "electrum", "Electrum"),
    ("sp", "silver", "Silver"),
    ("cp", "copper", "Copper"),
];
/// The largest maximum HP modifier Splat sends, either way.
const MAX_HP_BONUS: i64 = 999;
/// The largest count Splat sends for a coin, an item or a use.
const MAX_COUNT: i64 = 1_000_000;
type Groups = BTreeMap<String, Value>;
/// The values Splat would send, with a revision that every change bumps. Code outside the
/// panel's steppers writes through `write`; the panel compares what its steppers drew and calls
/// `touch` when one changed a value. The stepper wells (`Wells`) are kept per revision.
#[derive(Default)]
struct Sheet {
    groups: Groups,
    revision: u64,
}
impl std::ops::Deref for Sheet {
    type Target = Groups;
    fn deref(&self) -> &Groups {
        &self.groups
    }
}
impl Sheet {
    fn write(&mut self) -> &mut Groups {
        self.touch();
        &mut self.groups
    }
    fn touch(&mut self) {
        self.revision += 1;
    }
}
/// One class's hit dice, for a short rest.
#[derive(Clone, Debug, PartialEq)]
pub struct HitDice {
    /// The class's mapping ID on the sheet, which the short rest names.
    pub class: u64,
    pub name: String,
    pub die: i64,
    pub level: i64,
    pub used: i64,
}
/// A change that D&D Beyond applies itself, so sending it twice would apply it twice.
/// Splat sends one only when the DM asks, never again by itself, and then reads the sheet.
#[derive(Clone, Debug, PartialEq)]
pub enum SheetAction {
    /// Hit dice to spend now, by class mapping ID.
    ShortRest {
        hit_dice: BTreeMap<u64, i64>,
        reset_max_hp_bonus: bool,
    },
    LongRest {
        reset_max_hp_bonus: bool,
        reduce_exhaustion: bool,
    },
    /// Back from death, with full HP or with 1 HP.
    RestoreLife { full: bool },
}
impl SheetAction {
    fn name(&self) -> &'static str {
        match self {
            Self::ShortRest { .. } => "short rest",
            Self::LongRest { .. } => "long rest",
            Self::RestoreLife { .. } => "restore to life",
        }
    }
    fn mutation(&self, remote: &Remote) -> Result<Mutation, String> {
        let (method, route, mut body) = match self {
            Self::ShortRest {
                hit_dice,
                reset_max_hp_bonus,
            } => {
                let mut used = serde_json::Map::new();
                for dice in &remote.hit_dice {
                    let spend = hit_dice.get(&dice.class).copied().unwrap_or(0);
                    if spend < 0 || dice.used + spend > dice.level {
                        return Err(format!("{} has too few hit dice left.", dice.name));
                    }
                    used.insert(dice.class.to_string(), (dice.used + spend).into());
                }
                if hit_dice
                    .keys()
                    .any(|class| !remote.hit_dice.iter().any(|d| d.class == *class))
                {
                    return Err("A class is no longer on the sheet.".into());
                }
                (
                    Method::POST,
                    "/character/rest/short",
                    json!({"classHitDiceUsed":used,"resetMaxHpModifier":reset_max_hp_bonus}),
                )
            }
            Self::LongRest {
                reset_max_hp_bonus,
                reduce_exhaustion,
            } => (
                Method::POST,
                "/character/rest/long",
                json!({"resetMaxHpModifier":reset_max_hp_bonus,"adjustConditionLevel":reduce_exhaustion}),
            ),
            // The types are D&D Beyond's rule data: 1 is full HP, 2 is 1 HP.
            Self::RestoreLife { full } => {
                // The button showed at 0 HP, but the sheet may have changed since.
                let removed = remote
                    .groups
                    .get("hp")
                    .and_then(|hp| hp["removedHitPoints"].as_i64());
                if removed.is_none_or(|removed| removed < remote.max_hp) {
                    return Err("The character is no longer at 0 HP.".into());
                }
                (
                    Method::PUT,
                    "/life/restore",
                    json!({"totalHp":remote.max_hp,"restoreType":if *full { 1 } else { 2 }}),
                )
            }
        };
        body["characterId"] = remote.id.into();
        Ok(Mutation {
            method,
            route: route.into(),
            body,
            expected: Value::Null,
        })
    }
}
#[derive(Clone)]
struct Remote {
    id: u64,
    max_hp: i64,
    can_edit: bool,
    groups: Groups,
    labels: BTreeMap<String, String>,
    /// The most a value can be, where the sheet says: an item's charges, a companion's HP.
    limits: BTreeMap<String, i64>,
    hit_dice: Vec<HitDice>,
    record: Option<EntitySnapshot>,
}
fn integer(v: &Value, field: &str) -> Result<i64, String> {
    v.as_i64()
        .filter(|n| *n >= 0)
        .ok_or_else(|| format!("D&D Beyond returned invalid {field}."))
}
impl Remote {
    /// Each named group whose key starts with `prefix`: its key, its name, and its limit.
    fn named(&self, prefix: &str) -> impl Iterator<Item = (&String, &String, Option<i64>)> {
        self.labels
            .iter()
            .filter(move |(key, _)| key.starts_with(prefix))
            .map(|(key, name)| (key, name, self.limits.get(key).copied()))
    }
    fn from_raw(id: u64, max_hp: i64, raw: &Value) -> Result<Self, String> {
        if raw["id"].as_u64() != Some(id) || max_hp <= 0 {
            return Err("Character identity or maximum HP is invalid.".into());
        }
        let mut groups = Groups::new();
        let counter = |v: &Value, field: &str| {
            if v.is_null() {
                Ok(0)
            } else {
                integer(v, field)
            }
        };
        if let (Some(removed), Some(temp)) =
            (raw.get("removedHitPoints"), raw.get("temporaryHitPoints"))
        {
            groups.insert("hp".into(), json!({"removedHitPoints":counter(removed,"HP")?,"temporaryHitPoints":counter(temp,"temporary HP")?}));
        }
        if let Some(death) = raw.get("deathSaves") {
            let (success, fail) = if death.is_null() {
                (0, 0)
            } else {
                (
                    counter(
                        death
                            .get("successCount")
                            .ok_or("Missing death save successes.")?,
                        "death saves",
                    )?,
                    counter(
                        death
                            .get("failCount")
                            .ok_or("Missing death save failures.")?,
                        "death saves",
                    )?,
                )
            };
            if success > 3 || fail > 3 {
                return Err("Invalid death save counts.".into());
            }
            groups.insert(
                "death".into(),
                json!({"successCount":success,"failCount":fail}),
            );
        }
        if let Some(conditions) = raw["conditions"].as_array() {
            let mut values = serde_json::Map::new();
            for c in conditions {
                let key = c["id"]
                    .as_u64()
                    .ok_or("A condition has no source ID.")?
                    .to_string();
                if values
                    .insert(key, c.get("level").cloned().unwrap_or(Value::Null))
                    .is_some()
                {
                    return Err("Duplicate condition ID.".into());
                }
            }
            groups.insert("conditions".into(), values.into());
        }
        for (field, group, levels) in [("spellSlots", "slots", 9), ("pactMagic", "pact", 5)] {
            if let Some(slots) = raw[field].as_array() {
                let mut values = serde_json::Map::new();
                for level in 1..=levels {
                    values.insert(format!("level{level}"), 0.into());
                }
                let mut seen = BTreeSet::new();
                for slot in slots {
                    let level = slot["level"].as_u64().ok_or("Invalid spell slot level.")?;
                    if !(1..=levels).contains(&level) || !seen.insert(level) {
                        return Err("Invalid or duplicate slot level.".into());
                    }
                    values.insert(
                        format!("level{level}"),
                        integer(&slot["used"], "slots used")?.into(),
                    );
                }
                groups.insert(group.into(), values.into());
            }
        }
        let mut labels = BTreeMap::new();
        fn resources(
            value: &Value,
            groups: &mut Groups,
            labels: &mut BTreeMap<String, String>,
        ) -> Result<(), String> {
            match value {
                Value::Array(items) => {
                    for item in items {
                        resources(item, groups, labels)?;
                    }
                }
                Value::Object(map) => {
                    if let (Some(id), Some(kind), Some(used)) = (
                        map.get("id").and_then(Value::as_u64),
                        map.get("entityTypeId").and_then(Value::as_u64),
                        map.get("limitedUse").and_then(|v| v.get("numberUsed")),
                    ) {
                        let key = format!("limited/{kind}/{id}");
                        let used = integer(used, "limited uses")?;
                        let group = json!({"id":id,"entityTypeId":kind,"uses":used});
                        if groups.get(&key).is_some_and(|old| old != &group) {
                            return Err("Conflicting resource identities.".into());
                        }
                        groups.insert(key.clone(), group);
                        labels.insert(
                            key,
                            value["name"]
                                .as_str()
                                .or_else(|| value["definition"]["name"].as_str())
                                .unwrap_or("Limited-use resource")
                                .into(),
                        );
                    }
                    for (key, child) in map {
                        if key != "definition" && key != "limitedUse" {
                            resources(child, groups, labels)?;
                        }
                    }
                }
                _ => {}
            }
            Ok(())
        }
        for source in ["actions", "spells", "classSpells"] {
            resources(&raw[source], &mut groups, &mut labels)?;
        }
        if let Some(inspiration) = raw.get("inspiration") {
            let inspiration = match inspiration {
                Value::Null => false,
                Value::Bool(b) => *b,
                _ => return Err("D&D Beyond returned invalid inspiration.".into()),
            };
            groups.insert("inspiration".into(), json!({"inspiration":inspiration}));
        }
        if let Some(bonus) = raw.get("bonusHitPoints") {
            let bonus = if bonus.is_null() {
                0
            } else {
                bonus
                    .as_i64()
                    .ok_or("D&D Beyond returned an invalid maximum HP modifier.")?
            };
            groups.insert("hp_bonus".into(), json!({"bonusHitPoints":bonus}));
        }
        if let Some(purse) = raw.get("currencies").filter(|v| !v.is_null()) {
            let mut coins = serde_json::Map::new();
            for (field, _, name) in COINS {
                coins.insert(field.into(), counter(&purse[field], name)?.into());
            }
            groups.insert("currency".into(), coins.into());
        }
        let mut limits = BTreeMap::new();
        for item in raw["inventory"].as_array().into_iter().flatten() {
            let id = item["id"].as_u64().ok_or("An item has no source ID.")?;
            let key = format!("item/{id}");
            let mut group =
                json!({"id":id,"quantity":integer(&item["quantity"], "item quantity")?});
            if let Some(used) = item["limitedUse"].get("numberUsed") {
                group["charges"] = counter(used, "item charges")?.into();
                if let Some(max) = item["limitedUse"]["maxUses"].as_i64() {
                    limits.insert(key.clone(), max);
                }
            }
            if groups.insert(key.clone(), group).is_some() {
                return Err("Duplicate item ID.".into());
            }
            labels.insert(
                key,
                item["definition"]["name"].as_str().unwrap_or("Item").into(),
            );
        }
        for creature in raw["creatures"].as_array().into_iter().flatten() {
            let id = creature["id"]
                .as_u64()
                .ok_or("A companion has no source ID.")?;
            let key = format!("creature/{id}");
            let group = json!({
                "id": id,
                "removedHitPoints": counter(&creature["removedHitPoints"], "companion HP")?,
                "temporaryHitPoints": counter(&creature["temporaryHitPoints"], "companion temporary HP")?,
            });
            if groups.insert(key.clone(), group).is_some() {
                return Err("Duplicate companion ID.".into());
            }
            if let Some(max) = creature["definition"]["averageHitPoints"].as_i64() {
                limits.insert(key.clone(), max);
            }
            labels.insert(
                key,
                creature["name"]
                    .as_str()
                    .filter(|n| !n.trim().is_empty())
                    .or_else(|| creature["definition"]["name"].as_str())
                    .unwrap_or("Companion")
                    .into(),
            );
        }
        let mut hit_dice = Vec::new();
        for class in raw["classes"].as_array().into_iter().flatten() {
            let (Some(class_id), Some(level)) = (class["id"].as_u64(), class["level"].as_i64())
            else {
                continue;
            };
            hit_dice.push(HitDice {
                class: class_id,
                name: class["definition"]["name"]
                    .as_str()
                    .unwrap_or("Class")
                    .into(),
                die: class["definition"]["hitDice"].as_i64().unwrap_or(0),
                level,
                used: counter(&class["hitDiceUsed"], "hit dice used")?,
            });
        }
        Ok(Self {
            id,
            max_hp,
            can_edit: raw["canEdit"] == true,
            groups,
            labels,
            limits,
            hit_dice,
            record: None,
        })
    }
}
#[derive(Clone, Debug)]
struct Mutation {
    method: Method,
    route: String,
    body: Value,
    expected: Value,
}
fn mutations(remote: &Remote, key: &str, target: &Value) -> Result<Vec<Mutation>, String> {
    let current = remote
        .groups
        .get(key)
        .ok_or("This combat stat is unavailable on the sheet.")?;
    let mut body = target
        .as_object()
        .cloned()
        .ok_or("Invalid combat values.")?;
    let route = match key {
        "hp" => "/life/hp/damage-taken",
        "death" => "/life/death-saves",
        "slots" => "/spell/slots",
        "pact" => "/spell/pact-magic",
        key if key.starts_with("limited/") => "/action/limited-use",
        "conditions" => "/condition",
        "inspiration" => "/character/inspiration",
        "hp_bonus" => "/life/hp/bonus",
        "currency" => "/inventory/currency",
        key if key.starts_with("item/") => "/inventory/item",
        key if key.starts_with("creature/") => "/creature/hp",
        _ => return Err("Unsupported combat field.".into()),
    };
    // Inspiration and the maximum HP modifier each hold one value.
    let single = match key {
        "inspiration" => Some(("inspiration", "Inspiration must be on or off.")),
        "hp_bonus" => Some((
            "bonusHitPoints",
            "The maximum HP modifier must be a whole number from −999 to 999.",
        )),
        _ => None,
    };
    if let Some((field, error)) = single {
        let value = body
            .get(field)
            .filter(|v| match key {
                "inspiration" => v.is_boolean(),
                _ => v.as_i64().is_some_and(|n| n.abs() <= MAX_HP_BONUS),
            })
            .ok_or(error)?;
        if body.len() != 1 {
            return Err("Combat stat fields do not match the sheet.".into());
        }
        return Ok(vec![Mutation {
            method: Method::PUT,
            route: route.into(),
            body: json!({"characterId": remote.id, field: value}),
            expected: target.clone(),
        }]);
    }
    if key == "conditions" {
        let old = current.as_object().ok_or("Invalid conditions.")?;
        let mut expected = old.clone();
        let mut changes = Vec::new();
        for id in old.keys().filter(|id| !body.contains_key(*id)) {
            let parsed = id.parse::<u64>().map_err(|_| "Invalid condition ID.")?;
            if !CONDITIONS.iter().any(|(i, _)| *i == parsed) {
                return Err("Only recognized conditions can be edited.".into());
            }
            expected.remove(id);
            changes.push(Mutation {
                method: Method::DELETE,
                route: route.into(),
                body: json!({"characterId":remote.id,"id":parsed}),
                expected: expected.clone().into(),
            });
        }
        for (id, level) in &body {
            if old.get(id) == Some(level) {
                continue;
            }
            let parsed = id.parse::<u64>().map_err(|_| "Invalid condition ID.")?;
            if !CONDITIONS.iter().any(|(i, _)| *i == parsed)
                || (parsed == 4 && !level.as_u64().is_some_and(|n| (1..=6).contains(&n)))
                || (parsed != 4 && !level.is_null())
            {
                return Err("Invalid condition or Exhaustion level.".into());
            }
            expected.insert(id.clone(), level.clone());
            changes.push(Mutation {method:Method::PUT,route:route.into(),body:json!({"characterId":remote.id,"id":parsed,"level":level,"totalHp":remote.max_hp}),expected:expected.clone().into()});
        }
        return Ok(changes);
    }
    // Only accept the exact group shape supplied by the provider.
    let old = current.as_object().ok_or("Invalid baseline.")?;
    if body.keys().collect::<Vec<_>>() != old.keys().collect::<Vec<_>>() {
        return Err("Combat stat fields do not match the sheet.".into());
    }
    for (field, value) in &body {
        let n = integer(value, field)?;
        if key == "death" && n > 3 {
            return Err("Death save counts must be zero to three.".into());
        }
        if key == "hp"
            && field == "removedHitPoints"
            && n > remote.max_hp
            && old.get(field) != Some(value)
        {
            return Err("Current HP must be between zero and maximum HP.".into());
        }
        if ["id", "entityTypeId"].contains(&field.as_str()) && old.get(field) != Some(value) {
            return Err("Resource identity cannot change.".into());
        }
        // D&D Beyond may not delete an item at quantity 0, or may. Splat does not find out.
        if key.starts_with("item/") && field == "quantity" && n < 1 && old.get(field) != Some(value)
        {
            return Err("Quantity must be at least 1. Remove an item on D&D Beyond.".into());
        }
    }
    // Currency and items have one route for each field. Each write is read back on its own.
    if key == "currency" || key.starts_with("item/") {
        let mut expected = old.clone();
        let mut changes = Vec::new();
        for (field, value) in &body {
            if old.get(field) == Some(value) {
                continue;
            }
            expected.insert(field.clone(), value.clone());
            let (route, body) = if key == "currency" {
                let coin = COINS
                    .iter()
                    .find(|(f, ..)| f == field)
                    .ok_or("Unknown coin.")?
                    .1;
                (
                    format!("/inventory/currency/{coin}"),
                    json!({"characterId":remote.id,"amount":value,"destinationEntityId":remote.id,"destinationEntityTypeId":CHARACTER_ENTITY_TYPE}),
                )
            } else {
                let id = &old["id"];
                match field.as_str() {
                    "quantity" => (
                        "/inventory/item/quantity".into(),
                        json!({"characterId":remote.id,"id":id,"quantity":value}),
                    ),
                    "charges" => (
                        "/inventory/item/charge".into(),
                        json!({"characterId":remote.id,"id":id,"charges":value}),
                    ),
                    _ => return Err("Unsupported item field.".into()),
                }
            };
            changes.push(Mutation {
                method: Method::PUT,
                route,
                body,
                expected: expected.clone().into(),
            });
        }
        return Ok(changes);
    }
    body.insert("characterId".into(), remote.id.into());
    Ok(vec![Mutation {
        method: Method::PUT,
        route: route.into(),
        body: body.into(),
        expected: target.clone(),
    }])
}
trait Transport {
    fn read(&mut self) -> Result<Remote, String>;
    fn write(&mut self, mutation: &Mutation) -> Result<(), String>;
}
struct Live {
    session: Session,
    client: Client,
    id: String,
    campaign: String,
    active: Arc<AtomicBool>,
}
impl Transport for Live {
    fn read(&mut self) -> Result<Remote, String> {
        if !self.active.load(Ordering::SeqCst) {
            return Err("Sync stopped.".into());
        }
        let mut record =
            crate::party_sync::fetch(&self.client, &self.session, &self.id, &self.campaign)
                .map_err(|e| e.message)?;
        let mut remote = Remote::from_raw(
            self.id.parse().map_err(|_| "Invalid character ID.")?,
            record.stat("max_hp").ok_or("Missing maximum HP.")?,
            &record.source.raw,
        )?;
        if let Some(hp) = remote.groups.get("hp") {
            record.initial.hp =
                (remote.max_hp - hp["removedHitPoints"].as_i64().unwrap_or(0)).max(0);
            record.initial.temporary_hp = hp["temporaryHitPoints"].as_i64().unwrap_or(0);
        }
        if let Some(death) = remote.groups.get("death") {
            record.initial.death_saves.successes = death["successCount"].as_u64().unwrap_or(0);
            record.initial.death_saves.failures = death["failCount"].as_u64().unwrap_or(0);
        }
        remote.record = Some(record);
        Ok(remote)
    }
    fn write(&mut self, mutation: &Mutation) -> Result<(), String> {
        if !self.active.load(Ordering::SeqCst) {
            return Err("Sync stopped before writing.".into());
        }
        let response = self
            .session
            .authorize(
                self.client
                    .request(mutation.method.clone(), format!("{BASE}{}", mutation.route)),
            )
            .json(&mutation.body)
            .send()
            .map_err(|_| {
                "Write response was not received. Reading the sheet again before any retry."
                    .to_owned()
            })?;
        let status = response.status().as_u16();
        if !(200..300).contains(&status) {
            return Err(match status {
                401 | 403 => {
                    "D&D Beyond denied this write. Reconnect and check character edit permission."
                }
                400 => "D&D Beyond rejected the change; the sheet may have changed elsewhere.",
                429 => "D&D Beyond is limiting writes. Refresh later before retrying.",
                _ => "D&D Beyond could not confirm the write.",
            }
            .into());
        }
        let mut bytes = Vec::new();
        response
            .take(1024 * 1024)
            .read_to_end(&mut bytes)
            .map_err(|_| "Could not read the write response.")?;
        let response: Value =
            serde_json::from_slice(&bytes).map_err(|_| "Unexpected write response.")?;
        if response["success"] != true {
            return Err("D&D Beyond did not accept the change.".into());
        }
        Ok(())
    }
}
struct Outcome {
    remote: Remote,
    blocked: BTreeMap<String, String>,
}
fn exchange(
    transport: &mut impl Transport,
    baseline: Option<&Remote>,
    desired: &Groups,
    blocked: &BTreeSet<String>,
    read_only: bool,
) -> Result<Outcome, String> {
    let mut remote = transport.read()?;
    let mut problems = BTreeMap::new();
    if !read_only && let Some(baseline) = baseline {
        for (key, target) in desired {
            if baseline.groups.get(key) == Some(target)
                || remote.groups.get(key) == Some(target)
                || blocked.contains(key)
            {
                continue;
            }
            if remote.groups.get(key) != baseline.groups.get(key)
                || (key == "hp" && baseline.max_hp != remote.max_hp)
            {
                problems.insert(
                    key.clone(),
                    "Changed on D&D Beyond since your edit. Choose which values to keep.".into(),
                );
                continue;
            }
            // A sheet this session cannot edit: the run keeps its value, and nothing goes.
            if !remote.can_edit {
                continue;
            }
            let changes = match mutations(&remote, key, target) {
                Ok(v) => v,
                Err(e) => {
                    problems.insert(key.clone(), e);
                    continue;
                }
            };
            for change in changes {
                let sent = transport.write(&change);
                remote = transport.read()?;
                if remote.groups.get(key) != Some(&change.expected) {
                    problems.insert(key.clone(),sent.err().unwrap_or_else(||"The read-back did not confirm this change. Review the current sheet before retrying.".into()));
                    break;
                }
            }
        }
    }
    Ok(Outcome {
        remote,
        blocked: problems,
    })
}
/// Whether D&D Beyond took a sheet action. `None` when the reply was an ordinary exchange.
type ActionResult = Option<Result<(), String>>;
/// Send one sheet action between two reads. It is never sent again by itself: if its result
/// is uncertain, the DM checks the sheet first.
fn act(
    transport: &mut impl Transport,
    action: &SheetAction,
) -> Result<(Outcome, ActionResult), String> {
    let name = action.name();
    let remote = transport
        .read()
        .map_err(|e| format!("Splat did not send the {name}. {e}"))?;
    let outcome = |remote| Outcome {
        remote,
        blocked: BTreeMap::new(),
    };
    let mutation = if remote.can_edit {
        action.mutation(&remote)
    } else {
        Err("This session cannot edit this character on D&D Beyond.".into())
    };
    let mutation = match mutation {
        Ok(m) => m,
        Err(e) => {
            return Ok((
                outcome(remote),
                Some(Err(format!("Splat did not send the {name}. {e}"))),
            ));
        }
    };
    let sent = transport.write(&mutation);
    let remote = transport.read().map_err(|_| {
        format!("D&D Beyond may have taken the {name}, but Splat could not read the sheet again. Check the sheet before you try again.")
    })?;
    Ok((
        outcome(remote),
        Some(sent.map_err(|e| {
            format!("Splat could not confirm the {name}. {e} Check the sheet on D&D Beyond before you try again.")
        })),
    ))
}
struct Pending {
    receiver: mpsc::Receiver<Result<(Outcome, ActionResult), String>>,
    sent: Groups,
    /// The sheet action in this exchange, for its message.
    action: Option<&'static str>,
}
/// The sync states the brief names. Each has a word and an icon on the participant row.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum SyncState {
    Off,
    FirstRead,
    Matching,
    Pending,
    Conflict,
    ReadOnly,
    Error,
    /// The session ended under a running sync. It needs a connection and `Sync party`.
    Stopped,
    /// Sync is off, and this row has a change it has not sent.
    NotSent,
}

const SESSION_LOST: &str = "The D&D Beyond session ended, so this sync stopped. Changes since then are kept in Splat and have not been sent. Connect in Settings. Sync party then starts again, if it was on. Splat reads the sheet first, and asks before it replaces a value you changed.";
/// Groups the run holds as its own values, so a saved run keeps them.
const KEPT_BY_THE_RUN: [&str; 3] = ["hp", "death", "conditions"];
const DIFFER_AFTER_READ: &str =
    "Splat and D&D Beyond differ, and you changed this while sync was off. Choose which to keep.";

/// A sheet with this damage and nothing else recorded. Test builds cannot read a live one.
#[cfg(test)]
fn simulated_sheet(character: u64, max_hp: i64, damage: i64, can_edit: bool) -> Remote {
    Remote::from_raw(
        character,
        max_hp,
        &json!({"id":character,"canEdit":can_edit,"removedHitPoints":damage,"temporaryHitPoints":0,"deathSaves":{"successCount":0,"failCount":0},"conditions":[]}),
    )
    .expect("simulated sheet")
}

/// The choices for a rest, in the panel, before it goes.
#[derive(Clone, Debug, PartialEq)]
enum RestForm {
    Short {
        hit_dice: BTreeMap<u64, i64>,
        reset_max_hp_bonus: bool,
    },
    Long {
        reset_max_hp_bonus: bool,
        reduce_exhaustion: bool,
    },
}
impl RestForm {
    fn into_action(self) -> SheetAction {
        match self {
            Self::Short {
                hit_dice,
                reset_max_hp_bonus,
            } => SheetAction::ShortRest {
                hit_dice,
                reset_max_hp_bonus,
            },
            Self::Long {
                reset_max_hp_bonus,
                reduce_exhaustion,
            } => SheetAction::LongRest {
                reset_max_hp_bonus,
                reduce_exhaustion,
            },
        }
    }
}
/// What happened to the last sheet action, as the panel says it.
#[derive(Clone, Debug, PartialEq)]
enum ActionNotice {
    /// D&D Beyond took it. This message goes at the next read.
    Taken(String),
    /// It did not go, or Splat could not confirm it. This message stays until the DM closes it.
    Failed(String),
}

#[derive(Default)]
pub struct CombatSync {
    pub enabled: bool,
    scope: Option<(u64, String, String)>,
    remote: Option<Remote>,
    desired: Sheet,
    /// The stepper wells, and what they were measured from.
    wells: Option<(WellsKey, Wells)>,
    /// How often frames asked for the wells, and how often the values were read to measure
    /// them (`CachedWork` in tests).
    #[cfg(test)]
    pub(crate) well_counts: (usize, usize),
    blocked: BTreeMap<String, String>,
    pending: Option<Pending>,
    gate: Option<Arc<AtomicBool>>,
    next: Option<Instant>,
    changed: Option<Instant>,
    refresh_only: bool,
    pub error: Option<String>,
    pub updated: bool,
    /// A conflict choice made by the DM, consumed by the run as a named undo step.
    pub chose_remote: bool,
    session_lost: bool,
    /// The last sheet values read before this sync was started again. What the run changed
    /// since then is the DM's own, and the next first read must not replace it unasked.
    matched: Option<Groups>,
    /// Unsent values the run does not hold itself, such as spell slots. They last until the
    /// next first read, where a difference asks. They are not in a saved run.
    unsent: Groups,
    /// A first read arrived, and the run has not compared it with its values yet.
    first_read: bool,
    /// A sheet action the DM asked for. The next exchange takes it and sends it once.
    action: Option<SheetAction>,
    /// What happened to the last sheet action.
    action_notice: Option<ActionNotice>,
    /// A rest the DM is setting up in the panel, before they send it.
    rest_form: Option<RestForm>,
    /// Unsent HP, death saves and conditions also last until the next first read. A run holds
    /// those itself. The Party page does not, so without this they would be lost there.
    keeps_all_unsent: bool,
    /// Never talks to D&D Beyond. It only shows the state it was given.
    #[cfg(test)]
    simulated: bool,
}
impl Drop for CombatSync {
    fn drop(&mut self) {
        if let Some(gate) = &self.gate {
            gate.store(false, Ordering::SeqCst);
        }
    }
}
impl CombatSync {
    /// A sync in a state test builds cannot read from a live sheet. It only shows that.
    #[cfg(test)]
    pub fn simulated_state(state: SyncState, character: u64, max_hp: i64, damage: i64) -> Self {
        if state == SyncState::Conflict {
            return Self::simulated_conflict(character, max_hp, damage);
        }
        let mut sync = Self::default();
        sync.simulated = true;
        sync.enabled = state != SyncState::Off;
        if matches!(state, SyncState::FirstRead | SyncState::Off) {
            return sync;
        }
        let read = simulated_sheet(character, max_hp, damage, state != SyncState::ReadOnly);
        *sync.desired.write() = read.groups.clone();
        sync.remote = Some(read);
        if state == SyncState::Pending {
            sync.desired.write().get_mut("hp").expect("HP group")["removedHitPoints"] =
                (damage + 1).into();
        }
        if state == SyncState::Error {
            sync.error = Some("Could not read this character from D&D Beyond.".into());
        }
        sync
    }

    /// The state after a refresh finds that D&D Beyond and Splat changed the same HP.
    /// Test builds cannot read a live sheet, so a visual test shows the panel this way.
    #[cfg(test)]
    pub fn simulated_conflict(character: u64, max_hp: i64, damage: i64) -> Self {
        let read = simulated_sheet(character, max_hp, damage, true);
        let mut sync = Self::default();
        sync.enabled = true;
        sync.simulated = true;
        sync.remote = Some(read.clone());
        *sync.desired.write() = read.groups.clone();
        sync.desired.write().get_mut("hp").expect("HP group")["removedHitPoints"] =
            (damage + 4).into();
        let mut changed = read;
        changed.groups.get_mut("hp").expect("HP group")["removedHitPoints"] = (damage + 9).into();
        sync.accept_read(changed);
        sync
    }

    pub fn stop(&mut self) {
        if let Some(gate) = &self.gate {
            gate.store(false, Ordering::SeqCst);
        }
        // A rest on its way may have reached D&D Beyond. Splat will not hear back.
        if let Some(name) = self.pending.as_ref().and_then(|pending| pending.action) {
            self.action_notice = Some(ActionNotice::Failed(format!(
                "Sync stopped before Splat heard back about the {name}. Check the sheet on D&D Beyond before you try again."
            )));
        }
        self.pending = None;
        self.gate = None;
        self.enabled = false;
        // A rest that has not gone does not go later, when sync starts again.
        self.action = None;
        self.rest_form = None;
    }
    /// Why a sheet action cannot go now, or `None` when it can.
    pub fn action_blocker(&self) -> Option<&'static str> {
        if !self.ready() {
            Some("Wait for the first sheet read.")
        } else if self.read_only() {
            Some("This session cannot edit this character on D&D Beyond.")
        } else if self.busy() || self.action.is_some() {
            Some("Wait for the current sheet read.")
        } else if !self.blocked.is_empty() {
            Some("Settle the conflicts first.")
        } else if self.dirty() {
            Some("Wait until your changes reach D&D Beyond.")
        } else {
            None
        }
    }
    /// Send a sheet action with the next exchange, once. The DM asked for it.
    pub fn request(&mut self, action: SheetAction) -> Result<(), &'static str> {
        if let Some(reason) = self.action_blocker() {
            return Err(reason);
        }
        self.action = Some(action);
        self.action_notice = None;
        Ok(())
    }
    /// Each class's hit dice, from the last read.
    pub fn hit_dice(&self) -> &[HitDice] {
        self.remote.as_ref().map_or(&[], |r| &r.hit_dice)
    }
    /// The session ended. This sync stops, and says so until the DM starts it again.
    pub fn session_ended(&mut self) {
        #[cfg(test)]
        if self.simulated {
            return;
        }
        if self.enabled {
            self.stop();
            self.error = Some(SESSION_LOST.into());
            self.session_lost = true;
        }
    }
    pub fn session_lost(&self) -> bool {
        self.session_lost
    }
    /// Changes D&D Beyond has not confirmed: on their way, or stranded with sync off. A read
    /// with nothing to send does not count, or sync that is always on would hold a close.
    pub fn unconfirmed(&self) -> bool {
        !self.read_only() && ((self.enabled && (self.writing() || self.dirty())) || self.stranded())
    }
    /// This session cannot edit the sheet, so a value that differs is the run's own to keep.
    pub fn read_only(&self) -> bool {
        self.remote.as_ref().is_some_and(|remote| !remote.can_edit)
    }
    pub fn keep_all_unsent(&mut self) {
        self.keeps_all_unsent = true;
    }
    fn writing(&self) -> bool {
        match (&self.pending, &self.remote) {
            (Some(pending), Some(remote)) => pending.sent != remote.groups,
            _ => false,
        }
    }
    /// Sync is off, by the session or by the DM, and these changes have not gone.
    pub fn stranded(&self) -> bool {
        !self.enabled && self.dirty() && !self.read_only()
    }
    /// Off, with a read to compare the run's values with.
    pub fn stopped_after_read(&self) -> bool {
        !self.enabled && self.remote.is_some()
    }
    /// Sync is off, and values only this sync holds have not gone. A close loses them.
    pub fn unsent_only_here(&self) -> bool {
        !self.enabled && !self.only_here().is_empty()
    }
    fn only_here(&self) -> Groups {
        self.desired
            .iter()
            .filter(|(key, value)| {
                (self.keeps_all_unsent || !KEPT_BY_THE_RUN.contains(&key.as_str()))
                    && self.remote.as_ref().and_then(|r| r.groups.get(*key)) != Some(*value)
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }
    fn keep_unsent(&mut self) {
        self.unsent.extend(self.only_here());
    }
    /// What D&D Beyond last confirmed.
    pub fn confirmed(&self) -> Option<&Groups> {
        self.remote.as_ref().map(|remote| &remote.groups)
    }
    /// Let this sync react to the session, as a real one does.
    #[cfg(test)]
    pub fn stop_simulating(&mut self) {
        self.simulated = false;
    }
    pub fn ready(&self) -> bool {
        self.enabled && self.remote.is_some()
    }
    pub fn state(&self) -> SyncState {
        if !self.blocked.is_empty() {
            SyncState::Conflict
        } else if self.stranded() {
            SyncState::NotSent
        } else if self.session_lost {
            SyncState::Stopped
        } else if self.error.is_some() {
            SyncState::Error
        } else if !self.enabled {
            SyncState::Off
        } else {
            match &self.remote {
                None => SyncState::FirstRead,
                Some(remote) if !remote.can_edit => SyncState::ReadOnly,
                Some(_) if self.busy() || self.dirty() => SyncState::Pending,
                Some(_) => SyncState::Matching,
            }
        }
    }
    /// Test builds have no session, and `tick` stops sync without one. Keep it on.
    #[cfg(test)]
    pub fn keep_without_connection(&mut self) {
        self.simulated = true;
    }
    #[cfg(not(test))]
    pub fn keep_without_connection(&mut self) {}
    /// Turn sync on. It reads the sheet before it writes anything.
    pub fn start(&mut self) {
        self.enabled = true;
        self.keep_unsent();
        if let Some(remote) = self.remote.take() {
            self.matched = Some(remote.groups);
        }
        self.desired.write().clear();
        self.blocked.clear();
        self.next = None;
        self.error = None;
        self.session_lost = false;
    }
    pub fn busy(&self) -> bool {
        self.pending.is_some()
    }
    pub fn dirty(&self) -> bool {
        self.remote
            .as_ref()
            .is_some_and(|r| r.groups != *self.desired)
    }
    pub fn record(&self) -> Option<&EntitySnapshot> {
        if self.enabled {
            self.remote.as_ref()?.record.as_ref()
        } else {
            None
        }
    }
    pub fn max_hp(&self) -> Option<i64> {
        self.remote.as_ref().map(|r| r.max_hp)
    }
    pub fn values(&self) -> &Groups {
        &self.desired
    }
    pub fn set_group(&mut self, key: &str, value: Value) {
        if self.ready() && self.desired.contains_key(key) && self.desired.get(key) != Some(&value) {
            self.desired.write().insert(key.into(), value);
            self.changed = Some(Instant::now());
        }
    }
    pub fn tick(
        &mut self,
        ctx: &egui::Context,
        session: Option<Session>,
        epoch: u64,
        campaign: &str,
        id: &str,
    ) {
        // Test builds have no route to D&D Beyond. A signed-in test stays on its first read.
        if cfg!(test) {
            let _ = (ctx, &session, epoch, campaign, id);
            return;
        }
        // Its words and unsent changes stay until the DM starts it again.
        if self.session_lost {
            return;
        }
        let scope = (epoch, campaign.to_owned(), id.to_owned());
        if self.scope.as_ref() != Some(&scope) {
            self.enter_scope(scope);
        }
        if !self.enabled || session.is_none() {
            if self.pending.is_some() || session.is_none() {
                self.stop();
            }
            return;
        }
        if let Some(pending) = &self.pending {
            let result = match pending.receiver.try_recv() {
                Ok(r) => Some(r),
                Err(mpsc::TryRecvError::Disconnected) => {
                    Some(Err("Combat sync stopped unexpectedly.".into()))
                }
                Err(mpsc::TryRecvError::Empty) => None,
            };
            if let Some(result) = result {
                let Pending { sent, action, .. } = self.pending.take().unwrap();
                match result {
                    Ok((outcome, taken)) => {
                        self.accept(outcome, &sent);
                        // A success says so until the next read. An error stays until closed.
                        if action.is_none()
                            && matches!(self.action_notice, Some(ActionNotice::Taken(_)))
                        {
                            self.action_notice = None;
                        }
                        if let (Some(name), Some(taken)) = (action, taken) {
                            self.action_notice = Some(match taken {
                                Ok(()) => ActionNotice::Taken(format!(
                                    "D&D Beyond took the {name}. The values below are the sheet’s new values."
                                )),
                                Err(e) => ActionNotice::Failed(e),
                            });
                        }
                    }
                    Err(error) => {
                        if action.is_some() {
                            self.action_notice = Some(ActionNotice::Failed(error.clone()));
                        }
                        self.error = Some(error);
                        if let Some(remote) = &self.remote {
                            for (key, value) in self.desired.iter() {
                                if remote.groups.get(key) != Some(value) {
                                    self.blocked.insert(key.clone(),"Write status is uncertain. Refresh and review before retrying.".into());
                                }
                            }
                        }
                    }
                }
                self.next = Some(
                    Instant::now()
                        + if self.error.is_some() {
                            Duration::from_secs(60)
                        } else {
                            INTERVAL
                        },
                );
            }
        }
        let pending_edits = self.dirty()
            && !self.read_only()
            && self.desired.iter().any(|(k, v)| {
                !self.blocked.contains_key(k)
                    && self.remote.as_ref().and_then(|r| r.groups.get(k)) != Some(v)
            });
        let due = self.next.is_none_or(|t| t <= Instant::now())
            || self.action.is_some()
            || (pending_edits
                && self
                    .changed
                    .is_none_or(|t| t.elapsed() >= Duration::from_millis(500)));
        if self.pending.is_none() && due {
            let baseline = self.remote.clone();
            let desired = self.desired.clone();
            let blocked = self.blocked.keys().cloned().collect();
            let read_only = std::mem::take(&mut self.refresh_only);
            // Taken here, so it goes once, whatever the result.
            let action = self.action.take();
            let id = id.to_owned();
            let campaign = campaign.to_owned();
            let session = session.unwrap();
            let gate = Arc::new(AtomicBool::new(true));
            self.gate = Some(gate.clone());
            let (tx, rx) = mpsc::channel();
            self.pending = Some(Pending {
                receiver: rx,
                sent: desired.clone(),
                action: action.as_ref().map(SheetAction::name),
            });
            let ctx = ctx.clone();
            std::thread::spawn(move || {
                static LOCKS: OnceLock<Mutex<BTreeMap<String, Arc<Mutex<()>>>>> = OnceLock::new();
                let result = (|| {
                    let lock = LOCKS
                        .get_or_init(Default::default)
                        .lock()
                        .map_err(|_| "Sync lock unavailable.")?
                        .entry(id.clone())
                        .or_default()
                        .clone();
                    let _guard = lock
                        .lock()
                        .map_err(|_| "Character sync lock unavailable.")?;
                    let client = Client::builder()
                        .timeout(Duration::from_secs(20))
                        .redirect(Policy::none())
                        .build()
                        .map_err(|_| "Could not create the sync client.")?;
                    let mut live = Live {
                        session,
                        client,
                        id,
                        campaign,
                        active: gate,
                    };
                    match &action {
                        Some(action) => act(&mut live, action),
                        None => {
                            exchange(&mut live, baseline.as_ref(), &desired, &blocked, read_only)
                                .map(|outcome| (outcome, None))
                        }
                    }
                })();
                let _ = tx.send(result);
                ctx.request_repaint();
            });
        }
        ctx.request_repaint_after(Duration::from_millis(500));
    }
    /// A new session or character starts clean, apart from what was last matched: that is
    /// how the next read can tell the DM's changes from the sheet's.
    fn enter_scope(&mut self, scope: (u64, String, String)) {
        let enabled = self.enabled;
        let keeps_all_unsent = self.keeps_all_unsent;
        // What this character's sheet held says nothing about another character's.
        let same_character = self
            .scope
            .as_ref()
            .is_none_or(|old| old.1 == scope.1 && old.2 == scope.2);
        self.keep_unsent();
        let unsent = std::mem::take(&mut self.unsent);
        let matched = self
            .remote
            .take()
            .map(|remote| remote.groups)
            .or_else(|| self.matched.take());
        self.stop();
        let notice = self.action_notice.take();
        *self = Self::default();
        self.enabled = enabled;
        self.keeps_all_unsent = keeps_all_unsent;
        self.scope = Some(scope);
        if same_character {
            self.matched = matched;
            self.unsent = unsent;
            self.action_notice = notice;
        }
    }
    /// True once for each first read, for the run to compare with its own values.
    pub fn take_first_read(&mut self) -> bool {
        std::mem::take(&mut self.first_read)
    }
    pub fn take_matched(&mut self) -> Option<Groups> {
        self.matched.take()
    }
    /// The run's value and the sheet's differ, and the DM changed the run's: their choice.
    pub fn hold_conflict(&mut self, key: &str, local: Value) {
        self.desired.write().insert(key.into(), local);
        self.blocked.insert(key.into(), DIFFER_AFTER_READ.into());
    }
    /// A change the DM recorded while sync was off. It cannot be sent yet, and it counts.
    pub fn note_while_stopped(&mut self, key: &str, local: Value) {
        self.desired.write().insert(key.into(), local);
    }
    pub fn use_remote(&mut self, key: &str) {
        self.chose_remote = true;
        self.updated = true;
        if let Some(value) = self.remote.as_ref().and_then(|r| r.groups.get(key)) {
            self.desired.write().insert(key.to_owned(), value.clone());
        } else {
            self.desired.write().remove(key);
        }
        self.blocked.remove(key);
    }
    pub fn keep_local(&mut self, key: &str) {
        self.blocked.remove(key);
        self.changed = None;
        self.next = None;
    }
    /// A read from D&D Beyond, with the sheet's HP. Test builds have no route to a live one.
    #[cfg(test)]
    pub fn simulate_read(&mut self, character: u64, max_hp: i64, damage: i64) {
        self.accept_read(simulated_sheet(character, max_hp, damage, true));
    }
    /// A read of a sheet given in full, in D&D Beyond's own fields.
    #[cfg(test)]
    pub fn simulate_sheet(&mut self, character: u64, max_hp: i64, sheet: &Value) {
        self.accept_read(Remote::from_raw(character, max_hp, sheet).expect("simulated sheet"));
    }
    #[cfg(test)]
    fn accept_read(&mut self, remote: Remote) {
        let sent = self.desired.clone();
        self.accept(
            Outcome {
                remote,
                blocked: BTreeMap::new(),
            },
            &sent,
        );
    }
    #[cfg(test)]
    pub fn conflict_keys(&self) -> Vec<String> {
        self.blocked.keys().cloned().collect()
    }
    fn accept(&mut self, outcome: Outcome, sent: &Groups) {
        let initial = self.remote.is_none();
        self.first_read |= initial;
        let previous = self
            .remote
            .as_ref()
            .map(|r| r.groups.clone())
            .unwrap_or_default();
        for (key, value) in &outcome.remote.groups {
            let local = self.desired.get(key).cloned();
            let unchanged = local.as_ref() == sent.get(key);
            if initial
                || (unchanged
                    && !outcome.blocked.contains_key(key)
                    && !self.blocked.contains_key(key)
                    && (previous.get(key) == sent.get(key) || sent.get(key) == Some(value)))
            {
                self.desired.write().insert(key.clone(), value.clone());
            } else if local.as_ref() == Some(value) {
                self.blocked.remove(key);
            } else if previous.get(key) != Some(value)
                && previous.get(key) != local.as_ref()
                && sent.get(key) != Some(value)
                && !outcome.blocked.contains_key(key)
            {
                self.blocked.insert(
                    key.clone(),
                    "Splat and D&D Beyond both changed this. Choose which to keep.".into(),
                );
            }
        }
        // Removed resources are a conflict if an edit is pending, never a write to an old ID.
        for key in previous
            .keys()
            .filter(|k| !outcome.remote.groups.contains_key(*k))
        {
            if self.desired.get(key) != previous.get(key) {
                self.blocked.insert(
                    key.clone(),
                    "This resource is no longer on the sheet.".into(),
                );
            } else {
                self.desired.write().remove(key);
            }
        }
        self.blocked.extend(outcome.blocked);
        // Back from a stop: an unsent value the sheet does not have is the DM's to settle.
        for (key, value) in std::mem::take(&mut self.unsent) {
            if outcome.remote.groups.get(&key).is_some_and(|v| v != &value) {
                self.hold_conflict(&key, value);
            }
        }
        self.remote = Some(outcome.remote);
        // A new read can add or drop the items and resources the wells measure.
        self.desired.touch();
        self.error = None;
        self.updated = true;
    }
    /// Each conflict with both values and its two ways out. The run shows this beside
    /// the roster, so it does not stop the table.
    pub fn conflicts(&mut self, ui: &mut egui::Ui) {
        self.conflicts_inner(ui, None, true);
    }
    pub(crate) fn conflicts_for(&mut self, ui: &mut egui::Ui, name: &str) {
        self.conflicts_inner(ui, Some(name), false);
    }
    fn conflicts_inner(&mut self, ui: &mut egui::Ui, name: Option<&str>, explanation: bool) {
        use crate::design::{self, Action, Density, Type, colors};
        for (key, message) in self.blocked.clone() {
            let c = colors();
            let status = format!("{} {} conflict", design::icons::WARN, self.group_name(&key));
            match name {
                // FK3: the ended view's one heading form, the name in the text colour.
                Some(name) => {
                    sync_heading(ui, name, &status, c.sync.conflict);
                }
                None => {
                    ui.label(Type::Heading.rich(ui, status).color(c.sync.conflict));
                }
            }
            if explanation {
                ui.add(
                    egui::Label::new(Type::Small.rich(ui, message.as_str()).color(c.text)).wrap(),
                );
            }
            // The two values side by side, so the difference is what the eye finds.
            egui::Grid::new(("conflict_values", &key))
                .num_columns(2)
                .spacing([design::L, 2.0])
                .show(ui, |ui| {
                    for (label, value) in [
                        ("Splat", self.desired.get(&key)),
                        (
                            "D&D Beyond",
                            self.remote.as_ref().and_then(|r| r.groups.get(&key)),
                        ),
                    ] {
                        ui.label(Type::Small.rich(ui, label).color(colors().muted));
                        ui.label(
                            Type::Strong
                                .rich(ui, describe(&key, value, self.max_hp().unwrap_or(0)))
                                .color(colors().text),
                        );
                        ui.end_row();
                    }
                });
            ui.push_id(&key, |ui| {
                ui.horizontal_wrapped(|ui| {
                    let busy = self.busy().then_some("Wait for the current sheet read.");
                    let max = self.max_hp().unwrap_or(0);
                    let sheet = self.remote.as_ref().and_then(|r| r.groups.get(&key));
                    let read_only = self.read_only();
                    let (use_label, keep_label) = (
                        format!("Use {} from D&D Beyond", short_value(&key, sheet, max)),
                        format!(
                            "Keep {}, {}",
                            short_value(&key, self.desired.get(&key), max),
                            if read_only {
                                "in Splat only"
                            } else {
                                "send to D&D Beyond"
                            }
                        ),
                    );
                    let use_word = &use_label;
                    let keep_word = &keep_label;
                    if Action::secondary(use_word)
                        .hint(&use_label)
                        .name("Use D&D Beyond values")
                        .density(Density::Read)
                        .blocked(busy)
                        .show(ui)
                        .clicked()
                    {
                        self.use_remote(&key);
                    }
                    let writable = read_only
                        || self
                            .remote
                            .as_ref()
                            .is_some_and(|r| r.can_edit && r.groups.contains_key(&key));
                    if Action::secondary(keep_word)
                        .hint(&keep_label)
                        .name("Keep Splat values and retry")
                        .density(Density::Read)
                        .blocked(busy.or(
                            (!writable).then_some(
                                "This session cannot edit this character on D&D Beyond.",
                            ),
                        ))
                        .show(ui)
                        .clicked()
                    {
                        self.keep_local(&key);
                    }
                });
            });
        }
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        let was = self.enabled;
        ui.checkbox(&mut self.enabled, "Sync this sheet with D&D Beyond");
        crate::design::help(
            ui,
            "During a fight, Sync party in the run turns sync on for the whole party.",
        );
        if was && !self.enabled {
            self.stop();
        }
        if !was && self.enabled {
            self.start();
        }
        if !self.enabled {
            if let Some(error) = &self.error {
                ui.colored_label(crate::design::colors().result.error, error);
            }
            crate::design::help(
                ui,
                "Enable to read the live sheet. Edits made here will then update D&D Beyond.",
            );
            return;
        }
        crate::design::help(ui, "Splat checks the sheet every 15 seconds.");
        if self.busy() {
            ui.horizontal(|ui| {
                crate::design::spinner(ui);
                ui.label("Checking character…");
            });
        }
        if let Some(e) = &self.error {
            ui.colored_label(crate::design::colors().result.error, e);
        }
        ui.horizontal(|ui| {
            if crate::design::Action::secondary("Check now")
                .density(crate::design::Density::Read)
                .blocked_if(self.busy(), "Wait for the current sheet read.")
                .show(ui)
                .clicked()
            {
                self.refresh_only = true;
                self.next = None;
            }
            if self.dirty() {
                ui.label("Local changes pending");
            } else if self.remote.is_some() {
                ui.label("Values match the last read");
            }
        });
        self.conflicts(ui);
        let Some(remote) = &self.remote else {
            // Help with a link in it: the reader measure, as `design::help` keeps.
            ui.set_max_width(ui.available_width().min(crate::design::READER_MEASURE));
            ui.horizontal_wrapped(|ui| {
                use crate::design::{self, Type};
                ui.spacing_mut().item_spacing.x = 0.0;
                let words = "Waiting for the first sheet read. If it does not come, check the connection in ";
                for word in words.split_inclusive(' ') {
                    ui.label(Type::Small.rich(ui, word).color(design::colors().muted));
                }
                if ui.available_width() < design::number_width(ui, "Settings.", &Type::Small.font(ui)) {
                    ui.end_row();
                }
                if design::small_link(ui, "Settings").clicked() {
                    ui.ctx().data_mut(|data| {
                        data.insert_temp(egui::Id::new("open_account_settings"), true)
                    });
                }
                ui.label(Type::Small.rich(ui, ".").color(design::colors().muted));
            });
            return;
        };
        use crate::design::{self, Type, colors, icons};
        let max_hp = remote.max_hp;
        let writable = remote.can_edit;
        // Say once, at the top, which values can change and what a change does.
        ui.horizontal_wrapped(|ui| {
            let (icon, color, words) = if writable {
                (
                    icons::EDIT,
                    colors().accent,
                    "Values with − and + are editable. Splat sends a change to D&D Beyond half a second after you stop, then reads the sheet to confirm it.",
                )
            } else {
                (
                    icons::READ_ONLY,
                    colors().sync.read_only,
                    "Read only: D&D Beyond reports that this session cannot edit this character. Nothing here can change.",
                )
            };
            ui.label(Type::Small.rich(ui, icon).color(color));
            // Round 25 review: help keeps the reader measure.
            design::help(ui, words);
        });
        // Round 26: an action's result sits in the sheet's footer (`footer_messages`), so neither
        // the result nor its dismissal or the next read moves the fields.
        let wells = self.wells(ui, max_hp);
        let before = self.desired.groups.clone();
        ui.add_enabled_ui(writable, |ui| {
            self.life(ui, max_hp, wells);
            self.rests(ui, max_hp);
            self.sheet_conditions(ui, wells);
            self.spending(ui, wells);
            self.purse(ui, wells);
            self.inventory(ui, wells);
            self.companions(ui, wells);
        });
        if self.desired.groups != before {
            self.desired.touch();
            self.changed = Some(Instant::now());
        }
        design::help(
            ui,
            "Concentration, custom effects, initiative and DM notes stay in Splat. D&D Beyond has no place for them.",
        );
    }
    /// Round 26: a sync action's result (a rest taken, or why it failed), in the footer of the
    /// sheet pane. It lives until its dismiss, the next action or another character; the
    /// dismiss is the close icon at the message end, in the one Party form.
    pub fn footer_messages<'a>(&'a mut self, footer: &mut crate::design::Footer<'a>) {
        use crate::design::{self, Tone};
        let Some((tone, words)) = self.action_notice.as_ref().map(|notice| match notice {
            ActionNotice::Taken(words) => (Tone::Success, words.clone()),
            ActionNotice::Failed(words) => (Tone::Error, words.clone()),
        }) else {
            return;
        };
        let notice = &mut self.action_notice;
        footer.result(words.clone(), false, move |ui| {
            if crate::party::dismissable(ui, |ui| {
                design::message(ui, tone, &words);
            }) {
                *notice = None;
            }
        });
    }
    /// The conflict heading for a group: its name on the sheet, where it has one.
    fn group_name(&self, key: &str) -> String {
        match self.remote.as_ref().and_then(|r| r.labels.get(key)) {
            Some(name) if key.starts_with("creature/") => format!("{name} HP"),
            Some(name) => name.clone(),
            None => group_label(key).into(),
        }
    }
    /// The stepper wells for this frame: kept until a value, the maximum HP or the type size
    /// changes, so an unchanged frame or a resize reads no values.
    fn wells(&mut self, ui: &egui::Ui, max_hp: i64) -> Wells {
        use crate::design::Type;
        #[cfg(test)]
        {
            self.well_counts.0 += 1;
        }
        let key = (
            self.desired.revision,
            max_hp,
            Type::Small.font(ui).size.to_bits(),
            ui.style()
                .drag_value_text_style
                .resolve(ui.style())
                .size
                .to_bits(),
        );
        if let Some((cached, wells)) = self.wells
            && cached == key
        {
            return wells;
        }
        let wells = self.measure_wells(ui, max_hp);
        self.wells = Some((key, wells));
        wells
    }
    /// Reads every value the panel shows to find each block's widest. Only `wells` calls it.
    fn measure_wells(&mut self, ui: &egui::Ui, max_hp: i64) -> Wells {
        #[cfg(test)]
        {
            self.well_counts.1 += 1;
        }
        let digit = digit_width(ui);
        let well = |values: &mut dyn Iterator<Item = i64>| {
            well_for(ui, digit, values.map(digits).max().unwrap_or(1))
        };
        let groups = &self.desired.groups;
        let field = |key: &str, field: &str| groups.get(key).and_then(|g| g[field].as_i64());
        let named = |prefix: &'static str| {
            self.remote
                .iter()
                .flat_map(move |remote| remote.named(prefix))
                .filter_map(|(key, _, _)| groups.get(key))
        };
        let life = well(
            &mut [
                Some(max_hp),
                field("hp", "temporaryHitPoints"),
                field("hp_bonus", "bonusHitPoints"),
            ]
            .into_iter()
            .flatten(),
        );
        let slots = well(
            &mut ["slots", "pact"]
                .into_iter()
                .filter_map(|key| groups.get(key))
                .flat_map(|slots| {
                    (1..=9).filter_map(|level| slots[format!("level{level}")].as_i64())
                }),
        );
        let limited = well(&mut named("limited/").filter_map(|group| group["uses"].as_i64()));
        let currency = well(
            &mut COINS
                .iter()
                .filter_map(|(coin, _, _)| field("currency", coin)),
        );
        let exhaustion = well(&mut field("conditions", "4").into_iter());
        let inventory = well(&mut named("item/").flat_map(|item| {
            [
                item["quantity"].as_i64(),
                item.get("charges").and_then(Value::as_i64),
            ]
            .into_iter()
            .flatten()
        }));
        let companions = well(&mut named("creature/").flat_map(|creature| {
            [
                creature["removedHitPoints"].as_i64(),
                creature["temporaryHitPoints"].as_i64(),
            ]
            .into_iter()
            .flatten()
        }));
        Wells {
            life,
            slots,
            limited,
            currency,
            exhaustion: Column::new(ui, &["Exhaustion level"], exhaustion),
            inventory: Column::new(ui, &["Quantity", "Charges spent"], inventory),
            companions: Column::new(ui, &["Damage taken", "Temporary HP"], companions),
        }
    }
    /// HP, temporary HP, the maximum HP modifier, death saves and inspiration.
    fn life(&mut self, ui: &mut egui::Ui, max_hp: i64, wells: Wells) {
        use crate::design;
        ui.add_space(design::XS);
        design::group_label(ui, "Hit points");
        sheet_grid(ui, "sheet_life", wells.life, |ui| {
            if let Some(hp) = self.desired.groups.get_mut("hp") {
                let mut current = current_hp(hp, max_hp);
                let note = Some(format!("of {max_hp}"));
                if edit_row(ui, "Current HP", &mut current, 0..=max_hp, note) {
                    hp["removedHitPoints"] = (max_hp - current).into();
                }
                step_field(hp, "temporaryHitPoints", |n| {
                    edit_row(ui, "Temporary HP", n, 0..=MAX_HP_BONUS, None)
                });
            }
            if let Some(bonus) = self.desired.groups.get_mut("hp_bonus") {
                step_field(bonus, "bonusHitPoints", |n| {
                    let range = -MAX_HP_BONUS..=MAX_HP_BONUS;
                    edit_row(ui, "Maximum HP modifier", n, range, None)
                });
            }
            if let Some(death) = self.desired.groups.get_mut("death") {
                for (field, label) in [
                    ("successCount", "Death save successes"),
                    ("failCount", "Death save failures"),
                ] {
                    step_field(death, field, |n| edit_row(ui, label, n, 0..=3, None));
                }
            }
            if let Some(inspiration) = self.desired.groups.get_mut("inspiration") {
                row_label(ui, "Inspiration");
                let mut on = inspiration["inspiration"] == true;
                if ui.checkbox(&mut on, "Has inspiration").changed() {
                    inspiration["inspiration"] = on.into();
                }
                ui.end_row();
            }
        });
        if self.desired.contains_key("hp_bonus") {
            // Round 26 AM2: every help line keeps the reader measure.
            crate::design::help(
                ui,
                "The maximum HP modifier raises or lowers maximum HP, as Aid does.",
            );
        }
    }
    /// Short rest, long rest and restore to life. D&D Beyond applies each one itself.
    fn rests(&mut self, ui: &mut egui::Ui, max_hp: i64) {
        use crate::design::{self, Action, Density, Type, colors, icons};
        ui.add_space(design::S);
        design::group_label(ui, "Rests and recovery");
        let blocker = self.action_blocker();
        let hp = self.desired.get("hp").map(|hp| current_hp(hp, max_hp));
        // Taken out while it shows, and put back unless the DM takes the rest or cancels.
        match self.rest_form.take() {
            None => {
                ui.horizontal_wrapped(|ui| {
                    if Action::secondary("Short rest…")
                        .icon(icons::SHORT_REST)
                        .density(Density::Read)
                        .blocked(blocker)
                        .show(ui)
                        .clicked()
                    {
                        self.rest_form = Some(RestForm::Short {
                            hit_dice: BTreeMap::new(),
                            reset_max_hp_bonus: false,
                        });
                    }
                    if Action::secondary("Long rest…")
                        .icon(icons::LONG_REST)
                        .density(Density::Read)
                        .blocked(blocker)
                        .show(ui)
                        .clicked()
                    {
                        self.rest_form = Some(RestForm::Long {
                            reset_max_hp_bonus: false,
                            reduce_exhaustion: true,
                        });
                    }
                    // Restore to life is for a character at 0 HP, as on the sheet.
                    if hp == Some(0) {
                        for (full, label) in [
                            (true, "Restore to life with full HP"),
                            (false, "Restore to life with 1 HP"),
                        ] {
                            if Action::secondary(label)
                                .density(Density::Read)
                                .blocked(blocker)
                                .show(ui)
                                .clicked()
                            {
                                let _ = self.request(SheetAction::RestoreLife { full });
                            }
                        }
                    }
                });
                // Round 26 AM2: help keeps the reader measure.
                design::help(
                    ui,
                    "A rest goes to D&D Beyond once, when you take it. D&D Beyond resets the resources by its own rules.",
                );
            }
            Some(mut form) => {
                let (title, take_label, note) = match form {
                    RestForm::Short { .. } => (
                        "Short rest",
                        "Take short rest",
                        "This marks the hit dice as spent. Add the HP that they give in Current HP.",
                    ),
                    RestForm::Long { .. } => (
                        "Long rest",
                        "Take long rest",
                        "D&D Beyond restores HP, hit dice, spell slots and resources by its own rules.",
                    ),
                };
                ui.label(Type::Heading.rich(ui, title).color(colors().text));
                let reset_max_hp_bonus = match &mut form {
                    RestForm::Short {
                        hit_dice,
                        reset_max_hp_bonus,
                    } => {
                        ui.label(
                            Type::Small
                                .rich(ui, "Hit dice to spend")
                                .color(colors().muted),
                        );
                        // A class has at most 20 hit dice: two digits, with no scan.
                        let well = well_for(ui, digit_width(ui), 2);
                        sheet_grid(ui, "sheet_short_rest", well, |ui| {
                            for dice in self.hit_dice() {
                                let left = (dice.level - dice.used).max(0);
                                edit_row(
                                    ui,
                                    &format!("{} (d{})", dice.name, dice.die),
                                    hit_dice.entry(dice.class).or_insert(0),
                                    0..=left,
                                    Some(format!("{left} of {} left", dice.level)),
                                );
                            }
                        });
                        reset_max_hp_bonus
                    }
                    RestForm::Long {
                        reset_max_hp_bonus, ..
                    } => reset_max_hp_bonus,
                };
                ui.checkbox(reset_max_hp_bonus, "Reset the maximum HP modifier to 0");
                if let RestForm::Long {
                    reduce_exhaustion, ..
                } = &mut form
                {
                    ui.checkbox(reduce_exhaustion, "Reduce Exhaustion by one level");
                }
                design::help(ui, note);
                let (mut take, mut cancel) = (false, false);
                ui.horizontal(|ui| {
                    take = Action::primary(take_label)
                        .density(Density::Read)
                        .blocked(blocker)
                        .show(ui)
                        .clicked();
                    cancel = Action::quiet("Cancel")
                        .density(Density::Read)
                        .show(ui)
                        .clicked();
                });
                if take {
                    let _ = self.request(form.into_action());
                } else if !cancel {
                    self.rest_form = Some(form);
                }
            }
        }
    }
    fn sheet_conditions(&mut self, ui: &mut egui::Ui, wells: Wells) {
        crate::design::section(ui, "Sheet conditions", |ui| {
            if let Some(Value::Object(conditions)) = self.desired.groups.get_mut("conditions") {
                ui.horizontal_wrapped(|ui| {
                    for (id, name) in CONDITIONS {
                        let key = id.to_string();
                        let mut present = conditions.contains_key(&key);
                        if ui.checkbox(&mut present, name).changed() {
                            if present {
                                conditions.insert(
                                    key.clone(),
                                    if id == 4 { 1.into() } else { Value::Null },
                                );
                            } else {
                                conditions.remove(&key);
                            }
                        }
                    }
                });
                // On its own line: a stepper inside a wrapping row can run off a narrow column.
                if let Some(level) = conditions.get_mut("4") {
                    let mut n = level.as_i64().unwrap_or(1);
                    if labelled_stepper(
                        ui,
                        wells.exhaustion,
                        "Exhaustion level",
                        "Exhaustion level",
                        &mut n,
                        1..=6,
                        None,
                    ) {
                        *level = n.into();
                    }
                }
            }
        });
    }
    /// Spell slots and limited uses spent.
    fn spending(&mut self, ui: &mut egui::Ui, wells: Wells) {
        use crate::design;
        design::section(ui, "Spell slots spent", |ui| {
            sheet_grid(ui, "sheet_slots", wells.slots, |ui| {
                for (key, label, levels) in [("slots", "Spell slots", 9), ("pact", "Pact slots", 5)]
                {
                    let Some(slots) = self.desired.groups.get_mut(key) else {
                        continue;
                    };
                    for level in 1..=levels {
                        let label = format!("{label}, level {level}");
                        step_field(slots, &format!("level{level}"), |n| {
                            edit_row(ui, &label, n, 0..=99, None)
                        });
                    }
                }
            });
        });
        let remote = self.remote.as_ref();
        design::section(ui, "Limited uses spent", |ui| {
            sheet_grid(ui, "sheet_limited", wells.limited, |ui| {
                for (key, label, _) in remote.into_iter().flat_map(|r| r.named("limited/")) {
                    if let Some(resource) = self.desired.groups.get_mut(key) {
                        step_field(resource, "uses", |n| {
                            edit_row(ui, label, n, 0..=MAX_COUNT, None)
                        });
                    }
                }
            });
        });
    }
    fn purse(&mut self, ui: &mut egui::Ui, wells: Wells) {
        let Some(purse) = self.desired.groups.get_mut("currency") else {
            return;
        };
        crate::design::section(ui, "Currency", |ui| {
            sheet_grid(ui, "sheet_currency", wells.currency, |ui| {
                for (field, _, name) in COINS {
                    step_field(purse, field, |n| edit_row(ui, name, n, 0..=MAX_COUNT, None));
                }
            });
        });
    }
    /// Each item's quantity, and the charges spent where it has charges.
    fn inventory(&mut self, ui: &mut egui::Ui, wells: Wells) {
        use crate::design;
        let Some(remote) = &self.remote else {
            return;
        };
        let items: Vec<_> = remote.named("item/").collect();
        if items.is_empty() {
            return;
        }
        // One label column and one well width for every item, so each − and each + of the
        // section sits on one x (round 24 FK4).
        let column = wells.inventory;
        // The title holds the count, so the section keeps its own id as items come and go.
        design::section_with_id(
            ui,
            "sheet_inventory",
            format!("Inventory · {} items", items.len()),
            |ui| {
                for (key, name, max) in items {
                    let Some(item) = self.desired.groups.get_mut(key) else {
                        continue;
                    };
                    // The name on its own line, and each value on a line under it, so a narrow
                    // column never cuts one off.
                    row_label(ui, name);
                    ui.indent(key, |ui| {
                        step_field(item, "quantity", |n| {
                            let name = format!("{name} quantity");
                            labelled_stepper(ui, column, "Quantity", &name, n, 1..=MAX_COUNT, None)
                        });
                        if item.get("charges").is_some() {
                            step_field(item, "charges", |n| {
                                let name = format!("{name} charges spent");
                                let note = max.map(|max| format!("of {max}"));
                                labelled_stepper(
                                    ui,
                                    column,
                                    "Charges spent",
                                    &name,
                                    n,
                                    0..=MAX_COUNT,
                                    note,
                                )
                            });
                        }
                    });
                    ui.add_space(design::XS);
                }
                design::help(
                    ui,
                    "Party inventory and the coins in containers stay on D&D Beyond.",
                );
            },
        );
    }
    /// Companion and summon HP, as damage taken: the sheet works their maximum out itself.
    fn companions(&mut self, ui: &mut egui::Ui, wells: Wells) {
        use crate::design;
        let Some(remote) = &self.remote else {
            return;
        };
        let creatures: Vec<_> = remote.named("creature/").collect();
        if creatures.is_empty() {
            return;
        }
        let column = wells.companions;
        design::section(ui, "Companions and summons", |ui| {
            for (key, name, max) in creatures {
                let Some(creature) = self.desired.groups.get_mut(key) else {
                    continue;
                };
                row_label(ui, name);
                ui.indent(key, |ui| {
                    // The sheet works a companion's maximum out itself. The stat block's
                    // average is only a guide, so it is a note and not a limit.
                    step_field(creature, "removedHitPoints", |n| {
                        let name = format!("{name} damage taken");
                        let note = max.map(|max| format!("Stat block average {max} HP"));
                        labelled_stepper(ui, column, "Damage taken", &name, n, 0..=MAX_COUNT, note)
                    });
                    step_field(creature, "temporaryHitPoints", |n| {
                        let name = format!("{name} temporary HP");
                        labelled_stepper(
                            ui,
                            column,
                            "Temporary HP",
                            &name,
                            n,
                            0..=MAX_HP_BONUS,
                            None,
                        )
                    });
                });
                ui.add_space(design::XS);
            }
        });
    }
}
/// A small label, the value's stepper and an optional note, kept together on one line.
fn labelled_stepper(
    ui: &mut egui::Ui,
    column: Column,
    label: &str,
    name: &str,
    value: &mut i64,
    range: std::ops::RangeInclusive<i64>,
    note: Option<String>,
) -> bool {
    use crate::design::{Type, colors};
    ui.horizontal(|ui| {
        let left = ui.cursor().left();
        ui.label(Type::Small.rich(ui, label).color(colors().muted));
        // The stepper starts at the block's label column, whatever this label's width.
        let gap = left + column.label - ui.cursor().left();
        if gap > 0.0 {
            ui.add_space(gap);
        }
        ui.spacing_mut().interact_size.x = column.well;
        stepper_and_note(ui, name, value, range, note)
    })
    .inner
}
/// One block of labelled steppers: its label column (the widest label plus `S`) and one well
/// width (the widest value), so every − and every + of the block share an x.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Column {
    label: f32,
    well: f32,
}
impl Column {
    fn new(ui: &egui::Ui, labels: &[&str], well: f32) -> Self {
        use crate::design::{self, Type};
        let font = Type::Small.font(ui);
        let label = labels
            .iter()
            .map(|label| {
                ui.fonts_mut(|fonts| {
                    fonts.layout_no_wrap(
                        (*label).to_owned(),
                        font.clone(),
                        egui::Color32::PLACEHOLDER,
                    )
                })
                .size()
                .x
            })
            .fold(0.0, f32::max)
            + design::S;
        Self { label, well }
    }
}
/// Every block's well width and label column. `CombatSync::wells` keeps them until a value, the
/// maximum HP or the type size changes; only `CombatSync::measure_wells` reads the values.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Wells {
    life: f32,
    slots: f32,
    limited: f32,
    currency: f32,
    exhaustion: Column,
    inventory: Column,
    companions: Column,
}
/// What the wells were measured from: the values' revision, the maximum HP, and the label and
/// number type sizes.
type WellsKey = (u64, i64, u32, u32);
/// The characters a whole number shows: its digits, and a minus sign when it is negative.
fn digits(value: i64) -> usize {
    value
        .unsigned_abs()
        .checked_ilog10()
        .map_or(1, |log| log as usize + 1)
        + usize::from(value < 0)
}
/// The widest digit in the stepper's number type.
fn digit_width(ui: &egui::Ui) -> f32 {
    let font = ui.style().drag_value_text_style.resolve(ui.style());
    ('0'..='9')
        .map(|digit| {
            ui.fonts_mut(|fonts| {
                fonts.layout_no_wrap(digit.to_string(), font.clone(), egui::Color32::PLACEHOLDER)
            })
            .size()
            .x
        })
        .fold(0.0, f32::max)
}
/// A stepper's well that holds `digits` characters: their width plus the well's padding, never
/// below the usual control width.
fn well_for(ui: &egui::Ui, digit: f32, digits: usize) -> f32 {
    (digit * digits as f32 + 2.0 * ui.spacing().button_padding.x).max(ui.spacing().interact_size.x)
}
/// A row label in a sheet grid.
fn row_label(ui: &mut egui::Ui, label: &str) {
    use crate::design::{Type, colors};
    ui.label(Type::Body.rich(ui, label).color(colors().text));
}
/// One grid row: a label, the value's stepper, and an optional note. True on a change.
fn edit_row(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut i64,
    range: std::ops::RangeInclusive<i64>,
    note: Option<String>,
) -> bool {
    row_label(ui, label);
    let changed = ui
        .horizontal(|ui| stepper_and_note(ui, label, value, range, note))
        .inner;
    ui.end_row();
    changed
}
/// The value's stepper, and a small note after it. True on a change.
fn stepper_and_note(
    ui: &mut egui::Ui,
    name: &str,
    value: &mut i64,
    range: std::ops::RangeInclusive<i64>,
    note: Option<String>,
) -> bool {
    use crate::design;
    let changed = design::stepper(ui, name, value, range);
    if let Some(note) = note {
        design::help(ui, &note);
    }
    changed
}
/// A two-column grid of sheet values: a label on the left, its stepper on the right. Every
/// well is `well` wide, so the − and + of the grid each share an x.
fn sheet_grid(ui: &mut egui::Ui, id: &str, well: f32, rows: impl FnOnce(&mut egui::Ui)) {
    use crate::design;
    ui.scope(|ui| {
        ui.spacing_mut().interact_size.x = well;
        egui::Grid::new(id)
            .num_columns(2)
            .spacing([design::L, design::XS])
            .show(ui, rows);
    });
}
/// Edit the whole number at `field` in a sheet group. The group changes only when `edit` says
/// the number changed.
fn step_field(group: &mut Value, field: &str, edit: impl FnOnce(&mut i64) -> bool) {
    let mut n = group[field].as_i64().unwrap_or(0);
    if edit(&mut n) {
        group[field] = n.into();
    }
}
/// Current HP from the HP group, never below 0.
fn current_hp(hp: &Value, max_hp: i64) -> i64 {
    (max_hp - hp["removedHitPoints"].as_i64().unwrap_or(0)).max(0)
}
fn describe(key: &str, value: Option<&Value>, max_hp: i64) -> String {
    let Some(value) = value else {
        return "Unavailable".into();
    };
    match key {
        "hp" => format!(
            "{} / {max_hp} HP, {} temporary HP",
            (max_hp - value["removedHitPoints"].as_i64().unwrap_or(0)).max(0),
            value["temporaryHitPoints"]
        ),
        "death" => format!(
            "{} successes, {} failures",
            value["successCount"], value["failCount"]
        ),
        "conditions" => {
            let names: Vec<_> = CONDITIONS
                .iter()
                .filter_map(|(id, name)| {
                    value.get(id.to_string()).map(|level| {
                        if *id == 4 {
                            format!("{name} {level}")
                        } else {
                            (*name).into()
                        }
                    })
                })
                .collect();
            if names.is_empty() {
                "None".into()
            } else {
                names.join(", ")
            }
        }
        "slots" | "pact" => value
            .as_object()
            .map(|slots| {
                slots
                    .iter()
                    .map(|(level, count)| {
                        format!("Level {}: {count} spent", level.trim_start_matches("level"))
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default(),
        "inspiration" => {
            if value["inspiration"] == true {
                "Inspiration".into()
            } else {
                "No inspiration".into()
            }
        }
        "hp_bonus" => format!(
            "{} maximum HP",
            signed(value["bonusHitPoints"].as_i64().unwrap_or(0))
        ),
        "currency" => COINS
            .iter()
            .map(|(field, ..)| format!("{} {field}", value[field]))
            .collect::<Vec<_>>()
            .join(", "),
        key if key.starts_with("item/") => match value.get("charges") {
            Some(charges) => format!("Quantity {}, {charges} charges spent", value["quantity"]),
            None => format!("Quantity {}", value["quantity"]),
        },
        key if key.starts_with("creature/") => format!(
            "{} damage taken, {} temporary HP",
            value["removedHitPoints"], value["temporaryHitPoints"]
        ),
        _ => format!("{} uses spent", value["uses"]),
    }
}
/// A whole number with its sign, with a true minus.
fn signed(n: i64) -> String {
    if n < 0 {
        format!("−{}", n.unsigned_abs())
    } else {
        format!("+{n}")
    }
}
/// A value on a button: the number, for HP, and otherwise which side's values.
fn short_value(key: &str, value: Option<&Value>, max_hp: i64) -> String {
    match (key, value) {
        ("hp", Some(hp)) => format!(
            "{}/{max_hp} HP",
            (max_hp - hp["removedHitPoints"].as_i64().unwrap_or(0)).max(0)
        ),
        _ => "these values".to_owned(),
    }
}

fn group_label(key: &str) -> &str {
    match key {
        "hp" => "HP",
        "death" => "Death saves",
        "conditions" => "Conditions",
        "slots" => "Spell slots",
        "pact" => "Pact slots",
        "inspiration" => "Inspiration",
        "hp_bonus" => "Maximum HP modifier",
        "currency" => "Currency",
        key if key.starts_with("item/") => "Item",
        key if key.starts_with("creature/") => "Companion HP",
        _ => "Limited-use resource",
    }
}

/// A character's sync block heading in the ended view, one form for every state (FK3): the
/// name in `Heading` and the text colour, ` · `, then the status's icon and word in its colour.
/// A status is an icon, a word and a colour; the name never takes the status colour.
pub(crate) fn sync_heading(
    ui: &mut egui::Ui,
    name: &str,
    status: &str,
    color: egui::Color32,
) -> egui::Response {
    use crate::design::{Type, colors};
    let font = Type::Heading.font(ui);
    let mut job = egui::text::LayoutJob::default();
    for (text, color) in [
        (name, colors().text),
        (" · ", colors().muted),
        (status, color),
    ] {
        job.append(
            text,
            0.0,
            egui::text::TextFormat {
                font_id: font.clone(),
                color,
                ..Default::default()
            },
        );
    }
    ui.add(egui::Label::new(job).wrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn remote() -> Remote {
        Remote::from_raw(123,30,&json!({"id":123,"canEdit":true,"removedHitPoints":5,"temporaryHitPoints":3,"deathSaves":{"successCount":0,"failCount":0},"conditions":[{"id":11,"level":null}],"spellSlots":[{"level":1,"used":1},{"level":2,"used":2}],"pactMagic":[{"level":2,"used":1}],"actions":{"class":[{"id":8,"entityTypeId":9,"name":"Second Wind","limitedUse":{"numberUsed":1}}]},"classSpells":[{"spells":[{"id":20,"entityTypeId":21,"definition":{"id":999,"entityTypeId":998,"name":"Gift spell"},"limitedUse":{"numberUsed":0}}]}]})).unwrap()
    }
    struct Fake {
        remote: Remote,
        writes: Vec<Mutation>,
        no_apply: bool,
        failure_after_apply: bool,
        read_count: usize,
    }
    impl Fake {
        fn new(remote: Remote) -> Self {
            Self {
                remote,
                writes: Vec::new(),
                no_apply: false,
                failure_after_apply: false,
                read_count: 0,
            }
        }
    }
    impl Transport for Fake {
        fn read(&mut self) -> Result<Remote, String> {
            self.read_count += 1;
            Ok(self.remote.clone())
        }
        fn write(&mut self, m: &Mutation) -> Result<(), String> {
            self.writes.push(m.clone());
            let key = match m.route.as_str() {
                "/life/hp/damage-taken" => "hp".into(),
                "/life/death-saves" => "death".into(),
                "/spell/slots" => "slots".into(),
                "/spell/pact-magic" => "pact".into(),
                "/condition" => "conditions".into(),
                "/action/limited-use" => {
                    format!("limited/{}/{}", m.body["entityTypeId"], m.body["id"])
                }
                "/character/inspiration" => "inspiration".into(),
                "/life/hp/bonus" => "hp_bonus".into(),
                "/inventory/item/quantity" | "/inventory/item/charge" => {
                    format!("item/{}", m.body["id"])
                }
                "/creature/hp" => format!("creature/{}", m.body["id"]),
                route if route.starts_with("/inventory/currency/") => "currency".into(),
                // A sheet action changes what D&D Beyond's rules say. The fake keeps the sheet.
                "/character/rest/short" | "/character/rest/long" | "/life/restore" => {
                    return if self.failure_after_apply {
                        Err("Timeout after send".into())
                    } else {
                        Ok(())
                    };
                }
                _ => panic!("Unexpected route"),
            };
            if !self.no_apply {
                self.remote.groups.insert(key, m.expected.clone());
            }
            if self.failure_after_apply {
                Err("Timeout after send".into())
            } else {
                Ok(())
            }
        }
    }
    #[test]
    fn a_sheet_this_session_cannot_edit_keeps_the_runs_value_and_holds_nothing_up() {
        let mut read_only = remote();
        read_only.can_edit = false;
        let mut sync = CombatSync::default();
        sync.enabled = true;
        sync.remote = Some(read_only.clone());
        *sync.desired.write() = read_only.groups.clone();
        sync.desired.write().get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        assert!(sync.dirty());
        assert!(
            !sync.unconfirmed(),
            "nothing is on its way, so a close can go"
        );
        assert_eq!(sync.state(), SyncState::ReadOnly);
        // A read leaves it alone.
        let sent = sync.desired.clone();
        sync.accept(
            Outcome {
                remote: read_only.clone(),
                blocked: BTreeMap::new(),
            },
            &sent,
        );
        assert_eq!(sync.desired["hp"]["removedHitPoints"], 8);
        assert!(sync.blocked.is_empty());
        // The sheet changes too: the DM chooses, and can keep their value here.
        let mut changed = read_only;
        changed.groups.get_mut("hp").unwrap()["removedHitPoints"] = 12.into();
        sync.accept(
            Outcome {
                remote: changed.clone(),
                blocked: BTreeMap::new(),
            },
            &sent,
        );
        assert_eq!(sync.state(), SyncState::Conflict);
        sync.keep_local("hp");
        sync.accept(
            Outcome {
                remote: changed,
                blocked: BTreeMap::new(),
            },
            &sent,
        );
        assert_eq!(sync.state(), SyncState::ReadOnly);
        assert_eq!(sync.desired["hp"]["removedHitPoints"], 8);
    }
    #[test]
    fn unsent_values_stay_with_their_character() {
        let mut sync = CombatSync::default();
        sync.keep_all_unsent();
        sync.enabled = true;
        sync.enter_scope((1, "1".into(), "123".into()));
        sync.remote = Some(remote());
        *sync.desired.write() = remote().groups;
        sync.desired.write().get_mut("slots").unwrap()["level1"] = 9.into();
        sync.enter_scope((1, "1".into(), "456".into()));
        assert!(sync.unsent.is_empty(), "these are the other character's");
        assert_eq!(sync.take_matched(), None);
    }
    #[test]
    fn the_party_page_keeps_an_unsent_hp_change_across_a_restart() {
        let mut sync = CombatSync::default();
        sync.keep_all_unsent();
        sync.enabled = true;
        sync.remote = Some(remote());
        *sync.desired.write() = remote().groups;
        sync.desired.write().get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        sync.stop();
        sync.start();
        sync.accept_read(remote());
        assert_eq!(sync.conflict_keys(), ["hp"]);
        assert_eq!(sync.desired["hp"]["removedHitPoints"], 8);
    }
    #[test]
    fn exact_hp_payload_preserves_untouched_damage_when_only_temp_hp_changes() {
        let r = remote();
        let mut desired = r.groups.clone();
        desired.get_mut("hp").unwrap()["temporaryHitPoints"] = 7.into();
        let mut fake = Fake::new(r.clone());
        let outcome = exchange(&mut fake, Some(&r), &desired, &BTreeSet::new(), false).unwrap();
        assert_eq!(fake.writes.len(), 1);
        assert_eq!(
            fake.writes[0].body,
            json!({"characterId":123,"removedHitPoints":5,"temporaryHitPoints":7})
        );
        assert_eq!(fake.read_count, 2);
        assert!(outcome.blocked.is_empty());
        assert_eq!(outcome.remote.groups["hp"], desired["hp"]);
    }
    #[test]
    fn inbound_changes_and_same_field_conflicts_do_not_send_stale_values() {
        let old = remote();
        let mut current = old.clone();
        current.groups.get_mut("hp").unwrap()["removedHitPoints"] = 12.into();
        let mut fake = Fake::new(current.clone());
        let read = exchange(&mut fake, Some(&old), &old.groups, &BTreeSet::new(), false).unwrap();
        assert_eq!(read.remote.groups, current.groups);
        assert!(fake.writes.is_empty());
        let mut desired = old.groups.clone();
        desired.get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        let result = exchange(&mut fake, Some(&old), &desired, &BTreeSet::new(), false).unwrap();
        assert!(result.blocked.contains_key("hp"));
        assert!(fake.writes.is_empty());
    }
    #[test]
    fn read_only_permission_and_changed_maximum_block_writes() {
        let old = remote();
        let mut target = old.groups.clone();
        target.get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        let mut denied = old.clone();
        denied.can_edit = false;
        let mut fake = Fake::new(denied);
        assert!(
            exchange(&mut fake, Some(&old), &target, &BTreeSet::new(), false)
                .unwrap()
                .blocked
                .is_empty()
        );
        assert!(fake.writes.is_empty());
        let mut changed = old.clone();
        changed.max_hp = 40;
        let mut fake = Fake::new(changed);
        assert!(
            exchange(&mut fake, Some(&old), &target, &BTreeSet::new(), false)
                .unwrap()
                .blocked
                .contains_key("hp")
        );
        assert!(fake.writes.is_empty());
    }
    #[test]
    fn conditions_use_real_ids_level_and_total_hp_with_separate_delete() {
        let r = remote();
        let target = json!({"4":2,"12":null});
        let changes = mutations(&r, "conditions", &target).unwrap();
        assert_eq!(changes.len(), 3);
        assert_eq!(changes[0].method, Method::DELETE);
        assert_eq!(changes[0].body, json!({"characterId":123,"id":11}));
        let exhaustion = changes.iter().find(|m| m.body["id"] == 4).unwrap();
        assert_eq!(
            exhaustion.body,
            json!({"characterId":123,"id":4,"level":2,"totalHp":30})
        );
        assert_eq!(changes.last().unwrap().expected, target);
        assert!(mutations(&r, "conditions", &json!({"4":7})).is_err());
    }
    #[test]
    fn slot_pools_and_resource_mapping_ids_stay_separate() {
        let r = remote();
        assert_eq!(r.groups["slots"]["level2"], 2);
        assert_eq!(r.groups["pact"]["level2"], 1);
        assert!(r.groups.contains_key("limited/21/20"));
        assert!(!r.groups.contains_key("limited/998/999"));
        let mut desired = r.groups.clone();
        desired.get_mut("slots").unwrap()["level1"] = 2.into();
        desired.get_mut("pact").unwrap()["level2"] = 2.into();
        desired.get_mut("limited/21/20").unwrap()["uses"] = 1.into();
        let mut fake = Fake::new(r.clone());
        assert!(
            exchange(&mut fake, Some(&r), &desired, &BTreeSet::new(), false)
                .unwrap()
                .blocked
                .is_empty()
        );
        assert_eq!(fake.writes.len(), 3);
        assert!(
            fake.writes.iter().any(|m| m.route == "/spell/slots"
                && m.body["level2"] == 2
                && m.body["level9"] == 0)
        );
        assert!(fake.writes.iter().any(|m| m.route == "/action/limited-use"
            && m.body["id"] == 20
            && m.body["entityTypeId"] == 21));
    }
    #[test]
    fn uncertain_writes_are_read_back_and_never_blindly_retried() {
        let old = remote();
        let mut desired = old.groups.clone();
        desired.get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        let mut fake = Fake::new(old.clone());
        fake.failure_after_apply = true;
        assert!(
            exchange(&mut fake, Some(&old), &desired, &BTreeSet::new(), false)
                .unwrap()
                .blocked
                .is_empty()
        );
        assert_eq!(fake.writes.len(), 1);
        exchange(&mut fake, Some(&old), &desired, &BTreeSet::new(), false).unwrap();
        assert_eq!(fake.writes.len(), 1);
        let mut fake = Fake::new(old.clone());
        fake.no_apply = true;
        assert!(
            exchange(&mut fake, Some(&old), &desired, &BTreeSet::new(), false)
                .unwrap()
                .blocked
                .contains_key("hp")
        );
        assert_eq!(fake.writes.len(), 1);
        exchange(
            &mut fake,
            Some(&old),
            &desired,
            &BTreeSet::from(["hp".into()]),
            false,
        )
        .unwrap();
        assert_eq!(fake.writes.len(), 1);
    }
    #[test]
    fn null_counters_are_zero_but_missing_or_invalid_values_are_not_invented() {
        let r = Remote::from_raw(
            1,
            20,
            &json!({"id":1,"removedHitPoints":null,"temporaryHitPoints":null,"deathSaves":null}),
        )
        .unwrap();
        assert_eq!(
            r.groups["hp"],
            json!({"removedHitPoints":0,"temporaryHitPoints":0})
        );
        assert_eq!(r.groups["death"], json!({"successCount":0,"failCount":0}));
        let missing = Remote::from_raw(1, 20, &json!({"id":1})).unwrap();
        assert!(!missing.groups.contains_key("hp"));
        assert!(
            Remote::from_raw(
                1,
                20,
                &json!({"id":1,"removedHitPoints":-1,"temporaryHitPoints":0})
            )
            .is_err()
        );
    }
    #[test]
    fn initial_connection_only_reads_even_if_given_old_saved_run_values() {
        let old = remote();
        let mut target = old.groups.clone();
        target.get_mut("hp").unwrap()["removedHitPoints"] = 30.into();
        let mut fake = Fake::new(old);
        exchange(&mut fake, None, &target, &BTreeSet::new(), false).unwrap();
        assert!(fake.writes.is_empty());
    }
    #[test]
    fn response_keeps_newer_edits_and_does_not_conflict_with_own_write() {
        let old = remote();
        let mut sync = CombatSync::default();
        sync.remote = Some(old.clone());
        let mut sent = old.groups.clone();
        sent.insert(
            "hp".into(),
            json!({"removedHitPoints":5,"temporaryHitPoints":0}),
        );
        *sync.desired.write() = sent.clone();
        sync.desired.write().get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        let mut confirmed = old;
        confirmed.groups = sent.clone();
        sync.accept(
            Outcome {
                remote: confirmed,
                blocked: BTreeMap::new(),
            },
            &sent,
        );
        assert_eq!(sync.desired["hp"]["removedHitPoints"], 8);
        assert!(sync.blocked.is_empty());
        assert!(sync.dirty());
    }
    #[test]
    fn a_read_with_nothing_to_send_does_not_hold_a_close_and_a_write_does() {
        let old = remote();
        let mut sync = CombatSync::default();
        sync.enabled = true;
        *sync.desired.write() = old.groups.clone();
        sync.remote = Some(old.clone());
        let (_tx, rx) = mpsc::channel();
        sync.pending = Some(Pending {
            receiver: rx,
            sent: old.groups.clone(),
            action: None,
        });
        assert!(sync.busy() && !sync.unconfirmed());
        let mut sent = old.groups.clone();
        sent.get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        sync.pending.as_mut().unwrap().sent = sent;
        assert!(sync.unconfirmed(), "the DM's change is on its way");
    }
    #[test]
    fn a_new_session_keeps_what_was_last_matched() {
        let mut sync = CombatSync::default();
        sync.enabled = true;
        sync.remote = Some(remote());
        let matched = remote().groups;
        sync.enter_scope((2, "1".into(), "123".into()));
        assert!(sync.enabled);
        assert!(sync.remote.is_none());
        assert_eq!(sync.take_matched(), Some(matched.clone()));
        // Started again, then a new session: what was matched still stands.
        sync.remote = Some(remote());
        sync.start();
        sync.enter_scope((3, "1".into(), "123".into()));
        assert_eq!(sync.take_matched(), Some(matched));
    }
    #[test]
    fn refresh_preserves_unsent_edits_and_blocks_external_conflicts() {
        let old = remote();
        let mut sync = CombatSync::default();
        sync.remote = Some(old.clone());
        *sync.desired.write() = old.groups.clone();
        sync.desired.write().get_mut("hp").unwrap()["removedHitPoints"] = 8.into();
        let sent = sync.desired.clone();
        sync.accept(
            Outcome {
                remote: old.clone(),
                blocked: BTreeMap::new(),
            },
            &sent,
        );
        assert_eq!(*sync.desired, sent);
        let mut changed = old;
        changed.groups.get_mut("hp").unwrap()["removedHitPoints"] = 12.into();
        sync.accept(
            Outcome {
                remote: changed,
                blocked: BTreeMap::new(),
            },
            &sent,
        );
        assert_eq!(*sync.desired, sent);
        assert!(sync.blocked.contains_key("hp"));
    }
    #[test]
    fn render_connected_controls_at_minimum_size() {
        let remote = remote();
        let mut sync = CombatSync::default();
        sync.enabled = true;
        *sync.desired.write() = remote.groups.clone();
        sync.remote = Some(remote);
        let ctx = egui::Context::default();
        let before = sync.desired.clone();
        let out = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(700.0, 600.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| sync.show(ui));
            },
        );
        assert!(!out.shapes.is_empty());
        assert_eq!(*sync.desired, before);
    }
    fn full_remote(can_edit: bool) -> Remote {
        Remote::from_raw(
            123,
            30,
            &json!({
                "id": 123, "canEdit": can_edit, "removedHitPoints": 30, "temporaryHitPoints": 0,
                "inspiration": null, "bonusHitPoints": null,
                "currencies": {"cp": 1, "sp": 2, "ep": 0, "gp": 40, "pp": null},
                "inventory": [
                    {"id": 7, "quantity": 20, "definition": {"name": "Arrows"}},
                    {"id": 8, "quantity": 1, "limitedUse": {"maxUses": 7, "numberUsed": 2}, "definition": {"name": "Wand of Magic Missiles"}}
                ],
                "creatures": [{"id": 9, "name": "", "removedHitPoints": 3, "temporaryHitPoints": null, "definition": {"name": "Wolf", "averageHitPoints": 11}}],
                "classes": [
                    {"id": 50, "level": 3, "hitDiceUsed": 1, "definition": {"name": "Fighter", "hitDice": 10}},
                    {"id": 51, "level": 2, "hitDiceUsed": null, "definition": {"name": "Wizard", "hitDice": 6}}
                ]
            }),
        )
        .unwrap()
    }
    #[test]
    fn inspiration_bonus_purse_items_companions_and_hit_dice_are_read_with_their_identities() {
        let r = full_remote(true);
        assert_eq!(r.groups["inspiration"], json!({"inspiration": false}));
        assert_eq!(r.groups["hp_bonus"], json!({"bonusHitPoints": 0}));
        assert_eq!(
            r.groups["currency"],
            json!({"pp": 0, "gp": 40, "ep": 0, "sp": 2, "cp": 1})
        );
        assert_eq!(r.groups["item/7"], json!({"id": 7, "quantity": 20}));
        assert_eq!(
            r.groups["item/8"],
            json!({"id": 8, "quantity": 1, "charges": 2})
        );
        assert_eq!(r.limits["item/8"], 7);
        assert_eq!(r.labels["item/8"], "Wand of Magic Missiles");
        assert_eq!(
            r.groups["creature/9"],
            json!({"id": 9, "removedHitPoints": 3, "temporaryHitPoints": 0})
        );
        // A companion with no name of its own has its creature's name.
        assert_eq!(r.labels["creature/9"], "Wolf");
        assert_eq!(r.limits["creature/9"], 11);
        assert_eq!(r.hit_dice.len(), 2);
        assert_eq!(
            (r.hit_dice[0].class, r.hit_dice[0].used, r.hit_dice[0].die),
            (50, 1, 10)
        );
        assert_eq!(r.hit_dice[1].used, 0);
        // A sheet without these fields has no such groups, and so offers no controls.
        let bare = remote();
        for key in ["inspiration", "hp_bonus", "currency"] {
            assert!(!bare.groups.contains_key(key));
        }
    }
    #[test]
    fn new_groups_write_exact_payloads_and_read_back_each_change() {
        let old = full_remote(true);
        let mut desired = old.groups.clone();
        desired.insert("inspiration".into(), json!({"inspiration": true}));
        desired.insert("hp_bonus".into(), json!({"bonusHitPoints": -5}));
        desired.get_mut("currency").unwrap()["gp"] = 35.into();
        desired.get_mut("currency").unwrap()["pp"] = 1.into();
        desired.insert(
            "item/8".into(),
            json!({"id": 8, "quantity": 2, "charges": 3}),
        );
        desired.insert(
            "creature/9".into(),
            json!({"id": 9, "removedHitPoints": 11, "temporaryHitPoints": 4}),
        );
        let mut fake = Fake::new(old.clone());
        let out = exchange(&mut fake, Some(&old), &desired, &BTreeSet::new(), false).unwrap();
        assert!(out.blocked.is_empty(), "{:?}", out.blocked);
        assert_eq!(out.remote.groups, desired);
        let sent: Vec<_> = fake
            .writes
            .iter()
            .map(|w| (w.route.as_str(), w.body.clone()))
            .collect();
        assert_eq!(
            sent,
            vec![
                (
                    "/creature/hp",
                    json!({"characterId": 123, "id": 9, "removedHitPoints": 11, "temporaryHitPoints": 4})
                ),
                (
                    "/inventory/currency/gold",
                    json!({"characterId": 123, "amount": 35, "destinationEntityId": 123, "destinationEntityTypeId": CHARACTER_ENTITY_TYPE})
                ),
                (
                    "/inventory/currency/platinum",
                    json!({"characterId": 123, "amount": 1, "destinationEntityId": 123, "destinationEntityTypeId": CHARACTER_ENTITY_TYPE})
                ),
                (
                    "/life/hp/bonus",
                    json!({"characterId": 123, "bonusHitPoints": -5})
                ),
                (
                    "/character/inspiration",
                    json!({"characterId": 123, "inspiration": true})
                ),
                (
                    "/inventory/item/charge",
                    json!({"characterId": 123, "id": 8, "charges": 3})
                ),
                (
                    "/inventory/item/quantity",
                    json!({"characterId": 123, "id": 8, "quantity": 2})
                ),
            ]
        );
        // Each write is read back before the next.
        assert_eq!(fake.read_count, 1 + sent.len());
    }
    #[test]
    fn new_groups_refuse_values_the_sheet_cannot_hold() {
        let r = full_remote(true);
        let wand = |quantity: i64, charges: i64| json!({"id": 8, "quantity": quantity, "charges": charges});
        assert!(mutations(&r, "item/8", &wand(0, 2)).is_err());
        assert!(
            mutations(
                &r,
                "item/8",
                &json!({"id": 99, "quantity": 1, "charges": 2})
            )
            .is_err()
        );
        // The sheet works maximums out itself, so the stat block's numbers are not limits.
        assert!(mutations(&r, "item/8", &wand(1, 8)).is_ok());
        let summon = json!({"id": 9, "removedHitPoints": 12, "temporaryHitPoints": 0});
        assert!(mutations(&r, "creature/9", &summon).is_ok());
        assert!(mutations(&r, "hp_bonus", &json!({"bonusHitPoints": 1000})).is_err());
        assert!(mutations(&r, "inspiration", &json!({"inspiration": 1})).is_err());
        assert!(
            mutations(
                &r,
                "currency",
                &json!({"pp": 0, "gp": -1, "ep": 0, "sp": 2, "cp": 1})
            )
            .is_err()
        );
        assert!(mutations(&r, "item/8", &wand(2, 7)).is_ok());
    }
    #[test]
    fn rests_and_restore_send_once_with_the_sheets_own_values() {
        let r = full_remote(true);
        let short = SheetAction::ShortRest {
            hit_dice: BTreeMap::from([(50, 2), (51, 1)]),
            reset_max_hp_bonus: false,
        };
        let mut fake = Fake::new(r.clone());
        let (out, taken) = act(&mut fake, &short).unwrap();
        assert_eq!(taken, Some(Ok(())));
        assert!(out.blocked.is_empty());
        assert_eq!(fake.read_count, 2);
        assert_eq!(fake.writes.len(), 1);
        let w = &fake.writes[0];
        assert_eq!(
            (w.method.clone(), w.route.as_str()),
            (Method::POST, "/character/rest/short")
        );
        // Hit dice used are absolute: what the sheet had plus what the DM spends.
        assert_eq!(
            w.body,
            json!({"characterId": 123, "classHitDiceUsed": {"50": 3, "51": 1}, "resetMaxHpModifier": false})
        );
        let too_many = SheetAction::ShortRest {
            hit_dice: BTreeMap::from([(50, 3)]),
            reset_max_hp_bonus: false,
        };
        let mut fake = Fake::new(r.clone());
        let (_, taken) = act(&mut fake, &too_many).unwrap();
        assert!(matches!(taken, Some(Err(_))));
        assert!(fake.writes.is_empty());

        let mut fake = Fake::new(r.clone());
        act(
            &mut fake,
            &SheetAction::LongRest {
                reset_max_hp_bonus: true,
                reduce_exhaustion: true,
            },
        )
        .unwrap();
        assert_eq!(
            fake.writes[0].body,
            json!({"characterId": 123, "resetMaxHpModifier": true, "adjustConditionLevel": true})
        );
        // Restore to life goes only while the new read still shows 0 HP.
        let mut alive = r.clone();
        alive.groups.insert(
            "hp".into(),
            json!({"removedHitPoints": 10, "temporaryHitPoints": 0}),
        );
        let mut fake = Fake::new(alive);
        let (_, taken) = act(&mut fake, &SheetAction::RestoreLife { full: false }).unwrap();
        assert!(matches!(taken, Some(Err(e)) if e.contains("no longer at 0 HP")));
        assert!(fake.writes.is_empty());
        let mut fake = Fake::new(r.clone());
        act(&mut fake, &SheetAction::RestoreLife { full: false }).unwrap();
        assert_eq!(
            (
                fake.writes[0].method.clone(),
                fake.writes[0].route.as_str(),
                fake.writes[0].body.clone()
            ),
            (
                Method::PUT,
                "/life/restore",
                json!({"characterId": 123, "totalHp": 30, "restoreType": 2})
            )
        );

        // A sheet this session cannot edit gets nothing.
        let mut fake = Fake::new(full_remote(false));
        let (_, taken) = act(&mut fake, &SheetAction::RestoreLife { full: true }).unwrap();
        assert!(matches!(taken, Some(Err(_))));
        assert!(fake.writes.is_empty());

        // An uncertain result says so, and the sheet is read again.
        let mut fake = Fake::new(r);
        fake.failure_after_apply = true;
        let (_, taken) = act(
            &mut fake,
            &SheetAction::LongRest {
                reset_max_hp_bonus: false,
                reduce_exhaustion: true,
            },
        )
        .unwrap();
        assert!(matches!(taken, Some(Err(e)) if e.contains("Check the sheet")));
        assert_eq!(fake.read_count, 2);
    }
    #[test]
    fn a_rest_waits_for_unsent_changes_and_conflicts_and_goes_only_once() {
        let r = full_remote(true);
        let mut sync = CombatSync::default();
        sync.enabled = true;
        *sync.desired.write() = r.groups.clone();
        sync.remote = Some(r.clone());
        let long = SheetAction::LongRest {
            reset_max_hp_bonus: false,
            reduce_exhaustion: true,
        };
        sync.desired
            .write()
            .insert("inspiration".into(), json!({"inspiration": true}));
        assert!(sync.request(long.clone()).is_err());
        *sync.desired.write() = r.groups.clone();
        sync.blocked.insert("hp".into(), "conflict".into());
        assert!(sync.request(long.clone()).is_err());
        sync.blocked.clear();
        assert!(sync.request(long.clone()).is_ok());
        // One is queued, so another waits.
        assert!(sync.request(long).is_err());
        // Stopping sync drops a rest that has not gone, and says nothing about it.
        sync.stop();
        assert!(sync.action.is_none() && sync.action_notice.is_none());
        // A rest on its way when sync stops has an unknown result, and the panel says so.
        let (_tx, rx) = mpsc::channel();
        sync.pending = Some(Pending {
            receiver: rx,
            sent: r.groups.clone(),
            action: Some("long rest"),
        });
        sync.stop();
        assert!(
            matches!(&sync.action_notice, Some(ActionNotice::Failed(words)) if words.contains("Check the sheet"))
        );
    }
    #[test]
    fn render_every_editable_group_at_minimum_size() {
        let remote = full_remote(true);
        let mut sync = CombatSync::default();
        sync.enabled = true;
        *sync.desired.write() = remote.groups.clone();
        sync.remote = Some(remote);
        let ctx = egui::Context::default();
        ctx.enable_accesskit();
        // Round 25 FD3: the action notice ends with its close icon, after its words.
        const TAKEN: &str = "D&D Beyond took the long rest.";
        // Round 25 review, round 26: the result sits in the sheet's footer, drawn as Party draws
        // it (the footer, then the sheet's scroll area sized to the footer's top), so the first
        // editable field keeps its y before the result, while it shows, after an error result
        // and after it ends, at 960, 1200 and 1920. The result must be in view while it lives.
        let field_top = |sync: &mut CombatSync, size: egui::Vec2| {
            let out = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let rest = ui.available_rect_before_wrap();
                        ui.scope_builder(egui::UiBuilder::new().max_rect(rest), |ui| {
                            let width = ui.available_width().min(crate::design::LIST_MEASURE);
                            let mut footer = crate::design::Footer::new("sync_test_sheet", width);
                            sync.footer_messages(&mut footer);
                            let bottom = footer.show(ui);
                            egui::ScrollArea::vertical()
                                .id_salt("sync_test_sheet_body")
                                .max_height((bottom - ui.cursor().top()).max(0.0))
                                .auto_shrink([false, false])
                                .show(ui, |ui| sync.show(ui));
                        });
                    });
                },
            );
            let notice = out.shapes.iter().find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text)
                    if text.galley.job.text.starts_with("D&D Beyond")
                        && clipped.clip_rect.contains_rect(egui::Rect::from_min_size(
                            text.pos,
                            text.galley.size(),
                        )) =>
                {
                    Some(text.pos.y)
                }
                _ => None,
            });
            let top = out.platform_output.accesskit_update.and_then(|update| {
                update.nodes.iter().find_map(|(_, node)| {
                    (node.label() == Some("Increase Current HP"))
                        .then(|| node.bounds().map(|bounds| bounds.y0))
                        .flatten()
                })
            });
            (top, notice)
        };
        for size in [
            egui::vec2(960.0, 640.0),
            egui::vec2(1200.0, 800.0),
            egui::vec2(1920.0, 1080.0),
        ] {
            let mut tops = Vec::new();
            let mut shown = Vec::new();
            for notice in [
                None,
                Some(ActionNotice::Taken(TAKEN.into())),
                Some(ActionNotice::Failed(
                    "D&D Beyond did not take the long rest. Check the sheet in D&D Beyond, then read it again before you send another change.".into(),
                )),
                None,
            ] {
                let expected = notice.is_some();
                sync.action_notice = notice;
                // Two passes: the first lays out, the second reads the settled place.
                field_top(&mut sync, size);
                let (top, notice) = field_top(&mut sync, size);
                tops.push(top);
                shown.push(notice.is_some() == expected);
            }
            assert!(
                tops[0].is_some() && tops.iter().all(|top| *top == tops[0]),
                "At {size:?}, Increase Current HP top with no result, a result, an error and none \
                 again: {tops:?}"
            );
            assert!(
                shown.iter().all(|shown| *shown),
                "At {size:?}, the result is in view exactly while it lives: {shown:?}"
            );
        }
        sync.action_notice = Some(ActionNotice::Taken(TAKEN.into()));
        let before = sync.desired.clone();
        for form in [
            None,
            Some(RestForm::Short {
                hit_dice: BTreeMap::new(),
                reset_max_hp_bonus: false,
            }),
            Some(RestForm::Long {
                reset_max_hp_bonus: false,
                reduce_exhaustion: true,
            }),
        ] {
            sync.rest_form = form;
            let out = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(700.0, 600.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        // Round 26: the result sits in the sheet's footer.
                        let mut footer = crate::design::Footer::new("sync_test_footer", 600.0);
                        sync.footer_messages(&mut footer);
                        let _ = footer.show(ui);
                        sync.show(ui);
                    });
                },
            );
            assert!(!out.shapes.is_empty());
            let words = out.shapes.iter().find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) if text.galley.job.text == TAKEN => {
                    Some(egui::Rect::from_min_size(text.pos, text.galley.size()))
                }
                _ => None,
            });
            let close = out
                .platform_output
                .accesskit_update
                .as_ref()
                .and_then(|update| {
                    update.nodes.iter().find_map(|(_, node)| {
                        (node.label() == Some("Dismiss this message")).then(|| node.bounds())
                    })
                })
                .flatten();
            assert!(
                words
                    .zip(close)
                    .is_some_and(|(words, close)| close.x0 as f32 > words.right()
                        && (close.y0 as f32) < words.top() + crate::design::L),
                "words {words:?}, close {close:?}"
            );
        }
        assert_eq!(*sync.desired, before);
    }
    #[test]
    fn a_stepper_never_changes_a_value_outside_its_range_by_itself() {
        let ctx = egui::Context::default();
        let mut value = 33;
        let mut changed = true;
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                changed = crate::design::stepper(ui, "Damage taken", &mut value, 0..=30);
            });
        });
        assert!(!changed);
        assert_eq!(value, 33);
    }
}
