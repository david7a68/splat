//! Markdown note syntax shared by the app and saved data readers.

/// A Markdown link to another note.
pub const NOTE_LINK: &str = "splat-note:";

/// The frontmatter block with its fences, and the body after it.
pub fn split_note(text: &str) -> Option<(&str, &str)> {
    let start = text.strip_prefix('\u{feff}').unwrap_or(text);
    let newline = if start.starts_with("---\r\n") {
        "\r\n"
    } else if start.starts_with("---\n") {
        "\n"
    } else {
        return None;
    };
    let offset = text.len() - start.len();
    let close = format!("{newline}---");
    let mut from = offset + 3 + newline.len();
    loop {
        let found = text[from..].find(&close)? + from;
        let after = found + close.len();
        let rest = &text[after..];
        if rest.is_empty() {
            return Some((&text[..after], ""));
        }
        if let Some(body) = rest.strip_prefix(newline) {
            return Some((&text[..after + newline.len()], body));
        }
        from = after;
    }
}

/// The first number of an XP line: `XP 1,250 for 3 at 0 HP` is 1,250.
pub fn first_number(words: &str) -> Option<u64> {
    let start = words.find(|c: char| c.is_ascii_digit())?;
    let digits: String = words[start..]
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == ',')
        .filter(char::is_ascii_digit)
        .collect();
    digits.parse().ok()
}
