//! Scenes in a session's plan (F3 of the session-cycle design): task lines as rows with a done
//! box, `Scene` in the Add strip's More list, a scene's link, and `Start fight` for a fight that
//! nobody planned, from a scene's menu and from a reader's header. Then F6: `Add to Log` from a
//! done scene and from a reader's header.
use super::*;
use crate::test_support::{CachedWork, Counts};
use std::fs;
use uuid::Uuid;

const VARRA: &str = "Warden Ostrel";
const MILLER: &str = "The miller is the informer";
const SHRINE: &str = "Shrine of Cinders";
const KEEP: &str = "Ashfall Keep";
/// A marked table, so a Travel row offers its encounter checks.
const ROAD: &str = "# Road\n\n## On the road\n\n<!-- splat:rolls key=road dice=1d20 per=day -->\n\n| d20 | On the road |\n| --- | --- |\n| 1-14 | Nothing stirs. |\n| 15-20 | A toll gang blocks the way. |\n";

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

/// Each painted text with exactly these words: where it is, and its sections' formats.
fn painted(d: &Driver<'_>, words: &str) -> Vec<(egui::Rect, Vec<egui::TextFormat>)> {
    let mut found = Vec::new();
    d.each_painted_text(|text, _| {
        if text.galley.job.text == words {
            found.push((
                text.galley.rect.translate(text.pos.to_vec2()),
                text.galley
                    .job
                    .sections
                    .iter()
                    .map(|section| section.format.clone())
                    .collect(),
            ));
        }
    });
    found
}

/// The fold link on the Tonight line, `N scenes`, when the scenes fold.
fn fold_shown(d: &Driver<'_>) -> bool {
    d.nodes().iter().any(|node| {
        node.name.split_once(' ').is_some_and(|(n, word)| {
            n.parse::<usize>().is_ok() && ["scene", "scenes"].contains(&word)
        })
    })
}

