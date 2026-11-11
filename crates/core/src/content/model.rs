//! A note's model, built from its fields and body before the writer lays it out.
use super::layout::Document;
use super::stat_blocks::identity_value;
use super::{
    Entry, Mapping, Value, character, definitions, field, formats, inventory, markdown,
    resource_key, stat_blocks, string,
};
use pulldown_cmark::{Event, Options, Parser, Tag};
use uuid::Uuid;

/// `body` is the note text below its properties.
pub(super) fn entry(path: &str, body: &str, fields: &Mapping) -> Result<Entry, String> {
    let mut fields = fields.clone();
    fields.remove("splat_schema");
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
    markdown::read(body, &mut fields)?;
    if kind == "creature" && !fields.contains_key("splat_character") {
        read_identity(body, &mut fields);
        stat_blocks::normalize_plain_stats(&mut fields);
    }
    normalize(&mut fields);
    if kind == "creature" && !fields.contains_key("splat_character") {
        stat_blocks::complete_challenge_fields(&mut fields);
    }
    let (inventory, inventory_containers) = inventory(body)?;
    let entry = Entry {
        path: path.into(),
        id,
        key,
        name,
        kind,
        fields: fields.into(),
        body_line: 1,
        body: body.into(),
        inventory,
        inventory_containers,
        flags: Vec::new(),
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

/// A printed identity line supplies only properties missing from the creature's fields.
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
    // Citations are display text; no execution code consumes them.
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
