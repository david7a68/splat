//! Read-only schema reader. Parsing/reference validation never implies execution support.
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_yaml_ng::{Mapping, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

/// Action shown for a retired on-disk layout. Loading never changes the file.
pub const OLD_LAYOUT: &str = "Old layout: Splat no longer reads this file. Use Convert files.";

/// A note's line that makes the table below it rollable. It holds no entry data.
pub const ROLLS_MARKER: &str = "<!-- splat:rolls ";

/// True if a note's text has a marker that holds entry data.
pub fn has_data_marker(text: &str) -> bool {
    text.match_indices("<!-- splat:")
        .any(|(at, _)| !text[at..].starts_with(ROLLS_MARKER))
}

pub mod adapters;
pub mod art;
pub mod builtin;
pub mod character;
pub mod contracts;
pub mod creature_filter;
pub mod definitions;
pub mod encounter_group;
pub mod overrides;
mod reading;
pub use reading::Reading;
pub mod formats;
pub mod layout;
pub mod legacy;
pub mod list_formats;
mod markdown;
pub use markdown::{body_text, inline_text};
mod shared;
pub mod stat_blocks;
pub use shared::validate_with_builtin;
mod table_mapping;

#[cfg(test)]
mod formats_tests;
#[cfg(test)]
mod retirement_tests;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "EntryWire")]
pub struct Entry {
    pub path: String,
    pub id: Uuid,
    pub key: Option<String>,
    pub name: String,
    pub kind: String,
    pub fields: Fields,
    pub body: String,
    /// Physical source line of the body, for flags resolved after all notes load.
    #[serde(skip)]
    pub body_line: usize,
    pub inventory: Vec<InventoryRow>,
    pub inventory_containers: Vec<String>,
    #[serde(default)]
    pub flags: Vec<layout::Flag>,
}

// Saved runs contain parsed entry copies. They obey the same schema boundary as notes.
#[derive(Deserialize)]
struct EntryWire {
    path: String,
    id: Uuid,
    key: Option<String>,
    name: String,
    kind: String,
    fields: Fields,
    body: String,
    inventory: Vec<InventoryRow>,
    inventory_containers: Vec<String>,
    #[serde(default)]
    flags: Vec<layout::Flag>,
}
impl TryFrom<EntryWire> for Entry {
    type Error = String;
    fn try_from(wire: EntryWire) -> Result<Self, String> {
        match wire.fields.get("splat_schema").and_then(Value::as_u64) {
            Some(2) => {}
            Some(1) => return Err(OLD_LAYOUT.into()),
            _ => return Err("Unsupported content schema; expected 2".into()),
        }
        let entry = Self {
            path: wire.path,
            id: wire.id,
            key: wire.key,
            name: wire.name,
            kind: wire.kind,
            fields: wire.fields,
            body: wire.body,
            body_line: 1,
            inventory: wire.inventory,
            inventory_containers: wire.inventory_containers,
            flags: wire.flags,
        };
        entry.validate_shape()?;
        Ok(entry)
    }
}

/// Parsed fields are read repeatedly by independent catalogues and app views.
/// Share their nested YAML values until a caller edits its own entry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Fields(Arc<Mapping>);

impl From<Mapping> for Fields {
    fn from(fields: Mapping) -> Self {
        Self(Arc::new(fields))
    }
}

impl std::ops::Deref for Fields {
    type Target = Mapping;

    fn deref(&self) -> &Mapping {
        &self.0
    }
}

impl std::ops::DerefMut for Fields {
    fn deref_mut(&mut self) -> &mut Mapping {
        Arc::make_mut(&mut self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryRow {
    pub key: String,
    pub item: Reference,
    pub quantity: u64,
    pub equipment: String,
    pub attuned: bool,
    pub container: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Reference {
    Id(Uuid),
    Key(String),
}
impl std::fmt::Display for Reference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Id(id) => id.fmt(f),
            Self::Key(key) => key.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub path: String,
    pub message: String,
}

#[derive(Debug, Default, Serialize)]
pub struct Report {
    pub entries: usize,
    pub reference_only: usize,
    pub executable_entries: usize,
    pub errors: Vec<Diagnostic>,
    pub warnings: Vec<Diagnostic>,
    pub limitations: Vec<String>,
}

fn field<'a>(map: &'a Mapping, name: &str) -> Option<&'a Value> {
    map.get(Value::String(name.into()))
}
fn string<'a>(map: &'a Mapping, name: &str) -> Result<&'a str, String> {
    field(map, name)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{name} must be a string"))
}
fn local_key(value: &str) -> bool {
    value.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
        && value
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || b"_-".contains(&c))
}
pub fn resource_key(value: &str) -> bool {
    let Some((ns, tail)) = value.split_once(':') else {
        return false;
    };
    let valid = |s: &str| {
        !s.is_empty()
            && s.bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || b"_-".contains(&c))
    };
    valid(ns) && tail.split('/').all(valid)
}
fn reference(value: &str) -> Result<Reference, String> {
    if let Ok(id) = Uuid::parse_str(value) {
        Ok(Reference::Id(id))
    } else if resource_key(value) {
        Ok(Reference::Key(value.into()))
    } else {
        Err(format!("Invalid entry reference: {value}"))
    }
}

