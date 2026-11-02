//! Library navigation and reading order, independent of note storage.
use crate::design;
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
            "encounter" | "encounter_group" | "sequence" | "strategy" | "seed" => Self::Encounters,
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
        return "Package".into();
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
    // Only a real parent. A file's name is not a word for a reader.
    match parent {
        Some(parent) if !parent.eq_ignore_ascii_case(&entry.name) => {
            format!("{category} · {parent}")
        }
        _ => category,
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
    let lines: Vec<&str> = source
        .lines()
        .map(|raw| raw.trim().trim_end_matches('\\'))
        .filter(|line| {
            !line.starts_with("<!-- SRD page ") && !line.starts_with("Original layout: [[")
        })
        .collect();
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        let next = lines.get(index + 1).copied();
        if line.is_empty() {
            row = false;
            if !output.ends_with("\n\n") {
                output.push_str("\n\n");
            }
            index += 1;
            continue;
        }
        // A Markdown table is already structured. Its rows stay lines.
        if line.starts_with('|') {
            if !output.is_empty() && !output.ends_with('\n') {
                output.push_str("\n\n");
            }
            output.push_str(line);
            output.push('\n');
            row = false;
            index += 1;
            continue;
        }
        let finished = previous_finished(&output);
        if finished && let Some((table, used)) = printed_table(&lines[index..]) {
            if !output.is_empty() && !output.ends_with("\n\n") {
                output.push_str("\n\n");
            }
            output.push_str(table.trim_end());
            output.push_str("\n\n");
            row = false;
            index += used;
            continue;
        }
        index += 1;
        if finished && standalone_heading(line, next) {
            if !output.is_empty() && !output.ends_with("\n\n") {
                output.push_str("\n\n");
            }
            output.push_str("### ");
            output.push_str(line);
            output.push_str("\n\n");
            row = false;
            continue;
        }
        // The imported text has lost cell boundaries. Recognize known headers
        // rather than guessing that title-case prose starts a printed table.
        let table_heading = source_table_header(line)
            || (source_heading(line) && next.is_some_and(source_table_header));
        row = source_table_row(line) || (row && line.starts_with(char::is_lowercase));
        let block = line.starts_with('#')
            || line.starts_with("* ")
            || line.starts_with("- ")
            || (finished && design::reader::run_in_title(line).is_some());
        let stat = table_heading || source_stat_line(line);
        if spell_list_line(line) && !output.ends_with('\n') {
            // Its own line, but the same paragraph, whose spells the reader links.
            output.push_str("  \n");
        } else if (block || stat) && !output.ends_with('\n') {
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
        if stat || (row && next.is_none_or(|next| !next.starts_with(char::is_lowercase))) {
            output.push_str("  \n");
        } else if line.starts_with('#') || line.starts_with("* ") || line.starts_with("- ") {
            output.push_str("\n\n");
        }
    }
    output
}

/// Whether the text so far ends a sentence, so the next line may start a new block. A
/// wrapped sentence that happens to bring a capital to a line start must not.
fn previous_finished(output: &str) -> bool {
    output.ends_with('\n')
        || output
            .trim_end()
            .chars()
            .last()
            .is_none_or(|c| ".!?:)\u{201d}\"".contains(c))
}

/// `At Will: Fear`, `1/Day Each: Speak with Dead`: a stat block's list of spells.
fn spell_list_line(line: &str) -> bool {
    line.split_once(':').is_some_and(|(label, _)| {
        label == "At Will"
            || label
                .split_once("/Day")
                .is_some_and(|(n, _)| n.parse::<u32>().is_ok())
    })
}