/// App round 1, V2 and V8: a scene's words start at the fights' title column, at a plan row
/// title's size, and its second line sits at a fight row's pitch.
fn scene_matches_fight(d: &Driver<'_>) -> (bool, String) {
    let words = d.place_of(&format!("{VARRA} (opens {VARRA})"));
    let detail = d.place_of("NPC · alive · Ashfall Keep");
    let fight = d.place_of(SHRINE);
    let summary = fight.and_then(|fight| {
        d.nodes()
            .into_iter()
            .map(|node| node.rect)
            .filter(|rect| (rect.left() - fight.left()).abs() <= 0.5 && rect.top() > fight.top())
            .min_by(|a, b| a.top().total_cmp(&b.top()))
    });
    let sizes: Vec<f32> = painted(d, VARRA)
        .iter()
        .chain(painted(d, MILLER).iter())
        .flat_map(|(_, formats)| formats.iter().map(|format| format.font_id.size))
        .collect();
    let column = words
        .zip(fight)
        .is_some_and(|(words, fight)| (words.left() - fight.left()).abs() <= 0.5);
    let pitch = words.zip(detail).zip(fight.zip(summary)).is_some_and(
        |((words, detail), (fight, summary))| {
            ((detail.top() - words.top()) - (summary.top() - fight.top())).abs() <= 0.5
        },
    );
    let size = !sizes.is_empty()
        && sizes
            .iter()
            .all(|size| *size == crate::design::Type::Body.size());
    (
        column && pitch && size,
        format!(
            "words {words:?}, second line {detail:?}; fight {fight:?}, its second line {summary:?}; sizes {sizes:?}"
        ),
    )
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
        let (matches, detail) = d.at_size(size, scene_matches_fight);
        d.observe(
            "T12.13",
            matches,
            format!(
                "At {size:?}, a scene's words start at the fights' title column, at a plan row title's size, with its second line at a fight row's pitch: {detail}"
            ),
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
    // App round 1, V3 and V13: Start fight first, the removal last, both lines with an icon,
    // so their words start at one x.
    let (start, remove) = (
        d.place_of(&format!("Start a fight with {VARRA}")),
        d.place_of("Remove from tonight"),
    );
    let lines: Vec<f32> = [("Start fight", start), ("Remove from tonight", remove)]
        .iter()
        .filter_map(|(words, line)| {
            let line = (*line)?;
            painted(d, words)
                .into_iter()
                .find(|(rect, _)| line.contains(rect.center()))
                .map(|(rect, _)| rect.left())
        })
        .collect();
    d.observe(
        "T12.13",
        start
            .zip(remove)
            .is_some_and(|(start, remove)| start.top() < remove.top())
            && lines.len() == 2
            && (lines[0] - lines[1]).abs() <= 0.5,
        format!("The scene's menu: Start fight {start:?} above Remove from tonight {remove:?}; words start at {lines:?}"),
    );
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
    let first = d.place_of(&format!("Resume: {RUN}"));
    d.click(&format!("{KEEP} · Location"));
    d.expect("Scene added to tonight.");
    d.observe(
        "T12.10",
        note(d).contains(&format!("- [[{SHRINE}]]\n- [ ] [[{KEEP}]]\n")) && d.has("Plan a scene"),
        "A suggestion adds a scene that links it, and the field stays open",
    );
    // App round 1, W8-D7: the field's result shows directly under the field, and no row above
    // the field moves for it.
    let (now, field, undo) = (
        d.place_of(&format!("Resume: {RUN}")),
        d.place_of("Plan a scene"),
        d.place_of("Undo scene"),
    );
    d.observe(
        "T12.14",
        first.zip(now).is_some_and(|(first, now)| (first.top() - now.top()).abs() <= 0.5)
            && field.zip(undo).is_some_and(|(field, undo)| {
                undo.top() >= field.bottom() && undo.top() - field.bottom() <= crate::design::XL
            }),
        format!("The first row was at {first:?} and is at {now:?}; the field is at {field:?}, Undo scene at {undo:?}"),
    );
    d.state("scene-added", Sizes::All);
    // App round 1, W1-D1: at 960x640 the scene just added stays in view while the DM types the
    // next one and its suggestions show: the fold holds while the field has focus.
    d.click("Plan a scene");
    d.type_text("ash");
    d.expect(&format!("{KEEP} · Location"));
    let (kept, detail) = d.at_size(SMALL, |d| {
        let added = d
            .nodes()
            .into_iter()
            .find(|node| node.name == format!("Open: {KEEP}"));
        (
            added.as_ref().is_some_and(|node| node.fully_in_view()) && !fold_shown(d),
            format!(
                "Open: {KEEP} {:?}, fold shown: {}",
                added.map(|node| node.rect),
                fold_shown(d)
            ),
        )
    });
    d.observe(
        "T12.14",
        kept,
        format!("At 960x640, the scene just added stays in view while the DM types: {detail}"),
    );
    // Escape clears the typed words and closes the field.
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
    // App round 1, V1: a done scene's words and links are muted; a link keeps its underline.
    let words: Vec<_> = [VARRA, MILLER]
        .iter()
        .flat_map(|words| painted(d, words))
        .flat_map(|(_, formats)| formats)
        .map(|format| format.color)
        .collect();
    let underline = d
        .place_of(&format!("{VARRA} (opens {VARRA})"))
        .map(|link| d.line_colors_in(link))
        .unwrap_or_default();
    let muted = crate::design::colors().muted;
    d.observe(
        "T12.13",
        words.len() >= 2
            && words.iter().all(|color| *color == muted)
            && !underline.is_empty()
            && underline.iter().all(|color| *color == muted),
        format!("A done scene's words and link are muted, the link underlined: words {words:?}, underline {underline:?}"),
    );
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

    // Round 26 FL1 contract: the plan's footer keeps every live Undo. A scene (A), then an
    // unrelated Long rest (B), then Undo scene: A's line goes, B's event and its Undo stay.
    d.click("More Add actions");
    d.click("Add Scene");
    d.type_text("ash");
    d.click(&format!("{KEEP} · Location"));
    d.expect("Scene added to tonight.");
    d.press(Key::Escape);
    let events = |d: &mut Driver<'_>| {
        d.app_mut()
            .timeline
            .events
            .iter()
            .filter(|event| event.title == "Long rest")
            .count()
    };
    let rests = events(d);
    d.click("Add Long rest");
    d.expect("Undo add");
    let added = events(d) == rests + 1;
    let both = d.in_view("Undo scene") && d.in_view("Undo add") && added;
    // Review of PR 274, item 2: live Undo results go newest first, whatever their kind.
    let top = |d: &Driver<'_>, name: &str| d.place_of(name).map(|rect| rect.top());
    let (rest_top, scene_top) = (top(d, "Undo add"), top(d, "Undo scene"));
    d.observe(
        "review-274-2-rest-newest",
        rest_top
            .zip(scene_top)
            .is_some_and(|(rest, scene)| rest < scene),
        format!("The newer Long rest's Undo {rest_top:?} is above the older scene's {scene_top:?}"),
    );
    d.click("Undo scene");
    let kept = events(d) == rests + 1;
    let undone = !note(d).contains(KEEP);
    d.observe(
        "iter-26-S-FL1-undo",
        both && undone && kept && d.in_view("Undo add"),
        "Both results stay listed; Undo scene takes A's line out, and B's event and Undo stay",
    );

    // The other order: the Long rest (now the older) and then a scene (C). C's Undo is above;
    // Undo of the older Long rest keeps C's line and its Undo.
    d.click("More Add actions");
    d.click("Add Scene");
    d.type_text("ash");
    d.click(&format!("{KEEP} · Location"));
    d.expect("Scene added to tonight.");
    d.press(Key::Escape);
    let (rest_top, scene_top) = (top(d, "Undo add"), top(d, "Undo scene"));
    d.observe(
        "review-274-2-scene-newest",
        rest_top
            .zip(scene_top)
            .is_some_and(|(rest, scene)| scene < rest),
        format!("The newer scene's Undo {scene_top:?} is above the older Long rest's {rest_top:?}"),
    );
    d.click("Undo add");
    let rest_gone = events(d) == rests;
    let scene_kept = note(d).contains(KEEP) && d.in_view("Undo scene");
    d.observe(
        "review-274-2-undo-older",
        rest_gone && scene_kept,
        format!("Undo of the older Long rest: its event gone {rest_gone}, the newer scene and its Undo kept {scene_kept}"),
    );

    // App round 1, W1-D1: the fold measures the rows as they are when closed. Two fights and two
    // scenes at 1200x800: a Travel row opened with its checks does not fold the scenes.
    fs::write(root(d).join("Road.md"), ROAD).unwrap();
    d.app_mut().lookup.refresh();
    d.settle();
    d.click_add_event("Add Day");
    d.click_add_event("Add Travel");
    d.click("More actions for Travel");
    d.click("Roll for encounters");
    d.expect("Encounter checks added to this journey.");
    let (shown, detail) = d.at_size(DEFAULT, |d| {
        (
            d.has(&format!("Done: {VARRA}")) && d.has(&format!("Done: {MILLER}")) && !fold_shown(d),
            format!(
                "Encounter checks shown: {}, fold shown: {}",
                d.has_containing("Encounter checks"),
                fold_shown(d)
            ),
        )
    });
    d.observe(
        "T12.14",
        shown,
        format!(
            "At 1200x800, the open Travel row with its checks leaves the scenes in place: {detail}"
        ),
    );
    d.click("Travel");

    // App round 1, V14: when the scenes fold, `N scenes` has the caret of a folded section.
    // At 960x640 these closed rows do not fit with the scenes, so the scenes fold.
    let caret = |d: &Driver<'_>, icon: &str| {
        let mut found = None;
        d.each_painted_text(|text, _| {
            let words = &text.galley.job.text;
            if words.starts_with(icon) && words.ends_with(" scenes") {
                found = Some(words.clone());
            }
        });
        found
    };
    let (folded, detail) = d.at_size(SMALL, |d| {
        let found = caret(d, crate::design::icons::NEXT);
        (
            fold_shown(d) && !d.has("Open: Pay the toll") && found.is_some(),
            format!("fold shown: {}, painted: {found:?}", fold_shown(d)),
        )
    });
    d.observe(
        "T12.13",
        folded,
        format!("At 960x640 the folded scenes' link has the section caret: {detail}"),
    );
    d.state("scenes-folded", Sizes::Small);
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
    // A done scene's fit is measured when its room changes, not on each frame.
    let fit = CachedWork::start("Done scene fit", d, |_: &mut Driver<'_>| {
        let (reached, work) = crate::sessions::page::scene_fit_counts();
        Counts { reached, work }
    });
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

    let short = format!("Add to Log: {VARRA_BLACK}");
    let done = format!("Done: {VARRA_BLACK}");
    let warm = fit.warm(d, d.has(&done), 1);
    d.observe(
        "T18.2-fit-warm",
        warm.is_ok(),
        format!("The plan measured the done scenes' fit: {warm:?}"),
    );
    let refresh = fit.refresh(d, 1, |d| d.at_size(LARGE, |d| d.has(&short)));
    d.observe(
        "T18.2-fit-refresh",
        refresh.is_ok(),
        format!("A wider room measures the fit again: {refresh:?}"),
    );
    let steady = fit.steady(d, |d| {
        d.settle();
        d.at_size(LARGE, |_| ());
        d.settle();
        d.has(&done)
    });
    d.observe(
        "T18.2-fit-steady",
        steady.is_ok(),
        format!(
            "Unchanged frames, and a room it measured before, do not measure it again: {steady:?}"
        ),
    );

    // A done scene that links one World entry has `Add to Log` in its action place, left of its
    // menu, while its words keep their rows; else in its menu. A scene still to come has none.
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
    d.state("done-scene-add-to-log", Sizes::All);
    // The fit rule: beside the long words the action would move a word, so it is a line of
    // that scene's menu, and the row is as a scene's row with no action.
    d.click(&long_menu);
    d.observe(
        "T18.2",
        d.has(&long)
            && d.place_of(&long).is_some_and(|line| {
                d.place_of(&long_menu)
                    .is_some_and(|menu| line.top() > menu.bottom())
            }),
        "Beside long words, Add to Log is a line of the scene's menu, not on its row",
    );
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
    // The measured journeys of app round 2 (J6 step 2): Lookup's reader of a campaign World
    // entry has `Change` on Status, as the World page's reader has. One act writes the value
    // and its Log line, its Undo is in Lookup's footer, and Lookup keeps the entry.
    d.click("Change Status");
    d.observe(
        "T15.4",
        d.focused_name().as_deref() == Some("Status"),
        "Change on Status in Lookup puts the keyboard in the field",
    );
    d.type_text("missing");
    d.press(Key::Enter);
    d.expect("Undo status change");
    let changed = fs::read_to_string(&path).unwrap();
    d.observe(
        "T15.4",
        changed.contains("status: missing")
            && changed.contains(": Status alive → missing.\n")
            && d.has(&format!("Edit {VARRA_BLACK}"))
            && d.has("Close Lookup"),
        format!("Change on Status in Lookup writes the value and its Log line: {changed:?}"),
    );
    d.state("status-changed-in-lookup", Sizes::Default);
    d.click("Undo status change");
    d.observe(
        "T15.4",
        fs::read_to_string(&path).unwrap() == before && d.has(&format!("Edit {VARRA_BLACK}")),
        "Undo status change in Lookup takes back the value and its Log line",
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
    // X30 of app round 2: beside the plan at 1200x800 the stamp shares the field's line, for
    // the field keeps the rule's room, whatever words it holds.
    let (label, place) = d.at_size(DEFAULT, |d| {
        (d.place_of(&format!("{stamp}:")), d.place_of(&field))
    });
    d.observe(
        "T18.2",
        label.zip(place).is_some_and(|(label, place)| {
            label.right() < place.left()
                && label.bottom() > place.top()
                && label.top() < place.bottom()
        }),
        format!("At 1200x800 the stamp {label:?} shares the field's line {place:?}"),
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
        !d.has(&field) && d.has("Close Lookup") && fs::read_to_string(&path).unwrap() == before,
        "Escape closes the field, not Lookup, and writes nothing",
    );

    // The reader's header, with a 19-letter title: `Add to Log` next to Edit gives way first, to
    // its icon, then away; `Start fight` keeps its word or does not show; the title keeps its
    // width. Beside Lookup at 960 neither fits, at 1200 `Start fight` alone, and wider both
    // words (section 1.2, measured).
    for (size, log_shown, start_shown) in [
        (SMALL, false, false),
        (DEFAULT, false, true),
        (LARGE, true, true),
    ] {
        let (fits, detail) = d.at_size(size, |d| {
            let (log, start, order) = reader_header(d, VARRA_BLACK);
            let word =
                log.is_some_and(|log| log.width() > crate::design::Density::Read.control() + 0.5);
            (
                order
                    && d.has(&format!("Edit {VARRA_BLACK}"))
                    && log.is_some() == log_shown
                    && (!log_shown || word)
                    && start.is_some() == start_shown,
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
    d.press(Key::Escape);

    // An edit form for her on the World page holds back the Log field in Lookup too.
    d.click("Close Lookup");
    d.click(NAV_WORLD);
    d.click_starting(&format!("{VARRA_BLACK} · "));
    d.click(&format!("Edit {VARRA_BLACK}"));
    d.type_into_editor(" Draft.");
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.click(&long_menu);
    d.click(&long);
    d.observe(
        "T18.2",
        d.visible_text("Save or discard your edit first.")
            && !d.has(&field)
            && fs::read_to_string(&path).unwrap() == before,
        "An edit form on the World page holds back the Log field in Lookup",
    );
    d.click("Close Lookup");
    d.click(NAV_WORLD);
    d.click("Discard");

    // Words typed in the Log field survive Trackers: Ctrl+T, and Ctrl+T again, give back the
    // entry with its field.
    d.click(NAV_SESSIONS);
    show_plan(d);
    d.click(&long_menu);
    d.click(&long);
    d.expect(&field);
    d.type_text(TYPED);
    d.press_ctrl(Key::T);
    d.press_ctrl(Key::T);
    d.observe(
        "T18.2",
        d.field_text(&field).as_deref() == Some(TYPED),
        "Typed Log words survive Ctrl+T and Ctrl+T again",
    );

    // Every draft that the campaign switch's guard and the close warning count has a field in
    // view, reached from where the warning sends the DM. A renamed played session does not
    // hide the words: they belong to the campaign and the entry.
    let note_path = d.app().sessions.selected_path().unwrap().to_owned();
    fs::rename(
        &note_path,
        note_path.with_file_name("Renamed played session.md"),
    )
    .unwrap();
    d.app_mut().sessions.refresh();
    d.settle();
    let blocked =
        d.app().blocked_switch() == Some("Save or discard your changes to change campaigns.");
    d.request_close();
    let warned = d.in_view_containing(&format!("Edits to “{VARRA_BLACK}” are not saved."));
    d.click("Keep editing");
    d.observe(
        "T18.2-guard",
        blocked && warned && d.field_text(&field).as_deref() == Some(TYPED),
        "After the played session is renamed, Keep editing shows the field with the typed words",
    );
    // An entry that is gone: its draft waits for no guard, and the footer says the words.
    fs::remove_file(&path).unwrap();
    d.app_mut().lookup.refresh();
    d.settle();
    let free = d.app().blocked_switch().is_none();
    d.app_mut().lookup.open_id(keep);
    d.settle();
    d.observe(
        "T18.2-guard",
        free && d.visible_text(&format!(
            "“{VARRA_BLACK}” was deleted. The Log line was not added: “{TYPED}”"
        )),
        "A deleted entry's typed Log words block nothing, and the footer says them",
    );
    // X28 of app round 2: the notice offers the words with `Copy text`, as a lost draft does,
    // and stays until the DM copies or dismisses it, also while Lookup shows no entry.
    let notice = format!("“{VARRA_BLACK}” was deleted. The Log line was not added: “{TYPED}”");
    let copy = format!("Copy the Log line for {VARRA_BLACK}");
    d.observe(
        "T18.2-guard",
        d.has(&copy) && d.has(&format!("Dismiss the Log line for {VARRA_BLACK}")),
        "The deleted entry's notice has Copy text and Dismiss",
    );
    d.state("log-words-of-a-deleted-entry", Sizes::Default);
    d.app_mut().lookup.search_for(KEEP);
    d.settle();
    d.click("Clear");
    d.observe(
        "T18.2-guard",
        !d.has("Edit Ashfall Keep") && d.visible_text(&notice) && d.has(&copy),
        "With no entry on show, Lookup's footer still says the deleted entry's words",
    );
    d.state("log-words-without-an-entry", Sizes::Default);
    d.app_mut().lookup.open_id(keep);
    d.settle();
    d.click(&copy);
    d.observe(
        "T18.2-guard",
        d.copied_text().as_deref() == Some(TYPED) && !d.visible_text(&notice) && !d.has(&copy),
        "Copy text puts the words on the clipboard and ends the notice",
    );
}
