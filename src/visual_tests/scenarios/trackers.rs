//! Trackers between fights (F4 of the session-cycle design): the Tournament's house rule note
//! in play, the Trackers view of Lookup beside the Session page's plan, Ctrl+T there and back,
//! quick actions on a tracker and on an instance, a bell's Due row with its named rolls, the
//! rail's `Due N`, the ledger's Undo through a restart, `Add to Timeline`, and a fight started
//! beside the view.
use super::*;
use crate::trackers::view::RULES_ROUTE;
use egui::accesskit::Role;
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
pub fn write_the_race(d: &Driver<'_>) {
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

/// Each pinned tracker line with actions, by its name, with its first action: Elapsed race time,
/// Crowd Favor, and the four lines of Seals. Souls in the jar has no action.
const LINES: [(&str, &str); 6] = [
    ("Elapsed race time", "Enter room"),
    ("Crowd Favor", "Crowd loves it"),
    ("Party", "Take seal: Party"),
    ("Old Guard", "Take seal: Old Guard"),
    ("Keyring", "Take seal: Keyring"),
    ("Convicts", "Take seal: Convicts"),
];

/// The pinned lines in the Lookup panel: the left edge of each name, the right edge of each
/// painted value (the text nearest the name on its line, after it), and the left edge of each
/// line's first action.
struct Lines {
    names: Vec<f32>,
    values: Vec<f32>,
    actions: Vec<f32>,
}

fn panel_columns(d: &Driver<'_>) -> Lines {
    let found = line_places(d);
    lines_of(found.into_iter().map(Option::unwrap_or_default))
}

/// Each pinned line's name, value and first action, where it is painted now.
fn line_places(d: &Driver<'_>) -> Vec<Option<(f32, Option<f32>, f32)>> {
    let panel = d
        .place_of(SEARCH_LOOKUP)
        .map_or(f32::MAX, |field| field.left() - 1.0);
    LINES
        .iter()
        .map(|(name, first)| {
            let mut named = None;
            d.each_painted_text(|text, _| {
                let bounds = text.visual_bounding_rect();
                if text.galley.job.text == *name && bounds.left() >= panel {
                    named = Some(bounds);
                }
            });
            let (named, action) = (named?, d.place_of(first)?);
            let mut value: Option<egui::Rect> = None;
            d.each_painted_text(|text, _| {
                let bounds = text.visual_bounding_rect();
                if (named.top()..named.bottom()).contains(&bounds.center().y)
                    && bounds.left() > named.right()
                    && value.is_none_or(|v| bounds.left() < v.left())
                {
                    value = Some(bounds);
                }
            });
            Some((named.left(), value.map(|v| v.right()), action.left()))
        })
        .collect()
}

/// The lines found, in `LINES` order; a line not found is left out, so the check fails.
fn lines_of(found: impl Iterator<Item = (f32, Option<f32>, f32)>) -> Lines {
    let mut lines = Lines {
        names: Vec::new(),
        values: Vec::new(),
        actions: Vec::new(),
    };
    for (name, value, action) in found.filter(|(_, _, action)| *action != 0.0) {
        lines.names.push(name);
        lines.values.extend(value);
        lines.actions.push(action);
    }
    lines
}

/// `panel_columns` for a panel whose last lines are below its fold: the lines in view, then the
/// view scrolled to the others, and back to its top. Only what the panel paints is measured;
/// a column edge is the same at any scroll.
fn panel_columns_scrolled(d: &mut Driver<'_>) -> Lines {
    let mut found = line_places(d);
    let wheel = d
        .place_of(SEARCH_LOOKUP)
        .map_or(egui::pos2(780.0, 300.0), |field| {
            egui::pos2(field.center().x, field.bottom() + 200.0)
        });
    let mut scrolled = 0.0;
    while found.iter().any(Option::is_none) && scrolled < 1200.0 {
        d.scroll_points_at(wheel, 40.0);
        scrolled += 40.0;
        for (at, place) in line_places(d).into_iter().enumerate() {
            found[at] = found[at].or(place);
        }
    }
    if scrolled > 0.0 {
        d.scroll_points_at(wheel, -scrolled);
    }
    lines_of(found.into_iter().map(Option::unwrap_or_default))
}

/// One edge for every pinned line with an action. A right-aligned number's text snaps to a
/// whole pixel, so its painted edge can differ by up to 1 px with its width.
fn one_edge(edges: &[f32]) -> bool {
    edges.len() == LINES.len() && edges.iter().all(|edge| (edge - edges[0]).abs() <= 1.0)
}

/// The same edges as before, each within the 1 px a snapped number can move.
fn same_edges(now: &[f32], before: &[f32]) -> bool {
    now.len() == LINES.len()
        && now.len() == before.len()
        && now.iter().zip(before).all(|(a, b)| (a - b).abs() <= 1.0)
}

/// FA2: each line of Seals keeps its actions with it. `Lose seal` starts at or after its line's
/// first action, never left of the line's name, and a line whose `Lose seal` wrapped stands
/// closer to it than the next line does. Returns the check, how many lines wrapped, and where.
fn seal_actions_with_their_line(d: &Driver<'_>) -> (bool, usize, String) {
    let places: Vec<_> = ["Party", "Old Guard", "Keyring", "Convicts"]
        .iter()
        .map(|line| {
            (
                d.place_of(&format!("Take seal: {line}")),
                d.place_of(&format!("Lose seal: {line}")),
            )
        })
        .collect();
    let mut held = places.iter().all(|p| matches!(p, (Some(_), Some(_))));
    let mut wrapped = 0;
    for (at, place) in places.iter().enumerate() {
        let (Some(take), Some(lose)) = place else {
            continue;
        };
        held &= lose.left() >= take.left() - 0.5;
        if lose.top() > take.bottom() {
            wrapped += 1;
            let inside = lose.top() - take.bottom();
            if let Some((Some(next), _)) = places.get(at + 1) {
                held &= next.top() - lose.bottom() > inside + 0.5;
            }
        }
    }
    let shown: Vec<_> = places
        .iter()
        .map(|(take, lose)| (take.map(|r| r.min), lose.map(|r| r.min)))
        .collect();
    (held, wrapped, format!("Take and Lose seal at {shown:?}"))
}

/// The panel's columns hold: every value ends at one edge, and the first actions either all
/// follow the values at one edge (`beside`) or each starts at its own line's name, under it.
fn columns_hold(lines: &Lines, beside: bool) -> bool {
    one_edge(&lines.values)
        && if beside {
            one_edge(&lines.actions) && lines.actions[0] > lines.values[0]
        } else {
            same_edges(&lines.actions, &lines.names)
        }
}

/// The words painted on the Trackers title line after `Trackers`, inside the Lookup panel.
fn title_line_words(d: &Driver<'_>) -> Vec<String> {
    let Some(field) = d.place_of(SEARCH_LOOKUP) else {
        return vec!["no Lookup".into()];
    };
    let mut title = None;
    d.each_painted_text(|text, _| {
        let bounds = text.visual_bounding_rect();
        if text.galley.job.text == "Trackers"
            && bounds.left() >= field.left() - 1.0
            && bounds.bottom() <= field.top()
        {
            title = Some(bounds);
        }
    });
    let Some(title) = title else {
        return vec!["no title".into()];
    };
    let mut words = Vec::new();
    d.each_painted_text(|text, _| {
        let bounds = text.visual_bounding_rect();
        if (title.top()..title.bottom()).contains(&bounds.center().y)
            && bounds.left() > title.right()
        {
            words.push(text.galley.job.text.clone());
        }
    });
    words
}

/// The tops of the painted lines of a Due row's words, from the line that starts them to the
/// row's `Done`, and the height of one line of the words.
fn notice_line_tops(d: &Driver<'_>, words: &str, done: &str) -> (Vec<f32>, f32) {
    let mut tops = Vec::new();
    let mut line = 0.0;
    let mut start = None;
    d.each_painted_text(|text, _| {
        if text.galley.job.text.starts_with(&words[..20]) {
            start = Some(text.pos.y);
        }
    });
    let (Some(start), Some(done)) = (start, d.place_of(done)) else {
        return (tops, line);
    };
    d.each_painted_text(|text, _| {
        let own = &text.galley.job.text;
        if own.len() > 3
            && words.contains(own.as_str())
            && (start - 0.5..done.bottom()).contains(&text.pos.y)
        {
            tops.extend(
                text.galley
                    .rows
                    .iter()
                    .map(|row| text.pos.y + row.rect().top()),
            );
            line = text.galley.rows[0].rect().height();
        }
    });
    tops.sort_by(f32::total_cmp);
    (tops, line)
}

/// The quick actions of the pinned trackers that record at case D, by their accessible names.
/// `Throw a gift` and `Lose seal` on a line at 0 are refused at their `min`, so they record
/// nothing to take back.
const QUICK: [&str; 13] = [
    "Enter room",
    "Cistern or Sands",
    "Reach Kitchens seal",
    "Alcove rest",
    "Fight minute",
    "Collect soul",
    "Crowd loves it",
    "Take seal: Party",
    "Lose seal: Party",
    "Take seal: Old Guard",
    "Lose seal: Old Guard",
    "Take seal: Keyring",
    "Take seal: Convicts",
];

/// With the result line at the panel's bottom, the lowest quick action that is wholly in view
/// takes its own press, and the footer's Undo takes the next: each press changes only its own
/// target. The act before stays the result line's after the Undo. In view means painted whole:
/// a tracker's lines are one row for the footer's whole-row cut (N12), and a line under the cut
/// keeps an accessible node in the scroll area's view while nothing of it is painted.
fn presses_at_the_footer_edge(d: &mut Driver<'_>) -> (bool, String) {
    let seen = |d: &Driver<'_>, node: &crate::visual_tests::driver::Found, name: &str| {
        let label = name.split(": ").next().unwrap_or(name);
        node.name == name && node.fully_in_view() && d.painted_whole_in(label, node.rect)
    };
    let whole = |d: &Driver<'_>| {
        QUICK
            .iter()
            .any(|name| d.nodes().iter().any(|node| seen(d, node, name)))
    };
    // Where the Due rows fill the panel (960), the DM scrolls to the trackers first.
    let wheel = d
        .place_of(SEARCH_LOOKUP)
        .map_or(egui::pos2(780.0, 300.0), |field| {
            egui::pos2(field.center().x, field.bottom() + 200.0)
        });
    for _ in 0..15 {
        if whole(d) {
            break;
        }
        d.scroll_points_at(wheel, 40.0);
    }
    let lowest = QUICK
        .iter()
        .filter_map(|name| {
            d.nodes()
                .into_iter()
                .find(|node| seen(d, node, name))
                .map(|node| (*name, node.rect.bottom()))
        })
        .max_by(|a, b| a.1.total_cmp(&b.1));
    let Some((name, bottom)) = lowest else {
        return (false, "no quick action wholly in view".into());
    };
    let label = name.split(": ").next().unwrap_or(name);
    let why = format!("why: {label}");
    let entries = |d: &Driver<'_>| ledger(d).matches(&why).count();
    let (before, total) = (entries(d), ledger(d).matches("why: ").count());
    d.click(name);
    let pressed = entries(d) > before && d.has(&format!("Undo: {label}"));
    d.click(&format!("Undo: {label}"));
    let undone = entries(d) == before
        && ledger(d).matches("why: ").count() == total
        && d.has_containing("Seals for Party changed to 1.");
    // Then a field outside the panel's body takes the next press and the keys.
    d.click(SEARCH_LOOKUP);
    d.type_text("zq");
    let outside = (d.focused_name(), d.field_text(SEARCH_LOOKUP));
    let typed = outside == (Some(SEARCH_LOOKUP.to_owned()), Some("zq".to_owned()));
    // Words typed there are a search that leaves Trackers: clear them, close Lookup, and
    // open it on Trackers with Ctrl+T, as it was.
    d.press_ctrl(Key::A);
    d.press(Key::Backspace);
    d.press_ctrl(Key::K);
    d.press_ctrl(Key::T);
    let restored = d.has_containing("Elapsed race time")
        && d.field_text(SEARCH_LOOKUP).unwrap_or_default().is_empty();
    (
        pressed && undone && typed && restored,
        format!(
            "{name} (bottom {bottom:.0}): pressed {pressed}, footer Undo took it back {undone}, outside field {outside:?}, Trackers again {restored}"
        ),
    )
}

