//! The Timeline page: add, name, rule, remove and their Undos, the filters, Ctrl+S, a journey's
//! encounter checks, and the captures (`<scenario>__<step>`, paired with the references).
use crate::fixtures::Seed;
use crate::gui::harness::Shell;
use crate::timeline::Kind;
use gpui::{px, size};
use std::fs;

/// A road table: a Bridge Ambush on 15-17, a toll gang (Shrine of Cinders) on 18-20.
pub(crate) const ROAD: &str = "# Road\n\n## On the road\n\n<!-- splat:rolls key=road dice=1d20 per=day -->\n\n| d20 | On the road |\n| --- | --- |\n| 1-14 | Nothing stirs. |\n| 15-17 | [[Bridge Ambush]] waits by the old quarry gate, where the road narrows between walls of loose stone. |\n| 18-20 | A toll gang blocks the way: [[Shrine of Cinders]]. |\n";

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
        shell.click("Start with day 1");
    } else {
        shell.click("Add Day");
    }
    shell.settle();
}

/// The session-notes scenario's Timeline steps, as egui's made them: a session's plan adds a
/// Long rest (linked in the note), a Travel and a Major event; on the Timeline a Day named
/// Harvest Eve starts the timeline, and the rest is renamed and placed. Then the Rests filter,
/// and the rest removed while the note still links to it.
#[test]
fn session_notes_timeline_steps() {
    let mut shell = Shell::new("session-notes", Seed::Campaign);
    shell.click("Session");
    shell.update(|root, _, _| root.app.encounters.wait_for_load());
    shell.settle();
    shell.click("New session");
    shell.type_text("Ash on the Road");
    shell.click("Create session");
    // As egui's scenario (step 06): the session's processing has finished before the Timeline
    // steps, so the rail says `Processed` on every page.
    shell.update(|root, _, cx| {
        root.app.sessions.simulate_job_finished();
        cx.notify();
    });
    shell.click("Long rest");
    shell.click("id:plan-add-more-button");
    shell.click("Travel");
    shell.click("id:plan-add-more-button");
    shell.click("Major event");
    shell.click("Add to notes: Long rest");
    shell.update(|root, _, _| root.app.sessions.flush());
    shell.click("Timeline");
    shell.click("Add Day");
    shell.type_text("Harvest Eve");
    shell.click("Place this event after");
    shell.click("Start of the timeline");
    let rest = events(&mut shell)
        .into_iter()
        .find(|event| event.kind == Kind::LongRest)
        .expect("the plan's rest");
    shell.update(|root, _, cx| {
        root.app.timeline_page.select(Some(rest.id.clone()));
        cx.notify();
    });
    shell.click("Title");
    shell.press("ctrl-a");
    shell.type_text("Rest at the mill");
    shell.click("Where");
    shell.type_text("Old mill");
    shell.press("ctrl-s");
    let now = events(&mut shell);
    assert_eq!(
        now.len(),
        4,
        "a Day, a Long rest, a Travel and a Major event"
    );
    assert_eq!(now[0].title, "Harvest Eve", "the Day starts the timeline");
    let renamed = now.iter().find(|event| event.id == rest.id).unwrap();
    assert_eq!(
        (renamed.title.as_str(), renamed.place.as_str()),
        ("Rest at the mill", "Old mill")
    );
    assert!(renamed.session.is_some(), "the rest is the session's");
    // The title was typed letter by letter: the offer names only the old name the note's link
    // says.
    let old = shell.read(|root| {
        root.app
            .sessions
            .event_link_offer(&root.app.timeline, &rest.id)
            .map(|offer| offer.old().to_vec())
    });
    assert_eq!(old, Some(vec!["Long rest".to_owned()]));
    let processed = |shell: &mut Shell| {
        shell.bounds("id:rail-recording").is_some()
            && shell
                .read(|root| root.app.sessions.recording_detail())
                .as_deref()
                == Some("Processed")
    };
    assert!(
        processed(&mut shell),
        "the rail says Processed on the Timeline page"
    );
    crate::gui::pages::sessions::tests::capture(
        &mut shell,
        "session-notes__18-timeline-event__1200x800",
    );
    shell.click("Rests");
    assert_eq!(
        selected(&mut shell),
        Some(rest.id.clone()),
        "the rest stays open"
    );
    assert!(processed(&mut shell), "the rail still says Processed");
    crate::gui::pages::sessions::tests::capture(
        &mut shell,
        "session-notes__21-timeline-filtered__1200x800",
    );
    shell.click("All");
    shell.click("More actions for Rest at the mill");
    shell.click("Remove");
    crate::gui::pages::sessions::tests::capture(
        &mut shell,
        "session-notes__22-timeline-removed__1200x800",
    );
}

