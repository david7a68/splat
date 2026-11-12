//! Scenes in a session's plan (F3 of the session-cycle design): task lines as rows with a done
//! box, `Scene` in the Add strip's More list, a scene's link, and `Start fight` for a fight that
//! nobody planned, from a scene's menu and from a reader's header.
use super::*;
use std::fs;
use uuid::Uuid;

const VARRA: &str = "Warden Ostrel";
const MILLER: &str = "The miller is the informer";
const SHRINE: &str = "Shrine of Cinders";
const KEEP: &str = "Ashfall Keep";

fn root(d: &Driver<'_>) -> std::path::PathBuf {
    d.root.join("campaigns/emberfall")
}

fn entry(d: &Driver<'_>, path: &str, name: &str, kind: &str, fields: &str) -> Uuid {
    let id = splat::character::stable_id(path);
    let path = root(d).join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let text = splat::content::formats::fixture(
        "World.md",
        &format!(
            "splat_id: {id}\nsplat_kind: world\nsplat_name: {name}\nsplat_world:\n  type: {kind}\n{fields}"
        ),
        &format!("\n# {name}\n\nA note for tonight.\n"),
    )
    .unwrap();
    fs::write(path, text).unwrap();
    id
}

/// The session note's text on disk.
fn note(d: &Driver<'_>) -> String {
    let session = d.app().sessions.selected_path().expect("a session is open");
    fs::read_to_string(session).unwrap()
}

