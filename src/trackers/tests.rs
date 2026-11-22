use super::*;
use crate::lookup::ContentIndex;
use ledger::{Entry, Ledger, NoticeRef, Op, Roll, TableRoll};

fn today(book: &crate::timeline::Book) -> Today {
    super::today(&clock(book))
}
use crate::{content_workspace::CampaignDocument, encounter_settings::Options};
use std::fs;

/// M-S0.3: an act that wrote nothing, as the footer answers it: the act, the domain's cause
/// (`Refused`) and the one next step (`refusal`).
fn refused(act: crate::design::results::FailedAct, cause: Refused) -> Unrecorded {
    Unrecorded::Refused(refusal(act, cause))
}

/// A tracker's act by its label or notice name: `Nothing was recorded for <act>.`
fn recorded(act: &str) -> crate::design::results::FailedAct {
    crate::design::results::FailedAct::NotRecorded { act: named(act) }
}

/// The Undo of a tracker's act: `Nothing was undone for <act>.`
fn undone(act: &str) -> crate::design::results::FailedAct {
    crate::design::results::FailedAct::NotUndone { act: named(act) }
}

/// A history entry's Remove, or its Undo (`back`), by the entry's label.
fn history(label: &str, back: bool) -> crate::design::results::FailedAct {
    use crate::design::results::{FailedAct, Kind, Thing};
    let what = Thing::new(Kind::Entry, named(label));
    let file = named(ledger::FILE);
    if back {
        FailedAct::NotPutBack { what, into: file }
    } else {
        FailedAct::NotRemoved { what, from: file }
    }
}

/// The words of the act's refusal, as the footer says them.
fn refused_words(reason: &Option<Unrecorded>) -> Option<String> {
    match reason {
        Some(Unrecorded::Refused(refusal)) => Some(refusal.words().to_string()),
        _ => None,
    }
}

const EXAMPLE: &str = include_str!("../../docs/design/world-examples/heralds-tournament.md");

/// Review of PR 284, finding 3: the day count that day marks use is the clock's, so a fight
/// placed inside a journey never takes it back, and a fight that ends after the journey
/// carries it on.
#[test]
fn pr284_review_the_day_count_never_goes_back_for_a_fight_inside_a_journey() {
    use crate::timeline::{Kind, RunFact, RunState, Within};
    let session = Some(Uuid::from_u128(1));
    let mut book = crate::timeline::Book::default();
    book.add(Kind::Day, session);
    let road = book.add(Kind::Travel, session);
    book.event_mut(&road).unwrap().duration = "2 days".into();
    assert_eq!(today(&book), Today::Day(3));
    let fact = |state| RunFact {
        id: Uuid::from_u128(7),
        copied_from: None,
        name: "Wolf Pack".into(),
        round: 1,
        state,
        steps: 1,
    };
    let within = Within {
        journey: road,
        at: 24,
    };
    book.record_change(
        Some(&fact(RunState::Setup)),
        &fact(RunState::Running),
        session,
        Some(&within),
    );
    assert_eq!(
        today(&book),
        Today::Day(3),
        "the fight is on Day 2; the count stays"
    );
    let fight = book.events.iter().position(|e| e.run.is_some()).unwrap();
    book.events[fight].duration = "2 days".into();
    assert_eq!(today(&book), Today::Day(4), "the fight ends on Day 4");
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
struct Race {
    _dir: tempfile::TempDir,
    campaign: Campaign,
    trackers: Trackers,
    index: Arc<ContentIndex>,
    /// The campaign's Timeline: the day count's one owner.
    timeline: crate::timeline::Book,
}

const TONIGHT: Uuid = Uuid::from_u128(0x0070_eca3_6331_c46c_b5a6_32a6_e352_65da);

fn race() -> Race {
    let dir = tempfile::tempdir().unwrap();
    let campaign = Campaign {
        id: Uuid::new_v4(),
        name: "Heralds".into(),
        path: dir.path().into(),
        dndbeyond_campaign_id: None,
        dndbeyond_campaign_name: None,
    };
    CampaignDocument::new(&campaign).unwrap().save().unwrap();
    let start = EXAMPLE.find("````markdown\n").unwrap() + "````markdown\n".len();
    let end = start + EXAMPLE[start..].find("````").unwrap();
    let id = Uuid::new_v4();
    let note = EXAMPLE[start..end].replace("<a new UUID>", &id.to_string());
    fs::write(dir.path().join("Tournament.md"), note).unwrap();
    let wander = [
        ("1-7", "Nothing comes."),
        ("8", "Two Sand Wraiths at the next door."),
    ];
    let tables = [
        table(
            "Rival Track",
            "rival_track",
            "1d6",
            &[
                ("1", "Loses its way."),
                ("2–5", "Moves on."),
                ("6", "Takes a seal."),
            ],
        ),
        table("Wandering, 0:00 to 0:39", "wander_early", "1d8", &wander),
        table("Wandering, 0:40 to 1:19", "wander_middle", "1d8", &wander),
        table("Wandering, 1:20 and later", "wander_late", "1d8", &wander),
        table("Gifts", "gifts", "1d6", &[("1-6", "A healing potion.")]),
    ];
    fs::write(
        dir.path().join("Undercroft Tables.md"),
        format!("# Undercroft Tables\n\n{}", tables.concat()),
    )
    .unwrap();
    // The switch in Campaign → Rules and runs puts the note in play.
    let catalogue = crate::content_workspace::Catalogue::load(&[], Some(&campaign)).unwrap();
    let mut options = Options::load(&campaign, &catalogue).unwrap();
    assert_eq!(options.house_rules, vec![(id, "Tournament".to_owned())]);
    assert!(options.in_play.is_empty());
    options.in_play.push(id);
    let document = CampaignDocument::load(&campaign).unwrap();
    options.save(&campaign, &document.text).unwrap();
    assert_eq!(
        Options::load(&campaign, &catalogue).unwrap().in_play,
        vec![id]
    );
    let index = Arc::new(ContentIndex::load(&[], Some(&campaign)).unwrap());
    let mut timeline = crate::timeline::Book::default();
    timeline.open(Some(dir.path()));
    let mut trackers = Trackers::with_ids(crate::hermetic::new_uuid);
    poll(
        &mut trackers,
        Some(index.clone()),
        Some(&campaign),
        Some(TONIGHT),
        &timeline,
    );
    Race {
        _dir: dir,
        campaign,
        trackers,
        index,
        timeline,
    }
}

fn value(race: &Race, key: &str, instance: Option<&str>) -> i64 {
    race.trackers
        .state
        .value(&race.trackers.rules, key, instance)
}

fn file(race: &Race) -> String {
    fs::read_to_string(ledger::path(&race.campaign.path)).unwrap_or_default()
}

fn newest_label(trackers: &Trackers) -> String {
    label(&trackers.rules, &trackers.ledger, trackers.newest())
}

#[test]
fn the_tournament_note_is_read_as_written_and_a_race_runs_between_fights() {
    let mut race = race();
    let rules = &race.trackers.rules;
    assert_eq!(rules.notes, ["Tournament"]);
    let keys: Vec<_> = rules.trackers.iter().map(|t| t.key.as_str()).collect();
    assert_eq!(
        keys,
        [
            "race_clock",
            "favor",
            "souls",
            "seals",
            "members",
            "activation"
        ]
    );
    for tracker in &rules.trackers {
        assert!(tracker.def.is_ok(), "{}: {:?}", tracker.key, tracker.def);
    }
    assert!(
        !rules.def("activation").unwrap().pin,
        "activation is not pinned"
    );
    // The note is a valid entry: no warning in the Library or the Campaign's review.
    let note = race
        .index
        .items
        .iter()
        .find(|item| item.entry.name == "Tournament")
        .unwrap();
    let checked = splat::content::validate(std::slice::from_ref(&note.entry));
    assert!(
        checked.warnings.is_empty() && checked.errors.is_empty(),
        "{:?} {:?}",
        checked.warnings,
        checked.errors
    );
    assert_eq!(race.trackers.due(), 0);
    assert_eq!(file(&race), "", "nothing is written before an act");

    // A press that passes a mark raises its notice, and the result line names both.
    let t = &mut race.trackers;
    t.press("race_clock", None, "Cistern or Sands", None);
    assert_eq!(t.reason, None);
    assert_eq!(t.due(), 1);
    let words = result_words(&t.rules, &t.ledger, &t.state, t.newest());
    assert_eq!(words, "Elapsed race time changed to 20. Bell is due.");
    assert_eq!(newest_label(t), "Cistern or Sands");
    // The result line's label and words are worked out with the act, not in each frame.
    assert_eq!(t.test_result(), Some(("Cistern or Sands".into(), words)));

    // `for:` rolls for the next name, and then the button is off.
    let bell = t.state.due[0].clone();
    for name in ["Old Guard", "Keyring", "Convicts"] {
        t.roll_notice(&bell, "rival_track");
        let roll = t
            .state
            .notice(&bell.id)
            .unwrap()
            .rolls
            .last()
            .unwrap()
            .clone();
        assert_eq!(roll.name.as_deref(), Some(name));
        assert_eq!(newest_label(t), "Rival Track roll");
        // The result is a sentence; Undo keeps the label.
        assert_eq!(
            result_words(&t.rules, &t.ledger, &t.state, t.newest()),
            format!(
                "Rolled {} on Rival Track for {name}: {}",
                roll.value, roll.row
            )
        );
    }
    let before = file(&race);
    let t = &mut race.trackers;
    t.roll_notice(&bell, "rival_track");
    assert_eq!(
        t.reason,
        Some(refused(
            crate::design::results::FailedAct::NotRolled {
                table: named(t.rules.table_name("rival_track")),
            },
            Refused::EveryNameRolled
        ))
    );
    assert_eq!(file(&race), before, "a refused act writes nothing");
    // A table with no names rolls as often as it is pressed.
    let t = &mut race.trackers;
    t.roll_notice(&bell, "wander_early");
    t.roll_notice(&bell, "wander_early");
    assert_eq!(t.state.notice(&bell.id).unwrap().rolls.len(), 5);
    t.done(&bell, &[]);
    assert_eq!(t.due(), 0);
    assert_eq!(newest_label(t), "Bell done");
    assert_eq!(
        result_words(&t.rules, &t.ledger, &t.state, t.newest()),
        "Bell is done."
    );

    // `min` blocks a press, and says why beside it.
    let before = file(&race);
    let t = &mut race.trackers;
    t.press("favor", None, "Throw a gift", None);
    // The tracker's own limit ends at its cause (M13).
    assert_eq!(
        refused_words(&t.reason).as_deref(),
        Some("Nothing was recorded for Throw a gift. Crowd Favor cannot go below 0.")
    );
    assert_eq!(file(&race), before);
    let t = &mut race.trackers;
    t.press("favor", None, "Crowd loves it", None);
    t.press("favor", None, "Throw a gift", None);
    let words = result_words(&t.rules, &t.ledger, &t.state, t.newest()).to_string();
    assert!(
        words.starts_with("Crowd Favor changed to 0. Rolled ")
            && words.ends_with(": A healing potion."),
        "{words}"
    );
    // One change that passes two marks names both, in one sentence.
    let jump = Ledger {
        changes: vec![Entry {
            id: "c-0000j040".into(),
            tracker: Some("race_clock".into()),
            add: Some(40),
            why: Some("Enter room".into()),
            ..Entry::default()
        }],
        ..Ledger::default()
    };
    let jumped = replay(&t.rules, &jump, &Today::Day(1));
    assert_eq!(
        result_words(&t.rules, &jump, &jumped, &jump.changes),
        "Elapsed race time changed to 40. Bell at 20 and Bell at 40 are due."
    );

    // An instance has its own value and its own notice.
    t.press("seals", Some("Old Guard"), "Take seal", None);
    assert_eq!(value(&race, "seals", Some("Old Guard")), 1);
    assert_eq!(value(&race, "seals", Some("Party")), 0);
    let t = &mut race.trackers;
    assert_eq!(t.due(), 1);
    assert_eq!(
        short_name(&t.rules, "seals", Some("Old Guard"), 1),
        "First seal for Old Guard"
    );
    // The DM clicks Dismiss on that notice; a moment before, Obsidian moved its seal to
    // Party. The click writes nothing and says so, and the view draws from the file.
    let drawn = t.state.due[0].clone();
    let path = ledger::path(&race.campaign.path);
    let before = fs::read_to_string(&path).unwrap();
    fs::write(&path, before.replace("for: Old Guard", "for: Party")).unwrap();
    let t = &mut race.trackers;
    t.dismiss(&drawn);
    assert_eq!(
        t.reason,
        Some(refused(
            recorded(&drawn.name(&t.rules)),
            Refused::Changed(Part::Notice)
        ))
    );
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        before.replace("for: Old Guard", "for: Party")
    );
    fs::write(&path, &before).unwrap();
    let t = &mut race.trackers;
    t.read_disk();
    assert_eq!(t.state.due[0], drawn);
    // A wrong press seen late: the opposite action corrects the value, and the history
    // keeps both presses. The notice the wrong press raised had nothing on it, so the value
    // back below its mark ends it; nothing is written for that.
    t.press("seals", Some("Old Guard"), "Lose seal", None);
    assert_eq!(value(&race, "seals", Some("Old Guard")), 0);
    let t = &mut race.trackers;
    assert_eq!(
        t.due(),
        0,
        "a notice with nothing on it ends when its value falls"
    );
    assert_eq!(
        result_words(&t.rules, &t.ledger, &t.state, t.newest()),
        "Seals for Old Guard changed to 0. First seal for Old Guard is no longer due."
    );
    // M-S0.1 (M0565): the history says the lowering with a true minus (`signed`).
    let lowered_row = t.test_history()["seals"]
        .iter()
        .map(|row| row.words.clone())
        .find(|words| words.starts_with("Lose seal"));
    assert!(
        lowered_row
            .as_deref()
            .is_some_and(|words| words.ends_with("\u{2212}1")),
        "m-s0-1-history-minus: {lowered_row:?}"
    );
    let lowered = t.newest().to_vec();
    t.undo(&lowered);
    assert_eq!(
        t.state.due,
        [drawn],
        "Undo of the fall brings the notice back"
    );

    // One press that changes two trackers is two entries with one `use`; Undo takes both.
    t.press("race_clock", None, "Collect soul", None);
    let pressed = t.newest().to_vec();
    assert_eq!(pressed.len(), 2);
    assert!(pressed[0].press.is_some() && pressed[0].press == pressed[1].press);
    assert_eq!(value(&race, "souls", None), 1);
    let t = &mut race.trackers;
    assert_eq!(
        result_words(&t.rules, &t.ledger, &t.state, t.newest()),
        "Elapsed race time changed to 30. Souls in the jar changed to 1."
    );
    t.undo(&pressed);
    assert_eq!(value(&race, "souls", None), 0);
    assert_eq!(value(&race, "race_clock", None), 20);
    // The line then names the act before it: Undo steps back, newest first.
    assert_eq!(newest_label(&race.trackers), "Take seal");
    // A dice amount records its roll; `set` records what it replaced.
    let t = &mut race.trackers;
    t.press("activation", None, "Touch controls", None);
    let rolled = value(&race, "activation", None);
    assert!((1..=6).contains(&rolled));
    let t = &mut race.trackers;
    t.press("activation", None, "Missed twice or Ring reset", None);
    assert_eq!(t.newest()[0].was, Some(rolled));
    assert_eq!(value(&race, "activation", None), 0);

    // An act reads the definitions on disk at the click. Done on a mark whose `at` changed
    // in Obsidian, and a press on a tracker whose note was deleted, write nothing.
    let drawn = race.trackers.state.due[0].clone();
    let note = race.campaign.path.join("Tournament.md");
    let text = fs::read_to_string(&note).unwrap();
    assert!(text.contains("at: 1\n"));
    fs::write(&note, text.replace("at: 1\n", "at: 2\n")).unwrap();
    let before = file(&race);
    let t = &mut race.trackers;
    // The refusal names the notice as the view drew it, before the act read the changed note.
    let shown = drawn.name(&t.rules);
    t.done(&drawn, &[]);
    assert_eq!(
        t.reason,
        Some(refused(recorded(&shown), Refused::RulesChanged))
    );
    assert_eq!(file(&race), before);
    fs::remove_file(&note).unwrap();
    let t = &mut race.trackers;
    t.press("favor", None, "Crowd loves it", None);
    assert_eq!(
        t.reason,
        Some(refused(recorded("Crowd loves it"), Refused::RulesChanged))
    );
    assert_eq!(file(&race), before);
}

