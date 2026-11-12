//! Carry-over between sessions (F5 of the session-cycle design): `Carry over` at the end of the
//! Last time line while the session is prepared, its result and `Undo carry over`, and the recap's
//! embed in Read, drawn by its target. With the long words of the design's wireframes.
use super::*;
use std::fs;

const LAST: &str = "The Road to the Burning Shrine";
const TONIGHT: &str = "The Crossing";
const RECAP: &str = "Hobb the miller says the toll collectors have not been paid, and the ferry ropes were cut in the night.";
const SCENE: &str = "Talk to Hobb the miller about the ropes";
const THREAD: &str = "Who cut the ferry ropes at the crossing below the shrine?";
const RESULT: &str = "Carried over from Session 1: the recap, 2 plan lines and 1 open thread.";
const CARRY: &str = "Carry over";
const UNDO: &str = "Undo carry over";

/// A session's note on disk, by its title.
fn note_path(d: &Driver<'_>, title: &str) -> std::path::PathBuf {
    let folder = d
        .app()
        .campaigns
        .active_campaign()
        .unwrap()
        .path
        .join(crate::sessions::DEFAULT_FOLDER);
    crate::sessions::list_notes(&folder)
        .into_iter()
        .find(|note| note.title == title)
        .expect("the session's note")
        .path
}

/// Row 11: `Carry over` ends the Last time line, at the line's height and inside the page,
/// and the line stays one line.
fn one_line(d: &mut Driver<'_>, id: &str, sizes: &[(f32, f32)]) {
    for &size in sizes {
        let (fits, detail) = d.at_size(size, |d| {
            let label = d.place_of("Last time:");
            let carry = d.place_of(CARRY);
            let width = d.window_size().0;
            (
                label.zip(carry).is_some_and(|(label, carry)| {
                    (carry.center().y - label.center().y).abs() <= 0.5
                        && carry.height() <= crate::design::XL + 0.5
                        && carry.left() > label.right()
                        && carry.right() <= width
                }),
                format!("Last time: {label:?}, Carry over: {carry:?}"),
            )
        });
        d.observe(
            id,
            fits,
            format!("At {size:?}, Carry over ends the Last time line on one line: {detail}"),
        );
    }
}

pub fn run(d: &mut Driver<'_>) {
    d.click(NAV_SESSIONS);
    new_session(d, LAST);
    let body = d.editor_text().expect("a new session has a note");
    d.replace_editor_text(
        &body
            .replace(
                "## Plan for tonight\n",
                &format!(
                    "## Plan for tonight\n\n- [[{RUN}]]\n- [ ] {SCENE}\n- [x] Read the rumours\n"
                ),
            )
            .replace(
                "## What happened\n",
                &format!("## What happened\n\n{RECAP}\n"),
            )
            .replace(
                "## Open threads\n",
                &format!("## Open threads\n\n- [ ] {THREAD}\n- [x] Who paid the toll?\n"),
            ),
    );
    d.press_ctrl(Key::S);
    d.click("New session");
    d.type_into("Session title", TONIGHT);
    d.click("Create session");
    d.expect(&format!("Session 2: {TONIGHT}"));
    d.expect(CARRY);
    one_line(d, "T18.3-line", &[SMALL, DEFAULT, LARGE]);
    d.state("carry-over-offered", Sizes::All);
    d.click(NAV_LOOKUP);
    one_line(d, "T18.3-line-lookup", &[SMALL, DEFAULT]);
    d.state("carry-over-offered-beside-lookup", Sizes::Standard);
    d.click(NAV_LOOKUP);

    // T18.3: the recap's link, the plan lines and the open threads in 1, below the DM's words,
    // with both properties in the same save.
    let tonight = note_path(d, TONIGHT);
    let before = fs::read_to_string(&tonight).unwrap();
    let last = note_path(d, LAST);
    let last_before = fs::read_to_string(&last).unwrap();
    // T18.4: a note the DM did not act on does not change. The new note is the template,
    // with no carry-over property, while the line offers Carry over.
    d.observe(
        "T18.4",
        !before.contains("carried_")
            && before.contains("## Recap of last time\n\n## Plan for tonight"),
        format!("The new session's note is the template until the DM acts: {before:?}"),
    );
    d.begin_task();
    d.click(CARRY);
    d.expect(RESULT);
    d.audit_goal_within("T18.3", "carry over what the last session left", 1);
    let carried = fs::read_to_string(&tonight).unwrap();
    let link = last.file_stem().unwrap().to_string_lossy().into_owned();
    d.observe(
        "T18.3-file",
        carried.contains(&format!("## Recap of last time\n\n![[{link}#What happened]]\n"))
            && carried.contains(&format!("- [[{RUN}]]\n- [ ] {SCENE}\n"))
            && carried.contains(&format!("## Open threads\n\n- [ ] {THREAD}\n"))
            && !carried.contains("Read the rumours")
            && !carried.contains("Who paid the toll?")
            && carried.contains("carried_from: ")
            && carried.contains("carried_parts:"),
        format!("The note holds the embed, the plan lines still to come and the open thread: {carried:?}"),
    );
    d.observe(
        "T18.3-done",
        !d.has(CARRY) && d.has(UNDO),
        "Every part is done, so the line has no action, and the result has its Undo",
    );
    d.state("carried-over", Sizes::All);
    d.click(NAV_LOOKUP);
    d.state("carried-over-beside-lookup", Sizes::Small);
    d.click(NAV_LOOKUP);

    // Row 18: Read draws the embed as the recap's words under a muted line that links the session.
    read_note(d);
    d.expect("From");
    d.wait_for_text("Hobb the miller says the toll collectors");
    d.state("carried-recap-in-read", Sizes::Standard);
    d.click(NAV_LOOKUP);
    d.state("carried-recap-in-read-beside-lookup", Sizes::Small);
    d.click(NAV_LOOKUP);

    // An act A, unrelated acts B on another page and in another session, then Undo of A: the
    // Undo is this session's, and it puts back the exact bytes.
    d.click(NAV_TIMELINE);
    d.click(NAV_SESSIONS);
    d.click_link(&format!("Session 1: {LAST}"));
    show_plan(d);
    d.click(&format!("Open: {SCENE}"));
    d.expect(&format!("Done: {SCENE}"));
    let ticked = fs::read_to_string(&last).unwrap();
    // T18.4: carry-over never wrote the last session's note; only the DM's tick did.
    d.observe(
        "T18.4-last",
        ticked == last_before.replace(&format!("- [ ] {SCENE}"), &format!("- [x] {SCENE}")),
        "The last session's note changed only by the DM's own tick",
    );
    d.click_starting(&format!("Session 2: {TONIGHT}"));
    d.expect(UNDO);
    d.click(UNDO);
    d.wait_for_text(CARRY);
    let undone = fs::read_to_string(&tonight).unwrap();
    d.observe(
        "T18.3-undo",
        undone == before && ticked.contains(&format!("- [x] {SCENE}")) && d.has(CARRY),
        format!("After acts on another page and another session, Undo carry over puts back {before:?}; the note holds {undone:?}"),
    );
}
