//! No raw colours or text sizes outside the design system, and no straight apostrophe in UI
//! words; phrases and facts have one owner. The scan reads every Rust file of the app and the
//! logic crates, the GPUI pages (`src/gui/`) too. The allowlist can only shrink: a count above
//! it fails, and a count below it asks you to lower it.
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

/// Whether a label is only arrows (and spaces): a bare back or forward arrow with no word.
fn arrow_only(label: &str) -> bool {
    let phosphor = [
        egui_phosphor::regular::ARROW_LEFT,
        egui_phosphor::regular::ARROW_RIGHT,
        egui_phosphor::regular::ARROW_UP,
        egui_phosphor::regular::ARROW_DOWN,
        egui_phosphor::regular::ARROW_U_UP_LEFT,
        egui_phosphor::regular::ARROW_BEND_UP_LEFT,
        egui_phosphor::regular::ARROW_ARC_LEFT,
        egui_phosphor::regular::ARROW_CIRCLE_LEFT,
        egui_phosphor::regular::ARROW_FAT_LEFT,
        egui_phosphor::regular::CARET_LEFT,
        egui_phosphor::regular::CARET_RIGHT,
    ];
    let arrow = |c: char| {
        matches!(c as u32, 0x2190..=0x21FF | 0x27F0..=0x27FF | 0x2900..=0x297F | 0x2B00..=0x2BFF)
            || "<>‹›«»".contains(c)
            || phosphor.iter().any(|glyph| glyph.starts_with(c))
    };
    let label = label.trim();
    !label.is_empty() && label.chars().filter(|c| !c.is_whitespace()).all(arrow)
}

/// The name of the deprecated bare back arrow in `design::icons`, spelled in two parts so that
/// its STAGED text count (`design::results::STAGED`) counts only real uses.
const LEGACY: &str = concat!("LEGACY", "_BACK");

/// Whether a path's last segment names an arrow glyph: the legacy back arrow, or any of
/// Phosphor's `ARROW_…` and `CARET_…` families (`egui_phosphor::regular::ARROW_ELBOW_UP_LEFT`).
fn arrow_name(name: &str) -> bool {
    name == LEGACY || name.starts_with("ARROW_") || name.starts_with("CARET_")
}

/// Whether a label argument is an arrow alone: a string literal of arrows, or a path whose last
/// segment names an arrow ([`arrow_name`]), also inside parentheses or a reference.
fn arrow_argument(argument: Option<&syn::Expr>) -> bool {
    match argument {
        Some(syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(label),
            ..
        })) => arrow_only(&label.value()),
        Some(syn::Expr::Path(path)) => path
            .path
            .segments
            .last()
            .is_some_and(|segment| arrow_name(&segment.ident.to_string())),
        Some(syn::Expr::Reference(reference)) => arrow_argument(Some(&reference.expr)),
        Some(syn::Expr::Paren(inner)) => arrow_argument(Some(&inner.expr)),
        Some(syn::Expr::Group(inner)) => arrow_argument(Some(&inner.expr)),
        _ => false,
    }
}

