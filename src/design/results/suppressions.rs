//! Every suppression of `clippy::disallowed_methods` in the source, by the item it is on (tests
//! only). `STAGED_ALLOWS` lists the ones that may stay; any other fails.
//!
//! A suppression is an `allow` or `expect` (also inside `cfg_attr`) of `clippy::disallowed_methods`
//! or of a group that holds it (`clippy::style`, `clippy::all`, `warnings`), with any reason or
//! none. It may sit on one function or one statement (a `let` with a closure, a statement in a
//! macro). It may not sit on a file, a module, an `impl`, a trait or a macro, or on a function
//! that holds a nested function: those would free every banned call below them.

/// What a suppression is for. Package O's bans: `ResultOrder::issue`, `Footer::result` and
/// `save_row_state`. Package K (PR 304) adds its own variant for its egui bans when it merges
/// after O; a function that calls both kinds keeps one attribute and one row that lists both.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Ban {
    Results,
    /// Package M: `Sentence::outside`, the named escape for words from outside the app. Its rows
    /// need not reach 0, but a new one is a visible change of this table.
    Words,
    /// Package M: the helpers of `sentence!` and `choice!` (only the macros and `outcome.rs` use
    /// them).
    Sentence,
    /// Package M: `Footer::outcome_at`, a result with a hand-issued order (staged).
    Order,
    /// Ruling Q-x: `Messages::answer`, the answer to the DM's last act that leads the footer. The
    /// part cannot prove that a fault is the newest answer to an act, so each use is a counted
    /// review decision.
    Answer,
    /// Package M: the body places `banner`, `row_fault`, `PagePlaceholder::show`, `BodyFault::show`
    /// and `RowMessages::new` (ruling R24). A named escape: a body message is a review decision.
    Body,
    /// Package K: raw egui calls that a layout part replaces. The number is how many calls of
    /// `clippy.toml`'s package K paths the item holds, counted in its tokens
    /// (`design::layout::bans::count`), so a second raw call in an allowed item fails.
    Layout(u16),
    /// Package K: `RowIndex::from_data` and `TextKey::from_data`, row keys a reviewer reads; the
    /// number of uses in the item.
    RowKey(u16),
    /// Package X: raw space, raw selectable choices, and the headings that do not
    /// add their own space (`clippy.toml`, section "Package X"). The number is how many of those
    /// calls the item holds, counted in its tokens as `Layout` is.
    Extras(u16),
    /// Package P: `Fills::because`
    /// (the pane's one named escape), and the room-centred layouts; the number of calls in the
    /// item, counted in its tokens as `Layout` is.
    Pane(u16),
    /// Package M: `Results::undo` and `Results::use_press`, a footer press by owner instead of by
    /// its ticket; the number of calls in the item, counted in its tokens as `Layout` is, also in
    /// an item allowed for another ban.
    ByOwner(u16),
    /// Layout PR 2b: a footer line lowered to a closure (`Footer::legacy_reason`,
    /// `legacy_notice`, `legacy_kept`, `legacy_kept_line`): the legacy footer's. A typed footer
    /// (`outcome::Messages`) holds data. Staged out; the calls are also counted per file
    /// (`STAGED`).
    Footer,
}

impl Ban {
    /// The bans whose call text names nothing else, so a call of one in an item whose row does
    /// not list it is refused even when the row lists another ban. (`Results`, `Words` and
    /// `Body` match ordinary calls too: `::issue_count_title`, `outside_after`, `.show(`; they
    /// need a token-exact match first.)
    pub(crate) const EXACT: [Ban; 4] = [Ban::Sentence, Ban::Order, Ban::Answer, Ban::Footer];

    /// The bans an allowed item may call exactly once (`Messages::answer`: ruling Q-x's one
    /// answer to the DM's last act, one call site).
    pub(crate) const ONCE: [Ban; 1] = [Ban::Answer];

