//! G7, the plain-campaign check of `docs/design/session-cycle.md` section 5.3.
//!
//! The scenario `plain-evening` plays the frozen plain campaign (`Seed::Plain`) with today's
//! controls only. On each screen it records, at 1200x800 and 960x640 and with Lookup closed and
//! open, every named control with its rectangle and every painted text with its rectangle. It
//! also records every file of the campaigns at the start and at the end of the evening. It then
//! compares all of it with `plain_evening/reference.json`.
//!
//! Any difference fails, unless one row of `plain_evening/allowed.json` names exactly that
//! difference: the screen, the view, the control, text or file, and its rectangle or lines. A
//! row also names its row of section 2.5. A row that matches nothing fails too, so the list
//! stays exact. Nothing is masked.
//!
//! `audit-freeze <label>` sets `SPLAT_PLAIN_EVENING_WRITE=1`: the scenario then writes the
//! reference and `artifacts/dm-audit/titles.tsv` in place of the check. It refuses while
//! `allowed.json` has a row, because the rows describe changes from the reference.
use super::{
    driver::{Driver, Sizes},
    fixture::{PLAIN_LAST, PLAIN_PLAYED, PLAIN_RUN, PLAIN_TITLES, plain_reader},
    steps::*,
};
use eframe::egui::{self, Key};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

const WRITE: &str = "SPLAT_PLAIN_EVENING_WRITE";
const WIDE: (f32, f32) = (1200.0, 800.0);
const NARROW: (f32, f32) = (960.0, 640.0);
/// Section 2.5 numbers its rows 1 to 21.
const CONTRACT_ROWS: std::ops::RangeInclusive<u32> = 1..=21;

/// A control (`Role: name`) or a painted text, with its rectangle `[x, y, width, height]`.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Item(String, [f32; 4]);

/// One screen at one size, with Lookup closed or open.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct View {
    screen: String,
    view: String,
    controls: Vec<Item>,
    texts: Vec<Item>,
}

/// Each file of the campaigns: its lines, or for a file that is not text or is longer than
/// `WHOLE`, its hash and size. At the end, a file that the evening did not change is `SAME`.
type Files = BTreeMap<String, Vec<String>>;
const WHOLE: usize = 16 * 1024;
const SAME: &str = "the same as at the start";

#[derive(Serialize, Deserialize, Default)]
struct Reference {
    about: String,
    views: Vec<View>,
    files_at_start: Files,
    files_at_end: Files,
}

/// One difference from the reference. An allowed row holds one of these, whole.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Difference {
    screen: String,
    view: String,
    /// `control`, `text`, `file` or `view`.
    what: String,
    /// `new`, `gone`, `moved`, `height`, `width`, `resized` or `lines`.
    change: String,
    name: String,
    /// The rectangles, or the lines that differ.
    detail: String,
}

impl std::fmt::Display for Difference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} / {}: {} {} {:?}: {}",
            self.screen, self.view, self.what, self.change, self.name, self.detail
        )
    }
}

#[derive(Serialize, Deserialize)]
struct AllowedRow {
    /// The row of section 2.5 that permits this difference.
    row: u32,
    why: String,
    difference: Difference,
}

#[derive(Serialize, Deserialize)]
struct Allowed {
    about: String,
    rows: Vec<AllowedRow>,
}

fn folder() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/visual_tests/plain_evening")
}

fn tenth(value: f32) -> f32 {
    (value * 10.0).round() / 10.0
}

fn rect(r: egui::Rect) -> [f32; 4] {
    [
        tenth(r.min.x),
        tenth(r.min.y),
        tenth(r.width()),
        tenth(r.height()),
    ]
}

fn words(r: [f32; 4]) -> String {
    format!("{} {} {}x{}", r[0], r[1], r[2], r[3])
}