/// Controls whose label is only an arrow, read by `syn`, on lines up to `last_line`: the first
/// argument of `Action::quiet`, `secondary`, `primary` or `destructive`, of `Link::new` and
/// `Button::new`, of a `.link(…)`, `.button(…)` or `.small_button(…)` call, and the icon of
/// `icon_button(ui, icon, name)` and `icon_button_with_hint(ui, icon, name, hint)`, when it is a literal of arrows alone or names an arrow glyph
/// (the legacy back arrow, `egui_phosphor::regular::ARROW_LEFT`). Limit: an arrow behind another
/// name (a local `const` or binding) passes; the legacy arrow's STAGED count is the second net.
fn arrow_labels(text: &str, last_line: usize) -> usize {
    use syn::{
        spanned::Spanned,
        visit::{self, Visit},
    };
    struct Labels {
        last_line: usize,
        bad: usize,
    }
    impl<'ast> Visit<'ast> for Labels {
        fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
            if let syn::Expr::Path(path) = &*call.func {
                let names: Vec<String> = path
                    .path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect();
                let last = names.last().map_or("", String::as_str);
                let owner = names
                    .len()
                    .checked_sub(2)
                    .map_or("", |at| names[at].as_str());
                let label = if owner == "Action"
                    && ["quiet", "secondary", "primary", "destructive"].contains(&last)
                    || ["Link", "Button"].contains(&owner) && last == "new"
                {
                    call.args.first()
                } else if ["icon_button", "icon_button_with_hint"].contains(&last) {
                    call.args.iter().nth(1)
                } else {
                    None
                };
                if label.is_some()
                    && call.span().start().line <= self.last_line
                    && arrow_argument(label)
                {
                    self.bad += 1;
                }
            }
            visit::visit_expr_call(self, call);
        }
        fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
            if ["link", "button", "small_button"].contains(&call.method.to_string().as_str())
                && call.span().start().line <= self.last_line
                && arrow_argument(call.args.first())
            {
                self.bad += 1;
            }
            visit::visit_expr_method_call(self, call);
        }
    }
    let Ok(file) = syn::parse_file(text) else {
        return 0;
    };
    let mut labels = Labels { last_line, bad: 0 };
    labels.visit_file(&file);
    labels.bad
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
/// module (`code`, the first `last_line` lines of `text`). Calls are read by `syn`, never matched
/// as text. Each rule here is a count per file in the allowlist. "Shrink only" holds for a count
/// alone: a change that raises a count and its allowance together passes this test, so a
/// reviewer reads every raised line (AGENTS.md).
///
/// - `reason-end-mark`: a blocked reason without `.`, `?` or `!`.
/// - `phrase-owner`: a phrase in a string literal outside its owner file.
/// - `arrow-label`: a control whose label is only an arrow, as a literal or by the glyph's name
///   (`Action::quiet("←")`, the legacy back arrow by name; see `arrow_labels`). An
///   editor's back is `EditorHead`; a back link that starts a line is `TitleLine::back`; both
///   always show their word (K4b, R38).
fn message_rules(path: &str, text: &str, last_line: usize) -> Vec<&'static str> {
    let mut found = Vec::new();
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
    found.extend((0..arrow_labels(text, last_line)).map(|_| "arrow-label"));
    found
}

