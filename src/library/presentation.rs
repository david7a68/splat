//! Library navigation and reading order, independent of note storage.
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use splat::content::Entry;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(crate) enum Section {
    #[default]
    All,
    Creatures,
    Spells,
    Items,
    Characters,
    Rules,
    Glossary,
    Encounters,
    Other,
}

impl Section {
    pub const GROUPS: &[(&str, &[Self])] = &[
        ("", &[Self::All]),
        (
            "Game content",
            &[Self::Creatures, Self::Spells, Self::Items],
        ),
        ("Characters", &[Self::Characters]),
        ("Game rules", &[Self::Rules, Self::Glossary]),
        ("Preparation", &[Self::Encounters]),
        ("Package", &[Self::Other]),
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::All => "All entries",
            Self::Creatures => "Creatures",
            Self::Spells => "Spells",
            Self::Items => "Items",
            Self::Characters => "Character options",
            Self::Rules => "Rules",
            Self::Glossary => "Glossary",
            Self::Encounters => "Encounters",
            Self::Other => "Other entries",
        }
    }

    pub fn for_entry(entry: &Entry) -> Self {
        match category(entry) {
            "creature" => Self::Creatures,
            "spell" => Self::Spells,
            "item" | "magic_item" | "equipment" | "container" => Self::Items,
            "class" | "subclass" | "class_feature" | "species" | "background" | "feat" => {
                Self::Characters
            }
            "glossary" => Self::Glossary,
            "rule" | "rule_group" | "effect" | "table" => Self::Rules,
            "encounter" | "sequence" | "strategy" | "seed" => Self::Encounters,
            _ => Self::Other,
        }
    }
}

pub(crate) fn category(entry: &Entry) -> &str {
    entry
        .fields
        .get("splat_conversion")
        .and_then(|meta| meta.get("category"))
        .and_then(|value| value.as_str())
        .unwrap_or_else(|| entry.category())
}

/// A list's second line. Everyday words and existing facts only: never schema terms.
pub(crate) fn subtitle(entry: &Entry) -> String {
    if entry.kind == "package" {
        return "Package manifest".into();
    }
    let category = category(entry).replace('_', " ");
    let mut category = {
        let mut letters = category.chars();
        letters.next().map_or_else(String::new, |first| {
            first.to_uppercase().collect::<String>() + letters.as_str()
        })
    };
    if let Some(fact) = list_fact(entry) {
        category.push_str(" · ");
        category.push_str(&fact);
    }
    let parent = entry
        .fields
        .get("splat_conversion")
        .and_then(|meta| meta.get("parent_name"))
        .and_then(|value| value.as_str());
    let context = parent.map(str::to_owned).unwrap_or_else(|| {
        std::path::Path::new(&entry.path)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .replace('_', " ")
    });
    // A file named by its identity says nothing to a reader.
    if context.eq_ignore_ascii_case(&entry.name) || uuid::Uuid::parse_str(&context).is_ok() {
        category
    } else {
        format!("{category} · {context}")
    }
}

/// The one fact a DM scans for: CR, spell level, or item rarity. Nothing is invented.
fn list_fact(entry: &Entry) -> Option<String> {
    use serde_yaml_ng::Value;
    if let Some(creature) = entry.fields.get("splat_creature") {
        let cr = creature.get("cr_text").and_then(Value::as_str)?;
        let cr = cr.split_whitespace().next()?;
        return Some(format!("CR {cr}"));
    }
    if let Some(spell) = entry.fields.get("splat_spell") {
        return match spell.get("level").and_then(Value::as_u64)? {
            0 => Some("Cantrip".into()),
            level => Some(format!("Level {level}")),
        };
    }
    let rarity = entry.fields.get("splat_item")?.get("rarity")?.as_str()?;
    Some(rarity.replace('_', " "))
}

/// Class scope already owns the feature's class and required level.
pub(crate) fn class_level(entry: &Entry) -> Option<String> {
    let scope = entry
        .fields
        .get("splat_rule")?
        .get("parameters")?
        .get("class_scope")?;
    let class = scope.get("class")?.as_str()?;
    let level = scope.get("minimum_level")?.as_u64()?;
    let mut letters = class.chars();
    let name = format!("{}{}", letters.next()?.to_uppercase(), letters.as_str());
    Some(format!("{name} · Level {level}"))
}

pub(crate) struct ReadingSection<'a> {
    pub title: String,
    pub body: &'a str,
}

