//! Schema-2 notes and pure, explicit conversion. Loading never writes files.
use super::{Entry, Mapping, Value, layout::Flag, patch_fields};

pub fn schema(fields: &Mapping) -> Result<u64, String> {
    match fields.get("splat_schema").and_then(Value::as_u64) {
        Some(version @ (1 | 2)) => Ok(version),
        _ => {
            Err("Unsupported content schema (expected 1 or 2); raw source remains unchanged".into())
        }
    }
}

pub const WORLD_FIELDS: &[(&str, &str)] = &[
    ("type", "splat_type"),
    ("status", "splat_status"),
    ("creature", "splat_stat_block"),
    ("in", "splat_in"),
    ("part_of", "splat_part_of"),
    ("when", "splat_when"),
    ("until", "splat_until"),
    ("aliases", "aliases"),
];

/// A world type is the DM's own word. It only names a group on the World page.
fn type_word(word: &str) -> bool {
    word.len() <= 40
        && word.starts_with(|c: char| c.is_alphabetic())
        && word
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, ' ' | '_' | '-'))
}

pub fn flat_reference(value: &Value) -> Value {
    if let Some(link) = value.get("link").and_then(Value::as_str) {
        let terms = ["id", "key"]
            .iter()
            .filter_map(|key| {
                value
                    .get(*key)
                    .and_then(Value::as_str)
                    .map(|v| format!("{key}={v}"))
            })
            .collect::<Vec<_>>();
        if !terms.is_empty() {
            return format!("{link} %% splat {} %%", terms.join(" ")).into();
        }
    }
    value
        .get("key")
        .or_else(|| value.get("id"))
        .or_else(|| value.get("link"))
        .unwrap_or(value)
        .clone()
}

pub fn model_reference(value: Value) -> Value {
    if let Some(raw) = value.as_str() {
        if let Some((link, _)) = raw.split_once(" %% splat ") {
            let mut fields = Mapping::new();
            for key in ["id", "key"] {
                if let Some(value) = super::list_formats::machine(raw, key) {
                    fields.insert(key.into(), value.into());
                }
            }
            if !fields.is_empty() {
                fields.insert("link".into(), link.into());
                return fields.into();
            }
        }
        if super::resource_key(raw) {
            return Value::Mapping([("key".into(), value)].into_iter().collect());
        }
        if uuid::Uuid::parse_str(raw).is_ok() {
            return Value::Mapping([("id".into(), value)].into_iter().collect());
        }
    }
    value
}

