//! Data-only contracts for unavailable SRD adapters. Declarations cannot execute.
use super::{Entry, field, resource_key};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Input {
    #[serde(rename = "type")]
    pub kind: String,
    pub required: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Interface {
    pub id: String,
    pub inputs: BTreeMap<String, Input>,
    pub outputs: Vec<String>,
    pub events: Vec<String>,
    pub capabilities: Vec<String>,
}

impl Interface {
    pub fn validate(&self) -> Result<(), String> {
        if !resource_key(&self.id) || self.capabilities.is_empty() {
            return Err("Contract requires an ID and engine capabilities".into());
        }
        const TYPES: &[&str] = &[
            "participant",
            "participant_list",
            "item_instance",
            "item_list",
            "object",
            "object_list",
            "location",
            "location_list",
            "integer",
            "number",
            "boolean",
            "choice",
            "effect",
            "resolution",
            "event",
            "campaign",
            "duration",
            "definition",
            "definition_list",
            "text",
        ];
        for (name, input) in &self.inputs {
            if !super::local_key(name) || !TYPES.contains(&input.kind.as_str()) {
                return Err(format!("Invalid contract input {name}: {}", input.kind));
            }
        }
        if self
            .outputs
            .iter()
            .chain(&self.events)
            .chain(&self.capabilities)
            .any(|v| v.trim().is_empty())
        {
            return Err("Empty contract interface name".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Shape {
    Null,
    Boolean,
    Integer,
    Number,
    String,
    Array { items: Vec<Shape> },
    Object { properties: BTreeMap<String, Shape> },
}

impl Shape {
    pub fn validate(&self, value: &Json, path: &str) -> Result<(), String> {
        match self {
            Self::Null if value.is_null() => Ok(()),
            Self::Boolean if value.is_boolean() => Ok(()),
            Self::Integer if value.is_i64() || value.is_u64() => Ok(()),
            Self::Number if value.is_number() => Ok(()),
            Self::String if value.is_string() => Ok(()),
            Self::Array { items } => {
                let values = value
                    .as_array()
                    .ok_or_else(|| format!("{path}: expected array"))?;
                for (index, value) in values.iter().enumerate() {
                    if !items
                        .iter()
                        .any(|shape| shape.validate(value, path).is_ok())
                    {
                        return Err(format!("{path}/{index}: no matching contract shape"));
                    }
                }
                Ok(())
            }
            Self::Object { properties } => {
                let values = value
                    .as_object()
                    .ok_or_else(|| format!("{path}: expected object"))?;
                if properties.len() != values.len()
                    || properties.keys().any(|key| !values.contains_key(key))
                {
                    return Err(format!("{path}: fields differ from the contract"));
                }
                for (key, shape) in properties {
                    shape.validate(&values[key], &format!("{path}/{key}"))?;
                }
                Ok(())
            }
            _ => Err(format!("{path}: wrong parameter type")),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Registry {
    pub format: u32,
    pub contracts: Vec<RegisteredContract>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegisteredContract {
    pub id: String,
    pub inputs: BTreeMap<String, Input>,
    pub outputs: Vec<String>,
    pub events: Vec<String>,
    pub capabilities: Vec<String>,
    pub definition: String,
    pub source_sha256: String,
    pub research_path: String,
    pub citations: Vec<String>,
    pub implemented: bool,
    pub specification_sha256: String,
    pub schema: Shape,
}

impl RegisteredContract {
    pub fn interface(&self) -> Interface {
        Interface {
            id: self.id.clone(),
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
            events: self.events.clone(),
            capabilities: self.capabilities.clone(),
        }
    }

    pub fn validate(&self, entry: &Entry) -> Result<(), String> {
        self.interface().validate()?;
        if self.implemented {
            return Err("A content contract cannot register an engine implementation".into());
        }
        if entry.key.as_deref() != Some(&self.definition) || self.citations.is_empty() {
            return Err("Contract definition or citations are invalid".into());
        }
        let rule = field(&entry.fields, "splat_rule").and_then(serde_yaml_ng::Value::as_mapping);
        if rule
            .and_then(|rule| field(rule, "implementation"))
            .and_then(serde_yaml_ng::Value::as_str)
            != Some(self.id.as_str())
        {
            return Err(format!(
                "Rule implementation differs from its contract: {}",
                self.definition
            ));
        }
        let fields = specification(entry)?;
        self.schema.validate(&fields, &self.definition)?;
        if fingerprint(&fields)? != self.specification_sha256 {
            return Err(format!(
                "Contract specification mismatch: {}",
                self.definition
            ));
        }
        let declarations = declarations(entry)?;
        if !declarations.contains(&self.interface()) {
            return Err(format!("Missing contract declaration: {}", self.id));
        }
        Ok(())
    }
}

pub fn declarations(entry: &Entry) -> Result<Vec<Interface>, String> {
    let Some(value) = field(&entry.fields, "splat_contracts") else {
        return Ok(vec![]);
    };
    let interfaces: Vec<Interface> =
        serde_yaml_ng::from_value(value.clone()).map_err(|e| e.to_string())?;
    let mut seen = std::collections::BTreeSet::new();
    for interface in &interfaces {
        interface.validate()?;
        if !seen.insert(&interface.id) {
            return Err("Duplicate contract declaration".into());
        }
    }
    Ok(interfaces)
}

pub fn specification(entry: &Entry) -> Result<Json, String> {
    let mut fields = serde_json::Map::new();
    for (key, value) in &entry.fields {
        let key = key.as_str().ok_or("Schema field name must be a string")?;
        if key.starts_with("splat_")
            && ![
                "splat_schema",
                "splat_id",
                "splat_kind",
                "splat_key",
                "splat_name",
                "splat_origin",
                "splat_sources",
                "splat_conversion",
                "splat_clauses",
                "splat_requires_mechanics",
                "splat_contracts",
            ]
            .contains(&key)
        {
            fields.insert(
                key.into(),
                serde_json::to_value(value).map_err(|e| e.to_string())?,
            );
        }
    }
    Ok(Json::Object(fields))
}

pub fn fingerprint(value: &Json) -> Result<String, String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn closed_contract_shapes_reject_unknown_fields_and_wrong_types() {
        let schema: Shape = serde_json::from_value(json!({
            "type": "object", "properties": {"charges": {"type": "integer"}, "consumed": {"type": "boolean"}}
        })).unwrap();
        assert!(
            schema
                .validate(&json!({"charges": 7, "consumed": false}), "wand")
                .is_ok()
        );
        assert!(
            schema
                .validate(&json!({"charges": "7", "consumed": false}), "wand")
                .is_err()
        );
        assert!(
            schema
                .validate(
                    &json!({"charges": 7, "consumed": false, "ignore_failure": true}),
                    "wand"
                )
                .is_err()
        );
        assert!(schema.validate(&json!({"charges": 7}), "wand").is_err());
    }

    #[test]
    fn fingerprint_is_stable_across_mapping_order() {
        assert_eq!(
            fingerprint(&json!({"b": 2, "a": "é"})).unwrap(),
            fingerprint(&json!({"a": "é", "b": 2})).unwrap()
        );
    }
}
