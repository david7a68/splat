//! Screenshots and interaction tests of the Session page. Captures pair with the references
//! by name (`<scenario>__<step>__<w>x<h>`).
use crate::fixtures::Seed;
use crate::gui::harness::Shell;
use crate::recording::Health;
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

/// A capture as the egui driver took it (`finish_capture_saves`): the note, the template and the
/// Timeline are saved first, since test builds save only when told to, so the header says what
/// the DM sees once the save timer has run, never a held "Saving…".
pub(crate) fn capture(shell: &mut Shell, name: &str) -> std::path::PathBuf {
    shell.update(|root, _, cx| {
        root.app.sessions.flush();
        root.app.timeline.save();
        cx.notify();
    });
    shell.capture(name)
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
    // The campaign's paused run shows in the rail once its encounters are read.
    wait_for_fights(&mut shell);
    capture(&mut shell, "session-notes__01-sessions-empty__1200x800");
    shell.click("New session");
    capture(&mut shell, "session-notes__02-session-form__1200x800");
    shell.type_text(TITLE);
    shell.click("Create session");
    assert_eq!(
        selected_label(&mut shell).as_deref(),
        Some("Session 1: Ash on the Road")
    );
    capture(&mut shell, "session-notes__04-session-write__1200x800");
    // Read with nothing written says so, with Write.
    shell.click("Read");
    capture(&mut shell, "session-notes__05-empty-note-read__1200x800");
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
    // As egui's scenario: processing of the session finishes while the DM writes.
    shell.update(|root, _, cx| {
        root.app.sessions.simulate_job_finished();
        cx.notify();
    });
    capture(&mut shell, "session-notes__06-session-written__1200x800");
    shell.click("Read");
    capture(&mut shell, "session-notes__07-session-read__1200x800");
    // The note stays in Read while the DM uses the plan's Add strip (egui: 09-11 in Read).
    // The plan's Add strip: the fight picker and the rarer kinds.
    shell.click("id:plan-add-fight-button");
    capture(&mut shell, "session-notes__09-session-plan-fight-picker__1200x800");
    shell.press("escape");
    shell.click("id:plan-add-more-button");
    capture(&mut shell, "session-notes__10-plan-more-add-actions__1200x800");
    shell.press("escape");
    shell.click("Long rest");
    capture(&mut shell, "session-notes__11-session-event-added__1200x800");
    // Templates: the built-in one, a customized copy, a new one, and a deleted one.
    shell.click("Templates");
    capture(&mut shell, "session-notes__29-templates-built-in__1200x800");
    shell.click("Customize");
    capture(&mut shell, "session-notes__30-templates-own__1200x800");
    // The DM's copy is edited, and the edit is saved with it.
    shell.click("Template text");
    shell.press("ctrl-end");
    shell.type_text("Edited in the template.\n");
    shell.click("New template");
    shell.type_text("Downtime");
    capture(&mut shell, "session-notes__31-templates-new__1200x800");
    shell.click("Create template");
    shell.click("id:template-more-button");
    shell.click("Delete");
    capture(&mut shell, "session-notes__32-templates-deleted__1200x800");
    let own = shell.read(|root| {
        root.app
            .sessions
            .gv_templates()
            .iter()
            .find(|template| template.name == "Standard session")
            .and_then(|template| template.path.clone())
    });
    let text = std::fs::read_to_string(own.expect("the DM's own Standard session")).unwrap();
    assert!(
        text.contains("Edited in the template."),
        "the template edit is saved: {text:?}"
    );
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
    capture(&mut shell, "session-notes__04-session-write__960x640");
    shell.click("Note");
    capture(&mut shell, "session-notes__04-session-write-note-tab__960x640");
    // In the tab layout, the `/` menu's Search field is the note's own: the Note tab stays, with
    // its menu and field.
    shell.click("Note text");
    shell.press("ctrl-end");
    shell.type_text(" /");
    shell.click("Search");
    assert!(
        shell.bounds("Search to link").is_some(),
        "the note's search field stays drawn in the tab layout"
    );
    assert!(
        shell.bounds("Note text").is_some(),
        "the note stays, not the plan"
    );
}

