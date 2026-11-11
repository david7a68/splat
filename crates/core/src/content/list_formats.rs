//! Schema 2 campaign lists and encounter tables. Unknown text stays in the source.
use super::{
    Entry, Mapping, Value, formats,
    layout::{Document, Flag, FlagKind},
    patch_fields,
};
use uuid::Uuid;

pub const GROUP: &[&str] = &["Count", "Creature", "Name", "Changes", "Note"];
pub const ROSTER: &[&str] = &["Side", "Creature", "Name", "Changes", "Note"];
const RULES: &[&str] = &["Rule", "Choice"];
const STATS: &[(&str, &str)] = &[
    ("HP", "max_hp"),
    ("AC", "armor_class"),
    ("Attack", "attack_bonus"),
    ("Damage", "damage_bonus"),
    ("Speed", "speed"),
    ("Initiative", "initiative_bonus"),
];

pub fn machine(text: &str, key: &str) -> Option<String> {
    text.split("%% splat ").skip(1).find_map(|comment| {
        comment
            .split_once("%%")?
            .0
            .split_whitespace()
            .find_map(|term| term.strip_prefix(&format!("{key}=")).map(str::to_owned))
    })
}
fn visible(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("%% splat ") {
        let Some(end) = rest[start + 9..].find("%%") else {
            break;
        };
        out.push_str(&rest[..start]);
        rest = &rest[start + 9 + end + 2..];
        if out.ends_with(char::is_whitespace) {
            rest = rest.trim_start();
        }
    }
    out.push_str(rest);
    out.trim().to_owned()
}

fn map(items: &[(&str, Value)]) -> Value {
    Value::Mapping(
        items
            .iter()
            .map(|(k, v)| (Value::from(*k), v.clone()))
            .collect(),
    )
}
fn seq(value: &Value, key: &str) -> Vec<Value> {
    value
        .get(key)
        .and_then(Value::as_sequence)
        .cloned()
        .unwrap_or_default()
}
fn words<'a>(value: &'a Value, key: &str) -> &'a str {
    value.get(key).and_then(Value::as_str).unwrap_or("")
}

fn unread_change(term: &str) -> String {
    let example = term.split_whitespace().next().and_then(|stat| {
        STATS
            .iter()
            .find(|(label, _)| label.eq_ignore_ascii_case(stat))
    });
    let repair = example.map_or_else(
        || "Write a change such as HP +5.".to_owned(),
        |(stat, _)| format!("Use a number, such as {stat} +5."),
    );
    format!(
        "the change {} was not read. {repair}",
        super::formats::quoted(term)
    )
}

pub fn changes(text: &str, name: &str, note: &str) -> Result<Mapping, String> {
    let mut out = Mapping::new();
    if !name.is_empty() {
        out.insert("name".into(), name.into());
    }
    if !note.is_empty() {
        out.insert("note".into(), note.into());
    }
    let (text, loot) = super::loot::split_changes(text);
    if let Some(loot) = loot {
        if loot.is_empty() {
            return Err("Changes: Loot needs what the creature carries".into());
        }
        out.insert("loot".into(), loot.into());
    }
    let mut conditions = Vec::new();
    for term in text.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if term.eq_ignore_ascii_case("loot") {
            return Err("Changes: Loot needs what the creature carries".into());
        }
        if term.to_ascii_lowercase().starts_with("start hp ") {
            return Err("Start HP is set on an encounter’s roster".into());
        }
        if let Some((stat, rest)) = term.split_once(' ') {
            if stat.eq_ignore_ascii_case("side") {
                if rest.trim().is_empty() || out.contains_key("side") {
                    return Err("Changes: invalid Side".into());
                }
                out.insert("side".into(), rest.trim().into());
                continue;
            }
            let (_, key) = STATS
                .iter()
                .find(|(label, _)| label.eq_ignore_ascii_case(stat))
                .ok_or_else(|| unread_change(term))?;
            let amount = if *key == "speed" {
                rest.trim()
                    .strip_suffix(" ft.")
                    .or_else(|| rest.trim().strip_suffix(" ft"))
                    .unwrap_or(rest.trim())
            } else {
                rest.trim()
            };
            let number: i64 = amount.parse().map_err(|_| unread_change(term))?;
            if out.contains_key(*key) {
                return Err(format!("Changes: duplicate {stat}"));
            }
            out.insert(
                (*key).into(),
                map(&[(
                    if rest.trim().starts_with(['+', '-']) {
                        "add"
                    } else {
                        "set"
                    },
                    number.into(),
                )]),
            );
        } else if term.chars().all(|c| c.is_alphabetic() || c == '-') {
            conditions.push(Value::from(term));
        } else {
            return Err(unread_change(term));
        }
    }
    if !conditions.is_empty() {
        out.insert("conditions".into(), Value::Sequence(conditions));
    }
    super::overrides::Overrides::parse(&out)?;
    Ok(out)
}
pub fn show_changes(overrides: &Value) -> String {
    let mut terms = Vec::new();
    for (label, key) in STATS {
        if let Some(change) = overrides.get(*key) {
            if let Some(n) = change.get("add").and_then(Value::as_i64) {
                terms.push(format!(
                    "{label} {n:+}{}",
                    if *key == "speed" { " ft." } else { "" }
                ));
            }
            if let Some(n) = change.get("set").and_then(Value::as_i64) {
                terms.push(format!(
                    "{label} {n}{}",
                    if *key == "speed" { " ft." } else { "" }
                ));
            }
        }
    }
    if let Some(side) = overrides.get("side").and_then(Value::as_str) {
        terms.push(format!("Side {side}"));
    }
    terms.extend(super::overrides::condition_names(&overrides["conditions"]));
    // Last, because its words can hold commas.
    if let Some(loot) = overrides.get("loot").and_then(Value::as_str) {
        terms.push(format!("Loot {loot}"));
    }
    terms.join(", ")
}
fn reference(text: &str) -> Result<Value, String> {
    let raw = visible(text);
    if let Some((_, tail)) = raw.split_once("](splat:") {
        let key = tail.strip_suffix(')').ok_or("Creature: invalid link")?;
        if !super::resource_key(key) {
            return Err("Creature: invalid key".into());
        }
        if let Some(id) = machine(text, "id") {
            return Ok(map(&[("id", id.into()), ("link", raw.into())]));
        }
        return Ok(map(&[("key", key.into())]));
    }
    if raw.starts_with("[[") && raw.ends_with("]]") {
        let target = raw[2..raw.len() - 2].split('|').next().unwrap_or("");
        let id = machine(text, "id").or_else(|| {
            machine(text, "key")
                .is_none()
                .then(|| Uuid::parse_str(target).ok().map(|id| id.to_string()))
                .flatten()
        });
        let mut result = Mapping::new();
        if let Some(id) = id {
            result.insert("id".into(), id.into());
        }
        if let Some(key) = machine(text, "key") {
            result.insert("key".into(), key.into());
        }
        // A UUID-only wikilink is the readable spelling of an id-only reference.
        if Uuid::parse_str(target).is_err() || raw != format!("[[{target}]]") {
            result.insert("link".into(), raw.into());
        }
        return Ok(Value::Mapping(result));
    }
    let value = formats::model_reference(raw.clone().into());
    if value.is_mapping() {
        Ok(value)
    } else if !raw.is_empty() {
        Ok(map(&[("link", raw.into())]))
    } else {
        Err("Creature: enter a name or one link".into())
    }
}
fn show_reference(value: &Value) -> String {
    if let Some(link) = value.get("link").and_then(Value::as_str) {
        let backup = value
            .get("id")
            .and_then(Value::as_str)
            .map(|id| format!(" %% splat id={id} %%"))
            .unwrap_or_default();
        let key = value
            .get("key")
            .and_then(Value::as_str)
            .map(|key| format!(" %% splat key={key} %%"))
            .unwrap_or_default();
        return format!("{link}{backup}{key}");
    }
    if let Some(key) = value.get("key").and_then(Value::as_str) {
        return format!(
            "[{}](splat:{key})",
            key.rsplit('/').next().unwrap_or(key).replace('_', " ")
        );
    }
    format!("[[{}]]", words(value, "id"))
}
fn table(text: &str, section: &str, headers: &[&str]) -> Option<super::layout::Table> {
    Document::new(text)
        .section(section)
        .and_then(|s| s.table(headers))
        .or_else(|| {
            (headers == GROUP)
                .then(|| Document::new(text).table(headers))
                .flatten()
        })
}

