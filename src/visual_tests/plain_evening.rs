//! G7, the plain-campaign check of `docs/design/session-cycle.md` section 5.3.
//!
//! The scenario `plain-evening` plays the frozen plain campaign (`Seed::Plain`) with today's
//! controls only, and compares what it shows and writes with `plain_evening/reference.json`:
//!
//! - **Views.** `SCREENS` fixes every view in code. A page is seen at 1200x800 and 960x640 with
//!   Lookup closed and open; a Lookup reader at both sizes; an open menu or a hover text at both
//!   sizes as it stands. Each view holds every named control in view and every painted text,
//!   with its rectangle, and the part a person sees when that is less. A missing or extra view
//!   always fails, and no allowed row can excuse it.
//! - **Acts.** Every input of the evening is an act. For each, the words that any frame painted
//!   and the last frame before it did not, the words that went, and each campaign file that was
//!   written or changed, by digest. A notice shown for one frame, or a file written and put
//!   back, is in this record.
//! - **Files.** Every campaign file at the start and the end, compared by its bytes. A run's
//!   file must open as a run (`Run::open`); in it, and nowhere else, the test folder in
//!   `splat_run_source_path` becomes `<world>`. Each file that the evening changed is kept
//!   whole in `plain_evening/files/`, so a new reference shows as a readable diff.
//!
//! Any other difference fails unless one row of `plain_evening/allowed.json` names exactly that
//! difference, with its row of section 2.5. A row that matches nothing fails.
//!
//! **G7 compares only with `SPLAT_FROZEN=1`.** Any change of the user interface fails it by
//! design, so the default gate runs the scenario and its captures and skips the comparison. The
//! pull requests of the session-cycle build plan run the gate with the switch:
//!
//! - `devenv shell -- audit-freeze reference` writes the reference, its files and
//!   `artifacts/dm-audit/titles.tsv` on an unchanged commit of `main`, and records that commit in
//!   `frozen_at`. A pull request carries that output as one commit of its own, and makes it again
//!   after each merge of `main`. It refuses a tree with other changes, and while `allowed.json`
//!   has a row.
//! - `devenv shell -- audit-freeze add` is how a feature adds a view of its own surface (to
//!   `SCREENS` or `HOVERS`) and the acts that reach it: it inserts the new views and acts in their
//!   places and leaves every old record as it is. It refuses, and writes nothing, while any other
//!   difference has no row in `allowed.json`.
//! - `devenv shell -- audit-freeze verify` writes the reference into a scratch folder and fails
//!   unless the tree's reference is byte for byte the same: a reviewer runs it at the commit that
//!   `frozen_at` names, with the pull request's frozen files copied in.
use super::{
    driver::{Driver, Sizes},
    fixture::{
        PLAIN_LAST, PLAIN_OLDER, PLAIN_PLAYED, PLAIN_RUN, PLAIN_TITLES, plain_package_creature,
        plain_reader,
    },
    steps::*,
};
use eframe::egui::{self, Key};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use splat::character::stable_id;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

/// F3 of the session-cycle design: tonight's next fight and its `Start fight` stay whole and in
/// view at 960x640, with Lookup closed and open, whatever scenes the note's plan holds; an
/// opened fight's `Read encounter` too. Scenes fold first.
fn next_fight_in_view(d: &mut Driver<'_>, screen: &str, lookup: bool) {
    if !screen.starts_with("session-tonight") {
        return;
    }
    let (whole, detail) = d.at_size(NARROW, |d| {
        let nodes = d.nodes();
        let start = nodes
            .iter()
            .find(|node| node.name == "Start fight: Shrine of Cinders");
        let read = nodes.iter().find(|node| node.name == "Read encounter");
        let opened = screen.ends_with("planned fight open");
        (
            start.is_some_and(|node| node.fully_in_view())
                && (!opened || read.is_some_and(|node| node.fully_in_view())),
            format!(
                "Start fight {:?}, Read encounter {:?}",
                start.map(|node| (node.rect, node.shown)),
                read.map(|node| (node.rect, node.shown))
            ),
        )
    });
    assert!(
        whole,
        "G7, F3: at 960x640 with Lookup {}, {screen}: the next fight's action is not whole and in view. {detail}",
        if lookup { "open" } else { "closed" }
    );
}

/// `all` writes the reference from this run; `add` adds new views and acts only.
const WRITE: &str = "SPLAT_PLAIN_EVENING_WRITE";
/// G7 compares with the reference only when this is `1`.
const FROZEN: &str = "SPLAT_FROZEN";
/// The commit of `main` a reference is written on, which `audit-freeze reference` passes.
const FROZEN_AT: &str = "SPLAT_FROZEN_AT";
/// Where `audit-freeze verify` writes its reference, in place of the tree.
const OUT: &str = "SPLAT_PLAIN_EVENING_OUT";
/// The one way a feature adds its views and acts.
const ADD_COMMAND: &str = "devenv shell -- audit-freeze add";
const SCENARIO: &str = "plain-evening";
const WIDE: (f32, f32) = (1200.0, 800.0);
const NARROW: (f32, f32) = (960.0, 640.0);
/// Section 2.5 numbers its rows 1 to 26.
const CONTRACT_ROWS: std::ops::RangeInclusive<u32> = 1..=26;
/// What the test folder becomes in a run's `splat_run_source_path`.
const WORLD: &str = "<world>";
const SAME: &str = "the same as at the start";
/// The app writes this index again with the same bytes on a one-second poll of real time, so
/// the act in which that write lands is not repeatable. A change of its bytes is still recorded.
const POLLED: &[&str] = &["emberfall/Sessions/.splat-session-paths.json"];

/// How a screen is seen.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Shown {
    /// A page: both sizes, with Lookup closed and open.
    Page,
    /// Lookup's reader: both sizes, Lookup open.
    Reader,
    /// An open menu or a hover text: both sizes, as it stands.
    Held,
}
use Shown::{Held, Page, Reader};

/// Every screen of the plain evening, in the order it reaches them. G7 needs each view of each,
/// and no other; `allowed.json` cannot change this list.
const SCREENS: &[(&str, Shown)] = &[
    ("start", Held),
    ("session-tonight", Page),
    ("session-tonight: More open", Held),
    ("session-tonight: Travel row menu", Held),
    ("session-tonight: fight row menu", Held),
    ("session-tonight: Travel row open", Page),
    ("session-tonight: planned fight open", Page),
    ("session-last", Held),
    ("session-last: played fight open", Page),
    ("run-ended: played last", Page),
    ("run-ended: closed", Held),
    ("session-last: older fight open", Held),
    ("run-ended: played before", Page),
    ("run-ended: older closed", Held),
    ("encounters", Page),
    ("encounters: row menu", Held),
    ("encounter-editor", Page),
    ("encounter-editor: loot table menu", Held),
    ("run-twelve", Page),
    ("run-keys", Page),
    ("run-twelve: Brannoch selected", Held),
    ("run-conditions", Page),
    ("run-arrivals", Page),
    ("run-next-round", Page),
    ("run-closed", Held),
    ("timeline", Page),
    ("session-last: again", Held),
    ("lookup-reader Captain Varra, npc", Reader),
    ("lookup-reader Captain Varra, creature", Reader),
    ("lookup-reader Oren Thistlewick, npc", Reader),
    ("lookup-reader Oren Thistlewick, creature", Reader),
    ("lookup-reader Captain Varra Black, npc", Reader),
    ("lookup-reader Captain Varra Black, creature", Reader),
    ("lookup-reader Road Encounters", Reader),
    ("lookup-reader Cinder Wolf", Reader),
    ("world-list", Held),
    ("world-reader", Page),
    ("session-new: form", Held),
    ("session-new: form with a title", Held),
    ("session-new", Page),
    ("session-new: note written", Held),
    ("session-new: note saved", Held),
    ("timeline: Road to the ford open", Held),
];

/// What a hover rests on.
#[derive(Clone, Copy)]
enum Target {
    /// A control by its whole accessible name.
    Name(&'static str),
    /// A control whose accessible name starts with these words.
    Starting(&'static str),
    /// Painted words, where the words have no control of their own.
    Words(&'static str),
}
use Target::{Name, Starting, Words};

/// Each hover G7 records, on the screen it belongs to: a control that section 2.5 protects, or
/// that a feature may give words to. A control with no hover today is here too, so a hover it
/// gains is a difference. The view is `<screen>: <label> hover`, at both sizes.
const HOVERS: &[(&str, &str, Target, &str)] = &[
    (
        "session-tonight",
        "Last time",
        Name("Session 7: The Road to Emberfall"),
        "Rows 11 and 16: the Last time line gains Carry over and reads another source; its hover holds the whole recap",
    ),
    (
        "session-tonight",
        "planned fight",
        Name("Shrine of Cinders"),
        "Row 1: a planned fight's summary gains a difficulty word, whose whole words may go to a hover",
    ),
    (
        "session-tonight",
        "Travel row menu button",
        Name("More actions for Road to the ford"),
        "Row 12: the Travel row's menu gains a line; its button's hover names the menu",
    ),
    (
        "session-last: played fight open",
        "played fight",
        Name("Wolves at the Ford Below the Burning Shrine Road"),
        "Row 10: Add rewards joins a played row whose long title may then shorten to a hover",
    ),
    (
        "run-ended: played last",
        "Continue fight",
        Name("Continue fight"),
        "F8: the one control that takes an ending away; a feature may give it words",
    ),
    (
        "run-ended: played before",
        "reason",
        Words("A later fight or rest was played"),
        "F8: the reason a fight cannot go on; a narrow place may shorten it to a hover",
    ),
    (
        "encounters",
        "row menu button",
        Name("More actions for Shrine of Cinders"),
        "Row 13: Remove… joins a Ready to run row's menu",
    ),
    (
        "run-twelve",
        "row",
        Words("Bless 10 rounds"),
        "Rows 5 and 15: a row's condition words; F2 folds effects and puts their whole words in the hover",
    ),
    (
        "run-conditions",
        "Conditions",
        Name("Conditions for this participant"),
        "Row 6: Effect joins the dock beside Conditions",
    ),
    (
        "run-next-round",
        "Undo",
        Starting("Undo"),
        "Row 20: Undo shares its line with the notice line, and its target may shorten to its hover",
    ),
    (
        "world-reader",
        "Edit",
        Name("Edit Captain Varra Black"),
        "Rows 8 and 9: Add to Log and Start fight follow Edit in the reader's header",
    ),
    (
        "lookup-reader Cinder Wolf",
        "art",
        Name("Cinder Wolf"),
        "Row 21: Start fight joins a package creature's header, after its art",
    ),
];

/// The acts after which the screen needs no view of its own, each with why. After any other
/// act, the screen must equal a view that G7 records, or the next view must be of it.
const PASSING: &[(&str, &str)] = &[
    (
        "open Captain Varra in Lookup",
        "Setting the stage: the same reader is the view lookup-reader Captain Varra, creature",
    ),
    (
        "Next turn ",
        "One turn of twelve: its words are in the act's record, and the round's end is a view",
    ),
    (
        "close Roll on a table for the encounter at 960x640",
        "The editor stays scrolled to its loot row, as the view of that menu at 960x640 shows",
    ),
];

fn passing(act: &str) -> bool {
    PASSING
        .iter()
        .any(|(name, _)| act == *name || (name.ends_with(' ') && act.starts_with(name)))
}

fn view_name(size: (f32, f32), lookup: Option<bool>) -> String {
    match lookup {
        Some(open) => format!(
            "{}x{} Lookup {}",
            size.0,
            size.1,
            if open { "open" } else { "closed" }
        ),
        None => format!("{}x{}", size.0, size.1),
    }
}

/// Every view G7 needs, in order.
fn required_views() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (screen, shown) in SCREENS {
        let lookups: &[Option<bool>] = match shown {
            Page => &[Some(false), Some(true)],
            Reader => &[Some(true)],
            Held => &[None],
        };
        for lookup in lookups {
            for size in [WIDE, NARROW] {
                out.push(((*screen).to_owned(), view_name(size, *lookup)));
            }
        }
        for (on, label, ..) in HOVERS.iter().filter(|(on, ..)| on == screen) {
            for size in [WIDE, NARROW] {
                out.push((format!("{on}: {label} hover"), view_name(size, None)));
            }
        }
    }
    out
}

/// A control (`Role: name`) or a painted text, with its rectangle `[x, y, width, height]`, then
/// the part a person can see when that is less: 4 numbers, or 8.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Item(String, Vec<f32>);

/// One screen at one size.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct View {
    screen: String,
    view: String,
    controls: Vec<Item>,
    texts: Vec<Item>,
}

/// One act of the evening: what appeared in any frame, what went, and what it wrote.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Act {
    act: String,
    appeared: Vec<String>,
    gone: Vec<String>,
    files: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
struct Reference {
    about: String,
    /// The commit of `main` this reference was written on.
    frozen_at: String,
    views: Vec<View>,
    acts: Vec<Act>,
    /// Each file at the start, by the SHA-256 of its bytes.
    files_at_start: BTreeMap<String, String>,
    /// Each file at the end: `SAME`, or the SHA-256 of its bytes, kept whole in `files/`.
    files_at_end: BTreeMap<String, String>,
}

/// Each campaign file by its path, as G7 compares it.
type Bytes = BTreeMap<String, Vec<u8>>;

/// One part of an act, as `compare_acts` lists it.
type ActPart = (&'static str, fn(&Act) -> &Vec<String>);

/// One difference from the reference. An allowed row holds one of these, whole.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Difference {
    screen: String,
    view: String,
    /// `control`, `text`, `act` or `file`.
    what: String,
    /// `new`, `gone`, `moved`, `height`, `width`, `resized`, `clipped`, `changed` or `bytes`.
    change: String,
    name: String,
    /// The rectangles, the words, or the lines that differ.
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

#[derive(Clone, Serialize, Deserialize)]
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

fn item(name: String, whole: egui::Rect, shown: egui::Rect) -> Item {
    let mut at = rect(whole).to_vec();
    if rect(shown) != rect(whole) {
        at.extend(rect(shown));
    }
    Item(name, at)
}

fn words(r: &[f32]) -> String {
    let mut out = format!("{} {} {}x{}", r[0], r[1], r[2], r[3]);
    if let [_, _, _, _, x, y, w, h] = r {
        let _ = write!(out, ", seen {x} {y} {w}x{h}");
    }
    out
}

/// What a person can see now: each named control in view, and each painted text in view. The
/// World page names a note by its full path, so the test folder `world` becomes `<world>`; a
/// name that already says `<world>` is marked, so it cannot pass for a path.
fn look(d: &Driver<'_>, world: &str, screen: &str, view: String) -> View {
    let portable = |words: String| {
        if words.contains(WORLD) {
            format!("G7: the app shows {WORLD} itself: {words}")
        } else {
            words.replace(world, WORLD)
        }
    };
    let size = d.window_size();
    let window = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(size.0, size.1));
    let controls = d
        .nodes()
        .into_iter()
        .filter(|node| !node.name.trim().is_empty() && node.in_view())
        .map(|node| {
            item(
                portable(format!("{:?}: {}", node.role, node.name)),
                node.rect,
                node.shown,
            )
        })
        .collect();
    let mut texts = Vec::new();
    d.each_painted_text(|text, clip| {
        let words = text.galley.job.text.clone();
        let at = text.galley.rect.translate(text.pos.to_vec2());
        let shown = at.intersect(clip).intersect(window);
        if !words.trim().is_empty() && shown.is_positive() {
            texts.push(item(portable(words), at, shown));
        }
    });
    View {
        screen: screen.to_owned(),
        view,
        controls,
        texts,
    }
}

fn hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// The campaigns' files, as G7 compares them.
struct Files {
    root: PathBuf,
    /// The test folder, which a run's source path holds.
    world: String,
}

impl Files {
    fn paths(&self) -> Vec<PathBuf> {
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
        let mut out = Vec::new();
        walk(&self.root, &mut out);
        out
    }

    fn name(&self, path: &Path) -> String {
        path.strip_prefix(&self.root)
            .expect("inside the campaigns")
            .to_string_lossy()
            .into_owned()
    }

    /// The file's bytes. A run's file must open as a run of its campaign, and only its typed
    /// `splat_run_source_path` loses the test folder. A file that is not right says why instead.
    fn bytes(&self, path: &Path) -> Vec<u8> {
        let bytes = fs::read(path)
            .unwrap_or_else(|error| panic!("G7: cannot read {}: {error}", path.display()));
        let name = self.name(path);
        let mut out = bytes.clone();
        if name.contains("/Encounter Runs/") && name.ends_with(".json") {
            let folder = name.split('/').next().unwrap_or_default();
            let campaign = stable_id(&format!("visual:{SCENARIO}:{folder}"));
            if let Err(error) = crate::encounter_run::Run::open(path, campaign) {
                return format!("G7: this file does not open as a run: {error}").into_bytes();
            }
            let value: serde_json::Value =
                serde_json::from_slice(&bytes).expect("a run that opened is JSON");
            let mut sources = BTreeSet::new();
            collect_sources(&value, &mut sources);
            let field = |path: &str| {
                format!(
                    "\"splat_run_source_path\": {}",
                    serde_json::to_string(path).expect("a JSON string")
                )
            };
            let mut text = String::from_utf8(out).expect("a run is UTF-8");
            for source in sources {
                if source.contains(WORLD) {
                    return format!("G7: a source path already says {WORLD}: {source}")
                        .into_bytes();
                }
                if let Some(rest) = source.strip_prefix(&self.world) {
                    text = text.replace(&field(&source), &field(&format!("{WORLD}{rest}")));
                }
            }
            out = text.into_bytes();
        }
        if out
            .windows(self.world.len())
            .any(|window| window == self.world.as_bytes())
        {
            return format!("G7: this file holds the test folder {}", self.world).into_bytes();
        }
        out
    }

    fn all(&self) -> Bytes {
        self.paths()
            .into_iter()
            .map(|path| (self.name(&path), self.bytes(&path)))
            .collect()
    }

    /// Each file's time and digest, to see which an act wrote.
    fn stamps(&self) -> BTreeMap<String, (Option<SystemTime>, String)> {
        self.paths()
            .into_iter()
            .map(|path| {
                let time = fs::metadata(&path)
                    .and_then(|metadata| metadata.modified())
                    .ok();
                (self.name(&path), (time, hex(&self.bytes(&path))))
            })
            .collect()
    }
}

fn collect_sources(value: &serde_json::Value, out: &mut BTreeSet<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                if key == "splat_run_source_path"
                    && let Some(path) = value.as_str()
                {
                    out.insert(path.to_owned());
                }
                collect_sources(value, out);
            }
        }
        serde_json::Value::Array(values) => values.iter().for_each(|v| collect_sources(v, out)),
        _ => {}
    }
}

/// The evening's record, in order.
struct Evening {
    files: Files,
    views: Vec<View>,
    acts: Vec<Act>,
    titles: Vec<String>,
    /// Each view taken, by its window size and its controls and words.
    seen: BTreeSet<(String, String)>,
    /// The act after which the screen is in no view yet.
    unseen: Option<String>,
    /// What the evening itself found wrong.
    faults: Vec<String>,
}

/// A view's controls and words, apart from its names.
fn fingerprint(view: &View) -> String {
    hex(json(&(&view.controls, &view.texts)).as_bytes())
}