impl Entry {
    pub fn parse(path: &str, text: &str) -> Result<Self, String> {
        let text = text
            .strip_prefix('\u{feff}')
            .unwrap_or(text)
            .replace("\r\n", "\n");
        let rest = text
            .strip_prefix("---\n")
            .ok_or("Missing YAML frontmatter")?;
        let end = rest
            .find("\n---\n")
            .or_else(|| rest.strip_suffix("\n---").map(str::len))
            .ok_or("Unclosed YAML frontmatter")?;
        let value: Value =
            serde_yaml_ng::from_str(&rest[..end]).map_err(|e| format!("Invalid YAML: {e}"))?;
        let Value::Mapping(mut fields) = value else {
            return Err("Frontmatter must be a mapping".into());
        };
        match fields.get("splat_schema").and_then(Value::as_u64) {
            Some(2) => {}
            Some(1) => return Err(OLD_LAYOUT.into()),
            _ => return Err("Unsupported content schema; expected 2".into()),
        }
        let id =
            Uuid::parse_str(string(&fields, "splat_id")?).map_err(|_| "Invalid splat_id UUID")?;
        let name = string(&fields, "splat_name")?.to_owned();
        if name.trim().is_empty() {
            return Err("Empty splat_name".into());
        }
        let kind = string(&fields, "splat_kind")?.to_owned();
        if ![
            "package",
            "campaign",
            "creature",
            "item",
            "spell",
            "effect",
            "container",
            "encounter",
            "encounter_group",
            "world",
            "sequence",
            "rule",
            "rule_group",
            "strategy",
            "seed",
            "reference",
        ]
        .contains(&kind.as_str())
        {
            return Err(format!("Unknown kind {kind}"));
        }
        let key = match field(&fields, "splat_key") {
            Some(v) => {
                let k = v.as_str().ok_or("splat_key must be a string")?;
                if !resource_key(k) {
                    return Err("Invalid splat_key".into());
                }
                Some(k.into())
            }
            None => None,
        };
        let body = rest.get(end + 5..).unwrap_or("").to_owned();
        let tables = stat_blocks::owns(
            &kind,
            fields.contains_key("splat_character") || body.contains("path=splat_payload"),
        );
        if tables {
            markdown::read(&stat_blocks::without_legacy(&body, &kind), &mut fields)?;
        } else {
            markdown::read(&body, &mut fields)?;
        }
        let flags = formats::read(&text, &mut fields);
        if kind == "creature" && !fields.contains_key("splat_character") {
            stat_blocks::complete_challenge_fields(&mut fields);
        }
        let (inventory, inventory_containers) = inventory(&body)?;
        let entry = Self {
            path: path.into(),
            id,
            key,
            name,
            kind,
            fields: fields.into(),
            body_line: text[..text.len() - body.len()]
                .bytes()
                .filter(|b| *b == b'\n')
                .count()
                + 1,
            body,
            inventory,
            inventory_containers,
            flags,
        };
        entry.validate_shape()?;
        entry.reading()?;
        character::validate(&entry)?;
        definitions::conversion(&entry)?;
        Ok(entry)
    }
    pub fn reference_metadata(&self) -> Option<&Mapping> {
        field(&self.fields, "splat_reference").and_then(Value::as_mapping)
    }
    pub fn category(&self) -> &str {
        self.reference_metadata()
            .and_then(|m| field(m, "category"))
            .and_then(Value::as_str)
            .unwrap_or(&self.kind)
    }
    fn validate_shape(&self) -> Result<(), String> {
        let required = match self.kind.as_str() {
            "package" | "campaign" => Some("splat_namespace"),
            "creature" => Some("splat_creature"),
            "item" | "container" => Some("splat_item"),
            "spell" => Some("splat_spell"),
            "effect" => Some("splat_effect"),
            "encounter" => Some("splat_encounter"),
            "encounter_group" => Some("splat_encounter_group"),
            "world" => Some("splat_world"),
            "sequence" => Some("splat_sequence"),
            "rule" => Some("splat_rule"),
            "rule_group" => Some("splat_rule_group"),
            "strategy" => Some("splat_strategy"),
            "seed" => Some("splat_seed"),
            "reference" => Some("splat_reference"),
            _ => None,
        };
        if let Some(name) = required {
            let value = field(&self.fields, name).ok_or_else(|| format!("Missing {name}"))?;
            if name == "splat_namespace" {
                let ns = value.as_str().ok_or("splat_namespace must be a string")?;
                if !resource_key(&format!("{ns}:entry")) {
                    return Err("Invalid namespace".into());
                }
            } else if !value.is_mapping() {
                return Err(format!("{name} must be a mapping"));
            }
        }
        if self.kind == "encounter"
            && let Some(roster) = field(&self.fields, "splat_encounter")
                .and_then(|encounter| encounter.get("roster"))
                .and_then(Value::as_sequence)
        {
            for slot in roster {
                let Some(block) = slot.get("overrides") else {
                    continue;
                };
                let key = slot.get("key").and_then(Value::as_str).unwrap_or("A slot");
                let block = block
                    .as_mapping()
                    .ok_or_else(|| format!("{key}: overrides must be a mapping."))?;
                overrides::Overrides::parse(block).map_err(|e| format!("{key}: {e}"))?;
            }
        }
        if self.kind == "reference" {
            let meta = self
                .reference_metadata()
                .ok_or("splat_reference must be a mapping")?;
            string(meta, "source")?;
            string(meta, "category")?;
            if field(meta, "executable").and_then(Value::as_bool) != Some(false) {
                return Err("Reference notes must explicitly be non-executable".into());
            }
        }
        for row in &self.inventory {
            let inside_container = row.container.is_some() || self.kind == "container";
            if inside_container && row.equipment != "stored" {
                return Err(format!(
                    "Contained inventory row {} must be stored",
                    row.key
                ));
            }
            if self.kind == "creature" && !inside_container && row.equipment == "stored" {
                return Err(format!(
                    "Root creature inventory row {} cannot be stored",
                    row.key
                ));
            }
        }
        if let Some(stats) = field(&self.fields, "splat_stats") {
            for (key, value) in stats.as_mapping().ok_or("splat_stats must be a mapping")? {
                let stat = value
                    .as_mapping()
                    .ok_or("Stat definition must be a mapping")?;
                if field(stat, "value").is_some() == field(stat, "derive").is_some() {
                    return Err(format!("Stat {key:?} requires exactly one of value/derive"));
                }
            }
        }
        if let Some(resources) = field(&self.fields, "splat_resources") {
            for (key, value) in resources
                .as_mapping()
                .ok_or("splat_resources must be a mapping")?
            {
                let key = key.as_str().ok_or("Pool key must be string")?;
                if !local_key(key) {
                    return Err(format!("Invalid pool key {key}"));
                }
                let pool = value.as_mapping().ok_or("Pool must be a mapping")?;
                let max = field(pool, "max").ok_or("Pool requires max")?;
                if !max.is_mapping() && max.as_u64().is_none() {
                    return Err(format!("Invalid pool maximum {key}"));
                }
                if let Some(max) = field(pool, "max").and_then(Value::as_u64)
                    && let Some(initial) = field(pool, "initial")
                    && initial.as_u64().is_none_or(|i| i > max)
                {
                    return Err(format!("Invalid initial pool amount {key}"));
                }
            }
        }
        for name in ["splat_actions", "splat_effects", "splat_budgets"] {
            if let Some(value) = field(&self.fields, name) {
                for key in value
                    .as_mapping()
                    .ok_or_else(|| format!("{name} must be a mapping"))?
                    .keys()
                {
                    if !key.as_str().is_some_and(local_key) {
                        return Err(format!("Invalid local key in {name}"));
                    }
                }
            }
        }
        Ok(())
    }
}

