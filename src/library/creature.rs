//! A reading view of creature values, with the original rules prose retained.
use super::presentation;
use crate::design::{
    self, Type, colors,
    reader::{self as reader_style, heavy_rule, section_heading},
};
use eframe::egui;
use pulldown_cmark::{Event, Options, Parser, Tag};
use splat::content::{
    Entry, Reference,
    dice_text::{self, D20, RollLink},
};

const ABILITIES: [&str; 6] = [
    "strength",
    "dexterity",
    "constitution",
    "intelligence",
    "wisdom",
    "charisma",
];
const DETAILS: [(&str, &str); 10] = [
    ("movement_text", "Speed"),
    ("skills_text", "Skills"),
    ("resistances_text", "Resistances"),
    ("immunities_text", "Immunities"),
    ("vulnerabilities_text", "Vulnerabilities"),
    ("gear_text", "Gear"),
    ("senses_text", "Senses"),
    ("languages_text", "Languages"),
    ("cr_text", "Challenge"),
    ("habitat_text", "Habitat"),
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

/// Printed vital lines are prose. Only splat_stats supplies fight values.
fn body_vital(line: &str) -> Option<(&'static str, &'static str, i64)> {
    let plain = line.trim().replace('*', "");
    for (prefix, key, label) in [
        ("Hit Points", "max_hp", "Hit Points"),
        ("HP", "max_hp", "Hit Points"),
        ("Armor Class", "armor_class", "Armor Class"),
        ("AC", "armor_class", "Armor Class"),
    ] {
        if let Some(rest) = plain.strip_prefix(prefix)
            && rest.starts_with([' ', ':'])
            && let Some(value) = rest
                .trim_start_matches([' ', ':'])
                .split_whitespace()
                .next()
                .and_then(|number| number.parse().ok())
        {
            return Some((key, label, value));
        }
    }
    None
}

/// Each printed vital line that disagrees with the stat block: its stat key, the DM's word for it
/// (`Hit Points`), the printed value and the warning.
pub(super) fn vital_checks(entry: &Entry) -> Vec<(&'static str, &'static str, i64, String)> {
    if !has_stats(entry) {
        return Vec::new();
    }
    vital_lines(&entry.body)
        .into_iter()
        .filter_map(|range| body_vital(&entry.body[range]))
        .filter_map(|(key, label, printed)| {
            let value = stat(entry, key)?;
            (value != printed).then(|| {
                (
                    key,
                    label,
                    printed,
                    format!(
                        "{label} line says {printed}, but the stat block says {value}. The fight \
                         uses the stat block."
                    ),
                )
            })
        })
        .collect()
}

fn vital_lines(body: &str) -> Vec<std::ops::Range<usize>> {
    let mut lines = Vec::new();
    for (event, range) in Parser::new_ext(body, Options::ENABLE_TABLES).into_offset_iter() {
        if matches!(event, Event::Start(Tag::Paragraph)) {
            let mut start = range.start;
            for line in body[range].split_inclusive('\n') {
                if body_vital(line).is_some() {
                    lines.push(start..start + line.len());
                }
                start += line.len();
            }
        }
    }
    lines
}

pub(super) fn without_vitals(entry: &Entry, body: &str) -> String {
    let mut output = String::new();
    let mut start = 0;
    for range in vital_lines(body) {
        if body_vital(&body[range.clone()]).is_some_and(|(key, _, _)| stat(entry, key).is_some()) {
            output.push_str(&body[start..range.start]);
            start = range.end;
        }
    }
    output.push_str(&body[start..]);
    splat::content::stat_blocks::reading_body(entry, &output)
}

pub(super) fn use_vital(
    text: &str,
    entry: &Entry,
    key: &str,
    value: i64,
) -> Result<String, String> {
    let mut stats = entry.fields["splat_stats"].clone();
    stats[key]["value"] = value.into();
    splat::content::formats::patch_model(text, &[("splat_stats", Some(stats))])
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
                && (stat(entry, "armor_class").is_some()
                    || field(entry, "armor_class_text").is_some())
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
                if line.starts_with(&format!("{label} ")) && detail(entry, key).is_some() {
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

/// `text` is one part of the entry's `body`. A tile's number is its action's place in the
/// whole body's list, which is the list Ctrl+number arms from.
fn prose(
    ui: &mut egui::Ui,
    name: &str,
    body: &str,
    text: &str,
    values: Option<&super::view::RunValues>,
    spell_links: &mut impl FnMut(&mut egui::Ui, &str),
) {
    let all = crate::rolls::actions(body);
    let here = crate::rolls::actions(text);
    let mut tiles: Vec<(Option<usize>, crate::rolls::Chip)> = Vec::new();
    // Action tiles in a row. In a run, a tile shows the numbers Splat rolls, with a run's
    // attack and damage changes in them, so no line under the tiles repeats those changes.
    let flush = |ui: &mut egui::Ui, tiles: &mut Vec<(Option<usize>, crate::rolls::Chip)>| {
        if !tiles.is_empty() {
            crate::rolls::tiles(ui, name, tiles);
            tiles.clear();
        }
    };
    for (at, paragraph) in text
        .split("\n\n")
        .map(str::trim)
        .enumerate()
        .filter(|(_, s)| !s.is_empty())
    {
        // A paragraph with an attack, a save or damage is an action tile; its text is in hover.
        if let Some((_, chip)) = here.iter().find(|(place, _)| *place == at) {
            tiles.push((crate::rolls::place_in(&all, chip), chip.clone()));
            continue;
        }
        flush(ui, &mut tiles);
        if let Some(speed) = values.and_then(|values| values.speed.as_ref())
            && paragraph.lines().any(|line| line.starts_with("Speed "))
        {
            for line in paragraph.lines() {
                if let Some(printed) = line.strip_prefix("Speed ") {
                    changed_speed(ui, printed, speed);
                } else {
                    reader_style::source_prose(ui, line);
                }
            }
        } else {
            reader_style::source_prose(ui, paragraph);
        }
        if paragraph.starts_with("Spellcasting.") {
            spell_links(ui, paragraph);
        }
    }
    flush(ui, &mut tiles);
}

/// Rules text written by hand: its actions as tiles, the rest as prose.
pub(super) fn rules_text(ui: &mut egui::Ui, name: &str, body: &str, text: &str) {
    prose(ui, name, body, text, None, &mut |_, _| {});
}

fn changed_speed(ui: &mut egui::Ui, printed: &str, speed: &str) {
    let preview = crate::encounter_run::change_preview(&splat::content::overrides::Row {
        label: "Speed",
        base: printed.into(),
        changed: speed.into(),
        by_hand: true,
        in_rolls: false,
    });
    reader_style::stat_detail(ui, "Speed", &preview, colors().participant.changed);
}

/// One ability row of the stat block's attribute table.
fn ability_row(ui: &mut egui::Ui, entry: &Entry, ability: &str, saves: bool) {
    let c = colors();
    let modifier = stat(entry, ability).map(|score| (score - 10).div_euclid(2));
    // An absent override does not establish proficiency or a save total.
    let save = stat(entry, &format!("save_{ability}"));
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = design::XS;
        let [label, score, modifier_cell] = ABILITY_CELLS;
        cell(
            ui,
            label,
            &ability[..3].to_uppercase(),
            Type::StatLabel,
            c.text,
        );
        cell(ui, score, &value(entry, ability), Type::Stat, c.text);
        let name = dice_text::ability(ability).unwrap_or(ability);
        match modifier {
            Some(bonus) => link_cell(
                ui,
                modifier_cell,
                &design::signed(bonus),
                &RollLink::d20(D20::Check, bonus, name),
            ),
            None => cell(ui, modifier_cell, "—", Type::Stat, c.muted),
        }
        match save {
            Some(bonus) => link_cell(
                ui,
                SAVE_CELL,
                &format!("Save {}", design::signed(bonus)),
                &RollLink::d20(D20::Save, bonus, name),
            ),
            None if saves => cell(ui, SAVE_CELL, "", Type::Stat, c.muted),
            None => {}
        }
    });
}

/// The widths of an attribute row's cells: label, score, modifier, then the save.
const ABILITY_CELLS: [f32; 3] = [30.0, 24.0, 26.0];
const SAVE_CELL: f32 = 56.0;

/// One half of the attribute table, physical or mental, with or without its Save column.
fn ability_half_width(saves: bool) -> f32 {
    let cells: f32 = ABILITY_CELLS.iter().sum::<f32>() + 2.0 * design::XS;
    if saves {
        cells + design::XS + SAVE_CELL
    } else {
        cells
    }
}

/// An attribute row is as tall as a pointer target, as its numbers roll.
const ABILITY_ROW: f32 = 24.0;

fn cell(ui: &mut egui::Ui, width: f32, text: &str, role: Type, color: egui::Color32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, ABILITY_ROW), egui::Sense::hover());
    let galley = design::one_line(ui, text, role.font(ui), color, width + design::S);
    let top = rect.center().y - galley.size().y / 2.0;
    // Numbers line up on their right edge, so that a column of them can be read down.
    let numeric = text.starts_with(|c: char| c.is_ascii_digit() || "+\u{2212}\u{2014}".contains(c));
    let left = if numeric {
        rect.right() - galley.size().x
    } else {
        rect.left()
    };
    ui.painter().galley(egui::pos2(left, top), galley, color);
}