#[test]
fn session_notes_conflict_and_gone() {
    let mut shell = Shell::new("session-notes", Seed::Campaign);
    shell.click("Session");
    new_session(&mut shell, TITLE);
    let words = note_words(&mut shell);
    // As egui's scenario: the note as last saved, the DM's line, a line written in Obsidian,
    // then a save.
    shell.update(|root, _, _| root.app.sessions.flush());
    set_note(
        &mut shell,
        &format!("{words}\nMy words during the conflict.\n"),
    );
    shell.update(|root, _, cx| {
        root.app.sessions.simulate_disk_change();
        root.app.sessions.flush();
        cx.notify();
    });
    assert_eq!(
        shell.read(|root| root.app.sessions.gv_draft_state().cloned()),
        Some(crate::sessions::DraftState::Conflict)
    );
    capture(&mut shell, "session-notes__24-note-conflict__1200x800");
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
    capture(&mut shell, "plain-evening__01-session-tonight__1200x800");
    shell.click("Road to the ford");
    capture(&mut shell, "plain-evening__02-session-tonight: Travel row open__1200x800");
    shell.click("Road to the ford");
    shell.click("Shrine of Cinders");
    capture(&mut shell, "plain-evening__03-session-tonight: planned fight open__1200x800");
    // Gap 4: the opened played fight shows how it ended: each player's
    // final HP, `Log · Whole run · Timeline`, and the Log under them while it is open.
    use crate::fixtures::{PLAIN_LAST, PLAIN_PLAYED};
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
    capture(&mut shell, "plain-evening__04-session-last: played fight open__1200x800");
    shell.click("Log");
    assert!(
        shell.read(|root| root.app.sessions.gv_fight_log_open()),
        "Log opens the log"
    );
    capture(&mut shell, "plain-evening__04-session-last: played fight log__1200x800");
    new_session(&mut shell, "The Shrine");
    assert!(
        selected_label(&mut shell).is_some_and(|label| label.ends_with("The Shrine")),
        "the new session is made and selected"
    );
    capture(&mut shell, "plain-evening__16-session-new__1200x800");
}

/// Scenes in the plan: task lines are rows in the note's order;
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
        capture(&mut shell, &format!(
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
        capture(&mut shell, &format!("session-plan-scenes__08-scenes-folded__{name}"));
        // `3 scenes` opens them in place, and closes them again.
        shell.click("3 scenes");
        assert_eq!(folded(&mut shell), Some((3, false)));
        assert_eq!(scene_rows(), 3, "the scenes show");
        shell.click("3 scenes");
        assert_eq!(folded(&mut shell), Some((3, true)));
    }
}

/// After an add, the row after the new one scrolls into view above the plan's footer, so its action needs no extra gesture.
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
    capture(&mut shell, "session-plan-reveal__01-added-row-in-view__960x640");
}

#[test]
fn plain_evening_small_window() {
    let mut shell = Shell::with_size("plain-evening", Seed::Plain, size(px(SMALL.0), px(SMALL.1)));
    shell.click("Session");
    wait_for_fights(&mut shell);
    capture(&mut shell, "plain-evening__01-session-tonight__960x640");
}

#[test]
fn session_recording_states() {
    let mut shell = Shell::new("session-recording", Seed::Campaign);
    shell.click("Session");
    // The campaign's paused run shows in the rail once its encounters are read.
    wait_for_fights(&mut shell);
    new_session(&mut shell, "The Bridge");
    shell.update(|root, _, _| root.app.processing.simulate_input("table.mic"));
    shell.click("Record");
    capture(&mut shell, "session-recording__01-record-consent__1200x800");
    shell.click("Start recording");
    capture(&mut shell, "session-recording__02-recording-starting__1200x800");
    shell.update(|root, _, _| root.app.sessions.simulate_recorder_health(Health::Hearing));
    capture(&mut shell, "session-recording__03-recording-hearing__1200x800");
    shell.click("id:session-tabs-Recordings 1");
    shell.update(|root, _, _| root.app.sessions.simulate_recorder_health(Health::Hearing));
    capture(&mut shell, "session-recording__04-recording-listed__1200x800");
    shell.click("Stop recording");
    // As in egui's scenario, both keys are saved by this step, so Process is ready.
    shell.update(|root, _, cx| {
        root.app.processing.simulate_keys();
        cx.notify();
    });
    assert!(
        crate::gui::kit::probe::names()
            .iter()
            .any(|name| name.starts_with("Process")),
        "the recording offers Process"
    );
    capture(&mut shell, "session-recording__16-recording-ready__1200x800");
    // The recording is processed, with a warning, before the DM deletes it.
    shell.update(|root, _, cx| {
        root.app
            .sessions
            .simulate_job(crate::processing::Progress::Asking {
                pass: crate::processing::Pass::Review,
                seconds: 9,
            });
        root.app.sessions.simulate_job_end_warned(
            "The check returned nothing, so the clean transcript was not checked.",
        );
        cx.notify();
    });
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
    capture(&mut shell, "session-recording__35-recording-delete__1200x800");
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
    capture(&mut shell, "session-carry-over__01-carry-over-offered__1200x800");
    assert!(
        shell.bounds("Carry over").is_some(),
        "a prepared session offers Carry over"
    );
}

