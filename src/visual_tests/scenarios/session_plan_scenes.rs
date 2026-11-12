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
    d.state("travel-row-open-beside-scenes", Sizes::Standard);
    d.click("Travel");

    // App round 1, V14: when the scenes fold, `N scenes` has the caret of a folded section.
    // At 960x640, three more scenes fold them.
    d.act_at_size(SMALL, |d| {
        d.click("More Add actions");
        d.click("Add Scene");
        for words in ["Pay the toll", "Cross at dawn", "Rest at the ford"] {
            d.type_text(words);
            d.press(Key::Enter);
            d.expect(&format!("Open: {words}"));
        }
        d.press(Key::Escape);
    });
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