fn inventory(body: &str) -> Result<(Vec<InventoryRow>, Vec<String>), String> {
    // The parser only recognizes this literal marker. Avoid a second Markdown
    // pass for notes without one; code examples containing it still go through
    // the parser so they cannot become inventory data.
    if !body.contains("<!-- splat:inventory") {
        return Ok((Vec::new(), Vec::new()));
    }
    let mut result = Vec::new();
    let mut keys = BTreeSet::new();
    let mut targets = BTreeSet::new();
    let mut pending: Option<Option<String>> = None;
    let mut current: Option<Option<String>> = None;
    let mut headers = Vec::<String>::new();
    let mut cells = Vec::<String>::new();
    let mut cell = String::new();
    let mut in_head = false;
    let mut in_cell = false;
    for event in Parser::new_ext(body, Options::ENABLE_TABLES) {
        match event {
            Event::Html(ref s) | Event::InlineHtml(ref s)
                if s.trim().starts_with("<!-- splat:inventory") =>
            {
                if pending.is_some() {
                    return Err("Inventory marker without table".into());
                }
                let marker = s.trim();
                let target = if marker == "<!-- splat:inventory -->" {
                    None
                } else {
                    let key = marker
                        .strip_prefix("<!-- splat:inventory container=")
                        .and_then(|s| s.strip_suffix(" -->"))
                        .ok_or("Invalid inventory marker")?;
                    if !local_key(key) {
                        return Err("Invalid container row key".into());
                    }
                    Some(key.to_owned())
                };
                if !targets.insert(target.clone()) {
                    return Err("Duplicate inventory table target".into());
                }
                pending = Some(target);
            }
            Event::Start(Tag::Table(_)) => {
                current = pending.take();
                headers.clear();
            }
            Event::Start(Tag::TableHead) if current.is_some() => {
                in_head = true;
                cells.clear();
            }
            Event::Start(Tag::TableRow) if current.is_some() => {
                cells.clear();
            }
            Event::Start(Tag::TableCell) if current.is_some() => {
                in_cell = true;
                cell.clear();
            }
            Event::Text(s) | Event::Code(s) if in_cell => cell.push_str(&s),
            Event::End(TagEnd::TableCell) if current.is_some() => {
                cells.push(cell.trim().to_owned());
                in_cell = false;
            }
            Event::End(TagEnd::TableHead) if current.is_some() => {
                headers = cells.clone();
                in_head = false;
                if headers.iter().collect::<BTreeSet<_>>().len() != headers.len() {
                    return Err("Duplicate inventory column".into());
                }
                for name in ["Key", "Item", "Quantity", "Equipment"] {
                    if !headers.iter().any(|h| h == name) {
                        return Err(format!("Missing inventory column {name}"));
                    }
                }
                if headers.iter().any(|h| {
                    !["Key", "Item", "Quantity", "Equipment", "Attuned", "Link"]
                        .contains(&h.as_str())
                }) {
                    return Err("Unknown inventory column".into());
                }
            }
            Event::End(TagEnd::TableRow) if current.is_some() && !in_head => {
                let get = |name: &str| {
                    headers
                        .iter()
                        .position(|h| h == name)
                        .and_then(|i| cells.get(i))
                        .map(String::as_str)
                        .unwrap_or("")
                };
                let key = get("Key");
                if !local_key(key) || !keys.insert(key.to_owned()) {
                    return Err(format!("Invalid or duplicate inventory key {key}"));
                }
                let quantity = get("Quantity")
                    .parse::<u64>()
                    .ok()
                    .filter(|q| *q > 0)
                    .ok_or("Quantity must be a positive integer")?;
                let equipment = get("Equipment");
                if !["carried", "worn", "wielded", "stored"].contains(&equipment) {
                    return Err("Invalid equipment state".into());
                }
                let attuned = match get("Attuned") {
                    "" | "false" => false,
                    "true" => true,
                    _ => return Err("Attuned must be true or false".into()),
                };
                result.push(InventoryRow {
                    key: key.into(),
                    item: reference(get("Item"))?,
                    quantity,
                    equipment: equipment.into(),
                    attuned,
                    container: current.clone().flatten(),
                });
            }
            Event::End(TagEnd::Table) => {
                current = None;
            }
            Event::Start(_) if pending.is_some() => {
                return Err("Inventory marker must immediately precede a table".into());
            }
            _ => (),
        }
    }
    if pending.is_some() {
        return Err("Inventory marker without table".into());
    }
    let containers: Vec<_> = targets.into_iter().flatten().collect();
    for key in &containers {
        if !keys.contains(key) {
            return Err(format!("Missing container row {key}"));
        }
    }
    for row in &result {
        let mut seen = BTreeSet::new();
        let mut parent = row.container.as_deref();
        while let Some(key) = parent {
            if key == row.key || !seen.insert(key) {
                return Err("Inventory containment cycle".into());
            }
            parent = result
                .iter()
                .find(|r| r.key == key)
                .ok_or_else(|| format!("Missing container row {key}"))?
                .container
                .as_deref();
        }
    }
    Ok((result, containers))
}

