//! The legacy host freeze (migration 32, layout PR 0, ruling L12). Tests only.
//!
//! `src/design/legacy_hosts.tsv` lists every use of a layout or host path that the pane
//! mechanism replaces ([`CAPABILITIES`]), by the function that holds it (`Type::name` in an
//! `impl` or a trait), with the pane's owner: in the app and in `src/design/`, whose rows are
//! the implementation boundary. Uses are read from the tokens (`layout::bans::calls`): a method
//! or associated call, a reference and a macro's argument all name the path; words in strings
//! and comments never do. `#[test]` and `#[cfg(test)]` items, and files that only `cfg(test)`
//! module declarations load, do not count; a file's name proves nothing.
//!
//! - The list is exact: a new use fails, and fewer uses ask to lower the row.
//! - No row grows against its base. The uses are counted again in the source of the merge base
//!   of `HEAD` and `origin/main` (read with `git`; `SPLAT_LEGACY_BASE` names another ref, never
//!   in the landing gate), so a use added together with its row fails, and so does a new
//!   function around a counted use, in the app or in the design system (a wrapper is a new
//!   site). A raise is a line in the gate's raise file (`SPLAT_STILLNESS_RAISE`), one per site:
//!   `legacy-host / <capability> / <file> / <function> => <reason>`; the lead passes that file
//!   at landing (a workflow rule: the test reads the line, not who wrote it). A base that cannot
//!   be read fails.
//! - Escapes fail with no row and no raise: a frozen function named but not called (a
//!   reference, a macro's argument), a frozen path inside `macro_rules!`, and an alias of a
//!   frozen owner or function (`… as …`, `type … =`), anywhere, `src/design/` too.
//! - `src/design/c3_ledger.tsv` lists the 85 round-31 C3 deductions: report:ID, root, the steps
//!   of ruling L1 that own it, whether the layout design names it there or only its root, and
//!   its status. It claims no geometry. A row is never removed; its owning step closes it.
use crate::design::layout::bans::{Shape, aliases, calls};
use std::collections::{BTreeMap, BTreeSet};
use std::process::Command;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

const MANIFEST: &str = include_str!("../legacy_hosts.tsv");
const LEDGER: &str = include_str!("../c3_ledger.tsv");
const LEDGER_PATH: &str = "src/design/c3_ledger.tsv";

/// The capabilities the pane mechanism replaces (layout design §4.3), each with how its uses
/// are spelled, split at spaces (`bans::Shape::of`): a bare name is every mention of that
/// function, or of that type when it starts with a capital; `Owner::name` a path; `name()` a
/// free function's call; `.name/n` a method called with at least `n` arguments.
const CAPABILITIES: [(&str, &str); 12] = [
    // A body drawn by a closure into a pane. `Self::pane` counts in `Footer` and `Messages` only.
    // `pane_then` (migration-32 L1c) is a pane whose messages come after the body.
    (
        "pane-host",
        ".pane/3 Footer::pane Messages::pane Self::pane pane_content pane_editor pane_then \
         pane_content_then pane_editor_then",
    ),
    // A footer placed by hand.
    ("hand-footer", "show_by_hand"),
    // A body that manages its own room, also through the Sessions wrapper `fills_body`: each
    // Sessions pane that names it counts.
    ("fills", "Fills::because fills_body"),
    // Rows drawn by a closure for the part of a list in view.
    ("virtual-rows", "show_rows show_viewport"),
    // A row registered after it painted, for the old whole-row mask.
    ("row-register", "register_row"),
    // A clip that a caller sets.
    ("row-clip", "set_clip_rect with_clip_rect shrink_clip_rect"),
    // A scroll request outside the reveal part (B6's list, PR 403).
    (
        "scroll-request",
        "scroll_to_rect scroll_to_rect_animation scroll_to_cursor scroll_to_cursor_animation \
         scroll_to_me scroll_to_me_animation scroll_with_delta scroll_with_delta_animation",
    ),
    // An offset that a caller writes into a scroll area.
    (
        "scroll-offset",
        "vertical_scroll_offset horizontal_scroll_offset scroll_offset",
    ),
    // A raw scroll area: Clippy refuses it outside a counted row, and here it cannot grow.
    (
        "scroll-area",
        "ScrollArea::vertical ScrollArea::both ScrollArea::horizontal ScrollArea::neither \
         ScrollArea::new",
    ),
    // The allowed host composition: the page regions that hold panes.
    (
        "host-region",
        "CentralPanel SidePanel TopBottomPanel Column::panel",
    ),
    // Children of composite columns drawn by a closure (`PaneEdges::show_children`, layout step
    // 7): the composite owns their edges and height, not yet what they draw. Each host is a row
    // until its children are closed blocks.
    ("composite-child", "show_children"),
    // An owner's messages fed into a footer by hand (`footer_messages`). One footer fed by more
    // than one owner is a mixed task footer: the Settings debt of step 8.
    ("footer-feed", "footer_messages"),
];

