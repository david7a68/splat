//! Human-readable stat tables. The table is the sole source of printed values.
//! Non-printing mechanics can remain in an extras fence; table-owned keys there
//! are ignored. All edits use the shared cell editor and check the complete model.
use super::{
    Entry, Mapping, Value,
    layout::{self, Document, Flag, FlagKind, Table},
};
use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

const ABILITIES: &[(&str, &str)] = &[
    ("Str", "strength"),
    ("Dex", "dexterity"),
    ("Con", "constitution"),
    ("Int", "intelligence"),
    ("Wis", "wisdom"),
    ("Cha", "charisma"),
];
const DETAILS: &[(&str, &str)] = &[
    ("Skills", "skills_text"),
    ("Saving Throws", "saving_throws_text"),
    ("Senses", "senses_text"),
    ("Languages", "languages_text"),
    ("Gear", "gear_text"),
    ("Resistances", "resistances_text"),
    ("Immunities", "immunities_text"),
    ("Vulnerabilities", "vulnerabilities_text"),
    // The 2024 Monster Manual prints this under the stat block; the SRD does not.
    ("Habitat", "habitat_text"),
];
const CREATURE_ROWS: &[&str] = &[
    "Size",
    "Type",
    "Alignment",
    "Armor Class",
    "Hit Points",
    "Initiative",
    "Speed",
    "Challenge",
];
const SPELL_ROWS: &[&str] = &[
    "Level",
    "School",
    "Classes",
    "Casting Time",
    "Range",
    "Components",
    "Duration",
];
const ITEM_ROWS: &[&str] = &["Type", "Rarity", "Attunement"];

pub fn owns(kind: &str, character: bool) -> bool {
    !character && ["creature", "spell", "item"].contains(&kind)
}
fn roots(kind: &str) -> &'static [&'static str] {
    match kind {
        "creature" => &["splat_creature", "splat_stats"],
        "spell" => &["splat_spell"],
        "item" => &["splat_item"],
        _ => &[],
    }
}
fn section(kind: &str) -> &'static str {
    match kind {
        "creature" => "Stat block",
        "spell" => "Spell",
        _ => "Item",
    }
}
fn headers(kind: &str) -> &'static [&'static str] {
    if kind == "creature" {
        &["Stat", "Value"]
    } else {
        &["Field", "Value"]
    }
}
fn stat(map: &mut Mapping, key: &str, value: i64) {
    map.insert(
        key.into(),
        Value::Mapping([("value".into(), value.into())].into_iter().collect()),
    );
}
fn number(raw: &str, signed: bool) -> Result<i64, String> {
    let token = raw.split_whitespace().next().unwrap_or("");
    if signed && !token.starts_with(['+', '-', '−']) {
        return Err("needs a signed number".into());
    }
    token
        .replace('−', "-")
        .parse::<i64>()
        .map_err(|_| "needs a number first".into())
}
fn read(table: &Table, names: &[&str], flags: &mut Vec<Flag>) -> Option<String> {
    // Treat aliases as the same row, so the first occurrence always wins.
    table
        .read_any(names, 1, flags, |v| Ok(v.replace("\\|", "|")))
        .filter(|v| !v.is_empty())
}
fn checked<T>(
    raw: Option<String>,
    name: &str,
    table: &Table,
    flags: &mut Vec<Flag>,
    parse: impl FnOnce(&str) -> Result<T, String>,
) -> Option<T> {
    let raw = raw?;
    match parse(&raw) {
        Ok(v) => Some(v),
        Err(message) => {
            let line = table
                .row(name)
                .or_else(|| {
                    table.row(match name {
                        "Hit Points" => "HP",
                        "Armor Class" => "AC",
                        "Challenge" => "CR",
                        _ => name,
                    })
                })
                .map_or(table.header[0].line, |r| r[0].line);
            let fix = match (name, message.as_str()) {
                // Hit Points often holds a note after the number, so the hint shows that shape.
                ("Hit Points", "needs a number first") => {
                    "Start with a number, such as 22 (4d8 + 4).".to_owned()
                }
                (_, "needs a number first") => "Use a number first.".to_owned(),
                (_, message) => format!(
                    "Use {}.",
                    message
                        .strip_prefix("needs ")
                        .unwrap_or(message)
                        .trim_end_matches('.')
                ),
            };
            let field = match name {
                "Hit Points" => layout::Field::HitPoints,
                "Rarity" => layout::Field::Rarity,
                _ => layout::Field::Other,
            };
            flags.push(Flag::unread(field, line, name, &raw, &fix));
            None
        }
    }
}
fn text(map: &mut Mapping, key: &str, value: Option<String>) {
    if let Some(v) = value {
        map.insert(key.into(), v.into());
    }
}
fn signed(n: i64) -> String {
    format!("{n:+}")
}
fn val<'a>(fields: &'a Mapping, root: &str, key: &str) -> Option<&'a Value> {
    fields.get(root)?.get(key)
}
fn string(fields: &Mapping, root: &str, key: &str) -> String {
    val(fields, root, key)
        .and_then(Value::as_str)
        .unwrap_or("")
        .into()
}
fn numeric(fields: &Mapping, key: &str) -> Option<i64> {
    val(fields, "splat_stats", key)?.get("value")?.as_i64()
}

/// Remove marked data blocks for the stat roots, and the headings they leave empty.
/// The stat table owns these values, so stale marked data cannot mask it.
pub(super) fn without_marked_stats(text: &str, kind: &str) -> String {
    // Only a marker naming one of these roots changes the text. Most notes have none,
    // and two full Markdown parses would find nothing.
    if !roots(kind).iter().any(|root| {
        text.contains(format!("path={root} ").as_str())
            || text.contains(format!("path={root}/").as_str())
    }) {
        return text.to_owned();
    }
    let mut ranges = Vec::new();
    let mut pending = None;
    let mut heading = None;
    let mut touched = std::collections::HashSet::new();
    for (event, range) in Parser::new_ext(text, Options::ENABLE_TABLES).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                heading = Some(text[range].trim().trim_start_matches('#').trim().to_owned());
            }
            Event::Html(ref raw)
                if raw.starts_with("<!-- splat:")
                    && roots(kind).iter().any(|root| {
                        raw.contains(&format!("path={root} "))
                            || raw.contains(&format!("path={root}/"))
                    }) =>
            {
                pending = Some(range.start);
                if let Some(heading) = &heading {
                    touched.insert(heading.clone());
                }
            }
            Event::End(TagEnd::Table) | Event::End(TagEnd::CodeBlock) => {
                if let Some(start) = pending.take() {
                    ranges.push(start..range.end);
                }
            }
            _ => {}
        }
    }
    let mut out = text.to_owned();
    for range in ranges.into_iter().rev() {
        out.replace_range(range, "");
    }
    let headings: Vec<_> = Parser::new(&out)
        .into_offset_iter()
        .filter_map(|(event, range)| {
            matches!(event, Event::Start(Tag::Heading { .. })).then_some(range)
        })
        .collect();
    for (index, range) in headings.iter().enumerate().rev() {
        let end = headings.get(index + 1).map_or(out.len(), |next| next.start);
        let name = out[range.clone()].trim().trim_start_matches('#').trim();
        if touched.contains(name) && out[range.end..end].trim().is_empty() {
            out.replace_range(range.clone(), "");
        }
    }
    out
}

const PRINTED_BLOCK: &str = "> [!quote]- Printed stat block (not read; edit the table above)";

/// Keep the book's printed header as a record, separate from the editable table.
pub fn fold_printed_stats(text: &str) -> String {
    let Some(source) =
        Document::new(text).section_at_level("Source text", Some(pulldown_cmark::HeadingLevel::H2))
    else {
        return text.into();
    };
    let source = source.range();
    let mut ranges = Vec::new();
    let mut offset = source.start;
    for raw in text[source].split_inclusive('\n') {
        let line = raw.trim().trim_end_matches('\\').trim();
        if line.starts_with('#') {
            break;
        }
        let plain = line.replace('*', "");
        let identity = plain.split_once(' ').is_some_and(|(size, rest)| {
            identity_value("size", size).is_some()
                && identity_value(
                    "type",
                    rest.split_whitespace()
                        .next()
                        .unwrap_or(rest)
                        .trim_end_matches(','),
                )
                .is_some()
        });
        let stat = [
            "AC ",
            "HP ",
            "Armor Class ",
            "Hit Points ",
            "Initiative ",
            "Speed ",
            "CR ",
            "Challenge ",
            "MOD SAVE",
            "Str ",
            "Dex ",
            "Con ",
            "Int ",
            "Wis ",
            "Cha ",
            "Skills ",
            "Saving Throws ",
            "Senses ",
            "Languages ",
            "Gear ",
            "Resistances ",
            "Immunities ",
            "Vulnerabilities ",
        ]
        .iter()
        .any(|prefix| plain.starts_with(prefix));
        if identity || stat {
            if let Some(last) = ranges
                .last_mut()
                .filter(|last: &&mut std::ops::Range<usize>| last.end == offset)
            {
                last.end += raw.len();
            } else {
                ranges.push(offset..offset + raw.len());
            }
        }
        offset += raw.len();
    }
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let mut out = text.to_owned();
    for range in ranges.into_iter().rev() {
        let quote = text[range.clone()]
            .lines()
            .map(|line| format!("> {line}{newline}"))
            .collect::<String>();
        out.replace_range(range, &format!("{PRINTED_BLOCK}{newline}{quote}{newline}"));
    }
    out
}

