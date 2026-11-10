//! Manual load benchmark for real content. It is ignored, and it reads only the settings
//! file named by `SPLAT_BENCH_SETTINGS`, so normal test runs stay hermetic.
//!
//! `SPLAT_BENCH_SETTINGS=~/.config/splat/settings.json SPLAT_BENCH_PHASE=app \
//!  cargo test --locked --bin splat load_bench -- --ignored --nocapture`
//!
//! Phases:
//! - `app` (default): every page loader at once, as opening a campaign starts them.
//! - `builtin`: the embedded SRD and definitions.
//! - `read`: one read of each package and campaign folder.
//! - `library`, `packages`: one page's loader.
//!
//! Add `+warm` to load built-in content first, as a running app already has. Add `+read`
//! to read every folder first, as when a page loads again. To count work on a busy
//! computer, run the test binary under `perf stat -e instructions:u` with
//! `RAYON_NUM_THREADS=1`, so idle worker threads add nothing.
use crate::persistence::UserData;
use std::time::Instant;

fn time<T>(label: &str, run: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let value = run();
    eprintln!("{label}: {:?}", start.elapsed());
    value
}

#[test]
#[ignore = "reads the settings and content named by SPLAT_BENCH_SETTINGS"]
fn load_bench() {
    let path = std::env::var("SPLAT_BENCH_SETTINGS").expect("Set SPLAT_BENCH_SETTINGS");
    let data: UserData = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let campaign = data
        .campaigns
        .iter()
        .find(|c| Some(c.id) == data.active_campaign_id)
        .or(data.campaigns.first())
        .cloned()
        .expect("The settings name a campaign");
    let modules = data.modules.clone();
    let phase = std::env::var("SPLAT_BENCH_PHASE").unwrap_or_else(|_| "app".into());
    let mut parts = phase.split('+');
    let phase = parts.next().unwrap_or_default();
    let options: Vec<_> = parts.collect();
    let roots: Vec<_> = modules
        .iter()
        .map(|m| &m.path)
        .chain([&campaign.path])
        .collect();
    if phase == "builtin" || options.contains(&"warm") {
        time("builtin", || {
            splat::content::builtin::entries();
            splat::content::definitions::entries().unwrap();
        });
    }
    if options.contains(&"read") {
        for root in &roots {
            splat::content::read_directory(root).unwrap();
        }
    }
    let (modules, campaign) = (&modules, &campaign);
    match phase {
        "builtin" => {}
        "read" => {
            for root in &roots {
                time("read", || splat::content::read_directory(root).unwrap());
            }
        }
        "library" => {
            time("library", || {
                crate::library::load_all(modules, Some(campaign))
            })
            .unwrap();
        }
        "packages" => {
            time("packages", || {
                crate::campaign_content::load_snapshot(campaign, modules)
            })
            .2
            .unwrap();
        }
        "app" => time("app", || {
            std::thread::scope(|scope| {
                scope.spawn(|| {
                    time("  library", || {
                        crate::library::load_all(modules, Some(campaign))
                    })
                    .unwrap();
                });
                scope.spawn(|| {
                    time("  packages", || {
                        crate::campaign_content::load_snapshot(campaign, modules)
                    })
                    .2
                    .unwrap();
                });
                scope.spawn(|| {
                    time("  lookup", || {
                        crate::lookup::ContentIndex::load(modules, Some(campaign))
                    })
                    .unwrap();
                });
                scope.spawn(|| {
                    time("  encounters", || {
                        crate::encounters::load_for_bench(campaign, modules)
                    })
                    .unwrap();
                });
                scope.spawn(|| {
                    time("  package check", || {
                        crate::content_workspace::destinations(&campaign.path, modules)
                    });
                });
            });
        }),
        other => panic!("Unknown phase {other}"),
    }
}