/// A right-aligned number in the attribute table that rolls.
fn link_cell(ui: &mut egui::Ui, width: f32, text: &str, link: &RollLink) {
    let c = colors();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, ABILITY_ROW), egui::Sense::hover());
    // An id from the link, so its result follows it when lines appear above the table.
    let response = ui.interact(
        rect,
        ui.id().with(("dice_cell", link.destination())),
        egui::Sense::click(),
    );
    let color = if response.hovered() {
        c.focus
    } else {
        c.accent
    };
    let galley = design::one_line(ui, text, Type::Stat.font(ui), color, width + design::S);
    let left = rect.right() - galley.size().x;
    let top = rect.center().y - galley.size().y / 2.0;
    let bottom = top + galley.size().y - 1.0;
    ui.painter().hline(
        left..=rect.right(),
        bottom,
        egui::Stroke::new(1.0, color.gamma_multiply(0.6)),
    );
    ui.painter().galley(egui::pos2(left, top), galley, color);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Link, true, crate::rolls::hint(link))
    });
    crate::rolls::interact(ui, &response, link);
}

/// The Skills row, with each bonus a dice link.
fn skills_detail(ui: &mut egui::Ui, text: &str) {
    let mut markdown = String::new();
    let mut at = 0;
    let skills = dice_text::skills(text);
    if skills.is_empty() {
        reader_style::stat_detail(ui, "Skills", text, colors().text);
        return;
    }
    for (range, name, bonus) in skills {
        // A skill and its bonus wrap as one unit: the blanks inside `Sleight of Hand +5` do not
        // break, so the bonus never starts a line without its name.
        let between = &text[at..range.start];
        let skill = between.trim_start_matches(|c: char| c == ',' || c == ';' || c.is_whitespace());
        markdown.push_str(&between[..between.len() - skill.len()]);
        markdown.push_str(&skill.replace(char::is_whitespace, "\u{a0}"));
        markdown.push_str(&RollLink::d20(D20::Skill, bonus, name).markdown(&text[range.clone()]));
        at = range.end;
    }
    markdown.push_str(&text[at..]);
    reader_style::linked_detail(ui, "Skills", |ui| {
        crate::markdown_view::show_skills(ui, &markdown);
    });
}

