//! Small, lossless edits to one key in `splat_world`. Unsupported YAML stays untouched.
#![warn(deprecated)] // Its parent allows old message functions; this file does not.
use serde_yaml_ng::Value;

pub(super) fn patch(text: &str, key: &str, value: Value) -> Result<String, String> {
    let entry = splat::content::Entry::parse("World.md", text)?;

    let flat = splat::content::formats::WORLD_FIELDS
        .iter()
        .find(|(old, _)| *old == key)
        .ok_or("Unknown World field")?
        .1;
    // An empty list leaves no property behind.
    let stored = (!value.as_sequence().is_some_and(Vec::is_empty))
        .then(|| splat::content::formats::flat_reference(&value));
    if key == "part_of"
        && let Some(unread) = unread_groups(&entry, text)
    {
        return Err(unread.reason);
    }
    let output = if key == "part_of"
        && let Some(groups) = stored.as_ref().and_then(Value::as_sequence)
        && let Some(patched) = patch_groups(text, groups)?
    {
        patched
    } else {
        splat::content::patch_fields(text, &[(flat, stored)])?
    };
    let checked = splat::content::Entry::parse("World.md", &output)?;
    let mut expected = (*entry.fields).clone();
    let world = expected
        .get_mut("splat_world")
        .and_then(Value::as_mapping_mut)
        .ok_or("Missing World properties")?;
    match checked.fields["splat_world"].get(key) {
        Some(value) => world.insert(key.into(), value.clone()),
        None => world.remove(key),
    };
    if *checked.fields != expected {
        return Err("Edit changed another World value".into());
    }
    Ok(output)
}

/// A Part of list the note reader could not read: the line of the value it names, and the
/// reader's own message, which quotes that value and says how to write it.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Unread {
    pub line: usize,
    /// The reader's sentence, with a long value cut around `…` (round 25 FD4).
    pub reason: String,
    /// The same sentence with the value whole, for the warning's hover and accessible name.
    pub whole: String,
}

/// Use the parser's flag inside the Part of value, including rejected object-list values. A
/// flag on a list item belongs to the key above its block.
pub(super) fn unread_groups(entry: &splat::content::Entry, file: &str) -> Option<Unread> {
    let lines: Vec<&str> = file.lines().collect();
    entry.flags.iter().find_map(|flag| {
        let mut at = flag.line.checked_sub(1)?;
        while lines
            .get(at)
            .is_some_and(|line| line.trim().is_empty() || line.starts_with([' ', '\t', '-', '#']))
        {
            at = at.checked_sub(1)?;
        }
        lines
            .get(at)
            .and_then(|line| line.split_once(':'))
            .filter(|(key, _)| {
                serde_yaml_ng::from_str::<String>(key).is_ok_and(|key| key == "splat_part_of")
            })
            .map(|_| Unread {
                line: flag.line,
                reason: super::copy_words(flag.short_message()),
                whole: super::copy_words(&flag.message),
            })
    })
}

