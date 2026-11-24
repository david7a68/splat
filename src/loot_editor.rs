//! The loot list editor, for an encounter and for one creature's Changes. Each line is a row.
//! The add field takes the typed words with Enter, and offers the campaign's roll tables and
//! Library items while the DM types. A roll names its table from a list, so a key that no
//! table has shows on its row before play.
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use crate::roll_table::Table;
use eframe::egui;
use splat::content::loot::{Loot, MAX_ROLLS};
use std::sync::Arc;

pub use splat_campaign::loot::{Item, Sources};

/// The most suggestions of each kind under the add field.
pub const MOST_TABLES: usize = 4;
const MOST_ITEMS: usize = 6;
pub const NO_SEPARATOR: &str =
    "Use one row for each item. A ; separates items in the roster table.";

/// Tables, then items, that match what the DM typed. `Roll` before a word looks for tables
/// only.
pub fn matching<'a>(sources: &'a Sources, typed: &str) -> (Vec<&'a Table>, Vec<&'a Item>) {
    let lower = typed.trim().to_lowercase();
    let (tables_only, query) = match lower.strip_prefix("roll") {
        Some(rest) if rest.is_empty() || rest.starts_with(' ') => {
            let rest = rest.trim();
            // `roll 2× gems` looks for `gems`.
            let rest = rest
                .split_once(' ')
                .filter(|(count, _)| count.trim_end_matches(['×', 'x']).parse::<u32>().is_ok())
                .map_or(rest, |(_, key)| key.trim_start_matches(['×', 'x', ' ']));
            (true, rest.to_owned())
        }
        _ => (false, lower),
    };
    let tables = sources
        .tables
        .iter()
        .filter(|table| table.key.to_lowercase().contains(&query))
        .take(MOST_TABLES)
        .collect();
    let items = if tables_only || query.is_empty() {
        Vec::new()
    } else {
        let mut found: Vec<(u8, &Item)> = sources
            .items
            .iter()
            .filter_map(|item| {
                crate::matching::name_rank(&item.lower, &query).map(|rank| (rank, item))
            })
            .collect();
        found.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.name.cmp(&b.1.name)));
        found
            .into_iter()
            .take(MOST_ITEMS)
            .map(|(_, item)| item)
            .collect()
    };
    (tables, items)
}

/// The campaign's sources, from the index every page shares. The index keeps them, so they
/// are read once for each load of the campaign.
pub fn sources(ctx: &egui::Context) -> Option<Arc<Sources>> {
    ctx.data(|data| {
        data.get_temp::<Arc<crate::lookup::ContentIndex>>(egui::Id::new("reader_link_index"))
    })
    .map(|index| index.loot_sources())
}

/// The table keys in one creature's loot that cannot roll, when the sources are known.
pub fn missing_in(ctx: &egui::Context, loot: Option<&str>) -> Vec<String> {
    match (sources(ctx), loot) {
        (Some(sources), Some(loot)) => {
            let parts = splat::content::loot::parts(loot);
            sources.missing(parts.iter().map(String::as_str))
        }
        _ => Vec::new(),
    }
}

/// The words a link line shows: `[[Items/Potion|Potion of Healing]]` reads `Potion of Healing`.
pub fn shown(line: &str) -> Option<&str> {
    let line = line.trim();
    if let Some(inner) = line.strip_prefix("[[").and_then(|l| l.strip_suffix("]]")) {
        return Some(inner.rsplit('|').next().unwrap_or(inner));
    }
    let inner = line.strip_prefix('[')?;
    let (name, rest) = inner.split_once("](")?;
    rest.ends_with(')').then_some(name)
}

/// Where a wiki link line goes: `[[Items/Potion|Potion of Healing]]` goes to `Items/Potion`.
pub fn link_target(line: &str) -> Option<&str> {
    let inner = line.trim().strip_prefix("[[")?.strip_suffix("]]")?;
    Some(inner.split('|').next().unwrap_or(inner))
}

/// The line in plain words, for a summary.
pub fn plain(line: &str) -> String {
    match Loot::parse(line) {
        Loot::Roll { times: 1, key } => format!("a roll on {key}"),
        Loot::Roll { times, key } => format!("{times} rolls on {key}"),
        Loot::Text(text) => shown(&text).unwrap_or(&text).to_owned(),
    }
}

pub const NO_TABLES: &str =
    "No roll tables yet. Put <!-- splat:rolls key=gems dice=1d6 --> above a table in a World note.";

pub const ROLL_MENU: &str = "Roll on a table";