impl Evening {
    /// One input of the evening, with what it showed and wrote.
    fn act(&mut self, d: &mut Driver<'_>, what: &str, step: impl FnOnce(&mut Driver<'_>)) {
        self.check_seen();
        let before = d.last_words();
        let files = self.files.stamps();
        let ((), during) = d.traced(step);
        let after = d.last_words();
        let now = self.files.stamps();
        let mut written = Vec::new();
        for (path, (time, digest)) in &now {
            match files.get(path) {
                None => written.push(format!("{path}: new, sha256 {}", &digest[..16])),
                Some((old_time, old))
                    if old != digest || (old_time != time && !POLLED.contains(&path.as_str())) =>
                {
                    written.push(format!(
                        "{path}: written, sha256 {}{}",
                        &digest[..16],
                        if old == digest {
                            ", the same bytes"
                        } else {
                            ""
                        }
                    ));
                }
                Some(_) => {}
            }
        }
        for path in files.keys().filter(|path| !now.contains_key(*path)) {
            written.push(format!("{path}: removed"));
        }
        let portable = |words: &String| {
            if words.contains(WORLD) {
                format!("G7: the app shows {WORLD} itself: {words}")
            } else {
                words.replace(&self.files.world, WORLD)
            }
        };
        self.acts.push(Act {
            act: what.to_owned(),
            appeared: during.difference(&before).map(portable).collect(),
            gone: before.difference(&after).map(portable).collect(),
            files: written,
        });
        let size = d.window_size();
        let now = look(d, &self.files.world, "", String::new());
        if !self
            .seen
            .contains(&(view_name(size, None), fingerprint(&now)))
        {
            self.unseen = Some(what.to_owned());
        }
    }

    /// The screen after the last act must be in a view by now, unless that act is passing.
    fn check_seen(&mut self) {
        if let Some(act) = self.unseen.take()
            && !passing(&act)
        {
            self.faults.push(format!(
                "the screen after the act {act:?} is in no view: add a view of it to SCREENS, or list the act in PASSING with a reason"
            ));
        }
    }

    /// Keep a view, and know its screen when an act shows it again.
    fn keep(&mut self, view: View, size: (f32, f32)) {
        self.seen
            .insert((view_name(size, None), fingerprint(&view)));
        // A view taken now shows the screen that the last act left.
        self.unseen = None;
        self.views.push(view);
    }

    /// Each hover of `HOVERS` on this screen, at both sizes.
    fn hovers(&mut self, d: &mut Driver<'_>, screen: &str) {
        for (_, label, target, _) in HOVERS.iter().filter(|(on, ..)| *on == screen) {
            let name = format!("{screen}: {label} hover");
            self.hover(d, &name, |d| match *target {
                Name(name) => d.hover_name(name),
                Starting(words) => {
                    let name = d.name_starting(words);
                    d.hover_name(&name);
                }
                Words(words) => d.hover_words(words),
            });
        }
    }

    fn lookup(&mut self, d: &mut Driver<'_>, open: bool) {
        if d.app_mut().lookup.open != open {
            let what = if open {
                "Ctrl+K opens Lookup"
            } else {
                "Ctrl+K closes Lookup"
            };
            self.act(d, what, |d| d.press_ctrl(Key::K));
            assert_eq!(
                d.app_mut().lookup.open,
                open,
                "G7: Ctrl+K did not toggle Lookup"
            );
        }
    }

    fn both_sizes(&mut self, d: &mut Driver<'_>, screen: &str, lookup: Option<bool>) {
        for size in [WIDE, NARROW] {
            let view = view_name(size, lookup);
            let world = &self.files.world;
            let seen = d.at_size(size, |d| look(d, world, screen, view));
            self.keep(seen, size);
        }
    }

    /// A screen as it stands, at both sizes, Lookup as it is.
    fn held(&mut self, d: &mut Driver<'_>, screen: &str) {
        self.both_sizes(d, screen, None);
    }

    /// A page with Lookup closed, then open, as Ctrl+K shows what Lookup showed last. Also a
    /// capture for the gate's ergonomics.
    fn page(&mut self, d: &mut Driver<'_>, screen: &str) {
        let was_open = d.app_mut().lookup.open;
        self.lookup(d, false);
        d.state(screen, Sizes::Standard);
        self.both_sizes(d, screen, Some(false));
        next_fight_in_view(d, screen, false);
        self.lookup(d, true);
        self.both_sizes(d, screen, Some(true));
        next_fight_in_view(d, screen, true);
        self.lookup(d, was_open);
    }

    /// An open menu at both sizes, opened at each, since a resize closes it. Each of `items`
    /// must be in each view.
    fn menu(&mut self, d: &mut Driver<'_>, screen: &str, button: &str, items: &[&str]) {
        for size in [WIDE, NARROW] {
            let view = view_name(size, None);
            d.set_base(size);
            self.act(d, &format!("open {button} at {view}"), |d| d.click(button));
            let seen = look(d, &self.files.world, screen, view.clone());
            for wanted in items {
                assert!(
                    seen.controls
                        .iter()
                        .any(|Item(name, _)| name.ends_with(wanted)),
                    "G7: the menu of {screen} at {view} does not hold {wanted:?}"
                );
            }
            self.keep(seen, size);
            self.act(d, &format!("close {button} at {view}"), |d| {
                d.press(Key::Escape);
            });
        }
        d.set_base(WIDE);
    }

    /// A hover text, as it stands at both sizes. `rest` puts the pointer on its target.
    fn hover(&mut self, d: &mut Driver<'_>, screen: &str, rest: impl Fn(&mut Driver<'_>)) {
        for size in [WIDE, NARROW] {
            let view = view_name(size, None);
            let world = &self.files.world;
            let seen = d.at_size_mut(size, |d| {
                rest(d);
                let seen = look(d, world, screen, view);
                d.pointer_away();
                seen
            });
            self.views.push(seen);
            // A hover's view is not a screen an act leaves, so it does not count as seen.
        }
    }

    /// Lookup's reader on one entry, at both sizes.
    fn reader(&mut self, d: &mut Driver<'_>, screen: &str, open: impl FnOnce(&mut Driver<'_>)) {
        self.act(d, &format!("open {screen}"), open);
        assert!(d.app_mut().lookup.open, "G7: {screen} did not open Lookup");
        self.both_sizes(d, screen, Some(true));
    }

    /// A reader of a title that section 1.2 sums, with the title measured.
    fn title_reader(&mut self, d: &mut Driver<'_>, title: &str, portrait: bool) {
        let screen = format!(
            "lookup-reader {title}, {}",
            if portrait { "creature" } else { "npc" }
        );
        self.reader(d, &screen, |d| {
            d.app_mut().lookup.open_id(plain_reader(title, portrait));
            d.settle();
        });
        for size in [WIDE, NARROW] {
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
        self.lookup(d, false);
    }
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
    // The first control of this name in view, right of the title and level with it.
    let beside = |name: &str| {
        nodes
            .iter()
            .find(|node| {
                node.name == name
                    && node.in_view()
                    && node.rect.left() >= at.right()
                    && node.rect.top() < at.bottom()
            })
            .map(|node| node.rect)
    };
    let art = beside(title);
    let edit = beside(&format!("Edit {title}"));
    let search = nodes
        .iter()
        .find(|node| node.name == SEARCH_LOOKUP && node.in_view())
        .map_or(f32::NAN, |node| node.rect.right());
    let lane = d
        .chrome_lanes()
        .into_iter()
        .filter(|lane| {
            lane.left() > at.right() && lane.top() <= at.bottom() && lane.bottom() >= at.top()
        })
        .map(|lane| lane.left())
        .fold(f32::NAN, f32::min);
    // `min` keeps the number when the other side is NaN: no lane, or no search field.
    let inside = search.min(lane);
    let last = [art, edit]
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
        if lane.is_nan() {
            "no".to_owned()
        } else {
            format!("at {}", tenth(lane))
        },
        tenth(inside - at.left()),
        tenth(inside - last)
    )
}

/// The plain evening: today's controls only, and no act that a feature adds. Returns the
/// record and the files at the start and the end.
fn play(d: &mut Driver<'_>) -> (Evening, Bytes, Bytes) {
    d.set_base(WIDE);
    let mut ev = Evening {
        files: Files {
            root: d.root.join("campaigns"),
            world: d.root.to_string_lossy().into_owned(),
        },
        views: Vec::new(),
        acts: Vec::new(),
        titles: Vec::new(),
        seen: BTreeSet::new(),
        unseen: None,
        faults: Vec::new(),
    };
    let at_start = ev.files.all();
    // Lookup holds Captain Varra's page, as the DM left it.
    ev.act(d, "open Captain Varra in Lookup", |d| {
        d.app_mut()
            .lookup
            .open_id(plain_reader("Captain Varra", true));
        d.settle();
    });
    ev.lookup(d, false);
    ev.held(d, "start");

    ev.act(d, "Session", |d| {
        d.click(NAV_SESSIONS);
        show_plan(d);
    });
    d.expect("Road to the ford");
    ev.page(d, "session-tonight");
    ev.menu(
        d,
        "session-tonight: More open",
        "More Add actions",
        &["Add Travel", "Add Short rest"],
    );
    ev.menu(
        d,
        "session-tonight: Travel row menu",
        "More actions for Road to the ford",
        &["Remove this event"],
    );
    ev.menu(
        d,
        "session-tonight: fight row menu",
        "More actions for Shrine of Cinders",
        &["Remove from tonight"],
    );
    ev.hovers(d, "session-tonight");
    ev.act(d, "open the Travel row", |d| d.click("Road to the ford"));
    ev.page(d, "session-tonight: Travel row open");
    ev.act(d, "open the planned fight", |d| {
        d.click("Shrine of Cinders")
    });
    ev.page(d, "session-tonight: planned fight open");

    ev.act(d, "open last session", |d| {
        d.click_starting(&format!("Session 7: {PLAIN_LAST} ·"));
    });
    d.expect(PLAIN_PLAYED);
    ev.held(d, "session-last");
    ev.act(d, "open the played fight", |d| d.click(PLAIN_PLAYED));
    ev.page(d, "session-last: played fight open");
    ev.hovers(d, "session-last: played fight open");
    // `Whole run` opens the ended run in place, in the Encounters history: the fight that was
    // played last, then an older one, whose view says why it cannot go on.
    ev.act(d, "Whole run", |d| d.click_link("Whole run"));
    assert!(
        d.has("Continue fight"),
        "G7: the fight that was played last offers no Continue fight"
    );
    ev.page(d, "run-ended: played last");
    ev.hovers(d, "run-ended: played last");
    ev.act(d, "close the ended run", |d| {
        d.click_starting(&format!("Close the run of {PLAIN_PLAYED}"));
    });
    ev.held(d, "run-ended: closed");
    ev.act(d, "Session", |d| d.click(NAV_SESSIONS));
    ev.act(d, "open the older fight", |d| d.click(PLAIN_OLDER));
    ev.held(d, "session-last: older fight open");
    ev.act(d, "Whole run", |d| d.click_link("Whole run"));
    // The reason's own words are in the view, so G7 names any word that changes.
    assert!(
        !d.has("Continue fight"),
        "G7: the older fight offers Continue fight"
    );
    ev.page(d, "run-ended: played before");
    ev.hovers(d, "run-ended: played before");
    ev.act(d, "close the older run", |d| {
        d.click_starting(&format!("Close the run of {PLAIN_OLDER}"));
    });
    ev.held(d, "run-ended: older closed");

    ev.act(d, "Encounters", |d| d.click(NAV_ENCOUNTERS));
    ev.page(d, "encounters");
    ev.menu(
        d,
        "encounters: row menu",
        "More actions for Shrine of Cinders",
        &["Start fight"],
    );
    ev.hovers(d, "encounters");
    ev.act(d, "edit Shrine of Cinders", |d| {
        open_saved_encounter(d, "Shrine of Cinders");
    });
    ev.page(d, "encounter-editor");
    ev.menu(
        d,
        "encounter-editor: loot table menu",
        "Roll on a table for the encounter",
        &["road · 1d6 · World/Road Encounters.md"],
    );
    ev.act(d, "back to Encounters", |d| d.click("Back to encounters"));

    ev.act(d, "resume the 12-person fight", |d| {
        resume_run(d, PLAIN_RUN)
    });
    ev.page(d, "run-twelve");
    ev.hovers(d, "run-twelve");
    ev.act(d, "open Keys", |d| d.click("Keyboard commands"));
    ev.page(d, "run-keys");
    ev.act(d, "close Keys", |d| d.click("Keyboard commands"));
    ev.act(d, "select Brannoch", |d| d.click_row("Brannoch Vale"));
    ev.held(d, "run-twelve: Brannoch selected");
    ev.act(d, "Conditions", |d| d.click(CONDITIONS));
    d.expect("Other effect");
    ev.page(d, "run-conditions");
    ev.hovers(d, "run-conditions");
    ev.act(d, "Add to fight", |d| d.click("Add to fight"));
    ev.page(d, "run-arrivals");
    // A whole round, so words typed as conditions meet each turn and a new round.
    for turn in 1..=12 {
        ev.act(d, &format!("Next turn {turn}"), |d| d.click(NEXT_TURN));
    }
    ev.page(d, "run-next-round");
    ev.hovers(d, "run-next-round");
    ev.act(d, "close the run", close_run);
    ev.held(d, "run-closed");

    ev.act(d, "Timeline", |d| d.click(NAV_TIMELINE));
    ev.page(d, "timeline");

    ev.act(d, "Session", |d| d.click(NAV_SESSIONS));
    ev.held(d, "session-last: again");
    for title in PLAIN_TITLES {
        for portrait in [false, true] {
            ev.title_reader(d, title, portrait);
        }
    }
    ev.reader(d, "lookup-reader Road Encounters", |d| {
        assert!(d.app_mut().lookup.open_name("Road Encounters"));
        d.settle();
    });
    ev.lookup(d, false);
    ev.reader(d, "lookup-reader Cinder Wolf", |d| {
        d.app_mut().lookup.open_id(plain_package_creature());
        d.settle();
    });
    ev.hovers(d, "lookup-reader Cinder Wolf");
    ev.lookup(d, false);

    ev.act(d, "World", |d| d.click(NAV_WORLD));
    ev.held(d, "world-list");
    ev.act(d, "open Captain Varra Black", |d| {
        d.click_starting("Captain Varra Black");
    });
    ev.page(d, "world-reader");
    ev.hovers(d, "world-reader");

    ev.act(d, "Session", |d| d.click(NAV_SESSIONS));
    ev.act(d, "New session", |d| d.click("New session"));
    ev.held(d, "session-new: form");
    ev.act(d, "type the title", |d| {
        d.type_into("Session title", "The Shrine");
    });
    ev.held(d, "session-new: form with a title");
    ev.act(d, "Create session", |d| d.click("Create session"));
    ev.page(d, "session-new");
    ev.act(d, "write in the note", |d| {
        write_note(d);
        d.type_into_editor("\nThe party camps at the shrine.\n");
    });
    ev.held(d, "session-new: note written");
    ev.act(d, "save", |d| {
        d.press_ctrl(Key::S);
        d.app_mut().sessions.flush();
        d.app_mut().timeline.save();
        d.settle();
    });
    ev.held(d, "session-new: note saved");
    // Last, so that no old act starts on a Timeline with an event open.
    ev.act(d, "open Road to the ford on Timeline", |d| {
        d.click(NAV_TIMELINE);
        d.click_containing("Road to the ford");
    });
    ev.held(d, "timeline: Road to the ford open");
    // An ordinary journey takes its length as the DM types, before focus leaves the field, as
    // it did before encounter checks. Then the DM puts the length back.
    let mut typed = None;
    ev.act(d, "type a length for Road to the ford", |d| {
        d.replace_text_in("How long", "2 hours");
        let road = d
            .app()
            .timeline
            .events
            .iter()
            .find(|e| e.title == "Road to the ford");
        typed = Some((d.focused_name(), road.map(|e| e.duration.clone())));
        d.replace_text_in("How long", "half a day");
        d.press(Key::Enter);
        d.app_mut().timeline.save();
        d.settle();
    });
    assert_eq!(
        typed,
        Some((Some("How long".to_owned()), Some("2 hours".to_owned()))),
        "G7, section 2.5: an ordinary Travel event takes its length as the DM types it"
    );
    ev.check_seen();
    let at_end = ev.files.all();
    (ev, at_start, at_end)
}

fn compare_items(
    screen: &str,
    view: &str,
    what: &str,
    old: &[Item],
    new: &[Item],
    out: &mut Vec<Difference>,
) {
    fn group(items: &[Item]) -> BTreeMap<String, Vec<&[f32]>> {
        let mut by: BTreeMap<String, Vec<&[f32]>> = BTreeMap::new();
        for Item(name, at) in items {
            by.entry(name.clone()).or_default().push(at.as_slice());
        }
        by
    }
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
                None => out.push(difference("gone", name, format!("was {}", words(a)))),
                Some(b) if a != b => {
                    let change = if a[..4] == b[..4] {
                        "clipped"
                    } else if a[2] == b[2] && a[3] == b[3] {
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
                        format!("{} -> {}", words(a), words(b)),
                    ));
                }
                Some(_) => {}
            }
        }
        for b in now.iter().skip(was.len()) {
            out.push(difference("new", name, format!("at {}", words(b))));
        }
    }
    for (name, now) in &new {
        if !old.contains_key(name) {
            for b in now {
                out.push(difference("new", name, format!("at {}", words(b))));
            }
        }
    }
}

/// A file's lines as they are, with a carriage return shown as `\r`.
fn lines(bytes: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(bytes)
        .split('\n')
        .map(|line| line.replace('\r', "\\r"))
        .collect()
}

/// The lines that differ, each with its line number: `-3 old words` from the reference and
/// `+3 new words` from this evening, along the longest run of lines that both keep in order.
/// A line that moves is one removal and one addition, each at its place.
fn line_diff(old: &[String], new: &[String]) -> Vec<String> {
    let (n, m) = (old.len(), new.len());
    let mut kept = vec![vec![0_u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            kept[i][j] = if old[i] == new[j] {
                kept[i + 1][j + 1] + 1
            } else {
                kept[i + 1][j].max(kept[i][j + 1])
            };
        }
    }
    let (mut i, mut j, mut out) = (0, 0, Vec::new());
    while i < n || j < m {
        if i < n && j < m && old[i] == new[j] {
            i += 1;
            j += 1;
        } else if j == m || (i < n && kept[i + 1][j] >= kept[i][j + 1]) {
            out.push(format!("-{} {}", i + 1, old[i]));
            i += 1;
        } else {
            out.push(format!("+{} {}", j + 1, new[j]));
            j += 1;
        }
    }
    out
}

fn file_difference(when: &str, change: &str, path: &str, detail: String) -> Difference {
    Difference {
        screen: format!("files at the {when}"),
        view: String::new(),
        what: "file".to_owned(),
        change: change.to_owned(),
        name: path.to_owned(),
        detail,
    }
}

fn compare_files(reference: &Reference, start: &Bytes, end: &Bytes, out: &mut Vec<Difference>) {
    for (path, digest) in &reference.files_at_start {
        match start.get(path) {
            None => out.push(file_difference("start", "gone", path, String::new())),
            Some(bytes) if hex(bytes) != *digest => out.push(file_difference(
                "start",
                "bytes",
                path,
                format!("sha256 {digest} -> {}", hex(bytes)),
            )),
            Some(_) => {}
        }
    }
    for (path, bytes) in start {
        if !reference.files_at_start.contains_key(path) {
            out.push(file_difference(
                "start",
                "new",
                path,
                lines(bytes).join(" | "),
            ));
        }
    }
    for (path, kept) in &reference.files_at_end {
        // A file the evening changed is kept whole beside the reference, so it diffs by line.
        let was = if kept == SAME {
            start.get(path).cloned().unwrap_or_default()
        } else {
            let copy = fs::read(folder().join("files").join(path)).unwrap_or_default();
            assert_eq!(
                hex(&copy),
                *kept,
                "G7: plain_evening/files/{path} is not the file the reference names"
            );
            copy
        };
        match end.get(path) {
            None => out.push(file_difference("end", "gone", path, String::new())),
            Some(now) if *now != was => {
                let mut detail = line_diff(&lines(&was), &lines(now));
                if detail.is_empty() {
                    detail.push(format!("sha256 {} -> {}", hex(&was), hex(now)));
                }
                out.push(file_difference("end", "bytes", path, detail.join(" | ")));
            }
            Some(_) => {}
        }
    }
    for (path, bytes) in end {
        if !reference.files_at_end.contains_key(path) {
            out.push(file_difference(
                "end",
                "new",
                path,
                lines(bytes).join(" | "),
            ));
        }
    }
}

