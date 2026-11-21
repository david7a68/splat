//! The same session in the DM screen's tiles and the full note.
use super::*;
use crate::sessions::page::{HIDE_LIST, SHOW_LIST};
use std::fs;

const LIVE: &str = "The river fell quiet when the lantern went out.";
const THREAD: &str = "Who cut the ferry rope?";
const CLUE: &str = "A silver key hangs inside the toll box.";
const FIGHT: &str = "Start fight: Shrine of Cinders";
const SCENE: &str = "Ask Hobb about the ferryman";
const TOLL: &str = "The toll keeper waved the party through.";

pub fn run(d: &mut Driver<'_>) {
    d.click(NAV_SESSIONS);
    new_session(d, "The Lantern Crossing");
    let body = format!(
        "# Session 1: The Lantern Crossing\n\n\
         ## Recap of last time\n\nHobb warned the party to cross before dusk.\n\n\
         ## Plan for tonight\n\n- [[Shrine of Cinders]]\n- [ ] Ask Hobb about the ferryman\n\n\
         ## Open threads\n\n- {THREAD}\n- Find the missing ferry ledger.\n\n\
         ## People and places\n\nHobb the Miller keeps watch by the ford.\n\n\
         ## Secrets and clues\n\n{CLUE}\n\n\
         ## What happened\nThe party reached the ferry at dusk.\n"
    );
    d.replace_editor_text(&body);
    d.press_ctrl(Key::S);
    let path = d.app().sessions.selected_path().unwrap().to_owned();
    let before = fs::read_to_string(&path).unwrap();

    d.click("DM screen");
    d.expect("Close DM screen");
    d.expect("Plan");
    d.at_size(LARGE, |d| {
        let plan = d.place_of("Plan for tonight").unwrap();
        let happened = d.place_of("What happened").unwrap();
        let threads = d.place_of("Open threads").unwrap();
        assert!(plan.right() < happened.left() && happened.right() < threads.left());
        assert!(d.visible_passage(THREAD) && d.visible_passage(CLUE));
        assert!(d.in_view(FIGHT));
    });
    d.press_ctrl(Key::PageDown);
    assert!(!d.has("Open threads"));
    d.click("Play");
    assert_eq!(fs::read_to_string(&path).unwrap(), before);

    d.click("What happened");
    d.type_into_editor("");
    d.press(Key::Enter);
    d.type_text(LIVE);
    d.press_ctrl(Key::S);
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        before.replace(
            "The party reached the ferry at dusk.",
            &format!("The party reached the ferry at dusk.\n{LIVE}")
        ),
    );
    d.state("writing-at-the-table", Sizes::All);
    let written = fs::read_to_string(&path).unwrap();
    // The Plan tile draws before What happened, and its act shortens the note in that frame.
    // What happened ends the note, so its old place runs past the end.
    d.act_at_size(LARGE, |d| {
        d.click(&format!("More actions for {SCENE}"));
        d.click("Remove from tonight");
        d.expect(&format!("Removed “{SCENE}” from tonight."));
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            written.replace(&format!("- [ ] {SCENE}\n"), "")
        );
        d.click("Undo remove");
        assert_eq!(fs::read_to_string(&path).unwrap(), written);
    });
    // Hiding the session list gives its room to the tiles. A 1440p screen, or 4K at 150 %,
    // gives Open threads and the rest of the note a column each.
    d.act_at_size(LARGE, |d| {
        let shown = d.place_of("What happened").unwrap();
        d.click(HIDE_LIST);
        let hidden = d.place_of("What happened").unwrap();
        assert!(hidden.left() < shown.left(), "{hidden:?} {shown:?}");
        let threads = d.place_of("Open threads").unwrap();
        let rest = d.place_of("Rest of note").unwrap();
        assert!(rest.top() > threads.bottom(), "{threads:?} {rest:?}");
    });
    d.at_size((2560.0, 1440.0), |d| {
        let happened = d.place_of("What happened").unwrap();
        let threads = d.place_of("Open threads").unwrap();
        let rest = d.place_of("Rest of note").unwrap();
        assert!(happened.right() < threads.left() && threads.right() < rest.left());
        assert!(
            (threads.top() - rest.top()).abs() < 1.0,
            "{threads:?} {rest:?}"
        );
        assert!(d.visible_passage(THREAD) && d.visible_passage(CLUE));
    });
    d.act_at_size(LARGE, |d| d.click(SHOW_LIST));
    d.click("Open threads");
    assert!(d.visible_passage(THREAD));
    assert!(!d.visible_passage(CLUE));
    d.click("Rest of note");
    assert!(d.visible_passage(CLUE));
    assert!(!d.visible_passage(THREAD));

    d.click("Notes");
    write_note(d);
    assert!(d.editor_text().unwrap().contains(LIVE));
    d.press_ctrl(Key::PageUp);
    d.expect("Plan");
    d.click("Close DM screen");
    assert!(!d.has("Play"));
    d.click("DM screen");
    d.click(FIGHT);
    d.expect(&format!("{RUN_HEADING}Shrine of Cinders"));

    // A note without a What happened section: the tile's words start one at its end.
    d.click(NAV_SESSIONS);
    d.click("Close DM screen");
    d.click("New session");
    d.expect("Create session");
    d.type_into("Session title", "The Toll Road");
    d.click("Create session");
    d.expect("Session 2: The Toll Road");
    d.replace_editor_text(
        "# Session 2: The Toll Road\n\n## Plan for tonight\n\n- [ ] Pay the toll\n",
    );
    d.press_ctrl(Key::S);
    let path = d.app().sessions.selected_path().unwrap().to_owned();
    let before = fs::read_to_string(&path).unwrap();
    d.click("DM screen");
    d.click("What happened");
    d.type_into_editor(TOLL);
    d.press_ctrl(Key::S);
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        format!("{}\n\n## What happened\n{TOLL}\n", before.trim_end()),
    );
}