/// A tracker with an amount the DM types, on two lines, and a mark that the amount can pass.
const PURSES: &str = "> trackers:\n>   purses:\n>     name: Purses\n>     pin: true\n>     per: [Ann, Bo]\n>     quick:\n>     - {label: Pay, add: ask}\n>     - {label: Tip, add: 1}\n>     marks:\n>     - {at: 3, say: Paid.}\n";

/// Open the amount field of `Pay` on Ann's line: it has the action's place and width and is at
/// least 24 px high, and `Tip` on both lines stays. Then 3 and Enter raise `Paid` above, and
/// `Pay` keeps its place. Undo takes the act back.
/// Where a line's actions are under it (960 beside Lookup), the DM scrolls Purses into view
/// first, as the panel's footer covers its lower rows. Are Ann's `Pay` and Bo's `Tip` wholly in
/// view?
fn purses_in_view(d: &mut Driver<'_>) -> bool {
    in_view_after_scroll(d, &["Pay: Ann", "Tip: Bo"])
}

/// Scroll the panel until each named control of Purses is wholly in view.
fn in_view_after_scroll(d: &mut Driver<'_>, names: &[&str]) -> bool {
    let wheel = d
        .place_of(SEARCH_LOOKUP)
        .map_or(egui::pos2(780.0, 300.0), |field| {
            egui::pos2(field.center().x, field.bottom() + 200.0)
        });
    let whole = |d: &Driver<'_>| {
        names.iter().all(|name| {
            d.nodes()
                .iter()
                .any(|node| node.name == *name && node.fully_in_view())
        })
    };
    for _ in 0..12 {
        if whole(d) {
            break;
        }
        d.scroll_points_at(wheel, 40.0);
    }
    whole(d)
}

