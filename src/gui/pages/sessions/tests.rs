//! Screenshots and interaction tests of the Session page. Captures pair with the egui references
//! by name (`<scenario>__<step>__<w>x<h>`).
use crate::gui::harness::Shell;
use crate::recording::Health;
use crate::visual_tests::fixture::Seed;
use gpui::{px, size};

const TITLE: &str = "Ash on the Road";
const SMALL: (f32, f32) = (960.0, 640.0);

/// Wait for the campaign's encounters to load (a background read), so the plan has its fights:
/// on the read's own result, never a clock.
fn wait_for_fights(shell: &mut Shell) {
    shell.update(|root, _, _| root.app.encounters.wait_for_load());
    shell.settle();
    assert!(
        shell.update(|root, _, _| !root.app.encounters.plan_kept().fights.is_empty()),
        "the campaign's fights are read"
    );
}

/// A session made through the form.
fn new_session(shell: &mut Shell, title: &str) {
    shell.click("New session");
    shell.type_text(title);
    shell.click("Create session");
}

/// The open note's words, as its draft holds them.
fn note_words(shell: &mut Shell) -> String {
    shell.update(|root, _, _| root.app.sessions.gv_note_words(false).unwrap_or_default())
}

/// The open note's words, as the DM's edit in the editor would leave them.
fn set_note(shell: &mut Shell, words: &str) {
    shell.update(|root, _, cx| {
        root.app.sessions.gv_note_edited(false, words);
        cx.notify();
    });
}

fn selected_label(shell: &mut Shell) -> Option<String> {
    shell.read(|root| {
        root.app
            .sessions
            .gv_selected_note()
            .map(crate::sessions::Note::label)
    })
}

#[test]
fn session_notes_main_states() {
    let mut shell = Shell::new("session-notes", Seed::Campaign);
    shell.click("Session");
    shell.capture("session-notes__01-sessions-empty__1200x800");
    shell.click("New session");
    shell.capture("session-notes__02-session-form__1200x800");
    shell.type_text(TITLE);
    shell.click("Create session");
    assert_eq!(
        selected_label(&mut shell).as_deref(),
        Some("Session 1: Ash on the Road")
    );
    shell.capture("session-notes__04-session-write__1200x800");
    // Read with nothing written says so, with Write.
    shell.click("Read");
    shell.capture("session-notes__05-empty-note-read__1200x800");
    assert!(
        shell.bounds("Write").is_some(),
        "an empty Read view offers Write"
    );
    shell.click("Write");
    let words = note_words(&mut shell);
    set_note(
        &mut shell,
        &format!("{words}The road was covered in ash, and the wolves followed us.\n"),
    );
    shell.capture("session-notes__06-session-written__1200x800");
    shell.click("Read");
    shell.capture("session-notes__07-session-read__1200x800");
    shell.click("Write");
    // The plan's Add strip: the fight picker and the rarer kinds.
    shell.click("id:plan-add-fight-button");
    shell.capture("session-notes__09-session-plan-fight-picker__1200x800");
    shell.press("escape");
    shell.click("id:plan-add-more-button");
    shell.capture("session-notes__10-plan-more-add-actions__1200x800");
    shell.press("escape");
    shell.click("Long rest");
    shell.capture("session-notes__11-session-event-added__1200x800");
    // Templates: the built-in one, a customized copy, a new one, and a deleted one.
    shell.click("Templates");
    shell.capture("session-notes__29-templates-built-in__1200x800");
    shell.click("Customize");
    shell.capture("session-notes__30-templates-own__1200x800");
    shell.click("New template");
    shell.type_text("Downtime");
    shell.capture("session-notes__31-templates-new__1200x800");
    shell.click("Create template");
    shell.click("id:template-more-button");
    shell.click("Delete");
    shell.capture("session-notes__32-templates-deleted__1200x800");
}

#[test]
fn session_notes_small_window() {
    let mut shell = Shell::with_size(
        "session-notes",
        Seed::Campaign,
        size(px(SMALL.0), px(SMALL.1)),
    );
    shell.click("Session");
    new_session(&mut shell, TITLE);
    shell.capture("session-notes__04-session-write__960x640");
    shell.click("Note");
    shell.capture("session-notes__04-session-write-note-tab__960x640");
}