/// What a person can see now: each named control in view, and each painted text in view.
fn look(d: &Driver<'_>, screen: &str, view: String) -> View {
    let size = d.window_size();
    let window = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(size.0, size.1));
    let controls = d
        .nodes()
        .into_iter()
        .filter(|node| !node.name.trim().is_empty() && node.in_view())
        .map(|node| Item(format!("{:?}: {}", node.role, node.name), rect(node.rect)))
        .collect();
    let mut texts = Vec::new();
    d.each_painted_text(|text, clip| {
        let words = text.galley.job.text.clone();
        let at = text.galley.rect.translate(text.pos.to_vec2());
        if !words.trim().is_empty() && at.intersect(clip).intersect(window).is_positive() {
            texts.push(Item(words, rect(at)));
        }
    });
    View {
        screen: screen.to_owned(),
        view,
        controls,
        texts,
    }
}

/// A run's file and its ID are a random UUID, so each run's ID becomes `<run NAME>` in every
/// path and every file's text. Nothing else in a file changes before the comparison.
fn files(root: &Path) -> Files {
    fn walk(at: &Path, out: &mut Vec<PathBuf>) {
        let mut entries: Vec<_> = fs::read_dir(at)
            .unwrap_or_else(|error| panic!("G7: cannot list {}: {error}", at.display()))
            .map(|entry| entry.expect("a folder entry").path())
            .collect();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                walk(&path, out);
            } else {
                out.push(path);
            }
        }
    }
    #[derive(Deserialize)]
    struct RunName {
        name: String,
    }
    let mut paths = Vec::new();
    walk(root, &mut paths);
    let mut runs: Vec<(String, String)> = Vec::new();
    for path in &paths {
        let (Some(stem), Some("json")) = (
            path.file_stem().and_then(|stem| stem.to_str()),
            path.extension().and_then(|extension| extension.to_str()),
        ) else {
            continue;
        };
        if uuid::Uuid::parse_str(stem).is_err() {
            continue;
        }
        let text = fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("G7: cannot read {}: {error}", path.display()));
        let run: RunName = serde_json::from_str(&text)
            .unwrap_or_else(|error| panic!("G7: {} is not a run: {error}", path.display()));
        let mut label = format!("<run {}>", run.name);
        let mut more = 1;
        while runs.iter().any(|(_, other)| *other == label) {
            more += 1;
            label = format!("<run {} {more}>", run.name);
        }
        runs.push((stem.to_owned(), label));
    }
    let named = |text: &str| {
        runs.iter()
            .fold(text.to_owned(), |text, (id, label)| text.replace(id, label))
    };
    paths
        .iter()
        .map(|path| {
            let bytes = fs::read(path)
                .unwrap_or_else(|error| panic!("G7: cannot read {}: {error}", path.display()));
            let lines = match String::from_utf8(bytes) {
                Ok(text) => {
                    let text = named(&text);
                    if text.len() <= WHOLE {
                        text.lines().map(str::to_owned).collect()
                    } else {
                        vec![
                            format!("sha256 {:x}", Sha256::digest(text.as_bytes())),
                            format!("{} bytes", text.len()),
                        ]
                    }
                }
                Err(error) => {
                    let bytes = error.into_bytes();
                    vec![
                        format!("sha256 {:x}", Sha256::digest(&bytes)),
                        format!("{} bytes", bytes.len()),
                    ]
                }
            };
            let name = path
                .strip_prefix(root)
                .expect("inside the root")
                .to_string_lossy();
            (named(&name), lines)
        })
        .collect()
}

/// The evening's captures, in order.
struct Evening {
    views: Vec<View>,
    titles: Vec<String>,
}

impl Evening {
    /// Record this screen at both sizes, with Lookup closed and open. Lookup shows what it
    /// showed last, as Ctrl+K opens it.
    fn screen(&mut self, d: &mut Driver<'_>, screen: &str) {
        let was_open = d.app_mut().lookup.open;
        for open in [was_open, !was_open] {
            if d.app_mut().lookup.open != open {
                d.press_ctrl(Key::K);
                assert_eq!(
                    d.app_mut().lookup.open,
                    open,
                    "G7: Ctrl+K did not toggle Lookup on {screen}"
                );
            }
            self.both_sizes(d, screen, open);
        }
        if d.app_mut().lookup.open != was_open {
            d.press_ctrl(Key::K);
        }
    }