/// A counted site: capability, file, function.
type Site = (String, String, String);
type Counts = BTreeMap<Site, usize>;

/// Every name an alias could hide: each frozen function or type, and each owner of a frozen path.
fn alias_names() -> Vec<&'static str> {
    CAPABILITIES
        .iter()
        .flat_map(|(_, spellings)| spellings.split_whitespace())
        .filter(|spelling| !spelling.starts_with('.'))
        .map(|spelling| spelling.trim_end_matches("()"))
        .map(|spelling| spelling.split("::").next().unwrap_or(spelling))
        .filter(|name| *name != "Self")
        .collect()
}

/// A module declared with a file of its own (`mod x;`): the inline modules around it, its name,
/// its `#[path]`, and whether only a test build compiles it.
struct Declaration {
    inline: Vec<String>,
    name: String,
    path: Option<String>,
    test: bool,
}

/// The functions, test items, `macro_rules!` and module declarations of one file, by line.
#[derive(Default)]
struct Items {
    /// Each function: its name (`Type::name` in an `impl` or a trait), first and last line.
    functions: Vec<(String, usize, usize)>,
    /// The first and last line of each test item.
    tests: Vec<(usize, usize)>,
    /// The first and last line of each `macro_rules!`.
    macros: Vec<(usize, usize)>,
    declarations: Vec<Declaration>,
    owner: Option<String>,
    inline: Vec<String>,
    in_test: bool,
}

fn lines(node: &impl Spanned) -> (usize, usize) {
    let span = node.span();
    (span.start().line, span.end().line)
}

fn is_test(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        attribute.path().is_ident("test")
            || (attribute.path().is_ident("cfg")
                && attribute
                    .parse_args::<syn::Ident>()
                    .is_ok_and(|word| word == "test"))
    })
}

fn within(spans: &[(usize, usize)], line: usize) -> bool {
    spans
        .iter()
        .any(|(first, last)| (*first..=*last).contains(&line))
}

impl Items {
    /// The innermost function around `line`, or `(file)` outside every function.
    fn holder(&self, line: usize) -> String {
        self.functions
            .iter()
            .filter(|(_, first, last)| (*first..=*last).contains(&line))
            .max_by_key(|(_, first, _)| *first)
            .map_or_else(|| "(file)".to_owned(), |(name, ..)| name.clone())
    }

    /// Records a function; false for a test item, whose body is not read.
    fn function(
        &mut self,
        attributes: &[syn::Attribute],
        name: &syn::Ident,
        node: &impl Spanned,
    ) -> bool {
        if is_test(attributes) {
            self.tests.push(lines(node));
            return false;
        }
        let name = match &self.owner {
            Some(owner) => format!("{owner}::{name}"),
            None => name.to_string(),
        };
        let (first, last) = lines(node);
        self.functions.push((name, first, last));
        true
    }

    /// Visits a function's body; a function inside it has no owner.
    fn body(&mut self, visit: impl FnOnce(&mut Self)) {
        let owner = self.owner.take();
        visit(self);
        self.owner = owner;
    }
}