/// Format strings are Rust literals, so read them through `syn`, including raw and multiline
/// literals. The formatter owns HP fractions; sheet facts go through the separator-free part.
///
/// `fact-owner` (ruling M5): a sheet fact is drawn only by `design::fact`. Outside
/// `src/design/fact.rs`, read by `syn` per function (a method too), following each value to
/// where it is used, it counts a local copy of the part:
/// - a fact format: a format string with `{…}: {…}` whose value is joined by the middle dot,
///   given as an argument (`values.join(" · ")`) or read through a name bound to such a join
///   (`{values}`), whatever the helper is called;
/// - a tile label with no colon: a format string of placeholders and spaces only (`{}{} {}`,
///   raw or not) given as the label of a stat tile (`stat_tile`, `roll_tile`, `metric`,
///   `roll_metric`), directly or through a name bound to it; the part's form is
///   `fact::tile_label`;
/// - a rolling tile: `rolls::link(…, Type::NumeralLarge, …)`; the part is `fact::roll_tile`.
///
/// A name is bound to a value by a `let` of it or of another such name, typed or not, also
/// through a chain (`let b = a.clone(); let c: &str = &b;`: `clone`, `to_owned`, `to_string`,
/// `as_str`, `into`, a reference); a new binding of the name ends it. The named legacy exceptions are counts in
/// the allowlist.
///
/// Named limits (ruling of the PR 470 review): the rule reads one function at a time. It does
/// not find a join made in another function and returned (`fn joined() -> String`), nor a
/// label made in a closure and passed through another function (`label()`) into `stat_tile`.
/// Review finds those; `docs/design-system.md` (Party) names them too.
fn format_rules(path: &str, text: &str, last_line: usize) -> Vec<&'static str> {
    use syn::spanned::Spanned;
    use syn::visit::Visit;
    /// The stat tile calls whose second argument is the tile's label.
    const TILES: [&str; 4] = ["stat_tile", "roll_tile", "metric", "roll_metric"];
    /// What one function does that a fact's part owns.
    #[derive(Default, Clone)]
    struct Fact {
        /// The names bound to values joined by the middle dot.
        dot_names: std::collections::BTreeSet<String>,
        /// The names bound to a format of placeholders and spaces only, each with its format
        /// site: a name bound to another name has that name's site.
        bare_names: std::collections::BTreeMap<String, usize>,
        /// The format sites of `bare_names` so far, and those already counted (one count per
        /// format site, however often and under however many names it is used).
        sites: usize,
        counted: std::collections::BTreeSet<usize>,
        /// The local copies found.
        copies: usize,
    }
    /// A format string of placeholders and spaces only, two placeholders or more and a space.
    fn bare(words: &str) -> bool {
        if words.contains("{{") || words.contains("}}") {
            return false;
        }
        let (mut places, mut rest, mut inside) = (0, String::new(), false);
        for c in words.chars() {
            match (inside, c) {
                (false, '{') => inside = true,
                (true, '}') => {
                    inside = false;
                    places += 1;
                }
                (true, _) => {}
                (false, c) => rest.push(c),
            }
        }
        places >= 2 && !rest.is_empty() && rest.chars().all(|c| c == ' ')
    }
    /// A macro's arguments, when they parse as expressions.
    fn macro_args(mac: &syn::Macro) -> Option<Vec<syn::Expr>> {
        mac.parse_body_with(
            syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated,
        )
        .ok()
        .map(|args| args.into_iter().collect())
    }
    /// The format string of a macro call (`format!("…", …)`), raw or not.
    fn template(expr: &syn::Expr) -> Option<String> {
        let syn::Expr::Macro(call) = expr else {
            return None;
        };
        match macro_args(&call.mac)?.first()? {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(words),
                ..
            }) => Some(words.value()),
            _ => None,
        }
    }
    /// `expr` without the references around it (`&label`).
    fn unref(expr: &syn::Expr) -> &syn::Expr {
        match expr {
            syn::Expr::Reference(reference) => unref(&reference.expr),
            other => other,
        }
    }
    /// The name whose value `expr` is (`a`, `&a`, `a.clone()`, `(a).to_string()`), if any.
    fn alias(expr: &syn::Expr) -> Option<String> {
        match unref(expr) {
            syn::Expr::Paren(inner) => alias(&inner.expr),
            syn::Expr::MethodCall(call)
                if call.args.is_empty()
                    && ["clone", "to_owned", "to_string", "as_str", "into"]
                        .iter()
                        .any(|method| call.method == method) =>
            {
                alias(&call.receiver)
            }
            other => ident(other),
        }
    }
    /// The one name `expr` is (`label`), if it is a plain name.
    fn ident(expr: &syn::Expr) -> Option<String> {
        match unref(expr) {
            syn::Expr::Path(path) => path.path.get_ident().map(ToString::to_string),
            _ => None,
        }
    }
    /// The names a pattern binds (`label`, `(label, key)`).
    fn bound(pattern: &syn::Pat) -> Vec<String> {
        struct Names(Vec<String>);
        impl<'ast> Visit<'ast> for Names {
            fn visit_pat_ident(&mut self, name: &'ast syn::PatIdent) {
                self.0.push(name.ident.to_string());
                syn::visit::visit_pat_ident(self, name);
            }
        }
        let mut names = Names(Vec::new());
        names.visit_pat(pattern);
        names.0
    }
    /// Whether `expr` joins values with the middle dot (`.join(" · ")`).
    fn dot_join(expr: &syn::Expr) -> bool {
        struct Join(bool);
        impl<'ast> Visit<'ast> for Join {
            fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
                if call.method == "join"
                    && let Some(syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(separator),
                        ..
                    })) = call.args.first()
                    && separator.value().contains('·')
                {
                    self.0 = true;
                }
                syn::visit::visit_expr_method_call(self, call);
            }
        }
        let mut join = Join(false);
        join.visit_expr(expr);
        join.0
    }
    /// The names a format string reads inline: `{values}`, `{values:>8}`.
    fn inline_names(words: &str) -> Vec<String> {
        words
            .split('{')
            .skip(1)
            .filter_map(|rest| rest.split('}').next())
            .map(|inside| inside.split(':').next().unwrap_or("").trim().to_owned())
            .filter(|name| {
                !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
            })
            .collect()
    }
    struct Formats<'a> {
        path: &'a str,
        function: String,
        last_line: usize,
        found: Vec<&'static str>,
        fact: Fact,
    }
    impl Formats<'_> {
        /// Visits one function's body with its own fact state, then counts its copies.
        fn function(&mut self, name: String, visit: impl FnOnce(&mut Self)) {
            let previous = std::mem::replace(&mut self.function, name);
            let outer = std::mem::take(&mut self.fact);
            visit(self);
            if !self.path.ends_with("src/design/fact.rs") {
                self.found
                    .extend((0..self.fact.copies).map(|_| "fact-owner"));
            }
            self.fact = outer;
            self.function = previous;
        }
        fn counts(&self, span: proc_macro2::Span) -> bool {
            span.start().line <= self.last_line
        }
    }
    impl<'ast> Visit<'ast> for Formats<'_> {
        fn visit_local(&mut self, local: &'ast syn::Local) {
            // What the value is, read before the binding ends what its names held: a join, a
            // bare format, or another name's value (`let b = a.clone();`).
            let init = local.init.as_ref().map(|init| &*init.expr);
            let source = init.and_then(alias);
            let dots = init.is_some_and(dot_join)
                || source
                    .as_ref()
                    .is_some_and(|source| self.fact.dot_names.contains(source));
            let site = if init.and_then(template).is_some_and(|words| bare(&words)) {
                self.fact.sites += 1;
                Some(self.fact.sites)
            } else {
                source.and_then(|source| self.fact.bare_names.get(&source).copied())
            };
            // A new binding ends what the name held before.
            for name in bound(&local.pat) {
                self.fact.dot_names.remove(&name);
                self.fact.bare_names.remove(&name);
            }
            // A typed binding (`let shown: String = …`) binds its inner name the same way.
            let pattern = match &local.pat {
                syn::Pat::Type(typed) => &*typed.pat,
                other => other,
            };
            if let syn::Pat::Ident(name) = pattern {
                let name = name.ident.to_string();
                if dots {
                    self.fact.dot_names.insert(name.clone());
                }
                if let Some(site) = site {
                    self.fact.bare_names.insert(name, site);
                }
            }
            syn::visit::visit_local(self, local);
        }
        fn visit_expr_for_loop(&mut self, looped: &'ast syn::ExprForLoop) {
            for name in bound(&looped.pat) {
                self.fact.dot_names.remove(&name);
                self.fact.bare_names.remove(&name);
            }
            syn::visit::visit_expr_for_loop(self, looped);
        }
        fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
            self.function(item.sig.ident.to_string(), |this| {
                syn::visit::visit_item_fn(this, item);
            });
        }
        fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
            self.function(item.sig.ident.to_string(), |this| {
                syn::visit::visit_impl_item_fn(this, item);
            });
        }
        fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
            if let syn::Expr::Path(function) = &*call.func
                && self.counts(function.path.span())
            {
                let names: Vec<String> = function
                    .path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect();
                let last = names.last().map_or("", String::as_str);
                // A tile's label with no colon, given here or through a name.
                if TILES.contains(&last)
                    && let Some(label) = call.args.iter().nth(1)
                {
                    if template(unref(label)).is_some_and(|words| bare(&words)) {
                        self.fact.copies += 1;
                    } else if let Some(site) =
                        alias(label).and_then(|name| self.fact.bare_names.get(&name).copied())
                        && self.fact.counted.insert(site)
                    {
                        self.fact.copies += 1;
                    }
                }
                // A rolling tile by hand.
                let numeral = call.args.iter().any(|argument| {
                    matches!(argument, syn::Expr::Path(path)
                        if path.path.segments.last().is_some_and(|segment| segment.ident == "NumeralLarge"))
                });
                if names.ends_with(&["rolls".to_owned(), "link".to_owned()]) && numeral {
                    self.fact.copies += 1;
                }
            }
            syn::visit::visit_expr_call(self, call);
        }
        fn visit_macro(&mut self, mac: &'ast syn::Macro) {
            // Macro tokens are not expressions in syn's default visitor. Parse expression
            // arguments here, so nested format calls and literal labels are checked too.
            if let Ok(args) = mac.parse_body_with(
                syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated,
            ) {
                // A fact format: `{…}: {…}` whose value is joined by the middle dot, given as an
                // argument or read inline through a name bound to such a join.
                if let Some(syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(words),
                    ..
                })) = args.first()
                    && self.counts(words.span())
                    && words.value().contains("}: {")
                {
                    let names = &self.fact.dot_names;
                    let inline = inline_names(&words.value())
                        .iter()
                        .any(|name| names.contains(name));
                    let given = args.iter().skip(1).any(|argument| {
                        dot_join(argument)
                            || alias(argument).is_some_and(|name| names.contains(&name))
                    });
                    if inline || given {
                        self.fact.copies += 1;
                    }
                }
                let mut nested = Formats {
                    path: self.path,
                    function: self.function.clone(),
                    last_line: self.last_line,
                    found: Vec::new(),
                    fact: Fact {
                        copies: 0,
                        ..self.fact.clone()
                    },
                };
                for argument in &args {
                    nested.visit_expr(argument);
                }
                self.found.extend(nested.found);
                self.fact.copies += nested.fact.copies;
                self.fact.counted = nested.fact.counted;
                self.fact.sites = nested.fact.sites;
            }
        }
        fn visit_path(&mut self, path: &'ast syn::Path) {
            if path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "CHOICES_ROLE")
                && path
                    .segments
                    .last()
                    .is_some_and(|segment| segment.ident.span().start().line <= self.last_line)
                && !self.path.ends_with("src/design/popups.rs")
            {
                self.found.push("popup-marker-owner");
            }
            syn::visit::visit_path(self, path);
        }
        fn visit_lit_str(&mut self, literal: &'ast syn::LitStr) {
            if literal.span().start().line > self.last_line {
                return;
            }
            let words = literal.value();
            if words == ["Dropdown", "choices"].join(" ")
                && !self.path.ends_with("src/design/popups.rs")
            {
                self.found.push("popup-marker-owner");
            }

            let fraction = words.split_once('/').is_some_and(|(left, right)| {
                let left = left.trim_end();
                let right = right.trim_start();
                if !left.ends_with('}') || !right.starts_with('{') {
                    return false;
                }
                let current = left
                    .rsplit_once('{')
                    .map_or("", |(_, tail)| tail.trim_end_matches('}'));
                let maximum = right[1..].split('}').next().unwrap_or("");
                ["hp", "current_hp", "current"].contains(&current)
                    || ["max", "max_hp", "maximum"].contains(&maximum)
                    || (current.is_empty()
                        && maximum.is_empty()
                        && (words.contains("HP") || words == "{} / {}"))
            });
            if fraction {
                self.found.push("hp-fraction-owner");
            }
            if ["src/party.rs", "src/party_support/sheets.rs"]
                .iter()
                .any(|path| self.path.ends_with(path))
                && ["overview", "detail_intro", "background", "details"]
                    .contains(&self.function.as_str())
                && (["{label}: ", "{label} · ", "{}: ", "{} · {}"]
                    .iter()
                    .any(|prefix| words.starts_with(prefix))
                    || [
                        "Coins",
                        "Resistance",
                        "Immunity",
                        "Vulnerability",
                        "Darkvision",
                        "Feature",
                        "Feats",
                    ]
                    .iter()
                    .any(|label| {
                        words.starts_with(&format!("{label}: "))
                            || words.starts_with(&format!("{label} · "))
                    }))
            {
                self.found.push("sheet-label-owner");
            }
        }
    }
    let mut formats = Formats {
        path,
        function: String::new(),
        last_line,
        found: Vec::new(),
        fact: Fact::default(),
    };
    formats.visit_file(&syn::parse_file(text).expect("format lint source parses"));
    formats.found
}