pub fn read_tables(kind: &str, source: &str, fields: &mut Mapping, flags: &mut Vec<Flag>) {
    let mut parsed = Mapping::new();
    let table = Document::new(source)
        .section_at_level(section(kind), Some(pulldown_cmark::HeadingLevel::H2))
        .and_then(|s| s.table(headers(kind)));
    for root in roots(kind) {
        if fields.remove(*root).is_some()
            || (source.contains(&format!("path={root} "))
                || source.contains(&format!("path={root}/")))
        {
            let line = source
                .lines()
                .position(|l| l.contains(root))
                .map_or(1, |i| i + 1);
            flags.push(Flag::new(line, "Unused data ignored".into()));
        }
        parsed.insert((*root).into(), Value::Mapping(Mapping::new()));
    }
    if let Some(ref table) = table {
        match kind {
            "creature" => creature(table, source, &mut parsed, flags),
            "spell" => spell(table, &mut parsed, flags),
            _ => item(table, &mut parsed, flags),
        }
    } else {
        flags.push(Flag::new(
            source
                .lines()
                .position(|l| l.eq_ignore_ascii_case(&format!("## {}", section(kind))))
                .map_or(1, |i| i + 1),
            format!("{} table not found; values not set", section(kind)),
        ));
    }
    for shape in metadata(source, kind, "shape") {
        if shape.split(',').any(|s| s == "size_literal")
            && let Some(creature) = parsed
                .get_mut("splat_creature")
                .and_then(Value::as_mapping_mut)
            && let Some(raw) = creature
                .remove("size_options_text")
                .and_then(|v| v.as_str().map(str::to_owned))
        {
            creature.insert("size".into(), raw.to_lowercase().into());
        }
        if shape.split(',').any(|s| s == "type_literal")
            && let Some(creature) = parsed
                .get_mut("splat_creature")
                .and_then(Value::as_mapping_mut)
            && let Some(raw) = table
                .as_ref()
                .and_then(|t| read(t, &["Type"], &mut Vec::new()))
        {
            creature.remove("type_options_text");
            creature.remove("tags_text");
            creature.insert("type".into(), raw.into());
            for (row, key) in [
                ("Type Tags", "tags_text"),
                ("Type Options", "type_options_text"),
            ] {
                if let Some(raw) = table
                    .as_ref()
                    .and_then(|t| read(t, &[row], &mut Vec::new()))
                {
                    creature.insert(key.into(), raw.into());
                }
            }
        }
        if shape.split(',').any(|s| s == "rarity_text")
            && let Some(item) = parsed.get_mut("splat_item").and_then(Value::as_mapping_mut)
            && let Some(raw) = table
                .as_ref()
                .and_then(|t| read(t, &["Rarity"], &mut Vec::new()))
        {
            item.insert("rarity".into(), raw.to_lowercase().replace(' ', "_").into());
            if let Some(row) = table.as_ref().and_then(|t| t.row("Rarity")) {
                flags.retain(|f| {
                    f.line != row[0].line || f.kind != FlagKind::Unread(layout::Field::Rarity)
                });
            }
        }
        if let Some(spell) = parsed
            .get_mut("splat_spell")
            .and_then(Value::as_mapping_mut)
        {
            if shape.split(',').any(|s| s == "range_text")
                && let Some(raw) = table
                    .as_ref()
                    .and_then(|t| read(t, &["Range"], &mut Vec::new()))
            {
                spell.remove("range");
                spell.insert("range_text".into(), raw.into());
            }
            if shape.split(',').any(|s| s == "casting_time")
                && let Some(raw) = spell
                    .remove("casting_time")
                    .and_then(|v| v.as_str().map(str::to_owned))
            {
                let (amount, unit) = raw
                    .split_once(' ')
                    .and_then(|(a, u)| a.parse::<u64>().ok().map(|n| (Value::from(n), u)))
                    .unwrap_or((Value::Null, &raw));
                spell.insert(
                    "casting_time".into(),
                    Value::Mapping(
                        [("amount".into(), amount), ("unit".into(), unit.into())]
                            .into_iter()
                            .collect(),
                    ),
                );
            }
            if shape.split(',').any(|s| s == "duration_plain")
                && let Some(raw) = spell.get_mut("duration")
                && let Some(text) = raw.as_str().map(str::to_owned)
            {
                *raw = text
                    .strip_prefix("Concentration,")
                    .unwrap_or(&text)
                    .trim_start()
                    .into();
            }
            if shape.split(',').any(|s| s == "material_text")
                && let Some(raw) = spell
                    .remove("material")
                    .and_then(|v| v.get("description").cloned())
            {
                spell.insert("material".into(), raw);
            }
        }
    }
    // Extras hold only mechanics the printed tables cannot express. A stale
    // copy of a table-owned key is flagged, never used as a fallback.
    for root in roots(kind) {
        let extra_key = format!("{root}_extras");
        if let Some(Value::Mapping(mut extra)) = fields.remove(extra_key.as_str()) {
            reject_owned(&mut extra, root, flags);
            let target = parsed.get_mut(*root).unwrap().as_mapping_mut().unwrap();
            merge_extra(target, extra);
        }
    }
    for paths in metadata(source, kind, "omit") {
        {
            for path in paths.split(',') {
                if OPTIONAL_PATHS.contains(&path) {
                    remove_optional(&mut parsed, path);
                }
            }
        }
    }
    fields.extend(parsed);
}

fn metadata<'a>(source: &'a str, kind: &str, key: &str) -> Vec<&'a str> {
    let prefix = format!("%% splat {key}=");
    // Most notes have no metadata. Finding the section needs a full Markdown parse.
    if !source.contains(&prefix) {
        return Vec::new();
    }
    let Some(section) = Document::new(source)
        .section_at_level(section(kind), Some(pulldown_cmark::HeadingLevel::H2))
    else {
        return Vec::new();
    };
    let range = section.range();
    Parser::new(&source[range.clone()])
        .into_offset_iter()
        .filter_map(|(event, span)| {
            if matches!(event, Event::Start(Tag::Paragraph)) {
                source[range.start + span.start..range.start + span.end]
                    .trim()
                    .strip_prefix(&prefix)
                    .and_then(|s| s.strip_suffix(" %%"))
            } else {
                None
            }
        })
        .collect()
}

const IDENTITY: &[(&str, &str, &[&str])] = &[
    (
        "Size",
        "size",
        &["tiny", "small", "medium", "large", "huge", "gargantuan"],
    ),
    (
        "Type",
        "type",
        &[
            "aberration",
            "beast",
            "celestial",
            "construct",
            "dragon",
            "elemental",
            "fey",
            "fiend",
            "giant",
            "humanoid",
            "monstrosity",
            "ooze",
            "plant",
            "undead",
        ],
    ),
];

pub(super) fn identity_value(key: &str, raw: &str) -> Option<Mapping> {
    let (_, _, allowed) = IDENTITY.iter().find(|(_, k, _)| *k == key)?;
    let lower = raw.to_lowercase();
    let valid = if key == "size" {
        let lower = lower
            .replace(" swarm of ", " or ")
            .replace(" or smaller", "");
        lower.split(" or ").all(|s| allowed.contains(&s))
    } else {
        let head = lower.split(" (").next().unwrap_or("");
        head.replace(", or ", ", ")
            .replace(" or ", ", ")
            .split(", ")
            .all(|s| allowed.contains(&s))
    };
    if !valid {
        return None;
    }
    let mut creature = Mapping::new();
    let first = lower
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches(',');
    if key == "size" && lower != first {
        creature.insert("size".into(), first.into());
        creature.insert("size_options_text".into(), raw.into());
    } else if key == "type" && lower.contains(" or ") {
        creature.insert("type".into(), first.into());
        creature.insert("type_options_text".into(), raw.into());
    } else if key == "type"
        && let Some((kind, tags)) = raw.split_once(" (")
    {
        creature.insert("type".into(), kind.to_lowercase().into());
        creature.insert("tags_text".into(), format!("({tags}").into());
    } else {
        creature.insert(key.into(), lower.into());
    }
    Some(creature)
}

