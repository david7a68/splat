//! A common reading layout for all non-creature entries.
use super::presentation::{self, ReadingSection};
use crate::design::reader as style;
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

fn item_summary(ui: &mut egui::Ui, entry: &Entry, printed: Option<&str>) {
    let Some(item) = entry.fields.get("splat_item").and_then(|v| v.as_mapping()) else {
        return;
    };
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
            style::detail(
                ui,
                &key.trim_end_matches("_text").replace('_', " "),
                &display_value(value),
            );
        }
    }
}

fn remaining_item_detail(
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

pub(super) fn item_details(ui: &mut egui::Ui, entry: &Entry) {
    if let Some(item) = entry.fields.get("splat_item").and_then(|v| v.as_mapping()) {
        for (key, value) in item {
            let value = remaining_item_detail(entry, key, value);
            if !blank(&value) && (value.is_mapping() || value.is_sequence()) {
                crate::design::quiet_section(ui, &style::human_label(&display_value(key)), |ui| {
                    structured_value(ui, &value);
                });
            }
        }
    }
}

pub(super) fn has_item_details(entry: &Entry) -> bool {
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
fn blank(value: &serde_yaml_ng::Value) -> bool {
    match value {
        serde_yaml_ng::Value::Null => true,
        serde_yaml_ng::Value::String(text) => text.trim().is_empty(),
        serde_yaml_ng::Value::Sequence(values) => values.iter().all(blank),
        serde_yaml_ng::Value::Mapping(values) => values.values().all(blank),
        _ => false,
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

fn spell_facts(spell: &serde_yaml_ng::Value) -> Vec<(String, String)> {
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

fn preparation_summary(ui: &mut egui::Ui, entry: &Entry, has_source: bool) {
    if !has_source && let Some(spell) = entry.fields.get("splat_spell") {
        style::facts(ui, &spell_facts(spell));
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
    has_summary: bool,
    lead_gap: bool,
) {
    let table_stats = entry.is_some_and(|e| {
        e.fields.get("splat_schema").and_then(|v| v.as_u64()) == Some(2)
            && ["spell", "item"].contains(&e.kind.as_str())
    });
    // This reader also receives already-split sections. Restore their headings
    // before filtering, because table ownership is defined by the named section.
    let clean_body = entry.filter(|_| table_stats).map(|entry| {
        let body = sections
            .iter()
            .map(|section| {
                if section.title == "Overview" {
                    section.body.to_owned()
                } else {
                    format!("## {}\n\n{}", section.title, section.body)
                }
            })
            .collect::<Vec<_>>()
            .join("\n\n");
        splat::content::stat_blocks::reading_body(entry, &body)
    });
    let clean_sections;
    let sections = if let Some(body) = &clean_body {
        clean_sections = presentation::reading_sections(body, "");
        clean_sections.as_slice()
    } else {
        sections
    };
    if !has_summary && lead_gap {
        ui.add_space(crate::design::L);
    }
    if let Some(entry) = entry {
        let printed = sections
            .iter()
            .find(|section| section.title == "Source text")
            .filter(|_| builtin && !table_stats)
            .and_then(|section| {
                source_facts(section.body)
                    .0
                    .into_iter()
                    .find(|(label, _)| label == "Identity")
                    .map(|(_, identity)| identity)
            });
        item_summary(ui, entry, printed.as_deref());
        preparation_summary(
            ui,
            entry,
            !table_stats && sections.iter().any(|s| s.title == "Source text"),
        );
    }
    for section in sections
        .iter()
        .filter(|section| presentation::visible_reading_section(section))
    {
        if section.title == "Source text" && has_summary {
            continue;
        }
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
            if section.title == "Source text" && (builtin || table_stats) {
                let (mut facts, prose) = source_facts(section.body);
                if table_stats {
                    facts.clear();
                }
                if let Some(index) = facts.iter().position(|(label, _)| label == "Identity") {
                    let identity = facts.remove(index).1;
                    // An item's is already its identity line, above.
                    if !entry.is_some_and(|e| e.fields.contains_key("splat_item")) {
                        style::identity(ui, &identity);
                    }
                }
                let has_facts = !facts.is_empty();
                if has_facts {
                    ui.scope(|ui| {
                        ui.spacing_mut().item_spacing.y = crate::design::XS / 2.0;
                        for (label, value) in facts {
                            style::detail(ui, &label, &value);
                        }
                    });
                    ui.add_space(crate::design::M);
                }
                style::source_prose(ui, &presentation::overview_markdown(&prose));
            } else {
                // An intro has no printed heading, so the reader adds none.
                if section.title != "Overview" {
                    if crate::markdown_view::wanted_heading(ui.ctx(), &section.title) {
                        ui.scroll_to_cursor(Some(eframe::egui::Align::TOP));
                    }
                    style::section_heading(ui, &section.title);
                }
                // A hand-written stat block's actions read as the book's do.
                if let Some(entry) = entry.filter(|entry| super::creature::has_stats(entry)) {
                    super::creature::rules_text(ui, &entry.name, &entry.body, section.body);
                } else {
                    style::markdown(ui, section.body);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_two_reader_hides_reserved_and_empty_sections_but_keeps_readable_content() {
        for size in [egui::vec2(960.0, 640.0), egui::vec2(1920.0, 1080.0)] {
            let ctx = egui::Context::default();
            crate::design::theme::install(&ctx);
            let body = "## Splat data\n\nStorage only.\n\n## Empty\n\n<!-- hidden -->\n\n## Words\n\nKeep these words.\n\n## Code\n\n`Keep this code`\n";
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        show(
                            ui,
                            None,
                            &presentation::reading_sections(body, ""),
                            false,
                            false,
                            true,
                        );
                    });
                },
            );
            let text: Vec<_> = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                    _ => None,
                })
                .collect();
            for hidden in ["Splat data", "Storage only.", "Empty"] {
                assert!(!text.contains(&hidden), "{text:?}");
            }
            for kept in ["Words", "Keep these words.", "Code", "Keep this code"] {
                assert!(text.contains(&kept), "{text:?}");
            }
        }
    }

    #[test]
    fn converted_spell_and_item_readers_hide_tables_and_machine_lines() {
        for (kind, fields) in [
            (
                "item",
                "splat_item: {category: wondrous_item, rarity: uncommon, equipment: {attunement: required}}",
            ),
            ("spell", "splat_spell: {level: 1}"),
            (
                "spell",
                "splat_spell: {level: 1, casting_time: {amount: null, unit: ''}, material: ''}",
            ),
            (
                "spell",
                "splat_spell: {level: 1, casting_time: Action, range_text: Self, components: [verbal], duration: '1 minute'}",
            ),
            (
                "item",
                "splat_item: {category: wondrous_item, rarity: uncommon}",
            ),
        ] {
            let old = splat::content::formats::convert("Fixture.md", &format!(
                "---\nsplat_schema: 1\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: {kind}\nsplat_name: Rook\n{fields}\n---\n\n# Rook\n\nKeep this effect.\n"
            )).expect("schema-2 fixture");
            let converted = splat::content::formats::convert("Rook.md", &old).unwrap();
            let entry = Entry::parse("Rook.md", &converted).unwrap();
            let ctx = egui::Context::default();
            crate::design::theme::install(&ctx);
            let output = ctx.run(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    show(
                        ui,
                        Some(&entry),
                        &presentation::reading_sections(&entry.body, &entry.name),
                        false,
                        false,
                        true,
                    );
                });
            });
            let text: Vec<_> = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                    _ => None,
                })
                .collect();
            assert!(
                !text
                    .iter()
                    .any(|text| text.contains("%% splat") || *text == "Field"),
                "{text:?}"
            );
            assert!(text.iter().any(|text| text.contains("Keep this effect.")));
            if fields.contains("attunement: required") {
                assert_eq!(
                    text.iter()
                        .filter(|text| **text == "Wondrous Item, Uncommon (Requires Attunement)")
                        .count(),
                    1,
                    "{text:?}"
                );
            }
            if fields.contains("casting_time: Action") {
                for fact in ["Self", "V", "1 minute"] {
                    assert_eq!(
                        text.iter().filter(|text| **text == fact).count(),
                        1,
                        "{text:?}"
                    );
                }
                assert_eq!(
                    text.iter().filter(|text| **text == "Action").count(),
                    1,
                    "{text:?}"
                );
            }
        }
    }

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
