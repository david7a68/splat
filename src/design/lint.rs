//! No raw colours or text sizes outside the design system, and no straight apostrophe in UI
//! words. The allowlist can only shrink: a count above it fails, and a count below it asks
//! you to lower it.
//!
//! The same allowlist counts the functions that may still make a raw egui call that
//! `clippy.toml` disallows (`#[allow(clippy::disallowed_methods)]`, rule
//! `disallowed-methods-allow`). An allow for a whole module or for a Clippy group would let a
//! new file or a new function through, so it is never allowed (`broad-clippy-allow`).
//!
//! Staged calls are counted one by one, also inside a function whose allow already lets Clippy
//! pass them: `Footer::show_by_hand` (`footer-by-hand`; use `Footer::pane`) and the pane's one
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
    // A layer built by hand, or a menu container from `Default`: the forms Clippy's method list
    // cannot see (clippy.toml).
    let bare = without_strings(code);
    let word_at = |pattern: &str| {
        bare.match_indices(pattern)
            .any(|(at, _)| !bare[..at].ends_with(|c: char| c.is_alphanumeric() || c == '_'))
    };
    if [
        "Order::Foreground",
        "Order::Tooltip",
        "Order::Middle",
        "Order::Debug",
        "Order::Background",
        "LayerId {",
        "MenuBar",
        "SubMenu::",
        "SubMenu {",
    ]
    .into_iter()
    .any(word_at)
    {
        found.push("foreground-layer");
    }
    // `create_reasons!` writes `ALL` from the variants; a hand-made impl could leave one out.
    if bare.contains("CreateReasons for") {
        found.push("hand-create-reasons");
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

/// `code` with the inside of each string literal blanked, so a lint name in words is not a lint.
fn without_strings(code: &str) -> String {
    let mut out = String::with_capacity(code.len());
    let mut inside = false;
    let mut escaped = false;
    for c in code.chars() {
        match c {
            '\\' if inside && !escaped => {
                escaped = true;
                out.push(' ');
                continue;
            }
            '"' if !escaped => {
                inside = !inside;
                out.push('"');
            }
            _ if inside => out.push(' '),
            _ => out.push(c),
        }
        escaped = false;
    }
    out
}

/// Whether `code` names a Clippy lint that can let a raw egui call through: the lint itself, or
/// a group that holds it.
fn names_the_ban(code: &str) -> bool {
    let code: String = code.chars().filter(|c| !c.is_whitespace()).collect();
    code.match_indices("clippy::").any(|(at, _)| {
        let name: String = code[at + "clippy::".len()..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        matches!(name.as_str(), "disallowed_methods" | "style" | "all")
    })
}

/// Whether `line` starts a function item, after its visibility and qualifiers.
fn starts_fn(line: &str) -> bool {
    let mut rest = line.trim_start();
    loop {
        if let Some(after) = rest.strip_prefix("pub") {
            let after = after.trim_start();
            rest = if after.starts_with('(') {
                after
                    .split_once(')')
                    .map_or(after, |(_, tail)| tail)
                    .trim_start()
            } else {
                after
            };
            continue;
        }
        let mut stripped = false;
        for word in ["const ", "async ", "unsafe ", "extern \"C\" "] {
            if let Some(after) = rest.strip_prefix(word) {
                rest = after.trim_start();
                stripped = true;
            }
        }
        if !stripped {
            break;
        }
    }
    rest.starts_with("fn ")
}

/// The one counted allow; on a `fn`, or on the harness's `mod visual_tests;` in `src/lib.rs`.
const ALLOW: &str = "#[allow(clippy::disallowed_methods)]";

/// Every Clippy allow in a file that can let a raw egui call through, by rule. The attribute is
/// read whole, also over several lines, with the item it sits on. Counted
/// (`disallowed-methods-allow`): exactly `#[allow(clippy::disallowed_methods)]`, alone on its
/// line, on a `fn`. Refused (`broad-clippy-allow`): any other form, which frees more than one
/// function or hides from the count: an inner `#![…]` (a module or a block), one on an `impl`,
/// `mod`, `trait`, closure or statement, `expect(…)`, `cfg_attr(…)`, a list with other lints, an
/// attribute over several lines, or a group (`clippy::style`, `clippy::all`).
fn clippy_allows(key: &str, text: &str) -> Vec<&'static str> {
    let lines: Vec<String> = text
        .lines()
        .map(|line| without_strings(line.split("//").next().unwrap_or("")))
        .collect();
    let mut found = Vec::new();
    let mut at = 0;
    while at < lines.len() {
        let start = lines[at].trim_start();
        if !start.starts_with("#[") && !start.starts_with("#![") {
            if names_the_ban(&lines[at]) && lines[at].contains('#') {
                // An attribute in the middle of a line: on an expression or after an item.
                found.push("broad-clippy-allow");
            }
            at += 1;
            continue;
        }
        // The whole attribute: until its brackets close.
        let mut depth = 0i32;
        let mut end = at;
        let mut attribute = String::new();
        loop {
            for c in lines[end].chars() {
                depth += match c {
                    '[' => 1,
                    ']' => -1,
                    _ => 0,
                };
            }
            attribute.push_str(&lines[end]);
            if depth <= 0 || end + 1 >= lines.len() {
                break;
            }
            end += 1;
        }
        if names_the_ban(&attribute) {
            // The item it sits on: the next line that is not an attribute or blank.
            let mut item = end + 1;
            while item < lines.len() {
                let line = lines[item].trim_start();
                if line.is_empty() || line.starts_with("#[") {
                    item += 1;
                } else {
                    break;
                }
            }
            let item = lines.get(item).map_or("", |line| line.trim());
            let exact = end == at && attribute.trim() == ALLOW;
            let harness = key == "src/lib.rs" && item == "mod visual_tests;";
            found.push(if exact && (starts_fn(item) || harness) {
                "disallowed-methods-allow"
            } else {
                "broad-clippy-allow"
            });
        }
        at = end + 1;
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
            for rule in clippy_allows(key, &text)
                .into_iter()
                .chain(text.lines().flat_map(staged_calls))
            {
                *counts.entry(format!("{key} {rule}")).or_default() += 1;
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
                "Write `#[allow(clippy::disallowed_methods)]` alone on one line, on one `fn`"
            } else if key.ends_with("foreground-layer") {
                "Foreground layers and menus belong to src/design/"
            } else if key.ends_with("hand-create-reasons") {
                "Declare a form's reasons with create_reasons!"
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
fn the_lint_finds_staged_calls() {
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
}

#[test]
fn the_lint_finds_clippy_allows() {
    let allows = |text: &str| clippy_allows("src/x.rs", text);
    // The one counted form: alone on its line, on a `fn`, with its docs and other attributes.
    assert_eq!(
        allows("    #[allow(clippy::disallowed_methods)]\n    pub(crate) fn draw() {}"),
        ["disallowed-methods-allow"]
    );
    assert_eq!(
        allows("/// Docs.\n#[allow(clippy::disallowed_methods)]\n#[must_use]\nconst fn f() {}"),
        ["disallowed-methods-allow"]
    );
    // Every other form is refused.
    for text in [
        // A module, a file, a block.
        "#![allow(clippy::disallowed_methods)]\nfn f() {}",
        "fn f() {\n    #![allow(clippy::disallowed_methods)]\n}",
        "#[allow(clippy::disallowed_methods)]\nmod screens;",
        // A whole `impl` frees every method in it.
        "#[allow(clippy::disallowed_methods)]\nimpl Library {",
        "#[allow(clippy::disallowed_methods)]\ntrait Draw {",
        // A statement or a closure.
        "    #[allow(clippy::disallowed_methods)]\n    let area = egui::ScrollArea::vertical();",
        "    let area = #[allow(clippy::disallowed_methods)] egui::ScrollArea::vertical();",
        // Other spellings.
        "#[expect(clippy::disallowed_methods)]\nfn f() {}",
        "#[cfg_attr(test, allow(clippy::disallowed_methods))]\nfn f() {}",
        "#[allow(dead_code, clippy::disallowed_methods)]\nfn f() {}",
        "#[allow(\n    clippy::disallowed_methods\n)]\nfn f() {}",
        "#[allow(clippy :: disallowed_methods)]\nfn f() {}",
        // A group that holds the lint.
        "#[allow(clippy::style)]\nfn f() {}",
        "#[expect(clippy::all, reason = \"x\")]\nfn f() {}",
    ] {
        assert_eq!(allows(text), ["broad-clippy-allow"], "{text}");
    }
    // The capture harness's one exception is in `src/lib.rs` only.
    let harness = "#[cfg(test)]\n#[allow(clippy::disallowed_methods)]\nmod visual_tests;";
    assert_eq!(
        clippy_allows("src/lib.rs", harness),
        ["disallowed-methods-allow"]
    );
    assert_eq!(allows(harness), ["broad-clippy-allow"]);
    // Words and comments are not lints, and another Clippy lint is not this one.
    assert!(allows("// #[allow(clippy::disallowed_methods)] in a comment").is_empty());
    assert!(allows("let lint = \"#[allow(clippy::disallowed_methods)]\";").is_empty());
    assert!(allows("#[allow(clippy::allow_attributes)]\nimpl X {").is_empty());
    assert!(allows("#[allow(clippy::too_many_arguments)]\nimpl X {").is_empty());
}

#[test]
fn the_lint_finds_hand_made_layers_and_reasons() {
    for code in [
        "let layer = egui::LayerId { order: egui::Order::Foreground, id };",
        "let layer = LayerId { order: Order::Tooltip, id };",
        "let bar = egui::containers::menu::MenuBar::default();",
        "let menu = SubMenu::default();",
    ] {
        assert_eq!(violations(code), ["foreground-layer"], "{code}");
    }
    assert!(violations("let order = SortOrder::Foreground;").is_empty());
    assert!(violations("let order = ResultOrder::issue();").is_empty());
    assert_eq!(
        violations("impl design::CreateReasons for Why {"),
        ["hand-create-reasons"]
    );
}

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
