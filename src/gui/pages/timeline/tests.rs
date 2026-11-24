//! The Timeline page: add, name, rule, remove and their Undos, the filters, Ctrl+S, a journey's
//! encounter checks, and the captures (`<egui-scenario>__<step>`, paired with the references).
use crate::gui::harness::Shell;
use crate::timeline::Kind;
use crate::visual_tests::fixture::Seed;
use gpui::{px, size};
use std::fs;

const ROAD: &str = "# Road\n\n## On the road\n\n<!-- splat:rolls key=road dice=1d20 per=day -->\n\n| d20 | On the road |\n| --- | --- |\n| 1-14 | Nothing stirs. |\n| 15-17 | [[Bridge Ambush]] waits by the old quarry gate, where the road narrows between walls of loose stone. |\n| 18-20 | A toll gang blocks the way: [[Shrine of Cinders]]. |\n";

fn events(shell: &mut Shell) -> Vec<crate::timeline::Event> {
    shell.read(|root| root.app.timeline.events.clone())
}

fn selected(shell: &mut Shell) -> Option<String> {
    shell.read(|root| root.app.timeline_page.selected.clone())
}

/// The first day: on an empty Timeline `Start with day 1`, else `Add Day`.
fn first_day(shell: &mut Shell) {
    if events(shell).is_empty() {
        assert!(
            shell.bounds("Start with day 1").is_some(),
            "the empty state"
        );
        shell.capture("session-notes__15-timeline-empty__1200x800");
        shell.click("Start with day 1");
    } else {
        shell.click("Add Day");
    }
    shell.settle();
}

/// T11.2: an event added with the buttons opens with its Title taking the keyboard; its rest
/// ruling changes with Undo; Remove has one Undo; a filter shows how many events show; Ctrl+S
/// saves what was typed.
#[test]
fn add_name_rule_remove_and_save() {
    let mut shell = Shell::new("timeline-events", Seed::Campaign);
    shell.click("Timeline");
    shell.settle();
    assert_eq!(shell.read(|root| root.app.page), crate::Page::Timeline);
    first_day(&mut shell);
    let before = events(&mut shell).len();
    shell.click("Add Long rest");
    shell.settle();
    let now = events(&mut shell);
    assert_eq!(now.len(), before + 1);
    let rest = now.last().unwrap().clone();
    assert_eq!(rest.kind, Kind::LongRest);
    assert_eq!(
        selected(&mut shell),
        Some(rest.id.clone()),
        "the new event opens"
    );
    assert!(
        shell.bounds("Undo add Long rest").is_some(),
        "its result has its Undo"
    );
    // The Title has the keyboard.
    shell.type_text("Rest at the mill");
    shell.settle();
    let title = |shell: &mut Shell| {
        shell.read(|root| {
            root.app
                .timeline
                .events
                .iter()
                .find(|e| e.id == rest.id)
                .map(|e| e.title.clone())
        })
    };
    assert_eq!(title(&mut shell).as_deref(), Some("Rest at the mill"));
    shell.capture("session-notes__18-timeline-event__1200x800");

    // The ruling and its Undo.
    let restores = |shell: &mut Shell| {
        shell.read(|root| {
            root.app
                .timeline
                .events
                .iter()
                .find(|e| e.id == rest.id)
                .is_some_and(|e| e.restores)
        })
    };
    assert!(restores(&mut shell));
    shell.click("Mark not at full");
    shell.settle();
    assert!(!restores(&mut shell));
    shell.capture("session-plan__33-timeline-rest-ruling-off__1200x800");
    shell.click("Undo change to ruling");
    shell.settle();
    assert!(restores(&mut shell), "Undo puts the ruling back");

    // Remove and its Undo.
    shell.click("More actions for Rest at the mill");
    shell.settle();
    shell.click("Remove");
    shell.settle();
    assert!(title(&mut shell).is_none(), "the event is gone");
    assert_eq!(selected(&mut shell), None);
    shell.capture("session-notes__22-timeline-removed__1200x800");
    shell.click("Undo remove Rest at the mill");
    shell.settle();
    assert_eq!(title(&mut shell).as_deref(), Some("Rest at the mill"));

    // A filter: the header says how many show.
    shell.click("Days");
    shell.settle();
    let shown =
        shell.update(|root, _, _| root.app.timeline_page.header_context(&root.app.timeline));
    assert!(shown.contains(" of "), "{shown}");
    shell.capture("session-notes__21-timeline-filtered__1200x800");
    shell.click("All");
    shell.settle();

    // Ctrl+S saves now.
    shell.press("ctrl-s");
    shell.settle();
    let path = shell.read(|root| root.app.timeline.path().map(|p| p.to_path_buf()));
    let saved = fs::read_to_string(path.expect("a Timeline file")).unwrap_or_default();
    assert!(saved.contains("Rest at the mill"), "{saved}");
}