pub fn read(text: &str, fields: &mut Mapping) -> Vec<Flag> {
    let mut flags = Vec::new();
    if let Some(Value::Mapping(payload)) = fields.remove("splat_payload") {
        for (key, value) in payload {
            if fields.contains_key(&key) {
                flags.push(Flag {
                    line: 1,
                    message: format!("Duplicate payload field {key:?}; front matter used"),
                });
            } else {
                fields.insert(key, value);
            }
        }
    }
    let flag = |flags: &mut Vec<Flag>, key: &str, message: &str| {
        let line = text
            .lines()
            .position(|line| {
                !line.starts_with([' ', '\t'])
                    && line.split_once(':').is_some_and(|(raw, _)| {
                        serde_yaml_ng::from_str::<Value>(raw)
                            .ok()
                            .and_then(|v| v.as_str().map(str::to_owned))
                            .as_deref()
                            == Some(key)
                    })
            })
            .map_or(1, |at| at + 1);
        flags.push(Flag {
            line,
            message: message.into(),
        });
    };
    {
        super::list_formats::read(text, fields, &mut flags);
        let kind = fields
            .get("splat_kind")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_owned();
        if [
            "rule",
            "effect",
            "sequence",
            "strategy",
            "seed",
            "rule_group",
        ]
        .contains(&kind.as_str())
        {
            fields
                .entry(Value::from(format!("splat_{kind}")))
                .or_insert_with(|| Value::Mapping(Mapping::new()));
        }
        for key in ["splat_origin", "splat_sources", "splat_requires"] {
            let invalid = fields.get(key).is_some_and(|value| {
                if key == "splat_origin" {
                    !value.is_string() && !value.is_null()
                } else {
                    !value
                        .as_sequence()
                        .is_some_and(|values| values.iter().all(Value::is_string))
                }
            });
            if invalid {
                fields.remove(key);
                flag(&mut flags, key, &format!("{key}: value not read"));
            }
        }
        if fields.get("splat_kind").and_then(Value::as_str) == Some("world") {
            if !fields.contains_key("splat_type") {
                flag(&mut flags, "splat_type", "splat_type: value not set");
            }
            if fields.remove("splat_world").is_some() {
                flag(&mut flags, "splat_world", "Old data ignored");
            }
            let mut world = Mapping::new();
            for &(old, flat) in WORLD_FIELDS {
                if let Some(mut value) = fields.remove(flat) {
                    // One group reads as a list of one, as Obsidian shows a text property.
                    if old == "part_of" && value.is_string() {
                        value = Value::Sequence(vec![value]);
                    }
                    // Without quotes, YAML reads `- [[Verori]]` as a list in a list.
                    if old == "part_of"
                        && let Value::Sequence(groups) = &mut value
                    {
                        for group in groups {
                            let name = group
                                .as_sequence()
                                .and_then(|outer| outer.first())
                                .and_then(Value::as_sequence)
                                .and_then(|inner| inner.first())
                                .and_then(Value::as_str);
                            if let Some(name) = name {
                                *group = format!("[[{name}]]").into();
                            }
                        }
                    }
                    let valid = match old {
                        "type" => value.as_str().is_some_and(type_word),
                        "aliases" | "part_of" => value
                            .as_sequence()
                            .is_some_and(|a| a.iter().all(Value::is_string)),
                        _ => value.is_string() || value.is_null(),
                    };
                    if valid {
                        world.insert(
                            old.into(),
                            if ["creature", "in"].contains(&old) {
                                model_reference(value)
                            } else if old == "type" {
                                value.as_str().unwrap().to_lowercase().into()
                            } else {
                                value
                            },
                        );
                    } else {
                        let shown = text
                            .lines()
                            .find_map(|line| line.strip_prefix(&format!("{flat}:")))
                            .map(str::trim)
                            .unwrap_or("");
                        let (label, help) = match old {
                            "type" => ("Type", "Use one short word, such as npc or faction."),
                            "status" => ("Status", "Use a short word, such as alive."),
                            "creature" => ("Stat block", "Use one creature name or link."),
                            "in" => ("In", "Use one place name or link."),
                            "part_of" => ("Part of", "Use a list of names or links."),
                            "when" => ("When", "Use words such as 120 years ago."),
                            "until" => ("Until", "Use words such as 90 years ago."),
                            _ => ("Aliases", "Use a list of names."),
                        };
                        flag(&mut flags, flat, "");
                        let last = flags.last_mut().unwrap();
                        last.message =
                            format!("{label} \"{shown}\" not read (line {}). {help}", last.line);
                    }
                }
            }
            fields.insert("splat_world".into(), Value::Mapping(world));
        }
        if let Some(pages) = fields.remove("splat_pages") {
            let reference = fields
                .entry(Value::String("splat_reference".into()))
                .or_insert(Value::Mapping(Mapping::new()));
            if let Some(reference) = reference.as_mapping_mut() {
                match expand_pages(&pages) {
                    Some(pages) => {
                        reference.insert("pages".into(), pages);
                    }
                    None => flag(
                        &mut flags,
                        "splat_pages",
                        "splat_pages: use page numbers or ranges",
                    ),
                }
            }
        }
    }
    if let Some(value) = fields.remove("splat_origin") {
        fields.insert("splat_origin".into(), model_reference(value));
    }
    if let Some(Value::Sequence(values)) = fields.get_mut("splat_requires") {
        for value in values {
            *value = model_reference(value.clone());
        }
    }
    if let Some(world) = fields
        .get_mut("splat_world")
        .and_then(Value::as_mapping_mut)
    {
        for key in ["in", "creature"] {
            if let Some(value) = world.get_mut(key) {
                *value = model_reference(value.clone());
            }
        }
    }
    let kind = fields
        .get("splat_kind")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();
    if super::stat_blocks::owns(&kind, fields.contains_key("splat_character")) {
        super::stat_blocks::read_tables(&kind, text, fields, &mut flags);
    }
    if kind == "creature" && !fields.contains_key("splat_character") {
        super::stat_blocks::normalize_plain_stats(fields);
    }
    flags
}

fn expand_pages(value: &Value) -> Option<Value> {
    if value
        .as_sequence()
        .is_some_and(|values| values.iter().all(|v| v.as_u64().is_some()))
    {
        return Some(value.clone());
    }
    let mut pages = Vec::new();
    for term in value.as_str()?.split(',') {
        let (start, end) = term
            .trim()
            .split_once('-')
            .map_or((term.trim(), term.trim()), |(a, b)| (a.trim(), b.trim()));
        let start = start.parse::<u64>().ok()?;
        let end = end.parse::<u64>().ok()?;
        if start > end || end - start > 10000 {
            return None;
        }
        pages.extend((start..=end).map(Value::from));
    }
    Some(Value::Sequence(pages))
}

