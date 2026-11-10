//! Old-layout decoding is private to explicit conversion. Normal loads reject it.
use super::layout::Document;
use super::stat_blocks::identity_value;
use super::{
    Entry, Mapping, Value, character, definitions, field, formats, inventory, markdown,
    resource_key, stat_blocks, string,
};
use pulldown_cmark::{Event, Options, Parser, Tag};
use uuid::Uuid;

pub use super::OLD_LAYOUT;

pub fn read_directory(
    root: &std::path::Path,
) -> Result<(Vec<Entry>, Vec<super::Diagnostic>), String> {
    super::read_directory_using(root, parse)
}

pub fn parse(path: &str, text: &str) -> Result<Entry, String> {
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
    let schema = formats::schema(&fields)?;
    if schema == 2 {
        return Entry::parse(path, &text);
    }
    let id = Uuid::parse_str(string(&fields, "splat_id")?).map_err(|_| "Invalid splat_id UUID")?;
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
    markdown::read(&body, &mut fields)?;
    if kind == "creature" && !fields.contains_key("splat_character") {
        read_identity(&body, &mut fields);
        stat_blocks::normalize_plain_stats(&mut fields);
    }
    normalize(&mut fields);
    let flags = Vec::new();
    if kind == "creature" && !fields.contains_key("splat_character") {
        stat_blocks::complete_challenge_fields(&mut fields);
    }
    let (inventory, inventory_containers) = inventory(&body)?;
    let entry = Entry {
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
    if entry.kind == "encounter_group" {
        super::encounter_group::EncounterGroup::of(&entry)?;
    }
    if entry.kind == "world"
        && field(&entry.fields, "splat_world")
            .and_then(|world| world.get("type"))
            .and_then(Value::as_str)
            .is_none_or(|kind| kind.trim().is_empty())
    {
        return Err("splat_world needs a type, such as npc or location".into());
    }

    entry.reading()?;
    character::validate(&entry)?;
    definitions::conversion(&entry)?;
    Ok(entry)
}

/// Legacy printed identity supplies only properties missing from its YAML model.
fn read_identity(body: &str, fields: &mut Mapping) {
    let source = Document::new(body)
        .section_at_level("Source text", Some(pulldown_cmark::HeadingLevel::H2))
        .map(|s| &body[s.range()])
        .unwrap_or(body);
    for (event, range) in Parser::new_ext(source, Options::ENABLE_TABLES).into_offset_iter() {
        if !matches!(event, Event::Start(Tag::Paragraph)) {
            continue;
        }
        let line = source[range]
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .trim_end_matches('\\')
            .trim()
            .trim_matches('*');
        let mut depth = 0_i32;
        let mut comma = None;
        for (at, ch) in line.char_indices() {
            match ch {
                '(' => depth += 1,
                ')' => depth -= 1,
                ',' if depth == 0 => comma = Some(at),
                _ => {}
            }
        }
        if depth != 0 {
            continue;
        }
        let (identity, alignment) = comma.map_or((line, None), |at| {
            (&line[..at], Some(line[at + 1..].trim()))
        });
        let Some((size, kind)) = identity.split_once(' ') else {
            continue;
        };
        let Some(mut printed) = identity_value("size", size) else {
            continue;
        };
        let Some(kind) = identity_value("type", kind) else {
            continue;
        };
        printed.extend(kind);
        if let Some(alignment) = alignment.filter(|s| !s.is_empty()) {
            printed.insert("alignment".into(), alignment.into());
        }
        if let Some(creature) = fields
            .entry(Value::from("splat_creature"))
            .or_insert_with(|| Value::Mapping(Mapping::new()))
            .as_mapping_mut()
        {
            for (key, value) in printed {
                creature.entry(key).or_insert(value);
            }
        }
        break;
    }
}

/// Decode the old timeline list only for conversion.
pub fn timeline<T: serde::de::DeserializeOwned>(text: &str) -> Result<Vec<T>, String> {
    let text = text.trim_start_matches('\u{feff}').replace("\r\n", "\n");
    let rest = text
        .strip_prefix("---\n")
        .ok_or("Missing timeline properties")?;
    let front = rest
        .split("\n---")
        .next()
        .ok_or("Unclosed timeline properties")?;
    let fields: Mapping = serde_yaml_ng::from_str(front).map_err(|e| e.to_string())?;
    if fields.get("timeline_version").and_then(Value::as_u64) != Some(1) {
        return Err("Unsupported timeline version; expected 1 for conversion".into());
    }
    let value = fields
        .get("timeline")
        .cloned()
        .unwrap_or_else(|| Value::Sequence(Vec::new()));
    let mut ids = std::collections::BTreeSet::new();
    for event in value.as_sequence().ok_or("Timeline must be a list")? {
        let id = event
            .get("id")
            .and_then(Value::as_str)
            .ok_or("Missing event ID")?;
        if id.trim().is_empty() || !ids.insert(id) {
            return Err(format!("The event ID “{id}” is not unique."));
        }
    }
    serde_yaml_ng::from_value(value).map_err(|e| e.to_string())
}

/// Convert embedded note models in a saved run. Run state and JSON version stay intact.
pub fn convert_run(text: &str) -> Result<String, String> {
    fn visit(value: &mut serde_json::Value) -> Result<bool, String> {
        if value
            .get("fields")
            .and_then(|v| v.get("splat_schema"))
            .and_then(serde_json::Value::as_u64)
            == Some(1)
        {
            let fields: Mapping = serde_yaml_ng::from_value(
                serde_yaml_ng::to_value(&value["fields"]).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            let body = value["body"].as_str().ok_or("Saved entry has no body")?;
            let path = value["path"].as_str().unwrap_or("Run copy.md");
            let source = format!(
                "---\n{}---\n{body}",
                serde_yaml_ng::to_string(&fields).map_err(|e| e.to_string())?
            );
            let after = formats::convert(path, &source)?;
            *value =
                serde_json::to_value(Entry::parse(path, &after)?).map_err(|e| e.to_string())?;
            return Ok(true);
        }
        let mut changed = false;
        match value {
            serde_json::Value::Object(map) => {
                for child in map.values_mut() {
                    changed |= visit(child)?;
                }
            }
            serde_json::Value::Array(values) => {
                for child in values {
                    changed |= visit(child)?;
                }
            }
            _ => {}
        }
        Ok(changed)
    }
    let mut value: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    if visit(&mut value)? {
        serde_json::to_string_pretty(&value)
            .map(|s| s + "\n")
            .map_err(|e| e.to_string())
    } else {
        Ok(text.into())
    }
}

fn normalize(fields: &mut Mapping) {
    if let Some(value) = fields.remove("splat_origin") {
        fields.insert("splat_origin".into(), formats::model_reference(value));
    }
    if let Some(Value::Sequence(values)) = fields.get_mut("splat_requires") {
        for value in values {
            *value = formats::model_reference(value.clone());
        }
    }
    if let Some(world) = fields
        .get_mut("splat_world")
        .and_then(Value::as_mapping_mut)
    {
        for key in ["in", "creature"] {
            if let Some(value) = world.get_mut(key) {
                *value = formats::model_reference(value.clone());
            }
        }
    }
    // Citations are display text in both models; no execution code consumes them.
    if let Some(Value::Sequence(sources)) = fields.get_mut("splat_sources") {
        for source in sources {
            if let Some(map) = source.as_mapping() {
                let words = ["title", "locator", "url"]
                    .iter()
                    .filter_map(|key| map.get(*key).and_then(Value::as_str))
                    .collect::<Vec<_>>()
                    .join(", ");
                // Keep unknown citation metadata exactly, as a scalar YAML record.
                *source = if map.keys().all(|key| {
                    key.as_str()
                        .is_some_and(|key| ["title", "locator", "url"].contains(&key))
                }) {
                    words.into()
                } else {
                    serde_json::to_string(&source).unwrap_or_default().into()
                };
            }
        }
    }
}