#[test]
fn recording_banner_priority() {
    let mut shell = Shell::new("recording-banner-priority", Seed::Campaign);
    shell.click("Session");
    // The campaign's paused run shows in the rail once its encounters are read.
    wait_for_fights(&mut shell);
    // As egui's scenario: the first session's processing fails while the DM is there...
    new_session(&mut shell, "Failed processing");
    shell.update(|root, _, cx| {
        root.app.processing.simulate_keys();
        root.app.processing.simulate_input("table.mic");
        cx.notify();
    });
    let record = |shell: &mut Shell| {
        shell.click("Record");
        // The consent is asked once an evening.
        if shell.bounds("Start recording").is_some() {
            shell.click("Start recording");
        }
        shell.click("Stop recording");
        let tab = first_target(shell, "id:session-tabs-Recordings");
        shell.click(&tab);
    };
    record(&mut shell);
    shell.update(|root, _, cx| {
        root.app
            .sessions
            .simulate_job(crate::processing::Progress::Asking {
                pass: crate::processing::Pass::Review,
                seconds: 42,
            });
        root.app.sessions.simulate_job_end(Err(
            "The transcript and summary service is busy. Wait, then select Continue processing."
                .to_owned(),
        ));
        cx.notify();
    });
    // ...then the DM records another session and leaves it unprocessed.
    new_session(&mut shell, "Unprocessed recording");
    record(&mut shell);
    assert!(
        shell.bounds("Continue processing").is_some(),
        "the failed session's banner offers Continue processing"
    );
    assert_eq!(
        shell.read(|root| root.app.sessions.gv_recording_count()),
        1,
        "the open session has its unprocessed recording"
    );
    capture(&mut shell, "recording-banner-priority__01-failed-elsewhere-with-unprocessed-row__1200x800");
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
    // After a step of the run, what is typed next is a new paragraph.
    shell.update(|root, _, cx| {
        root.app.sessions.after_a_step_back_to_note(false);
        cx.notify();
    });
    shell.type_text("Ogre arrives.");
    let words = note_words(&mut shell);
    assert!(
        words.contains("Wolves at the ford.\n\nOgre arrives."),
        "a new thought starts a paragraph: {words:?}"
    );
    // A correction key goes on with the same line.
    shell.update(|root, _, cx| {
        root.app.sessions.after_a_step_back_to_note(false);
        cx.notify();
    });
    shell.press("left right");
    shell.type_text("!");
    let words = note_words(&mut shell);
    assert!(
        words.contains("Ogre arrives.!"),
        "a correction keeps the line: {words:?}"
    );
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
    // The list dragged to its widest: the pane left is too narrow for the plan beside the note,
    // so the page takes its tabs.
    assert!(shell.bounds("id:session-tabs-Plan").is_none(), "plan beside the note");
    shell.update(|_, _, cx| {
        crate::gui::kit::edge::set_column_width("session_list_panel", Some(420.0), cx);
    });
    shell.settle();
    assert!(
        shell.bounds("id:session-tabs-Plan").is_some(),
        "the saved list width leaves the pane its tabs"
    );
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
        let ctx = root.ctx.clone();
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
    capture(&mut shell, "session-notes__38-notes-panel__1200x800");
    // A stray click leaves the keyboard on nothing: what the DM types still goes in the note.
    shell.update(|root, window, cx| window.focus(&root.focus, cx));
    shell.type_text("Stray words");
    let slice =
        shell.update(|root, _, _| root.app.sessions.gv_note_words(true).unwrap_or_default());
    assert!(
        slice.contains("Stray words"),
        "words typed on nothing go in the note: {slice:?}"
    );
    // The note's own write (here the page's words) keeps the panel's typing history: Ctrl+Z
    // still takes something back.
    let words = note_words(&mut shell);
    set_note(&mut shell, &format!("{words}Page words.\n"));
    let before =
        shell.update(|root, _, _| root.app.sessions.gv_note_words(true).unwrap_or_default());
    assert!(before.contains("Page words."));
    shell.press("ctrl-z");
    let after =
        shell.update(|root, _, _| root.app.sessions.gv_note_words(true).unwrap_or_default());
    assert_ne!(after, before, "the panel's Undo history stays under the own write");
    // The file changes under the draft: the panel shows the comparison, not an editor.
    shell.update(|root, _, cx| {
        root.app.sessions.simulate_conflict();
        cx.notify();
    });
    assert!(
        shell.bounds("Note text").is_none(),
        "no editor while the draft cannot save"
    );
    capture(&mut shell, "session-notes-panel__02-conflict__1200x800");
    shell.click("Close notes");
    assert!(!shell.read(|root| root.app.sessions.notes_open));
}

