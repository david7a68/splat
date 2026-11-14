//! Package K's source checks (tests only), read from Rust syntax, never from text.
//!
//! - [`count`]: how many of `clippy.toml`'s package K paths an item calls. The paths come from
//!   `clippy.toml` itself, so the count and Clippy cannot list different bans. The item is read
//!   as tokens by `proc_macro2` (the lexer `syn` uses), so words in a string or a comment never
//!   count. `design::results::STAGED_ALLOWS` and `OWNED_ALLOWS` give each suppression's row the
//!   number its item may hold (`Ban::Layout(n)`, `Ban::RowKey(n)`): a second raw call inside an
//!   allowed function raises the count and fails.
//! - [`syntax_rules`]: a foreground layer built by hand, which no method ban can see
//!   (`LayerId { order, id }`, an `egui::Order` value, the menu containers made by `Default`), and
//!   a `CreateReasons` impl by hand, which could leave a reason out of `ALL`.
//! - [`field_id_names`]: every `field_id!` name, for the check that each is used once.
use std::str::FromStr;

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use syn::visit::{self, Visit};

const CLIPPY: &str = include_str!("../../../clippy.toml");

/// Which counted ban a path belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Group {
    /// A raw egui call that a layout part replaces.
    Layout,
    /// A row key a reviewer checks: `RowIndex::from_data`, `TextKey::from_data`.
    RowKey,
    /// Package P's paths (`clippy.toml`, "# Package P"): a footer placed by hand, the pane's
    /// escape `Fills::because`, and the layouts that place content by the room's height.
    Pane,
    /// Package X: raw space, raw selectable choices, a search field without `Clear`, a heading
    /// that does not add its own space (`clippy.toml`, section "Package X").
    Extras,
    /// Package M: a footer press undone or used by owner (`Results::undo`, `Results::use_press`),
    /// which could reach a newer result of that owner. Counted by method name and at least its
    /// number of arguments (`.undo(owner, run)`, `.use_press(owner)`), so a message's
    /// `.undo(step)` does not count.
    ByOwner,
}

/// The `clippy.toml` lines of one package's section: from its `# Package <letter>` line to the
/// next package's, or to the end.
fn section(letter: char) -> &'static str {
    let head = format!("# Package {letter}");
    let start = CLIPPY
        .find(&head)
        .unwrap_or_else(|| panic!("clippy.toml has package {letter}'s list"));
    let end = CLIPPY[start + head.len()..]
        .find("# Package ")
        .map_or(CLIPPY.len(), |at| start + head.len() + at);
    &CLIPPY[start..end]
}

/// How a banned path is written at a call.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Shape {
    /// `Type::name`, also `Type::name::<T>`.
    Assoc(String, String),
    /// `.name(…)`: a method of a value (`ui.menu_button(…)`).
    Method(String),
    /// `name(…)` or `module::name`: a free function.
    Free(Option<String>, String),
    /// `Owner::ctor(…)` then `.x(…)` calls up to `.name(…)`: a builder method of a type that is
    /// not a `RECEIVERS` value (`egui::Button::new(…).selected(…)`).
    Chain(String, String),
    /// `.name(…)` with at least this many arguments (a turbofish's comma can count as one more).
    Arity(String, usize),
}

/// Builder types whose banned methods are counted in a constructor's chain.
const BUILDERS: [&str; 1] = ["Button"];

/// The egui types whose banned functions are methods of a value.
const RECEIVERS: [&str; 8] = [
    "Footer",
    "Ui",
    "Response",
    "Context",
    "Memory",
    "Areas",
    "UiBuilder",
    "TextEdit",
];

/// Method names too common to count by name alone (`.id(…)` is on many builders). Clippy holds
/// these bans; the count does not see them.
const COMMON: [&str; 4] = ["id", "id_salt", "id_source", "layer_id"];

/// The free functions of egui's deprecated `menu` module, whose names are common.
const MENU_MODULE: [&str; 6] = [
    "bar",
    "context_menu",
    "menu_button",
    "menu_custom_button",
    "menu_image_button",
    "submenu_button",
];