#[test]
fn session_notes_conflict_and_gone() {
    let mut shell = Shell::new("session-notes", Seed::Campaign);
    shell.click("Session");
    new_session(&mut shell, TITLE);
    let words = note_words(&mut shell);
    set_note(
        &mut shell,
        &format!("{words}My words during the conflict.\n"),
    );
    shell.update(|root, _, _| root.app.sessions.simulate_conflict());
    shell.capture("session-notes__24-note-conflict__1200x800");
    assert!(shell.bounds("Overwrite with my draft").is_some());
    shell.click("Overwrite with my draft");
    let state = shell.read(|root| root.app.sessions.gv_draft_state().cloned());
    assert_ne!(state, Some(crate::sessions::DraftState::Conflict));
}

#[test]
fn plain_evening_states() {
    let mut shell = Shell::new("plain-evening", Seed::Plain);
    shell.click("Session");
    wait_for_fights(&mut shell);
    shell.capture("plain-evening__01-session-tonight__1200x800");
    shell.click("Road to the ford");
    shell.capture("plain-evening__02-session-tonight: Travel row open__1200x800");
    shell.click("Road to the ford");
    shell.click("Shrine of Cinders");
    shell.capture("plain-evening__03-session-tonight: planned fight open__1200x800");
    // Gap 4: the opened played fight shows how it ended (egui `fight_block`): each player's
    // final HP, `Log · Whole run · Timeline`, and the Log under them while it is open.
    use crate::visual_tests::fixture::{PLAIN_LAST, PLAIN_PLAYED};
    let last = shell
        .read(|root| {
            root.app
                .sessions
                .gv_notes()
                .iter()
                .find(|note| note.label().contains(PLAIN_LAST))
                .map(|note| note.id)
        })
        .expect("the last session");
    shell.update(|root, _, cx| {
        root.app.sessions.gv_choose(last);
        cx.notify();
    });
    shell.click(PLAIN_PLAYED);
    for words in ["Log", "Whole run", "Timeline", "Where"] {
        assert!(
            shell.bounds(words).is_some(),
            "the played fight's block shows {words}"
        );
    }
    let names = crate::gui::kit::probe::names();
    assert!(
        names
            .iter()
            .any(|name| name.starts_with("Brannoch Vale: 58/62 HP")),
        "its players' final HP show, each with its whole line: {names:?}"
    );
    shell.capture("plain-evening__04-session-last: played fight open__1200x800");
    shell.click("Log");
    assert!(
        shell.read(|root| root.app.sessions.gv_fight_log_open()),
        "Log opens the log"
    );
    shell.capture("plain-evening__04-session-last: played fight log__1200x800");
    shell.click("New session");
    shell.capture("plain-evening__16-session-new__1200x800");
}

/// Scenes in the plan (egui `session-plan-scenes`): task lines are rows in the note's order;
/// where the closed rows with the scenes do not fit the plan, the scenes fold under `N scenes`
/// on the heading's line, which opens them in place and closes them again.
#[test]
fn plan_scenes_and_their_fold() {
    let scenes = "## Plan for tonight\n\n- [[Bridge Ambush]]\n- [ ] [[Hobb the Miller]]\n- [x] The miller is the informer\n- [ ] Pay the toll\n- [[Shrine of Cinders]]\n";
    for (size, name) in [((1200.0, 800.0), "1200x800"), (SMALL, "960x640")] {
        let mut shell = Shell::with_size(
            "session-plan-scenes",
            Seed::Campaign,
            gpui::size(px(size.0), px(size.1)),
        );
        wait_for_fights(&mut shell);
        shell.click("Session");
        new_session(&mut shell, "The Crossing");
        let words = note_words(&mut shell);
        set_note(&mut shell, &words.replace("## Plan for tonight\n", scenes));
        if shell.bounds("id:session-tabs-Plan").is_some() {
            shell.click("id:session-tabs-Plan");
        }
        let folded = |shell: &mut Shell| {
            shell.update(|root, _, _| {
                let index = root.app.lookup.index();
                super::with_plan(&mut root.app, |sessions, _, book, plan| {
                    sessions.gv_plan(book, plan, index.as_deref()).fold
                })
                .flatten()
            })
        };
        assert_eq!(folded(&mut shell), None, "{name}: the scenes fit the plan");
        shell.capture(&format!(
            "session-plan-scenes__01-scenes-in-the-plan__{name}"
        ));
        if name == "1200x800" {
            continue;
        }
        // More rows than the plan holds with its scenes: they fold, the next fight in view.
        for _ in 0..4 {
            shell.click("id:plan-add-more-button");
            shell.click("Day");
        }
        assert_eq!(
            folded(&mut shell),
            Some((3, true)),
            "{name}: the scenes give way to the rows"
        );
        let scene_rows = || {
            crate::gui::kit::probe::names()
                .iter()
                .filter(|name| name.starts_with("id:plan-scene-box-"))
                .count()
        };
        assert_eq!(scene_rows(), 0, "{name}: folded scenes draw no rows");
        shell.capture(&format!("session-plan-scenes__08-scenes-folded__{name}"));
        // `3 scenes` opens them in place, and closes them again.
        shell.click("3 scenes");
        assert_eq!(folded(&mut shell), Some((3, false)));
        assert_eq!(scene_rows(), 3, "the scenes show");
        shell.click("3 scenes");
        assert_eq!(folded(&mut shell), Some((3, true)));
    }
}

