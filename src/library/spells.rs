//! A stat block's spells as tiles: each spell that its Spellcasting lists, with what the spell
//! rolls when this creature casts it. A spell with an attack or a save is an action tile; a
//! spell with neither opens its description.
#![warn(deprecated)] // Its parent allows old message functions; this file does not.
use super::{presentation, reader, view::Links};
use crate::rolls::Chip;
use splat::content::{
    Entry, Reference,
    spell_rolls::{Caster, spell_roll},
};
use splat::memo::Memo;
use std::sync::Arc;

/// One spell of a list line.
#[derive(Clone, Debug)]
pub(crate) struct SpellTile {
    /// `Guiding Bolt`, without the words in brackets after it.
    pub name: String,
    /// The spell's place in the links' entries.
    pub open: usize,
    /// `Level 2 · Action · 120 feet · Concentration`.
    pub facts: String,
    /// The spell's rules text, for hover.
    pub text: String,
    /// What it rolls when this creature casts it.
    pub chip: Option<Chip>,
}

/// One line of a Spellcasting list: its label and its spells.
pub(crate) struct SpellLine {
    /// `At Will`, `2/Day Each`.
    pub label: String,
    pub tiles: Vec<SpellTile>,
    /// Names the links cannot find.
    pub missing: Vec<String>,
}

/// A line of a list of spells: a short label, a colon, then the spells (`At Will: Light`,
/// `1/Day Each: Fear`, `Cantrips (at will): fire bolt`). A line ends with `\` when another follows.
pub(crate) fn list_line(line: &str) -> bool {
    let line = line.trim().trim_end_matches('\\').trim_end();
    line.split_once(':').is_some_and(|(label, spells)| {
        label.len() <= 40
            && !label.contains('.')
            && label.starts_with(|c: char| c.is_uppercase() || c.is_ascii_digit())
            && !spells.trim().is_empty()
    })
}

/// The spells of a list, split at the commas outside brackets: `Shapechange (Beast or
/// Humanoid form only, …)` is one spell.
fn list_items(spells: &str) -> Vec<&str> {
    let mut items = Vec::new();
    let (mut depth, mut start) = (0_usize, 0);
    for (at, c) in spells.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                items.push(&spells[start..at]);
                start = at + 1;
            }
            _ => {}
        }
    }
    items.push(&spells[start..]);
    items
}

/// A list item's spell name and the level it is cast at, if the item says one:
/// `Guiding Bolt (level 2 version)` is `Guiding Bolt` at level 2.
fn item_name(item: &str) -> (&str, Option<u32>) {
    let item = item
        .trim()
        .trim_end_matches('\\')
        .trim()
        .trim_matches(['*', '_']);
    let (name, rest) = item.split_once(" (").unwrap_or((item, ""));
    let level = rest
        .to_lowercase()
        .split_once("level ")
        .and_then(|(_, after)| {
            after
                .split(|c: char| !c.is_ascii_digit())
                .next()?
                .parse()
                .ok()
        });
    (name.trim().trim_matches(['*', '_']).trim(), level)
}

/// A creature's numbers for its spells: the save DC its data gives, or else its words
/// (`spell save DC 13`), and its bonus to hit (`+6 to hit with spell attacks`).
pub(crate) fn caster(entry: &Entry, spellcasting: &str) -> Caster {
    let data = entry
        .fields
        .get("splat_rule")
        .and_then(|v| v.get("parameters"))
        .and_then(|v| v.get("features"))
        .and_then(|v| v.get("spellcasting"))
        .and_then(|v| v.get("save_dc"))
        .and_then(serde_yaml_ng::Value::as_i64);
    let words = spellcasting
        .split_once("spell save DC ")
        .and_then(|(_, rest)| {
            rest.split(|c: char| !c.is_ascii_digit())
                .next()?
                .parse()
                .ok()
        });
    Caster {
        dc: data.or(words),
        attack: Caster::attack_in(spellcasting),
    }
}