/// Retain the source spelling of each kept group, including aliases, quotes and comments.
fn patch_groups(text: &str, groups: &[Value]) -> Result<Option<String>, String> {
    let (_, body) = crate::sessions::split_note(text).ok_or("Missing front matter")?;
    let front_end = text.len() - body.len();
    let mut offset = 0;
    let mut start = None;
    let mut stop = front_end;
    for line in text[..front_end].split_inclusive('\n') {
        if start.is_none() {
            if line.split_once(':').is_some_and(|(key, _)| {
                serde_yaml_ng::from_str::<String>(key).is_ok_and(|key| key == "splat_part_of")
            }) {
                start = Some(offset);
            }
        } else if (!line.trim().is_empty() && !line.starts_with([' ', '\t', '-', '#']))
            || line.trim() == "---"
        {
            stop = offset;
            break;
        }
        offset += line.len();
    }
    let Some(start) = start else { return Ok(None) };
    let raw = &text[start..stop];
    let map: serde_yaml_ng::Mapping = serde_yaml_ng::from_str(raw).map_err(|e| e.to_string())?;
    let Some(raw_previous) = map.get("splat_part_of").and_then(Value::as_sequence) else {
        return Ok(None);
    };
    let entry = splat::content::Entry::parse("World.md", text)?;
    let previous = super::part_of(&entry);
    if previous.is_empty() {
        return Ok(None);
    }
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let colon = raw.find(':').unwrap() + 1;
    let mut value_start = colon;
    loop {
        value_start += raw[value_start..].len() - raw[value_start..].trim_start().len();
        if raw[value_start..].starts_with('#')
            && let Some(end) = raw[value_start..].find('\n')
        {
            value_start += end + 1;
        } else {
            break;
        }
    }
    let mut encoded = Vec::new();
    let mut expected_groups = Vec::new();
    let mut used = vec![false; previous.len()];
    let (prefix, originals, suffix, flow) = if raw[value_start..].starts_with('[') {
        // Group values are strings. Commas and ] inside quotes are part of the string.
        let mut quote = None;
        let mut escaped = false;
        let mut comment = false;
        let mut begin = value_start + 1;
        let mut pieces = Vec::new();
        let mut end = None;
        for (at, ch) in raw.char_indices().filter(|(at, _)| *at > value_start) {
            if comment {
                if ch == '\n' {
                    comment = false;
                } else {
                    continue;
                }
            }
            if ch == '#' && quote.is_none() && raw.as_bytes()[at - 1].is_ascii_whitespace() {
                comment = true;
                continue;
            }
            if !escaped && matches!(ch, '\'' | '"') {
                if quote == Some(ch) {
                    quote = None;
                } else if quote.is_none() {
                    quote = Some(ch);
                }
            }
            if quote.is_none() && matches!(ch, ',' | ']') {
                pieces.push(&raw[begin..at]);
                begin = at + 1;
                if ch == ']' {
                    end = Some(at);
                    break;
                }
            }
            escaped = ch == '\\' && quote == Some('"') && !escaped;
        }
        let end = end.ok_or("Part of could not be edited safely. Edit it in Obsidian.")?;
        (&raw[..value_start + 1], pieces, &raw[end..], true)
    } else {
        let mut starts = Vec::new();
        let mut at = 0;
        let mut indent = None;
        for line in raw.split_inclusive('\n') {
            let spaces = line.len() - line.trim_start_matches(' ').len();
            if line[spaces..].starts_with("- ") && indent.is_none_or(|indent| spaces == indent) {
                indent = Some(spaces);
                starts.push(at);
            }
            at += line.len();
        }
        let Some(first) = starts.first().copied() else {
            return Err("Part of could not be edited safely. Edit it in Obsidian.".into());
        };
        let pieces = starts
            .iter()
            .enumerate()
            .map(|(i, at)| &raw[*at..starts.get(i + 1).copied().unwrap_or(raw.len())])
            .collect();
        (&raw[..first], pieces, "", false)
    };
    if originals.len() != previous.len() {
        return Err("Part of could not be edited safely. Edit it in Obsidian.".into());
    }
    for group in groups {
        if let Some(at) = previous
            .iter()
            .enumerate()
            .position(|(at, old)| !used[at] && *old == group)
        {
            used[at] = true;
            expected_groups.push(raw_previous[at].clone());
            encoded.push(originals[at].to_owned());
        } else {
            expected_groups.push(group.clone());
            let scalar = serde_yaml_ng::to_string(group).map_err(|e| e.to_string())?;
            encoded.push(if flow {
                format!(" {}", scalar.trim_end())
            } else {
                format!(
                    "{}- {}{newline}",
                    originals[0].split('-').next().unwrap_or(""),
                    scalar.trim_end().replace('\n', &format!("{newline}  "))
                )
            });
        }
    }
    let replacement = format!(
        "{prefix}{}{suffix}",
        encoded.join(if flow { "," } else { "" })
    );
    let mut expected = map;
    expected.insert("splat_part_of".into(), Value::Sequence(expected_groups));
    let checked: serde_yaml_ng::Mapping =
        serde_yaml_ng::from_str(&replacement).map_err(|e| e.to_string())?;
    if checked != expected {
        return Err("Part of could not be edited safely. Edit it in Obsidian.".into());
    }
    Ok(Some(format!(
        "{}{replacement}{}",
        &text[..start],
        &text[stop..]
    )))
}