/// Two day marks for the Tournament: the month's end every 30 days from Day 2, as the design
/// counts it, and the wagon on one day, with a change for one line of a tracker with `per`.
/// The wagon comes second in the note and first in the Due list: oldest day first.
const DAY_MARKS: &str = "> day_marks:\n>   month_end:\n>     name: End of month\n>     every: 30\n>     from: 2\n>     say: Move the rival powers.\n>     changes:\n>     - {tracker: favor, add: 1}\n>   wagon:\n>     name: Supply wagon\n>     at: 3\n>     say: The supply wagon arrives. The party takes a seal.\n>     changes:\n>     - {tracker: seals, for: Party, add: 1}\n";

fn with_day_marks(race: &mut Race) {
    let note = race.campaign.path.join("Tournament.md");
    let text = fs::read_to_string(&note).unwrap();
    let marked = text.replacen("> trackers:\n", &format!("{DAY_MARKS}> trackers:\n"), 1);
    assert_ne!(marked, text);
    fs::write(&note, marked).unwrap();
    race.index = Arc::new(ContentIndex::load(&[], Some(&race.campaign)).unwrap());
    let index = Some(race.index.clone());
    poll(
        &mut race.trackers,
        index,
        Some(&race.campaign),
        Some(TONIGHT),
        &race.timeline,
    );
}

/// An event on the Timeline, saved, and the day count followed as the shell does.
fn add_event(race: &mut Race, kind: crate::timeline::Kind, length: &str) -> String {
    let id = race.timeline.add(kind, Some(TONIGHT));
    race.timeline.event_mut(&id).unwrap().duration = length.into();
    race.timeline.touch();
    race.timeline.save();
    race.trackers.follow_clock(clock(&race.timeline));
    id
}

fn day(mark: &str, day: u64) -> NoticeRef {
    NoticeRef::Day {
        mark: mark.into(),
        day,
    }
}

