//! Declarative table columns and bounded data codecs. No formulas or scripts.
use super::{local_key, reference};
use serde::Deserialize;
use serde_yaml_ng::{Mapping, Value};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Table {
    pub path: String,
    pub key: String,
    pub order: Vec<String>,
    pub columns: BTreeMap<String, Column>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Column {
    pub path: Option<String>,
    #[serde(rename = "type")]
    pub kind: Kind,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub nullable: bool,
    /// Compound fields use these names instead of fixed codec field names.
    pub fields: Option<Vec<String>>,
}
#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(super) enum Kind {
    Section,
    Key,
    String,
    Symbol,
    Integer,
    Number,
    Boolean,
    Yaml,
    Reference,
    Quantity,
    Dice,
    Range,
}

pub(super) fn field_path(path: &str) -> bool {
    !path.is_empty() && path.split('/').all(local_key)
}
impl Table {
    pub fn validate(&self) -> Result<(), String> {
        if self.order.is_empty()
            || self.order.iter().any(|key| !local_key(key))
            || self.order.iter().collect::<BTreeSet<_>>().len() != self.order.len()
        {
            return Err("Table order requires unique stable row keys".into());
        }
        if self.columns.is_empty() || !self.columns.contains_key(&self.key) {
            return Err("Table requires an identity column".into());
        }
        let key = &self.columns[&self.key];
        if key.optional
            || key.nullable
            || ![Kind::Key, Kind::Symbol, Kind::String, Kind::Reference].contains(&key.kind)
        {
            return Err("Table identity must be a required key or string column".into());
        }
        for column in self.columns.values() {
            if column
                .path
                .as_deref()
                .is_some_and(|p| !p.is_empty() && !field_path(p))
            {
                return Err("Invalid column field path".into());
            }
            if column.path.is_none() && ![Kind::Key, Kind::Section].contains(&column.kind) {
                return Err("Only row key and section columns may omit their data paths".into());
            }
            if column.kind == Kind::Section && column.path.is_some() {
                return Err("Section links cannot supply game data".into());
            }
            let expected = match column.kind {
                Kind::Range | Kind::Quantity => 2,
                Kind::Dice => 3,
                _ => 0,
            };
            if let Some(fields) = &column.fields
                && (fields.len() != expected
                    || expected == 0
                    || fields.iter().any(|f| !local_key(f))
                    || fields.iter().collect::<BTreeSet<_>>().len() != fields.len())
            {
                return Err("Invalid compound column fields".into());
            }
        }
        // Reject overlapping assignments even when all cells happen to be blank.
        let mut owned_paths = Vec::new();
        for column in self.columns.values() {
            if let Some(path) = &column.path {
                if path.is_empty() {
                    if ![Kind::Range, Kind::Quantity, Kind::Dice].contains(&column.kind) {
                        return Err("Root columns require a compound codec".into());
                    }
                    owned_paths.extend(column.names());
                } else {
                    owned_paths.push(path.clone());
                }
            }
        }
        let paths: Vec<_> = owned_paths.iter().map(String::as_str).collect();
        for (i, a) in paths.iter().enumerate() {
            if paths[i + 1..].iter().any(|b| {
                a == b || a.starts_with(&format!("{b}/")) || b.starts_with(&format!("{a}/"))
            }) {
                return Err("Overlapping table column paths".into());
            }
        }
        Ok(())
    }
}
fn yaml(raw: &str) -> Result<Value, String> {
    serde_yaml_ng::from_str(raw).map_err(|e| format!("Invalid cell: {e}"))
}
fn number(raw: &str) -> Result<Value, String> {
    let raw = raw.trim();
    if raw.contains(',') {
        let integer = raw
            .split('.')
            .next()
            .unwrap()
            .trim_start_matches(['+', '-']);
        let groups: Vec<_> = integer.split(',').collect();
        if groups[0].is_empty()
            || groups[0].len() > 3
            || groups
                .iter()
                .any(|s| !s.bytes().all(|b| b.is_ascii_digit()))
            || groups[1..].iter().any(|s| s.len() != 3)
            || raw
                .split_once('.')
                .is_some_and(|(_, tail)| tail.contains(','))
        {
            return Err("Invalid digit grouping".into());
        }
    }
    let raw = raw.replace(',', "");
    if !raw.contains(['/', '½']) {
        if let Ok(n) = raw.parse::<i64>() {
            return Ok(Value::Number(n.into()));
        }
        if let Ok(n) = raw.parse::<u64>() {
            return Ok(Value::Number(n.into()));
        }
        let n = raw.parse::<f64>().map_err(|_| "Expected a number")?;
        if !n.is_finite() {
            return Err("Number must be finite".into());
        }
        return serde_yaml_ng::to_value(n).map_err(|e| e.to_string());
    }
    let n = if let Some(head) = raw.strip_suffix('½') {
        let whole = if head.is_empty() {
            0.0
        } else {
            head.parse::<u64>().map_err(|_| "Invalid mixed fraction")? as f64
        };
        whole + 0.5
    } else if let Some((a, b)) = raw.split_once('/') {
        let a = a.parse::<u64>().map_err(|_| "Invalid numerator")?;
        let b = b.parse::<u64>().map_err(|_| "Invalid denominator")?;
        if b == 0 {
            return Err("Zero fraction denominator".into());
        }
        a as f64 / b as f64
    } else {
        return Err("Invalid fraction".into());
    };
    serde_yaml_ng::to_value(n).map_err(|e| e.to_string())
}