    /// Record Lookup's reader only, at both sizes, open on this entry. The DM would search
    /// for it; the ID keeps apart two entries of one name.
    fn reader(&mut self, d: &mut Driver<'_>, title: &str, portrait: bool) {
        d.app_mut().lookup.open_id(plain_reader(title, portrait));
        d.settle();
        let screen = format!(
            "lookup-reader {title}, {}",
            if portrait { "creature" } else { "npc" }
        );
        for size in [WIDE, NARROW] {
            self.views
                .push(d.at_size(size, |d| look(d, &screen, view_name(size, true))));
            let measured = d.at_size(size, |d| measure_title(d, title));
            self.titles.push(format!(
                "Lookup\t{}x{}\t{title}\t{}\t{measured}",
                size.0,
                size.1,
                if portrait {
                    "creature, with art"
                } else {
                    "NPC"
                }
            ));
        }
        d.press_ctrl(Key::K);
        assert!(!d.app_mut().lookup.open, "G7: Ctrl+K did not close Lookup");
    }

    fn both_sizes(&mut self, d: &mut Driver<'_>, screen: &str, open: bool) {
        for size in [WIDE, NARROW] {
            self.views
                .push(d.at_size(size, |d| look(d, screen, view_name(size, open))));
        }
    }
}

fn view_name(size: (f32, f32), open: bool) -> String {
    format!(
        "{}x{} Lookup {}",
        size.0,
        size.1,
        if open { "open" } else { "closed" }
    )
}

/// The reader's title in the `Display` type, what follows it, and the room left after them.
/// Inside ends at Lookup's search field, or at a scroll bar beside the reader.
fn measure_title(d: &Driver<'_>, title: &str) -> String {
    let mut found = None;
    d.each_painted_text(|text, _| {
        let size = text
            .galley
            .job
            .sections
            .first()
            .map_or(0.0, |section| section.format.font_id.size);
        if text.galley.job.text == title && found.is_none_or(|(_, old)| size > old) {
            found = Some((text.galley.rect.translate(text.pos.to_vec2()), size));
        }
    });
    let Some((at, size)) = found else {
        return "no title\t\t\t\t\t\t\t".to_owned();
    };
    let nodes = d.nodes();
    let beside = |node: &&super::driver::Found| {
        node.in_view() && node.rect.left() >= at.right() && node.rect.top() < at.bottom()
    };
    let art = nodes
        .iter()
        .filter(beside)
        .find(|node| node.name == title)
        .map(|node| node.rect);
    let edit = nodes
        .iter()
        .filter(beside)
        .find(|node| node.name == format!("Edit {title}"))
        .map(|node| node.rect);
    let search = nodes
        .iter()
        .find(|node| node.name == SEARCH_LOOKUP && node.in_view())
        .map_or(f32::NAN, |node| node.rect.right());
    let lane = d
        .chrome_lanes()
        .into_iter()
        .filter(|lane| lane.left() > at.right() && lane.left() < search && lane.top() < at.bottom())
        .map(|lane| lane.left())
        .fold(f32::NAN, f32::min);
    let inside = if lane.is_nan() { search } else { lane };
    let last = [Some(at), art, edit]
        .into_iter()
        .flatten()
        .map(|rect| rect.right())
        .fold(at.right(), f32::max);
    let edge = |rect: Option<egui::Rect>| {
        rect.map_or("-".to_owned(), |r| {
            format!("{}-{}", tenth(r.left()), tenth(r.right()))
        })
    };
    format!(
        "{size}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        tenth(at.width()),
        tenth(at.left()),
        edge(art),
        edge(edit),
        if lane.is_nan() { "no" } else { "yes" },
        tenth(inside - at.left()),
        tenth(inside - last)
    )
}