fn speed(raw: &str) -> Result<(i64, &str), String> {
    let raw = raw.trim();
    let end = raw.find(|c: char| !c.is_ascii_digit()).unwrap_or(raw.len());
    let n = raw[..end]
        .parse()
        .map_err(|_| "needs a number followed by ft.".to_owned())?;
    let units = raw[end..].trim_start();
    if units.is_empty() {
        return Ok((n, ""));
    }
    let tail = ["feet", "ft.", "ft"]
        .iter()
        .find_map(|unit| {
            units
                .strip_prefix(unit)
                .filter(|tail| tail.is_empty() || tail.starts_with([',', ' ']))
        })
        .ok_or("needs a number followed by ft.")?;
    Ok((n, tail))
}

/// Standard CR rewards. Explicit printed values still take precedence.
fn challenge_defaults(raw: &str) -> Option<(u64, i64)> {
    const XP: &[u64] = &[
        10, 200, 450, 700, 1100, 1800, 2300, 2900, 3900, 5000, 5900, 7200, 8400, 10000, 11500,
        13000, 15000, 18000, 20000, 22000, 25000, 33000, 41000, 50000, 62000, 75000, 90000, 105000,
        120000, 135000, 155000,
    ];
    let cr = raw.split_whitespace().next()?;
    match cr {
        "1/8" => Some((25, 2)),
        "1/4" => Some((50, 2)),
        "1/2" => Some((100, 2)),
        _ => {
            let cr = cr.parse::<usize>().ok()?;
            Some((*XP.get(cr)?, 2 + cr.saturating_sub(1) as i64 / 4))
        }
    }
}

fn printed_xp(raw: &str) -> Option<u64> {
    let raw = raw.to_uppercase();
    let digits = if let Some((_, tail)) = raw.split_once("XP ") {
        tail.chars()
            .take_while(|c| c.is_ascii_digit() || *c == ',')
            .collect::<String>()
    } else {
        raw.split_once(" XP")?
            .0
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_digit() || *c == ',')
            .collect::<String>()
            .chars()
            .rev()
            .collect()
    };
    digits.replace(',', "").parse().ok()
}

fn printed_pb(raw: &str) -> Option<i64> {
    number(
        raw.to_uppercase()
            .split_once("PB ")?
            .1
            .split([';', ')'])
            .next()?
            .trim(),
        true,
    )
    .ok()
}

pub fn challenge_xp(raw: &str) -> Option<u64> {
    printed_xp(raw).or_else(|| challenge_defaults(raw).map(|(xp, _)| xp))
}

fn complete_challenge(raw: &str) -> String {
    let Some((xp, pb)) = challenge_defaults(raw) else {
        return raw.into();
    };
    let mut missing = Vec::new();
    if printed_xp(raw).is_none() {
        missing.push(format!("XP {xp}"));
    }
    if printed_pb(raw).is_none() {
        missing.push(format!("PB {pb:+}"));
    }
    if missing.is_empty() {
        return raw.into();
    }
    if let Some(prefix) = raw.trim_end().strip_suffix(')') {
        format!("{prefix}; {})", missing.join("; "))
    } else {
        format!("{raw} ({})", missing.join("; "))
    }
}

pub(crate) fn complete_challenge_fields(fields: &mut Mapping) {
    if let Some(creature) = fields
        .get_mut("splat_creature")
        .and_then(Value::as_mapping_mut)
        && let Some(raw) = creature.get("cr_text").and_then(Value::as_str)
    {
        let completed = complete_challenge(raw);
        creature.insert("cr_text".into(), completed.into());
    }
}