/// A rejected table row stays visible without becoming a runnable member.
pub struct UnreadGroupMember {
    pub before: usize,
    pub creature: Option<Value>,
    pub label: String,
    pub name: String,
    pub count: Option<u64>,
    pub flag: Flag,
}

pub fn unread_group_members(text: &str, flags: &[Flag]) -> Vec<UnreadGroupMember> {
    let mut before = 0;
    let mut unread = Vec::new();
    if let Some(table) = table(text, "Members", GROUP) {
        for row in table.rows {
            if let Some(flag) = flags.iter().find(|flag| {
                row.first().is_some_and(|c| c.line == flag.line)
                    && !flag.message.starts_with("Creature ")
            }) {
                let label = row.get(1).map_or("", |cell| cell.value.as_str());
                unread.push(UnreadGroupMember {
                    before,
                    creature: reference(label).ok(),
                    label: label.into(),
                    name: row
                        .get(2)
                        .map_or_else(String::new, |cell| cell.value.clone()),
                    count: row
                        .first()
                        .and_then(|cell| {
                            if cell.value.is_empty() {
                                Some(1)
                            } else {
                                cell.value.parse().ok()
                            }
                        })
                        .filter(|count| (1..=super::encounter_group::MAX_COUNT).contains(count)),
                    flag: flag.clone(),
                });
            } else {
                before += 1;
            }
        }
    }
    unread
}

fn only_references(raw: &str) -> Result<Vec<Value>, String> {
    let mut square = 0usize;
    let mut round = 0usize;
    let mut start = 0;
    let mut values = Vec::new();
    for (at, ch) in raw.char_indices() {
        match ch {
            '[' => square += 1,
            ']' => square = square.saturating_sub(1),
            '(' if square == 0 => round += 1,
            ')' if square == 0 => round = round.saturating_sub(1),
            ',' if square == 0 && round == 0 => {
                values.push(reference(raw[start..at].trim())?);
                start = at + 1;
            }
            _ => {}
        }
    }
    if square != 0 || round != 0 {
        return Err("Only: link not closed".into());
    }
    if !raw.trim().is_empty() {
        values.push(reference(raw[start..].trim())?);
    }
    Ok(values)
}

