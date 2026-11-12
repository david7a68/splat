//! Scenes in a session's plan (F3 of the session-cycle design): task lines as rows with a done
//! box, `Scene` in the Add strip's More list, a scene's link, and `Start fight` for a fight that
//! nobody planned, from a scene's menu and from a reader's header. Then F6: `Add to Log` from a
//! done scene and from a reader's header.
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
    // Section 1.2: the reader's header shows `Add to Log` next to Edit and `Start fight` with
    // its word after it, where the room after the title holds them. At 960 the room beside a
    // 13-letter title holds `Add to Log` as its icon alone; at 1200, `Start fight` too.
    for (size, shown) in [(SMALL, false), (DEFAULT, true), (LARGE, true)] {
        let (fits, detail) = d.at_size(size, |d| {
            let (log, start, order) = reader_header(d, VARRA);
            (
                start.is_some() == shown && log.is_some() && order,
                format!("Add to Log {log:?}, Start fight {start:?}"),
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

/// F6 of the session-cycle design: a line in an entry's own Log, from a done scene that links it
/// and from its reader's header, with the long name of the design's wireframes.
pub fn log_from_play(d: &mut Driver<'_>) {
    const VARRA_BLACK: &str = "Captain Varra Black";
    const WORDS: &str = "told the party who burned the ropes at the ford";
    const TYPED: &str = "Told the party who burned the ropes.";
    let field = format!("Log line for {VARRA_BLACK}");
    let added = format!("Added to {VARRA_BLACK}’s Log.");
    let keep = entry(d, "Locations/Ashfall Keep.md", KEEP, "location", "");
    entry(
        d,
        "NPCs/Captain Varra Black.md",
        VARRA_BLACK,
        "npc",
        &format!("  status: alive\n  in: {{id: {keep}}}\n"),
    );
    let path = root(d).join("NPCs/Captain Varra Black.md");
    let before = fs::read_to_string(&path).unwrap();
    d.app_mut().lookup.refresh();
    d.settle();
    d.click(NAV_SESSIONS);
    new_session(d, "The Crossing");
    let body = d.editor_text().expect("a new session has a note");
    // Two done scenes with her: her link alone, and her link with long words. One scene still
    // to come, with a place.
    d.replace_editor_text(&body.replace(
        "## Plan for tonight\n",
        &format!(
            "## Plan for tonight\n\n- [x] [[{VARRA_BLACK}]]\n- [x] [[{VARRA_BLACK}]]: {WORDS}\n- [ ] [[{KEEP}]]\n"
        ),
    ));
    d.press_ctrl(Key::S);
    show_plan(d);
    // The line's stamp: the session being played, and the day on the Timeline's clock.
    let session = d
        .app()
        .sessions
        .selected_path()
        .and_then(|path| {
            path.file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
        })
        .expect("a session is open");
    let stamp = match crate::timeline::day_now(&d.app().timeline.events) {
        Some(day) => format!("{session}, Day {day}"),
        None => session.clone(),
    };

    // A done scene that links one World entry has `Add to Log` in its action place, left of its
    // menu, while its words keep their rows; else in its menu. A scene still to come has none.
    let short = format!("Add to Log: {VARRA_BLACK}");
    let long = format!("Add to Log: {VARRA_BLACK}: {WORDS}");
    let long_menu = format!("More actions for {VARRA_BLACK}: {WORDS}");
    for size in [SMALL, DEFAULT, LARGE] {
        let (placed, detail) = d.at_size(size, |d| {
            let action = d.place_of(&short);
            let menu = d.place_of(&format!("More actions for {VARRA_BLACK}"));
            let open = d.has(&format!("Add to Log: {KEEP}"));
            (
                !open
                    && (size != DEFAULT || action.is_some())
                    && action.is_none_or(|action| {
                        menu.is_some_and(|menu| {
                            action.right() <= menu.left()
                                && (action.top() - menu.top()).abs() <= 0.5
                        })
                    }),
                format!("Add to Log {action:?}, menu {menu:?}, on the scene to come {open}"),
            )
        });
        d.observe(
            "T18.2",
            placed,
            format!(
                "At {size:?}, a done scene has Add to Log left of its menu, and at 1200x800 on its row; the scene to come has none: {detail}"
            ),
        );
    }
    // The fit rule: beside the long words the action would move a word, so it is a line of
    // that scene's menu, and the row is as a scene's row with no action.
    d.click(&long_menu);
    d.observe(
        "T18.2",
        d.has(&long) && d.place_of(&long).is_some_and(|line| {
            d.place_of(&long_menu).is_some_and(|menu| line.top() > menu.bottom())
        }),
        "Beside long words, Add to Log is a line of the scene's menu, not on its row",
    );
    d.state("done-scene-add-to-log", Sizes::All);
    d.press(Key::Escape);

    // From a done scene on the row: Lookup opens on the entry with the field and its stamp,
    // empty, for the scene has no words outside its link. Panel 2, Do 2.
    d.set_base(DEFAULT);
    d.begin_task();
    d.click(&short);
    d.expect(&field);
    d.observe(
        "T18.2",
        d.has("Close Lookup")
            && d.focused_name().as_deref() == Some(field.as_str())
            && d.field_text(&field).as_deref() == Some("")
            && d.visible_text(&format!("{stamp}:")),
        "A done scene's Add to Log opens Lookup on its entry with the Log field and the stamp",
    );
    d.type_text(TYPED);
    d.press(Key::Enter);
    d.expect(&added);
    d.click("Close Lookup");
    d.audit_goal_within(
        "T18.2",
        "put what an NPC said in her own note, from a done scene",
        4,
    );
    let logged = fs::read_to_string(&path).unwrap();
    let expected = format!(
        "{before}\n## Log\n\n- {}: {TYPED}\n",
        stamp.replacen(&session, &format!("[[{session}]]"), 1)
    );
    d.observe(
        "T18.2",
        logged == expected && !note(d).contains(TYPED),
        format!(
            "The entry's note gains one line under a new Log heading, and the session note none: {logged:?}"
        ),
    );

    // The result and its Undo stay with the entry: Lookup shows them again.
    d.click(NAV_LOOKUP);
    d.expect(&added);
    d.state("log-line-added-in-lookup", Sizes::All);
    d.click("Undo Log line");
    d.observe(
        "T18.2",
        fs::read_to_string(&path).unwrap() == before && !d.has("Undo Log line"),
        "Undo Log line takes back that one line, and the note is as before",
    );

    // From the long scene's menu: the field starts with the scene's words outside its link,
    // selected, so typing replaces them.
    d.click("Close Lookup");
    d.click(&long_menu);
    d.click(&long);
    d.expect(&field);
    d.observe(
        "T18.2",
        d.focused_name().as_deref() == Some(field.as_str())
            && d.field_text(&field).as_deref() == Some(WORDS),
        "The menu's Add to Log fills the field with the scene's words outside its link",
    );
    d.state("log-field-from-scene", Sizes::All);
    d.type_text(TYPED);
    d.observe(
        "T18.2",
        d.field_text(&field).as_deref() == Some(TYPED),
        "The scene's words were selected, so typing replaced them",
    );
    // Escape clears and closes the field, and writes nothing.
    d.press(Key::Escape);
    d.observe(
        "T18.2",
        !d.has(&field) && fs::read_to_string(&path).unwrap() == before,
        "Escape closes the field and writes nothing",
    );

    // The reader's header, with a 19-letter title: `Add to Log` next to Edit gives way first, to
    // its icon, then away; `Start fight` keeps its word or does not show; the title keeps its
    // width.
    for size in [SMALL, DEFAULT, LARGE] {
        let (fits, detail) = d.at_size(size, |d| {
            let (log, start, order) = reader_header(d, VARRA_BLACK);
            let icon =
                log.is_some_and(|log| log.width() <= crate::design::Density::Read.control() + 0.5);
            (
                order && (log.is_none() || icon || start.is_some()),
                format!("Add to Log {log:?}, Start fight {start:?}"),
            )
        });
        d.observe(
            "T18.2",
            fits,
            format!("At {size:?}, the header's actions follow section 1.2: {detail}"),
        );
    }
    d.state("log-header-long-title", Sizes::All);

    // An edit form blocks the field: Splat makes no "changed on disk" warning of its own.
    d.click(&format!("Edit {VARRA_BLACK}"));
    d.type_into_editor(" Draft.");
    d.click("Close Lookup");
    d.click(&long_menu);
    d.click(&long);
    d.observe(
        "T18.2",
        d.visible_text("Save or discard your edit first.") && !d.has(&field),
        "While the entry has an edit form, the Log field waits",
    );
    d.state("log-field-blocked-by-edit", Sizes::Default);
    d.click("Discard");
    d.expect(&field);
    d.observe(
        "T18.2",
        d.field_text(&field).as_deref() == Some(WORDS)
            && fs::read_to_string(&path).unwrap() == before,
        "After Discard the field opens with the scene's words, and the note is as before",
    );
}