impl<'ast> Visit<'ast> for Items {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        let test = self.in_test || is_test(&node.attrs);
        let name = node.ident.to_string();
        if node.content.is_none() {
            let path = node.attrs.iter().find_map(|attribute| {
                let value = &attribute.meta.require_name_value().ok()?.value;
                match value {
                    syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(file),
                        ..
                    }) if attribute.path().is_ident("path") => Some(file.value()),
                    _ => None,
                }
            });
            let inline = self.inline.clone();
            self.declarations.push(Declaration {
                inline,
                name,
                path,
                test,
            });
            return;
        }
        if test && !self.in_test {
            self.tests.push(lines(node));
        }
        let outer = std::mem::replace(&mut self.in_test, test);
        self.inline.push(name);
        visit::visit_item_mod(self, node);
        self.inline.pop();
        self.in_test = outer;
    }

    fn visit_item_macro(&mut self, node: &'ast syn::ItemMacro) {
        if node.mac.path.is_ident("macro_rules") {
            self.macros.push(lines(node));
        }
        visit::visit_item_macro(self, node);
    }

    fn visit_stmt_macro(&mut self, node: &'ast syn::StmtMacro) {
        if node.mac.path.is_ident("macro_rules") {
            self.macros.push(lines(node));
        }
        visit::visit_stmt_macro(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        if is_test(&node.attrs) {
            self.tests.push(lines(node));
            return;
        }
        let owner = match &*node.self_ty {
            syn::Type::Path(path) => path.path.segments.last().map(|s| s.ident.to_string()),
            _ => None,
        };
        let outer = std::mem::replace(&mut self.owner, owner);
        visit::visit_item_impl(self, node);
        self.owner = outer;
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        let outer = self.owner.replace(node.ident.to_string());
        visit::visit_item_trait(self, node);
        self.owner = outer;
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.body(|this| {
            if this.function(&node.attrs, &node.sig.ident, node) {
                visit::visit_item_fn(this, node);
            }
        });
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        if self.function(&node.attrs, &node.sig.ident, node) {
            self.body(|this| visit::visit_impl_item_fn(this, node));
        }
    }

    fn visit_trait_item_fn(&mut self, node: &'ast syn::TraitItemFn) {
        if self.function(&node.attrs, &node.sig.ident, node) {
            self.body(|this| visit::visit_trait_item_fn(this, node));
        }
    }
}

/// `path` with `.` and `..` segments resolved.
fn normal(path: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "." | "" => {}
            ".." => {
                parts.pop();
            }
            part => parts.push(part),
        }
    }
    parts.join("/")
}

/// The file that a declaration in `path` loads, among `known`: `src/a.rs` and `src/a/mod.rs`
/// load `src/a/x.rs` or `src/a/x/mod.rs`; a `#[path]` outside an inline module is relative to
/// the declaring file's folder.
fn module_file(path: &str, declaration: &Declaration, known: &BTreeSet<&str>) -> Option<String> {
    let (folder, file) = path.rsplit_once('/').unwrap_or(("", path));
    let own = match file {
        "mod.rs" | "lib.rs" | "main.rs" => folder.to_owned(),
        _ => path.trim_end_matches(".rs").to_owned(),
    };
    let inline = declaration.inline.join("/");
    let candidates = match &declaration.path {
        Some(file) if inline.is_empty() => vec![format!("{folder}/{file}")],
        Some(file) => vec![format!("{own}/{inline}/{file}")],
        None => {
            let base = format!("{own}/{inline}/{}", declaration.name);
            vec![format!("{base}.rs"), format!("{base}/mod.rs")]
        }
    };
    candidates
        .iter()
        .map(|candidate| normal(candidate))
        .find(|candidate| known.contains(candidate.as_str()))
}

pub(super) fn is_crate_root(path: &str) -> bool {
    let (folder, file) = path.rsplit_once('/').unwrap_or(("", path));
    (folder == "src" || folder.ends_with("/src")) && matches!(file, "lib.rs" | "main.rs")
}

/// Production modules loaded by path from a crate root may live outside its source directory.
pub(super) fn crate_root_parents(
    files: &[(String, String)],
) -> Result<BTreeMap<String, Vec<String>>, String> {
    let known = files.iter().map(|(path, _)| path.as_str()).collect();
    let mut parents: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (root, text) in files.iter().filter(|(path, _)| is_crate_root(path)) {
        let file = syn::parse_file(text).map_err(|error| format!("{root}: {error}"))?;
        let mut items = Items::default();
        items.visit_file(&file);
        for declaration in items.declarations {
            if !declaration.test
                && declaration.path.is_some()
                && let Some(child) = module_file(root, &declaration, &known)
            {
                parents.entry(child).or_default().push(root.clone());
            }
        }
    }
    Ok(parents)
}

/// The files that a production build never compiles: every declaration that loads them is
/// `cfg(test)`, or sits in a file that is itself never compiled. A crate root (`src/lib.rs`,
/// `src/main.rs`) is compiled; a file that nothing declares is not proven test-only and counts.
fn test_only(files: &[(&String, Items)]) -> BTreeSet<String> {
    let known: BTreeSet<&str> = files.iter().map(|(path, _)| path.as_str()).collect();
    let loads: Vec<(&str, String, bool)> = files
        .iter()
        .flat_map(|(path, items)| {
            items.declarations.iter().filter_map(|declaration| {
                module_file(path, declaration, &known)
                    .map(|child| (path.as_str(), child, declaration.test))
            })
        })
        .collect();
    let mut live: BTreeSet<&str> = known
        .iter()
        .copied()
        .filter(|path| is_crate_root(path))
        .collect();
    loop {
        let more: Vec<&str> = loads
            .iter()
            .filter(|(parent, child, test)| {
                !test && live.contains(parent) && !live.contains(child.as_str())
            })
            .map(|(_, child, _)| child.as_str())
            .collect();
        if more.is_empty() {
            break;
        }
        live.extend(more);
    }
    loads
        .iter()
        .filter(|(_, child, _)| !live.contains(child.as_str()))
        .map(|(_, child, _)| child.clone())
        .collect()
}

