//! Class reading layout. Keep source wording and raw definitions available.
use super::{presentation, view::Links};
use crate::design::{self, Type, colors, reader as style};
use eframe::egui;
use splat::content::{Entry, Reference};

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
    style::section_heading(ui, "Core traits");
    ui.add_space(6.0);
    for (label, value) in rows {
        let value = presentation::overview_markdown(&value).trim().to_owned();
        if label == "Starting Equipment" {
            ui.add_space(10.0);
            style::section_heading(ui, "Starting equipment");
            // Preserve the choice instruction and each option, including classes
            // with three alternatives. Do not split on punctuation inside items.
            let value = value
                .replace("(A)", "\n\n- **A:**")
                .replace("; or (B)", "\n\n- **B:**")
                .replace("; (B)", "\n\n- **B:**")
                .replace("; or (C)", "\n\n- **C:**");
            style::markdown(ui, &value);
        } else {
            let label = if label == "Saving Throw" {
                "Saving throw proficiencies"
            } else {
                label
            };
            style::markdown(ui, &format!("**{label}:** {value}"));
        }
    }
    if !remainder.is_empty() {
        design::section(ui, "Additional source text", |ui| {
            // Keep printed progression rows separate instead of joining a table
            // into prose; the named feature list is shown below.
            style::markdown(ui, &remainder.lines().collect::<Vec<_>>().join("  \n"));
        });
    }
    true
}

struct Feature {
    level: Option<u32>,
    name: String,
    target: Option<usize>,
}

fn features(entry: &Entry, links: &dyn Links) -> Vec<Feature> {
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
        let target = key
            .map(|key| Reference::Key(key.to_owned()))
            .or(id.map(Reference::Id))
            .and_then(|reference| links.find(&reference));
        let name = target.as_ref().map_or_else(
            || {
                key.or_else(|| member.get("id").and_then(|value| value.as_str()))
                    .unwrap_or("Unresolved feature")
                    .to_owned()
            },
            |(_, name)| name.clone(),
        );
        let (level, name) = name
            .strip_prefix("Level ")
            .and_then(|name| name.split_once(": "))
            .and_then(|(level, name)| level.parse().ok().map(|level| (Some(level), name)))
            .unwrap_or((None, name.as_str()));
        features.push(Feature {
            level,
            name: name.to_owned(),
            target: target.map(|(index, _)| index),
        });
    }
    features.sort_by(|a, b| {
        a.level
            .unwrap_or(u32::MAX)
            .cmp(&b.level.unwrap_or(u32::MAX))
            .then(a.name.cmp(&b.name))
    });
    features
}

pub(super) fn show_features(ui: &mut egui::Ui, entry: &Entry, links: &dyn Links) -> Option<usize> {
    let features = features(entry, links);
    if features.is_empty() {
        return None;
    }
    ui.add_space(design::M);
    style::section_heading(ui, "Class features");
    let mut selected = None;
    for feature in features {
        ui.horizontal_top(|ui| {
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(60.0, Type::Stat.line().max(24.0)),
                egui::Sense::hover(),
            );
            let level = feature
                .level
                .map_or("—".into(), |level| format!("Level {level}"));
            let galley = design::one_line(ui, &level, Type::Stat.font(ui), colors().muted, 60.0);
            ui.painter().galley(
                egui::pos2(rect.left(), rect.center().y - galley.size().y / 2.0),
                galley,
                colors().muted,
            );
            if let Some(target) = feature.target {
                if design::link(ui, &feature.name).clicked() {
                    selected = Some(target);
                }
            } else {
                ui.add(
                    egui::Label::new(
                        Type::Stat
                            .rich(ui, format!("{} (unavailable)", feature.name))
                            .color(colors().muted),
                    )
                    .wrap(),
                );
            }
        });
    }
    selected
}
