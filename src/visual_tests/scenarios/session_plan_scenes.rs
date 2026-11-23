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

/// The baseline of the first line of the painted text with exactly these words.
fn first_baseline(d: &Driver<'_>, words: &str) -> Option<f32> {
    let mut found = None;
    d.each_painted_text(|text, _| {
        if text.galley.job.text == words
            && let Some(row) = text.galley.rows.first()
            && let Some(glyph) = row.glyphs.first()
        {
            found.get_or_insert(text.pos.y + row.pos.y + glyph.pos.y);
        }
    });
    found
}

/// Round 27 FL4: a fight's caret is centred on its title's target, and a scene's box sits on
/// its words as the caret sits on its title (each measured from its title's baseline).
fn lead_centred(d: &Driver<'_>) -> (bool, String) {
    let line = crate::design::Type::Body.line();
    let scene_box = d
        .place_of(&format!("Open: {VARRA}"))
        .map(|rect| rect.center().y);
    let fight = d.place_of(SHRINE);
    let caret = fight.and_then(|fight| {
        let area = egui::Rect::from_x_y_ranges(
            fight.left() - 2.0 * crate::design::XL..=fight.left(),
            fight.top() - crate::design::S..=fight.top() + line + crate::design::S,
        );
        d.painted_word_rect(crate::design::icons::NEXT, area)
            .map(|rect| rect.center().y)
    });
    let (scene_line, title_line) = (first_baseline(d, VARRA), first_baseline(d, SHRINE));
    let centred = fight
        .zip(caret)
        .is_some_and(|(fight, caret)| (fight.center().y - caret).abs() <= 1.0);
    let same = scene_box
        .zip(scene_line)
        .zip(caret.zip(title_line))
        .is_some_and(|((lead, words), (caret, title))| {
            ((words - lead) - (title - caret)).abs() <= 1.0
        });
    (
        centred && same,
        format!(
            "fight target {fight:?}, caret {caret:?}, title baseline {title_line:?}; box {scene_box:?}, scene baseline {scene_line:?}"
        ),
    )
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
    // App round 2, X12: measured on the painted words, by the baseline of each text's first
    // line. The scene's first line sits as far below its row's top (its box) as the fight's
    // title below its own (its link's target), and its second line as far below its first as
    // the fight's summary below the title.
    let baseline = |left: f32, below: f32, words: Option<&str>| {
        let mut found: Option<f32> = None;
        d.each_painted_text(|text, _| {
            let rect = text.galley.rect.translate(text.pos.to_vec2());
            let Some((row, glyph)) = text
                .galley
                .rows
                .first()
                .and_then(|row| Some((row, row.glyphs.first()?)))
            else {
                return;
            };
            let line = text.pos.y + row.pos.y + glyph.pos.y;
            if (rect.left() - left).abs() <= 0.5
                && rect.top() >= below - 0.5
                && words.is_none_or(|words| text.galley.job.text == words)
                && found.is_none_or(|found| line < found)
            {
                found = Some(line);
            }
        });
        found
    };
    let scene_top = d.place_of(&format!("Open: {VARRA}"));
    let scene_line = words.and_then(|words| baseline(words.left(), words.top(), Some(VARRA)));
    let detail_line = detail.and_then(|detail| {
        baseline(
            detail.left(),
            detail.top(),
            Some("NPC · alive · Ashfall Keep"),
        )
    });
    let title_line = fight.and_then(|fight| baseline(fight.left(), fight.top(), Some(SHRINE)));
    let summary_line = summary.and_then(|summary| baseline(summary.left(), summary.top(), None));
    let first_line = scene_top
        .zip(scene_line)
        .zip(fight.zip(title_line))
        .is_some_and(|((row, line), (fight, title))| {
            ((line - row.top()) - (title - fight.top())).abs() <= 0.5
        });
    let pitch = scene_line
        .zip(detail_line)
        .zip(title_line.zip(summary_line))
        .is_some_and(|((words, detail), (title, summary))| {
            ((detail - words) - (summary - title)).abs() <= 0.5
        });
    let size = !sizes.is_empty()
        && sizes
            .iter()
            .all(|size| *size == crate::design::Type::Body.size());
    (
        column && first_line && pitch && size,
        format!(
            "words {words:?}, second line {detail:?}; fight {fight:?}, its second line {summary:?}; \
             baselines: scene row {scene_top:?}, words {scene_line:?}, second line {detail_line:?}; \
             fight title {title_line:?}, summary {summary_line:?}; sizes {sizes:?}"
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
    // caret. Round 28 FD3: Tonight counts the four rows under it, the done scene too.
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
                d.has_label("TONIGHT 4")
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
    // Round 27 FL4: the leading slot sits on its title line's centre: a scene's box and a
    // fight's caret, each within 1 px.
    for size in [SMALL, DEFAULT, LARGE] {
        let (centred, detail) = d.at_size(size, lead_centred);
        d.observe(
            "iter-27-S-FL4",
            centred,
            format!("At {size:?}, the box and the caret are centred on their titles: {detail}"),
        );
    }
    d.state("scenes-in-the-plan", Sizes::All);

    // T12.11: a scene's link opens its entry in Lookup, beside the plan, in one click.
    d.begin_task();
    d.click_starting(&format!("{VARRA} (opens"));
    d.wait_for_text("A note for tonight.");
    d.audit_goal_within("T12.11", "read the NPC that a scene links", 1);
    // Round 27 FC4, FJ3, FM5: the reader's header shows Edit, `Start fight` and `Add to Log`,
    // each with its word, at every size: after the title when they fit, else together on one
    // line under it.
    for size in [SMALL, DEFAULT, LARGE] {
        let (fits, detail) = d.at_size(size, |d| {
            let (log, start, order) = reader_header(d, VARRA);
            let word = |rect: Option<egui::Rect>| {
                rect.is_some_and(|rect| rect.width() > crate::design::Density::Read.control() + 0.5)
            };
            (
                word(start) && word(log) && order,
                format!("Add to Log {log:?}, Start fight {start:?}"),
            )
        });
        d.observe(
            "T12.12",
            fits,
            format!("At {size:?}, the reader's actions all show with their words. {detail}"),
        );
    }
    d.state("scene-link-in-lookup", Sizes::All);

    // A scene that links one NPC has Start fight in its menu, beside Remove from tonight: 2.
    d.click(&format!("More actions for {VARRA}"));
    d.expect("Remove from tonight");
    // App round 1, V3 and V13: Start fight first, the removal last, both lines with an icon,
    // so their words start at one x.
    // The reader beside the plan has its own `Start fight` on its title line (round 27): the
    // menu's is the one just above the removal.
    let remove = d.place_of("Remove from tonight");
    let start = remove.and_then(|remove| {
        d.nodes()
            .into_iter()
            .filter(|node| {
                node.name == format!("Start a fight with {VARRA}") && node.rect.top() < remove.top()
            })
            .map(|node| node.rect)
            .max_by(|a, b| a.top().total_cmp(&b.top()))
    });
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
    // the field moves for it. Round 29 P-1: the field's form ends at its action line (`Add`,
    // `Cancel`), so the result is directly under that. R30, amended: where the grown list pushes
    // the result out of the column, the least scroll that shows it whole is allowed: the rows
    // move up only so far that `Undo scene` ends at the column's end (1.5 px of tolerance).
    let (now, field, undo) = (
        d.place_of(&format!("Resume: {RUN}")),
        d.place_of("Plan a scene")
            .zip(d.place_of("Cancel"))
            .map(|(field, cancel)| field.union(cancel)),
        d.nodes()
            .into_iter()
            .find(|node| node.name == "Undo add scene"),
    );
    let column_end = undo.as_ref().and_then(|undo| {
        d.pane_records().into_iter().find_map(|pane| {
            pane.viewport()
                .filter(|view| view.contains(undo.rect.center()))
                .map(|view| view.bottom())
        })
    });
    let scroll = first.zip(now).map(|(first, now)| first.top() - now.top());
    let least = scroll.is_some_and(|scroll| scroll.abs() <= 0.5)
        || scroll
            .zip(undo.as_ref())
            .zip(column_end)
            .is_some_and(|((scroll, undo), end)| {
                scroll > 0.0 && undo.fully_in_view() && undo.rect.bottom() >= end - 1.5
            });
    let undo = undo.map(|undo| undo.rect);
    d.observe(
        "T12.14",
        least
            && field.zip(undo).is_some_and(|(field, undo)| {
                undo.top() >= field.bottom() && undo.top() - field.bottom() <= crate::design::XL
            }),
        format!("The first row was at {first:?} and is at {now:?} (scroll {scroll:?}, the column ends at {column_end:?}); the field is at {field:?}, Undo scene at {undo:?}"),
    );
    // App round 2, X13: `Undo scene` takes a line of its own only when it is wider than the
    // room after the words in their column, which ends where the rows' menus end.
    for size in [SMALL, DEFAULT, LARGE] {
        let (words, undo, column) = d.at_size(size, |d| {
            (
                painted(d, "Scene added to tonight.")
                    .first()
                    .map(|(rect, _)| *rect),
                d.place_of("Undo add scene"),
                d.place_of(&format!("More actions for {SHRINE}"))
                    .map(|rect| rect.right()),
            )
        });
        let wraps_only_when_wider =
            words
                .zip(undo)
                .zip(column)
                .is_some_and(|((words, undo), column)| {
                    undo.top() < words.bottom()
                        || words.right() + crate::design::S + undo.width() > column + 0.5
                });
        d.observe(
            "X13",
            wraps_only_when_wider,
            format!("At {size:?}: the words at {words:?}, Undo scene at {undo:?}, the column ends at {column:?}"),
        );
    }
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
    d.click("Undo add scene");
    d.observe(
        "T12.10",
        !note(d).contains(KEEP) && !d.has("Plan a scene"),
        "Undo scene takes the line back out, and Escape closed the field",
    );
    // Ruling R30, amended (round 29, P-1 review): the open field keeps room for its result. A
    // press on the Add button adds the scene as a row ABOVE the field, so `Add` moves down with
    // that row (the list grows, as on main with Enter). The result and its Undo are then whole in
    // view. Where the grown list pushes the room out of the column (960x640), the LEAST scroll that
    // shows the whole result is allowed (rounded up to a whole pixel, 1 px of tolerance), and
    // `Add` moves by its row less exactly that scroll; elsewhere nothing scrolls. Words that match
    // no World entry, so no suggestion comes or goes.
    for size in [SMALL, DEFAULT, LARGE] {
        let (still, detail) = d.act_at_size(size, |d| {
            let before_note = note(d);
            d.click("More Add actions");
            d.click("Add Scene");
            d.type_text("Cross at dawn");
            let add_button = |d: &Driver<'_>| {
                d.nodes()
                    .into_iter()
                    .find(|node| node.name == "Add" && node.role == egui::accesskit::Role::Button)
                    .map(|node| node.rect)
            };
            // The plan's scroll column: the pane whose viewport holds `Add`.
            let column = |d: &Driver<'_>, add: egui::Rect| {
                d.pane_records().into_iter().find_map(|pane| {
                    pane.viewport()
                        .filter(|view| view.contains(add.center()))
                        .map(|view| (view, pane.offset()))
                })
            };
            let places = |d: &Driver<'_>| {
                (
                    add_button(d),
                    d.place_of("Plan a scene"),
                    d.place_of(&format!("Resume: {RUN}")),
                )
            };
            let (add, field, first) = places(d);
            let before = add.and_then(|add| column(d, add));
            d.click("Add");
            d.expect("Undo add scene");
            let (add_now, field_now, first_now) = places(d);
            let after = add_now.and_then(|add| column(d, add));
            let nodes = d.nodes();
            let find = |name: &str| nodes.iter().find(|node| node.name == name);
            let (words, undo) = (find("Scene added to tonight."), find("Undo add scene"));
            // The scroll the press made, and the new row's height: how far the form went down
            // in the content (on screen, less that scroll).
            let scroll = before.zip(after).map(|((_, was), (_, now))| now - was);
            let row = field
                .zip(field_now)
                .zip(scroll)
                .map(|((field, now), scroll)| now.top() - field.top() + scroll);
            // How far the whole result would reach past the column's end with no scroll.
            let overflow = after.zip(undo).zip(words).zip(scroll).map(
                |((((view, _), undo), words), scroll)| {
                    (undo.rect.bottom().max(words.rect.bottom()) + scroll - view.bottom()).max(0.0)
                },
            );
            let least = scroll.zip(overflow).is_some_and(|(scroll, overflow)| {
                scroll >= -0.5 && scroll <= overflow.ceil() + 1.0
            }) && (size == SMALL || scroll.is_some_and(|scroll| scroll.abs() <= 0.5));
            let still = least
                && first.zip(first_now).zip(scroll).is_some_and(|((first, now), scroll)| {
                    (first.top() - now.top() - scroll).abs() <= 0.5
                })
                && add.zip(add_now).zip(row).zip(scroll).is_some_and(
                    |(((add, now), row), scroll)| {
                        row > 0.0 && (now.top() - add.top() - (row - scroll)).abs() <= 0.5
                    },
                )
                // Whole inside the column's edges, to the hundredth of a point.
                && after.zip(words).zip(undo).is_some_and(|(((view, _), words), undo)| {
                    [words.rect, undo.rect].iter().all(|rect| {
                        rect.top() >= view.top() - 0.01 && rect.bottom() <= view.bottom() + 0.01
                    })
                });
            let detail = format!(
                "scroll {scroll:?} px for an overflow of {overflow:?} px; the first row at {first:?} then {first_now:?}; Add at {add:?} then {add_now:?}, the new row {row:?} px; the words at {:?} shown {:?}, Undo scene at {:?} shown {:?}",
                words.map(|node| node.rect),
                words.map(|node| node.shown),
                undo.map(|node| node.rect),
                undo.map(|node| node.shown),
            );
            d.click("Undo add scene");
            d.click("Cancel");
            (still && note(d) == before_note && !d.has("Plan a scene"), detail)
        });
        d.observe(
            "T12.14",
            still,
            format!("At {size:?}, Add moves by its new row (less the least scroll that shows the result), and the result is whole in view: {detail}"),
        );
    }

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
            && d.has_label("TONIGHT 4"),
        "A tick changes one character of the note, and the count of rows stays",
    );
    d.click(&format!("More actions for {MILLER}"));
    d.click("Remove from tonight");
    d.expect(&format!("Removed “{MILLER}” from tonight."));
    d.state("scene-removed", Sizes::All);
    // Round 29 P-1 review (R30): the field's reveal follows its layout room, not the paint clip.
    // With the field open and a message in the plan's footer, the DM's own wheel scroll (which
    // changes the footer's whole-row cut) stays where the wheel put it, in a window low enough
    // (960x520) that the plan scrolls with the field open.
    let (kept, detail) = d.act_at_size((960.0, 520.0), |d| {
        d.click("More Add actions");
        d.click("Add Scene");
        let column = |d: &Driver<'_>| {
            let field = d.place_of("Plan a scene")?;
            d.pane_records().into_iter().find_map(|pane| {
                let view = pane.viewport().filter(|view| view.contains(field.center()))?;
                Some((view, pane.offset(), pane.content_height().unwrap_or(0.0)))
            })
        };
        let Some((view, was, content)) = column(d) else {
            d.click("Cancel");
            return (false, "no scroll column holds the Scene field".to_owned());
        };
        let footer = d.has(&format!("Removed “{MILLER}” from tonight."));
        // Up, far enough that the field's room leaves the view (a reveal would pull it back);
        // down when the column is at its top. The scroll area clamps at its ends.
        let wheel = if was >= 1.0 { -was.min(60.0) } else { 60.0 };
        d.scroll_points_at(view.center(), wheel);
        let expected = (was + wheel).clamp(0.0, (content - view.height()).max(0.0));
        let now = column(d).map(|(_, offset, _)| offset);
        d.click("Cancel");
        (
            // 1 px: the wheel itself lands on physical pixels.
            footer && now.is_some_and(|now| (now - expected).abs() <= 1.0 && (now - was).abs() > 1.0),
            format!("footer message shown: {footer}; offset {was} then {now:?}, the wheel put it at {expected}"),
        )
    });
    d.observe(
        "T12.14",
        kept,
        format!("At 960x520, the DM's wheel scroll stays with the Scene field open: {detail}"),
    );
    // M-N1.1: the removal's Undo names the scene it puts back (`UndoStep::of`).
    d.click(&format!("Undo remove {MILLER}"));
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
    // M-N1.7: a removed event's Undo names it (`Undo remove Long rest`), on the plan as on
    // Timeline; an added event's is `Undo add` (M0529 waits for layout 11c), a scene's
    // `Undo add scene`.
    let rest_undo = |d: &Driver<'_>| {
        d.nodes()
            .into_iter()
            .find(|node| node.name == "Undo add")
            .map(|node| node.name)
            .unwrap_or_default()
    };
    let remove_undo = |d: &Driver<'_>| {
        d.removal_undo("Long rest")
            .expect("the removal\u{2019}s Undo names the Long rest")
    };
    d.click("Add Long rest");
    assert!(
        !rest_undo(d).is_empty(),
        "the Long rest\u{2019}s Undo shows"
    );
    let added = events(d) == rests + 1;
    let both = d.in_view("Undo add scene") && d.in_view(&rest_undo(d)) && added;
    // Review of PR 274, item 2: live Undo results go newest first, whatever their kind.
    let top = |d: &Driver<'_>, name: &str| d.place_of(name).map(|rect| rect.top());
    let (rest_top, scene_top) = (top(d, &rest_undo(d)), top(d, "Undo add scene"));
    d.observe(
        "review-274-2-rest-newest",
        rest_top
            .zip(scene_top)
            .is_some_and(|(rest, scene)| rest < scene),
        format!("The newer Long rest's Undo {rest_top:?} is above the older scene's {scene_top:?}"),
    );
    d.click("Undo add scene");
    let kept = events(d) == rests + 1;
    let undone = !note(d).contains(KEEP);
    d.observe(
        "iter-26-S-FL1-undo",
        both && undone && kept && d.in_view(&rest_undo(d)),
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
    let (rest_top, scene_top) = (top(d, &rest_undo(d)), top(d, "Undo add scene"));
    d.observe(
        "review-274-2-scene-newest",
        rest_top
            .zip(scene_top)
            .is_some_and(|(rest, scene)| scene < rest),
        format!("The newer scene's Undo {scene_top:?} is above the older Long rest's {rest_top:?}"),
    );
    d.click(&rest_undo(d));
    let rest_gone = events(d) == rests;
    let scene_kept = note(d).contains(KEEP) && d.in_view("Undo add scene");
    d.observe(
        "review-274-2-undo-older",
        rest_gone && scene_kept,
        format!("Undo of the older Long rest: its event gone {rest_gone}, the newer scene and its Undo kept {scene_kept}"),
    );

    // Review of PR 298, item 1: a removal made on Timeline after the scene is the newer result in
    // the plan too, as its order is kept with the removal itself. Then Undo, and the same removal
    // again on Timeline, with no plan drawn between: the new removal is newer again.
    d.click("Add Long rest");
    assert!(
        !rest_undo(d).is_empty(),
        "the Long rest\u{2019}s Undo shows"
    );
    let removal_newest = |d: &mut Driver<'_>| {
        d.click(NAV_TIMELINE);
        d.click_starting("Long rest · ");
        d.click("More actions for Long rest");
        d.click("Remove this event");
        d.click(NAV_SESSIONS);
        d.expect("Undo remove Long rest");
        let (remove_top, scene_top) = (top(d, &remove_undo(d)), top(d, "Undo add scene"));
        (
            events(d) == rests
                && remove_top
                    .zip(scene_top)
                    .is_some_and(|(remove, scene)| remove < scene),
            format!("The Timeline removal's Undo {remove_top:?}, the older scene's {scene_top:?}"),
        )
    };
    let (first, first_detail) = removal_newest(d);
    d.observe("review-298-1-removal-newest", first, first_detail);
    d.click(&remove_undo(d));
    let back = events(d) == rests + 1;
    let (again, again_detail) = removal_newest(d);
    d.observe(
        "review-298-1-removal-again",
        back && again,
        format!("Undo put the Long rest back {back}; {again_detail}"),
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
    let varra = entry(
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
    // Round 28 FC8: one session, one name. Splat names it as the Sessions page does
    // (`sessions::Note::label`); the line it writes links the file.
    let label = {
        let path = d.app().sessions.selected_path().expect("a session is open");
        crate::sessions::list_notes(path.parent().unwrap())
            .into_iter()
            .find(|note| note.path == path)
            .expect("the open session's note")
            .label()
    };
    assert_eq!(label, "Session 1: The Crossing");
    let shown_stamp = stamp.replacen(&session, &label, 1);

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
            // App round 2, X29: on the row it has no icon, as the row's other actions.
            let icon = action
                .is_some_and(|action| d.painted_in(action).contains(crate::design::icons::ADD));
            (
                !open
                    && !icon
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
    // The window is 1200x800 here (the base above).
    let beside = d
        .place_of(&format!("{shown_stamp}:"))
        .zip(d.place_of(&field))
        .is_some_and(|(label, place)| {
            label.right() < place.left()
                && label.bottom() > place.top()
                && label.top() < place.bottom()
        });
    d.observe(
        "T18.2",
        d.has("Close Lookup")
            && d.focused_name().as_deref() == Some(field.as_str())
            && d.field_text(&field).as_deref() == Some("")
            && d.visible_text(&format!("{shown_stamp}:"))
            && beside,
        "A done scene's Add to Log opens Lookup on its entry with the Log field and the stamp on its line at 1200x800",
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
    d.click("Undo add Log line");
    d.observe(
        "T18.2",
        fs::read_to_string(&path).unwrap() == before && !d.has("Undo add Log line"),
        "Undo add Log line takes back that one line, and the note is as before",
    );
    // The measured journeys of app round 2 (J6 step 2): Lookup's reader of a campaign World
    // entry has `Change` on Status, as the World page's reader has. One act writes the value
    // and its Log line, its Undo is in Lookup's footer, and Lookup keeps the entry. The field
    // starts from the status on disk, also when another reader or Obsidian changed it after
    // Lookup opened the entry (Astra's review of PR 287).
    let away = before.replacen("status: alive", "status: away", 1);
    fs::write(&path, &away).unwrap();
    d.app_mut().lookup.refresh();
    d.settle();
    // D-2: the DM changes the status at 960, where the reader is short: the field opens with
    // room for the new Log entry in view, so the act moves nothing and the entry is whole.
    d.set_base(SMALL);
    d.click("Change Status");
    d.observe(
        "T15.4",
        d.focused_name().as_deref() == Some("Status")
            && d.field_text("Status").as_deref() == Some("away"),
        "Change on Status in Lookup puts the keyboard in the field, with the status on disk",
    );
    d.type_text("missing");
    const STATUS_ENTRY: &str = ": Status away → missing.";
    let lookup_pane = |d: &Driver<'_>| {
        d.place_of(SEARCH_LOOKUP).map(|search| {
            egui::Rect::from_min_max(
                egui::pos2(search.left(), 0.0),
                egui::pos2(f32::INFINITY, f32::INFINITY),
            )
        })
    };
    // Immediately before and after the act, at 960: the title (the witness of any scroll),
    // the focused field and, after the act, `Change` in the field's place.
    let title_before = lookup_pane(d).and_then(|pane| d.painted_word_rect(VARRA_BLACK, pane));
    let field_before = d
        .nodes()
        .into_iter()
        .find(|node| node.name == "Status" && node.role == egui::accesskit::Role::TextInput)
        .map(|node| node.rect);
    // Layout step 11b, from the act's own frame on (Astra's review of PR 736): Enter in one
    // input frame, then each frame until the app settles. In each: the title's top (the witness
    // of any scroll), whether the new entry is painted cut by the pane's clip, and whether the
    // result's Undo is whole, when it is there.
    let enter = |pressed| egui::Event::Key {
        key: Key::Enter,
        physical_key: None,
        pressed,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    };
    let frames = d.each_frame_after([enter(true), enter(false)], 6, |d| {
        let pane = lookup_pane(d);
        (
            pane.and_then(|pane| d.painted_word_rect(VARRA_BLACK, pane))
                .map(|title| title.top()),
            pane.is_some_and(|pane| {
                d.cut_texts_in(pane)
                    .iter()
                    .any(|text| text.contains(STATUS_ENTRY))
            }),
            d.nodes()
                .into_iter()
                .find(|node| node.name == "Undo change to Status")
                .map(|node| node.fully_in_view()),
        )
    });
    d.expect("Undo change to Status");
    let title_after = lookup_pane(d).and_then(|pane| d.painted_word_rect(VARRA_BLACK, pane));
    let change_after = d.place_of("Change Status");
    // The new entry lands whole in the body room (`design::pane::keep_room`) and is never cut on
    // the way; the title keeps its place in every frame, the act's own first; and the result's
    // Undo is whole in every frame it shows, as the footer grew into room the arbiter kept clear
    // (`reveal_with_focus_and_growth`). Issue #761 (`m-p0-lookup-undo-at-act`): Lookup builds its
    // footer after its body draws the act (`pane_editor_then`), so the Undo is there, whole, in
    // the act's own frame.
    let entry_at_act = lookup_pane(d).is_some_and(|pane| d.painted_in(pane).contains(STATUS_ENTRY))
        && frames.iter().all(|(_, cut, _)| !cut);
    let undo_at_act = frames
        .first()
        .is_some_and(|(_, _, undo)| *undo == Some(true))
        && frames.iter().all(|(_, _, undo)| undo.unwrap_or(true))
        && d.nodes()
            .into_iter()
            .any(|node| node.name == "Undo change to Status" && node.fully_in_view());
    let title_kept = title_before.is_some_and(|before| {
        frames
            .iter()
            .all(|(top, _, _)| top.is_some_and(|top| (top - before.top()).abs() <= 0.5))
    });
    let changed = fs::read_to_string(&path).unwrap();
    d.observe(
        "T15.4",
        changed.contains("status: missing")
            && changed.contains(": Status away → missing.\n")
            && d.has(&format!("Edit {VARRA_BLACK}"))
            && d.has("Close Lookup"),
        format!("Change on Status in Lookup writes the value and its Log line: {changed:?}"),
    );
    // Round 28 FC8: Lookup names the session as the Session page beside it does, in the Log
    // line, in `From other notes` and in its row's link, never by its file.
    let named = |d: &Driver<'_>| {
        let pane = d.place_of(SEARCH_LOOKUP).map(|search| {
            egui::Rect::from_min_max(
                egui::pos2(search.left(), 0.0),
                egui::pos2(f32::INFINITY, f32::INFINITY),
            )
        });
        let words = pane.map(|pane| d.painted_in(pane)).unwrap_or_default();
        // `From other notes` links the session by its name; at 960 it is below the fold.
        let link = d
            .nodes()
            .into_iter()
            .any(|node| node.role == egui::accesskit::Role::Link && node.name == label);
        (
            pane.is_some() && !words.contains(&session) && words.contains(&label) && link,
            format!("link {link}, painted {words:?}"),
        )
    };
    // FC8 reads each size from the capture below, in its settled frame after its resize; a look
    // across resizes before it would scroll the reader away from what the act left in view.
    let mut at_capture = Vec::new();
    // Capture all three reviewer sizes; each look is after its resize.
    d.state_prepared("status-changed-in-lookup", Sizes::All, |d| {
        d.settle();
        let entry = lookup_pane(d).is_some_and(|pane| d.painted_in(pane).contains(STATUS_ENTRY));
        at_capture.push((named(d), entry));
    });
    // D-2 (a), before and after the act at 960: `Change` stands where the focused field stood,
    // the title does not move in any frame (nothing scrolls when the entry and the result come),
    // the entry is never cut, and the Undo is whole in each frame it shows.
    let still = field_before
        .zip(change_after)
        .is_some_and(|(field, change)| {
            change.top() >= field.top() - 0.5
                && change.bottom() <= field.bottom() + crate::design::L
        })
        && title_kept
        && title_before
            .zip(title_after)
            .is_some_and(|(before, after)| (before.top() - after.top()).abs() <= 0.5);
    d.observe(
        "iter-29-D2-status-still",
        still && entry_at_act && undo_at_act,
        format!("At {SMALL:?}: title {title_before:?} → {title_after:?}; the field {field_before:?}, then Change {change_after:?}; entry whole {entry_at_act}, Undo whole {undo_at_act}; each frame from the act (title top, entry cut, Undo whole) {frames:?}"),
    );
    for (size, ((fits, detail), entry)) in [SMALL, DEFAULT, LARGE].into_iter().zip(at_capture) {
        d.observe(
            "iter-28-D-FC8",
            fits,
            format!("At {size:?}, Lookup names the session `{label}`, not `{session}`: {detail}"),
        );
        // The new Status entry, both its lines, painted whole after each resize.
        d.observe(
            "iter-29-D2-status-log",
            entry,
            format!("At {size:?}, the new entry `{STATUS_ENTRY}` is painted whole: {detail}"),
        );
    }
    d.set_base(DEFAULT);
    d.click("Undo change to Status");
    d.observe(
        "T15.4",
        fs::read_to_string(&path).unwrap() == away && d.has(&format!("Edit {VARRA_BLACK}")),
        "Undo change to Status in Lookup takes back the value and its Log line",
    );
    fs::write(&path, &before).unwrap();
    d.app_mut().lookup.refresh();
    d.settle();
    // Review of PR 355: a new title in Session details reaches Lookup at once, through the
    // app's own refresh: its link names the session as the Sessions page now does.
    let session_links = |d: &Driver<'_>| -> Vec<String> {
        d.nodes()
            .into_iter()
            .filter(|node| {
                node.role == egui::accesskit::Role::Link && node.name.starts_with("Session 1")
            })
            .map(|node| node.name)
            .collect()
    };
    let named_before = session_links(d);
    d.click("Edit session details");
    d.replace_text_in("Session title", "The Long Crossing");
    d.click("Save details");
    d.settle();
    let named_after = session_links(d);
    d.observe(
        "iter-28-D-FC8-title",
        named_before == [label.clone()] && named_after == ["Session 1: The Long Crossing"],
        format!(
            "Lookup's session link before {named_before:?}, after the new title {named_after:?}"
        ),
    );
    d.click("Edit session details");
    d.replace_text_in("Session title", "The Crossing");
    d.click("Save details");
    d.settle();

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
    // X30 of app round 2: beside the plan at 1200x800 the stamp stands above a field whose words
    // would not fit beside it, so no word of the scene is cut; an empty field shares its line
    // (the first check of this scenario's field).
    let (label, place) = d.at_size(DEFAULT, |d| {
        (d.place_of(&format!("{shown_stamp}:")), d.place_of(&field))
    });
    d.observe(
        "T18.2",
        label
            .zip(place)
            .is_some_and(|(label, place)| label.bottom() <= place.top() + 0.5),
        format!("At 1200x800 the stamp {label:?} stands above the field {place:?} of long words"),
    );
    d.state("log-field-from-scene", Sizes::All);
    // Round 27: the title-line actions move under the title at 960 and back beside it when
    // they fit; the field below keeps its ID, so it keeps the keyboard across both changes.
    d.observe(
        "iter-27-D-title-focus",
        d.focused_name().as_deref() == Some(field.as_str()),
        "After 960, 1200 and 1920, the Log field still has the keyboard",
    );
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

    // The reader's header, with a 19-letter title (round 27 FC4, FJ3, FM5): no action goes
    // away or loses its word. When the title and all of them do not fit on the title's line,
    // Edit, `Start fight` and `Add to Log` move together to one line under the title, `XS`
    // apart. Beside Lookup at 960 they are under the title.
    for size in [SMALL, DEFAULT, LARGE] {
        let (fits, detail) = d.at_size(size, |d| {
            let (log, start, order) = reader_header(d, VARRA_BLACK);
            let word = |rect: Option<egui::Rect>| {
                rect.is_some_and(|rect| rect.width() > crate::design::Density::Read.control() + 0.5)
            };
            let edit = d.place_of(&format!("Edit {VARRA_BLACK}"));
            let title = d.place_of(SEARCH_LOOKUP).and_then(|search| {
                let (width, height) = d.window_size();
                d.painted_word_rect(
                    VARRA_BLACK,
                    egui::Rect::from_min_max(
                        egui::pos2(search.left() - 1.0, search.bottom()),
                        egui::pos2(width, height),
                    ),
                )
            });
            let under = edit
                .zip(title)
                .is_some_and(|(edit, title)| edit.top() >= title.bottom() - 0.5);
            (
                order && word(edit) && word(log) && word(start) && (size != SMALL || under),
                format!(
                    "Edit {edit:?}, Add to Log {log:?}, Start fight {start:?}, title {title:?}"
                ),
            )
        });
        d.observe(
            "T18.2",
            fits,
            format!(
                "At {size:?}, every title-line action shows with its word, on one line: {detail}"
            ),
        );
    }
    d.state("log-header-long-title", Sizes::All);

    // An edit form blocks the field: Splat makes no "changed on disk" warning of its own.
    d.click(&format!("Edit {VARRA_BLACK}"));
    d.type_into_editor(" Draft.");
    let discard = d.place_of("Discard");
    d.click("Close Lookup");
    d.click(&long_menu);
    d.click(&long);
    let both = crate::world::LOG_WAITS_FOR_SAVE;
    d.observe(
        "T18.2",
        d.visible_text(both) && !d.has(&field),
        "While the entry has an edit form, the Log field waits",
    );
    // Round 27 FC2: the form the DM types in keeps its place when the waiting field comes.
    // Round 29 FC10 (ruling Q-t): the one cause is said once, at the save row, in one sentence
    // with the campaign switch; the footer does not say it again.
    let waiting = d.place_of("Discard");
    let reason = d.painted_rect_in(both, egui::Rect::EVERYTHING);
    d.observe(
        "iter-27-D-FC2",
        discard.zip(waiting).is_some_and(|(a, b)| (a.top() - b.top()).abs() <= 0.5)
            && reason.zip(waiting).is_some_and(|(reason, discard)| {
                reason.center().y >= discard.top() - 0.5
                    && reason.center().y <= discard.bottom() + 0.5 + crate::design::L * 2.0
            })
            && !d.visible_text("Save or discard your edit to add a Log line.")
            && no_switch_reason(d),
        format!("Discard stays at {discard:?} → {waiting:?}; the one reason {reason:?} is at the save row"),
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
        d.visible_text("Save or discard your edit to add a Log line.")
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
    // R3f X42: the discard of the lost draft is a result of the pane, so it and its Undo sit in
    // Lookup's footer, first, before the warning; the open entry's text keeps its place.
    let undo = d.place_of(&format!("Undo discard of {VARRA_BLACK}"));
    let footer = footer_with(d, &format!("{VARRA_BLACK} draft discarded."));
    let warning = d.painted_rect_in(
        &format!("“{VARRA_BLACK}” was deleted."),
        egui::Rect::EVERYTHING,
    );
    d.observe(
        "r3f-X42-discard-in-footer",
        undo.zip(footer)
            .is_some_and(|(undo, footer)| footer.contains_rect(undo))
            && undo
                .zip(warning)
                .is_some_and(|(undo, warning)| undo.top() < warning.top()),
        format!(
            "Undo discard {undo:?} in Lookup's footer {footer:?}, above the warning {warning:?}"
        ),
    );
    // App round 2, X14: the scene whose entry is gone says `Not found`, in the warning tone, on
    // its second line at the words' column, as Lookup and World say it.
    for size in [SMALL, DEFAULT] {
        let (said, detail) = d.at_size(size, |d| {
            let row = d.place_of(&format!("Done: {VARRA_BLACK}"));
            let found: Vec<_> = painted(d, "Not found")
                .into_iter()
                .filter(|(rect, formats)| {
                    row.is_some_and(|row| {
                        (rect.left() - row.right()).abs() <= 0.5
                            && rect.top() > row.top()
                            && rect.top() < row.top() + 2.0 * crate::design::XL
                    }) && formats
                        .iter()
                        .all(|format| format.color == crate::design::colors().result.warning)
                })
                .map(|(rect, _)| rect)
                .collect();
            (
                found.len() == 1,
                format!("row {row:?}, Not found at {found:?}"),
            )
        });
        d.observe("X14", said, format!("At {size:?}: {detail}"));
    }
    // X28 of app round 2: the notice offers the words with `Copy text`, as a lost draft does,
    // and stays until the DM copies or dismisses it, also while Lookup shows no entry.
    let notice = format!("“{VARRA_BLACK}” was deleted. The Log line was not added: “{TYPED}”");
    let copy = "Copy Log text".to_owned();
    d.observe(
        "T18.2-guard",
        d.has(&copy) && d.has(crate::design::outcome::DISMISS_NAME),
        "The deleted entry's notice has Copy text and Dismiss",
    );
    // Round 27 FC1, AC1, SC1: the lost draft's `Undo discard` is no body place of Ashfall
    // Keep: it is a footer line, just above the deleted entry's notice, and its press works.
    let everywhere = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(f32::MAX, f32::MAX));
    let discarded = format!("{VARRA_BLACK} draft discarded.");
    let undo = format!("Undo discard of {VARRA_BLACK}");
    for size in [SMALL, DEFAULT] {
        let (result, notice_at) = d.at_size(size, |d| {
            (
                d.painted_rect_in(&discarded, everywhere),
                d.painted_rect_in(&notice, everywhere),
            )
        });
        d.observe(
            "iter-27-D-FC1",
            result.zip(notice_at).is_some_and(|(result, notice)| {
                result.bottom() <= notice.top()
                    && notice.top() - result.bottom() < 3.0 * crate::design::XXL
            }) && d.has(&undo),
            format!("At {size:?}, the discarded draft's result {result:?} sits in the footer above the notice {notice_at:?}"),
        );
    }
    d.state("log-words-of-a-deleted-entry", Sizes::Default);
    d.click(&undo);
    let lost = "Discard draft".to_owned();
    d.observe(
        "iter-27-D-FC1-undo",
        d.has(&lost) && !d.has(&undo) && d.has(&copy),
        "Undo discard in Lookup's footer brings the lost draft back, with its choices, and keeps the notice",
    );
    // Review of PR 333: the two copy choices have two names. Copy draft text puts the lost
    // draft's text on the clipboard and keeps both notices; Copy Log text (below) puts the Log
    // words there and ends only its notice.
    let draft = d
        .app()
        .lookup
        .reader
        .draft_text(varra)
        .or(d.app().world_page.reader.draft_text(varra))
        .map(str::to_owned);
    d.click("Copy draft text");
    d.observe(
        "iter-28-D-copy-draft",
        draft.is_some()
            && d.copied_text() == draft
            && d.has(&lost)
            && d.visible_text(&notice)
            && d.has(&copy),
        format!("Copy draft text copies {draft:?}; both notices stay"),
    );
    d.click(&lost);
    d.expect(&undo);
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
    // Round 28 row 134, ruling R3: the close icon lets the typed words go only with an Undo,
    // and the Undo brings the notice and its words back.
    let undo_dismiss = format!("Undo dismiss {VARRA_BLACK}’s Log line");
    d.click(crate::design::outcome::DISMISS_NAME);
    d.observe(
        "iter-28-D-134-dismiss",
        !d.visible_text(&notice) && d.has(&undo_dismiss),
        "Dismiss ends the notice and offers Undo dismiss",
    );
    // Review of PR 333: no unrelated reset drops the dismissed words. Lookup's Clear (the
    // reader starts again), another entry B, and a campaign round trip; then Undo.
    let b_path = root(d).join("Locations/Ashfall Keep.md");
    let b_before = fs::read(&b_path).unwrap();
    d.type_into(SEARCH_LOOKUP, "Ash");
    d.click("Clear");
    d.app_mut().lookup.open_id(keep);
    d.settle();
    crate::visual_tests::steps::switch_campaign(d, crate::visual_tests::fixture::QUIET_CAMPAIGN);
    d.settle();
    crate::visual_tests::steps::switch_campaign(d, crate::visual_tests::fixture::CAMPAIGN);
    d.settle();
    d.app_mut().lookup.open_id(keep);
    d.settle();
    d.expect(&undo_dismiss);
    d.click(&undo_dismiss);
    d.observe(
        "iter-28-D-134-undo",
        d.visible_text(&notice)
            && d.has(&copy)
            && !d.has(&undo_dismiss)
            && fs::read(&b_path).unwrap() == b_before,
        "After Clear, entry B and a campaign round trip, Undo dismiss brings A's exact words back; B is unchanged",
    );
    d.click(&copy);
    d.observe(
        "T18.2-guard",
        d.copied_text().as_deref() == Some(TYPED)
            && !d.visible_text(&notice)
            && !d.has(&copy)
            && d.has(&undo),
        "Copy Log text puts the Log words on the clipboard and ends only its notice; the discarded draft's Undo stays",
    );
    // Astra's review: a link that two entries share is not gone; that scene says no `Not found`.
    entry(
        d,
        "Locations/Ashfall Keep, the old fort.md",
        KEEP,
        "location",
        "",
    );
    d.app_mut().lookup.refresh();
    d.settle();
    let row = d.place_of(&format!("Open: {KEEP}"));
    let found: Vec<_> = painted(d, "Not found")
        .into_iter()
        .map(|(rect, _)| rect)
        .filter(|rect| {
            row.is_some_and(|row| {
                rect.top() > row.top() && rect.top() < row.top() + 2.0 * crate::design::XL
            })
        })
        .collect();
    d.observe(
        "X14",
        row.is_some() && found.is_empty(),
        format!(
            "A name that two entries share is not Not found: row {row:?}, Not found at {found:?}"
        ),
    );

    // D-2 (a) for Place, in World's reader at 960: `Change` on Place opens its picker, and the
    // room for the Log entry comes into view in that pass. Choosing `Riverside Mill` writes
    // `Place Ashfall Keep → Riverside Mill.` into that room. The room holds two lines and this entry needs one, so
    // the content gives that line back: `Change` may move by one line, never more and never
    // out of view (ruling R30's amendment; weaker than "moves nothing").
    d.set_base(SMALL);
    // The note was deleted above (state 07); it comes back with its place set.
    fs::write(&path, &before).unwrap();
    entry(
        d,
        "Locations/Riverside Mill.md",
        "Riverside Mill",
        "location",
        "",
    );
    d.app_mut().lookup.refresh();
    d.settle();
    if d.has("Close Lookup") {
        d.click("Close Lookup");
    }
    d.click(NAV_WORLD);
    let campaign = d.app().campaigns.active_campaign().expect("a campaign").id;
    d.app_mut().world_page.reveal(campaign, varra);
    d.settle();
    d.expect("Change Place");
    d.click("Change Place");
    d.expect("Find a location");
    let world_reader = |d: &Driver<'_>| {
        d.place_of("Resize World notes column").map(|splitter| {
            egui::Rect::from_min_max(
                egui::pos2(splitter.right(), 0.0),
                egui::pos2(f32::INFINITY, f32::INFINITY),
            )
        })
    };
    let place_title =
        |d: &Driver<'_>| world_reader(d).and_then(|pane| d.painted_word_rect(VARRA_BLACK, pane));
    let _ = place_title;
    let before_clear = d.place_of("Change Place");
    d.type_text("Riverside");
    d.click("Riverside Mill");
    d.expect("Undo change to Place");
    let after_clear = d.place_of("Change Place");
    let in_view = d.in_view("Change Place");
    const PLACE_ENTRY: &str = "→ Riverside Mill.";
    let mut place_capture = Vec::new();
    d.state_prepared("place-cleared-log-in-view", Sizes::Default, |d| {
        d.settle();
        place_capture
            .push(world_reader(d).is_some_and(|pane| d.painted_in(pane).contains(PLACE_ENTRY)));
    });
    d.observe(
        "iter-29-D2-place-still",
        in_view
            && before_clear
                .zip(after_clear)
                .is_some_and(|(before, after)| {
                    (before.top() - after.top()).abs() <= crate::design::XL
                }),
        format!("At {SMALL:?}, Change Place {before_clear:?} → {after_clear:?} (at most one line)"),
    );
    for (size, entry) in [SMALL, DEFAULT].into_iter().zip(place_capture) {
        d.observe(
            "iter-29-D2-place-log",
            entry,
            format!("At {size:?}, the new Place entry is painted whole: {entry}"),
        );
    }
    fs::write(&path, &before).unwrap();
    d.app_mut().lookup.refresh();
    d.settle();
    d.set_base(DEFAULT);
}
