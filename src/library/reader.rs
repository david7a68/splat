//! A common reading layout for all non-creature entries.
#![warn(deprecated)] // Its parent allows old message functions; this file does not.
use super::presentation::{self};
use crate::design::reader as style;
use splat::content::Entry;

const LABELS: &[&str] = &[
    "Casting Time",
    "Range",
    "Components",
    "Duration",
    "Creature Type",
    "Size",
    "Speed",
    "Prerequisite",
    "Prerequisites",
    "Category",
    "Ability Scores",
    "Skill Proficiencies",
    "Tool Proficiency",
    "Equipment",
    "Feat",
];

/// Promote only explicit source labels. Everything else remains in the prose.
pub fn source_facts(source: &str) -> (Vec<(String, String)>, String) {
    let mut facts: Vec<(String, String)> = Vec::new();
    let mut prose = Vec::new();
    let mut preamble = true;
    for raw in source.lines() {
        let line = raw.trim().trim_end_matches('\\').trim();
        if line.is_empty() || line.starts_with("<!-- SRD page ") {
            prose.push(raw);
            continue;
        }
        if preamble
            && let Some((_, value)) = facts.last_mut()
            && value.matches('(').count() > value.matches(')').count()
        {
            value.push(' ');
            value.push_str(line);
            continue;
        }
        if preamble
            && let Some((label, value)) = line.split_once(':')
            && LABELS.contains(&label)
            && !value.trim().is_empty()
        {
            facts.push((label.to_owned(), value.trim().to_owned()));
            continue;
        }
        if preamble
            && (line.starts_with("Level ")
                || line.contains("Cantrip (")
                || line.ends_with(" Feat")
                || line.starts_with("Wondrous Item,"))
        {
            facts.push(("Identity".into(), line.to_owned()));
            continue;
        }
        preamble = false;
        prose.push(raw);
    }
    (facts, prose.join("\n"))
}

pub fn display_value(value: &serde_yaml_ng::Value) -> String {
    use serde_yaml_ng::Value;
    match value {
        Value::String(s) => s.clone(),
        Value::Bool(v) => if *v { "Yes" } else { "No" }.into(),
        Value::Null => "—".into(),
        Value::Sequence(values) => values
            .iter()
            .map(display_value)
            .collect::<Vec<_>>()
            .join(", "),
        Value::Mapping(values) => values
            .iter()
            .filter(|(_, value)| !blank(value))
            .map(|(key, value)| {
                format!(
                    "{}: {}",
                    display_value(key).replace('_', " "),
                    display_value(value)
                )
            })
            .collect::<Vec<_>>()
            .join(" · "),
        _ => serde_yaml_ng::to_string(value)
            .unwrap_or_default()
            .trim()
            .to_owned(),
    }
}

/// An item's summary, as the reader shows it under its title: its identity line, its facts, and
/// its other plain details (each a label and its value).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ItemSummary {
    pub identity: String,
    pub facts: Vec<(String, String)>,
    pub details: Vec<(String, String)>,
}