#[test]
fn day_marks_follow_the_campaigns_day_count() {
    use crate::timeline::{Book, Kind};
    use std::os::unix::fs::PermissionsExt;
    let mut race = race();
    // With no day mark in play, a Timeline that cannot be read says nothing here.
    let plain = replay(
        &race.trackers.rules,
        &race.trackers.ledger,
        &Today::NotRead("Timeline.md could not be read.".into()),
    );
    assert_eq!(plain.clock_not_read, None);
    with_day_marks(&mut race);
    let marks = &race.trackers.rules.day_marks;
    assert_eq!(marks.len(), 2);
    assert!(marks.iter().all(|mark| mark.def.is_ok()), "{marks:?}");
    // Before the first Day the clock has not started: nothing is due.
    assert_eq!(race.trackers.due(), 0);
    add_event(&mut race, Kind::Day, "");
    add_event(&mut race, Kind::Travel, "2 days");
    let t = &race.trackers;
    assert_eq!((t.due(), &t.state.due[0].id), (1, &day("wagon", 3)));
    assert_eq!(
        file(&race),
        "",
        "a day notice is worked out, and raising it writes nothing"
    );

    // Several days in one step: 60 days of travel pass Day 32 and Day 62. One row, oldest
    // first, acts on its first day.
    let travel = add_event(&mut race, Kind::Travel, "60 days");
    let t = &race.trackers;
    let month = t.state.due[1].clone();
    assert_eq!(
        (t.due(), &month.id, month.days.as_slice()),
        (2, &day("month_end", 32), &[32, 62][..])
    );
    // Done on the wagon applies its change to the line it names; that raises Party's seal.
    let wagon = t.state.due[0].clone();
    race.trackers.done(&wagon, &[true]);
    assert_eq!(value(&race, "seals", Some("Party")), 1);
    let t = &race.trackers;
    assert_eq!(newest_label(t), "Supply wagon, Day 3 done");
    // Next ends the oldest day only, with today's values.
    race.trackers.done(&month, &[true]);
    let t = &race.trackers;
    assert_eq!(t.reason, None);
    assert_eq!(newest_label(t), "End of month, Day 32 done");
    assert_eq!(value(&race, "favor", None), 1);
    let rows: Vec<_> = t.state.due.iter().map(|n| n.id.clone()).collect();
    assert_eq!(rows[0], day("month_end", 62), "{rows:?}");

    // Remove in a day mark's history, from the note's reader. The change that a Done applied
    // takes its Done with it, so the day is due again; Undo remove puts both back.
    let note = t.rules.day_marks[0].note;
    let applied = t.test_history()["favor"].last().unwrap().entry.clone();
    assert!(
        t.test_history()["month_end"]
            .iter()
            .any(|row| row.entry.done
                && row.entry.press.is_some()
                && row.entry.press == applied.press)
    );
    race.trackers.remove(note, &applied);
    let t = &race.trackers;
    assert_eq!(
        t.test_removed().as_ref().map(|r| r.words.as_str()),
        Some("Removed End of month, Day 32 done: Crowd Favor +1.")
    );
    assert_eq!(
        t.state
            .due
            .iter()
            .find(|n| n.key == "month_end")
            .map(|n| n.days.clone()),
        Some(vec![32, 62])
    );
    assert_eq!(value(&race, "favor", None), 0);
    race.trackers.test_undo_remove();
    assert_eq!(value(&race, "favor", None), 1);
    assert_eq!(race.trackers.state.due[0].id, day("month_end", 62));
    // A day notice comes from the day count, so no entry raises it: the wagon's Done goes
    // with the seal it applied and the seal's notice, and Day 3 is due again.
    let t = &race.trackers;
    let wagon = t.test_history()["wagon"]
        .iter()
        .find(|row| row.entry.done)
        .unwrap()
        .entry
        .clone();
    race.trackers.remove(note, &wagon);
    let t = &race.trackers;
    assert_eq!(t.test_history_reason(), None);
    assert_eq!(value(&race, "seals", Some("Party")), 0);
    assert!(t.state.due.iter().any(|n| n.id == day("wagon", 3)));
    assert!(!t.state.due.iter().any(|n| n.key == "seals"));
    race.trackers.test_undo_remove();
    assert_eq!(value(&race, "seals", Some("Party")), 1);
    let t = &race.trackers;
    let rows: Vec<_> = t.state.due.iter().map(|n| n.id.clone()).collect();
    assert_eq!(rows[0], day("month_end", 62), "{rows:?}");

    // The clock goes back by the Timeline's Undo: a day no longer reached goes away, and a
    // Done stays in the ledger. Forward again, a done day stays done.
    race.timeline.undo_add(&travel);
    race.timeline.save();
    race.trackers.follow_clock(clock(&race.timeline));
    let t = &race.trackers;
    assert!(
        t.state
            .due
            .iter()
            .all(|n| matches!(n.id, NoticeRef::Value { .. }))
    );
    assert!(file(&race).contains("day: 32"));
    add_event(&mut race, Kind::Travel, "90 days");
    add_event(&mut race, Kind::Travel, "1 day");
    let month = race.trackers.state.due[0].clone();
    assert_eq!(month.days, [62, 92]);

    // Dismiss all of a row drawn before the clock moved on writes nothing.
    let before = file(&race);
    add_event(&mut race, Kind::Travel, "30 days");
    race.trackers.dismiss_all(&month);
    assert_eq!(
        race.trackers.reason,
        Some(refused(
            recorded(&month.name(&race.trackers.rules)),
            Refused::Changed(Part::DayCount)
        ))
    );
    assert_eq!(file(&race), before);
    // Dismiss all ends each open day as one act; one Undo takes them back.
    let month = race.trackers.state.due[0].clone();
    assert_eq!(month.days, [62, 92, 122]);
    race.trackers.dismiss_all(&month);
    let t = &race.trackers;
    assert_eq!(newest_label(t), "End of month ×3 dismissed");
    assert!(t.state.due.iter().all(|n| n.key != "month_end"));
    let dismissed = t.newest().to_vec();
    assert_eq!(dismissed.len(), 3);
    // A, then an unrelated B with another stamp, then Undo of A: B stays.
    race.trackers.played = Some(Uuid::from_u128(2));
    race.trackers.press("race_clock", None, "Enter room", None);
    race.trackers.played = Some(TONIGHT);
    race.trackers.test_settle();
    race.trackers.undo(&dismissed);
    let t = &race.trackers;
    assert_eq!(t.state.due[0].days, [62, 92, 122]);
    assert_eq!(value(&race, "race_clock", None), 10, "B’s change stays");

    // An edit in Obsidian a moment before ends Day 62: the click on the row drawn before
    // writes nothing and says so, and the row draws from the file.
    let month = race.trackers.state.due[0].clone();
    let path = ledger::path(&race.campaign.path);
    let outside = format!(
        "{}- id: c-0b51de62\n  notice:\n    mark: month_end\n    day: 62\n  done: true\n- id: c-01dca1e5\n  notice: {{mark: old_calendar, day: 5}}\n  dismissed: true\n---\n",
        file(&race).trim_end().trim_end_matches("---")
    );
    fs::write(&path, &outside).unwrap();
    race.trackers.done(&month, &[true]);
    assert_eq!(
        race.trackers.reason,
        Some(refused(
            recorded(&month.name(&race.trackers.rules)),
            Refused::NoLongerDue
        ))
    );
    assert_eq!(file(&race), outside);
    assert_eq!(race.trackers.state.due[0].days, [92, 122]);
    // An entry for a day mark that no note in play has is kept and named, as a tracker's.
    assert_eq!(race.trackers.state.unknown, [("old_calendar".into(), 1)]);

    // A failed save records nothing and keeps nothing.
    let month = race.trackers.state.due[0].clone();
    let mut mode = fs::metadata(&race.campaign.path).unwrap().permissions();
    mode.set_mode(0o555);
    fs::set_permissions(&race.campaign.path, mode.clone()).unwrap();
    race.trackers.done(&month, &[true]);
    mode.set_mode(0o755);
    fs::set_permissions(&race.campaign.path, mode).unwrap();
    assert!(matches!(
        race.trackers.reason,
        Some(Unrecorded::NotSaved(_))
    ));
    assert_eq!(file(&race), outside);
    assert_eq!(race.trackers.state.due[0].days, [92, 122]);

    // A Timeline that cannot be read: no day notice shows, never an old one, the Due list
    // says why, and an act on a day row writes nothing. A notice at a value still shows.
    let timeline = race.campaign.path.join(crate::timeline::FILE);
    let readable = fs::read(&timeline).unwrap();
    fs::set_permissions(&timeline, fs::Permissions::from_mode(0o000)).unwrap();
    race.timeline.reload();
    race.trackers.follow_clock(clock(&race.timeline));
    let t = &race.trackers;
    let why = t.state.clock_not_read.clone().unwrap_or_default();
    assert!(why.starts_with("Timeline.md could not be read."), "{why}");
    assert_eq!(t.due(), 1, "Party's First seal");
    race.trackers.dismiss(&month);
    assert_eq!(
        race.trackers.reason,
        Some(refused(
            recorded(&month.name(&race.trackers.rules)),
            Refused::TimelineUnreadable
        ))
    );
    assert_eq!(file(&race), outside);
    fs::set_permissions(&timeline, fs::Permissions::from_mode(0o644)).unwrap();

    // A day passes while Splat is closed: another program adds 30 days to the Timeline.
    // After a restart the row and the last act's Undo come from the files.
    let mut other = Book::default();
    other.open(Some(&race.campaign.path));
    let id = other.add(Kind::Travel, None);
    other.event_mut(&id).unwrap().duration = "30 days".into();
    other.touch();
    other.save();
    assert_ne!(fs::read(&timeline).unwrap(), readable);
    let mut book = Book::default();
    book.open(Some(&race.campaign.path));
    let mut again = Trackers::with_ids(crate::hermetic::new_uuid);
    poll(
        &mut again,
        Some(race.index.clone()),
        Some(&race.campaign),
        Some(TONIGHT),
        &book,
    );
    assert_eq!(again.state.due[0].days, [92, 122, 152]);
    assert_eq!(newest_label(&again), "End of month, Day 32 done");

    // The act reads the day at the click. Obsidian takes the Timeline back to Day 3 while
    // the shell's Timeline still holds Day 153; the press in the view writes nothing.
    let mut app = crate::Splat {
        trackers: again,
        ..Default::default()
    };
    app.timeline.open(Some(&race.campaign.path));
    app.trackers.follow_clock(clock(&app.timeline));
    let month = app.trackers.state.due[0].clone();
    assert_eq!(month.days, [92, 122, 152]);
    // Next pressed in the view ends the oldest day, and the row moves on to the next.
    app.trackers.press_done_in_view(&month, vec![true]);
    app.apply_tracker_act();
    assert_eq!(app.trackers.reason, None);
    assert_eq!(app.trackers.state.due[0].days, [122, 152]);
    let month = app.trackers.state.due[0].clone();
    let before = file(&race);
    // A Timeline edit waits for its pause, and under it the file changes on disk, or
    // cannot be read: the day count is not known. One answer serves the act and the view:
    // the click records nothing, no day row shows, the Due list says why, and the draft is
    // kept. When the file reads as it was, the rows come back.
    let saved = fs::read(&timeline).unwrap();
    let edited = app.timeline.events[0].id.clone();
    app.timeline.event_mut(&edited).unwrap().title = "Dawn".into();
    app.timeline.touch();
    let mut outside = String::from_utf8(saved.clone()).unwrap();
    outside.push_str("\nWritten in Obsidian.\n");
    let day_rows = |t: &Trackers| {
        t.state
            .due
            .iter()
            .filter(|n| matches!(n.id, NoticeRef::Day { .. }))
            .count()
    };
    let value_rows = app.trackers.due() - day_rows(&app.trackers);
    for (text, mode, why) in [
        (
            outside.as_bytes(),
            0o644,
            crate::timeline::CHANGED_UNDER_DRAFT,
        ),
        (&saved[..], 0o000, "Timeline.md could not be read."),
    ] {
        fs::write(&timeline, text).unwrap();
        fs::set_permissions(&timeline, fs::Permissions::from_mode(mode)).unwrap();
        app.trackers.press_done_in_view(&month, vec![true]);
        app.apply_tracker_act();
        fs::set_permissions(&timeline, fs::Permissions::from_mode(0o644)).unwrap();
        let t = &app.trackers;
        let shown = t.state.clock_not_read.clone().unwrap_or_default();
        assert!(shown.starts_with(why), "{shown}");
        assert_eq!(
            t.reason,
            Some(refused(
                recorded(&month.name(&t.rules)),
                Refused::TimelineUnreadable
            ))
        );
        assert_eq!((day_rows(t), t.due()), (0, value_rows), "{why}");
        assert_eq!(file(&race), before);
        assert!(app.timeline.unsaved() && app.timeline.events[0].title == "Dawn");
        // The file reads as it was: the rows come back, with the draft still waiting.
        fs::write(&timeline, &saved).unwrap();
        app.timeline.read_if_changed();
        app.trackers.follow_clock(clock(&app.timeline));
        let t = &app.trackers;
        assert_eq!(t.state.clock_not_read, None);
        assert_eq!(t.state.due[0].days, [122, 152]);
    }
    app.timeline.save();
    app.timeline.read_if_changed();
    let mut back = Book::default();
    back.open(Some(&race.campaign.path));
    let ids: Vec<String> = back
        .events
        .iter()
        .filter(|event| event.kind == Kind::Travel)
        .skip(1)
        .map(|event| event.id.clone())
        .collect();
    for id in &ids {
        back.remove(id);
    }
    back.save();
    app.trackers.press_done_in_view(&month, vec![true]);
    app.apply_tracker_act();
    assert_eq!(
        app.trackers.reason,
        Some(refused(
            recorded(&month.name(&app.trackers.rules)),
            Refused::NoLongerDue
        ))
    );
    assert_eq!(file(&race), before);
    assert!(app.trackers.state.due.iter().all(|n| n.key != "month_end"));
}