/// The plain evening: today's controls only, and no act that a feature adds.
fn play(d: &mut Driver<'_>) -> (Evening, Files) {
    d.set_base(WIDE);
    let root = d.root.join("campaigns");
    let at_start = files(&root);
    let mut evening = Evening {
        views: Vec::new(),
        titles: Vec::new(),
    };
    // Lookup holds Captain Varra's page, as the DM left it.
    d.app_mut()
        .lookup
        .open_id(plain_reader("Captain Varra", true));
    d.settle();
    d.press_ctrl(Key::K);

    d.click(NAV_SESSIONS);
    show_plan(d);
    d.expect("Road to the ford");
    d.state("session-tonight", Sizes::Standard);
    evening.screen(d, "session-tonight");

    d.click_starting(&format!("Session 7: {PLAIN_LAST} ·"));
    d.expect(PLAIN_PLAYED);
    d.click(PLAIN_PLAYED);
    d.state("session-last-played", Sizes::Standard);
    evening.screen(d, "session-last-played");

    d.click(NAV_ENCOUNTERS);
    d.state("encounters", Sizes::Standard);
    evening.screen(d, "encounters");

    resume_run(d, PLAIN_RUN);
    d.state("run-twelve", Sizes::Standard);
    evening.screen(d, "run-twelve");
    d.click("Keyboard commands");
    d.state("run-keys", Sizes::Standard);
    evening.screen(d, "run-keys");
    d.click("Keyboard commands");
    close_run(d);

    d.click(NAV_TIMELINE);
    d.state("timeline", Sizes::Standard);
    evening.screen(d, "timeline");

    d.click(NAV_SESSIONS);
    for title in PLAIN_TITLES {
        for portrait in [false, true] {
            evening.reader(d, title, portrait);
        }
    }

    d.click("New session");
    d.type_into("Session title", "The Shrine");
    d.click("Create session");
    d.state("session-new", Sizes::Standard);
    evening.screen(d, "session-new");
    write_note(d);
    d.type_into_editor("\nThe party camps at the shrine.\n");
    d.press_ctrl(Key::S);
    d.app_mut().sessions.flush();
    d.app_mut().timeline.save();
    d.settle();
    let at_end = files(&root)
        .into_iter()
        .map(|(path, lines)| {
            if at_start.get(&path) == Some(&lines) {
                (path, vec![SAME.to_owned()])
            } else {
                (path, lines)
            }
        })
        .collect();
    evening.views.iter().for_each(|view| {
        assert!(
            !view.controls.is_empty() && !view.texts.is_empty(),
            "G7: {} / {} captured nothing",
            view.screen,
            view.view
        );
    });
    (evening, merge_files(at_start, at_end))
}

/// Start and end, apart: `start:` and `end:` before each path.
fn merge_files(start: Files, end: Files) -> Files {
    start
        .into_iter()
        .map(|(path, lines)| (format!("start:{path}"), lines))
        .chain(
            end.into_iter()
                .map(|(path, lines)| (format!("end:{path}"), lines)),
        )
        .collect()
}

fn compare_items(
    screen: &str,
    view: &str,
    what: &str,
    old: &[Item],
    new: &[Item],
    out: &mut Vec<Difference>,
) {
    let group = |items: &[Item]| {
        let mut by: BTreeMap<String, Vec<[f32; 4]>> = BTreeMap::new();
        for Item(name, at) in items {
            by.entry(name.clone()).or_default().push(*at);
        }
        by
    };
    let (old, new) = (group(old), group(new));
    let difference = |change: &str, name: &str, detail: String| Difference {
        screen: screen.to_owned(),
        view: view.to_owned(),
        what: what.to_owned(),
        change: change.to_owned(),
        name: name.to_owned(),
        detail,
    };
    for (name, was) in &old {
        let now = new.get(name).map_or(&[][..], Vec::as_slice);
        for (index, a) in was.iter().enumerate() {
            match now.get(index) {
                None => out.push(difference("gone", name, format!("was {}", words(*a)))),
                Some(b) if a != b => {
                    let change = if a[2] == b[2] && a[3] == b[3] {
                        "moved"
                    } else if a[0] == b[0] && a[1] == b[1] && a[2] == b[2] {
                        "height"
                    } else if a[0] == b[0] && a[1] == b[1] && a[3] == b[3] {
                        "width"
                    } else {
                        "resized"
                    };
                    out.push(difference(
                        change,
                        name,
                        format!("{} -> {}", words(*a), words(*b)),
                    ));
                }
                Some(_) => {}
            }
        }
        for b in now.iter().skip(was.len()) {
            out.push(difference("new", name, format!("at {}", words(*b))));
        }
    }
    for (name, now) in &new {
        if !old.contains_key(name) {
            for b in now {
                out.push(difference("new", name, format!("at {}", words(*b))));
            }
        }
    }
}

