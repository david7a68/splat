//! Typographic parts of the one Reader. `library::view` composes them.

/// A named run-in paragraph's title: `Rend.`, `Acid Breath (Recharge 5–6).` Title case,
/// a few words, and an optional qualifier in brackets. A sentence does not match, so it
/// never turns bold.
pub fn run_in_title(line: &str) -> Option<&str> {
    let mut depth = 0i32;
    for (index, c) in line.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            '.' if depth == 0 => {
                let rest = &line[index + 1..];
                let title = &line[..index];
                return (rest.is_empty() || rest.starts_with(' '))
                    .then_some(title)
                    .filter(|title| title_words(title));
            }
            _ => {}
        }
    }
    None
}

fn title_words(title: &str) -> bool {
    let name = match title.split_once(" (") {
        Some((name, qualifier)) if qualifier.ends_with(')') => name,
        Some(_) => return false,
        None => title,
    };
    let words: Vec<&str> = name.split_whitespace().collect();
    let small = |word: &str| {
        matches!(
            word,
            "a" | "an"
                | "and"
                | "at"
                | "by"
                | "for"
                | "from"
                | "in"
                | "of"
                | "on"
                | "or"
                | "the"
                | "to"
                | "with"
        )
    };
    (1..=7).contains(&words.len())
        && words[0].starts_with(char::is_uppercase)
        && !small(words[words.len() - 1])
        && words.iter().all(|word| {
            (word.starts_with(|c: char| c.is_uppercase() || c.is_ascii_digit()) || small(word))
                && word
                    .chars()
                    .all(|c| c.is_alphanumeric() || "'\u{2019}-/".contains(c))
        })
}

/// Bold a paragraph's run-in title, if it has one.
pub fn run_in_markdown(paragraph: &str) -> String {
    let paragraph = paragraph.trim_start();
    match run_in_title(paragraph) {
        Some(title) => {
            let rest = &paragraph[title.len() + 1..];
            format!("**{title}.**{rest}")
        }
        None => paragraph.to_owned(),
    }
}

pub fn human_label(value: &str) -> String {
    let value = value.replace('_', " ");
    let mut chars = value.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + chars.as_str()
    })
}
