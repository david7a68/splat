//! Changes an encounter roster slot or encounter group member builds in to a creature.
//! They belong to that slot or member alone. Nothing here writes to the creature's entry.
use super::Entry;
use serde::{Deserialize, Serialize};
use serde_json::{Value as Json, json};
use serde_yaml_ng::{Mapping, Value};
use std::collections::BTreeSet;
use uuid::Uuid;

const FIELDS: [&str; 11] = [
    "name",
    "max_hp",
    "armor_class",
    "initiative_bonus",
    "attack_bonus",
    "damage_bonus",
    "speed",
    "side",
    "conditions",
    "note",
    "loot",
];
/// The run's own limit for a maximum.
const MAX_HP: i64 = 1_000_000;
/// The run's own limit for a bonus, and a bound for the other small numbers.
const MAX_STAT: i64 = 1000;
pub const MAX_NAME: usize = 120;

/// Why the card shows these beside the stat block, and not in it.
pub const BY_HAND: &str = "Attacks and speed are printed stat block text, so a run shows these changes and cannot apply them.";
/// What a card says above a stat block, which keeps its printed numbers.
pub const PRINTED: &str =
    "The stat block below is as printed. Its numbers do not include these changes.";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Change {
    Set(i64),
    Add(i64),
}
impl Change {
    fn apply(self, base: Option<i64>) -> Option<i64> {
        match self {
            Self::Set(value) => Some(value),
            Self::Add(amount) => base.map(|base| base.saturating_add(amount)),
        }
    }
    fn to_yaml(self) -> Value {
        let (key, value) = match self {
            Self::Set(value) => ("set", value),
            Self::Add(value) => ("add", value),
        };
        let mut map = Mapping::new();
        map.insert(key.into(), value.into());
        Value::Mapping(map)
    }
}

/// What a creature's stats are before any change, kept with a run so a card can show both.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Base {
    pub name: String,
    pub side: String,
    pub max_hp: Option<i64>,
    pub armor_class: Option<i64>,
    pub initiative_bonus: Option<i64>,
    pub speed: Option<i64>,
    /// What the creature starts with before these changes add to it.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<String>,
}
/// Whether two names are one condition, whatever their case.
pub fn same_condition(a: &str, b: &str) -> bool {
    a.trim().to_lowercase() == b.trim().to_lowercase()
}
/// The names in a list of starting conditions, as a note can hold them.
pub fn condition_names(value: &Value) -> Vec<String> {
    value
        .as_sequence()
        .map(|items| {
            items
                .iter()
                .map(|v| {
                    v.as_str()
                        .or_else(|| v.get("name").and_then(Value::as_str))
                        .map(str::to_owned)
                        .unwrap_or_else(|| {
                            serde_yaml_ng::to_string(v)
                                .unwrap_or_default()
                                .trim()
                                .to_owned()
                        })
                })
                .collect()
        })
        .unwrap_or_default()
}
/// The first whole number in text such as `30 ft., Fly 80 ft.`.
fn leading_number(text: &str) -> Option<i64> {
    let digits: String = text
        .trim_start()
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    digits.parse().ok()
}
impl Base {
    pub fn of(entry: &Entry, side: &str) -> Self {
        let stat = |name: &str| {
            entry
                .fields
                .get("splat_stats")?
                .get(name)?
                .get("value")?
                .as_i64()
        };
        Self {
            name: entry.name.clone(),
            side: side.to_owned(),
            max_hp: stat("max_hp").filter(|hp| *hp > 0),
            armor_class: stat("armor_class"),
            initiative_bonus: stat("initiative"),
            // A creature can have its speed only in its text.
            speed: stat("speed")
                .or_else(|| {
                    entry
                        .fields
                        .get("splat_creature")?
                        .get("movement_text")?
                        .as_str()
                        .and_then(leading_number)
                })
                .or_else(|| {
                    entry
                        .body
                        .lines()
                        .find_map(|line| line.trim_start().strip_prefix("Speed "))
                        .and_then(leading_number)
                }),
            conditions: entry
                .fields
                .get("splat_initial")
                .and_then(|initial| initial.get("conditions"))
                .map(condition_names)
                .unwrap_or_default(),
        }
    }
}