fn references(value: &Value, out: &mut Vec<Reference>) -> Result<(), String> {
    match value {
        Value::Mapping(map) => {
            if !map.is_empty()
                && map.keys().all(|k| {
                    k.as_str()
                        .is_some_and(|s| ["id", "key", "link"].contains(&s))
                })
            {
                let id = field(map, "id");
                let key = field(map, "key");
                if id.is_some() && key.is_some() {
                    return Err("Reference has both id and key".into());
                }
                if let Some(id) = id {
                    out.push(Reference::Id(
                        Uuid::parse_str(id.as_str().ok_or("Reference id must be string")?)
                            .map_err(|_| "Invalid reference UUID")?,
                    ));
                } else if let Some(key) = key {
                    out.push(reference(
                        key.as_str().ok_or("Reference key must be string")?,
                    )?);
                } else {
                    return Err("Reference requires id or key".into());
                }
                if let Some(link) = field(map, "link")
                    && !link.is_string()
                {
                    return Err("Reference link must be string".into());
                }
            }
            for v in map.values() {
                references(v, out)?
            }
        }
        Value::Sequence(seq) => {
            for v in seq {
                references(v, out)?
            }
        }
        _ => (),
    }
    Ok(())
}

/// Required content references, excluding provenance and optional selections.
/// Engine contract names are not content entry references.
pub fn required_references(entry: &Entry) -> Result<Vec<Reference>, String> {
    if let Some(raw) = entry
        .fields
        .get("splat_requires")
        .and_then(Value::as_sequence)
        .and_then(|values| values.iter().find_map(Value::as_str))
    {
        return Err(format!("Requirement {raw} needs a catalogue match"));
    }
    let fields: Mapping = entry
        .fields
        .iter()
        .filter(|(key, _)| {
            key.as_str().is_some_and(|key| {
                key.starts_with("splat_")
                    && ![
                        "splat_optional",
                        "splat_origin",
                        "splat_sources",
                        "splat_reading",
                        "splat_conversion",
                        "splat_contracts",
                        "splat_packages",
                    ]
                    .contains(&key)
            })
        })
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let mut result = Vec::new();
    references(&Value::Mapping(fields), &mut result)?;
    result.extend(entry.inventory.iter().map(|r| r.item.clone()));
    Ok(result)
}

pub fn validate(entries: &[Entry]) -> Report {
    validate_all(&entries.iter().collect::<Vec<_>>())
}