fn compare_files(old: &Files, new: &Files, out: &mut Vec<Difference>) {
    let difference = |change: &str, path: &str, detail: String| {
        let (when, path) = path.split_once(':').unwrap_or(("", path));
        Difference {
            screen: format!("files at the {when}"),
            view: String::new(),
            what: "file".to_owned(),
            change: change.to_owned(),
            name: path.to_owned(),
            detail,
        }
    };
    for (path, was) in old {
        match new.get(path) {
            None => out.push(difference("gone", path, format!("{} lines", was.len()))),
            Some(now) if now != was => {
                let mut left: Vec<&String> = now.iter().collect();
                let mut lines = Vec::new();
                for line in was {
                    if let Some(at) = left.iter().position(|other| *other == line) {
                        left.remove(at);
                    } else {
                        lines.push(format!("- {line}"));
                    }
                }
                lines.extend(left.into_iter().map(|line| format!("+ {line}")));
                if lines.is_empty() {
                    lines.push("the same lines in another order".to_owned());
                }
                out.push(difference("lines", path, lines.join(" | ")));
            }
            Some(_) => {}
        }
    }
    for (path, now) in new {
        if !old.contains_key(path) {
            out.push(difference("new", path, now.join(" | ")));
        }
    }
}

fn differences(reference: &Reference, views: &[View], files: &Files) -> Vec<Difference> {
    let mut out = Vec::new();
    let key = |view: &View| (view.screen.clone(), view.view.clone());
    for old in &reference.views {
        match views.iter().find(|new| key(new) == key(old)) {
            None => out.push(Difference {
                screen: old.screen.clone(),
                view: old.view.clone(),
                what: "view".into(),
                change: "gone".into(),
                name: String::new(),
                detail: "the evening no longer reaches this view".into(),
            }),
            Some(new) => {
                compare_items(
                    &old.screen,
                    &old.view,
                    "control",
                    &old.controls,
                    &new.controls,
                    &mut out,
                );
                compare_items(
                    &old.screen,
                    &old.view,
                    "text",
                    &old.texts,
                    &new.texts,
                    &mut out,
                );
            }
        }
    }
    for new in views {
        if !reference.views.iter().any(|old| key(old) == key(new)) {
            out.push(Difference {
                screen: new.screen.clone(),
                view: new.view.clone(),
                what: "view".into(),
                change: "new".into(),
                name: String::new(),
                detail: "a view the reference does not hold".into(),
            });
        }
    }
    let old_files = merge_files(
        reference.files_at_start.clone(),
        reference.files_at_end.clone(),
    );
    compare_files(&old_files, files, &mut out);
    out
}

/// The reference, one item to a line, so that a change reads as a short diff.
fn reference_text(reference: &Reference) -> String {
    let mut out = String::from("{\n");
    let _ = writeln!(out, "\"about\": {},", json(&reference.about));
    out.push_str("\"views\": [\n");
    for (index, view) in reference.views.iter().enumerate() {
        let _ = writeln!(
            out,
            "{{\"screen\": {}, \"view\": {},",
            json(&view.screen),
            json(&view.view)
        );
        for (name, items) in [("controls", &view.controls), ("texts", &view.texts)] {
            let _ = writeln!(out, "\"{name}\": [");
            let lines: Vec<String> = items.iter().map(json).collect();
            out.push_str(&lines.join(",\n"));
            out.push_str(if name == "controls" { "\n],\n" } else { "\n]" });
        }
        out.push_str(if index + 1 == reference.views.len() {
            "}\n"
        } else {
            "},\n"
        });
    }
    out.push_str("],\n");
    for (name, files) in [
        ("files_at_start", &reference.files_at_start),
        ("files_at_end", &reference.files_at_end),
    ] {
        let _ = writeln!(out, "\"{name}\": {{");
        let entries: Vec<String> = files
            .iter()
            .map(|(path, lines)| {
                let lines: Vec<String> = lines.iter().map(|line| json(line)).collect();
                format!("{}: [\n{}\n]", json(path), lines.join(",\n"))
            })
            .collect();
        out.push_str(&entries.join(",\n"));
        out.push_str(if name == "files_at_start" {
            "\n},\n"
        } else {
            "\n}\n"
        });
    }
    out.push_str("}\n");
    out
}