    /// Text that one of this ban's calls leaves in a function that still needs its allow.
    fn calls(self) -> &'static [&'static str] {
        match self {
            Self::Results => &["::issue", "save_row_state", ".result("],
            Self::Words => &["outside"],
            Self::Sentence => &[
                "__formatted",
                "__literal",
                "check_template",
                "check_literal",
                "check_choice",
                "__template",
                // `lead!`'s check (`Lead::__checked`).
                "__checked",
            ],
            Self::Order => &["outcome_at"],
            // Every call form (`exact_calls` reads them as tokens: a method, an associated call,
            // a reference).
            Self::Footer => &FOOTER_LINES,
            Self::Answer => &[".answer(", "Messages::answer"],
            Self::Body => &["banner", "row_fault", ".show(", "RowMessages::new"],
            // Counted in the item's tokens instead (`check`).
            Self::Layout(_)
            | Self::RowKey(_)
            | Self::Extras(_)
            | Self::Pane(_)
            | Self::ByOwner(_) => &[],
        }
    }
}

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

/// Every suppression of `clippy::disallowed_methods` in one file, and the faults of the ones on
/// a place they may not be. The attributes come from the shared `syn` reader
/// (`design::attributes`), so a list across lines, `cfg_attr` and an attribute inside a macro's
/// tokens all count.
pub(crate) fn find(path: &str, text: &str) -> (Vec<Suppression>, Vec<String>) {
    use crate::design::attributes::{Kind, read};
    let raw: Vec<&str> = text.lines().collect();
    let found = match read(text) {
        Ok(found) => found,
        Err(error) => return (Vec::new(), vec![format!("{path}: does not parse: {error}")]),
    };
    let (mut kept, mut faults) = (Vec::new(), Vec::new());
    for attribute in found.iter().filter(|found| found.lets_pass(&COVERS)) {
        let place = format!("{path}:{}", attribute.line);
        if attribute.inner {
            faults.push(format!(
                "{place}: a suppression of `clippy::disallowed_methods` for a whole file or module. Put it on the one function that calls."
            ));
            continue;
        }
        let (first, last) = attribute.lines;
        let body = raw[first.saturating_sub(1)..last.min(raw.len())].join("\n");
        let item = match &attribute.kind {
            Kind::Fn { nested } => {
                if let Some(nested) = nested {
                    faults.push(format!(
                        "{place}: a suppression on `fn {}`, which holds `fn {nested}`: it frees the nested function too. Put it on the innermost function that calls.",
                        attribute.item
                    ));
                }
                attribute.item.clone()
            }
            Kind::Mod { .. } | Kind::Impl | Kind::Trait | Kind::MacroRules => {
                faults.push(format!(
                    "{place}: a suppression on `{}`. Put it on the one function that calls.",
                    attribute.item
                ));
                continue;
            }
            Kind::Statement => attribute.item.clone(),
            Kind::File | Kind::Other => "file statement".to_owned(),
        };
        kept.push(Suppression {
            line: attribute.line,
            item,
            body,
        });
    }
    (kept, faults)
}

/// The one item that may call `RowMessages::new` (ruling R24), once.
const ROW_MESSAGES_USE: (&str, &str) = ("src/sessions/page.rs", "folder_block");