/// After an add, the row after the new one scrolls into view above the plan's footer (egui
/// `Reveal`), so its action needs no extra gesture.
#[test]
fn plan_add_reveals_its_row() {
    let mut shell = Shell::with_size(
        "session-plan-reveal",
        Seed::Campaign,
        size(px(SMALL.0), px(SMALL.1)),
    );
    shell.click("Session");
    new_session(&mut shell, TITLE);
    shell.click("id:session-tabs-Plan");
    // A long plan: many days, made as the Timeline makes them, with no reveal of their own.
    shell.update(|root, _, cx| {
        let session = root.app.sessions.gv_selected();
        for _ in 0..12 {
            root.app.timeline.add(crate::timeline::Kind::Day, session);
        }
        cx.notify();
    });
    shell.click("Long rest");
    let added = timeline_events(&mut shell).last().unwrap().id.clone();
    let row = shell
        .bounds(&format!("id:plan-event-{added}"))
        .expect("the new row is drawn");
    let undo = shell.bounds("Undo add").expect("the footer's Undo");
    assert!(
        row.top() >= px(0.0) && row.bottom() <= undo.top(),
        "the added row is in view above the footer: {row:?}, footer at {undo:?}"
    );
    shell.capture("session-plan-reveal__01-added-row-in-view__960x640");
}

#[test]
fn plain_evening_small_window() {
    let mut shell = Shell::with_size("plain-evening", Seed::Plain, size(px(SMALL.0), px(SMALL.1)));
    shell.click("Session");
    wait_for_fights(&mut shell);
    shell.capture("plain-evening__01-session-tonight__960x640");
}

#[test]
fn session_recording_states() {
    let mut shell = Shell::new("session-recording", Seed::Campaign);
    shell.click("Session");
    new_session(&mut shell, "The Bridge");
    shell.update(|root, _, _| root.app.processing.simulate_input("table.mic"));
    shell.click("Record");
    shell.capture("session-recording__01-record-consent__1200x800");
    shell.click("Start recording");
    shell.capture("session-recording__02-recording-starting__1200x800");
    shell.update(|root, _, _| root.app.sessions.simulate_recorder_health(Health::Hearing));
    shell.capture("session-recording__03-recording-hearing__1200x800");
    shell.click("id:session-tabs-Recordings 1");
    shell.update(|root, _, _| root.app.sessions.simulate_recorder_health(Health::Hearing));
    shell.capture("session-recording__04-recording-listed__1200x800");
    shell.click("Stop recording");
    shell.capture("session-recording__16-recording-ready__1200x800");
    let path = shell.update(|root, _, _| {
        let app = &mut root.app;
        app.sessions
            .gv_recording_rows(&app.processing, &app.timeline)
            .first()
            .map(|row| row.path.display().to_string())
    });
    shell.click(&format!(
        "id:recording-menu-button-{}",
        path.expect("a recording")
    ));
    shell.click("Delete…");
    shell.capture("session-recording__35-recording-delete__1200x800");
    assert!(shell.read(|root| root.app.sessions.gv_confirm_delete().is_some()));
    shell.click("Keep the recording");
    assert!(shell.read(|root| root.app.sessions.gv_confirm_delete().is_none()));
    assert_eq!(shell.read(|root| root.app.sessions.gv_recording_count()), 1);
}

#[test]
fn session_carry_over_offered() {
    let mut shell = Shell::new("session-carry-over", Seed::Campaign);
    shell.click("Session");
    new_session(&mut shell, "The Road to the Burning Shrine");
    let words = note_words(&mut shell);
    set_note(
        &mut shell,
        &words.replace(
            "## What happened\n",
            "## What happened\n\nHobb the miller says the toll collectors have not been paid.\n",
        ),
    );
    shell.update(|root, _, _| root.app.sessions.flush());
    new_session(&mut shell, "The Crossing");
    shell.capture("session-carry-over__01-carry-over-offered__1200x800");
    assert!(
        shell.bounds("Carry over").is_some(),
        "a prepared session offers Carry over"
    );
}