/// The file changed under an edit of the same event: the draft beside the file, aligned, the
/// differing line marked (`kit::DraftComparison`), until the DM chooses. A later change of the
/// file shows in the comparison.
#[test]
fn a_disk_conflict_compares_the_draft_and_the_file() {
    let mut shell = Shell::new("timeline-conflict", Seed::Campaign);
    shell.click("Timeline");
    first_day(&mut shell);
    shell.click("Add Long rest");
    shell.type_text("Rest at the mill");
    shell.press("ctrl-s");
    let path = shell.read(|root| root.app.timeline.path().map(|p| p.to_path_buf()));
    let path = path.expect("a Timeline file");
    let saved = fs::read_to_string(&path).unwrap();
    assert!(saved.contains("Rest at the mill"));
    // The DM renames the rest, and Obsidian renames it too before the DM saves. The outside
    // write comes after the DM's typing: written before it, the file watcher can take the
    // outside name first (no conflict, the DM then edits the newer name), and the test would
    // depend on when the watcher runs.
    shell.click("Title");
    shell.press("ctrl-a");
    shell.type_text("Rest at the old mill");
    fs::write(
        &path,
        saved.replace("Rest at the mill", "Rest by the river"),
    )
    .unwrap();
    shell.press("ctrl-s");
    assert!(
        shell.bounds("Overwrite with my timeline").is_some(),
        "the choice waits"
    );
    // The rail says it in words its column holds (as `Notes changed`); the hover has them all.
    let names = crate::gui::kit::probe::names();
    assert!(
        names.iter().any(|name| name == "Timeline changed")
            && !names.iter().any(|name| name == "Timeline changed on disk"),
        "the rail's short words: {names:?}"
    );
    crate::gui::pages::sessions::tests::capture(&mut shell, "timeline__02-disk-conflict__1200x800");
    let texts = |shell: &mut Shell| {
        shell.read(|root| {
            root.pages
                .timeline
                .comparison
                .as_ref()
                .and_then(|kept| kept.texts.clone().ok())
        })
    };
    assert!(
        shell.bounds("Next difference").is_none()
            && texts(&mut shell).is_some_and(|(draft, disk)| {
                draft.contains("Rest at the old mill") && disk.contains("Rest by the river")
            }),
        "one change, in the shared comparison"
    );
    // The file changes again: the comparison follows it.
    let now = fs::read_to_string(&path).unwrap();
    fs::write(&path, now.replace("Rest by the river", "Rest by the ford")).unwrap();
    shell.settle();
    assert!(
        texts(&mut shell).is_some_and(|(_, disk)| disk.contains("Rest by the ford")),
        "the comparison reads the file again once it changed"
    );
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
    crate::gui::pages::sessions::tests::capture(&mut shell, "timeline__01-empty__1200x800");
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
    crate::gui::pages::sessions::tests::capture(
        &mut shell,
        "session-plan__33-timeline-rest-ruling-off__1200x800",
    );
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
    shell.click("Undo remove Rest at the mill");
    shell.settle();
    assert_eq!(title(&mut shell).as_deref(), Some("Rest at the mill"));

    // A filter: the header says how many show.
    shell.click("Days");
    shell.settle();
    let shown =
        shell.update(|root, _, _| root.app.timeline_page.header_context(&root.app.timeline));
    assert!(shown.contains(" of "), "{shown}");
    shell.click("All");
    shell.settle();

    // Issue #513: under a modal (an open menu), Ctrl+S saves nothing.
    let path = shell.read(|root| root.app.timeline.path().map(|p| p.to_path_buf()));
    let path = path.expect("a Timeline file");
    let rest_id = rest.id.clone();
    shell.update(|root, _, cx| {
        if let Some(event) = root.app.timeline.event_mut(&rest_id) {
            event.title = "Rest at the old mill".to_owned();
        }
        root.app.timeline.touch();
        root.app.timeline_page.select(Some(rest_id.clone()));
        cx.notify();
    });
    shell.settle();
    shell.click("More actions for Rest at the old mill");
    shell.settle();
    assert!(
        shell.update(|root, _, cx| root.modal_open(cx)),
        "the menu is open"
    );
    shell.press("ctrl-s");
    shell.settle();
    let saved = fs::read_to_string(&path).unwrap_or_default();
    assert!(
        !saved.contains("Rest at the old mill"),
        "under a modal, Ctrl+S saves nothing: {saved}"
    );
    shell.press("escape");
    shell.settle();

    // Ctrl+S saves now.
    shell.press("ctrl-s");
    shell.settle();
    let saved = fs::read_to_string(&path).unwrap_or_default();
    assert!(saved.contains("Rest at the old mill"), "{saved}");
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
    crate::gui::pages::sessions::tests::capture(
        &mut shell,
        "session-notes__18-timeline-event__960x640",
    );
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
    crate::gui::pages::trackers::tests::read_content_again(&mut shell);
    assert!(
        shell.read(|root| root
            .app
            .lookup
            .index()
            .is_some_and(|index| !index.loot_sources().tables.is_empty())),
        "the road's table is read"
    );
    // As egui's scenario: the journey is the session's, added in its plan after a Day, then
    // opened on Timeline.
    shell.click("Session");
    shell.click("New session");
    shell.type_text("The Crossing");
    shell.click("Create session");
    shell.click("id:plan-add-more-button");
    shell.click("Day");
    shell.click("id:plan-add-more-button");
    shell.click("Travel");
    let journey = events(&mut shell).last().unwrap().id.clone();
    shell.click("Timeline");
    shell.update(|root, _, cx| {
        root.app.timeline_page.select(Some(journey.clone()));
        cx.notify();
    });
    shell.settle();
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
    crate::gui::pages::sessions::tests::capture(
        &mut shell,
        "travel-checks__07-travel-checks-timeline__1200x800",
    );
    let label = event(&mut shell).checks[0].label.clone();
    shell.click(&format!("id:timeline-checks-{journey}-clear-{label}"));
    shell.settle();
    assert!(
        event(&mut shell).checks.is_empty(),
        "Clear takes the result"
    );
}