/// Armor class, hit points and initiative, under capital labels. Only what the entry has.
fn vital_rows(entry: &Entry) -> Vec<(&'static str, String)> {
    let hp = field(entry, "hit_points_text")
        .filter(|text| {
            text.split_whitespace()
                .next()
                .and_then(|n| n.parse::<i64>().ok())
                == stat(entry, "max_hp")
        })
        .map(str::to_owned)
        .or_else(|| stat(entry, "max_hp").map(|hp| hp.to_string()))
        .or_else(|| splat::content::stat_blocks::invalid_vital(entry, "max_hp"));
    let armor_class = field(entry, "armor_class_text")
        .filter(|text| {
            text.split_whitespace()
                .next()
                .and_then(|n| n.parse::<i64>().ok())
                == stat(entry, "armor_class")
        })
        .map(str::to_owned)
        .or_else(|| stat(entry, "armor_class").map(|ac| ac.to_string()))
        .or_else(|| splat::content::stat_blocks::invalid_vital(entry, "armor_class"));
    [
        ("ARMOR CLASS", armor_class),
        ("HIT POINTS", hp),
        (
            "INITIATIVE",
            stat(entry, "initiative")
                .map(design::signed)
                .or_else(|| splat::content::stat_blocks::invalid_vital(entry, "initiative")),
        ),
    ]
    .into_iter()
    .filter_map(|(label, text)| text.map(|text| (label, text)))
    .collect()
}

pub(super) fn vitals(ui: &mut egui::Ui, entry: &Entry, values: Option<&super::view::RunValues>) {
    let c = colors();
    ui.with_layout(
        egui::Layout::left_to_right(egui::Align::Min).with_main_wrap(true),
        |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(design::L, 2.0);
            let printed = vital_rows(entry);
            let rows = if values.is_some() {
                ["ARMOR CLASS", "HIT POINTS", "INITIATIVE"]
                    .into_iter()
                    .map(|label| {
                        (
                            label,
                            printed
                                .iter()
                                .find(|(key, _)| *key == label)
                                .map_or_else(|| "Unknown".to_owned(), |(_, text)| text.clone()),
                        )
                    })
                    .collect()
            } else {
                printed
            };
            for (label, text) in rows {
                let run_hp = values.filter(|_| label == "HIT POINTS").map(|values| {
                    format!(
                        "{} / {}",
                        values
                            .current_hp
                            .map_or_else(|| "Unknown".to_owned(), |hp| hp.to_string()),
                        values
                            .hp
                            .map_or_else(|| "Unknown".to_owned(), |hp| hp.to_string())
                    )
                });
                let printed = match label {
                    "ARMOR CLASS" => stat(entry, "armor_class"),
                    "HIT POINTS" => stat(entry, "max_hp"),
                    _ => stat(entry, "initiative"),
                };
                let invalid = printed.is_none() && text != "Unknown";
                let display = if invalid {
                    format!("{} {text}", design::icons::WARN)
                } else {
                    text.clone()
                };
                let width = design::number_width(ui, label, &Type::Label.font(ui))
                    .max(design::number_width(ui, &display, &Type::Prose.font(ui)))
                    .max(design::number_width(
                        ui,
                        run_hp.as_deref().unwrap_or(&text),
                        &Type::Prose.font(ui),
                    ))
                    + design::S;
                ui.allocate_ui_with_layout(
                    egui::vec2(width, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        ui.label(Type::Label.rich(ui, label).color(c.muted));
                        let live = values.map(|values| match label {
                            "ARMOR CLASS" => values.armor_class,
                            "HIT POINTS" => values.hp,
                            _ => values.initiative,
                        });
                        let shown = live.map_or_else(
                            || text.clone(),
                            |n| {
                                n.map_or_else(
                                    || "Unknown".to_owned(),
                                    |n| {
                                        if label == "INITIATIVE" {
                                            design::signed(n)
                                        } else {
                                            n.to_string()
                                        }
                                    },
                                )
                            },
                        );
                        let shown = run_hp.unwrap_or(shown);
                        if invalid {
                            ui.label(
                                Type::Prose
                                    .rich(ui, format!("{} {shown}", design::icons::WARN))
                                    .color(c.result.warning),
                            );
                        } else {
                            ui.label(Type::Prose.rich(ui, shown).color(c.text));
                        }
                        if let Some(live) = live
                            && live != printed
                        {
                            ui.label(
                                Type::Small
                                    .rich(ui, format!("printed {}", unbroken_dice(&text)))
                                    .color(c.muted),
                            );
                        }
                    },
                );
            }
        },
    );
}