#[test]
fn recording_banner_priority() {
    let mut shell = Shell::new("recording-banner-priority", Seed::Campaign);
    shell.click("Session");
    new_session(&mut shell, "The Bridge");
    shell.update(|root, _, _| {
        root.app
            .sessions
            .simulate_alert("Processing failed: the speech service did not answer.");
    });
    shell.click("Timeline");
    shell.capture("recording-banner-priority__01-failed-elsewhere-with-unprocessed-row__1200x800");
}

// Interaction tests: the main flows, driven through presses and keys.

/// The form: Escape cancels it; Create makes the session and selects it; Details edits it.
#[test]
fn form_creates_cancels_and_edits() {
    let mut shell = Shell::new("session-form-flow", Seed::Campaign);
    shell.click("Session");
    shell.click("New session");
    assert!(shell.read(|root| root.app.sessions.gv_form_open()));
    shell.press("escape");
    assert!(
        !shell.read(|root| root.app.sessions.gv_form_open()),
        "Escape cancels the form"
    );
    shell.click("New session");
    shell.type_text(TITLE);
    shell.press("enter");
    assert_eq!(
        selected_label(&mut shell).as_deref(),
        Some("Session 1: Ash on the Road")
    );
    shell.click("Details");
    shell.click("Session title");
    shell.press("ctrl-a");
    shell.type_text("The Crossing");
    shell.click("Save details");
    assert_eq!(
        selected_label(&mut shell).as_deref(),
        Some("Session 1: The Crossing")
    );
}

/// Words typed into the note save at Ctrl+S, and reach the file.
#[test]
fn note_typing_saves_at_ctrl_s() {
    let mut shell = Shell::new("session-note-typing", Seed::Campaign);
    shell.click("Session");
    new_session(&mut shell, TITLE);
    shell.click("Note text");
    shell.press("ctrl-end");
    shell.type_text("Wolves at the ford.");
    assert!(note_words(&mut shell).contains("Wolves at the ford."));
    shell.press("ctrl-s");
    let path = shell.read(|root| root.app.sessions.gv_selected_note().map(|n| n.path.clone()));
    let file = std::fs::read_to_string(path.expect("a note")).expect("the note's file");
    assert!(
        file.contains("Wolves at the ford."),
        "Ctrl+S saved the typed words"
    );
}

/// Ctrl+\ hides and shows the session list; the header's toggle does too.
#[test]
fn list_toggle_key_and_button() {
    let mut shell = Shell::new("session-list-toggle", Seed::Campaign);
    shell.click("Session");
    new_session(&mut shell, TITLE);
    assert!(
        shell
            .bounds(&format!("Session 1: {TITLE} · 2026-01-01 · Playing"))
            .is_some()
    );
    shell.press("ctrl-\\");
    assert!(shell.read(|root| root.app.sessions.list_hidden));
    assert!(
        shell
            .bounds(&format!("Session 1: {TITLE} · 2026-01-01 · Playing"))
            .is_none(),
        "the hidden list draws no rows"
    );
    shell.click(crate::sessions::page::LIST_TOGGLE);
    assert!(!shell.read(|root| root.app.sessions.list_hidden));
}

/// The plan's Add strip adds an event, and the footer's Undo takes it back.
#[test]
fn plan_add_and_undo() {
    let mut shell = Shell::new("session-plan-add", Seed::Campaign);
    shell.click("Session");
    new_session(&mut shell, TITLE);
    let before = shell.read(|root| root.app.timeline.events.len());
    shell.click("Long rest");
    assert_eq!(
        shell.read(|root| root.app.timeline.events.len()),
        before + 1
    );
    shell.click("Undo add");
    assert_eq!(
        shell.read(|root| root.app.timeline.events.len()),
        before,
        "the footer's Undo takes the added event back"
    );
}

