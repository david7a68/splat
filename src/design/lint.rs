//! No raw colours or text sizes outside the design system, and no straight apostrophe in UI
//! words. The allowlist can only shrink: a count above it fails, and a count below it asks
//! you to lower it.
//!
//! The same allowlist counts, per file, the banned egui names of `clippy.toml` as text
//! (`raw-egui-call`), so a second raw call inside a function whose suppression
//! `design::results::STAGED_ALLOWS` lists is seen; and the row keys that take any number or text
//! (`row-key`). The suppressions themselves are `STAGED_ALLOWS`'s and `OWNED_ALLOWS`'s rows.
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

/// The banned egui names the lint counts per file, as text (`raw-egui-call`), beside the count of
/// allows. An allow on a function frees every call inside it, also in a nested function or a
/// closure, so a second raw call in an allowed function raises this count and fails. The set
/// matches `clippy.toml`; `UiBuilder::layer_id` is the one ban it cannot tell apart from the
/// `Ui::layer_id` getter by its words, so Clippy alone holds that one.
pub(crate) const RAW_CALLS: [&str; 55] = [
    "ScrollArea::",
    "TextEdit::",
    ".text_edit_singleline(",
    ".text_edit_multiline(",
    ".code_editor(",
    "text_field(",
    "text_input(",
    "DragValue::",
    "Slider::",
    "Word::checked",
    "FieldId::checked",
    "Area::new",
    "Window::new",
    "Modal::",
    "LayerId::",
    "layer_painter",
    "debug_painter",
    "Painter::new",
    "Ui::new",
    ".with_layer_id(",
    "Popup::",
    ".menu_button(",
    ".menu_image_button(",
    ".menu_image_text_button(",
    ".context_menu(",
    "MenuButton::",
    "SubMenuButton::",
    "SubMenu::",
    "MenuBar",
    "ComboBox::",
    "Tooltip::",
    "on_hover_ui",
    "on_disabled_hover_ui",
    "show_tooltip",
    "popup_below_widget",
    "popup_above_or_below_widget",
    "dnd_drag_source",
    "dnd_drop_zone",
    "dnd_set_drag_payload",
    "debug_paint_cursor",
    "paint_debug_info",
    "Scene::new",
    "color_edit_button",
    "egui::menu::",
    "open_popup",
    "toggle_popup",
    "move_to_top",
    "set_sublayer",
    "set_transform_layer",
    "settings_ui",
    "inspection_ui",
    "texture_ui",
    "memory_ui",
    "style_ui",
    "show_viewport_",
];

/// The counted names on one line of code, by rule: each banned egui name (`raw-egui-call`), and
/// each key that takes any number or text (`row-key`: `RowIndex(`, `TextKey::`).
fn counted_names(line: &str) -> Vec<&'static str> {
    let code = without_strings(line.split("//").next().unwrap_or(""));
    let starts_word = |at: usize| !code[..at].ends_with(|c: char| c.is_alphanumeric() || c == '_');
    let mut found = Vec::new();
    for name in RAW_CALLS {
        let word = !name.starts_with('.');
        for (at, _) in code.match_indices(name) {
            if !word || starts_word(at) {
                found.push("raw-egui-call");
            }
        }
    }
    for name in ["RowIndex(", "TextKey::"] {
        for (at, _) in code.match_indices(name) {
            if starts_word(at) {
                found.push("row-key");
            }
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
            let root = concat!(env!("CARGO_MANIFEST_DIR"), "/");
            let key = relative.strip_prefix(root).unwrap_or(&relative);
            // Each banned name counts, in tests too: a second raw call in a function whose allow
            // `design::results::STAGED_ALLOWS` lists raises the count.
            for rule in text.lines().flat_map(counted_names) {
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
            } else if key.ends_with("raw-egui-call") {
                "A new raw egui call, also inside an allowed function: use the design part that clippy.toml names"
            } else if key.ends_with("row-key") {
                "A new RowIndex or TextKey: give it the row's data, never a value read from the layout"
            } else if key.ends_with("foreground-layer") {
                "Foreground layers and menus belong to src/design/"
            } else if key.ends_with("hand-create-reasons") {
                "Declare a form's reasons with create_reasons!"
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
fn the_lint_counts_raw_names_and_row_keys() {
    let count = |code: &str| counted_names(code);
    assert_eq!(
        count("let a = egui::ScrollArea::vertical(); let b = egui::ScrollArea::both();"),
        ["raw-egui-call"; 2]
    );
    assert_eq!(count("ui.menu_button(\"m\", add);"), ["raw-egui-call"]);
    assert_eq!(
        count("ui.color_edit_button_srgba(&mut c);"),
        ["raw-egui-call"]
    );
    assert_eq!(count("ui.dnd_drag_source(id, 1, add);"), ["raw-egui-call"]);
    // The design's own parts, words in a string and a comment are not raw calls.
    assert!(count("design::menu_button(ui, \"Sort\", true);").is_empty());
    assert!(count("design::labelled_field(ui, NAME, \"Name\", &mut text, None, None);").is_empty());
    assert!(count("let words = \"ScrollArea::vertical\"; // TextEdit::singleline").is_empty());
    assert!(count("let output: ScrollAreaOutput<()> = column.show(ui, add);").is_empty());
    assert_eq!(
        count("NAME.with(RowIndex(at)); NAME.with(TextKey::from_data(name));"),
        ["row-key"; 2]
    );
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
    assert!(violations("let order = ResultOrder::default();").is_empty());
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