/// The summary of an item entry; `printed` is the identity line its source prints.
pub fn item_summary_parts(entry: &Entry, printed: Option<&str>) -> Option<ItemSummary> {
    let item = entry
        .fields
        .get("splat_item")
        .and_then(|v| v.as_mapping())?;
    let identity = ["weapon_category", "category", "rarity"]
        .into_iter()
        .filter_map(|key| item.get(key).map(display_value))
        .map(|value| presentation::book_case(&value))
        .collect::<Vec<_>>()
        .join(", ");
    let attunement = item
        .get("equipment")
        .and_then(|v| v.get("attunement"))
        .and_then(|v| v.as_str());
    let identity = match attunement {
        Some("required") => format!("{identity} (Requires Attunement)"),
        Some(value) if !["none", ""].contains(&value) => format!(
            "{identity} (Requires Attunement {})",
            style::human_label(value)
        ),
        _ => identity,
    };
    let identity = printed.map_or(identity, str::to_owned);
    let mut facts = Vec::new();
    for key in ["damage", "armor_class", "price", "mass"] {
        if let Some(value) = item.get(key) {
            let label = match key {
                "armor_class" => "Armor class",
                "mass" => "Weight",
                "price" => "Cost",
                _ => "Damage",
            };
            let simple = value.as_mapping().is_some_and(|m| m.len() <= 2);
            let text = if simple && let Some(value) = value.get("value") {
                format!(
                    "{} {}",
                    display_value(value),
                    item[key].get("unit").map(display_value).unwrap_or_default()
                )
            } else if simple && let Some(expression) = value.get("expression") {
                format!(
                    "{} {}",
                    display_value(expression),
                    value.get("type").map(display_value).unwrap_or_default()
                )
            } else {
                display_value(value)
            };
            facts.push((label.into(), text));
        }
    }
    if let Some(ac) = item.get("armor").and_then(|armor| armor.get("base_ac")) {
        facts.insert(0, ("Base AC".into(), display_value(ac)));
    }
    let mut details = Vec::new();
    for (key, value) in item {
        let Some(key) = key.as_str() else { continue };
        if ![
            "damage",
            "armor_class",
            "price",
            "mass",
            "category",
            "rarity",
            "weapon_category",
        ]
        .contains(&key)
        {
            if value.is_mapping() || value.is_sequence() {
                continue;
            }
            details.push((
                key.trim_end_matches("_text").replace('_', " "),
                display_value(value),
            ));
        }
    }
    Some(ItemSummary {
        identity,
        facts,
        details,
    })
}

pub fn remaining_item_detail(
    _entry: &Entry,
    key: &serde_yaml_ng::Value,
    value: &serde_yaml_ng::Value,
) -> serde_yaml_ng::Value {
    let mut value = value.clone();
    if key.as_str() == Some("equipment")
        && let Some(equipment) = value.as_mapping_mut()
    {
        equipment.remove("attunement");
    }
    value
}

pub fn has_item_details(entry: &Entry) -> bool {
    entry
        .fields
        .get("splat_item")
        .and_then(|v| v.as_mapping())
        .is_some_and(|item| {
            item.iter().any(|(key, value)| {
                let value = remaining_item_detail(entry, key, value);
                !blank(&value) && (value.is_mapping() || value.is_sequence())
            })
        })
}

/// A value with nothing to read. Sheet copies often have these, and they must not show
/// as a label with no text after it.
pub fn blank(value: &serde_yaml_ng::Value) -> bool {
    match value {
        serde_yaml_ng::Value::Null => true,
        serde_yaml_ng::Value::String(text) => text.trim().is_empty(),
        serde_yaml_ng::Value::Sequence(values) => values.iter().all(blank),
        serde_yaml_ng::Value::Mapping(values) => values.values().all(blank),
        _ => false,
    }
}

pub fn spell_facts(spell: &serde_yaml_ng::Value) -> Vec<(String, String)> {
    // Use the table's printed values, including text ranges and
    // component codes. A reader must not expose the model's internal shapes.
    let mut spell = spell.clone();
    if let Some(range) = spell
        .get("range")
        .and_then(|value| value.as_str())
        .map(str::to_owned)
    {
        spell["range_text"] = range.into();
    }
    let fields = serde_yaml_ng::Mapping::from_iter([("splat_spell".into(), spell)]);
    let body = splat::content::stat_blocks::render(&fields, "spell");
    let Some(table) = splat::content::layout::Document::new(&body)
        .section("Spell")
        .and_then(|section| section.table(&["Field", "Value"]))
    else {
        return Vec::new();
    };
    table
        .rows
        .into_iter()
        .filter_map(|row| {
            let label = &row[0].value;
            let value = row[1].value.replace("\\|", "|");
            if !["Casting Time", "Range", "Components", "Duration"].contains(&label.as_str())
                || value.is_empty()
            {
                return None;
            }
            let value = if label == "Casting Time" {
                value
                    .strip_suffix(" (Ritual)")
                    .map_or_else(|| value.clone(), |value| format!("{value} or Ritual"))
            } else {
                value
            };
            Some((label.clone(), value))
        })
        .collect()
}

