//! Screenshots and interaction tests of the Session page. Captures pair with the egui references
//! by name (`<scenario>__<step>__<w>x<h>`).
use crate::gui::harness::Shell;
use crate::visual_tests::fixture::Seed;
use gpui::{px, size};

const TITLE: &str = "Ash on the Road";

/// The Campaign seed's first session, made through the form.
fn first_session(shell: &mut Shell) {
    shell.click("Session");
    shell.click("New session");
    shell.type_text(TITLE);
    shell.click("Create session");
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
    let label = shell.read(|root| {
        root.app
            .sessions
            .gv_selected_note()
            .map(crate::sessions::Note::label)
    });
    assert_eq!(label.as_deref(), Some("Session 1: Ash on the Road"));
    shell.capture("session-notes__04-session-write__1200x800");
}

#[test]
fn session_notes_small_window() {
    let mut shell = Shell::with_size("session-notes", Seed::Campaign, size(px(960.0), px(640.0)));
    first_session(&mut shell);
    shell.capture("session-notes__04-session-write__960x640");
}

#[test]
fn plain_evening_session_tonight() {
    let mut shell = Shell::new("plain-evening", Seed::Plain);
    shell.click("Session");
    shell.capture("plain-evening__01-session-tonight__1200x800");
}
