//! A note as lines: one list item, one paragraph or one table row each. A line that links to
//! another page, or names it, is a fact about that page, and Splat shows it there. So a fact
//! needs one home only. See `docs/design/world-data.md`.
use super::each_link;

/// The heading that holds an entry's history from play.
pub const LOG: &str = "Log";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Paragraph,
    Item,
    Row,
    /// A heading names a page too, but it states no fact.
    Heading,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub kind: Kind,
    /// The nearest heading above, unless it is the note's title.
    pub heading: Option<String>,
    /// Bold words and a colon at the start, such as `Look`.
    pub label: Option<String>,
    /// The words as a reader sees them, without the label.
    pub text: String,
    lower: String,
    /// Each link's file or note, in lower case, without its heading.
    links: Vec<String>,
    /// Each link's page as the note wrote it, without embeds.
    targets: Vec<String>,
    /// Only "See" and links. It adds no fact.
    pub bare: bool,
    /// A task line: `Some(true)` when it is ticked.
    pub task: Option<bool>,
    /// An unticked task line's box: the number of the body's line that holds it, and the whole
    /// question as read, that line and its continuation lines joined by `\n`, so a tick finds
    /// exactly this question.
    pub open_box: Option<(usize, String)>,
    /// Under the Log heading, or under a heading inside the Log.
    log: bool,
}

impl Line {
    pub fn links_to(&self, keys: &[String]) -> bool {
        self.links.iter().any(|link| keys.contains(link))
    }

    pub fn names(&self, lower_name: &str) -> bool {
        crate::matching::names_lower_word(&self.lower, lower_name)
    }

    /// The pages this line links to, as the note wrote each one, with any heading.
    pub fn link_targets(&self) -> &[String] {
        &self.targets
    }

    /// The line holds a date in words, such as `about 90 years ago`.
    pub fn has_date(&self) -> bool {
        let words: Vec<&str> = self
            .lower
            .split_whitespace()
            .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric()))
            .collect();
        words.windows(3).any(|w| {
            w[2] == "ago"
                && ["year", "month", "week", "day"].contains(&w[1].trim_end_matches('s'))
                && w[0].parse::<f64>().is_ok()
        })
    }

    /// Each sentence of at least `words` words, in lower case without punctuation, so the
    /// same sentence on another page compares equal.
    pub fn sentences(&self, words: usize) -> Vec<String> {
        let mut found = Vec::new();
        let mut sentence = String::new();
        let mut chars = self.lower.chars().peekable();
        let mut close = |sentence: &mut String| {
            let plain: Vec<&str> = sentence.split_whitespace().collect();
            if plain.len() >= words {
                found.push(plain.join(" "));
            }
            sentence.clear();
        };
        while let Some(c) = chars.next() {
            if matches!(c, '.' | '?' | '!') && chars.peek().is_none_or(|next| next.is_whitespace())
            {
                close(&mut sentence);
            } else if c.is_alphanumeric() || c.is_whitespace() {
                sentence.push(c);
            } else if matches!(c, '-' | '–' | '—' | '/') {
                sentence.push(' ');
            }
        }
        close(&mut sentence);
        found
    }

    pub fn in_log(&self) -> bool {
        self.log
    }

    /// The label and the words, as one sentence.
    pub fn words(&self) -> String {
        match &self.label {
            Some(label) if self.text.is_empty() => label.clone(),
            Some(label) => format!("{label}: {}", self.text),
            None => self.text.clone(),
        }
    }

    /// About `width` characters around the first of these lower-case names, for a long line.
    pub fn snippet(&self, names: &[String], width: usize) -> String {
        let words = self.words();
        if words.chars().count() <= width {
            return words;
        }
        let lower = words.to_lowercase();
        let found = names
            .iter()
            .filter(|name| !name.is_empty())
            .filter_map(|name| lower.find(name.as_str()))
            .min()
            // Lower case can change a character's length. Then the start is good enough.
            .filter(|_| lower.len() == words.len())
            .unwrap_or(0);
        let mut start = found.saturating_sub(width / 3);
        while !words.is_char_boundary(start) {
            start -= 1;
        }
        if start > 0 {
            start = words[start..]
                .find(' ')
                .map_or(start, |space| start + space + 1);
        }
        let shown: String = words[start..].chars().take(width).collect();
        let cut = start + shown.len() < words.len();
        format!(
            "{}{}{}",
            if start > 0 { "…" } else { "" },
            shown.trim_end(),
            if cut { "…" } else { "" }
        )
    }
}