#[test]
fn a_linked_change_names_a_line_its_tracker_has() {
    let def = |yaml: &str| serde_yaml_ng::from_str::<Definition>(yaml).unwrap();
    let rules = Rules {
        notes: vec!["Rules".into()],
        trackers: vec![
            Tracker {
                key: "teams".into(),
                note: Uuid::nil(),
                def: Ok(def(
                    "name: Teams\nper: [Party]\nquick:\n- label: Score\n  changes: [{tracker: rivals, add: 1}]\n- label: Score for Old Guard\n  changes: [{tracker: rivals, for: Old Guard, add: 1}]",
                )),
            },
            Tracker {
                key: "rivals".into(),
                note: Uuid::nil(),
                def: Ok(def("name: Rivals\nper: [Old Guard]")),
            },
        ],
        ..Rules::default()
    };
    // A nested `add: ask` asks for the amount as a top-level one does.
    assert!(def("name: Gold\nquick: [{label: Spend, changes: [{add: -ask}]}]").quick[0].asks());
    assert!(!rules.def("teams").unwrap().quick[0].asks());
    let teams = rules.def("teams").unwrap();
    assert_eq!(
        validate_definition(teams, &rules),
        Err("Score: rivals has no line for Party.".into())
    );
    // Two amount fields with one owner (tracker, line, label) are a repeat: the definition
    // loads, and the view leaves out each field after the first. Two actions with one label
    // and fixed amounts draw no field, and load as they are (as before).
    let pay_twice = "name: Purses\nquick: [{label: Pay, add: ask}, {label: Pay, add: 1}]";
    let ann_twice =
        "name: Purses\nper: [Ann, Ann]\nquick: [{label: Tip, add: 1}, {label: Pay, add: ask}]";
    for (yaml, want) in [
        (
            pay_twice,
            Some(Repeat::Label {
                label: "Pay".into(),
                keep: 0,
            }),
        ),
        (
            ann_twice,
            Some(Repeat::Line {
                word: "Ann".into(),
                asking: vec![1],
            }),
        ),
        (
            "name: Purses\nquick: [{label: Tip, add: 2}, {label: Tip, add: 1}]",
            None,
        ),
        (
            "name: Purses\nper: [Ann, Ann]\nquick: [{label: Tip, add: 1}]",
            None,
        ),
        (
            "name: Purses\nper: [Ann, Bo]\nquick: [{label: Pay, add: ask}, {label: Tip, add: 1}]",
            None,
        ),
    ] {
        assert_eq!(validate_definition(&def(yaml), &rules), Ok(()), "{yaml}");
        assert_eq!(repeat(&def(yaml)), want, "{yaml}");
    }
    assert_eq!(
        repeat(&def(pay_twice)).as_ref().map(repeat_words),
        Some("Two actions are named “Pay”, and one asks for an amount. Give each action its own name.".into())
    );
    // Of two actions with one label, a press from the second button makes its own change.
    let tips = Rules {
        notes: vec!["Rules".into()],
        trackers: vec![Tracker {
            key: "purses".into(),
            note: Uuid::nil(),
            def: Ok(def(
                "name: Purses\nquick: [{label: Tip, add: 2}, {label: Tip, add: 1}]",
            )),
        }],
        ..Rules::default()
    };
    let second = QuickRef {
        label: "Tip",
        at: Some(1),
    };
    let Ok(Op::Append(entries)) = Act::Press {
        key: "purses",
        instance: None,
        action: second,
        asked: None,
    }
    .build(
        &tips,
        &Ledger::default(),
        &Today::default(),
        None,
        crate::hermetic::new_uuid,
    ) else {
        panic!("the second Tip records");
    };
    assert_eq!(entries[0].add, Some(1));
    assert_eq!(
        Act::Press {
            key: "teams",
            instance: Some("Party"),
            action: "Score".into(),
            asked: None
        }
        .build(
            &rules,
            &Ledger::default(),
            &Today::default(),
            None,
            crate::hermetic::new_uuid
        ),
        Err(Refused::NoLine {
            tracker: "Rivals".into(),
            line: Some("Party".into())
        })
    );
    // A change that names its line with `for:` goes there, from whichever line is pressed.
    let pressed = "Score for Old Guard";
    let Ok(Op::Append(entries)) = Act::Press {
        key: "teams",
        instance: Some("Party"),
        action: pressed.into(),
        asked: None,
    }
    .build(
        &rules,
        &Ledger::default(),
        &Today::default(),
        None,
        crate::hermetic::new_uuid,
    ) else {
        panic!("the press records");
    };
    assert_eq!(entries[0].instance.as_deref(), Some("Old Guard"));
    // A day mark has no line of its own, so its change names its tracker, and `for:` names
    // a line that tracker has.
    let day_mark = |yaml: &str| {
        let mark = serde_yaml_ng::from_str::<DayMark>(yaml).unwrap();
        validate_day_mark(&mark, &rules)
    };
    assert_eq!(
        day_mark("{every: 7, say: Rest., changes: [{tracker: rivals, for: Old Guard, add: 1}]}"),
        Ok(())
    );
    for (yaml, why) in [
        (
            "{at: 3, every: 7, say: Rest.}",
            "A day mark has at or every.",
        ),
        ("{say: Rest.}", "A day mark has at or every."),
        (
            "{at: 3, from: 2, say: Rest.}",
            "from works only with every.",
        ),
        ("{every: 0, say: Rest.}", "A day mark’s day is 1 or more."),
        (
            "{at: 3, say: Rest., changes: [{add: 1}]}",
            "A day mark’s change names its tracker.",
        ),
        (
            "{at: 3, say: Rest., changes: [{tracker: rivals, add: 1}]}",
            "rivals has per, so a change to it needs a name.",
        ),
        (
            "{at: 3, say: Rest., changes: [{tracker: rivals, for: Party, add: 1}]}",
            "rivals has no line for Party.",
        ),
    ] {
        assert_eq!(day_mark(yaml), Err(why.into()), "{yaml}");
    }
    // `every: 30, from: 2` is Day 32, Day 62; with no `from` a schedule counts from Day 1.
    let month = serde_yaml_ng::from_str::<DayMark>("{every: 30, from: 2, say: x}").unwrap();
    assert_eq!(month.days_until(91), [32, 62]);
    let plain = serde_yaml_ng::from_str::<DayMark>("{every: 30, say: x}").unwrap();
    assert_eq!(plain.days_until(31), [31]);
    // `Done` whose ticked change takes a value back below a mark says that notice is no
    // longer due, as a press does.
    let rules = Rules {
        notes: vec!["Rules".into()],
        trackers: vec![Tracker {
            key: "rivals".into(),
            note: Uuid::nil(),
            def: Ok(def(
                "name: Rivals\nper: [Old Guard]\nmarks: [{at: 1, say: First.}]",
            )),
        }],
        day_marks: vec![Dated {
            key: "rest".into(),
            def: Ok(serde_yaml_ng::from_str(
                "{at: 7, say: Rest., changes: [{tracker: rivals, for: Old Guard, add: -1}]}",
            )
            .unwrap()),
            note: Uuid::nil(),
        }],
        ..Rules::default()
    };
    let change = |id: &str, add: i64| Entry {
        id: id.into(),
        tracker: Some("rivals".into()),
        instance: Some("Old Guard".into()),
        add: Some(add),
        ..Entry::default()
    };
    let done = vec![
        change("c-00000002", -1),
        Entry {
            id: "c-00000003".into(),
            notice: Some(NoticeRef::Day {
                mark: "rest".into(),
                day: 7,
            }),
            done: true,
            ..Entry::default()
        },
    ];
    let ledger = Ledger {
        changes: [vec![change("c-00000001", 1)], done.clone()].concat(),
        ..Ledger::default()
    };
    let state = replay(&rules, &ledger, &Today::Day(7));
    assert!(state.due.is_empty(), "{:?}", state.due);
    assert_eq!(
        result_words(&rules, &ledger, &state, &done),
        "Rest, Day 7 is done. First for Old Guard is no longer due."
    );
}

#[test]
fn undo_and_remove_of_a_change_that_raised_a_notice_take_the_notice_and_its_entries() {
    let mut race = race();
    let t = &mut race.trackers;
    t.press("race_clock", None, "Cistern or Sands", None);
    let bell = t.state.due[0].clone();
    let raised = t.newest().to_vec();
    // The roll is made with no session being played, so it has another stamp.
    t.played = None;
    t.roll_notice(&bell, "wander_early");
    t.played = Some(TONIGHT);
    t.test_settle();
    // A notice with a roll on it stays when the value falls below its mark: the DM acted
    // on it. Here an edit in Obsidian takes the clock back to 0, then away again.
    let path = ledger::path(&race.campaign.path);
    let back = Entry {
        id: "c-0b5e1e01".into(),
        tracker: Some("race_clock".into()),
        add: Some(-20),
        ..Entry::default()
    };
    ledger::commit(&path, |_| Ok(Op::Append(vec![back.clone()]))).unwrap();
    let t = &mut race.trackers;
    t.read_disk();
    assert_eq!(
        (
            value(&race, "race_clock", None),
            race.trackers.state.due.len()
        ),
        (0, 1),
        "the rolled bell stays"
    );
    ledger::commit(&path, |_| Ok(Op::Remove(vec![back]))).unwrap();
    let t = &mut race.trackers;
    t.read_disk();
    let target = t.newest().to_vec();
    assert_eq!(target[0], raised[0]);
    assert_eq!(target.len(), 2, "the change and the roll on its notice");
    assert_eq!(newest_label(t), "Cistern or Sands");
    t.undo(&target);
    assert!(t.ledger.changes.is_empty());
    assert_eq!(t.due(), 0);

    // An older session: two rooms, the bell they raised, its roll and its Done.
    let note = t.rules.trackers[0].note;
    t.played = Some(Uuid::from_u128(1));
    t.press("race_clock", None, "Enter room", None);
    t.press("race_clock", None, "Enter room", None);
    let bell = t.state.due[0].clone();
    t.roll_notice(&bell, "wander_early");
    t.done(&bell, &[]);
    t.played = Some(TONIGHT);
    t.test_settle();
    let rows: Vec<&str> = t.test_history()["race_clock"]
        .iter()
        .map(|row| row.words.as_str())
        .collect();
    assert_eq!(rows[..2], ["Enter room: +10", "Enter room: +10"]);
    assert!(
        rows[2].starts_with("Rolled ") && rows[2].contains(" on Wandering, 0:00 to 0:39: "),
        "{}",
        rows[2]
    );
    assert_eq!(rows[3], "Bell is done.");
    assert_eq!(t.test_history()["race_clock"][1].value, Some(20));
    // The first room is what the second room's crossing depends on: Remove of it would
    // leave the roll and the Done with no notice, so it writes nothing and names them.
    let before = file(&race);
    let t = &mut race.trackers;
    let first = t.test_history()["race_clock"][0].entry.clone();
    t.remove(note, &first);
    assert_eq!(
        t.test_history_reason(),
        Some((
            note,
            refused(
                history(
                    &label(&t.rules, &t.ledger, std::slice::from_ref(&first)),
                    false
                ),
                Refused::DependsOn("Wandering, 0:00 to 0:39 roll".into())
            )
        ))
    );
    assert_eq!(
        (file(&race), race.trackers.test_removed().clone()),
        (before, None)
    );
    // The crossing itself leaves with its notice's roll and Done, as Undo of it would.
    let t = &mut race.trackers;
    let second = t.test_history()["race_clock"][1].entry.clone();
    let kept = t.ledger.clone();
    t.remove(note, &second);
    assert_eq!(t.test_history_reason(), None);
    assert_eq!(
        t.test_removed().as_ref().map(|r| r.words.as_str()),
        Some(
            "Removed Enter room: Elapsed race time +10. With it: Wandering, 0:00 to 0:39 roll; Bell done."
        )
    );
    assert_eq!(t.ledger.changes, kept.changes[..1]);
    assert_eq!(
        (value(&race, "race_clock", None), race.trackers.due()),
        (10, 0)
    );
    // A notice raised since needs the removal: its bell crosses 20 only while the room is
    // out. Undo remove would leave its roll with no notice, so it writes nothing.
    let t = &mut race.trackers;
    t.press("race_clock", None, "Cistern or Sands", None);
    let bell = t.state.due[0].clone();
    t.roll_notice(&bell, "wander_early");
    let before = file(&race);
    let t = &mut race.trackers;
    t.test_undo_remove();
    assert_eq!(
        t.test_history_reason(),
        Some((
            note,
            refused(
                history(&t.test_removed_name(), true),
                Refused::DependsOn("Wandering, 0:00 to 0:39 roll".into())
            )
        ))
    );
    assert!(
        t.test_removed().is_some(),
        "the Undo stays for when that roll is gone"
    );
    assert_eq!(file(&race), before);

    // A notice with a roll on it stays, whatever puts a fall before the roll: a Remove, an
    // Undo, an `Undo remove` or an edit in Obsidian. Rooms of an older session: A to 20
    // raises the bell, B to 30, C back to 20, then a roll on A's bell.
    let path = ledger::path(&race.campaign.path);
    let older = Some(Uuid::from_u128(1));
    let room = |id: &str, add: i64, session| Entry {
        id: id.into(),
        tracker: Some("race_clock".into()),
        add: Some(add),
        session,
        ..Entry::default()
    };
    let rolled = |id: &str, change: &str, at: i64, table: &str| Entry {
        id: id.into(),
        notice: Some(NoticeRef::Value {
            change: change.into(),
            at,
        }),
        roll: Some(Roll::Table(TableRoll {
            table: table.into(),
            name: None,
            value: 3,
            row: "Nothing comes.".into(),
        })),
        session: older,
        ..Entry::default()
    };
    let write = |entries: Vec<Entry>| {
        fs::remove_file(&path).ok();
        ledger::commit(&path, |_| Ok(Op::Append(entries))).unwrap();
        fs::read_to_string(&path).unwrap()
    };
    let bell_with_roll = |t: &Trackers| {
        t.state
            .due
            .iter()
            .any(|n| n.id.change() == Some("c-0000000a") && n.rolls.len() == 1)
    };
    let rooms = |b_session| {
        vec![
            room("c-0000000a", 20, older),
            room("c-0000000b", 10, b_session),
            room("c-0000000c", -10, older),
            rolled("c-0000000d", "c-0000000a", 20, "wander_early"),
        ]
    };
    write(rooms(older));
    let t = &mut race.trackers;
    t.read_disk();
    let b = t.test_history()["race_clock"]
        .iter()
        .find(|row| row.entry.id == "c-0000000b")
        .unwrap()
        .entry
        .clone();
    t.remove(note, &b);
    assert_eq!(t.test_history_reason(), None);
    assert_eq!(value(&race, "race_clock", None), 10);
    assert!(
        bell_with_roll(&race.trackers),
        "Remove of B keeps the bell and its roll"
    );
    write(rooms(Some(TONIGHT)));
    let t = &mut race.trackers;
    t.read_disk();
    let newest = t.newest().to_vec();
    assert_eq!(newest.len(), 1, "B is tonight's newest act");
    t.undo(&newest);
    assert_eq!(t.reason, None);
    assert!(
        bell_with_roll(&race.trackers),
        "Undo of B keeps the bell and its roll"
    );
    // A correction that took the clock back to 0 is removed, the bell is rolled, and then
    // `Undo remove` puts the correction back before the roll: the bell stays, with its roll,
    // also as the saved file rebuilds it.
    write(vec![
        room("c-0000000a", 20, older),
        room("c-0000000e", -20, older),
    ]);
    let t = &mut race.trackers;
    t.read_disk();
    let correction = t.test_history()["race_clock"][1].entry.clone();
    t.remove(note, &correction);
    let bell = t.state.due[0].clone();
    t.roll_notice(&bell, "wander_early");
    t.test_undo_remove();
    assert_eq!(t.test_history_reason(), None);
    assert_eq!(value(&race, "race_clock", None), 0);
    assert!(
        bell_with_roll(&race.trackers),
        "Undo remove keeps the bell and its roll"
    );
    let t = &race.trackers;
    let saved = ledger::parse_bytes(Some(&fs::read(&path).unwrap())).unwrap();
    assert_eq!(replay(&t.rules, &saved, t.test_today()).due, t.state.due);
    // An Undo that would leave a roll with no notice writes nothing and names it: D crosses
    // 40 only because of tonight's B.
    let before = write(vec![
        room("c-0000000a", 20, older),
        room("c-0000000b", 10, Some(TONIGHT)),
        room("c-0000000f", 10, older),
        rolled("c-00000010", "c-0000000f", 40, "wander_middle"),
    ]);
    let t = &mut race.trackers;
    t.read_disk();
    let newest = t.newest().to_vec();
    t.undo(&newest);
    assert_eq!(
        t.reason,
        Some(refused(
            undone(&label(&t.rules, &t.ledger, &newest)),
            Refused::DependsOn("Wandering, 0:40 to 1:19 roll".into())
        ))
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), before);

    // A value that an edit in Obsidian forced below its `min` counts as written, and joins
    // the problems list of its note; Splat changes nothing.
    let path = ledger::path(&race.campaign.path);
    let forced =
        "---\ntrackers_version: 1\nchanges:\n- id: c-0b51de01\n  tracker: favor\n  add: -2\n---\n";
    fs::write(&path, forced).unwrap();
    let t = &mut race.trackers;
    t.read_disk();
    assert_eq!(value(&race, "favor", None), -2);
    assert!(
        race.trackers
            .test_problems()
            .contains(&(Some(note), "Crowd Favor is -2, below its min of 0.".into()))
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), forced);
}

