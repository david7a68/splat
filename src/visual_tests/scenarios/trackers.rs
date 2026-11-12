//! Trackers between fights (F4 of the session-cycle design): the Tournament's house rule note
//! in play, the Trackers view of Lookup beside the Session page's plan, Ctrl+T there and back,
//! quick actions on a tracker and on an instance, a bell's Due row with its named rolls, the
//! rail's `Due N`, the ledger's Undo through a restart, `Add to Timeline`, and a fight started
//! beside the view.
use super::*;
use std::fs;

const EXAMPLE: &str = include_str!("../../../docs/design/world-examples/heralds-tournament.md");
const SHRINE: &str = "Shrine of Cinders";
const VARRA: &str = "Captain Varra";
const VARRA_WORDS: &str = "The last commander of the Emberfall watch.";

fn root(d: &Driver<'_>) -> std::path::PathBuf {
    d.root.join("campaigns/emberfall")
}

fn ledger(d: &Driver<'_>) -> String {
    fs::read_to_string(root(d).join("Trackers.md")).unwrap_or_default()
}

fn table(heading: &str, key: &str, dice: &str, rows: &[(&str, &str)]) -> String {
    let mut text = format!(
        "## {heading}\n\n<!-- splat:rolls key={key} dice={dice} -->\n\n| {} | Result |\n| --- | --- |\n",
        dice.trim_start_matches('1')
    );
    for (value, row) in rows {
        text.push_str(&format!("| {value} | {row} |\n"));
    }
    text.push('\n');
    text
}

/// The vault's Tournament note as the example writes it, and its tables.
fn write_the_race(d: &Driver<'_>) {
    let start = EXAMPLE.find("````markdown\n").unwrap() + "````markdown\n".len();
    let end = start + EXAMPLE[start..].find("````").unwrap();
    let id = splat::character::stable_id("Tournament.md");
    let note = EXAMPLE[start..end].replace("<a new UUID>", &id.to_string());
    fs::write(root(d).join("Tournament.md"), note).unwrap();
    let rival = [
        ("1", "Loses its way in the Kitchens."),
        ("2–5", "Moves one room on."),
        ("6", "Takes a seal and wounds a rival."),
    ];
    let wander = [
        ("1-6", "Nothing comes."),
        ("7-8", "Two Sand Wraiths at the next door."),
    ];
    let tables = [
        table("Rival Track", "rival_track", "1d6", &rival),
        table("Wandering, 0:00 to 0:39", "wander_early", "1d8", &wander),
        table("Wandering, 0:40 to 1:19", "wander_middle", "1d8", &wander),
        table("Wandering, 1:20 and later", "wander_late", "1d8", &wander),
        table("Gifts", "gifts", "1d6", &[("1-6", "A healing potion.")]),
    ];
    fs::write(
        root(d).join("Undercroft Tables.md"),
        format!("# Undercroft Tables\n\n{}", tables.concat()),
    )
    .unwrap();
}

