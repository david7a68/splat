//! Trackers in Lookup's panel: the empty state's route, the Tournament's note in play beside the
//! Session page, a quick action and its Undo, a Due row and `Done`, an amount typed for an action
//! that asks, and the panel's captures (`trackers__*`, paired with the egui references).
use crate::gui::harness::Shell;
use crate::visual_tests::fixture::Seed;
use gpui::{px, size};
use std::fs;

const EXAMPLE: &str = include_str!("../../../../docs/design/world-examples/heralds-tournament.md");

fn table(heading: &str, key: &str, dice: &str, rows: &[(&str, &str)]) -> String {
    let mut text = format!(
        "## {heading}\n\n<!-- splat:rolls key={key} dice={dice} -->\n\n| {} | Result |\n| --- | --- |\n",
        dice.trim_start_matches('1')
    );
    for (value, row) in rows {
        text.push_str(&format!("| {value} | {row} |\n"));
    }
    text.push('\n');
    text
}

/// The vault's Tournament note as the example writes it (with `Fight minute` asking for its
/// amount), its tables, and the note in play, as the switch in Campaign → Rules and runs saves it.
fn race_in_play(shell: &mut Shell) {
    shell.update(|root, _, _| {
        let campaign = root.app.campaigns.active_campaign().expect("a campaign");
        let dir = campaign.path.clone();
        let start = EXAMPLE.find("````markdown\n").unwrap() + "````markdown\n".len();
        let end = start + EXAMPLE[start..].find("````").unwrap();
        let id = splat::character::stable_id("Tournament.md");
        let note = EXAMPLE[start..end]
            .replace("<a new UUID>", &id.to_string())
            .replace(
                ">     - label: Fight minute\n>       add: 1",
                ">     - label: Fight minute\n>       add: ask",
            );
        fs::write(dir.join("Tournament.md"), note).unwrap();
        let rival = [
            ("1", "Loses its way in the Kitchens."),
            ("2–5", "Moves one room on."),
            ("6", "Takes a seal and wounds a rival."),
        ];
        let wander = [
            ("1-6", "Nothing comes."),
            ("7-8", "Two Sand Wraiths at the next door."),
        ];
        let tables = [
            table("Rival Track", "rival_track", "1d6", &rival),
            table("Wandering, 0:00 to 0:39", "wander_early", "1d8", &wander),
            table("Wandering, 0:40 to 1:19", "wander_middle", "1d8", &wander),
            table("Wandering, 1:20 and later", "wander_late", "1d8", &wander),
            table("Gifts", "gifts", "1d6", &[("1-6", "A healing potion.")]),
        ];
        fs::write(
            dir.join("Undercroft Tables.md"),
            format!("# Undercroft Tables\n\n{}", tables.concat()),
        )
        .unwrap();
        let catalogue = crate::content_workspace::Catalogue::load(&[], Some(&campaign)).unwrap();
        let mut options = crate::encounter_settings::Options::load(&campaign, &catalogue).unwrap();
        options.in_play.push(id);
        let document = crate::content_workspace::CampaignDocument::load(&campaign).unwrap();
        options.save(&campaign, &document.text).unwrap();
        root.app.lookup.refresh();
        root.app.lookup.files_changed = true;
    });
    // The content is read again in the background.
    for _ in 0..40 {
        std::thread::sleep(std::time::Duration::from_millis(50));
        shell.wait(std::time::Duration::from_millis(100));
        if shell.read(|root| root.app.trackers.in_play()) {
            break;
        }
    }
    assert!(
        shell.read(|root| root.app.trackers.in_play()),
        "the note is in play"
    );
}

fn value(shell: &mut Shell, key: &str, instance: Option<&str>) -> i64 {
    shell.read(|root| {
        root.app
            .trackers
            .state
            .value(&root.app.trackers.rules, key, instance)
    })
}

