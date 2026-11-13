//! Visual and ergonomics tests. They run the real `Splat` app headlessly, drive it
//! through the AccessKit tree, and check every state a usage pattern reaches.
//!
//! `cargo test` runs every scenario and enforces the ergonomics baseline, without
//! a GPU or any screenshots. `devenv shell -- visual-test <label>` also renders
//! PNGs, on a GPU when one is available. See "Visual tests" in the README.
//!
//! Environment:
//! - `SPLAT_VISUAL_CAPTURE=1` renders PNGs. Setting `SPLAT_VISUAL_DIR` also does.
//! - `SPLAT_VISUAL_DIR` is the output folder. The default is
//!   `artifacts/screenshots/latest`.
//! - `SPLAT_VISUAL_ENFORCE` is `baseline` (default), `strict` (any violation fails)
//!   or `report` (never fails on a check).
//! - `SPLAT_VISUAL_UPDATE_BASELINE=1` rewrites `baseline.json` from this run.
//! - `SPLAT_VISUAL_SCENARIOS=party,settings` runs only those scenarios and collects
//!   every reached text expectation before failing. Action failures still stop a scenario.
//! - `SPLAT_VISUAL_DIAGNOSE=1` collects text expectations, then still fails the run.
//! - `SPLAT_VISUAL_TIMINGS=1` prints stage timings without changing captures.
//! - `SPLAT_VISUAL_JOBS` is how many scenarios run at once (default 4). Memory
//!   grows with it: a full capture run needs about 1.1 GiB at 4 and 1.6 GiB at 8.
//! - `SPLAT_VISUAL_SLOTS` overrides the machine-wide suite limit. The default
//!   comes from this computer's cores and RAM. Set the same positive count in
//!   every worktree; each slot needs full-suite RAM.
//! - Linux isolates temporary files with a mount namespace when available.
//!   `SPLAT_VISUAL_ISOLATE=0` retains the shared-world lock for comparison.
//! - `visual-baseline-chrome <label>` applies eligible saved chrome measurements.
//! - `SPLAT_VISUAL_VIDEO=1`, with captures on, also encodes one video per scenario
//!   with ffmpeg, into the output folder's `video/`.
//! - `SPLAT_VISUAL_ENCODER=vaapi|nvenc|x264` uses only that kind of video encoder.
mod compare;
mod dm_audit;
mod driver;
pub(crate) mod ergonomics;
pub(crate) mod fixture;
#[cfg(target_os = "linux")]
mod isolation;
mod plain_evening;
mod render;
mod repeated_captures;
mod scenarios;
#[cfg(target_os = "linux")]
mod settle_probe;
mod slots;
mod steps;
mod stillness;
mod stillness_known;
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

const DEFAULT_JOBS: usize = 4;

