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

/// The lint levels an attribute line sets for `deprecated`: `allow(...)` or `expect(...)` that
/// name it, and `warn`, `deny` or `forbid` that name it. Lists and `cfg_attr` are read whole:
/// `#![allow(deprecated, dead_code)]` is an allow.
fn deprecated_levels(line: &str) -> (bool, bool) {
    let line = line.trim_start();
    if !line.starts_with("#[") && !line.starts_with("#![") {
        return (false, false);
    }
    let names = |level: &str| {
        line.match_indices(&format!("{level}(")).any(|(at, _)| {
            line[at + level.len() + 1..]
                .split(')')
                .next()
                .unwrap_or("")
                .split(',')
                .any(|name| name.trim() == "deprecated")
        })
    };
    (
        names("allow") || names("expect"),
        names("warn") || names("deny") || names("forbid"),
    )
}

/// How a file sets the `deprecated` level for the modules below it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct FileLevel {
    /// An inner `#![allow(deprecated)]`, or an allow on a `mod` item: its children inherit it.
    allows_children: bool,
    /// An inner attribute that sets the level for this whole file: allow, or warn or deny.
    explicit: bool,
}

fn file_level(code: &str) -> FileLevel {
    let mut level = FileLevel::default();
    let lines: Vec<&str> = code.lines().collect();
    for (at, line) in lines.iter().enumerate() {
        let (allow, reset) = deprecated_levels(line);
        let inner = line.trim_start().starts_with("#![");
        if inner && (allow || reset) {
            level.explicit = true;
        }
        let on_mod = !inner
            && lines[at + 1..]
                .iter()
                .find(|next| !next.trim_start().starts_with("#["))
                .is_some_and(|next| {
                    let next = next.trim_start();
                    next.starts_with("mod ") || next.contains(" mod ") && next.ends_with(';')
                });
        if allow && (inner || on_mod) {
            level.allows_children = true;
        }
    }
    level
}

/// The file of the module that holds the module in `path` (`src/a/b.rs` → `src/a.rs` or
/// `src/a/mod.rs`; `src/a.rs` → `src/lib.rs` or `src/main.rs`).
fn parents(path: &str) -> Vec<String> {
    // A crate root has no parent module.
    if path.ends_with("src/lib.rs") || path.ends_with("src/main.rs") {
        return Vec::new();
    }
    let dir = match path.strip_suffix("/mod.rs") {
        Some(module) => module.rsplit_once('/').map_or("", |(dir, _)| dir),
        None => path.rsplit_once('/').map_or("", |(dir, _)| dir),
    };
    if dir.ends_with("/src") || dir == "src" {
        return vec![format!("{dir}/lib.rs"), format!("{dir}/main.rs")];
    }
    vec![format!("{dir}.rs"), format!("{dir}/mod.rs")]
}

/// Does the file at `path` run under an allow of `deprecated` that it did not write itself? Then
/// the compiler cannot stop an alias, an import or a call of an old function in it.
fn inherits_allow(path: &str, levels: &BTreeMap<String, FileLevel>) -> bool {
    if levels.get(path).is_some_and(|level| level.explicit) {
        return false;
    }
    let mut at = path.to_owned();
    loop {
        let Some((parent, level)) = parents(&at)
            .into_iter()
            .find_map(|parent| levels.get(&parent).map(|level| (parent, *level)))
        else {
            return false;
        };
        if parent == at {
            return false;
        }
        if level.allows_children {
            return true;
        }
        if level.explicit {
            return false;
        }
        at = parent;
    }
}

/// The arguments of the call that starts just after `open` (the byte after its `(`), up to the
/// matching `)`, across lines; brackets inside string literals do not count.
fn call_arguments(code: &str, open: usize) -> &str {
    let (mut depth, mut quoted, mut escaped) = (1usize, false, false);
    for (at, c) in code[open..].char_indices() {
        match c {
            '\\' if quoted && !escaped => {
                escaped = true;
                continue;
            }
            '"' if !escaped => quoted = !quoted,
            '(' if !quoted => depth += 1,
            ')' if !quoted => {
                depth -= 1;
                if depth == 0 {
                    return &code[open..open + at];
                }
            }
            _ => {}
        }
        escaped = false;
    }
    &code[open..]
}

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

/// The helpers of `sentence!`: outside `src/design/outcome.rs` they build an unchecked sentence.
const OUTCOME_INTERNALS: [&str; 4] = [
    "__formatted(",
    "__literal(",
    "check_template(",
    "check_literal(",
];

