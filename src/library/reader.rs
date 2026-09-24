//! A common reading layout for all non-creature entries.
use super::presentation::{self, ReadingSection};
use crate::reader_style as style;
use eframe::egui;
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
fn source_facts(source: &str) -> (Vec<(String, String)>, String) {
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

fn display_value(value: &serde_yaml_ng::Value) -> String {
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

fn item_summary(ui: &mut egui::Ui, entry: &Entry) {
    let Some(item) = entry.fields.get("splat_item").and_then(|v| v.as_mapping()) else {
        return;
    };
    let identity = ["weapon_category", "category", "rarity"]
        .into_iter()
        .filter_map(|key| item.get(key).map(display_value))
        .collect::<Vec<_>>()
        .join(" · ")
        .replace('_', " ");
    if !identity.is_empty() {
        style::identity(ui, &identity);
    }
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
    style::facts(ui, &facts);
    for (key, value) in item {
        let Some(key) = key.as_str() else { continue };
        if key == "armor"
            && let Some(armor) = value.as_mapping()
        {
            style::section_heading(ui, "Armor");
            for (label, value) in armor {
                if label.as_str() != Some("base_ac") {
                    style::detail(ui, &display_value(label), &display_value(value));
                }
            }
        } else if ![
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
                egui::CollapsingHeader::new(
                    egui::RichText::new(style::human_label(key)).color(style::ACCENT),
                )
                .id_salt(("item_summary", key))
                .show(ui, |ui| structured_value(ui, value));
                continue;
            }
            style::detail(
                ui,
                &key.trim_end_matches("_text").replace('_', " "),
                &display_value(value),
            );
        }
    }
}

fn structured_value(ui: &mut egui::Ui, value: &serde_yaml_ng::Value) {
    if let Some(fields) = value.as_mapping() {
        for (key, value) in fields {
            let key = display_value(key);
            if value.is_mapping() || value.is_sequence() {
                style::section_heading(ui, &style::human_label(&key));
                structured_value(ui, value);
            } else {
                style::detail(ui, &key, &display_value(value));
            }
        }
    } else if let Some(values) = value.as_sequence() {
        for value in values {
            structured_value(ui, value);
        }
    } else {
        ui.label(display_value(value));
    }
}

fn preparation_summary(ui: &mut egui::Ui, entry: &Entry, has_source: bool) {
    if !has_source && let Some(spell) = entry.fields.get("splat_spell") {
        let facts = [
            "level",
            "school",
            "casting_time",
            "range",
            "duration",
            "concentration",
            "ritual",
        ]
        .into_iter()
        .filter_map(|key| Some((style::human_label(key), display_value(spell.get(key)?))))
        .collect::<Vec<_>>();
        style::facts(ui, &facts);
        if let Some(components) = spell.get("components") {
            style::detail(ui, "Components", &display_value(components));
        }
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
        style::facts(ui, &facts);
        if let Some(mode) = encounter.get("completion").and_then(|v| v.get("mode")) {
            style::detail(ui, "Completion", &display_value(mode));
        }
    }
}

/// Generated item source tables contain JSON. Hide it only if it exactly
/// matches the item fields displayed above; unmatched source always stays.
fn represented_item_source(entry: &Entry, source: &str) -> bool {
    let Some((caption, json)) = source.trim().split_once("\n\n") else {
        return false;
    };
    caption.starts_with("Source table, SRD 5.2.1 page ")
        && serde_yaml_ng::from_str::<serde_yaml_ng::Value>(json)
            .ok()
            .as_ref()
            .is_some_and(|value| Some(value) == entry.fields.get("splat_item"))
}

pub(super) fn show(
    ui: &mut egui::Ui,
    entry: Option<&Entry>,
    sections: &[ReadingSection<'_>],
    builtin: bool,
) {
    ui.add_space(12.0);
    if let Some(entry) = entry {
        item_summary(ui, entry);
        preparation_summary(ui, entry, sections.iter().any(|s| s.title == "Source text"));
    }
    for section in sections {
        if section.body.contains("<!-- splat:") {
            continue;
        } else {
            if section.title == "Source text"
                && let Some(entry) = entry.filter(|entry| super::class::supports(entry))
                && super::class::show_source(ui, section.body, &entry.name)
            {
                continue;
            }
            if section.title == "Source text"
                && entry.is_some_and(|e| represented_item_source(e, section.body))
            {
                continue;
            }
            if section.title == "Source text" && builtin {
                let (mut facts, prose) = source_facts(section.body);
                if let Some(index) = facts.iter().position(|(label, _)| label == "Identity") {
                    let identity = facts.remove(index).1;
                    if !entry.is_some_and(|e| e.fields.contains_key("splat_item")) {
                        style::identity(ui, &identity);
                    } else {
                        // Preserve the source qualifier even when item metadata differs.
                        style::detail(ui, "Source classification", &identity);
                    }
                }
                let (cards, details): (Vec<_>, Vec<_>) =
                    facts.into_iter().partition(|(label, value)| {
                        !matches!(
                            label.as_str(),
                            "Components" | "Equipment" | "Prerequisites" | "Prerequisite"
                        ) && value.len() < 70
                    });
                style::facts(ui, &cards);
                for (label, value) in details {
                    style::detail(ui, &label, &value);
                }
                style::section_heading(ui, "Overview");
                style::source_prose(ui, &presentation::overview_markdown(&prose));
            } else {
                style::section_heading(ui, &section.title);
                style::markdown(ui, section.body, false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