/// `/` at a word's start opens the command menu at the note; its letters narrow it; Escape
/// closes it and leaves the words; letters no command has close it, and a slash in a word opens
/// none. `Search` keeps Tab in its field and links the chosen entry where the `/` was; Escape
/// after a command leaves no link; `Generate NPC` saves the NPC to the World with Enter and
/// links it. The note goes on after each link.
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
    // The menu opens at the `/`: under it, from its left edge.
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
    capture(&mut shell, "session-notes__53-slash-menu__1200x800");
    // Keys pressed while another control has the keyboard are not the menu's: Down and Enter
    // choose nothing.
    shell.update(|root, window, cx| window.focus(&root.focus, cx));
    shell.press("down enter");
    assert!(
        shell.bounds("id:slash-search").is_none() && note_words(&mut shell).ends_with("Met /"),
        "keys elsewhere choose no command"
    );
    shell.click("Note text");
    assert!(
        shell.read(|root| root.pages.sessions.note.slash.is_none()),
        "a press outside the menu closes it"
    );
    shell.press("ctrl-end backspace");
    shell.type_text("/");
    assert!(shell.bounds("Search").is_some(), "the menu opens again");
    // With only the `/`, Enter ends the line, and the menu goes.
    shell.press("enter");
    assert!(
        shell.read(|root| root.pages.sessions.note.slash.is_none()),
        "Enter after a bare / closes the menu"
    );
    let words = note_words(&mut shell);
    assert!(
        words.ends_with("Met /\n"),
        "Enter after a bare / ends the line: {words:?}"
    );
    shell.press("backspace backspace");
    shell.type_text("/");
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
    // No command starts with these letters: the menu goes and the words stay. In a word, a
    // slash is only a slash.
    shell.type_text(" /zz");
    assert!(shell.bounds("Search").is_none(), "no command: no menu");
    shell.type_text(" and/");
    assert!(
        shell.bounds("Search").is_none(),
        "a slash in a word opens no menu"
    );
    assert!(note_words(&mut shell).ends_with("Met /se /zz and/"));
    // Search: the command's words go, and Tab in the search field stays there. The search
    // and the NPC's tables read Lookup's index, a background read: wait on its result.
    shell.update(|root, _, _| root.app.lookup.wait_for_load());
    shell.settle();
    shell.type_text(" /");
    shell.click("Search");
    let before = note_words(&mut shell);
    assert!(
        before.ends_with("and/ "),
        "the command's words are gone: {before:?}"
    );
    shell.type_text("cinder");
    shell.press("tab");
    shell.press("tab");
    shell.type_text(" wolf");
    assert_eq!(
        note_words(&mut shell),
        before,
        "Tab in the search stays there"
    );
    assert!(
        shell.bounds("id:slash-result-0").is_some(),
        "the search finds the entry"
    );
    // Enter links the chosen entry where the `/` was, and the note goes on after the link.
    shell.press("enter");
    shell.type_text(" today");
    let words = note_words(&mut shell);
    assert!(
        words.ends_with("and/ [[Cinder Wolf]] today"),
        "the chosen entry's link takes the / place: {words:?}"
    );
    // Escape after a command goes back to the note with no link.
    shell.type_text(" /se");
    shell.press("enter");
    shell.press("escape");
    assert!(
        shell.bounds("id:slash-result-0").is_none(),
        "Escape closes the search"
    );
    shell.type_text("x");
    let words = note_words(&mut shell);
    assert!(
        words.ends_with("today x"),
        "no link, the words go on: {words:?}"
    );
    // Generate NPC: Enter saves the NPC to the World and links it; the note goes on after it.
    let npcs = shell.read(|root| {
        root.app
            .campaigns
            .active_campaign()
            .map(|campaign| campaign.path.join("NPCs"))
            .expect("a campaign is open")
    });
    let count = || std::fs::read_dir(&npcs).map_or(0, Iterator::count);
    let made = count();
    shell.type_text(" /gen");
    shell.press("enter");
    assert!(
        shell.bounds("Save to World").is_some(),
        "the NPC card offers Save to World"
    );
    shell.press("enter");
    assert_eq!(count(), made + 1, "the NPC is written to the World");
    shell.type_text(" joined");
    let words = note_words(&mut shell);
    assert!(
        words.ends_with("]] joined") && words.contains("x [["),
        "the NPC's link takes the / place: {words:?}"
    );
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

