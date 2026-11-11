//! Undo after a close and a restart, and a fight that ended by mistake goes on. Each surface is
//! shown with Lookup closed and open, at the minimum window too.
use super::*;

const TARGET: &str = "Cinder Wolf";
const DAMAGE_STEP: &str = "Undo: 3 damage to Cinder Wolf";
const END_STEP: &str = "Undo: end encounter";
const LOOT_STEP: &str = "Undo: 2 loot rolls on gems";
const CONTINUE_STEP: &str = "Undo: continue fight";
const CONTINUE: &str = "Continue fight";
const ROLL_LOOT: &str = "Roll 2 × gems for Sunken Vault";
const SPELL: &str = "Fireball";
const SESSION: &str = "The Vault";
const UNREAD: &str = "Splat could not read this run’s saved Undo steps";
const DISMISS: &str = "Dismiss the Undo message";
const LATER: &str = "A later fight or rest was played. Start a new run to play this one again.";
/// The same reason for a fight whose encounter is deleted: only its cause.
const LATER_GONE: &str = "A later fight or rest was played.";
const NO_TIMELINE: &str = "Splat cannot read Timeline.md. Fix that on the Timeline page first.";
const DISK_CHANGED: &str = "Timeline.md changed on disk. Settle that on the Timeline page first.";

/// The Undo control's whole action, which its accessible name keeps when its words are cut.
fn undo_step(d: &Driver<'_>) -> String {
    d.name_starting("Undo")
}

/// What Timeline says about the vault fight.
fn story(d: &mut Driver<'_>) -> String {
    d.app_mut()
        .timeline
        .events
        .iter()
        .find(|event| event.title == RUN_VAULT)
        .map_or_else(String::new, |event| event.result.clone())
}

/// Lookup is open on the spell the DM chose, with their search words.
fn lookup_keeps_its_entry(d: &Driver<'_>) -> bool {
    d.in_view(SPELL) && d.field_text(SEARCH_LOOKUP).as_deref() == Some("fireball")
}

/// The footer's first step of ending. A narrow footer keeps it in its menu.
fn end_encounter(d: &mut Driver<'_>) {
    if !d.has("End encounter…") && d.has("Close menu") {
        d.click("Close menu");
    }
    d.click("End encounter…");
}

fn open_lookup(d: &mut Driver<'_>) {
    look_up(d, "fireball");
    d.expect(SPELL);
}

fn close_lookup(d: &mut Driver<'_>) {
    d.press_ctrl(Key::K);
    assert!(!d.has(SEARCH_LOOKUP), "Ctrl+K closes Lookup");
}

/// At every size, with Lookup as it is: each named control is whole and in view, and the
/// ended view's title is not cut.
fn whole_at_every_size(d: &mut Driver<'_>, id: &str, what: &str, controls: &[&str], title: &str) {
    for size in [SMALL, DEFAULT] {
        let (shown, missing) = d.at_size(size, |d| {
            let missing: Vec<_> = controls
                .iter()
                .filter(|name| {
                    !d.nodes()
                        .iter()
                        .any(|node| node.name == **name && node.fully_in_view())
                })
                .map(|name| (*name).to_owned())
                .collect();
            (title.is_empty() || d.visible_text(title), missing)
        });
        d.observe(
            id,
            shown && missing.is_empty(),
            format!("{what} at {size:?}: title {title:?} shown {shown}, not whole: {missing:?}"),
        );
    }
}