/// The design system owns raw drawing values.
fn design_or_capture(path: &str) -> bool {
    path.starts_with("src/design/")
}

fn scan(folder: &Path, counts: &mut BTreeMap<String, usize>) {
    for item in fs::read_dir(folder).expect("read source folder") {
        let path = item.expect("read source entry").path();
        let relative = path.to_string_lossy().replace('\\', "/");
        if path.is_dir() {
            scan(&path, counts);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let text = fs::read_to_string(&path).expect("read source file");
            let root = format!("{}/", crate::workspace().display());
            let key = relative.strip_prefix(&root).unwrap_or(&relative);
            // A file of unit tests, such as `formats_tests.rs`, does not ship either.
            if relative.ends_with("tests.rs") {
                continue;
            }
            // Unit tests may name colours and sizes. They do not ship.
            // `mod tests;` near the top names a file of its own, so only an inline module ends it.
            let code = text.split("#[cfg(test)]\nmod tests {").next().unwrap_or("");
            // Design code owns raw drawing values. The popup marker belongs to its own part.
            if design_or_capture(key) {
                for rule in format_rules(key, &text, code.lines().count())
                    .into_iter()
                    .filter(|rule| *rule == "popup-marker-owner")
                {
                    *counts.entry(format!("{key} {rule}")).or_default() += 1;
                }
                continue;
            }
            for line in code.lines() {
                for rule in violations(line) {
                    *counts.entry(format!("{key} {rule}")).or_default() += 1;
                }
            }
            for rule in message_rules(key, &text, code.lines().count()) {
                *counts.entry(format!("{key} {rule}")).or_default() += 1;
            }
            for rule in format_rules(key, &text, code.lines().count()) {
                *counts.entry(format!("{key} {rule}")).or_default() += 1;
            }
            // A renaming of a banned owner or function, a `CreateReasons` impl by hand (the
            // syntax tree).
            for rule in super::layout::bans::syntax_rules(&text)
                .unwrap_or_else(|error| panic!("{key} does not parse: {error}"))
            {
                *counts.entry(format!("{key} {rule}")).or_default() += 1;
            }
        }
    }
}

