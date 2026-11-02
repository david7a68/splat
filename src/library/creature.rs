//! A reading view of creature values, with the original rules prose retained.
use super::presentation;
use crate::design::{
    self, Type, colors,
    reader::{self as reader_style, heavy_rule, section_heading},
};
use eframe::egui;
use pulldown_cmark::{Event, Options, Parser, Tag};
use splat::content::{Entry, Reference};

const ABILITIES: [&str; 6] = [
    "strength",
    "dexterity",
    "constitution",
    "intelligence",
    "wisdom",
    "charisma",
];
const DETAILS: [(&str, &str); 9] = [
    ("movement_text", "Speed"),
    ("skills_text", "Skills"),
    ("resistances_text", "Resistances"),
    ("immunities_text", "Immunities"),
    ("vulnerabilities_text", "Vulnerabilities"),
    ("gear_text", "Gear"),
    ("senses_text", "Senses"),
    ("languages_text", "Languages"),
    ("cr_text", "Challenge"),
];

fn field<'a>(entry: &'a Entry, key: &str) -> Option<&'a str> {
    entry.fields.get("splat_creature")?.get(key)?.as_str()
}
fn stat(entry: &Entry, key: &str) -> Option<i64> {
    entry
        .fields
        .get("splat_stats")?
        .get(key)?
        .get("value")?
        .as_i64()
}
fn value(entry: &Entry, key: &str) -> String {
    stat(entry, key).map_or_else(|| "—".into(), |v| v.to_string())
}

fn size(entry: &Entry) -> Option<&str> {
    field(entry, "size_options_text").or_else(|| field(entry, "size"))
}

