//! Class reading layout. Keep source wording and raw definitions available.
use super::{Note, presentation};
use eframe::egui;
use splat::content::Entry;

pub(super) fn supports(entry: &Entry) -> bool {
    entry
        .fields
        .get("splat_rule_group")
        .and_then(|group| group.get("category"))
        .and_then(|value| value.as_str())
        == Some("class")
}

/// Only recognize the imported core-traits block. Authored text and any trailing
/// source material stay in the ordinary Markdown reader.
fn traits(source: &str, name: &str) -> Option<(Vec<(&'static str, String)>, String)> {
    const LABELS: &[&str] = &[
        "Primary Ability",
        "Hit Point Die",
        "Saving Throw",
        "Skill Proficiencies",
        "Weapon Proficiencies",
        "Tool Proficiencies",
        "Armor Training",
        "Starting Equipment",
    ];
    let mut lines = source
        .lines()
        .map(|line| line.trim().trim_end_matches('\\'))
        .filter(|line| !line.is_empty() && !line.starts_with("<!-- SRD page "));
    if lines.next()? != format!("Core {name} Traits") {
        return None;
    }
    let mut rows: Vec<(&'static str, String)> = Vec::new();
    let mut remainder = String::new();
    let mut trailing = false;
    for line in lines {
        trailing |= line == format!("{name} Features") || line.starts_with('#');
        if trailing {
            remainder.push_str(line);
            remainder.push('\n');
        } else if let Some((label, value)) = LABELS.iter().find_map(|label| {
            line.strip_prefix(&format!("{label} "))
                .map(|value| (*label, value))
        }) {
            rows.push((label, value.to_owned()));
        } else if let Some((label, value)) = rows.last_mut() {
            if *label == "Saving Throw" && line == "Proficiencies" {
                continue;
            }
            value.push('\n');
            value.push_str(line);
        } else {
            return None;
        }
    }
    if rows.is_empty() {
        return None;
    }
    for (_, value) in &mut rows {
        *value = presentation::overview_markdown(value).trim().to_owned();
    }
    Some((rows, remainder))
}

pub(super) fn show_source(ui: &mut egui::Ui, source: &str, name: &str) -> bool {
    let Some((rows, remainder)) = traits(source, name) else {
        return false;
    };
    ui.heading("Core traits");
    ui.add_space(6.0);
    for (label, value) in rows {
        if label == "Starting Equipment" {
            ui.add_space(10.0);
            ui.heading("Starting equipment");
            // Preserve the choice instruction and each option, including classes
            // with three alternatives. Do not split on punctuation inside items.
            let value = value
                .replace("(A)", "\n\n- **A:**")
                .replace("; or (B)", "\n\n- **B:**")
                .replace("; (B)", "\n\n- **B:**")
                .replace("; or (C)", "\n\n- **C:**");
            crate::markdown_view::show(ui, &value);
        } else {
            let label = if label == "Saving Throw" {
                "Saving throw proficiencies"
            } else {
                label
            };
            // Stack on narrow windows rather than forcing two minimum widths.
            if ui.available_width() < 460.0 {
                ui.strong(label);
                ui.add(egui::Label::new(value).wrap());
            } else {
                ui.horizontal_top(|ui| {
                    ui.add_sized(
                        [175.0, 0.0],
                        egui::Label::new(egui::RichText::new(label).strong()).wrap(),
                    );
                    ui.add(egui::Label::new(value).wrap());
                });
            }
            ui.add_space(5.0);
        }
    }
    if !remainder.is_empty() {
        ui.collapsing("Additional source text", |ui| {
            // Keep printed progression rows separate instead of joining a table
            // into prose; the named feature list is shown below.
            crate::markdown_view::show(ui, &remainder.lines().collect::<Vec<_>>().join("  \n"));
        });
    }
    true
}

struct Feature<'a> {
    level: Option<u32>,
    name: &'a str,
    target: Option<usize>,
}

fn features<'a>(entry: &'a Entry, notes: &'a [Note]) -> Vec<Feature<'a>> {
    let mut features = Vec::new();
    for member in entry
        .fields
        .get("splat_rule_group")
        .and_then(|group| group.get("members"))
        .and_then(|members| members.as_sequence())
        .into_iter()
        .flatten()
    {
        let key = member.get("key").and_then(|v| v.as_str());
        let id = member.get("id").and_then(|v| v.as_str());
        let id = id.and_then(|id| uuid::Uuid::parse_str(id).ok());
        let mut targets = notes.iter().enumerate().filter_map(|(index, note)| {
            note.entry
                .as_ref()
                .filter(|entry| {
                    key.is_some_and(|key| entry.key.as_deref() == Some(key)) || id == Some(entry.id)
                })
                .map(|_| index)
        });
        let target = targets.next().filter(|_| targets.next().is_none());
        let name = target.map(|i| notes[i].name()).unwrap_or_else(|| {
            key.or_else(|| member.get("id").and_then(|value| value.as_str()))
                .unwrap_or("Unresolved feature")
        });
        let (level, name) = name
            .strip_prefix("Level ")
            .and_then(|name| name.split_once(": "))
            .and_then(|(level, name)| level.parse().ok().map(|level| (Some(level), name)))
            .unwrap_or((None, name));
        features.push(Feature {
            level,
            name,
            target,
        });
    }
    features.sort_by(|a, b| {
        a.level
            .unwrap_or(u32::MAX)
            .cmp(&b.level.unwrap_or(u32::MAX))
            .then(a.name.cmp(b.name))
    });
    features
}

pub(super) fn show_features(
    ui: &mut egui::Ui,
    entry: &Entry,
    notes: &[Note],
    dirty: bool,
) -> Option<usize> {
    let features = features(entry, notes);
    if features.is_empty() {
        return None;
    }
    ui.add_space(14.0);
    ui.heading("Class features");
    ui.add_space(6.0);
    let mut selected = None;
    for feature in features {
        ui.horizontal_top(|ui| {
            ui.add_sized(
                [60.0, 0.0],
                egui::Label::new(
                    feature
                        .level
                        .map_or("—".into(), |level| format!("Level {level}")),
                ),
            );
            if let Some(target) = feature.target {
                let response = ui.add_enabled(!dirty, egui::Link::new(feature.name));
                if response
                    .on_disabled_hover_text("Save or discard changes before opening a feature.")
                    .clicked()
                {
                    selected = Some(target);
                }
            } else {
                ui.add(egui::Label::new(format!("{} (unavailable)", feature.name)).wrap());
            }
        });
    }
    selected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_imported_classes_keep_trait_boundaries_and_equipment() {
        let loaded = super::super::load(None).unwrap();
        let mut count = 0;
        for entry in loaded
            .notes
            .iter()
            .filter_map(|note| note.entry.as_ref())
            .filter(|entry| supports(entry))
        {
            let sections = presentation::reading_sections(&entry.body, &entry.name);
            let source = sections
                .iter()
                .find(|section| section.title == "Source text")
                .unwrap();
            let (rows, remainder) = traits(source.body, &entry.name).unwrap();
            assert_eq!(rows[0].0, "Primary Ability");
            assert_eq!(rows[1].0, "Hit Point Die");
            assert_eq!(rows[2].0, "Saving Throw");
            assert!(!rows[2].1.contains("Proficiencies"));
            let equipment = &rows.last().unwrap().1;
            assert!(
                equipment.contains("(A)") && equipment.contains("(B)"),
                "{equipment}"
            );
            assert!(!equipment.contains("Features"));
            if entry.name == "Fighter" {
                assert!(equipment.contains("(C) 155 GP"), "{equipment}");
                assert!(remainder.contains("20 +6 Three Extra Attacks"));
            }
            if entry.name == "Barbarian" {
                assert_eq!(
                    rows[3].1,
                    "Choose 2: Animal Handling, Athletics, Intimidation, Nature, Perception, or Survival"
                );
                assert_eq!(rows[5].1, "Light and Medium armor and Shields");
                let features = features(entry, &loaded.notes);
                assert_eq!(features.len(), 20);
                assert_eq!((features[0].level, features[0].name), (Some(1), "Rage"));
                assert_eq!(features.last().unwrap().level, Some(20));
                assert!(features.iter().all(|feature| feature.target.is_some()));
                assert!(
                    features
                        .windows(2)
                        .all(|pair| pair[0].level <= pair[1].level)
                );
            }
            count += 1;
        }
        assert_eq!(count, 12);
        assert!(traits("My custom class prose", "Barbarian").is_none());
    }
}
