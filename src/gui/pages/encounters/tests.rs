//! The Encounters page's screenshots and interaction tests. Captures are named
//! `<egui-scenario>__<step>__<w>x<h>` so they pair with `artifacts/screenshots/egui-reference/`.
use crate::Page;
use crate::gui::harness::Shell;
use crate::visual_tests::fixture::Seed;
use gpui::{px, size};
use std::time::Duration;

/// The page after its campaign is read (the read runs on a thread of its own).
fn ready(shell: &mut Shell) {
    shell.update(|root, _, cx| root.show_page(Page::Encounters, cx));
    for _ in 0..400 {
        let reading = shell.read(|root| {
            root.app.encounters.busy() || !root.app.encounters.loaded()
        });
        if !reading {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
        shell.wait(Duration::from_millis(50));
    }
    shell.settle();
}

fn small(name: &str, seed: Seed) -> Shell {
    Shell::with_size(name, seed, size(px(960.0), px(640.0)))
}

/// The list: In progress, Ready to run, Groups and History, at both sizes.
#[test]
fn encounter_list() {
    let mut shell = Shell::new("encounter-build", Seed::Campaign);
    ready(&mut shell);
    assert!(shell.bounds("Start fight").is_some(), "a Ready row's Start fight");
    assert!(shell.bounds("Resume").is_some(), "the paused run's Resume");
    shell.capture("encounter-build__01-encounter-list__1200x800");
    let mut shell = small("encounter-build", Seed::Campaign);
    ready(&mut shell);
    shell.capture("encounter-build__01-encounter-list__960x640");
}

/// History lists ended runs by session; Open reads one in place and Close closes it.
#[test]
fn history_opens_a_run_in_place() {
    let mut shell = Shell::new("encounter-history", Seed::History);
    ready(&mut shell);
    shell.capture("encounter-history__01-encounters-with-history__1200x800");
    let open = shell
        .read(|_| crate::gui::kit::probe::names())
        .into_iter()
        .find(|name| name.starts_with("id:history-open:"))
        .expect("an Open in History");
    shell.click(&open);
    assert!(shell.bounds("Final state").is_some() || shell.bounds("Continue fight").is_some());
    assert!(shell.read(|root| root.app.encounters.history_open()));
    shell.capture("encounter-history__02-history-open__1200x800");
    shell.click(&open);
    assert!(!shell.read(|root| root.app.encounters.history_open()));
}