/// What a spell or an encounter shows under its title: groups of facts, then an encounter's
/// completion mode.
pub fn preparation_parts(
    entry: &Entry,
    has_source: bool,
) -> (Vec<Vec<(String, String)>>, Option<String>) {
    let mut groups = Vec::new();
    let mut completion = None;
    if !has_source && let Some(spell) = entry.fields.get("splat_spell") {
        groups.push(spell_facts(spell));
    }
    if let Some(encounter) = entry.fields.get("splat_encounter") {
        let mut facts = Vec::new();
        if let Some(roster) = encounter.get("roster").and_then(|v| v.as_sequence()) {
            facts.push(("Participants".into(), roster.len().to_string()));
        }
        if let Some(sides) = encounter.get("sides").and_then(|v| v.as_mapping()) {
            facts.push(("Sides".into(), sides.len().to_string()));
        }
        if let Some(rounds) = encounter.get("limits").and_then(|v| v.get("rounds")) {
            facts.push(("Round limit".into(), display_value(rounds)));
        }
        groups.push(facts);
        completion = encounter
            .get("completion")
            .and_then(|v| v.get("mode"))
            .map(display_value);
    }
    (groups, completion)
}

/// Generated item source tables contain JSON. Hide it only if it exactly
/// matches the item fields displayed above; unmatched source always stays.
pub fn represented_item_source(entry: &Entry, source: &str) -> bool {
    let Some((caption, json)) = source.trim().split_once("\n\n") else {
        return false;
    };
    caption.starts_with("Source table, SRD 5.2.1 page ")
        && serde_yaml_ng::from_str::<serde_yaml_ng::Value>(json)
            .ok()
            .as_ref()
            .is_some_and(|value| Some(value) == entry.fields.get("splat_item"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn campaign_spell_uses_the_book_fact_order_and_true_flags_only() {
        let mut spell: serde_yaml_ng::Value = serde_yaml_ng::from_str("level: 3\nschool: evocation\ncasting_time: Action\nrange: 150 feet\ncomponents: [V, S, M]\nduration: up to 1 minute\nconcentration: false\nritual: false\n").unwrap();
        let facts = spell_facts(&spell);
        assert_eq!(
            facts
                .iter()
                .map(|(label, _)| label.as_str())
                .collect::<Vec<_>>(),
            ["Casting Time", "Range", "Components", "Duration"]
        );
        assert_eq!(facts[0].1, "Action");
        assert_eq!(facts[3].1, "up to 1 minute");
        spell["concentration"] = true.into();
        spell["ritual"] = true.into();
        let facts = spell_facts(&spell);
        assert_eq!(facts[0].1, "Action or Ritual");
        assert_eq!(facts[3].1, "Concentration, up to 1 minute");
    }

    #[test]
    fn source_facts_keep_wrapped_materials_and_unrecognised_prose() {
        let source = "Level 1 Abjuration (Wizard)\\\nCasting Time: Action\\\nComponents: V, M (a long\\\nstrand of copper wire)\\\nDuration: 1 hour\\\nKeep this custom rule.\\\nRange: This is part of the rule.";
        let (facts, prose) = source_facts(source);
        assert_eq!(facts[2].1, "V, M (a long strand of copper wire)");
        assert_eq!(facts[0].1, "Level 1 Abjuration (Wizard)");
        assert!(prose.contains("Keep this custom rule."));
        assert!(prose.contains("Range: This is part of the rule."));
        assert!(!prose.contains("Casting Time:"));
    }

    #[test]
    fn item_source_is_hidden_only_when_fully_represented() {
        let path = "definitions/equipment/longsword.md";
        let text = splat::content::definitions::source_file(path).unwrap();
        let entry = Entry::parse(path, text).unwrap();
        let sections = presentation::reading_sections(&entry.body, &entry.name);
        let source = sections
            .iter()
            .find(|s| s.title == "Source text")
            .unwrap()
            .body;
        assert!(represented_item_source(&entry, source));
        assert!(!represented_item_source(
            &entry,
            &source.replace("1d8", "2d8")
        ));
        assert!(!represented_item_source(
            &entry,
            &format!("{source}\nCustom rule.")
        ));
    }
}
