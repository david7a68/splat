//! A stat block's spells as tiles: each spell that its Spellcasting lists, with what the spell
//! rolls when this creature casts it. A spell with an attack or a save is an action tile; a
//! spell with neither opens its description.
#![warn(deprecated)] // Its parent allows old message functions; this file does not.
use super::{presentation, reader, view::Links};
use crate::rolls::{Chip, SpellOccurrence};
use splat::content::{
    Entry, Reference,
    spell_rolls::{Caster, spell_roll},
};
use std::sync::Arc;

/// One spell of a list line.
#[derive(Clone, Debug)]
pub struct SpellTile {
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
    /// Its place in the creature's complete action order, when it was resolved from the body.
    pub place: Option<usize>,
    /// The printed item, including any preceding items that could not be resolved.
    item: usize,
}

/// One line of a Spellcasting list: its label and its spells.
pub struct SpellLine {
    /// `At Will`, `2/Day Each`.
    pub label: String,
    pub tiles: Vec<SpellTile>,
    /// Names the links cannot find.
    pub missing: Vec<String>,
}

/// A line of a list of spells: how often, a colon, then the spells (`At Will: Light`,
/// `1/Day Each: Fear`, `2/Day Each (Elder Only): Fly`, `Cantrips (at will): fire bolt`,
/// `1st level (4 slots): shield`). A line ends with `\` when another follows. A book may print
/// the label in bold: `**At Will:** Light`. Other labels, such as `Environments: Forest`,
/// are not lists of spells.
pub fn list_line(line: &str) -> bool {
    let line = line.trim().trim_end_matches('\\').trim_end();
    line.split_once(':').is_some_and(|(label, spells)| {
        how_often(plain_label(label)) && !spells.trim_matches(['*', '_', ' ']).is_empty()
    })
}

/// Whether a label says how often its spells are cast: `At Will`, `3/Day`, `Cantrips`, or a
/// spell slot level, `1st level`.
fn how_often(label: &str) -> bool {
    let lower = label.to_lowercase();
    let digits = lower.len() - lower.trim_start_matches(|c: char| c.is_ascii_digit()).len();
    let after = &lower[digits..];
    lower.starts_with("at will")
        || lower.starts_with("cantrip")
        || (digits > 0 && after.starts_with("/day"))
        || (digits > 0
            && ["st level", "nd level", "rd level", "th level"]
                .iter()
                .any(|ordinal| after.starts_with(ordinal)))
}