fn amount_in_its_column(d: &mut Driver<'_>) -> (bool, String) {
    let place = |d: &Driver<'_>, name: &str| d.place_of(name).unwrap_or(egui::Rect::NOTHING);
    let same = |a: egui::Rect, b: egui::Rect| (a.min - b.min).length() < 0.5;
    let visible = purses_in_view(d);
    let (pay, tip, bo) = (
        place(d, "Pay: Ann"),
        place(d, "Tip: Ann"),
        place(d, "Tip: Bo"),
    );
    d.click("Pay: Ann");
    let field = place(d, "Pay");
    let (tip_now, bo_now) = (place(d, "Tip: Ann"), place(d, "Tip: Bo"));
    d.type_text("3");
    d.press(Key::Enter);
    d.expect("Paid for Ann is due.");
    let pay_now = place(d, "Pay: Ann");
    d.click("Undo: Pay");
    let held = visible
        && same(field, pay)
        && (field.width() - pay.width()).abs() < 0.5
        && field.height() >= 24.0
        && same(tip_now, tip)
        && same(bo_now, bo)
        && same(pay_now, pay);
    (
        held,
        format!(
            "Pay {pay:?}, field {field:?}, Tip {tip:?} then {tip_now:?}, Bo's Tip {bo:?} then {bo_now:?}, Pay after Enter {pay_now:?}"
        ),
    )
}