#[test]
fn each_write_path_of_the_ledger_holds() {
    let mut race = race();
    let path = ledger::path(&race.campaign.path);
    // An edit on disk a moment before: the act is applied to the file as it is now.
    fs::write(
        &path,
        "---\ntrackers_version: 1\nchanges:\n- id: c-0b51de00\n  tracker: favor\n  add: 2\n  why: Typed in Obsidian\n---\n",
    )
    .unwrap();
    let t = &mut race.trackers;
    t.press("favor", None, "Throw a gift", None);
    assert_eq!(t.reason, None);
    assert_eq!(t.ledger.changes[0].id, "c-0b51de00");
    assert_eq!(value(&race, "favor", None), 1);

    // A, then an unrelated B with another stamp, then Undo of A: B's change stays.
    let t = &mut race.trackers;
    t.press("favor", None, "Crowd loves it", None);
    let a = t.newest().to_vec();
    // Each session's newest act keeps the order its act issued: switching sessions records
    // nothing, and a second act gets a newer order.
    let order = |t: &Trackers| t.test_kept().map(|(order, _)| order);
    let a_order = order(t).expect("act A is kept");
    t.played = Some(Uuid::from_u128(2));
    t.press("souls", None, "Crowd loves it", None);
    assert!(t.reason.is_some(), "souls has no such action");
    t.press("race_clock", None, "Enter room", None);
    let b_order = order(t).expect("act B is kept");
    assert!(b_order > a_order, "a second act gets a newer order");
    t.played = Some(TONIGHT);
    t.test_settle();
    assert_eq!(t.newest(), a.as_slice());
    assert_eq!(
        order(t),
        Some(a_order),
        "back in A's session, A keeps its order"
    );
    t.played = Some(Uuid::from_u128(2));
    t.test_settle();
    assert_eq!(
        order(t),
        Some(b_order),
        "back in B's session, B keeps its order"
    );
    t.played = Some(TONIGHT);
    t.test_settle();
    // A campaign switch hides the kept acts; switching back finds A with its order.
    let index = Some(race.index.clone());
    poll(
        &mut race.trackers,
        None,
        None,
        Some(TONIGHT),
        &race.timeline,
    );
    assert!(!race.trackers.test_has_played_owner());
    poll(
        &mut race.trackers,
        index,
        Some(&race.campaign),
        Some(TONIGHT),
        &race.timeline,
    );
    let t = &mut race.trackers;
    t.read_disk();
    assert_eq!(t.newest(), a.as_slice());
    assert_eq!(
        order(t),
        Some(a_order),
        "after a campaign switch, A keeps its order"
    );
    t.undo(&a);
    assert_eq!(value(&race, "favor", None), 1);
    assert_eq!(value(&race, "race_clock", None), 10, "B’s change stays");

    // `Remove` on a row with another stamp, in the reader of its note. The entry it was
    // drawn from must be on disk as it was read; its words stay in that reader, not on the
    // view's result line.
    let t = &mut race.trackers;
    let note = t.rules.trackers[0].note;
    let older = t.test_history()["favor"][0].entry.clone();
    assert_eq!(older.id, "c-0b51de00");
    let text = fs::read_to_string(&path).unwrap();
    let edited = text.replace("why: Typed in Obsidian", "why: Typed again");
    fs::write(&path, &edited).unwrap();
    t.remove(note, &older);
    assert_eq!(
        t.test_history_reason(),
        Some((
            note,
            refused(
                history(
                    &label(&t.rules, &t.ledger, std::slice::from_ref(&older)),
                    false
                ),
                Refused::Changed(Part::Entry)
            )
        ))
    );
    assert_eq!(t.reason, None);
    assert_eq!(fs::read_to_string(&path).unwrap(), edited);
    fs::write(&path, &text).unwrap();
    // Remove A, an unrelated act B, then Undo of A: A goes back where it was, and B stays.
    let t = &mut race.trackers;
    t.read_disk();
    t.remove(note, &older);
    assert_eq!(
        t.test_removed().as_ref().map(|r| r.words.as_str()),
        Some("Removed Typed in Obsidian: Crowd Favor +2.")
    );
    assert_eq!(value(&race, "favor", None), -1, "Splat clamps nothing");
    let t = &mut race.trackers;
    assert!(
        t.test_problems()
            .contains(&(Some(note), "Crowd Favor is -1, below its min of 0.".into()))
    );
    t.press("race_clock", None, "Enter room", None);
    let b = t.newest().to_vec();
    t.test_undo_remove();
    assert_eq!(t.test_removed(), None);
    assert_eq!(t.ledger.changes[0], older);
    assert_eq!(t.ledger.changes.last(), b.last());
    assert_eq!(value(&race, "favor", None), 1);
    let t = &mut race.trackers;
    t.undo(&b);
    // An Undo whose entry changed on disk puts nothing back, says so, and ends.
    t.remove(note, &older);
    ledger::commit(&path, |_| {
        Ok(Op::Append(vec![Entry {
            add: Some(5),
            ..older.clone()
        }]))
    })
    .unwrap();
    let outside = fs::read_to_string(&path).unwrap();
    let t = &mut race.trackers;
    t.read_disk();
    let removed_name = t.test_removed_name();
    t.test_undo_remove();
    assert_eq!(
        t.test_history_reason(),
        Some((
            note,
            refused(history(&removed_name, true), Refused::Changed(Part::Entry))
        ))
    );
    assert_eq!(
        (
            t.test_removed().is_none(),
            fs::read_to_string(&path).unwrap()
        ),
        (true, outside)
    );
    fs::write(&path, &text).unwrap();
    race.trackers.read_disk();
    assert_eq!(value(&race, "favor", None), 1);
    // M0566 (SH7, AH7): a Remove that takes a notice off the Due list says so in its result,
    // as an act's fall does; a Remove that ends none says nothing more.
    let t = &mut race.trackers;
    t.press("seals", Some("Old Guard"), "Take seal", None);
    let due = t.due();
    let seal = t.test_history()["seals"]
        .iter()
        .find(|row| row.entry.instance.as_deref() == Some("Old Guard"))
        .unwrap()
        .entry
        .clone();
    t.remove(note, &seal);
    let words = t.test_removed().map(|removed| removed.words).unwrap();
    assert!(
        words.starts_with("Removed ")
            && words.ends_with(". First seal for Old Guard is no longer due."),
        "{words}"
    );
    assert_eq!(t.due(), due - 1);
    // Review of PR 703: of two seals, Remove of the first leaves the first seal due, now
    // raised by the second, and says nothing of it.
    let t = &mut race.trackers;
    t.press("seals", Some("Old Guard"), "Take seal", None);
    t.press("seals", Some("Old Guard"), "Take seal", None);
    let first = t.test_history()["seals"]
        .iter()
        .find(|row| row.entry.instance.as_deref() == Some("Old Guard"))
        .unwrap()
        .entry
        .clone();
    t.remove(note, &first);
    let words = t.test_removed().map(|removed| removed.words).unwrap();
    assert!(
        !words.contains("First seal for Old Guard is no longer due"),
        "{words}"
    );
    assert_eq!(value(&race, "seals", Some("Old Guard")), 1);
    let t = &mut race.trackers;
    assert!(
        t.state
            .due
            .iter()
            .any(|notice| notice.name(&t.rules) == "First seal for Old Guard"),
        "the first seal stays due"
    );
    let rest: Vec<_> = t.test_history()["seals"]
        .iter()
        .filter(|row| row.entry.instance.as_deref() == Some("Old Guard"))
        .map(|row| row.entry.clone())
        .collect();
    for entry in rest {
        race.trackers.remove(note, &entry);
    }
    // An Undo whose place changed on disk: the entry before the removed one was edited in
    // Obsidian. It puts nothing back, says so, and ends.
    let t = &mut race.trackers;
    let clock = t.test_history()["race_clock"][0].entry.clone();
    t.remove(note, &clock);
    assert!(t.test_removed().is_some());
    let edited = fs::read_to_string(&path)
        .unwrap()
        .replace("row: A healing potion.", "row: A healing potion, edited.");
    fs::write(&path, &edited).unwrap();
    t.read_disk();
    let removed_name = t.test_removed_name();
    t.test_undo_remove();
    assert_eq!(
        t.test_history_reason(),
        Some((
            note,
            refused(
                history(&removed_name, true),
                Refused::Changed(Part::EntryBefore)
            )
        ))
    );
    assert_eq!(
        (
            t.test_removed().is_none(),
            fs::read_to_string(&path).unwrap()
        ),
        (true, edited)
    );
    fs::write(&path, &text).unwrap();
    race.trackers.read_disk();

    // A failed save writes nothing and keeps nothing: the file stays byte for byte, and the
    // value and the result line stay as the file gives them. The result line says why.
    let entries = race.trackers.ledger.changes.len();
    let before = fs::read(&path).unwrap();
    let shown = race.trackers.test_result().clone();
    let mut readonly = fs::metadata(&race.campaign.path).unwrap().permissions();
    use std::os::unix::fs::PermissionsExt;
    readonly.set_mode(0o555);
    fs::set_permissions(&race.campaign.path, readonly.clone()).unwrap();
    let t = &mut race.trackers;
    t.press("favor", None, "Crowd loves it", None);
    assert_eq!(fs::read(&path).unwrap(), before);
    let Some(Unrecorded::NotSaved(error)) = &t.reason else {
        panic!("the result line says the save failed: {:?}", t.reason);
    };
    let words = view::not_saved(error).to_string();
    assert!(
        words.starts_with("Trackers.md was not saved: ")
            && words.ends_with(". Nothing was recorded."),
        "{words}"
    );
    assert_eq!(t.test_result(), shown);
    assert_eq!(value(&race, "favor", None), 1);
    // A failed Undo keeps the act's entry and its order.
    let t = &mut race.trackers;
    let kept = t.test_kept();
    assert!(kept.is_some(), "the newest act is kept");
    let newest = t.newest().to_vec();
    t.undo(&newest);
    assert!(matches!(t.reason, Some(Unrecorded::NotSaved(_))));
    assert_eq!(
        t.test_kept(),
        kept,
        "a failed Undo keeps the entry and its order"
    );
    // A `Remove` whose save fails records nothing, and nothing waits.
    let t = &mut race.trackers;
    t.remove(note, &older);
    assert!(matches!(
        t.test_history_reason(),
        Some((_, Unrecorded::NotSaved(_)))
    ));
    assert_eq!(
        (t.test_removed().clone(), fs::read(&path).unwrap()),
        (None, before.clone())
    );
    // An edit in Obsidian a moment before a failed save: the view shows what the file holds.
    let outside = String::from_utf8(before.clone()).unwrap().replace(
        "  add: 2\n  why: Typed in Obsidian",
        "  add: 8\n  why: Typed in Obsidian",
    );
    fs::write(&path, &outside).unwrap();
    let t = &mut race.trackers;
    t.press("favor", None, "Crowd loves it", None);
    assert_eq!(fs::read_to_string(&path).unwrap(), outside);
    assert!(matches!(t.reason, Some(Unrecorded::NotSaved(_))));
    assert_eq!(value(&race, "favor", None), 7);
    fs::write(&path, &before).unwrap();
    race.trackers.read_disk();
    // A number typed for `add: ask` stays in its field when the save fails.
    let note = race.campaign.path.join("Tournament.md");
    let text = fs::read_to_string(&note).unwrap();
    let with_ask = text.replace(
        "label: Crowd loves it\n>       add: 1\n",
        "label: Crowd loves it\n>       add: 1\n>     - label: Bet\n>       add: ask\n",
    );
    assert_ne!(with_ask, text);
    fs::write(&note, with_ask).unwrap();
    let index = Arc::new(ContentIndex::load(&[], Some(&race.campaign)).unwrap());
    let t = &mut race.trackers;
    poll(
        t,
        Some(index),
        Some(&race.campaign),
        Some(TONIGHT),
        &race.timeline,
    );
    t.press("favor", None, "Bet", Some(3));
    assert!(matches!(t.reason, Some(Unrecorded::NotSaved(_))));
    assert_eq!(t.test_typed(), Some("3"));
    assert_eq!(fs::read(&path).unwrap(), before);
    // The next press is one more act, and writes exactly one entry.
    readonly.set_mode(0o755);
    fs::set_permissions(&race.campaign.path, readonly.clone()).unwrap();
    let t = &mut race.trackers;
    t.press("favor", None, "Crowd loves it", None);
    assert_eq!(t.reason, None);
    assert_eq!(t.ledger.changes.len(), entries + 1);
    assert_eq!(value(&race, "favor", None), 2);

    // `Add to Timeline` whose Timeline save fails says the Timeline's own words, not
    // success. The event waits in the Timeline's draft, and saves later.
    let mut app = crate::Splat {
        trackers: std::mem::take(&mut race.trackers),
        ..Default::default()
    };
    app.timeline.open(Some(&race.campaign.path));
    let entry = app.trackers.newest()[0].id.clone();
    app.trackers.wants_timeline = Some((entry, "Crowd loves it".into()));
    readonly.set_mode(0o555);
    fs::set_permissions(&race.campaign.path, readonly.clone()).unwrap();
    app.add_tracker_act_to_timeline();
    app.timeline.save();
    readonly.set_mode(0o755);
    fs::set_permissions(&race.campaign.path, readonly.clone()).unwrap();
    // The shell's next frame.
    app.add_tracker_act_to_timeline();
    let words = app
        .trackers
        .added
        .as_ref()
        .and_then(|added| added.not_saved.clone());
    assert!(
        words
            .as_deref()
            .is_some_and(|words| words.starts_with("The timeline was not saved.")),
        "{words:?}"
    );
    app.timeline.save();
    app.add_tracker_act_to_timeline();
    assert_eq!(
        app.trackers
            .added
            .as_ref()
            .map(|added| added.not_saved.clone()),
        Some(None)
    );
    race.trackers = std::mem::take(&mut app.trackers);

    // A restart: the values, the result line and its Undo come from the file.
    let mut again = Trackers::with_ids(crate::hermetic::new_uuid);
    poll(
        &mut again,
        Some(race.index.clone()),
        Some(&race.campaign),
        Some(TONIGHT),
        &race.timeline,
    );
    assert_eq!(again.state.value(&again.rules, "favor", None), 2);
    assert_eq!(newest_label(&again), "Crowd loves it");
    assert_eq!(
        again
            .test_result()
            .as_ref()
            .map(|(label, _)| label.as_str()),
        Some("Crowd loves it")
    );
    let newest = again.newest().to_vec();
    again.undo(&newest);
    assert_eq!(again.state.value(&again.rules, "favor", None), 1);
    // The other Trackers still holds the old Undo, and the file no longer has it as the
    // newest act of tonight: it writes nothing and says so.
    let before = fs::read(&path).unwrap();
    let stale = race.trackers.newest().to_vec();
    race.trackers.undo(&stale);
    assert_eq!(
        race.trackers.reason,
        Some(refused(
            undone(&label(&race.trackers.rules, &race.trackers.ledger, &stale)),
            Refused::LedgerChanged
        ))
    );
    assert_eq!(fs::read(&path).unwrap(), before);

    // `Campaign.md` that cannot be read is no empty list of rules: the act writes nothing,
    // no value shows, and the words name the file.
    let readable = fs::read(&path).unwrap();
    assert_eq!(value(&race, "favor", None), 1);
    let manifest = race.campaign.path.join("Campaign.md");
    fs::set_permissions(&manifest, fs::Permissions::from_mode(0o000)).unwrap();
    let t = &mut race.trackers;
    t.press("favor", None, "Crowd loves it", None);
    fs::set_permissions(&manifest, fs::Permissions::from_mode(0o644)).unwrap();
    assert_eq!(fs::read(&path).unwrap(), readable);
    assert_eq!(
        t.reason,
        Some(refused(
            recorded("Crowd loves it"),
            Refused::RulesUnreadable(RulesFile::Manifest)
        ))
    );
    // PR 661 review (P2): the refusal names the file that cannot be read, and sends the DM
    // there, not to the house rule notes.
    assert_eq!(
        refused_words(&t.reason).as_deref(),
        Some(
            "Nothing was recorded for Crowd loves it. Splat cannot read Campaign.md. Check \
             Campaign.md first."
        ),
        "m-s0-5-manifest"
    );
    // The reader's words stay with the notes' own fault (`TrackerFault::NotesUnreadable`).
    let why = t.rules.unreadable.clone().unwrap_or_default();
    assert!(why.starts_with("Campaign.md cannot be read: "), "{why}");
    assert!(t.in_play() && t.rules.unreadable.is_some());
    assert_eq!((t.due(), value(&race, "favor", None)), (0, 0));

    // A file that cannot be read shows nothing old, and is never written over.
    fs::write(&path, "---\nchanges: [\n---\n").unwrap();
    let t = &mut race.trackers;
    t.read_disk();
    assert!(t.unreadable.is_some());
    assert_eq!(t.due(), 0);
    assert_eq!(t.state.value(&t.rules, "favor", None), 0);
    t.press("favor", None, "Crowd loves it", None);
    assert_eq!(fs::read_to_string(&path).unwrap(), "---\nchanges: [\n---\n");
}