pub(super) fn supports(entry: &Entry, sections: &[presentation::ReadingSection<'_>]) -> bool {
    has_stats(entry)
        && sections
            .iter()
            .any(|section| section.title == "Source text")
}

/// A creature with stats, from the book or written by hand, has a stat block at its head.
pub(super) fn has_stats(entry: &Entry) -> bool {
    entry.kind == "creature" && entry.fields.contains_key("splat_stats")
}

/// Remove only source rows represented by the styled values. Unknown prose stays.
fn rules_prose(entry: &Entry, source: &str) -> String {
    let identity = size(entry)
        .zip(field(entry, "type"))
        .map(|(size, kind)| format!("{size} {kind}").to_lowercase());
    let mut in_rules = false;
    let source = source
        .lines()
        .filter(|raw| {
            let line = raw.trim().trim_end_matches('\\').trim();
            in_rules |= line.starts_with('#');
            if in_rules {
                return true;
            }
            if identity
                .as_ref()
                .is_some_and(|prefix| line.to_lowercase().starts_with(prefix))
            {
                return false;
            }
            if line.starts_with("AC ")
                && stat(entry, "armor_class").is_some()
                && (!line.contains("Initiative") || stat(entry, "initiative").is_some())
            {
                return false;
            }
            if line.starts_with("HP ") && field(entry, "hit_points_text").is_some() {
                return false;
            }
            if line.starts_with("Passive Perception")
                && field(entry, "senses_text").is_some_and(|senses| senses.contains(line))
            {
                return false;
            }
            if line == "MOD SAVE MOD SAVE MOD SAVE"
                && ABILITIES.iter().all(|a| stat(entry, a).is_some())
            {
                return false;
            }
            let ability_row = match line.split_whitespace().next() {
                Some("Str") => &ABILITIES[..3],
                Some("Int") => &ABILITIES[3..],
                _ => &[],
            };
            if !ability_row.is_empty()
                && ability_row
                    .iter()
                    .all(|ability| stat(entry, ability).is_some())
                && line.split_whitespace().count() >= 9
            {
                return false;
            }
            for (key, label) in DETAILS {
                let label = if key == "cr_text" { "CR" } else { label };
                if line.starts_with(&format!("{label} ")) && field(entry, key).is_some() {
                    return false;
                }
            }
            true
        })
        .collect::<Vec<_>>()
        .join("\n");
    presentation::overview_markdown(&source)
}

/// Keep authored prose around data blocks, without their generated headings.
fn authored_prose(body: &str) -> String {
    let mut depth = 0usize;
    let mut data = false;
    let mut output = String::new();
    for (event, range) in Parser::new_ext(body, Options::ENABLE_TABLES).into_offset_iter() {
        match event {
            Event::Html(text) if text.trim().starts_with("<!-- splat:") => data = true,
            Event::Start(tag) => {
                if depth == 0 && !matches!(tag, Tag::Heading { .. } | Tag::HtmlBlock) {
                    if data && matches!(tag, Tag::Table(_) | Tag::CodeBlock(_)) {
                        data = false;
                    } else if !matches!(tag, Tag::Paragraph)
                        || !block_anchor(body[range.clone()].trim())
                    {
                        output.push_str(&body[range]);
                        output.push_str("\n\n");
                    }
                }
                depth += 1;
            }
            Event::End(_) => depth = depth.saturating_sub(1),
            _ => {}
        }
    }
    output
}

fn block_anchor(text: &str) -> bool {
    text.strip_prefix("^splat-").is_some_and(|id| {
        !id.is_empty()
            && id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
    })
}

pub(super) fn spell_references(entry: &Entry) -> Vec<Reference> {
    let mut value = entry.fields.get("splat_rule");
    for key in ["parameters", "features", "spellcasting", "spells"] {
        value = value.and_then(|v| v.get(key));
    }
    value
        .and_then(|v| v.as_sequence())
        .into_iter()
        .flatten()
        .filter_map(|row| {
            let spell = row.get("spell")?;
            if let Some(id) = spell.get("id").and_then(|v| v.as_str()) {
                id.parse().ok().map(Reference::Id)
            } else {
                spell
                    .get("key")
                    .and_then(|v| v.as_str())
                    .map(|key| Reference::Key(key.into()))
            }
        })
        .collect()
}

fn prose(ui: &mut egui::Ui, text: &str, spell_links: &mut impl FnMut(&mut egui::Ui, &str)) {
    for paragraph in text.split("\n\n").map(str::trim).filter(|s| !s.is_empty()) {
        reader_style::source_prose(ui, paragraph);
        if paragraph.starts_with("Spellcasting.") {
            spell_links(ui, paragraph);
        }
    }
}

/// One ability row of the stat block's attribute table.
fn ability_row(ui: &mut egui::Ui, entry: &Entry, ability: &str) {
    let c = colors();
    let [label_width, score_width, modifier_width, save_width] = ABILITY_COLUMNS;
    let modifier = stat(entry, ability).map(|score| (score - 10).div_euclid(2));
    // An absent override does not establish proficiency or a save total.
    let save = stat(entry, &format!("save_{ability}"));
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = design::XS;
        cell(
            ui,
            label_width,
            &ability[..3].to_uppercase(),
            Type::StatLabel,
            c.text,
        );
        cell(ui, score_width, &value(entry, ability), Type::Stat, c.text);
        cell(
            ui,
            modifier_width,
            &modifier.map_or_else(|| "—".into(), design::signed),
            Type::Stat,
            c.muted,
        );
        cell(
            ui,
            save_width,
            &save.map_or_else(|| "—".into(), design::signed),
            Type::Stat,
            c.muted,
        );
    });
}

/// Label, score, modifier and saving throw.
const ABILITY_COLUMNS: [f32; 4] = [30.0, 24.0, 26.0, 38.0];

/// The heads of the attribute table's last two columns, once for each side of the table.
fn ability_head(ui: &mut egui::Ui) {
    let c = colors();
    let [label_width, score_width, modifier_width, save_width] = ABILITY_COLUMNS;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = design::XS;
        ui.allocate_exact_size(
            egui::vec2(label_width + design::XS + score_width, Type::Label.line()),
            egui::Sense::hover(),
        );
        for (width, word) in [(modifier_width, "MOD"), (save_width, "SAVE")] {
            let (rect, _) =
                ui.allocate_exact_size(egui::vec2(width, Type::Label.line()), egui::Sense::hover());
            ui.painter().text(
                rect.right_center(),
                egui::Align2::RIGHT_CENTER,
                word,
                Type::Label.font(ui),
                c.muted,
            );
        }
    });
}

fn cell(ui: &mut egui::Ui, width: f32, text: &str, role: Type, color: egui::Color32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, role.line()), egui::Sense::hover());
    let galley = design::one_line(ui, text, role.font(ui), color, width + design::S);
    // Numbers line up on their right edge, so that a column of them can be read down.
    let numeric = text.starts_with(|c: char| c.is_ascii_digit() || "+\u{2212}\u{2014}".contains(c));
    let left = if numeric {
        rect.right() - galley.size().x
    } else {
        rect.left()
    };
    ui.painter()
        .galley(egui::pos2(left, rect.top()), galley, color);
}