/// The count typed after `Roll`, such as 2 in `Roll 2× ge`.
fn typed_count(typed: &str) -> Option<u32> {
    let mut words = typed.split_whitespace();
    let roll = words
        .next()
        .is_some_and(|word| word.eq_ignore_ascii_case("roll"));
    words
        .next()
        .filter(|_| roll)
        .and_then(|word| word.trim_end_matches(['×', 'x', 'X']).parse().ok())
}

/// The count to keep when the DM picks a table, at most `MAX_ROLLS`.
pub fn typed_times(typed: &str) -> u32 {
    typed_count(typed).map_or(1, |times| times.clamp(1, MAX_ROLLS))
}

/// A roll with a count above what one line can hold.
pub fn too_many(typed: &str) -> bool {
    typed_count(typed).is_some_and(|times| times > MAX_ROLLS)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sources() -> Sources {
        Sources {
            tables: vec![
                Table {
                    key: "gems".into(),
                    dice: "1d4".into(),
                    path: "Treasure.md".into(),
                    heading: None,
                    per: Default::default(),
                    problem: None,
                    rows: Vec::new(),
                    markup: Vec::new(),
                },
                Table {
                    key: "broken".into(),
                    dice: "1d6".into(),
                    path: "Treasure.md".into(),
                    heading: None,
                    per: Default::default(),
                    problem: Some("1d6 needs a row for 3.".into()),
                    rows: Vec::new(),
                    markup: Vec::new(),
                },
            ],
            items: vec![Item {
                name: "Potion of Healing".into(),
                lower: "potion of healing".into(),
                link: "[[Potion of Healing]]".into(),
                detail: "Potion".into(),
            }],
        }
    }

    #[test]
    fn typed_words_find_tables_and_items_and_roll_finds_tables_only() {
        let sources = sources();
        let (tables, items) = matching(&sources, "po");
        assert!(tables.is_empty());
        assert_eq!(items[0].name, "Potion of Healing");
        let (tables, items) = matching(&sources, "Roll 2× ge");
        assert_eq!(tables[0].key, "gems");
        assert!(items.is_empty());
        let (tables, _) = matching(&sources, "roll");
        assert_eq!(tables.len(), 2);
    }

    #[test]
    fn a_key_with_no_table_or_a_bad_table_cannot_roll() {
        let sources = sources();
        assert_eq!(
            sources.missing([
                "Roll gems",
                "Roll 2× coins",
                "Roll broken",
                "30 gp",
                "Roll coins"
            ]),
            ["coins", "broken"]
        );
        assert!(sources.problem("coins").unwrap().contains("No roll table"));
    }

    #[test]
    fn the_first_table_with_a_key_decides_it_and_a_bad_one_says_why() {
        let good = "<!-- splat:rolls key=gems dice=1d2 -->\n\n| d2 | Gem |\n|---|---|\n| 1 | Ruby |\n| 2 | Opal |\n";
        let bad = good.replace("| 2 | Opal |\n", "");
        let sources = Sources {
            tables: Sources::tables_from([("A.md", good), ("B.md", bad.as_str())]),
            items: Vec::new(),
        };
        let roll = sources.roll("gems").unwrap();
        assert_eq!((roll.dice.as_str(), roll.rows.len()), ("1d2", 2));
        let sources = Sources {
            tables: Sources::tables_from([("B.md", bad.as_str()), ("A.md", good)]),
            items: Vec::new(),
        };
        assert!(sources.roll("gems").unwrap_err().starts_with("B.md: "));
        assert!(sources.roll("coins").unwrap_err().contains("key “coins”."));
    }

    #[test]
    fn link_lines_show_their_names() {
        assert_eq!(
            shown("[[Items/Potion|Potion of Healing]]"),
            Some("Potion of Healing")
        );
        assert_eq!(shown("[[Potion]]"), Some("Potion"));
        assert_eq!(
            shown("[Potion](splat:dnd2024:items/potion)"),
            Some("Potion")
        );
        assert_eq!(shown("30 gp"), None);
        assert_eq!(
            link_target("[[Items/Potion|Potion of Healing]]"),
            Some("Items/Potion")
        );
        assert_eq!(link_target("[Potion](splat:dnd2024:items/potion)"), None);
        assert_eq!(plain("Roll 2× gems"), "2 rolls on gems");
        assert_eq!(plain("[[Potion]]"), "Potion");
        assert_eq!(typed_times("Roll 2× ge"), 2);
        assert_eq!(typed_times("roll 3x"), 3);
        assert_eq!(typed_times("gems"), 1);
        assert_eq!(typed_times("Roll 99 gems"), MAX_ROLLS);
        assert!(too_many("Roll 99 gems") && !too_many("Roll 2 gems") && !too_many("gems"));
    }
}
