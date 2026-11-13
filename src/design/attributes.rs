//! Every lint attribute of a Rust file with the item it is on, read by `syn` (tests only). The
//! source checks use it instead of reading text: package O's suppression table
//! (`results::suppressions`), and the message lint's counted `allow(deprecated)` and the level of
//! each child module (`lint`). It reads inner and outer attributes, lists across lines, `cfg_attr`,
//! and attributes inside macro bodies (`macro_rules!` and macro calls), whose tokens `syn` does
//! not parse as items.
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

/// A lint level an attribute sets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Level {
    Allow,
    Expect,
    Warn,
    Deny,
    Forbid,
}

impl Level {
    fn of(word: &str) -> Option<Self> {
        Some(match word {
            "allow" => Self::Allow,
            "expect" => Self::Expect,
            "warn" => Self::Warn,
            "deny" => Self::Deny,
            "forbid" => Self::Forbid,
            _ => return None,
        })
    }

    /// `allow` or `expect`: it lets the lint pass.
    pub(crate) fn lets_pass(self) -> bool {
        matches!(self, Self::Allow | Self::Expect)
    }
}

/// What an attribute is on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    /// An inner attribute at the top of the file.
    File,
    /// A module: `mod x;` (its file below) or `mod x { … }`.
    Mod {
        has_file: bool,
    },
    Impl,
    Trait,
    MacroRules,
    /// A function or method; `nested` names a function inside it.
    Fn {
        nested: Option<String>,
    },
    /// A statement, an expression, or an attribute inside a macro's tokens.
    Statement,
    /// Any other item: a struct, a const, a `use`.
    Other,
}

/// One lint attribute.
#[derive(Clone, Debug)]
pub(crate) struct Found {
    /// `#![…]`.
    pub inner: bool,
    /// The 1-based line where the attribute starts.
    pub line: usize,
    /// Each level it sets, with the lints it names (`clippy::disallowed_methods`, `deprecated`).
    pub levels: Vec<(Level, Vec<String>)>,
    pub kind: Kind,
    /// The function's or module's name; for a statement, `<enclosing> statement`, where the
    /// enclosing item is a function, `<name>!` for a `macro_rules!`, or `file`.
    pub item: String,
    /// The 1-based lines of what it is on.
    pub lines: (usize, usize),
}

impl Found {
    /// Does it let one of `lints` pass?
    pub(crate) fn lets_pass(&self, lints: &[&str]) -> bool {
        self.levels.iter().any(|(level, names)| {
            level.lets_pass() && names.iter().any(|n| lints.contains(&n.as_str()))
        })
    }

    /// Does it set a level other than allow for one of `lints`?
    pub(crate) fn resets(&self, lints: &[&str]) -> bool {
        self.levels.iter().any(|(level, names)| {
            !level.lets_pass() && names.iter().any(|n| lints.contains(&n.as_str()))
        })
    }
}

fn path_words(path: &syn::Path) -> String {
    path.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}

/// The levels of one attribute's meta: `allow(a, b)`, `cfg_attr(pred, allow(a), warn(b))`.
fn levels_of(meta: &syn::Meta, out: &mut Vec<(Level, Vec<String>)>) {
    let syn::Meta::List(list) = meta else {
        return;
    };
    let name = path_words(&list.path);
    type Metas = syn::punctuated::Punctuated<syn::Meta, syn::Token![,]>;
    let Ok(metas) = list.parse_args_with(Metas::parse_terminated) else {
        return;
    };
    if name == "cfg_attr" {
        for meta in metas.iter().skip(1) {
            levels_of(meta, out);
        }
    } else if let Some(level) = Level::of(&name) {
        let names = metas
            .iter()
            .filter_map(|meta| match meta {
                syn::Meta::Path(path) => Some(path_words(path)),
                _ => None,
            })
            .collect();
        out.push((level, names));
    }
}