pub fn read(text: &str, fields: &mut Mapping, flags: &mut Vec<Flag>) {
    let kind = fields
        .get("splat_kind")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();
    if !["campaign", "encounter", "encounter_group"].contains(&kind.as_str()) {
        return;
    }
    let mut data = fields
        .remove("splat_list_data")
        .and_then(|v| v.as_mapping().cloned())
        .unwrap_or_default();
    if kind == "campaign" {
        for (flat, old) in [
            ("splat_run_folder", "splat_run_storage"),
            ("splat_session_folder", "splat_session_storage"),
        ] {
            if let Some(value) = fields.remove(flat) {
                fields.insert(old.into(), map(&[("directory", value)]));
            }
        }
        let mut packages = Vec::new();
        let mut in_packages = false;
        let mut last_enabled = false;
        let mut last_task = false;
        let mut seen = std::collections::BTreeSet::new();
        for source_line in super::layout::content_lines(text) {
            let line = source_line.line - 1;
            let raw = source_line.value.as_str();
            if raw.starts_with("## ") {
                in_packages = raw.trim().eq_ignore_ascii_case("## Packages");
                continue;
            }
            if !in_packages {
                continue;
            }
            let trimmed = raw.trim_start();
            if raw.len() != trimmed.len()
                && let Some(only) = trimmed
                    .get(2..)
                    .filter(|_| {
                        ["- ", "* ", "+ "]
                            .iter()
                            .any(|prefix| trimmed.starts_with(prefix))
                    })
                    .and_then(|s| s.strip_prefix("Only:"))
            {
                if !last_task {
                    flags.push(Flag::new(
                        line + 1,
                        "Only: needs a package task first.".into(),
                    ));
                } else if !last_enabled && let Err(message) = only_references(only.trim()) {
                    flags.push(Flag::new(line + 1, message));
                }
                if last_enabled && let Some(last) = packages.last_mut() {
                    *last = set_selection(
                        last,
                        map(&[
                            ("mode", "selected".into()),
                            ("entries", Value::Sequence(Vec::new())),
                        ]),
                    );
                    let entries = only_references(only.trim());
                    match entries {
                        Ok(entries) => {
                            *last = set_selection(
                                last,
                                map(&[
                                    ("mode", "selected".into()),
                                    ("entries", Value::Sequence(entries)),
                                ]),
                            )
                        }
                        Err(message) => flags.push(Flag::new(line + 1, message)),
                    }
                }
                continue;
            }
            if trimmed.is_empty() || (trimmed.starts_with("%%") && trimmed.ends_with("%%")) {
                continue;
            }
            if !["- ", "* ", "+ "]
                .iter()
                .any(|prefix| trimmed.starts_with(prefix))
            {
                last_enabled = false;
                last_task = false;
                continue;
            }
            let raw = trimmed;
            let parsed = (|| {
                let enabled = match raw.get(2..5) {
                    Some("[x]" | "[X]") => true,
                    Some("[ ]") => false,
                    _ => return Err("Package task not read".to_owned()),
                };
                let item = raw.get(6..).ok_or("Package name not found")?;
                let (name, id) = item.rsplit_once(" ^").unwrap_or((item, ""));
                if id.is_empty() {
                    return Ok((
                        enabled,
                        map(&[
                            ("name", name.trim().into()),
                            ("package", map(&[("link", name.trim().into())])),
                            ("selection", map(&[("mode", "all".into())])),
                        ]),
                    ));
                }
                Uuid::parse_str(id).map_err(|_| "Package id not found")?;
                if !seen.insert(id.to_owned()) {
                    return Err("Duplicate package".into());
                }
                Ok((
                    enabled,
                    map(&[
                        ("name", name.trim().into()),
                        ("package", map(&[("id", id.into())])),
                        ("selection", map(&[("mode", "all".into())])),
                    ]),
                ))
            })();
            last_enabled = matches!(&parsed, Ok((true, _)));
            last_task = parsed.is_ok();
            match parsed {
                Ok((true, value)) => packages.push(value),
                Ok((false, _)) => {}
                Err(message) => flags.push(Flag::new(line + 1, message)),
            }
        }
        fields.insert("splat_packages".into(), Value::Sequence(packages));
        let mut choices = Mapping::new();
        if let Some(table) = table(text, "Rule choices", RULES) {
            for row in table.rows {
                let parsed = (|| {
                    if row.len() != 2 {
                        return Err("Rule: missing or extra cells".into());
                    }
                    let slot = machine(&row[1].value, "rule").unwrap_or_else(|| {
                        if row[0].value.eq_ignore_ascii_case("Combat initiative") {
                            "dnd2024:rules/combat_initiative".into()
                        } else {
                            row[0].value.clone()
                        }
                    });
                    if !super::resource_key(&slot) {
                        return Err("Rule not found; default used".into());
                    }
                    let id = machine(&row[1].value, "id").ok_or("Rule choice id not found")?;
                    Uuid::parse_str(&id).map_err(|_| "Rule choice id not found")?;
                    if choices.contains_key(slot.as_str()) {
                        return Err("Duplicate rule choice".into());
                    }
                    choices.insert(slot.into(), map(&[("id", id.into())]));
                    Ok::<_, String>(())
                })();
                if let Err(message) = parsed {
                    flags.push(Flag::new(row.first().map_or(1, |c| c.line), message));
                }
            }
        }
        fields.insert("splat_rule_choices".into(), Value::Mapping(choices));
        for (key, value) in data {
            fields.entry(key).or_insert(value);
        }
        return;
    }
    let group = kind == "encounter_group";
    let key = if group {
        "splat_encounter_group"
    } else {
        "splat_encounter"
    };
    let mut model = data
        .remove(key)
        .and_then(|v| v.as_mapping().cloned())
        .unwrap_or_default();
    if group {
        model.insert(
            "side".into(),
            fields
                .remove("splat_side")
                .unwrap_or_else(|| "enemies".into()),
        );
        let backup = model.remove("encounter_backup");
        if let Some(owner) = fields.remove("splat_encounter") {
            let mut owner = reference(owner.as_str().unwrap_or(""))
                .unwrap_or_else(|_| formats::model_reference(owner));
            if let Some(backup) = backup
                && let Some(owner) = owner.as_mapping_mut()
            {
                owner.entry(Value::from("id")).or_insert(backup);
            }
            model.insert("encounter".into(), owner);
        }
    } else {
        for (flat, old) in [("splat_rounds", "limits"), ("splat_sides", "sides")] {
            if let Some(value) = fields.remove(flat) {
                if old == "limits" {
                    model
                        .entry(Value::from("limits"))
                        .or_insert_with(|| Value::Mapping(Mapping::new()))["rounds"] = value;
                } else {
                    let sides = if let Some(names) = value.as_sequence() {
                        let saved = model
                            .remove("sides")
                            .and_then(|v| v.as_mapping().cloned())
                            .unwrap_or_default();
                        Value::Mapping(
                            names
                                .iter()
                                .filter_map(Value::as_str)
                                .map(|name| {
                                    (
                                        Value::from(name),
                                        saved.get(name).cloned().unwrap_or_else(|| {
                                            map(&[("hostile_to", Value::Sequence(Vec::new()))])
                                        }),
                                    )
                                })
                                .collect(),
                        )
                    } else {
                        value
                    };
                    model.insert(old.into(), sides);
                }
            }
        }
        model.entry(Value::from("sides")).or_insert_with(|| {
            serde_yaml_ng::from_str(
                "party: {hostile_to: [enemies]}\nenemies: {hostile_to: [party]}",
            )
            .unwrap()
        });
        let limits = model
            .entry(Value::from("limits"))
            .or_insert_with(|| Value::Mapping(Mapping::new()));
        if let Some(limits) = limits.as_mapping_mut() {
            limits.entry(Value::from("rounds")).or_insert(20.into());
            limits
                .entry(Value::from("resolution_events"))
                .or_insert(10000.into());
        }
        model
            .entry(Value::from("assumptions"))
            .or_insert_with(|| Value::Sequence(Vec::new()));
        model
            .entry(Value::from("completion"))
            .or_insert_with(|| map(&[("mode", "manual".into())]));
    }
    let mut rows = Vec::new();
    let mut keys = std::collections::BTreeSet::new();
    let mut party_slots = std::collections::BTreeSet::new();
    if let Some(table) = table(
        text,
        if group { "Members" } else { "Roster" },
        if group { GROUP } else { ROSTER },
    ) {
        for (index, row) in table.rows.into_iter().enumerate() {
            let parsed = (|| {
                if row.len() != 5 {
                    return Err("Missing or extra cells".into());
                }
                let mut start_hp = None;
                let mut terms = Vec::new();
                let (stats, loot) = super::loot::split_changes(&row[3].value);
                for term in stats.split(',').map(str::trim) {
                    if !group && term.to_ascii_lowercase().starts_with("start hp ") {
                        let hp = term[9..]
                            .trim()
                            .parse::<i64>()
                            .ok()
                            .filter(|hp| *hp >= 0)
                            .ok_or_else(|| unread_change(term))?;
                        if start_hp.replace(hp).is_some() {
                            return Err("Changes: duplicate Start HP".into());
                        }
                    } else {
                        terms.push(term);
                    }
                }
                let mut text = terms.join(", ");
                if let Some(loot) = loot {
                    text.push_str(&format!(", Loot {loot}"));
                }
                let mut overrides = changes(&text, &row[2].value, &visible(&row[4].value))?;
                if group {
                    let creature = reference(&row[1].value)?;
                    if let Some(extra) = model
                        .get("members_data")
                        .and_then(|v| v.get(index))
                        .and_then(Value::as_mapping)
                    {
                        for (key, value) in extra {
                            overrides.entry(key.clone()).or_insert(value.clone());
                        }
                    }
                    let count = if row[0].value.is_empty() {
                        1
                    } else {
                        row[0]
                            .value
                            .parse::<u64>()
                            .map_err(|_| "Count: use a positive integer")?
                    };
                    if !(1..=super::encounter_group::MAX_COUNT).contains(&count) {
                        return Err("Count: use 1 to 100".into());
                    }
                    let mut member = map(&[("creature", creature), ("count", count.into())]);
                    if !overrides.is_empty() {
                        member["overrides"] = overrides.into();
                    }
                    Ok(member)
                } else {
                    let original_slot = machine(&row[4].value, "slot")
                        .unwrap_or_else(|| format!("actor_line_{}", row[0].line));
                    let mut slot = original_slot.clone();
                    let mut suffix = 1;
                    while !keys.insert(slot.clone()) {
                        slot = format!("actor_line_{}_{}", row[0].line, suffix);
                        suffix += 1;
                    }
                    let mut extra = model
                        .get("slots")
                        .and_then(|v| v.get(original_slot.as_str()))
                        .and_then(Value::as_mapping)
                        .cloned()
                        .unwrap_or_default();
                    if let Some(party) = extra.get("party_slot").and_then(Value::as_str)
                        && !party_slots.insert(party.to_owned())
                    {
                        return Err("This party member is already in the roster".into());
                    }
                    if slot != original_slot {
                        extra.clear();
                    }
                    if row[1].value.is_empty() && extra.contains_key("party_slot") {
                        extra.remove("creature");
                    } else {
                        extra.insert("creature".into(), reference(&row[1].value)?);
                    }
                    let mut side = row[0].value.clone();
                    let explicit_role = side.ends_with(" (player)") || side.ends_with(" (GM)");
                    let role = if side.ends_with(" (player)") {
                        side.truncate(side.len() - 9);
                        "player"
                    } else if side.ends_with(" (GM)") {
                        side.truncate(side.len() - 5);
                        "gm"
                    } else if side.eq_ignore_ascii_case("party") {
                        "player"
                    } else {
                        "gm"
                    };
                    let side = model
                        .get("sides")
                        .and_then(Value::as_mapping)
                        .and_then(|sides| {
                            sides
                                .keys()
                                .filter_map(Value::as_str)
                                .find(|key| key.eq_ignore_ascii_case(&side))
                        })
                        .unwrap_or(&side)
                        .to_owned();
                    if side.is_empty() {
                        return Err("Side not set".into());
                    }
                    if let Some(hp) = start_hp {
                        let initial = extra
                            .entry(Value::from("initial"))
                            .or_insert_with(|| map(&[]));
                        initial
                            .as_mapping_mut()
                            .ok_or("Invalid starting values")?
                            .insert("hp".into(), hp.into());
                    }
                    extra.insert("key".into(), slot.into());
                    extra.insert("side".into(), side.into());
                    extra.entry(Value::from("controller")).or_insert_with(|| {
                        map(&[("kind", "manual".into()), ("role", role.into())])
                    });
                    if explicit_role {
                        extra["controller"]["role"] = role.into();
                    }
                    let mut merged = extra
                        .remove("overrides")
                        .and_then(|v| v.as_mapping().cloned())
                        .unwrap_or_default();
                    merged.extend(overrides);
                    extra.insert("overrides".into(), merged.into());
                    Ok(extra.into())
                }
            })();
            match parsed {
                Ok(value) => rows.push(value),
                Err(message) => flags.push(Flag::new(row.first().map_or(1, |c| c.line), message)),
            }
        }
    } else {
        flags.push(Flag::new(1, "Member table not found".into()));
    }
    model.remove("slots");
    model.remove("members_data");
    model.insert(
        if group { "members" } else { "roster" }.into(),
        Value::Sequence(rows),
    );
    fields.insert(key.into(), model.into());
}
fn set_selection(value: &Value, selection: Value) -> Value {
    let mut value = value.clone();
    value["selection"] = selection;
    value
}
fn cell(value: &str) -> String {
    value
        .replace("\\|", "|")
        .replace('|', "\\|")
        .replace(['\n', '\r'], " ")
}
fn row(values: &[String]) -> String {
    format!(
        "| {} |\n",
        values
            .iter()
            .map(|value| cell(value))
            .collect::<Vec<_>>()
            .join(" | ")
    )
}
fn header(headers: &[&str]) -> String {
    format!(
        "\n{}{}",
        row(&headers.iter().map(|s| s.to_string()).collect::<Vec<_>>()),
        row(&headers.iter().map(|_| "---".into()).collect::<Vec<_>>())
    )
}
fn choice_name(choice: &Value) -> &str {
    choice
        .get("link")
        .and_then(Value::as_str)
        .filter(|name| !name.trim().is_empty() && Uuid::parse_str(name).is_err())
        .unwrap_or("Campaign initiative")
}

