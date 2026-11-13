//! Every suppression of `clippy::disallowed_methods` in the source, by the item it is on (tests
//! only). `STAGED_ALLOWS` lists the ones that may stay; any other fails.
//!
//! A suppression is an `allow` or `expect` (also inside `cfg_attr`) of `clippy::disallowed_methods`
//! or of a group that holds it (`clippy::style`, `clippy::all`, `warnings`), with any reason or
//! none. It may sit on one function or one statement (a `let` with a closure, a statement in a
//! macro). It may not sit on a file, a module, an `impl`, a trait or a macro, or on a function
//! that holds a nested function: those would free every banned call below them.

/// What a suppression is for. Package O's bans: `ResultOrder::issue`, `Footer::result` and
/// `save_row_state`. Package K's: the raw egui calls in `clippy.toml` that a layout part
/// replaces (`Ban::Layout`). Package P's: `Footer::show_by_hand` and `Fills::because`
/// (`Ban::Pane`). A function that calls both kinds keeps one attribute and one row
/// that lists both. A row lists exactly the bans its item still calls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Ban {
    Results,
    Layout,
    /// Package P: `Footer::show_by_hand` (use `Footer::pane`) and the pane's one escape,
    /// `Fills::because`.
    Pane,
}

impl Ban {
    /// Whether `body` still holds a call of this ban.
    fn calls(self, body: &str) -> bool {
        match self {
            Self::Results => RESULTS_CALLS.iter().any(|call| body.contains(call)),
            Self::Layout => crate::design::lint::RAW_CALLS
                .iter()
                .any(|call| body.contains(call)),
            Self::Pane => crate::design::lint::PANE_CALLS
                .iter()
                .any(|call| body.contains(call)),
        }
    }
}

/// Text that one of `Ban::Results`'s calls leaves in a function that still needs its allow.
const RESULTS_CALLS: [&str; 3] = ["::issue", "save_row_state", ".result("];

/// Lint names whose suppression also suppresses `clippy::disallowed_methods`.
const COVERS: [&str; 4] = [
    "clippy::disallowed_methods",
    "clippy::style",
    "clippy::all",
    "warnings",
];

#[derive(Debug)]
pub(crate) struct Suppression {
    pub line: usize,
    /// The function's name, or `<enclosing> statement` for a statement.
    pub item: String,
    /// The text it covers.
    pub body: String,
}