/// The Add strip keeps Fight and Long rest on its line; when the line has no room for every
/// kind, the rarer ones share `More Add actions`.
#[test]
fn the_add_strip_shares_more_when_short() {
    let mut wide = Shell::new("timeline-add-strip", Seed::Campaign);
    wide.click("Timeline");
    assert!(wide.bounds("Add Day").is_some(), "every kind fits at 1200");
    assert!(wide.bounds("More Add actions").is_none());
    let mut shell = Shell::with_size(
        "timeline-add-strip",
        Seed::Campaign,
        size(px(960.0), px(640.0)),
    );
    shell.click("Timeline");
    shell.press("ctrl-k");
    assert!(
        shell.read(|root| root.app.lookup.open),
        "Lookup beside the page"
    );
    for kept in ["Add Fight", "Add Long rest"] {
        assert!(shell.bounds(kept).is_some(), "{kept} stays on the line");
    }
    assert!(shell.bounds("Add Day").is_none(), "Day goes to More");
    shell.click("More Add actions");
    crate::gui::pages::sessions::tests::capture(
        &mut shell,
        "timeline-add-strip__01-more-add-actions__960x640",
    );
    let before = events(&mut shell).len();
    shell.click("Day");
    let now = events(&mut shell);
    assert_eq!(now.len(), before + 1);
    assert_eq!(now.last().unwrap().kind, Kind::Day, "More adds its kind");
}

