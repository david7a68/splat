//! Class reading layout. Keep source wording and raw definitions available.
#![warn(deprecated)] // Its parent allows old message functions; this file does not.
use super::{presentation, view::Links};
use crate::design::{self, Type, colors, reader as style};
use eframe::egui;
use splat::content::{Entry, Reference};

fn category(entry: &Entry) -> Option<&str> {
    entry
        .fields
        .get("splat_rule_group")
        .and_then(|group| group.get("category"))
        .and_then(|value| value.as_str())
}

pub(super) fn supports(entry: &Entry) -> bool {
    category(entry) == Some("class")
}

/// A class or a subclass grants features.
pub(super) fn lists_features(entry: &Entry) -> bool {
    matches!(category(entry), Some("class" | "subclass"))
}

/// Only recognize the imported core-traits block. Authored text and any trailing
/// source material stay in the ordinary Markdown reader.
#[allow(
    clippy::disallowed_methods,
    reason = "staged: raw space, choices, search fields or headings that a design part replaces (design::results::STAGED_ALLOWS)"
)]
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
    // The traits are a two-column table: a label and its value. Later tables stay tables.
    let flatten = |line: &str| {
        line.strip_prefix('|').map_or_else(
            || line.to_owned(),
            |row| {
                row.trim_end_matches('|')
                    .split(" | ")
                    .map(str::trim)
                    .filter(|cell| !cell.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            },
        )
    };
    let mut lines = source
        .lines()
        .map(|line| line.trim().trim_end_matches('\\'))
        .filter(|line| !line.is_empty() && !line.starts_with("<!-- SRD page "));
    let first = lines
        .by_ref()
        .find(|line| !line.chars().all(|c| "| -".contains(c)));
    if first.map(|line| line.trim_matches('*')) != Some(format!("Core {name} Traits").as_str()) {
        return false;
    }
    let mut rows: Vec<(&'static str, String)> = Vec::new();
    let mut remainder = String::new();
    let mut trailing = false;
    for raw in lines {
        let flat = flatten(raw.trim_matches('*'));
        let line = flat.as_str();
        trailing |= line == format!("{name} Features") || line.starts_with('#');
        if trailing {
            remainder.push_str(raw);
            remainder.push('\n');
        } else if raw.chars().all(|c| "| -".contains(c)) {
            continue;
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
    design::heading(ui, design::Heading::Section("Core traits"));
    ui.add_space(6.0);
    let mut equipment = None;
    let mut facts = Vec::new();
    for (label, value) in rows {
        let value = presentation::overview_markdown(&value).trim().to_owned();
        if label == "Starting Equipment" {
            equipment = Some(value);
        } else {
            // Short enough to share the row with the value, as a spell's labels do.
            let label = match label {
                "Primary Ability" => "Primary ability",
                "Hit Point Die" => "Hit point die",
                "Saving Throw" => "Saving throws",
                "Skill Proficiencies" => "Skills",
                "Weapon Proficiencies" => "Weapons",
                "Tool Proficiencies" => "Tools",
                "Armor Training" => "Armor training",
                other => other,
            };
            facts.push((label.to_owned(), value));
        }
    }
    style::facts(ui, &facts);
    if let Some(value) = equipment {
        design::heading(ui, design::Heading::Section("Starting equipment"));
        ui.add_space(6.0);
        // Preserve the choice instruction and each option, including classes
        // with three alternatives. Do not split on punctuation inside items.
        let value = value
            .replace("(A)", "\n\n(A)")
            .replace("; or (B)", "\n\n(B)")
            .replace("; (B)", "\n\n(B)")
            .replace("; or (C)", "\n\n(C)");
        let mut options = Vec::new();
        for part in value.split("\n\n").map(str::trim) {
            let option = ["A", "B", "C"]
                .into_iter()
                .find_map(|letter| Some((letter, part.strip_prefix(&format!("({letter})"))?)));
            if let Some((letter, text)) = option {
                options.push((format!("Option {letter}"), text.trim().to_owned()));
            } else if !part.is_empty() {
                style::markdown(ui, part);
            }
        }
        style::facts(ui, &options);
    }
    if !remainder.is_empty() {
        design::heading(ui, design::Heading::Section("Class table"));
        style::markdown(ui, &presentation::overview_markdown(&remainder));
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
    // Some classes list their members, and others grant their features. Both are the list.
    let members = entry
        .fields
        .get("splat_rule_group")
        .and_then(|group| group.get("members"))
        .and_then(|members| members.as_sequence())
        .into_iter()
        .flatten();
    let granted = entry
        .fields
        .get("splat_rule")
        .and_then(|rule| rule.get("parameters"))
        .and_then(|parameters| parameters.get("feature_grants"))
        .and_then(|grants| grants.as_sequence())
        .into_iter()
        .flatten()
        .filter_map(|grant| grant.get("definition"));
    let mut seen = std::collections::BTreeSet::new();
    for member in members.chain(granted) {
        let key = member.get("key").and_then(|v| v.as_str());
        let id = member.get("id").and_then(|v| v.as_str());
        let reference = key.or(id);
        if !seen.insert(reference) {
            continue;
        }
        let target = key
            .map(|key| Reference::Key(key.to_owned()))
            .or(id
                .and_then(|id| uuid::Uuid::parse_str(id).ok())
                .map(Reference::Id))
            .and_then(|reference| links.find(&reference));
        let name = target.as_ref().map_or_else(
            || reference.unwrap_or("Unresolved feature").to_owned(),
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

/// What else a class has, by the keys the definitions use: its progression and spell list.
fn related(entry: &Entry, links: &dyn Links) -> Vec<(usize, String)> {
    let class = entry
        .fields
        .get("splat_rule")
        .and_then(|rule| rule.get("parameters"))
        .and_then(|parameters| parameters.get("class"))
        .and_then(|class| class.as_str());
    let Some(class) = class else {
        return Vec::new();
    };
    [
        format!("dnd2024:tables/{class}_progression"),
        format!("dnd2024:rules/{class}_{class}_spell_list"),
    ]
    .into_iter()
    .filter_map(|key| links.find(&Reference::Key(key)))
    .collect()
}

pub(super) fn show_features(ui: &mut egui::Ui, entry: &Entry, links: &dyn Links) -> Option<usize> {
    let features = features(entry, links);
    let related = related(entry, links);
    let mut selected = None;
    if !related.is_empty() {
        design::heading(ui, design::Heading::Section("Related"));
        for (target, name) in related {
            if design::link(ui, &name).clicked() {
                selected = Some(target);
            }
        }
    }
    if features.is_empty() {
        return selected;
    }
    design::heading(
        ui,
        design::Heading::Section(if supports(entry) {
            "Class features"
        } else {
            "Subclass features"
        }),
    );
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