/// The values a participant starts with, once the changes are applied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Resolved {
    pub name: String,
    pub side: String,
    pub max_hp: Option<i64>,
    pub armor_class: Option<i64>,
    pub initiative_bonus: Option<i64>,
    pub speed: Option<i64>,
}

/// One line of a card: the creature's value and the changed one. `by_hand` marks what the
/// run cannot apply, because it exists only as stat block text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub label: &'static str,
    pub base: String,
    pub changed: String,
    pub by_hand: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Overrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_hp: Option<Change>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armor_class: Option<Change>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiative_bonus: Option<Change>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_bonus: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_bonus: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<Change>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// What this one creature carries, as one line: `Iron key; Roll goblin-pockets`. It does
    /// not change the creature, so it is not a change on its row or card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot: Option<String>,
}

fn text(map: &Mapping, field: &str) -> Result<Option<String>, String> {
    let Some(value) = map.get(field) else {
        return Ok(None);
    };
    let text = value
        .as_str()
        .filter(|text| !text.trim().is_empty())
        .ok_or_else(|| format!("{field} must be text that is not empty."))?;
    Ok(Some(text.to_owned()))
}

/// A number in `{set: n}` or `{add: n}` form. `set` is `None` where a value cannot be set.
fn change(
    map: &Mapping,
    field: &str,
    set: Option<(i64, i64)>,
    limit: i64,
) -> Result<Option<Change>, String> {
    let Some(value) = map.get(field) else {
        return Ok(None);
    };
    let forms = if set.is_some() {
        "{set: n} or {add: n}"
    } else {
        "{add: n}, because each attack in a stat block has its own value"
    };
    let invalid = || format!("{field} must be {forms}.");
    let change = value
        .as_mapping()
        .filter(|m| m.len() == 1)
        .ok_or_else(invalid)?;
    let (key, amount) = change.iter().next().ok_or_else(invalid)?;
    let amount = amount.as_i64().ok_or_else(invalid)?;
    match key.as_str() {
        Some("set") => {
            let (low, high) = set.ok_or_else(invalid)?;
            if !(low..=high).contains(&amount) {
                return Err(format!("{field}: set must be from {low} to {high}."));
            }
            Ok(Some(Change::Set(amount)))
        }
        Some("add") => {
            if amount == 0 || !(-limit..=limit).contains(&amount) {
                return Err(format!(
                    "{field}: add must not be zero, and at most {limit} either way."
                ));
            }
            Ok(Some(Change::Add(amount)))
        }
        _ => Err(invalid()),
    }
}

fn signed(value: i64) -> String {
    format!("{value:+}")
}

fn shown(value: Option<i64>) -> String {
    value.map_or_else(|| "?".to_owned(), |value| value.to_string())
}

/// The change, in words, after the value it gives.
fn shown_change(change: Change, base: Option<i64>, format: impl Fn(i64) -> String) -> String {
    let value = change.apply(base).map_or_else(|| "?".to_owned(), &format);
    match change {
        Change::Set(_) => value,
        Change::Add(amount) => format!("{value} ({})", signed(amount)),
    }
}