/// Round 29 G142: after a roll, the check's result and its offers scroll least into view.
#[test]
fn a_rolled_checks_result_comes_into_view() {
    let mut shell = Shell::with_size(
        "timeline-checks-reveal",
        Seed::Campaign,
        size(px(1200.0), px(520.0)),
    );
    shell.update(|root, _, _| {
        let campaign = root.app.campaigns.active_campaign().unwrap();
        fs::write(campaign.path.join("Road.md"), ROAD).unwrap();
        root.app.lookup.refresh();
    });
    crate::gui::pages::trackers::tests::read_content_again(&mut shell);
    shell.click("Timeline");
    first_day(&mut shell);
    shell.click("Add Travel");
    let journey = events(&mut shell).last().unwrap().id.clone();
    shell.click("More actions for Travel");
    shell.click("Roll for encounters");
    let table = shell.read(|root| {
        root.app
            .timeline
            .events
            .iter()
            .find(|e| e.id == journey)
            .and_then(|e| e.checks_table.clone())
    });
    if table.as_deref() == Some("") {
        shell.click(&format!("id:timeline-checks-{journey}-table-button"));
        shell.click("On the road");
    }
    shell.type_text("6 days");
    shell.press("enter");
    // The last day's roll, made as its Roll makes it, while its line is below the form's room.
    let last = 5 * 24;
    let clear_of = |shell: &mut Shell| {
        let label = shell.read(|root| {
            root.app
                .timeline
                .events
                .iter()
                .find(|e| e.id == journey)
                .and_then(|e| e.checks.first().map(|check| check.label.clone()))
        });
        label.and_then(|label| shell.bounds(&format!("id:timeline-checks-{journey}-clear-{label}")))
    };
    shell.update(|root, _, cx| {
        let acts = crate::timeline::page::DetailActs {
            checks: Some(crate::timeline::checks::Act::Roll(vec![last])),
            ..Default::default()
        };
        super::detail::apply(root, &journey, acts, cx);
    });
    let clear = clear_of(&mut shell).expect("the result's Clear is drawn");
    let window = shell.size();
    assert!(
        clear.top() >= px(0.0) && clear.bottom() <= window.height,
        "the result is in view: {clear:?} in {window:?}"
    );
    crate::gui::pages::sessions::tests::capture(
        &mut shell,
        "timeline-checks-reveal__01-result-in-view__1200x520",
    );
}

/// Review of #817: in a window so short that the title line would leave the form no room, the
/// title line scrolls with the form; with room, it stays above it.
#[test]
fn the_title_line_scrolls_with_the_form_when_room_is_short() {
    for (height, scrolls) in [(800.0, false), (270.0, true)] {
        let mut shell = Shell::with_size(
            "timeline-short",
            Seed::Campaign,
            size(px(1200.0), px(height)),
        );
        shell.click("Timeline");
        first_day(&mut shell);
        shell.click("Add Long rest");
        let title = |shell: &mut Shell| shell.bounds("Title").expect("the Title field").top();
        let before = title(&mut shell);
        shell.update(|root, _, cx| {
            root.pages
                .timeline
                .form
                .set_offset(gpui::point(px(0.0), px(-40.0)));
            cx.notify();
        });
        let after = title(&mut shell);
        assert_eq!(
            after < before,
            scrolls,
            "at {height}: the Title moved from {before:?} to {after:?}"
        );
        if scrolls {
            crate::gui::pages::sessions::tests::capture(
                &mut shell,
                "timeline-short__01-title-line-scrolls__1200x270",
            );
        }
    }
}

