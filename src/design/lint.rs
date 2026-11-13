//! No raw colours or text sizes outside the design system, and no straight apostrophe in UI
//! words. The allowlist can only shrink: a count above it fails, and a count below it asks
//! you to lower it.
//!
//! The same allowlist counts the functions that may still make a raw egui call that
//! `clippy.toml` disallows (`#[allow(clippy::disallowed_methods)]`, rule
//! `disallowed-methods-allow`). An allow for a whole module or for a Clippy group would let a
//! new file or a new function through, so it is never allowed (`broad-clippy-allow`), nor one
//! on an `impl` block or a `mod` item.
//!
//! Staged calls are counted one by one, also inside a function that holds an allow for another
//! raw call: `Footer::show_by_hand` (`footer-by-hand`; use `Footer::pane`) and the pane's one
//! escape, `Body::Fills` (`fills-escape`).
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

/// The Clippy allows on one line of code, by the rule that counts them. Each item-level
/// `allow(clippy::disallowed_methods)` counts once; one for a whole module (`#![…]`) or for a
/// group that holds the lint (`clippy::style`, `clippy::all`) is a fault of its own.
fn clippy_allows(line: &str) -> Vec<&'static str> {
    let code = line.split("//").next().unwrap_or("");
    let attribute = code.contains("allow(") || code.contains("expect(");
    let mut found = Vec::new();
    if !attribute {
        return found;
    }
    if code.contains("clippy::disallowed_methods") {
        found.push("disallowed-methods-allow");
        if code.contains("#![") {
            found.push("broad-clippy-allow");
        }
    }
    if code.contains("clippy::style") || code.contains("clippy::all") {
        found.push("broad-clippy-allow");
    }
    found
}

/// The staged calls on one line of code, by the rule that counts them: each one counts, even in
/// a function whose allow already lets Clippy pass it.
fn staged_calls(line: &str) -> Vec<&'static str> {
    let code = line.split("//").next().unwrap_or("");
    let mut found = Vec::new();
    for (pattern, rule) in [
        (".show_by_hand(", "footer-by-hand"),
        ("Body::Fills(", "fills-escape"),
    ] {
        found.extend(std::iter::repeat_n(rule, code.matches(pattern).count()));
    }
    found
}

