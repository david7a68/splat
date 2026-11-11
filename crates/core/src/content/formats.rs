//! Schema-2 notes: their flat properties, model edits, and the writer. Loading never writes files.
use super::{Entry, Mapping, Value, layout::Flag, patch_fields};

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

pub(super) fn model_reference(value: Value) -> Value {
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

/// Give each reference in `fields` its model form. The reader and the writer share this.
pub(super) fn model_references(fields: &mut Mapping) {
    // Not an edit in place: `remove` puts the last field where the origin was, and `insert`
    // puts the origin last. The writer lays out marked data in field order, so this order is
    // part of its output.
    if let Some(value) = fields.remove("splat_origin") {
        fields.insert("splat_origin".into(), model_reference(value));
    }
    if let Some(Value::Sequence(values)) = fields.get_mut("splat_requires") {
        for value in values {
            *value = model_reference(std::mem::take(value));
        }
    }
    if let Some(world) = fields
        .get_mut("splat_world")
        .and_then(Value::as_mapping_mut)
    {
        for key in ["in", "creature"] {
            if let Some(value) = world.get_mut(key) {
                *value = model_reference(std::mem::take(value));
            }
        }
    }
}

/// A value the reader could not read, as UI words name it: in curly quotation marks, on one
/// line. A control character, such as a tab, reads as a space.
pub(crate) fn quoted(value: &str) -> String {
    let words: String = value
        .chars()
        .map(|c| if c.is_control() { ' ' } else { c })
        .collect();
    format!("“{words}”")
}

/// The text of a value that was not read, without its comment, and its line. A list written
/// below its key names its first item that is not text; the key's line names the rest.
fn unread_value(text: &str, key_line: usize) -> (String, usize) {
    let lines: Vec<&str> = text.lines().collect();
    let shown = |raw: &str| {
        let raw = raw.trim();
        let parsed = serde_yaml_ng::from_str::<Value>(raw).ok();
        // A comment starts at ` #` outside quotes: cutting there reads as the same value.
        raw.match_indices(" #")
            .map(|(at, _)| raw[..at].trim_end())
            .find(|cut| parsed.is_some() && serde_yaml_ng::from_str::<Value>(cut).ok() == parsed)
            .unwrap_or(if raw.starts_with('#') { "" } else { raw })
            .to_owned()
    };
    let on_key = lines
        .get(key_line.saturating_sub(1))
        .and_then(|line| line.split_once(':'))
        .map(|(_, value)| shown(value))
        .unwrap_or_default();
    if !on_key.is_empty() {
        return (on_key, key_line);
    }
    lines
        .iter()
        .enumerate()
        .skip(key_line)
        // The closing `---` starts with `-` too. The note's text after it is never a value.
        .take_while(|(_, line)| {
            line.trim() != "---"
                && (line.trim().is_empty() || line.starts_with([' ', '\t', '-', '#']))
        })
        .filter_map(|(at, line)| Some((at + 1, shown(line.trim_start().strip_prefix('-')?))))
        .find(|(_, item)| {
            serde_yaml_ng::from_str::<Value>(item).is_ok_and(|value| {
                // `- [[Name]]` reads as a list in a list, and the reader accepts it.
                !value.is_string()
                    && !value
                        .as_sequence()
                        .and_then(|outer| outer.first())
                        .and_then(Value::as_sequence)
                        .is_some_and(|inner| inner.first().is_some_and(Value::is_string))
            })
        })
        .map_or((on_key, key_line), |(line, item)| (item, line))
}

pub fn read(text: &str, fields: &mut Mapping) -> Vec<Flag> {
    let mut flags = Vec::new();
    if let Some(Value::Mapping(payload)) = fields.remove("splat_payload") {
        for (key, value) in payload {
            if fields.contains_key(&key) {
                // A YAML key is usually text; any other key keeps its YAML form.
                let name = key.as_str().map_or_else(
                    || serde_yaml_ng::to_string(&key).unwrap_or_default(),
                    str::to_owned,
                );
                flags.push(Flag::new(
                    1,
                    format!(
                        "Duplicate payload field {}; front matter used",
                        quoted(&name)
                    ),
                ));
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
        flags.push(Flag::new(line, message.into()));
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
                flag(&mut flags, "splat_world", "Unused data ignored");
            }
            let mut world = Mapping::new();
            for &(old, flat) in WORLD_FIELDS {
                if let Some(mut value) = fields.remove(flat) {
                    // One group reads as a list of one, as Obsidian shows a text property.
                    if old == "part_of" && value.is_string() {
                        value = Value::Sequence(vec![value]);
                    }
                    // A key with no value is an empty list, as Obsidian shows it.
                    if old == "part_of" && value.is_null() {
                        value = Value::Sequence(Vec::new());
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
                        let (label, help) = match old {
                            "type" => ("Type", "Use one short word, such as npc or faction."),
                            "status" => ("Status", "Use a short word, such as alive."),
                            "creature" => ("Stat block", "Use one creature name or link."),
                            "in" => ("In", "Use one place name or link."),
                            "part_of" => (
                                "Part of",
                                "Write each group as a link, such as [[Arena Guild]].",
                            ),
                            "when" => ("When", "Use words such as 120 years ago."),
                            "until" => ("Until", "Use words such as 90 years ago."),
                            _ => ("Aliases", "Use a list of names."),
                        };
                        flag(&mut flags, flat, "");
                        let last = flags.last_mut().unwrap();
                        let (shown, line) = unread_value(text, last.line);
                        last.line = line;
                        last.message =
                            format!("{label} {} not read (line {line}). {help}", quoted(&shown));
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
    model_references(fields);
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

/// The text of a note: its `fields` with `changes` as the properties, then its `body`.
/// A changed property keeps its place. A new one goes to the end.
pub(super) fn note(
    fields: &Mapping,
    changes: Vec<(&str, Option<Value>)>,
    body: &str,
) -> Result<String, String> {
    let mut front = Mapping::new();
    front.insert("splat_schema".into(), 2.into());
    for (key, value) in fields {
        if key != "splat_schema" {
            front.insert(key.clone(), value.clone());
        }
    }
    for (key, value) in changes {
        match value {
            Some(value) => front.insert(key.into(), value),
            None => front.shift_remove(key),
        };
    }
    let front = serde_yaml_ng::to_string(&front).map_err(|e| e.to_string())?;
    Ok(format!("---\n{front}---\n{body}"))
}

/// The properties that hold a reference, or a list of references, as readable text.
const REFERENCE_FIELDS: [&str; 3] = ["splat_origin", "splat_sources", "splat_requires"];

/// The flat properties that replace a World note's `splat_world`.
fn world_properties(fields: &Mapping) -> Result<Vec<(&'static str, Option<Value>)>, String> {
    let Some(world) = fields.get("splat_world").and_then(Value::as_mapping) else {
        return Ok(Vec::new());
    };
    if world.keys().any(|key| {
        !WORLD_FIELDS
            .iter()
            .any(|(old, _)| key.as_str() == Some(old))
    }) {
        return Err("Unknown World properties need review before writing.".into());
    }
    let mut changes = vec![("splat_world", None)];
    for &(old, flat) in WORLD_FIELDS {
        if let Some(value) = world.get(old) {
            changes.push((flat, Some(flat_reference(value))));
        }
    }
    Ok(changes)
}

/// Each reference property of `fields`, with its references as readable text.
fn reference_properties(fields: &Mapping) -> Result<Vec<(&'static str, Option<Value>)>, String> {
    let mut changes = Vec::new();
    for key in REFERENCE_FIELDS {
        let Some(value) = fields.get(key) else {
            continue;
        };
        let flat = match value {
            Value::Sequence(values) => Value::Sequence(values.iter().map(flat_reference).collect()),
            _ => flat_reference(value),
        };
        if flat.is_mapping()
            || flat
                .as_sequence()
                .is_some_and(|values| values.iter().any(|v| !v.is_string()))
        {
            return Err(format!("{key} needs a readable reference."));
        }
        changes.push((key, Some(flat)));
    }
    Ok(changes)
}

/// Marked data for the end of a note: `value` in a YAML fence that names its `path`.
fn fence(path: &str, value: &impl serde::Serialize) -> Result<String, String> {
    Ok(format!(
        "\n<!-- splat:yaml path={path} -->\n\n```yaml\n{}```\n",
        serde_yaml_ng::to_string(value).map_err(|e| e.to_string())?
    ))
}

/// Write a schema-2 note from its model: `fields`, and the `body` below the properties, which can
/// also hold marked data. The written note must read back to the same model, or nothing is returned.
pub fn write(path: &str, fields: &Mapping, body: &str) -> Result<String, String> {
    let entry = super::model::entry(path, body, fields)?;
    let kind = entry.kind.as_str();
    if ["campaign", "encounter_group", "encounter"].contains(&kind) {
        return super::list_formats::write(&entry, fields, body);
    }
    let newline = if body.contains("\r\n") { "\r\n" } else { "\n" };
    let character = entry.fields.contains_key("splat_character");
    // The stat table holds the data of these roots. Marked data holds what the table cannot.
    let stats = super::stat_blocks::owns(kind, character)
        .then(|| super::stat_blocks::section_and_extras(&entry.fields, kind))
        .transpose()?;
    let table_roots = match stats {
        Some(_) => super::stat_blocks::data_roots(kind),
        None => &[],
    };
    let mut changes: Vec<_> = table_roots.iter().map(|root| (*root, None)).collect();
    changes.extend(world_properties(&entry.fields)?);
    changes.extend(reference_properties(&entry.fields)?);
    // Data that a property cannot show moves out of the properties: a linked character's
    // into one payload, any other note's into one fence for each field.
    let mut fences = String::new();
    let mut payload = Mapping::new();
    for (key, value) in entry.fields.iter() {
        let Some(key) = key.as_str().filter(|key| key.starts_with("splat_")) else {
            continue;
        };
        if table_roots.contains(&key) || key == "splat_world" || REFERENCE_FIELDS.contains(&key) {
            continue;
        }
        let nested = value.is_mapping()
            || value
                .as_sequence()
                .is_some_and(|a| a.iter().any(|v| v.is_mapping() || v.is_sequence()));
        let sheet_data = character
            && ![
                "splat_schema",
                "splat_id",
                "splat_kind",
                "splat_key",
                "splat_name",
            ]
            .contains(&key);
        if !(nested || sheet_data) || !fields.contains_key(key) {
            continue;
        }
        changes.push((key, None));
        if character {
            payload.insert(key.into(), value.clone());
        } else if value.as_mapping().is_some_and(Mapping::is_empty)
            && key == format!("splat_{kind}")
        {
            // An empty mapping for the note's own kind is not written.
        } else if key == "splat_reference"
            && let Some(mut reference) = value.as_mapping().cloned()
        {
            // The pages are a property.
            reference.remove("pages");
            fences.push_str(&fence(key, &reference)?);
        } else {
            fences.push_str(&fence(key, value)?);
        }
    }
    let mut body = body.to_owned();
    if let Some((section, extras)) = &stats {
        for (key, value) in extras {
            fences.push_str(&fence(key, value)?);
        }
        body = super::stat_blocks::without_marked_stats(&body, kind);
        if kind == "creature" {
            body = super::stat_blocks::fold_printed_stats(&body);
        }
        body = super::stat_blocks::insert_section(&body, section);
    }
    if character {
        fences.push_str(&fence("splat_payload", &payload)?);
    }
    if let Some(pages) = entry
        .fields
        .get("splat_reference")
        .and_then(Value::as_mapping)
        .and_then(|reference| reference.get("pages"))
    {
        changes.push(("splat_pages", Some(pages.clone())));
    }
    if !fences.is_empty() {
        let title = if character {
            "Splat data (from D&D Beyond; do not edit)"
        } else {
            "Splat data"
        };
        body.push_str(&format!("{newline}## {title}{newline}"));
        body.push_str(&fences.replace('\n', newline));
    }
    let after = folded_data(&note(fields, changes, &body)?);
    let checked = Entry::parse(path, &after)?;
    let mut read_back = (*checked.fields).clone();
    read_back.remove("splat_schema");
    let blocking_flag = checked
        .flags
        .iter()
        .any(|flag| !(flag.message.starts_with("CR ") && flag.message.ends_with(" is used")));
    if *entry.fields != read_back || blocking_flag {
        return Err("The written note does not read back to the same values.".into());
    }
    Ok(after)
}

/// Write a note from `fields` given as YAML, and the body below the properties.
/// Tests and seeded example data use this.
pub fn fixture(path: &str, fields: &str, body: &str) -> Result<String, String> {
    write(
        path,
        &serde_yaml_ng::from_str(fields).map_err(|e| e.to_string())?,
        body,
    )
}
