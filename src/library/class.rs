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

pub fn supports(entry: &Entry) -> bool {
    category(entry) == Some("class")
}

/// A class or a subclass grants features.
pub fn lists_features(entry: &Entry) -> bool {
    matches!(category(entry), Some("class" | "subclass"))
}

/// A class's imported core traits, read from its source text (`core_traits`): the facts, the
/// starting equipment's instruction and options, and the source after them (a heading and its
/// Markdown).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CoreTraits {
    pub facts: Vec<(String, String)>,
    /// `Starting equipment`: its words that are not an option, then its options.
    pub equipment: Option<Equipment>,
    /// The source after the traits: its heading and its Markdown.
    pub rest: Option<(String, String)>,
}

/// A class's starting equipment: its words that are not an option, then its options.
pub type Equipment = (Vec<String>, Vec<(String, String)>);

/// Only recognize the imported core-traits block. Authored text and any trailing
/// source material stay in the ordinary Markdown reader.
pub fn core_traits(source: &str, name: &str) -> Option<CoreTraits> {
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
        return None;
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
            return None;
        }
    }
    if rows.is_empty() {
        return None;
    }
    let mut traits = CoreTraits::default();
    for (label, value) in rows {
        let value = presentation::overview_markdown(&value).trim().to_owned();
        if label == "Starting Equipment" {
            // Preserve the choice instruction and each option, including classes
            // with three alternatives. Do not split on punctuation inside items.
            let value = value
                .replace("(A)", "\n\n(A)")
                .replace("; or (B)", "\n\n(B)")
                .replace("; (B)", "\n\n(B)")
                .replace("; or (C)", "\n\n(C)");
            let mut words = Vec::new();
            let mut options = Vec::new();
            for part in value.split("\n\n").map(str::trim) {
                let option = ["A", "B", "C"]
                    .into_iter()
                    .find_map(|letter| Some((letter, part.strip_prefix(&format!("({letter})"))?)));
                if let Some((letter, text)) = option {
                    options.push((format!("Option {letter}"), text.trim().to_owned()));
                } else if !part.is_empty() {
                    words.push(part.to_owned());
                }
            }
            traits.equipment = Some((words, options));
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
            traits.facts.push((label.to_owned(), value));
        }
    }
    if !remainder.is_empty() {
        // The table's own title (`**Wizard Features**`) is its heading. Only a table with no
        // title gets the generic one, so one block never has two headings.
        let (first, rest) = remainder.split_once('\n').unwrap_or((&remainder, ""));
        let title = first
            .trim()
            .trim_start_matches('#')
            .trim()
            .trim_matches('*')
            .trim();
        let (title, body) = if title.is_empty() || first.trim_start().starts_with('|') {
            ("Class table", remainder.as_str())
        } else {
            (title, rest)
        };
        traits.rest = Some((title.to_owned(), presentation::overview_markdown(body)));
    }
    Some(traits)
}

/// Draw a class's imported core traits, if its source text has them (`core_traits`).
pub(super) fn show_source(ui: &mut egui::Ui, source: &str, name: &str) -> bool {
    let Some(traits) = core_traits(source, name) else {
        return false;
    };
    design::heading(ui, design::Heading::Section("Core traits"));
    style::facts(ui, &traits.facts);
    if let Some((words, options)) = &traits.equipment {
        design::heading(ui, design::Heading::Section("Starting equipment"));
        for part in words {
            style::markdown(ui, part);
        }
        style::facts(ui, options);
    }
    if let Some((title, body)) = &traits.rest {
        design::heading(ui, design::Heading::Section(title));
        style::markdown(ui, body);
    }
    true
}

/// A class's or subclass's feature: its level, its name, and the entry it opens.
pub struct Feature {
    pub level: Option<u32>,
    pub name: String,
    pub target: Option<usize>,
}

pub fn features(entry: &Entry, links: &dyn Links) -> Vec<Feature> {
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
pub fn related(entry: &Entry, links: &dyn Links) -> Vec<(usize, String)> {
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
