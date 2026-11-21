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
    // The DM's own layout: a column's edge dragged, a tile's edge moved with the keys, a tile
    // docked by its grip and with its menu, swapped, hidden and shown. No tile goes under its
    // minimum.
    d.act_at_size(LARGE, |d| {
        // A grip starts inside the band along the screen's edge. A short slip of it moves
        // nothing.
        let plan = d.place_of("Plan for tonight").unwrap();
        let grip = d.place_of("Move Plan for tonight").unwrap();
        let slip = egui::Rect::from_center_size(grip.center() - egui::vec2(8.0, 0.0), grip.size());
        d.drag_hold("Move Plan for tonight", slip);
        d.drag_release(slip);
        assert_eq!(d.place_of("Plan for tonight"), Some(plan));
        assert!(d.app().sessions.dm_screen_layout().is_none());
        let happened = d.place_of("What happened").unwrap();
        let edge = d.place_of("Resize Plan for tonight column").unwrap();
        d.drag_column_edge(edge.center().x, edge.center().x + 120.0);
        let moved = d.place_of("What happened").unwrap();
        assert!(
            (moved.left() - happened.left() - 120.0).abs() < 2.0,
            "{happened:?} {moved:?}"
        );
        let edge = d.place_of("Resize Plan for tonight column").unwrap();
        d.drag_column_edge(edge.center().x, plan.left());
        let narrowest = d.place_of("What happened").unwrap();
        // The plan's column keeps the room its controls need, and the column gap.
        assert!(
            narrowest.left() - plan.left() >= 394.0 + crate::design::L - 1.0,
            "{plan:?} {narrowest:?}"
        );
        let rest = d.place_of("Rest of note").unwrap();
        assert!(d.tab_to("Resize Open threads tile"));
        d.press(Key::ArrowDown);
        let lower = d.place_of("Rest of note").unwrap();
        assert!(
            (lower.top() - rest.top() - crate::design::L).abs() < 1.0,
            "{rest:?} {lower:?}"
        );
        // Dragged by its grip to the top edge of the whole screen: the plan goes across the top.
        let plan = d.place_of("Plan for tonight").unwrap();
        let happened = d.place_of("What happened").unwrap();
        let top = egui::Rect::from_center_size(
            pos2(happened.center().x, plan.top() - 4.0),
            egui::vec2(2.0, 2.0),
        );
        d.drag_hold("Move Plan for tonight", top);
        d.drag_release(top);
        let plan = d.place_of("Plan for tonight").unwrap();
        let happened = d.place_of("What happened").unwrap();
        assert!(plan.bottom() < happened.top(), "{plan:?} {happened:?}");
        let plan_menu = d.place_of("More actions for Plan for tonight").unwrap();
        let rest_menu = d.place_of("More actions for Rest of note").unwrap();
        assert!((plan_menu.right() - rest_menu.right()).abs() < 1.0);
        // The same with the keys: the tile's menu puts it beside another.
        d.click("More actions for Rest of note");
        d.click("Move to the left of");
        d.click("Move to the left of What happened");
        let rest = d.place_of("Rest of note").unwrap();
        let happened = d.place_of("What happened").unwrap();
        assert!(rest.right() < happened.left(), "{rest:?} {happened:?}");
        assert!((rest.top() - happened.top()).abs() < 1.0);
        d.click("More actions for Plan for tonight");
        d.click("Swap with");
        d.click("Swap with Open threads");
        let threads = d.place_of("Open threads").unwrap();
        let plan = d.place_of("Plan for tonight").unwrap();
        let happened = d.place_of("What happened").unwrap();
        assert!(threads.bottom() < happened.top() && plan.left() > happened.right());
        d.click("More actions for Open threads");
        d.click("Hide tile");
        assert!(!d.has("Open threads"));
    });
    // Tabs have no tile menu, so they list a hidden tile too.
    d.click("Open threads");
    assert!(d.visible_passage(THREAD));
    d.click("Plan");
    d.act_at_size(LARGE, |d| {
        assert!(!d.has("Open threads"));
        // A tile shown again goes below the last tile in reading order.
        d.click("More actions for Plan for tonight");
        d.click("Show Open threads");
        let threads = d.place_of("Open threads").unwrap();
        let plan = d.place_of("Plan for tonight").unwrap();
        assert!(threads.top() > plan.bottom(), "{threads:?} {plan:?}");
        assert!(d.app().sessions.dm_screen_layout().is_some());
    });
    d.state("own-layout", Sizes::Large);
    // The layout comes back after a restart.
    d.restart();
    if !d.has("Close DM screen") {
        d.click(NAV_SESSIONS);
        d.click("DM screen");
    }
    d.act_at_size(LARGE, |d| {
        let threads = d.place_of("Open threads").unwrap();
        let plan = d.place_of("Plan for tonight").unwrap();
        assert!(threads.top() > plan.bottom(), "{threads:?} {plan:?}");
    });
    d.act_at_size(LARGE, |d| {
        d.click("More actions for What happened");
        d.click("Reset layout");
        assert!(d.app().sessions.dm_screen_layout().is_none());
        let threads = d.place_of("Open threads").unwrap();
        let rest = d.place_of("Rest of note").unwrap();
        assert!(rest.top() > threads.bottom(), "{threads:?} {rest:?}");
    });
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