fn list_marker(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    for marker in ["- ", "* ", "+ "] {
        if let Some(rest) = trimmed.strip_prefix(marker) {
            return Some(rest);
        }
    }
    let digits = trimmed.chars().take_while(char::is_ascii_digit).count();
    if digits > 0 && digits < 10 {
        let rest = &trimmed[digits..];
        if let Some(rest) = rest.strip_prefix(". ").or_else(|| rest.strip_prefix(") ")) {
            return Some(rest);
        }
    }
    None
}

/// A heading's level and its words.
fn heading_words(line: &str) -> Option<(usize, &str)> {
    let hashes = line.chars().take_while(|c| *c == '#').count();
    ((1..=6).contains(&hashes) && line[hashes..].starts_with(' '))
        .then(|| (hashes, line[hashes..].trim().trim_end_matches('#').trim()))
}

fn separator_row(line: &str) -> bool {
    line.contains('-')
        && line
            .chars()
            .all(|c| matches!(c, '|' | '-' | ':' | ' ' | '\t'))
}

/// Remove `open … close` spans, such as Obsidian's `%% … %%` comments.
fn without(text: &str, open: &str, close: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find(open) {
        output.push_str(&rest[..start]);
        match rest[start + open.len()..].find(close) {
            Some(end) => rest = &rest[start + open.len() + end + close.len()..],
            None => {
                rest = "";
                break;
            }
        }
    }
    output.push_str(rest);
    output
}

/// Markdown links as their words.
fn link_words(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('[') {
        let after = &rest[open + 1..];
        if let Some(close) = after.find("](")
            && !after[..close].contains(['[', '\n'])
            && let Some(end) = after[close + 2..].find(')')
        {
            output.push_str(&rest[..open]);
            output.push_str(&after[..close]);
            rest = &after[close + 2 + end + 1..];
        } else {
            output.push_str(&rest[..=open]);
            rest = after;
        }
    }
    output.push_str(rest);
    output
}