#[test]
fn raw_colours_and_sizes_stay_inside_the_design_system() {
    let mut counts = BTreeMap::new();
    // The logic crates write many UI words too, such as a note's warnings.
    for folder in [
        "/src",
        "/crates/campaign/src",
        "/crates/character/src",
        "/crates/content/src",
        "/crates/execution/src",
    ] {
        scan(
            &crate::workspace().join(folder.trim_start_matches('/')),
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
            let fix = match key.rsplit_once(' ').map_or("", |(_, rule)| rule) {
                "straight-apostrophe" => "Use ’ in UI words",
                "reason-end-mark" => "End a blocked reason with `.`, `?` or `!`",
                "hand-create-reasons" => "Declare a form's reasons with `create_reasons!`",
                "banned-alias" => {
                    "Name a banned owner or function as clippy.toml names it: an alias (`use … as …`, `type … =`) hides it from review"
                }
                "phrase-owner" => {
                    "Call the phrase's owner: `XP {` in src/difficulty.rs, `· line {` in `fault_words`, `N of M chosen` in `chosen_count`, a close icon's `Dismiss this …` in `MessageChoice::Dismiss`"
                }
                "fact-owner" => {
                    "Draw a sheet's fact with `design::fact::line`, a tile label that holds its number with `fact::tile_label` and a rolling tile with `fact::roll_tile` (M5)"
                }
                "arrow-label" => {
                    "Give the control its word: an editor's back is `EditorHead` (its `BackTo` gives the word) above its name row; a back link that starts a line is `TitleLine::back` with its word"
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
    // K4b: an action whose label is only an arrow; a word with an arrow, or an icon, passes.
    for label in [
        "←",
        " ← ",
        "‹",
        "\u{2190}",
        egui_phosphor::regular::ARROW_LEFT,
    ] {
        assert_eq!(
            rules(
                "src/a.rs",
                &format!("fn f() {{ Action::quiet({label:?}).show(ui); }}")
            ),
            ["arrow-label"],
            "{label:?}"
        );
    }
    assert_eq!(
        rules(
            "src/a.rs",
            "fn f() { design::Action::secondary(\"→\").show(ui); }"
        ),
        ["arrow-label"]
    );
    // R38: a bare back arrow by name, as both Encounters editors had it, and the other controls
    // that take a label.
    for code in [
        format!(
            "fn f() {{ Action::quiet(icons::{LEGACY}).name(\"Back to encounters\").show(ui); }}"
        ),
        "fn f() { Action::quiet(egui_phosphor::regular::ARROW_LEFT).show(ui); }".to_owned(),
        "fn f() { Action::quiet(ARROW_U_UP_LEFT).show(ui); }".to_owned(),
        "fn f() { Action::quiet(egui_phosphor::regular::ARROW_ELBOW_UP_LEFT).show(ui); }"
            .to_owned(),
        "fn f() { Action::quiet((egui_phosphor::regular::ARROW_LEFT)).show(ui); }".to_owned(),
        "fn f() { Action::quiet((\"←\")).show(ui); }".to_owned(),
        format!("fn f() {{ design::icon_button(ui, icons::{LEGACY}, \"Back\"); }}"),
        "fn f() { icon_button(ui, \"←\", \"Back\"); }".to_owned(),
        "fn f() { icon_button_with_hint(ui, \"←\", \"Back\", \"Back (Alt+Left)\"); }".to_owned(),
        "fn f() { ui.link(\"←\"); }".to_owned(),
        "fn f() { ui.add(egui::Link::new(egui_phosphor::regular::CARET_LEFT)); }".to_owned(),
        "fn f() { ui.add(egui::Button::new(\"‹\")); }".to_owned(),
        format!("fn f() {{ ui.button(&icons::{LEGACY}); }}"),
    ] {
        assert_eq!(rules("src/a.rs", &code), ["arrow-label"], "{code}");
    }
    for code in [
        "fn f() { Action::quiet(\"← Back\").show(ui); }".to_owned(),
        "fn f() { Action::quiet(\"Back\").icon(icons::UNDO).show(ui); }".to_owned(),
        "fn f() { Action::quiet(icons::EXPAND).show(ui); }".to_owned(),
        "fn f() { other::quiet(\"←\"); }".to_owned(),
        format!("fn f() {{ Action::quiet(\"Back\").icon(icons::{LEGACY}).show(ui); }}"),
        "fn f() { icon_button(ui, icons::CLOSE, \"Close\"); }".to_owned(),
        "fn f() { ui.link(\"Encounters\"); }".to_owned(),
    ] {
        assert!(rules("src/a.rs", &code).is_empty(), "{code}");
    }
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
/// per object says its owner with `FieldName::of`; two live widgets with one id are the gate's
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
    names(&crate::workspace().join("src"), &mut found);
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
    let root = crate::workspace().display();
    for file in [
        "Cargo.toml",
        "crates/campaign/Cargo.toml",
        "crates/character/Cargo.toml",
        "crates/content/Cargo.toml",
        "crates/execution/Cargo.toml",
        "src/campaign_packages/Cargo.toml",
        "src/content_ui/Cargo.toml",
        "src/design/Cargo.toml",
        "src/encounters_ui/Cargo.toml",
        "src/library_page/Cargo.toml",
        "src/sessions_ui/Cargo.toml",
        "src/trackers/Cargo.toml",
        "src/world_page/Cargo.toml",
        ".cargo/config.toml",
    ] {
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
fn format_owners_reject_old_fractions_and_sheet_separators() {
    let rules = |path: &str, code: &str| format_rules(path, code, usize::MAX);
    for code in [
        r#"fn f() { let _ = Dropdown::CHOICES_ROLE; }"#,
        r#"fn f() { let _ = "Dropdown choices"; }"#,
    ] {
        for path in [
            "src/party.rs",
            "src/design/components.rs",
            "src/gui/kit/menu.rs",
        ] {
            assert_eq!(rules(path, code), ["popup-marker-owner"]);
        }
        assert!(rules("src/design/popups.rs", code).is_empty());
    }

    for words in [
        "HP {}/{}",
        "{hp} / {max}",
        "{} / {max_hp} HP",
        "{} / {}",
        "{current}/{maximum}",
    ] {
        let code = format!("fn row() {{ let _ = format!({words:?}, 1, 2); }}");
        assert_eq!(
            rules("src/party.rs", &code),
            ["hp-fraction-owner"],
            "{words}"
        );
        assert_eq!(
            rules("src/combat_sync.rs", &code),
            ["hp-fraction-owner"],
            "the owning file cannot bypass its formatter"
        );
    }
    assert_eq!(
        rules(
            "src/combat_sync.rs",
            r##"fn fraction() { format!(r#"{current}/{maximum}"#); }"##
        ),
        ["hp-fraction-owner"],
        "Combat Sync uses the design formatter",
    );
    assert!(
        rules(
            "src/a.rs",
            r#"fn path() { format!("{folder}/{stem}.md"); }"#
        )
        .is_empty()
    );
    for words in [
        "{label}: {value}",
        "{} · {}",
        "Coins · {}",
        "Darkvision: 60 ft.",
    ] {
        for function in ["background", "overview", "detail_intro", "details"] {
            let code = format!("fn {function}() {{ ui.label(format!({words:?}, 1, 2)); }}");
            for path in ["src/party.rs", "src/party_support/sheets.rs"] {
                assert_eq!(
                    rules(path, &code),
                    ["sheet-label-owner"],
                    "{function}: {words}"
                );
            }
        }
    }
    assert!(
        rules(
            "src/party.rs",
            r#"fn sheet_label() { ui.label(format!("{label}: {values}")); }"#
        )
        .is_empty()
    );
    assert_eq!(
        rules(
            "src/a.rs",
            r##"fn row() { ui.label(format!(
        r#"{hp} / {max}"#)); }"##
        ),
        ["hp-fraction-owner"]
    );
    // `fact-owner` (M5, review of PR 470): a local copy of the fact part is found by what it
    // does, under any name, in any file; the part itself is its owner.
    let copy = r#"fn sheet_label2(ui: &mut Ui, label: &str, values: &[&str]) {
        let values = values.join(" · ");
        ui.label(format!("{label}: {values}"));
    }"#;
    for path in ["src/party.rs", "src/a.rs"] {
        assert_eq!(
            rules(path, copy),
            ["fact-owner"],
            "a renamed helper in {path}"
        );
    }
    assert!(
        rules("src/design/fact.rs", copy).is_empty(),
        "the part owns it"
    );
    assert_eq!(
        rules(
            "src/party.rs",
            r#"fn senses_line() { ui.label(format!("{}: {}", "Senses", senses.join(" · "))); }"#
        ),
        ["fact-owner"],
        "the join given as the value"
    );
    // A chain of `let` bindings carries the join, given or read inline, also when a name is
    // bound again to its own copy (review of PR 470).
    for code in [
        r#"fn senses() {
            let joined = senses.join(" · ");
            let shown = joined.clone();
            let text = &shown;
            ui.label(format!("{}: {}", "Senses", text));
        }"#,
        r#"fn senses() {
            let joined = senses.join(" · ");
            let joined = joined.to_owned();
            let words = (joined).to_string();
            ui.label(format!("{label}: {words}"));
        }"#,
        r#"fn senses() {
            let joined = values.join(" · ");
            let shown: String = joined.clone();
            ui.label(format!("{label}: {shown}"));
        }"#,
    ] {
        assert_eq!(rules("src/a.rs", code), ["fact-owner"], "{code}");
    }
    // A tile label with no colon, raw or not, counted once at its format site; a rolling tile
    // by hand.
    assert_eq!(
        rules(
            "src/party.rs",
            r##"fn overview() {
                let label = format!(r#"{}{} {}"#, a, b, c);
                match bonus {
                    Some(bonus) => roll_metric(ui, &label, &value, &link),
                    None => metric(ui, &label, value),
                }
            }"##
        ),
        ["fact-owner"]
    );
    assert_eq!(
        rules(
            "src/a.rs",
            r#"fn tiles() { design::stat_tile(ui, &format!("{} {}", a, b), "15"); }"#
        ),
        ["fact-owner"]
    );
    assert_eq!(
        rules(
            "src/a.rs",
            r#"fn tiles() {
                let label = format!("{} {}", a, b);
                let shown = label.clone();
                let text = shown.as_str();
                stat_tile(ui, text, "15");
                metric(ui, &label, "15");
            }"#
        ),
        ["fact-owner"],
        "a chain to a tile's label, one format site"
    );
    assert_eq!(
        rules(
            "src/a.rs",
            r#"fn tiles() {
                let label: String = format!("{} {}", a, b);
                let shown: &str = label.as_str();
                stat_tile(ui, shown, "15");
            }"#
        ),
        ["fact-owner"],
        "typed bindings to a tile's label"
    );
    assert_eq!(
        rules(
            "src/a.rs",
            r#"fn dex() { crate::rolls::link(ui, value, Type::NumeralLarge, &link); }"#
        ),
        ["fact-owner"]
    );
    // Not a fact: a join with no colon form, a name bound again to another value (its count),
    // a status word beside a tile, a name a loop binds again, the part itself.
    for code in [
        r#"fn row() { let s = parts.join(" · "); ui.label(format!("{} rows", s)); }"#,
        r#"fn row() {
            let s = parts.join(" · ");
            let s = s.len().to_string();
            ui.label(format!("{label}: {s}"));
        }"#,
        r#"fn reference() { stat_tile(ui, "AC", &ac); let icon = format!("{} {word}", a); }"#,
        r#"fn passive() {
            let label = format!("{} {}", a, b);
            for (label, key) in pairs { metric(ui, label, value(key)); }
        }"#,
        r#"fn facts(ui: &mut Ui) { design::fact::line(ui, "Senses", senses); }"#,
    ] {
        assert!(rules("src/a.rs", code).is_empty(), "{code}");
    }
}