impl Overrides {
    /// Read the block as written. A value that depends on the creature is checked by `resolve`.
    pub fn parse(map: &Mapping) -> Result<Self, String> {
        for key in map.keys() {
            let name = key.as_str().ok_or("Override names must be text.")?;
            if !FIELDS.contains(&name) {
                return Err(format!(
                    "unknown override {name}. Use one of: {}.",
                    FIELDS.join(", ")
                ));
            }
        }
        let name = text(map, "name")?;
        if name
            .as_ref()
            .is_some_and(|name| name.chars().count() > MAX_NAME || name.contains(['\n', '\r']))
        {
            return Err(format!(
                "name must be one line of at most {MAX_NAME} characters."
            ));
        }
        let bonus = |field: &str| -> Result<Option<i64>, String> {
            Ok(match change(map, field, None, MAX_STAT)? {
                Some(Change::Add(amount)) => Some(amount),
                _ => None,
            })
        };
        let mut conditions = Vec::new();
        if let Some(value) = map.get("conditions") {
            let items = value
                .as_sequence()
                .ok_or("conditions must be a list of names.")?;
            let mut seen = BTreeSet::new();
            for item in items {
                let condition = item
                    .as_str()
                    .map(str::trim)
                    .filter(|condition| !condition.is_empty())
                    .ok_or("conditions must be a list of names.")?;
                if !seen.insert(condition.to_lowercase()) {
                    return Err(format!("conditions lists {condition} twice."));
                }
                conditions.push(condition.to_owned());
            }
        }
        let loot = text(map, "loot")?;
        if loot
            .as_ref()
            .is_some_and(|loot| loot.contains(['\n', '\r']))
        {
            return Err("loot must be one line. Separate its parts with ;.".into());
        }
        Ok(Self {
            name,
            max_hp: change(map, "max_hp", Some((1, MAX_HP)), MAX_HP)?,
            armor_class: change(map, "armor_class", Some((0, MAX_STAT)), MAX_STAT)?,
            initiative_bonus: change(
                map,
                "initiative_bonus",
                Some((-MAX_STAT, MAX_STAT)),
                MAX_STAT,
            )?,
            attack_bonus: bonus("attack_bonus")?,
            damage_bonus: bonus("damage_bonus")?,
            speed: change(map, "speed", Some((0, MAX_STAT)), MAX_STAT)?,
            side: text(map, "side")?,
            conditions,
            note: text(map, "note")?,
            loot,
        })
    }

    /// The block as a note holds it. `Change` has its own form there, so this is by hand.
    pub fn to_mapping(&self) -> Mapping {
        let mut map = Mapping::new();
        if let Some(name) = &self.name {
            map.insert("name".into(), name.clone().into());
        }
        for (field, value) in [
            ("max_hp", self.max_hp),
            ("armor_class", self.armor_class),
            ("initiative_bonus", self.initiative_bonus),
            ("attack_bonus", self.attack_bonus.map(Change::Add)),
            ("damage_bonus", self.damage_bonus.map(Change::Add)),
            ("speed", self.speed),
        ] {
            if let Some(value) = value {
                map.insert(field.into(), value.to_yaml());
            }
        }
        if let Some(side) = &self.side {
            map.insert("side".into(), side.clone().into());
        }
        if !self.conditions.is_empty() {
            map.insert(
                "conditions".into(),
                Value::Sequence(self.conditions.iter().map(|c| c.clone().into()).collect()),
            );
        }
        if let Some(note) = &self.note {
            map.insert("note".into(), note.clone().into());
        }
        if let Some(loot) = &self.loot {
            map.insert("loot".into(), loot.clone().into());
        }
        map
    }

    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// Whether anything here changes the creature or its label. Loot alone does not.
    pub fn changes_creature(&self) -> bool {
        !Self {
            loot: None,
            ..self.clone()
        }
        .is_empty()
    }

    /// Each part of this creature's loot.
    pub fn loot(&self) -> Vec<super::loot::Loot> {
        self.loot
            .as_deref()
            .map(super::loot::split)
            .unwrap_or_default()
    }

    /// Apply the changes to this creature's values. Nothing here is written anywhere.
    pub fn resolve(&self, base: &Base) -> Result<Resolved, String> {
        let number = |field: &str,
                      change: Option<Change>,
                      value: Option<i64>,
                      low: i64,
                      high: i64|
         -> Result<Option<i64>, String> {
            let Some(change) = change else {
                return Ok(value);
            };
            let changed = change
                .apply(value)
                .ok_or_else(|| format!("{field} has no value to add to. Set a value instead."))?;
            if changed < low {
                return Err(format!(
                    "{field} would become {changed}. It must be {low} or more."
                ));
            }
            if changed > high {
                return Err(format!(
                    "{field} would become {changed}. It must be {high} or less."
                ));
            }
            Ok(Some(changed))
        };
        Ok(Resolved {
            name: self
                .name
                .as_deref()
                .map_or_else(|| base.name.clone(), |name| name.trim().to_owned()),
            // The same side, however it was typed: by-side initiative compares these.
            side: self.side.as_deref().unwrap_or(&base.side).trim().to_owned(),
            max_hp: number("Maximum HP", self.max_hp, base.max_hp, 1, MAX_HP)?,
            armor_class: number("AC", self.armor_class, base.armor_class, 0, MAX_STAT)?,
            initiative_bonus: number(
                "Initiative bonus",
                self.initiative_bonus,
                base.initiative_bonus,
                -MAX_STAT,
                MAX_STAT,
            )?,
            speed: number("Speed", self.speed, base.speed, 0, MAX_STAT)?,
        })
    }

