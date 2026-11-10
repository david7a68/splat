//! Optional reading text, independent of mechanical rules and dependencies.
use super::{Entry, reference};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reading {
    pub title: String,
    pub summary: String,
    #[serde(default = "related_heading")]
    pub related_heading: String,
    /// Resource keys or UUIDs mapped to the labels readers see.
    #[serde(default)]
    pub related: BTreeMap<String, String>,
}

fn related_heading() -> String {
    "Related entries".into()
}

impl Entry {
    pub fn reading(&self) -> Result<Option<Reading>, String> {
        let Some(value) = self.fields.get("splat_reading") else {
            return Ok(None);
        };
        let reading = Reading::deserialize(value)
            .map_err(|error| format!("Invalid splat_reading: {error}"))?;
        if [&reading.title, &reading.summary, &reading.related_heading]
            .iter()
            .any(|text| text.trim().is_empty())
        {
            return Err("Reading title, summary, and related heading must not be empty".into());
        }
        for (target, label) in &reading.related {
            reference(target)?;
            if label.trim().is_empty() {
                return Err("Reading link label must not be empty".into());
            }
        }
        Ok(Some(reading))
    }
}