/// The lint attributes among `attrs`.
/// An attribute's style (inner), its line and its levels.
type LintAttr = (bool, usize, Vec<(Level, Vec<String>)>);

fn lint_attrs(attrs: &[syn::Attribute]) -> Vec<LintAttr> {
    attrs
        .iter()
        .filter_map(|attr| {
            let mut levels = Vec::new();
            levels_of(&attr.meta, &mut levels);
            (!levels.is_empty()).then(|| {
                (
                    matches!(attr.style, syn::AttrStyle::Inner(_)),
                    attr.span().start().line,
                    levels,
                )
            })
        })
        .collect()
}

fn lines_of(node: &impl Spanned) -> (usize, usize) {
    let span = node.span();
    (span.start().line, span.end().line)
}

/// Does `block` hold a function (not inside a closure's macro)? Its name.
fn nested_fn(block: &syn::Block) -> Option<String> {
    struct Finder(Option<String>);
    impl<'ast> Visit<'ast> for Finder {
        fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
            self.0.get_or_insert_with(|| item.sig.ident.to_string());
        }
        // A method of an `impl` inside the function is freed by its allow too.
        fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
            self.0.get_or_insert_with(|| item.sig.ident.to_string());
        }
    }
    let mut finder = Finder(None);
    finder.visit_block(block);
    finder.0
}

struct Reader {
    found: Vec<Found>,
    /// The enclosing function or macro, innermost last.
    within: Vec<String>,
}

impl Reader {
    fn enclosing(&self) -> String {
        self.within
            .last()
            .cloned()
            .unwrap_or_else(|| "file".to_owned())
    }

    fn push(&mut self, attrs: &[syn::Attribute], kind: Kind, item: String, lines: (usize, usize)) {
        for (inner, line, levels) in lint_attrs(attrs) {
            self.found.push(Found {
                inner,
                line,
                levels,
                kind: kind.clone(),
                item: item.clone(),
                lines,
            });
        }
    }

    /// Lint attributes in a macro's tokens: `#[allow(…)]` and `#![allow(…)]` before what they
    /// are on, at any depth.
    fn tokens(&mut self, tokens: TokenStream, item: &str) {
        let trees: Vec<TokenTree> = tokens.into_iter().collect();
        for (at, tree) in trees.iter().enumerate() {
            match tree {
                TokenTree::Punct(punct) if punct.as_char() == '#' => {
                    let (inner, next) = match trees.get(at + 1) {
                        Some(TokenTree::Punct(bang)) if bang.as_char() == '!' => (true, at + 2),
                        _ => (false, at + 1),
                    };
                    let Some(TokenTree::Group(group)) = trees.get(next) else {
                        continue;
                    };
                    if group.delimiter() != Delimiter::Bracket {
                        continue;
                    }
                    let Ok(meta) = syn::parse2::<syn::Meta>(group.stream()) else {
                        continue;
                    };
                    let mut levels = Vec::new();
                    levels_of(&meta, &mut levels);
                    if levels.is_empty() {
                        continue;
                    }
                    let end = trees
                        .get(next + 1)
                        .map_or(group.span().end().line, |t| t.span().end().line);
                    self.found.push(Found {
                        inner,
                        line: punct.span().start().line,
                        levels,
                        kind: Kind::Statement,
                        item: format!("{item} statement"),
                        lines: (punct.span().start().line, end),
                    });
                }
                TokenTree::Group(group) => self.tokens(group.stream(), item),
                _ => {}
            }
        }
    }

    fn function(
        &mut self,
        attrs: &[syn::Attribute],
        name: String,
        block: &syn::Block,
        lines: (usize, usize),
    ) {
        self.push(
            attrs,
            Kind::Fn {
                nested: nested_fn(block),
            },
            name.clone(),
            lines,
        );
        self.within.push(name);
    }
}