pub fn eligible(entry: &Entry) -> bool {
    [
        "reference",
        "container",
        "world",
        "creature",
        "spell",
        "item",
        "campaign",
        "encounter_group",
        "encounter",
        "package",
        "rule",
        "rule_group",
        "effect",
        "sequence",
        "strategy",
        "seed",
    ]
    .contains(&entry.kind.as_str())
        || entry.fields.contains_key("splat_character")
}

/// Compare reader models in memory, ignoring schema and resolved link spelling.
/// Error messages contain field names only, never source values.
pub fn verify_conversion(
    path: &str,
    before: &str,
    after: &str,
    entries: &[Entry],
) -> Result<String, String> {
    fn reference(value: &mut Value, entries: &[Entry]) {
        let raw = value
            .get("key")
            .or_else(|| value.get("id"))
            .or_else(|| value.get("link"))
            .unwrap_or(value);
        if let Some(raw) = raw.as_str()
            && let Ok(id) = resolve_readable(raw, entries)
        {
            *value = Value::Mapping([("id".into(), id.to_string().into())].into_iter().collect());
        }
    }
    fn model(entry: &Entry, entries: &[Entry]) -> Mapping {
        fn visit(value: &mut Value, entries: &[Entry]) {
            match value {
                Value::Mapping(map) => {
                    if !map.is_empty()
                        && map.keys().all(|k| {
                            k.as_str()
                                .is_some_and(|s| ["id", "key", "link"].contains(&s))
                        })
                    {
                        reference(value, entries);
                    } else {
                        for value in map.values_mut() {
                            visit(value, entries);
                        }
                    }
                }
                Value::Sequence(values) => {
                    for value in values {
                        visit(value, entries);
                    }
                }
                _ => {}
            }
        }
        let mut fields = (*entry.fields).clone();
        for key in ["splat_origin", "splat_requires"] {
            if let Some(value) = fields.get_mut(key) {
                if let Value::Sequence(values) = value {
                    for value in values {
                        reference(value, entries);
                    }
                } else {
                    reference(value, entries);
                }
            }
        }
        if let Some(world) = fields
            .get_mut("splat_world")
            .and_then(Value::as_mapping_mut)
        {
            for key in ["in", "creature"] {
                if let Some(value) = world.get_mut(key) {
                    reference(value, entries);
                }
            }
        }
        for value in fields.values_mut() {
            visit(value, entries);
        }
        super::list_formats::canonical(&fields)
    }
    let old = super::legacy::parse(path, before)?;
    let new = Entry::parse(path, after)?;
    let a = model(&old, entries);
    let b = model(&new, entries);
    if a != b {
        let mut keys: Vec<_> = a
            .keys()
            .chain(b.keys())
            .filter(|k| a.get(*k) != b.get(*k))
            .filter_map(Value::as_str)
            .collect();
        keys.sort();
        keys.dedup();
        return Err(format!("Model differs in {}", keys.join(", ")));
    }
    Ok(old.kind)
}