/// What [`scan`] finds: each counted use by site, and the escapes that no row or raise allows.
struct Found {
    counts: Counts,
    escapes: Vec<String>,
}

/// Each counted use in `files` (path, text), by capability and the function that holds it.
fn scan(files: &[(String, String)]) -> Result<Found, String> {
    let patterns: Vec<(usize, Shape)> = CAPABILITIES
        .iter()
        .enumerate()
        .flat_map(|(at, (_, spellings))| {
            spellings
                .split_whitespace()
                .map(move |s| (at, Shape::of(s)))
        })
        .collect();
    let shapes: Vec<Shape> = patterns.iter().map(|(_, shape)| shape.clone()).collect();
    let names = alias_names();
    let mut read = Vec::new();
    for (path, text) in files {
        let file = syn::parse_file(text).map_err(|error| format!("{path}: {error}"))?;
        let mut items = Items::default();
        items.visit_file(&file);
        read.push((path, items));
    }
    let test_only = test_only(&read);
    let (mut counts, mut escapes) = (Counts::new(), Vec::new());
    for ((path, items), (_, text)) in read.iter().zip(files) {
        if test_only.contains(*path) {
            continue;
        }
        let uses = calls(text, &shapes).map_err(|error| format!("{path}: {error}"))?;
        for (line, at, called) in uses {
            if within(&items.tests, line) {
                continue;
            }
            let (capability, _) = CAPABILITIES[patterns[at].0];
            let holder = items.holder(line);
            let type_name =
                matches!(&shapes[at], Shape::Name(name) if name.starts_with(char::is_uppercase));
            if let Shape::Assoc(owner, _) = &shapes[at]
                && owner == "Self"
                && !(holder.starts_with("Footer::") || holder.starts_with("Messages::"))
            {
                continue;
            }
            let place = format!("{path}:{line} `{holder}`");
            if !called && !type_name {
                escapes.push(format!(
                    "{place}: a {capability} path named but not called (a reference, a macro's argument). Call it directly where it is used, so that each use is a counted site (ruling L12)."
                ));
            }
            if within(&items.macros, line) {
                escapes.push(format!(
                    "{place}: a {capability} path inside `macro_rules!`, whose every expansion would be an uncounted use. Use it in a function (ruling L12)."
                ));
            }
            let site = (capability.to_owned(), (*path).clone(), holder);
            *counts.entry(site).or_default() += 1;
        }
        for line in aliases(text, &names).map_err(|error| format!("{path}: {error}"))? {
            if !within(&items.tests, line) {
                escapes.push(format!(
                    "{path}:{line}: an alias (`… as …`, `type … =`) of a frozen owner or function hides its uses from the legacy host freeze (ruling L12). Name it as it is."
                ));
            }
        }
    }
    Ok(Found { counts, escapes })
}

/// The manifest's rows (capability, file, function, count, owner, note; split at tabs) by site.
fn manifest(text: &str) -> Result<Counts, Vec<String>> {
    let (mut rows, mut faults) = (Counts::new(), Vec::new());
    for (at, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let place = format!("src/design/legacy_hosts.tsv:{}", at + 1);
        let fields: Vec<&str> = line.split('\t').collect();
        let [capability, file, function, count, owner, ..] = fields.as_slice() else {
            faults.push(format!(
                "{place}: a row is `capability<TAB>file<TAB>function<TAB>count<TAB>owner<TAB>note`"
            ));
            continue;
        };
        let count = count.parse::<usize>().unwrap_or(0);
        if !CAPABILITIES.iter().any(|(name, ..)| name == capability)
            || owner.trim().is_empty()
            || count == 0
        {
            faults.push(format!(
                "{place}: a known capability, an owner and a count above 0"
            ));
        }
        let site = (
            (*capability).to_owned(),
            (*file).to_owned(),
            (*function).to_owned(),
        );
        if rows.insert(site, count).is_some() {
            faults.push(format!("{place}: a second row for one site"));
        }
    }
    if faults.is_empty() {
        Ok(rows)
    } else {
        Err(faults)
    }
}