/// A spell's rules text: its printed description, without the lines of facts at its top.
fn rules_text(spell: &Entry) -> String {
    let sections = presentation::reading_sections(&spell.body, &spell.name);
    let section = sections
        .iter()
        .find(|section| section.title == "Source text")
        .or_else(|| {
            sections.iter().find(|section| {
                !["Spell", "Rule", "Personal notes"].contains(&section.title.as_str())
            })
        });
    let Some(section) = section else {
        return String::new();
    };
    let facts = |line: &str| {
        let line = line.trim();
        [
            "Level ",
            "Casting Time:",
            "Range:",
            "Components:",
            "Duration:",
        ]
        .iter()
        .any(|start| line.starts_with(start))
            || line.contains(" Cantrip (")
    };
    presentation::overview_markdown(section.body)
        .split("\n\n")
        .filter(|paragraph| !paragraph.trim().is_empty() && !paragraph.lines().all(facts))
        .collect::<Vec<_>>()
        .join("\n\n")
}

/// The level of a spell, 0 for a cantrip.
fn spell_level(spell: &Entry) -> Option<u32> {
    let level = spell.fields.get("splat_spell")?.get("level")?.as_u64()?;
    u32::try_from(level).ok()
}

/// A spell's tile facts at the level it is cast: `Level 2 · Action · 120 feet ·
/// Concentration`, and `half on a success` when a save halves its damage.
fn facts(spell: &Entry, cast: Option<u32>, chip: Option<&Chip>) -> String {
    let mut facts = Vec::new();
    match cast {
        Some(0) => facts.push("Cantrip".to_owned()),
        Some(level) => facts.push(format!("Level {level}")),
        None => {}
    }
    if let Some(fields) = spell.fields.get("splat_spell") {
        let printed = reader::spell_facts(fields);
        let fact = |name: &str| {
            printed
                .iter()
                .find(|(label, _)| label == name)
                .map(|(_, value)| value.clone())
        };
        facts.extend(fact("Casting Time"));
        // A range of `Self` is left out of the table; the printed text keeps it.
        facts.extend(fact("Range").or_else(|| {
            spell.body.lines().find_map(|line| {
                let range = line.trim().strip_prefix("Range:")?;
                let range = range.trim().trim_end_matches('\\').trim();
                (!range.is_empty()).then(|| range.to_owned())
            })
        }));
        if fields
            .get("concentration")
            .and_then(serde_yaml_ng::Value::as_bool)
            == Some(true)
        {
            facts.push("Concentration".to_owned());
        }
    }
    if chip.is_some_and(|chip| {
        matches!(
            chip.link,
            splat::content::dice_text::RollLink::SaveDc { half: true, .. }
        )
    }) {
        facts.push("half on a success".to_owned());
    }
    facts.join(" · ")
}

/// One spell's tile, kept by the spell, the level cast and the caster's numbers, so a frame
/// that draws the same list reads no spell text again.
fn tile(name: &str, open: usize, spell: &Entry, cast: Option<u32>, caster: Caster) -> SpellTile {
    static TILES: Memo<Arc<SpellTile>> = Memo::new(256);
    let cast_bytes = cast.map_or([0xff; 4], u32::to_le_bytes);
    let numbers = [caster.dc, caster.attack].map(|n| n.map_or([0xff; 8], i64::to_le_bytes));
    let tile = TILES.get_or(
        &[
            name.as_bytes(),
            open.to_le_bytes().as_slice(),
            spell.id.as_bytes().as_slice(),
            spell.body.as_bytes(),
            cast_bytes.as_slice(),
            numbers[0].as_slice(),
            numbers[1].as_slice(),
        ],
        || {
            let text = rules_text(spell);
            let level = spell_level(spell);
            let cast = cast.or(level);
            let chip = level
                .zip(cast)
                .and_then(|(level, cast)| spell_roll(name, &text, level, cast, &caster))
                .and_then(|link| Chip::new(link, text.clone()));
            let facts = facts(spell, cast, chip.as_ref());
            let chip = chip.map(|chip| Chip {
                facts: Some(facts.clone()),
                ..chip
            });
            Arc::new(SpellTile {
                name: name.to_owned(),
                open,
                facts,
                text,
                chip,
            })
        },
    );
    (*tile).clone()
}