/// The open event at the smallest window: two columns still fit beside the rail.
#[test]
fn an_open_event_at_the_smallest_window() {
    let mut shell = Shell::with_size("timeline-small", Seed::Campaign, size(px(960.0), px(640.0)));
    shell.click("Timeline");
    shell.settle();
    first_day(&mut shell);
    shell.click("Add Long rest");
    shell.settle();
    shell.capture("session-notes__18-timeline-event__960x640");
}

/// F9: a journey's encounter checks on Timeline: `Roll for encounters` from its menu, a table,
/// How long on Enter, a roll and its result with Undo, and `Clear`.
#[test]
fn a_journeys_checks() {
    let mut shell = Shell::new("timeline-checks", Seed::Campaign);
    shell.update(|root, _, _| {
        let campaign = root.app.campaigns.active_campaign().unwrap();
        fs::write(campaign.path.join("Road.md"), ROAD).unwrap();
        root.app.lookup.refresh();
    });
    // The content is read in the background in real time: up to 20 s while the suite runs.
    for _ in 0..400 {
        std::thread::sleep(std::time::Duration::from_millis(50));
        shell.wait(std::time::Duration::from_millis(100));
        let tables = shell.read(|root| {
            root.app
                .lookup
                .index()
                .is_some_and(|index| !index.loot_sources().tables.is_empty())
        });
        if tables {
            break;
        }
    }
    shell.click("Timeline");
    shell.settle();
    first_day(&mut shell);
    shell.click("Add Travel");
    shell.settle();
    let journey = events(&mut shell).last().unwrap().id.clone();
    shell.click("More actions for Travel");
    shell.settle();
    shell.click("Roll for encounters");
    shell.settle();
    let event = |shell: &mut Shell| {
        shell.read(|root| {
            root.app
                .timeline
                .events
                .iter()
                .find(|e| e.id == journey)
                .cloned()
                .unwrap()
        })
    };
    assert!(event(&mut shell).checks_table.is_some(), "the block shows");
    assert!(
        shell.bounds("Undo checks").is_some(),
        "its result has its Undo"
    );
    // Several tables: the DM chooses one.
    if event(&mut shell).checks_table.as_deref() == Some("") {
        shell.click(&format!("id:timeline-checks-{journey}-table-button"));
        shell.settle();
        shell.click("On the road");
        shell.settle();
    }
    assert_eq!(event(&mut shell).checks_table.as_deref(), Some("road"));
    // How long has the keyboard; the journey takes it on Enter.
    shell.type_text("1 day");
    shell.press("enter");
    shell.settle();
    assert_eq!(event(&mut shell).duration, "1 day");
    shell.click(&format!("id:timeline-checks-{journey}-roll-0"));
    shell.settle();
    assert_eq!(event(&mut shell).checks.len(), 1, "one check rolled");
    assert!(shell.bounds("Undo roll 1 travel check").is_some());
    shell.capture("travel-checks__07-travel-checks-timeline__1200x800");
    let label = event(&mut shell).checks[0].label.clone();
    shell.click(&format!("id:timeline-checks-{journey}-clear-{label}"));
    shell.settle();
    assert!(
        event(&mut shell).checks.is_empty(),
        "Clear takes the result"
    );
}