fn unbroken_dice(text: &str) -> String {
    let mut in_dice = false;
    text.chars()
        .map(|c| {
            if c == '(' {
                in_dice = true;
            }
            if c == ')' {
                in_dice = false;
            }
            if in_dice && c == ' ' { '\u{a0}' } else { c }
        })
        .collect()
}

fn detail(entry: &Entry, key: &str) -> Option<String> {
    if key == "cr_text" {
        return presentation::challenge_text(entry).map(str::to_owned);
    }
    if let Some(text) = field(entry, key) {
        return Some(text.to_owned());
    }
    if key != "movement_text" {
        return None;
    }
    let movement = entry
        .fields
        .get("splat_stats")?
        .as_mapping()?
        .iter()
        .filter_map(|(key, data)| {
            let key = key.as_str()?;
            if key != "speed" && !key.ends_with("_speed") {
                return None;
            }
            let amount = data.get("value")?.as_i64()?;
            let unit = data.get("unit").and_then(|v| v.as_str()).unwrap_or("");
            Some(if key == "speed" {
                format!("{amount} {unit}")
            } else {
                format!("{} {amount} {unit}", key.replace('_', " "))
            })
        })
        .collect::<Vec<_>>();
    (!movement.is_empty()).then(|| movement.join(", "))
}

fn stat_identity(entry: &Entry) -> String {
    let kind = field(entry, "type_options_text")
        .or_else(|| field(entry, "type"))
        .map(|kind| {
            let mut chars = kind.chars();
            let kind = chars.next().map_or_else(String::new, |first| {
                first.to_uppercase().chain(chars).collect()
            });
            match field(entry, "tags_text") {
                Some(tags) => format!("{kind} {tags}"),
                None => kind,
            }
        });
    let alignment = field(entry, "alignment").map(|value| {
        value
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                chars.next().map_or_else(String::new, |first| {
                    first.to_uppercase().chain(chars).collect()
                })
            })
            .collect::<Vec<_>>()
            .join(" ")
    });
    let identity = [size(entry), kind.as_deref(), alignment.as_deref()]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" · ");
    reader_style::human_label(&identity)
}