/// The embedded SRD preserves printed column wraps for provenance. Reflow only
/// its reading view; authored Markdown and the stored source are never rewritten.
pub(crate) fn overview_markdown(source: &str) -> String {
    let mut output = String::new();
    let mut row = false;
    let mut lines = source
        .lines()
        .map(|raw| raw.trim().trim_end_matches('\\'))
        .peekable();
    while let Some(line) = lines.next() {
        if line.starts_with("<!-- SRD page ") || line.starts_with("Original layout: [[") {
            continue;
        }
        if line.is_empty() {
            row = false;
            output.push_str("\n\n");
            continue;
        }
        if let Some(columns) = dc_table_columns(line) {
            let mut table = String::new();
            while let Some(row) = lines
                .peek()
                .copied()
                .filter(|row| number_row(row, columns.len()))
            {
                table.push_str("| ");
                table.push_str(&row.split_whitespace().collect::<Vec<_>>().join(" | "));
                table.push_str(" |\n");
                lines.next();
            }
            if !table.is_empty() {
                if !output.is_empty() && !output.ends_with("\n\n") {
                    output.push_str("\n\n");
                }
                output.push_str(&format!(
                    "| {} |\n|{}\n{table}\n",
                    columns.join(" | "),
                    " --- |".repeat(columns.len())
                ));
                row = false;
                continue;
            }
        }
        // The imported text has lost cell boundaries. Recognize known headers
        // rather than guessing that title-case prose starts a printed table.
        let table_heading = source_table_header(line)
            || (source_heading(line) && lines.peek().is_some_and(|next| source_table_header(next)));
        row = source_table_row(line) || (row && line.starts_with(char::is_lowercase));
        let block = line.starts_with('#')
            || line.starts_with("* ")
            || line.starts_with("- ")
            || line
                .split_once(". ")
                .is_some_and(|(title, _)| source_heading(title));
        let stat = table_heading || source_stat_line(line);
        if (block || stat) && !output.ends_with('\n') {
            output.push_str("\n\n");
        } else if !output.is_empty() && !output.ends_with('\n') {
            if output.ends_with('-') {
                if printed_word_break(&output, line) {
                    output.pop();
                }
            } else {
                output.push(' ');
            }
        } else if output.ends_with("  \n") && !stat && !row {
            output.push('\n');
        }
        output.push_str(line);
        if stat
            || (row
                && lines
                    .peek()
                    .is_none_or(|next| !next.starts_with(char::is_lowercase)))
        {
            output.push_str("  \n");
        } else if line.starts_with('#') || line.starts_with("* ") || line.starts_with("- ") {
            output.push_str("\n\n");
        }
    }
    output
}

fn source_heading(line: &str) -> bool {
    line.starts_with(char::is_uppercase)
        && line.ends_with(char::is_alphabetic)
        && line
            .chars()
            .all(|c| c.is_alphabetic() || c.is_whitespace() || c == '-')
        && line.split_whitespace().all(|word| {
            word.starts_with(char::is_uppercase)
                || matches!(word, "a" | "an" | "and" | "for" | "of" | "the" | "to")
        })
}

/// A printed header of difficulty columns, such as `Foraging DC Navigation DC Search DC`.
fn dc_table_columns(line: &str) -> Option<Vec<String>> {
    let names = line.strip_suffix(" DC")?;
    let columns: Vec<String> = names
        .split(" DC ")
        .map(|name| format!("{} DC", name.trim()))
        .collect();
    (columns.len() >= 2
        && line
            .split_whitespace()
            .all(|word| word.starts_with(char::is_uppercase)))
    .then_some(columns)
}

fn number_row(line: &str, columns: usize) -> bool {
    let cells: Vec<&str> = line.split_whitespace().collect();
    cells.len() == columns && cells.iter().all(|cell| cell.parse::<u32>().is_ok())
}

fn source_table_header(line: &str) -> bool {
    matches!(
        line,
        "Ability Make a Check To …"
            | "Ability Make a Save To …"
            | "Ability Attack Type"
            | "Omen For Results That Will Be …"
    )
}

fn source_table_row(line: &str) -> bool {
    line.starts_with("Weal and woe ")
        || line.split_once(' ').is_some_and(|(label, cells)| {
            matches!(
                label,
                "Strength"
                    | "Dexterity"
                    | "Constitution"
                    | "Intelligence"
                    | "Wisdom"
                    | "Charisma"
                    | "Varies"
                    | "Weal"
                    | "Woe"
                    | "Indifference"
            ) && cells.starts_with(char::is_uppercase)
        })
}

fn source_stat_line(line: &str) -> bool {
    let numeric_row = line.split_whitespace().count() > 1
        && line
            .chars()
            .all(|c| c.is_ascii_digit() || c.is_whitespace() || "+−–-.,%".contains(c));
    numeric_row
        || line.starts_with("Casting Time:")
        || line.matches(" DC").count() > 1
        || matches!(
            line.split_whitespace().next().unwrap_or(""),
            "Level"
                | "Range:"
                | "Components:"
                | "Duration:"
                | "AC"
                | "HP"
                | "Speed"
                | "Str"
                | "Int"
                | "Skills"
                | "Senses"
                | "Languages"
                | "CR"
                | "Resistances"
                | "Immunities"
                | "Vulnerabilities"
        )
}

