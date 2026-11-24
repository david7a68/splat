//! Source checks (tests only), read from Rust syntax, never from text.
//!
//! - [`syntax_rules`]: a renaming import or a type alias of a `clippy.toml` ban, which would hide
//!   it from review, and a `CreateReasons` impl by hand, which could leave a reason out of `ALL`.
//! - [`field_id_names`]: every `field_id!` name, for the check that each is used once.
use syn::visit::{self, Visit};

const CLIPPY: &str = include_str!("../../../clippy.toml");

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
/// whose functions `clippy.toml` bans: `Fills`, `Footer`, `ScrollArea`), a banned method by its
/// owner (`Fills::because`), or a banned free function by its name through any module: its own
/// (`outcome::banner`) or one that re-exports it (`design::group_label`, from `components`).
fn names_a_ban(path: &[String], bans: &[(String, String)]) -> bool {
    let Some(last) = path.last() else {
        return false;
    };
    let owner = bans
        .iter()
        .any(|(owner, _)| owner == last && owner.starts_with(|c: char| c.is_ascii_uppercase()));
    let function = path.len() >= 2
        && bans.iter().any(|(owner, name)| {
            name == last
                && (*owner == path[path.len() - 2]
                    || !owner.starts_with(|c: char| c.is_ascii_uppercase()))
        });
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
                // `use a::Owner::{self as Other}` renames `a::Owner` itself.
                let named = rename.ident != "self";
                if named {
                    prefix.push(rename.ident.to_string());
                }
                if names_a_ban(prefix, &self.bans) {
                    self.found.push("banned-alias");
                }
                if named {
                    prefix.pop();
                }
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

/// The rules that only the syntax tree can see, once per finding (see the module's doc).
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
    fn the_syntax_rules_find_aliases_and_hand_made_reasons() {
        let rules = |text: &str| syntax_rules(text).unwrap();
        assert_eq!(
            rules("impl design::CreateReasons for Why {}"),
            ["hand-create-reasons"]
        );
        // Review of PR 305: an alias would hide a banned name from review, so a renaming import
        // or a type alias of any banned owner or function is refused, in an allowed function
        // too.
        for text in [
            "#[allow(clippy::disallowed_methods)] fn show() { use crate::design::Results as Store; let _ = Store::undo; }",
            "use crate::design::{Body, ResultOrder as Order};",
            "type Store = crate::design::results::Results<A, B>;",
            "use crate::design::outcome::Sentence::outside as words;",
            "use crate::design::components::save_row_state as save_state;",
            // Round 29 P-1: a free function through a module that re-exports it.
            "#[allow(clippy::disallowed_methods)] fn add_row() { use crate::design::save_row_state as state; }",
            "use crate::design::{save_row_state as state, Heading};",
            // Review of PR 446 (N3): `self` renames the path before its group.
            "use crate::design::outcome::Messages::{self as Lines};",
        ] {
            assert_eq!(rules(text), ["banned-alias"], "{text}");
        }
        for text in [
            "use crate::design::{Body, Results};",
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