impl<'ast> Visit<'ast> for Reader {
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        self.function(
            &item.attrs,
            item.sig.ident.to_string(),
            &item.block,
            lines_of(item),
        );
        visit::visit_item_fn(self, item);
        self.within.pop();
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        self.function(
            &item.attrs,
            item.sig.ident.to_string(),
            &item.block,
            lines_of(item),
        );
        visit::visit_impl_item_fn(self, item);
        self.within.pop();
    }

    fn visit_trait_item_fn(&mut self, item: &'ast syn::TraitItemFn) {
        match &item.default {
            Some(block) => {
                self.function(
                    &item.attrs,
                    item.sig.ident.to_string(),
                    block,
                    lines_of(item),
                );
                visit::visit_trait_item_fn(self, item);
                self.within.pop();
            }
            None => {
                let kind = Kind::Fn { nested: None };
                self.push(
                    &item.attrs,
                    kind,
                    item.sig.ident.to_string(),
                    lines_of(item),
                );
            }
        }
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let kind = Kind::Mod {
            has_file: item.content.is_none(),
        };
        self.push(&item.attrs, kind, item.ident.to_string(), lines_of(item));
        visit::visit_item_mod(self, item);
    }

    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        self.push(&item.attrs, Kind::Impl, "impl".to_owned(), lines_of(item));
        visit::visit_item_impl(self, item);
    }

    fn visit_item_trait(&mut self, item: &'ast syn::ItemTrait) {
        self.push(
            &item.attrs,
            Kind::Trait,
            item.ident.to_string(),
            lines_of(item),
        );
        visit::visit_item_trait(self, item);
    }

    fn visit_item_macro(&mut self, item: &'ast syn::ItemMacro) {
        let rules = item.mac.path.is_ident("macro_rules");
        let name = item
            .ident
            .as_ref()
            .map_or_else(|| path_words(&item.mac.path), ToString::to_string);
        if rules {
            self.push(&item.attrs, Kind::MacroRules, name.clone(), lines_of(item));
            self.tokens(item.mac.tokens.clone(), &format!("{name}!"));
        } else {
            self.push(
                &item.attrs,
                Kind::Statement,
                format!("{} statement", self.enclosing()),
                lines_of(item),
            );
            self.tokens(item.mac.tokens.clone(), &self.enclosing());
        }
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        // A macro call inside a function (`vec![…]`, `ui.horizontal(…)` bodies are parsed).
        self.tokens(mac.tokens.clone(), &self.enclosing());
    }

    fn visit_local(&mut self, local: &'ast syn::Local) {
        let item = format!("{} statement", self.enclosing());
        self.push(&local.attrs, Kind::Statement, item, lines_of(local));
        visit::visit_local(self, local);
    }

    fn visit_stmt_macro(&mut self, stmt: &'ast syn::StmtMacro) {
        let item = format!("{} statement", self.enclosing());
        self.push(&stmt.attrs, Kind::Statement, item, lines_of(stmt));
        visit::visit_stmt_macro(self, stmt);
    }

    fn visit_expr(&mut self, expr: &'ast syn::Expr) {
        let attrs: &[syn::Attribute] = match expr {
            syn::Expr::Block(e) => &e.attrs,
            syn::Expr::Call(e) => &e.attrs,
            syn::Expr::Closure(e) => &e.attrs,
            syn::Expr::MethodCall(e) => &e.attrs,
            syn::Expr::Path(e) => &e.attrs,
            syn::Expr::If(e) => &e.attrs,
            syn::Expr::Match(e) => &e.attrs,
            syn::Expr::Unsafe(e) => &e.attrs,
            syn::Expr::Assign(e) => &e.attrs,
            _ => &[],
        };
        if !attrs.is_empty() {
            let item = format!("{} statement", self.enclosing());
            self.push(attrs, Kind::Statement, item, lines_of(expr));
        }
        visit::visit_expr(self, expr);
    }

    fn visit_item(&mut self, item: &'ast syn::Item) {
        let other: Option<(&[syn::Attribute], String)> = match item {
            syn::Item::Const(i) => Some((&i.attrs, i.ident.to_string())),
            syn::Item::Static(i) => Some((&i.attrs, i.ident.to_string())),
            syn::Item::Struct(i) => Some((&i.attrs, i.ident.to_string())),
            syn::Item::Enum(i) => Some((&i.attrs, i.ident.to_string())),
            syn::Item::Type(i) => Some((&i.attrs, i.ident.to_string())),
            syn::Item::Use(i) => Some((&i.attrs, "use".to_owned())),
            _ => None,
        };
        if let Some((attrs, name)) = other {
            self.push(attrs, Kind::Other, name, lines_of(item));
        }
        visit::visit_item(self, item);
    }
}