/// [`validate`] for entries that are not held in one slice, such as the Library's notes.
pub fn validate_all(entries: &[&Entry]) -> Report {
    let mut report = Report {
        entries: entries.len(),
        reference_only: entries.iter().filter(|e| e.kind == "reference").count(),
        ..Report::default()
    };
    let mut ids = BTreeMap::new();
    let mut keys = BTreeMap::new();
    let mut namespaces = BTreeSet::new();
    let mut error = |path: &str, message: String| {
        report.errors.push(Diagnostic {
            path: path.into(),
            message,
        })
    };
    for &e in entries {
        if ids.insert(e.id, e).is_some() {
            error(&e.path, format!("Duplicate UUID {}", e.id))
        }
        if let Some(key) = &e.key
            && keys.insert(key.clone(), e).is_some()
        {
            error(&e.path, format!("Duplicate key {key}"))
        }
        if ["package", "campaign"].contains(&e.kind.as_str()) {
            match string(&e.fields, "splat_namespace") {
                Ok(ns) => {
                    if !namespaces.insert(ns) {
                        error(&e.path, format!("Duplicate namespace {ns}"))
                    }
                }
                Err(msg) => error(&e.path, msg),
            }
        }
    }
    let resolve = |r: &Reference| match r {
        Reference::Id(id) => ids.get(id).copied(),
        Reference::Key(k) => keys.get(k).copied(),
    };
    for &e in entries {
        let mut refs = Vec::new();
        let mut required_fields: Mapping = e
            .fields
            .iter()
            .filter(|(key, _)| key.as_str().is_some_and(|key| key.starts_with("splat_")))
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        required_fields.remove(Value::String("splat_optional".into()));
        required_fields.remove(Value::String("splat_reading".into()));
        // Provenance is optional: deleting the original must not invalidate a copy.
        required_fields.remove(Value::String("splat_origin".into()));
        if let Some(origin) = field(&e.fields, "splat_origin")
            && let Err(msg) = references(origin, &mut Vec::new())
        {
            error(&e.path, msg);
        }
        if let Some(optional) = field(&e.fields, "splat_optional")
            && let Err(msg) = references(optional, &mut Vec::new())
        {
            error(&e.path, msg);
        }
        if let Err(msg) = references(&Value::Mapping(required_fields), &mut refs) {
            error(&e.path, msg)
        }
        refs.extend(e.inventory.iter().map(|r| r.item.clone()));
        for r in refs {
            if resolve(&r).is_none() {
                error(&e.path, format!("Unresolved reference {r:?}"))
            }
        }
        if e.kind == "encounter_group"
            && let Ok(group) = encounter_group::EncounterGroup::of(e)
        {
            for member in &group.members {
                if resolve(&member.creature).is_some_and(|entry| entry.kind != "creature") {
                    error(
                        &e.path,
                        format!("{} must reference a creature entry", member.creature),
                    );
                }
            }
            if let Some(owner) = group.encounter
                && resolve(&Reference::Id(owner)).is_some_and(|entry| entry.kind != "encounter")
            {
                error(
                    &e.path,
                    format!("A group's encounter {owner} must reference an encounter entry"),
                );
            }
        }
        if let Some(character) = field(&e.fields, "splat_character") {
            match serde_yaml_ng::from_value::<character::Sheet>(character.clone()) {
                Ok(sheet) => {
                    for spell in sheet.spells.values() {
                        if resolve(&Reference::Id(spell.entry.id))
                            .is_some_and(|e| e.kind != "spell")
                        {
                            error(
                                &e.path,
                                format!("{} must reference a spell entry", spell.name),
                            );
                        }
                    }
                }
                Err(message) => error(&e.path, message.to_string()),
            }
        }
        for row in &e.inventory {
            if let Some(item) = resolve(&row.item) {
                if !["item", "container"].contains(&item.kind.as_str()) {
                    error(
                        &e.path,
                        format!("Inventory {} must reference item or container", row.key),
                    )
                }
                if item.kind == "container" && row.quantity != 1 {
                    error(
                        &e.path,
                        "Container setup placements require quantity one".into(),
                    )
                }
                if e.inventory_containers.contains(&row.key) {
                    if row.quantity != 1 {
                        error(&e.path, "Populated containers require quantity one".into())
                    }
                    if item.kind == "container" {
                        error(
                            &e.path,
                            "Referenced setup and local contents cannot merge".into(),
                        )
                    }
                    if field(&item.fields, "splat_item")
                        .and_then(Value::as_mapping)
                        .and_then(|m| field(m, "containment"))
                        .is_none()
                    {
                        error(&e.path, format!("{} is not a container item", row.key))
                    }
                }
            }
        }
        if let Some(details) = field(&e.fields, "splat_inventory_details") {
            if let Some(details) = details.as_mapping() {
                for (key, detail) in details {
                    let Some(row) = key
                        .as_str()
                        .and_then(|key| e.inventory.iter().find(|r| r.key == key))
                    else {
                        error(
                            &e.path,
                            format!("Inventory details name unknown row {key:?}"),
                        );
                        continue;
                    };
                    let Some(detail) = detail.as_mapping() else {
                        error(&e.path, "Inventory details must be a mapping".into());
                        continue;
                    };
                    let Some(resources) = field(detail, "resources") else {
                        continue;
                    };
                    let Some(resources) = resources.as_mapping() else {
                        error(&e.path, "Item resource amounts must be a mapping".into());
                        continue;
                    };
                    if row.quantity != 1 {
                        error(
                            &e.path,
                            format!("Per-item resources require quantity one: {}", row.key),
                        );
                    }
                    let Some(mut item) = resolve(&row.item) else {
                        continue;
                    };
                    if item.kind == "container" {
                        let mut underlying = Vec::new();
                        if let Some(value) = field(&item.fields, "splat_item") {
                            let _ = references(value, &mut underlying);
                        }
                        let Some(definition) = underlying.first().and_then(&resolve) else {
                            continue;
                        };
                        item = definition;
                    }
                    let pools = field(&item.fields, "splat_resources").and_then(Value::as_mapping);
                    for (pool, amount) in resources {
                        let definition =
                            pools.and_then(|m| m.get(pool)).and_then(Value::as_mapping);
                        match definition {
                            None => {
                                error(&e.path, format!("Unknown item pool {}.{pool:?}", row.key))
                            }
                            Some(definition) => {
                                let max = field(definition, "max").and_then(Value::as_u64);
                                if amount
                                    .as_u64()
                                    .is_none_or(|amount| max.is_some_and(|max| amount > max))
                                {
                                    error(
                                        &e.path,
                                        format!("Invalid item pool amount {}.{pool:?}", row.key),
                                    );
                                }
                            }
                        }
                    }
                }
            } else {
                error(&e.path, "splat_inventory_details must be a mapping".into());
            }
        }
    }
    // Setup contents may reference another setup; detect cycles across notes.
    fn setup_cycle<'a>(
        entry: &Entry,
        resolve: &impl Fn(&Reference) -> Option<&'a Entry>,
        active: &mut BTreeSet<Uuid>,
        done: &mut BTreeSet<Uuid>,
    ) -> bool {
        if done.contains(&entry.id) {
            return false;
        }
        if !active.insert(entry.id) {
            return true;
        }
        for row in &entry.inventory {
            if let Some(target) = resolve(&row.item)
                && target.kind == "container"
                && setup_cycle(target, resolve, active, done)
            {
                return true;
            }
        }
        active.remove(&entry.id);
        done.insert(entry.id);
        false
    }
    let mut done = BTreeSet::new();
    for entry in entries.iter().filter(|e| e.kind == "container") {
        if setup_cycle(entry, &resolve, &mut BTreeSet::new(), &mut done) {
            error(&entry.path, "Container setup reference cycle".into());
        }
        let mut underlying = Vec::new();
        if let Some(value) = field(&entry.fields, "splat_item")
            && let Err(message) = references(value, &mut underlying)
        {
            error(&entry.path, message);
        }
        if underlying.len() != 1
            || underlying.first().and_then(&resolve).is_none_or(|e| {
                e.kind != "item"
                    || field(&e.fields, "splat_item")
                        .and_then(Value::as_mapping)
                        .and_then(|m| field(m, "containment"))
                        .is_none()
            })
        {
            error(
                &entry.path,
                "Container setup must reference one item with containment".into(),
            );
        }
    }
    const KNOWN_FIELDS: &[&str] = &[
        "splat_reading",
        "splat_actions",
        "splat_character",
        "splat_import",
        "splat_conversion",
        "splat_contracts",
        "splat_clauses",
        "splat_attack_options",
        "splat_attack_bindings",
        "splat_stat_policy",
        "splat_budgets",
        "splat_creature",
        "splat_effect",
        "splat_external",
        "splat_effects",
        "splat_encounter",
        "splat_encounter_group",
        "splat_world",
        "splat_forms",
        "splat_health",
        "splat_hit_dice",
        "splat_dndbeyond_ids",
        "splat_id",
        "splat_import_routes",
        "splat_initial",
        "splat_inventory_details",
        "splat_item",
        "splat_key",
        "splat_kind",
        "splat_languages",
        "splat_limits",
        "splat_name",
        "splat_namespace",
        "splat_optional",
        "splat_origin",
        "splat_packages",
        "splat_parameters",
        "splat_party",
        "splat_placement",
        "splat_dndbeyond_campaign_id",
        "splat_dndbeyond_campaign_name",
        "splat_proficiencies",
        "splat_relationships",
        "splat_requires",
        "splat_requires_mechanics",
        "splat_resources",
        "splat_rule",
        "splat_rule_choices",
        "splat_rule_group",
        "splat_run_settings",
        "splat_schema",
        "splat_seed",
        "splat_senses",
        "splat_sequence",
        "splat_sources",
        "splat_spell",
        "splat_stats",
        "splat_strategy",
        "splat_traits",
        "splat_reference",
    ];
    for entry in entries {
        for key in entry.fields.keys().filter_map(Value::as_str) {
            if key.starts_with("splat_") && !KNOWN_FIELDS.contains(&key) {
                report.warnings.push(Diagnostic{path:entry.path.clone(),message:format!("Unknown field {key} preserved; structured editing and execution are unavailable")});
            }
        }
    }
    report.limitations.push("Read-only structural/reference validation. Nested mechanics, ownership, rule selection, and runtime semantics are not fully validated. This structural check does not compile or execute mechanics; executable_entries is zero. Use verify-content --executable for the bounded execution profiles.".into());
    report
}