/// Every package K and package X path of `clippy.toml`, as the shapes it is written in at a call.
fn patterns() -> Vec<(Group, Shape)> {
    let mut out = Vec::new();
    for (letter, lines) in [
        ('K', section('K')),
        ('X', section('X')),
        ('P', section('P')),
    ] {
        shapes(letter, lines, &mut out);
    }
    for (name, args) in [
        ("undo", 2),
        ("use_press", 1),
        ("revise", 2),
        ("revise_where", 2),
    ] {
        out.push((Group::ByOwner, Shape::Arity(name.to_owned(), args)));
        out.push((
            Group::ByOwner,
            Shape::Assoc("Results".to_owned(), name.to_owned()),
        ));
    }
    out
}

/// The shapes of one section's paths.
fn shapes(letter: char, lines: &str, out: &mut Vec<(Group, Shape)>) {
    for line in lines.lines() {
        let Some(path) = line
            .split("path = \"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
        else {
            continue;
        };
        let segments: Vec<&str> = path.split("::").collect();
        let name = segments[segments.len() - 1].to_owned();
        let owner = segments[segments.len() - 2];
        let group = if letter == 'X' {
            Group::Extras
        } else if letter == 'P' {
            Group::Pane
        } else if matches!(owner, "RowIndex" | "TextKey") {
            Group::RowKey
        } else {
            Group::Layout
        };
        if owner.starts_with(|c: char| c.is_ascii_uppercase()) {
            if RECEIVERS.contains(&owner)
                && name != "new"
                && name != "singleline"
                && name != "multiline"
            {
                if !COMMON.contains(&name.as_str()) {
                    out.push((group, Shape::Method(name.clone())));
                }
                // The same function called by its path: `egui::Ui::menu_button(ui, …)`.
                out.push((group, Shape::Assoc(owner.to_owned(), name)));
            } else {
                if BUILDERS.contains(&owner) {
                    out.push((group, Shape::Chain(owner.to_owned(), name.clone())));
                }
                out.push((group, Shape::Assoc(owner.to_owned(), name)));
            }
        } else if owner == "menu" && MENU_MODULE.contains(&name.as_str()) {
            out.push((group, Shape::Free(Some("menu".to_owned()), name)));
        } else {
            out.push((group, Shape::Free(None, name)));
        }
    }
}

/// One token of a flattened stream.
#[derive(Debug, PartialEq)]
enum Token {
    Ident(String),
    Punct(char),
    Open(Delimiter),
    Close,
    Literal,
}

fn flatten(stream: TokenStream, out: &mut Vec<Token>) {
    for tree in stream {
        match tree {
            TokenTree::Ident(ident) => out.push(Token::Ident(ident.to_string())),
            TokenTree::Punct(punct) => out.push(Token::Punct(punct.as_char())),
            TokenTree::Literal(_) => out.push(Token::Literal),
            TokenTree::Group(group) => {
                out.push(Token::Open(group.delimiter()));
                flatten(group.stream(), out);
                out.push(Token::Close);
            }
        }
    }
}

fn ident(token: Option<&Token>) -> Option<&str> {
    match token {
        Some(Token::Ident(word)) => Some(word),
        _ => None,
    }
}

/// `::` at `at`: two `:` puncts.
fn path_sep(tokens: &[Token], at: usize) -> bool {
    tokens.get(at) == Some(&Token::Punct(':')) && tokens.get(at + 1) == Some(&Token::Punct(':'))
}

/// The index after the group that opens at `at` (an `Open` token), or `None`.
fn after_group(tokens: &[Token], at: usize) -> Option<usize> {
    if !matches!(tokens.get(at), Some(Token::Open(_))) {
        return None;
    }
    let mut depth = 0usize;
    for (offset, token) in tokens[at..].iter().enumerate() {
        match token {
            Token::Open(_) => depth += 1,
            Token::Close => {
                depth -= 1;
                if depth == 0 {
                    return Some(at + offset + 1);
                }
            }
            _ => {}
        }
    }
    None
}

/// `Owner::ctor(…)` at `at`, then method calls, one of them `.name(…)`.
fn chain_hits(tokens: &[Token], at: usize, owner: &str, name: &str) -> bool {
    if ident(tokens.get(at)) != Some(owner) || !path_sep(tokens, at + 1) {
        return false;
    }
    let Some(mut next) = after_group(tokens, at + 4) else {
        return false;
    };
    while tokens.get(next) == Some(&Token::Punct('.')) {
        let Some(method) = ident(tokens.get(next + 1)) else {
            return false;
        };
        if method == name {
            return true;
        }
        match after_group(tokens, next + 2) {
            Some(after) => next = after,
            None => return false,
        }
    }
    false
}

/// The index after a turbofish (`::<…>`) that starts at `at`, or `at` when there is none. The
/// brackets are counted, so `::<Vec<u8>, ()>` is skipped whole; the `>` of an `->` is not one.
fn after_turbofish(tokens: &[Token], at: usize) -> usize {
    if !path_sep(tokens, at) || tokens.get(at + 2) != Some(&Token::Punct('<')) {
        return at;
    }
    let mut depth = 0usize;
    for index in at + 2..tokens.len() {
        match &tokens[index] {
            Token::Punct('<') => depth += 1,
            Token::Punct('>') if tokens.get(index - 1) != Some(&Token::Punct('-')) => {
                depth -= 1;
                if depth == 0 {
                    return index + 1;
                }
            }
            _ => {}
        }
    }
    at
}

/// The number of arguments of the call whose parentheses open at `at`: its commas at the top
/// level, outside a closure's `|…|`, and one more unless it is empty or ends in a comma.
fn arguments(tokens: &[Token], at: usize) -> Option<usize> {
    if tokens.get(at) != Some(&Token::Open(Delimiter::Parenthesis)) {
        return None;
    }
    let end = after_group(tokens, at)? - 1;
    let (mut depth, mut pipes, mut commas) = (0usize, 0usize, 0usize);
    for token in &tokens[at + 1..end] {
        match token {
            Token::Open(_) => depth += 1,
            Token::Close => depth -= 1,
            Token::Punct('|') if depth == 0 => pipes += 1,
            Token::Punct(',') if depth == 0 && pipes % 2 == 0 => commas += 1,
            _ => {}
        }
    }
    let inner = &tokens[at + 1..end];
    Some(match inner.last() {
        None => 0,
        Some(Token::Punct(',')) => commas,
        Some(_) => commas + 1,
    })
}

/// How many calls of `group`'s paths `source` holds. `Err` when it does not lex.
pub(crate) fn count(source: &str, group: Group) -> Result<usize, String> {
    let stream = TokenStream::from_str(source).map_err(|error| error.to_string())?;
    let mut tokens = Vec::new();
    flatten(stream, &mut tokens);
    let patterns: Vec<Shape> = patterns()
        .into_iter()
        .filter(|(of, _)| *of == group)
        .map(|(_, shape)| shape)
        .collect();
    let mut found = 0;
    for at in 0..tokens.len() {
        for shape in &patterns {
            let hit = match shape {
                Shape::Assoc(owner, name) => {
                    // `Owner::name`, also `Owner::<T>::name`.
                    let after = after_turbofish(&tokens, at + 1);
                    ident(tokens.get(at)) == Some(owner)
                        && path_sep(&tokens, after)
                        && ident(tokens.get(after + 2)) == Some(name)
                }
                Shape::Method(name) => {
                    tokens.get(at) == Some(&Token::Punct('.'))
                        && ident(tokens.get(at + 1)) == Some(name)
                        && (tokens.get(at + 2) == Some(&Token::Open(Delimiter::Parenthesis))
                            || path_sep(&tokens, at + 2))
                }
                Shape::Free(Some(module), name) => {
                    ident(tokens.get(at)) == Some(module)
                        && path_sep(&tokens, at + 1)
                        && ident(tokens.get(at + 3)) == Some(name)
                }
                Shape::Chain(owner, name) => chain_hits(&tokens, at, owner, name),
                Shape::Arity(name, args) => {
                    tokens.get(at) == Some(&Token::Punct('.'))
                        && ident(tokens.get(at + 1)) == Some(name)
                        && arguments(&tokens, after_turbofish(&tokens, at + 2))
                            .is_some_and(|found| found >= *args)
                }
                Shape::Free(None, name) => {
                    ident(tokens.get(at)) == Some(name)
                        && tokens.get(at + 1) == Some(&Token::Open(Delimiter::Parenthesis))
                        && at.checked_sub(1).and_then(|before| tokens.get(before))
                            != Some(&Token::Punct('.'))
                        && ident(at.checked_sub(1).and_then(|before| tokens.get(before)))
                            != Some("fn")
                }
            };
            if hit {
                found += 1;
            }
        }
    }
    Ok(found)
}

/// The rules [`syntax_rules`] can find.
const ORDERS: [&str; 5] = ["Foreground", "Tooltip", "Middle", "Debug", "Background"];

/// Every banned path of `clippy.toml`, all packages' lists, by its last two segments: the
/// owner or module and the name. A renaming import or a type alias of one would hide it from
/// the counters, which match names in an item's tokens.
fn banned_paths() -> Vec<(String, String)> {
    CLIPPY
        .lines()
        .filter_map(|line| line.split("path = \"").nth(1)?.split('"').next())
        .filter_map(|path| {
            let segments: Vec<&str> = path.split("::").collect();
            let [.., owner, name] = segments.as_slice() else {
                return None;
            };
            Some(((*owner).to_owned(), (*name).to_owned()))
        })
        .collect()
}

/// Whether `path` (an import's or an alias's, segment by segment) names a banned owner (a type
/// whose functions `clippy.toml` bans: `Fills`, `Footer`, `ScrollArea`) or a banned function by
/// its owner or module (`Fills::because`, `outcome::banner`).
fn names_a_ban(path: &[String], bans: &[(String, String)]) -> bool {
    let Some(last) = path.last() else {
        return false;
    };
    let owner = bans
        .iter()
        .any(|(owner, _)| owner == last && owner.starts_with(|c: char| c.is_ascii_uppercase()));
    let function = path.len() >= 2
        && bans
            .iter()
            .any(|(owner, name)| name == last && *owner == path[path.len() - 2]);
    owner || function
}

#[derive(Default)]
struct Rules {
    found: Vec<&'static str>,
    bans: Vec<(String, String)>,
}

impl Rules {
    /// A renaming import (`use … as …`) of a banned owner or function, anywhere in `tree`.
    fn renamed(&mut self, tree: &syn::UseTree, prefix: &mut Vec<String>) {
        match tree {
            syn::UseTree::Path(path) => {
                prefix.push(path.ident.to_string());
                self.renamed(&path.tree, prefix);
                prefix.pop();
            }
            syn::UseTree::Rename(rename) => {
                prefix.push(rename.ident.to_string());
                if names_a_ban(prefix, &self.bans) {
                    self.found.push("banned-alias");
                }
                prefix.pop();
            }
            syn::UseTree::Group(group) => {
                for tree in &group.items {
                    self.renamed(tree, prefix);
                }
            }
            syn::UseTree::Name(_) | syn::UseTree::Glob(_) => {}
        }
    }
}

impl<'ast> Visit<'ast> for Rules {
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        self.renamed(&node.tree, &mut Vec::new());
        visit::visit_item_use(self, node);
    }

    fn visit_item_type(&mut self, node: &'ast syn::ItemType) {
        if let syn::Type::Path(alias) = &*node.ty {
            let path: Vec<String> = alias
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect();
            if names_a_ban(&path, &self.bans) {
                self.found.push("banned-alias");
            }
        }
        visit::visit_item_type(self, node);
    }

    fn visit_expr_struct(&mut self, node: &'ast syn::ExprStruct) {
        if node
            .path
            .segments
            .last()
            .is_some_and(|last| last.ident == "LayerId")
        {
            self.found.push("foreground-layer");
        }
        visit::visit_expr_struct(self, node);
    }

    fn visit_path(&mut self, node: &'ast syn::Path) {
        let names: Vec<String> = node.segments.iter().map(|s| s.ident.to_string()).collect();
        let order = names
            .windows(2)
            .any(|pair| pair[0] == "Order" && ORDERS.contains(&pair[1].as_str()));
        let menu = names
            .iter()
            .any(|name| name == "MenuBar" || name == "SubMenu");
        if order || menu {
            self.found.push("foreground-layer");
        }
        visit::visit_path(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        if node.trait_.as_ref().is_some_and(|(_, path, _)| {
            path.segments
                .last()
                .is_some_and(|last| last.ident == "CreateReasons")
        }) {
            self.found.push("hand-create-reasons");
        }
        visit::visit_item_impl(self, node);
    }
}

/// K's rules that only the syntax tree can see, once per finding (see the module's doc).
pub(crate) fn syntax_rules(text: &str) -> Result<Vec<&'static str>, String> {
    let file = syn::parse_file(text).map_err(|error| error.to_string())?;
    let mut rules = Rules {
        bans: banned_paths(),
        ..Rules::default()
    };
    rules.visit_file(&file);
    Ok(rules.found)
}