/// Every lint attribute of a file, or why the file does not parse.
pub(crate) fn read(text: &str) -> Result<Vec<Found>, String> {
    let file = syn::parse_file(text).map_err(|error| {
        let at = error.span().start();
        format!("line {}: {error}", at.line)
    })?;
    let mut reader = Reader {
        found: Vec::new(),
        within: Vec::new(),
    };
    for (inner, line, levels) in lint_attrs(&file.attrs) {
        let lines = (1, text.lines().count());
        reader.found.push(Found {
            inner,
            line,
            levels,
            kind: Kind::File,
            item: "file".to_owned(),
            lines,
        });
    }
    reader.visit_file(&file);
    reader.found.sort_by_key(|found| found.line);
    Ok(reader.found)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(text: &str) -> Found {
        let found = read(text).unwrap();
        assert_eq!(found.len(), 1, "{text}: {found:?}");
        found.into_iter().next().unwrap()
    }

    /// Attributes in every form the hand-written readers missed: lists across lines, `cfg_attr`,
    /// a combined list, inner and outer, on a module declaration, in a macro's tokens.
    #[test]
    fn every_lint_attribute_is_read_with_its_item() {
        let multi =
            one("#![allow(\n    deprecated,\n    reason = \"old messages\"\n)]\nfn a() {}\n");
        assert!(multi.inner && multi.kind == Kind::File && multi.lets_pass(&["deprecated"]));
        let combined = one("#[allow(dead_code, deprecated)]\nfn draw() {}\n");
        assert!(combined.lets_pass(&["deprecated"]) && combined.item == "draw");
        let cfg = one("#[cfg_attr(test, allow(clippy::disallowed_methods))]\nfn order() {}\n");
        assert!(cfg.lets_pass(&["clippy::disallowed_methods"]));
        let reset = one("#![warn(deprecated)]\n");
        assert!(reset.resets(&["deprecated"]) && !reset.lets_pass(&["deprecated"]));
        let declared = one("#[allow(deprecated)]\nmod child;\n");
        assert_eq!(declared.kind, Kind::Mod { has_file: true });
        let nested = one("#[allow(warnings)]\nfn draw() {\n    fn order() {}\n}\n");
        assert_eq!(
            nested.kind,
            Kind::Fn {
                nested: Some("order".to_owned())
            }
        );
        let statement = one(
            "fn draw() {\n    #[allow(clippy::disallowed_methods)]\n    let order = || 1;\n}\n",
        );
        assert_eq!(
            (statement.kind, statement.item.as_str()),
            (Kind::Statement, "draw statement")
        );
        let in_macro = one(
            "macro_rules! m {\n    () => {{\n        #[allow(clippy::disallowed_methods)]\n        let x = 1;\n    }};\n}\n",
        );
        assert_eq!(in_macro.item, "m! statement");
        let method = one("impl A {\n    #[allow(deprecated)]\n    fn page(&self) {}\n}\n");
        assert_eq!(method.item, "page");
        // Words in strings and comments are not attributes.
        assert!(
            read("// #[allow(deprecated)]\nconst A: &str = \"#[allow(deprecated)]\";\n")
                .unwrap()
                .is_empty()
        );
    }
}
