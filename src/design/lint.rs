//! No raw colours or text sizes outside the design system, and no straight apostrophe in UI
//! words. The allowlist can only shrink: a count above it fails, and a count below it asks
//! you to lower it.
use super::attributes::{self, Found, Kind};
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

/// Lints whose allow lets an old message function (`#[deprecated]`) pass.
const DEPRECATED: [&str; 2] = ["deprecated", "warnings"];

/// How a file sets the `deprecated` level for the modules below it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct FileLevel {
    /// An inner allow at the top of the file, or an allow on a `mod x;`: its children inherit it.
    allows_children: bool,
    /// An inner attribute that sets the level for this whole file: allow, or warn or deny.
    explicit: bool,
}

fn file_level(found: &[Found]) -> FileLevel {
    let mut level = FileLevel::default();
    for attribute in found {
        let top = attribute.inner && attribute.kind == Kind::File;
        if top && (attribute.lets_pass(&DEPRECATED) || attribute.resets(&DEPRECATED)) {
            level.explicit = true;
        }
        let on_mod_file = !attribute.inner && attribute.kind == Kind::Mod { has_file: true };
        if attribute.lets_pass(&DEPRECATED) && (top || on_mod_file) {
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

/// Blocked reasons without an end mark, read by `syn`: the last non-empty string literal in the
/// arguments of a `.blocked(…)` or `.blocked_if(…)` method call or a `blocked_reason(…)` call,
/// on lines up to `last_line`.
fn reasons_without_end_mark(text: &str, last_line: usize) -> usize {
    use syn::visit::{self, Visit};
    struct Literals(Vec<String>);
    impl<'ast> Visit<'ast> for Literals {
        fn visit_lit_str(&mut self, literal: &'ast syn::LitStr) {
            self.0.push(literal.value());
        }
    }
    struct Reasons {
        last_line: usize,
        bad: usize,
    }
    impl Reasons {
        fn check<'a>(&mut self, line: usize, args: impl Iterator<Item = &'a syn::Expr>) {
            if line > self.last_line {
                return;
            }
            let mut literals = Literals(Vec::new());
            for arg in args {
                literals.visit_expr(arg);
            }
            if literals
                .0
                .iter()
                .rfind(|words| !words.is_empty())
                .is_some_and(|words| {
                    !words
                        .trim_end_matches(['”', '’', ')'])
                        .ends_with(['.', '?', '!'])
                })
            {
                self.bad += 1;
            }
        }
    }
    impl<'ast> Visit<'ast> for Reasons {
        fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
            if call.method == "blocked" || call.method == "blocked_if" {
                self.check(call.method.span().start().line, call.args.iter());
            }
            visit::visit_expr_method_call(self, call);
        }
        fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
            if let syn::Expr::Path(path) = &*call.func
                && path
                    .path
                    .segments
                    .last()
                    .is_some_and(|s| s.ident == "blocked_reason")
            {
                use syn::spanned::Spanned;
                self.check(call.span().start().line, call.args.iter());
            }
            visit::visit_expr_call(self, call);
        }
    }
    let Ok(file) = syn::parse_file(text) else {
        return 0;
    };
    let mut reasons = Reasons { last_line, bad: 0 };
    reasons.visit_file(&file);
    reasons.bad
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
        // A message's close icon (N6): `MessageChoice::Dismiss` names and places it.
        (
            words.starts_with("Dismiss this ") || words.starts_with("Dismiss the "),
            "src/design/outcome.rs",
        ),
    ];
    owned
        .iter()
        .any(|(found, owner)| *found && !path.ends_with(owner))
}

