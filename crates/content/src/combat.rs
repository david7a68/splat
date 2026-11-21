//! The note fields shared by character sheets and the rules interpreter.
use serde::{Deserialize, Serialize};

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