/// A title on its own line with prose after it, such as a subclass's motto. A title-case
/// line followed by another is more likely the header of a printed table.
fn standalone_heading(line: &str, next: Option<&str>) -> bool {
    const STAT_LABELS: [&str; 8] = [
        "Skills",
        "Senses",
        "Immunities",
        "Resistances",
        "Vulnerabilities",
        "Languages",
        "Gear",
        "Speed",
    ];
    source_heading(line)
        && !STAT_LABELS.contains(&line.split_whitespace().next().unwrap_or(""))
        && !source_table_header(line)
        && !source_table_row(line)
        && line.split_whitespace().count() >= 2
        && line.chars().any(char::is_lowercase)
        && next.is_some_and(|next| {
            next.starts_with(char::is_uppercase)
                && !source_heading(next)
                && !source_table_header(next)
                && design::reader::run_in_title(next).is_none()
        })
}

/// One cell of a printed table: a number, a modifier, dice, a fraction, or a dash.
fn number_cell(cell: &str) -> bool {
    let cell = cell.trim_start_matches(['+', '−', '-', '–']);
    cell.is_empty()
        || cell == "—"
        || (cell.starts_with(|c: char| c.is_ascii_digit())
            && cell
                .chars()
                .all(|c| c.is_ascii_digit() || "d/,.%".contains(c)))
}

/// `2 +2 2`: a row of numbers only.
fn numbers_row(line: &str) -> Option<usize> {
    let cells: Vec<&str> = line.split_whitespace().collect();
    (cells.len() >= 2 && cells.iter().all(|cell| number_cell(cell))).then_some(cells.len())
}

/// `Daylight 1`: a short name, then one number.
fn labelled_row(line: &str) -> Option<(&str, &str)> {
    let (label, value) = line.rsplit_once(' ')?;
    (line.len() <= 48
        && label.starts_with(char::is_uppercase)
        && number_cell(value)
        && value != "—"
        && !label.contains(':')
        && !label.ends_with([',', '.'])
        && !line.starts_with("Str ")
        && !line.starts_with("Int "))
    .then_some((label, value))
}

/// Title-case lines above the rows, up to three: the table's wrapped header. Their cell
/// boundaries are gone, so they are kept as printed.
fn table_caption<'a>(lines: &'a [&'a str]) -> (Vec<&'a str>, usize) {
    let caption: Vec<&str> = lines
        .iter()
        .take(3)
        .take_while(|line| {
            !line.is_empty()
                && labelled_row(line).is_none()
                && numbers_row(line).is_none()
                && line.len() <= 48
                && line
                    .split_whitespace()
                    .all(|word| word.starts_with(|c: char| c.is_uppercase() || c == '('))
        })
        .copied()
        .collect();
    let used = caption.len();
    (caption, used)
}

/// A printed table whose cell boundaries were lost, from these lines' start. Three rows
/// or more, or nothing: fewer could as well be sentences. Returns Markdown and the lines
/// it used.
fn printed_table(lines: &[&str]) -> Option<(String, usize)> {
    if let Some(columns) = lines.first().copied().and_then(dc_table_columns) {
        let rows: Vec<&str> = lines[1..]
            .iter()
            .take_while(|row| numbers_row(row) == Some(columns.len()))
            .copied()
            .collect();
        if !rows.is_empty() {
            let header = columns.iter().map(String::as_str).collect::<Vec<_>>();
            return Some((
                markdown_table(&header, rows.iter().map(|r| r.split_whitespace().collect())),
                rows.len() + 1,
            ));
        }
    }
    let (caption, start) = table_caption(lines);
    let body = &lines[start..];
    let width = body.first().copied().and_then(numbers_row);
    if let Some(width) = width {
        let rows: Vec<&str> = body
            .iter()
            .take_while(|row| numbers_row(row) == Some(width))
            .copied()
            .collect();
        if rows.len() >= 3 {
            let table = markdown_table(
                &vec![""; width],
                rows.iter().map(|r| r.split_whitespace().collect()),
            );
            return Some((with_caption(&caption, table), start + rows.len()));
        }
    }
    let rows: Vec<(&str, &str)> = body.iter().map_while(|row| labelled_row(row)).collect();
    if rows.len() >= 3 {
        let words: Vec<&str> = caption.iter().flat_map(|l| l.split_whitespace()).collect();
        // Two columns, so the last word names the numbers unless it is only a wrapped
        // second line, in which case that line does.
        let table = if caption.is_empty() {
            markdown_table(&["", ""], rows.iter().map(|(l, v)| vec![*l, *v]))
        } else {
            let split = if caption.len() >= 2 {
                caption[0]
                    .split_whitespace()
                    .count()
                    .saturating_sub(1)
                    .max(1)
            } else {
                words.len().saturating_sub(1).max(1)
            };
            let (first, second) = words.split_at(split.min(words.len()));
            let (first, second) = (first.join(" "), second.join(" "));
            markdown_table(&[&first, &second], rows.iter().map(|(l, v)| vec![*l, *v]))
        };
        return Some((table, start + rows.len()));
    }
    None
}

