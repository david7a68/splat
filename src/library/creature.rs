//! A reading view of creature values, with the original rules prose retained.
use super::presentation;
use eframe::egui::{self, Color32, RichText};
use pulldown_cmark::{Event, Options, Parser, Tag};
use splat::content::{Entry, Reference};

const ACCENT: Color32 = Color32::from_rgb(100, 199, 194);
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
    entry.kind == "creature"
        && entry.fields.contains_key("splat_stats")
        && sections
            .iter()
            .any(|section| section.title == "Source text")
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

fn prose(ui: &mut egui::Ui, text: &str, spell_links: &mut impl FnMut(&mut egui::Ui)) {
    for paragraph in text.split("\n\n").map(str::trim).filter(|s| !s.is_empty()) {
        if let Some(title) = paragraph.strip_prefix("### ") {
            section_heading(ui, title);
        } else {
            let markdown = if let Some((name, rest)) = paragraph.split_once(". ") {
                if name.len() < 65 && !name.contains('\n') && !name.contains('*') {
                    format!("**{name}.** {rest}")
                } else {
                    paragraph.into()
                }
            } else {
                paragraph.into()
            };
            crate::markdown_view::show(ui, &markdown);
            if paragraph.starts_with("Spellcasting.") {
                spell_links(ui);
            }
        }
    }
}

fn section_heading(ui: &mut egui::Ui, title: &str) {
    ui.add_space(10.0);
    ui.label(RichText::new(title).size(20.0).color(ACCENT).strong());
    ui.separator();
}

fn card(ui: &mut egui::Ui, margin: i8, contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(ui.visuals().faint_bg_color)
        .corner_radius(6.0)
        .inner_margin(margin)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            contents(ui);
        });
}

fn ability_card(ui: &mut egui::Ui, entry: &Entry, ability: &str) {
    card(ui, 8, |ui| {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new(ability[..3].to_uppercase())
                    .color(ACCENT)
                    .strong(),
            );
            ui.label(RichText::new(value(entry, ability)).size(22.0).strong());
            let modifier = stat(entry, ability).map(|score| (score - 10).div_euclid(2));
            ui.label(modifier.map_or_else(|| "Mod —".into(), |v| format!("Mod {v:+}")));
            // An absent override does not establish proficiency or a save total.
            let save = stat(entry, &format!("save_{ability}"));
            ui.label(
                RichText::new(save.map_or_else(|| "Save —".into(), |v| format!("Save {v:+}")))
                    .small(),
            );
        });
    });
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

pub(super) fn show(
    ui: &mut egui::Ui,
    entry: &Entry,
    sections: &[presentation::ReadingSection<'_>],
    mut spell_links: impl FnMut(&mut egui::Ui),
) {
    ui.add_space(12.0);
    let identity = [size(entry), field(entry, "type"), field(entry, "alignment")]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" · ");
    ui.label(
        RichText::new(identity)
            .italics()
            .color(ui.visuals().weak_text_color()),
    );
    ui.add_space(8.0);
    let hp = field(entry, "hit_points_text")
        .map(str::to_owned)
        .unwrap_or_else(|| value(entry, "max_hp"));
    let initiative = stat(entry, "initiative").map_or_else(|| "—".into(), |v| format!("{v:+}"));
    ui.columns(3, |columns| {
        for (column, (label, text)) in columns.iter_mut().zip([
            ("ARMOR CLASS", value(entry, "armor_class")),
            ("HIT POINTS", hp),
            ("INITIATIVE", initiative),
        ]) {
            card(column, 10, |ui| {
                ui.label(RichText::new(label).size(11.0).color(ACCENT).strong());
                ui.label(RichText::new(text).size(20.0).strong());
            });
        }
    });
    ui.add_space(10.0);
    let per_row = if ui.available_width() < 480.0 { 3 } else { 6 };
    for abilities in ABILITIES.chunks(per_row) {
        ui.columns(abilities.len(), |columns| {
            for (column, ability) in columns.iter_mut().zip(abilities) {
                ability_card(column, entry, ability);
            }
        });
        ui.add_space(6.0);
    }
    ui.add_space(4.0);
    for (key, label) in DETAILS {
        if let Some(text) = detail(entry, key).filter(|text| !text.is_empty()) {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(label).strong().color(ACCENT));
                ui.label(text);
            });
        }
    }
    ui.add_space(8.0);
    for section in sections {
        if section.title == "Source text" {
            prose(ui, &rules_prose(entry, section.body), &mut spell_links);
        } else {
            let body = if section.body.contains("<!-- splat:") {
                authored_prose(section.body)
            } else {
                section.body.to_owned()
            };
            if body.trim().is_empty() {
                continue;
            }
            if section.title != "Overview" {
                section_heading(ui, &section.title);
            }
            crate::markdown_view::show(ui, &body);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