/// Round 26 FG4: a row whose length the clock cannot count says why on hover.
#[test]
fn an_uncounted_length_says_why_on_hover() {
    let mut shell = Shell::new("timeline-uncounted", Seed::Campaign);
    shell.click("Timeline");
    first_day(&mut shell);
    shell.click("Add Travel");
    let journey = events(&mut shell).last().unwrap().id.clone();
    shell.update(|root, _, cx| {
        if let Some(event) = root.app.timeline.event_mut(&journey) {
            event.duration = "a while".to_owned();
        }
        root.app.timeline.touch();
        root.app.timeline_page.select(None);
        cx.notify();
    });
    let row = shell
        .bounds(&format!("id:timeline-row-{journey}"))
        .expect("the journey's row");
    shell.update(|_, window, cx| {
        window.dispatch_event(
            gpui::PlatformInput::MouseMove(gpui::MouseMoveEvent {
                position: row.center(),
                pressed_button: None,
                modifiers: gpui::Modifiers::default(),
            }),
            cx,
        );
    });
    // Before the hover delay the row shows no tooltip; after it, its words are drawn over the
    // page, so the picture changes.
    let before = shell.capture("timeline-uncounted__00-row-hover-before-tip__1200x800");
    shell.wait(std::time::Duration::from_secs(1));
    let after = crate::gui::pages::sessions::tests::capture(
        &mut shell,
        "timeline-uncounted__01-row-hover__1200x800",
    );
    let pixels = |path: &std::path::Path| image::open(path).expect("the capture").to_rgba8();
    assert_ne!(
        pixels(&before),
        pixels(&after),
        "the uncounted length says why on hover"
    );
}

/// M0556: a long Timeline opens at its last event, whole in view, with its first out of view.
/// M0559, M0560: an event opened again, after another, opens its form at its top.
#[test]
fn a_long_timeline_opens_at_its_end_and_a_form_at_its_top() {
    let mut shell = Shell::with_size("timeline-long", Seed::Campaign, size(px(1200.0), px(640.0)));
    let ids: Vec<String> = shell.update(|root, _, _| {
        let book = &mut root.app.timeline;
        (0..60)
            .map(|at| {
                let id = book.add(Kind::Event, None);
                book.event_mut(&id).unwrap().title = format!("Stop {at:02}");
                id
            })
            .collect()
    });
    shell.click("Timeline");
    shell.settle();
    let window = shell.size();
    let row = |shell: &mut Shell, at: usize| shell.bounds(&format!("id:timeline-row-{}", ids[at]));
    let last = row(&mut shell, 59).expect("the last event is drawn");
    let before = row(&mut shell, 58).expect("the event before it is drawn");
    assert!(
        last.top() >= before.bottom() && last.bottom() <= window.height,
        "the last event {last:?} is whole in the {window:?} window"
    );
    assert!(
        row(&mut shell, 0).is_none_or(|first| first.bottom() <= px(0.0)),
        "the list moved: the first event is out of view"
    );
    // The form of Stop 57, scrolled; then Stop 56; then Stop 57 again opens at its top.
    let open = |shell: &mut Shell, at: usize| {
        shell.click(&format!("id:timeline-row-{}", ids[at]));
        shell.settle();
        assert_eq!(selected(shell).as_deref(), Some(ids[at].as_str()));
    };
    let form = |shell: &mut Shell| shell.read(|root| root.pages.timeline.form.offset().y);
    open(&mut shell, 57);
    shell.update(|root, _, cx| {
        root.pages
            .timeline
            .form
            .set_offset(gpui::point(px(0.0), px(-40.0)));
        cx.notify();
    });
    shell.settle();
    assert!(form(&mut shell) < px(0.0), "the form scrolled");
    open(&mut shell, 56);
    open(&mut shell, 57);
    assert_eq!(
        form(&mut shell),
        px(0.0),
        "opened again, the form is at its top"
    );
}