/// How many times `body` calls `RowMessages::new`, also with generic arguments
/// (`RowMessages::<A>::new`) and in a qualified type (`<RowMessages<A>>::new`); strings and
/// comments do not count. A text scan by design: not syntax-aware.
fn row_messages_calls(body: &str) -> usize {
    let code = code_only(body);
    let bytes = code.as_bytes();
    let mut found = 0;
    let mut from = 0;
    while let Some(at) = code[from..].find("RowMessages") {
        let mut i = from + at + "RowMessages".len();
        from = i;
        let skip = |i: &mut usize| {
            while *i < bytes.len() && bytes[*i].is_ascii_whitespace() {
                *i += 1;
            }
        };
        skip(&mut i);
        // Generic arguments, with a turbofish (`RowMessages::<A>`) or in a qualified type
        // (`<RowMessages<A>>`).
        let mut j = i;
        if bytes[j..].starts_with(b"::") {
            j += 2;
            skip(&mut j);
        }
        if bytes.get(j) == Some(&b'<') {
            let mut depth = 0;
            while j < bytes.len() {
                match bytes[j] {
                    b'<' => depth += 1,
                    b'>' => {
                        depth -= 1;
                        if depth == 0 {
                            j += 1;
                            break;
                        }
                    }
                    _ => {}
                }
                j += 1;
            }
            i = j;
            skip(&mut i);
        }
        // The end of a qualified type (`<RowMessages<A>>::new`, `<RowMessages>::new`).
        if bytes.get(i) == Some(&b'>') {
            i += 1;
            skip(&mut i);
        }
        if !bytes[i..].starts_with(b"::") {
            continue;
        }
        let mut k = i + 2;
        skip(&mut k);
        let rest = &bytes[k..];
        if rest.starts_with(b"new")
            && !rest
                .get(3)
                .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_')
        {
            found += 1;
        }
    }
    found
}

/// Every suppression in `files` against `table`: one row per suppression (file, item, bans).
/// How many calls of an exact-text ban `body` holds, with strings and comments blanked. A
/// `Messages::answer` is counted called or referenced (`let f = Messages::answer;`), and not
/// when an identifier character follows (`Messages::answer_all`). A text scan, as
/// `row_messages_calls`: not syntax-aware (a token-aware checker is a known limit).
/// The legacy footer's closure lines (`Ban::Footer`, layout PR 2b, ruling L13).
pub(crate) const FOOTER_LINES: [&str; 4] = [
    "legacy_reason",
    "legacy_notice",
    "legacy_kept",
    "legacy_kept_line",
];

/// Each use of a legacy footer line in `source`, in any call form: a method (`f.legacy_notice(…)`),
/// an associated call (`Footer::legacy_notice(f, …)`), a reference (`let n = Footer::legacy_notice;`).
/// Read as tokens by the L0 freeze's machinery (`layout::bans::calls`, `Shape::Name`), so words in
/// strings and comments, `use` declarations and the definitions do not count.
pub(crate) fn footer_line_uses(source: &str) -> usize {
    let shapes: Vec<crate::design::layout::bans::Shape> = FOOTER_LINES
        .iter()
        .map(|name| crate::design::layout::bans::Shape::Name((*name).to_owned()))
        .collect();
    crate::design::layout::bans::calls(source, &shapes).map_or_else(
        // A source that does not lex: count every word, so nothing hides.
        |_| {
            let code = code_only(source);
            FOOTER_LINES
                .iter()
                .map(|name| code.matches(name).count())
                .sum()
        },
        |found| found.len(),
    )
}