/// Gap 1 of the port: an open Travel row of the plan shows its encounter
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
    shell.click("Undo roll 1 travel check");
    assert!(
        event(&mut shell).checks.is_empty(),
        "Undo takes the roll back"
    );
    // As egui's scenario: Day 1 rolled, Day 2 a player's 16, which the plan can take.
    shell.click(&format!("id:plan-checks-{journey}-roll-0"));
    shell.click("Enter a roll for Day 2");
    shell.type_text("16");
    shell.press("enter");
    assert_eq!(event(&mut shell).checks.len(), 2, "both days have a result");
    assert!(shell.bounds("Plan “Bridge Ambush”").is_some());
    capture(&mut shell, "travel-checks__01-travel-checks-rolled__1200x800");
    shell.click("Clear the check for Day 2");
    shell.click("Clear the check for Day 1");
    assert!(event(&mut shell).checks.is_empty(), "Clear takes each result off");
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
    capture(&mut shell, "travel-checks-plan__04-plan-answer-in-session-plan__1200x800");
}

/// The first target whose name starts with `prefix`.
fn first_target(shell: &mut Shell, prefix: &str) -> String {
    shell.settle();
    crate::gui::kit::probe::names()
        .into_iter()
        .find(|name| name.starts_with(prefix))
        .unwrap_or_else(|| panic!("no target starts with {prefix:?}"))
}