/// The spells of one list line, each found by the entry's own references first, then by
/// name.
pub(crate) fn line(
    line: &str,
    caster: Caster,
    references: &[Option<(usize, String)>],
    links: &dyn Links,
) -> Option<SpellLine> {
    let line = line.trim().trim_end_matches('\\').trim_end();
    let (label, spells) = line.split_once(':')?;
    let mut tiles = Vec::new();
    let mut missing = Vec::new();
    for item in list_items(spells) {
        let (name, cast) = item_name(item);
        if name.is_empty() {
            continue;
        }
        let found = references
            .iter()
            .flatten()
            .find(|(_, spell)| spell.eq_ignore_ascii_case(name))
            .cloned()
            .or_else(|| links.spell_named(name));
        match found.and_then(|(open, _)| Some((open, links.entry(open)?))) {
            Some((open, spell)) => tiles.push(tile(name, open, spell, cast, caster)),
            None => missing.push(name.to_owned()),
        }
    }
    Some(SpellLine {
        label: label.trim().to_owned(),
        tiles,
        missing,
    })
}

/// The entry's spells, each with the entry it opens, or `None` when it is missing.
pub(crate) fn references(entry: &Entry, links: &dyn Links) -> Vec<Option<(usize, String)>> {
    super::creature::spell_references(entry)
        .iter()
        .map(|reference: &Reference| links.find_spell(reference))
        .collect()
}

/// Each Spellcasting paragraph of `body` with the list paragraphs after it.
pub(crate) fn groups(body: &str) -> Vec<(&str, Vec<&str>)> {
    let mut groups: Vec<(&str, Vec<&str>)> = Vec::new();
    let mut open = false;
    for paragraph in body.split("\n\n").map(str::trim).filter(|p| !p.is_empty()) {
        if paragraph.starts_with("Spellcasting.") {
            groups.push((paragraph, Vec::new()));
            open = true;
        } else if open && paragraph.lines().all(list_line) {
            if let Some((_, lists)) = groups.last_mut() {
                lists.push(paragraph);
            }
        } else {
            open = false;
        }
    }
    groups
}

/// The spells an entry's lists name that roll, in printed order. In a run they follow the
/// stat block's own actions, in the panel, on the action bar and by Ctrl+number.
pub(crate) fn rolls(entry: &Entry, links: &dyn Links) -> Vec<Chip> {
    let references = references(entry, links);
    groups(&entry.body)
        .into_iter()
        .flat_map(|(spellcasting, lists)| {
            let caster = caster(entry, spellcasting);
            lists
                .into_iter()
                .flat_map(str::lines)
                .filter_map(|text| line(text, caster, &references, links))
                .flat_map(|line| line.tiles)
                .filter_map(|tile| tile.chip)
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_list_item_gives_its_name_and_the_level_cast() {
        assert_eq!(
            item_name(" Guiding Bolt (level 2 version)"),
            ("Guiding Bolt", Some(2))
        );
        assert_eq!(
            item_name(" Shapechange (Beast or Humanoid form only, no HP)\\"),
            ("Shapechange", None)
        );
        assert_eq!(item_name("*fire bolt*"), ("fire bolt", None));
        assert_eq!(
            list_items(" Guiding Bolt (level 2 version), Shapechange (Beast, no HP), Light"),
            vec![
                " Guiding Bolt (level 2 version)",
                " Shapechange (Beast, no HP)",
                " Light"
            ]
        );
    }
}