/// Armor class, hit points and initiative, under capital labels. Only what the entry has.
fn vital_rows(entry: &Entry) -> Vec<(&'static str, String)> {
    let hp = field(entry, "hit_points_text")
        .map(str::to_owned)
        .or_else(|| {
            hit_points_line(entry).and_then(|line| {
                line.strip_prefix("HP ")
                    .map(|text| text.trim_end_matches('\\').trim().to_owned())
            })
        })
        .or_else(|| stat(entry, "max_hp").map(|hp| hp.to_string()));
    [
        (
            "ARMOR CLASS",
            stat(entry, "armor_class").map(|ac| ac.to_string()),
        ),
        ("HIT POINTS", hp),
        ("INITIATIVE", stat(entry, "initiative").map(design::signed)),
    ]
    .into_iter()
    .filter_map(|(label, text)| text.map(|text| (label, text)))
    .collect()
}

fn vitals(ui: &mut egui::Ui, entry: &Entry) {
    let c = colors();
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(design::L, 2.0);
        for (label, text) in vital_rows(entry) {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.label(Type::Label.rich(ui, label).color(c.muted));
                ui.label(Type::Prose.rich(ui, text).color(c.text));
            });
        }
    });
}

/// The body line that gives this creature's hit points, such as `HP 22 (4d8 + 4)`. The
/// vitals show it, with the dice, so the page says it once.
fn hit_points_line(entry: &Entry) -> Option<&str> {
    let max = stat(entry, "max_hp")?;
    entry.body.lines().map(str::trim).find(|line| {
        line.strip_prefix("HP ")
            .and_then(|rest| rest.split_whitespace().next())
            .and_then(|number| number.parse::<i64>().ok())
            == Some(max)
    })
}

fn without_line(body: &str, line: Option<&str>) -> String {
    let Some(line) = line else {
        return body.to_owned();
    };
    let mut out = String::with_capacity(body.len());
    for text in body.split_inclusive('\n') {
        if text.trim() != line {
            out.push_str(text);
        }
    }
    out
}

fn detail(entry: &Entry, key: &str) -> Option<String> {
    if let Some(text) = field(entry, key) {
        return Some(text.to_owned());
    }
    if key != "movement_text" {
        return None;
    }
    let stats = entry.fields.get("splat_stats")?.as_mapping()?;
    let movement = stats
        .iter()
        .filter_map(|(key, data)| {
            let key = key.as_str()?;
            if key != "speed" && !key.ends_with("_speed") {
                return None;
            }
            let amount = data.get("value")?.as_i64()?;
            let unit = data.get("unit").and_then(|v| v.as_str()).unwrap_or("");
            Some(format!("{} {amount} {unit}", key.replace('_', " ")))
        })
        .collect::<Vec<_>>();
    Some(movement.join(", "))
}

/// Identity, vitals, abilities and details: what the entry has, and no empty cells.
pub(super) fn stat_head(ui: &mut egui::Ui, entry: &Entry) {
    ui.add_space(design::S);
    let identity = [size(entry), field(entry, "type"), field(entry, "alignment")]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" · ");
    reader_style::identity(ui, &reader_style::human_label(&identity));
    heavy_rule(ui);
    ui.add_space(design::S);
    vitals(ui, entry);
    ui.add_space(design::S);
    // The rulebook's table: physical abilities beside mental ones.
    let two_columns = ui.available_width() >= 300.0;
    let any_ability = ABILITIES
        .iter()
        .any(|ability| stat(entry, ability).is_some());
    if any_ability {
        design::rule(ui);
        ui.add_space(design::XS);
    }
    if !any_ability {
        // An authored creature can have no scores. There is nothing to tabulate.
    } else if two_columns {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = design::XL;
            for column in ABILITIES.chunks(3) {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 1.0;
                    ability_head(ui);
                    for ability in column {
                        ability_row(ui, entry, ability);
                    }
                });
            }
        });
    } else {
        ui.spacing_mut().item_spacing.y = 1.0;
        ability_head(ui);
        for ability in ABILITIES {
            ability_row(ui, entry, ability);
        }
    }
    if any_ability {
        ui.add_space(design::XS);
        design::rule(ui);
        ui.add_space(design::XS);
    }
    ui.spacing_mut().item_spacing.y = 2.0;
    for (key, label) in DETAILS {
        if let Some(text) = detail(entry, key).filter(|text| !text.is_empty()) {
            reader_style::detail(ui, label, &text);
        }
    }
    ui.add_space(design::XS);
    heavy_rule(ui);
    ui.add_space(design::S);
    ui.spacing_mut().item_spacing.y = design::XS;
}