fn exact_calls(ban: Ban, body: &str) -> usize {
    if ban == Ban::Footer {
        return footer_line_uses(body);
    }
    let code = code_only(body);
    ban.calls()
        .iter()
        .map(|call| {
            code.match_indices(call)
                .filter(|(at, _)| {
                    !call.ends_with(|c: char| c.is_alphanumeric() || c == '_')
                        || !code[at + call.len()..]
                            .starts_with(|c: char| c.is_alphanumeric() || c == '_')
                })
                .count()
        })
        .sum()
}

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
                    let mut holds = bans
                        .iter()
                        .any(|ban| ban.calls().iter().any(|call| suppression.body.contains(call)));
                    // Packages K and X: the row's number of calls is the item's, exactly.
                    use crate::design::layout::bans::{Group, count};
                    for group in [
                        Group::Layout,
                        Group::RowKey,
                        Group::Extras,
                        Group::Pane,
                        Group::ByOwner,
                    ] {
                        let listed = bans
                            .iter()
                            .find_map(|ban| match (ban, group) {
                                (Ban::Layout(n), Group::Layout)
                                | (Ban::RowKey(n), Group::RowKey)
                                | (Ban::Extras(n), Group::Extras)
                                | (Ban::Pane(n), Group::Pane)
                                | (Ban::ByOwner(n), Group::ByOwner) => Some(usize::from(*n)),
                                _ => None,
                            })
                            .unwrap_or(0);
                        match count(&suppression.body, group) {
                            Ok(found) => {
                                holds |= found > 0;
                                if found != listed {
                                    faults.push(format!(
                                        "{path}:{}: `{}` holds {found} {group:?} calls; its row lists {listed}. {}",
                                        suppression.line,
                                        suppression.item,
                                        if found > listed {
                                            "A new raw call: use the part that clippy.toml names."
                                        } else {
                                            "Lower the row's number."
                                        }
                                    ));
                                }
                            }
                            Err(error) => faults.push(format!(
                                "{path}:{}: `{}` does not lex: {error}",
                                suppression.line, suppression.item
                            )),
                        }
                    }
                    // A call of a ban that the row does not list is not allowed by the row's
                    // other bans (a `Ban::Order` row does not allow `answer`).
                    for ban in Ban::EXACT {
                        if !bans.contains(&ban) && exact_calls(ban, &suppression.body) > 0 {
                            faults.push(format!(
                                "{path}:{}: `{}` calls {ban:?} but its row in `design::results::STAGED_ALLOWS` does not list it.",
                                suppression.line, suppression.item
                            ));
                        }
                    }
                    for ban in Ban::ONCE {
                        let calls = exact_calls(ban, &suppression.body);
                        if bans.contains(&ban) && calls != 1 {
                            faults.push(format!(
                                "{path}:{}: `{}` holds {calls} {ban:?} calls; its row allows exactly one.",
                                suppression.line, suppression.item
                            ));
                        }
                    }
                    // R24: `RowMessages::new` is counted on its own, in every allowed item,
                    // whatever bans its row lists: exactly one call, in `Sessions::folder_block`.
                    let made = row_messages_calls(&suppression.body);
                    let allowed = usize::from(
                        (path.as_str(), suppression.item.as_str()) == ROW_MESSAGES_USE,
                    );
                    holds |= made > 0;
                    if made != allowed {
                        faults.push(format!(
                            "{path}:{}: `{}` holds {made} `RowMessages::new` calls; R24 allows {allowed} here (one, in `{}` `{}`).",
                            suppression.line, suppression.item, ROW_MESSAGES_USE.0, ROW_MESSAGES_USE.1
                        ));
                    }
                    if !holds {
                        faults.push(format!(
                            "{path}:{}: the suppression on `{}` has no staged call left. Remove it and its row in `design::results::STAGED_ALLOWS`.",
                            suppression.line, suppression.item
                        ));
                    }
                }
                None => faults.push(format!(
                    "{path}:{}: a suppression of `clippy::disallowed_methods` on `{}` that `design::results::STAGED_ALLOWS` does not list. Use `design::Results`, `Footer::legacy_kept` and `design::save_row` instead.",
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
        // Package M's bans: a listed suppression whose function still calls its escape passes;
        // a list across lines is read whole.
        let order = "#[allow(\n    clippy::disallowed_methods,\n    reason = \"staged\"\n)]\nfn draw() {\n    footer.outcome_at(order, done, &mut pressed);\n}\n";
        assert_eq!(check(&file(order), &[]).len(), 1, "{order}");
        assert!(check(&file(order), &[("src/new.rs", "draw", &[Ban::Order])]).is_empty());
        // Ruling Q-x: an `answer` call needs `Ban::Answer` in its own row; a row for another
        // ban (a `Ban::Order` row) does not allow it.
        let answer = "#[allow(clippy::disallowed_methods)]\nfn draw() {\n    footer.outcome_at(order, done, &mut pressed);\n    messages.answer(fault);\n}\n";
        assert_eq!(
            check(&file(answer), &[("src/new.rs", "draw", &[Ban::Order])]).len(),
            1,
            "{answer}"
        );
        assert!(
            check(
                &file(answer),
                &[("src/new.rs", "draw", &[Ban::Order, Ban::Answer])]
            )
            .is_empty()
        );
        // Exactly one `answer` call in the item allowed for it: a second call fails, also a
        // qualified one or a reference; a qualified call under another ban's allowance fails.
        let twice = "#[allow(clippy::disallowed_methods)]\nfn draw() {\n    messages.answer(fault);\n    messages.answer(other);\n}\n";
        assert_eq!(
            check(&file(twice), &[("src/new.rs", "draw", &[Ban::Answer])]).len(),
            1,
            "{twice}"
        );
        let qualified = "#[allow(clippy::disallowed_methods)]\nfn draw() {\n    messages.answer(fault);\n    Messages::answer(&mut messages, other);\n}\n";
        assert_eq!(
            check(&file(qualified), &[("src/new.rs", "draw", &[Ban::Answer])]).len(),
            1,
            "{qualified}"
        );
        let referenced = "#[allow(clippy::disallowed_methods)]\nfn draw() {\n    messages.answer(fault);\n    let again = Messages::answer;\n}\n";
        assert_eq!(
            check(&file(referenced), &[("src/new.rs", "draw", &[Ban::Answer])]).len(),
            1,
            "{referenced}"
        );
        let other = "#[allow(clippy::disallowed_methods)]\nfn draw() {\n    footer.outcome_at(order, done, &mut pressed);\n    Messages::answer(&mut messages, fault);\n}\n";
        assert_eq!(
            check(&file(other), &[("src/new.rs", "draw", &[Ban::Order])]).len(),
            1,
            "{other}"
        );
        // Layout PR 2b, review of PR 452 (ruling L13): a legacy footer line is found in every
        // call form, also in an item allowed for another ban: a method, an associated call and a
        // reference. A row that lists `Ban::Footer` allows them.
        for footer in [
            "    footer.legacy_notice(words, |_| {});\n",
            "    Footer::legacy_notice(footer, words, |_| {});\n",
            "    let notice = Footer::legacy_reason;\n",
            "    crate::design::Footer::legacy_kept_line(footer, line, true, |_, _| {});\n",
        ] {
            let item = format!(
                "#[allow(clippy::disallowed_methods)]\nfn page_error() {{\n    footer.result(order, words, true, |_| {{}});\n{footer}}}\n"
            );
            assert_eq!(
                check(
                    &file(&item),
                    &[("src/new.rs", "page_error", &[Ban::Results])]
                )
                .len(),
                1,
                "{item}"
            );
            assert!(
                check(
                    &file(&item),
                    &[("src/new.rs", "page_error", &[Ban::Results, Ban::Footer])]
                )
                .is_empty(),
                "{item}"
            );
        }
        let words = "#[allow(clippy::disallowed_methods)]\nfn cause() {\n    let f = Sentence::outside;\n}\n";
        assert!(check(&file(words), &[("src/new.rs", "cause", &[Ban::Words])]).is_empty());
        // Review of PR 308: a by-owner Undo added to an item already allowed for another ban
        // (its order issued by hand keeps the row) is counted; its row lists none.
        let by_owner = format!(
            "#[allow(clippy::disallowed_methods)]\nfn stamp_kept() {{\n{call}    store.undo(owner, |_| Ok::<_, ()>(()));\n}}\n"
        );
        let faults = check(&file(&by_owner), &[("src/new.rs", "stamp_kept", STAGED)]);
        assert!(
            faults.len() == 1 && faults[0].contains("holds 1 ByOwner calls; its row lists 0"),
            "{faults:?}"
        );
        assert!(
            check(
                &file(&by_owner),
                &[("src/new.rs", "stamp_kept", &[Ban::Results, Ban::ByOwner(1)])]
            )
            .is_empty()
        );
        // By its path, and `use_press`: two calls, a row of one fails.
        let both = "#[allow(clippy::disallowed_methods)]\nfn press() {\n    Results::undo(&mut store, owner, run);\n    store.use_press(owner);\n}\n";
        // Review 4: generic arguments before the call's arguments count too, on a method and on
        // a path: two calls.
        let generic = "#[allow(clippy::disallowed_methods)]\nfn press() {\n    store.undo::<(), ()>(owner, |_| Ok(()));\n    Results::<O, T>::undo(&mut store, owner, run);\n}\n";
        let faults = check(
            &file(generic),
            &[("src/new.rs", "press", &[Ban::ByOwner(1)])],
        );
        assert!(
            faults.len() == 1 && faults[0].contains("holds 2 ByOwner calls; its row lists 1"),
            "{faults:?}"
        );
        // The same reader serves the other counted bans: a raw egui call by a generic path.
        let raw = "#[allow(clippy::disallowed_methods)]\nfn draw() {\n    let _ = egui::ScrollArea::<>::vertical();\n}\n";
        assert!(check(&file(raw), &[("src/new.rs", "draw", &[Ban::Layout(1)])]).is_empty());
        let faults = check(&file(both), &[("src/new.rs", "press", &[Ban::ByOwner(1)])]);
        assert!(
            faults.len() == 1 && faults[0].contains("holds 2 ByOwner calls; its row lists 1"),
            "{faults:?}"
        );
    }

    /// Astra FINAL review of 83e0fd5f, P2 2: `RowMessages::new` is counted on its own. A second
    /// call in `folder_block` fails; a call in an item allowed only for another ban fails; the
    /// one call in `folder_block` passes, also with generic arguments.
    #[test]
    fn row_messages_new_has_exactly_one_use() {
        let page = |text: &str| vec![("src/sessions/page.rs".to_owned(), text.to_owned())];
        let block = |calls: &str| {
            format!("#[allow(clippy::disallowed_methods)]\nfn folder_block() {{\n{calls}}}\n")
        };
        let one = "    let m = RowMessages::new(&frame);\n";
        let rows: &[(&str, &str, &[Ban])] =
            &[("src/sessions/page.rs", "folder_block", &[Ban::Body])];
        assert!(check(&page(&block(one)), rows).is_empty());
        let generic = "    let m = RowMessages::<FolderAct>::new(&frame);\n";
        assert!(check(&page(&block(generic)), rows).is_empty());
        let faults = check(&page(&block(&format!("{one}{generic}"))), rows);
        assert!(
            faults.len() == 1 && faults[0].contains("holds 2 `RowMessages::new` calls"),
            "a second call in folder_block: {faults:?}"
        );
        // An item allowed only for another ban (here `Ban::Results`).
        let other = format!(
            "#[allow(clippy::disallowed_methods)]\nfn order() {{\n    let _ = <crate::design::ResultOrder>::issue();\n{one}}}\n"
        );
        let faults = check(&file(&other), &[("src/new.rs", "order", STAGED)]);
        assert!(
            faults.len() == 1
                && faults[0].contains("holds 1 `RowMessages::new` calls; R24 allows 0"),
            "a call under another ban's allowance: {faults:?}"
        );
        // The qualified-type spellings count too (re-check of da245d42, P2 2).
        let qualified = "    let m = <RowMessages<FolderAct>>::new(&frame);\n";
        let faults = check(&page(&block(&format!("{one}{qualified}"))), rows);
        assert!(
            faults.len() == 1 && faults[0].contains("holds 2 `RowMessages::new` calls"),
            "a qualified-type second call: {faults:?}"
        );
        assert_eq!(row_messages_calls("<RowMessages>::new(&frame)"), 1);
        // Words in strings and comments are not calls.
        let words = "    let _ = \"RowMessages::new\"; // RowMessages::new\n";
        assert_eq!(row_messages_calls(words), 0);
    }

    /// Package K's rows: the number of raw egui calls (and row keys) in a row is the item's,
    /// exactly, counted in its tokens; a second raw call in an allowed item fails.
    #[test]
    fn layout_rows_count_their_items_calls() {
        let raw = "    let _ = egui::ScrollArea::vertical();\n";
        let order = "    let _ = <crate::design::ResultOrder>::issue();\n";
        let item =
            |body: &str| format!("#[allow(clippy::disallowed_methods)]\nfn draw() {{\n{body}}}\n");
        let one = item(raw);
        assert!(check(&file(&one), &[("src/new.rs", "draw", &[Ban::Layout(1)])]).is_empty());
        // A second raw call in the allowed function.
        let two = item(&format!("{raw}{raw}"));
        let faults = check(&file(&two), &[("src/new.rs", "draw", &[Ban::Layout(1)])]);
        assert!(
            faults.len() == 1 && faults[0].contains("holds 2 Layout calls; its row lists 1"),
            "{faults:?}"
        );
        // Fewer calls than the row lists: lower it.
        let faults = check(&file(&one), &[("src/new.rs", "draw", &[Ban::Layout(2)])]);
        assert!(faults[0].contains("Lower the row's number"), "{faults:?}");
        // A row for O's ban only, while the item makes a raw egui call.
        let both = item(&format!("{raw}{order}"));
        let faults = check(&file(&both), &[("src/new.rs", "draw", STAGED)]);
        assert!(faults[0].contains("its row lists 0"), "{faults:?}");
        assert!(
            check(
                &file(&both),
                &[("src/new.rs", "draw", &[Ban::Results, Ban::Layout(1)])]
            )
            .is_empty()
        );
        // A word in a string is not a call; a row key is counted on its own.
        let words = item(
            "    let _ = \"egui::ScrollArea::vertical()\";\n    let _ = RowIndex::from_data(1);\n",
        );
        assert!(check(&file(&words), &[("src/new.rs", "draw", &[Ban::RowKey(1)])]).is_empty());
        // A function in a macro's body is unlisted; an allow on the macro is refused.
        let in_macro = format!(
            "macro_rules! screens {{\n    () => {{\n        #[allow(clippy::disallowed_methods)]\n        fn draw() {{\n{raw}        }}\n    }};\n}}\n"
        );
        assert_eq!(check(&file(&in_macro), &[]).len(), 1, "{in_macro}");
        let on_macro =
            "#[allow(clippy::disallowed_methods)]\nmacro_rules! screens {\n    () => {};\n}\n";
        assert_eq!(check(&file(on_macro), &[]).len(), 1, "{on_macro}");
        // Package X: a raw space counts as `Extras`, apart from K's calls; a second one fails.
        let space = "    ui.add_space(8.0);\n";
        let mixed = item(&format!("{raw}{space}"));
        assert!(
            check(
                &file(&mixed),
                &[("src/new.rs", "draw", &[Ban::Layout(1), Ban::Extras(1)])]
            )
            .is_empty()
        );
        let faults = check(&file(&mixed), &[("src/new.rs", "draw", &[Ban::Layout(1)])]);
        assert!(
            faults.len() == 1 && faults[0].contains("holds 1 Extras calls; its row lists 0"),
            "{faults:?}"
        );
        let two = item(&format!("{space}{space}"));
        let faults = check(&file(&two), &[("src/new.rs", "draw", &[Ban::Extras(1)])]);
        assert!(
            faults.len() == 1 && faults[0].contains("holds 2 Extras calls; its row lists 1"),
            "{faults:?}"
        );
    }
}