#[derive(Default)]
struct Names {
    found: Vec<String>,
}

impl<'ast> Visit<'ast> for Names {
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        if node
            .path
            .segments
            .last()
            .is_some_and(|last| last.ident == "field_id")
            && let Ok(name) = node.parse_body::<syn::LitStr>()
        {
            self.found.push(name.value());
        }
        visit::visit_macro(self, node);
    }
}

/// Every `field_id!` name of a file.
pub(crate) fn field_id_names(text: &str) -> Result<Vec<String>, String> {
    let file = syn::parse_file(text).map_err(|error| error.to_string())?;
    let mut names = Names::default();
    names.visit_file(&file);
    Ok(names.found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_package_k_path_has_a_shape() {
        let all = patterns();
        // Every path of the K and X lists, less the four common method names counted by path
        // only.
        let paths = [section('K'), section('X')]
            .iter()
            .map(|lines| lines.matches("path = \"").count())
            .sum::<usize>();
        assert!(
            all.len() >= paths - COMMON.len(),
            "{} shapes for {paths} paths",
            all.len()
        );
        assert!(all.iter().any(|(group, _)| *group == Group::RowKey));
        assert!(all.iter().any(|(group, _)| *group == Group::Extras));
        // Package M's escapes are Clippy's alone: never counted as K's or X's.
        assert!(!section('K').contains("Sentence::outside") && !section('X').contains("outcome::"));
    }

    /// Package X's paths are counted as calls, by every shape they are written in.
    #[test]
    fn the_count_reads_package_x_calls() {
        let extras = |source: &str| count(source, Group::Extras).unwrap();
        assert_eq!(
            extras(
                "fn f() { ui.add_space(8.0); egui::Ui::add_space(ui, 4.0); ui.selectable_value(&mut a, 1, \"One\"); let _ = ui.selectable_label(true, \"Two\"); egui::Button::selectable(true, \"Three\"); }"
            ),
            5
        );
        assert_eq!(
            extras(
                "fn f() { design::search_field(ui, \"Search\", t, \"…\", 200.0); design::group_heading(ui, \"Ready\", 2); design::group_label(ui, \"Party\"); design::reader::section_heading(ui, \"Actions\"); }"
            ),
            // `reader::section_heading` is gone (its last callers moved to `heading`): not counted.
            3
        );
        // The parts that replace them, and words in strings, are not calls.
        assert_eq!(
            extras(
                "fn f() { design::gap(ui, Gap::S); design::heading(ui, Heading::Label(\"add_space(\")); design::choice(ui, true, \"One\"); SEARCH.search(ui, \"Search\", t, \"…\", 200.0); let _ = egui::Label::new(\"x\").selectable(false); }"
            ),
            0
        );
        // A builder method in its constructor's chain, across lines; `Action::selected` is not one.
        assert_eq!(
            extras(
                "fn f() { let b = egui::Button::new(Type::Body.rich(ui, \"x\"))\n.min_size(s)\n.selected(on); ui.add(egui::Button::new(\"a\").selected(true)); }"
            ),
            2
        );
        assert_eq!(
            extras("fn f() { Action::quiet(\"Absent\").selected(true).show(ui); }"),
            0
        );
        // K's count does not see X's calls, nor X's K's.
        assert_eq!(
            count("fn f() { ui.add_space(8.0); }", Group::Layout).unwrap(),
            0
        );
        assert_eq!(
            extras("fn f() { let a = egui::ScrollArea::vertical(); }"),
            0
        );
    }

    #[test]
    fn the_count_reads_calls_not_words() {
        let layout = |source: &str| count(source, Group::Layout).unwrap();
        assert_eq!(
            layout("fn f() { let a = egui::ScrollArea::vertical(); let b = ScrollArea::both(); }"),
            2
        );
        // A method of a value, the same function by its path, a turbofish, and a free function.
        assert_eq!(layout("fn f() { ui.menu_button(\"m\", add); }"), 1);
        assert_eq!(
            layout("fn f() { egui::Ui::menu_button(ui, \"m\", add); }"),
            1
        );
        assert_eq!(
            layout("fn f() { ui.dnd_drag_source::<u32, ()>(id, 1, add); }"),
            1
        );
        assert_eq!(
            layout("fn f() { design::text_field(ui, \"a\", t, None, None); }"),
            1
        );
        assert_eq!(layout("fn f() { egui::menu::bar(ui, add); }"), 1);
        // Nested functions and closures count inside the item.
        assert_eq!(
            layout(
                "fn f() { fn g() { let a = egui::ScrollArea::vertical(); } let c = |ui: &mut Ui| ui.text_edit_singleline(&mut s); }"
            ),
            2
        );
        // Words in strings and comments, the design's own parts, a field and another `bar`.
        assert_eq!(
            layout(
                "fn f() { let w = \"ScrollArea::vertical()\"; // TextEdit::singleline(t)\n design::menu_button(ui, \"Sort\", true); let o: ScrollAreaOutput<()> = out; chart.bar(1); fn text_field() {} }"
            ),
            0
        );
        assert_eq!(
            count(
                "fn f() { NAME.with(RowIndex::from_data(1)); TextKey::from_data(name); }",
                Group::RowKey
            )
            .unwrap(),
            2
        );
    }

    #[test]
    fn the_syntax_rules_find_hand_made_layers_and_reasons() {
        let rules = |text: &str| syntax_rules(text).unwrap();
        for text in [
            "fn f() { let layer = egui::LayerId { order: egui::Order::Foreground, id }; }",
            "fn f() { let layer = LayerId::new(Order::Tooltip, id); }",
            "fn f() { let bar = egui::containers::menu::MenuBar::default(); }",
            "fn f() { let menu: SubMenu = Default::default(); }",
        ] {
            assert!(rules(text).contains(&"foreground-layer"), "{text}");
        }
        for text in [
            "fn f() { let order = SortOrder::Foreground; let words = \"Order::Tooltip\"; }",
            "fn f() { let order = ResultOrder::default(); }",
        ] {
            assert!(rules(text).is_empty(), "{text}");
        }
        assert_eq!(
            rules("impl design::CreateReasons for Why {}"),
            ["hand-create-reasons"]
        );
        // Review of PR 305: an alias would hide a banned name from the token counters, so a
        // renaming import or a type alias of any banned owner or function is refused, in an
        // allowed function too, for every package's list.
        for text in [
            "#[allow(clippy::disallowed_methods)] fn show() { use crate::design::Fills as Fill; let _ = Fill::because(FillsReason::Columns); }",
            "use crate::design::{Body, Fills as Room};",
            "type Fill = crate::design::Fills;",
            "use crate::design::pane::Fills::because as fills;",
            "use eframe::egui::ScrollArea as Area;",
            "use crate::design::outcome::banner as notice;",
            "type Hand = splat::design::components::Footer<'static>;",
        ] {
            assert_eq!(rules(text), ["banned-alias"], "{text}");
        }
        for text in [
            "use crate::design::{Body, Fills};",
            "use egui::accesskit::Action as Access;",
            "type Groups = BTreeMap<String, Value>;",
            "use splat::content::formats::patch_model as patch_fields;",
        ] {
            assert!(rules(text).is_empty(), "{text}");
        }
        assert_eq!(
            field_id_names("const A: FieldId = crate::field_id!(\"a_name\");").unwrap(),
            ["a_name"]
        );
    }
}