/// A line's words without Markdown, as the reader shows them.
fn plain(raw: &str) -> String {
    let mut text = without(&without(raw, "%%", "%%"), "<!--", "-->");
    let mut linked = String::with_capacity(text.len());
    let mut copied = 0;
    for (range, embed, target, words) in each_link(&text) {
        linked.push_str(&text[copied..range.start]);
        copied = range.end;
        if !embed {
            linked.push_str(&words.unwrap_or_else(|| {
                target
                    .rsplit('#')
                    .next()
                    .unwrap_or(&target)
                    .trim_start_matches('^')
                    .to_owned()
            }));
        }
    }
    linked.push_str(&text[copied..]);
    text = link_words(&linked);
    let chars: Vec<char> = text.chars().collect();
    let mut output = String::with_capacity(text.len());
    for (at, &c) in chars.iter().enumerate() {
        let edge = |other: Option<&char>| other.is_none_or(|c| !c.is_alphanumeric());
        match c {
            '*' | '`' => {}
            '\\' if chars
                .get(at + 1)
                .is_some_and(|next| !next.is_alphanumeric()) => {}
            // An underscore inside a word is a letter. At its edge, it is emphasis.
            '_' if edge(at.checked_sub(1).and_then(|before| chars.get(before)))
                || edge(chars.get(at + 1)) => {}
            _ => output.push(c),
        }
    }
    // A block ID at the end is for links, not for readers.
    if let Some(at) = output.rfind(" ^")
        && output[at + 2..]
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
        && output.len() > at + 2
    {
        output.truncate(at);
    }
    output.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Bold words and a colon at the start: `**Look:** …` or `**Look**: …`.
fn split_label(raw: &str) -> (Option<String>, &str) {
    let Some(rest) = raw.trim_start().strip_prefix("**") else {
        return (None, raw);
    };
    let Some(end) = rest.find("**") else {
        return (None, raw);
    };
    let (inside, after) = (&rest[..end], &rest[end + 2..]);
    let (label, after) = if let Some(label) = inside.strip_suffix(':') {
        (label, after)
    } else if let Some(after) = after.strip_prefix(':') {
        (inside, after)
    } else {
        return (None, raw);
    };
    let label = plain(label);
    if label.is_empty() || label.len() > 60 {
        return (None, raw);
    }
    (Some(label), after)
}

fn bare(raw: &str) -> bool {
    let text = without(&without(raw, "%%", "%%"), "<!--", "-->");
    let links = each_link(&text);
    if links.is_empty() {
        return false;
    }
    let mut rest = String::new();
    let mut copied = 0;
    for (range, ..) in links {
        rest.push_str(&text[copied..range.start]);
        rest.push(' ');
        copied = range.end;
    }
    rest.push_str(&text[copied..]);
    rest.split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .all(|word| {
            ["see", "also", "and", "or", "the"]
                .iter()
                .any(|filler| word.eq_ignore_ascii_case(filler))
        })
}

fn line(kind: Kind, heading: &Option<String>, log: bool, raw: &str) -> Option<Line> {
    let mut raw = raw.trim();
    let mut task = None;
    if kind == Kind::Item {
        for (mark, done) in [("[ ] ", false), ("[x] ", true), ("[X] ", true)] {
            if let Some(rest) = raw.strip_prefix(mark) {
                task = Some(done);
                raw = rest;
            }
        }
    }
    let (label, rest) = if kind == Kind::Row {
        (None, raw)
    } else {
        split_label(raw)
    };
    let text = if kind == Kind::Row {
        let cells = raw.replace("\\|", "\u{1}");
        cells
            .trim_matches('|')
            .split('|')
            .map(|cell| plain(&cell.replace('\u{1}', "|")))
            .filter(|cell| !cell.is_empty())
            .collect::<Vec<_>>()
            .join(" · ")
    } else {
        plain(rest)
    };
    if text.is_empty() && label.is_none() {
        return None;
    }
    let mut links = Vec::new();
    let mut targets = Vec::new();
    for (_, embed, target, _) in each_link(raw) {
        let name = target.split('#').next().unwrap_or("").trim();
        let name = name.strip_suffix(".md").unwrap_or(name);
        if !name.is_empty() {
            links.push(name.to_lowercase());
            if !embed {
                targets.push(target);
            }
        }
    }
    let lower = match &label {
        Some(label) => format!("{label}: {text}").to_lowercase(),
        None => text.to_lowercase(),
    };
    Some(Line {
        kind,
        heading: heading.clone(),
        bare: kind == Kind::Heading || bare(raw),
        label,
        text,
        lower,
        links,
        targets,
        task,
        open_box: None,
        log,
    })
}

/// Every line of a note's body. `title` is the note's name, whose heading is not a section.
pub fn lines(body: &str, title: &str) -> Vec<Line> {
    /// The open block: its kind, its raw words, and for a list item its first line's number and
    /// words as read.
    type Open = Option<(Kind, String, Option<(usize, String)>)>;
    /// The heading above, and the level of the Log heading when the lines are inside it.
    type Section = (Option<String>, Option<usize>);
    fn close(found: &mut Vec<Line>, open: &mut Open, (heading, log): &Section) {
        if let Some((kind, raw, first)) = open.take() {
            found.extend(line(kind, heading, log.is_some(), &raw).map(|mut line| {
                if line.task == Some(false) {
                    line.open_box = first;
                }
                line
            }));
        }
    }
    let mut found = Vec::new();
    let mut heading: Section = (None, None);
    let mut open: Open = None;
    let mut fence = false;
    let mut header_row = false;
    for (number, source) in body.lines().enumerate() {
        // A quote or a callout holds lines too.
        let mut text = source.trim_start();
        while let Some(rest) = text.strip_prefix('>') {
            text = rest.trim_start();
        }
        if text.starts_with("```") || text.starts_with("~~~") {
            fence = !fence;
            close(&mut found, &mut open, &heading);
            continue;
        }
        if fence {
            continue;
        }
        if text.is_empty() || text.starts_with("[!") {
            close(&mut found, &mut open, &heading);
            header_row = false;
            continue;
        }
        if let Some((level, words)) = heading_words(text) {
            close(&mut found, &mut open, &heading);
            let name = plain(words);
            // The first heading is the title, whatever its words, when nothing is above it.
            let is_title = name.eq_ignore_ascii_case(title.trim())
                || (level == 1 && found.is_empty() && heading.0.is_none());
            // The Log ends at the next heading of its own level or above.
            if heading.1.is_some_and(|log| level <= log) {
                heading.1 = None;
            }
            if name.eq_ignore_ascii_case(LOG) {
                heading.1 = Some(level);
            }
            heading.0 = (!is_title).then_some(name);
            if heading.0.is_some() {
                found.extend(line(Kind::Heading, &heading.0, heading.1.is_some(), words));
            }
            header_row = false;
            continue;
        }
        if text.starts_with('|') {
            close(&mut found, &mut open, &heading);
            if separator_row(text) {
                // The row above a separator names the columns.
                if header_row {
                    found.pop();
                }
                header_row = false;
            } else {
                let before = found.len();
                found.extend(line(Kind::Row, &heading.0, heading.1.is_some(), text));
                header_row = found.len() > before;
            }
            continue;
        }
        header_row = false;
        if let Some(rest) = list_marker(text) {
            close(&mut found, &mut open, &heading);
            let first = rest
                .trim_start()
                .starts_with("[ ] ")
                .then(|| (number, source.to_owned()));
            open = Some((Kind::Item, rest.to_owned(), first));
            continue;
        }
        match &mut open {
            Some((_, raw, first)) => {
                raw.push(' ');
                raw.push_str(text);
                // The question as read holds its continuation lines too.
                if let Some((_, question)) = first {
                    question.push('\n');
                    question.push_str(source);
                }
            }
            None => open = Some((Kind::Paragraph, text.to_owned(), None)),
        }
    }
    close(&mut found, &mut open, &heading);
    found
}

/// A file with one more line in its Log. The Log is the last heading unless the DM put it
/// elsewhere, and Splat makes it when the note has none.
pub fn add_log(file: &str, words: &str) -> String {
    let newline = if file.contains("\r\n") { "\r\n" } else { "\n" };
    let mut fence = false;
    // The level of the last Log heading, and where its section ends.
    let mut log = None;
    let mut end = file.len();
    let mut at = 0;
    // Front matter can hold a `#` comment, which is not a heading.
    let mut front = file.starts_with("---");
    for (number, line) in file.split_inclusive('\n').enumerate() {
        let text = line.trim();
        if front {
            front = number == 0 || text != "---";
        } else if text.starts_with("```") || text.starts_with("~~~") {
            fence = !fence;
        } else if !fence && let Some((level, heading)) = heading_words(text) {
            // The Log ends at the next heading of its own level or above.
            if log.is_some_and(|log| level <= log) && end == file.len() {
                end = at;
            }
            if plain(heading).eq_ignore_ascii_case(LOG) {
                log = Some(level);
                end = file.len();
            }
        }
        at += line.len();
    }
    let entry = format!("- {words}{newline}");
    match log {
        Some(_) => {
            let kept = file[..end].trim_end();
            let rest = &file[end..];
            // The first line of a Log follows a blank line. Later lines follow the list.
            let gap = if kept
                .lines()
                .last()
                .is_some_and(|l| heading_words(l.trim()).is_some())
            {
                format!("{newline}{newline}")
            } else {
                newline.to_owned()
            };
            let after = if rest.is_empty() {
                String::new()
            } else {
                format!("{newline}{rest}")
            };
            format!("{kept}{gap}{entry}{after}")
        }
        None => {
            let kept = file.trim_end();
            format!("{kept}{newline}{newline}## {LOG}{newline}{newline}{entry}")
        }
    }
}

/// What the subject is: the first paragraph, when the note starts with one.
pub fn summary(lines: &[Line]) -> &str {
    lines
        .iter()
        .find(|line| line.kind != Kind::Heading)
        .filter(|line| line.kind == Kind::Paragraph && line.heading.is_none())
        .map_or("", |line| line.text.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZAREL: &str = "\n# Zarel Second\n\nTiefling. The Games-Master, who runs the race for the [[Arena Guild]].\n\n- **Look:** Gilded [[Horn Rings|horn rings]], loud and new. A [[Verrash Gunworks|Verrash]] pistol.\n- **Says:** \"Filed!\"\n\n## Ties\n\n| Team | Sponsor |\n|---|---|\n| [[The Ledger\\|the Ledger]] | Merchant bank |\n\n```yaml\nlook: [[Horn Rings]]\n```\n\nSee [[Tournament]] and [[Undercroft]].\n\n## Log\n\n- [[Session 1]], Day 1: Met the party. ^w-1a2b\n- [ ] Does Zarel know about the _fix_? %% splat id=1 %%\n";

    #[test]
    fn a_note_reads_as_lines_with_their_headings_labels_and_links() {
        let found = lines(ZAREL, "Zarel Second");
        let words: Vec<_> = found.iter().map(Line::words).collect();
        assert_eq!(
            words,
            [
                "Tiefling. The Games-Master, who runs the race for the Arena Guild.",
                "Look: Gilded horn rings, loud and new. A Verrash pistol.",
                "Says: \"Filed!\"",
                "Ties",
                "the Ledger · Merchant bank",
                "See Tournament and Undercroft.",
                "Log",
                "Session 1, Day 1: Met the party.",
                "Does Zarel know about the fix?",
            ]
        );
        assert_eq!(summary(&found), words[0]);
        let look = &found[1];
        assert_eq!(look.label.as_deref(), Some("Look"));
        assert!(look.links_to(&["horn rings".into()]));
        assert!(look.names("verrash"));
        assert!(!look.names("verr"));
        assert_eq!(found[4].kind, Kind::Row);
        assert_eq!(found[4].heading.as_deref(), Some("Ties"));
        assert!(found[4].links_to(&["the ledger".into()]));
        // A line of links states nothing, and a heading states nothing.
        assert!(found[5].bare && found[3].bare && !found[1].bare);
        assert!(found[7].in_log() && !found[5].in_log());
        assert_eq!(found[8].task, Some(false));
        assert_eq!(found[7].task, None);
        // An open question knows the line that holds its box, as read.
        let (at, line) = found[8].open_box.clone().expect("the open question's box");
        assert_eq!(ZAREL.lines().nth(at), Some(line.as_str()));
        assert!(line.starts_with("- [ ] Does Zarel know"));
        assert_eq!(found[7].open_box, None);
        assert_eq!(found[5].link_targets(), ["Tournament", "Undercroft"]);
    }

    #[test]
    fn the_first_heading_is_the_title_and_a_log_keeps_its_own_headings() {
        let found = lines(
            "# Kabrigar, the Coal City\n\nThe capital of Kaun.\n\n## **Log**\n\n### Day 2\n\n- Burned.\n\n## Notes\n\n- Cold.\n",
            "Kabrigar",
        );
        assert_eq!(summary(&found), "The capital of Kaun.");
        let logged: Vec<_> = found
            .iter()
            .filter(|line| line.in_log())
            .map(Line::words)
            .collect();
        assert_eq!(logged, ["Log", "Day 2", "Burned."]);
        // A new line goes after the Log's own headings, and the next section stays below.
        let file = "---\n# a comment\n---\n# Hob\n\n## **Log**\n\n### Day 2\n\n- Burned.\n\n## Notes\n\n- Cold.\n";
        assert_eq!(
            add_log(file, "Rebuilt."),
            "---\n# a comment\n---\n# Hob\n\n## **Log**\n\n### Day 2\n\n- Burned.\n- Rebuilt.\n\n## Notes\n\n- Cold.\n"
        );
    }

    #[test]
    fn a_line_knows_its_dates_and_its_long_sentences() {
        let found = lines(
            "About 90 years ago, Kaun won its revolt with Velash's money. Short one. It was 3 days long.\n\n- Verori settled Kabrigar for its coal and iron — losing it hurt!\n",
            "Note",
        );
        assert!(found[0].has_date() && !found[1].has_date());
        assert_eq!(
            found[0].sentences(8),
            ["about 90 years ago kaun won its revolt with velashs money"]
        );
        assert_eq!(
            found[1].sentences(8),
            ["verori settled kabrigar for its coal and iron losing it hurt"]
        );
    }

    #[test]
    fn a_log_line_goes_at_the_end_of_the_log_and_makes_one_when_needed() {
        let made = add_log(
            "---\na: 1\n---\n\n# Hob\n\nGoblin.\n",
            "[[Session 1]], Day 1: Status alive → dead.",
        );
        assert_eq!(
            made,
            "---\na: 1\n---\n\n# Hob\n\nGoblin.\n\n## Log\n\n- [[Session 1]], Day 1: Status alive → dead.\n"
        );
        let second = add_log(&made, "[[Session 2]]: Status dead → risen.");
        assert!(
            second.ends_with("dead.\n- [[Session 2]]: Status dead → risen.\n"),
            "{second}"
        );
        // A Log the DM put in the middle keeps the heading after it, and code holds no heading.
        let middle = "# Hob\n\n## Log\n\n- One.\n\n```\n## Log\n```\n\n## Notes\n\nKeep.\n";
        assert_eq!(
            add_log(middle, "Two."),
            "# Hob\n\n## Log\n\n- One.\n\n```\n## Log\n```\n- Two.\n\n## Notes\n\nKeep.\n"
        );
        let found = lines(&second, "Hob");
        assert!(
            found
                .iter()
                .filter(|line| line.in_log() && !line.bare)
                .count()
                == 2
        );
        assert_eq!(
            add_log("# Hob\r\n", "One."),
            "# Hob\r\n\r\n## Log\r\n\r\n- One.\r\n"
        );
    }

    #[test]
    fn a_note_that_starts_with_a_list_has_no_summary() {
        assert_eq!(summary(&lines("# Hob\n\n- **Look:** A hat.\n", "Hob")), "");
        assert_eq!(summary(&lines("## People\n\nNot a summary.\n", "Hob")), "");
        assert_eq!(
            summary(&lines("Goblin.\nThe oldest.\n", "Hob")),
            "Goblin. The oldest."
        );
    }

    #[test]
    fn a_long_line_shows_the_words_around_the_name() {
        let text = format!(
            "{} Verori fired on the miners. {}",
            "word ".repeat(60),
            "end ".repeat(60)
        );
        let found = lines(&text, "Note");
        let shown = found[0].snippet(&["verori".into()], 80);
        assert!(shown.contains("Verori fired"), "{shown}");
        assert!(shown.starts_with('…') && shown.ends_with('…'));
        assert!(shown.chars().count() <= 82);
        assert_eq!(lines("Short.", "Note")[0].snippet(&[], 80), "Short.");
    }
}