/// A scene typed in the Scene field becomes a plan row; ticking it marks it done in the note.
#[test]
fn plan_scene_add_and_tick() {
    let mut shell = Shell::new("session-plan-scene", Seed::Campaign);
    shell.click("Session");
    new_session(&mut shell, TITLE);
    shell.click("id:plan-add-more-button");
    shell.click("Scene");
    shell.click("Plan a scene: words, a person or a place");
    shell.type_text("Meet the ferryman");
    shell.press("enter");
    assert!(note_words(&mut shell).contains("- [ ] Meet the ferryman"));
    shell.update(|root, _, cx| {
        let ctx = root.bridge.ctx().clone();
        let index = root.app.lookup.index();
        super::with_plan(&mut root.app, |sessions, _, book, plan| {
            let view = sessions.gv_plan(book, plan, index.as_deref());
            let line = view.rows.iter().find_map(|row| match row {
                crate::sessions::page::gpui_facade::PlanRow::Scene(scene) => Some(scene.line),
                _ => None,
            });
            if let Some(line) = line {
                sessions.gv_plan_act(
                    crate::sessions::page::gpui_facade::PlanAct::SceneTick(line),
                    book,
                    plan,
                    index.as_deref(),
                    &ctx,
                );
            }
        });
        cx.notify();
    });
    assert!(note_words(&mut shell).contains("- [x] Meet the ferryman"));
}

/// A template deleted from its menu comes back with the footer's Undo.
#[test]
fn template_delete_and_undo() {
    let mut shell = Shell::new("session-template-delete", Seed::Campaign);
    shell.click("Session");
    shell.click("Templates");
    shell.click("New template");
    shell.type_text("Downtime");
    shell.click("Create template");
    let names = |shell: &mut Shell| {
        shell.read(|root| {
            root.app
                .sessions
                .gv_templates()
                .iter()
                .map(|t| t.name.clone())
                .collect::<Vec<_>>()
        })
    };
    assert!(names(&mut shell).contains(&"Downtime".to_owned()));
    shell.click("id:template-more-button");
    shell.click("Delete");
    assert!(!names(&mut shell).contains(&"Downtime".to_owned()));
    shell.click("Undo delete Downtime");
    assert!(
        names(&mut shell).contains(&"Downtime".to_owned()),
        "Undo delete brings the template back"
    );
}

/// Ctrl+J opens the notes panel beside another page, with the note's slice from What happened;
/// its close icon closes it.
#[test]
fn notes_panel_beside_another_page() {
    let mut shell = Shell::new("session-notes-panel", Seed::Campaign);
    shell.click("Session");
    new_session(&mut shell, TITLE);
    shell.click("Timeline");
    shell.press("ctrl-j");
    assert!(shell.read(|root| root.app.sessions.notes_open));
    let slice =
        shell.update(|root, _, _| root.app.sessions.gv_note_words(true).unwrap_or_default());
    assert!(
        slice.starts_with("## What happened"),
        "the panel holds the note from What happened: {slice:?}"
    );
    shell.capture("session-notes__38-notes-panel__1200x800");
    shell.click("Close notes");
    assert!(!shell.read(|root| root.app.sessions.notes_open));
}

/// `/` at a word's start opens the command menu at the note; its letters narrow it; Escape
/// closes it and leaves the words; `Search` links the chosen entry where the `/` was.
#[test]
fn slash_menu_in_the_note() {
    let mut shell = Shell::new("session-slash", Seed::Campaign);
    shell.click("Session");
    new_session(&mut shell, TITLE);
    shell.click("Note text");
    shell.press("ctrl-end");
    shell.type_text("Met /");
    assert!(
        shell.bounds("Generate NPC").is_some(),
        "the / menu lists Generate NPC"
    );
    assert!(shell.bounds("Search").is_some(), "the / menu lists Search");
    // The menu opens at the `/`: under it, from its left edge (egui `popup_at`).
    let slash = shell
        .update(|root, _, cx| {
            let words = root.app.sessions.gv_note_words(false).unwrap_or_default();
            super::note::bounds_at(root, false, words.rfind('/').unwrap(), cx)
        })
        .expect("the / was drawn");
    let search = [shell.bounds("Search"), shell.bounds("Generate NPC")]
        .into_iter()
        .flatten()
        .min_by(|a, b| f32::from(a.top()).total_cmp(&f32::from(b.top())))
        .expect("the first row");
    assert!(
        search.top() >= slash.bottom() && search.top() - slash.bottom() < px(40.0),
        "the menu opens under the /: {search:?} under {slash:?}"
    );
    assert!(
        (search.left() - slash.left()).abs() < px(24.0),
        "the menu starts at the /: {search:?} at {slash:?}"
    );
    shell.capture("session-notes__53-slash-menu__1200x800");
    shell.type_text("se");
    assert!(
        shell.bounds("Generate NPC").is_none(),
        "the letters narrow the menu"
    );
    shell.press("escape");
    assert!(shell.bounds("Search").is_none(), "Escape closes the menu");
    assert!(
        note_words(&mut shell).ends_with("Met /se"),
        "Escape leaves the words"
    );
    shell.type_text(" and /");
    shell.click("Search");
    shell.type_text("Hobb");
    let first = shell.bounds("id:slash-result-0");
    if first.is_some() {
        shell.press("enter");
        let words = note_words(&mut shell);
        assert!(
            words.contains("and [["),
            "the chosen entry's link takes the / place: {words:?}"
        );
    }
}