pub(super) fn show(
    ui: &mut egui::Ui,
    entry: &Entry,
    sections: &[presentation::ReadingSection<'_>],
    mut spell_links: impl FnMut(&mut egui::Ui, &str),
) {
    stat_head(ui, entry);
    for section in sections {
        if section.title == "Source text" {
            let body = without_line(section.body, hit_points_line(entry));
            prose(ui, &rules_prose(entry, &body), &mut spell_links);
        } else {
            let body = if section.body.contains("<!-- splat:") {
                authored_prose(section.body)
            } else {
                without_line(section.body, hit_points_line(entry))
            };
            if body.trim().is_empty() {
                continue;
            }
            if section.title != "Overview" {
                section_heading(ui, &section.title);
            }
            reader_style::source_prose(ui, &body);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hand_written_creature_reads_as_a_stat_block_with_only_what_it_has() {
        let text = "---\nsplat_schema: 1\nsplat_id: 8f4a2f8e-41f0-4c4a-9d4c-0a7a1a1b2c3d\nsplat_kind: creature\nsplat_name: Varra\nsplat_creature: {size: medium}\nsplat_stats:\n  max_hp: {value: 52}\n  armor_class: {value: 16}\n---\n\n# Varra\n\n## Actions\n\nLongsword. Melee Attack Roll: +5.\n";
        let entry = Entry::parse("Notes/Varra.md", text).unwrap();
        assert!(has_stats(&entry));
        assert_eq!(
            vital_rows(&entry),
            vec![
                ("ARMOR CLASS", "16".to_string()),
                ("HIT POINTS", "52".to_string())
            ]
        );
    }

    #[test]
    fn custom_prose_around_structured_data_is_preserved() {
        let body = "Before.\n\n<!-- splat:values path=splat_creature -->\n\n| Field | Value |\n| --- | --- |\n| size | medium |\n\n### Notes\n\nAfter.\n\n- Keep this list.\n";
        let result = authored_prose(body);
        assert!(result.contains("Before."));
        assert!(result.contains("After."));
        assert!(result.contains("- Keep this list."));
        assert!(!result.contains("medium"));
    }

    #[test]
    fn internal_anchors_are_hidden_without_removing_authored_text() {
        let body = "###### Detect magic\n\n^splat-68a383d1-detect_magic\n\n<!-- splat:yaml path=splat_rule/cast_level -->\n\n```yaml\nspell_minimum\n```\n\nUse this spell before entering.\n\n^splat-example is explained here.\n\n`^splat-example`\n";
        let result = authored_prose(body);
        assert!(!result.contains("^splat-68a383d1-detect_magic"));
        assert!(!result.contains("spell_minimum"));
        assert!(result.contains("Use this spell before entering."));
        assert!(result.contains("^splat-example is explained here."));
        assert!(result.contains("`^splat-example`"));
    }

    #[test]
    fn incomplete_stats_keep_unrepresented_source_details() {
        let path = "definitions/creatures/vampire_familiar.md";
        let text = splat::content::definitions::source_file(path).unwrap();
        let mut entry = Entry::parse(path, text).unwrap();
        let source = presentation::reading_sections(&entry.body, &entry.name)
            .into_iter()
            .find(|s| s.title == "Source text")
            .unwrap()
            .body
            .to_owned();
        entry
            .fields
            .get_mut("splat_creature")
            .unwrap()
            .as_mapping_mut()
            .unwrap()
            .remove("immunities_text");
        let prose = rules_prose(&entry, &source);
        assert!(prose.contains("Immunities Charmed (except from its vampire master)"));
        assert!(prose.contains("Vampiric Connection."));
        assert!(!prose.contains("HP 65"));
        entry.body = "## Description\nA custom creature with structured actions.".into();
        assert!(
            !supports(
                &entry,
                &presentation::reading_sections(&entry.body, &entry.name)
            ),
            "Keep the existing complete view when no source rules prose exists"
        );
    }
}