/// A list label without its emphasis: `**At Will` is `At Will`.
fn plain_label(label: &str) -> &str {
    label.trim().trim_matches(['*', '_']).trim()
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
pub fn caster(entry: &Entry, spellcasting: &str) -> Caster {
    let data = entry
        .fields
        .get("splat_rule")
        .and_then(|v| v.get("parameters"))
        .and_then(|v| v.get("features"))
        .and_then(|v| v.get("spellcasting"))
        .and_then(|v| v.get("save_dc"))
        .and_then(serde_yaml_ng::Value::as_i64);
    let dc = data.or(dc_in(spellcasting));
    // A spell attack bonus is always the save DC less 8, so a stat block that prints only
    // the DC still gives it.
    numbers(
        entry,
        dc,
        Caster::attack_in(spellcasting).or(dc.map(|dc| dc - 8)),
    )
}

/// A caster's numbers, with its spellcasting ability modifier: the attack bonus less the
/// proficiency bonus that its challenge line prints (`CR 2 (XP 450; PB +2)`).
fn numbers(entry: &Entry, dc: Option<i64>, attack: Option<i64>) -> Caster {
    let proficiency = entry.body.split_once("PB +").and_then(|(_, rest)| {
        rest.split(|c: char| !c.is_ascii_digit())
            .next()?
            .parse::<i64>()
            .ok()
    });
    Caster {
        dc,
        attack,
        modifier: attack.zip(proficiency).map(|(attack, pb)| attack - pb),
    }
}

/// The spell save DC that words give: `spell save DC 13`.
fn dc_in(words: &str) -> Option<i64> {
    let (_, rest) = words.split_once("spell save DC ")?;
    rest.split(|c: char| !c.is_ascii_digit())
        .next()?
        .parse()
        .ok()
}

/// Whether a paragraph is a Spellcasting feature whose lists follow it: `Spellcasting.`,
/// `Spellcasting (Snake Form Only).`, `Innate Spellcasting.`, each introducing its lists (`one
/// of the following spells`, or a closing colon). A feature so named that casts a spell
/// itself (`Hellfire Spellcasting (3/Day). The fiend casts Fireball ...`) is a cast line.
pub fn spellcasting(paragraph: &str) -> bool {
    let named = crate::design::reader::run_in_title(paragraph).is_some_and(|title| {
        title
            .split(" (")
            .next()
            .is_some_and(|name| name == "Spellcasting" || name.ends_with(" Spellcasting"))
    });
    named && (paragraph.contains("following spell") || paragraph.trim_end().ends_with(':'))
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
fn facts(spell: &Entry, cast: Option<u32>, chip: Option<&Chip>, text: &str) -> String {
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
        let range = fact("Range").or_else(|| {
            spell.body.lines().find_map(|line| {
                let range = line.trim().strip_prefix("Range:")?;
                let range = range.trim().trim_end_matches('\\').trim();
                (!range.is_empty()).then(|| range.to_owned())
            })
        });
        // The area of a spell that rolls, as its text gives it, unless the range names it:
        // `Self (15-foot Cube)`. A spell with no roll may have several areas, one per effect.
        let area = crate::rolls::area(&text.to_lowercase()).filter(|_| {
            chip.is_some() && !range.as_deref().is_some_and(|range| range.contains('('))
        });
        facts.extend(range);
        facts.extend(area);
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

/// One spell's tile: its facts, its rules text, and what it rolls when this caster casts it
/// at `cast`. Built only when an entry is resolved, which its links' cache keeps.
fn tile(
    name: &str,
    open: usize,
    spell: &Entry,
    cast: Option<u32>,
    caster: Caster,
    item: usize,
) -> SpellTile {
    let text = rules_text(spell);
    let level = spell_level(spell);
    let cast = cast.or(level);
    let chip = level
        .zip(cast)
        .and_then(|(level, cast)| spell_roll(name, &text, level, cast, &caster))
        .and_then(|link| Chip::new(link, text.clone()));
    let facts = facts(spell, cast, chip.as_ref(), &text);
    let chip = chip.map(|chip| Chip {
        facts: Some(facts.clone()),
        ..chip
    });
    SpellTile {
        name: name.to_owned(),
        open,
        facts,
        text,
        chip,
        place: None,
        item,
    }
}

/// A tile's facts without `Concentration`, which a list item may waive.
fn without_concentration(facts: &str) -> String {
    facts.replace(" · Concentration", "")
}

/// One list item's spell tile, found by the entry's own references first, then by name:
/// `Err` with the name when no spell has it, `None` for an empty item.
fn item_tile(
    position: usize,
    item: &str,
    caster: Caster,
    references: &[Option<(usize, String)>],
    links: &dyn Links,
) -> Option<Result<SpellTile, String>> {
    let (name, cast) = item_name(item);
    if name.is_empty() {
        return None;
    }
    // Books print an apostrophe as `’` or `'`; the two name the same spell.
    let straight = name.replace('’', "'");
    let curly = name.replace('\'', "’");
    let named = |name: &str| {
        [name, &straight, &curly]
            .into_iter()
            .find_map(|name| links.spell_named(name))
    };
    let found = references
        .iter()
        .flatten()
        .find(|(_, spell)| spell.replace('’', "'").eq_ignore_ascii_case(&straight))
        .cloned()
        .or_else(|| named(name))
        // The SRD drops the name of a spell's maker: `Vellora’s Hideous Laughter` is
        // `Hideous Laughter` there.
        .or_else(|| {
            let (_, rest) = straight.split_once("'s ")?;
            named(rest)
        });
    let Some((open, spell)) = found.and_then(|(open, _)| Some((open, links.entry(open)?))) else {
        return Some(Err(name.to_owned()));
    };
    let mut spell = tile(name, open, spell, cast, caster, position);
    // The list may waive it: `Shapechange (... no Concentration ... required)`.
    if item.to_lowercase().contains("no concentration") {
        spell.facts = without_concentration(&spell.facts);
        if let Some(chip) = &mut spell.chip {
            chip.facts = chip.facts.as_deref().map(without_concentration);
        }
    }
    Some(Ok(spell))
}

/// The spells of one list line.
pub fn line(
    line: &str,
    caster: Caster,
    references: &[Option<(usize, String)>],
    links: &dyn Links,
) -> Option<SpellLine> {
    let line = line.trim().trim_end_matches('\\').trim_end();
    let (label, spells) = line.split_once(':')?;
    let mut tiles = Vec::new();
    let mut missing = Vec::new();
    for (position, item) in list_items(spells).into_iter().enumerate() {
        match item_tile(position, item, caster, references, links) {
            Some(Ok(spell)) => tiles.push(spell),
            Some(Err(name)) => missing.push(name),
            None => {}
        }
    }
    Some(SpellLine {
        label: plain_label(label).to_owned(),
        tiles,
        missing,
    })
}

/// The spells that a paragraph of its own casts by name, as list items: `Dread Command. The
/// mummy casts Command (level 2 version), using ...` gives `Command (level 2 version)`. Not
/// a Spellcasting paragraph, whose lists follow it.
pub fn cast_names(paragraph: &str) -> Option<Vec<&str>> {
    if spellcasting(paragraph) {
        return None;
    }
    crate::design::reader::run_in_title(paragraph)?;
    let (_, rest) = paragraph.split_once(" casts ")?;
    let end = [
        ", using",
        " using",
        ", requiring",
        " requiring",
        " in response",
        " twice",
        " on ",
        ", but",
        ". ",
        ".\n",
    ]
    .iter()
    .filter_map(|stop| rest.find(stop))
    .min()
    .unwrap_or(rest.len());
    // `casts the Slow spell` names Slow.
    let names = rest[..end].trim().trim_end_matches('.');
    let names = names.strip_prefix("the ").unwrap_or(names);
    let names = names.strip_suffix(" spell").unwrap_or(names);
    if !names.starts_with(char::is_uppercase) {
        return None;
    }
    // `Darkness, Faerie Fire, or Web` and `Fireball or Lightning Bolt`.
    Some(
        list_items(names)
            .into_iter()
            .map(|item| {
                let item = item.trim();
                item.strip_prefix("or ")
                    .or_else(|| item.strip_prefix("and "))
                    .unwrap_or(item)
            })
            .flat_map(|item| item.split(" or "))
            .collect(),
    )
}

/// The tiles of the spells a paragraph of its own casts. Its numbers are its own
/// (`spell save DC 13`), or else those of the creature's Spellcasting, which such a paragraph
/// names (`using the same spellcasting ability as Spellcasting`). A name that no spell has is
/// left out: the words may name no spell at all.
pub fn cast_line(
    entry: &Entry,
    paragraph: &str,
    references: &[Option<(usize, String)>],
    links: &dyn Links,
) -> Option<SpellLine> {
    let names = cast_names(paragraph)?;
    let base = groups(&entry.body)
        .first()
        .map_or_else(|| caster(entry, ""), |(words, _)| caster(entry, words));
    let caster = match dc_in(paragraph) {
        Some(dc) => numbers(
            entry,
            Some(dc),
            Caster::attack_in(paragraph).or(Some(dc - 8)),
        ),
        None => base,
    };
    let tiles: Vec<SpellTile> = names
        .into_iter()
        .enumerate()
        .filter_map(|(position, item)| item_tile(position, item, caster, references, links)?.ok())
        .collect();
    (!tiles.is_empty()).then(|| SpellLine {
        label: String::new(),
        tiles,
        missing: Vec::new(),
    })
}

/// The entry's spells, each with the entry it opens, or `None` when it is missing.
pub fn references(entry: &Entry, links: &dyn Links) -> Vec<Option<(usize, String)>> {
    super::creature::spell_references(entry)
        .iter()
        .map(|reference: &Reference| links.find_spell(reference))
        .collect()
}

/// A paragraph that is only a comment, such as `<!-- SRD page 325 -->` where a printed page
/// ends: it parts no Spellcasting paragraph from its lists.
pub fn aside(paragraph: &str) -> bool {
    paragraph.starts_with("<!--") && paragraph.ends_with("-->") && !paragraph[4..].contains("<!--")
}

/// Each Spellcasting paragraph of `body` with the list paragraphs after it.
pub fn groups(body: &str) -> Vec<(&str, Vec<&str>)> {
    let mut groups: Vec<(&str, Vec<&str>)> = Vec::new();
    let mut open = false;
    for paragraph in body.split("\n\n").map(str::trim).filter(|p| !p.is_empty()) {
        if spellcasting(paragraph) {
            groups.push((paragraph, Vec::new()));
            open = true;
        } else if open && aside(paragraph) {
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

/// What an entry's Spellcasting resolves to, once for all the draws that need it: the tiles of
/// each list line and of each paragraph that casts spells by name, and the spells that roll in
/// printed order. In a run they follow the stat block's own actions, in the panel, on the
/// action bar and by Ctrl+number.
#[derive(Default)]
pub struct Resolved {
    /// The entry's spells, each with the entry it opens, or `None` when it is missing.
    pub references: Vec<Option<(usize, String)>>,
    /// Each list line, trimmed, with its tiles.
    lines: Vec<(String, Caster, SpellLine)>,
    /// Each paragraph that casts spells by name, by its words, with its tiles, or `None` when
    /// it names no spell that can be found.
    casts: Vec<(String, Option<SpellLine>)>,
    /// Each Spellcasting paragraph with its lists, as the reader joins them, with the spells
    /// their words name, for the line of spells that no list shows.
    named: Vec<(String, Vec<(usize, String)>)>,
    rolls: Vec<Chip>,
}

/// Text as its words alone, so the reader's text and the body's compare equal: the reader
/// joins and breaks lines differently and drops a line break's `\`.
fn words(text: &str) -> String {
    text.split(|c: char| c.is_whitespace() || c == '\\')
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

impl Resolved {
    /// The next occurrence of a list line. The reader presents the source in order and may
    /// omit sections. Identical lines in separate lists still have separate positions.
    pub fn line(&self, text: &str, caster: Caster, next: &mut usize) -> Option<&SpellLine> {
        let text = words(text);
        let at = *next
            + self.lines[*next..]
                .iter()
                .position(|(line, numbers, _)| *line == text && *numbers == caster)?;
        *next = at + 1;
        Some(&self.lines[at].2)
    }

    /// The spells that a Spellcasting paragraph and its lists name, by their joined words.
    pub fn named_in(&self, text: &str) -> Option<&[(usize, String)]> {
        let text = words(text);
        self.named
            .iter()
            .find(|(named, _)| *named == text)
            .map(|(_, named)| named.as_slice())
    }

    /// What a paragraph that casts spells by name resolved to: `Some(None)` when it names no
    /// spell that can be found, `None` when the paragraph was not resolved.
    pub fn cast(&self, paragraph: &str, next: &mut usize) -> Option<Option<&SpellLine>> {
        let paragraph = words(paragraph);
        let at = *next
            + self.casts[*next..]
                .iter()
                .position(|(text, _)| *text == paragraph)?;
        *next = at + 1;
        Some(self.casts[at].1.as_ref())
    }

    fn number(&mut self, spells: &mut SpellLine, paragraph: usize, line: usize) {
        for tile in &mut spells.tiles {
            if let Some(chip) = &mut tile.chip {
                tile.place = Some(self.rolls.len());
                chip.spell_occurrence = Some(SpellOccurrence {
                    paragraph,
                    line,
                    item: tile.item,
                });
                self.rolls.push(chip.clone());
            }
        }
    }
}

/// Resolved Spellcasting, kept by the content it was resolved from, with that content's other
/// caches: the Library's notes clear theirs whenever a note changes (`Loaded::invalidate`),
/// and a content index, which never changes, keeps its own until it is replaced. An entry
/// is found again by all that resolving it reads: its id, its text and its fields (the
/// save DC and the spells its data gives), so a changed draft resolves again.
#[derive(Default)]
pub struct SpellCache(std::sync::Mutex<Vec<(Kept, Arc<Resolved>)>>);

/// What a resolved entry is known by.
struct Kept {
    id: uuid::Uuid,
    body: String,
    fields: splat::content::Fields,
}

impl Kept {
    fn of(entry: &Entry) -> Self {
        Self {
            id: entry.id,
            body: entry.body.clone(),
            fields: entry.fields.clone(),
        }
    }

    fn is(&self, entry: &Entry) -> bool {
        self.id == entry.id && self.body == entry.body && *self.fields == *entry.fields
    }
}

impl SpellCache {
    /// How many entries are kept: the oldest goes first.
    const KEEP: usize = 64;

    /// Forget everything resolved: the content it was resolved from changed.
    pub(crate) fn clear(&mut self) {
        self.0
            .get_mut()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear();
    }

    fn get(&self, entry: &Entry) -> Option<Arc<Resolved>> {
        let kept = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        kept.iter()
            .find(|(known, _)| known.is(entry))
            .map(|(_, resolved)| Arc::clone(resolved))
    }

    fn keep(&self, entry: &Entry, resolved: &Arc<Resolved>) {
        let mut kept = self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if kept.len() >= Self::KEEP {
            kept.remove(0);
        }
        kept.push((Kept::of(entry), Arc::clone(resolved)));
    }
}

/// An entry's Spellcasting, resolved once and kept by the links' cache, so an unchanged frame
/// looks nothing up. Links with no cache resolve it on each call.
pub fn resolve(entry: &Entry, links: &dyn Links) -> Arc<Resolved> {
    let Some(cache) = links.spell_cache() else {
        return Arc::new(build(entry, links));
    };
    if let Some(resolved) = cache.get(entry) {
        return resolved;
    }
    let resolved = Arc::new(build(entry, links));
    cache.keep(entry, &resolved);
    resolved
}

fn build(entry: &Entry, links: &dyn Links) -> Resolved {
    let mut resolved = Resolved {
        references: references(entry, links),
        rolls: crate::rolls::chips(&entry.body),
        ..Resolved::default()
    };
    // The numbers of the Spellcasting paragraph whose lists come next, if any, and its words
    // with its lists', joined as the reader joins them.
    let mut lists: Option<Caster> = None;
    let mut group = String::new();
    let end_group = |group: &mut String, resolved: &mut Resolved| {
        if !group.is_empty() {
            let named = links.spells_named_in(group);
            resolved.named.push((words(group), named));
            group.clear();
        }
    };
    for (at, paragraph) in entry.body.split("\n\n").map(str::trim).enumerate() {
        if paragraph.is_empty() {
            continue;
        }
        if spellcasting(paragraph) {
            end_group(&mut group, &mut resolved);
            lists = Some(caster(entry, paragraph));
            group.push_str(paragraph);
            continue;
        }
        if let Some(caster) = lists {
            if aside(paragraph) {
                continue;
            }
            if paragraph.lines().all(list_line) {
                for (row, text) in paragraph.lines().enumerate() {
                    if let Some(mut found) = line(text, caster, &resolved.references, links) {
                        resolved.number(&mut found, at, row);
                        resolved.lines.push((words(text), caster, found));
                    }
                }
                group.push_str("\n\n");
                group.push_str(paragraph);
                continue;
            }
            lists = None;
            end_group(&mut group, &mut resolved);
        }
        // A paragraph that casts by name is kept also when no spell it names is found, so
        // the reader does not look again.
        if cast_names(paragraph).is_some() {
            let mut found = cast_line(entry, paragraph, &resolved.references, links);
            if let Some(found) = &mut found {
                resolved.number(found, at, 0);
            }
            resolved.casts.push((words(paragraph), found));
        }
    }
    end_group(&mut group, &mut resolved);
    resolved
}

/// A creature's action tiles in Run order. Its own actions come first, then each rolling spell
/// occurrence. Only the creature's own actions are available without a content index.
pub fn ordered_actions(entry: &Entry, links: Option<&dyn Links>) -> Vec<Chip> {
    links.map_or_else(
        || crate::rolls::chips(&entry.body),
        |links| resolve(entry, links).rolls.clone(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes what Splat makes of every creature, one JSON file each, for
    /// `scripts/check-creatures.sh`, which asks a model to check each one against its text.
    /// `SPLAT_CREATURE_PACKAGES`: package folders, separated by `:`, to load beside the
    /// built-in content. `SPLAT_CREATURE_DUMP`: the folder to write into. Reads only.
    #[test]
    #[ignore = "needs SPLAT_CREATURE_PACKAGES and SPLAT_CREATURE_DUMP"]
    fn dump_creatures() {
        use crate::rolls::tile_words;
        use serde_json::json;
        let out = std::path::PathBuf::from(std::env::var("SPLAT_CREATURE_DUMP").unwrap());
        std::fs::create_dir_all(&out).unwrap();
        let modules: Vec<crate::persistence::Module> = std::env::var("SPLAT_CREATURE_PACKAGES")
            .unwrap_or_default()
            .split(':')
            .filter(|path| !path.is_empty())
            .map(|path| {
                let path = std::path::PathBuf::from(path);
                let package = std::fs::read_to_string(path.join("Package.md")).unwrap();
                let field = |key: &str| {
                    package
                        .lines()
                        .find_map(|line| line.strip_prefix(key))
                        .unwrap()
                        .trim()
                        .trim_matches('"')
                        .to_owned()
                };
                crate::persistence::Module {
                    id: field("splat_id:").parse().unwrap(),
                    name: field("splat_name:"),
                    path,
                }
            })
            .collect();
        let index = crate::lookup::ContentIndex::load(&modules, None).unwrap();
        let tile = |chip: &Chip| {
            let (numbers, facts) = tile_words(chip);
            json!({ "label": chip.label, "numbers": numbers, "facts": facts })
        };
        let mut written = 0;
        for item in index
            .items
            .iter()
            .filter(|item| item.entry.kind == "creature")
        {
            let entry = &item.entry;
            let references = references(entry, &index);
            let groups: Vec<_> = groups(&entry.body)
                .into_iter()
                .map(|(spellcasting, lists)| {
                    let caster = caster(entry, spellcasting);
                    let lines: Vec<_> = lists
                        .iter()
                        .flat_map(|list| list.lines())
                        .filter_map(|text| {
                            let line = line(text, caster, &references, &index)?;
                            Some(json!({
                                "printed": text,
                                "label": line.label,
                                "missing": line.missing,
                                "spells": line.tiles.iter().map(|spell| json!({
                                    "name": spell.name,
                                    "opens": index.entry(spell.open).map(|e| e.key.clone().unwrap_or_else(|| e.name.clone())),
                                    "facts": spell.facts,
                                    "rolls": spell.chip.as_ref().map(&tile),
                                    "spell_text": spell.text,
                                    "spell_record": index
                                        .entry(spell.open)
                                        .and_then(|e| e.fields.get("splat_spell"))
                                        .and_then(|v| serde_json::to_value(v).ok()),
                                })).collect::<Vec<_>>(),
                            }))
                        })
                        .collect();
                    json!({
                        "spellcasting": spellcasting,
                        "save_dc": caster.dc,
                        "spell_attack": caster.attack,
                        "lines": lines,
                    })
                })
                .collect();
            let record = json!({
                "name": entry.name,
                "key": entry.key,
                "path": entry.path,
                "owner": item.owner,
                "actions": crate::rolls::chips(&entry.body).iter().map(&tile).collect::<Vec<_>>(),
                "spellcasting": groups,
                "run_order": ordered_actions(entry, Some(&index))
                    .into_iter()
                    .map(|chip| chip.label)
                    .collect::<Vec<_>>(),
                "references": references
                    .iter()
                    .map(|found| found.as_ref().map(|(_, name)| name.clone()))
                    .collect::<Vec<_>>(),
                "body": entry.body,
            });
            let slug: String = format!("{} {}", entry.name, &entry.id.to_string()[..8])
                .chars()
                .map(|c| {
                    if c.is_ascii_alphanumeric() {
                        c.to_ascii_lowercase()
                    } else {
                        '_'
                    }
                })
                .collect();
            std::fs::write(
                out.join(format!("{slug}.json")),
                serde_json::to_string_pretty(&record).unwrap(),
            )
            .unwrap();
            written += 1;
        }
        eprintln!("wrote {written} creatures to {}", out.display());
    }

    /// Links that count their lookups, with a version of their content to key on, or none.
    struct Counting<'a> {
        index: &'a crate::lookup::ContentIndex,
        cache: Option<SpellCache>,
        /// Use the content index's own cache.
        index_cache: bool,
        lookups: std::cell::Cell<usize>,
    }

    impl Links for Counting<'_> {
        fn find(&self, reference: &Reference) -> Option<(usize, String)> {
            self.lookups.set(self.lookups.get() + 1);
            self.index.find(reference)
        }
        fn find_spell(&self, reference: &Reference) -> Option<(usize, String)> {
            self.lookups.set(self.lookups.get() + 1);
            self.index.find_spell(reference)
        }
        fn spells_named_in(&self, text: &str) -> Vec<(usize, String)> {
            self.lookups.set(self.lookups.get() + 1);
            self.index.spells_named_in(text)
        }
        fn spell_named(&self, name: &str) -> Option<(usize, String)> {
            self.lookups.set(self.lookups.get() + 1);
            self.index.spell_named(name)
        }
        fn entry(&self, index: usize) -> Option<&Entry> {
            Links::entry(self.index, index)
        }
        fn spell_cache(&self) -> Option<&SpellCache> {
            if self.index_cache {
                return Links::spell_cache(self.index);
            }
            self.cache.as_ref()
        }
    }

    #[test]
    fn an_unchanged_frame_resolves_no_spell_again() {
        let index = crate::lookup::ContentIndex::load(&[], None).unwrap();
        let path = "definitions/creatures/druid.md";
        let druid = Entry::parse(
            path,
            splat::content::definitions::source_file(path).unwrap(),
        )
        .unwrap();
        // Links with a cache resolve an entry once: the second frame looks nothing up.
        let mut kept = Counting {
            index: &index,
            cache: Some(SpellCache::default()),
            index_cache: false,
            lookups: std::cell::Cell::new(0),
        };
        let first = resolve(&druid, &kept);
        let once = kept.lookups.get();
        assert!(once > 0);
        let second = resolve(&druid, &kept);
        assert_eq!(
            kept.lookups.get(),
            once,
            "the second frame looks nothing up"
        );
        assert_eq!(first.rolls, second.rolls);
        // A changed text (a draft) resolves again, as does anything after a clear.
        let mut draft = druid.clone();
        draft.body.push_str("\n\nA line the DM added.");
        resolve(&draft, &kept);
        assert_eq!(
            kept.lookups.get(),
            2 * once,
            "a changed text resolves again"
        );
        // So does a change to the fields alone, such as the save DC, with the new numbers.
        let mut harder = druid.clone();
        let dc = harder
            .fields
            .get_mut("splat_rule")
            .and_then(|v| v.get_mut("parameters"))
            .and_then(|v| v.get_mut("features"))
            .and_then(|v| v.get_mut("spellcasting"))
            .and_then(|v| v.get_mut("save_dc"))
            .unwrap();
        *dc = 19.into();
        let resolved = resolve(&harder, &kept);
        assert_eq!(kept.lookups.get(), 3 * once, "changed fields resolve again");
        assert!(
            resolved
                .rolls
                .iter()
                .any(|chip| chip.label == "Entangle DC 19")
        );
        kept.cache.as_mut().unwrap().clear();
        resolve(&druid, &kept);
        assert_eq!(
            kept.lookups.get(),
            4 * once,
            "a cleared cache resolves again"
        );
        // The content index of Lookup and a run keeps what it resolved.
        let through_index = Counting {
            index: &index,
            cache: None,
            index_cache: true,
            lookups: std::cell::Cell::new(0),
        };
        resolve(&druid, &through_index);
        let first = through_index.lookups.get();
        resolve(&druid, &through_index);
        assert_eq!(
            through_index.lookups.get(),
            first,
            "the content index keeps it"
        );
        // Links with no cache resolve on each call.
        let fresh = Counting {
            index: &index,
            cache: None,
            index_cache: false,
            lookups: std::cell::Cell::new(0),
        };
        resolve(&druid, &fresh);
        resolve(&druid, &fresh);
        assert_eq!(fresh.lookups.get(), 2 * once);
    }

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
        // A book prints its labels in bold; the spells after them are the same.
        assert!(list_line("**At Will:** Light, Mage Hand"));
        assert_eq!(item_name("** Light"), ("Light", None));
        assert_eq!(plain_label("**1/Day Each"), "1/Day Each");
        assert!(!list_line(
            "Spellcasting. The mage casts one of the following spells:"
        ));
        // A page comment between a Spellcasting paragraph and its lists parts nothing.
        let body =
            "Spellcasting. It casts:\n\n<!-- SRD page 325 -->\n\nAt Will: Light\n\nBite. Hit.";
        assert_eq!(
            groups(body),
            vec![("Spellcasting. It casts:", vec!["At Will: Light"])]
        );
        // The SRD drops the name of a spell's maker.
        let index = crate::lookup::ContentIndex::load(&[], None).unwrap();
        let laughter = line(
            "1/Day: Vellora’s Hideous Laughter",
            Caster::default(),
            &[],
            &index,
        )
        .unwrap();
        assert!(laughter.missing.is_empty());
        assert_eq!(laughter.tiles[0].name, "Vellora’s Hideous Laughter");
        let straight = line(
            "1/Day: Vellora's Hideous Laughter",
            Caster::default(),
            &[],
            &index,
        );
        assert!(straight.unwrap().missing.is_empty(), "either apostrophe");
        // A spell's own name may have either apostrophe, as may the list that names it.
        let source = splat::content::formats::fixture(
            "Vellora.md",
            "splat_id: 5b0e2f38-6a7c-4d1e-9a52-0c3f1e2d4b67\nsplat_kind: spell\nsplat_name: Vellora's Storm\nsplat_spell: {level: 5}\n",
            "",
        )
        .unwrap();
        let loaded = super::super::Loaded {
            notes: vec![super::super::Note {
                entry: Some(Entry::parse("Vellora.md", &source).unwrap()),
                path: "Vellora.md".into(),
                text: crate::design::memo::Revised::new(source.clone()),
                error: None,
                fault: None,
                manifest_gone: false,
                owner: super::super::Owner::Builtin,
            }],
            ..Default::default()
        };
        let curly = line("1/Day: Vellora’s Storm", Caster::default(), &[], &loaded).unwrap();
        assert!(
            curly.missing.is_empty(),
            "a curly apostrophe finds a straight one"
        );
        // A Spellcasting feature may have more words in its title.
        assert!(spellcasting("Spellcasting (Snake Form Only). It casts:"));
        assert!(spellcasting("Innate Spellcasting. It casts:"));
        assert!(!spellcasting("Spell Turning. It reflects spells."));
        // A stat block that prints only the DC still gives the attack bonus: DC less 8.
        let path = "definitions/creatures/druid.md";
        let druid = Entry::parse(
            path,
            splat::content::definitions::source_file(path).unwrap(),
        )
        .unwrap();
        let numbers = caster(&druid, "Spellcasting. It casts (spell save DC 13):");
        assert_eq!((numbers.dc, numbers.attack), (Some(13), Some(5)));
        // Its ability modifier is the attack bonus less its proficiency bonus (`PB +2`).
        assert_eq!(numbers.modifier, Some(3));
        // A list may waive Concentration; a spell with no roll shows no area, since it may
        // have several, one for each effect.
        let shapechange = line(
            "At Will: Shapechange (Beast form only, no Concentration required)",
            numbers,
            &[],
            &index,
        )
        .unwrap();
        assert!(!shapechange.tiles[0].facts.contains("Concentration"));
        let druidcraft = line("At Will: Druidcraft", numbers, &[], &index).unwrap();
        assert_eq!(druidcraft.tiles[0].facts, "Cantrip · Action · 30 feet");
        // A paragraph of its own may cast spells by name, with its own DC or Spellcasting's.
        assert_eq!(
            cast_names("Web Magic (Recharge 5–6). The spider casts Darkness, Faerie Fire, or Web, requiring no Material components (spell save DC 13).").unwrap(),
            ["Darkness", "Faerie Fire", "Web"]
        );
        assert_eq!(
            cast_names("Dread Shout. The wight casts Fear or Command (level 2 version), using the same spellcasting ability as Spellcasting.").unwrap(),
            ["Fear", "Command (level 2 version)"]
        );
        assert!(cast_names("Spellcasting. The mage casts one of the following spells:").is_none());
        assert!(cast_names("Multiattack. The mage makes two attacks, and it can use Spellcasting to cast Light.").is_none());
        let path = "definitions/creatures/adult_white_dragon.md";
        let dragon = Entry::parse(
            path,
            splat::content::definitions::source_file(path).unwrap(),
        )
        .unwrap();
        let presence = dragon
            .body
            .split("\n\n")
            .find(|paragraph| paragraph.starts_with("Frightful Presence."))
            .unwrap();
        let fear = cast_line(&dragon, presence, &[], &index).unwrap();
        assert_eq!(fear.tiles[0].chip.as_ref().unwrap().label, "Fear DC 14");
        // In a run it follows the stat block's actions and its lists, in printed order.
        let order: Vec<String> = ordered_actions(&dragon, Some(&index))
            .into_iter()
            .map(|c| c.label)
            .collect();
        assert_eq!(
            order.last().map(String::as_str),
            Some("Fear DC 14"),
            "{order:?}"
        );
        // `casts the Slow spell` names Slow; a feature named Spellcasting that casts a spell
        // itself, with no list after it, is a cast line.
        assert_eq!(
            cast_names("Slow (Recharge 5–6). The golem casts the Slow spell, requiring no spell components (spell save DC 17).").unwrap(),
            ["Slow"]
        );
        let hellfire = "Hellfire Spellcasting (3/Day). The fiend casts Fireball (level 5 version) twice, requiring no Material components and using Charisma as the spellcasting ability (spell save DC 21).";
        assert!(!spellcasting(hellfire));
        assert_eq!(
            cast_names(hellfire).unwrap(),
            ["Fireball (level 5 version)"]
        );
        for (creature, spell) in [
            ("stone_golem", "Slow DC 17"),
            ("pit_fiend", "Fireball DC 21"),
        ] {
            let path = format!("definitions/creatures/{creature}.md");
            let entry = Entry::parse(
                &path,
                splat::content::definitions::source_file(&path).unwrap(),
            )
            .unwrap();
            let order: Vec<String> = ordered_actions(&entry, Some(&index))
                .into_iter()
                .map(|c| c.label)
                .collect();
            assert!(
                order.iter().any(|label| label == spell),
                "{creature}: {order:?}"
            );
        }
        // A tile's facts give the area of the spell's text.
        let fireball = line("1/Day: Fireball", numbers, &[], &index).unwrap();
        assert_eq!(
            fireball.tiles[0].facts,
            "Level 3 · Action · 150 feet · 20-ft-radius sphere · half on a success"
        );
        // Only a label that says how often is a list of spells.
        assert!(list_line("1/Day Each (Elder Only): Fly"));
        assert!(list_line("Cantrips (at will): fire bolt, light"));
        assert!(list_line("3rd level (2 slots): fireball"));
        assert!(!list_line("**Environments:** Forest, Hill"));
        assert!(!list_line(
            "Original layout: [[../../srd-5.2.1/pages/280|page 280]]."
        ));
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