pub fn run(d: &mut Driver<'_>) {
    write_the_race(d);
    // As when the DM comes back from Obsidian: the content on disk changed.
    d.app_mut().lookup.refresh();
    d.app_mut().lookup.files_changed = true;
    d.settle();
    // The plain evening: a house rule note that is not in play shows nothing anywhere.
    d.click(NAV_SESSIONS);
    new_session(d, "The Race");
    let body = d.editor_text().expect("a new session has a note");
    d.replace_editor_text(&body.replace(
        "## Plan for tonight\n",
        &format!("## Plan for tonight\n\n- [[{SHRINE}]]\n"),
    ));
    d.press_ctrl(Key::S);
    show_plan(d);
    d.observe(
        "T19.3",
        !d.has_containing("due. Open Trackers") && ledger(d).is_empty(),
        "With no house rule note in play, the rail says nothing about trackers",
    );

    // Campaign → Rules and runs has a switch for each house rule note.
    d.click(NAV_CAMPAIGN);
    d.click(TAB_RUN_SETTINGS);
    d.expect("House rules in play");
    d.click("Tournament");
    d.state("house-rules-in-play-switch", Sizes::Standard);
    d.click("Save run settings");
    d.settle();
    d.observe(
        "T19.1",
        fs::read_to_string(root(d).join("Campaign.md"))
            .unwrap()
            .contains("splat_in_play"),
        "Save run settings puts the note in play in Campaign.md",
    );

    // T19.1: Ctrl+T opens Trackers in Lookup beside the Session page's plan.
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.begin_task();
    d.press_ctrl(Key::T);
    d.expect("Trackers");
    d.expect("Elapsed race time");
    d.observe(
        "T19.1",
        d.has_label("PINNED") && d.has_containing("MORE TRACKERS 2") && !d.has_label("DUE"),
        "The view lists the pinned trackers, then More trackers, and no Due row yet",
    );
    d.state("trackers-beside-the-plan", Sizes::All);

    // T19.2: a quick action is one press; a press that passes a mark raises a Due row.
    d.click("Cistern or Sands");
    d.expect("Cistern or Sands: Elapsed race time 20. Due: Bell.");
    d.audit_goal_within("T19.2", "open the numbers and record a room", 2);
    d.observe(
        "T19.3",
        d.has_label("DUE 1") && d.has_containing("1 notice is due"),
        "The press raises a Due row above the trackers, and the rail counts it",
    );
    d.click("Done: Bell");
    d.expect("Bell done.");
    // The race reaches 40: the second bell rolls each rival by name, and the wandering table.
    d.click("Enter room");
    d.click("Enter room");
    d.expect("Enter room: Elapsed race time 40. Due: Bell.");
    for _ in 0..3 {
        d.click("Roll Rival Track");
    }
    d.observe(
        "T19.2",
        d.has_containing("Old Guard: ")
            && d.has_containing("Keyring: ")
            && d.has_containing("Convicts: ")
            && d.is_disabled("Roll Rival Track"),
        "Each press rolls for the next name, and after the last name the button is off",
    );
    d.expect("Each name has its roll.");
    d.click("Roll Wandering, 0:40 to 1:19");
    // An instance's line has its own action: one press.
    d.click("Take seal: Old Guard");
    d.click("Take seal: Party");
    d.expect("Take seal: Seals (Party) 1. Due: First seal for Party.");
    d.observe(
        "T19.3",
        d.has_label("DUE 3") && d.has_containing("3 notices are due"),
        "Three notices are due, in the view's heading and in the rail",
    );
    // T19.4: at case D the three notices, their four rolls and the result line are in view
    // when the view opens.
    d.press_ctrl(Key::T);
    d.press_ctrl(Key::T);
    let (whole, detail) = d.at_size(SMALL, |d| {
        let parts = [
            "Done: Bell",
            "Done: First seal for Old Guard",
            "Done: First seal for Party",
            "Undo: Take seal",
        ];
        let words = [
            "Bell. The Cistern floods.",
            "Old Guard: ",
            "Keyring: ",
            "Convicts: ",
            "Wandering, 0:40 to 1:19: ",
            "Old Guard: First seal.",
            "Party: First seal.",
            "Take seal: Seals (Party) 1.",
        ];
        let missing: Vec<_> = parts
            .iter()
            .filter(|name| !d.in_view(name))
            .chain(words.iter().filter(|words| !d.in_view_containing(words)))
            .collect();
        (missing.is_empty(), format!("not in view: {missing:?}"))
    });
    d.observe(
        "T19.4",
        whole,
        format!("Case D beside the Session page: three notices, four rolls and the result line in view. {detail}"),
    );
    d.state("trackers-three-notices", Sizes::All);

    // A wrong press seen late: the opposite action corrects the value, and the history keeps
    // both presses.
    d.click("Lose seal: Party");
    d.expect("Lose seal: Seals (Party) 0.");
    d.observe(
        "T19.2",
        ledger(d).matches("why: Take seal").count() == 2
            && ledger(d).contains("why: Lose seal")
            && d.has_label("DUE 3"),
        "Lose seal corrects the value in one press; a mark does not fire when a value falls",
    );
    // Undo steps back through tonight's acts, newest first.
    d.click("Undo: Lose seal");
    d.expect("Take seal: Seals (Party) 1. Due: First seal for Party.");
    d.click("Undo: Take seal");
    d.expect("Take seal: Seals (Old Guard) 1. Due: First seal for Old Guard.");
    d.observe(
        "T19.2",
        d.has_label("DUE 2") && !ledger(d).contains("for: Party"),
        "Undo takes the press and the notice it raised out of the ledger",
    );

    // Ctrl+T again goes back: Ctrl+T opened Lookup, so it closes it. The rail keeps the count
    // on every page.
    d.press_ctrl(Key::T);
    d.observe(
        "T19.3",
        !d.has(SEARCH_LOOKUP) && d.has_containing("2 notices are due"),
        "Ctrl+T closes the Lookup it opened, and the rail says Due 2",
    );
    d.state("rail-due-beside-the-plan", Sizes::All);

    // T19.1: with an entry in Lookup, Ctrl+T shows Trackers, and Ctrl+T or Back shows the
    // entry again. Ctrl+K opens Lookup on what it showed last, also on Trackers.
    look_up(d, VARRA);
    d.wait_for_text(VARRA_WORDS);
    d.press_ctrl(Key::T);
    d.expect("Elapsed race time");
    d.state("trackers-over-an-entry", Sizes::Small);
    d.press_ctrl(Key::T);
    d.observe(
        "T19.1",
        d.has(SEARCH_LOOKUP)
            && !d.has_containing("Elapsed race time")
            && d.has_containing(VARRA_WORDS),
        "Ctrl+T again shows the entry that Lookup held, and Lookup stays open",
    );
    d.press_ctrl(Key::T);
    d.click("Back");
    d.observe(
        "T19.1",
        !d.has_containing("Elapsed race time") && d.has_containing(VARRA_WORDS),
        "Back returns to the entry that Lookup held",
    );
    d.press_ctrl(Key::T);
    d.press_ctrl(Key::K);
    d.press_ctrl(Key::K);
    d.observe(
        "T19.1",
        d.has_containing("Elapsed race time"),
        "Ctrl+K opens Lookup on what it showed last: Trackers",
    );

    // Add to Timeline is a second act, with the Timeline's own Undo add.
    d.click("Add to Timeline");
    d.expect("Added to the Timeline.");
    let added = |d: &Driver<'_>| {
        d.app()
            .timeline
            .events
            .iter()
            .any(|event| event.title == "Take seal")
    };
    d.observe(
        "T19.2",
        added(d) && ledger(d).matches("why: Take seal").count() == 1,
        "Add to Timeline makes a Major event with the act's label, and writes no ledger entry",
    );
    d.click("Undo add");
    d.observe(
        "T19.2",
        !added(d) && d.has("Undo: Take seal"),
        "Undo add takes the event back; the ledger's Undo is still there",
    );

    // T19.2: after a restart the values, the Due rows and Undo come from the file.
    d.app_mut().timeline.save();
    d.settle();
    d.restart();
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.press_ctrl(Key::T);
    d.expect("Take seal: Seals (Old Guard) 1. Due: First seal for Old Guard.");
    d.observe(
        "T19.2",
        d.has("Undo: Take seal") && d.has_label("DUE 2"),
        "After a restart the result line, its Undo and the Due rows are as before",
    );

    // T19.5: last week's wrong press is corrected from the house rule note's reader. Each
    // tracker's history is folded under it there; a row with another session's stamp has
    // Remove in its menu, with one Undo where the DM acted.
    use crate::trackers::ledger::{self, Entry, Op};
    let wrong = Entry {
        id: "c-01de5ea1".into(),
        tracker: Some("seals".into()),
        instance: Some("Keyring".into()),
        add: Some(1),
        why: Some("Take seal".into()),
        session: Some(uuid::Uuid::from_u128(7)),
        ..Entry::default()
    };
    let path = root(d).join(ledger::FILE);
    ledger::commit(&path, |_| Ok(Op::Restore(vec![(None, wrong.clone())]))).unwrap();
    d.app_mut().trackers.read_disk();
    d.settle();
    d.press_ctrl(Key::T);
    look_up(d, "Tournament");
    d.click("SEALS HISTORY 2");
    let keyring = "More actions for Take seal: Keyring +1";
    d.observe(
        "T19.5",
        d.has_label("Start 0")
            && d.has(keyring)
            && d.has_containing("Take seal: Old Guard +1")
            && !d.has("More actions for Take seal: Old Guard +1")
            && d.has_containing("3 notices are due"),
        "The history opens under its tracker with the start values; only another session's row has Remove",
    );
    d.state("tracker-history-in-the-reader", Sizes::All);
    d.click(keyring);
    d.click("Remove");
    d.expect("Removed: Take seal.");
    d.observe(
        "T19.5",
        !ledger(d).contains("c-01de5ea1")
            && d.has("Undo remove")
            && !d.has(keyring)
            && d.has_containing("2 notices are due"),
        "Remove takes the entry and the notice it raised out of Trackers.md; the reader's footer offers Undo remove",
    );
    d.state("tracker-history-removed", Sizes::Standard);
    // An unrelated act B in the view, then Undo of the removal: the entry goes back to its
    // place at the top, and B stays.
    d.press_ctrl(Key::T);
    d.click("Crowd loves it");
    d.expect("Crowd loves it: Crowd Favor 1.");
    d.press_ctrl(Key::T);
    d.click("Undo remove");
    let text = ledger(d);
    d.observe(
        "T19.5",
        matches!(
            (text.find("c-01de5ea1"), text.find("why: Cistern or Sands")),
            (Some(wrong), Some(first)) if wrong < first
        ) && text.contains("why: Crowd loves it")
            && d.has(keyring),
        "Undo remove puts the entry back where it was, after an unrelated press",
    );
    // A value that an edit in Obsidian forced below its min is in the note's problems list,
    // and nothing changes it. The row that forced it is removed from the history.
    ledger::commit(&path, |_| {
        Ok(Op::Append(vec![Entry {
            id: "c-0f0ced01".into(),
            tracker: Some("favor".into()),
            add: Some(-3),
            session: Some(uuid::Uuid::from_u128(7)),
            ..Entry::default()
        }]))
    })
    .unwrap();
    d.app_mut().trackers.read_disk();
    d.settle();
    d.click(keyring);
    d.click("Remove");
    d.click("CROWD FAVOR HISTORY 2");
    let forced = "Crowd Favor is -2, below its min of 0.";
    d.observe(
        "T19.5",
        d.has_containing(forced) && ledger(d).contains("add: -3"),
        "A value below its min is named in the note's reader and changed by nothing",
    );
    d.click("More actions for Crowd Favor change: -3");
    d.click("Remove");
    d.observe(
        "T19.5",
        !d.has_containing(forced) && !ledger(d).contains("c-0f0ced01"),
        "Remove of the forced row clears the problem",
    );
    // A ledger that cannot be read shows its name, and no history and no value.
    let good = ledger(d);
    fs::write(&path, "---\nchanges: [\n---\n").unwrap();
    d.app_mut().trackers.read_disk();
    d.settle();
    d.observe(
        "T19.5",
        d.has_containing("Trackers.md cannot be read: ")
            && !d.has_containing("SEALS HISTORY")
            && !d.has_containing("Take seal: Old Guard +1"),
        "An unreadable ledger shows its name and no history in the reader",
    );
    fs::write(&path, good).unwrap();
    d.app_mut().trackers.read_disk();
    d.settle();
    d.press(Key::Escape);
    d.press_ctrl(Key::T);

    // A fight starts beside the view; the run shows no tracker part, only the rail's count.
    d.click(&format!("Start fight: {SHRINE}"));
    d.expect(&format!("{RUN_HEADING}{SHRINE}"));
    if d.has_containing("Elapsed race time") {
        d.press_ctrl(Key::T);
    }
    d.observe(
        "T19.3",
        !d.has_containing("Elapsed race time")
            && !d.has_label("DUE 2")
            && d.has_containing("2 notices are due"),
        "In a fight the rail's count is the only tracker sign",
    );
    d.state("run-with-notices-due", Sizes::All);

    d.act_at_size(DEFAULT, |d| {
        // Trackers over a reader's unsaved draft: Escape acts on the view in sight, and the
        // hidden draft keeps its words.
        d.click(NAV_SESSIONS);
        let id = splat::character::stable_id("Clerk.md");
        let text = splat::content::formats::fixture(
            "Clerk.md",
            &format!("splat_id: {id}\nsplat_kind: world\nsplat_name: Clerk\nsplat_world:\n  type: npc\n"),
            "\n# Clerk\n\nThe clerk keeps the lists.\n",
        )
        .unwrap();
        fs::write(root(d).join("Clerk.md"), text).unwrap();
        d.app_mut().lookup.refresh();
        d.settle();
        d.app_mut().lookup.open_id(id);
        d.settle();
        d.wait_for_text("The clerk keeps the lists.");
        d.click("Edit Clerk");
        let draft = "The clerk's draft: keep these unsaved words.";
        d.replace_text_in("Text", draft);
        assert!(d.app().lookup.reader.takes_escape(), "the field has focus");
        d.press_ctrl(Key::T);
        d.expect("Trackers");
        d.press(Key::Escape);
        d.observe(
            "T19.1",
            !d.has(SEARCH_LOOKUP),
            "Escape closes Lookup from Trackers while a reader's unsaved draft is behind it",
        );
        if d.app().lookup.open {
            d.press_ctrl(Key::K);
        }
        d.press_ctrl(Key::K);
        d.press_ctrl(Key::T);
        d.observe(
            "T19.1",
            d.editor_text().as_deref() == Some(draft),
            "The unsaved draft keeps its words through Trackers, Escape and Ctrl+K",
        );

        // A ledger row for a tracker that no note in play has, with another session's stamp,
        // is named at the end of the view.
        use crate::trackers::ledger::{self, Entry, Op};
        ledger::commit(&root(d).join(ledger::FILE), |_| {
            Ok(Op::Append(vec![Entry {
                id: "c-0b5e1e7e".into(),
                tracker: Some("old_rivalry".into()),
                add: Some(7),
                session: Some(uuid::Uuid::from_u128(42)),
                ..Entry::default()
            }]))
        })
        .unwrap();
        d.app_mut().trackers.read_disk();
        d.press_ctrl(Key::T);
        d.observe(
            "T19.2",
            d.has_containing(
                "1 entry in Trackers.md names old_rivalry, which no house rule note in play has. It is not counted.",
            ),
            "The view's problems name a ledger row that counts for nothing",
        );

        // A press whose note went away from disk: its row goes, and the words stay in view on
        // the result line, with what the DM can do.
        fs::remove_file(root(d).join("Tournament.md")).unwrap();
        d.app_mut().trackers.press("favor", None, "Crowd loves it", None);
        d.settle();
        d.observe(
            "T19.2",
            d.has_containing("A house rule note changed on disk. Nothing was recorded.")
                && d.has_containing("Splat reads the notes again when its window gets focus.")
                && !d.has("Crowd loves it"),
            "A refused press says so on the result line when its row went away",
        );
        d.act_at_size(SMALL, |d| {
            d.observe(
                "T19.2",
                d.in_view_containing("A house rule note changed on disk. Nothing was recorded."),
                "The refusal is in view at 960x640 with Lookup narrow",
            );
        });
    });
}