/// Generate just the owned list/table and flat fields; all other machine values stay at the end.
type Rendered = (Vec<(&'static str, Option<Value>)>, String, Mapping);
fn render(fields: &Mapping) -> Result<Rendered, String> {
    let kind = fields
        .get("splat_kind")
        .and_then(Value::as_str)
        .unwrap_or("");
    let mut front = Vec::new();
    let mut body = String::new();
    let mut data = Mapping::new();
    if kind == "campaign" {
        body.push_str("\n## Packages\n\n");
        for package in fields
            .get("splat_packages")
            .and_then(Value::as_sequence)
            .into_iter()
            .flatten()
        {
            let id = words(&package["package"], "id");
            if id.is_empty() {
                return Err("Package needs an id".into());
            }
            let name = package.get("name").and_then(Value::as_str).unwrap_or(id);
            body.push_str(&format!("- [x] {} ^{id}\n", cell(name)));
            if words(&package["selection"], "mode") == "selected" {
                body.push_str(&format!(
                    "  - Only: {}\n",
                    seq(&package["selection"], "entries")
                        .iter()
                        .map(show_reference)
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }
        front.push(("splat_packages", None));
        body.push_str("\n## Rule choices\n");
        body.push_str(&header(RULES));
        if let Some(choices) = fields.get("splat_rule_choices").and_then(Value::as_mapping) {
            for (slot, choice) in choices {
                let slot = slot.as_str().ok_or("Rule slot must be text")?;
                let id = words(choice, "id");
                if id.is_empty() {
                    return Err("Rule choice needs an id".into());
                }
                body.push_str(&row(&[
                    cell(if slot == "dnd2024:rules/combat_initiative" {
                        "Combat initiative"
                    } else {
                        slot
                    }),
                    format!("{} %% splat rule={slot} id={id} %%", choice_name(choice)),
                ]));
            }
        }
        front.push(("splat_rule_choices", None));
        for (old, flat) in [
            ("splat_run_storage", "splat_run_folder"),
            ("splat_session_storage", "splat_session_folder"),
        ] {
            if let Some(value) = fields.get(old) {
                if value.as_mapping().is_none_or(|m| m.len() != 1)
                    || value.get("directory").and_then(Value::as_str).is_none()
                {
                    return Err(format!("{old} needs review"));
                }
                front.push((old, None));
                front.push((flat, Some(value["directory"].clone())));
            }
        }
        return Ok((front, body, data));
    }
    let group = kind == "encounter_group";
    let key = if group {
        "splat_encounter_group"
    } else {
        "splat_encounter"
    };
    let mut model = fields
        .get(key)
        .and_then(Value::as_mapping)
        .cloned()
        .ok_or("Missing encounter model")?;
    let members = model
        .remove(if group { "members" } else { "roster" })
        .unwrap_or_else(|| Value::Sequence(Vec::new()));
    front.push((key, None));
    if group {
        front.push(("splat_side", model.remove("side")));
        let owner = model.remove("encounter");
        let flat = owner.as_ref().map(|owner| {
            if let Some(link) = owner.get("link").and_then(Value::as_str) {
                if let Some(id) = owner.get("id") {
                    model.insert("encounter_backup".into(), id.clone());
                }
                Value::from(link)
            } else {
                formats::flat_reference(owner)
            }
        });
        front.push(("splat_encounter", flat));
    }
    body.push_str(if group {
        "\n## Members\n"
    } else {
        "\n## Roster\n"
    });
    body.push_str(&header(if group { GROUP } else { ROSTER }));
    let mut slots = Mapping::new();
    let mut members_data = Vec::new();
    for member in members.as_sequence().into_iter().flatten() {
        let overrides = member
            .get("overrides")
            .cloned()
            .unwrap_or_else(|| Value::Mapping(Mapping::new()));
        let mut overrides_left = overrides.as_mapping().cloned().ok_or("Invalid overrides")?;
        for (_, key) in STATS {
            overrides_left.remove(*key);
        }
        for key in ["name", "note", "conditions", "side", "loot"] {
            overrides_left.remove(key);
        }
        members_data.push(Value::Mapping(overrides_left.clone()));
        let mut stat_changes = overrides.clone();
        if let Some(changes) = stat_changes.as_mapping_mut() {
            changes.remove("loot");
        }
        let mut shown_changes = show_changes(&stat_changes);
        if !group
            && let Some(hp) = member
                .get("initial")
                .and_then(|initial| initial.get("hp"))
                .and_then(Value::as_i64)
        {
            if !shown_changes.is_empty() {
                shown_changes.push_str(", ");
            }
            shown_changes.push_str(&format!("Start HP {hp}"));
        }
        // Loot is last, because its words can hold commas.
        if let Some(loot) = overrides.get("loot").and_then(Value::as_str) {
            if !shown_changes.is_empty() {
                shown_changes.push_str(", ");
            }
            shown_changes.push_str(&format!("Loot {loot}"));
        }
        let mut note = cell(words(&overrides, "note"));
        let first = if group {
            member
                .get("count")
                .and_then(Value::as_u64)
                .unwrap_or(1)
                .to_string()
        } else {
            let key = words(member, "key");
            note.push_str(&format!(" %% splat slot={key} %%"));
            let mut extra = member.as_mapping().cloned().ok_or("Invalid roster slot")?;
            for key in ["key", "creature", "side", "overrides"] {
                extra.remove(key);
            }
            if !overrides_left.is_empty() {
                extra.insert("overrides".into(), overrides_left.into());
            }
            let side = words(member, "side");
            let default_role = if side == "party" { "player" } else { "gm" };
            let role = words(&member["controller"], "role");
            if words(&member["controller"], "kind") == "manual"
                && member["controller"]
                    .as_mapping()
                    .is_some_and(|m| m.len() == 2)
            {
                extra.remove("controller");
            }
            if let Some(initial) = extra.get_mut("initial").and_then(Value::as_mapping_mut) {
                initial.remove("hp");
            }
            if extra
                .get("initial")
                .and_then(Value::as_mapping)
                .is_some_and(Mapping::is_empty)
            {
                extra.remove("initial");
            }
            if !extra.is_empty() {
                slots.insert(key.into(), extra.into());
            }
            if ["player", "gm"].contains(&role) && role != default_role {
                format!(
                    "{side} ({})",
                    if role == "player" { "player" } else { "GM" }
                )
            } else {
                side.to_owned()
            }
        };
        body.push_str(&row(&[
            cell(&first),
            if member.get("creature").is_none_or(Value::is_null) {
                String::new()
            } else {
                show_reference(&member["creature"])
            },
            cell(words(&overrides, "name")),
            shown_changes,
            note,
        ]));
    }
    if group
        && members_data
            .iter()
            .any(|v| v.as_mapping().is_some_and(|m| !m.is_empty()))
    {
        model.insert("members_data".into(), members_data.into());
    }
    if !slots.is_empty() {
        model.insert("slots".into(), slots.into());
    }
    if !group {
        if let Some(limits) = model.get_mut("limits").and_then(Value::as_mapping_mut) {
            if let Some(rounds) = limits.remove("rounds") {
                front.push((
                    "splat_rounds",
                    (rounds.as_u64() != Some(20)).then_some(rounds),
                ));
            }
            if limits.get("resolution_events") == Some(&Value::from(10000)) {
                limits.remove("resolution_events");
            }
            if limits.is_empty() {
                model.remove("limits");
            }
        }
        // Defaults have no separate source in schema 2.
        if model.get("limits")
            == Some(&map(&[
                ("rounds", 20.into()),
                ("resolution_events", 10000.into()),
            ]))
        {
            model.remove("limits");
        }
        if model.get("completion") == Some(&map(&[("mode", "manual".into())])) {
            model.remove("completion");
        }
        if model
            .get("assumptions")
            .and_then(Value::as_sequence)
            .is_some_and(Vec::is_empty)
        {
            model.remove("assumptions");
        }
        let defaults: Value = serde_yaml_ng::from_str(
            "party: {hostile_to: [enemies]}\nenemies: {hostile_to: [party]}",
        )
        .unwrap();
        if model.get("sides") == Some(&defaults) {
            model.remove("sides");
            front.push(("splat_sides", None));
        } else if let Some(sides) = model.get("sides").and_then(Value::as_mapping) {
            front.push((
                "splat_sides",
                Some(Value::Sequence(sides.keys().cloned().collect())),
            ));
        }
    }
    if !model.is_empty() {
        data.insert(key.into(), model.into());
    }
    Ok((front, body, data))
}

pub(super) fn canonical(fields: &Mapping) -> Mapping {
    let mut fields = fields.clone();
    fields.remove("splat_schema");
    if let Some(model) = fields
        .get_mut("splat_encounter")
        .and_then(Value::as_mapping_mut)
    {
        let defaults: Mapping = serde_yaml_ng::from_str("roster: []\nsides: {party: {hostile_to: [enemies]}, enemies: {hostile_to: [party]}}\nlimits: {rounds: 20, resolution_events: 10000}\nassumptions: []\ncompletion: {mode: manual}\n").unwrap();
        for (key, value) in defaults {
            model.entry(key).or_insert(value);
        }
    }

    if let Some(members) = fields
        .get_mut("splat_encounter_group")
        .and_then(|v| v.get_mut("members"))
        .and_then(Value::as_sequence_mut)
    {
        for member in members {
            if let Some(member) = member.as_mapping_mut() {
                member.entry(Value::from("count")).or_insert(1.into());
            }
        }
    }
    if let Some(packages) = fields
        .get_mut("splat_packages")
        .and_then(Value::as_sequence_mut)
    {
        for package in packages {
            package.as_mapping_mut().unwrap().remove("name");
        }
    }
    for key in ["splat_packages", "splat_rule_choices"] {
        if fields.get(key).is_some_and(|v| {
            v.as_sequence().is_some_and(Vec::is_empty)
                || v.as_mapping().is_some_and(Mapping::is_empty)
        }) {
            fields.remove(key);
        }
    }
    fn tidy(value: &mut Value) {
        match value {
            Value::Mapping(m) => {
                if m.contains_key("id") || m.contains_key("key") {
                    m.remove("link");
                }
                for v in m.values_mut() {
                    tidy(v);
                }
                for key in ["overrides", "initial"] {
                    if m.get(key)
                        .and_then(Value::as_mapping)
                        .is_some_and(Mapping::is_empty)
                    {
                        m.remove(key);
                    }
                }
            }
            Value::Sequence(a) => {
                for v in a {
                    tidy(v)
                }
            }
            _ => {}
        }
    }
    for v in fields.values_mut() {
        tidy(v);
    }
    fields
}

/// Write a list note. `entry` is the model of `fields` and `body`.
pub(super) fn write(entry: &Entry, fields: &Mapping, body: &str) -> Result<String, String> {
    let (changes, lists, data) = render(&entry.fields)?;
    let newline = if body.contains("\r\n") { "\r\n" } else { "\n" };
    let mut after = formats::note(fields, changes, body)?;
    after.push_str(&lists.replace('\n', newline));
    if !data.is_empty() {
        after.push_str(
            &format!(
                "\n## Splat data\n\n<!-- splat:yaml path=splat_list_data -->\n\n```yaml\n{}```\n",
                serde_yaml_ng::to_string(&data).map_err(|e| e.to_string())?
            )
            .replace('\n', newline),
        );
    }
    let checked = Entry::parse(&entry.path, &after)?;
    if canonical(&entry.fields) != canonical(&checked.fields) || !checked.flags.is_empty() {
        return Err("The written note does not read back to the same values.".into());
    }
    Ok(formats::folded_data(&after))
}

pub fn patch(text: &str, changes: &[(&str, Option<Value>)]) -> Result<String, String> {
    let original = Entry::parse("note.md", text)?;
    let mut expected = (*original.fields).clone();
    for (key, value) in changes {
        if let Some(value) = value {
            expected.insert((*key).into(), value.clone());
        } else {
            expected.remove(*key);
        }
    }
    let mut after = text.to_owned();
    for (key, value) in changes {
        let needs_slot_keys = *key == "splat_encounter"
            && table(text, "Roster", ROSTER).is_some_and(|table| {
                table
                    .rows
                    .iter()
                    .zip(seq(&original.fields["splat_encounter"], "roster"))
                    .any(|(row, slot)| {
                        row.get(4)
                            .and_then(|cell| machine(&cell.value, "slot"))
                            .as_deref()
                            != Some(words(&slot, "key"))
                            || (slot
                                .get("initial")
                                .and_then(|initial| initial.get("hp"))
                                .is_some()
                                && !row[3].value.to_ascii_lowercase().contains("start hp "))
                    })
            });
        if original.fields.get(*key) == value.as_ref() && !needs_slot_keys {
            continue;
        }
        if *key == "splat_packages" {
            let wanted = value
                .as_ref()
                .and_then(Value::as_sequence)
                .cloned()
                .unwrap_or_default();
            let mut seen = std::collections::BTreeSet::new();
            let mut offset = 0;
            let mut patches = Vec::new();
            let mut in_packages = false;
            for line in after.split_inclusive('\n') {
                if line.starts_with("## ") {
                    in_packages = line.trim().eq_ignore_ascii_case("## Packages");
                }
                if in_packages
                    && ["- ", "* ", "+ "]
                        .iter()
                        .any(|prefix| line.trim_start().starts_with(prefix))
                    && matches!(line.trim_start().get(2..5), Some("[x]" | "[X]" | "[ ]"))
                {
                    let indent = line.len() - line.trim_start().len();
                    let item = &line.trim()[6..];
                    let (name, id) = item.rsplit_once(" ^").unwrap_or((item, ""));
                    let selected = wanted.iter().find(|p| {
                        if id.is_empty() {
                            words(p, "name") == name || words(&p["package"], "link") == name
                        } else {
                            words(&p["package"], "id") == id
                        }
                    });
                    let enabled = selected.is_some();
                    if let Some(package) = selected {
                        let resolved = words(&package["package"], "id");
                        seen.insert(resolved.to_owned());
                        if id.is_empty() && !resolved.is_empty() {
                            let at = offset + line.trim_end().len();
                            patches.push((at..at, format!(" ^{resolved}")));
                        }
                    }
                    patches.push((
                        offset + indent + 3..offset + indent + 4,
                        if enabled { "x" } else { " " }.to_owned(),
                    ));
                }
                offset += line.len();
            }
            for (range, value) in patches.into_iter().rev() {
                after.replace_range(range, &value);
            }
            for package in &wanted {
                let id = words(&package["package"], "id");
                let cached = package.get("name").and_then(Value::as_str);
                if let Some(name) = cached
                    && let Some(old) = original
                        .fields
                        .get("splat_packages")
                        .and_then(Value::as_sequence)
                        .and_then(|a| a.iter().find(|p| words(&p["package"], "id") == id))
                    && old.get("name").and_then(Value::as_str) != Some(name)
                {
                    let mut offset = 0;
                    for line in after.split_inclusive('\n') {
                        if line.trim_end().ends_with(&format!(" ^{id}")) {
                            let range = offset + (line.len() - line.trim_start().len()) + 6
                                ..offset + line.trim_end().len() - id.len() - 2;
                            after.replace_range(range, name);
                            break;
                        }
                        offset += line.len();
                    }
                }
                let old = original
                    .fields
                    .get("splat_packages")
                    .and_then(Value::as_sequence)
                    .and_then(|a| a.iter().find(|p| words(&p["package"], "id") == id));
                if old.and_then(|p| p.get("selection")) == package.get("selection") {
                    continue;
                }
                let mut offset = 0;
                let mut child_range = None;
                let mut insertion = None;
                for line in after.split_inclusive('\n') {
                    if insertion.is_some() && !line.starts_with([' ', '\t']) {
                        break;
                    }
                    if line.trim_end().ends_with(&format!(" ^{id}")) {
                        insertion = Some(offset + line.len());
                    } else if insertion.is_some()
                        && line.starts_with([' ', '\t'])
                        && ["- Only:", "* Only:", "+ Only:"]
                            .iter()
                            .any(|prefix| line.trim_start().starts_with(prefix))
                    {
                        child_range = Some(offset..offset + line.len());
                    }
                    offset += line.len();
                }
                let replacement = if words(&package["selection"], "mode") == "selected" {
                    format!(
                        "  - Only: {}\n",
                        seq(&package["selection"], "entries")
                            .iter()
                            .map(show_reference)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                } else {
                    String::new()
                };
                if let Some(range) = child_range {
                    after.replace_range(range, &replacement);
                } else if let Some(at) = insertion {
                    after.insert_str(at, &replacement);
                }
            }
            for package in wanted
                .iter()
                .filter(|p| !seen.contains(words(&p["package"], "id")))
            {
                let mut fields = Mapping::new();
                fields.insert("splat_kind".into(), "campaign".into());
                fields.insert(
                    "splat_packages".into(),
                    Value::Sequence(vec![package.clone()]),
                );
                let (_, body, _) = render(&fields)?;
                let task = body
                    .split("## Packages\n\n")
                    .nth(1)
                    .unwrap()
                    .split("\n## Rule choices")
                    .next()
                    .unwrap();
                let at = after.find("\n## Rule choices").unwrap_or(after.len());
                after.insert_str(at, task);
            }
        } else if ["splat_run_storage", "splat_session_storage"].contains(key) {
            after = patch_fields(
                &after,
                &[(
                    if *key == "splat_run_storage" {
                        "splat_run_folder"
                    } else {
                        "splat_session_folder"
                    },
                    value.as_ref().and_then(|v| v.get("directory")).cloned(),
                )],
            )?;
        } else if *key == "splat_rule_choices" {
            let wanted = value
                .as_ref()
                .and_then(Value::as_mapping)
                .cloned()
                .unwrap_or_default();
            let old = original
                .fields
                .get("splat_rule_choices")
                .and_then(Value::as_mapping)
                .cloned()
                .unwrap_or_default();
            let mut keys: std::collections::BTreeSet<String> = old
                .keys()
                .chain(wanted.keys())
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect();
            for slot in std::mem::take(&mut keys) {
                if old.get(slot.as_str()) == wanted.get(slot.as_str()) {
                    continue;
                }
                let found = table(&after, "Rule choices", RULES).and_then(|t| {
                    t.rows.into_iter().find(|r| {
                        r.len() == 2
                            && (machine(&r[1].value, "rule").as_deref() == Some(slot.as_str())
                                || r[0].value == slot
                                || (r[0].value.eq_ignore_ascii_case("Combat initiative")
                                    && slot == "dnd2024:rules/combat_initiative"))
                    })
                });
                let replacement = wanted.get(slot.as_str()).map(|choice| {
                    format!(
                        "{} %% splat rule={slot} id={} %%",
                        choice_name(choice),
                        words(choice, "id")
                    )
                });
                match (found, replacement) {
                    (Some(row), Some(value)) => {
                        after = super::layout::replace_cell(
                            &after,
                            "Rule choices",
                            RULES,
                            &row[0].value,
                            1,
                            &value,
                        )?
                    }
                    (Some(row), None) => {
                        let start = after[..row[0].range.start].rfind('\n').map_or(0, |n| n + 1);
                        let end = after[row[0].range.start..]
                            .find('\n')
                            .map_or(after.len(), |n| row[0].range.start + n + 1);
                        after.replace_range(start..end, "");
                    }
                    (None, Some(value)) => {
                        if let Some(t) = table(&after, "Rule choices", RULES) {
                            let last = t
                                .rows
                                .last()
                                .and_then(|r| r.first())
                                .unwrap_or(&t.header[0]);
                            let mut at = after[last.range.start..]
                                .find('\n')
                                .map_or(after.len(), |n| last.range.start + n + 1);
                            if t.rows.is_empty() {
                                at = after[at..].find('\n').map_or(after.len(), |n| at + n + 1);
                            }
                            after.insert_str(at, &row(&[slot, value]));
                        } else {
                            after.push_str(&format!(
                                "\n## Rule choices\n{}{}",
                                header(RULES),
                                row(&[slot, value])
                            ));
                        }
                    }
                    _ => {}
                }
            }
        } else if ["splat_encounter_group", "splat_encounter"].contains(key) {
            let group = original.kind == "encounter_group";
            let section = if group { "Members" } else { "Roster" };
            let headers = if group { GROUP } else { ROSTER };
            let (front, body, data) = render(&expected)?;
            let proposed = format!("# Note\n{body}");
            let old = table(&after, section, headers).ok_or("Member table not found")?;
            let next = table(&proposed, section, headers).ok_or("Member table not found")?;
            if !original.flags.is_empty() {
                return Err("Repair flagged table rows before saving this list.".into());
            }
            let old_model = original.fields[*key]
                .get(if group { "members" } else { "roster" })
                .and_then(Value::as_sequence)
                .cloned()
                .unwrap_or_default();
            let new_model = expected[*key]
                .get(if group { "members" } else { "roster" })
                .and_then(Value::as_sequence)
                .cloned()
                .unwrap_or_default();
            let mut patches = Vec::new();
            for (index, (old, new)) in old.rows.iter().zip(&next.rows).enumerate() {
                if old.len() != 5 || new.len() != 5 {
                    return Err("Repair flagged table rows before saving".into());
                }
                let old_value = &old_model[index];
                let new_value = &new_model[index];
                let changed = [
                    old_value.get(if group { "count" } else { "side" })
                        != new_value.get(if group { "count" } else { "side" }),
                    clean_reference(old_value.get("creature"))
                        != clean_reference(new_value.get("creature")),
                    old_value.get("overrides").and_then(|v| v.get("name"))
                        != new_value.get("overrides").and_then(|v| v.get("name")),
                    show_changes(&old_value["overrides"]) != show_changes(&new_value["overrides"])
                        || old_value.get("initial").and_then(|v| v.get("hp"))
                            != new_value.get("initial").and_then(|v| v.get("hp"))
                        || (!group
                            && new_value.get("initial").and_then(|v| v.get("hp")).is_some()
                            && !old[3].value.to_ascii_lowercase().contains("start hp ")),
                    old_value.get("overrides").and_then(|v| v.get("note"))
                        != new_value.get("overrides").and_then(|v| v.get("note"))
                        || old_value.get("key") != new_value.get("key")
                        || (!group
                            && machine(&old[4].value, "slot").as_deref()
                                != new_value.get("key").and_then(Value::as_str)),
                ];
                for (column, (a, b)) in old.iter().zip(new).enumerate() {
                    if changed[column] && a.value != b.value {
                        patches.push((a.range.clone(), cell(&b.value)));
                    }
                }
            }
            for row in old.rows.iter().skip(next.rows.len()) {
                let start = after[..row[0].range.start].rfind('\n').map_or(0, |n| n + 1);
                let end = after[row[0].range.start..]
                    .find('\n')
                    .map_or(after.len(), |n| row[0].range.start + n + 1);
                patches.push((start..end, String::new()));
            }
            if next.rows.len() > old.rows.len() {
                let at = old.rows.last().map_or_else(
                    || {
                        let start = old.header[0].range.start;
                        let header_end = after[start..]
                            .find('\n')
                            .map(|n| start + n + 1)
                            .unwrap_or(after.len());
                        after[header_end..]
                            .find('\n')
                            .map(|n| header_end + n + 1)
                            .unwrap_or(after.len())
                    },
                    |r| {
                        after[r[0].range.start..]
                            .find('\n')
                            .map_or(after.len(), |n| r[0].range.start + n + 1)
                    },
                );
                let mut addition = next
                    .rows
                    .iter()
                    .skip(old.rows.len())
                    .map(|r| row(&r.iter().map(|c| c.value.clone()).collect::<Vec<_>>()))
                    .collect::<String>();
                if at > 0 && !after[..at].ends_with('\n') {
                    addition.insert(0, '\n');
                }
                patches.push((at..at, addition));
            }
            for (range, value) in patches.into_iter().rev() {
                after.replace_range(range, &value);
            }
            after = patch_fields(
                &after,
                &front
                    .into_iter()
                    .filter(|(key, _)| {
                        *key != if group {
                            "splat_encounter_group"
                        } else {
                            "splat_encounter"
                        }
                    })
                    .collect::<Vec<_>>(),
            )?;
            after = patch_data(&after, &data)?;
        } else {
            after = patch_fields(&after, &[(*key, value.clone())])?;
        }
    }
    let checked = Entry::parse("note.md", &after)?;
    if canonical(&checked.fields) != canonical(&expected) {
        return Err("Edit changed another value. The file was kept.".into());
    }
    super::layout::replace_value(
        text,
        0..text.len(),
        &after,
        &["fields"],
        Value::Mapping(canonical(&expected)),
        |raw| {
            Ok(map(&[(
                "fields",
                Value::Mapping(canonical(&Entry::parse("note.md", raw)?.fields)),
            )]))
        },
    )
}
fn clean_reference(value: Option<&Value>) -> Option<Value> {
    value.map(|value| {
        let mut value = value.clone();
        if let Some(map) = value.as_mapping_mut()
            && (map.contains_key("id") || map.contains_key("key"))
        {
            map.remove("link");
        }
        value
    })
}
fn patch_data(text: &str, data: &Mapping) -> Result<String, String> {
    let mut existing = Mapping::new();
    super::markdown::read(text, &mut existing)?;
    if existing.get("splat_list_data").and_then(Value::as_mapping) == Some(data)
        || (data.is_empty() && !existing.contains_key("splat_list_data"))
    {
        return Ok(text.into());
    }
    let marker = "<!-- splat:yaml path=splat_list_data -->";
    let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
    let encoded = serde_yaml_ng::to_string(data)
        .map_err(|e| e.to_string())?
        .replace('\n', newline);
    if let Some(at) = text.find(marker) {
        let open = text[at..].find("```yaml").ok_or("Missing data fence")? + at;
        let start = text[open..].find('\n').ok_or("Missing data line")? + open + 1;
        let end = text[start..].find("```").ok_or("Missing data end")? + start;
        let folded = text[start..].starts_with('>');
        let end = if folded {
            text[..end].rfind('>').unwrap_or(end)
        } else {
            end
        };
        let encoded = if folded {
            encoded
                .lines()
                .map(|line| format!("> {line}{newline}"))
                .collect::<String>()
        } else {
            encoded
        };
        let mut after = text.to_owned();
        after.replace_range(start..end, &encoded);
        Ok(after)
    } else if data.is_empty() {
        Ok(text.into())
    } else {
        Ok(formats::folded_data(&format!(
            "{text}{newline}## Splat data{newline}{newline}{marker}{newline}{newline}```yaml{newline}{encoded}```{newline}"
        )))
    }
}

fn visible_target(raw: &str) -> &str {
    raw.split_once("](splat:")
        .and_then(|(_, tail)| tail.strip_suffix(')'))
        .unwrap_or(raw)
}

fn resolve_reference(reference: &Value, entries: &[&Entry]) -> Result<Uuid, String> {
    if let Some(link) = reference.get("link").and_then(Value::as_str)
        && let Ok(id) = formats::resolve_readable(
            visible_target(&link.replace("\\|", "|")),
            entries.iter().copied(),
        )
    {
        return Ok(id);
    }
    let raw = reference
        .get("key")
        .or_else(|| reference.get("id"))
        .or_else(|| reference.get("link"))
        .and_then(Value::as_str)
        .ok_or("Creature reference not read")?;
    formats::resolve_readable(raw, entries.iter().copied())
}

/// Save resolved cells with the catalogue's name, preserving all other cells and prose.
pub fn write_references<'a>(
    text: &str,
    entries: impl IntoIterator<Item = &'a Entry>,
) -> Result<String, String> {
    let entries: Vec<_> = entries.into_iter().collect();
    let entry = Entry::parse("note.md", text)?;
    let mut patches = Vec::new();
    if ["encounter", "encounter_group"].contains(&entry.kind.as_str()) {
        let group = entry.kind == "encounter_group";
        if let Some(table) = table(
            text,
            if group { "Members" } else { "Roster" },
            if group { GROUP } else { ROSTER },
        ) {
            for row in table.rows {
                let Some(cell) = row.get(1).filter(|cell| !cell.value.is_empty()) else {
                    continue;
                };
                let reference = reference(&cell.value)?;
                let id = match resolve_reference(&reference, &entries) {
                    Ok(id) => id,
                    Err(_) if reference.get("id").is_some() || reference.get("key").is_some() => {
                        continue;
                    }
                    Err(error) => return Err(error),
                };
                let target = entries
                    .iter()
                    .find(|entry| entry.id == id)
                    .ok_or("Creature not found")?;
                let shown = if let Some(key) = &target.key {
                    // Keep a hand-written alias while refreshing its backup identity.
                    let visible = visible(&cell.value);
                    let label = visible
                        .strip_prefix('[')
                        .and_then(|link| link.split_once("](splat:"))
                        .map(|(label, _)| label)
                        .filter(|label| {
                            *label != key.rsplit('/').next().unwrap_or(key).replace('_', " ")
                        })
                        .unwrap_or(&target.name);
                    let backup = if reference.get("key").is_none() {
                        format!(" %% splat id={} %%", target.id)
                    } else {
                        String::new()
                    };
                    format!("[{label}](splat:{key}){backup}")
                } else {
                    format!(
                        "[[{}|{}]] %% splat id={} %%",
                        target.path.trim_end_matches(".md"),
                        target.name,
                        target.id
                    )
                };
                if cell.value != shown {
                    patches.push((cell.range.clone(), self::cell(&shown)));
                }
            }
        }
    }
    // Only: cells use the same catalogue labels as roster cells.
    if entry.kind == "campaign" {
        let mut offset = 0;
        for line in text.split_inclusive('\n') {
            if ["- Only:", "* Only:", "+ Only:"]
                .iter()
                .any(|prefix| line.trim_start().starts_with(prefix))
            {
                let start = line.find("Only:").unwrap() + "Only:".len();
                let raw = line[start..].trim();
                let refs = only_references(raw)?;
                let mut shown = Vec::new();
                for reference in refs {
                    let target = resolve_reference(&reference, &entries)
                        .ok()
                        .and_then(|id| entries.iter().find(|entry| entry.id == id));
                    shown.push(if let Some(target) = target {
                        let backup = reference
                            .get("id")
                            .map(|_| format!(" %% splat id={} %%", target.id))
                            .unwrap_or_default();
                        if let Some(key) = &target.key {
                            format!("[{}](splat:{key}){backup}", target.name)
                        } else {
                            format!(
                                "[[{}|{}]]{backup}",
                                target.path.trim_end_matches(".md"),
                                target.name
                            )
                        }
                    } else {
                        show_reference(&reference)
                    });
                }
                patches.push((
                    offset + start..offset + line.trim_end().len(),
                    format!(" {}", shown.join(", ")),
                ));
            }
            offset += line.len();
        }
    }
    patches.sort_by_key(|(range, _)| range.start);
    let mut after = text.to_owned();
    for (range, shown) in patches.into_iter().rev() {
        after.replace_range(range, &shown);
    }
    Ok(after)
}

/// Resolve Obsidian links after all files have been read; no bytes are written.
pub fn resolve(entry: &mut Entry, entries: &[Entry], source: &str) {
    entry.flags.retain(|flag| {
        !(flag.message.starts_with("Creature ")
            && flag.message.ends_with(" not found or ambiguous"))
    });
    fn visit(value: &mut Value, entries: &[Entry], kind: Option<&str>) {
        match value {
            Value::Mapping(m) => {
                if let Some(raw) = m
                    .get("link")
                    .and_then(Value::as_str)
                    .map(|raw| raw.replace("\\|", "|"))
                    && let Ok(id) = formats::resolve_readable(
                        visible_target(&raw),
                        entries.iter().filter(|entry| match kind {
                            Some("encounter") => entry.kind == "encounter",
                            Some("creature") => {
                                ["creature", "world"].contains(&entry.kind.as_str())
                            }
                            Some("package") => entry.kind == "package",
                            _ => true,
                        }),
                    )
                {
                    if m.contains_key("id") {
                        m.remove("key");
                        m.insert("id".into(), id.to_string().into());
                    } else if let Some(key) = entries
                        .iter()
                        .find(|entry| entry.id == id)
                        .and_then(|entry| entry.key.as_ref())
                    {
                        m.remove("link");
                        m.insert("key".into(), key.clone().into());
                    } else {
                        m.remove("key");
                        m.insert("id".into(), id.to_string().into());
                    }
                }
                for (key, value) in m.iter_mut() {
                    let kind = key
                        .as_str()
                        .filter(|key| ["encounter", "creature", "package"].contains(key));
                    visit(value, entries, kind);
                }
            }
            Value::Sequence(a) => {
                for value in a {
                    visit(value, entries, None)
                }
            }
            _ => {}
        }
    }
    if ["encounter", "encounter_group"].contains(&entry.kind.as_str()) {
        let group = entry.kind == "encounter_group";
        if let Some(table) = table(
            source,
            if group { "Members" } else { "Roster" },
            if group { GROUP } else { ROSTER },
        ) {
            for row in table.rows {
                if let Some(cell) = row.get(1)
                    && let Ok(reference) = reference(&cell.value)
                    && reference.get("id").is_none()
                    && reference.get("key").is_none()
                    && let Some(raw) = reference.get("link").and_then(Value::as_str)
                    && formats::resolve_readable(raw, entries.iter()).is_err()
                    && !entry.flags.iter().any(|flag| flag.line == cell.line)
                {
                    entry.flags.push(Flag::new(
                        cell.line,
                        format!("Creature {raw} not found or ambiguous"),
                    ));
                }
            }
        }
    }
    if entry.kind == "campaign" {
        entry.flags.retain(|flag| flag.kind != FlagKind::RuleChoice);
        if let Some(table) = table(&entry.body, "Rule choices", RULES) {
            let rules: Vec<_> = entries.iter().filter(|e| e.kind == "rule").collect();
            let mut seen = std::collections::BTreeSet::new();
            for row in table.rows.into_iter().filter(|row| row.len() == 2) {
                let slot = machine(&row[1].value, "rule").unwrap_or_else(|| {
                    if row[0].value.eq_ignore_ascii_case("Combat initiative") {
                        "dnd2024:rules/combat_initiative".into()
                    } else {
                        row[0].value.clone()
                    }
                });
                if !super::resource_key(&slot) || !seen.insert(slot.clone()) {
                    continue;
                }
                let line = row[1].line + entry.body_line.saturating_sub(1);
                entry
                    .flags
                    .retain(|flag| flag.line != line || flag.message != "Rule choice id not found");
                let visible = row[1].value.split("%%").next().unwrap_or("").trim();
                let resolved = if Uuid::parse_str(visible).is_ok() {
                    Err("Use a rule name".into())
                } else {
                    formats::resolve_readable(visible, rules.iter().copied())
                }
                .or_else(|error| {
                    machine(&row[1].value, "id")
                        .and_then(|id| Uuid::parse_str(&id).ok())
                        .filter(|id| rules.iter().any(|rule| rule.id == *id))
                        .ok_or(error)
                });
                if let Some(choices) = entry
                    .fields
                    .get_mut("splat_rule_choices")
                    .and_then(Value::as_mapping_mut)
                {
                    match resolved {
                        Ok(id) => {
                            choices.insert(slot.into(), map(&[("id", id.to_string().into())]));
                        }
                        Err(_) => {
                            choices.remove(slot.as_str());
                            let same_name = rules
                                .iter()
                                .filter(|rule| rule.name.eq_ignore_ascii_case(visible))
                                .count();
                            let message = if same_name > 1 {
                                format!(
                                    "{} choice “{visible}” matches {same_name} rules. Use a link to one of them.",
                                    row[0].value
                                )
                            } else {
                                format!(
                                    "{} choice “{visible}” is not a rule. {}",
                                    row[0].value,
                                    if slot == "dnd2024:rules/combat_initiative" {
                                        "Splat uses the D&D default (groups of identical NPCs) until you fix it."
                                    } else {
                                        "Splat uses the default until you fix it."
                                    }
                                )
                            };
                            entry.flags.push(Flag {
                                kind: FlagKind::RuleChoice,
                                ..Flag::new(line, message)
                            });
                        }
                    }
                }
            }
        }
    }
    if ["campaign", "encounter", "encounter_group"].contains(&entry.kind.as_str()) {
        for value in entry.fields.values_mut() {
            visit(value, entries, None);
        }
    }
}

fn owned_ranges(body: &str, group: bool) -> Vec<std::ops::Range<usize>> {
    let section = if group { "Members" } else { "Roster" };
    let mut ranges = Vec::new();
    if let Some(table) = table(body, section, if group { GROUP } else { ROSTER }) {
        let start = body[..table.header[0].range.start]
            .rfind('\n')
            .map_or(0, |n| n + 1);
        let last = table
            .rows
            .last()
            .and_then(|r| r.first())
            .unwrap_or(&table.header[0]);
        let mut end = body[last.range.start..]
            .find('\n')
            .map_or(body.len(), |n| last.range.start + n + 1);
        if table.rows.is_empty() {
            end = body[end..].find('\n').map_or(body.len(), |n| end + n + 1);
        }
        let heading = format!("## {section}");
        let start = body[..start]
            .rfind(&heading)
            .filter(|at| body[*at + heading.len()..start].trim().is_empty())
            .unwrap_or(start);
        ranges.push(start..end);
    }
    if let Some(at) = body.find("<!-- splat:yaml path=splat_list_data -->") {
        let start = body[..at]
            .rfind("## Splat data")
            .filter(|n| body[*n + "## Splat data".len()..at].trim().is_empty())
            .unwrap_or(at);
        if let Some(open) = body[at..].find("```yaml").map(|n| at + n + 7)
            && let Some(end) = body[open..].find("```").map(|n| open + n + 3)
        {
            ranges.push(start..end);
        }
    }
    ranges.sort_by_key(|r| r.start);
    ranges
}

/// Notes forms show the DM's prose; the owned table stays in its original bytes.
pub fn prose(entry: &Entry) -> String {
    if !["encounter_group", "encounter"].contains(&entry.kind.as_str()) {
        return entry.body.trim().into();
    }
    let mut body = entry.body.clone();
    for range in owned_ranges(&body, entry.kind == "encounter_group")
        .into_iter()
        .rev()
    {
        body.replace_range(range, "");
    }
    body.trim().into()
}
pub fn with_prose(text: &str, notes: &str) -> Result<String, String> {
    let entry = Entry::parse("note.md", text)?;
    let normalized = text.replace("\r\n", "\n");
    let rest = normalized
        .strip_prefix('\u{feff}')
        .unwrap_or(&normalized)
        .strip_prefix("---\n")
        .ok_or("Missing properties")?;
    let body_at =
        normalized.len() - rest.len() + rest.find("\n---\n").ok_or("Missing property end")? + 5;
    let body = &normalized[body_at..];
    let owned = owned_ranges(body, entry.kind == "encounter_group")
        .into_iter()
        .map(|r| body[r].to_owned())
        .collect::<Vec<_>>()
        .join("\n\n");
    let after = format!("{}\n{}\n\n{}", &normalized[..body_at], notes.trim(), owned);
    Ok(if text.contains("\r\n") {
        after.replace('\n', "\r\n")
    } else {
        after
    })
}

#[cfg(test)]
mod iter6_b {
    #[test]
    fn unread_change_names_the_value_and_a_number_repair() {
        assert_eq!(
            super::changes("HP lots", "", "").unwrap_err(),
            "the change “HP lots” was not read. Use a number, such as HP +5."
        );
    }
}