    /// As `resolve`, for a creature with its entry. Sync replaces a linked character's maximum
    /// and conditions from its sheet, so a change to those could only be lost.
    pub fn resolve_for(&self, entry: &Entry, side: &str) -> Result<Resolved, String> {
        if entry.fields.contains_key("splat_external") {
            if self.max_hp.is_some() {
                return Err(format!(
                    "Maximum HP: D&D Beyond sets the maximum HP of {}.",
                    entry.name
                ));
            }
            if !self.conditions.is_empty() {
                return Err(format!(
                    "Starting conditions: D&D Beyond sets the conditions of {}.",
                    entry.name
                ));
            }
        }
        self.resolve(&Base::of(entry, side))
    }

    /// Every change, with the value it replaces.
    pub fn rows(&self, base: &Base) -> Vec<Row> {
        let plain = |value: i64| value.to_string();
        let mut rows = Vec::new();
        if let Some(name) = &self.name {
            rows.push(Row {
                label: "Name",
                base: base.name.clone(),
                changed: name.trim().to_owned(),
                by_hand: false,
            });
        }
        if let Some(change) = self.max_hp {
            rows.push(Row {
                label: "Maximum HP",
                base: shown(base.max_hp),
                changed: shown_change(change, base.max_hp, plain),
                by_hand: false,
            });
        }
        if let Some(change) = self.armor_class {
            rows.push(Row {
                label: "AC",
                base: shown(base.armor_class),
                changed: shown_change(change, base.armor_class, plain),
                by_hand: false,
            });
        }
        if let Some(change) = self.initiative_bonus {
            rows.push(Row {
                label: "Initiative bonus",
                base: base.initiative_bonus.map_or_else(|| "?".to_owned(), signed),
                changed: shown_change(change, base.initiative_bonus, signed),
                by_hand: false,
            });
        }
        if let Some(amount) = self.attack_bonus {
            rows.push(Row {
                label: "Attack bonus",
                base: "As printed".into(),
                changed: format!("{} on each attack", signed(amount)),
                by_hand: true,
            });
        }
        if let Some(amount) = self.damage_bonus {
            rows.push(Row {
                label: "Damage bonus",
                base: "As printed".into(),
                changed: format!("{} on each hit", signed(amount)),
                by_hand: true,
            });
        }
        if let Some(change) = self.speed {
            let feet = |value: i64| format!("{value} ft");
            rows.push(Row {
                label: "Speed",
                base: base.speed.map_or_else(|| "?".to_owned(), feet),
                changed: shown_change(change, base.speed, feet),
                by_hand: true,
            });
        }
        if let Some(side) = &self.side {
            rows.push(Row {
                label: "Side",
                base: base.side.clone(),
                changed: side.clone(),
                by_hand: false,
            });
        }
        if !self.conditions.is_empty() {
            rows.push(Row {
                label: "Starting conditions",
                base: if base.conditions.is_empty() {
                    "None".into()
                } else {
                    base.conditions.join(", ")
                },
                changed: {
                    let mut all = base.conditions.clone();
                    for condition in &self.conditions {
                        if !all.iter().any(|c| same_condition(c, condition)) {
                            all.push(condition.clone());
                        }
                    }
                    all.join(", ")
                },
                by_hand: false,
            });
        }
        if let Some(note) = &self.note {
            rows.push(Row {
                label: "Note",
                base: "None".into(),
                changed: note.clone(),
                by_hand: false,
            });
        }
        rows
    }