/// Any value as one line of JSON.
fn json<T: Serialize + ?Sized>(value: &T) -> String {
    serde_json::to_string(value).expect("encode JSON")
}

fn read<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("G7: cannot read {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("G7: cannot read {}: {error}", path.display()))
}

/// G7. The scenario `plain-evening`.
pub fn run(d: &mut Driver<'_>) {
    let (evening, files) = play(d);
    let allowed: Allowed = read(&folder().join("allowed.json"));
    assert!(
        !evening.views.is_empty() && !files.is_empty(),
        "G7: the evening captured no view or no file, so it can prove nothing"
    );
    if std::env::var_os(WRITE).is_some() {
        assert!(
            allowed.rows.is_empty(),
            "G7: allowed.json has rows. They describe changes from this reference, so write it only on main with no rows"
        );
        let (start, end): (Files, Files) = files
            .into_iter()
            .partition(|(path, _)| path.starts_with("start:"));
        let strip = |files: Files, prefix: &str| -> Files {
            files
                .into_iter()
                .map(|(path, lines)| (path[prefix.len()..].to_owned(), lines))
                .collect()
        };
        let reference = Reference {
            about:
                "G7 reads this. Written by audit-freeze from the plain-evening scenario on main. \
                    Change it only by writing it again on main."
                    .into(),
            views: evening.views,
            files_at_start: strip(start, "start:"),
            files_at_end: strip(end, "end:"),
        };
        fs::write(folder().join("reference.json"), reference_text(&reference))
            .expect("write the reference");
        let mut titles = String::from(
            "reader\twindow\ttitle\tentry\tfont size\ttitle width\ttitle left\tart\tEdit\tscroll bar\tinside\troom after\n",
        );
        for line in &evening.titles {
            let _ = writeln!(titles, "{line}");
        }
        fs::write(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("artifacts/dm-audit/titles.tsv"),
            titles,
        )
        .expect("write titles.tsv");
        println!("plain-evening: wrote the reference");
        return;
    }
    let reference: Reference = read(&folder().join("reference.json"));
    assert!(
        !reference.views.is_empty()
            && reference
                .views
                .iter()
                .all(|view| !view.controls.is_empty() && !view.texts.is_empty())
            && !reference.files_at_start.is_empty()
            && !reference.files_at_end.is_empty(),
        "G7: the reference holds an empty view or no files, so it can prove nothing"
    );
    let mut found = differences(&reference, &evening.views, &files);
    let mut faults = Vec::new();
    for row in &allowed.rows {
        if !CONTRACT_ROWS.contains(&row.row) {
            faults.push(format!(
                "allowed.json names row {}, which section 2.5 does not have: {}",
                row.row, row.difference
            ));
        }
        match found
            .iter()
            .position(|difference| *difference == row.difference)
        {
            Some(at) => {
                found.remove(at);
            }
            None => faults.push(format!(
                "allowed.json row {} matches no difference of this evening: {}",
                row.row, row.difference
            )),
        }
    }
    faults.extend(
        found
            .iter()
            .map(|difference| format!("not allowed: {difference}")),
    );
    assert!(
        faults.is_empty(),
        "G7: {} differences from the plain reference. First: {}\nAll:\n{}",
        faults.len(),
        faults[0],
        faults.join("\n")
    );
    d.observe(
        "G7",
        true,
        format!(
            "{} views and {} files match the reference, with {} allowed rows",
            evening.views.len(),
            files.len(),
            allowed.rows.len()
        ),
    );
}