/// The message rules of one file (package M, round 28). Each is a count per file in the
/// allowlist. "Shrink only" holds for a count alone: a change that raises a count and its
/// allowance together passes this test, so a reviewer reads every raised line (AGENTS.md).
///
/// - `allow-deprecated`: each attribute that allows `deprecated` (old message functions).
/// - `crate-wide-allow`: an inner allow at a crate root, which would cover every module.
/// - `reason-end-mark`: a blocked reason (the last literal of `.blocked(Some(…))`,
///   `.blocked_if(…)` or `blocked_reason(…)`, across lines) without `.`, `?` or `!`.
/// - `phrase-owner`: a phrase outside its owner file.
/// - `outcome-internal`: a helper of `sentence!`, or `Sentence` renamed in a `use`.
/// - `outside-words`: each `Sentence::outside(`, the escape for words from outside the app.
/// - `body-message`: each body place (`outcome::banner(`, `outcome::row_fault(`,
///   `PagePlaceholder::new(`), or an import that would hide one from this count.
fn message_rules(path: &str, code: &str) -> Vec<&'static str> {
    let mut found = Vec::new();
    let crate_root = path.ends_with("src/lib.rs") || path.ends_with("src/main.rs");
    for line in code.lines() {
        let line = line.split("//").next().unwrap_or("");
        if deprecated_levels(line).0 {
            found.push("allow-deprecated");
            if line.trim_start().starts_with("#![") && crate_root {
                found.push("crate-wide-allow");
            }
        }
        if string_literals(line)
            .into_iter()
            .any(|words| phrase_owner(path, words))
        {
            found.push("phrase-owner");
        }
        if !path.ends_with("src/design/outcome.rs")
            && (OUTCOME_INTERNALS.iter().any(|name| line.contains(name))
                || line.trim_start().starts_with("use ") && line.contains("Sentence as "))
        {
            found.push("outcome-internal");
        }
        found.extend(line.matches("Sentence::outside(").map(|_| "outside-words"));
        for place in [
            "outcome::banner(",
            "outcome::row_fault(",
            "PagePlaceholder::new(",
        ] {
            found.extend(line.matches(place).map(|_| "body-message"));
        }
        let trimmed = line.trim_start();
        if trimmed.starts_with("use ")
            && trimmed.contains("outcome")
            && trimmed
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .any(|word| word == "banner" || word == "row_fault")
        {
            found.push("body-message");
        }
    }
    // A blocked reason is a sentence: it ends with `.`, `?` or `!` (F12).
    let code_only: String = code
        .lines()
        .map(|line| line.split("//").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n");
    for pattern in [".blocked(Some(", ".blocked_if(", "blocked_reason("] {
        for (at, _) in code_only.match_indices(pattern) {
            let arguments = call_arguments(&code_only, at + pattern.len());
            let last = arguments
                .lines()
                .flat_map(string_literals)
                .filter(|words| !words.is_empty())
                .next_back();
            if last.is_some_and(|words| {
                !words
                    .trim_end_matches(['”', '’', ')'])
                    .ends_with(['.', '?', '!'])
            }) {
                found.push("reason-end-mark");
            }
        }
    }
    found
}