fn site_name((capability, file, function): &Site) -> String {
    format!("legacy-host / {capability} / {file} / {function}")
}

/// The calls found against the manifest: more calls than a row fails; fewer ask to lower it.
fn against_manifest(found: &Counts, listed: &Counts) -> Vec<String> {
    let sites: BTreeSet<&Site> = found.keys().chain(listed.keys()).collect();
    let mut problems = Vec::new();
    for site in sites {
        let now = found.get(site).copied().unwrap_or(0);
        let allowed = listed.get(site).copied().unwrap_or(0);
        let name = site_name(site);
        if now > allowed {
            problems.push(format!(
                "{name}: {now} here, {allowed} in src/design/legacy_hosts.tsv. The pane mechanism replaces this path (migration 32); a row raised with its calls still fails against the base (ruling L12)."
            ));
        } else if now < allowed {
            problems.push(format!(
                "{name}: only {now} remain. Lower its row in src/design/legacy_hosts.tsv to {now} (remove it at 0)."
            ));
        }
    }
    problems
}

/// Each site with more calls than in the base, unless a raise line names it
/// (`<site> => <reason>`, the gate's `SPLAT_STILLNESS_RAISE`).
fn growth(found: &Counts, base: &Counts, raises: &str, base_name: &str) -> Vec<String> {
    let raised: BTreeSet<&str> = raises
        .lines()
        .filter_map(|line| line.split_once(" => "))
        .filter(|(_, reason)| !reason.trim().is_empty())
        .map(|(key, _)| key.trim())
        .collect();
    let mut problems = Vec::new();
    for (site, now) in found {
        let then = base.get(site).copied().unwrap_or(0);
        let name = site_name(site);
        if *now > then && !raised.contains(name.as_str()) {
            problems.push(format!(
                "{name}: {now} here, {then} in the base {base_name}. A legacy site never grows, also when its row grows with it, and a new function around a counted call is a new site (ruling L12). Only the lead raises one: `{name} => <reason>` in the gate's raise file."
            ));
        }
    }
    problems
}