    /// What a table needs on the row, for what the run cannot apply itself.
    pub fn table_bonuses(&self, base: &Base) -> Vec<String> {
        let mut bonuses = Vec::new();
        if let Some(amount) = self.attack_bonus {
            bonuses.push(format!("Attack {}", signed(amount)));
        }
        if let Some(amount) = self.damage_bonus {
            bonuses.push(format!("Damage {}", signed(amount)));
        }
        if let Some(speed) = self.speed.and_then(|change| change.apply(base.speed)) {
            bonuses.push(format!("Speed {speed} ft"));
        }
        bonuses
    }

    /// Whether two creatures stay identical. A label or a note does not change a creature.
    pub fn mechanics(&self) -> String {
        let mechanics = Self {
            name: None,
            note: None,
            side: None,
            conditions: Vec::new(),
            loot: None,
            ..self.clone()
        };
        if mechanics.is_empty() {
            String::new()
        } else {
            serde_json::to_string(&mechanics).unwrap_or_default()
        }
    }

    fn projection(&self) -> Result<Json, String> {
        serde_json::to_value(self).map_err(|e| e.to_string())
    }
}

/// The roster component of `splat-inputs-1`, for slots as a note holds them, each with its
/// creature resolved. Any change to an override changes it, and a slot with none adds nothing.
pub fn roster_fingerprint(placements: &[(Uuid, &Mapping)]) -> Result<String, String> {
    let mut projection = Vec::new();
    for (creature, slot) in placements {
        let mut fields = (*slot).clone();
        let overrides = match fields.remove("overrides") {
            Some(value) => {
                Overrides::parse(value.as_mapping().ok_or("overrides must be a mapping.")?)?
            }
            None => Overrides::default(),
        };
        fields.remove("creature");
        let mut slot = serde_json::to_value(&fields).map_err(|e| e.to_string())?;
        slot["creature"] = json!(creature);
        if !overrides.is_empty() {
            slot["overrides"] = overrides.projection()?;
        }
        projection.push(slot);
    }
    super::contracts::fingerprint(&Json::Array(projection))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(text: &str) -> Mapping {
        serde_yaml_ng::from_str(text).unwrap()
    }
    fn base() -> Base {
        Base {
            name: "Goblin".into(),
            side: "enemies".into(),
            max_hp: Some(10),
            armor_class: Some(15),
            initiative_bonus: Some(2),
            speed: Some(30),
            conditions: Vec::new(),
        }
    }
    fn parsed(text: &str) -> Overrides {
        Overrides::parse(&map(text)).unwrap()
    }

    #[test]
    fn each_override_applies_and_none_changes_nothing() {
        let none = Overrides::default().resolve(&base()).unwrap();
        assert_eq!(none.name, "Goblin");
        assert_eq!(none.max_hp, Some(10));
        let all = parsed(
            "{name: Rival, max_hp: {add: 10}, armor_class: {set: 18}, initiative_bonus: {add: 3}, attack_bonus: {add: 1}, damage_bonus: {add: -1}, speed: {set: 40}, side: rivals, conditions: [Poisoned], note: Sealed}",
        );
        let done = all.resolve(&base()).unwrap();
        assert_eq!(
            done,
            Resolved {
                name: "Rival".into(),
                side: "rivals".into(),
                max_hp: Some(20),
                armor_class: Some(18),
                initiative_bonus: Some(5),
                speed: Some(40),
            }
        );
        assert_eq!(all.attack_bonus, Some(1));
        assert_eq!(all.damage_bonus, Some(-1));
        assert_eq!(
            all.table_bonuses(&base()),
            vec!["Attack +1", "Damage -1", "Speed 40 ft"]
        );
        assert_eq!(parsed("{max_hp: {set: 30}}").max_hp, Some(Change::Set(30)));
    }

    #[test]
    fn invalid_blocks_are_refused_in_words() {
        for (text, words) in [
            ("{hit_points: 3}", "unknown override hit_points"),
            ("{max_hp: 30}", "max_hp must be"),
            ("{max_hp: {set: 0}}", "from 1 to"),
            ("{max_hp: {add: 0}}", "not be zero"),
            ("{max_hp: {set: 3, add: 3}}", "max_hp must be"),
            ("{max_hp: {times: 3}}", "max_hp must be"),
            ("{armor_class: {set: -1}}", "from 0 to"),
            ("{attack_bonus: {set: 3}}", "each attack"),
            ("{damage_bonus: 2}", "each attack"),
            ("{speed: {set: 99999}}", "from 0 to"),
            ("{name: ''}", "name must be"),
            ("{name: \"two\\nlines\"}", "one line"),
            ("{side: ' '}", "side must be"),
            ("{conditions: Prone}", "list of names"),
            ("{conditions: [3]}", "list of names"),
            ("{conditions: [Prone, prone]}", "twice"),
            ("{note: 3}", "note must be"),
            ("{loot: \"a\\nb\"}", "one line"),
            ("{loot: ''}", "loot must be"),
        ] {
            let error = Overrides::parse(&map(text)).unwrap_err();
            assert!(error.contains(words), "{text}: {error}");
        }
    }

    #[test]
    fn iter_two_override_range_messages_name_only_the_limit_that_was_crossed() {
        assert_eq!(
            parsed("{max_hp: {add: -38}}").resolve(&base()).unwrap_err(),
            "Maximum HP would become -28. It must be 1 or more."
        );
        assert_eq!(
            parsed("{max_hp: {add: 1000000}}")
                .resolve(&base())
                .unwrap_err(),
            "Maximum HP would become 1000010. It must be 1000000 or less."
        );
        assert_eq!(
            parsed("{armor_class: {add: -16}}")
                .resolve(&base())
                .unwrap_err(),
            "AC would become -1. It must be 0 or more."
        );
        assert_eq!(
            parsed("{max_hp: {set: 1}}")
                .resolve(&base())
                .unwrap()
                .max_hp,
            Some(1)
        );
        assert_eq!(
            parsed("{max_hp: {set: 1000000}}")
                .resolve(&base())
                .unwrap()
                .max_hp,
            Some(1000000)
        );
    }

    #[test]
    fn a_change_that_leaves_a_stat_invalid_names_the_creature() {
        let error = parsed("{max_hp: {add: -10}}").resolve(&base()).unwrap_err();
        assert!(error.contains("Maximum HP would become 0"), "{error}");
        let unknown = Base {
            armor_class: None,
            ..base()
        };
        assert!(parsed("{armor_class: {add: 1}}").resolve(&unknown).is_err());
        assert_eq!(
            parsed("{armor_class: {set: 12}}")
                .resolve(&unknown)
                .unwrap()
                .armor_class,
            Some(12)
        );
    }

    #[test]
    fn a_linked_character_cannot_change_what_its_sheet_decides() {
        let linked = Entry::parse(
            "hero.md",
            &crate::content::formats::convert("Fixture.md", &format!(
                "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: Hero\nsplat_creature: {{}}\nsplat_external: {{provider: dndbeyond}}\nsplat_stats:\n  max_hp: {{value: 20}}\n---\n",
                Uuid::new_v4()
            )).expect("schema-2 fixture"),
        )
        .unwrap();
        assert!(
            parsed("{max_hp: {add: 1}}")
                .resolve_for(&linked, "party")
                .is_err()
        );
        assert!(
            parsed("{conditions: [Prone]}")
                .resolve_for(&linked, "party")
                .is_err()
        );
        assert_eq!(
            parsed("{armor_class: {set: 12}}")
                .resolve_for(&linked, "party")
                .unwrap()
                .max_hp,
            Some(20)
        );
    }

    #[test]
    fn a_block_survives_its_note_form_and_a_run_save() {
        let all = parsed(
            "{name: Rival, max_hp: {set: 10}, armor_class: {add: 1}, attack_bonus: {add: 1}, conditions: [Prone], note: Sealed}",
        );
        assert_eq!(Overrides::parse(&all.to_mapping()).unwrap(), all);
        let saved = serde_json::to_string(&all).unwrap();
        assert_eq!(serde_json::from_str::<Overrides>(&saved).unwrap(), all);
        assert_eq!(serde_json::to_string(&Overrides::default()).unwrap(), "{}");
    }

    #[test]
    fn a_card_shows_each_base_and_changed_value() {
        let rows =
            parsed("{max_hp: {add: 10}, attack_bonus: {add: 1}, speed: {add: 10}, side: rivals}")
                .rows(&base());
        let row = |label: &str| rows.iter().find(|row| row.label == label).unwrap();
        assert_eq!(row("Maximum HP").base, "10");
        assert_eq!(row("Maximum HP").changed, "20 (+10)");
        assert!(!row("Maximum HP").by_hand);
        assert_eq!(row("Attack bonus").changed, "+1 on each attack");
        assert!(row("Attack bonus").by_hand);
        assert_eq!(row("Speed").changed, "40 ft (+10)");
        assert!(row("Speed").by_hand);
        assert_eq!(row("Side").base, "enemies");
    }

    #[test]
    fn a_card_shows_what_conditions_a_creature_starts_with_and_ends_with() {
        let mut poisoned = base();
        poisoned.conditions = vec!["Poisoned".into()];
        let rows = parsed("{conditions: [Prone, poisoned]}").rows(&poisoned);
        assert_eq!(rows[0].base, "Poisoned");
        assert_eq!(rows[0].changed, "Poisoned, Prone");
        assert_eq!(
            parsed("{conditions: [Prone]}").rows(&base())[0].base,
            "None"
        );
    }

    #[test]
    fn a_speed_only_in_text_is_still_a_base_value() {
        let creature = |extra: &str, body: &str| {
            Entry::parse(
                "c.md",
                &crate::content::formats::convert("Fixture.md", &format!(
                    "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: creature\nsplat_name: C\nsplat_creature: {{{extra}}}\n---\n\n{body}\n",
                    Uuid::new_v4()
                )).expect("schema-2 fixture"),
            )
            .unwrap()
        };
        let speed = |entry: Entry| Base::of(&entry, "enemies").speed;
        assert_eq!(
            speed(creature("movement_text: 40 ft., Fly 80 ft.", "")),
            Some(40)
        );
        assert_eq!(speed(creature("", "Speed 30 ft.")), Some(30));
        assert_eq!(speed(creature("", "No such line.")), None);
    }

    #[test]
    fn identical_creatures_differ_by_mechanics_and_not_by_labels() {
        assert_eq!(Overrides::default().mechanics(), "");
        assert_eq!(parsed("{name: Rival, note: Sealed}").mechanics(), "");
        assert_ne!(parsed("{max_hp: {add: 1}}").mechanics(), "");
    }

    #[test]
    fn the_fingerprint_follows_every_override_and_ignores_their_absence() {
        let creature = Uuid::new_v4();
        let digest = |text: &str| {
            let slot = map(text);
            roster_fingerprint(&[(creature, &slot)]).unwrap()
        };
        let plain = digest("{key: a, side: enemies, creature: {id: x}}");
        assert_eq!(
            plain,
            digest("{key: a, side: enemies, creature: {id: y}, overrides: {}}"),
            "a slot with no overrides keeps the digest it had"
        );
        let mut seen = BTreeSet::from([plain]);
        for changed in [
            "{name: Rival}",
            "{name: Other}",
            "{max_hp: {add: 1}}",
            "{max_hp: {set: 1}}",
            "{armor_class: {set: 12}}",
            "{initiative_bonus: {add: 1}}",
            "{attack_bonus: {add: 1}}",
            "{damage_bonus: {add: 1}}",
            "{speed: {set: 10}}",
            "{side: rivals}",
            "{conditions: [Prone]}",
            "{note: Sealed}",
        ] {
            let text = format!("{{key: a, side: enemies, overrides: {changed}}}");
            assert!(seen.insert(digest(&text)), "{changed} changed nothing");
        }
    }
}