fn creature(table: &Table, source: &str, fields: &mut Mapping, flags: &mut Vec<Flag>) {
    let shapes = metadata(source, "creature", "shape");
    let mut creature = Mapping::new();
    let mut stats = Mapping::new();
    for (name, key, _) in IDENTITY {
        let raw = read(table, &[name], flags);
        if let Some(identity) = checked(raw, name, table, flags, |s| {
            identity_value(key, s).ok_or_else(|| "needs a known size or creature type".into())
        }) {
            creature.extend(identity);
        }
    }
    text(
        &mut creature,
        "alignment",
        read(table, &["Alignment"], flags),
    );
    for (names, key, prose) in [
        (
            &["Armor Class", "AC"][..],
            "armor_class",
            "armor_class_text",
        ),
        (&["Hit Points", "HP"][..], "max_hp", "hit_points_text"),
        (&["Initiative"][..], "initiative", ""),
    ] {
        let raw = read(table, names, flags);
        if let Some(expression) = raw.as_deref().and_then(|raw| raw.strip_prefix("= ")) {
            match serde_yaml_ng::from_str::<Value>(expression) {
                Ok(expression) if expression.is_mapping() => {
                    stats.insert(
                        key.into(),
                        Value::Mapping([("derive".into(), expression)].into_iter().collect()),
                    );
                }
                _ => flags.push(Flag::new(
                    table.row(names[0]).map_or(1, |row| row[0].line),
                    format!("{} formula not read", names[0]),
                )),
            }
            continue;
        }
        if shapes.iter().any(|s| {
            s.split(',').any(|s| {
                (key == "max_hp" && s == "hp_text") || (key == "armor_class" && s == "ac_text")
            })
        }) {
            text(&mut creature, prose, raw);
            continue;
        }
        if let Some((n, raw)) = checked(raw, names[0], table, flags, |s| {
            let n = number(s, key == "initiative")?;
            if key != "initiative" && n < 0 {
                return Err("needs a nonnegative number".into());
            }
            Ok((n, s.to_owned()))
        }) {
            stat(&mut stats, key, n);
            if !prose.is_empty() && raw.trim().parse::<i64>().is_err() {
                creature.insert(prose.into(), raw.clone().into());
            }
        }
    }
    let raw = read(table, &["Speed"], flags);
    if let Some(raw) = checked(raw, "Speed", table, flags, |s| {
        let (n, tail) = speed(s)?;
        if shapes
            .iter()
            .any(|s| s.split(',').any(|s| s == "speed_text"))
        {
            return Ok(s.to_owned());
        }
        let n = if shapes
            .iter()
            .any(|s| s.split(',').any(|s| s == "speed_max"))
        {
            s.split(',')
                .filter_map(|part| {
                    speed(
                        part.trim()
                            .trim_start_matches(|c: char| !c.is_ascii_digit()),
                    )
                    .ok()
                    .map(|(n, _)| n)
                })
                .max()
                .unwrap_or(n)
        } else {
            n
        };
        stat(&mut stats, "speed", n);
        stats
            .get_mut("speed")
            .unwrap()
            .as_mapping_mut()
            .unwrap()
            .insert("unit".into(), "ft".into());
        Ok(
            if shapes
                .iter()
                .any(|s| s.split(',').any(|s| s == "speed_max"))
            {
                s.to_owned()
            } else if s.parse::<i64>().is_ok() {
                n.to_string()
            } else {
                format!("{n} ft.{tail}")
            },
        )
    }) && raw.parse::<i64>().is_err()
    {
        creature.insert("movement_text".into(), raw.into());
    }
    let raw = read(table, &["Challenge", "CR"], flags);
    if let Some(raw) = checked(raw, "Challenge", table, flags, |s| {
        let cr = s.split_whitespace().next().unwrap_or("");
        let valid = cr.eq_ignore_ascii_case("none")
            || ["1/8", "1/4", "1/2"].contains(&cr)
            || cr.parse::<u8>().is_ok_and(|n| n <= 30);
        valid
            .then(|| s.to_owned())
            .ok_or_else(|| "needs a challenge rating first".into())
    }) {
        if !raw.to_lowercase().contains("lair")
            && let Some((xp, pb)) = challenge_defaults(&raw)
        {
            let cr = raw.split_whitespace().next().unwrap();
            let line = table.row("Challenge").or_else(|| table.row("CR")).unwrap()[0].line;
            if let Some(printed) = printed_xp(&raw)
                && printed != xp
                && !metadata(source, "creature", "reward").contains(&"printed")
                // CR 0 can award either 0 or 10 XP in the books.
                && !(cr == "0" && printed == 0)
            {
                flags.push(Flag::of(
                    FlagKind::PrintedValueUsed,
                    line,
                    format!("CR {cr} is XP {xp}; XP {printed} is used"),
                ));
            }
            if let Some(printed) = printed_pb(&raw)
                && printed != pb
            {
                flags.push(Flag::of(
                    FlagKind::PrintedValueUsed,
                    line,
                    format!("CR {cr} is PB {pb:+}; PB {printed:+} is used"),
                ));
            }
        }
        creature.insert("cr_text".into(), complete_challenge(&raw).into());
    }
    for (name, key) in DETAILS {
        text(&mut creature, key, read(table, &[*name], flags));
    }
    if let Some(table) = Document::new(source)
        .section_at_level("Stat block", Some(pulldown_cmark::HeadingLevel::H2))
        .and_then(|s| s.table(&["Ability", "Score", "Mod", "Save"]))
    {
        for (short, key) in ABILITIES {
            let mut table = table.clone();
            for row in &mut table.rows {
                if let Some(cell) = row.first_mut()
                    && layout::label(&cell.value) == *key
                {
                    cell.value = (*short).into();
                }
            }
            let score = table
                .read(short, 1, flags, |raw| {
                    if raw.is_empty() {
                        return Ok(None);
                    }
                    raw.parse::<i64>()
                        .ok()
                        .filter(|n| (1..=30).contains(n))
                        .map(Some)
                        .ok_or_else(|| format!("{short} Score needs a number from 1 to 30"))
                })
                .flatten();
            if let Some(score) = score {
                stat(&mut stats, key, score);
                let row = table.row(short).unwrap();
                if !row[3].value.is_empty() {
                    match number(&row[3].value, true) {
                        Ok(n) if row[3].value.split_whitespace().count() == 1 => {
                            stat(&mut stats, &format!("save_{key}"), n)
                        }
                        _ => flags.push(Flag::new(
                            row[3].line,
                            format!("{short} Save needs a signed number"),
                        )),
                    }
                }
            }
        }
    }
    fields.insert("splat_creature".into(), creature.into());
    fields.insert("splat_stats".into(), stats.into());
}
fn spell(table: &Table, fields: &mut Mapping, flags: &mut Vec<Flag>) {
    let mut map = Mapping::new();
    let raw = read(table, &["Level"], flags);
    if let Some(n) = checked(raw, "Level", table, flags, |s| {
        if s.eq_ignore_ascii_case("cantrip") {
            return Ok(0);
        }
        s.parse::<u8>()
            .ok()
            .filter(|n| *n <= 9)
            .ok_or_else(|| "needs a number from 0 to 9".into())
    }) {
        map.insert("level".into(), n.into());
    }
    let raw = read(table, &["School"], flags);
    text(
        &mut map,
        "school",
        checked(raw, "School", table, flags, |s| {
            let s = s.to_lowercase();
            [
                "abjuration",
                "conjuration",
                "divination",
                "enchantment",
                "evocation",
                "illusion",
                "necromancy",
                "transmutation",
            ]
            .contains(&s.as_str())
            .then_some(s)
            .ok_or_else(|| "needs a known school".into())
        }),
    );
    if let Some(raw) = read(table, &["Classes"], flags) {
        map.insert(
            "classes".into(),
            Value::Sequence(
                raw.split(',')
                    .map(|s| Value::from(s.trim().to_lowercase()))
                    .collect(),
            ),
        );
    }
    if let Some(raw) = read(table, &["Casting Time"], flags) {
        if raw.to_lowercase().contains("ritual") {
            map.insert("ritual".into(), true.into());
        }
        map.entry(Value::from("ritual")).or_insert(false.into());
        map.insert(
            "casting_time".into(),
            raw.trim_end_matches(" (Ritual)").into(),
        );
    }
    if let Some(raw) = read(table, &["Range"], flags) {
        if let Some(n) = raw
            .strip_suffix(" feet")
            .or_else(|| raw.strip_suffix(" ft."))
            .and_then(|s| s.parse::<u64>().ok())
        {
            map.insert(
                "range".into(),
                Value::Mapping([("feet".into(), n.into())].into_iter().collect()),
            );
        } else {
            map.insert("range_text".into(), raw.into());
        }
    }
    if let Some(raw) = read(table, &["Range Text"], flags) {
        map.insert("range_text".into(), raw.into());
    }
    let raw = read(table, &["Components"], flags);
    if let Some((components, material)) = checked(raw, "Components", table, flags, |s| {
        let (head, material) = s.split_once('(').map_or((s, None), |(head, tail)| {
            (head.trim_end_matches([' ', ',']), tail.strip_suffix(')'))
        });
        let mut components = Vec::new();
        for word in head.split(',').map(str::trim) {
            components.push(match word.to_uppercase().as_str() {
                "V" => "verbal".into(),
                "S" => "somatic".into(),
                "M" => "material".into(),
                _ => return Err("needs V, S, or M, with optional material in parentheses".into()),
            });
        }
        if s.contains('(') && material.is_none() {
            return Err("needs a closing parenthesis".into());
        }
        Ok((components, material.map(str::to_owned)))
    }) {
        map.insert("components".into(), Value::Sequence(components));
        if let Some(material) = material {
            map.insert(
                "material".into(),
                Value::Mapping(
                    [("description".into(), material.into())]
                        .into_iter()
                        .collect(),
                ),
            );
        }
    }
    if let Some(raw) = read(table, &["Duration"], flags) {
        if raw.to_lowercase().starts_with("concentration") {
            map.insert("concentration".into(), true.into());
        }
        map.entry(Value::from("concentration"))
            .or_insert(false.into());
        map.insert("duration".into(), raw.into());
    }
    for (row, key) in [("Ritual", "ritual"), ("Concentration", "concentration")] {
        let raw = read(table, &[row], flags);
        if let Some(value) = checked(raw, row, table, flags, |s| {
            match s.to_ascii_lowercase().as_str() {
                "yes" | "true" => Ok(true),
                "no" | "false" => Ok(false),
                _ => Err("needs Yes or No".into()),
            }
        }) {
            map.insert(key.into(), value.into());
        }
    }
    fields.insert("splat_spell".into(), map.into());
}
fn item(table: &Table, fields: &mut Mapping, flags: &mut Vec<Flag>) {
    let mut map = Mapping::new();
    text(
        &mut map,
        "category",
        read(table, &["Type"], flags).map(|s| s.to_lowercase().replace(' ', "_")),
    );
    let raw = read(table, &["Rarity"], flags);
    text(
        &mut map,
        "rarity",
        checked(raw, "Rarity", table, flags, |s| {
            let s = s.to_lowercase().replace('_', " ");
            [
                "common",
                "uncommon",
                "rare",
                "very rare",
                "legendary",
                "artifact",
                "varies",
                "rarity varies",
                "same",
            ]
            .contains(&s.as_str())
            .then(|| s.replace(' ', "_"))
            .ok_or_else(|| "needs a known rarity".into())
        }),
    );
    if let Some(raw) = read(table, &["Attunement"], flags) {
        let value = if raw.eq_ignore_ascii_case("no") || raw.eq_ignore_ascii_case("none") {
            "none".into()
        } else if raw.eq_ignore_ascii_case("yes") || raw.eq_ignore_ascii_case("required") {
            "required".into()
        } else {
            raw
        };
        map.insert(
            "equipment".into(),
            Value::Mapping([("attunement".into(), value.into())].into_iter().collect()),
        );
    }
    fields.insert("splat_item".into(), map.into());
}

fn owned_keys(root: &str) -> Vec<&'static str> {
    match root {
        "splat_creature" => [
            "size",
            "size_options_text",
            "type_options_text",
            "tags_text",
            "type",
            "alignment",
            "armor_class_text",
            "hit_points_text",
            "movement_text",
            "cr_text",
        ]
        .into_iter()
        .chain(DETAILS.iter().map(|(_, key)| *key))
        .collect(),
        "splat_stats" => ["armor_class", "max_hp", "initiative", "speed"]
            .into_iter()
            .chain(ABILITIES.iter().flat_map(|(_, key)| {
                [
                    *key,
                    match *key {
                        "strength" => "save_strength",
                        "dexterity" => "save_dexterity",
                        "constitution" => "save_constitution",
                        "intelligence" => "save_intelligence",
                        "wisdom" => "save_wisdom",
                        _ => "save_charisma",
                    },
                ]
            }))
            .collect(),
        "splat_spell" => vec![
            "level",
            "school",
            "classes",
            "casting_time",
            "range",
            "range_text",
            "components",
            "material",
            "duration",
            "ritual",
            "concentration",
        ],
        _ => vec!["category", "rarity", "equipment"],
    }
}