#[test]
fn an_open_history_prepares_its_rows_once_and_draws_only_those_in_view() {
    use crate::design::test_harness::Themed as _;
    use crate::test_support::{CachedWork, Counts};
    use egui_kittest::{Harness, kittest::Queryable};
    let mut race = race();
    let note = race
        .index
        .items
        .iter()
        .find(|item| item.entry.name == "Tournament")
        .unwrap()
        .entry
        .clone();
    let mut t = std::mem::take(&mut race.trackers);
    t.ledger.changes = (0..5000)
        .map(|n| Entry {
            id: format!("c-{n:08x}"),
            tracker: Some("favor".into()),
            add: Some(1),
            why: Some("Crowd loves it".into()),
            ..Entry::default()
        })
        .collect();
    t.test_settle();
    let mut h = Harness::builder()
        .with_size(eframe::egui::vec2(346.0, 640.0))
        .themed_ui_state(
            move |ui, t: &mut Trackers| {
                crate::design::ScrollColumn::new("tracker history")
                    .show(ui, |ui| history::show(ui, t, &note));
            },
            t,
        );
    let counts = |_: &mut Harness<'_, Trackers>| {
        let (reached, _) = history::DRAWN.with(std::cell::Cell::get);
        Counts {
            reached,
            work: history::PREPARED.with(std::cell::Cell::get),
        }
    };
    let drawn = || history::DRAWN.with(std::cell::Cell::get).1;
    // Folded, the section draws no row.
    h.run_steps(3);
    let folded = drawn();
    h.run_steps(3);
    assert_eq!(drawn(), folded, "a folded history draws no row");
    let work = CachedWork::start("Open history rows", &mut h, counts);
    h.get_by_label("CROWD FAVOR HISTORY 5,000").click();
    h.run_steps(3);
    let shown = h.query_all_by_label("Crowd loves it: +1").next().is_some();
    work.warm(&mut h, shown, 0)
        .expect("the open history shows its rows");
    // Unchanged frames and a resize prepare no row again, and each frame draws only the
    // rows in view: at 640 px, some 20 rows of 5,000.
    let before = drawn();
    work.steady(&mut h, |h| {
        h.run_steps(3);
        h.set_size(eframe::egui::vec2(400.0, 640.0));
        h.run_steps(2);
        h.query_all_by_label("Crowd loves it: +1").next().is_some()
    })
    .expect("unchanged frames prepare no row");
    let per_frame = (drawn() - before) / 5;
    assert!(
        (1..=30).contains(&per_frame),
        "an open history of 5,000 rows drew {per_frame} rows a frame"
    );
    // A change of the ledger prepares the rows again, once.
    work.refresh(&mut h, 5001, |h| {
        let t = h.state_mut();
        t.ledger.changes.push(Entry {
            id: "c-ffffffff".into(),
            tracker: Some("favor".into()),
            add: Some(1),
            why: Some("Crowd loves it".into()),
            ..Entry::default()
        });
        t.test_settle();
        h.run_steps(2);
        h.query_by_label("CROWD FAVOR HISTORY 5,001").is_some()
    })
    .expect("a changed ledger prepares its rows again");
    // M0571: unchanged frames fit no row's words again; a row too long for its room is cut at
    // a whole word and still says its words whole to assistive technology.
    let fitted = || history::FITTED.with(std::cell::Cell::get);
    h.run_steps(2);
    let before = fitted();
    h.run_steps(3);
    assert_eq!(
        fitted(),
        before,
        "unchanged frames fit no row's words again"
    );
    let long = "Crowd loves it when the champion bows to every banner in the hall";
    let t = h.state_mut();
    // The first row of the history, in view at its top.
    t.ledger.changes.insert(
        0,
        Entry {
            id: "c-fffffffe".into(),
            tracker: Some("favor".into()),
            add: Some(1),
            why: Some(long.into()),
            ..Entry::default()
        },
    );
    t.test_settle();
    h.run_steps(2);
    assert!(fitted() > before, "a settle fits the rows in view again");
    // A label's painted words are its value; its accessible label is the whole row.
    let whole = format!("{long}: +1");
    let cut_row = h.query_all(egui_kittest::kittest::By::new().predicate(|node| {
        node.role() == eframe::egui::accesskit::Role::Label
            && node.label().is_some_and(|label| label == whole)
            && node.value().is_some_and(|shown| {
                shown.ends_with('…')
                    && whole.starts_with(&format!("{} ", shown.trim_end_matches('…')))
            })
    }));
    assert_eq!(
        cut_row.count(),
        1,
        "the row is cut at a whole word and says its words whole"
    );
}