/// Patch canonical fields in their original flat property or marked end fence.
/// This is for model-based copy and repair code. It never renders the note body.
pub fn patch_model(text: &str, changes: &[(&str, Option<Value>)]) -> Result<String, String> {
    let original = Entry::parse("note.md", text)?;
    if super::stat_blocks::owns(
        &original.kind,
        original.fields.contains_key("splat_character"),
    ) && changes
        .iter()
        .any(|(key, _)| super::stat_blocks::data_roots(&original.kind).contains(key))
    {
        let table_changes: Vec<_> = changes
            .iter()
            .filter(|(key, _)| super::stat_blocks::data_roots(&original.kind).contains(key))
            .cloned()
            .collect();
        let after = super::stat_blocks::patch(text, &table_changes)?;
        let others: Vec<_> = changes
            .iter()
            .filter(|(key, _)| !super::stat_blocks::data_roots(&original.kind).contains(key))
            .cloned()
            .collect();
        return if others.is_empty() {
            Ok(after)
        } else {
            patch_model(&after, &others)
        };
    }
    if ["campaign", "encounter_group", "encounter"].contains(&original.kind.as_str()) {
        return super::list_formats::patch(text, changes);
    }
    let mut after = text.to_owned();
    for (key, value) in changes {
        if *key == "splat_world"
            && let Some(world) = value.as_ref().and_then(Value::as_mapping)
        {
            let changes: Vec<_> = WORLD_FIELDS
                .iter()
                .filter_map(|(old, flat)| {
                    let previous = original
                        .fields
                        .get("splat_world")
                        .and_then(|world| world.get(*old));
                    (previous != world.get(*old))
                        .then(|| (*flat, world.get(*old).map(flat_reference)))
                })
                .collect();
            after = patch_fields(&after, &changes)?;
            continue;
        }
        let direct = format!("<!-- splat:yaml path={key} -->");
        let payload = "<!-- splat:yaml path=splat_payload -->";
        let direct = fence_value(&after, &direct)?;
        let payload = fence_value(&after, payload)?.filter(|range| {
            serde_yaml_ng::from_str::<Mapping>(&fence_text(&after[range.clone()]))
                .is_ok_and(|payload| payload.contains_key(*key))
        });
        let marked = direct
            .map(|range| (range, false))
            .or_else(|| payload.map(|range| (range, true)));
        if let Some((range, in_payload)) = marked {
            let (start, stop) = (range.start, range.end);
            let encoded = if in_payload {
                let mut payload: Mapping =
                    serde_yaml_ng::from_str(&fence_text(&after[start..stop]))
                        .map_err(|e| e.to_string())?;
                if let Some(value) = value {
                    payload.insert((*key).into(), value.clone());
                } else {
                    payload.remove(*key);
                }
                serde_yaml_ng::to_string(&payload).map_err(|e| e.to_string())?
            } else {
                serde_yaml_ng::to_string(
                    value
                        .as_ref()
                        .ok_or("Cannot remove a required data fence")?,
                )
                .map_err(|e| e.to_string())?
            };
            let encoded = if after[start..stop].starts_with('>') {
                encoded
                    .lines()
                    .map(|line| format!("> {line}\n"))
                    .collect::<String>()
            } else {
                encoded
            };
            let encoded = if after.contains("\r\n") {
                encoded.replace('\n', "\r\n")
            } else {
                encoded
            };
            after.replace_range(start..stop, &encoded);
        } else {
            let flat = value.as_ref().map(|value| {
                if let Value::Sequence(values) = value {
                    Value::Sequence(values.iter().map(flat_reference).collect())
                } else {
                    flat_reference(value)
                }
            });
            if key.starts_with("splat_") && flat.as_ref().is_some_and(Value::is_mapping) {
                let newline = if after.contains("\r\n") { "\r\n" } else { "\n" };
                let encoded =
                    serde_yaml_ng::to_string(flat.as_ref().unwrap()).map_err(|e| e.to_string())?;
                after = patch_fields(&after, &[(*key, None)])?;
                after.push_str(&format!("\n## Splat data\n\n<!-- splat:yaml path={key} -->\n\n```yaml\n{encoded}```\n").replace('\n', newline));
                after = folded_data(&after);
                continue;
            }
            after = patch_fields(&after, &[(*key, flat)])?;
        }
    }
    let mut expected = (*original.fields).clone();
    for (key, value) in changes {
        if let Some(value) = value {
            expected.insert((*key).into(), value.clone());
        } else {
            expected.remove(*key);
        }
    }
    let checked = Entry::parse("note.md", &after)?;
    if *checked.fields != expected || !checked.flags.is_empty() {
        return Err("Edit changed another value. The file was kept.".into());
    }
    Ok(after)
}

/// Replace non-printing data while retaining the surrounding note and fence.
pub(super) fn patch_fence(text: &str, key: &str, value: &Value) -> Result<String, String> {
    let marker = format!("<!-- splat:yaml path={key} -->");
    let encoded = serde_yaml_ng::to_string(value).map_err(|e| e.to_string())?;
    let mut after = text.to_owned();
    if let Some(range) = fence_value(text, &marker)? {
        let encoded = if text[range.clone()].starts_with('>') {
            encoded
                .lines()
                .map(|line| format!("> {line}\n"))
                .collect::<String>()
        } else {
            encoded
        };
        after.replace_range(range, &encoded);
    } else {
        after.push_str(&format!(
            "\n## Splat data\n\n{marker}\n\n```yaml\n{encoded}```\n"
        ));
        after = folded_data(&after);
    }
    Ok(after)
}

fn fence_value(text: &str, marker: &str) -> Result<Option<std::ops::Range<usize>>, String> {
    let Some(at) = text.find(marker).map(|at| at + marker.len()) else {
        return Ok(None);
    };
    let open = text[at..].find("```yaml").ok_or("Missing data fence")? + at + "```yaml".len();
    let start = text[open..].find('\n').ok_or("Missing data line")? + open + 1;
    let stop = text[start..]
        .find("\n```")
        .or_else(|| text[start..].find("\n> ```"))
        .ok_or("Missing fence end")?
        + start
        + 1;
    Ok(Some(start..stop))
}