/// The execution adapter ignores known reader text, while retaining unknown
/// keys for its strict mechanics validator to reject.
pub(crate) fn creature_mechanics(fields: &Mapping) -> Mapping {
    let display = owned_keys("splat_creature");
    fields
        .iter()
        .filter(|(key, _)| {
            key.as_str()
                .is_none_or(|key| ["size", "type"].contains(&key) || !display.contains(&key))
        })
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect()
}

fn printed_identity(value: String) -> String {
    let mut chars = value.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

fn title_words(raw: String) -> String {
    raw.replace('_', " ")
        .split_whitespace()
        .map(|word| printed_identity(word.into()))
        .collect::<Vec<_>>()
        .join(" ")
}

fn literal_type(fields: &Mapping) -> bool {
    let raw = string(fields, "splat_creature", "type");
    identity_value("type", &raw)
        .is_some_and(|m| m.get("type").and_then(Value::as_str) != Some(raw.as_str()))
}

pub fn render(fields: &Mapping, kind: &str) -> String {
    let mut rows: Vec<(&str, String)> = Vec::new();
    match kind {
        "creature" => {
            for (name, key) in [
                ("Size", "size"),
                ("Type", "type"),
                ("Alignment", "alignment"),
            ] {
                let value = if name == "Size" {
                    val(fields, "splat_creature", "size_options_text")
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                        .unwrap_or_else(|| printed_identity(string(fields, "splat_creature", key)))
                } else if name == "Type" && literal_type(fields) {
                    string(fields, "splat_creature", "type")
                } else if name == "Type" {
                    let mut value = val(fields, "splat_creature", "type_options_text")
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                        .unwrap_or_else(|| printed_identity(string(fields, "splat_creature", key)));
                    let tags = string(fields, "splat_creature", "tags_text");
                    if !tags.is_empty() {
                        value.push(' ');
                        value.push_str(&tags);
                    }
                    value
                } else {
                    string(fields, "splat_creature", key)
                };
                if name != "Alignment" || !value.is_empty() {
                    rows.push((name, value));
                }
            }
            if literal_type(fields) {
                for (row, key) in [
                    ("Type Tags", "tags_text"),
                    ("Type Options", "type_options_text"),
                ] {
                    if let Some(raw) = val(fields, "splat_creature", key).and_then(Value::as_str) {
                        rows.push((row, raw.into()));
                    }
                }
            }
            for (name, key, prose) in [
                ("Armor Class", "armor_class", "armor_class_text"),
                ("Hit Points", "max_hp", "hit_points_text"),
                ("Initiative", "initiative", ""),
                ("Speed", "speed", "movement_text"),
            ] {
                let value = numeric(fields, key).map_or_else(
                    || {
                        val(fields, "splat_stats", key)
                            .and_then(|s| s.get("derive"))
                            .map(|expression| {
                                format!("= {}", serde_json::to_string(expression).expect("formula"))
                            })
                            .unwrap_or_else(|| string(fields, "splat_creature", prose))
                    },
                    |n| {
                        let printed = string(fields, "splat_creature", prose);
                        if !printed.is_empty() {
                            printed
                        } else if key == "initiative" {
                            n.checked_add(10)
                                .map_or_else(|| signed(n), |passive| format!("{n:+} ({passive})"))
                        } else {
                            n.to_string()
                        }
                    },
                );
                rows.push((name, value));
            }
            rows.push(("Challenge", string(fields, "splat_creature", "cr_text")));
            for (name, key) in DETAILS {
                let value = string(fields, "splat_creature", key);
                if !value.is_empty() {
                    rows.push((name, value));
                }
            }
        }
        "spell" => {
            for name in SPELL_ROWS {
                let value = match *name {
                    "Level" => val(fields, "splat_spell", "level")
                        .and_then(Value::as_u64)
                        .map_or_else(String::new, |n| {
                            if n == 0 {
                                "Cantrip".into()
                            } else {
                                n.to_string()
                            }
                        }),
                    "School" => title_words(string(fields, "splat_spell", "school")),
                    "Classes" => val(fields, "splat_spell", "classes")
                        .and_then(Value::as_sequence)
                        .map(|s| {
                            s.iter()
                                .filter_map(Value::as_str)
                                .map(|value| title_words(value.into()))
                                .collect::<Vec<_>>()
                                .join(", ")
                        })
                        .unwrap_or_default(),
                    "Casting Time" => {
                        let mut s = string(fields, "splat_spell", "casting_time");
                        if let Some(time) =
                            val(fields, "splat_spell", "casting_time").and_then(Value::as_mapping)
                        {
                            let amount = time
                                .get("amount")
                                .and_then(Value::as_u64)
                                .map(|n| format!("{n} "))
                                .unwrap_or_default();
                            s = format!(
                                "{amount}{}",
                                time.get("unit").and_then(Value::as_str).unwrap_or("")
                            );
                        }
                        if val(fields, "splat_spell", "ritual").and_then(Value::as_bool)
                            == Some(true)
                            && !s.to_lowercase().contains("ritual")
                        {
                            s.push_str(" (Ritual)");
                        }
                        s
                    }
                    "Range" => val(fields, "splat_spell", "range")
                        .and_then(|r| r.get("feet"))
                        .and_then(Value::as_u64)
                        .map_or_else(
                            || string(fields, "splat_spell", "range_text"),
                            |n| format!("{n} feet"),
                        ),
                    "Components" => {
                        let mut s = val(fields, "splat_spell", "components")
                            .and_then(Value::as_sequence)
                            .map(|s| {
                                s.iter()
                                    .filter_map(Value::as_str)
                                    .map(|s| match s {
                                        "verbal" => "V",
                                        "somatic" => "S",
                                        "material" => "M",
                                        _ => s,
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            })
                            .unwrap_or_default();
                        if let Some(description) = val(fields, "splat_spell", "material")
                            .and_then(|m| {
                                m.as_str()
                                    .or_else(|| m.get("description").and_then(Value::as_str))
                            })
                            .filter(|s| !s.is_empty())
                        {
                            s.push_str(&format!(" ({description})"));
                        }
                        s
                    }
                    _ => {
                        let raw = string(fields, "splat_spell", "duration");
                        if val(fields, "splat_spell", "concentration").and_then(Value::as_bool)
                            == Some(true)
                            && !raw.to_lowercase().starts_with("concentration")
                        {
                            format!("Concentration, {raw}")
                        } else {
                            raw
                        }
                    }
                };
                rows.push((name, value));
            }
            for (name, key) in [("Ritual", "ritual"), ("Concentration", "concentration")] {
                if let Some(value) = val(fields, "splat_spell", key).and_then(Value::as_bool) {
                    rows.push((name, if value { "Yes" } else { "No" }.into()));
                }
            }
            if val(fields, "splat_spell", "range")
                .and_then(|v| v.get("feet"))
                .and_then(Value::as_u64)
                .is_some()
                && let Some(raw) = val(fields, "splat_spell", "range_text").and_then(Value::as_str)
            {
                rows.push(("Range Text", raw.into()));
            }
        }
        _ => {
            rows.push((
                "Type",
                title_words(string(fields, "splat_item", "category")),
            ));
            rows.push((
                "Rarity",
                title_words(string(fields, "splat_item", "rarity")),
            ));
            rows.push((
                "Attunement",
                val(fields, "splat_item", "equipment")
                    .and_then(|m| m.get("attunement"))
                    .and_then(Value::as_str)
                    .map(|value| match value {
                        "none" => "No",
                        "required" => "Yes",
                        _ => value,
                    })
                    .unwrap_or("")
                    .into(),
            ));
        }
    }
    let head = headers(kind);
    let mut out = format!(
        "## {}\n\n| {} | {} |\n| --- | --- |\n",
        section(kind),
        head[0],
        head[1]
    );
    for (name, value) in rows {
        out.push_str(&format!("| {name} | {} |\n", value.replace('|', "\\|")));
    }
    if kind == "creature" {
        out.push_str("\n| Ability | Score | Mod | Save |\n| --- | --- | --- | --- |\n");
        for (short, key) in ABILITIES {
            let score = numeric(fields, key);
            let modifier = score
                .map(|n| signed((n - 10).div_euclid(2)))
                .unwrap_or_default();
            let save = numeric(fields, &format!("save_{key}"))
                .map(signed)
                .unwrap_or_default();
            out.push_str(&format!(
                "| {short} | {} | {modifier} | {save} |\n",
                score.map(|n| n.to_string()).unwrap_or_default()
            ));
        }
    }
    out.push('\n');
    out
}

// Presence hints preserve optional fields of the model, without storing a
// second value. A new non-default value always comes from its visible cell.
const OPTIONAL_PATHS: &[&str] = &[
    "splat_creature/armor_class_text",
    "splat_creature/hit_points_text",
    "splat_creature/movement_text",
    "splat_stats/speed/unit",
    "splat_spell/ritual",
    "splat_spell/concentration",
    "splat_stats",
];
fn at_path<'a>(fields: &'a Mapping, path: &str) -> Option<&'a Value> {
    let mut parts = path.split('/');
    let mut value = fields.get(parts.next()?)?;
    for part in parts {
        value = value.get(part)?;
    }
    Some(value)
}
fn remove_path(fields: &mut Mapping, path: &str) {
    let parts: Vec<_> = path.split('/').collect();
    if parts.len() == 1 {
        fields.remove(parts[0]);
        return;
    }
    let mut value = fields.get_mut(parts[0]);
    for part in &parts[1..parts.len() - 1] {
        value = value.and_then(|v| v.get_mut(*part));
    }
    if let Some(map) = value.and_then(Value::as_mapping_mut) {
        map.remove(parts[parts.len() - 1]);
    }
}
pub(super) fn normalize_plain_stats(fields: &mut Mapping) {
    for (path, stat) in OPTIONAL_PATHS[..3]
        .iter()
        .zip(["armor_class", "max_hp", "speed"])
    {
        if at_path(fields, path)
            .and_then(Value::as_str)
            .and_then(|s| s.parse::<i64>().ok())
            .is_some_and(|n| numeric(fields, stat) == Some(n))
        {
            remove_path(fields, path);
        }
    }
}
fn remove_optional(fields: &mut Mapping, path: &str) {
    let Some(value) = at_path(fields, path) else {
        return;
    };
    let default = match path {
        "splat_creature/armor_class_text" | "splat_creature/hit_points_text" => {
            value.as_str().is_some_and(|s| s.parse::<i64>().is_ok())
        }
        "splat_creature/movement_text" => value.as_str().is_some_and(|s| {
            s.parse::<i64>().is_ok() || number(s, false).is_ok_and(|n| s == format!("{n} ft."))
        }),
        "splat_stats/speed/unit" => value.as_str() == Some("ft"),
        "splat_stats" => value.as_mapping().is_some_and(Mapping::is_empty),
        _ => value.as_bool() == Some(false),
    };
    if default {
        remove_path(fields, path);
    }
}

/// The stat section for the table-owned data, and all other mechanics for an end fence.
pub(super) fn section_and_extras(
    fields: &Mapping,
    kind: &str,
) -> Result<(String, Vec<(String, Value)>), String> {
    let mut rendered = render(fields, kind);
    let mut shapes = Vec::new();
    if literal_type(fields) {
        shapes.push("type_literal");
    }
    if val(fields, "splat_item", "rarity")
        .and_then(Value::as_str)
        .is_some_and(|raw| {
            ![
                "common",
                "uncommon",
                "rare",
                "very_rare",
                "legendary",
                "artifact",
                "varies",
                "rarity_varies",
                "same",
            ]
            .contains(&raw)
        })
    {
        shapes.push("rarity_text");
    }
    if string(fields, "splat_creature", "size").contains(' ')
        && val(fields, "splat_creature", "size_options_text").is_none()
    {
        shapes.push("size_literal");
    }
    if val(fields, "splat_spell", "range_text").is_some()
        && val(fields, "splat_spell", "range")
            .and_then(|v| v.get("feet"))
            .and_then(Value::as_u64)
            .is_none()
    {
        shapes.push("range_text");
    }

    if kind == "creature" {
        if numeric(fields, "max_hp").is_none()
            && val(fields, "splat_creature", "hit_points_text").is_some()
        {
            shapes.push("hp_text");
        }
        if numeric(fields, "armor_class").is_none()
            && val(fields, "splat_creature", "armor_class_text").is_some()
        {
            shapes.push("ac_text");
        }
        let movement = string(fields, "splat_creature", "movement_text");
        if numeric(fields, "speed").is_none() && !movement.is_empty() {
            shapes.push("speed_text");
        }

        if numeric(fields, "speed")
            .is_some_and(|n| speed(&movement).is_ok_and(|(first, _)| n != first))
        {
            shapes.push("speed_max");
        }
    }
    if val(fields, "splat_spell", "casting_time").is_some_and(Value::is_mapping) {
        shapes.push("casting_time");
    }
    if val(fields, "splat_spell", "material").is_some_and(Value::is_string) {
        shapes.push("material_text");
    }
    if val(fields, "splat_spell", "concentration").and_then(Value::as_bool) == Some(true)
        && !string(fields, "splat_spell", "duration")
            .to_lowercase()
            .starts_with("concentration")
    {
        shapes.push("duration_plain");
    }
    if !shapes.is_empty() {
        rendered.push_str(&format!("%% splat shape={} %%\n\n", shapes.join(",")));
    }
    let mut parsed = Mapping::new();
    let mut flags = Vec::new();
    read_tables(kind, &rendered, &mut parsed, &mut flags);
    let mut omit = Vec::new();
    for path in OPTIONAL_PATHS {
        if at_path(fields, path).is_none() && at_path(&parsed, path).is_some() {
            let before = at_path(&parsed, path).cloned();
            remove_optional(&mut parsed, path);
            if at_path(&parsed, path).cloned() != before {
                omit.push(*path);
            }
        }
    }
    if !omit.is_empty() {
        rendered.push_str(&format!("%% splat omit={} %%\n\n", omit.join(",")));
    }
    let mut extras = Vec::new();
    for root in roots(kind) {
        let mut extra = fields
            .get(*root)
            .and_then(Value::as_mapping)
            .cloned()
            .unwrap_or_default();
        if let Some(map) = parsed.get(*root).and_then(Value::as_mapping) {
            subtract(&mut extra, map, root)?;
        }
        let mut invalid = Vec::new();
        let mut checked = extra.clone();
        reject_owned(&mut checked, root, &mut invalid);
        if !invalid.is_empty() {
            return Err(format!(
                "Review unsupported {root} values before writing: {}",
                invalid
                    .iter()
                    .map(|f| f.words())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !extra.is_empty() {
            extras.push((format!("{root}_extras"), extra.into()));
        }
    }
    Ok((rendered, extras))
}

fn subtract(extra: &mut Mapping, printed: &Mapping, path: &str) -> Result<(), String> {
    for (key, value) in printed {
        if let Some(old) = extra.get_mut(key) {
            if let (Some(old), Some(new)) = (old.as_mapping_mut(), value.as_mapping()) {
                subtract(
                    old,
                    new,
                    &format!("{path}/{}", key.as_str().unwrap_or("field")),
                )?;
                if old.is_empty() {
                    extra.remove(key);
                }
            } else if old == value {
                extra.remove(key);
            } else {
                return Err(format!(
                    "The printed stats and saved stats disagree: {path}/{}",
                    key.as_str().unwrap_or("field")
                ));
            }
        }
    }
    Ok(())
}
fn merge_extra(target: &mut Mapping, extra: Mapping) {
    for (key, value) in extra {
        if let Some(old) = target.get_mut(&key) {
            if let (Some(old), Value::Mapping(new)) = (old.as_mapping_mut(), value) {
                merge_extra(old, new);
            }
        } else {
            target.insert(key, value);
        }
    }
}
fn reject_owned(extra: &mut Mapping, root: &str, flags: &mut Vec<Flag>) {
    for key in owned_keys(root) {
        if extra.get(key).is_some_and(empty_value) {
            continue;
        }
        let nested = match (root, key) {
            ("splat_spell", "material") => &["description"][..],
            ("splat_spell", "range") => &["feet"][..],
            ("splat_item", "equipment") => &["attunement"][..],
            ("splat_stats", _) => &["value", "derive"][..],
            _ => &[][..],
        };
        if !nested.is_empty()
            && let Some(map) = extra.get_mut(key).and_then(Value::as_mapping_mut)
        {
            for part in nested {
                if !map.get(*part).is_some_and(empty_value) && map.remove(*part).is_some() {
                    flags.push(Flag::new(
                        1,
                        format!("Unused data ignored: {root}/{key}/{part}"),
                    ));
                }
            }
            if map.is_empty() {
                extra.remove(key);
            }
        } else if extra.remove(key).is_some() {
            flags.push(Flag::new(1, format!("Unused data ignored: {root}/{key}")));
        }
    }
}

pub fn data_roots(kind: &str) -> &'static [&'static str] {
    roots(kind)
}

pub fn insert_section(text: &str, body: &str) -> String {
    let at = Parser::new(text)
        .into_offset_iter()
        .find_map(|(event, range)| {
            matches!(
                event,
                Event::Start(Tag::Heading {
                    level: pulldown_cmark::HeadingLevel::H1,
                    ..
                })
            )
            .then_some(range.end)
        })
        .unwrap_or(text.len());
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let mut out = text.to_owned();
    out.insert_str(
        at,
        &format!("{newline}{newline}{}", body.replace('\n', newline)),
    );
    out
}

/// Apply a model edit through individual cells, without rendering the saved table.
/// HP/AC formulas follow their number. A score edit also corrects its printed Mod.
pub fn patch(text: &str, changes: &[(&str, Option<Value>)]) -> Result<String, String> {
    let entry = Entry::parse("note.md", text)?;
    let kind = &entry.kind;
    let mut expected = (*entry.fields).clone();
    for (key, value) in changes {
        match value {
            Some(value) => {
                expected.insert((*key).into(), value.clone());
            }
            None => {
                expected.remove(*key);
            }
        }
    }
    if kind == "creature" {
        for (key, prose) in [
            ("max_hp", "hit_points_text"),
            ("armor_class", "armor_class_text"),
            ("speed", "movement_text"),
        ] {
            let before = numeric(&entry.fields, key);
            let after = numeric(&expected, key);
            if before != after
                && let Some(n) = after
            {
                let raw = string(&expected, "splat_creature", prose);
                if !raw.is_empty() {
                    let tail = raw.find(char::is_whitespace).map_or("", |at| &raw[at..]);
                    expected
                        .get_mut("splat_creature")
                        .unwrap()
                        .as_mapping_mut()
                        .unwrap()
                        .insert(prose.into(), format!("{n}{tail}").into());
                }
            }
        }
    }
    let rendered = render(&expected, kind);
    let desired = Document::new(&rendered)
        .section_at_level(section(kind), Some(pulldown_cmark::HeadingLevel::H2))
        .and_then(|s| s.table(headers(kind)))
        .ok_or("Missing stat table")?;
    let old_rendered = render(&entry.fields, kind);
    let old = Document::new(&old_rendered)
        .section_at_level(section(kind), Some(pulldown_cmark::HeadingLevel::H2))
        .and_then(|s| s.table(headers(kind)))
        .ok_or("Missing stat table")?;
    let mut after = text.to_owned();
    for row in &desired.rows {
        let name = &row[0].value;
        let derived_changed = kind == "creature"
            && ["Challenge", "Initiative", "Speed"].contains(&name.as_str())
            && Document::new(&after)
                .section_at_level(section(kind), Some(pulldown_cmark::HeadingLevel::H2))
                .and_then(|s| s.table(headers(kind)))
                .and_then(|t| t.row(name).map(<[_]>::to_vec))
                .is_some_and(|r| {
                    r[1].value != row[1].value
                        && !r[1].value.is_empty()
                        && !entry.flags.iter().any(|flag| flag.line == r[1].line)
                });
        if derived_changed
            || old.row(name).and_then(|r| r.get(1)).map(|c| &c.value) != Some(&row[1].value)
        {
            after = set_cell(&after, kind, headers(kind), name, 1, &row[1].value)?;
        }
    }
    if kind == "creature" {
        for (name, _) in DETAILS {
            if old.row(name).is_some() && desired.row(name).is_none() {
                after = set_cell(&after, kind, headers(kind), name, 1, "")?;
            }
        }
        let desired = Document::new(&rendered)
            .section_at_level(section(kind), Some(pulldown_cmark::HeadingLevel::H2))
            .and_then(|s| s.table(&["Ability", "Score", "Mod", "Save"]))
            .unwrap();
        for row in &desired.rows {
            let short = &row[0].value;
            let actual = Document::new(&after)
                .section_at_level(section(kind), Some(pulldown_cmark::HeadingLevel::H2))
                .and_then(|s| s.table(&["Ability", "Score", "Mod", "Save"]));
            let full = ABILITIES.iter().find(|(name, _)| name == short).unwrap().1;
            let name = actual
                .as_ref()
                .and_then(|t| {
                    t.rows.iter().filter_map(|r| r.first()).find(|cell| {
                        [full, short.as_str()]
                            .iter()
                            .any(|name| layout::same_label(&cell.value, name))
                    })
                })
                .map_or(short.as_str(), |cell| cell.value.as_str());
            let old_score = numeric(&entry.fields, full);
            let new_score = numeric(&expected, full);
            for column in [1, 2, 3] {
                let previous = actual
                    .as_ref()
                    .and_then(|t| t.row(name))
                    .and_then(|r| r.get(column))
                    .map(|c| c.value.as_str());
                let should_edit = if column == 2 {
                    new_score.is_some()
                        && previous.and_then(|raw| number(raw, true).ok())
                            != number(&row[2].value, true).ok()
                } else if column == 3 {
                    numeric(&entry.fields, &format!("save_{full}"))
                        != numeric(&expected, &format!("save_{full}"))
                } else {
                    old_score != new_score
                };
                if should_edit && previous != Some(&row[column].value) {
                    after = set_cell(
                        &after,
                        kind,
                        &["Ability", "Score", "Mod", "Save"],
                        name,
                        column,
                        &row[column].value,
                    )?;
                }
            }
        }
    }
    let (_, old_extras) = section_and_extras(&entry.fields, kind)?;
    let (_, new_extras) = section_and_extras(&expected, kind)?;
    for root in roots(kind) {
        let key = format!("{root}_extras");
        let empty = Value::Mapping(Mapping::new());
        let before = old_extras
            .iter()
            .find(|(k, _)| k == &key)
            .map_or(&empty, |(_, v)| v);
        let desired = new_extras
            .iter()
            .find(|(k, _)| k == &key)
            .map_or(&empty, |(_, v)| v);
        if desired != before {
            after = super::formats::patch_fence(&after, &key, desired)?;
        }
    }
    let checked = Entry::parse("note.md", &after)?;
    // A newly set number also supplies its display text. Only these linked,
    // computed fields may appear as a consequence of the requested cell edit.
    if kind == "creature" {
        for (key, prose) in [
            ("max_hp", "hit_points_text"),
            ("armor_class", "armor_class_text"),
            ("speed", "movement_text"),
        ] {
            if numeric(&entry.fields, key) != numeric(&expected, key)
                && val(&expected, "splat_creature", prose).is_none()
                && let Some(value) = val(&checked.fields, "splat_creature", prose)
            {
                expected
                    .get_mut("splat_creature")
                    .unwrap()
                    .as_mapping_mut()
                    .unwrap()
                    .insert(prose.into(), value.clone());
            }
        }
    }
    if *checked.fields != expected {
        return Err("Edit changed another value. The file was kept.".into());
    }
    Ok(after)
}
fn set_cell(
    text: &str,
    kind: &str,
    head: &[&str],
    name: &str,
    column: usize,
    value: &str,
) -> Result<String, String> {
    let table = Document::new(text)
        .section_at_level(section(kind), Some(pulldown_cmark::HeadingLevel::H2))
        .and_then(|s| s.table(head));
    let mut actual_name = name.to_owned();
    if let Some(table) = &table {
        let aliases = match name {
            "Hit Points" => &["HP"][..],
            "Armor Class" => &["AC"][..],
            "Challenge" => &["CR"][..],
            _ => &[],
        };
        if let Some(cell) = table
            .rows
            .iter()
            .filter_map(|row| row.first())
            .find(|cell| {
                std::iter::once(name)
                    .chain(aliases.iter().copied())
                    .any(|name| layout::same_label(&cell.value, name))
            })
        {
            actual_name = cell.value.clone();
        }
        if table.row(&actual_name).is_some() {
            let mut value = value.to_owned();
            if kind == "creature"
                && name == "Initiative"
                && let Some(old) = table.row(&actual_name).and_then(|r| r.get(column))
                && old.value.contains('(')
                && !value.contains('(')
                && let Ok(n) = number(&value, true)
                && let Some(passive) = n.checked_add(10)
            {
                value = format!("{value} ({passive})");
            }
            return layout::replace_cell_at_level(
                text,
                section(kind),
                head,
                &actual_name,
                column,
                &value,
                Some(pulldown_cmark::HeadingLevel::H2),
            );
        }
        // Insert in fixed order. Existing rows and their bytes never move relative
        // to each other. A missing ability table is handled below.
        let order: Vec<_> = if head.len() == 4 {
            ABILITIES.iter().map(|(short, _)| *short).collect()
        } else if kind == "creature" {
            CREATURE_ROWS
                .iter()
                .copied()
                .chain(DETAILS.iter().map(|(name, _)| *name))
                .collect()
        } else if kind == "spell" {
            SPELL_ROWS.to_vec()
        } else {
            ITEM_ROWS.to_vec()
        };
        let rank = |name: &str| {
            order
                .iter()
                .position(|n| layout::same_label(n, name))
                .unwrap_or(order.len())
        };
        let next = table
            .rows
            .iter()
            .find(|r| r.first().is_some_and(|c| rank(&c.value) > rank(name)));
        let at = if let Some(row) = next {
            text[..row[0].range.start].rfind('\n').map_or(0, |i| i + 1)
        } else {
            let cell = table
                .rows
                .last()
                .and_then(|r| r.last())
                .unwrap_or(table.header.last().unwrap());
            let end = text[cell.range.end..]
                .find('\n')
                .map_or(text.len(), |i| cell.range.end + i + 1);
            // With no rows, skip the separator too.
            if table.rows.is_empty() {
                text[end..].find('\n').map_or(text.len(), |i| end + i + 1)
            } else {
                end
            }
        };
        let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
        let mut cells = vec![String::new(); head.len()];
        cells[0] = name.into();
        cells[column] = value.into();
        let mut after = text.to_owned();
        after.insert_str(at, &format!("| {} |{newline}", cells.join(" | ")));
        return Ok(after);
    }
    let section = Document::new(text)
        .section_at_level(section(kind), Some(pulldown_cmark::HeadingLevel::H2))
        .ok_or("Stat section not found")?;
    let at = section.range().end;
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let mut cells = vec![String::new(); head.len()];
    cells[0] = name.into();
    cells[column] = value.into();
    let mut after = text.to_owned();
    after.insert_str(
        at,
        &format!(
            "{newline}| {} |{newline}| {} |{newline}| {} |{newline}{newline}",
            head.join(" | "),
            vec!["---"; head.len()].join(" | "),
            cells.join(" | ")
        ),
    );
    Ok(after)
}

/// Rejected vitals are display text only. The model still has no numeric value.
pub fn invalid_vital(entry: &Entry, key: &str) -> Option<String> {
    if entry.kind != "creature" || numeric(&entry.fields, key).is_some() {
        return None;
    }
    let names: &[&str] = match key {
        "max_hp" => &["Hit Points", "HP"],
        "armor_class" => &["Armor Class", "AC"],
        "initiative" => &["Initiative"],
        _ => return None,
    };
    let table = Document::new(&entry.body)
        .section_at_level("Stat block", Some(pulldown_cmark::HeadingLevel::H2))?
        .table(&["Stat", "Value"])?;
    let row = table.rows.iter().find(|row| {
        row.first().is_some_and(|cell| {
            names
                .iter()
                .any(|name| layout::same_label(name, &cell.value))
        })
    })?;
    row.get(1)
        .map(|cell| cell.value.replace("\\|", "|"))
        .filter(|raw| !raw.is_empty())
}

fn without_printed_record(body: &str) -> String {
    let mut out = body.to_owned();
    let ranges: Vec<_> = Parser::new(body)
        .into_offset_iter()
        .filter_map(|(event, range)| {
            (matches!(event, Event::Start(Tag::BlockQuote(_)))
                && body[range.clone()].starts_with(PRINTED_BLOCK))
            .then_some(range)
        })
        .collect();
    for range in ranges.into_iter().rev() {
        out.replace_range(range, "");
    }
    out
}

/// Compare the words and model, independent of the on-disk layout.
pub fn comparison_body(entry: &Entry) -> String {
    let mut body = entry.body.clone();
    for root in roots(&entry.kind) {
        body = body.replace(&format!("path={root}_extras "), &format!("path={root} "));
    }
    body = without_marked_stats(&body, &entry.kind);
    if entry.kind == "creature" {
        body = fold_printed_stats(&body);
    }
    reading_body(entry, &body)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Omit model-backed rows and rejected vitals shown in the reader's stat area.
/// Unknown rows remain visible as text.
pub fn reading_body(entry: &Entry, body: &str) -> String {
    let clean_body = without_printed_record(body);
    let body = clean_body.as_str();
    let kind = entry.kind.as_str();
    let Some(stat_section) =
        Document::new(body).section_at_level(section(kind), Some(pulldown_cmark::HeadingLevel::H2))
    else {
        return body.into();
    };
    let mut ranges = Vec::new();
    let section_range = stat_section.range();
    for (event, range) in Parser::new(&body[section_range.clone()]).into_offset_iter() {
        if matches!(event, Event::Start(Tag::Paragraph)) {
            let range = section_range.start + range.start..section_range.start + range.end;
            let raw = body[range.clone()].trim();
            if (raw.starts_with("%% splat omit=") || raw.starts_with("%% splat shape="))
                && raw.ends_with(" %%")
            {
                ranges.push(range);
            }
        }
    }
    let heads = if kind == "creature" {
        vec![headers(kind), &["Ability", "Score", "Mod", "Save"][..]]
    } else {
        vec![headers(kind)]
    };
    for head in heads {
        if let Some(table) = stat_section.table(head) {
            let extra_rows: &[&str] = match kind {
                "spell" => &["Ritual", "Concentration", "Range Text"],
                "creature" => &["Type Tags", "Type Options"],
                _ => &[],
            };
            let known = |name: &str| {
                (if kind == "spell" {
                    SPELL_ROWS
                } else if kind == "item" {
                    ITEM_ROWS
                } else {
                    CREATURE_ROWS
                })
                .iter()
                .chain(extra_rows.iter())
                .chain(["HP", "AC", "CR"].iter())
                .chain(DETAILS.iter().map(|(name, _)| name))
                .any(|n| layout::same_label(n, name))
                    || ABILITIES.iter().any(|(short, key)| {
                        layout::same_label(name, short) || layout::label(name) == *key
                    })
            };
            let start = body[..table.header[0].range.start]
                .rfind('\n')
                .map_or(0, |i| i + 1);
            let header_end = body[table.header[0].range.end..].find('\n').unwrap()
                + table.header[0].range.end
                + 1;
            let end = body[header_end..]
                .find('\n')
                .map_or(body.len(), |i| header_end + i + 1);
            ranges.push(start..end);
            for row in &table.rows {
                if row.len() == head.len() && known(&row[0].value) {
                    let start = body[..row[0].range.start].rfind('\n').map_or(0, |i| i + 1);
                    let end = body[row[0].range.end..]
                        .find('\n')
                        .map_or(body.len(), |i| row[0].range.end + i + 1);
                    let raw = &row[1].value;
                    let key = match layout::label(&row[0].value).as_str() {
                        "hit points" | "hp" => Some("max_hp"),
                        "armor class" | "ac" => Some("armor_class"),
                        "initiative" => Some("initiative"),
                        "speed" => Some("speed"),
                        _ => None,
                    };
                    if key.is_none_or(|key| {
                        raw.is_empty()
                            || invalid_vital(entry, key).as_deref() == Some(raw)
                            || (if key == "speed" {
                                speed(raw).map(|(n, _)| n)
                            } else {
                                number(raw, key == "initiative")
                            })
                            .ok()
                            .is_some_and(|n| Some(n) == numeric(&entry.fields, key))
                    }) {
                        ranges.push(start..end);
                    }
                }
            }
        }
    }
    let mut out = body.to_owned();
    ranges.sort_by_key(|range| range.start);
    for range in ranges.into_iter().rev() {
        out.replace_range(range, "");
    }
    if let Some((_, range)) = Parser::new(&out).into_offset_iter().find(|(event, range)| {
        matches!(
            event,
            Event::Start(Tag::Heading {
                level: pulldown_cmark::HeadingLevel::H2,
                ..
            })
        ) && layout::label(out[range.clone()].trim_start_matches('#'))
            == layout::label(section(kind))
    }) {
        out.replace_range(range, "");
    }
    out
}

fn empty_value(value: &Value) -> bool {
    value.is_null()
        || value.as_str() == Some("")
        || value.as_sequence().is_some_and(Vec::is_empty)
        || value.as_bool() == Some(false)
        || value
            .as_mapping()
            .is_some_and(|m| m.values().all(empty_value))
}