/// Whether the item after an attribute on `lines[at]` is an `impl` block or a `mod`: an allow
/// there covers many functions.
fn allows_a_block(lines: &[&str], at: usize) -> bool {
    lines[at + 1..]
        .iter()
        .map(|line| line.trim_start())
        .find(|line| !line.starts_with("#[") && !line.starts_with("//") && !line.is_empty())
        .is_some_and(|item| {
            let item = item.strip_prefix("pub ").unwrap_or(item);
            let item = item.strip_prefix("pub(crate) ").unwrap_or(item);
            item.starts_with("impl") || item.starts_with("mod ")
        })
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
            let root = concat!(env!("CARGO_MANIFEST_DIR"), "/");
            let key = relative.strip_prefix(root).unwrap_or(&relative);
            // Every allow counts, in tests too: a test's raw call is still a raw call.
            let lines: Vec<&str> = text.lines().collect();
            for (at, line) in lines.iter().enumerate() {
                let mut rules = clippy_allows(line);
                if rules.contains(&"disallowed-methods-allow") && allows_a_block(&lines, at) {
                    rules.push("broad-clippy-allow");
                }
                rules.extend(staged_calls(line));
                for rule in rules {
                    *counts.entry(format!("{key} {rule}")).or_default() += 1;
                }
            }
            // Unit tests may name colours and sizes. They do not ship.
            // `mod tests;` near the top names a file of its own, so only an inline module ends it.
            let code = text.split("#[cfg(test)]\nmod tests {").next().unwrap_or("");
            for line in code.lines() {
                for rule in violations(line) {
                    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/");
                    let key = relative.strip_prefix(root).unwrap_or(&relative);
                    *counts.entry(format!("{key} {rule}")).or_default() += 1;
                }
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
            let fix = if key.ends_with("straight-apostrophe") {
                "Use ’ in UI words"
            } else if key.ends_with("disallowed-methods-allow") {
                "Use the design part that clippy.toml names, not a raw egui call"
            } else if key.ends_with("broad-clippy-allow") {
                "Allow `clippy::disallowed_methods` on one function, never on a module, an impl or a group"
            } else if key.ends_with("footer-by-hand") {
                "Give the body to `Footer::pane`, which places the footer"
            } else if key.ends_with("fills-escape") {
                "Use `Body::Content`: the footer follows the content. `Body::Fills` is only for an editor or the live roster that fills its room"
            } else {
                "Use design tokens"
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

/// Every `field_id!` name in `src/`, with where it is. Comments and doc examples do not count.
fn field_ids(folder: &Path, found: &mut BTreeMap<String, Vec<String>>) {
    for item in fs::read_dir(folder).expect("read source folder") {
        let path = item.expect("read source entry").path();
        if path.is_dir() {
            field_ids(&path, found);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let text = fs::read_to_string(&path).expect("read source file");
            for (at, line) in text.lines().enumerate() {
                let code = line.split("//").next().unwrap_or("");
                for (start, _) in code.match_indices("field_id!(\"") {
                    let rest = &code[start + "field_id!(\"".len()..];
                    if let Some(name) = rest.split('"').next() {
                        found.entry(name.to_owned()).or_default().push(format!(
                            "{}:{}",
                            path.display(),
                            at + 1
                        ));
                    }
                }
            }
        }
    }
}

/// Two fields with one `FieldId` would share focus and text state: each name is used once.
/// A field shown once per row adds the row's key with `FieldId::with`.
#[test]
fn each_field_id_name_is_used_once() {
    let mut found = BTreeMap::new();
    field_ids(
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src")),
        &mut found,
    );
    assert!(
        found.contains_key("campaign_name"),
        "the scan finds field ids"
    );
    let twice: Vec<_> = found
        .iter()
        .filter(|(_, places)| places.len() > 1)
        .map(|(name, places)| format!("{name}: {}", places.join(", ")))
        .collect();
    assert!(
        twice.is_empty(),
        "One name per field id:\n{}",
        twice.join("\n")
    );
}

#[test]
fn the_lint_finds_staged_calls_and_block_allows() {
    assert_eq!(
        staged_calls("        let bottom = footer.show_by_hand(ui);"),
        ["footer-by-hand"]
    );
    assert_eq!(
        staged_calls("footer.pane(ui, design::Body::Fills(FillsReason::NoteEditor), |ui| {"),
        ["fills-escape"]
    );
    assert!(staged_calls("// footer.show_by_hand(ui) in a comment").is_empty());
    assert!(staged_calls("footer.pane(ui, Body::Content, |ui| ui.label(\"x\"));").is_empty());
    let block = ["#[allow(clippy::disallowed_methods)]", "impl Party {", "}"];
    assert!(allows_a_block(&block, 0));
    let function = [
        "#[allow(clippy::disallowed_methods)]",
        "/// Doc.",
        "    fn body(ui: &mut Ui) {",
    ];
    assert!(!allows_a_block(&function, 0));
}

#[test]
fn the_lint_finds_clippy_allows() {
    assert_eq!(
        clippy_allows("    #[allow(clippy::disallowed_methods)]"),
        ["disallowed-methods-allow"]
    );
    assert_eq!(
        clippy_allows("#![allow(clippy::disallowed_methods)]"),
        ["disallowed-methods-allow", "broad-clippy-allow"]
    );
    assert_eq!(
        clippy_allows("#[allow(clippy::style)]"),
        ["broad-clippy-allow"]
    );
    assert_eq!(
        clippy_allows("#[expect(clippy::all, reason = \"x\")]"),
        ["broad-clippy-allow"]
    );
    assert!(clippy_allows("// #[allow(clippy::disallowed_methods)] in a comment").is_empty());
    assert!(clippy_allows("let lint = \"clippy::disallowed_methods\";").is_empty());
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
