//! The same session in the DM screen's tiles and the full note.
use super::*;
use std::fs;

const LIVE: &str = "The river fell quiet when the lantern went out.";
const THREAD: &str = "Who cut the ferry rope?";
const CLUE: &str = "A silver key hangs inside the toll box.";
const FIGHT: &str = "Start fight: Shrine of Cinders";

pub fn run(d: &mut Driver<'_>) {
    d.click(NAV_SESSIONS);
    new_session(d, "The Lantern Crossing");
    let body = format!(
        "# Session 1: The Lantern Crossing\n\n\
         ## Recap of last time\n\nHobb warned the party to cross before dusk.\n\n\
         ## Plan for tonight\n\n- [[Shrine of Cinders]]\n- [ ] Ask Hobb about the ferryman\n\n\
         ## What happened\nThe party reached the ferry at dusk.\n\
         ## Open threads\n\n- {THREAD}\n- Find the missing ferry ledger.\n\n\
         ## People and places\n\nHobb the Miller keeps watch by the ford.\n\n\
         ## Secrets and clues\n\n{CLUE}\n"
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
    d.click("Open threads");
    assert!(d.visible_passage(THREAD));
    assert!(!d.visible_passage(CLUE));
    d.click("Note");
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
}