fn printed_word_break(before: &str, after: &str) -> bool {
    static WORDS: std::sync::OnceLock<std::collections::HashSet<String>> =
        std::sync::OnceLock::new();
    let words = WORDS.get_or_init(|| {
        let source = splat::content::builtin::source_file("source.txt").unwrap_or_default();
        attested_words(&String::from_utf8_lossy(source))
    });
    let left = before
        .trim_end_matches('-')
        .rsplit(|c: char| !c.is_alphabetic())
        .next()
        .unwrap_or("");
    let right = after
        .split(|c: char| !c.is_alphabetic())
        .next()
        .unwrap_or("");
    joins_across_break(left, right, words)
}

/// Words the source prints whole. A word that follows a line ending in a hyphen is half
/// of a broken word, so it does not count.
fn attested_words(source: &str) -> std::collections::HashSet<String> {
    let mut words = std::collections::HashSet::new();
    let mut broken = false;
    for line in source.lines() {
        let line = line.trim().trim_end_matches('\\');
        let tokens = line
            .split(|c: char| !c.is_alphabetic() && c != '-')
            .filter(|s| !s.is_empty());
        for (position, token) in tokens.enumerate() {
            if (!broken || position > 0) && !token.ends_with('-') {
                words.insert(token.to_lowercase());
            }
        }
        broken = line.ends_with('-');
    }
    words
}

/// Whether a hyphen at a line break is a printer's, and not part of the word. A compound
/// the source prints, or two real words, keeps its hyphen. Otherwise the hyphen goes.
fn joins_across_break(left: &str, right: &str, words: &std::collections::HashSet<String>) -> bool {
    if left.is_empty() || right.is_empty() || right.starts_with(char::is_uppercase) {
        return false;
    }
    let (left, right) = (left.to_lowercase(), right.to_lowercase());
    if words.contains(&format!("{left}{right}")) {
        return !words.contains(&format!("{left}-{right}"));
    }
    !words.contains(&format!("{left}-{right}"))
        && (!words.contains(&left) || !words.contains(&right))
}

/// Split at real Markdown headings; headings in fenced examples remain content.
/// Keep every section, including custom homebrew prose and structured tables.
pub(crate) fn reading_sections<'a>(body: &'a str, name: &str) -> Vec<ReadingSection<'a>> {
    let body = body.trim_start();
    let body = body.strip_prefix(&format!("# {name}\n")).unwrap_or(body);
    let headings: Vec<_> = Parser::new(body)
        .into_offset_iter()
        .filter_map(|(event, range)| {
            if !matches!(
                event,
                Event::Start(Tag::Heading {
                    level: HeadingLevel::H2,
                    ..
                })
            ) {
                return None;
            }
            let title = Parser::new(&body[range.clone()])
                .filter_map(|event| match event {
                    Event::Text(text) | Event::Code(text) => Some(text.into_string()),
                    _ => None,
                })
                .collect::<String>();
            Some((title, range))
        })
        .collect();
    let mut sections = Vec::new();
    let intro = &body[..headings.first().map_or(body.len(), |h| h.1.start)];
    if !intro.trim().is_empty() {
        sections.push(ReadingSection {
            title: "Overview".into(),
            body: intro,
        });
    }
    for (index, (title, range)) in headings.iter().enumerate() {
        let next = headings.get(index + 1).map_or(body.len(), |h| h.1.start);
        sections.push(ReadingSection {
            title: title.clone(),
            body: &body[range.end..next],
        });
    }
    sections
}

#[cfg(test)]
mod reading_tests {
    use super::*;

    #[test]
    fn a_printed_difficulty_table_reads_as_a_table() {
        let result = overview_markdown(
            "Before.\\\nForaging DC Navigation DC Search DC\\\n20 10 10\\\n10 5 15\\\nAfter.\n",
        );
        assert!(
            result.contains("| Foraging DC | Navigation DC | Search DC |"),
            "{result}"
        );
        assert!(result.contains("| --- | --- | --- |"), "{result}");
        assert!(result.contains("| 10 | 5 | 15 |"), "{result}");
        assert!(result.contains("Before."), "{result}");
        assert!(result.contains("After."), "{result}");
    }

    #[test]
    fn a_header_without_number_rows_stays_prose() {
        let result = overview_markdown("Foraging DC Search DC\nThe rest is prose.\n");
        assert!(!result.contains('|'), "{result}");
        assert!(result.contains("Foraging DC Search DC"), "{result}");
    }

    #[test]
    fn printed_hyphens_go_and_real_compounds_stay() {
        let words = attested_words(
            "half-elf well known festi-\nvals con-\ndenses con shield bearer shield-bearer non\nnonmagical",
        );
        assert!(joins_across_break("festi", "vals", &words));
        assert!(joins_across_break("con", "denses", &words));
        assert!(joins_across_break("non", "magical", &words));
        assert!(!joins_across_break("half", "elf", &words));
        assert!(!joins_across_break("well", "known", &words));
        assert!(!joins_across_break("shield", "bearer", &words));
        assert!(!joins_across_break("Sword", "Coast", &words));
    }
}