/// The event `id` as the Timeline holds it.
fn event_of(root: &crate::gui::SplatRoot, id: &str) -> crate::timeline::Event {
    root.app
        .timeline
        .events
        .iter()
        .find(|event| event.id == id)
        .cloned()
        .expect("the event")
}

/// The events of the selected session's plan, as the Timeline holds them.
fn timeline_events(shell: &mut Shell) -> Vec<crate::timeline::Event> {
    shell.read(|root| root.app.timeline.events.clone())
}

/// Gap 1 of the port (egui `travel_block`): an open Travel row of the plan shows its encounter
/// checks under it, as the Timeline does: `Roll for encounters` from the row's menu, How long
/// with its range, a roll and its result with Undo, a typed roll, and a result's
/// `Plan “<name>”`, which adds the fight to tonight's plan.
#[test]
fn travel_checks_in_the_plan() {
    let mut shell = Shell::new("travel-checks-plan", Seed::World);
    shell.update(|root, _, _| {
        let campaign = root.app.campaigns.active_campaign().unwrap();
        std::fs::write(
            campaign.path.join("Road.md"),
            crate::gui::pages::timeline::tests::ROAD,
        )
        .unwrap();
        root.app.lookup.refresh();
    });
    crate::gui::pages::trackers::tests::read_content_again(&mut shell);
    wait_for_fights(&mut shell);
    shell.click("Session");
    new_session(&mut shell, "The Crossing");
    shell.click("id:plan-add-more-button");
    shell.click("Day");
    shell.click("id:plan-add-more-button");
    shell.click("Travel");
    let journey = timeline_events(&mut shell).last().unwrap().id.clone();
    let event = |shell: &mut Shell| {
        timeline_events(shell)
            .into_iter()
            .find(|event| event.id == journey)
            .unwrap()
    };
    assert!(
        shell.bounds("Enter a roll for Day 1").is_none(),
        "a Travel row shows no checks until the DM asks"
    );
    shell.click("More actions for Travel");
    shell.click("Roll for encounters");
    assert!(event(&mut shell).checks_table.is_some(), "the block shows");
    // How long has the keyboard while the journey has no length.
    shell.type_text("2");
    assert_eq!(
        shell.read(|root| root.app.timeline.length.words(&event_of(root, &journey))),
        "2",
        "How long takes the keyboard"
    );
    if event(&mut shell).checks_table.as_deref() == Some("") {
        shell.click(&format!("id:plan-checks-{journey}-table-button"));
        shell.click("On the road");
    }
    assert_eq!(event(&mut shell).checks_table.as_deref(), Some("road"));
    // The journey takes its length on Enter, and the block has a line a day.
    shell.click("How long");
    shell.press("end");
    shell.type_text(" days");
    shell.press("enter");
    assert_eq!(event(&mut shell).duration, "2 days");
    assert!(
        shell.bounds("Enter a roll for Day 2").is_some(),
        "a line for each day"
    );
    // A roll, its result with Undo, and the Undo.
    shell.click(&format!("id:plan-checks-{journey}-roll-0"));
    assert_eq!(event(&mut shell).checks.len(), 1, "one check rolled");
    assert!(shell.bounds("Undo roll 1 travel check").is_some());
    shell.capture("travel-checks__01-travel-checks-rolled__1200x800");
    shell.click("Undo roll 1 travel check");
    assert!(
        event(&mut shell).checks.is_empty(),
        "Undo takes the roll back"
    );
    // A player's roll, typed: the toll gang, which the plan can take.
    shell.click("Enter a roll for Day 2");
    shell.type_text("19");
    shell.press("enter");
    assert_eq!(
        event(&mut shell).checks.len(),
        1,
        "the typed roll is a result"
    );
    shell.click("Plan “Shrine of Cinders”");
    assert!(
        note_words(&mut shell).contains("[[Shrine of Cinders]]"),
        "the plan line is in the note"
    );
    shell.capture("travel-checks-plan__04-plan-answer-in-session-plan__1200x800");
}