pub fn run(d: &mut Driver<'_>) {
    let keep = entry(d, "Locations/Ashfall Keep.md", KEEP, "location", "");
    entry(
        d,
        "NPCs/Warden Ostrel.md",
        VARRA,
        "npc",
        &format!("  status: alive\n  in: {{id: {keep}}}\n"),
    );
    d.app_mut().lookup.refresh();
    d.settle();
    d.click(NAV_SESSIONS);
    new_session(d, "The Crossing");
    let body = d.editor_text().expect("a new session has a note");
    d.replace_editor_text(&body.replace(
        "## Plan for tonight\n",
        &format!(
            "## Plan for tonight\n\n- [[{RUN}]]\n- [ ] [[{VARRA}]]\n- [x] {MILLER}\n- [[{SHRINE}]]\n"
        ),
    ));
    d.press_ctrl(Key::S);
    show_plan(d);

    // T12.10: a task line is a scene, in the note's order, with its box where a fight has its
    // caret. A done scene does not count among the rows to come.
    d.expect(&format!("Open: {VARRA}"));
    d.expect(&format!("Done: {MILLER}"));
    d.expect("NPC · alive · Ashfall Keep");
    for size in [SMALL, DEFAULT, LARGE] {
        let (ordered, detail) = d.at_size(size, |d| {
            let tops: Vec<_> = [
                format!("Resume: {RUN}"),
                format!("Open: {VARRA}"),
                format!("Done: {MILLER}"),
                format!("Start fight: {SHRINE}"),
            ]
            .iter()
            .map(|name| d.place_of(name))
            .collect();
            let boxes: Vec<_> = [format!("Open: {VARRA}"), format!("Done: {MILLER}")]
                .iter()
                .filter_map(|name| d.place_of(name))
                .collect();
            (
                d.has_label("TONIGHT 3")
                    && tops.iter().all(Option::is_some)
                    && tops
                        .windows(2)
                        .all(|pair| pair[0].unwrap().top() < pair[1].unwrap().top())
                    && boxes.len() == 2
                    && boxes.iter().all(|rect| {
                        rect.width() >= crate::design::XL - 0.5
                            && rect.height() >= crate::design::XL - 0.5
                    }),
                format!("rows {tops:?}, boxes {boxes:?}"),
            )
        });
        d.observe(
            "T12.10",
            ordered,
            format!("At {size:?}, the scenes stand in the note's order with 24 px boxes: {detail}"),
        );
    }
    d.state("scenes-in-the-plan", Sizes::All);

    // T12.11: a scene's link opens its entry in Lookup, beside the plan, in one click.
    d.begin_task();
    d.click_starting(&format!("{VARRA} (opens"));
    d.wait_for_text("A note for tonight.");
    d.audit_goal_within("T12.11", "read the NPC that a scene links", 1);
    // Section 1.2: the reader's header shows `Start fight` with its word where the room after
    // the title holds it, and else does not show it. At 960 the room beside a 13-letter title
    // is too small.
    for (size, shown) in [(SMALL, false), (DEFAULT, true), (LARGE, true)] {
        let (fits, detail) = d.at_size(size, |d| {
            let start = d.place_of(&format!("Start a fight with {VARRA}"));
            let edit = d.place_of(&format!("Edit {VARRA}"));
            (
                start.is_some() == shown
                    && start.zip(edit).is_none_or(|(start, edit)| {
                        (start.left() - edit.right() - crate::design::XS).abs() <= 0.5
                            && (start.top() - edit.top()).abs() <= 0.5
                    }),
                format!("Start fight {start:?}, Edit {edit:?}"),
            )
        });
        d.observe(
            "T12.12",
            fits,
            format!("At {size:?}, the reader's Start fight shows: {shown}. {detail}"),
        );
    }
    d.state("scene-link-in-lookup", Sizes::All);

    // A scene that links one NPC has Start fight in its menu, beside Remove from tonight: 2.
    d.click(&format!("More actions for {VARRA}"));
    d.expect("Remove from tonight");
    d.state("scene-menu-with-lookup", Sizes::Standard);
    d.begin_task();
    if !d.has(&format!("Start a fight with {VARRA}")) {
        d.click(&format!("More actions for {VARRA}"));
    }
    d.click(&format!("Start a fight with {VARRA}"));
    d.expect(&format!("{RUN_HEADING}Fight with {VARRA}"));
    // An NPC with no stat block: the made-up form, with its name and the keyboard in HP.
    d.observe(
        "T12.12",
        d.focused_name().as_deref() == Some("New creature HP")
            && d.field_text("Find or name a creature").as_deref() == Some(VARRA)
            && !d.has(SEARCH_LOOKUP),
        "Start fight with an NPC that has no stat block opens the made-up form in Arrivals",
    );
    d.state("npc-with-no-stat-block-joins", Sizes::All);
    d.type_text("40");
    d.press(Key::Enter);
    d.expect(&format!("Undo: add {VARRA}"));
    d.audit_goal_within(
        "T12.12",
        "start a fight with a planned NPC from her scene",
        4,
    );
    close_run(d);
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.observe(
        "T12.12",
        root(d)
            .join(format!("Encounters/Fight with {VARRA}.md"))
            .is_file()
            && !note(d).contains("Fight with"),
        "The fight's encounter is saved, and the note gains no line",
    );

    // Scene, in the More list only: Enter adds a task line and keeps the field open; a
    // suggestion links a World entry.
    d.click("More Add actions");
    d.click("Add Scene");
    d.type_text("ash");
    d.expect(KEEP);
    d.state("scene-field-suggests", Sizes::All);
    d.click(&format!("{KEEP} · Location"));
    d.expect("Scene added to tonight.");
    d.observe(
        "T12.10",
        note(d).contains(&format!("- [[{SHRINE}]]\n- [ ] [[{KEEP}]]\n")) && d.has("Plan a scene"),
        "A suggestion adds a scene that links it, and the field stays open",
    );
    d.state("scene-added", Sizes::All);
    d.click("Plan a scene");
    d.press(Key::Escape);
    d.click("Undo scene");
    d.observe(
        "T12.10",
        !note(d).contains(KEEP) && !d.has("Plan a scene"),
        "Undo scene takes the line back out, and Escape closed the field",
    );

    // The box: one click, one character.
    let before = note(d);
    d.click(&format!("Open: {VARRA}"));
    d.expect(&format!("Done: {VARRA}"));
    let after = note(d);
    d.observe(
        "T12.10",
        after.len() == before.len()
            && after
                .chars()
                .zip(before.chars())
                .filter(|(a, b)| a != b)
                .count()
                == 1
            && d.has_label("TONIGHT 2"),
        "A tick changes one character of the note, and the scene no longer counts",
    );
    d.click(&format!("More actions for {MILLER}"));
    d.click("Remove from tonight");
    d.expect(&format!("Removed “{MILLER}” from tonight."));
    d.state("scene-removed", Sizes::All);
    d.click("Undo remove");
    d.expect(&format!("Done: {MILLER}"));
    d.observe(
        "T12.10",
        note(d) == after,
        "Undo remove puts the scene's line back where it was",
    );
}