/// Copies of the campaign's `Bridge Ambush` named `Fight 1` to `Fight {count}`, each with its
/// own id, read again at once: the Fight menu then lists more than `FIGHT_SEARCH_FROM`.
fn more_fights(shell: &mut Shell, count: usize) {
    shell.update(|root, _, _| {
        let active = root.app.campaigns.active_campaign().unwrap().clone();
        let folder = active.path.join("Encounters");
        let words = std::fs::read_to_string(folder.join("Bridge Ambush.md")).unwrap();
        for at in 1..=count {
            let name = format!("Fight {at}");
            let copy: String = words
                .lines()
                .map(|line| {
                    if line.starts_with("splat_id:") {
                        format!("splat_id: {}", uuid::Uuid::new_v4())
                    } else {
                        line.replace("Bridge Ambush", &name)
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            std::fs::write(folder.join(format!("{name}.md")), copy).unwrap();
        }
        let modules = root.app.campaigns.modules().to_vec();
        root.app.encounters.refresh_now(&active, &modules);
    });
    shell.settle();
}

/// The plan's Fight menu: a search shows only once the campaign has more
/// than `FIGHT_SEARCH_FROM` fights, takes the keyboard as the menu opens, and lists the fights
/// that hold its words; a chosen fight joins the plan and the search empties.
#[test]
fn plan_fight_picker_search() {
    let mut shell = Shell::new("session-plan", Seed::Campaign);
    shell.click("Session");
    new_session(&mut shell, TITLE);
    wait_for_fights(&mut shell);
    shell.click("id:plan-add-fight-button");
    assert!(shell.bounds("Plan Bridge Ambush").is_some());
    assert!(
        shell.bounds("Find a fight").is_none(),
        "two fights need no search"
    );
    shell.press("escape");
    more_fights(&mut shell, 9);
    shell.click("id:plan-add-fight-button");
    assert!(
        shell.bounds("Find a fight").is_some(),
        "eleven fights get a search"
    );
    shell.type_text("fight 7");
    assert!(shell.bounds("Plan Fight 7").is_some());
    assert!(shell.bounds("Plan Fight 1").is_none());
    assert!(shell.bounds("Plan Bridge Ambush").is_none());
    capture(&mut shell, "session-plan__02-plan-fight-picker-search__1200x800");
    shell.click("Plan Fight 7");
    assert!(
        note_words(&mut shell).contains("Fight 7"),
        "the chosen fight joins the plan"
    );
    shell.click("id:plan-add-fight-button");
    assert!(
        shell.bounds("Plan Bridge Ambush").is_some(),
        "the search emptied"
    );
}

/// A group assembled in the plan: New group opens the group editor
/// in the plan's place, on the Session page, with its back line to the plan; Discard has its
/// Undo in the plan's footer; Ctrl+S saves it and links it in the plan.
#[test]
fn plan_group_assembly() {
    let mut shell = Shell::new("session-plan-groups", Seed::Groups);
    shell.click("Session");
    new_session(&mut shell, "The Crossing");
    wait_for_fights(&mut shell);
    shell.click("id:plan-add-group-button");
    shell.click("New group");
    assert!(shell.read(|root| root.app.encounters.group_open_in_plan()));
    assert_eq!(shell.read(|root| root.app.page), crate::Page::Sessions);
    assert!(shell.bounds("Back to the plan").is_some());
    shell.type_text("Ash Pack");
    capture(&mut shell, "session-plan-groups__01-plan-group-empty__1200x800");

    // Discard closes it; the plan's footer has its Undo, which opens it again.
    shell.click("Discard");
    assert!(!shell.read(|root| root.app.encounters.group_open_in_plan()));
    assert!(
        shell.bounds("Back to the plan").is_none(),
        "the plan is back"
    );
    shell.click("Undo discard of Ash Pack");
    assert!(shell.read(|root| root.app.encounters.group_open_in_plan()));

    // As in egui's scenario, the session list stays beside the plan.
    shell.click("Find a creature");
    shell.type_text("wolf");
    // Two wolves, as egui's scenario added them.
    shell.click("Add Wolf");
    shell.click("Add Wolf");
    assert_eq!(
        shell.read(|root| root.app.encounters.group_members().map(|m| m.len())),
        Some(1)
    );
    capture(&mut shell, "session-plan-groups__02-plan-group-assembled__1200x800");
    shell.press("ctrl-s");
    assert!(
        !shell.read(|root| root.app.encounters.group_open_in_plan()),
        "Ctrl+S saves the group and closes it"
    );
    assert!(
        note_words(&mut shell).contains("Ash Pack"),
        "the saved group joins the plan"
    );
    // The plan's row comes with the campaign's next read of its groups (a background read after
    // the save): wait on the read's own result, as egui's scenario waited for the row.
    shell.update(|root, _, _| root.app.encounters.wait_for_load());
    shell.settle();
    assert!(
        shell.bounds("Edit group Ash Pack").is_some(),
        "the saved group is a row of the plan"
    );
    capture(&mut shell, "session-plan-groups__03-plan-group-saved__1200x800");

    // Back waits while a new group is unsaved; once the saved group opens again unchanged,
    // Back returns to the plan.
    let group = first_target(&mut shell, "id:plan-add-group-button");
    shell.click(&group);
    shell.click("New group");
    shell.click("id:group-back");
    assert!(
        shell.read(|root| root.app.encounters.group_open_in_plan()),
        "an unsaved group holds Back"
    );
    shell.click("Discard");
    // The saved group's link comes with the campaign's next read of its groups (a background
    // read after the save): wait on the read's own result, never a clock.
    shell.update(|root, _, _| root.app.encounters.wait_for_load());
    shell.settle();
    shell.click("Edit group Ash Pack");
    assert!(shell.read(|root| root.app.encounters.group_open_in_plan()));
    shell.click("id:group-back");
    assert!(!shell.read(|root| root.app.encounters.group_open_in_plan()));
    assert_eq!(shell.read(|root| root.app.page), crate::Page::Sessions);
}