/// Old and new acts paired by name along the longest run that both keep in order, so that a
/// feature's new act is one new act, not a change of every act after it.
type Pair<'a> = (Option<(usize, &'a Act)>, Option<(usize, &'a Act)>);

fn align<'a>(old: &'a [Act], new: &'a [Act]) -> Vec<Pair<'a>> {
    let (n, m) = (old.len(), new.len());
    let mut kept = vec![vec![0_u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            kept[i][j] = if old[i].act == new[j].act {
                kept[i + 1][j + 1] + 1
            } else {
                kept[i + 1][j].max(kept[i][j + 1])
            };
        }
    }
    let (mut i, mut j, mut out) = (0, 0, Vec::new());
    while i < n || j < m {
        if i < n && j < m && old[i].act == new[j].act {
            out.push((Some((i, &old[i])), Some((j, &new[j]))));
            i += 1;
            j += 1;
        } else if j == m || (i < n && kept[i + 1][j] >= kept[i][j + 1]) {
            out.push((Some((i, &old[i])), None));
            i += 1;
        } else {
            out.push((None, Some((j, &new[j]))));
            j += 1;
        }
    }
    out
}

fn compare_acts(old: &[Act], new: &[Act], out: &mut Vec<Difference>) {
    for (a, b) in align(old, new) {
        let (change, at) = match (a, b) {
            (Some((_, a)), Some((_, b))) if a == b => continue,
            (Some((at, _)), Some(_)) => ("changed", at),
            (Some((at, _)), None) => ("gone", at),
            (None, Some((at, _))) => ("new", at),
            (None, None) => continue,
        };
        let (a, b) = (a.map(|(_, act)| act), b.map(|(_, act)| act));
        let mut detail = Vec::new();
        let parts: [ActPart; 3] = [
            ("appeared", |act| &act.appeared),
            ("went", |act| &act.gone),
            ("files", |act| &act.files),
        ];
        for (label, of) in parts {
            let was: BTreeSet<&String> = a.map(of).into_iter().flatten().collect();
            let now: BTreeSet<&String> = b.map(of).into_iter().flatten().collect();
            for word in now.difference(&was) {
                detail.push(format!("{label} + {word}"));
            }
            for word in was.difference(&now) {
                detail.push(format!("{label} - {word}"));
            }
        }
        out.push(Difference {
            screen: format!("act {}", at + 1),
            view: String::new(),
            what: "act".to_owned(),
            change: change.to_owned(),
            name: b.or(a).map_or_else(String::new, |act| act.act.clone()),
            detail: detail.join(" | "),
        });
    }
}

fn differences(reference: &Reference, ev: &Evening, start: &Bytes, end: &Bytes) -> Vec<Difference> {
    let mut out = Vec::new();
    for old in &reference.views {
        if let Some(new) = ev
            .views
            .iter()
            .find(|new| new.screen == old.screen && new.view == old.view)
        {
            let (screen, view) = (&old.screen, &old.view);
            compare_items(
                screen,
                view,
                "control",
                &old.controls,
                &new.controls,
                &mut out,
            );
            compare_items(screen, view, "text", &old.texts, &new.texts, &mut out);
        }
    }
    compare_acts(&reference.acts, &ev.acts, &mut out);
    compare_files(reference, start, end, &mut out);
    out
}

/// The reference, one item to a line, so that a change reads as a short diff.
fn reference_text(reference: &Reference) -> String {
    let mut out = String::from("{\n");
    let _ = writeln!(out, "\"about\": {},", json(&reference.about));
    let _ = writeln!(out, "\"frozen_at\": {},", json(&reference.frozen_at));
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
    out.push_str("],\n\"acts\": [\n");
    let acts: Vec<String> = reference.acts.iter().map(json).collect();
    out.push_str(&acts.join(",\n"));
    out.push_str("\n],\n");
    for (name, files) in [
        ("files_at_start", &reference.files_at_start),
        ("files_at_end", &reference.files_at_end),
    ] {
        let _ = writeln!(out, "\"{name}\": {{");
        let entries: Vec<String> = files
            .iter()
            .map(|(path, digest)| format!("{}: {}", json(path), json(digest)))
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

/// The views of `views`, in order, against the views G7 needs. No allowed row changes this.
fn view_faults(what: &str, views: &[View]) -> Vec<String> {
    let views: Vec<(String, String)> = views
        .iter()
        .map(|view| (view.screen.clone(), view.view.clone()))
        .collect();
    let required = required_views();
    let mut faults = Vec::new();
    for view in &required {
        if !views.contains(view) {
            faults.push(format!("{what} lacks the view {} / {}", view.0, view.1));
        }
    }
    for view in &views {
        if !required.contains(view) {
            faults.push(format!(
                "{what} has a view that G7 does not name: {} / {}",
                view.0, view.1
            ));
        }
    }
    faults
}

/// The reference, its files and `titles.tsv` go to the tree, or to `out` for `verify`.
fn write_reference(
    ev: Evening,
    start: &Bytes,
    end: &Bytes,
    frozen_at: String,
    out: Option<PathBuf>,
) {
    let reference_folder = out.clone().unwrap_or_else(folder);
    let titles_path = out.map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("artifacts/dm-audit/titles.tsv"),
        |out| out.join("titles.tsv"),
    );
    fs::create_dir_all(&reference_folder).expect("create the reference folder");
    let files = reference_folder.join("files");
    let _ = fs::remove_dir_all(&files);
    let mut at_end = BTreeMap::new();
    for (path, bytes) in end {
        if start.get(path) == Some(bytes) {
            at_end.insert(path.clone(), SAME.to_owned());
        } else {
            let copy = files.join(path);
            fs::create_dir_all(copy.parent().expect("a file has a folder"))
                .expect("create the files folder");
            fs::write(&copy, bytes).expect("keep the file");
            at_end.insert(path.clone(), hex(bytes));
        }
    }
    let reference = Reference {
        about:
            "G7 reads this with SPLAT_FROZEN=1. `devenv shell -- audit-freeze reference` writes it \
                on the commit of main in frozen_at; a feature adds its views with \
                `devenv shell -- audit-freeze add`."
                .into(),
        frozen_at,
        views: ev.views,
        acts: ev.acts,
        files_at_start: start
            .iter()
            .map(|(path, bytes)| (path.clone(), hex(bytes)))
            .collect(),
        files_at_end: at_end,
    };
    fs::write(
        reference_folder.join("reference.json"),
        reference_text(&reference),
    )
    .expect("write the reference");
    let mut titles = String::from(
        "reader\twindow\ttitle\tentry\tfont size\ttitle width\ttitle left\tart\tEdit\tscroll bar\tinside\troom after\n",
    );
    for line in &ev.titles {
        let _ = writeln!(titles, "{line}");
    }
    fs::write(titles_path, titles).expect("write titles.tsv");
}

/// The reference with this evening's new views and new acts in their places. Every record
/// that the reference holds stays as it is and where it is, also where an allowed row lets it
/// differ, so the diff of the reference is the new records only. A new view follows the view
/// that the evening took before it.
fn add_to_reference(reference: Reference, ev: &Evening) {
    let same = |a: &View, b: &View| a.screen == b.screen && a.view == b.view;
    let mut views = reference.views.clone();
    let mut last: Option<usize> = None;
    for new in &ev.views {
        match views.iter().position(|old| same(old, new)) {
            Some(at) => last = Some(at),
            None => {
                let at = last.map_or(0, |at| at + 1);
                views.insert(at, new.clone());
                last = Some(at);
            }
        }
    }
    let acts = align(&reference.acts, &ev.acts)
        .into_iter()
        .filter_map(|(old, new)| old.or(new).map(|(_, act)| act.clone()))
        .collect();
    let reference = Reference {
        views,
        acts,
        ..reference
    };
    fs::write(folder().join("reference.json"), reference_text(&reference))
        .expect("write the reference");
}

/// G7. The scenario `plain-evening`.
pub fn run(d: &mut Driver<'_>) {
    let (ev, start, end) = play(d);
    let allowed: Allowed = read(&folder().join("allowed.json"));
    let mut faults = view_faults("this evening", &ev.views);
    faults.extend(ev.faults.iter().cloned());
    for view in &ev.views {
        if view.controls.is_empty() || view.texts.is_empty() {
            faults.push(format!("{} / {} captured nothing", view.screen, view.view));
        }
    }
    if ev.acts.is_empty() || start.is_empty() || end.is_empty() {
        faults.push("the evening recorded no act or no file, so it can prove nothing".into());
    }
    let mode = std::env::var(WRITE).ok();
    if let Some(mode) = mode.as_deref() {
        assert!(
            ["all", "add"].contains(&mode),
            "G7: {WRITE} is `all` or `add`, not {mode:?}"
        );
    }
    if mode.as_deref() == Some("all") {
        assert!(
            faults.is_empty(),
            "G7: nothing written:\n{}",
            faults.join("\n")
        );
        assert!(
            allowed.rows.is_empty(),
            "G7: allowed.json has rows. They describe changes from this reference, so write it only on main with no rows"
        );
        let frozen_at = std::env::var(FROZEN_AT).unwrap_or_else(|_| {
            panic!("G7: {FROZEN_AT} names no commit: write the reference with `devenv shell -- audit-freeze reference`")
        });
        write_reference(
            ev,
            &start,
            &end,
            frozen_at,
            std::env::var_os(OUT).map(PathBuf::from),
        );
        println!("plain-evening: wrote the reference");
        return;
    }
    let adding = mode.as_deref() == Some("add");
    if !adding && std::env::var(FROZEN).as_deref() != Ok("1") {
        // Off by default: every change of the user interface would fail it.
        d.not_measured(
            "G7",
            "the plain reference is compared only with SPLAT_FROZEN=1, which the session-cycle build plan's pull requests use",
        );
        return;
    }
    let reference: Reference = read(&folder().join("reference.json"));
    // A view that SCREENS names and the reference lacks is a feature's new view: `add` writes it.
    for fault in view_faults("the reference", &reference.views) {
        if !fault.starts_with("the reference lacks the view") {
            faults.push(fault);
        } else if !adding {
            faults.push(format!("{fault}. A feature adds it with `{ADD_COMMAND}`"));
        }
    }
    if reference.acts.is_empty()
        || reference.files_at_start.is_empty()
        || reference.files_at_end.is_empty()
        || reference
            .views
            .iter()
            .any(|view| view.controls.is_empty() || view.texts.is_empty())
    {
        faults.push("the reference holds an empty view, no act or no files".into());
    }
    let mut found = differences(&reference, &ev, &start, &end);
    if adding {
        // A feature's new act is what `add` writes, with its new views.
        found.retain(|difference| !(difference.what == "act" && difference.change == "new"));
    }
    for row in &allowed.rows {
        if !CONTRACT_ROWS.contains(&row.row) {
            faults.push(format!(
                "allowed.json names row {}, which section 2.5 does not have: {}",
                row.row, row.difference
            ));
        }
        if !["control", "text", "act", "file"].contains(&row.difference.what.as_str()) {
            faults.push(format!(
                "allowed.json row {} names a {:?}: only a control, a text, an act or a file can differ",
                row.row, row.difference.what
            ));
            continue;
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
    faults.extend(found.iter().map(|difference| {
        if difference.what == "act" && difference.change == "new" {
            format!("not allowed: {difference}. A feature adds its acts with `{ADD_COMMAND}`")
        } else {
            format!("not allowed: {difference}")
        }
    }));
    if adding {
        assert!(
            faults.is_empty(),
            "G7: `{ADD_COMMAND}` writes new views and new acts only, and the evening differs from the \
             reference where no row of allowed.json says so, so nothing is written:\n{}",
            faults.join("\n")
        );
        add_to_reference(reference, &ev);
        println!("plain-evening: added the new views and acts to the reference");
        return;
    }
    assert!(
        faults.is_empty(),
        "G7 (reference frozen at main {}): {} differences from the plain reference. First: {}\n\
         If the reference is older than this branch's base, write it again at the base with \
         `devenv shell -- audit-freeze reference` as one commit. A feature's own change needs a row \
         in allowed.json; its new view, `devenv shell -- audit-freeze add`.\nAll:\n{}",
        reference.frozen_at,
        faults.len(),
        faults[0],
        faults.join("\n")
    );
    d.observe(
        "G7",
        true,
        format!(
            "{} views, {} acts and {} files match the reference frozen at main {}, with {} allowed rows",
            ev.views.len(),
            ev.acts.len(),
            end.len(),
            reference.frozen_at,
            allowed.rows.len()
        ),
    );
}

/// Each row of `allowed.json` is in a named class of its row of section 2.5. A class lets a new
/// thing appear and an old one move, never go, be cut or get narrower. A row that fits no
/// class fails, so the list cannot allow a fault by copying the failures. Each pull request of
/// the build plan keeps its own classes here: P5 the Travel menu (row 12), P2 the difficulty
/// word (rows 1 to 3), P7 `Add rewards` (row 10).
#[cfg(test)]
mod allowed_classes {
    use super::*;

    /// A class of one pull request's rows. `fits` reads the difference, and may read the other
    /// rows: a move proves itself against the other moves of its view.
    struct Class {
        row: u32,
        name: &'static str,
        fits: fn(&Difference, &[AllowedRow]) -> bool,
    }

    const TRAVEL_MENU: &str = "session-tonight: Travel row menu";
    /// One line of a row's menu, as the reference measures it.
    const MENU_LINE: f32 = 36.0;

    const CLASSES: &[Class] = &[
        // P5, F9. Its reference at 993aa81f holds this line, so no row of it is left; the
        // classes and their checks below stay for the next reference that does not.
        Class {
            row: 12,
            name: "P5: the new line in a Travel row's open menu",
            fits: |d, _| new_travel_menu_line(d),
        },
        Class {
            row: 12,
            name: "P5: `Remove` one line lower in that menu",
            fits: |d, _| remove_one_line_lower(d),
        },
        // P3, F3 (the review of PR 261 named the first five).
        Class {
            row: 14,
            name: "P3 a: an unchanged item down by the scenes' height",
            fits: scenes::moved_down,
        },
        Class {
            row: 14,
            name: "P3 b: new scene content on the Session page",
            fits: scenes::scene_content,
        },
        Class {
            row: 7,
            name: "P3 c: `Scene` in the More menu",
            fits: |d, _| scenes::scene_line(d),
        },
        Class {
            row: 9,
            name: "P3 d: the reader's `Start fight`",
            fits: |d, _| scenes::reader_start(d),
        },
        Class {
            row: 21,
            name: "P3 d: the reader's `Start fight`",
            fits: |d, _| scenes::reader_start(d),
        },
        Class {
            row: 13,
            name: "P3 e: `Remove…` in a Ready row's menu",
            fits: |d, _| scenes::remove_encounter(d),
        },
        Class {
            row: 14,
            name: "P3 f: the fold, `N scenes` on the Tonight line",
            fits: |d, _| scenes::fold(d),
        },
        Class {
            row: 14,
            name: "P3 n: the Tonight count, with its pair",
            fits: scenes::count,
        },
        Class {
            row: 14,
            name: "P3 r: Read leaves out the plan section",
            fits: |d, _| scenes::read_leaves_out(d),
        },
        Class {
            row: 14,
            name: "P3 t: an act's words of these classes",
            fits: |d, _| scenes::act_words(d),
        },
        Class {
            row: 7,
            name: "P3 t: an act's words of these classes",
            fits: |d, _| scenes::act_words(d),
        },
        Class {
            row: 9,
            name: "P3 t: an act's words of these classes",
            fits: |d, _| scenes::act_words(d),
        },
        Class {
            row: 13,
            name: "P3 t: an act's words of these classes",
            fits: |d, _| scenes::act_words(d),
        },
        // P6, F6. Row 8: `Add to Log` next to Edit, with the reader's `Start fight` after it.
        Class {
            row: 8,
            name: "P6 a: the reader's new Add to Log, its word or its icon",
            fits: log::add_to_log,
        },
        Class {
            row: 8,
            name: "P6 b: the Add icon, matched by its order",
            fits: log::icon_by_order,
        },
        Class {
            row: 8,
            name: "P6 d: an act shows or hides Add to Log",
            fits: |d, _| log::act_words(d),
        },
        // P8, F5.
        Class {
            row: 16,
            name: "P8 a: the Last time preview reads What happened, in place, one to one",
            fits: carry::preview,
        },
        Class {
            row: 16,
            name: "P8 a: an act shows or hides only the preview's words",
            fits: |d, _| carry::preview_act(d),
        },
        Class {
            row: 11,
            name: "P8 b: `Carry over` ends the new session's Last time line",
            fits: |d, _| carry::carry_over(d),
        },
        Class {
            row: 11,
            name: "P8 c: an act shows `Carry over` and nothing else",
            fits: |d, _| carry::act(d),
        },
        // R1c, row 22: History scrolls once to the run it opens (V31), with V11's line.
        Class {
            row: 2,
            name: "R1c c: the opened row's words start at the link's right edge (V9)",
            fits: history::words_at_the_link,
        },
        Class {
            row: 22,
            name: "R1c a: History scrolled once to the opened run, and V11's line under Read-only",
            fits: history::scrolled,
        },
        Class {
            row: 22,
            name: "R1c b: an act's words that scrolled out of or into History's view",
            fits: history::act_words,
        },
        // P7, F7.
        Class {
            row: 10,
            name: "P7 a: `Add rewards` new on a played row, or as the opened row's last link",
            fits: |d, _| rewards::action(d),
        },
        Class {
            row: 10,
            name: "P7 b: its word and its `·`, inside or just before a new action of its view",
            fits: rewards::words,
        },
        Class {
            row: 10,
            name: "P7 c: an act's words of these classes",
            fits: |d, _| rewards::act_words(d),
        }, // R1a, rows 23 and 24 (by the lead's ruling after app round 1): a scene's row matches a
        // fight row (V2, V8), and the folded scenes' link has the section caret (V14).
        Class {
            row: 23,
            name: "R1a a: a scene's own part at the title column and the title's size",
            fits: scene_rows::part,
        },
        Class {
            row: 23,
            name: "R1a b: an unchanged item below changed scene rows, straight up by their change",
            fits: scene_rows::below,
        },
        Class {
            row: 23,
            name: "R1a c: a done scene's `Add to Log` on its row, where its words now keep their rows",
            fits: scene_rows::log_on_row,
        },
        Class {
            row: 24,
            name: "R1a d: the section caret before `N scenes`",
            fits: scene_rows::caret,
        },
        // P4, F2: row 6. The dock's `Effect` is new, in the room after Absent; nothing of
        // today's dock moves, so no class here allows a move.
        Class {
            row: 6,
            name: "P4 a: the dock's new `Effect`",
            fits: |d, _| effect::control(d),
        },
        Class {
            row: 6,
            name: "P4 b: `Effect`'s own words, inside it",
            fits: effect::words,
        },
        Class {
            row: 6,
            name: "P4 c: an act shows or hides only `Effect`'s words",
            fits: |d, _| effect::act_words(d),
        },
        // R3h, after app round 2: row 25, Lookup's reader of a campaign NPC has the Status row
        // with its Change; row 26, `Add to Log` has its own glyph.
        Class {
            row: 25,
            name: "R3h a: the Status row's words and Change in Lookup's NPC reader",
            fits: status_row::part,
        },
        Class {
            row: 25,
            name: "R3h b: an unchanged item below the Status row, straight down by its height",
            fits: status_row::below,
        },
        Class {
            row: 25,
            name: "R3h c: an act shows or hides only the Status row's words and Add to Log's glyph",
            fits: |d, _| status_row::act_words(d),
        },
        Class {
            row: 26,
            name: "R3h d: Add to Log's `+` becomes its list glyph at the same place; other `+` stay",
            fits: log_glyph::glyph,
        },
        Class {
            row: 26,
            name: "R3h e: an act shows or hides only Add to Log's glyph",
            fits: |d, _| log_glyph::act_words(d),
        },
    ];

    /// R3h's row 25: the Status row that Lookup's reader of a campaign NPC now shows, with its
    /// Change. Its words are new; every item below moves straight down by one height for its
    /// view, the same for all, with no move sideways and no change of size; nothing goes.
    mod status_row {
        use super::super::{AllowedRow, Difference};
        use super::{rect_of, rects};

        const CHANGE: &str = "Button: Change Status";
        /// The row's words: its label, the plain campaign's status, and Change.
        const PARTS: [&str; 8] = [
            CHANGE,
            "Label: Status",
            "TextRun: Status",
            "Label: alive",
            "TextRun: alive",
            "Status",
            "alive",
            "Change",
        ];

        fn same_view<'a>(
            d: &'a Difference,
            rows: &'a [AllowedRow],
        ) -> impl Iterator<Item = &'a Difference> {
            rows.iter()
                .filter(|row| row.row == 25)
                .map(|row| &row.difference)
                .filter(move |other| other.screen == d.screen && other.view == d.view)
        }

        /// The view's one downward shift, and the new Status label's left and top, when the
        /// view has the new Change and Status label and every move in it is that one straight
        /// shift of an item that stood at or below the label's top, in the reader's column (not
        /// left of the label): so the title, the kind line and the page beside Lookup never
        /// move under this class (Astra's review of PR 287).
        fn shift(d: &Difference, rows: &[AllowedRow]) -> Option<(f32, f32, f32)> {
            if !d.screen.starts_with("lookup-reader ")
                || !same_view(d, rows).any(|o| o.change == "new" && o.name == CHANGE)
            {
                return None;
            }
            let [left, top, ..] = same_view(d, rows)
                .find(|o| o.what == "control" && o.change == "new" && o.name == "Label: Status")
                .and_then(|o| rect_of(&o.detail, "at "))?;
            let moves: Vec<_> = same_view(d, rows)
                .filter(|o| o.change == "moved")
                .map(|o| rects(&o.detail))
                .collect::<Option<_>>()?;
            let (old, new) = moves.first()?;
            let dy = new[1] - old[1];
            let straight = moves.iter().all(|(old, new)| {
                (new[0] - old[0]).abs() < 0.05
                    && (new[2] - old[2]).abs() < 0.05
                    && (new[3] - old[3]).abs() < 0.05
                    && (new[1] - old[1] - dy).abs() < 0.05
                    && old[1] >= top - 0.5
                    && old[0] >= left - 0.5
            });
            (straight && dy > 0.0 && dy <= 64.0).then_some((dy, top, left))
        }

        pub(super) fn below(d: &Difference, rows: &[AllowedRow]) -> bool {
            ["control", "text"].contains(&d.what.as_str())
                && d.change == "moved"
                && shift(d, rows).is_some()
        }

        /// A word of the Status row, new, in the band that the shift opened from the label's
        /// line: a control's target may start `S` above the label's words.
        pub(super) fn part(d: &Difference, rows: &[AllowedRow]) -> bool {
            let Some((dy, top, left)) = shift(d, rows) else {
                return false;
            };
            ["control", "text"].contains(&d.what.as_str())
                && d.change == "new"
                && PARTS.contains(&d.name.as_str())
                && rect_of(&d.detail, "at ").is_some_and(|[x, y, _, h]| {
                    x >= left - 0.5 && y >= top - crate::design::S - 0.5 && y + h <= top + dy + 0.5
                })
        }

        pub(super) fn act_words(d: &Difference) -> bool {
            let words = ["Change", "Status", "alive", "\u{e2f8}"];
            d.what == "act"
                && d.change == "changed"
                && d.detail
                    .split(" | ")
                    .any(|part| part.ends_with(" + Status"))
                && d.detail.split(" | ").all(|part| {
                    ["appeared + ", "went + "].iter().any(|verb| {
                        part.strip_prefix(verb)
                            .is_some_and(|word| words.contains(&word))
                    })
                })
        }
    }

    /// R3h's row 26: the `+` (`icons::ADD`) of the reader's `Add to Log` becomes its list glyph
    /// (`icons::ADD_TO_LOG`) at the same place. G7 pairs a view's `+` glyphs in order, so taking
    /// one away shows as a chain of moves and one loss: the class takes them only when the
    /// places that the `+` glyphs leave, less those they take, are exactly the new glyph's one
    /// place, at one size. So every other `+` keeps its place.
    mod log_glyph {
        use super::super::{AllowedRow, Difference, Reference, folder, read};
        use super::{rect_of, rects};

        const ADD: &str = "\u{e3d4}";
        const ADD_TO_LOG: &str = "\u{e2f8}";

        /// The frozen reference, read once.
        fn reference() -> &'static Reference {
            static REFERENCE: std::sync::OnceLock<Reference> = std::sync::OnceLock::new();
            REFERENCE.get_or_init(|| read(&folder().join("reference.json")))
        }

        /// The reference's `Add to Log` in this view holds this place: the glyph is that
        /// action's, not another action's `+` (Astra's review of PR 287).
        fn in_add_to_log(screen: &str, view: &str, at: &[f32; 4]) -> bool {
            reference()
                .views
                .iter()
                .filter(|v| v.screen == screen && v.view == view)
                .flat_map(|v| &v.controls)
                .any(|control| {
                    control.0 == "Button: Add to Log"
                        && control.1.len() >= 4
                        && at[0] >= control.1[0] - 0.05
                        && at[1] >= control.1[1] - 0.05
                        && at[0] + at[2] <= control.1[0] + control.1[2] + 0.05
                        && at[1] + at[3] <= control.1[1] + control.1[3] + 0.05
                })
        }

        pub(super) fn glyph(d: &Difference, rows: &[AllowedRow]) -> bool {
            let ours = |o: &Difference| {
                o.what == "text"
                    && o.screen == d.screen
                    && o.view == d.view
                    && ((o.name == ADD && ["moved", "gone"].contains(&o.change.as_str()))
                        || (o.name == ADD_TO_LOG && o.change == "new"))
            };
            if !ours(d) {
                return false;
            }
            let (mut left, mut taken, mut new) = (Vec::new(), Vec::new(), Vec::new());
            for o in rows
                .iter()
                .filter(|row| row.row == 26)
                .map(|row| &row.difference)
            {
                if !ours(o) {
                    continue;
                }
                match o.change.as_str() {
                    "moved" => match rects(&o.detail) {
                        Some((old, to)) => {
                            left.push(old);
                            taken.push(to);
                        }
                        None => return false,
                    },
                    "gone" => match rect_of(&o.detail, "was ") {
                        Some(old) => left.push(old),
                        None => return false,
                    },
                    _ => match rect_of(&o.detail, "at ") {
                        Some(at) => new.push(at),
                        None => return false,
                    },
                }
            }
            let near =
                |a: &[f32; 4], b: &[f32; 4]| a.iter().zip(b).all(|(a, b)| (a - b).abs() < 0.05);
            for to in &taken {
                match left.iter().position(|old| near(old, to)) {
                    Some(at) => {
                        left.remove(at);
                    }
                    None => return false,
                }
            }
            new.len() == 1
                && left.len() == 1
                && near(&left[0], &new[0])
                && in_add_to_log(&d.screen, &d.view, &new[0])
        }

        pub(super) fn act_words(d: &Difference) -> bool {
            d.what == "act"
                && d.change == "changed"
                && ["appeared + \u{e2f8}", "went + \u{e2f8}"].contains(&d.detail.as_str())
        }
    }

    /// P4's classes: the dock's `Effect`, its icon, word and key letter, and acts that show or
    /// hide only those. Each is new; none lets an item of today go, move, narrow or be cut.
    mod effect {
        use super::super::{AllowedRow, Difference};
        use super::rect_of;

        const NAME: &str = "Button: Effect: words for this row";
        const WORDS: [&str; 3] = ["\u{e3e2}", "Effect", "E"];

        fn run_view(d: &Difference) -> bool {
            d.screen.starts_with("run-")
        }

        pub(super) fn control(d: &Difference) -> bool {
            run_view(d) && d.what == "control" && d.change == "new" && d.name == NAME
        }

        /// A word of `Effect` that lies inside a new `Effect` of the same view.
        pub(super) fn words(d: &Difference, rows: &[AllowedRow]) -> bool {
            let Some(at) = rect_of(&d.detail, "at ") else {
                return false;
            };
            run_view(d)
                && d.what == "text"
                && d.change == "new"
                && WORDS.contains(&d.name.as_str())
                && rows.iter().any(|row| {
                    let o = &row.difference;
                    control(o)
                        && (&o.screen, &o.view) == (&d.screen, &d.view)
                        && rect_of(&o.detail, "at ").is_some_and(|button| {
                            at[0] >= button[0]
                                && at[1] >= button[1]
                                && at[0] + at[2] <= button[0] + button[2]
                                && at[1] + at[3] <= button[1] + button[3]
                        })
                })
        }

        /// Every part of the act is a word of `Effect` that this evening shows or hides.
        pub(super) fn act_words(d: &Difference) -> bool {
            d.what == "act"
                && d.change == "changed"
                && d.detail.split(" | ").all(|part| {
                    ["appeared + ", "went + "]
                        .iter()
                        .find_map(|prefix| part.strip_prefix(prefix))
                        .is_some_and(|word| WORDS.contains(&word))
                })
        }
    }

    /// P6's classes. Each proves itself from the geometry of its view: Add to Log is new only in a
    /// reader, after what the header shows today; a glyph matched by its order pairs with a new
    /// glyph at its old place. Nothing of today goes, moves, is cut, or narrows: `Start fight`
    /// stays next to Edit.
    mod log {
        use super::super::{AllowedRow, Difference};
        use super::{rect_of, rects};

        const ADD: &str = "\u{e3d4}";
        const BUTTON: &str = "Button: Add to Log";

        fn reader(d: &Difference) -> bool {
            d.screen.starts_with("world-reader") || d.screen.starts_with("lookup-reader ")
        }

        fn same_view(o: &Difference, d: &Difference) -> bool {
            (&o.screen, &o.view) == (&d.screen, &d.view)
        }

        fn inside(r: [f32; 4], outer: [f32; 4]) -> bool {
            outer[0] <= r[0]
                && r[0] + r[2] <= outer[0] + outer[2] + 0.05
                && outer[1] <= r[1]
                && r[1] + r[3] <= outer[1] + outer[3] + 0.05
        }

        /// The new Add to Log targets of `d`'s view.
        fn buttons<'a>(
            d: &'a Difference,
            rows: &'a [AllowedRow],
        ) -> impl Iterator<Item = [f32; 4]> + 'a {
            rows.iter()
                .map(|row| &row.difference)
                .filter(move |o| {
                    same_view(o, d) && o.what == "control" && o.change == "new" && o.name == BUTTON
                })
                .filter_map(|o| rect_of(&o.detail, "at "))
        }

        pub(super) fn add_to_log(d: &Difference, rows: &[AllowedRow]) -> bool {
            if !reader(d) || d.change != "new" {
                return false;
            }
            let Some(at) = rect_of(&d.detail, "at ") else {
                return false;
            };
            match (d.what.as_str(), d.name.as_str()) {
                // Its word, or its icon alone: the height of a reader's header action.
                ("control", BUTTON) => (at[3] - 28.0).abs() < 0.05,
                ("text", "Add to Log" | ADD) => {
                    buttons(d, rows).any(|button| inside(at, button))
                        || (d.name == ADD && moved_away_from(d, at, rows))
                }
                _ => false,
            }
        }

        /// A glyph of today matched by its order to the new icon left a new glyph at its old
        /// place: that is the glyph of today, which did not move.
        fn moved_away_from(d: &Difference, at: [f32; 4], rows: &[AllowedRow]) -> bool {
            rows.iter().map(|row| &row.difference).any(|o| {
                same_view(o, d)
                    && o.what == "text"
                    && o.change == "moved"
                    && o.name == ADD
                    && rects(&o.detail).is_some_and(|(was, now)| {
                        was == at && buttons(d, rows).any(|button| inside(now, button))
                    })
            })
        }

        pub(super) fn icon_by_order(d: &Difference, rows: &[AllowedRow]) -> bool {
            reader(d)
                && d.what == "text"
                && d.change == "moved"
                && d.name == ADD
                && rects(&d.detail).is_some_and(|(was, now)| {
                    buttons(d, rows).any(|button| inside(now, button))
                        && rows.iter().map(|row| &row.difference).any(|o| {
                            same_view(o, d)
                                && o.what == "text"
                                && o.change == "new"
                                && o.name == ADD
                                && rect_of(&o.detail, "at ") == Some(was)
                        })
                })
        }

        pub(super) fn act_words(d: &Difference) -> bool {
            d.what == "act"
                && d.change == "changed"
                && d.detail
                    .split(" | ")
                    .all(|part| ["appeared + Add to Log", "went + Add to Log"].contains(&part))
        }
    }

    /// P8's classes. Row 16: the Last time preview's words go from the last note's recap to its
    /// `What happened`, at the same place and height, each old form with its own new form. Row
    /// 11: `Carry over` is new at the end of the new session's Last time line, at the line's
    /// top and height and inside the window. No class lets another text or control change.
    mod carry {
        use super::super::{AllowedRow, Difference};

        /// The plain campaign's Session 7, as its note holds them.
        const RECAP: &str = "The party left Emberfall at dawn with the miller's warning.";
        const WHAT_HAPPENED: &str =
            "Wolves at the ford. Hobb said the toll collectors were not paid.";
        /// The Last time link's hover begins with these words.
        const HOVER: &str = "Last time: Session 7: The Road to Emberfall — ";
        /// The Last time line's top, with Lookup closed and open (the reference).
        const LINE_TOPS: [f32; 2] = [140.0, 174.0];
        /// The widest word and its ellipsis that a cut at `Small` drops (the reference's cuts
        /// differ by at most 8 px).
        const CUT_WORD: f32 = 40.0;

        fn bare(name: &str) -> &str {
            name.strip_prefix("Label: ")
                .or_else(|| name.strip_prefix("TextRun: "))
                .unwrap_or(name)
        }

        /// `at x y wxh` or `was x y wxh`.
        fn rect(detail: &str) -> Option<[f32; 4]> {
            let mut words = detail
                .strip_prefix("at ")
                .or_else(|| detail.strip_prefix("was "))?
                .split_whitespace();
            let x = words.next()?.parse().ok()?;
            let y = words.next()?.parse().ok()?;
            let (w, h) = words.next()?.split_once('x')?;
            Some([x, y, w.parse().ok()?, h.parse().ok()?])
        }

        /// The preview or the hover, with `full`'s words, whole or cut to an ellipsis.
        fn words_of(name: &str, full: &str) -> bool {
            let name = bare(name);
            name.strip_prefix("— ")
                .or_else(|| name.strip_prefix(HOVER))
                .is_some_and(|words| {
                    words == full
                        || words
                            .strip_suffix('…')
                            .is_some_and(|cut| !cut.is_empty() && full.starts_with(cut))
                })
        }

        /// The kind of node: `Label: `, `TextRun: ` or a painted text.
        fn kind(name: &str) -> &str {
            &name[..name.len() - bare(name).len()]
        }

        pub(super) fn preview(d: &Difference, rows: &[AllowedRow]) -> bool {
            if !d.screen.starts_with("session-tonight")
                || !["control", "text"].contains(&d.what.as_str())
            {
                return false;
            }
            let (own, other, partner) = match d.change.as_str() {
                "gone" => (RECAP, WHAT_HAPPENED, "new"),
                "new" => (WHAT_HAPPENED, RECAP, "gone"),
                _ => return false,
            };
            let Some(at) = rect(&d.detail) else {
                return false;
            };
            if !words_of(&d.name, own) {
                return false;
            }
            // At the same place and height, each form pairs with one of the other.
            let here = |o: &Difference, change: &str, words: &str| {
                (&o.screen, &o.view, &o.what) == (&d.screen, &d.view, &d.what)
                    && o.change == change
                    && kind(&o.name) == kind(&d.name)
                    && words_of(&o.name, words)
                    && rect(&o.detail)
                        .is_some_and(|r| r[0] == at[0] && r[1] == at[1] && r[3] == at[3])
            };
            let mine = rows
                .iter()
                .filter(|row| here(&row.difference, &d.change, own))
                .count();
            let theirs = rows
                .iter()
                .filter(|row| here(&row.difference, partner, other))
                .count();
            // These screens show no `Carry over`, so a cut new form has today's room: it may be
            // narrower than the old form only by the word and the ellipsis that the cut drops.
            let widths = |change: &str, words: &str| {
                rows.iter()
                    .map(|row| &row.difference)
                    .filter(|o| here(o, change, words))
                    .filter_map(|o| Some((o.name.ends_with('…'), rect(&o.detail)?[2])))
                    .collect::<Vec<_>>()
            };
            let old = widths("gone", RECAP);
            let in_room = widths("new", WHAT_HAPPENED)
                .iter()
                .all(|(cut, new)| !cut || old.iter().all(|(_, old)| old - new <= CUT_WORD));
            mine == theirs && mine > 0 && in_room
        }

        /// An act's parts are only the preview: its new words in this record (`+`), and its old
        /// words in the reference's (`-`).
        pub(super) fn preview_act(d: &Difference) -> bool {
            d.what == "act"
                && d.change == "changed"
                && d.detail.split(" | ").all(|part| {
                    ["appeared ", "went "]
                        .iter()
                        .find_map(|lead| part.strip_prefix(lead))
                        .is_some_and(|part| {
                            part.strip_prefix("+ ")
                                .is_some_and(|words| words_of(words, WHAT_HAPPENED))
                                || part
                                    .strip_prefix("- ")
                                    .is_some_and(|words| words_of(words, RECAP))
                        })
                })
        }

        pub(super) fn carry_over(d: &Difference) -> bool {
            let width: f32 = d
                .view
                .split('x')
                .next()
                .and_then(|w| w.parse().ok())
                .unwrap_or(0.0);
            let Some([x, y, w, h]) = rect(&d.detail) else {
                return false;
            };
            let (top, height) = match (d.what.as_str(), d.name.as_str()) {
                ("control", "Button: Carry over") => (y, h),
                // The words sit inside the button, 3 px down at 18 px.
                ("text", "Carry over") => (y - 3.0, h + 6.0),
                _ => return false,
            };
            d.screen.starts_with("session-new")
                && d.change == "new"
                && LINE_TOPS.contains(&top)
                && (height - crate::design::XL).abs() < 0.05
                && x + w <= width
        }

        pub(super) fn act(d: &Difference) -> bool {
            d.what == "act" && d.change == "changed" && d.detail == "appeared + Carry over"
        }
    }

    /// R1c's classes, row 22. When a run opens in History, the list scrolls once so that the
    /// run's row is at its top (V31), and the line under Read-only is `S` below it and `M`
    /// above `FINAL STATE` (V11). Each class proves itself from all the rows of its view, name
    /// by name, because G7 pairs the items of one name by their order: an item that scrolled
    /// out can show as another item of its name that moved.
    mod history {
        use super::super::{AllowedRow, Difference};
        use std::collections::BTreeMap;

        /// Rectangles `x y w h` of the items of one name.
        type Places = Vec<[f32; 4]>;

        /// The acts that open, show or close a run in History.
        const ACTS: [&str; 5] = [
            "Whole run",
            "Ctrl+K opens Lookup",
            "Ctrl+K closes Lookup",
            "close the ended run",
            "close the older run",
        ];
        /// V11 moves what is under Read-only up by at most this much more than the scroll.
        const V11: f32 = 8.0;
        /// The Read-only line's words were as tall as a control, and are as tall as a line.
        const OLD_LINE: f32 = 26.0;
        const NEW_LINE: f32 = 16.0;

        /// History with an opened run, and the hovers taken there.
        fn screen(d: &Difference) -> bool {
            d.screen.starts_with("run-ended: played ")
        }

        /// `x y w h` of `x y wxh` or `x y wxh, seen …`.
        fn numbers(text: &str) -> Option<[f32; 4]> {
            let whole = text.split(',').next()?.replace('x', " ");
            let numbers: Vec<f32> = whole
                .split_whitespace()
                .map(|n| n.parse().ok())
                .collect::<Option<_>>()?;
            numbers.try_into().ok()
        }

        /// The item after, as written: `x y wxh` with its `seen` part when only part shows.
        fn after_text(d: &Difference) -> Option<&str> {
            match d.change.as_str() {
                "gone" => None,
                "new" => d.detail.strip_prefix("at "),
                _ => Some(d.detail.split_once(" -> ")?.1),
            }
        }

        /// The part of the item in view after: its `seen` part, or the whole item.
        fn shown_after(d: &Difference) -> Option<[f32; 4]> {
            let after = after_text(d)?;
            match after.split_once(", seen ") {
                Some((_, seen)) => numbers(seen),
                None => numbers(after),
            }
        }

        /// The item's place before and after.
        fn places(d: &Difference) -> (Option<[f32; 4]>, Option<[f32; 4]>) {
            match d.change.as_str() {
                "gone" => (d.detail.strip_prefix("was ").and_then(numbers), None),
                "new" => (None, d.detail.strip_prefix("at ").and_then(numbers)),
                _ => d
                    .detail
                    .split_once(" -> ")
                    .map_or((None, None), |(a, b)| (numbers(a), numbers(b))),
            }
        }

        /// How far the list scrolled in a view: how far the opened run's own row moved, with
        /// its place across and its size kept.
        fn scroll(d: &Difference, rows: &[AllowedRow]) -> Option<f32> {
            rows.iter()
                .map(|row| &row.difference)
                .filter(|o| (&o.screen, &o.view) == (&d.screen, &d.view))
                .filter(|o| o.what == "control" && o.change == "moved")
                .filter(|o| o.name.starts_with("Button: Close the run of "))
                .find_map(|o| match places(o) {
                    (Some(a), Some(b)) if a[0] == b[0] && a[2..] == b[2..] => Some(b[1] - a[1]),
                    _ => None,
                })
        }

        /// Where the bands of a History view start in its frozen view, for items of `d`'s
        /// kind: the middle of the Read-only line, and the top of `FINAL STATE`.
        fn marks(d: &Difference) -> Option<(f32, f32)> {
            let view = reference()
                .views
                .iter()
                .find(|view| (&view.screen, &view.view) == (&d.screen, &d.view))?;
            let items = if d.what == "control" {
                &view.controls
            } else {
                &view.texts
            };
            let find = |end: &str| items.iter().find(|item| item.0.ends_with(end));
            let line = find("Read-only").map(|item| item.1[1] + item.1[3] / 2.0)?;
            Some((line, find("FINAL STATE")?.1[1]))
        }

        /// An item's band, by where it was: 0 the list above the Read-only line, 1 that line,
        /// 2 the line under it (Continue fight or its reason), 3 `FINAL STATE` and below.
        fn band(a: &[f32; 4], (line, last): (f32, f32)) -> usize {
            let middle = a[1] + a[3] / 2.0;
            if (middle - line).abs() < 0.5 {
                1
            } else if middle < line {
                0
            } else if a[1] < last {
                2
            } else {
                3
            }
        }

        /// The decisions' rule, proved for the whole view of `d`. The list scrolled by one
        /// offset, the run's own row's, and V11 gave the line under Read-only `S` above and
        /// `M` below. So each item of today is one to one an item after (G7 pairs the items
        /// of one name by their order, so each name pairs by place, less what scrolled out at
        /// the top or in from below), with its place across and its size kept, and every
        /// item of one band moves by the SAME amount: the list and the Read-only line's top by
        /// the scroll alone (only that line's own height changes, 26 to 16 px); the line under
        /// it, and everything from `FINAL STATE` down, each by one shift of at most V11 more.
        /// The bands keep their order and do not overlap. An item after may be cut only by the
        /// list's own edges.
        pub(super) fn scrolled(d: &Difference, rows: &[AllowedRow]) -> bool {
            if !screen(d) || !["control", "text"].contains(&d.what.as_str()) {
                return false;
            }
            let (Some(dy), Some(marks)) = (scroll(d, rows), marks(d)) else {
                return false;
            };
            if places(d) == (None, None) {
                return false;
            }
            let view: Vec<&Difference> = rows
                .iter()
                .map(|row| &row.difference)
                .filter(|o| (&o.screen, &o.view, &o.what) == (&d.screen, &d.view, &d.what))
                .collect();
            let pairs: Vec<_> = view
                .iter()
                .filter_map(|o| match (places(o).0, shown_after(o)) {
                    (Some(a), Some(shown)) => Some((a, shown)),
                    _ => None,
                })
                .collect();
            // The edges of what stayed in view: the top of what shows after, and the bottom
            // before.
            let top = pairs
                .iter()
                .map(|(_, b)| b[1])
                .fold(f32::INFINITY, f32::min);
            let bottom = pairs
                .iter()
                .map(|(a, _)| a[1] + a[3])
                .fold(f32::NEG_INFINITY, f32::max);
            // An item after may be cut only by the list's own edges, top or bottom: never
            // across, and never inside the view.
            let edge = view
                .iter()
                .filter_map(|o| shown_after(o))
                .map(|b| b[1] + b[3])
                .fold(f32::NEG_INFINITY, f32::max);
            let whole_across = |o: &Difference| {
                let (Some(whole), Some(shown)) = (places(o).1, shown_after(o)) else {
                    return true;
                };
                let cut_top = shown[1] > whole[1] + 0.05;
                let cut_bottom = shown[1] + shown[3] < whole[1] + whole[3] - 0.05;
                shown[0] == whole[0]
                    && shown[2] == whole[2]
                    && (!cut_top || (shown[1] - top).abs() < 0.5)
                    && (!cut_bottom || (shown[1] + shown[3] - edge).abs() < 0.5)
            };
            // Each name's items, today's and after, less what scrolled out of or into view.
            let mut names: BTreeMap<&str, (Places, Places)> = BTreeMap::new();
            for o in &view {
                if !whole_across(o) {
                    return false;
                }
                let (a, b) = places(o);
                let (before, after) = names.entry(o.name.as_str()).or_default();
                before.extend(a.filter(|a| a[1] + dy + a[3] > top + 0.5));
                after.extend(b.filter(|b| b[1] - dy + V11 < bottom - 0.5));
            }
            // Each band's one shift beyond the scroll, and its extent after.
            let line_top = marks.0 - OLD_LINE / 2.0;
            let mut shift = [Some(0.0), Some(0.0), None, None];
            let mut extent = [(f32::INFINITY, f32::NEG_INFINITY); 4];
            for (before, after) in names.values_mut() {
                if before.len() != after.len() {
                    return false;
                }
                before.sort_by(|a, b| a[1].total_cmp(&b[1]));
                after.sort_by(|a, b| a[1].total_cmp(&b[1]));
                for (a, b) in before.iter().zip(after.iter()) {
                    let band = band(a, marks);
                    // The Read-only line's items go to the line's top, which only scrolled.
                    let up = b[1] - if band == 1 { line_top } else { a[1] } - dy;
                    let height =
                        a[3] == b[3] || (band == 1 && a[3] == OLD_LINE && b[3] == NEW_LINE);
                    if a[0] != b[0] || a[2] != b[2] || !height {
                        return false;
                    }
                    match shift[band] {
                        Some(shift) if (shift - up).abs() > 0.05 => return false,
                        _ => shift[band] = Some(up),
                    }
                    extent[band] = (extent[band].0.min(b[1]), extent[band].1.max(b[1] + b[3]));
                }
            }
            let order = extent.iter().filter(|(top, _)| top.is_finite());
            shift
                .iter()
                .flatten()
                .all(|shift| (-V11 - 0.05..=0.05).contains(shift))
                && order
                    .clone()
                    .zip(order.skip(1))
                    .all(|(upper, lower)| upper.1 <= lower.0 + 0.05)
        }

        /// One blank of the opened row's words between two glyphs' boxes:
        /// `read_encounter_has_one_blank_each_side_of_each_separator` measures 3.44 px.
        const BLANK: f32 = 3.5;

        /// V9, row 2: F1's words after `Read encounter` start at the link's right edge. They
        /// keep their line, their height and their visible part (the whole item, or cut only
        /// where today's is cut). Their text run keeps its width; the painted words grow by
        /// at most the one blank that the justified layout took from a separator, and never
        /// past their text run. The text run and the painted words start at one x in the view.
        pub(super) fn words_at_the_link(d: &Difference, rows: &[AllowedRow]) -> bool {
            let bare = |o: &Difference| {
                o.name
                    .trim_start_matches("Label: ")
                    .trim_start_matches("TextRun: ")
                    .to_owned()
            };
            // How far in from the whole item its visible part starts and ends, on each side.
            let cut = |text: &str| {
                let whole = numbers(text)?;
                let seen = match text.split_once(", seen ") {
                    Some((_, seen)) => numbers(seen)?,
                    None => whole,
                };
                Some([
                    seen[0] - whole[0],
                    seen[1] - whole[1],
                    whole[0] + whole[2] - seen[0] - seen[2],
                    whole[1] + whole[3] - seen[1] - seen[3],
                ])
            };
            let at = |o: &Difference| {
                let (before, after) = o.detail.split_once(" -> ")?;
                let (a, b) = (numbers(before)?, numbers(after)?);
                let grows = b[2] - a[2];
                let width = if o.what == "control" {
                    grows.abs() < 0.05
                } else {
                    (-0.05..=BLANK).contains(&grows)
                };
                let same_cut = cut(before)?
                    .iter()
                    .zip(cut(after)?)
                    .all(|(a, b)| (a - b).abs() < 0.05);
                (a[1] == b[1] && a[3] == b[3] && width && same_cut).then_some(b)
            };
            let run_width = |o: &Difference, b: [f32; 4]| (o.what == "control").then_some(b[2]);
            let Some(b) = at(d).filter(|_| {
                d.screen == "session-tonight: planned fight open"
                    && ["moved", "resized", "width"].contains(&d.change.as_str())
                    && super::f1_new_text(&d.name)
            }) else {
                return false;
            };
            let x = b[0];
            // The link's right edge in the frozen view: the link itself does not change.
            let link = reference()
                .views
                .iter()
                .filter(|view| (&view.screen, &view.view) == (&d.screen, &d.view))
                .flat_map(|view| &view.controls)
                .find(|item| item.0 == "Link: Read encounter")
                .map(|item| item.1[0] + item.1[2]);
            link.is_some_and(|link| (link - x).abs() < 0.05)
                && rows.iter().map(|row| &row.difference).any(|o| {
                    (&o.screen, &o.view) == (&d.screen, &d.view)
                        && o.what != d.what
                        && bare(o) == bare(d)
                        && at(o).is_some_and(|other| {
                            // The painted words are no wider than their text run.
                            let (run, words) = match run_width(d, b) {
                                Some(run) => (run, other[2]),
                                None => (run_width(o, other).unwrap_or(0.0), b[2]),
                            };
                            (other[0] - x).abs() < 0.05 && words <= run + 0.05
                        })
                })
        }

        /// The views of the plain reference that these classes were written against, read
        /// once. A later freeze writes a new `reference.json`, in which History already scrolls;
        /// these classes measure the views of the freeze before pull request 275
        /// (`history_r1c.json`, frozen at 0855cf95), as their rows did.
        fn reference() -> &'static super::super::Reference {
            static REFERENCE: std::sync::OnceLock<super::super::Reference> =
                std::sync::OnceLock::new();
            REFERENCE.get_or_init(|| {
                super::super::read(&super::super::folder().join("history_r1c.json"))
            })
        }

        /// An act around History whose every changed word is a text that scrolled out of or
        /// into a History view.
        pub(super) fn act_words(d: &Difference, rows: &[AllowedRow]) -> bool {
            if d.what != "act" || d.change != "changed" || !ACTS.contains(&d.name.as_str()) {
                return false;
            }
            let scrolled_words = |word: &str| {
                rows.iter().map(|row| &row.difference).any(|o| {
                    screen(o)
                        && o.what == "text"
                        && ["gone", "new"].contains(&o.change.as_str())
                        && o.name == word
                })
            };
            d.detail.split(" | ").all(|part| {
                ["appeared + ", "appeared - ", "went + ", "went - "]
                    .iter()
                    .find_map(|prefix| part.strip_prefix(prefix))
                    .is_some_and(scrolled_words)
            })
        }

        /// The faults R1c's classes refuse, beside one scroll of the opened run's row.
        #[test]
        fn the_r1c_classes_refuse_a_loss_a_sideways_move_a_narrowing_and_other_words() {
            let row =
                |screen: &str, what: &str, change: &str, name: &str, detail: &str| AllowedRow {
                    row: 22,
                    why: String::new(),
                    difference: Difference {
                        screen: screen.into(),
                        view: "960x640 Lookup closed".into(),
                        what: what.into(),
                        change: change.into(),
                        name: name.into(),
                        detail: detail.into(),
                    },
                };
            const LAST: &str = "run-ended: played last";
            let scroll = row(
                LAST,
                "control",
                "moved",
                "Button: Close the run of Wolves",
                "847.2 445 50.8x28 -> 847.2 164 50.8x28",
            );
            let continued = |change: &str, detail: &str| {
                row(LAST, "control", change, "Button: Continue fight", detail)
            };
            let fits =
                |rows: &[AllowedRow]| rows.iter().all(|row| super::class_of(row, rows).is_some());
            // Today's view: the scroll, V11's line, one item out at the top, one in below.
            let good = [
                scroll.clone(),
                continued("moved", "180 517 134.1x28 -> 180 228 134.1x28"),
                row(
                    LAST,
                    "control",
                    "moved",
                    "Label: Read-only",
                    "180 490 70x16 -> 180 204 70x16",
                ),
                row(
                    LAST,
                    "control",
                    "resized",
                    "Label: Ended manually",
                    "258 485 86x26 -> 258 204 86x16",
                ),
                row(
                    LAST,
                    "control",
                    "gone",
                    "Button: Resume",
                    "was 753.1 99 95.8x28",
                ),
                row(
                    LAST,
                    "control",
                    "new",
                    "Label: COMBAT LOG",
                    "at 180 512 73.8x14",
                ),
                row(
                    LAST,
                    "control",
                    "moved",
                    "Label: HP 0/22",
                    "306.8 603 44.2x26 -> 306.8 320 44.2x26",
                ),
                row(
                    LAST,
                    "control",
                    "moved",
                    "Label: HP 21/22",
                    "306.8 633 44.2x26 -> 306.8 350 44.2x26",
                ),
            ];
            assert!(fits(&good), "today's scroll fits its class");
            // V9: the words' text run and painted words start at one x, on their line.
            const OPEN: &str = "session-tonight: planned fight open";
            const WORDS: &str = " · Low · XP 400 of 2,800";
            let words = |what: &str, detail: &str| {
                let name = if what == "control" {
                    format!("TextRun: {WORDS}")
                } else {
                    WORDS.to_owned()
                };
                let change = if what == "control" {
                    "moved"
                } else {
                    "resized"
                };
                AllowedRow {
                    row: 2,
                    ..row(OPEN, what, change, &name, detail)
                }
            };
            let painted = words("text", "494.8 549.6 146.4x18 -> 493.1 549.6 149.4x18");
            assert!(fits(&[
                words("control", "564.8 549.6 149.4x18 -> 493.1 549.6 149.4x18"),
                painted.clone(),
            ]));
            assert!(
                !fits(&[
                    words("control", "564.8 549.6 149.4x18 -> 480 549.6 149.4x18"),
                    words("text", "494.8 549.6 146.4x18 -> 480 549.6 149.4x18"),
                ]),
                "V9 words moved together over the link were accepted"
            );
            let run = |after: &str| words("control", &format!("564.8 549.6 149.4x18 -> {after}"));
            let text = |after: &str| words("text", &format!("494.8 549.6 146.4x18 -> {after}"));
            for (fault, why) in [
                ([run("480 549.6 149.4x18"), painted.clone()], "apart"),
                ([run("493.1 560 149.4x18"), painted.clone()], "lower"),
                ([run("493.1 549.6 120x18"), painted.clone()], "cut"),
                // The review of PR 275: cut by their visible part, or far wider.
                (
                    [
                        run("493.1 549.6 149.4x18, seen 493.1 549.6 74.7x18"),
                        text("493.1 549.6 149.4x18, seen 493.1 549.6 74.7x18"),
                    ],
                    "cut to half their width",
                ),
                (
                    [
                        run("493.1 549.6 149.4x18, seen 493.1 553.6 149.4x10"),
                        text("493.1 549.6 149.4x18, seen 493.1 553.6 149.4x10"),
                    ],
                    "cut through the middle",
                ),
                (
                    [run("493.1 549.6 649.4x18"), text("493.1 549.6 649.4x18")],
                    "500 px wider",
                ),
                (
                    [run("493.1 549.6 149.4x18"), text("493.1 549.6 160x18")],
                    "painted wider than their text run",
                ),
            ] {
                assert!(!fits(&fault), "V9 words {why} were accepted");
            }
            // Two HP rows of one band, 4 px apart today, each moved by an amount of its own
            // within V11, so that they overlap by 4 px (the review of PR 275).
            let hp = |name: &str, detail: &str| row(LAST, "control", "moved", name, detail);
            assert!(
                !fits(&[
                    scroll.clone(),
                    hp("Label: HP 20/22", "306.8 603 44.2x26 -> 306.8 322 44.2x26"),
                    hp("Label: HP 21/22", "306.8 633 44.2x26 -> 306.8 344 44.2x26"),
                ]),
                "two rows of one band moved by amounts of their own were accepted"
            );
            for (fault, why) in [
                (
                    continued("gone", "was 180 517 134.1x28"),
                    "Continue fight gone",
                ),
                (
                    continued("moved", "190 517 134.1x28 -> 200 228 134.1x28"),
                    "Continue fight moved sideways",
                ),
                (
                    continued("width", "180 517 134.1x28 -> 180 228 120x28"),
                    "Continue fight narrower",
                ),
                (
                    continued(
                        "moved",
                        "180 517 134.1x28 -> 180 228 134.1x28, seen 180 228 60x28",
                    ),
                    "Continue fight cut across",
                ),
                (
                    continued(
                        "moved",
                        "180 517 134.1x28 -> 180 228 134.1x28, seen 180 240 134.1x16",
                    ),
                    "Continue fight cut inside the view",
                ),
                (
                    continued("moved", "180 517 134.1x28 -> 180 200 134.1x28"),
                    "Continue fight moved further than the scroll and V11",
                ),
                (
                    row(
                        LAST,
                        "control",
                        "resized",
                        "Label: HP 0/22",
                        "306.8 603 44.2x26 -> 306.8 320 44.2x16",
                    ),
                    "a row that is not the Read-only line resized",
                ),
                (
                    row(
                        "session-tonight",
                        "control",
                        "gone",
                        "Button: Resume",
                        "was 753.1 99 95.8x28",
                    ),
                    "another screen",
                ),
                (
                    row(
                        "act 28",
                        "act",
                        "changed",
                        "Whole run",
                        "appeared + Start fight",
                    ),
                    "an act's word that did not scroll",
                ),
            ] {
                let mut rows = vec![scroll.clone(), fault];
                rows.push(row(LAST, "text", "gone", "Resume", "was 753.1 99 95.8x28"));
                assert!(!fits(&rows), "{why} was accepted");
            }
        }
    }

    /// P7's classes. Every difference is new, whole and of the plan's views; none lets an item
    /// of today go, move, narrow or be cut.
    /// R1a's classes. Repair 5 draws a scene's words at a plan row title's size, from the fights'
    /// title column (12 px left of where they started), with the box's 24 px target ending at
    /// that column, so a scene row's height changes. ONE rule for each view: every unchanged item
    /// below a changed scene row, control or text, moves straight up by one amount for all items
    /// below the same scene, the sum of the height changes of the scene rows above it; nothing
    /// moves sideways, changes size, is cut or goes. A scene's words and links take their exact
    /// size at the title's size (`TITLE`). The scene rows keep their order and never overlap.
    /// Repair 8 puts the section caret before `N scenes`.
    mod scene_rows {
        use super::super::{AllowedRow, Difference};

        /// The plain campaign's scenes, their links and their second line.
        const SCENES: [&str; 4] = [
            "Read Emberfall Rumours",
            "Talk to Hobb the Miller about the ropes",
            "Ask about the toll",
            "Meet Hobb the Miller at the mill",
        ];
        const LINKS: [&str; 2] = [
            "Hobb the Miller (opens Hobb the Miller)",
            "Emberfall Rumours (opens Emberfall Rumours)",
        ];
        const DETAIL: &str = "NPC · alive";
        /// How far left the words start now: from after the 26 px box to the fights' column.
        const LEFT: f32 = 12.0;
        /// The box's target, and the leading slot with its gap that it ends with.
        const BOX: f32 = 24.0;
        const SLOT: f32 = 22.0;
        /// The section caret of a folded section, and of an open one.
        const CARETS: [&str; 2] = ["\u{e13a}", "\u{e136}"];
        /// What the caret and its blank add to the width of `N scenes`.
        const CARET: f32 = 17.0;
        /// Each scene's words, and each link in them, at the title's size: their width on one
        /// line of `BOX`, the same at every width of the pane.
        const TITLE: [(&str, f32); 6] = [
            ("Read Emberfall Rumours", 155.8),
            ("Talk to Hobb the Miller about the ropes", 244.8),
            ("Ask about the toll", 109.8),
            ("Meet Hobb the Miller at the mill", 196.8),
            ("Hobb the Miller (opens Hobb the Miller)", 96.1),
            ("Emberfall Rumours (opens Emberfall Rumours)", 120.8),
        ];
        /// The gap between a scene's words and an action on its row (`design::L`).
        const GAP: f32 = 16.0;

        fn bare(name: &str) -> &str {
            ["Button: ", "CheckBox: ", "Label: ", "TextRun: ", "Link: "]
                .iter()
                .find_map(|prefix| name.strip_prefix(prefix))
                .unwrap_or(name)
        }

        fn plan_view(d: &Difference) -> bool {
            (d.screen.starts_with("session-") || d.screen.starts_with("lookup-reader "))
                && ["control", "text"].contains(&d.what.as_str())
                && !d.detail.contains("seen")
        }

        fn numbers(text: &str) -> Option<[f32; 4]> {
            let numbers: Vec<f32> = text
                .replace('x', " ")
                .split_whitespace()
                .map(|n| n.parse().ok())
                .collect::<Option<_>>()?;
            numbers.try_into().ok()
        }

        /// The item's place before and after, whole: a `seen` (cut) item has none.
        fn places(d: &Difference) -> (Option<[f32; 4]>, Option<[f32; 4]>) {
            if d.detail.contains("seen") {
                return (None, None);
            }
            match d.change.as_str() {
                "gone" => (d.detail.strip_prefix("was ").and_then(numbers), None),
                "new" => (None, d.detail.strip_prefix("at ").and_then(numbers)),
                _ => d
                    .detail
                    .split_once(" -> ")
                    .map_or((None, None), |(a, b)| (numbers(a), numbers(b))),
            }
        }

        fn same_view<'a>(
            d: &'a Difference,
            rows: &'a [AllowedRow],
        ) -> impl Iterator<Item = &'a Difference> {
            rows.iter()
                .map(|row| &row.difference)
                .filter(move |o| (&o.screen, &o.view) == (&d.screen, &d.view))
        }

        /// The scene a box names, by its accessible name.
        fn box_of(name: &str) -> Option<&'static str> {
            let words = bare(name)
                .strip_prefix("Done: ")
                .or_else(|| bare(name).strip_prefix("Open: "))?;
            SCENES.iter().copied().find(|scene| *scene == words)
        }

        /// Each scene box of the view, top down: its place before and after.
        fn boxes(d: &Difference, rows: &[AllowedRow]) -> Vec<([f32; 4], [f32; 4])> {
            let mut found: Vec<_> = same_view(d, rows)
                .filter(|o| o.what == "control" && box_of(&o.name).is_some())
                .filter_map(|o| match places(o) {
                    (Some(a), Some(b)) => Some((a, b)),
                    _ => None,
                })
                .collect();
            found.sort_by(|a, b| a.0[1].total_cmp(&b.0[1]));
            found
        }

        fn near(a: f32, b: f32) -> bool {
            (a - b).abs() < 0.05
        }

        /// Words or a link of a scene at their exact size at the title's size.
        fn title_size(name: &str, b: &[f32; 4]) -> bool {
            TITLE
                .iter()
                .any(|(words, width)| *words == name && near(b[2], *width) && near(b[3], BOX))
        }

        /// A scene's own item: its box, its words or a piece of them, a link, its second line.
        fn own(o: &Difference) -> bool {
            let name = bare(&o.name);
            box_of(&o.name).is_some()
                || SCENES
                    .iter()
                    .any(|scene| scene.contains(name.trim()) && !name.trim().is_empty())
                || LINKS.contains(&name)
                || name == DETAIL
        }

        /// A scene row: its box's place before and after, and the bottom after of its own items.
        type SceneRow = ([f32; 4], [f32; 4], f32);

        /// The scene rows of the view, top down: each box's place before and after, and the
        /// bottom after of the row's own items. `None` when the rows change their order or
        /// overlap.
        fn scene_rows_of(d: &Difference, rows: &[AllowedRow]) -> Option<Vec<SceneRow>> {
            let boxes = boxes(d, rows);
            let mut out: Vec<SceneRow> = Vec::new();
            for (k, (old, new)) in boxes.iter().enumerate() {
                let next = boxes.get(k + 1);
                let inside = |y: f32, top: f32, end: Option<f32>| {
                    y + 0.5 >= top && end.is_none_or(|end| y + 0.5 < end)
                };
                let bottom = same_view(d, rows)
                    .filter(|o| plan_view(o) && own(o))
                    .filter_map(|o| match places(o) {
                        (Some(a), Some(b)) if inside(a[1], old[1], next.map(|n| n.0[1])) => {
                            Some(b[1] + b[3])
                        }
                        (None, Some(b)) if inside(b[1], new[1], next.map(|n| n.1[1])) => {
                            Some(b[1] + b[3])
                        }
                        _ => None,
                    })
                    .fold(new[1] + new[3], f32::max);
                if out
                    .last()
                    .is_some_and(|(_, _, above)| new[1] + 0.05 < *above)
                {
                    return None;
                }
                out.push((*old, *new, bottom));
            }
            Some(out)
        }

        /// A scene's own part: its box, its words, its links, its second line, or a piece of its
        /// words that now joins one line. The words start `LEFT` further left, at the title's
        /// size: never taller, never larger, never lower; the box's target is 24 px and ends at
        /// the fights' title column.
        pub(super) fn part(d: &Difference, rows: &[AllowedRow]) -> bool {
            if !plan_view(d) {
                return false;
            }
            let name = bare(&d.name);
            match places(d) {
                (Some(a), Some(b)) if d.what == "control" && box_of(&d.name).is_some() => {
                    near(b[2], BOX)
                        && near(b[3], BOX)
                        && near(b[0] + BOX, a[0] + SLOT)
                        && b[1] <= a[1]
                        && scene_rows_of(d, rows).is_some()
                }
                (Some(a), Some(b)) => {
                    // The words start `LEFT` further left; a link in them starts inside its
                    // scene's words after. Each takes its exact size at the title's size; the
                    // second line keeps its size.
                    let across = if SCENES.contains(&name) || name == DETAIL {
                        near(b[0], a[0] - LEFT)
                    } else {
                        LINKS.contains(&name)
                            && b[0] <= a[0] - LEFT
                            && same_view(d, rows).any(|o| {
                                let words = bare(&o.name);
                                o.what == d.what
                                    && SCENES.contains(&words)
                                    && name
                                        .split(" (opens ")
                                        .next()
                                        .is_some_and(|l| words.contains(l))
                                    && places(o).1.is_some_and(|w| {
                                        w[0] <= b[0]
                                            && b[0] + b[2] <= w[0] + w[2] + 0.05
                                            && near(w[1], b[1])
                                    })
                            })
                    };
                    let size = title_size(name, &b) || name == DETAIL && a[2..] == b[2..];
                    across && size && b[1] <= a[1] && scene_rows_of(d, rows).is_some()
                }
                // A scene's words that wrapped at the Reader's size join one line: each piece
                // goes, and the whole words come, in one view.
                (Some(_), None) => {
                    d.name.starts_with("TextRun: ")
                        && !name.trim().is_empty()
                        && SCENES.iter().any(|scene| {
                            scene.contains(name.trim())
                                && *scene != name
                                && same_view(d, rows).any(|o| {
                                    o.change == "new" && o.name == format!("TextRun: {scene}")
                                })
                        })
                }
                (None, Some(b)) => {
                    d.name.starts_with("TextRun: ") && SCENES.contains(&name) && {
                        let pieces: Vec<([f32; 4], String)> = same_view(d, rows)
                            .filter(|o| o.change == "gone" && o.name.starts_with("TextRun: "))
                            .filter_map(|o| Some((places(o).0?, bare(&o.name).to_owned())))
                            .filter(|(_, piece)| name.contains(piece.trim()))
                            .collect();
                        let mut pieces = pieces;
                        pieces.sort_by(|a, b| a.0[1].total_cmp(&b.0[1]));
                        let start = pieces
                            .iter()
                            .map(|(a, _)| a[0])
                            .fold(f32::INFINITY, f32::min);
                        let joined: String = pieces.into_iter().map(|(_, piece)| piece).collect();
                        joined == name && near(b[0], start - LEFT) && title_size(name, &b)
                    }
                }
                _ => false,
            }
        }

        /// The decisions' rule for one view: an item moved straight up (its place across and its
        /// size kept), by the change of the scene rows above it. On a scene row's first line (its
        /// menu), by that scene box's own move; below it and above the next scene, by the next
        /// box's move; below the last scene row, by one amount that every such item of the view
        /// shares, controls and texts alike, at least the last box's move. Below a scene row, an
        /// item never lies over that row's own items.
        pub(super) fn below(d: &Difference, rows: &[AllowedRow]) -> bool {
            if !plan_view(d) || d.change != "moved" {
                return false;
            }
            let (Some(a), Some(b)) = places(d) else {
                return false;
            };
            if !(a[0] == b[0] && a[2..] == b[2..] && b[1] < a[1]) {
                return false;
            }
            let shift = a[1] - b[1];
            let Some(scenes) = scene_rows_of(d, rows) else {
                return false;
            };
            let Some(at) = scenes.iter().rposition(|(old, ..)| old[1] <= a[1] + 0.5) else {
                return false;
            };
            let moved = |(old, new, _): &SceneRow| old[1] - new[1];
            if a[1] < scenes[at].0[1] + BOX + 2.0 {
                return near(shift, moved(&scenes[at]));
            }
            if b[1] + 0.05 < scenes[at].2 {
                return false;
            }
            if let Some(next) = scenes.get(at + 1) {
                return near(shift, moved(next));
            }
            let last = scenes[at].0[1] + BOX + 2.0;
            shift + 0.05 >= moved(&scenes[at])
                && same_view(d, rows)
                    .filter(|o| o.change == "moved" && plan_view(o))
                    .filter_map(|o| match places(o) {
                        (Some(a), Some(b)) if a[0] == b[0] && a[2..] == b[2..] && a[1] >= last => {
                            Some(a[1] - b[1])
                        }
                        _ => None,
                    })
                    .all(|other| near(other, shift))
        }

        /// A done and linked scene's `Add to Log` on its row's first line, at least a 24 px
        /// target, `GAP` right of that scene's words, with its word and its icon inside it; or an
        /// act that shows or hides only that word.
        pub(super) fn log_on_row(d: &Difference, rows: &[AllowedRow]) -> bool {
            if d.what == "act" {
                return d.change == "changed"
                    && d.detail.split(" | ").all(|part| {
                        ["appeared + Add to Log", "went + Add to Log"].contains(&part)
                    });
            }
            if !plan_view(d) || d.change != "new" {
                return false;
            }
            let Some(at) = places(d).1 else {
                return false;
            };
            // The button, on the first line of its own done, linked scene's row, clear of its
            // words.
            let button = |o: &Difference| {
                let scene = bare(&o.name).strip_prefix("Add to Log: ")?;
                let rect = places(o).1?;
                let linked = LINKS.iter().any(|link| {
                    link.split(" (opens ")
                        .next()
                        .is_some_and(|words| scene.contains(words))
                });
                let on_row = same_view(o, rows).any(|b| {
                    b.what == "control"
                        && bare(&b.name).strip_prefix("Done: ") == Some(scene)
                        && SCENES.contains(&scene)
                        && places(b)
                            .1
                            .is_some_and(|r| near(r[1], rect[1]) && r[0] + r[2] <= rect[0])
                });
                let clear = same_view(o, rows).any(|w| {
                    w.what == "control"
                        && bare(&w.name) == scene
                        && places(w)
                            .1
                            .is_some_and(|w| w[0] + w[2] + GAP <= rect[0] + 0.05)
                });
                let target = rect[2] >= BOX && rect[3] >= BOX;
                (o.what == "control" && o.change == "new" && linked && on_row && clear && target)
                    .then_some(rect)
            };
            if d.what == "control" {
                return button(d).is_some();
            }
            ["Add to Log", "\u{e3d4}"].contains(&d.name.as_str())
                && same_view(d, rows).filter_map(button).any(|rect| {
                    rect[0] <= at[0]
                        && at[0] + at[2] <= rect[0] + rect[2]
                        && rect[1] <= at[1]
                        && at[1] + at[3] <= rect[1] + rect[3]
                })
        }

        fn fold_words(words: &str) -> bool {
            words.split_once(' ').is_some_and(|(n, word)| {
                !n.is_empty()
                    && n.chars().all(|c| c.is_ascii_digit())
                    && ["scene", "scenes"].contains(&word)
            })
        }

        /// The caret before `N scenes`: the link wider by the caret, its place and height kept;
        /// its words' old text gone, paired with the caret and the same words new, at the same
        /// place and height, wider.
        pub(super) fn caret(d: &Difference, rows: &[AllowedRow]) -> bool {
            if !plan_view(d) {
                return false;
            }
            let name = bare(&d.name);
            let with_caret = |words: &str| {
                CARETS
                    .iter()
                    .find_map(|caret| words.strip_prefix(caret)?.strip_prefix(' '))
                    .filter(|rest| fold_words(rest))
                    .map(str::to_owned)
            };
            let pair = |old: &Difference, new: &Difference| match (places(old).0, places(new).1) {
                (Some(a), Some(b)) => {
                    old.change == "gone"
                        && new.change == "new"
                        && old.what == "text"
                        && new.what == "text"
                        && with_caret(&new.name).as_deref() == Some(old.name.as_str())
                        && a[0] == b[0]
                        && a[1] == b[1]
                        && a[3] == b[3]
                        && near(b[2], a[2] + CARET)
                }
                _ => false,
            };
            match (d.what.as_str(), d.change.as_str()) {
                ("control", "width") => {
                    fold_words(name)
                        && matches!(places(d), (Some(a), Some(b))
                            if a[..2] == b[..2] && a[3] == b[3] && near(b[2], a[2] + CARET))
                }
                ("text", "gone") => same_view(d, rows).any(|new| pair(d, new)),
                ("text", "new") => same_view(d, rows).any(|old| pair(old, d)),
                _ => false,
            }
        }
    }

    mod rewards {
        use super::super::{AllowedRow, Difference};
        use super::rect_of;

        const WORD: &str = "Add rewards";

        fn plan_view(d: &Difference) -> bool {
            d.screen.starts_with("session-") || d.screen.starts_with("lookup-reader ")
        }

        fn new_whole(d: &Difference) -> Option<[f32; 4]> {
            (plan_view(d)
                && ["control", "text"].contains(&d.what.as_str())
                && d.change == "new"
                && !d.detail.contains("seen"))
            .then(|| rect_of(&d.detail, "at "))
            .flatten()
        }

        /// The row's action, named for its fight, or the opened row's link.
        pub(super) fn action(d: &Difference) -> bool {
            new_whole(d).is_some()
                && (d
                    .name
                    .strip_prefix("Button: Add rewards: ")
                    .is_some_and(|title| !title.is_empty())
                    || d.name == "Link: Add rewards")
        }

        /// The painted word lies inside a new action of its view; a separator sits on that
        /// link's line, just before it.
        pub(super) fn words(d: &Difference, rows: &[AllowedRow]) -> bool {
            let Some(at) = new_whole(d) else {
                return false;
            };
            let actions = rows.iter().map(|row| &row.difference).filter(|o| {
                (&o.screen, &o.view, o.what.as_str()) == (&d.screen, &d.view, "control")
                    && action(o)
            });
            let (x, y) = (at[0] + at[2] / 2.0, at[1] + at[3] / 2.0);
            match d.name.as_str() {
                WORD => actions
                    .filter_map(|o| rect_of(&o.detail, "at "))
                    .any(|r| r[0] <= x && x <= r[0] + r[2] && r[1] <= y && y <= r[1] + r[3]),
                "·" | "Label: ·" | "TextRun: ·" => actions
                    .filter(|o| o.name == "Link: Add rewards")
                    .filter_map(|o| rect_of(&o.detail, "at "))
                    .any(|r| {
                        at[0] + at[2] <= r[0]
                            && r[0] - (at[0] + at[2]) <= crate::design::M
                            && (r[1] + r[3] / 2.0 - y).abs() <= 4.0
                    }),
                _ => false,
            }
        }

        /// An act now shows or hides only the action's word.
        pub(super) fn act_words(d: &Difference) -> bool {
            d.what == "act"
                && d.change == "changed"
                && d.detail.split(" | ").all(|part| {
                    [format!("appeared + {WORD}"), format!("went + {WORD}")]
                        .contains(&part.to_owned())
                })
        }
    }

    /// P3's classes. Each proves itself from the difference's geometry and words: no class lets
    /// an item of today go, be cut, be resized or narrow, apart from the count (n) and Read's
    /// plan section (r), which row 14 rules.
    mod scenes {
        use super::super::{AllowedRow, Difference};

        /// The plain campaign's task lines, as its notes hold them.
        const SCENES: [&str; 4] = [
            "Read Emberfall Rumours",
            "Talk to Hobb the Miller about the ropes",
            "Ask about the toll",
            "Meet Hobb the Miller at the mill",
        ];
        const PARTS: [&str; 3] = ["Talk to Hobb the Miller about the ", "ropes", "NPC · alive"];
        const LINKS: [&str; 2] = [
            "Hobb the Miller (opens Hobb the Miller)",
            "Emberfall Rumours (opens Emberfall Rumours)",
        ];
        const MENU: &str = "\u{e1fe}";
        /// The task boxes that Read draws, and the More menu's `Scene` icon.
        const GLYPHS: [&str; 2] = ["\u{e186}", "\u{e45e}"];
        /// Where the Session page's Read pane starts at 1200x800.
        const READ_X: f32 = 700.0;

        fn bare(name: &str) -> &str {
            ["Button: ", "CheckBox: ", "Label: ", "TextRun: ", "Link: "]
                .iter()
                .find_map(|prefix| name.strip_prefix(prefix))
                .unwrap_or(name)
        }

        /// The numbers of a rectangle `x y wxh`, before any `, seen`.
        fn rect(text: &str) -> Vec<f32> {
            text.split(", seen")
                .next()
                .unwrap_or_default()
                .split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-'))
                .filter_map(|part| part.parse().ok())
                .collect()
        }

        fn moved(d: &Difference) -> Option<(Vec<f32>, Vec<f32>)> {
            let (before, after) = d.detail.split_once(" -> ")?;
            let (before, after) = (rect(before), rect(after));
            (before.len() == 4 && after.len() == 4).then_some((before, after))
        }

        fn plan_view(d: &Difference) -> bool {
            d.screen.starts_with("session-") || d.screen.starts_with("lookup-reader ")
        }

        fn shown(d: &Difference) -> bool {
            ["control", "text"].contains(&d.what.as_str()) && !d.detail.contains("seen")
        }

        fn is_count(name: &str) -> bool {
            ["TONIGHT ", "THIS SESSION "].iter().any(|word| {
                name.strip_prefix(word)
                    .is_some_and(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()))
            })
        }

        fn is_fold(name: &str) -> bool {
            name.split_once(' ').is_some_and(|(n, word)| {
                !n.is_empty()
                    && n.chars().all(|c| c.is_ascii_digit())
                    && ["scene", "scenes"].contains(&word)
            })
        }

        fn scene_words(name: &str) -> bool {
            SCENES.contains(&name)
                || PARTS.contains(&name)
                || LINKS.contains(&name)
                || SCENES.iter().any(|words| {
                    ["More actions for ", "Done: ", "Open: "]
                        .iter()
                        .any(|prefix| name.strip_prefix(prefix) == Some(words))
                })
        }

        /// Every unchanged item that moves down in a view moves by the one same distance.
        fn one_distance(d: &Difference, rows: &[AllowedRow]) -> Option<f32> {
            let mut distances = rows.iter().filter_map(|row| {
                let o = &row.difference;
                let (before, after) = moved(o).filter(|_| {
                    (&o.screen, &o.view, o.change.as_str()) == (&d.screen, &d.view, "moved")
                        && shown(o)
                })?;
                let same = before[0] == after[0] && before[2..] == after[2..];
                (same && after[1] > before[1]).then(|| ((after[1] - before[1]) * 10.0).round())
            });
            let first = distances.next()?;
            distances.all(|other| other == first).then_some(first)
        }

        fn inside(at: &[f32], r: &[f32]) -> bool {
            at.len() >= 2
                && r.len() == 4
                && r[0] <= at[0]
                && at[0] <= r[0] + r[2]
                && r[1] <= at[1]
                && at[1] <= r[1] + r[3]
        }

        pub(super) fn moved_down(d: &Difference, rows: &[AllowedRow]) -> bool {
            if !plan_view(d) || !shown(d) {
                return false;
            }
            if d.change == "moved" {
                return moved(d).is_some_and(|(before, after)| {
                    before[0] == after[0]
                        && before[2..] == after[2..]
                        && after[1] > before[1]
                        && one_distance(d, rows).is_some()
                });
            }
            // A painted glyph is matched by its order: a fight's menu glyph that moved down
            // with its row shows as new, inside a menu button that moved by that distance.
            if d.change == "new" && bare(&d.name) == MENU {
                let at = rect(&d.detail);
                return rows.iter().any(|row| {
                    let o = &row.difference;
                    (&o.screen, &o.view, o.change.as_str()) == (&d.screen, &d.view, "moved")
                        && bare(&o.name).starts_with("More actions for ")
                        && moved(o).is_some_and(|(_, r)| inside(&at, &r))
                        && moved_down(o, rows)
                });
            }
            false
        }

        pub(super) fn scene_content(d: &Difference, rows: &[AllowedRow]) -> bool {
            if !plan_view(d) || !shown(d) || d.change != "new" {
                return false;
            }
            let name = bare(&d.name);
            if scene_words(name) {
                return true;
            }
            // A scene menu's glyph lies inside that new menu's button.
            name == MENU && {
                let at = rect(&d.detail);
                rows.iter().any(|row| {
                    let o = &row.difference;
                    (&o.screen, &o.view, o.change.as_str()) == (&d.screen, &d.view, "new")
                        && SCENES.iter().any(|words| {
                            bare(&o.name).strip_prefix("More actions for ") == Some(words)
                        })
                        && inside(&at, &rect(&o.detail))
                })
            }
        }

        pub(super) fn scene_line(d: &Difference) -> bool {
            shown(d)
                && d.change == "new"
                && d.screen == "session-tonight: More open"
                && ["Add Scene", "Scene", GLYPHS[0]].contains(&bare(&d.name))
        }

        pub(super) fn reader_start(d: &Difference) -> bool {
            let name = bare(&d.name);
            let reader =
                d.screen.starts_with("lookup-reader ") || d.screen.starts_with("world-reader");
            shown(d)
                && d.change == "new"
                && (name.starts_with("Start a fight with ")
                    || (reader && ["Start fight", "\u{e3d0}"].contains(&name)))
        }

        pub(super) fn remove_encounter(d: &Difference) -> bool {
            shown(d)
                && d.change == "new"
                && d.screen == "encounters: row menu"
                && bare(&d.name) == "Remove…"
        }

        pub(super) fn fold(d: &Difference) -> bool {
            plan_view(d) && shown(d) && d.change == "new" && is_fold(bare(&d.name))
        }

        /// The count goes from 2 to 3: each gone count has a new count in its view.
        pub(super) fn count(d: &Difference, rows: &[AllowedRow]) -> bool {
            if !plan_view(d)
                || !shown(d)
                || !["new", "gone"].contains(&d.change.as_str())
                || !is_count(bare(&d.name))
            {
                return false;
            }
            let of = |change: &str| {
                rows.iter()
                    .filter(|row| {
                        let o = &row.difference;
                        (&o.screen, &o.view, &o.what) == (&d.screen, &d.view, &d.what)
                            && o.change == change
                            && is_count(bare(&o.name))
                    })
                    .count()
            };
            of("new") == of("gone")
        }

        /// A task line as Read draws it: its words, after a space or after its box.
        fn read_task(name: &str) -> bool {
            SCENES.iter().any(|words| {
                name.strip_prefix(' ') == Some(words)
                    || GLYPHS.iter().any(|glyph| {
                        name.strip_prefix(glyph)
                            .and_then(|rest| rest.strip_prefix(' '))
                            == Some(words)
                    })
            })
        }

        pub(super) fn read_leaves_out(d: &Difference) -> bool {
            if !d.screen.starts_with("session-last") || !shown(d) {
                return false;
            }
            let name = bare(&d.name);
            match d.change.as_str() {
                // The plan section in Read: its heading and its task lines, with their box.
                "gone" => {
                    let line = ["Plan for tonight", "Done", "Open"].contains(&name)
                        || GLYPHS.contains(&name)
                        || read_task(name);
                    line && rect(&d.detail).first().is_some_and(|x| *x >= READ_X)
                }
                // The words below it in Read move up, unchanged.
                "moved" => moved(d).is_some_and(|(before, after)| {
                    before[0] >= READ_X
                        && before[0] == after[0]
                        && before[2..] == after[2..]
                        && after[1] < before[1]
                }),
                // The link that Read showed in a task line is now on the scene's row.
                "resized" => {
                    LINKS.contains(&name)
                        && moved(d)
                            .is_some_and(|(before, after)| before[0] >= READ_X && after[0] < READ_X)
                }
                _ => false,
            }
        }

        /// An act now shows or hides only words of the classes above.
        pub(super) fn act_words(d: &Difference) -> bool {
            if d.what != "act" || d.change != "changed" {
                return false;
            }
            let word = |word: &str| {
                SCENES.contains(&word)
                    || PARTS.contains(&word)
                    || GLYPHS.contains(&word)
                    || ["Scene", "", "Start fight", "Remove…", "Plan for tonight"].contains(&word)
                    || is_count(word)
                    || is_fold(word)
                    || read_task(word)
            };
            d.detail.split(" | ").all(|part| {
                ["appeared + ", "appeared - ", "went + ", "went - "]
                    .iter()
                    .find_map(|prefix| part.strip_prefix(prefix))
                    .is_some_and(word)
            })
        }
    }

    fn new_travel_menu_line(d: &Difference) -> bool {
        let line = ["Button: Roll for encounters", "Roll for encounters"];
        let shown = d.screen == TRAVEL_MENU
            && ["control", "text"].contains(&d.what.as_str())
            && d.change == "new"
            && line.contains(&d.name.as_str());
        // Opening the menu shows the line, and closing it takes the same line away.
        let act = d.what == "act"
            && d.change == "changed"
            && ((d.name.starts_with("open More actions for ")
                && d.detail == "appeared + Roll for encounters")
                || (d.name.starts_with("close More actions for ")
                    && d.detail == "went + Roll for encounters"));
        shown || act
    }

    /// `x y wxh -> x y wxh`.
    fn rects(detail: &str) -> Option<([f32; 4], [f32; 4])> {
        let read = |part: &str| -> Option<[f32; 4]> {
            let mut words = part.split_whitespace();
            let x = words.next()?.parse().ok()?;
            let y = words.next()?.parse().ok()?;
            let (w, h) = words.next()?.split_once('x')?;
            Some([x, y, w.parse().ok()?, h.parse().ok()?])
        };
        let (old, new) = detail.split_once(" -> ")?;
        Some((read(old)?, read(new)?))
    }

    fn remove_one_line_lower(d: &Difference) -> bool {
        let remove = ["Button: Remove this event", "Remove"];
        d.screen == TRAVEL_MENU
            && ["control", "text"].contains(&d.what.as_str())
            && ["moved", "resized"].contains(&d.change.as_str())
            && remove.contains(&d.name.as_str())
            && rects(&d.detail).is_some_and(|(old, new)| {
                // One line lower, the same height, and never narrower: the menu took the
                // width of its new line.
                (new[1] - old[1] - MENU_LINE).abs() < 0.5
                    && (new[3] - old[3]).abs() < 0.05
                    && new[2] >= old[2]
            })
    }

    fn class_of(row: &AllowedRow, rows: &[AllowedRow]) -> Option<&'static str> {
        CLASSES
            .iter()
            .find(|class| class.row == row.row && (class.fits)(&row.difference, rows))
            .map(|class| class.name)
    }

    // P2, F1: the difficulty word. A text that exists today may never be gone, cut, resized or
    // moved. The one "gone" a class accepts is the old form of a text that the same view shows
    // again, at the same place and height, with only passive words at its end; each old form
    // pairs with its own new form, one to one, so two losses cannot share one replacement.

    /// The difficulty word, and the XP before it in the editor and the opened row.
    const WORDS: [&str; 4] = ["Above High", "Moderate", "High", "Low"];
    const F1_NEW_TEXT: &str = "F1: a new text that ends a line: the word and the XP";
    const F1_EXTENDED: &str =
        "F1: the word added at the end of a text that exists, its start unchanged";
    const F1_OLD_FORM: &str = "F1: the old form of that text, shown again with the word at its end";
    const F1_ACT: &str = "F1: an act's words, with the word at the end of texts that exist";

    /// `x y w h` from a difference's `at …` or `was …` detail.
    fn rect_of(detail: &str, lead: &str) -> Option<[f32; 4]> {
        let numbers: Vec<f32> = detail
            .strip_prefix(lead)?
            .replace('x', " ")
            .split_whitespace()
            .map(|n| n.parse().ok())
            .collect::<Option<_>>()?;
        <[f32; 4]>::try_from(numbers).ok()
    }

    /// `6,300 of 6,400` or `11,800`: XP, and the budget when there is one.
    fn xp_numbers(numbers: &str) -> bool {
        numbers
            .split(" of ")
            .all(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit() || c == ','))
    }

    /// The words F1 may put at the end of `old` to make `new`: ` · Low`, or in the editor's
    /// heading ` · XP 400 of 2,800 · Low`. Plan summaries join with a no-break space.
    fn f1_suffix(old: &str, new: &str) -> bool {
        let (old, new) = (old.replace('\u{a0}', " "), new.replace('\u{a0}', " "));
        let Some(rest) = new.strip_prefix(&old) else {
            return false;
        };
        WORDS.iter().any(|word| {
            rest == format!(" · {word}")
                || rest
                    .strip_prefix(" · XP ")
                    .and_then(|rest| rest.strip_suffix(&format!(" · {word}")))
                    .is_some_and(xp_numbers)
        })
    }

    /// A new text on a line that exists: `Low · XP 400 of 2,800` at the right of the narrow
    /// editor's tab row, or ` · Low · XP 400 of 2,800` after `Read encounter`.
    fn f1_new_text(name: &str) -> bool {
        let name = name
            .trim_start_matches("Label: ")
            .trim_start_matches("TextRun: ")
            .replace('\u{a0}', " ");
        let name = name.strip_prefix(" · ").unwrap_or(&name);
        WORDS.iter().any(|word| {
            name.strip_prefix(word)
                .and_then(|rest| rest.strip_prefix(" · XP "))
                .is_some_and(xp_numbers)
        })
    }

    /// Each part of an act that changed: `appeared + …`, `went - …`. Every added text is an old
    /// text of the same kind with F1's words at its end, or a new F1 text; every removed text is
    /// paired, one to one, with its extended form added in the same kind.
    fn f1_act(detail: &str) -> bool {
        let parts: Vec<(&str, &str, &str)> = detail
            .split(" | ")
            .filter_map(|part| {
                let (kind, rest) = part.split_once(' ')?;
                let (sign, words) = rest.split_once(' ')?;
                Some((kind, sign, words))
            })
            .collect();
        if parts.len() != detail.split(" | ").count()
            || parts.iter().any(|(_, sign, _)| !["+", "-"].contains(sign))
        {
            return false;
        }
        let mut used = vec![false; parts.len()];
        for (i, (kind, sign, old)) in parts.iter().enumerate() {
            if *sign != "-" {
                continue;
            }
            let Some(j) = (0..parts.len()).find(|&j| {
                let (k, s, new) = parts[j];
                !used[j] && k == *kind && s == "+" && f1_suffix(old, new)
            }) else {
                return false;
            };
            used[i] = true;
            used[j] = true;
        }
        parts
            .iter()
            .zip(&used)
            .all(|((_, _, words), used)| *used || f1_new_text(words))
    }

    /// The extended form, at the same place and height, at least as wide, in the same view.
    fn f1_pair(old: &AllowedRow, new: &AllowedRow) -> bool {
        let (o, n) = (&old.difference, &new.difference);
        let rects = rect_of(&o.detail, "was ").zip(rect_of(&n.detail, "at "));
        o.screen == n.screen
            && o.view == n.view
            && o.what == n.what
            && ["control", "text"].contains(&o.what.as_str())
            && o.change == "gone"
            && n.change == "new"
            && f1_suffix(&o.name, &n.name)
            && rects.is_some_and(|(was, at)| {
                was[0] == at[0] && was[1] == at[1] && was[3] == at[3] && at[2] >= was[2]
            })
    }

    /// Each row's class: P5's from `CLASSES`, F1's one-row classes, then F1's pairs, one to one.
    fn classes(rows: &[AllowedRow]) -> Vec<Option<&'static str>> {
        let f1 = |row: &AllowedRow| (1..=3).contains(&row.row);
        let mut out: Vec<_> = rows
            .iter()
            .map(|row| {
                let d = &row.difference;
                class_of(row, rows).or_else(|| {
                    if !f1(row) {
                        None
                    } else if ["control", "text"].contains(&d.what.as_str())
                        && d.change == "new"
                        && f1_new_text(&d.name)
                    {
                        Some(F1_NEW_TEXT)
                    } else if d.what == "act" && d.change == "changed" && f1_act(&d.detail) {
                        Some(F1_ACT)
                    } else {
                        None
                    }
                })
            })
            .collect();
        for i in 0..rows.len() {
            if out[i].is_some() || !f1(&rows[i]) {
                continue;
            }
            if let Some(j) = (0..rows.len())
                .find(|&j| out[j].is_none() && f1(&rows[j]) && f1_pair(&rows[i], &rows[j]))
            {
                out[i] = Some(F1_OLD_FORM);
                out[j] = Some(F1_EXTENDED);
            }
        }
        out
    }

    #[test]
    fn the_f1_classes_refuse_a_loss_a_cut_a_move_and_two_losses_for_one_text() {
        let row = |change: &str, name: &str, detail: &str| AllowedRow {
            row: 1,
            why: String::new(),
            difference: Difference {
                screen: "s".into(),
                view: "v".into(),
                what: "text".into(),
                change: change.into(),
                name: name.into(),
                detail: detail.into(),
            },
        };
        let gone = || row("gone", "5 party · 2 × Ember Cultist", "was 10 20 140x16");
        let new = |detail: &str| row("new", "5 party · 2 × Ember Cultist · Low", detail);
        let pair = [gone(), new("at 10 20 172x16")];
        assert!(classes(&pair).iter().all(Option::is_some));
        for (rows, why) in [
            (vec![gone()], "a text gone alone"),
            (
                vec![row("new", "5 party · 2 × Ember", "at 10 20 100x16")],
                "a cut text",
            ),
            (
                vec![row("moved", "Read encounter", "1 2 3x4 -> 1 3 3x4")],
                "a moved control",
            ),
            (
                vec![row("height", "5 party", "16 -> 32")],
                "a row that grows",
            ),
            (
                vec![gone(), new("at 10 21 172x16")],
                "an extended text that moved",
            ),
            (
                vec![gone(), new("at 10 20 140x32")],
                "an extended text that wrapped",
            ),
            (
                vec![gone(), gone(), new("at 10 20 172x16")],
                "two losses for one new text",
            ),
        ] {
            assert!(
                classes(&rows).iter().any(Option::is_none),
                "{why} was accepted"
            );
        }
        // An act may only swap a text for its extended form, one to one.
        assert!(f1_act("appeared + 5 party · Low | appeared - 5 party"));
        assert!(!f1_act(
            "appeared + 5 party · Low | appeared - 5 party | appeared - 5 party"
        ));
        assert!(!f1_act("went - 5 party"));
    }

    #[test]
    fn the_r1a_classes_refuse_a_move_sideways_a_wrong_amount_a_cut_and_a_loss() {
        let row = |row: u32, what: &str, change: &str, name: &str, detail: &str| AllowedRow {
            row,
            why: String::new(),
            difference: Difference {
                screen: "session-tonight".into(),
                view: "960x640 Lookup closed".into(),
                what: what.into(),
                change: change.into(),
                name: name.into(),
                detail: detail.into(),
            },
        };
        let scene_box = row(
            23,
            "control",
            "resized",
            "CheckBox: Open: Ask about the toll",
            "372 300 26x26 -> 370 290 24x24",
        );
        let words = row(
            23,
            "control",
            "resized",
            "Label: Ask about the toll",
            "406 300 120x24 -> 394 290 109.8x24",
        );
        let fight = |detail: &str| row(23, "control", "moved", "Link: Shrine of Cinders", detail);
        let below = fight("394 400 106.8x24 -> 394 380 106.8x24");
        let good = [scene_box.clone(), words.clone(), below.clone()];
        assert!(
            classes(&good).iter().all(Option::is_some),
            "the intended rows"
        );
        // A done, linked scene's `Add to Log` and the fold's caret, as `session-last` and
        // `session-tonight: Travel row open` measure them at 960x640.
        let done_box = row(
            23,
            "control",
            "resized",
            "CheckBox: Done: Meet Hobb the Miller at the mill",
            "372 363 26x26 -> 370 363 24x24",
        );
        let done_words = row(
            23,
            "control",
            "resized",
            "Label: Meet Hobb the Miller at the mill",
            "406 363 229.8x24 -> 394 363 196.8x24",
        );
        let log = |detail: &str| {
            row(
                23,
                "control",
                "new",
                "Button: Add to Log: Meet Hobb the Miller at the mill",
                detail,
            )
        };
        let fold = |detail: &str| row(24, "control", "width", "Link: 2 scenes", detail);
        let fold_words = [
            row(24, "text", "gone", "2 scenes", "was 449.6 209 55.8x18"),
            row(
                24,
                "text",
                "new",
                "\u{e13a} 2 scenes",
                "at 449.6 209 72.8x18",
            ),
        ];
        let mut caret = fold_words.to_vec();
        caret.push(fold("449.6 206 55.8x24 -> 449.6 206 72.8x24"));
        for (rows, why) in [
            (
                vec![
                    done_box.clone(),
                    done_words.clone(),
                    log("at 738.5 363 103.5x28"),
                ],
                "a done scene's Add to Log",
            ),
            (caret.clone(), "the fold's caret"),
        ] {
            assert!(
                classes(&rows).iter().all(Option::is_some),
                "{why} was refused"
            );
        }
        for (rows, why) in [
            (
                vec![
                    scene_box.clone(),
                    fight("394 400 106.8x24 -> 390 380 106.8x24"),
                ],
                "a fight moved sideways",
            ),
            (
                vec![
                    scene_box.clone(),
                    fight("394 400 106.8x24 -> 394 380 100x24"),
                ],
                "a fight narrowed",
            ),
            (
                vec![
                    scene_box.clone(),
                    fight("394 400 106.8x24 -> 394 410 106.8x24"),
                ],
                "a fight moved down",
            ),
            (
                vec![
                    scene_box.clone(),
                    fight("394 200 106.8x24 -> 394 190 106.8x24"),
                ],
                "an item above the scenes moved",
            ),
            (
                vec![
                    scene_box.clone(),
                    below.clone(),
                    row(
                        23,
                        "control",
                        "moved",
                        "Button: Start fight: Shrine of Cinders",
                        "545 400 109x28 -> 545 385 109x28",
                    ),
                ],
                "two amounts below the last scene",
            ),
            (
                vec![
                    scene_box.clone(),
                    fight("394 400 106.8x24 -> 394 380 106.8x24, seen 394 380 106.8x10"),
                ],
                "a cut fight",
            ),
            (
                vec![
                    scene_box.clone(),
                    row(
                        23,
                        "control",
                        "gone",
                        "Button: Start fight: Shrine of Cinders",
                        "was 545 400 109x28",
                    ),
                ],
                "a control gone",
            ),
            (
                vec![row(
                    23,
                    "control",
                    "resized",
                    "CheckBox: Open: Ask about the toll",
                    "372 300 26x26 -> 372 290 22x24",
                )],
                "a box below 24 px",
            ),
            (
                vec![row(
                    23,
                    "control",
                    "resized",
                    "Label: Ask about the toll",
                    "406 300 120x24 -> 400 290 110x24",
                )],
                "words not at the title column",
            ),
            (
                vec![row(
                    23,
                    "control",
                    "resized",
                    "Label: Ask about the toll",
                    "406 300 120x24 -> 394 290 110x48",
                )],
                "words that wrap more",
            ),
            (
                vec![
                    scene_box.clone(),
                    row(
                        23,
                        "control",
                        "new",
                        "Button: Add to Log: Ask about the toll",
                        "at 700 320 103.5x28",
                    ),
                ],
                "an action off its scene's row",
            ),
            (
                vec![row(
                    24,
                    "control",
                    "width",
                    "Link: 2 scenes",
                    "449.6 206 55.8x24 -> 449.6 206 50x24",
                )],
                "a narrower fold link",
            ),
            (
                vec![row(24, "text", "gone", "2 scenes", "was 449.6 209 55.8x18")],
                "the fold's words gone alone",
            ),
            // The review of PR 280 made up these five.
            (
                vec![row(
                    23,
                    "control",
                    "resized",
                    "Label: Ask about the toll",
                    "406 300 120x24 -> 394 290 1x1",
                )],
                "a scene's words shrunk to 1x1",
            ),
            (
                vec![
                    scene_box.clone(),
                    row(
                        23,
                        "control",
                        "resized",
                        "CheckBox: Done: Read Emberfall Rumours",
                        "372 360 26x26 -> 370 290 24x24",
                    ),
                ],
                "two scene boxes that overlap",
            ),
            (
                vec![
                    scene_box.clone(),
                    below.clone(),
                    row(
                        23,
                        "text",
                        "moved",
                        "Shrine of Cinders",
                        "394 403 106.8x18 -> 394 388 106.8x18",
                    ),
                ],
                "a text 15 px up below the scene, its control 20 px",
            ),
            (
                vec![
                    done_box.clone(),
                    done_words.clone(),
                    log("at 738.5 363 1x1"),
                ],
                "an Add to Log of 1x1",
            ),
            (
                vec![
                    done_box.clone(),
                    done_words.clone(),
                    log("at 394 363 103.5x28"),
                ],
                "an Add to Log over its scene's words",
            ),
            (
                vec![
                    row(
                        23,
                        "control",
                        "resized",
                        "CheckBox: Open: Meet Hobb the Miller at the mill",
                        "372 363 26x26 -> 370 363 24x24",
                    ),
                    done_words.clone(),
                    log("at 738.5 363 103.5x28"),
                ],
                "an Add to Log on an open scene",
            ),
            (
                {
                    let mut rows = fold_words.to_vec();
                    rows.push(fold("449.6 206 55.8x24 -> 449.6 206 255.8x24"));
                    rows
                },
                "a fold link 200 px wider than its words and caret",
            ),
        ] {
            assert!(
                classes(&rows).iter().any(Option::is_none),
                "{why} was accepted"
            );
        }
    }

    #[test]
    fn every_allowed_row_is_in_a_class_of_its_row() {
        let allowed: Allowed = read(&folder().join("allowed.json"));
        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        let mut unfit = Vec::new();
        for (row, class) in allowed.rows.iter().zip(classes(&allowed.rows)) {
            match class {
                Some(class) => *counts.entry(class).or_default() += 1,
                None => unfit.push(format!("row {}: {}", row.row, row.difference)),
            }
        }
        assert!(
            unfit.is_empty(),
            "{} allowed.json rows fit no class of their row:\n{}",
            unfit.len(),
            unfit.join("\n")
        );
        println!("allowed.json classes: {counts:?}");
        // The classes refuse what a fault looks like: a line gone, or `Remove` narrower.
        let remove = Difference {
            screen: TRAVEL_MENU.into(),
            view: "1200x800".into(),
            what: "control".into(),
            change: "resized".into(),
            name: "Button: Remove this event".into(),
            detail: "681 299 67.8x30 -> 681 335 137.9x30".into(),
        };
        assert!(remove_one_line_lower(&remove));
        let narrower = Difference {
            detail: "681 299 67.8x30 -> 681 335 60.0x30".into(),
            ..remove.clone()
        };
        let gone = Difference {
            change: "gone".into(),
            ..remove.clone()
        };
        let other = Difference {
            name: "Button: Delete the saved run".into(),
            ..remove
        };
        for fault in [narrower, gone, other] {
            assert!(
                CLASSES
                    .iter()
                    .all(|class| !(class.fits)(&fault, &allowed.rows)),
                "a class allows {fault}"
            );
        }
        // P8's classes refuse a preview that moved, wrapped or went alone, two old forms for one
        // new form, `Carry over` taller than the line or on a line of its own, and another word.
        let old = Difference {
            screen: "session-tonight".into(),
            view: "1200x800 Lookup closed".into(),
            what: "text".into(),
            change: "gone".into(),
            name: "— The party left Emberfall at dawn with the miller's warning.".into(),
            detail: "was 647.3 144 322.3x16".into(),
        };
        let new = Difference {
            change: "new".into(),
            name: "— Wolves at the ford. Hobb said the toll collectors were not paid.".into(),
            detail: "at 647.3 144 346.3x16".into(),
            ..old.clone()
        };
        let row16 = |d: &Difference| AllowedRow {
            row: 16,
            why: String::new(),
            difference: d.clone(),
        };
        assert!(carry::preview(&old, &[row16(&old), row16(&new)]));
        let moved = Difference {
            detail: "at 647.3 164 346.3x16".into(),
            ..new.clone()
        };
        let wrapped = Difference {
            detail: "at 647.3 144 346.3x32".into(),
            ..new.clone()
        };
        for (rows, why) in [
            (vec![row16(&old)], "an old form gone alone"),
            (vec![row16(&old), row16(&moved)], "a new form one line down"),
            (vec![row16(&old), row16(&wrapped)], "a new form that wraps"),
            (
                vec![row16(&old), row16(&old), row16(&new)],
                "two old forms for one",
            ),
            // The review of PR 277: a cut far past what the room needs.
            (
                vec![
                    row16(&old),
                    row16(&Difference {
                        name: "— W…".into(),
                        detail: "at 647.3 144 25x16".into(),
                        ..new.clone()
                    }),
                ],
                "a preview cut to one word with room to spare",
            ),
        ] {
            assert!(
                !carry::preview(&rows[0].difference, &rows),
                "{why} was accepted"
            );
        }
        let carry = Difference {
            screen: "session-new".into(),
            view: "1200x800 Lookup closed".into(),
            what: "control".into(),
            change: "new".into(),
            name: "Button: Carry over".into(),
            detail: "at 1106.6 140 81.4x24".into(),
        };
        assert!(carry::carry_over(&carry));
        for fault in [
            Difference {
                detail: "at 1106.6 138 81.4x28".into(),
                ..carry.clone()
            },
            Difference {
                detail: "at 372 164 81.4x24".into(),
                ..carry.clone()
            },
            Difference {
                detail: "at 1150 140 81.4x24".into(),
                ..carry.clone()
            },
            Difference {
                screen: "session-tonight".into(),
                ..carry.clone()
            },
            Difference {
                change: "moved".into(),
                ..carry.clone()
            },
        ] {
            assert!(!carry::carry_over(&fault), "a class allows {fault}");
        }
        assert!(!carry::preview_act(&Difference {
            screen: "act 3".into(),
            view: String::new(),
            what: "act".into(),
            change: "changed".into(),
            name: "Session".into(),
            detail: "appeared + — Wolves at the ford. | appeared - — The party left Emberfall at dawn with the miller's warning.".into(),
        }));
        assert!(!carry::act(&Difference {
            screen: "act 109".into(),
            view: String::new(),
            what: "act".into(),
            change: "changed".into(),
            name: "Create session".into(),
            detail: "appeared + Carry over | went - Last time:".into(),
        }));
        // P3's classes refuse the faults of the review of PR 261: the next fight gone, cut,
        // narrowed, or moved in another direction; and an item of today gone outside Read.
        let fight = Difference {
            screen: "session-tonight: planned fight open".into(),
            view: "960x640 Lookup closed".into(),
            what: "control".into(),
            change: "gone".into(),
            name: "Button: Start fight: Shrine of Cinders".into(),
            detail: "was 733 618.6 109x28".into(),
        };
        let faults = [
            fight.clone(),
            Difference {
                change: "moved".into(),
                detail: "733 452 109x28 -> 733 618.6 109x28, seen 733 618.6 109x13.4".into(),
                ..fight.clone()
            },
            Difference {
                change: "width".into(),
                name: "TextInput: Title".into(),
                detail: "394 335 554x28.2 -> 394 335 542x28.2".into(),
                ..fight.clone()
            },
            Difference {
                change: "moved".into(),
                detail: "733 452 109x28 -> 703 452 109x28".into(),
                ..fight.clone()
            },
            Difference {
                screen: "session-tonight".into(),
                name: "Label: Road to the ford".into(),
                detail: "was 394 266 101x24".into(),
                ..fight
            },
        ];
        // P7's classes refuse what a fault of row 10 looks like: the action cut, a control of
        // today moved, a title that grew a line, the word outside any action, and other words.
        let action = Difference {
            screen: "session-last".into(),
            view: "1200x800".into(),
            what: "control".into(),
            change: "new".into(),
            name: "Button: Add rewards: The Toll Bridge Brawl".into(),
            detail: "at 557.1 290 96.9x28".into(),
        };
        let rewards_faults = [
            Difference {
                detail: "at 557.1 290 96.9x28, seen 557.1 290 40x28".into(),
                ..action.clone()
            },
            Difference {
                change: "moved".into(),
                name: "Button: More actions for The Toll Bridge Brawl".into(),
                detail: "662 290 28x28 -> 691.6 290 28x28".into(),
                ..action.clone()
            },
            Difference {
                change: "height".into(),
                name: "Link: Wolves at the Ford Below the Burning Shrine Road".into(),
                detail: "24 -> 48".into(),
                ..action.clone()
            },
            Difference {
                what: "text".into(),
                name: "Add rewards".into(),
                detail: "at 100 600 80.9x18".into(),
                ..action.clone()
            },
            Difference {
                change: "gone".into(),
                name: "Link: Timeline".into(),
                detail: "was 560 476 60x24".into(),
                ..action.clone()
            },
            Difference {
                screen: "act 3".into(),
                view: String::new(),
                what: "act".into(),
                change: "changed".into(),
                name: "Session".into(),
                detail: "appeared + Add rewards | went - Timeline".into(),
            },
        ];
        for fault in rewards_faults {
            let row = AllowedRow {
                row: 10,
                why: String::new(),
                difference: fault,
            };
            assert!(
                class_of(&row, &allowed.rows).is_none(),
                "a class allows {}",
                row.difference
            );
        }
        // P4's classes refuse a dock control of today that moved or went, Effect's words
        // outside Effect, and an act that shows or hides another word.
        let effect = |change: &str, what: &str, name: &str, detail: &str| Difference {
            screen: "run-conditions".into(),
            view: "1200x800 Lookup closed".into(),
            what: what.into(),
            change: change.into(),
            name: name.into(),
            detail: detail.into(),
        };
        for fault in [
            effect(
                "moved",
                "control",
                "Button: Absent",
                "618.8 725 80.9x28 -> 650.8 725 80.9x28",
            ),
            effect(
                "gone",
                "control",
                "Button: Conditions for this participant",
                "was 1 2 3x4",
            ),
            effect("new", "text", "Effect", "at 10 730 37.1x18"),
            effect("new", "text", "Conc", "at 574.1 730 37.1x18"),
            effect(
                "changed",
                "act",
                "select Brannoch",
                "appeared + E | went - Conditions",
            ),
            effect("changed", "act", "select Brannoch", "appeared - Effect"),
        ] {
            let row = AllowedRow {
                row: 6,
                why: String::new(),
                difference: fault,
            };
            assert!(
                class_of(&row, &allowed.rows).is_none(),
                "a class allows {}",
                row.difference
            );
        }
        for fault in faults {
            for row in [14, 7, 9, 13, 21] {
                let row = AllowedRow {
                    row,
                    why: String::new(),
                    difference: fault.clone(),
                };
                assert!(
                    class_of(&row, &allowed.rows).is_none(),
                    "a class allows {}",
                    row.difference
                );
            }
        }
        // P6's classes refuse the reader's Start fight gone, moved sideways (as Add to Log next
        // to Edit would move it), moved down or narrowed; Add to Log outside a reader; an
        // existing glyph moved elsewhere; and an act that hides Start fight.
        let start = Difference {
            screen: "world-reader".into(),
            view: "1200x800 Lookup closed".into(),
            what: "control".into(),
            change: "moved".into(),
            name: "Button: Start a fight with Captain Varra Black".into(),
            detail: "726.6 67 100x28 -> 834.1 67 100x28".into(),
        };
        let p6_faults = [
            Difference {
                change: "gone".into(),
                detail: "was 726.6 67 100x28".into(),
                ..start.clone()
            },
            Difference {
                detail: "726.6 67 100x28 -> 840.1 67 100x28".into(),
                ..start.clone()
            },
            Difference {
                detail: "726.6 67 100x28 -> 834.1 95 100x28".into(),
                ..start.clone()
            },
            Difference {
                change: "resized".into(),
                detail: "726.6 67 100x28 -> 834.1 67 90x28".into(),
                ..start.clone()
            },
            Difference {
                screen: "session-tonight".into(),
                change: "new".into(),
                name: "Button: Add to Log".into(),
                detail: "at 726.6 67 103.5x28".into(),
                ..start.clone()
            },
            Difference {
                what: "text".into(),
                name: "\u{e3d4}".into(),
                detail: "643.9 50 14x18 -> 700.0 50 14x18".into(),
                ..start.clone()
            },
            Difference {
                screen: "act 103".into(),
                view: String::new(),
                what: "act".into(),
                change: "changed".into(),
                name: "open Captain Varra Black".into(),
                detail: "appeared + Add to Log | went + Start fight".into(),
            },
        ];
        for fault in p6_faults {
            let row = AllowedRow {
                row: 8,
                why: String::new(),
                difference: fault,
            };
            assert!(
                class_of(&row, &allowed.rows).is_none(),
                "a class allows {}",
                row.difference
            );
        }
        // R3h's classes refuse an item of today that goes, moves sideways, moves by another
        // height or changes size below the Status row, another word in its band, a `+` that
        // leaves a place other than the new glyph's, and an act that hides another word.
        let view = |change: &str, name: &str, detail: &str| AllowedRow {
            row: 25,
            why: String::new(),
            difference: Difference {
                screen: "lookup-reader Oren Thistlewick, npc".into(),
                view: "1200x800 Lookup open".into(),
                what: "control".into(),
                change: change.into(),
                name: name.into(),
                detail: detail.into(),
            },
        };
        let status_rows = vec![
            view("new", "Button: Change Status", "at 946.2 196 63.8x30"),
            view("new", "Label: Status", "at 781 203 33.8x16"),
            view(
                "moved",
                "Button: Source and identifiers",
                "781 289 142.8x26 -> 781 325 142.8x26",
            ),
            view(
                "moved",
                "Label: Keeps watch on the road to the ford.",
                "781 212 242.5x22 -> 781 248 242.5x22",
            ),
        ];
        assert!(classes(&status_rows).iter().all(Option::is_some));
        for (fault, why) in [
            (
                view(
                    "moved",
                    "Label: Keeps watch",
                    "781 212 242.5x22 -> 790 248 242.5x22",
                ),
                "a move sideways",
            ),
            (
                view(
                    "moved",
                    "Label: Keeps watch",
                    "781 212 242.5x22 -> 781 260 242.5x22",
                ),
                "another height",
            ),
            (
                view(
                    "moved",
                    "Label: Keeps watch",
                    "781 212 242.5x22 -> 781 248 200x22",
                ),
                "a cut",
            ),
            (
                view("gone", "Label: Keeps watch", "was 781 212 242.5x22"),
                "an item gone",
            ),
            (
                view("new", "Label: Owes Nell a favour", "at 781 203 90x16"),
                "another word",
            ),
            (
                view("new", "Label: Status", "at 781 400 33.8x16"),
                "a word below the band",
            ),
            (
                view(
                    "moved",
                    "Label: Oren Thistlewick",
                    "781 126 200x36 -> 781 162 200x36",
                ),
                "the title moved",
            ),
            (
                view(
                    "moved",
                    "Button: New session",
                    "631.9 44 124.1x30 -> 631.9 80 124.1x30",
                ),
                "a control left of the reader moved",
            ),
        ] {
            let mut rows = status_rows.clone();
            rows.push(fault);
            assert!(
                classes(&rows).last().unwrap().is_none(),
                "{why} was accepted"
            );
        }
        let glyph = |change: &str, name: &str, detail: &str| AllowedRow {
            row: 26,
            why: String::new(),
            difference: Difference {
                screen: "lookup-reader Captain Varra, npc".into(),
                view: "1200x800 Lookup open".into(),
                what: "text".into(),
                change: change.into(),
                name: name.into(),
                detail: detail.into(),
            },
        };
        let swap = vec![
            glyph("moved", "\u{e3d4}", "1125.3 131 14x18 -> 643.9 50 14x18"),
            glyph("moved", "\u{e3d4}", "643.9 50 14x18 -> 522.5 476 14x18"),
            glyph("gone", "\u{e3d4}", "was 522.5 476 14x18"),
            glyph("new", "\u{e2f8}", "at 1125.3 131 14x18"),
        ];
        assert!(classes(&swap).iter().all(Option::is_some));
        for (rows, why) in [
            (swap[2..3].to_vec(), "a `+` gone with no new glyph"),
            (
                vec![
                    glyph("gone", "\u{e3d4}", "was 522.5 476 14x18"),
                    glyph("new", "\u{e2f8}", "at 1125.3 131 14x18"),
                ],
                "a `+` gone from another place",
            ),
            (
                vec![
                    glyph("moved", "\u{e3d4}", "1125.3 131 14x18 -> 700 50 14x18"),
                    glyph("gone", "\u{e3d4}", "was 643.9 50 14x18"),
                    glyph("new", "\u{e2f8}", "at 1125.3 131 14x18"),
                ],
                "a `+` that moved to a new place",
            ),
            (
                vec![
                    glyph("gone", "\u{e3d4}", "was 643.9 50 14x18"),
                    glyph("new", "\u{e2f8}", "at 643.9 50 14x18"),
                ],
                "another action's `+` (`New session`) became the glyph",
            ),
        ] {
            assert!(
                classes(&rows).iter().any(Option::is_none),
                "{why} was accepted"
            );
        }
        let act = |row: u32, detail: &str| AllowedRow {
            row,
            why: String::new(),
            difference: Difference {
                screen: "act 86".into(),
                view: String::new(),
                what: "act".into(),
                change: "changed".into(),
                name: "open lookup-reader Captain Varra, npc".into(),
                detail: detail.into(),
            },
        };
        assert!(
            class_of(
                &act(
                    25,
                    "appeared + Change | appeared + Status | appeared + alive"
                ),
                &[]
            )
            .is_some()
        );
        for fault in [
            act(
                25,
                "appeared + Change | appeared + Status | went + Add to fight",
            ),
            act(25, "appeared + Change | went - Status"),
            act(26, "appeared + \u{e2f8} | went + Edit"),
        ] {
            assert!(
                class_of(&fault, &[]).is_none(),
                "a class allows {}",
                fault.difference
            );
        }
    }
}
