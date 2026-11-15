//! The legacy host freeze (migration 32, layout PR 0, ruling L12). Tests only.
//!
//! `src/design/legacy_hosts.tsv` lists every call of a layout or host path that the pane
//! mechanism replaces ([`CAPABILITIES`]), by the function that holds it (`Type::name` in an
//! `impl` or a trait), with the pane's owner. Calls are read from the tokens
//! (`layout::bans::calls`), so words in strings and comments never count; `#[test]` and
//! `#[cfg(test)]` items and test-only module files do not count.
//!
//! - The list is exact: a new call fails, and fewer calls ask to lower the row.
//! - No row grows against its base. The calls are counted again in the source of the merge base
//!   of `HEAD` and `origin/main` (read with `git`; `SPLAT_LEGACY_BASE` names another ref), so a
//!   call added together with its row fails, and so does a new function around a counted call
//!   (a wrapper is a new site). Only the lead raises a site, in the gate's raise file
//!   (`SPLAT_STILLNESS_RAISE`), one line per site: `legacy-host / <capability> / <file> /
//!   <function> => <reason>`. A base that cannot be read fails.
//! - `src/design/c3_ledger.tsv` lists the 85 round-31 C3 deductions: report:ID, root, the steps
//!   of ruling L1 that own it, whether the layout design names it there or only its root, and
//!   its status. It claims no geometry. A row is never removed; its owning step closes it.
use crate::design::layout::bans::{Shape, calls};
use std::collections::{BTreeMap, BTreeSet};
use std::process::Command;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

const MANIFEST: &str = include_str!("../legacy_hosts.tsv");
const LEDGER: &str = include_str!("../c3_ledger.tsv");
const LEDGER_PATH: &str = "src/design/c3_ledger.tsv";

/// Where a capability is counted.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Scope {
    /// Everywhere, `src/design/` too: a legacy design path, which a new part could wrap again.
    Everywhere,
    /// Outside `src/design/`: a raw egui call, which the design system owns.
    App,
}

/// The capabilities the pane mechanism replaces (layout design §4.3), each with how its calls
/// are written, split at spaces (`bans::Shape::of`: `.name` a method, `.name/n` one with at
/// least `n` arguments, `Owner::name` a path, a bare name a free function).
const CAPABILITIES: [(&str, Scope, &str); 11] = [
    // A body drawn by a closure into a pane (`Footer::pane`, `Messages::pane` and kin).
    (
        "pane-host",
        Scope::Everywhere,
        ".pane/3 .pane_content/3 .pane_editor/2 Footer::pane Footer::pane_content \
         Footer::pane_editor Messages::pane Messages::pane_content Messages::pane_editor",
    ),
    // A footer placed by hand.
    (
        "hand-footer",
        Scope::Everywhere,
        ".show_by_hand Footer::show_by_hand",
    ),
    // A body that manages its own room, also through the Sessions wrapper `fills(reason)`:
    // each Sessions pane that invokes it counts.
    ("fills", Scope::Everywhere, "Fills::because fills"),
    // Rows drawn by a closure for the part of a list in view.
    (
        "virtual-rows",
        Scope::App,
        ".show_rows .show_viewport ScrollColumn::show_rows ScrollColumn::show_viewport",
    ),
    // A row registered after it painted, for the old whole-row mask.
    ("row-register", Scope::App, "register_row"),
    // A clip that a caller sets.
    (
        "row-clip",
        Scope::App,
        ".set_clip_rect .with_clip_rect .shrink_clip_rect",
    ),
    // A scroll request outside the reveal part (B6's list, PR 403).
    (
        "scroll-request",
        Scope::App,
        ".scroll_to_rect .scroll_to_rect_animation .scroll_to_cursor .scroll_to_cursor_animation \
         .scroll_to_me .scroll_to_me_animation .scroll_with_delta .scroll_with_delta_animation",
    ),
    // An offset that a caller writes into a scroll area.
    (
        "scroll-offset",
        Scope::App,
        ".vertical_scroll_offset .horizontal_scroll_offset .scroll_offset",
    ),
    // A raw scroll area: Clippy refuses it outside a counted row, and here it cannot grow.
    (
        "scroll-area",
        Scope::App,
        "ScrollArea::vertical ScrollArea::both ScrollArea::horizontal ScrollArea::neither \
         ScrollArea::new",
    ),
    // The allowed host composition: the page regions that hold panes.
    (
        "host-region",
        Scope::App,
        "CentralPanel::default SidePanel::left SidePanel::right TopBottomPanel::top \
         TopBottomPanel::bottom Column::panel",
    ),
    // The messages of more than one owner fed into one footer: a mixed task footer. A function
    // that feeds one counts nothing.
    (
        "mixed-footer",
        Scope::App,
        ".footer_messages footer_messages",
    ),
];

