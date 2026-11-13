//! No raw colours or text sizes outside the design system, and no straight apostrophe in UI
//! words. The allowlist can only shrink: a count above it fails, and a count below it asks
//! you to lower it.
use std::{collections::BTreeMap, fs, path::Path};

const ALLOWLIST: &str = include_str!("lint_allowlist.txt");
const COLOUR_CONSTANTS: [&str; 22] = [
    "BLACK",
    "BLUE",
    "BROWN",
    "CYAN",
    "DARK_BLUE",
    "DARK_GRAY",
    "DARK_GREEN",
    "DARK_RED",
    "GOLD",
    "GRAY",
    "GREEN",
    "KHAKI",
    "LIGHT_BLUE",
    "LIGHT_GRAY",
    "LIGHT_GREEN",
    "LIGHT_RED",
    "LIGHT_YELLOW",
    "MAGENTA",
    "ORANGE",
    "RED",
    "WHITE",
    "YELLOW",
];

fn followed_by_number(text: &str) -> bool {
    text.trim_start()
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_digit() || c == '-')
}

/// The words inside each string literal that opens and closes on this line of code.
fn string_literals(code: &str) -> Vec<&str> {
    // A `'"'` character literal would turn the line inside out, so such a line has none.
    if code.contains("'\"'") {
        return Vec::new();
    }
    let mut literals = Vec::new();
    let mut start = None;
    let mut escaped = false;
    for (at, c) in code.char_indices() {
        match (start, c) {
            (Some(_), '\\') if !escaped => escaped = true,
            (Some(from), '"') if !escaped => {
                literals.push(&code[from..at]);
                start = None;
            }
            (None, '"') => start = Some(at + 1),
            _ => escaped = false,
        }
    }
    literals
}

/// A straight apostrophe inside a word (`campaign's`, `{name}'s`). UI words use `’`.
fn straight_apostrophe(words: &str) -> bool {
    let chars: Vec<char> = words.chars().collect();
    chars
        .windows(3)
        .any(|w| w[1] == '\'' && (w[0].is_alphanumeric() || w[0] == '}') && w[2].is_alphabetic())
}

/// Every raw visual value on one line of code, by the rule it breaks.
fn violations(line: &str) -> Vec<&'static str> {
    let mut found = Vec::new();
    let code = line.split("//").next().unwrap_or("");
    // UI words use curly marks (docs/design-system.md, Widgets). File content, such as YAML or
    // a prompt for a model, keeps straight ones and is counted in the allowlist.
    if string_literals(code).into_iter().any(straight_apostrophe) {
        found.push("straight-apostrophe");
    }
    for constant in COLOUR_CONSTANTS {
        let pattern = format!("Color32::{constant}");
        if code.match_indices(&pattern).any(|(at, _)| {
            !code[at + pattern.len()..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
        }) {
            found.push("raw-colour");
        }
    }
    for pattern in ["Color32::from_", "Rgba::from_", "Hsva::new"] {
        if code.contains(pattern) {
            found.push("raw-colour");
        }
    }
    for symbol in ['⋯', '⚠', 'ⓘ', '▾', '▸', '✕'] {
        if code.contains(symbol) && !code.contains(".name(") {
            found.push("raw-symbol");
        }
    }
    // A moving spinner draws from the clock, so captures of one state would differ.
    if code.contains("ui.spinner()") {
        found.push("raw-spinner");
    }
    for pattern in [
        ".size(",
        "FontId::proportional(",
        "FontId::monospace(",
        "FontId::new(",
    ] {
        if code
            .match_indices(pattern)
            .any(|(at, _)| followed_by_number(&code[at + pattern.len()..]))
        {
            found.push("raw-size");
        }
    }
    found
}

/// The old message functions (`#[deprecated]`). The compiler stops a call in a file without
/// `#![allow(deprecated)]`; this count also stops one in a file that only inherits the allow from
/// its parent module.
const OLD_MESSAGES: [&str; 11] = [
    "message",
    "small_message",
    "message_with_action",
    "message_with_actions",
    "message_with_code",
    "message_with_links",
    "undo_message",
    "undo_line",
    "undo_message_named",
    "draft_discarded",
    "reloaded_draft",
];

