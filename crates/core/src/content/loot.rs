//! Loot: what the party can take after a fight. An encounter keeps its loot in a `## Loot`
//! list in its note, and a roster row or group member keeps its own in its Changes. A line
//! is plain words, or a roll on one of the campaign's marked roll tables by its key:
//! `Roll trinkets`, or `Roll 2× trinkets` for two rolls.

/// The heading of an encounter's loot list.
pub const HEADING: &str = "Loot";
/// The most rolls one line can ask for.
pub const MAX_ROLLS: u32 = 20;
/// What separates loot in one Changes cell.
pub const SEPARATOR: char = ';';

/// One line of loot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Loot {
    /// Words the DM reads out, such as `30 gp` or a link to an item.
    Text(String),
    /// Roll this many times on the campaign's roll table with this key.
    Roll { times: u32, key: String },
}

impl Loot {
    /// Read one line. A line that is not a well-formed roll is words, so nothing is lost.
    pub fn parse(line: &str) -> Self {
        let line = line.trim();
        let words: Vec<&str> = line.split_whitespace().collect();
        let roll = match words.as_slice() {
            [first, key] if first.eq_ignore_ascii_case("roll") => Some((1, *key)),
            [first, times, key] if first.eq_ignore_ascii_case("roll") => {
                times_of(times).map(|times| (times, *key))
            }
            [first, times, cross, key]
                if first.eq_ignore_ascii_case("roll") && matches!(*cross, "×" | "x" | "X") =>
            {
                times.parse().ok().map(|times| (times, *key))
            }
            _ => None,
        };
        match roll {
            Some((times, key)) if (1..=MAX_ROLLS).contains(&times) => Self::Roll {
                times,
                key: key.to_owned(),
            },
            _ => Self::Text(line.to_owned()),
        }
    }

    /// The line as a note spells it.
    pub fn line(&self) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Roll { times: 1, key } => format!("Roll {key}"),
            Self::Roll { times, key } => format!("Roll {times}× {key}"),
        }
    }
}

/// `2×` or `2x`, as a count of rolls.
fn times_of(word: &str) -> Option<u32> {
    word.strip_suffix('×')
        .or_else(|| word.strip_suffix(['x', 'X']))?
        .parse()
        .ok()
}

/// The loot in one Changes cell or override, separated by semicolons.
pub fn split(text: &str) -> Vec<Loot> {
    text.split(SEPARATOR)
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(Loot::parse)
        .collect()
}

/// Each part of one creature's loot, as written.
pub fn parts(text: &str) -> Vec<String> {
    text.split(SEPARATOR)
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_owned)
        .collect()
}

/// Split a Changes cell at its loot. Loot comes last, because its words can hold commas:
/// `HP +5, Loot Iron key; Roll goblin-pockets`.
pub fn split_changes(text: &str) -> (&str, Option<&str>) {
    let mut start = 0;
    loop {
        let term = &text[start..];
        let trimmed = term.trim_start();
        let at = start + term.len() - trimmed.len();
        if trimmed
            .get(..5)
            .is_some_and(|word| word.eq_ignore_ascii_case("loot "))
        {
            let before = text[..at].trim_end().trim_end_matches(',').trim_end();
            return (before, Some(trimmed[5..].trim()));
        }
        match term.find(',') {
            Some(comma) => start += comma + 1,
            None => return (text, None),
        }
    }
}

/// One line of a note, as the section scanner reads it.
struct Line<'a> {
    start: usize,
    end: usize,
    text: &'a str,
    fenced: bool,
}

fn lines(body: &str) -> Vec<Line<'_>> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut fence = false;
    for raw in body.split_inclusive('\n') {
        let text = raw.trim_end_matches(['\n', '\r']);
        let opens = text.trim_start().starts_with("```") || text.trim_start().starts_with("~~~");
        out.push(Line {
            start,
            end: start + raw.len(),
            text,
            fenced: fence || opens,
        });
        if opens {
            fence = !fence;
        }
        start += raw.len();
    }
    out
}

/// The level of a Markdown heading line, if it is one.
fn heading_level(text: &str) -> Option<usize> {
    let hashes = text.chars().take_while(|c| *c == '#').count();
    (1..=6)
        .contains(&hashes)
        .then(|| text[hashes..].starts_with(' ').then_some(hashes))
        .flatten()
}

/// Where the `## Loot` section is, heading included, and where its own lines start.
fn section(body: &str) -> Option<(std::ops::Range<usize>, usize)> {
    let lines = lines(body);
    let at = lines.iter().position(|line| {
        !line.fenced
            && heading_level(line.text) == Some(2)
            && line.text[2..]
                .trim()
                .trim_end_matches('#')
                .trim()
                .eq_ignore_ascii_case(HEADING)
    })?;
    let end = lines[at + 1..]
        .iter()
        .find(|line| !line.fenced && heading_level(line.text).is_some_and(|level| level <= 2))
        .map_or(body.len(), |line| line.start);
    Some((lines[at].start..end, lines[at].end))
}

fn item(text: &str) -> Option<&str> {
    let trimmed = text.trim_start();
    ["- ", "* ", "+ "]
        .iter()
        .find_map(|bullet| trimmed.strip_prefix(bullet))
        .map(str::trim)
}

/// Each top-level list item in a note's `## Loot` section, as written.
pub fn items(body: &str) -> Vec<String> {
    let Some((range, first)) = section(body) else {
        return Vec::new();
    };
    lines(&body[first..range.end])
        .into_iter()
        .filter(|line| !line.fenced && !line.text.starts_with([' ', '\t']))
        .filter_map(|line| item(line.text))
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
        .collect()
}