/// T19.1: with no house rule note in play, Ctrl+T shows the empty state, whose route shows
/// Campaign → Rules and runs.
#[test]
fn no_note_in_play_routes_to_the_rules() {
    let mut shell = Shell::new("trackers-empty", Seed::Campaign);
    shell.press("ctrl-t");
    assert!(shell.read(|root| root.app.lookup.on_trackers()));
    assert!(
        shell.bounds(crate::trackers::view::RULES_ROUTE).is_some(),
        "the empty state shows its route"
    );
    shell.capture("trackers__00-no-house-rules__1200x800");
    shell.click(crate::trackers::view::RULES_ROUTE);
    shell.settle();
    assert_eq!(shell.read(|root| root.app.page), crate::Page::Campaign);
    assert!(shell.read(|root| root.app.campaign_tab == crate::CampaignTab::RunSettings));
}

/// T19.2: a quick action is one press and its result has `Undo: <label>`; a press that passes a
/// mark raises a Due row; `Done` ends it; an action that asks takes a typed amount on Enter, and
/// Escape ends the amount without closing Lookup.
#[test]
fn quick_actions_notices_and_amounts() {
    let mut shell = Shell::new("trackers-race", Seed::Campaign);
    race_in_play(&mut shell);
    shell.click("Session");
    shell.press("ctrl-t");
    shell.settle();
    assert!(
        shell.bounds("Enter room").is_some(),
        "the pinned trackers show"
    );
    shell.capture("trackers__02-trackers-beside-the-plan__1200x800");

    // A quick action and its Undo.
    shell.click("Enter room");
    shell.settle();
    assert_eq!(value(&mut shell, "race_clock", None), 10);
    assert!(
        shell.bounds("Undo: Enter room").is_some(),
        "the result has its Undo"
    );
    shell.click("Undo: Enter room");
    shell.settle();
    assert_eq!(value(&mut shell, "race_clock", None), 0);

    // A seal raises its notice; Done ends it.
    shell.click("Take seal: Party");
    shell.settle();
    assert_eq!(value(&mut shell, "seals", Some("Party")), 1);
    assert_eq!(shell.read(|root| root.app.trackers.due()), 1);
    assert!(shell.bounds("Done: First seal for Party").is_some() || shell.bounds("Done").is_some());
    shell.capture("trackers__03-trackers-a-notice__1200x800");
    shell.click("Done");
    shell.settle();
    assert_eq!(shell.read(|root| root.app.trackers.due()), 0);

    // An action that asks: its field, a typed amount and Enter.
    shell.click("Fight minute");
    shell.settle();
    assert!(
        shell.read(|root| root.app.trackers.takes_escape()),
        "the field asks"
    );
    shell.type_text("3");
    shell.press("enter");
    shell.settle();
    assert_eq!(value(&mut shell, "race_clock", None), 3);
    // Escape ends a typed amount, not Lookup.
    shell.click("Fight minute");
    shell.settle();
    shell.type_text("5");
    shell.press("escape");
    shell.settle();
    assert!(!shell.read(|root| root.app.trackers.takes_escape()));
    assert!(
        shell.read(|root| root.app.lookup.on_trackers()),
        "Lookup stays"
    );
    assert_eq!(
        value(&mut shell, "race_clock", None),
        3,
        "Escape presses nothing"
    );
    // The note's reader in Lookup has each tracker's folded history under the note (egui
    // `TrackerUi::history`, drawn by the shared World reader).
    shell.update(|root, _, cx| {
        root.app.lookup.back_from_trackers();
        assert!(root.app.lookup.open_name("Tournament"), "the note opens");
        cx.notify();
    });
    shell.settle();
    let history = crate::gui::kit::probe::names()
        .into_iter()
        .find(|name| name.starts_with("Seals history"))
        .unwrap_or_else(|| {
            panic!(
                "the history of Seals shows: {:?}",
                crate::gui::kit::probe::names()
            )
        });
    shell.click(&history);
    shell.capture("trackers__07-tracker-history-in-the-reader__1200x800");
}

/// The panel at the smallest window, with the race in play.
#[test]
fn trackers_at_the_smallest_window() {
    let mut shell = Shell::with_size("trackers-small", Seed::Campaign, size(px(960.0), px(640.0)));
    race_in_play(&mut shell);
    shell.click("Session");
    shell.press("ctrl-t");
    shell.settle();
    shell.click("Take seal: Party");
    shell.settle();
    shell.capture("trackers__03-trackers-a-notice__960x640");
}
