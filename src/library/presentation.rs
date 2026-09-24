//! Library navigation and reading order, independent of note storage.
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use splat::content::Entry;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(super) enum Section {
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

pub(super) fn category(entry: &Entry) -> &str {
    entry
        .fields
        .get("splat_conversion")
        .and_then(|meta| meta.get("category"))
        .and_then(|value| value.as_str())
        .unwrap_or_else(|| entry.category())
}

pub(super) fn subtitle(entry: &Entry) -> String {
    let category = category(entry).replace('_', " ");
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
    let kind = if entry.kind == "reference" {
        "Reference"
    } else {
        "Definition"
    };
    if context.eq_ignore_ascii_case(&entry.name) {
        format!("{kind} · {category}")
    } else {
        format!("{kind} · {category} · {context}")
    }
}

/// Class scope already owns the feature's class and required level.
pub(super) fn class_level(entry: &Entry) -> Option<String> {
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

pub(super) struct ReadingSection<'a> {
    pub title: String,
    pub body: &'a str,
}

/// The embedded SRD preserves printed column wraps for provenance. Reflow only
/// its reading view; authored Markdown and the stored source are never rewritten.
pub(super) fn overview_markdown(source: &str) -> String {
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
    // Remove a printed hyphen only when the intact word is attested elsewhere
    // in this same source. Retain compounds such as "half-elf" and unknown words.
    static WORDS: std::sync::OnceLock<std::collections::HashSet<String>> =
        std::sync::OnceLock::new();
    let words = WORDS.get_or_init(|| {
        let source = splat::content::builtin::source_file("source.txt").unwrap_or_default();
        String::from_utf8_lossy(source)
            .split(|c: char| !c.is_alphabetic() && c != '-')
            .filter(|s| !s.is_empty())
            .map(str::to_lowercase)
            .collect()
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
    !left.is_empty()
        && !right.is_empty()
        && words.contains(&format!("{left}{right}").to_lowercase())
        && !words.contains(&format!("{left}-{right}").to_lowercase())
}

/// Split at real Markdown headings; headings in fenced examples remain content.
/// Keep every section, including custom homebrew prose and structured tables.
pub(super) fn reading_sections<'a>(body: &'a str, name: &str) -> Vec<ReadingSection<'a>> {
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