fn scan(
    folder: &Path,
    counts: &mut BTreeMap<String, usize>,
    levels: &mut BTreeMap<String, FileLevel>,
) {
    for item in fs::read_dir(folder).expect("read source folder") {
        let path = item.expect("read source entry").path();
        let relative = path.to_string_lossy().replace('\\', "/");
        if path.is_dir() {
            if relative.ends_with("src/design") {
                // Design is exempt from the rules, except that only `outcome.rs` may use the
                // helpers of `sentence!`.
                design_internals(&path, counts);
            } else if !relative.ends_with("src/visual_tests") {
                scan(&path, counts, levels);
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
            levels.insert(key.to_owned(), file_level(&text));
        }
    }
}

/// `outcome-internal` in the design folder: only `outcome.rs` may build a sentence by hand.
fn design_internals(folder: &Path, counts: &mut BTreeMap<String, usize>) {
    for item in fs::read_dir(folder).expect("read design folder") {
        let path = item.expect("read design entry").path();
        let relative = path.to_string_lossy().replace('\\', "/");
        if relative.ends_with(".rs") && !relative.ends_with("src/design/outcome.rs") {
            let text = fs::read_to_string(&path).expect("read design file");
            let hits = text
                .lines()
                .filter(|line| OUTCOME_INTERNALS.iter().any(|name| line.contains(name)))
                .count();
            if hits > 0 {
                let root = concat!(env!("CARGO_MANIFEST_DIR"), "/");
                let key = relative.strip_prefix(root).unwrap_or(&relative);
                *counts.entry(format!("{key} outcome-internal")).or_default() += hits;
            }
        }
    }
}

#[test]
fn raw_colours_and_sizes_stay_inside_the_design_system() {
    let mut counts = BTreeMap::new();
    let mut levels = BTreeMap::new();
    // The core crate writes many UI words too, such as a note's warnings.
    for folder in ["/src", "/crates/core/src"] {
        scan(
            Path::new(&format!("{}{folder}", env!("CARGO_MANIFEST_DIR"))),
            &mut counts,
            &mut levels,
        );
    }
    // A module under an allow of `deprecated` writes its own level: an allow (counted), or
    // `#![warn(deprecated)]`, so the compiler stops every alias, import and call in it.
    for key in levels.keys() {
        if inherits_allow(key, &levels) {
            *counts.entry(format!("{key} inherited-allow")).or_default() += 1;
        }
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
                "allow-deprecated" => {
                    "Draw messages through `Footer::outcome`, `design::outcome::banner`, `row_fault` or `PagePlaceholder`, and remove the allow"
                }
                "inherited-allow" => {
                    "Write `#![warn(deprecated)]` at the top of this file (its parent allows old message functions)"
                }
                "crate-wide-allow" => {
                    "Put `#[allow(deprecated)]` on the function that calls, not on the crate"
                }
                "reason-end-mark" => "End a blocked reason with `.`, `?` or `!`",
                "phrase-owner" => {
                    "Call the phrase's owner: `XP {` in src/difficulty.rs, `· line {` in `fault_words`, `N of M chosen` in `chosen_count`"
                }
                "outcome-internal" => "Make sentences with `sentence!` or `Sentence::outside`",
                "outside-words" => {
                    "Use `sentence!`; words from outside the app go through `Sentence::outside`, and a reviewer reads each new use"
                }
                "body-message" => {
                    "Put results, reasons and notices in the pane's `Footer`; a new body place is a review decision"
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
        rules("src/a.rs", "#![allow(deprecated)]"),
        ["allow-deprecated"]
    );
    // A combined attribute is an allow too; another lint alone is not.
    assert_eq!(
        rules("src/a.rs", "#![allow(deprecated, dead_code)]"),
        ["allow-deprecated"]
    );
    assert_eq!(
        rules(
            "src/a.rs",
            "#[cfg_attr(test, allow(dead_code, deprecated))]"
        ),
        ["allow-deprecated"]
    );
    assert!(rules("src/a.rs", "#![allow(dead_code)]").is_empty());
    assert_eq!(
        rules("src/lib.rs", "#![allow(deprecated)]"),
        ["allow-deprecated", "crate-wide-allow"]
    );
    assert_eq!(
        rules("src/lib.rs", "    #[allow(deprecated)]"),
        ["allow-deprecated"]
    );
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
    // A reason on the line after the call.
    assert_eq!(
        rules(
            "src/a.rs",
            ".blocked_if(\n    name == \"\",\n    \"Name it first\",\n)"
        ),
        ["reason-end-mark"]
    );
    assert!(
        rules(
            "src/a.rs",
            ".blocked_if(\n    name == \"\",\n    \"Name it first.\",\n)"
        )
        .is_empty()
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
    // The helpers of `sentence!` and a renamed `Sentence`.
    assert_eq!(
        rules(
            "src/a.rs",
            "Sentence::__formatted(check_template(\"Saved.\"), x)"
        ),
        ["outcome-internal"]
    );
    assert_eq!(
        rules("src/a.rs", "use crate::design::outcome::Sentence as Words;"),
        ["outcome-internal"]
    );
    assert!(rules("src/design/outcome.rs", "check_template(t)").is_empty());
    assert_eq!(
        rules("src/a.rs", "Sentence::outside(error)"),
        ["outside-words"]
    );
    assert_eq!(
        rules(
            "src/a.rs",
            "design::outcome::row_fault(ui, &f); PagePlaceholder::new(&f)"
        ),
        ["body-message", "body-message"]
    );
    assert_eq!(
        rules("src/a.rs", "use crate::design::outcome::{banner, Banner};"),
        ["body-message"]
    );
    // A method of the same name elsewhere is not a body place.
    assert!(rules("src/a.rs", "self.banner(ui, x); self.row_fault(ui, p, s);").is_empty());

    // Levels across files: a child under an allow must write its own level.
    let level = |code| file_level(code);
    let levels: BTreeMap<String, FileLevel> = [
        (
            "src/lib.rs",
            level("    #[allow(deprecated)]\n    fn page() {}"),
        ),
        ("src/a.rs", level("#![allow(deprecated)] // old\nmod b;")),
        ("src/a/b.rs", level("use crate::design::message as show;")),
        ("src/a/c.rs", level("#![warn(deprecated)]")),
        ("src/a/d.rs", level("#![allow(deprecated)]")),
        ("src/a/b/e.rs", level("")),
        ("src/f.rs", level("#[allow(deprecated)]\nmod g;")),
        ("src/f/g.rs", level("")),
        ("src/h.rs", level("")),
        ("src/h/mod.rs", level("")),
    ]
    .into_iter()
    .map(|(path, level)| (path.to_owned(), level))
    .collect();
    let inherits: Vec<&str> = levels
        .keys()
        .filter(|path| inherits_allow(path, &levels))
        .map(String::as_str)
        .collect();
    // An alias in `b` and a grandchild `e` inherit; `c` resets; `d` is counted; an allow on a
    // function in `lib.rs` covers no module; an allow on `mod g;` covers `g`.
    assert_eq!(inherits, ["src/a/b.rs", "src/a/b/e.rs", "src/f/g.rs"]);
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
