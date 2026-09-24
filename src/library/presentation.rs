//! Library navigation and reading order, independent of note storage.
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
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

pub(super) struct ReadingSection<'a> {
    pub title: String,
    pub body: &'a str,
}

/// The embedded SRD preserves printed column wraps for provenance. Reflow only
/// its reading view; authored Markdown and the stored source are never rewritten.
pub(super) fn overview_markdown(source: &str) -> String {
    let mut output = String::new();
    let mut paragraph = String::new();
    let flush = |paragraph: &mut String, output: &mut String| {
        if !paragraph.is_empty() {
            output.push_str(paragraph);
            output.push_str("\n\n");
            paragraph.clear();
        }
    };
    for raw in source.lines() {
        let line = raw
            .trim()
            .strip_suffix('\\')
            .unwrap_or(raw.trim())
            .trim_end();
        if line.starts_with("<!-- SRD page ") || line.starts_with("Original layout: [[") {
            continue;
        }
        if line.is_empty() {
            flush(&mut paragraph, &mut output);
        } else if line.starts_with('#') || line.starts_with("- ") || line.starts_with("* ") {
            flush(&mut paragraph, &mut output);
            output.push_str(line);
            output.push_str("\n\n");
        } else if source_stat_line(line) {
            flush(&mut paragraph, &mut output);
            // Keep each stat distinct, without preserving the PDF's prose wraps.
            output.push_str(line);
            output.push_str("  \n");
        } else {
            if output.ends_with("  \n") {
                output.push('\n');
            }
            if line.starts_with("Using a Higher-Level Spell Slot.") {
                flush(&mut paragraph, &mut output);
            }
            if !paragraph.is_empty() {
                if paragraph.ends_with('-') {
                    if printed_word_break(&paragraph, line) {
                        paragraph.pop();
                    }
                } else {
                    paragraph.push(' ');
                }
            }
            paragraph.push_str(line);
        }
    }
    flush(&mut paragraph, &mut output);
    output
}

fn source_stat_line(line: &str) -> bool {
    // Extracted tables must keep their rows; they are not prose paragraphs.
    let numeric_row = line.split_whitespace().count() > 1
        && line
            .chars()
            .all(|c| c.is_ascii_digit() || c.is_whitespace() || "+−–-.,%".contains(c));
    numeric_row
        || line.matches(" DC").count() > 1
        || [
            "Level ",
            "Casting Time:",
            "Range:",
            "Components:",
            "Duration:",
            "AC ",
            "HP ",
            "Speed ",
            "Str ",
            "Int ",
            "Skills ",
            "Senses ",
            "Languages ",
            "CR ",
            "Resistances ",
            "Immunities ",
            "Vulnerabilities ",
        ]
        .iter()
        .any(|prefix| line.starts_with(prefix))
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
    let mut headings = Vec::new();
    let mut heading = None;
    for (event, range) in Parser::new(body).into_offset_iter() {
        match event {
            Event::Start(Tag::Heading {
                level: HeadingLevel::H2,
                ..
            }) => {
                heading = Some((String::new(), range.start));
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some((title, _)) = &mut heading {
                    title.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(HeadingLevel::H2)) => {
                if let Some((title, start)) = heading.take() {
                    headings.push((title, start, range.end));
                }
            }
            _ => {}
        }
    }
    let mut sections = Vec::new();
    let intro = &body[..headings.first().map_or(body.len(), |h| h.1)];
    if !intro.trim().is_empty() {
        sections.push(ReadingSection {
            title: "Overview".into(),
            body: intro,
        });
    }
    for (index, (title, _, end)) in headings.iter().enumerate() {
        let next = headings.get(index + 1).map_or(body.len(), |h| h.1);
        sections.push(ReadingSection {
            title: title.clone(),
            body: &body[*end..next],
        });
    }
    sections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imported_prose_reflows_but_keeps_stats_paragraphs_and_compounds() {
        let text = overview_markdown(
            "Range: 30 feet\\\nDuration: 1 minute\\\nWhen-\\\never you make an at-\\\ntack, add a die.\\\n\nA half-\\\nelf waits.\\\n\n### Actions\n\nRead on.\\\n",
        );
        assert!(
            text.contains("Whenever you make an attack, add a die."),
            "{text}"
        );
        assert!(text.contains("A half-elf waits."), "{text}");
        assert!(text.contains("Range: 30 feet  \nDuration: 1 minute"));
        assert!(text.contains("\n\n### Actions\n\nRead on."));
        assert!(!text.contains('\\'));
    }

    #[test]
    fn built_in_subjects_are_classified_without_losing_entries() {
        let entries = splat::content::definitions::entries().unwrap();
        for entry in &entries {
            assert_ne!(Section::for_entry(entry), Section::Other, "{}", entry.path);
        }
        for (name, section) in [
            ("Bless", Section::Spells),
            ("Fighter", Section::Characters),
            ("Ability Check", Section::Glossary),
            ("Aboleth", Section::Creatures),
        ] {
            let entry = entries.iter().find(|e| e.name == name).unwrap();
            assert_eq!(Section::for_entry(entry), section);
        }
        let text =
            std::str::from_utf8(splat::content::builtin::source_file("spell/bless.md").unwrap())
                .unwrap();
        let bless = Entry::parse("spell/bless.md", text).unwrap();
        assert_eq!(Section::for_entry(&bless), Section::Spells);
    }

    #[test]
    fn reading_order_keeps_nested_sections_and_fenced_headings() {
        let body = "# Example\n\nIntro\n\n## Source text\n\nRead me\n### Detail\nKeep me\n\n## Rule\n\n```md\n## Not a section\n```\n\n## Notes\n\nCustom text\n";
        let sections = reading_sections(body, "Example");
        assert_eq!(
            sections
                .iter()
                .map(|s| s.title.as_str())
                .collect::<Vec<_>>(),
            ["Overview", "Source text", "Rule", "Notes"]
        );
        assert!(sections[1].body.contains("### Detail"));
        assert!(sections[2].body.contains("## Not a section"));
        assert!(sections[3].body.contains("Custom text"));
    }
}