impl Column {
    pub fn names(&self) -> Vec<String> {
        let defaults: &[&str] = match self.kind {
            Kind::Range => &["min", "max"],
            Kind::Quantity => &["value", "unit"],
            Kind::Dice => &["count", "sides", "add"],
            _ => &[],
        };
        self.fields
            .clone()
            .unwrap_or_else(|| defaults.iter().map(|s| (*s).into()).collect())
    }
    fn compound(&self, values: Vec<Value>, defaults: &[&str]) -> Value {
        let names = self
            .fields
            .clone()
            .unwrap_or_else(|| defaults.iter().map(|s| (*s).into()).collect());
        Value::Mapping(
            names
                .into_iter()
                .zip(values)
                .map(|(name, value)| (Value::String(name), value))
                .collect::<Mapping>(),
        )
    }
    pub fn decode(&self, raw: &str, link_identity: Option<&str>) -> Result<Value, String> {
        let raw = raw.trim();
        if self.nullable && raw == "null" && link_identity.is_none() {
            return Ok(Value::Null);
        }
        match self.kind {
            Kind::Section | Kind::Key | Kind::String => {
                if raw.starts_with('"') {
                    let value = yaml(raw)?;
                    if !value.is_string() {
                        return Err("Expected quoted text".into());
                    }
                    Ok(value)
                } else {
                    Ok(Value::String(raw.into()))
                }
            }
            Kind::Symbol => Ok(Value::String(raw.to_lowercase().replace(' ', "_"))),
            Kind::Yaml => yaml(raw),
            Kind::Integer => {
                let value = yaml(raw)?;
                if value.as_i64().is_none() && value.as_u64().is_none() {
                    return Err("Expected an integer".into());
                }
                Ok(value)
            }
            Kind::Number => number(raw),
            Kind::Boolean => match raw {
                "true" => Ok(Value::Bool(true)),
                "false" => Ok(Value::Bool(false)),
                _ => Err("Expected true or false".into()),
            },
            Kind::Reference => {
                let identity = link_identity.unwrap_or(raw);
                let key = match reference(identity)? {
                    super::Reference::Id(_) => "id",
                    super::Reference::Key(_) => "key",
                };
                Ok(Value::Mapping(
                    [(Value::String(key.into()), Value::String(identity.into()))]
                        .into_iter()
                        .collect(),
                ))
            }
            Kind::Quantity => {
                let (amount, unit) = raw
                    .rsplit_once(' ')
                    .ok_or("Quantity requires a value and unit")?;
                let unit = unit.trim_end_matches('.').to_lowercase();
                if !local_key(&unit) {
                    return Err("Invalid quantity unit".into());
                }
                Ok(self.compound(
                    vec![number(amount)?, Value::String(unit)],
                    &["value", "unit"],
                ))
            }
            Kind::Dice => {
                let raw = raw.replace(' ', "");
                let (count, tail) = raw
                    .split_once('d')
                    .ok_or("Dice require NdS with optional modifier")?;
                let split = tail.find(['+', '-']).unwrap_or(tail.len());
                let count = count.parse::<u64>().map_err(|_| "Invalid dice count")?;
                let sides = tail[..split]
                    .parse::<u64>()
                    .map_err(|_| "Invalid die sides")?;
                let add = if split == tail.len() {
                    0
                } else {
                    tail[split..]
                        .parse::<i64>()
                        .map_err(|_| "Invalid dice modifier")?
                };
                if count == 0 || sides < 2 {
                    return Err("Invalid dice dimensions".into());
                }
                Ok(self.compound(
                    vec![
                        Value::Number(count.into()),
                        Value::Number(sides.into()),
                        Value::Number(add.into()),
                    ],
                    &["count", "sides", "add"],
                ))
            }
            Kind::Range => {
                let parts: Vec<_> = raw.split(['–', '-']).collect();
                if parts.len() > 2 {
                    return Err("Invalid integer range".into());
                }
                let min = parts[0]
                    .trim()
                    .parse::<u64>()
                    .map_err(|_| "Invalid range start")?;
                let max = parts
                    .last()
                    .unwrap()
                    .trim()
                    .parse::<u64>()
                    .map_err(|_| "Invalid range end")?;
                if min > max {
                    return Err("Range start exceeds end".into());
                }
                Ok(self.compound(
                    vec![Value::Number(min.into()), Value::Number(max.into())],
                    &["min", "max"],
                ))
            }
        }
    }
}