/// Resolve a readable requirement without choosing an ambiguous catalogue name.
pub fn resolve_readable<'a>(
    raw: &str,
    entries: impl IntoIterator<Item = &'a Entry>,
) -> Result<uuid::Uuid, String> {
    let entries: Vec<_> = entries.into_iter().collect();
    if let Some(link) = raw.strip_prefix("[[").and_then(|s| s.strip_suffix("]]")) {
        let target = link
            .split(['|', '#'])
            .next()
            .unwrap_or(link)
            .trim()
            .trim_end_matches(".md");
        let found: Vec<_> = entries
            .iter()
            .filter(|entry| {
                entry
                    .path
                    .trim_end_matches(".md")
                    .eq_ignore_ascii_case(target)
                    || std::path::Path::new(&entry.path)
                        .file_stem()
                        .is_some_and(|name| name.to_string_lossy().eq_ignore_ascii_case(target))
            })
            .collect();
        return match found.as_slice() {
            [entry] => Ok(entry.id),
            [] => resolve_readable(target, entries.iter().copied()),
            many => Err(format!(
                "Requirement {raw} matches {} entries; use a link",
                many.len()
            )),
        };
    }
    if let Some(entry) = entries
        .iter()
        .find(|entry| entry.key.as_deref() == Some(raw))
    {
        return Ok(entry.id);
    }
    if let Ok(id) = uuid::Uuid::parse_str(raw)
        && entries.iter().any(|entry| entry.id == id)
    {
        return Ok(id);
    }
    let found: Vec<_> = entries
        .iter()
        .filter(|entry| entry.name.eq_ignore_ascii_case(raw))
        .collect();
    match found.as_slice() {
        [entry] => Ok(entry.id),
        [] => Err(format!("Requirement {raw} not found")),
        many => Err(format!(
            "Requirement {raw} matches {} entries; use a link",
            many.len()
        )),
    }
}

pub(super) fn folded_data(text: &str) -> String {
    let Some(at) = text.find("\n## Splat data") else {
        return text.into();
    };
    let title_end = text[at + 1..].find('\n').map_or(text.len(), |n| at + 1 + n);
    let section_end = text[title_end..]
        .find("\n## ")
        .map_or(text.len(), |n| title_end + n);
    let Some(close) = text[title_end..section_end].rfind("\n```") else {
        return text.into();
    };
    let end = title_end + close + "\n```".len();
    let title = if text[at + 4..title_end].trim().contains("from D&D Beyond") {
        "Splat data (from D&D Beyond; do not edit)"
    } else {
        "Splat data (edit in Splat)"
    };
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let mut folded = format!("{newline}> [!note]- {title}{newline}");
    for line in text[title_end..end].trim_matches(['\n', '\r']).lines() {
        folded.push_str("> ");
        folded.push_str(line.trim_end_matches('\r'));
        folded.push_str(newline);
    }
    format!("{}{folded}{}", &text[..at], &text[end..])
}