/// Runs `git` in the repository.
fn git(args: &[&str]) -> Result<String, String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(crate::workspace())
        .output()
        .map_err(|error| format!("git {}: {error}", args.join(" ")))?;
    if !out.status.success() {
        return Err(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

/// The base commit: the merge base of `HEAD` and `origin/main`, or of `SPLAT_LEGACY_BASE`,
/// which the landing gate (`SPLAT_LAND_LOCKED=1`) refuses.
fn base_commit() -> Result<String, String> {
    let named = match std::env::var("SPLAT_LEGACY_BASE") {
        Ok(_) if std::env::var("SPLAT_LAND_LOCKED").is_ok_and(|locked| locked == "1") => {
            return Err(
                "the landing gate compares with origin/main only; unset SPLAT_LEGACY_BASE"
                    .to_owned(),
            );
        }
        Ok(named) => named,
        Err(_) => "origin/main".to_owned(),
    };
    git(&["merge-base", "HEAD", &named])
}

/// The app and logic crates' Rust files at `commit`.
fn files_at(commit: &str) -> Result<Vec<(String, String)>, String> {
    let listed = git(&[
        "ls-tree",
        "-r",
        "--name-only",
        commit,
        "--",
        "src",
        "crates/campaign/src",
        "crates/character/src",
        "crates/content/src",
        "crates/execution/src",
    ])?;
    listed
        .lines()
        .filter(|path| path.ends_with(".rs"))
        .map(|path| {
            Ok((
                path.to_owned(),
                git(&["show", &format!("{commit}:{path}")])?,
            ))
        })
        .collect()
}

/// What the lint test reports for the freeze, for `files` (the working tree): its escapes, the
/// manifest against its uses, and its uses against the base.
pub(super) fn problems(files: &[(String, String)]) -> Vec<String> {
    let Found {
        counts: found,
        escapes,
    } = match scan(files) {
        Ok(found) => found,
        Err(error) => return vec![format!("legacy host freeze: {error}")],
    };
    let mut problems = escapes;
    match manifest(MANIFEST) {
        Ok(listed) => problems.extend(against_manifest(&found, &listed)),
        Err(faults) => problems.extend(faults),
    }
    let unavailable = |why: String| {
        format!(
            "legacy host freeze: the base cannot be read, so no site was compared with it (ruling L12): {why}. Fetch main (`git fetch origin main`) so that `origin/main` exists, or name the base in SPLAT_LEGACY_BASE."
        )
    };
    match base_commit().and_then(|commit| Ok((files_at(&commit)?, commit))) {
        Ok((base_files, commit)) => match scan(&base_files).map(|found| found.counts) {
            // An empty base is no base: nothing would be compared.
            Ok(base) if !base.is_empty() => {
                let raises = std::env::var("SPLAT_STILLNESS_RAISE").unwrap_or_default();
                problems.extend(growth(&found, &base, &raises, &commit));
            }
            Ok(_) => problems.push(unavailable(format!("{commit} holds no counted call"))),
            Err(error) => problems.push(unavailable(error)),
        },
        Err(error) => problems.push(unavailable(error)),
    }
    problems
}

/// The round-31 C3 roots and their deductions (layout design §6): 85 in all.
const ROOTS: [(&str, usize); 7] = [
    ("pane-whole-row-cut", 28),
    ("action-near-row", 22),
    ("pane-footer-follows-content", 14),
    ("session-header-measure", 9),
    ("repair-editor-frame", 8),
    ("focused-control-footer-clearance", 2),
    ("group-dirty-reason-slot", 2),
];

/// The reviewable cuts of the layout plan (ruling L1, with the attack's step order).
const STEPS: [&str; 21] = [
    "1a", "1b", "2a", "2b", "3a", "3b", "4", "5a", "5b", "6", "7a", "7b", "7c", "8", "9", "9b",
    "10a", "10b", "11", "12", "reader",
];

/// The ledger's faults, and its report IDs (`report:ID → root → steps → basis → status`).
fn ledger(text: &str) -> (Vec<String>, BTreeSet<String>) {
    let (mut faults, mut ids) = (Vec::new(), BTreeSet::new());
    let mut roots: BTreeMap<&str, usize> = BTreeMap::new();
    for (at, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let place = format!("{LEDGER_PATH}:{}", at + 1);
        let fields: Vec<&str> = line.split('\t').collect();
        let [report, root, steps, basis, status] = fields.as_slice() else {
            faults.push(format!(
                "{place}: a row is `report:ID<TAB>root<TAB>steps<TAB>basis<TAB>status`"
            ));
            continue;
        };
        let id = report.split_once(".md:").filter(|(file, id)| {
            (file.starts_with("sol-") || file.starts_with("astra-")) && !id.is_empty()
        });
        let known_root = ROOTS.iter().find(|(name, _)| name == root);
        let steps_known = !steps.is_empty() && steps.split(',').all(|step| STEPS.contains(&step));
        let closed = status
            .strip_prefix("closed #")
            .is_some_and(|pr| pr.parse::<u32>().is_ok());
        if id.is_none()
            || known_root.is_none()
            || !steps_known
            || !matches!(*basis, "named" | "root")
            || !(*status == "open" || closed)
        {
            faults.push(format!(
                "{place}: `<reviewer>-<n>.md:<ID>`, a C3 root, steps of ruling L1, `named` or `root`, `open` or `closed #<PR>`"
            ));
        }
        if !ids.insert((*report).to_owned()) {
            faults.push(format!("{place}: {report} is listed twice"));
        }
        if let Some((name, _)) = known_root {
            *roots.entry(name).or_default() += 1;
        }
    }
    for (root, deductions) in ROOTS {
        let rows = roots.get(root).copied().unwrap_or(0);
        if rows != deductions {
            faults.push(format!(
                "{LEDGER_PATH}: {root} has {rows} rows; round 31 gave it {deductions} deductions. A row is never removed: its owning step closes it (`closed #<PR>`)."
            ));
        }
    }
    (faults, ids)
}

/// The ledger holds all 85 C3 deductions, each with its root and owning steps (ruling L12), and
/// keeps every row of its base.
#[test]
fn the_c3_ledger_keeps_every_deduction() {
    let (mut faults, ids) = ledger(LEDGER);
    assert_eq!(ids.len(), 85, "{}", faults.join("\n"));
    let commit = base_commit().unwrap_or_else(|why| panic!("the ledger's base: {why}"));
    let path = format!("{commit}:{LEDGER_PATH}");
    // Before the freeze lands, its base has no ledger: the counts above hold it.
    if git(&["cat-file", "-e", &path]).is_ok() {
        let base = git(&["show", &path]).unwrap_or_else(|why| panic!("{why}"));
        for gone in ledger(&base).1.difference(&ids) {
            faults.push(format!(
                "{LEDGER_PATH}: {gone} is in the base {commit} and not here. A row is never removed: its owning step closes it."
            ));
        }
    }
    assert!(faults.is_empty(), "{}", faults.join("\n"));
}

/// The freeze's own cases: a use raised with its row, wrappers in the app and in the design
/// system, associated calls, references, macros, aliases, and which files a build compiles.
#[test]
fn the_legacy_freeze_finds_what_it_should() {
    let at = |files: &[(&str, &str)]| {
        let files: Vec<(String, String)> = files
            .iter()
            .map(|(path, text)| ((*path).to_owned(), (*text).to_owned()))
            .collect();
        scan(&files).unwrap()
    };
    let counts = |text: &str| at(&[("src/a.rs", text)]).counts;
    let site = |capability: &str, file: &str, function: &str| {
        (capability.to_owned(), file.to_owned(), function.to_owned())
    };
    let base = counts("fn draw(footer: Footer, ui: &mut Ui) {\n    footer.show_by_hand(ui);\n}");
    assert_eq!(
        base,
        Counts::from([(site("hand-footer", "src/a.rs", "draw"), 1)])
    );
    // A second use, with its row raised: the manifest agrees, the base does not.
    let two = counts(
        "fn draw(f: Footer, g: Footer, ui: &mut Ui) {\n    f.show_by_hand(ui);\n    g.show_by_hand(ui);\n}",
    );
    assert!(against_manifest(&two, &two).is_empty());
    let raised = growth(&two, &base, "", "B0");
    assert!(
        raised.len() == 1 && raised[0].contains("draw: 2 here, 1 in the base B0"),
        "{raised:?}"
    );
    // A raise line lets exactly that site pass.
    let line = "legacy-host / hand-footer / src/a.rs / draw => ruled";
    assert!(growth(&two, &base, line, "B0").is_empty());
    // A wrapper moves the use into a new site: it fails against the base, in the app ...
    let wrapped = counts(
        "impl Host {\n    fn by_hand(f: Footer, ui: &mut Ui) -> f32 {\n        f.show_by_hand(ui)\n    }\n}\nfn draw(f: Footer, ui: &mut Ui) {\n    Host::by_hand(f, ui);\n}",
    );
    assert_eq!(
        wrapped,
        Counts::from([(site("hand-footer", "src/a.rs", "Host::by_hand"), 1)])
    );
    assert_eq!(growth(&wrapped, &base, "", "B0").len(), 1);
    // ... and in the design system (review F2): the app's row goes, the design helper's comes.
    let moved = at(&[
        (
            "src/design/layout.rs",
            "pub fn by_hand(f: Footer, ui: &mut Ui) -> f32 {\n    f.show_by_hand(ui)\n}",
        ),
        (
            "src/a.rs",
            "fn draw(f: Footer, ui: &mut Ui) {\n    design::by_hand(f, ui);\n}",
        ),
    ]);
    assert_eq!(
        moved.counts,
        Counts::from([(site("hand-footer", "src/design/layout.rs", "by_hand"), 1)])
    );
    assert_eq!(growth(&moved.counts, &base, "", "B0").len(), 1);
    // An associated call and a reference are uses (F1); a reference, a macro's argument, a use
    // inside `macro_rules!` and an alias are escapes that no row allows (F1, F2, F4).
    let named = at(&[(
        "src/a.rs",
        "fn draw(ui: &mut Ui, r: Rect) {\n    egui::Ui::scroll_to_rect(ui, r, None);\n    let mark = crate::design::register_row;\n    mark(ui, r);\n    rows!(set_clip_rect);\n}\nmacro_rules! reveal {\n    ($ui:expr) => { $ui.scroll_to_me(None) };\n}\nuse crate::design::register_row as note;",
    )]);
    assert_eq!(
        named.counts,
        Counts::from([
            (site("scroll-request", "src/a.rs", "draw"), 1),
            (site("row-register", "src/a.rs", "draw"), 1),
            (site("row-clip", "src/a.rs", "draw"), 1),
            (site("scroll-request", "src/a.rs", "(file)"), 1),
        ])
    );
    let escapes = named.escapes.join("\n");
    for expected in [
        "src/a.rs:3 `draw`: a row-register path named but not called",
        "src/a.rs:5 `draw`: a row-clip path named but not called",
        "src/a.rs:8 `(file)`: a scroll-request path inside `macro_rules!`",
        "src/a.rs:10: an alias",
    ] {
        assert!(escapes.contains(expected), "{expected}\n{escapes}");
    }
    assert_eq!(named.escapes.len(), 4, "{escapes}");
    // `Footer::pane` through a type alias in the design system (F2), or through a nested `self`
    // import (re-check N3): each alias fails.
    let host = at(&[(
        "src/design/pane.rs",
        "type Host<'a> = Footer<'a>;\nuse crate::design::Footer::{self as Hand};\nfn show(h: Host, ui: &mut Ui) {\n    Host::pane(h, ui, Body::Content, |_| ());\n}",
    )]);
    let aliased: Vec<&str> = host
        .escapes
        .iter()
        .map(|e| &e[..e.find(": ").unwrap()])
        .collect();
    assert_eq!(
        aliased,
        ["src/design/pane.rs:1", "src/design/pane.rs:2"],
        "{:?}",
        host.escapes
    );
    assert!(
        host.escapes.iter().all(|e| e.contains(": an alias")),
        "{:?}",
        host.escapes
    );
    // The Sessions wrapper passed as a value (re-check N1) is a use, and an escape.
    let wrapper = at(&[(
        "src/sessions/page.rs",
        "fn body(reason: Option<FillsReason>) -> Body {\n    reason.map(fills_body).unwrap_or(Body::Content)\n}",
    )]);
    assert_eq!(
        wrapper.counts,
        Counts::from([(site("fills", "src/sessions/page.rs", "body"), 1)])
    );
    assert!(
        wrapper.escapes.len() == 1
            && wrapper.escapes[0].contains("a fills path named but not called"),
        "{:?}",
        wrapper.escapes
    );
    // Words, a call with too few arguments, one feeder, a nested function, test items.
    let quiet = counts(
        "fn draw(p: Processing, ui: &mut Ui) {\n    let w = \"ui.scroll_to_rect(r)\";\n    p.pane(ui);\n    s.footer_messages(&mut f);\n    fn inner(ui: &mut Ui) {\n        ui.scroll_to_me(None);\n    }\n}\n#[cfg(test)]\nmod tests {\n    fn t(ui: &mut Ui) { ui.scroll_to_rect(r, None); }\n}\n#[test]\nfn u() { Fills::because(r); }",
    );
    assert_eq!(
        quiet,
        Counts::from([
            (site("scroll-request", "src/a.rs", "inner"), 1),
            (site("footer-feed", "src/a.rs", "draw"), 1),
        ])
    );
    // Two owners' messages in one footer; extracting one feeder into a helper is a new site
    // (re-check N4).
    let mixed =
        counts("fn settings(f: &mut Footer) { a.footer_messages(f); B::footer_messages(b, f); }");
    assert_eq!(
        mixed,
        Counts::from([(site("footer-feed", "src/a.rs", "settings"), 2)])
    );
    let fed = counts(
        "fn settings(f: &mut Footer) {\n    a.footer_messages(f);\n    feed(b, f);\n}\nfn feed(b: &B, f: &mut Footer) {\n    b.footer_messages(f);\n}",
    );
    let grown = growth(&fed, &mixed, "", "B0");
    assert!(
        grown.len() == 1
            && grown[0].contains("footer-feed / src/a.rs / feed: 1 here, 0 in the base"),
        "{grown:?}"
    );
    // Only `cfg(test)` declarations make a file test-only, never its name (F3): `live_tests.rs`
    // loads without `cfg(test)` and counts, as does `#[path]`'s file; `gallery.rs` does not.
    let modules = at(&[
        (
            "src/lib.rs",
            "mod live_tests;\nmod sessions;\n#[cfg(test)]\nmod gallery;",
        ),
        (
            "src/live_tests.rs",
            "fn live(ui: &mut Ui) {\n    ui.scroll_to_me(None);\n}",
        ),
        (
            "src/gallery.rs",
            "fn g(f: Footer, ui: &mut Ui) {\n    f.show_by_hand(ui);\n}",
        ),
        ("src/sessions.rs", "pub mod page;"),
        ("src/sessions/page.rs", "#[path = \"carry.rs\"]\nmod carry;"),
        (
            "src/sessions/carry.rs",
            "fn c(ui: &mut Ui) {\n    ui.scroll_to_me(None);\n}",
        ),
    ]);
    assert_eq!(
        modules.counts,
        Counts::from([
            (site("scroll-request", "src/live_tests.rs", "live"), 1),
            (site("scroll-request", "src/sessions/carry.rs", "c"), 1),
        ])
    );
    assert_eq!(growth(&modules.counts, &Counts::new(), "", "B0").len(), 2);
}
