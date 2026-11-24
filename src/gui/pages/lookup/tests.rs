//! Lookup: screenshots of the panel beside a page (egui `run-lookup`), and its flows: Ctrl+K
//! opens it with the field focused, typing finds and opens the best match, the arrows move
//! through the results, Back returns, Escape closes.
use crate::gui::harness::Shell;
use crate::visual_tests::fixture::Seed;

fn wait_for_index(shell: &mut Shell) {
    for _ in 0..200 {
        if shell.read(|root| root.app.lookup.index().is_some()) {
            return;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    panic!("Lookup's index was not read");
}

#[test]
fn search_open_and_back() {
    let mut shell = Shell::new("run-lookup", Seed::Campaign);
    shell.press("ctrl-k");
    assert!(shell.read(|root| root.app.lookup.open));
    wait_for_index(&mut shell);
    shell.capture("run-lookup__00-empty-search__1200x800");
    shell.type_text("fire");
    assert_eq!(shell.read(|root| root.app.lookup.query().to_owned()), "fire");
    let count = shell.read(|root| root.app.lookup.match_count());
    assert!(count > 1, "fire finds several entries: {count}");
    // The best match opens while the others stay listed.
    assert!(shell.read(|root| matches!(
        root.app.lookup.showing(),
        crate::lookup::panel_view::Showing::Entry(_)
    )));
    assert!(shell.read(|root| root.app.lookup.results_shown()));
    shell.capture("run-lookup__02-reading-spell-beside-run__1200x800");
    let first = shell.read(entry_name);
    shell.press("down");
    let second = shell.read(entry_name);
    assert_ne!(first, second, "Down opens the next match");
    // A press on a row reads it and closes the list.
    shell.click("id:lookup-result-0");
    assert_eq!(shell.read(entry_name), first);
    assert!(!shell.read(|root| root.app.lookup.results_shown()));
    shell.press("escape");
    assert!(
        !shell.read(|root| root.app.lookup.open),
        "Escape after a row press closes Lookup"
    );
}

/// Escape in the search field closes Lookup.
#[test]
fn escape_in_the_field_closes() {
    let mut shell = Shell::new("lookup-escape", Seed::Campaign);
    shell.press("ctrl-k");
    wait_for_index(&mut shell);
    shell.type_text("fire");
    shell.press("escape");
    assert!(!shell.read(|root| root.app.lookup.open));
}

/// Nothing found says so, and offers New NPC for the typed name.
#[test]
fn nothing_found_offers_new_entries() {
    let mut shell = Shell::new("lookup-new", Seed::Campaign);
    shell.press("ctrl-k");
    wait_for_index(&mut shell);
    shell.type_text("Zzyzx Qwerty");
    assert_eq!(shell.read(|root| root.app.lookup.match_count()), 0);
    assert!(shell.bounds("New NPC “Zzyzx Qwerty”").is_some());
    shell.capture("lookup-new__01-nothing-found__1200x800");
}

fn entry_name(root: &crate::gui::SplatRoot) -> Option<String> {
    let crate::lookup::panel_view::Showing::Entry(at) = root.app.lookup.showing() else {
        return None;
    };
    root.app
        .lookup
        .index()
        .and_then(|index| index.items.get(at).map(|item| item.entry.name.clone()))
}