fn fence_text(raw: &str) -> String {
    raw.lines()
        .map(|line| {
            line.strip_prefix("> ")
                .or_else(|| line.strip_prefix('>'))
                .unwrap_or(line)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn convert(path: &str, text: &str) -> Result<String, String> {
    // Keep a generated sheet's checksum around the generated part, with its
    // personal notes outside it. A format conversion is not a local sheet edit.
    const STAMP: &str = "\n<!-- splat-generated-sha256:";
    if let Some((generated, tail)) = text.rsplit_once(STAMP)
        && let Some((hash, notes)) = tail.split_once(" -->\n")
    {
        use sha2::{Digest, Sha256};
        if hash != format!("{:x}", Sha256::digest(generated.as_bytes())) {
            return Err("Character copy has local edits. Review them before converting.".into());
        }
        let after = convert(path, generated)?;
        if after == generated {
            return Ok(text.into());
        }
        return Ok(format!(
            "{after}{STAMP}{:x} -->\n{notes}",
            Sha256::digest(after.as_bytes())
        ));
    }
    let entry = super::legacy::parse(path, text)?;
    if schema(&entry.fields)? == 2 || !eligible(&entry) {
        return Ok(folded_data(text));
    }
    if !entry.flags.is_empty() {
        return Err("Fix flagged values before converting this file.".into());
    }
    if ["campaign", "encounter_group", "encounter"].contains(&entry.kind.as_str()) {
        return super::list_formats::convert(text, &entry);
    }
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let mut changes = vec![("splat_schema", Some(2.into()))];
    let stat_kind =
        super::stat_blocks::owns(&entry.kind, entry.fields.contains_key("splat_character"));
    let mut stat_body = String::new();
    let mut stat_extras = Vec::new();
    if stat_kind {
        (stat_body, stat_extras) = super::stat_blocks::conversion(&entry.fields, &entry.kind)?;
        for key in super::stat_blocks::data_roots(&entry.kind) {
            changes.push((*key, None));
        }
    }
    if let Some(world) = entry.fields.get("splat_world").and_then(Value::as_mapping) {
        if world.keys().any(|key| {
            !WORLD_FIELDS
                .iter()
                .any(|(old, _)| key.as_str() == Some(old))
        }) {
            return Err("Unknown World properties need review before conversion.".into());
        }
        changes.push(("splat_world", None));
        for &(old, flat) in WORLD_FIELDS {
            if let Some(value) = world.get(old) {
                changes.push((flat, Some(flat_reference(value))));
            }
        }
    }
    for key in ["splat_origin", "splat_sources", "splat_requires"] {
        if let Some(value) = entry.fields.get(key) {
            let flat = match value {
                Value::Sequence(values) => {
                    Value::Sequence(values.iter().map(flat_reference).collect())
                }
                _ => flat_reference(value),
            };
            if flat.is_mapping()
                || flat
                    .as_sequence()
                    .is_some_and(|values| values.iter().any(|v| !v.is_string()))
            {
                return Err(format!(
                    "{key} needs a readable reference before conversion."
                ));
            }
            changes.push((key, Some(flat)));
        }
    }
    let character = entry.fields.contains_key("splat_character");
    let mut fences = String::new();
    let mut payload = Mapping::new();
    for (key, value) in entry.fields.iter() {
        let Some(key) = key.as_str() else {
            continue;
        };
        if stat_kind && super::stat_blocks::data_roots(&entry.kind).contains(&key) {
            continue;
        }
        if key == "splat_world"
            || key == "splat_sources"
            || key == "splat_origin"
            || key == "splat_requires"
        {
            continue;
        }
        let machine = (key.starts_with("splat_")
            && (value.is_mapping()
                || value
                    .as_sequence()
                    .is_some_and(|a| a.iter().any(|v| v.is_mapping() || v.is_sequence()))))
            || (character
                && key.starts_with("splat_")
                && ![
                    "splat_schema",
                    "splat_id",
                    "splat_kind",
                    "splat_key",
                    "splat_name",
                ]
                .contains(&key));
        let front_key = text.lines().any(|line| {
            !line.starts_with([' ', '\t'])
                && line.split_once(':').is_some_and(|(raw, _)| {
                    serde_yaml_ng::from_str::<Value>(raw)
                        .ok()
                        .as_ref()
                        .and_then(Value::as_str)
                        == Some(key)
                })
        });
        if !machine || !front_key {
            continue;
        }
        changes.push((key, None));
        if value.as_mapping().is_some_and(Mapping::is_empty)
            && !character
            && key == format!("splat_{}", entry.kind)
        {
            continue;
        }
        if character {
            payload.insert(key.into(), value.clone());
        } else {
            let mut fenced = value.clone();
            if key == "splat_reference"
                && let Some(reference) = fenced.as_mapping_mut()
            {
                reference.remove("pages");
            }
            fences.push_str(&format!(
                "\n<!-- splat:yaml path={key} -->\n\n```yaml\n{}```\n",
                serde_yaml_ng::to_string(&fenced).map_err(|e| e.to_string())?
            ));
        }
    }
    for (key, value) in stat_extras {
        fences.push_str(&format!(
            "\n<!-- splat:yaml path={key} -->\n\n```yaml\n{}```\n",
            serde_yaml_ng::to_string(&value).map_err(|e| e.to_string())?
        ));
    }
    if character {
        fences.push_str(&format!(
            "\n<!-- splat:yaml path=splat_payload -->\n\n```yaml\n{}```\n",
            serde_yaml_ng::to_string(&payload).map_err(|e| e.to_string())?
        ));
    }
    if let Some(reference) = entry
        .fields
        .get("splat_reference")
        .and_then(Value::as_mapping)
        && let Some(pages) = reference.get("pages")
    {
        changes.push(("splat_pages", Some(pages.clone())));
    }
    let source = if stat_kind {
        let source = super::stat_blocks::without_legacy(text, &entry.kind);
        if entry.kind == "creature" {
            super::stat_blocks::fold_printed_stats(&source)
        } else {
            source
        }
    } else {
        text.into()
    };
    let mut after = patch_fields(&source, &changes)?;
    if stat_kind {
        after = super::stat_blocks::insert_section(&after, &stat_body);
    }
    if !fences.is_empty() {
        after.push_str(&format!(
            "{newline}## {}{newline}",
            if character {
                "Splat data (from D&D Beyond; do not edit)"
            } else {
                "Splat data"
            }
        ));
        after.push_str(&fences.replace('\n', newline));
    }
    after = folded_data(&after);
    let checked = Entry::parse(path, &after)?;
    let mut before = (*entry.fields).clone();
    let mut next = (*checked.fields).clone();
    before.remove("splat_schema");
    next.remove("splat_schema");
    let blocking_flag = checked
        .flags
        .iter()
        .any(|flag| !(flag.message.starts_with("CR ") && flag.message.ends_with(" is used")));
    if before != next || blocking_flag {
        return Err("Conversion changed a value. The old file was kept.".into());
    }
    Ok(after)
}

/// Write readable references and refresh rule-choice ids from their visible text.
/// Check the resolved identity before accepting each replacement.
pub fn convert_resolved<'a>(
    path: &str,
    text: &str,
    entries: impl IntoIterator<Item = &'a Entry>,
) -> Result<String, String> {
    let original = super::legacy::parse(path, text)?;
    let entries: Vec<_> = entries.into_iter().collect();
    if original.kind == "campaign" && schema(&original.fields)? == 2 {
        let mut resolved = original.clone();
        super::list_formats::resolve(
            &mut resolved,
            &entries.iter().map(|e| (*e).clone()).collect::<Vec<_>>(),
            text,
        );
        let mut after = text.to_owned();
        if let Some(table) = super::layout::Document::new(text)
            .section("Rule choices")
            .and_then(|s| s.table(super::list_formats::RULES))
        {
            for row in table.rows.into_iter().rev().filter(|row| row.len() == 2) {
                if original
                    .flags
                    .iter()
                    .any(|flag| flag.line == row[1].line && flag.message == "Duplicate rule choice")
                {
                    continue;
                }
                let slot =
                    super::list_formats::machine(&row[1].value, "rule").unwrap_or_else(|| {
                        if row[0].value.eq_ignore_ascii_case("Combat initiative") {
                            "dnd2024:rules/combat_initiative".into()
                        } else {
                            row[0].value.clone()
                        }
                    });
                if let Some(id) = resolved
                    .fields
                    .get("splat_rule_choices")
                    .and_then(|v| v.get(&slot))
                    .and_then(|v| v.get("id"))
                    .and_then(Value::as_str)
                {
                    let visible = row[1].value.split("%%").next().unwrap_or("").trim();
                    let shown = if resolve_readable(
                        visible,
                        entries.iter().copied().filter(|entry| entry.kind == "rule"),
                    )
                    .is_ok()
                    {
                        visible
                    } else {
                        entries
                            .iter()
                            .find(|entry| entry.id.to_string() == id)
                            .map_or(visible, |entry| entry.name.as_str())
                    };
                    after.replace_range(
                        row[1].range.clone(),
                        &format!("{shown} %% splat rule={slot} id={id} %%"),
                    );
                }
            }
        }
        return super::list_formats::write_references(&after, entries.iter().copied());
    }
    if schema(&original.fields)? == 1
        && ["campaign", "encounter_group", "encounter"].contains(&original.kind.as_str())
    {
        let mut enriched = original.clone();
        fn enrich(value: &mut Value, entries: &[&Entry]) {
            match value {
                Value::Mapping(map) => {
                    if let Some(id) = map
                        .get("id")
                        .and_then(Value::as_str)
                        .and_then(|s| uuid::Uuid::parse_str(s).ok())
                        && let Some(target) = entries.iter().find(|e| e.id == id)
                        && target.key.is_none()
                        && !map.contains_key("link")
                    {
                        map.insert(
                            "link".into(),
                            format!(
                                "[[{}|{}]]",
                                target.path.trim_end_matches(".md"),
                                target.name
                            )
                            .into(),
                        );
                    }
                    for value in map.values_mut() {
                        enrich(value, entries);
                    }
                }
                Value::Sequence(values) => {
                    for value in values {
                        enrich(value, entries)
                    }
                }
                _ => {}
            }
        }
        for key in ["splat_encounter_group", "splat_encounter"] {
            if let Some(value) = enriched.fields.get_mut(key) {
                enrich(value, &entries);
            }
        }
        if let Some(packages) = enriched
            .fields
            .get_mut("splat_packages")
            .and_then(Value::as_sequence_mut)
        {
            for package in packages {
                let id = package
                    .get("package")
                    .and_then(|p| p.get("id"))
                    .and_then(Value::as_str)
                    .and_then(|s| uuid::Uuid::parse_str(s).ok());
                if let Some(target) = entries.iter().find(|e| Some(e.id) == id) {
                    package["name"] = target.name.clone().into();
                }
            }
        }
        let mut after = super::list_formats::convert(text, &enriched)?;
        if let Some(table) = super::layout::Document::new(&after)
            .section("Rule choices")
            .and_then(|s| s.table(super::list_formats::RULES))
        {
            let mut patches = Vec::new();
            for row in table.rows {
                let Some(slot) = super::list_formats::machine(&row[1].value, "rule") else {
                    continue;
                };
                let chosen = super::list_formats::machine(&row[1].value, "id");
                if let Some(target) = entries
                    .iter()
                    .find(|e| e.key.as_deref() == Some(slot.as_str()))
                {
                    patches.push((row[0].range.clone(), target.name.clone()));
                }
                if let Some(target) = entries.iter().find(|e| Some(e.id.to_string()) == chosen) {
                    patches.push((
                        row[1].range.clone(),
                        format!("{} %% splat rule={slot} id={} %%", target.name, target.id),
                    ));
                }
            }
            patches.sort_by_key(|(r, _)| r.start);
            for (range, value) in patches.into_iter().rev() {
                after.replace_range(range, &value);
            }
        }
        return super::list_formats::write_references(&after, entries.iter().copied());
    }
    let mut after = convert(path, text)?;
    if let Some(required) = original
        .fields
        .get("splat_requires")
        .and_then(Value::as_sequence)
    {
        let readable: Vec<_> = required
            .iter()
            .map(|value| {
                let raw = flat_reference(value);
                let Some(raw) = raw.as_str() else {
                    return raw;
                };
                let Ok(id) = resolve_readable(raw, entries.iter().copied()) else {
                    return raw.into();
                };
                let target = entries.iter().find(|entry| entry.id == id).unwrap();
                let readable = if let Some(key) = target
                    .key
                    .as_ref()
                    .filter(|key| key.starts_with("dnd2024:"))
                {
                    if resolve_readable(&target.name, entries.iter().copied()) == Ok(id) {
                        target.name.clone()
                    } else {
                        key.clone()
                    }
                } else {
                    format!(
                        "[[{}|{}]]",
                        target.path.trim_end_matches(".md"),
                        target.name
                    )
                };
                if resolve_readable(&readable, entries.iter().copied()) == Ok(id) {
                    readable.into()
                } else {
                    raw.into()
                }
            })
            .collect();
        after = patch_fields(
            &after,
            &[("splat_requires", Some(Value::Sequence(readable)))],
        )?;
    }
    if original.kind != "world" {
        return super::list_formats::write_references(&after, entries.iter().copied());
    }
    for (key, flat) in [("creature", "splat_stat_block"), ("in", "splat_in")] {
        let Some(raw) = original
            .fields
            .get("splat_world")
            .and_then(|world| world.get(key))
            .map(flat_reference)
        else {
            continue;
        };
        let Some(raw) = raw.as_str() else {
            continue;
        };
        if raw.starts_with("[[") {
            continue;
        }
        let Ok(id) = resolve_readable(raw, entries.iter().copied()) else {
            continue;
        };
        let Some(target) = entries.iter().find(|entry| entry.id == id) else {
            continue;
        };
        let readable = if key == "creature"
            && resolve_readable(&target.name, entries.iter().copied()) == Ok(id)
        {
            target.name.clone()
        } else {
            target
                .key
                .as_ref()
                .filter(|target_key| key == "creature" || target_key.starts_with("dnd2024:"))
                .cloned()
                .unwrap_or_else(|| {
                    format!(
                        "[[{}|{}]]",
                        target.path.trim_end_matches(".md"),
                        target.name
                    )
                })
        };
        if resolve_readable(&readable, entries.iter().copied()) != Ok(id) {
            continue;
        }
        let before = Entry::parse(path, &after)?;
        let next = patch_fields(&after, &[(flat, Some(readable.clone().into()))])?;
        let mut expected = (*before.fields).clone();
        expected
            .get_mut("splat_world")
            .and_then(Value::as_mapping_mut)
            .ok_or("Missing World properties")?
            .insert(key.into(), model_reference(readable.into()));
        if *Entry::parse(path, &next)?.fields != expected {
            return Err("Reference conversion changed another value.".into());
        }
        after = next;
    }
    Ok(after)
}
