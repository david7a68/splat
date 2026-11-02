//! Visual and ergonomics tests. They run the real `Splat` app headlessly, drive it
//! through the AccessKit tree, and check every state a usage pattern reaches.
//!
//! `cargo test` runs every scenario and enforces the ergonomics baseline, without
//! a GPU or any screenshots. `devenv shell -- visual-test <label>` also renders
//! PNGs with a software rasterizer. See "Visual tests" in the README.
//!
//! Environment:
//! - `SPLAT_VISUAL_CAPTURE=1` renders PNGs. Setting `SPLAT_VISUAL_DIR` also does.
//! - `SPLAT_VISUAL_DIR` is the output folder. The default is
//!   `artifacts/screenshots/latest`.
//! - `SPLAT_VISUAL_ENFORCE` is `baseline` (default), `strict` (any violation fails)
//!   or `report` (never fails on a check).
//! - `SPLAT_VISUAL_UPDATE_BASELINE=1` rewrites `baseline.json` from this run.
//! - `SPLAT_VISUAL_SCENARIOS=party,settings` runs only those scenarios.
//! - `SPLAT_VISUAL_VIDEO=1`, with captures on, also encodes one video per scenario
//!   with ffmpeg, into the output folder's `video/`.
mod compare;
mod dm_audit;
mod driver;
mod ergonomics;
mod fixture;
mod scenarios;
mod steps;
mod video;

use driver::{Driver, Outcome};
use ergonomics::{Baseline, Report};
use fixture::{Seed, World};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

const BASELINE: &str = include_str!("baseline.json");
const BASELINE_PATH: &str = "src/visual_tests/baseline.json";