/// A day mark, as a DM writes one in the note's Splat data: the month's end, every 30 days from
/// Day 2, with one proposed change.
const MONTH_END: &str = "> day_marks:\n>   month_end:\n>     name: End of month\n>     every: 30\n>     from: 2\n>     say: Move the rival powers. Pay each team its prize money, and roll Instability if chaos has begun.\n>     changes:\n>     - {tracker: favor, add: 1}\n";

/// Day marks (P11): the Timeline's day count raises a day mark's Due row, with the words, the
/// rows and the acts of a mark at a value. Days that pass in one step are one row whose `Done`
/// names the day it ends;
/// a clock that goes back takes the row away and keeps what was done.
fn day_marks(d: &mut Driver<'_>) {
    use crate::timeline::Kind;
    let note = root(d).join("Tournament.md");
    let text = fs::read_to_string(&note).unwrap();
    fs::write(
        &note,
        text.replacen("> trackers:\n", &format!("{MONTH_END}> trackers:\n"), 1),
    )
    .unwrap();
    d.app_mut().lookup.refresh();
    d.app_mut().lookup.files_changed = true;
    d.settle();
    // Day 1, then 61 days on the road: Day 62. The month ended on Day 32 and on Day 62.
    let timeline = &mut d.app_mut().timeline;
    timeline.add(Kind::Day, None);
    let road = timeline.add(Kind::Travel, None);
    timeline.event_mut(&road).unwrap().duration = "61 days".into();
    timeline.touch();
    timeline.save();
    d.settle();
    d.observe(
        "T19.5",
        d.has_containing("End of month ×2. Next: Day 32. Move the rival powers.")
            && d.has("Done: End of month, Day 32")
            && d.painted_left("Done: Day 32").is_some()
            && d.has_label("DUE 3")
            && d.has_containing("3 notices are due")
            && !ledger(d).contains("month_end"),
        "Days that pass in one step are one Due row that says how many, with Done: Day 32; raising it writes nothing",
    );
    let (whole, detail) = d.at_size(SMALL, |d| {
        let missing: Vec<_> = ["Done: End of month, Day 32", "Undo: Take seal"]
            .into_iter()
            .filter(|name| !d.in_view(name))
            .chain(
                ["End of month ×2. Next: Day 32.", "Crowd Favor +1"]
                    .into_iter()
                    .filter(|words| !d.in_view_containing(words)),
            )
            .collect();
        (missing.is_empty(), format!("not in view: {missing:?}"))
    });
    d.observe(
        "T19.5",
        whole,
        format!("At 960x640 with Lookup narrow, the day mark's row, its Done and the result line are in view. {detail}"),
    );
    d.state("trackers-day-mark-due", Sizes::Standard);
    d.click("Done: End of month, Day 32");
    d.expect("End of month, Day 32 is done.");
    d.observe(
        "T19.5",
        d.has_containing("Day 62: Move the rival powers.")
            && d.has("Done: End of month, Day 62")
            && ledger(d).contains("day: 32"),
        "Done: Day 32 ends the oldest day and applies its change; the row names the next day",
    );
    // The Timeline's Undo takes the road back: Day 62 is no longer reached and its row goes;
    // the Done of Day 32 stays in the ledger.
    d.app_mut().timeline.undo_add(&road);
    d.app_mut().timeline.save();
    d.settle();
    d.observe(
        "T19.5",
        !d.has_containing("Move the rival powers.")
            && d.has_label("DUE 2")
            && ledger(d).contains("day: 32"),
        "A clock that goes back takes the row away and keeps what was done",
    );
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
    // FN6: the empty state's route is a bordered button that shows Campaign → Rules and runs.
    d.aside(|d| {
        d.press_ctrl(Key::T);
        d.expect("No house rules in play");
        let framed = d
            .control_frame(RULES_ROUTE)
            .is_some_and(|frame| frame.stroke.width > 0.0);
        let away = !d.has("House rules in play");
        d.click(RULES_ROUTE);
        d.expect("House rules in play");
        d.observe(
            "T19.1",
            framed
                && away
                && d.has_containing("Trackers (Ctrl+T) shows the trackers of each note in play."),
            format!(
                "The empty state's {RULES_ROUTE} is bordered ({framed}) and shows the switches of the house rules"
            ),
        );
        d.press_ctrl(Key::T);
        d.observe(
            "T19.1",
            !d.has(SEARCH_LOOKUP),
            "Ctrl+T again closes the Lookup that Ctrl+T opened",
        );
    });

    // Campaign → Rules and runs has a switch for each house rule note.
    d.click(NAV_CAMPAIGN);
    d.click(TAB_RUN_SETTINGS);
    d.expect("House rules in play");
    d.observe(
        "T19.1",
        d.has_containing("Trackers (Ctrl+T) shows the trackers of each note in play."),
        "Under House rules in play a help line says where the trackers are",
    );
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
    // FE1: every pinned line stands in the panel's columns, not only the lines of Seals: each
    // value ends at one edge. Where every first action fits after the values (1200x800 and
    // 1920x1080) each first action starts at one edge after them; at 960x640 beside Lookup
    // `Enter room` does not, and every line's actions start under its name.
    // At 960x640 the last lines are below the panel's fold: the view scrolls to them, and back.
    // A proof step that comes back to the top: not part of the DM's task.
    let small = d.aside(panel_columns_scrolled);
    let wide = d.at_size(DEFAULT, panel_columns);
    let big = d.at_size(LARGE, panel_columns);
    d.observe(
        "T19.2",
        columns_hold(&small, false) && columns_hold(&wide, true) && columns_hold(&big, true),
        format!(
            "Values, actions and names of the pinned lines: {:?} {:?} {:?} at 960x640, {:?} {:?} at 1200x800, {:?} {:?} at 1920x1080",
            small.values, small.actions, small.names, wide.values, wide.actions, big.values, big.actions
        ),
    );
    // FA2: at 1200x800 beside Lookup `Lose seal` wraps; it stays right of its line's name, at
    // the first action's edge, and closer to its own line than to the next.
    let (wide_held, wraps, wide_where) = d.at_size(DEFAULT, seal_actions_with_their_line);
    let (big_held, _, big_where) = d.at_size(LARGE, seal_actions_with_their_line);
    let (small_held, _, small_where) = seal_actions_with_their_line(d);
    d.observe(
        "T19.2",
        wide_held && wraps > 0 && big_held && small_held,
        format!(
            "A wrapped action stays with its line: {wraps} wrapped at 1200x800, {wide_where}; at 1920x1080 {big_where}; at 960x640 {small_where}"
        ),
    );
    d.state("trackers-beside-the-plan", Sizes::All);

    // T19.2: a quick action is one press; a press that passes a mark raises a Due row. The
    // pressed action keeps its place, so a still pointer presses it again, not a roll.
    let pressed = d.place_of("Cistern or Sands");
    d.click("Cistern or Sands");
    d.expect("Elapsed race time changed to 20. Bell is due.");
    let now = d.place_of("Cistern or Sands");
    d.observe(
        "T19.2",
        matches!((pressed, now), (Some(a), Some(b)) if (a.min - b.min).length() < 0.5),
        format!(
            "A press that raises a Due row keeps the pressed action under the pointer: {pressed:?}, then {now:?}"
        ),
    );
    d.audit_goal_within("T19.2", "open the numbers and record a room", 2);
    // Round 28 review of PR 308: the view is the pane's scroll column. With a result in its
    // footer and short content at 1920x1080, the footer follows the content by `S`, and G6
    // finds no footer far below a short body.
    let (follows, gaps, detail) = d.at_size(LARGE, |d| {
        let gap = d.pane_records().iter().find_map(|record| {
            let (footer, viewport, height) = (
                record.footer()?,
                record.viewport()?,
                record.content_height()?,
            );
            (record.fits() && record.offset() <= 0.5)
                .then(|| footer.top() - (viewport.top() + height))
        });
        (
            gap.is_some_and(|gap| (gap - crate::design::S).abs() <= 1.0),
            d.short_body_gaps(),
            format!("{gap:?}"),
        )
    });
    d.observe(
        "T19.2",
        follows && gaps.is_empty(),
        format!("At 1920x1080 the footer follows the short Trackers list by S: gap {detail}, G6 {gaps:?}"),
    );
    d.observe(
        "T19.3",
        d.has_label("DUE 1") && d.has_containing("1 notice is due"),
        "The press raises a Due row above the trackers, and the rail counts it",
    );
    d.click("Done: Bell");
    d.expect("Bell is done.");
    // The race reaches 40: the second bell rolls each rival by name, and the wandering table.
    d.click("Enter room");
    d.click("Enter room");
    d.expect("Elapsed race time changed to 40. Bell is due.");
    // AE2: `0 / 120` became `40 / 120`, and the panel's columns held: no value edge and no
    // first action moved.
    let forty = d.at_size(LARGE, panel_columns);
    d.observe(
        "T19.2",
        same_edges(&forty.values, &big.values) && same_edges(&forty.actions, &big.actions),
        format!(
            "At 1920x1080, at 40 / 120 the values end and the first actions start where they did at 0 / 120: {:?} {:?}, then {:?} {:?}",
            big.values, big.actions, forty.values, forty.actions
        ),
    );
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
    d.expect("Seals for Party changed to 1. First seal for Party is due.");
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
            "Seals for Party changed to 1.",
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
    // A Due row's last line, with Done, keeps the words' line pitch; the result line's Undo
    // starts at its message's words.
    let seal_words =
        "Old Guard: First seal. The party reaches level 2. A rival uses its 1-2 seals group.";
    let pitch = |d: &Driver<'_>| {
        let (tops, line) = notice_line_tops(d, seal_words, "Done: First seal for Old Guard");
        let even = tops.len() > 1
            && tops
                .windows(2)
                .all(|pair| (pair[1] - pair[0] - line).abs() < 0.5);
        (even, tops)
    };
    let undo_at_words = |d: &Driver<'_>| {
        let (undo, words) = (
            d.painted_left("Undo: Take seal"),
            d.painted_left("Seals for Party changed to 1."),
        );
        (
            matches!((undo, words), (Some(u), Some(w)) if (u - w).abs() < 0.5),
            undo,
            words,
        )
    };
    let ((small_even, small_tops), small_undo) = (pitch(d), undo_at_words(d));
    let ((wide_even, wide_tops), wide_undo) = d.at_size(DEFAULT, |d| (pitch(d), undo_at_words(d)));
    d.observe(
        "T19.4",
        small_even && wide_even,
        format!("A Due row's lines, with Done on the last, keep one pitch: {small_tops:?} at 960x640, {wide_tops:?} at 1200x800"),
    );
    d.observe(
        "T19.2",
        small_undo.0 && wide_undo.0,
        format!("Undo's words start at the result's words: {small_undo:?} at 960x640, {wide_undo:?} at 1200x800"),
    );
    d.state("trackers-three-notices", Sizes::All);
    // Optimizer scan 26: with the result line at the bottom, a press near its edge goes to the
    // action under the pointer, and a press on the footer to the footer's action.
    let (edge, edge_detail) = presses_at_the_footer_edge(d);
    let (wide_edge, wide_detail) = d.act_at_size(DEFAULT, presses_at_the_footer_edge);
    d.observe(
        "T19.2",
        edge && wide_edge,
        format!("At 960x640: {edge_detail}. At 1200x800: {wide_detail}"),
    );

    // A wrong press seen late: the opposite action corrects the value, and the history keeps
    // both presses. The notice the wrong press raised has nothing on it, so it ends.
    d.click("Lose seal: Party");
    d.expect("Seals for Party changed to 0. First seal for Party is no longer due.");
    d.observe(
        "T19.2",
        ledger(d).matches("why: Take seal").count() == 2
            && ledger(d).contains("why: Lose seal")
            && !ledger(d).contains("dismissed")
            && d.has_label("DUE 2")
            && d.has_containing("2 notices are due")
            && !d.has("Done: First seal for Party"),
        "Lose seal corrects the value in one press, and the notice the wrong press raised ends; nothing is written for it",
    );
    // Undo steps back through tonight's acts, newest first: the notice comes back.
    d.click("Undo: Lose seal");
    d.expect("Seals for Party changed to 1. First seal for Party is due.");
    d.observe(
        "T19.2",
        d.has_label("DUE 3") && d.has("Done: First seal for Party"),
        "Undo of the opposite press brings the notice back",
    );
    d.click("Undo: Take seal");
    d.expect("Seals for Old Guard changed to 1. First seal for Old Guard is due.");
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
    // The search field does not read as a filter of the trackers: it is empty, with no Clear.
    d.observe(
        "T19.1",
        d.field_text(SEARCH_LOOKUP).unwrap_or_default().is_empty() && !d.has("Clear"),
        "Over Trackers the search field shows its placeholder, not the last search",
    );
    // FH8: the count belongs to the `DUE 2` heading; the title line says only `Trackers` and
    // its actions. FE3: `Add to Timeline` has no icon, so it is bordered.
    let title = title_line_words(d);
    d.observe(
        "T19.3",
        d.has_label("DUE 2") && !title.iter().any(|words| words.contains("Due")),
        format!("The title line holds no Due count beside the DUE 2 heading: {title:?}"),
    );
    d.observe(
        "T19.2",
        d.in_view("Add to Timeline")
            && d.control_frame("Add to Timeline")
                .is_some_and(|frame| frame.stroke.width > 0.0),
        "Add to Timeline, a message action without an icon, has a border",
    );
    d.state("trackers-over-an-entry", Sizes::Small);
    // Words typed there are a new search, which leaves Trackers for its result.
    d.type_into(SEARCH_LOOKUP, VARRA);
    d.wait_for_text(VARRA_WORDS);
    d.observe(
        "T19.1",
        !d.has_containing("Elapsed race time")
            && d.field_text(SEARCH_LOOKUP).as_deref() == Some(VARRA),
        "Words typed in the field over Trackers search, whole, and leave Trackers",
    );
    d.press_ctrl(Key::T);
    d.press_ctrl(Key::T);
    d.observe(
        "T19.1",
        d.has(SEARCH_LOOKUP)
            && !d.has_containing("Elapsed race time")
            && d.has_containing(VARRA_WORDS)
            && d.field_text(SEARCH_LOOKUP).as_deref() == Some(VARRA),
        "Ctrl+T again shows the entry that Lookup held with its search words, and Lookup stays open",
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
            .any(|event| event.title == "Take seal: Old Guard")
    };
    d.observe(
        "T19.2",
        added(d) && ledger(d).matches("why: Take seal").count() == 1,
        "Add to Timeline makes a Major event with the act's label and its line, and writes no ledger entry",
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
    d.expect("Seals for Old Guard changed to 1. First seal for Old Guard is due.");
    d.observe(
        "T19.2",
        d.has("Undo: Take seal") && d.has_label("DUE 2"),
        "After a restart the result line, its Undo and the Due rows are as before",
    );

    day_marks(d);

    // T19.6: last week's wrong press is corrected from the house rule note's reader. Each
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
        "T19.6",
        d.has_label("Start 0")
            && d.has(keyring)
            && d.has_containing("Take seal: Old Guard +1")
            && d.has_containing("3 notices are due"),
        "The history opens under its tracker with the start values",
    );
    d.state("tracker-history-in-the-reader", Sizes::All);
    // FE9: every row has its menu, so two rows look alike; on a row of the session being
    // played, Remove is off and says that Undo in Trackers takes it back.
    let played = "More actions for Take seal: Old Guard +1";
    // Opened by keyboard, the off Remove takes no focus and shows no tooltip: its reason is
    // painted in the menu.
    let reached = d.tab_to(played);
    d.press(Key::Enter);
    let painted = d
        .painted_in(egui::Rect::EVERYTHING)
        .contains(crate::trackers::history::REMOVE_PLAYED);
    d.observe(
        "T19.6",
        reached && painted && d.is_disabled("Remove"),
        format!("Opened by keyboard, the played row's menu paints why Remove is off: Tab reached it {reached}, reason painted {painted}"),
    );
    d.press(Key::Escape);
    d.click(played);
    let off = d.is_disabled("Remove");
    let why = d.tooltip_contains("Remove", crate::trackers::history::REMOVE_PLAYED);
    d.observe(
        "T19.6",
        off && why,
        format!("A row of the session being played has its menu, with Remove off ({off}) and its reason ({why})"),
    );
    d.press(Key::Escape);
    d.click(keyring);
    d.observe(
        "T19.6",
        d.has("Remove") && !d.is_disabled("Remove"),
        "Another session's row has Remove on",
    );
    d.press(Key::Escape);
    d.click(keyring);
    d.click("Remove");
    // AE1, FE11: of two rows named `Take seal`, the result names the one that went.
    d.expect("Removed Take seal: Keyring +1.");
    d.observe(
        "T19.6",
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
    d.expect("Crowd Favor changed to 2.");
    d.press_ctrl(Key::T);
    d.click("Undo remove");
    let text = ledger(d);
    d.observe(
        "T19.6",
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
            add: Some(-4),
            session: Some(uuid::Uuid::from_u128(7)),
            ..Entry::default()
        }]))
    })
    .unwrap();
    d.app_mut().trackers.read_disk();
    d.settle();
    d.click(keyring);
    d.click("Remove");
    d.click("CROWD FAVOR HISTORY 3");
    let forced = "Crowd Favor is -2, below its min of 0.";
    d.observe(
        "T19.6",
        d.has_containing(forced) && ledger(d).contains("add: -4"),
        "A value below its min is named in the note's reader and changed by nothing",
    );
    d.click("More actions for Crowd Favor change: -4");
    d.click("Remove");
    d.observe(
        "T19.6",
        !d.has_containing(forced) && !ledger(d).contains("c-0f0ced01"),
        "Remove of the forced row clears the problem",
    );
    // A ledger that cannot be read shows its name, and no history and no value.
    let good = ledger(d);
    fs::write(&path, "---\nchanges: [\n---\n").unwrap();
    d.app_mut().trackers.read_disk();
    d.settle();
    d.observe(
        "T19.6",
        d.has_containing("Trackers.md cannot be read: ")
            && !d.has_containing("SEALS HISTORY")
            && !d.has_containing("Take seal: Old Guard +1"),
        "An unreadable ledger shows its name and no history in the reader",
    );
    fs::write(&path, &good).unwrap();
    d.app_mut().trackers.read_disk();
    d.settle();
    // A row's menu is kept by its entry: when an edit on disk puts another entry in that row's
    // place, the open menu does not offer Remove for it.
    ledger::commit(&path, |_| Ok(Op::Restore(vec![(None, wrong.clone())]))).unwrap();
    d.app_mut().trackers.read_disk();
    d.settle();
    d.click(keyring);
    d.expect("Remove");
    fs::write(&path, ledger(d).replace("c-01de5ea1", "c-01de5ea2")).unwrap();
    d.app_mut().trackers.read_disk();
    d.settle();
    d.observe(
        "T19.6",
        d.has(keyring) && !d.has("Remove"),
        "A menu open on a row that an edit took away does not open on the entry in its place",
    );
    fs::write(&path, good).unwrap();
    d.app_mut().trackers.read_disk();
    d.settle();
    d.press(Key::Escape);
    d.press_ctrl(Key::T);

    // An amount field takes exactly its action's column at a target's height, so no action
    // moves; Enter is the press, and the action keeps its place when that raises a Due row.
    // Two amount fields with one owner (tracker, line, label) would share one id and one draft:
    // Purses still shows, the first keeps its field, a later one is left out, and the problem
    // stands under the line that leaves it out. Two actions with one label and fixed amounts
    // have no field: they load as before, and each press makes its own change.
    let note = root(d).join("Tournament.md");
    let text = fs::read_to_string(&note).unwrap();
    let show = |d: &mut Driver<'_>, purses: &str| {
        fs::write(&note, text.replacen("> trackers:\n", purses, 1)).unwrap();
        d.app_mut().lookup.refresh();
        d.app_mut().lookup.files_changed = true;
        d.settle();
        if !d.has_containing("Elapsed race time") {
            d.press_ctrl(Key::T);
        }
    };
    let count = |d: &Driver<'_>, name: &str| {
        d.nodes()
            .iter()
            .filter(|node| node.name == name && node.role != Role::Label)
            .count()
    };
    let mut repeats = Vec::new();
    for (purses, why, field, kept) in [
        (
            PURSES.replace("[Ann, Bo]", "[Ann, Ann]"),
            "Two lines are named “Ann”, and an action asks for an amount. Give each line its own name.",
            "Pay: Ann",
            "Tip: Ann",
        ),
        (
            PURSES.replace("{label: Tip, add: 1}", "{label: Pay, add: 1}"),
            "Two actions are named “Pay”, and one asks for an amount. Give each action its own name.",
            "Pay: Bo",
            "Pay: Ann",
        ),
    ] {
        show(d, &purses);
        in_view_after_scroll(d, &[field]);
        repeats.push((
            d.has_containing(why),
            count(d, field),
            count(d, kept),
            d.has_containing("Purses"),
        ));
    }
    // Ann twice: one Pay (the first Ann's), Tip on both lines. Pay twice: one Pay on each line.
    d.observe(
        "T19.2",
        repeats == [(true, 1, 2, true), (true, 1, 1, true)],
        format!("A repeated amount field is left out after the first, and its problem shows with the tracker: {repeats:?}"),
    );
    show(
        d,
        &PURSES.replace("{label: Pay, add: ask}", "{label: Tip, add: 2}"),
    );
    let tips = in_view_after_scroll(d, &["Tip: Ann"]);
    let mut buttons: Vec<egui::Rect> = d
        .nodes()
        .iter()
        .filter(|node| node.name == "Tip: Ann" && node.role != Role::Label)
        .map(|node| node.shown)
        .collect();
    buttons.sort_by(|a, b| {
        (a.top(), a.left())
            .partial_cmp(&(b.top(), b.left()))
            .unwrap()
    });
    let problem = d.has_containing("Give each action its own name.");
    let mut said = Vec::new();
    for rect in buttons.iter().take(2) {
        d.click_rect(*rect, "Click: Tip: Ann");
        said.push((
            d.has_containing("Purses for Ann changed to 2."),
            d.has_containing("Purses for Ann changed to 3."),
        ));
    }
    let each_its_own = said.len() == 2 && said[0] == (true, false) && said[1].1;
    d.observe(
        "T19.2",
        tips && buttons.len() == 2 && !problem && each_its_own,
        format!("Two Tip actions with fixed amounts load with no problem, and each press makes its own change (+2, then +1): {} buttons, problem {problem}, results {said:?}", buttons.len()),
    );
    for _ in 0..said.len() {
        d.click("Undo: Tip");
    }
    fs::write(&note, text.replacen("> trackers:\n", PURSES, 1)).unwrap();
    d.app_mut().lookup.refresh();
    d.app_mut().lookup.files_changed = true;
    d.settle();
    let shown = d.has_containing("Elapsed race time");
    if !shown {
        d.press_ctrl(Key::T);
    }
    let (held, detail) = amount_in_its_column(d);
    let (wide_held, wide_detail) = d.act_at_size(DEFAULT, amount_in_its_column);
    d.observe(
        "T19.2",
        held && wide_held,
        format!("An amount field keeps its action's column and the place of every action: {detail} at 960x640; {wide_detail} at 1200x800"),
    );
    // An amount typed at 1200 and not sent yet keeps its field, its focus and its words while
    // the width crosses to 960, where the line's actions are under it, and back; Enter then
    // saves it once.
    let pays = |d: &Driver<'_>| ledger(d).matches("why: Pay").count();
    let before = pays(d);
    let typed = |d: &Driver<'_>| (d.focused_name(), d.field_text("Pay"));
    let (wide, wide_field) = d.act_at_size(DEFAULT, |d| {
        purses_in_view(d);
        d.click("Pay: Ann");
        d.type_text("3");
        (typed(d), d.place_of("Pay"))
    });
    let narrow = typed(d);
    let (back, back_field) = d.act_at_size(DEFAULT, |d| {
        let held = (typed(d), d.place_of("Pay"));
        d.press(Key::Enter);
        held
    });
    let saved = pays(d) - before;
    let kept = (Some("Pay".to_owned()), Some("3".to_owned()));
    d.observe(
        "T19.2",
        wide == kept
            && narrow == kept
            && back == kept
            && wide_field.is_some()
            && back_field == wide_field
            && saved == 1,
        format!(
            "A typed amount crosses 1200 → 960 → 1200: focus and words {wide:?}, {narrow:?}, {back:?}; field {wide_field:?} then {back_field:?}; {saved} amount saved"
        ),
    );
    // The check above needs exactly one saved amount; take it back when there is one.
    if saved == 1 {
        d.click("Undo: Pay");
    }
    if !shown {
        d.press_ctrl(Key::T);
    }
    fs::write(&note, text).unwrap();
    d.app_mut().lookup.refresh();
    d.app_mut().lookup.files_changed = true;
    d.settle();

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