/// A counted site: capability, file, function.
type Site = (String, String, String);
type Counts = BTreeMap<Site, usize>;

/// The owner and the name of each path the freeze counts, for the source lint's `banned-alias`
/// rule (`layout::bans::syntax_rules`): an alias would hide a call from the count.
pub(crate) fn alias_paths() -> Vec<(String, String)> {
    CAPABILITIES
        .iter()
        .flat_map(|(_, _, spellings)| spellings.split_whitespace())
        .filter_map(|spelling| Shape::of(spelling).path())
        .collect()
}

/// The functions and test items of one file, by line (`syn`).
#[derive(Default)]
struct Items {
    /// Each function: its name (`Type::name` in an `impl` or a trait), first and last line.
    functions: Vec<(String, usize, usize)>,
    /// The first and last line of each test item.
    tests: Vec<(usize, usize)>,
    /// Test-only modules declared here with a file of their own (`#[cfg(test)] mod x;`).
    test_modules: Vec<String>,
    owner: Option<String>,
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
        match (is_test(&node.attrs), &node.content) {
            (true, Some(_)) => self.tests.push(lines(node)),
            (true, None) => self.test_modules.push(node.ident.to_string()),
            (false, _) => visit::visit_item_mod(self, node),
        }
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

/// The path, less `.rs`, of the child module `name` that `path` declares (`src/a.rs` and
/// `src/a/mod.rs` → `src/a/name`; `src/lib.rs` → `src/name`).
fn child(path: &str, name: &str) -> String {
    let dir = match path.rsplit_once('/') {
        Some((dir, "mod.rs" | "lib.rs" | "main.rs")) => dir,
        _ => path.trim_end_matches(".rs"),
    };
    format!("{dir}/{name}")
}

/// Each counted call in `files` (path, text), by capability and the function that holds it.
fn counts(files: &[(String, String)]) -> Result<Counts, String> {
    let patterns: Vec<(usize, Shape)> = CAPABILITIES
        .iter()
        .enumerate()
        .flat_map(|(at, (_, _, spellings))| {
            spellings
                .split_whitespace()
                .map(move |s| (at, Shape::of(s)))
        })
        .collect();
    let shapes: Vec<Shape> = patterns.iter().map(|(_, shape)| shape.clone()).collect();
    let mut read = Vec::new();
    let mut test_only = Vec::new();
    for (path, text) in files {
        let file = syn::parse_file(text).map_err(|error| format!("{path}: {error}"))?;
        let mut items = Items::default();
        items.visit_file(&file);
        test_only.extend(items.test_modules.iter().map(|name| child(path, name)));
        read.push((path, text, items));
    }
    let mut counts = Counts::new();
    for (path, text, items) in read {
        let module = path.trim_end_matches(".rs").trim_end_matches("/mod");
        if path.ends_with("tests.rs")
            || test_only
                .iter()
                .any(|dir| module == dir || module.starts_with(&format!("{dir}/")))
        {
            continue;
        }
        let own = path.starts_with("src/design/");
        for (line, at) in calls(text, &shapes).map_err(|error| format!("{path}: {error}"))? {
            let (capability, scope, _) = CAPABILITIES[patterns[at].0];
            if (own && scope == Scope::App)
                || items
                    .tests
                    .iter()
                    .any(|(first, last)| (*first..=*last).contains(&line))
            {
                continue;
            }
            let site = (capability.to_owned(), path.clone(), items.holder(line));
            *counts.entry(site).or_default() += 1;
        }
    }
    counts.retain(|(capability, ..), count| capability != "mixed-footer" || *count > 1);
    Ok(counts)
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
        .current_dir(env!("CARGO_MANIFEST_DIR"))
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

/// The base commit: the merge base of `HEAD` and `origin/main`, or of `SPLAT_LEGACY_BASE`.
fn base_commit() -> Result<String, String> {
    let named = std::env::var("SPLAT_LEGACY_BASE").unwrap_or_else(|_| "origin/main".to_owned());
    git(&["merge-base", "HEAD", &named])
}

/// The Rust files of `src/` and `crates/core/src/` at `commit`.
fn files_at(commit: &str) -> Result<Vec<(String, String)>, String> {
    let listed = git(&[
        "ls-tree",
        "-r",
        "--name-only",
        commit,
        "--",
        "src",
        "crates/core/src",
    ])?;
    listed
        .lines()
        .filter(|path| path.ends_with(".rs") && !path.ends_with("tests.rs"))
        .map(|path| {
            Ok((
                path.to_owned(),
                git(&["show", &format!("{commit}:{path}")])?,
            ))
        })
        .collect()
}

/// What the lint test reports for the freeze: the manifest against the calls of `files` (the
/// working tree), and the calls against the base.
pub(super) fn problems(files: &[(String, String)]) -> Vec<String> {
    let found = match counts(files) {
        Ok(found) => found,
        Err(error) => return vec![format!("legacy host freeze: {error}")],
    };
    let mut problems = match manifest(MANIFEST) {
        Ok(listed) => against_manifest(&found, &listed),
        Err(faults) => faults,
    };
    let unavailable = |why: String| {
        format!(
            "legacy host freeze: the base cannot be read, so no site was compared with it (ruling L12): {why}. Fetch main (`git fetch origin main`) so that `origin/main` exists, or name the base in SPLAT_LEGACY_BASE."
        )
    };
    match base_commit().and_then(|commit| Ok((files_at(&commit)?, commit))) {
        Ok((base_files, commit)) => match counts(&base_files) {
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

/// The freeze's own cases: a call raised with its row, a wrapper, a test item, a base.
#[test]
fn the_legacy_freeze_finds_what_it_should() {
    let file = |text: &str| vec![("src/a.rs".to_owned(), text.to_owned())];
    let site = |capability: &str, function: &str| {
        (
            capability.to_owned(),
            "src/a.rs".to_owned(),
            function.to_owned(),
        )
    };
    let base = counts(&file(
        "fn draw(footer: Footer, ui: &mut Ui) {\n    footer.show_by_hand(ui);\n}",
    ))
    .unwrap();
    assert_eq!(base, Counts::from([(site("hand-footer", "draw"), 1)]));
    // A second call, with its row raised: the manifest agrees, the base does not.
    let two = counts(&file(
        "fn draw(f: Footer, g: Footer, ui: &mut Ui) {\n    f.show_by_hand(ui);\n    g.show_by_hand(ui);\n}",
    ))
    .unwrap();
    assert!(against_manifest(&two, &two).is_empty());
    let raised = growth(&two, &base, "", "B0");
    assert!(
        raised.len() == 1 && raised[0].contains("draw: 2 here, 1 in the base B0"),
        "{raised:?}"
    );
    // The lead's raise line lets exactly that site pass.
    let line = "legacy-host / hand-footer / src/a.rs / draw => ruled";
    assert!(growth(&two, &base, line, "B0").is_empty());
    // A wrapper moves the call into a new site: it fails against the base.
    let wrapped = counts(&file(
        "impl Host {\n    fn by_hand(f: Footer, ui: &mut Ui) -> f32 {\n        f.show_by_hand(ui)\n    }\n}\nfn draw(f: Footer, ui: &mut Ui) {\n    Host::by_hand(f, ui);\n}",
    ))
    .unwrap();
    assert_eq!(
        wrapped,
        Counts::from([(site("hand-footer", "Host::by_hand"), 1)])
    );
    assert_eq!(growth(&wrapped, &base, "", "B0").len(), 1);
    // Words, a call with too few arguments, one feeder, a nested function, test items.
    let quiet = counts(&file(
        "fn draw(p: Processing, ui: &mut Ui) {\n    let w = \"ui.scroll_to_rect(r)\";\n    p.pane(ui);\n    s.footer_messages(&mut f);\n    fn inner(ui: &mut Ui) {\n        ui.scroll_to_me(None);\n    }\n}\n#[cfg(test)]\nmod tests {\n    fn t(ui: &mut Ui) { ui.scroll_to_rect(r, None); }\n}\n#[test]\nfn u() { Fills::because(r); }",
    ))
    .unwrap();
    assert_eq!(quiet, Counts::from([(site("scroll-request", "inner"), 1)]));
    // Two owners' messages in one footer.
    let mixed = counts(&file(
        "fn settings(f: &mut Footer) { a.footer_messages(f); B::footer_messages(b, f); }",
    ))
    .unwrap();
    assert_eq!(mixed, Counts::from([(site("mixed-footer", "settings"), 2)]));
    // A test-only module's file, and raw egui calls inside the design system, do not count.
    let files = vec![
        (
            "src/design/mod.rs".to_owned(),
            "#[cfg(test)]\nmod gallery;".to_owned(),
        ),
        (
            "src/design/gallery.rs".to_owned(),
            "fn g(f: Footer, ui: &mut Ui) { f.show_by_hand(ui); }".to_owned(),
        ),
        (
            "src/design/layout.rs".to_owned(),
            "fn area(ui: &mut Ui) { ui.scroll_to_rect(r, None); }".to_owned(),
        ),
    ];
    assert!(counts(&files).unwrap().is_empty());
}