const SCENARIOS: [(&str, Seed, Script); 94] = [
    ("file-formats", Seed::Formats, scenarios::file_formats::run),
    ("stat-formats", Seed::Formats, scenarios::stat_formats::run),
    ("data-integrity", Seed::Campaign, scenarios::integrity::run),
    (
        "package-to-table",
        Seed::Campaign,
        scenarios::package_to_table::run,
    ),
    (
        "share-to-package",
        Seed::Campaign,
        scenarios::share_to_package::run,
    ),
    (
        "remove-package-in-use",
        Seed::Campaign,
        scenarios::remove_package_in_use::run,
    ),
    (
        "library-browse",
        Seed::Empty,
        scenarios::library_browse::run,
    ),
    (
        "creature-filter",
        Seed::Empty,
        scenarios::creature_filter::run,
    ),
    (
        "library-author",
        Seed::Campaign,
        scenarios::library_author::run,
    ),
    (
        "library-navigation",
        Seed::Campaign,
        scenarios::library_navigation::run,
    ),
    (
        "reader-tables",
        Seed::Campaign,
        scenarios::reader_tables::run,
    ),
    (
        "campaign-content",
        Seed::Campaign,
        scenarios::campaign_content::run,
    ),
    (
        "npc-in-encounter",
        Seed::Campaign,
        scenarios::npc_in_encounter::run,
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
    ("run-rolls", Seed::Campaign, scenarios::run_rolls::run),
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
    ("run-effects", Seed::Campaign, scenarios::run_effects::run),
    (
        "session-notes",
        Seed::Campaign,
        scenarios::session_notes::run,
    ),
    ("world", Seed::World, scenarios::world::run),
    (
        "world-broken-note",
        Seed::World,
        scenarios::world_build::broken,
    ),
    (
        "npc-in-lookup",
        Seed::Campaign,
        scenarios::world_build::npc_in_lookup,
    ),
    (
        "package-rename",
        Seed::Campaign,
        scenarios::world_build::package_rename,
    ),
    (
        "package-about",
        Seed::Campaign,
        scenarios::world_build::package_about,
    ),
    (
        "package-manifest-broken",
        Seed::Campaign,
        scenarios::world_build::package_manifest_broken,
    ),
    (
        "package-file-changed",
        Seed::Campaign,
        scenarios::world_build::package_file_changed,
    ),
    (
        "package-delete-entry",
        Seed::Campaign,
        scenarios::world_build::package_delete_entry,
    ),
    (
        "package-duplicate-id",
        Seed::Campaign,
        scenarios::world_build::package_duplicate_id,
    ),
    (
        "book-to-package",
        Seed::Campaign,
        scenarios::world_build::book_to_package,
    ),
    (
        "world-note-duplicated",
        Seed::Campaign,
        scenarios::world_build::world_note_duplicated,
    ),
    (
        "world-note-renamed",
        Seed::Campaign,
        scenarios::world_build::world_note_renamed,
    ),
    (
        "package-folder-copied",
        Seed::Campaign,
        scenarios::world_build::package_folder_copied,
    ),
    ("world-places", Seed::World, scenarios::world_build::places),
    ("world-facts", Seed::World, scenarios::world_build::facts),
    (
        "world-note-moved",
        Seed::World,
        scenarios::world_build::world_note_moved,
    ),
    (
        "world-note-deleted",
        Seed::World,
        scenarios::world_build::world_note_deleted,
    ),
    (
        "world-empty-links",
        Seed::World,
        scenarios::world_build::empty_links,
    ),
    (
        "world-part-of-safety",
        Seed::World,
        scenarios::world_build::part_of_safety,
    ),
    (
        "world-edit-round-trip",
        Seed::World,
        scenarios::world_build::round_trip,
    ),
    (
        "roll-table-author",
        Seed::World,
        scenarios::world_build::tables,
    ),
    ("travel-checks", Seed::World, scenarios::travel_checks::run),
    (
        "generator-tables",
        Seed::World,
        scenarios::world_build::generator,
    ),
    (
        "world-held-npc",
        Seed::World,
        scenarios::world_build::held_npc,
    ),
    (
        "markdown-links",
        Seed::World,
        scenarios::markdown_links::run,
    ),
    (
        "session-recording",
        Seed::Campaign,
        scenarios::session_recording::run,
    ),
    (
        "recording-banner-priority",
        Seed::Campaign,
        scenarios::session_recording::banner_priority,
    ),
    (
        "encounter-groups",
        Seed::Groups,
        scenarios::encounter_groups::run,
    ),
    (
        "encounter-owned-groups",
        Seed::OwnedGroups,
        scenarios::encounter_owned_groups::run,
    ),
    ("run-overrides", Seed::Groups, scenarios::run_overrides::run),
    (
        "encounter-history",
        Seed::History,
        scenarios::encounter_history::run,
    ),
    (
        "run-no-groups",
        Seed::Campaign,
        scenarios::run_no_groups::run,
    ),
    (
        "party-bar-crowded",
        Seed::Campaign,
        scenarios::party_bar_crowded::run,
    ),
    ("session-plan", Seed::Campaign, scenarios::session_plan::run),
    (
        "session-plan-groups",
        Seed::Groups,
        scenarios::session_plan_groups::run,
    ),
    (
        "run-add-creature",
        Seed::Groups,
        scenarios::run_add_creature::run,
    ),
    (
        "list-formats",
        Seed::ListFormats,
        scenarios::list_formats::run,
    ),
    ("run-loot", Seed::Loot, scenarios::run_loot::run),
    // New scenarios go last: a scenario's repeatable IDs and dice start from its place here.
    (
        "library-file-fault",
        Seed::Campaign,
        scenarios::world_build::library_file_fault,
    ),
    (
        "campaign-copy-registered",
        Seed::Campaign,
        scenarios::world_build::campaign_copy_registered,
    ),
    // G6: the frozen evening of `artifacts/dm-audit/`.
    ("dm-audit", Seed::Campaign, dm_audit::run),
    // G7: the plain campaign against its reference.
    ("plain-evening", Seed::Plain, plain_evening::run),
    // G6 too: the other journeys of the workflow score, each against its own frozen counts.
    ("journey-j2", Seed::Campaign, dm_audit::journeys::j2),
    ("journey-j3", Seed::Campaign, dm_audit::journeys::j3),
    ("journey-j4", Seed::Campaign, dm_audit::journeys::j4),
    ("journey-j6", Seed::Campaign, dm_audit::journeys::j6),
    ("run-continue", Seed::Loot, scenarios::run_continue::run),
    (
        "encounter-difficulty",
        Seed::Campaign,
        scenarios::encounter_difficulty::run,
    ),
    (
        "session-plan-scenes",
        Seed::Campaign,
        scenarios::session_plan_scenes::run,
    ),
    (
        "log-from-play",
        Seed::Campaign,
        scenarios::session_plan_scenes::log_from_play,
    ),
    (
        "session-carry-over",
        Seed::Campaign,
        scenarios::session_carry_over::run,
    ),
    ("trackers", Seed::Campaign, scenarios::trackers::run),
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
            || self.art.busy()
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

/// The first line of a panic's message.
fn panic_line(panic: &(dyn std::any::Any + Send)) -> &str {
    panic
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| panic.downcast_ref::<&str>().copied())
        .unwrap_or("panicked")
        .lines()
        .next()
        .unwrap_or("panicked")
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
/// keeps another test process out while this one owns the folder. On Linux this
/// is inside a private mount namespace when supported, with private fixtures on
/// the same temporary filesystem.
fn world_root(locked: Option<fs::File>) -> (fs::File, PathBuf) {
    let temp = std::env::temp_dir();
    let started = Instant::now();
    let lock = locked.unwrap_or_else(|| {
        let lock =
            fs::File::create(temp.join("splat-visual-world.lock")).expect("create world lock");
        lock.lock().expect("lock world folder");
        lock
    });
    if std::env::var_os("SPLAT_VISUAL_TIMINGS").is_some() {
        println!("Timing world lock wait = {:?}", started.elapsed());
    }
    let root = temp.join("splat-visual-world");
    if root.exists() {
        fs::remove_dir_all(&root).expect("clear the previous world");
    }
    fs::create_dir_all(&root).expect("create world folder");
    (lock, root)
}

/// FNV-1a, 64 bit, over the exact UTF-8 bytes. A fixed algorithm, unlike Rust's
/// `DefaultHasher`, so a scenario keeps its IDs from one toolchain to the next.
fn name_hash(name: &str) -> u64 {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    name.bytes().fold(OFFSET, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(PRIME)
    })
}

/// A scenario's first generated ID: the hash of its name in the high half, with the
/// low half left for the IDs it generates. Run creation takes its dice seed from the
/// high half. The same IDs and dice on each run, whatever scenarios are added, removed
/// or moved in `SCENARIOS`, and apart from those of every other scenario. A renamed
/// scenario gets new IDs and new dice.
fn first_uuid(scenarios: &[&str], name: &str) -> u128 {
    assert_eq!(
        scenarios.iter().filter(|other| **other == name).count(),
        1,
        "{name} must be listed once"
    );
    let hash = name_hash(name);
    assert_ne!(hash, 0, "{name} hashes to the nil ID: rename it");
    if let Some(other) = scenarios
        .iter()
        .find(|other| **other != name && name_hash(other) == hash)
    {
        panic!("{name} and {other} hash to the same IDs: rename one");
    }
    u128::from(hash) << 64
}

/// A scenario's IDs and dice come from its name alone. Other scenarios, their order
/// and a focused selection change nothing.
#[test]
fn a_scenarios_ids_come_from_its_name_alone() {
    // Published FNV-1a vectors and one scenario's start, so the algorithm cannot drift.
    assert_eq!(name_hash(""), 0xcbf2_9ce4_8422_2325);
    assert_eq!(name_hash("a"), 0xaf63_dc4c_8601_ec8c);
    assert_eq!(name_hash("foobar"), 0x8594_4171_f739_67e8);
    let names: Vec<&str> = SCENARIOS.iter().map(|(name, ..)| *name).collect();
    let name = "encounter-owned-groups";
    let start = first_uuid(&names, name);
    let position = names
        .iter()
        .position(|other| *other == name)
        .expect("listed");
    assert!(position > 0, "a scenario before it can be removed");
    let mut inserted = names.clone();
    inserted.insert(0, "a-new-first-scenario");
    let mut removed = names.clone();
    removed.remove(0);
    let mut reversed = names.clone();
    reversed.reverse();
    for list in [inserted, removed, reversed, vec![name]] {
        assert_eq!(first_uuid(&list, name), start, "{list:?}");
    }
    assert_eq!(start, 0x778e_2ce9_ee43_99d9_u128 << 64);
    // Each listed scenario has IDs of its own, none of them the nil ID.
    let mut hashes: Vec<u64> = names.iter().map(|name| name_hash(name)).collect();
    hashes.sort_unstable();
    hashes.dedup();
    assert_eq!(hashes.len(), names.len(), "two scenarios share IDs");
    for name in &names {
        first_uuid(&names, name);
    }
    // A name listed twice is refused, not shared.
    let twice = std::panic::catch_unwind(|| first_uuid(&["party", "party"], "party"));
    assert!(twice.is_err());
}

/// Lines of `text` that make a random ID directly. Scenario code takes each new ID from
/// `crate::hermetic::new_uuid`, which follows the scenario's seed, so two runs show the same IDs.
fn random_id_lines(text: &str) -> Vec<usize> {
    // Built from parts, so this function's own file does not match.
    let calls = [concat!("Uuid::", "new_v4"), concat!("Uuid::", "now_v7")];
    text.lines()
        .enumerate()
        .filter(|(_, line)| calls.iter().any(|call| line.contains(call)))
        .map(|(at, _)| at + 1)
        .collect()
}

/// Every Rust file under `src/visual_tests/`, the scenarios included.
fn visual_test_sources() -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    let mut folders = vec![Path::new(env!("CARGO_MANIFEST_DIR")).join("src/visual_tests")];
    while let Some(folder) = folders.pop() {
        for entry in std::fs::read_dir(&folder).expect("read src/visual_tests") {
            let path = entry.expect("read a visual test entry").path();
            if path.is_dir() {
                folders.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// Visual test code makes no random ID: a random ID differs on every run, so its captures
/// differ too.
#[test]
fn visual_test_code_takes_ids_from_the_scenario_seed() {
    let files = visual_test_sources();
    // The scan reads the scenario files, so an empty scan cannot pass.
    for scenario in ["scenarios.rs", "scenarios/world_build.rs"] {
        assert!(
            files.iter().any(|file| file.ends_with(scenario)),
            "the scan missed {scenario}: {files:?}"
        );
    }
    // It finds a direct call, in either form.
    let random = concat!("let id = uuid::Uuid::", "new_v4();\n");
    let unwrap = concat!("x.unwrap_or_else(Uuid::", "new_v4)\n");
    assert_eq!(random_id_lines(&format!("ok\n{random}{unwrap}")), [2, 3]);
    assert!(random_id_lines("crate::hermetic::new_uuid()\n").is_empty());
    let found: Vec<String> = files
        .iter()
        .flat_map(|file| {
            let text = std::fs::read_to_string(file).expect("read a visual test file");
            random_id_lines(&text)
                .into_iter()
                .map(|line| format!("{}:{line}", file.display()))
                .collect::<Vec<_>>()
        })
        .collect();
    assert!(
        found.is_empty(),
        "Use crate::hermetic::new_uuid() for a new ID in visual test code: {found:?}"
    );
}

fn run_scenario(
    root: &Path,
    output: Option<&Path>,
    record: bool,
    (name, seed, script): (&'static str, Seed, Script),
) -> Outcome {
    let cache = root.join("character-cache");
    let names: Vec<&str> = SCENARIOS.iter().map(|(name, ..)| *name).collect();
    // Workers run several scenarios, so start each from clean thread state.
    crate::hermetic::closed::repeatable_uuids(first_uuid(&names, name));
    crate::hermetic::closed::clear_next_folder();
    let started = Instant::now();
    let world = World::build(&root.join(name), &cache, name, seed)
        .unwrap_or_else(|error| panic!("{name}: could not build its world: {error}"));
    if std::env::var_os("SPLAT_VISUAL_TIMINGS").is_some() {
        println!("Timing {name}: fixture = {:?}", started.elapsed());
    }
    let started = Instant::now();
    let mut driver = Driver::new(&world, name, &root.join(name), output, record);
    script(&mut driver);
    let outcome = driver.finish();
    if std::env::var_os("SPLAT_VISUAL_TIMINGS").is_some() {
        println!("Timing {name}: driver + script = {:?}", started.elapsed());
    }
    outcome
}

/// How many scenarios run at once. Each live scenario holds a whole app and its
/// renderer, so this, not the scenario count, sets the suite's memory.
fn jobs() -> usize {
    match std::env::var("SPLAT_VISUAL_JOBS") {
        Ok(value) => value
            .parse()
            .ok()
            .filter(|count| *count > 0)
            .expect("SPLAT_VISUAL_JOBS must be a positive integer"),
        Err(_) => DEFAULT_JOBS,
    }
}

/// Rust code in test builds uses jemalloc, but Mesa's renderers still
/// uses glibc's malloc. glibc gives busy threads their own heap arenas and keeps
/// the memory they free. It also raises its mmap threshold after a large free,
/// so large buffers then stay in those arenas. One arena per worker and a fixed
/// threshold keep that memory small. Before jemalloc, these settings alone took
/// a four-worker capture run from 2.2 to 0.8 GiB at the same speed.
/// An arena limit applies only before a process has nine arenas. In the isolated
/// child that is always so; in the shared fallback it can come too late.
#[cfg(all(target_os = "linux", target_env = "gnu"))]
fn limit_allocator(jobs: usize) {
    // SAFETY: mallopt only changes allocator settings; it is safe on any thread.
    unsafe {
        libc::mallopt(
            libc::M_ARENA_MAX,
            libc::c_int::try_from(jobs).unwrap_or(libc::c_int::MAX),
        );
        libc::mallopt(libc::M_MMAP_THRESHOLD, 1 << 20);
    }
}

/// A fixed set of workers takes the scenarios in list order. The results keep
/// that order, whichever scenario finishes first.
fn run_scenarios(
    scenarios: &[(&'static str, Seed, Script)],
    root: &Path,
    output: Option<&Path>,
    record: bool,
) -> Vec<(&'static str, std::thread::Result<Outcome>)> {
    let jobs = jobs();
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    limit_allocator(jobs);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let results = std::sync::Mutex::new(Vec::from_iter(scenarios.iter().map(|_| None)));
    std::thread::scope(|scope| {
        let mut workers = Vec::new();
        for _ in 0..jobs.min(scenarios.len()) {
            workers.push(scope.spawn(|| {
                loop {
                    let index = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let Some(&scenario) = scenarios.get(index) else {
                        break;
                    };
                    // A panic ends only this scenario, as a thread's panic did.
                    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        run_scenario(root, output, record, scenario)
                    }));
                    results.lock().unwrap_or_else(|e| e.into_inner())[index] =
                        Some((scenario.0, outcome));
                }
            }));
        }
        // A scope can end before its threads have exited. A worker's render device is
        // thread-local: it drops as the thread exits, and the Vulkan loader then unloads the
        // driver. The process must not exit before that is over, or an exit handler can run
        // in a driver that is being unloaded (SIGSEGV). `join` waits for the whole exit.
        for worker in workers {
            worker
                .join()
                .expect("a worker catches each scenario's panic");
        }
    });
    results
        .into_inner()
        .unwrap_or_else(|e| e.into_inner())
        .into_iter()
        .map(|result| result.expect("every scenario ran"))
        .collect()
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

/// Reports describe only a completed run, including runs without captures.
fn prepare_output(directory: &Path, capture: bool) {
    fs::create_dir_all(directory).expect("create output folder");
    for name in [
        "ergonomics.json",
        "ergonomics.md",
        "chrome-observation.json",
        "stillness.json",
    ] {
        match fs::remove_file(directory.join(name)) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!("clear previous {name}: {error}"),
        }
    }
    if capture {
        clear_old_captures(directory);
    }
}

#[test]
fn a_run_without_captures_clears_reports_but_keeps_other_files() {
    let directory = tempfile::tempdir().unwrap();
    let reports = [
        "ergonomics.json",
        "ergonomics.md",
        "chrome-observation.json",
        "stillness.json",
    ];
    let retained = ["empty__01__960x640.png", "manifest.txt", "notes.md"];
    for name in reports.into_iter().chain(retained) {
        fs::write(directory.path().join(name), "previous run").unwrap();
    }
    prepare_output(directory.path(), false);
    for name in reports {
        assert!(
            !directory.path().join(name).exists(),
            "H-report-cleared: {name}"
        );
    }
    for name in retained {
        assert_eq!(
            fs::read_to_string(directory.path().join(name)).unwrap(),
            "previous run",
            "H-output-retained: {name}",
        );
    }
    // A first run, or a run after an early exit, need not have any reports.
    prepare_output(directory.path(), false);
}

/// A partial run replaces only its own captures.
fn is_selected(scenario: &str) -> bool {
    selected().iter().any(|(name, ..)| *name == scenario)
}

/// Only files this suite writes. The folder can be one the caller chose.
fn clear_old_captures(directory: &Path) {
    let names = ["manifest.txt", "renderer.txt", "contact-sheet.png"];
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

#[derive(serde::Serialize)]
struct ChromeObservation<'a> {
    scenarios: Vec<String>,
    known: &'a Baseline,
    current: &'a Baseline,
}

/// Apply measurements without running scenarios or accepting app/task allowances.
#[test]
#[ignore = "Use visual-baseline-chrome <label> after reviewing a completed run"]
fn visual_update_chrome() {
    #[derive(serde::Deserialize)]
    struct Observation {
        scenarios: Vec<String>,
        known: Baseline,
        current: Baseline,
    }
    let directory =
        std::env::var_os("SPLAT_VISUAL_BASELINE_RUN").expect("Use visual-baseline-chrome <label>");
    let observation: Observation = serde_json::from_slice(
        &fs::read(Path::new(&directory).join("chrome-observation.json"))
            .expect("No eligible completed run: fix scenario/workflow/app failures and rerun"),
    )
    .expect("valid chrome measurements");
    let names: Vec<&str> = observation.scenarios.iter().map(String::as_str).collect();
    assert!(
        !names.is_empty()
            && names
                .iter()
                .all(|name| SCENARIOS.iter().any(|s| s.0 == *name)),
        "unknown or empty scenarios"
    );
    let previous = fs::read(BASELINE_PATH).expect("read baseline");
    let text = chrome_refresh(&previous, &observation.known, &observation.current, &names)
        .unwrap_or_else(|error| panic!("{BASELINE_PATH}: {error}"));
    // An unchanged include_str! input must not trigger a needless test rebuild.
    if previous != text.as_bytes() {
        fs::write(BASELINE_PATH, text).expect("write baseline");
    }
    println!(
        "Applied chrome counts for {}. Run the full merge gates before merging.",
        names.join(", ")
    );
}

/// The baseline text after a chrome refresh. It changes only chrome counts, and it
/// refuses a baseline on disk that changed since the run or that allows a fault.
fn chrome_refresh(
    previous: &[u8],
    known: &Baseline,
    current: &Baseline,
    names: &[&str],
) -> Result<String, String> {
    let disk = Baseline::parse(previous)?;
    if disk != *known {
        return Err("Baseline changed since this run; rerun before updating".into());
    }
    let updated = disk.with_chrome_from(current, names);
    updated.refuse_never_allowed()?;
    let mut text = serde_json::to_string_pretty(&updated).expect("encode baseline");
    text.push('\n');
    Ok(text)
}

/// The baseline text that a full run writes. Failed scenarios, failed goals and faults
/// that no baseline may allow refuse it.
fn full_baseline(
    current: &Baseline,
    violations: &[ergonomics::Violation],
    failed: &[String],
    every_scenario: bool,
) -> Result<String, String> {
    if !failed.is_empty() || !every_scenario {
        return Err(format!(
            "Update the baseline only from a full, successful run: {failed:?}"
        ));
    }
    // A goal that failed is a bug, not an allowance.
    let workflow: Vec<_> = violations
        .iter()
        .filter(|violation| violation.check == "workflow")
        .map(|violation| violation.detail.as_str())
        .collect();
    if !workflow.is_empty() {
        return Err(format!(
            "The baseline may not absorb a failed goal: {workflow:?}"
        ));
    }
    current.refuse_never_allowed()?;
    let mut text = serde_json::to_string_pretty(current).expect("encode baseline");
    text.push('\n');
    Ok(text)
}

fn mixed_overlap_run() -> Baseline {
    serde_json::from_value(serde_json::json!({
        "violations": {
            "no-overlap/egui-chrome": {"party": 4},
            ergonomics::APP_VS_CHROME: {"party": 2}
        },
        "task_cost": {}
    }))
    .unwrap()
}

/// A chrome refresh cannot carry a mixed overlap into the baseline, and refuses a
/// baseline on disk that allows one.
#[test]
fn a_chrome_refresh_refuses_a_mixed_overlap() {
    let known = Baseline::default();
    let previous = serde_json::to_vec(&known).unwrap();
    let current = mixed_overlap_run();
    let text = chrome_refresh(&previous, &known, &current, &["party"]).unwrap();
    let refreshed = Baseline::parse(text.as_bytes()).unwrap();
    assert_eq!(refreshed.violations["no-overlap/egui-chrome"]["party"], 4);
    assert!(
        !refreshed.violations.contains_key(ergonomics::APP_VS_CHROME),
        "{text}"
    );
    let allowed = br#"{"violations": {"no-overlap/app-vs-chrome": {"party": 2}}, "task_cost": {}}"#;
    let disk: Baseline = serde_json::from_slice(allowed).unwrap();
    let error = chrome_refresh(allowed, &disk, &current, &["party"])
        .expect_err("a baseline that allows the fault is refused");
    assert!(
        error.contains("no-overlap/app-vs-chrome / party: 2"),
        "{error}"
    );
}

/// The full baseline writer refuses a run with a mixed overlap, and writes one without.
#[test]
fn the_full_baseline_writer_refuses_a_mixed_overlap() {
    let error = full_baseline(&mixed_overlap_run(), &[], &[], true)
        .expect_err("a mixed overlap is never written");
    assert!(
        error.contains("no-overlap/app-vs-chrome / party: 2"),
        "{error}"
    );
    let mut clean = mixed_overlap_run();
    clean.violations.remove(ergonomics::APP_VS_CHROME);
    let text = full_baseline(&clean, &[], &[], true).unwrap();
    assert_eq!(Baseline::parse(text.as_bytes()).unwrap(), clean);
}

/// Every scenario runs in one test, so the report and baseline see one whole run.
#[test]
fn visual_suite() {
    let mut slot = slots::acquire();
    #[cfg(target_os = "linux")]
    if isolation::run() {
        return;
    }
    let started = Instant::now();
    let (directory, capture) = output_directory();
    prepare_output(&directory, capture);
    let chrome_observation = directory.join("chrome-observation.json");
    let record = capture && std::env::var("SPLAT_VISUAL_VIDEO").is_ok_and(|value| value == "1");
    if record {
        fs::create_dir_all(directory.join("video")).expect("create video folder");
        // Choose the encoder before the workers start, so no scenario waits for the probe.
        let encoder = video::encoder();
        if std::env::var_os("SPLAT_VISUAL_TIMINGS").is_some() {
            println!("Video encoder = {encoder}");
        }
    }
    let (_lock, root) = world_root(slot.as_mut().and_then(slots::Permit::take_world_lock));
    let cache = root.join("character-cache");
    fs::create_dir_all(&cache).expect("create character cache");
    crate::hermetic::closed::use_cache_root(cache);
    let scenarios = selected();
    let output = capture.then_some(directory.as_path());
    let outcomes = run_scenarios(&scenarios, &root, output, record);
    let mut violations = Vec::new();
    let mut passes = Vec::new();
    let mut costs = Vec::new();
    let mut screenshots = Vec::new();
    let mut unreached = Vec::new();
    let mut not_measured = Vec::new();
    let mut look_review = Vec::new();
    let mut videos = Vec::new();
    let mut failed = Vec::new();
    let mut still = Vec::new();
    let mut still_escapes = Vec::new();
    let mut still_time = std::time::Duration::ZERO;
    for (name, outcome) in outcomes {
        match outcome {
            Ok(outcome) => {
                failed.extend(outcome.failed_expectations);
                violations.extend(outcome.violations);
                passes.extend(outcome.passes);
                costs.extend(outcome.task_costs);
                screenshots.extend(outcome.screenshots);
                unreached.extend(outcome.unreached);
                not_measured.extend(outcome.not_measured);
                look_review.extend(outcome.look_review);
                videos.extend(outcome.video);
                still.extend(outcome.stillness);
                still_escapes.extend(outcome.stillness_escapes);
                still_time += outcome.stillness_time;
            }
            Err(panic) => failed.push(format!("{name}: {}", panic_line(panic.as_ref()))),
        }
    }
    let names: Vec<&str> = scenarios.iter().map(|(name, ..)| *name).collect();
    let current = Baseline::measure(&violations, &costs);
    let known = Baseline::parse(BASELINE.as_bytes())
        .unwrap_or_else(|error| panic!("{BASELINE_PATH}: {error}"));
    let (mut regressions, improvements) = known.compare(&current, &names);
    // A new mixed pair, or a known one that is fixed, fails like a regression.
    let (mixed, fixed) =
        ergonomics::compare_app_vs_chrome(&violations, ergonomics::KNOWN_APP_VS_CHROME, &names);
    regressions.extend(mixed);
    regressions.extend(fixed);
    // A blocking G check's line that the known list lacks, or a stale line, fails like a
    // regression.
    let (still_worse, still_stale) =
        stillness::compare_known(&still, stillness_known::KNOWN, &names);
    regressions.extend(still_worse);
    regressions.extend(still_stale);
    let known_open: Vec<String> = ergonomics::KNOWN_APP_VS_CHROME
        .iter()
        .filter(|((scenario, ..), _)| names.contains(scenario))
        .map(|((scenario, state, window), count)| {
            format!("{scenario} / {state} / {window}: {count}")
        })
        .collect();
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
        known_open: &known_open,
        violations: &violations,
    };
    fs::write(
        directory.join("ergonomics.json"),
        serde_json::to_vec_pretty(&report).expect("encode report"),
    )
    .expect("write ergonomics.json");
    let still_report =
        stillness::Report::new(&still, &still_escapes, stillness_known::KNOWN, still_time);
    fs::write(
        directory.join("stillness.json"),
        stillness::json_lines(&still),
    )
    .expect("write stillness.json");
    fs::write(
        directory.join("ergonomics.md"),
        ergonomics::write_markdown(&report) + &still_report.markdown(),
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
        if let Some(adapter) = render::adapter() {
            let mut text = format!("{adapter}\n");
            if record {
                text.push_str(&format!("video: {}\n", video::encoder()));
            }
            fs::write(directory.join("renderer.txt"), text).expect("write renderer.txt");
        }
    }
    let _ = fs::remove_dir_all(&root);

    println!(
        "Visual suite: {} scenarios, {} screenshots, {} videos, {:?}. {totals:?}",
        names.len(),
        screenshots.len(),
        videos.len(),
        started.elapsed(),
    );
    // Print every class before any assertion can hide another class of failure.
    for line in &regressions {
        println!("Regression: {line}");
    }
    for line in &failed {
        println!("Scenario failed: {line}");
    }
    for line in &improvements {
        println!("Improved: {line}");
    }
    for line in &known_open {
        println!("Known app-vs-chrome pair (the list must end empty): {line}");
    }
    println!("{}", still_report.summary());
    let silent = if names.len() == SCENARIOS.len() {
        silent_section8_tests(&passes, &violations, &not_measured, &look_review)
    } else {
        Vec::new()
    };
    if failed.is_empty()
        && silent.is_empty()
        && !violations.iter().any(|v| v.check == "workflow")
        && regressions.iter().all(|line| {
            line.split(" / ")
                .next()
                .is_some_and(|check| check.ends_with(ergonomics::CHROME_SUFFIX))
        })
    {
        let observation = ChromeObservation {
            scenarios: names.iter().map(|name| (*name).to_owned()).collect(),
            known: &known,
            current: &current,
        };
        fs::write(
            &chrome_observation,
            serde_json::to_vec_pretty(&observation).expect("encode chrome measurements"),
        )
        .expect("write chrome measurements");
        println!(
            "Chrome measurements: {}. Apply with visual-baseline-chrome <label> after review.",
            chrome_observation.display()
        );
        if (!regressions.is_empty() || !stale.is_empty())
            && stale.iter().all(|line| {
                line.split(" / ")
                    .next()
                    .is_some_and(|check| check.ends_with(ergonomics::CHROME_SUFFIX))
            })
            && let Ok(hint) = std::env::var("SPLAT_VISUAL_CHROME_HINT")
        {
            println!("{hint}");
        }
    }
    if std::env::var_os("SPLAT_VISUAL_UPDATE_BASELINE").is_some() {
        let text = full_baseline(
            &current,
            &violations,
            &failed,
            names.len() == SCENARIOS.len(),
        )
        .unwrap_or_else(|error| panic!("{error}"));
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