/// Whether the section holds only one-line list items, so a form can own it and lose nothing.
pub fn only_items(body: &str) -> bool {
    let Some((range, first)) = section(body) else {
        return true;
    };
    lines(&body[first..range.end]).iter().all(|line| {
        line.text.trim().is_empty()
            || (!line.text.starts_with([' ', '\t']) && item(line.text).is_some())
    })
}

/// The note without its `## Loot` section.
pub fn without(body: &str) -> String {
    match section(body) {
        Some((range, _)) => {
            let mut out = body[..range.start].trim_end().to_owned();
            let rest = body[range.end..].trim();
            if !rest.is_empty() {
                if !out.is_empty() {
                    out.push_str("\n\n");
                }
                out.push_str(rest);
            }
            out
        }
        None => body.trim_end().to_owned(),
    }
}

/// Whether the note has a `## Loot` section.
pub fn has_section(body: &str) -> bool {
    section(body).is_some()
}

/// The heading line right after the `## Loot` section, so a rewrite can put it back there.
pub fn heading_after(body: &str) -> Option<String> {
    let (range, _) = section(body)?;
    let rest = &body[range.end..];
    let line = rest.lines().next()?.trim();
    heading_level(line).map(|_| line.to_owned())
}

/// The note with these loot lines as its `## Loot` section. It goes before the heading line
/// `before`, where the section was, or at the end. No lines, no section.
pub fn with(body: &str, loot: &[String], before: Option<&str>) -> String {
    let rest = without(body);
    let loot: Vec<_> = loot
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    if loot.is_empty() {
        return rest;
    }
    let mut section = format!("## {HEADING}\n\n");
    for line in loot {
        section.push_str(&format!("- {line}\n"));
    }
    let at = before.and_then(|before| {
        lines(&rest)
            .into_iter()
            .find(|line| !line.fenced && line.text.trim() == before)
            .map(|line| line.start)
    });
    match at {
        Some(at) => format!("{}{section}\n{}", &rest[..at], &rest[at..]),
        None if rest.is_empty() => section,
        None => format!("{rest}\n\n{section}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_is_a_roll_only_when_it_names_one_key() {
        let roll = |times, key: &str| Loot::Roll {
            times,
            key: key.into(),
        };
        assert_eq!(Loot::parse("Roll trinkets"), roll(1, "trinkets"));
        assert_eq!(
            Loot::parse("roll 2× goblin-pockets"),
            roll(2, "goblin-pockets")
        );
        assert_eq!(Loot::parse("Roll 3x gems"), roll(3, "gems"));
        assert_eq!(Loot::parse("Roll 4 × gems"), roll(4, "gems"));
        for text in [
            "30 gp",
            "Roll",
            "Roll for the gate",
            "Roll 0× gems",
            "Roll 21× gems",
        ] {
            assert_eq!(Loot::parse(text), Loot::Text(text.into()), "{text}");
        }
        assert_eq!(roll(2, "gems").line(), "Roll 2× gems");
        assert_eq!(roll(1, "gems").line(), "Roll gems");
    }

    #[test]
    fn loot_is_the_last_term_of_a_changes_cell_and_keeps_its_commas() {
        assert_eq!(
            split_changes("HP +5, Loot Iron key, rusty; Roll gems"),
            ("HP +5", Some("Iron key, rusty; Roll gems"))
        );
        assert_eq!(split_changes("loot 30 gp"), ("", Some("30 gp")));
        assert_eq!(split_changes("HP +5, Prone"), ("HP +5, Prone", None));
        assert_eq!(split_changes("Lootless"), ("Lootless", None));
        assert_eq!(
            split("Iron key, rusty; Roll gems; "),
            vec![
                Loot::Text("Iron key, rusty".into()),
                Loot::Roll {
                    times: 1,
                    key: "gems".into()
                }
            ]
        );
    }

    #[test]
    fn the_section_reads_and_writes_without_touching_other_text() {
        let body = "Read this.\n\n## Loot\n\n- 30 gp\n* Roll gems\n\n## Tactics\n\nHide.\n";
        assert_eq!(items(body), ["30 gp", "Roll gems"]);
        assert!(only_items(body));
        assert_eq!(without(body), "Read this.\n\n## Tactics\n\nHide.");
        assert_eq!(
            with(body, &["A key".into(), " ".into()], None),
            "Read this.\n\n## Tactics\n\nHide.\n\n## Loot\n\n- A key\n"
        );
        // Where it was: before the heading that followed it.
        assert_eq!(heading_after(body).as_deref(), Some("## Tactics"));
        assert_eq!(
            with(body, &["A key".into()], Some("## Tactics")),
            "Read this.\n\n## Loot\n\n- A key\n\n## Tactics\n\nHide."
        );
        assert_eq!(with(body, &[], None), without(body));
        assert_eq!(with("", &["A key".into()], None), "## Loot\n\n- A key\n");
        assert!(has_section(body) && !has_section(&without(body)));
        // A deeper heading stays inside the section, and fenced text is not a heading.
        let mixed =
            "## Loot\n\nIn the chest:\n- Gold\n  - Under a cloth\n### Hidden\n```\n## Loot\n```\n";
        assert_eq!(items(mixed), ["Gold"]);
        assert!(!only_items(mixed));
        assert!(items("Words\n### Loot\n- Not this").is_empty());
    }
}