pub fn run(d: &mut Driver<'_>) {
    // The fight belongs to a session, so the plan and Timeline have it.
    d.click(NAV_SESSIONS);
    new_session(d, SESSION);
    go_encounters(d);
    resume_run(d, RUN_VAULT);
    d.observe(
        "T17.1",
        undo_step(d) == "Undo: next turn",
        format!(
            "A resumed run names the last step from before it closed: {}",
            undo_step(d)
        ),
    );
    d.click_row(TARGET);
    let whole = d.row_hp(TARGET);
    d.type_text("3");
    d.press(Key::Enter);
    let turn = current_participant(d).unwrap_or_default();
    d.observe(
        "T17.1",
        undo_step(d) == DAMAGE_STEP && d.row_hp(TARGET) == whole.map(|hp| hp - 3),
        format!("Undo names the damage: {}", undo_step(d)),
    );

    // Close run by mistake. It goes back to the page the fight began on, where the run that
    // closed last has Resume: one press brings the fight back, with its Undo.
    close_run(d);
    d.begin_task();
    d.click_beside(&format!("{RUN_VAULT} · Closed last"), RESUME);
    d.expect(&format!("{RUN_HEADING}{RUN_VAULT}"));
    d.goal_within("T17.1", "come back from Close run", 1);
    d.observe(
        "T17.1",
        undo_step(d) == DAMAGE_STEP && current_participant(d).as_deref() == Some(turn.as_str()),
        format!(
            "After Close run and Resume, Undo names the same step on the same turn: {}, {:?}",
            undo_step(d),
            current_participant(d)
        ),
    );
    // A quit, or a crash: the app opens the run again by itself.
    d.restart();
    d.observe(
        "T17.1",
        undo_step(d) == DAMAGE_STEP && current_participant(d).as_deref() == Some(turn.as_str()),
        format!(
            "After a restart, Undo names the same step on the same turn: {}, {:?}",
            undo_step(d),
            current_participant(d)
        ),
    );
    d.state("undo-after-restart", Sizes::All);
    // Another page by mistake: the run stays open behind it.
    go_library(d);
    d.begin_task();
    d.click(BACK_TO_RUN);
    d.goal_within("T17.1", "come back from another page", 1);
    d.click_containing(DAMAGE_STEP);
    d.observe(
        "T17.1",
        d.row_hp(TARGET) == whole && undo_step(d) == "Undo: next turn",
        format!(
            "Undo takes the damage back, and the step before it is next: {:?}, {}",
            d.row_hp(TARGET),
            undo_step(d)
        ),
    );

    // End encounter by mistake. The ending is the newest step, so Undo is the one way on.
    end_encounter(d);
    d.click("End encounter");
    d.expect("Outcome: Undecided");
    d.observe(
        "T17.2",
        undo_step(d) == END_STEP && !d.has(CONTINUE),
        "While the ending is the newest step, the ended view has Undo and no second control",
    );
    whole_at_every_size(
        d,
        "T17.6",
        "The ended view",
        &[END_STEP, CLOSE_RUN],
        "Ended · Round 2",
    );
    d.state("ended-undo-is-the-way-on", Sizes::All);
    // A long rest is ruled, and the app starts again. The ending now comes from the run file,
    // so its Undo goes by the rule, as Continue fight does: no Undo of the ending, and the
    // reason in its place. An Undo that reaches the run anyway leaves the fight ended.
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.click("Add Long rest");
    d.click(BACK_TO_RUN);
    d.app_mut().timeline.save();
    d.restart();
    let ended = |d: &mut Driver<'_>| d.app_mut().encounters.run().is_some_and(|run| run.ended());
    let shown = ended(d) && !d.has(END_STEP) && d.visible_passage(LATER) && d.in_view(CLOSE_RUN);
    d.observe(
        "T17.8",
        shown,
        "After a later rest and a restart, the saved ending has no Undo and says why",
    );
    d.app_mut().encounters.run_mut().unwrap().wants_unend = Some(crate::encounter_run::Unend::Undo);
    d.settle();
    let still = ended(d);
    d.observe(
        "T17.8",
        still,
        "No restart bypass: an Undo of the saved ending leaves the fight ended after a later rest",
    );
    if d.has("Dismiss this message") {
        d.click("Dismiss this message");
    }
    // The DM takes the rest away. The fight is the last again, and Undo of its ending is back.
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.click("More actions for Long rest");
    d.click("Remove this event");
    d.click(BACK_TO_RUN);
    d.observe(
        "T17.8",
        undo_step(d) == END_STEP && !d.visible_passage(LATER),
        "With the rest gone, Undo takes back the saved ending",
    );
    open_lookup(d);
    whole_at_every_size(
        d,
        "T17.6",
        "The ended view beside Lookup",
        &[CLOSE_RUN],
        "Ended · Round 2",
    );
    d.state("ended-beside-lookup", Sizes::All);
    d.begin_task();
    d.click_starting("Undo");
    d.goal_within("T17.2", "take back an ending", 1);
    d.observe(
        "T17.2",
        current_participant(d).as_deref() == Some(turn.as_str()) && d.in_view(NEXT_TURN),
        format!(
            "Undo of the ending returns to the turn the fight ended on: {:?}",
            current_participant(d)
        ),
    );
    d.observe(
        "T17.6",
        lookup_keeps_its_entry(d),
        "Undo of the ending leaves Lookup on its entry",
    );

    // Ended again, and the party searches. Undo now names the loot roll, so Continue fight
    // is the way on.
    end_encounter(d);
    d.click("End encounter");
    d.expect("Outcome: Undecided");
    d.click(ROLL_LOOT);
    d.expect(" on 1d4");
    d.observe(
        "T17.2",
        undo_step(d) == LOOT_STEP && d.has(CONTINUE),
        format!(
            "After a loot roll, Undo names the roll and Continue fight shows: {}",
            undo_step(d)
        ),
    );
    whole_at_every_size(
        d,
        "T17.6",
        "The ended view with loot beside Lookup",
        &[CONTINUE, CLOSE_RUN],
        "Ended · Round 2",
    );
    // iter-25-R SI1: Final state comes before loot, so every row of it is in view beside
    // Lookup, at every size, after two loot rolls.
    for size in [SMALL, DEFAULT, LARGE] {
        let rows = d.at_size(size, |d| {
            let rows: Vec<_> = d
                .nodes()
                .into_iter()
                .filter(|node| node.name.contains(", HP ") && node.name.contains(", AC "))
                .collect();
            let hidden: Vec<_> = rows
                .iter()
                .filter(|row| !row.fully_in_view())
                .map(|row| row.name.clone())
                .collect();
            (rows.len(), hidden)
        });
        d.observe(
            "iter-25-R-SI1",
            rows.0 >= 6 && rows.1.is_empty(),
            format!("At {size:?} every Final state row is in view above the loot: {rows:?}"),
        );
    }
    d.state("ended-with-loot-beside-lookup", Sizes::All);
    close_lookup(d);
    whole_at_every_size(
        d,
        "T17.6",
        "The ended view with loot",
        &[CONTINUE, CLOSE_RUN],
        "Ended · Round 2",
    );
    d.state("ended-continue-fight", Sizes::All);

    // Continue fight is for the fight that was played last. A long rest ruled after the
    // ending has a later place, so the ended view says why the fight cannot go on, where
    // the control was. Nothing else in the view moves.
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.click("Add Long rest");
    d.click(BACK_TO_RUN);
    d.observe(
        "T17.8",
        !d.has(CONTINUE) && d.visible_passage(LATER) && d.in_view(CLOSE_RUN),
        "After a later rest, the ended view offers no Continue fight and says why",
    );
    whole_at_every_size(
        d,
        "T17.6",
        "The ended view of a fight that is not the last",
        &[CLOSE_RUN, LATER],
        "Ended · Round 2",
    );
    d.state("ended-not-the-last-fight", Sizes::All);
    open_lookup(d);
    whole_at_every_size(
        d,
        "T17.6",
        "The ended view of a fight that is not the last, beside Lookup",
        &[CLOSE_RUN, LATER],
        "Ended · Round 2",
    );
    d.state("ended-not-the-last-fight-beside-lookup", Sizes::All);
    close_lookup(d);
    // The rest keeps its place whatever it restored, because the DM can change that ruling.
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.click("Long rest");
    d.click("Mark not at full");
    d.click(BACK_TO_RUN);
    d.observe(
        "T17.8",
        !d.has(CONTINUE) && d.visible_passage(LATER),
        "A rest that restored nobody is still later: no Continue fight",
    );
    // The DM takes the rest away. The fight is the last again.
    let rest = d
        .app_mut()
        .timeline
        .events
        .iter()
        .find(|event| event.kind == crate::timeline::Kind::LongRest)
        .cloned()
        .unwrap();
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.click("More actions for Long rest");
    d.click("Remove this event");
    d.click(BACK_TO_RUN);
    d.observe(
        "T17.8",
        d.has(CONTINUE) && !d.visible_passage(LATER),
        "With no later fight or rest, Continue fight is offered again",
    );
    // The rule reads every event, so it is worked out when the Timeline changes, and not on
    // each frame. An event put back with no word of a change is not seen until there is one.
    d.app_mut().timeline.events.push(rest.clone());
    d.settle();
    let kept = d.has(CONTINUE) && !d.visible_passage(LATER);
    d.app_mut().timeline.touch();
    d.settle();
    d.observe(
        "T17.8",
        kept && !d.has(CONTINUE) && d.visible_passage(LATER),
        format!("The rule is worked out when the Timeline changes, not on a quiet frame: {kept}"),
    );
    d.app_mut()
        .timeline
        .events
        .retain(|event| event.id != rest.id);
    d.app_mut().timeline.touch();
    d.settle();
    // An outside edit makes Timeline.md unreadable. Splat cannot tell which fight was played
    // last, so the ended view says that. When the file can be read again, the control is back.
    let campaign = d.app_mut().campaigns.active_campaign().unwrap();
    let timeline = crate::sessions::folder(&campaign)
        .unwrap()
        .join(crate::timeline::FILE);
    d.app_mut().timeline.save();
    let valid = std::fs::read(&timeline).unwrap();
    std::fs::write(&timeline, "---\ntimeline_version: [broken\n---\n").unwrap();
    d.app_mut().timeline.reload();
    d.settle();
    d.observe(
        "T17.8",
        !d.has(CONTINUE) && d.visible_passage(NO_TIMELINE),
        "While Timeline.md cannot be read, the ended view offers no Continue fight and says why",
    );
    whole_at_every_size(
        d,
        "T17.6",
        "The ended view while Timeline.md cannot be read",
        &[CLOSE_RUN, NO_TIMELINE],
        "Ended · Round 2",
    );
    d.state("ended-timeline-unreadable", Sizes::All);
    open_lookup(d);
    whole_at_every_size(
        d,
        "T17.6",
        "The ended view while Timeline.md cannot be read, beside Lookup",
        &[CLOSE_RUN, NO_TIMELINE],
        "Ended · Round 2",
    );
    close_lookup(d);
    std::fs::write(&timeline, &valid).unwrap();
    d.app_mut().timeline.reload();
    d.settle();
    d.observe(
        "T17.8",
        d.has(CONTINUE) && !d.visible_passage(NO_TIMELINE),
        "When Timeline.md can be read again, Continue fight is offered again",
    );
    // A Timeline note waits for its save, and another copy of Splat changes Timeline.md: first
    // with nothing that matters to the rule, then with a later rest. Continue fight cannot be
    // sure which fight was played last, so it refuses and says so in its place. The save that
    // settles it works the rule out again, and only the true sentence stays.
    let notes = d.app_mut().timeline.events[0].notes.clone();
    for later in [false, true] {
        d.app_mut().timeline.events[0].notes = format!("A note that waits for its save, {later}");
        d.app_mut().timeline.touch();
        if later {
            let mut other = crate::timeline::Book::default();
            other.open(timeline.parent());
            other.add_long_rest(None);
            other.save();
        } else {
            let mut bytes = std::fs::read(&timeline).unwrap();
            bytes.push(b'\n');
            std::fs::write(&timeline, bytes).unwrap();
        }
        d.click(CONTINUE);
        let ended = d.app_mut().encounters.run().unwrap().ended();
        d.observe(
            "T17.8",
            ended && d.visible_passage(DISK_CHANGED) && !d.has(CONTINUE),
            format!(
                "A file changed under an unsaved note refuses Continue fight (later rest {later})"
            ),
        );
        d.app_mut().timeline.save();
        let settled = !d.app_mut().timeline.unsaved() && !d.app_mut().timeline.conflict;
        d.click(NAV_TIMELINE);
        d.click(BACK_TO_RUN);
        d.observe(
            "T17.8",
            settled
                && !d.visible_passage(DISK_CHANGED)
                && if later {
                    d.visible_passage(LATER) && !d.has(CONTINUE)
                } else {
                    d.has(CONTINUE)
                },
            format!("A settled save must clear the disk-change instruction (later rest {later})"),
        );
    }
    // The rest and the note go again, so the fight is the last one, as before.
    d.app_mut()
        .timeline
        .events
        .retain(|event| event.kind != crate::timeline::Kind::LongRest);
    d.app_mut().timeline.events[0].notes = notes;
    d.app_mut().timeline.touch();
    d.app_mut().timeline.save();
    d.settle();
    let told = story(d);
    d.observe(
        "T17.4",
        told == "Undecided, round 2",
        format!("Timeline has the ending: {}", told),
    );
    open_lookup(d);
    d.begin_task();
    d.click(CONTINUE);
    d.goal_within("T17.2", "go on with an ended fight", 1);
    d.observe(
        "T17.2",
        current_participant(d).as_deref() == Some(turn.as_str())
            && d.in_view(NEXT_TURN)
            && undo_step(d) == CONTINUE_STEP,
        format!(
            "Continue fight returns to the turn the fight ended on, as one named step: {:?}, {}",
            current_participant(d),
            undo_step(d)
        ),
    );
    d.observe(
        "T17.6",
        lookup_keeps_its_entry(d),
        "Continue fight leaves Lookup on its entry",
    );
    d.state("fight-continued-beside-lookup", Sizes::All);
    close_lookup(d);
    // Everything that reads the fight says it is live again.
    let told = story(d);
    d.observe(
        "T17.4",
        told == "Round 2",
        format!("Timeline says the fight is live: {}", told),
    );
    go_encounters(d);
    d.observe(
        "T17.4",
        d.has_label("IN PROGRESS 2") && d.visible_text("No ended fights yet."),
        format!(
            "Encounters lists the continued fight in In progress, and History has none: {:?}",
            d.name_starting("IN PROGRESS")
        ),
    );
    d.click(NAV_SESSIONS);
    show_plan(d);
    // The row itself, and not the strip above the page: its name, with the link to the
    // running fight on the line below it.
    let row = d.place_of(RUN_VAULT);
    let running = d
        .nodes()
        .into_iter()
        .find(|node| node.name == "Round 2" && node.is_link())
        .map(|node| node.rect);
    d.observe(
        "T17.4",
        row.zip(running).is_some_and(|(row, link)| {
            link.top() >= row.top() && link.top() - row.bottom() <= crate::design::XL
        }) && !d.has_containing("Undecided · Ended"),
        format!("The plan's row is a running fight again: row {row:?}, its link {running:?}"),
    );
    d.state("plan-with-continued-fight", Sizes::Default);
    d.click(BACK_TO_RUN);
    // Undo of Continue fight is the ended fight again, with its loot.
    d.click_containing(CONTINUE_STEP);
    d.expect("Outcome: Undecided");
    let told = story(d);
    d.observe(
        "T17.2",
        undo_step(d) == LOOT_STEP && d.visible_text(" on 1d4") && told == "Undecided, round 2",
        format!(
            "Undo of Continue fight restores the ending, its loot and its Timeline result: {}, {}",
            undo_step(d),
            told
        ),
    );

    // The ended run is closed, and the app starts again. The plan's played row leads to the
    // run in History, which offers the same action.
    close_run(d);
    d.restart();
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.begin_task();
    d.click(RUN_VAULT);
    let played: Vec<_> = d
        .nodes()
        .into_iter()
        .map(|node| node.name.replace('\u{a0}', " "))
        .filter(|name| name.contains("Ended"))
        .collect();
    d.observe(
        "T17.4",
        played
            .iter()
            .any(|name| name == "Undecided · Ended · Round 2"),
        format!("The plan's played row has the result and the last round: {played:?}"),
    );
    d.state("plan-played-row", Sizes::Default);
    d.click("Whole run");
    d.expect("Read-only");
    whole_at_every_size(d, "T17.6", "The ended run in History", &[CONTINUE], "");
    d.state("history-continue-fight", Sizes::All);
    d.click(CONTINUE);
    d.goal_within("T17.3", "go on with a closed, ended fight from the plan", 3);
    let told = story(d);
    d.observe(
        "T17.3",
        current_participant(d).as_deref() == Some(turn.as_str())
            && d.in_view(NEXT_TURN)
            && undo_step(d) == CONTINUE_STEP
            && told == "Round 2",
        format!(
            "From History, Continue fight opens the run on the turn it ended on: {:?}, {}, {}",
            current_participant(d),
            undo_step(d),
            told
        ),
    );
    // The same from the Encounters page, with Lookup open: one press on the open row.
    d.click_containing(CONTINUE_STEP);
    d.expect("Outcome: Undecided");
    close_run(d);
    go_encounters(d);
    if !d.has_containing("Open the run of Sunken Vault") {
        d.click_starting(HISTORY);
    }
    open_lookup(d);
    d.begin_task();
    d.click_starting("Open the run of Sunken Vault");
    d.expect("Read-only");
    whole_at_every_size(
        d,
        "T17.6",
        "The ended run in History beside Lookup",
        &[CONTINUE, "New encounter", "Reload from disk"],
        "",
    );
    d.state("history-beside-lookup", Sizes::All);
    d.click(CONTINUE);
    d.goal_within("T17.3", "go on with a closed, ended fight from History", 2);
    d.observe(
        "T17.6",
        lookup_keeps_its_entry(d) && d.in_view(NEXT_TURN) && undo_step(d) == CONTINUE_STEP,
        "Continue fight from History leaves Lookup on its entry",
    );
    close_lookup(d);
    // The steps from before the restart are still behind it.
    d.click_containing(CONTINUE_STEP);
    d.expect("Outcome: Undecided");
    d.observe(
        "T17.1",
        undo_step(d) == LOOT_STEP,
        format!(
            "After a restart, the ended run still has its loot step: {}",
            undo_step(d)
        ),
    );
    // The fight goes on and ends a second time, with another result.
    d.click(CONTINUE);
    d.click(NEXT_TURN);
    end_encounter(d);
    d.type_text("Held the vault");
    d.press(Key::Enter);
    d.expect("Outcome: Held the vault");
    let told = story(d);
    d.observe(
        "T17.4",
        told == "Held the vault, round 2" && undo_step(d) == END_STEP && !d.has(CONTINUE),
        format!(
            "A second ending is an ending like the first: {}, {}",
            told,
            undo_step(d)
        ),
    );
    // The party searches the guard, so Continue fight is the way on again. The next ending
    // starts with the outcome of the ending that went, selected: Enter keeps it.
    d.click("Roll gems for Vault Guard");
    d.click(CONTINUE);
    // The app starts again, so nothing typed before is left in the field: the outcome
    // comes from the run file.
    d.restart();
    end_encounter(d);
    let offered = d.field_text("Outcome");
    d.press(Key::Enter);
    d.observe(
        "T17.7",
        offered.as_deref() == Some("Held the vault") && d.has("Outcome: Held the vault"),
        format!("The outcome field starts with the old outcome, and Enter keeps it: {offered:?}"),
    );
    // Typing replaces it.
    d.click_starting("Undo");
    end_encounter(d);
    d.type_text("Fled");
    d.press(Key::Enter);
    let outcome = d
        .nodes()
        .into_iter()
        .find(|node| node.name.starts_with("Outcome: "))
        .map_or_else(String::new, |node| node.name);
    d.observe(
        "T17.7",
        outcome == "Outcome: Fled",
        format!("Typing replaces the selected outcome: {outcome}"),
    );
    // Close run writes the fight's last result to Timeline.md at once, and not after the
    // pause before a save: a closed run cannot say it again after a crash. The file is read
    // here before anything else can save it.
    let before = std::fs::read_to_string(&timeline).unwrap_or_default();
    close_run(d);
    let after = std::fs::read_to_string(&timeline).unwrap_or_default();
    d.observe(
        "T17.4",
        !before.contains("Fled, round 2") && after.contains("Fled, round 2"),
        format!(
            "Close run puts the last result in Timeline.md at once: before {}, after {}",
            before.contains("Fled, round 2"),
            after.contains("Fled, round 2")
        ),
    );
    go_encounters(d);
    d.observe(
        "T17.4",
        d.has_label("IN PROGRESS 1") && !d.visible_text("No ended fights yet."),
        "After the second ending, the fight has left In progress for History",
    );

    // A file whose Undo steps this version cannot read: the run opens, and says so once.
    let paused = d
        .app_mut()
        .encounters
        .saved_runs()
        .into_iter()
        .find(|run| run.encounter == RUN)
        .unwrap();
    let mut saved: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&paused.path).unwrap()).unwrap();
    saved["undo"]["version"] = 99.into();
    std::fs::write(&paused.path, serde_json::to_vec_pretty(&saved).unwrap()).unwrap();
    resume_run(d, RUN);
    d.observe(
        "T17.5",
        d.visible_passage(UNREAD) && d.in_view(NEXT_TURN) && d.in_view(DISMISS),
        "The run opens on its turn and says that its Undo steps could not be read",
    );
    // iter-25-R AI3, FI2: the warning says the cause, so Undo's reason does not repeat it.
    const UNREAD_REASON: &str = "Nothing to undo. Splat could not read the saved Undo steps.";
    d.observe(
        "iter-25-R-AI3-short",
        d.visible_text("Nothing to undo") && !d.visible_text("could not read the saved Undo steps"),
        "While the warning shows, Undo's reason says only Nothing to undo",
    );
    d.state("undo-steps-unread", Sizes::All);
    open_lookup(d);
    whole_at_every_size(
        d,
        "T17.6",
        "The unread-steps message beside Lookup",
        &[DISMISS, NEXT_TURN],
        "",
    );
    d.state("undo-steps-unread-beside-lookup", Sizes::All);
    d.click(DISMISS);
    d.observe(
        "T17.5",
        !d.visible_passage(UNREAD) && lookup_keeps_its_entry(d),
        "Dismiss removes the message and leaves Lookup on its entry",
    );
    close_lookup(d);
    d.observe(
        "iter-25-R-AI3-after-dismiss",
        d.visible_text(UNREAD_REASON),
        "After Dismiss, Undo's reason gives the cause, with a full stop",
    );
    d.click(NEXT_TURN);
    d.restart();
    d.observe(
        "T17.5",
        !d.visible_passage(UNREAD) && undo_step(d) == "Undo: next turn",
        format!(
            "It is said once, and Undo starts again from there: {}",
            undo_step(d)
        ),
    );

    // That fight was played after the vault's. History offers no Continue fight for the
    // vault and says why, where the control was.
    go_encounters(d);
    if !d.has_containing("Open the run of Sunken Vault") {
        d.click_starting(HISTORY);
    }
    d.click_starting("Open the run of Sunken Vault");
    d.expect("Read-only");
    // iter-25-R FI1: the vault's encounter is deleted, so no new run can start from it. The
    // reason gives only its cause.
    d.observe(
        "iter-25-R-FI1",
        d.visible_text("Encounter deleted")
            && !d.has(CONTINUE)
            && d.visible_passage(LATER_GONE)
            && !d.visible_text("Start a new run"),
        "After a later fight, History offers no Continue fight and says why, with no step a deleted encounter cannot take",
    );
    whole_at_every_size(
        d,
        "T17.6",
        "The History run that is not the last fight",
        &[LATER_GONE],
        "",
    );
    d.state("history-not-the-last-fight", Sizes::All);
    open_lookup(d);
    whole_at_every_size(
        d,
        "T17.6",
        "The History run that is not the last fight, beside Lookup",
        &[LATER_GONE, "New encounter", "Reload from disk"],
        "",
    );
    d.state("history-not-the-last-fight-beside-lookup", Sizes::All);
    d.observe(
        "T17.6",
        lookup_keeps_its_entry(d),
        "The reason in History leaves Lookup on its entry",
    );
    close_lookup(d);
}