/// FE1, AE2: every tracker line of the panel shares one value column and one first-action
/// edge, and a value that gains a digit moves no action. The columns are measured once for
/// each change of the trackers, never on an unchanged frame.
#[test]
fn the_panel_shares_its_columns_and_measures_them_once_per_change() {
    use crate::design::test_harness::Themed as _;
    use egui_kittest::{Harness, kittest::Queryable};
    let mut race = race();
    let t = std::mem::take(&mut race.trackers);
    let mut h = Harness::builder()
        // Wide enough that every line's first two actions fit after the values (round 30
        // `tracker-action-wrap`): the layout in which a wider value could push the actions.
        .with_size(eframe::egui::vec2(640.0, 640.0))
        .themed_ui_state(|ui, t: &mut Trackers| view::show(ui, t, false), t);
    h.run_steps(4);
    let left = |h: &Harness<'_, Trackers>, name: &str| h.get_by_label(name).rect().left();
    let firsts = |h: &Harness<'_, Trackers>| {
        [
            "Enter room",
            "Crowd loves it",
            "Take seal: Party",
            "Take seal: Keyring",
        ]
        .map(|name| left(h, name))
    };
    let before = firsts(&h);
    assert!(
        before.iter().all(|x| (x - before[0]).abs() < 0.5),
        "the first actions of three trackers start at one edge: {before:?}"
    );
    // Round 30 review of PR 400 (F1): at this width every pair fits, so the actions stand
    // BESIDE the values, past the name and value columns, and a pair shares its row; an
    // implementation that always puts them under the names fails here.
    let race_value = h.get_by_label("Cistern or Sands").rect();
    assert!(
        before[0] > 120.0
            && (race_value.top() - h.get_by_label("Enter room").rect().top()).abs() < 0.5,
        "the actions stand beside the values: first actions at {before:?}, Cistern or Sands at {race_value:?}"
    );
    let measured = h.state().test_measured();
    let lines = Trackers::test_trackers_laid();
    h.run_steps(5);
    assert_eq!(
        h.state().test_measured(),
        measured,
        "an unchanged frame measures no column"
    );
    // Layout 11c scope C: the lines are laid out before paint (`design::ActionLine`) and kept
    // with their parts: an unchanged frame lays out no tracker, from the host's entry through
    // paint.
    assert_eq!(
        Trackers::test_trackers_laid(),
        lines,
        "an unchanged frame lays out no tracker line"
    );
    // 0 / 120 becomes 10 / 120, and Crowd Favor 0 becomes 1: measured again, no move.
    h.state_mut().press("race_clock", None, "Enter room", None);
    h.state_mut().press("favor", None, "Crowd loves it", None);
    h.run_steps(4);
    let t = h.state();
    assert_eq!(t.state.value(&t.rules, "race_clock", None), 10);
    let changed = h.state().test_measured();
    assert!(
        changed > measured,
        "a change of the values measures the columns again"
    );
    // Crowd Favor 0 becomes 1, a value of the same length: its tracker is laid out again.
    assert!(
        Trackers::test_trackers_laid() > lines,
        "a change of a value lays the lines out again"
    );
    h.run_steps(5);
    assert_eq!(
        h.state().test_measured(),
        changed,
        "unchanged frames after the change measure no column"
    );
    assert_eq!(
        firsts(&h),
        before,
        "a value with one more digit moves no action"
    );
    // 999 / 120, then `Fight minute` makes 1000 / 120, wider than the slot of three
    // digits; Undo takes it back to 999. The value column keeps the room it needed, so the
    // actions do not move back and forth.
    let typed = Entry {
        id: "c-00000999".into(),
        tracker: Some("race_clock".into()),
        set: Some(999),
        why: Some("Typed in Obsidian".into()),
        ..Entry::default()
    };
    ledger::commit(&ledger::path(&race.campaign.path), |_| {
        Ok(Op::Append(vec![typed]))
    })
    .unwrap();
    h.state_mut().read_disk();
    h.run_steps(4);
    h.state_mut()
        .press("race_clock", None, "Fight minute", None);
    h.run_steps(4);
    let t = h.state();
    assert_eq!(t.state.value(&t.rules, "race_clock", None), 1000);
    let wide = firsts(&h);
    let pressed = h.state().newest().to_vec();
    h.state_mut().undo(&pressed);
    h.run_steps(4);
    let t = h.state();
    assert_eq!(t.state.value(&t.rules, "race_clock", None), 999);
    assert_eq!(
        firsts(&h),
        wide,
        "Undo of 1000 / 120 keeps the value column, and no action moves back"
    );
    // Layout 11c scope C: the Due rows are laid out before paint (`design::NoticeRow`) and kept
    // by the trackers' revision and the panel's measure. Taking the Party's first seal brings a
    // notice due: the rows are laid again and the new row draws; unchanged frames lay none.
    let laid = crate::design::notices_laid;
    h.state_mut()
        .press("seals", Some("Party"), "Take seal", None);
    h.run_steps(4);
    let done = "Done: First seal for Party";
    assert!(
        h.query_by_label(done).is_some(),
        "the new Due row draws its Done"
    );
    let made = Trackers::test_due_parts_made;
    let (after, parts) = (laid(), made());
    h.run_steps(5);
    assert!(
        h.query_by_label(done).is_some() && laid() == after && made() == parts,
        "an unchanged frame with Due rows lays none out and makes no parts, from the host's \
         entry through paint: laid {after} then {}, parts {parts} then {}",
        laid(),
        made()
    );
    // Astra's P2 on PR 885: a change of content at the same length. The mark's words change in
    // the note ("First" to "Final"); the next press reads the note (and records nothing), and
    // the row shows the new words.
    let note = race.campaign.path.join("Tournament.md");
    let text = fs::read_to_string(&note).unwrap();
    assert!(text.contains("say: First seal."));
    fs::write(&note, text.replace("say: First seal.", "say: Final seal.")).unwrap();
    h.state_mut().press("favor", None, "Crowd loves it", None);
    h.run_steps(4);
    let painted = |h: &Harness<'_, Trackers>, words: &str| {
        // A label's words are its accessible value.
        h.query_all(
            egui_kittest::kittest::By::new()
                .predicate(|node| node.value().is_some_and(|value| value.contains(words))),
        )
        .count()
    };
    assert!(
        painted(&h, "Party: Final seal.") > 0 && painted(&h, "Party: First seal.") == 0,
        "a same-length change of a Due row's words shows the new words"
    );
    // A new font atlas, and the other theme, each lay the rows again (the galleys hold the
    // atlas's places and their colours).
    let before = laid();
    let alpha = h.ctx.style().visuals.text_alpha_from_coverage;
    h.ctx.all_styles_mut(|style| {
        style.visuals.text_alpha_from_coverage = eframe::egui::epaint::AlphaFromCoverage::Linear;
    });
    h.step();
    h.ctx
        .all_styles_mut(|style| style.visuals.text_alpha_from_coverage = alpha);
    h.run_steps(3);
    assert!(laid() > before, "a new font atlas lays the Due rows again");
    let before = laid();
    let visuals = h.ctx.style().visuals.clone();
    // Light colours with the same text alpha, so the atlas stays: only the theme changes.
    let mut light = eframe::egui::Visuals::light();
    light.text_alpha_from_coverage = visuals.text_alpha_from_coverage;
    h.ctx.set_visuals(light);
    h.run_steps(3);
    assert!(laid() > before, "the other theme lays the Due rows again");
    h.ctx.set_visuals(visuals);
    h.run_steps(3);
    let after = laid();
    // A change of the width is a new measure: the rows are laid again.
    h.set_size(eframe::egui::vec2(520.0, 640.0));
    h.run_steps(4);
    assert!(laid() > after, "a new width lays the Due rows again");
}