/// The code of `text` with string and char literals and comments blanked, kept to the same
/// length and lines, so brackets in them do not count.
pub(crate) fn code_only(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    let blank = |out: &mut String, c: char| out.push(if c == '\n' { '\n' } else { ' ' });
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        if c == '/' && next == Some('/') {
            while i < chars.len() && chars[i] != '\n' {
                blank(&mut out, chars[i]);
                i += 1;
            }
        } else if c == '/' && next == Some('*') {
            let mut depth = 0;
            while i < chars.len() {
                if chars[i] == '/' && chars.get(i + 1) == Some(&'*') {
                    depth += 1;
                    blank(&mut out, '/');
                    blank(&mut out, '*');
                    i += 2;
                } else if chars[i] == '*' && chars.get(i + 1) == Some(&'/') {
                    depth -= 1;
                    blank(&mut out, '*');
                    blank(&mut out, '/');
                    i += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    blank(&mut out, chars[i]);
                    i += 1;
                }
            }
        } else if c == 'r'
            && matches!(next, Some('"') | Some('#'))
            && (i == 0 || !(chars[i - 1].is_alphanumeric() || chars[i - 1] == '_'))
        {
            let mut j = i + 1;
            let mut hashes = 0;
            while chars.get(j) == Some(&'#') {
                hashes += 1;
                j += 1;
            }
            if chars.get(j) != Some(&'"') {
                out.push(c);
                i += 1;
                continue;
            }
            j += 1;
            while j < chars.len() {
                if chars[j] == '"' && (0..hashes).all(|h| chars.get(j + 1 + h) == Some(&'#')) {
                    j += 1 + hashes;
                    break;
                }
                j += 1;
            }
            for &skipped in &chars[i..j.min(chars.len())] {
                blank(&mut out, skipped);
            }
            i = j;
        } else if c == '"' {
            blank(&mut out, c);
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' {
                    blank(&mut out, chars[i]);
                    i += 1;
                }
                if i < chars.len() {
                    blank(&mut out, chars[i]);
                    i += 1;
                }
            }
            if i < chars.len() {
                blank(&mut out, '"');
                i += 1;
            }
        } else if c == '\'' {
            // A char literal ('{', '\n', '\u{7b}'); otherwise a lifetime.
            let close = if next == Some('\\') {
                (i + 2..chars.len().min(i + 12)).find(|&j| chars[j] == '\'')
            } else {
                (chars.get(i + 2) == Some(&'\'')).then_some(i + 2)
            };
            match close {
                Some(end) => {
                    for &skipped in &chars[i..=end] {
                        blank(&mut out, skipped);
                    }
                    i = end + 1;
                }
                None => {
                    out.push(c);
                    i += 1;
                }
            }
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

/// The name of the function a line starts, if it starts one.
fn fn_name(line: &str) -> Option<&str> {
    let at = line.find("fn ")?;
    let before = &line[..at];
    let allowed = |word: &str| {
        word.starts_with("pub")
            || word.ends_with(')')
            || ["const", "async", "unsafe", "extern", "\"C\"", "default"].contains(&word)
    };
    if !before.split_whitespace().all(allowed) {
        return None;
    }
    let rest = &line[at + 3..];
    let end = rest
        .find(|c: char| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(rest.len());
    (end > 0).then(|| &rest[..end])
}

/// The line where the bracket opened on `from` (or after it) closes.
fn close_of(code: &[&str], from: usize, open: char, close: char) -> usize {
    let mut depth = 0i32;
    let mut seen = false;
    for (at, line) in code.iter().enumerate().skip(from) {
        for c in line.chars() {
            if c == open {
                depth += 1;
                seen = true;
            } else if c == close {
                depth -= 1;
            }
        }
        if seen && depth <= 0 {
            return at;
        }
    }
    code.len().saturating_sub(1)
}

/// Every suppression of `clippy::disallowed_methods` in one file, and the faults of the ones on
/// a place they may not be.
pub(crate) fn find(path: &str, text: &str) -> (Vec<Suppression>, Vec<String>) {
    let code = code_only(text);
    let code: Vec<&str> = code.lines().collect();
    let raw: Vec<&str> = text.lines().collect();
    let (mut found, mut faults) = (Vec::new(), Vec::new());
    for at in 0..code.len() {
        let line = code[at].trim_start();
        if !(line.starts_with("#[") || line.starts_with("#![")) {
            continue;
        }
        let end = close_of(&code, at, '[', ']');
        let attribute = code[at..=end].join(" ");
        let words: Vec<&str> = attribute
            .split(|c: char| !(c.is_alphanumeric() || c == '_' || c == ':'))
            .filter(|word| !word.is_empty())
            .map(|word| word.trim_matches(':'))
            .collect();
        let suppresses = words.iter().any(|w| *w == "allow" || *w == "expect");
        if !(suppresses && words.iter().any(|w| COVERS.contains(w))) {
            continue;
        }
        let place = format!("{path}:{}", at + 1);
        if line.starts_with("#![") {
            faults.push(format!(
                "{place}: a suppression of `clippy::disallowed_methods` for a whole file or module. Put it on the one function that calls."
            ));
            continue;
        }
        // The item: the first line after other attributes, blank lines and comments.
        let mut target = end + 1;
        while target < code.len() {
            let next = code[target].trim();
            if next.is_empty() {
                target += 1;
            } else if next.starts_with("#[") {
                target = close_of(&code, target, '[', ']') + 1;
            } else {
                break;
            }
        }
        let Some(item_line) = code.get(target).map(|l| l.trim()) else {
            continue;
        };
        let first = item_line
            .split(|c: char| c.is_whitespace() || c == '<' || c == '!')
            .find(|w| !(w.starts_with("pub") || *w == "unsafe"))
            .unwrap_or_default();
        if let Some(name) = fn_name(item_line) {
            let close = close_of(&code, target, '{', '}');
            if let Some(nested) = code[target + 1..close]
                .iter()
                .find_map(|l| fn_name(l.trim()))
            {
                faults.push(format!(
                    "{place}: a suppression on `fn {name}`, which holds `fn {nested}`: it frees the nested function too. Put it on the innermost function that calls."
                ));
            }
            found.push(Suppression {
                line: at + 1,
                item: name.to_owned(),
                body: raw[target..=close.min(raw.len() - 1)].join("\n"),
            });
        } else if ["mod", "impl", "trait", "macro_rules"].contains(&first) {
            faults.push(format!(
                "{place}: a suppression on `{item_line}`. Put it on the one function that calls."
            ));
        } else {
            // A statement: named by the function or macro around it.
            let indent = code[target].len() - code[target].trim_start().len();
            let enclosing = (0..target)
                .rev()
                .filter(|&i| code[i].len() - code[i].trim_start().len() < indent)
                .find_map(|i| {
                    let l = code[i].trim();
                    fn_name(l).map(str::to_owned).or_else(|| {
                        l.strip_prefix("macro_rules!")
                            .map(|m| format!("{}!", m.trim().trim_end_matches('{').trim()))
                    })
                })
                .unwrap_or_else(|| "file".to_owned());
            let close = (target..code.len())
                .find(|&i| code[i].trim_end().ends_with(';'))
                .unwrap_or(target);
            found.push(Suppression {
                line: at + 1,
                item: format!("{enclosing} statement"),
                body: raw[target..=close].join("\n"),
            });
        }
    }
    (found, faults)
}

/// Every suppression in `files` against `table`: one row per suppression (file, item, bans).
pub(crate) fn check(files: &[(String, String)], table: &[(&str, &str, &[Ban])]) -> Vec<String> {
    let mut faults = Vec::new();
    let mut rows: Vec<(&str, &str, &[Ban])> = table.to_vec();
    for (path, text) in files {
        let (found, wrong) = find(path, text);
        faults.extend(wrong);
        for suppression in found {
            let row = rows
                .iter()
                .position(|(file, item, _)| file == path && *item == suppression.item);
            match row {
                Some(row) => {
                    let (_, _, bans) = rows.remove(row);
                    for ban in [Ban::Results, Ban::Layout] {
                        let listed = bans.contains(&ban);
                        let calls = ban.calls(&suppression.body);
                        if listed && !calls {
                            faults.push(format!(
                                "{path}:{}: the suppression on `{}` has no staged call left. Remove it and its row in `design::results::STAGED_ALLOWS`.",
                                suppression.line, suppression.item
                            ));
                        } else if calls && !listed && ban == Ban::Layout {
                            // Only for K's names: a row of O's text names can match a method
                            // of another type, which Clippy does not ban.
                            faults.push(format!(
                                "{path}:{}: the row of `{}` does not list `Ban::{ban:?}`, which it calls. Add it.",
                                suppression.line, suppression.item
                            ));
                        }
                    }
                }
                None => faults.push(format!(
                    "{path}:{}: a suppression of `clippy::disallowed_methods` on `{}` that `design::results::STAGED_ALLOWS` does not list. Use `design::Results`, `Footer::kept`, `design::save_row` and the layout parts instead.",
                    suppression.line, suppression.item
                )),
            }
        }
    }
    for (file, item, _) in rows {
        faults.push(format!(
            "{file}: `{item}` has no suppression left. Remove its row from `design::results::STAGED_ALLOWS`."
        ));
    }
    faults
}

#[cfg(test)]
mod tests {
    use super::*;

    const STAGED: &[Ban] = &[Ban::Results];

    fn file(text: &str) -> Vec<(String, String)> {
        vec![("src/new.rs".to_owned(), text.to_owned())]
    }

    /// Review of PR 301: every suppression counts, whatever its reason or form, and a parent item
    /// is refused even when the table lists it.
    #[test]
    fn every_suppression_is_counted_by_the_item_it_is_on() {
        let call = "    let _ = <crate::design::ResultOrder>::issue();\n";
        // An unreasoned allow on a function in a new file.
        let plain = format!("#[allow(clippy::disallowed_methods)]\nfn order() {{\n{call}}}\n");
        assert_eq!(check(&file(&plain), &[]).len(), 1, "{plain}");
        assert!(check(&file(&plain), &[("src/new.rs", "order", STAGED)]).is_empty());
        // `expect`, `cfg_attr` and a group that holds the lint.
        for attribute in [
            "#[expect(clippy::disallowed_methods)]",
            "#[cfg_attr(test, allow(clippy::disallowed_methods))]",
            "#[allow(clippy::style)]",
            "#[allow(warnings)]",
        ] {
            let text = format!("{attribute}\nfn order() {{\n{call}}}\n");
            assert_eq!(check(&file(&text), &[]).len(), 1, "{text}");
        }
        // On a closure's statement.
        let closure = format!(
            "fn draw() {{\n    #[allow(clippy::disallowed_methods)]\n    let order = || {{\n    {call}    }};\n}}\n"
        );
        let faults = check(&file(&closure), &[]);
        assert!(
            faults.len() == 1 && faults[0].contains("`draw statement`"),
            "{faults:?}"
        );
        // On a nested function.
        let nested = format!(
            "fn draw() {{\n    #[allow(clippy::disallowed_methods)]\n    fn order() {{\n    {call}    }}\n}}\n"
        );
        let faults = check(&file(&nested), &[]);
        assert!(
            faults.len() == 1 && faults[0].contains("`order`"),
            "{faults:?}"
        );
        // On a parent item: refused even when listed.
        let parent = format!(
            "#[allow(clippy::disallowed_methods)]\nfn draw() {{\n    fn order() {{\n    {call}    }}\n}}\n"
        );
        let faults = check(&file(&parent), &[("src/new.rs", "draw", STAGED)]);
        assert!(
            faults.len() == 1 && faults[0].contains("holds `fn order`"),
            "{faults:?}"
        );
        // On a file, a module and an `impl`.
        for wide in [
            "#![allow(clippy::disallowed_methods)]\n",
            "#[allow(clippy::disallowed_methods)]\nmod draw {}\n",
            "#[allow(clippy::disallowed_methods)]\nimpl Draw {}\n",
        ] {
            assert_eq!(check(&file(wide), &[]).len(), 1, "{wide}");
        }
        // A listed suppression whose function no longer calls.
        let stale = "#[allow(clippy::disallowed_methods)]\nfn order() {\n    let _ = 1;\n}\n";
        let faults = check(&file(stale), &[("src/new.rs", "order", STAGED)]);
        assert!(
            faults.len() == 1 && faults[0].contains("no staged call"),
            "{faults:?}"
        );
        // A row with no suppression left.
        assert_eq!(
            check(&file("fn order() {}\n"), &[("src/new.rs", "order", STAGED)]).len(),
            1
        );
        // Brackets in strings and comments do not end the attribute or the body.
        let quoted = format!(
            "#[allow(clippy::disallowed_methods, reason = \"staged: ]\")]\nfn order() {{\n    let _ = \"}}\"; // }}\n{call}}}\nfn other() {{}}\n"
        );
        assert!(check(&file(&quoted), &[("src/new.rs", "order", STAGED)]).is_empty());
    }

    /// Package K's rows: a row lists exactly the bans its item calls, and an allow in a macro's
    /// body, or on the macro, is seen.
    #[test]
    fn layout_rows_list_what_their_item_calls() {
        const LAYOUT: &[Ban] = &[Ban::Layout];
        const BOTH: &[Ban] = &[Ban::Results, Ban::Layout];
        let raw = "    let _ = egui::ScrollArea::vertical();\n";
        let order = "    let _ = <crate::design::ResultOrder>::issue();\n";
        let draw = format!("#[allow(clippy::disallowed_methods)]\nfn draw() {{\n{raw}}}\n");
        assert!(check(&file(&draw), &[("src/new.rs", "draw", LAYOUT)]).is_empty());
        // Listed for O's ban only, while it makes a raw egui call: the row must name K's too.
        let faults = check(&file(&draw), &[("src/new.rs", "draw", STAGED)]);
        assert!(
            faults
                .iter()
                .any(|fault| fault.contains("does not list `Ban::Layout`")),
            "{faults:?}"
        );
        // Both kinds in one function: one attribute, one row with both.
        let both = format!("#[allow(clippy::disallowed_methods)]\nfn draw() {{\n{raw}{order}}}\n");
        assert!(check(&file(&both), &[("src/new.rs", "draw", BOTH)]).is_empty());
        // A row for a ban the item no longer calls.
        let plain = "#[allow(clippy::disallowed_methods)]\nfn draw() {\n    let _ = 1;\n}\n";
        assert_eq!(
            check(&file(plain), &[("src/new.rs", "draw", LAYOUT)]).len(),
            1
        );
        // In a macro's body: a function the macro makes is unlisted; on the macro, refused.
        let in_macro = format!(
            "macro_rules! screens {{\n    () => {{\n        #[allow(clippy::disallowed_methods)]\n        fn draw() {{\n{raw}        }}\n    }};\n}}\n"
        );
        assert_eq!(check(&file(&in_macro), &[]).len(), 1, "{in_macro}");
        let on_macro =
            "#[allow(clippy::disallowed_methods)]\nmacro_rules! screens {\n    () => {};\n}\n";
        assert_eq!(check(&file(on_macro), &[]).len(), 1, "{on_macro}");
    }
}
