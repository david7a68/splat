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
pub(super) fn show_source(ui: &mut egui::Ui, source: &str, name: &str) -> bool {
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
    if lines.next() != Some(format!("Core {name} Traits").as_str()) {
        return false;
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
            return false;
        }
    }
    if rows.is_empty() {
        return false;
    }
    ui.heading("Core traits");
    ui.add_space(6.0);
    for (label, value) in rows {
        let value = presentation::overview_markdown(&value).trim().to_owned();
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
            crate::markdown_view::show(ui, &format!("**{label}:** {value}"));
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