/// Astra's P2 on PR 897: a Due row's check box is keyed by its change's own identity, in the
/// real host. The wagon's mark proposes two changes; the second check box has the keyboard; the
/// first change leaves the mark in the note (read again as the shell reads it); the check box
/// that stays keeps its id and its focus. Red: a key by the change's place.
#[test]
fn a_due_check_keeps_its_focus_when_the_change_before_it_goes() {
    use crate::design::test_harness::Themed as _;
    use crate::timeline::Kind;
    use egui_kittest::{Harness, kittest::Queryable};
    const TWO: &str = "> day_marks:\n>   wagon:\n>     name: Supply wagon\n>     at: 3\n>     say: The supply wagon arrives.\n>     changes:\n>     - {tracker: favor, add: 1}\n>     - {tracker: seals, for: Party, add: 1}\n";
    let mut race = race();
    let note = race.campaign.path.join("Tournament.md");
    let text = fs::read_to_string(&note).unwrap();
    fs::write(
        &note,
        text.replacen("> trackers:\n", &format!("{TWO}> trackers:\n"), 1),
    )
    .unwrap();
    race.index = Arc::new(ContentIndex::load(&[], Some(&race.campaign)).unwrap());
    poll(
        &mut race.trackers,
        Some(race.index.clone()),
        Some(&race.campaign),
        Some(TONIGHT),
        &race.timeline,
    );
    add_event(&mut race, Kind::Day, "");
    add_event(&mut race, Kind::Travel, "2 days");
    assert_eq!(race.trackers.due(), 1, "the wagon is due");
    let t = std::mem::take(&mut race.trackers);
    let mut h = Harness::builder()
        .with_size(eframe::egui::vec2(640.0, 900.0))
        .themed_ui_state(|ui, t: &mut Trackers| view::show(ui, t, false), t);
    h.run_steps(4);
    // Astra's P2 on PR 897: the whole host, from its entry through paint, on unchanged frames
    // with a Due row and the lines (and no footer line: the wagon came due with no act): no
    // action lays its own words (each paints the words laid with its parts) and no text is
    // measured.
    let words = || crate::design::components::ACTION_WORDS_LAID.with(std::cell::Cell::get);
    let measures = || crate::design::components::TEXT_MEASURES.with(std::cell::Cell::get);
    let before = (words(), measures());
    h.run_steps(5);
    assert!(
        h.query_by_label("Seals (Party) +1").is_some()
            && h.query_by_label("Enter room").is_some(),
        "quiet-host-reached: the Due row and the lines drew"
    );
    assert_eq!(
        (words(), measures()),
        before,
        "quiet-host: an unchanged frame with a Due row and the lines lays no action's words and \
         measures no text"
    );
    const SECOND: &str = "Seals (Party) +1";
    h.get_by_label(SECOND).focus();
    h.run_steps(2);
    let focused = h.ctx.memory(|memory| memory.focused());
    assert!(focused.is_some(), "the second check box has the keyboard");
    // The first change leaves the mark, and the rules are read again.
    let text = fs::read_to_string(&note).unwrap();
    fs::write(
        &note,
        text.replacen(">     - {tracker: favor, add: 1}\n", "", 1),
    )
    .unwrap();
    race.index = Arc::new(ContentIndex::load(&[], Some(&race.campaign)).unwrap());
    poll(
        h.state_mut(),
        Some(race.index.clone()),
        Some(&race.campaign),
        Some(TONIGHT),
        &race.timeline,
    );
    h.run_steps(4);
    assert!(
        h.query_by_label("Crowd Favor +1").is_none() && h.query_by_label(SECOND).is_some(),
        "the first change is gone and the second stays"
    );
    assert_eq!(
        h.ctx.memory(|memory| memory.focused()),
        focused,
        "due-check-key: the check box that stays keeps its id and its focus"
    );
}

/// Layout 11c scope C (N1's threshold cases, on the real lines): the race clock's line, with its
/// six actions, one point wider and one point narrower than the panel width where its actions
/// take one row fewer. At both, everything the line painted (its name and each action) lies in
/// its measured block, and the lowest painted bottom with the line's nonzero tail (`XS`, the
/// tracker's last line) ends it. The reference: the painted controls' rects, read from what
/// was drawn.
#[test]
fn a_tracker_line_paints_in_its_block_at_its_wrap_threshold() {
    use crate::design::test_harness::Themed as _;
    use egui_kittest::{Harness, kittest::Queryable};
    const NAME: &str = "Elapsed race time";
    const ACTIONS: [&str; 6] = [
        "Enter room",
        "Cistern or Sands",
        "Reach Kitchens seal",
        "Alcove rest",
        "Fight minute",
        "Collect soul",
    ];
    let at = |width: f32| {
        let mut race = race();
        let t = std::mem::take(&mut race.trackers);
        let mut h = Harness::builder()
            .with_size(eframe::egui::vec2(width, 1400.0))
            .themed_ui_state(|ui, t: &mut Trackers| view::show(ui, t, false), t);
        h.run_steps(4);
        let actions: Vec<eframe::egui::Rect> = ACTIONS
            .iter()
            .map(|name| h.get_by_label(name).rect())
            .collect();
        let name = h.get_by_label(NAME).rect();
        let rows = h
            .ctx
            .data(|data| {
                data.get_temp::<(u64, Vec<eframe::egui::Rect>)>(eframe::egui::Id::new(
                    crate::design::components::ROW_RECTS,
                ))
            })
            .map(|(_, rows)| rows)
            .unwrap_or_default();
        // The line's own block: the registered row that holds its first action and starts at
        // its name's line (not the Pinned heading's block over it).
        let block = rows
            .into_iter()
            .filter(|row| {
                row.contains(actions[0].center()) && row.top() >= name.top() - crate::design::S
            })
            .min_by(|a, b| a.height().total_cmp(&b.height()));
        let mut tops: Vec<f32> = actions.iter().map(|rect| rect.top()).collect();
        tops.dedup_by(|a, b| (*a - *b).abs() < 0.5);
        (tops.len(), block, name, actions)
    };
    // The narrowest width whose actions take as few rows as the widest panel's.
    let fewest = at(1200.0).0;
    let (mut narrow, mut wide) = (300.0_f32, 1200.0_f32);
    while wide - narrow > 1.0 {
        let middle = ((narrow + wide) / 2.0).floor();
        if at(middle).0 > fewest {
            narrow = middle;
        } else {
            wide = middle;
        }
    }
    let (above, below) = (at(wide), at(narrow));
    assert!(
        below.0 > above.0,
        "line-threshold-reached: {narrow} px wraps to {} rows, {wide} px holds {}",
        below.0,
        above.0
    );
    for (width, (_, block, name, actions)) in [(wide, above), (narrow, below)] {
        let block = block.unwrap_or_else(|| panic!("line-threshold: {width} px: the line's block"));
        let painted: Vec<eframe::egui::Rect> = actions.into_iter().chain([name]).collect();
        if let Err(why) = crate::design::pane::painted_in_block(block, crate::design::XS, &painted)
        {
            panic!("line-threshold: {width} px: {why}");
        }
    }
}

#[test]
fn no_note_in_play_reads_no_ledger_and_shows_nothing() {
    let mut race = race();
    // A Remove's result and its Undo are in the note's reader while the note is in play.
    let t = &mut race.trackers;
    let note = t.rules.trackers[0].note;
    t.played = None;
    t.press("favor", None, "Crowd loves it", None);
    t.played = Some(TONIGHT);
    t.test_settle();
    let older = t.test_history()["favor"][0].entry.clone();
    t.remove(note, &older);
    assert_eq!(history::footer_lines(t, note).len(), 1);
    // The note is renamed outside Splat before the next click. The note is still in play, so
    // its reader keeps the click's refusal and says what became of the note.
    t.played = None;
    t.press("favor", None, "Crowd loves it", None);
    t.played = Some(TONIGHT);
    t.test_settle();
    let other = t.test_history()["favor"][0].entry.clone();
    let (file_now, renamed) = (
        race.campaign.path.join("Tournament.md"),
        race.campaign.path.join("Renamed.md"),
    );
    let ledger_before = fs::read(ledger::path(&race.campaign.path)).unwrap();
    fs::rename(&file_now, &renamed).unwrap();
    let t = &mut race.trackers;
    t.remove(note, &other);
    assert_eq!(
        fs::read(ledger::path(&race.campaign.path)).unwrap(),
        ledger_before
    );
    let words: Vec<String> = history::footer_lines(t, note)
        .into_iter()
        .map(|line| line.words)
        .collect();
    assert!(
        words.contains(
            &"The entry “Crowd loves it” was not removed from Trackers.md. A house rule note \
              changed on disk. Check the house rule notes first."
                .to_owned()
        ),
        "{words:?}"
    );
    assert!(t.test_problems().contains(&(
        Some(note),
        "Tournament is not where it was. Splat reads the notes again when its window gets focus."
            .into()
    )));
    fs::rename(&renamed, &file_now).unwrap();
    // A note in play that was deleted keeps its row in Campaign → Rules and runs, so that the
    // DM can switch it off.
    fs::remove_file(race.campaign.path.join("Tournament.md")).unwrap();
    let mut options = Options::load(
        &race.campaign,
        &crate::content_workspace::Catalogue::load(&[], Some(&race.campaign)).unwrap(),
    )
    .unwrap();
    let id = options.in_play[0];
    assert_eq!(options.house_rules, [(id, format!("{id} (not found)"))]);
    options.in_play.clear();
    let document = CampaignDocument::load(&race.campaign).unwrap();
    options.save(&race.campaign, &document.text).unwrap();
    assert!(
        !fs::read_to_string(race.campaign.path.join("Campaign.md"))
            .unwrap()
            .contains("splat_in_play")
    );
    let index = Arc::new(ContentIndex::load(&[], Some(&race.campaign)).unwrap());
    poll(
        &mut race.trackers,
        Some(index.clone()),
        Some(&race.campaign),
        Some(TONIGHT),
        &race.timeline,
    );
    assert!(
        history::footer_lines(&race.trackers, note).is_empty(),
        "a note out of play shows no Undo remove"
    );
    // PR 676 review (P1): the Remove is still kept in its store, and the footer that is drawn
    // (its kept line, `history::footer`) offers no Undo remove for a note out of play.
    assert!(
        race.trackers.test_removed().is_some(),
        "the Remove is still kept"
    );
    assert!(
        !race.trackers.test_removed_drawn(note),
        "m-s0-4-out-of-play: the footer draws Undo remove for a note out of play"
    );
    fs::write(ledger::path(&race.campaign.path), "not a ledger").unwrap();
    let mut trackers = Trackers::with_ids(crate::hermetic::new_uuid);
    poll(
        &mut trackers,
        Some(index),
        Some(&race.campaign),
        None,
        &race.timeline,
    );
    assert!(!trackers.in_play());
    assert_eq!(trackers.due(), 0);
    assert!(trackers.unreadable.is_none(), "the ledger is not read");
}

/// M-S0.4 (M0567, M0569): the last Remove is a kept result of its store, and its Undo's press
/// reaches only the Remove its line showed. Remove A, then Remove B, which replaces A's Undo:
/// a press with A's old ticket puts nothing back; B's own Undo puts B back, and A stays out.
#[test]
fn a_remove_undo_reaches_only_the_remove_its_line_showed() {
    let mut race = race();
    let t = &mut race.trackers;
    let note = t.rules.trackers[0].note;
    t.played = None;
    t.press("favor", None, "Crowd loves it", None);
    t.press("favor", None, "Crowd loves it", None);
    t.played = Some(TONIGHT);
    t.test_settle();
    let a = t.test_history()["favor"][0].entry.clone();
    let b = t.test_history()["favor"][1].entry.clone();
    t.remove(note, &a);
    let a_ticket = t.test_removed_ticket();
    assert!(
        a_ticket.is_some(),
        "A's Remove is kept with its line's ticket"
    );
    t.remove(note, &b);
    let kept = t.ledger.clone();
    t.undo_remove(a_ticket);
    assert_eq!(
        t.ledger.changes, kept.changes,
        "m-s0-4-ticket: A's old ticket put an entry back"
    );
    assert_eq!(t.test_history_reason(), None);
    assert!(t.test_removed().is_some(), "B's Undo stays");
    t.test_undo_remove();
    assert!(
        t.ledger.changes.contains(&b) && !t.ledger.changes.contains(&a),
        "B's own Undo puts B back, and A stays out"
    );
    assert_eq!(t.test_removed(), None);
}