/// A file can vanish between listing and reading, because a save replaces it through a
/// temporary file. That file is simply no longer there, and the load goes on.
fn unless_vanished<T>(result: std::io::Result<T>) -> Result<Option<T>, String> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error.to_string()),
    }
}

/// Replace only named top-level fields; all other bytes, comments and body stay intact.
pub fn patch_fields(
    text: &str,
    changes: &[(&str, Option<serde_yaml_ng::Value>)],
) -> Result<String, String> {
    let prefix = if text.starts_with('\u{feff}') {
        "\u{feff}"
    } else {
        ""
    };
    let raw = text.trim_start_matches('\u{feff}');
    let newline = if raw.starts_with("---\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let start = format!("---{newline}");
    let raw = raw.strip_prefix(&start).ok_or("Missing frontmatter")?;
    let end = format!("{newline}---");
    let index = raw.find(&end).ok_or("Missing frontmatter end")?;
    let front = &raw[..index];
    let value: serde_yaml_ng::Value = serde_yaml_ng::from_str(front).map_err(|e| e.to_string())?;
    let map = value.as_mapping().ok_or("Frontmatter must be a mapping")?;
    let lines: Vec<_> = front.split(newline).collect();
    let mut remove = std::collections::BTreeSet::new();
    let mut replacements = BTreeMap::new();
    let mut appended = Vec::new();
    let mut expected = map.clone();
    for (key, value) in changes {
        if let Some(value) = value {
            expected.insert((*key).into(), value.clone());
        } else {
            expected.remove(*key);
        }
        let encoded = if let Some(value) = value {
            let mut field = Mapping::new();
            field.insert((*key).into(), value.clone());
            serde_yaml_ng::to_string(&field)
                .map_err(|e| e.to_string())?
                .trim_end()
                .replace('\n', newline)
        } else {
            String::new()
        };
        if map.contains_key(*key) {
            let matches: Vec<_> = lines
                .iter()
                .enumerate()
                .filter(|(_, line)| {
                    if line.starts_with([' ', '\t', '#', '-']) {
                        return false;
                    }
                    line.split_once(':').is_some_and(|(raw, _)| {
                        serde_yaml_ng::from_str::<Value>(raw)
                            .ok()
                            .and_then(|v| v.as_str().map(str::to_owned))
                            .as_deref()
                            == Some(key)
                    })
                })
                .map(|(i, _)| i)
                .collect();
            if matches.len() != 1 {
                return Err(format!(
                    "Cannot safely edit {key}. Use a plain, unquoted top-level field in Markdown first."
                ));
            }
            let start = matches[0];
            if let Some(value) = value
                && !value.is_mapping()
                && !value.is_sequence()
                && map
                    .get(*key)
                    .is_some_and(|v| !v.is_mapping() && !v.is_sequence())
            {
                let line = lines[start];
                let colon = line.find(':').unwrap() + 1;
                let raw = &line[colon..];
                if !raw.trim_start().starts_with(['|', '>']) {
                    let mut quote = None;
                    let mut escaped = false;
                    let mut comment = raw.len();
                    for (at, ch) in raw.char_indices() {
                        if ch == '#'
                            && quote.is_none()
                            && (at == 0 || raw.as_bytes()[at - 1].is_ascii_whitespace())
                        {
                            comment = at;
                            break;
                        }
                        if !escaped && matches!(ch, '\'' | '"') {
                            if quote == Some(ch) {
                                quote = None;
                            } else if quote.is_none() {
                                quote = Some(ch);
                            }
                        }
                        escaped = ch == '\\' && !escaped;
                    }
                    let old = &raw[..comment];
                    let begin = colon + old.len() - old.trim_start().len();
                    let end = colon + old.trim_end().len();
                    let scalar = serde_yaml_ng::to_string(value).map_err(|e| e.to_string())?;
                    if !scalar.trim_end().contains('\n') {
                        let mut updated = line.to_string();
                        let encoded = format!(
                            "{}{}",
                            if raw.is_empty() { " " } else { "" },
                            scalar.trim_end()
                        );
                        updated.replace_range(begin..end.max(begin), &encoded);
                        replacements.insert(start, updated);
                        continue;
                    }
                }
            }
            let mut stop = start + 1;
            while stop < lines.len() {
                let line = lines[stop];
                if !line.is_empty() && !line.starts_with([' ', '\t', '-', '#']) {
                    break;
                }
                // Keep comments and blank lines; removing a changed field's data is intentional.
                if !line.trim().is_empty() && !line.trim_start().starts_with('#') {
                    remove.insert(stop);
                }
                stop += 1;
            }
            remove.insert(start);
            if value.is_some() {
                replacements.insert(start, encoded);
            }
        } else if value.is_some() {
            appended.push(encoded);
        }
    }
    let mut output = format!("{prefix}{start}");
    for (i, line) in lines.iter().enumerate() {
        if let Some(replacement) = replacements.get(&i) {
            output.push_str(replacement);
            output.push_str(newline);
        } else if !remove.contains(&i) {
            output.push_str(line);
            output.push_str(newline);
        }
    }
    for encoded in appended {
        output.push_str(&encoded);
        output.push_str(newline);
    }
    output.push_str(raw[index + newline.len()..].as_ref());
    let result = output
        .trim_start_matches('\u{feff}')
        .strip_prefix(&start)
        .ok_or("Missing frontmatter")?;
    let end = result.find(&end).ok_or("Missing frontmatter end")?;
    let checked: Mapping = serde_yaml_ng::from_str(&result[..end]).map_err(|e| e.to_string())?;
    if checked != expected {
        return Err("Edit changed another property. The file was kept.".into());
    }
    Ok(output)
}

pub fn read_directory(root: &Path) -> Result<(Vec<Entry>, Vec<Diagnostic>), String> {
    read_directory_using(root, Entry::parse)
}

fn read_directory_using(
    root: &Path,
    parse: fn(&str, &str) -> Result<Entry, String>,
) -> Result<(Vec<Entry>, Vec<Diagnostic>), String> {
    fn walk(
        root: &Path,
        current: &Path,
        files: &mut Vec<(String, String)>,
        errors: &mut Vec<Diagnostic>,
    ) -> Result<(), String> {
        let mut report = |path: &Path, message: String| {
            errors.push(Diagnostic {
                path: path
                    .strip_prefix(root)
                    .unwrap_or(path)
                    .to_string_lossy()
                    .into_owned(),
                message,
            })
        };
        let listing = match std::fs::read_dir(current) {
            Ok(listing) => listing,
            Err(error) if current == root => return Err(format!("{}: {error}", root.display())),
            Err(error) => {
                report(current, error.to_string());
                return Ok(());
            }
        };
        let mut paths = Vec::new();
        for entry in listing {
            match entry {
                Ok(entry) => paths.push(entry.path()),
                Err(error) => report(current, error.to_string()),
            }
        }
        paths.sort();
        for path in paths {
            if path
                .file_name()
                .is_some_and(|name| name.to_string_lossy().starts_with('.'))
            {
                continue;
            }
            let meta = match unless_vanished(std::fs::symlink_metadata(&path)) {
                Ok(Some(meta)) => meta,
                Ok(None) => continue,
                Err(message) => {
                    errors.push(Diagnostic {
                        path: path
                            .strip_prefix(root)
                            .unwrap_or(&path)
                            .to_string_lossy()
                            .into_owned(),
                        message,
                    });
                    continue;
                }
            };
            if meta.file_type().is_symlink() {
                continue;
            }
            if meta.is_dir() {
                walk(root, &path, files, errors)?
            } else if path.extension().is_some_and(|x| x == "md") {
                let text = match unless_vanished(std::fs::read_to_string(&path)) {
                    Ok(Some(text)) => text,
                    Ok(None) => continue,
                    Err(message) => {
                        errors.push(Diagnostic {
                            path: path
                                .strip_prefix(root)
                                .unwrap_or(&path)
                                .to_string_lossy()
                                .into_owned(),
                            message,
                        });
                        continue;
                    }
                };
                let normalized = text.trim_start_matches('\u{feff}').replace("\r\n", "\n");
                let name = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned();
                let Some(frontmatter) = normalized.strip_prefix("---\n") else {
                    if normalized.trim_start().starts_with("splat_") {
                        errors.push(Diagnostic {
                            path: name,
                            message: "Line 1: Splat fields need an opening --- line.".into(),
                        });
                    }
                    continue;
                };
                let frontmatter = frontmatter.split("\n---").next().unwrap_or(frontmatter);
                if !frontmatter.lines().any(|line| line.starts_with("splat_")) {
                    if ["schema:", "id:", "kind:"]
                        .iter()
                        .all(|key| frontmatter.lines().any(|line| line.starts_with(key)))
                    {
                        errors.push(Diagnostic { path: name, message: "Line 2: These fields look like Splat content. Restore their splat_ prefixes.".into() });
                    }
                    continue;
                }
                let name = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .into_owned();
                files.push((name, text));
            }
        }
        Ok(())
    }
    let mut files = Vec::new();
    let mut errors = Vec::new();
    walk(root, root, &mut files, &mut errors)?;
    // Parallel parsing preserves traversal order for entries and diagnostics.
    let parsed: Vec<_> = files
        .par_iter()
        .map(|(path, text)| {
            parse(path, text).map_err(|message| Diagnostic {
                path: path.clone(),
                message,
            })
        })
        .collect();
    let mut entries = Vec::new();
    for result in parsed {
        match result {
            Ok(entry) => entries.push(entry),
            Err(error) => errors.push(error),
        }
    }
    let catalogue = entries.clone();
    let sources: std::collections::BTreeMap<_, _> = files.into_iter().collect();
    for entry in &mut entries {
        let source = &sources[&entry.path];
        list_formats::resolve(entry, &catalogue, source);
    }
    Ok((entries, errors))
}