/// The message rules of one file (package M, round 28), on its code before an inline test
/// module (`code`, the first `last_line` lines of `text`). Attributes and calls are read by
/// `syn`, never matched as text; the escapes of `design::outcome` are Clippy bans counted in
/// `design::results::STAGED_ALLOWS`. Each rule here is a count per file in the allowlist.
/// "Shrink only" holds for a count alone: a change that raises a count and its allowance
/// together passes this test, so a reviewer reads every raised line (AGENTS.md).
///
/// - `allow-deprecated`: each attribute that lets `deprecated` (or `warnings`) pass, in any
///   form: inner or outer, a list across lines, `cfg_attr`.
/// - `crate-wide-allow`: such an inner attribute at a crate root, which would cover every module.
/// - `reason-end-mark`: a blocked reason without `.`, `?` or `!`.
/// - `phrase-owner`: a phrase in a string literal outside its owner file.
fn message_rules(path: &str, text: &str, last_line: usize) -> Vec<&'static str> {
    let mut found = Vec::new();
    let crate_root = path.ends_with("src/lib.rs") || path.ends_with("src/main.rs");
    for attribute in attributes::read(text).unwrap_or_default() {
        if attribute.line <= last_line && attribute.lets_pass(&DEPRECATED) {
            found.push("allow-deprecated");
            if attribute.inner && attribute.kind == Kind::File && crate_root {
                found.push("crate-wide-allow");
            }
        }
    }
    for line in text.lines().take(last_line) {
        let line = line.split("//").next().unwrap_or("");
        if string_literals(line)
            .into_iter()
            .any(|words| phrase_owner(path, words))
        {
            found.push("phrase-owner");
        }
    }
    found.extend((0..reasons_without_end_mark(text, last_line)).map(|_| "reason-end-mark"));
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
            if !relative.ends_with("src/design") && !relative.ends_with("src/visual_tests") {
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
            for rule in message_rules(key, &text, code.lines().count()) {
                *counts.entry(format!("{key} {rule}")).or_default() += 1;
            }
            // Package K: a layer built by hand, a `CreateReasons` impl by hand (the syntax tree).
            for rule in super::layout::bans::syntax_rules(&text)
                .unwrap_or_else(|error| panic!("{key} does not parse: {error}"))
            {
                *counts.entry(format!("{key} {rule}")).or_default() += 1;
            }
            let found = attributes::read(&text)
                .unwrap_or_else(|error| panic!("{key} does not parse: {error}"));
            levels.insert(key.to_owned(), file_level(&found));
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
                "foreground-layer" => "Foreground layers and menus belong to src/design/",
                "hand-create-reasons" => "Declare a form's reasons with `create_reasons!`",
                "banned-alias" => {
                    "Name a banned owner or function as clippy.toml names it: an alias (`use … as …`, `type … =`) hides it from the counts"
                }
                "phrase-owner" => {
                    "Call the phrase's owner: `XP {` in src/difficulty.rs, `· line {` in `fault_words`, `N of M chosen` in `chosen_count`, a close icon's `Dismiss this …` in `MessageChoice::Dismiss`"
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
    let rules = |path: &str, code: &str| message_rules(path, code, code.lines().count());
    assert_eq!(
        rules("src/a.rs", "#![allow(deprecated)]"),
        ["allow-deprecated"]
    );
    // A list across lines, as a new module would write it: counted, so it fails the lint until
    // the allowlist names it.
    assert_eq!(
        rules(
            "src/new.rs",
            "#![allow(\n    deprecated,\n    reason = \"old message functions\"\n)]\nfn f() {}"
        ),
        ["allow-deprecated"]
    );
    // A combined list, `cfg_attr` and `warnings` are allows too; another lint alone is not.
    assert_eq!(
        rules("src/a.rs", "#![allow(deprecated, dead_code)]"),
        ["allow-deprecated"]
    );
    assert_eq!(
        rules(
            "src/a.rs",
            "#[cfg_attr(test, allow(dead_code, deprecated))]\nfn f() {}"
        ),
        ["allow-deprecated"]
    );
    assert_eq!(
        rules("src/a.rs", "#[allow(warnings)]\nfn f() {}"),
        ["allow-deprecated"]
    );
    assert!(rules("src/a.rs", "#![allow(dead_code)]").is_empty());
    assert!(
        rules(
            "src/a.rs",
            "// #![allow(deprecated)]\nconst A: &str = \"#![allow(deprecated)]\";"
        )
        .is_empty()
    );
    assert_eq!(
        rules("src/lib.rs", "#![allow(deprecated)]"),
        ["allow-deprecated", "crate-wide-allow"]
    );
    assert_eq!(
        rules(
            "src/lib.rs",
            "impl A {\n    #[allow(deprecated)]\n    fn page(&self) {}\n}"
        ),
        ["allow-deprecated"]
    );
    let reason = |code: &str| rules("src/a.rs", &format!("fn f() {{\n{code}\n}}"));
    assert_eq!(
        reason("Action::quiet(\"Save\").blocked(Some(\"Stop recording first\"));"),
        ["reason-end-mark"]
    );
    assert!(reason("Action::quiet(\"Save\").blocked(Some(\"Stop recording first.\"));").is_empty());
    assert_eq!(reason("x.blocked_if(busy, \"Wait\");"), ["reason-end-mark"]);
    // A reason on the line after the call, and a call through a qualified path.
    assert_eq!(
        reason("x.blocked_if(\n    name == \"\",\n    \"Name it first\",\n);"),
        ["reason-end-mark"]
    );
    assert!(reason("x.blocked_if(\n    name == \"\",\n    \"Name it first.\",\n);").is_empty());
    assert_eq!(
        reason("crate::design::blocked_reason(ui, Some(\"Wait\"));"),
        ["reason-end-mark"]
    );
    assert_eq!(
        rules("src/a.rs", "const A: &str = \"XP {xp}\";"),
        ["phrase-owner"]
    );
    assert!(rules("src/difficulty.rs", "const A: &str = \"XP {xp}\";").is_empty());
    // N6: a close icon named by hand, outside `MessageChoice::Dismiss`.
    assert_eq!(
        rules(
            "src/a.rs",
            "fn f() { icon_button(ui, icons::CLOSE, \"Dismiss this message\"); }"
        ),
        ["phrase-owner"]
    );
    assert!(rules("src/a.rs", "const A: &str = \"Dismiss\";").is_empty());
    assert_eq!(
        rules(
            "src/a.rs",
            "const A: &str = \"{name} · line {n}: {cause}\";"
        ),
        ["phrase-owner"]
    );
    assert_eq!(
        rules("src/a.rs", "const A: &str = \"{a} of {b} chosen\";"),
        ["phrase-owner"]
    );

    // Levels across files: a child under an allow must write its own level.
    let level = |code: &str| file_level(&attributes::read(code).unwrap());
    let levels: BTreeMap<String, FileLevel> = [
        (
            "src/lib.rs",
            level("impl A {\n    #[allow(deprecated)]\n    fn page() {}\n}"),
        ),
        (
            "src/a.rs",
            level("#![allow(\n    deprecated,\n    reason = \"old\"\n)]\nmod b;"),
        ),
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

/// Package K: two fields with one `FieldId` name would share focus and text state; each name is
/// used once (`field_id!` names, read from the syntax tree, in all of `src/`). A field shown once
/// per row adds the row's key with `FieldId::with`; two live widgets with one id are the gate's
/// `widget-id-clash` check.
#[test]
fn each_field_id_name_is_used_once() {
    fn names(folder: &Path, found: &mut BTreeMap<String, Vec<String>>) {
        for item in fs::read_dir(folder).expect("read source folder") {
            let path = item.expect("read source entry").path();
            if path.is_dir() {
                names(&path, found);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = fs::read_to_string(&path).expect("read source file");
                let key = path.display().to_string();
                for name in super::layout::bans::field_id_names(&text)
                    .unwrap_or_else(|error| panic!("{key} does not parse: {error}"))
                {
                    found.entry(name).or_default().push(key.clone());
                }
            }
        }
    }
    let mut found = BTreeMap::new();
    names(
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

/// Package K: no Clippy level is set outside the source, where no row could count it.
#[test]
fn no_lint_level_frees_raw_calls_outside_the_source() {
    let root = env!("CARGO_MANIFEST_DIR");
    for file in ["Cargo.toml", "crates/core/Cargo.toml", ".cargo/config.toml"] {
        let text = fs::read_to_string(format!("{root}/{file}")).unwrap_or_default();
        assert!(
            !text.contains("disallowed_methods") && !text.contains("-A clippy"),
            "{file} sets a Clippy level; set it on one `fn` instead"
        );
    }
    let devenv = fs::read_to_string(format!("{root}/devenv.nix")).expect("devenv.nix");
    let check = devenv
        .lines()
        .find(|line| line.contains("scripts.check.exec"))
        .expect("the check script");
    assert!(
        check.contains("-D warnings") && !check.contains("-A "),
        "{check}"
    );
}