/// Phrases with one owner file: the one function that writes that form. A literal outside its
/// owner fails (round 27 family F4: one fact in two forms).
fn phrase_owner(path: &str, words: &str) -> bool {
    let owned = [
        // The XP of an encounter or a reward.
        (words.contains("XP {"), "src/difficulty.rs"),
        // A fault's place: `fault_words`.
        (words.contains("· line {"), "src/design/outcome.rs"),
        // A count of chosen things: `chosen_count`.
        (
            words.contains("} of {") && words.ends_with("chosen"),
            "src/design/outcome.rs",
        ),
    ];
    owned
        .iter()
        .any(|(found, owner)| *found && !path.ends_with(owner))
}

/// The message rules of one file (package M, round 28): the counted allows, the old message
/// calls, a reason without its end mark, and a phrase outside its owner.
fn message_rules(path: &str, code: &str) -> Vec<&'static str> {
    let mut found = Vec::new();
    for line in code.lines() {
        let line = line.split("//").next().unwrap_or("");
        if line.contains("allow(deprecated)") {
            found.push("allow-deprecated");
            // An allow at the crate root would cover every module.
            if line.contains("#![")
                && (path.ends_with("src/lib.rs") || path.ends_with("src/main.rs"))
            {
                found.push("crate-wide-allow");
            }
        }
        for old in OLD_MESSAGES {
            let call = format!("design::{old}(");
            found.extend(line.matches(&call).map(|_| "old-message"));
        }
        // A blocked reason is a sentence: it ends with `.`, `?` or `!` (F12).
        for pattern in [".blocked(Some(", ".blocked_if(", "blocked_reason("] {
            if let Some(at) = line.find(pattern)
                && string_literals(&line[at..]).into_iter().any(|words| {
                    !words.is_empty()
                        && !words
                            .trim_end_matches(['”', '’', ')'])
                            .ends_with(['.', '?', '!'])
                })
            {
                found.push("reason-end-mark");
            }
        }
        if string_literals(line)
            .into_iter()
            .any(|words| phrase_owner(path, words))
        {
            found.push("phrase-owner");
        }
    }
    found
}

fn scan(folder: &Path, counts: &mut BTreeMap<String, usize>) {
    for item in fs::read_dir(folder).expect("read source folder") {
        let path = item.expect("read source entry").path();
        let relative = path.to_string_lossy().replace('\\', "/");
        if path.is_dir() {
            if !relative.ends_with("src/design") && !relative.ends_with("src/visual_tests") {
                scan(&path, counts);
            }
        } else if path.extension().is_some_and(|e| e == "rs")
            // A file of unit tests, such as `formats_tests.rs`, does not ship either.
            && !relative.ends_with("tests.rs")
        {
            let text = fs::read_to_string(&path).expect("read source file");
            // Unit tests may name colours and sizes. They do not ship.
            // `mod tests;` near the top names a file of its own, so only an inline module ends it.
            let code = text.split("#[cfg(test)]\nmod tests {").next().unwrap_or("");
            let root = concat!(env!("CARGO_MANIFEST_DIR"), "/");
            let key = relative.strip_prefix(root).unwrap_or(&relative);
            for line in code.lines() {
                for rule in violations(line) {
                    *counts.entry(format!("{key} {rule}")).or_default() += 1;
                }
            }
            for rule in message_rules(key, code) {
                *counts.entry(format!("{key} {rule}")).or_default() += 1;
            }
        }
    }
}