/// Identity, vitals, abilities and details: what the entry has, and no empty cells.
pub(super) fn stat_head(ui: &mut egui::Ui, entry: &Entry, values: Option<&super::view::RunValues>) {
    ui.add_space(design::S);
    reader_style::identity(ui, &stat_identity(entry));
    heavy_rule(ui);
    ui.add_space(design::S);
    vitals(ui, entry, values);
    ui.add_space(design::S);
    // The rulebook's table: physical abilities beside mental ones, with a clear gutter between
    // them. A creature with no saving throw has no Save column.
    let saves = ABILITIES
        .iter()
        .any(|ability| stat(entry, &format!("save_{ability}")).is_some());
    let two_columns = ui.available_width() >= 2.0 * ability_half_width(saves) + design::XL;
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
                    ui.spacing_mut().item_spacing.y = 0.0;
                    for ability in column {
                        ability_row(ui, entry, ability, saves);
                    }
                });
            }
        });
    } else {
        ui.spacing_mut().item_spacing.y = 0.0;
        for ability in ABILITIES {
            ability_row(ui, entry, ability, saves);
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
            if label == "Speed"
                && let Some(speed) = values.and_then(|values| values.speed.as_ref())
            {
                changed_speed(ui, &text, speed);
            } else if label == "Skills" {
                skills_detail(ui, &text);
            } else {
                reader_style::stat_detail(ui, label, &text, colors().text);
            }
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
    values: Option<&super::view::RunValues>,
    mut spell_links: impl FnMut(&mut egui::Ui, &str),
) {
    stat_head(ui, entry, values);
    for section in sections {
        if section.title == "Source text" {
            prose(
                ui,
                &entry.name,
                &entry.body,
                &rules_prose(entry, section.body),
                values,
                &mut spell_links,
            );
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
            prose(
                ui,
                &entry.name,
                &entry.body,
                &body,
                values,
                &mut spell_links,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use splat::content::layout::{Field, FlagKind};

    #[test]
    fn creature_identity_uses_book_case_and_keeps_tags() {
        let source =
            splat::content::definitions::source_file("definitions/creatures/aboleth.md").unwrap();
        let mut entry = Entry::parse("Aboleth.md", source).unwrap();
        assert_eq!(stat_identity(&entry), "Large · Aberration · Lawful Evil");
        entry.fields.get_mut("splat_creature").unwrap()["type"] = "dragon".into();
        entry.fields.get_mut("splat_creature").unwrap()["tags_text"] = "(Chromatic)".into();
        assert_eq!(
            stat_identity(&entry),
            "Large · Dragon (Chromatic) · Lawful Evil"
        );
        let template = crate::library::note_text("New", "creature", None);
        assert!(template.contains("| Alignment | Neutral |"));
    }

    #[test]
    fn source_cr_and_speed_are_shown_once_without_markdown_breaks() {
        let text = crate::library::note_text("Cinder Wolf", "creature", None);
        let mut entry = Entry::parse("Cinder Wolf.md", &text).unwrap();
        entry.body = "## Source text\n\nMedium humanoid, Unaligned\\\nAC 10  Initiative +0 (10)\\\nHP 1\\\nSpeed 30 ft.\\\nCR 1 (XP 200; PB +2)\\  \n\n### Actions\n\nBite. Roll damage.\n".into();
        let sections = presentation::reading_sections(&entry.body, &entry.name);
        let source = sections
            .iter()
            .find(|s| s.title == "Source text")
            .unwrap()
            .body;
        assert_eq!(
            detail(&entry, "cr_text").as_deref(),
            Some("1 (XP 200; PB +2)")
        );
        assert_eq!(detail(&entry, "movement_text").as_deref(), Some("30 ft."));
        let prose = rules_prose(&entry, source);
        assert!(!prose.contains("CR 1"));
        assert!(!prose.contains("Speed 30"));
        assert!(prose.contains("Bite."));
        entry.fields.get_mut("splat_creature").unwrap()["cr_text"] = "2 (XP 450)".into();
        assert_eq!(detail(&entry, "cr_text").as_deref(), Some("2 (XP 450)"));
        assert!(!rules_prose(&entry, source).contains("CR 1"));
    }

    #[test]
    fn fmt_f2_rejected_hp_uses_the_vital_row_without_becoming_a_stat() {
        let text = crate::library::note_text("Bad HP", "creature", None)
            .replace("| Hit Points |  |", "| Hit Points | about twenty |");
        let entry = Entry::parse("Bad HP.md", &text).unwrap();
        assert_eq!(stat(&entry, "max_hp"), None);
        assert!(vital_rows(&entry).contains(&("HIT POINTS", "about twenty".into())));
        assert!(!without_vitals(&entry, &entry.body).contains("| Hit Points |"));
        assert!(text.contains("| Hit Points | about twenty |"));
        let repaired = use_vital(&text, &entry, "max_hp", 22).unwrap();
        let repaired = Entry::parse("Bad HP.md", &repaired).unwrap();
        assert_eq!(stat(&repaired, "max_hp"), Some(22));
        assert!(
            repaired
                .flags
                .iter()
                .all(|flag| flag.kind != FlagKind::Unread(Field::HitPoints))
        );
    }

    #[test]
    fn prose_vitals_warn_and_use_updates_the_fight_source() {
        let text = format!(
            "{}\n**Hit Points** 45\n\n**Armor Class** 17\n",
            crate::library::note_text("Guard", "creature", None)
                .replace("| Hit Points |  |", "| Hit Points | 1 |")
                .replace("| Armor Class |  |", "| Armor Class | 10 |")
        );
        let entry = Entry::parse("Guard.md", &text).unwrap();
        let checks = vital_checks(&entry);
        assert_eq!(checks.len(), 2);
        assert_eq!(checks[0].1, "Hit Points");
        assert_eq!(
            checks[0].3,
            "Hit Points line says 45, but the stat block says 1. The fight uses the stat block."
        );
        let changed = use_vital(&text, &entry, "max_hp", 45).unwrap();
        let entry = Entry::parse("Guard.md", &changed).unwrap();
        assert_eq!(stat(&entry, "max_hp"), Some(45));
        assert!(!without_vitals(&entry, &entry.body).contains("**Hit Points**"));
        assert!(!crate::library::note_text("New", "creature", None).contains("**Hit Points**"));
    }

    #[test]
    fn copied_stat_action_keeps_one_line_per_stat_in_book_order() {
        let source =
            splat::content::definitions::source_file("definitions/creatures/aboleth.md").unwrap();
        let text = source.to_owned();
        let entry = Entry::parse("Aboleth.md", &text).unwrap();
        let changed = use_vital(&text, &entry, "max_hp", 180).unwrap();
        assert!(
            changed.contains("| Hit Points | 180 (20d10 + 40) |"),
            "{}",
            changed.chars().take(1800).collect::<String>()
        );
        assert_eq!(
            Entry::parse("Aboleth.md", &changed).unwrap().fields["splat_stats"]["max_hp"]["value"]
                .as_i64(),
            Some(180)
        );
    }

    #[test]
    fn book_stat_action_updates_the_marked_map_and_reader_uses_its_value() {
        let text =
            splat::content::definitions::source_file("definitions/creatures/aboleth.md").unwrap();
        let entry = Entry::parse("Aboleth.md", text).unwrap();
        let changed = use_vital(text, &entry, "max_hp", 180).unwrap();
        let changed = Entry::parse("Aboleth.md", &changed).unwrap();
        assert_eq!(stat(&changed, "max_hp"), Some(180));
        assert!(vital_rows(&changed).contains(&("HIT POINTS", "180 (20d10 + 40)".into())));
        assert!(without_vitals(&changed, &changed.body).contains("Mucus Cloud"));
    }

    #[test]
    fn a_hand_written_creature_reads_as_a_stat_block_with_only_what_it_has() {
        let text = splat::content::formats::fixture(
            "Notes/Varra.md",
            "splat_id: 8f4a2f8e-41f0-4c4a-9d4c-0a7a1a1b2c3d\nsplat_kind: creature\nsplat_name: Varra\nsplat_creature: {size: medium}\nsplat_stats:\n  max_hp: {value: 52}\n  armor_class: {value: 16}\n",
            "\n# Varra\n\n## Actions\n\nLongsword. Melee Attack Roll: +5.\n",
        )
        .unwrap();
        let entry = Entry::parse("Notes/Varra.md", &text).unwrap();
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
        assert_eq!(prose.matches("HP 65").count(), 1);
        entry.body = "## Description\nA custom creature with structured actions.".into();
        assert!(
            !supports(
                &entry,
                &presentation::reading_sections(&entry.body, &entry.name)
            ),
            "Keep the existing complete view when no source rules prose exists"
        );
    }

    /// A harness that draws `add` in a panel `width` wide.
    fn drawn(
        width: f32,
        add: impl FnMut(&mut egui::Ui) + 'static,
    ) -> egui_kittest::Harness<'static> {
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(width, 900.0))
            .build_ui(add);
        design::theme::install(&harness.ctx);
        harness.run_ok();
        harness
    }

    /// One painted glyph: its row in its galley, its character (a no-break space reads as a
    /// blank), its left and right edges and its baseline.
    #[derive(Clone, Copy, Debug)]
    struct Drawn {
        row: usize,
        chr: char,
        left: f32,
        right: f32,
        baseline: f32,
    }

    /// The glyphs of the first painted text that holds `needle`.
    fn glyphs_of(harness: &egui_kittest::Harness<'_>, needle: &str) -> Vec<Drawn> {
        for shape in &harness.output().shapes {
            let egui::Shape::Text(text) = &shape.shape else {
                continue;
            };
            let glyphs: Vec<Drawn> = text
                .galley
                .rows
                .iter()
                .enumerate()
                .flat_map(|(row_at, row)| {
                    row.glyphs.iter().map(move |glyph| {
                        let left = text.pos.x + row.pos.x + glyph.pos.x;
                        Drawn {
                            row: row_at,
                            chr: if glyph.chr == '\u{a0}' {
                                ' '
                            } else {
                                glyph.chr
                            },
                            left,
                            right: left + glyph.advance_width,
                            baseline: text.pos.y + row.pos.y + glyph.pos.y,
                        }
                    })
                })
                .collect();
            let words: String = glyphs.iter().map(|glyph| glyph.chr).collect();
            if words.contains(needle) {
                return glyphs;
            }
        }
        panic!("{needle} is painted");
    }

    /// The glyphs of each place `words` is drawn, in reading order.
    fn runs(glyphs: &[Drawn], words: &str) -> Vec<Vec<Drawn>> {
        let chars: Vec<char> = glyphs.iter().map(|glyph| glyph.chr).collect();
        let wanted: Vec<char> = words.chars().collect();
        (0..=chars.len().saturating_sub(wanted.len()))
            .filter(|at| chars[*at..*at + wanted.len()] == wanted[..])
            .map(|at| glyphs[at..at + wanted.len()].to_vec())
            .collect()
    }

    /// Every link's press target, in reading order.
    fn link_targets(harness: &egui_kittest::Harness<'_>) -> Vec<egui::Rect> {
        use egui_kittest::kittest::NodeT;
        let mut targets: Vec<egui::Rect> = harness
            .root()
            .children_recursive()
            .filter(|node| node.accesskit_node().role() == egui::accesskit::Role::Link)
            .map(|node| node.rect())
            .collect();
        targets.sort_by(|a, b| {
            (a.top(), a.left())
                .partial_cmp(&(b.top(), b.left()))
                .unwrap()
        });
        targets
    }

    /// FD3 joint witness: `Deception +2, Religion +2` reads with no space before the comma,
    /// both bonuses are `XL` square targets apart from each other and from the words around them,
    /// and each bonus is as far from its skill as the other.
    #[test]
    fn iter20_s_fd3_a_skills_row_has_no_space_before_its_comma() {
        // The panel widths of a stat block beside the editor, in Library and in a narrow side.
        for width in [960.0, 480.0, 300.0] {
            let harness = drawn(width, |ui| skills_detail(ui, "Deception +2, Religion +2"));
            let at = format!("at {width}");
            let glyphs = glyphs_of(&harness, "Deception");
            let bonuses = runs(&glyphs, "+2");
            assert_eq!(bonuses.len(), 2, "FD3-bonuses: {at}");
            let comma = runs(&glyphs, ",");
            assert_eq!(comma.len(), 1, "FD3-comma: {at}");
            let gap = comma[0][0].left - bonuses[0][1].right;
            // The font may kern the comma under the digit; a blank would be about 3 px or more.
            assert!(
                gap.abs() <= 1.0,
                "FD3-no-space-before-comma: {at}: gap {gap}"
            );
            let targets = link_targets(&harness);
            assert_eq!(targets.len(), 2, "FD3-targets: {at}: {targets:?}");
            let religion = runs(&glyphs, "Religion")[0][0];
            for (target, bonus) in targets.iter().zip(&bonuses) {
                assert!(
                    target.width() >= design::XL - 0.01 && target.height() >= design::XL - 0.01,
                    "FD3-target-size: {at}: {target:?}"
                );
                assert!(
                    target.left() <= bonus[0].left + 0.5 && target.right() >= bonus[1].right - 0.5,
                    "FD3-target-covers-bonus: {at}: {target:?} {bonus:?}"
                );
            }
            // On one row, the first target ends before the next skill starts.
            assert!(
                religion.row != bonuses[0][0].row || targets[0].right() <= religion.left + 0.5,
                "FD3-target-clear-of-next-skill: {at}: {:?}, Religion at {}",
                targets[0],
                religion.left
            );
            let shared = targets[0].intersect(targets[1]);
            assert!(
                shared.width() <= 0.01 || shared.height() <= 0.01,
                "FD3-targets-apart: {at}: {targets:?}"
            );
            // Must stay true: the bonus that ends the row is as far from its skill as the one
            // before the comma, so the row reads evenly.
            let names = [
                runs(&glyphs, "Deception")[0][8],
                runs(&glyphs, "Religion")[0][7],
            ];
            let gaps: Vec<f32> = names
                .iter()
                .zip(&bonuses)
                .map(|(name, bonus)| bonus[0].left - name.right)
                .collect();
            // Glyphs snap to whole pixels, so the gaps can differ by one.
            assert!(
                (gaps[0] - gaps[1]).abs() <= 1.0,
                "FD3-even-skill-gaps: {at}: {gaps:?}"
            );
        }
    }

    /// Iteration 21, FD4 joint witness (revised): a Skills row keeps one normal blank between each
    /// name and its bonus and one after each comma. A bonus's target is at least `XL` square: it
    /// extends left over its own skill's name, stays inside its entry, and is clear of the other
    /// targets. The row wraps by whole entries. From the narrowest stat block to the widest
    /// reader, so the Library and Lookup panels at 960 and 1920 are both inside it.
    #[test]
    fn iter21_s_fd4_a_skill_bonus_target_covers_its_own_entry() {
        // The tightest text: the narrowest bonus, then two bonuses as wide as they get.
        const SKILLS: &str = "Insight +8, Perception +14, Persuasion +14";
        let skills = [
            ("Insight", "+8"),
            ("Perception", "+14"),
            ("Persuasion", "+14"),
        ];
        let (mut wrapped, mut one_row) = (0, 0);
        // From the narrowest panel where the longest entry, `Persuasion +14,`, fits a line (as SD3).
        for width in (260..=1000).step_by(4) {
            let harness = drawn(width as f32, |ui| skills_detail(ui, SKILLS));
            let at = format!("at {width}");
            let glyphs = glyphs_of(&harness, "Insight");
            if glyphs.last().unwrap().row > 0 {
                wrapped += 1;
            } else {
                one_row += 1;
            }
            let targets = link_targets(&harness);
            assert_eq!(targets.len(), skills.len(), "FD4-links: {at}");
            let mut name_gaps = Vec::new();
            for (at_skill, ((name, bonus), target)) in skills.iter().zip(&targets).enumerate() {
                let unit = format!("{name} {bonus}");
                let found = runs(&glyphs, &unit);
                assert_eq!(found.len(), 1, "FD4-unit-drawn: {at}: {unit}");
                let unit_glyphs = &found[0];
                let name_end = unit_glyphs[name.len() - 1];
                let digits = &unit_glyphs[name.len() + 1..];
                let row = unit_glyphs[0].row;
                // Find this unit's place, then what follows it.
                let start = glyphs
                    .iter()
                    .position(|glyph| {
                        glyph.row == row
                            && glyph.chr == unit_glyphs[0].chr
                            && glyph.left == unit_glyphs[0].left
                    })
                    .unwrap();
                let after = &glyphs[start + unit.chars().count()..];
                assert!(
                    target.width() >= design::XL - 0.01 && target.height() >= design::XL - 0.01,
                    "FD4-target-size: {at}: {unit} {target:?}"
                );
                // The target may cover its own skill's name, but stays inside its entry.
                assert!(
                    target.left() >= unit_glyphs[0].left - 0.5,
                    "FD4-target-inside-entry: {at}: {unit} target {target:?}, entry starts {}",
                    unit_glyphs[0].left
                );
                let blank = unit_glyphs[name.len()];
                name_gaps.push((digits[0].left - name_end.right, blank.right - blank.left));
                assert!(
                    target.contains(egui::pos2(
                        (digits[0].left + digits.last().unwrap().right) / 2.0,
                        digits[0].baseline - 2.0
                    )),
                    "FD4-target-on-bonus: {at}: {unit} target {target:?}"
                );
                // The spare width is all before the bonus, so its target ends with its words.
                assert!(
                    target.right() <= digits.last().unwrap().right + 0.5,
                    "FD4-spare-to-the-left: {at}: {unit} target {target:?}, bonus ends {}",
                    digits.last().unwrap().right
                );
                if at_skill + 1 == skills.len() {
                    continue;
                }
                let [comma, blank, next, ..] = after else {
                    panic!("FD4-next-entry: {at}: nothing follows {unit}");
                };
                assert_eq!(
                    (comma.chr, blank.chr),
                    (',', ' '),
                    "FD4-comma: {at}: {unit}"
                );
                // Whole entries: the name, its bonus and its comma share a row.
                assert!(
                    unit_glyphs
                        .iter()
                        .chain([comma])
                        .all(|glyph| glyph.row == row),
                    "FD4-whole-entries: {at}: {unit} is split across rows"
                );
                // Must stay true (iteration 20): no space before the comma.
                assert!(
                    (comma.left - digits.last().unwrap().right).abs() <= 1.0,
                    "FD4-no-space-before-comma: {at}: {unit}"
                );
                // One blank after the comma: the next entry starts where the blank's own width
                // ends.
                assert!(
                    blank.left - comma.right <= 0.5 && next.left - blank.right <= 0.5,
                    "FD4-one-blank: {at}: {unit}, comma ends {}, blank {}..{}, next starts {}",
                    comma.right,
                    blank.left,
                    blank.right,
                    next.left
                );
                assert!(
                    next.row != row || target.right() <= next.left + 0.5,
                    "FD4-target-clear-of-next: {at}: {unit} target {target:?}, next starts {}",
                    next.left
                );
            }
            // `+8` and `+14` sit as far from their names as each other. Glyphs snap to whole
            // pixels, so the gaps can differ by one.
            let (least, most) = name_gaps
                .iter()
                .fold((f32::MAX, f32::MIN), |(a, b), (gap, _)| {
                    (a.min(*gap), b.max(*gap))
                });
            assert!(
                most - least <= 1.0,
                "FD4-equal-name-gaps: {at}: {name_gaps:?}"
            );
            // One normal blank between each name and its bonus, whatever the bonus's width.
            for (gap, blank) in &name_gaps {
                assert!(
                    (gap - blank).abs() <= 0.5,
                    "FD4-one-blank-before-bonus: {at}: gap {gap}, blank {blank}"
                );
            }
            for (a, first) in targets.iter().enumerate() {
                for second in &targets[a + 1..] {
                    let shared = first.intersect(*second);
                    assert!(
                        shared.width() <= 0.01 || shared.height() <= 0.01,
                        "FD4-targets-apart: {at}: {targets:?}"
                    );
                }
            }
        }
        assert!(
            wrapped > 0 && one_row > 0,
            "FD4-widths-reached: wrapped {wrapped}, one row {one_row}"
        );
    }

    /// SD3: a skill and its bonus wrap as one unit at every width, and each bonus keeps its own
    /// link target.
    #[test]
    fn iter20_s_sd3_a_skill_and_its_bonus_wrap_together() {
        const SKILLS: &str =
            "Insight +8, Perception +14, Persuasion +9, Sleight of Hand +5, Stealth +8";
        let skills = [
            ("Insight", "+8"),
            ("Perception", "+14"),
            ("Persuasion", "+9"),
            ("Sleight of Hand", "+5"),
            ("Stealth", "+8"),
        ];
        let mut wrapped = 0;
        // From the narrowest panel where the longest unit, `Sleight of Hand +5`, fits a line.
        for width in (260..=600).step_by(4) {
            let harness = drawn(width as f32, |ui| skills_detail(ui, SKILLS));
            let at = format!("at {width}");
            let glyphs = glyphs_of(&harness, "Insight");
            let rows = glyphs.last().map_or(0, |glyph| glyph.row + 1);
            if rows > 1 {
                wrapped += 1;
            }
            let targets = link_targets(&harness);
            assert_eq!(targets.len(), skills.len(), "SD3-links: {at}");
            for ((name, bonus), target) in skills.iter().zip(&targets) {
                let unit = format!("{name} {bonus}");
                let found = runs(&glyphs, &unit);
                assert_eq!(found.len(), 1, "SD3-unit-drawn: {at}: {unit}");
                let unit_glyphs = &found[0];
                assert!(
                    unit_glyphs
                        .iter()
                        .all(|glyph| glyph.row == unit_glyphs[0].row),
                    "SD3-one-row: {at}: {unit} is split across rows"
                );
                let digit = unit_glyphs.last().unwrap();
                assert!(
                    target.contains(egui::pos2(
                        (digit.left + digit.right) / 2.0,
                        digit.baseline - 2.0
                    )),
                    "SD3-bonus-keeps-target: {at}: {unit} target {target:?}"
                );
            }
        }
        assert!(wrapped > 0, "SD3-wrap-reached: no width wrapped the row");
    }

    /// FD4: every detail row of a stat block has one pitch, the link pitch, whether or not it
    /// holds a link; a wrapped row keeps that pitch on every line.
    #[test]
    fn iter20_s_fd4_stat_detail_rows_have_one_pitch() {
        let source =
            splat::content::definitions::source_file("definitions/creatures/aboleth.md").unwrap();
        for width in [960.0, 240.0] {
            let entry = Entry::parse("Aboleth.md", source).unwrap();
            let harness = drawn(width, move |ui| stat_head(ui, &entry, None));
            let at = format!("at {width}");
            let mut rows: Vec<(f32, Vec<Drawn>)> = Vec::new();
            for (label, first) in [
                ("Speed", "10 ft."),
                ("Skills", "History"),
                ("Senses", "Darkvision"),
                ("Languages", "Deep"),
                ("Challenge", "(XP"),
            ] {
                let label_glyphs = glyphs_of(&harness, label);
                let value = glyphs_of(&harness, first);
                let start = value[0];
                // Must stay true: the label sits on its value's first baseline.
                assert!(
                    (label_glyphs[0].baseline - start.baseline).abs() <= 0.5,
                    "FD4-label-baseline: {at}: {label} {} vs {first} {}",
                    label_glyphs[0].baseline,
                    start.baseline
                );
                rows.push((label_glyphs[0].baseline, value));
            }
            for pair in rows.windows(2) {
                let (top, value) = &pair[0];
                let lines = value.last().unwrap().row + 1;
                let pitch = pair[1].0 - top;
                let expected = lines as f32 * design::XL + 2.0;
                assert!(
                    (pitch - expected).abs() <= 0.5,
                    "FD4-pitch: {at}: {lines} lines then {pitch} (expected {expected})"
                );
                let mut baselines: Vec<f32> = value.iter().map(|glyph| glyph.baseline).collect();
                baselines.dedup_by(|a, b| (*a - *b).abs() < 0.5);
                for line in baselines.windows(2) {
                    assert!(
                        (line[1] - line[0] - design::XL).abs() <= 0.5,
                        "FD4-wrapped-pitch: {at}: {baselines:?}"
                    );
                }
            }
            if width < 300.0 {
                assert!(
                    rows.iter().any(|(_, value)| value.last().unwrap().row > 0),
                    "FD4-wrap-reached: {at}"
                );
            }
        }
    }
}