type Script = fn(&mut Driver<'_>);

const SCENARIOS: [(&str, Seed, Script); 35] = [
    (
        "library-browse",
        Seed::Empty,
        scenarios::library_browse::run,
    ),
    (
        "library-author",
        Seed::Campaign,
        scenarios::library_author::run,
    ),
    (
        "campaign-content",
        Seed::Campaign,
        scenarios::campaign_content::run,
    ),
    ("party", Seed::Campaign, scenarios::party::run),
    (
        "encounter-build",
        Seed::Campaign,
        scenarios::encounter_build::run,
    ),
    (
        "encounter-run",
        Seed::Campaign,
        scenarios::encounter_run::run,
    ),
    ("settings", Seed::Campaign, scenarios::settings::run),
    ("dialogs", Seed::Campaign, scenarios::dialogs::run),
    ("empty", Seed::Empty, scenarios::empty::run),
    ("run-lookup", Seed::Campaign, scenarios::run_lookup::run),
    (
        "prep-to-table",
        Seed::Campaign,
        scenarios::prep_to_table::run,
    ),
    ("run-resume", Seed::Campaign, scenarios::run_resume::run),
    (
        "run-large-roster",
        Seed::LargeRosters,
        scenarios::run_large_roster::run,
    ),
    (
        "run-sync-conflict",
        Seed::Campaign,
        scenarios::run_sync_conflict::run,
    ),
    ("nav-state", Seed::Campaign, scenarios::nav_state::run),
    (
        "party-from-run",
        Seed::Campaign,
        scenarios::party_from_run::run,
    ),
    (
        "library-to-encounter",
        Seed::Campaign,
        scenarios::library_to_encounter::run,
    ),
    (
        "run-folder-defect",
        Seed::Campaign,
        scenarios::run_folder_defect::run,
    ),
    ("gallery", Seed::Empty, scenarios::gallery::run),
    ("run-restart", Seed::Campaign, scenarios::run_restart::run),
    (
        "column-resize",
        Seed::Campaign,
        scenarios::column_resize::run,
    ),
    (
        "library-scope",
        Seed::Campaign,
        scenarios::library_scope::run,
    ),
    ("signed-in", Seed::Campaign, scenarios::signed_in::run),
    ("nav-table", Seed::LargeRosters, scenarios::nav_table::run),
    (
        "run-sync-states",
        Seed::Campaign,
        scenarios::run_sync_states::run,
    ),
    (
        "run-roster-conflict",
        Seed::LargeRosters,
        scenarios::run_roster_conflict::run,
    ),
    (
        "run-edge-states",
        Seed::Campaign,
        scenarios::run_edge_states::run,
    ),
    (
        "run-absent-player",
        Seed::Campaign,
        scenarios::run_absent_player::run,
    ),
    (
        "run-long-names",
        Seed::Campaign,
        scenarios::run_long_names::run,
    ),
    (
        "session-notes",
        Seed::Campaign,
        scenarios::session_notes::run,
    ),
    (
        "session-recording",
        Seed::Campaign,
        scenarios::session_recording::run,
    ),
    (
        "encounter-groups",
        Seed::Groups,
        scenarios::encounter_groups::run,
    ),
    ("run-overrides", Seed::Groups, scenarios::run_overrides::run),
    (
        "run-no-groups",
        Seed::Campaign,
        scenarios::run_no_groups::run,
    ),
    ("session-plan", Seed::Campaign, scenarios::session_plan::run),
];

impl crate::Splat {
    /// Is any background load still due to change what is on screen?
    fn busy(&self) -> bool {
        self.settings.busy()
            || self.library.busy()
            || self.campaign_content.busy()
            || self.encounters.busy()
            || self.party.busy()
            || self.lookup.busy()
            || self.dndbeyond.busy()
            || self.sessions.busy()
            || self.copy_dialog.as_ref().is_some_and(|d| d.busy())
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Enforce {
    Baseline,
    Strict,
    Report,
}

fn enforcement() -> Enforce {
    match std::env::var("SPLAT_VISUAL_ENFORCE").as_deref() {
        Ok("strict") => Enforce::Strict,
        Ok("report") => Enforce::Report,
        Ok("baseline") | Err(_) => Enforce::Baseline,
        Ok(other) => panic!("SPLAT_VISUAL_ENFORCE must be baseline, strict or report, not {other}"),
    }
}

/// A fixed folder, because the campaign paths it holds appear in the UI. A lock
/// keeps another test process out while this one owns the folder.
fn world_root() -> (fs::File, PathBuf) {
    let temp = std::env::temp_dir();
    let lock = fs::File::create(temp.join("splat-visual-world.lock")).expect("create world lock");
    lock.lock().expect("lock world folder");
    let root = temp.join("splat-visual-world");
    if root.exists() {
        fs::remove_dir_all(&root).expect("clear the previous world");
    }
    fs::create_dir_all(&root).expect("create world folder");
    (lock, root)
}

fn run_scenario(
    root: &Path,
    output: Option<&Path>,
    record: bool,
    (name, seed, script): (&'static str, Seed, Script),
) -> Outcome {
    let cache = root.join("character-cache");
    // The same IDs on each run, and apart from those of other scenarios.
    let index = SCENARIOS
        .iter()
        .position(|(other, ..)| *other == name)
        .expect("listed scenario") as u128;
    crate::hermetic::closed::repeatable_uuids((index + 1) << 64);
    let world = World::build(&root.join(name), &cache, name, seed)
        .unwrap_or_else(|error| panic!("{name}: could not build its world: {error}"));
    let mut driver = Driver::new(&world, name, &root.join(name), output, record);
    script(&mut driver);
    driver.finish()
}

fn selected() -> Vec<(&'static str, Seed, Script)> {
    let filter = std::env::var("SPLAT_VISUAL_SCENARIOS").ok();
    let names: Option<Vec<&str>> = filter.as_deref().map(|v| v.split(',').collect());
    if let Some(names) = &names {
        for name in names {
            assert!(
                SCENARIOS.iter().any(|(other, ..)| other == name),
                "Unknown scenario {name}",
            );
        }
    }
    SCENARIOS
        .into_iter()
        .filter(|(name, ..)| names.as_ref().is_none_or(|names| names.contains(name)))
        .collect()
}

fn output_directory() -> (PathBuf, bool) {
    let explicit = std::env::var_os("SPLAT_VISUAL_DIR").map(PathBuf::from);
    let capture = explicit.is_some() || std::env::var_os("SPLAT_VISUAL_CAPTURE").is_some();
    (
        explicit.unwrap_or_else(|| PathBuf::from("artifacts/screenshots/latest")),
        capture,
    )
}

/// A partial run replaces only its own captures.
fn is_selected(scenario: &str) -> bool {
    selected().iter().any(|(name, ..)| *name == scenario)
}

/// Only files this suite writes. The folder can be one the caller chose.
fn clear_old_captures(directory: &Path) {
    let names = [
        "ergonomics.json",
        "ergonomics.md",
        "manifest.txt",
        "contact-sheet.png",
    ];
    let is_ours = |path: &Path| {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        names.contains(&name)
            || (name.ends_with(".png")
                && name
                    .split_once("__")
                    .is_some_and(|(scenario, _)| is_selected(scenario)))
    };
    for file in fs::read_dir(directory).into_iter().flatten().flatten() {
        if is_ours(&file.path()) {
            fs::remove_file(file.path()).expect("clear old output");
        }
    }
    for file in fs::read_dir(directory.join("video"))
        .into_iter()
        .flatten()
        .flatten()
    {
        let path = file.path();
        let stem = path
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if path.extension().is_some_and(|e| e == "mp4") && is_selected(stem) {
            fs::remove_file(&path).expect("clear old video");
        }
    }
}

/// Every scenario runs in one test, so the report and baseline see one whole run.
#[test]
fn visual_suite() {
    let started = Instant::now();
    let (directory, capture) = output_directory();
    if capture {
        clear_old_captures(&directory);
    }
    fs::create_dir_all(&directory).expect("create output folder");
    let record = capture && std::env::var("SPLAT_VISUAL_VIDEO").is_ok_and(|value| value == "1");
    if record {
        fs::create_dir_all(directory.join("video")).expect("create video folder");
    }
    let (_lock, root) = world_root();
    let cache = root.join("character-cache");
    fs::create_dir_all(&cache).expect("create character cache");
    crate::hermetic::closed::use_cache_root(cache);
    let scenarios = selected();
    let output = capture.then_some(directory.as_path());
    let outcomes: Vec<(&str, std::thread::Result<Outcome>)> = std::thread::scope(|scope| {
        let root = &root;
        let threads: Vec<_> = scenarios
            .iter()
            .map(|&scenario| {
                (
                    scenario.0,
                    scope.spawn(move || run_scenario(root, output, record, scenario)),
                )
            })
            .collect();
        threads
            .into_iter()
            .map(|(name, thread)| (name, thread.join()))
            .collect()
    });
    let mut violations = Vec::new();
    let mut passes = Vec::new();
    let mut costs = Vec::new();
    let mut screenshots = Vec::new();
    let mut unreached = Vec::new();
    let mut not_measured = Vec::new();
    let mut look_review = Vec::new();
    let mut videos = Vec::new();
    let mut failed = Vec::new();
    for (name, outcome) in outcomes {
        match outcome {
            Ok(outcome) => {
                violations.extend(outcome.violations);
                passes.extend(outcome.passes);
                costs.extend(outcome.task_costs);
                screenshots.extend(outcome.screenshots);
                unreached.extend(outcome.unreached);
                not_measured.extend(outcome.not_measured);
                look_review.extend(outcome.look_review);
                videos.extend(outcome.video);
            }
            Err(panic) => failed.push(format!(
                "{name}: {}",
                panic
                    .downcast_ref::<String>()
                    .map(String::as_str)
                    .or_else(|| panic.downcast_ref::<&str>().copied())
                    .unwrap_or("panicked")
                    .lines()
                    .next()
                    .unwrap_or("panicked")
            )),
        }
    }
    let names: Vec<&str> = scenarios.iter().map(|(name, ..)| *name).collect();
    let current = Baseline::measure(&violations, &costs);
    let known: Baseline = serde_json::from_str(BASELINE).expect("valid baseline.json");
    let (regressions, improvements) = known.compare(&current, &names);
    let stale: Vec<String> = improvements
        .iter()
        .filter(|line| !line.starts_with("task-cost"))
        .cloned()
        .collect();
    let totals: BTreeMap<String, usize> = current
        .violations
        .iter()
        .map(|(check, scenarios)| (check.clone(), scenarios.values().sum()))
        .collect();
    let report = Report {
        videos: &videos,
        not_measured: &not_measured,
        look_review: &look_review,
        totals: totals.clone(),
        by_check_and_scenario: &current.violations,
        task_cost: &costs,
        passes: &passes,
        unreached: &unreached,
        failed_scenarios: &failed,
        regressions: &regressions,
        improvements: &improvements,
        violations: &violations,
    };
    fs::write(
        directory.join("ergonomics.json"),
        serde_json::to_vec_pretty(&report).expect("encode report"),
    )
    .expect("write ergonomics.json");
    fs::write(
        directory.join("ergonomics.md"),
        ergonomics::write_markdown(&report),
    )
    .expect("write ergonomics.md");
    if capture {
        let mut manifest = String::new();
        screenshots.sort();
        for file in &screenshots {
            let bytes = fs::read(directory.join(file)).expect("read capture");
            manifest.push_str(&format!("{:x}  {file}\n", Sha256::digest(&bytes)));
        }
        fs::write(directory.join("manifest.txt"), manifest).expect("write manifest");
    }
    let _ = fs::remove_dir_all(&root);

    println!(
        "Visual suite: {} scenarios, {} screenshots, {} videos, {:?}. {totals:?}",
        names.len(),
        screenshots.len(),
        videos.len(),
        started.elapsed(),
    );
    for line in &improvements {
        println!("Improved: {line}");
    }
    if std::env::var_os("SPLAT_VISUAL_UPDATE_BASELINE").is_some() {
        assert!(
            failed.is_empty() && names.len() == SCENARIOS.len(),
            "Update the baseline only from a full, successful run: {failed:?}"
        );
        // A goal that failed is a bug, not an allowance.
        let workflow: Vec<_> = violations
            .iter()
            .filter(|violation| violation.check == "workflow")
            .map(|violation| violation.detail.as_str())
            .collect();
        assert!(
            workflow.is_empty(),
            "The baseline may not absorb a failed goal: {workflow:?}"
        );
        let mut text = serde_json::to_string_pretty(&current).expect("encode baseline");
        text.push('\n');
        fs::write(BASELINE_PATH, text).expect("write baseline");
        println!("Updated {BASELINE_PATH}");
        return;
    }
    assert!(
        failed.is_empty(),
        "Scenarios failed:\n{}",
        failed.join("\n")
    );
    // A run of every scenario must speak for every test in section 8. A deleted `observe`
    // would otherwise look like nothing at all.
    if names.len() == SCENARIOS.len() {
        let silent = silent_section8_tests(&passes, &violations, &not_measured, &look_review);
        assert!(
            silent.is_empty(),
            "Section 8 tests with no result in this run: {silent:?}"
        );
    }
    match enforcement() {
        Enforce::Report => {}
        // A count that fell must fall in the baseline too, or it can quietly grow back.
        Enforce::Baseline if names.len() == SCENARIOS.len() && !stale.is_empty() => panic!(
            "These allowances are larger than this run needs. Refresh the baseline:\n{}",
            stale.join("\n")
        ),
        Enforce::Baseline => assert!(
            regressions.is_empty(),
            "Ergonomics regressions against {BASELINE_PATH}:\n{}\nDetails: {}",
            regressions.join("\n"),
            directory.join("ergonomics.md").display(),
        ),
        Enforce::Strict => assert!(
            violations.is_empty(),
            "{} ergonomics violations in strict mode: {totals:?}\nDetails: {}",
            violations.len(),
            directory.join("ergonomics.md").display(),
        ),
    }
}

/// Every test ID in section 8 of the brief that this run has no line for.
fn silent_section8_tests(
    passes: &[ergonomics::Pass],
    violations: &[ergonomics::Violation],
    not_measured: &[String],
    look_review: &[String],
) -> Vec<String> {
    let brief = include_str!("../../docs/ux-intent.md");
    let mut ids: Vec<&str> = brief
        .lines()
        .filter_map(|line| line.strip_prefix("| "))
        .filter_map(|line| line.split(" |").next())
        .filter(|id| {
            let mut letters = id.chars();
            matches!(letters.next(), Some('T' | 'G'))
                && letters.next().is_some_and(|c| c.is_ascii_digit())
                && id.len() <= 5
        })
        .collect();
    ids.sort_unstable();
    ids.dedup();
    let spoken = |id: &str| {
        let says =
            |text: &str| text.starts_with(&format!("{id}:")) || text.starts_with(&format!("{id} "));
        passes.iter().any(|pass| says(&pass.detail))
            || violations.iter().any(|violation| violation.state == id)
            || not_measured.iter().any(|line| says(line))
            || look_review.iter().any(|line| says(line))
    };
    ids.into_iter()
        .filter(|id| !spoken(id))
        .map(str::to_owned)
        .collect()
}

/// The closed routes must stay closed, whatever any scenario does.
#[test]
fn test_builds_have_no_route_to_user_state() {
    use crate::{
        auth,
        credentials::{Action, Worker},
        persistence::Store,
    };
    use std::time::Duration;
    assert!(Store::user().is_err());
    assert!(auth::connect(zeroize::Zeroizing::new("synthetic".into())).is_err());
    assert_eq!(crate::hermetic::closed::now(), crate::hermetic::now());
    assert!(crate::hermetic::pick_folder().is_none());
    let worker = Worker::new(eframe::egui::Context::default());
    worker.send(1, Action::Restore).unwrap();
    let restored = worker.rx.recv_timeout(Duration::from_secs(5)).unwrap();
    assert!(restored.result.unwrap().session.is_none());
    worker
        .send(
            2,
            Action::Connect {
                cookie: zeroize::Zeroizing::new("synthetic".into()),
                remember: true,
            },
        )
        .unwrap();
    let connected = worker.rx.recv_timeout(Duration::from_secs(5)).unwrap();
    assert!(connected.result.is_err());
}