#[test]
fn raw_colours_and_sizes_stay_inside_the_design_system() {
    let mut counts = BTreeMap::new();
    // The core crate writes many UI words too, such as a note's warnings.
    for folder in ["/src", "/crates/core/src"] {
        scan(
            Path::new(&format!("{}{folder}", env!("CARGO_MANIFEST_DIR"))),
            &mut counts,
        );
    }
    let allowed: BTreeMap<String, usize> = ALLOWLIST
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (key, count) = line.rsplit_once(' ').expect("`path rule count`");
            (key.to_owned(), count.parse().expect("a count"))
        })
        .collect();
    let mut problems = Vec::new();
    for (key, count) in &counts {
        let limit = allowed.get(key).copied().unwrap_or(0);
        if *count > limit {
            let fix = match key.rsplit_once(' ').map_or("", |(_, rule)| rule) {
                "straight-apostrophe" => "Use ’ in UI words",
                "allow-deprecated" | "old-message" => {
                    "Draw messages through `Footer::outcome`, `design::outcome::banner`, `row_fault` or `PagePlaceholder`"
                }
                "crate-wide-allow" => {
                    "Put `#[allow(deprecated)]` on the function that calls, not on the crate"
                }
                "reason-end-mark" => "End a blocked reason with `.`, `?` or `!`",
                "phrase-owner" => {
                    "Call the phrase's owner: `XP {` in src/difficulty.rs, `· line {` in `fault_words`, `N of M chosen` in `chosen_count`"
                }
                _ => "Use design tokens",
            };
            problems.push(format!("{key}: {count} found, {limit} allowed. {fix}."));
        }
    }
    for (key, limit) in &allowed {
        let count = counts.get(key).copied().unwrap_or(0);
        if count < *limit {
            problems.push(format!(
                "{key}: only {count} remain. Lower the allowlist to {count}."
            ));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn the_message_rules_find_what_they_should() {
    let rules = |path, code| message_rules(path, code);
    assert_eq!(
        rules(
            "src/a.rs",
            "#![allow(deprecated)]\n design::message(ui, t, x);"
        ),
        ["allow-deprecated", "old-message"]
    );
    assert_eq!(
        rules("src/lib.rs", "#![allow(deprecated)]"),
        ["allow-deprecated", "crate-wide-allow"]
    );
    assert!(rules("src/lib.rs", "    #[allow(deprecated)]").len() == 1);
    assert!(rules("src/a.rs", "design::outcome::row_fault(ui, &f);").is_empty());
    assert_eq!(
        rules(
            "src/a.rs",
            "Action::quiet(\"Save\").blocked(Some(\"Stop recording first\"))"
        ),
        ["reason-end-mark"]
    );
    assert!(
        rules(
            "src/a.rs",
            "Action::quiet(\"Save\").blocked(Some(\"Stop recording first.\"))"
        )
        .is_empty()
    );
    assert_eq!(
        rules("src/a.rs", ".blocked_if(busy, \"Wait\")"),
        ["reason-end-mark"]
    );
    assert_eq!(rules("src/a.rs", "format!(\"XP {xp}\")"), ["phrase-owner"]);
    assert!(rules("src/difficulty.rs", "format!(\"XP {xp}\")").is_empty());
    assert_eq!(
        rules("src/a.rs", "format!(\"{name} · line {n}: {cause}\")"),
        ["phrase-owner"]
    );
    assert_eq!(
        rules("src/a.rs", "format!(\"{a} of {b} chosen\")"),
        ["phrase-owner"]
    );
    assert!(rules("src/a.rs", "let chosen = 3; // design::message(").is_empty());
}

#[test]
fn the_lint_finds_what_it_should() {
    assert_eq!(
        violations("ui.colored_label(Color32::LIGHT_RED, x)"),
        ["raw-colour"]
    );
    assert_eq!(violations("Color32::from_rgb(1, 2, 3)"), ["raw-colour"]);
    assert_eq!(violations("egui::Color32::from_gray(205)"), ["raw-colour"]);
    assert_eq!(violations("RichText::new(x).size(17.0)"), ["raw-size"]);
    assert_eq!(violations("egui::FontId::proportional(13.0)"), ["raw-size"]);
    assert_eq!(violations("ui.menu_button(\"⋯\", add)"), ["raw-symbol"]);
    assert!(violations("design::row_menu(ui, name, add)").is_empty());
    assert!(violations("Type::Body.size()").is_empty());
    assert!(violations("// Color32::RED in a comment").is_empty());
    assert!(violations("Color32::TRANSPARENT").is_empty());
    assert!(violations("font_id.size = prose.size();").is_empty());
    assert_eq!(
        violations("format!(\"Not in this campaign's packages.\")"),
        ["straight-apostrophe"]
    );
    assert_eq!(
        violations("format!(\"{name}'s sheet changed\")"),
        ["straight-apostrophe"]
    );
    assert!(violations("format!(\"Not in this campaign’s packages.\")").is_empty());
    // A lifetime, a character literal and a comment are not UI words.
    assert!(violations("fn read<'a>(text: &'a str) -> &'a str {").is_empty());
    assert!(violations("if c == '\"' && b == b'd' { \"x\" }").is_empty());
    assert!(violations("let s = \"a\"; // the campaign's rule").is_empty());
    // YAML's quoted scalars are file content: no letter on both sides of the mark.
    assert!(violations("\"splat_in: '[[Undercroft]]'\"").is_empty());
}