fn with_caption(caption: &[&str], table: String) -> String {
    if caption.is_empty() {
        table
    } else {
        format!("**{}**\n\n{table}", caption.join(" "))
    }
}

fn markdown_table<'a>(header: &[&str], rows: impl Iterator<Item = Vec<&'a str>>) -> String {
    let line = |cells: &[&str]| format!("| {} |\n", cells.join(" | "));
    let mut table = line(header);
    table.push_str(&format!("|{}\n", " --- |".repeat(header.len())));
    for row in rows {
        table.push_str(&line(&row));
    }
    table
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
                | "Ability:"
                | "Utilize:"
                | "Craft:"
                | "Variants:"
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
    use crate::design;

    /// The `## Source text` of every built-in note that the reader shows.
    fn corpus() -> Vec<(String, String)> {
        fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            for entry in std::fs::read_dir(dir).unwrap().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // Whole printed pages are provenance, in two columns. They are not read.
                    if path.file_name().is_some_and(|n| n != "pages") {
                        walk(&path, out);
                    }
                } else if path.extension().is_some_and(|e| e == "md") {
                    out.push(path);
                }
            }
        }
        let mut files = Vec::new();
        walk(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("builtin"),
            &mut files,
        );
        files.sort();
        let notes: Vec<_> = files
            .into_iter()
            .filter_map(|path| {
                let text = std::fs::read_to_string(&path).ok()?;
                let start = text.find("## Source text\n")? + "## Source text\n".len();
                let body = &text[start..];
                let end = body.find("\n## ").unwrap_or(body.len());
                let name = path.strip_prefix(env!("CARGO_MANIFEST_DIR")).ok()?;
                Some((name.display().to_string(), body[..end].to_owned()))
            })
            .collect();
        assert!(
            notes.len() > 2_000,
            "the corpus was not found: {}",
            notes.len()
        );
        notes
    }

    fn letters(text: &str) -> String {
        text.chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect()
    }

    /// Reflowing is only allowed to move line breaks. Every word stays, in order.
    #[test]
    fn across_the_corpus_reflow_keeps_every_word_in_order() {
        let mut lost = Vec::new();
        for (name, source) in corpus() {
            let kept: String = source
                .lines()
                .filter(|l| {
                    !l.starts_with("<!-- SRD page ") && !l.starts_with("Original layout: [[")
                })
                .collect::<Vec<_>>()
                .join("\n");
            let reading = overview_markdown(&source);
            if letters(&kept) != letters(&reading) {
                lost.push(name);
            }
        }
        assert!(
            lost.is_empty(),
            "{} notes changed: {:?}",
            lost.len(),
            &lost[..lost.len().min(8)]
        );
    }

    /// A named action with a qualifier starts its own paragraph, however it was wrapped.
    /// The pattern is written out here, apart from the code it checks.
    #[test]
    fn across_the_corpus_named_actions_start_their_own_paragraph() {
        let qualified = |text: &str| {
            text.split_once(" (").is_some_and(|(name, rest)| {
                !name.is_empty()
                    && name.len() < 40
                    && name.starts_with(char::is_uppercase)
                    && name
                        .chars()
                        .all(|c| c.is_alphabetic() || " '\u{2019}-".contains(c))
                    && ["Recharge", "1/Day", "2/Day", "3/Day", "Costs"]
                        .iter()
                        .any(|q| rest.starts_with(q))
                    && rest
                        .split_once(')')
                        .is_some_and(|(_, after)| after.starts_with(". "))
            })
        };
        let (mut seen, mut merged) = (0, Vec::new());
        for (name, source) in corpus() {
            let reading = overview_markdown(&source);
            let mut sentence_ended = true;
            for line in source.lines().map(|l| l.trim().trim_end_matches('\\')) {
                if sentence_ended && qualified(line) {
                    seen += 1;
                    let start = reading.find(line.split_once(" (").unwrap().0);
                    let own = start.is_some_and(|at| {
                        at == 0 || reading[..at].ends_with('\n') || !reading[..at].ends_with(". ")
                    });
                    if !own {
                        merged.push(format!("{name}: {line}"));
                    }
                }
                if !line.is_empty() {
                    sentence_ended = line.ends_with(['.', ':', ')', '!', '?']);
                }
            }
        }
        assert!(seen > 50, "the pattern found only {seen} actions");
        assert!(
            merged.is_empty(),
            "{} merged: {:?}",
            merged.len(),
            &merged[..merged.len().min(8)]
        );
    }

    /// A printed row of numbers is never swallowed by a sentence.
    #[test]
    fn across_the_corpus_number_rows_stay_rows() {
        let mut merged = Vec::new();
        let mut seen = 0;
        for (name, source) in corpus() {
            let reading = overview_markdown(&source);
            let rows: Vec<String> = reading
                .lines()
                .map(|line| line.replace('|', " "))
                .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
                .collect();
            for line in source.lines().map(|l| l.trim().trim_end_matches('\\')) {
                let line = line.replace('|', " ");
                let cells: Vec<&str> = line.split_whitespace().collect();
                if cells.len() >= 3 && cells.iter().all(|c| c.chars().all(|c| c.is_ascii_digit())) {
                    seen += 1;
                    if !rows.iter().any(|row| row == &cells.join(" ")) {
                        merged.push(format!("{name}: {line}"));
                    }
                }
            }
        }
        assert!(seen > 10, "the pattern found only {seen} rows");
        assert!(
            merged.is_empty(),
            "{} merged: {:?}",
            merged.len(),
            &merged[..merged.len().min(8)]
        );
    }

    /// Bold is for a title. A whole opening sentence is not one.
    #[test]
    fn across_the_corpus_only_titles_turn_bold() {
        let mut wrong = Vec::new();
        for (name, source) in corpus() {
            for paragraph in overview_markdown(&source).split("\n\n") {
                // A break inside the markers would print them as stars.
                let bold = design::reader::run_in_markdown(paragraph);
                if bold.starts_with("**\n") || bold.starts_with("** ") {
                    wrong.push(format!("{name}: {bold}"));
                }
                if let Some(title) = design::reader::run_in_title(paragraph)
                    && (title.len() >= 65
                        || title
                            .split_whitespace()
                            .filter(|w| w.starts_with(char::is_lowercase))
                            .count()
                            > 3)
                {
                    wrong.push(format!("{name}: {title}"));
                }
            }
        }
        assert!(
            wrong.is_empty(),
            "{} wrong: {:?}",
            wrong.len(),
            &wrong[..wrong.len().min(8)]
        );
    }

    fn real_note(path: &str) -> String {
        let text =
            std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path))
                .unwrap();
        let start = text.find("## Source text\n").map_or(0, |at| at + 15);
        let body = &text[start..];
        body[..body.find("\n## ").unwrap_or(body.len())].to_owned()
    }

    #[test]
    fn a_real_stat_block_keeps_its_actions_apart() {
        let source = std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("builtin/dnd2024-definitions/creatures/adult_black_dragon.md"),
        )
        .unwrap();
        let reading = overview_markdown(&source);
        for start in [
            "Legendary Resistance (3/Day, or 4/Day in Lair). If the dragon",
            "Rend. Melee Attack Roll",
            "Acid Breath (Recharge 5\u{2013}6). Dexterity Saving Throw",
            "Spellcasting. The dragon casts",
        ] {
            assert!(
                reading.contains(&format!("\n\n{start}")),
                "{start}\n{reading}"
            );
        }
        assert!(
            reading.contains("attacks):\n\nAt Will: Acid Arrow"),
            "{reading}"
        );
        // The last spell on a list has no full stop, and the next list still starts a line.
        assert!(reading.contains("Fear  \n1/Day Each:"), "{reading}");
    }

    #[test]
    fn a_table_with_a_wrapped_header_reads_as_a_table() {
        let reading = overview_markdown(&real_note(
            "builtin/dnd2024-definitions/items/dragon_orb.md",
        ));
        assert!(reading.contains("| Spell | Charge Cost |"), "{reading}");
        assert!(
            reading.contains("| Cure Wounds (level 9 version) | 4 |"),
            "{reading}"
        );
        assert!(
            reading.contains("| Scrying (save DC 18) | 3 |"),
            "{reading}"
        );
        assert!(reading.contains("\n\nCall Dragons. While you"), "{reading}");
    }

    #[test]
    fn a_table_across_both_columns_stays_with_its_entry() {
        let reading = overview_markdown(&real_note(
            "builtin/dnd2024-definitions/items/apparatus_of_the_crab.md",
        ));
        assert!(reading.contains("| Lever | Up | Down |"), "{reading}");
        assert!(
            reading.contains(
                "| 1 | Legs extend, allowing the apparatus to walk and swim. | Legs retract,"
            ),
            "{reading}"
        );
        let next = overview_markdown(&real_note(
            "builtin/dnd2024-definitions/items/armor_of_resistance.md",
        ));
        assert!(
            next.contains("| 1d10 | Damage Type | 1d10 | Damage Type |"),
            "{next}"
        );
        assert!(!next.contains("Lever"), "{next}");
    }

    #[test]
    fn corrected_printing_errors_reach_the_reader() {
        let spell = overview_markdown(&real_note(
            "builtin/dnd2024-definitions/spells/telekinesis.md",
        ));
        assert!(
            spell.contains("or pouring the contents from a vial."),
            "{spell}"
        );
        let dragon = overview_markdown(&real_note(
            "builtin/dnd2024-definitions/creatures/young_white_dragon.md",
        ));
        assert!(dragon.contains("Int 6 −2 −2 Wis 11"), "{dragon}");
    }

    #[test]
    fn a_motto_on_its_own_line_is_a_heading() {
        let reading = overview_markdown(&real_note(
            "builtin/dnd2024-definitions/subclasses/bard_college_of_lore.md",
        ));
        assert!(
            reading
                .trim_start()
                .starts_with("### Plumb the Depths of Magical Knowledge\n\nBards"),
            "{reading}"
        );
    }

    #[test]
    fn run_in_titles_are_titles() {
        use design::reader::run_in_markdown as bold;
        assert_eq!(bold("Rend. Melee"), "**Rend.** Melee");
        assert_eq!(bold("\nRend. Melee"), "**Rend.** Melee");
        assert_eq!(
            bold("Acid Breath (Recharge 5\u{2013}6). Dexterity"),
            "**Acid Breath (Recharge 5\u{2013}6).** Dexterity"
        );
        assert_eq!(
            bold("Strength or Dexterity. These"),
            "**Strength or Dexterity.** These"
        );
        let sentence = "An orb is an etched crystal globe about 10 inches in diameter. When";
        assert_eq!(bold(sentence), sentence);
        assert_eq!(bold("Half damage. Then"), "Half damage. Then");
        assert_eq!(bold("Hit: 13 damage. Then"), "Hit: 13 damage. Then");
    }

    #[test]
    fn a_wrapped_sentence_does_not_start_a_paragraph_at_a_capital() {
        let reading = overview_markdown("the spell ends on the\nDragon. It then falls.\n");
        assert_eq!(reading, "the spell ends on the Dragon. It then falls.");
    }

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
